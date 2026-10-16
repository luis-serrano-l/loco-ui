//! # Pricing page
//!
//! A title, a line, and the plans side by side: each a card with its name, price, who it is
//! for, what it includes and a button. One plan can be featured: a "Popular" badge, a
//! stronger border and the primary button. Fine print or questions go under them. Copied
//! from the demo's own pricing card (`demo/src/pricing.rs`, the example of a component
//! written outside the library), after the shadcn/ui pricing blocks.
//!
//! **Platform features:** those of [`crate::card`] and [`crate::grid`]: a CSS grid with
//! `repeat(auto-fill, minmax(min(16rem, 100%), 1fr))` (`min()`: Chrome 79, Firefox 75,
//! Safari 11.1), no media query; the plans wrap to one column in a narrow box.
//!
//! **Accessibility:** the title is the page's `<h1>`; each plan's name is its card's heading;
//! the feature list is a `<ul>` whose check marks are decorative. Checked by axe-core with
//! the demo routes.
//!
//! **What it does not do without script:** switch between monthly and yearly prices in place;
//! link to the page again with a query (`?period=year`) and fill in the other prices.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.pricing_page("Pricing").description("Start free, upgrade when you grow.")
//!     .tier("Hobby", "$0").period("/month").feature("One project").cta("Start", "/signup")
//!     .tier("Pro", "$12").period("/month").blurb("For small teams.")
//!         .feature("Ten projects").feature("Email support").cta("Upgrade", "/signup?plan=pro").featured()
//!     .render().into_string();
//! assert!(m.contains(r#"<h1>Pricing</h1>"#) && m.contains(r#"data-featured"#));
//! assert_eq!(m.matches("lui-pricing-page-tier").count(), 2);
//!
//! // The same in `lui!`:
//! let same = lui! { PricingPage("Pricing") description="Start free, upgrade when you grow." {
//!     tier "Hobby" "$0" period="/month" feature="One project" cta=("Start", "/signup");
//!     tier "Pro" "$12" period="/month" blurb="For small teams."
//!         feature="Ten projects" feature="Email support" cta=("Upgrade", "/signup?plan=pro") featured;
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::{Icon, Ui};

/// One plan: name, price, what the price is per, a line, features and a button.
#[derive(Clone, Debug)]
struct Tier<'a> {
    name: &'a str,
    price: &'a str,
    period: Option<&'a str>,
    blurb: Option<&'a str>,
    features: Vec<&'a str>,
    cta: Option<(&'a str, &'a str)>,
    featured: bool,
}

/// A page of plans, made by [`Ui::pricing_page`].
///
/// **Setters.** Values and items: `.description(..)`, `.tier(..)`, `.period(..)`,
/// `.blurb(..)`, `.feature(..)`, `.cta(..)`, `.body(..)`; switches: `.featured()`.
#[derive(Clone, Debug)]
pub struct PricingPage<'a> {
    ui: &'a Ui,
    title: &'a str,
    description: Option<&'a str>,
    tiers: Vec<Tier<'a>>,
    body: Markup,
}

impl PricingPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[Key::new(
        "Tab / Enter",
        "Moves between the plans' buttons and follows one.",
    )];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("description", PropKind::Value, "text: &'a str").doc("A line under the title."),
        Prop::new("tier", PropKind::Item, "name: &'a str, price: &'a str")
            .doc("A plan, after the ones added before it: its name and its price, formatted."),
        Prop::new("period", PropKind::Modifier, "text: &'a str")
            .applies_to(&["tier"])
            .doc("What the last plan's price is per (`/month`)."),
        Prop::new("blurb", PropKind::Modifier, "text: &'a str")
            .applies_to(&["tier"])
            .doc("Who the last plan is for, under its name."),
        Prop::new("feature", PropKind::Modifier, "text: &'a str")
            .applies_to(&["tier"])
            .doc("One thing the last plan includes; call once per line."),
        Prop::new("cta", PropKind::Modifier, "text: &'a str, href: &'a str")
            .applies_to(&["tier"])
            .doc("The last plan's button: its text and where it goes."),
        Prop::new("featured", PropKind::Modifier, "")
            .applies_to(&["tier"])
            .doc("The plan to steer people to: a badge, a stronger border, the primary button."),
        Prop::new("body", PropKind::Value, "body: Markup").doc("What goes under the plans."),
    ];
}

