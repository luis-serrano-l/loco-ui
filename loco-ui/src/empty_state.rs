//! # Empty state
//!
//! What a list, table or search shows when there is nothing in it: what is missing, why, and
//! what to do next, as real links and form buttons: as many of each as the page needs (the
//! first form button is the primary one, the others and the links secondary).
//!
//! **Platform features:** plain HTML; the action is an `<a>` or, for something that changes
//! data, a `<form method="post">` button, so it works with nothing else.
//!
//! **Accessibility:** a heading, a sentence and a real action; the illustration is
//! `aria-hidden`. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing; it is static content.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! assert!(ui.empty_state("No files yet").render().into_string().contains("No files yet"));
//! let m = ui.empty_state("No results for \u{201c}zzz\u{201d}")
//!     .icon("\u{1f50d}")
//!     .body(html! { "Check the spelling or clear the filter." })
//!     .link("Clear the filter", "/table")
//!     .action("Create a file", "/files/new");
//! let m = m.render().into_string();
//! assert!(m.contains(r#"href="/table""#) && m.contains(r#"action="/files/new""#));
//! // The same in `lui!`:
//! let same = lui! {
//!     EmptyState("No results for \u{201c}zzz\u{201d}") icon="\u{1f50d}"
//!         body=(html! { "Check the spelling or clear the filter." }) {
//!         link "Clear the filter" "/table";
//!         action "Create a file" "/files/new";
//!     }
//! };
//! assert_eq!(same.into_string(), m);
//! // Several actions: two forms, then two links.
//! let m = ui.empty_state("No projects").action("New project", "/p/new").action("Import", "/p/import")
//!     .link("Templates", "/templates").link("Docs", "/docs").render().into_string();
//! assert_eq!(m.matches("<form").count(), 2);
//! assert_eq!(m.matches("lui-button-primary").count(), 1);
//! let same = lui! { EmptyState("No projects") {
//!     action "New project" "/p/new"; action "Import" "/p/import"; link "Templates" "/templates"; link "Docs" "/docs";
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::icon::Glyph;
use crate::props::{Prop, PropKind};
use crate::{Caps, Ui};

/// What a list shows when there is nothing in it, made by [`Ui::empty_state`].
///
/// **Setters.** Values and items: `.icon(..)`, `.body(..)`, `.link(..)`, `.action(..)` (the
/// last two as often as needed).
#[derive(Clone, Debug, Default)]
pub struct EmptyState<'a> {
    title: &'a str,
    icon: Option<Glyph<'a>>,
    text: Option<Markup>,
    links: Vec<(&'a str, &'a str)>,
    posts: Vec<(&'a str, &'a str)>,
}

impl EmptyState<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("icon", PropKind::Value, "icon: impl Into<Glyph<'a>>")
            .doc("An icon, or a glyph or emoji, above the title, hidden from screen readers."),
        Prop::new("body", PropKind::Value, "text: Markup").doc("One or two sentences."),
        Prop::new("link", PropKind::Item, "label: &'a str, href: &'a str")
            .doc("A link to follow; add as many as needed."),
        Prop::new("action", PropKind::Item, "label: &'a str, action: &'a str")
            .doc("A button posting to `action`; the first is the primary one."),
    ];
}

impl Ui {
    /// An empty state saying `title`: what is missing.
    pub fn empty_state<'a>(&self, title: &'a str) -> EmptyState<'a> {
        EmptyState {
            title,
            ..EmptyState::default()
        }
    }
}

impl<'a> EmptyState<'a> {
    /// An icon, or a glyph or emoji, above the title, hidden from screen readers.
    pub fn icon(mut self, icon: impl Into<Glyph<'a>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// One or two sentences: why it is empty.
    pub fn body(mut self, text: Markup) -> Self {
        self.text = Some(text);
        self
    }

    /// The old name of [`Self::body`], kept for one release.
    #[deprecated(note = "use .body()")]
    pub fn text(self, text: Markup) -> Self {
        self.body(text)
    }

    /// A link to follow, after the form buttons; add as many as needed.
    pub fn link(mut self, label: &'a str, href: &'a str) -> Self {
        self.links.push((label, href));
        self
    }

    /// A button posting to `action`; the first one added is the primary action, the others
    /// secondary.
    pub fn action(mut self, label: &'a str, action: &'a str) -> Self {
        self.posts.push((label, action));
        self
    }

    /// The old name of [`Self::action`], kept for one release.
    #[deprecated(note = "use .action()")]
    pub fn post(self, label: &'a str, action: &'a str) -> Self {
        self.action(label, action)
    }
}

impl Render for EmptyState<'_> {
    fn render(&self) -> Markup {
        html! {
            div class="lui-empty" {
                @if let Some(i) = self.icon { span class="lui-empty-icon" aria-hidden="true" { (i) } }
                p class="lui-empty-title" { (self.title) }
                @if let Some(t) = &self.text { p class="lui-empty-text" { (t) } }
                @if !self.links.is_empty() || !self.posts.is_empty() {
                    div class="lui-empty-actions" {
                        @for (i, (label, action)) in self.posts.iter().enumerate() {
                            @let button = Button::new(Caps::NONE, label);
                            form method="post" action=(action) { @if i == 0 { (button.primary()) } @else { (button) } }
                        }
                        @for (label, href) in &self.links { (Button::link(Caps::NONE, label, href)) }
                    }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* The shadcn empty state: centred in a dashed frame, an icon in a soft brand tile, a title,
   one line of body and the actions as buttons. */
.lui-empty {
  display: grid; justify-items: center; gap: var(--lui-space); text-align: center;
  padding: calc(var(--lui-space) * 6) calc(var(--lui-space) * 3);
  border: 1px dashed var(--lui-line); border-radius: var(--lui-radius-lg);
}
.lui-empty-icon {
  display: grid; place-items: center; width: 3rem; height: 3rem; margin-bottom: var(--lui-space-2); font-size: 1.5rem; line-height: 1;
  border-radius: var(--lui-radius-lg); background: var(--lui-brand-3); color: var(--lui-brand-11);
}
.lui-empty-icon > .lui-icon { width: 1.5rem; height: 1.5rem; }
.lui-empty-title { margin: 0; font-size: 1.125rem; font-weight: 600; letter-spacing: -0.0125em; }
.lui-empty-text { margin: 0; color: var(--lui-muted); max-width: 24rem; font-size: 0.875rem; }
.lui-empty-actions { display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: calc(var(--lui-space) * 2); margin-top: var(--lui-space); }
.lui-empty-actions form { margin: 0; }
"#;
