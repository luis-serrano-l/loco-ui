//! # Toast
//!
//! Short notices in a stack at the corner of the viewport, outside the page flow, that fade
//! on their own: "Copied", "Invite sent". The same one-shot cookie as [`crate::Ui::flash`] carries
//! them across a Post/Redirect/Get, with the same `level:` lines ([`crate::flash::stack`]).
//!
//! The look follows shadcn Sonner: an icon by level, the message and a close ×, the toasts
//! stacked in depth (the newest in front, two behind it scaled and peeking) and fanned out
//! while the pointer is over the stack or a toast has focus, in CSS only; under reduced
//! motion they are shown fanned out, still.
//!
//! `.position(..)` puts the stack in another corner or the middle of an edge, `.duration(..)`
//! sets how long a calm toast stays, `.max(..)` keeps the newest few, and `.action(..)` adds
//! a link button ("Undo") to the newest.
//!
//! **Platform features:** `position: fixed` in the bottom-end corner (full width at the
//! bottom or top, 16px from the edges, on a narrow screen); overlapping grid items for the stack; `role="status"` per notice and `role="alert"` for danger; a CSS fade with
//! `@keyframes` that pauses on `:hover` and `:focus-within` and that
//! `prefers-reduced-motion: reduce` switches off. Danger toasts never fade. Each toast fades
//! in and rises on the `--lui-ease-spring` curve through `@starting-style` (Chrome 117,
//! Firefox 129, Safari 17.5); older browsers show it at once.
//!
//! **Fallback:** without CSS animations the toasts stay until the next page; the dismiss link
//! (`.dismiss()`) clears them sooner. `/lui/enhance.js` carries the list across a
//! swap like the flash.
//!
//! **Accessibility:** `role="status"` (errors `role="alert"`) in a live list; dismiss links are
//! named with the message. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** it cannot appear without a request (a toast is the
//! answer to a round trip), and dismissing one is a navigation, not an instant removal.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // What `ui.redirect("/toast").ok("Invite sent.").danger("Mail server down.")` sends.
//! let ui = Ui::from_request("/toast", "", "lui-flash=ok%3AInvite%20sent.%0Adanger%3AMail%20server%20down.");
//! let m = ui.toasts().render().into_string();
//! assert!(m.contains("lui-toast-ok") && m.contains(r#"role="alert""#));
//! assert!(ui.toasts().dismiss().render().into_string().contains(r#"href="/toast""#));
//! assert_eq!(Ui::default().toasts().render().into_string(), "");
//! // The same in `lui!`:
//! let same = lui! { Toasts; };
//! assert_eq!(same.into_string(), m);
//! // At the top, the newest one only, with an undo link, for eight seconds.
//! let m = ui.toasts().position(Position::TopCenter).max(1).action("Undo", "/toast/undo").duration(8000).render().into_string();
//! assert!(m.starts_with(r#"<ol class="lui-toasts" data-position="top-center" style="--lui-toast-duration: 8000ms">"#));
//! assert_eq!(m.matches("<li").count(), 1);
//! assert!(m.contains(r#"href="/toast/undo""#) && m.contains("Mail server down."));
//! let same = lui! { Toasts position=(Position::TopCenter) max=1 action=("Undo", "/toast/undo") duration=8000; };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::flash::{Level, parse};
use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::{Icon, Ui};

/// Where the toasts stack: a corner, or the middle of the top or bottom edge. Start and end
/// follow the writing direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Position {
    /// Top, at the start edge.
    TopStart,
    /// Top, centred.
    TopCenter,
    /// Top, at the end edge.
    TopEnd,
    /// Bottom, at the start edge.
    BottomStart,
    /// Bottom, centred.
    BottomCenter,
    /// Bottom, at the end edge (the default, as Sonner's).
    #[default]
    BottomEnd,
}

impl Position {
    /// Every value as written in `data-position`.
    pub const NAMES: &'static [&'static str] = &[
        "top-start",
        "top-center",
        "top-end",
        "bottom-start",
        "bottom-center",
        "bottom-end",
    ];

    /// The value as written in `data-position`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Position::TopStart => "top-start",
            Position::TopCenter => "top-center",
            Position::TopEnd => "top-end",
            Position::BottomStart => "bottom-start",
            Position::BottomCenter => "bottom-center",
            Position::BottomEnd => "bottom-end",
        }
    }
}

