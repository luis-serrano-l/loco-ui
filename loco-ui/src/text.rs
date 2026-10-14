//! # Text
//!
//! A run of text at a size, weight and colour from the type scale: a caption, a lede, a
//! muted hint, a status in the danger colour, a file name cut short with an ellipsis. After
//! Radix Themes Text: sizes 1 to 9 on the scale Heading uses, an inline `<span>` by default
//! and a `<p>` or `<div>` on request, `.truncate()` for one line.
//!
//! `ui.text(..)` is also how a component reads its own words: given a
//! [`Text`](crate::i18n::Text) key it returns the word in the visitor's language, given a
//! `&str` it returns this builder ([`TextArg`]).
//!
//! **Platform features:** `<span>`, `<p>` and `<div>` (baseline 1995); `.truncate()` is
//! `text-overflow: ellipsis` (Chrome 1, Firefox 7, Safari 1.3) on one line.
//!
//! **Accessibility:** plain text, read in place; colour is never the only signal (a danger
//! text says what went wrong). A truncated text keeps its whole content in the DOM, so a
//! screen reader reads all of it. Checked by axe-core in headless Firefox on every demo route,
//! both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let t = ui.text("Last edited today").render().into_string();
//! assert_eq!(t, r#"<span class="lui-text">Last edited today</span>"#);
//! // The same in `lui!`:
//! let same = lui! { Text("Last edited today"); };
//! assert_eq!(same.into_string(), t);
//! // A muted paragraph at size 2, and a danger line in medium weight.
//! let p = ui.text("No invoices yet.").as_p().size(2).muted().render().into_string();
//! assert_eq!(p, r#"<p class="lui-text lui-text-muted" data-size="2">No invoices yet.</p>"#);
//! assert_eq!(lui! { Text("No invoices yet.") as_p size=2 muted; }.into_string(), p);
//! let e = ui.text("Payment failed").tone(Tone::Danger).weight(Weight::Medium).truncate();
//! let e = e.render().into_string();
//! assert!(e.contains(r#"class="lui-text lui-text-truncate" data-weight="medium" data-tone="danger""#));
//! assert_eq!(lui! { Text("Payment failed") tone="danger" weight="medium" truncate; }.into_string(), e);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n;
use crate::props::{Prop, PropKind};
use crate::variant::{Tone, Weight};

/// What [`Ui::text`] takes: an [`i18n::Text`] key, which gives that component word in the
/// visitor's language, or a `&str`, which gives a [`Text`] reading it.
pub trait TextArg<'a> {
    /// A `&'static str` for a key, a [`Text`] builder for a `&str`.
    type Output;
    /// What `ui.text(self)` returns.
    fn make(self, ui: &Ui) -> Self::Output;
}

impl TextArg<'_> for i18n::Text {
    type Output = &'static str;
    fn make(self, ui: &Ui) -> &'static str {
        ui.strings.get(self)
    }
}

impl<'a> TextArg<'a> for &'a str {
    type Output = Text<'a>;
    fn make(self, _: &Ui) -> Text<'a> {
        Text {
            text: self,
            element: Element::Span,
            size: None,
            weight: None,
            tone: None,
            muted: false,
            truncate: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Element {
    Span,
    P,
    Div,
}

/// A run of text, made by [`Ui::text`] with a `&str`.
///
/// **Setters.** Values: `.size(..)`, `.weight(..)`, `.tone(..)`; switches: `.as_p()`,
/// `.as_div()`, `.muted()`, `.truncate()`.
#[derive(Clone, Debug)]
pub struct Text<'a> {
    text: &'a str,
    element: Element,
    size: Option<u8>,
    weight: Option<Weight>,
    tone: Option<Tone>,
    muted: bool,
    truncate: bool,
}

impl Text<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("size", PropKind::Value, "size: u8")
            .default("inherit")
            .doc("Radix's sizes 1–9: 12, 14, 16, 18, 20, 24, 28, 35 or 60px."),
        Prop::new("weight", PropKind::Choice { of: Weight::NAMES }, "weight: Weight")
            .default("inherit")
            .doc("How heavy the letters are."),
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .default("inherit")
            .doc("The colour: step 11 of the tone's scale."),
        Prop::new("as_p", PropKind::Switch, "").doc("A `<p>`: a paragraph of its own."),
        Prop::new("as_div", PropKind::Switch, "")
            .doc("A `<div>`: a block that is not a paragraph (a caption, a cell's second line)."),
        Prop::new("muted", PropKind::Switch, "")
            .doc("The muted gray of hints and secondary lines."),
        Prop::new("truncate", PropKind::Switch, "")
            .doc("One line, cut with an ellipsis where it overflows."),
    ];
}

