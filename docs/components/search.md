# Search

`loco_ui::search` · demo: `/field`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Search` | `ui.search(name: &str, label: &str)` | beta |

## Props of `Search`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `action` | value | `action: &'a str` |  | `action` | Where the search goes; this page by default. |
| `placeholder` | value | `placeholder: &'a str` |  | `placeholder` | Placeholder text. |
| `results_id` | value | `id: &'a str` |  | `aria-controls` | The id of the results it updates, for `aria-controls`. |
| `keep` | item | `name: &'a str` |  |  | A query parameter of this page carried by the search and by Clear. |
| `id` | value | `id: &'a str` |  | `id` | The field's id, `f-<name>` by default. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<search>` | 118 | 118 | 17 |
| `type="search"` | 5 | 4 | 5 |
| `aria-controls` | 1 | 1 | 1 |

- Fallback: none needed: an unknown <search> is a block holding the same GET form
- Needs script: partly: searching as you type needs script
- Responsive by: container
- Look after: Radix Themes TextField, shadcn search form

## Documentation

A search box for a page's results: a GET form in a `<search>` landmark, the query in the
URL (`?q=`), so the answer is a page that can be bookmarked, shared and reloaded. A Clear
link appears once there is a query; it is a plain link to the same page without it.

After Radix Themes TextField with a leading magnifier slot (its search example), and the
shadcn/ui search form: the field, then the Search button; Clear as a ghost link.

**Platform features:** the `<search>` element (Chrome 118, Firefox 118, Safari 17; an
unknown element before, which is still a block holding a form), `<form method="get">`
and `<input type="search">` (baseline 2015), `aria-controls` naming the results it
updates.

**Accessibility:** a `search` landmark; the field is named by its label (visually
hidden, as `aria-label`), the magnifier is decoration, Clear is a link named "Clear".
Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

**What it does not do without script:** search as you type; each search is a page load.
With the enhancement script, a search inside a swap root replaces only that root.

**Fallback:** none needed; where `<search>` is unknown it is an ordinary block and the
form inside works the same.

```rust
use loco_ui::prelude::*;
// The query is the request's `q`; the form goes back to this page.
let ui = Ui::from_request("/notes", "q=rust&view=list", "");
let m = ui.search("q", "Search notes").render().into_string();
assert!(m.starts_with(r#"<search class="lui-search"><form method="get" action="/notes">"#));
assert!(m.contains(r#"name="q" type="search" value="rust""#));
assert!(m.contains(r#"href="/notes""#), "Clear drops the query");
// The same in `lui!`:
assert_eq!(lui! { Search("q", "Search notes"); }.into_string(), m);
// Keep another parameter, name the results it updates, post elsewhere.
let m = ui.search("q", "Search notes").keep("view").results_id("notes").action("/find");
let html = m.render().into_string();
assert!(html.contains(r#"<input type="hidden" name="view" value="list">"#));
assert!(html.contains(r#"aria-controls="notes""#) && html.contains(r#"href="/find?view=list""#));
let same = lui! { Search("q", "Search notes") keep="view" results_id="notes" action="/find"; };
assert_eq!(same.into_string(), html);
```
