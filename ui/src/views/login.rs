use crate::{
    CogsApp,
    comps::{AppComponent, PasswordInput},
    views::{AppView, ViewType},
};
use cogs_shared::{app::AppError, domain::model::UserAccount, dtos::LoginRequest};
use egui::{Align2, RichText, Shadow, Stroke};
use poll_promise::{Promise, Sender};

pub struct Login {}

impl AppView for Login {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        //
        egui::CentralPanel::default().show(ectx, |ui| {
            let frame = egui::Frame::new()
                .corner_radius(6.0)
                .inner_margin(20.0)
                .stroke(Stroke::new(1.0, ui.style().visuals.faint_bg_color))
                .shadow(Shadow::NONE);

            let window = egui::Window::new("")
                .id(egui::Id::new("login_window_id")) // required since we change the title
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(frame)
                .max_size((340.0, 200.0))
                .resizable(false)
                .title_bar(false);

            window.show(ectx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(10.0);
                        ui.label(RichText::new("Login").heading());
                        ui.add_space(8.0);
                        ui.label("Provide the credentials below to authenticate into the system.");
                        ui.add_space(20.0);
                    });
                });

                ui.horizontal(|ui| {
                    ui.label("Username: ");
                    ui.add_space(4.0);
                    ui.text_edit_singleline(&mut ctx.state.user);
                    ui.add_space(20.0);
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label(" Password: ");
                    ui.add_space(4.0);
                    PasswordInput::show_input(ui, &mut ctx.state.pass);
                });

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    let login_btn = ui.button("  Login  ");
                    if login_btn.clicked() {
                        log::info!("Logging in w/ user: {} pass: {}", ctx.state.user, ctx.state.pass);

                        let (sender, promise) = Promise::<Result<UserAccount, AppError>>::new();
                        ctx.state.promise = Some(promise);

                        handle_login(ctx.state.user.clone(), ctx.state.pass.clone(), sender);
                        log::info!("Login promise set. is_some: {:?}", ctx.state.promise.is_some());

                        if let Some(promise) = &ctx.state.promise {
                            log::info!("Waiting for login promise.");
                            if let Some(res) = promise.ready() {
                                log::info!("Login promise ready.");
                                if let Ok(account) = res {
                                    log::info!("Remembering user account and going to home view.");
                                    ctx.state.user_account = Some(account.clone());
                                    ctx.state.view_type = ViewType::Home;
                                }
                            }
                        }
                    };
                    ui.add_space(10.0);
                });
            });
        });
    }
}

fn handle_login(user: String, pass: String, sender: Sender<Result<UserAccount, AppError>>) {
    let req_body = LoginRequest::new(user, pass);
    let mut req = ehttp::Request::post(
        "http://localhost:9010/api/login",
        req_body.as_json().as_bytes().to_vec(),
    );
    req.headers.insert("Content-Type", "application/json".to_string());
    ehttp::fetch(req, move |rsp| match rsp {
        Ok(rsp) => {
            if rsp.status == 200 {
                log::info!("Login successful!");
                let account = serde_json::from_slice::<UserAccount>(rsp.bytes.as_slice()).unwrap();
                sender.send(Ok(account));
            } else {
                log::info!("Login failed!");
            }
        }
        Err(e) => log::info!("Login failed! Error: {}", e),
    });
}
