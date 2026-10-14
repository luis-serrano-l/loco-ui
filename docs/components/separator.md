# Separator

`loco_ui::separator` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Separator` | `ui.separator()` | stable |

## Props of `Separator`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `label` | value | `text: &'a str` |  |  | A word in the middle of the line. |
| `vertical` | switch |  | `off` |  | Down instead of across, between items in a row (a `ui.cluster`). |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<hr>` | 1 | 1 | 1 |
| `aria-orientation` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Separator

## Documentation

A hairline between groups of content, across or down, optionally with a word in the middle
("or" between two sign-in methods).

**Platform features:** `<hr>` for a plain horizontal rule (its semantics are a thematic
break); a labelled or vertical one is a `role="separator"` element with
`aria-orientation`.

**Accessibility:** a native `<hr>` or `role="separator"` with `aria-orientation`. Checked by
axe-core in headless Firefox on every demo route, both capability variants, light and dark
(no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
assert_eq!(ui.separator().render().into_string(), r#"<hr class="lui-separator">"#);
let or = ui.separator().label("or").render().into_string();
assert!(or.contains(r#"role="separator""#) && or.contains(">or<"));
assert!(ui.separator().vertical().render().into_string().contains(r#"aria-orientation="vertical""#));
// The same in `lui!`:
let same = lui! { Separator label="or"; };
assert_eq!(same.into_string(), or);
```
