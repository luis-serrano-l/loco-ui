//! # Range
//!
//! A slider whose value the server remembers, no script.
//!
//! **Platform features:** `<input type="range">` (Chrome 4, Firefox 23, Safari 3.1) with
//! `min`, `max`, `step`, and an `<output>` that shows the value the server last saw. A
//! `list` of `<datalist>` ticks (Chrome 20, Firefox 110, Safari 12.1) labels the stops.
//!
//! [`Ui::range_pair`] is a min/max pair: two range inputs (`<name>_min`, `<name>_max`) laid over
//! one track in a CSS grid cell. The track ignores the pointer (`pointer-events: none`) and
//! only the thumbs take it (`::-webkit-slider-thumb`, `::-moz-range-thumb`), so either thumb
//! can be dragged. Each has its own `<output>`.
//!
//! **Accessibility:** native range inputs, each named (the pair's thumbs as "Minimum" and
//! "Maximum"); the value is shown in an `<output>`. Checked by axe-core in headless Firefox on
//! every demo route, both capability variants, light and dark (no serious or critical
//! violation).
//!
//! **What it does not do without script:** show the value while dragging (the `<output>` holds
//! the value the server last saw), or stop the two thumbs of a pair crossing; the server
//! reorders them with [`order`].
//!
//! **Fallback:** none needed. Without `list` support the ticks are simply not drawn. A pair
//! whose thumbs cross posts a low above the high; [`order`] swaps them back on the server.
//!
//! The look follows Radix Themes Slider: a thin gray track filled in the primary colour up to
//! the value (between the thumbs for a pair; `--lui-range-fill`, or `--lui-range-lo` and
//! `--lui-range-hi`, set by the server), a round white thumb, and `.ticks()` for the lowest,
//! middle and highest values under the track. The value stays beside the slider: CSS cannot
//! place a label over a thumb that moves without script.
//!
//! **Enhanced:** the enhancement script mirrors each slider into its `<output for>` and its
//! fill while it moves. Without it the `<output>` shows the value the server last saw.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The values are the query's (`volume`; `price_min` and `price_max` for a pair), or
//! // `.value(..)` and `.values(..)` (saved ones).
//! let ui = Ui::from_request("/", "volume=40&price_min=80&price_max=20", "");
//! let m = ui.range("volume", "Volume").step(5).render().into_string();
//! assert!(m.contains(r#"<label for="f-volume">Volume</label>"#) && m.contains("<output for=\"f-volume\">40</output>"));
//! assert!(ui.range("volume", "Volume").value(70).render().into_string().contains("<output for=\"f-volume\">70</output>"));
//! assert!(m.contains(r#"style="--lui-range-fill: 40%""#));
//! let ticked = ui.range("volume", "Volume").ticks().render().into_string();
//! assert!(ticked.contains(r#"<div class="lui-range-ticks" aria-hidden="true"><span>0</span>"#));
//! assert_eq!(lui! { Range("volume", "Volume") ticks; }.into_string(), ticked);
//! // A pair posted the wrong way round is put back in order.
//! let m = ui.range_pair("price", "Price").step(10).render().into_string();
//! assert!(m.contains("name=\"price_min\"") && m.contains("name=\"price_max\""));
//! assert!(m.contains("<output for=\"f-price_min\">20</output>"));
//!
//! // The same in `lui!`:
//! let same = lui! { RangePair("price", "Price") step=10; };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::{Strings, Text};
use crate::props::{Prop, PropKind};

/// A slider with the server's current value beside it, made by [`Ui::range`] or, as a
/// low/high pair over one track, by [`Ui::range_pair`]. 0 to 100 in steps of 1 unless told
/// otherwise.
///
/// **Setters.** Values and items: `.value(..)`, `.values(..)`, `.min(..)`, `.max(..)`,
/// `.step(..)`; switches: `.ticks()`.
#[derive(Clone, Debug)]
pub struct Range<'a> {
    name: &'a str,
    label: &'a str,
    /// The value, or a pair's low one, when known.
    value: Option<i64>,
    /// A pair's high value, when known; `None` inside `Some` for a single slider.
    high: Option<Option<i64>>,
    min: i64,
    max: i64,
    step: i64,
    ticks: bool,
    strings: &'static Strings,
}

impl Range<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("value", PropKind::Number, "value: i64")
            .attr("value")
            .doc("The value (a pair's low one), instead of the query's."),
        Prop::new("values", PropKind::Value, "low: i64, high: i64")
            .doc("A pair's two values, put in order, instead of the query's."),
        Prop::new("min", PropKind::Number, "min: i64")
            .default("0")
            .attr("min")
            .doc("Lowest value."),
        Prop::new("max", PropKind::Number, "max: i64")
            .default("100")
            .attr("max")
            .doc("Highest value."),
        Prop::new("step", PropKind::Number, "step: i64")
            .default("1")
            .attr("step")
            .doc("Distance between allowed values (at least 1)."),
        Prop::new("ticks", PropKind::Switch, "")
            .doc("The lowest, middle and highest values marked under the track."),
    ];
}

