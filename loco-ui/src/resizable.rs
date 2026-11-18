//! # Resizable panes
//!
//! Two panes side by side with a divider between them, and a size the server remembers: a
//! file list beside a preview, a navigation beside a page. The size is the first pane's share
//! of the width in percent. Two links on the divider make it one step smaller or larger; with
//! the enhancement script the divider drags and answers the arrow keys. After shadcn/ui
//! Resizable with a handle: a one-pixel line and a small grip in its middle.
//!
//! **Platform features:** CSS grid with a custom property for the first column (baseline);
//! links that set `size.<id>` in the query, which the `lui-ui` cookie then remembers
//! ([`crate::UiState`]); `role="separator"` with `aria-valuenow`; a container query (Chrome
//! 105, Firefox 110, Safari 16) that puts the panes side by side from 30rem; with the script,
//! pointer events and `setPointerCapture` (Chrome 55, Firefox 59, Safari 13).
//!
//! **Accessibility:** the divider is a `role="separator"` named by the label, with its value,
//! minimum and maximum; the two links are named "Make … smaller" and "Make … larger". With
//! the script the separator takes the focus and the links are hidden, since the arrow keys
//! do their work. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no moderate, serious or critical violation).
//!
//! The root is not a swap root: links and forms inside the panes behave as they would
//! outside, and the script changes the size itself, with no request for markup.
//!
//! **What it does not do without script:** dragging, and the arrow keys on the separator.
//! Each change is a link, one step at a time.
//!
//! **Fallback:** under 30rem the panes stack and there is no divider: there is no width to
//! share.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.resizable("Files").first(html! { p { "list" } }).second(html! { p { "preview" } })
//!     .render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-resizable" id="lui-resizable-files" style="--lui-resizable-size:50%">"#));
//! assert!(m.contains(r#"role="separator" aria-orientation="vertical" aria-label="Files" aria-valuenow="50" aria-valuemin="20" aria-valuemax="80""#));
//! assert!(m.contains(r#"?size.files=40" aria-label="Make Files smaller""#) && m.contains(r#"aria-label="Make Files larger""#));
//! // The same in `lui!`:
//! let same = lui! { Resizable("Files") first={ p { "list" } } second={ p { "preview" } }; };
//! assert_eq!(same.into_string(), m);
//! // A narrower first pane with finer steps; the request's `size.<id>` wins over `.value(..)`:
//! let ui = Ui::from_request("/", "size.files=35", "");
//! let m = ui.resizable("Files").value(30).min(15).max(60).step(5).render().into_string();
//! assert!(m.contains("--lui-resizable-size:35%") && m.contains(r#"href="/?size.files=30""#));
//! assert_eq!(lui! { Resizable("Files") value=30 min=15 max=60 step=5; }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::icon::Icon;
use crate::props::{Key, Prop, PropKind};

/// Two panes and the divider between them, made by [`Ui::resizable`].
///
/// **Setters.** Values: `.id(..)`, `.first(..)`, `.second(..)`, `.value(..)`, `.min(..)`,
/// `.max(..)`, `.step(..)`.
#[derive(Clone, Debug)]
pub struct Resizable<'a> {
    ui: &'a Ui,
    label: &'a str,
    id: Option<&'a str>,
    first: Markup,
    second: Markup,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
}

