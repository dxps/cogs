use crate::messages::UiMessage;
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        Id,
        meta::{AttrTemplate, ItemTemplate, Kind, LinkTemplate},
    },
    dtos::IdDto,
};
use std::sync::mpsc::Sender;

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DataState {
    /// It tells whether we have fetched the data or not.
    #[serde(skip)]
    pub fetch_done: bool,

    #[serde(skip)]
    fetched_attr_templates: Vec<AttrTemplate>,

    #[serde(skip)]
    fetched_item_templates: Vec<ItemTemplate>,
}

impl DataState {
    //

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
                let dto: IdDto = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
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
        self.fetched_attr_templates = data;
    }

    pub fn set_item_templates(&mut self, data: Vec<ItemTemplate>) {
        self.fetched_item_templates = data;
    }

    pub fn get_attr_templates(&self) -> Vec<AttrTemplate> {
        self.fetched_attr_templates.clone()
    }

    pub fn fetch_all_attr_templates(&self, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::get("http://localhost:9010/api/attribute_templates");
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            if let Ok(rsp) = rsp {
                let data: Vec<AttrTemplate> = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                log::info!("[DataState::fetch_all_attr_templates] Got {} elements.", data.len());
                ectx.request_repaint(); // wake up UI thread
                if let Err(e) = sender.send(UiMessage::AttrTemplatesFetched(Ok(data))) {
                    log::info!("[DataState::fetch_all_attr_templates] Failed to send UiMessage. Error: {e}");
                    return;
                }
            }
        });
    }

    pub fn delete_attr_template(&self, id: Id, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(format!("http://localhost:9010/api/attribute_templates/{}/delete", id), vec![]);
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[DataState::delete_attr_template] Got response: {:?}", rsp);
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
                        log::debug!("[DataState::save_item_template] Got saved id: {}", dto.id);
                        ars = Ok(dto.id);
                    }

                    if let Err(e) = sender.send(UiMessage::ElementCreated(Kind::ItemTemplate, ars)) {
                        log::info!("[DataState::save_item_template] Failed to send UiMessage. Error: {e}");
                    }
                }
                Err(err) => {
                    let ars = Err(AppError::from(err));
                    if let Err(e) = sender.send(UiMessage::ElementUpdated(Kind::ItemTemplate, ars)) {
                        log::info!("[DataState::save_item_template] Failed to send UiMessage. Error: {e}");
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
                log::info!("[DataState::fetch_all_item_templates] Got {} elements.", data.len());
                ectx.request_repaint(); // wake up UI thread
                if let Err(e) = sender.send(UiMessage::ItemTemplatesFetched(Ok(data))) {
                    log::info!("[DataState::fetch_all_item_templates] Failed to send UiMessage. Error: {e}");
                    return;
                }
            }
        });
    }

    pub fn get_item_templates(&self) -> Vec<ItemTemplate> {
        self.fetched_item_templates.clone()
    }

    pub fn delete_item_template(&self, id: Id, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(format!("http://localhost:9010/api/item_templates/{}/delete", id), vec![]);
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[DataState::delete_item_template] Got response: {:?}", rsp);
            if let Err(e) = sender.send(UiMessage::ElementDeleted(Kind::ItemTemplate, Ok(id))) {
                log::info!("[DataState::delete_item_template] Failed to send UiMessage. Error: {e}");
            }
            ectx.request_repaint();
        });
    }

    // -----------------
    // LinkTemplate mgmt
    // -----------------

    pub fn save_link_template(&self, element: LinkTemplate, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(
            "http://localhost:9010/api/link_templates",
            serde_json::json!(element).to_string().into_bytes(),
        );
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[DataState::save_link_template] Response: {:?}", rsp);
            if let Ok(rsp) = rsp {
                let dto: IdDto = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                log::debug!("[DataState::save_link_template] Got saved id: {}", dto.id);
                if let Err(e) = sender.send(UiMessage::ElementUpdated(Kind::LinkTemplate, Ok(dto.id))) {
                    log::info!("[DataState::save_link_template] Failed to send UiMessage. Error: {e}");
                }
                ectx.request_repaint();
            }
        });
    }
}
