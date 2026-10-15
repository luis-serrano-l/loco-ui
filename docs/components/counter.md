# Counter

`loco_ui::counter` · demo: `/counter`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Counter` | `ui.counter(action: &str, value: impl Into<f64>)` | stable |

## Props of `Counter`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `min` | number | `min: impl Into<f64>` |  | `min` | Lowest value. |
| `max` | number | `max: impl Into<f64>` |  | `max` | Highest value. |
| `step` | number | `step: impl Into<f64>` | `1` | `step` | How far one click moves (above 0). |
| `typed` | switch |  | `off` |  | A number field and a Set button posting `op=set&value=n`. |
| `disabled` | switch |  | `off` | `disabled` | Every button and the number field disabled. |
| `readonly` | switch |  | `off` |  | The value shown, its buttons off and the number field `readonly`. |

## Keyboard of `Counter`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the decrement, increment and reset buttons and the value field. | no |
| `Enter / Space` | Presses the focused button, which posts the change. | no |
| `ArrowUp / ArrowDown` | Steps the typed value up or down by the step before it is set. | no |
| `Enter` | Sets the typed value, as the Set button does. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-counter` | `--lui-space`, `--lui-text-6` |
| `.lui-counter-input` |  |
| `.lui-counter-bounds` | `--lui-muted`, `--lui-text-2` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<form method="post">` | 1 | 1 | 1 |
| `<button name value>` | 1 | 1 | 1 |
| `<input type="number">` | 6 | 29 | 5.1 |
| `cookie` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: -

## Documentation

The canonical "click me" demo done with a form round trip. State lives on the server
(here: a cookie), no script.

**Platform features:** `<form method="post">`, `<button name value>` so one form carries
several actions, Post/Redirect/Get, and `view-transition-name` (only when `Caps` says the
browser has view transitions) so the number morphs instead of flashing. With
`.min()`, `.max()`, `.step()` and `.typed()` the buttons are `disabled` at `min` and `max`, step by `step`, and the
value can be typed into an `<input type="number">` with `min`, `max`, `step` (baseline 2015), posted
with `op=set`; the browser refuses a typed value off the bounds or the step.

**Accessibility:** native buttons named "increment" and "decrement" (disabled at the
bounds), a labelled number field, the bounds in text. Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** change the number without a round trip; each step is
a POST and a redirect.

**Fallback:** without view transitions the page simply reloads.

**Enhanced:** the form is a swap root (`data-lui="swap"`), so with the `crate::enhance`
script each click is a background POST and only the form is replaced; its buttons are
disabled until the answer lands, so a double click counts once.

**Finding:** without the script every click is a full navigation, and a click that lands
while the page unloads is dropped. There is no optimistic update and no offline behaviour.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let plain = ui.counter("/counter", 3);
let bounded = ui.counter("/counter", 10).min(0).max(10).step(2).typed();
let html = bounded.render().into_string();
assert!(html.contains("value=\"inc\" aria-label=\"increment\" disabled"));
assert!(html.contains("name=\"value\" type=\"number\" value=\"10\" min=\"0\" max=\"10\" step=\"2\""));
// The handler's half: the same counter applies the posted `op`, clamped to its bounds.
assert_eq!(bounded.apply("dec", None), 8.0);

// The same in `lui!`:
let same = lui! { Counter("/counter", 10) min=0 max=10 step=2 typed; };
assert_eq!(same.into_string(), bounded.render().into_string());
```
