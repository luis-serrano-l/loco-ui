//! # Counter
//!
//! The canonical "click me" demo done with a form round trip. State lives on the server
//! (here: a cookie), no script.
//!
//! **Platform features:** `<form method="post">`, `<button name value>` so one form carries
//! several actions, Post/Redirect/Get, and `view-transition-name` (only when `Caps` says the
//! browser has view transitions) so the number morphs instead of flashing. With
//! `.min()`, `.max()`, `.step()` and `.typed()` the buttons are `disabled` at `min` and `max`, step by `step`, and the
//! value can be typed into an `<input type="number">` with `min`, `max`, `step` (baseline 2015), posted
//! with `op=set`; the browser refuses a typed value off the bounds or the step.
//!
//! **Accessibility:** native buttons named "increment" and "decrement" (disabled at the
//! bounds), a labelled number field, the bounds in text. Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** change the number without a round trip; each step is
//! a POST and a redirect.
//!
//! **Fallback:** without view transitions the page simply reloads.
//!
//! **Enhanced:** the form is a swap root (`data-lui="swap"`), so with the [`crate::enhance`]
//! script each click is a background POST and only the form is replaced; its buttons are
//! disabled until the answer lands, so a double click counts once.
//!
//! **Finding:** without the script every click is a full navigation, and a click that lands
//! while the page unloads is dropped. There is no optimistic update and no offline behaviour.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let plain = ui.counter("/counter", 3);
//! let bounded = ui.counter("/counter", 10).min(0).max(10).step(2).typed();
//! let html = bounded.render().into_string();
//! assert!(html.contains("value=\"inc\" aria-label=\"increment\" disabled"));
//! assert!(html.contains("name=\"value\" type=\"number\" value=\"10\" min=\"0\" max=\"10\" step=\"2\""));
//! // The handler's half: the same counter applies the posted `op`, clamped to its bounds.
//! assert_eq!(bounded.apply("dec", None), 8.0);
//! # let _ = plain;
//!
//! // The same in `lui!`:
//! let same = lui! { Counter("/counter", 10) min=0 max=10 step=2 typed; };
//! assert_eq!(same.into_string(), bounded.render().into_string());
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::{Strings, Text};
use crate::input::Input;
use crate::props::{Key, Prop, PropKind};
use crate::{Cap, Caps, Icon, Ui, enhance};

/// A number with buttons posting `op` to `action`, made by [`Ui::counter`]. Unbounded and
/// stepping by 1 unless told otherwise.
///
/// **Setters.** Values and items: `.min(..)`, `.max(..)`, `.step(..)`; switches: `.typed()`,
/// `.disabled()`, `.readonly()`.
#[derive(Clone, Debug)]
pub struct Counter<'a> {
    caps: Caps,
    strings: &'static Strings,
    action: &'a str,
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
    typed: bool,
    disabled: bool,
    readonly: bool,
}

impl Counter<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves between the decrement, increment and reset buttons and the value field.",
        ),
        Key::new(
            "Enter / Space",
            "Presses the focused button, which posts the change.",
        ),
        Key::new(
            "ArrowUp / ArrowDown",
            "Steps the typed value up or down by the step before it is set.",
        ),
        Key::new("Enter", "Sets the typed value, as the Set button does."),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("min", PropKind::Number, "min: impl Into<f64>")
            .attr("min")
            .doc("Lowest value."),
        Prop::new("max", PropKind::Number, "max: impl Into<f64>")
            .attr("max")
            .doc("Highest value."),
        Prop::new("step", PropKind::Number, "step: impl Into<f64>")
            .default("1")
            .attr("step")
            .doc("How far one click moves (above 0)."),
        Prop::new("typed", PropKind::Switch, "")
            .doc("A number field and a Set button posting `op=set&value=n`."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("Every button and the number field disabled."),
        Prop::new("readonly", PropKind::Switch, "")
            .doc("The value shown, its buttons off and the number field `readonly`."),
    ];
}

impl Ui {
    /// A counter showing `value`, its buttons posting to `action`.
    pub fn counter<'a>(&self, action: &'a str, value: impl Into<f64>) -> Counter<'a> {
        Counter {
            caps: self.caps,
            strings: self.strings,
            action,
            value: value.into(),
            min: None,
            max: None,
            step: 1.0,
            typed: false,
            disabled: false,
            readonly: false,
        }
    }
}

impl Counter<'_> {
    /// Lowest value; the decrement button is disabled at it.
    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self
    }

    /// Highest value; the increment button is disabled at it.
    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = Some(max.into());
        self
    }

    /// How far one click moves (above 0; 1 when given 0 or less).
    pub fn step(mut self, step: impl Into<f64>) -> Self {
        let step = step.into();
        self.step = if step > 0.0 { step } else { 1.0 };
        self
    }

    /// Every button and the number field disabled: shown, not changeable, nothing posted.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// The value shown and its buttons off; the number field is `readonly`, so it still posts.
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    /// A number field and a Set button posting `op=set&value=n`.
    pub fn typed(mut self) -> Self {
        self.typed = true;
        self
    }

    /// The value after a posted `op` (`inc`, `dec`, `reset`, `set` with the typed value),
    /// clamped to the bounds: the handler's half of the component.
    pub fn apply(&self, op: &str, typed: Option<f64>) -> f64 {
        let (value, min, max) = (self.value, self.min, self.max);
        let next = match op {
            "inc" => value + self.step,
            "dec" => value - self.step,
            "set" => typed.filter(|t| t.is_finite()).unwrap_or(value),
            _ => min.unwrap_or(0.0).max(0.0).min(max.unwrap_or(f64::MAX)),
        };
        let next = crate::input::tidy(next, self.step, min.unwrap_or(0.0));
        next.clamp(min.unwrap_or(f64::MIN), max.unwrap_or(f64::MAX))
    }
}

