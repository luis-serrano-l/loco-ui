//! # Auth page
//!
//! A sign-in, sign-up or reset page: a narrow card in the middle of the page with a title, a
//! line under it, the form, and the links to the other account pages below. `cargo lui auth`
//! writes plain views; this is the same page ready-made.
//!
//! **Platform features:** none of its own: a card and a form that posts.
//!
//! **Accessibility:** the title is the page's `<h1>`; the form brings its labels, error
//! summary and focus handling. Checked by axe-core with the demo routes.
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let form = ui.form("/signin").email("email", "Email").required().submit("Sign in");
//! let m = ui.auth_page("Sign in").description("Welcome back.").body(form.render())
//!     .footer(html! { a href="/signup" { "Create an account" } }).render().into_string();
//! assert!(m.contains("<h1 class=\"lui-auth-title\">Sign in</h1>") && m.contains("Create an account"));
//! // The same in `lui!`:
//! let same = lui! { AuthPage("Sign in") description="Welcome back." body=(form.render())
//!     footer=(html! { a href="/signup" { "Create an account" } }); };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A centred account form, made by [`Ui::auth_page`].
///
/// **Setters.** Values and items: `.description(..)`, `.body(..)`, `.footer(..)`.
#[derive(Clone, Debug)]
pub struct AuthPage<'a> {
    title: &'a str,
    description: Option<&'a str>,
    body: Markup,
    footer: Option<Markup>,
}

impl AuthPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("description", PropKind::Value, "text: &'a str").doc("A line under the title."),
        Prop::new("body", PropKind::Value, "body: Markup").doc("The form."),
        Prop::new("footer", PropKind::Value, "footer: Markup")
            .doc("Under the card: links to the other account pages."),
    ];
}

impl Ui {
    /// An account page titled `title`.
    pub fn auth_page<'a>(&self, title: &'a str) -> AuthPage<'a> {
        AuthPage {
            title,
            description: None,
            body: Markup::default(),
            footer: None,
        }
    }
}

impl<'a> AuthPage<'a> {
    /// A line under the title.
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// The form.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }

    /// Under the card: links to the other account pages.
    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }
}

impl Render for AuthPage<'_> {
    fn render(&self) -> Markup {
        html! {
            div class="lui-auth" {
                div class="lui-auth-card" {
                    h1 class="lui-auth-title" { (self.title) }
                    @if let Some(d) = self.description { p class="lui-auth-description" { (d) } }
                    (self.body)
                }
                @if let Some(f) = &self.footer { div class="lui-auth-footer" { (f) } }
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. shadcn's login block: a 24rem
/// card, centred.
pub const CSS: &str = r#"
.lui-auth { display: grid; justify-items: center; gap: calc(var(--lui-space) * 2); padding-block: calc(var(--lui-space) * 6); }
.lui-auth-card {
  box-sizing: border-box; width: min(24rem, 100%); display: grid; gap: calc(var(--lui-space) * 2);
  padding: calc(var(--lui-space) * 3); background: var(--lui-card); border: 1px solid var(--lui-line);
  border-radius: var(--lui-radius-lg); box-shadow: var(--lui-shadow-xs);
}
.lui-auth-title { margin: 0; font-size: var(--lui-text-6); line-height: 2rem; font-weight: 600; }
.lui-auth-description { margin: -0.5rem 0 0; color: var(--lui-muted); font-size: var(--lui-text-2); }
.lui-auth-card .lui-form { max-width: none; }
.lui-auth-footer { font-size: var(--lui-text-2); color: var(--lui-muted); text-align: center; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_title_is_the_h1_in_a_card_and_nothing_optional_is_rendered_by_default() {
        let ui = Ui::default();
        let m = ui.auth_page("Sign in").render().into_string();
        assert!(
            m.starts_with(r#"<div class="lui-auth"><div class="lui-auth-card">"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<h1 class="lui-auth-title">Sign in</h1>"#),
            "{m}"
        );
        assert!(
            !m.contains("lui-auth-description") && !m.contains("lui-auth-footer"),
            "{m}"
        );
    }

    #[test]
    fn description_and_body_go_inside_the_card_in_order() {
        let ui = Ui::default();
        let m = ui
            .auth_page("T")
            .description("Welcome back.")
            .body(html! { form { "F" } })
            .render()
            .into_string();
        let card_end = m.find("</form>").unwrap();
        let (h1, d, f) = (
            m.find("<h1").unwrap(),
            m.find("Welcome back.").unwrap(),
            m.find("<form>").unwrap(),
        );
        assert!(h1 < d && d < f && f < card_end, "{m}");
        assert!(
            m.contains(r#"<p class="lui-auth-description">Welcome back.</p>"#),
            "{m}"
        );
    }

    #[test]
    fn the_footer_sits_after_the_card() {
        let ui = Ui::default();
        let m = ui
            .auth_page("T")
            .body(html! { i { "body" } })
            .footer(html! { a href="/signup" { "Create" } })
            .render()
            .into_string();
        assert!(
            m.ends_with(r#"<div class="lui-auth-footer"><a href="/signup">Create</a></div></div>"#),
            "{m}"
        );
        assert!(
            m.find("body").unwrap() < m.find("lui-auth-footer").unwrap(),
            "{m}"
        );
    }
}
