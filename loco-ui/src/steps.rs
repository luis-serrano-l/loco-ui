//! # Steps
//!
//! Where a visitor is in a process of several steps: numbered dots joined by connectors,
//! each step done, current or still to come, a done step a link back to it. A row of dots
//! with the titles under them from 30rem, a column below that (or always, with
//! `.vertical()`). The [wizard](crate::wizard) shows its progress with it. After the Origin
//! UI Stepper.
//!
//! **Platform features:** an `<ol>` (baseline since HTML 2) whose current step carries
//! `aria-current="step"`; the connectors are `::after` boxes; a container query
//! (`@container`, Chrome 105, Firefox 110, Safari 16) on the list turns the column into a
//! row.
//!
//! **Accessibility:** an ordered list, so a screen reader announces "step 2 of 4"; the
//! current step has `aria-current="step"`; a done step is followed by a hidden "completed",
//! a step in error by "has errors"; the dots are `aria-hidden`. Checked by axe-core in
//! headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing; it is a list of links.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.steps().step("Cart").done().step("Address").current().step("Pay").render().into_string();
//! assert!(m.contains(r#"<li class="lui-steps-current" aria-current="step">"#));
//! // A done step that links back, an optional one with a line under it, a column.
//! let m = ui.steps()
//!     .step("Account").done().href("/signup?step=0")
//!     .step("Profile").current().optional().description("A photo and a bio.")
//!     .step("Review")
//!     .vertical();
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<a href="/signup?step=0">Account</a>"#) && html.contains("lui-steps-vertical"));
//! // The same in `lui!`:
//! let same = lui! { Steps vertical {
//!     step "Account" done href="/signup?step=0";
//!     step "Profile" current optional description="A photo and a bio.";
//!     step "Review";
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::{Strings, Text};
use crate::props::{Prop, PropKind};
use crate::{Icon, Ui};

/// How far along a step is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Todo,
    Done,
    Current,
}

/// One step: its title, state and what it shows besides.
#[derive(Clone, Debug, Default)]
struct Step<'a> {
    title: &'a str,
    state: State,
    error: bool,
    optional: bool,
    href: Option<&'a str>,
    description: Option<&'a str>,
}

/// A row (or column) of numbered steps, made by [`Ui::steps`].
///
/// **Setters.** Values and items: `.step(..)`, `.href(..)`, `.description(..)`; switches:
/// `.done()`, `.current()`, `.invalid()`, `.optional()`, `.vertical()`.
#[derive(Clone, Debug)]
pub struct Steps<'a> {
    strings: &'static Strings,
    steps: Vec<Step<'a>>,
    vertical: bool,
}

impl Steps<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("step", PropKind::Item, "title: &'a str").doc("A step titled `title`."),
        Prop::new("done", PropKind::Modifier, "")
            .applies_to(&["step"])
            .doc("The step added last is done: a check in its dot, the connector after it filled."),
        Prop::new("current", PropKind::Modifier, "")
            .applies_to(&["step"])
            .doc("The step added last is the one the visitor is on (`aria-current=\"step\"`)."),
        Prop::new("invalid", PropKind::Modifier, "")
            .applies_to(&["step"])
            .doc("The step added last needs attention: a \"!\" in a danger dot."),
        Prop::new("optional", PropKind::Modifier, "")
            .applies_to(&["step"])
            .doc("The step added last can be skipped: \"Optional\" beside its title."),
        Prop::new("href", PropKind::Modifier, "href: &'a str")
            .applies_to(&["step"])
            .attr("href")
            .doc("The title of the step added last links here."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .applies_to(&["step"])
            .doc("A muted line under the title of the step added last."),
        Prop::new("vertical", PropKind::Switch, "")
            .doc("A column at every width instead of a row from 30rem."),
    ];
}

impl Ui {
    /// An empty list of steps; add them with [`Steps::step`].
    pub fn steps(&self) -> Steps<'_> {
        Steps {
            strings: self.strings,
            steps: Vec::new(),
            vertical: false,
        }
    }
}

