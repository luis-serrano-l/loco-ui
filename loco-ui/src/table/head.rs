//! A table's `<colgroup>` and `<thead>`: widths, sort links (with the "then by" link and the
//! index of each sort key), the select-all box and the hidden names of the select, edit,
//! actions and link columns.

use maud::{Markup, html};

use super::{Ctx, prio};
use crate::i18n::Text;

/// The sort a click on the header of `key` asks for: flipped when it is the first key (the
/// others kept behind it), else that column alone, ascending.
fn header_sort<'s>(sort: &[(&'s str, bool)], key: &'s str) -> Vec<(&'s str, bool)> {
    match sort.first() {
        Some((first, desc)) if *first == key => std::iter::once((key, !desc))
            .chain(sort.iter().skip(1).copied())
            .collect(),
        _ => vec![(key, false)],
    }
}

/// The sort the "then by" link of `key` asks for: `key` added last, ascending, or flipped in
/// place when it is already a later key.
fn then_sort<'s>(sort: &[(&'s str, bool)], key: &'s str) -> Vec<(&'s str, bool)> {
    let mut next = sort.to_vec();
    match next.iter_mut().find(|(k, _)| *k == key) {
        Some(found) => found.1 = !found.1,
        None => next.push((key, false)),
    }
    next
}

pub(super) fn head(cx: &Ctx) -> Markup {
    let v = cx.v;
    let several = v.sort.len() > 1;
    html! {
        colgroup {
            @if v.bulk.is_some() { col class="lui-table-select-col"; }
            @for (_, c) in cx.visible { col class=[prio(c)] style=[c.width.map(|w| format!("width: {w}"))]; }
            @if v.edit.is_some() { col class="lui-table-edit-col"; }
            @if cx.has_menu { col class="lui-table-menu-col"; }
            @if cx.has_link { col class="lui-table-go-col"; }
        }
        thead { tr {
            @if v.bulk.is_some() {
                th scope="col" class="lui-table-select" {
                    @if v.select_all {
                        input type="checkbox" class="lui-table-all" name="all" value="1" form=(cx.bulk_id) aria-label=(v.strings.fill(Text::SelectAllMatching, &[&v.matching]));
                    } @else {
                        span class="lui-sr" { (cx.t(Text::Select)) }
                    }
                }
            }
            @for (_, col) in cx.visible {
                @let at = v.sort.iter().position(|(k, _)| *k == col.key);
                @let sorted = at.and_then(|i| v.sort.get(i)).copied();
                // aria-sort belongs on one header at a time (ARIA 1.2): the first key's.
                @let aria = sorted.filter(|_| at == Some(0)).map(|(_, d)| if d { "descending" } else { "ascending" });
                th scope="col" aria-sort=[aria] class={ @if sorted.is_some() { "lui-table-sorted" } @if col.numeric { " lui-table-num" } @if let Some(p) = prio(col) { " " (p) } } {
                    @if col.sortable {
                        a href=(cx.link(&header_sort(v.sort, col.key), cx.carried)) {
                            (col.label)
                            @match sorted { Some((_, true)) => span class="lui-table-arrow" { "▼" }, Some((_, false)) => span class="lui-table-arrow" { "▲" }, None => {} }
                            @if let (true, Some(i)) = (several, at) { span class="lui-table-sort-index" aria-hidden="true" { (i + 1) } }
                        }
                        @if !v.sort.is_empty() && at != Some(0) {
                            a class="lui-table-then" href=(cx.link(&then_sort(v.sort, col.key), cx.carried)) aria-label=(v.strings.fill(Text::SortThenBy, &[&col.label])) title=(v.strings.fill(Text::SortThenBy, &[&col.label])) { "+" }
                        }
                    } @else { (col.label) }
                }
            }
            @if v.edit.is_some() { th scope="col" class="lui-table-edit" { span class="lui-sr" { (cx.t(Text::Edit)) } } }
            @if cx.has_menu { th scope="col" class="lui-table-menu" { span class="lui-sr" { (cx.t(Text::Actions)) } } }
            @if cx.has_link { th scope="col" class="lui-table-go" { span class="lui-sr" { (cx.t(Text::Open)) } } }
        } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_header_flips_the_first_key_or_sorts_alone_and_then_by_adds_one() {
        let sort = [("name", false), ("size", true)];
        assert_eq!(header_sort(&sort, "name"), [("name", true), ("size", true)]);
        assert_eq!(header_sort(&sort, "size"), [("size", false)]);
        assert_eq!(header_sort(&[], "kind"), [("kind", false)]);
        assert_eq!(
            then_sort(&sort, "kind"),
            [("name", false), ("size", true), ("kind", false)]
        );
        assert_eq!(then_sort(&sort, "size"), [("name", false), ("size", false)]);
    }
}
