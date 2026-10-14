//! # App shell
//!
//! The frame of a signed-in app: its name and navigation in a sidebar (a drawer on narrow
//! screens), in groups with icons and counts as a [`crate::sidebar`], who is signed in with a sign-out button at the foot, and the page's content
//! beside it. The link to the current path is marked `aria-current="page"`.
//!
//! **Platform features:** those of [`crate::drawer`] (a modal `<dialog>` on narrow screens, a
//! plain sidebar when wide); the sign-out is a form that posts.
//!
//! **Accessibility:** the navigation is a `<nav>` inside the drawer, named by the app's name;
//! the current page's link has `aria-current="page"`; sign-out is a real button. Checked by
//! axe-core with the demo routes.
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** that of the drawer: without invoker commands the menu opens through
//! `?dialog=`.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/", "", "");
//! let m = ui.app_shell("Acme").link("Dashboard", "/").user("Ada Lovelace", "/signout")
//!     .body(html! { h1 { "Hello" } }).render().into_string();
//! assert!(m.contains("Ada Lovelace") && m.contains(r#"action="/signout""#));
//!
//! // Groups under small headings, an icon and a count on a link, as in [`crate::sidebar`]:
//! let m = ui.app_shell("Acme")
//!     .link("Inbox", "/").icon(Icon::Mail).badge(3)
//!     .group("Projects").link("Apollo", "/p/apollo").badge(12)
//!     .user("Ada Lovelace", "/signout").body(html! { h1 { "Hello" } });
//! let html = m.render().into_string();
//! assert!(html.contains(">Projects</p>") && html.contains("lui-sidebar-badge\">12<"));
//! // The same in `lui!`:
//! let same = lui! { AppShell("Acme") user=("Ada Lovelace", "/signout") {
//!     link "Inbox" "/" icon=(Icon::Mail) badge=3;
//!     group "Projects";
//!     link "Apollo" "/p/apollo" badge=12;
//!     body (html! { h1 { "Hello" } });
//! } };
//! assert_eq!(same.into_string(), html);
//! // A banner across the top, above the sidebar and the content.
//! let m = ui.app_shell("Acme").banner(ui.banner("Maintenance at 22:00.").render()).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-app-shell"><aside class="lui-banner""#));
//! assert_eq!(lui! { AppShell("Acme") banner=(ui.banner("Maintenance at 22:00.").render()); }.into_string(), m);
//! // On `/p/apollo/settings` the link to `/p/apollo` is current.
//! let ui = Ui::from_request("/p/apollo/settings", "", "");
//! let m = ui.app_shell("Acme").current_prefix().link("Apollo", "/p/apollo").render().into_string();
//! assert!(m.contains(r#"<a href="/p/apollo" aria-current="page">"#));
//! ```

use std::fmt::Display;

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::icon::Glyph;
use crate::props::{Prop, PropKind};
use crate::sidebar::Sidebar;

/// A sidebar and content, made by [`Ui::app_shell`].
///
/// **Setters.** Values and items: `.group(..)`, `.link(..)`, `.icon(..)`, `.badge(..)`,
/// `.user(..)`, `.body(..)`, `.banner(..)`.
/// `.user(..)`, `.body(..)`; switches: `.current_prefix()`.
#[derive(Clone, Debug)]
pub struct AppShell<'a> {
    ui: &'a Ui,
    name: &'a str,
    nav: Sidebar<'a>,
    user: Option<(&'a str, &'a str)>,
    body: Markup,
    banner: Option<Markup>,
}

