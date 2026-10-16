//! # Invoice page
//!
//! A bill to read, pay or print: its title and status, who it is from and to, the dates, the
//! lines with their quantities and amounts, the totals, a note and the actions. Every figure
//! is text the app formats (currency, decimals, tax rules stay the app's). Built from the
//! Radix Themes parts it uses (Card, Table, Badge, Data List); Radix has no invoice of its
//! own.
//!
//! **Platform features:** a `<table>` with `<thead>` and `<tfoot>`; a container query
//! (Chrome 105, Firefox 110, Safari 16) that shows the quantity and price columns from 30rem;
//! `@media print` to drop the actions from paper.
//!
//! **Accessibility:** the title is the page's `<h1>`; the lines are a real table with column
//! heads, the totals in its footer with row heads; the status is a badge in words.
//!
//! **What it does not do without script:** nothing is missing; printing is the browser's.
//!
//! **Fallback:** without container queries the quantity and price show under each line's
//! name (the narrow layout) at every width.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.invoice_page("Invoice INV-042").status(ui.badge("Paid").ok())
//!     .from(html! { "Acme Ltd" br; "1 Engine Row" }).to(html! { "Ada Lovelace" })
//!     .meta("Issued", "1 Oct 2026").meta("Due", "31 Oct 2026")
//!     .line_item("Design review", "2", "$400.00", "$800.00")
//!     .line_item("Hosting", "1", "$40.00", "$40.00")
//!     .total("Subtotal", "$840.00").total("Total", "$840.00")
//!     .render().into_string();
//! assert!(m.contains("<h1>Invoice INV-042</h1>") && m.contains("<th scope=\"row\">Total</th>"));
//! assert_eq!(m.matches("lui-invoice-line\"").count(), 2);
//!
//! // The same in `lui!`:
//! let same = lui! { InvoicePage("Invoice INV-042") status=(ui.badge("Paid").ok())
//!     from={ "Acme Ltd" br; "1 Engine Row" } to={ "Ada Lovelace" } {
//!     meta "Issued" "1 Oct 2026"; meta "Due" "31 Oct 2026";
//!     line_item "Design review" "2" "$400.00" "$800.00";
//!     line_item "Hosting" "1" "$40.00" "$40.00";
//!     total "Subtotal" "$840.00"; total "Total" "$840.00";
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::badge::Badge;
use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};

/// One line: what, how many, at what price, for how much.
#[derive(Clone, Debug)]
struct Line<'a> {
    item: &'a str,
    quantity: &'a str,
    price: &'a str,
    amount: &'a str,
}

/// An invoice, made by [`Ui::invoice_page`].
///
/// **Setters.** Values and items: `.status(..)`, `.from(..)`, `.to(..)`, `.meta(..)`,
/// `.line_item(..)`, `.total(..)`, `.note(..)`, `.actions(..)`.
#[derive(Clone, Debug)]
pub struct InvoicePage<'a> {
    ui: &'a Ui,
    title: &'a str,
    status: Option<Badge<'a>>,
    from: Option<Markup>,
    to: Option<Markup>,
    meta: Vec<(&'a str, &'a str)>,
    lines: Vec<Line<'a>>,
    totals: Vec<(&'a str, &'a str)>,
    note: Option<&'a str>,
    actions: Markup,
}

impl InvoicePage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[Key::new(
        "Tab / Enter",
        "Moves between the actions and follows or presses one.",
    )];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("status", PropKind::Value, "badge: Badge<'a>")
            .doc("A badge beside the title: paid, due, overdue."),
        Prop::new("from", PropKind::Value, "address: impl Render")
            .doc("Who the invoice is from: a name and address."),
        Prop::new("to", PropKind::Value, "address: impl Render")
            .doc("Who it is billed to: a name and address."),
        Prop::new("meta", PropKind::Item, "label: &'a str, value: &'a str")
            .doc("A date or reference beside the addresses (`Issued`, `Due`)."),
        Prop::new(
            "line_item",
            PropKind::Item,
            "item: &'a str, quantity: &'a str, price: &'a str, amount: &'a str",
        )
        .doc("A line of the bill, every figure formatted by the app."),
        Prop::new("total", PropKind::Item, "label: &'a str, value: &'a str")
            .doc("A row under the lines; the last one is the amount due, in bold."),
        Prop::new("note", PropKind::Value, "text: &'a str")
            .doc("Payment terms or a thank-you under the totals."),
        Prop::new("actions", PropKind::Value, "actions: Markup")
            .doc("Buttons beside the title (pay, download); left off paper."),
    ];
}

impl Ui {
    /// An invoice titled `title` (`"Invoice INV-042"`).
    pub fn invoice_page<'a>(&'a self, title: &'a str) -> InvoicePage<'a> {
        InvoicePage {
            ui: self,
            title,
            status: None,
            from: None,
            to: None,
            meta: Vec::new(),
            lines: Vec::new(),
            totals: Vec::new(),
            note: None,
            actions: Markup::default(),
        }
    }
}

