# Input, checkbox, switch, radio group

`loco_ui::input` · demo: `/field`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Input` | `ui.input(name: &str, label: &str)` | stable |
| `Input` | `ui.checkbox(name: &str, label: &str)` | stable |
| `Input` | `ui.switch(name: &str, label: &str)` | stable |
| `RadioGroup` | `ui.radio_group(name: &str, legend: &str)` | stable |

## Props of `Input`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `email` | switch |  | `off` | `type` | `type="email"`. |
| `password` | switch |  | `off` | `type` | `type="password"`. |
| `number` | value | `min: i64, max: i64` |  |  | A whole number from `min` to `max`, inclusive. |
| `pattern` | value | `pattern: &'a str, hint: &'a str` |  | `pattern` | Text that must match `pattern`. |
| `textarea` | number | `rows: u8` |  |  | Multi-line text, `rows` high. |
| `file` | value | `accept: &'a str` |  |  | A file picker. |
| `date` | value | `min: &'a str, max: &'a str` |  |  | `type="date"`, bounds as `YYYY-MM-DD`. |
| `time` | value | `min: &'a str, max: &'a str` |  |  | `type="time"`, bounds as `HH:MM`. |
| `datetime` | value | `min: &'a str, max: &'a str` |  |  | `type="datetime-local"`, bounds as `YYYY-MM-DDTHH:MM`. |
| `multiple` | switch |  | `off` | `multiple` | The file picker takes several files. |
| `required` | switch |  | `off` | `required` | The `required` attribute, and a `*` after the label (hidden from screen readers, which say "required"). |
| `help` | value | `help: &'a str` |  |  | Help text under the field. |
| `maxlength` | number | `max: usize` |  | `maxlength` | `maxlength`, counted in an `<output>` under the field. |
| `value` | value | `value: &'a str` |  | `value` | The current value (ignored for files and passwords). |
| `checked` | condition | `checked: bool` | `off` | `checked` | Tick the checkbox or switch. |
| `error` | value | `message: &'a str` |  |  | A server message under the field. |
| `placeholder` | value | `placeholder: &'a str` |  | `placeholder` | Placeholder text. |
| `search` | switch |  | `off` | `type` | `type="search"`. |
| `hide_label` | switch |  | `off` |  | Only the control, the label kept as its `aria-label`. |
| `list` | value | `id: &'a str` |  | `list` | `list`. |
| `autocomplete` | value | `value: &'a str` |  | `autocomplete` | `autocomplete` (`"off"`, `"email"`, `"new-password"`). |
| `autofocus` | switch |  | `off` | `autofocus` | `autofocus`. |
| `inputmode` | value | `mode: &'a str` |  | `inputmode` | `inputmode` (`"numeric"`), the on-screen keyboard to show. |
| `step` | number | `step: i64` |  | `step` | `step` of a number field. |
| `aria_controls` | value | `id: &'a str` |  | `aria-controls` | `aria-controls`. |
| `form` | value | `id: &'a str` |  | `form` | `form`. |
| `class` | value | `class: &'a str` |  | `class` | A class on the control, for a component's part name. |
| `id` | value | `id: &'a str` |  | `id` | The control's id, `f-<name>` by default. |
| `gradient_border` | switch |  | `off` |  | The border drawn with `--lui-gradient-ring`. |
| `leading` | value | `slot: impl Into<Glyph<'a>>` |  |  | An icon or text inside the field's border, before the text. |
| `trailing` | value | `slot: impl Into<Glyph<'a>>` |  |  | An icon or text inside the field's border, after the text. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | A text field's height and text: Radix's sizes 1 to 3. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `surface` |  | A text field's look: surface (a border), soft (a tint) or ghost (neither). |

## Props of `RadioGroup`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `option` | item | `value: &'a str, label: &'a str` |  |  | One choice posting `value`, labelled `label`. |
| `description` | value | `text: &'a str` |  |  | A line under the choice added last, read out with it. |
| `cards` | switch |  | `off` |  | Each choice a selectable card, 1, 2 or 3 across by the container's width. |
| `value` | value | `value: &'a str` |  | `value` | The value of the choice that is selected. |
| `required` | switch |  | `off` | `required` | One choice must be picked before the form submits. |
| `help` | value | `help: &'a str` |  |  | Help text under the choices. |
| `error` | value | `message: &'a str` |  |  | A server message under the choices. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `constraint validation` | 10 | 4 | 10.1 |
| `type=date` | 20 | 57 | 14.1 |
| `:user-invalid` | 119 | 88 | 16.5 |
| `role="switch"` | 1 | 1 | 1 |
| `appearance: none` | 84 | 80 | 15.4 |
| `<fieldset>` | 1 | 1 | 1 |
| `linear-gradient(in oklch` | 111 | 127 | 16.2 |

- Fallback: none needed: native controls; the switch stays a checkbox without appearance: none; a gradient border is a plain one without in oklch gradients
- Needs script: partly: a live character count while typing needs the enhancement script
- Responsive by: container
- Look after: Radix Themes TextField, Checkbox, Switch, RadioGroup and RadioCards

## Documentation

