# Alert

`loco_ui::alert` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Alert` | `ui.alert(title: &str)` | stable |

## Props of `Alert`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `description` | value | `text: &'a str` |  |  | A line of text under the title. |
| `body` | value | `markup: Markup` |  |  | Markup under the title (a list, a link), after the description. |
| `icon` | value | `icon: impl Into<Glyph<'a>>` |  |  | Another icon than the tone's own. |
| `danger` | switch |  | `off` |  | Something went wrong. |
| `warn` | switch |  | `off` |  | Something to watch. |
| `ok` | switch |  | `off` |  | Something worked. |
| `surface` | switch |  | `off` |  | A paler tint inside a border in the level's colour. |
| `outline` | switch |  | `off` |  | No tint: a border in the level's colour on the page. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | Radix's sizes 1 to 3: the padding, gaps and text. |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `info` |  | The colour; danger also makes it `role="alert"`. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `soft` |  | Soft, surface or outline; ghost drops the frame, and solid draws as soft. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `role="alert"` | 1 | 1 | 1 |
| `role="status"` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Callout

## Documentation

A callout in the page, after Radix Themes Callout: a title, a line or two of explanation
and an icon, in an info, danger, warning or success tone (or any
`Tone` with `.tone(..)`), soft by default or `.surface()`,
`.outline()` (`.variant(..)`), at Radix's sizes 1 to 3. For a message that stays where it
is, unlike a flash or a toast.

**Platform features:** a `role="alert"` box for the danger tone (read out when the page
loads) and `role="status"` for the others; the icon is decorative.

**Accessibility:** `role="alert"` for the danger tone, `role="status"` for the others; the
icon is decorative. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.alert("Heads up").description("You can add components to your app.").render().into_string();
assert!(m.contains(r#"role="status""#) && m.contains("Heads up"));
let m = ui.alert("Payment failed").danger().description("Your card was declined.").render().into_string();
assert!(m.contains(r#"class="lui-alert lui-callout lui-callout-danger" data-tone="danger" role="alert""#));
// The same in `lui!`:
let same = lui! { Alert("Payment failed") danger description="Your card was declined."; };
assert_eq!(same.into_string(), m);
// Surface and outline variants.
let m = ui.alert("Heads up").surface().render().into_string();
assert!(m.contains("lui-callout-info lui-callout-surface"));
assert_eq!(lui! { Alert("Heads up") surface; }.into_string(), m);
// The typed setters: a quiet gray note, smaller.
let m = ui.alert("Draft").tone(Tone::Neutral).variant(Variant::Outline).size(1).render().into_string();
assert!(m.contains(r#"class="lui-alert lui-callout lui-callout-neutral lui-callout-outline" data-size="1" data-tone="neutral" data-variant="outline""#));
assert_eq!(lui! { Alert("Draft") tone="neutral" variant="outline" size=1; }.into_string(), m);
```
