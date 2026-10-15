//! # Record page
//!
//! One record: its title, its fields as a description list in a card, and the actions on it
//! (edit, delete) beside the title. The show page a scaffold writes, ready-made.
//!
//! **Platform features:** a [`crate::description_list`] (its container query and subgrid);
//! delete opens a [`crate::dialog`] ("Delete Invoice 42?") whose confirm button is a form that
//! posts (Post/Redirect/Get).
//!
//! **Accessibility:** the title is the page's `<h1>`; labels and values are `<dt>`/`<dd>`
//! pairs; edit is a link, delete a real button that opens a modal dialog. Checked by axe-core
//! with the demo routes.
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** the dialog's own: without invoker commands the Delete button is a link to
//! `#lui-record-delete` and the dialog opens through `:target`.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.record_page("Invoice 42").field("Customer", "Ada").field("Total", "€120")
//!     .edit("/invoices/42/edit").delete("/invoices/42/delete").render().into_string();
//! assert!(m.contains("<dt>Customer</dt><dd>Ada</dd>") && m.contains(r#"action="/invoices/42/delete""#));
//! assert!(m.contains(r#"<h2 id="lui-record-delete-title" class="lui-dialog-title">Delete Invoice 42?</h2>"#));
//! // The same in `lui!`:
//! let same = lui! { RecordPage("Invoice 42") edit="/invoices/42/edit" delete="/invoices/42/delete" {
//!     field "Customer" "Ada"; field "Total" "€120";
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};

/// A record's show page, made by [`Ui::record_page`].
///
/// **Setters.** Values and items: `.field(..)`, `.edit(..)`, `.delete(..)`, `.back(..)`.
#[derive(Clone, Debug)]
pub struct RecordPage<'a> {
    ui: &'a Ui,
    title: &'a str,
    fields: Vec<(&'a str, Markup)>,
    edit: Option<&'a str>,
    delete: Option<&'a str>,
    back: Option<&'a str>,
}

impl RecordPage<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new("Enter", "Follows the Back or Edit link."),
        Key::new(
            "Enter / Space",
            "Opens the delete confirmation from the Delete button.",
        ),
        Key::new(
            "Escape",
            "Closes the delete confirmation without deleting. Without invoker commands or the script, its Close link closes it instead.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets.
    pub const PROPS: &'static [Prop] = &[
        Prop::new(
            "field",
            PropKind::Item,
            "label: &'a str, value: impl Render",
        )
        .doc("A label and its value."),
        Prop::new("edit", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("An Edit link."),
        Prop::new("delete", PropKind::Value, "action: &'a str")
            .attr("action")
            .doc("A Delete button that asks first, in a dialog, then posts to `action`."),
        Prop::new("back", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("A Back link to the list."),
    ];
}

impl Ui {
    /// The page of the record titled `title`.
    pub fn record_page<'a>(&'a self, title: &'a str) -> RecordPage<'a> {
        RecordPage {
            ui: self,
            title,
            fields: Vec::new(),
            edit: None,
            delete: None,
            back: None,
        }
    }
}

impl<'a> RecordPage<'a> {
    /// A label and its value.
    pub fn field(mut self, label: &'a str, value: impl Render) -> Self {
        self.fields.push((label, value.render()));
        self
    }

    /// An Edit link.
    pub fn edit(mut self, href: &'a str) -> Self {
        self.edit = Some(href);
        self
    }

    /// A Delete button that asks first, in a dialog, then posts to `action`.
    pub fn delete(mut self, action: &'a str) -> Self {
        self.delete = Some(action);
        self
    }

    /// A Back link to the list.
    pub fn back(mut self, href: &'a str) -> Self {
        self.back = Some(href);
        self
    }
}

impl Render for RecordPage<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let list = (self.fields.iter()).fold(ui.description_list(), |l, (label, value)| {
            l.item(label, value)
        });
        html! {
            div class="lui-record" {
                div class="lui-record-head" {
                    h1 { (self.title) }
                    div class="lui-record-actions" {
                        @if let Some(b) = self.back { (ui.link_button(ui.text(Text::Back), b).ghost()) }
                        @if let Some(e) = self.edit { (ui.link_button(ui.text(Text::Edit), e)) }
                        @if let Some(d) = self.delete {
                            @let title = ui.fill(Text::DeleteConfirm, &[&self.title]);
                            (ui.dialog(ui.text(Text::Delete)).id("lui-record-delete").title(&title).size(1).danger()
                                .confirm(ui.text(Text::Delete), d)
                                .body(html! { p { (ui.text(Text::CannotUndo)) } }))
                        }
                    }
                }
                div class="lui-record-fields" { (list) }
            }
        }
    }
}

/// Styles for this block; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-record { display: grid; grid-template-columns: minmax(0, 1fr); gap: calc(var(--lui-space) * 2); }
.lui-record-head { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: calc(var(--lui-space) * 2); }
.lui-record-head h1 { margin: 0; }
.lui-record-actions { display: flex; gap: var(--lui-space); }
.lui-record-actions form { margin: 0; }
/* The fields are a description list in a card. */
.lui-record-fields { padding: var(--lui-space-6); background: var(--lui-card); border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg); }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Caps;

    #[test]
    fn fields_are_term_and_detail_pairs_under_an_h1_with_no_actions_by_default() {
        let ui = Ui::default();
        let m = ui
            .record_page("Invoice 42")
            .field("Customer", "Ada")
            .field("Total", html! { b { "€120" } })
            .render()
            .into_string();
        assert!(
            m.starts_with(
                r#"<div class="lui-record"><div class="lui-record-head"><h1>Invoice 42</h1>"#
            ),
            "{m}"
        );
        assert!(m.contains("<dt>Customer</dt><dd>Ada</dd>"), "{m}");
        assert!(m.contains("<dt>Total</dt><dd><b>€120</b></dd>"), "{m}");
        assert!(
            m.find("Customer").unwrap() < m.find("Total").unwrap(),
            "{m}"
        );
        assert!(!m.contains("<dialog") && !m.contains("lui-button"), "{m}");
    }

    #[test]
    fn edit_and_back_are_links_and_delete_asks_in_a_dialog_that_posts() {
        let ui = Ui::default();
        let m = ui
            .record_page("Invoice 42")
            .back("/invoices")
            .edit("/invoices/42/edit")
            .delete("/invoices/42/delete")
            .render()
            .into_string();
        assert!(
            m.contains(r#"href="/invoices""#) && m.contains(r#"href="/invoices/42/edit""#),
            "{m}"
        );
        assert!(m.contains(r#"<h2 id="lui-record-delete-title" class="lui-dialog-title">Delete Invoice 42?</h2>"#), "{m}");
        assert!(
            m.contains(r#"method="post""#) && m.contains(r#"action="/invoices/42/delete""#),
            "{m}"
        );
        assert!(
            m.find("/invoices\"").unwrap() < m.find("/edit\"").unwrap(),
            "{m}"
        );
    }

    #[test]
    fn the_delete_button_opens_by_invoker_or_by_target_depending_on_caps() {
        let page = |caps| {
            Ui::from(caps)
                .record_page("R")
                .delete("/d")
                .render()
                .into_string()
        };
        let (on, off) = (page(Caps::all()), page(Caps::NONE));
        assert!(
            on.contains("commandfor") && !off.contains("commandfor"),
            "{on}\n{off}"
        );
        assert!(
            off.contains(r##"href="#lui-record-delete""##)
                && !on.contains(r##"href="#lui-record-delete""##),
            "{on}\n{off}"
        );
    }
}
