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
use crate::props::{Key, Prop, PropKind};

/// A slider with the server's current value beside it, made by [`Ui::range`] or, as a
/// low/high pair over one track, by [`Ui::range_pair`]. 0 to 100 in steps of 1 unless told
/// otherwise.
///
/// **Setters.** Values and items: `.value(..)`, `.values(..)`, `.min(..)`, `.max(..)`,
/// `.step(..)`, `.id(..)`; switches: `.ticks()`, `.disabled()`, `.readonly()`.
#[derive(Clone, Debug)]
pub struct Range<'a> {
    pub(crate) id: Option<&'a str>,
    name: &'a str,
    label: &'a str,
    /// The value, or a pair's low one, when known.
    value: Option<f64>,
    /// A pair's high value, when known; `None` inside `Some` for a single slider.
    high: Option<Option<f64>>,
    min: f64,
    max: f64,
    step: f64,
    ticks: bool,
    strings: &'static Strings,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
}

impl Range<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves focus to the slider, or between the two thumbs of a pair.",
        ),
        Key::new("ArrowRight / ArrowUp", "Raises the value by one step."),
        Key::new("ArrowLeft / ArrowDown", "Lowers the value by one step."),
        Key::new("Home / End", "Sets the value to the minimum or maximum."),
        Key::new(
            "PageUp / PageDown",
            "Raises or lowers the value by a larger amount the browser picks.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("value", PropKind::Number, "value: impl Into<f64>")
            .attr("value")
            .doc("The value (a pair's low one), instead of the query's."),
        Prop::new(
            "values",
            PropKind::Value,
            "low: impl Into<f64>, high: impl Into<f64>",
        )
        .doc("A pair's two values, put in order, instead of the query's."),
        Prop::new("min", PropKind::Number, "min: impl Into<f64>")
            .default("0")
            .attr("min")
            .doc("Lowest value."),
        Prop::new("max", PropKind::Number, "max: impl Into<f64>")
            .default("100")
            .attr("max")
            .doc("Highest value."),
        Prop::new("step", PropKind::Number, "step: impl Into<f64>")
            .default("1")
            .attr("step")
            .doc("Distance between allowed values (above 0)."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("`disabled` and `aria-disabled`: shown greyed and not posted."),
        Prop::new("readonly", PropKind::Switch, "").doc(
            "Shown and posted, not movable: a disabled slider and a hidden input with its value.",
        ),
        Prop::new("ticks", PropKind::Switch, "")
            .doc("The lowest, middle and highest values marked under the track."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The slider's id, `f-<name>` by default."),
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
            ..Range::bare(self.strings, name, label)
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
    /// A slider with nothing read from a request: what a form's range field starts from.
    pub(crate) fn bare(strings: &'static Strings, name: &'a str, label: &'a str) -> Self {
        Range {
            strings,
            name,
            label,
            id: None,
            value: None,
            high: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            ticks: false,
            disabled: false,
            readonly: false,
        }
    }

    /// `disabled` and `aria-disabled`: shown greyed and not posted.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// The slider's id when a field gives one.
    pub(crate) fn id_from(mut self, id: Option<&'a str>) -> Self {
        self.id = id;
        self
    }

    /// The slider's id, `f-<name>` by default (a pair's two are `<id>_min` and `<id>_max`);
    /// its ticks list is `<id>-ticks` then.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Shown and posted, not movable. HTML has no read-only slider, so the slider is
    /// disabled and a hidden input posts its value.
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
}

impl<'a> Range<'a> {
    /// The value (a pair's low one), instead of the query's: a saved value.
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// A pair's two values, put in order, instead of the query's; on a single slider, the
    /// value is `low`.
    pub fn values(mut self, low: impl Into<f64>, high: impl Into<f64>) -> Self {
        let (low, high) = order(low.into(), high.into());
        self.value = Some(low);
        if self.high.is_some() {
            self.high = Some(Some(high));
        }
        self
    }

    /// Lowest value.
    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = min.into();
        self
    }

    /// Highest value.
    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = max.into();
        self
    }

    /// Distance between allowed values (above 0; 1 when given 0 or less).
    pub fn step(mut self, step: impl Into<f64>) -> Self {
        let step = step.into();
        self.step = if step > 0.0 { step } else { 1.0 };
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
fn percent(value: f64, min: f64, max: f64) -> String {
    let span = (max - min).max(f64::EPSILON);
    let at = ((value.clamp(min, max.max(min)) - min) / span * 100.0).round();
    format!("{at}%")
}

impl Render for Range<'_> {
    fn render(&self) -> Markup {
        let Range {
            id: own,
            strings: _,
            disabled,
            readonly,
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
        // The middle, on a step.
        let mid = crate::input::tidy(min + ((max - min) / 2.0 / step).round() * step, step, min);
        let off = disabled || readonly;
        let aria_off = disabled.then_some("true");
        let value = match high {
            None => (value.unwrap_or(mid), None),
            Some(high) => {
                let (lo, hi) = order(value.unwrap_or(min), high.unwrap_or(max));
                (lo, Some(hi))
            }
        };
        let base = own.map_or_else(|| crate::field_id(name), str::to_string);
        let control = match value {
            (value, None) => {
                let list = own.map_or_else(|| format!("{name}-ticks"), |i| format!("{i}-ticks"));
                let id = base.clone();
                html! {
                    div class="lui-range" {
                        div class="lui-range-main" {
                            input type="range" class="lui-range-input" id=(id) name=(name) min=(min) max=(max) step=(step) value=(value) list=(list)
                                disabled[off] aria-disabled=[aria_off] style={ "--lui-range-fill: " (percent(value, min, max)) };
                            @if let Some(t) = &ticks { (t) }
                        }
                        datalist id=(list) {
                            option value=(min) label=(min) {}
                            option value=(mid) {}
                            option value=(max) label=(max) {}
                        }
                        output for=(id) { (value) }
                        @if readonly { (crate::input::keep(name, &value.to_string())) }
                    }
                }
            }
            (lo, Some(hi)) => {
                let (lo_id, hi_id) = (format!("{base}_min"), format!("{base}_max"));
                html! {
                    div class="lui-range lui-range-pair" {
                        div class="lui-range-main" {
                        div class="lui-range-track" style={ "--lui-range-lo: " (percent(lo, min, max)) "; --lui-range-hi: " (percent(hi, min, max)) } {
                            input type="range" class="lui-range-input" id=(lo_id) name={ (name) "_min" } min=(min) max=(max) step=(step) value=(lo) aria-label=(self.strings.get(Text::Minimum)) disabled[off] aria-disabled=[aria_off];
                            input type="range" class="lui-range-input" id=(hi_id) name={ (name) "_max" } min=(min) max=(max) step=(step) value=(hi) aria-label=(self.strings.get(Text::Maximum)) disabled[off] aria-disabled=[aria_off];
                        }
                        @if let Some(t) = &ticks { (t) }
                        }
                        span class="lui-range-values" { output for=(lo_id) { (lo) } " – " output for=(hi_id) { (hi) } }
                        @if readonly {
                            (crate::input::keep(&format!("{name}_min"), &lo.to_string()))
                            (crate::input::keep(&format!("{name}_max"), &hi.to_string()))
                        }
                    }
                }
            }
        };
        // A pair's label names the group; it points at the low thumb.
        let id = if value.1.is_some() {
            format!("{base}_min")
        } else {
            base
        };
        crate::labelled(Some(label), &id, control)
    }
}
/// A posted pair in order: the thumbs can cross, the stored range should not.
pub fn order<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if b < a { (b, a) } else { (a, b) }
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
.lui-range output, .lui-range-values { line-height: 1.25rem; min-width: 3ch; text-align: end; font-size: var(--lui-text-2); font-variant-numeric: tabular-nums; text-wrap: nowrap; }
/* .ticks(): the lowest, middle and highest value under the track, the ends labelled. */
.lui-range-ticks { display: flex; justify-content: space-between; padding-inline: 0.5rem; color: var(--lui-muted); font-size: var(--lui-text-1); line-height: 1rem; font-variant-numeric: tabular-nums; }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_slider_has_a_label_an_output_a_datalist_and_takes_the_query_value() {
        let m = Ui::default()
            .range("vol", "Volume")
            .value(30)
            .render()
            .into_string();
        assert!(m.contains(r#"<label for="f-vol">Volume</label>"#), "{m}");
        assert!(m.contains(r#"type="range" class="lui-range-input" id="f-vol" name="vol" min="0" max="100" step="1" value="30" list="vol-ticks""#), "{m}");
        assert!(m.contains(r#"style="--lui-range-fill: 30%""#), "{m}");
        assert!(m.contains(r#"<output for="f-vol">30</output>"#), "{m}");
        assert!(m.contains(r#"<datalist id="vol-ticks">"#), "{m}");
        assert!(
            !m.contains("lui-range-ticks") && !m.contains("lui-range-pair"),
            "{m}"
        );
    }

    #[test]
    fn the_query_supplies_the_value_and_a_default_sits_in_the_middle() {
        let q = Ui::from_request("/", "vol=75", "");
        assert!(
            q.range("vol", "Volume")
                .render()
                .into_string()
                .contains(r#"value="75""#)
        );
        let m = Ui::default().range("vol", "Volume").render().into_string();
        assert!(m.contains(r#"value="50""#), "{m}");
    }

    #[test]
    fn ticks_mark_the_lowest_and_highest_values() {
        let m = Ui::default()
            .range("vol", "Volume")
            .min(10)
            .max(90)
            .ticks()
            .render()
            .into_string();
        assert!(m.contains(r#"<div class="lui-range-ticks" aria-hidden="true"><span>10</span><span></span><span>90</span></div>"#), "{m}");
    }

    #[test]
    fn a_pair_is_put_in_order_and_names_its_thumbs() {
        let m = Ui::default()
            .range_pair("price", "Price")
            .values(80, 20)
            .render()
            .into_string();
        assert!(m.contains("lui-range-pair"), "{m}");
        assert!(
            m.contains(r#"name="price_min""#) && m.contains(r#"name="price_max""#),
            "{m}"
        );
        assert!(m.contains(r#"value="20" aria-label="#), "{m}");
        assert!(
            m.contains(r#"style="--lui-range-lo: 20%; --lui-range-hi: 80%""#),
            "{m}"
        );
        assert!(
            m.contains(
                r#"<output for="f-price_min">20</output> – <output for="f-price_max">80</output>"#
            ),
            "{m}"
        );
        assert!(
            m.contains(r#"<label for="f-price_min">Price</label>"#),
            "{m}"
        );
    }

    #[test]
    fn readonly_posts_a_hidden_value_and_disabled_does_not() {
        let ro = Ui::default()
            .range("vol", "Volume")
            .value(40)
            .readonly()
            .render()
            .into_string();
        assert!(
            ro.contains("disabled")
                && ro.contains(r#"type="hidden""#)
                && ro.contains(r#"name="vol""#),
            "{ro}"
        );
        let dis = Ui::default()
            .range("vol", "Volume")
            .disabled()
            .render()
            .into_string();
        assert!(
            dis.contains(r#"aria-disabled="true""#) && !dis.contains(r#"type="hidden""#),
            "{dis}"
        );
    }
}
