//! # Select
//!
//! A `<select>` whose closed state shows the chosen option's full content, no script. Options
//! come in labelled groups and can carry an icon; a long list gets a filter box.
//!
//! **Platform features:**
//! - `<select>` with a `<button>` first child holding `<selectedcontent>` (Chrome 135, Safari
//!   27; Firefox behind flags): the button is the closed control and `<selectedcontent>`
//!   mirrors the picked option's markup into it, so options can carry a swatch or an icon.
//! - `appearance: base-select` on the select and its `::picker(select)` pseudo-element hands
//!   both to author CSS.
//! - `<optgroup label>` (baseline) for [`Select::group`].
//! - A filter box when there are more than [`Select::search_over`] options: an
//!   `<input type="search" name="<name>-q">` and a button with `formmethod="get"` and
//!   `formaction` (baseline 2015), so filtering re-requests the page through the enclosing
//!   form without saving it. The server renders only the options whose text contains the
//!   query, and always the selected one.
//!
//! **Accessibility:** a native `<select>` with its `<label>`; the filter box is a labelled
//! search field. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** type-ahead search beyond what the browser offers;
//! long lists get a server-side filter box instead.
//!
//! **Fallback:** without `Caps::BaseSelect` a plain `<select>` with plain options (icons as
//! text before the label). Older parsers also drop a `<button>` inside `<select>`, so the
//! enhanced markup is only emitted when the browser is known to want it.
//!
//! **Enhanced:** the enhancement script filters as you type, through the same GET.
//!
//! ```rust
//! use loco_ui::{prelude::*, select::SelectOption};
//! // The chosen value is the query's `food`, as a form posted back by GET leaves it.
//! let mut ui = Ui::from_request("/shop", "food=leek&food-q=k", "");
//! ui.caps = Caps::all();
//! // Plain tuples of (value, label) or (value, label, icon) are options.
//! let m = ui.select("size", "Size").options([("s", "Small", "🐭"), ("l", "Large", "🐘")]).value("l");
//! let m = m.render().into_string();
//! assert!(m.contains("<selectedcontent>") && m.contains(r#"<label for="f-size">Size</label>"#));
//! assert!(m.contains(r#"<option value="l" selected>"#));
//! // A server error, linked from the error summary like a form field's.
//! let bad = ui.select("size", "Size").options([("s", "Small")]).error("Pick a size.");
//! assert!(bad.render().into_string().contains(r#"aria-describedby="f-size-error""#));
//!
//! let m = ui.select("food", "Food")
//!     .group("Fruit", [SelectOption::new("apple", "Apple"), SelectOption::new("kiwi", "Kiwi").body(html! { b { "Kiwi" } })])
//!     .group("Vegetables", [("leek", "Leek")])
//!     .search("/shop")
//!     .search_over(2);
//! let m = m.render().into_string();
//! assert!(m.contains("<optgroup label=\"Fruit\">") && !m.contains("Apple"), "filtered to 'k'");
//! assert!(m.contains("formmethod=\"get\" formaction=\"/shop\""));
//!
//! // The same in `lui!`:
//! let same = lui! { Select("food", "Food") search="/shop" search_over=2 {
//!     group "Fruit" ([
//!         SelectOption::new("apple", "Apple"),
//!         SelectOption::new("kiwi", "Kiwi").body(html! { b { "Kiwi" } }),
//!     ]);
//!     group "Vegetables" ([("leek", "Leek")]);
//! } };
//! assert_eq!(same.into_string(), m);
//! // A small soft trigger.
//! let sort = ui.select("sort", "Sort").options([("new", "Newest")]).size(1).variant(Variant::Soft);
//! let sort = sort.render().into_string();
//! assert!(sort.contains(r#"<span class="lui-select" data-size="1" data-variant="soft">"#));
//! let same = lui! { Select("sort", "Sort") options=([("new", "Newest")]) size=1 variant="soft"; };
//! assert_eq!(same.into_string(), sort);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::{Strings, Text};
use crate::icon::Glyph;
use crate::input::{Input, described_by, keep};
use crate::props::{Key, Prop, PropKind};
use crate::variant::{Size, Variant};
use crate::{Cap, Caps, Ui};

