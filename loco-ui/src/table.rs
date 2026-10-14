//! # Table
//!
//! A data table an admin panel can sort, filter, select from and act on, no script: every
//! column header is a link that re-requests the page sorted by that column (by several, in
//! order), a search box and per-column filters narrow the rows on the server, checkboxes
//! pick rows (or every matching row) for a bulk form, a "Columns" chooser hides columns
//! through `?cols.<id>=`, a row can expand a detail block, carry its own action menu, link
//! as a whole and take a tone, numbers line up, a `<tfoot>` holds totals, CSV and JSON links
//! download the current filter, and a row can be edited in place (`?edit.<id>=<key>` draws
//! it as text boxes posting to one form, Post/Redirect/Get).
//!
//! **Platform features:**
//! - Ordinary links to `?sort.<id>=<col>` in each `<th>` (`-<col>` descending, a comma list
//!   for several keys: `name,-size`); clicking the first sort column again flips it, and
//!   once sorted, a "then by" link (`+`) beside another header adds it as the next key. The
//!   first key's header carries `aria-sort`; each sorted header shows its place when there
//!   are several.
//! - `<form method="get">` inside a `<search>` element (baseline 2023) for the filter; the
//!   sort, page size and hidden columns are kept in hidden inputs so filtering never loses
//!   them and the URL stays shareable. Column filters (`?q.<id>.<col>=`) sit in a
//!   `<details>` "Filters" panel of that form, open while one is set.
//! - Row selection through the form attribute (`form="id"`, Chrome 10, Firefox 4, Safari 5.1):
//!   each checkbox belongs to a
//!   bulk `<form method="post">` that sits after the table, so nothing nests and the row's
//!   own menu can still post. The row being edited uses the same attribute: its text boxes
//!   and "Save" belong to an edit form after the table, since a form cannot wrap a row.
//! - `<details>` (baseline 2020) in the first cell for a row's detail block; the row's menu
//!   is a [`crate::Ui::menu`].
//! - `<colgroup>` widths and `font-variant-numeric: tabular-nums` (Chrome 52, Firefox 34,
//!   Safari 9.1) for numeric columns.
//! - `position: sticky` (Chrome 56, Firefox 32, Safari 13) on the header row and the first
//!   column (`inset-inline-start: 0`, so it is the right edge in a right-to-left page).
//! - A row link is a link in a last cell whose `::after` box covers the row, which is
//!   `position: relative` (positioned table rows: Firefox 30, Chrome 91; Safari unchecked,
//!   see FINDINGS).
//! - `view-transition-name` on the body so re-sorted rows fade rather than jump.
//! - `aria-busy` on a loading body, drawn as skeleton bars, for a table filled by a later
//!   stream chunk.
//!
//! **Accessibility:** a `<table>` with `scope="col"` headers, an optional hidden
//! `<caption>`, `aria-sort` on the first sort key, labelled filter and page-size controls,
//! visually hidden names for the select, edit, actions and link columns, `aria-disabled` on
//! a disabled row. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** resize or reorder columns by dragging (the
//! "Columns" chooser and `?cols.<id>=` choose and order them), collapse a group, or keep row
//! selection across sorts.
//!
//! **Fallback:** none needed. Without `Caps::ViewTransitions` the transition name is
//! omitted; sorting, filtering, choosing columns and the bulk form are plain navigations and
//! posts either way.
//!
//! **Without script:** ticking every box at once needs script. `.select_all()` instead puts
//! a box in the header that posts `all=1` with the table's query in the bulk form's URL, so
//! the server acts on every row the filter matches; the enhancement script also ticks the
//! visible boxes. Everything else works the same.
//!
//! **Finding:** the server sorts and filters; the component only renders what it is given and
//! the links to ask for something else. That is what keeps it usable with `curl`.
//!
//! **Query keys:** each carries the table's id, the way its page size (`per.<id>`) and in-place
//! edit (`edit.<id>`) do: `q.<id>`, `q.<id>.<col>`, `sort.<id>`, `page.<id>` and `cols.<id>`
//! ([`Keys`]), so two tables on one page sort, filter and page on their own. They live in the
//! URL only; the page size alone is state, remembered in the `lui-ui` cookie.
//!
//! ```rust
//! use loco_ui::{prelude::*, table::Row};
//! // The URL says: sorted by name, filtered to "a", only two columns shown.
//! let ui = Ui::from_request("/table", "sort.files=name&q.files=a&cols.files=name,size", "");
//! // `sortable`, `numeric` and `width` apply to the column added last.
//! let files = ui.table("files", "/table")
//!     .column("name", "Name").sortable()
//!     .column("size", "Size").sortable().numeric().width("6rem")
//!     .column("note", "Note")
//!     .choose_columns()
//!     .bulk("/files/bulk", [("archive", "Archive"), ("delete", "Delete")])
//!     .csv("/table.csv")
//!     .empty("No files yet.");
//! // The route fetches its data with what the table read from the URL.
//! assert_eq!((files.sort(), files.filter()), (&[("name", false)][..], "a"));
//! let row = Row::new([html! { "a.txt" }, html! { "1 KB" }, html! { "—" }])
//!     .key("a.txt")
//!     .detail(html! { p { "Modified today." } })
//!     .menu([MenuItem::link("Open", "/files/a.txt"), MenuItem::action("Delete", "/files/a.txt/delete").danger()]);
//! let html = files.rows([row.clone()]).render().into_string();
//! assert!(html.contains("aria-sort=\"ascending\""));
//! assert!(html.contains("<input type=\"checkbox\" class=\"lui-table-check\" name=\"row\" value=\"a.txt\" form=\"lui-table-files-bulk\""));
//! assert!(html.contains("href=\"/table.csv?sort.files=name&amp;q.files=a&amp;cols.files=name%2Csize\""));
//! assert!(!html.contains("<td>—</td>"), "a hidden column's cells are not rendered (its name stays in the chooser)");
//! // The same in `lui!`:
//! let same = lui! { Table("files", "/table") choose_columns csv="/table.csv" empty="No files yet." {
//!     column "name" "Name" sortable;
//!     column "size" "Size" sortable numeric width="6rem";
//!     column "note" "Note";
//!     bulk "/files/bulk" ([("archive", "Archive"), ("delete", "Delete")]);
//!     rows ([row]);
//! } };
//! assert_eq!(same.into_string(), html);
//! // Priorities: 2 hides the column in a narrow table, 3 below a medium one; the values
//! // then show under the first cell.
//! let ui = Ui::default();
//! let t = ui.table("t", "/t").column("name", "Name").column("size", "Size").priority(2)
//!     .rows([(html! { "a.txt" }, html! { "1 KB" })]).render().into_string();
//! assert!(t.contains(r#"<td class="lui-table-p2">1 KB</td>"#) && t.contains(r#"<div class="lui-table-more-p2"><dt>Size</dt><dd>1 KB</dd>"#));
//! let same = lui! { Table("t", "/t") { column "name" "Name"; column "size" "Size" priority=2; rows ([(html! { "a.txt" }, html! { "1 KB" })]); } };
//! assert_eq!(same.into_string(), t);
//! ```
//!
//! **Fetching first, in `lui!`:** [`Ui::table_query`] reads the same sort, filter, page and page
//! size before the markup, and sorts and slices a `Vec` for you; rows can be tuples of
//! anything `Render`. A filter of the page's own (`?status=`) is a `.filter_select(..)` in the
//! search form, or a `.keep(..)` when the page draws it elsewhere: either way every sort link,
//! the search form and the pager carry it.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/orders", "status=paid&sort.orders=-total", "");
//! let orders = [(1, "Ada", "paid", 12.5_f64), (2, "Grace", "pending", 30.0), (3, "Ken", "paid", 20.0)];
//! let q = ui.table_query("orders", &["id", "total"]);
//! let status = ui.param("status").unwrap_or("");
//! let mut found: Vec<_> = orders.iter().filter(|o| (status.is_empty() || o.2 == status) && q.matches(o.1)).collect();
//! q.sort_by(&mut found, |a, b, key| match key { "total" => a.3.total_cmp(&b.3), _ => a.0.cmp(&b.0) });
//! let (page, total) = q.page_of(&found);
//! let html = lui! { Table("orders", "/orders") paged=(total) {
//!     column "id" "Order" sortable numeric;
//!     column "customer" "Customer";
//!     column "status" "Status";
//!     column "total" "Total" sortable numeric;
//!     filter_select "status" "Status" ([("", "All"), ("paid", "Paid"), ("pending", "Pending")]);
//!     rows (page.iter().map(|o| (o.0, o.1, ui.badge(o.2), format!("${:.2}", o.3))));
//! } }.into_string();
//! assert!(html.contains("<td>Ken</td>") && !html.contains("Grace"));
//! assert!(html.contains(r#"href="/orders?sort.orders=total&amp;status=paid&amp;"#), "kept by the sort links");
//! assert!(html.contains(r#"<option value="paid" selected>Paid</option>"#));
//! // A display-only table: no search box, no sortable column, no pager, so no link at all.
//! let recent = lui! { Table("recent", "") hide_search {
//!     column "who" "Who"; column "what" "What";
//!     rows ([("Ada", "upgraded to Pro"), ("Ken", "signed up")]);
//! } }.into_string();
//! assert!(!recent.contains("<a ") && !recent.contains("<form"));
//! ```
//!
//! **More of a data table:** column filters, select-all, a caption, totals and footer rows,
//! stripes, a compact density, group rows, row links, disabled and toned rows, JSON, and
//! markup when there are no rows.
//!
//! ```rust
//! use loco_ui::{prelude::*, table::Density};
//! let ui = Ui::from_request("/orders", "q.orders.customer=a&sort.orders=status,-total", "");
//! let q = ui.table_query("orders", &["status", "total"]);
//! let orders = [(1, "Ada", "paid", 12.5_f64), (2, "Grace", "due", 30.0), (3, "Ken", "paid", 20.0)];
//! let mut found: Vec<_> = orders.iter().filter(|o| q.matches_in("customer", o.1)).collect();
//! q.sort_by(&mut found, |a, b, key| match key { "total" => a.3.total_cmp(&b.3), _ => a.2.cmp(b.2) });
//! let row = |o: &&(u32, &'static str, &'static str, f64)| {
//!     let r = Row::from((o.0, o.1, o.2, format!("${:.2}", o.3))).href("/orders/1");
//!     if o.2 == "due" { r.warn() } else { r }
//! };
//! let html = ui.table("orders", "/orders")
//!     .column("id", "Order").numeric()
//!     .column("customer", "Customer").filterable()
//!     .column("status", "Status").sortable()
//!     .column("total", "Total").sortable().numeric()
//!     .bulk("/orders/bulk", [("archive", "Archive")]).select_all()
//!     .caption("Orders").total("total", "$32.50").striped().density(Density::Compact)
//!     .group_by("status").json("/orders.json").empty(html! { a href="/orders/new" { "Add an order" } })
//!     .rows(found.iter().map(row)).render().into_string();
//! assert!(html.contains(r#"<details class="lui-table-filters" open>"#) && html.contains(r#"name="q.orders.customer""#));
//! assert!(html.contains(r#"aria-label="Select all 2 matching rows""#));
//! assert!(html.contains(r#"<tr class="lui-table-group"><td colspan="6">paid</td></tr>"#));
//! assert!(html.contains("<tfoot>") && html.contains("$32.50"));
//! let same = lui! { Table("orders", "/orders") select_all caption="Orders" striped density=(Density::Compact)
//!         group_by="status" json="/orders.json" empty={ a href="/orders/new" { "Add an order" } } {
//!     column "id" "Order" numeric;
//!     column "customer" "Customer" filterable;
//!     column "status" "Status" sortable;
//!     column "total" "Total" sortable numeric;
//!     bulk "/orders/bulk" ([("archive", "Archive")]);
//!     total "total" "$32.50";
//!     rows (found.iter().map(row));
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

