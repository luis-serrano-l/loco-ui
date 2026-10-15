//! # Heading
//!
//! A section title at one of nine sizes, its level chosen apart from its size so the outline
//! stays right when the look calls for a small `<h1>` or a large `<h3>`. After Radix Themes
//! Heading: sizes 1 to 9 on Radix's type scale (12 to 60px), size 6 and bold by default,
//! the letters drawn tighter as they grow, and `.trim()` for the space above the capitals
//! and below the baseline.
//!
//! **Platform features:** `<h1>` to `<h6>` (baseline 1995); `text-wrap: balance` (Chrome 114,
//! Firefox 121, Safari 17.5) so a long title breaks into even lines; `.trim()` uses
//! `text-box` (Chrome 133, Safari 18.2, not Firefox yet) under `@supports`.
//!
//! **Accessibility:** a real heading element, so screen readers list it and jump to it; the
//! level is `.as_h(..)`, never implied by the size. Checked by axe-core in headless Firefox on
//! every demo route, both capability variants, light and dark (no serious or critical
//! violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** without `text-box` a trimmed heading keeps its line's half-leading above and
//! below; without `text-wrap: balance` the lines break greedily.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let h = ui.heading("Projects").render().into_string();
//! assert_eq!(h, r#"<h1 class="lui-heading" data-size="6">Projects</h1>"#);
//! // The same in `lui!`:
//! let same = lui! { Heading("Projects"); };
//! assert_eq!(same.into_string(), h);
//! // A small second-level heading in medium weight, its line box trimmed.
//! let h = ui.heading("Members").size(3).as_h(2).weight(Weight::Medium).trim().render().into_string();
//! assert_eq!(h, r#"<h2 class="lui-heading lui-heading-trim" data-size="3" data-weight="medium">Members</h2>"#);
//! assert_eq!(lui! { Heading("Members") size=3 as_h=2 weight="medium" trim; }.into_string(), h);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};
use crate::variant::Weight;

/// A heading, made by [`Ui::heading`].
///
/// **Setters.** Values: `.size(..)`, `.as_h(..)`, `.weight(..)`; switches: `.trim()`.
#[derive(Clone, Debug)]
pub struct Heading<'a> {
    text: &'a str,
    size: u8,
    level: u8,
    weight: Option<Weight>,
    trim: bool,
}

impl Heading<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("size", PropKind::Value, "size: u8")
            .default("6")
            .doc("Radix's sizes 1–9: 12, 14, 16, 18, 20, 24, 28, 35 or 60px."),
        Prop::new("as_h", PropKind::Number, "level: u8")
            .default("1")
            .doc("The element, `<h1>` to `<h6>`: the level in the page's outline."),
        Prop::new(
            "weight",
            PropKind::Choice { of: Weight::NAMES },
            "weight: Weight",
        )
        .default("bold")
        .doc("How heavy the letters are."),
        Prop::new("trim", PropKind::Switch, "")
            .doc("Trim the space above the capitals and below the baseline."),
    ];
}

impl Ui {
    /// A heading reading `text`: an `<h1>` at size 6 until `.as_h(..)` and `.size(..)` say
    /// otherwise.
    pub fn heading<'a>(&self, text: &'a str) -> Heading<'a> {
        Heading {
            text,
            size: 6,
            level: 1,
            weight: None,
            trim: false,
        }
    }
}

impl<'a> Heading<'a> {
    /// Radix's sizes 1–9 (clamped): 12, 14, 16, 18, 20, 24 (the default), 28, 35 or 60px.
    pub fn size(mut self, size: u8) -> Self {
        self.size = size.clamp(1, 9);
        self
    }

    /// The element, `<h1>` (the default) to `<h6>` (clamped): the level in the page's
    /// outline, whatever the size.
    pub fn as_h(mut self, level: u8) -> Self {
        self.level = level.clamp(1, 6);
        self
    }

    /// How heavy the letters are: light, regular, medium or bold (the default).
    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Trim the space above the capitals and below the baseline, so the heading lines up
    /// with the top of a card or the icon beside it.
    pub fn trim(mut self) -> Self {
        self.trim = true;
        self
    }
}

impl Render for Heading<'_> {
    fn render(&self) -> Markup {
        let class = if self.trim {
            "lui-heading lui-heading-trim"
        } else {
            "lui-heading"
        };
        let (size, weight, text) = (self.size, self.weight.map(Weight::as_str), self.text);
        // Maud names elements at compile time: one arm per level.
        html! {
            @match self.level {
                1 => h1 class=(class) data-size=(size) data-weight=[weight] { (text) },
                2 => h2 class=(class) data-size=(size) data-weight=[weight] { (text) },
                3 => h3 class=(class) data-size=(size) data-weight=[weight] { (text) },
                4 => h4 class=(class) data-size=(size) data-weight=[weight] { (text) },
                5 => h5 class=(class) data-size=(size) data-weight=[weight] { (text) },
                _ => h6 class=(class) data-size=(size) data-weight=[weight] { (text) },
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Heading: bold, balanced lines; the type scale 1–9 (--lui-text-n over
   --lui-leading-n, letter spacing from 0 to -0.025em) and the weights are text.rs's rules,
   which select .lui-heading too. */
.lui-heading { margin: 0; font-weight: 700; color: var(--lui-fg); text-wrap: balance; overflow-wrap: break-word; }
@supports (text-box: trim-both cap alphabetic) {
  .lui-heading-trim { text-box: trim-both cap alphabetic; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_an_h1_at_size_six() {
        let m = Ui::default().heading("Title").render().into_string();
        assert!(m.starts_with("<h1 ") && m.ends_with(">Title</h1>"), "{m}");
        assert!(
            m.contains(r#"data-size="6""#) && !m.contains("data-weight"),
            "{m}"
        );
    }

    #[test]
    fn the_level_is_chosen_apart_from_the_size_and_both_are_clamped() {
        let ui = Ui::default();
        let m = ui.heading("T").as_h(3).size(9).render().into_string();
        assert!(
            m.starts_with("<h3 ") && m.contains(r#"data-size="9""#),
            "{m}"
        );
        let m = ui.heading("T").as_h(0).size(0).render().into_string();
        assert!(
            m.starts_with("<h1 ") && m.contains(r#"data-size="1""#),
            "{m}"
        );
        let m = ui.heading("T").as_h(9).size(200).render().into_string();
        assert!(
            m.starts_with("<h6 ") && m.contains(r#"data-size="9""#),
            "{m}"
        );
    }

    #[test]
    fn weight_and_trim_show_up_as_an_attribute_and_a_class() {
        let m = Ui::default()
            .heading("T")
            .weight(Weight::Light)
            .trim()
            .render()
            .into_string();
        assert!(m.contains(r#"class="lui-heading lui-heading-trim""#), "{m}");
        assert!(m.contains(r#"data-weight="light""#), "{m}");
    }
}
