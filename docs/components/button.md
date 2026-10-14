# Button

`loco_ui::button` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Button` | `ui.button(text: &str)` | stable |
| `Button` | `ui.link_button(text: &str, href: &str)` | stable |

## Props of `Button`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `primary` | switch |  | `off` |  | The main action of a form or page. |
| `danger` | switch |  | `off` |  | Destroys or removes something. |
| `ghost` | switch |  | `off` |  | No border or fill until hovered. |
| `soft` | switch |  | `off` |  | A quiet tinted fill in the brand colour, for a secondary action. |
| `surface` | switch |  | `off` |  | A pale brand fill inside a brand border. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | 1 (2rem tall), 2 (2.25rem) or 3 (2.5rem); 2.75rem each on a touch screen. |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `neutral` |  | The colour; a tone other than neutral makes the button solid unless a variant is given. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `outline` |  | How the tone is painted: solid, soft, surface, outline or ghost. |
| `radius` | choice: none, small, medium, large, full | `radius: Radius` | `small` |  | The corners, from square to a pill. |
| `small` | switch |  | `off` |  | Size 1: 2rem tall instead of 2.25rem. |
| `icon_only` | switch |  | `off` |  | Square, for a glyph or an icon. |
| `submit` | switch |  | `off` | `type` | `type="submit"`, the default unless a command or popover target is set. |
| `reset` | switch |  | `off` | `type` | `type="reset"`. |
| `command` | value | `command: &'a str, target: &'a str` |  | `command` | An invoker command (`command`, `commandfor`). |
| `popovertarget` | value | `id: &'a str` |  | `popovertarget` | Toggle the popover with this id (`popovertarget`). |
| `form` | value | `id: &'a str` |  | `form` | Submit the form with this id, wherever the button sits in the page (`form=`). |
| `name` | value | `name: &'a str` |  | `name` | Sent as `name=value` with the form when this button submits it. |
| `value` | value | `value: &'a str` |  | `value` | The submitted `value` that goes with `name`. |
| `aria_label` | value | `label: &'a str` |  | `aria-label` | `aria-label`. |
| `class` | value | `class: &'a str` |  | `class` | One more class after the button's own, for a component's part name. |
| `disabled` | switch |  | `off` | `disabled` | Greyed out and not clickable. |
| `loading` | condition | `on: bool` | `off` |  | A spinner before the text, `disabled` and `aria-busy`, while `on`. |
| `body` | value | `markup: Markup` |  |  | Markup to show instead of the text. |
| `id` | value | `id: &'a str` |  | `id` | The element's `id`. |
| `role` | value | `role: &'a str` |  | `role` | `role`, for a button that is a menu item or a tab, or a link that acts as a button. |
| `title` | value | `title: &'a str` |  | `title` | `title`. |
| `style` | value | `style: impl Into<String>` |  | `style` | Inline `style`, for a per-element custom property or anchor name. |
| `aria_haspopup` | value | `kind: &'a str` |  | `aria-haspopup` | `aria-haspopup` (`"menu"`, `"dialog"`). |
| `pressed` | condition | `on: bool` | `off` | `aria-pressed` | `aria-pressed`, for a toggle button. |
| `accesskey` | value | `key: &'a str` |  | `accesskey` | `accesskey`. |
| `aria_keyshortcuts` | value | `keys: &'a str` |  | `aria-keyshortcuts` | `aria-keyshortcuts`, the shortcut spelled out for assistive technology. |
| `formmethod` | value | `method: &'a str` |  | `formmethod` | `formmethod`. |
| `formaction` | value | `action: &'a str` |  | `formaction` | `formaction`. |
| `formnovalidate` | switch |  | `off` | `formnovalidate` | `formnovalidate`. |
| `rel` | value | `rel: &'a str` |  | `rel` | `rel` of a link (`"prev"`, `"next"`). |
| `current` | condition | `on: bool` | `off` | `aria-current` | `aria-current="page"`. |
| `shimmer` | switch |  | `off` |  | A light sweeps across the button, over its own background. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `invoker commands` | 135 | 144 | 26.2 |
| `popovertarget` | 114 | 125 | 17 |
| `aria-busy` | 1 | 1 | 1 |
| `prefers-reduced-motion` | 74 | 63 | 10.1 |
| `translate` | 104 | 72 | 14.1 |

- Fallback: a popover command becomes popovertarget; other commands need the component's own fallback; a shimmer button is at rest without translate
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Button

