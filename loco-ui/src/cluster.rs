//! # Cluster
//!
//! Children side by side, wrapping onto the next line when they run out of room: a row of
//! buttons, badges, filters or links.
//!
//! **Platform features:** `flex-wrap` with `gap` (Chrome 84, Firefox 63, Safari 14.1), so a
//! wrapped line keeps the same spacing and no child carries a margin. `.align(..)` and
//! `.justify(..)` write `data-align` and `data-justify` (`align-items`, `justify-content`),
//! `.padding(n)` a step of the space scale, as Radix Themes Flex's props do.
//!
//! **Accessibility:** layout only: no roles, reading order is source order. Checked by axe-core
//! in headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.cluster().body(html! { (ui.badge("rust")) (ui.badge("maud")) }).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-cluster">"#));
//! // A toolbar: the title on the left, the actions pushed to the right.
//! let m = ui.cluster()
//!     .between()
//!     .gap(4)
//!     .body(html! { h2 { "Orders" } (ui.button("New order").primary()) });
//! let m = m.render().into_string();
//! assert!(m.contains(r#"class="lui-cluster lui-cluster-between lui-gap-4""#));
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { Cluster between gap=4 { h2 { "Orders" } Button("New order") primary; } };
//! assert_eq!(same.into_string(), m);
//! // On one line, on the text baseline, centred, with padding round it.
//! let m = ui.cluster().nowrap().align(Align::Baseline).justify(Justify::Center).padding(3)
//!     .body(html! { h2 { "Plan" } span { "$9" } }).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-cluster lui-cluster-nowrap lui-pad-3" data-align="baseline" data-justify="center">"#));
//! let same = lui! { Cluster nowrap align="baseline" justify="center" padding=3 { h2 { "Plan" } span { "$9" } } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};
use crate::variant::{Align, Justify};

/// A wrapping row, made by [`Ui::cluster`].
///
/// **Setters.** Values and items: `.body(..)`, `.gap(..)`, `.align(..)`, `.justify(..)`,
/// `.padding(..)`; switches: `.between()`, `.end()`, `.nowrap()`.
#[derive(Clone, Debug)]
pub struct Cluster {
    content: Markup,
    justify: Option<Justify>,
    align: Option<Align>,
    gap: Option<u8>,
    padding: Option<u8>,
    nowrap: bool,
}

impl Cluster {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup")
            .doc("What goes in the row: each top-level element beside the one before."),
        Prop::new("between", PropKind::Switch, "")
            .doc("First child at the start, last at the end, the rest spread between."),
        Prop::new("end", PropKind::Switch, "")
            .doc("Children packed at the end of the row (a dialog's or card's actions)."),
        Prop::new("gap", PropKind::Number, "n: u8")
            .doc("The gap as a step of the `--lui-space-*` scale."),
        Prop::new(
            "align",
            PropKind::Choice { of: Align::NAMES },
            "align: Align",
        )
        .default("center")
        .doc("Where the children sit across the row (`align-items`)."),
        Prop::new(
            "justify",
            PropKind::Choice { of: Justify::NAMES },
            "justify: Justify",
        )
        .default("start")
        .doc("How the children share the row (`justify-content`)."),
        Prop::new("padding", PropKind::Number, "n: u8")
            .doc("Padding on every side as a step of the `--lui-space-*` scale."),
        Prop::new("nowrap", PropKind::Switch, "")
            .doc("Keep the children on one line; they shrink instead of wrapping."),
    ];
}

impl Ui {
    /// A cluster: the top-level elements of its [`Cluster::body`] in a row that wraps, 8px
    /// apart by default, centred on each other vertically.
    pub fn cluster(&self) -> Cluster {
        Cluster {
            content: Markup::default(),
            justify: None,
            align: None,
            gap: None,
            padding: None,
            nowrap: false,
        }
    }
}

impl Cluster {
    /// What goes in the row: each top-level element beside the one before.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// First child at the start, last at the end, the rest spread between.
    pub fn between(mut self) -> Self {
        self.justify = Some(Justify::Between);
        self
    }

    /// Children packed at the end of the row (a dialog's or card's actions).
    pub fn end(mut self) -> Self {
        self.justify = Some(Justify::End);
        self
    }

    /// The gap as a step of the `--lui-space-*` scale: 0, 1, 2, 3, 4, 6 or 8.
    pub fn gap(mut self, n: u8) -> Self {
        self.gap = Some(n);
        self
    }

    /// Where the children sit across the row: start, center (the default), end, baseline
    /// (a row of mixed text sizes) or stretch.
    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    /// How the children share the row: start (the default), center, end or between;
    /// `.between()` and `.end()` are the short forms.
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = Some(justify);
        self
    }

    /// Padding on every side as a step of the `--lui-space-*` scale: 0, 1, 2, 3, 4, 6 or 8.
    pub fn padding(mut self, n: u8) -> Self {
        self.padding = Some(n);
        self
    }

    /// Keep the children on one line; they shrink instead of wrapping.
    pub fn nowrap(mut self) -> Self {
        self.nowrap = true;
        self
    }
}

impl Render for Cluster {
    fn render(&self) -> Markup {
        // `.between()` and `.end()` keep their classes; the other choices are data attributes.
        let (class, justify) = match self.justify {
            Some(Justify::Between) => (Some("lui-cluster-between"), None),
            Some(Justify::End) => (Some("lui-cluster-end"), None),
            other => (None, other.map(Justify::as_str)),
        };
        html! {
            div class={
                "lui-cluster"
                @if let Some(c) = class { " " (c) }
                @if self.nowrap { " lui-cluster-nowrap" }
                @if let Some(n) = self.gap { " " (crate::gap_class(n)) }
                @if let Some(n) = self.padding { " " (crate::stack::pad_class(n)) }
            } data-align=[self.align.map(Align::as_str)] data-justify=[justify] { (self.content) }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-cluster { display: flex; flex-wrap: wrap; align-items: center; }
:where(.lui-cluster) { gap: var(--lui-space-2); }
.lui-cluster > * { margin: 0; }
.lui-cluster-between { justify-content: space-between; }
.lui-cluster-end { justify-content: flex-end; }
.lui-cluster-nowrap { flex-wrap: nowrap; min-width: 0; }
.lui-cluster-nowrap > * { min-width: 0; }
/* .align(..) and .justify(..), after Radix Themes Flex. */
.lui-cluster[data-align=start] { align-items: flex-start; }
.lui-cluster[data-align=center] { align-items: center; }
.lui-cluster[data-align=end] { align-items: flex-end; }
.lui-cluster[data-align=baseline] { align-items: baseline; }
.lui-cluster[data-align=stretch] { align-items: stretch; }
.lui-cluster[data-justify=start] { justify-content: flex-start; }
.lui-cluster[data-justify=center] { justify-content: center; }
.lui-cluster[data-justify=end] { justify-content: flex-end; }
.lui-cluster[data-justify=between] { justify-content: space-between; }
"#;
