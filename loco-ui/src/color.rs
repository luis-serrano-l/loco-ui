//! # Color
//!
//! A colour picker whose value the server remembers, no script. It can offer a row of preset
//! swatches and an opacity slider.
//!
//! **Platform features:** `<input type="color">` (Chrome 20, Firefox 29, Safari 12.1). The
//! swatch beside it is a plain `<span>` painted with the server's current value through the
//! custom properties `--lui-color-value` and `--lui-color-alpha`, mixed with `color-mix()`
//! (Chrome 111, Firefox 113, Safari 16.2) over a checkerboard so transparency shows. Presets
//! are `<button name="<name>-preset" value="#rrggbb">`: one click posts the form with that
//! colour. Opacity is an `<input type="range">` named `<name>-alpha` (0 to 100).
//!
//! **Accessibility:** a native colour input with its label; presets are named buttons in a
//! labelled group with `aria-pressed`. Checked by axe-core in headless Firefox on every demo
//! route, both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** a live preview of the chosen colour before the form
//! is sent, and an eyedropper.
//!
//! **Fallback:** none needed; a browser without a colour picker shows a text field that
//! accepts `#rrggbb`.
//!
//! **Enhanced:** the enhancement script repaints the swatch and the code while the picker moves
//! and mirrors the opacity into its `<output>`.
//!
//! ```rust
//! use loco_ui::{prelude::*, color::hex_alpha};
//! // The value is the query's `accent`, or `.value(..)` (a saved colour).
//! let ui = Ui::from_request("/", "accent=%232f5bea", "");
//! assert!(ui.color("accent", "Accent").render().into_string().contains(r##"value="#2f5bea""##));
//! let m = ui.color("accent", "Accent").value("#2f5bea").presets(&["#1f6f5f", "#b3261e"]).alpha(80);
//! let html = m.render().into_string();
//! assert!(html.contains("name=\"accent-preset\" value=\"#b3261e\""));
//! assert!(html.contains("name=\"accent-alpha\"") && html.contains(r#"<label for="f-accent">"#));
//! assert_eq!(hex_alpha("#2f5bea", 80), "#2f5beacc");
//!
//! // The same in `lui!`:
//! let same = lui! {
//!     Color("accent", "Accent") value="#2f5bea" presets=(&["#1f6f5f", "#b3261e"]) alpha=80;
//! };
//! assert_eq!(same.into_string(), m.render().into_string());
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::{Strings, Text};
use crate::props::{Key, Prop, PropKind};
use crate::{Caps, Ui};

/// A colour input with a swatch of its current value, made by [`Ui::color`].
///
/// **Setters.** Values and items: `.value(..)`, `.presets(..)`, `.alpha(..)`; switches:
/// `.disabled()`, `.readonly()`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color<'a> {
    name: &'a str,
    value: &'a str,
    presets: &'a [&'a str],
    alpha: Option<u8>,
    label: &'a str,
    strings: &'static Strings,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
}

impl Color<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves between the colour well, the opacity slider and the presets.",
        ),
        Key::new(
            "Enter / Space",
            "Opens the system colour picker from the colour well.",
        ),
        Key::new(
            "ArrowRight / ArrowLeft",
            "Raises or lowers the opacity by one percent.",
        ),
        Key::new("Home / End", "Sets the opacity to 0 or 100 percent."),
        Key::new(
            "Enter / Space",
            "Picks the focused preset, which posts the form.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("value", PropKind::Value, "value: &'a str")
            .attr("value")
            .doc("The `#rrggbb` colour, instead of the query's `name`."),
        Prop::new("presets", PropKind::Value, "presets: &'a [&'a str]")
            .doc("`#rrggbb` swatches that post `<name>-preset` when clicked."),
        Prop::new("alpha", PropKind::Number, "percent: u8")
            .doc("An opacity slider (`<name>-alpha`) at `percent`, clamped to 100."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("`disabled` and `aria-disabled`: shown greyed and not posted."),
        Prop::new("readonly", PropKind::Switch, "")
            .doc("Shown and posted, not changeable: disabled controls and hidden inputs with the values."),
    ];
}

impl Ui {
    /// A colour input named `name` under the label `label`, in a `div.lui-field` like a form
    /// field; it holds the query's `name` (`#000000` without one) unless [`Color::value`]
    /// says otherwise.
    pub fn color<'a>(&'a self, name: &'a str, label: &'a str) -> Color<'a> {
        Color {
            value: self.param(name).unwrap_or("#000000"),
            ..Color::bare(self.strings, name, label)
        }
    }
}

