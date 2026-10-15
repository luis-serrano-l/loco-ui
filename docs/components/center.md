# Center

`loco_ui::center` · demo: `/container`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Center` | `ui.center()` | beta |

## Props of `Center`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What is centred. |
| `min_height` | value | `height: &'a str` |  |  | At least this tall (a CSS length), so there is room to centre in. |
| `text_center` | switch |  | `off` |  | Centre the lines of text too. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-center` | `--lui-space-2` |
| `.lui-center-text` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `place-items: center` | 59 | 45 | 11 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Flex align=center justify=center

## Documentation

Its content in the middle of the space it is given, across and down: an empty state in a
panel, a spinner in a card, a sign-in form on a page. After Radix Themes Flex with
`align="center" justify="center"`, as one word.

**Platform features:** `display: grid` with `place-items: center` (Chrome 59, Firefox 45,
Safari 11).

**Accessibility:** layout only: a `<div>` with no role; reading order is source order.
Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.center().body(html! { p { "Nothing here yet" } }).render().into_string();
assert_eq!(m, r#"<div class="lui-center"><p>Nothing here yet</p></div>"#);
// The same in `lui!`, where the block is the body:
let same = lui! { Center { p { "Nothing here yet" } } };
assert_eq!(same.into_string(), m);
// At least 12rem tall, the text centred too.
let m = ui.center().min_height("12rem").text_center().body(html! { p { "Nothing here yet" } }).render().into_string();
assert_eq!(m, r#"<div class="lui-center lui-center-text" style="min-height: 12rem"><p>Nothing here yet</p></div>"#);
assert_eq!(lui! { Center min_height="12rem" text_center { p { "Nothing here yet" } } }.into_string(), m);
```
