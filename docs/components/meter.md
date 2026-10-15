# Meter

`loco_ui::meter` · demo: `/feedback`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Meter` | `ui.meter(value: impl Into<f64>, min: impl Into<f64>, max: impl Into<f64>)` | stable |

## Props of `Meter`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `label` | value | `text: &'a str` |  |  | A label above the meter, with the value beside it. |
| `low` | number | `low: impl Into<f64>` |  | `low` | Below this is "low". |
| `high` | number | `high: impl Into<f64>` |  | `high` | Above this is "high". |
| `optimum` | number | `optimum: impl Into<f64>` |  | `optimum` | The best value. |
| `id` | value | `id: &'a str` |  | `id` | The meter's id instead of `lui-meter-<label>`. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-meter` | `--lui-gray-4`, `--lui-ok`, `--lui-warn`, `--lui-danger` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<meter>` | 6 | 16 | 6 |

- Fallback: without the pseudo-elements a browser draws its own meter
- Needs script: no
- Responsive by: content
- Look after: Tremor CategoryBar colours

## Documentation

A measurement within a known range, like disk space or a password's strength: unlike a
progress bar it is not a task moving forward, and it turns warn or danger past thresholds.

**Platform features:** `<meter>` (Chrome 6, Firefox 16, Safari 6) with `low`, `high` and
`optimum`, so the browser picks the good, average or bad colour itself; the
`::-webkit-meter-*` and `::-moz-meter-bar` pseudo-elements map those to `--lui-ok`,
`--lui-warn` and `--lui-danger`.

**Accessibility:** a native `<meter>` with its label. Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** update live.

**Fallback:** without the pseudo-elements a browser draws its own meter, in its own colours.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.meter(82, 0, 100).label("Disk").low(60).high(80).optimum(0).render().into_string();
assert!(m.contains(r#"value="82" min="0" max="100" low="60" high="80" optimum="0""#));
// The same in `lui!`:
let same = lui! { Meter(82, 0, 100) label="Disk" low=60 high=80 optimum=0; };
assert_eq!(same.into_string(), m);
```
