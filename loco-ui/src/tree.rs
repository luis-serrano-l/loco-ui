//! # Tree
//!
//! Nested navigation: folders that open and close, each node a link or a heading, the node
//! of the current page marked and its folders open. A file browser, a docs outline, a
//! category tree. After the Origin UI Tree.
//!
//! **Platform features:** nested `<details>`/`<summary>` (baseline 2020) for the folders,
//! inside a `<nav>` of lists; `aria-current="page"` on the link to the current path; the
//! indent is `--lui-tree-depth`, a custom property on each item.
//!
//! **Accessibility:** a `<nav>` named by its label; each folder is a `<summary>` (Enter or
//! Space opens it); a folder that is also a link has its link beside the summary, never in
//! it, and the summary is named by the folder's label; icons and chevrons are `aria-hidden`.
//! Checked by axe-core in headless Firefox on every demo route, both capability variants,
//! light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** arrow keys between nodes (the WAI-ARIA tree
//! pattern) and remembering a folder opened by hand on the next page: the server opens the
//! folders `?open.<id>=` names and those around the current page.
//!
//! **Fallback:** none needed; `<details>` works everywhere.
//!
//! **Server state:** the folders are numbered in document order from 0, and
//! `?open.<id>=0,2` (a comma list, see [`crate::UiState::opens`], remembered in the `lui-ui`
//! cookie like an accordion) opens them; so do `.open(true)` and holding the current page.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from_request("/docs/install", "", "");
//! let m = ui.tree("Docs")
//!     .node("Intro").link("/docs")
//!     .node("Guide").children(|t| t.node("Install").link("/docs/install").node("Use").link("/docs/use"))
//!     .render().into_string();
//! assert!(m.contains(r#"href="/docs/install" aria-current="page""#));
//! assert!(m.contains("<details open>"), "the folder holding the current page is open");
//! // Icons, a folder that is a link too, a folder opened by the URL.
//! let ui = Ui::from_request("/files", "open.files=1", "");
//! let m = ui.tree("Files")
//!     .node("src").icon(Icon::Folder).link("/files/src").children(|t| t
//!         .node("lib.rs").icon(Icon::File).link("/files/src/lib.rs")
//!         .node("bin").icon(Icon::Folder).children(|t| t.node("main.rs").icon(Icon::File)))
//!     .node("README.md").icon(Icon::File);
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<summary class="lui-tree-toggle">"#) && !html.contains("<summary class=\"lui-tree-toggle\"><a"));
//! assert!(html.contains("--lui-tree-depth: 2"));
//! // The same in `lui!`:
//! let same = lui! { Tree("Files") {
//!     node "src" icon=(Icon::Folder) link="/files/src" children=(|t| t
//!         .node("lib.rs").icon(Icon::File).link("/files/src/lib.rs")
//!         .node("bin").icon(Icon::Folder).children(|t| t.node("main.rs").icon(Icon::File)));
//!     node "README.md" icon=(Icon::File);
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use std::cell::Cell;

use maud::{Markup, Render, html};

use crate::icon::Glyph;
use crate::props::{Key, Prop, PropKind};
use crate::{Icon, Ui, slug};

/// One node: a label, where it links, an icon and the nodes under it.
#[derive(Clone, Debug)]
struct Node<'a> {
    label: &'a str,
    href: Option<&'a str>,
    icon: Option<Glyph<'a>>,
    open: bool,
    children: Vec<Node<'a>>,
}

impl Node<'_> {
    /// Whether this node or one under it links to `here`.
    fn holds(&self, here: &str) -> bool {
        self.href == Some(here) || self.children.iter().any(|c| c.holds(here))
    }
}

/// Nested links in folders, made by [`Ui::tree`].
///
/// **Setters.** Values and items: `.node(..)`, `.link(..)`, `.icon(..)`, `.children(..)`,
/// `.id(..)`; conditions: `.open(..)`.
#[derive(Clone, Debug)]
pub struct Tree<'a> {
    ui: &'a Ui,
    label: &'a str,
    id: Option<&'a str>,
    nodes: Vec<Node<'a>>,
}

