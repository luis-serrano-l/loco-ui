//! # Form
//!
//! A validated form: the browser blocks bad input before submit, the server checks again and
//! re-renders with messages. Fields come in groups with legends, carry help text, count their
//! characters, take files, dates, times and bounded numbers, and lay out stacked (label above)
//! or inline (label beside). No script needed.
//!
//! The look follows the shadcn Forms examples and the Radix Themes settings layouts: a group
//! is a section (a heading, an optional `.description(..)` and its fields) drawn without a
//! frame, a hairline between sections. The form is its own container: `.inline()` puts the
//! label and its help beside the field once the form is 30rem wide, and the submit button
//! fills a narrower form and sits at the end of a wider one.
//!
//! **Platform features:**
//! - Constraint validation attributes `required`, `pattern`, `min`, `max`, `maxlength`,
//!   `type=email` (baseline 2015); `type=date` and `type=time` with `min`/`max` (Chrome 20,
//!   Firefox 57, Safari 14.1).
//! - `:user-invalid` / `:user-valid` (baseline 2023): styles only after the user has interacted,
//!   so fields are not red on first paint.
//! - `<fieldset>` + `<legend>` per [`Form::group`], described by its `.description(..)`; help
//!   text and the error are tied to the field with `aria-describedby`.
//! - Container queries (Chrome 105, Firefox 110, Safari 16) for the side labels and the
//!   submit row.
//! - `<output>` counts characters for a field with a `maxlength`: the server renders the count
//!   of the value it has, the enhancement script keeps it live while typing.
//! - `<input type=file accept>` (baseline 2015); any file field makes the form
//!   `enctype="multipart/form-data"`.
//! - `field-sizing` (Chrome 123, not yet in Firefox or Safari): a textarea grows with its
//!   content.
//! - Post/Redirect/Get for success; on error the server re-renders the form with values and
//!   messages, and an [error summary](crate::error_summary) at the top that links to each
//!   field in error and takes the focus.
//!
//! **Accessibility:** labelled fields grouped in `<fieldset>`/`<legend>`, native constraint
//! validation, and an error summary that takes the focus when the server sends the form back.
//! Checked by axe-core in headless Firefox on every demo route, both capability variants, light
//! and dark (no serious or critical violation).
//!
//! **What it does not do without script:** validate against the server as you type, or warn
//! about unsaved changes on leaving the page.
//!
//! **Fallback:** without container queries the labels stay above the fields and the button
//! fills the form's width. Without `field-sizing` a textarea keeps its `rows` and can be resized by
//! hand. Without the script the counter shows the length of the last submitted value and
//! `maxlength` still stops input at the limit.
//!
//! **Enhanced:** the form is a swap root, so with the [`crate::enhance`] script a submit
//! replaces only the form (errors included) and a successful redirect swaps in the result.
//!
//! **Finding:** custom cross-field rules (password confirmation, async uniqueness) only run
//! on the server round trip, and warning about unsaved changes when leaving needs script.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let m = ui.form("/signup").email("email", "Email").required();
//! assert!(m.render().into_string().contains(r#"type="email" value="" required"#));
//!
//! // `required`, `help`, `maxlength`, `value` and friends apply to the field added last;
//! // `group` starts a fieldset for the fields after it.
//! let m = ui.form("/profile")
//!     .group("About you").description("Shown on your public page.")
//!     .textarea("bio", "Bio", 3).maxlength(280).value("Hi").help("Shown on your profile.")
//!     .file("avatar", "Avatar", "image/png,image/jpeg")
//!     .date("born", "Born", "1900-01-01", "2026-12-31")
//!     .select("digest", "Digest", ["daily", "weekly", "never"]).value("weekly")
//!     .submit("Save profile")
//!     .inline();
//! let html = m.render().into_string();
//! assert!(html.contains("enctype=\"multipart/form-data\""));
//! assert!(html.contains(r#"<legend class="lui-form-group-legend">About you</legend>"#));
//! assert!(html.contains(r#"aria-describedby="lui-form-about-you-description""#));
//! assert!(html.contains(">2 / 280</output>"));
//! assert!(html.contains("accept=\"image/png,image/jpeg\""));
//! assert!(html.contains(r#"<option value="weekly" selected>"#));
//!
//! // The same in `lui!`:
//! let same = lui! { Form("/profile") submit="Save profile" inline {
//!     group "About you" description="Shown on your public page.";
//!     textarea "bio" "Bio" 3 maxlength=280 value="Hi" help="Shown on your profile.";
//!     file "avatar" "Avatar" "image/png,image/jpeg";
//!     date "born" "Born" "1900-01-01" "2026-12-31";
//!     select "digest" "Digest" (["daily", "weekly", "never"]) value="weekly";
//! } };
//! assert_eq!(same.into_string(), m.render().into_string());
//! ```
//!
//! **Any control.** `.switch(..)` adds a switch like a checkbox, and `.body(..)` puts any
//! markup at that point among the fields (a date picker, a range, a toggle group, a select
//! built with its own setters), so it sits inside the form's swap root, under its error
//! summary and before its button. `.get()` makes it a GET form: a filter or a search whose
//! answer is a page with the choices in its URL.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/filters", "size=l", "");
//! let m = ui.form("/filters")
//!     .get()
//!     .text("q", "Search")
//!     .switch("open", "Open only").checked(true)
//!     .body(html! { (ui.select("size", "Size").options([("s", "Small"), ("l", "Large")])) })
//!     .submit("Apply");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"method="get" action="/filters""#));
//! assert!(html.contains(r#"role="switch""#) && html.contains(r#"<option value="l" selected>"#));
//! assert!(html.find("f-size") < html.find("lui-form-actions"), "the markup before the button");
//! // The same in `lui!`: a block of markup among the items is `body { .. }`, and a block with
//! // no items at all is the body.
//! let same = lui! { Form("/filters") get submit="Apply" {
//!     text "q" "Search";
//!     switch "open" "Open only" checked=(true);
//!     body { Select("size", "Size") options=([("s", "Small"), ("l", "Large")]); }
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::{Strings, Text};
use crate::input::{Bound, Choice, Field, FieldKind, Pick};
use crate::props::{Key, Prop, PropKind};
use crate::select::SelectOption;
use crate::variant::Size;
use crate::{Caps, Ui, enhance};

