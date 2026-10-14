//! # Timeline
//!
//! What happened, in order: a vertical line with a dot (or an icon) per event, each with
//! when it happened, a title and a line of description. From 30rem the dates move into a
//! column of their own left of the line. After the Flowbite Timeline.
//!
//! **Platform features:** an `<ol>` (baseline since HTML 2); the line is each item's
//! `::before`; a date given with `.datetime(..)` is a `<time datetime>` (Chrome 62, Firefox
//! 22, Safari 7); a container query (`@container`, Chrome 105, Firefox 110, Safari 16) adds
//! the date column from 30rem.
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
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("event", PropKind::Item, "title: &'a str").doc("An event titled `title`."),
        Prop::new("when", PropKind::Modifier, "text: &'a str")
            .doc("When the event added last happened, as the reader should see it."),
        Prop::new("datetime", PropKind::Modifier, "iso: &'a str")
            .attr("datetime")
            .doc("The same date for machines (`2026-10-12`): the date becomes a `<time datetime>`."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .doc("A muted line under the title of the event added last."),
        Prop::new("icon", PropKind::Modifier, "icon: impl Into<Glyph<'a>>")
            .doc("An icon in a ring on the line instead of the dot, for the event added last."),
        Prop::new("tone", PropKind::Modifier, "tone: Tone")
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
    fn last(mut self, change: impl FnOnce(&mut Event<'a>)) -> Self {
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
        self.last(|e| e.when = Some(text))
    }

    /// The same date for machines (`2026-10-12`): the date becomes a `<time datetime>`.
    pub fn datetime(self, iso: &'a str) -> Self {
        self.last(|e| e.datetime = Some(iso))
    }

    /// A muted line under the title of the event added last.
    pub fn description(self, text: &'a str) -> Self {
        self.last(|e| e.description = Some(text))
    }

    /// An icon in a ring on the line instead of the dot, for the event added last.
    pub fn icon(self, icon: impl Into<Glyph<'a>>) -> Self {
        let icon = icon.into();
        self.last(|e| e.icon = Some(icon))
    }

    /// The colour of the event added last's dot or icon (neutral by default).
    pub fn tone(self, tone: Tone) -> Self {
        self.last(|e| e.tone = Some(tone))
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
/* After the Flowbite Timeline: a 1px line at the start, a 0.75rem dot on it per event (or a
   1.5rem icon in a ring of the background), the date small and muted above a semibold title
   and a muted description, 2.5rem between events. Its own container: from 30rem the dates
   sit in an 8rem column, right-aligned, left of the line. */
.lui-timeline { container: lui-timeline / inline-size; list-style: none; margin: 0; padding: 0; --lui-timeline-dot: var(--lui-gray-6); }
.lui-timeline > li {
  position: relative; display: grid; grid-template-columns: 1.5rem minmax(0, 1fr); column-gap: var(--lui-space-3);
  padding-bottom: var(--lui-space-8); max-width: none; overflow-wrap: anywhere;
}
.lui-timeline > li:last-child { padding-bottom: 0; }
/* The line: from under this event's dot down to the next one. */
.lui-timeline > li::before {
  content: ""; position: absolute; inset-inline-start: calc(0.75rem - 0.5px); top: 0; bottom: 0; width: 1px; background: var(--lui-line);
}
.lui-timeline > li:last-child::before { bottom: auto; height: 0.75rem; }
.lui-timeline > li:first-child::before { top: 0.75rem; }
.lui-timeline > li:only-child::before { display: none; }
.lui-timeline > li > * { grid-column: 2; }
.lui-timeline-dot {
  grid-column: 1 !important; grid-row: 1 / span 3; position: relative; z-index: 1; justify-self: center; align-self: start;
  width: 0.75rem; height: 0.75rem; margin-top: 0.375rem; border-radius: 50%;
  background: var(--lui-timeline-dot); box-shadow: 0 0 0 3px var(--lui-bg);
}
.lui-timeline-icon {
  display: grid; place-items: center; width: 1.5rem; height: 1.5rem; margin-top: 0; font-size: 0.75rem;
  background: color-mix(in srgb, var(--lui-timeline-dot) 18%, var(--lui-bg)); color: var(--lui-timeline-dot);
  box-shadow: 0 0 0 4px var(--lui-bg);
}
.lui-timeline-icon > .lui-icon { width: 0.875rem; height: 0.875rem; }
.lui-timeline > li[data-tone=brand], .lui-timeline > li[data-tone=info] { --lui-timeline-dot: var(--lui-primary); }
.lui-timeline > li[data-tone=ok] { --lui-timeline-dot: var(--lui-ok); }
.lui-timeline > li[data-tone=warn] { --lui-timeline-dot: var(--lui-warn); }
.lui-timeline > li[data-tone=danger] { --lui-timeline-dot: var(--lui-danger); }
.lui-timeline > li[data-tone=neutral] { --lui-timeline-dot: var(--lui-gray-9); }
.lui-timeline-when { margin-bottom: var(--lui-space-1); font-size: 0.875rem; line-height: 1.5rem; color: var(--lui-muted); font-variant-numeric: tabular-nums; }
.lui-timeline-title { margin: 0; font-size: 1rem; line-height: 1.5rem; font-weight: 600; color: var(--lui-fg); max-width: none; }
.lui-timeline-description { margin: var(--lui-space-1) 0 0; font-size: 0.875rem; color: var(--lui-muted); max-width: none; }
@container lui-timeline (width >= 30rem) {
  .lui-timeline > li { grid-template-columns: 8rem 1.5rem minmax(0, 1fr); }
  .lui-timeline > li::before { inset-inline-start: calc(8rem + var(--lui-space-3) + 0.75rem - 0.5px); }
  .lui-timeline > li > * { grid-column: 3; }
  .lui-timeline-dot { grid-column: 2 !important; }
  .lui-timeline > li > .lui-timeline-when { grid-column: 1; grid-row: 1 / span 3; margin: 0; text-align: end; }
}
"#;
