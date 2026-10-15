# Calendar

`loco_ui::calendar` · demo: `/calendar`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Calendar` | `ui.calendar(name: &str)` | stable |

## Props of `Calendar`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `today` | value | `date: &str` |  |  | The date to mark as today (`YYYY-MM-DD`), instead of the server's clock in UTC. |
| `min` | value | `date: &str` |  |  | The first day that can be picked (`YYYY-MM-DD`). |
| `max` | value | `date: &str` |  |  | The last day that can be picked (`YYYY-MM-DD`). |
| `disabled_dates` | value | `off: &'a dyn Fn(Date) -> bool` |  |  | Days for which `off` returns true cannot be picked. |
| `event` | item | `date: &str, text: &'a str` |  |  | A dot under the day `date` (`YYYY-MM-DD`), with `text` for screen readers and as the day's tooltip. |
| `sunday_first` | switch |  | `off` |  | Weeks start on Sunday, whatever the language's first day. |
| `radio` | switch |  | `off` |  | Each day is a radio button named after the calendar, to post inside a form, instead of a link. |
| `value` | value | `date: &str` |  | `value` | The picked day (`YYYY-MM-DD`) when it does not come from the query string. |
| `required` | switch |  | `off` | `required` | In radio mode, a day must be picked before the form submits. |
| `range` | value | `end: &'a str` |  |  | Pick a first and a last day: the first in the calendar's name, the last in `end`. |
| `weeks` | switch |  | `off` |  | A column of ISO 8601 week numbers before the days. |

## Keyboard of `Calendar`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves to the month links, then into the days. | no |
| `ArrowRight / ArrowDown` | Picks the next day when the calendar is a radio group. | no |
| `ArrowLeft / ArrowUp` | Picks the previous day when the calendar is a radio group. | no |
| `Enter` | Follows the focused day or month link. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-calendar` | `--lui-space-3`, `--lui-card`, `--lui-fg`, `--lui-line`, `--lui-radius-lg`, `--lui-shadow-xs` |
| `.lui-calendar-head` | `--lui-space-2`, `--lui-space-4` |
| `.lui-calendar-title` | `--lui-text-2` |
| `.lui-calendar-grid` | `--lui-text-2`, `--lui-space-2`, `--lui-text-1`, `--lui-muted` |
| `.lui-calendar-day` | `--lui-radius-sm`, `--lui-duration-fast`, `--lui-accent`, `--lui-on-accent` |
| `.lui-calendar-off` | `--lui-accent`, `--lui-on-accent`, `--lui-gray-9` |
| `.lui-calendar-outside` | `--lui-muted` |
| `.lui-calendar-today` | `--lui-accent`, `--lui-on-accent` |
| `.lui-calendar-picked` | `--lui-primary`, `--lui-on-primary` |
| `.lui-calendar-radio` | `--lui-primary`, `--lui-on-primary`, `--lui-ring` |
| `.lui-calendar-dots` | `--lui-primary` |
| `.lui-calendar-between` | `--lui-accent`, `--lui-on-accent` |
| `.lui-calendar-week` | `--lui-text-1`, `--lui-leading-1`, `--lui-muted` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<table>` | 1 | 1 | 1 |
| `aria-current="date"` | 1 | 1 | 1 |
| `role="radiogroup"` | 1 | 1 | 1 |
| `:has(:checked)` | 105 | 121 | 15.4 |

- Fallback: without :has() the picked radio's day is not filled in; it is still checked and posts
- Needs script: partly: changing month in place and arrow-key moves between days need script
- Responsive by: content
- Look after: shadcn Calendar

## Documentation

A month grid drawn on the server: previous and next month are links, each day is a link
(pick a day and the page comes back with it) or a radio button (the day is a form field,
for a date picker). Days before `.min()`, after `.max()` or ruled out by `.disabled_dates(..)`
cannot be picked; `.event(..)` puts a dot under a day. `.range(end)` picks two days, a
first and a last, into two parameters; `.weeks()` numbers the weeks (ISO 8601). Today is
`Ui::today`, which the app may set to the visitor's own day (`Ui::with_today`).

**Platform features:** a `<table>` of links or `<input type="radio">` inside `<label>`s,
`aria-current="date"` on today, `role="radiogroup"` when the days are radios, `:has(:checked)`
(Chrome 105, Firefox 121, Safari 15.4) to draw the picked radio's day. The month shown
travels in `?month.<name>=YYYY-MM`, the picked day in `?<name>=YYYY-MM-DD`.

**Accessibility:** a `<table>` with column headers (`abbr` holds the full weekday); each day
is a link or radio whose hidden text says the whole date, today is `aria-current="date"`,
the month title is `aria-live`; arrow keys are not wired (Tab moves through days). Checked
by axe-core in headless Firefox on every demo route, both capability variants, light and
dark (no serious or critical violation).

A range is links only: the first click sets the first day, the second the last, a third
starts again; the days between are shaded.

**What it does not do without script:** change month without a page load (the enhancement
script swaps the calendar in place: it is a swap root), or move between days with the arrow
keys (Tab goes through them in order). In radio mode a month link leaves the page, so a
form's other unsaved fields are lost; put the calendar in its own form or step.

**Fallback:** without `:has()` the picked radio's day is not filled in, but the radio is
still checked and posts.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/book", "month.due=2026-09", "");
let m = ui.calendar("due").today("2026-09-24").render().into_string();
assert!(m.contains("September 2026") && m.contains(r#"aria-current="date""#));
assert!(m.contains(r#"href="/book?month.due=2026-09&amp;due=2026-09-15""#));
// Radios for a form, a window of days, weekends and holidays off (the closure may
// capture), a deadline marked, Sunday first, week numbers.
let holidays = [loco_ui::calendar::Date::parse("2026-09-16").unwrap()];
let off = |d: loco_ui::calendar::Date| d.weekday() >= 5 || holidays.contains(&d);
let m = ui.calendar("due").radio().required()
    .min("2026-09-10").max("2026-10-31")
    .disabled_dates(&off)
    .event("2026-09-30", "Invoice due")
    .sunday_first()
    .weeks();
let m = m.render().into_string();
assert!(m.contains(r#"type="radio" name="due" value="2026-09-11""#));
assert!(m.contains(r#"value="2026-09-12" disabled"#) && m.contains("Invoice due"));
assert!(m.contains(r#"value="2026-09-16" disabled"#) && m.contains("Week 38"));
// The same in `lui!`:
let same = lui! { Calendar("due") radio required min="2026-09-10" max="2026-10-31"
    disabled_dates=(&off) event=("2026-09-30", "Invoice due") sunday_first weeks; };
assert_eq!(same.into_string(), m);
// A range: the first day is picked, so a later day's link sets the last one.
let ui = Ui::from_request("/stay", "from=2026-09-10", "");
let m = ui.calendar("from").range("to").render().into_string();
assert!(m.contains(r#"href="/stay?from=2026-09-10&amp;to=2026-09-14""#));
```
