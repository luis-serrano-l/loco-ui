//! # Drawer
//!
//! Site navigation that is a sidebar on a wide screen and a drawer sliding in from the edge on
//! a narrow one, from the same markup: a menu button opens it, Escape, a click outside or the
//! close button shuts it. No script.
//!
//! The look follows the shadcn Sheet: a header (title, `.description(..)`, the close ×), the
//! navigation scrolling in the middle and a `.footer(..)` pinned to the bottom. Under a 30rem
//! viewport it is the shadcn Drawer instead: the same `<dialog>` rises from the bottom with a
//! grab handle and rounded top corners, at most 85vh tall.
//!
//! **Platform features:**
//! - `<dialog>` opened as a modal by an invoker button, `command="show-modal"` (Chrome 135+,
//!   Firefox 144+, Safari 26.2+), closed by `command="close"` and by `closedby="any"` for
//!   Escape and light dismiss.
//! - `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) for the slide in, and
//!   `transition-behavior: allow-discrete` on `display` and `overlay` (Chrome 117,
//!   Firefox 129, Safari 17.4) for the slide out, the backdrop fading with it; off under
//!   `prefers-reduced-motion`.
//! - Sidebar mode: above 60rem a `@media` rule shows the closed `<dialog>` in a grid column
//!   and hides the menu button, so desktop gets a permanent sidebar with no request.
//!
//! **Fallback:** without `Caps` `Invokers`, the menu button is a link to `#id` and a `:target`
//! rule shows the drawer; its close control is a link to `#`. `?dialog=<id>` in the URL (or
//! `.open(true)`) renders it open from the server.
//!
//! **Accessibility:** a native modal `<dialog>` named by its title: focus moves in, Escape
//! closes it. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** swipe to close; focus is not trapped in the
//! `:target` fallback.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let nav = html! { ul { li { a href="/" { "Home" } } } };
//! // The id is the label's slug: this drawer is `#menu`.
//! let m = ui.drawer("Menu").nav(nav.clone()).body(html! { p { "Page" } }).render().into_string();
//! assert!(m.contains(r#"command="show-modal" commandfor="menu""#) && m.contains(r#"closedby="any""#));
//! // `?dialog=menu` opens it from the server.
//! let ui = Ui::from_request("/", "dialog=menu", "");
//! let m = ui.drawer("Menu").title("Browse").sidebar().nav(nav.clone()).render().into_string();
//! assert!(m.contains("lui-drawer-sidebar") && m.contains(" open>"));
//! // The same in `lui!`:
//! let same = lui! { Drawer("Menu") title="Browse" sidebar nav=(nav); };
//! assert_eq!(same.into_string(), m);
//! // A description under the title and a footer under the navigation.
//! let m = ui.drawer("Menu").description("Everything in the app.").footer(html! { a href="/signout" { "Sign out" } });
//! let m = m.render().into_string();
//! assert!(m.contains(r#"aria-describedby="menu-description""#) && m.contains(r#"<div class="lui-drawer-foot">"#));
//! let same = lui! { Drawer("Menu") description="Everything in the app." footer={ a href="/signout" { "Sign out" } }; };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::{Cap, Icon, Ui, slug};

/// Navigation in a drawer beside the page's content, made by [`Ui::drawer`].
///
/// **Setters.** Values and items: `.nav(..)`, `.body(..)`, `.id(..)`, `.title(..)`,
/// `.description(..)`, `.footer(..)`; switches:
/// `.sidebar()`; from a condition: `.open(bool)`.
#[derive(Clone, Debug)]
pub struct Drawer<'a> {
    ui: &'a Ui,
    id: String,
    label: &'a str,
    nav: Markup,
    body: Markup,
    title: Option<&'a str>,
    description: Option<&'a str>,
    footer: Option<Markup>,
    sidebar: bool,
    open: bool,
}

impl Drawer<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("nav", PropKind::Value, "nav: Markup")
            .doc("The navigation inside the drawer, usually a `ul` of links."),
        Prop::new("body", PropKind::Value, "body: Markup")
            .doc("The page's content, beside the drawer."),
        Prop::new("id", PropKind::Value, "id: &str")
            .attr("id")
            .doc("The drawer's id instead of the label's slug."),
        Prop::new("title", PropKind::Value, "title: &'a str")
            .doc("A heading at the top of the panel (also its accessible name)."),
        Prop::new("description", PropKind::Value, "text: &'a str")
            .doc("A line under the title, the panel's description."),
        Prop::new("footer", PropKind::Value, "footer: Markup")
            .doc("Actions pinned to the bottom of the panel, under the scrolling navigation."),
        Prop::new("sidebar", PropKind::Switch, "").doc("A permanent sidebar above 60rem."),
        Prop::new("open", PropKind::Condition, "open: bool")
            .attr("open")
            .doc("Render it open (non-modal) from the server."),
    ];
}

