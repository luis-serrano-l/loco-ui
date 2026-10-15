# Context menu

`loco_ui::context_menu` · demo: `/context-menu`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `ContextMenu` | `ui.context_menu(label: &str)` | beta |

## Props of `ContextMenu`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `link` | item | `text: &'a str, href: &'a str` |  |  | A link. |
| `action` | item | `text: &'a str, action: &'a str` |  | `action` | A `<form method="post">` button posting to `action`. |
| `group` | item | `text: &'a str` |  |  | A heading over the items after it. |
| `separator` | item |  |  |  | A rule between groups. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon, or a glyph or emoji, before the item's text (decorative, hidden from assistive tech). |
| `shortcut` | modifier | `keys: &'a str` |  |  | A shortcut shown after the item's text, as `<kbd>`. |
| `disabled` | modifier |  |  | `disabled` | The item is shown but not usable. |
| `danger` | modifier |  |  |  | The item is destructive. |
| `body` | value | `body: Markup` |  |  | The thing the actions are on. |
| `id` | value | `id: &'a str` |  | `id` | The menu's id instead of `ctx-<label>`. |

## Keyboard of `ContextMenu`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter / Space` | Opens the row's actions menu from its button. | no |
| `Tab / Shift+Tab` | Moves to the next or previous item in the open menu. | no |
| `ArrowDown / ArrowUp` | Moves to the next or previous item, wrapping at the ends. | yes |
| `Enter` | Follows the focused link or sends the focused action. | no |
| `Space` | Ticks the focused checkbox or radio item. | no |
| `Escape` | Closes the menu and returns focus to its button. Without popover support the script closes it; without the script either, its button does. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-context-menu` | `--lui-space`, `--lui-line`, `--lui-radius`, `--lui-card` |
| `.lui-context-menu-body` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `popover` | 114 | 125 | 17 |
| `popovertarget` | 114 | 125 | 17 |

- Fallback: that of the popover menu: a <details> dropdown
- Needs script: partly: opening on right-click or a long press needs script
- Responsive by: content
- Look after: Radix Themes ContextMenu

## Documentation

Actions on one thing (a file, a card, a row) from a small "more" button in its corner:
the popover menu the way a right-click menu would offer it. A right-click
cannot be caught without script, so the actions sit behind a visible button instead.

**Platform features:** those of the popover menu: `popover`, `popovertarget` and anchor
positioning, a `<details>` dropdown where popovers are missing.

**Accessibility:** the button is named "More actions" (or the label given) and says it opens
a menu; the menu is the popover menu's (`role="menu"`, Escape closes). Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no serious
or critical violation).

**What it does not do without script:** open on right-click or a long press; the button
does it.

**Fallback:** that of the popover menu.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
// The items are the popover menu's: `icon`, `shortcut`, `disabled` and `danger` apply to
// the item just added.
let m = ui.context_menu("report.pdf")
    .link("Open", "/files/1").shortcut("o")
    .separator()
    .action("Delete", "/files/1/delete").danger()
    .body(html! { p { "report.pdf" } });
let html = m.render().into_string();
assert!(html.contains(r#"aria-label="More actions: report.pdf""#) && html.contains(r#"action="/files/1/delete""#));
// The same in `lui!`:
let same = lui! { ContextMenu("report.pdf") {
    link "Open" "/files/1" shortcut="o";
    separator();
    action "Delete" "/files/1/delete" danger;
    body() { p { "report.pdf" } }
} };
assert_eq!(same.into_string(), html);
```
