# Text

`loco_ui::text` · demo: `/toggle-group`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Text` | `ui.text(text: &str)` | beta |

## Props of `Text`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `size` | value | `size: u8` | `inherit` |  | Radix's sizes 1–9: 12, 14, 16, 18, 20, 24, 28, 35 or 60px. |
| `weight` | choice: light, regular, medium, bold | `weight: Weight` | `inherit` |  | How heavy the letters are. |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `inherit` |  | The colour: step 11 of the tone's scale. |
| `as_p` | switch |  | `off` |  | A `<p>`: a paragraph of its own. |
| `as_div` | switch |  | `off` |  | A `<div>`: a block that is not a paragraph (a caption, a cell's second line). |
| `muted` | switch |  | `off` |  | The muted gray of hints and secondary lines. |
| `truncate` | switch |  | `off` |  | One line, cut with an ellipsis where it overflows. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `text-overflow: ellipsis` | 1 | 7 | 1.3 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Text

## Documentation

A run of text at a size, weight and colour from the type scale: a caption, a lede, a
muted hint, a status in the danger colour, a file name cut short with an ellipsis. After
Radix Themes Text: sizes 1 to 9 on the scale Heading uses, an inline `<span>` by default
and a `<p>` or `<div>` on request, `.truncate()` for one line.

`ui.text(..)` is also how a component reads its own words: given a
`Text` key it returns the word in the visitor's language, given a
`&str` it returns this builder (`TextArg`).

**Platform features:** `<span>`, `<p>` and `<div>` (baseline 1995); `.truncate()` is
`text-overflow: ellipsis` (Chrome 1, Firefox 7, Safari 1.3) on one line.

**Accessibility:** plain text, read in place; colour is never the only signal (a danger
text says what went wrong). A truncated text keeps its whole content in the DOM, so a
screen reader reads all of it. Checked by axe-core in headless Firefox on every demo route,
both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let t = ui.text("Last edited today").render().into_string();
assert_eq!(t, r#"<span class="lui-text">Last edited today</span>"#);
// The same in `lui!`:
let same = lui! { Text("Last edited today"); };
assert_eq!(same.into_string(), t);
// A muted paragraph at size 2, and a danger line in medium weight.
let p = ui.text("No invoices yet.").as_p().size(2).muted().render().into_string();
assert_eq!(p, r#"<p class="lui-text lui-text-muted" data-size="2">No invoices yet.</p>"#);
assert_eq!(lui! { Text("No invoices yet.") as_p size=2 muted; }.into_string(), p);
let e = ui.text("Payment failed").tone(Tone::Danger).weight(Weight::Medium).truncate();
let e = e.render().into_string();
assert!(e.contains(r#"class="lui-text lui-text-truncate" data-weight="medium" data-tone="danger""#));
assert_eq!(lui! { Text("Payment failed") tone="danger" weight="medium" truncate; }.into_string(), e);
```
