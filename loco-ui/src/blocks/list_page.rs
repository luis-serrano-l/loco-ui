//! # List page
//!
//! The index page of a resource: a title with its actions, a search box and filter links, how
//! many there are, the list itself (a table, cards, an empty state) and what goes under it
//! (the page links). The index page a scaffold writes, ready-made. After the shadcn/ui Tasks
//! example, built from Radix Themes parts (TextField, Button, Badge).
//!
//! **Platform features:** the search is a GET form in a `<search>` landmark
//! ([`crate::search`]: Chrome 118, Firefox 118, Safari 17); the filters are plain links, the
//! current one marked with `aria-current`; a container query (Chrome 105, Firefox 110,
//! Safari 16) puts the search and the filters on one row from 48rem.
//!
//! **Accessibility:** the title is the page's `<h1>`; the filters are a `<nav>` with a name;
//! the count is text, read in order before the list. Checked by axe-core with the demo routes.
//!
//! **What it does not do without script:** filter as you type; the search is a round trip.
//!
//! **Fallback:** a browser without `<search>` shows the form all the same, without the
//! landmark.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/customers", "status=open", "");
//! let m = ui.list_page("Customers").body(html! { ul { li { "Ada" } } }).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-list-page"><div class="lui-list-page-head">"#));
//! // With a search box, filters, a count and an action:
//! let m = ui.list_page("Customers").description("Everyone who bought something.")
//!     .search("Search customers")
//!     .filter("All", "/customers").filter("Open", "/customers?status=open").badge(3)
//!     .count("3 customers")
//!     .actions(html! { a href="/customers/new" { "New" } })
//!     .body(html! { ul { li { "Ada" } } })
//!     .render().into_string();
//! assert!(m.contains(r#"<a class="lui-list-page-filter" href="/customers?status=open" aria-current="page">"#));
//! assert!(m.contains(r#"<input type="hidden" name="status" value="open">"#));
//! // The same in `lui!`:
//! let same = lui! { ListPage("Customers") description="Everyone who bought something."
//!     search="Search customers" count="3 customers" actions={ a href="/customers/new" { "New" } } {
//!     filter "All" "/customers"; filter "Open" "/customers?status=open" badge=3;
//!     body { ul { li { "Ada" } } }
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use std::fmt::Display;

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};

/// A filter link: its label, where it goes and a count.
#[derive(Clone, Debug)]
struct Filter<'a> {
    label: &'a str,
    href: &'a str,
    badge: Option<String>,
}

/// A resource's index page, made by [`Ui::list_page`].
///
/// **Setters.** Values and items: `.description(..)`, `.actions(..)`, `.search(..)`,
/// `.filter(..)`, `.badge(..)`, `.count(..)`, `.body(..)`, `.footer(..)`.
#[derive(Clone, Debug)]
pub struct ListPage<'a> {
    ui: &'a Ui,
    title: &'a str,
    description: Option<&'a str>,
    actions: Markup,
    search: Option<&'a str>,
    filters: Vec<Filter<'a>>,
    count: Option<&'a str>,
    body: Markup,
    footer: Markup,
}

impl ListPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Enter",
            "In the search box, searches; on a filter, follows it.",
        ),
        Key::new(
            "Tab",
            "Moves through the actions, the search, the filters and the list.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("description", PropKind::Value, "text: &'a str").doc("A line under the title."),
        Prop::new("actions", PropKind::Value, "actions: Markup")
            .doc("What sits beside the title: a New button, an export link."),
        Prop::new("search", PropKind::Value, "label: &'a str").doc(
            "A search box named `q` that searches this page and keeps the current filter; `label` names it.",
        ),
        Prop::new("filter", PropKind::Item, "label: &'a str, href: &'a str").doc(
            "A filter link. The current one is the one whose query matches the page's.",
        ),
        Prop::new("badge", PropKind::Modifier, "text: impl Display")
            .applies_to(&["filter"])
            .doc("A count after the last filter's label."),
        Prop::new("count", PropKind::Value, "text: &'a str")
            .doc("How many there are, above the list (`\"42 customers\"`)."),
        Prop::new("body", PropKind::Value, "body: Markup")
            .doc("The list: a table, cards or an empty state."),
        Prop::new("footer", PropKind::Value, "footer: Markup")
            .doc("What goes under the list: the page links."),
    ];
}