impl<'a> InvoicePage<'a> {
    /// A badge beside the title: paid, due, overdue.
    pub fn status(mut self, badge: Badge<'a>) -> Self {
        self.status = Some(badge);
        self
    }

    /// Who the invoice is from: a name and address.
    pub fn from(mut self, address: impl Render) -> Self {
        self.from = Some(address.render());
        self
    }

    /// Who it is billed to: a name and address.
    pub fn to(mut self, address: impl Render) -> Self {
        self.to = Some(address.render());
        self
    }

    /// A date or reference beside the addresses (`"Issued"`, `"Due"`, `"PO number"`).
    pub fn meta(mut self, label: &'a str, value: &'a str) -> Self {
        self.meta.push((label, value));
        self
    }

    /// A line of the bill: what, how many, the unit price and the amount, each formatted.
    pub fn line_item(
        mut self,
        item: &'a str,
        quantity: &'a str,
        price: &'a str,
        amount: &'a str,
    ) -> Self {
        self.lines.push(Line {
            item,
            quantity,
            price,
            amount,
        });
        self
    }

    /// A row under the lines (`"Subtotal"`, `"Tax"`); the last one is the amount due.
    pub fn total(mut self, label: &'a str, value: &'a str) -> Self {
        self.totals.push((label, value));
        self
    }

    /// Payment terms or a thank-you under the totals.
    pub fn note(mut self, text: &'a str) -> Self {
        self.note = Some(text);
        self
    }

    /// Buttons beside the title (pay, download a PDF); left off paper.
    pub fn actions(mut self, actions: Markup) -> Self {
        self.actions = actions;
        self
    }
}

