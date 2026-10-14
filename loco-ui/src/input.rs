//! # Input
//!
//! One labelled field on its own: the label, the control, help text, a character counter and
//! the server's error, tied together with `aria-describedby`. `ui.checkbox`, `ui.switch` and
//! `ui.radio_group` are the same for choices. [`crate::form`] renders its fields with this
//! file, so a field looks and behaves the same inside a form built with `ui.form` and in a
//! form a route writes by hand.
//!
//! The look follows Radix Themes (TextField, TextArea, Checkbox, Switch, RadioGroup and
//! RadioCards): a gray-7 border with no shadow, `.leading(..)`/`.trailing(..)` slots inside
//! the border, help in gray-11 and the error after a warning icon, a drawn checkbox, radio and
//! switch, and `.cards()` for a radio group of selectable cards. A text field takes Radix's
//! sizes (`.size(..)`, 1 to 3) and its surface, soft or ghost variant (`.variant(..)`).
//!
//! **Platform features:** constraint validation (`required`, `pattern`, `min`, `max`,
//! `maxlength`, `type=email`, baseline 2015; `type=date`/`time`/`datetime-local`, Chrome 20, Firefox 57 (93 for `datetime-local`),
//! Safari 14.1); `:user-invalid` (baseline 2023) so a field is not red before it is touched;
//! `<output for>` counts characters; `role="switch"` on a checkbox (ARIA 1.2) drawn as a
//! track and thumb with `appearance: none`; `<fieldset>` + `<legend>` for a radio group;
//! `appearance: none` checkboxes and radios with the tick drawn in CSS; `:has()` (Chrome 105,
//! Firefox 121, Safari 15.4) for the slot wrapper's focus ring and the picked card; a
//! container query (Chrome 105, Firefox 110, Safari 16) lays radio cards 1, 2 or 3 across.
//! `.gradient_border()` draws a text field's or textarea's border with `--lui-gradient-ring`, a
//! padding-box layer over a border-box one, behind `@supports` for
//! `linear-gradient(in oklch, ..)` (Chrome 111, Firefox 127, Safari 16.2); nothing moves.
//!
//! **Accessibility:** every field has a `<label for>`; help, counter and error are joined by
//! `aria-describedby`, a server error sets `aria-invalid` and is `role="alert"`; the switch is
//! a checkbox with `role="switch"`. Checked by axe-core in headless Firefox on every demo
//! route, both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** keep the counter live while typing (the
//! enhancement script does; without it the counter shows the length the server rendered).
//!
//! **Fallback:** none needed: every control is a native one, and the switch is still a
//! checkbox where `appearance: none` is not supported. Without `:has()` a slotted field keeps
//! the input's own focus outline and a picked card shows no ring; without container queries
//! radio cards stay one per row. A `.gradient_border()` field keeps its
//! plain border where `in oklch` gradients are not supported.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let name = ui.input("name", "Name").render().into_string();
//! assert!(name.contains(r#"<label for="f-name">Name</label>"#) && name.contains(r#"type="text""#));
//! // The same in `lui!`:
//! let same = lui! { Input("name", "Name"); };
//! assert_eq!(same.into_string(), name);
//! // An opt-in gradient border, a class on the control.
//! let key = ui.input("key", "API key").gradient_border().render().into_string();
//! assert!(key.contains(r#"class="lui-input-gradient-border""#));
//! assert_eq!(lui! { Input("key", "API key") gradient_border; }.into_string(), key);
//! // Setters as on form fields, and one per input type.
//! let email = ui.input("email", "Email").email().required().value("ada@x.org")
//!     .help("We never share it.").error("Already taken.");
//! let email = email.render().into_string();
//! assert!(email.contains(r#"aria-describedby="f-email-help f-email-error""#));
//! assert!(email.contains(r#"aria-invalid="true""#) && email.contains(r#"type="email""#));
//! let on = ui.switch("digest", "Weekly digest").checked(true).render().into_string();
//! assert!(on.contains(r#"role="switch""#) && on.contains("checked"));
//! let plan = ui.radio_group("plan", "Plan").option("free", "Free").option("pro", "Pro").value("pro");
//! let plan = plan.render().into_string();
//! assert!(plan.contains("<legend>Plan</legend>") && plan.contains(r#"value="pro" checked"#));
//! // Text or an icon inside the border; text is read out with the field.
//! let site = ui.input("site", "Website").leading("https://").trailing(Icon::Search);
//! let site = site.render().into_string();
//! assert!(site.contains(r#"<div class="lui-input-slots"><span id="f-site-leading""#));
//! assert!(site.contains(r#"aria-describedby="f-site-leading""#));
//! assert_eq!(lui! { Input("site", "Website") leading="https://" trailing=(Icon::Search); }.into_string(), site);
//! // A radio group of cards, 1, 2 or 3 across by the container's width.
//! let tier = ui.radio_group("tier", "Tier").cards()
//!     .option("hobby", "Hobby").description("One project")
//!     .option("team", "Team").description("Unlimited projects").value("team");
//! let tier = tier.render().into_string();
//! assert!(tier.contains("lui-radio-cards") && tier.contains(r#"aria-describedby="f-tier-1-description""#));
//! let same = lui! { RadioGroup("tier", "Tier") cards value="team" {
//!     option "hobby" "Hobby" description="One project";
//!     option "team" "Team" description="Unlimited projects";
//! } };
//! assert_eq!(same.into_string(), tier);
//! // A small soft field, the size and variant on its root.
//! let q = ui.input("q", "Search").size(1).variant(Variant::Soft).render().into_string();
//! assert!(q.starts_with(r#"<div class="lui-field" data-size="1" data-variant="soft">"#));
//! assert_eq!(lui! { Input("q", "Search") size=1 variant="soft"; }.into_string(), q);
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::{Strings, Text};
use crate::icon::{Glyph, Icon};
use crate::props::{Prop, PropKind};
use crate::select::SelectOption;
use crate::variant::{Size, Variant};
use crate::{Caps, Ui};

/// A bound of a number field: a number, or no bound. Any number converts (`0`, `2.5`), and
/// `None::<f64>` is no bound, so `.number(0, None)` has no maximum.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Bound(pub Option<f64>);

impl From<f64> for Bound {
    fn from(n: f64) -> Self {
        Bound(Some(n))
    }
}

impl From<i32> for Bound {
    fn from(n: i32) -> Self {
        Bound(Some(f64::from(n)))
    }
}

impl From<i16> for Bound {
    fn from(n: i16) -> Self {
        Bound(Some(f64::from(n)))
    }
}

impl From<u32> for Bound {
    fn from(n: u32) -> Self {
        Bound(Some(f64::from(n)))
    }
}

impl From<i64> for Bound {
    #[allow(clippy::cast_precision_loss)]
    fn from(n: i64) -> Self {
        Bound(Some(n as f64))
    }
}

impl From<Option<f64>> for Bound {
    fn from(n: Option<f64>) -> Self {
        Bound(n)
    }
}

/// `n` rounded to the decimals `step` and `base` are written with, so `0.1 + 0.2` shows as
/// `0.3`: what a stepper posts and a number field shows.
pub(crate) fn tidy(n: f64, step: f64, base: f64) -> f64 {
    let decimals = |x: f64| {
        let s = x.to_string();
        s.find('.').map_or(0, |at| s.len() - at - 1).min(10)
    };
    let scale = 10f64.powi(decimals(step).max(decimals(base)) as i32);
    (n * scale).round() / scale
}

/// Input type of a field.
#[derive(Clone, Debug)]
pub(crate) enum FieldKind<'a> {
    Text,
    Search,
    Email,
    Password,
    Tel,
    Url,
    Number {
        min: Option<f64>,
        max: Option<f64>,
    },
    Pattern {
        pattern: &'a str,
        hint: &'a str,
    },
    Textarea {
        rows: u8,
    },
    File {
        accept: &'a str,
    },
    Date {
        min: &'a str,
        max: &'a str,
    },
    Time {
        min: &'a str,
        max: &'a str,
    },
    DateTime {
        min: &'a str,
        max: &'a str,
    },
    Month {
        min: &'a str,
        max: &'a str,
    },
    Week {
        min: &'a str,
        max: &'a str,
    },
    /// A `<select>`, drawn by [`crate::select`].
    Select(Vec<SelectOption<'a>>),
    /// Radio buttons, drawn by [`RadioGroup`].
    Radio(Vec<Pick<'a>>),
    /// Checkboxes, drawn by [`crate::checkbox_group`].
    Checkboxes(Vec<Pick<'a>>),
    /// A slider, drawn by [`crate::range`].
    Range {
        min: f64,
        max: f64,
    },
    /// A colour picker, drawn by [`crate::color`].
    Color,
    Checkbox,
    Switch,
    Hidden,
    /// Markup a form holds among its fields (a switch, a date picker, a select).
    Markup(Markup),
}

/// One option of a form's `<select>`: what it posts and what it shows. A `&str` is both;
/// a `(value, label)` pair shows the label and posts the value (a row's id, say).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Choice<'a> {
    /// What the form posts.
    pub value: &'a str,
    /// What the visitor sees.
    pub label: &'a str,
}

impl<'a> From<&'a str> for Choice<'a> {
    fn from(text: &'a str) -> Self {
        Choice {
            value: text,
            label: text,
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Choice<'a> {
    fn from((value, label): (&'a str, &'a str)) -> Self {
        Choice { value, label }
    }
}

/// One choice of a radio or checkbox group: what it posts, its label, a line under it and
/// whether it can be picked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Pick<'a> {
    pub(crate) value: &'a str,
    pub(crate) label: &'a str,
    pub(crate) description: Option<&'a str>,
    pub(crate) disabled: bool,
}

impl<'a> From<Choice<'a>> for Pick<'a> {
    fn from(c: Choice<'a>) -> Self {
        Pick {
            value: c.value,
            label: c.label,
            description: None,
            disabled: false,
        }
    }
}