#[cfg(feature = "loco")]
use loco_rs::controller::views::pagination::PagerMeta;
use maud::{Markup, Render, html};

use crate::i18n::{Strings, Text};
use crate::popover::MenuItem;
use crate::props::{Prop, PropKind};
use crate::state::decode;
use crate::{Caps, Ui, enhance};

mod body;
mod foot;
mod head;
mod toolbar;

/// One column: the query key it sorts by, its header text, whether it can be sorted, how
/// its cells align and how wide it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Column<'a> {
    /// Its name in `?sort.<id>=`, `?cols.<id>=` and `?q.<id>.<key>=`.
    pub key: &'a str,
    /// Header text.
    pub label: &'a str,
    /// Whether the header is a sort link.
    pub sortable: bool,
    /// Whether it has a text box in the "Filters" panel.
    pub filterable: bool,
    /// Right-aligned tabular figures.
    pub numeric: bool,
    /// A CSS width for the `<col>`, such as `6rem` or `30%`.
    pub width: Option<&'a str>,
    /// Becomes a text box when its row is edited in place.
    pub editable: bool,
    /// 1 always shown, 2 hidden in a narrow table, 3 hidden below a medium one.
    pub priority: u8,
}

impl<'a> Column<'a> {
    /// A column with a plain header.
    pub const fn plain(key: &'a str, label: &'a str) -> Column<'a> {
        Column {
            key,
            label,
            sortable: false,
            filterable: false,
            numeric: false,
            width: None,
            editable: false,
            priority: 1,
        }
    }
}

/// One row: its cells, and optionally a key (for selection and its menu id), a detail
/// block opened from the first cell, an action menu in a last column, a link for the whole
/// row and a tone.
///
/// **Setters.** Values and items: `.key(..)`, `.detail(..)`, `.values(..)`, `.menu(..)`,
/// `.href(..)`; switches: `.disabled()`, `.ok()`, `.warn()`, `.danger()`.
#[derive(Clone, Debug)]
pub struct Row<'a> {
    cells: Vec<Markup>,
    key: Option<&'a str>,
    detail: Option<Markup>,
    menu: Vec<MenuItem<'a>>,
    values: Vec<&'a str>,
    href: Option<&'a str>,
    disabled: bool,
    tone: Option<&'static str>,
}

impl Row<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("key", PropKind::Value, "key: &'a str")
            .doc("The value posted for this row when its checkbox is ticked; also names its menu."),
        Prop::new("detail", PropKind::Value, "detail: Markup")
            .doc("A block shown under the first cell when its `<details>` is opened."),
        Prop::new(
            "values",
            PropKind::Value,
            "values: impl IntoIterator<Item = &'a str>",
        )
        .doc("The raw text of each cell, in column order, for editing the row in place."),
        Prop::new(
            "menu",
            PropKind::Value,
            "items: impl IntoIterator<Item = MenuItem<'a>>",
        )
        .doc("Items of the row's action menu (needs a `key`)."),
        Prop::new("href", PropKind::Value, "href: &'a str").doc(
            "The whole row links to `href`: a click anywhere on it that is not a control opens it.",
        ),
        Prop::new("disabled", PropKind::Switch, "")
            .doc("The row cannot be selected or opened: no checkbox, no row link, muted."),
        Prop::new("ok", PropKind::Switch, "").doc("The row tinted with the `--lui-ok` tone."),
        Prop::new("warn", PropKind::Switch, "").doc("The row tinted with the `--lui-warn` tone."),
        Prop::new("danger", PropKind::Switch, "")
            .doc("The row tinted with the `--lui-danger` tone."),
    ];
}

impl<'a> Row<'a> {
    /// A row of cells, one per column (a hidden column's cell is skipped).
    pub fn new(cells: impl IntoIterator<Item = Markup>) -> Self {
        Row {
            cells: cells.into_iter().collect(),
            key: None,
            detail: None,
            menu: Vec::new(),
            values: Vec::new(),
            href: None,
            disabled: false,
            tone: None,
        }
    }
    /// The value posted for this row when its checkbox is ticked; also names its menu.
    pub const fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }
    /// A block shown under the first cell when its `<details>` is opened.
    pub fn detail(mut self, detail: Markup) -> Self {
        self.detail = Some(detail);
        self
    }
    /// The raw text of each cell, in column order, for editing the row in place: an editable
    /// column's text box starts with its value (the cells may be formatted, `1 KB`).
    pub fn values(mut self, values: impl IntoIterator<Item = &'a str>) -> Self {
        self.values = values.into_iter().collect();
        self
    }
    /// Items of the row's action menu (needs a `key`).
    pub fn menu(mut self, items: impl IntoIterator<Item = MenuItem<'a>>) -> Self {
        self.menu = items.into_iter().collect();
        self
    }
    /// The whole row links to `href`, as Tremor's linked rows do: a link in a last column
    /// whose box is stretched over the row, so a click anywhere on it that is not a control
    /// (checkbox, detail, menu, a link in a cell) opens it, and the keyboard reaches it as
    /// one link named after the row's key.
    pub const fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }
    /// The row cannot be selected or opened: no checkbox, no row link, `aria-disabled`, muted.
    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    /// The row tinted with the `--lui-ok` tone (a paid order, a passing check).
    pub const fn ok(mut self) -> Self {
        self.tone = Some("lui-table-ok");
        self
    }
    /// The row tinted with the `--lui-warn` tone (overdue, nearly full).
    pub const fn warn(mut self) -> Self {
        self.tone = Some("lui-table-warn");
        self
    }
    /// The row tinted with the `--lui-danger` tone (failed, refused).
    pub const fn danger(mut self) -> Self {
        self.tone = Some("lui-table-danger");
        self
    }
}

/// A row from its cells: `vec![html! { "a.txt" }, html! { "1 KB" }].into()`.
impl From<Vec<Markup>> for Row<'_> {
    fn from(cells: Vec<Markup>) -> Self {
        Row::new(cells)
    }
}

// A row from a tuple of cells, anything `Render` (text, numbers, markup, a badge):
// `(o.id, o.customer, ui.badge(o.status)).into()`, or the tuples themselves in `.rows(..)`.
impl<A: Render, B: Render> From<(A, B)> for Row<'_> {
    fn from((a, b): (A, B)) -> Self {
        Row::new([a.render(), b.render()])
    }
}

impl<A: Render, B: Render, C: Render> From<(A, B, C)> for Row<'_> {
    fn from((a, b, c): (A, B, C)) -> Self {
        Row::new([a.render(), b.render(), c.render()])
    }
}

impl<A: Render, B: Render, C: Render, D: Render> From<(A, B, C, D)> for Row<'_> {
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        Row::new([a.render(), b.render(), c.render(), d.render()])
    }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render> From<(A, B, C, D, E)> for Row<'_> {
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        Row::new([a.render(), b.render(), c.render(), d.render(), e.render()])
    }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render, F: Render> From<(A, B, C, D, E, F)>
    for Row<'_>
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        Row::new([
            a.render(),
            b.render(),
            c.render(),
            d.render(),
            e.render(),
            f.render(),
        ])
    }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render, F: Render, G: Render>
    From<(A, B, C, D, E, F, G)> for Row<'_>
{
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        Row::new([
            a.render(),
            b.render(),
            c.render(),
            d.render(),
            e.render(),
            f.render(),
            g.render(),
        ])
    }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render, F: Render, G: Render, H: Render>
    From<(A, B, C, D, E, F, G, H)> for Row<'_>
{
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        Row::new([
            a.render(),
            b.render(),
            c.render(),
            d.render(),
            e.render(),
            f.render(),
            g.render(),
            h.render(),
        ])
    }
}

/// The query keys of the table `id`: `q.<id>`, `sort.<id>`, `page.<id>` and `cols.<id>`,
/// named like its page size (`per.<id>`) and its in-place edit (`edit.<id>`), so two tables
/// on one page sort, filter and page on their own; a column's filter is
/// [`Keys::filter`], `q.<id>.<column>`. An empty id gives the bare names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keys {
    /// The search text.
    pub q: String,
    /// The sort: a comma list of column keys, each `-` first when descending.
    pub sort: String,
    /// The page, 1-based.
    pub page: String,
    /// The visible columns, a comma list.
    pub cols: String,
}

impl Keys {
    /// The keys of the table `id`.
    ///
    /// ```rust
    /// let keys = loco_ui::table::Keys::new("files");
    /// assert_eq!((keys.q.as_str(), keys.sort.as_str()), ("q.files", "sort.files"));
    /// assert_eq!(format!("/table?{}=-size,name&{}=src", keys.sort, keys.filter("name")), "/table?sort.files=-size,name&q.files.name=src");
    /// ```
    pub fn new(id: &str) -> Keys {
        let key = |name: &str| {
            if id.is_empty() {
                name.to_string()
            } else {
                format!("{name}.{id}")
            }
        };
        Keys {
            q: key("q"),
            sort: key("sort"),
            page: key("page"),
            cols: key("cols"),
        }
    }

    /// The key of the filter of the column `column`: `q.<id>.<column>`.
    pub fn filter(&self, column: &str) -> String {
        format!("{}.{column}", self.q)
    }
}

/// `name,-size`: the sort as a form field holds it (the browser encodes it).
pub(crate) fn sort_text(sort: &[(&str, bool)]) -> String {
    let keys: Vec<String> = (sort.iter())
        .map(|(k, desc)| format!("{}{k}", if *desc { "-" } else { "" }))
        .collect();
    keys.join(",")
}