/// One option: its value, its text (what the filter matches), an optional icon and optional
/// rich content shown instead of the text.
///
/// **Setters.** Values and items: `.icon(..)`, `.body(..)`; switches: `.disabled()`.
#[derive(Clone, Debug)]
pub struct SelectOption<'a> {
    /// Posted value.
    pub value: &'a str,
    /// Plain label; matched by the filter.
    pub text: &'a str,
    /// An icon, or a glyph or emoji, before the label, hidden from assistive tech.
    pub icon: Option<Glyph<'a>>,
    /// Markup shown instead of `text` when the select is rich.
    pub content: Option<Markup>,
    /// Shown, but cannot be picked.
    pub disabled: bool,
}

impl SelectOption<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("icon", PropKind::Value, "icon: impl Into<Glyph<'a>>")
            .doc("An icon before the label."),
        Prop::new("body", PropKind::Value, "content: Markup")
            .doc("Rich markup instead of the text (with `Caps::BaseSelect` only)."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("Shown, but cannot be picked."),
    ];
}

impl<'a> SelectOption<'a> {
    /// An option with a plain label.
    pub const fn new(value: &'a str, text: &'a str) -> Self {
        SelectOption {
            value,
            text,
            icon: None,
            content: None,
            disabled: false,
        }
    }
    /// An icon before the label; a plain `<select>` shows only a text glyph.
    pub fn icon(mut self, icon: impl Into<Glyph<'a>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    /// Rich markup instead of the text (with `Caps::BaseSelect` only).
    pub fn body(mut self, content: Markup) -> Self {
        self.content = Some(content);
        self
    }
    /// Shown, but cannot be picked (`disabled`): sold out, not in your plan.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// `("m", "Medium")`: a value and its label.
impl<'a> From<(&'a str, &'a str)> for SelectOption<'a> {
    fn from((value, text): (&'a str, &'a str)) -> Self {
        SelectOption::new(value, text)
    }
}

/// `("m", "Medium", "🐕")`: a value, its label and an icon.
impl<'a> From<(&'a str, &'a str, &'a str)> for SelectOption<'a> {
    fn from((value, text, icon): (&'a str, &'a str, &'a str)) -> Self {
        SelectOption::new(value, text).icon(icon)
    }
}

/// A form's `&str` or `(value, label)` choice.
impl<'a> From<crate::input::Choice<'a>> for SelectOption<'a> {
    fn from(c: crate::input::Choice<'a>) -> Self {
        SelectOption::new(c.value, c.label)
    }
}

/// Options under an `<optgroup label>`, or loose when `label` is `None`.
#[derive(Clone, Debug)]
struct Group<'a> {
    label: Option<&'a str>,
    options: Vec<SelectOption<'a>>,
}

/// A select, made by [`Ui::select`]; a form's `.select(..)` field is one too. No filter box
/// unless [`Select::search`] asks for one.
///
/// **Setters.** Values and items: `.options(..)`, `.group(..)`, `.groups(..)`, `.search(..)`,
/// `.search_over(..)`, `.value(..)`, `.error(..)`, `.help(..)`, `.placeholder(..)`,
/// `.size(..)`, `.variant(..)`; switches: `.required()`, `.disabled()`, `.readonly()`,
/// `.multiple()`, `.hide_label()`.
#[derive(Clone, Debug)]
pub struct Select<'a> {
    caps: Caps,
    strings: &'static Strings,
    /// The filter box's text (the query's `<name>-q`).
    filter: &'a str,
    name: &'a str,
    label: &'a str,
    id: Option<&'a str>,
    pub(crate) selected: Vec<&'a str>,
    /// Whether `.value(..)` replaced the query's picks yet.
    by_hand: bool,
    groups: Vec<Group<'a>>,
    search: Option<&'a str>,
    search_over: usize,
    pub(crate) error: Option<&'a str>,
    pub(crate) help: Option<&'a str>,
    pub(crate) required: bool,
    pub(crate) optional: bool,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
    pub(crate) multiple: bool,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) hide_label: bool,
    pub(crate) autocomplete: Option<&'a str>,
    pub(crate) autofocus: bool,
    pub(crate) form: Option<&'a str>,
    pub(crate) class: Option<&'a str>,
    pub(crate) size: Size,
    pub(crate) variant: Option<Variant>,
}

