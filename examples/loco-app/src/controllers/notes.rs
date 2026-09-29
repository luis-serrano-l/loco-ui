//! Note pages: the lists (all, pinned, archive, filtered by tag or text), the reading view,
//! new, edit, pin, archive and delete, as plain HTML forms with Post/Redirect/Get. Started as
//! scaffold output, then written by hand.
//!
//! Every note belongs to a user: each query is scoped to the signed-in user, a new note's
//! `user_id` comes from the session, and a notebook from the form is kept only if it is
//! theirs. Another user's note answers 404.
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use loco_ui::{loco::Valid, prelude::*};
use sea_orm::{Condition, Select, prelude::Date};
use serde::Deserialize;

use crate::{
    controllers::session::{nav, owner},
    models::{
        _entities::{notebooks, tags},
        notes::{ActiveModel, Column, Entity, Listed},
        users,
    },
    views::{self, shell},
};

/// The posted fields; `Valid<Params>` gives back a message per field in error.
#[derive(Deserialize, Validate)]
struct Params {
    title: String,
    body: Option<String>,
    #[serde(default)]
    notebook_id: String,
    #[serde(default)]
    tags: String,
    #[serde(default, deserialize_with = "loco_ui::loco::checkbox")]
    pinned: bool,
    due: Option<Date>,
}

/// The user's notebooks as `(id, name)`, for the form's select.
async fn books(ctx: &AppContext, me: &users::Model) -> Result<Vec<(i64, String)>> {
    Ok(notebooks::Entity::find()
        .filter(notebooks::Column::UserId.eq(me.id))
        .order_by_asc(notebooks::Column::Name)
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|b| (b.id, b.name))
        .collect())
}

/// Note `id` of `me`, with its tags and notebook, else 404.
async fn load(ctx: &AppContext, me: &users::Model, id: i64) -> Result<Listed> {
    Entity::listed(&ctx.db, me.id, |query| query.filter(Column::Id.eq(id)))
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// Save `params` into `item` for `me`; a notebook that is not theirs is dropped.
async fn save(
    ctx: &AppContext,
    me: &users::Model,
    mut item: ActiveModel,
    params: Params,
) -> Result<i64> {
    let book = params.notebook_id.parse::<i64>().ok();
    let theirs = books(ctx, me)
        .await?
        .iter()
        .any(|(id, _)| Some(*id) == book);
    item.title = Set(params.title);
    item.body = Set(params
        .body
        .filter(|b| !b.trim().is_empty())
        .map(|b| b.replace("\r\n", "\n")));
    item.notebook_id = Set(book.filter(|_| theirs));
    item.pinned = Set(params.pinned);
    item.due = Set(params.due);
    item.user_id = Set(me.id);
    let saved = item.save(&ctx.db).await?;
    let id = sea_orm::TryIntoModel::try_into_model(saved)?.id;
    Entity::set_tags(&ctx.db, me.id, id, &params.tags).await?;
    Ok(id)
}

/// A list page: `which` narrows the notes, `?q=` searches, `?sel=` keeps notes with a tag.
async fn listing(
    auth: auth::JWT,
    ui: Ui,
    ctx: AppContext,
    heading: &str,
    path: &str,
    which: impl FnOnce(Select<Entity>) -> Select<Entity>,
) -> Result<Page> {
    let me = owner(&ctx, &auth).await?;
    let query = ui.param("q").unwrap_or("").trim().to_string();
    let found = Entity::listed(&ctx.db, me.id, |select| {
        let select = which(select);
        if query.is_empty() {
            return select;
        }
        select.filter(
            Condition::any()
                .add(Column::Title.contains(&query))
                .add(Column::Body.contains(&query)),
        )
    })
    .await?;
    let chosen: Vec<&str> = ui.params("sel").collect();
    let items: Vec<Listed> = (found.into_iter())
        .filter(|listed| {
            chosen
                .iter()
                .all(|wanted| listed.tags.iter().any(|tag| tag == wanted))
        })
        .collect();
    let all_tags: Vec<String> = tags::Entity::find()
        .filter(tags::Column::UserId.eq(me.id))
        .order_by_asc(tags::Column::Name)
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|t| t.name)
        .collect();
    let heading = if query.is_empty() {
        heading.to_string()
    } else {
        format!("“{query}”")
    };
    let body = views::notes::list(&ui, &heading, path, &items, &all_tags);
    Ok(shell::page(&ui, &nav(&ctx, &me).await?, &heading, body))
}

#[debug_handler]
async fn list(auth: auth::JWT, ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    listing(auth, ui, ctx, "All notes", "/notes", |s| {
        s.filter(Column::Archived.eq(false))
    })
    .await
}

#[debug_handler]
async fn pinned(auth: auth::JWT, ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    listing(auth, ui, ctx, "Pinned", "/notes/pinned", |s| {
        s.filter(Column::Archived.eq(false))
            .filter(Column::Pinned.eq(true))
    })
    .await
}