/// `name,-size`: the sort as `?sort.<id>=` writes it, each key encoded, the commas kept.
pub(crate) struct SortValue<'a>(pub(crate) &'a [(&'a str, bool)]);

impl std::fmt::Display for SortValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (key, desc)) in self.0.iter().enumerate() {
            let sep = if i == 0 { "" } else { "," };
            let dash = if *desc { "-" } else { "" };
            write!(f, "{sep}{dash}{}", Encoded(key))?;
        }
        Ok(())
    }
}

/// A table's URL parameters (`?sort.<id>=&q.<id>=&q.<id>.<column>=&page.<id>=&cols.<id>=`,
/// see [`Keys`]) and its page size, read before the markup: made by [`Ui::table_query`], so
/// a route can fetch, sort and slice its rows and then write the table in `lui!` without
/// holding the builder in a variable. The [`Table`] with the same id reads the same
/// parameters, so the two agree.
///
/// ```rust
/// use loco_ui::prelude::*;
/// let ui = Ui::from_request("/orders", "sort.orders=-total,id&q.orders=ADA&page.orders=2", "lui-ui=per.orders=5");
/// let q = ui.table_query("orders", &["id", "total"]);
/// assert_eq!((q.sort(), q.filter(), q.page(), q.per_page()), (&[("total", true), ("id", false)][..], "ADA", 2, 5));
/// let mut orders: Vec<(u32, &str, f64)> = (1..=20).map(|i| (i, "Ada", (i % 4) as f64)).collect();
/// orders.retain(|o| q.matches(o.1)); // case-insensitive, and true when there is no filter
/// // Compared by each key in turn: total descending, then id.
/// q.sort_by(&mut orders, |a, b, key| match key {
///     "total" => a.2.total_cmp(&b.2),
///     _ => a.0.cmp(&b.0),
/// });
/// let (page, total) = q.page_of(&orders);
/// assert_eq!((page[0].0, page.len(), total), (2, 5, 20));
/// // A column's own filter, `q.<id>.<column>`:
/// let ui = Ui::from_request("/orders", "q.orders.status=PAID&sort.orders=secret", "");
/// let q = ui.table_query("orders", &["id"]);
/// assert_eq!((q.filters(), q.matches_in("status", "paid"), q.matches_in("status", "due")), (&[("status", "PAID")][..], true, false));
/// // Defaults: no sort, no filter, page 1 of 10 rows.
/// assert_eq!((q.sort(), q.filter(), q.page(), q.per_page()), (&[][..], "", 1, 10));
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TableQuery<'a> {
    /// `?sort.<id>=a,-b` as `(key, descending)`, each key once, in order; checked against
    /// the sortable keys (or the table's columns).
    pub(crate) sort: Vec<(&'a str, bool)>,
    /// `?q.<id>=`, the search text as typed, trimmed.
    pub(crate) filter: &'a str,
    /// The search text lowercased, for [`TableQuery::matches`].
    lower: String,
    /// `?q.<id>.<column>=` as `(column, text)`, trimmed, the empty ones left out.
    pub(crate) filters: Vec<(&'a str, &'a str)>,
    /// `?page.<id>=`, 1-based.
    pub(crate) page: Option<usize>,
    /// `?cols.<id>=a,b`, checked against the columns by [`TableQuery::cols_in`].
    pub(crate) cols: Option<&'a str>,
    /// Rows per page: `per.<id>` from the `lui-ui` cookie or the URL, else 10, at most 50.
    per_page: usize,
}

impl Ui {
    /// The query of the table `id` ([`TableQuery`]): its sort (only by keys in `sortable`),
    /// search text, column filters, page and page size, for fetching the rows before writing
    /// the table. Defaults: unsorted, no filter, page 1, 10 rows per page, or the size the
    /// visitor picked (`ui.state.per_page(id)`, capped at 50).
    pub fn table_query<'a>(&'a self, id: &str, sortable: &[&str]) -> TableQuery<'a> {
        let mut q = TableQuery::from_ui(self, id);
        q.sort.retain(|(k, _)| sortable.contains(k));
        q
    }
}

/// `name,-size,name` as `[("name", false), ("size", true)]`: each key once, the first time.
fn parse_sort(value: &str) -> Vec<(&str, bool)> {
    let mut sort: Vec<(&str, bool)> = Vec::new();
    for part in value.split(',').map(str::trim) {
        let (key, desc) = match part.strip_prefix('-') {
            Some(key) => (key, true),
            None => (part, false),
        };
        if !key.is_empty() && !sort.iter().any(|(k, _)| *k == key) {
            sort.push((key, desc));
        }
    }
    sort
}

impl<'a> TableQuery<'a> {
    /// Read the table's parameters from the request, the sort keys unchecked.
    pub(crate) fn from_ui(ui: &'a Ui, id: &str) -> TableQuery<'a> {
        let keys = Keys::new(id);
        let filter = ui.param(&keys.q).unwrap_or("").trim();
        let prefix = format!("{}.", keys.q);
        let mut filters: Vec<(&str, &str)> = Vec::new();
        for (k, v) in ui.query_pairs() {
            if let Some(column) = k.strip_prefix(&prefix)
                && !v.trim().is_empty()
                && !filters.iter().any(|(c, _)| *c == column)
            {
                filters.push((column, v.trim()));
            }
        }
        let sizes = crate::paged_table::PAGE_SIZES;
        TableQuery {
            sort: ui.param(&keys.sort).map(parse_sort).unwrap_or_default(),
            filter,
            lower: filter.to_lowercase(),
            filters,
            page: ui
                .param(&keys.page)
                .and_then(|v| v.parse().ok())
                .filter(|&n| n > 0),
            cols: ui.param(&keys.cols),
            per_page: ui
                .state
                .per_page(id)
                .map_or(sizes[1], |n| n.min(sizes[sizes.len() - 1]))
                .max(1),
        }
    }

    /// The requested sort as `(column key, descending)`, most significant first; empty when
    /// unsorted.
    pub fn sort(&self) -> &[(&'a str, bool)] {
        &self.sort
    }

    /// The requested search text, trimmed, as typed.
    pub fn filter(&self) -> &'a str {
        self.filter
    }

    /// The requested column filters as `(column key, text)`, trimmed, as typed.
    pub fn filters(&self) -> &[(&'a str, &'a str)] {
        &self.filters
    }

    /// Whether `text` contains the search text, ignoring case; always true with no search.
    pub fn matches(&self, text: &str) -> bool {
        self.lower.is_empty() || text.to_lowercase().contains(&self.lower)
    }

    /// Whether `text` contains the filter of the column `key`, ignoring case; always true
    /// when that column has none.
    pub fn matches_in(&self, key: &str, text: &str) -> bool {
        self.filters
            .iter()
            .find(|(k, _)| *k == key)
            .is_none_or(|(_, f)| text.to_lowercase().contains(&f.to_lowercase()))
    }

    /// The requested page, 1-based (not yet clamped to the rows there are: see
    /// [`TableQuery::page_of`]).
    pub fn page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    /// Rows per page: the visitor's remembered choice (`per.<id>`), or 10.
    pub fn per_page(&self) -> usize {
        self.per_page
    }

    /// Sort `rows` as asked, when asked: `cmp(a, b, key)` compares two rows by the column
    /// `key` in ascending order; each sort key is tried in turn until one tells the rows
    /// apart, and a descending key reverses its order. Stable.
    pub fn sort_by<T>(&self, rows: &mut [T], cmp: impl Fn(&T, &T, &str) -> std::cmp::Ordering) {
        if self.sort.is_empty() {
            return;
        }
        rows.sort_by(|a, b| {
            self.sort
                .iter()
                .map(|(key, desc)| {
                    let order = cmp(a, b, key);
                    if *desc { order.reverse() } else { order }
                })
                .find(|o| o.is_ne())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// The current page of `rows` (every row, filtered and sorted) and their count, the
    /// `total` for [`Table::paged`]. A page past the end gives the last one, as the pager does.
    pub fn page_of<'r, T>(&self, rows: &'r [T]) -> (&'r [T], usize) {
        let total = rows.len();
        let pages = total.div_ceil(self.per_page).max(1);
        let start = (self.page().clamp(1, pages) - 1) * self.per_page;
        let page = rows
            .get(start.min(total)..(start + self.per_page).min(total))
            .unwrap_or(&[]);
        (page, total)
    }

    /// The keys of the columns `?cols.<id>=` shows, in table order; every key when it names none.
    pub fn visible<'k>(&self, keys: &[&'k str]) -> Vec<&'k str> {
        let shown: Vec<&'k str> = keys
            .iter()
            .copied()
            .filter(|k| self.cols.is_some_and(|c| c.split(',').any(|w| w == *k)))
            .collect();
        if shown.is_empty() {
            keys.to_vec()
        } else {
            shown
        }
    }

    /// The visible column keys, only those `columns` has; `None` means every column.
    pub(crate) fn cols_in<'c>(&self, columns: &[Column<'c>]) -> Option<Vec<&'c str>> {
        cols_from_query(columns, self.cols)
    }
}

/// Parse `?cols.<id>=a,b` into the visible keys, keeping only keys the table has and only when at
/// least one is left; `None` means every column.
pub(crate) fn cols_from_query<'a>(
    columns: &[Column<'a>],
    cols: Option<&str>,
) -> Option<Vec<&'a str>> {
    let wanted = cols?;
    let keys: Vec<&str> = columns
        .iter()
        .filter(|c| wanted.split(',').any(|w| w == c.key))
        .map(|c| c.key)
        .collect();
    (!keys.is_empty()).then_some(keys)
}

/// How much room a table's cells get: [`Density::Comfortable`] (the default) or
/// [`Density::Compact`], whose cells have half the padding and smaller text, for a dense
/// admin list. The names follow the page-wide density M39 plans; a table's own setting wins.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Density {
    /// Half the cell padding (`--lui-space-1` by `--lui-space-2`) and 0.8125rem text.
    Compact,
    /// The default: `0.5rem` cell padding, 0.875rem text.
    #[default]
    Comfortable,
}

