//! # Combobox
//!
//! A text input with native suggestions, a server-filtered result list, and a selection
//! shown as removable chips, no script. Suggestions can be grouped, several values can be
//! selected, a "create" row appears when nothing matches, and the selection survives every
//! re-filter because it travels in the form as hidden fields.
//!
//! **Platform features:**
//! - `<input list>` + `<datalist>` (baseline 2020) gives native type-ahead from a fixed list
//!   the server already knows; `<optgroup>` inside it (Chrome 20, Firefox 4, Safari 12.1)
//!   groups the suggestions where the browser draws them grouped.
//! - `<search>` element (baseline 2023) for semantics; the results are a plain list of links
//!   (a `listbox` of options cannot hold links, and without script it could not be operated
//!   as one), named "Results", with `aria-live="polite"` so a swapped list is announced.
//! - Submitting the form (Enter or the button) re-renders with the server's results. Every
//!   result is a link that adds it to the selection (or replaces it, unless `multiple`), every
//!   chip has a link that removes it, and the "create" row is a plain post form.
//!
//! **Accessibility:** a `<search>` form: a labelled search input with a datalist, results a
//! plain list of links named "Results" in an `aria-live` region, chips removable by named
//! links. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** filter as you type against the server or move
//! through results with arrow keys and a live `aria-activedescendant`; suggestions come from
//! the `<datalist>` sent with the page.
//!
//! **Fallback:** none needed; `Caps` is accepted for uniformity and unused.
//!
//! **Without script:** results update per round trip and the arrow keys do not move into
//! the list (Tab does). With the enhancement script the form is submitted as you type and
//! ArrowDown/ArrowUp walk the input and the results.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The text is `?q=` and the selection `?sel=`, read from the request.
//! let ui = Ui::from_request("/langs", "q=ru&sel=Zig", "");
//! let m = ui.combobox("q", "/langs")
//!     .group("Systems", ["Rust", "Zig"])
//!     .options(["Ruby"])
//!     .multiple()
//!     .create("/langs/new")
//!     .label("Language")
//!     .placeholder("Type a language");
//! let html = m.render().into_string();
//! assert!(html.contains("<optgroup label=\"Systems\">"));
//! assert!(html.contains("name=\"sel\" value=\"Zig\""), "the selection rides along with the next search");
//! assert!(html.contains("href=\"/langs?q=ru&amp;sel=Zig&amp;sel=Rust\""), "a result adds itself");
//! assert!(html.contains("href=\"/langs?q=ru\" aria-label=\"Remove Zig\""), "a chip removes itself");
//! // The same in `lui!`:
//! let same = lui! { Combobox("q", "/langs") group=("Systems", ["Rust", "Zig"])
//!     options=(["Ruby"]) multiple create="/langs/new" label="Language"
//!     placeholder="Type a language"; };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::input::Input;
use crate::props::{Prop, PropKind};
use crate::{Icon, Ui};

/// A search form sending `name` (the text) and `sel` (the selection) by GET, made by
/// [`Ui::combobox`]. Single choice, labelled "Search", unless told otherwise.
///
/// **Setters.** Values and items: `.options(..)`, `.group(..)`, `.results(..)`, `.create(..)`,
/// `.label(..)`, `.placeholder(..)`, `.keep(..)`; switches: `.multiple()`.
#[derive(Clone, Debug)]
pub struct Combobox<'a> {
    ui: &'a Ui,
    name: &'a str,
    action: &'a str,
    suggestions: Vec<(Option<&'a str>, Vec<&'a str>)>,
    results: Option<Vec<&'a str>>,
    multi: bool,
    create: Option<&'a str>,
    label: &'a str,
    placeholder: &'a str,
    keep: Vec<&'a str>,
}

impl Combobox<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new(
            "options",
            PropKind::Value,
            "values: impl IntoIterator<Item = &'a str>",
        )
        .doc("Suggestions with no `<optgroup>`."),
        Prop::new(
            "group",
            PropKind::Value,
            "label: &'a str, values: impl IntoIterator<Item = &'a str>",
        )
        .doc("Suggestions under `<optgroup label>`."),
        Prop::new(
            "results",
            PropKind::Value,
            "results: impl IntoIterator<Item = &'a str>",
        )
        .doc("The server's own results for the query."),
        Prop::new("multiple", PropKind::Switch, "")
            .doc("Results add to the selection instead of replacing it."),
        Prop::new("create", PropKind::Value, "action: &'a str")
            .doc("A \"Create\" row posting to `action` when the query matches nothing."),
        Prop::new("label", PropKind::Value, "label: &'a str")
            .default("Search")
            .attr("aria-label")
            .doc("Accessible name of the input."),
        Prop::new("placeholder", PropKind::Value, "placeholder: &'a str")
            .default("Type to search…")
            .doc("Placeholder of the input."),
        Prop::new("keep", PropKind::Item, "name: &'a str")
            .doc("A query parameter of this page (`view`) carried, with the request's value, by the search form, every result and every chip's remove link."),
    ];
}

