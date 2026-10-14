# Popover menu

`loco_ui::popover` · demo: `/popover`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Menu` | `ui.menu(label: &str)` | stable |

## Props of `Menu`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `link` | item | `text: &'a str, href: &'a str` |  |  | A link. |
| `action` | item | `text: &'a str, action: &'a str` |  | `action` | A `<form method="post">` button posting to `action`. |
| `group` | item | `text: &'a str` |  |  | A section heading. |
| `separator` | item |  |  |  | A rule between groups. |
| `submenu` | item | `text: &'a str, items: impl IntoIterator<Item = I>` |  |  | A nested menu of `items` (`MenuItem`s or `(text, href)` links). |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon, or a glyph or emoji, before the item's text (decorative, hidden from assistive tech). |
| `shortcut` | modifier | `keys: &'a str` |  |  | A shortcut shown after the item's text, as `<kbd>`. |
| `disabled` | modifier |  |  | `disabled` | The item is shown but not usable. |
| `danger` | modifier |  |  |  | The item is destructive. |
| `id` | value | `id: &str` |  | `id` | The menu's id instead of the label's slug. |
| `align_end` | switch |  | `off` |  | Open below the button with right edges aligned, for a button at the end of a row. |
| `open_right` | switch |  | `off` |  | Open to the right of the button. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `popover` | 114 | 125 | 17 |
| `anchor-name` | 125 | 147 | 26 |
| `@starting-style` | 117 | 129 | 17.5 |
| `transition-behavior: allow-discrete` | 117 | 129 | 17.4 |

- Fallback: no anchor: UA-centred popover; no popover: <details> dropdown (submenus nested); actions are plain post forms either way
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Popover and DropdownMenu

## Documentation

A dropdown menu that opens on click, closes on outside click or Escape, no script. Items
are links, `<form method="post">` actions, headings, separators and submenus; each item
can show an icon and a keyboard shortcut and be disabled or destructive.

**Platform features:**
- `popover` attribute + `popovertarget` button (Chrome 114, Firefox 125, Safari 17). Light
  dismiss and top-layer stacking come for free; a submenu is a nested popover, which the
  platform keeps open with its parent.
- CSS anchor positioning `anchor-name` / `position-anchor` / `position-area`
  (Chrome 125, Firefox 147, Safari 26) to place the menu under its button, at its end, or
  to its right (`.align_end()`, `.open_right()`).
- Motion: `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) and
  `transition-behavior: allow-discrete` on `display` and `overlay` (Chrome 117,
  Firefox 129, Safari 17.4) fade the menu in and out with a 4px drop; the `<details>`
  fallback fades in only. Older browsers show and hide it at once.

**Accessibility:** a `popover` menu: `aria-haspopup="menu"` on the trigger, `role="menu"`
and `menuitem` on the items, Escape and a click outside close it; the script adds arrow
keys. Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

**What it does not do without script:** position itself against the opener where anchor
positioning is missing; it is centred instead.

**Fallback:** without `Caps::Anchor` the popover is UA-centred, which is still usable.
Without `Caps::Popover` the menu is a `<details>` dropdown (a submenu a nested one): it
opens and closes on click but has no light dismiss.

**Without script:** a shortcut shown beside an item is a label; binding the key needs
script the crate does not ship. Arrow keys move between items only with the enhancement
script; Tab always works.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
// The id is the label's slug: this menu is `#account`.
let m = ui.menu("Account").link("Profile", "/profile").link("Sign out", "/logout");
assert!(m.render().into_string().contains(r#"id="account""#));
// `icon`, `shortcut`, `disabled` and `danger` apply to the item just added.
let m = ui.menu("Account")
    .group("Signed in as Ada")
    .link("Profile", "/profile").icon("@").shortcut("g p")
    .link("Billing", "/billing").disabled()
    .separator()
    .submenu("Theme", [("Light", "/?t=light"), ("Dark", "/?t=dark")])
    .separator()
    .action("Sign out", "/logout").danger()
    .align_end();
let html = m.render().into_string();
assert!(html.contains("<form method=\"post\" action=\"/logout\""));
assert!(html.contains("position-area: bottom span-left") && html.contains(r#"id="account-theme""#));
// The same in `lui!`:
let same = lui! { Menu("Account") align_end {
    group "Signed in as Ada";
    link "Profile" "/profile" icon="@" shortcut="g p";
    link "Billing" "/billing" disabled;
    separator();
    submenu "Theme" ([("Light", "/?t=light"), ("Dark", "/?t=dark")]);
    separator();
    action "Sign out" "/logout" danger;
} };
assert_eq!(same.into_string(), html);
```
