# Skeleton

`loco_ui::skeleton` · demo: `/stream`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Skeleton` | `ui.skeleton(lines: usize)` | stable |

## Props of `Skeleton`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `label` | value | `label: &'a str` | `Loading` |  | What screen readers hear instead of "Loading". |
| `heading` | switch |  | `off` |  | Start with a wider, taller bar standing in for a heading. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `aria-busy` | 1 | 1 | 1 |
| `role="status"` | 1 | 1 | 1 |
| `@keyframes` | 43 | 16 | 9 |
| `prefers-reduced-motion` | 74 | 63 | 10.1 |

- Fallback: without CSS animations the bars are still
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Skeleton

## Documentation

Grey bars in the shape of content that is still coming: the placeholder for a streamed
`Ui::slot` (feature `http`), a lazy panel, anything the server fills in later.

**Platform features:** `aria-busy="true"` and `role="status"` with a visually hidden
label, so assistive technology hears "Loading" once instead of reading empty boxes; a
shimmer drawn with `@keyframes` over a `linear-gradient` that
`prefers-reduced-motion: reduce` stops.

**Fallback:** without CSS animations the bars are still; nothing else differs.

**Accessibility:** `role="status"` with `aria-busy` and a "Loading" name; the bars are
`aria-hidden`. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** it cannot be removed by the client; it is replaced
when the real content arrives (a streamed slot, a swap, the next page).

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
assert_eq!(ui.skeleton(3).render().into_string().matches("lui-skeleton-line").count(), 3);
let m = ui.skeleton(2).label("Loading orders").heading().render().into_string();
assert!(m.contains("Loading orders") && m.contains("lui-skeleton-heading"));
// The same in `lui!`:
let same = lui! { Skeleton(2) label="Loading orders" heading; };
assert_eq!(same.into_string(), m);
```
