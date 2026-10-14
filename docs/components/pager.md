# Load-more list

`loco_ui::pager` · demo: `/list`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Pager` | `ui.pager(href: &str, total: usize)` | stable |

## Props of `Pager`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `per_page` | number | `per_page: usize` | `10` |  | Rows per page. |
| `rows` | value | `row: impl Fn(usize) -> Markup + 'a` |  |  | Row `i` (0-based) of the list. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `view-transition-name` | 111 | 144 | 18 |
| `scroll-margin` | 69 | 90 | 14.1 |

- Fallback: plain navigation to ?page=n#more
- Needs script: partly: click-to-load; scroll-to-load needs script
- Responsive by: content
- Look after: shadcn Pagination

## Documentation

A long list delivered a page at a time with a "Load more" link, no script.

**Platform features:**
- Ordinary links to `?page=n`. The key stays bare: a load-more list has no id, and a page
  holds one. A paged table beside it pages by its own `page.<id>` (see `crate::table`),
  so the two no longer move together.
- `view-transition-name` on the list, together with the layout's
  `@view-transition { navigation: auto }` (Chrome 126+, Safari 18.2+) or the enhancement
  script's `startViewTransition`, so the new rows fade in under the old ones. The button
  has no name on purpose: a named button morphs from its old spot to its new one, which
  reads as a green block sliding down over the fresh rows.
- `scroll-margin` + fragment `#more` keeps the viewport on the new rows after navigation.

**Accessibility:** a real link for "Load more"; the new rows land after an anchor so focus
and reading continue there. Checked by axe-core in headless Firefox on every demo route,
both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** infinite scroll or keeping the scroll position
across pages.

**Fallback:** without `Caps::ViewTransitions` the transition names are omitted and the page
navigates normally; the `#more` fragment still scrolls to the new rows.

**Finding:** true infinite scroll (loading on scroll) is impossible without script.
Cumulative pages (`?page=3` shows rows 1..3×N) give the same *feel* with one click per page.

```rust
use loco_ui::prelude::*;
// `?page=2` of 30 rows, 10 at a time: rows 0 to 19 are shown.
let ui = Ui::from_request("/list", "page=2", "");
let list = ui.pager("/list", 30).per_page(10);
assert_eq!(list.shown(), 20);
let m = list.rows(|i| html! { "Row " (i + 1) }).render().into_string();
assert!(m.contains("Row 20") && !m.contains("Row 21") && m.contains("?page=3#more"));
// The same in `lui!`:
let same = lui! { Pager("/list", 30) per_page=10 rows=|i| { "Row " (i + 1) }; };
assert_eq!(same.into_string(), m);
```
