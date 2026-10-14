# Checkbox group

`loco_ui::checkbox_group` · demo: `/checkbox-group`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `CheckboxGroup` | `ui.checkbox_group(name: &str, legend: &str)` | beta |

## Props of `CheckboxGroup`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `option` | item | `value: &'a str, label: &'a str` |  |  | One choice posting `value` under `<name>[]` when ticked, labelled `label`. |
| `description` | modifier | `text: &'a str` |  |  | A line under the choice added last, read out with it. |
| `disabled` | modifier |  |  | `disabled` | The choice added last cannot be ticked. |
| `cards` | switch |  | `off` |  | Each choice a card, 1, 2 or 3 across by the container's width. |
| `value` | value | `values: &'a [&'a str]` |  | `checked` | The ticked values, instead of the query's `<name>[]`. |
| `help` | value | `help: &'a str` |  |  | Help text under the choices. |
| `error` | value | `message: &'a str` |  |  | A server message under the choices. |
| `id` | value | `id: &'a str` |  | `id` | What the checkboxes' ids start from, `f-<name>` by default. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<fieldset>` | 1 | 1 | 1 |
| `appearance: none` | 84 | 80 | 15.4 |
| `:has()` | 105 | 121 | 15.4 |

- Fallback: none needed: native checkboxes; a ticked card shows no ring without :has()
- Needs script: partly: requiring at least one tick needs the server: HTML has no required for a group
- Responsive by: container
- Look after: Radix Themes CheckboxGroup and CheckboxCards

## Documentation

Several choices under one legend, any number of them ticked: the topics of a newsletter,
the days a shop opens. Each choice is a native checkbox posting `<name>[]`, so the form
sends one pair per ticked box and the server reads them with `Posted::all("<name>[]")`.
A form adds one with `.checkboxes(..)`.

After Radix Themes CheckboxGroup and CheckboxCards: a column of drawn checkboxes with the
label beside each and an optional line under it, or `.cards()` for a grid of bordered
cards that ring in the primary colour when ticked.

**Platform features:** `<fieldset>` + `<legend>` (Chrome 1, Firefox 1, Safari 3) around
`<input type="checkbox">`, drawn with `appearance: none` (Chrome 84, Firefox 80, Safari 15.4); `:has()`
(Chrome 105, Firefox 121, Safari 15.4) rings a ticked card; a container query (Chrome 105,
Firefox 110, Safari 16) lays cards 1, 2 or 3 across.

**Accessibility:** the legend names the group; each checkbox has its `<label>`, a choice's
description is joined by `aria-describedby`, help and the server's error describe the
fieldset, which gets `aria-invalid` with an error. Checked by axe-core in headless Firefox
on every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** require at least one tick (HTML has no
`required` for a group of checkboxes); the server checks it and sends an `.error(..)`.

**Fallback:** none needed: native checkboxes. Without `:has()` a ticked card shows its
tick but no ring; without container queries the cards stay one per row.

```rust
use loco_ui::prelude::*;
// The ticked values are the query's `topics[]`, or `.value(..)` (saved ones).
let ui = Ui::from_request("/news", "topics%5B%5D=rust", "");
let m = ui.checkbox_group("topics", "Topics").option("rust", "Rust").option("css", "CSS");
let html = m.render().into_string();
assert!(html.contains(r#"<legend>Topics</legend>"#));
assert!(html.contains(r#"type="checkbox" name="topics[]" value="rust" checked"#));
// The same in `lui!`:
let same = lui! { CheckboxGroup("topics", "Topics") { option "rust" "Rust"; option "css" "CSS"; } };
assert_eq!(same.into_string(), html);
// Cards, a description and a choice that cannot be ticked, with saved values.
let m = ui.checkbox_group("days", "Open on").cards()
    .option("sat", "Saturday").description("10:00 to 14:00")
    .option("sun", "Sunday").disabled()
    .value(&["sat"]).help("Pick the days you open.");
let html = m.render().into_string();
assert!(html.contains("lui-checkbox-cards") && html.contains(r#"value="sun" disabled"#));
assert!(html.contains(r#"aria-describedby="f-days-0-description""#));
let same = lui! { CheckboxGroup("days", "Open on") cards value=(&["sat"]) help="Pick the days you open." {
    option "sat" "Saturday" description="10:00 to 14:00";
    option "sun" "Sunday" disabled;
} };
assert_eq!(same.into_string(), html);
```
