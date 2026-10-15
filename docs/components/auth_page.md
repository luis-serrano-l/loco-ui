# AuthPage

`loco_ui::blocks::auth_page` · demo: `/blocks/auth`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `AuthPage` | `ui.auth_page(title: &str)` | beta |

## Props of `AuthPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `description` | value | `text: &'a str` |  |  | A line under the title. |
| `body` | value | `body: Markup` |  |  | The form. |
| `footer` | value | `footer: Markup` |  |  | Under the card: links to the other account pages. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-auth` | `--lui-space` |
| `.lui-auth-card` | `--lui-space`, `--lui-card`, `--lui-line`, `--lui-radius-lg`, `--lui-shadow-xs` |
| `.lui-auth-title` | `--lui-text-6` |
| `.lui-auth-description` | `--lui-muted`, `--lui-text-2` |
| `.lui-auth-footer` | `--lui-text-2`, `--lui-muted` |

## Documentation

A sign-in, sign-up or reset page: a narrow card in the middle of the page with a title, a
line under it, the form, and the links to the other account pages below. `cargo lui auth`
writes plain views; this is the same page ready-made.

**Platform features:** none of its own: a card and a form that posts.

**Accessibility:** the title is the page's `<h1>`; the form brings its labels, error
summary and focus handling. Checked by axe-core with the demo routes.

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let form = ui.form("/signin").email("email", "Email").required().submit("Sign in");
let m = ui.auth_page("Sign in").description("Welcome back.").body(form.render())
    .footer(html! { a href="/signup" { "Create an account" } }).render().into_string();
assert!(m.contains("<h1 class=\"lui-auth-title\">Sign in</h1>") && m.contains("Create an account"));
// The same in `lui!`:
let same = lui! { AuthPage("Sign in") description="Welcome back." body=(form.render())
    footer=(html! { a href="/signup" { "Create an account" } }); };
assert_eq!(same.into_string(), m);
```
