//! The pages: home and settings in the app shell, and the theme toggle's post.

use axum::Form;
use loco_ui::prelude::*;
use serde::Deserialize;

/// The app shell around every page: the app's name, its links, then `body`.
fn shell(ui: &Ui, body: Markup) -> Markup {
    lui! {
        AppShell("{{name}}") {
            link "Home" "/" icon=(Icon::House);
            link "Settings" "/settings" icon=(Icon::User);
            body (body);
        }
    }
}

pub async fn home(ui: Ui) -> Page {
    let body = lui! {
        h1 { "{{name}}" }
        p { "Every page here works with JavaScript off. Add a route in " code { "src/lib.rs" } " and a page beside these." }
        LinkButton("Settings", "/settings") primary;
    };
    ui.page("{{name}}", shell(&ui, body))
}

pub async fn settings(ui: Ui) -> Page {
    let body = lui! {
        SettingsPage("Settings") {
            section "Theme" "Light, dark, or as the system says." (ui.theme_toggle("/theme").render());
        }
    };
    ui.page("Settings", shell(&ui, body))
}

#[derive(Deserialize)]
pub struct ThemeForm {
    theme: String,
}

/// The theme toggle posts here: the choice is kept in a cookie for a year.
pub async fn theme(ui: Ui, Form(form): Form<ThemeForm>) -> Redirect {
    ui.redirect("/settings").theme(Theme::parse(&form.theme))
}
