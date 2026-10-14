# SettingsPage

`loco_ui::blocks::settings_page` · demo: `/blocks/settings`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `SettingsPage` | `ui.settings_page(title: &str)` | beta |

## Props of `SettingsPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `section` | item | `title: &'a str, description: &'a str, body: Markup` |  |  | A section: its title and a line on the left, `body` (a form) on the right. |

## Documentation

A page of settings in sections: each has a title and a line on the left and its form on
the right (stacked on narrow screens), with a list of the sections at the top that jumps
to each.

**Platform features:** fragment links to each section's `id`; a two-column grid that falls
back to one column under 48rem.

**Accessibility:** one `<h1>`, an `<h2>` per section, the section list a `<nav>` named by
the page title; each section is a `<section>` labelled by its heading. Checked by axe-core
with the demo routes.

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.settings_page("Settings")
    .section("Profile", "How others see you.", html! { p { "form" } })
    .section("Email", "Where we write to you.", html! { p { "form" } })
    .render().into_string();
assert!(m.contains(r##"<a href="#settings-email">Email</a>"##) && m.contains(r#"<section id="settings-email""#));
// The same in `lui!`:
let same = lui! { SettingsPage("Settings") {
    section "Profile" "How others see you." (html! { p { "form" } });
    section "Email" "Where we write to you." (html! { p { "form" } });
} };
assert_eq!(same.into_string(), m);
```
