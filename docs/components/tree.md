# Tree

`loco_ui::tree` · demo: `/tree`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Tree` | `ui.tree(label: &str)` | beta |

## Props of `Tree`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `node` | item | `label: &'a str` |  |  | A node reading `label`. |
| `link` | modifier | `href: &'a str` |  |  | The node added last is a link; the one to the current path is marked current. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon before the label of the node added last. |
| `children` | modifier | `nodes: impl FnOnce(Tree<'a>) -> Tree<'a>` |  |  | The nodes under the node added last, which makes it a folder. |
| `open` | modifier | `open: bool` |  | `open` | The folder added last starts open while `open` holds. |
| `id` | value | `id: &'a str` |  |  | The key in `?open.<id>=` instead of the label's slug. |

## Keyboard of `Tree`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the folder toggles and the links. | no |
| `Enter / Space` | Opens or closes the focused folder. | no |
| `Enter` | Follows the focused link. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-tree` | `--lui-text-2` |
| `.lui-tree-row` | `--lui-space-2`, `--lui-tree-depth`, `--lui-radius-sm`, `--lui-fg`, `--lui-accent`, `--lui-on-accent`, `--lui-gray-4`, `--lui-hit` |
| `.lui-tree-label` |  |
| `.lui-tree-icon` | `--lui-muted` |
| `.lui-tree-toggle` | `--lui-tree-depth`, `--lui-radius-sm`, `--lui-accent`, `--lui-hit` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<details>` | 12 | 49 | 6 |
| `aria-current="page"` | 1 | 1 | 1 |
| `--lui-tree-depth` | 49 | 31 | 9.1 |

- Fallback: none needed: <details> works everywhere
- Needs script: partly: arrow keys between nodes (the WAI-ARIA tree pattern) need script
- Responsive by: content
- Look after: Origin UI Tree

## Documentation

Nested navigation: folders that open and close, each node a link or a heading, the node
of the current page marked and its folders open. A file browser, a docs outline, a
category tree. After the Origin UI Tree.

**Platform features:** nested `<details>`/`<summary>` (baseline 2020) for the folders,
inside a `<nav>` of lists; `aria-current="page"` on the link to the current path; the
indent is `--lui-tree-depth`, a custom property on each item.

**Accessibility:** a `<nav>` named by its label; each folder is a `<summary>` (Enter or
Space opens it); a folder that is also a link has its link beside the summary, never in
it, and the summary is named by the folder's label; icons and chevrons are `aria-hidden`.
Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

**What it does not do without script:** arrow keys between nodes (the WAI-ARIA tree
pattern) and remembering a folder opened by hand on the next page: the server opens the
folders `?open.<id>=` names and those around the current page.

**Fallback:** none needed; `<details>` works everywhere.

**Server state:** the folders are numbered in document order from 0, and
`?open.<id>=0,2` (a comma list, see `crate::UiState::opens`, remembered in the `lui-ui`
cookie like an accordion) opens them; so do `.open(true)` and holding the current page.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/docs/install", "", "");
let m = ui.tree("Docs")
    .node("Intro").link("/docs")
    .node("Guide").children(|t| t.node("Install").link("/docs/install").node("Use").link("/docs/use"))
    .render().into_string();
assert!(m.contains(r#"href="/docs/install" aria-current="page""#));
assert!(m.contains("<details open>"), "the folder holding the current page is open");
// Icons, a folder that is a link too, a folder opened by the URL.
let ui = Ui::from_request("/files", "open.files=1", "");
let m = ui.tree("Files")
    .node("src").icon(Icon::Folder).link("/files/src").children(|t| t
        .node("lib.rs").icon(Icon::File).link("/files/src/lib.rs")
        .node("bin").icon(Icon::Folder).children(|t| t.node("main.rs").icon(Icon::File)))
    .node("README.md").icon(Icon::File);
let html = m.render().into_string();
assert!(html.contains(r#"<summary class="lui-tree-toggle">"#) && !html.contains("<summary class=\"lui-tree-toggle\"><a"));
assert!(html.contains("--lui-tree-depth: 2"));
// The same in `lui!`:
let same = lui! { Tree("Files") {
    node "src" icon=(Icon::Folder) link="/files/src" children=(|t| t
        .node("lib.rs").icon(Icon::File).link("/files/src/lib.rs")
        .node("bin").icon(Icon::Folder).children(|t| t.node("main.rs").icon(Icon::File)));
    node "README.md" icon=(Icon::File);
} };
assert_eq!(same.into_string(), html);
```
