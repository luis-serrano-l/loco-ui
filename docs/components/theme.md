# Theme toggle

`loco_ui::theme`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `ThemeToggle` | `ui.theme_toggle(action: &str)` | stable |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `prefers-color-scheme` | 76 | 67 | 12.1 |
| `color-scheme` | 81 | 96 | 13 |
| `cookie` | 1 | 1 | 1 |

- Fallback: OS preference
- Needs script: no
- Responsive by: content
- Look after: Radix Themes SegmentedControl

## Documentation

Light/dark theme chosen by the user and remembered by the server.

**Platform features:** `prefers-color-scheme` (baseline 2020), `color-scheme` property,
CSS custom properties. The toggle is a `<form method="post">`; the server stores the choice
in a cookie and sets `data-theme` on `<html>`.

**Accessibility:** three native buttons with `aria-pressed` on the current theme. Checked by
axe-core in headless Firefox on every demo route, both capability variants, light and dark
(no serious or critical violation).

**What it does not do without script:** follow a change of the OS preference while a cookie
choice is set; the cookie wins until reset to auto.

**Fallback:** none needed. Without a cookie the OS preference wins. Colours are switched by
a media query and `data-theme`, so `light-dark()` support (`Caps::LightDark`) is only
reported, never required. The form is a swap root: the `crate::enhance` script applies
the new `data-theme` without a reload.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/", "", "theme=dark");
let m = ui.theme_toggle("/theme").render().into_string();
assert!(m.contains(r#"value="dark" aria-pressed="true""#));

// The handler the toggle posts to keeps the choice for a year.
let r = ui.redirect("/").theme(Theme::parse("light"));
assert!(r.set_cookies()[0].starts_with("theme=light;"));

// The same in `lui!`:
let same = lui! { ThemeToggle("/theme"); };
assert_eq!(same.into_string(), m);
```
