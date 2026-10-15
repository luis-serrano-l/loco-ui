# Stat

`loco_ui::stat` · demo: `/dashboard`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Stat` | `ui.stat(label: &str, value: &str)` | stable |

## Props of `Stat`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `delta` | value | `text: &'a str` |  |  | The change. |
| `trend` | value | `trend: Trend` |  |  | Override the direction read from the delta's sign. |
| `down_is_good` | switch |  | `off` |  | A fall is good news (error rates, latency). |
| `description` | value | `note: &'a str` |  |  | Small print under the value. |
| `href` | value | `href: &'a str` |  | `href` | Make the whole card a link to the details. |
| `reveal` | switch |  | `off` |  | Fades and rises as it scrolls into view. |
| `sample` | item | `value: f64` |  |  | A recent value for the sparkline along the card's bottom edge (oldest first). |
| `progress` | value | `value: u64, max: u64` |  |  | A bar under the value: how far towards a target. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-stat-grid` | `--lui-space-4` |
| `.lui-stat` | `--lui-space-1`, `--lui-space-4`, `--lui-fg`, `--lui-card`, `--lui-line`, `--lui-radius-lg`, `--lui-shadow-sm`, `--lui-highlight`, `--lui-duration-fast` |
| `.lui-stat-link` | `--lui-accent`, `--lui-card` |
| `.lui-stat-label` | `--lui-muted`, `--lui-text-2` |
| `.lui-stat-value` | `--lui-text-7` |
| `.lui-stat-delta` | `--lui-muted`, `--lui-radius-sm`, `--lui-text-1`, `--lui-stat-tone`, `--lui-fg`, `--lui-bg` |
| `.lui-stat-good` | `--lui-ok` |
| `.lui-stat-bad` | `--lui-danger` |
| `.lui-stat-flat` | `--lui-muted` |
| `.lui-stat-note` | `--lui-muted`, `--lui-text-1` |
| `.lui-stat-progress` | `--lui-space-2`, `--lui-gray-4`, `--lui-primary` |
| `.lui-stat-spark` | `--lui-space-2`, `--lui-space-4` |
| `.lui-stat-spark-line` | `--lui-primary` |
| `.lui-stat-spark-area` | `--lui-primary` |
| `.lui-stat-reveal` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `repeat(auto-fit` | 57 | 52 | 10.1 |
| `animation-timeline: view()` | 115 | no | 26 |

- Fallback: none needed; a reveal tile is shown in place without animation-timeline
- Needs script: no
- Responsive by: content
- Look after: Tremor KPI cards

## Documentation

One number that matters, with its label and how it moved: the tiles at the top of a
dashboard. Several sit in a `div.lui-stat-grid`, which wraps them to the width available.

**Platform features:** a card of `<p>` elements that reads in order; the change
carries its direction in words for screen readers (`<span class="lui-sr">`) as well as an
arrow and a colour; the grid is `repeat(auto-fit, minmax(12rem, 1fr))`, so no media query.
`.reveal()` fades and rises the tile as it scrolls into view, as a card's does: a
scroll-driven animation with `animation-timeline: view()` (Chrome 115, Firefox no,
Safari 26), off under `prefers-reduced-motion: reduce`.

**Accessibility:** a label and a value in text; trend arrows are `aria-hidden` and the
change is in words. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** update live; the number is as fresh as the page.

**Fallback:** none needed. A `.reveal()` tile where `animation-timeline: view()` is missing,
or under `prefers-reduced-motion: reduce`, is shown in place from the start.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
assert!(ui.stat("Visitors", "12,480").render().into_string().contains("12,480"));
// The sign of the delta says the direction: `+` up, `-` down, zero flat.
let m = ui.stat("Error rate", "0.4%")
    .delta("-0.2 pt")
    .down_is_good()
    .description("last 7 days")
    .href("/errors");
let m = m.render().into_string();
assert!(m.contains("lui-stat-good") && m.contains("down") && m.contains(r#"href="/errors""#));
// The same in `lui!`:
let same = lui! {
    Stat("Error rate", "0.4%") delta="-0.2 pt" down_is_good description="last 7 days" href="/errors";
};
assert_eq!(same.into_string(), m);
// Opt-in motion: it fades in as it scrolls into view.
let up = ui.stat("Visitors", "12,480").reveal().render().into_string();
assert!(up.starts_with(r#"<div class="lui-stat lui-stat-reveal">"#));
assert_eq!(lui! { Stat("Visitors", "12,480") reveal; }.into_string(), up);
// A sparkline along the bottom and a progress bar towards a target.
let kpi = ui.stat("Revenue", "$48,210").sample(30.0).sample(34.0).sample(31.0).sample(38.0).progress(62, 100).render().into_string();
assert!(kpi.contains(r#"<progress class="lui-stat-progress" value="62" max="100""#) && kpi.contains("lui-stat-spark"));
assert_eq!(lui! { Stat("Revenue", "$48,210") sample=30.0 sample=34.0 sample=31.0 sample=38.0 progress=(62, 100); }.into_string(), kpi);
```
