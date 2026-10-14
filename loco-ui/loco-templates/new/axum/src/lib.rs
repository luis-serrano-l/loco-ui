//! {{name}}: an Axum app on loco-ui, written by `cargo lui new`. Every page works with
//! JavaScript off; `/lui/enhance.js` (optional) updates parts of them in place.

pub mod look;
pub mod pages;

use axum::{
    Router,
    routing::{get, post},
};
use loco_ui::{blocks, caps, enhance};

/// Every route: the pages, the theme post, loco-ui's script and capability beacon, the 404
/// page for every other path, the app's look on all of them and the strict
/// Content-Security-Policy on every HTML answer.
pub fn router() -> Router {
    Router::new()
        .route("/", get(pages::home))
        .route("/settings", get(pages::settings))
        .route("/theme", post(pages::theme))
        .merge(caps::router())
        .merge(enhance::router())
        .fallback(blocks::not_found)
        .layer(look::LOOK.layer())
        .layer(axum::middleware::from_fn(enhance::csp))
}
