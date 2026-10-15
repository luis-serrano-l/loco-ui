//! # Carousel
//!
//! Slides side by side in a row that scrolls sideways and snaps to each one, with previous
//! and next buttons and a dot per slide. After shadcn/ui Carousel: full-width slides, round
//! outline buttons beside them, small dots under them; rebuilt on CSS scroll snapping instead
//! of a script.
//!
//! **Platform features:**
//! - `scroll-snap-type` (Chrome 69, Firefox 68, Safari 11) and `scroll-behavior: smooth` on
//!   a scroll container that takes the keyboard (`tabindex="0"`): swipe, trackpad, arrow keys
//!   and Shift+wheel all move it a slide at a time.
//! - Where the browser has them, `::scroll-button()` (Chrome 135; not yet in Firefox or
//!   Safari) draws the previous and next buttons and `::scroll-marker` with
//!   `scroll-marker-group` (Chrome 135) the dots, both under `@supports`; their names come
//!   from `data-prev` and `data-next` on the scroller through `attr()`, so they are translated.
//!   Spec: <https://drafts.csswg.org/css-overflow-5/>.
//! - Elsewhere, plain links: each slide has an `id` (`<id>-<n>`) and carries a previous and
//!   a next link to its neighbours' fragments, and a row of links under the row is the dots.
//!
//! **Accessibility:** a `<section>` with `aria-roledescription="carousel"` named by its
//! label; each slide a `role="group"` "slide" named "Slide n of N". The fallback links are
//! named the same way. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** play on its own, loop from the last slide to the
//! first, or mark the current dot in the link fallback (the `::scroll-marker` dots show it
//! with `:target-current`); a fallback link scrolls the page to the row if it is out of view.
//!
//! **Fallback:** without `::scroll-button()` and `::scroll-marker` (Firefox, Safari today),
//! the fragment links above; without scroll snapping, a row that scrolls freely.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.carousel("Photos").item(html! { "One" }).item(html! { "Two" }).render().into_string();
//! assert!(m.starts_with(r#"<section class="lui-carousel" aria-roledescription="carousel" aria-label="Photos">"#));
//! assert!(m.contains(r#"id="photos-2" class="lui-carousel-item" role="group" aria-roledescription="slide" aria-label="Slide 2 of 2""#));
//! assert!(m.contains(r##"href="#photos-1""##) && m.contains(r##"href="#photos-2""##));
//! // The same in `lui!`:
//! let same = lui! { Carousel("Photos") { item (html! { "One" }); item (html! { "Two" }); } };
//! assert_eq!(same.into_string(), m);
//! let m = ui.carousel("Photos").id("trip").item(html! { "One" }).render().into_string();
//! assert!(m.contains(r#"id="trip-1""#));
//! assert_eq!(lui! { Carousel("Photos") id="trip" { item (html! { "One" }); } }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::{Icon, Ui, slug};

/// Slides in a snapping row, made by [`Ui::carousel`].
///
/// **Setters.** Values and items: `.item(..)`, `.id(..)`.
#[derive(Clone, Debug)]
pub struct Carousel<'a> {
    ui: &'a Ui,
    label: &'a str,
    id: Option<&'a str>,
    items: Vec<Markup>,
}

impl Carousel<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves focus to the slide strip, then to the slide buttons and the dots.",
        ),
        Key::new(
            "ArrowLeft / ArrowRight",
            "Scrolls the focused strip to the previous or next slide.",
        ),
        Key::new(
            "Enter",
            "Goes to the slide the focused button or dot names.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("item", PropKind::Item, "markup: Markup").doc("A slide."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .doc("The prefix of the slides' ids (`<id>-<n>`) instead of the label's slug."),
    ];
}

impl Ui {
    /// A carousel named `label` (what a screen reader says it is: "Photos", "Plans").
    pub fn carousel<'a>(&'a self, label: &'a str) -> Carousel<'a> {
        Carousel {
            ui: self,
            label,
            id: None,
            items: Vec::new(),
        }
    }
}

impl<'a> Carousel<'a> {
    /// A slide.
    pub fn item(mut self, markup: Markup) -> Self {
        self.items.push(markup);
        self
    }

