# Cluster

`loco_ui::cluster` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Cluster` | `ui.cluster()` | stable |

## Props of `Cluster`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What goes in the row: each top-level element beside the one before. |
| `between` | switch |  | `off` |  | First child at the start, last at the end, the rest spread between. |
| `end` | switch |  | `off` |  | Children packed at the end of the row (a dialog's or card's actions). |
| `gap` | number | `n: u8` |  |  | The gap as a step of the `--lui-space-*` scale. |
| `align` | choice: start, center, end, baseline, stretch | `align: Align` | `center` |  | Where the children sit across the row (`align-items`). |
| `justify` | choice: start, center, end, between | `justify: Justify` | `start` |  | How the children share the row (`justify-content`). |
| `padding` | number | `n: u8` |  |  | Padding on every side as a step of the `--lui-space-*` scale. |
| `nowrap` | switch |  | `off` |  | Keep the children on one line; they shrink instead of wrapping. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-cluster` | `--lui-space-2` |
| `.lui-cluster-between` |  |
| `.lui-cluster-end` |  |
| `.lui-cluster-nowrap` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `flex-wrap` | 29 | 28 | 9 |
| `gap` | 84 | 63 | 14.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: -

## Documentation

Children side by side, wrapping onto the next line when they run out of room: a row of
buttons, badges, filters or links.

**Platform features:** `flex-wrap` with `gap` (Chrome 84, Firefox 63, Safari 14.1), so a
wrapped line keeps the same spacing and no child carries a margin. `.align(..)` and
`.justify(..)` write `data-align` and `data-justify` (`align-items`, `justify-content`),
`.padding(n)` a step of the space scale, as Radix Themes Flex's props do.

**Accessibility:** layout only: no roles, reading order is source order. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.cluster().body(html! { (ui.badge("rust")) (ui.badge("maud")) }).render().into_string();
assert!(m.starts_with(r#"<div class="lui-cluster">"#));
// A toolbar: the title on the left, the actions pushed to the right.
let m = ui.cluster()
    .between()
    .gap(4)
    .body(html! { h2 { "Orders" } (ui.button("New order").primary()) });
let m = m.render().into_string();
assert!(m.contains(r#"class="lui-cluster lui-cluster-between lui-gap-4""#));
// The same in `lui!`, where the block is the body:
let same = lui! { Cluster between gap=4 { h2 { "Orders" } Button("New order") primary; } };
assert_eq!(same.into_string(), m);
// On one line, on the text baseline, centred, with padding round it.
let m = ui.cluster().nowrap().align(Align::Baseline).justify(Justify::Center).padding(3)
    .body(html! { h2 { "Plan" } span { "$9" } }).render().into_string();
assert!(m.starts_with(r#"<div class="lui-cluster lui-cluster-nowrap lui-pad-3" data-align="baseline" data-justify="center">"#));
let same = lui! { Cluster nowrap align="baseline" justify="center" padding=3 { h2 { "Plan" } span { "$9" } } };
assert_eq!(same.into_string(), m);
```