impl<'a> Steps<'a> {
    fn last(mut self, method: &str, change: impl FnOnce(&mut Step<'a>)) -> Self {
        let last = self.steps.last().map(|_| "step");
        crate::props::check_last::<Self>(Steps::PROPS, method, last);
        if let Some(step) = self.steps.last_mut() {
            change(step);
        }
        self
    }

    /// A step titled `title`.
    pub fn step(mut self, title: &'a str) -> Self {
        self.steps.push(Step {
            title,
            ..Step::default()
        });
        self
    }

    /// The step added last is done: a check in its dot, the connector after it filled.
    pub fn done(self) -> Self {
        self.last("done", |s| s.state = State::Done)
    }

    /// The step added last is the one the visitor is on (`aria-current="step"`).
    pub fn current(self) -> Self {
        self.last("current", |s| s.state = State::Current)
    }

    /// The step added last needs attention: a "!" in a danger dot, its title in the danger
    /// colour, and "has errors" for assistive tech.
    pub fn invalid(self) -> Self {
        self.last("invalid", |s| s.error = true)
    }

    /// The step added last can be skipped: "Optional" beside its title.
    pub fn optional(self) -> Self {
        self.last("optional", |s| s.optional = true)
    }

    /// The title of the step added last links here (a done step, to go back to it).
    pub fn href(self, href: &'a str) -> Self {
        self.last("href", |s| s.href = Some(href))
    }

    /// A muted line under the title of the step added last.
    pub fn description(self, text: &'a str) -> Self {
        self.last("description", |s| s.description = Some(text))
    }

    /// A column at every width instead of a row from 30rem.
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
}

impl Render for Steps<'_> {
    fn render(&self) -> Markup {
        let t = |text| self.strings.get(text);
        html! {
            ol class={ "lui-steps" @if self.vertical { " lui-steps-vertical" } } {
                @for (i, s) in self.steps.iter().enumerate() {
                    @let class = match (s.state, s.error) {
                        (State::Current, true) => "lui-steps-current lui-steps-error",
                        (State::Current, false) => "lui-steps-current",
                        (State::Done, _) => "lui-steps-done",
                        (State::Todo, true) => "lui-steps-error",
                        (State::Todo, false) => "",
                    };
                    li class=[(!class.is_empty()).then_some(class)] aria-current=[(s.state == State::Current).then_some("step")] {
                        span class="lui-steps-dot" aria-hidden="true" {
                            @if s.error { "!" } @else if s.state == State::Done { (Icon::Check) } @else { (i + 1) }
                        }
                        span class="lui-steps-label" {
                            span class="lui-steps-title" {
                                @if let Some(href) = s.href { a href=(href) { (s.title) } } @else { (s.title) }
                                @if s.optional { " " small { (t(Text::Optional)) } }
                                @if s.state == State::Done { span class="lui-sr" { " " (t(Text::Completed)) } }
                                @if s.error { span class="lui-sr" { (t(Text::HasErrors)) } }
                            }
                            @if let Some(d) = s.description { span class="lui-steps-description" { (d) } }
                        }
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Origin UI Stepper: each step a 2rem dot (its number; a check when done; "!" when
   in error) with the title and "optional" beside or under it, joined by 2px connectors that
   are the progress: primary after a done step, --lui-gray-6 after the rest. The list is its
   own container: a column in a narrow one, a row from 30rem (dots on a line, titles under
   them); .vertical() keeps the column. */
.lui-steps { container: lui-steps / inline-size; display: flex; flex-wrap: wrap; list-style: none; margin: 0; padding: 0; --lui-steps-dot: 2rem; }
.lui-steps > li {
  position: relative; flex: 1 0 100%; display: grid; grid-template-columns: var(--lui-steps-dot) minmax(0, 1fr); align-items: start;
  column-gap: var(--lui-space-3); padding-bottom: var(--lui-space-6); max-width: none; color: var(--lui-muted); font-size: var(--lui-text-2);
}
.lui-steps > li:last-child { padding-bottom: 0; }
.lui-steps-dot {
  display: grid; place-items: center; width: var(--lui-steps-dot); height: var(--lui-steps-dot); border-radius: 50%;
  background: var(--lui-gray-3); color: var(--lui-gray-11); font-size: var(--lui-text-1); font-weight: 600; font-variant-numeric: tabular-nums;
}
.lui-steps-dot > .lui-icon { width: 0.875rem; height: 0.875rem; }
.lui-steps-done .lui-steps-dot, .lui-steps-current .lui-steps-dot { background: var(--lui-primary); color: var(--lui-on-primary); }
.lui-steps-current .lui-steps-dot { box-shadow: 0 0 0 3px color-mix(in srgb, var(--lui-primary) 25%, transparent); }
.lui-steps-error .lui-steps-dot { background: var(--lui-danger); color: var(--lui-on-danger); box-shadow: 0 0 0 3px color-mix(in srgb, var(--lui-danger) 25%, transparent); }
.lui-steps-label { display: grid; align-content: center; min-height: var(--lui-steps-dot); min-width: 0; overflow-wrap: anywhere; }
.lui-steps-title { font-weight: 500; }
.lui-steps-title small { font-size: var(--lui-text-1); font-weight: 400; color: var(--lui-muted); }
.lui-steps-description { font-size: var(--lui-text-1); color: var(--lui-muted); }
.lui-steps-current .lui-steps-title, .lui-steps-done .lui-steps-title { color: var(--lui-fg); }
.lui-steps-error .lui-steps-title { color: var(--lui-danger); }
.lui-steps-title a { color: inherit; text-decoration: none; }
.lui-steps-title a:hover { text-decoration: underline; }
@media (pointer: coarse) { .lui-steps-title a { display: inline-flex; align-items: center; min-height: var(--lui-hit); } }
/* Connectors: down from each dot to the next (narrow), across (wide). */
.lui-steps > li:not(:last-child)::after {
  content: ""; position: absolute; inset-inline-start: calc(var(--lui-steps-dot) / 2 - 1px); top: calc(var(--lui-steps-dot) + var(--lui-space-1));
  bottom: var(--lui-space-1); width: 2px; border-radius: 1px; background: var(--lui-gray-6);
}
.lui-steps > .lui-steps-done::after { background: var(--lui-primary); }
@container lui-steps (width >= 30rem) {
  .lui-steps:not(.lui-steps-vertical) > li { flex: 1 1 0; min-width: 0; grid-template-columns: none; align-content: start; justify-items: center; row-gap: var(--lui-space-2); padding-bottom: 0; text-align: center; }
  .lui-steps:not(.lui-steps-vertical) .lui-steps-label { min-height: 0; align-content: start; padding-inline: var(--lui-space-2); }
  .lui-steps:not(.lui-steps-vertical) > li:not(:last-child)::after {
    top: calc(var(--lui-steps-dot) / 2 - 1px); bottom: auto; width: auto; height: 2px;
    inset-inline-start: calc(50% + var(--lui-steps-dot) / 2 + var(--lui-space-2)); inset-inline-end: calc(-50% + var(--lui-steps-dot) / 2 + var(--lui-space-2));
  }
}
"#;
