//! The table pages: `/table/minimal`, a table whose snippet is all its code, and `/table`,
//! with sort (several keys), search and column filters, paging, columns, CSV and JSON,
//! in-place edit, bulk actions on the ticked or every matching row, a totals row and the
//! features matrix; and `/table/rows`, the row-level features.

use crate::site::page;
use axum::{
    Form, Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use loco_ui::prelude::*;
use loco_ui::table::Density;
use serde::{Deserialize, Serialize};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/table", get(table_page))
        .route("/table/minimal", get(minimal_page))
        .route("/table/rows", get(rows_page))
        .route("/table.csv", get(table_csv))
        .route("/table.json", get(table_json))
        .route("/table/bulk", post(table_bulk))
        .route("/table/edit", post(table_edit))
}

/// Each page's live component, which the index shows too (`site::preview`), without the
/// visitor's renamed kinds there.
pub(crate) const PREVIEWS: &[super::Preview] = &[
    ("/table", |ui| files_page(ui, &Kinds::default())),
    ("/table/minimal", minimal_table),
    ("/table/rows", kinds_table),
];

const FILES: [(&str, u32, &str); 12] = [
    ("archive.tar", 40960, "backup"),
    ("build.rs", 1200, "script"),
    ("cargo.lock", 8800, "generated"),
    ("index.html", 2100, "page"),
    ("logo.svg", 3400, "image"),
    ("main.rs", 5600, "source"),
    ("notes.md", 900, "text"),
    ("photo.jpg", 250000, "image"),
    ("readme.md", 4100, "text"),
    ("style.css", 1500, "stylesheet"),
    ("tests.rs", 7700, "source"),
    ("video.mp4", 9800000, "video"),
];

