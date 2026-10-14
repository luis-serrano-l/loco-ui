//! # Number
//!
//! A number field with its unit beside it and buttons that step it: a quantity, a price, a
//! weight. The value is an `f64` written the way Rust writes it, so `2.0` shows as `2` and
//! `0.1 + 0.2` as `0.3`.
//!
//! After the Origin UI number inputs ("with chevrons" and "with add-ons"): one bordered row
//! holding the field, a gray `.prefix(..)`/`.suffix(..)` addon at either end, and
//! `.steppers()` as two stacked chevron buttons at the end.
//!
//! **Platform features:** `<input type="number">` with `min`, `max` and `step` (baseline
//! 2015), so the browser refuses a value off the bounds or the step and its arrow keys step
//! it; the steppers are `<button name value formnovalidate>` (baseline 2015) posting the
//! value one step down or up; `:has()` (Chrome 105, Firefox 121, Safari 15.4) draws the row's
//! focus ring.
//!
//! **Accessibility:** a labelled number field; a text addon is read out with it
//! (`aria-describedby`), an icon is decoration; the steppers are named "increment" and
//! "decrement" and disabled at the bounds. Checked by axe-core in headless Firefox on every
//! demo route, both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** step in place. Without the script a stepper
//! submits the form around it with the stepped value as the first `name` posted (the buttons
//! come before the field in the markup and show after it), which [`crate::Posted::get`] and
//! [`Ui::param`] read; the page answers with the new value. With the enhancement script a
//! stepper calls `stepUp()`/`stepDown()` on the field and nothing is sent. A stepper is a
//! submit button, so in a form with several fields put the number after the field Enter is
//! pressed in, or give the form its own submit button first (the browser's Enter presses
//! the first one).
//!
//! **Fallback:** none needed: a native number field. Without `:has()` the field keeps its
//! own focus outline inside the row.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The value is the query's `qty`, or `.value(..)` (a saved one).
//! let ui = Ui::from_request("/cart", "qty=2", "");
//! let m = ui.number("qty", "Quantity").min(1).max(10).render().into_string();
//! assert!(m.contains(r#"name="qty" type="number" value="2" min="1" max="10""#));
//! // The same in `lui!`:
//! let same = lui! { Number("qty", "Quantity") min=1 max=10; };
//! assert_eq!(same.into_string(), m);
//! // A price with its currency and steppers that post 2.5 and 1.5.
//! let m = ui.number("price", "Price").value(2).step(0.5).min(0).prefix("$").steppers();
//! let html = m.render().into_string();
//! assert!(html.contains(r#"name="price" value="2.5""#) && html.contains(r#"name="price" value="1.5""#));
//! assert!(html.contains(r#"<span id="f-price-prefix" class="lui-input-group-prefix">$</span>"#));
//! let same = lui! { Number("price", "Price") value=2 step=0.5 min=0 prefix="$" steppers; };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render};

use crate::Ui;
use crate::icon::Glyph;
use crate::input::{FieldKind, Input};
use crate::props::{Prop, PropKind};

/// A number field, made by [`Ui::number`]. Unbounded, stepping by 1, unless told otherwise.
///
/// **Setters.** Values and items: `.min(..)`, `.max(..)`, `.step(..)`, `.value(..)`,
/// `.prefix(..)`, `.suffix(..)`, `.help(..)`, `.error(..)`, `.placeholder(..)`, `.id(..)`;
/// switches: `.steppers()`, `.required()`, `.disabled()`, `.readonly()`, `.hide_label()`.
#[derive(Clone, Debug)]
pub struct Number<'a> {
    input: Input<'a>,
    min: Option<f64>,
    max: Option<f64>,
    value: Option<f64>,
}