impl<'a> Color<'a> {
    /// A colour input with nothing read from a request: what a form's colour field starts
    /// from.
    pub(crate) fn bare(strings: &'static Strings, name: &'a str, label: &'a str) -> Self {
        Color {
            name,
            label,
            value: "#000000",
            strings,
            ..Color::default()
        }
    }

    /// `disabled` and `aria-disabled`: shown greyed and not posted.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Shown and posted, not changeable. HTML has no read-only colour input, so the controls
    /// are disabled and hidden inputs post the colour and its opacity.
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
}

impl<'a> Color<'a> {
    /// The `#rrggbb` colour, instead of the query's `name`: a saved value.
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    /// `#rrggbb` swatches that post `<name>-preset` when clicked.
    pub fn presets(mut self, presets: &'a [&'a str]) -> Self {
        self.presets = presets;
        self
    }

    /// An opacity slider (`<name>-alpha`) at `percent`, clamped to 100.
    pub fn alpha(mut self, percent: u8) -> Self {
        self.alpha = Some(percent.min(100));
        self
    }
}

/// `#rrggbb` plus an opacity percent as `#rrggbbaa`.
pub fn hex_alpha(hex: &str, percent: u8) -> String {
    format!(
        "{hex}{:02x}",
        (u32::from(percent.min(100)) * 255 + 50) / 100
    )
}

