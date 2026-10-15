//! # Progress
//!
//! How far along a task is: a bar filled to a value out of a maximum, or an indeterminate
//! bar (`.indeterminate()`) when the server does not know yet; `.show_value()` writes the
//! percentage beside it. Radix Themes Progress's props: three heights
//! (`.size(..)`), a [`Tone`] for the fill, a soft or surface track
//! (`.variant(..)`) and `.radius(..)`.
//!
//! **Platform features:** `<progress>` (baseline 2013) with its label tied by `<label for>`,
//! so a screen reader says the value; `appearance: none` and the `::-webkit-progress-*` and
//! `::-moz-progress-bar` pseudo-elements draw it in the shadcn shape. An indeterminate bar is
//! a `<progress>` with no value, matched by `:indeterminate` (baseline 2020) and drawn as a
//! sliding stripe that `prefers-reduced-motion: reduce` stops.
//!
//! **Accessibility:** a native `<progress>` with its label. Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** move on its own; the value is what the server knew
//! when it rendered the page (a streamed page or the enhancement script can send a newer one).
//!
//! **Fallback:** without the pseudo-elements a browser draws its own bar.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.progress(33, 100).label("Upload").render().into_string();
//! assert!(m.contains(r#"<progress id="lui-progress-upload" class="lui-progress" value="33" max="100">33%</progress>"#));
//! assert!(!m.contains("lui-progress-value"), "the percentage only with .show_value()");
//! let m = ui.progress(33, 100).label("Upload").show_value().render().into_string();
//! assert!(m.contains(r#"<span class="lui-progress-value">33%</span>"#));
//! assert_eq!(lui! { Progress(33, 100) label="Upload" show_value; }.into_string(), m);
//! let m = ui.progress(0, 100).label("Waiting").indeterminate().render().into_string();
//! assert!(m.contains(r#"class="lui-progress">"#) && !m.contains("value="));
//! // The same in `lui!`:
//! let same = lui! { Progress(0, 100) label="Waiting" indeterminate; };
//! assert_eq!(same.into_string(), m);
//! // A thin green bar on a tinted track, square ends.
//! let m = ui.progress(9, 10).size(1).tone(Tone::Ok).variant(Variant::Soft).radius(Radius::None).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-progress-field" data-size="1" data-tone="ok" data-variant="soft" data-radius="none">"#));
//! assert_eq!(lui! { Progress(9, 10) size=1 tone="ok" variant="soft" radius="none"; }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::props::{Prop, PropKind};
use crate::variant::{Radius, Size, Tone, Variant};
use crate::{Ui, slug};

/// A progress bar, made by [`Ui::progress`].
///
/// **Setters.** Values and items: `.label(..)`, `.id(..)`, `.size(..)`, `.tone(..)`,
/// `.variant(..)`, `.radius(..)`; switches: `.indeterminate()`, `.show_value()`.
#[derive(Clone, Debug)]
pub struct Progress<'a> {
    value: u64,
    max: u64,
    label: Option<&'a str>,
    id: Option<&'a str>,
    size: Size,
    tone: Option<Tone>,
    variant: Option<Variant>,
    radius: Option<Radius>,
    indeterminate: bool,
    show_value: bool,
}

impl Progress<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("label", PropKind::Value, "text: &'a str").doc("A label above the bar."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The bar's id instead of `lui-progress-<label>`."),
        Prop::new(
            "size",
            PropKind::Choice { of: Size::NAMES },
            "size: impl Into<Size>",
        )
        .default("2")
        .doc("The bar's height: 0.25, 0.5 or 0.75rem."),
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .default("brand")
            .doc("The fill's colour."),
        Prop::new(
            "variant",
            PropKind::Choice { of: Variant::NAMES },
            "variant: Variant",
        )
        .default("surface")
        .doc("Soft tints the track with the tone; the others keep the gray track."),
        Prop::new(
            "radius",
            PropKind::Choice { of: Radius::NAMES },
            "radius: Radius",
        )
        .default("full")
        .doc("The ends of the track and the fill, from square to round."),
        Prop::new("indeterminate", PropKind::Switch, "")
            .doc("No value yet: a stripe slides along the track."),
        Prop::new("show_value", PropKind::Switch, "")
            .doc("The percentage beside the label, or above the bar without one."),
    ];
}

impl Ui {
    /// A bar filled to `value` of `max`; [`Progress::indeterminate`] when there is no value
    /// yet (a `max` of 0 draws one too).
    pub fn progress<'a>(&self, value: u64, max: u64) -> Progress<'a> {
        Progress {
            value,
            max,
            label: None,
            id: None,
            size: Size::S2,
            tone: None,
            variant: None,
            radius: None,
            indeterminate: false,
            show_value: false,
        }
    }
}

impl<'a> Progress<'a> {
    /// A label above the bar.
    pub fn label(mut self, text: &'a str) -> Self {
        self.label = Some(text);
        self
    }

