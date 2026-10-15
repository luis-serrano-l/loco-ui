# Split

`loco_ui::split` · demo: `/layout`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Split` | `ui.split(side: Markup, main: Markup)` | stable |

## Props of `Split`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `side_width` | value | `width: &'a str` |  |  | The side's width, any CSS length. |
| `side_end` | switch |  | `off` |  | The side after the main part, in the markup and on screen. |
| `gap` | number | `n: u8` |  |  | The gap as a step of the `--lui-space-*` scale. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-split` | `--lui-space-4` |
| `.lui-split-side` | `--lui-split-side` |
| `.lui-split-main` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `flex-wrap` | 29 | 28 | 9 |
| `min-inline-size` | 57 | 41 | 12.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: -

## Documentation

A narrow side beside a wide main part, the side stacking on top when the main part would
get narrower than half the width: a settings nav beside its form, filters beside results.

**Platform features:** the "sidebar" flex pattern: `flex-wrap`, the side at its width and
the main part growing with `flex-grow: 999` and `min-inline-size: 50%`, so the switch to
one column happens by container width, with no media query.

**Accessibility:** layout only: no roles, reading order is source order. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.split(html! { nav { "Settings" } }, html! { p { "Form" } }).render().into_string();
assert!(m.contains(r#"<div class="lui-split-side"><nav>Settings</nav></div>"#));
// The same in `lui!`:
let same = lui! { Split(html! { nav { "Settings" } }, html! { p { "Form" } }); };
assert_eq!(same.into_string(), m);
// A wider side, placed after the main part.
let m = ui.split(html! { aside { "Filters" } }, html! { p { "Results" } }).side_width("20rem").side_end();
let m = m.render().into_string();
assert!(m.contains("--lui-split-side: 20rem") && m.find("Results") < m.find("Filters"));
```
