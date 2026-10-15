# Indicator

`loco_ui::indicator` · demo: `/spinner`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Indicator` | `ui.indicator(count: u64)` | beta |

## Props of `Indicator`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | What the badge sits on: a button, an avatar, a link. |
| `max` | number | `max: u64` |  |  | Above this count, show `max+` (`99+`). |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `danger` |  | The badge's colour. |
| `label` | value | `label: &'a str` | `{n} new` |  | What a screen reader says instead of the count in words. |
| `dot` | switch |  | `off` |  | A dot instead of the number. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-indicator` |  |
| `.lui-indicator-badge` |  |
| `.lui-indicator-dot` | `--lui-danger`, `--lui-indicator-tone`, `--lui-bg`, `--lui-primary`, `--lui-ok`, `--lui-warn`, `--lui-gray-9` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `inset-inline-end` | 87 | 63 | 14.1 |
| `translate` | 104 | 72 | 14.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: Origin UI notification badge

## Documentation

A count or a dot pinned to the corner of what it wraps: unread messages on an inbox
button, a status dot on an avatar, new items on a tab. After Origin UI's notification
badges: a solid pill 1.25rem tall in the danger colour (or any tone) on the top end corner,
ringed in the page colour so it reads against the thing under it; "99+" past a maximum.

**Platform features:** `position: absolute` with `inset-inline-end` (Chrome 87, Firefox 63,
Safari 14.1), so the badge moves to the left corner in a right-to-left page;
`translate` (Chrome 104, Firefox 72, Safari 14.1) centres it on the corner.

**Accessibility:** the visible number is `aria-hidden` and a visually hidden text says the
count in words ("3 new", in the visitor's language, or `.label(..)`), so a screen reader
reads "Inbox, 3 new" on a wrapped button. A count of 0 shows nothing. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing; the count is the server's, as
of this page (a swap root around it updates it in place with the script).

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.indicator(3).body(html! { (ui.button("Inbox")) }).render().into_string();
assert!(m.starts_with(r#"<span class="lui-indicator"><button type="submit" class="lui-button">Inbox</button>"#));
assert!(m.contains(r#"<span class="lui-indicator-badge" aria-hidden="true">"#) && m.contains(r#"<span class="lui-sr">3 new</span>"#));
// The same in `lui!`, where the block is what it wraps:
let same = lui! { Indicator(3) { Button("Inbox"); } };
assert_eq!(same.into_string(), m);
// A brand dot, and a capped count.
let dot = ui.indicator(1).dot().tone(Tone::Brand).body(html! { (ui.avatar("Ada Lovelace")) });
let dot = dot.render().into_string();
assert!(dot.contains(r#"<span class="lui-indicator-dot" data-tone="brand" aria-hidden="true"></span>"#));
assert_eq!(lui! { Indicator(1) dot tone="brand" { Avatar("Ada Lovelace"); } }.into_string(), dot);
let many = ui.indicator(120).max(99).body(html! { "Mail" }).render().into_string();
assert!(many.contains(">99+</span>") && many.contains("120 new"));
assert_eq!(lui! { Indicator(120) max=99 { "Mail" } }.into_string(), many);
```
