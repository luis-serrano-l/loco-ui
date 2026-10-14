//! # Scroll area
//!
//! A box of a set height whose content scrolls inside it: a long list in a sidebar, a log, a
//! changelog in a card. After Radix Themes ScrollArea: a thin scrollbar in a gray step that
//! does not move the content when it appears, and (where the browser can tell) a soft shadow
//! at an edge that has more to scroll to.
//!
//! **Platform features:** `overflow: auto` (baseline); `scrollbar-gutter: stable` (Chrome 94,
//! Firefox 97, Safari 18.2) keeps the width when the bar comes and goes; `scrollbar-width`
//! (Chrome 121, Firefox 64, Safari 18.2) and `scrollbar-color` (Safari 26); the edge shadows are
//! `scroll-state()` container queries (Chrome 133, not Firefox or Safari yet) on two sticky
//! edges.
//!
//! **Accessibility:** a `role="region"` named by its label, with `tabindex="0"` so a keyboard
//! user can focus it and scroll with the arrow keys (axe's scrollable-region-focusable).
//! Checked by axe-core in headless Firefox on every demo route, both capability variants,
//! light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** without `scroll-state()` there are no edge shadows: the scrollbar alone says
//! there is more.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.scroll_area("Changelog").body(html! { p { "v0.2" } }).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-scroll-area" role="region" aria-label="Changelog" tabindex="0">"#));
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { ScrollArea("Changelog") { p { "v0.2" } } };
//! assert_eq!(same.into_string(), m);
//! // 12rem tall.
//! let m = ui.scroll_area("Log").height("12rem").body(html! { p { "…" } }).render().into_string();
//! assert!(m.contains(r#"style="--lui-scroll-area-height: 12rem""#));
//! assert_eq!(lui! { ScrollArea("Log") height="12rem" { p { "…" } } }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A scrolling box, made by [`Ui::scroll_area`].
///
/// **Setters.** Values: `.body(..)`, `.height(..)`.
#[derive(Clone, Debug)]
pub struct ScrollArea<'a> {
    label: &'a str,
    content: Markup,
    height: Option<&'a str>,
}

impl ScrollArea<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup").doc("What scrolls."),
        Prop::new("height", PropKind::Value, "height: &'a str")
            .default("16rem")
            .doc("The box's height as a CSS length (`12rem`, `50vh`)."),
    ];
}

impl Ui {
    /// A box 16rem tall whose content scrolls inside it, named `label` for assistive tech;
    /// the content goes in [`ScrollArea::body`].
    pub fn scroll_area<'a>(&self, label: &'a str) -> ScrollArea<'a> {
        ScrollArea {
            label,
            content: Markup::default(),
            height: None,
        }
    }
}

impl<'a> ScrollArea<'a> {
    /// What scrolls.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// The box's height as a CSS length (`12rem`, `50vh`); 16rem by default.
    pub fn height(mut self, height: &'a str) -> Self {
        self.height = Some(height);
        self
    }
}

impl Render for ScrollArea<'_> {
    fn render(&self) -> Markup {
        let style = self
            .height
            .map(|h| format!("--lui-scroll-area-height: {h}"));
        html! {
            div class="lui-scroll-area" role="region" aria-label=(self.label) tabindex="0" style=[style] {
                span class="lui-scroll-area-edge lui-scroll-area-top" aria-hidden="true" {}
                div class="lui-scroll-area-content" { (self.content) }
                span class="lui-scroll-area-edge lui-scroll-area-bottom" aria-hidden="true" {}
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes ScrollArea: a thin bar in --lui-gray-8 on a clear track, the content
   steady when it appears; an edge shadow where there is more to scroll to. */
.lui-scroll-area {
  position: relative; height: var(--lui-scroll-area-height, 16rem); max-width: 100%; overflow: auto; overscroll-behavior: contain;
  scrollbar-gutter: stable; scrollbar-width: thin; scrollbar-color: var(--lui-gray-8) transparent;
  border-radius: var(--lui-radius); container-type: scroll-state;
}
.lui-scroll-area-content { min-width: 0; }
.lui-scroll-area-edge { position: sticky; z-index: 1; display: block; height: 0; pointer-events: none; transition: box-shadow var(--lui-duration-fast) ease; }
.lui-scroll-area-top { top: 0; }
.lui-scroll-area-bottom { bottom: 0; }
@container scroll-state(scrollable: top) {
  .lui-scroll-area-top { box-shadow: 0 0 0.625rem 0.375rem color-mix(in srgb, var(--lui-fg) 14%, transparent); }
}
@container scroll-state(scrollable: bottom) {
  .lui-scroll-area-bottom { box-shadow: 0 0 0.625rem 0.375rem color-mix(in srgb, var(--lui-fg) 14%, transparent); }
}
"#;
