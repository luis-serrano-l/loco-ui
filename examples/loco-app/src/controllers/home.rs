//! The front page: a landing page when signed out, the overview when signed in; and the
//! theme toggle every page posts to.
use axum::http::{HeaderMap, header};
use chrono::{Datelike, Duration, Utc};
use loco_rs::prelude::*;
use loco_ui::prelude::*;
use serde::Deserialize;

use crate::{
    controllers::session::{nav, owner},
    models::{
        _entities::tasks,
        notes::{Entity, Listed},
    },
    views::{self, shell},
};

#[debug_handler]
async fn index(auth: Option<auth::JWT>, ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    // A session whose user is gone is signed out; a database error is still an error.
    let me = match auth {
        Some(auth) => match owner(&ctx, &auth).await {
            Ok(user) => Some(user),
            Err(Error::Unauthorized(_)) => None,
            Err(e) => return Err(e),
        },
        None => None,
    };
    let Some(me) = me else {
        return Ok(ui.page("Notes", views::home::landing(&ui)));
    };
    let notes = Entity::listed(&ctx.db, me.id, |s| s).await?;
    let weeks = notes_per_week(&notes);
    let mut due = tasks::Entity::find()
        .filter(tasks::Column::UserId.eq(me.id))
        .filter(tasks::Column::Status.ne("done"))
        .all(&ctx.db)
        .await?;
    due.sort_by_key(|task| (task.due_on.is_none(), task.due_on));
    let nav = nav(&ctx, &me).await?;
    let body = views::home::overview(&ui, &nav, &notes, &weeks, &due);
    Ok(shell::page(&ui, &nav, "Overview", body))
}

#[derive(Deserialize)]
struct ThemeForm {
    theme: String,
}

/// Keep the picked theme and go back to the page the toggle was on.
#[debug_handler]
async fn theme(ui: Ui, headers: HeaderMap, Form(form): Form<ThemeForm>) -> Redirect {
    let back = (headers
        .get(header::REFERER)
        .and_then(|value| value.to_str().ok()))
    .and_then(|referer| referer.find("://").map(|i| &referer[i + 3..]))
    .and_then(|rest| rest.find('/').map(|i| rest[i..].to_string()))
    .unwrap_or_else(|| "/".into());
    ui.redirect(&back).theme(Theme::parse(&form.theme))
}

/// Notes written in each of the last eight weeks, oldest first, labelled by the Monday.
fn notes_per_week(notes: &[Listed]) -> Vec<(String, f64)> {
    let monday = Utc::now().date_naive()
        - Duration::days(Utc::now().weekday().num_days_from_monday().into());
    (0..8)
        .rev()
        .map(|week| {
            let start = monday - Duration::weeks(week);
            let count = (notes.iter())
                .filter(|listed| {
                    (start..start + Duration::weeks(1))
                        .contains(&listed.note.created_at.date_naive())
                })
                .count();
            (start.format("%-d %b").to_string(), count as f64)
        })
        .collect()
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(index))
        .add("/theme", post(theme))
}