impl Select<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new("Tab / Shift+Tab", "Moves focus to the select or past it."),
        Key::new("Space / Alt+ArrowDown", "Opens the list of options."),
        Key::new(
            "ArrowUp / ArrowDown",
            "Moves to the previous or next option.",
        ),
        Key::new("Home / End", "Moves to the first or last option."),
        Key::new("Enter", "Picks the highlighted option and closes the list."),
        Key::new("Escape", "Closes the list without changing the choice."),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new(
            "options",
            PropKind::Value,
            "options: impl IntoIterator<Item = O>",
        )
        .doc("Options with no `<optgroup>`."),
        Prop::new(
            "group",
            PropKind::Value,
            "label: &'a str, options: impl IntoIterator<Item = O>",
        )
        .doc("Options under `<optgroup label>`."),
        Prop::new(
            "groups",
            PropKind::Value,
            "groups: impl IntoIterator<Item = (&'a str, G)>",
        )
        .doc("Several labelled groups at once."),
        Prop::new("search", PropKind::Value, "action: &'a str")
            .doc("A filter box submitting `<name>-q` to `action` with GET."),
        Prop::new("search_over", PropKind::Number, "n: usize")
            .default("15")
            .doc("Show the filter box above this many options (default 15)."),
        Prop::new("value", PropKind::Value, "value: &'a str")
            .doc("The chosen option's value, instead of the query's `name` (once more per pick with `.multiple()`)."),
        Prop::new("error", PropKind::Value, "message: &'a str")
            .doc("A server message under the select, which gets `aria-invalid`."),
        Prop::new("help", PropKind::Value, "help: &'a str").doc("Help text under the select."),
        Prop::new("placeholder", PropKind::Value, "placeholder: &'a str")
            .doc("A first option posting nothing (\"Choose…\"), picked until another is."),
        Prop::new("required", PropKind::Switch, "")
            .attr("required")
            .doc("An option other than the placeholder must be picked."),
        Prop::new("disabled", PropKind::Switch, "")
            .attr("disabled")
            .doc("`disabled` and `aria-disabled`: shown greyed and not posted."),
        Prop::new("readonly", PropKind::Switch, "")
            .doc("Shown and posted, not changeable: a disabled select and a hidden input per picked value."),
        Prop::new("multiple", PropKind::Switch, "")
            .attr("multiple")
            .doc("Several options may be picked (a list box; no rich trigger)."),
        Prop::new("hide_label", PropKind::Switch, "")
            .doc("Only the select, the label kept as its `aria-label`."),
        Prop::new(
            "size",
            PropKind::Choice { of: Size::NAMES },
            "size: impl Into<Size>",
        )
        .default("2")
        .doc("The trigger's height and text: Radix's sizes 1 to 3."),
        Prop::new(
            "variant",
            PropKind::Choice { of: Variant::NAMES },
            "variant: Variant",
        )
        .default("surface")
        .doc("The trigger's look: surface (a border), soft (a tint) or ghost (neither)."),
    ];
}

impl Ui {
    /// A select named `name` under the label `label`, in a `div.lui-field` like a form field
    /// (its id is `f-<name>`); the chosen option is the query's `name` unless
    /// [`Select::value`] says otherwise. Add options with [`Select::options`] or
    /// [`Select::group`].
    pub fn select<'a>(&'a self, name: &'a str, label: &'a str) -> Select<'a> {
        let mut s = Select::bare(self.caps, self.strings, name, label);
        s.selected = self.params(name).collect();
        s.filter = self.param(&format!("{name}-q")).unwrap_or("");
        s
    }
}

