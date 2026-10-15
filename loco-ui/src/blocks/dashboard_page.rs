//! # Dashboard page
//!
//! A page of numbers: a title and a line, a row of stats that wraps to the width available,
//! and whatever goes under them (a table, a chart, recent activity).
//!
//! **Platform features:** those of [`crate::stat`]: an auto-fit grid, no media query.
//!
//! **Accessibility:** the title is the page's `<h1>`; each stat says its change in words.
//! Checked by axe-core with the demo routes.
//!
//! **What it does not do without script:** update live; the numbers are as fresh as the page.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.dashboard_page("Overview").description("Last 30 days.")
//!     .stat(ui.stat("Revenue", "$12,480").delta("+8%"))
//!     .stat(ui.stat("Customers", "312"))
//!     .body(html! { p { "Recent orders" } })
//!     .render().into_string();
//! assert!(m.contains("lui-stat-grid") && m.contains("$12,480") && m.contains("Recent orders"));
//! // The same in `lui!`:
//! let same = lui! { DashboardPage("Overview") description="Last 30 days." {
//!     stat (ui.stat("Revenue", "$12,480").delta("+8%"));
//!     stat (ui.stat("Customers", "312"));
//!     body (html! { p { "Recent orders" } });
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};
use crate::stat::Stat;

/// A page of stats, made by [`Ui::dashboard_page`].
///
/// **Setters.** Values and items: `.description(..)`, `.stat(..)`, `.body(..)`.
#[derive(Clone, Debug)]
pub struct DashboardPage<'a> {
    title: &'a str,
    description: Option<&'a str>,
    stats: Vec<Stat<'a>>,
    body: Markup,
}

impl DashboardPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("description", PropKind::Value, "text: &'a str").doc("A line under the title."),
        Prop::new("stat", PropKind::Item, "stat: Stat<'a>").doc("A number in the row of stats."),
        Prop::new("body", PropKind::Value, "body: Markup").doc("What goes under the stats."),
    ];
}

impl Ui {
    /// A dashboard titled `title`.
    pub fn dashboard_page<'a>(&self, title: &'a str) -> DashboardPage<'a> {
        DashboardPage {
            title,
            description: None,
            stats: Vec::new(),
            body: Markup::default(),
        }
    }
}

impl<'a> DashboardPage<'a> {
    /// A line under the title.
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// A number in the row of stats.
    pub fn stat(mut self, stat: Stat<'a>) -> Self {
        self.stats.push(stat);
        self
    }

    /// What goes under the stats.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }
}

impl Render for DashboardPage<'_> {
    fn render(&self) -> Markup {
        html! {
            div class="lui-dashboard" {
                h1 { (self.title) }
                @if let Some(d) = self.description { p class="lui-dashboard-description" { (d) } }
                @if !self.stats.is_empty() { div class="lui-stat-grid" { @for s in &self.stats { (s) } } }
                (self.body)
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-dashboard h1 { margin: 0; }
.lui-dashboard-description { margin: 0.25rem 0 0; color: var(--lui-muted); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_dashboard_is_a_root_with_an_h1_and_no_stat_grid() {
        let ui = Ui::default();
        let m = ui.dashboard_page("Overview").render().into_string();
        assert_eq!(
            m, r#"<div class="lui-dashboard"><h1>Overview</h1></div>"#,
            "{m}"
        );
    }

    #[test]
    fn stats_are_rendered_in_order_inside_one_grid() {
        let ui = Ui::default();
        let m = ui
            .dashboard_page("O")
            .stat(ui.stat("Revenue", "$12,480"))
            .stat(ui.stat("Customers", "312"))
            .render()
            .into_string();
        assert_eq!(m.matches(r#"class="lui-stat-grid""#).count(), 1, "{m}");
        let (a, b) = (m.find("$12,480").unwrap(), m.find("312").unwrap());
        assert!(a < b, "{m}");
        assert!(m.find("lui-stat-grid").unwrap() < a, "{m}");
    }

    #[test]
    fn description_follows_the_title_and_the_body_comes_after_the_stats() {
        let ui = Ui::default();
        let m = ui
            .dashboard_page("O")
            .description("Last 30 days.")
            .stat(ui.stat("Users", "9"))
            .body(html! { p { "Recent orders" } })
            .render()
            .into_string();
        assert!(
            m.contains(r#"<p class="lui-dashboard-description">Last 30 days.</p>"#),
            "{m}"
        );
        let (h1, d, grid, body) = (
            m.find("<h1>").unwrap(),
            m.find("Last 30 days.").unwrap(),
            m.find("lui-stat-grid").unwrap(),
            m.find("Recent orders").unwrap(),
        );
        assert!(h1 < d && d < grid && grid < body, "{m}");
    }
}
