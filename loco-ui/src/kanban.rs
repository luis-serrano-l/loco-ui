//! # Kanban
//!
//! A board of columns and cards. Moving a card is a form post: each card carries arrow buttons
//! to the column on its left and on its right, posting `card=<key>&to=<column>`; the route
//! saves the move and redirects back. A column can have a work-in-progress limit.
//!
//! **Platform features:** `<form method="post">` with `name`/`value` buttons, Post/Redirect/Get;
//! each column is a `<section>` with a heading and an ordered list; the board scrolls sideways
//! (`overflow-x: auto`, `scroll-snap-type`, Chrome 69, Firefox 68, Safari 11) with each column
//! at 85% of the board's width when the board is narrow, so the next one peeks; `@container`
//! (Chrome 105, Firefox 110, Safari 16) puts the columns side by side from 48rem; `view-transition-name` per card (Chrome 111, Firefox 144, Safari 18) so, with the
//! enhancement script, a moved card slides to its new column.
//!
//! **Accessibility:** each column is a `<section>` labelled by its heading; every card moves
//! with named buttons ("Move … to …") instead of drag and drop. Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** drag and drop, or reorder cards within a column
//! (the server decides the order: a moved card goes last in its new column).
//!
//! **Fallback:** without view transitions a card is simply in its new column after the move;
//! without container queries the board keeps its narrow form (snapping columns that scroll).
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let board = ui.kanban("/board/move")
//!     .column("todo", "To do").card("c1", "Write the docs")
//!     .column("doing", "Doing").limit(2).card("c2", "Calendar").description("M23").badge("Beta")
//!     .card("c3", "Upload")
//!     .column("done", "Done");
//! let m = board.render().into_string();
//! assert!(m.contains(r#"name="card" value="c1""#) && m.contains(r#"name="to" value="doing""#));
//! assert!(m.contains("2/2") && m.contains(">Beta</span>") && !m.contains(r#"value="todo" aria-label="Move Write"#));
//!
//! // The same in `lui!`:
//! let same = lui! { Kanban("/board/move") {
//!     column "todo" "To do" { card "c1" "Write the docs"; }
//!     column "doing" "Doing" limit=2 {
//!         card "c2" "Calendar" description="M23" badge="Beta";
//!         card "c3" "Upload";
//!     }
//!     column "done" "Done";
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use std::fmt::Display;

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::{Cap, Icon, Ui, enhance, slug};

/// A card: its key (what the move posts), title and small print.
#[derive(Clone, Debug)]
struct Card<'a> {
    key: &'a str,
    title: &'a str,
    note: Option<&'a str>,
    badge: Option<String>,
}

/// A column: its key (the `to` a move posts), title, limit and cards.
#[derive(Clone, Debug)]
struct Column<'a> {
    key: &'a str,
    title: &'a str,
    limit: Option<usize>,
    cards: Vec<Card<'a>>,
}

/// A board, made by [`Ui::kanban`]. Add columns with [`Kanban::column`] and cards, into the
/// column added last, with [`Kanban::card`].
///
/// **Setters.** Values and items: `.column(..)`, `.limit(..)`, `.card(..)`, `.description(..)`,
/// `.badge(..)`.
#[derive(Clone, Debug)]
pub struct Kanban<'a> {
    ui: &'a Ui,
    action: &'a str,
    columns: Vec<Column<'a>>,
}

impl Kanban<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("column", PropKind::Item, "key: &'a str, title: &'a str").doc("A column."),
        Prop::new("limit", PropKind::Modifier, "limit: usize")
            .doc("A work-in-progress limit for the column added last."),
        Prop::new("card", PropKind::Modifier, "key: &'a str, title: &'a str")
            .doc("A card in the column added last."),
        Prop::new("description", PropKind::Modifier, "text: &'a str")
            .doc("Small print under the card added last."),
        Prop::new("badge", PropKind::Modifier, "text: impl Display")
            .doc("A badge in the meta row of the card added last: a label, an estimate, a tag."),
    ];
}

impl Ui {
    /// A board whose moves post to `action`.
    pub fn kanban<'a>(&'a self, action: &'a str) -> Kanban<'a> {
        Kanban {
            ui: self,
            action,
            columns: Vec::new(),
        }
    }
}

impl<'a> Kanban<'a> {
    /// A column: `key` is what a move to it posts as `to`, `title` its heading.
    pub fn column(mut self, key: &'a str, title: &'a str) -> Self {
        self.columns.push(Column {
            key,
            title,
            limit: None,
            cards: Vec::new(),
        });
        self
    }

    /// A work-in-progress limit for the column added last: its count reads `n / limit` and
    /// turns red past it (the server decides whether to refuse a move).
    pub fn limit(mut self, limit: usize) -> Self {
        if let Some(c) = self.columns.last_mut() {
            c.limit = Some(limit);
        }
        self
    }

    /// A card in the column added last: `key` is what a move posts as `card`.
    pub fn card(mut self, key: &'a str, title: &'a str) -> Self {
        if let Some(c) = self.columns.last_mut() {
            c.cards.push(Card {
                key,
                title,
                note: None,
                badge: None,
            });
        }
        self
    }

    /// Small print under the card added last: an owner, a due date, a tag.
    pub fn description(mut self, text: &'a str) -> Self {
        if let Some(card) = self.columns.last_mut().and_then(|c| c.cards.last_mut()) {
            card.note = Some(text);
        }
        self
    }

