//! # Hover card
//!
//! A preview card that shows while the pointer rests on a link or it has focus: who a user
//! is behind their @name, what a linked page holds. Where the browser cannot open it on
//! hover, a click opens the same card. After the Radix Themes HoverCard.
//!
//! **Platform features:** with `Caps::InterestInvokers`, `interestfor` on the trigger
//! (Chrome 142; Firefox and Safari not yet) opens a `popover="hint"` (Chrome 133) on hover
//! and focus, and closes it when both leave; the trigger is the popover's implicit anchor,
//! so `position-area` (anchor positioning, Chrome 125, Firefox 147, Safari 26) puts the card
//! under it. Without it, `popover` with `popovertarget` (Chrome 114, Firefox 125, Safari 17)
//! opens the card on click and light dismiss closes it.
//!
//! **Accessibility:** the trigger is a link or a button, and the card follows it in the
//! document, so a screen reader reaches it after the trigger. A button trigger toggles the card
//! with Enter or Space; a link trigger gets a separate button, named "More about …", where
//! hover cannot open the card. Checked by axe-core in headless Firefox on every demo route,
//! both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** wait before opening (the platform's
//! `interest-delay` decides) or open on hover where `interestfor` is missing.
//!
//! **Fallback:** without `Caps::InterestInvokers` the card is `popover="auto"` opened by
//! click through `popovertarget`: on a button trigger the button itself, on a link trigger a
//! small chevron button after the link (a link cannot target a popover). Without
//! `Caps::Popover` it is a `<details>` disclosure under the trigger. Without anchor
//! positioning the popover sits where the browser centres it.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let m = ui.hover_card("@ada", html! { "Ada Lovelace, first programmer." }).render().into_string();
//! assert!(m.contains(r#"interestfor="lui-hover-card-ada""#) && m.contains(r#"popover="hint""#));
//! // A link trigger with its own id; without interest invokers a click opens the card.
//! let old = Ui::from(Caps::ASSUMED);
//! let m = old.hover_card("Release notes", html! { "What changed in 0.2." }).href("/changelog").id("notes");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<a class="lui-hover-card-link" href="/changelog">Release notes</a>"#) && html.contains(r#"popovertarget="notes""#));
//! assert!(html.contains(r#"popover="auto""#) && !html.contains("interestfor"));
//! // The same in `lui!`:
//! let ui = old;
//! let same = lui! { HoverCard("Release notes", html! { "What changed in 0.2." }) href="/changelog" id="notes"; };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::{Strings, Text};
use crate::props::{Key, Prop, PropKind};
use crate::{Cap, Caps, Icon, Ui, slug};

/// A trigger with a card that opens on hover, made by [`Ui::hover_card`].
///
/// **Setters.** Values and items: `.href(..)`, `.id(..)`.
#[derive(Clone, Debug)]
pub struct HoverCard<'a> {
    caps: Caps,
    strings: &'static Strings,
    trigger: &'a str,
    body: Markup,
    href: Option<&'a str>,
    id: Option<&'a str>,
}

impl HoverCard<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Shows the card while its trigger has focus, where the browser has interest invokers.",
        ),
        Key::new(
            "Enter / Space",
            "Opens or closes the card from its button where the browser has no interest invokers.",
        ),
        Key::new("Enter", "Follows the trigger link when the card has one."),
        Key::new(
            "Escape",
            "Closes the card. Without popover support it closes from its button instead.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("href", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("The trigger is a link to `href` instead of a button."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The card's id, `lui-hover-card-<slug of the trigger>` by default."),
    ];
}

impl Ui {
    /// `body` in a card that opens while the pointer rests on `trigger` or it has focus.
    pub fn hover_card<'a>(&self, trigger: &'a str, body: Markup) -> HoverCard<'a> {
        HoverCard {
            caps: self.caps,
            strings: self.strings,
            trigger,
            body,
            href: None,
            id: None,
        }
    }
}

