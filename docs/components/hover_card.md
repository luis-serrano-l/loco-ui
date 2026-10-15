# Hover card

`loco_ui::hover_card` · demo: `/hover-card`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `HoverCard` | `ui.hover_card(trigger: &str, body: Markup)` | beta |

## Props of `HoverCard`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `href` | value | `href: &'a str` |  | `href` | The trigger is a link to `href` instead of a button. |
| `id` | value | `id: &'a str` |  | `id` | The card's id, `lui-hover-card-<slug of the trigger>` by default. |

## Keyboard of `HoverCard`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Shows the card while its trigger has focus, where the browser has interest invokers. | no |
| `Enter / Space` | Opens or closes the card from its button where the browser has no interest invokers. | no |
| `Enter` | Follows the trigger link when the card has one. | no |
| `Escape` | Closes the card. Without popover support it closes from its button instead. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-hover-card` | `--lui-space-1` |
| `.lui-hover-card-trigger` | `--lui-link`, `--lui-space-1`, `--lui-hit` |
| `.lui-hover-card-content` | `--lui-space-4`, `--lui-text-2`, `--lui-fg`, `--lui-popover`, `--lui-line`, `--lui-radius-lg`, `--lui-shadow-md`, `--lui-highlight`, `--lui-space-2` |
| `.lui-hover-card-link` | `--lui-hit` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `interestfor` | 142 | no | no |
| `popover="hint"` | 133 | no | no |
| `popovertarget` | 114 | 125 | 17 |
| `position-area` | 125 | 147 | 26 |

- Fallback: without interestfor a click opens the card (popovertarget); without popover a <details>
- Needs script: no
- Responsive by: -
- Look after: Radix Themes HoverCard

## Documentation

A preview card that shows while the pointer rests on a link or it has focus: who a user
is behind their @name, what a linked page holds. Where the browser cannot open it on
hover, a click opens the same card. After the Radix Themes HoverCard.

**Platform features:** with `Caps::InterestInvokers`, `interestfor` on the trigger
(Chrome 142; Firefox and Safari not yet) opens a `popover="hint"` (Chrome 133) on hover
and focus, and closes it when both leave; the trigger is the popover's implicit anchor,
so `position-area` (anchor positioning, Chrome 125, Firefox 147, Safari 26) puts the card
under it. Without it, `popover` with `popovertarget` (Chrome 114, Firefox 125, Safari 17)
opens the card on click and light dismiss closes it.

**Accessibility:** the trigger is a link or a button, and the card follows it in the
document, so a screen reader reaches it after the trigger. A button trigger toggles the card
with Enter or Space; a link trigger gets a separate button, named "More about …", where
hover cannot open the card. Checked by axe-core in headless Firefox on every demo route,
both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** wait before opening (the platform's
`interest-delay` decides) or open on hover where `interestfor` is missing.

**Fallback:** without `Caps::InterestInvokers` the card is `popover="auto"` opened by
click through `popovertarget`: on a button trigger the button itself, on a link trigger a
small chevron button after the link (a link cannot target a popover). Without
`Caps::Popover` it is a `<details>` disclosure under the trigger. Without anchor
positioning the popover sits where the browser centres it.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let m = ui.hover_card("@ada", html! { "Ada Lovelace, first programmer." }).render().into_string();
assert!(m.contains(r#"interestfor="lui-hover-card-ada""#) && m.contains(r#"popover="hint""#));
// A link trigger with its own id; without interest invokers a click opens the card.
let old = Ui::from(Caps::ASSUMED);
let m = old.hover_card("Release notes", html! { "What changed in 0.2." }).href("/changelog").id("notes");
let html = m.render().into_string();
assert!(html.contains(r#"<a class="lui-hover-card-link" href="/changelog">Release notes</a>"#) && html.contains(r#"popovertarget="notes""#));
assert!(html.contains(r#"popover="auto""#) && !html.contains("interestfor"));
// The same in `lui!`:
let ui = old;
let same = lui! { HoverCard("Release notes", html! { "What changed in 0.2." }) href="/changelog" id="notes"; };
assert_eq!(same.into_string(), html);
```
