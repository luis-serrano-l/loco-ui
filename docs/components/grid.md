# Grid

`loco_ui::grid` · demo: `/pricing`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Grid` | `ui.grid(min: &str)` | stable |

## Props of `Grid`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What is laid out: each top-level element is a cell. |
| `gap` | number | `n: u8` |  |  | The gap as a step of the `--lui-space-*` scale. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `repeat(auto-fill` | 57 | 52 | 10.1 |
| `@media` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: -

## Documentation

As many equal columns as fit, each at least a given width: cards, stats, a gallery. It
drops to fewer columns on a narrow screen on its own.

**Platform features:** `grid-template-columns: repeat(auto-fill, minmax(<min>, 1fr))`
(CSS Grid, baseline 2017); the minimum travels in a `--lui-grid-min` custom property on
the element. Under a 30rem viewport an `@media` rule caps the minimum at the grid's width
with `min(<min>, 100%)`, so a wide minimum never overflows a phone. (Not at every width:
Taffy, Blitz's layout engine, lays out a single column whenever a track minimum uses
`min()`: FINDINGS, M21.)

**Accessibility:** layout only: no roles, reading order is source order. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.grid("15rem").body(html! { (ui.card().title("A")) (ui.card().title("B")) });
let m = m.render().into_string();
assert!(m.starts_with(r#"<div class="lui-grid" style="--lui-grid-min: 15rem">"#));
let m = ui.grid("10rem").gap(2).body(html! { p { "x" } }).render().into_string();
assert!(m.contains("lui-grid lui-gap-2"));
// The same in `lui!`, where the block is the body:
let same = lui! { Grid("10rem") gap=2 { p { "x" } } };
assert_eq!(same.into_string(), m);
```