    /// The bar's id instead of `lui-progress-<label>`.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// The bar's height: 0.25rem at size 1, 0.5rem at 2 (the default), 0.75rem at 3.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// The fill's colour: brand (the default), neutral, ok, warn, danger or info.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// Soft tints the track with the tone (Radix's soft); surface, the default, and the
    /// others keep the gray track.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// The ends of the track and the fill: none, small, medium, large or full (the default).
    pub fn radius(mut self, radius: Radius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// No value yet: the `<progress>` has none, and a stripe slides along the track.
    pub fn indeterminate(mut self) -> Self {
        self.indeterminate = true;
        self
    }

    /// The percentage beside the label, or above the bar without one.
    pub fn show_value(mut self) -> Self {
        self.show_value = true;
        self
    }
}

impl Render for Progress<'_> {
    fn render(&self) -> Markup {
        let known = self.max > 0 && !self.indeterminate;
        let shown = known && self.show_value;
        let pct = if known {
            (self.value.min(self.max) * 100) / self.max
        } else {
            0
        };
        let id = self.id.map_or_else(
            || format!("lui-progress-{}", slug(self.label.unwrap_or("bar"))),
            str::to_string,
        );
        html! {
            div class="lui-progress-field" data-size=[(self.size != Size::S2).then(|| self.size.as_str())]
                data-tone=[self.tone.map(Tone::as_str)] data-variant=[self.variant.map(Variant::as_str)]
                data-radius=[self.radius.map(Radius::as_str)] {
                @if self.label.is_some() || shown {
                    label for=(id) {
                        span { @if let Some(l) = self.label { (l) } }
                        @if shown { span class="lui-progress-value" { (pct) "%" } }
                    }
                }
                @if known {
                    progress id=(id) class="lui-progress" value=(self.value.min(self.max)) max=(self.max) { (pct) "%" }
                } @else {
                    progress id=(id) class="lui-progress" {}
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. shadcn Progress: an 8px
/// rounded track in the primary colour at 20%, the fill in the primary colour.
pub const CSS: &str = r#"
/* Tremor ProgressBar: the label and the value on one line over a rounded --lui-gray-4 track
   with the primary fill. */
.lui-progress-field { display: grid; gap: var(--lui-space-2); }
.lui-progress-field label { display: flex; justify-content: space-between; font-size: var(--lui-text-2); font-weight: 500; }
.lui-progress-value { color: var(--lui-muted); font-variant-numeric: tabular-nums; font-weight: 400; direction: ltr; unicode-bidi: isolate; }
.lui-progress {
  appearance: none; display: block; width: 100%; height: var(--lui-progress-h, 0.5rem); border: 0; border-radius: var(--lui-progress-radius, 9999px); overflow: hidden;
  background: var(--lui-progress-track, var(--lui-gray-4)); accent-color: var(--lui-progress-fill, var(--lui-primary));
}
.lui-progress::-webkit-progress-bar { background: transparent; }
.lui-progress::-webkit-progress-value { background: var(--lui-progress-fill, var(--lui-primary)); border-radius: var(--lui-progress-radius, 9999px); transition: width var(--lui-duration-slow) var(--lui-ease-out); }
.lui-progress::-moz-progress-bar { background: var(--lui-progress-fill, var(--lui-primary)); border-radius: var(--lui-progress-radius, 9999px); }
/* .size(..), .tone(..), .variant(..) and .radius(..) on the field set the bar's custom
   properties; soft tints the track with the fill. */
.lui-progress-field[data-size="1"] { --lui-progress-h: 0.25rem; }
.lui-progress-field[data-size="3"] { --lui-progress-h: 0.75rem; }
.lui-progress-field[data-tone=neutral] { --lui-progress-fill: var(--lui-gray-11); }
.lui-progress-field[data-tone=ok] { --lui-progress-fill: var(--lui-ok); }
.lui-progress-field[data-tone=warn] { --lui-progress-fill: var(--lui-warn); }
.lui-progress-field[data-tone=danger] { --lui-progress-fill: var(--lui-danger); }
.lui-progress-field[data-variant=soft] { --lui-progress-track: color-mix(in srgb, var(--lui-progress-fill, var(--lui-primary)) 20%, transparent); }
.lui-progress-field[data-radius=none] { --lui-progress-radius: 0; }
.lui-progress-field[data-radius=small] { --lui-progress-radius: var(--lui-radius-sm); }
.lui-progress-field[data-radius=medium] { --lui-progress-radius: var(--lui-radius); }
.lui-progress-field[data-radius=large] { --lui-progress-radius: var(--lui-radius-lg); }
/* Indeterminate: a stripe a third of the track wide slides across it. */
.lui-progress:indeterminate {
  background-image: linear-gradient(90deg, transparent 0 33%, var(--lui-progress-fill, var(--lui-primary)) 33% 67%, transparent 67%);
  background-size: 300% 100%; animation: lui-progress-slide 1.4s ease-in-out infinite;
}
.lui-progress:indeterminate::-moz-progress-bar { background: transparent; }
@keyframes lui-progress-slide { from { background-position: 100% 0; } to { background-position: 0 0; } }
@media (prefers-reduced-motion: reduce) { .lui-progress:indeterminate { animation: none; background-position: 50% 0; } }
"#;