impl Ui {
    /// A combobox whose text is the query parameter `name` and whose selection is every
    /// `sel`, both read from this request; the form goes to `action`.
    pub fn combobox<'a>(&'a self, name: &'a str, action: &'a str) -> Combobox<'a> {
        Combobox {
            ui: self,
            name,
            action,
            suggestions: Vec::new(),
            results: None,
            multi: false,
            create: None,
            label: self.text(Text::Search),
            placeholder: self.text(Text::TypeToSearch),
            keep: Vec::new(),
        }
    }
}

impl<'a> Combobox<'a> {
    /// Suggestions with no `<optgroup>`.
    pub fn options(mut self, values: impl IntoIterator<Item = &'a str>) -> Self {
        self.suggestions.push((None, values.into_iter().collect()));
        self
    }

    /// Suggestions under `<optgroup label>`.
    pub fn group(mut self, label: &'a str, values: impl IntoIterator<Item = &'a str>) -> Self {
        self.suggestions
            .push((Some(label), values.into_iter().collect()));
        self
    }

    /// The server's own results for the query. Without this, the results are the
    /// suggestions containing the query, ignoring case.
    pub fn results(mut self, results: impl IntoIterator<Item = &'a str>) -> Self {
        self.results = Some(results.into_iter().collect());
        self
    }

    /// Results add to the selection instead of replacing it.
    pub fn multiple(mut self) -> Self {
        self.multi = true;
        self
    }

    /// The old name of [`Self::multiple`], kept for one release.
    #[deprecated(note = "use .multiple()")]
    pub fn multi(self) -> Self {
        self.multiple()
    }

    /// A "Create" row posting to `action` when the query matches nothing; it receives the
    /// text as `name` and the selection as `sel`.
    pub fn create(mut self, action: &'a str) -> Self {
        self.create = Some(action);
        self
    }

    /// A query parameter of this page (`view`, a search of its own) carried, with the
    /// request's value, by the search form, every result and every chip's remove link, so
    /// picking a value keeps what else the page shows. Absent or empty, it is not carried.
    pub fn keep(mut self, name: &'a str) -> Self {
        self.keep.push(name);
        self
    }

    /// Accessible name of the input.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    /// Placeholder of the input.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }
}