impl<'a> Select<'a> {
    /// A select with nothing read from a request: what a form's select field starts from.
    pub(crate) fn bare(
        caps: Caps,
        strings: &'static Strings,
        name: &'a str,
        label: &'a str,
    ) -> Self {
        Select {
            caps,
            strings,
            filter: "",
            name,
            label,
            id: None,
            selected: Vec::new(),
            by_hand: false,
            groups: Vec::new(),
            search: None,
            search_over: 15,
            error: None,
            help: None,
            required: false,
            optional: false,
            disabled: false,
            readonly: false,
            multiple: false,
            placeholder: None,
            hide_label: false,
            autocomplete: None,
            autofocus: false,
            form: None,
            class: None,
            size: Size::S2,
            variant: None,
        }
    }

    /// The control's id, `f-<name>` unless `id` says otherwise (a form field's `.id(..)`).
    pub(crate) fn id_from(mut self, id: Option<&'a str>) -> Self {
        self.id = id;
        self
    }

    /// Options with no `<optgroup>`: [`SelectOption`]s or `(value, label)` and
    /// `(value, label, icon)` tuples.
    pub fn options<O: Into<SelectOption<'a>>>(
        mut self,
        options: impl IntoIterator<Item = O>,
    ) -> Self {
        self.groups.push(Group {
            label: None,
            options: options.into_iter().map(Into::into).collect(),
        });
        self
    }

    /// Options under `<optgroup label>`.
    pub fn group<O: Into<SelectOption<'a>>>(
        mut self,
        label: &'a str,
        options: impl IntoIterator<Item = O>,
    ) -> Self {
        self.groups.push(Group {
            label: Some(label),
            options: options.into_iter().map(Into::into).collect(),
        });
        self
    }

    /// Several labelled groups at once: `(label, options)` pairs.
    pub fn groups<O: Into<SelectOption<'a>>, G: IntoIterator<Item = O>>(
        self,
        groups: impl IntoIterator<Item = (&'a str, G)>,
    ) -> Self {
        groups
            .into_iter()
            .fold(self, |s, (label, options)| s.group(label, options))
    }

    /// A filter box submitting `<name>-q` to `action` with GET; the options shown are those
    /// matching the `<name>-q` in this request's query, and always the selected one.
    pub fn search(mut self, action: &'a str) -> Self {
        self.search = Some(action);
        self
    }

    /// Show the filter box above this many options (default 15).
    pub fn search_over(mut self, n: usize) -> Self {
        self.search_over = n;
        self
    }

    /// The chosen option's value, instead of the query's `name`: a saved value. With
    /// `.multiple()` (called first), each call picks one more.
    pub fn value(mut self, value: &'a str) -> Self {
        if !self.by_hand || !self.multiple {
            self.selected.clear();
        }
        self.by_hand = true;
        self.selected.push(value);
        self
    }

    /// A server message under the select, which gets `aria-invalid`; the error summary links
    /// to it as it does to a form field. An empty message is no error.
    pub fn error(mut self, message: &'a str) -> Self {
        self.error = (!message.is_empty()).then_some(message);
        self
    }

    /// Help text under the select.
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// A first option posting nothing (`"Choose…"`), picked until another is; with
    /// `.required()` the browser asks for a real choice.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// An option other than the placeholder must be picked, and a `*` after the label.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// `disabled` and `aria-disabled`: shown greyed and not posted.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Shown and posted, not changeable. HTML has no read-only select, so the select is
    /// disabled and a hidden input posts each picked value.
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    /// Several options may be picked: a list box posting `name` once per pick. A multiple
    /// select keeps the native list look (`base-select` has no multiple mode yet).
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// Only the select, the label kept as its `aria-label`: a sort order in a toolbar. Help
    /// and error still follow it.
    pub fn hide_label(mut self) -> Self {
        self.hide_label = true;
        self
    }

    /// The trigger's height and text, Radix's sizes 1 to 3 (2 is the default).
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// The trigger's look: surface (the default, a border), soft (a brand tint and no
    /// border) or ghost (neither); solid and outline draw as surface.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }
}

