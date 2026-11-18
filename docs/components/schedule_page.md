# SchedulePage

`loco_ui::blocks::schedule_page` · demo: `/blocks/schedule`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `SchedulePage` | `ui.schedule_page(title: &str)` | beta |

## Props of `SchedulePage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `period` | value | `text: &'a str` |  |  | The period shown, under the title (`"16 to 22 November 2026"`). |
| `prev` | value | `href: &'a str` |  | `href` | The page of the period before. |
| `next` | value | `href: &'a str` |  | `href` | The page of the period after. |
| `today` | value | `href: &'a str` |  | `href` | The page of the period that holds today. |
| `actions` | value | `actions: Markup` |  |  | What sits beside the period links: a New event button. |
| `day` | item | `label: &'a str` |  |  | A day, after the ones added before it. One with no event says so. |
| `datetime` | modifier | `date: &'a str` |  | `datetime` | The last day's date for machines (`2026-11-16`): its name becomes a `<time>`, and each of its events' times too. |
| `current` | modifier |  |  |  | The last day is today. |
| `event` | item | `time: &'a str, title: &'a str` |  |  | An event of the last day: when (`"09:00"`, `"All day"`) and what. |
| `description` | modifier | `text: &'a str` |  |  | A line under the last event's title: where, how long, who. |
| `href` | modifier | `href: &'a str` |  | `href` | The last event's page: the whole event becomes a link. |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` |  |  | The colour of the last event's bar. |
| `aside` | value | `aside: Markup` |  |  | What sits beside the agenda from 48rem and above it when narrower: a month calendar. |

## Keyboard of `SchedulePage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | Follows a period link (previous, today, next) or an event's link. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-schedule` | `--lui-space-6` |
| `.lui-schedule-head` | `--lui-space-3` |
| `.lui-schedule-title` |  |
| `.lui-schedule-period` | `--lui-space-1`, `--lui-muted` |
| `.lui-schedule-tools` | `--lui-space-2` |
| `.lui-schedule-nav` | `--lui-space-2` |
| `.lui-schedule-panes` | `--lui-space-6` |
| `.lui-schedule-aside` |  |
| `.lui-schedule-days` |  |
| `.lui-schedule-day` | `--lui-space-2`, `--lui-space-4`, `--lui-line` |
| `.lui-schedule-day-title` | `--lui-text-2`, `--lui-leading-2`, `--lui-muted`, `--lui-primary` |
| `.lui-schedule-empty` | `--lui-muted`, `--lui-text-2` |
| `.lui-schedule-events` | `--lui-space-2` |
| `.lui-schedule-event` | `--lui-primary`, `--lui-space-3`, `--lui-space-2`, `--lui-schedule-tone`, `--lui-radius-sm`, `--lui-radius`, `--lui-gray-2`, `--lui-text-2`, `--lui-gray-3`, `--lui-ok`, `--lui-warn`, `--lui-danger`, `--lui-info`, `--lui-gray-9`, `--lui-hit` |
| `.lui-schedule-event-time` | `--lui-muted` |
| `.lui-schedule-event-text` |  |
| `.lui-schedule-event-title` |  |
| `.lui-schedule-event-description` | `--lui-muted`, `--lui-text-1` |

## Documentation

An agenda: the period shown with links to the one before, the one after and today, then
each day with its events in time order, and room beside it for a month calendar. After the
Origin UI agenda view of its event calendar, built from Radix Themes parts (Button, Card).

**Platform features:** the period links are plain links, so each week or month is a page
with its own address; a day's date and an event's time are `<time>` when given for
machines; a container query (Chrome 105, Firefox 110, Safari 16) sets a day's name beside
its events from 30rem and the aside beside the agenda from 48rem.

**Accessibility:** the title is the page's `<h1>` and each day an `<h2>`; today's day is
marked with `aria-current="date"`; the period links are a `<nav>` named by the title; an
event with a page is one link. Checked by axe-core with the demo routes.

**What it does not do without script:** drag an event to another time, or show a day's
hours as a grid; it is a list.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.schedule_page("Schedule")
    .day("Monday 16 November").event("09:00", "Stand-up")
    .render().into_string();
assert!(m.contains(r#"<h2 class="lui-schedule-day-title">Monday 16 November</h2>"#));
// A week with its links, today marked, an event with a page and a tone:
let m = ui.schedule_page("Schedule").period("16 to 22 November 2026")
    .prev("/schedule?week=46").next("/schedule?week=48").today("/schedule")
    .day("Monday 16 November").datetime("2026-11-16").current()
    .event("09:00", "Stand-up").description("15 minutes, room 2")
    .event("14:00", "Release review").href("/events/7").tone(Tone::Warn)
    .day("Tuesday 17 November")
    .render().into_string();
assert!(m.contains(r#"<section class="lui-schedule-day" aria-current="date">"#));
assert!(m.contains(r#"<a class="lui-schedule-event" href="/events/7" data-tone="warn">"#));
assert!(m.contains(r#"<p class="lui-schedule-empty">Nothing planned</p>"#));
// The same in `lui!`:
let same = lui! { SchedulePage("Schedule") period="16 to 22 November 2026"
    prev="/schedule?week=46" next="/schedule?week=48" today="/schedule" {
    day "Monday 16 November" datetime="2026-11-16" current;
    event "09:00" "Stand-up" description="15 minutes, room 2";
    event "14:00" "Release review" href="/events/7" tone=(Tone::Warn);
    day "Tuesday 17 November";
} };
assert_eq!(same.into_string(), m);
```