## Documentation

The one button every other component builds on, drawn after Radix Themes' Button and
IconButton: a neutral outline by default, then any `Tone` in any
`Variant` (`.tone(..)`, `.variant(..)`), with switches for the
common ones: solid (`.primary()` with the brand gradient, `.danger()`), `.soft()`,
`.surface()` and `.ghost()`, which compose (`.danger().ghost()` is a red ghost button);
three sizes (`.size(1..=3)`, `.small()` for 1); `.radius(..)`; a square icon button. Pressed, busy and disabled each have their own look: a busy
button keeps its width and shows a spinner over its label, a disabled one turns gray.
`ui.link_button` gives a link the same look, for actions that are a navigation (GET) rather
than a post.

**Platform features:** `<button>` with its `type`, `name`/`value` (sent with the form that
submits it), `form=` (submit a form elsewhere on the page), invoker commands
(`command`/`commandfor`, Chrome 135, Firefox 144, Safari 26.2) and `popovertarget`
(Chrome 114, Firefox 125, Safari 17). A loading button is `disabled` and `aria-busy`, with a
CSS-only spinner that slows down under `prefers-reduced-motion`. `.shimmer()` sweeps a light
across the button: an `::after` layer over the button's own background, moved with the
`translate` property (Chrome 104, Firefox 72, Safari 14.1) under `@supports (translate: 100%)`
and `prefers-reduced-motion: no-preference`; the colours are `--lui-shimmer` over a filled
tone and `--lui-shimmer-surface` over the others, the pace `--lui-shimmer-duration`.

**Accessibility:** a native `<button>` or `<a>` (Enter and Space, or Enter for a link); an
icon button carries `aria-label`, a busy one `aria-busy` and `aria-disabled`, a toggle
`aria-pressed`, a menu trigger `aria-haspopup`. Checked by axe-core in headless Firefox on
every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** turn itself into a loading button while its form
posts; the server sets `.loading(true)` on the page it renders (the enhancement script
marks a posting form `aria-busy` on its own).

**Fallback:** a popover command (`toggle-popover`, `show-popover`, `hide-popover`) in a
browser without invoker commands is written as `popovertarget` and `popovertargetaction`,
which popover browsers have had since 2023. Other commands (`show-modal`, `close`) have
no attribute fallback; the dialog component uses a `:target` link there instead. A
`.shimmer()` button without `translate`, or under `prefers-reduced-motion: reduce`, is the
same button at rest, with no sweep.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let save = ui.button("Save").primary().render().into_string();
assert!(save.contains(r#"class="lui-button lui-button-primary""#) && save.contains(r#"type="submit""#));
// The same in `lui!`:
let same = lui! { Button("Save") primary; };
assert_eq!(same.into_string(), save);
// A small ghost icon button that toggles a popover, and a link that looks like a button.
let more = ui.button("\u{22ef}").ghost().small().icon_only().aria_label("More").command("toggle-popover", "menu");
let more = more.render().into_string();
assert!(more.contains(r#"commandfor="menu""#) && more.contains(r#"aria-label="More""#));
let docs = ui.link_button("Read the docs", "/docs").render().into_string();
assert!(docs.starts_with("<a") && docs.contains(r#"href="/docs""#));
// The server knows the job is still running, so the page it renders says so.
let busy = ui.button("Export").loading(true).render().into_string();
assert!(busy.contains("disabled") && busy.contains(r#"aria-busy="true""#));
// Radix's quieter variants and sizes.
let soft = ui.button("Invite").soft().size(3).render().into_string();
assert!(soft.contains(r#"class="lui-button lui-button-soft lui-button-large""#));
assert_eq!(lui! { Button("Invite") soft size=3; }.into_string(), soft);
// Opt-in motion: a light sweeps across it, CSS only.
let shiny = ui.button("Upgrade").primary().shimmer().render().into_string();
assert!(shiny.contains(r#"class="lui-button lui-button-primary lui-button-shimmer""#));
assert_eq!(lui! { Button("Upgrade") primary shimmer; }.into_string(), shiny);
// Tone and variant compose; each one set is also a `data-*` attribute on the root.
let remove = ui.button("Remove").danger().ghost().radius(Radius::Full).render().into_string();
assert!(remove.contains(r#"data-tone="danger" data-variant="ghost" data-radius="full""#));
assert_eq!(lui! { Button("Remove") danger ghost radius="full"; }.into_string(), remove);
```