impl Tree<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[
        Key::new(
            "Tab / Shift+Tab",
            "Moves between the folder toggles and the links.",
        ),
        Key::new("Enter / Space", "Opens or closes the focused folder."),
        Key::new("Enter", "Follows the focused link."),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("node", PropKind::Item, "label: &'a str").doc("A node reading `label`."),
        Prop::new("link", PropKind::Modifier, "href: &'a str")
            .applies_to(&["node"])
            .doc("The node added last is a link; the one to the current path is marked current."),
        Prop::new("icon", PropKind::Modifier, "icon: impl Into<Glyph<'a>>")
            .applies_to(&["node"])
            .doc("An icon before the label of the node added last."),
        Prop::new(
            "children",
            PropKind::Modifier,
            "nodes: impl FnOnce(Tree<'a>) -> Tree<'a>",
        )
        .applies_to(&["node"])
        .doc("The nodes under the node added last, which makes it a folder."),
        Prop::new("open", PropKind::Modifier, "open: bool")
            .applies_to(&["node"])
            .attr("open")
            .doc("The folder added last starts open while `open` holds."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .doc("The key in `?open.<id>=` instead of the label's slug."),
    ];
}

impl Ui {
    /// A tree named `label` (for assistive tech); add nodes with [`Tree::node`].
    pub fn tree<'a>(&'a self, label: &'a str) -> Tree<'a> {
        Tree {
            ui: self,
            label,
            id: None,
            nodes: Vec::new(),
        }
    }
}