impl Ui {
    /// A range input named `name` under the label `label`, in a `div.lui-field` like a form
    /// field; at the query's `name` (the middle without one) unless [`Range::value`] says
    /// otherwise.
    pub fn range<'a>(&self, name: &'a str, label: &'a str) -> Range<'a> {
        Range {
            strings: self.strings,
            name,
            label,
            value: self.param(name).and_then(|v| v.trim().parse().ok()),
            high: None,
            min: 0,
            max: 100,
            step: 1,
            ticks: false,
        }
    }

    /// Two thumbs over one track, named `<name>_min` and `<name>_max`, under the label
    /// `label`; at the query's two values (the whole track without them) unless
    /// [`Range::values`] says otherwise, put in order if they crossed.
    pub fn range_pair<'a>(&self, name: &'a str, label: &'a str) -> Range<'a> {
        let at = |key: String| self.param(&key).and_then(|v| v.trim().parse().ok());
        Range {
            value: at(format!("{name}_min")),
            high: Some(at(format!("{name}_max"))),
            ..self.range(name, label)
        }
    }
}

impl<'a> Range<'a> {
    /// The value (a pair's low one), instead of the query's: a saved value.
    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// A pair's two values, put in order, instead of the query's; on a single slider, the
    /// value is `low`.
    pub fn values(mut self, low: i64, high: i64) -> Self {
        let (low, high) = order(low, high);
        self.value = Some(low);
        if self.high.is_some() {
            self.high = Some(Some(high));
        }
        self
    }

    /// Lowest value.
    pub fn min(mut self, min: i64) -> Self {
        self.min = min;
        self
    }

    /// Highest value.
    pub fn max(mut self, max: i64) -> Self {
        self.max = max;
        self
    }

    /// Distance between allowed values (at least 1).
    pub fn step(mut self, step: i64) -> Self {
        self.step = step.max(1);
        self
    }

    /// The lowest, middle and highest values marked under the track, with a label at each
    /// end.
    pub fn ticks(mut self) -> Self {
        self.ticks = true;
        self
    }
}

/// Where `value` sits between `min` and `max`, as a CSS percentage: what the track is filled
/// up to (`--lui-range-fill`, or `--lui-range-lo` and `--lui-range-hi` for a pair).
fn percent(value: i64, min: i64, max: i64) -> String {
    let span = (max - min).max(1) as f64;
    let at = ((value.clamp(min, max) - min) as f64 / span * 100.0).round();
    format!("{at}%")
}

