//! # Spinner
//!
//! A small turning ring that says something is on its way: a panel streaming in, a search
//! waiting on the server, a busy button (which draws this same ring). After Radix Themes
//! Spinner: three sizes (12, 16 and 20px), in the colour of the text around it.
//!
//! **Platform features:** a CSS `@keyframes` rotation (baseline 2015) of a bordered circle with
//! one side clear; `prefers-reduced-motion` (Chrome 74, Firefox 63, Safari 10.1) slows it
//! down rather than stopping it, so it still reads as busy.
//!
//! **Accessibility:** `role="status"` named "Loading" (or `.label(..)`), so a screen reader
//! announces it once without moving the focus. The ring inside a busy button is
//! `aria-hidden`, since the button says `aria-busy`. Checked by axe-core in headless Firefox
//! on every demo route, both capability variants, light and dark (no serious or critical
//! violation).
//!
//! **What it does not do without script:** nothing is missing; it turns while it is on the
//! page, and the server takes it off by sending the next page (or a stream's next chunk).
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let s = ui.spinner().render().into_string();
//! assert_eq!(s, r#"<span class="lui-spinner" role="status" aria-label="Loading"></span>"#);
//! // The same in `lui!`:
//! let same = lui! { Spinner; };
//! assert_eq!(same.into_string(), s);
//! // Larger, saying what it waits for.
//! let s = ui.spinner().size(3).label("Loading results").render().into_string();
//! assert_eq!(s, r#"<span class="lui-spinner" role="status" aria-label="Loading results" data-size="3"></span>"#);
//! assert_eq!(lui! { Spinner size=3 label="Loading results"; }.into_string(), s);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::variant::Size;

/// A loading ring, made by [`Ui::spinner`].
///
/// **Setters.** Values: `.size(..)`, `.label(..)`.
#[derive(Clone, Debug)]
pub struct Spinner<'a> {
    size: Size,
    label: Option<&'a str>,
}

impl Spinner<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new(
            "size",
            PropKind::Choice { of: Size::NAMES },
            "size: impl Into<Size>",
        )
        .default("2")
        .doc("12px (size 1), 16px (2, the default) or 20px (3) across."),
        Prop::new("label", PropKind::Value, "label: &'a str")
            .default("Loading")
            .doc("What a screen reader announces: what is loading."),
    ];

    /// The ring alone, `aria-hidden`, for a part that says it is busy itself (a button with
    /// `aria-busy`).
    pub(crate) fn decoration() -> Markup {
        html! { span class="lui-spinner" aria-hidden="true" {} }
    }
}

impl Ui {
    /// A loading ring at 16px, announced as "Loading".
    pub fn spinner<'a>(&self) -> Spinner<'a> {
        Spinner {
            size: Size::S2,
            label: Some(self.text(Text::Loading)),
        }
    }
}

impl<'a> Spinner<'a> {
    /// 12px (size 1), 16px (2, the default) or 20px (3) across.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// What a screen reader announces ("Loading results"); "Loading" by default.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

impl Render for Spinner<'_> {
    fn render(&self) -> Markup {
        let size = (self.size != Size::S2).then(|| self.size.as_str());
        html! { span class="lui-spinner" role="status" aria-label=[self.label] data-size=[size] {} }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Spinner: 12, 16 or 20px, in currentColor; drawn as a 2px ring with one
   side clear (Radix's eight leaves need eight elements), a turn every 0.6s. */
.lui-spinner {
  display: inline-block; width: 1rem; height: 1rem; flex: none; box-sizing: border-box; vertical-align: middle;
  border-radius: 50%; border: 2px solid currentColor; border-right-color: transparent;
  animation: lui-spin 0.6s linear infinite;
}
.lui-spinner[data-size="1"] { width: 0.75rem; height: 0.75rem; border-width: 1.5px; }
.lui-spinner[data-size="3"] { width: 1.25rem; height: 1.25rem; }
@keyframes lui-spin { to { transform: rotate(1turn); } }
@media (prefers-reduced-motion: reduce) { .lui-spinner { animation-duration: 1.5s; } }
"#;
