//! Who is signed in, and what the sidebar lists for them: shared by every signed-in page.
use loco_rs::prelude::*;
use sea_orm::{
    QuerySelect,
    sea_query::{Expr, ExprTrait},
};

use crate::{
    models::{
        _entities::{notebooks, notes, tasks},
        users,
    },
    views::shell::Nav,
};

/// The signed-in user, whose rows every handler reads and writes. A session whose user is
/// gone (deleted, or a database reset since the cookie was set) is treated as signed out: 401,
/// as with no cookie at all, which `loco_ui::loco::SignIn` turns into the sign-in form.
pub async fn owner(ctx: &AppContext, auth: &auth::JWT) -> Result<users::Model> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(user) => Ok(user),
        Err(ModelError::EntityNotFound) => Err(Error::Unauthorized("no such user".into())),
        Err(e) => Err(e.into()),
    }
}

/// The counts and notebooks the sidebar shows to `me`.
pub async fn nav(ctx: &AppContext, me: &users::Model) -> Result<Nav> {
    let db = &ctx.db;
    let mine = || notes::Entity::find().filter(notes::Column::UserId.eq(me.id));
    let live = || mine().filter(notes::Column::Archived.eq(false));
    let per_book: Vec<(Option<i64>, i64)> = live()
        .select_only()
        .column(notes::Column::NotebookId)
        .column_as(Expr::col(notes::Column::Id).count(), "n")
        .group_by(notes::Column::NotebookId)
        .into_tuple()
        .all(db)
        .await?;
    let count = |id: i64| {
        (per_book.iter())
            .find(|(b, _)| *b == Some(id))
            .map_or(0, |(_, n)| u64::try_from(*n).unwrap_or(0))
    };
    let notebooks = notebooks::Entity::find()
        .filter(notebooks::Column::UserId.eq(me.id))
        .order_by_asc(notebooks::Column::Name)
        .all(db)
        .await?;
    Ok(Nav {
        user: me.name.clone(),
        notes: live().count(db).await?,
        pinned: live()
            .filter(notes::Column::Pinned.eq(true))
            .count(db)
            .await?,
        archived: mine()
            .filter(notes::Column::Archived.eq(true))
            .count(db)
            .await?,
        open_tasks: tasks::Entity::find()
            .filter(tasks::Column::UserId.eq(me.id))
            .filter(tasks::Column::Status.ne("done"))
            .count(db)
            .await?,
        notebooks: (notebooks.into_iter())
            .map(|b| (b.id, b.name, count(b.id)))
            .collect(),
    })
}
