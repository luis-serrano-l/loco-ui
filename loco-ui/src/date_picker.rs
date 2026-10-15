//! # Date picker
//!
//! A form field for a date: a button showing the date, which opens a calendar in a popover;
//! picking a day checks a radio named after the field, so the form posts `name=YYYY-MM-DD`.
//! `.native()` gives the browser's own `<input type="date">` instead.
//!
//! **Platform features:** `popover` with `popovertarget` (Chrome 114, Firefox 125, Safari 17);
//! CSS anchor positioning (`anchor-name`, `position-anchor`, Chrome 125, Firefox 147, Safari 26)
//! places it under the button; inside, the [`crate::calendar`] in radio mode. `.native()` is
//! `<input type="date">` with `min`/`max` (Chrome 20, Firefox 57, Safari 14.1).
//!
//! **Accessibility:** a button naming the picked date that opens the calendar popover, the days
//! a radio group labelled by the field's legend; Escape closes the popover. Checked by axe-core
//! in headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** write the picked day onto the button before the form
//! is sent (the button shows the saved value; the picked day is filled in the calendar), or
//! change month without a page load. A month link comes back with the calendar laid out in the
//! page instead of in the popover, because a popover cannot arrive open.
//!
//! `.range(end)` picks a first and a last day into two names with one calendar, laid out in
//! the page (its days are links, so it cannot sit in a popover that a page load closes); the
//! form carries both days as hidden fields. `.disabled()` shows the date without a way to
//! change it and posts nothing; `.readonly()` shows it and posts it.
//!
//! **Fallback:** without `popover` the calendar is laid out in the form under the label.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let m = ui.date_picker("due", "Due date").value("2026-09-24").render().into_string();
//! assert!(m.contains(r#"popovertarget="f-due-calendar""#) && m.contains("24 September 2026"));
//! assert!(m.contains(r#"type="radio" name="due" value="2026-09-24" checked"#));
//! // No popover support: the same calendar, in the page.
//! let old = Ui::from(Caps::default());
//! assert!(!old.date_picker("due", "Due date").render().into_string().contains("popover"));
//! // The browser's own control, with bounds.
//! let m = ui.date_picker("due", "Due date").min("2026-01-01").max("2026-12-31").native();
//! assert!(m.render().into_string().contains(r#"type="date" value="" min="2026-01-01" max="2026-12-31""#));
//! // The same in `lui!`:
//! let same = lui! { DatePicker("due", "Due date") min="2026-01-01" max="2026-12-31" native; };
//! assert_eq!(same.into_string(), m.render().into_string());
//! // A stay: two names, one calendar; the form posts both.
//! let ui = Ui::from_request("/stay", "from=2026-09-10&to=2026-09-14", "");
//! let m = ui.date_picker("from", "Dates").range("to").render().into_string();
//! assert!(m.contains(r#"<input type="hidden" name="to" value="2026-09-14">"#));
//! assert!(m.contains("lui-calendar-between"));
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::calendar::{Date, Off};
use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::{Cap, Icon, Ui};

/// A date field, made by [`Ui::date_picker`].
///
/// **Setters.** Values and items: `.disabled_dates(..)`, `.value(..)`, `.min(..)`, `.max(..)`,
/// `.range(..)`; switches: `.required()`, `.native()`, `.disabled()`, `.readonly()`.
#[derive(Clone, Debug)]
pub struct DatePicker<'a> {
    ui: &'a Ui,
    name: &'a str,
    label: &'a str,
    value: Option<&'a str>,
    min: Option<&'a str>,
    max: Option<&'a str>,
    off: Option<Off<'a>>,
    end: Option<&'a str>,
    required: bool,
    native: bool,
    disabled: bool,
    readonly: bool,
}