#[debug_handler]
async fn archive(auth: auth::JWT, ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    listing(auth, ui, ctx, "Archive", "/notes/archive", |s| {
        s.filter(Column::Archived.eq(true))
    })
    .await
}

#[debug_handler]
async fn show(
    auth: auth::JWT,
    ui: Ui,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Page> {
    let me = owner(&ctx, &auth).await?;
    let listed = load(&ctx, &me, id).await?;
    let body = views::notes::show(&ui, &listed);
    Ok(shell::page(
        &ui,
        &nav(&ctx, &me).await?,
        &listed.note.title,
        body,
    ))
}

#[debug_handler]
async fn new(auth: auth::JWT, ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    let me = owner(&ctx, &auth).await?;
    let book = ui.param("notebook").unwrap_or("").to_string();
    let values = [("notebook_id".to_string(), book)];
    let form = views::notes::form(
        &ui,
        "New note",
        "/notes",
        &values,
        &[],
        &books(&ctx, &me).await?,
    );
    Ok(shell::page(&ui, &nav(&ctx, &me).await?, "New note", form))
}

#[debug_handler]
async fn create(
    auth: auth::JWT,
    ui: Ui,
    State(ctx): State<AppContext>,
    Valid(form): Valid<Params>,
) -> Result<Response> {
    let me = owner(&ctx, &auth).await?;
    let params = match form {
        Ok(params) => params,
        Err(bad) => {
            let body = views::notes::form(
                &ui,
                "New note",
                "/notes",
                &bad.values,
                &bad.errors.pairs(),
                &books(&ctx, &me).await?,
            );
            return Ok(shell::page(&ui, &nav(&ctx, &me).await?, "New note", body)
                .invalid()
                .into_response());
        }
    };
    let id = save(&ctx, &me, <ActiveModel as Default>::default(), params).await?;
    Ok(ui
        .redirect(&format!("/notes/{id}"))
        .ok("Note created.")
        .into_response())
}

#[debug_handler]
async fn edit(
    auth: auth::JWT,
    ui: Ui,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Page> {
    let me = owner(&ctx, &auth).await?;
    let listed = load(&ctx, &me, id).await?;
    let action = format!("/notes/{id}");
    let form = views::notes::form(
        &ui,
        "Edit note",
        &action,
        &views::notes::values(&listed),
        &[],
        &books(&ctx, &me).await?,
    );
    Ok(shell::page(&ui, &nav(&ctx, &me).await?, "Edit note", form))
}

#[debug_handler]
async fn update(
    auth: auth::JWT,
    ui: Ui,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
    Valid(form): Valid<Params>,
) -> Result<Response> {
    let me = owner(&ctx, &auth).await?;
    let listed = load(&ctx, &me, id).await?;
    let action = format!("/notes/{id}");
    let params = match form {
        Ok(params) => params,
        Err(bad) => {
            let body = views::notes::form(
                &ui,
                "Edit note",
                &action,
                &bad.values,
                &bad.errors.pairs(),
                &books(&ctx, &me).await?,
            );
            return Ok(shell::page(&ui, &nav(&ctx, &me).await?, "Edit note", body)
                .invalid()
                .into_response());
        }
    };
    save(&ctx, &me, listed.note.into_active_model(), params).await?;
    Ok(ui.redirect(&action).ok("Saved.").into_response())
}

#[debug_handler]
async fn pin(
    auth: auth::JWT,
    ui: Ui,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Redirect> {
    let me = owner(&ctx, &auth).await?;
    let note = load(&ctx, &me, id).await?.note;
    let now = !note.pinned;
    let mut item = note.into_active_model();
    item.pinned = Set(now);
    item.update(&ctx.db).await?;
    Ok(ui
        .redirect(&format!("/notes/{id}"))
        .ok(if now { "Pinned." } else { "Unpinned." }))
}

#[debug_handler]
async fn archive_one(
    auth: auth::JWT,
    ui: Ui,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Redirect> {
    let me = owner(&ctx, &auth).await?;
    let note = load(&ctx, &me, id).await?.note;
    let now = !note.archived;
    let mut item = note.into_active_model();
    item.archived = Set(now);
    item.update(&ctx.db).await?;
    Ok(ui.redirect(&format!("/notes/{id}")).ok(if now {
        "Archived."
    } else {
        "Back from the archive."
    }))
}

#[debug_handler]
async fn remove(
    auth: auth::JWT,
    ui: Ui,
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Redirect> {
    let me = owner(&ctx, &auth).await?;
    load(&ctx, &me, id)
        .await?
        .note
        .into_active_model()
        .delete(&ctx.db)
        .await?;
    Ok(ui.redirect("/notes").ok("Note deleted."))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/notes")
        .add("/", get(list).post(create))
        .add("/pinned", get(pinned))
        .add("/archive", get(archive))
        .add("/new", get(new))
        .add("/{id}", get(show).post(update))
        .add("/{id}/edit", get(edit))
        .add("/{id}/pin", post(pin))
        .add("/{id}/archive", post(archive_one))
        .add("/{id}/delete", post(remove))
}