impl Resizable<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new("Enter", "Follows the smaller or larger link."),
        Key::new("ArrowLeft / ArrowRight", "Moves the divider one step.").script(),
        Key::new("Home / End", "Moves the divider to its minimum or maximum.").script(),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("id", PropKind::Value, "id: &'a str")
            .doc("Replace the key derived from the label: the state is `size.<id>`, the element `lui-resizable-<id>`."),
        Prop::new("first", PropKind::Value, "body: Markup").doc("The first pane, the one the size is of."),
        Prop::new("second", PropKind::Value, "body: Markup").doc("The second pane, which takes the rest."),
        Prop::new("value", PropKind::Number, "value: impl Into<f64>")
            .default("50")
            .doc("The first pane's share in percent until the visitor changes it."),
        Prop::new("min", PropKind::Number, "min: impl Into<f64>")
            .default("20")
            .doc("The smallest share, in percent."),
        Prop::new("max", PropKind::Number, "max: impl Into<f64>")
            .default("80")
            .doc("The largest share, in percent."),
        Prop::new("step", PropKind::Number, "step: impl Into<f64>")
            .default("10")
            .doc("How far a link or an arrow key moves the divider, in percent."),
    ];
}

impl Ui {
    /// Two panes whose divider is named `label`.
    pub fn resizable<'a>(&'a self, label: &'a str) -> Resizable<'a> {
        Resizable {
            ui: self,
            label,
            id: None,
            first: Markup::default(),
            second: Markup::default(),
            value: 50.0,
            min: 20.0,
            max: 80.0,
            step: 10.0,
        }
    }
}

impl<'a> Resizable<'a> {
    /// Replace the key derived from the label: the state is `size.<id>`, the element
    /// `lui-resizable-<id>`.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// The first pane, the one the size is of.
    pub fn first(mut self, body: Markup) -> Self {
        self.first = body;
        self
    }

    /// The second pane, which takes the rest.
    pub fn second(mut self, body: Markup) -> Self {
        self.second = body;
        self
    }

    /// The first pane's share in percent until the visitor changes it.
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = value.into();
        self
    }

    /// The smallest share, in percent.
    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = min.into();
        self
    }

    /// The largest share, in percent.
    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = max.into();
        self
    }

    /// How far a link or an arrow key moves the divider, in percent.
    pub fn step(mut self, step: impl Into<f64>) -> Self {
        self.step = step.into();
        self
    }
}