/// A file: its path, its size in bytes and its kind.
type File = (String, u32, &'static str);

/// The table's columns, every one sortable; the CSV writes the visible ones.
const COLUMNS: [&str; 3] = ["name", "size", "kind"];

/// Thirty-six files filtered (search box and column filters) and sorted (every key) on the
/// server as the table's query says, in one place for the page, the downloads and the bulk
/// action on every matching row.
fn files(query: &TableQuery) -> Vec<File> {
    let mut files: Vec<File> = ["src", "docs", "old"]
        .iter()
        .flat_map(|dir| {
            FILES
                .iter()
                .map(move |f| (format!("{dir}/{}", f.0), f.1 * (dir.len() as u32), f.2))
        })
        .filter(|f| query.matches(&f.0) || query.matches(f.2))
        .filter(|f| query.matches_in("name", &f.0) && query.matches_in("kind", f.2))
        .collect();
    query.sort_by(&mut files, |a, b, key| match key {
        "size" => a.1.cmp(&b.1),
        "kind" => a.2.cmp(b.2),
        _ => a.0.cmp(&b.0),
    });
    files
}

/// Kinds renamed in place on `/table`, remembered per visitor.
#[derive(Default, Deserialize, Serialize)]
struct Kinds(Vec<(String, String)>);

impl Kinds {
    fn of<'a>(&'a self, file: &str, kind: &'a str) -> &'a str {
        self.0
            .iter()
            .find(|(f, _)| f == file)
            .map_or(kind, |(_, k)| k.as_str())
    }
}

// code: /table
/// A file as a row that expands, has its own menu, can be selected and edited in place.
fn file_row<'a>(file: &'a File, kinds: &'a Kinds) -> Row<'a> {
    let size = format!(
        "{} KB",
        loco_ui::paged_table::thousands(file.1 as usize / 1024)
    );
    Row::from((html! { code { (file.0) } }, size, kinds.of(&file.0, file.2)))
        .key(&file.0)
        .values(["", "", kinds.of(&file.0, file.2)])
        .detail(html! { p { "A " (file.2) " of " (file.1) " bytes, in " code { (file.0.split('/').next().unwrap_or("")) } "." } })
        .menu([MenuItem::link("Open", "/table"), MenuItem::action("Delete", "/table/bulk").danger()])
}
// end code

/// The table only renders and links: the route reads its query, fetches, sorts and slices.
fn files_page(ui: &Ui, kinds: &Kinds) -> Markup {
    // code: /table
    let query = ui.table_query("files", &COLUMNS);
    let files = files(&query);
    let (page, total) = query.page_of(&files);
    let bytes: u32 = files.iter().map(|f| f.1 / 1024).sum();
    let size = format!("{} KB", loco_ui::paged_table::thousands(bytes as usize));
    let count = format!("{total} files");
    lui! { Table("files", "/table") paged=(total) choose_columns csv="/table.csv" json="/table.json"
            edit="/table/edit" select_all caption="Files"
            empty="No files match this filter." loading=(ui.param("loading") == Some("1")) {
        column "name" "Name" sortable filterable;
        column "size" "Size" sortable numeric width="7rem" priority=2;
        column "kind" "Kind" sortable filterable editable width="9rem" priority=3;
        bulk "/table/bulk" ([("archive", "Archive"), ("delete", "Delete")]);
        total "name" (&count);
        total "size" (&size);
        rows (page.iter().map(|f| file_row(f, kinds)));
    } }
    // end code
}

/// Sorted by a header link, filtered by the search box, paged: the URL carries all three, so
/// the route reads them, fetches and slices, and the table draws the links for the next ones.
fn minimal_table(ui: &Ui) -> Markup {
    // code: /table/minimal
    let query = ui.table_query("minimal", &["name", "size"]);
    let mut files: Vec<_> = FILES.iter().filter(|f| query.matches(f.0)).collect();
    query.sort_by(&mut files, |a, b, key| match key {
        "size" => a.1.cmp(&b.1),
        _ => a.0.cmp(b.0),
    });
    let (page, total) = query.page_of(&files);
    lui! { Table("minimal", "/table/minimal") paged=(total) {
        column "name" "Name" sortable;
        column "size" "Bytes" sortable numeric;
        rows (page.iter().map(|f| Row::from((f.0, f.1))));
    } }
    // end code
}

async fn minimal_page(ui: Ui) -> Page {
    let table = minimal_table(&ui);
    page(
        &ui,
        "Table, minimal",
        html! {
            p { "Sort, filter and page: the whole route is the code below. " a href="/table" { "The full table" } " adds columns, CSV, row menus, in-place edit and bulk actions." }
            (table)
        },
    )
}

/// The row-level features on the twelve files: grouped by kind, links per row, tones, a
/// disabled row, stripes, the compact density, a footer row and markup when nothing matches.
fn kinds_table(ui: &Ui) -> Markup {
    // code: /table/rows
    let query = ui.table_query("kinds", &["kind", "name"]);
    // Nine of the files, in five kinds; the scripts, pages and stylesheets stay out.
    let shown = |f: &&(&str, u32, &str)| !["script", "page", "stylesheet"].contains(&f.2);
    let mut files: Vec<_> = FILES
        .iter()
        .filter(shown)
        .filter(|f| query.matches(f.0))
        .collect();
    query.sort_by(&mut files, |a, b, key| match key {
        "kind" => a.2.cmp(b.2),
        _ => a.0.cmp(b.0),
    });
    // Grouped by kind: a stable sort by kind last keeps the visitor's order inside each group.
    if query.sort().first().is_none_or(|s| s.0 != "kind") {
        files.sort_by(|a, b| a.2.cmp(b.2));
    }
    let links: Vec<String> = files
        .iter()
        .map(|f| format!("/table?q.files.name={}", f.0))
        .collect();
    fn row<'a>((f, href): (&&(&'static str, u32, &'static str), &'a String)) -> Row<'a> {
        let row = Row::from((f.0, f.2, f.1)).key(f.0).href(href);
        match f {
            (_, _, "backup") => row.disabled(),
            (_, size, _) if *size > 1_000_000 => row.danger(),
            (_, _, "generated") => row.warn(),
            (_, _, "source") => row.ok(),
            _ => row,
        }
    }
    lui! { Table("kinds", "/table/rows") striped density=(Density::Compact) group_by="kind" caption="Files by kind"
            empty={ "No file is called that. " a href="/table?q.files=" { "Search every file" } "." } {
        column "name" "Name" sortable;
        column "kind" "Kind" sortable priority=3;
        column "size" "Bytes" numeric;
        footer ((format!("{} files", files.len()), "", ""));
        rows (files.iter().zip(&links).map(row));
    } }
    // end code
}

/// What the table does without script, what the script adds, and what is out of scope.
const FEATURES: [(&str, &str, &str); 14] = [
    (
        "Sort by a column, flip it",
        "Header link ?sort.files=size, -size",
        "Swapped in place",
    ),
    (
        "Sort by several columns",
        "\"+\" link: ?sort.files=kind,-size; the header shows each key's place",
        "Swapped in place",
    ),
    (
        "Search all columns",
        "<search> form, ?q.files=",
        "Swapped in place",
    ),
    (
        "Filter one column",
        "\"Filters\" panel, ?q.files.kind=",
        "Swapped in place",
    ),
    (
        "Page, page size",
        "Page links and two forms; the size is remembered",
        "Swapped in place",
    ),
    (
        "Show, hide, order columns",
        "\"Columns\" links, ?cols.files=name,kind",
        "Swapped in place",
    ),
    (
        "Select rows, act on them",
        "Checkboxes posting to one bulk form",
        "Same",
    ),
    (
        "Select all matching rows",
        "Header box posts all=1; the form's URL carries the query",
        "Also ticks the boxes on the page",
    ),
    (
        "Edit a row in place",
        "?edit.files=<key>, one form, Post/Redirect/Get",
        "Swapped in place",
    ),
    (
        "Row detail, row menu, row link",
        "<details>, a popover menu, a stretched link",
        "Menu keys move between items",
    ),
    (
        "Totals, footer rows, groups",
        "<tfoot> rows, group header rows",
        "Same",
    ),
    ("Download CSV, JSON", "Links with the current query", "Same"),
    (
        "Resize or drag columns",
        "Out of scope: needs script; .width(..) and ?cols.files= instead",
        "Not added",
    ),
    (
        "Collapse a group",
        "Out of scope: a <details> cannot wrap table rows",
        "Not added",
    ),
];

