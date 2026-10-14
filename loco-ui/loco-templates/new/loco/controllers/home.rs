//! The starter's own pages: home, settings (signed in) and the theme toggle's post.
//! Written by `cargo lui new`; edit freely.

use loco_rs::prelude::*;
use loco_ui::prelude::*;
use serde::Deserialize;

use crate::views;

#[debug_handler]
async fn home(ui: Ui) -> Page {
    ui.page("{{name}}", views::home::home(&ui))
}

/// Signed-out visitors go to the sign-in form first (`loco_ui::loco::SignIn`).
#[debug_handler]
async fn settings(_auth: auth::JWT, ui: Ui) -> Page {
    ui.page("Settings", views::home::settings(&ui))
}

#[derive(Deserialize)]
struct ThemeForm {
    theme: String,
}

/// The theme toggle posts here: the choice is kept in a cookie for a year.
#[debug_handler]
async fn theme(ui: Ui, Form(form): Form<ThemeForm>) -> Redirect {
    ui.redirect("/settings").theme(Theme::parse(&form.theme))
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(home))
        .add("/settings", get(settings))
        .add("/theme", post(theme))
}
