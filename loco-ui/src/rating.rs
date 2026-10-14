//! # Rating
//!
//! A row of stars to pick a score from 1 to 5 (or `.max(n)`), part of the form around it:
//! each star is a radio button, so the form posts the number and the arrow keys move it.
//! `.readonly()` shows a score without letting it change, and still posts it.
//!
//! After the Origin UI star ratings: 1.25rem stars in --lui-gray-7, filled in the warning
//! colour up to the picked one, the stars up to the pointer filled while it hovers.
//!
//! **Platform features:** `<fieldset>` of `<input type="radio">` (every browser), each star an
//! inline SVG; `:has()` (Chrome 105, Firefox 121, Safari 15.4) fills the stars before the
//! checked one and the ones up to the hovered star.
//!
//! **Accessibility:** a `<fieldset>` named by its legend, each radio named "3 stars"
//! (visually hidden text), arrow keys move the pick as in any radio group; a read-only rating
//! is an image named "3 out of 5". Checked by axe-core in headless Firefox on every demo
//! route, both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing: picking a star is choosing a
//! radio, and the surrounding form sends it.
//!
//! **Fallback:** without `:has()` only the picked star is filled (an adjacent-sibling rule),
//! not the ones before it, and hovering fills nothing.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The score is the query's `stars`, or `.value(..)` (a saved one).
//! let ui = Ui::from_request("/review", "stars=4", "");
//! let m = ui.rating("stars", "Your rating").render().into_string();
//! assert!(m.contains(r#"<legend>Your rating</legend>"#));
//! assert!(m.contains(r#"type="radio" name="stars" value="4" checked"#));
//! assert!(m.contains(r#"<span class="lui-sr">4 stars</span>"#));
//! // The same in `lui!`:
//! assert_eq!(lui! { Rating("stars", "Your rating"); }.into_string(), m);
//! // Out of 10, shown and posted but not changeable.
//! let m = ui.rating("score", "Score").max(10).value(7).readonly().render().into_string();
//! assert!(m.contains(r#"role="img" aria-label="7 out of 10""#));
//! assert!(m.contains(r#"<input type="hidden" name="score" value="7">"#));
//! assert_eq!(lui! { Rating("score", "Score") max=10 value=7 readonly; }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::{Strings, Text};
use crate::icon::Icon;
use crate::input::{described_by, error_line, keep};
use crate::props::{Prop, PropKind};

/// A star rating, made by [`Ui::rating`]. Five stars, nothing picked, unless told otherwise.
///
/// **Setters.** Values and items: `.max(..)`, `.value(..)`, `.help(..)`, `.error(..)`,
/// `.id(..)`; switches: `.required()`, `.readonly()`, `.disabled()`.
#[derive(Clone, Debug)]
pub struct Rating<'a> {
    name: &'a str,
    label: &'a str,
    max: u8,
    value: u8,
    required: bool,
    readonly: bool,
    disabled: bool,
    help: Option<&'a str>,
    error: Option<&'a str>,
    id: Option<&'a str>,
    strings: &'static Strings,
}

impl Rating<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("max", PropKind::Number, "stars: u8")
            .default("5")
            .doc("How many stars (1 to 10)."),
        Prop::new("value", PropKind::Number, "stars: u8")
            .attr("checked")
            .doc("The picked score, instead of the query's `name`; 0 for none."),
        Prop::new("required", PropKind::Switch, "")
            .attr("required")
            .doc("A star must be picked before the form submits."),
        Prop::new("readonly", PropKind::Switch, "")
            .doc("The score shown as an image and posted from a hidden input, not changeable."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("`disabled` and `aria-disabled`: shown greyed and not posted."),
        Prop::new("help", PropKind::Value, "help: &'a str").doc("Help text under the stars."),
        Prop::new("error", PropKind::Value, "message: &'a str")
            .doc("A server message under the stars."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("What the stars' ids start from, `f-<name>` by default."),
    ];
}

impl Ui {
    /// Stars posting `name` (1 to 5) under the legend `label`, at the query's `name` unless
    /// [`Rating::value`] says otherwise.
    pub fn rating<'a>(&self, name: &'a str, label: &'a str) -> Rating<'a> {
        Rating {
            name,
            label,
            max: 5,
            value: self
                .param(name)
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or(0),
            required: false,
            readonly: false,
            disabled: false,
            help: None,
            error: None,
            id: None,
            strings: self.strings,
        }
    }
}

impl<'a> Rating<'a> {
    /// How many stars, 1 to 10 (5 by default).
    pub fn max(mut self, stars: u8) -> Self {
        self.max = stars.clamp(1, 10);
        self
    }

    /// The picked score, instead of the query's `name`: a saved one; 0 for none.
    pub fn value(mut self, stars: u8) -> Self {
        self.value = stars;
        self
    }

    /// A star must be picked before the form submits.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// The score shown as an image ("4 out of 5") and posted from a hidden input, not
    /// changeable: a review's own score on its edit page.
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    /// `disabled` and `aria-disabled`: shown greyed and not posted.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Help text under the stars.
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// A server message under the stars; the group gets `aria-invalid`. An empty message is
    /// no error.
    pub fn error(mut self, message: &'a str) -> Self {
        self.error = (!message.is_empty()).then_some(message);
        self
    }