impl DatePicker<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves to the trigger button, or between the day, month and year parts of the native date field.",
        ),
        Key::new(
            "Enter / Space",
            "Opens the calendar from the trigger button.",
        ),
        Key::new(
            "Escape",
            "Closes the calendar popover. Without popover support the script closes it; without the script either, its button does.",
        ),
        Key::new(
            "ArrowRight / ArrowDown",
            "Picks the next day in the calendar.",
        ),
        Key::new(
            "ArrowLeft / ArrowUp",
            "Picks the previous day in the calendar.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("value", PropKind::Value, "date: &'a str")
            .attr("value")
            .doc("The current value, `YYYY-MM-DD` (a saved date)."),
        Prop::new("min", PropKind::Value, "date: &'a str")
            .attr("min")
            .doc("The first day that can be picked, `YYYY-MM-DD`."),
        Prop::new("max", PropKind::Value, "date: &'a str")
            .attr("max")
            .doc("The last day that can be picked, `YYYY-MM-DD`."),
        Prop::new("disabled_dates", PropKind::Value, "off: &'a dyn Fn(Date) -> bool").doc(
            "Days for which `off` returns true cannot be picked (the native control ignores this).",
        ),
        Prop::new("range", PropKind::Value, "end: &'a str")
            .doc("A first and a last day: the first in `name`, the last in `end`, one calendar in the page."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("Shown, not changeable and not posted."),
        Prop::new("readonly", PropKind::Switch, "")
            .doc("Shown and posted, not changeable."),
        Prop::new("required", PropKind::Switch, "")
            .attr("required")
            .doc("A day must be picked before the form submits."),
        Prop::new("native", PropKind::Switch, "")
            .doc("The browser's own `<input type=\"date\">` instead of the calendar."),
    ];
}

impl Ui {
    /// A date field named `name` under `label`; its value is `?<name>=` unless `.value()` says
    /// otherwise.
    pub fn date_picker<'a>(&'a self, name: &'a str, label: &'a str) -> DatePicker<'a> {
        DatePicker {
            ui: self,
            name,
            label,
            value: None,
            min: None,
            max: None,
            off: None,
            end: None,
            required: false,
            native: false,
            disabled: false,
            readonly: false,
        }
    }
}

impl<'a> DatePicker<'a> {
    /// The current value, `YYYY-MM-DD` (a saved date).
    pub fn value(mut self, date: &'a str) -> Self {
        self.value = Some(date);
        self
    }

    /// The first day that can be picked, `YYYY-MM-DD`.
    pub fn min(mut self, date: &'a str) -> Self {
        self.min = Some(date);
        self
    }

    /// The last day that can be picked, `YYYY-MM-DD`.
    pub fn max(mut self, date: &'a str) -> Self {
        self.max = Some(date);
        self
    }

    /// Days for which `off` returns true cannot be picked (the native control ignores this).
    /// The closure may capture (holidays), so it is borrowed.
    pub fn disabled_dates(mut self, off: &'a dyn Fn(Date) -> bool) -> Self {
        self.off = Some(Off(off));
        self
    }

    /// A first and a last day: the first posted as `name`, the last as `end`, picked in one
    /// calendar laid out in the page ([`crate::calendar::Calendar::range`]).
    pub fn range(mut self, end: &'a str) -> Self {
        self.end = Some(end);
        self
    }

    /// Shown, not changeable and not posted: the button is `disabled`, there is no calendar.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Shown and posted, not changeable: no calendar, the value in a hidden field.
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    /// A day must be picked before the form submits.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// The browser's own `<input type="date">` instead of the calendar.
    pub fn native(mut self) -> Self {
        self.native = true;
        self
    }
}

