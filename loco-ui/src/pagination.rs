//! # Pagination
//!
//! Numbered page links for a list too long for one page: previous and next, the first and
//! last pages, the current page's neighbours and an ellipsis over each gap. The page shown is
//! the request's `?page=` (or `?page.<id>=` once given an id), so every page is a URL that
//! can be bookmarked, and the links keep whatever else the caller names. After shadcn/ui
//! Pagination.
//!
//! **Platform features:** a `<nav>` of ordinary links (baseline since HTML 2), the current
//! page marked `aria-current="page"`, the previous and next links `rel="prev"` and
//! `rel="next"`; [`Pagination::link_rels`] puts the same two in the page's `<head>` as
//! `<link rel>`. A container query (`@container`, Chrome 105, Firefox 110, Safari 16) shows
//! every number from 30rem; narrower, only the first, the current and the last stay, and
//! Previous and Next keep their icons.
//!
//! **Accessibility:** a `<nav>` named "Pages" around a list of links; the current page has
//! `aria-current="page"`; the ellipses are `aria-hidden`; at either end Previous or Next is a
//! disabled link (`aria-disabled`, no `href`) so the row keeps its shape. Checked by axe-core
//! in headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing: each page is a navigation.
//!
//! **Fallback:** none needed. Every control is a link.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // Page 5 of 20 (200 items, 10 a page).
//! let ui = Ui::from_request("/posts", "page=5", "");
//! let m = ui.pagination("/posts").total(200);
//! assert_eq!((m.current(), m.pages(), m.range()), (5, 20, 40..50));
//! let html = m.render().into_string();
//! assert!(html.contains(r#"href="/posts?page=5" aria-current="page""#));
//! assert!(html.contains(r#"href="/posts?page=4" rel="prev""#) && html.contains(r#"href="/posts?page=6" rel="next""#));
//! // An id of its own, 25 a page, two neighbours each side, first and last links, and the
//! // filter carried along.
//! let ui = Ui::from_request("/posts", "page.posts=3&q=rust", "");
//! let m = ui.pagination("/posts").id("posts").total(400).per_page(25).window(2).first_last().param("q", "rust");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"href="/posts?q=rust&amp;page.posts=16">"#));
//! // The same in `lui!`:
//! let same = lui! { Pagination("/posts") id="posts" total=400 per_page=25 window=2 first_last param=("q", "rust"); };
//! assert_eq!(same.into_string(), html);
//! ```

use std::ops::Range;

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::{Icon, Page, Ui};

/// Numbered page links, made by [`Ui::pagination`]: 10 items a page, one neighbour each side
/// of the current page unless told otherwise.
///
/// **Setters.** Values and items: `.id(..)`, `.page(..)`, `.total(..)`, `.per_page(..)`,
/// `.window(..)`, `.param(..)`; switches: `.first_last()`.
#[derive(Clone, Debug)]
pub struct Pagination<'a> {
    ui: &'a Ui,
    href: &'a str,
    id: Option<&'a str>,
    page: Option<usize>,
    total: usize,
    per_page: usize,
    window: usize,
    keep: Vec<(&'a str, &'a str)>,
    first_last: bool,
}

impl Pagination<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("id", PropKind::Value, "id: &'a str")
            .doc("Read and write `?page.<id>=` instead of `?page=`, so two lists on one page page on their own."),
        Prop::new("page", PropKind::Number, "page: usize")
            .doc("Show page `page` (1-based) whatever the request says."),
        Prop::new("total", PropKind::Number, "total: usize")
            .default("0")
            .doc("How many items the list holds."),
        Prop::new("per_page", PropKind::Number, "per_page: usize")
            .default("10")
            .doc("Items a page."),
        Prop::new("window", PropKind::Number, "neighbours: usize")
            .default("1")
            .doc("How many pages to number on each side of the current one."),
        Prop::new("param", PropKind::Item, "key: &'a str, value: &'a str")
            .doc("A query pair every link carries (a filter, a sort)."),
        Prop::new("first_last", PropKind::Switch, "")
            .doc("\"First\" and \"Last\" links outside Previous and Next."),
    ];
}

impl Ui {
    /// Page links to `href?page=n`; the page shown is this request's `?page=`.
    pub fn pagination<'a>(&'a self, href: &'a str) -> Pagination<'a> {
        Pagination {
            ui: self,
            href,
            id: None,
            page: None,
            total: 0,
            per_page: 10,
            window: 1,
            keep: Vec::new(),
            first_last: false,
        }
    }
}

impl<'a> Pagination<'a> {
    /// Read and write `?page.<id>=` instead of `?page=`, so two lists on one page page on
    /// their own (the key a paged table uses).
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Show page `page` (1-based) whatever the request says.
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// How many items the list holds.
    pub fn total(mut self, total: usize) -> Self {
        self.total = total;
        self
    }

