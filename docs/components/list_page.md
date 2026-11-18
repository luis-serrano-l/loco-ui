# ListPage

`loco_ui::blocks::list_page` · demo: `/blocks/list`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `ListPage` | `ui.list_page(title: &str)` | beta |

## Props of `ListPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `description` | value | `text: &'a str` |  |  | A line under the title. |
| `actions` | value | `actions: Markup` |  |  | What sits beside the title: a New button, an export link. |
| `search` | value | `label: &'a str` |  |  | A search box named `q` that searches this page and keeps the current filter; `label` names it. |
| `filter` | item | `label: &'a str, href: &'a str` |  |  | A filter link. The current one is the one whose query matches the page's. |
| `badge` | modifier | `text: impl Display` |  |  | A count after the last filter's label. |
| `count` | value | `text: &'a str` |  |  | How many there are, above the list (`"42 customers"`). |
| `body` | value | `body: Markup` |  |  | The list: a table, cards or an empty state. |
| `footer` | value | `footer: Markup` |  |  | What goes under the list: the page links. |

## Keyboard of `ListPage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | In the search box, searches; on a filter, follows it. | no |
| `Tab` | Moves through the actions, the search, the filters and the list. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-list-page` | `--lui-space-4` |
| `.lui-list-page-head` | `--lui-space-3` |
| `.lui-list-page-title` |  |
| `.lui-list-page-description` | `--lui-space-1`, `--lui-muted` |
| `.lui-list-page-actions` | `--lui-space-2` |
| `.lui-list-page-toolbar` | `--lui-space-3` |
| `.lui-list-page-filters` | `--lui-space-1` |
| `.lui-list-page-filter` | `--lui-space-2`, `--lui-control-h`, `--lui-space-3`, `--lui-radius`, `--lui-muted`, `--lui-text-2`, `--lui-gray-3`, `--lui-fg`, `--lui-gray-4`, `--lui-hit` |
| `.lui-list-page-count` | `--lui-muted`, `--lui-text-2` |
| `.lui-list-page-body` |  |
| `.lui-list-page-footer` | `--lui-space-3` |

## Documentation

The index page of a resource: a title with its actions, a search box and filter links, how
many there are, the list itself (a table, cards, an empty state) and what goes under it
(the page links). The index page a scaffold writes, ready-made. After the shadcn/ui Tasks
example, built from Radix Themes parts (TextField, Button, Badge).

**Platform features:** the search is a GET form in a `<search>` landmark
(`crate::search`: Chrome 118, Firefox 118, Safari 17); the filters are plain links, the
current one marked with `aria-current`; a container query (Chrome 105, Firefox 110,
Safari 16) puts the search and the filters on one row from 48rem.

**Accessibility:** the title is the page's `<h1>`; the filters are a `<nav>` with a name;
the count is text, read in order before the list. Checked by axe-core with the demo routes.

**What it does not do without script:** filter as you type; the search is a round trip.

**Fallback:** a browser without `<search>` shows the form all the same, without the
landmark.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/customers", "status=open", "");
let m = ui.list_page("Customers").body(html! { ul { li { "Ada" } } }).render().into_string();
assert!(m.starts_with(r#"<div class="lui-list-page"><div class="lui-list-page-head">"#));
// With a search box, filters, a count and an action:
let m = ui.list_page("Customers").description("Everyone who bought something.")
    .search("Search customers")
    .filter("All", "/customers").filter("Open", "/customers?status=open").badge(3)
    .count("3 customers")
    .actions(html! { a href="/customers/new" { "New" } })
    .body(html! { ul { li { "Ada" } } })
    .render().into_string();
assert!(m.contains(r#"<a class="lui-list-page-filter" href="/customers?status=open" aria-current="page">"#));
assert!(m.contains(r#"<input type="hidden" name="status" value="open">"#));
// The same in `lui!`:
let same = lui! { ListPage("Customers") description="Everyone who bought something."
    search="Search customers" count="3 customers" actions={ a href="/customers/new" { "New" } } {
    filter "All" "/customers"; filter "Open" "/customers?status=open" badge=3;
    body { ul { li { "Ada" } } }
} };
assert_eq!(same.into_string(), m);
```
