//! # Wizard
//!
//! A form split into steps, no script: each step is one `<form method="post">`, the server
//! checks it and either re-renders it with messages beside the fields or stores it and
//! redirects to the next step. Optional steps can be skipped, a progress bar shows how far
//! along the visitor is, the last step is a review whose every value links back to its step,
//! and a visitor who closes the tab comes back to the step they left.
//!
//! The look follows the Origin UI Stepper: the progress is a [`Steps`](crate::steps) list,
//! numbered dots (a check when done) with the title and "optional" beside them, joined by
//! connectors that are the progress on screen (primary after a done step). A column in a
//! narrow container, a row from 30rem.
//!
//! **Platform features:**
//! - Post/Redirect/Get per step: a redirect after each valid POST, so refresh never
//!   re-submits. An invalid POST answers with the same step, the values kept and the
//!   messages beside the fields ([`Wizard::errors`]), and the step list shows it in error.
//! - The current step is a `step.<id>` key in [`crate::UiState`]: it travels in `?step.<id>=n`
//!   and the `lui-ui` cookie, like a tab, so the URL of a step can be shared, Back/Forward in
//!   the browser work, and a bare visit resumes where the cookie says
//!   ([`crate::UiState::remembered`]) with a "Start over" link.
//! - `<ol>` step list with `aria-current="step"` on the current one; done steps are links.
//! - `<fieldset>` + `<legend>` for the step's fields, `<button name="skip" formnovalidate>`
//!   (baseline 2015) to skip an optional step without the browser checking its fields.
//! - `<progress>` (baseline 2015) for the steps done out of the total, read by assistive tech
//!   (visually hidden: the connectors show it); a container query (Chrome 105, Firefox 110,
//!   Safari 16) turns the step column into a row.
//! - [`Wizard::review`] renders the review as a `<dl>` with an "Edit" link per value.
//!
//! **Accessibility:** an ordered list of steps with `aria-current="step"`, a labelled
//! `<progress>`, one `<fieldset>` per step with the step in its `<legend>`, `aria-invalid` on a
//! step with errors. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** keep the step inputs when the browser drops the form
//! on back; state travels in hidden fields and the query.
//!
//! **Fallback:** none needed. Everything is a link or a form.
//!
//! **Finding:** the entered values are the app's data, not UI state, so they do not belong
//! in the URL. The demo keeps them in one cookie; a real app would use its session store.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The request is on step 2 (0-based): the review.
//! let ui = Ui::from_request("/wizard", "step.signup=2", "");
//! let m = ui.wizard("signup", "/wizard")
//!     .step("Account", ui.fields().email("email", "Email").required().value("a@b.c"))
//!     .step("Newsletter", ui.fields().text("topics", "Topics")).optional()
//!     // Every field of the steps before, each with a link back to its step.
//!     .review("Review")
//!     .finish("Create account");
//! let html = m.render().into_string();
//! assert!(html.contains("aria-current=\"step\""));
//! assert!(html.contains("<progress class=\"lui-wizard-progress\" value=\"2\" max=\"2\""));
//! assert!(html.contains("href=\"/wizard?step.signup=0\" aria-label=\"Edit Email\""));
//! assert!(html.contains("(skipped)"), "an empty value");
//! // The same in `lui!`:
//! let same = lui! { Wizard("signup", "/wizard") {
//!     step "Account" (ui.fields().email("email", "Email").required().value("a@b.c"));
//!     step "Newsletter" (ui.fields().text("topics", "Topics")) optional;
//!     review "Review";
//!     finish "Create account";
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::{Ui, enhance, form::Form};

/// What a step shows: fields (listed by the review) or any markup.
#[derive(Clone, Debug)]
enum Body<'a> {
    Fields(Form<'a>),
    Html(Markup),
    Review,
}

/// What [`Wizard::step`] takes: fields from `ui.fields()` or any `Markup`.
#[derive(Clone, Debug)]
pub struct StepBody<'a>(Body<'a>);

/// A step's fields: `ui.fields()…`.
impl<'a> From<Form<'a>> for StepBody<'a> {
    fn from(form: Form<'a>) -> Self {
        StepBody(Body::Fields(form))
    }
}

/// Any markup as a step.
impl From<Markup> for StepBody<'_> {
    fn from(markup: Markup) -> Self {
        StepBody(Body::Html(markup))
    }
}

/// One step: its title in the step list and what it shows.
#[derive(Clone, Debug)]
struct Step<'a> {
    title: &'a str,
    body: Body<'a>,
    optional: bool,
}

