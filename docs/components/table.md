# Table

`loco_ui::table` · demo: `/app/notes`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Row` | `Row::new(cells: impl IntoIterator<Item = Markup>)` | stable |
| `Table` | `ui.table(id: &str, href: &str)` | stable |

## Props of `Row`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `key` | value | `key: &'a str` |  |  | The value posted for this row when its checkbox is ticked; also names its menu. |
| `detail` | value | `detail: Markup` |  |  | A block shown under the first cell when its `<details>` is opened. |
| `values` | value | `values: impl IntoIterator<Item = &'a str>` |  |  | The raw text of each cell, in column order, for editing the row in place. |
| `menu` | value | `items: impl IntoIterator<Item = MenuItem<'a>>` |  |  | Items of the row's action menu (needs a `key`). |
| `href` | value | `href: &'a str` |  |  | The whole row links to `href`: a click anywhere on it that is not a control opens it. |
| `disabled` | switch |  | `off` |  | The row cannot be selected or opened: no checkbox, no row link, muted. |
| `ok` | switch |  | `off` |  | The row tinted with the `--lui-ok` tone. |
| `warn` | switch |  | `off` |  | The row tinted with the `--lui-warn` tone. |
| `danger` | switch |  | `off` |  | The row tinted with the `--lui-danger` tone. |

## Props of `Table`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `column` | item | `key: &'a str, label: &'a str` |  |  | A column. |
| `sortable` | modifier |  |  |  | The column added last sorts the table. |
| `filterable` | modifier |  |  |  | The column added last gets a text box in the "Filters" panel, `?q.<id>.<key>=`. |
| `numeric` | modifier |  |  |  | The column added last holds numbers. |
| `editable` | modifier |  |  |  | The column added last becomes a text box when its row is edited in place (`Table::edit`). |
| `edit` | value | `action: &'a str` |  |  | Rows (with a `Row::key`) can be edited in place. |
| `width` | modifier | `width: &'a str` |  |  | A CSS width for the column added last, such as `6rem` or `30%`. |
| `priority` | modifier | `priority: u8` | `1` |  | 1 always shown, 2 hidden in a narrow table, 3 below a medium one; values then show under the first cell. |
| `rows` | value | `rows: impl IntoIterator<Item = R>` |  |  | The rows, already sorted and filtered as `Table::sort` and `Table::filter` say: `Row`s, or tuples of cells. |
| `keep` | item | `name: &'a str` |  |  | A query parameter of this page (`status`) carried, with the request's value, by every sort link, the search form, the pager and the CSV link. |
| `filter_select` | item | `name: &'a str, label: &'a str, options: impl IntoIterator<Item = (&'a str, &'a str)>` |  |  | A `<select>` of `(value, text)` options in the search form, filtering by the query parameter `name`, which the table then keeps. |
| `hide_search` | switch |  | `off` |  | No search box (a display-only table, or one filtered by `filter_select` alone). |
| `paged` | number | `total: usize` |  |  | Page the rows. |
| `paged_from` | value | `meta: &PagerMeta` |  |  | Page the rows from what Loco's `query::fetch_page` or `query::paginate` returns (feature `loco`). |
| `choose_columns` | switch |  | `off` |  | A "Columns" chooser. |
| `bulk` | value | `action: &'a str, buttons: impl IntoIterator<Item = (&'a str, &'a str)>` |  |  | A checkbox per row (rows need a `Row::key`) and a bar of `(value, label)` buttons posting to `action`. |
| `select_all` | switch |  | `off` |  | A header checkbox posting `all=1` to the bulk action, whose URL carries the query: every row the filter matches. |
| `csv` | value | `href: &'a str` |  |  | A "Download CSV" link to `href` with the current sort, filter and columns appended. |
| `json` | value | `href: &'a str` |  |  | A "Download JSON" link to `href` with the current sort, filter and columns appended. |
| `empty` | value | `message: impl Render` | `No rows match.` |  | What the body says when there are no rows: text, or markup (a call to action). |
| `loading` | condition | `loading: bool` | `off` |  | Skeleton rows with `aria-busy` instead of the rows. |
| `caption` | value | `text: &'a str` |  |  | A `<caption>` naming the table for assistive tech, visually hidden. |
| `total` | value | `key: &'a str, text: &'a str` |  |  | The cell of the column `key` in a totals row in `<tfoot>`. |
| `footer` | item | `row: impl Into<Row<'a>>` |  |  | A row in `<tfoot>`, after the totals. |
| `striped` | switch |  | `off` |  | Every other row on a gray band. |
| `density` | value | `density: Density` | `Density::Comfortable` |  | `Density::Compact` halves the cell padding. |
| `group_by` | value | `key: &'a str` |  |  | A group header row wherever the column `key`'s cell changes (rows sorted by it). |

