# InvoicePage

`loco_ui::blocks::invoice_page` · demo: `/blocks/invoice`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `InvoicePage` | `ui.invoice_page(title: &str)` | beta |

## Props of `InvoicePage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `status` | value | `badge: Badge<'a>` |  |  | A badge beside the title: paid, due, overdue. |
| `from` | value | `address: impl Render` |  |  | Who the invoice is from: a name and address. |
| `to` | value | `address: impl Render` |  |  | Who it is billed to: a name and address. |
| `meta` | item | `label: &'a str, value: &'a str` |  |  | A date or reference beside the addresses (`Issued`, `Due`). |
| `line_item` | item | `item: &'a str, quantity: &'a str, price: &'a str, amount: &'a str` |  |  | A line of the bill, every figure formatted by the app. |
| `total` | item | `label: &'a str, value: &'a str` |  |  | A row under the lines; the last one is the amount due, in bold. |
| `note` | value | `text: &'a str` |  |  | Payment terms or a thank-you under the totals. |
| `actions` | value | `actions: Markup` |  |  | Buttons beside the title (pay, download); left off paper. |

## Keyboard of `InvoicePage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Enter` | Moves between the actions and follows or presses one. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-invoice` | `--lui-space-6` |
| `.lui-invoice-head` | `--lui-space-4` |
| `.lui-invoice-title` | `--lui-space-3` |
| `.lui-invoice-actions` | `--lui-space-2` |
| `.lui-invoice-sheet` | `--lui-space-6` |
| `.lui-invoice-parties` | `--lui-space-4` |
| `.lui-invoice-party` | `--lui-space-1`, `--lui-text-1`, `--lui-muted` |
| `.lui-invoice-meta` | `--lui-space-1`, `--lui-space-2`, `--lui-muted` |
| `.lui-invoice-lines` | `--lui-text-2`, `--lui-space-2`, `--lui-muted`, `--lui-line` |
| `.lui-invoice-num` |  |
| `.lui-invoice-due` | `--lui-fg`, `--lui-text-4` |
| `.lui-invoice-wide` |  |
| `.lui-invoice-each` | `--lui-muted`, `--lui-text-1` |
| `.lui-invoice-note` | `--lui-muted`, `--lui-text-2` |

## Documentation

A bill to read, pay or print: its title and status, who it is from and to, the dates, the
lines with their quantities and amounts, the totals, a note and the actions. Every figure
is text the app formats (currency, decimals, tax rules stay the app's). Built from the
Radix Themes parts it uses (Card, Table, Badge, Data List); Radix has no invoice of its
own.

**Platform features:** a `<table>` with `<thead>` and `<tfoot>`; a container query
(Chrome 105, Firefox 110, Safari 16) that shows the quantity and price columns from 30rem;
`@media print` to drop the actions from paper.

**Accessibility:** the title is the page's `<h1>`; the lines are a real table with column
heads, the totals in its footer with row heads; the status is a badge in words.

**What it does not do without script:** nothing is missing; printing is the browser's.

**Fallback:** without container queries the quantity and price show under each line's
name (the narrow layout) at every width.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.invoice_page("Invoice INV-042").status(ui.badge("Paid").ok())
    .from(html! { "Acme Ltd" br; "1 Engine Row" }).to(html! { "Ada Lovelace" })
    .meta("Issued", "1 Oct 2026").meta("Due", "31 Oct 2026")
    .line_item("Design review", "2", "$400.00", "$800.00")
    .line_item("Hosting", "1", "$40.00", "$40.00")
    .total("Subtotal", "$840.00").total("Total", "$840.00")
    .render().into_string();
assert!(m.contains("<h1>Invoice INV-042</h1>") && m.contains("<th scope=\"row\">Total</th>"));
assert_eq!(m.matches("lui-invoice-line\"").count(), 2);

// The same in `lui!`:
let same = lui! { InvoicePage("Invoice INV-042") status=(ui.badge("Paid").ok())
    from={ "Acme Ltd" br; "1 Engine Row" } to={ "Ada Lovelace" } {
    meta "Issued" "1 Oct 2026"; meta "Due" "31 Oct 2026";
    line_item "Design review" "2" "$400.00" "$800.00";
    line_item "Hosting" "1" "$40.00" "$40.00";
    total "Subtotal" "$840.00"; total "Total" "$840.00";
} };
assert_eq!(same.into_string(), m);
```