async fn rows_page(ui: Ui) -> Page {
    let table = kinds_table(&ui);
    page(
        &ui,
        "Table, rows",
        html! {
            p { "Nine of the files grouped by kind, striped and compact: a row opens " a href="/table" { "the full table" } " filtered to it, the backup cannot be picked, and the tones mark large, generated and source files." }
            (table)
        },
    )
}

async fn table_page(ui: Ui, Saved(kinds): Saved<Kinds>) -> Page {
    let table = files_page(&ui, &kinds);
    let features = lui! { Table("features", "") hide_search caption="Features" {
        column "feature" "Feature";
        column "without" "Without script";
        column "with" "With the enhancement script";
        rows (FEATURES.iter().map(|(a, b, c)| (*a, *b, *c)));
    } };
    page(
        &ui,
        "Table",
        html! {
            p { a href="/table/minimal" { "The minimal table" } " shows only sorting, filtering and paging. Click a header to sort, again to flip, and \"+\" beside another to sort by it next. Type to filter, or open \"Filters\" for one column. Hide columns, tick rows (or every matching row) for the bulk form, open a row's menu or its detail. The page size you pick is remembered for your next visit. Every state is a URL, including " a href="/table?loading=1" { "the loading one" } "." }
            (table)
            p { a href="/table/rows" { "Table, rows" } " shows groups, whole-row links, tones, a disabled row, stripes and the compact density." }
            h2 { "Features" }
            (features)
        },
    )
}

/// Every matching file, as the table's query says, with the visible columns: for the CSV
/// and JSON downloads.
fn download(ui: &Ui) -> (Vec<&'static str>, Vec<File>) {
    let query = ui.table_query("files", &COLUMNS);
    (query.visible(&COLUMNS), files(&query))
}

/// The same rows as text/csv, for the sort, filter and columns in the URL.
async fn table_csv(ui: Ui) -> impl IntoResponse {
    let (columns, files) = download(&ui);
    let mut csv = columns.join(",") + "\n";
    for file in files {
        let cells = [
            ("name", file.0.clone()),
            ("size", file.1.to_string()),
            ("kind", file.2.to_string()),
        ];
        csv += &cells
            .iter()
            .filter(|(k, _)| columns.contains(k))
            .map(|(_, v)| v.as_str())
            .collect::<Vec<_>>()
            .join(",");
        csv.push('\n');
    }
    (
        [
            ("content-type", "text/csv; charset=utf-8"),
            ("content-disposition", "attachment; filename=\"files.csv\""),
        ],
        csv,
    )
}

/// One file in the JSON download; a hidden column is left out.
#[derive(Serialize)]
struct FileJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<&'static str>,
}

/// The same rows as JSON, for the sort, filter and columns in the URL.
async fn table_json(ui: Ui) -> impl IntoResponse {
    let (columns, files) = download(&ui);
    let shown = |key: &str| columns.contains(&key);
    let rows: Vec<FileJson> = files
        .into_iter()
        .map(|f| FileJson {
            name: shown("name").then_some(f.0),
            size: shown("size").then_some(f.1),
            kind: shown("kind").then_some(f.2),
        })
        .collect();
    (
        [("content-disposition", "attachment; filename=\"files.json\"")],
        Json(rows),
    )
}
/// A row edited in place: the file's new kind, saved, then back to the page it came from.
#[derive(Deserialize)]
struct EditedRow {
    key: String,
    kind: String,
    returns_to: String,
}

async fn table_edit(
    ui: Ui,
    Saved(mut kinds): Saved<Kinds>,
    Form(row): Form<EditedRow>,
) -> Redirect {
    let kind: String = row.kind.trim().chars().take(20).collect();
    kinds.0.retain(|(f, _)| *f != row.key);
    if !kind.is_empty() {
        kinds.0.push((row.key.clone(), kind));
    }
    // Only back to this page: `returns_to` is posted, so it is not trusted as a URL.
    let back = if row.returns_to.starts_with("/table") {
        &row.returns_to
    } else {
        "/table"
    };
    ui.redirect(back)
        .flash(&format!("Saved {}.", row.key))
        .save(&kinds)
}

/// `row=<key>` per ticked box and `action=<value>` from the button, or `all=1` for every file
/// the table's query matches (the form's URL carries it): acknowledged with a flash.
async fn table_bulk(ui: Ui, posted: Posted) -> Redirect {
    let rows = if posted.get("all") == "1" {
        files(&ui.table_query("files", &COLUMNS)).len()
    } else {
        posted.all("row").count()
    };
    let action = match posted.get("action") {
        "" => "delete",
        a => a,
    };
    let message = if rows == 0 {
        "Nothing selected: tick a row first.".to_string()
    } else {
        format!("{action}: {rows} file(s) (not really).")
    };
    ui.redirect("/table").flash(&message)
}