    /// A badge in the meta row of the card added last, beside its move buttons: a label, an
    /// estimate, a tag.
    pub fn badge(mut self, text: impl Display) -> Self {
        if let Some(card) = self.columns.last_mut().and_then(|c| c.cards.last_mut()) {
            card.badge = Some(text.to_string());
        }
        self
    }

    /// The old name of [`Self::description`], kept for one release.
    #[deprecated(note = "use .description()")]
    pub fn note(self, text: &'a str) -> Self {
        self.description(text)
    }
}

impl Render for Kanban<'_> {
    fn render(&self) -> Markup {
        let caps = self.ui.caps;
        let vt = caps.has(Cap::ViewTransitions);
        let root = enhance::swap_id("lui-kanban", self.action);
        let cols = &self.columns;
        html! {
            div id=(root) data-lui="swap" data-lui-morph class="lui-kanban" {
                div class="lui-kanban-board" {
                    @for (i, col) in cols.iter().enumerate() {
                        @let heading = format!("{root}-{}", slug(col.key));
                        @let over = col.limit.is_some_and(|l| col.cards.len() > l);
                        @let count = match col.limit { Some(l) => format!("{}/{l}", col.cards.len()), None => col.cards.len().to_string() };
                        section class="lui-kanban-column" aria-labelledby=(heading) {
                            header class="lui-kanban-head" {
                                h3 id=(heading) { (col.title) }
                                span class={ "lui-kanban-count" @if over { " lui-kanban-over" } } {
                                    @if over { (self.ui.badge(&count).danger()) span class="lui-sr" { (self.ui.text(Text::OverLimit)) } }
                                    @else { (self.ui.badge(&count).secondary()) }
                                }
                            }
                            @if col.cards.is_empty() {
                                p class="lui-kanban-empty" { (self.ui.text(Text::NoCards)) }
                            } @else {
                                ol class="lui-kanban-cards" {
                                    @for card in &col.cards {
                                        li class="lui-kanban-card" style=[vt.then(|| format!("view-transition-name: lui-kanban-{}", slug(card.key)))] {
                                            p class="lui-kanban-title" { (card.title) }
                                            @if let Some(n) = card.note { p class="lui-kanban-note" { (n) } }
                                            div class="lui-kanban-meta" {
                                                @if let Some(b) = &card.badge { (self.ui.badge(b).secondary()) }
                                                form method="post" action=(self.action) class="lui-kanban-move" {
                                                    input type="hidden" name="card" value=(card.key);
                                                    @if let Some(prev) = i.checked_sub(1).and_then(|p| cols.get(p)) {
                                                        @let label = self.ui.fill(Text::MoveTo, &[&card.title, &prev.title]);
                                                        (Button::new(caps, "").ghost().small().icon_only().name("to").value(prev.key).aria_label(&label).body(html! { (Icon::ArrowLeft) }))
                                                    }
                                                    @if let Some(next) = cols.get(i + 1) {
                                                        @let label = self.ui.fill(Text::MoveTo, &[&card.title, &next.title]);
                                                        (Button::new(caps, "").ghost().small().icon_only().name("to").value(next.key).aria_label(&label).body(html! { (Icon::ArrowRight) }))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. Columns on the surface, cards
/// as small shadcn cards; the board scrolls sideways when the columns do not fit.
pub const CSS: &str = r#"
/* The board is its own container. Narrow (and where container queries are missing) each
   column is 85% of the board, so the next one peeks, and the board snaps column by column;
   from 48rem the columns share the width, and scroll only when there are too many. */
.lui-kanban { container: lui-kanban / inline-size; }
.lui-kanban-board {
  display: grid; grid-auto-flow: column; grid-auto-columns: 85%; gap: 0.75rem; align-items: start;
  overflow-x: auto; overscroll-behavior-x: contain; scroll-snap-type: x mandatory; padding-bottom: 0.5rem;
  position: relative; /* holds the visually hidden (absolute) "over the limit" text inside the scroller */
}
@container lui-kanban (min-width: 48rem) {
  .lui-kanban-board { grid-auto-columns: minmax(15rem, 1fr); scroll-snap-type: none; }
}
.lui-kanban-column {
  display: grid; align-content: start; gap: 0.5rem; padding: 0.75rem; scroll-snap-align: start;
  background: var(--lui-gray-3); border-radius: var(--lui-radius-lg);
}
.lui-kanban-head { display: flex; align-items: center; justify-content: space-between; gap: 0.5rem; padding: 0 0.25rem 0.25rem; }
.lui-kanban-head h3 { margin: 0; font-size: 0.875rem; font-weight: 600; }
.lui-kanban-count { font-variant-numeric: tabular-nums; }
.lui-kanban-cards { list-style: none; margin: 0; padding: 0; display: grid; gap: 0.5rem; }
.lui-kanban-card {
  display: grid; gap: 0.25rem; padding: 0.75rem 0.75rem 0.5rem;
  background: var(--lui-card); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); box-shadow: var(--lui-shadow-sm), var(--lui-highlight);
}
.lui-kanban-title { margin: 0; font-size: 0.875rem; font-weight: 500; }
.lui-kanban-note { margin: 0; font-size: 0.75rem; color: var(--lui-muted); }
.lui-kanban-meta { display: flex; align-items: center; gap: 0.5rem; min-height: 2rem; }
.lui-kanban-move { display: flex; margin: 0 -0.375rem 0 auto; }
.lui-kanban-empty { margin: 0; padding: 1rem; text-align: center; font-size: 0.875rem; color: var(--lui-muted); border: 1px dashed var(--lui-gray-6); border-radius: var(--lui-radius); }
"#;
