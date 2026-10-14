# Tooltip

`loco_ui::tooltip` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Tooltip` | `ui.tooltip(text: &str, trigger: Markup)` | stable |

## Props of `Tooltip`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `below` | switch |  | `off` |  | Show the text under the trigger instead of above it. |
| `id` | value | `id: &'a str` |  | `id` | The tooltip's id, `lui-tooltip-<slug of the text>` by default. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `:focus-within` | 60 | 52 | 10.1 |
| `@media (hover: none)` | 41 | 64 | 9 |

- Fallback: none needed
- Needs script: partly: a delay before opening and Escape to close need script
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

**What it does not do without script:** open after a delay, close on Escape while the
pointer stays over it, or flip to fit the viewport. `popover="hint"` with hover triggers
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
assert!(below.contains("lui-tooltip-below"));
// The same in `lui!`:
let same = lui! { Tooltip("Copy the link", lui! {
    Button("") icon_only ghost aria_label="Copy" body=(html! { (Icon::Copy) });
}); };
assert_eq!(same.into_string(), m);
```
