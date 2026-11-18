# BillingPage

`loco_ui::blocks::billing_page` · demo: `/blocks/billing`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `BillingPage` | `ui.billing_page(title: &str)` | beta |

## Props of `BillingPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `plan` | value | `name: &'a str, price: &'a str` |  |  | The plan the account is on and what it costs (`"$12 per seat / month"`). |
| `renews` | value | `text: &'a str` |  |  | A line under the plan: when it renews or ends. |
| `actions` | value | `actions: Markup` |  |  | What sits in the plan's card: a Change plan link, a Cancel button. |
| `usage` | item | `label: &'a str, value: u64, max: u64` |  |  | An allowance: how much of `max` is used. The bar turns red when it is full. |
| `method` | value | `text: &'a str, href: &'a str` |  |  | The payment method (`"Visa ending 4242"`) and the page that changes it. |
| `invoice` | item | `number: &'a str, date: &'a str, amount: &'a str` |  |  | A past invoice, after the ones added before it. |
| `href` | modifier | `href: &'a str` |  | `href` | The last invoice's page or file: its number becomes a link. |
| `status` | modifier | `badge: Badge<'a>` |  |  | The last invoice's status as a badge: paid, due, overdue. |

## Keyboard of `BillingPage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | Follows the plan's actions, the payment method's Edit link or an invoice's link. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-billing` | `--lui-space-6` |
| `.lui-billing-top` | `--lui-space-6` |
| `.lui-billing-section` | `--lui-space-2`, `--lui-space-6`, `--lui-card`, `--lui-line`, `--lui-radius-lg`, `--lui-shadow-sm`, `--lui-text-2`, `--lui-leading-2`, `--lui-muted` |
| `.lui-billing-plan-name` | `--lui-text-6`, `--lui-leading-6` |
| `.lui-billing-plan-price` | `--lui-text-3` |
| `.lui-billing-method-text` | `--lui-text-3` |
| `.lui-billing-note` | `--lui-muted`, `--lui-text-2` |
| `.lui-billing-actions` | `--lui-space-2` |
| `.lui-billing-usage-list` | `--lui-space-4` |
| `.lui-billing-usage-item` | `--lui-space-1` |
| `.lui-billing-usage-value` | `--lui-muted`, `--lui-text-1` |
| `.lui-billing-invoice-list` |  |
| `.lui-billing-invoice` | `--lui-space-1`, `--lui-space-3`, `--lui-line`, `--lui-text-2` |
| `.lui-billing-invoice-number` | `--lui-hit` |
| `.lui-billing-invoice-date` | `--lui-muted` |
| `.lui-billing-invoice-status` |  |
| `.lui-billing-invoice-amount` |  |

## Documentation

An account's billing: the plan it is on and what it costs, how much of each allowance is
used, the payment method, and the past invoices. After the Tremor billing and usage
blocks, built from Radix Themes parts (Card, Progress, Badge).

**Platform features:** each allowance is a `<progress>` (`crate::progress`) with its
numbers beside it; a container query (Chrome 105, Firefox 110, Safari 16) sets the plan
and the payment method side by side from 48rem and gives each invoice one row from 30rem.

**Accessibility:** the title is the page's `<h1>`; each part is a `<section>` named by its
`<h2>`; an allowance's bar is labelled by its name and the numbers are text; an invoice's
number is the link. Checked by axe-core with the demo routes.

**What it does not do without script:** nothing is missing; changing the plan or the card
is a link to a page of the app.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.billing_page("Billing").plan("Team", "$12 per seat / month").render().into_string();
assert!(m.contains(r#"<h2 id="lui-billing-plan-title">Current plan</h2>"#) && m.contains("$12 per seat / month"));
// With allowances, a card and invoices:
let m = ui.billing_page("Billing").plan("Team", "$12 per seat / month")
    .renews("Renews on 1 December 2026.")
    .actions(html! { a href="/pricing" { "Change plan" } })
    .usage("Seats", 8, 10).usage("Storage (GB)", 42, 100)
    .method("Visa ending 4242", "/billing/card")
    .invoice("INV-0042", "1 Oct 2026", "$96.00").href("/invoices/42").status(ui.badge("Paid").ok())
    .render().into_string();
assert!(m.contains(r#"<progress id="lui-billing-usage-1" class="lui-progress" value="8" max="10">"#));
assert!(m.contains(r#"<a href="/invoices/42">INV-0042</a>"#) && m.contains("Visa ending 4242"));
// The same in `lui!`:
let same = lui! { BillingPage("Billing") plan=("Team", "$12 per seat / month")
    renews="Renews on 1 December 2026." actions={ a href="/pricing" { "Change plan" } }
    method=("Visa ending 4242", "/billing/card") {
    usage "Seats" 8 10; usage "Storage (GB)" 42 100;
    invoice "INV-0042" "1 Oct 2026" "$96.00" href="/invoices/42" status=(ui.badge("Paid").ok());
} };
assert_eq!(same.into_string(), m);
```