/// One field with its current value and server-side error: what [`Ui::input`] builds and
/// what a [`crate::form::Form`] holds a list of.
#[derive(Clone, Debug)]
pub(crate) struct Field<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: &'a str,
    pub(crate) kind: FieldKind<'a>,
    pub(crate) value: &'a str,
    /// Every value, for a field that takes several (checkboxes, a multiple select); `value`
    /// is its first.
    pub(crate) values: Vec<&'a str>,
    pub(crate) error: Option<&'a str>,
    pub(crate) required: bool,
    pub(crate) help: Option<&'a str>,
    pub(crate) maxlength: Option<usize>,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) id: Option<&'a str>,
    pub(crate) multiple: bool,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
    pub(crate) optional: bool,
    pub(crate) extra: Extra<'a>,
    pub(crate) caps: Caps,
    pub(crate) strings: &'static Strings,
}

/// Attributes only some fields need, each set by the `Input` setter of the same name.
#[derive(Clone, Debug, Default)]
pub(crate) struct Extra<'a> {
    pub(crate) hide_label: bool,
    pub(crate) list: Option<&'a str>,
    pub(crate) autocomplete: Option<&'a str>,
    pub(crate) autofocus: bool,
    pub(crate) inputmode: Option<&'a str>,
    pub(crate) step: Option<f64>,
    pub(crate) aria_controls: Option<&'a str>,
    pub(crate) class: Option<&'a str>,
    pub(crate) form: Option<&'a str>,
    pub(crate) gradient_border: bool,
    pub(crate) leading: Option<Glyph<'a>>,
    pub(crate) trailing: Option<Glyph<'a>>,
    pub(crate) prefix: Option<Glyph<'a>>,
    pub(crate) suffix: Option<Glyph<'a>>,
    pub(crate) addon: Option<Markup>,
    pub(crate) reveal: bool,
    pub(crate) steppers: bool,
    pub(crate) cards: bool,
    /// Ids of elements outside the field that describe it (a component's own help).
    pub(crate) described: Option<&'a str>,
    pub(crate) width: Option<Size>,
    pub(crate) size: Size,
    pub(crate) variant: Option<Variant>,
}

impl<'a> Field<'a> {
    pub(crate) fn new(name: &'a str, label: &'a str, kind: FieldKind<'a>) -> Self {
        Field {
            name,
            label,
            kind,
            value: "",
            values: Vec::new(),
            error: None,
            required: false,
            help: None,
            maxlength: None,
            placeholder: None,
            id: None,
            multiple: false,
            disabled: false,
            readonly: false,
            optional: false,
            extra: Extra::default(),
            caps: Caps::NONE,
            strings: &Strings::ENGLISH,
        }
    }

    /// The same field speaking `ui`'s language and knowing its browser.
    pub(crate) fn of(mut self, ui: &Ui) -> Self {
        self.caps = ui.caps;
        self.strings = ui.strings;
        self
    }

    /// Whether a person sees it (a review lists only these).
    pub(crate) fn shown(&self) -> bool {
        !matches!(self.kind, FieldKind::Hidden | FieldKind::Markup(_))
    }

    /// The control's id: `f-<name>` unless set.
    pub(crate) fn control_id(&self) -> String {
        self.id
            .map_or_else(|| format!("f-{}", self.name), str::to_string)
    }

    /// The marks after a label: the required star or the "(optional)" note.
    fn marks(&self) -> Markup {
        html! {
            @if self.required { " " (REQUIRED) }
            @if self.optional && !self.required { " " span class="lui-field-optional" { (self.strings.get(Text::Optional)) } }
        }
    }
}

/// A hidden input posting `value` as `name`: what a read-only control that HTML cannot make
/// read-only (a select, a checkbox, a slider) posts instead of itself.
pub(crate) fn keep(name: &str, value: &str) -> Markup {
    html! { input type="hidden" name=(name) value=(value); }
}

/// A labelled field, made by [`Ui::input`], [`Ui::checkbox`] or [`Ui::switch`].
///
/// **Setters.** Values and items: `.number(..)`, `.pattern(..)`, `.textarea(..)`, `.file(..)`,
/// `.date(..)`, `.time(..)`, `.datetime(..)`, `.month(..)`, `.week(..)`, `.help(..)`,
/// `.maxlength(..)`, `.value(..)`, `.error(..)`, `.placeholder(..)`, `.list(..)`,
/// `.autocomplete(..)`, `.inputmode(..)`, `.step(..)`, `.aria_controls(..)`, `.form(..)`,
/// `.class(..)`, `.id(..)`, `.leading(..)`, `.trailing(..)`, `.prefix(..)`, `.suffix(..)`,
/// `.addon_button(..)`, `.width(..)`, `.size(..)`, `.variant(..)`; switches: `.email()`,
/// `.password()`, `.tel()`, `.url()`, `.multiple()`, `.required()`, `.optional()`,
/// `.disabled()`, `.readonly()`, `.search()`, `.hide_label()`, `.autofocus()`,
/// `.gradient_border()`, `.reveal()`; from a condition: `.checked(bool)`.
#[derive(Clone, Debug)]
pub struct Input<'a>(pub(crate) Field<'a>);

impl Input<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("email", PropKind::Switch, "")
            .attr("type")
            .doc("`type=\"email\"`."),
        Prop::new("password", PropKind::Switch, "")
            .attr("type")
            .doc("`type=\"password\"`."),
        Prop::new("tel", PropKind::Switch, "")
            .attr("type")
            .doc("`type=\"tel\"`: a phone number, the phone keypad on a touch screen."),
        Prop::new("url", PropKind::Switch, "")
            .attr("type")
            .doc("`type=\"url\"`: the browser checks it is an absolute URL."),
        Prop::new(
            "number",
            PropKind::Value,
            "min: impl Into<Bound>, max: impl Into<Bound>",
        )
        .doc("A number from `min` to `max`, inclusive; `None::<f64>` for no bound."),
        Prop::new(
            "pattern",
            PropKind::Value,
            "pattern: &'a str, hint: &'a str",
        )
        .attr("pattern")
        .doc("Text that must match `pattern`."),
        Prop::new("textarea", PropKind::Number, "rows: u8").doc("Multi-line text, `rows` high."),
        Prop::new("file", PropKind::Value, "accept: &'a str").doc("A file picker."),
        Prop::new("date", PropKind::Value, "min: &'a str, max: &'a str")
            .doc("`type=\"date\"`, bounds as `YYYY-MM-DD`."),
        Prop::new("time", PropKind::Value, "min: &'a str, max: &'a str")
            .doc("`type=\"time\"`, bounds as `HH:MM`."),
        Prop::new("datetime", PropKind::Value, "min: &'a str, max: &'a str")
            .doc("`type=\"datetime-local\"`, bounds as `YYYY-MM-DDTHH:MM`."),
        Prop::new("month", PropKind::Value, "min: &'a str, max: &'a str")
            .doc("`type=\"month\"`, bounds as `YYYY-MM`."),
        Prop::new("week", PropKind::Value, "min: &'a str, max: &'a str")
            .doc("`type=\"week\"`, bounds as `YYYY-Www`."),
        Prop::new("multiple", PropKind::Switch, "")
            .attr("multiple")
            .doc("The file picker takes several files."),
        Prop::new("required", PropKind::Switch, "")
            .attr("required")
            .doc("The `required` attribute, and a `*` after the label (hidden from screen readers, which say \"required\")."),
        Prop::new("optional", PropKind::Switch, "")
            .doc("\"(optional)\" after the label, for a form whose fields are mostly required."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("`disabled` and `aria-disabled`: shown greyed and not posted."),
        Prop::new("readonly", PropKind::Switch, "")
            .doc("Shown and posted, not editable (`readonly`, or a disabled control and a hidden input where HTML has no `readonly`)."),
        Prop::new("help", PropKind::Value, "help: &'a str").doc("Help text under the field."),
        Prop::new("maxlength", PropKind::Number, "max: usize")
            .attr("maxlength")
            .doc("`maxlength`, counted in an `<output>` under the field."),
        Prop::new("value", PropKind::Value, "value: &'a str")
            .attr("value")
            .doc("The current value (ignored for files and passwords)."),
        Prop::new("checked", PropKind::Condition, "checked: bool")
            .attr("checked")
            .doc("Tick the checkbox or switch."),
        Prop::new("error", PropKind::Value, "message: &'a str")
            .doc("A server message under the field."),
        Prop::new("placeholder", PropKind::Value, "placeholder: &'a str")
            .attr("placeholder")
            .doc("Placeholder text."),
        Prop::new("search", PropKind::Switch, "")
            .attr("type")
            .doc("`type=\"search\"`."),
        Prop::new("hide_label", PropKind::Switch, "")
            .doc("Only the control, the label kept as its `aria-label`; help, counter and error still follow it."),
        Prop::new("list", PropKind::Value, "id: &'a str")
            .attr("list")
            .doc("`list`."),
        Prop::new("autocomplete", PropKind::Value, "value: &'a str")
            .attr("autocomplete")
            .doc("`autocomplete` (`\"off\"`, `\"email\"`, `\"new-password\"`)."),
        Prop::new("autofocus", PropKind::Switch, "")
            .attr("autofocus")
            .doc("`autofocus`."),
        Prop::new("inputmode", PropKind::Value, "mode: &'a str")
            .attr("inputmode")
            .doc("`inputmode` (`\"numeric\"`), the on-screen keyboard to show."),
        Prop::new("step", PropKind::Number, "step: impl Into<f64>")
            .attr("step")
            .doc("`step` of a number field."),
        Prop::new("aria_controls", PropKind::Value, "id: &'a str")
            .attr("aria-controls")
            .doc("`aria-controls`."),
        Prop::new("form", PropKind::Value, "id: &'a str")
            .attr("form")
            .doc("`form`."),
        Prop::new("class", PropKind::Value, "class: &'a str")
            .attr("class")
            .doc("A class on the control, for a component's part name."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The control's id, `f-<name>` by default."),
        Prop::new("gradient_border", PropKind::Switch, "")
            .doc("The border drawn with `--lui-gradient-ring`."),
        Prop::new("leading", PropKind::Value, "slot: impl Into<Glyph<'a>>")
            .doc("An icon or text inside the field's border, before the text."),
        Prop::new("trailing", PropKind::Value, "slot: impl Into<Glyph<'a>>")
            .doc("An icon or text inside the field's border, after the text."),
        Prop::new("prefix", PropKind::Value, "addon: impl Into<Glyph<'a>>")
            .doc("An addon joined to the field's start (`\"https://\"`, `\"$\"`), outside its border."),
        Prop::new("suffix", PropKind::Value, "addon: impl Into<Glyph<'a>>")
            .doc("An addon joined to the field's end (`\"kg\"`, `\".com\"`), outside its border."),
        Prop::new("addon_button", PropKind::Value, "button: Button<'a>")
            .doc("A button joined to the field's end (Copy, Send)."),
        Prop::new("reveal", PropKind::Switch, "")
            .doc("A password field's show/hide button, which only the enhancement script shows."),
        Prop::new(
            "width",
            PropKind::Choice { of: Size::NAMES },
            "width: impl Into<Size>",
        )
        .doc("A narrower control: 1 (8rem), 2 (16rem) or 3 (24rem); the container's width by default."),
        Prop::new(
            "size",
            PropKind::Choice { of: Size::NAMES },
            "size: impl Into<Size>",
        )
        .default("2")
        .doc("A text field's height and text: Radix's sizes 1 to 3."),
        Prop::new(
            "variant",
            PropKind::Choice { of: Variant::NAMES },
            "variant: Variant",
        )
        .default("surface")
        .doc("A text field's look: surface (a border), soft (a tint) or ghost (neither)."),
    ];
}