/// What one render of a table needs, worked out by [`Table`] from its setters and the
/// request; a paged table draws the same view with the page's rows and its page size kept.
#[derive(Clone, Debug)]
pub(crate) struct View<'v> {
    pub caps: Caps,
    pub id: &'v str,
    /// The links' and forms' target, its own query moved into `keep`.
    pub href: &'v str,
    pub columns: &'v [Column<'v>],
    pub rows: &'v [Row<'v>],
    /// The current sort as `(key, descending)`, most significant first, sortable keys only.
    pub sort: &'v [(&'v str, bool)],
    /// The current search text, echoed into the box and kept in the sort links.
    pub filter: &'v str,
    /// The current column filters as `(key, text)`, filterable columns only.
    pub filters: &'v [(&'v str, &'v str)],
    /// Extra query pairs (a page size, say) carried by every link and the filter form.
    pub keep: Vec<(&'v str, &'v str)>,
    /// Visible column keys; `None` shows all.
    pub cols: Option<&'v [&'v str]>,
    /// Show the "Columns" chooser; also shown whenever `cols` is set, so a hidden column can
    /// always be brought back.
    pub choose_columns: bool,
    /// A bulk post `action` and its buttons `(value, label)`.
    pub bulk: Option<(&'v str, &'v [(&'v str, &'v str)])>,
    /// A header checkbox posting `all=1`: every row the filter matches.
    pub select_all: bool,
    /// How many rows the filter matches: the paged total, or the rows given.
    pub matching: usize,
    /// Base URLs of the CSV and JSON downloads; the current sort, filters and columns are
    /// appended.
    pub csv: Option<&'v str>,
    pub json: Option<&'v str>,
    /// What the empty body says.
    pub empty: &'v Markup,
    /// Draw skeleton rows with `aria-busy` instead of `rows`: the data is still coming.
    pub loading: bool,
    /// Rows can be edited in place.
    pub edit: Option<Editing<'v>>,
    /// The visitor's language, for the table's own words.
    pub strings: &'static Strings,
    /// The search box.
    pub search: bool,
    /// `<select>`s in the search form, each filtering by one query parameter.
    pub selects: &'v [FilterSelect<'v>],
    /// A visually hidden `<caption>`.
    pub caption: Option<&'v str>,
    /// `(column key, text)` cells of a totals row in `<tfoot>`.
    pub totals: &'v [(&'v str, &'v str)],
    /// More `<tfoot>` rows.
    pub footer: &'v [Row<'v>],
    pub striped: bool,
    pub density: Density,
    /// The index of the column whose value heads a group of rows.
    pub group_by: Option<usize>,
}

impl<'v> View<'v> {
    /// Every pair a link or form of this table carries besides the sort: the search text,
    /// the column filters (`filter_keys` names them, in `filters` order), the kept
    /// parameters and the visible columns.
    pub(crate) fn pairs<'s>(
        &'s self,
        keys: &'s Keys,
        filter_keys: &'s [String],
        cols: Option<&'s str>,
    ) -> Vec<(&'s str, &'s str)> {
        let mut pairs: Vec<(&'s str, &'s str)> = Vec::with_capacity(4 + self.keep.len());
        if !self.filter.is_empty() {
            pairs.push((&keys.q, self.filter));
        }
        for (key, (_, text)) in filter_keys.iter().zip(self.filters) {
            pairs.push((key, text));
        }
        pairs.extend(self.keep.iter().copied());
        if let Some(c) = cols {
            pairs.push((&keys.cols, c));
        }
        pairs
    }

    /// The keys of the column filters, `q.<id>.<column>`, in `filters` order.
    pub(crate) fn filter_keys(&self, keys: &Keys) -> Vec<String> {
        self.filters.iter().map(|(c, _)| keys.filter(c)).collect()
    }
}

/// A `<select>` in the table's search form: its parameter, label, `(value, text)` options and
/// the value the request chose.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FilterSelect<'a> {
    name: &'a str,
    label: &'a str,
    options: Vec<(&'a str, &'a str)>,
    value: &'a str,
}

/// `href` followed by `query` (empty, or starting with `?`): joined with `&` when `href` has
/// a query of its own, and `?` alone when both are empty, so a link to "this page, no
/// parameters" never becomes `href=""` (which keeps the current ones).
fn join(href: &str, query: &str) -> String {
    match query.strip_prefix('?') {
        None if href.is_empty() => "?".to_string(),
        None => href.to_string(),
        Some(rest) if href.contains('?') => format!("{href}&{rest}"),
        Some(_) => format!("{href}{query}"),
    }
}

/// The in-place edit of a table's rows, worked out by [`Table`] from the request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Editing<'a> {
    /// Where the edit form posts: `key=<row key>`, one field per editable column, `returns_to`.
    pub action: &'a str,
    /// The key of the row being edited, from `?edit.<id>=`.
    pub key: Option<&'a str>,
    /// This page's URL ending in `edit.<id>=`: a row's key completes its "Edit" link.
    pub link: &'a str,
    /// This page's URL without `edit.<id>`: "Cancel", and where the route returns.
    pub done: &'a str,
}

/// A [`View`] with what its parts share worked out once: the query keys, the element ids,
/// the visible columns and the pairs every link carries.
struct Ctx<'c, 'v> {
    v: &'c View<'v>,
    keys: &'c Keys,
    root: &'c str,
    edit_id: &'c str,
    bulk_id: &'c str,
    /// `(index in the row's cells, column)` of each shown column.
    visible: &'c [(usize, &'v Column<'v>)],
    /// Query pairs every link and form carries besides the sort: filters, extras, columns.
    carried: &'c [(&'c str, &'c str)],
    has_menu: bool,
    /// Some row links somewhere: a last column holds the links.
    has_link: bool,
    /// Cells in a full-width row: the shown columns and the select, edit, menu and link
    /// columns.
    span: usize,
}

impl Ctx<'_, '_> {
    fn t(&self, text: Text) -> &'static str {
        self.v.strings.get(text)
    }

    fn shown(&self, c: &Column) -> bool {
        self.v.cols.is_none_or(|v| v.contains(&c.key))
    }

    /// `?sort.<id>=..&pairs..`, or nothing when there is neither.
    fn query(&self, sort: &[(&str, bool)], pairs: &[(&str, &str)]) -> String {
        use std::fmt::Write;
        let mut out = String::new();
        if !sort.is_empty() {
            let _ = write!(out, "?{}={}", Encoded(&self.keys.sort), SortValue(sort));
        }
        for (k, v) in pairs {
            let sep = if out.is_empty() { '?' } else { '&' };
            let _ = write!(out, "{sep}{}={}", Encoded(k), Encoded(v));
        }
        out
    }

    /// `href` with `sort` and `pairs`.
    fn link(&self, sort: &[(&str, bool)], pairs: &[(&str, &str)]) -> String {
        join(self.v.href, &self.query(sort, pairs))
    }
}

impl View<'_> {
    /// The table, as a swap root or not: inside a paged table the pager's root is the swap
    /// root, so a sort also refreshes the page links.
    pub(crate) fn render(&self, swap: bool) -> Markup {
        let keys = Keys::new(self.id);
        let root = enhance::swap_id("lui-table", self.id);
        let edit_id = format!("{root}-edit");
        let bulk_id = format!("{root}-bulk");
        let visible: Vec<(usize, &Column)> = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| self.cols.is_none_or(|v| v.contains(&c.key)))
            .collect();
        let cols_value = self.cols.map(|v| v.join(","));
        let filter_keys = self.filter_keys(&keys);
        let carried = self.pairs(&keys, &filter_keys, cols_value.as_deref());
        let has_menu = self.rows.iter().any(|r| !r.menu.is_empty());
        let has_link = self.rows.iter().any(|r| r.href.is_some());
        let span = visible.len()
            + usize::from(self.bulk.is_some())
            + usize::from(has_menu)
            + usize::from(has_link)
            + usize::from(self.edit.is_some());
        let cx = Ctx {
            v: self,
            keys: &keys,
            root: &root,
            edit_id: &edit_id,
            bulk_id: &bulk_id,
            visible: &visible,
            carried: &carried,
            has_menu,
            has_link,
            span,
        };
        html! {
            div id=(root) data-lui=[swap.then_some("swap")] class={ "lui-table" @if self.striped { " lui-table-striped" } @if self.density == Density::Compact { " lui-table-compact" } } {
                (toolbar::toolbar(&cx))
                div class={ "lui-table-scroll" @if self.bulk.is_some() { " lui-table-has-select" } } {
                    table {
                        @if let Some(caption) = self.caption { caption class="lui-sr" { (caption) } }
                        (head::head(&cx))
                        (body::body(&cx))
                        (foot::tfoot(&cx))
                    }
                }
                (foot::forms(&cx))
            }
        }
    }
}

