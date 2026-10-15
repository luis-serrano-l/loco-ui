# Tooltip

`loco_ui::tooltip` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Tooltip` | `ui.tooltip(text: &str, trigger: Markup)` | stable |

## Props of `Tooltip`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `below` | switch |  | `off` |  | Show the text under the trigger instead of above it: `.side(Side::Bottom)`. |
| `side` | choice: top, bottom, start, end | `side: Side` | `top` |  | The side of the trigger the text shows on. |
| `delay` | number | `ms: u32` | `0` |  | How long the pointer rests on the trigger before the text shows, in milliseconds. |
| `id` | value | `id: &'a str` |  | `id` | The tooltip's id, `lui-tooltip-<slug of the text>` by default. |
| `trigger` | value | `button: Button<'a>` |  |  | A button builder as the trigger, in place of the markup: it gets `aria-describedby` as an attribute. |

## Keyboard of `Tooltip`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Shows the tooltip while its trigger has focus and hides it when focus leaves. | no |
| `Escape` | Hides the tooltip until its trigger loses focus and the pointer leaves. | yes |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-tooltip` |  |
| `.lui-tooltip-text` | `--lui-z-tooltip`, `--lui-text-1`, `--lui-gray-1`, `--lui-gray-12`, `--lui-radius-sm`, `--lui-shadow-md`, `--lui-tooltip-delay` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `:focus-within` | 60 | 52 | 10.1 |
| `@media (hover: none)` | 41 | 64 | 9 |

- Fallback: none needed
- Needs script: partly: Escape to close needs script
- Responsive by: content
- Look after: Radix Themes Tooltip

## Documentation

A short label that appears beside a control while the pointer is over it or it has focus:
what an icon button does, what a shortcut is.

**Platform features:** CSS only: the text is a `role="tooltip"` element shown by
`:hover` and `:focus-within` (baseline 2020) on the wrapper, and the trigger inside names
it with `aria-describedby` so a screen reader says it too. `@media (hover: none)` (Chrome
41, Firefox 64, Safari 9) keeps it off touch screens, where hover does not exist.

**Accessibility:** `role="tooltip"` joined to its trigger by `aria-describedby`; shown on
hover and on keyboard focus. Checked by axe-core in headless Firefox on every demo route,
both capability variants, light and dark (no serious or critical violation).

`.side(..)` puts it on any side of the trigger (the same `Side` as the popover's) and
`.delay(ms)` waits before showing it on hover: a `transition-delay` on the way in, none
on the way out.

**What it does not do without script:** close on Escape (the script hides it until its
trigger loses focus and the pointer leaves), or flip to fit the viewport. `popover="hint"` with hover triggers
would do some of this, but it is Chromium-only and needs script to open on hover.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.tooltip("Copy the link", html! { (ui.button("").icon_only().ghost().aria_label("Copy").body(html! { (Icon::Copy) })) });
let m = m.render().into_string();
assert!(m.contains(r#"role="tooltip""#) && m.contains("Copy the link"));
assert!(m.contains(r#"aria-describedby="lui-tooltip-copy-the-link""#));
let below = ui.tooltip("Saved", html! { span { "3" } }).below().render().into_string();
assert!(below.contains(r#"data-side="bottom""#));
// The same in `lui!`:
let same = lui! { Tooltip("Copy the link", lui! {
    Button("") icon_only ghost aria_label="Copy" body=(html! { (Icon::Copy) });
}); };
assert_eq!(same.into_string(), m);
// A button builder as the trigger.
let copy = ui.button("").icon_only().ghost().aria_label("Copy").leading(Icon::Copy);
let m = ui.tooltip("Copy the link", html! {}).trigger(copy).render().into_string();
assert!(m.contains(r#"aria-label="Copy" aria-describedby="lui-tooltip-copy-the-link""#));
// At the end of the trigger, after half a second.
let m = ui.tooltip("Saved", html! { span { "3" } }).side(Side::End).delay(500).render().into_string();
assert!(m.starts_with(r#"<span class="lui-tooltip" data-side="end" style="--lui-tooltip-delay: 500ms">"#));
assert_eq!(lui! { Tooltip("Saved", html! { span { "3" } }) side=(Side::End) delay=500; }.into_string(), m);
```
