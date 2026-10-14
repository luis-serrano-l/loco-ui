//! Home and settings in the app shell. Written by `cargo lui new`; edit freely.

use loco_ui::prelude::*;

/// The app shell around a page: the app's name, its links, then `body`.
pub fn shell(ui: &Ui, body: Markup) -> Markup {
    lui! {
        AppShell("{{name}}") {
            link "Home" "/" icon=(Icon::House);
            link "Settings" "/settings" icon=(Icon::User);
            body (body);
        }
    }
}

pub fn home(ui: &Ui) -> Markup {
    let body = lui! {
        Flash;
        h1 { "{{name}}" }
        p {
            "Every page here works with JavaScript off. In development the first start makes a "
            "demo account: " code { "demo@example.com" } " / " code { "demo-password" } "."
        }
        LinkButton("Sign in", "/signin") primary;
        " "
        LinkButton("Settings", "/settings");
    };
    shell(ui, body)
}

pub fn settings(ui: &Ui) -> Markup {
    let body = lui! {
        Flash;
        SettingsPage("Settings") {
            section "Theme" "Light, dark, or as the system says." (ui.theme_toggle("/theme").render());
        }
    };
    shell(ui, body)
}
