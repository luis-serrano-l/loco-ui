//! # Section
//!
//! A band of a page with room above and below it: the hero, the features, the pricing, each
//! one `<section>` apart from the next. After Radix Themes Section: four heights of vertical
//! padding (24, 40, 64 and 80px), size 3 by default.
//!
//! **Platform features:** `<section>` (baseline 2011) and `padding-block` (Chrome 87,
//! Firefox 66, Safari 14.1).
//!
//! **Accessibility:** a `<section>` with no name is a generic block to assistive tech; give it
//! a heading inside (a `ui.heading(..)`) and `.label(..)` to make it a named region in the
//! page's landmarks. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.section().body(html! { h2 { "Features" } }).render().into_string();
//! assert_eq!(m, r#"<section class="lui-section"><h2>Features</h2></section>"#);
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { Section { h2 { "Features" } } };
//! assert_eq!(same.into_string(), m);
//! // The tallest band, named for the landmarks list.
//! let m = ui.section().size(4).label("Pricing").body(html! { h2 { "Pricing" } }).render().into_string();
//! assert_eq!(m, r#"<section class="lui-section" data-size="4" aria-label="Pricing"><h2>Pricing</h2></section>"#);
//! assert_eq!(lui! { Section size=4 label="Pricing" { h2 { "Pricing" } } }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A band of a page, made by [`Ui::section`].
///
/// **Setters.** Values: `.body(..)`, `.size(..)`, `.label(..)`.
#[derive(Clone, Debug)]
pub struct Section<'a> {
    content: Markup,
    size: u8,
    label: Option<&'a str>,
}

impl Section<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup").doc("What the band holds."),
        Prop::new("size", PropKind::Value, "size: u8")
            .default("3")
            .doc("Radix's sizes 1–4: 24, 40, 64 or 80px above and below."),
        Prop::new("label", PropKind::Value, "label: &'a str")
            .attr("aria-label")
            .doc("A name (`aria-label`), which makes the section a region landmark."),
    ];
}

impl Ui {
    /// A band of the page, 64px of room above and below; its content goes in
    /// [`Section::body`].
    pub fn section<'a>(&self) -> Section<'a> {
        Section {
            content: Markup::default(),
            size: 3,
            label: None,
        }
    }
}

impl<'a> Section<'a> {
    /// What the band holds.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// Radix's sizes 1–4 (clamped): 24, 40, 64 (the default) or 80px above and below.
    pub fn size(mut self, size: u8) -> Self {
        self.size = size.clamp(1, 4);
        self
    }

    /// A name (`aria-label`), which makes the section a region landmark; usually the text of
    /// its heading.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

impl Render for Section<'_> {
    fn render(&self) -> Markup {
        let size = (self.size != 3).then_some(self.size);
        html! { section class="lui-section" data-size=[size] aria-label=[self.label] { (self.content) } }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Section: padding-block 24, 40, 64 or 80px. */
.lui-section { box-sizing: border-box; padding-block: 4rem; }
.lui-section[data-size="1"] { padding-block: 1.5rem; }
.lui-section[data-size="2"] { padding-block: 2.5rem; }
.lui-section[data-size="4"] { padding-block: 5rem; }
"#;