/// A data table, made by [`Ui::table`]. It reads its sort, filters, page and visible columns
/// from the request (`?sort.<id>=&q.<id>=&q.<id>.<column>=&page.<id>=&cols.<id>=`, with
/// `<id>` its id), so a route asks it how to fetch the rows ([`Table::sort`],
/// [`Table::filter`], [`Table::filters`]) and hands them over with [`Table::rows`].
///
/// To fetch the rows before writing the table (in `lui!`, say), read the same parameters
/// with [`Ui::table_query`].
///
/// **Setters.** Values and items: `.bulk(..)`, `.column(..)`, `.edit(..)`, `.width(..)`, `.priority(..)`,
/// `.rows(..)`, `.paged(..)`, `.paged_from(..)`, `.csv(..)`, `.json(..)`, `.empty(..)`, `.keep(..)`,
/// `.filter_select(..)`, `.caption(..)`, `.total(..)`, `.footer(..)`, `.density(..)`,
/// `.group_by(..)`; switches: `.sortable()`, `.filterable()`, `.numeric()`, `.editable()`,
/// `.choose_columns()`, `.hide_search()`, `.select_all()`, `.striped()`; from a condition:
/// `.loading(bool)`.
#[derive(Clone, Debug)]
pub struct Table<'a> {
    ui: &'a Ui,
    id: &'a str,
    href: &'a str,
    query: TableQuery<'a>,
    /// The request's sort, only by sortable columns.
    sort: Vec<(&'a str, bool)>,
    /// The request's column filters, only of filterable columns.
    filters: Vec<(&'a str, &'a str)>,
    keep: Vec<&'a str>,
    selects: Vec<FilterSelect<'a>>,
    search: bool,
    columns: Vec<Column<'a>>,
    rows: Vec<Row<'a>>,
    total: Option<usize>,
    choose_columns: bool,
    bulk: Option<(&'a str, Vec<(&'a str, &'a str)>)>,
    select_all: bool,
    csv: Option<&'a str>,
    json: Option<&'a str>,
    empty: Markup,
    loading: bool,
    edit: Option<&'a str>,
    caption: Option<&'a str>,
    totals: Vec<(&'a str, &'a str)>,
    footer: Vec<Row<'a>>,
    striped: bool,
    density: Density,
    group_by: Option<&'a str>,
}

impl Table<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("column", PropKind::Item, "key: &'a str, label: &'a str")
            .doc("A column."),
        Prop::new("sortable", PropKind::Modifier, "")
            .doc("The column added last sorts the table."),
        Prop::new("filterable", PropKind::Modifier, "")
            .doc("The column added last gets a text box in the \"Filters\" panel, `?q.<id>.<key>=`."),
        Prop::new("numeric", PropKind::Modifier, "")
            .doc("The column added last holds numbers."),
        Prop::new("editable", PropKind::Modifier, "")
            .doc("The column added last becomes a text box when its row is edited in place (`Table::edit`)."),
        Prop::new("edit", PropKind::Value, "action: &'a str")
            .doc("Rows (with a `Row::key`) can be edited in place."),
        Prop::new("width", PropKind::Modifier, "width: &'a str")
            .doc("A CSS width for the column added last, such as `6rem` or `30%`."),
        Prop::new("priority", PropKind::Modifier, "priority: u8")
            .default("1")
            .doc("1 always shown, 2 hidden in a narrow table, 3 below a medium one; values then show under the first cell."),
        Prop::new("rows", PropKind::Value, "rows: impl IntoIterator<Item = R>")
            .doc("The rows, already sorted and filtered as `Table::sort` and `Table::filter` say: `Row`s, or tuples of cells."),
        Prop::new("keep", PropKind::Item, "name: &'a str")
            .doc("A query parameter of this page (`status`) carried, with the request's value, by every sort link, the search form, the pager and the CSV link."),
        Prop::new("filter_select", PropKind::Item, "name: &'a str, label: &'a str, options: impl IntoIterator<Item = (&'a str, &'a str)>")
            .doc("A `<select>` of `(value, text)` options in the search form, filtering by the query parameter `name`, which the table then keeps."),
        Prop::new("hide_search", PropKind::Switch, "")
            .doc("No search box (a display-only table, or one filtered by `filter_select` alone)."),
        Prop::new("paged", PropKind::Number, "total: usize")
            .doc("Page the rows."),
        Prop::new("paged_from", PropKind::Value, "meta: &PagerMeta")
            .doc("Page the rows from what Loco's `query::fetch_page` or `query::paginate` returns (feature `loco`)."),
        Prop::new("choose_columns", PropKind::Switch, "")
            .doc("A \"Columns\" chooser."),
        Prop::new("bulk", PropKind::Value, "action: &'a str, buttons: impl IntoIterator<Item = (&'a str, &'a str)>")
            .doc("A checkbox per row (rows need a `Row::key`) and a bar of `(value, label)` buttons posting to `action`."),
        Prop::new("select_all", PropKind::Switch, "")
            .doc("A header checkbox posting `all=1` to the bulk action, whose URL carries the query: every row the filter matches."),
        Prop::new("csv", PropKind::Value, "href: &'a str")
            .doc("A \"Download CSV\" link to `href` with the current sort, filter and columns appended."),
        Prop::new("json", PropKind::Value, "href: &'a str")
            .doc("A \"Download JSON\" link to `href` with the current sort, filter and columns appended."),
        Prop::new("empty", PropKind::Value, "message: impl Render").default("No rows match.")
            .doc("What the body says when there are no rows: text, or markup (a call to action)."),
        Prop::new("loading", PropKind::Condition, "loading: bool")
            .doc("Skeleton rows with `aria-busy` instead of the rows."),
        Prop::new("caption", PropKind::Value, "text: &'a str")
            .doc("A `<caption>` naming the table for assistive tech, visually hidden."),
        Prop::new("total", PropKind::Value, "key: &'a str, text: &'a str")
            .doc("The cell of the column `key` in a totals row in `<tfoot>`."),
        Prop::new("footer", PropKind::Item, "row: impl Into<Row<'a>>")
            .doc("A row in `<tfoot>`, after the totals."),
        Prop::new("striped", PropKind::Switch, "")
            .doc("Every other row on a gray band."),
        Prop::new("density", PropKind::Value, "density: Density").default("Density::Comfortable")
            .doc("`Density::Compact` halves the cell padding."),
        Prop::new("group_by", PropKind::Value, "key: &'a str")
            .doc("A group header row wherever the column `key`'s cell changes (rows sorted by it)."),
    ];
}

impl Ui {
    /// A table `id` whose links and forms go to `href`; add columns with [`Table::column`].
    /// `href` may carry a query of its own (`/orders?status=paid`), kept by every link and
    /// form, or be empty for this page (links then start with `?`), as a display-only table
    /// with no sortable column, no search box and no pager never links anywhere.
    pub fn table<'a>(&'a self, id: &'a str, href: &'a str) -> Table<'a> {
        Table {
            ui: self,
            id,
            href,
            query: TableQuery::from_ui(self, id),
            sort: Vec::new(),
            filters: Vec::new(),
            keep: Vec::new(),
            selects: Vec::new(),
            search: true,
            columns: Vec::new(),
            rows: Vec::new(),
            total: None,
            choose_columns: false,
            bulk: None,
            select_all: false,
            csv: None,
            json: None,
            empty: html! { (self.text(Text::NoRows)) },
            loading: false,
            edit: None,
            caption: None,
            totals: Vec::new(),
            footer: Vec::new(),
            striped: false,
            density: Density::Comfortable,
            group_by: None,
        }
    }
}

/// The class that hides a column of priority 2 or 3 by the table's width.
fn prio(c: &Column) -> Option<&'static str> {
    match c.priority {
        2 => Some("lui-table-p2"),
        3 => Some("lui-table-p3"),
        _ => None,
    }
}

impl<'a> Table<'a> {
    fn last(mut self, change: impl FnOnce(&mut Column<'a>)) -> Self {
        if let Some(c) = self.columns.last_mut() {
            change(c);
        }
        self
    }

    /// A column: `key` names it in `?sort.<id>=`, `?cols.<id>=` and `?q.<id>.<key>=`, `label`
    /// is its header.
    pub fn column(mut self, key: &'a str, label: &'a str) -> Self {
        self.columns.push(Column::plain(key, label));
        self
    }

    /// The column added last sorts the table: its header links to `?sort.<id>=<key>` (and
    /// `-<key>` when it is already sorted ascending), and once the table is sorted a small
    /// "then by" link beside it adds it as the next sort key (`?sort.<id>=name,-size`).
    pub fn sortable(self) -> Self {
        let mut table = self.last(|c| c.sortable = true);
        let columns = &table.columns;
        table.sort = (table.query.sort.iter())
            .filter(|(k, _)| columns.iter().any(|c| c.sortable && c.key == *k))
            .copied()
            .collect();
        table
    }

    /// The column added last gets a text box in the "Filters" panel of the search form
    /// (`?q.<id>.<key>=`); the route filters by [`Table::filters`] or
    /// [`TableQuery::matches_in`].
    pub fn filterable(self) -> Self {
        let mut table = self.last(|c| c.filterable = true);
        let columns = &table.columns;
        table.filters = (table.query.filters.iter())
            .filter(|(k, _)| columns.iter().any(|c| c.filterable && c.key == *k))
            .copied()
            .collect();
        table
    }

    /// The column added last holds numbers: right-aligned, tabular figures.
    pub fn numeric(self) -> Self {
        self.last(|c| c.numeric = true)
    }

    /// The column added last becomes a text box when its row is edited in place ([`Table::edit`]).
    pub fn editable(self) -> Self {
        self.last(|c| c.editable = true)
    }

    /// Rows (with a [`Row::key`]) can be edited in place: each gets an "Edit" link to
    /// `?edit.<id>=<key>`, which draws that row's editable columns as text boxes and a
    /// "Save" button posting to `action` (`key`, one field per editable column named after
    /// it, `returns_to`). The route saves and redirects to `returns_to`.
    pub fn edit(mut self, action: &'a str) -> Self {
        self.edit = Some(action);
        self
    }

    /// A CSS width for the column added last, such as `6rem` or `30%`.
    pub fn width(self, width: &'a str) -> Self {
        self.last(|c| c.width = Some(width))
    }

    /// How much the column added last matters on a small screen: 1 (the default) is always
    /// shown, 2 hides when the table is under 30rem wide, 3 under 48rem (container queries).
    /// A hidden column's values still show, label and value, under the row's first cell.
    pub fn priority(self, priority: u8) -> Self {
        self.last(|c| c.priority = priority.clamp(1, 3))
    }

    /// The rows, already sorted and filtered as [`Table::sort`] and [`Table::filter`] say;
    /// cells in column order. Each is a [`Row`], or a tuple of cells (anything `Render`:
    /// `(o.id, o.customer, ui.badge(o.status))`) when it needs no key, detail or menu.
    pub fn rows<R: Into<Row<'a>>>(mut self, rows: impl IntoIterator<Item = R>) -> Self {
        self.rows = rows.into_iter().map(Into::into).collect();
        self
    }

    /// A query parameter of this page (`status`, say) that every sort link, the search form,
    /// the pager and the CSV link carry with the request's value, so a filter the page draws
    /// itself survives sorting and paging. Absent or empty, it is not carried.
    pub fn keep(mut self, name: &'a str) -> Self {
        self.keep.push(name);
        self
    }

    /// A `<select>` in the search form filtering by the query parameter `name`: `(value,
    /// text)` options, the request's value chosen (an `("", "All")` option for no filter). The
    /// table keeps `name` in its links; the route filters the rows by `ui.param(name)`. With
    /// filterable columns it sits in the "Filters" panel beside their boxes.
    pub fn filter_select(
        mut self,
        name: &'a str,
        label: &'a str,
        options: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        self.selects.push(FilterSelect {
            name,
            label,
            options: options.into_iter().collect(),
            value: self.ui.param(name).unwrap_or(""),
        });
        self.keep(name)
    }

    /// No search box: a display-only table, or one filtered by [`Table::filter_select`] alone.
    pub fn hide_search(mut self) -> Self {
        self.search = false;
        self
    }

    /// Page the rows: `total` is the row count after filtering. Given every row, the table
    /// shows the current page of them; given only the current page's rows ([`Table::page`],
    /// [`Table::per_page`]), it shows those. The page size the visitor picks is remembered
    /// as `per.<id>`.
    pub fn paged(mut self, total: usize) -> Self {
        self.total = Some(total);
        self
    }

    /// Page the rows from what Loco's `query::fetch_page` or `query::paginate` returns
    /// (feature `loco`): `.paged(meta.total_items)`. Ask Loco for the page the table wants
    /// with `PaginationQuery { page: table.page() as u64, page_size: table.per_page() as u64 }`,
    /// ordered by [`crate::loco::order_by`]; the `loco` module docs have the whole handler.
    #[cfg(feature = "loco")]
    pub fn paged_from(self, meta: &PagerMeta) -> Self {
        self.paged(usize::try_from(meta.total_items).unwrap_or(usize::MAX))
    }

    /// A "Columns" chooser: links that toggle `?cols.<id>=`.
    pub fn choose_columns(mut self) -> Self {
        self.choose_columns = true;
        self
    }

    /// A checkbox per row (rows need a [`Row::key`]) and a bar of `(value, label)` buttons
    /// posting to `action`: `row=<key>` per ticked row and `action=<value>`.
    pub fn bulk(
        mut self,
        action: &'a str,
        buttons: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        self.bulk = Some((action, buttons.into_iter().collect()));
        self
    }