impl Ui {
    /// The index page of the resource titled `title`.
    pub fn list_page<'a>(&'a self, title: &'a str) -> ListPage<'a> {
        ListPage {
            ui: self,
            title,
            description: None,
            actions: Markup::default(),
            search: None,
            filters: Vec::new(),
            count: None,
            body: Markup::default(),
            footer: Markup::default(),
        }
    }
}

impl<'a> ListPage<'a> {
    /// A line under the title.
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// What sits beside the title: a New button, an export link.
    pub fn actions(mut self, actions: Markup) -> Self {
        self.actions = actions;
        self
    }

    /// A search box named `q` that searches this page and keeps the current filter; `label`
    /// names it.
    pub fn search(mut self, label: &'a str) -> Self {
        self.search = Some(label);
        self
    }

    /// A filter link. The current one is the one whose query matches the page's.
    pub fn filter(mut self, label: &'a str, href: &'a str) -> Self {
        self.filters.push(Filter {
            label,
            href,
            badge: None,
        });
        self
    }

    /// A count after the last filter's label.
    #[track_caller]
    pub fn badge(mut self, text: impl Display) -> Self {
        crate::props::check_last::<Self>(
            ListPage::PROPS,
            "badge",
            self.filters.last().map(|_| "filter"),
        );
        if let Some(f) = self.filters.last_mut() {
            f.badge = Some(text.to_string());
        }
        self
    }

    /// How many there are, above the list (`"42 customers"`).
    pub fn count(mut self, text: &'a str) -> Self {
        self.count = Some(text);
        self
    }

    /// The list: a table, cards or an empty state.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }

    /// What goes under the list: the page links.
    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = footer;
        self
    }
}

/// The `key=value` pairs of an href's query.
fn pairs(href: &str) -> impl Iterator<Item = (&str, &str)> {
    let query = href.split_once('?').map_or("", |(_, q)| q);
    query
        .split('&')
        .filter(|p| !p.is_empty())
        .map(|p| p.split_once('=').unwrap_or((p, "")))
}

impl<'a> ListPage<'a> {
    /// Every query key a filter sets, once each.
    fn keys(&self) -> Vec<&'a str> {
        let mut keys: Vec<&'a str> = Vec::new();
        for (k, _) in self.filters.iter().flat_map(|f| pairs(f.href)) {
            if !keys.contains(&k) {
                keys.push(k);
            }
        }
        keys
    }

    /// Whether `filter` is the one shown: same path, and for every key a filter sets, the
    /// page's value is this filter's (or absent where it sets none).
    fn is_current(&self, filter: &Filter<'_>, keys: &[&str]) -> bool {
        let path = filter.href.split('?').next().unwrap_or_default();
        path == self.ui.state.path()
            && keys.iter().all(|k| {
                let here = self.ui.param(k).filter(|v| !v.is_empty());
                here == pairs(filter.href).find(|(key, _)| key == k).map(|(_, v)| v)
            })
    }
}