    /// Items a page.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = per_page.max(1);
        self
    }

    /// How many pages to number on each side of the current one; the first and last are
    /// always numbered.
    pub fn window(mut self, neighbours: usize) -> Self {
        self.window = neighbours;
        self
    }

    /// A query pair every link carries (a filter, a sort), before the page key; one call
    /// per pair.
    pub fn param(mut self, key: &'a str, value: &'a str) -> Self {
        self.keep.push((key, value));
        self
    }

    /// "First" and "Last" links outside Previous and Next.
    pub fn first_last(mut self) -> Self {
        self.first_last = true;
        self
    }

    /// The query key the page travels in: `page`, or `page.<id>`.
    pub fn key(&self) -> String {
        self.id
            .map_or_else(|| "page".to_string(), |id| format!("page.{id}"))
    }

    /// How many pages there are; at least one.
    pub fn pages(&self) -> usize {
        self.total.div_ceil(self.per_page).max(1)
    }

    /// The page shown, 1-based, within `1..=pages()`.
    pub fn current(&self) -> usize {
        self.page
            .or_else(|| self.ui.param(&self.key()).and_then(|p| p.parse().ok()))
            .unwrap_or(1)
            .clamp(1, self.pages())
    }

    /// The items the current page shows, 0-based: slice the list with it.
    pub fn range(&self) -> Range<usize> {
        let start = (self.current() - 1) * self.per_page;
        start.min(self.total)..(start + self.per_page).min(self.total)
    }

    /// The link to page `n`, keeping the [`Pagination::param`] pairs.
    pub fn link(&self, n: usize) -> String {
        let key = self.key();
        let n = n.to_string();
        crate::href(
            self.href,
            self.keep
                .iter()
                .copied()
                .chain([(key.as_str(), n.as_str())]),
        )
    }

    /// `page` with `<link rel="prev">` and `<link rel="next">` in its head for the pages
    /// before and after this one (what search engines and reader modes follow).
    pub fn link_rels(&self, page: Page) -> Page {
        let (current, pages) = (self.current(), self.pages());
        let page = if current > 1 {
            page.link_rel("prev", &self.link(current - 1))
        } else {
            page
        };
        if current < pages {
            page.link_rel("next", &self.link(current + 1))
        } else {
            page
        }
    }
}

/// The numbered slots for page `page` of `pages` with `w` neighbours each side: every page
/// when they fit in `2w + 5` slots, else the first, the last and the window, with `None` for
/// each gap. Near an end the window grows so the row keeps its length.
pub(crate) fn slots(page: usize, pages: usize, w: usize) -> Vec<Option<usize>> {
    let len = 2 * w + 5;
    if pages <= len {
        return (1..=pages).map(Some).collect();
    }
    let (lo, hi) = if page <= w + 3 {
        (2, 2 * w + 3)
    } else if page + w + 2 >= pages {
        (pages - 2 * w - 2, pages - 1)
    } else {
        (page - w, page + w)
    };
    let mut out = vec![Some(1)];
    if lo > 2 {
        out.push(None);
    }
    out.extend((lo..=hi).map(Some));
    if hi < pages - 1 {
        out.push(None);
    }
    out.push(Some(pages));
    out
}

