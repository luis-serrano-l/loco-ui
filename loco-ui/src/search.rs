//! # Search
//!
//! A search box for a page's results: a GET form in a `<search>` landmark, the query in the
//! URL (`?q=`), so the answer is a page that can be bookmarked, shared and reloaded. A Clear
//! link appears once there is a query; it is a plain link to the same page without it.
//!
//! After Radix Themes TextField with a leading magnifier slot (its search example), and the
//! shadcn/ui search form: the field, then the Search button; Clear as a ghost link.
//!
//! **Platform features:** the `<search>` element (Chrome 118, Firefox 118, Safari 17; an
//! unknown element before, which is still a block holding a form), `<form method="get">`
//! and `<input type="search">` (baseline 2015), `aria-controls` naming the results it
//! updates.
//!
//! **Accessibility:** a `search` landmark; the field is named by its label (visually
//! hidden, as `aria-label`), the magnifier is decoration, Clear is a link named "Clear".
//! Checked by axe-core in headless Firefox on every demo route, both capability variants,
//! light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** search as you type; each search is a page load.
//! With the enhancement script, a search inside a swap root replaces only that root.
//!
//! **Fallback:** none needed; where `<search>` is unknown it is an ordinary block and the
//! form inside works the same.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The query is the request's `q`; the form goes back to this page.
//! let ui = Ui::from_request("/notes", "q=rust&view=list", "");
//! let m = ui.search("q", "Search notes").render().into_string();
//! assert!(m.starts_with(r#"<search class="lui-search"><form method="get" action="/notes">"#));
//! assert!(m.contains(r#"name="q" type="search" value="rust""#));
//! assert!(m.contains(r#"href="/notes""#), "Clear drops the query");
//! // The same in `lui!`:
//! assert_eq!(lui! { Search("q", "Search notes"); }.into_string(), m);
//! // Keep another parameter, name the results it updates, post elsewhere.
//! let m = ui.search("q", "Search notes").keep("view").results_id("notes").action("/find");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<input type="hidden" name="view" value="list">"#));
//! assert!(html.contains(r#"aria-controls="notes""#) && html.contains(r#"href="/find?view=list""#));
//! let same = lui! { Search("q", "Search notes") keep="view" results_id="notes" action="/find"; };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::icon::Icon;
use crate::input::Input;
use crate::props::{Prop, PropKind};
use crate::{Ui, button::Button};

/// A search form, made by [`Ui::search`]. It answers on this page unless told otherwise.
///
/// **Setters.** Values and items: `.action(..)`, `.placeholder(..)`, `.results_id(..)`,
/// `.keep(..)`, `.id(..)`.
#[derive(Clone, Debug)]
pub struct Search<'a> {
    ui: &'a Ui,
    name: &'a str,
    label: &'a str,
    action: Option<&'a str>,
    placeholder: Option<&'a str>,
    results: Option<&'a str>,
    keep: Vec<&'a str>,
    id: Option<&'a str>,
}

impl Search<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("action", PropKind::Value, "action: &'a str")
            .attr("action")
            .doc("Where the search goes; this page by default."),
        Prop::new("placeholder", PropKind::Value, "placeholder: &'a str")
            .attr("placeholder")
            .doc("Placeholder text."),
        Prop::new("results_id", PropKind::Value, "id: &'a str")
            .attr("aria-controls")
            .doc("The id of the results it updates, for `aria-controls`."),
        Prop::new("keep", PropKind::Item, "name: &'a str")
            .doc("A query parameter of this page carried by the search and by Clear."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The field's id, `f-<name>` by default."),
    ];
}

impl Ui {
    /// A search for the query parameter `name`, the field named `label` (visually hidden).
    pub fn search<'a>(&'a self, name: &'a str, label: &'a str) -> Search<'a> {
        Search {
            ui: self,
            name,
            label,
            action: None,
            placeholder: None,
            results: None,
            keep: Vec::new(),
            id: None,
        }
    }
}

impl<'a> Search<'a> {
    /// Where the search goes (a results page); this page by default.
    pub fn action(mut self, action: &'a str) -> Self {
        self.action = Some(action);
        self
    }

    /// Placeholder text (`"Search notes…"`).
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// The id of the results it updates: the field's `aria-controls`.
    pub fn results_id(mut self, id: &'a str) -> Self {
        self.results = Some(id);
        self
    }

    /// A query parameter of this page (`view`, a filter) carried, with the request's value,
    /// by the search and by Clear. Absent or empty, it is not carried.
    pub fn keep(mut self, name: &'a str) -> Self {
        self.keep.push(name);
        self
    }

    /// The field's id, `f-<name>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Render for Search<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let action = self.action.unwrap_or_else(|| ui.state.path());
        let query = ui.param(self.name).unwrap_or("");
        let kept: Vec<(&str, &str)> = (self.keep.iter())
            .filter_map(|&k| Some((k, ui.param(k).filter(|v| !v.is_empty())?)))
            .collect();
        let field = Input::search_box(self.name, self.label, query)
            .of(ui)
            .leading(Icon::Search)
            .class("lui-search-input");
        let field = match self.placeholder {
            Some(p) => field.placeholder(p),
            None => field,
        };
        let field = match self.results {
            Some(r) => field.aria_controls(r),
            None => field,
        };
        let field = match self.id {
            Some(id) => field.id(id),
            None => field,
        };
        html! {
            search class="lui-search" {
                form method="get" action=(action) {
                    @for (k, v) in &kept { input type="hidden" name=(k) value=(v); }
                    (field)
                    (Button::new(ui.caps, ui.text(Text::Search)))
                    @if !query.is_empty() {
                        (Button::link(ui.caps, ui.text(Text::Clear), &crate::href(action, kept.iter().copied())).ghost())
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes TextField with a magnifier slot and the shadcn/ui search form: the
   field takes the row, the Search button and the Clear link sit after it and wrap under it
   in a narrow container. */
.lui-search { display: block; container: lui-search / inline-size; min-width: 0; }
.lui-search form { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-2); }
.lui-search .lui-input-slots { flex: 1 1 12rem; width: auto; min-width: 0; }
@container lui-search (width >= 30rem) { .lui-search form { flex-wrap: nowrap; } }
"#;