impl Render for ListPage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let keys = self.keys();
        let search = self.search.map(|label| {
            keys.iter()
                .fold(ui.search("q", label), |search, key| search.keep(key))
        });
        html! {
            div class="lui-list-page" {
                div class="lui-list-page-head" {
                    div class="lui-list-page-title" {
                        h1 { (self.title) }
                        @if let Some(d) = self.description { p class="lui-list-page-description" { (d) } }
                    }
                    @if !self.actions.0.is_empty() { div class="lui-list-page-actions" { (self.actions) } }
                }
                @if search.is_some() || !self.filters.is_empty() {
                    div class="lui-list-page-toolbar" {
                        @if let Some(s) = &search { (s) }
                        @if !self.filters.is_empty() {
                            nav class="lui-list-page-filters" aria-label=(ui.text(Text::Filters)) {
                                @for f in &self.filters {
                                    a class="lui-list-page-filter" href=(f.href) aria-current=[self.is_current(f, &keys).then_some("page")] {
                                        (f.label)
                                        @if let Some(b) = &f.badge { " " (ui.badge(b).secondary()) }
                                    }
                                }
                            }
                        }
                    }
                }
                @if let Some(c) = self.count { p class="lui-list-page-count" { (c) } }
                div class="lui-list-page-body" { (self.body) }
                @if !self.footer.0.is_empty() { div class="lui-list-page-footer" { (self.footer) } }
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. Narrow: the search above the
/// filters, which scroll sideways inside their row. From 48rem: both on one row.
pub const CSS: &str = r#"
.lui-list-page { container: lui-list-page / inline-size; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-4); }
.lui-list-page-head { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--lui-space-3); }
.lui-list-page-title { min-width: 0; }
.lui-list-page-title h1 { margin: 0; overflow-wrap: anywhere; }
.lui-list-page-description { margin: var(--lui-space-1) 0 0; color: var(--lui-muted); }
.lui-list-page-actions { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); }
.lui-list-page-toolbar { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-3); align-items: center; }
.lui-list-page-filters { display: flex; gap: var(--lui-space-1); overflow-x: auto; scrollbar-width: thin; }
.lui-list-page-filter { display: inline-flex; flex: none; align-items: center; gap: var(--lui-space-2); height: var(--lui-control-h); padding: 0 var(--lui-space-3); border-radius: var(--lui-radius); color: var(--lui-muted); font-size: var(--lui-text-2); font-weight: 500; text-decoration: none; white-space: nowrap; }
.lui-list-page-filter:hover { background: var(--lui-gray-3); color: var(--lui-fg); }
.lui-list-page-filter[aria-current="page"] { background: var(--lui-gray-4); color: var(--lui-fg); }
.lui-list-page-count { margin: 0; color: var(--lui-muted); font-size: var(--lui-text-2); }
.lui-list-page-body { min-width: 0; }
.lui-list-page-footer { display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: var(--lui-space-3); }
@media (pointer: coarse) { .lui-list-page-filter { min-height: var(--lui-hit); } }
@container lui-list-page (width >= 48rem) {
  .lui-list-page-toolbar:has(.lui-search) { grid-template-columns: minmax(0, 24rem) minmax(0, 1fr); }
  .lui-list-page-toolbar:has(.lui-search) .lui-list-page-filters { justify-content: flex-end; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_list_is_a_title_and_a_body_with_no_toolbar() {
        let ui = Ui::default();
        let m = ui
            .list_page("Customers")
            .body(html! { i { "rows" } })
            .render()
            .into_string();
        assert_eq!(
            m,
            r#"<div class="lui-list-page"><div class="lui-list-page-head"><div class="lui-list-page-title"><h1>Customers</h1></div></div><div class="lui-list-page-body"><i>rows</i></div></div>"#
        );
    }

    #[test]
    fn the_current_filter_is_the_one_whose_query_matches() {
        let page = |query| {
            let ui = Ui::from_request("/customers", query, "");
            ui.list_page("Customers")
                .filter("All", "/customers")
                .filter("Open", "/customers?status=open")
                .filter("Closed", "/customers?status=closed")
                .badge(2)
                .render()
                .into_string()
        };
        let current = |m: &str| {
            let at = m.find(r#" aria-current="page""#).expect("a current filter");
            assert_eq!(m.matches("aria-current").count(), 1, "{m}");
            m[..at]
                .rsplit("href=\"")
                .next()
                .unwrap_or_default()
                .to_string()
        };
        assert_eq!(current(&page("")), "/customers\"");
        assert_eq!(
            current(&page("status=open&q=ada")),
            "/customers?status=open\""
        );
        assert_eq!(
            current(&page("status=closed")),
            "/customers?status=closed\""
        );
        assert!(!page("status=other").contains("aria-current"));
        assert!(page("").contains(">2</span>"), "{}", page(""));
    }

    #[test]
    fn the_search_keeps_the_filter_and_shows_what_was_typed() {
        let ui = Ui::from_request("/customers", "status=open&q=ada", "");
        let m = ui
            .list_page("Customers")
            .search("Search customers")
            .filter("All", "/customers")
            .filter("Open", "/customers?status=open")
            .count("1 customer")
            .footer(html! { b { "pages" } })
            .render()
            .into_string();
        assert!(
            m.contains(r#"<form method="get" action="/customers">"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<input type="hidden" name="status" value="open">"#),
            "{m}"
        );
        assert!(m.contains(r#"value="ada""#), "{m}");
        assert!(
            m.contains(r#"<p class="lui-list-page-count">1 customer</p>"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<div class="lui-list-page-footer"><b>pages</b></div>"#),
            "{m}"
        );
        assert!(m.find("lui-search").unwrap_or(0) < m.find("lui-list-page-filters").unwrap_or(0));
    }
}
