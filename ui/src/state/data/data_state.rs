use crate::messages::UiMessage;
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        AccessLevel, Id,
        meta::{AttrTemplate, ItemTemplate, Kind},
    },
    dtos::IdDto,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DataState {
    #[serde(skip)]
    attr_templates: Vec<AttrTemplate>,

    #[serde(skip)]
    fetched_attr_templates: bool,

    #[serde(skip)]
    item_templates: Vec<ItemTemplate>,

    #[serde(skip)]
    fetched_item_templates: bool,

    #[serde(skip)]
    access_levels: Vec<AccessLevel>,

    #[serde(skip)]
    fetched_access_levels: bool,

    #[serde(skip)]
    access_levels_fetch_requested: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SourceType {
    Scratch,
    Template,
}

impl DataState {
    //

    pub fn has_fetched_all(&self) -> bool {
        self.fetched_attr_templates && self.fetched_item_templates
    }

    pub fn has_fetched_attr_templates(&self) -> bool {
        self.fetched_attr_templates
    }

    pub fn has_fetched_item_templates(&self) -> bool {
        self.fetched_item_templates
    }

    pub fn should_fetch_access_levels(&self) -> bool {
        !self.fetched_access_levels && !self.access_levels_fetch_requested
    }

    // ------------------------
    // Attribute Templates mgmt
    // ------------------------

    pub fn save_attr_template(&self, element: AttrTemplate, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(
            "http://localhost:9010/api/attribute_templates",
            serde_json::json!(element).to_string().into_bytes(),
        );
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        let is_new = element.id.is_zero();
        ehttp::fetch(req, move |rsp| {
            log::info!("[DataState::save_attr_template] Response: {:?}", rsp);
            if let Ok(rsp) = rsp {
                let dto: IdDto = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap_or_else(|e| {
                    log::error!("[DataState::save_attr_template] Error: {e}");
                    // TODO: tell the caller.
                    IdDto::default()
                });
                let ui_msg = if is_new {
                    log::debug!("[DataState::save_attr_template] Got id: {}", dto.id);
                    UiMessage::ElementCreated(Kind::AttributeTemplate, Ok(dto.id))
                } else {
                    UiMessage::ElementUpdated(Kind::AttributeTemplate, Ok(dto.id))
                };
                if let Err(e) = sender.send(ui_msg) {
                    log::info!("[DataState::save_attr_template] Failed to send UiMessage. Error: {e}");
                }
                ectx.request_repaint();
            }
        });
    }

    pub fn set_attr_templates(&mut self, data: Vec<AttrTemplate>) {
        self.attr_templates = data;
        self.fetched_attr_templates = true;
    }

    pub fn set_item_templates(&mut self, data: Vec<ItemTemplate>) {
        self.item_templates = data;
        self.fetched_item_templates = true;
    }

    pub fn set_access_levels(&mut self, data: Vec<AccessLevel>) {
        self.access_levels = data;
        self.fetched_access_levels = true;
    }

    pub fn get_attr_templates(&self) -> Vec<AttrTemplate> {
        self.attr_templates.clone()
    }

    pub fn fetch_all_attr_templates(&self, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::get("http://localhost:9010/api/attribute_templates");
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            if let Ok(rsp) = rsp {
                let data: Vec<AttrTemplate> = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                log::trace!("[DataState::fetch_all_attr_templates] Got {} elements.", data.len());
                if let Err(e) = sender.send(UiMessage::AttrTemplatesFetched(Ok(data))) {
                    log::error!("[DataState::fetch_all_attr_templates] Failed to send UiMessage. Error: {e}");
                }
                ectx.request_repaint();
            }
        });
    }

    pub fn delete_attr_template(&self, id: Id, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(format!("http://localhost:9010/api/attribute_templates/{}/delete", id), vec![]);
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::trace!("[DataState::delete_attr_template] Got response: {:?}", rsp);
            if let Err(e) = sender.send(UiMessage::ElementDeleted(Kind::AttributeTemplate, Ok(id))) {
                log::info!("[DataState::delete_attr_template] Failed to send UiMessage. Error: {e}");
            }
            ectx.request_repaint();
        });
    }

    // -------------------
    // Item Templates mgmt
    // -------------------

    pub fn save_item_template(&self, element: ItemTemplate, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(
            "http://localhost:9010/api/item_templates",
            serde_json::json!(element).to_string().into_bytes(),
        );
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[DataState::save_item_template] Response: {:?}", rsp);
            match rsp {
                Ok(rsp) => {
                    let ars: AppResult<Id>;

                    if rsp.status != 200 {
                        ars = Err(AppError::ErrDetails(
                            format!("{}", rsp.status),
                            rsp.text().unwrap_or_default().into(),
                        ));
                    } else {
                        let dto: IdDto = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                        log::trace!("[DataState::save_item_template] Got saved id: {}", dto.id);
                        ars = Ok(dto.id);
                    }

                    if let Err(e) = sender.send(UiMessage::ElementCreated(Kind::ItemTemplate, ars)) {
                        log::error!("[DataState::save_item_template] Failed to send UiMessage. Error: {e}");
                    }
                }
                Err(err) => {
                    let ars = Err(AppError::from(err));
                    if let Err(e) = sender.send(UiMessage::ElementUpdated(Kind::ItemTemplate, ars)) {
                        log::error!("[DataState::save_item_template] Failed to send UiMessage. Error: {e}");
                    }
                }
            }
            ectx.request_repaint();
        });
    }

    pub fn fetch_all_item_templates(&self, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::get("http://localhost:9010/api/item_templates");
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            if let Ok(rsp) = rsp {
                let data: Vec<ItemTemplate> = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                log::trace!("[DataState::fetch_all_item_templates] Got {} elements.", data.len());
                if let Err(e) = sender.send(UiMessage::ItemTemplatesFetched(Ok(data))) {
                    log::info!("[DataState::fetch_all_item_templates] Failed to send UiMessage. Error: {e}");
                }
                ectx.request_repaint();
            }
        });
    }

    pub fn get_item_templates(&self) -> Vec<ItemTemplate> {
        self.item_templates.clone()
    }

    pub fn get_access_levels(&self) -> Vec<AccessLevel> {
        self.access_levels.clone()
    }

    pub fn get_item_template_name(&self, id: &Id) -> String {
        self.item_templates
            .iter()
            .find(|it| &it.id == id)
            .map(|it| it.name.clone())
            .unwrap_or_default()
    }

    pub fn delete_item_template(&self, id: Id, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(format!("http://localhost:9010/api/item_templates/{}/delete", id), vec![]);
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::trace!("[DataState::delete_item_template] Got response: {:?}", rsp);
            if let Err(e) = sender.send(UiMessage::ElementDeleted(Kind::ItemTemplate, Ok(id))) {
                log::error!("[DataState::delete_item_template] Failed to send UiMessage. Error: {e}");
            }
            ectx.request_repaint();
        });
    }

    pub fn fetch_all_access_levels(&mut self, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        self.access_levels_fetch_requested = true;
        let mut req = ehttp::Request::get("http://localhost:9010/api/access_levels");
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            let result = match rsp {
                Ok(rsp) => decode_json_response::<Vec<AccessLevel>>(&rsp),
                Err(err) => Err(AppError::ErrDetails("failed to fetch access levels".to_string(), err)),
            };

            if let Ok(data) = &result {
                log::trace!("[DataState::fetch_all_access_levels] Got {} elements.", data.len());
            }

            if let Err(e) = sender.send(UiMessage::AccessLevelsFetched(result)) {
                log::error!("[DataState::fetch_all_access_levels] Failed to send UiMessage. Error: {e}");
            }
            ectx.request_repaint();
        });
    }
}

