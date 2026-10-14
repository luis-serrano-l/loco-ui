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
| `disabled_dates` | value | `off: fn(Date) -> bool` |  |  | Days for which `off` returns true cannot be picked. |
| `event` | item | `date: &str, text: &'a str` |  |  | A dot under the day `date` (`YYYY-MM-DD`), with `text` for screen readers and as the day's tooltip. |
| `sunday_first` | switch |  | `off` |  | Weeks start on Sunday, whatever the language's first day. |
| `radio` | switch |  | `off` |  | Each day is a radio button named after the calendar, to post inside a form, instead of a link. |
| `value` | value | `date: &str` |  | `value` | The picked day (`YYYY-MM-DD`) when it does not come from the query string. |
| `required` | switch |  | `off` | `required` | In radio mode, a day must be picked before the form submits. |

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
cannot be picked; `.event(..)` puts a dot under a day.

**Platform features:** a `<table>` of links or `<input type="radio">` inside `<label>`s,
`aria-current="date"` on today, `role="radiogroup"` when the days are radios, `:has(:checked)`
(Chrome 105, Firefox 121, Safari 15.4) to draw the picked radio's day. The month shown
travels in `?month.<name>=YYYY-MM`, the picked day in `?<name>=YYYY-MM-DD`.

**Accessibility:** a `<table>` with column headers (`abbr` holds the full weekday); each day
is a link or radio whose hidden text says the whole date, today is `aria-current="date"`,
the month title is `aria-live`; arrow keys are not wired (Tab moves through days). Checked
by axe-core in headless Firefox on every demo route, both capability variants, light and
dark (no serious or critical violation).

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
// Radios for a form, a window of days, weekends off, a deadline marked, Sunday first.
let m = ui.calendar("due").radio().required()
    .min("2026-09-10").max("2026-10-31")
    .disabled_dates(|d| d.weekday() >= 5)
    .event("2026-09-30", "Invoice due")
    .sunday_first();
let m = m.render().into_string();
assert!(m.contains(r#"type="radio" name="due" value="2026-09-11""#));
assert!(m.contains(r#"value="2026-09-12" disabled"#) && m.contains("Invoice due"));
// The same in `lui!`:
let same = lui! { Calendar("due") radio required min="2026-09-10" max="2026-10-31"
    disabled_dates=(|d| d.weekday() >= 5) event=("2026-09-30", "Invoice due") sunday_first; };
assert_eq!(same.into_string(), m);
```