impl Render for Pagination<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let (page, pages) = (self.current(), self.pages());
        let slots = slots(page, pages, self.window);
        // In a narrow box only the first, the current and the last page stay numbered; a gap
        // that only the narrow row has gets its own ellipsis.
        let narrow = |n: usize| n == 1 || n == page || n == pages;
        let edge = |n: usize, text: Text, off: bool| {
            let href = self.link(n);
            let b = ui
                .link_button(ui.text(text), &href)
                .ghost()
                .class("lui-pagination-link lui-pagination-step");
            let b = if off { b.disabled() } else { b };
            html! { li class="lui-pagination-wide" { (b) } }
        };
        let prev = ui.text(Text::Previous);
        let next = ui.text(Text::Next);
        html! {
            nav class="lui-pagination" aria-label=(ui.text(Text::Pages)) {
                ul {
                    @if self.first_last { (edge(1, Text::First, page == 1)) }
                    li {
                        @let href = self.link(page.saturating_sub(1).max(1));
                        @let b = ui.link_button(prev, &href).ghost().class("lui-pagination-link lui-pagination-step").rel("prev")
                            .body(html! { (Icon::ChevronLeft) span class="lui-pagination-label" { (prev) } });
                        @if page == 1 { (b.disabled()) } @else { (b) }
                    }
                    @for (i, slot) in slots.iter().enumerate() {
                        @match slot {
                            Some(n) => {
                                @let n = *n;
                                li class={ "lui-pagination-num" @if !narrow(n) { " lui-pagination-wide" } } {
                                    @let href = self.link(n);
                                    @let b = ui.link_button("", &href).class("lui-pagination-link").current(n == page).body(html! { (ui.format_number(n as f64)) });
                                    @if n == page { (b) } @else { (b.ghost()) }
                                }
                                // A narrow-only gap: the next slot shown when narrow is not n + 1
                                // and the wide row has no ellipsis between them.
                                @if narrow(n) && n < pages {
                                    @let rest = &slots[i + 1..];
                                    @let next_narrow = rest.iter().position(|s| matches!(s, Some(m) if narrow(*m)));
                                    @if let Some(j) = next_narrow {
                                        @if !rest[..j].contains(&None) && rest[..j].iter().any(Option::is_some) {
                                            li class="lui-pagination-gap lui-pagination-narrow" aria-hidden="true" { "\u{2026}" }
                                        }
                                    }
                                }
                            },
                            None => li class="lui-pagination-gap" aria-hidden="true" { "\u{2026}" },
                        }
                    }
                    li {
                        @let href = self.link((page + 1).min(pages));
                        @let b = ui.link_button(next, &href).ghost().class("lui-pagination-link lui-pagination-step").rel("next")
                            .body(html! { span class="lui-pagination-label" { (next) } (Icon::ChevronRight) });
                        @if page == pages { (b.disabled()) } @else { (b) }
                    }
                    @if self.first_last { (edge(pages, Text::Last, page == pages)) }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After shadcn/ui Pagination: a centred row of 2.25rem ghost links 0.25rem apart, the
   current page in the outline button, Previous and Next with a chevron, ellipses in a
   2.25rem square. Its own container: under 30rem the neighbours and the labels go, so
   the row is Previous, first, current, last, Next and fits 320px. */
.lui-pagination { container: lui-pagination / inline-size; }
.lui-pagination > ul { display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: 0.25rem; list-style: none; margin: 0; padding: 0; }
.lui-pagination > ul > li { display: flex; }
.lui-pagination-link { min-width: var(--lui-control-h); padding-inline: 0.5rem; font-variant-numeric: tabular-nums; }
.lui-pagination-step { gap: 0.25rem; }
.lui-pagination-step[aria-disabled] { opacity: 0.5; cursor: default; }
.lui-pagination-gap { align-items: center; justify-content: center; width: var(--lui-control-h); height: var(--lui-control-h); color: var(--lui-muted); }
.lui-pagination > ul > .lui-pagination-wide { display: none; }
.lui-pagination-label { position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
@container lui-pagination (width >= 30rem) {
  .lui-pagination > ul > .lui-pagination-wide { display: flex; }
  .lui-pagination > ul > .lui-pagination-narrow { display: none; }
  .lui-pagination-label { position: static; width: auto; height: auto; overflow: visible; clip-path: none; }
}
@media (pointer: coarse) { .lui-pagination-link { min-width: var(--lui-hit); min-height: var(--lui-hit); } }
"#;

#[cfg(test)]
mod tests {
    use super::slots;
    use crate::Ui;
    use maud::Render;

    #[test]
    fn slots_keep_their_length_and_match_the_table() {
        assert_eq!(
            slots(1, 5, 1),
            [Some(1), Some(2), Some(3), Some(4), Some(5)]
        );
        assert_eq!(
            slots(1, 20, 1),
            [Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(20)]
        );
        assert_eq!(
            slots(10, 20, 1),
            [Some(1), None, Some(9), Some(10), Some(11), None, Some(20)]
        );
        assert_eq!(
            slots(19, 20, 1),
            [
                Some(1),
                None,
                Some(16),
                Some(17),
                Some(18),
                Some(19),
                Some(20)
            ]
        );
        assert_eq!(slots(10, 20, 2).len(), 9);
        assert_eq!(slots(1, 1, 2), [Some(1)]);
    }

    #[test]
    fn ends_disable_their_step_and_narrow_gaps_appear() {
        let ui = Ui::from_request("/p", "", "");
        let m = ui.pagination("/p").total(60).render().into_string();
        assert!(m.contains(r#"rel="prev""#) && m.contains(r#"role="link" aria-disabled="true""#));
        // Pages 1 to 6 all fit; narrow shows 1 … 6 with an ellipsis of its own.
        assert!(m.contains("lui-pagination-narrow"), "{m}");
        let last = Ui::from_request("/p", "page=99", "");
        assert_eq!(last.pagination("/p").total(60).current(), 6);
        assert_eq!(ui.pagination("/p").range(), 0..0);
    }
}
