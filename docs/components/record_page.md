# RecordPage

`loco_ui::blocks::record_page` · demo: `/blocks/record`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `RecordPage` | `ui.record_page(title: &str)` | beta |

## Props of `RecordPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `field` | item | `label: &'a str, value: impl Render` |  |  | A label and its value. |
| `edit` | value | `href: &'a str` |  | `href` | An Edit link. |
| `delete` | value | `action: &'a str` |  | `action` | A Delete button that asks first, in a dialog, then posts to `action`. |
| `back` | value | `href: &'a str` |  | `href` | A Back link to the list. |

## Keyboard of `RecordPage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | Follows the Back or Edit link. | no |
| `Enter / Space` | Opens the delete confirmation from the Delete button. | no |
| `Escape` | Closes the delete confirmation without deleting. Without invoker commands or the script, its Close link closes it instead. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-record` | `--lui-space` |
| `.lui-record-head` | `--lui-space` |
| `.lui-record-actions` | `--lui-space` |
| `.lui-record-fields` | `--lui-space-6`, `--lui-card`, `--lui-line`, `--lui-radius-lg` |

## Documentation

One record: its title, its fields as a description list in a card, and the actions on it
(edit, delete) beside the title. The show page a scaffold writes, ready-made.

**Platform features:** a `crate::description_list` (its container query and subgrid);
delete opens a `crate::dialog` ("Delete Invoice 42?") whose confirm button is a form that
posts (Post/Redirect/Get).

**Accessibility:** the title is the page's `<h1>`; labels and values are `<dt>`/`<dd>`
pairs; edit is a link, delete a real button that opens a modal dialog. Checked by axe-core
with the demo routes.

**What it does not do without script:** nothing is missing.

**Fallback:** the dialog's own: without invoker commands the Delete button is a link to
`#lui-record-delete` and the dialog opens through `:target`.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.record_page("Invoice 42").field("Customer", "Ada").field("Total", "€120")
    .edit("/invoices/42/edit").delete("/invoices/42/delete").render().into_string();
assert!(m.contains("<dt>Customer</dt><dd>Ada</dd>") && m.contains(r#"action="/invoices/42/delete""#));
assert!(m.contains(r#"<h2 id="lui-record-delete-title" class="lui-dialog-title">Delete Invoice 42?</h2>"#));
// The same in `lui!`:
let same = lui! { RecordPage("Invoice 42") edit="/invoices/42/edit" delete="/invoices/42/delete" {
    field "Customer" "Ada"; field "Total" "€120";
} };
assert_eq!(same.into_string(), m);
```
