use crate::{
    constants::ATTR_TEMPL_NEW_ID,
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

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    pub view_type: ViewType,
    pub auth: AuthState,
    pub explore: ExploreViewState,
    pub data_mgmt: DataMgmtState,
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AuthState {
    pub user: String,
    // #[serde(skip)]  // todo: temporary stored, during development
    pub pass: String,
    pub login_error: Option<AppError>,
    pub user_account: Option<UserAccount>,
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
}

impl DataMgmtState {
    pub fn save_attr_template(&self, ectx: &egui::Context) {
        //
        log::debug!("Saving attribute template: {self:#?}");
        let mut req = ehttp::Request::post(
            "http://localhost:9010/api/attribute_templates",
            serde_json::json!(self.curr_attr_template.clone())
                .to_string()
                .into_bytes(),
        );
        req.headers.insert("content-type", "application/json");
        let ectx = ectx.clone();
        ehttp::fetch(req, move |rsp| {
            log::info!("[status] clicked. Test response: {:#?}", rsp);
            if let Ok(rsp) = rsp {
                let dto: IdDto = serde_json::from_str(rsp.text().unwrap_or_default()).unwrap();
                log::debug!("Saved attribute template. Got id: {}", dto.id);
                ectx.data_mut(|data| data.insert_temp(ATTR_TEMPL_NEW_ID.into(), dto.id));
            }
        });
    }
}

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
