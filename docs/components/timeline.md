# Timeline

`loco_ui::timeline` · demo: `/timeline`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Timeline` | `ui.timeline()` | beta |

## Props of `Timeline`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `event` | item | `title: &'a str` |  |  | An event titled `title`. |
| `when` | modifier | `text: &'a str` |  |  | When the event added last happened, as the reader should see it. |
| `datetime` | modifier | `iso: &'a str` |  | `datetime` | The same date for machines (`2026-10-12`): the date becomes a `<time datetime>`. |
| `description` | modifier | `text: &'a str` |  |  | A muted line under the title of the event added last. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon in a ring on the line instead of the dot, for the event added last. |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` |  |  | The colour of the event added last's dot or icon (neutral by default). |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-timeline` | `--lui-primary`, `--lui-ok`, `--lui-warn`, `--lui-danger`, `--lui-gray-9` |
| `.lui-timeline-icon` | `--lui-timeline-tone`, `--lui-bg` |
| `.lui-timeline-dot` | `--lui-timeline-tone` |
| `.lui-timeline-when` | `--lui-space-1`, `--lui-text-1`, `--lui-muted` |
| `.lui-timeline-title` | `--lui-text-2`, `--lui-fg` |
| `.lui-timeline-description` | `--lui-text-2`, `--lui-muted` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<ol>` | 1 | 1 | 1 |
| `<time datetime>` | 62 | 22 | 7 |
| `:has()` | 105 | 121 | 15.4 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: Origin UI Timeline

## Documentation

What happened, in order: a vertical line with a dot (or an icon) per event, each with
when it happened, a title and a line of description. After the Origin UI Timeline.

**Platform features:** an `<ol>` (baseline since HTML 2); the line is each item's
`::before`; a date given with `.datetime(..)` is a `<time datetime>` (Chrome 62, Firefox
22, Safari 7); `:has()` (Chrome 105, Firefox 121, Safari 15.4) starts the line below an
icon.

**Accessibility:** an ordered list read in order; each event's dot or icon is
`aria-hidden`, so the date, title and description are what is read. Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.timeline().event("Founded").when("2019").event("First release").when("2021").render().into_string();
assert!(m.contains(r#"<span class="lui-timeline-when">2019</span>"#));
// An icon and a tone on an event, a description and a machine-readable date.
let m = ui.timeline()
    .event("Deployed").when("12 October").datetime("2026-10-12").icon(Icon::Check).tone(Tone::Ok)
    .description("Version 0.2 is live.")
    .event("Rolled back").when("13 October").tone(Tone::Danger);
let html = m.render().into_string();
assert!(html.contains(r#"<li data-tone="ok">"#) && html.contains(r#"<time class="lui-timeline-when" datetime="2026-10-12">"#));
// The same in `lui!`:
let same = lui! { Timeline {
    event "Deployed" when="12 October" datetime="2026-10-12" icon=(Icon::Check) tone="ok" description="Version 0.2 is live.";
    event "Rolled back" when="13 October" tone="danger";
} };
assert_eq!(same.into_string(), html);
```
