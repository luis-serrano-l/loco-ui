//! # Toggle group
//!
//! A row of pressable options, one picked (alignment) or several (bold, italic), part of the
//! form around it. Each option is a real radio button or checkbox, drawn as a segmented
//! button, so the form posts the choice and the browser handles the keys.
//!
//! **Platform features:** `<fieldset>` of `<input type="radio">` (one) or
//! `type="checkbox"` (several) inside their labels; `:checked` and `:focus-visible` on the
//! input style the face next to it (every browser).
//!
//! **Accessibility:** a `<fieldset>` named by its visually hidden `<legend>`; arrow keys move
//! between radios and Space toggles a checkbox, as the platform does; an icon-only option has
//! its text as a hidden label. Checked by axe-core in headless Firefox on every demo route,
//! both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** apply a choice the moment it is pressed; the
//! surrounding form sends it (a submit button, or the enhancement script's in-place post).
//!
//! The look follows Radix Themes SegmentedControl: a gray track, items of one width, the
//! picked one a raised chip; with one pick the chip slides between items (CSS only, `:has()`
//! and a transition). A narrow container scrolls the row inside itself.
//!
//! **Fallback:** without `:has()` (Chrome < 105, Firefox < 121) the checked item takes the
//! chip look itself and nothing slides.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/editor", "align=center", "");
//! let m = ui.toggle_group("align", "Alignment").option("left", "Left").option("center", "Center");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"type="radio" name="align" value="center" checked"#));
//!
//! let m = ui.toggle_group("style", "Text style").multiple()
//!     .option("bold", "Bold").icon(Icon::Bold)
//!     .option("italic", "Italic").icon(Icon::Italic)
//!     .value("bold");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"type="checkbox" name="style" value="bold" checked"#));
//! // The same in `lui!`:
//! let same = lui! { ToggleGroup("style", "Text style") multiple {
//!     option "bold" "Bold" icon=(Icon::Bold);
//!     option "italic" "Italic" icon=(Icon::Italic);
//!     value "bold";
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::icon::Glyph;
use crate::props::{Prop, PropKind};

/// Pressable options, made by [`Ui::toggle_group`].
///
/// **Setters.** Values and items: `.option(..)`, `.icon(..)`, `.value(..)`; switches: `.multiple()`.
#[derive(Clone, Debug)]
pub struct ToggleGroup<'a> {
    name: &'a str,
    label: &'a str,
    items: Vec<(&'a str, &'a str, Option<Glyph<'a>>)>,
    picked: Vec<String>,
    multi: bool,
}

impl ToggleGroup<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("option", PropKind::Item, "value: &'a str, text: &'a str")
            .attr("value")
            .doc("An option posting `value`, shown as `text`."),
        Prop::new("icon", PropKind::Modifier, "icon: impl Into<Glyph<'a>>")
            .doc("The option added last shows this icon, its text becoming a hidden label."),
        Prop::new("value", PropKind::Value, "value: &'a str")
            .attr("checked")
            .doc("Press this option (again for more with `.multiple()`); the query's own values by default."),
        Prop::new("multiple", PropKind::Switch, "")
            .attr("type")
            .doc("Several options may be pressed (checkboxes, not radios)."),
    ];
}

impl Ui {
    /// Options posted as `name`, the group named `label` for assistive tech. The pressed ones
    /// start from the query's `name` values.
    pub fn toggle_group<'a>(&self, name: &'a str, label: &'a str) -> ToggleGroup<'a> {
        ToggleGroup {
            name,
            label,
            items: Vec::new(),
            picked: self.params(name).map(str::to_string).collect(),
            multi: false,
        }
    }
}

impl<'a> ToggleGroup<'a> {
    /// An option posting `value`, shown as `text`.
    pub fn option(mut self, value: &'a str, text: &'a str) -> Self {
        self.items.push((value, text, None));
        self
    }

    /// The old name of [`Self::option`], kept for one release.
    #[deprecated(note = "use .option()")]
    pub fn item(self, value: &'a str, text: &'a str) -> Self {
        self.option(value, text)
    }

    /// The option added last shows this icon, its text becoming a hidden label.
    pub fn icon(mut self, icon: impl Into<Glyph<'a>>) -> Self {
        if let Some(last) = self.items.last_mut() {
            last.2 = Some(icon.into());
        }
        self
    }

    /// Press this option (again for more with `.multiple()`); the query's own values by default.
    pub fn value(mut self, value: &'a str) -> Self {
        if !self.multi {
            self.picked.clear();
        }
        self.picked.push(value.to_string());
        self
    }

    /// Several options may be pressed (checkboxes, not radios).
    pub fn multiple(mut self) -> Self {
        self.multi = true;
        self
    }

    /// The old name of [`Self::multiple`], kept for one release.
    #[deprecated(note = "use .multiple()")]
    pub fn multi(self) -> Self {
        self.multiple()
    }
}

