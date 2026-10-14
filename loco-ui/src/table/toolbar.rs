//! The bar above a table: the search form (box and `<select>`s), the "Columns" chooser and
//! the download link.

use maud::{Markup, html};

use super::{Column, Ctx, join};
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

pub(super) fn toolbar(cx: &Ctx) -> Markup {
    let (v, keys) = (cx.v, cx.keys);
    let filter_id = format!("{}-q", cx.root);
    // A `<select>` sends its own value, so the form carries everything else.
    let hidden = |k: &str| (v.search && k == keys.q) || v.selects.iter().any(|s| s.name == k);
    let filtering = v.search || !v.selects.is_empty();
    let toolbar = filtering || v.choose_columns || v.cols.is_some() || v.csv.is_some();
    let action = (!v.href.is_empty()).then_some(v.href);
    let shown = |c: &Column| cx.shown(c);
    html! {
        @if toolbar {
        div class="lui-table-toolbar" {
            @if filtering {
            search class="lui-table-filter" {
                form method="get" action=[action] {
                    @if let Some((key, desc)) = v.sort {
                        input type="hidden" name=(keys.sort) value=(key);
                        input type="hidden" name=(keys.dir) value=(if desc { "desc" } else { "asc" });
                    }
                    @for (k, val) in cx.carried { @if !hidden(k) { input type="hidden" name=(k) value=(val); } }
                    @if v.search {
                        (Input::search_box(&keys.q, cx.t(Text::FilterRows), v.filter).id(&filter_id).placeholder(cx.t(Text::FilterRowsHint)).autocomplete("off").class("lui-table-filter-input"))
                    }
                    @for s in v.selects {
                        label class="lui-table-filter-select" {
                            span class="lui-sr" { (s.label) }
                            select name=(s.name) {
                                @for (value, text) in &s.options { option value=(value) selected[*value == s.value] { (text) } }
                            }
                        }
                    }
                    (Button::new(v.caps, cx.t(Text::Filter)))
                    @if !v.filter.is_empty() {
                        a class="lui-table-clear" href=(cx.link(v.sort, &cx.carried.iter().copied().filter(|(k, _)| *k != keys.q).collect::<Vec<_>>())) { (cx.t(Text::Clear)) }
                    }
                }
            }
            }
            @if v.choose_columns || v.cols.is_some() {
                details class="lui-table-cols" {
                    summary class="lui-button" { (cx.t(Text::Columns)) (Icon::ChevronDown) }
                    ul {
                        @for c in v.columns {
                            @let on = shown(c);
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
        }
        }
    }
}
