//! # Billing page
//!
//! An account's billing: the plan it is on and what it costs, how much of each allowance is
//! used, the payment method, and the past invoices. After the Tremor billing and usage
//! blocks, built from Radix Themes parts (Card, Progress, Badge).
//!
//! **Platform features:** each allowance is a `<progress>` ([`crate::progress`]) with its
//! numbers beside it; a container query (Chrome 105, Firefox 110, Safari 16) sets the plan
//! and the payment method side by side from 48rem and gives each invoice one row from 30rem.
//!
//! **Accessibility:** the title is the page's `<h1>`; each part is a `<section>` named by its
//! `<h2>`; an allowance's bar is labelled by its name and the numbers are text; an invoice's
//! number is the link. Checked by axe-core with the demo routes.
//!
//! **What it does not do without script:** nothing is missing; changing the plan or the card
//! is a link to a page of the app.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.billing_page("Billing").plan("Team", "$12 per seat / month").render().into_string();
//! assert!(m.contains(r#"<h2 id="lui-billing-plan-title">Current plan</h2>"#) && m.contains("$12 per seat / month"));
//! // With allowances, a card and invoices:
//! let m = ui.billing_page("Billing").plan("Team", "$12 per seat / month")
//!     .renews("Renews on 1 December 2026.")
//!     .actions(html! { a href="/pricing" { "Change plan" } })
//!     .usage("Seats", 8, 10).usage("Storage (GB)", 42, 100)
//!     .method("Visa ending 4242", "/billing/card")
//!     .invoice("INV-0042", "1 Oct 2026", "$96.00").href("/invoices/42").status(ui.badge("Paid").ok())
//!     .render().into_string();
//! assert!(m.contains(r#"<progress id="lui-billing-usage-1" class="lui-progress" value="8" max="10">"#));
//! assert!(m.contains(r#"<a href="/invoices/42">INV-0042</a>"#) && m.contains("Visa ending 4242"));
//! // The same in `lui!`:
//! let same = lui! { BillingPage("Billing") plan=("Team", "$12 per seat / month")
//!     renews="Renews on 1 December 2026." actions={ a href="/pricing" { "Change plan" } }
//!     method=("Visa ending 4242", "/billing/card") {
//!     usage "Seats" 8 10; usage "Storage (GB)" 42 100;
//!     invoice "INV-0042" "1 Oct 2026" "$96.00" href="/invoices/42" status=(ui.badge("Paid").ok());
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::badge::Badge;
use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::variant::Tone;

/// A past invoice: its number, date, amount, page and status.
#[derive(Clone, Debug)]
struct Invoice<'a> {
    number: &'a str,
    date: &'a str,
    amount: &'a str,
    href: Option<&'a str>,
    status: Markup,
}

/// An account's billing page, made by [`Ui::billing_page`].
///
/// **Setters.** Values and items: `.plan(..)`, `.renews(..)`, `.actions(..)`, `.usage(..)`,
/// `.method(..)`, `.invoice(..)`, `.href(..)`, `.status(..)`.
#[derive(Clone, Debug)]
pub struct BillingPage<'a> {
    ui: &'a Ui,
    title: &'a str,
    plan: Option<(&'a str, &'a str)>,
    renews: Option<&'a str>,
    actions: Markup,
    usage: Vec<(&'a str, u64, u64)>,
    method: Option<(&'a str, &'a str)>,
    invoices: Vec<Invoice<'a>>,
}

