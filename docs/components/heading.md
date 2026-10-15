# Heading

`loco_ui::heading` · demo: `/typography`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Heading` | `ui.heading(text: &str)` | beta |

## Props of `Heading`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `size` | value | `size: u8` | `6` |  | Radix's sizes 1–9: 12, 14, 16, 18, 20, 24, 28, 35 or 60px. |
| `as_h` | number | `level: u8` | `1` |  | The element, `<h1>` to `<h6>`: the level in the page's outline. |
| `weight` | choice: light, regular, medium, bold | `weight: Weight` | `bold` |  | How heavy the letters are. |
| `trim` | switch |  | `off` |  | Trim the space above the capitals and below the baseline. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-heading-trim` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<h1>` | 1 | 1 | 1 |
| `text-wrap: balance` | 114 | 121 | 17.5 |
| `text-box` | 133 | no | 18.2 |

- Fallback: untrimmed line box without text-box; greedy line breaks without balance
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Heading

## Documentation

A section title at one of nine sizes, its level chosen apart from its size so the outline
stays right when the look calls for a small `<h1>` or a large `<h3>`. After Radix Themes
Heading: sizes 1 to 9 on Radix's type scale (12 to 60px), size 6 and bold by default,
the letters drawn tighter as they grow, and `.trim()` for the space above the capitals
and below the baseline.

**Platform features:** `<h1>` to `<h6>` (baseline 1995); `text-wrap: balance` (Chrome 114,
Firefox 121, Safari 17.5) so a long title breaks into even lines; `.trim()` uses
`text-box` (Chrome 133, Safari 18.2, not Firefox yet) under `@supports`.

**Accessibility:** a real heading element, so screen readers list it and jump to it; the
level is `.as_h(..)`, never implied by the size. Checked by axe-core in headless Firefox on
every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** nothing is missing.

**Fallback:** without `text-box` a trimmed heading keeps its line's half-leading above and
below; without `text-wrap: balance` the lines break greedily.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let h = ui.heading("Projects").render().into_string();
assert_eq!(h, r#"<h1 class="lui-heading" data-size="6">Projects</h1>"#);
// The same in `lui!`:
let same = lui! { Heading("Projects"); };
assert_eq!(same.into_string(), h);
// A small second-level heading in medium weight, its line box trimmed.
let h = ui.heading("Members").size(3).as_h(2).weight(Weight::Medium).trim().render().into_string();
assert_eq!(h, r#"<h2 class="lui-heading lui-heading-trim" data-size="3" data-weight="medium">Members</h2>"#);
assert_eq!(lui! { Heading("Members") size=3 as_h=2 weight="medium" trim; }.into_string(), h);
```
