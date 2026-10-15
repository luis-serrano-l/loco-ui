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

## Keyboard of `Accordion`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the section headers. | no |
| `Enter` | Opens or closes the focused section. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-accordion-item` | `--lui-line` |
| `.lui-accordion-trigger` | `--lui-text-2`, `--lui-muted`, `--lui-duration`, `--lui-ease-out` |
| `.lui-accordion-title` |  |
| `.lui-accordion-icon` |  |
| `.lui-accordion-summary` | `--lui-text-2`, `--lui-muted` |
| `.lui-accordion-body` | `--lui-text-2` |
| `.lui-accordion-controls` | `--lui-space`, `--lui-text-2`, `--lui-line` |
| `.lui-accordion` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `hidden` | 1 | 1 | 1 |

- Fallback: none needed: every toggle, expand and collapse is a link
- Needs script: no
- Responsive by: content
- Look after: shadcn Accordion

## Documentation

Stacked disclosure sections, no script. Exclusive by default (one open at a time), or
`multiple` so several stay open, with "Expand all" and "Collapse all" links. An item can
carry a summary line under its title and an icon before it, and a body can hold another
accordion.

**Platform features:** links and the `hidden` attribute (baseline). The markup follows
Radix Accordion: a `.lui-accordion-item` per section holding a `.lui-accordion-trigger`
link and a `.lui-accordion-body`, each with `data-state="open|closed"`.

**Accessibility:** each title is a link to its own state, with `aria-expanded` and
`aria-controls` naming its body; icons are `aria-hidden`. Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** arrow keys between titles (the WAI-ARIA accordion
pattern). The body does not animate its height: it is shown by the server's answer.

**Fallback:** none needed; the markup is the same everywhere. Before M38 each title was a
link inside a `<summary>`, a link inside the summary's implicit button, which is not
conforming. Every click already went to the server, so the server alone decides what is open.

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
assert!(html.contains(r#"href="/help?open.faq=2" data-state="open" aria-expanded="true""#), "open item's link removes itself from the list");
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