impl Render for ToggleGroup<'_> {
    fn render(&self) -> Markup {
        let kind = if self.multi { "checkbox" } else { "radio" };
        html! {
            fieldset class={ "lui-toggle-group" @if !self.multi { " lui-toggle-group-one" } }
                style={ "--lui-toggle-n: " (self.items.len().max(1)) } {
                legend class="lui-sr" { (self.label) }
                @for (value, text, icon) in &self.items {
                    label class="lui-toggle-group-item" {
                        input class="lui-toggle-group-input" type=(kind) name=(self.name) value=(value)
                            checked[self.picked.iter().any(|p| p == value)];
                        span class="lui-toggle-group-face" {
                            @if let Some(i) = icon { (i.hidden()) span class="lui-sr" { (text) } } @else { (text) }
                        }
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes SegmentedControl: a --lui-gray-3 track, items of one width, the picked one a
   raised card-coloured chip with a small shadow, highlight and a faint line-coloured edge
   (which keeps it visible in dark, where the card is barely lighter than the track). With one pick and :has(), the
   chip is one element that slides to the checked item (--lui-toggle-at, from its position
   among up to 8 items, over --lui-toggle-n set by the server), no script or view transition
   needed; otherwise the checked face takes the chip look itself. A narrow container
   scrolls the row inside itself. */
.lui-toggle-group {
  position: relative; display: inline-grid; grid-auto-flow: column; grid-auto-columns: 1fr;
  justify-self: start; align-self: start; max-width: 100%; box-sizing: border-box; overflow-x: auto;
  margin: 0; padding: 2px; border: 0; border-radius: var(--lui-radius); background: var(--lui-gray-3);
}
.lui-toggle-group-item { position: relative; z-index: 1; display: flex; }
.lui-toggle-group-input { position: absolute; opacity: 0; width: 1px; height: 1px; margin: 0; }
.lui-toggle-group-face {
  display: inline-flex; flex: 1; align-items: center; justify-content: center; gap: 0.375rem; white-space: nowrap;
  min-width: calc(var(--lui-control-h) - 4px); height: calc(var(--lui-control-h) - 4px); padding: 0 0.75rem; box-sizing: border-box;
  border-radius: calc(var(--lui-radius) - 2px); font-size: 0.875rem; font-weight: 500; color: var(--lui-gray-11); cursor: pointer;
  transition: color var(--lui-duration-fast), background-color var(--lui-duration-fast);
}
.lui-toggle-group-face:hover { color: var(--lui-fg); }
.lui-toggle-group-input:checked + .lui-toggle-group-face { color: var(--lui-fg); background: var(--lui-card); box-shadow: 0 0 0 1px color-mix(in srgb, var(--lui-line) 70%, transparent), var(--lui-shadow-xs), var(--lui-highlight); }
.lui-toggle-group-input:focus-visible + .lui-toggle-group-face { outline: 2px solid var(--lui-ring); outline-offset: -2px; }
@supports selector(:has(*)) {
  .lui-toggle-group-one .lui-toggle-group-input:checked + .lui-toggle-group-face { background: transparent; box-shadow: none; }
  .lui-toggle-group-one::before {
    content: ""; position: absolute; top: 2px; bottom: 2px; left: 2px; width: calc((100% - 4px) / var(--lui-toggle-n, 1));
    translate: calc(100% * var(--lui-toggle-at, 0)) 0; border-radius: calc(var(--lui-radius) - 2px);
    background: var(--lui-card); box-shadow: 0 0 0 1px color-mix(in srgb, var(--lui-line) 70%, transparent), var(--lui-shadow-xs), var(--lui-highlight);
    transition: translate var(--lui-duration) var(--lui-ease-spring);
  }
  .lui-toggle-group-one:not(:has(.lui-toggle-group-input:checked))::before { opacity: 0; }
  .lui-toggle-group-one:has(> .lui-toggle-group-item:nth-of-type(2) > .lui-toggle-group-input:checked) { --lui-toggle-at: 1; }
  .lui-toggle-group-one:has(> .lui-toggle-group-item:nth-of-type(3) > .lui-toggle-group-input:checked) { --lui-toggle-at: 2; }
  .lui-toggle-group-one:has(> .lui-toggle-group-item:nth-of-type(4) > .lui-toggle-group-input:checked) { --lui-toggle-at: 3; }
  .lui-toggle-group-one:has(> .lui-toggle-group-item:nth-of-type(5) > .lui-toggle-group-input:checked) { --lui-toggle-at: 4; }
  .lui-toggle-group-one:has(> .lui-toggle-group-item:nth-of-type(6) > .lui-toggle-group-input:checked) { --lui-toggle-at: 5; }
  .lui-toggle-group-one:has(> .lui-toggle-group-item:nth-of-type(7) > .lui-toggle-group-input:checked) { --lui-toggle-at: 6; }
  .lui-toggle-group-one:has(> .lui-toggle-group-item:nth-of-type(8) > .lui-toggle-group-input:checked) { --lui-toggle-at: 7; }
}
"#;
