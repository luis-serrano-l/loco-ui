//! # Empty state
//!
//! What a list, table or search shows when there is nothing in it: what is missing, why, and
//! the one thing to do next, as a real link or form button.
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
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::icon::Glyph;
use crate::props::{Key, Prop, PropKind};
use crate::{Caps, Ui};

/// What a list shows when there is nothing in it, made by [`Ui::empty_state`].
///
/// **Setters.** Values and items: `.icon(..)`, `.body(..)`, `.link(..)`, `.action(..)`.
#[derive(Clone, Debug, Default)]
pub struct EmptyState<'a> {
    title: &'a str,
    icon: Option<Glyph<'a>>,
    text: Option<Markup>,
    link: Option<(&'a str, &'a str)>,
    post: Option<(&'a str, &'a str)>,
}

impl EmptyState<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new("Enter / Space", "Sends the action button's form."),
        Key::new("Enter", "Follows the link button."),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("icon", PropKind::Value, "icon: impl Into<Glyph<'a>>")
            .doc("An icon, or a glyph or emoji, above the title, hidden from screen readers."),
        Prop::new("body", PropKind::Value, "text: Markup").doc("One or two sentences."),
        Prop::new("link", PropKind::Value, "label: &'a str, href: &'a str")
            .doc("A link to follow."),
        Prop::new("action", PropKind::Value, "label: &'a str, action: &'a str")
            .doc("A button posting to `action`."),
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

    /// A link to follow: the secondary action.
    pub fn link(mut self, label: &'a str, href: &'a str) -> Self {
        self.link = Some((label, href));
        self
    }

    /// A button posting to `action`: the primary action.
    pub fn action(mut self, label: &'a str, action: &'a str) -> Self {
        self.post = Some((label, action));
        self
    }
}

impl Render for EmptyState<'_> {
    fn render(&self) -> Markup {
        html! {
            div class="lui-empty" {
                @if let Some(i) = self.icon { span class="lui-empty-icon" aria-hidden="true" { (i) } }
                p class="lui-empty-title" { (self.title) }
                @if let Some(t) = &self.text { p class="lui-empty-text" { (t) } }
                @if self.link.is_some() || self.post.is_some() {
                    div class="lui-empty-actions" {
                        @if let Some((label, action)) = self.post {
                            form method="post" action=(action) { (Button::new(Caps::NONE, label).primary()) }
                        }
                        @if let Some((label, href)) = self.link { (Button::link(Caps::NONE, label, href)) }
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
  display: grid; place-items: center; width: 3rem; height: 3rem; margin-bottom: var(--lui-space-2); font-size: var(--lui-text-6); line-height: 1;
  border-radius: var(--lui-radius-lg); background: var(--lui-brand-3); color: var(--lui-brand-11);
}
.lui-empty-icon > .lui-icon { width: 1.5rem; height: 1.5rem; }
.lui-empty-title { margin: 0; font-size: var(--lui-text-4); font-weight: 600; letter-spacing: -0.0125em; }
.lui-empty-text { margin: 0; color: var(--lui-muted); max-width: 24rem; font-size: var(--lui-text-2); }
.lui-empty-actions { display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: calc(var(--lui-space) * 2); margin-top: var(--lui-space); }
.lui-empty-actions form { margin: 0; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn ui() -> Ui {
        Ui::default()
    }

    #[test]
    fn a_bare_empty_state_is_a_root_and_a_title() {
        let m = ui().empty_state("Nothing here").render().into_string();
        assert_eq!(
            m, r#"<div class="lui-empty"><p class="lui-empty-title">Nothing here</p></div>"#,
            "{m}"
        );
    }

    #[test]
    fn the_icon_is_hidden_from_screen_readers_and_the_body_is_a_paragraph() {
        let m = ui()
            .empty_state("Empty")
            .icon("?")
            .body(html! { "Because." })
            .render()
            .into_string();
        assert!(
            m.contains(r#"<span class="lui-empty-icon" aria-hidden="true">"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<p class="lui-empty-text">Because.</p>"#),
            "{m}"
        );
        assert!(!m.contains("lui-empty-actions"), "{m}");
    }

    #[test]
    fn the_action_is_a_post_form_and_the_link_is_an_anchor_in_the_actions_row() {
        let m = ui()
            .empty_state("Empty")
            .link("Back", "/back")
            .action("Create", "/new")
            .render()
            .into_string();
        assert!(m.contains(r#"<div class="lui-empty-actions">"#), "{m}");
        assert!(m.contains(r#"<form method="post" action="/new">"#), "{m}");
        assert!(m.contains(r#"href="/back""#), "{m}");
        assert!(
            m.find("<form").unwrap() < m.find(r#"href="/back""#).unwrap(),
            "{m}"
        );
    }
}
