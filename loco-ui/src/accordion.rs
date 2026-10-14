//! # Accordion
//!
//! Stacked disclosure sections, no script. Exclusive by default (one open at a time), or
//! `multiple` so several stay open, with "Expand all" and "Collapse all" links. An item can
//! carry a summary line under its title and an icon before it, and a body can hold another
//! accordion.
//!
//! **Platform features:** links and the `hidden` attribute (baseline). The markup follows
//! Radix Accordion: a `.lui-accordion-item` per section holding a `.lui-accordion-trigger`
//! link and a `.lui-accordion-body`, each with `data-state="open|closed"`.
//!
//! **Accessibility:** each title is a link to its own state, with `aria-expanded` and
//! `aria-controls` naming its body; icons are `aria-hidden`. Checked by axe-core in headless
//! Firefox on every demo route, both capability variants, light and dark (no serious or
//! critical violation).
//!
//! **What it does not do without script:** arrow keys between titles (the WAI-ARIA accordion
//! pattern). The body does not animate its height: it is shown by the server's answer.
//!
//! **Fallback:** none needed; the markup is the same everywhere. Before M38 each title was a
//! link inside a `<summary>`, a link inside the summary's implicit button, which is not
//! conforming. Every click already went to the server, so the server alone decides what is open.
//!
//! **Server persistence:** the open sections come from the request's `?open.<group>=0,2` (a
//! comma list, see [`crate::UiState::opens`]) and each title is a link that toggles its own index in
//! that list, so the choice survives navigation. "Expand all" links to every index, "Collapse
//! all" to `open.<group>=` (which removes the key). The group is a swap root for the
//! [`crate::enhance`] script. A nested accordion is its own group with its own key.
//!
//! **Without script:** nothing is lost.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // Items 0 and 2 of "faq" were left open, as the URL records it.
//! let ui = Ui::from_request("/help", "open.faq=0,2", "");
//! // `icon` and `description` apply to the item added last.
//! let m = ui.accordion("faq")
//!     .item("Install", html! { p { "cargo add" } }).icon("\u{1F4E6}").description("One line.")
//!     .item("Use", html! { p { "html!" } })
//!     .item("More", html! { (ui.accordion("faq-more").item("Nested", html! { p { "Own group." } })) })
//!     .multiple()
//!     .controls();
//! let html = m.render().into_string();
//! assert!(html.contains("href=\"/help?open.faq=2\">Install"), "open item's link removes itself from the list");
//! assert!(html.contains("href=\"/help?open.faq=0%2C1%2C2\">Expand all"));
//! assert!(html.contains("class=\"lui-accordion-summary\">One line."));
//! // The same in `lui!`:
//! let same = lui! { Accordion("faq") multiple controls {
//!     item "Install" icon="\u{1F4E6}" description="One line." { p { "cargo add" } }
//!     item "Use" { p { "html!" } }
//!     item "More" { (lui! { Accordion("faq-more") { item "Nested" { p { "Own group." } } } }) }
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::icon::Glyph;
use crate::props::{Prop, PropKind};

/// One section: a title, a body, an optional icon before the title and summary line under it.
#[derive(Clone, Debug)]
struct Item<'a> {
    title: &'a str,
    body: Markup,
    icon: Option<Glyph<'a>>,
    summary: Option<&'a str>,
}

/// Stacked sections, made by [`Ui::accordion`]: the open ones are `?open.<group>=` (or the
/// cookie's memory of it), and each title links to toggle its own. One open at a time unless
/// [`Accordion::multiple`].
///
/// **Setters.** Values and items: `.item(..)`, `.icon(..)`, `.description(..)`; switches:
/// `.multiple()`, `.controls()`.
#[derive(Clone, Debug)]
pub struct Accordion<'a> {
    ui: &'a Ui,
    group: &'a str,
    items: Vec<Item<'a>>,
    multi: bool,
    controls: bool,
}

impl Accordion<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("item", PropKind::Item, "title: &'a str, body: Markup")
            .doc("A section titled `title` with its body."),
        Prop::new("icon", PropKind::Modifier, "icon: impl Into<Glyph<'a>>")
            .doc("An icon, or a glyph or emoji, before the title of the section added last, hidden from assistive tech."),
        Prop::new("description", PropKind::Modifier, "summary: &'a str")
            .doc("A muted line under the title of the section added last, visible while it is closed."),
        Prop::new("multiple", PropKind::Switch, "")
            .doc("Several sections may be open at once (`?open.<group>=0,2`)."),
        Prop::new("controls", PropKind::Switch, "")
            .doc("\"Expand all\" and \"Collapse all\" links above the sections (with `multiple`)."),
    ];
}

impl Ui {
    /// An empty accordion named `group` (the key in `?open.<group>=`); add sections with
    /// [`Accordion::item`].
    pub fn accordion<'a>(&'a self, group: &'a str) -> Accordion<'a> {
        Accordion {
            ui: self,
            group,
            items: Vec::new(),
            multi: false,
            controls: false,
        }
    }
}

impl<'a> Accordion<'a> {
    /// A section titled `title` with its body.
    pub fn item(mut self, title: &'a str, body: Markup) -> Self {
        self.items.push(Item {
            title,
            body,
            icon: None,
            summary: None,
        });
        self
    }