impl AppShell<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("group", PropKind::Item, "heading: &'a str")
            .doc("Start a group of links under a small heading."),
        Prop::new("link", PropKind::Item, "label: &'a str, href: &'a str")
            .doc("A link in the sidebar; the one to the current path is marked current."),
        Prop::new("icon", PropKind::Modifier, "icon: impl Into<Glyph<'a>>")
            .applies_to(&["link"])
            .doc("An icon before the link added last."),
        Prop::new("badge", PropKind::Modifier, "text: impl Display")
            .applies_to(&["link"])
            .doc("A count after the link added last."),
        Prop::new("user", PropKind::Value, "name: &'a str, signout: &'a str")
            .doc("Who is signed in, and where the sign-out button posts."),
        Prop::new("body", PropKind::Value, "body: Markup").doc("The page's content."),
        Prop::new("banner", PropKind::Value, "banner: Markup")
            .doc("A bar across the top, above the sidebar and the content (a `ui.banner(..)`)."),
        Prop::new("current_prefix", PropKind::Switch, "").doc(
            "Mark a link current also when the path is under it (`/projects` on `/projects/42`).",
        ),
    ];
}

impl Ui {
    /// The frame of an app called `name`.
    pub fn app_shell<'a>(&'a self, name: &'a str) -> AppShell<'a> {
        AppShell {
            ui: self,
            name,
            nav: self.sidebar(name).id("app-nav").inner(),
            user: None,
            body: Markup::default(),
            banner: None,
        }
    }
}

impl<'a> AppShell<'a> {
    /// Start a group of links under a small heading.
    pub fn group(mut self, heading: &'a str) -> Self {
        self.nav = self.nav.group(heading);
        self
    }

    /// A link in the sidebar; the one to the current path is marked current.
    pub fn link(mut self, label: &'a str, href: &'a str) -> Self {
        self.nav = self.nav.link(label, href);
        self
    }

    /// An icon before the link added last.
    #[track_caller]
    pub fn icon(mut self, icon: impl Into<Glyph<'a>>) -> Self {
        crate::props::check_last::<Self>(AppShell::PROPS, "icon", self.nav.last_item());
        self.nav = self.nav.icon(icon);
        self
    }

    /// A count after the link added last.
    #[track_caller]
    pub fn badge(mut self, text: impl Display) -> Self {
        crate::props::check_last::<Self>(AppShell::PROPS, "badge", self.nav.last_item());
        self.nav = self.nav.badge(text);
        self
    }

    /// Who is signed in, and where the sign-out button posts.
    pub fn user(mut self, name: &'a str, signout: &'a str) -> Self {
        self.user = Some((name, signout));
        self
    }

    /// The page's content.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }

    /// A bar across the top, above the sidebar and the content: a [`crate::banner::Banner`]
    /// rendered (`ui.banner("..").dismiss().render()`).
    pub fn banner(mut self, banner: Markup) -> Self {
        self.banner = Some(banner);
        self
    }

    /// Mark a link current also when the path is under it: `/projects` on `/projects/42`
    /// (see [`Sidebar::current_prefix`]).
    pub fn current_prefix(mut self) -> Self {
        self.nav = self.nav.current_prefix();
        self
    }
}

impl Render for AppShell<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let nav = html! {
            div class="lui-app-shell-links" { (self.nav) }
            @if let Some((name, signout)) = self.user {
                form method="post" action=(signout) class="lui-app-shell-user" {
                    (ui.avatar(name).small())
                    span { (name) }
                    (ui.button(ui.text(Text::SignOut)).ghost().small().submit())
                }
            }
        };
        html! {
            div class="lui-app-shell" {
                @if let Some(b) = &self.banner { (b) }
                (ui.drawer(self.name).id("app").title(self.name).sidebar().nav(nav).body(self.body.clone()))
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* The links are a sidebar (its groups, icons and counts); the user sits at the foot. */
.lui-app-shell-user { display: grid; grid-template-columns: auto 1fr; align-items: center; gap: 0.5rem; margin-top: calc(var(--lui-space) * 3); padding-top: calc(var(--lui-space) * 2); border-top: 1px solid var(--lui-line); font-size: 0.875rem; }
.lui-app-shell-user > span { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.lui-app-shell-user > .lui-button { grid-column: 1 / -1; justify-self: start; }
"#;
