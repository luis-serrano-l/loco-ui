//! # Tag input
//!
//! A field that collects several short values (labels, keywords, emails) as removable chips.
//! The tags travel in the form as hidden `<name>[]` fields, so the post that saves the form
//! sends them all; the text box beside them is a `<name>[]` too, so a typed tag rides along
//! with that post as well. Without script, the Add button sends the form back to the page
//! with GET and the tag becomes a chip; each chip's × is a link to the page without it.
//!
//! After the Origin UI tag inputs and the shadcn/ui badge: one bordered box holding soft
//! brand chips (a remove × in each), the text box and the Add button, wrapping as it fills.
//! A combobox's selected values use the same chips (`.lui-tag-input-chip`).
//!
//! **Platform features:** hidden inputs named `<name>[]` (every browser), a button with
//! `formmethod="get"`, `formaction` and `formnovalidate` (baseline 2015) for Add, plain links
//! for remove, and `:has()` (Chrome 105, Firefox 121, Safari 15.4) for the box's focus ring.
//!
//! **Accessibility:** the text box has the field's `<label>`; the chips are a list named
//! "Selected", each × a link named "Remove rust"; help and error describe the text box.
//! Checked by axe-core in headless Firefox on every demo route, both capability variants,
//! light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** turn a tag into a chip on Enter or a comma, or
//! remove one without a round trip. Without script Add and × are page loads that keep the
//! other tags in the URL.
//!
//! **Fallback:** none needed. Without `:has()` the box keeps no ring and the text box shows
//! its own focus outline.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! // The tags are the query's `tags[]` (an Add round trip leaves them there), or `.tags(..)`.
//! let ui = Ui::from_request("/note", "tags%5B%5D=rust&tags%5B%5D=web", "");
//! let m = ui.tag_input("tags", "Tags").render().into_string();
//! assert!(m.contains(r#"<input type="hidden" name="tags[]" value="rust">"#));
//! assert!(m.contains(r#"href="/note?tags%5B%5D=web" aria-label="Remove rust""#));
//! assert!(m.contains(r#"formmethod="get" formaction="/note""#));
//! // The same in `lui!`:
//! assert_eq!(lui! { TagInput("tags", "Tags"); }.into_string(), m);
//! // Saved tags, a placeholder and the page the Add round trip goes to.
//! let m = ui.tag_input("tags", "Tags").tags(&["css"]).placeholder("Add a tag").action("/note/edit");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"value="css""#) && !html.contains(r#"value="rust""#));
//! let same = lui! { TagInput("tags", "Tags") tags=(&["css"]) placeholder="Add a tag" action="/note/edit"; };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::button::Button;
use crate::i18n::Text;
use crate::input::{Input, described_by, error_line};
use crate::props::{Key, Prop, PropKind};

/// Chips and a text box posting `<name>[]`, made by [`Ui::tag_input`].
///
/// **Setters.** Values and items: `.tags(..)`, `.action(..)`, `.placeholder(..)`,
/// `.help(..)`, `.error(..)`, `.id(..)`.
#[derive(Clone, Debug)]
pub struct TagInput<'a> {
    ui: &'a Ui,
    name: &'a str,
    label: &'a str,
    tags: Vec<&'a str>,
    action: Option<&'a str>,
    placeholder: Option<&'a str>,
    help: Option<&'a str>,
    error: Option<&'a str>,
    id: Option<&'a str>,
}

impl TagInput<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves between the remove links, the text box and the Add button.",
        ),
        Key::new(
            "Enter",
            "Adds the typed tag when the Add button is the first submit button in its form.",
        ),
        Key::new(
            "Enter / Space",
            "Adds the typed tag from the focused Add button.",
        ),
        Key::new("Enter", "Removes the tag from its focused remove link."),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("tags", PropKind::Value, "tags: &'a [&'a str]")
            .doc("The tags, instead of the query's `<name>[]`."),
        Prop::new("action", PropKind::Value, "action: &'a str")
            .attr("formaction")
            .doc("The page Add sends the form back to with GET; this page by default."),
        Prop::new("placeholder", PropKind::Value, "placeholder: &'a str")
            .attr("placeholder")
            .doc("Placeholder of the text box."),
        Prop::new("help", PropKind::Value, "help: &'a str").doc("Help text under the box."),
        Prop::new("error", PropKind::Value, "message: &'a str")
            .doc("A server message under the box."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The text box's id, `f-<name>` by default."),
    ];
}

impl Ui {
    /// Tags posted as `<name>[]` under the label `label`; they start from the query's
    /// `<name>[]` (empty ones and repeats dropped) unless [`TagInput::tags`] says otherwise.
    pub fn tag_input<'a>(&'a self, name: &'a str, label: &'a str) -> TagInput<'a> {
        let mut tags: Vec<&'a str> = Vec::new();
        for t in self.param_values(&format!("{name}[]")) {
            let t = t.trim();
            if !t.is_empty() && !tags.contains(&t) {
                tags.push(t);
            }
        }
        TagInput {
            ui: self,
            name,
            label,
            tags,
            action: None,
            placeholder: None,
            help: None,
            error: None,
            id: None,
        }
    }
}

