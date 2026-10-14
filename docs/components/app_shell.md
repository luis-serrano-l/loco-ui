# AppShell

`loco_ui::blocks::app_shell` · demo: `/blocks/shell`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `AppShell` | `ui.app_shell(name: &str)` | beta |

## Props of `AppShell`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `group` | item | `heading: &'a str` |  |  | Start a group of links under a small heading. |
| `link` | item | `label: &'a str, href: &'a str` |  |  | A link in the sidebar; the one to the current path is marked current. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon before the link added last. |
| `badge` | modifier | `text: impl Display` |  |  | A count after the link added last. |
| `user` | value | `name: &'a str, signout: &'a str` |  |  | Who is signed in, and where the sign-out button posts. |
| `body` | value | `body: Markup` |  |  | The page's content. |

## Documentation

The frame of a signed-in app: its name and navigation in a sidebar (a drawer on narrow
screens), in groups with icons and counts as a `crate::sidebar`, who is signed in with a sign-out button at the foot, and the page's content
beside it. The link to the current path is marked `aria-current="page"`.

**Platform features:** those of `crate::drawer` (a modal `<dialog>` on narrow screens, a
plain sidebar when wide); the sign-out is a form that posts.

**Accessibility:** the navigation is a `<nav>` inside the drawer, named by the app's name;
the current page's link has `aria-current="page"`; sign-out is a real button. Checked by
axe-core with the demo routes.

**What it does not do without script:** nothing is missing.

**Fallback:** that of the drawer: without invoker commands the menu opens through
`?dialog=`.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/", "", "");
let m = ui.app_shell("Acme").link("Dashboard", "/").user("Ada Lovelace", "/signout")
    .body(html! { h1 { "Hello" } }).render().into_string();
assert!(m.contains("Ada Lovelace") && m.contains(r#"action="/signout""#));

// Groups under small headings, an icon and a count on a link, as in [`crate::sidebar`]:
let m = ui.app_shell("Acme")
    .link("Inbox", "/").icon(Icon::Mail).badge(3)
    .group("Projects").link("Apollo", "/p/apollo").badge(12)
    .user("Ada Lovelace", "/signout").body(html! { h1 { "Hello" } });
let html = m.render().into_string();
assert!(html.contains(">Projects</p>") && html.contains("lui-sidebar-badge\">12<"));
// The same in `lui!`:
let same = lui! { AppShell("Acme") user=("Ada Lovelace", "/signout") {
    link "Inbox" "/" icon=(Icon::Mail) badge=3;
    group "Projects";
    link "Apollo" "/p/apollo" badge=12;
    body (html! { h1 { "Hello" } });
} };
assert_eq!(same.into_string(), html);
```