## Keyboard of `Table`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | Sorts by the focused column header, or reverses that sort when it is already first. | no |
| `Enter` | Applies the filters from the search box or any filter field. | no |
| `Space` | Ticks or clears the focused row checkbox for the bulk actions. | no |
| `Enter / Space` | Opens or closes the Filters and Columns lists and a row's details. | no |
| `Enter` | Saves the row being edited from any of its fields. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-table` | `--lui-gray-2`, `--lui-muted`, `--lui-text-2`, `--lui-duration-fast`, `--lui-fg` |
| `.lui-table-toolbar` | `--lui-space` |
| `.lui-table-filter` | `--lui-space` |
| `.lui-table-filter-input` |  |
| `.lui-table-clear` | `--lui-muted`, `--lui-text-2`, `--lui-fg` |
| `.lui-table-cols` | `--lui-text-2`, `--lui-popover`, `--lui-line`, `--lui-radius`, `--lui-shadow-md`, `--lui-highlight`, `--lui-radius-sm`, `--lui-accent`, `--lui-on-accent` |
| `.lui-table-cols-mark` | `--lui-fg` |
| `.lui-table-scroll` | `--lui-line`, `--lui-radius-lg`, `--lui-bg`, `--lui-fg`, `--lui-gray-2` |
| `.lui-table-has-select` | `--lui-bg` |
| `.lui-table-check` | `--lui-brand-3` |
| `.lui-table-sorted` | `--lui-fg` |
| `.lui-table-arrow` |  |
| `.lui-table-num` |  |
| `.lui-table-select` |  |
| `.lui-table-menu` |  |
| `.lui-table-edit-col` |  |
| `.lui-table-edit` |  |
| `.lui-table-edit-input` |  |
| `.lui-table-empty` | `--lui-muted`, `--lui-space-6` |
| `.lui-table-more` | `--lui-space-1`, `--lui-text-2`, `--lui-muted`, `--lui-fg` |
| `.lui-table-p3` |  |
| `.lui-table-more-p3` |  |
| `.lui-table-p2` |  |
| `.lui-table-more-p2` |  |
| `.lui-table-detail` | `--lui-muted` |
| `.lui-table-detail-body` | `--lui-text-2`, `--lui-muted` |
| `.lui-table-bulk` | `--lui-space`, `--lui-text-2`, `--lui-muted` |
| `.lui-table-skeleton` | `--lui-radius-sm`, `--lui-accent` |
| `.lui-table-sort-index` | `--lui-gray-4`, `--lui-fg`, `--lui-text-1` |
| `.lui-table-then` | `--lui-radius-sm`, `--lui-muted`, `--lui-gray-4`, `--lui-fg` |
| `.lui-table-filters` | `--lui-text-2` |
| `.lui-table-filters-count` | `--lui-primary`, `--lui-on-primary`, `--lui-text-1` |
| `.lui-table-filters-body` | `--lui-space-2`, `--lui-space-3`, `--lui-gray-2`, `--lui-line`, `--lui-radius` |
| `.lui-table-all` |  |
| `.lui-table-striped` | `--lui-gray-2`, `--lui-gray-3` |
| `.lui-table-compact` | `--lui-space-1`, `--lui-space-2`, `--lui-text-2` |
| `.lui-table-group` | `--lui-gray-2`, `--lui-muted`, `--lui-text-2` |
| `.lui-table-disabled` | `--lui-muted` |
| `.lui-table-ok` | `--lui-ok`, `--lui-table-tone`, `--lui-bg` |
| `.lui-table-warn` | `--lui-warn`, `--lui-table-tone`, `--lui-bg` |
| `.lui-table-danger` | `--lui-danger`, `--lui-table-tone`, `--lui-bg` |
| `.lui-table-go-col` |  |
| `.lui-table-go` |  |
| `.lui-table-linked` |  |
| `.lui-table-row-link` | `--lui-muted`, `--lui-text-5` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `?sort.<id>=<col>` | 1 | 1 | 1 |
| `<search>` | 118 | 118 | 17 |
| `aria-sort` | 1 | 1 | 1 |
| `form attribute` | 10 | 4 | 5.1 |
| `<details>` | 12 | 49 | 6 |
| `<colgroup>` | 1 | 1 | 1 |
| `tabular-nums` | 52 | 34 | 9.1 |
| `position: sticky` | 56 | 32 | 13 |
| `view-transition-name` | 111 | 144 | 18 |
| `aria-busy` | 1 | 1 | 1 |

- Fallback: none needed: sorting, filtering, column choice and the bulk form are plain navigations and posts; select-all posts `all=1` with the query instead of ticking boxes
- Needs script: no
- Responsive by: container
- Look after: shadcn data table, Origin UI

## Documentation

A data table an admin panel can sort, filter, select from and act on, no script: every
column header is a link that re-requests the page sorted by that column (by several, in
order), a search box and per-column filters narrow the rows on the server, checkboxes
pick rows (or every matching row) for a bulk form, a "Columns" chooser hides columns
through `?cols.<id>=`, a row can expand a detail block, carry its own action menu, link
as a whole and take a tone, numbers line up, a `<tfoot>` holds totals, CSV and JSON links
download the current filter, and a row can be edited in place (`?edit.<id>=<key>` draws
it as text boxes posting to one form, Post/Redirect/Get).

**Platform features:**
- Ordinary links to `?sort.<id>=<col>` in each `<th>` (`-<col>` descending, a comma list
  for several keys: `name,-size`); clicking the first sort column again flips it, and
  once sorted, a "then by" link (`+`) beside another header adds it as the next key. The
  first key's header carries `aria-sort`; each sorted header shows its place when there
  are several.
- `<form method="get">` inside a `<search>` element (baseline 2023) for the filter; the
  sort, page size and hidden columns are kept in hidden inputs so filtering never loses
  them and the URL stays shareable. Column filters (`?q.<id>.<col>=`) sit in a
  `<details>` "Filters" panel of that form, open while one is set.
- Row selection through the form attribute (`form="id"`, Chrome 10, Firefox 4, Safari 5.1):
  each checkbox belongs to a
  bulk `<form method="post">` that sits after the table, so nothing nests and the row's
  own menu can still post. The row being edited uses the same attribute: its text boxes
  and "Save" belong to an edit form after the table, since a form cannot wrap a row.
- `<details>` (baseline 2020) in the first cell for a row's detail block; the row's menu
  is a `crate::Ui::menu`.
- `<colgroup>` widths and `font-variant-numeric: tabular-nums` (Chrome 52, Firefox 34,
  Safari 9.1) for numeric columns.
- `position: sticky` (Chrome 56, Firefox 32, Safari 13) on the header row and the first
  column (`inset-inline-start: 0`, so it is the right edge in a right-to-left page).
- A row link is a link in a last cell whose `::after` box covers the row, which is
  `position: relative` (positioned table rows: Firefox 30, Chrome 91; Safari unchecked,
  see FINDINGS).
- `view-transition-name` on the body so re-sorted rows fade rather than jump.
- `aria-busy` on a loading body, drawn as skeleton bars, for a table filled by a later
  stream chunk.

**Accessibility:** a `<table>` with `scope="col"` headers, an optional hidden
`<caption>`, `aria-sort` on the first sort key, labelled filter and page-size controls,
visually hidden names for the select, edit, actions and link columns, `aria-disabled` on
a disabled row. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** resize or reorder columns by dragging (the
"Columns" chooser and `?cols.<id>=` choose and order them), collapse a group, or keep row
selection across sorts.

**Fallback:** none needed. Without `Caps::ViewTransitions` the transition name is
omitted; sorting, filtering, choosing columns and the bulk form are plain navigations and
posts either way.

**Without script:** ticking every box at once needs script. `.select_all()` instead puts
a box in the header that posts `all=1` with the table's query in the bulk form's URL, so
the server acts on every row the filter matches; the enhancement script also ticks the
visible boxes. Everything else works the same.

**Finding:** the server sorts and filters; the component only renders what it is given and
the links to ask for something else. That is what keeps it usable with `curl`.

**Query keys:** each carries the table's id, the way its page size (`per.<id>`) and in-place
edit (`edit.<id>`) do: `q.<id>`, `q.<id>.<col>`, `sort.<id>`, `page.<id>` and `cols.<id>`
(`Keys`), so two tables on one page sort, filter and page on their own. They live in the
URL only; the page size alone is state, remembered in the `lui-ui` cookie.

```rust
use loco_ui::{prelude::*, table::Row};
// The URL says: sorted by name, filtered to "a", only two columns shown.
let ui = Ui::from_request("/table", "sort.files=name&q.files=a&cols.files=name,size", "");
// `sortable`, `numeric` and `width` apply to the column added last.
let files = ui.table("files", "/table")
    .column("name", "Name").sortable()
    .column("size", "Size").sortable().numeric().width("6rem")
    .column("note", "Note")
    .choose_columns()
    .bulk("/files/bulk", [("archive", "Archive"), ("delete", "Delete")])
    .csv("/table.csv")
    .empty("No files yet.");
