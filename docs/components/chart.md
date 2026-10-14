# Chart

`loco_ui::chart` · demo: `/chart`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Chart` | `ui.chart(title: &str)` | beta |

## Props of `Chart`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `point` | item | `label: &'a str, value: f64` |  |  | One value, labelled on the x axis. |
| `points` | value | `points: impl IntoIterator<Item = (&'a str, f64)>` |  |  | Many values at once, from a slice or a query: `(label, value)` pairs. |
| `description` | value | `text: &'a str` |  |  | What the chart shows, in a sentence: its `<desc>`. |
| `unit` | value | `unit: &'a str` |  |  | Written after every value, as `" ms"` or `"%"`. |
| `id` | value | `id: &'a str` |  | `id` | The id its parts are named from; `chart-<title>` by default. |
| `bar` | switch |  | `off` |  | Bars, one per point (the default). |
| `line` | switch |  | `off` |  | A line through the points. |
| `sparkline` | switch |  | `off` |  | A small line with no axes, to sit beside a number. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<svg>` | 7 | 4 | 5.1 |
| `role="img"` | 1 | 1 | 1 |
| `<title>` | 1 | 1 | 1 |
| `CSS custom properties in SVG` | 49 | 31 | 9.1 |

- Fallback: none needed
- Needs script: partly: zoom, pan and a crosshair that follows the pointer need script
- Responsive by: container
- Look after: Tremor AreaChart, BarChart, LineChart

## Documentation

A bar chart, a line chart or a sparkline drawn on the server as inline SVG: one series of
labelled values, a y axis rounded to readable steps, and the numbers themselves in a
visually hidden table for anyone who cannot see the picture. No chart library and no
script, where most kits need both.

**Platform features:**
- Inline `<svg>` in HTML (Chrome 7, Firefox 4, Safari 5.1), a `role="group"` named by its
  `<title>` and `<desc>` through `aria-labelledby` (a sparkline is one `role="img"`).
- Each bar and point is a focusable `<g tabindex="0" role="img">` named by its label and
  value, whose value chip CSS shows on `:hover` and `:focus-visible`.
- A container query (Chrome 105, Firefox 110, Safari 16) drops every other x label and
  draws the text larger in a chart under 30rem wide.
- CSS custom properties in SVG `fill` and `stroke` (Chrome 49, Firefox 31, Safari 9.1): the
  colours are `--lui-*` tokens, so a chart follows the theme and dark mode.

The look follows Tremor's charts: dashed gray gridlines, a legend swatch over the chart,
the series in the primary colour, a dark value chip on each mark.

**Accessibility:** the SVG is a group named by the chart's title and description, each
mark an image named "label: value" that takes the focus; the data is also a table (row headers are the labels) hidden visually but read by screen
readers; a sparkline, which sits in a sentence, lists its values as hidden text instead. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** zoom, pan, or a crosshair that follows the pointer;
the value shows on the mark under the pointer or the focus.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.chart("Signups").point("Mon", 12.0).point("Tue", 18.0).point("Wed", 9.0);
let html = m.render().into_string();
assert!(html.contains(r#"role="group""#) && html.contains(r#"aria-label="Tue: 18""#) && html.contains("<title id=\"chart-signups-title\">Signups</title>"));
assert!(html.contains(r#"<th scope="row">Tue</th><td>18</td>"#));
// Values from data in one call:
const SIGNUPS: [(&str, f64); 3] = [("Mon", 12.0), ("Tue", 18.0), ("Wed", 9.0)];
assert_eq!(ui.chart("Signups").points(SIGNUPS).render().into_string(), html);

let m = ui.chart("Latency").line().unit(" ms").description("p50 per day, last week")
    .point("Mon", 41.5).point("Tue", 38.0);
let html = m.render().into_string();
assert!(html.contains("lui-chart-line") && html.contains("<td>41.5 ms</td>"));
// The same in `lui!`:
let same = lui! { Chart("Latency") line unit=" ms" description="p50 per day, last week" {
    point "Mon" 41.5; point "Tue" 38.0;
} };
assert_eq!(same.into_string(), html);
```
