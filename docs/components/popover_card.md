# Popover

`loco_ui::popover_card` · demo: `/popover-card`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Popover` | `ui.popover(trigger: &str, body: Markup)` | beta |

## Props of `Popover`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `side` | choice: top, bottom, start, end | `side: Side` | `bottom` |  | The side of the button it opens on; it flips when there is no room. |
| `align` | choice: start, center, end, baseline, stretch | `align: Align` | `start` |  | How it lines up with the button along that side. |
| `arrow` | switch |  | `off` |  | A small arrow pointing at the button. |
| `id` | value | `id: &'a str` |  | `id` | The panel's id instead of the trigger's slug. |

## Keyboard of `Popover`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter / Space` | Opens or closes the panel from its button. | no |
| `Tab / Shift+Tab` | Moves from the button into the panel's controls and back out. | no |
| `Escape` | Closes the panel and returns focus to its button. Without popover support the script closes it; without the script either, its button does. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-popover-card` |  |
| `.lui-popover-card-panel` | `--lui-space-4`, `--lui-text-2`, `--lui-leading-2`, `--lui-fg`, `--lui-popover`, `--lui-line`, `--lui-radius`, `--lui-shadow-md`, `--lui-highlight`, `--lui-popover-card-gap`, `--lui-z-popover`, `--lui-duration-fast`, `--lui-ease-out`, `--lui-duration`, `--lui-ease-spring` |
| `.lui-popover-card-arrow` |  |
| `.lui-popover-card-details` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `popover` | 114 | 125 | 17 |
| `anchor-name` | 125 | 147 | 26 |
| `position-try-fallbacks` | 128 | 147 | 26 |
| `@starting-style` | 117 | 129 | 17.5 |

- Fallback: centred in the viewport without anchor positioning; a <details> disclosure without popover
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Popover

## Documentation

A panel of anything (a short form, a profile card, help text) that opens over the page from
a button and closes on a click outside or Escape. Unlike a `crate::popover::Menu` it has
no items: the body is the caller's markup. After Radix Themes Popover: a popover surface
with a medium radius and shadow, 0.5rem from its button (0.75rem with the arrow), on the side and alignment asked
for, with an optional arrow.

**Platform features:**
- `popover` attribute with a `popovertarget` button (Chrome 114, Firefox 125, Safari 17):
  light dismiss, Escape and the top layer.
- CSS anchor positioning (Baseline 2026: Chrome 125, Firefox 147, Safari 26), used
  whatever the caps say: `anchor-name` on the button, `position-anchor` and `position-area`
  on the panel for `.side(..)` and `.align(..)`, and `position-try-fallbacks: flip-block,
  flip-inline` (Chrome 128, Firefox 147, Safari 26) so a panel that would leave the
  viewport opens on the other side. `crate::popover::Menu` and the context menu take
  the same fallbacks; `crate::tooltip::Tooltip` takes the same `Side`.
- `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) and `transition-behavior:
  allow-discrete` for the fade in and out.

**Accessibility:** the button says `aria-haspopup="dialog"` and the platform reports it
expanded while the panel is open; the panel is `role="dialog"` named by the button's text.
Tab moves from the button into the panel. Checked by axe-core in headless Firefox on every
demo route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** turn the arrow when a fallback flips the panel
(that needs anchored container queries, Chrome-only); the arrow points the asked-for way.

**Fallback:** a browser without anchor positioning shows the popover centred in the
viewport, still usable. Without `Caps` `Popover` it is a `<details>` disclosure whose panel
drops under the button; it closes on a second click, not on a click outside.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let m = ui.popover("Share", html! { p { "Anyone with the link can view." } }).render().into_string();
assert!(m.contains(r#"popovertarget="share""#) && m.contains(r#"<div id="share" class="lui-popover-card-panel" popover role="dialog" aria-label="Share""#));
let m = ui.popover("Share", html! { p { "Copied." } }).side(Side::Top).align(Align::End).arrow().id("share-2").render().into_string();
assert!(m.contains("position-area: block-start span-inline-start") && m.contains("lui-popover-card-arrow"));
// The same in `lui!`:
let same = lui! { Popover("Share", html! { p { "Copied." } }) side=(Side::Top) align=(Align::End) arrow id="share-2"; };
assert_eq!(same.into_string(), m);
// Without the popover attribute: a <details> disclosure.
let m = Ui::from(Caps::NONE).popover("Share", html! { "x" }).render().into_string();
assert!(m.starts_with(r#"<details class="lui-popover-card lui-popover-card-details""#));
```
