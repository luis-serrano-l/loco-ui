# Card

`loco_ui::card` · demo: `/marquee`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Card` | `ui.card()` | stable |

## Props of `Card`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `title` | value | `title: &'a str` |  |  | The heading at the top (`h3`). |
| `description` | value | `text: &'a str` |  |  | Muted text under the title. |
| `header` | value | `markup: Markup` |  |  | More header content, placed at the top right beside the title. |
| `body` | value | `markup: Markup` |  |  | The main content. |
| `footer` | value | `markup: Markup` |  |  | A row at the bottom, usually buttons, packed at its end like a dialog's. |
| `id` | value | `id: &'a str` |  | `id` | The root's id, for a link to the card or a swap target. |
| `beam` | switch |  | `off` |  | A light runs round the border, drawn with a conic gradient. |
| `glow` | switch |  | `off` |  | A spotlight at the top centre that brightens and grows on hover. |
| `gradient_border` | switch |  | `off` |  | The border drawn with `--lui-gradient-ring`. |
| `reveal` | switch |  | `off` |  | Fades and rises as it scrolls into view. |
| `classic` | switch |  | `off` |  | Radix's classic card: a deeper shadow and no border line. |
| `ghost` | switch |  | `off` |  | No frame, fill or shadow: only the card's spacing. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | The padding and gaps: 1rem, 1.5rem (the default) or 2rem. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `surface` |  | Surface or ghost, as in Radix; the others draw as surface. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-card` | `--lui-card`, `--lui-fg`, `--lui-line`, `--lui-radius-lg`, `--lui-shadow-sm`, `--lui-highlight` |
| `.lui-card-header` | `--lui-space-2`, `--lui-space-4` |
| `.lui-card-heading` |  |
| `.lui-card-title` | `--lui-text-3` |
| `.lui-card-description` | `--lui-muted`, `--lui-text-2` |
| `.lui-card-classic` | `--lui-shadow-md`, `--lui-highlight` |
| `.lui-card-ghost` |  |
| `.lui-card-body` |  |
| `.lui-card-footer` |  |
| `.lui-card-beam` | `--lui-beam-angle`, `--lui-beam`, `--lui-fg`, `--lui-beam-duration` |
| `.lui-card-glow` | `--lui-glow` |
| `.lui-card-gradient-border` | `--lui-card`, `--lui-gradient-ring` |
| `.lui-card-reveal` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `grid` | 57 | 52 | 10.1 |
| `conic-gradient` | 69 | 83 | 12.1 |
| `@property` | 85 | 128 | 16.4 |
| `mask-composite` | 120 | 53 | 15.4 |
| `color-mix()` | 111 | 113 | 16.2 |
| `linear-gradient(in oklch` | 111 | 127 | 16.2 |
| `animation-timeline: view()` | 115 | no | 26 |

- Fallback: none needed; each showpiece effect is at rest without its feature or under reduced motion
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Card

## Documentation

A bordered box with a header (title, description, and anything else such as an action),
a body and a footer: shadcn's Card, in Radix Themes Card's variants (surface, `.classic()`,
`.ghost()`) and three of its sizes (`.size(..)`, the padding). Settings panels, pricing
tiers, sign-in boxes.

**Platform features:** plain `<div>`s and a heading; the header is a grid, so an action
put in `.header(..)` sits at the top right beside the title. Four opt-in showpiece
effects, CSS only, each off under `prefers-reduced-motion: reduce` where it moves:
- `.beam()`: a light running round the border, a `conic-gradient` (Chrome 69, Firefox 83,
  Safari 12.1) whose start angle is an `@property` (Chrome 85, Firefox 128, Safari 16.4)
  animated by `@keyframes`, cut to a 1px ring with `mask-composite: exclude` (Chrome 120,
  Firefox 53, Safari 15.4), in `--lui-beam`, once round every `--lui-beam-duration`.
- `.glow()`: a spotlight at the top centre, `--lui-glow` in a `radial-gradient`, that
  brightens and grows on `:hover`; it does not follow the pointer. Behind
  `@supports` for `color-mix()` (Chrome 111, Firefox 113, Safari 16.2).
- `.gradient_border()`: the border drawn with `--lui-gradient-ring`, a padding-box layer
  over a border-box one, behind `@supports` for `linear-gradient(in oklch, ..)`
  (Chrome 111, Firefox 127, Safari 16.2).
- `.reveal()`: fades and rises as it scrolls into view, a scroll-driven animation with
  `animation-timeline: view()` (Chrome 115, Firefox no, Safari 26).

**Accessibility:** a `<div>` of ordinary markup; its title is an `<h3>`, so it shows in the
outline. Checked by axe-core in headless Firefox on every
demo route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed. Each effect sits inside `@supports` for the feature it needs,
and in `@media (prefers-reduced-motion: no-preference)` when it moves, so a browser without
it, or a visitor who asked for less motion, gets the card at rest: a plain border, no beam,
no glow, and shown in place from the start (a `.reveal()` card is never left hidden).

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.card().title("Team").body(html! { p { "3 members" } }).render().into_string();
assert!(m.contains(r#"<h3 class="lui-card-title">Team</h3>"#) && m.contains("3 members"));
// The same in `lui!`:
let same = lui! { Card title="Team" { p { "3 members" } } };
assert_eq!(same.into_string(), m);
// A description, an action in the header, and a footer of buttons.
let m = ui.card()
    .title("Plan")
    .description("Billed monthly.")
    .header(html! { (ui.badge("Current").secondary()) })
    .body(html! { p { "Pro, 12 seats" } })
    .footer(html! { (ui.button("Change plan").primary()) });
let m = m.render().into_string();
assert!(m.contains("lui-card-description") && m.contains("lui-card-action") && m.contains("lui-card-footer"));
// Opt-in showpieces, one class each on the root.
let m = ui.card().title("Pro").beam().glow().gradient_border().reveal().render().into_string();
assert!(m.starts_with(r#"<div class="lui-card lui-card-beam lui-card-glow lui-card-gradient-border lui-card-reveal">"#));
assert_eq!(lui! { Card title="Pro" beam glow gradient_border reveal; }.into_string(), m);
// Tighter padding, and no frame.
let m = ui.card().title("Note").size(1).variant(Variant::Ghost).render().into_string();
assert!(m.starts_with(r#"<div class="lui-card lui-card-ghost" data-size="1" data-variant="ghost">"#));
assert_eq!(lui! { Card title="Note" size=1 variant="ghost"; }.into_string(), m);
```