One labelled field on its own: the label, the control, help text, a character counter and
the server's error, tied together with `aria-describedby`. `ui.checkbox`, `ui.switch` and
`ui.radio_group` are the same for choices. `crate::form` renders its fields with this
file, so a field looks and behaves the same inside a form built with `ui.form` and in a
form a route writes by hand.

The look follows Radix Themes (TextField, TextArea, Checkbox, Switch, RadioGroup and
RadioCards): a gray-7 border with no shadow, `.leading(..)`/`.trailing(..)` slots inside
the border, help in gray-11 and the error after a warning icon, a drawn checkbox, radio and
switch, and `.cards()` for a radio group of selectable cards. A text field takes Radix's
sizes (`.size(..)`, 1 to 3) and its surface, soft or ghost variant (`.variant(..)`).

**Platform features:** constraint validation (`required`, `pattern`, `min`, `max`,
`maxlength`, `type=email`, baseline 2015; `type=date`/`time`/`datetime-local`, Chrome 20, Firefox 57 (93 for `datetime-local`),
Safari 14.1); `:user-invalid` (baseline 2023) so a field is not red before it is touched;
`<output for>` counts characters; `role="switch"` on a checkbox (ARIA 1.2) drawn as a
track and thumb with `appearance: none`; `<fieldset>` + `<legend>` for a radio group;
`appearance: none` checkboxes and radios with the tick drawn in CSS; `:has()` (Chrome 105,
Firefox 121, Safari 15.4) for the slot wrapper's focus ring and the picked card; a
container query (Chrome 105, Firefox 110, Safari 16) lays radio cards 1, 2 or 3 across.
`.gradient_border()` draws a text field's or textarea's border with `--lui-gradient-ring`, a
padding-box layer over a border-box one, behind `@supports` for
`linear-gradient(in oklch, ..)` (Chrome 111, Firefox 127, Safari 16.2); nothing moves.

**Accessibility:** every field has a `<label for>`; help, counter and error are joined by
`aria-describedby`, a server error sets `aria-invalid` and is `role="alert"`; the switch is
a checkbox with `role="switch"`. Checked by axe-core in headless Firefox on every demo
route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** keep the counter live while typing (the
enhancement script does; without it the counter shows the length the server rendered).

**Fallback:** none needed: every control is a native one, and the switch is still a
checkbox where `appearance: none` is not supported. Without `:has()` a slotted field keeps
the input's own focus outline and a picked card shows no ring; without container queries
radio cards stay one per row. A `.gradient_border()` field keeps its
plain border where `in oklch` gradients are not supported.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let name = ui.input("name", "Name").render().into_string();
assert!(name.contains(r#"<label for="f-name">Name</label>"#) && name.contains(r#"type="text""#));
// The same in `lui!`:
let same = lui! { Input("name", "Name"); };
assert_eq!(same.into_string(), name);
// An opt-in gradient border, a class on the control.
let key = ui.input("key", "API key").gradient_border().render().into_string();
assert!(key.contains(r#"class="lui-input-gradient-border""#));
assert_eq!(lui! { Input("key", "API key") gradient_border; }.into_string(), key);
// Setters as on form fields, and one per input type.
let email = ui.input("email", "Email").email().required().value("ada@x.org")
    .help("We never share it.").error("Already taken.");
let email = email.render().into_string();
assert!(email.contains(r#"aria-describedby="f-email-help f-email-error""#));
assert!(email.contains(r#"aria-invalid="true""#) && email.contains(r#"type="email""#));
let on = ui.switch("digest", "Weekly digest").checked(true).render().into_string();
assert!(on.contains(r#"role="switch""#) && on.contains("checked"));
let plan = ui.radio_group("plan", "Plan").option("free", "Free").option("pro", "Pro").value("pro");
let plan = plan.render().into_string();
assert!(plan.contains("<legend>Plan</legend>") && plan.contains(r#"value="pro" checked"#));
// Text or an icon inside the border; text is read out with the field.
let site = ui.input("site", "Website").leading("https://").trailing(Icon::Search);
let site = site.render().into_string();
assert!(site.contains(r#"<div class="lui-input-slots"><span id="f-site-leading""#));
assert!(site.contains(r#"aria-describedby="f-site-leading""#));
assert_eq!(lui! { Input("site", "Website") leading="https://" trailing=(Icon::Search); }.into_string(), site);
// A radio group of cards, 1, 2 or 3 across by the container's width.
let tier = ui.radio_group("tier", "Tier").cards()
    .option("hobby", "Hobby").description("One project")
    .option("team", "Team").description("Unlimited projects").value("team");
let tier = tier.render().into_string();
assert!(tier.contains("lui-radio-cards") && tier.contains(r#"aria-describedby="f-tier-1-description""#));
let same = lui! { RadioGroup("tier", "Tier") cards value="team" {
    option "hobby" "Hobby" description="One project";
    option "team" "Team" description="Unlimited projects";
} };
assert_eq!(same.into_string(), tier);
// A small soft field, the size and variant on its root.
let q = ui.input("q", "Search").size(1).variant(Variant::Soft).render().into_string();
assert!(q.starts_with(r#"<div class="lui-field" data-size="1" data-variant="soft">"#));
assert_eq!(lui! { Input("q", "Search") size=1 variant="soft"; }.into_string(), q);
```
