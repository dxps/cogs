use crate::messages::UiMessage;
use cogs_shared::{
    domain::model::{
        Id,
        meta::{AttrTemplate, AttributeValueType},
    },
    dtos::IdDto,
};
use std::sync::mpsc::Sender;

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DataState {
    #[serde(skip)]
    pub fetch_done: bool,

    #[serde(skip)]
    pub fetched_attr_templates: Vec<ManagedAttrTemplate>,
}

impl DataState {
    pub fn save_attr_template(&self, element: ManagedAttrTemplate, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(
            "http://localhost:9010/api/attribute_templates",
            serde_json::json!(element).to_string().into_bytes(),
        );
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[save_attr_template] Response: {:?}", rsp);
            if let Ok(rsp) = rsp {
                let dto: IdDto = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                log::debug!("[save_attr_template] Got saved id: {}", dto.id);
                if let Err(e) = sender.send(UiMessage::AttrTemplateUpserted(Ok(dto.id))) {
                    log::info!("[save_attr_template] Failed to send AttrTemplateUpserted message. Error: {e}");
                }
                ectx.request_repaint();
            }
        });
    }

    pub fn get_all_attr_templates(&self, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::get("http://localhost:9010/api/attribute_templates");
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            if let Ok(rsp) = rsp {
                let data: Vec<ManagedAttrTemplate> = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                log::info!("[get_all_attr_template] Got {} entries.", data.len());
                ectx.request_repaint(); // wake up UI thread
                if let Err(e) = sender.send(UiMessage::AttrTemplatesFetched(Ok(data))) {
                    log::info!("[get_all_attr_template] Failed to send AttrTemplatesFetched message. Error: {e}");
                    return;
                }
            }
        });
    }

    pub fn delete_attr_template(&self, id: Id, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(
            format!("http://localhost:9010/api/attribute_templates/{}/delete", id),
            vec![],
        );
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[delte_attr_template] Got response: {:?}", rsp);
            if let Err(e) = sender.send(UiMessage::AttrTemplateDeleted(Ok(id))) {
                log::info!("[save_attr_template] Failed to send AttrTemplateUpserted message. Error: {e}");
            }
            ectx.request_repaint();
        });
    }
}

/// The attribute template to be created or edited.
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManagedAttrTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
}

impl ManagedAttrTemplate {
    pub fn reset(&mut self) {
        self.id = Id::from(0);
        self.name = "".into();
        self.description = "".into();
        self.value_type = AttributeValueType::Text;
        self.default_value = "".into();
        self.is_required = false;
    }
}

impl From<AttrTemplate> for ManagedAttrTemplate {
    fn from(val: AttrTemplate) -> Self {
        Self {
            id: val.id,
            name: val.name,
            description: val.description.unwrap_or_default(),
            value_type: val.value_type,
            default_value: val.default_value,
            is_required: val.is_required,
        }
    }
}

impl From<&String> for ManagedAttrTemplate {
    fn from(value: &String) -> Self {
        serde_json::from_str::<Self>(value.as_str()).unwrap_or_default()
    }
}
