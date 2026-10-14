# Navigation menu

`loco_ui::nav_menu` · demo: `/nav-menu`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `NavMenu` | `ui.nav_menu(label: &str)` | beta |

## Props of `NavMenu`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `link` | item | `text: &'a str, href: &'a str` |  | `href` | A link in the row; the one to the current path is marked current. |
| `panel` | item | `text: &'a str, items: impl IntoIterator<Item = I>` |  |  | A button that opens a panel of `items` (`MenuItem`s or `(text, href)` links). |
| `id` | value | `id: &'a str` |  | `id` | The prefix of the panels' ids instead of `nav-<label>`. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `popover` | 114 | 125 | 17 |
| `aria-current="page"` | 1 | 1 | 1 |

- Fallback: a <details> dropdown without popover; a centred panel without anchor positioning
- Needs script: partly: opening a panel on hover needs script
- Responsive by: container
- Look after: shadcn Navigation Menu

## Documentation

A site's top navigation: a row of links, some of which open a panel of more links. Panels
are the popover menu's: they open on click, close on a click outside or
Escape, and are `<details>` dropdowns where popovers are missing.

**Platform features:** a `<nav>` of lists; the panels use `popover` and anchor positioning
as the popover menu does (see its header); `aria-current="page"` on the link to the current
path.

**Accessibility:** a `<nav>` named by its label; plain links are links, and a panel's button
says it opens a menu (`aria-haspopup`); the current page's link has `aria-current="page"`.
Checked by axe-core in headless Firefox on every demo route, both capability variants, light
and dark (no serious or critical violation).

The look follows shadcn NavigationMenu: quiet triggers, and panels that are a grid of links,
each a title over a line of description (`(text, href, description)` items), two across
in a wide panel and one in a narrow one.

**What it does not do without script:** open a panel on hover; a click opens it.

**Fallback:** that of the popover menu: a `<details>` dropdown without `popover`, a centred
panel without anchor positioning.

```rust
use loco_ui::prelude::*;
let mut ui = Ui::from_request("/pricing", "", "");
ui.caps = Caps::all();
let m = ui.nav_menu("Main").link("Pricing", "/pricing").render().into_string();
assert!(m.contains(r#"<a class="lui-nav-menu-link" href="/pricing" aria-current="page">"#));

let m = ui.nav_menu("Main")
    .panel("Products", [("Mail", "/mail"), ("Calendar", "/calendar")])
    .link("Pricing", "/pricing");
let html = m.render().into_string();
assert!(html.contains(r#"popovertarget="nav-main-products""#) && html.contains(r#"href="/calendar""#));
// The same in `lui!`:
let same = lui! { NavMenu("Main") {
    panel "Products" ([("Mail", "/mail"), ("Calendar", "/calendar")]);
    link "Pricing" "/pricing";
} };
assert_eq!(same.into_string(), html);
// Panel links with a line of description each.
let m = ui.nav_menu("Main").panel("Products", [("Mail", "/mail", "Read and send")]).render().into_string();
assert!(m.contains(r#"<span class="lui-popover-description">Read and send</span>"#));
```
