//! The front page: a landing page when signed out, the overview when signed in; and the
//! theme toggle every page posts to.
use axum::http::{HeaderMap, header};
use chrono::{Datelike, Duration, Utc};
use loco_rs::prelude::*;
use loco_ui::prelude::*;
use serde::Deserialize;

use crate::{
    controllers::session::{nav, owner},
    models::{_entities::tasks, notes::Entity},
    views::{self, shell},
};

#[debug_handler]
async fn index(auth: Option<auth::JWT>, ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    // No session, or one whose user is gone: the landing page.
    let me = match auth {
        Some(auth) => owner(&ctx, &auth).await.ok(),
        None => None,
    };
    let Some(me) = me else {
        return Ok(ui.page("Notes", views::home::landing(&ui)));
    };
    let notes = Entity::listed(&ctx.db, me.id, |s| s).await?;
    // Notes written in each of the last eight weeks, oldest first, labelled by the Monday.
    let monday = Utc::now().date_naive()
        - Duration::days(Utc::now().weekday().num_days_from_monday().into());
    let weeks: Vec<(String, f64)> = (0..8)
        .rev()
        .map(|w| {
            let start = monday - Duration::weeks(w);
            let n = (notes.iter())
                .filter(|it| {
                    (start..start + Duration::weeks(1)).contains(&it.note.created_at.date_naive())
                })
                .count();
            (start.format("%-d %b").to_string(), n as f64)
        })
        .collect();
    let mut due = tasks::Entity::find()
        .filter(tasks::Column::UserId.eq(me.id))
        .filter(tasks::Column::Status.ne("done"))
        .all(&ctx.db)
        .await?;
    due.sort_by_key(|t| (t.due_on.is_none(), t.due_on));
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
async fn theme(ui: Ui, headers: HeaderMap, Form(f): Form<ThemeForm>) -> Redirect {
    let back = (headers.get(header::REFERER).and_then(|v| v.to_str().ok()))
        .and_then(|r| r.find("://").map(|i| &r[i + 3..]))
        .and_then(|r| r.find('/').map(|i| r[i..].to_string()))
        .unwrap_or_else(|| "/".into());
    ui.redirect(&back).theme(Theme::parse(&f.theme))
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(index))
        .add("/theme", post(theme))
}