impl Ui {
    /// A drawer opened by a button labelled `label`; its id is the label's slug, and
    /// `?dialog=<id>` renders it open.
    pub fn drawer<'a>(&'a self, label: &'a str) -> Drawer<'a> {
        Drawer {
            ui: self,
            id: slug(label),
            label,
            nav: Markup::default(),
            body: Markup::default(),
            title: None,
            description: None,
            footer: None,
            sidebar: false,
            open: false,
        }
    }
}

impl<'a> Drawer<'a> {
    /// The navigation inside the drawer, usually a `ul` of links.
    pub fn nav(mut self, nav: Markup) -> Self {
        self.nav = nav;
        self
    }

    /// The page's content, beside the drawer.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }

    /// The drawer's id instead of the label's slug.
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    /// A heading at the top of the panel (also its accessible name); the label by default.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// A line under the title, the panel's description (`aria-describedby`).
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// Actions pinned to the bottom of the panel (a sign-out button, a settings link), under
    /// the navigation, which scrolls between header and footer.
    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }

    /// A permanent sidebar above 60rem; a drawer below.
    pub fn sidebar(mut self) -> Self {
        self.sidebar = true;
        self
    }

    /// Render it open (non-modal) from the server.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Drawer<'_> {
    fn render(&self) -> Markup {
        let Drawer {
            ui,
            ref id,
            label,
            ref nav,
            ref body,
            title,
            description,
            ref footer,
            sidebar,
            open,
        } = *self;
        let desc_id = format!("{id}-description");
        let invokers = ui.has(Cap::Invokers);
        let open = open || ui.state.dialog() == Some(id);
        let title = title.unwrap_or(label);
        let title_id = format!("{id}-title");
        let open_href = format!("#{id}");
        html! {
            div class={ "lui-drawer" @if sidebar { " lui-drawer-sidebar" } } {
                @let menu = html! { (Icon::Menu) (label) };
                @if invokers {
                    (ui.button(label).class("lui-drawer-open").body(menu).command("show-modal", id).aria_haspopup("dialog"))
                } @else {
                    (ui.link_button(label, &open_href).class("lui-drawer-open").body(menu).role("button"))
                }
                dialog id=(id) class="lui-drawer-panel" closedby="any" aria-labelledby=(title_id)
                    aria-describedby=[description.map(|_| &desc_id)] open[open] {
                    div class="lui-drawer-head" {
                        p id=(title_id) class="lui-drawer-title" { (title) }
                        @if let Some(d) = description { p id=(desc_id) class="lui-drawer-description" { (d) } }
                        @let x = html! { (Icon::X) };
                        @if invokers {
                            (ui.button("").ghost().small().icon_only().class("lui-drawer-close").aria_label(ui.text(Text::Close)).body(x).command("close", id))
                        } @else {
                            (ui.link_button("", "#").ghost().small().icon_only().class("lui-drawer-close").aria_label(ui.text(Text::Close)).body(x))
                        }
                    }
                    nav class="lui-drawer-nav" aria-labelledby=(title_id) { (nav) }
                    @if let Some(f) = footer { div class="lui-drawer-foot" { (f) } }
                }
                div class="lui-drawer-content" { (body) }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-drawer { display: grid; gap: var(--lui-space-4); }
.lui-drawer-open { justify-self: start; }
/* After the shadcn Sheet (side="left"): full height, 24rem at most, a border on the open edge,
   shadow-lg; a header (title, description, the close ×), the navigation scrolling in the
   middle, and a footer pinned to the bottom. Under a 30rem viewport it is the shadcn Drawer:
   the same <dialog> rises from the bottom with rounded top corners and a grab handle, at
   most 85vh tall. */
.lui-drawer-panel {
  box-sizing: border-box; margin: 0; padding: var(--lui-space-4);
  color: var(--lui-fg); background: var(--lui-popover); border: 0; border-inline-end: 1px solid var(--lui-line);
}
.lui-drawer-panel:modal, .lui-drawer-panel:target {
  display: flex; flex-direction: column; position: fixed; inset: 0 auto 0 0; height: 100dvh; max-height: none; width: min(24rem, 85vw); z-index: 10;
  padding: var(--lui-space-6); box-shadow: var(--lui-shadow-lg), var(--lui-highlight);
}
.lui-drawer-panel:target { box-shadow: var(--lui-shadow-lg), var(--lui-highlight), 0 0 0 100vmax var(--lui-overlay); }
.lui-drawer-panel::backdrop { background: var(--lui-overlay); }
.lui-drawer-panel:not(:modal):not(:target)[open] { position: static; width: auto; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg); }
.lui-drawer-head { flex: none; display: grid; grid-template-columns: minmax(0, 1fr) auto; column-gap: var(--lui-space-2); align-items: center; margin-bottom: var(--lui-space-4); }
.lui-drawer-title { margin: 0; font-size: 1rem; line-height: 1.5rem; font-weight: 600; }
.lui-drawer-description { grid-column: 1; margin: 0; font-size: 0.875rem; line-height: 1.25rem; color: var(--lui-muted); }
.lui-drawer-close { grid-column: 2; grid-row: 1; opacity: 0.7; }
.lui-drawer-close:hover { opacity: 1; }
.lui-drawer-nav { flex: 1 1 auto; min-height: 0; overflow-y: auto; margin-inline: calc(var(--lui-space-2) * -1); padding-inline: var(--lui-space-2); }
.lui-drawer-foot { flex: none; display: flex; flex-wrap: wrap; gap: var(--lui-space-2); margin-top: auto; padding-top: var(--lui-space-4); }
/* Links as shadcn sidebar menu buttons: text-sm, rounded-md, accent on hover and when current. */
.lui-drawer-panel ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 0.125rem; }
.lui-drawer-panel li a {
  display: flex; align-items: center; min-height: 2rem; padding: 0 0.5rem; border-radius: var(--lui-radius-sm);
  font-size: 0.875rem; line-height: 1.25rem; color: var(--lui-fg); text-decoration: none;
}
@media (pointer: coarse) { .lui-drawer-panel li a { min-height: var(--lui-hit); } }
.lui-drawer-panel li a:hover { background: var(--lui-accent); color: var(--lui-on-accent); }
.lui-drawer-panel li a[aria-current] { background: var(--lui-accent); color: var(--lui-on-accent); font-weight: 500; }
.lui-drawer-content { min-width: 0; }
/* The bottom sheet: a phone-width viewport. */
@media (max-width: 30rem) {
  .lui-drawer-panel:modal, .lui-drawer-panel:target {
    inset: auto 0 0 0; width: 100%; height: auto; max-height: 85vh; padding-top: var(--lui-space-8);
    border: 0; border-top: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg) var(--lui-radius-lg) 0 0;
  }
  .lui-drawer-panel:modal::before, .lui-drawer-panel:target::before {
    content: ""; position: absolute; top: var(--lui-space-3); left: 50%; translate: -50% 0;
    width: 3rem; height: 0.375rem; border-radius: 9999px; background: var(--lui-gray-5);
  }
}
@media (min-width: 60rem) {
  .lui-drawer-sidebar { grid-template-columns: 14rem 1fr; align-items: start; }
  .lui-drawer-sidebar > .lui-drawer-open { display: none; }
  .lui-drawer-sidebar > .lui-drawer-panel:not(:modal) {
    display: flex; flex-direction: column; position: sticky; top: var(--lui-space-4); width: auto; height: auto; box-shadow: none;
    padding: var(--lui-space-2); background: var(--lui-surface);
    border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg); z-index: auto;
  }
  .lui-drawer-sidebar .lui-drawer-close { display: none; }
}
/* Motion: the sheet slides out the way it came in (from the side, or from the bottom on a
   phone), the backdrop fading with it. The base rule holds the transition so it still runs
   once :modal stops matching; display and overlay are discrete so the closing sheet keeps its
   top-layer box until the slide ends. The wide-screen sidebar never slides. */