impl Ui {
    /// A single-line text field named `name` under `label`; the setters change its type.
    pub fn input<'a>(&self, name: &'a str, label: &'a str) -> Input<'a> {
        Input(Field::new(name, label, FieldKind::Text).of(self))
    }

    /// A checkbox posting `true` when ticked and nothing when not (so a `bool` with
    /// `#[serde(default)]` reads it), its label beside it.
    pub fn checkbox<'a>(&self, name: &'a str, label: &'a str) -> Input<'a> {
        Input(Field::new(name, label, FieldKind::Checkbox).of(self))
    }

    /// A checkbox drawn as an on/off switch, with `role="switch"`; posts like
    /// [`Ui::checkbox`].
    pub fn switch<'a>(&self, name: &'a str, label: &'a str) -> Input<'a> {
        Input(Field::new(name, label, FieldKind::Switch).of(self))
    }
}

impl<'a> Input<'a> {
    /// A number field whose bounds a component may not have (the counter's).
    pub(crate) fn number_within(
        name: &'a str,
        label: &'a str,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Self {
        Input(Field::new(name, label, FieldKind::Number { min, max }))
    }

    /// A text box with no visible label: `label` is its `aria-label`.
    pub(crate) fn text_box(name: &'a str, label: &'a str, value: &'a str) -> Self {
        Input(Field::new(name, label, FieldKind::Text))
            .hide_label()
            .value(value)
    }

    /// A search box with no visible label: `label` is its `aria-label`, `value` the query.
    pub(crate) fn search_box(name: &'a str, label: &'a str, value: &'a str) -> Self {
        Input(Field::new(name, label, FieldKind::Search))
            .hide_label()
            .value(value)
    }

    /// The same field speaking `ui`'s language and knowing its browser.
    pub(crate) fn of(self, ui: &Ui) -> Self {
        Input(self.0.of(ui))
    }

    /// `aria-describedby` for elements a component renders outside the field.
    pub(crate) fn described(mut self, ids: Option<&'a str>) -> Self {
        self.0.extra.described = ids;
        self
    }

    /// Steppers on a number field ([`crate::number`]).
    pub(crate) fn steppers(mut self) -> Self {
        self.0.extra.steppers = true;
        self
    }

    fn kind(mut self, kind: FieldKind<'a>) -> Self {
        self.0.kind = kind;
        self
    }

    /// `type="email"`: the browser checks the shape.
    pub fn email(self) -> Self {
        self.kind(FieldKind::Email)
    }

    /// `type="password"`: the value is never echoed back into the page.
    pub fn password(self) -> Self {
        self.kind(FieldKind::Password)
    }

    /// `type="tel"`: a phone number, the phone keypad on a touch screen. The browser checks
    /// nothing; give it a `.pattern(..)` of your own if you need one.
    pub fn tel(self) -> Self {
        self.kind(FieldKind::Tel)
    }

    /// `type="url"`: the browser checks it is an absolute URL (`https://..`).
    pub fn url(self) -> Self {
        self.kind(FieldKind::Url)
    }

    /// A number from `min` to `max`, inclusive: any number (`0`, `2.5`), or `None::<f64>` for
    /// no bound. Steps by 1 unless `.step(..)` says otherwise.
    pub fn number(self, min: impl Into<Bound>, max: impl Into<Bound>) -> Self {
        self.kind(FieldKind::Number {
            min: min.into().0,
            max: max.into().0,
        })
    }

    /// Text that must match `pattern`; `hint` explains the rule under the field and as the
    /// input's `title`.
    pub fn pattern(self, pattern: &'a str, hint: &'a str) -> Self {
        self.kind(FieldKind::Pattern { pattern, hint })
    }

    /// Multi-line text, `rows` high; it grows with its content where `field-sizing` works.
    pub fn textarea(self, rows: u8) -> Self {
        self.kind(FieldKind::Textarea { rows })
    }

    /// A file picker; `accept` lists MIME types or extensions, empty for any. The form
    /// around it needs `enctype="multipart/form-data"`.
    pub fn file(self, accept: &'a str) -> Self {
        self.kind(FieldKind::File { accept })
    }

    /// `type="date"`, bounds as `YYYY-MM-DD`; an empty bound is left out.
    pub fn date(self, min: &'a str, max: &'a str) -> Self {
        self.kind(FieldKind::Date { min, max })
    }

    /// `type="time"`, bounds as `HH:MM`; an empty bound is left out.
    pub fn time(self, min: &'a str, max: &'a str) -> Self {
        self.kind(FieldKind::Time { min, max })
    }

    /// `type="datetime-local"`, bounds as `YYYY-MM-DDTHH:MM`; an empty bound is left out.
    pub fn datetime(self, min: &'a str, max: &'a str) -> Self {
        self.kind(FieldKind::DateTime { min, max })
    }

    /// `type="month"`, bounds as `YYYY-MM`; an empty bound is left out. Firefox and Safari
    /// show a text field (FINDINGS), so give it a `.help(..)` with the format.
    pub fn month(self, min: &'a str, max: &'a str) -> Self {
        self.kind(FieldKind::Month { min, max })
    }

    /// `type="week"`, bounds as `YYYY-Www` (`2026-W40`); an empty bound is left out. Firefox
    /// and Safari show a text field (FINDINGS).
    pub fn week(self, min: &'a str, max: &'a str) -> Self {
        self.kind(FieldKind::Week { min, max })
    }

    /// The file picker takes several files, before or after `.file(..)`.
    pub fn multiple(mut self) -> Self {
        self.0.multiple = true;
        self
    }

    /// The `required` attribute, and a `*` after the label. The star is `aria-hidden`: the
    /// attribute already makes a screen reader say "required", in the browser's language.
    pub fn required(mut self) -> Self {
        self.0.required = true;
        self
    }

    /// "(optional)" after the label, for a form whose fields are mostly required; a required
    /// field shows its star instead.
    pub fn optional(mut self) -> Self {
        self.0.optional = true;
        self
    }

    /// `disabled` and `aria-disabled`: shown greyed and not posted.
    pub fn disabled(mut self) -> Self {
        self.0.disabled = true;
        self
    }

    /// Shown and posted, not editable: `readonly` on a text field; a checkbox, which HTML
    /// cannot make read-only, is disabled and posts its value from a hidden input.
    pub fn readonly(mut self) -> Self {
        self.0.readonly = true;
        self
    }

    /// Help text under the field.
    pub fn help(mut self, help: &'a str) -> Self {
        self.0.help = Some(help);
        self
    }

    /// `maxlength`, counted in an `<output>` under the field.
    pub fn maxlength(mut self, max: usize) -> Self {
        self.0.maxlength = Some(max);
        self
    }

    /// The current value (ignored for files and passwords).
    pub fn value(mut self, value: &'a str) -> Self {
        self.0.value = value;
        self
    }

    /// Tick the checkbox or switch.
    pub fn checked(mut self, checked: bool) -> Self {
        self.0.value = if checked { "true" } else { "" };
        self
    }

    /// A server message under the field; the control gets `aria-invalid`. An empty message
    /// is no error, so a route can pass `if bad { ".." } else { "" }`.
    pub fn error(mut self, message: &'a str) -> Self {
        self.0.error = (!message.is_empty()).then_some(message);
        self
    }

