//! # Checkbox group
//!
//! Several choices under one legend, any number of them ticked: the topics of a newsletter,
//! the days a shop opens. Each choice is a native checkbox posting `<name>[]`, so the form
//! sends one pair per ticked box and the server reads them with `Posted::all("<name>[]")`.
//! A form adds one with `.checkboxes(..)`.
//!
//! After Radix Themes CheckboxGroup and CheckboxCards: a column of drawn checkboxes with the
//! label beside each and an optional line under it, or `.cards()` for a grid of bordered
//! cards that ring in the primary colour when ticked.
//!
//! **Platform features:** `<fieldset>` + `<legend>` (Chrome 1, Firefox 1, Safari 3) around
//! `<input type="checkbox">`, drawn with `appearance: none` (Chrome 84, Firefox 80, Safari 15.4); `:has()`
//! (Chrome 105, Firefox 121, Safari 15.4) rings a ticked card; a container query (Chrome 105,
//! Firefox 110, Safari 16) lays cards 1, 2 or 3 across.
//!
//! **Accessibility:** the legend names the group; each checkbox has its `<label>`, a choice's
//! description is joined by `aria-describedby`, help and the server's error describe the
//! fieldset, which gets `aria-invalid` with an error. Checked by axe-core in headless Firefox
//! on every demo route, both capability variants, light and dark (no serious or critical
//! violation).
//!
//! **What it does not do without script:** require at least one tick (HTML has no
//! `required` for a group of checkboxes); the server checks it and sends an `.error(..)`.
//!
//! **Fallback:** none needed: native checkboxes. Without `:has()` a ticked card shows its
//! tick but no ring; without container queries the cards stay one per row.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The ticked values are the query's `topics[]`, or `.value(..)` (saved ones).
//! let ui = Ui::from_request("/news", "topics%5B%5D=rust", "");
//! let m = ui.checkbox_group("topics", "Topics").option("rust", "Rust").option("css", "CSS");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<legend>Topics</legend>"#));
//! assert!(html.contains(r#"type="checkbox" name="topics[]" value="rust" checked"#));
//! // The same in `lui!`:
//! let same = lui! { CheckboxGroup("topics", "Topics") { option "rust" "Rust"; option "css" "CSS"; } };
//! assert_eq!(same.into_string(), html);
//! // Cards, a description and a choice that cannot be ticked, with saved values.
//! let m = ui.checkbox_group("days", "Open on").cards()
//!     .option("sat", "Saturday").description("10:00 to 14:00")
//!     .option("sun", "Sunday").disabled()
//!     .value(&["sat"]).help("Pick the days you open.");
//! let html = m.render().into_string();
//! assert!(html.contains("lui-checkbox-cards") && html.contains(r#"value="sun" disabled"#));
//! assert!(html.contains(r#"aria-describedby="f-days-0-description""#));
//! let same = lui! { CheckboxGroup("days", "Open on") cards value=(&["sat"]) help="Pick the days you open." {
//!     option "sat" "Saturday" description="10:00 to 14:00";
//!     option "sun" "Sunday" disabled;
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::{Strings, Text};
use crate::input::{Choice, Field, Pick, described_by, error_line};
use crate::props::{Prop, PropKind};

/// Checkboxes under a legend, made by [`Ui::checkbox_group`]; a form's `.checkboxes(..)`
/// field is one too.
///
/// **Setters.** Values and items: `.option(..)`, `.description(..)`, `.value(..)`,
/// `.help(..)`, `.error(..)`, `.id(..)`; switches: `.cards()`, `.disabled()`.
#[derive(Clone, Debug)]
pub struct CheckboxGroup<'a> {
    name: &'a str,
    legend: &'a str,
    id: Option<&'a str>,
    pub(crate) options: Vec<Pick<'a>>,
    pub(crate) value: Vec<&'a str>,
    cards: bool,
    required: bool,
    optional: bool,
    help: Option<&'a str>,
    error: Option<&'a str>,
    strings: &'static Strings,
}

impl CheckboxGroup<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("option", PropKind::Item, "value: &'a str, label: &'a str")
            .doc("One choice posting `value` under `<name>[]` when ticked, labelled `label`."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .applies_to(&["option"])
            .doc("A line under the choice added last, read out with it."),
        Prop::new("disabled", PropKind::Modifier, "")
            .applies_to(&["option"])
            .attr("disabled")
            .doc("The choice added last cannot be ticked."),
        Prop::new("cards", PropKind::Switch, "")
            .doc("Each choice a card, 1, 2 or 3 across by the container's width."),
        Prop::new("value", PropKind::Value, "values: &'a [&'a str]")
            .attr("checked")
            .doc("The ticked values, instead of the query's `<name>[]`."),
        Prop::new("help", PropKind::Value, "help: &'a str").doc("Help text under the choices."),
        Prop::new("error", PropKind::Value, "message: &'a str")
            .doc("A server message under the choices."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("What the checkboxes' ids start from, `f-<name>` by default."),
    ];
}

