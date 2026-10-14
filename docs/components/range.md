# Range

`loco_ui::range` · demo: `/inputs`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Range` | `ui.range(name: &str, label: &str)` | stable |
| `Range` | `ui.range_pair(name: &str, label: &str)` | stable |

## Props of `Range`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `value` | number | `value: impl Into<f64>` |  | `value` | The value (a pair's low one), instead of the query's. |
| `values` | value | `low: impl Into<f64>, high: impl Into<f64>` |  |  | A pair's two values, put in order, instead of the query's. |
| `min` | number | `min: impl Into<f64>` | `0` | `min` | Lowest value. |
| `max` | number | `max: impl Into<f64>` | `100` | `max` | Highest value. |
| `step` | number | `step: impl Into<f64>` | `1` | `step` | Distance between allowed values (above 0). |
| `disabled` | switch |  | `off` | `disabled` | `disabled` and `aria-disabled`: shown greyed and not posted. |
| `readonly` | switch |  | `off` |  | Shown and posted, not movable: a disabled slider and a hidden input with its value. |
| `ticks` | switch |  | `off` |  | The lowest, middle and highest values marked under the track. |
| `id` | value | `id: &'a str` |  | `id` | The slider's id, `f-<name>` by default. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<input type="range">` | 4 | 23 | 3.1 |
| `<datalist>` | 20 | 110 | 12.1 |
| `pointer-events` | 1 | 1 | 1 |

- Fallback: ticks not drawn
- Needs script: partly: value shown after submit; live mirroring needs script
- Responsive by: content
- Look after: Radix Themes Slider

## Documentation

A slider whose value the server remembers, no script.

**Platform features:** `<input type="range">` (Chrome 4, Firefox 23, Safari 3.1) with
`min`, `max`, `step`, and an `<output>` that shows the value the server last saw. A
`list` of `<datalist>` ticks (Chrome 20, Firefox 110, Safari 12.1) labels the stops.

`Ui::range_pair` is a min/max pair: two range inputs (`<name>_min`, `<name>_max`) laid over
one track in a CSS grid cell. The track ignores the pointer (`pointer-events: none`) and
only the thumbs take it (`::-webkit-slider-thumb`, `::-moz-range-thumb`), so either thumb
can be dragged. Each has its own `<output>`.

**Accessibility:** native range inputs, each named (the pair's thumbs as "Minimum" and
"Maximum"); the value is shown in an `<output>`. Checked by axe-core in headless Firefox on
every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** show the value while dragging (the `<output>` holds
the value the server last saw), or stop the two thumbs of a pair crossing; the server
reorders them with `order`.

**Fallback:** none needed. Without `list` support the ticks are simply not drawn. A pair
whose thumbs cross posts a low above the high; `order` swaps them back on the server.

The look follows Radix Themes Slider: a thin gray track filled in the primary colour up to
the value (between the thumbs for a pair; `--lui-range-fill`, or `--lui-range-lo` and
`--lui-range-hi`, set by the server), a round white thumb, and `.ticks()` for the lowest,
middle and highest values under the track. The value stays beside the slider: CSS cannot
place a label over a thumb that moves without script.

**Enhanced:** the enhancement script mirrors each slider into its `<output for>` and its
fill while it moves. Without it the `<output>` shows the value the server last saw.

```rust
use loco_ui::prelude::*;
// The values are the query's (`volume`; `price_min` and `price_max` for a pair), or
// `.value(..)` and `.values(..)` (saved ones).
let ui = Ui::from_request("/", "volume=40&price_min=80&price_max=20", "");
let m = ui.range("volume", "Volume").step(5).render().into_string();
assert!(m.contains(r#"<label for="f-volume">Volume</label>"#) && m.contains("<output for=\"f-volume\">40</output>"));
assert!(ui.range("volume", "Volume").value(70).render().into_string().contains("<output for=\"f-volume\">70</output>"));
assert!(m.contains(r#"style="--lui-range-fill: 40%""#));
let ticked = ui.range("volume", "Volume").ticks().render().into_string();
assert!(ticked.contains(r#"<div class="lui-range-ticks" aria-hidden="true"><span>0</span>"#));
assert_eq!(lui! { Range("volume", "Volume") ticks; }.into_string(), ticked);
// A pair posted the wrong way round is put back in order.
let m = ui.range_pair("price", "Price").step(10).render().into_string();
assert!(m.contains("name=\"price_min\"") && m.contains("name=\"price_max\""));
assert!(m.contains("<output for=\"f-price_min\">20</output>"));

// The same in `lui!`:
let same = lui! { RangePair("price", "Price") step=10; };
assert_eq!(same.into_string(), m);
```
