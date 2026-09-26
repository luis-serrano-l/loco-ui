//! # Sortable list
//!
//! A vertical list whose order the server keeps. Each item has a grip and two named buttons,
//! "Move … up" and "Move … down", in a form posting `item=<key>&to=<index>` (the index the
//! item should end at); the route saves the order and redirects back. With the enhancement
//! script the grip also drags: dropping posts the same form with the new index, and the list
//! is replaced in place.
//!
//! **Platform features:** `<form method="post">` with `name`/`value` buttons, Post/Redirect/Get;
//! an ordered list labelled by `aria-label`; `view-transition-name` per item (Chrome 111,
//! Firefox 144, Safari 18) so a moved item slides; `@media (scripting: enabled)` (Chrome 120,
//! Firefox 113, Safari 17) for the grab cursor; with the script, HTML drag and drop (baseline
//! 2010 on desktop, Safari 15 on iOS).
//!
//! **Accessibility:** the move buttons are the keyboard and screen-reader path, named with the
//! item's title; the grip is decorative (`aria-hidden`). The first item has no "up" and the
//! last no "down"; an empty slot keeps the buttons of every row in line. On a touch screen
//! the buttons are 44px (`--lui-control-h-sm`).
//!
//! **What it does not do without script:** dragging. Every move is a button press, one place
//! at a time.
//!
//! **Fallback:** without view transitions a moved item is simply in its new place.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let list = ui.sortable("Tasks", "/tasks/order")
//!     .item("a", "Write the docs").description("Due Friday")
//!     .item("b", "Ship the release");
//! let m = list.render().into_string();
//! assert!(m.contains(r#"name="item" value="a""#) && m.contains(r#"name="to" value="1""#));
//! assert!(m.contains(r#"aria-label="Move Write the docs down""#));
//! assert!(!m.contains("Move Write the docs up") && !m.contains("Move Ship the release down"));
//!
//! // The same in `lui!`:
//! let same = lui! { Sortable("Tasks", "/tasks/order") {
//!     item "a" "Write the docs" description="Due Friday";
//!     item "b" "Ship the release";
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::{Cap, Icon, Ui, enhance, slug};

/// An item: its key (what a move posts), title and small print.
#[derive(Clone, Debug)]
struct Item<'a> {
    key: &'a str,
    title: &'a str,
    note: Option<&'a str>,
}

/// A sortable list, made by [`Ui::sortable`]. Add items, in their current order, with
/// [`Sortable::item`].
///
/// **Setters.** Values and items: `.item(..)`, `.description(..)`.
#[derive(Clone, Debug)]
pub struct Sortable<'a> {
    ui: &'a Ui,
    label: &'a str,
    action: &'a str,
    items: Vec<Item<'a>>,
}

impl Sortable<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("item", PropKind::Item, "key: &'a str, title: &'a str")
            .doc("An item, after the ones added before it."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .doc("Small print under the item added last."),
    ];
}

impl Ui {
    /// A list named `label` whose moves post to `action`.
    pub fn sortable<'a>(&'a self, label: &'a str, action: &'a str) -> Sortable<'a> {
        Sortable {
            ui: self,
            label,
            action,
            items: Vec::new(),
        }
    }
}

impl<'a> Sortable<'a> {
    /// An item, after the ones added before it: `key` is what a move posts as `item`.
    pub fn item(mut self, key: &'a str, title: &'a str) -> Self {
        self.items.push(Item {
            key,
            title,
            note: None,
        });
        self
    }

    /// Small print under the item added last: an owner, a due date.
    pub fn description(mut self, text: &'a str) -> Self {
        if let Some(item) = self.items.last_mut() {
            item.note = Some(text);
        }
        self
    }
}

impl Render for Sortable<'_> {
    fn render(&self) -> Markup {
        let caps = self.ui.caps;
        let vt = caps.has(Cap::ViewTransitions);
        let root = enhance::swap_id("lui-sortable", self.action);
        let last = self.items.len().saturating_sub(1);
        let arrow = |item: &Item, to: usize, text: Text, icon: Icon| {
            let label = self.ui.fill(text, &[&item.title]);
            Button::new(caps, "")
                .ghost()
                .small()
                .icon_only()
                .name("to")
                .value(&to.to_string())
                .aria_label(&label)
                .body(html! { (icon) })
                .render()
        };
        html! {
            div id=(root) data-lui="swap" data-lui-morph class="lui-sortable" {
                ol class="lui-sortable-list" aria-label=(self.label) {
                    @for (i, item) in self.items.iter().enumerate() {
                        li class="lui-sortable-item" style=[vt.then(|| format!("view-transition-name: lui-sortable-{}", slug(item.key)))] {
                            span class="lui-sortable-grip" aria-hidden="true" { (Icon::GripVertical) }
                            div class="lui-sortable-text" {
                                p class="lui-sortable-title" { (item.title) }
                                @if let Some(n) = item.note { p class="lui-sortable-note" { (n) } }
                            }
                            form method="post" action=(self.action) class="lui-sortable-move" {
                                input type="hidden" name="item" value=(item.key);
                                @if i > 0 { (arrow(item, i - 1, Text::MoveUp, Icon::ChevronUp)) } @else { span class="lui-sortable-slot" {} }
                                @if i < last { (arrow(item, i + 1, Text::MoveDown, Icon::ChevronDown)) } @else { span class="lui-sortable-slot" {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. Rows as small raised cards,
/// the grip on the left, the move buttons on the right; one column at every width.
pub const CSS: &str = r#"
.lui-sortable-list { list-style: none; margin: 0; padding: 0; display: grid; gap: 0.5rem; }
.lui-sortable-item {
  display: flex; align-items: center; gap: 0.5rem; padding: 0.375rem 0.5rem 0.375rem 0.25rem;
  background: var(--lui-card); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); box-shadow: var(--lui-shadow-sm), var(--lui-highlight);
}
.lui-sortable-grip { display: grid; place-items: center; align-self: stretch; width: 1.5rem; color: var(--lui-muted); }
.lui-sortable-grip svg { width: 1rem; height: 1rem; }
@media (scripting: enabled) { .lui-sortable-grip { cursor: grab; } }
.lui-sortable-item[data-lui-dragging] { opacity: 0.5; box-shadow: none; }
.lui-sortable-text { flex: 1; min-width: 0; }
.lui-sortable-title { margin: 0; font-size: 0.875rem; font-weight: 500; overflow-wrap: anywhere; }
.lui-sortable-note { margin: 0; font-size: 0.75rem; color: var(--lui-muted); }
.lui-sortable-move { display: flex; flex: none; }
.lui-sortable-slot { width: var(--lui-control-h-sm); }
"#;
