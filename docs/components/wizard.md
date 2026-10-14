# Wizard

`loco_ui::wizard` · demo: `/wizard`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Wizard` | `ui.wizard(id: &str, action: &str)` | stable |

## Props of `Wizard`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `step` | item | `title: &'a str, body: impl Into<StepBody<'a>>` |  |  | A step titled `title` showing `body`. |
| `review` | item | `title: &'a str` |  |  | The last step. |
| `optional` | modifier |  |  |  | The step added last can be skipped. |
| `values` | value | `values: &'a [(String, String)]` |  |  | What the visitor entered so far, by field name, for every step's fields and the review. |
| `at` | number | `n: usize` |  |  | Show step `n` whatever the request says. |
| `errors` | value | `errors: &'a [(&'a str, &'a str)]` |  |  | Server messages `(field name, message)` for the posted step. |
| `finish` | value | `label: &'a str` | `Finish` |  | Label of the last step's submit button. |
| `hide_progress` | switch |  | `off` |  | No `<progress>` element (the connectors still show progress on screen). |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<form method="post">` | 1 | 1 | 1 |
| `aria-current="step"` | 1 | 1 | 1 |
| `<fieldset>` | 1 | 1 | 1 |
| `formnovalidate` | 4 | 4 | 5 |
| `<progress>` | 8 | 16 | 6 |

- Fallback: none needed: one form per step, PRG between them
- Needs script: no
- Responsive by: container
- Look after: Origin UI Stepper

## Documentation

A form split into steps, no script: each step is one `<form method="post">`, the server
checks it and either re-renders it with messages beside the fields or stores it and
redirects to the next step. Optional steps can be skipped, a progress bar shows how far
along the visitor is, the last step is a review whose every value links back to its step,
and a visitor who closes the tab comes back to the step they left.

The look follows the Origin UI Stepper: the progress is a `Steps` list,
numbered dots (a check when done) with the title and "optional" beside them, joined by
connectors that are the progress on screen (primary after a done step). A column in a
narrow container, a row from 30rem.

**Platform features:**
- Post/Redirect/Get per step: a redirect after each valid POST, so refresh never
  re-submits. An invalid POST answers with the same step, the values kept and the
  messages beside the fields (`Wizard::errors`), and the step list shows it in error.
- The current step is a `step.<id>` key in `crate::UiState`: it travels in `?step.<id>=n`
  and the `lui-ui` cookie, like a tab, so the URL of a step can be shared, Back/Forward in
  the browser work, and a bare visit resumes where the cookie says
  (`crate::UiState::remembered`) with a "Start over" link.
- `<ol>` step list with `aria-current="step"` on the current one; done steps are links.
- `<fieldset>` + `<legend>` for the step's fields, `<button name="skip" formnovalidate>`
  (baseline 2015) to skip an optional step without the browser checking its fields.
- `<progress>` (baseline 2015) for the steps done out of the total, read by assistive tech
  (visually hidden: the connectors show it); a container query (Chrome 105, Firefox 110,
  Safari 16) turns the step column into a row.
- `Wizard::review` renders the review as a `<dl>` with an "Edit" link per value.

**Accessibility:** an ordered list of steps with `aria-current="step"`, a labelled
`<progress>`, one `<fieldset>` per step with the step in its `<legend>`, `aria-invalid` on a
step with errors. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** keep the step inputs when the browser drops the form
on back; state travels in hidden fields and the query.

**Fallback:** none needed. Everything is a link or a form.

**Finding:** the entered values are the app's data, not UI state, so they do not belong
in the URL. The demo keeps them in one cookie; a real app would use its session store.

```rust
use loco_ui::prelude::*;
// The request is on step 2 (0-based): the review.
let ui = Ui::from_request("/wizard", "step.signup=2", "");
let m = ui.wizard("signup", "/wizard")
    .step("Account", ui.fields().email("email", "Email").required().value("a@b.c"))
    .step("Newsletter", ui.fields().text("topics", "Topics")).optional()
    // Every field of the steps before, each with a link back to its step.
    .review("Review")
    .finish("Create account");
let html = m.render().into_string();
assert!(html.contains("aria-current=\"step\""));
assert!(html.contains("<progress class=\"lui-wizard-progress\" value=\"2\" max=\"2\""));
assert!(html.contains("href=\"/wizard?step.signup=0\" aria-label=\"Edit Email\""));
assert!(html.contains("(skipped)"), "an empty value");
// The same in `lui!`:
let same = lui! { Wizard("signup", "/wizard") {
    step "Account" (ui.fields().email("email", "Email").required().value("a@b.c"));
    step "Newsletter" (ui.fields().text("topics", "Topics")) optional;
    review "Review";
    finish "Create account";
} };
assert_eq!(same.into_string(), html);
```
