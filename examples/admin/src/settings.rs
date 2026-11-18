//! Settings for the signed-in account: the name, and the password.

use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use loco_ui::prelude::*;

use crate::{
    Db, Res,
    auth::{self, Admin},
    db, views,
};

pub fn routes() -> Router<Db> {
    Router::new()
        .route("/settings", get(index))
        .route("/settings/profile", post(profile))
        .route("/settings/password", post(password))
}

fn page(ui: &Ui, admin: &Admin, profile: &[(&str, &str)], password: &[(&str, &str)]) -> Page {
    let name = [("name".to_string(), admin.name.clone())];
    let profile = ui
        .form("/settings/profile")
        .text("name", "Name")
        .required()
        .maxlength(80)
        .values(&name)
        .errors(profile)
        .submit("Save name");
    let password = ui
        .form("/settings/password")
        .password("current", "Current password")
        .required()
        .autocomplete("current-password")
        .password("new", "New password")
        .required()
        .autocomplete("new-password")
        .help("At least 12 characters.")
        .errors(password)
        .submit("Change password");
    let settings = ui
        .settings_page("Settings")
        .section(
            "Profile",
            "Your name in the shell and in the audit log.",
            profile.render(),
        )
        .section(
            "Password",
            "Changing it signs you out everywhere else.",
            password.render(),
        );
    views::shell(ui, admin, "Settings", settings.render())
}

async fn index(ui: Ui, admin: Admin) -> Page {
    page(&ui, &admin, &[], &[])
}

async fn profile(
    ui: Ui,
    admin: Admin,
    State(db): State<Db>,
    posted: Posted,
) -> Res<Result<Redirect, Page>> {
    let name = posted.get("name").trim();
    if name.is_empty() {
        return Ok(Err(
            page(&ui, &admin, &[("name", "Enter a name.")], &[]).invalid()
        ));
    }
    db::rename_user(&db, admin.id, name).await?;
    db::log(&db, &admin.name, "renamed themselves", name).await?;
    Ok(Ok(ui.redirect("/settings").ok("Name saved.")))
}

async fn password(
    ui: Ui,
    admin: Admin,
    State(db): State<Db>,
    posted: Posted,
) -> Res<Result<Redirect, Page>> {
    let mut errors = Vec::new();
    if !auth::verify(
        &db::password_hash(&db, admin.id).await?,
        posted.get("current"),
    ) {
        errors.push(("current", "That is not your current password."));
    }
    if posted.get("new").len() < 12 {
        errors.push(("new", "Use at least 12 characters."));
    }
    if !errors.is_empty() {
        return Ok(Err(page(&ui, &admin, &[], &errors).invalid()));
    }
    db::set_password(&db, admin.id, &auth::hash(posted.get("new")), &admin.token).await?;
    db::log(&db, &admin.name, "changed their password", "").await?;
    Ok(Ok(ui.redirect("/settings").ok("Password changed.")))
}
