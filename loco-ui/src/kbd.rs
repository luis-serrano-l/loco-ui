//! # Kbd
//!
//! A key or a shortcut drawn as a keycap: `⌘K` in a menu, `Esc` in a dialog's hint, `Ctrl`
//! `Shift` `P` in the docs. After Radix Themes Kbd: the system font at 0.75em, a box
//! 1.75em tall and at least as wide, a pale fill and a two-part inset shadow that reads as a
//! key's edge.
//!
//! **Platform features:** `<kbd>` (baseline 1995); `box-shadow` (Chrome 10, Firefox 4,
//! Safari 5.1).
//!
//! **Accessibility:** `<kbd>` is user input, read as its text ("⌘K" is read as the
//! characters it holds; write `Ctrl K` where the symbol would be unclear). Checked by axe-core
//! in headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing; it shows a shortcut, it does
//! not bind one (`accesskey` on a button does that).
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let k = ui.kbd("⌘K").render().into_string();
//! assert_eq!(k, r#"<kbd class="lui-kbd">⌘K</kbd>"#);
//! // The same in `lui!`:
//! let same = lui! { Kbd("⌘K"); };
//! assert_eq!(same.into_string(), k);
//! // Larger, in a heading-sized hint.
//! let k = ui.kbd("Esc").size(3).render().into_string();
//! assert_eq!(k, r#"<kbd class="lui-kbd" data-size="3">Esc</kbd>"#);
//! assert_eq!(lui! { Kbd("Esc") size=3; }.into_string(), k);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};
use crate::variant::Size;

/// A keycap, made by [`Ui::kbd`].
///
/// **Setters.** Values: `.size(..)`.
#[derive(Clone, Debug)]
pub struct Kbd<'a> {
    keys: &'a str,
    size: Size,
}

impl Kbd<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[Prop::new(
        "size",
        PropKind::Choice { of: Size::NAMES },
        "size: impl Into<Size>",
    )
    .default("2")
    .doc("Size 1 at 0.6875rem, 2 at 0.75rem (the default), 3 at 0.875rem.")];
}

impl Ui {
    /// A keycap reading `keys`: one key (`Esc`) or a shortcut (`⌘K`).
    pub fn kbd<'a>(&self, keys: &'a str) -> Kbd<'a> {
        Kbd {
            keys,
            size: Size::S2,
        }
    }
}

impl<'a> Kbd<'a> {
    /// Size 1 at 0.6875rem, 2 at 0.75rem (the default), 3 at 0.875rem.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl Render for Kbd<'_> {
    fn render(&self) -> Markup {
        let size = (self.size != Size::S2).then(|| self.size.as_str());
        html! { kbd class="lui-kbd" data-size=[size] { (self.keys) } }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Kbd: 1.75em tall, at least as wide, 0.5em padding, radius 0.35em, the
   system font at 0.75rem, a --lui-gray-1 fill with an inset edge in gray steps 7 and 3. */
.lui-kbd {
  display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;
  height: 1.75em; min-width: 1.75em; padding: 0 0.5em; vertical-align: text-top;
  font-family: var(--lui-font-sans); font-size: 0.75rem; font-weight: 400; line-height: 1; letter-spacing: 0.02em;
  white-space: nowrap; user-select: none; color: var(--lui-gray-12); background: var(--lui-gray-1);
  border-radius: 0.35em;
  box-shadow: inset 0 -0.05em 0.5px 0.5px var(--lui-gray-7), inset 0 0.05em 0 0.5px var(--lui-gray-3), 0 0.05em 0.1em var(--lui-gray-4);
}
.lui-kbd[data-size="1"] { font-size: 0.6875rem; }
.lui-kbd[data-size="3"] { font-size: 0.875rem; }
"#;
