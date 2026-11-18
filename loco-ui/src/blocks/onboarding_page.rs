//! # Onboarding page
//!
//! The first-run page of an app: where the person is ("Step 2 of 3"), a title and a line, the
//! steps as a row, the current step's form in a card, and a way to skip. Each step is a page
//! of its own, so Back and a reload keep the place. After the Origin UI Stepper in an
//! onboarding card, built from [`crate::steps`] and [`crate::card`].
//!
//! **Platform features:** those of [`crate::steps`] (an ordered list, `aria-current="step"`,
//! a container query that turns the row into a column under 30rem); the page is a column
//! capped at a reading width.
//!
//! **Accessibility:** the title is the page's `<h1>`, with the position in words before it;
//! finished steps with an `href` are links back; Skip is a link. Checked by axe-core with the
//! demo routes.
//!
//! **What it does not do without script:** nothing is missing; each step is a round trip.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.onboarding_page("Set up your workspace")
//!     .step("Profile").step("Team").step("Invite")
//!     .body(html! { p { "form" } }).render().into_string();
//! assert!(m.contains(r#"<p class="lui-onboarding-progress">Step 1 of 3</p><h1>Set up your workspace</h1>"#));
//! // The second step, with a way back and a way out:
//! let m = ui.onboarding_page("Set up your workspace").description("About two minutes.")
//!     .at(1).step("Profile").href("/welcome/1").step("Team").step("Invite")
//!     .skip("/app").body(html! { p { "form" } }).render().into_string();
//! assert!(m.contains("Step 2 of 3") && m.contains(r#"<a href="/welcome/1">Profile</a>"#));
//! assert!(m.contains(r#"<a class="lui-onboarding-skip" href="/app">Skip</a>"#));
//! // The same in `lui!`:
//! let same = lui! { OnboardingPage("Set up your workspace") description="About two minutes." at=1 skip="/app" {
//!     step "Profile" href="/welcome/1"; step "Team"; step "Invite";
//!     body { p { "form" } }
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};

/// A step: its title and a link back to it.
#[derive(Clone, Debug)]
struct Step<'a> {
    title: &'a str,
    href: Option<&'a str>,
}

/// A first-run page, made by [`Ui::onboarding_page`].
///
/// **Setters.** Values and items: `.description(..)`, `.at(..)`, `.step(..)`, `.href(..)`,
/// `.skip(..)`, `.body(..)`.
#[derive(Clone, Debug)]
pub struct OnboardingPage<'a> {
    ui: &'a Ui,
    title: &'a str,
    description: Option<&'a str>,
    at: usize,
    steps: Vec<Step<'a>>,
    skip: Option<&'a str>,
    body: Markup,
}

impl OnboardingPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[Key::new(
        "Enter",
        "Follows a finished step's link or the Skip link.",
    )];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("description", PropKind::Value, "text: &'a str").doc("A line under the title."),
        Prop::new("at", PropKind::Number, "n: usize")
            .default("0")
            .doc("The step shown, counting from 0; the ones before it are done. One past the last shows every step done."),
        Prop::new("step", PropKind::Item, "title: &'a str")
            .doc("A step, after the ones added before it."),
        Prop::new("href", PropKind::Modifier, "href: &'a str")
            .applies_to(&["step"])
            .attr("href")
            .doc("The last step's page: its title is a link once the step is done."),
        Prop::new("skip", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("A Skip link under the card, to the app."),
        Prop::new("body", PropKind::Value, "body: Markup")
            .doc("The current step: its form, in a card."),
    ];
}

impl Ui {
    /// A first-run page titled `title`.
    pub fn onboarding_page<'a>(&'a self, title: &'a str) -> OnboardingPage<'a> {
        OnboardingPage {
            ui: self,
            title,
            description: None,
            at: 0,
            steps: Vec::new(),
            skip: None,
            body: Markup::default(),
        }
    }
}

impl<'a> OnboardingPage<'a> {
    /// A line under the title.
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// A step, after the ones added before it.
    pub fn step(mut self, title: &'a str) -> Self {
        self.steps.push(Step { title, href: None });
        self
    }

    /// The step shown, counting from 0; the ones before it are done. One past the last shows
    /// every step done.
    pub fn at(mut self, n: usize) -> Self {
        self.at = n;
        self
    }

