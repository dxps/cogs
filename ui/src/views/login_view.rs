use crate::{
    CogsApp,
    comps::{AppComponent, Modal, PasswordInput},
    constants::{MODAL_BTN_LABEL, MODAL_BTN_MSG, MODAL_CONTENT, MODAL_TITLE},
    messages::UiMessage,
    views::AppView,
};
use cogs_shared::{app::AppError, domain::model::UserAccount, dtos::LoginRequest};
use egui::{Align2, Id, RichText, Shadow, Stroke};
use std::sync::mpsc::Sender;

pub struct Login {}

impl AppView for Login {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        //
        egui::CentralPanel::default().show(ectx, |ui| {
            let frame = egui::Frame::new()
                .corner_radius(6.0)
                .inner_margin(20.0)
                .stroke(Stroke::new(1.0, ui.style().visuals.code_bg_color))
                .shadow(Shadow::NONE);

            let window = egui::Window::new("")
                .id(egui::Id::new("login_window_id"))
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(frame)
                .max_size((340.0, 200.0))
                .resizable(false)
                .title_bar(false);

            window.show(ectx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Login").heading());
                        ui.add_space(20.0);
                        ui.label("Provide the credentials below to authenticate into the system.");
                        ui.add_space(20.0);
                    });
                });

                ui.horizontal(|ui| {
                    ui.label("Username: ");
                    ui.add_space(4.0);
                    let user_input = ui.text_edit_singleline(&mut ctx.state.auth.user);
                    if ctx.state.auth.login_user_focus {
                        user_input.request_focus();
                        ctx.state.auth.login_user_focus = false;
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label(" Password: ");
                    ui.add_space(4.0);
                    PasswordInput::show_input_entered(ui, &mut ctx.state.auth.pass, &mut ctx.state.auth.login_pass_enter);
                });

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    if ui.button("   Login   ").clicked() {
                        handle_login(
                            ctx.state.auth.user.clone(),
                            ctx.state.auth.pass.clone(),
                            ctx.sendr.clone(),
                            ectx.clone(),
                        );
                    };
                    ui.add_space(10.0);
                });

                if ctx.state.auth.login_pass_enter && !ctx.state.auth.login_user_focus {
                    ctx.state.auth.login_pass_enter = false;
                    handle_login(
                        ctx.state.auth.user.clone(),
                        ctx.state.auth.pass.clone(),
                        ctx.sendr.clone(),
                        ectx.clone(),
                    );
                }

                if let Some(login_err) = &ctx.state.auth.login_error {
                    if *login_err == AppError::LoginWrongCredentials {
                        ectx.data_mut(|data| {
                            data.insert_temp::<String>(Id::new(MODAL_TITLE), "Authentication failed".to_string());
                            data.insert_temp::<String>(
                                Id::new(MODAL_CONTENT),
                                "Invalid username or password. Please try again.".to_string(),
                            );
                            data.insert_temp::<String>(Id::new(MODAL_BTN_LABEL), "   Close   ".to_string());
                            data.insert_temp::<UiMessage>(Id::new(MODAL_BTN_MSG), UiMessage::Login(Ok(None)));
                        });
                        ectx.request_repaint();
                        Modal::show(ctx, ui);
                    }
                }
            });
        });
    }
}

fn handle_login(user: String, pass: String, sender: Sender<UiMessage>, ectx: egui::Context) {
    let body = LoginRequest::new(user, pass);
    let mut req = ehttp::Request::post("http://localhost:9010/api/login", body.as_json().as_bytes().to_vec());
    req.headers.insert("Content-Type", "application/json".to_string());
    ehttp::fetch(req, move |rsp| {
        match rsp {
            Ok(rsp) => {
                if rsp.status == 200 {
                    log::info!("[handle_login] Login successful!");
                    let account = serde_json::from_slice::<UserAccount>(rsp.bytes.as_slice()).unwrap();
                    ectx.request_repaint(); // wake up UI thread
                    if let Err(e) = sender.send(UiMessage::Login(Ok(Some(account)))) {
                        log::info!("[handle_login] Failed to send Login message. Error: {e}");
                    }
                } else {
                    log::info!("[handle_login] Login failed! HTTP status code: {}", rsp.status);
                    if rsp.status == 401 {
                        let aerr = AppError::LoginWrongCredentials;
                        if let Err(e) = sender.send(UiMessage::Login(Err(aerr))) {
                            log::info!("[handle_login] Failed to send Login message. Error: {e}");
                        }
                    } else {
                        let aerr = AppError::from(format!("{}", rsp.status));
                        if let Err(e) = sender.send(UiMessage::Login(Err(aerr))) {
                            log::info!("[handle_login] Failed to send Login message. Error: {e}");
                        }
                    }
                }
            }
            Err(e) => log::info!("[handle_login] Login failed! Error: {}", e),
        }
    });
}