impl Render for Color<'_> {
    fn render(&self) -> Markup {
        let Color {
            strings: _,
            disabled,
            readonly,
            name,
            value,
            presets,
            alpha,
            label,
        } = *self;
        let id = crate::field_id(name);
        let pct = alpha.unwrap_or(100);
        let preset_name = format!("{name}-preset");
        let off = disabled || readonly;
        let aria_off = disabled.then_some("true");
        crate::labelled(
            Some(label),
            &id,
            html! {
                div class="lui-color" {
                    input type="color" class="lui-color-input" id=(id) name=(name) value=(value) disabled[off] aria-disabled=[aria_off];
                    @if readonly { (crate::input::keep(name, value)) }
                    span class="lui-color-value" {
                        span class="lui-color-swatch" style={ "--lui-color-value: " (value) "; --lui-color-alpha: " (pct) "%" } aria-hidden="true" {}
                        code { @if pct < 100 { (hex_alpha(value, pct)) } @else { (value) } }
                    }
                    @if alpha.is_some() {
                        label class="lui-color-alpha" {
                            (self.strings.get(Text::Opacity).replace("{}", "").trim_end()) " "
                            input type="range" class="lui-range-input lui-color-alpha-range" id={ (id) "-alpha" } name={ (name) "-alpha" } min="0" max="100" value=(pct) disabled[off]
                                style={ "--lui-range-fill: " (pct) "%" };
                            span { output for={ (id) "-alpha" } { (pct) } "%" }
                            @if readonly { (crate::input::keep(&format!("{name}-alpha"), &pct.to_string())) }
                        }
                    }
                    @if !presets.is_empty() {
                        span class="lui-color-presets" role="group" aria-label=(self.strings.get(Text::Presets)) {
                            @for p in presets {
                                @let use_p = self.strings.fill(Text::UseValue, &[p]);
                                @let b = Button::new(Caps::NONE, "").name(&preset_name).value(p).aria_label(&use_p).pressed(p.eq_ignore_ascii_case(value)).style(format!("--lui-color-value: {p}"));
                                (if off { b.disabled() } else { b })
                            }
                        }
                    }
                }
            },
        )
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Origin UI colour inputs: the native picker as a square well, then one chip with
   a round swatch (over a checkerboard, for opacity) and the hex value, the opacity slider in
   the range look, and presets as a row of round swatches. Wraps by itself. */
.lui-color { display: inline-flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-2); max-width: 100%; }
.lui-color-input { width: var(--lui-control-h); height: var(--lui-control-h); padding: 0.1875rem; cursor: pointer; }
.lui-color-input::-webkit-color-swatch-wrapper { padding: 0; }
.lui-color-input::-webkit-color-swatch { border: 0; border-radius: calc(var(--lui-radius-sm) - 2px); }
.lui-color-input::-moz-color-swatch { border: 0; border-radius: calc(var(--lui-radius-sm) - 2px); }
.lui-color-value {
  display: inline-flex; align-items: center; gap: var(--lui-space-2); height: var(--lui-control-h); box-sizing: border-box;
  padding: 0 0.75rem 0 0.5rem; border: 1px solid var(--lui-input); border-radius: var(--lui-radius-sm);
}
.lui-color-value code { font-family: var(--lui-font-mono); font-size: var(--lui-text-2); background: none; border: 0; padding: 0; }
.lui-color-swatch {
  width: 1.25rem; height: 1.25rem; border-radius: 50%; box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--lui-fg) 15%, transparent);
  background: linear-gradient(color-mix(in srgb, var(--lui-color-value) var(--lui-color-alpha, 100%), transparent) 0 0),
    repeating-conic-gradient(var(--lui-line) 0 25%, var(--lui-surface) 0 50%) 0 0 / 0.5rem 0.5rem;
}
.lui-color-alpha { display: inline-flex; align-items: center; gap: var(--lui-space-2); font-weight: 400; font-size: var(--lui-text-2); }
.lui-color-alpha-range { width: 8rem; }
.lui-color-alpha output { min-width: 3ch; text-align: end; font-variant-numeric: tabular-nums; }
.lui-color-presets { display: flex; flex-wrap: wrap; flex-basis: 100%; gap: var(--lui-space-2); }
/* A preset is a button drawn as a round swatch of its colour, ringed when it is the value. */
.lui-color-presets .lui-button {
  width: 1.75rem; height: 1.75rem; min-height: 0; padding: 0; border-radius: 50%;
  background: var(--lui-color-value); border: 2px solid var(--lui-bg); box-shadow: 0 0 0 1px var(--lui-input);
}
.lui-color-presets .lui-button:hover { background: var(--lui-color-value); box-shadow: 0 0 0 1px var(--lui-gray-8); }
.lui-color-presets .lui-button[aria-pressed=true] { box-shadow: 0 0 0 2px var(--lui-primary); }
@media (pointer: coarse) { .lui-color-presets .lui-button { width: var(--lui-hit); height: var(--lui-hit); } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_render_is_a_labelled_colour_input_with_a_swatch_and_code() {
        let ui = Ui::default();
        let m = ui.color("accent", "Accent").render().into_string();
        assert!(m.contains(r##"<input type="color" class="lui-color-input" id="f-accent" name="accent" value="#000000">"##), "{m}");
        assert!(m.contains(r#"<label for="f-accent">Accent</label>"#), "{m}");
        assert!(
            m.contains("lui-color-swatch") && m.contains("<code>#000000</code>"),
            "{m}"
        );
        assert!(
            !m.contains("class=\"lui-color-alpha\"") && !m.contains("lui-color-presets"),
            "{m}"
        );
    }

    #[test]
    fn the_value_comes_from_the_query_unless_set() {
        let ui = Ui::from_request("/", "accent=%23112233", "");
        let m = ui.color("accent", "A").render().into_string();
        assert!(m.contains(r##"value="#112233""##), "{m}");
        let m = ui
            .color("accent", "A")
            .value("#445566")
            .render()
            .into_string();
        assert!(
            m.contains(r##"value="#445566""##) && !m.contains("#112233"),
            "{m}"
        );
    }

    #[test]
    fn alpha_adds_a_slider_and_shows_the_hex_with_alpha_in_the_code() {
        let m = Ui::default()
            .color("c", "C")
            .value("#2f5bea")
            .alpha(80)
            .render()
            .into_string();
        assert!(
            m.contains(r#"name="c-alpha""#) && m.contains(r#"value="80""#),
            "{m}"
        );
        assert!(
            m.contains("<code>#2f5beacc</code>") && m.contains("--lui-color-alpha: 80%"),
            "{m}"
        );
        let over = Ui::default()
            .color("c", "C")
            .alpha(250)
            .render()
            .into_string();
        assert!(over.contains(r#"value="100""#), "{over}");
    }

    #[test]
    fn presets_are_named_buttons_and_the_current_one_is_pressed() {
        let m = Ui::default()
            .color("c", "C")
            .value("#B3261E")
            .presets(&["#1f6f5f", "#b3261e"])
            .render()
            .into_string();
        assert!(m.contains(r##"name="c-preset" value="#1f6f5f""##), "{m}");
        assert!(m.contains(r##"name="c-preset" value="#b3261e""##), "{m}");
        assert_eq!(m.matches(r#"aria-pressed="true""#).count(), 1, "{m}");
        assert!(m.contains(r#"role="group""#), "{m}");
    }
}