/// The request's flash messages as toasts in the corner, made by [`Ui::toasts`]; nothing when
/// there are none.
///
/// **Setters.** Values and items: `.position(..)`, `.duration(..)`, `.action(..)`, `.max(..)`;
/// switches: `.dismiss()`.
#[derive(Clone, Debug)]
pub struct Toasts<'a> {
    ui: &'a Ui,
    dismiss: bool,
    position: Position,
    duration: Option<u32>,
    action: Option<(&'a str, &'a str)>,
    max: Option<usize>,
}

impl Toasts<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("dismiss", PropKind::Switch, "")
            .doc("A close link on each toast, back to this page."),
        Prop::new(
            "position",
            PropKind::Choice {
                of: Position::NAMES,
            },
            "position: Position",
        )
        .default("bottom-end")
        .doc("Where the stack sits: a corner, or the middle of the top or bottom edge."),
        Prop::new("duration", PropKind::Number, "ms: u32")
            .default("5000")
            .doc("How long a calm toast stays before it fades, in milliseconds."),
        Prop::new("action", PropKind::Value, "label: &'a str, href: &'a str")
            .doc("A link button on the newest toast (\"Undo\", \"View\")."),
        Prop::new("max", PropKind::Number, "n: usize")
            .default("all")
            .doc("Show only the newest `n` toasts."),
    ];
}

impl Ui {
    /// The flash messages a [`Ui::redirect`] left for this page, as toasts.
    pub fn toasts(&self) -> Toasts<'_> {
        Toasts {
            ui: self,
            dismiss: false,
            position: Position::BottomEnd,
            duration: None,
            action: None,
            max: None,
        }
    }
}

impl<'a> Toasts<'a> {
    /// A close link on each toast, back to this page.
    pub fn dismiss(mut self) -> Self {
        self.dismiss = true;
        self
    }

    /// Where the stack sits: bottom-end (the default), or another corner, or the middle of the
    /// top or bottom edge. At the top the newest toast is the highest.
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// How long a calm toast stays before it fades, in milliseconds (5000 by default);
    /// pointing at the stack pauses it, and danger toasts never fade.
    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = Some(ms);
        self
    }

    /// A link button on the newest toast ("Undo", "View"): the flash cookie carries text
    /// only, so the route that shows the toasts says where it goes.
    pub fn action(mut self, label: &'a str, href: &'a str) -> Self {
        self.action = Some((label, href));
        self
    }

    /// Show only the newest `n` toasts; the older ones are dropped (the stack shows three
    /// in depth whatever the number).
    pub fn max(mut self, n: usize) -> Self {
        self.max = Some(n);
        self
    }
}

