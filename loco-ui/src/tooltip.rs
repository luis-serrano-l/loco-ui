//! # Tooltip
//!
//! A short label that appears beside a control while the pointer is over it or it has focus:
//! what an icon button does, what a shortcut is.
//!
//! **Platform features:** CSS only: the text is a `role="tooltip"` element shown by
//! `:hover` and `:focus-within` (baseline 2020) on the wrapper, and the trigger inside names
//! it with `aria-describedby` so a screen reader says it too. `@media (hover: none)` (Chrome
//! 41, Firefox 64, Safari 9) keeps it off touch screens, where hover does not exist.
//!
//! **Accessibility:** `role="tooltip"` joined to its trigger by `aria-describedby`; shown on
//! hover and on keyboard focus. Checked by axe-core in headless Firefox on every demo route,
//! both capability variants, light and dark (no serious or critical violation).
//!
//! `.side(..)` puts it on any side of the trigger (the same [`Side`] as the popover's) and
//! `.delay(ms)` waits before showing it on hover: a `transition-delay` on the way in, none
//! on the way out.
//!
//! **What it does not do without script:** close on Escape while the pointer stays over it,
//! or flip to fit the viewport. `popover="hint"` with hover triggers
//! would do some of this, but it is Chromium-only and needs script to open on hover.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.tooltip("Copy the link", html! { (ui.button("").icon_only().ghost().aria_label("Copy").body(html! { (Icon::Copy) })) });
//! let m = m.render().into_string();
//! assert!(m.contains(r#"role="tooltip""#) && m.contains("Copy the link"));
//! assert!(m.contains(r#"aria-describedby="lui-tooltip-copy-the-link""#));
//! let below = ui.tooltip("Saved", html! { span { "3" } }).below().render().into_string();
//! assert!(below.contains(r#"data-side="bottom""#));
//! // The same in `lui!`:
//! let same = lui! { Tooltip("Copy the link", lui! {
//!     Button("") icon_only ghost aria_label="Copy" body=(html! { (Icon::Copy) });
//! }); };
//! assert_eq!(same.into_string(), m);
//! // A button builder as the trigger.
//! let copy = ui.button("").icon_only().ghost().aria_label("Copy").leading(Icon::Copy);
//! let m = ui.tooltip("Copy the link", html! {}).trigger(copy).render().into_string();
//! assert!(m.contains(r#"aria-label="Copy" aria-describedby="lui-tooltip-copy-the-link""#));
//! // At the end of the trigger, after half a second.
//! let m = ui.tooltip("Saved", html! { span { "3" } }).side(Side::End).delay(500).render().into_string();
//! assert!(m.starts_with(r#"<span class="lui-tooltip" data-side="end" style="--lui-tooltip-delay: 500ms">"#));
//! assert_eq!(lui! { Tooltip("Saved", html! { span { "3" } }) side=(Side::End) delay=500; }.into_string(), m);
//! ```

use maud::{Markup, PreEscaped, Render, html};

use crate::button::Button;
use crate::popover_card::Side;
use crate::props::{Key, Prop, PropKind};
use crate::{Ui, slug};

/// A trigger with a tooltip, made by [`Ui::tooltip`].
///
/// **Setters.** Values and items: `.id(..)`, `.side(..)`, `.delay(..)`, `.trigger(..)`; switches: `.below()`.
#[derive(Clone, Debug)]
pub struct Tooltip<'a> {
    text: &'a str,
    trigger: Markup,
    side: Side,
    delay: Option<u32>,
    id: Option<&'a str>,
    button: Option<Button<'a>>,
}

impl Tooltip<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[Key::new(
        "Tab / Shift+Tab",
        "Shows the tooltip while its trigger has focus and hides it when focus leaves.",
    )];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("below", PropKind::Switch, "")
            .doc("Show the text under the trigger instead of above it: `.side(Side::Bottom)`."),
        Prop::new("side", PropKind::Choice { of: Side::NAMES }, "side: Side")
            .default("top")
            .doc("The side of the trigger the text shows on."),
        Prop::new("delay", PropKind::Number, "ms: u32")
            .default("0")
            .doc(
                "How long the pointer rests on the trigger before the text shows, in milliseconds.",
            ),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The tooltip's id, `lui-tooltip-<slug of the text>` by default."),
        Prop::new("trigger", PropKind::Value, "button: Button<'a>")
            .doc("A button builder as the trigger, in place of the markup: it gets `aria-describedby` as an attribute."),
    ];
}