impl<'a> Tree<'a> {
    fn last(mut self, method: &str, change: impl FnOnce(&mut Node<'a>)) -> Self {
        let last = self.nodes.last().map(|_| "node");
        crate::props::check_last::<Self>(Self::PROPS, method, last);
        if let Some(node) = self.nodes.last_mut() {
            change(node);
        }
        self
    }

    /// A node reading `label`.
    pub fn node(mut self, label: &'a str) -> Self {
        self.nodes.push(Node {
            label,
            href: None,
            icon: None,
            open: false,
            children: Vec::new(),
        });
        self
    }

    /// The node added last is a link; the one to the current path is marked current.
    pub fn link(self, href: &'a str) -> Self {
        self.last("link", |n| n.href = Some(href))
    }

    /// An icon before the label of the node added last.
    pub fn icon(self, icon: impl Into<Glyph<'a>>) -> Self {
        let icon = icon.into();
        self.last("icon", |n| n.icon = Some(icon))
    }

    /// The nodes under the node added last, which makes it a folder: `nodes` gets an empty
    /// tree and returns it filled (`|t| t.node("a").node("b")`).
    pub fn children(self, nodes: impl FnOnce(Tree<'a>) -> Tree<'a>) -> Self {
        let inner = nodes(Tree {
            ui: self.ui,
            label: self.label,
            id: self.id,
            nodes: Vec::new(),
        });
        self.last("children", |n| n.children = inner.nodes)
    }

    /// The folder added last starts open while `open` holds (a route that knows which
    /// folder the visitor wants).
    pub fn open(self, open: bool) -> Self {
        self.last("open", |n| n.open = open)
    }

    /// The key in `?open.<id>=` instead of the label's slug.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    fn list(
        &self,
        nodes: &[Node<'a>],
        depth: usize,
        folder: &Cell<usize>,
        opened: &[usize],
    ) -> Markup {
        let here = self.ui.state.path();
        html! {
            ul {
                @for n in nodes {
                    li style={ "--lui-tree-depth: " (depth) } {
                        @let face = html! {
                            @if let Some(icon) = n.icon { span class="lui-tree-icon" aria-hidden="true" { (icon) } }
                            span class="lui-tree-label" { (n.label) }
                        };
                        @let row = html! {
                            @if let Some(href) = n.href {
                                a class="lui-tree-row" href=(href) aria-current=[(href == here).then_some("page")] { (face) }
                            } @else {
                                span class="lui-tree-row" { (face) }
                            }
                        };
                        @if n.children.is_empty() {
                            (row)
                        } @else {
                            @let index = { let i = folder.get(); folder.set(i + 1); i };
                            @let open = n.open || opened.contains(&index) || n.children.iter().any(|c| c.holds(here));
                            @if n.href.is_some() {
                                (row)
                                details open[open] {
                                    summary class="lui-tree-toggle" { (Icon::ChevronRight) span class="lui-sr" { (n.label) } }
                                    (self.list(&n.children, depth + 1, folder, opened))
                                }
                            } @else {
                                details open[open] {
                                    summary class="lui-tree-row" { (Icon::ChevronRight) (face) }
                                    (self.list(&n.children, depth + 1, folder, opened))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Render for Tree<'_> {
    fn render(&self) -> Markup {
        let key = self.id.map_or_else(|| slug(self.label), str::to_string);
        let opened = self.ui.state.opens(&key);
        html! {
            nav class="lui-tree" aria-label=(self.label) {
                (self.list(&self.nodes, 0, &Cell::new(0), &opened))
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Origin UI Tree: 2rem rows (44px on touch) in text-sm, 0.25rem radius, the
   accent on hover and on the current page, a chevron that turns when its folder opens,
   1.25rem of indent per level (--lui-tree-depth on each item). A folder that is also a link
   puts its summary, the chevron alone, over the start of the link's row. */
.lui-tree { font-size: var(--lui-text-2); }
.lui-tree ul { list-style: none; margin: 0; padding: 0; }
.lui-tree li { position: relative; max-width: none; }
.lui-tree-row {
  display: flex; align-items: center; gap: var(--lui-space-2); min-height: 2rem; box-sizing: border-box; min-width: 0;
  padding-block: 0.25rem; padding-inline: calc(var(--lui-tree-depth, 0) * 1.25rem + 1.75rem) 0.5rem;
  border-radius: var(--lui-radius-sm); color: var(--lui-fg); text-decoration: none; cursor: default;
}
a.lui-tree-row, summary.lui-tree-row { cursor: pointer; }
.lui-tree summary { list-style: none; }
.lui-tree summary::-webkit-details-marker { display: none; }
summary.lui-tree-row { padding-inline-start: calc(var(--lui-tree-depth, 0) * 1.25rem + 0.25rem); }
.lui-tree-row:is(a, summary):hover { background: var(--lui-accent); color: var(--lui-on-accent); }
.lui-tree-row[aria-current="page"] { background: var(--lui-gray-4); color: var(--lui-fg); font-weight: 500; }
.lui-tree-label { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.lui-tree-icon { display: inline-flex; color: var(--lui-muted); }
.lui-tree .lui-icon { flex: none; width: 1rem; height: 1rem; }
.lui-tree summary > .lui-icon { color: var(--lui-muted); transition: rotate var(--lui-duration) var(--lui-ease-out); }
.lui-tree details[open] > summary > .lui-icon { rotate: 90deg; }
.lui-tree-toggle {
  position: absolute; top: 0; inset-inline-start: calc(var(--lui-tree-depth, 0) * 1.25rem + 0.125rem);
  display: grid; place-items: center; width: 1.5rem; height: 2rem; border-radius: var(--lui-radius-sm); cursor: pointer;
}
.lui-tree-toggle:hover { background: var(--lui-accent); }
@media (pointer: coarse) {
  .lui-tree-row { min-height: var(--lui-hit); }
  a.lui-tree-row { padding-inline-start: calc(var(--lui-tree-depth, 0) * 1.25rem + var(--lui-hit) + 0.25rem); }
  .lui-tree-toggle { height: var(--lui-hit); width: var(--lui-hit); }
}
@media (prefers-reduced-motion: reduce) { .lui-tree summary > .lui-icon { transition: none; } }
"#;
