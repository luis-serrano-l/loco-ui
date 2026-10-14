//! # Banner
//!
//! A full-width bar across the top of the page for news that concerns the whole site: a
//! release, planned downtime, a trial running out. One line of text, an optional link, and an
//! optional close button that keeps it closed on every page from then on. After Origin UI
//! Banner: a dark bar by default (the neutral tone), or tinted in a [`Tone`].
//!
//! **Platform features:** an `<aside>` named by its text, and a `<form method="post">` for
//! the close button. The form posts to [`DISMISS_PATH`], which sets a `cookie`
//! `lui-dismiss-<id>` for a year and redirects back (Post/Redirect/Get); the next page reads
//! the cookie through [`Ui::dismissed`] and renders nothing. The `axum` feature serves that
//! route in [`crate::enhance::router`] (Loco's initializer mounts it too); another server
//! answers it with [`dismiss_cookie`].
//!
//! **Accessibility:** a complementary landmark named by its text; the close button is named
//! "Dismiss: <text>". Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** close in place: closing is a round trip (with the
//! enhancement script too, since the answer is a redirect).
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/pricing", "", "");
//! let m = ui.banner("Version 2 is out.").render().into_string();
//! assert_eq!(m, r#"<aside class="lui-banner" aria-label="Version 2 is out."><p class="lui-banner-text">Version 2 is out.</p></aside>"#);
//! let m = ui.banner("Version 2 is out.").tone(Tone::Brand).link("Read the notes", "/changelog").dismiss().render().into_string();
//! assert!(m.contains(r#"action="/lui/dismiss""#) && m.contains(r#"name="id" value="version-2-is-out""#));
//! assert!(m.contains(r#"name="returns_to" value="/pricing""#) && m.contains(r#"href="/changelog""#));
//! // The same in `lui!`:
//! let same = lui! { Banner("Version 2 is out.") tone="brand" link=("Read the notes", "/changelog") dismiss; };
//! assert_eq!(same.into_string(), m);
//! // Once dismissed, the cookie hides it.
//! let ui = Ui::from_request("/pricing", "", "lui-dismiss-version-2-is-out=1");
//! assert_eq!(ui.banner("Version 2 is out.").dismiss().render().into_string(), "");
//! ```

use maud::{Markup, Render, html};

use crate::cookie::SetCookie;
use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::variant::Tone;
use crate::{Icon, Ui, slug};

/// Where a banner's or an alert's close form posts: fields `id` and `returns_to`.
pub const DISMISS_PATH: &str = "/lui/dismiss";

/// The prefix of the cookie that keeps a banner or an alert closed: `lui-dismiss-<id>`.
pub const DISMISS_COOKIE: &str = "lui-dismiss-";

/// The `Set-Cookie` value that keeps `id` dismissed for a year, for a server without the
/// `axum` feature answering [`DISMISS_PATH`] itself. `None` when `id` is not a slug (letters,
/// digits and `-` only), so a posted id cannot write another cookie.
pub fn dismiss_cookie(id: &str, secure: bool) -> Option<String> {
    let fine = !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
    fine.then(|| {
        let name = format!("{DISMISS_COOKIE}{id}");
        SetCookie::new(&name, "1", 60 * 60 * 24 * 365)
            .secure(secure)
            .to_string()
    })
}

/// The close form of a banner or an alert: the id and the page to come back to, and an icon
/// button named after `text`.
pub(crate) fn dismiss_form(ui: &Ui, id: &str, text: &str, class: &str) -> Markup {
    let back = ui.state.path();
    html! {
        form class=(class) method="post" action=(DISMISS_PATH) {
            input type="hidden" name="id" value=(id);
            @if !back.is_empty() { input type="hidden" name="returns_to" value=(back); }
            (ui.button("").ghost().small().icon_only().submit().aria_label(&ui.fill(Text::DismissMessage, &[&text])).body(html! { (Icon::X) }))
        }
    }
}

/// A site-wide bar, made by [`Ui::banner`].
///
/// **Setters.** Values and items: `.tone(..)`, `.link(..)`, `.id(..)`; switches: `.dismiss()`.
#[derive(Clone, Debug)]
pub struct Banner<'a> {
    ui: &'a Ui,
    text: &'a str,
    tone: Option<Tone>,
    link: Option<(&'a str, &'a str)>,
    id: Option<&'a str>,
    dismiss: bool,
}

impl Banner<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .default("neutral")
            .doc("The bar's colour: neutral is the dark bar, the others a tint."),
        Prop::new("link", PropKind::Value, "label: &'a str, href: &'a str")
            .doc("A link after the text, with an arrow."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .doc("The id its dismiss cookie is named after instead of the text's slug."),
        Prop::new("dismiss", PropKind::Switch, "")
            .doc("A close button that keeps it closed with a `lui-dismiss-<id>` cookie."),
    ];
}

impl Ui {
    /// A bar across the page saying `text`.
    pub fn banner<'a>(&'a self, text: &'a str) -> Banner<'a> {
        Banner {
            ui: self,
            text,
            tone: None,
            link: None,
            id: None,
            dismiss: false,
        }
    }
}

impl<'a> Banner<'a> {
    /// The bar's colour: neutral (the default) is the dark bar, the others a tint of the
    /// tone.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// A link after the text, with an arrow.
    pub fn link(mut self, label: &'a str, href: &'a str) -> Self {
        self.link = Some((label, href));
        self
    }

