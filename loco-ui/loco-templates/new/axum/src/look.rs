//! The app's look, on every page (the 404 included) through the layer in `router()`:
//! loco-ui's tokens, and the CSS of each component `cargo lui add` copied into `src/ui/`.
//! A brand colour is `tokens: Tokens { brand: .., ..Tokens::DEFAULT }`.

use loco_ui::layout::{Look, Tokens};
use maud::{Markup, html};

pub static LOOK: Look = Look {
    tokens: Tokens::DEFAULT,
    // `cargo lui add` appends each copied component's `CSS` here.
    css: &[],
    header: Some(no_header),
};

/// The app shell names the app, so pages carry no header above it.
fn no_header() -> Markup {
    html! {}
}