    /// A checkbox in the header of the [`Table::bulk`] column, "Select all N matching rows",
    /// posting `all=1`. The bulk form's URL then carries the table's query (sort, search,
    /// column filters, kept parameters), so the route reads the same [`Ui::table_query`] and
    /// acts on every row it matches, not only this page's. Ticking the visible boxes as
    /// well needs script: the enhancement script does it.
    pub fn select_all(mut self) -> Self {
        self.select_all = true;
        self
    }

    /// A "Download CSV" link to `href` with the current sort, filter and columns appended.
    pub fn csv(mut self, href: &'a str) -> Self {
        self.csv = Some(href);
        self
    }

    /// A "Download JSON" link to `href` with the current sort, filter and columns appended,
    /// beside or instead of [`Table::csv`].
    pub fn json(mut self, href: &'a str) -> Self {
        self.json = Some(href);
        self
    }

    /// What the body says when there are no rows: text, or markup such as a message and a
    /// link to add the first row.
    pub fn empty(mut self, message: impl Render) -> Self {
        self.empty = message.render();
        self
    }

    /// Skeleton rows with `aria-busy` instead of the rows: the data is still coming.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// A `<caption>` that names the table for screen readers (the first thing they read of
    /// it), visually hidden since the page's heading usually names it for everyone else.
    pub fn caption(mut self, text: &'a str) -> Self {
        self.caption = Some(text);
        self
    }

    /// The cell under the column `key` in a totals row in `<tfoot>` (`"36 files"`,
    /// `"12.4 MB"`); one row holds every total, the other cells empty.
    pub fn total(mut self, key: &'a str, text: &'a str) -> Self {
        self.totals.push((key, text));
        self
    }

    /// A row in `<tfoot>` after the totals, cells in column order as in [`Table::rows`].
    pub fn footer(mut self, row: impl Into<Row<'a>>) -> Self {
        self.footer.push(row.into());
        self
    }

    /// Every other body row on a gray band (`--lui-gray-2`).
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }

    /// How much room the cells get: [`Density::Compact`] halves the padding.
    pub fn density(mut self, density: Density) -> Self {
        self.density = density;
        self
    }

    /// A full-width group header row before each run of rows whose cell in the column `key`
    /// is the same; the route sorts the rows by that column first (Radix Table has none;
    /// after Tremor's and TanStack's grouped rows, without collapsing, which needs script
    /// or a `<details>` per group that a table cannot hold).
    pub fn group_by(mut self, key: &'a str) -> Self {
        self.group_by = Some(key);
        self
    }

    /// The requested sort as `(column key, descending)`, most significant first, only by
    /// sortable columns; empty when unsorted.
    pub fn sort(&self) -> &[(&'a str, bool)] {
        &self.sort
    }

    /// The requested search text, trimmed, as typed.
    pub fn filter(&self) -> &'a str {
        self.query.filter
    }

    /// The requested column filters as `(column key, text)`, only of filterable columns.
    pub fn filters(&self) -> &[(&'a str, &'a str)] {
        &self.filters
    }

    /// The keys of the columns to show, in table order.
    pub fn visible(&self) -> Vec<&'a str> {
        self.query
            .cols_in(&self.columns)
            .unwrap_or_else(|| self.columns.iter().map(|c| c.key).collect())
    }

    /// The requested page, 1-based.
    pub fn page(&self) -> usize {
        self.query.page.unwrap_or(1)
    }

    /// Rows per page: the visitor's remembered choice (at most 50), or 10.
    pub fn per_page(&self) -> usize {
        self.query.per_page()
    }
}

impl Render for Table<'_> {
    fn render(&self) -> Markup {
        let cols = self.query.cols_in(&self.columns);
        // A query in `href` is kept like `.keep(..)`: forms drop an action's query, and links
        // would otherwise end up with two `?`.
        let (href, own) = self.href.split_once('?').unwrap_or((self.href, ""));
        let own: Vec<(String, String)> = own
            .split('&')
            .filter_map(|p| p.split_once('=').or(Some((p, ""))))
            .filter(|(k, _)| !k.is_empty())
            .map(|(k, v)| (decode(k).into_owned(), decode(v).into_owned()))
            .collect();
        let mut keep: Vec<(&str, &str)> =
            own.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        for name in &self.keep {
            if let Some(v) = self.ui.param(name).filter(|v| !v.is_empty())
                && !keep.iter().any(|(k, _)| k == name)
            {
                keep.push((name, v));
            }
        }
        let edit_key = format!("edit.{}", self.id);
        let (link, done) = (
            self.ui.link_with(&edit_key, ""),
            self.ui.link_without(&edit_key),
        );
        let edit = self.edit.map(|action| Editing {
            action,
            key: self.ui.param(&edit_key).filter(|k| !k.is_empty()),
            link: &link,
            done: &done,
        });
        let view = View {
            caps: self.ui.caps,
            id: self.id,
            href,
            columns: &self.columns,
            rows: &self.rows,
            sort: &self.sort,
            filter: self.filter(),
            filters: &self.filters,
            keep,
            cols: cols.as_deref(),
            choose_columns: self.choose_columns,
            bulk: self
                .bulk
                .as_ref()
                .map(|(action, buttons)| (*action, buttons.as_slice())),
            select_all: self.select_all,
            matching: self.total.unwrap_or(self.rows.len()),
            csv: self.csv,
            json: self.json,
            empty: &self.empty,
            loading: self.loading,
            edit,
            strings: self.ui.strings,
            search: self.search,
            selects: &self.selects,
            caption: self.caption,
            totals: &self.totals,
            footer: &self.footer,
            striped: self.striped,
            density: self.density,
            group_by: self
                .group_by
                .and_then(|key| self.columns.iter().position(|c| c.key == key)),
        };
        match self.total {
            Some(total) => crate::paged_table::paged(view, total, self.page(), self.per_page()),
            None => view.render(true),
        }
    }
}

/// Percent-encode a query value: everything but unreserved characters.
pub(crate) fn encode(s: &str) -> String {
    Encoded(s).to_string()
}

/// [`encode`] written straight into a formatter, so a link can be built without a `String`
/// per pair.
pub(crate) struct Encoded<'a>(pub(crate) &'a str);

impl std::fmt::Display for Encoded<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for b in self.0.bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    f.write_char(b as char)?
                }
                b' ' => f.write_char('+')?,
                _ => write!(f, "%{b:02X}")?,
            }
        }
        Ok(())
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the shadcn data-table and the Origin UI tables: a toolbar (search, filters, columns,
   CSV as an outline button) that wraps with the search full width in a narrow table; a
   bordered rounded frame that scrolls sideways inside itself, the first column (with the
   checkbox, if any) sticky and soft shadows at an edge only while there is more that way;
   a --lui-gray-2 header, --lui-gray-2 row hover, --lui-brand-3 selected rows. The table is its own
   container: .priority(2) columns hide under 30rem and .priority(3) under 48rem, their
   values then listed under the row's first cell. */
