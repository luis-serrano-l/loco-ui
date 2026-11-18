//! The audit log: who did what, newest first, with a search and a filter by action.

use axum::{Router, extract::State, routing::get};
use loco_ui::prelude::*;

use crate::{Db, Res, auth::Admin, db, views};

pub fn routes() -> Router<Db> {
    Router::new().route("/audit", get(index))
}

const FILTERS: [(&str, &str, &str); 4] = [
    ("All", "/audit", ""),
    ("Sign-ins", "/audit?action=signed+in", "signed in"),
    (
        "Failed sign-ins",
        "/audit?action=failed+sign-in",
        "failed sign-in",
    ),
    (
        "Deleted users",
        "/audit?action=deleted+user",
        "deleted user",
    ),
];

async fn index(ui: Ui, admin: Admin, State(db): State<Db>) -> Res<Page> {
    let (q, action) = (
        ui.param("q").unwrap_or("").trim(),
        ui.param("action").unwrap_or(""),
    );
    let entries = db::audit(&db, q, action).await?;
    let count = format!("{} entries", entries.len());
    let rows = (entries.iter()).map(|e| {
        (
            e.at.as_str(),
            e.actor.as_str(),
            ui.badge(&e.action).render(),
            e.target.as_str(),
        )
    });
    let table = ui
        .table("audit", "/audit")
        .hide_search()
        .caption("Audit log")
        .column("at", "When (UTC)")
        .column("actor", "Who")
        .column("action", "Did")
        .column("target", "To")
        .priority(2)
        .rows(rows);
    let body = html! {
        @if entries.is_empty() { (ui.empty_state("Nothing matches").link("Show everything", "/audit")) }
        @else { (table) }
    };
    let list = ui
        .list_page("Audit log")
        .description("The newest 200 entries. Nothing here can be edited.")
        .search("Search the log")
        .count(&count);
    let list = FILTERS
        .iter()
        .fold(list, |list, (label, href, _)| list.filter(label, href));
    Ok(views::shell(
        &ui,
        &admin,
        "Audit log",
        list.body(body).render(),
    ))
}
