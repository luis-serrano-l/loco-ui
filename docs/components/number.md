# Number

`loco_ui::number` · demo: `/number`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Number` | `ui.number(name: &str, label: &str)` | beta |

## Props of `Number`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `min` | number | `min: impl Into<f64>` |  | `min` | Lowest value. |
| `max` | number | `max: impl Into<f64>` |  | `max` | Highest value. |
| `step` | number | `step: impl Into<f64>` | `1` | `step` | How far one step moves; the browser accepts only values on a step. |
| `value` | number | `value: impl Into<f64>` |  | `value` | The value, instead of the query's `name`. |
| `prefix` | value | `addon: impl Into<Glyph<'a>>` |  |  | An addon joined to the field's start (`"$"`). |
| `suffix` | value | `addon: impl Into<Glyph<'a>>` |  |  | An addon joined to the field's end (`"kg"`). |
| `steppers` | switch |  | `off` |  | Buttons stepping the value down and up: a post without script, `stepUp()` with it. |
| `required` | switch |  | `off` | `required` | The `required` attribute, and a `*` after the label. |
| `disabled` | switch |  | `off` | `disabled` | `disabled` and `aria-disabled`: shown greyed and not posted. |
| `readonly` | switch |  | `off` |  | Shown and posted, not editable; the steppers are off. |
| `hide_label` | switch |  | `off` |  | Only the control, the label kept as its `aria-label`. |
| `help` | value | `help: &'a str` |  |  | Help text under the field. |
| `error` | value | `message: &'a str` |  |  | A server message under the field. |
| `placeholder` | value | `placeholder: &'a str` |  | `placeholder` | Placeholder text. |
| `id` | value | `id: &'a str` |  | `id` | The control's id, `f-<name>` by default. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `type="number"` | 6 | 29 | 5.1 |
| `formnovalidate` | 4 | 4 | 5 |
| `:has()` | 105 | 121 | 15.4 |

- Fallback: none needed: a native number field; a stepper posts the stepped value
- Needs script: partly: stepping in place without a round trip needs the enhancement script
- Responsive by: content
- Look after: Origin UI number inputs

## Documentation

A number field with its unit beside it and buttons that step it: a quantity, a price, a
weight. The value is an `f64` written the way Rust writes it, so `2.0` shows as `2` and
`0.1 + 0.2` as `0.3`.

After the Origin UI number inputs ("with chevrons" and "with add-ons"): one bordered row
holding the field, a gray `.prefix(..)`/`.suffix(..)` addon at either end, and
`.steppers()` as two stacked chevron buttons at the end.

**Platform features:** `<input type="number">` with `min`, `max` and `step` (baseline
2015), so the browser refuses a value off the bounds or the step and its arrow keys step
it; the steppers are `<button name value formnovalidate>` (baseline 2015) posting the
value one step down or up; `:has()` (Chrome 105, Firefox 121, Safari 15.4) draws the row's
focus ring.

**Accessibility:** a labelled number field; a text addon is read out with it
(`aria-describedby`), an icon is decoration; the steppers are named "increment" and
"decrement" and disabled at the bounds. Checked by axe-core in headless Firefox on every
demo route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** step in place. Without the script a stepper
submits the form around it with the stepped value as the first `name` posted (the buttons
come before the field in the markup and show after it), which `crate::Posted::get` and
`Ui::param` read (a derived serde struct rejects the repeated name); the page answers
with the new value. With the enhancement script a
stepper calls `stepUp()`/`stepDown()` on the field and nothing is sent. A stepper is a
submit button, and Enter in any field presses a form's first submit button wherever the
field is: give a stepped number a form of its own, or put a submit button before it.

**Fallback:** none needed: a native number field. Without `:has()` the field keeps its
own focus outline inside the row.

```rust
use loco_ui::prelude::*;
// The value is the query's `qty`, or `.value(..)` (a saved one).
let ui = Ui::from_request("/cart", "qty=2", "");
let m = ui.number("qty", "Quantity").min(1).max(10).render().into_string();
assert!(m.contains(r#"name="qty" type="number" value="2" min="1" max="10""#));
// The same in `lui!`:
let same = lui! { Number("qty", "Quantity") min=1 max=10; };
assert_eq!(same.into_string(), m);
// A price with its currency and steppers that post 2.5 and 1.5.
let m = ui.number("price", "Price").value(2).step(0.5).min(0).prefix("$").steppers();
let html = m.render().into_string();
assert!(html.contains(r#"name="price" value="2.5""#) && html.contains(r#"name="price" value="1.5""#));
assert!(html.contains(r#"<span id="f-price-prefix" class="lui-input-group-prefix" dir="auto">$</span>"#));
let same = lui! { Number("price", "Price") value=2 step=0.5 min=0 prefix="$" steppers; };
assert_eq!(same.into_string(), html);
```
