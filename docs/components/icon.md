# Icon

`loco_ui::icon` · demo: `/button`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Icon` | `ui.icon(icon: Icon)` | stable |

## Props of `IconMark`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `label` | value | `label: &'a str` |  |  | The name a screen reader says, for an icon that carries meaning on its own. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<svg>` | 4 | 3 | 3.2 |
| `role="img"` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: Lucide

## Documentation

A small set of inline SVG icons, drawn in `currentColor` so they take the colour of the
text around them: the ones buttons, menus, tables and alerts need. The shapes are
[Lucide](https://lucide.dev)'s (ISC licence), the set shadcn/ui uses.

**Platform features:** inline `<svg>` (no icon font, no request, no sprite); an icon is
`aria-hidden` unless it is given a `.label()`, which makes it `role="img"` with that name.

**Accessibility:** decorative icons are `aria-hidden`; a meaningful one is `role="img"` with
`aria-label`. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let check = ui.icon(Icon::Check).render().into_string();
assert!(check.starts_with(r#"<svg class="lui-icon" aria-hidden="true""#));
// The same in `lui!`:
let same = lui! { Icon(Icon::Check); };
assert_eq!(same.into_string(), check);
// An icon that is the only content of a link or a button needs a name.
let warn = ui.icon(Icon::TriangleAlert).label("Warning").render().into_string();
assert!(warn.contains(r#"role="img" aria-label="Warning""#));
// An icon on its own renders decoratively, so it drops into any `html!`.
let m = html! { (Icon::Plus) " Add" };
assert!(m.into_string().contains("M12 5v14"));
```
