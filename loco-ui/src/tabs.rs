//! # Tabs
//!
//! Tab strip where exactly one panel is open at a time, no script. Tabs can carry a badge
//! count, be lazy (their body rendered only when open), run vertically, and collapse to a
//! `<select>` when the tabs are narrow (a container query), take an `.underline()` look after Radix
//! TabNav instead of the pill, and come in Radix's sizes (`.size(..)`, 1 to 3); the open tab
//! morphs to the next one on switch.
//!
//! **Platform features:**
//! - Links and the `hidden` attribute (baseline): each title is a link to its own tab, every
//!   panel but the open one is `hidden`. The markup follows Radix Tabs: a `.lui-tabs-list` of
//!   `.lui-tabs-trigger` links and one `.lui-tabs-panel` per tab, each with
//!   `data-state="active|inactive"`.
//! - `view-transition-name` (Chrome 111, Firefox 144, Safari 18) on the open tab's chip
//!   (an empty `.lui-tabs-mark`, never the title, so no text moves): the raised chip slides to
//!   the new tab, across documents through the layout's `@view-transition` rule and in place
//!   with the enhancement script. `view-transition-class: lui-tabs-mark` (Chrome 125,
//!   Firefox 144, Safari 18.2) gives the slide the `--lui-ease-spring` curve.
//!
//! **Accessibility:** each tab is a link, the open one `aria-current="true"` (one tab stop
//! each, Enter follows it); the narrow select is named "Tab"; not the ARIA tablist pattern,
//! which needs script (FINDINGS, M29 · axe-core over every route). Checked by axe-core in
//! headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//!
//! **What it does not do without script:** arrow keys between tabs (the WAI-ARIA tabs
//! pattern); each tab is a link reached by Tab.
//!
//! **Fallback:** none needed: links, a flex row and a grid work everywhere. Without
//! `Caps::ViewTransitions` the chip carries no transition name and simply appears on the new
//! tab.
//!
//! **Server persistence:** the open tab comes from the request's `?tab.<name>=` (see
//! [`crate::UiState::tab`]) and each title is a link to `?tab.<name>=i`, so the choice survives
//! navigation (query first, cookie after). The strip is a
//! swap root, so the [`crate::enhance`] script replaces just the strip instead of the page;
//! it also opens the clicked tab before the request, so the switch shows on the click.
//! Because every switch is a request, a lazy tab ([`Tabs::lazy`]) costs nothing until
//! opened: the component calls its closure only when it is the open tab.
//!
//! **Without script:** the narrow `<select>` needs its "Go" button; the script submits
//! it on change.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let mut ui = Ui::from_request("/docs", "tab.docs=1", "");
//! ui.caps = Caps::all();
//! // `badge` applies to the tab added last.
//! let m = ui.tabs("docs")
//!     .tab("Install", html! { p { "cargo add" } })
//!     .tab("Use", html! { p { "html!" } }).badge(3)
//!     .lazy("Changelog", || html! { p { "(long)" } })
//!     .vertical()
//!     .select_below();
//! let html = m.render().into_string();
//! assert!(html.contains("href=\"/docs?tab.docs=0\""));
//! assert!(html.contains("view-transition-name: lui-tabs-docs"));
//! assert!(html.contains("<select name=\"tab.docs\""));
//! // The same in `lui!`:
//! let same = lui! { Tabs("docs") vertical select_below {
//!     tab "Install" { p { "cargo add" } }
//!     tab "Use" badge=3 { p { "html!" } }
//!     lazy "Changelog" || { p { "(long)" } }
//! } };
//! assert_eq!(same.into_string(), html);
//! // Radix's underline look instead of the pill.
//! let line = ui.tabs("line").tab("One", html! {}).tab("Two", html! {}).underline().render().into_string();
//! assert!(line.contains("lui-tabs lui-tabs-underline"));
//! assert_eq!(lui! { Tabs("line") underline { tab "One" {} tab "Two" {} } }.into_string(), line);
//! // Smaller titles.
//! let small = ui.tabs("small").tab("One", html! {}).size(1).render().into_string();
//! assert!(small.contains(r#"class="lui-tabs" data-size="1""#));
//! assert_eq!(lui! { Tabs("small") size=1 { tab "One" {} } }.into_string(), small);
//! ```

use std::fmt::Display;
use std::rc::Rc;

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::variant::Size;
use crate::{Cap, Ui};

