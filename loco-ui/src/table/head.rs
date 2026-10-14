//! A table's `<colgroup>` and `<thead>`: widths, sort links and the hidden names of the
//! select, edit and actions columns.

use maud::{Markup, html};

use super::{Ctx, prio};
use crate::i18n::Text;

pub(super) fn head(cx: &Ctx) -> Markup {
    let v = cx.v;
    html! {
        colgroup {
            @if v.bulk.is_some() { col class="lui-table-select-col"; }
            @for (_, c) in cx.visible { col class=[prio(c)] style=[c.width.map(|w| format!("width: {w}"))]; }
            @if v.edit.is_some() { col class="lui-table-edit-col"; }
            @if cx.has_menu { col class="lui-table-menu-col"; }
        }
        thead { tr {
            @if v.bulk.is_some() { th scope="col" class="lui-table-select" { span class="lui-sr" { (cx.t(Text::Select)) } } }
            @for (_, col) in cx.visible {
                @let sorted = v.sort.filter(|(k, _)| *k == col.key);
                @let aria = sorted.map(|(_, d)| if d { "descending" } else { "ascending" });
                th scope="col" aria-sort=[aria] class={ @if sorted.is_some() { "lui-table-sorted" } @if col.numeric { " lui-table-num" } @if let Some(p) = prio(col) { " " (p) } } {
                    @if col.sortable {
                        @let next_desc = matches!(sorted, Some((_, false)));
                        a href=(cx.link(Some((col.key, next_desc)), cx.carried)) {
                            (col.label)
                            @match sorted { Some((_, true)) => span class="lui-table-arrow" { "▼" }, Some((_, false)) => span class="lui-table-arrow" { "▲" }, None => {} }
                        }
                    } @else { (col.label) }
                }
            }
            @if v.edit.is_some() { th scope="col" class="lui-table-edit" { span class="lui-sr" { (cx.t(Text::Edit)) } } }
            @if cx.has_menu { th scope="col" class="lui-table-menu" { span class="lui-sr" { (cx.t(Text::Actions)) } } }
        } }
    }
}
