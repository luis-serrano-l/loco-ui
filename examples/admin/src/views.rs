//! The shell every signed-in page sits in, and the two small things the areas share.

use loco_ui::prelude::*;

use crate::auth::Admin;

/// A signed-in page: `body` in the app shell, with the navigation and the sign-out button.
pub fn shell(ui: &Ui, admin: &Admin, title: &str, body: Markup) -> Page {
    let shell = ui
        .app_shell("Admin")
        .current_prefix()
        .link("Users", "/users")
        .icon(Icon::User)
        .link("Roles", "/roles")
        .icon(Icon::Star)
        .link("Audit log", "/audit")
        .icon(Icon::File)
        .group("Account")
        .link("Settings", "/settings")
        .icon(Icon::Pencil)
        .user(&admin.name, "/signout")
        .body(body);
    ui.page(title, shell.render())
}

/// "Active" or "Suspended", as a badge.
pub fn status(ui: &Ui, active: bool) -> Markup {
    if active {
        ui.badge("Active").ok().render()
    } else {
        ui.badge("Suspended").warn().render()
    }
}

/// The answer for an id that is not there: a 404 page inside the shell.
pub fn missing(ui: &Ui, admin: &Admin, what: &str, back: &str) -> crate::Failure {
    let body = ui
        .empty_state(&format!("No such {what}"))
        .link("Back to the list", back)
        .render();
    crate::Failure::Missing(Box::new(shell(ui, admin, "Not found", body)))
}
