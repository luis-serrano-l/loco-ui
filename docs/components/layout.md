# Layout

`loco_ui::layout`

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-gap-0` |  |
| `.lui-gap-1` | `--lui-space-1` |
| `.lui-gap-2` | `--lui-space-2` |
| `.lui-gap-3` | `--lui-space-3` |
| `.lui-gap-4` | `--lui-space-4` |
| `.lui-gap-6` | `--lui-space-6` |
| `.lui-gap-8` | `--lui-space-8` |
| `.lui-sr` |  |
| `.lui-skip` | `--lui-space-2`, `--lui-z-tooltip`, `--lui-space-3`, `--lui-radius-sm`, `--lui-popover`, `--lui-fg`, `--lui-shadow-md` |
| `.lui-note` | `--lui-muted`, `--lui-text-2` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `@view-transition` | 126 | no | 18.2 |
| `prefers-color-scheme` | 76 | 67 | 12.1 |
| `custom properties` | 49 | 31 | 9.1 |
| `@layer` | 99 | 97 | 15.4 |
| `light-dark()` | 123 | 120 | 17.5 |
| `forced-colors` | 89 | 89 | 16 |

- Fallback: plain navigations (root never cross-fades); without light-dark() the page is unthemed black on white
- Needs script: no
- Responsive by: content
- Look after: Radix Themes scales and depth, a Linear / Magic UI finish

## Documentation

The HTML shell every page uses: doctype, head, the stylesheet (inline, or linked to
`/lui.css`), a skip link, `<main id="main">`, the capability beacons that teach the server
what this browser supports, and the optional `crate::enhance` script tag (the page works
the same without it).

**Platform features:** cascade layers, `@layer` (Chrome 99, Firefox 97, Safari 15.4): the
stylesheet is `lui.reset`, `lui.tokens`, `lui.components`, `lui.blocks` and `lui.modes`,
and an app's own CSS (`Look::css`, `Page::css`) comes unlayered after
it, so it wins without `!important`. `light-dark()` (Chrome 123, Firefox 120, Safari 17.5)
holds both schemes in one declaration per token; `color-scheme` on `:root` picks one, from
`prefers-color-scheme` or `data-theme`. `@view-transition { navigation: auto }` (Chrome
126+, Safari 18.2+, not Firefox) lets elements with a `view-transition-name` morph across
full-page navigations; the root itself swaps instantly. `<link rel="expect"
blocking="render">` (Chrome 124+) holds the transition until `<main>` is parsed; it is
emitted only with `Cap::ViewTransitions`. `@media (forced-colors: active)`,
`(prefers-contrast: more)` and `print` adjust the components in `MODES`.

**Fallback:** browsers without view transitions navigate normally. A browser without
`light-dark()` (Chrome before 123) drops every colour token and shows the page in its
default black on white: readable, unthemed (FINDINGS, M39).

**Theming:** every colour, size, radius, shadow, duration and layer the components use is
one of the `--lui-*` custom properties. `Tokens` holds them; `ui.page(..).tokens(&tokens)` emits them
once per page after the stylesheet, so a different theme is a value, not a CSS file.
`docs/theming.md` lists each token and what it affects.

**Responsive (M34):** components respond to the box they sit in, not the viewport. Every
component root that changes layout sets `container: lui-<component> / inline-size`, and
its CSS writes the narrow layout as the base, then adds wider layouts with
`@container lui-<component> (width >= …)`. Three breakpoints, used everywhere and never
others: *narrow* below `30rem`, *medium* from `30rem`, *wide* from `48rem`. Custom
properties cannot appear in an `@container` condition, so the numbers are written
literally. Viewport `@media` is kept for `prefers-*`, `forced-colors`, `print` and
`pointer` only. Blitz does not support `@container` (FINDINGS, M34), so its shots show the
narrow base.

```rust
use std::borrow::Cow;
use loco_ui::{prelude::*, layout::{Palette, Tokens}};
let ui = Ui::from(Caps::all());
let page = ui.page("Title", html! { p { "body" } });
let tokens = Tokens { light: Palette { primary: Cow::Borrowed("#7a3b1e"), ..Tokens::DEFAULT.light }, ..Tokens::DEFAULT };
assert!(page.tokens(&tokens).into_string().contains("--lui-primary: light-dark(#7a3b1e, var(--lui-brand-9))"));
```