    /// The last step's page: its title is a link once the step is done.
    #[track_caller]
    pub fn href(mut self, href: &'a str) -> Self {
        crate::props::check_last::<Self>(
            OnboardingPage::PROPS,
            "href",
            self.steps.last().map(|_| "step"),
        );
        if let Some(s) = self.steps.last_mut() {
            s.href = Some(href);
        }
        self
    }

    /// A Skip link under the card, to the app.
    pub fn skip(mut self, href: &'a str) -> Self {
        self.skip = Some(href);
        self
    }

    /// The current step: its form, in a card.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }
}

impl Render for OnboardingPage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let at = (self.at < self.steps.len()).then_some(self.at);
        let steps = (self.steps.iter().enumerate()).fold(ui.steps(), |steps, (i, s)| {
            let step = steps.step(s.title);
            if i == self.at {
                step.current()
            } else if i < self.at {
                let step = step.done();
                match s.href {
                    Some(href) => step.href(href),
                    None => step,
                }
            } else {
                step
            }
        });
        html! {
            div class="lui-onboarding" {
                div class="lui-onboarding-head" {
                    @if let Some(at) = at {
                        p class="lui-onboarding-progress" { (ui.fill(Text::StepOf, &[&(at + 1), &self.steps.len()])) }
                    }
                    h1 { (self.title) }
                    @if let Some(d) = self.description { p class="lui-onboarding-description" { (d) } }
                }
                @if !self.steps.is_empty() { (steps) }
                (ui.card().body(html! { (self.body) }))
                @if let Some(href) = self.skip {
                    a class="lui-onboarding-skip" href=(href) { (ui.text(Text::Skip)) }
                }
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. One column at every width (the head is centred from 30rem),
/// capped at a reading width and centred; the steps turn into a column by themselves.
pub const CSS: &str = r#"
.lui-onboarding { container: lui-onboarding / inline-size; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-6); width: 100%; max-width: 40rem; margin-inline: auto; }
.lui-onboarding-head h1 { margin: 0; overflow-wrap: anywhere; }
.lui-onboarding-progress { margin: 0 0 var(--lui-space-1); color: var(--lui-muted); font-size: var(--lui-text-2); font-weight: 500; }
.lui-onboarding-description { margin: var(--lui-space-1) 0 0; color: var(--lui-muted); }
.lui-onboarding-skip { justify-self: center; display: inline-flex; align-items: center; color: var(--lui-muted); font-size: var(--lui-text-2); }
.lui-onboarding-skip:hover { color: var(--lui-fg); }
@media (pointer: coarse) { .lui-onboarding-skip { min-height: var(--lui-hit); } }
@container lui-onboarding (width >= 30rem) { .lui-onboarding-head { text-align: center; } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_page_is_a_title_and_a_card_with_no_progress() {
        let ui = Ui::default();
        let m = ui
            .onboarding_page("Welcome")
            .body(html! { i { "form" } })
            .render()
            .into_string();
        assert!(
            m.starts_with(
                r#"<div class="lui-onboarding"><div class="lui-onboarding-head"><h1>Welcome</h1></div><div class="lui-card""#
            ),
            "{m}"
        );
        assert!(m.contains("<i>form</i>"), "{m}");
        assert!(
            !m.contains("lui-steps") && !m.contains("lui-onboarding-skip"),
            "{m}"
        );
    }

    #[test]
    fn the_step_at_n_is_current_and_the_ones_before_it_done() {
        let ui = Ui::default();
        let page = |at| {
            (ui.onboarding_page("W").at(at).step("A").step("B").step("C"))
                .render()
                .into_string()
        };
        assert!(page(0).contains("Step 1 of 3"), "{}", page(0));
        let last = page(2);
        assert!(last.contains("Step 3 of 3"), "{last}");
        assert_eq!(last.matches(r#"aria-current="step""#).count(), 1, "{last}");
        let all = page(3);
        assert!(
            !all.contains("lui-onboarding-progress") && !all.contains("aria-current"),
            "{all}"
        );
    }

    #[test]
    fn only_finished_steps_link_back() {
        let ui = Ui::default();
        let m = ui
            .onboarding_page("W")
            .at(1)
            .step("A")
            .href("/1")
            .step("B")
            .href("/2")
            .step("C")
            .href("/3")
            .render()
            .into_string();
        assert!(m.contains(r#"<a href="/1">A</a>"#), "{m}");
        assert!(
            !m.contains(r#"href="/2""#) && !m.contains(r#"href="/3""#),
            "{m}"
        );
    }
}
