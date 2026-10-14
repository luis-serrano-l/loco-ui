# Empty state

`loco_ui::empty_state` · demo: `/dashboard`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `EmptyState` | `ui.empty_state(title: &str)` | stable |

## Props of `EmptyState`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `icon` | value | `icon: impl Into<Glyph<'a>>` |  |  | An icon, or a glyph or emoji, above the title, hidden from screen readers. |
| `body` | value | `text: Markup` |  |  | One or two sentences. |
| `link` | value | `label: &'a str, href: &'a str` |  |  | A link to follow. |
| `action` | value | `label: &'a str, action: &'a str` |  |  | A button posting to `action`. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<form method="post">` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: content
- Look after: shadcn Empty

## Documentation

What a list, table or search shows when there is nothing in it: what is missing, why, and
the one thing to do next, as a real link or form button.

**Platform features:** plain HTML; the action is an `<a>` or, for something that changes
data, a `<form method="post">` button, so it works with nothing else.

**Accessibility:** a heading, a sentence and a real action; the illustration is
`aria-hidden`. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing; it is static content.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
assert!(ui.empty_state("No files yet").render().into_string().contains("No files yet"));
let m = ui.empty_state("No results for \u{201c}zzz\u{201d}")
    .icon("\u{1f50d}")
    .body(html! { "Check the spelling or clear the filter." })
    .link("Clear the filter", "/table")
    .action("Create a file", "/files/new");
let m = m.render().into_string();
assert!(m.contains(r#"href="/table""#) && m.contains(r#"action="/files/new""#));
// The same in `lui!`:
let same = lui! {
    EmptyState("No results for \u{201c}zzz\u{201d}") icon="\u{1f50d}"
        body=(html! { "Check the spelling or clear the filter." }) {
        link "Clear the filter" "/table";
        action "Create a file" "/files/new";
    }
};
assert_eq!(same.into_string(), m);
```