impl Render for DatePicker<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let value = self.value.or_else(|| ui.param(self.name)).unwrap_or("");
        if self.native {
            let input = ui
                .input(self.name, self.label)
                .date(self.min.unwrap_or(""), self.max.unwrap_or(""));
            let input = if self.required {
                input.required()
            } else {
                input
            };
            let input = match (self.disabled, self.readonly) {
                (true, _) => input.disabled(),
                (_, true) => input.readonly(),
                _ => input,
            };
            return input.value(value).render();
        }
        let shown = Date::parse(value).map(|d| d.long(ui.strings));
        let id = crate::field_id(self.name);
        if self.disabled || self.readonly {
            let face = html! {
                (Icon::Calendar)
                span class=[shown.is_none().then_some("lui-date-picker-empty")] {
                    @if let Some(s) = &shown { (s) } @else { (ui.text(Text::PickDate)) }
                }
            };
            return html! {
                div class="lui-field lui-date-picker" {
                    label for=(id) { (self.label) }
                    (Button::new(ui.caps, self.label).id(&id).class("lui-date-picker-trigger").body(face).disabled())
                    @if self.readonly && !value.is_empty() { (crate::input::keep(self.name, value)) }
                }
            };
        }
        if let Some(end) = self.end {
            return self.range_field(end, value);
        }
        let mut calendar = ui.calendar(self.name).radio().value(value);
        if let Some(m) = self.min {
            calendar = calendar.min(m);
        }
        if let Some(m) = self.max {
            calendar = calendar.max(m);
        }
        if let Some(off) = self.off {
            calendar = calendar.disabled_dates(off.0);
        }
        if self.required {
            calendar = calendar.required();
        }
        let panel = format!("{id}-calendar");
        // A month link comes back with the calendar in the page: a popover cannot arrive open.
        let browsing = ui.param(&format!("month.{}", self.name)).is_some();
        let popover = ui.has(Cap::Popover) && !browsing;
        let anchor = ui.has(Cap::Anchor);
        let face = html! {
            (Icon::Calendar)
            span class=[shown.is_none().then_some("lui-date-picker-empty")] {
                @if let Some(s) = &shown { (s) } @else { (ui.text(Text::PickDate)) }
            }
            span class="lui-date-picker-chevron" { (Icon::ChevronDown) }
        };
        let legend = format!("{id}-label");
        let text = html! { (self.label) @if self.required { " *" } };
        html! {
            div class="lui-field lui-date-picker" role=[(!popover).then_some("group")]
                aria-labelledby=[(!popover).then_some(&legend)] {
                @if popover {
                    label for=(id) { (text) }
                    div class="lui-date-picker-anchor" style=[anchor.then(|| format!("anchor-name: --{panel}"))] {
                        (Button::new(ui.caps, self.label).id(&id).class("lui-date-picker-trigger").popovertarget(&panel).aria_haspopup("dialog").body(face))
                        div id=(panel) popover class="lui-date-picker-panel"
                            style=[anchor.then(|| format!("position-anchor: --{panel}; position-area: bottom span-right"))] {
                            (calendar)
                        }
                    }
                } @else {
                    span id=(legend) class="lui-date-picker-label" { (text) }
                    (calendar)
                }
            }
        }
    }
}

