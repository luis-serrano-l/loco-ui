# Pagination

`loco_ui::pagination` · demo: `/pagination`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Pagination` | `ui.pagination(href: &str)` | beta |

## Props of `Pagination`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `id` | value | `id: &'a str` |  |  | Read and write `?page.<id>=` instead of `?page=`, so two lists on one page page on their own. |
| `page` | number | `page: usize` |  |  | Show page `page` (1-based) whatever the request says. |
| `total` | number | `total: usize` | `0` |  | How many items the list holds. |
| `per_page` | number | `per_page: usize` | `10` |  | Items a page. |
| `window` | number | `neighbours: usize` | `1` |  | How many pages to number on each side of the current one. |
| `param` | item | `key: &'a str, value: &'a str` |  |  | A query pair every link carries (a filter, a sort). |
| `first_last` | switch |  | `off` |  | "First" and "Last" links outside Previous and Next. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-pagination` |  |
| `.lui-pagination-link` | `--lui-control-h`, `--lui-hit` |
| `.lui-pagination-step` |  |
| `.lui-pagination-gap` | `--lui-control-h`, `--lui-muted` |
| `.lui-pagination-wide` |  |
| `.lui-pagination-label` |  |
| `.lui-pagination-narrow` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<nav>` | 1 | 1 | 1 |
| `aria-current="page"` | 1 | 1 | 1 |
| `rel="prev"` | 1 | 1 | 1 |
| `<link rel>` | 1 | 1 | 1 |
| `@container` | 105 | 110 | 16 |

- Fallback: none needed: every page is a link
- Needs script: no
- Responsive by: container
- Look after: shadcn/ui Pagination

## Documentation

Numbered page links for a list too long for one page: previous and next, the first and
last pages, the current page's neighbours and an ellipsis over each gap. The page shown is
the request's `?page=` (or `?page.<id>=` once given an id), so every page is a URL that
can be bookmarked, and the links keep whatever else the caller names. After shadcn/ui
Pagination.

**Platform features:** a `<nav>` of ordinary links (baseline since HTML 2), the current
page marked `aria-current="page"`, the previous and next links `rel="prev"` and
`rel="next"`; `Pagination::link_rels` puts the same two in the page's `<head>` as
`<link rel>`. A container query (`@container`, Chrome 105, Firefox 110, Safari 16) shows
every number from 30rem; narrower, only the first, the current and the last stay, and
Previous and Next keep their icons.

**Accessibility:** a `<nav>` named "Pages" around a list of links; the current page has
`aria-current="page"`; the ellipses are `aria-hidden`; at either end Previous or Next is a
disabled link (`aria-disabled`, no `href`) so the row keeps its shape. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing: each page is a navigation.

**Fallback:** none needed. Every control is a link.

```rust
use loco_ui::prelude::*;
// Page 5 of 20 (200 items, 10 a page).
let ui = Ui::from_request("/posts", "page=5", "");
let m = ui.pagination("/posts").total(200);
assert_eq!((m.current(), m.pages(), m.range()), (5, 20, 40..50));
let html = m.render().into_string();
assert!(html.contains(r#"href="/posts?page=5" aria-current="page""#));
assert!(html.contains(r#"href="/posts?page=4" rel="prev""#) && html.contains(r#"href="/posts?page=6" rel="next""#));
// An id of its own, 25 a page, two neighbours each side, first and last links, and the
// filter carried along.
let ui = Ui::from_request("/posts", "page.posts=3&q=rust", "");
let m = ui.pagination("/posts").id("posts").total(400).per_page(25).window(2).first_last().param("q", "rust");
let html = m.render().into_string();
assert!(html.contains(r#"href="/posts?q=rust&amp;page.posts=16">"#));
// The same in `lui!`:
let same = lui! { Pagination("/posts") id="posts" total=400 per_page=25 window=2 first_last param=("q", "rust"); };
assert_eq!(same.into_string(), html);
```
