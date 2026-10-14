//! # Collapsible
//!
//! One section that shows and hides under a button: "Show 3 more", advanced options, the
//! rest of a long message. The accordion's single cousin, with no group and no state in the
//! URL. After the shadcn/ui Collapsible.
//!
//! **Platform features:** one `<details>`/`<summary>` (baseline 2020) whose summary wears the
//! button look; `<details name>` (baseline 2024) makes several collapsibles exclusive.
//! `::details-content` (Chrome 131, Firefox 143, Safari 18.4) under `@supports`, with
//! `interpolate-size: allow-keywords` (Chrome 129), animates the height; elsewhere it snaps.
//!
//! **Accessibility:** a `<summary>` is a button with an expanded state; Enter or Space
//! toggles it; the chevron is `aria-hidden`. Checked by axe-core in headless Firefox on every
//! demo route, both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** without `::details-content` it opens and closes at once; without `name` the
//! sections are not exclusive. No `Caps` branch is needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.collapsible("Show 3 more", html! { p { "The rest." } }).render().into_string();
//! assert!(m.starts_with(r#"<details class="lui-collapsible"><summary class="lui-button lui-collapsible-trigger">"#));
//! // Open from the server, ghost, and one of a group where one opens at a time.
//! let m = ui.collapsible("Advanced", html! { p { "Options." } }).open(true).ghost().name("settings");
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<details class="lui-collapsible" name="settings" open>"#));
//! // The same in `lui!`:
//! let same = lui! { Collapsible("Advanced", html! { p { "Options." } }) open=(true) ghost name="settings"; };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::props::{Prop, PropKind};
use crate::{Icon, Ui};

/// A section under a button that shows and hides it, made by [`Ui::collapsible`].
///
/// **Setters.** Values and items: `.name(..)`; switches: `.ghost()`; conditions: `.open(..)`.
#[derive(Clone, Debug)]
pub struct Collapsible<'a> {
    trigger: &'a str,
    body: Markup,
    open: bool,
    ghost: bool,
    name: Option<&'a str>,
}

impl Collapsible<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("open", PropKind::Condition, "open: bool")
            .attr("open")
            .doc("Render it open."),
        Prop::new("ghost", PropKind::Switch, "")
            .doc("The ghost button look instead of the outline one."),
        Prop::new("name", PropKind::Value, "group: &'a str")
            .attr("name")
            .doc("Collapsibles with the same `name` are exclusive: opening one closes the others."),
    ];
}

impl Ui {
    /// `body` under a button reading `trigger`, closed until the button is pressed.
    pub fn collapsible<'a>(&self, trigger: &'a str, body: Markup) -> Collapsible<'a> {
        Collapsible {
            trigger,
            body,
            open: false,
            ghost: false,
            name: None,
        }
    }
}

impl<'a> Collapsible<'a> {
    /// Render it open (a route that knows the visitor wants it open, an error inside it).
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The ghost button look instead of the outline one.
    pub fn ghost(mut self) -> Self {
        self.ghost = true;
        self
    }

    /// Collapsibles with the same `name` are exclusive: opening one closes the others.
    pub fn name(mut self, group: &'a str) -> Self {
        self.name = Some(group);
        self
    }
}

impl Render for Collapsible<'_> {
    fn render(&self) -> Markup {
        let class = if self.ghost {
            "lui-button lui-button-ghost lui-collapsible-trigger"
        } else {
            "lui-button lui-collapsible-trigger"
        };
        html! {
            details class="lui-collapsible" name=[self.name] open[self.open] {
                summary class=(class) {
                    span { (self.trigger) } (Icon::ChevronDown)
                }
                div class="lui-collapsible-body" { (self.body) }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the shadcn/ui Collapsible: the trigger is an outline (or ghost) button with a
   chevron that turns when open; the content follows it 0.5rem below. Height animates where
   ::details-content and interpolate-size exist; elsewhere it snaps. */
.lui-collapsible { interpolate-size: allow-keywords; }
.lui-collapsible > summary { list-style: none; width: fit-content; max-width: 100%; }
.lui-collapsible > summary::-webkit-details-marker { display: none; }
.lui-collapsible > summary > span { min-width: 0; overflow: hidden; text-overflow: ellipsis; }
.lui-collapsible > summary > .lui-icon { color: var(--lui-muted); transition: rotate var(--lui-duration) var(--lui-ease-out); }
.lui-collapsible[open] > summary > .lui-icon { rotate: 180deg; }
.lui-collapsible-body { padding-top: var(--lui-space-2); }
@supports selector(::details-content) {
  .lui-collapsible::details-content { height: 0; overflow: clip; transition: height var(--lui-duration) var(--lui-ease-out), content-visibility var(--lui-duration) allow-discrete; }
  .lui-collapsible[open]::details-content { height: auto; }
}
@media (prefers-reduced-motion: reduce) {
  .lui-collapsible::details-content, .lui-collapsible > summary > .lui-icon { transition: none; }
}
@media (pointer: coarse) { .lui-collapsible > summary { min-height: var(--lui-hit); } }
"#;
