//! Roles: a table with how many users have each, a dialog to add one, and a dialog that
//! confirms deleting one nobody has.

use axum::{
    Router,
    extract::{Path, State},
    routing::{get, post},
};
use loco_ui::prelude::*;

use crate::{Db, Res, auth::Admin, db, views};

pub fn routes() -> Router<Db> {
    Router::new()
        .route("/roles", get(index).post(create))
        .route("/roles/{id}/delete", post(delete))
}

async fn index(ui: Ui, admin: Admin, State(db): State<Db>) -> Res<Page> {
    let roles = db::roles(&db).await?;
    let names: Vec<(String, String, String)> = (roles.iter())
        .map(|r| {
            (
                format!("delete-role-{}", r.id),
                format!("Delete {}?", r.name),
                format!("/roles/{}/delete", r.id),
            )
        })
        .collect();
    let rows = roles.iter().zip(&names).map(|(r, (id, title, action))| {
        let users = html! { a href=(format!("/users?role={}", r.id)) { (r.users) } };
        let delete = html! {
            @if r.users == 0 {
                (ui.dialog("Delete").id(id).title(title).danger().size(1)
                    .description("Nobody has this role. It cannot be brought back.")
                    .confirm("Delete role", action))
            } @else { (ui.text("In use").muted()) }
        };
        (
            html! { strong { (r.name) } },
            r.description.as_str(),
            users,
            delete,
        )
    });
    let table = ui
        .table("roles", "/roles")
        .hide_search()
        .caption("Roles")
        .column("name", "Role")
        .column("description", "What it may do")
        .priority(2)
        .column("users", "Users")
        .numeric()
        .column("delete", "Delete")
        .rows(rows);
    let form = ui
        .form("/roles")
        .text("name", "Name")
        .required()
        .maxlength(40)
        .text("description", "What it may do")
        .required()
        .maxlength(120)
        .submit("Add role");
    let new = ui
        .dialog("New role")
        .id("new-role")
        .title("New role")
        .body(form.render());
    let count = format!("{} roles", roles.len());
    let list = ui
        .list_page("Roles")
        .description("What a user may do. A role in use cannot be deleted.")
        .count(&count)
        .actions(new.render())
        .body(table.render());
    Ok(views::shell(&ui, &admin, "Roles", list.render()))
}

async fn create(ui: Ui, admin: Admin, State(db): State<Db>, posted: Posted) -> Res<Redirect> {
    let (name, description) = (posted.get("name").trim(), posted.get("description").trim());
    if name.is_empty() || description.is_empty() {
        return Ok(ui
            .redirect("/roles")
            .danger("A role needs a name and a description."));
    }
    if !db::create_role(&db, name, description).await? {
        return Ok(ui
            .redirect("/roles")
            .danger(&format!("There is already a role named {name}.")));
    }
    db::log(&db, &admin.name, "created role", name).await?;
    Ok(ui.redirect("/roles").ok("Role added."))
}

async fn delete(ui: Ui, admin: Admin, State(db): State<Db>, Path(id): Path<i64>) -> Res<Redirect> {
    if !db::delete_role(&db, id).await? {
        return Ok(ui
            .redirect("/roles")
            .danger("That role is in use, or already gone."));
    }
    db::log(&db, &admin.name, "deleted role", &format!("#{id}")).await?;
    Ok(ui.redirect("/roles").ok("Role deleted."))
}
