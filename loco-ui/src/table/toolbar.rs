//! The bar above a table: the search form (box, the "Filters" panel of column filters and
//! `<select>`s), the "Columns" chooser and the download links.

use maud::{Markup, html};

use super::{Column, Ctx, FilterSelect, join};
use crate::Icon;
use crate::button::Button;
use crate::i18n::Text;
use crate::input::Input;

impl Ctx<'_, '_> {
    /// The "Columns" link that shows or hides `key`, keeping the others in table order.
    fn cols_link(&self, key: &str) -> String {
        let (v, keys) = (self.v, self.keys);
        let current: Vec<&str> = v
            .cols
            .map(|c| c.to_vec())
            .unwrap_or_else(|| v.columns.iter().map(|c| c.key).collect());
        let next: Vec<&str> = if current.contains(&key) {
            current.iter().copied().filter(|k| *k != key).collect()
        } else {
            v.columns
                .iter()
                .map(|c| c.key)
                .filter(|k| current.contains(k) || *k == key)
                .collect()
        };
        let joined = next.join(",");
        let mut pairs: Vec<(&str, &str)> = self
            .carried
            .iter()
            .copied()
            .filter(|(k, _)| *k != keys.cols)
            .collect();
        pairs.push((&keys.cols, &joined));
        self.link(v.sort, &pairs)
    }
}

fn select(s: &FilterSelect) -> Markup {
    html! {
        label class="lui-table-filter-select" {
            span class="lui-sr" { (s.label) }
            select name=(s.name) {
                @for (value, text) in &s.options { option value=(value) selected[*value == s.value] { (text) } }
            }
        }
    }
}

pub(super) fn toolbar(cx: &Ctx) -> Markup {
    let (v, keys) = (cx.v, cx.keys);
    let filter_id = format!("{}-q", cx.root);
    let filterable: Vec<&Column> = cx
        .visible
        .iter()
        .map(|(_, c)| *c)
        .filter(|c| c.filterable)
        .collect();
    let column_key = |c: &Column| keys.filter(c.key);
    // A `<select>` and a column filter send their own value, so the form carries the rest.
    let own = |k: &str| {
        (v.search && k == keys.q)
            || v.selects.iter().any(|s| s.name == k)
            || filterable.iter().any(|c| column_key(c) == k)
    };
    let filtering = v.search || !v.selects.is_empty() || !filterable.is_empty();
    let toolbar =
        filtering || v.choose_columns || v.cols.is_some() || v.csv.is_some() || v.json.is_some();
    let action = (!v.href.is_empty()).then_some(v.href);
    let filtered = !v.filter.is_empty() || !v.filters.is_empty();
    let unfiltered: Vec<(&str, &str)> = (cx.carried.iter().copied())
        .filter(|(k, _)| *k != keys.q && !k.starts_with(&format!("{}.", keys.q)))
        .collect();
    html! {
        @if toolbar {
        div class="lui-table-toolbar" {
            @if filtering {
            search class="lui-table-filter" {
                form method="get" action=[action] {
                    @if !v.sort.is_empty() {
                        input type="hidden" name=(keys.sort) value=(super::sort_text(v.sort));
                    }
                    @for (k, val) in cx.carried { @if !own(k) { input type="hidden" name=(k) value=(val); } }
                    @if v.search {
                        (Input::search_box(&keys.q, cx.t(Text::FilterRows), v.filter).id(&filter_id).placeholder(cx.t(Text::FilterRowsHint)).autocomplete("off").class("lui-table-filter-input"))
                    }
                    @if filterable.is_empty() {
                        @for s in v.selects { (select(s)) }
                    } @else {
                        details class="lui-table-filters" open[!v.filters.is_empty()] {
                            summary class="lui-button" {
                                (cx.t(Text::Filters))
                                @if !v.filters.is_empty() { span class="lui-table-filters-count" { (v.filters.len()) } }
                                (Icon::ChevronDown)
                            }
                            div class="lui-table-filters-body" {
                                @for c in &filterable {
                                    @let key = column_key(c);
                                    @let value = v.filters.iter().find(|(k, _)| *k == c.key).map_or("", |(_, t)| *t);
                                    (Input::text_box(&key, c.label, value).id(&format!("{}-{}", filter_id, c.key)).placeholder(c.label).autocomplete("off").class("lui-table-filters-input"))
                                }
                                @for s in v.selects { (select(s)) }
                            }
                        }
                    }
                    (Button::new(v.caps, cx.t(Text::Filter)))
                    @if filtered {
                        a class="lui-table-clear" href=(cx.link(v.sort, &unfiltered)) { (cx.t(Text::Clear)) }
                    }
                }
            }
            }
            @if v.choose_columns || v.cols.is_some() {
                details class="lui-table-cols" {
                    summary class="lui-button" { (cx.t(Text::Columns)) (Icon::ChevronDown) }
                    ul {
                        @for c in v.columns {
                            @let on = cx.shown(c);
                            li { a href=(cx.cols_link(c.key)) aria-pressed=(on) {
                                span class="lui-table-cols-mark" aria-hidden="true" { @if on { "\u{2611}" } @else { "\u{2610}" } } " " (c.label)
                            } }
                        }
                    }
                }
            }
            @if let Some(base) = v.csv {
                a class="lui-button lui-button-small lui-table-csv" href=(join(base, &cx.query(v.sort, cx.carried))) download { (Icon::Download) (cx.t(Text::DownloadCsv)) }
            }
            @if let Some(base) = v.json {
                a class="lui-button lui-button-small lui-table-json" href=(join(base, &cx.query(v.sort, cx.carried))) download { (Icon::Download) (cx.t(Text::DownloadJson)) }
            }
        }
        }
    }
}
