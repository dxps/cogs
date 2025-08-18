use crate::{
    messages::UiMessage,
    views::{ExploreCategory, ExploreKind, ViewType},
};
use cogs_shared::{
    app::AppError,
    domain::model::{
        Id, UserAccount,
        meta::{AttributeValueType, Kind},
    },
    dtos::IdDto,
};
use std::sync::mpsc::Sender;

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    pub curr_view_type: ViewType,
    #[serde(skip)]
    pub prev_view_type: ViewType,
    pub auth: AuthState,
    pub explore: ExploreViewState,
    pub data_mgmt: DataMgmtState,
    #[serde(skip)]
    sender: Option<Sender<UiMessage>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AuthState {
    pub user: String,
    // #[serde(skip)]  // todo: temporary stored, during development
    pub pass: String,
    #[serde(skip)]
    pub login_user_focus: bool,
    #[serde(skip)]
    /// Used to determine if the enter key was pressed, after the password field was focused.
    pub login_pass_enter: bool,
    pub login_error: Option<AppError>,
    pub user_account: Option<UserAccount>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: Default::default(),
            pass: Default::default(),
            login_user_focus: true,
            login_pass_enter: false,
            login_error: Default::default(),
            user_account: Default::default(),
        }
    }
}

impl AppState {
    pub fn set_sender(&mut self, sender: Sender<UiMessage>) {
        self.sender = Some(sender);
    }

    pub fn send(&self, msg: UiMessage) {
        if let Err(e) = self.sender.as_ref().unwrap().send(msg) {
            log::info!("[AppState] Failed to send message. Error: {e}");
        }
    }
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExploreViewState {
    pub category: ExploreCategory,
    pub kind: ExploreKind,
    pub add_kind: Option<Kind>,
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DataMgmtState {
    pub curr_attr_template: ManagedAttrTemplate,
    #[serde(skip)]
    pub fetch_done: bool,
    #[serde(skip)]
    pub fetched_attr_templates: Vec<ManagedAttrTemplate>,
}

impl DataMgmtState {
    pub fn save_attr_template(&self, ectx: &egui::Context, sender: Sender<UiMessage>) {
        //
        let mut req = ehttp::Request::post(
            "http://localhost:9010/api/attribute_templates",
            serde_json::json!(self.curr_attr_template.clone())
                .to_string()
                .into_bytes(),
        );
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[save_attr_template] Response: {:#?}", rsp);
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

    pub fn get_all_attr_template(&mut self, ectx: &egui::Context, sender: Sender<UiMessage>) {
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