impl Ui {
    /// `trigger` (a button, an icon, a word) with `text` shown above it on hover and focus.
    pub fn tooltip<'a>(&self, text: &'a str, trigger: Markup) -> Tooltip<'a> {
        Tooltip {
            text,
            trigger,
            side: Side::Top,
            delay: None,
            id: None,
            button: None,
        }
    }
}

impl<'a> Tooltip<'a> {
    /// Show the text under the trigger instead of above it: `.side(Side::Bottom)`.
    pub fn below(self) -> Self {
        self.side(Side::Bottom)
    }

    /// The side of the trigger the text shows on: top (the default), bottom, start or end.
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// How long the pointer rests on the trigger before the text shows, in milliseconds;
    /// keyboard focus shows it at once, and it hides at once.
    pub fn delay(mut self, ms: u32) -> Self {
        self.delay = Some(ms);
        self
    }

    /// The tooltip's id, `lui-tooltip-<slug of the text>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// A button builder as the trigger, in place of the markup given to [`Ui::tooltip`]: it
    /// gets `aria-describedby` as an attribute instead of by editing the markup's text.
    pub fn trigger(mut self, button: Button<'a>) -> Self {
        self.button = Some(button);
        self
    }
}

impl Render for Tooltip<'_> {
    fn render(&self) -> Markup {
        let id = self.id.map_or_else(
            || format!("lui-tooltip-{}", slug(self.text)),
            str::to_string,
        );
        // The trigger is caller markup: its first element gets `aria-describedby` by string,
        // since Maud cannot add an attribute to markup it did not build.
        let trigger = self.trigger.0.replacen(
            '>',
            &format!(" aria-describedby=\"{id}\">"),
            usize::from(self.trigger.0.starts_with('<')),
        );
        html! {
            span class="lui-tooltip" data-side=[(self.side != Side::Top).then(|| self.side.as_str())]
                style=[self.delay.map(|ms| format!("--lui-tooltip-delay: {ms}ms"))] {
                @if let Some(b) = &self.button { (b.clone().describedby(&id)) } @else { (PreEscaped(trigger)) }
                span id=(id) role="tooltip" class="lui-tooltip-text" { (self.text) }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. shadcn Tooltip: the primary
/// colour as the background, text-xs, rounded-md, px-3 py-1.5.
pub const CSS: &str = r#"
/* Radix Themes Tooltip: the inverted chip, --lui-gray-12 with --lui-gray-1 text, small type. */
.lui-tooltip { position: relative; display: inline-flex; }
.lui-tooltip-text {
  position: absolute; z-index: var(--lui-z-tooltip); inset-inline-start: 50%; bottom: calc(100% + 6px); translate: -50% 0;
  width: max-content; max-width: 16rem; padding: 0.375rem 0.75rem; pointer-events: none;
  font-size: var(--lui-text-1); line-height: 1rem; font-weight: 400; text-align: center;
  color: var(--lui-gray-1); background: var(--lui-gray-12); border-radius: var(--lui-radius-sm);
  box-shadow: var(--lui-shadow-md);
  opacity: 0; visibility: hidden; transition: opacity 0.15s, visibility 0.15s, display 0.15s allow-discrete;
  /* Out of the layout at rest, so a tooltip beside a button at the edge never widens the page. */
  display: none;
}
/* Centred with a translate, which is physical: mirror it in right-to-left. */
.lui-tooltip-text:dir(rtl) { translate: 50% 0; }
.lui-tooltip[data-side=bottom] .lui-tooltip-text { bottom: auto; top: calc(100% + 6px); }
.lui-tooltip:is([data-side=start], [data-side=end]) .lui-tooltip-text { inset-inline-start: auto; bottom: auto; inset-block-start: 50%; translate: 0 -50%; }
.lui-tooltip[data-side=start] .lui-tooltip-text { inset-inline-end: calc(100% + 6px); }
.lui-tooltip[data-side=end] .lui-tooltip-text { inset-inline-start: calc(100% + 6px); }
.lui-tooltip:hover .lui-tooltip-text, .lui-tooltip:focus-within .lui-tooltip-text { opacity: 1; visibility: visible; display: block; }
@starting-style { .lui-tooltip:hover .lui-tooltip-text, .lui-tooltip:focus-within .lui-tooltip-text { opacity: 0; } }
/* .delay(ms): the hover waits; focus does not, and leaving hides at once. */
.lui-tooltip:hover:not(:focus-within) .lui-tooltip-text { transition-delay: var(--lui-tooltip-delay, 0s); }
@media (hover: none) { .lui-tooltip:hover:not(:focus-within) .lui-tooltip-text { opacity: 0; visibility: hidden; display: none; } }
@media (prefers-reduced-motion: reduce) { .lui-tooltip-text { transition: none; } }
"#;
