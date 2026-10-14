# Container

`loco_ui::container` · demo: `/container`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Container` | `ui.container()` | beta |

## Props of `Container`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What the column holds. |
| `size` | value | `size: u8` | `4` |  | Radix's sizes 1–4: at most 448, 688, 880 or 1136px wide. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `max-width` | 1 | 1 | 1 |
| `min()` | 79 | 75 | 11.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Container

## Documentation

A centred column of a fixed greatest width: the readable middle of a page, a narrow form,
a wide dashboard. After Radix Themes Container: four widths (448, 688, 880 and 1136px),
size 4 by default, centred by auto margins, as wide as its parent below that.

**Platform features:** `max-width` with `margin-inline: auto` (baseline 2010); `min()`
(Chrome 79, Firefox 75, Safari 11.1) keeps the column inside a narrower parent.

**Accessibility:** layout only: a `<div>` with no role; reading order is source order.
Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.container().body(html! { p { "Centred" } }).render().into_string();
assert_eq!(m, r#"<div class="lui-container"><p>Centred</p></div>"#);
// The same in `lui!`, where the block is the body:
let same = lui! { Container { p { "Centred" } } };
assert_eq!(same.into_string(), m);
// Size 1: 448px, for a sign-in form.
let m = ui.container().size(1).body(html! { p { "Narrow" } }).render().into_string();
assert_eq!(m, r#"<div class="lui-container" data-size="1"><p>Narrow</p></div>"#);
assert_eq!(lui! { Container size=1 { p { "Narrow" } } }.into_string(), m);
```
