//! Users: the list (search, status and role filters), a record page, and the forms.

use axum::{
    Router,
    extract::{Path, State},
    routing::{get, post},
};
use loco_ui::prelude::*;

use crate::{
    Db, Res,
    auth::{self, Admin},
    db::{self, Role, User},
    views,
};

pub fn routes() -> Router<Db> {
    Router::new()
        .route("/users", get(index).post(create))
        .route("/users/new", get(new))
        .route("/users/{id}", get(show).post(update))
        .route("/users/{id}/edit", get(edit))
        .route("/users/{id}/delete", post(delete))
}

async fn index(ui: Ui, admin: Admin, State(db): State<Db>) -> Res<Page> {
    let (q, status) = (
        ui.param("q").unwrap_or("").trim(),
        ui.param("status").unwrap_or(""),
    );
    let role = ui.param("role").and_then(|r| r.parse().ok()).unwrap_or(0);
    let users = db::users(&db, q, status, role).await?;
    let (all, active) = db::user_counts(&db).await?;
    let count = format!("{} of {all} users", users.len());
    let rows = users.iter().map(|u| {
        let name = html! { a href=(format!("/users/{}", u.id)) { (u.name) } };
        (
            name,
            u.email.as_str(),
            ui.badge(&u.role).render(),
            views::status(&ui, u.active),
            u.created.as_str(),
        )
    });
    let table = ui
        .table("users", "/users")
        .hide_search()
        .caption("Users")
        .column("name", "Name")
        .column("email", "Email")
        .priority(2)
        .column("role", "Role")
        .column("status", "Status")
        .column("created", "Created")
        .priority(3)
        .rows(rows);
    let body = html! {
        @if users.is_empty() { (ui.empty_state("No user matches").link("Show everyone", "/users")) }
        @else { (table) }
    };
    let list = ui
        .list_page("Users")
        .description("Everyone who can sign in.")
        .search("Search users")
        .filter("All", "/users")
        .badge(all)
        .filter("Active", "/users?status=active")
        .badge(active)
        .filter("Suspended", "/users?status=suspended")
        .badge(all - active)
        .count(&count)
        .actions(ui.link_button("New user", "/users/new").primary().render())
        .body(body);
    Ok(views::shell(&ui, &admin, "Users", list.render()))
}

/// What the user form posted, checked. `id` is 0 for a new user.
async fn check(db: &Db, posted: &Posted, id: i64) -> Res<Vec<(&'static str, &'static str)>> {
    let mut errors = Vec::new();
    if posted.get("name").trim().is_empty() {
        errors.push(("name", "Enter a name."));
    }
    let email = posted.get("email").trim().to_lowercase();
    if !email.contains('@') {
        errors.push(("email", "Enter an email address."));
    } else if db::email_taken(db, &email, id).await? {
        errors.push(("email", "Another user has this address."));
    }
    if id == 0 && posted.get("password").len() < 12 {
        errors.push(("password", "Use at least 12 characters."));
    }
    Ok(errors)
}

fn form(
    ui: &Ui,
    user: Option<&User>,
    roles: &[Role],
    values: &[(String, String)],
    errors: &[(&str, &str)],
) -> Markup {
    let action = user.map_or("/users".to_string(), |u| format!("/users/{}", u.id));
    let cancel = user.map_or("/users".to_string(), |u| format!("/users/{}", u.id));
    let ids: Vec<String> = roles.iter().map(|r| r.id.to_string()).collect();
    let options = ids
        .iter()
        .zip(roles)
        .map(|(id, r)| (id.as_str(), r.name.as_str()));
    // A sent-back form says whether the box was ticked; a fresh one starts from the record.
    let active = if values.is_empty() {
        user.is_none_or(|u| u.active)
    } else {
        values.iter().any(|(k, _)| k == "active")
    };
    let form = ui
        .form(&action)
        .text("name", "Name")
        .required()
        .maxlength(80)
        .email("email", "Email")
        .required();
    let form = if user.is_none() {
        form.password("password", "Password")
            .required()
            .autocomplete("new-password")
            .help("At least 12 characters. They can change it in Settings.")
    } else {
        form
    };
    form.select("role", "Role", options)
        .switch("active", "Can sign in")
        .checked(active)
        .values(values)
        .errors(errors)
        .submit(if user.is_some() {
            "Save"
        } else {
            "Create user"
        })
        .action_secondary("Cancel", &cancel)
        .render()
}

/// The values a fresh edit form starts with.
fn values_of(u: &User) -> Vec<(String, String)> {
    vec![
        ("name".into(), u.name.clone()),
        ("email".into(), u.email.clone()),
        ("role".into(), u.role_id.to_string()),
    ]
}

fn form_page(ui: &Ui, admin: &Admin, title: &str, form: Markup) -> Page {
    let body = ui
        .stack()
        .gap(4)
        .body(html! { h1 { (title) } (ui.card().body(form)) })
        .render();
    views::shell(ui, admin, title, body)
}