impl Render for Combobox<'_> {
    fn render(&self) -> Markup {
        let Combobox {
            ui,
            name,
            action,
            ref suggestions,
            ref results,
            multi,
            create,
            label,
            placeholder,
            ref keep,
        } = *self;
        let query = ui.param(name).unwrap_or("");
        let selected: Vec<&str> = ui.params("sel").collect();
        let needle = query.trim().to_lowercase();
        let results: Vec<&str> = match results {
            Some(r) => r.clone(),
            None if needle.is_empty() => Vec::new(),
            None => suggestions
                .iter()
                .flat_map(|(_, v)| v.iter().copied())
                .filter(|v| v.to_lowercase().contains(&needle))
                .collect(),
        };
        let list_id = format!("{name}-options");
        let results_id = format!("{name}-results");
        let input_id = format!("{name}-input");
        let kept: Vec<(&str, &str)> = (keep.iter())
            .filter_map(|&k| Some((k, ui.param(k).filter(|v| !v.is_empty())?)))
            .collect();
        let link = |typed: &str, chosen: &[&str]| {
            let chosen = chosen.iter().map(|&s| ("sel", s));
            crate::href(
                action,
                [(name, typed)]
                    .into_iter()
                    .chain(chosen)
                    .chain(kept.iter().copied()),
            )
        };
        let add = |v: &str| -> String {
            let mut chosen: Vec<&str> = if multi { selected.clone() } else { Vec::new() };
            chosen.push(v);
            link(query, &chosen)
        };
        let remove = |v: &str| -> String {
            let chosen: Vec<&str> = selected.iter().copied().filter(|s| *s != v).collect();
            link(query, &chosen)
        };
        let nothing = results.is_empty() && !query.is_empty();
        html! {
            search class="lui-combobox" {
                form method="get" action=(action) {
                    @for (k, v) in &kept { input type="hidden" name=(k) value=(v); }
                    @if !selected.is_empty() {
                        ul class="lui-combobox-chips" aria-label=(ui.text(Text::Selected)) {
                            @for v in &selected {
                                li class="lui-combobox-chip" {
                                    (v)
                                    input type="hidden" name="sel" value=(v);
                                    a href=(remove(v)) aria-label=(ui.fill(Text::RemoveValue, &[v])) { "\u{d7}" }
                                }
                            }
                        }
                    }
                    (Input::search_box(name, label, query).id(&input_id).list(&list_id).aria_controls(&results_id).placeholder(placeholder).autocomplete("off").class("lui-combobox-input"))
                    datalist id=(list_id) {
                        @for (group, values) in suggestions {
                            @match group {
                                Some(l) => optgroup label=(l) { @for v in values { option value=(v) {} } },
                                None => { @for v in values { option value=(v) {} } },
                            }
                        }
                    }
                    (ui.button(ui.text(Text::Search)).primary())
                }
                div id=(results_id) class="lui-combobox-results" aria-live="polite" {
                    @if !results.is_empty() {
                        p class="lui-combobox-status" { (crate::palette::count(ui, results.len())) }
                        ul class="lui-combobox-list" aria-label=(ui.text(Text::Results)) {
                            @for r in &results {
                                @let picked = selected.contains(r);
                                li class=[picked.then_some("lui-combobox-chosen")] {
                                    @if picked { (Icon::Check) (r) span class="lui-combobox-picked" { (ui.text(Text::IsSelected)) } }
                                    @else { a href=(add(r)) { (r) } }
                                }
                            }
                        }
                    } @else if nothing {
                        div class="lui-combobox-list" {
                            p class="lui-combobox-status lui-combobox-empty" { (ui.text(Text::NoMatches)) }
                            @if let Some(to) = create {
                                form method="post" action=(to) class="lui-combobox-create" {
                                    @for v in &selected { input type="hidden" name="sel" value=(v); }
                                    input type="hidden" name="name" value=(query);
                                    (ui.button(ui.text(Text::Create)).ghost().body(html! { (Icon::Plus) (ui.fill(Text::CreateValue, &[&query])) }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* One row of chips, search box and button that wraps by itself. */
.lui-combobox form { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); align-items: center; }
.lui-combobox-input { flex: 1; min-width: min(10rem, 100%); }
.lui-combobox-chips { display: contents; }
/* Picked values are soft badges with a remove link. */
.lui-combobox-chip {
  display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.125rem 0.25rem 0.125rem 0.5rem;
  border-radius: var(--lui-radius-sm); background: var(--lui-brand-3); color: var(--lui-brand-11);
  font-size: 0.75rem; line-height: 1rem; font-weight: 500;
}
.lui-combobox-chip a { color: inherit; text-decoration: none; padding: 0 0.25rem; border-radius: var(--lui-radius-sm); line-height: 1rem; }
.lui-combobox-chip a:hover { background: var(--lui-brand-5); }
/* Results take the Select picker's look (Radix Themes Select content): a raised panel of
   2rem rows, a check in the left gutter on the picked one, the primary colour on hover and
   focus, and an empty row when nothing matches. */
.lui-combobox-results { margin: var(--lui-space-2) 0 var(--lui-space-4); }
.lui-combobox-status { margin: 0 0 var(--lui-space-2); font-size: 0.875rem; color: var(--lui-muted); }
.lui-combobox-list {
  list-style: none; margin: 0; padding: var(--lui-space-1); background: var(--lui-popover);
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius); box-shadow: var(--lui-shadow-md), var(--lui-highlight);
}
.lui-combobox-list > li { max-width: none; font-size: 0.875rem; }
.lui-combobox-list a, .lui-combobox-list > .lui-combobox-chosen {
  display: flex; align-items: center; gap: var(--lui-space-2); min-height: 2rem; box-sizing: border-box;
  padding: 0 0.75rem 0 1.5rem; border-radius: var(--lui-radius-sm); text-decoration: none; color: inherit;
}
.lui-combobox-list a:hover, .lui-combobox-list a:focus-visible { background: var(--lui-primary); color: var(--lui-on-primary); outline: none; }
.lui-combobox-list > .lui-combobox-chosen { position: relative; }
.lui-combobox-chosen > .lui-icon { position: absolute; left: 0.375rem; width: 0.875rem; height: 0.875rem; }
.lui-combobox-picked { margin-left: auto; color: var(--lui-muted); font-size: 0.75rem; }
.lui-combobox-empty { margin: 0; padding: var(--lui-space-6) 0; text-align: center; }
.lui-combobox-create { display: flex; justify-content: center; padding-bottom: var(--lui-space-1); }
@media (pointer: coarse) { .lui-combobox-list a, .lui-combobox-list > .lui-combobox-chosen { min-height: var(--lui-hit); } }
"#;

#[cfg(test)]
mod tests {
    use crate::Ui;
    use maud::Render;

    #[test]
    fn keep_carries_a_parameter_through_the_form_and_every_link() {
        let ui = Ui::from_request("/notes", "tag=ru&sel=go&view=list&q=late%20fee&page=2", "");
        let html = (ui.combobox("tag", "/notes").options(["rust"]).multiple())
            .keep("view")
            .keep("q")
            .keep("missing")
            .render()
            .into_string();
        assert!(
            html.contains(r#"<input type="hidden" name="view" value="list">"#),
            "{html}"
        );
        assert!(
            html.contains(r#"<input type="hidden" name="q" value="late fee">"#),
            "{html}"
        );
        let add = "/notes?tag=ru&amp;sel=go&amp;sel=rust&amp;view=list&amp;q=late%20fee";
        let remove = "/notes?tag=ru&amp;view=list&amp;q=late%20fee";
        assert!(html.contains(add) && html.contains(remove), "{html}");
        assert!(
            !html.contains("page") && !html.contains("missing"),
            "{html}"
        );
    }
}