.lui-drawer-panel, .lui-drawer-panel:modal {
  --lui-drawer-away: -100% 0;
  transition: translate var(--lui-duration-slow) var(--lui-ease-out),
    display var(--lui-duration-slow) allow-discrete, overlay var(--lui-duration-slow) allow-discrete;
}
@media (max-width: 30rem) { .lui-drawer-panel, .lui-drawer-panel:modal { --lui-drawer-away: 0 100%; } }
.lui-drawer-panel:not([open]):not(:target) { translate: var(--lui-drawer-away); }
@starting-style { .lui-drawer-panel:modal, .lui-drawer-panel:target { translate: var(--lui-drawer-away); } }
.lui-drawer-panel::backdrop {
  transition: opacity var(--lui-duration-slow) var(--lui-ease-out), display var(--lui-duration-slow) allow-discrete, overlay var(--lui-duration-slow) allow-discrete;
}
.lui-drawer-panel:not([open])::backdrop { opacity: 0; }
@starting-style { .lui-drawer-panel[open]::backdrop { opacity: 0; } }
@media (prefers-reduced-motion: reduce) { .lui-drawer-panel, .lui-drawer-panel:modal { transition: none; } }
@media (min-width: 60rem) { .lui-drawer-sidebar > .lui-drawer-panel:not(:modal) { translate: none; transition: none; } }
"#;
