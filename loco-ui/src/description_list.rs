//! # Description list
//!
//! Terms and what they are: a record's fields, a summary before confirming, key facts. Terms
//! sit in a column beside their details, or above them with `.stacked()`.
//!
//! **Platform features:** `<dl>` with `<dt>`/`<dd>` pairs, each pair in a `<div>` (allowed in a
//! `<dl>`, every browser); container queries (`@container`: Chrome 105, Firefox 110, Safari 16) put the
//! terms beside their details once the list is 30rem wide; subgrid (Chrome 117, Firefox 71,
//! Safari 16) gives every row one shared term column, as wide as the longest term.
//!
//! **Accessibility:** a real description list, so screen readers announce the pairs and their
//! count; a detail may hold any markup (a badge, a link). Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** without container queries the terms stay above their details; without
//! subgrid the term column is a fixed 10rem. An empty detail shows a dash.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.description_list().item("Plan", "Team").item("Seats", "12").render().into_string();
//! assert!(m.contains("<dt>Plan</dt><dd>Team</dd>"));
//!
//! let m = ui.description_list().stacked().item("Status", ui.badge("Active").ok());
//! let html = m.render().into_string();
//! assert!(html.contains("lui-description-list lui-description-list-stacked") && html.contains("Active"));
//! // The same in `lui!`:
//! let same = lui! { DescriptionList stacked { item "Status" (ui.badge("Active").ok()); } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// Terms and their details, made by [`Ui::description_list`].
///
/// **Setters.** Values and items: `.item(..)`; switches: `.stacked()`.
#[derive(Clone, Debug, Default)]
pub struct DescriptionList<'a> {
    items: Vec<(&'a str, Markup)>,
    stacked: bool,
}

impl DescriptionList<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("item", PropKind::Item, "term: &'a str, detail: impl Render")
            .doc("A term and its detail (text, or markup such as a badge)."),
        Prop::new("stacked", PropKind::Switch, "")
            .doc("Each term above its detail instead of beside it."),
    ];
}

impl Ui {
    /// An empty description list; add pairs with [`DescriptionList::item`].
    pub fn description_list<'a>(&self) -> DescriptionList<'a> {
        DescriptionList::default()
    }
}

impl<'a> DescriptionList<'a> {
    /// A term and its detail (text, or markup such as a badge).
    pub fn item(mut self, term: &'a str, detail: impl Render) -> Self {
        self.items.push((term, detail.render()));
        self
    }

    /// Each term above its detail instead of beside it.
    pub fn stacked(mut self) -> Self {
        self.stacked = true;
        self
    }
}

impl Render for DescriptionList<'_> {
    fn render(&self) -> Markup {
        html! {
            dl class={ "lui-description-list" @if self.stacked { " lui-description-list-stacked" } } {
                @for (term, detail) in &self.items { div class="lui-description-list-item" { dt { (term) } dd { (detail) } } }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes DataList: the label muted, the value beside it, rows split by hairlines.
   Narrow (the base, and what a browser without container queries draws): each label above its
   value. From 30rem: label and value side by side, both tops on one line (same 1.25rem line
   height, so first lines align); with subgrid every row shares the list's label column, sized
   to the longest label (up to 16rem); without it the label column is a fixed 10rem. Controls
   taller than a line (a badge, a button) give back their extra height so a row with one is as
   tall as a row of text. .stacked() keeps the narrow layout at every width. */
.lui-description-list {
  container: lui-description-list / inline-size;
  display: grid; grid-template-columns: fit-content(16rem) minmax(0, 1fr); column-gap: var(--lui-space-4); margin: 0;
}
.lui-description-list-item { grid-column: 1 / -1; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-1); padding-block: var(--lui-space-3); }
.lui-description-list-item:first-child { padding-top: 0; }
.lui-description-list-item:last-child { padding-bottom: 0; }
.lui-description-list-item + .lui-description-list-item { border-top: 1px solid var(--lui-line); }
.lui-description-list dt, .lui-description-list dd { min-width: 0; font-size: 0.875rem; line-height: 1.25rem; }
.lui-description-list dt { color: var(--lui-muted); }
.lui-description-list dd { margin: 0; overflow-wrap: anywhere; }
.lui-description-list dd:empty::before { content: "—"; color: var(--lui-muted); }
@container lui-description-list (width >= 30rem) {
  .lui-description-list:not(.lui-description-list-stacked) > .lui-description-list-item { display: flex; align-items: start; gap: var(--lui-space-4); }
  .lui-description-list:not(.lui-description-list-stacked) > .lui-description-list-item > dt { flex: 0 0 10rem; }
  .lui-description-list:not(.lui-description-list-stacked) > .lui-description-list-item > dd { flex: 1 1 0; }
  .lui-description-list:not(.lui-description-list-stacked) dd > .lui-badge { vertical-align: top; margin-block: -1px; }
  .lui-description-list:not(.lui-description-list-stacked) dd > .lui-button { vertical-align: top; margin-block: calc((1.25rem - var(--lui-control-h)) / 2); }
  .lui-description-list:not(.lui-description-list-stacked) dd > .lui-button-small { margin-block: calc((1.25rem - var(--lui-control-h-sm)) / 2); }
  @supports (grid-template-columns: subgrid) {
    .lui-description-list:not(.lui-description-list-stacked) > .lui-description-list-item { display: grid; grid-template-columns: subgrid; }
  }
}
"#;
