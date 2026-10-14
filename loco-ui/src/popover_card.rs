//! # Popover
//!
//! A panel of anything (a short form, a profile card, help text) that opens over the page from
//! a button and closes on a click outside or Escape. Unlike a [`crate::popover::Menu`] it has
//! no items: the body is the caller's markup. After Radix Themes Popover: a popover surface
//! with a medium radius and shadow, 0.75rem from its button, on the side and alignment asked
//! for, with an optional arrow.
//!
//! **Platform features:**
//! - `popover` attribute with a `popovertarget` button (Chrome 114, Firefox 125, Safari 17):
//!   light dismiss, Escape and the top layer.
//! - CSS anchor positioning (Baseline 2026: Chrome 125, Firefox 147, Safari 26), used
//!   whatever the caps say: `anchor-name` on the button, `position-anchor` and `position-area`
//!   on the panel for `.side(..)` and `.align(..)`, and `position-try-fallbacks: flip-block,
//!   flip-inline` (Chrome 128, Firefox 147, Safari 26) so a panel that would leave the
//!   viewport opens on the other side. [`crate::popover::Menu`] and the context menu take
//!   the same fallbacks; [`crate::tooltip::Tooltip`] takes the same [`Side`].
//! - `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) and `transition-behavior:
//!   allow-discrete` for the fade in and out.
//!
//! **Accessibility:** the button says `aria-haspopup="dialog"` and the platform reports it
//! expanded while the panel is open; the panel is `role="dialog"` named by the button's text.
//! Tab moves from the button into the panel. Checked by axe-core in headless Firefox on every
//! demo route, both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** turn the arrow when a fallback flips the panel
//! (that needs anchored container queries, Chrome-only); the arrow points the asked-for way.
//!
//! **Fallback:** a browser without anchor positioning shows the popover centred in the
//! viewport, still usable. Without `Caps` `Popover` it is a `<details>` disclosure whose panel
//! drops under the button; it closes on a second click, not on a click outside.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let m = ui.popover("Share", html! { p { "Anyone with the link can view." } }).render().into_string();
//! assert!(m.contains(r#"popovertarget="share""#) && m.contains(r#"<div id="share" class="lui-popover-card-panel" popover role="dialog" aria-label="Share""#));
//! let m = ui.popover("Share", html! { p { "Copied." } }).side(Side::Top).align(Align::End).arrow().id("share-2").render().into_string();
//! assert!(m.contains("position-area: block-start span-inline-start") && m.contains("lui-popover-card-arrow"));
//! // The same in `lui!`:
//! let same = lui! { Popover("Share", html! { p { "Copied." } }) side=(Side::Top) align=(Align::End) arrow id="share-2"; };
//! assert_eq!(same.into_string(), m);
//! // Without the popover attribute: a <details> disclosure.
//! let m = Ui::from(Caps::NONE).popover("Share", html! { "x" }).render().into_string();
//! assert!(m.starts_with(r#"<details class="lui-popover-card lui-popover-card-details""#));
//! ```

use maud::{Markup, Render, html};

use crate::props::{Prop, PropKind};
use crate::{Cap, Ui, slug};

/// Which side of its trigger a popover, tooltip or drawer opens on. Start and end follow the
/// writing direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Side {
    /// Above.
    Top,
    /// Below (the default for a popover).
    #[default]
    Bottom,
    /// Before it in the line: the left in a left-to-right page.
    Start,
    /// After it in the line: the right in a left-to-right page.
    End,
}

impl Side {
    /// Every value as written in `data-side`.
    pub const NAMES: &'static [&'static str] = &["top", "bottom", "start", "end"];

    /// The value as written in `data-side`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Side::Top => "top",
            Side::Bottom => "bottom",
            Side::Start => "start",
            Side::End => "end",
        }
    }
}

/// How a popover lines up with its trigger along the side it opens on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Align {
    /// Start edges together (the default).
    #[default]
    Start,
    /// Centred on the trigger.
    Center,
    /// End edges together.
    End,
}

impl Align {
    /// Every value as written in `data-align`.
    pub const NAMES: &'static [&'static str] = &["start", "center", "end"];

    /// The value as written in `data-align`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Align::Start => "start",
            Align::Center => "center",
            Align::End => "end",
        }
    }
}

