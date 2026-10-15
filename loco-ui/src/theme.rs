//! # Theme
//!
//! Light/dark theme chosen by the user and remembered by the server.
//!
//! **Platform features:** `prefers-color-scheme` (baseline 2020), `color-scheme` property,
//! CSS custom properties. The toggle is a `<form method="post">`; the server stores the choice
//! in a cookie and sets `data-theme` on `<html>`.
//!
//! **Accessibility:** three native buttons with `aria-pressed` on the current theme. Checked by
//! axe-core in headless Firefox on every demo route, both capability variants, light and dark
//! (no serious or critical violation).
//!
//! **What it does not do without script:** follow a change of the OS preference while a cookie
//! choice is set; the cookie wins until reset to auto.
//!
//! **Fallback:** none needed. Without a cookie the OS preference wins. Colours are switched by
//! a media query and `data-theme`, so `light-dark()` support (`Caps::LightDark`) is only
//! reported, never required. The form is a swap root: the [`crate::enhance`] script applies
//! the new `data-theme` without a reload.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/", "", "theme=dark");
//! let m = ui.theme_toggle("/theme").render().into_string();
//! assert!(m.contains(r#"value="dark" aria-pressed="true""#));
//!
//! // The handler the toggle posts to keeps the choice for a year.
//! let r = ui.redirect("/").theme(Theme::parse("light"));
//! assert!(r.set_cookies()[0].starts_with("theme=light;"));
//!
//! // The same in `lui!`:
//! let same = lui! { ThemeToggle("/theme"); };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::props::Key;
use crate::{Caps, Redirect, Ui, cookie::SetCookie};

/// Name of the cookie that remembers the chosen theme.
pub const THEME_COOKIE: &str = "theme";

/// The theme the page should render with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    /// Follow the operating system preference.
    #[default]
    Auto,
    /// Always light.
    Light,
    /// Always dark.
    Dark,
}

impl Theme {
    /// Value used in the `data-theme` attribute and the cookie.
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Auto => "auto",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    /// Parse a cookie or form value. Unknown values become `Auto`.
    pub fn parse(value: &str) -> Theme {
        match value {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::Auto,
        }
    }
}

/// A three-button form posting the chosen theme, made by [`Ui::theme_toggle`].
#[derive(Clone, Debug)]
pub struct ThemeToggle<'a> {
    action: &'a str,
    current: Theme,
}

impl ThemeToggle<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves between the Auto, Light and Dark buttons.",
        ),
        Key::new("Enter / Space", "Switches to the focused theme."),
    ];
}

impl Ui {
    /// Auto, light and dark, posted to `action`; the request's theme is pressed.
    pub fn theme_toggle<'a>(&self, action: &'a str) -> ThemeToggle<'a> {
        ThemeToggle {
            action,
            current: self.theme,
        }
    }
}

impl Redirect {
    /// Remember `theme` for this visitor: what the handler behind [`Ui::theme_toggle`] sends.
    pub fn theme(self, theme: Theme) -> Self {
        let cookie = self.set_cookie(SetCookie::new(THEME_COOKIE, theme.as_str(), 31_536_000));
        let cookie = cookie.to_string();
        self.cookie(cookie)
    }
}

impl Render for ThemeToggle<'_> {
    fn render(&self) -> Markup {
        html! {
            form id="lui-theme" data-lui="swap" class="lui-theme" method="post" action=(self.action) {
                @for choice in [Theme::Auto, Theme::Light, Theme::Dark] {
                    (Button::new(Caps::NONE, choice.as_str()).ghost().small().name("theme").value(choice.as_str()).pressed(choice == self.current))
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* shadcn ToggleGroup, outline variant: one bordered strip, the pressed item on the accent. */
.lui-theme { display: inline-flex; gap: 0; border: 1px solid var(--lui-input); border-radius: var(--lui-radius-sm); overflow: hidden; box-shadow: var(--lui-shadow-xs); }
.lui-theme .lui-button { border-radius: 0; text-transform: capitalize; }
.lui-theme .lui-button + .lui-button { border-inline-start: 1px solid var(--lui-input); }
.lui-theme .lui-button[aria-pressed="true"] { background: var(--lui-accent); color: var(--lui-on-accent); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_toggle_is_a_swap_form_with_three_buttons_posting_to_the_action() {
        let ui = Ui::default();
        let m = ui.theme_toggle("/theme").render().into_string();
        assert!(
            m.contains(r#"class="lui-theme""#) && m.contains(r#"data-lui="swap""#),
            "{m}"
        );
        assert!(
            m.contains(r#"method="post""#) && m.contains(r#"action="/theme""#),
            "{m}"
        );
        for value in ["auto", "light", "dark"] {
            assert!(
                m.contains(&format!(r#"name="theme" value="{value}""#)),
                "{m}"
            );
        }
        assert_eq!(m.matches("<button").count(), 3, "{m}");
    }

    #[test]
    fn only_the_requests_theme_is_pressed() {
        let ui = Ui::from_request("/", "", "theme=light");
        let m = ui.theme_toggle("/theme").render().into_string();
        assert!(m.contains(r#"value="light" aria-pressed="true""#), "{m}");
        assert_eq!(m.matches(r#"aria-pressed="true""#).count(), 1, "{m}");
        let auto = Ui::default().theme_toggle("/t").render().into_string();
        assert!(
            auto.contains(r#"value="auto" aria-pressed="true""#),
            "{auto}"
        );
    }

    #[test]
    fn parse_round_trips_and_unknown_values_are_auto() {
        for theme in [Theme::Auto, Theme::Light, Theme::Dark] {
            assert_eq!(Theme::parse(theme.as_str()), theme);
        }
        assert_eq!(Theme::parse("purple"), Theme::Auto);
        assert_eq!(Theme::parse(""), Theme::Auto);
    }

    #[test]
    fn the_redirect_sets_the_theme_cookie() {
        let ui = Ui::default();
        let cookies = ui.redirect("/").theme(Theme::Dark).set_cookies();
        assert!(cookies[0].starts_with("theme=dark;"), "{cookies:?}");
    }
}