impl Ui {
    /// Checkboxes posting `<name>[]` in a `<fieldset>` whose `<legend>` is `legend`; the
    /// ticked ones are the query's `<name>[]` unless [`CheckboxGroup::value`] says otherwise.
    pub fn checkbox_group<'a>(&'a self, name: &'a str, legend: &'a str) -> CheckboxGroup<'a> {
        CheckboxGroup {
            value: self.param_values(&format!("{name}[]")),
            ..CheckboxGroup::bare(self.strings, name, legend)
        }
    }
}

impl<'a> CheckboxGroup<'a> {
    fn bare(strings: &'static Strings, name: &'a str, legend: &'a str) -> Self {
        CheckboxGroup {
            name,
            legend,
            id: None,
            options: Vec::new(),
            value: Vec::new(),
            cards: false,
            required: false,
            optional: false,
            help: None,
            error: None,
            strings,
        }
    }

    /// A form's checkboxes field drawn as a group.
    pub(crate) fn of(f: &Field<'a>) -> Self {
        CheckboxGroup {
            id: f.id,
            cards: f.extra.cards,
            required: f.required,
            optional: f.optional,
            help: f.help,
            error: f.error,
            ..CheckboxGroup::bare(f.strings, f.name, f.label)
        }
    }

    /// One choice posting `value` under `<name>[]` when ticked, labelled `label`.
    pub fn option(mut self, value: &'a str, label: &'a str) -> Self {
        self.options.push(Choice { value, label }.into());
        self
    }

    /// A line under the choice added last, read out with it (`aria-describedby`).
    pub fn description(mut self, text: &'a str) -> Self {
        debug_assert!(
            !self.options.is_empty(),
            "CheckboxGroup::description() applies to an option, but there is none yet"
        );
        if let Some(o) = self.options.last_mut() {
            o.description = Some(text);
        }
        self
    }

    /// The choice added last cannot be ticked: `disabled`, greyed. A disabled box posts
    /// nothing, ticked or not.
    pub fn disabled(mut self) -> Self {
        debug_assert!(
            !self.options.is_empty(),
            "CheckboxGroup::disabled() applies to an option, but there is none yet"
        );
        if let Some(o) = self.options.last_mut() {
            o.disabled = true;
        }
        self
    }

    /// Each choice a card (title and description, the box hidden, the card ringed when
    /// ticked), 1, 2 or 3 across by the container's width.
    pub fn cards(mut self) -> Self {
        self.cards = true;
        self
    }