impl<'a> HoverCard<'a> {
    /// The trigger is a link to `href` instead of a button.
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    /// The card's id, `lui-hover-card-<slug of the trigger>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Render for HoverCard<'_> {
    fn render(&self) -> Markup {
        let id = self.id.map_or_else(
            || {
                format!(
                    "lui-hover-card-{}",
                    slug(self.trigger.trim_start_matches('@'))
                )
            },
            str::to_string,
        );
        let card = |kind: &str| html! { span id=(id) popover=(kind) class="lui-hover-card-content" { (self.body) } };
        let more = self.strings.fill(Text::MoreAbout, &[&self.trigger]);
        if !self.caps.has(Cap::Popover) {
            return html! {
                span class="lui-hover-card" {
                    @if let Some(href) = self.href { a class="lui-hover-card-link" href=(href) { (self.trigger) } }
                    details {
                        @if self.href.is_some() {
                            summary class="lui-button lui-button-ghost lui-button-small lui-button-icon" aria-label=(more) { (Icon::ChevronDown) }
                        } @else {
                            summary class="lui-button lui-button-ghost lui-hover-card-trigger" { (self.trigger) }
                        }
                        span id=(id) class="lui-hover-card-content" { (self.body) }
                    }
                }
            };
        }
        let hover = self.caps.has(Cap::InterestInvokers);
        let button = Button::new(self.caps, self.trigger)
            .ghost()
            .popovertarget(&id);
        html! {
            span class="lui-hover-card" {
                @match (self.href, hover) {
                    (Some(href), true) => a class="lui-hover-card-link" href=(href) interestfor=(id) { (self.trigger) },
                    (Some(href), false) => {
                        a class="lui-hover-card-link" href=(href) { (self.trigger) }
                        (button.small().icon_only().aria_label(&more).body(html! { (Icon::ChevronDown) }))
                    },
                    (None, true) => (button.interestfor(&id).class("lui-hover-card-trigger")),
                    (None, false) => (button.class("lui-hover-card-trigger")),
                }
                (card(if hover { "hint" } else { "auto" }))
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Radix Themes HoverCard: a panel of the popover colour, 1rem of padding, the large
   radius and a medium shadow, at most 20rem wide (and never wider than the viewport), 0.5rem
   under its trigger where anchor positioning places it. A button trigger is a ghost button
   with the text colour of a link. */
.lui-hover-card { position: relative; display: inline-flex; align-items: center; gap: var(--lui-space-1); vertical-align: baseline; }
.lui-button.lui-hover-card-trigger { color: var(--lui-link); padding-inline: var(--lui-space-1); }
.lui-hover-card-content {
  display: block; box-sizing: border-box; width: max-content; max-width: min(20rem, calc(100vw - 2rem)); margin: 0; padding: var(--lui-space-4);
  font-size: var(--lui-text-2); line-height: 1.25rem; color: var(--lui-fg); text-align: start; overflow-wrap: anywhere;
  background: var(--lui-popover); border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  box-shadow: var(--lui-shadow-md), var(--lui-highlight);
}
.lui-hover-card-content:popover-open { inset: auto; }
.lui-hover-card-content[popover]:not(:popover-open) { display: none; }
@supports (position-area: bottom) {
  .lui-hover-card-content[popover] { position-area: bottom; position-try-fallbacks: flip-block; margin-top: var(--lui-space-2); }
}
@supports not (position-area: bottom) { .lui-hover-card-content:popover-open { inset: 0; margin: auto; } }
/* The <details> fallback: the card drops under the trigger in the page's flow, so with no
   anchor to place it by it can never run past the viewport's edge. */
.lui-hover-card details { display: inline-block; }
.lui-hover-card details > summary { list-style: none; }
.lui-hover-card details > summary::-webkit-details-marker { display: none; }
.lui-hover-card details > .lui-hover-card-content { margin-top: var(--lui-space-2); }
@media (pointer: coarse) {
  .lui-hover-card-trigger, .lui-hover-card-link { min-height: var(--lui-hit); }
  .lui-hover-card-link { display: inline-flex; align-items: center; }
}
"#;
