# Progress

`loco_ui::progress` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Progress` | `ui.progress(value: u64, max: u64)` | stable |

## Props of `Progress`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `label` | value | `text: &'a str` |  |  | A label above the bar. |
| `id` | value | `id: &'a str` |  | `id` | The bar's id instead of `lui-progress-<label>`. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | The bar's height: 0.25, 0.5 or 0.75rem. |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `brand` |  | The fill's colour. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `surface` |  | Soft tints the track with the tone; the others keep the gray track. |
| `radius` | choice: none, small, medium, large, full | `radius: Radius` | `full` |  | The ends of the track and the fill, from square to round. |
| `indeterminate` | switch |  | `off` |  | No value yet: a stripe slides along the track. |
| `show_value` | switch |  | `off` |  | The percentage beside the label, or above the bar without one. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-progress-field` | `--lui-space-2`, `--lui-text-2`, `--lui-gray-11`, `--lui-ok`, `--lui-warn`, `--lui-danger`, `--lui-progress-fill`, `--lui-primary`, `--lui-radius-sm`, `--lui-radius`, `--lui-radius-lg` |
| `.lui-progress-value` | `--lui-muted` |
| `.lui-progress` | `--lui-progress-h`, `--lui-progress-radius`, `--lui-progress-track`, `--lui-gray-4`, `--lui-progress-fill`, `--lui-primary`, `--lui-duration-slow`, `--lui-ease-out` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<progress>` | 6 | 6 | 6 |
| `appearance: none` | 84 | 80 | 15.4 |

- Fallback: without the pseudo-elements a browser draws its own bar
- Needs script: partly: moving on its own needs a streamed page or the script
- Responsive by: content
- Look after: Tremor ProgressBar

## Documentation

How far along a task is: a bar filled to a value out of a maximum, or an indeterminate
bar (`.indeterminate()`) when the server does not know yet; `.show_value()` writes the
percentage beside it. Radix Themes Progress's props: three heights
(`.size(..)`), a `Tone` for the fill, a soft or surface track
(`.variant(..)`) and `.radius(..)`.

**Platform features:** `<progress>` (baseline 2013) with its label tied by `<label for>`,
so a screen reader says the value; `appearance: none` and the `::-webkit-progress-*` and
`::-moz-progress-bar` pseudo-elements draw it in the shadcn shape. An indeterminate bar is
a `<progress>` with no value, matched by `:indeterminate` (baseline 2020) and drawn as a
sliding stripe that `prefers-reduced-motion: reduce` stops.

**Accessibility:** a native `<progress>` with its label. Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** move on its own; the value is what the server knew
when it rendered the page (a streamed page or the enhancement script can send a newer one).

**Fallback:** without the pseudo-elements a browser draws its own bar.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.progress(33, 100).label("Upload").render().into_string();
assert!(m.contains(r#"<progress id="lui-progress-upload" class="lui-progress" value="33" max="100">33%</progress>"#));
assert!(!m.contains("lui-progress-value"), "the percentage only with .show_value()");
let m = ui.progress(33, 100).label("Upload").show_value().render().into_string();
assert!(m.contains(r#"<span class="lui-progress-value">33%</span>"#));
assert_eq!(lui! { Progress(33, 100) label="Upload" show_value; }.into_string(), m);
let m = ui.progress(0, 100).label("Waiting").indeterminate().render().into_string();
assert!(m.contains(r#"class="lui-progress">"#) && !m.contains("value="));
// The same in `lui!`:
let same = lui! { Progress(0, 100) label="Waiting" indeterminate; };
assert_eq!(same.into_string(), m);
// A thin green bar on a tinted track, square ends.
let m = ui.progress(9, 10).size(1).tone(Tone::Ok).variant(Variant::Soft).radius(Radius::None).render().into_string();
assert!(m.starts_with(r#"<div class="lui-progress-field" data-size="1" data-tone="ok" data-variant="soft" data-radius="none">"#));
assert_eq!(lui! { Progress(9, 10) size=1 tone="ok" variant="soft" radius="none"; }.into_string(), m);
```