    /// Placeholder text.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.0.placeholder = Some(placeholder);
        self
    }

    /// `type="search"`: a filter box, with the browser's clear button.
    pub fn search(self) -> Self {
        self.kind(FieldKind::Search)
    }

    /// Only the control, the label kept as its `aria-label`: a filter box in a toolbar, a
    /// number beside buttons. Help, counter and error still follow the control, so its
    /// `aria-describedby` points at them.
    pub fn hide_label(mut self) -> Self {
        self.0.extra.hide_label = true;
        self
    }

    /// `list`: the id of a `<datalist>` of suggestions.
    pub fn list(mut self, id: &'a str) -> Self {
        self.0.extra.list = Some(id);
        self
    }

    /// `autocomplete` (`"off"`, `"email"`, `"new-password"`).
    pub fn autocomplete(mut self, value: &'a str) -> Self {
        self.0.extra.autocomplete = Some(value);
        self
    }

    /// `autofocus`: focused when the page or the popover around it opens.
    pub fn autofocus(mut self) -> Self {
        self.0.extra.autofocus = true;
        self
    }

    /// `inputmode` (`"numeric"`), the on-screen keyboard to show.
    pub fn inputmode(mut self, mode: &'a str) -> Self {
        self.0.extra.inputmode = Some(mode);
        self
    }

    /// `step` of a number field.
    pub fn step(mut self, step: impl Into<f64>) -> Self {
        self.0.extra.step = Some(step.into());
        self
    }

    /// `aria-controls`: the id of the region this field updates (a results list).
    pub fn aria_controls(mut self, id: &'a str) -> Self {
        self.0.extra.aria_controls = Some(id);
        self
    }

    /// `form`: the id of the form this control posts with, when it cannot sit inside it (a
    /// table row being edited).
    pub fn form(mut self, id: &'a str) -> Self {
        self.0.extra.form = Some(id);
        self
    }

    /// A class on the control, for a component's part name.
    pub fn class(mut self, class: &'a str) -> Self {
        self.0.extra.class = Some(class);
        self
    }

    /// The control's id, `f-<name>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.0.id = Some(id);
        self
    }

    /// The border drawn with `--lui-gradient-ring`, on a text field or textarea: the one
    /// field a page leads with. A plain border without `in oklch` gradients.
    pub fn gradient_border(mut self) -> Self {
        self.0.extra.gradient_border = true;
        self
    }

    /// A text field's height and text, Radix's sizes 1 to 3 (2 is the default); every size is
    /// 2.75rem tall on a touch screen.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.0.extra.size = size.into();
        self
    }

    /// A text field's look: surface (the default, a border), soft (a brand tint and no
    /// border) or ghost (neither); solid and outline draw as surface.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.0.extra.variant = Some(variant);
        self
    }

    /// An icon or text inside the field's border, before the text: an [`Icon`] (a search
    /// glass, decorative) or a `&str` (`"https://"`, read out after the label). Single-line
    /// fields only.
    pub fn leading(mut self, slot: impl Into<Glyph<'a>>) -> Self {
        self.0.extra.leading = Some(slot.into());
        self
    }

    /// An icon or text inside the field's border, after the text: a unit (`"kg"`) or an
    /// [`Icon`]. Single-line fields only.
    pub fn trailing(mut self, slot: impl Into<Glyph<'a>>) -> Self {
        self.0.extra.trailing = Some(slot.into());
        self
    }

    /// An addon joined to the field's start, outside its border on a gray fill (`"https://"`,
    /// `"$"`): text is read out with the field, an icon is decoration. Single-line fields only.
    pub fn prefix(mut self, addon: impl Into<Glyph<'a>>) -> Self {
        self.0.extra.prefix = Some(addon.into());
        self
    }

    /// An addon joined to the field's end (`"kg"`, `".com"`), like [`Self::prefix`].
    pub fn suffix(mut self, addon: impl Into<Glyph<'a>>) -> Self {
        self.0.extra.suffix = Some(addon.into());
        self
    }

    /// A button joined to the field's end (Copy, Send, Apply): it submits the form unless
    /// given a command.
    pub fn addon_button(mut self, button: Button<'a>) -> Self {
        self.0.extra.addon = Some(button.render());
        self
    }

    /// A show/hide button on a password field. An input's type cannot change without script,
    /// so the button carries `hidden` and only the enhancement script shows it and switches
    /// the field between `password` and `text`; without the script the field is the same,
    /// with no button.
    pub fn reveal(mut self) -> Self {
        self.0.extra.reveal = true;
        self
    }

    /// A narrower control: 1 (8rem), 2 (16rem) or 3 (24rem), never wider than its container;
    /// the container's width by default. For a postcode, a year, an amount.
    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.0.extra.width = Some(width.into());
        self
    }
}

/// A server message under a field, after a warning icon; `role="alert"` so it is read out.
pub(crate) fn error_line(id: &str, message: &str) -> Markup {
    html! { p id=(id) class="lui-error" role="alert" { (Icon::TriangleAlert) (message) } }
}

/// A slot's or addon's text is read out with the field (`aria-describedby`); an icon is
/// decoration.
fn slot_id(id: &str, side: &str, glyph: Option<Glyph>) -> Option<String> {
    matches!(glyph, Some(Glyph::Text(_))).then(|| format!("{id}-{side}"))
}

/// The ids of what describes a control (its help, counter and error), joined for
/// `aria-describedby`, `None` when there are none.
pub(crate) fn described_by(ids: &[Option<String>]) -> Option<String> {
    let found: Vec<&str> = ids.iter().flatten().map(String::as_str).collect();
    (!found.is_empty()).then(|| found.join(" "))
}

/// Help, counter and error, in that order: what follows a control, labelled or not.
fn after(
    id: &str,
    help: Option<&str>,
    count: Option<(usize, usize)>,
    error: Option<&str>,
) -> Markup {
    html! {
        @if let Some(h) = help { small id={ (id) "-help" } class="lui-field-help" { (h) } }
        @if let Some((n, max)) = count {
            output id={ (id) "-count" } for=(id) class="lui-field-count" { (n) " / " (max) }
        }
        @if let Some(e) = error { (error_line(&format!("{id}-error"), e)) }
    }
}

impl Render for Input<'_> {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

impl Field<'_> {
    /// A select field, drawn by [`crate::select`].
    fn select(&self, options: &[SelectOption<'_>]) -> Markup {
        let mut s = crate::select::Select::bare(self.caps, self.strings, self.name, self.label)
            .options(options.iter().cloned())
            .id_from(self.id);
        s.selected = if self.values.is_empty() {
            vec![self.value]
        } else {
            self.values.clone()
        };
        s.error = self.error;
        s.help = self.help;
        s.required = self.required;
        s.optional = self.optional;
        s.disabled = self.disabled;
        s.readonly = self.readonly;
        s.multiple = self.multiple;
        s.placeholder = self.placeholder;
        s.hide_label = self.extra.hide_label;
        s.autocomplete = self.extra.autocomplete;
        s.autofocus = self.extra.autofocus;
        s.form = self.extra.form;
        s.class = self.extra.class;
        s.size = self.extra.size;
        s.variant = self.extra.variant;
        s.render()
    }

    /// The fields that delegate to another component, drawn by it; `None` for the rest.
    fn delegated(&self) -> Option<Markup> {
        let picked = || -> Vec<&str> {
            if self.values.is_empty() {
                vec![self.value]
            } else {
                self.values.clone()
            }
        };
        Some(match &self.kind {
            FieldKind::Markup(m) => m.clone(),
            FieldKind::Hidden => keep(self.name, self.value),
            FieldKind::Select(options) => self.select(options),
            FieldKind::Radio(picks) => RadioGroup {
                options: picks.clone(),
                ..RadioGroup::of(self)
            }
            .render(),
            FieldKind::Checkboxes(picks) => {
                let mut g = crate::checkbox_group::CheckboxGroup::of(self);
                g.options = picks.clone();
                g.value = picked();
                g.render()
            }
            FieldKind::Range { min, max } => {
                let mut r = crate::range::Range::bare(self.strings, self.name, self.label)
                    .id_from(self.id)
                    .min(*min)
                    .max(*max);
                if let Some(step) = self.extra.step {
                    r = r.step(step);
                }
                if let Ok(v) = self.value.trim().parse::<f64>() {
                    r = r.value(v);
                }
                r.disabled = self.disabled;
                r.readonly = self.readonly;
                r.render()
            }
            FieldKind::Color => {
                let mut c = crate::color::Color::bare(self.strings, self.name, self.label);
                if !self.value.is_empty() {
                    c = c.value(self.value);
                }
                c.disabled = self.disabled;
                c.readonly = self.readonly;
                c.render()
            }
            _ => return None,
        })
    }

    /// A checkbox or switch, its label beside it.
    fn check(&self, id: &str, help: Option<&str>, described: Option<&str>) -> Markup {
        let checked = matches!(self.value, "true" | "on" | "1");
        let switch = matches!(self.kind, FieldKind::Switch);
        let invalid = self.error.map(|_| "true");
        let off = self.disabled || self.readonly;
        html! {
            div class={ "lui-field lui-field-check" @if switch { " lui-field-switch" } } {
                label for=(id) {
                    input id=(id) name=[(!self.readonly).then_some(self.name)] type="checkbox" value="true" checked[checked] required[self.required]
                        disabled[off] aria-disabled=[self.disabled.then_some("true")]
                        role=[switch.then_some("switch")] class=[switch.then_some("lui-switch")]
                        aria-invalid=[invalid] aria-describedby=[described];
                    " " (self.label) (self.marks())
                }
                @if self.readonly && checked { (keep(self.name, "true")) }
                (after(id, help, None, self.error))
            }
        }
    }
}