impl Render for Select<'_> {
    fn render(&self) -> Markup {
        let name = self.name;
        let id = self
            .id
            .map_or_else(|| crate::field_id(name), str::to_string);
        let error_id = format!("{id}-error");
        let rich = self.caps.has(Cap::BaseSelect) && !self.multiple;
        let total: usize = self.groups.iter().map(|g| g.options.len()).sum();
        let search = self.search.filter(|_| total > self.search_over);
        let query_name = format!("{name}-q");
        let query_id = format!("{name}-filter");
        let query = if search.is_some() {
            self.filter.trim().to_lowercase()
        } else {
            String::new()
        };
        let chosen = |v: &str| self.selected.contains(&v);
        let shown = |o: &SelectOption| {
            query.is_empty() || chosen(o.value) || o.text.to_lowercase().contains(&query)
        };
        let option = |o: &SelectOption| {
            html! {
                option value=(o.value) selected[chosen(o.value)] disabled[o.disabled] {
                    @if let Some(i) = o.icon.filter(|i| rich || matches!(i, Glyph::Text(_))) { span class="lui-select-icon" aria-hidden="true" { (i) } " " }
                    @match (&o.content, rich) { (Some(c), true) => (c), _ => (o.text) }
                }
            }
        };
        let described = described_by(&[
            self.help.map(|_| format!("{id}-help")),
            self.error.map(|_| error_id.clone()),
        ]);
        let off = self.disabled || self.readonly;
        let nothing = self.selected.iter().all(|v| v.is_empty());
        let t = |text| self.strings.get(text);
        let size = (self.size != Size::S2).then(|| self.size.as_str());
        let control = html! {
            span class="lui-select" data-size=[size] data-variant=[self.variant.map(Variant::as_str)] {
                @if let Some(action) = search {
                    span class="lui-select-search" {
                        (Input::search_box(&query_name, t(Text::FilterOptions), self.filter).id(&query_id).placeholder(t(Text::Filter)).class("lui-select-filter"))
                        (crate::button::Button::new(self.caps, t(Text::Filter)).formmethod("get").formaction(action).formnovalidate())
                    }
                }
                select id=(id) name=(name) class=[self.class] required[self.required] multiple[self.multiple]
                    disabled[off] aria-disabled=[self.disabled.then_some("true")]
                    form=[self.form] autocomplete=[self.autocomplete] autofocus[self.autofocus]
                    aria-label=[self.hide_label.then_some(self.label)]
                    aria-invalid=[self.error.map(|_| "true")] aria-describedby=[described.as_deref()] {
                    @if rich { button type="button" class="lui-select-trigger" { selectedcontent {} } }
                    @if let Some(p) = self.placeholder { option value="" selected[nothing] { (p) } }
                    @for g in &self.groups {
                        @let visible: Vec<&SelectOption> = g.options.iter().filter(|o| shown(o)).collect();
                        @if let (Some(label), false) = (g.label, visible.is_empty()) {
                            optgroup label=(label) { @for o in &visible { (option(o)) } }
                        } @else {
                            @for o in &visible { (option(o)) }
                        }
                    }
                }
                @if self.readonly { @for v in self.selected.iter().filter(|v| !v.is_empty()) { (keep(name, v)) } }
            }
            @if let Some(h) = self.help { small id={ (id) "-help" } class="lui-field-help" { (h) } }
            @if let Some(e) = self.error { (crate::input::error_line(&error_id, e)) }
        };
        if self.hide_label {
            return control;
        }
        html! {
            div class="lui-field" {
                label for=(id) {
                    (self.label) @if self.required { " " (crate::input::REQUIRED) }
                    @if self.optional && !self.required { " " span class="lui-field-optional" { (t(Text::Optional)) } }
                }
                (control)
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Select. The trigger is the field look from input.rs at the control
   height, the value and a chevron. With appearance: base-select the picker is Radix's
   content panel: 2rem rows with a check in a 1.5rem left gutter, the primary colour on the
   highlighted row, group labels in --lui-gray-10. A native <select> keeps the gradient chevron. */
.lui-select { display: inline-grid; gap: var(--lui-space-2); max-width: 100%; }
.lui-select-search { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); }
.lui-select-filter { flex: 1; min-width: min(10rem, 100%); }
.lui-select select, .lui-select select::picker(select) { appearance: base-select; }
.lui-select select { min-width: min(12rem, 100%); }
/* .size(..) and .variant(..) on the trigger: Radix's sizes 1 and 3, its soft and ghost looks. */
.lui-select[data-size="1"] select { min-height: var(--lui-control-h-sm); padding-block: 0.25rem; font-size: var(--lui-text-2); }
.lui-select[data-size="3"] select { min-height: var(--lui-control-h-lg); padding-block: 0.5rem; font-size: var(--lui-text-3); line-height: 1.5rem; }
.lui-select[data-variant=soft] select { background-color: var(--lui-brand-3); border-color: transparent; }
.lui-select[data-variant=ghost] select { background-color: transparent; border-color: transparent; }
/* base-select draws its own ::picker-icon; drop the gradient chevron from input.rs. */
@supports (appearance: base-select) {
  .lui-select select { display: flex; align-items: center; gap: var(--lui-space-2); background-image: none; padding-inline-end: 0.75rem; }
  .lui-select-trigger { display: contents; }
}
.lui-select select::picker-icon { margin-inline-start: auto; color: var(--lui-muted); transition: rotate var(--lui-duration-fast); }
.lui-select select:open::picker-icon { rotate: 180deg; }
.lui-select select::picker(select) {
  margin-block: var(--lui-space-1); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); padding: var(--lui-space-1);
  background: var(--lui-popover); color: var(--lui-fg); box-shadow: var(--lui-shadow-md), var(--lui-highlight);
  max-height: 20rem;
}
.lui-select option {
  position: relative; display: flex; align-items: center; gap: var(--lui-space-2); min-height: 2rem; box-sizing: border-box;
  padding: 0 0.75rem 0 1.5rem; border-radius: var(--lui-radius-sm); font-size: var(--lui-text-2); cursor: default;
}
.lui-select option:hover, .lui-select option:focus-visible { background: var(--lui-primary); color: var(--lui-on-primary); outline: none; }
.lui-select option::checkmark { position: absolute; inset-inline-start: 0.5rem; }
.lui-select optgroup { font-size: var(--lui-text-1); font-weight: 500; color: var(--lui-gray-10); padding-top: var(--lui-space-2); }
.lui-select optgroup + optgroup { border-top: 1px solid var(--lui-line); margin-top: var(--lui-space-1); }
.lui-select optgroup option { font-weight: 400; color: var(--lui-fg); }
.lui-select optgroup option:hover, .lui-select optgroup option:focus-visible { color: var(--lui-on-primary); }
@media (pointer: coarse) { .lui-select option { min-height: var(--lui-hit); } }
.lui-select-icon { display: inline-block; width: 1.25em; text-align: center; }
.lui-swatch { display: inline-block; width: 1em; height: 1em; border-radius: 50%; vertical-align: -0.15em; margin-inline-end: 0.4em; border: 1px solid var(--lui-line); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn sizes() -> [(&'static str, &'static str); 2] {
        [("s", "Small"), ("l", "Large")]
    }

    #[test]
    fn the_rich_trigger_and_option_content_need_base_select_and_the_plain_select_has_neither() {
        let rich = Ui::from(Caps::all())
            .select("size", "Size")
            .options(sizes())
            .render()
            .into_string();
        assert!(
            rich.contains("lui-select-trigger") && rich.contains("<selectedcontent>"),
            "{rich}"
        );
        let plain = Ui::from(Caps::NONE)
            .select("size", "Size")
            .options(sizes())
            .render()
            .into_string();
        assert!(
            !plain.contains("selectedcontent") && !plain.contains("lui-select-trigger"),
            "{plain}"
        );
        assert!(
            plain.contains(r#"<label for="f-size">Size</label>"#)
                && plain.contains("<option value=\"s\">Small</option>"),
            "{plain}"
        );
    }

    #[test]
    fn rich_option_content_replaces_the_text_only_with_base_select() {
        let opts = [SelectOption::new("k", "Kiwi").body(html! { b { "Kiwi!" } })];
        let rich = Ui::from(Caps::all())
            .select("f", "Fruit")
            .options(opts.clone())
            .render()
            .into_string();
        assert!(rich.contains("<b>Kiwi!</b>"), "{rich}");
        let plain = Ui::from(Caps::NONE)
            .select("f", "Fruit")
            .options(opts)
            .render()
            .into_string();
        assert!(
            plain.contains(">Kiwi</option>") && !plain.contains("<b>"),
            "{plain}"
        );
    }

    #[test]
    fn the_value_or_query_marks_the_selected_option_and_groups_become_optgroups() {
        let ui = Ui::from(Caps::NONE);
        let m = ui
            .select("size", "Size")
            .options(sizes())
            .value("l")
            .render()
            .into_string();
        assert!(
            m.contains(r#"<option value="l" selected>"#)
                && !m.contains(r#"<option value="s" selected>"#),
            "{m}"
        );
        let q = Ui::from_request("/", "size=s", "");
        let m = q
            .select("size", "Size")
            .options(sizes())
            .render()
            .into_string();
        assert!(m.contains(r#"<option value="s" selected>"#), "{m}");
        let ui = Ui::from(Caps::NONE);
        let m = ui
            .select("f", "Fruit")
            .placeholder("Pick")
            .group("Citrus", [("o", "Orange")])
            .render()
            .into_string();
        assert!(
            m.contains(r#"<option value="" selected>Pick</option>"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<optgroup label="Citrus"><option value="o">Orange</option></optgroup>"#),
            "{m}"
        );
        let m = ui
            .select("f", "Fruit")
            .placeholder("Pick")
            .options([("o", "Orange")])
            .value("o")
            .render()
            .into_string();
        assert!(m.contains(r#"<option value="">Pick</option>"#), "{m}");
    }

    #[test]
    fn the_filter_box_appears_over_the_limit_and_keeps_matches_and_the_selection() {
        let ui = Ui::from_request("/shop", "f=pear&f-q=ap", "");
        let opts = [("apple", "Apple"), ("pear", "Pear"), ("fig", "Fig")];
        let m = ui
            .select("f", "Fruit")
            .options(opts)
            .search("/shop")
            .search_over(2)
            .render()
            .into_string();
        assert!(
            m.contains("lui-select-search") && m.contains(r#"formaction="/shop""#),
            "{m}"
        );
        assert!(
            m.contains(">Apple<") && m.contains(">Pear<") && !m.contains(">Fig<"),
            "{m}"
        );
        let few = ui
            .select("f", "Fruit")
            .options(opts)
            .search("/shop")
            .search_over(3)
            .render()
            .into_string();
        assert!(
            !few.contains("lui-select-search") && few.contains(">Fig<"),
            "{few}"
        );
    }

    #[test]
    fn error_help_hidden_label_readonly_and_multiple_shape_the_field() {
        let ui = Ui::from(Caps::NONE);
        let m = ui
            .select("size", "Size")
            .options(sizes())
            .help("Hint")
            .error("Bad")
            .required()
            .render()
            .into_string();
        assert!(
            m.contains(r#"aria-invalid="true""#)
                && m.contains("f-size-help")
                && m.contains("f-size-error"),
            "{m}"
        );
        assert!(
            m.contains("Hint") && m.contains("Bad") && m.contains("required"),
            "{m}"
        );
        let bare = ui
            .select("size", "Size")
            .options(sizes())
            .hide_label()
            .render()
            .into_string();
        assert!(
            bare.contains(r#"aria-label="Size""#)
                && !bare.contains("<label")
                && !bare.contains("lui-field\""),
            "{bare}"
        );
    }

    #[test]
    fn readonly_posts_hidden_values_and_multiple_drops_the_rich_trigger() {
        let ui = Ui::from(Caps::all());
        let ro = ui
            .select("size", "Size")
            .options(sizes())
            .value("l")
            .readonly()
            .render()
            .into_string();
        assert!(
            ro.contains("disabled")
                && ro.contains(r#"type="hidden""#)
                && ro.contains(r#"value="l""#),
            "{ro}"
        );
        let multi = ui
            .select("size", "Size")
            .options(sizes())
            .multiple()
            .render()
            .into_string();
        assert!(
            multi.contains("multiple") && !multi.contains("selectedcontent"),
            "{multi}"
        );
    }
}
