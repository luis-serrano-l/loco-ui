# Flash

`loco_ui::flash` · demo: `/settings`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Flash` | `ui.flash()` | stable |

## Props of `Flash`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `dismiss` | switch |  | `off` |  | A dismiss link on each message, back to this page (which no longer has the flash). |
| `auto_hide` | switch |  | `off` |  | Fade calm messages out after a few seconds. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `soft` |  | Each message's callout: soft, surface or outline, as on an alert. |
| `title` | value | `text: &'a str` |  |  | A bold line over each message. |
| `action` | value | `label: &'a str, href: &'a str` |  |  | A link button under each message ("Undo", "View"). |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `cookie` | 1 | 1 | 1 |
| `role="status"` | 1 | 1 | 1 |
| `role="alert"` | 1 | 1 | 1 |
| `@keyframes` | 43 | 16 | 9 |
| `prefers-reduced-motion` | 74 | 63 | 10.1 |

- Fallback: without CSS animations the message stays
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Callout

## Documentation

One-shot status messages that survive a Post/Redirect/Get, no script. Four levels
(`info`, `ok`, `warn`, `danger`), several at once stacked in arrival order, an optional
dismiss link, and an optional auto-hide for the calm levels. Each message is an alert's
callout, soft unless `.variant(..)` makes them surface or outline. `.title(..)` puts a
bold line over each message and `.action(..)` a link button under it.

**Platform features:** a cookie with `Max-Age=60` set by `crate::Ui::redirect` on the
redirect, read on the next request through `UiState`, and cleared by the next
`crate::Ui::page` that shows it. `role="status"` announces info, ok and warn politely;
`role="alert"` announces danger at once. Auto-hide is a CSS animation
(`@keyframes`, `animation-fill-mode: forwards`) that `prefers-reduced-motion: reduce`
switches off, so the message then stays until the next page.

The cookie value is plain text: one message per line, each optionally prefixed with its
level (`ok:Saved.`). A line without a known prefix is `info`. `ui.redirect(to).ok(..)`
writes it; `stack` builds the same text by hand.

**Dismiss:** a link back to the page (`.dismiss()`). Reading the flash already
queued the cookie's deletion on that response, so following the link renders the page
without it; with `/lui/enhance.js` inside a swap root it updates in place.

**Accessibility:** `role="status"` for information, `role="alert"` for errors; the dismiss
link is named with the message. Checked by axe-core in headless Firefox on every demo route,
both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** it cannot vanish in place when dismissed; the
dismiss link is a navigation.

**Fallback:** without CSS animations the message stays; nothing else differs.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/settings", "", "lui-flash=Saved.");
let m = ui.flash().render().into_string();
assert!(m.contains("lui-flash-info") && m.contains("Saved."));
assert_eq!(Ui::default().flash().render().into_string(), "", "no message, no banner");

// What `ui.redirect("/settings").ok("Saved.").danger("Avatar too large.")` sends, shown
// with a dismiss link and auto-hide.
let ui = Ui::from_request("/settings", "", "lui-flash=ok%3ASaved.%0Adanger%3AAvatar%20too%20large.");
let m = ui.flash().dismiss().auto_hide().render().into_string();
assert!(m.contains(r#"role="alert""#) && m.contains("lui-flash-auto"));
assert_eq!(m.matches(r#"href="/settings""#).count(), 2);

// The same in `lui!`:
let same = lui! { Flash dismiss auto_hide; };
assert_eq!(same.into_string(), m);
// Outlined callouts instead of tinted ones.
let m = ui.flash().variant(Variant::Outline).render().into_string();
assert!(m.starts_with(r#"<div class="lui-flash" data-variant="outline">"#) && m.contains("lui-callout-outline"));
assert_eq!(lui! { Flash variant="outline"; }.into_string(), m);
// A title over the message and a link under it.
let ui = Ui::from_request("/notes", "", "lui-flash=ok%3ANote%20moved%20to%20Archive.");
let m = ui.flash().title("Moved").action("Open Archive", "/archive").render().into_string();
assert!(m.contains(r#"<p class="lui-flash-title">Moved</p><p class="lui-flash-text">Note moved to Archive.</p>"#));
assert!(m.contains(r#"href="/archive""#));
assert_eq!(lui! { Flash title="Moved" action=("Open Archive", "/archive"); }.into_string(), m);
```
