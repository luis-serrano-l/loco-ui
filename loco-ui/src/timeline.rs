//! # Timeline
//!
//! What happened, in order: a vertical line with a dot (or an icon) per event, each with
//! when it happened, a title and a line of description. After the Origin UI Timeline.
//!
//! **Platform features:** an `<ol>` (baseline since HTML 2); the line is each item's
//! `::before`; a date given with `.datetime(..)` is a `<time datetime>` (Chrome 62, Firefox
//! 22, Safari 7); `:has()` (Chrome 105, Firefox 121, Safari 15.4) starts the line below an
//! icon.
//!
//! **Accessibility:** an ordered list read in order; each event's dot or icon is
//! `aria-hidden`, so the date, title and description are what is read. Checked by axe-core in
//! headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.timeline().event("Founded").when("2019").event("First release").when("2021").render().into_string();
//! assert!(m.contains(r#"<span class="lui-timeline-when">2019</span>"#));
//! // An icon and a tone on an event, a description and a machine-readable date.
//! let m = ui.timeline()
//!     .event("Deployed").when("12 October").datetime("2026-10-12").icon(Icon::Check).tone(Tone::Ok)
//!     .description("Version 0.2 is live.")
//!     .event("Rolled back").when("13 October").tone(Tone::Danger);
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<li data-tone="ok">"#) && html.contains(r#"<time class="lui-timeline-when" datetime="2026-10-12">"#));
//! // The same in `lui!`:
//! let same = lui! { Timeline {
//!     event "Deployed" when="12 October" datetime="2026-10-12" icon=(Icon::Check) tone="ok" description="Version 0.2 is live.";
//!     event "Rolled back" when="13 October" tone="danger";
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::icon::Glyph;
use crate::props::{Prop, PropKind};
use crate::variant::Tone;

/// One event: its title and what is shown with it.
#[derive(Clone, Debug, Default)]
struct Event<'a> {
    title: &'a str,
    when: Option<&'a str>,
    datetime: Option<&'a str>,
    description: Option<&'a str>,
    icon: Option<Glyph<'a>>,
    tone: Option<Tone>,
}

/// Events on a line, made by [`Ui::timeline`].
///
/// **Setters.** Values and items: `.event(..)`, `.when(..)`, `.datetime(..)`,
/// `.description(..)`, `.icon(..)`, `.tone(..)`.
#[derive(Clone, Debug, Default)]
pub struct Timeline<'a> {
    events: Vec<Event<'a>>,
}

impl Timeline<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("event", PropKind::Item, "title: &'a str").doc("An event titled `title`."),
        Prop::new("when", PropKind::Modifier, "text: &'a str")
            .applies_to(&["event"])
            .doc("When the event added last happened, as the reader should see it."),
        Prop::new("datetime", PropKind::Modifier, "iso: &'a str")
            .applies_to(&["event"])
            .attr("datetime")
            .doc(
                "The same date for machines (`2026-10-12`): the date becomes a `<time datetime>`.",
            ),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .applies_to(&["event"])
            .doc("A muted line under the title of the event added last."),
        Prop::new("icon", PropKind::Modifier, "icon: impl Into<Glyph<'a>>")
            .applies_to(&["event"])
            .doc("An icon in a ring on the line instead of the dot, for the event added last."),
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .applies_to(&["event"])
            .doc("The colour of the event added last's dot or icon (neutral by default)."),
    ];
}

impl Ui {
    /// An empty timeline; add events with [`Timeline::event`], oldest or newest first.
    pub fn timeline(&self) -> Timeline<'_> {
        Timeline::default()
    }
}