impl Render for Resizable<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let key = self
            .id
            .map_or_else(|| crate::slug(self.label), str::to_string);
        let state = format!("size.{key}");
        // Whole percents inside 1..=99 whatever the setters said, so both panes always show.
        let min = self.min.clamp(1.0, 99.0).round();
        let max = self.max.clamp(min, 99.0).round();
        let step = self.step.max(1.0).round();
        let asked = ui.state.get(&state).and_then(|v| v.parse::<f64>().ok());
        let now = asked.unwrap_or(self.value).clamp(min, max).round();
        let link = |to: f64| ui.state.link(&state, &to.to_string());
        let smaller = (now > min).then(|| link((now - step).max(min)));
        let larger = (now < max).then(|| link((now + step).min(max)));
        let (less, more) = (
            ui.fill(Text::MakeSmaller, &[&self.label]),
            ui.fill(Text::MakeLarger, &[&self.label]),
        );
        let arrow = |href: &str, label: &str, icon: Icon| {
            html! {
                (ui.link_button("", href).icon_only().ghost().size(1).class("lui-resizable-step").aria_label(label).body(html! { (icon) }))
            }
        };
        html! {
            div class="lui-resizable" id=(crate::enhance::swap_id("lui-resizable", &key)) style=(format!("--lui-resizable-size:{now}%")) {
                div class="lui-resizable-row" {
                    div class="lui-resizable-pane" { (self.first) }
                    div class="lui-resizable-divider" {
                        @if let Some(href) = &smaller { (arrow(href, &less, Icon::ChevronLeft)) }
                        div class="lui-resizable-handle" role="separator" aria-orientation="vertical" aria-label=(self.label)
                            aria-valuenow=(now) aria-valuemin=(min) aria-valuemax=(max) data-lui-key=(state) data-lui-step=(step) {
                            span class="lui-resizable-grip" aria-hidden="true" { (Icon::GripVertical) }
                        }
                        @if let Some(href) = &larger { (arrow(href, &more, Icon::ChevronRight)) }
                    }
                    div class="lui-resizable-pane" { (self.second) }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After shadcn/ui Resizable (withHandle): a 1px line, a 0.75rem by 1rem grip with a border
   and a small radius in its middle, panes in one bordered box. Stacked under 30rem. */
.lui-resizable { container: lui-resizable / inline-size; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg); background: var(--lui-card); overflow: hidden; }
.lui-resizable-row { display: grid; grid-template-columns: minmax(0, 1fr); }
.lui-resizable-pane { min-width: 0; padding: var(--lui-space-4); overflow: auto; overflow-wrap: anywhere; }
.lui-resizable-divider { display: none; }
.lui-resizable-divider + .lui-resizable-pane { border-block-start: 1px solid var(--lui-line); }
.lui-resizable-handle { display: grid; place-items: center; }
.lui-resizable-grip { position: relative; display: grid; place-items: center; width: 0.75rem; height: 1rem; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-sm); background: var(--lui-card); color: var(--lui-muted); }
.lui-resizable-grip svg { width: 0.625rem; height: 0.625rem; }
.lui-resizable-step { position: relative; background: var(--lui-card); }
.lui-resizable-step svg:dir(rtl) { scale: -1 1; }
[data-lui-js] .lui-resizable-step { display: none; }
[data-lui-js] .lui-resizable-handle { position: absolute; inset: 0; cursor: col-resize; touch-action: none; }
.lui-resizable-handle:focus-visible { outline: 0; }
.lui-resizable-handle:focus-visible .lui-resizable-grip { border-color: var(--lui-ring); box-shadow: 0 0 0 2px var(--lui-ring); }
@container lui-resizable (width >= 30rem) {
  .lui-resizable-row { grid-template-columns: minmax(0, var(--lui-resizable-size, 50%)) auto minmax(0, 1fr); }
  .lui-resizable-divider { position: relative; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: var(--lui-space-1); min-width: 0.75rem; }
  .lui-resizable-divider::before { content: ""; position: absolute; inset-block: 0; inset-inline-start: calc(50% - 0.5px); width: 1px; background: var(--lui-line); }
  .lui-resizable-divider + .lui-resizable-pane { border-block-start: 0; }
}
@media (pointer: coarse) { [data-lui-js] .lui-resizable-divider { min-width: var(--lui-hit); } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn at(query: &str) -> String {
        let ui = Ui::from_request("/", query, "");
        ui.resizable("Files").render().into_string()
    }

    #[test]
    fn the_request_sets_the_size_inside_the_limits() {
        assert!(at("").contains("--lui-resizable-size:50%"));
        assert!(at("size.files=30").contains("--lui-resizable-size:30%"));
        assert!(at("size.files=3").contains("--lui-resizable-size:20%"));
        assert!(at("size.files=300").contains("--lui-resizable-size:80%"));
        assert!(at("size.files=x").contains("--lui-resizable-size:50%"));
    }

    #[test]
    fn a_limit_has_no_link_past_it() {
        let m = at("size.files=20");
        assert!(
            !m.contains("Make Files smaller") && m.contains(r#"href="/?size.files=30""#),
            "{m}"
        );
        let m = at("size.files=75");
        assert!(
            m.contains(r#"href="/?size.files=65""#) && m.contains(r#"href="/?size.files=80""#),
            "{m}"
        );
        let m = at("size.files=80");
        assert!(!m.contains("Make Files larger"), "{m}");
    }

    #[test]
    fn the_id_names_the_state_and_the_element() {
        let ui = Ui::from_request("/", "size.mail=40", "");
        let m = ui.resizable("Files").id("mail").render().into_string();
        assert!(
            m.contains(r#"id="lui-resizable-mail""#) && m.contains("--lui-resizable-size:40%"),
            "{m}"
        );
        assert!(
            m.contains(r#"data-lui-key="size.mail" data-lui-step="10""#),
            "{m}"
        );
    }
}
