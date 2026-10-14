# Validated form

`loco_ui::form` · demo: `/app/signin`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Form` | `ui.form(action: &str)` | stable |
| `Form` | `ui.fields()` | stable |

## Props of `Form`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `group` | item | `legend: &'a str` |  |  | A `<fieldset>` with this `<legend>` around the fields added after it. |
| `description` | value | `text: &'a str` |  |  | A line under the legend of the group added last. |
| `text` | item | `name: &'a str, label: &'a str` |  |  | Single-line text. |
| `password` | item | `name: &'a str, label: &'a str` |  |  | `type="password"`. |
| `email` | item | `name: &'a str, label: &'a str` |  |  | `type="email"`. |
| `number` | item | `name: &'a str, label: &'a str, min: i64, max: i64` |  |  | A whole number from `min` to `max`, inclusive. |
| `pattern` | item | `name: &'a str, label: &'a str, pattern: &'a str, hint: &'a str` |  |  | Text that must match `pattern` (the HTML `pattern` attribute). |
| `textarea` | item | `name: &'a str, label: &'a str, rows: u8` |  |  | Multi-line text, `rows` high. |
| `file` | item | `name: &'a str, label: &'a str, accept: &'a str` |  |  | A file picker. |
| `date` | item | `name: &'a str, label: &'a str, min: &'a str, max: &'a str` |  |  | `type="date"`, bounds as `YYYY-MM-DD`. |
| `time` | item | `name: &'a str, label: &'a str, min: &'a str, max: &'a str` |  |  | `type="time"`, bounds as `HH:MM`. |
| `datetime` | item | `name: &'a str, label: &'a str, min: &'a str, max: &'a str` |  |  | `type="datetime-local"`, bounds as `YYYY-MM-DDTHH:MM`. |
| `select` | item | `name: &'a str, label: &'a str, options: impl IntoIterator<Item = O>` |  |  | A `<select>` of `options`: `&str`s, or `(value, label)` pairs. |
| `checkbox` | item | `name: &'a str, label: &'a str` |  |  | A checkbox posting `true` when ticked and nothing when not (so a `bool` with `#[serde(default)]` reads it). |
| `switch` | item | `name: &'a str, label: &'a str` |  |  | A checkbox drawn as an on/off switch, posting like `.checkbox(..)`. |
| `hidden` | item | `name: &'a str, value: &'a str` |  |  | `type="hidden"`. |
| `body` | item | `markup: Markup` |  |  | Any markup at this point among the fields: a control built with its own setters. |
| `required` | modifier |  |  |  | The `required` attribute, and a `*` after the label; the form says what the star means. |
| `help` | modifier | `help: &'a str` |  |  | Help text under the field. |
| `maxlength` | modifier | `max: usize` |  |  | `maxlength`, counted in an `<output>` under the field. |
| `value` | modifier | `value: &'a str` |  | `value` | The field's current value (ignored for files). |
| `checked` | modifier | `checked: bool` |  |  | Tick the checkbox. |
| `error` | modifier | `message: &'a str` |  |  | A server message beside the field. |
| `placeholder` | modifier | `placeholder: &'a str` |  |  | Placeholder text. |
| `multiple` | modifier |  |  |  | The file picker takes several files. |
| `submit` | value | `label: &'a str` | `Submit` |  | Label of the submit button. |
| `inline` | switch |  | `off` |  | Labels beside the fields on screens wider than 40rem, above them on narrower ones. |
| `get` | switch |  | `off` |  | `method="get"`: the fields go in the URL of the page it answers with (a filter, a search). |
| `values` | value | `values: &'a [(String, String)]` |  |  | Submitted values by field name, as a form post parses them. |
| `errors` | value | `errors: &'a [(&'a str, &'a str)]` |  |  | Server messages `(field name, message)`. |
| `id` | value | `id: &'a str` |  | `id` | What the swap root's id is built from, when two forms on a page post to the same action (one per tab, say). |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `required` | 4 | 4 | 5 |
| `pattern` | 4 | 4 | 5 |
| `:user-invalid` | 119 | 88 | 16.5 |
| `<fieldset>` | 1 | 1 | 1 |
| `<output>` | 10 | 4 | 7 |
| `type=date` | 20 | 57 | 14.1 |
| `accept` | 1 | 1 | 1 |
| `field-sizing` | 123 | no | no |

- Fallback: server re-renders with messages; no early styling; textareas keep their rows; the counter shows the submitted length
- Needs script: no
- Responsive by: container
- Look after: shadcn Forms, Radix Themes

## Documentation

A validated form: the browser blocks bad input before submit, the server checks again and
re-renders with messages. Fields come in groups with legends, carry help text, count their
characters, take files, dates, times and bounded numbers, and lay out stacked (label above)
or inline (label beside). No script needed.

The look follows the shadcn Forms examples and the Radix Themes settings layouts: a group
is a section (a heading, an optional `.description(..)` and its fields) drawn without a
frame, a hairline between sections. The form is its own container: `.inline()` puts the
label and its help beside the field once the form is 30rem wide, and the submit button
fills a narrower form and sits at the end of a wider one.