/// One tab: a title, a panel (ready or rendered on demand), an optional badge count.
#[derive(Clone, Debug)]
struct Tab<'a> {
    title: &'a str,
    body: Body<'a>,
    badge: Option<String>,
}

/// A panel: rendered already, or rendered on demand.
#[derive(Clone)]
enum Body<'a> {
    Ready(Markup),
    Lazy(Rc<dyn Fn() -> Markup + 'a>),
}

/// A lazy panel prints as `Lazy(<fn>)`.
impl std::fmt::Debug for Body<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Body::Ready(m) => f.debug_tuple("Ready").field(m).finish(),
            Body::Lazy(_) => f.write_str("Lazy(<fn>)"),
        }
    }
}

/// A tab strip, made by [`Ui::tabs`]: the open tab is `?tab.<name>=i` (or the cookie's
/// memory of it), and each title links to its own. Horizontal unless told otherwise.
///
/// **Setters.** Values and items: `.lazy(..)`, `.tab(..)`, `.badge(..)`, `.size(..)`; switches:
/// `.vertical()`, `.select_below()`, `.underline()`.
#[derive(Clone, Debug)]
pub struct Tabs<'a> {
    ui: &'a Ui,
    name: &'a str,
    tabs: Vec<Tab<'a>>,
    vertical: bool,
    select_below: bool,
    underline: bool,
    size: Size,
}

impl Tabs<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("tab", PropKind::Item, "title: &'a str, body: Markup")
            .doc("A tab titled `title` with its panel."),
        Prop::new(
            "lazy",
            PropKind::Item,
            "title: &'a str, body: impl Fn() -> Markup + 'a",
        )
        .doc("A tab whose panel is rendered only when it is the open one."),
        Prop::new("badge", PropKind::Modifier, "text: impl Display")
            .applies_to(&["tab", "lazy"])
            .doc("A count after the title of the tab added last."),
        Prop::new("vertical", PropKind::Switch, "")
            .doc("Titles in a column on the left, the open panel beside them."),
        Prop::new("select_below", PropKind::Switch, "")
            .doc("Under 30rem of its own width the titles give way to a `<select>`."),
        Prop::new("underline", PropKind::Switch, "")
            .doc("Titles on a hairline, the open one marked by a 2px bar (Radix TabNav)."),
        Prop::new(
            "size",
            PropKind::Choice { of: Size::NAMES },
            "size: impl Into<Size>",
        )
        .default("2")
        .doc("The titles' text and padding: smaller at 1, larger at 3."),
    ];
}

impl Ui {
    /// An empty strip named `name` (the key in `?tab.<name>=`); add tabs with [`Tabs::tab`].
    pub fn tabs<'a>(&'a self, name: &'a str) -> Tabs<'a> {
        Tabs {
            ui: self,
            name,
            tabs: Vec::new(),
            vertical: false,
            select_below: false,
            underline: false,
            size: Size::S2,
        }
    }
}

impl<'a> Tabs<'a> {
    /// A tab titled `title` with its panel.
    pub fn tab(mut self, title: &'a str, body: Markup) -> Self {
        self.tabs.push(Tab {
            title,
            body: Body::Ready(body),
            badge: None,
        });
        self
    }

    /// A tab whose panel is rendered only when it is the open one: an expensive panel costs
    /// nothing until the round trip that opens it.
    pub fn lazy(mut self, title: &'a str, body: impl Fn() -> Markup + 'a) -> Self {
        self.tabs.push(Tab {
            title,
            body: Body::Lazy(Rc::new(body)),
            badge: None,
        });
        self
    }

    /// A count after the title of the tab added last.
    #[track_caller]
    pub fn badge(mut self, text: impl Display) -> Self {
        let last = self.tabs.last().map(|t| match t.body {
            Body::Ready(_) => "tab",
            Body::Lazy(_) => "lazy",
        });
        crate::props::check_last::<Self>(Tabs::PROPS, "badge", last);
        if let Some(t) = self.tabs.last_mut() {
            t.badge = Some(text.to_string());
        }
        self
    }

    /// Titles in a column on the left, the open panel beside them.
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// Under 30rem of its own width (a container query) the titles give way to a `<select>`.
    pub fn select_below(mut self) -> Self {
        self.select_below = true;
        self
    }

