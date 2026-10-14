# Collapsible

`loco_ui::collapsible` · demo: `/collapsible`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Collapsible` | `ui.collapsible(trigger: &str, body: Markup)` | beta |

## Props of `Collapsible`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `open` | condition | `open: bool` | `off` | `open` | Render it open. |
| `ghost` | switch |  | `off` |  | The ghost button look instead of the outline one. |
| `name` | value | `group: &'a str` |  | `name` | Collapsibles with the same `name` are exclusive: opening one closes the others. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<details>` | 12 | 49 | 6 |
| `<details name` | 120 | 130 | 17.2 |
| `::details-content` | 131 | 143 | 18.4 |

- Fallback: opens and closes at once without ::details-content
- Needs script: no
- Responsive by: -
- Look after: shadcn/ui Collapsible

## Documentation

One section that shows and hides under a button: "Show 3 more", advanced options, the
rest of a long message. The accordion's single cousin, with no group and no state in the
URL. After the shadcn/ui Collapsible.

**Platform features:** one `<details>`/`<summary>` (baseline 2020) whose summary wears the
button look; `<details name>` (baseline 2024) makes several collapsibles exclusive.
`::details-content` (Chrome 131, Firefox 143, Safari 18.4) under `@supports`, with
`interpolate-size: allow-keywords` (Chrome 129), animates the height; elsewhere it snaps.

**Accessibility:** a `<summary>` is a button with an expanded state; Enter or Space
toggles it; the chevron is `aria-hidden`. Checked by axe-core in headless Firefox on every
demo route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** without `::details-content` it opens and closes at once; without `name` the
sections are not exclusive. No `Caps` branch is needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.collapsible("Show 3 more", html! { p { "The rest." } }).render().into_string();
assert!(m.starts_with(r#"<details class="lui-collapsible"><summary class="lui-button lui-collapsible-trigger">"#));
// Open from the server, ghost, and one of a group where one opens at a time.
let m = ui.collapsible("Advanced", html! { p { "Options." } }).open(true).ghost().name("settings");
let html = m.render().into_string();
assert!(html.contains(r#"<details class="lui-collapsible" name="settings" open>"#));
// The same in `lui!`:
let same = lui! { Collapsible("Advanced", html! { p { "Options." } }) open=(true) ghost name="settings"; };
assert_eq!(same.into_string(), html);
```
