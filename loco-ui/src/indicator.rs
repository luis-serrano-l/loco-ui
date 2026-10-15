//! # Indicator
//!
//! A count or a dot pinned to the corner of what it wraps: unread messages on an inbox
//! button, a status dot on an avatar, new items on a tab. After Origin UI's notification
//! badges: a solid pill 1.25rem tall in the danger colour (or any tone) on the top end corner,
//! ringed in the page colour so it reads against the thing under it; "99+" past a maximum.
//!
//! **Platform features:** `position: absolute` with `inset-inline-end` (Chrome 87, Firefox 63,
//! Safari 14.1), so the badge moves to the left corner in a right-to-left page;
//! `translate` (Chrome 104, Firefox 72, Safari 14.1) centres it on the corner.
//!
//! **Accessibility:** the visible number is `aria-hidden` and a visually hidden text says the
//! count in words ("3 new", in the visitor's language, or `.label(..)`), so a screen reader
//! reads "Inbox, 3 new" on a wrapped button. A count of 0 shows nothing. Checked by axe-core
//! in headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing; the count is the server's, as
//! of this page (a swap root around it updates it in place with the script).
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.indicator(3).body(html! { (ui.button("Inbox")) }).render().into_string();
//! assert!(m.starts_with(r#"<span class="lui-indicator"><button type="submit" class="lui-button">Inbox</button>"#));
//! assert!(m.contains(r#"<span class="lui-indicator-badge" aria-hidden="true">"#) && m.contains(r#"<span class="lui-sr">3 new</span>"#));
//! // The same in `lui!`, where the block is what it wraps:
//! let same = lui! { Indicator(3) { Button("Inbox"); } };
//! assert_eq!(same.into_string(), m);
//! // A brand dot, and a capped count.
//! let dot = ui.indicator(1).dot().tone(Tone::Brand).body(html! { (ui.avatar("Ada Lovelace")) });
//! let dot = dot.render().into_string();
//! assert!(dot.contains(r#"<span class="lui-indicator-dot" data-tone="brand" aria-hidden="true"></span>"#));
//! assert_eq!(lui! { Indicator(1) dot tone="brand" { Avatar("Ada Lovelace"); } }.into_string(), dot);
//! let many = ui.indicator(120).max(99).body(html! { "Mail" }).render().into_string();
//! assert!(many.contains(">99+</span>") && many.contains("120 new"));
//! assert_eq!(lui! { Indicator(120) max=99 { "Mail" } }.into_string(), many);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::variant::{Radius, Tone, Variant};

/// A count or a dot on the corner of what it wraps, made by [`Ui::indicator`].
///
/// **Setters.** Values: `.body(..)`, `.max(..)`, `.tone(..)`, `.label(..)`; switches:
/// `.dot()`.
#[derive(Clone, Debug)]
pub struct Indicator<'a> {
    ui: &'a Ui,
    count: u64,
    content: Markup,
    max: Option<u64>,
    tone: Tone,
    label: Option<&'a str>,
    dot: bool,
}

impl Indicator<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup")
            .doc("What the badge sits on: a button, an avatar, a link."),
        Prop::new("max", PropKind::Number, "max: u64")
            .doc("Above this count, show `max+` (`99+`)."),
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .default("danger")
            .doc("The badge's colour."),
        Prop::new("label", PropKind::Value, "label: &'a str")
            .default("{n} new")
            .doc("What a screen reader says instead of the count in words."),
        Prop::new("dot", PropKind::Switch, "").doc("A dot instead of the number."),
    ];
}

impl Ui {
    /// A badge reading `count` on the top end corner of its [`Indicator::body`]; nothing
    /// shows while `count` is 0.
    pub fn indicator(&self, count: u64) -> Indicator<'_> {
        Indicator {
            ui: self,
            count,
            content: Markup::default(),
            max: None,
            tone: Tone::Danger,
            label: None,
            dot: false,
        }
    }
}

impl<'a> Indicator<'a> {
    /// What the badge sits on: a button, an avatar, a link.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// Above this count, show `max+` (`99+`); the words for screen readers keep the count.
    pub fn max(mut self, max: u64) -> Self {
        self.max = Some(max);
        self
    }

    /// The badge's colour: danger (the default), brand, ok, warn, info or neutral.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    /// What a screen reader says instead of "{n} new" ("2 failed checks").
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// A dot instead of the number: something is new, the count does not matter.
    pub fn dot(mut self) -> Self {
        self.dot = true;
        self
    }
}

