//! # Container
//!
//! A centred column of a fixed greatest width: the readable middle of a page, a narrow form,
//! a wide dashboard. After Radix Themes Container: four widths (448, 688, 880 and 1136px),
//! size 4 by default, centred by auto margins, as wide as its parent below that.
//!
//! **Platform features:** `max-width` with `margin-inline: auto` (baseline 2010); `min()`
//! (Chrome 79, Firefox 75, Safari 11.1) keeps the column inside a narrower parent.
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
//! let m = ui.container().body(html! { p { "Centred" } }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-container"><p>Centred</p></div>"#);
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { Container { p { "Centred" } } };
//! assert_eq!(same.into_string(), m);
//! // Size 1: 448px, for a sign-in form.
//! let m = ui.container().size(1).body(html! { p { "Narrow" } }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-container" data-size="1"><p>Narrow</p></div>"#);
//! assert_eq!(lui! { Container size=1 { p { "Narrow" } } }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A centred column, made by [`Ui::container`].
///
/// **Setters.** Values: `.body(..)`, `.size(..)`.
#[derive(Clone, Debug)]
pub struct Container {
    content: Markup,
    size: u8,
}

impl Container {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup").doc("What the column holds."),
        Prop::new("size", PropKind::Value, "size: u8")
            .default("4")
            .doc("Radix's sizes 1–4: at most 448, 688, 880 or 1136px wide."),
    ];
}

impl Ui {
    /// A centred column at most 1136px wide; its content goes in [`Container::body`].
    pub fn container(&self) -> Container {
        Container {
            content: Markup::default(),
            size: 4,
        }
    }
}

impl Container {
    /// What the column holds.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// Radix's sizes 1–4 (clamped): at most 448, 688, 880 or 1136px (the default) wide.
    pub fn size(mut self, size: u8) -> Self {
        self.size = size.clamp(1, 4);
        self
    }
}

impl Render for Container {
    fn render(&self) -> Markup {
        let size = (self.size != 4).then_some(self.size);
        html! { div class="lui-container" data-size=[size] { (self.content) } }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Container: max widths 448, 688, 880 and 1136px, centred. */
.lui-container { box-sizing: border-box; width: 100%; max-width: min(71rem, 100%); margin-inline: auto; }
.lui-container[data-size="1"] { max-width: min(28rem, 100%); }
.lui-container[data-size="2"] { max-width: min(43rem, 100%); }
.lui-container[data-size="3"] { max-width: min(55rem, 100%); }
"#;
