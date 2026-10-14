# Spinner

`loco_ui::spinner` · demo: `/spinner`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Spinner` | `ui.spinner()` | beta |

## Props of `Spinner`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | 12px (size 1), 16px (2, the default) or 20px (3) across. |
| `label` | value | `label: &'a str` | `Loading` |  | What a screen reader announces: what is loading. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `@keyframes` | 1 | 1 | 1 |
| `prefers-reduced-motion` | 74 | 63 | 10.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Spinner

## Documentation

A small turning ring that says something is on its way: a panel streaming in, a search
waiting on the server, a busy button (which draws this same ring). After Radix Themes
Spinner: three sizes (12, 16 and 20px), in the colour of the text around it.

**Platform features:** a CSS `@keyframes` rotation (baseline 2015) of a bordered circle with
one side clear; `prefers-reduced-motion` (Chrome 74, Firefox 63, Safari 10.1) slows it
down rather than stopping it, so it still reads as busy.

**Accessibility:** `role="status"` named "Loading" (or `.label(..)`), so a screen reader
announces it once without moving the focus. The ring inside a busy button is
`aria-hidden`, since the button says `aria-busy`. Checked by axe-core in headless Firefox
on every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** nothing is missing; it turns while it is on the
page, and the server takes it off by sending the next page (or a stream's next chunk).

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let s = ui.spinner().render().into_string();
assert_eq!(s, r#"<span class="lui-spinner" role="status" aria-label="Loading"></span>"#);
// The same in `lui!`:
let same = lui! { Spinner; };
assert_eq!(same.into_string(), s);
// Larger, saying what it waits for.
let s = ui.spinner().size(3).label("Loading results").render().into_string();
assert_eq!(s, r#"<span class="lui-spinner" role="status" aria-label="Loading results" data-size="3"></span>"#);
assert_eq!(lui! { Spinner size=3 label="Loading results"; }.into_string(), s);
```