/// What a [`Form`] added last, which its modifiers check: `.required()` after a group is a
/// mistake, loud in a debug build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Last {
    /// Nothing yet.
    Nothing,
    /// A field a person sees.
    Field,
    /// A hidden field.
    Hidden,
    /// Markup from `.body(..)`.
    Body,
    /// A group's legend.
    Group,
}

/// A POST form of fields, made by [`Ui::form`], or the fields alone, made by [`Ui::fields`].
/// Fields are added in order; `required`, `help`, `maxlength`, `value`, `error`,
/// `placeholder`, `multiple`, `checked`, `autocomplete`, `disabled`, `readonly`, `optional`
/// and `width` apply to the field added last, and `description` to the group. A stacked
/// form with a "Submit" button unless told otherwise.
///
/// **Setters.** Values and items: `.values(..)`, `.errors(..)`, `.group(..)`, `.description(..)`, `.text(..)`, `.password(..)`,
/// `.email(..)`, `.number(..)`, `.pattern(..)`, `.textarea(..)`, `.file(..)`, `.date(..)`,
/// `.time(..)`, `.datetime(..)`, `.select(..)`, `.radio(..)`, `.checkboxes(..)`, `.search(..)`,
/// `.url(..)`, `.tel(..)`, `.color(..)`, `.range(..)`, `.checkbox(..)`, `.switch(..)`, `.hidden(..)`,
/// `.body(..)`, `.help(..)`, `.maxlength(..)`,
/// `.value(..)`, `.error(..)`, `.placeholder(..)`, `.autocomplete(..)`, `.width(..)`, `.submit(..)`,
/// `.action_secondary(..)`, `.method_override(..)`, `.id(..)`; switches:
/// `.required()`, `.optional()`, `.disabled()`, `.readonly()`, `.multiple()`, `.inline()`, `.get()`; from a condition: `.checked(bool)`.
#[derive(Clone, Debug)]
pub struct Form<'a> {
    action: Option<&'a str>,
    get: bool,
    groups: Vec<Group<'a>>,
    submit: &'a str,
    secondary: Option<(&'a str, &'a str)>,
    method: Option<&'a str>,
    inline: bool,
    values: &'a [(String, String)],
    errors: &'a [(&'a str, &'a str)],
    id: Option<&'a str>,
    last: Last,
    caps: Caps,
    strings: &'static Strings,
}

/// A section of a form: its legend and description (none for the fields before the first
/// `.group(..)`) and its fields.
#[derive(Clone, Debug, Default)]
pub(crate) struct Group<'a> {
    legend: Option<&'a str>,
    description: Option<&'a str>,
    pub(crate) fields: Vec<Field<'a>>,
}

/// What every field modifier applies to.
const FIELD: &[&str] = &["field"];