async fn new(ui: Ui, admin: Admin, State(db): State<Db>) -> Res<Page> {
    let roles = db::roles(&db).await?;
    Ok(form_page(
        &ui,
        &admin,
        "New user",
        form(
            &ui,
            None,
            &roles,
            &[("role".into(), "3".into()), ("active".into(), "on".into())],
            &[],
        ),
    ))
}

async fn create(
    ui: Ui,
    admin: Admin,
    State(db): State<Db>,
    posted: Posted,
) -> Res<Result<Redirect, Page>> {
    let errors = check(&db, &posted, 0).await?;
    if !errors.is_empty() {
        let roles = db::roles(&db).await?;
        return Ok(Err(form_page(
            &ui,
            &admin,
            "New user",
            form(&ui, None, &roles, posted.pairs(), &errors),
        )
        .invalid()));
    }
    let (name, email) = (
        posted.get("name").trim(),
        posted.get("email").trim().to_lowercase(),
    );
    let role = posted.get("role").parse().unwrap_or(3);
    let id = db::create_user(
        &db,
        name,
        &email,
        &auth::hash(posted.get("password")),
        role,
        !posted.get("active").is_empty(),
    )
    .await?;
    db::log(&db, &admin.name, "created user", &email).await?;
    Ok(Ok(ui.redirect(&format!("/users/{id}")).ok("User created.")))
}

async fn show(ui: Ui, admin: Admin, State(db): State<Db>, Path(id): Path<i64>) -> Res<Page> {
    let user = db::find_user(&db, id)
        .await?
        .ok_or_else(|| views::missing(&ui, &admin, "user", "/users"))?;
    let (edit, delete) = (format!("/users/{id}/edit"), format!("/users/{id}/delete"));
    let record = ui
        .record_page(&user.name)
        .back("/users")
        .field("Email", &user.email)
        .field("Role", ui.badge(&user.role))
        .field("Status", views::status(&ui, user.active))
        .field("Created", &user.created)
        .edit(&edit);
    // Nobody deletes the account they are signed in with.
    let record = if id == admin.id {
        record
    } else {
        record.delete(&delete)
    };
    Ok(views::shell(&ui, &admin, &user.name, record.render()))
}

async fn edit(ui: Ui, admin: Admin, State(db): State<Db>, Path(id): Path<i64>) -> Res<Page> {
    let user = db::find_user(&db, id)
        .await?
        .ok_or_else(|| views::missing(&ui, &admin, "user", "/users"))?;
    let roles = db::roles(&db).await?;
    let mut values = values_of(&user);
    if user.active {
        values.push(("active".into(), "on".into()));
    }
    let title = format!("Edit {}", user.name);
    Ok(form_page(
        &ui,
        &admin,
        &title,
        form(&ui, Some(&user), &roles, &values, &[]),
    ))
}

async fn update(
    ui: Ui,
    admin: Admin,
    State(db): State<Db>,
    Path(id): Path<i64>,
    posted: Posted,
) -> Res<Result<Redirect, Page>> {
    let user = db::find_user(&db, id)
        .await?
        .ok_or_else(|| views::missing(&ui, &admin, "user", "/users"))?;
    let mut errors = check(&db, &posted, id).await?;
    let active = !posted.get("active").is_empty();
    if id == admin.id && !active {
        errors.push((
            "active",
            "You cannot suspend the account you are signed in with.",
        ));
    }
    if !errors.is_empty() {
        let roles = db::roles(&db).await?;
        let title = format!("Edit {}", user.name);
        return Ok(Err(form_page(
            &ui,
            &admin,
            &title,
            form(&ui, Some(&user), &roles, posted.pairs(), &errors),
        )
        .invalid()));
    }
    let email = posted.get("email").trim().to_lowercase();
    let role = posted.get("role").parse().unwrap_or(user.role_id);
    db::update_user(&db, id, posted.get("name").trim(), &email, role, active).await?;
    db::log(&db, &admin.name, "updated user", &email).await?;
    Ok(Ok(ui.redirect(&format!("/users/{id}")).ok("Saved.")))
}

async fn delete(ui: Ui, admin: Admin, State(db): State<Db>, Path(id): Path<i64>) -> Res<Redirect> {
    if id == admin.id {
        return Ok(ui
            .redirect(&format!("/users/{id}"))
            .danger("You cannot delete the account you are signed in with."));
    }
    let Some(user) = db::find_user(&db, id).await? else {
        return Ok(ui.redirect("/users").warn("That user was already gone."));
    };
    db::delete_user(&db, id).await?;
    db::log(&db, &admin.name, "deleted user", &user.email).await?;
    Ok(ui
        .redirect("/users")
        .ok(&format!("{} was deleted.", user.name)))
}