impl Render for InvoicePage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let last = self.totals.len().saturating_sub(1);
        let address = |text: Text, body: &Option<Markup>| {
            html! {
                @if let Some(b) = body {
                    div class="lui-invoice-party" {
                        h2 { (ui.text(text)) }
                        address { (b) }
                    }
                }
            }
        };
        html! {
            div class="lui-invoice" {
                div class="lui-invoice-head" {
                    div class="lui-invoice-title" {
                        h1 { (self.title) }
                        @if let Some(s) = &self.status { (s) }
                    }
                    @if !self.actions.0.is_empty() { div class="lui-invoice-actions" { (self.actions) } }
                }
                (ui.card().body(html! { div class="lui-invoice-sheet" {
                    div class="lui-invoice-parties" {
                        (address(Text::Sender, &self.from))
                        (address(Text::BilledTo, &self.to))
                        @if !self.meta.is_empty() {
                            dl class="lui-invoice-meta" {
                                @for (label, value) in &self.meta { div { dt { (label) } dd { (value) } } }
                            }
                        }
                    }
                    table class="lui-invoice-lines" {
                        thead {
                            tr {
                                th scope="col" { (ui.text(Text::Item)) }
                                th scope="col" class="lui-invoice-num lui-invoice-wide" { (ui.text(Text::Quantity)) }
                                th scope="col" class="lui-invoice-num lui-invoice-wide" { (ui.text(Text::Price)) }
                                th scope="col" class="lui-invoice-num" { (ui.text(Text::Amount)) }
                            }
                        }
                        tbody {
                            @for l in &self.lines {
                                tr class="lui-invoice-line" {
                                    td {
                                        (l.item)
                                        span class="lui-invoice-each" { (l.quantity) " \u{d7} " (l.price) }
                                    }
                                    td class="lui-invoice-num lui-invoice-wide" { (l.quantity) }
                                    td class="lui-invoice-num lui-invoice-wide" { (l.price) }
                                    td class="lui-invoice-num" { (l.amount) }
                                }
                            }
                        }
                        @if !self.totals.is_empty() {
                            tfoot {
                                @for (i, (label, value)) in self.totals.iter().enumerate() {
                                    tr class=[(i == last).then_some("lui-invoice-due")] {
                                        th scope="row" { (label) }
                                        td class="lui-invoice-wide" {}
                                        td class="lui-invoice-wide" {}
                                        td class="lui-invoice-num" { (value) }
                                    }
                                }
                            }
                        }
                    }
                    @if let Some(n) = self.note { p class="lui-invoice-note" { (n) } }
                } }))
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. Narrow: the addresses stacked
/// and each line's quantity and price under its name. From 30rem: the addresses in a row
/// and the quantity and price in columns of their own.
pub const CSS: &str = r#"
.lui-invoice { container: lui-invoice / inline-size; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-6); }
.lui-invoice-head { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--lui-space-4); }
.lui-invoice-title { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-3); }
.lui-invoice-title h1 { margin: 0; }
.lui-invoice-actions { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); }
.lui-invoice-sheet { display: grid; gap: var(--lui-space-6); }
.lui-invoice-parties { display: grid; gap: var(--lui-space-4); }
.lui-invoice-party h2 { margin: 0 0 var(--lui-space-1); font-size: var(--lui-text-1); font-weight: 500; color: var(--lui-muted); text-transform: uppercase; letter-spacing: 0.04em; }
.lui-invoice-party address { font-style: normal; overflow-wrap: anywhere; }
.lui-invoice-meta { display: grid; gap: var(--lui-space-1); margin: 0; }
.lui-invoice-meta div { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); }
.lui-invoice-meta dt { color: var(--lui-muted); }
.lui-invoice-meta dd { margin: 0; font-weight: 500; }
.lui-invoice-lines { width: 100%; border-collapse: collapse; font-size: var(--lui-text-2); }
.lui-invoice-lines th, .lui-invoice-lines td { padding: var(--lui-space-2) var(--lui-space-2); text-align: start; vertical-align: top; overflow-wrap: anywhere; }
.lui-invoice-lines thead th { color: var(--lui-muted); font-weight: 500; border-block-end: 1px solid var(--lui-line); }
.lui-invoice-lines tbody tr { border-block-end: 1px solid var(--lui-line); }
.lui-invoice-lines tfoot th { font-weight: 400; color: var(--lui-muted); }
.lui-invoice-lines .lui-invoice-num { text-align: end; font-variant-numeric: tabular-nums; white-space: nowrap; }
.lui-invoice-lines .lui-invoice-due th, .lui-invoice-lines .lui-invoice-due td { font-weight: 600; color: var(--lui-fg); font-size: var(--lui-text-4); }
.lui-invoice-wide { display: none; }
.lui-invoice-each { display: block; color: var(--lui-muted); font-size: var(--lui-text-1); font-variant-numeric: tabular-nums; }
.lui-invoice-note { margin: 0; color: var(--lui-muted); font-size: var(--lui-text-2); }
@container lui-invoice (width >= 30rem) {
  .lui-invoice-parties { grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr)); }
  .lui-invoice-wide { display: table-cell; }
  .lui-invoice-each { display: none; }
}
@media print {
  .lui-invoice-actions { display: none; }
  .lui-invoice > .lui-card { border: 0; padding: 0; box-shadow: none; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_invoice_is_a_title_and_an_empty_table_with_its_heads() {
        let ui = Ui::default();
        let m = ui.invoice_page("Bill 7").render().into_string();
        assert!(m.starts_with(r#"<div class="lui-invoice"><div class="lui-invoice-head"><div class="lui-invoice-title"><h1>Bill 7</h1></div></div>"#), "{m}");
        assert!(
            m.contains(">Item</th>")
                && m.contains(">Quantity</th>")
                && m.contains(">Price</th>")
                && m.contains(">Amount</th>"),
            "{m}"
        );
        assert!(
            !m.contains("<tfoot")
                && !m.contains("<address")
                && !m.contains("<dl")
                && !m.contains("lui-invoice-actions"),
            "{m}"
        );
    }

    #[test]
    fn lines_show_every_figure_and_the_last_total_is_the_amount_due() {
        let ui = Ui::default();
        let m = ui
            .invoice_page("Bill 7")
            .line_item("Widgets", "3", "€2", "€6")
            .total("Subtotal", "€6")
            .total("Due", "€7.20")
            .render()
            .into_string();
        assert!(
            m.contains(r#"<td>Widgets<span class="lui-invoice-each">3 × €2</span></td>"#),
            "{m}"
        );
        assert!(m.contains(r#"<td class="lui-invoice-num">€6</td>"#), "{m}");
        assert!(m.contains(r#"<tr><th scope="row">Subtotal</th>"#), "{m}");
        assert!(
            m.contains(r#"<tr class="lui-invoice-due"><th scope="row">Due</th>"#),
            "{m}"
        );
    }

    #[test]
    fn parties_meta_status_note_and_actions_have_their_places() {
        let ui = Ui::default();
        let m = ui
            .invoice_page("Bill 7")
            .status(ui.badge("Overdue").danger())
            .from("Shop")
            .to(html! { b { "Ada" } })
            .meta("Due", "1 Nov")
            .note("Thanks.")
            .actions(html! { a href="/pay" { "Pay" } })
            .render()
            .into_string();
        assert!(
            m.contains(r#"<h1>Bill 7</h1><span class="lui-badge"#) && m.contains("Overdue"),
            "{m}"
        );
        assert!(m.contains(r#"<h2>From</h2><address>Shop</address>"#), "{m}");
        assert!(
            m.contains(r#"<h2>Billed to</h2><address><b>Ada</b></address>"#),
            "{m}"
        );
        assert!(m.contains("<div><dt>Due</dt><dd>1 Nov</dd></div>"), "{m}");
        assert!(
            m.contains(r#"<p class="lui-invoice-note">Thanks.</p>"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<div class="lui-invoice-actions"><a href="/pay">Pay</a></div>"#),
            "{m}"
        );
    }
}