/// A form split into steps, made by [`Ui::wizard`]: the current step is `?step.<id>=n` (or
/// the cookie's memory of it). The last button reads "Finish" and a progress bar shows
/// unless told otherwise.
///
/// **Setters.** Values and items: `.values(..)`, `.errors(..)`, `.step(..)`, `.review(..)`,
/// `.at(..)`, `.finish(..)`; switches: `.optional()`, `.hide_progress()`.
#[derive(Clone, Debug)]
pub struct Wizard<'a> {
    ui: &'a Ui,
    id: &'a str,
    action: &'a str,
    steps: Vec<Step<'a>>,
    values: &'a [(String, String)],
    errors: &'a [(&'a str, &'a str)],
    at: Option<usize>,
    finish: &'a str,
    progress: bool,
}

impl Wizard<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves through the step's fields and buttons.",
        ),
        Key::new(
            "Enter",
            "Sends the step from a field, as Next (or Finish) does, also on an optional step.",
        ),
        Key::new(
            "Enter / Space",
            "Presses the focused Back, Skip or Next button.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("step", PropKind::Item, "title: &'a str, body: impl Into<StepBody<'a>>")
            .doc("A step titled `title` showing `body`."),
        Prop::new("review", PropKind::Item, "title: &'a str")
            .doc("The last step."),
        Prop::new("optional", PropKind::Modifier, "")
            .applies_to(&["step"])
            .doc("The step added last can be skipped."),
        Prop::new("values", PropKind::Value, "values: &'a [(String, String)]")
            .doc("What the visitor entered so far, by field name, for every step's fields and the review."),
        Prop::new("at", PropKind::Number, "n: usize")
            .doc("Show step `n` whatever the request says."),
        Prop::new("errors", PropKind::Value, "errors: &'a [(&'a str, &'a str)]")
            .doc("Server messages `(field name, message)` for the posted step."),
        Prop::new("finish", PropKind::Value, "label: &'a str").default("Finish")
            .doc("Label of the last step's submit button."),
        Prop::new("hide_progress", PropKind::Switch, "")
            .doc("No `<progress>` element (the connectors still show progress on screen)."),
    ];
}

impl Ui {
    /// A wizard `id` whose steps post to `action`; add steps with [`Wizard::step`].
    pub fn wizard<'a>(&'a self, id: &'a str, action: &'a str) -> Wizard<'a> {
        Wizard {
            ui: self,
            id,
            action,
            steps: Vec::new(),
            values: &[],
            errors: &[],
            at: None,
            finish: self.text(Text::Finish),
            progress: true,
        }
    }
}

impl<'a> Wizard<'a> {
    /// A step titled `title` showing `body`: `ui.fields()…` (which the review lists) or any
    /// markup.
    pub fn step(mut self, title: &'a str, body: impl Into<StepBody<'a>>) -> Self {
        self.steps.push(Step {
            title,
            body: body.into().0,
            optional: false,
        });
        self
    }

    /// The last step: every field of the steps before it as a `<dl>`, each value with an
    /// "Edit" link to its step; an empty value reads "(skipped)".
    pub fn review(mut self, title: &'a str) -> Self {
        self.steps.push(Step {
            title,
            body: Body::Review,
            optional: false,
        });
        self
    }

    /// The step added last can be skipped: a "Skip" button beside Next posts `skip=1`.
    #[track_caller]
    pub fn optional(mut self) -> Self {
        let last = (self.steps.last()).map(|s| {
            if matches!(s.body, Body::Review) {
                "review"
            } else {
                "step"
            }
        });
        crate::props::check_last::<Self>(Wizard::PROPS, "optional", last);
        if let Some(s) = self.steps.last_mut() {
            s.optional = true;
        }
        self
    }

    /// What the visitor entered so far, by field name, for every step's fields and the
    /// review: the pairs the posts carried, usually kept in a `Saved` cookie (feature `axum`).
    pub fn values(mut self, values: &'a [(String, String)]) -> Self {
        self.values = values;
        self
    }

    /// Show step `n` whatever the request says: the posted step, when a handler answers an
    /// invalid post with the same step and its [`Wizard::errors`].
    pub fn at(mut self, n: usize) -> Self {
        self.at = Some(n);
        self
    }

    /// Server messages `(field name, message)` for the posted step: shown beside the fields,
    /// and the step is marked in error.
    pub fn errors(mut self, errors: &'a [(&'a str, &'a str)]) -> Self {
        self.errors = errors;
        self
    }

    /// Label of the last step's submit button.
    pub fn finish(mut self, label: &'a str) -> Self {
        self.finish = label;
        self
    }

