//! # Center
//!
//! Its content in the middle of the space it is given, across and down: an empty state in a
//! panel, a spinner in a card, a sign-in form on a page. After Radix Themes Flex with
//! `align="center" justify="center"`, as one word.
//!
//! **Platform features:** `display: grid` with `place-items: center` (Chrome 59, Firefox 45,
//! Safari 11).
//!
//! **Accessibility:** layout only: a `<div>` with no role; reading order is source order.
//! Checked by axe-core in headless Firefox on every demo route, both capability variants,
//! light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.center().body(html! { p { "Nothing here yet" } }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-center"><p>Nothing here yet</p></div>"#);
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { Center { p { "Nothing here yet" } } };
//! assert_eq!(same.into_string(), m);
//! // At least 12rem tall, the text centred too.
//! let m = ui.center().min_height("12rem").text_center().body(html! { p { "Nothing here yet" } }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-center lui-center-text" style="min-height: 12rem"><p>Nothing here yet</p></div>"#);
//! assert_eq!(lui! { Center min_height="12rem" text_center { p { "Nothing here yet" } } }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// Centred content, made by [`Ui::center`].
///
/// **Setters.** Values: `.body(..)`, `.min_height(..)`; switches: `.text_center()`.
#[derive(Clone, Debug)]
pub struct Center<'a> {
    content: Markup,
    min_height: Option<&'a str>,
    text: bool,
}

impl Center<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup").doc("What is centred."),
        Prop::new("min_height", PropKind::Value, "height: &'a str")
            .doc("At least this tall (a CSS length), so there is room to centre in."),
        Prop::new("text_center", PropKind::Switch, "").doc("Centre the lines of text too."),
    ];
}

impl Ui {
    /// Its content in the middle of the space it is given; the content goes in
    /// [`Center::body`].
    pub fn center<'a>(&self) -> Center<'a> {
        Center {
            content: Markup::default(),
            min_height: None,
            text: false,
        }
    }
}

impl<'a> Center<'a> {
    /// What is centred.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// At least this tall (a CSS length: `12rem`, `100dvh`), so there is room to centre in.
    pub fn min_height(mut self, height: &'a str) -> Self {
        self.min_height = Some(height);
        self
    }

    /// Centre the lines of text too (`text-align: center`).
    pub fn text_center(mut self) -> Self {
        self.text = true;
        self
    }
}

impl Render for Center<'_> {
    fn render(&self) -> Markup {
        let class = if self.text {
            "lui-center lui-center-text"
        } else {
            "lui-center"
        };
        let style = self.min_height.map(|h| format!("min-height: {h}"));
        html! { div class=(class) style=[style] { (self.content) } }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Flex align=center justify=center: both axes centred, the children in a
   column 0.5rem apart. */
.lui-center { display: grid; place-items: center; align-content: center; gap: var(--lui-space-2); min-width: 0; }
.lui-center > * { margin: 0; max-width: 100%; }
.lui-center-text { text-align: center; }
"#;