    /// The ticked values, instead of the query's `<name>[]`: saved ones.
    pub fn value(mut self, values: &'a [&'a str]) -> Self {
        self.value = values.to_vec();
        self
    }

    /// Help text under the choices.
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// A server message under the choices ("Pick at least one."); the group gets
    /// `aria-invalid`. An empty message is no error.
    pub fn error(mut self, message: &'a str) -> Self {
        self.error = (!message.is_empty()).then_some(message);
        self
    }

    /// What the checkboxes' ids start from (`<id>-0`, `<id>-1`..), `f-<name>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Render for CheckboxGroup<'_> {
    fn render(&self) -> Markup {
        let id = self
            .id
            .map_or_else(|| format!("f-{}", crate::slug(self.name)), str::to_string);
        let posted = format!("{}[]", self.name);
        let described = described_by(&[
            self.help.map(|_| format!("{id}-help")),
            self.error.map(|_| format!("{id}-error")),
        ]);
        html! {
            fieldset class={ "lui-field lui-checkbox-group" @if self.cards { " lui-checkbox-cards" } }
                aria-describedby=[described.as_deref()] aria-invalid=[self.error.map(|_| "true")] {
                legend {
                    (self.legend) @if self.required { " " (crate::input::REQUIRED) }
                    @if self.optional && !self.required { " " span class="lui-field-optional" { (self.strings.get(Text::Optional)) } }
                }
                div class="lui-checkbox-group-options" {
                    @for (i, o) in self.options.iter().enumerate() {
                        @let oid = format!("{id}-{i}");
                        @let did = o.description.map(|_| format!("{oid}-description"));
                        label for=(oid) class="lui-checkbox-group-option" {
                            input id=(oid) class="lui-checkbox-group-input" type="checkbox" name=(posted) value=(o.value)
                                checked[self.value.contains(&o.value)]
                                disabled[o.disabled] aria-disabled=[o.disabled.then_some("true")]
                                aria-describedby=[did.as_deref()];
                            span class="lui-checkbox-group-text" {
                                span class="lui-checkbox-group-title" { (o.label) }
                                @if let (Some(d), Some(did)) = (o.description, &did) {
                                    span id=(did) class="lui-checkbox-group-description" { (d) }
                                }
                            }
                        }
                    }
                }
                @if let Some(h) = self.help { small id={ (id) "-help" } class="lui-field-help" { (h) } }
                @if let Some(e) = self.error { (error_line(&format!("{id}-error"), e)) }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes CheckboxGroup (size 2): a column of 1rem drawn boxes (input.rs) with the
   label at 0.875rem/500 and a --lui-muted description under it, 0.75rem apart. .cards() is
   CheckboxCards: --lui-gray-7 bordered cards, px-4 py-3, ringed in the primary colour when
   ticked, 1, 2 or 3 across as the group's own width allows; the box stays visible in the
   card's corner, as Radix draws it. */
.lui-checkbox-group { container: lui-checkbox-group / inline-size; margin: 0; padding: 0; border: 0; gap: 0.75rem; min-width: 0; }
.lui-checkbox-group legend { padding: 0; margin-bottom: 0.75rem; font-size: var(--lui-text-2); font-weight: 500; }
.lui-checkbox-group[aria-invalid=true] legend { color: var(--lui-danger); }
.lui-checkbox-group-options { display: grid; gap: 0.75rem; }
.lui-checkbox-group-option { display: flex; align-items: flex-start; gap: 0.5rem; cursor: pointer; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); font-weight: 500; }
.lui-checkbox-group-option > .lui-checkbox-group-input { margin-top: 0.125rem; }
.lui-checkbox-group-option:has(> .lui-checkbox-group-input:disabled) { color: var(--lui-gray-11); cursor: not-allowed; }
.lui-checkbox-group-text { display: grid; gap: 0.125rem; min-width: 0; overflow-wrap: anywhere; }
.lui-checkbox-group-description { color: var(--lui-muted); font-weight: 400; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); }
.lui-checkbox-cards .lui-checkbox-group-options { grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-3); }
@container lui-checkbox-group (width >= 30rem) { .lui-checkbox-cards .lui-checkbox-group-options { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
@container lui-checkbox-group (width >= 48rem) { .lui-checkbox-cards .lui-checkbox-group-options { grid-template-columns: repeat(3, minmax(0, 1fr)); } }
.lui-checkbox-cards .lui-checkbox-group-option {
  flex-direction: row-reverse; justify-content: space-between; padding: var(--lui-space-3) var(--lui-space-4);
  border: 1px solid var(--lui-input); border-radius: var(--lui-radius); background: var(--lui-bg);
  transition: border-color var(--lui-duration-fast), box-shadow var(--lui-duration-fast);
}
.lui-checkbox-cards .lui-checkbox-group-option:hover { border-color: var(--lui-gray-8); }
.lui-checkbox-cards .lui-checkbox-group-option:has(> .lui-checkbox-group-input:checked) { border-color: var(--lui-primary); box-shadow: inset 0 0 0 1px var(--lui-primary); }
.lui-checkbox-cards .lui-checkbox-group-option:has(> .lui-checkbox-group-input:focus-visible) { outline: 3px solid color-mix(in srgb, var(--lui-ring) 50%, transparent); }
.lui-checkbox-cards .lui-checkbox-group-option:has(> .lui-checkbox-group-input:disabled) { background: var(--lui-gray-2); }
@media (pointer: coarse) { .lui-checkbox-group-option { min-height: var(--lui-hit); align-items: center; } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_described_by_id_exists() {
        let ui = Ui::default();
        let html = ui
            .checkbox_group("t", "T")
            .option("a", "A")
            .description("d")
            .help("h")
            .error("e")
            .render()
            .into_string();
        crate::input::assert_described_ids_exist(&html);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "CheckboxGroup::disabled() applies to an option")]
    fn a_modifier_before_any_option_is_loud() {
        let _ = Ui::default().checkbox_group("t", "T").disabled();
    }
}
