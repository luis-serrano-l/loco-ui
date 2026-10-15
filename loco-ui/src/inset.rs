//! # Inset
//!
//! Content that reaches the edges of the card it sits in: a cover picture across the top, a
//! map, a table with no margin around it. After Radix Themes Inset: it cancels the card's
//! padding with negative margins on the sides it names, and rounds the corners it shares
//! with the card.
//!
//! **Platform features:** negative `margin-inline` and `margin-block` against the card's
//! padding (baseline); `overflow: clip` (Chrome 90, Firefox 81, Safari 16) so a picture
//! takes the card's rounded corners.
//!
//! **Accessibility:** layout only: a `<div>` with no role. Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** without `overflow: clip` the corners are cut by `overflow: hidden`, the same
//! to the eye.
//!
//! An inset bleeds through a card's sides by default. `.top()` and `.bottom()` add the
//! edges; they work when the inset is the first (or last) thing in a card with no header
//! (or footer), as Radix's do.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.inset().body(html! { img src="/cover.jpg" alt=""; }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-inset"><img src="/cover.jpg" alt=""></div>"#);
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { Inset { img src="/cover.jpg" alt=""; } };
//! assert_eq!(same.into_string(), m);
//! // A cover picture across the top of a card.
//! let m = ui.inset().top().body(html! { img src="/cover.jpg" alt=""; }).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-inset lui-inset-top">"#));
//! assert_eq!(lui! { Inset top { img src="/cover.jpg" alt=""; } }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// Content that bleeds to its card's edges, made by [`Ui::inset`].
///
/// **Setters.** Values: `.body(..)`; switches: `.top()`, `.bottom()`.
#[derive(Clone, Debug)]
pub struct Inset {
    content: Markup,
    top: bool,
    bottom: bool,
}

impl Inset {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup")
            .doc("What reaches the edges: a picture, a map, a table."),
        Prop::new("top", PropKind::Switch, "")
            .doc("Bleed through the card's top edge too (first in a card with no header)."),
        Prop::new("bottom", PropKind::Switch, "")
            .doc("Bleed through the card's bottom edge too (last in a card with no footer)."),
    ];
}

impl Ui {
    /// Content that reaches the sides of the card it sits in; it goes in [`Inset::body`].
    pub fn inset(&self) -> Inset {
        Inset {
            content: Markup::default(),
            top: false,
            bottom: false,
        }
    }
}

impl Inset {
    /// What reaches the edges: a picture, a map, a table.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// Bleed through the card's top edge too, with its rounded corners; for an inset that is
    /// the first thing in a card with no header.
    pub fn top(mut self) -> Self {
        self.top = true;
        self
    }

    /// Bleed through the card's bottom edge too; for an inset that is the last thing in a card
    /// with no footer.
    pub fn bottom(mut self) -> Self {
        self.bottom = true;
        self
    }
}

impl Render for Inset {
    fn render(&self) -> Markup {
        let class = match (self.top, self.bottom) {
            (false, false) => "lui-inset",
            (true, false) => "lui-inset lui-inset-top",
            (false, true) => "lui-inset lui-inset-bottom",
            (true, true) => "lui-inset lui-inset-top lui-inset-bottom",
        };
        html! { div class=(class) { (self.content) } }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Inset: margins of minus the card's padding (1.5rem at the default size,
   1rem and 2rem at sizes 1 and 3), the shared corners at the card's radius less its 1px
   border. Outside a card the inset has no padding to cancel and sits as a plain block. */
.lui-inset { --lui-inset-pad: 0px; overflow: hidden; overflow: clip; margin-inline: calc(-1 * var(--lui-inset-pad)); }
.lui-inset > :is(img, video, svg, iframe) { display: block; width: 100%; height: auto; object-fit: cover; }
.lui-card > .lui-card-body > .lui-inset { --lui-inset-pad: 1.5rem; }
.lui-card[data-size="1"] > .lui-card-body > .lui-inset { --lui-inset-pad: 1rem; }
.lui-card[data-size="3"] > .lui-card-body > .lui-inset { --lui-inset-pad: 2rem; }
/* A card inside a card has no inline padding (card.rs), so there is nothing to cancel. */
.lui-card .lui-card > .lui-card-body > .lui-inset { --lui-inset-pad: 0px; }
.lui-card-body > .lui-inset.lui-inset-top { margin-top: calc(-1 * var(--lui-inset-pad)); border-start-start-radius: calc(var(--lui-radius-lg) - 1px); border-start-end-radius: calc(var(--lui-radius-lg) - 1px); }
.lui-card-body > .lui-inset.lui-inset-bottom { margin-bottom: calc(-1 * var(--lui-inset-pad)); border-end-start-radius: calc(var(--lui-radius-lg) - 1px); border-end-end-radius: calc(var(--lui-radius-lg) - 1px); }
"#;