impl Number<'_> {
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
            .doc("How far one step moves; the browser accepts only values on a step."),
        Prop::new("value", PropKind::Number, "value: impl Into<f64>")
            .attr("value")
            .doc("The value, instead of the query's `name`."),
        Prop::new("prefix", PropKind::Value, "addon: impl Into<Glyph<'a>>")
            .doc("An addon joined to the field's start (`\"$\"`)."),
        Prop::new("suffix", PropKind::Value, "addon: impl Into<Glyph<'a>>")
            .doc("An addon joined to the field's end (`\"kg\"`)."),
        Prop::new("steppers", PropKind::Switch, "").doc(
            "Buttons stepping the value down and up: a post without script, `stepUp()` with it.",
        ),
        Prop::new("required", PropKind::Switch, "")
            .attr("required")
            .doc("The `required` attribute, and a `*` after the label."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("`disabled` and `aria-disabled`: shown greyed and not posted."),
        Prop::new("readonly", PropKind::Switch, "")
            .doc("Shown and posted, not editable; the steppers are off."),
        Prop::new("hide_label", PropKind::Switch, "")
            .doc("Only the control, the label kept as its `aria-label`."),
        Prop::new("help", PropKind::Value, "help: &'a str").doc("Help text under the field."),
        Prop::new("error", PropKind::Value, "message: &'a str")
            .doc("A server message under the field."),
        Prop::new("placeholder", PropKind::Value, "placeholder: &'a str")
            .attr("placeholder")
            .doc("Placeholder text."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The control's id, `f-<name>` by default."),
    ];
}

impl Ui {
    /// A number field named `name` under `label`, at the query's `name` unless
    /// [`Number::value`] says otherwise.
    pub fn number<'a>(&'a self, name: &'a str, label: &'a str) -> Number<'a> {
        Number {
            input: self
                .input(name, label)
                .value(self.param(name).unwrap_or("")),
            min: None,
            max: None,
            value: None,
        }
    }
}

impl<'a> Number<'a> {
    /// Lowest value; the down stepper is off at it.
    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self
    }

    /// Highest value; the up stepper is off at it.
    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = Some(max.into());
        self
    }

    /// How far one step moves (1 by default); the browser accepts only values on a step
    /// counted from `min`.
    pub fn step(mut self, step: impl Into<f64>) -> Self {
        self.input = self.input.step(step);
        self
    }

    /// The value, instead of the query's `name`: a saved one.
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// An addon joined to the field's start (`"$"`), read out with it when it is text.
    pub fn prefix(mut self, addon: impl Into<Glyph<'a>>) -> Self {
        self.input = self.input.prefix(addon);
        self
    }

    /// An addon joined to the field's end (`"kg"`), read out with it when it is text.
    pub fn suffix(mut self, addon: impl Into<Glyph<'a>>) -> Self {
        self.input = self.input.suffix(addon);
        self
    }

    /// Buttons stepping the value down and up, off at the bounds: each posts the form with
    /// the stepped value without script, and steps the field in place with it.
    pub fn steppers(mut self) -> Self {
        self.input = self.input.steppers();
        self
    }

    /// The `required` attribute, and a `*` after the label.
    pub fn required(mut self) -> Self {
        self.input = self.input.required();
        self
    }

    /// `disabled` and `aria-disabled`: shown greyed and not posted.
    pub fn disabled(mut self) -> Self {
        self.input = self.input.disabled();
        self
    }

    /// Shown and posted, not editable; the steppers are off.
    pub fn readonly(mut self) -> Self {
        self.input = self.input.readonly();
        self
    }

    /// Only the control, the label kept as its `aria-label`; help and error still follow.
    pub fn hide_label(mut self) -> Self {
        self.input = self.input.hide_label();
        self
    }

    /// Help text under the field.
    pub fn help(mut self, help: &'a str) -> Self {
        self.input = self.input.help(help);
        self
    }

    /// A server message under the field; the control gets `aria-invalid`. An empty message
    /// is no error.
    pub fn error(mut self, message: &'a str) -> Self {
        self.input = self.input.error(message);
        self
    }

    /// Placeholder text.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.input = self.input.placeholder(placeholder);
        self
    }

    /// The control's id, `f-<name>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.input = self.input.id(id);
        self
    }
}

impl Render for Number<'_> {
    fn render(&self) -> Markup {
        let mut field = self.input.0.clone();
        field.kind = FieldKind::Number {
            min: self.min,
            max: self.max,
        };
        let text = self.value.map(|v| v.to_string());
        if let Some(t) = &text {
            field.value = t;
        }
        field.render()
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. The row, the addons and
/// the steppers are [`crate::input`]'s `.lui-input-group`, shared with every field.
pub const CSS: &str = r#"
/* After the Origin UI number inputs: the row is input.rs's .lui-input-group; a number reads
   in tabular figures, aligned to the end like a column of amounts. */
.lui-input-group > input[type=number] { font-variant-numeric: tabular-nums; text-align: end; }
"#;
