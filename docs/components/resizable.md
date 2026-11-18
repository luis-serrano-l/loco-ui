# Resizable panes

`loco_ui::resizable` · demo: `/resizable`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Resizable` | `ui.resizable(label: &str)` | beta |

## Props of `Resizable`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `id` | value | `id: &'a str` |  |  | Replace the key derived from the label: the state is `size.<id>`, the element `lui-resizable-<id>`. |
| `first` | value | `body: Markup` |  |  | The first pane, the one the size is of. |
| `second` | value | `body: Markup` |  |  | The second pane, which takes the rest. |
| `value` | number | `value: impl Into<f64>` | `50` |  | The first pane's share in percent until the visitor changes it. |
| `min` | number | `min: impl Into<f64>` | `20` |  | The smallest share, in percent. |
| `max` | number | `max: impl Into<f64>` | `80` |  | The largest share, in percent. |
| `step` | number | `step: impl Into<f64>` | `10` |  | How far a link or an arrow key moves the divider, in percent. |

## Keyboard of `Resizable`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | Follows the smaller or larger link. | no |
| `ArrowLeft / ArrowRight` | Moves the divider one step. | yes |
| `Home / End` | Moves the divider to its minimum or maximum. | yes |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-resizable` | `--lui-line`, `--lui-radius-lg`, `--lui-card` |
| `.lui-resizable-row` | `--lui-resizable-size` |
| `.lui-resizable-pane` | `--lui-space-4`, `--lui-line` |
| `.lui-resizable-divider` | `--lui-space-1`, `--lui-line`, `--lui-hit` |
| `.lui-resizable-handle` |  |
| `.lui-resizable-grip` | `--lui-line`, `--lui-radius-sm`, `--lui-card`, `--lui-muted`, `--lui-ring` |
| `.lui-resizable-step` | `--lui-card` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `role="separator"` | 1 | 1 | 1 |
| `container query` | 105 | 110 | 16 |
| `setPointerCapture` | 55 | 59 | 13 |

- Fallback: under 30rem the panes stack and there is no divider
- Needs script: partly: the size changes a step at a time by links; dragging and the arrow keys need script
- Responsive by: container
- Look after: shadcn/ui Resizable

## Documentation

Two panes side by side with a divider between them, and a size the server remembers: a
file list beside a preview, a navigation beside a page. The size is the first pane's share
of the width in percent. Two links on the divider make it one step smaller or larger; with
the enhancement script the divider drags and answers the arrow keys. After shadcn/ui
Resizable with a handle: a one-pixel line and a small grip in its middle.

**Platform features:** CSS grid with a custom property for the first column (baseline);
links that set `size.<id>` in the query, which the `lui-ui` cookie then remembers
(`crate::UiState`); `role="separator"` with `aria-valuenow`; a container query (Chrome
105, Firefox 110, Safari 16) that puts the panes side by side from 30rem; with the script,
pointer events and `setPointerCapture` (Chrome 55, Firefox 59, Safari 13).

**Accessibility:** the divider is a `role="separator"` named by the label, with its value,
minimum and maximum; the two links are named "Make … smaller" and "Make … larger". With
the script the separator takes the focus and the links are hidden, since the arrow keys
do their work. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no moderate, serious or critical violation).

The root is not a swap root: links and forms inside the panes behave as they would
outside, and the script changes the size itself, with no request for markup.

**What it does not do without script:** dragging, and the arrow keys on the separator.
Each change is a link, one step at a time.

**Fallback:** under 30rem the panes stack and there is no divider: there is no width to
share.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.resizable("Files").first(html! { p { "list" } }).second(html! { p { "preview" } })
    .render().into_string();
assert!(m.starts_with(r#"<div class="lui-resizable" id="lui-resizable-files" style="--lui-resizable-size:50%">"#));
assert!(m.contains(r#"role="separator" aria-orientation="vertical" aria-label="Files" aria-valuenow="50" aria-valuemin="20" aria-valuemax="80""#));
assert!(m.contains(r#"?size.files=40" aria-label="Make Files smaller""#) && m.contains(r#"aria-label="Make Files larger""#));
// The same in `lui!`:
let same = lui! { Resizable("Files") first={ p { "list" } } second={ p { "preview" } }; };
assert_eq!(same.into_string(), m);
// A narrower first pane with finer steps; the request's `size.<id>` wins over `.value(..)`:
let ui = Ui::from_request("/", "size.files=35", "");
let m = ui.resizable("Files").value(30).min(15).max(60).step(5).render().into_string();
assert!(m.contains("--lui-resizable-size:35%") && m.contains(r#"href="/?size.files=30""#));
assert_eq!(lui! { Resizable("Files") value=30 min=15 max=60 step=5; }.into_string(), m);
```