impl<'a> TagInput<'a> {
    /// The tags, instead of the query's `<name>[]`: saved ones.
    pub fn tags(mut self, tags: &'a [&'a str]) -> Self {
        self.tags = tags.to_vec();
        self
    }

    /// The page Add sends the form back to with GET (an edit page), and the chips' remove
    /// links point at; this page by default.
    pub fn action(mut self, action: &'a str) -> Self {
        self.action = Some(action);
        self
    }

    /// Placeholder of the text box (`"Add a tag"`).
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Help text under the box.
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// A server message under the box; the text box gets `aria-invalid`. An empty message
    /// is no error.
    pub fn error(mut self, message: &'a str) -> Self {
        self.error = (!message.is_empty()).then_some(message);
        self
    }

    /// The text box's id, `f-<name>` by default.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Render for TagInput<'_> {
    fn render(&self) -> Markup {
        let ui = self.ui;
        let id = self
            .id
            .map_or_else(|| format!("f-{}", self.name), str::to_string);
        let key = format!("{}[]", self.name);
        let action = self.action.unwrap_or_else(|| ui.state.path());
        let others: Vec<(&str, &str)> = ui.param_pairs().filter(|(k, _)| *k != key).collect();
        // A chip's link: the page with every other parameter and every other tag.
        let without = |tag: &str| {
            let rest = self
                .tags
                .iter()
                .filter(|t| **t != tag)
                .map(|t| (key.as_str(), *t));
            crate::href(action, others.iter().copied().chain(rest))
        };
        let described = described_by(&[
            self.help.map(|_| format!("{id}-help")),
            self.error.map(|_| format!("{id}-error")),
        ]);
        html! {
            div class="lui-field lui-tag-input" {
                label for=(id) { (self.label) }
                div class="lui-tag-input-box" {
                    @if !self.tags.is_empty() {
                        ul class="lui-tag-input-chips" aria-label=(ui.text(Text::Selected)) {
                            @for t in &self.tags {
                                li class="lui-tag-input-chip" {
                                    (t)
                                    input type="hidden" name=(key) value=(t);
                                    a href=(without(t)) aria-label=(ui.fill(Text::RemoveValue, &[t])) { "\u{d7}" }
                                }
                            }
                        }
                    }
                    @let field = Input::text_box(&key, self.label, "").id(&id).class("lui-tag-input-field").autocomplete("off").described(described.as_deref());
                    @let field = match self.placeholder { Some(p) => field.placeholder(p), None => field };
                    (field)
                    (Button::new(ui.caps, ui.text(Text::Add)).ghost().formmethod("get").formaction(action).formnovalidate())
                }
                @if let Some(h) = self.help { small id={ (id) "-help" } class="lui-field-help" { (h) } }
                @if let Some(e) = self.error { (error_line(&format!("{id}-error"), e)) }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. The chip rules are shared
/// with [`crate::combobox`]'s selection.
pub const CSS: &str = r#"
/* After the Origin UI tag inputs: one --lui-input bordered box at the control height holding
   the chips, a borderless text box that takes the rest of the row, and a ghost Add button;
   it wraps onto more rows as the chips fill it. A chip is the shadcn soft badge: --lui-brand-3
   fill, --lui-brand-11 text, text-1/500, a remove x that tints on hover. */
.lui-tag-input-box {
  display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-1); box-sizing: border-box; width: 100%;
  min-height: var(--lui-control-h); padding: 0.1875rem; border: 1px solid var(--lui-input); border-radius: var(--lui-radius-sm);
  transition: border-color var(--lui-duration-fast);
}
.lui-tag-input-box:has(.lui-tag-input-field:focus-visible) { border-color: var(--lui-ring); outline: 3px solid color-mix(in srgb, var(--lui-ring) 50%, transparent); }
.lui-tag-input-box:has([aria-invalid=true]) { border-color: var(--lui-danger); }
.lui-tag-input-chips { display: contents; list-style: none; }
.lui-tag-input-box > .lui-tag-input-field { flex: 1 1 8rem; width: auto; min-width: 0; min-height: calc(var(--lui-control-h) - 0.5rem); padding-block: 0; border: 0; background: transparent; }
.lui-tag-input-box > .lui-tag-input-field:focus-visible { outline: none; }
.lui-tag-input-chip {
  display: inline-flex; align-items: center; gap: 0.25rem; max-width: 100%; padding-block: 0.125rem; padding-inline: 0.5rem 0.25rem;
  border-radius: var(--lui-radius-sm); background: var(--lui-brand-3); color: var(--lui-brand-11);
  font-size: var(--lui-text-1); line-height: var(--lui-leading-1); font-weight: 500; overflow-wrap: anywhere;
}
.lui-tag-input-chip a { color: inherit; text-decoration: none; padding: 0 0.25rem; border-radius: var(--lui-radius-sm); }
.lui-tag-input-chip a:hover { background: var(--lui-brand-5); }
@media (pointer: coarse) {
  .lui-tag-input-chip a { display: inline-grid; place-items: center; min-width: var(--lui-hit); min-height: var(--lui-hit); }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_and_repeated_tags_are_dropped() {
        let ui = Ui::from_request("/n", "t%5B%5D=a&t%5B%5D=&t%5B%5D=a&t%5B%5D=b", "");
        let m = ui
            .tag_input("t", "T")
            .error("Too many.")
            .render()
            .into_string();
        assert_eq!(m.matches(r#"type="hidden""#).count(), 2, "{m}");
        crate::input::assert_described_ids_exist(&m);
    }
}
