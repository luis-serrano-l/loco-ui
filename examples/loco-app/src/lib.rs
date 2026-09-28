//! A notes app on Loco and loco-ui: notebooks, tagged and pinned notes, a task board, every
//! page working with script off. Notebooks and tasks were written by `cargo loco generate
//! scaffold` with the templates in `.loco-templates/` (copied from `loco-ui/loco-templates/`)
//! and then edited; notes, the overview and the app shell are written by hand; the account
//! pages by `cargo lui auth` on the starter's `users` model and `AuthMailer`.

pub mod app;
pub mod controllers;
pub mod mailers;
pub mod models;
pub mod seed;
pub mod views;