impl Render for Range<'_> {
    fn render(&self) -> Markup {
        let Range {
            strings: _,
            name,
            label,
            value,
            high,
            min,
            max,
            step,
            ticks,
        } = *self;
        let ticks = ticks.then(|| {
            html! {
                div class="lui-range-ticks" aria-hidden="true" {
                    span { (min) } span {} span { (max) }
                }
            }
        });
        let value = match high {
            None => (value.unwrap_or((min + max) / 2), None),
            Some(high) => {
                let (lo, hi) = order(value.unwrap_or(min), high.unwrap_or(max));
                (lo, Some(hi))
            }
        };
        let control = match value {
            (value, None) => {
                let (list, id) = (format!("{name}-ticks"), format!("f-{name}"));
                html! {
                    div class="lui-range" {
                        div class="lui-range-main" {
                            input type="range" class="lui-range-input" id=(id) name=(name) min=(min) max=(max) step=(step) value=(value) list=(list)
                                style={ "--lui-range-fill: " (percent(value, min, max)) };
                            @if let Some(t) = &ticks { (t) }
                        }
                        datalist id=(list) {
                            option value=(min) label=(min) {}
                            option value=((min + max) / 2) {}
                            option value=(max) label=(max) {}
                        }
                        output for=(id) { (value) }
                    }
                }
            }
            (lo, Some(hi)) => {
                let (lo_id, hi_id) = (format!("f-{name}_min"), format!("f-{name}_max"));
                html! {
                    div class="lui-range lui-range-pair" {
                        div class="lui-range-main" {
                        div class="lui-range-track" style={ "--lui-range-lo: " (percent(lo, min, max)) "; --lui-range-hi: " (percent(hi, min, max)) } {
                            input type="range" class="lui-range-input" id=(lo_id) name={ (name) "_min" } min=(min) max=(max) step=(step) value=(lo) aria-label=(self.strings.get(Text::Minimum));
                            input type="range" class="lui-range-input" id=(hi_id) name={ (name) "_max" } min=(min) max=(max) step=(step) value=(hi) aria-label=(self.strings.get(Text::Maximum));
                        }
                        @if let Some(t) = &ticks { (t) }
                        }
                        span class="lui-range-values" { output for=(lo_id) { (lo) } " – " output for=(hi_id) { (hi) } }
                    }
                }
            }
        };
        // A pair's label names the group; it points at the low thumb.
        let id = if value.1.is_some() {
            format!("f-{name}_min")
        } else {
            format!("f-{name}")
        };
        crate::labelled(Some(label), &id, control)
    }
}
/// A posted pair in order: the thumbs can cross, the stored range should not.
pub fn order(a: i64, b: i64) -> (i64, i64) {
    (a.min(b), a.max(b))
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Slider (size 2) and the Origin UI sliders: a 6px --lui-gray-4 track filled
   in the primary colour up to the value, a 1rem round --lui-on-primary thumb with a hairline ring and
   a small shadow, a wide soft ring on focus. The fill comes from --lui-range-fill, which the
   server sets and the enhancement script keeps live while dragging; Firefox draws it itself
   with ::-moz-range-progress. The value sits beside the slider in its <output>. */
.lui-range { display: flex; align-items: flex-start; gap: var(--lui-space-3); }
.lui-range-main { flex: 1; min-width: 0; display: grid; gap: var(--lui-space-1); }
.lui-range-input {
  appearance: none; width: 100%; height: 1.25rem; margin: 0; padding: 0; border: 0;
  background: transparent; box-shadow: none; cursor: pointer; --lui-range-fill: 50%;
}
.lui-range-input::-webkit-slider-runnable-track {
  height: 0.375rem; border-radius: 9999px;
  background: linear-gradient(to right, var(--lui-primary) var(--lui-range-fill), var(--lui-gray-4) var(--lui-range-fill));
}
.lui-range-input::-moz-range-track { height: 0.375rem; border-radius: 9999px; background: var(--lui-gray-4); }
.lui-range-input::-moz-range-progress { height: 0.375rem; border-radius: 9999px; background: var(--lui-primary); }
.lui-range-input::-webkit-slider-thumb {
  appearance: none; width: 1rem; height: 1rem; margin-top: -0.3125rem; border-radius: 50%;
  background: var(--lui-on-primary); box-shadow: 0 0 0 1px var(--lui-gray-7), var(--lui-shadow-xs);
  transition: box-shadow var(--lui-duration-fast);
}
.lui-range-input::-moz-range-thumb {
  width: 1rem; height: 1rem; border: 0; border-radius: 50%;
  background: var(--lui-on-primary); box-shadow: 0 0 0 1px var(--lui-gray-7), var(--lui-shadow-xs);
  transition: box-shadow var(--lui-duration-fast);
}
.lui-range-input:focus-visible { outline: none; }
.lui-range-input:focus-visible::-webkit-slider-thumb { box-shadow: 0 0 0 1px var(--lui-primary), 0 0 0 5px color-mix(in srgb, var(--lui-ring) 40%, transparent); }
.lui-range-input:focus-visible::-moz-range-thumb { box-shadow: 0 0 0 1px var(--lui-primary), 0 0 0 5px color-mix(in srgb, var(--lui-ring) 40%, transparent); }
.lui-range-input:disabled { cursor: not-allowed; }
.lui-range-input:disabled::-webkit-slider-runnable-track { background: var(--lui-gray-4); }
.lui-range-input:disabled::-moz-range-progress { background: var(--lui-gray-8); }
.lui-range output, .lui-range-values { line-height: 1.25rem; min-width: 3ch; text-align: right; font-size: 0.875rem; font-variant-numeric: tabular-nums; text-wrap: nowrap; }
/* .ticks(): the lowest, middle and highest value under the track, the ends labelled. */
.lui-range-ticks { display: flex; justify-content: space-between; padding-inline: 0.5rem; color: var(--lui-muted); font-size: 0.75rem; line-height: 1rem; font-variant-numeric: tabular-nums; }
.lui-range-ticks > span { display: grid; justify-items: center; width: 0; white-space: nowrap; }
.lui-range-ticks > span::before { content: ""; width: 1px; height: 0.25rem; margin-bottom: 0.125rem; background: var(--lui-gray-7); }
/* A pair: two inputs share one grid cell over one drawn track, filled between the thumbs;
   only the thumbs catch the pointer. */
.lui-range-track { display: grid; align-items: center; min-height: 1.25rem; }
.lui-range-track::before {
  content: ""; grid-area: 1 / 1; height: 0.375rem; border-radius: 9999px;
  background: linear-gradient(to right, var(--lui-gray-4) var(--lui-range-lo, 0%), var(--lui-primary) var(--lui-range-lo, 0%) var(--lui-range-hi, 100%), var(--lui-gray-4) var(--lui-range-hi, 100%));
}
.lui-range-track .lui-range-input { grid-area: 1 / 1; pointer-events: none; }
.lui-range-track .lui-range-input::-webkit-slider-runnable-track { background: none; }
.lui-range-track .lui-range-input::-moz-range-track { background: none; }
.lui-range-track .lui-range-input::-moz-range-progress { background: none; }
.lui-range-track .lui-range-input::-webkit-slider-thumb { pointer-events: auto; }
.lui-range-track .lui-range-input::-moz-range-thumb { pointer-events: auto; }
/* Touch: a bigger thumb in a 44px tall input. */
@media (pointer: coarse) {
  .lui-range-input, .lui-range-track { height: var(--lui-hit); }
  .lui-range output, .lui-range-values { line-height: var(--lui-hit); }
  .lui-range-input::-webkit-slider-thumb { width: 1.5rem; height: 1.5rem; margin-top: -0.5625rem; }
  .lui-range-input::-moz-range-thumb { width: 1.5rem; height: 1.5rem; }
}
"#;