    /// Titles on a hairline, the open one marked by a 2px bar in the primary colour that
    /// slides with view transitions (Radix Tabs and TabNav), instead of the pill.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// The titles' text and padding: Radix's sizes 1 and 2 (the default), and a larger 3.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl Render for Tabs<'_> {
    fn render(&self) -> Markup {
        let Tabs {
            ui,
            name,
            ref tabs,
            vertical,
            select_below,
            underline,
            size,
        } = *self;
        let state = &ui.state;
        let vt = ui.has(Cap::ViewTransitions);
        let active = state.tab(name);
        let key = format!("tab.{name}");
        let class = match (vertical, underline) {
            (false, false) => "lui-tabs",
            (false, true) => "lui-tabs lui-tabs-underline",
            (true, _) => "lui-tabs lui-tabs-vertical",
        };
        let tab_state = |i: usize| if i == active { "active" } else { "inactive" };
        html! {
            div id={ "lui-tabs-" (name) } data-lui="swap"
                class=(class) data-size=[(size != Size::S2).then(|| size.as_str())] data-orientation=[vertical.then_some("vertical")] {
                @if select_below {
                    form method="get" action=(state.path()) class="lui-tabs-select" {
                        @for (k, v) in state.entries() { @if k != key { input type="hidden" name=(k) value=(v); } }
                        select name=(key) aria-label=(ui.text(Text::Tab)) {
                            @for (i, t) in tabs.iter().enumerate() {
                                option value=(i) selected[i == active] { (t.title) @if let Some(n) = &t.badge { " (" (n) ")" } }
                            }
                        }
                        (ui.button(ui.text(Text::Go)))
                    }
                }
                div class="lui-tabs-list" {
                    @for (i, t) in tabs.iter().enumerate() {
                        a class="lui-tabs-trigger" href=(state.link(&key, &i.to_string()))
                            data-state=(tab_state(i)) aria-current=[(i == active).then_some("true")] {
                            @if i == active {
                                span class="lui-tabs-mark" style=[vt.then(|| format!("view-transition-name: lui-tabs-{name}; view-transition-class: lui-tabs-mark"))] {}
                            }
                            span class="lui-tabs-label" { (t.title) (badge(t)) }
                        }
                    }
                }
                @for (i, t) in tabs.iter().enumerate() {
                    div class="lui-tabs-panel" data-state=(tab_state(i)) hidden[i != active] {
                        @match (&t.body, i == active) {
                            (Body::Ready(body), _) => (body),
                            (Body::Lazy(render), true) => (render()),
                            (Body::Lazy(_), false) => span class="lui-tabs-lazy" {},
                        }
                    }
                }
            }
        }
    }
}
fn badge(tab: &Tab) -> Markup {
    html! { @if let Some(n) = &tab.badge { " " span class="lui-tabs-badge" { (n) } } }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* The tabs are their own container: .select_below() swaps the titles for a select when the
   tabs are narrow, not the viewport. */
.lui-tabs { container: lui-tabs / inline-size; }
/* shadcn Tabs: the titles sit in a muted pill (secondary, 3px padding, rounded-lg) that wraps
   onto another row when they do not fit; the open one is a raised chip on the page
   background. */
.lui-tabs-list {
  display: inline-flex; flex-wrap: wrap; max-width: 100%; box-sizing: border-box; padding: 3px;
  border-radius: var(--lui-radius); background: var(--lui-secondary);
}
.lui-tabs-trigger {
  position: relative; display: flex; align-items: center; padding: 0.25rem 0.75rem;
  border-radius: var(--lui-radius-sm); font-size: 0.875rem; line-height: 1.25rem; font-weight: 500;
  white-space: nowrap; text-decoration: none; color: color-mix(in srgb, var(--lui-muted) 70%, var(--lui-fg));
}
.lui-tabs-trigger:hover, .lui-tabs-trigger[data-state="active"] { color: var(--lui-fg); }
/* Positioned after the chip, so the title paints over it. */
.lui-tabs-label { position: relative; display: flex; align-items: center; gap: 0.375rem; }
.lui-tabs[data-size="1"] .lui-tabs-trigger { font-size: 0.8125rem; padding-inline: 0.5rem; }
.lui-tabs[data-size="3"] .lui-tabs-trigger { font-size: 1rem; line-height: 1.5rem; padding-inline: 1rem; }
/* The chip is its own empty element so the view transition slides it, not the text. */
.lui-tabs-mark {
  position: absolute; inset: 0; border-radius: var(--lui-radius-sm);
  background: var(--lui-bg); box-shadow: var(--lui-shadow-xs);
}
/* Motion: the chip slides on the spring curve (a view transition group, named only when the
   browser has view transitions). */
::view-transition-group(*.lui-tabs-mark) { animation-duration: var(--lui-duration-slow); animation-timing-function: var(--lui-ease-spring); }
.lui-tabs-badge {
  display: inline-block; min-width: 1.25rem; padding: 0 0.3rem; border-radius: 1em; text-align: center;
  font-size: 0.75rem; font-weight: 500; line-height: 1.25rem; background: var(--lui-line); color: var(--lui-fg);
}
.lui-tabs-trigger[data-state="active"] .lui-tabs-badge { background: var(--lui-primary); color: var(--lui-on-primary); }
.lui-tabs-panel { padding: 1rem 0; }
/* Vertical: a plain list with a rule, the open title marked by a bar on the rule; the open
   panel beside the titles. */
.lui-tabs[data-orientation=vertical] { display: grid; grid-template-columns: max-content minmax(0, 1fr); align-items: start; }
.lui-tabs[data-orientation=vertical] .lui-tabs-select { grid-column: 1 / -1; }
.lui-tabs[data-orientation=vertical] .lui-tabs-list {
  grid-column: 1; flex-direction: column; flex-wrap: nowrap; align-self: stretch; padding: 0;
  background: none; border-radius: 0; border-right: 1px solid var(--lui-line);
}
.lui-tabs[data-orientation=vertical] .lui-tabs-trigger { padding: 0.5rem 1rem; border-radius: 0; }
.lui-tabs[data-orientation=vertical] .lui-tabs-mark { inset: 0 -1px 0 auto; width: 2px; border-radius: 0; background: var(--lui-fg); box-shadow: none; }
.lui-tabs[data-orientation=vertical] .lui-tabs-panel { grid-column: 2; padding: 0 0 0 calc(var(--lui-space) * 3); }
/* .underline() (Radix Tabs/TabNav): no pill; the list carries the full-width hairline and the
   open title is marked by a 2px primary bar on it, the chip element restyled, so it slides the
   same way. Titles are muted, fg when open. */
.lui-tabs-underline .lui-tabs-list { display: flex; padding: 0; background: none; border-radius: 0; border-bottom: 1px solid var(--lui-line); }
.lui-tabs-underline .lui-tabs-trigger { padding: 0.625rem 0.75rem; border-radius: 0; color: var(--lui-muted); }
.lui-tabs-underline .lui-tabs-trigger:hover, .lui-tabs-underline .lui-tabs-trigger[data-state="active"] { color: var(--lui-fg); }
.lui-tabs-underline .lui-tabs-mark { inset: auto 0.5rem -1px; height: 2px; border-radius: 1px; background: var(--lui-primary); box-shadow: none; }
/* Narrow tabs: the titles give way to the select (needs the enhancement script for
   submit-on-change; the Go button is always there). */
.lui-tabs-select { display: none; gap: var(--lui-space-2); margin-bottom: var(--lui-space-2); }
@container lui-tabs (width < 30rem) {
  .lui-tabs-select ~ .lui-tabs-list { display: none; }
  .lui-tabs-select { display: flex; }
  .lui-tabs[data-orientation=vertical] { display: block; }
  .lui-tabs[data-orientation=vertical] .lui-tabs-panel { padding: 1rem 0; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn a_lazy_tab_renders_only_when_open() {
        let calls = Cell::new(0);
        let render = || {
            calls.set(calls.get() + 1);
            html! { p { "Changelog body" } }
        };
        let strip = |query: &str| {
            let ui = Ui::from_request("/docs", query, "");
            ui.tabs("docs")
                .tab("Install", html! { "cargo add" })
                .lazy("Changelog", render)
                .render()
                .into_string()
        };
        let closed = strip("");
        assert!(!closed.contains("Changelog body") && closed.contains("lui-tabs-lazy"));
        assert_eq!(calls.get(), 0);
        let open = strip("tab.docs=1");
        assert!(open.contains("Changelog body") && !open.contains("lui-tabs-lazy"));
        assert_eq!(calls.get(), 1);
    }
}