/// The `position-area` for a box on `side` of its anchor, lined up by `align` (one keyword
/// alone spans the other axis, which centres it): shared by the popover and the menu.
pub(crate) const fn area(side: Side, align: Align) -> &'static str {
    match (side, align) {
        (Side::Top, Align::Start) => "block-start span-inline-end",
        (Side::Top, Align::Center) => "block-start",
        (Side::Top, Align::End) => "block-start span-inline-start",
        (Side::Bottom, Align::Start) => "block-end span-inline-end",
        (Side::Bottom, Align::Center) => "block-end",
        (Side::Bottom, Align::End) => "block-end span-inline-start",
        (Side::Start, Align::Start) => "inline-start span-block-end",
        (Side::Start, Align::Center) => "inline-start",
        (Side::Start, Align::End) => "inline-start span-block-start",
        (Side::End, Align::Start) => "inline-end span-block-end",
        (Side::End, Align::Center) => "inline-end",
        (Side::End, Align::End) => "inline-end span-block-start",
    }
}

/// A button that opens a panel of markup, made by [`Ui::popover`].
///
/// **Setters.** Values and items: `.side(..)`, `.align(..)`, `.id(..)`; switches: `.arrow()`.
#[derive(Clone, Debug)]
pub struct Popover<'a> {
    ui: &'a Ui,
    trigger: &'a str,
    body: Markup,
    side: Side,
    align: Align,
    arrow: bool,
    id: Option<&'a str>,
}

impl Popover<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("side", PropKind::Choice { of: Side::NAMES }, "side: Side")
            .default("bottom")
            .doc("The side of the button it opens on; it flips when there is no room."),
        Prop::new(
            "align",
            PropKind::Choice { of: Align::NAMES },
            "align: Align",
        )
        .default("start")
        .doc("How it lines up with the button along that side."),
        Prop::new("arrow", PropKind::Switch, "").doc("A small arrow pointing at the button."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The panel's id instead of the trigger's slug."),
    ];
}

impl Ui {
    /// A button labelled `trigger` that opens `body` in a panel under it.
    pub fn popover<'a>(&'a self, trigger: &'a str, body: Markup) -> Popover<'a> {
        Popover {
            ui: self,
            trigger,
            body,
            side: Side::Bottom,
            align: Align::Start,
            arrow: false,
            id: None,
        }
    }
}

impl<'a> Popover<'a> {
    /// The side of the button it opens on (below by default); `position-try-fallbacks`
    /// flips it when there is no room.
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// How it lines up with the button along that side: start (the default), center or end.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// A small arrow pointing at the button.
    pub fn arrow(mut self) -> Self {
        self.arrow = true;
        self
    }

