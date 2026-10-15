# Rating

`loco_ui::rating` · demo: `/rating`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Rating` | `ui.rating(name: &str, label: &str)` | beta |

## Props of `Rating`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `max` | number | `stars: u8` | `5` |  | How many stars (1 to 10). |
| `value` | number | `stars: u8` |  | `checked` | The picked score, instead of the query's `name`; 0 for none. |
| `required` | switch |  | `off` | `required` | A star must be picked before the form submits. |
| `readonly` | switch |  | `off` |  | The score shown as an image and posted from a hidden input, not changeable. |
| `disabled` | switch |  | `off` | `disabled` | `disabled` and `aria-disabled`: shown greyed and not posted. |
| `help` | value | `help: &'a str` |  |  | Help text under the stars. |
| `error` | value | `message: &'a str` |  |  | A server message under the stars. |
| `id` | value | `id: &'a str` |  | `id` | What the stars' ids start from, `f-<name>` by default. |

## Keyboard of `Rating`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves focus onto the picked star, or the first one. | no |
| `ArrowRight / ArrowUp` | Picks one star more, wrapping from the last star to the first. | no |
| `ArrowLeft / ArrowDown` | Picks one star fewer, wrapping from the first star to the last. | no |
| `Space` | Picks the focused star when none is picked yet. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-rating` | `--lui-text-2`, `--lui-leading-2`, `--lui-danger` |
| `.lui-rating-label` | `--lui-text-2`, `--lui-leading-2` |
| `.lui-rating-stars` |  |
| `.lui-rating-star` | `--lui-gray-7`, `--lui-hit` |
| `.lui-icon` | `--lui-duration-fast`, `--lui-warn`, `--lui-gray-7` |
| `.lui-rating-input` | `--lui-ring`, `--lui-radius-sm` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `type="radio"` | 1 | 1 | 1 |
| `:has()` | 105 | 121 | 15.4 |

- Fallback: only the picked star is filled, and hovering fills nothing, without :has()
- Needs script: no
- Responsive by: content
- Look after: Origin UI star rating

## Documentation

A row of stars to pick a score from 1 to 5 (or `.max(n)`), part of the form around it:
each star is a radio button, so the form posts the number and the arrow keys move it.
`.readonly()` shows a score without letting it change, and still posts it.

After the Origin UI star ratings: 1.25rem stars in --lui-gray-7, filled in the warning
colour up to the picked one, the stars up to the pointer filled while it hovers.

**Platform features:** `<fieldset>` of `<input type="radio">` (every browser), each star an
inline SVG; `:has()` (Chrome 105, Firefox 121, Safari 15.4) fills the stars before the
checked one and the ones up to the hovered star.

**Accessibility:** a `<fieldset>` named by its legend, each radio named "3 stars"
(visually hidden text), arrow keys move the pick as in any radio group; a read-only rating
is an image named "3 out of 5". Checked by axe-core in headless Firefox on every demo
route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing: picking a star is choosing a
radio, and the surrounding form sends it.

**Fallback:** without `:has()` only the picked star is filled (a general-sibling `~` rule),
not the ones before it, and hovering fills nothing.

```rust
use loco_ui::prelude::*;
// The score is the query's `stars`, or `.value(..)` (a saved one).
let ui = Ui::from_request("/review", "stars=4", "");
let m = ui.rating("stars", "Your rating").render().into_string();
assert!(m.contains(r#"<legend>Your rating</legend>"#));
assert!(m.contains(r#"type="radio" name="stars" value="4" checked"#));
assert!(m.contains(r#"<span class="lui-sr">4 stars</span>"#));
// The same in `lui!`:
assert_eq!(lui! { Rating("stars", "Your rating"); }.into_string(), m);
// Out of 10, shown and posted but not changeable.
let m = ui.rating("score", "Score").max(10).value(7).readonly().render().into_string();
assert!(m.contains(r#"role="img" aria-label="7 out of 10""#));
assert!(m.contains(r#"<input type="hidden" name="score" value="7">"#));
assert_eq!(lui! { Rating("score", "Score") max=10 value=7 readonly; }.into_string(), m);
```