impl Render for Field<'_> {
    fn render(&self) -> Markup {
        if let Some(m) = self.delegated() {
            return m;
        }
        let id = self.control_id();
        let extra = &self.extra;
        let help = self.help.or(match self.kind {
            FieldKind::Pattern { hint, .. } => Some(hint),
            _ => None,
        });
        let textarea = matches!(self.kind, FieldKind::Textarea { .. });
        let (lead, trail, prefix, suffix) = if textarea {
            (None, None, None, None)
        } else {
            (extra.leading, extra.trailing, extra.prefix, extra.suffix)
        };
        let count = self.maxlength.map(|max| (self.value.chars().count(), max));
        let described = described_by(&[
            slot_id(&id, "prefix", prefix),
            slot_id(&id, "leading", lead),
            slot_id(&id, "trailing", trail),
            slot_id(&id, "suffix", suffix),
            help.map(|_| format!("{id}-help")),
            count.map(|_| format!("{id}-count")),
            self.error.map(|_| format!("{id}-error")),
            extra.described.map(str::to_string),
        ]);
        if let FieldKind::Checkbox | FieldKind::Switch = self.kind {
            return self.check(&id, help, described.as_deref());
        }
        let bound = |s: &str| (!s.is_empty()).then(|| s.to_string());
        let (kind, min, max, pattern, accept) = match self.kind {
            FieldKind::Search => ("search", None, None, None, None),
            FieldKind::Email => ("email", None, None, None, None),
            FieldKind::Password => ("password", None, None, None, None),
            FieldKind::Tel => ("tel", None, None, None, None),
            FieldKind::Url => ("url", None, None, None, None),
            FieldKind::Number { min, max } => (
                "number",
                min.map(|m| m.to_string()),
                max.map(|m| m.to_string()),
                None,
                None,
            ),
            FieldKind::Pattern { pattern, .. } => ("text", None, None, Some(pattern), None),
            FieldKind::File { accept } => (
                "file",
                None,
                None,
                None,
                (!accept.is_empty()).then_some(accept),
            ),
            FieldKind::Date { min, max } => ("date", bound(min), bound(max), None, None),
            FieldKind::Time { min, max } => ("time", bound(min), bound(max), None, None),
            FieldKind::DateTime { min, max } => {
                ("datetime-local", bound(min), bound(max), None, None)
            }
            FieldKind::Month { min, max } => ("month", bound(min), bound(max), None, None),
            FieldKind::Week { min, max } => ("week", bound(min), bound(max), None, None),
            _ => ("text", None, None, None, None),
        };
        let invalid = self.error.map(|_| "true");
        let echo = !matches!(self.kind, FieldKind::File { .. } | FieldKind::Password);
        let multiple = self.multiple && matches!(self.kind, FieldKind::File { .. });
        // Size and variant go on the root: the field, or the outermost part of the control
        // when its label is hidden.
        let size = (extra.size != Size::S2).then(|| extra.size.as_str());
        let variant = extra.variant.map(Variant::as_str);
        let width = extra.width.map(Size::as_str);
        let slots = lead.is_some() || trail.is_some();
        let steppers = extra.steppers && matches!(self.kind, FieldKind::Number { .. });
        let reveal = extra.reveal && matches!(self.kind, FieldKind::Password);
        let group =
            prefix.is_some() || suffix.is_some() || extra.addon.is_some() || steppers || reveal;
        let bare = extra.hide_label;
        let gradient = extra.gradient_border.then_some("lui-input-gradient-border");
        let class = match (extra.class, gradient) {
            (Some(c), Some(g)) => Some(format!("{c} {g}")),
            (c, g) => c.or(g).map(str::to_string),
        };
        let on = |layer: u8| {
            let outer = if group {
                2
            } else if slots {
                1
            } else {
                0
            };
            bare && layer == outer
        };
        let step = extra.step;
        let disabled = self.disabled.then_some("true");
        let control = if let FieldKind::Textarea { rows } = self.kind {
            html! {
                textarea id=(id) class=[class.as_deref()] data-size=[size.filter(|_| bare)] data-variant=[variant.filter(|_| bare)]
                    name=(self.name) rows=(rows) required[self.required] disabled[self.disabled] aria-disabled=[disabled] readonly[self.readonly]
                    maxlength=[self.maxlength] placeholder=[self.placeholder]
                    form=[extra.form] autocomplete=[extra.autocomplete] autofocus[extra.autofocus] inputmode=[extra.inputmode]
                    aria-label=[bare.then_some(self.label)] aria-controls=[extra.aria_controls]
                    aria-invalid=[invalid] aria-describedby=[described.as_deref()] { (self.value) }
            }
        } else {
            html! {
                input id=(id) class=[class.as_deref()] data-size=[size.filter(|_| on(0))] data-variant=[variant.filter(|_| on(0))]
                    name=(self.name) type=(kind) value=[echo.then_some(self.value)]
                    required[self.required] disabled[self.disabled] aria-disabled=[disabled] readonly[self.readonly && kind != "file"]
                    min=[min] max=[max] step=[step] pattern=[pattern] title=[pattern.and(help)]
                    accept=[accept] multiple[multiple] maxlength=[self.maxlength] placeholder=[self.placeholder]
                    form=[extra.form] list=[extra.list] autocomplete=[extra.autocomplete] autofocus[extra.autofocus] inputmode=[extra.inputmode]
                    aria-label=[bare.then_some(self.label)] aria-controls=[extra.aria_controls]
                    aria-invalid=[invalid] aria-describedby=[described.as_deref()];
            }
        };
        let slot = |side: &str, glyph: Option<Glyph>| {
            html! { @if let Some(g) = glyph {
                span id=[slot_id(&id, side, glyph)] class={ "lui-input-" (side) } { (g) }
            } }
        };
        let control = if slots {
            html! { div class="lui-input-slots" data-size=[size.filter(|_| on(1))] data-variant=[variant.filter(|_| on(1))] {
                (slot("leading", lead)) (control) (slot("trailing", trail))
            } }
        } else {
            control
        };
        let control = if group {
            let addon = |side: &str, glyph: Option<Glyph>| {
                html! { @if let Some(g) = glyph {
                    span id=[slot_id(&id, side, glyph)] class={ "lui-input-group-" (side) } { (g) }
                } }
            };
            html! {
                div class="lui-input-group" data-size=[size.filter(|_| on(2))] data-variant=[variant.filter(|_| on(2))] {
                    @if steppers { (self.steppers()) }
                    (addon("prefix", prefix)) (control) (addon("suffix", suffix))
                    @if let Some(b) = &extra.addon { span class="lui-input-group-button" { (b) } }
                    @if reveal {
                        span class="lui-input-reveal" data-lui="reveal" hidden {
                            (Button::new(self.caps, "").ghost().icon_only().aria_label(self.strings.get(Text::ShowPassword)).pressed(false).body(html! { (Icon::Eye) }))
                        }
                    }
                }
            }
        } else {
            control
        };
        let tail = after(&id, help, count, self.error);
        if bare {
            return html! { (control) (tail) };
        }
        html! {
            div class="lui-field" data-size=[size] data-variant=[variant] data-width=[width] {
                label for=(id) { (self.label) (self.marks()) }
                (control)
                (tail)
            }
        }
    }
}

impl Field<'_> {
    /// A number field's bounds.
    fn number_bounds(&self) -> (Option<f64>, Option<f64>) {
        match self.kind {
            FieldKind::Number { min, max } => (min, max),
            _ => (None, None),
        }
    }

    /// Two buttons posting the field's value one step down and up, before the control so
    /// the pressed one's value is the first posted under the field's name (what
    /// [`crate::Posted::get`] and [`Ui::param`] read); CSS shows them after it. The
    /// enhancement script steps the field in place instead (`stepUp`/`stepDown`).
    fn steppers(&self) -> Markup {
        let (min, max) = self.number_bounds();
        let step = self.extra.step.unwrap_or(1.0);
        let base = min.unwrap_or(0.0);
        let now: f64 = self.value.trim().parse().unwrap_or(base.max(0.0));
        let at = |n: f64| {
            let n = tidy(n, step, base);
            let n = max.map_or(n, |m| n.min(m));
            min.map_or(n, |m| n.max(m))
        };
        let (down, up) = (at(now - step).to_string(), at(now + step).to_string());
        let off = self.disabled || self.readonly;
        let button = |value: &str, label: Text, icon: Icon, stop: bool| -> Markup {
            let b = Button::new(self.caps, "")
                .ghost()
                .icon_only()
                .name(self.name)
                .value(value)
                .formnovalidate()
                .aria_label(self.strings.get(label))
                .body(html! { (icon) });
            let b = match self.extra.form {
                Some(form) => b.form(form),
                None => b,
            };
            if stop || off { b.disabled() } else { b }.render()
        };

        html! {
            span class="lui-input-steppers" data-lui="steppers" {
                (button(&up, Text::Increment, Icon::ChevronUp, max.is_some_and(|m| now >= m)))
                (button(&down, Text::Decrement, Icon::ChevronDown, min.is_some_and(|m| now <= m)))
            }
        }
    }
}

/// A set of radio buttons under a legend, made by [`Ui::radio_group`]. Add choices with
/// [`RadioGroup::option`].
///
/// **Setters.** Values and items: `.option(..)`, `.description(..)`, `.value(..)`, `.help(..)`,
/// `.error(..)`; switches: `.required()`, `.cards()`, `.disabled()`.
#[derive(Clone, Debug)]
pub struct RadioGroup<'a> {
    name: &'a str,
    legend: &'a str,
    options: Vec<Pick<'a>>,
    cards: bool,
    value: &'a str,
    required: bool,
    optional: bool,
    help: Option<&'a str>,
    error: Option<&'a str>,
    strings: &'static Strings,
}