    /// The prefix of the slides' ids (`<id>-<n>`) instead of the label's slug; it must be
    /// unique on the page.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Render for Carousel<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let id = self.id.map_or_else(|| slug(self.label), str::to_string);
        let n = self.items.len();
        let slide = |i: usize| ui.fill(Text::SlideOf, &[&i, &n]);
        let to = |i: usize| format!("#{id}-{i}");
        html! {
            section class="lui-carousel" aria-roledescription=(ui.text(Text::Carousel)) aria-label=(self.label) {
                div class="lui-carousel-viewport" tabindex="0" data-prev=(ui.text(Text::PreviousSlide)) data-next=(ui.text(Text::NextSlide)) {
                    @for (i, item) in self.items.iter().enumerate().map(|(i, m)| (i + 1, m)) {
                        div id={ (id) "-" (i) } class="lui-carousel-item" role="group" aria-roledescription=(ui.text(Text::Slide)) aria-label=(slide(i)) {
                            (item)
                            @if i > 1 {
                                (ui.link_button("", &to(i - 1)).icon_only().class("lui-carousel-prev").aria_label(ui.text(Text::PreviousSlide)).body(html! { (Icon::ChevronLeft) }))
                            }
                            @if i < n {
                                (ui.link_button("", &to(i + 1)).icon_only().class("lui-carousel-next").aria_label(ui.text(Text::NextSlide)).body(html! { (Icon::ChevronRight) }))
                            }
                        }
                    }
                }
                @if n > 1 {
                    nav class="lui-carousel-dots" aria-label=(self.label) {
                        @for i in 1..=n { a class="lui-carousel-dot" href=(to(i)) aria-label=(slide(i)) {} }
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After shadcn/ui Carousel: full-width slides, 2rem round outline buttons over the edges, and
   0.5rem dots under the row (the current one in --lui-fg). */
.lui-carousel { container: lui-carousel / inline-size; position: relative; display: grid; gap: var(--lui-space-3); min-width: 0; }
.lui-carousel-viewport {
  display: grid; grid-auto-flow: column; grid-auto-columns: 100%; gap: var(--lui-space-4);
  overflow-x: auto; overscroll-behavior-x: contain; scroll-snap-type: x mandatory; scroll-behavior: smooth;
  scrollbar-width: none; border-radius: var(--lui-radius-lg);
}
.lui-carousel-viewport::-webkit-scrollbar { display: none; }
.lui-carousel-viewport:focus-visible { outline: 2px solid var(--lui-ring); outline-offset: 2px; }
.lui-carousel-item { position: relative; scroll-snap-align: start; scroll-margin-block: var(--lui-space-8); min-width: 0; }
/* The link fallback: previous and next over the slide's edges, the dots under the row. */
.lui-carousel-prev, .lui-carousel-next { position: absolute; top: 50%; translate: 0 -50%; border-radius: 9999px; background: var(--lui-bg); }
.lui-carousel-prev { inset-inline-start: var(--lui-space-2); }
.lui-carousel-next { inset-inline-end: var(--lui-space-2); }
/* From 30rem the buttons sit further in from the edges. */
@container lui-carousel (width >= 30rem) { .lui-carousel-prev { inset-inline-start: var(--lui-space-4); } .lui-carousel-next { inset-inline-end: var(--lui-space-4); } }
.lui-carousel-dots { display: flex; flex-wrap: wrap; justify-content: center; gap: var(--lui-space-2); }
.lui-carousel-dot { position: relative; width: 0.5rem; height: 0.5rem; border-radius: 9999px; background: var(--lui-gray-6); }
.lui-carousel-dot::after { content: ""; position: absolute; inset: -0.5rem; }
.lui-carousel-dot:hover { background: var(--lui-gray-9); }
@media (pointer: coarse) { .lui-carousel-dot::after { inset: calc((0.5rem - var(--lui-hit)) / 2); } }
@media (prefers-reduced-motion: reduce) { .lui-carousel-viewport { scroll-behavior: auto; } }
/* Chromium's scroll buttons and markers replace the links where they exist. */
@supports selector(::scroll-button(*)) {
  .lui-carousel-prev, .lui-carousel-next { display: none; }
  .lui-carousel-viewport { anchor-name: --lui-carousel; }
  .lui-carousel-viewport::scroll-button(*) {
    position: absolute; position-anchor: --lui-carousel; top: anchor(center); translate: 0 -50%;
    display: grid; place-items: center; width: 2rem; height: 2rem; border-radius: 9999px; font-size: var(--lui-text-3); line-height: 1;
    color: var(--lui-fg); background: var(--lui-bg); border: 1px solid var(--lui-line); box-shadow: var(--lui-shadow-sm); cursor: pointer;
  }
  .lui-carousel-viewport::scroll-button(*):disabled { opacity: 0.5; cursor: default; }
  .lui-carousel-viewport::scroll-button(inline-start) { content: "\2039" / attr(data-prev); inset-inline-start: calc(anchor(start) + 0.5rem); }
  .lui-carousel-viewport::scroll-button(inline-end) { content: "\203A" / attr(data-next); inset-inline-end: calc(anchor(end) + 0.5rem); }
  @media (pointer: coarse) { .lui-carousel-viewport::scroll-button(*) { width: var(--lui-hit); height: var(--lui-hit); } }
}
@supports selector(::scroll-marker) {
  .lui-carousel-dots { display: none; }
  .lui-carousel-viewport { scroll-marker-group: after; }
  .lui-carousel-viewport::scroll-marker-group { display: flex; justify-content: center; gap: var(--lui-space-2); padding-top: var(--lui-space-3); }
  .lui-carousel-item::scroll-marker { content: "" / attr(aria-label); width: 0.5rem; height: 0.5rem; border-radius: 9999px; background: var(--lui-gray-6); }
  .lui-carousel-item::scroll-marker:target-current { background: var(--lui-fg); }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn three() -> String {
        Ui::default()
            .carousel("Photos")
            .item(html! { "One" })
            .item(html! { "Two" })
            .item(html! { "Three" })
            .render()
            .into_string()
    }

    #[test]
    fn the_root_is_a_labelled_carousel_section_with_a_focusable_viewport() {
        let m = three();
        assert!(m.starts_with(r#"<section class="lui-carousel" aria-roledescription="carousel" aria-label="Photos">"#), "{m}");
        assert!(
            m.contains(r#"class="lui-carousel-viewport" tabindex="0""#),
            "{m}"
        );
        assert!(m.contains("data-prev=") && m.contains("data-next="), "{m}");
    }

    #[test]
    fn every_slide_has_an_id_from_the_label_and_a_group_name_with_its_position() {
        let m = three();
        for i in 1..=3 {
            assert!(
                m.contains(&format!(r#"id="photos-{i}" class="lui-carousel-item""#)),
                "{m}"
            );
            assert!(
                m.contains(&format!(r#"aria-label="Slide {i} of 3""#)),
                "{m}"
            );
        }
    }

    #[test]
    fn the_first_slide_has_no_previous_link_and_the_last_has_no_next() {
        let m = three();
        assert_eq!(m.matches("lui-carousel-prev").count(), 2, "{m}");
        assert_eq!(m.matches("lui-carousel-next").count(), 2, "{m}");
        let first = &m[m.find(r#"id="photos-1""#).unwrap()..m.find(r#"id="photos-2""#).unwrap()];
        assert!(
            !first.contains("lui-carousel-prev") && first.contains(r##"href="#photos-2""##),
            "{first}"
        );
        let last = &m[m.find(r#"id="photos-3""#).unwrap()..m.find("lui-carousel-dots").unwrap()];
        assert!(
            !last.contains("lui-carousel-next") && last.contains(r##"href="#photos-2""##),
            "{last}"
        );
        assert_eq!(m.matches("lui-carousel-dot\"").count(), 3, "{m}");
    }

    #[test]
    fn id_overrides_the_slug_and_a_single_slide_has_no_links_or_dots() {
        let m = Ui::default()
            .carousel("Photos")
            .id("trip")
            .item(html! { "Only" })
            .render()
            .into_string();
        assert!(
            m.contains(r#"id="trip-1""#) && !m.contains("photos-1"),
            "{m}"
        );
        assert!(
            !m.contains("lui-carousel-dots")
                && !m.contains("lui-carousel-prev")
                && !m.contains("lui-carousel-next"),
            "{m}"
        );
    }
}
