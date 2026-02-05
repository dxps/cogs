use crate::{UiState, messages::UiMessage};

pub(super) fn handle_msg(msg: UiMessage, state: &UiState) {
    //
    if msg == UiMessage::Logout {
        let mut req = ehttp::Request::post("http://localhost:9010/api/logout", vec![]);
        req.headers
            .insert("Authorization", state.auth.user_session.clone().unwrap_or_default());
        ehttp::fetch(req, move |rsp| {
            // Minimal enough, for now.
            log::info!("[handle_msg::Logout] Got response: {:?}", rsp);
        });
    }
}