    /// An icon, or a glyph or emoji, before the title of the section added last, hidden from
    /// assistive tech.
    pub fn icon(mut self, icon: impl Into<Glyph<'a>>) -> Self {
        if let Some(it) = self.items.last_mut() {
            it.icon = Some(icon.into());
        }
        self
    }

    /// A muted line under the title of the section added last, visible while it is closed.
    pub fn description(mut self, summary: &'a str) -> Self {
        if let Some(it) = self.items.last_mut() {
            it.summary = Some(summary);
        }
        self
    }

    /// Several sections may be open at once (`?open.<group>=0,2`).
    pub fn multiple(mut self) -> Self {
        self.multi = true;
        self
    }

    /// "Expand all" and "Collapse all" links above the sections (with `multiple`).
    pub fn controls(mut self) -> Self {
        self.controls = true;
        self
    }
}

impl Render for Accordion<'_> {
    fn render(&self) -> Markup {
        let Accordion {
            ui,
            group,
            ref items,
            multi,
            controls,
        } = *self;
        let state = &ui.state;
        let open: Vec<usize> = state.opens(group);
        let key = format!("open.{group}");
        let list = |ix: &[usize]| {
            ix.iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        };
        let toggled = |i: usize| -> String {
            if open.contains(&i) {
                list(&open.iter().copied().filter(|&o| o != i).collect::<Vec<_>>())
            } else if multi {
                let mut all: Vec<usize> = open.iter().copied().chain([i]).collect();
                all.sort_unstable();
                list(&all)
            } else {
                i.to_string()
            }
        };
        html! {
            div id={ "lui-accordion-" (group) } data-lui="swap" class="lui-accordion" {
                @if multi && controls {
                    p class="lui-accordion-controls" {
                        a href=(state.link(&key, &list(&(0..items.len()).collect::<Vec<_>>()))) { (ui.text(Text::ExpandAll)) }
                        a href=(state.link(&key, "")) { (ui.text(Text::CollapseAll)) }
                    }
                }
                @for (i, item) in items.iter().enumerate() {
                    @let is_open = open.contains(&i);
                    @let item_state = if is_open { "open" } else { "closed" };
                    @let body_id = format!("lui-accordion-{group}-{i}");
                    div class="lui-accordion-item" data-state=(item_state) {
                        a class="lui-accordion-trigger" href=(state.link(&key, &toggled(i)))
                            data-state=(item_state) aria-expanded=(is_open) aria-controls=(body_id) {
                            @if let Some(icon) = item.icon { span class="lui-accordion-icon" aria-hidden="true" { (icon) } }
                            span class="lui-accordion-title" {
                                (item.title)
                                @if let Some(line) = item.summary { span class="lui-accordion-summary" { (line) } }
                            }
                        }
                        div id=(body_id) class="lui-accordion-body" data-state=(item_state) hidden[!is_open] { (item.body) }
                    }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* shadcn Accordion: items divided by a bottom rule, text-sm font-medium triggers that
   underline on hover, a chevron on the right that turns when open. */
.lui-accordion-item { border-bottom: 1px solid var(--lui-line); }
.lui-accordion-trigger {
  display: flex; align-items: flex-start; gap: 0.5rem; padding: 1rem 0; color: inherit; text-decoration: none;
  font-size: 0.875rem; line-height: 1.25rem; font-weight: 500;
}
.lui-accordion-trigger:hover .lui-accordion-title { text-decoration: underline; }
/* The chevron is two borders of a rotated square in the muted colour. */
.lui-accordion-trigger::after {
  content: ""; flex: none; width: 0.45rem; height: 0.45rem; margin: 0.3rem 0.25rem 0 auto;
  border-right: 1.5px solid var(--lui-muted); border-bottom: 1.5px solid var(--lui-muted);
  rotate: 45deg; transition: rotate var(--lui-duration) var(--lui-ease-out);
}
.lui-accordion-trigger[data-state="open"]::after { rotate: 225deg; margin-top: 0.5rem; }
.lui-accordion-title { flex: 1; min-width: 0; }
.lui-accordion-icon { font-size: 1.1em; line-height: 1; }
.lui-accordion-summary { display: block; font-weight: 400; font-size: 0.875rem; color: var(--lui-muted); margin-top: 0.15rem; }
.lui-accordion-trigger[data-state="open"] .lui-accordion-summary { display: none; }
.lui-accordion-body { padding: 0 0 1rem; font-size: 0.875rem; }
.lui-accordion-controls { display: flex; gap: calc(var(--lui-space) * 2); margin: 0; max-width: none; padding: 0 0 0.5rem; font-size: 0.875rem; border-bottom: 1px solid var(--lui-line); }
/* A nested accordion sits inside a body, indented. */
.lui-accordion .lui-accordion { margin: 0.5rem 0 0 1rem; }
.lui-accordion .lui-accordion .lui-accordion-item:last-child { border-bottom: 0; }
.lui-accordion .lui-accordion .lui-accordion-trigger { padding: 0.5rem 0; }
.lui-accordion .lui-accordion .lui-accordion-body { padding: 0 0 0.75rem; }
"#;