impl<'a> Timeline<'a> {
    fn last(mut self, method: &str, change: impl FnOnce(&mut Event<'a>)) -> Self {
        let last = self.events.last().map(|_| "event");
        crate::props::check_last::<Self>(Self::PROPS, method, last);
        if let Some(event) = self.events.last_mut() {
            change(event);
        }
        self
    }

    /// An event titled `title`.
    pub fn event(mut self, title: &'a str) -> Self {
        self.events.push(Event {
            title,
            ..Event::default()
        });
        self
    }

    /// When the event added last happened, as the reader should see it ("12 October", "2019").
    pub fn when(self, text: &'a str) -> Self {
        self.last("when", |e| e.when = Some(text))
    }

    /// The same date for machines (`2026-10-12`): the date becomes a `<time datetime>`.
    pub fn datetime(self, iso: &'a str) -> Self {
        self.last("datetime", |e| e.datetime = Some(iso))
    }

    /// A muted line under the title of the event added last.
    pub fn description(self, text: &'a str) -> Self {
        self.last("description", |e| e.description = Some(text))
    }

    /// An icon in a ring on the line instead of the dot, for the event added last.
    pub fn icon(self, icon: impl Into<Glyph<'a>>) -> Self {
        let icon = icon.into();
        self.last("icon", |e| e.icon = Some(icon))
    }

    /// The colour of the event added last's dot or icon (neutral by default).
    pub fn tone(self, tone: Tone) -> Self {
        self.last("tone", |e| e.tone = Some(tone))
    }
}

impl Render for Timeline<'_> {
    fn render(&self) -> Markup {
        html! {
            ol class="lui-timeline" {
                @for e in &self.events {
                    li data-tone=[e.tone.map(Tone::as_str)] {
                        @if let Some(icon) = e.icon {
                            span class="lui-timeline-dot lui-timeline-icon" aria-hidden="true" { (icon) }
                        } @else {
                            span class="lui-timeline-dot" aria-hidden="true" {}
                        }
                        @if let Some(when) = e.when {
                            @if let Some(iso) = e.datetime {
                                time class="lui-timeline-when" datetime=(iso) { (when) }
                            } @else {
                                span class="lui-timeline-when" { (when) }
                            }
                        }
                        p class="lui-timeline-title" { (e.title) }
                        @if let Some(d) = e.description { p class="lui-timeline-description" { (d) } }
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Origin UI Timeline: each event 2rem in from a 2px rail, 3rem apart; a 1rem ring
   (2px, the tone at 20%) on the rail per event, or a 1.5rem icon on a 10% wash; the date
   small, medium and muted above a medium title and a muted description. A tone fills the
   ring (Origin's "completed"). The rail is the tone at 10% and stops at the last event. */
.lui-timeline { list-style: none; margin: 0; padding: 0; }
.lui-timeline > li {
  --lui-timeline-tone: var(--lui-primary);
  position: relative; display: flex; flex-direction: column; gap: 0.125rem; margin-inline-start: 2rem;
  padding-bottom: 3rem; max-width: none; overflow-wrap: anywhere;
}
.lui-timeline > li:last-child { padding-bottom: 0; }
.lui-timeline > li::before {
  content: ""; position: absolute; inset-inline-start: -1.5rem; margin-inline-start: -1px; top: 1.125rem;
  width: 2px; height: calc(100% - 1.25rem); background: color-mix(in srgb, var(--lui-primary) 10%, transparent);
}
.lui-timeline > li:has(> .lui-timeline-icon)::before { top: 1.625rem; height: calc(100% - 1.75rem); }
.lui-timeline > li:last-child::before { display: none; }
.lui-timeline-dot {
  position: absolute; inset-inline-start: -1.5rem; margin-inline-start: -0.5rem; top: 0; box-sizing: border-box;
  width: 1rem; height: 1rem; border-radius: 50%; border: 2px solid color-mix(in srgb, var(--lui-timeline-tone) 20%, transparent);
}
.lui-timeline > li[data-tone] > .lui-timeline-dot { border-color: var(--lui-timeline-tone); }
.lui-timeline-icon {
  display: flex; align-items: center; justify-content: center; margin-inline-start: -0.75rem; width: 1.5rem; height: 1.5rem; border: 0;
  background: color-mix(in srgb, var(--lui-timeline-tone) 10%, transparent); color: var(--lui-timeline-tone);
}
.lui-timeline > li[data-tone] > .lui-timeline-icon { background: var(--lui-timeline-tone); color: var(--lui-bg); }
.lui-timeline-icon > .lui-icon { width: 0.875rem; height: 0.875rem; }
.lui-timeline > li[data-tone=ok] { --lui-timeline-tone: var(--lui-ok); }
.lui-timeline > li[data-tone=warn] { --lui-timeline-tone: var(--lui-warn); }
.lui-timeline > li[data-tone=danger] { --lui-timeline-tone: var(--lui-danger); }
.lui-timeline > li[data-tone=neutral] { --lui-timeline-tone: var(--lui-gray-9); }
.lui-timeline-when { display: block; margin-bottom: var(--lui-space-1); font-size: var(--lui-text-1); line-height: 1rem; font-weight: 500; color: var(--lui-muted); font-variant-numeric: tabular-nums; }
.lui-timeline-title { margin: 0; font-size: var(--lui-text-2); line-height: 1.25rem; font-weight: 500; color: var(--lui-fg); max-width: none; }
.lui-timeline-description { margin: 0; font-size: var(--lui-text-2); line-height: 1.25rem; color: var(--lui-muted); max-width: none; }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Icon;

    #[test]
    fn each_event_is_a_list_item_with_a_hidden_dot_and_a_title() {
        let ui = Ui::default();
        let m = ui
            .timeline()
            .event("First")
            .event("Second")
            .render()
            .into_string();
        assert!(m.starts_with(r#"<ol class="lui-timeline">"#), "{m}");
        assert_eq!(m.matches("<li>").count(), 2, "{m}");
        assert_eq!(
            m.matches(r#"<span class="lui-timeline-dot" aria-hidden="true"></span>"#)
                .count(),
            2,
            "{m}"
        );
        assert!(
            m.contains(r#"<p class="lui-timeline-title">First</p>"#),
            "{m}"
        );
        assert!(
            !m.contains("lui-timeline-when") && !m.contains("lui-timeline-description"),
            "{m}"
        );
    }

    #[test]
    fn when_is_a_span_and_becomes_a_time_with_datetime() {
        let ui = Ui::default();
        let m = ui
            .timeline()
            .event("A")
            .when("2019")
            .event("B")
            .when("12 October")
            .datetime("2026-10-12")
            .render()
            .into_string();
        assert!(
            m.contains(r#"<span class="lui-timeline-when">2019</span>"#),
            "{m}"
        );
        assert!(
            m.contains(
                r#"<time class="lui-timeline-when" datetime="2026-10-12">12 October</time>"#
            ),
            "{m}"
        );
    }

    #[test]
    fn an_icon_replaces_the_dot_and_tone_and_description_are_set_on_that_event_only() {
        let ui = Ui::default();
        let m = ui
            .timeline()
            .event("Done")
            .icon(Icon::Check)
            .tone(Tone::Ok)
            .description("All good.")
            .event("Plain")
            .render()
            .into_string();
        assert!(
            m.contains(r#"<li data-tone="ok">"#) && m.contains("<li>"),
            "{m}"
        );
        assert!(m.contains("lui-timeline-dot lui-timeline-icon"), "{m}");
        assert_eq!(m.matches("lui-timeline-icon").count(), 1, "{m}");
        assert!(
            m.contains(r#"<p class="lui-timeline-description">All good.</p>"#),
            "{m}"
        );
    }
}
