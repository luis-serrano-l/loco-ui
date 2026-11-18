//! # Schedule page
//!
//! An agenda: the period shown with links to the one before, the one after and today, then
//! each day with its events in time order, and room beside it for a month calendar. After the
//! Origin UI agenda view of its event calendar, built from Radix Themes parts (Button, Card).
//!
//! **Platform features:** the period links are plain links, so each week or month is a page
//! with its own address; a day's date and an event's time are `<time>` when given for
//! machines; a container query (Chrome 105, Firefox 110, Safari 16) sets a day's name beside
//! its events from 30rem and the aside beside the agenda from 48rem.
//!
//! **Accessibility:** the title is the page's `<h1>` and each day an `<h2>`; today's day is
//! marked with `aria-current="date"`; the period links are a `<nav>` named by the title; an
//! event with a page is one link. Checked by axe-core with the demo routes.
//!
//! **What it does not do without script:** drag an event to another time, or show a day's
//! hours as a grid; it is a list.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.schedule_page("Schedule")
//!     .day("Monday 16 November").event("09:00", "Stand-up")
//!     .render().into_string();
//! assert!(m.contains(r#"<h2 class="lui-schedule-day-title">Monday 16 November</h2>"#));
//! // A week with its links, today marked, an event with a page and a tone:
//! let m = ui.schedule_page("Schedule").period("16 to 22 November 2026")
//!     .prev("/schedule?week=46").next("/schedule?week=48").today("/schedule")
//!     .day("Monday 16 November").datetime("2026-11-16").current()
//!     .event("09:00", "Stand-up").description("15 minutes, room 2")
//!     .event("14:00", "Release review").href("/events/7").tone(Tone::Warn)
//!     .day("Tuesday 17 November")
//!     .render().into_string();
//! assert!(m.contains(r#"<section class="lui-schedule-day" aria-current="date">"#));
//! assert!(m.contains(r#"<a class="lui-schedule-event" href="/events/7" data-tone="warn">"#));
//! assert!(m.contains(r#"<p class="lui-schedule-empty">Nothing planned</p>"#));
//! // The same in `lui!`:
//! let same = lui! { SchedulePage("Schedule") period="16 to 22 November 2026"
//!     prev="/schedule?week=46" next="/schedule?week=48" today="/schedule" {
//!     day "Monday 16 November" datetime="2026-11-16" current;
//!     event "09:00" "Stand-up" description="15 minutes, room 2";
//!     event "14:00" "Release review" href="/events/7" tone=(Tone::Warn);
//!     day "Tuesday 17 November";
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::icon::Icon;
use crate::props::{Key, Prop, PropKind};
use crate::variant::Tone;

/// An event: when, what, a line about it, its page and its colour.
#[derive(Clone, Debug)]
struct Event<'a> {
    time: &'a str,
    title: &'a str,
    description: Option<&'a str>,
    href: Option<&'a str>,
    tone: Option<Tone>,
}

/// A day: its name, its date for machines, whether it is today, and its events.
#[derive(Clone, Debug)]
struct Day<'a> {
    label: &'a str,
    datetime: Option<&'a str>,
    current: bool,
    events: Vec<Event<'a>>,
}

/// An agenda, made by [`Ui::schedule_page`].
///
/// **Setters.** Values and items: `.period(..)`, `.prev(..)`, `.next(..)`, `.today(..)`,
/// `.actions(..)`, `.day(..)`, `.datetime(..)`, `.event(..)`, `.description(..)`,
/// `.href(..)`, `.tone(..)`, `.aside(..)`; switches: `.current()`.
#[derive(Clone, Debug)]
pub struct SchedulePage<'a> {
    ui: &'a Ui,
    title: &'a str,
    period: Option<&'a str>,
    prev: Option<&'a str>,
    next: Option<&'a str>,
    today: Option<&'a str>,
    actions: Markup,
    days: Vec<Day<'a>>,
    aside: Markup,
}

