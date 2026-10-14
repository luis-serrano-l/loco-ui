# Badge

`loco_ui::badge` · demo: `/button`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Badge` | `ui.badge(text: impl Display)` | stable |

## Props of `Badge`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `secondary` | switch |  | `off` |  | A gray tone instead of the brand. |
| `danger` | switch |  | `off` |  | The `--lui-danger` tone. |
| `outline` | switch |  | `off` |  | A border and no fill (gray unless toned). |
| `ok` | switch |  | `off` |  | The `--lui-ok` tone. |
| `warn` | switch |  | `off` |  | The `--lui-warn` tone. |
| `solid` | switch |  | `off` |  | Filled with the tone, light text on it. |
| `surface` | switch |  | `off` |  | A pale tint inside a border of the tone. |
| `href` | value | `href: &'a str` |  | `href` | Make the badge a link. |
| `shimmer` | switch |  | `off` |  | A light sweeps across the badge, over its own fill. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `1` |  | Radix's sizes 1 to 3: more padding, and larger text at 3. |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `brand` |  | The colour. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `soft` |  | How the tone is painted; ghost is the tone's text with no fill or border. |
| `radius` | choice: none, small, medium, large, full | `radius: Radius` | `small` |  | The corners, from square to a pill. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<span>` | 1 | 1 | 1 |
| `translate` | 104 | 72 | 14.1 |

- Fallback: none needed; a shimmer badge is at rest without translate
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Badge

## Documentation

A short label beside something: a status, a count, a tag. After Radix Themes Badge: a tone
(the brand by default; `.secondary()` gray, `.danger()`, `.ok()`, `.warn()`, or any
`Tone` with `.tone(..)`) in a variant (soft by default: a tint with
darker text; `.solid()`, `.surface()`, `.outline()`, or `.variant(..)`), at Radix's size 1
unless `.size(..)` says 2 or 3, with no wrapping.

**Platform features:** a `<span>` (or an `<a>` with `.href()`); nothing interactive of its
own. `.shimmer()` sweeps a light across it, as on a button: an `::after` layer moved with
the `translate` property (Chrome 104, Firefox 72, Safari 14.1) under
`@supports (translate: 100%)` and `prefers-reduced-motion: no-preference`, in
`--lui-shimmer` over a filled badge and `--lui-shimmer-surface` over a tinted or outlined one.

**Accessibility:** plain text in a `<span>`; the tone colours are mixed with the text colour
so they pass AA on their tint. Checked by axe-core in headless Firefox on every demo route,
both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed. A `.shimmer()` badge without `translate`, or under
`prefers-reduced-motion: reduce`, is the same badge at rest.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
assert_eq!(ui.badge("New").render().into_string(), r#"<span class="lui-badge">New</span>"#);
// The same in `lui!`:
let same = lui! { Badge("New"); };
assert_eq!(same.into_string(), ui.badge("New").render().into_string());
let paid = ui.badge("Paid").ok().render().into_string();
assert!(paid.contains("lui-badge lui-badge-ok"));
let tag = ui.badge("rust").outline().href("/tags/rust").render().into_string();
assert!(tag.starts_with(r#"<a class="lui-badge lui-badge-outline" data-variant="outline" href="/tags/rust">"#));
// Opt-in motion: a light sweeps across it.
let new = ui.badge("New").shimmer().render().into_string();
assert_eq!(new, r#"<span class="lui-badge lui-badge-shimmer">New</span>"#);
assert_eq!(lui! { Badge("New") shimmer; }.into_string(), new);
// Radix's other looks, in any tone.
let loud = ui.badge("Failed").danger().solid().render().into_string();
assert_eq!(loud, r#"<span class="lui-badge lui-badge-danger lui-badge-solid" data-tone="danger" data-variant="solid">Failed</span>"#);
assert_eq!(lui! { Badge("Failed") danger solid; }.into_string(), loud);
// The typed setters, written as strings in `lui!`.
let beta = ui.badge("Beta").tone(Tone::Info).variant(Variant::Surface).size(2).radius(Radius::Full);
let beta = beta.render().into_string();
assert!(beta.contains(r#"data-size="2" data-tone="info" data-variant="surface" data-radius="full""#));
assert_eq!(lui! { Badge("Beta") tone="info" variant="surface" size=2 radius="full"; }.into_string(), beta);
```