impl Ui {
    /// A page of plans titled `title`.
    pub fn pricing_page<'a>(&'a self, title: &'a str) -> PricingPage<'a> {
        PricingPage {
            ui: self,
            title,
            description: None,
            tiers: Vec::new(),
            body: Markup::default(),
        }
    }
}

impl<'a> PricingPage<'a> {
    /// A line under the title.
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// A plan, after the ones added before it: its name and its price, formatted (`"$12"`).
    pub fn tier(mut self, name: &'a str, price: &'a str) -> Self {
        self.tiers.push(Tier {
            name,
            price,
            period: None,
            blurb: None,
            features: Vec::new(),
            cta: None,
            featured: false,
        });
        self
    }

    /// The plan added last, after checking that there is one.
    #[track_caller]
    fn last(&mut self, method: &str) -> Option<&mut Tier<'a>> {
        crate::props::check_last::<Self>(
            PricingPage::PROPS,
            method,
            self.tiers.last().map(|_| "tier"),
        );
        self.tiers.last_mut()
    }

    /// What the last plan's price is per (`"/month"`, `"per seat"`).
    #[track_caller]
    pub fn period(mut self, text: &'a str) -> Self {
        if let Some(t) = self.last("period") {
            t.period = Some(text);
        }
        self
    }

    /// Who the last plan is for, under its name.
    #[track_caller]
    pub fn blurb(mut self, text: &'a str) -> Self {
        if let Some(t) = self.last("blurb") {
            t.blurb = Some(text);
        }
        self
    }

    /// One thing the last plan includes; call once per line.
    #[track_caller]
    pub fn feature(mut self, text: &'a str) -> Self {
        if let Some(t) = self.last("feature") {
            t.features.push(text);
        }
        self
    }

    /// The last plan's button: its text and where it goes.
    #[track_caller]
    pub fn cta(mut self, text: &'a str, href: &'a str) -> Self {
        if let Some(t) = self.last("cta") {
            t.cta = Some((text, href));
        }
        self
    }

    /// The plan to steer people to: a "Popular" badge, a stronger border, the primary button.
    #[track_caller]
    pub fn featured(mut self) -> Self {
        if let Some(t) = self.last("featured") {
            t.featured = true;
        }
        self
    }

    /// What goes under the plans: fine print, questions and answers.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }
}

