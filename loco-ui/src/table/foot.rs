//! What follows a table's rows: the `<tfoot>` (a totals row, then footer rows), and the forms
//! its edit boxes and checkboxes belong to (a form cannot wrap a row, so they sit after the
//! table and the controls name them with `form=`).

use maud::{Markup, html};

use super::{Ctx, join, prio};
use crate::button::Button;
use crate::i18n::Text;

pub(super) fn tfoot(cx: &Ctx) -> Markup {
    let v = cx.v;
    // The cells after the shown columns: edit, menu and link columns, empty.
    let trailing =
        usize::from(v.edit.is_some()) + usize::from(cx.has_menu) + usize::from(cx.has_link);
    let cells = |text: &dyn Fn(usize, &str) -> Option<Markup>| {
        html! {
            @if v.bulk.is_some() { td class="lui-table-select" {} }
            @for (i, col) in cx.visible {
                @let class = [col.numeric.then_some("lui-table-num"), prio(col)].into_iter().flatten().collect::<Vec<_>>().join(" ");
                td class=[(!class.is_empty()).then_some(class)] { @if let Some(t) = text(*i, col.key) { (t) } }
            }
            @for _ in 0..trailing { td {} }
        }
    };
    html! {
        @if !v.totals.is_empty() || !v.footer.is_empty() {
            tfoot {
                @if !v.totals.is_empty() {
                    tr class="lui-table-total" {
                        (cells(&|_, key| v.totals.iter().find(|(k, _)| *k == key).map(|(_, t)| html! { (t) })))
                    }
                }
                @for row in v.footer {
                    tr { (cells(&|i, _| row.cells.get(i).cloned())) }
                }
            }
        }
    }
}

pub(super) fn forms(cx: &Ctx) -> Markup {
    let v = cx.v;
    html! {
        @if let (Some(e), Some(key)) = (v.edit, v.edit.and_then(|e| e.key)) {
            form method="post" action=(e.action) id=(cx.edit_id) class="lui-table-edit-form" {
                input type="hidden" name="key" value=(key);
                input type="hidden" name="returns_to" value=(e.done);
            }
        }
        @if let Some((action, buttons)) = v.bulk {
            // With select-all the action carries the query, so `all=1` means this filter's rows.
            @let action = if v.select_all { join(action, &cx.query(v.sort, cx.carried)) } else { action.to_string() };
            form method="post" action=(action) id=(cx.bulk_id) class="lui-table-bulk" {
                span { (cx.t(Text::WithSelected)) }
                @for (value, label) in buttons { (Button::new(v.caps, label).small().name("action").value(value)) }
            }
        }
    }
}
