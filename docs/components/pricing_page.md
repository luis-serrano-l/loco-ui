# PricingPage

`loco_ui::blocks::pricing_page` · demo: `/blocks/pricing`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `PricingPage` | `ui.pricing_page(title: &str)` | beta |

## Props of `PricingPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `description` | value | `text: &'a str` |  |  | A line under the title. |
| `tier` | item | `name: &'a str, price: &'a str` |  |  | A plan, after the ones added before it: its name and its price, formatted. |
| `period` | modifier | `text: &'a str` |  |  | What the last plan's price is per (`/month`). |
| `blurb` | modifier | `text: &'a str` |  |  | Who the last plan is for, under its name. |
| `feature` | modifier | `text: &'a str` |  |  | One thing the last plan includes; call once per line. |
| `cta` | modifier | `text: &'a str, href: &'a str` |  |  | The last plan's button: its text and where it goes. |
| `featured` | modifier |  |  |  | The plan to steer people to: a badge, a stronger border, the primary button. |
| `body` | value | `body: Markup` |  |  | What goes under the plans. |

## Keyboard of `PricingPage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Enter` | Moves between the plans' buttons and follows one. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-pricing-page` | `--lui-space-8` |
| `.lui-pricing-page-head` |  |
| `.lui-pricing-page-description` | `--lui-space-2`, `--lui-muted` |
| `.lui-pricing-page-tier` |  |
| `.lui-card-action` | `--lui-space-4` |
| `.lui-pricing-page-price` | `--lui-space-1` |
| `.lui-pricing-page-amount` | `--lui-text-8` |
| `.lui-pricing-page-period` | `--lui-muted`, `--lui-text-2` |
| `.lui-pricing-page-features` | `--lui-space-2`, `--lui-text-2` |
| `.lui-pricing-page-cta` |  |

## Documentation

A title, a line, and the plans side by side: each a card with its name, price, who it is
for, what it includes and a button. One plan can be featured: a "Popular" badge, a
stronger border and the primary button. Fine print or questions go under them. Copied
from the demo's own pricing card (`demo/src/pricing.rs`, the example of a component
written outside the library), after the shadcn/ui pricing blocks.

**Platform features:** those of `crate::card` and `crate::grid`: a CSS grid with
`repeat(auto-fill, minmax(min(16rem, 100%), 1fr))` (`min()`: Chrome 79, Firefox 75,
Safari 11.1), no media query; the plans wrap to one column in a narrow box.

**Accessibility:** the title is the page's `<h1>`; each plan's name is its card's heading;
the feature list is a `<ul>` whose check marks are decorative. Checked by axe-core with
the demo routes.

**What it does not do without script:** switch between monthly and yearly prices in place;
link to the page again with a query (`?period=year`) and fill in the other prices.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.pricing_page("Pricing").description("Start free, upgrade when you grow.")
    .tier("Hobby", "$0").period("/month").feature("One project").cta("Start", "/signup")
    .tier("Pro", "$12").period("/month").blurb("For small teams.")
        .feature("Ten projects").feature("Email support").cta("Upgrade", "/signup?plan=pro").featured()
    .render().into_string();
assert!(m.contains(r#"<h1>Pricing</h1>"#) && m.contains(r#"data-featured"#));
assert_eq!(m.matches("lui-pricing-page-tier").count(), 2);

// The same in `lui!`:
let same = lui! { PricingPage("Pricing") description="Start free, upgrade when you grow." {
    tier "Hobby" "$0" period="/month" feature="One project" cta=("Start", "/signup");
    tier "Pro" "$12" period="/month" blurb="For small teams."
        feature="Ten projects" feature="Email support" cta=("Upgrade", "/signup?plan=pro") featured;
} };
assert_eq!(same.into_string(), m);
```
