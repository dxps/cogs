use crate::{AuthState, DataState, ExploreViewState, messages::UiMessage, views::ViewType};
use std::sync::mpsc::Sender;

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct UiState {
    pub curr_view_type: ViewType,

    #[serde(skip)]
    pub prev_view_type: ViewType,

    pub auth: AuthState,

    pub explore: ExploreViewState,

    pub data: DataState,

    #[serde(skip)]
    sender: Option<Sender<UiMessage>>,
}

impl UiState {
    pub fn set_sender(&mut self, sender: Sender<UiMessage>) {
        self.sender = Some(sender);
    }

    pub fn send(&self, msg: UiMessage) {
        if let Err(e) = self.sender.as_ref().unwrap().send(msg) {
            log::info!("[UiState] Failed to send message. Error: {e}");
        }
    }
}
