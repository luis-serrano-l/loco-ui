//! # Inbox page
//!
//! A list and the item open beside it: messages, tickets, orders. Each item is a link to its
//! own page; the one whose `href` is the current path is marked and its page fills the detail
//! pane. In a wide box the list and the detail sit side by side; in a narrow one an open item
//! shows alone, with a Back link to the list. After the shadcn/ui Mail example: a column of
//! rows with a sender line, a muted preview and the date at the end, the open row on the
//! accent.
//!
//! **Platform features:** plain links and `aria-current="page"`; a container query
//! (Chrome 105, Firefox 110, Safari 16) for the two columns from 48rem.
//!
//! **Accessibility:** the list is a `<nav>` named by the title, the open item's link has
//! `aria-current="page"`, and an unread item says so in words after its title (the dot is
//! decorative). The detail is an `<article>`.
//!
//! **What it does not do without script:** open an item without a page load; each item is a
//! page of its own. With the enhancement script a swap root inside the detail still updates
//! in place.
//!
//! **Fallback:** without container queries the narrow layout shows at every width.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/inbox/2", "", "");
//! let m = ui.inbox_page("Inbox", "/inbox")
//!     .item("Ada Lovelace", "/inbox/1").description("The notes on the engine").when("09:12").unread(true)
//!     .item("Charles Babbage", "/inbox/2").description("Lunch on Friday?").when("Mon")
//!     .body(html! { h2 { "Lunch on Friday?" } })
//!     .render().into_string();
//! assert!(m.contains(r#"href="/inbox/2" aria-current="page""#) && m.contains("data-open"));
//! assert!(m.contains(r#"<span class="lui-sr">unread</span>"#));
//!
//! // The same in `lui!`:
//! let same = lui! { InboxPage("Inbox", "/inbox") {
//!     item "Ada Lovelace" "/inbox/1" description="The notes on the engine" when="09:12" unread=(true);
//!     item "Charles Babbage" "/inbox/2" description="Lunch on Friday?" when="Mon";
//!     body { h2 { "Lunch on Friday?" } }
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::sidebar::current_href;
use crate::{Icon, Ui};

/// One row: its title, its page, a preview line, when, and whether it is unread.
#[derive(Clone, Debug)]
struct Item<'a> {
    title: &'a str,
    href: &'a str,
    description: Option<&'a str>,
    when: Option<&'a str>,
    unread: bool,
}

/// A list beside the open item, made by [`Ui::inbox_page`].
///
/// **Setters.** Values and items: `.item(..)`, `.description(..)`, `.when(..)`, `.body(..)`;
/// conditions: `.unread(..)`.
#[derive(Clone, Debug)]
pub struct InboxPage<'a> {
    ui: &'a Ui,
    title: &'a str,
    href: &'a str,
    items: Vec<Item<'a>>,
    body: Markup,
}

impl InboxPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new("Tab / Shift+Tab", "Moves between the items, top to bottom."),
        Key::new("Enter", "Opens the focused item's page."),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("item", PropKind::Item, "title: &'a str, href: &'a str")
            .doc("An item, after the ones added before it: its title and its page."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .applies_to(&["item"])
            .doc("A preview line under the last item's title."),
        Prop::new("when", PropKind::Modifier, "text: &'a str")
            .applies_to(&["item"])
            .doc("When the last item arrived, formatted (`09:12`, `Mon`)."),
        Prop::new("unread", PropKind::Condition, "on: bool")
            .applies_to(&["item"])
            .doc("Whether the last item is unread: a dot and the word for screen readers."),
        Prop::new("body", PropKind::Value, "body: Markup")
            .doc("The open item, in the detail pane."),
    ];
}

impl Ui {
    /// A list titled `title` whose own page is `href` (where Back goes on a narrow screen).
    pub fn inbox_page<'a>(&'a self, title: &'a str, href: &'a str) -> InboxPage<'a> {
        InboxPage {
            ui: self,
            title,
            href,
            items: Vec::new(),
            body: Markup::default(),
        }
    }
}

impl<'a> InboxPage<'a> {
    /// An item, after the ones added before it: its title and its page.
    pub fn item(mut self, title: &'a str, href: &'a str) -> Self {
        self.items.push(Item {
            title,
            href,
            description: None,
            when: None,
            unread: false,
        });
        self
    }

    /// The item added last, after checking that there is one.
    #[track_caller]
    fn last(&mut self, method: &str) -> Option<&mut Item<'a>> {
        crate::props::check_last::<Self>(
            InboxPage::PROPS,
            method,
            self.items.last().map(|_| "item"),
        );
        self.items.last_mut()
    }

    /// A preview line under the last item's title.
    #[track_caller]
    pub fn description(mut self, text: &'a str) -> Self {
        if let Some(i) = self.last("description") {
            i.description = Some(text);
        }
        self
    }

    /// When the last item arrived, formatted (`"09:12"`, `"Mon"`).
    #[track_caller]
    pub fn when(mut self, text: &'a str) -> Self {
        if let Some(i) = self.last("when") {
            i.when = Some(text);
        }
        self
    }

    /// Whether the last item is unread: a dot, a stronger title, and the word for screen
    /// readers.
    #[track_caller]
    pub fn unread(mut self, on: bool) -> Self {
        if let Some(i) = self.last("unread") {
            i.unread = on;
        }
        self
    }

    /// The open item, in the detail pane.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }
}

