# InboxPage

`loco_ui::blocks::inbox_page` · demo: `/blocks/inbox`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `InboxPage` | `ui.inbox_page(title: &str, href: &str)` | beta |

## Props of `InboxPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `item` | item | `title: &'a str, href: &'a str` |  |  | An item, after the ones added before it: its title and its page. |
| `description` | modifier | `text: &'a str` |  |  | A preview line under the last item's title. |
| `when` | modifier | `text: &'a str` |  |  | When the last item arrived, formatted (`09:12`, `Mon`). |
| `unread` | condition | `on: bool` | `off` |  | Whether the last item is unread: a dot and the word for screen readers. |
| `body` | value | `body: Markup` |  |  | The open item, in the detail pane. |

## Keyboard of `InboxPage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the items, top to bottom. | no |
| `Enter` | Opens the focused item's page. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-inbox` |  |
| `.lui-inbox-panes` | `--lui-space-6` |
| `.lui-inbox-list` | `--lui-space-1` |
| `.lui-inbox-title` | `--lui-space-3`, `--lui-text-6` |
| `.lui-inbox-item` | `--lui-space-1`, `--lui-space-3`, `--lui-line`, `--lui-radius`, `--lui-gray-3`, `--lui-gray-4`, `--lui-gray-7`, `--lui-hit` |
| `.lui-inbox-item-head` | `--lui-space-2` |
| `.lui-inbox-item-title` | `--lui-text-2`, `--lui-space-2`, `--lui-primary` |
| `.lui-inbox-item-when` | `--lui-muted`, `--lui-text-1` |
| `.lui-inbox-item-description` | `--lui-muted`, `--lui-text-1` |
| `.lui-inbox-detail` |  |
| `.lui-inbox-back` | `--lui-space-1`, `--lui-space-4`, `--lui-text-2`, `--lui-hit` |

## Documentation

A list and the item open beside it: messages, tickets, orders. Each item is a link to its
own page; the one whose `href` is the current path is marked and its page fills the detail
pane. In a wide box the list and the detail sit side by side; in a narrow one an open item
shows alone, with a Back link to the list. After the shadcn/ui Mail example: a column of
rows with a sender line, a muted preview and the date at the end, the open row on the
accent.

**Platform features:** plain links and `aria-current="page"`; a container query
(Chrome 105, Firefox 110, Safari 16) for the two columns from 48rem.

**Accessibility:** the list is a `<nav>` named by the title, the open item's link has
`aria-current="page"`, and an unread item says so in words after its title (the dot is
decorative). The detail is an `<article>`.

**What it does not do without script:** open an item without a page load; each item is a
page of its own. With the enhancement script a swap root inside the detail still updates
in place.

**Fallback:** without container queries the narrow layout shows at every width.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/inbox/2", "", "");
let m = ui.inbox_page("Inbox", "/inbox")
    .item("Ada Lovelace", "/inbox/1").description("The notes on the engine").when("09:12").unread(true)
    .item("Charles Babbage", "/inbox/2").description("Lunch on Friday?").when("Mon")
    .body(html! { h2 { "Lunch on Friday?" } })
    .render().into_string();
assert!(m.contains(r#"href="/inbox/2" aria-current="page""#) && m.contains("data-open"));
assert!(m.contains(r#"<span class="lui-sr">unread</span>"#));

// The same in `lui!`:
let same = lui! { InboxPage("Inbox", "/inbox") {
    item "Ada Lovelace" "/inbox/1" description="The notes on the engine" when="09:12" unread=(true);
    item "Charles Babbage" "/inbox/2" description="Lunch on Friday?" when="Mon";
    body { h2 { "Lunch on Friday?" } }
} };
assert_eq!(same.into_string(), m);
```