impl Render for Counter<'_> {
    fn render(&self) -> Markup {
        let Counter {
            strings: _,
            caps,
            action,
            value,
            min,
            max,
            step,
            typed,
            disabled,
            readonly,
        } = *self;
        let off = disabled || readonly;
        let vt = caps
            .has(Cap::ViewTransitions)
            .then_some("view-transition-name: lui-counter");
        let at_min = min.is_some_and(|m| value <= m);
        let at_max = max.is_some_and(|m| value >= m);
        let value_text = value.to_string();
        let typed_id = format!("{}-value", enhance::swap_id("lui-counter", action));
        let op = |op: &'static str, stop: bool| {
            let button = Button::new(caps, "").name("op").value(op);
            if off || stop {
                button.disabled()
            } else {
                button
            }
        };
        html! {
            form id=(enhance::swap_id("lui-counter", action)) data-lui="swap" class="lui-counter" method="post" action=(action) {
                // Enter in the typed value sets it, not the decrement button before it.
                @if typed && !off { (crate::button::default_submit(Some(("op", "set")))) }
                (op("dec", at_min).icon_only().aria_label(self.strings.get(Text::Decrement)).body(html! { (Icon::Minus) }))
                output style=[vt] { (self.strings.digits(value)) }
                (op("inc", at_max).icon_only().aria_label(self.strings.get(Text::Increment)).body(html! { (Icon::Plus) }))
                (op("reset", false).ghost().body(html! { (self.strings.get(Text::Reset)) }))
                @if typed {
                    @let field = Input::number_within("value", self.strings.get(Text::Value), min, max).hide_label().class("lui-counter-input").step(step).inputmode("numeric").id(&typed_id).value(&value_text);
                    @let field = if disabled { field.disabled() } else if readonly { field.readonly() } else { field };
                    (field)
                    (op("set", false).body(html! { (self.strings.get(Text::Set)) }))
                }
                @if min.is_some() || max.is_some() {
                    small class="lui-counter-bounds" {
                        @match (min, max) {
                            (Some(a), Some(b)) => { (self.strings.fill(Text::FromTo, &[&self.strings.digits(a), &self.strings.digits(b)])) },
                            (Some(a), None) => { (self.strings.fill(Text::AtLeast, &[&self.strings.digits(a)])) },
                            (None, Some(b)) => { (self.strings.fill(Text::AtMost, &[&self.strings.digits(b)])) },
                            _ => {},
                        }
                        @if step != 1.0 { (self.strings.fill(Text::InStepsOf, &[&self.strings.digits(step)])) }
                    }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-counter { display: inline-flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space); }
.lui-counter output { min-width: 3ch; text-align: center; font-size: var(--lui-text-6); font-weight: 600; font-variant-numeric: tabular-nums; }
.lui-counter-input { width: 6em; }
.lui-counter-bounds { flex-basis: 100%; color: var(--lui-muted); font-size: var(--lui-text-2); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// The attributes of the first submit button in `m`: what Enter in a field presses.
    fn first_submit(m: &str) -> &str {
        let at = m.find("<button type=\"submit\"").expect("a submit button");
        &m[at..at + m[at..].find('>').unwrap()]
    }

    #[test]
    fn enter_in_the_typed_value_sets_it() {
        let ui = Ui::from(Caps::NONE);
        let m = ui.counter("/c", 3).min(0).typed().render().into_string();
        assert!(first_submit(&m).contains(r#"name="op" value="set""#), "{m}");
        let off = ui
            .counter("/c", 3)
            .typed()
            .disabled()
            .render()
            .into_string();
        assert!(!off.contains(r#"class="lui-sr""#), "{off}");
    }

    #[test]
    fn bounds_disable_and_clamp() {
        let ui = Ui::from(Caps::NONE);
        let c = |n: i32| ui.counter("/c", n).min(0).max(10).step(3);
        let m = c(0).render().into_string();
        assert!(
            m.contains("aria-label=\"decrement\" disabled")
                && !m.contains("aria-label=\"increment\" disabled"),
            "{m}"
        );
        assert!(m.contains("0 to 10, in steps of 3"));
        assert_eq!(c(9).apply("inc", None), 10.0);
        assert_eq!(c(1).apply("dec", None), 0.0);
        assert_eq!(c(5).apply("set", Some(42.0)), 10.0);
        assert_eq!(c(5).apply("reset", None), 0.0);
        assert_eq!(ui.counter("/c", 9).min(5).apply("reset", None), 5.0);
    }
}