impl SchedulePage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[Key::new(
        "Enter",
        "Follows a period link (previous, today, next) or an event's link.",
    )];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("period", PropKind::Value, "text: &'a str")
            .doc("The period shown, under the title (`\"16 to 22 November 2026\"`)."),
        Prop::new("prev", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("The page of the period before."),
        Prop::new("next", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("The page of the period after."),
        Prop::new("today", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("The page of the period that holds today."),
        Prop::new("actions", PropKind::Value, "actions: Markup")
            .doc("What sits beside the period links: a New event button."),
        Prop::new("day", PropKind::Item, "label: &'a str")
            .doc("A day, after the ones added before it. One with no event says so."),
        Prop::new("datetime", PropKind::Modifier, "date: &'a str")
            .applies_to(&["day"])
            .attr("datetime")
            .doc("The last day's date for machines (`2026-11-16`): its name becomes a `<time>`, and each of its events' times too."),
        Prop::new("current", PropKind::Modifier, "")
            .applies_to(&["day"])
            .doc("The last day is today."),
        Prop::new("event", PropKind::Item, "time: &'a str, title: &'a str")
            .doc("An event of the last day: when (`\"09:00\"`, `\"All day\"`) and what."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .applies_to(&["event"])
            .doc("A line under the last event's title: where, how long, who."),
        Prop::new("href", PropKind::Modifier, "href: &'a str")
            .applies_to(&["event"])
            .attr("href")
            .doc("The last event's page: the whole event becomes a link."),
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .applies_to(&["event"])
            .doc("The colour of the last event's bar."),
        Prop::new("aside", PropKind::Value, "aside: Markup")
            .doc("What sits beside the agenda from 48rem and above it when narrower: a month calendar."),
    ];
}

impl Ui {
    /// An agenda titled `title`.
    pub fn schedule_page<'a>(&'a self, title: &'a str) -> SchedulePage<'a> {
        SchedulePage {
            ui: self,
            title,
            period: None,
            prev: None,
            next: None,
            today: None,
            actions: Markup::default(),
            days: Vec::new(),
            aside: Markup::default(),
        }
    }
}

impl<'a> SchedulePage<'a> {
    /// The period shown, under the title (`"16 to 22 November 2026"`).
    pub fn period(mut self, text: &'a str) -> Self {
        self.period = Some(text);
        self
    }

    /// The page of the period before.
    pub fn prev(mut self, href: &'a str) -> Self {
        self.prev = Some(href);
        self
    }

    /// The page of the period after.
    pub fn next(mut self, href: &'a str) -> Self {
        self.next = Some(href);
        self
    }

    /// The page of the period that holds today.
    pub fn today(mut self, href: &'a str) -> Self {
        self.today = Some(href);
        self
    }

    /// What sits beside the period links: a New event button.
    pub fn actions(mut self, actions: Markup) -> Self {
        self.actions = actions;
        self
    }

    /// A day, after the ones added before it. One with no event says so.
    pub fn day(mut self, label: &'a str) -> Self {
        self.days.push(Day {
            label,
            datetime: None,
            current: false,
            events: Vec::new(),
        });
        self
    }

    /// What was added last: `"event"` when the last day has one, `"day"` when it has none.
    fn last(&self) -> Option<&'static str> {
        let day = self.days.last()?;
        Some(if day.events.is_empty() {
            "day"
        } else {
            "event"
        })
    }

    /// The day added last, when no event came after it.
    #[track_caller]
    fn last_day(&mut self, method: &str) -> Option<&mut Day<'a>> {
        crate::props::check_last::<Self>(SchedulePage::PROPS, method, self.last());
        self.days.last_mut().filter(|d| d.events.is_empty())
    }

    /// The event added last.
    #[track_caller]
    fn last_event(&mut self, method: &str) -> Option<&mut Event<'a>> {
        crate::props::check_last::<Self>(SchedulePage::PROPS, method, self.last());
        self.days.last_mut()?.events.last_mut()
    }

    /// The last day's date for machines (`2026-11-16`): its name becomes a `<time>`, and
    /// each of its events' times too.
    #[track_caller]
    pub fn datetime(mut self, date: &'a str) -> Self {
        if let Some(d) = self.last_day("datetime") {
            d.datetime = Some(date);
        }
        self
    }

    /// The last day is today.
    #[track_caller]
    pub fn current(mut self) -> Self {
        if let Some(d) = self.last_day("current") {
            d.current = true;
        }
        self
    }

    /// An event of the last day: when (`"09:00"`, `"All day"`) and what.
    #[track_caller]
    pub fn event(mut self, time: &'a str, title: &'a str) -> Self {
        debug_assert!(
            !self.days.is_empty(),
            "SchedulePage: .event() needs a .day() before it"
        );
        if let Some(d) = self.days.last_mut() {
            d.events.push(Event {
                time,
                title,
                description: None,
                href: None,
                tone: None,
            });
        }
        self
    }

    /// A line under the last event's title: where, how long, who.
    #[track_caller]
    pub fn description(mut self, text: &'a str) -> Self {
        if let Some(e) = self.last_event("description") {
            e.description = Some(text);
        }
        self
    }

    /// The last event's page: the whole event becomes a link.
    #[track_caller]
    pub fn href(mut self, href: &'a str) -> Self {
        if let Some(e) = self.last_event("href") {
            e.href = Some(href);
        }
        self
    }

    /// The colour of the last event's bar.
    #[track_caller]
    pub fn tone(mut self, tone: Tone) -> Self {
        if let Some(e) = self.last_event("tone") {
            e.tone = Some(tone);
        }
        self
    }

    /// What sits beside the agenda from 48rem and above it when narrower: a month calendar.
    pub fn aside(mut self, aside: Markup) -> Self {
        self.aside = aside;
        self
    }
}