impl DatePicker<'_> {
    /// A range: the label, the two days picked so far (posted as hidden fields) and the
    /// calendar in the page.
    fn range_field(&self, end: &str, start: &str) -> Markup {
        let ui = self.ui;
        let mut calendar = ui.calendar(self.name).range(end);
        if let Some(m) = self.min {
            calendar = calendar.min(m);
        }
        if let Some(m) = self.max {
            calendar = calendar.max(m);
        }
        if let Some(off) = self.off {
            calendar = calendar.disabled_dates(off.0);
        }
        let last = ui.param(end).unwrap_or("");
        let legend = crate::field_id(self.name) + "-label";
        let day = |v: &str| Date::parse(v).map(|d| d.long(ui.strings));
        html! {
            div class="lui-field lui-date-picker" role="group" aria-labelledby=(legend) {
                span id=(legend) class="lui-date-picker-label" { (self.label) @if self.required { " *" } }
                p class="lui-date-picker-range" {
                    span { (ui.text(Text::StartDate)) ": " @if let Some(d) = day(start) { (d) } @else { "\u{2014}" } }
                    span { (ui.text(Text::EndDate)) ": " @if let Some(d) = day(last) { (d) } @else { "\u{2014}" } }
                }
                @if !start.is_empty() { (crate::input::keep(self.name, start)) }
                @if !last.is_empty() { (crate::input::keep(end, last)) }
                (calendar)
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. shadcn DatePicker: an outline
/// button with a calendar icon, 15rem wide, the Calendar in a PopoverContent (p-0, no border).
pub const CSS: &str = r#"
/* The trigger takes the Select trigger's look (input.rs and select.rs): the field border,
   no shadow, --lui-control-h tall, a calendar icon, the date or a muted prompt, a chevron at
   the end. The popover holds the calendar, raised. */
.lui-date-picker-anchor { position: relative; }
.lui-date-picker-label { font-size: var(--lui-text-2); line-height: 1.25rem; font-weight: 500; }
.lui-date-picker > .lui-calendar { justify-self: start; }
.lui-date-picker-trigger.lui-button {
  width: min(15rem, 100%); justify-content: flex-start; gap: var(--lui-space-2); padding-inline: 0.75rem;
  font-weight: 400; background: transparent; border-color: var(--lui-input); box-shadow: none;
}
.lui-date-picker-trigger > .lui-icon:first-child { color: var(--lui-muted); }
.lui-date-picker-chevron { display: inline-flex; margin-inline-start: auto; color: var(--lui-muted); }
.lui-date-picker-empty { color: var(--lui-muted); }
.lui-date-picker-panel { margin: 0; margin-top: var(--lui-space-1); padding: 0; border: 0; background: none; overflow: visible; }
.lui-date-picker-panel > .lui-calendar { width: 17.5rem; box-shadow: var(--lui-shadow-md), var(--lui-highlight); }
.lui-date-picker-range { display: flex; flex-wrap: wrap; gap: var(--lui-space-1) var(--lui-space-4); margin: 0; color: var(--lui-muted); font-size: var(--lui-text-2); line-height: var(--lui-leading-2); }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Caps;

    #[test]
    fn with_popover_support_a_labelled_button_opens_a_calendar_popover() {
        let ui = Ui::from(Caps::all());
        let m = ui
            .date_picker("due", "Due date")
            .value("2026-09-24")
            .render()
            .into_string();
        assert!(m.contains(r#"<label for="f-due">Due date</label>"#), "{m}");
        assert!(
            m.contains(r#"popovertarget="f-due-calendar""#)
                && m.contains(r#"id="f-due-calendar" popover"#),
            "{m}"
        );
        assert!(m.contains("24 September 2026"), "{m}");
        assert!(
            m.contains(r#"type="radio" name="due" value="2026-09-24" checked"#),
            "{m}"
        );
        assert!(!m.contains(r#"role="group""#), "{m}");
    }

    #[test]
    fn without_popover_support_the_calendar_sits_in_a_labelled_group_instead() {
        let m = Ui::from(Caps::NONE)
            .date_picker("due", "Due date")
            .render()
            .into_string();
        assert!(!m.contains("popover"), "{m}");
        assert!(
            m.contains(r#"role="group""#) && m.contains(r#"aria-labelledby="f-due-label""#),
            "{m}"
        );
        assert!(
            m.contains(r#"<span id="f-due-label" class="lui-date-picker-label">Due date</span>"#),
            "{m}"
        );
        assert!(m.contains(r#"type="radio" name="due""#), "{m}");
    }

    #[test]
    fn the_value_comes_from_the_query_unless_set_and_required_marks_the_label() {
        let ui = Ui::from_request("/", "due=2026-01-05", "");
        let m = ui.date_picker("due", "Due").render().into_string();
        assert!(
            m.contains("5 January 2026") && m.contains("2026-01-05"),
            "{m}"
        );
        let m = ui
            .date_picker("due", "Due")
            .value("2026-02-06")
            .required()
            .render()
            .into_string();
        assert!(
            m.contains("6 February 2026") && !m.contains("5 January"),
            "{m}"
        );
        assert!(m.contains("Due *"), "{m}");
    }

    #[test]
    fn native_is_a_date_input_and_disabled_or_readonly_post_nothing_or_the_value() {
        let ui = Ui::from(Caps::all());
        let native = ui
            .date_picker("due", "Due")
            .min("2026-01-01")
            .native()
            .render()
            .into_string();
        assert!(
            native.contains(r#"type="date""#)
                && native.contains(r#"min="2026-01-01""#)
                && !native.contains("popover"),
            "{native}"
        );
        let off = ui
            .date_picker("due", "Due")
            .value("2026-09-24")
            .disabled()
            .render()
            .into_string();
        assert!(
            off.contains("24 September 2026")
                && off.contains("disabled")
                && !off.contains(r#"type="hidden""#)
                && !off.contains("popover"),
            "{off}"
        );
        let ro = ui
            .date_picker("due", "Due")
            .value("2026-09-24")
            .readonly()
            .render()
            .into_string();
        assert!(
            ro.contains(r#"type="hidden" name="due" value="2026-09-24""#),
            "{ro}"
        );
    }
}
