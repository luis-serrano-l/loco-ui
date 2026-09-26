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
//! **Platform features:** `position: fixed` in the bottom corner (full width at the bottom,
//! 16px from the edges, on a narrow screen); overlapping grid items for the stack; `role="status"` per notice and `role="alert"` for danger; a CSS fade with
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
//! ```

use maud::{Markup, Render, html};

use crate::flash::{Level, parse};
use crate::props::{Prop, PropKind};
use crate::{Icon, Ui};

/// The request's flash messages as toasts in the corner, made by [`Ui::toasts`]; nothing when
/// there are none.
///
/// **Setters.** Switches: `.dismiss()`.
#[derive(Clone, Debug)]
pub struct Toasts<'a> {
    ui: &'a Ui,
    dismiss: bool,
}

impl Toasts<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[Prop::new("dismiss", PropKind::Switch, "")
        .doc("A close link on each toast, back to this page.")];
}

impl Ui {
    /// The flash messages a [`Ui::redirect`] left for this page, as toasts.
    pub fn toasts(&self) -> Toasts<'_> {
        Toasts {
            ui: self,
            dismiss: false,
        }
    }
}

impl Toasts<'_> {
    /// A close link on each toast, back to this page.
    pub fn dismiss(mut self) -> Self {
        self.dismiss = true;
        self
    }
}

impl Render for Toasts<'_> {
    fn render(&self) -> Markup {
        let messages = self.ui.state.flash().map(parse).unwrap_or_default();
        let dismiss = self.dismiss.then(|| self.ui.state.path());
        html! {
            @if !messages.is_empty() {
                ol class="lui-toasts" {
                    @for (level, message) in &messages {
                        li class={ "lui-toast lui-toast-" (level.as_str()) } role=(if *level == Level::Danger { "alert" } else { "status" }) {
                            (match level { Level::Ok => Icon::CircleCheck, Level::Info => Icon::Info, _ => Icon::TriangleAlert })
                            span class="lui-toast-text" { (message) }
                            @if let Some(href) = dismiss {
                                a class="lui-toast-close" href=(href) aria-label={ "Dismiss: " (message) } { (Icon::X) }
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
  position: fixed; z-index: 20; inset-inline-end: var(--lui-space-6); bottom: var(--lui-space-6);
  display: grid; gap: var(--lui-space-3); width: min(22.25rem, calc(100vw - 2rem));
  list-style: none; margin: 0; padding: 0;
}
.lui-toast {
  --lui-toast-tone: var(--lui-muted);
  position: relative; z-index: 3; grid-area: 1 / 1; align-self: end; display: grid; grid-template-columns: 1rem minmax(0, 1fr) auto; align-items: start; column-gap: var(--lui-space-3);
  box-sizing: border-box; padding: var(--lui-space-4); color: var(--lui-fg); background: var(--lui-popover);
  font-size: 0.875rem; line-height: 1.25rem;
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  box-shadow: var(--lui-shadow-lg), var(--lui-highlight); transform-origin: 50% 0;
  animation: lui-toast-out 0.4s ease-in 5s forwards;
  transition: opacity var(--lui-duration) var(--lui-ease-out), translate var(--lui-duration-slow) var(--lui-ease-spring), scale var(--lui-duration-slow) var(--lui-ease-spring);
}
.lui-toast:nth-last-child(2) { z-index: 2; translate: 0 -0.875rem; scale: 0.95; }
.lui-toast:nth-last-child(3) { z-index: 1; translate: 0 -1.75rem; scale: 0.9; }
.lui-toast:nth-last-child(n+4) { z-index: 0; opacity: 0; scale: 0.85; pointer-events: none; }
.lui-toasts:hover > .lui-toast, .lui-toasts:focus-within > .lui-toast { grid-area: auto; translate: none; scale: none; opacity: 1; pointer-events: auto; }
.lui-toast > .lui-icon { margin-top: 0.125rem; color: var(--lui-toast-tone); }
.lui-toast-text { font-weight: 500; }
.lui-toast:hover, .lui-toast:focus-within { animation-play-state: paused; }
.lui-toast-info { --lui-toast-tone: var(--lui-link); }
.lui-toast-ok { --lui-toast-tone: var(--lui-ok); }
.lui-toast-warn { --lui-toast-tone: var(--lui-warn); }
.lui-toast-danger { --lui-toast-tone: var(--lui-danger); animation: none; }
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
}
@media (max-width: 30rem) { .lui-toasts { inset-inline: var(--lui-space-4); bottom: var(--lui-space-4); width: auto; } }
/* Each toast fades in and rises 8px on the spring as the page (or a swap) shows it. The fade
   out is the lui-toast-out animation above. */
@starting-style { .lui-toast { opacity: 0; translate: 0 0.5rem; } }
"#;
