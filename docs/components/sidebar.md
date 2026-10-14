# Sidebar

`loco_ui::sidebar` · demo: `/sidebar`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Sidebar` | `ui.sidebar(label: &str)` | beta |

## Props of `Sidebar`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `group` | item | `heading: &'a str` |  |  | Start a group of links under a small heading. |
| `link` | item | `text: &'a str, href: &'a str` |  | `href` | A link; the one to the current path is marked current. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon before the link added last. |
| `badge` | modifier | `text: impl Display` |  |  | A count after the link added last. |
| `id` | value | `id: &'a str` |  | `id` | The prefix of the group headings' ids instead of `lui-sidebar-<label>`. |
| `collapsible` | switch |  | `off` |  | A toggle that folds it to an icon rail, the labels shown as tooltips. |
| `current_prefix` | switch |  | `off` |  | Mark a link current also when the path is under it (`/projects` on `/projects/42`). |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `aria-current="page"` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: partly: collapsing to an icon rail kept between pages needs script
- Responsive by: content
- Look after: shadcn Sidebar

## Documentation

An app's navigation as a column: groups under small headings, each link with an optional
icon and a count, the link to the current path highlighted. Put it in a
`Drawer` `.sidebar()` (or `ui.app_shell`) to have it collapse to a drawer
on narrow screens, or anywhere a column fits.

**Platform features:** a `<nav>` of lists; `aria-current="page"` on the link to the current
path, compared on the server.

**Accessibility:** a `<nav>` named by its label; each group is a list labelled by its
heading; the current page's link has `aria-current="page"`; icons are decorative, counts are
text. Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

The look follows the shadcn Sidebar block: muted group labels, 2rem rows with icons and
counts, the current row on gray-4. `.collapsible()` adds a toggle that folds it to an icon
rail whose labels show as tooltips.

**What it does not do without script:** keep the rail folded between pages (each page
renders it open); wrap it in a drawer for the narrow-screen case.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/inbox", "", "");
let m = ui.sidebar("Mail").link("Inbox", "/inbox").link("Sent", "/sent").render().into_string();
assert!(m.contains(r#"<a href="/inbox" aria-current="page">"#));

let m = ui.sidebar("Mail")
    .group("Mail")
    .link("Inbox", "/inbox").icon(Icon::Mail).badge("12")
    .link("Sent", "/sent")
    .group("Labels")
    .link("Work", "/labels/work");
let html = m.render().into_string();
assert!(html.contains("lui-sidebar-badge\">12<") && html.contains(">Labels</p>"));
// The same in `lui!`:
let same = lui! { Sidebar("Mail") {
    group "Mail";
    link "Inbox" "/inbox" icon=(Icon::Mail) badge="12";
    link "Sent" "/sent";
    group "Labels";
    link "Work" "/labels/work";
} };
assert_eq!(same.into_string(), html);
// A toggle that folds it to an icon rail (where `::details-content` can be styled).
let ui = Ui::from(Caps::all());
let rail = ui.sidebar("Mail").collapsible().link("Inbox", "/inbox").icon(Icon::Mail).render().into_string();
assert!(rail.starts_with(r#"<details class="lui-sidebar-rail" open>"#));
assert_eq!(lui! { Sidebar("Mail") collapsible { link "Inbox" "/inbox" icon=(Icon::Mail); } }.into_string(), rail);
// `/projects/42` is under the link to `/projects`, which is marked current.
let ui = Ui::from_request("/projects/42", "", "");
let m = ui.sidebar("App").current_prefix().link("Home", "/").link("Projects", "/projects").render().into_string();
assert!(m.contains(r#"<a href="/projects" aria-current="page">"#) && !m.contains(r#"<a href="/" aria-current"#));
```
