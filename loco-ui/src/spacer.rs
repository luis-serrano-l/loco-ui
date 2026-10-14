//! # Spacer
//!
//! An empty part of a row or column that takes the room left over, pushing what follows it to
//! the far end: the account menu to the right of a toolbar, a footer to the bottom of a
//! sidebar. After Radix Themes Box with `flexGrow="1"`, the way Radix's own examples push
//! a toolbar apart (Radix has no spacer component).
//!
//! **Platform features:** `flex: 1 1 auto` (Chrome 29, Firefox 28, Safari 9) inside a flex
//! row or column (a `ui.cluster()`, a `ui.stack()`).
//!
//! **Accessibility:** `aria-hidden`: it holds nothing. Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** outside a flex container it is an empty block of no height.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.spacer().render().into_string();
//! assert_eq!(m, r#"<div class="lui-spacer" aria-hidden="true"></div>"#);
//! // The same in `lui!`:
//! let same = lui! { Spacer; };
//! assert_eq!(same.into_string(), m);
//! // At least 2rem, even when the row is full.
//! let m = ui.spacer().min("2rem").render().into_string();
//! assert_eq!(m, r#"<div class="lui-spacer" aria-hidden="true" style="min-width: 2rem; min-height: 2rem"></div>"#);
//! assert_eq!(lui! { Spacer min="2rem"; }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// An empty part that takes the room left over, made by [`Ui::spacer`].
///
/// **Setters.** Values: `.min(..)`.
#[derive(Clone, Debug)]
pub struct Spacer<'a> {
    min: Option<&'a str>,
}

impl Spacer<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[Prop::new("min", PropKind::Value, "length: &'a str")
        .doc("The least room it keeps (a CSS length), even when the row is full.")];
}

impl Ui {
    /// An empty part of a flex row or column that takes the room left over.
    pub fn spacer<'a>(&self) -> Spacer<'a> {
        Spacer { min: None }
    }
}

impl<'a> Spacer<'a> {
    /// The least room it keeps (a CSS length: `2rem`), even when the row is full.
    pub fn min(mut self, length: &'a str) -> Self {
        self.min = Some(length);
        self
    }
}

impl Render for Spacer<'_> {
    fn render(&self) -> Markup {
        let style = self
            .min
            .map(|m| format!("min-width: {m}; min-height: {m}"));
        html! { div class="lui-spacer" aria-hidden="true" style=[style] {} }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* A flex item that grows into the free space and shrinks to nothing (or to its minimum). */
.lui-spacer { flex: 1 1 auto; align-self: stretch; min-width: 0; min-height: 0; }
"#;
