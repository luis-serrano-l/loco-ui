//! # Error page
//!
//! The page for an HTTP error: the status, what happened in a sentence, and a way home, in
//! the site's look instead of the framework's plain text. `not_found` (feature `axum`) is an Axum handler
//! for `Router::fallback`, so every unknown path gets it; on Loco, add it in `after_routes`.
//! It has its own words for 401 (sign in), 403 (no access), 404 (not here), 422 (the form
//! could not be accepted) and 503 (down for a moment); any other status says something went
//! wrong.
//!
//! **Platform features:** none of its own; the HTTP status is set on the response.
//!
//! **Accessibility:** the title is the page's `<h1>`; the status number is decorative and
//! hidden from assistive tech (the title says it). Checked by axe-core with the demo routes.
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.error_page(404).render().into_string();
//! assert!(m.contains("<h1>Page not found</h1>") && m.contains(r#"href="/""#));
//! let m = ui.error_page(500).home("/dashboard").render().into_string();
//! assert!(m.contains("Something went wrong") && m.contains(r#"href="/dashboard""#));
//! // The same in `lui!`:
//! let same = lui! { ErrorPage(500) home="/dashboard"; };
//! assert_eq!(same.into_string(), m);
//! for (status, title) in [(401, "Sign in to continue"), (403, "You cannot open this page"),
//!     (422, "That could not be saved"), (503, "Back in a moment")] {
//!     assert!(ui.error_page(status).render().into_string().contains(&format!("<h1>{title}</h1>")));
//! }
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::{Page, Ui};

/// An error page for an HTTP status, made by [`Ui::error_page`].
///
/// **Setters.** Values and items: `.title(..)`, `.description(..)`, `.home(..)`.
#[derive(Clone, Debug)]
pub struct ErrorPage<'a> {
    ui: &'a Ui,
    status: u16,
    title: &'a str,
    message: &'a str,
    home: &'a str,
}

impl ErrorPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[Key::new("Enter", "Follows the Go home link.")];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("title", PropKind::Value, "text: &'a str")
            .default("the status's own heading")
            .doc("The heading."),
        Prop::new("description", PropKind::Value, "text: &'a str").doc("The sentence under it."),
        Prop::new("home", PropKind::Value, "href: &'a str")
            .default("/")
            .attr("href")
            .doc("Where the button goes."),
    ];
}

impl Ui {
    /// The page for HTTP `status`: 401, 403, 404, 422 and 503 say what happened in their own
    /// words, anything else that something went wrong.
    pub fn error_page(&self, status: u16) -> ErrorPage<'_> {
        let (title, message) = match status {
            401 => (Text::Unauthorized, Text::UnauthorizedMessage),
            403 => (Text::Forbidden, Text::ForbiddenMessage),
            404 => (Text::NotFound, Text::NotFoundMessage),
            422 => (Text::Unprocessable, Text::UnprocessableMessage),
            503 => (Text::Unavailable, Text::UnavailableMessage),
            _ => (Text::ServerError, Text::ServerErrorMessage),
        };
        ErrorPage {
            ui: self,
            status,
            title: self.text(title),
            message: self.text(message),
            home: "/",
        }
    }
}

impl<'a> ErrorPage<'a> {
    /// The heading.
    pub fn title(mut self, text: &'a str) -> Self {
        self.title = text;
        self
    }

    /// The sentence under it.
    pub fn description(mut self, text: &'a str) -> Self {
        self.message = text;
        self
    }

    /// Where the button goes.
    pub fn home(mut self, href: &'a str) -> Self {
        self.home = href;
        self
    }

    /// The whole page, titled by the heading.
    pub fn page(&self) -> Page {
        self.ui.page(self.title, self.render())
    }
}

impl Render for ErrorPage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        html! {
            div class="lui-error-page" {
                p class="lui-error-page-status" aria-hidden="true" { (self.status) }
                h1 { (self.title) }
                p { (self.message) }
                (ui.link_button(ui.text(Text::GoHome), self.home).primary())
            }
        }
    }
}

#[cfg(feature = "axum")]
mod axum_glue {
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};

    use super::ErrorPage;
    use crate::Ui;

    /// The page with its status: `ui.error_page(404).into_response()` answers 404.
    impl IntoResponse for ErrorPage<'_> {
        fn into_response(self) -> Response {
            let status =
                StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            (status, self.page()).into_response()
        }
    }

    /// The 404 page for `Router::fallback(loco_ui::blocks::not_found)`: every path no route
    /// answers gets it, in the visitor's theme and language.
    pub async fn not_found(ui: Ui) -> Response {
        ui.error_page(404).into_response()
    }
}

#[cfg(feature = "axum")]
pub use axum_glue::not_found;

/// Styles for this block; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-error-page { display: grid; justify-items: center; gap: var(--lui-space); padding-block: calc(var(--lui-space) * 8); text-align: center; }
.lui-error-page h1 { margin: 0; }
.lui-error-page p { margin: 0; color: var(--lui-muted); max-width: 32rem; }
.lui-error-page .lui-error-page-status { font-size: var(--lui-text-9); line-height: 1; font-weight: 700; color: var(--lui-fg); letter-spacing: -0.02em; }
.lui-error-page .lui-button { margin-top: calc(var(--lui-space) * 2); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_page_shows_a_hidden_status_a_title_a_sentence_and_a_home_button() {
        let ui = Ui::default();
        let m = ui.error_page(404).render().into_string();
        assert!(m.starts_with(r#"<div class="lui-error-page">"#), "{m}");
        assert!(
            m.contains(r#"<p class="lui-error-page-status" aria-hidden="true">404</p>"#),
            "{m}"
        );
        assert_eq!(m.matches("<h1>").count(), 1, "{m}");
        assert!(m.contains(r#"href="/""#) && m.contains("lui-button"), "{m}");
    }

    #[test]
    fn each_known_status_has_its_own_title_and_others_share_one() {
        let ui = Ui::default();
        let title = |status| {
            let m = ui.error_page(status).render().into_string();
            let start = m.find("<h1>").unwrap() + 4;
            m[start..m.find("</h1>").unwrap()].to_string()
        };
        let known = [401, 403, 404, 422, 503].map(title);
        for (i, a) in known.iter().enumerate() {
            for b in &known[i + 1..] {
                assert_ne!(a, b);
            }
        }
        assert_eq!(title(500), title(418));
        assert!(!known.contains(&title(500)));
    }

    #[test]
    fn title_description_and_home_override_the_defaults() {
        let ui = Ui::default();
        let m = ui
            .error_page(404)
            .title("Gone")
            .description("It moved.")
            .home("/dash")
            .render()
            .into_string();
        assert!(
            m.contains("<h1>Gone</h1>") && m.contains("<p>It moved.</p>"),
            "{m}"
        );
        assert!(
            m.contains(r#"href="/dash""#) && !m.contains(r#"href="/""#),
            "{m}"
        );
    }

    #[test]
    fn page_uses_the_title_as_the_document_title() {
        let ui = Ui::default();
        let html = ui.error_page(404).title("Gone").page().into_string();
        assert!(
            html.contains("<title>Gone</title>") && html.contains("lui-error-page"),
            "{html}"
        );
    }
}
