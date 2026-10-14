//! A table's `<tbody>`: skeleton rows while loading, the empty message, group header rows,
//! and each row with its checkbox, cells (a detail block, the values of hidden columns, edit
//! boxes), edit buttons, menu and row link.

use maud::{Markup, html};

use super::{Ctx, Row, encode, prio};
use crate::button::Button;
use crate::i18n::Text;
use crate::input::Input;
use crate::popover::{Placement, menu};
use crate::{Cap, slug};

pub(super) fn body(cx: &Ctx) -> Markup {
    let v = cx.v;
    let vt = v
        .caps
        .has(Cap::ViewTransitions)
        .then(|| format!("view-transition-name: lui-table-{}", v.id));
    // The group of each row: its cell in the `group_by` column, compared as HTML.
    fn group<'r>(at: Option<usize>, row: &'r Row) -> Option<&'r str> {
        at.and_then(|i| row.cells.get(i)).map(|m| m.0.as_str())
    }
    html! {
        tbody style=[vt] aria-busy=[v.loading.then_some("true")] {
            @if v.loading {
                @for _ in 0..3 { tr class="lui-table-skeleton" { @for _ in 0..cx.span { td { span {} } } } }
            } @else if v.rows.is_empty() {
                tr { td colspan=(cx.span) class="lui-table-empty" { (v.empty) } }
            }
            @for (n, row) in v.rows.iter().enumerate().filter(|_| !v.loading) {
                @if let Some(heading) = group(v.group_by, row).filter(|g| n == 0 || v.rows.get(n - 1).and_then(|r| group(v.group_by, r)) != Some(*g)) {
                    tr class="lui-table-group" { td colspan=(cx.span) { (maud::PreEscaped(heading)) } }
                }
                (body_row(cx, row))
            }
        }
    }
}

fn body_row(cx: &Ctx, row: &Row) -> Markup {
    let v = cx.v;
    let editing = v.edit.is_some_and(|e| e.key.is_some() && e.key == row.key);
    let more = cx.visible.iter().any(|(_, c)| c.priority > 1);
    let link = row.href.filter(|_| !row.disabled);
    let class = [
        editing.then_some("lui-table-editing"),
        row.disabled.then_some("lui-table-disabled"),
        link.is_some().then_some("lui-table-linked"),
        row.tone,
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" ");
    html! {
        tr class=[(!class.is_empty()).then_some(class)] aria-disabled=[row.disabled.then_some("true")] {
            @if v.bulk.is_some() {
                td class="lui-table-select" {
                    @if let (Some(k), false) = (row.key, row.disabled) { input type="checkbox" class="lui-table-check" name="row" value=(k) form=(cx.bulk_id) aria-label=(v.strings.fill(Text::SelectRow, &[&k])); }
                }
            }
            @for (n, (i, col)) in cx.visible.iter().enumerate() {
                @let cell = row.cells.get(*i);
                @let class = [col.numeric.then_some("lui-table-num"), prio(col)].into_iter().flatten().collect::<Vec<_>>().join(" ");
                td class=[(!class.is_empty()).then_some(class)] {
                    @if editing && col.editable {
                        (Input::text_box(col.key, col.label, row.values.get(*i).copied().unwrap_or("")).form(cx.edit_id).class("lui-table-edit-input"))
                    } @else {
                    @match (n, &row.detail) {
                        (0, Some(detail)) => details class="lui-table-detail" {
                            summary { @if let Some(c) = cell { (c) } }
                            div class="lui-table-detail-body" { (detail) }
                        },
                        _ => @if let Some(c) = cell { (c) },
                    }
                    @if n == 0 && more {
                        dl class="lui-table-more" {
                            @for (j, c) in cx.visible.iter().filter(|(_, c)| c.priority > 1) {
                                div class={ "lui-table-more-p" (c.priority) } {
                                    dt { (c.label) } dd { @if let Some(v) = row.cells.get(*j) { (v) } }
                                }
                            }
                        }
                    }
                    }
                }
            }
            @if let Some(e) = v.edit {
                td class="lui-table-edit" {
                    @if editing {
                        (Button::new(v.caps, cx.t(Text::Save)).primary().small().form(cx.edit_id))
                        (Button::link(v.caps, cx.t(Text::Cancel), e.done).ghost().small())
                    } @else if let Some(k) = row.key {
                        @let href = format!("{}{}", e.link, encode(k));
                        (Button::link(v.caps, cx.t(Text::Edit), &href).ghost().small())
                    }
                }
            }
            @if cx.has_menu {
                td class="lui-table-menu" {
                    @if let (Some(k), false) = (row.key, row.menu.is_empty()) {
                        (menu(&v.caps, &format!("{}-{}", cx.root, slug(k)), cx.t(Text::RowActions), &row.menu, Placement::BottomEnd, true))
                    }
                }
            }
            @if cx.has_link {
                td class="lui-table-go" {
                    @if let Some(href) = link {
                        a class="lui-table-row-link" href=(href) aria-label=(v.strings.fill(Text::OpenRow, &[&row.key.unwrap_or(href)])) { "\u{203A}" }
                    }
                }
            }
        }
    }
}