impl RadioGroup<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("option", PropKind::Item, "value: &'a str, label: &'a str")
            .doc("One choice posting `value`, labelled `label`."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .applies_to(&["option"])
            .doc("A line under the choice added last, read out with it."),
        Prop::new("disabled", PropKind::Modifier, "")
            .applies_to(&["option"])
            .attr("disabled")
            .doc("The choice added last cannot be picked."),
        Prop::new("cards", PropKind::Switch, "")
            .doc("Each choice a selectable card, 1, 2 or 3 across by the container's width."),
        Prop::new("value", PropKind::Value, "value: &'a str")
            .attr("value")
            .doc("The value of the choice that is selected."),
        Prop::new("required", PropKind::Switch, "")
            .attr("required")
            .doc("One choice must be picked before the form submits."),
        Prop::new("help", PropKind::Value, "help: &'a str").doc("Help text under the choices."),
        Prop::new("error", PropKind::Value, "message: &'a str")
            .doc("A server message under the choices."),
    ];
}

impl Ui {
    /// Radio buttons named `name` in a `<fieldset>` whose `<legend>` is `legend`.
    pub fn radio_group<'a>(&self, name: &'a str, legend: &'a str) -> RadioGroup<'a> {
        RadioGroup {
            name,
            legend,
            options: Vec::new(),
            cards: false,
            value: "",
            required: false,
            optional: false,
            help: None,
            error: None,
            strings: self.strings,
        }
    }
}

impl<'a> RadioGroup<'a> {
    /// A form's radio field drawn as a group.
    pub(crate) fn of(f: &Field<'a>) -> Self {
        RadioGroup {
            name: f.name,
            legend: f.label,
            options: Vec::new(),
            cards: f.extra.cards,
            value: f.value,
            required: f.required,
            optional: f.optional,
            help: f.help,
            error: f.error,
            strings: f.strings,
        }
    }

    /// One choice posting `value`, labelled `label`.
    pub fn option(mut self, value: &'a str, label: &'a str) -> Self {
        self.options.push(Choice { value, label }.into());
        self
    }

    /// A line under the choice added last, read out with it (`aria-describedby`); shown in
    /// both layouts, and what a card holds under its title.
    pub fn description(mut self, text: &'a str) -> Self {
        debug_assert!(
            !self.options.is_empty(),
            "RadioGroup::description() applies to an option, but there is none yet"
        );
        if let Some(o) = self.options.last_mut() {
            o.description = Some(text);
        }
        self
    }

    /// The choice added last cannot be picked: `disabled`, greyed.
    pub fn disabled(mut self) -> Self {
        debug_assert!(
            !self.options.is_empty(),
            "RadioGroup::disabled() applies to an option, but there is none yet"
        );
        if let Some(o) = self.options.last_mut() {
            o.disabled = true;
        }
        self
    }

    /// Each choice a selectable card (title and description, the radio hidden, the card
    /// ringed when picked), 1, 2 or 3 across by the container's width.
    pub fn cards(mut self) -> Self {
        self.cards = true;
        self
    }

    /// The value of the choice that is selected.
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    /// One choice must be picked before the form submits.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Help text under the choices.
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// A server message under the choices; the group gets `aria-invalid`. An empty message
    /// is no error.
    pub fn error(mut self, message: &'a str) -> Self {
        self.error = (!message.is_empty()).then_some(message);
        self
    }
}

impl Render for RadioGroup<'_> {
    fn render(&self) -> Markup {
        let id = format!("f-{}", self.name);
        let described = described_by(&[
            self.help.map(|_| format!("{id}-help")),
            self.error.map(|_| format!("{id}-error")),
        ]);
        // The first choice that can be picked carries `required`, which covers the group.
        let first = self.options.iter().position(|o| !o.disabled);
        html! {
            fieldset class={ "lui-field lui-radio-group" @if self.cards { " lui-radio-cards" } }
                aria-describedby=[described.as_deref()] aria-invalid=[self.error.map(|_| "true")] {
                legend {
                    (self.legend) @if self.required { " " (REQUIRED) }
                    @if self.optional && !self.required { " " span class="lui-field-optional" { (self.strings.get(Text::Optional)) } }
                }
                div class="lui-radio-group-options" {
                    @for (i, o) in self.options.iter().enumerate() {
                        @let oid = format!("{id}-{i}");
                        @let did = o.description.map(|_| format!("{oid}-description"));
                        label for=(oid) class="lui-radio-group-option" {
                            input id=(oid) type="radio" name=(self.name) value=(o.value)
                                checked[o.value == self.value] required[self.required && Some(i) == first]
                                disabled[o.disabled] aria-disabled=[o.disabled.then_some("true")]
                                aria-describedby=[did.as_deref()];
                            span class="lui-radio-group-text" {
                                span class="lui-radio-group-title" { (o.label) }
                                @if let (Some(d), Some(did)) = (o.description, &did) {
                                    span id=(did) class="lui-radio-group-description" { (d) }
                                }
                            }
                        }
                    }
                }
                (after(&id, self.help, None, self.error))
            }
        }
    }
}