// The route fetches its data with what the table read from the URL.
assert_eq!((files.sort(), files.filter()), (&[("name", false)][..], "a"));
let row = Row::new([html! { "a.txt" }, html! { "1 KB" }, html! { "—" }])
    .key("a.txt")
    .detail(html! { p { "Modified today." } })
    .menu([MenuItem::link("Open", "/files/a.txt"), MenuItem::action("Delete", "/files/a.txt/delete").danger()]);
let html = files.rows([row.clone()]).render().into_string();
assert!(html.contains("aria-sort=\"ascending\""));
assert!(html.contains("<input type=\"checkbox\" class=\"lui-table-check\" name=\"row\" value=\"a.txt\" form=\"lui-table-files-bulk\""));
assert!(html.contains("href=\"/table.csv?sort.files=name&amp;q.files=a&amp;cols.files=name%2Csize\""));
assert!(!html.contains("<td>—</td>"), "a hidden column's cells are not rendered (its name stays in the chooser)");
// The same in `lui!`:
let same = lui! { Table("files", "/table") choose_columns csv="/table.csv" empty="No files yet." {
    column "name" "Name" sortable;
    column "size" "Size" sortable numeric width="6rem";
    column "note" "Note";
    bulk "/files/bulk" ([("archive", "Archive"), ("delete", "Delete")]);
    rows ([row]);
} };
assert_eq!(same.into_string(), html);
// Priorities: 2 hides the column in a narrow table, 3 below a medium one; the values
// then show under the first cell.
let ui = Ui::default();
let t = ui.table("t", "/t").column("name", "Name").column("size", "Size").priority(2)
    .rows([(html! { "a.txt" }, html! { "1 KB" })]).render().into_string();
