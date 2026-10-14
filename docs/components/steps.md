# Steps

`loco_ui::steps` · demo: `/steps`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Steps` | `ui.steps()` | beta |

## Props of `Steps`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `step` | item | `title: &'a str` |  |  | A step titled `title`. |
| `done` | modifier |  |  |  | The step added last is done: a check in its dot, the connector after it filled. |
| `current` | modifier |  |  |  | The step added last is the one the visitor is on (`aria-current="step"`). |
| `invalid` | modifier |  |  |  | The step added last needs attention: a "!" in a danger dot. |
| `optional` | modifier |  |  |  | The step added last can be skipped: "Optional" beside its title. |
| `href` | modifier | `href: &'a str` |  | `href` | The title of the step added last links here. |
| `description` | modifier | `text: &'a str` |  |  | A muted line under the title of the step added last. |
| `vertical` | switch |  | `off` |  | A column at every width instead of a row from 30rem. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<ol>` | 1 | 1 | 1 |
| `aria-current="step"` | 1 | 1 | 1 |
| `@container` | 105 | 110 | 16 |

- Fallback: none needed: a list of links
- Needs script: no
- Responsive by: container
- Look after: Origin UI Stepper

## Documentation

Where a visitor is in a process of several steps: numbered dots joined by connectors,
each step done, current or still to come, a done step a link back to it. A row of dots
with the titles under them from 30rem, a column below that (or always, with
`.vertical()`). The wizard shows its progress with it. After the Origin
UI Stepper.

**Platform features:** an `<ol>` (baseline since HTML 2) whose current step carries
`aria-current="step"`; the connectors are `::after` boxes; a container query
(`@container`, Chrome 105, Firefox 110, Safari 16) on the list turns the column into a
row.

**Accessibility:** an ordered list, so a screen reader announces "step 2 of 4"; the
current step has `aria-current="step"`; a done step is followed by a hidden "completed",
a step in error by "has errors"; the dots are `aria-hidden`. Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing; it is a list of links.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.steps().step("Cart").done().step("Address").current().step("Pay").render().into_string();
assert!(m.contains(r#"<li class="lui-steps-current" aria-current="step">"#));
// A done step that links back, an optional one with a line under it, a column.
let m = ui.steps()
    .step("Account").done().href("/signup?step=0")
    .step("Profile").current().optional().description("A photo and a bio.")
    .step("Review")
    .vertical();
let html = m.render().into_string();
assert!(html.contains(r#"<a href="/signup?step=0">Account</a>"#) && html.contains("lui-steps-vertical"));
// The same in `lui!`:
let same = lui! { Steps vertical {
    step "Account" done href="/signup?step=0";
    step "Profile" current optional description="A photo and a bio.";
    step "Review";
} };
assert_eq!(same.into_string(), html);
```