impl Render for Toasts<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let mut messages = ui.state.flash().map(parse).unwrap_or_default();
        if let Some(n) = self.max {
            messages.drain(..messages.len().saturating_sub(n));
        }
        let dismiss = self.dismiss.then(|| ui.state.path());
        let position = (self.position != Position::BottomEnd).then(|| self.position.as_str());
        let duration = self
            .duration
            .map(|ms| format!("--lui-toast-duration: {ms}ms"));
        let last = messages.len().saturating_sub(1);
        html! {
            @if !messages.is_empty() {
                ol class="lui-toasts" data-position=[position] style=[duration] {
                    @for (i, (level, message)) in messages.iter().enumerate() {
                        li class={ "lui-toast lui-toast-" (level.as_str()) } role=(if *level == Level::Danger { "alert" } else { "status" }) {
                            (match level { Level::Ok => Icon::CircleCheck, Level::Info => Icon::Info, _ => Icon::TriangleAlert })
                            span class="lui-toast-text" { (message) }
                            @if let Some((label, href)) = self.action.filter(|_| i == last) {
                                (ui.link_button(label, href).small().class("lui-toast-action"))
                            }
                            @if let Some(href) = dismiss {
                                a class="lui-toast-close" href=(href) aria-label=(ui.fill(Text::DismissMessage, &[message])) { (Icon::X) }
                            }
                        }
                    }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After shadcn Sonner: bottom-right cards on the popover surface with the level's icon, the
   message and a close ×. They share one grid cell, so they stack in depth: the newest in
   front, the two before it 14px and 28px higher at 95% and 90% scale, older ones hidden.
   Explicit z-index keeps the newest on top (a transformed toast would otherwise paint over
   the untransformed front one). Pointing at the stack or focusing a toast fans it out (each back in its own row); under
   reduced motion it is always fanned out. Full width at the bottom on a narrow viewport. */
.lui-toasts {
  position: fixed; z-index: var(--lui-z-toast); inset-inline-end: var(--lui-space-6); bottom: var(--lui-space-6);
  display: grid; gap: var(--lui-space-3); width: min(22.25rem, calc(100vw - 2rem));
  list-style: none; margin: 0; padding: 0; --lui-toast-dir: -1;
}
/* .position(..): the other corners and the middle of an edge. At the top the stack hangs
   down: the toasts behind peek out below the newest, and the fan opens downwards. */
.lui-toasts[data-position^=top] { --lui-toast-dir: 1; bottom: auto; top: var(--lui-space-6); }
.lui-toasts[data-position^=top] > .lui-toast { align-self: start; transform-origin: 50% 100%; }
.lui-toasts[data-position^=top]:is(:hover, :focus-within) { display: flex; flex-direction: column-reverse; }
.lui-toasts[data-position$=start] { inset-inline-end: auto; inset-inline-start: var(--lui-space-6); }
.lui-toasts[data-position$=center] { inset-inline-end: auto; inset-inline-start: 50%; translate: -50% 0; }
.lui-toasts[data-position$=center]:dir(rtl) { translate: 50% 0; }
.lui-toast {
  --lui-toast-tone: var(--lui-muted);
  position: relative; z-index: 3; grid-area: 1 / 1; align-self: end; display: grid; grid-template-columns: 1rem minmax(0, 1fr); grid-auto-flow: column; grid-auto-columns: auto; align-items: start; column-gap: var(--lui-space-3);
  box-sizing: border-box; padding: var(--lui-space-4); color: var(--lui-fg); background: var(--lui-popover);
  font-size: var(--lui-text-2); line-height: 1.25rem;
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  box-shadow: var(--lui-shadow-lg), var(--lui-highlight); transform-origin: 50% 0;
  animation: lui-toast-out 0.4s ease-in var(--lui-toast-duration, 5s) forwards;
  transition: opacity var(--lui-duration) var(--lui-ease-out), translate var(--lui-duration-slow) var(--lui-ease-spring), scale var(--lui-duration-slow) var(--lui-ease-spring);
}
.lui-toast:nth-last-child(2) { z-index: 2; translate: 0 calc(var(--lui-toast-dir) * 0.875rem); scale: 0.95; }
.lui-toast:nth-last-child(3) { z-index: 1; translate: 0 calc(var(--lui-toast-dir) * 1.75rem); scale: 0.9; }
.lui-toast:nth-last-child(n+4) { z-index: 0; opacity: 0; scale: 0.85; pointer-events: none; }
.lui-toasts:hover > .lui-toast, .lui-toasts:focus-within > .lui-toast { grid-area: auto; translate: none; scale: none; opacity: 1; pointer-events: auto; }
.lui-toast > .lui-icon { margin-top: 0.125rem; color: var(--lui-toast-tone); }
.lui-toast-text { font-weight: 500; }
.lui-toast:hover, .lui-toast:focus-within { animation-play-state: paused; }
.lui-toast-info { --lui-toast-tone: var(--lui-link); }
.lui-toast-ok { --lui-toast-tone: var(--lui-ok); }
.lui-toast-warn { --lui-toast-tone: var(--lui-warn); }
.lui-toast-danger { --lui-toast-tone: var(--lui-danger); animation: none; }
.lui-toast-action { align-self: center; margin-block: -0.25rem; }
.lui-toast-close {
  position: relative; display: inline-grid; place-items: center; width: 1.25rem; height: 1.25rem;
  color: var(--lui-muted); text-decoration: none; border-radius: var(--lui-radius-sm);
}
.lui-toast-close > .lui-icon { width: 0.875rem; height: 0.875rem; }
.lui-toast-close::after { content: ""; position: absolute; inset: calc((1.25rem - var(--lui-hit)) / 2); }
.lui-toast-close:hover { color: var(--lui-fg); background: var(--lui-accent); }
@keyframes lui-toast-out { to { opacity: 0; visibility: hidden; transform: translateY(0.5rem); } }
@media (prefers-reduced-motion: reduce) {
  .lui-toast { animation: none; }
  .lui-toasts > .lui-toast { grid-area: auto; translate: none; scale: none; opacity: 1; pointer-events: auto; }
  .lui-toasts[data-position^=top] { display: flex; flex-direction: column-reverse; }
}
@media (max-width: 30rem) {
  .lui-toasts { inset-inline: var(--lui-space-4); bottom: var(--lui-space-4); width: auto; translate: none; }
  .lui-toasts[data-position^=top] { bottom: auto; top: var(--lui-space-4); }
}
/* Each toast fades in and rises 8px on the spring as the page (or a swap) shows it. The fade
   out is the lui-toast-out animation above. */
@starting-style { .lui-toast { opacity: 0; translate: 0 0.5rem; } }
"#;
