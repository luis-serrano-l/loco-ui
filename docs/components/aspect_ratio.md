# Aspect ratio

`loco_ui::aspect_ratio` · demo: `/container`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `AspectRatio` | `ui.aspect_ratio()` | beta |

## Props of `AspectRatio`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What fills the box: a picture, a video, a map. |
| `ratio` | value | `width: u32, height: u32` | `1 / 1` |  | Width to height: `(16, 9)`, `(4, 3)`. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-aspect-ratio` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `aspect-ratio` | 88 | 89 | 15 |
| `object-fit: cover` | 32 | 36 | 10 |

- Fallback: the box takes its content's height
- Needs script: no
- Responsive by: content
- Look after: Radix Themes AspectRatio

## Documentation

A box that keeps its width-to-height ratio at every width: a video, a map, a picture whose
file may not have loaded yet, so nothing below it jumps when it does. After Radix Themes
AspectRatio: the ratio as a number (16 / 9, 4 / 3, 1), square by default, the child
stretched to fill it.

**Platform features:** the `aspect-ratio` property (Chrome 88, Firefox 89, Safari 15),
set in a `style` attribute; `object-fit: cover` (Chrome 32, Firefox 36, Safari 10) crops a
picture to the box.

**Accessibility:** layout only: a `<div>` with no role; an `<img>` inside keeps its own
`alt`. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** a browser without `aspect-ratio` leaves the content in the flow (the child
is stretched over the box only under `@supports (aspect-ratio: 1 / 1)`), so the box takes
its content's height.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.aspect_ratio().body(html! { img src="/a.jpg" alt="A lake"; }).render().into_string();
assert_eq!(m, r#"<div class="lui-aspect-ratio"><img src="/a.jpg" alt="A lake"></div>"#);
// The same in `lui!`, where the block is the body:
let same = lui! { AspectRatio { img src="/a.jpg" alt="A lake"; } };
assert_eq!(same.into_string(), m);
// Widescreen.
let m = ui.aspect_ratio().ratio(16, 9).body(html! { img src="/a.jpg" alt="A lake"; }).render().into_string();
assert!(m.starts_with(r#"<div class="lui-aspect-ratio" style="aspect-ratio: 16 / 9">"#));
assert_eq!(lui! { AspectRatio ratio=(16, 9) { img src="/a.jpg" alt="A lake"; } }.into_string(), m);
```
