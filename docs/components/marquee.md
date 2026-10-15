# Marquee

`loco_ui::marquee` · demo: `/marquee`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Marquee` | `ui.marquee(label: &str)` | beta |

## Props of `Marquee`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `text` | item | `text: &'a str` |  |  | One item of plain text. |
| `item` | item | `markup: Markup` |  |  | One item of markup: a logo, a quote, a card. |
| `reverse` | switch |  | `off` |  | Scroll towards the end of the line instead of the start. |
| `duration` | number | `seconds: u32` | `40` | `style` | Seconds for one full loop (`--lui-marquee-duration`). |

## Keyboard of `Marquee`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Pauses the row while focus is on a link or button inside it. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-marquee` | `--lui-space-1`, `--lui-fg` |
| `.lui-marquee-track` | `--lui-space-4`, `--lui-marquee-duration` |
| `.lui-marquee-group` | `--lui-space-4` |
| `.lui-marquee-item` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `inert` | 102 | 112 | 15.5 |
| `translate` | 104 | 72 | 14.1 |
| `animation-play-state` | 43 | 16 | 9 |
| `mask-image` | 120 | 53 | 15.4 |

- Fallback: at rest: the items wrap and the copy is hidden, without translate or under reduced motion
- Needs script: no
- Responsive by: content
- Look after: Magic UI Marquee

## Documentation

A row that scrolls sideways on its own and loops: customer logos, a line of testimonials,
the tags of a changelog. It stops while the pointer is over it or focus is inside it, so a
link in it can be read and followed.

**Platform features:** a `<ul>` of the items, then the same list again with
`aria-hidden="true"` and `inert` (Chrome 102, Firefox 112, Safari 15.5), so the copy is
neither read nor focused. A `@keyframes` animation moves the pair with the `translate`
property (Chrome 104, Firefox 72, Safari 14.1) by exactly one list and a gap, so the copy
lands where the first list began and the loop has no seam; `animation-play-state` pauses it
on `:hover` and `:focus-within`. The edges fade out with `mask-image` (Chrome 120, Firefox
53, Safari 15.4). The pace is `--lui-marquee-duration` (40s), or `.duration(..)`.

**Accessibility:** a `role="region"` named by the label; the items are read once, in order,
since the copy is `aria-hidden` and `inert`; the motion stops on hover and focus, and does
not start under `prefers-reduced-motion: reduce`. Checked by axe-core in headless Firefox
on every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** nothing is missing.

**Fallback:** the motion sits inside `@supports (translate: 100%)` and
`@media (prefers-reduced-motion: no-preference)`. Without either, the row is at rest: the
items wrap onto as many lines as they need and the copy is not shown.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.marquee("Customers").text("Acme").text("Globex").render().into_string();
assert!(m.starts_with(r#"<div class="lui-marquee" role="region" aria-label="Customers">"#));
assert!(m.contains(r#"<ul class="lui-marquee-group" aria-hidden="true" inert>"#));
assert_eq!(m.matches("<li class=\"lui-marquee-item\">Acme</li>").count(), 2);
// Markup items, the other way round, at its own pace.
let quotes = ui.marquee("What people say")
    .item(html! { blockquote { "Fast." } })
    .item(html! { blockquote { "No script!" } })
    .reverse()
    .duration(20);
let quotes = quotes.render().into_string();
assert!(quotes.contains("lui-marquee lui-marquee-reverse") && quotes.contains("--lui-marquee-duration: 20s"));
// The same in `lui!`:
let same = lui! {
    Marquee("What people say") reverse duration=20 {
        item() { blockquote { "Fast." } }
        item() { blockquote { "No script!" } }
    }
};
assert_eq!(same.into_string(), quotes);
```