**Platform features:**
- Constraint validation attributes `required`, `pattern`, `min`, `max`, `maxlength`,
  `type=email` (baseline 2015); `type=date` and `type=time` with `min`/`max` (Chrome 20,
  Firefox 57, Safari 14.1).
- `:user-invalid` / `:user-valid` (baseline 2023): styles only after the user has interacted,
  so fields are not red on first paint.
- `<fieldset>` + `<legend>` per `Form::group`, described by its `.description(..)`; help
  text and the error are tied to the field with `aria-describedby`.
- Container queries (Chrome 105, Firefox 110, Safari 16) for the side labels and the
  submit row.
- `<output>` counts characters for a field with a `maxlength`: the server renders the count
  of the value it has, the enhancement script keeps it live while typing.
- `<input type=file accept>` (baseline 2015); any file field makes the form
  `enctype="multipart/form-data"`.
- `field-sizing` (Chrome 123, not yet in Firefox or Safari): a textarea grows with its
  content.
- Post/Redirect/Get for success; on error the server re-renders the form with values and
  messages, and an error summary at the top that links to each
  field in error and takes the focus.

**Accessibility:** labelled fields grouped in `<fieldset>`/`<legend>`, native constraint
validation, and an error summary that takes the focus when the server sends the form back.
Checked by axe-core in headless Firefox on every demo route, both capability variants, light
and dark (no serious or critical violation).

**What it does not do without script:** validate against the server as you type, or warn
about unsaved changes on leaving the page.

**Fallback:** without container queries the labels stay above the fields and the button
fills the form's width. Without `field-sizing` a textarea keeps its `rows` and can be resized by
hand. Without the script the counter shows the length of the last submitted value and
`maxlength` still stops input at the limit.

**Enhanced:** the form is a swap root, so with the `crate::enhance` script a submit
replaces only the form (errors included) and a successful redirect swaps in the result.

**Finding:** custom cross-field rules (password confirmation, async uniqueness) only run
on the server round trip, and warning about unsaved changes when leaving needs script.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let m = ui.form("/signup").email("email", "Email").required();
assert!(m.render().into_string().contains(r#"type="email" value="" required"#));

// `required`, `help`, `maxlength`, `value` and friends apply to the field added last;
// `group` starts a fieldset for the fields after it.
let m = ui.form("/profile")
    .group("About you").description("Shown on your public page.")
    .textarea("bio", "Bio", 3).maxlength(280).value("Hi").help("Shown on your profile.")
    .file("avatar", "Avatar", "image/png,image/jpeg")
    .date("born", "Born", "1900-01-01", "2026-12-31")
    .select("digest", "Digest", ["daily", "weekly", "never"]).value("weekly")
    .submit("Save profile")
    .inline();
let html = m.render().into_string();
assert!(html.contains("enctype=\"multipart/form-data\""));
assert!(html.contains(r#"<legend class="lui-form-group-legend">About you</legend>"#));
assert!(html.contains(r#"aria-describedby="lui-form-about-you-description""#));
assert!(html.contains(">2 / 280</output>"));
assert!(html.contains("accept=\"image/png,image/jpeg\""));
assert!(html.contains(r#"<option value="weekly" selected>"#));

// The same in `lui!`:
let same = lui! { Form("/profile") submit="Save profile" inline {
    group "About you" description="Shown on your public page.";
    textarea "bio" "Bio" 3 maxlength=280 value="Hi" help="Shown on your profile.";
    file "avatar" "Avatar" "image/png,image/jpeg";
    date "born" "Born" "1900-01-01" "2026-12-31";
    select "digest" "Digest" (["daily", "weekly", "never"]) value="weekly";
} };
assert_eq!(same.into_string(), m.render().into_string());
```

**Any control.** `.switch(..)` adds a switch like a checkbox, and `.body(..)` puts any
markup at that point among the fields (a date picker, a range, a toggle group, a select
built with its own setters), so it sits inside the form's swap root, under its error
summary and before its button. `.get()` makes it a GET form: a filter or a search whose
answer is a page with the choices in its URL.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/filters", "size=l", "");
let m = ui.form("/filters")
    .get()
    .text("q", "Search")
    .switch("open", "Open only").checked(true)
    .body(html! { (ui.select("size", "Size").options([("s", "Small"), ("l", "Large")])) })
    .submit("Apply");
let html = m.render().into_string();
assert!(html.contains(r#"method="get" action="/filters""#));
assert!(html.contains(r#"role="switch""#) && html.contains(r#"<option value="l" selected>"#));
assert!(html.find("f-size") < html.find("lui-form-actions"), "the markup before the button");
// The same in `lui!`: a block of markup among the items is `body { .. }`, and a block with
// no items at all is the body.
let same = lui! { Form("/filters") get submit="Apply" {
    text "q" "Search";
    switch "open" "Open only" checked=(true);
    body { Select("size", "Size") options=([("s", "Small"), ("l", "Large")]); }
} };
assert_eq!(same.into_string(), html);
```