assert!(t.contains(r#"<td class="lui-table-p2">1 KB</td>"#) && t.contains(r#"<div class="lui-table-more-p2"><dt>Size</dt><dd>1 KB</dd>"#));
let same = lui! { Table("t", "/t") { column "name" "Name"; column "size" "Size" priority=2; rows ([(html! { "a.txt" }, html! { "1 KB" })]); } };
assert_eq!(same.into_string(), t);
```

**Fetching first, in `lui!`:** `Ui::table_query` reads the same sort, filter, page and page
size before the markup, and sorts and slices a `Vec` for you; rows can be tuples of
anything `Render`. A filter of the page's own (`?status=`) is a `.filter_select(..)` in the
search form, or a `.keep(..)` when the page draws it elsewhere: either way every sort link,
the search form and the pager carry it.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/orders", "status=paid&sort.orders=-total", "");
let orders = [(1, "Ada", "paid", 12.5_f64), (2, "Grace", "pending", 30.0), (3, "Ken", "paid", 20.0)];
let q = ui.table_query("orders", &["id", "total"]);
let status = ui.param("status").unwrap_or("");
let mut found: Vec<_> = orders.iter().filter(|o| (status.is_empty() || o.2 == status) && q.matches(o.1)).collect();
q.sort_by(&mut found, |a, b, key| match key { "total" => a.3.total_cmp(&b.3), _ => a.0.cmp(&b.0) });
let (page, total) = q.page_of(&found);
let html = lui! { Table("orders", "/orders") paged=(total) {
    column "id" "Order" sortable numeric;
    column "customer" "Customer";
    column "status" "Status";
    column "total" "Total" sortable numeric;
    filter_select "status" "Status" ([("", "All"), ("paid", "Paid"), ("pending", "Pending")]);
    rows (page.iter().map(|o| (o.0, o.1, ui.badge(o.2), format!("${:.2}", o.3))));
} }.into_string();
assert!(html.contains("<td>Ken</td>") && !html.contains("Grace"));
assert!(html.contains(r#"href="/orders?sort.orders=total&amp;status=paid&amp;"#), "kept by the sort links");
assert!(html.contains(r#"<option value="paid" selected>Paid</option>"#));
// A display-only table: no search box, no sortable column, no pager, so no link at all.
let recent = lui! { Table("recent", "") hide_search {
    column "who" "Who"; column "what" "What";
    rows ([("Ada", "upgraded to Pro"), ("Ken", "signed up")]);
} }.into_string();
assert!(!recent.contains("<a ") && !recent.contains("<form"));
```

**More of a data table:** column filters, select-all, a caption, totals and footer rows,
stripes, a compact density, group rows, row links, disabled and toned rows, JSON, and
markup when there are no rows.

```rust
use loco_ui::{prelude::*, table::Density};
let ui = Ui::from_request("/orders", "q.orders.customer=a&sort.orders=status,-total", "");
let q = ui.table_query("orders", &["status", "total"]);
let orders = [(1, "Ada", "paid", 12.5_f64), (2, "Grace", "due", 30.0), (3, "Ken", "paid", 20.0)];
let mut found: Vec<_> = orders.iter().filter(|o| q.matches_in("customer", o.1)).collect();
q.sort_by(&mut found, |a, b, key| match key { "total" => a.3.total_cmp(&b.3), _ => a.2.cmp(b.2) });
let row = |o: &&(u32, &'static str, &'static str, f64)| {
    let r = Row::from((o.0, o.1, o.2, format!("${:.2}", o.3))).href("/orders/1");
    if o.2 == "due" { r.warn() } else { r }
};
let html = ui.table("orders", "/orders")
    .column("id", "Order").numeric()
    .column("customer", "Customer").filterable()
    .column("status", "Status").sortable()
    .column("total", "Total").sortable().numeric()
    .bulk("/orders/bulk", [("archive", "Archive")]).select_all()
    .caption("Orders").total("total", "$32.50").striped().density(Density::Compact)
    .group_by("status").json("/orders.json").empty(html! { a href="/orders/new" { "Add an order" } })
    .rows(found.iter().map(row)).render().into_string();
assert!(html.contains(r#"<details class="lui-table-filters" open>"#) && html.contains(r#"name="q.orders.customer""#));
assert!(html.contains(r#"aria-label="Select all 2 matching rows""#));
assert!(html.contains(r#"<tr class="lui-table-group"><td colspan="6">paid</td></tr>"#));
assert!(html.contains("<tfoot>") && html.contains("$32.50"));
let same = lui! { Table("orders", "/orders") select_all caption="Orders" striped density=(Density::Compact)
        group_by="status" json="/orders.json" empty={ a href="/orders/new" { "Add an order" } } {
    column "id" "Order" numeric;
    column "customer" "Customer" filterable;
    column "status" "Status" sortable;
    column "total" "Total" sortable numeric;
    bulk "/orders/bulk" ([("archive", "Archive")]);
    total "total" "$32.50";
    rows (found.iter().map(row));
} };
assert_eq!(same.into_string(), html);
```
