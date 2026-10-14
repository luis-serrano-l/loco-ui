# Toast

`loco_ui::toast` · demo: `/toast`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Toasts` | `ui.toasts()` | stable |

## Props of `Toasts`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `dismiss` | switch |  | `off` |  | A close link on each toast, back to this page. |
| `position` | choice: top-start, top-center, top-end, bottom-start, bottom-center, bottom-end | `position: Position` | `bottom-end` |  | Where the stack sits: a corner, or the middle of the top or bottom edge. |
| `duration` | number | `ms: u32` | `5000` |  | How long a calm toast stays before it fades, in milliseconds. |
| `action` | value | `label: &'a str, href: &'a str` |  |  | A link button on the newest toast ("Undo", "View"). |
| `max` | number | `n: usize` | `all` |  | Show only the newest `n` toasts. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `position: fixed` | 1 | 1 | 1 |
| `role="status"` | 1 | 1 | 1 |
| `role="alert"` | 1 | 1 | 1 |
| `@keyframes` | 43 | 16 | 9 |
| `prefers-reduced-motion` | 74 | 63 | 10.1 |
| `@starting-style` | 117 | 129 | 17.5 |

- Fallback: without CSS animations toasts stay until the next page
- Needs script: no
- Responsive by: viewport
- Look after: shadcn Sonner

## Documentation

Short notices in a stack at the corner of the viewport, outside the page flow, that fade
on their own: "Copied", "Invite sent". The same one-shot cookie as `crate::Ui::flash` carries
them across a Post/Redirect/Get, with the same `level:` lines (`crate::flash::stack`).

The look follows shadcn Sonner: an icon by level, the message and a close ×, the toasts
stacked in depth (the newest in front, two behind it scaled and peeking) and fanned out
while the pointer is over the stack or a toast has focus, in CSS only; under reduced
motion they are shown fanned out, still.

`.position(..)` puts the stack in another corner or the middle of an edge, `.duration(..)`
sets how long a calm toast stays, `.max(..)` keeps the newest few, and `.action(..)` adds
a link button ("Undo") to the newest.

**Platform features:** `position: fixed` in the bottom-end corner (full width at the
bottom or top, 16px from the edges, on a narrow screen); overlapping grid items for the stack; `role="status"` per notice and `role="alert"` for danger; a CSS fade with
`@keyframes` that pauses on `:hover` and `:focus-within` and that
`prefers-reduced-motion: reduce` switches off. Danger toasts never fade. Each toast fades
in and rises on the `--lui-ease-spring` curve through `@starting-style` (Chrome 117,
Firefox 129, Safari 17.5); older browsers show it at once.

**Fallback:** without CSS animations the toasts stay until the next page; the dismiss link
(`.dismiss()`) clears them sooner. `/lui/enhance.js` carries the list across a
swap like the flash.

**Accessibility:** `role="status"` (errors `role="alert"`) in a live list; dismiss links are
named with the message. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** it cannot appear without a request (a toast is the
answer to a round trip), and dismissing one is a navigation, not an instant removal.

```rust
use loco_ui::prelude::*;
// What `ui.redirect("/toast").ok("Invite sent.").danger("Mail server down.")` sends.
let ui = Ui::from_request("/toast", "", "lui-flash=ok%3AInvite%20sent.%0Adanger%3AMail%20server%20down.");
let m = ui.toasts().render().into_string();
assert!(m.contains("lui-toast-ok") && m.contains(r#"role="alert""#));
assert!(ui.toasts().dismiss().render().into_string().contains(r#"href="/toast""#));
assert_eq!(Ui::default().toasts().render().into_string(), "");
// The same in `lui!`:
let same = lui! { Toasts; };
assert_eq!(same.into_string(), m);
// At the top, the newest one only, with an undo link, for eight seconds.
let m = ui.toasts().position(Position::TopCenter).max(1).action("Undo", "/toast/undo").duration(8000).render().into_string();
assert!(m.starts_with(r#"<ol class="lui-toasts" data-position="top-center" style="--lui-toast-duration: 8000ms">"#));
assert_eq!(m.matches("<li").count(), 1);
assert!(m.contains(r#"href="/toast/undo""#) && m.contains("Mail server down."));
let same = lui! { Toasts position=(Position::TopCenter) max=1 action=("Undo", "/toast/undo") duration=8000; };
assert_eq!(same.into_string(), m);
```