impl BillingPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[Key::new(
        "Enter",
        "Follows the plan's actions, the payment method's Edit link or an invoice's link.",
    )];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("plan", PropKind::Value, "name: &'a str, price: &'a str")
            .doc("The plan the account is on and what it costs (`\"$12 per seat / month\"`)."),
        Prop::new("renews", PropKind::Value, "text: &'a str")
            .doc("A line under the plan: when it renews or ends."),
        Prop::new("actions", PropKind::Value, "actions: Markup")
            .doc("What sits in the plan's card: a Change plan link, a Cancel button."),
        Prop::new(
            "usage",
            PropKind::Item,
            "label: &'a str, value: u64, max: u64",
        )
        .doc("An allowance: how much of `max` is used. The bar turns red when it is full."),
        Prop::new("method", PropKind::Value, "text: &'a str, href: &'a str")
            .doc("The payment method (`\"Visa ending 4242\"`) and the page that changes it."),
        Prop::new(
            "invoice",
            PropKind::Item,
            "number: &'a str, date: &'a str, amount: &'a str",
        )
        .doc("A past invoice, after the ones added before it."),
        Prop::new("href", PropKind::Modifier, "href: &'a str")
            .applies_to(&["invoice"])
            .attr("href")
            .doc("The last invoice's page or file: its number becomes a link."),
        Prop::new("status", PropKind::Modifier, "badge: Badge<'a>")
            .applies_to(&["invoice"])
            .doc("The last invoice's status as a badge: paid, due, overdue."),
    ];
}

impl Ui {
    /// The billing page titled `title`.
    pub fn billing_page<'a>(&'a self, title: &'a str) -> BillingPage<'a> {
        BillingPage {
            ui: self,
            title,
            plan: None,
            renews: None,
            actions: Markup::default(),
            usage: Vec::new(),
            method: None,
            invoices: Vec::new(),
        }
    }
}

impl<'a> BillingPage<'a> {
    /// The plan the account is on and what it costs (`"$12 per seat / month"`).
    pub fn plan(mut self, name: &'a str, price: &'a str) -> Self {
        self.plan = Some((name, price));
        self
    }

    /// A line under the plan: when it renews or ends.
    pub fn renews(mut self, text: &'a str) -> Self {
        self.renews = Some(text);
        self
    }

    /// What sits in the plan's card: a Change plan link, a Cancel button.
    pub fn actions(mut self, actions: Markup) -> Self {
        self.actions = actions;
        self
    }

    /// An allowance: how much of `max` is used. The bar turns red when it is full.
    pub fn usage(mut self, label: &'a str, value: u64, max: u64) -> Self {
        self.usage.push((label, value, max));
        self
    }

    /// The payment method (`"Visa ending 4242"`) and the page that changes it.
    pub fn method(mut self, text: &'a str, href: &'a str) -> Self {
        self.method = Some((text, href));
        self
    }

    /// A past invoice, after the ones added before it.
    pub fn invoice(mut self, number: &'a str, date: &'a str, amount: &'a str) -> Self {
        self.invoices.push(Invoice {
            number,
            date,
            amount,
            href: None,
            status: Markup::default(),
        });
        self
    }

    /// The invoice added last, after checking that there is one.
    #[track_caller]
    fn last(&mut self, method: &str) -> Option<&mut Invoice<'a>> {
        crate::props::check_last::<Self>(
            BillingPage::PROPS,
            method,
            self.invoices.last().map(|_| "invoice"),
        );
        self.invoices.last_mut()
    }

    /// The last invoice's page or file: its number becomes a link.
    #[track_caller]
    pub fn href(mut self, href: &'a str) -> Self {
        if let Some(i) = self.last("href") {
            i.href = Some(href);
        }
        self
    }

    /// The last invoice's status as a badge: paid, due, overdue.
    #[track_caller]
    pub fn status(mut self, badge: Badge<'a>) -> Self {
        if let Some(i) = self.last("status") {
            i.status = badge.render();
        }
        self
    }
}

/// A part of the page: a section named by its heading.
fn section(id: &str, title: &str, body: Markup) -> Markup {
    let heading = format!("{id}-title");
    html! {
        section class="lui-billing-section" id=(id) aria-labelledby=(heading) {
            h2 id=(heading) { (title) }
            (body)
        }
    }
}

