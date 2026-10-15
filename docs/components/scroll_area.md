# Scroll area

`loco_ui::scroll_area` · demo: `/scroll-area`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `ScrollArea` | `ui.scroll_area(label: &str)` | beta |

## Props of `ScrollArea`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What scrolls. |
| `height` | value | `height: &'a str` | `16rem` |  | The box's height as a CSS length (`12rem`, `50vh`). |

## Keyboard of `ScrollArea`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves focus to the scrolling region. | no |
| `ArrowDown / ArrowUp` | Scrolls the focused region a little down or up. | no |
| `PageDown / PageUp / Space` | Scrolls the focused region by about its height. | no |
| `Home / End` | Scrolls the focused region to its start or end. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-scroll-area` | `--lui-scroll-area-height`, `--lui-gray-8`, `--lui-radius` |
| `.lui-scroll-area-content` |  |
| `.lui-scroll-area-edge` | `--lui-duration-fast` |
| `.lui-scroll-area-top` | `--lui-fg` |
| `.lui-scroll-area-bottom` | `--lui-fg` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `scrollbar-gutter: stable` | 94 | 97 | 18.2 |
| `scrollbar-width` | 121 | 64 | 18.2 |
| `scroll-state()` | 133 | no | no |

- Fallback: no edge shadows without scroll-state(): the scrollbar alone shows there is more
- Needs script: no
- Responsive by: content
- Look after: Radix Themes ScrollArea

## Documentation

A box of a set height whose content scrolls inside it: a long list in a sidebar, a log, a
changelog in a card. After Radix Themes ScrollArea: a thin scrollbar in a gray step that
does not move the content when it appears, and (where the browser can tell) a soft shadow
at an edge that has more to scroll to.

**Platform features:** `overflow: auto` (baseline); `scrollbar-gutter: stable` (Chrome 94,
Firefox 97, Safari 18.2) keeps the width when the bar comes and goes; `scrollbar-width`
(Chrome 121, Firefox 64, Safari 18.2) and `scrollbar-color` (Safari 26); the edge shadows are
`scroll-state()` container queries (Chrome 133, not Firefox or Safari yet) on two sticky
edges.

**Accessibility:** a `role="region"` named by its label, with `tabindex="0"` so a keyboard
user can focus it and scroll with the arrow keys (axe's scrollable-region-focusable).
Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** without `scroll-state()` there are no edge shadows: the scrollbar alone says
there is more.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.scroll_area("Changelog").body(html! { p { "v0.2" } }).render().into_string();
assert!(m.starts_with(r#"<div class="lui-scroll-area" role="region" aria-label="Changelog" tabindex="0">"#));
// The same in `lui!`, where the block is the body:
let same = lui! { ScrollArea("Changelog") { p { "v0.2" } } };
assert_eq!(same.into_string(), m);
// 12rem tall.
let m = ui.scroll_area("Log").height("12rem").body(html! { p { "…" } }).render().into_string();
assert!(m.contains(r#"style="--lui-scroll-area-height: 12rem""#));
assert_eq!(lui! { ScrollArea("Log") height="12rem" { p { "…" } } }.into_string(), m);
```
