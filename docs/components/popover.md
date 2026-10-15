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
| `checkbox` | item | `text: &'a str, action: &'a str, name: &'a str` |  |  | A checkbox in its own form posting `name=on` (or nothing) to `action`. |
| `radio` | item | `text: &'a str, action: &'a str, name: &'a str, value: &'a str` |  |  | A radio button in its own form posting `name=value` to `action`. |
| `checked` | modifier | `checked: bool` |  | `checked` | The checkbox or radio item is ticked. |
| `group` | item | `text: &'a str` |  |  | A section heading. |
| `separator` | item |  |  |  | A rule between groups. |
| `submenu` | item | `text: &'a str, items: impl IntoIterator<Item = I>` |  |  | A nested menu of `items` (`MenuItem`s or `(text, href)` links). |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon, or a glyph or emoji, before the item's text (decorative, hidden from assistive tech). |
| `shortcut` | modifier | `keys: &'a str` |  |  | A shortcut shown after the item's text, as `<kbd>`. |
| `disabled` | modifier |  |  | `disabled` | The item is shown but not usable. |
| `danger` | modifier |  |  |  | The item is destructive. |
| `id` | value | `id: &'a str` |  | `id` | The menu's id instead of `lui-menu-` and the label's slug. |
| `align_end` | switch |  | `off` |  | Open below the button with right edges aligned, for a button at the end of a row. |
| `open_right` | switch |  | `off` |  | Open to the right of the button. |
| `trigger` | value | `button: Button<'a>` |  |  | The opener as a styled button of the caller's; the menu sets its `popovertarget`. |

## Keyboard of `Menu`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter / Space` | Opens the menu from its button. | no |
| `Tab / Shift+Tab` | Moves to the next or previous item in the open menu. | no |
| `ArrowDown / ArrowUp` | Moves to the next or previous item, wrapping at the ends. | yes |
| `Enter` | Follows the focused link or sends the focused action. | no |
| `Space` | Ticks the focused checkbox or radio item. | no |
| `Escape` | Closes the menu and returns focus to its button. Without popover support it closes from its button instead. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-popover` | `--lui-space-1`, `--lui-popover`, `--lui-fg`, `--lui-line`, `--lui-radius`, `--lui-shadow-md`, `--lui-highlight`, `--lui-duration-fast`, `--lui-ease-out`, `--lui-duration`, `--lui-ease-spring` |
| `.lui-popover-anchored` |  |
| `.lui-popover-details` | `--lui-z-popover`, `--lui-radius-sm` |
| `.lui-popover-subnav` |  |
| `.lui-popover-check` | `--lui-primary` |
| `.lui-popover-apply` |  |
| `.lui-popover-item` | `--lui-space-2`, `--lui-text-2`, `--lui-radius-sm`, `--lui-hit`, `--lui-primary`, `--lui-on-primary` |
| `.lui-popover-icon` | `--lui-muted` |
| `.lui-popover-kbd` | `--lui-text-1`, `--lui-muted`, `--lui-space-4` |
| `.lui-popover-description` | `--lui-text-2`, `--lui-muted`, `--lui-space-2` |
| `.lui-popover-danger` | `--lui-danger` |
| `.lui-popover-heading` | `--lui-text-1`, `--lui-muted` |
| `.lui-popover-sep` | `--lui-space-1`, `--lui-line` |
| `.lui-popover-sub` |  |

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
are links, `<form method="post">` actions, checkboxes and radio buttons that post their
own form, headings, separators and submenus; each item can show an icon and a keyboard
shortcut and be disabled or destructive.

**Platform features:**
- `popover` attribute + `popovertarget` button (Chrome 114, Firefox 125, Safari 17). Light
  dismiss and top-layer stacking come for free; a submenu is a nested popover, which the
  platform keeps open with its parent.
- CSS anchor positioning `anchor-name` / `position-anchor` / `position-area`
  (Chrome 125, Firefox 147, Safari 26) to place the menu under its button, at its end, or
  to its right (`.align_end()`, `.open_right()`), with `position-try-fallbacks` (Chrome
  128, Firefox 147, Safari 26) flipping it when there is no room; the areas are the
  generic `crate::popover_card::Popover`'s.
- A checkbox or radio item (`.checkbox(..)`, `.radio(..)`) is a native control with
  `role="menuitemcheckbox"` or `menuitemradio` in its own `<form method="post">`, with an
  Apply button beside it. The enhancement script submits the form on `change`
  (`data-lui="autosubmit"`) and then hides the Apply buttons.
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

A checkbox or radio item needs its Apply button pressed without the script.

**Without script:** a shortcut shown beside an item is a label; binding the key needs
script the crate does not ship. Arrow keys move between items only with the enhancement
script; Tab always works.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
// The id is `lui-menu-` and the label's slug: this menu is `#lui-menu-account`.
let m = ui.menu("Account").link("Profile", "/profile").link("Sign out", "/logout");
assert!(m.render().into_string().contains(r#"id="lui-menu-account""#));
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
assert!(html.contains("position-area: block-end span-inline-start") && html.contains(r#"id="lui-menu-account-theme""#));
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
// A styled opener of your own: the menu sets its popover target.
let more = ui.button("").ghost().icon_only().aria_label("More").leading(Icon::Ellipsis);
let m = ui.menu("More").link("Rename", "/rename").trigger(more).render().into_string();
assert!(m.contains(r#"class="lui-button lui-button-ghost lui-button-icon""#) && m.contains(r#"popovertarget="lui-menu-more""#));
// Checkbox and radio items post their own form; `checked` applies to the item just added.
let m = ui.menu("View")
    .checkbox("Show grid", "/view", "grid").checked(true)
    .radio("Compact", "/view", "density", "compact")
    .radio("Comfortable", "/view", "density", "comfortable").checked(true);
let html = m.render().into_string();
assert!(html.contains(r#"<form method="post" action="/view" data-lui="autosubmit">"#));
assert!(html.contains(r#"<input class="lui-popover-check" type="checkbox" role="menuitemcheckbox" name="grid" value="on" checked>"#));
assert!(html.contains(r#"type="radio" role="menuitemradio" name="density" value="comfortable" checked>"#));
let same = lui! { Menu("View") {
    checkbox "Show grid" "/view" "grid" checked=(true);
    radio "Compact" "/view" "density" "compact";
    radio "Comfortable" "/view" "density" "comfortable" checked=(true);
} };
assert_eq!(same.into_string(), html);
```
