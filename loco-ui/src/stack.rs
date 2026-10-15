//! # Stack
//!
//! Children one under the other with an even gap between them and no margins to fight: the
//! vertical rhythm of a form, a card body, a settings page.
//!
//! **Platform features:** a flex column with `gap` (Chrome 84, Firefox 63, Safari 14.1);
//! the gap is a step of the `--lui-space-*` scale. `.align(..)` and `.justify(..)` write
//! `data-align` and `data-justify` (`align-items`, `justify-content`), `.padding(n)` a step
//! of the same scale, as Radix Themes Flex's props do.
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
//! let m = ui.stack().body(html! { p { "One" } p { "Two" } }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-stack"><p>One</p><p>Two</p></div>"#);
//! // `.gap(n)` picks a step: n × 4px with the default `--lui-space`.
//! let m = ui.stack().gap(2).body(html! { p { "Tight" } }).render().into_string();
//! assert!(m.contains(r#"class="lui-stack lui-gap-2""#));
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { Stack gap=2 { p { "Tight" } } };
//! assert_eq!(same.into_string(), m);
//! // Centred across, packed at the end, with padding round it.
//! let m = ui.stack().align(Align::Center).justify(Justify::End).padding(4)
//!     .body(html! { p { "Done" } }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-stack lui-pad-4" data-align="center" data-justify="end"><p>Done</p></div>"#);
//! let same = lui! { Stack align="center" justify="end" padding=4 { p { "Done" } } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};
use crate::variant::{Align, Justify};

/// A vertical stack, made by [`Ui::stack`].
///
/// **Setters.** Values and items: `.body(..)`, `.gap(..)`, `.align(..)`, `.justify(..)`,
/// `.padding(..)`.
#[derive(Clone, Debug)]
pub struct Stack {
    content: Markup,
    gap: Option<u8>,
    align: Option<Align>,
    justify: Option<Justify>,
    padding: Option<u8>,
}

impl Stack {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup")
            .doc("What is stacked: each top-level element under the one before."),
        Prop::new("gap", PropKind::Number, "n: u8")
            .doc("The gap as a step of the `--lui-space-*` scale."),
        Prop::new(
            "align",
            PropKind::Choice { of: Align::NAMES },
            "align: Align",
        )
        .default("stretch")
        .doc("Where the children sit across the column (`align-items`)."),
        Prop::new(
            "justify",
            PropKind::Choice { of: Justify::NAMES },
            "justify: Justify",
        )
        .default("start")
        .doc("How the children share the column's height (`justify-content`)."),
        Prop::new("padding", PropKind::Number, "n: u8")
            .doc("Padding on every side as a step of the `--lui-space-*` scale."),
    ];
}

impl Ui {
    /// A stack: the top-level elements of its [`Stack::body`] one under the other, 16px
    /// apart by default.
    pub fn stack(&self) -> Stack {
        Stack {
            content: Markup::default(),
            gap: None,
            align: None,
            justify: None,
            padding: None,
        }
    }
}

impl Stack {
    /// What is stacked: each top-level element under the one before.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// The gap as a step of the `--lui-space-*` scale: 0, 1, 2, 3, 4, 6 or 8 (n × 4px by
    /// default); other values take the step below.
    pub fn gap(mut self, n: u8) -> Self {
        self.gap = Some(n);
        self
    }

    /// Where the children sit across the column: stretch (the default: full width), start,
    /// center, end or baseline.
    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    /// How the children share the column's height when it is taller than they are: start
    /// (the default), center, end or between.
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = Some(justify);
        self
    }

    /// Padding on every side as a step of the `--lui-space-*` scale: 0, 1, 2, 3, 4, 6 or 8.
    pub fn padding(mut self, n: u8) -> Self {
        self.padding = Some(n);
        self
    }
}

/// The padding class for step `n` of the space scale, shared with [`crate::cluster`]: the
/// steps of `gap`, the step below for a value between.
pub(crate) fn pad_class(n: u8) -> &'static str {
    match n {
        0 => "lui-pad-0",
        1 => "lui-pad-1",
        2 => "lui-pad-2",
        3 => "lui-pad-3",
        4 | 5 => "lui-pad-4",
        6 | 7 => "lui-pad-6",
        _ => "lui-pad-8",
    }
}

impl Render for Stack {
    fn render(&self) -> Markup {
        html! {
            div class={
                "lui-stack"
                @if let Some(n) = self.gap { " " (crate::gap_class(n)) }
                @if let Some(n) = self.padding { " " (pad_class(n)) }
            } data-align=[self.align.map(Align::as_str)] data-justify=[self.justify.map(Justify::as_str)] { (self.content) }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-stack { display: flex; flex-direction: column; }
:where(.lui-stack) { gap: var(--lui-space-4); }
.lui-stack > * { margin-block: 0; }
/* Fields and blocks take the full width; a button or badge keeps its own. */
.lui-stack > :is(.lui-button, .lui-badge) { align-self: flex-start; }
/* .align(..) and .justify(..), after Radix Themes Flex (cluster.rs has its own). */
.lui-stack[data-align=start] { align-items: flex-start; }
.lui-stack[data-align=center] { align-items: center; }
.lui-stack[data-align=end] { align-items: flex-end; }
.lui-stack[data-align=baseline] { align-items: baseline; }
.lui-stack[data-align=stretch] { align-items: stretch; }
.lui-stack[data-align] > :is(.lui-button, .lui-badge) { align-self: auto; }
.lui-stack[data-justify=start] { justify-content: flex-start; }
.lui-stack[data-justify=center] { justify-content: center; }
.lui-stack[data-justify=end] { justify-content: flex-end; }
.lui-stack[data-justify=between] { justify-content: space-between; }
/* .padding(n) on a stack or a cluster: the steps of the gap scale, shared like lui-gap-N. */
.lui-pad-0 { padding: 0; }
.lui-pad-1 { padding: var(--lui-space-1); }
.lui-pad-2 { padding: var(--lui-space-2); }
.lui-pad-3 { padding: var(--lui-space-3); }
.lui-pad-4 { padding: var(--lui-space-4); }
.lui-pad-6 { padding: var(--lui-space-6); }
.lui-pad-8 { padding: var(--lui-space-8); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stack_wraps_its_body_with_no_extras_by_default() {
        let m = Ui::default()
            .stack()
            .body(html! { p { "A" } p { "B" } })
            .render()
            .into_string();
        assert_eq!(m, r#"<div class="lui-stack"><p>A</p><p>B</p></div>"#, "{m}");
    }

    #[test]
    fn gap_and_padding_add_scale_classes_gap_first() {
        let m = Ui::default()
            .stack()
            .gap(2)
            .padding(6)
            .render()
            .into_string();
        assert!(
            m.starts_with(r#"<div class="lui-stack lui-gap-2 lui-pad-6""#),
            "{m}"
        );
    }

    #[test]
    fn align_and_justify_write_data_attributes() {
        let m = Ui::default()
            .stack()
            .align(Align::Center)
            .justify(Justify::End)
            .render()
            .into_string();
        assert!(
            m.contains(r#"data-align="center" data-justify="end""#),
            "{m}"
        );
    }

    #[test]
    fn padding_snaps_to_the_scale() {
        assert_eq!(pad_class(0), "lui-pad-0");
        assert_eq!(pad_class(5), "lui-pad-4");
        assert_eq!(pad_class(7), "lui-pad-6");
        assert_eq!(pad_class(200), "lui-pad-8");
    }
}
