# Kanban

`loco_ui::kanban` · demo: `/kanban`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Kanban` | `ui.kanban(action: &str)` | stable |

## Props of `Kanban`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `column` | item | `key: &'a str, title: &'a str` |  |  | A column. |
| `limit` | modifier | `limit: usize` |  |  | A work-in-progress limit for the column added last. |
| `card` | modifier | `key: &'a str, title: &'a str` |  |  | A card in the column added last. |
| `description` | modifier | `text: &'a str` |  |  | Small print under the card added last. |
| `badge` | modifier | `text: impl Display` |  |  | A badge in the meta row of the card added last: a label, an estimate, a tag. |

## Keyboard of `Kanban`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the cards' move buttons, column by column. | no |
| `Enter / Space` | Moves the card one column left from its left arrow button. | no |
| `Enter / Space` | Moves the card one column right from its right arrow button. | no |
| `Enter / Space` | Keeps focus on the moved card's button after the board updates in place. | yes |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-kanban` |  |
| `.lui-kanban-board` |  |
| `.lui-kanban-column` | `--lui-gray-3`, `--lui-radius-lg` |
| `.lui-kanban-head` | `--lui-text-2` |
| `.lui-kanban-count` |  |
| `.lui-kanban-cards` |  |
| `.lui-kanban-card` | `--lui-card`, `--lui-line`, `--lui-radius`, `--lui-shadow-sm`, `--lui-highlight` |
| `.lui-kanban-title` | `--lui-text-2` |
| `.lui-kanban-note` | `--lui-text-1`, `--lui-muted` |
| `.lui-kanban-meta` |  |
| `.lui-kanban-move` |  |
| `.lui-kanban-empty` | `--lui-text-2`, `--lui-muted`, `--lui-gray-6`, `--lui-radius` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<form method="post">` | 1 | 1 | 1 |
| `scroll-snap-type` | 69 | 68 | 11 |
| `view-transition-name` | 111 | 144 | 18 |
| `@container` | 105 | 110 | 16 |

- Fallback: without view transitions a moved card is simply in its new column; without container queries the board keeps its narrow, snapping form
- Needs script: partly: drag and drop and reordering within a column need script
- Responsive by: container
- Look after: Dice UI Kanban

## Documentation

A board of columns and cards. Moving a card is a form post: each card carries arrow buttons
to the column on its left and on its right, posting `card=<key>&to=<column>`; the route
saves the move and redirects back. A column can have a work-in-progress limit.

**Platform features:** `<form method="post">` with `name`/`value` buttons, Post/Redirect/Get;
each column is a `<section>` with a heading and an ordered list; the board scrolls sideways
(`overflow-x: auto`, `scroll-snap-type`, Chrome 69, Firefox 68, Safari 11) with each column
at 85% of the board's width when the board is narrow, so the next one peeks; `@container`
(Chrome 105, Firefox 110, Safari 16) puts the columns side by side from 48rem; `view-transition-name` per card (Chrome 111, Firefox 144, Safari 18) so, with the
enhancement script, a moved card slides to its new column.

**Accessibility:** each column is a `<section>` labelled by its heading; every card moves
with named buttons ("Move … to …") instead of drag and drop. Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** drag and drop, or reorder cards within a column
(the server decides the order: a moved card goes last in its new column).

**Fallback:** without view transitions a card is simply in its new column after the move;
without container queries the board keeps its narrow form (snapping columns that scroll).

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let board = ui.kanban("/board/move")
    .column("todo", "To do").card("c1", "Write the docs")
    .column("doing", "Doing").limit(2).card("c2", "Calendar").description("M23").badge("Beta")
    .card("c3", "Upload")
    .column("done", "Done");
let m = board.render().into_string();
assert!(m.contains(r#"name="card" value="c1""#) && m.contains(r#"name="to" value="doing""#));
assert!(m.contains("2/2") && m.contains(">Beta</span>") && !m.contains(r#"value="todo" aria-label="Move Write"#));

// The same in `lui!`:
let same = lui! { Kanban("/board/move") {
    column "todo" "To do" { card "c1" "Write the docs"; }
    column "doing" "Doing" limit=2 {
        card "c2" "Calendar" description="M23" badge="Beta";
        card "c3" "Upload";
    }
    column "done" "Done";
} };
assert_eq!(same.into_string(), m);
```