/// `HH:MM` when `time` is a clock time, which a `<time datetime>` may carry after the date.
fn clock(time: &str) -> Option<&str> {
    let (h, m) = time.split_once(':')?;
    let digits = |s: &str| s.len() == 2 && s.bytes().all(|b| b.is_ascii_digit());
    (digits(h) && digits(m)).then_some(time)
}

/// An event's time, name and line: the inside of its link or its box.
fn event_parts(day: &Day<'_>, e: &Event<'_>) -> Markup {
    let machine = (day.datetime).and_then(|d| Some(format!("{d}T{}", clock(e.time)?)));
    html! {
        @if let Some(m) = machine { time class="lui-schedule-event-time" datetime=(m) { (e.time) } }
        @else { span class="lui-schedule-event-time" { (e.time) } }
        span class="lui-schedule-event-text" {
            span class="lui-schedule-event-title" { (e.title) }
            @if let Some(d) = e.description { span class="lui-schedule-event-description" { (d) } }
        }
    }
}

impl Render for SchedulePage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let has_nav = self.prev.is_some() || self.next.is_some() || self.today.is_some();
        html! {
            div class="lui-schedule" {
                div class="lui-schedule-head" {
                    div class="lui-schedule-title" {
                        h1 { (self.title) }
                        @if let Some(p) = self.period { p class="lui-schedule-period" { (p) } }
                    }
                    @if has_nav || !self.actions.0.is_empty() {
                        div class="lui-schedule-tools" {
                            @if has_nav {
                                nav class="lui-schedule-nav" aria-label=(self.title) {
                                    @if let Some(h) = self.prev {
                                        (ui.link_button("", h).icon_only().aria_label(ui.text(Text::Previous)).rel("prev").body(html! { (Icon::ChevronLeft) }))
                                    }
                                    @if let Some(h) = self.today { (ui.link_button(ui.text(Text::Today), h)) }
                                    @if let Some(h) = self.next {
                                        (ui.link_button("", h).icon_only().aria_label(ui.text(Text::Next)).rel("next").body(html! { (Icon::ChevronRight) }))
                                    }
                                }
                            }
                            (self.actions)
                        }
                    }
                }
                div class="lui-schedule-panes" {
                    @if !self.aside.0.is_empty() { div class="lui-schedule-aside" { (self.aside) } }
                    div class="lui-schedule-days" {
                        @for day in &self.days {
                            section class="lui-schedule-day" aria-current=[day.current.then_some("date")] {
                                h2 class="lui-schedule-day-title" {
                                    @if let Some(d) = day.datetime { time datetime=(d) { (day.label) } } @else { (day.label) }
                                }
                                @if day.events.is_empty() {
                                    p class="lui-schedule-empty" { (ui.text(Text::NoEvents)) }
                                } @else {
                                    ul class="lui-schedule-events" {
                                        @for e in &day.events {
                                            li {
                                                @let tone = e.tone.map(Tone::as_str);
                                                @if let Some(h) = e.href {
                                                    a class="lui-schedule-event" href=(h) data-tone=[tone] { (event_parts(day, e)) }
                                                } @else {
                                                    div class="lui-schedule-event" data-tone=[tone] { (event_parts(day, e)) }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. Narrow: the aside, then each
/// day's name above its events. From 30rem a day's name sits beside its events; from 48rem
/// the aside sits beside the agenda.
pub const CSS: &str = r#"
.lui-schedule { container: lui-schedule / inline-size; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-6); }
.lui-schedule-head { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--lui-space-3); }
.lui-schedule-title { min-width: 0; }
.lui-schedule-title h1 { margin: 0; overflow-wrap: anywhere; }
.lui-schedule-period { margin: var(--lui-space-1) 0 0; color: var(--lui-muted); }
.lui-schedule-tools, .lui-schedule-nav { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-2); }
.lui-schedule-nav svg:dir(rtl) { scale: -1 1; }
.lui-schedule-panes { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-6); align-items: start; }
.lui-schedule-aside { min-width: 0; }
.lui-schedule-days { display: grid; grid-template-columns: minmax(0, 1fr); min-width: 0; }
.lui-schedule-day { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-2); padding-block: var(--lui-space-4); border-block-start: 1px solid var(--lui-line); }
.lui-schedule-day:first-child { border-block-start: 0; padding-block-start: 0; }
.lui-schedule-day-title { margin: 0; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); letter-spacing: 0; font-weight: 500; color: var(--lui-muted); overflow-wrap: anywhere; }
.lui-schedule-day[aria-current="date"] .lui-schedule-day-title { color: var(--lui-primary); font-weight: 600; }
.lui-schedule-empty { margin: 0; color: var(--lui-muted); font-size: var(--lui-text-2); }
.lui-schedule-events > li { max-width: none; }
.lui-schedule-events { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-2); }
.lui-schedule-event { --lui-schedule-tone: var(--lui-primary); display: grid; grid-template-columns: 4.5rem minmax(0, 1fr); gap: var(--lui-space-3); align-items: baseline; padding: var(--lui-space-2) var(--lui-space-3); border-inline-start: 3px solid var(--lui-schedule-tone); border-radius: var(--lui-radius-sm, var(--lui-radius)); background: var(--lui-gray-2); color: inherit; text-decoration: none; font-size: var(--lui-text-2); }
a.lui-schedule-event:hover { background: var(--lui-gray-3); }
.lui-schedule-event[data-tone=ok] { --lui-schedule-tone: var(--lui-ok); }
.lui-schedule-event[data-tone=warn] { --lui-schedule-tone: var(--lui-warn); }
.lui-schedule-event[data-tone=danger] { --lui-schedule-tone: var(--lui-danger); }
.lui-schedule-event[data-tone=info] { --lui-schedule-tone: var(--lui-info); }
.lui-schedule-event[data-tone=neutral] { --lui-schedule-tone: var(--lui-gray-9); }
.lui-schedule-event-time { color: var(--lui-muted); font-variant-numeric: tabular-nums; }
.lui-schedule-event-text { display: grid; grid-template-columns: minmax(0, 1fr); min-width: 0; }
.lui-schedule-event-title { font-weight: 500; overflow-wrap: anywhere; }
.lui-schedule-event-description { color: var(--lui-muted); font-size: var(--lui-text-1); overflow-wrap: anywhere; }
@media (pointer: coarse) { a.lui-schedule-event { min-height: var(--lui-hit); } }
@container lui-schedule (width >= 30rem) {
  .lui-schedule-day { grid-template-columns: 11rem minmax(0, 1fr); gap: var(--lui-space-4); }
}
@container lui-schedule (width >= 48rem) {
  .lui-schedule-panes:has(.lui-schedule-aside) { grid-template-columns: minmax(0, 1fr) 18rem; }
  .lui-schedule-aside { grid-column: 2; grid-row: 1; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_page_is_a_title_and_no_links() {
        let ui = Ui::default();
        let m = ui.schedule_page("Schedule").render().into_string();
        assert_eq!(
            m,
            r#"<div class="lui-schedule"><div class="lui-schedule-head"><div class="lui-schedule-title"><h1>Schedule</h1></div></div><div class="lui-schedule-panes"><div class="lui-schedule-days"></div></div></div>"#
        );
    }

    #[test]
    fn the_period_links_are_a_named_nav_with_rel() {
        let ui = Ui::default();
        let m = ui
            .schedule_page("Schedule")
            .period("This week")
            .prev("/s?w=1")
            .next("/s?w=3")
            .today("/s")
            .actions(html! { b { "new" } })
            .render()
            .into_string();
        assert!(
            m.contains(r#"<nav class="lui-schedule-nav" aria-label="Schedule">"#),
            "{m}"
        );
        assert!(
            m.contains(r#"rel="prev""#) && m.contains(r#"rel="next""#),
            "{m}"
        );
        assert!(
            m.contains(r#"aria-label="Previous""#) && m.contains(">Today<"),
            "{m}"
        );
        assert!(
            m.contains(r#"<p class="lui-schedule-period">This week</p>"#),
            "{m}"
        );
        assert!(m.contains("</nav><b>new</b></div>"), "{m}");
    }

    #[test]
    fn days_hold_their_events_and_an_empty_day_says_so() {
        let ui = Ui::default();
        let m = ui
            .schedule_page("S")
            .day("Mon")
            .datetime("2026-11-16")
            .current()
            .event("09:00", "Stand-up")
            .description("Room 2")
            .event("All day", "Offsite")
            .href("/e/2")
            .tone(Tone::Ok)
            .day("Tue")
            .aside(html! { i { "month" } })
            .render()
            .into_string();
        assert!(
            m.contains(r#"<time datetime="2026-11-16">Mon</time>"#),
            "{m}"
        );
        assert!(
            m.contains(
                r#"<time class="lui-schedule-event-time" datetime="2026-11-16T09:00">09:00</time>"#
            ),
            "{m}"
        );
        assert!(
            m.contains(r#"<span class="lui-schedule-event-time">All day</span>"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<a class="lui-schedule-event" href="/e/2" data-tone="ok">"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<span class="lui-schedule-event-description">Room 2</span>"#),
            "{m}"
        );
        assert_eq!(m.matches("lui-schedule-empty").count(), 1, "{m}");
        assert_eq!(m.matches(r#"aria-current="date""#).count(), 1, "{m}");
        assert!(
            m.contains(r#"<div class="lui-schedule-aside"><i>month</i></div>"#),
            "{m}"
        );
    }

    #[test]
    #[should_panic(expected = "datetime")]
    fn a_day_modifier_after_an_event_is_refused() {
        let ui = Ui::default();
        let _ = ui
            .schedule_page("S")
            .day("Mon")
            .event("09:00", "Stand-up")
            .datetime("2026-11-16");
    }
}
