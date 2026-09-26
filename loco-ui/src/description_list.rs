//! # Description list
//!
//! Terms and what they are: a record's fields, a summary before confirming, key facts. Terms
//! sit in a column beside their details, or above them with `.stacked()`.
//!
//! **Platform features:** `<dl>` with `<dt>`/`<dd>` pairs, each pair in a `<div>` (allowed in a
//! `<dl>`, every browser); a wrapping flex row per pair stacks it when narrow; CSS grid for the
//! two columns.
//!
//! **Accessibility:** a real description list, so screen readers announce the pairs and their
//! count; a detail may hold any markup (a badge, a link). Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** without grid the terms stack above their details.
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
/* After Radix Themes DataList: rows separated by space, not rules; the label muted in a fixed
   10rem column, the value beside it. Each row is a wrapping flex line (a <div> round its
   <dt> and <dd>), so where the value no longer fits its 12rem it moves under the label:
   stacked in a narrow box with no breakpoint. .stacked() stacks always. */
.lui-description-list { display: grid; gap: var(--lui-space-3); margin: 0; }
.lui-description-list-item { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--lui-space-1) var(--lui-space-4); }
.lui-description-list dt { flex: 0 0 10rem; min-width: 0; color: var(--lui-muted); font-size: 0.875rem; line-height: 1.25rem; }
.lui-description-list dd { flex: 1 1 12rem; min-width: 0; margin: 0; overflow-wrap: anywhere; }
.lui-description-list-stacked dt, .lui-description-list-stacked dd { flex-basis: 100%; }
"#;
