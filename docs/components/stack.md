# Stack

`loco_ui::stack` · demo: `/marquee`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Stack` | `ui.stack()` | stable |

## Props of `Stack`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What is stacked: each top-level element under the one before. |
| `gap` | number | `n: u8` |  |  | The gap as a step of the `--lui-space-*` scale. |
| `align` | choice: start, center, end, baseline, stretch | `align: Align` | `stretch` |  | Where the children sit across the column (`align-items`). |
| `justify` | choice: start, center, end, between | `justify: Justify` | `start` |  | How the children share the column's height (`justify-content`). |
| `padding` | number | `n: u8` |  |  | Padding on every side as a step of the `--lui-space-*` scale. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `gap` | 84 | 63 | 14.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: -

## Documentation

Children one under the other with an even gap between them and no margins to fight: the
vertical rhythm of a form, a card body, a settings page.

**Platform features:** a flex column with `gap` (Chrome 84, Firefox 63, Safari 14.1);
the gap is a step of the `--lui-space-*` scale. `.align(..)` and `.justify(..)` write
`data-align` and `data-justify` (`align-items`, `justify-content`), `.padding(n)` a step
of the same scale, as Radix Themes Flex's props do.

**Accessibility:** layout only: no roles, reading order is source order. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.stack().body(html! { p { "One" } p { "Two" } }).render().into_string();
assert_eq!(m, r#"<div class="lui-stack"><p>One</p><p>Two</p></div>"#);
// `.gap(n)` picks a step: n × 4px with the default `--lui-space`.
let m = ui.stack().gap(2).body(html! { p { "Tight" } }).render().into_string();
assert!(m.contains(r#"class="lui-stack lui-gap-2""#));
// The same in `lui!`, where the block is the body:
let same = lui! { Stack gap=2 { p { "Tight" } } };
assert_eq!(same.into_string(), m);
// Centred across, packed at the end, with padding round it.
let m = ui.stack().align(Align::Center).justify(Justify::End).padding(4)
    .body(html! { p { "Done" } }).render().into_string();
assert_eq!(m, r#"<div class="lui-stack lui-pad-4" data-align="center" data-justify="end"><p>Done</p></div>"#);
let same = lui! { Stack align="center" justify="end" padding=4 { p { "Done" } } };
assert_eq!(same.into_string(), m);
```
