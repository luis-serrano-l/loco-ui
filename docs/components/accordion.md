# Accordion

`loco_ui::accordion` · demo: `/accordion`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Accordion` | `ui.accordion(group: &str)` | stable |

## Props of `Accordion`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `item` | item | `title: &'a str, body: Markup` |  |  | A section titled `title` with its body. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon, or a glyph or emoji, before the title of the section added last, hidden from assistive tech. |
| `description` | modifier | `summary: &'a str` |  |  | A muted line under the title of the section added last, visible while it is closed. |
| `multiple` | switch |  | `off` |  | Several sections may be open at once (`?open.<group>=0,2`). |
| `controls` | switch |  | `off` |  | "Expand all" and "Collapse all" links above the sections (with `multiple`). |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<details name` | 120 | 130 | 17.2 |
| `::details-content` | 131 | 143 | 18.4 |
| `interpolate-size` | 129 | no | no |

- Fallback: plain <details>: no exclusivity, no animation; expand/collapse and every toggle are links either way
- Needs script: no
- Responsive by: content
- Look after: shadcn Accordion

## Documentation

Stacked disclosure sections, no script. Exclusive by default (one open at a time), or
`multiple` so several stay open, with "Expand all" and "Collapse all" links. An item can
carry a summary line under its title and an icon before it, and a body can hold another
accordion.

**Platform features:** `<details name="group">` (baseline 2024) for exclusivity.
`::details-content` (Chrome 131, Firefox 143, Safari 18.4) plus
`interpolate-size: allow-keywords` (Chrome 129 only) animate the height between `0` and
`auto`; without `interpolate-size` the panel snaps.

**Accessibility:** `<details>`/`<summary>`: Enter or Space opens a section, each title is a
link to its state; icons are `aria-hidden`. Checked by axe-core in headless Firefox on every
demo route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** arrow keys between summaries (the WAI-ARIA accordion
pattern).

**Fallback:** `<details>` alone (baseline 2020) still toggles; only the exclusivity and the
animation are lost. No `Caps` branch is needed; the markup is the same everywhere.

**Server persistence:** the open sections come from the request's `?open.<group>=0,2` (a
comma list, see `crate::UiState::opens`) and each title is a link that toggles its own index in
that list, so the choice survives navigation. "Expand all" links to every index, "Collapse
all" to `open.<group>=` (which removes the key). The group is a swap root for the
`crate::enhance` script. A nested accordion is its own group with its own key.

**Without script:** nothing is lost.

```rust
use loco_ui::prelude::*;
// Items 0 and 2 of "faq" were left open, as the URL records it.
let ui = Ui::from_request("/help", "open.faq=0,2", "");
// `icon` and `description` apply to the item added last.
let m = ui.accordion("faq")
    .item("Install", html! { p { "cargo add" } }).icon("\u{1F4E6}").description("One line.")
    .item("Use", html! { p { "html!" } })
    .item("More", html! { (ui.accordion("faq-more").item("Nested", html! { p { "Own group." } })) })
    .multiple()
    .controls();
let html = m.render().into_string();
assert!(html.contains("href=\"/help?open.faq=2\">Install"), "open item's link removes itself from the list");
assert!(html.contains("href=\"/help?open.faq=0%2C1%2C2\">Expand all"));
assert!(html.contains("class=\"lui-accordion-summary\">One line."));
// The same in `lui!`:
let same = lui! { Accordion("faq") multiple controls {
    item "Install" icon="\u{1F4E6}" description="One line." { p { "cargo add" } }
    item "Use" { p { "html!" } }
    item "More" { (lui! { Accordion("faq-more") { item "Nested" { p { "Own group." } } } }) }
} };
assert_eq!(same.into_string(), html);
```