.lui-table { container: lui-table / inline-size; }
.lui-table-toolbar { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space); margin-bottom: calc(var(--lui-space) * 2); }
.lui-table-filter { flex: 1; min-width: min(14rem, 100%); }
.lui-table-filter form { display: flex; flex-wrap: wrap; gap: var(--lui-space); align-items: center; }
.lui-table-filter-input { flex: 1; min-width: 8rem; max-width: 24rem; }
.lui-table-clear { color: var(--lui-muted); font-size: 0.875rem; }
.lui-table-clear:hover { color: var(--lui-fg); }
.lui-table-cols { position: relative; font-size: 0.875rem; }
.lui-table-cols summary { list-style: none; }
.lui-table-cols summary::-webkit-details-marker { display: none; }
.lui-table-cols ul {
  position: absolute; right: 0; z-index: 2; margin: 0.25rem 0 0; padding: 0.25rem; list-style: none; min-width: 10rem;
  background: var(--lui-popover); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); box-shadow: var(--lui-shadow-md), var(--lui-highlight);
}
.lui-table-cols li { max-width: none; }
.lui-table-cols a { display: flex; gap: 0.5rem; padding: 0.375rem 0.5rem; border-radius: var(--lui-radius-sm); color: inherit; text-decoration: none; }
.lui-table-cols a:hover { background: var(--lui-accent); color: var(--lui-on-accent); }
.lui-table-cols-mark { color: var(--lui-fg); }
.lui-table-scroll {
  /* position: the scroller contains the visually hidden (absolute) labels inside it, which
     would otherwise hang out past it and widen the page. */
  position: relative; overflow-x: auto; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  background:
    linear-gradient(to right, var(--lui-bg) 40%, transparent) left / 1.5rem 100% no-repeat local,
    linear-gradient(to left, var(--lui-bg) 40%, transparent) right / 1.5rem 100% no-repeat local,
    radial-gradient(farthest-side at 0 50%, color-mix(in srgb, var(--lui-fg) 16%, transparent), transparent) left / 0.75rem 100% no-repeat scroll,
    radial-gradient(farthest-side at 100% 50%, color-mix(in srgb, var(--lui-fg) 16%, transparent), transparent) right / 0.75rem 100% no-repeat scroll;
}
.lui-table-scroll > table { width: 100%; margin: 0; }
.lui-table-scroll :is(th, td):first-child { position: sticky; inset-inline-start: 0; z-index: 1; background: var(--lui-bg); }
.lui-table-has-select :is(th, td):nth-child(2) { position: sticky; inset-inline-start: 2.5rem; z-index: 1; background: var(--lui-bg); }
.lui-table-scroll thead th:is(:first-child, :nth-child(2)) { background: var(--lui-gray-2); }
.lui-table table { table-layout: auto; border-collapse: separate; border-spacing: 0; }
.lui-table tbody tr:last-child > * { border-bottom: 0; }
.lui-table thead th { background: var(--lui-gray-2); color: var(--lui-muted); font-size: 0.8125rem; font-weight: 500; white-space: nowrap; }
.lui-table tbody tr > td { transition: background-color var(--lui-duration-fast); }
.lui-table tbody tr:hover > td { background: var(--lui-gray-2); }
.lui-table tbody tr:has(.lui-table-check:checked) > td { background: var(--lui-brand-3); }
.lui-table th a { color: inherit; text-decoration: none; display: inline-flex; align-items: center; gap: 0.25rem; }
.lui-table th a:hover { color: var(--lui-fg); }
.lui-table th.lui-table-sorted { color: var(--lui-fg); }
.lui-table-arrow { font-size: 0.75em; margin-left: 0.25em; }
/* Code in a cell is plain monospace text, so a long path wraps without a broken box. */
.lui-table td code { background: none; border: 0; padding: 0; }
.lui-table-num { text-align: right; font-variant-numeric: tabular-nums; white-space: nowrap; }
.lui-table-select { width: 2.5rem; text-align: center; }
.lui-table-menu { width: 3.5rem; text-align: center; text-wrap: nowrap; }
.lui-table-edit-col { width: 10rem; }
.lui-table-edit { white-space: nowrap; text-align: right; }
.lui-table-edit .lui-button + .lui-button { margin-left: 0.25rem; }
.lui-table-edit-input { width: 100%; box-sizing: border-box; min-height: 2rem; padding-block: 0.25rem; }
.lui-table-empty { color: var(--lui-muted); text-align: center; padding: var(--lui-space-6); height: 6rem; }
.lui-table tbody tr:hover > .lui-table-empty { background: none; }
/* Priorities: the cells hide, and the matching lines under the first cell appear. */
.lui-table-more { display: none; margin: var(--lui-space-1) 0 0; font-size: 0.8125rem; color: var(--lui-muted); }
.lui-table-more > div { display: none; gap: 0.375rem; }
.lui-table-more dt::after { content: ":"; }
.lui-table-more dd { margin: 0; color: var(--lui-fg); }
@container lui-table (width < 48rem) {
  .lui-table .lui-table-p3 { display: none; }
  .lui-table-more { display: grid; }
  .lui-table-more > .lui-table-more-p3 { display: flex; }
}
@container lui-table (width < 30rem) {
  .lui-table .lui-table-p2 { display: none; }
  .lui-table-more > .lui-table-more-p2 { display: flex; }
  .lui-table-filter { flex-basis: 100%; }
  .lui-table-filter-input { max-width: none; }
}
.lui-table-detail summary { cursor: pointer; list-style: none; font-weight: 400; }
.lui-table-detail summary::-webkit-details-marker { display: none; }
.lui-table-detail summary::before { content: "\25B8"; color: var(--lui-muted); margin-right: 0.4em; }
.lui-table-detail[open] summary::before { content: "\25BE"; }
.lui-table-detail-body { margin: 0.5rem 0 0.25rem 1.2em; font-size: 0.875rem; color: var(--lui-muted); }
.lui-table-detail-body > :last-child { margin-bottom: 0; }
.lui-table-bulk { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space); margin-top: calc(var(--lui-space) * 2); font-size: 0.875rem; color: var(--lui-muted); }
.lui-table-skeleton td span { display: block; height: 1rem; margin: 0.125rem 0; border-radius: var(--lui-radius-sm); background: var(--lui-accent); animation: lui-table-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
@keyframes lui-table-pulse { 50% { opacity: 0.5; } }
@media (prefers-reduced-motion: reduce) { .lui-table-skeleton td span { animation: none; } }
/* M41: multi-sort, the filters panel, select-all, footer, stripes, density, groups, row
   links and tones. */
.lui-table-sort-index { min-width: 1rem; padding: 0 0.25rem; border-radius: 999px; background: var(--lui-gray-4); color: var(--lui-fg); font-size: 0.6875rem; line-height: 1rem; text-align: center; }
.lui-table th .lui-table-then { margin-inline-start: 0.25rem; padding: 0 0.3rem; border-radius: var(--lui-radius-sm); color: var(--lui-muted); opacity: 0; }
.lui-table th:hover .lui-table-then, .lui-table th .lui-table-then:focus-visible { opacity: 1; }
.lui-table th .lui-table-then:hover { background: var(--lui-gray-4); color: var(--lui-fg); }
@media (pointer: coarse) { .lui-table th .lui-table-then { opacity: 1; } }
/* The "Filters" panel opens as a row of its own under the search, after the buttons, so an
   active filter (the panel stays open) never covers the table. */
.lui-table-filters { font-size: 0.875rem; }
.lui-table-filters[open] { flex-basis: 100%; order: 1; }
.lui-table-filters summary { list-style: none; width: fit-content; }
.lui-table-filters summary::-webkit-details-marker { display: none; }
.lui-table-filters-count { min-width: 1.25rem; padding: 0 0.375rem; border-radius: 999px; background: var(--lui-primary); color: var(--lui-on-primary); font-size: 0.75rem; line-height: 1.25rem; text-align: center; }
.lui-table-filters-body {
  display: grid; grid-template-columns: repeat(auto-fill, minmax(min(12rem, 100%), 1fr)); gap: var(--lui-space-2); margin-top: var(--lui-space-2); padding: var(--lui-space-3);
  background: var(--lui-gray-2); border: 1px solid var(--lui-line); border-radius: var(--lui-radius);
}
.lui-table-all, .lui-table-check { margin: 0; }
.lui-table .lui-table-scroll tfoot td { background: var(--lui-gray-2); font-weight: 500; border-top: 1px solid var(--lui-line); border-bottom: 0; }
.lui-table-striped tbody tr:nth-child(even) > td { background: var(--lui-gray-2); }
.lui-table-striped tbody tr:hover > td { background: var(--lui-gray-3); }
.lui-table-compact :is(th, td) { padding: var(--lui-space-1) var(--lui-space-2); font-size: 0.8125rem; }
.lui-table-compact thead th { height: 2rem; }
.lui-table tbody tr.lui-table-group > td { background: var(--lui-gray-2); color: var(--lui-muted); font-size: 0.8125rem; font-weight: 600; }
.lui-table tbody tr.lui-table-disabled > td { color: var(--lui-muted); }
.lui-table-ok { --lui-table-tone: var(--lui-ok); }
.lui-table-warn { --lui-table-tone: var(--lui-warn); }
.lui-table-danger { --lui-table-tone: var(--lui-danger); }
.lui-table tbody tr:is(.lui-table-ok, .lui-table-warn, .lui-table-danger) > td { background: color-mix(in srgb, var(--lui-table-tone) 8%, var(--lui-bg)); }
.lui-table tbody tr:is(.lui-table-ok, .lui-table-warn, .lui-table-danger) > td:first-child { box-shadow: inset 3px 0 var(--lui-table-tone); }
/* A row link: the chevron's box is stretched over its row (Tremor), the row's own controls
   lifted above it. A sticky cell is a stacking context, so it is lifted whole. */
.lui-table-go-col { width: 2.5rem; }
.lui-table-go { width: 2.5rem; text-align: center; }
.lui-table tbody tr.lui-table-linked { position: relative; }
.lui-table tbody tr.lui-table-linked:hover > td { cursor: pointer; }
.lui-table-row-link { display: inline-block; color: var(--lui-muted); font-size: 1.25rem; line-height: 1; text-decoration: none; }
.lui-table-row-link::after { content: ""; position: absolute; inset: 0; z-index: 1; }
.lui-table-linked :is(.lui-table-check, .lui-button, summary, a:not(.lui-table-row-link)) { position: relative; z-index: 2; }
.lui-table-linked > td:is(:first-child, .lui-table-select + td):has(.lui-table-check, .lui-button, summary, a:not(.lui-table-row-link)) { z-index: 2; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_flip_direction_and_keep_the_filter() {
        let ui = Ui::from_request("/t", "sort.t=name,note&q.t=a+b&per=5", "");
        let table = ui
            .table("t", "/t")
            .column("name", "Name")
            .sortable()
            .column("note", "Note")
            .keep("per");
        assert_eq!(table.sort(), [("name", false)], "only sortable keys");
        let m = table.render().into_string();
        assert!(
            m.contains("href=\"/t?sort.t=-name&amp;q.t=a+b&amp;per=5\""),
            "{m}"
        );
        assert!(m.contains("name=\"per\" value=\"5\""));
        assert!(m.contains("name=\"sort.t\" value=\"name\""));
        assert!(
            m.contains("href=\"/t?sort.t=name&amp;per=5\""),
            "clear link"
        );
        assert!(m.contains("aria-sort=\"ascending\""));
        assert!(m.contains("No rows match."));
        assert!(!m.contains("view-transition-name"));
    }

    /// `?sort.<id>=a,-b`: the first key flips from its header, another header sorts by
    /// itself alone, "then by" adds a key, and each sorted header shows its index.
    #[test]
    fn several_sort_keys() {
        let ui = Ui::from_request("/t", "sort.t=-size,name,zzz,name", "");
        let table = ui
            .table("t", "/t")
            .column("name", "Name")
            .sortable()
            .column("size", "Size")
            .sortable()
            .column("kind", "Kind")
            .sortable();
        assert_eq!(table.sort(), [("size", true), ("name", false)]);
        let m = table.render().into_string();
        assert!(
            m.contains(r#"href="/t?sort.t=size,name""#),
            "size flips, name stays: {m}"
        );
        assert!(m.contains(r#"href="/t?sort.t=name">Name"#), "name alone");
        assert!(
            m.contains(r#"href="/t?sort.t=-size,name,kind" aria-label="Then by Kind""#),
            "{m}"
        );
        assert!(
            m.contains(r#"href="/t?sort.t=-size,-name" aria-label="Then by Name""#),
            "a later key flips in place"
        );
        assert!(m.contains(r#"<span class="lui-table-sort-index" aria-hidden="true">2</span>"#));
        assert_eq!(m.matches("aria-sort").count(), 1, "only the first key");
        let mut v = vec![(1, "b"), (2, "a"), (2, "b"), (1, "a")];
        ui.table_query("t", &["name", "size"])
            .sort_by(&mut v, |a, b, key| match key {
                "size" => a.0.cmp(&b.0),
                _ => a.1.cmp(b.1),
            });
        assert_eq!(v, [(2, "a"), (2, "b"), (1, "a"), (1, "b")]);
    }

    #[test]
    fn columns_links_toggle_one_key_in_table_order() {
        let ui = Ui::from_request("/t", "cols.t=a,c&q.t=x", "");
        let table = ui
            .table("t", "/t")
            .column("a", "A")
            .sortable()
            .column("b", "B")
            .sortable()
            .numeric()
            .column("c", "C");
        let cols = &table.columns;
        assert_eq!(cols_from_query(cols, Some("c,zzz,a")), Some(vec!["a", "c"]));
        assert_eq!(cols_from_query(cols, Some("zzz")), None);
        let m = table.render().into_string();
        assert!(
            m.contains("href=\"/t?q.t=x&amp;cols.t=c\" aria-pressed=\"true\""),
            "a shown column links to hiding it: {m}"
        );
        assert!(
            m.contains("href=\"/t?q.t=x&amp;cols.t=a%2Cb%2Cc\" aria-pressed=\"false\""),
            "a hidden one links to showing it in place"
        );
        assert!(
            m.contains("name=\"cols.t\" value=\"a,c\""),
            "the filter form keeps the columns"
        );
        assert!(!m.contains(">B<"));
    }

    #[test]
    fn loading_and_bulk_states() {
        let ui = Ui::default();
        let m = ui
            .table("t", "/t")
            .column("a", "A")
            .sortable()
            .loading(true)
            .bulk("/b", [("x", "X")])
            .render()
            .into_string();
        assert!(m.contains("aria-busy=\"true\"") && m.contains("lui-table-skeleton"));
        assert!(m.contains("<form method=\"post\" action=\"/b\" id=\"lui-table-t-bulk\""));
        assert!(m.contains("<button type=\"submit\" class=\"lui-button lui-button-small\" name=\"action\" value=\"x\">X</button>"), "{m}");
        assert_eq!(crate::slug("src/a.txt"), "src-a-txt");
    }

    /// Column filters: a box per filterable column in the "Filters" panel, opened while one
    /// is set, carried by every link, and cleared with the search.
    #[test]
    fn column_filters() {
        let ui = Ui::from_request("/t", "q.t.kind=img&q.t.name=&q.t.size=9&status=a", "");
        let table = ui
            .table("t", "/t")
            .column("name", "Name")
            .sortable()
            .filterable()
            .column("kind", "Kind")
            .filterable()
            .column("size", "Size")
            .filter_select("status", "Status", [("", "All"), ("a", "A")]);
        assert_eq!(table.filters(), [("kind", "img")], "size is not filterable");
        let m = table.render().into_string();
        assert!(
            m.contains(r#"<details class="lui-table-filters" open>"#),
            "{m}"
        );
        assert!(m.contains(r#"name="q.t.kind""#) && m.contains(r#"value="img""#));
        assert!(m.contains(r#"<span class="lui-table-filters-count">1</span>"#));
        assert!(
            m.contains(r#"<div class="lui-table-filters-body">"#)
                && m.contains(r#"<label class="lui-table-filter-select">"#),
            "the select joins the panel"
        );
        assert!(
            !m.contains(r#"type="hidden" name="q.t.kind""#),
            "its box sends it"
        );
        assert!(
            m.contains(r#"href="/t?sort.t=name&amp;q.t.kind=img&amp;status=a""#),
            "{m}"
        );
        assert!(
            m.contains(r#"class="lui-table-clear" href="/t?status=a""#),
            "{m}"
        );
        let q = ui.table_query("t", &[]);
        assert!(
            q.matches_in("kind", "IMG.png")
                && !q.matches_in("kind", "doc")
                && q.matches_in("name", "x")
        );
    }

    /// Select-all posts `all=1` to a bulk URL that carries the query; caption, totals,
    /// footer rows, stripes, density, groups, row links, disabled rows, tones and JSON.
    #[test]
    fn the_rest_of_the_features() {
        let ui = Ui::from_request("/t", "q.t=a&sort.t=kind", "");
        let rows = [
            Row::from(("a.txt", "text")).key("a.txt").href("/f/a.txt"),
            Row::from(("b.txt", "text"))
                .key("b.txt")
                .disabled()
                .href("/f/b"),
            Row::from(("c.png", "image")).key("c.png").danger(),
        ];
        let m = ui
            .table("t", "/t")
            .column("name", "Name")
            .column("kind", "Kind")
            .sortable()
            .bulk("/b", [("x", "X")])
            .select_all()
            .caption("Files")
            .total("name", "3 files")
            .footer(("All", "—"))
            .striped()
            .density(Density::Compact)
            .group_by("kind")
            .json("/t.json")
            .empty(html! { a href="/new" { "Add one" } })
            .rows(rows)
            .render()
            .into_string();
        assert!(m.contains(r#"class="lui-table lui-table-striped lui-table-compact""#));
        assert!(m.contains(r#"<caption class="lui-sr">Files</caption>"#));
        assert!(m.contains(r#"<input type="checkbox" class="lui-table-all" name="all" value="1" form="lui-table-t-bulk" aria-label="Select all 3 matching rows">"#), "{m}");
        assert!(
            m.contains(r#"action="/b?sort.t=kind&amp;q.t=a" id="lui-table-t-bulk""#),
            "{m}"
        );
        assert!(m.contains(r#"<tfoot><tr class="lui-table-total"><td class="lui-table-select"></td><td>3 files</td><td></td><td></td></tr><tr><td class="lui-table-select"></td><td>All</td><td>—</td><td></td></tr></tfoot>"#), "{m}");
        assert_eq!(
            m.matches(r#"<tr class="lui-table-group">"#).count(),
            2,
            "text, then image"
        );
        assert!(m.contains(r#"<tr class="lui-table-linked">"#));
        assert!(
            m.contains(r#"<a class="lui-table-row-link" href="/f/a.txt" aria-label="Open a.txt">"#)
        );
        assert!(
            !m.contains("/f/b") && !m.contains(r#"value="b.txt""#),
            "a disabled row has no link or box"
        );
        assert!(m.contains(r#"<tr class="lui-table-disabled" aria-disabled="true">"#));
        assert!(m.contains(r#"<tr class="lui-table-danger">"#));
        assert!(m.contains(r#"href="/t.json?sort.t=kind&amp;q.t=a" download"#));
        let empty = ui
            .table("t", "/t")
            .column("n", "N")
            .empty(html! { a href="/new" { "Add one" } })
            .render()
            .into_string();
        assert!(empty.contains(
            r#"<td colspan="1" class="lui-table-empty"><a href="/new">Add one</a></td>"#
        ));
    }

    /// `.keep(..)` and a query in `href` reach every link and form: sort links, the search
    /// form, the columns chooser, the CSV link and the pager.
    #[test]
    fn kept_parameters_and_a_query_in_href_survive_everything() {
        let ui = Ui::from_request("/o", "status=paid&tab=2&q.o=a&page.o=2", "");
        let table = |href| {
            ui.table("o", href)
                .column("n", "N")
                .sortable()
                .choose_columns()
                .csv("/o.csv?x=1")
                .keep("status")
                .keep("absent")
                .rows([("a", ""), ("b", "")])
                .paged(40)
                .render()
                .into_string()
        };
        let m = table("/o?tab=2");
        assert!(!m.contains("?tab=2?") && !m.contains("absent"), "{m}");
        assert!(
            m.contains(r#"href="/o?sort.o=n&amp;q.o=a&amp;tab=2&amp;status=paid&amp;per.o=10""#),
            "sort link: {m}"
        );
        assert!(m.contains(r#"<form method="get" action="/o"><input type="hidden" name="tab" value="2"><input type="hidden" name="status" value="paid">"#));
        assert!(
            m.contains(r#"href="/o.csv?x=1&amp;q.o=a&amp;tab=2&amp;status=paid"#),
            "csv: {m}"
        );
        assert!(
            m.contains("q.o=a&amp;tab=2&amp;status=paid&amp;per.o=10&amp;page.o=3"),
            "pager: {m}"
        );
        // An empty `href`: relative links, forms with no action.
        let m = table("");
        assert!(
            m.contains(r#"href="?sort.o=n&amp;q.o=a"#)
                && m.contains(r#"<form method="get"><input"#)
        );
        assert!(
            m.contains(r#"class="lui-table-clear" href="?status=paid&amp;per.o=10""#),
            "{m}"
        );
    }

    #[test]
    fn search_box_and_filter_selects() {
        let ui = Ui::from_request("/o", "status=paid", "");
        let plain = |t: Table| t.column("n", "N").render().into_string();
        let hidden = plain(ui.table("o", "/o").hide_search());
        assert!(!hidden.contains("lui-table-toolbar") && !hidden.contains("<search"));
        let select = plain(ui.table("o", "/o").hide_search().filter_select(
            "status",
            "Status",
            [("", "All"), ("paid", "Paid")],
        ));
        assert!(
            !select.contains(r#"name="q.o""#) && !select.contains(r#"type="hidden""#),
            "{select}"
        );
        assert!(select.contains(r#"<label class="lui-table-filter-select"><span class="lui-sr">Status</span><select name="status"><option value="">All</option><option value="paid" selected>Paid</option></select></label>"#), "{select}");
        let row: Row = (1, "Ada", ui.badge("paid")).into();
        assert_eq!(row.cells.len(), 3);
        assert_eq!(row.cells[0].clone().into_string(), "1");
    }

    #[test]
    fn the_query_sorts_and_pages_a_vec() {
        let ui = Ui::from_request("/o", "sort.o=-n&page.o=9&q.o=%20X%20", "lui-ui=per.o=5");
        let q = ui.table_query("o", &["n"]);
        assert!(q.matches("axe") && !q.matches("bee") && q.filter() == "X");
        let mut v: Vec<u32> = (1..=12).collect();
        q.sort_by(&mut v, |a, b, _| a.cmp(b));
        assert_eq!(q.page_of(&v), (&[2, 1][..], 12), "page 9 of 3 is the last");
        assert_eq!(q.page_of::<u32>(&[]), (&[][..], 0));
        assert_eq!(q.visible(&["a", "b"]), ["a", "b"]);
        let ui = Ui::from_request("/o", "cols.o=b,zz", "");
        let q = ui.table_query("o", &[]);
        assert_eq!((q.visible(&["a", "b"]), q.sort()), (vec!["b"], &[][..]));
    }

    /// The bare keys of before M33 (`q`, `sort`, `dir`, `page`, `cols`) and `dir.<id>` are no
    /// longer read (M41).
    #[test]
    fn bare_keys_are_not_read() {
        let ui = Ui::from_request("/o", "sort=n&dir=desc&q=x&page=2&cols=n&dir.o=desc", "");
        let q = ui.table_query("o", &["n"]);
        assert_eq!(
            (q.sort(), q.filter(), q.page(), q.visible(&["n", "m"])),
            (&[][..], "", 1, vec!["n", "m"])
        );
    }

    /// Two tables on one page: each reads and writes its own keys, so one's sort, filter and
    /// page leave the other alone.
    #[test]
    fn two_tables_on_one_page_sort_and_filter_on_their_own() {
        let ui = Ui::from_request("/p", "sort.a=-n&q.b=zz&page.b=2", "");
        let (a, b) = (ui.table_query("a", &["n"]), ui.table_query("b", &["n"]));
        assert_eq!(
            (a.sort(), a.filter(), a.page()),
            (&[("n", true)][..], "", 1)
        );
        assert_eq!((b.sort(), b.filter(), b.page()), (&[][..], "zz", 2));
        let table = |id| {
            ui.table(id, "/p")
                .column("n", "N")
                .sortable()
                .column("m", "M")
                .rows([("1", "a"), ("2", "b")])
                .paged(40)
                .render()
                .into_string()
        };
        let (a, b) = (table("a"), table("b"));
        assert!(a.contains(r#"aria-sort="descending""#) && !b.contains("aria-sort"));
        assert!(a.contains(r#"name="q.a""#) && !a.contains("zz"), "{a}");
        assert!(
            b.contains(r#"value="zz""#) && b.contains(r#"name="q.b""#),
            "{b}"
        );
        assert!(
            a.contains(r#"href="/p?sort.a=n&amp;per.a=10""#),
            "a's sort link knows nothing of b: {a}"
        );
        assert!(
            b.contains(r#"href="/p?q.b=zz&amp;per.b=10&amp;page.b=3""#),
            "b pages on its own: {b}"
        );
        assert!(b.contains(r#"aria-current="page">2<"#) && a.contains(r#"aria-current="page">1<"#));
    }
}
