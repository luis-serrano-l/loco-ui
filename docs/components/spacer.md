# Spacer

`loco_ui::spacer`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Spacer` | `ui.spacer()` | beta |

## Props of `Spacer`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `min` | value | `length: &'a str` |  |  | The least room it keeps (a CSS length), even when the row is full. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-spacer` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `flex: 1 1 auto` | 29 | 28 | 9 |

- Fallback: an empty block outside a flex container
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Box flexGrow=1

## Documentation

An empty part of a row or column that takes the room left over, pushing what follows it to
the far end: the account menu to the right of a toolbar, a footer to the bottom of a
sidebar. After Radix Themes Box with `flexGrow="1"`, the way Radix's own examples push
a toolbar apart (Radix has no spacer component).

**Platform features:** `flex: 1 1 auto` (Chrome 29, Firefox 28, Safari 9) inside a flex
row or column (a `ui.cluster()`, a `ui.stack()`).

**Accessibility:** `aria-hidden`: it holds nothing. Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** outside a flex container it is an empty block of no height.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.spacer().render().into_string();
assert_eq!(m, r#"<div class="lui-spacer" aria-hidden="true"></div>"#);
// The same in `lui!`:
let same = lui! { Spacer; };
assert_eq!(same.into_string(), m);
// At least 2rem, even when the row is full.
let m = ui.spacer().min("2rem").render().into_string();
assert_eq!(m, r#"<div class="lui-spacer" aria-hidden="true" style="flex: 1 0 2rem"></div>"#);
assert_eq!(lui! { Spacer min="2rem"; }.into_string(), m);
```
