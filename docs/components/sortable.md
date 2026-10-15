# Sortable list

`loco_ui::sortable` · demo: `/sortable`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Sortable` | `ui.sortable(label: &str, action: &str)` | beta |

## Props of `Sortable`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `item` | item | `key: &'a str, title: &'a str` |  |  | An item, after the ones added before it. |
| `description` | modifier | `text: &'a str` |  |  | Small print under the item added last. |

## Keyboard of `Sortable`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the items' move buttons, top to bottom. | no |
| `Enter / Space` | Moves the item one place up from its up button. | no |
| `Enter / Space` | Moves the item one place down from its down button. | no |
| `Enter / Space` | Keeps focus on the moved item's button after the list updates in place. | yes |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-sortable-list` |  |
| `.lui-sortable-item` | `--lui-card`, `--lui-line`, `--lui-radius`, `--lui-shadow-sm`, `--lui-highlight` |
| `.lui-sortable-grip` | `--lui-muted` |
| `.lui-sortable-text` |  |
| `.lui-sortable-title` | `--lui-text-2` |
| `.lui-sortable-note` | `--lui-text-1`, `--lui-muted` |
| `.lui-sortable-move` |  |
| `.lui-sortable-slot` | `--lui-control-h-sm` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<form method="post">` | 1 | 1 | 1 |
| `view-transition-name` | 111 | 144 | 18 |
| `@media (scripting: enabled)` | 120 | 113 | 17 |

- Fallback: without view transitions a moved item is simply in its new place
- Needs script: partly: dragging needs the script; the named buttons move items one place at a time without it
- Responsive by: content
- Look after: Dioxus Components drag and drop list

## Documentation

A vertical list whose order the server keeps. Each item has a grip and two named buttons,
"Move … up" and "Move … down", in a form posting `item=<key>&to=<index>` (the index the
item should end at); the route saves the order and redirects back. With the enhancement
script the grip also drags: dropping posts the same form with the new index, and the list
is replaced in place.

**Platform features:** `<form method="post">` with `name`/`value` buttons, Post/Redirect/Get;
an ordered list labelled by `aria-label`; `view-transition-name` per item (Chrome 111,
Firefox 144, Safari 18) so a moved item slides; `@media (scripting: enabled)` (Chrome 120,
Firefox 113, Safari 17) for the grab cursor; with the script, HTML drag and drop (baseline
2010 on desktop, Safari 15 on iOS).

**Accessibility:** the move buttons are the keyboard and screen-reader path, named with the
item's title; the grip is decorative (`aria-hidden`). The first item has no "up" and the
last no "down"; an empty slot keeps the buttons of every row in line. On a touch screen
the buttons are 44px (`--lui-control-h-sm`).

**What it does not do without script:** dragging. Every move is a button press, one place
at a time.

**Fallback:** without view transitions a moved item is simply in its new place.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let list = ui.sortable("Tasks", "/tasks/order")
    .item("a", "Write the docs").description("Due Friday")
    .item("b", "Ship the release");
let m = list.render().into_string();
assert!(m.contains(r#"name="item" value="a""#) && m.contains(r#"name="to" value="1""#));
assert!(m.contains(r#"aria-label="Move Write the docs down""#));
assert!(!m.contains("Move Write the docs up") && !m.contains("Move Ship the release down"));

// The same in `lui!`:
let same = lui! { Sortable("Tasks", "/tasks/order") {
    item "a" "Write the docs" description="Due Friday";
    item "b" "Ship the release";
} };
assert_eq!(same.into_string(), m);
```