fn decode_json_response<T: DeserializeOwned>(rsp: &ehttp::Response) -> AppResult<T> {
    let body = rsp.text().unwrap_or("<response body is not valid UTF-8>");

    if !rsp.ok {
        return Err(AppError::ErrDetails(
            format!("HTTP {} {}", rsp.status, rsp.status_text),
            body.to_string(),
        ));
    }

    rsp.json()
        .map_err(|err| AppError::ErrDetails("invalid JSON response".to_string(), format!("{err}; response body: {body}")))
}

#[cfg(test)]
mod tests {
    use super::decode_json_response;
    use cogs_shared::domain::model::AccessLevel;

    fn response(status: u16, body: &str) -> ehttp::Response {
        ehttp::Response {
            url: "http://localhost/api/access_levels".to_string(),
            ok: (200..300).contains(&status),
            status,
            status_text: String::new(),
            headers: ehttp::Headers::default(),
            bytes: body.as_bytes().to_vec(),
        }
    }

    #[test]
    fn access_level_error_object_is_not_treated_as_a_list() {
        let result = decode_json_response::<Vec<AccessLevel>>(&response(404, r#"{"error":"not found"}"#));

        assert!(result.is_err());
    }

    #[test]
    fn invalid_access_level_shape_returns_an_error() {
        let result = decode_json_response::<Vec<AccessLevel>>(&response(200, r#"{"error":"internal error"}"#));

        assert!(result.is_err());
    }
}
