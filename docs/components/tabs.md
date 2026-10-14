# Tabs

`loco_ui::tabs` · demo: `/tabs`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Tabs` | `ui.tabs(name: &str)` | stable |

## Props of `Tabs`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `tab` | item | `title: &'a str, body: Markup` |  |  | A tab titled `title` with its panel. |
| `lazy` | item | `title: &'a str, body: impl Fn() -> Markup + 'a` |  |  | A tab whose panel is rendered only when it is the open one. |
| `badge` | modifier | `text: impl Display` |  |  | A count after the title of the tab added last. |
| `vertical` | switch |  | `off` |  | Titles in a column on the left, the open panel beside them. |
| `select_below` | switch |  | `off` |  | Under 30rem of its own width the titles give way to a `<select>`. |
| `underline` | switch |  | `off` |  | Titles on a hairline, the open one marked by a 2px bar (Radix TabNav). |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | The titles' text and padding: smaller at 1, larger at 3. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `hidden` | 1 | 1 | 1 |
| `view-transition-name` | 111 | 144 | 18 |

- Fallback: no view transitions: the chip appears on the new tab; the narrow-screen select has a Go button
- Needs script: no
- Responsive by: container
- Look after: Radix Themes Tabs and TabNav

## Documentation

Tab strip where exactly one panel is open at a time, no script. Tabs can carry a badge
count, be lazy (their body rendered only when open), run vertically, and collapse to a
`<select>` when the tabs are narrow (a container query), take an `.underline()` look after Radix
TabNav instead of the pill, and come in Radix's sizes (`.size(..)`, 1 to 3); the open tab
morphs to the next one on switch.

**Platform features:**
- Links and the `hidden` attribute (baseline): each title is a link to its own tab, every
  panel but the open one is `hidden`. The markup follows Radix Tabs: a `.lui-tabs-list` of
  `.lui-tabs-trigger` links and one `.lui-tabs-panel` per tab, each with
  `data-state="active|inactive"`.
- `view-transition-name` (Chrome 111, Firefox 144, Safari 18) on the open tab's chip
  (an empty `.lui-tabs-mark`, never the title, so no text moves): the raised chip slides to
  the new tab, across documents through the layout's `@view-transition` rule and in place
  with the enhancement script. `view-transition-class: lui-tabs-mark` (Chrome 125,
  Firefox 144, Safari 18.2) gives the slide the `--lui-ease-spring` curve.

**Accessibility:** each tab is a link, the open one `aria-current="true"` (one tab stop
each, Enter follows it); the narrow select is named "Tab"; not the ARIA tablist pattern,
which needs script (FINDINGS, M29 · axe-core over every route). Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** arrow keys between tabs (the WAI-ARIA tabs
pattern); each tab is a link reached by Tab.

**Fallback:** none needed: links, a flex row and a grid work everywhere. Without
`Caps::ViewTransitions` the chip carries no transition name and simply appears on the new
tab.

**Server persistence:** the open tab comes from the request's `?tab.<name>=` (see
`crate::UiState::tab`) and each title is a link to `?tab.<name>=i`, so the choice survives
navigation (query first, cookie after). The strip is a
swap root, so the `crate::enhance` script replaces just the strip instead of the page;
it also opens the clicked tab before the request, so the switch shows on the click.
Because every switch is a request, a lazy tab (`Tabs::lazy`) costs nothing until
opened: the component calls its closure only when it is the open tab.

**Without script:** the narrow `<select>` needs its "Go" button; the script submits
it on change.

```rust
use loco_ui::prelude::*;
let mut ui = Ui::from_request("/docs", "tab.docs=1", "");
ui.caps = Caps::all();
// `badge` applies to the tab added last.
let m = ui.tabs("docs")
    .tab("Install", html! { p { "cargo add" } })
    .tab("Use", html! { p { "html!" } }).badge(3)
    .lazy("Changelog", || html! { p { "(long)" } })
    .vertical()
    .select_below();
let html = m.render().into_string();
assert!(html.contains("href=\"/docs?tab.docs=0\""));
assert!(html.contains("view-transition-name: lui-tabs-docs"));
assert!(html.contains("<select name=\"tab.docs\""));
// The same in `lui!`:
let same = lui! { Tabs("docs") vertical select_below {
    tab "Install" { p { "cargo add" } }
    tab "Use" badge=3 { p { "html!" } }
    lazy "Changelog" || { p { "(long)" } }
} };
assert_eq!(same.into_string(), html);
// Radix's underline look instead of the pill.
let line = ui.tabs("line").tab("One", html! {}).tab("Two", html! {}).underline().render().into_string();
assert!(line.contains("lui-tabs lui-tabs-underline"));
assert_eq!(lui! { Tabs("line") underline { tab "One" {} tab "Two" {} } }.into_string(), line);
// Smaller titles.
let small = ui.tabs("small").tab("One", html! {}).size(1).render().into_string();
assert!(small.contains(r#"class="lui-tabs" data-size="1""#));
assert_eq!(lui! { Tabs("small") size=1 { tab "One" {} } }.into_string(), small);
```
