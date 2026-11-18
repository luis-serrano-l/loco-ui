//! An admin panel on plain Axum and SQLite, with loco-ui for every page: sign-in, users
//! (list with search and filters, record, forms), roles, an audit log and settings. No Loco,
//! no ORM and no script of its own: each change is a form post and a redirect.
//!
//! - [`db`]: the schema, the seed and every query.
//! - [`auth`]: password hashing, the session cookie and the [`auth::Admin`] extractor.
//! - [`views`]: the shell every signed-in page sits in.
//! - [`users`], [`roles`], [`audit`], [`settings`]: one module per area, handlers and pages.
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]

pub mod audit;
pub mod auth;
pub mod db;
pub mod roles;
pub mod settings;
pub mod users;
pub mod views;

use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

pub use db::Db;

/// The whole app over one database.
pub fn router(db: Db) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { axum::response::Redirect::to("/users") }),
        )
        .merge(auth::routes())
        .merge(users::routes())
        .merge(roles::routes())
        .merge(audit::routes())
        .merge(settings::routes())
        .with_state(db)
        .merge(loco_ui::caps::router())
        .merge(loco_ui::enhance::router())
        .fallback(loco_ui::blocks::not_found)
        .layer(axum::middleware::from_fn(loco_ui::enhance::slim))
        .layer(axum::middleware::from_fn(loco_ui::enhance::csp))
}

/// Why a handler has no page of its own to give: the database said no (logged, a plain
/// 500), or the record asked for is not there (a 404 page in the shell).
pub enum Failure {
    Db(sqlx::Error),
    Missing(Box<loco_ui::Page>),
}

impl From<sqlx::Error> for Failure {
    fn from(e: sqlx::Error) -> Self {
        Failure::Db(e)
    }
}

impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        match self {
            Failure::Db(e) => {
                eprintln!("database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.").into_response()
            }
            Failure::Missing(page) => (StatusCode::NOT_FOUND, *page).into_response(),
        }
    }
}

/// What a handler returns.
pub type Res<T> = Result<T, Failure>;
