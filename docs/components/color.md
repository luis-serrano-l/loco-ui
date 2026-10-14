# Color

`loco_ui::color` · demo: `/inputs`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Color` | `ui.color(name: &str, label: &str)` | stable |

## Props of `Color`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `value` | value | `value: &'a str` |  | `value` | The `#rrggbb` colour, instead of the query's `name`. |
| `presets` | value | `presets: &'a [&'a str]` |  |  | `#rrggbb` swatches that post `<name>-preset` when clicked. |
| `alpha` | number | `percent: u8` |  |  | An opacity slider (`<name>-alpha`) at `percent`, clamped to 100. |
| `disabled` | switch |  | `off` | `disabled` | `disabled` and `aria-disabled`: shown greyed and not posted. |
| `readonly` | switch |  | `off` |  | Shown and posted, not changeable: disabled controls and hidden inputs with the values. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<input type="color">` | 20 | 29 | 12.1 |
| `color-mix()` | 111 | 113 | 16.2 |

- Fallback: text field accepting #rrggbb
- Needs script: no
- Responsive by: content
- Look after: Radix Themes TextField

## Documentation

A colour picker whose value the server remembers, no script. It can offer a row of preset
swatches and an opacity slider.

**Platform features:** `<input type="color">` (Chrome 20, Firefox 29, Safari 12.1). The
swatch beside it is a plain `<span>` painted with the server's current value through the
custom properties `--lui-color-value` and `--lui-color-alpha`, mixed with `color-mix()`
(Chrome 111, Firefox 113, Safari 16.2) over a checkerboard so transparency shows. Presets
are `<button name="<name>-preset" value="#rrggbb">`: one click posts the form with that
colour. Opacity is an `<input type="range">` named `<name>-alpha` (0 to 100).

**Accessibility:** a native colour input with its label; presets are named buttons in a
labelled group with `aria-pressed`. Checked by axe-core in headless Firefox on every demo
route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** a live preview of the chosen colour before the form
is sent, and an eyedropper.

**Fallback:** none needed; a browser without a colour picker shows a text field that
accepts `#rrggbb`.

**Enhanced:** the enhancement script repaints the swatch and the code while the picker moves
and mirrors the opacity into its `<output>`.

```rust
use loco_ui::{prelude::*, color::hex_alpha};
// The value is the query's `accent`, or `.value(..)` (a saved colour).
let ui = Ui::from_request("/", "accent=%232f5bea", "");
assert!(ui.color("accent", "Accent").render().into_string().contains(r##"value="#2f5bea""##));
let m = ui.color("accent", "Accent").value("#2f5bea").presets(&["#1f6f5f", "#b3261e"]).alpha(80);
let html = m.render().into_string();
assert!(html.contains("name=\"accent-preset\" value=\"#b3261e\""));
assert!(html.contains("name=\"accent-alpha\"") && html.contains(r#"<label for="f-accent">"#));
assert_eq!(hex_alpha("#2f5bea", 80), "#2f5beacc");

// The same in `lui!`:
let same = lui! {
    Color("accent", "Accent") value="#2f5bea" presets=(&["#1f6f5f", "#b3261e"]) alpha=80;
};
assert_eq!(same.into_string(), m.render().into_string());
```