impl Form<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves through the fields in order, then to the submit button and the secondary link.",
        ),
        Key::new(
            "Enter",
            "Submits the form from a single-line field, as its submit button does, even when markup among the fields holds buttons of its own.",
        ),
        Key::new("Enter / Space", "Presses the focused submit button."),
        Key::new(
            "Enter",
            "Moves focus to the field from a link in the error summary.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("group", PropKind::Item, "legend: &'a str")
            .doc("A `<fieldset>` with this `<legend>` around the fields added after it."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .applies_to(&["group"])
            .doc("A line under the legend of the group added last."),
        Prop::new("text", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("Single-line text."),
        Prop::new("password", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("`type=\"password\"`."),
        Prop::new("email", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("`type=\"email\"`."),
        Prop::new("number", PropKind::Item, "name: &'a str, label: &'a str, min: impl Into<Bound>, max: impl Into<Bound>")
            .doc("A number from `min` to `max`, inclusive."),
        Prop::new("pattern", PropKind::Item, "name: &'a str, label: &'a str, pattern: &'a str, hint: &'a str")
            .doc("Text that must match `pattern` (the HTML `pattern` attribute)."),
        Prop::new("textarea", PropKind::Item, "name: &'a str, label: &'a str, rows: u8")
            .doc("Multi-line text, `rows` high."),
        Prop::new("file", PropKind::Item, "name: &'a str, label: &'a str, accept: &'a str")
            .doc("A file picker."),
        Prop::new("date", PropKind::Item, "name: &'a str, label: &'a str, min: &'a str, max: &'a str")
            .doc("`type=\"date\"`, bounds as `YYYY-MM-DD`."),
        Prop::new("time", PropKind::Item, "name: &'a str, label: &'a str, min: &'a str, max: &'a str")
            .doc("`type=\"time\"`, bounds as `HH:MM`."),
        Prop::new("datetime", PropKind::Item, "name: &'a str, label: &'a str, min: &'a str, max: &'a str")
            .doc("`type=\"datetime-local\"`, bounds as `YYYY-MM-DDTHH:MM`."),
        Prop::new("select", PropKind::Item, "name: &'a str, label: &'a str, options: impl IntoIterator<Item = O>")
            .doc("A `<select>` of `options`: `&str`s, or `(value, label)` pairs."),
        Prop::new("radio", PropKind::Item, "name: &'a str, label: &'a str, options: impl IntoIterator<Item = O>")
            .doc("Radio buttons under a legend, one per option: `&str`s, or `(value, label)` pairs."),
        Prop::new("checkboxes", PropKind::Item, "name: &'a str, label: &'a str, options: impl IntoIterator<Item = O>")
            .doc("Checkboxes under a legend posting `<name>[]`, one per option."),
        Prop::new("search", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("`type=\"search\"`."),
        Prop::new("url", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("`type=\"url\"`."),
        Prop::new("tel", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("`type=\"tel\"`."),
        Prop::new("color", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("A colour picker with its swatch."),
        Prop::new("range", PropKind::Item, "name: &'a str, label: &'a str, min: impl Into<f64>, max: impl Into<f64>")
            .doc("A slider from `min` to `max`, its value beside it."),
        Prop::new("checkbox", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("A checkbox posting `true` when ticked and nothing when not (so a `bool` with `#[serde(default)]` reads it)."),
        Prop::new("switch", PropKind::Item, "name: &'a str, label: &'a str")
            .doc("A checkbox drawn as an on/off switch, posting like `.checkbox(..)`."),
        Prop::new("hidden", PropKind::Item, "name: &'a str, value: &'a str")
            .doc("`type=\"hidden\"`."),
        Prop::new("body", PropKind::Item, "markup: Markup")
            .doc("Any markup at this point among the fields: a control built with its own setters."),
        Prop::new("required", PropKind::Modifier, "").applies_to(FIELD)
            .doc("The `required` attribute, and a `*` after the label; the form says what the star means."),
        Prop::new("optional", PropKind::Modifier, "").applies_to(FIELD)
            .doc("\"(optional)\" after the label."),
        Prop::new("disabled", PropKind::Modifier, "").applies_to(FIELD).attr("disabled")
            .doc("`disabled` and `aria-disabled`: shown greyed and not posted."),
        Prop::new("readonly", PropKind::Modifier, "").applies_to(FIELD)
            .doc("Shown and posted, not editable; a posted value never replaces it."),
        Prop::new("help", PropKind::Modifier, "help: &'a str").applies_to(FIELD)
            .doc("Help text under the field."),
        Prop::new("maxlength", PropKind::Modifier, "max: usize").applies_to(FIELD)
            .doc("`maxlength`, counted in an `<output>` under the field."),
        Prop::new("value", PropKind::Modifier, "value: &'a str").applies_to(&["field", "hidden"]).attr("value")
            .doc("The field's current value (ignored for files)."),
        Prop::new("checked", PropKind::Modifier, "checked: bool").applies_to(FIELD)
            .doc("Tick the checkbox."),
        Prop::new("error", PropKind::Modifier, "message: &'a str").applies_to(FIELD)
            .doc("A server message beside the field."),
        Prop::new("placeholder", PropKind::Modifier, "placeholder: &'a str").applies_to(FIELD)
            .doc("Placeholder text."),
        Prop::new("multiple", PropKind::Modifier, "").applies_to(FIELD)
            .doc("The file picker takes several files; a select, several options."),
        Prop::new("autocomplete", PropKind::Modifier, "value: &'a str").applies_to(FIELD).attr("autocomplete")
            .doc("`autocomplete` (`\"email\"`, `\"new-password\"`, `\"off\"`)."),
        Prop::new("width", PropKind::Modifier, "width: impl Into<Size>").applies_to(FIELD)
            .doc("A narrower control: 1 (8rem), 2 (16rem) or 3 (24rem)."),
        Prop::new("submit", PropKind::Value, "label: &'a str").default("Submit")
            .doc("Label of the submit button."),
        Prop::new("action_secondary", PropKind::Value, "label: &'a str, href: &'a str")
            .doc("A second action beside the submit button, a link (Cancel, Back)."),
        Prop::new("method_override", PropKind::Value, "method: &'a str")
            .doc("A hidden `_method` (`\"delete\"`, `\"put\"`) that `loco::method_override` turns into the request's method."),
        Prop::new("inline", PropKind::Switch, "")
            .doc("Labels beside the fields on screens wider than 40rem, above them on narrower ones."),
        Prop::new("get", PropKind::Switch, "")
            .doc("`method=\"get\"`: the fields go in the URL of the page it answers with (a filter, a search)."),
        Prop::new("values", PropKind::Value, "values: &'a [(String, String)]")
            .doc("Submitted values by field name, as a form post parses them."),
        Prop::new("errors", PropKind::Value, "errors: &'a [(&'a str, &'a str)]")
            .doc("Server messages `(field name, message)`."),
        Prop::new("id", PropKind::Value, "id: &'a str").attr("id")
            .doc("What the swap root's id is built from, when two forms on a page post to the same action (one per tab, say)."),
    ];
}

impl Ui {
    /// A form posting to `action`; add fields with [`Form::text`] and friends.
    pub fn form<'a>(&self, action: &'a str) -> Form<'a> {
        Form {
            action: Some(action),
            ..self.fields()
        }
    }

    /// Fields with no `<form>` around them, for a form built elsewhere (a wizard step, a
    /// dialog's confirm form).
    pub fn fields<'a>(&self) -> Form<'a> {
        Form {
            action: None,
            get: false,
            groups: vec![Group::default()],
            submit: self.text(Text::Submit),
            secondary: None,
            method: None,
            strings: self.strings,
            caps: self.caps,
            inline: false,
            values: &[],
            errors: &[],
            id: None,
            last: Last::Nothing,
        }
    }
}

impl<'a> Form<'a> {
    fn add(mut self, name: &'a str, label: &'a str, kind: FieldKind<'a>) -> Self {
        self.last = match kind {
            FieldKind::Hidden => Last::Hidden,
            FieldKind::Markup(_) => Last::Body,
            _ => Last::Field,
        };
        let mut field = Field::new(name, label, kind);
        field.caps = self.caps;
        field.strings = self.strings;
        if let Some(group) = self.groups.last_mut() {
            group.fields.push(field);
        }
        self
    }

    /// Change the field added last, which a modifier named `method` applies to; `hidden`
    /// says whether a hidden field will do. Anything else fails a debug assertion.
    fn last(mut self, method: &str, hidden: bool, change: impl FnOnce(&mut Field<'a>)) -> Self {
        debug_assert!(
            self.last == Last::Field || (hidden && self.last == Last::Hidden),
            "Form::{method}() applies to a field, but the last item was {:?}",
            self.last
        );
        if let Some(f) = self.groups.last_mut().and_then(|g| g.fields.last_mut()) {
            change(f);
        }
        self
    }

    /// A `<fieldset>` with this `<legend>` around the fields added after it.
    pub fn group(mut self, legend: &'a str) -> Self {
        self.groups.push(Group {
            legend: Some(legend),
            ..Group::default()
        });
        self.last = Last::Group;
        self
    }

    /// A line under the legend of the group added last, what the section is for; the
    /// fieldset is described by it. Call it right after `.group(..)`.
    pub fn description(mut self, text: &'a str) -> Self {
        debug_assert!(
            self.last == Last::Group,
            "Form::description() applies to a group, but the last item was {:?}",
            self.last
        );
        if let Some(g) = self.groups.last_mut().filter(|g| g.legend.is_some()) {
            g.description = Some(text);
        }
        self
    }

    /// Single-line text.
    pub fn text(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Text)
    }

    /// `type="password"`: the value is never written back into the page, even on an error.
    pub fn password(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Password)
    }

    /// `type="email"`: the browser checks the shape.
    pub fn email(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Email)
    }

    /// A number from `min` to `max`, inclusive: any number (`13`, `0.5`), or `None::<f64>`
    /// for no bound.
    pub fn number(
        self,
        name: &'a str,
        label: &'a str,
        min: impl Into<Bound>,
        max: impl Into<Bound>,
    ) -> Self {
        let (min, max) = (min.into().0, max.into().0);
        self.add(name, label, FieldKind::Number { min, max })
    }

    /// Text that must match `pattern` (the HTML `pattern` attribute); `hint` explains the
    /// rule under the field and as the input's `title`.
    pub fn pattern(self, name: &'a str, label: &'a str, pattern: &'a str, hint: &'a str) -> Self {
        self.add(name, label, FieldKind::Pattern { pattern, hint })
    }

    /// Multi-line text, `rows` high; it grows with its content where `field-sizing` works.
    pub fn textarea(self, name: &'a str, label: &'a str, rows: u8) -> Self {
        self.add(name, label, FieldKind::Textarea { rows })
    }

    /// A file picker; `accept` lists MIME types or extensions (`image/*,.pdf`), empty for
    /// any. The form becomes `multipart/form-data`.
    pub fn file(self, name: &'a str, label: &'a str, accept: &'a str) -> Self {
        self.add(name, label, FieldKind::File { accept })
    }

    /// `type="date"`, bounds as `YYYY-MM-DD`; an empty bound is left out.
    pub fn date(self, name: &'a str, label: &'a str, min: &'a str, max: &'a str) -> Self {
        self.add(name, label, FieldKind::Date { min, max })
    }

    /// `type="time"`, bounds as `HH:MM`; an empty bound is left out.
    pub fn time(self, name: &'a str, label: &'a str, min: &'a str, max: &'a str) -> Self {
        self.add(name, label, FieldKind::Time { min, max })
    }

    /// `type="datetime-local"`, bounds as `YYYY-MM-DDTHH:MM`; an empty bound is left out.
    /// The browser posts `2026-01-31T09:00`, with no seconds and no offset.
    pub fn datetime(self, name: &'a str, label: &'a str, min: &'a str, max: &'a str) -> Self {
        self.add(name, label, FieldKind::DateTime { min, max })
    }

    /// A `<select>` of `options`: each a `&str` that is its own value and text, or a
    /// `(value, label)` pair (a row's id and its name). Drawn by [`crate::select`], so it
    /// takes the same look as `ui.select(..)`.
    pub fn select<O: Into<Choice<'a>>>(
        self,
        name: &'a str,
        label: &'a str,
        options: impl IntoIterator<Item = O>,
    ) -> Self {
        let options: Vec<SelectOption<'a>> = options.into_iter().map(|o| o.into().into()).collect();
        self.add(name, label, FieldKind::Select(options))
    }

    /// Radio buttons named `name` under the legend `label`, one per option: `&str`s, or
    /// `(value, label)` pairs. Drawn by `ui.radio_group(..)`.
    pub fn radio<O: Into<Choice<'a>>>(
        self,
        name: &'a str,
        label: &'a str,
        options: impl IntoIterator<Item = O>,
    ) -> Self {
        let picks: Vec<Pick<'a>> = options.into_iter().map(|o| o.into().into()).collect();
        self.add(name, label, FieldKind::Radio(picks))
    }

    /// Checkboxes posting `<name>[]` under the legend `label`, one per option; the form's
    /// `.values(..)` ticks every `<name>[]` posted. Drawn by `ui.checkbox_group(..)`.
    pub fn checkboxes<O: Into<Choice<'a>>>(
        self,
        name: &'a str,
        label: &'a str,
        options: impl IntoIterator<Item = O>,
    ) -> Self {
        let picks: Vec<Pick<'a>> = options.into_iter().map(|o| o.into().into()).collect();
        self.add(name, label, FieldKind::Checkboxes(picks))
    }

    /// `type="search"`: a search field among the others, with the browser's clear button.
    pub fn search(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Search)
    }

    /// `type="url"`: the browser checks it is an absolute URL.
    pub fn url(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Url)
    }

    /// `type="tel"`: a phone number, the phone keypad on a touch screen.
    pub fn tel(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Tel)
    }

    /// A colour picker with its swatch, `#000000` until `.value(..)` or a posted value.
    pub fn color(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Color)
    }

    /// A slider from `min` to `max` in steps of 1, its value beside it; at the middle until
    /// `.value(..)` or a posted value.
    pub fn range(
        self,
        name: &'a str,
        label: &'a str,
        min: impl Into<f64>,
        max: impl Into<f64>,
    ) -> Self {
        let (min, max) = (min.into(), max.into());
        self.add(name, label, FieldKind::Range { min, max })
    }

    /// A checkbox posting `true` when ticked and nothing when not (so a `bool` with
    /// `#[serde(default)]` reads it).
    pub fn checkbox(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Checkbox)
    }

    /// A checkbox drawn as an on/off switch, with `role="switch"`, posting like
    /// [`Form::checkbox`]; `.checked(..)` turns it on.
    pub fn switch(self, name: &'a str, label: &'a str) -> Self {
        self.add(name, label, FieldKind::Switch)
    }

    /// `type="hidden"`: posted with the form, not shown.
    pub fn hidden(self, name: &'a str, value: &'a str) -> Self {
        self.add(name, "", FieldKind::Hidden).value(value)
    }

    /// Any markup at this point among the fields: a control built with its own setters (a
    /// date picker, a range, a toggle group, a select), inside the form and before its
    /// button. The form's values and errors do not reach it; give it its own `.value(..)`.
    pub fn body(self, markup: Markup) -> Self {
        self.add("", "", FieldKind::Markup(markup))
    }

    /// The `required` attribute, and a `*` after the label; a form with a required field
    /// says "* required" ([`Text::RequiredHint`]) above its button.
    pub fn required(self) -> Self {
        self.last("required", false, |f| f.required = true)
    }

    /// "(optional)" after the label, for a form whose fields are mostly required.
    pub fn optional(self) -> Self {
        self.last("optional", false, |f| f.optional = true)
    }

    /// `disabled` and `aria-disabled`: shown greyed and not posted.
    pub fn disabled(self) -> Self {
        self.last("disabled", false, |f| f.disabled = true)
    }

    /// Shown and posted, not editable: `readonly` on a text field, a disabled control and a
    /// hidden input where HTML has no `readonly`. A posted value never replaces its own, so a
    /// tampered post shows the value the server set.
    pub fn readonly(self) -> Self {
        self.last("readonly", false, |f| f.readonly = true)
    }

    /// Help text under the field.
    pub fn help(self, help: &'a str) -> Self {
        self.last("help", false, |f| f.help = Some(help))
    }

    /// `maxlength`, counted in an `<output>` under the field.
    pub fn maxlength(self, max: usize) -> Self {
        self.last("maxlength", false, |f| f.maxlength = Some(max))
    }

    /// The field's current value (ignored for files). A checkbox is ticked by `true`, `on`
    /// or `1`.
    pub fn value(self, value: &'a str) -> Self {
        self.last("value", true, |f| f.value = value)
    }

    /// Tick the checkbox.
    pub fn checked(self, checked: bool) -> Self {
        self.last("checked", false, |f| {
            f.value = if checked { "true" } else { "" };
        })
    }

    /// A server message beside the field.
    pub fn error(self, message: &'a str) -> Self {
        self.last("error", false, |f| {
            f.error = (!message.is_empty()).then_some(message);
        })
    }

    /// Placeholder text.
    pub fn placeholder(self, placeholder: &'a str) -> Self {
        self.last("placeholder", false, |f| f.placeholder = Some(placeholder))
    }

    /// The file picker takes several files; a select, several options.
    pub fn multiple(self) -> Self {
        self.last("multiple", false, |f| f.multiple = true)
    }

    /// `autocomplete` (`"email"`, `"new-password"`, `"off"`): what the browser may fill in.
    pub fn autocomplete(self, value: &'a str) -> Self {
        self.last("autocomplete", false, |f| {
            f.extra.autocomplete = Some(value);
        })
    }

    /// A narrower control: 1 (8rem), 2 (16rem) or 3 (24rem), never wider than the form.
    pub fn width(self, width: impl Into<Size>) -> Self {
        let width = width.into();
        self.last("width", false, |f| f.extra.width = Some(width))
    }

    /// Label of the submit button.
    pub fn submit(mut self, label: &'a str) -> Self {
        self.submit = label;
        self
    }

    /// A second action beside the submit button: a link (`"Cancel"` back to the list), so it
    /// leaves without posting.
    pub fn action_secondary(mut self, label: &'a str, href: &'a str) -> Self {
        self.secondary = Some((label, href));
        self
    }

    /// A hidden `_method` field (`"delete"`, `"put"`, `"patch"`): an HTML form can only send
    /// GET or POST, so the post carries the method it means and
    /// `loco_ui::loco::method_override` (feature `axum`) rewrites the request before routing.
    pub fn method_override(mut self, method: &'a str) -> Self {
        self.method = Some(method);
        self
    }

    /// Labels beside the fields on screens wider than 40rem, above them on narrower ones.
    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }

    /// `method="get"`: the fields go in the URL of the page it answers with (a filter, a
    /// search), which reads them back with `ui.param(..)`. Still a swap root, so with the
    /// script the answer replaces the form in place.
    pub fn get(mut self) -> Self {
        self.get = true;
        self
    }

    /// Submitted values by field name, as a form post parses them: each field without its
    /// own `value` shows the one named after it (every `<name>[]` for checkboxes).
    pub fn values(mut self, values: &'a [(String, String)]) -> Self {
        self.values = values;
        self
    }

    /// Server messages `(field name, message)`: each field without its own `error` shows the
    /// one named after it.
    pub fn errors(mut self, errors: &'a [(&'a str, &'a str)]) -> Self {
        self.errors = errors;
        self
    }

    /// What the swap root's id is built from, when two forms on a page post to the same
    /// action (one per tab, say); the action by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Every field, with the values and errors filled in by name. A read-only field keeps
    /// its own value: a post cannot change what it shows.
    pub(crate) fn filled(&self) -> impl Iterator<Item = Group<'a>> + '_ {
        self.groups
            .iter()
            .filter(|g| g.legend.is_some() || !g.fields.is_empty())
            .map(|g| {
                let fs = g.fields.iter().map(|f| {
                    let key = match f.kind {
                        FieldKind::Checkboxes(_) => format!("{}[]", f.name),
                        _ => f.name.to_string(),
                    };
                    let posted: Vec<&'a str> = if f.value.is_empty() && !f.readonly {
                        (self.values.iter())
                            .filter(|(n, _)| *n == key)
                            .map(|(_, v)| v.as_str())
                            .collect()
                    } else {
                        Vec::new()
                    };
                    let value = posted.first().copied().unwrap_or(f.value);
                    let error = f.error.or_else(|| {
                        self.errors
                            .iter()
                            .find(|(n, _)| *n == f.name)
                            .map(|(_, m)| *m)
                    });
                    Field {
                        value,
                        values: if posted.is_empty() {
                            f.values.clone()
                        } else {
                            posted
                        },
                        error,
                        ..f.clone()
                    }
                });
                Group {
                    legend: g.legend,
                    description: g.description,
                    fields: fs.collect(),
                }
            })
    }

    /// The error summary: every field with a message, linked by its id and named by its
    /// label, then the messages that name no field.
    fn summary(&self) -> crate::error_summary::ErrorSummary<'a> {
        let fields: Vec<Field<'a>> = self.filled().flat_map(|g| g.fields).collect();
        let mut items: Vec<_> = fields
            .iter()
            .filter_map(|f| {
                let id = match f.kind {
                    // A group's first choice, which the summary's link focuses.
                    FieldKind::Radio(_) => format!("f-{}-0", f.name),
                    FieldKind::Checkboxes(_) => {
                        f.id.map_or_else(|| format!("f-{}", crate::slug(f.name)), str::to_string)
                            + "-0"
                    }
                    _ => f.control_id(),
                };
                Some((Some(id), Some(f.label), f.error?))
            })
            .collect();
        let loose = self
            .errors
            .iter()
            .filter(|(n, _)| !fields.iter().any(|f| f.name == *n));
        items.extend(loose.map(|(_, m)| (None, None, *m)));
        crate::error_summary::ErrorSummary::from_fields(items)
            .title(self.strings.get(Text::Problem))
    }

    /// Whether any field has a server message.
    pub(crate) fn has_errors(&self) -> bool {
        self.filled()
            .any(|g| g.fields.iter().any(|f| f.error.is_some()))
    }

    fn fields(&self) -> Markup {
        html! {
            @for g in self.filled() {
                @if let Some(legend) = g.legend {
                    @let did = g.description.map(|_| format!("lui-form-{}-description", crate::slug(legend)));
                    fieldset class="lui-form-group" aria-describedby=[did.as_deref()] {
                        legend class="lui-form-group-legend" { (legend) }
                        @if let (Some(d), Some(did)) = (g.description, &did) {
                            p id=(did) class="lui-form-group-description" { (d) }
                        }
                        @for f in &g.fields { (f) }
                    }
                } @else {
                    @for f in &g.fields { (f) }
                }
            }
        }
    }
}

