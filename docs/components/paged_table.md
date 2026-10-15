# Paged table

`loco_ui::paged_table`

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-paged-table` |  |
| `.lui-paged-table-nav` | `--lui-space-2`, `--lui-space-4`, `--lui-text-2`, `--lui-muted` |
| `.lui-paged-table-pages` | `--lui-space-1` |
| `.lui-paged-table-gap` |  |
| `.lui-paged-table-of` | `--lui-space-2`, `--lui-fg` |
| `.lui-paged-table-jump` | `--lui-space-2` |
| `.lui-paged-table-per` | `--lui-space-2` |
| `.lui-paged-table-page` |  |
| `.lui-paged-table-num` |  |
| `.lui-paged-table-end` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `?page.<id>=n` | 1 | 1 | 1 |
| `<select>` | 1 | 1 | 1 |
| `<input type="number">` | 6 | 29 | 5.1 |
| `<output>` | 10 | 4 | 7 |

- Fallback: none needed: every control is a link or a form
- Needs script: no
- Responsive by: container
- Look after: shadcn data table, Origin UI

## Documentation

A `crate::Ui::table` with `.paged(total)`, for data too long for one page: first, previous, numbered, next and last
links with an ellipsis over long ranges, a jump-to-page form, a page-size `<select>`, and a
line saying which rows of how many are shown (`1–10 of 1,234`). Everything is a URL, so a
page can be bookmarked, and the sort, filter and columns survive paging.

**Platform features:**
- Ordinary links to `?page.<id>=n` that keep the table's sort, filter, columns and page
  size (`sort.<id>`, `q.<id>`, `q.<id>.<col>`, `cols.<id>`, `per.<id>`); the current page is `aria-current="page"` and the previous/next links are `rel="prev"` /
  `rel="next"`. Past seven pages only the first, the last and the current one's neighbours
  are numbered; the gaps are an `aria-hidden` ellipsis.
- `<form method="get">` with `<input type="number">` (`min`, `max`) to jump to a page, and another
  with a `<select>` for rows per page. Their buttons submit them, so both work with no script
  and by keyboard.
- The whole block (table and pager) is one swap root, so with the enhancement script a
  sort, filter, page or size change replaces both and the page links never go stale.
- `<output>` for the range, so assistive tech announces it as a result. Counts use a comma
  every three digits.

**Accessibility:** the pages are a `<nav>` named "Pages" of links with `rel=prev/next` and
`aria-current` on the current page; the page jump and page size are labelled forms. Checked
by axe-core in headless Firefox on every demo route, both capability variants, light and
dark (no serious or critical violation).

**What it does not do without script:** load the next page on scroll; paging, sorting and
filtering are each a navigation.

**Fallback:** none needed. Every control is a link or a form.

**Server state:** the page size is `per.<id>`, a state key, so the size a visitor picked
is remembered in the `lui-ui` cookie and read back through `state.per_page(id)`. Every
page link still names it, so a shared URL shows the same rows for everyone. The page
itself (`page.<id>`), like the sort and the filter, is a query key only: a bookmark of the
bare path starts on page 1. Each key carries the table's id, so two paged tables on one page
page on their own.

```rust
use loco_ui::{prelude::*, table::Row};
// The URL's sort, filter and page, and the page size the visitor picked before.
let ui = Ui::from_request("/table", "sort.files=-name&q.files=a&page.files=20", "lui-ui=per.files=25");
let files = ui.table("files", "/table").column("name", "Name").sortable().column("note", "Note");
assert_eq!((files.page(), files.per_page()), (20, 25), "what a database query needs");
// Only this page's rows, and the total after filtering.
let html = files.rows([Row::new([html! { "a" }, html! { "b" }])]).paged(1234).render().into_string();
assert!(html.contains("476–500 of 1,234"));
assert!(html.contains("per.files=25&amp;page.files=50\">Last"));
assert!(html.contains("<select name=\"per.files\""));
```