impl Render for Indicator<'_> {
    fn render(&self) -> Markup {
        let shown = match self.max {
            Some(max) if self.count > max => format!("{}+", self.ui.format_number(max as f64)),
            _ => self.ui.format_number(self.count as f64),
        };
        let words = match self.label {
            Some(label) => label.to_owned(),
            None => self.ui.count(Text::IndicatorNew, self.count),
        };
        html! {
            span class="lui-indicator" {
                (self.content)
                @if self.count > 0 {
                    @if self.dot {
                        span class="lui-indicator-dot" data-tone=(self.tone.as_str()) aria-hidden="true" {}
                    } @else {
                        span class="lui-indicator-badge" aria-hidden="true" {
                            (self.ui.badge(&shown).tone(self.tone).variant(Variant::Solid).radius(Radius::Full))
                        }
                    }
                    span class="lui-sr" { (words) }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Origin UI's notification badges: a solid pill min 1.25rem across, 0.375rem padding,
   --lui-text-1, centred on the top end corner by translate(50%, -50%), a 2px ring of the page
   colour; the dot is 0.625rem. */
.lui-indicator { position: relative; display: inline-flex; vertical-align: middle; max-width: 100%; }
.lui-indicator-badge, .lui-indicator-dot { position: absolute; top: 0; inset-inline-end: 0; translate: 50% -50%; pointer-events: none; }
[dir=rtl] .lui-indicator-badge, [dir=rtl] .lui-indicator-dot { translate: -50% -50%; }
.lui-indicator-badge > .lui-badge {
  min-width: 1.25rem; height: 1.25rem; padding: 0 0.375rem; justify-content: center;
  font-size: var(--lui-text-1); line-height: 1; font-variant-numeric: tabular-nums; box-shadow: 0 0 0 2px var(--lui-bg);
}
.lui-indicator-dot { --lui-indicator-tone: var(--lui-danger); width: 0.625rem; height: 0.625rem; border-radius: 9999px; background: var(--lui-indicator-tone); box-shadow: 0 0 0 2px var(--lui-bg); translate: 25% -25%; }
.lui-indicator-dot:is([data-tone=brand], [data-tone=info]) { --lui-indicator-tone: var(--lui-primary); }
.lui-indicator-dot[data-tone=ok] { --lui-indicator-tone: var(--lui-ok); }
.lui-indicator-dot[data-tone=warn] { --lui-indicator-tone: var(--lui-warn); }
.lui-indicator-dot[data-tone=neutral] { --lui-indicator-tone: var(--lui-gray-9); }
[dir=rtl] .lui-indicator-dot { translate: -25% -25%; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_content_comes_first_then_a_hidden_badge_and_a_spoken_count() {
        let ui = Ui::default();
        let m = ui
            .indicator(3)
            .body(html! { "Inbox" })
            .render()
            .into_string();
        assert!(m.starts_with(r#"<span class="lui-indicator">Inbox<span class="lui-indicator-badge" aria-hidden="true">"#), "{m}");
        assert!(m.contains(r#"<span class="lui-sr">3 new</span>"#), "{m}");
    }

    #[test]
    fn a_count_of_zero_shows_nothing() {
        let m = Ui::default()
            .indicator(0)
            .body(html! { "Inbox" })
            .render()
            .into_string();
        assert_eq!(m, r#"<span class="lui-indicator">Inbox</span>"#, "{m}");
    }

    #[test]
    fn max_caps_the_shown_number_but_not_the_spoken_one() {
        let ui = Ui::default();
        let m = ui.indicator(120).max(99).render().into_string();
        assert!(m.contains("99+") && !m.contains(">120<"), "{m}");
        assert!(m.contains("120 new"), "{m}");
        let m = ui.indicator(99).max(99).render().into_string();
        assert!(!m.contains("99+"), "{m}");
    }

    #[test]
    fn dot_tone_and_label_change_the_badge_and_the_words() {
        let m = Ui::default()
            .indicator(5)
            .dot()
            .tone(Tone::Ok)
            .label("Online")
            .render()
            .into_string();
        assert!(
            m.contains(
                r#"<span class="lui-indicator-dot" data-tone="ok" aria-hidden="true"></span>"#
            ),
            "{m}"
        );
        assert!(m.contains(r#"<span class="lui-sr">Online</span>"#), "{m}");
        assert!(
            !m.contains("lui-indicator-badge") && !m.contains("5 new"),
            "{m}"
        );
    }
}