impl Render for Form<'_> {
    fn render(&self) -> Markup {
        let Some(action) = self.action else {
            return self.fields();
        };
        let multipart = self
            .groups
            .iter()
            .flat_map(|g| &g.fields)
            .any(|f| matches!(f.kind, FieldKind::File { .. }));
        let required = (self.groups.iter())
            .flat_map(|g| &g.fields)
            .any(|f| f.required);
        // Markup among the fields may hold submit buttons of its own (a stepper, a tag
        // input's Add); Enter in a field should still press the form's button.
        let foreign = (self.groups.iter())
            .flat_map(|g| &g.fields)
            .any(|f| matches!(&f.kind, FieldKind::Markup(m) if m.0.contains("<button")));
        let class = if self.inline {
            "lui-form lui-form-inline"
        } else {
            "lui-form"
        };
        html! {
            form id=(enhance::swap_id("lui-form", self.id.unwrap_or(action))) data-lui="swap" class=(class)
                method=(if self.get { "get" } else { "post" }) action=(action)
                enctype=[multipart.then_some("multipart/form-data")] {
                @if let Some(m) = self.method { input type="hidden" name="_method" value=(m); }
                @if foreign { (crate::button::default_submit(None)) }
                (self.summary())
                (self.fields())
                // The stars are hidden from screen readers (the `required` attribute speaks),
                // so the line that explains them is too.
                @if required { p class="lui-form-hint" aria-hidden="true" { (self.strings.get(Text::RequiredHint)) } }
                div class="lui-form-actions" {
                    (Button::new(Caps::NONE, self.submit).primary())
                    @if let Some((label, href)) = self.secondary { (Button::link(Caps::NONE, label, href)) }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the shadcn Forms examples and the Radix Themes settings layouts: a section is a
   heading, one line of description and its fields; the fieldset stays for its semantics but
   draws no frame, and a hairline and space separate one section from the next. The form is
   its own container, so its layout follows the box it sits in, not the viewport. */
.lui-form { container: lui-form / inline-size; display: grid; gap: var(--lui-space-6); max-width: 28rem; }
.lui-form-inline { max-width: 48rem; }
.lui-form-group { display: grid; gap: var(--lui-space-4); min-width: 0; margin: 0; padding: 0; border: 0; }
fieldset.lui-form-group ~ fieldset.lui-form-group { padding-top: var(--lui-space-6); border-top: 1px solid var(--lui-line); }
.lui-form-group-legend { float: inline-start; width: 100%; padding: 0; font-size: var(--lui-text-3); line-height: 1.5rem; font-weight: 600; }
/* Floated, a legend is no longer drawn in the fieldset's border but laid out as an ordinary
   grid item, so the gap spaces it; the description pulls up under it. */
.lui-form-hint { margin: 0; color: var(--lui-muted); font-size: var(--lui-text-2); line-height: 1.25rem; }
.lui-form-group-description { margin: calc(var(--lui-space-4) * -1 + var(--lui-space-1)) 0 0; color: var(--lui-muted); font-size: var(--lui-text-2); line-height: 1.25rem; }
/* The submit row: the button fills a narrow form, and sits at the end of a wide one. */
.lui-form-actions { display: flex; flex-direction: column; gap: var(--lui-space-2); }
.lui-form-actions > .lui-button { width: 100%; }
@container lui-form (width >= 30rem) {
  .lui-form-actions { flex-direction: row-reverse; justify-content: flex-start; }
  .lui-form-actions > .lui-button { width: auto; }
}
/* .inline(): from 30rem, the label and its help in a column beside the field (Radix
   settings); the counter and error stay under the field. */
@container lui-form (width >= 30rem) {
  .lui-form-inline .lui-field { grid-template-columns: minmax(8rem, 1fr) minmax(0, 2fr); column-gap: var(--lui-space-6); row-gap: var(--lui-space-1); }
  .lui-form-inline .lui-field > * { grid-column: 2; }
  /* The label lines up with the text inside the field, its help under it; the control spans
     both rows, so help adds no empty row beside it. */
  .lui-form-inline .lui-field > label { grid-column: 1; grid-row: 1; align-self: start; padding-top: calc((var(--lui-control-h) - 1.25rem) / 2); }
  .lui-form-inline .lui-field > .lui-field-help { grid-column: 1; grid-row: 2; align-self: start; }
  .lui-form-inline .lui-field:not(.lui-field-check) > label + * { grid-row: 1 / span 2; align-self: start; }
  .lui-form-inline .lui-field-check > label { grid-column: 2; }
  .lui-form-inline .lui-field-check > .lui-field-help { grid-column: 2; grid-row: auto; }
}
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
    fn enter_presses_the_forms_button_past_buttons_in_its_body() {
        let ui = Ui::default();
        let stepped = ui.number("qty", "Quantity").steppers().render();
        let m = html(ui.form("/f").text("a", "A").body(stepped));
        let first = first_submit(&m);
        assert!(
            first.contains(r#"class="lui-sr""#) && !first.contains("name="),
            "{m}"
        );
        let plain = html(ui.form("/f").text("a", "A"));
        assert!(!plain.contains(r#"class="lui-sr""#), "{plain}");
    }

    fn html(f: Form) -> String {
        f.render().into_string()
    }

    #[test]
    fn checkbox_and_hidden_fields() {
        let ui = Ui::default();
        let fs = || {
            ui.fields()
                .hidden("tab", "1")
                .checkbox("notify", "Email me")
        };
        let m = html(fs().checked(true));
        assert!(
            m.starts_with(r#"<input type="hidden" name="tab" value="1">"#),
            "{m}"
        );
        assert!(
            m.contains(r#"type="checkbox" value="true" checked"#)
                && m.contains(" Email me</label>"),
            "{m}"
        );
        assert!(!html(fs()).contains("checked"));
    }

    #[test]
    fn values_and_errors_fill_fields_by_name() {
        let ui = Ui::default();
        let values = [
            ("name".to_string(), "Ada".to_string()),
            ("email".to_string(), "posted@x.org".to_string()),
            ("bio".to_string(), "Hi".to_string()),
        ];
        let errors = [("name", "Too short."), ("bio", "Posted message.")];
        let fs = |f: Form<'static>| {
            f.text("name", "Name")
                .email("email", "Email")
                .value("own@x.org")
                .textarea("bio", "Bio", 2)
                .error("Own message.")
        };
        let m = html(fs(ui.form("/p")).values(&values).errors(&errors));
        assert!(m.contains(r#"name="name" type="text" value="Ada""#), "{m}");
        assert!(
            m.contains(r#"value="own@x.org""#) && !m.contains("posted@x.org"),
            "a field's own value wins"
        );
        assert!(m.contains(">Too short.</p>") && m.contains(r#"aria-invalid="true""#));
        assert!(
            m.contains("Own message.") && !m.contains("Posted message."),
            "a field's own error wins"
        );
        assert!(m.contains(">Hi</textarea>"));
        let bare = html(fs(ui.fields()).values(&values));
        assert!(!bare.contains("<form") && bare.contains(r#"value="Ada""#));
    }

    #[test]
    fn readonly_keeps_its_own_value_and_groups_read_their_pairs() {
        let ui = Ui::default();
        let values = [
            ("plan".to_string(), "posted".to_string()),
            ("size".to_string(), "l".to_string()),
            ("days[]".to_string(), "mon".to_string()),
            ("days[]".to_string(), "fri".to_string()),
        ];
        let m = html(
            (ui.form("/p"))
                .text("plan", "Plan")
                .value("free")
                .readonly()
                .radio("size", "Size", [("m", "M"), ("l", "L")])
                .checkboxes(
                    "days",
                    "Days",
                    [("mon", "Mon"), ("tue", "Tue"), ("fri", "Fri")],
                )
                .values(&values),
        );
        assert!(
            m.contains(r#"value="free""#) && !m.contains("posted"),
            "{m}"
        );
        assert!(m.contains(r#"value="l" checked"#), "{m}");
        assert!(
            m.contains(r#"value="mon" checked"#)
                && m.contains(r#"value="fri" checked"#)
                && !m.contains(r#"value="tue" checked"#),
            "{m}"
        );
        crate::input::assert_described_ids_exist(&m);
    }

    #[test]
    fn secondary_action_and_method_override() {
        let ui = Ui::default();
        let m = html(
            (ui.form("/notes/1"))
                .text("t", "T")
                .disabled()
                .autocomplete("off")
                .action_secondary("Cancel", "/notes")
                .method_override("delete"),
        );
        assert!(
            m.contains(r#"<input type="hidden" name="_method" value="delete">"#),
            "{m}"
        );
        assert!(m.contains(r#"href="/notes">Cancel</a>"#), "{m}");
        assert!(
            m.contains(" disabled") && m.contains(r#"aria-disabled="true""#),
            "{m}"
        );
        assert!(m.contains(r#"autocomplete="off""#), "{m}");
    }

    #[test]
    #[should_panic(expected = "Form::required() applies to a field, but the last item was Group")]
    fn a_field_modifier_after_a_group_is_loud() {
        let _ = Ui::default().form("/f").group("G").required();
    }

    #[test]
    #[should_panic(
        expected = "Form::description() applies to a group, but the last item was Field"
    )]
    fn a_description_after_a_field_is_loud() {
        let _ = Ui::default().form("/f").text("n", "N").description("D");
    }

    #[test]
    #[should_panic(expected = "Form::help() applies to a field, but the last item was Nothing")]
    fn a_modifier_before_any_item_is_loud() {
        let _ = Ui::default().form("/f").help("H");
    }

    #[test]
    fn help_counter_and_error_describe_the_field() {
        let f = Ui::default()
            .form("/p")
            .textarea("bio", "Bio", 2)
            .value("héllo")
            .maxlength(10)
            .help("Short.")
            .error("Too dull.");
        let m = html(f);
        assert!(
            m.contains("aria-describedby=\"f-bio-help f-bio-count f-bio-error\""),
            "{m}"
        );
        assert!(
            m.contains(
                "<output id=\"f-bio-count\" for=\"f-bio\" class=\"lui-field-count\">5 / 10</output>"
            ),
            "counts chars, not bytes"
        );
        assert!(m.contains("aria-invalid=\"true\"") && m.contains(">héllo</textarea>"));
        assert!(!m.contains("<fieldset") && !m.contains("enctype"));
    }

    #[test]
    fn kinds_map_to_attributes() {
        let f = Ui::default()
            .form("/p")
            .group("G")
            .date("d", "D", "2026-01-01", "")
            .time("t", "T", "09:00", "17:00")
            .file("f", "F", "")
            .multiple()
            .value("ignored")
            .pattern("h", "H", "[a-z]+", "Lowercase.")
            .text("p", "P")
            .placeholder("Type")
            .inline();
        let m = html(f);
        assert!(
            m.contains("type=\"date\" value=\"\" min=\"2026-01-01\">"),
            "an empty bound is left out: {m}"
        );
        assert!(m.contains("type=\"time\" value=\"\" min=\"09:00\" max=\"17:00\""));
        assert!(
            m.contains("type=\"file\" multiple")
                && !m.contains("ignored")
                && !m.contains("accept=")
        );
        assert!(
            m.contains("pattern=\"[a-z]+\" title=\"Lowercase.\"") && m.contains("id=\"f-h-help\"")
        );
        assert!(m.contains("placeholder=\"Type\""));
        assert!(
            m.contains("class=\"lui-form lui-form-inline\"")
                && m.contains(r#"<legend class="lui-form-group-legend">G</legend>"#)
        );
    }
}
