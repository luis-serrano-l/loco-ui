# Error summary

`loco_ui::error_summary`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `ErrorSummary` | `ui.error_summary(errors: &[(&str, &str)])` | beta |

## Props of `ErrorSummary`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `title` | value | `text: &'a str` | `There is a problem` |  | The heading. |
| `field` | item | `name: &'a str, label: &'a str` |  |  | The label of the field `name`, written before its message as a form's summary does. |

## Keyboard of `ErrorSummary`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | Scrolls to the field a message names, so the next Tab lands in it. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-error-summary-title` | `--lui-text-2`, `--lui-ring` |
| `.lui-error-summary-list` | `--lui-space-1` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `role="alert"` | 1 | 1 | 1 |
| `aria-labelledby` | 1 | 1 | 1 |
| `autofocus` | 79 | 110 | 15.4 |

- Fallback: where autofocus only works on form controls the summary is still first in the form and read out as an alert
- Needs script: partly: moving the focus into the field a link points to needs script
- Responsive by: content
- Look after: Radix Themes Callout

## Documentation

The list of what went wrong, at the top of a form the server sent back (GOV.UK's error
summary): a heading, then one link per field in error that jumps to the field. The page
opens with the focus on it, so a keyboard or screen reader user starts at the problems
instead of hunting for them. `Form` shows one by itself whenever it
has messages; `ui.error_summary(..)` is for a form built from loose inputs.

**Platform features:**
- `role="alert"` (read out when the page loads) with `aria-labelledby` naming the heading.
- `autofocus` on the heading's link (global attribute on any focusable element: Chrome
  79, Firefox 110, Safari 15.4): the focus starts in the summary.
- Fragment links (`href="#f-email"`) to each field's `id`.

**Accessibility:** `role="alert"` named by its heading; the heading link has `autofocus`,
each item links to its field by id. Checked by axe-core in headless Firefox on every demo
route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** move the focus into the field a link points to;
the page scrolls to it, and Tab goes on from there.

**Fallback:** where `autofocus` only works on form controls, the summary is still the
first thing in the form and is read out as an alert.

**Enhanced:** when the enhancement script swaps a form in place, it focuses the
`autofocus` element of the new markup too, as a page load would.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let errors = [("email", "Enter an email address."), ("password", "At least 8 characters.")];
let m = ui.error_summary(&errors).render().into_string();
assert!(m.contains(r##"<a href="#f-email" autofocus>There is a problem</a>"##));
assert!(m.contains(r##"<li><a href="#f-password">At least 8 characters.</a></li>"##));
assert!(ui.error_summary(&[]).render().into_string().is_empty());

// With the fields' labels, each message reads as a form's summary writes it.
let m = ui.error_summary(&errors).field("email", "Email").title("Check your details");
let m = m.render().into_string();
assert!(m.contains(r#"role="alert""#) && m.contains(">Check your details</a>"));
assert!(m.contains(r##"<a href="#f-email">Email: Enter an email address.</a>"##));
// The same in `lui!`:
let same = lui! { ErrorSummary(&errors) title="Check your details" { field "email" "Email"; } };
assert_eq!(same.into_string(), m);
```
