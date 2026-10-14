# ErrorPage

`loco_ui::blocks::error_page` · demo: `/blocks/error`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `ErrorPage` | `ui.error_page(status: u16)` | beta |

## Props of `ErrorPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `title` | value | `text: &'a str` | `the status's own heading` |  | The heading. |
| `description` | value | `text: &'a str` |  |  | The sentence under it. |
| `home` | value | `href: &'a str` | `/` | `href` | Where the button goes. |

## Documentation

The page for an HTTP error: the status, what happened in a sentence, and a way home, in
the site's look instead of the framework's plain text. `not_found` (feature `axum`) is an Axum handler
for `Router::fallback`, so every unknown path gets it; on Loco, add it in `after_routes`.
It has its own words for 401 (sign in), 403 (no access), 404 (not here), 422 (the form
could not be accepted) and 503 (down for a moment); any other status says something went
wrong.

**Platform features:** none of its own; the HTTP status is set on the response.

**Accessibility:** the title is the page's `<h1>`; the status number is decorative and
hidden from assistive tech (the title says it). Checked by axe-core with the demo routes.

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.error_page(404).render().into_string();
assert!(m.contains("<h1>Page not found</h1>") && m.contains(r#"href="/""#));
let m = ui.error_page(500).home("/dashboard").render().into_string();
assert!(m.contains("Something went wrong") && m.contains(r#"href="/dashboard""#));
// The same in `lui!`:
let same = lui! { ErrorPage(500) home="/dashboard"; };
assert_eq!(same.into_string(), m);
for (status, title) in [(401, "Sign in to continue"), (403, "You cannot open this page"),
    (422, "That could not be saved"), (503, "Back in a moment")] {
    assert!(ui.error_page(status).render().into_string().contains(&format!("<h1>{title}</h1>")));
}
```