    /// What the stars' ids start from (`<id>-1`..), `f-<name>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Render for Rating<'_> {
    fn render(&self) -> Markup {
        let id = self
            .id
            .map_or_else(|| format!("f-{}", self.name), str::to_string);
        let value = self.value.min(self.max);
        let described = described_by(&[
            self.help.map(|_| format!("{id}-help")),
            self.error.map(|_| format!("{id}-error")),
        ]);
        let after = html! {
            @if let Some(h) = self.help { small id={ (id) "-help" } class="lui-field-help" { (h) } }
            @if let Some(e) = self.error { (error_line(&format!("{id}-error"), e)) }
        };
        if self.readonly {
            let of = self.strings.fill(Text::RatingOf, &[&value, &self.max]);
            return html! {
                div class="lui-field lui-rating" data-readonly aria-describedby=[described.as_deref()] {
                    span class="lui-rating-label" { (self.label) }
                    span class="lui-rating-stars" role="img" aria-label=(of) {
                        @for n in 1..=self.max {
                            span class={ "lui-rating-star" @if n <= value { " lui-rating-on" } } { (Icon::Star) }
                        }
                    }
                    @if value > 0 { (keep(self.name, &value.to_string())) }
                    (after)
                }
            };
        }
        html! {
            fieldset class="lui-field lui-rating" disabled[self.disabled] aria-disabled=[self.disabled.then_some("true")]
                aria-describedby=[described.as_deref()] aria-invalid=[self.error.map(|_| "true")] {
                legend { (self.label) @if self.required { " " (crate::input::REQUIRED) } }
                span class="lui-rating-stars" {
                    @for n in 1..=self.max {
                        @let sid = format!("{id}-{n}");
                        label for=(sid) class="lui-rating-star" {
                            input id=(sid) class="lui-rating-input" type="radio" name=(self.name) value=(n)
                                checked[n == value] required[self.required && n == 1];
                            span class="lui-sr" { (self.strings.count(Text::Stars, u64::from(n))) }
                            (Icon::Star)
                        }
                    }
                }
                (after)
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Origin UI star ratings: 1.25rem stars in --lui-gray-7 with no fill, the picked
   one and those before it filled and stroked in --lui-warn; while the pointer is over the
   row, the stars up to it are filled instead. Each star is a 2rem target (2.75rem on touch). */
.lui-rating { margin: 0; padding: 0; border: 0; min-width: 0; }
.lui-rating legend, .lui-rating-label { padding: 0; margin-bottom: 0.5rem; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); font-weight: 500; }
.lui-rating[aria-invalid=true] legend { color: var(--lui-danger); }
.lui-rating-stars { display: inline-flex; flex-wrap: wrap; }
.lui-rating-star { position: relative; display: inline-grid; place-items: center; width: 2rem; height: 2rem; color: var(--lui-gray-7); cursor: pointer; }
.lui-rating[data-readonly] .lui-rating-star { width: 1.5rem; cursor: default; }
.lui-rating-star > .lui-icon { width: 1.25rem; height: 1.25rem; transition: color var(--lui-duration-fast), fill var(--lui-duration-fast); }
.lui-rating-input { position: absolute; inset: 0; width: 100%; height: 100%; margin: 0; opacity: 0; cursor: pointer; }
.lui-rating-input:checked ~ .lui-icon, .lui-rating-on > .lui-icon { color: var(--lui-warn); fill: var(--lui-warn); }
.lui-rating-star:has(~ .lui-rating-star > .lui-rating-input:checked) > .lui-icon { color: var(--lui-warn); fill: var(--lui-warn); }
.lui-rating-stars:has(.lui-rating-input:enabled):hover .lui-rating-star > .lui-icon { color: var(--lui-gray-7); fill: none; }
.lui-rating-stars:has(.lui-rating-input:enabled):hover .lui-rating-star:has(~ .lui-rating-star:hover) > .lui-icon,
.lui-rating-stars:has(.lui-rating-input:enabled):hover .lui-rating-star:hover > .lui-icon { color: var(--lui-warn); fill: var(--lui-warn); }
.lui-rating-star:has(> .lui-rating-input:focus-visible) { outline: 2px solid var(--lui-ring); outline-offset: -2px; border-radius: var(--lui-radius-sm); }
.lui-rating:disabled .lui-rating-star { cursor: not-allowed; opacity: 0.6; }
@media (pointer: coarse) { .lui-rating-star { width: var(--lui-hit); height: var(--lui-hit); } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_star_is_one_and_the_rest_are_many() {
        let ui = Ui::default();
        let m = ui.rating("r", "R").max(2).render().into_string();
        assert!(m.contains(">1 star<") && m.contains(">2 stars<"), "{m}");
        assert_eq!(m.matches(" required").count(), 0);
        let req = ui
            .rating("r", "R")
            .required()
            .help("h")
            .render()
            .into_string();
        assert_eq!(
            req.matches(" required").count(),
            1,
            "one radio covers the group"
        );
        crate::input::assert_described_ids_exist(&req);
    }
}