impl<'a> Text<'a> {
    /// Radix's sizes 1–9 (clamped): 12, 14, 16, 18, 20, 24, 28, 35 or 60px; unset, the text
    /// takes its parent's size.
    pub fn size(mut self, size: u8) -> Self {
        self.size = Some(size.clamp(1, 9));
        self
    }

    /// How heavy the letters are: light, regular, medium or bold; unset, the parent's.
    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// The colour: step 11 of the tone's scale (the gray, the brand, ok, warn or danger).
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// A `<p>`: a paragraph of its own.
    pub fn as_p(mut self) -> Self {
        self.element = Element::P;
        self
    }

    /// A `<div>`: a block that is not a paragraph (a caption, a cell's second line).
    pub fn as_div(mut self) -> Self {
        self.element = Element::Div;
        self
    }

    /// The muted gray of hints and secondary lines.
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }

    /// One line, cut with an ellipsis where it overflows; the whole text stays in the page.
    pub fn truncate(mut self) -> Self {
        self.truncate = true;
        self
    }
}

impl Render for Text<'_> {
    fn render(&self) -> Markup {
        let class = match (self.muted, self.truncate) {
            (false, false) => "lui-text",
            (true, false) => "lui-text lui-text-muted",
            (false, true) => "lui-text lui-text-truncate",
            (true, true) => "lui-text lui-text-muted lui-text-truncate",
        };
        let (size, weight, tone) = (
            self.size,
            self.weight.map(Weight::as_str),
            self.tone.map(Tone::as_str),
        );
        let text = self.text;
        html! {
            @match self.element {
                Element::Span => span class=(class) data-size=[size] data-weight=[weight] data-tone=[tone] { (text) },
                Element::P => p class=(class) data-size=[size] data-weight=[weight] data-tone=[tone] { (text) },
                Element::Div => div class=(class) data-size=[size] data-weight=[weight] data-tone=[tone] { (text) },
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Text: the type scale of Heading (12/16 to 60/60 px), no margin of its
   own, step 11 of a scale for a tone and --lui-muted for a hint. */
.lui-text { margin: 0; overflow-wrap: break-word; }
.lui-text[data-size="1"] { font-size: 0.75rem; line-height: 1rem; letter-spacing: 0.0025em; }
.lui-text[data-size="2"] { font-size: 0.875rem; line-height: 1.25rem; }
.lui-text[data-size="3"] { font-size: 1rem; line-height: 1.5rem; }
.lui-text[data-size="4"] { font-size: 1.125rem; line-height: 1.625rem; letter-spacing: -0.0025em; }
.lui-text[data-size="5"] { font-size: 1.25rem; line-height: 1.75rem; letter-spacing: -0.005em; }
.lui-text[data-size="6"] { font-size: 1.5rem; line-height: 1.875rem; letter-spacing: -0.00625em; }
.lui-text[data-size="7"] { font-size: 1.75rem; line-height: 2.25rem; letter-spacing: -0.0075em; }
.lui-text[data-size="8"] { font-size: 2.1875rem; line-height: 2.5rem; letter-spacing: -0.01em; }
.lui-text[data-size="9"] { font-size: 3.75rem; line-height: 3.75rem; letter-spacing: -0.025em; }
.lui-text[data-weight=light] { font-weight: 300; }
.lui-text[data-weight=regular] { font-weight: 400; }
.lui-text[data-weight=medium] { font-weight: 500; }
.lui-text[data-weight=bold] { font-weight: 700; }
.lui-text-muted { color: var(--lui-muted); }
.lui-text[data-tone=neutral] { color: var(--lui-gray-11); }
.lui-text:is([data-tone=brand], [data-tone=info]) { color: var(--lui-brand-11); }
.lui-text[data-tone=ok] { color: color-mix(in srgb, var(--lui-ok) 80%, var(--lui-fg)); }
.lui-text[data-tone=warn] { color: color-mix(in srgb, var(--lui-warn) 75%, var(--lui-fg)); }
.lui-text[data-tone=danger] { color: color-mix(in srgb, var(--lui-danger) 85%, var(--lui-fg)); }
.lui-text-truncate { display: block; min-width: 0; max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
"#;
