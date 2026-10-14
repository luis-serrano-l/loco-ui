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
| `id` | value | `id: &'a str` |  | `id` | The drawer's id instead of `lui-drawer-` and the label's slug. |
| `title` | value | `title: &'a str` |  |  | A heading at the top of the panel (also its accessible name). |
| `description` | value | `text: &'a str` |  |  | A line under the title, the panel's description. |
| `footer` | value | `footer: Markup` |  |  | Actions pinned to the bottom of the panel, under the scrolling navigation. |
| `sidebar` | switch |  | `off` |  | A permanent sidebar above 60rem. |
| `trigger` | value | `button: Button<'a>` |  |  | The opener as a styled button of the caller's; its command or link is the drawer's. |
| `no_trigger` | switch |  | `off` |  | No opener: a link to `?dialog=<id>` or a button elsewhere opens it. |
| `open` | condition | `open: bool` | `off` | `open` | Render it open (non-modal) from the server. |
| `side` | choice: top, bottom, start, end | `side: Side` | `start` |  | The edge it slides in from. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | 18, 24 or 32rem wide (tall, from the top or bottom). |

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
grab handle and rounded top corners, at most 85vh tall. `.side(..)` opens it from the end
edge, the top or the bottom instead of the start, and `.size(..)` makes it narrower or
wider (18, 24 or 32rem; the height for the top and bottom).

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
// The id is `lui-drawer-` and the label's slug: this drawer is `#lui-drawer-menu`.
let m = ui.drawer("Menu").nav(nav.clone()).body(html! { p { "Page" } }).render().into_string();
assert!(m.contains(r#"command="show-modal" commandfor="lui-drawer-menu""#) && m.contains(r#"closedby="any""#));
// `?dialog=lui-drawer-menu` opens it from the server (and, for one release, the
// `?dialog=menu` written before the prefix).
let ui = Ui::from_request("/", "dialog=lui-drawer-menu", "");
let m = ui.drawer("Menu").title("Browse").sidebar().nav(nav.clone()).render().into_string();
assert!(m.contains("lui-drawer-sidebar") && m.contains(" open>"));
// The same in `lui!`:
let same = lui! { Drawer("Menu") title="Browse" sidebar nav=(nav); };
assert_eq!(same.into_string(), m);
// A description under the title and a footer under the navigation.
let m = ui.drawer("Menu").description("Everything in the app.").footer(html! { a href="/signout" { "Sign out" } });
let m = m.render().into_string();
assert!(m.contains(r#"aria-describedby="lui-drawer-menu-description""#) && m.contains(r#"<div class="lui-drawer-foot">"#));
let same = lui! { Drawer("Menu") description="Everything in the app." footer={ a href="/signout" { "Sign out" } }; };
assert_eq!(same.into_string(), m);
// A styled opener of your own.
let m = ui.drawer("Menu").trigger(ui.button("Browse").ghost().leading(Icon::Menu)).render().into_string();
assert!(m.contains(r#"class="lui-button lui-button-ghost lui-drawer-open""#));
// From the end edge, wide.
let m = ui.drawer("Filters").side(Side::End).size(3).render().into_string();
assert!(m.contains(r#"class="lui-drawer-panel" data-side="end" data-size="3""#));
assert_eq!(lui! { Drawer("Filters") side=(Side::End) size=3; }.into_string(), m);
```
