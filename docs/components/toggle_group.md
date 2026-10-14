# Toggle group

`loco_ui::toggle_group` · demo: `/toggle-group`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `ToggleGroup` | `ui.toggle_group(name: &str, label: &str)` | beta |

## Props of `ToggleGroup`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `option` | item | `value: &'a str, text: &'a str` |  | `value` | An option posting `value`, shown as `text`. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | The option added last shows this icon, its text becoming a hidden label. |
| `disabled` | modifier |  |  | `disabled` | The option added last cannot be pressed. |
| `value` | value | `value: &'a str` |  | `checked` | Press this option (again for more with `.multiple()`); the query's own values by default. |
| `multiple` | switch |  | `off` | `type` | Several options may be pressed (checkboxes, not radios). |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | The items' height and text: Radix's sizes 1 to 3. |
| `radius` | choice: none, small, medium, large, full | `radius: Radius` | `medium` |  | The corners of the track and the chip, from square to a pill. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<fieldset>` | 1 | 1 | 1 |
| `:checked` | 1 | 1 | 1 |
| `:focus-visible` | 86 | 85 | 15.4 |

- Fallback: none needed
- Needs script: partly: applying a choice the moment it is pressed needs script (or the form's submit)
- Responsive by: content
- Look after: Radix Themes SegmentedControl

## Documentation

A row of pressable options, one picked (alignment) or several (bold, italic), part of the
form around it. Each option is a real radio button or checkbox, drawn as a segmented
button, so the form posts the choice and the browser handles the keys.

**Platform features:** `<fieldset>` of `<input type="radio">` (one) or
`type="checkbox"` (several) inside their labels; `:checked` and `:focus-visible` on the
input style the face next to it (every browser).

**Accessibility:** a `<fieldset>` named by its visually hidden `<legend>`; arrow keys move
between radios and Space toggles a checkbox, as the platform does; an icon-only option has
its text as a hidden label. Checked by axe-core in headless Firefox on every demo route,
both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** apply a choice the moment it is pressed; the
surrounding form sends it (a submit button, or the enhancement script's in-place post).

The look follows Radix Themes SegmentedControl: a gray track, items of one width, the
picked one a raised chip; with one pick the chip slides between items (CSS only, `:has()`
and a transition), in Radix's sizes 1 to 3 (`.size(..)`) and radii (`.radius(..)`). A
narrow container scrolls the row inside itself.

**Fallback:** without `:has()` (Chrome < 105, Firefox < 121) the checked item takes the
chip look itself and nothing slides.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/editor", "align=center", "");
let m = ui.toggle_group("align", "Alignment").option("left", "Left").option("center", "Center");
let html = m.render().into_string();
assert!(html.contains(r#"type="radio" name="align" value="center" checked"#));

let m = ui.toggle_group("style", "Text style").multiple()
    .option("bold", "Bold").icon(Icon::Bold)
    .option("italic", "Italic").icon(Icon::Italic)
    .value("bold");
let html = m.render().into_string();
assert!(html.contains(r#"type="checkbox" name="style" value="bold" checked"#));
// The same in `lui!`:
let same = lui! { ToggleGroup("style", "Text style") multiple {
    option "bold" "Bold" icon=(Icon::Bold);
    option "italic" "Italic" icon=(Icon::Italic);
    value "bold";
} };
assert_eq!(same.into_string(), html);
// Smaller, and a pill.
let m = ui.toggle_group("view", "View").option("list", "List").option("grid", "Grid").size(1).radius(Radius::Full);
let html = m.render().into_string();
assert!(html.contains(r#"data-size="1" data-radius="full""#));
let same = lui! { ToggleGroup("view", "View") size=1 radius="full" { option "list" "List"; option "grid" "Grid"; } };
assert_eq!(same.into_string(), html);
```