    /// The id its dismiss cookie is named after (`lui-dismiss-<id>`) instead of the text's
    /// slug: keep it when the text changes but the news does not.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// A close button that keeps it closed with a `lui-dismiss-<id>` cookie.
    pub fn dismiss(mut self) -> Self {
        self.dismiss = true;
        self
    }
}

impl Render for Banner<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let id = self.id.map_or_else(
            || slug(self.text).trim_matches('-').to_string(),
            str::to_string,
        );
        if self.dismiss && ui.dismissed(&id) {
            return Markup::default();
        }
        html! {
            aside class="lui-banner" data-tone=[self.tone.map(Tone::as_str)] aria-label=(self.text) {
                p class="lui-banner-text" {
                    (self.text)
                    @if let Some((label, href)) = self.link {
                        " " a class="lui-banner-link" href=(href) { (label) (Icon::ArrowRight) }
                    }
                }
                @if self.dismiss { (dismiss_form(ui, &id, self.text, "lui-banner-dismiss")) }
            }
        }
    }
}

#[cfg(feature = "axum")]
mod axum_glue {
    use axum::response::{IntoResponse, Response};

    use crate::{Redirect, Ui};

    /// The answer to a close form: sets `lui-dismiss-<id>` and goes back to `returns_to` (a
    /// local path only, else `/`).
    pub(crate) async fn dismiss(ui: Ui, body: String) -> Response {
        let field = |name: &str| {
            form_urlencoded::parse(body.as_bytes())
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.into_owned())
        };
        let back = field("returns_to").filter(|to| to.starts_with('/') && !to.starts_with("//"));
        let redirect: Redirect = ui.redirect(back.as_deref().unwrap_or("/"));
        match field("id").and_then(|id| super::dismiss_cookie(&id, ui.is_secure())) {
            Some(cookie) => redirect.cookie(cookie).into_response(),
            None => redirect.into_response(),
        }
    }
}

#[cfg(feature = "axum")]
pub(crate) use axum_glue::dismiss;

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Origin UI Banner: a full-width bar, 0.75rem by 1rem, text-sm, the link underlined
   with an arrow, the close × at the end. Neutral is the dark bar (--lui-gray-12 on
   --lui-gray-1 text); a tone tints it with the tone's colour. On a narrow container the text
   wraps; the close button stays on the first line. */
.lui-banner {
  --lui-banner-bg: var(--lui-gray-12); --lui-banner-fg: var(--lui-gray-1);
  container: lui-banner / inline-size;
  display: grid; grid-template-columns: minmax(0, 1fr) auto; align-items: start; gap: var(--lui-space-2);
  box-sizing: border-box; padding: var(--lui-space-3) var(--lui-space-4);
  font-size: var(--lui-text-2); line-height: var(--lui-leading-2); color: var(--lui-banner-fg); background: var(--lui-banner-bg);
}
.lui-banner[data-tone=brand], .lui-banner[data-tone=info] { --lui-banner-bg: var(--lui-brand-3); --lui-banner-fg: var(--lui-brand-11); }
.lui-banner[data-tone=ok] { --lui-banner-bg: color-mix(in srgb, var(--lui-ok) 14%, var(--lui-bg)); --lui-banner-fg: color-mix(in srgb, var(--lui-ok) 80%, var(--lui-fg)); }
.lui-banner[data-tone=warn] { --lui-banner-bg: color-mix(in srgb, var(--lui-warn) 16%, var(--lui-bg)); --lui-banner-fg: color-mix(in srgb, var(--lui-warn) 80%, var(--lui-fg)); }
.lui-banner[data-tone=danger] { --lui-banner-bg: color-mix(in srgb, var(--lui-danger) 12%, var(--lui-bg)); --lui-banner-fg: color-mix(in srgb, var(--lui-danger) 85%, var(--lui-fg)); }
.lui-banner-text { margin: 0; padding-block: 0.125rem; overflow-wrap: anywhere; }
.lui-banner-link { display: inline-flex; align-items: center; gap: var(--lui-space-1); color: inherit; font-weight: 500; text-decoration: underline; text-underline-offset: 2px; }
.lui-banner-link:hover { text-decoration-thickness: 2px; }
.lui-banner-link > .lui-icon { width: 1rem; height: 1rem; }
.lui-banner-dismiss { margin: -0.25rem -0.5rem -0.25rem 0; }
.lui-banner-dismiss .lui-button { color: inherit; }
.lui-banner-dismiss .lui-button:hover { background: color-mix(in srgb, currentColor 14%, transparent); }
/* From 30rem the text is centred, as Origin UI's top banner. */
@container lui-banner (width >= 30rem) { .lui-banner-text { text-align: center; } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dismiss_cookie_takes_slugs_only() {
        let c = dismiss_cookie("v2-is-out", false).unwrap();
        assert!(c.starts_with("lui-dismiss-v2-is-out=1; Path=/; Max-Age=31536000"));
        assert_eq!(dismiss_cookie("a; Path=/x", false), None);
        assert_eq!(dismiss_cookie("", false), None);
    }

    #[test]
    fn dismissed_banner_is_gone_and_others_stay() {
        let ui = Ui::from_request("/", "", "lui-dismiss-old=1");
        assert_eq!(ui.banner("Old").dismiss().render().into_string(), "");
        assert!(
            !ui.banner("Old").render().into_string().is_empty(),
            "no close button, never hidden"
        );
        assert!(!ui.banner("New").dismiss().render().into_string().is_empty());
    }
}