impl Render for BillingPage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        html! {
            div class="lui-billing" {
                h1 { (self.title) }
                div class="lui-billing-top" {
                    @if let Some((name, price)) = self.plan {
                        (section("lui-billing-plan", ui.text(Text::CurrentPlan), html! {
                            p class="lui-billing-plan-name" { (name) }
                            p class="lui-billing-plan-price" { (price) }
                            @if let Some(r) = self.renews { p class="lui-billing-note" { (r) } }
                            @if !self.actions.0.is_empty() { div class="lui-billing-actions" { (self.actions) } }
                        }))
                    }
                    @if let Some((text, href)) = self.method {
                        (section("lui-billing-method", ui.text(Text::PaymentMethod), html! {
                            p class="lui-billing-method-text" { (text) }
                            div class="lui-billing-actions" { (ui.link_button(ui.text(Text::Edit), href)) }
                        }))
                    }
                }
                @if !self.usage.is_empty() {
                    (section("lui-billing-usage", ui.text(Text::Usage), html! {
                        div class="lui-billing-usage-list" {
                            @for (n, &(label, value, max)) in self.usage.iter().enumerate() {
                                @let id = format!("lui-billing-usage-{}", n + 1);
                                @let bar = ui.progress(value, max).label(label).id(&id);
                                div class="lui-billing-usage-item" {
                                    @if value >= max { (bar.tone(Tone::Danger)) } @else { (bar) }
                                    span class="lui-billing-usage-value" { (value) " / " (max) }
                                }
                            }
                        }
                    }))
                }
                @if !self.invoices.is_empty() {
                    (section("lui-billing-invoices", ui.text(Text::Invoices), html! {
                        ul class="lui-billing-invoice-list" {
                            @for i in &self.invoices {
                                li class="lui-billing-invoice" {
                                    span class="lui-billing-invoice-number" {
                                        @if let Some(h) = i.href { a href=(h) { (i.number) } } @else { (i.number) }
                                    }
                                    span class="lui-billing-invoice-date" { (i.date) }
                                    span class="lui-billing-invoice-status" { (i.status) }
                                    span class="lui-billing-invoice-amount" { (i.amount) }
                                }
                            }
                        }
                    }))
                }
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. Narrow: one column, an invoice
/// on two lines. From 30rem an invoice is one row; from 48rem the plan sits beside the
/// payment method.
pub const CSS: &str = r#"
.lui-billing { container: lui-billing / inline-size; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-6); }
.lui-billing > h1 { margin: 0; }
.lui-billing-top { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-6); }
.lui-billing-top:empty { display: none; }
.lui-billing-section { display: grid; grid-template-columns: minmax(0, 1fr); align-content: start; gap: var(--lui-space-2); padding: var(--lui-space-6); background: var(--lui-card); border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg); box-shadow: var(--lui-shadow-sm); }
.lui-billing-section > h2 { margin: 0; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); letter-spacing: 0; font-weight: 500; color: var(--lui-muted); }
.lui-billing-section p { margin: 0; overflow-wrap: anywhere; }
.lui-billing-plan-name { font-size: var(--lui-text-6); line-height: var(--lui-leading-6); font-weight: 600; }
.lui-billing-plan-price, .lui-billing-method-text { font-size: var(--lui-text-3); }
.lui-billing-note { color: var(--lui-muted); font-size: var(--lui-text-2); }
.lui-billing-actions { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); margin-block-start: var(--lui-space-2); }
.lui-billing-usage-list { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-4); }
.lui-billing-usage-item { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-1); }
.lui-billing-usage-value { justify-self: end; color: var(--lui-muted); font-size: var(--lui-text-1); font-variant-numeric: tabular-nums; }
.lui-billing-invoice-list { list-style: none; margin: 0; padding: 0; }
.lui-billing-invoice { max-width: none; display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: var(--lui-space-1) var(--lui-space-3); align-items: center; padding-block: var(--lui-space-3); border-block-start: 1px solid var(--lui-line); font-size: var(--lui-text-2); }
.lui-billing-invoice:first-child { border-block-start: 0; }
.lui-billing-invoice-number { font-weight: 500; overflow-wrap: anywhere; }
.lui-billing-invoice-number a { display: inline-flex; align-items: center; }
.lui-billing-invoice-date { grid-row: 2; color: var(--lui-muted); }
.lui-billing-invoice-status { grid-row: 2; justify-self: end; }
.lui-billing-invoice-amount { grid-row: 1; grid-column: 2; justify-self: end; font-variant-numeric: tabular-nums; }
@media (pointer: coarse) { .lui-billing-invoice-number a { min-height: var(--lui-hit); } }
@container lui-billing (width >= 30rem) {
  .lui-billing-invoice { grid-template-columns: minmax(0, 2fr) minmax(0, 2fr) minmax(0, 1fr) auto; }
  .lui-billing-invoice-date, .lui-billing-invoice-status, .lui-billing-invoice-amount { grid-row: auto; grid-column: auto; }
  .lui-billing-invoice-status { justify-self: start; }
}
@container lui-billing (width >= 48rem) {
  .lui-billing-top { grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr)); }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_page_is_a_title_with_no_section() {
        let ui = Ui::default();
        let m = ui.billing_page("Billing").render().into_string();
        assert_eq!(
            m,
            r#"<div class="lui-billing"><h1>Billing</h1><div class="lui-billing-top"></div></div>"#
        );
    }

    #[test]
    fn each_part_is_a_section_named_by_its_heading() {
        let ui = Ui::default();
        let m = ui
            .billing_page("Billing")
            .plan("Team", "$12")
            .renews("Renews soon.")
            .actions(html! { a href="/p" { "Change" } })
            .method("Visa 4242", "/card")
            .usage("Seats", 8, 10)
            .invoice("INV-1", "1 Oct", "$96")
            .render()
            .into_string();
        for id in ["plan", "method", "usage", "invoices"] {
            assert!(
                m.contains(&format!(
                    r#"<section class="lui-billing-section" id="lui-billing-{id}" aria-labelledby="lui-billing-{id}-title"><h2 id="lui-billing-{id}-title">"#
                )),
                "{id}: {m}"
            );
        }
        assert!(
            m.contains(r#"<p class="lui-billing-note">Renews soon.</p>"#),
            "{m}"
        );
        assert!(m.contains(r#"href="/card""#) && m.contains(">Edit<"), "{m}");
        assert!(m.contains(r#"<a href="/p">Change</a>"#), "{m}");
    }

    #[test]
    fn a_full_allowance_is_red_and_the_numbers_are_text() {
        let ui = Ui::default();
        let m = ui
            .billing_page("B")
            .usage("Seats", 8, 10)
            .usage("Projects", 5, 5)
            .render()
            .into_string();
        assert_eq!(m.matches(r#"data-tone="danger""#).count(), 1, "{m}");
        assert!(
            m.contains(r#"<span class="lui-billing-usage-value">8 / 10</span>"#),
            "{m}"
        );
        assert!(m.contains(r#"<label for="lui-billing-usage-2">"#), "{m}");
    }

    #[test]
    fn an_invoice_links_its_number_only_with_an_href() {
        let ui = Ui::default();
        let m = ui
            .billing_page("B")
            .invoice("INV-1", "1 Oct", "$96")
            .href("/i/1")
            .status(ui.badge("Paid").ok())
            .invoice("INV-2", "1 Sep", "$90")
            .render()
            .into_string();
        assert!(m.contains(r#"<a href="/i/1">INV-1</a>"#), "{m}");
        assert!(
            m.contains(r#"<span class="lui-billing-invoice-number">INV-2</span>"#),
            "{m}"
        );
        assert_eq!(m.matches(">Paid<").count(), 1, "{m}");
        assert!(
            m.find("INV-1").unwrap_or(0) < m.find("INV-2").unwrap_or(0),
            "{m}"
        );
    }
}