impl Render for PricingPage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let tier = |t: &Tier| {
            let body = html! {
                p class="lui-pricing-page-price" {
                    span class="lui-pricing-page-amount" { (t.price) }
                    @if let Some(p) = t.period { span class="lui-pricing-page-period" { (p) } }
                }
                @if !t.features.is_empty() {
                    ul class="lui-pricing-page-features" {
                        @for f in &t.features { li { (Icon::Check) span { (f) } } }
                    }
                }
            };
            let card = ui.card().title(t.name).body(body);
            let card = match t.blurb {
                Some(b) => card.description(b),
                None => card,
            };
            let card = if t.featured {
                card.header(html! { (ui.badge(ui.text(Text::Popular))) })
            } else {
                card
            };
            let card = match t.cta {
                Some((text, href)) => {
                    let button = ui.link_button(text, href).class("lui-pricing-page-cta");
                    let button = if t.featured { button.primary() } else { button };
                    card.footer(html! { (button) })
                }
                None => card,
            };
            html! { div class="lui-pricing-page-tier" data-featured[t.featured] { (card) } }
        };
        html! {
            div class="lui-pricing-page" {
                div class="lui-pricing-page-head" {
                    h1 { (self.title) }
                    @if let Some(d) = self.description { p class="lui-pricing-page-description" { (d) } }
                }
                (ui.grid("16rem").body(html! { @for t in &self.tiers { (tier(t)) } }))
                (self.body)
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. A centred head, then cards of
/// equal height whose buttons line up at the bottom.
pub const CSS: &str = r#"
.lui-pricing-page { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-8); }
.lui-pricing-page-head { text-align: center; }
.lui-pricing-page-head h1 { margin: 0; }
.lui-pricing-page-description { margin: var(--lui-space-2) 0 0; color: var(--lui-muted); }
.lui-pricing-page-tier, .lui-pricing-page-tier > .lui-card { height: 100%; box-sizing: border-box; }
.lui-pricing-page-tier > .lui-card { display: flex; flex-direction: column; }
.lui-pricing-page-tier .lui-card-body { flex: 1; display: grid; align-content: start; gap: var(--lui-space-4); }
.lui-pricing-page-tier[data-featured] > .lui-card { border-color: var(--lui-primary); box-shadow: var(--lui-shadow-lg); }
/* The badge sits in the corner, so the featured plan's price lines up with the others. */
.lui-pricing-page-tier[data-featured] { position: relative; }
.lui-pricing-page-tier[data-featured] .lui-card-action { position: absolute; inset-block-start: var(--lui-space-4); inset-inline-end: var(--lui-space-4); }
.lui-pricing-page-price { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--lui-space-1); margin: 0; }
.lui-pricing-page-amount { font-size: var(--lui-text-8); font-weight: 600; letter-spacing: -0.025em; line-height: 1.1; }
.lui-pricing-page-period { color: var(--lui-muted); font-size: var(--lui-text-2); }
.lui-pricing-page-features { list-style: none; margin: 0; padding: 0; display: grid; gap: var(--lui-space-2); font-size: var(--lui-text-2); }
.lui-pricing-page-features li { display: flex; align-items: center; gap: var(--lui-space-2); }
.lui-pricing-page-features .lui-icon { flex: none; color: var(--lui-ok); }
.lui-pricing-page-cta { width: 100%; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// Each plan's markup, from its root to the next one.
    fn tiers(m: &str) -> Vec<&str> {
        m.split(r#"<div class="lui-pricing-page-tier""#)
            .skip(1)
            .collect()
    }

    fn plans(ui: &Ui) -> String {
        ui.pricing_page("Plans")
            .tier("Free", "$0")
            .feature("One seat")
            .tier("Team", "$30")
            .period("/month")
            .blurb("For groups.")
            .feature("Ten seats")
            .feature("Audit log")
            .cta("Buy", "/buy")
            .featured()
            .render()
            .into_string()
    }

    #[test]
    fn each_plan_is_a_card_in_one_grid_under_the_h1() {
        let m = plans(&Ui::default());
        assert!(m.starts_with(r#"<div class="lui-pricing-page"><div class="lui-pricing-page-head"><h1>Plans</h1></div><div class="lui-grid""#), "{m}");
        let t = tiers(&m);
        assert_eq!(t.len(), 2, "{m}");
        assert!(t[0].contains(">Free<") && t[1].contains(">Team<"), "{m}");
    }

    #[test]
    fn modifiers_land_on_the_plan_added_last() {
        let m = plans(&Ui::default());
        let t = tiers(&m);
        assert!(
            t[0].contains("One seat") && !t[0].contains("/month") && !t[0].contains("lui-button"),
            "{m}"
        );
        assert!(
            !t[0].contains("data-featured") && !t[0].contains("Popular"),
            "{m}"
        );
        assert!(
            t[1].starts_with(" data-featured>") && t[1].contains("Popular"),
            "{m}"
        );
        assert!(
            t[1].contains("/month")
                && t[1].contains("For groups.")
                && t[1].contains(r#"href="/buy""#),
            "{m}"
        );
        assert_eq!(t[1].matches("<li>").count(), 2, "{m}");
    }

    #[test]
    fn only_the_featured_plan_gets_the_primary_button() {
        let ui = Ui::default();
        let m = ui
            .pricing_page("P")
            .tier("A", "1")
            .cta("Go", "/a")
            .tier("B", "2")
            .cta("Go", "/b")
            .featured()
            .render()
            .into_string();
        let t = tiers(&m);
        assert!(
            t[0].contains("lui-button") && !t[0].contains("lui-button-primary"),
            "{m}"
        );
        assert!(t[1].contains("lui-button-primary"), "{m}");
    }
}
