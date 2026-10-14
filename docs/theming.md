# Theming

Every colour, size, corner, shadow, duration and stacking level in `loco-ui` is a `--lui-*`
custom property. The components never name a colour of their own (a test in
`loco-ui/src/lib.rs` fails if one does), so a theme is a value, not a stylesheet:
`layout::Tokens` holds every token, `ui.page(..).tokens(&t)` emits them once per page, and
`Ui::tokens(t)` or a `Look` does it for every page.

## Your CSS wins without `!important`

`loco_ui::stylesheet()` is five cascade layers, declared once at its top:

```css
@layer lui.reset, lui.tokens, lui.components, lui.blocks, lui.modes;
```

| Layer | What it holds |
|---|---|
| `lui.reset` | `layout::RESET`: box sizing, body colours and font, `main`'s width, headings on the type ramp, links, `<code>`, bare tables. A minimal normalisation. |
| `lui.tokens` | `Tokens::DEFAULT.css()`: every `--lui-*` token on `:root`. A page's own tokens (`Page::tokens`, `Ui::tokens`, a `Look`) go in a second `<style class="lui-tokens">` in the same layer, later in source order, so they win. |
| `lui.components` | the layout's own rules, then each component's `CSS` const. |
| `lui.blocks` | the whole-page blocks (app shell, auth, settings, record, dashboard, error pages). |
| `lui.modes` | `layout::MODES`: forced colours, more contrast and print. Last, so a mode beats a component. |

Anything not in a layer beats everything in one, whatever its specificity. `Look::css` and
`Page::css` are emitted unlayered after the stylesheet, so an app rule such as
`.lui-button { border-radius: 0 }` wins over the library's `.lui-button:is(...)` without
`!important` or a longer selector. Put your own CSS in a layer only if you want the library to
beat it. Cascade layers are Baseline 2022 (Chrome 99, Firefox 97, Safari 15.4); older browsers
ignore the whole sheet, the same as a browser without `light-dark()` below.

## Both schemes in one declaration

Each colour token is declared once, with both schemes in `light-dark()`:

```css
:root { color-scheme: light dark; --lui-bg: light-dark(#fcfcfd, #111113); ... }
:root[data-theme="light"] { color-scheme: light; }
:root[data-theme="dark"]  { color-scheme: dark; }
```

`color-scheme` picks the side: the visitor's `prefers-color-scheme` by default, or the
`data-theme` that `theme_toggle` puts on `<html>` from a cookie, so a theme is server state like
everything else. A value the same in both schemes is written once, without `light-dark()`.
Any element can carry its own `color-scheme: dark` and its subtree turns dark: the theme
builder's previews and the token table do that. `light-dark()` is Baseline 2024 (Chrome 123,
Firefox 120, Safari 17.5) and used unconditionally: an older browser drops every colour token
and shows the page in its default black on white, readable but unthemed (FINDINGS, M39).

## The builder

