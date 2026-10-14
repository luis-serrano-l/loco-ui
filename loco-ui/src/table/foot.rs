//! What follows a table's rows: the forms its edit boxes and checkboxes belong to (a form
//! cannot wrap a row, so they sit after the table and the controls name them with `form=`).

use maud::{Markup, html};

use super::Ctx;
use crate::button::Button;
use crate::i18n::Text;

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
            form method="post" action=(action) id=(cx.bulk_id) class="lui-table-bulk" {
                span { (cx.t(Text::WithSelected)) }
                @for (value, label) in buttons { (Button::new(v.caps, label).small().name("action").value(value)) }
            }
        }
    }
}
