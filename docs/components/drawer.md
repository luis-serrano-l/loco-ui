# Drawer

`loco_ui::drawer` · demo: `/nav`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Drawer` | `ui.drawer(label: &str)` | stable |

## Props of `Drawer`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `nav` | value | `nav: Markup` |  |  | The navigation inside the drawer, usually a `ul` of links. |
| `body` | value | `body: Markup` |  |  | The page's content, beside the drawer. |
| `id` | value | `id: &str` |  | `id` | The drawer's id instead of the label's slug. |
| `title` | value | `title: &'a str` |  |  | A heading at the top of the panel (also its accessible name). |
| `description` | value | `text: &'a str` |  |  | A line under the title, the panel's description. |
| `footer` | value | `footer: Markup` |  |  | Actions pinned to the bottom of the panel, under the scrolling navigation. |
| `sidebar` | switch |  | `off` |  | A permanent sidebar above 60rem. |
| `open` | condition | `open: bool` | `off` | `open` | Render it open (non-modal) from the server. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<dialog>` | 37 | 98 | 15.4 |
| `command="show-modal"` | 135 | 144 | 26.2 |
| `closedby` | 134 | 141 | 26 |
| `@starting-style` | 117 | 129 | 17.5 |
| `transition-behavior: allow-discrete` | 117 | 129 | 17.4 |
| `@media` | 1 | 1 | 1 |

- Fallback: link to #id and a :target rule; open from the server
- Needs script: no
- Responsive by: viewport
- Look after: shadcn Sheet and Drawer

## Documentation

Site navigation that is a sidebar on a wide screen and a drawer sliding in from the edge on
a narrow one, from the same markup: a menu button opens it, Escape, a click outside or the
close button shuts it. No script.

The look follows the shadcn Sheet: a header (title, `.description(..)`, the close ×), the
navigation scrolling in the middle and a `.footer(..)` pinned to the bottom. Under a 30rem
viewport it is the shadcn Drawer instead: the same `<dialog>` rises from the bottom with a
grab handle and rounded top corners, at most 85vh tall.

**Platform features:**
- `<dialog>` opened as a modal by an invoker button, `command="show-modal"` (Chrome 135+,
  Firefox 144+, Safari 26.2+), closed by `command="close"` and by `closedby="any"` for
  Escape and light dismiss.
- `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) for the slide in, and
  `transition-behavior: allow-discrete` on `display` and `overlay` (Chrome 117,
  Firefox 129, Safari 17.4) for the slide out, the backdrop fading with it; off under
  `prefers-reduced-motion`.
- Sidebar mode: above 60rem a `@media` rule shows the closed `<dialog>` in a grid column
  and hides the menu button, so desktop gets a permanent sidebar with no request.

**Fallback:** without `Caps` `Invokers`, the menu button is a link to `#id` and a `:target`
rule shows the drawer; its close control is a link to `#`. `?dialog=<id>` in the URL (or
`.open(true)`) renders it open from the server.

**Accessibility:** a native modal `<dialog>` named by its title: focus moves in, Escape
closes it. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** swipe to close; focus is not trapped in the
`:target` fallback.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let nav = html! { ul { li { a href="/" { "Home" } } } };
// The id is the label's slug: this drawer is `#menu`.
let m = ui.drawer("Menu").nav(nav.clone()).body(html! { p { "Page" } }).render().into_string();
assert!(m.contains(r#"command="show-modal" commandfor="menu""#) && m.contains(r#"closedby="any""#));
// `?dialog=menu` opens it from the server.
let ui = Ui::from_request("/", "dialog=menu", "");
let m = ui.drawer("Menu").title("Browse").sidebar().nav(nav.clone()).render().into_string();
assert!(m.contains("lui-drawer-sidebar") && m.contains(" open>"));
// The same in `lui!`:
let same = lui! { Drawer("Menu") title="Browse" sidebar nav=(nav); };
assert_eq!(same.into_string(), m);
// A description under the title and a footer under the navigation.
let m = ui.drawer("Menu").description("Everything in the app.").footer(html! { a href="/signout" { "Sign out" } });
let m = m.render().into_string();
assert!(m.contains(r#"aria-describedby="menu-description""#) && m.contains(r#"<div class="lui-drawer-foot">"#));
let same = lui! { Drawer("Menu") description="Everything in the app." footer={ a href="/signout" { "Sign out" } }; };
assert_eq!(same.into_string(), m);
```
