# Date picker

`loco_ui::date_picker` · demo: `/calendar`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `DatePicker` | `ui.date_picker(name: &str, label: &str)` | stable |

## Props of `DatePicker`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `value` | value | `date: &'a str` |  | `value` | The current value, `YYYY-MM-DD` (a saved date). |
| `min` | value | `date: &'a str` |  | `min` | The first day that can be picked, `YYYY-MM-DD`. |
| `max` | value | `date: &'a str` |  | `max` | The last day that can be picked, `YYYY-MM-DD`. |
| `disabled_dates` | value | `off: fn(Date) -> bool` |  |  | Days for which `off` returns true cannot be picked (the native control ignores this). |
| `required` | switch |  | `off` | `required` | A day must be picked before the form submits. |
| `native` | switch |  | `off` |  | The browser's own `<input type="date">` instead of the calendar. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `popover` | 114 | 125 | 17 |
| `anchor-name` | 125 | 147 | 26 |
| `<input type="date">` | 20 | 57 | 14.1 |

- Fallback: without popover the calendar is laid out in the form; .native() is the browser's own control
- Needs script: partly: writing the picked day onto the button before the form is sent needs script
- Responsive by: content
- Look after: shadcn Date Picker

## Documentation

A form field for a date: a button showing the date, which opens a calendar in a popover;
picking a day checks a radio named after the field, so the form posts `name=YYYY-MM-DD`.
`.native()` gives the browser's own `<input type="date">` instead.

**Platform features:** `popover` with `popovertarget` (Chrome 114, Firefox 125, Safari 17);
CSS anchor positioning (`anchor-name`, `position-anchor`, Chrome 125, Firefox 147, Safari 26)
places it under the button; inside, the `crate::calendar` in radio mode. `.native()` is
`<input type="date">` with `min`/`max` (Chrome 20, Firefox 57, Safari 14.1).

**Accessibility:** a button naming the picked date that opens the calendar popover, the days
a radio group labelled by the field's legend; Escape closes the popover. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** write the picked day onto the button before the form
is sent (the button shows the saved value; the picked day is filled in the calendar), or
change month without a page load. A month link comes back with the calendar laid out in the
page instead of in the popover, because a popover cannot arrive open.

**Fallback:** without `popover` the calendar is laid out in the form under the label.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let m = ui.date_picker("due", "Due date").value("2026-09-24").render().into_string();
assert!(m.contains(r#"popovertarget="f-due-calendar""#) && m.contains("24 September 2026"));
assert!(m.contains(r#"type="radio" name="due" value="2026-09-24" checked"#));
// No popover support: the same calendar, in the page.
let old = Ui::from(Caps::default());
assert!(!old.date_picker("due", "Due date").render().into_string().contains("popover"));
// The browser's own control, with bounds.
let m = ui.date_picker("due", "Due date").min("2026-01-01").max("2026-12-31").native();
assert!(m.render().into_string().contains(r#"type="date" value="" min="2026-01-01" max="2026-12-31""#));
// The same in `lui!`:
let same = lui! { DatePicker("due", "Due date") min="2026-01-01" max="2026-12-31" native; };
assert_eq!(same.into_string(), m.render().into_string());
```
