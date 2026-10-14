# Description list

`loco_ui::description_list` · demo: `/description-list`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `DescriptionList` | `ui.description_list()` | beta |

## Props of `DescriptionList`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `item` | item | `term: &'a str, detail: impl Render` |  |  | A term and its detail (text, or markup such as a badge). |
| `stacked` | switch |  | `off` |  | Each term above its detail instead of beside it. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<dl>` | 1 | 1 | 1 |
| `@container` | 105 | 110 | 16 |
| `subgrid` | 117 | 71 | 16 |

- Fallback: without container queries the terms stay above their details; without subgrid the term column is a fixed 10rem
- Needs script: no
- Responsive by: container
- Look after: Radix Themes DataList

## Documentation

Terms and what they are: a record's fields, a summary before confirming, key facts. Terms
sit in a column beside their details, or above them with `.stacked()`.

**Platform features:** `<dl>` with `<dt>`/`<dd>` pairs, each pair in a `<div>` (allowed in a
`<dl>`, every browser); container queries (`@container`: Chrome 105, Firefox 110, Safari 16) put the
terms beside their details once the list is 30rem wide; subgrid (Chrome 117, Firefox 71,
Safari 16) gives every row one shared term column, as wide as the longest term.

**Accessibility:** a real description list, so screen readers announce the pairs and their
count; a detail may hold any markup (a badge, a link). Checked by axe-core in headless
Firefox on every demo route, both capability variants, light and dark (no serious or
critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** without container queries the terms stay above their details; without
subgrid the term column is a fixed 10rem. An empty detail shows a dash.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.description_list().item("Plan", "Team").item("Seats", "12").render().into_string();
assert!(m.contains("<dt>Plan</dt><dd>Team</dd>"));

let m = ui.description_list().stacked().item("Status", ui.badge("Active").ok());
let html = m.render().into_string();
assert!(html.contains("lui-description-list lui-description-list-stacked") && html.contains("Active"));
// The same in `lui!`:
let same = lui! { DescriptionList stacked { item "Status" (ui.badge("Active").ok()); } };
assert_eq!(same.into_string(), html);
```
