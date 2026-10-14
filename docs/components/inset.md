# Inset

`loco_ui::inset` · demo: `/container`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Inset` | `ui.inset()` | beta |

## Props of `Inset`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What reaches the edges: a picture, a map, a table. |
| `top` | switch |  | `off` |  | Bleed through the card's top edge too (first in a card with no header). |
| `bottom` | switch |  | `off` |  | Bleed through the card's bottom edge too (last in a card with no footer). |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `overflow: clip` | 90 | 81 | 16 |

- Fallback: overflow: hidden cuts the same corners
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Inset

## Documentation

Content that reaches the edges of the card it sits in: a cover picture across the top, a
map, a table with no margin around it. After Radix Themes Inset: it cancels the card's
padding with negative margins on the sides it names, and rounds the corners it shares
with the card.

**Platform features:** negative `margin-inline` and `margin-block` against the card's
padding (baseline); `overflow: clip` (Chrome 90, Firefox 81, Safari 16) so a picture
takes the card's rounded corners.

**Accessibility:** layout only: a `<div>` with no role. Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** without `overflow: clip` the corners are cut by `overflow: hidden`, the same
to the eye.

An inset bleeds through a card's sides by default. `.top()` and `.bottom()` add the
edges; they work when the inset is the first (or last) thing in a card with no header
(or footer), as Radix's do.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.inset().body(html! { img src="/cover.jpg" alt=""; }).render().into_string();
assert_eq!(m, r#"<div class="lui-inset"><img src="/cover.jpg" alt=""></div>"#);
// The same in `lui!`, where the block is the body:
let same = lui! { Inset { img src="/cover.jpg" alt=""; } };
assert_eq!(same.into_string(), m);
// A cover picture across the top of a card.
let m = ui.inset().top().body(html! { img src="/cover.jpg" alt=""; }).render().into_string();
assert!(m.starts_with(r#"<div class="lui-inset lui-inset-top">"#));
assert_eq!(lui! { Inset top { img src="/cover.jpg" alt=""; } }.into_string(), m);
```