To pick the tokens by eye, the demo's theme builder (`/theme`, `cargo run -p demo`) asks for a
brand and a gray (a picker, or a swatch of Radix Colors' step 9), a radius and its ramp factor,
the density and the font. Each colour becomes step 9 of a 12-step scale for light and dark
(`layout::Scale::derive`, shaped like `Scale::INDIGO` and `Scale::SLATE`), so the roles that
alias the scales follow; text on the brand fill turns near-black when white would fall under
4.5:1. The builder previews a card, a form, buttons, a badge, an alert, the four shadows, the
primary gradient and the radius ramp in both schemes, lists every colour role with its light
and dark value and a swatch of each, and downloads the result as `theme.css`. Paste the file
after the stylesheet (`page.css(include_str!("theme.css"))`), or the two `Scale` consts it
prints into a `Tokens`. It needs no script: the form sends its choices in the URL, so a theme
can be shared as a link.

## The tokens

`Tokens` has one field per group. Every value is a `Cow<'static, str>`, so a palette read from
a database is a `Tokens` value (`primary: tenant.colour.into()`), while `Tokens::DEFAULT` stays
a `const` built from `Cow::Borrowed`.

| Field | Type | Tokens it sets |
|---|---|---|
| `gray`, `brand` | `Scale` (12 light and 12 dark steps) | `--lui-gray-1` … `-12`, `--lui-brand-1` … `-12` |
| `light`, `dark` | `Palette` (21 roles) | `--lui-bg` … `--lui-on-info`, one `light-dark()` each |
| `font` | `Fonts { sans, mono, serif }` | `--lui-font-sans`, `--lui-font-mono`, `--lui-font-serif` |
| `type_scale` | `TypeScale { size, leading }` (9 steps each) | `--lui-text-1` … `-9`, `--lui-leading-1` … `-9` |
| `size` | `Sizes { height }` (3 steps) | `--lui-size-1` … `-3`, aliased as `--lui-control-h-sm`, `--lui-control-h`, `--lui-control-h-lg` |
| `radius` | `Radius { base, factor }` | `--lui-radius-factor`, `--lui-radius-none`, `-1`, `-2`, `-3`, `-full`, aliased as `--lui-radius-sm`, `--lui-radius`, `--lui-radius-lg` |
| `shadow` | `Depth { xs, sm, md, lg, highlight, gradient_primary, gradient_ring }` | `--lui-shadow-xs` … `-lg`, `--lui-highlight`, `--lui-gradient-primary`, `--lui-gradient-ring` |
| `motion` | `Motion { fast, base, slow, ease }` | `--lui-duration-fast`, `--lui-duration`, `--lui-duration-slow`, `--lui-ease-out` |
| `z` | `ZIndex { sticky, drawer, dialog, popover, toast, tooltip }` | `--lui-z-sticky` … `--lui-z-tooltip` |
| `density` | `Density::{Comfortable, Compact}` | multiplies `--lui-space` and `--lui-size-*` by 1 or 0.85 |
| `space` | a length | `--lui-space` and the derived `--lui-space-1` … `-16` |

Change one group and keep the rest: `Tokens { radius: Radius::new("0px"), ..Tokens::DEFAULT }`,
`Tokens { density: Density::Compact, ..Tokens::DEFAULT }`, or one role,
`Palette { primary: "#7a3b1e".into(), ..Tokens::DEFAULT.light }`.

### Colour

Colour starts from two 12-step scales after Radix Colors, `gray` and `brand`, written in oklch
and emitted as hex. Each step has one job: 1–2 backgrounds, 3–5 component surfaces (normal,
hover, pressed), 6–8 borders and the focus ring, 9–10 solid fills, 11–12 text. The default is a
Linear-like theme: Radix slate for the grays (`Scale::SLATE`) and indigo for the brand
(`Scale::INDIGO`). Swap a scale and every role built on it follows. `Scale::new(light, dark)`
builds one from two arrays of 12 colours in a `const`.

The roles are shadcn/ui's, under `--lui-*` names, and by default each is an alias onto a step
(`--lui-bg: var(--lui-gray-1)`). The status colours are Radix Colors step 11, the step made for
text (light green and amber a shade darker). A test in `layout.rs` checks every text role on
every surface in both schemes clears 4.5:1.

| Token | Default light / dark | What it affects |
|---|---|---|
| `--lui-gray-1` … `-12` | slate | The neutral scale the roles below alias. |
| `--lui-brand-1` … `-12` | indigo | The brand scale: `primary` is step 9, `ring` step 8, `link` step 11. |
| `--lui-bg` | gray 1: `#fcfcfd` / `#111113` | The page background. The sticky table header and the popover fallback paint it too, so they cover rows that scroll under them. |
| `--lui-fg` | gray 12: `#1c2024` / `#edeef0` | Body text, headings, tab titles, sorted column header, the dialog and flash text. |
| `--lui-muted` | gray 11: `#60646c` / `#b0b4ba` | Secondary text: notes, table headers, the wizard's step list and legends, streamed placeholders. |
| `--lui-line` | gray 6: `#d9d9e0` / `#363a3f` | Every 1px rule: dialogs, popovers, cards, accordion and wizard fieldsets, table rules, the pager list. |
| `--lui-surface` | gray 2: `#f9f9fb` / `#18191b` | Quiet raised areas: `<code>`, open accordion panels, streamed slots. |
| `--lui-card` | gray 1 / gray 2 | Cards and stat tiles. |
| `--lui-popover` | gray 1 / gray 2 | Floating layers: dialogs, drawers, popovers, menus, the command palette, toasts. |
| `--lui-secondary` | gray 3: `#f0f0f3` / `#212225` | Secondary buttons, the tab list, chips and badges, skeleton blocks. |
| `--lui-accent` | gray 3 / gray 4 (`#272a2d`) | The hover and highlighted surface: menu items, ghost buttons, table rows, the active palette entry. |
| `--lui-on-accent` | gray 12 | Text on the accent surface. |
| `--lui-primary` | brand 9: `#3e63dd` | Primary buttons, the current page, the current wizard step, the range slider, checked controls. |
| `--lui-on-primary` | `#ffffff` | Text on the primary colour. |
| `--lui-input` | gray 7: `#cdced6` / `#43484e` | Borders of inputs, selects, textareas, checkboxes and outline buttons. |
| `--lui-ring` | brand 8: `#8da4ef` / `#435db1` | The focus ring: 3px at 50% opacity, plus the focused control's border at full strength. |
| `--lui-link` | brand 11: `#3a5bc7` / `#9eb1ff` | Link text: the brand's text step, since step 9 is too dark to read on the dark background. |
| `--lui-danger` | `#ce2c31` / `#ff9592` | Validation messages and `aria-invalid` / `:user-invalid` rings, danger buttons, danger flashes. |
| `--lui-on-danger` | gray 1 | Text on danger buttons and badges. |
| `--lui-ok` | `#1f7d53` / `#3dd68c` | Ok flashes; free for your own success states. |
| `--lui-warn` | `#9c5b00` / `#ffca16` | Warning flashes. |
| `--lui-info` | `#0d74ce` / `#70b8ff` | Informational callouts and badges (Radix blue step 11). |
| `--lui-on-info` | gray 1 | Text on an info fill. |

### Type, size, shape and space

| Token | Default | What it affects |
|---|---|---|
| `--lui-font-sans`, `--lui-font-mono`, `--lui-font-serif` | system stacks (`ui-sans-serif`, `ui-monospace`, `ui-serif` and fallbacks) | Body and controls; `<code>`, `<kbd>` and snippets; long-form text for an app that wants it (no component uses serif). No web font is loaded. |
| `--lui-text-1` … `-9` | 12, 14, 16, 18, 20, 24, 28, 35, 60px (in rem) | Radix Themes' type ramp: step 2 controls, tables and menus; step 3 body text; `h3` step 4, `h2` step 6, `h1` step 8. |
| `--lui-leading-1` … `-9` | 16, 20, 24, 26, 28, 30, 36, 40, 60px | The line height that goes with each step. |
| `--lui-size-1` … `-3` | 2rem, 2.25rem, 2.5rem | Control heights: small, default and large buttons, fields and toggles. Under `@media (pointer: coarse)` all three become 2.75rem (44px). |
| `--lui-hit` | 2.25rem (2.75rem on a coarse pointer) | The least hit area of a small inline target (a toast's ×, a menu item). |
| `--lui-radius-factor` | `1` | Every radius step is multiplied by it: `0` squares every corner, `1.5` rounds more. |
| `--lui-radius-none`, `-1`, `-2`, `-3`, `-full` | 0, base − 2px, base, base + 4px, 9999px | `-1` buttons, inputs, chips and `<code>`; `-2` dialogs and popovers; `-3` cards, sheets and large panels; `-full` pills. The base is `Radius::base` (`0.5rem`). |
| `--lui-space` | `8px` | The unit every gap, margin and padding is a multiple of; `Density::Compact` makes it `calc(8px * 0.85)`. |
| `--lui-space-1` … `-16` | 4px steps | Steps 1, 2, 3, 4, 6, 8, 10, 12 and 16 are that many halves of the unit (Radix's nine spacing values): the `.gap(n)` of `ui.stack`, `ui.cluster`, `ui.grid` and `ui.split`. |

### Depth, motion and stacking

| Token | Default | What it affects |
|---|---|---|
| `--lui-shadow-xs` … `-lg` | stacked, darker in the dark scheme | A tight contact shadow over soft ambient ones; one geometry for both schemes, the shadow colours in `light-dark()`. `xs` under controls, `sm` under cards and stat tiles, `md` under popovers, menus and pickers, `lg` under dialogs, drawers, the palette and toasts. |
| `--lui-highlight` | inset top edge, white at 60% / 7% | Added to a raised surface's `box-shadow` so it reads as lit from above: primary and danger buttons, cards, floating layers. |
| `--lui-gradient-primary` | brand 9 → 11 / 9 → 8, `in oklch` | Primary fills, as `background-image` over `background-color: var(--lui-primary)`, so Chrome before 111 keeps the flat fill. White text clears 4.5:1 over both stops (tested). |
| `--lui-gradient-ring` | brand 11 → 8 | The 1px border of a focused outline button, inside `@supports`. |
| `--lui-shimmer`, `--lui-shimmer-surface`, `--lui-glow`, `--lui-beam` | `layout::EFFECTS` | Not `Tokens` fields: the opt-in effects (`.shimmer()`, `.glow()`, `.beam()`), each colour in `light-dark()`. |
| `--lui-shimmer-duration`, `--lui-beam-duration`, `--lui-marquee-duration` | `2.5s`, `6s`, `40s` | Not `Tokens` fields: one sweep, one lap of the beam, one loop of `ui.marquee`. |
| `--lui-duration-fast` / `--lui-duration` / `--lui-duration-slow` | `150ms` / `200ms` / `250ms` | How long menus, dialogs, toasts and sheets take to come and go. `prefers-reduced-motion: reduce` zeroes them (and every other transition and animation). |
| `--lui-ease-out` | `cubic-bezier(0.16, 1, 0.3, 1)` | The curve of fades and of the sheet's slide. |
| `--lui-ease-spring` | `linear(…)`, else `ease-out` | Not a field: a damped spring (about 5% overshoot) for the scale and rise of dialogs, menus and toasts, under `@supports (transition-timing-function: linear(0, 1))`. |
| `--lui-z-sticky` | `2` | Sticky table headers and columns. |
| `--lui-z-drawer`, `--lui-z-dialog`, `--lui-z-popover` | `10` | The drawer panel; a dialog or menu drawn without the top layer (their fallbacks). |
| `--lui-z-toast` | `20` | The toast stack. |
| `--lui-z-tooltip` | `30` | Tooltips, above everything else. |
| `--lui-overlay` | black at 50% | Not a field: the backdrop behind dialogs and drawers. |
| `--lui-busy` | `0.6` | Not a field: the opacity of a swap root or form while the enhancement script has a request in flight (`[data-lui-busy]`). Set it to `1` to turn the fade off. |

A test (`no_literal_font_size_or_z_outside_tokens`) holds component CSS to `var(--lui-text-n)`
and `var(--lui-z-*)`; the files that still write literals are listed in it until the sweep
that follows M39 (ROADMAP).

## A different palette

This one is "linen and copper": warm paper, near-black text, a copper primary that turns to
amber in the dark scheme. It sets both palettes and the radius and keeps the rest.

```rust
use std::borrow::Cow::Borrowed as B;
use loco_ui::prelude::*;
use loco_ui::layout::{Palette, Radius, Tokens};

const LINEN: Tokens = Tokens {
    light: Palette {
        bg: B("#f4efe6"), fg: B("#1d1a17"), muted: B("#5d574f"), line: B("#d6cdbf"),
        surface: B("#fffdf9"), card: B("#fffdf9"), popover: B("#fffdf9"),
        secondary: B("#ebe3d6"), accent: B("#ebe3d6"), on_accent: B("#1d1a17"),
        primary: B("#8a3b12"), on_primary: B("#ffffff"), input: B("#d6cdbf"),
        ring: B("#b5764f"), link: B("#8a3b12"), danger: B("#a0261c"), on_danger: B("#ffffff"),
        ok: B("#2f6b3a"), warn: B("#7a5500"), info: B("#245f8f"), on_info: B("#ffffff"),
    },
    dark: Palette {
        bg: B("#161311"), fg: B("#ece6dc"), muted: B("#a59c90"), line: B("#3a332c"),
        surface: B("#1f1b18"), card: B("#1f1b18"), popover: B("#1f1b18"),
        secondary: B("#2b2521"), accent: B("#2b2521"), on_accent: B("#ece6dc"),
        primary: B("#e8965a"), on_primary: B("#1a0f06"), input: B("#4a4038"),
        ring: B("#a8683a"), link: B("#e8965a"), danger: B("#ff8f85"), on_danger: B("#1a0f06"),
        ok: B("#8fd39a"), warn: B("#f0c060"), info: B("#8cc4f0"), on_info: B("#1a0f06"),
    },
    radius: Radius::new("3px"),
    ..Tokens::DEFAULT
};

let page = Ui::default().page("Hello", html! { p { "Warm." } }).tokens(&LINEN);
assert!(page.into_string().contains("--lui-bg: light-dark(#f4efe6, #161311);"));
```

Its ratios: `fg`/`bg` 15.1 and 14.9, `muted`/`bg` 6.2 and 6.8, `primary`/`bg` 6.8 and 7.9,
`on-primary`/`primary` 7.7 and 8.0, `ok`/`bg` 5.6 and 10.5. Every text pair clears 4.5. The
demo shows the pair: `/` is the default, `/?palette=linen` is this one.

`.tokens(..)` puts a second `<style class="lui-tokens">` right after the stylesheet, in the
`lui.tokens` layer; it declares every
token again, so it wins by source order and nothing else changes.

## One look for every page

`.tokens(..)` themes one page. An app wants every page themed, including those it does not
write: the account pages `cargo lui auth` writes and the 404 fallback. `layout::Look` holds
the tokens, any extra stylesheets and the header shown above `<main>`; `Ui::look(&LOOK)` makes
every `ui.page(..)` of that request apply them, and with the `axum` feature one layer does it
for every route, since the `Ui` extractor reads the look from the request extensions:

```rust,ignore
use loco_ui::layout::{Look, Radius, Tokens};

static LOOK: Look = Look {
    tokens: Tokens { radius: Radius::new("0.75rem"), ..Tokens::DEFAULT },
    css: &[".notes-article { max-width: 68ch; }"],
    header: None,
};

let router = router.fallback(loco_ui::blocks::not_found).layer(LOOK.layer());
```

`Look::css` is unlayered, so it beats the library (see the top of this page). `header: None`
shows nothing above `<main>`: since M39 the library has no header of its own, and the demo's
"loco-ui" header is its own `Look` (`demo/src/site.rs`). `Page::header(markup)` sets one for a
single page.

Tokens that change per request, such as a tenant's palette read from a database, go through
`Ui::tokens(tokens)`: every page of that `Ui` renders under them, over the look's. A page that
calls `.tokens(..)` itself still wins for that page. `examples/loco-app` installs its look in
`after_routes`.

## A cacheable, strict stylesheet

By default each page inlines the stylesheet in a `<style>`, which needs
`style-src 'unsafe-inline'`. `Page::linked_stylesheet()` writes
`<link rel="stylesheet" href="/lui.css?v=<hash>">` instead; `enhance::router()` (and so
`loco::Initializer`) serves `/lui.css` with an `ETag`, `Cache-Control: immutable` and a 304 on
`If-None-Match`. The page's remaining inline styles (its token overrides and the app's CSS)
are answered by hash: `enhance::csp` then sends
`style-src 'self' 'sha256-…'; style-src-attr 'unsafe-inline'`, dropping `'unsafe-inline'` for
`<style>` elements (`style=""` attributes, which components use for computed widths, keep it
through `style-src-attr`). `Page::style_hashes()` and `enhance::csp_hash(text)` give the
hashes for a policy of your own. The demo serves any page this way with `?css=linked`.

## Only the CSS a page uses

`loco_ui::stylesheet_for(&[Component::Dialog, Component::Table])` is the stylesheet with only
those components' CSS, plus the layout, the primitives every component builds on
(`Component::BASE`: button, input, badge, card, icon and the four layouts) and the modes, in
the same layers. `Page::subset(&[..])` inlines that instead of the whole sheet. A component
left out renders unstyled, so list every one the page shows. Each subset is computed once and
kept. The base alone is about 32 KB (7 KB gzipped) against 111 KB (19.5 KB) for everything.

## Modes

`layout::MODES`, the last layer, adjusts the components to the reader's settings:

- `@media (forced-colors: active)`: checkboxes, radios, switches, range sliders, progress and
  meter bars are drawn with `appearance: none`, so forced colours would erase them; these rules
  repaint them in system colours (`ButtonText`, `Highlight`, `Canvas`), and badges keep a
  border.
- `@media (prefers-contrast: more)`: lines and input borders a step darker, muted text at full
  strength, a 3px solid focus ring.
- `@media print`: the skip link, drawers, toasts, the sidebar, the command palette and the
  capability beacons are hidden, and sticky table headers print in place.

A media query cannot be forced from the server, so the classes `.lui-forced` and
`.lui-contrast` emulate the first two on a subtree; the demo's `/modes` page shows the stage
of controls as drawn and under each (`?forced=1`, `?contrast=1`).

## The skip link and focus

Every page starts with `<a class="lui-skip" href="#main">Skip to content</a>` (the words are
`Text::SkipToContent`, translated with the rest), hidden until it takes the focus, and the
content is `<main id="main" tabindex="-1">`. With the enhancement script, a swap that replaces
the focused element moves the focus to the new root's `h1` (or the root), and a swap that
changes the URL or title announces the new `document.title` through a `role=status` region
the script owns.

## Container breakpoints

Since M34 a component lays itself out by the width of its own box, not the window's: its root
declares `container: lui-<name> / inline-size` and its CSS asks `@container lui-<name>
(min-width: …)`, so the same table or form looks right in a sidebar, a card or a full page.
There are two breakpoints, both in `rem` so they follow the reader's font size:

| Width of the component's own box | What changes |
|---|---|
| under `30rem` (narrow) | the base CSS: one column, labels over fields, a table's priority-2 columns folded under the first cell, the kanban's snapping columns, the compact pager |
| from `30rem` | side by side where two fit: form labels beside fields, priority-2 columns shown, stat tiles in a row |
| from `48rem` (wide) | everything: priority-3 columns, the wizard's full step row, the kanban's shared-width columns |

The narrow form is the base, so a browser without container queries (Chrome before 105, Firefox
before 110, Safari before 16, and Blitz) gets it. Only the page frame and what covers it (the
layout, dialog, drawer, toasts and palette) ask the viewport; the rest either answer to their
container or wrap by their own content. The README matrix says which, per component, and a
test keeps that column in step with each file's CSS. There are no breakpoint tokens: a custom
property cannot be used inside a container query.

## Contrast requirements

Pairs that carry text must reach WCAG AA (4.5:1 for body text, 3:1 for large text and for
focus indicators). The pairs to check, with the default palette's ratios:

| Pair | Where it shows | Light | Dark | Needs |
|---|---|---|---|---|
| `fg` on `bg` | body text | 16.0 | 16.2 | 4.5 |
| `fg` on `popover` | text in dialogs, popovers, menus | 16.0 | 15.2 | 4.5 |
| `muted` on `bg` | notes, table headers | 5.8 | 9.1 | 4.5 |
| `muted` on `secondary` | inactive tabs in the tab list | 5.2 | 7.6 | 4.5 |
| `link` on `bg` | links | 5.9 | 9.1 | 4.5 |
| `on-primary` on `primary` | primary buttons, current page (both gradient stops too) | 5.2 | 5.2 | 4.5 |
| `on-accent` on `accent` | hovered menu items and rows | 14.4 | 12.4 | 4.5 |
| `danger` on `bg` | validation messages | 5.1 | 9.0 | 4.5 |
| `on-danger` on `danger` | danger buttons and badges | 5.1 | 9.0 | 4.5 |
| `ok` on `bg` | success text | 5.0 | 10.1 | 4.5 |
| `warn` on `bg` | warning text | 5.2 | 12.3 | 4.5 |
| `ring` on `bg` | focus ring | 2.4 | 3.1 | 3 |
| `line`, `input` on `bg` | borders | 1.4–1.5 | 1.6–2.0 | none: borders are not the only cue |

Every text pair clears 4.5 in both schemes, and a test in `layout.rs` keeps it so. One pair
sits under the line on purpose: the light ring (2.4), brand step 8 as Radix draws it. The ring
is never the only focus cue: the focused control's border turns `ring` too, and buttons, which
have no border change, get the ring on top of the pressed surface. If you need strict AA for
focus, set `ring` to `var(--lui-brand-9)` (5.1 on the light background).

`muted` is the one most palettes get wrong: a grey that reads fine on white drops under 4.5
on a tinted background. `on-primary` is the second: a mid-tone primary has no colour that
contrasts with it on both sides, so pick the primary dark (light scheme) or light (dark
scheme), never in the middle. The ratios above come from the standard relative-luminance
formula; any contrast checker gives the same numbers.

## Beyond the tokens

The look of buttons and native form controls lives in one place each, `button.rs` and
`input.rs`, and every component draws its buttons and fields through them, so one override
reaches every dialog, table and form. A component's parts are addressable by class,
`lui-<component>` on the root and `lui-<component>-<part>` inside, so overriding a single part
is one unlayered selector in `Look::css`.
