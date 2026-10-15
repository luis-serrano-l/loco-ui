# DashboardPage

`loco_ui::blocks::dashboard_page` · demo: `/blocks/dashboard`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `DashboardPage` | `ui.dashboard_page(title: &str)` | beta |

## Props of `DashboardPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `description` | value | `text: &'a str` |  |  | A line under the title. |
| `stat` | item | `stat: Stat<'a>` |  |  | A number in the row of stats. |
| `body` | value | `body: Markup` |  |  | What goes under the stats. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-dashboard` |  |
| `.lui-dashboard-description` | `--lui-muted` |

## Documentation

A page of numbers: a title and a line, a row of stats that wraps to the width available,
and whatever goes under them (a table, a chart, recent activity).

**Platform features:** those of `crate::stat`: an auto-fit grid, no media query.

**Accessibility:** the title is the page's `<h1>`; each stat says its change in words.
Checked by axe-core with the demo routes.

**What it does not do without script:** update live; the numbers are as fresh as the page.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.dashboard_page("Overview").description("Last 30 days.")
    .stat(ui.stat("Revenue", "$12,480").delta("+8%"))
    .stat(ui.stat("Customers", "312"))
    .body(html! { p { "Recent orders" } })
    .render().into_string();
assert!(m.contains("lui-stat-grid") && m.contains("$12,480") && m.contains("Recent orders"));
// The same in `lui!`:
let same = lui! { DashboardPage("Overview") description="Last 30 days." {
    stat (ui.stat("Revenue", "$12,480").delta("+8%"));
    stat (ui.stat("Customers", "312"));
    body (html! { p { "Recent orders" } });
} };
assert_eq!(same.into_string(), m);
```
