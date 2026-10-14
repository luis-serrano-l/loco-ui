//! The app's look, on every page (the account pages and the 404 included) through the layer
//! in `App::after_routes`: loco-ui's tokens, and the CSS of each component `cargo lui add`
//! copied into `src/ui/`. Written by `cargo lui install`; edit freely: a brand colour is
//! `tokens: Tokens { brand: .., ..Tokens::DEFAULT }` (see `loco_ui::layout::Scale::derive`).

use loco_ui::layout::{Look, Tokens};

pub static LOOK: Look = Look {
    tokens: Tokens::DEFAULT,
    // `cargo lui add` appends each copied component's `CSS` here.
    css: &[],
    header: None,
};
