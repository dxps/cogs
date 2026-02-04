use crate::{AuthState, DataState, ExploreViewState, messages::UiMessage, views::ViewName};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct UiState {
    //
    curr_view: ViewName,

    #[serde(skip)]
    prev_view: ViewName,

    pub auth: AuthState,

    pub explore: ExploreViewState,

    pub data: DataState,

    #[serde(skip)]
    sender: Option<Sender<UiMessage>>,
}

impl UiState {
    pub fn set_curr_view(&mut self, view: ViewName) {
        if self.prev_view != ViewName::Login {
            self.auth.login_user_focus = true;
        }
        self.prev_view = self.curr_view.clone();
        self.curr_view = view;
        #[cfg(target_arch = "wasm32")]
        self.update_url_hash();
    }

    pub fn curr_view(&self) -> &ViewName {
        &self.curr_view
    }

    pub fn prev_view(&self) -> &ViewName {
        &self.prev_view
    }

    pub fn set_sender(&mut self, sender: Sender<UiMessage>) {
        self.sender = Some(sender);
    }

    pub fn send(&self, msg: UiMessage) {
        if let Err(e) = self.sender.as_ref().unwrap().send(msg) {
            log::info!("[UiState] Failed to send message. Error: {e}");
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn update_url_hash(&self) {
        let hash = match self.curr_view {
            ViewName::Home => "#/",
            ViewName::Explore => "#/explore",
            ViewName::Settings => "#/settings",
            ViewName::Login => "#/login",
        };
        let window = web_sys::window().unwrap();
        window.location().set_hash(hash).unwrap();
    }
}
