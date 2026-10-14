# Section

`loco_ui::section` · demo: `/container`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Section` | `ui.section()` | beta |

## Props of `Section`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What the band holds. |
| `size` | value | `size: u8` | `3` |  | Radix's sizes 1–4: 24, 40, 64 or 80px above and below. |
| `label` | value | `label: &'a str` |  | `aria-label` | A name (`aria-label`), which makes the section a region landmark. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<section>` | 1 | 1 | 1 |
| `padding-block` | 87 | 66 | 14.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Section

## Documentation

A band of a page with room above and below it: the hero, the features, the pricing, each
one `<section>` apart from the next. After Radix Themes Section: four heights of vertical
padding (24, 40, 64 and 80px), size 3 by default.

**Platform features:** `<section>` (baseline 2011) and `padding-block` (Chrome 87,
Firefox 66, Safari 14.1).

**Accessibility:** a `<section>` with no name is a generic block to assistive tech; give it
a heading inside (a `ui.heading(..)`) and `.label(..)` to make it a named region in the
page's landmarks. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.section().body(html! { h2 { "Features" } }).render().into_string();
assert_eq!(m, r#"<section class="lui-section"><h2>Features</h2></section>"#);
// The same in `lui!`, where the block is the body:
let same = lui! { Section { h2 { "Features" } } };
assert_eq!(same.into_string(), m);
// The tallest band, named for the landmarks list.
let m = ui.section().size(4).label("Pricing").body(html! { h2 { "Pricing" } }).render().into_string();
assert_eq!(m, r#"<section class="lui-section" data-size="4" aria-label="Pricing"><h2>Pricing</h2></section>"#);
assert_eq!(lui! { Section size=4 label="Pricing" { h2 { "Pricing" } } }.into_string(), m);
```