/// The visual mark of a required field. Hidden from screen readers, since the `required`
/// attribute says it already; [`Text::RequiredHint`](crate::i18n::Text::RequiredHint)
/// explains the star under a form.
pub(crate) const REQUIRED: maud::PreEscaped<&str> =
    maud::PreEscaped(r#"<span class="lui-field-required" aria-hidden="true">*</span>"#);

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* Native controls after Radix Themes TextField (surface variant): a 1px --lui-gray-7 border, no
   shadow, --lui-control-h tall (Radix's 32px at our taller control height), px-3. These bare
   element rules are the one place inputs are styled: a hand-written field and a component's
   control look the same. */
input, select, textarea { font: inherit; font-size: 0.875rem; line-height: 1.25rem; color: inherit; }
label { font-weight: 500; }
input, select, textarea {
  min-height: var(--lui-control-h); padding: 0.375rem 0.75rem; min-width: 0;
  background: transparent; border: 1px solid var(--lui-input); border-radius: var(--lui-radius-sm);
  transition: border-color var(--lui-duration-fast), box-shadow var(--lui-duration-fast);
}
textarea { min-height: 4rem; }
input::placeholder, textarea::placeholder { color: var(--lui-gray-10); }
input[type=number] { font-variant-numeric: tabular-nums; }
/* Native select: no OS chrome, a chevron drawn from two gradients in the muted colour. */
select {
  appearance: none; padding-right: 2rem;
  background-image: linear-gradient(45deg, transparent 50%, var(--lui-muted) 50%), linear-gradient(135deg, var(--lui-muted) 50%, transparent 50%);
  background-position: right 1rem center, right 0.75rem center; background-size: 0.25rem 0.25rem; background-repeat: no-repeat;
}
select[multiple], select[size] { padding-right: 0.75rem; background-image: none; }
/* Radix Checkbox and Radio, size 2: a 1rem box (radius 4px) or circle in a --lui-gray-7 border,
   filled with the primary colour when checked; the tick (two borders turned 45°) and the
   dot scale in. A component that hides its input (calendar, toggle group) sets opacity 0. */
input:is([type=checkbox], [type=radio]) {
  appearance: none; display: inline-grid; place-content: center; flex: none;
  width: 1rem; height: 1rem; min-height: 0; padding: 0; margin: 0; vertical-align: -0.15em;
  border: 1px solid var(--lui-input); border-radius: 0.25rem; background: var(--lui-bg); cursor: pointer;
}
input[type=radio] { border-radius: 50%; }
input:is([type=checkbox], [type=radio]):checked { background: var(--lui-primary); border-color: var(--lui-primary); }
input[type=checkbox]:not(.lui-switch)::before {
  content: ""; width: 0.3rem; height: 0.55rem; translate: 0 -0.0625rem; rotate: 45deg; scale: 0;
  border: solid var(--lui-on-primary); border-width: 0 2px 2px 0;
  transition: scale var(--lui-duration-fast) var(--lui-ease-spring);
}
input[type=radio]::before {
  content: ""; width: 0.375rem; height: 0.375rem; border-radius: 50%; background: var(--lui-on-primary); scale: 0;
  transition: scale var(--lui-duration-fast) var(--lui-ease-spring);
}
input:is([type=checkbox], [type=radio]):checked::before { scale: 1; }
input[type=range] { min-height: 0; padding: 0; border: 0; box-shadow: none; accent-color: var(--lui-primary); }
input[type=color] { padding: 0.25rem; }
input[type=file] { padding-block: 0.25rem; }
input::file-selector-button { font: inherit; font-weight: 500; color: var(--lui-fg); background: transparent; border: 0; padding: 0 0.5rem 0 0; }
:is(input, select, textarea):focus-visible { border-color: var(--lui-ring); }
/* Radix disabled: a --lui-gray-2 fill, --lui-gray-6 border and --lui-gray-11 text, not a faded control. */
:is(input, select, textarea):disabled { background-color: var(--lui-gray-2); border-color: var(--lui-gray-6); color: var(--lui-gray-11); cursor: not-allowed; }
input:is([type=checkbox], [type=radio]):disabled { background: var(--lui-gray-3); }
input:is([type=checkbox], [type=radio]):disabled:checked { background: var(--lui-gray-8); border-color: var(--lui-gray-8); }
.lui-field { display: grid; gap: 0.5rem; }
.lui-field label { font-size: 0.875rem; line-height: 1.25rem; font-weight: 500; }
.lui-field-check label, .lui-radio-group-option { display: flex; align-items: center; gap: 0.5rem; }
/* :where keeps this at one class, so a component inside a field (colour, range) sizes itself. */
.lui-field :where(input:not([type=file], [type=color], [type=range], [type=checkbox], [type=radio]), textarea) { width: 100%; box-sizing: border-box; }
.lui-field textarea { resize: vertical; field-sizing: content; min-height: 3lh; max-height: 20lh; font: inherit; }
.lui-field-help { color: var(--lui-muted); font-size: 0.8125rem; line-height: 1.25rem; }
.lui-field-count { justify-self: end; color: var(--lui-muted); font-size: 0.75rem; font-variant-numeric: tabular-nums; }
/* One rule per newer selector (:user-invalid is Chrome 119, :has() Chrome 105 and not in
   Blitz): in a list, a browser that lacks one drops the whole rule (FINDINGS, M23). */
.lui-field [aria-invalid=true] { border-color: var(--lui-danger); }
.lui-field :is(input, textarea):user-invalid { border-color: var(--lui-danger); }
.lui-field [aria-invalid=true] ~ label { color: var(--lui-danger); }
.lui-field:has([aria-invalid=true]) > label { color: var(--lui-danger); }
.lui-error { display: flex; align-items: flex-start; gap: 0.375rem; color: var(--lui-danger); margin: 0; font-size: 0.8125rem; line-height: 1.25rem; }
.lui-error > .lui-icon { margin-top: 0.125rem; width: 0.875rem; height: 0.875rem; }
/* .leading() / .trailing(): Radix TextField slots. The wrapper draws the border; the input
   inside is borderless and the wrapper takes its focus, invalid and disabled looks. */
.lui-input-slots {
  display: flex; align-items: center; gap: var(--lui-space-2); box-sizing: border-box; width: 100%;
  min-height: var(--lui-control-h); padding-inline: 0.75rem; color: var(--lui-muted); font-size: 0.875rem;
  border: 1px solid var(--lui-input); border-radius: var(--lui-radius-sm); cursor: text;
  transition: border-color var(--lui-duration-fast);
}
.lui-input-slots > input { flex: 1; min-height: calc(var(--lui-control-h) - 2px); padding-inline: 0; border: 0; border-radius: 0; color: var(--lui-fg); }
.lui-input-slots > input:focus-visible { outline: none; }
.lui-input-slots:has(> input:focus-visible) { border-color: var(--lui-ring); outline: 3px solid color-mix(in srgb, var(--lui-ring) 50%, transparent); }
.lui-input-slots:has(> input[aria-invalid=true]) { border-color: var(--lui-danger); }
.lui-input-slots:has(> input:user-invalid) { border-color: var(--lui-danger); }
.lui-input-slots:has(> input:disabled) { background: var(--lui-gray-2); border-color: var(--lui-gray-6); cursor: not-allowed; }
.lui-input-leading, .lui-input-trailing { display: inline-flex; flex: none; white-space: nowrap; }
/* .size(..) and .variant(..), on the field's root (the control itself when its label is
   hidden): Radix TextField sizes 1 and 3 on the shared control heights, and its soft look (a
   brand-3 tint, no border) and a ghost one (no border or fill). */
:is(.lui-field[data-size="1"] :is(input, textarea, .lui-input-slots), :is(input, .lui-input-slots)[data-size="1"]) { min-height: var(--lui-control-h-sm); font-size: 0.8125rem; }
:is(.lui-field[data-size="1"] input, input[data-size="1"]) { padding-block: 0.25rem; }
:is(.lui-field[data-size="3"] :is(input, textarea, .lui-input-slots), :is(input, .lui-input-slots)[data-size="3"]) { min-height: var(--lui-control-h-lg); font-size: 1rem; line-height: 1.5rem; }
:is(.lui-field[data-size="3"] input, input[data-size="3"]) { padding-block: 0.5rem; }
:is(.lui-field[data-variant=soft] :is(input, textarea, .lui-input-slots), :is(input, .lui-input-slots)[data-variant=soft]) { background-color: var(--lui-brand-3); border-color: transparent; }
:is(.lui-field[data-variant=ghost] :is(input, textarea, .lui-input-slots), :is(input, .lui-input-slots)[data-variant=ghost]) { background-color: transparent; border-color: transparent; }
:is(.lui-field[data-variant] .lui-input-slots, .lui-input-slots[data-variant]) > input { background-color: transparent; }
/* .gradient_border(): the page's fill on the padding box over the ring gradient on the border box. */
@supports (background: linear-gradient(in oklch, currentColor, transparent)) {
  :is(input, textarea).lui-input-gradient-border {
    border-color: transparent;
    background: linear-gradient(var(--lui-bg), var(--lui-bg)) padding-box, var(--lui-gradient-ring) border-box;
  }
}
.lui-radio-group { container: lui-radio-group / inline-size; margin: 0; padding: 0; border: 0; gap: 0.75rem; }
.lui-radio-group legend { padding: 0; margin-bottom: 0.75rem; font-size: 0.875rem; font-weight: 500; }
.lui-radio-group[aria-invalid=true] legend { color: var(--lui-danger); }
.lui-radio-group-options { display: grid; gap: 0.75rem; }
.lui-radio-group-option { cursor: pointer; }
.lui-radio-group-option:has(.lui-radio-group-description) { align-items: flex-start; }
.lui-radio-group-option:has(.lui-radio-group-description) > input { margin-top: 0.125rem; }
.lui-radio-group-text { display: grid; gap: 0.125rem; min-width: 0; }
.lui-radio-group-description { color: var(--lui-muted); font-weight: 400; font-size: 0.8125rem; line-height: 1.25rem; }
/* .cards(): Radix RadioCards. Each choice a --lui-gray-7 bordered card, ringed in the primary
   colour when picked, 1, 2 or 3 across as the group's own width allows. The radio covers
   the card, invisible, so the whole card is its hit area. */
.lui-radio-cards .lui-radio-group-options { grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-3); }
@container lui-radio-group (width >= 30rem) { .lui-radio-cards .lui-radio-group-options { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
@container lui-radio-group (width >= 48rem) { .lui-radio-cards .lui-radio-group-options { grid-template-columns: repeat(3, minmax(0, 1fr)); } }
.lui-radio-cards .lui-radio-group-option {
  position: relative; padding: var(--lui-space-3) var(--lui-space-4); border: 1px solid var(--lui-input);
  border-radius: var(--lui-radius); background: var(--lui-bg);
  transition: border-color var(--lui-duration-fast), box-shadow var(--lui-duration-fast);
}
.lui-radio-cards .lui-radio-group-option:hover { border-color: var(--lui-gray-8); }
.lui-radio-cards .lui-radio-group-option > input { position: absolute; inset: 0; width: 100%; height: 100%; margin: 0; opacity: 0; }
.lui-radio-cards .lui-radio-group-option:has(> input:checked) { border-color: var(--lui-primary); box-shadow: inset 0 0 0 1px var(--lui-primary); }
.lui-radio-cards .lui-radio-group-option:has(> input:focus-visible) { outline: 3px solid color-mix(in srgb, var(--lui-ring) 50%, transparent); }
.lui-radio-cards .lui-radio-group-option:has(> input:disabled) { background: var(--lui-gray-2); color: var(--lui-gray-11); cursor: not-allowed; }
/* Radix Switch, size 2: a 2.1875 by 1.25rem pill (35 by 20px) in --lui-gray-5, the primary colour
   when on, an --lui-on-primary thumb with a small shadow that slides on the spring curve. */
input.lui-switch {
  appearance: none; position: relative; flex: none; width: 2.1875rem; height: 1.25rem; margin: 0;
  border: 0; border-radius: 9999px; background: var(--lui-gray-5);
  box-shadow: inset 0 0 0 1px var(--lui-gray-6); cursor: pointer; transition: background-color var(--lui-duration-fast);
}
input.lui-switch::before {
  content: ""; position: absolute; top: 1px; left: 1px; width: 1.125rem; height: 1.125rem;
  border-radius: 50%; background: var(--lui-on-primary); box-shadow: var(--lui-shadow-xs);
  transition: translate var(--lui-duration) var(--lui-ease-spring);
}
input.lui-switch:checked { background: var(--lui-primary); box-shadow: none; }
input.lui-switch:checked::before { translate: calc(2.1875rem - 1.125rem - 2px) 0; }
input.lui-switch:disabled { background: var(--lui-gray-4); }
/* .optional(): a quiet note after the label. */
.lui-field-optional { color: var(--lui-muted); font-weight: 400; }
/* .width(..): a narrower control for a short value, never wider than its container. */
.lui-field[data-width="1"] > :is(input, textarea, .lui-input-slots, .lui-input-group, .lui-select) { max-width: 8rem; }
.lui-field[data-width="2"] > :is(input, textarea, .lui-input-slots, .lui-input-group, .lui-select) { max-width: 16rem; }
.lui-field[data-width="3"] > :is(input, textarea, .lui-input-slots, .lui-input-group, .lui-select) { max-width: 24rem; }
/* .prefix(..) / .suffix(..) / .addon_button(..): the Origin UI inputs with add-ons. One border
   around the row; an addon is a --lui-gray-2 cell split from the field by a hairline, and a
   button sits flush at the end. The steppers (a number field's) and the reveal button live
   here too. The control inside drops its own border and takes the row's focus ring. */
.lui-input-group {
  display: flex; align-items: stretch; box-sizing: border-box; width: 100%; min-width: 0;
  min-height: var(--lui-control-h); border: 1px solid var(--lui-input); border-radius: var(--lui-radius-sm);
  font-size: 0.875rem; transition: border-color var(--lui-duration-fast);
}
.lui-input-group > :is(input, .lui-input-slots) { order: 3; flex: 1; min-width: 0; min-height: calc(var(--lui-control-h) - 2px); border: 0; border-radius: 0; background: transparent; }
.lui-input-group > :is(input, .lui-input-slots):focus-visible { outline: none; }
.lui-input-group:has(input:focus-visible) { border-color: var(--lui-ring); outline: 3px solid color-mix(in srgb, var(--lui-ring) 50%, transparent); }
.lui-input-group:has(input[aria-invalid=true]) { border-color: var(--lui-danger); }
.lui-input-group:has(input:user-invalid) { border-color: var(--lui-danger); }
.lui-input-group:has(input:disabled) { background: var(--lui-gray-2); border-color: var(--lui-gray-6); }
.lui-input-group-prefix, .lui-input-group-suffix {
  display: inline-flex; align-items: center; flex: none; padding-inline: 0.75rem; white-space: nowrap;
  color: var(--lui-muted); background: var(--lui-gray-2);
}
.lui-input-group-prefix { order: 1; border-right: 1px solid var(--lui-input); border-radius: calc(var(--lui-radius-sm) - 1px) 0 0 calc(var(--lui-radius-sm) - 1px); }
.lui-input-group-suffix { order: 5; border-left: 1px solid var(--lui-input); border-radius: 0 calc(var(--lui-radius-sm) - 1px) calc(var(--lui-radius-sm) - 1px) 0; }
.lui-input-group-button { order: 6; display: flex; flex: none; border-left: 1px solid var(--lui-input); }
.lui-input-group-button > .lui-button { min-height: 100%; border: 0; border-radius: 0 calc(var(--lui-radius-sm) - 1px) calc(var(--lui-radius-sm) - 1px) 0; }
/* A number's steppers: Origin UI's chevrons stacked at the end. They come first in the
   markup (the pressed one's value posts first) and show last. */
.lui-input-steppers { order: 4; display: flex; flex-direction: column; flex: none; border-left: 1px solid var(--lui-input); }
.lui-input-steppers > .lui-button { flex: 1; min-height: 0; height: auto; width: 1.75rem; padding: 0; border: 0; border-radius: 0; }
.lui-input-steppers > .lui-button + .lui-button { border-top: 1px solid var(--lui-input); }
.lui-input-steppers .lui-icon { width: 0.75rem; height: 0.75rem; }
.lui-input-reveal { order: 6; display: flex; flex: none; }
.lui-input-reveal[hidden] { display: none; }
[data-lui-js] .lui-input-reveal[hidden] { display: flex; }
.lui-input-reveal > .lui-button { min-height: 100%; border-radius: 0 calc(var(--lui-radius-sm) - 1px) calc(var(--lui-radius-sm) - 1px) 0; }
/* Touch: a whole row (label and control) is at least 44px tall. */
@media (pointer: coarse) {
  .lui-field-check label, .lui-radio-group-option { min-height: var(--lui-hit); }
  .lui-input-group { min-height: var(--lui-hit); }
  .lui-input-steppers { flex-direction: row; }
  .lui-input-steppers > .lui-button { width: var(--lui-hit); }
  .lui-input-steppers > .lui-button + .lui-button { border-top: 0; border-left: 1px solid var(--lui-input); }
}
"#;

#[cfg(test)]
pub(crate) use tests::assert_described_ids_exist;

#[cfg(test)]
mod tests {
    use super::*;

    /// Every id an `aria-describedby` names is the id of an element in `html`.
    pub(crate) fn assert_described_ids_exist(html: &str) {
        for part in html.split("aria-describedby=\"").skip(1) {
            let ids = &part[..part.find('"').unwrap()];
            for id in ids.split(' ') {
                assert!(
                    html.contains(&format!("id=\"{id}\"")),
                    "aria-describedby names `{id}`, which is not in: {html}"
                );
            }
        }
    }

    #[test]
    fn a_hidden_label_keeps_what_describes_the_control() {
        let ui = Ui::default();
        let fields = [
            ui.input("q", "Search")
                .hide_label()
                .help("Words.")
                .error("Too short.")
                .maxlength(9),
            ui.input("q", "Search")
                .hide_label()
                .leading("in:")
                .prefix("$")
                .suffix("kg"),
            ui.input("bio", "Bio")
                .textarea(2)
                .hide_label()
                .help("Short."),
            ui.input("n", "N")
                .number(0, 9)
                .steppers()
                .hide_label()
                .error("No."),
        ];
        for f in fields {
            let html = f.render().into_string();
            assert!(!html.contains("<label"), "{html}");
            assert_described_ids_exist(&html);
        }
        let s = ui
            .select("s", "Sort")
            .options([("a", "A")])
            .hide_label()
            .help("h")
            .error("e");
        let html = s.render().into_string();
        assert!(html.contains(r#"aria-label="Sort""#) && !html.contains("<label"));
        assert_described_ids_exist(&html);
    }

    #[test]
    fn multiple_before_file_keeps_the_flag() {
        let ui = Ui::default();
        let html = ui
            .input("cv", "CV")
            .multiple()
            .file(".pdf")
            .render()
            .into_string();
        assert!(html.contains(r#"accept=".pdf" multiple"#), "{html}");
        let text = ui.input("t", "T").multiple().render().into_string();
        assert!(
            !text.contains("multiple"),
            "only a file picker takes several"
        );
    }

    #[test]
    fn textarea_and_select_honour_the_control_attributes() {
        let ui = Ui::default();
        let t = ui
            .input("bio", "Bio")
            .textarea(3)
            .autocomplete("off")
            .form("other")
            .inputmode("text")
            .autofocus()
            .class("x")
            .render()
            .into_string();
        for a in [
            r#"autocomplete="off""#,
            r#"form="other""#,
            r#"inputmode="text""#,
            "autofocus",
            r#"class="x""#,
        ] {
            assert!(t.contains(a), "{a} in {t}");
        }
        let s = ui
            .fields()
            .select("plan", "Plan", ["a", "b"])
            .autocomplete("off")
            .render()
            .into_string();
        assert!(s.contains(r#"autocomplete="off""#), "{s}");
    }

    #[test]
    fn disabled_and_readonly_controls() {
        let ui = Ui::default();
        let d = ui.input("a", "A").disabled().render().into_string();
        assert!(d.contains(r#"disabled aria-disabled="true""#), "{d}");
        let r = ui
            .input("a", "A")
            .value("x")
            .readonly()
            .render()
            .into_string();
        assert!(r.contains("readonly") && !r.contains("disabled"), "{r}");
        let c = ui
            .checkbox("ok", "OK")
            .checked(true)
            .readonly()
            .render()
            .into_string();
        assert!(
            c.contains("disabled") && c.contains(r#"<input type="hidden" name="ok" value="true">"#),
            "{c}"
        );
        let s = ui
            .select("s", "S")
            .options([("a", "A")])
            .value("a")
            .readonly()
            .render()
            .into_string();
        assert!(
            s.contains("disabled") && s.contains(r#"<input type="hidden" name="s" value="a">"#),
            "{s}"
        );
    }

    #[test]
    fn numbers_take_any_bound_and_drop_a_trailing_zero() {
        let ui = Ui::default();
        let n = ui
            .input("w", "Weight")
            .number(0.5, None::<f64>)
            .step(0.5)
            .render()
            .into_string();
        assert!(
            n.contains(r#"min="0.5" step="0.5""#) && !n.contains("max="),
            "{n}"
        );
        let n = ui.input("n", "N").number(1, 10).render().into_string();
        assert!(n.contains(r#"min="1" max="10""#), "{n}");
        assert_eq!(tidy(0.1 + 0.2, 0.1, 0.0), 0.3);
    }

    #[test]
    fn new_kinds_and_marks() {
        let ui = Ui::default();
        for (f, t) in [
            (ui.input("p", "Phone").tel(), "tel"),
            (ui.input("u", "Site").url(), "url"),
            (ui.input("m", "Month").month("2026-01", ""), "month"),
            (ui.input("w", "Week").week("", "2026-W52"), "week"),
        ] {
            assert!(f.render().into_string().contains(&format!(r#"type="{t}""#)));
        }
        let o = ui
            .input("nick", "Nickname")
            .optional()
            .render()
            .into_string();
        assert!(
            o.contains(r#"<span class="lui-field-optional">(optional)</span>"#),
            "{o}"
        );
    }

    #[test]
    fn addons_and_the_reveal_button() {
        let ui = Ui::default();
        let a = ui
            .input("price", "Price")
            .prefix("$")
            .suffix("USD")
            .render()
            .into_string();
        assert!(a.contains(r#"<div class="lui-input-group"><span id="f-price-prefix" class="lui-input-group-prefix">$</span>"#), "{a}");
        assert_described_ids_exist(&a);
        let p = ui
            .input("pw", "Password")
            .password()
            .reveal()
            .render()
            .into_string();
        assert!(
            p.contains(r#"class="lui-input-reveal" data-lui="reveal" hidden"#),
            "{p}"
        );
    }
    #[test]
    fn passwords_and_files_are_never_echoed() {
        let ui = Ui::default();
        let p = ui
            .input("pw", "Password")
            .password()
            .value("hunter2")
            .render()
            .into_string();
        assert!(p.contains(r#"type="password""#) && !p.contains("hunter2"));
        let f = ui
            .input("cv", "CV")
            .file(".pdf")
            .multiple()
            .value("x")
            .render()
            .into_string();
        assert!(f.contains(r#"accept=".pdf" multiple"#) && !f.contains(r#"value="x""#));
    }

    #[test]
    fn a_radio_group_needs_one_choice_and_describes_itself() {
        let ui = Ui::default();
        let g = ui
            .radio_group("size", "Size")
            .option("s", "Small")
            .option("m", "Medium")
            .required()
            .error("Pick one.")
            .render()
            .into_string();
        assert!(
            g.contains(r#"aria-describedby="f-size-error""#)
                && g.contains(r#"aria-invalid="true""#)
        );
        assert_eq!(
            g.matches(" required").count(),
            1,
            "one required radio covers the group"
        );
        assert!(
            g.contains(
                r#"<label for="f-size-1" class="lui-radio-group-option"><input id="f-size-1" type="radio" name="size" value="m">"#
            ),
            "{g}"
        );
    }

    #[test]
    fn an_id_overrides_the_default() {
        let m = Ui::default()
            .input("q", "Search")
            .id("search")
            .render()
            .into_string();
        assert!(m.contains(r#"<label for="search">"#) && m.contains(r#"id="search""#));
    }
}
