# Kbd

`loco_ui::kbd` · demo: `/typography`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Kbd` | `ui.kbd(keys: &str)` | beta |

## Props of `Kbd`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | Size 1 at 0.7em of the text around it, 2 at 0.75rem (the default), 3 at 0.875rem. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-kbd` | `--lui-font-sans`, `--lui-text-1`, `--lui-gray-12`, `--lui-gray-1`, `--lui-gray-7`, `--lui-gray-3`, `--lui-gray-4`, `--lui-text-2` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<kbd>` | 1 | 1 | 1 |
| `box-shadow` | 10 | 4 | 5.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Kbd

## Documentation

A key or a shortcut drawn as a keycap: `⌘K` in a menu, `Esc` in a dialog's hint, `Ctrl`
`Shift` `P` in the docs. After Radix Themes Kbd: the system font at 0.75rem, a box
1.75em tall and at least as wide, a pale fill and a two-part inset shadow that reads as a
key's edge.

**Platform features:** `<kbd>` (baseline 1995); `box-shadow` (Chrome 10, Firefox 4,
Safari 5.1).

**Accessibility:** `<kbd>` is user input, read as its text ("⌘K" is read as the
characters it holds; write `Ctrl K` where the symbol would be unclear). Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing; it shows a shortcut, it does
not bind one (`accesskey` on a button does that).

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let k = ui.kbd("⌘K").render().into_string();
assert_eq!(k, r#"<kbd class="lui-kbd">⌘K</kbd>"#);
// The same in `lui!`:
let same = lui! { Kbd("⌘K"); };
assert_eq!(same.into_string(), k);
// Larger, in a heading-sized hint.
let k = ui.kbd("Esc").size(3).render().into_string();
assert_eq!(k, r#"<kbd class="lui-kbd" data-size="3">Esc</kbd>"#);
assert_eq!(lui! { Kbd("Esc") size=3; }.into_string(), k);
```