    /// No `<progress>` element: it is read by assistive tech only, since the connectors between
    /// the steps show progress on screen.
    pub fn hide_progress(mut self) -> Self {
        self.progress = false;
        self
    }

    /// The step this request shows, 0-based.
    pub fn current(&self) -> usize {
        self.at
            .unwrap_or_else(|| self.ui.state.step(self.id))
            .min(self.steps.len().saturating_sub(1))
    }

    /// The link to `step`, keeping the rest of the page's state: where a handler redirects
    /// after a valid post.
    pub fn link(&self, step: usize) -> String {
        self.ui
            .state
            .link(&format!("step.{}", self.id), &step.to_string())
    }

    /// Whether `step` is the last one.
    pub fn is_last(&self, step: usize) -> bool {
        step + 1 >= self.steps.len()
    }

    /// A step's fields with the wizard's values and messages filled in.
    fn filled(&self, form: &Form<'a>) -> Form<'a> {
        let form = form.clone().errors(self.errors);
        if self.values.is_empty() {
            form
        } else {
            form.values(self.values)
        }
    }

    fn review_list(&self, upto: usize) -> Markup {
        html! {
            dl class="lui-wizard-review" {
                @for (i, step) in self.steps[..upto].iter().enumerate() {
                    @if let Body::Fields(form) = &step.body {
                        @for g in self.filled(form).filled() {
                            @for f in g.fields.iter().filter(|f| f.shown()) {
                                dt { (f.label) }
                                dd {
                                    @if f.value.is_empty() { span class="lui-note" { (self.ui.text(Text::Skipped)) } } @else { (f.value) }
                                    " " a class="lui-wizard-edit" href=(self.link(i)) aria-label=(self.ui.fill(Text::EditValue, &[&f.label])) { (self.ui.text(Text::Edit)) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Render for Wizard<'_> {
    fn render(&self) -> Markup {
        let Wizard {
            ui,
            id,
            action,
            ref steps,
            finish,
            progress,
            ..
        } = *self;
        if steps.is_empty() {
            return html! {};
        }
        let state = &ui.state;
        let key = format!("step.{id}");
        let current = self.current();
        let last = self.is_last(current);
        let step = &steps[current];
        let failed = |i: usize| {
            i == current && matches!(&steps[i].body, Body::Fields(f) if self.filled(f).has_errors())
        };
        // The step list is a `Steps`: done steps link back, the current one may be in error.
        let links: Vec<String> = (0..current).map(|i| self.link(i)).collect();
        let mut list = ui.steps();
        for (i, s) in steps.iter().enumerate() {
            list = list.step(s.title);
            list = match i.cmp(&current) {
                std::cmp::Ordering::Less => list.done().href(&links[i]),
                std::cmp::Ordering::Equal if failed(i) => list.current().invalid(),
                std::cmp::Ordering::Equal => list.current(),
                std::cmp::Ordering::Greater => list,
            };
            if s.optional {
                list = list.optional();
            }
        }
        let progress_steps = list.render();
        html! {
            div id=(enhance::swap_id("lui-wizard", id)) data-lui="swap" class="lui-wizard" {
                @if current > 0 && state.remembered(&key) {
                    p class="lui-wizard-resume" role="status" {
                        (ui.fill(Text::Resumed, &[&(current + 1)])) " "
                        a href=(self.link(0)) { (ui.text(Text::StartOver)) }
                    }
                }
                (progress_steps)
                @if progress {
                    progress class="lui-wizard-progress" value=(current) max=(steps.len().saturating_sub(1).max(1)) aria-label=(ui.text(Text::Progress)) {
                        (ui.fill(Text::StepsDone, &[&current, &steps.len().saturating_sub(1)]))
                    }
                }
                form method="post" action=(action) class="lui-wizard-form" {
                    input type="hidden" name="step" value=(current);
                    // Enter in a field means Next (or Finish), not the Skip before it.
                    (crate::button::default_submit(None))
                    fieldset aria-invalid=[failed(current).then_some("true")] {
                        legend { (ui.fill(Text::StepOf, &[&(current + 1), &steps.len()])) ": " (step.title) @if step.optional { " " (ui.text(Text::Optional)) } }
                        @match &step.body {
                            Body::Fields(form) => (self.filled(form)),
                            Body::Html(markup) => (markup),
                            Body::Review => (self.review_list(current)),
                        }
                    }
                    p class="lui-wizard-actions" {
                        @if current > 0 {
                            @let back = self.link(current - 1).to_string();
                            (ui.link_button(ui.text(Text::Back), &back).ghost().class("lui-wizard-back"))
                        }
                        @if step.optional && !last { (ui.button(ui.text(Text::Skip)).ghost().name("skip").value("1").formnovalidate()) }
                        (ui.button(if last { finish } else { ui.text(Text::Next) }).primary())
                    }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Origin UI Stepper: the step list is a `Steps` (its CSS is in steps.rs), with
   room under it before the form. The <progress> element stays for assistive tech only;
   .hide_progress() drops it. */
.lui-wizard { container: lui-wizard / inline-size; }
.lui-wizard > .lui-steps { margin-bottom: var(--lui-space-6); }
.lui-wizard-progress { position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
.lui-wizard-resume { padding: 0.75rem 1rem; font-size: var(--lui-text-2); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); background: var(--lui-card); max-width: none; }
.lui-wizard-form fieldset { min-width: 0; margin: 0; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg); padding: var(--lui-space-4); }
@container lui-wizard (width >= 30rem) { .lui-wizard-form fieldset { padding: var(--lui-space-6); } }
.lui-wizard-form fieldset[aria-invalid=true] { border-color: var(--lui-danger); }
.lui-wizard-form legend { padding: 0 0.5rem; font-weight: 600; }
.lui-wizard-form label { display: block; margin: 0.75rem 0; font-size: var(--lui-text-2); }
.lui-wizard-form .lui-field { max-width: 24rem; }
.lui-wizard-actions { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-2); margin-top: var(--lui-space-4); }
.lui-wizard-review { margin: 0; }
.lui-wizard-review dt { color: var(--lui-muted); }
.lui-wizard-review dd { margin: 0 0 0.5rem; }
.lui-wizard-edit { margin-inline-start: var(--lui-space-2); font-size: var(--lui-text-2); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// The attributes of the first submit button in `m`: what Enter in a field presses.
    fn first_submit(m: &str) -> &str {
        let at = m.find("<button type=\"submit\"").expect("a submit button");
        &m[at..at + m[at..].find('>').unwrap()]
    }

    #[test]
    fn enter_on_an_optional_step_means_next() {
        let ui = Ui::from_request("/w", "step.x=1", "");
        let m = three(&ui).render().into_string();
        assert!(m.contains(r#"name="skip""#), "{m}");
        assert!(!first_submit(&m).contains("name="), "{m}");
    }

    fn three<'a>(ui: &'a Ui) -> Wizard<'a> {
        ui.wizard("x", "/w")
            .step("A", html! {})
            .step("B", ui.fields().text("b", "B"))
            .optional()
            .step("C", html! {})
    }

    #[test]
    fn step_list_marks_done_current_and_todo() {
        let ui = Ui::from_request("/w", "step.x=1", "");
        let m = three(&ui).finish("Done").render().into_string();
        assert!(
            m.contains("<a href=\"/w?step.x=0\">A</a>") && m.contains("class=\"lui-steps-done\""),
            "{m}"
        );
        assert!(m.contains(
            "aria-current=\"step\"><span class=\"lui-steps-dot\" aria-hidden=\"true\">2</span>"
        ));
        assert!(m.contains("value=\"1\"") && m.contains(">Next<") && !m.contains(">Done<"));
        assert!(
            !m.contains("lui-wizard-resume"),
            "the step came from the query"
        );
        let end = Ui::from_request("/w", "step.x=9", "");
        let m = three(&end).finish("Done").render().into_string();
        assert!(m.contains(">Done<") && m.contains("href=\"/w?step.x=1\">Back<"));
        assert_eq!(
            (three(&end).current(), three(&end).link(2)),
            (2, "/w?step.x=2".to_string())
        );
    }

    #[test]
    fn errors_skip_and_resume() {
        let ui = Ui::from_request("/w", "", "lui-ui=step.x=1");
        let errors = [("b", "Say more.")];
        let m = three(&ui).errors(&errors).render().into_string();
        assert!(
            m.contains("class=\"lui-steps-current lui-steps-error\" aria-current=\"step\""),
            "{m}"
        );
        assert!(m.contains("<fieldset aria-invalid=\"true\">") && m.contains("Say more."));
        assert!(m.contains("name=\"skip\" value=\"1\" formnovalidate"));
        assert!(
            m.contains("class=\"lui-wizard-resume\"")
                && m.contains("href=\"/w?step.x=0\">Start over")
        );
        assert!(
            m.contains("value=\"1\" max=\"2\""),
            "progress: one of two steps done"
        );
        assert!(
            !three(&ui)
                .hide_progress()
                .render()
                .into_string()
                .contains("<progress")
        );
    }
}