    /// The panel's id instead of the trigger's slug; it must be unique on the page.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Render for Popover<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let id = self.id.map_or_else(|| slug(self.trigger), str::to_string);
        let side = (self.side != Side::Bottom).then(|| self.side.as_str());
        let align = (self.align != Align::Start).then(|| self.align.as_str());
        let arrow =
            html! { @if self.arrow { span class="lui-popover-card-arrow" aria-hidden="true" {} } };
        html! {
            @if ui.has(Cap::Popover) {
                div class="lui-popover-card" data-side=[side] data-align=[align] {
                    (ui.button(self.trigger).popovertarget(&id).aria_haspopup("dialog").style(format!("anchor-name: --{id}")))
                    div id=(id) class="lui-popover-card-panel" popover role="dialog" aria-label=(self.trigger)
                        style={ "position-anchor: --" (id) "; position-area: " (area(self.side, self.align)) } {
                        (arrow) (self.body)
                    }
                }
            } @else {
                details class="lui-popover-card lui-popover-card-details" data-side=[side] data-align=[align] {
                    summary class="lui-button" aria-haspopup="dialog" { (self.trigger) }
                    div id=(id) class="lui-popover-card-panel" role="dialog" aria-label=(self.trigger) { (arrow) (self.body) }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Popover: the popover surface, --lui-radius, shadow-md, 1rem padding,
   at most 20rem wide (and never wider than the viewport less 1rem), 0.5rem from its button. */
.lui-popover-card { display: inline-block; position: relative; }
.lui-popover-card-panel {
  --lui-popover-card-gap: 0.5rem;
  box-sizing: border-box; width: max-content; max-width: min(20rem, calc(100vw - 1rem)); padding: var(--lui-space-4);
  font-size: var(--lui-text-2); line-height: var(--lui-leading-2); color: var(--lui-fg); background: var(--lui-popover);
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius); box-shadow: var(--lui-shadow-md), var(--lui-highlight);
}
.lui-popover-card-panel > :first-child:not(.lui-popover-card-arrow), .lui-popover-card-arrow + * { margin-top: 0; }
.lui-popover-card-panel > :last-child { margin-bottom: 0; }
.lui-popover-card-panel:has(> .lui-popover-card-arrow) { --lui-popover-card-gap: 0.75rem; }
/* Anchored where the browser can; flipped to the other side when it would not fit. */
@supports (position-area: block-end) {
  .lui-popover-card-panel[popover] {
    inset: auto; margin: 0; position-try-fallbacks: flip-block, flip-inline, flip-block flip-inline;
    margin-block-start: var(--lui-popover-card-gap);
  }
  .lui-popover-card[data-side=top] > .lui-popover-card-panel { margin: 0 0 var(--lui-popover-card-gap); }
  .lui-popover-card[data-side=start] > .lui-popover-card-panel { margin: 0; margin-inline-end: var(--lui-popover-card-gap); }
  .lui-popover-card[data-side=end] > .lui-popover-card-panel { margin: 0; margin-inline-start: var(--lui-popover-card-gap); }
}
/* The arrow: a square turned 45deg, half outside the edge that faces the button. */
.lui-popover-card-arrow {
  position: absolute; width: 0.625rem; height: 0.625rem; rotate: 45deg; background: inherit;
  border: inherit; border-width: 1px 0 0 1px; inset-block-start: -0.375rem; inset-inline-start: 1rem;
}
.lui-popover-card[data-align=center] .lui-popover-card-arrow { inset-inline-start: calc(50% - 0.3125rem); }
.lui-popover-card[data-align=end] .lui-popover-card-arrow { inset-inline-start: auto; inset-inline-end: 1rem; }
.lui-popover-card[data-side=top] .lui-popover-card-arrow { inset-block-start: auto; inset-block-end: -0.375rem; border-width: 0 1px 1px 0; }
.lui-popover-card:is([data-side=start], [data-side=end]) .lui-popover-card-arrow { inset-block-start: 0.75rem; inset-inline-start: auto; }
.lui-popover-card[data-side=start] .lui-popover-card-arrow { inset-inline-end: -0.375rem; border-width: 1px 1px 0 0; }
.lui-popover-card[data-side=end] .lui-popover-card-arrow { inset-inline-start: -0.375rem; border-width: 0 0 1px 1px; }
/* <details> fallback: the panel drops under (or over) the summary. */
.lui-popover-card-details > summary { list-style: none; }
.lui-popover-card-details > summary::-webkit-details-marker { display: none; }
.lui-popover-card-details > .lui-popover-card-panel { position: absolute; z-index: var(--lui-z-popover); inset-block-start: calc(100% + var(--lui-popover-card-gap)); inset-inline-start: 0; }
.lui-popover-card-details[data-align=end] > .lui-popover-card-panel { inset-inline-start: auto; inset-inline-end: 0; }
.lui-popover-card-details[data-side=top] > .lui-popover-card-panel { inset-block-start: auto; inset-block-end: calc(100% + var(--lui-popover-card-gap)); }
/* Motion: a fade and a 4px slide from the button, both ways. */
.lui-popover-card-panel[popover] {
  transition: opacity var(--lui-duration-fast) var(--lui-ease-out), translate var(--lui-duration) var(--lui-ease-spring),
    display var(--lui-duration-fast) allow-discrete, overlay var(--lui-duration-fast) allow-discrete;
}
.lui-popover-card-panel[popover]:not(:popover-open) { opacity: 0; translate: 0 -4px; }
@starting-style { .lui-popover-card-panel[popover]:popover-open { opacity: 0; translate: 0 -4px; } }
@media (prefers-reduced-motion: reduce) { .lui-popover-card-panel[popover] { transition: none; } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_side_and_alignment_has_an_area() {
        let ui = Ui::from(crate::Caps::all());
        for side in [Side::Top, Side::Bottom, Side::Start, Side::End] {
            for align in [Align::Start, Align::Center, Align::End] {
                let m = ui
                    .popover("P", html! {})
                    .side(side)
                    .align(align)
                    .render()
                    .into_string();
                assert!(m.contains(area(side, align)), "{side:?} {align:?}");
            }
        }
    }
}