impl Render for InboxPage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let current = current_href(ui.state.path(), self.items.iter().map(|i| i.href), false);
        html! {
            div class="lui-inbox" data-open[current.is_some()] { div class="lui-inbox-panes" {
                div class="lui-inbox-list" {
                    h1 class="lui-inbox-title" { (self.title) }
                    nav aria-label=(self.title) {
                        ul {
                            @for i in &self.items {
                                li {
                                    a class="lui-inbox-item" href=(i.href) aria-current=[(current == Some(i.href)).then_some("page")] data-unread[i.unread] {
                                        span class="lui-inbox-item-head" {
                                            span class="lui-inbox-item-title" {
                                                (i.title)
                                                @if i.unread { " " span class="lui-sr" { (ui.text(Text::Unread)) } }
                                            }
                                            @if let Some(w) = i.when { span class="lui-inbox-item-when" { (w) } }
                                        }
                                        @if let Some(d) = i.description { span class="lui-inbox-item-description" { (d) } }
                                    }
                                }
                            }
                        }
                    }
                }
                @if current.is_some() {
                    article class="lui-inbox-detail" {
                        a class="lui-inbox-back" href=(self.href) { (Icon::ChevronLeft) span { (ui.text(Text::Back)) } }
                        (self.body)
                    }
                }
            } }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`]. Narrow: the list, or the open
/// item alone with Back. From 48rem: the list in a fixed column beside the item.
pub const CSS: &str = r#"
.lui-inbox { container: lui-inbox / inline-size; }
.lui-inbox-panes { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-6); }
.lui-inbox[data-open] .lui-inbox-list { display: none; }
.lui-inbox-title { margin: 0 0 var(--lui-space-3); font-size: var(--lui-text-6); }
.lui-inbox-list ul { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-1); }
.lui-inbox-item { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--lui-space-1); padding: var(--lui-space-3); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); color: inherit; text-decoration: none; }
.lui-inbox-item:hover { background: var(--lui-gray-3); }
.lui-inbox-item[aria-current="page"] { background: var(--lui-gray-4); border-color: var(--lui-gray-7); }
.lui-inbox-item-head { display: flex; align-items: baseline; justify-content: space-between; gap: var(--lui-space-2); }
.lui-inbox-item-title { min-width: 0; overflow-wrap: anywhere; font-size: var(--lui-text-2); }
.lui-inbox-item[data-unread] .lui-inbox-item-title { font-weight: 600; }
.lui-inbox-item[data-unread] .lui-inbox-item-title::before { content: ""; display: inline-block; width: 0.5rem; height: 0.5rem; margin-inline-end: var(--lui-space-2); border-radius: 50%; background: var(--lui-primary); vertical-align: middle; }
.lui-inbox-item-when { flex: none; color: var(--lui-muted); font-size: var(--lui-text-1); }
.lui-inbox-item-description { color: var(--lui-muted); font-size: var(--lui-text-1); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.lui-inbox-detail { min-width: 0; }
.lui-inbox-back { display: inline-flex; align-items: center; gap: var(--lui-space-1); margin-block-end: var(--lui-space-4); font-size: var(--lui-text-2); }
.lui-inbox-back svg { width: 1rem; height: 1rem; }
.lui-inbox-back svg:dir(rtl) { scale: -1 1; }
@media (pointer: coarse) { .lui-inbox-item, .lui-inbox-back { min-height: var(--lui-hit); } }
@container lui-inbox (width >= 48rem) {
  .lui-inbox-panes { grid-template-columns: minmax(16rem, 22rem) minmax(0, 1fr); }
  .lui-inbox[data-open] .lui-inbox-list { display: block; }
  .lui-inbox-back { display: none; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn inbox(path: &str) -> String {
        let ui = Ui::from_request(path, "", "");
        ui.inbox_page("Tickets", "/t")
            .item("Login broken", "/t/1")
            .description("Since Monday")
            .when("Mon")
            .unread(true)
            .item("Invoice copy", "/t/2")
            .body(html! { p { "detail" } })
            .render()
            .into_string()
    }

    #[test]
    fn on_the_list_page_nothing_is_open_and_there_is_no_detail() {
        let m = inbox("/t");
        assert!(m.starts_with(r#"<div class="lui-inbox"><div class="lui-inbox-panes"><div class="lui-inbox-list"><h1 class="lui-inbox-title">Tickets</h1><nav aria-label="Tickets">"#), "{m}");
        assert!(
            !m.contains("aria-current") && !m.contains("data-open"),
            "{m}"
        );
        assert!(
            !m.contains("lui-inbox-detail") && !m.contains("detail"),
            "{m}"
        );
    }

    #[test]
    fn the_item_at_the_current_path_opens_beside_a_back_link() {
        let m = inbox("/t/2");
        assert!(m.starts_with(r#"<div class="lui-inbox" data-open>"#), "{m}");
        assert!(
            m.contains(r#"href="/t/2" aria-current="page""#)
                && !m.contains(r#"href="/t/1" aria-current"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<article class="lui-inbox-detail"><a class="lui-inbox-back" href="/t">"#),
            "{m}"
        );
        assert!(
            m.contains(">Back</span>") && m.contains("<p>detail</p>"),
            "{m}"
        );
    }

    #[test]
    fn description_when_and_unread_apply_to_the_item_added_last() {
        let m = inbox("/t");
        let (first, second) = m.split_at(m.find(r#"href="/t/2""#).unwrap());
        assert!(
            first.contains("data-unread")
                && first.contains(r#"<span class="lui-sr">unread</span>"#),
            "{m}"
        );
        assert!(
            first.contains("Since Monday") && first.contains(">Mon<"),
            "{m}"
        );
        assert!(
            !second.contains("data-unread")
                && !second.contains("lui-inbox-item-when")
                && !second.contains("lui-inbox-item-description"),
            "{m}"
        );
    }
}
