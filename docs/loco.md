# loco-ui on Loco

**Server-rendered scaffolds for Loco 1.x, zero JavaScript required.** Loco 1.0 removed its
`--html`/`--htmx` scaffolds in favour of a JSON API and a React SPA, so a Loco app that wants
server-rendered pages has had no first-party answer. With loco-ui, `cargo lui install` sets the
app up, `cargo loco generate scaffold` writes HTML controllers and Maud views for a model,
`cargo lui auth` writes the account pages, and every page works with JavaScript off (one
optional 12 KB script updates them in place).

[Loco](https://loco.rs) is a Rails-style framework on Axum, so a Loco controller is an Axum
handler and takes `ui: Ui` like any other. The `loco` feature adds the one piece of wiring
Loco needs, a form helper and a scaffold. The API reference is the `loco_ui::loco` module
docs, whose examples are compiled and run by `cargo test -p loco-ui --features loco --doc
loco`. [`examples/loco-app`](../examples/loco-app) is all of it in one app: sign-in and a
notes model, every page working with script off, tested through Loco's own router and Blitz.

## Install

One command sets a Loco app up: it adds the two dependencies below, the initializer line,
the scaffold templates, a `src/views/layout.rs` (a page with a header) and a
`src/views/look.rs` (the app's [`Look`](../loco-ui/src/layout.rs): tokens and extra CSS),
both registered in `src/views/mod.rs`, and two hooks in `src/app.rs`: `before_routes` gives
every path no route answers loco-ui's 404 page, and `after_routes` layers the look and the
strict Content-Security-Policy (`loco_ui::enhance::csp`) over every page. Running it again
changes nothing, a file you edited is kept (`--force` overwrites it), a hook the app already
defines is left alone with a note, and `--dry-run` prints what would change. The manifest is
edited with `toml_edit`, so comments, CRLF line ends and `[dependencies.x]` tables stay.

```sh
cargo install --git https://github.com/luis-serrano-l/loco-ui loco-ui --bin cargo-lui --features cli
cargo lui install            # in the app's directory, or `cargo lui install path/to/app`
cargo lui --help             # every command
```

`loco-ui/tests/install.rs` runs it on a fresh `loco new` app and builds the result. By hand,
the same steps are the rest of this page:

```toml
# Cargo.toml of the Loco app. The crate is not on crates.io yet: `install` pins the git
# dependency to the commit the installer was built from (`--rev` names another).
loco-ui = { git = "https://github.com/luis-serrano-l/loco-ui", rev = "<commit>", features = ["loco"] }
maud = "0.27" # `html!` expands to `maud::` paths, so the app names it too
```

### A new app

`cargo lui new my_app` writes an Axum app: the app shell with home and settings, the theme
toggle, the 404 page, the CSP layer, a `Look`, a test that every page answers without a
script of its own, and `scripts/check.sh` (format, clippy, tests, no `<script>` in the
source). `cargo lui new my_app --loco` runs `loco new` (SQLite, blocking workers, no assets;
`--from DIR` copies an existing Loco app instead), then `install` and `auth`, and adds the
same pages and a demo account (`demo@example.com` / `demo-password`) made on the first start
in development. The files come from `loco-ui/loco-templates/new/`.

### Copying a component

`cargo lui add badge` copies `loco-ui/src/badge.rs`, from the app's own loco-ui dependency,
into `src/ui/app_badge.rs`: the whole file, its `//!` header, doctests, `PROPS` and `CSS`.
Rust calls a type's own method before a trait's, so a copy named `ui.badge(..)` would never
run; the copy is `ui.app_badge(..)` (`AppBadge(..)` in `lui!`, classes `app-badge`), and
`--prefix acme` names it `acme_badge`. Its `impl Ui` becomes an extension trait,
`AppBadgeExt`, the pattern [docs/components.md](components.md) describes. `src/ui/mod.rs`
re-exports each copy, so `use crate::ui::*;` brings them all, and the copy's `CSS` joins the
`css` list in `src/views/look.rs`.

```sh
cargo lui list                     # what can be copied
cargo lui info badge               # its setters
cargo lui add badge --dry-run      # what would be written; --view prints the file
cargo lui add badge --diff         # your copy beside loco-ui's current one
cargo lui eject                    # all of loco-ui into vendor/, the dependency a path to it
```

`list` names the 26 components whose files use loco-ui's public API only; the others call
helpers private to the crate (the shared search box, `labelled`, the menu placement), and
`eject` is the way to own those.

### Nested scaffolds

`cargo lui scaffold note title:string! notebook:references --parent notebook` runs
`cargo loco generate scaffold` with `LUI_PARENT=notebook`, which the templates read: the
controller gets `nested_routes()`, `/notebooks/{id}/notes` (that notebook's notes, searched
and sorted like the list) and `/notebooks/{id}/notes/new` (the form with the notebook
chosen).

## The initializer line

```rust
// src/app.rs, in `impl Hooks for App`
async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
    Ok(vec![Box::new(loco_ui::loco::Initializer)])
}
```

It mounts the enhancement script (`/lui/enhance.js`) and the capability beacon
(`/lui/caps`) beside the app's routes, plus the layer that drops the stylesheet from
enhanced responses. Without it pages still work: the script 404s and every visitor gets the
baseline variant.

## A controller

`Page`, `Redirect` and Loco's `Error` all implement `IntoResponse`, so a handler returns
`Result<Page>` and uses `?` on model calls, or `Result<Response>` when branches answer
differently. Mutations are forms that post and redirect (Post/Redirect/Get), with a flash
message for the next page:

```rust
use loco_ui::prelude::*;
use loco_rs::prelude::*;

async fn show(ui: Ui, State(ctx): State<AppContext>, Path(id): Path<i64>) -> Result<Page> {
    let note = notes::Entity::find_by_id(id).one(&ctx.db).await?.ok_or(Error::NotFound)?;
    Ok(ui.page(&note.title, views::notes::show(&ui, &note)))
}

async fn remove(ui: Ui, State(ctx): State<AppContext>, Path(id): Path<i64>) -> Result<Redirect> {
    notes::Entity::delete_by_id(id).exec(&ctx.db).await?;
    Ok(ui.redirect("/notes").ok("Deleted."))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/notes")
        .add("/{id}", get(show))
        .add("/{id}/delete", post(remove))
}
```

The flash and the UI state are unsigned `lui-*` cookies, so there is no key to configure and
no clash with Loco's JWT cookie.

Paging: Loco's `query::fetch_page` (or `query::paginate`) asks for one page and the count,
and `Table::paged_from` draws the pager from its answer:

```rust
let table = ui.table("notes", "/notes").column("title", "Title");
let wanted = query::PaginationQuery { page: table.page() as u64, page_size: table.per_page() as u64 };
let found = query::fetch_page(&ctx.db, notes::Entity::find(), &wanted).await?;
let rows = found.page.iter().map(|n| Row::new([html! { (n.title) }]));
Ok(ui.page("Notes", html! { (table.rows(rows).paged_from(&found.meta)) }))
```

The module docs run the same against SeaORM's `MockDatabase`, with a sort and a filter.

## A form with validation

On bad input the handler shows the form again with what was typed and a message on each
field. `Valid<T>` is the extractor for that: where Loco's `FormValidate` answers a bad form
with an error response, `Valid` hands the handler `Ok(T)` or `Err(Invalid)` (the messages
by field and what was posted), so create and update are one `match`. The scaffold uses it.

```rust
use loco_ui::{loco::Valid, prelude::*};
use loco_rs::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
struct NewNote {
    #[validate(length(max = 80, message = "At most 80 characters."))]
    title: String,
    due: Option<Date>,
    #[serde(default, deserialize_with = "loco_ui::loco::checkbox")]
    done: bool,
}

async fn create(
    ui: Ui,
    State(ctx): State<AppContext>,
    Valid(form): Valid<NewNote>,
) -> Result<Response> {
    let note = match form {
        Ok(note) => note,
        Err(bad) => {
            let errors = bad.errors.pairs();
            let body = lui! {
                Form("/notes") values=(&bad.values) errors=(&errors) submit="Save" {
                    text "title" "Title" required;
                    date "due" "Due" "1900-01-01" "2100-12-31";
                    checkbox "done" "Done";
                }
            };
            return Ok(ui.page("New note", body).into_response());
        }
    };
    let note = notes::ActiveModel {
        title: Set(note.title), due: Set(note.due), done: Set(note.done), ..Default::default()
    }
    .insert(&ctx.db)
    .await?;
    Ok(ui.redirect(&format!("/notes/{}", note.id)).ok("Created.").into_response())
}
```

How it reads the form: values are trimmed and empty ones left out, so an empty field is
`None` for an `Option` and "This field is required." otherwise; a value that does not parse
gets "Check this field."; then the `#[validate(..)]` rules run. Every field in error gets its
message at once, not only the first. A checkbox posts `on`, which serde does not read as a
`bool`, hence `loco_ui::loco::checkbox`.

Two lower-level pieces, for handlers that do not fit a struct:

- `FieldErrors::from(&errors)` turns `validator`'s `ValidationErrors`, or the
  `ModelValidationErrors` inside `Error::Validation` / `ModelError::Validation` (a model's
  own rules, checked on save), into `(field, message)` pairs for `.errors(..)`.
- `Submitted`, from `Form<Vec<(String, String)>>`, parses field by field: `required::<T>`,
  `optional::<T>` and `checkbox`, recording the same messages as it goes. The account pages
  from `cargo lui auth` use it, since sign-in checks the pair against the database.

## Sign-in without script

`cargo lui auth` (after `cargo lui install`) writes every account page as a no-script form on
the starter's `users` model and `AuthMailer`: sign in, sign up (which mails a verification
link), sign out, forgot and reset password, email verification and magic link. It writes
`src/controllers/account.rs` and `src/views/account.rs`, registers them, points the
starter's mail links at the pages (`/verify/<token>`, `/reset/<token>`,
`/magic-link/<token>`) and does the cookie setting below in each `config/*.yaml`. The
forgot and magic-link forms give the same answer whether or not an account exists, and each
mailed link works once. `examples/loco-app` runs exactly these files, through Loco's router
and Blitz. Mail needs a `mailer:` section (`stub: true` drops every message, as the
example does in development and tests; `smtp` on 1025 with Mailpit shows the links).

Loco's `auth::JWT` extractor reads the token from a header by default, which a plain form
cannot send. Point it at a cookie and have the sign-in form set that cookie on its redirect:

```yaml
# config/development.yaml
auth:
  jwt:
    location:
      from: Cookie
      name: auth
    secret: <your secret>
    expiration: 604800
```

```rust
let token = user.generate_jwt(&jwt.secret, jwt.expiration)?;
let cookie = SetCookie::new("auth", &token, jwt.expiration).http_only().secure(ui.is_secure());
let cookie = cookie.to_string(); // `Secure` when the request came over HTTPS
Ok(ui.redirect("/").cookie(cookie).ok("Signed in."))
```

`SetCookie` is `loco_ui::cookie::SetCookie`. Signing out posts to a route that sends
`SetCookie::clear("auth")` with the same attributes.
`examples/loco-app/src/controllers/account.rs` has every account page in full.

A route with `auth::JWT` answers a signed-out visitor (no cookie, an expired token, a user
since deleted) with Loco's JSON 401. `cargo lui auth` also adds
`Box::new(loco_ui::loco::SignIn("/signin"))` to `App::initializers`, which turns that 401
into a redirect to the sign-in form with the flash "Sign in to see that page." A GET adds
`?next=` with the page asked for; the form posts back to its own URL, and the sign-in
handler redirects to `loco_ui::loco::landing(ui, HOME)`: that page when it is a path on
this site, else home. A request that accepts only JSON keeps its 401.

## The generator

`loco-ui/loco-templates/` overrides Loco's scaffold so that `cargo loco generate scaffold`
writes an HTML controller and Maud views instead of a JSON API and DTOs:

```sh
cargo lui install   # or: cp -r path/to/loco-ui/loco-ui/loco-templates .loco-templates
cargo loco generate scaffold note title:string! body:text done:bool! due:date
cargo fmt
```

That writes the migration and model as usual, then `src/controllers/notes.rs` (list with
paging, show, new, create, edit, update, delete; every change a form post and a redirect with
a flash; the form re-rendered with values and messages on bad input) and `src/views/notes.rs`
(`list`, `show`, `form`, `values`), and registers the routes in `app.rs`.

The list's search box and column headers work: `query()` in the controller reads
`ui.table_query("notes", SORTABLE)` and turns the search text into a SeaORM
`Condition::any()` of `LIKE '%text%'` over the text columns (`string` and `text` fields;
SQLite's `LIKE` ignores ASCII case, Postgres's does not, so use `ILIKE` there by hand), and
the sort into `order_by(column, Order::Desc | Asc)`, then by id. Every column is
`sortable` in the view. `/notes/export.csv` answers the same search and sort as CSV, every
row rather than one page (the table's "Download CSV" link), with a leading `=`, `+` or `@`
escaped so a spreadsheet does not run it. The show page's Delete opens a dialog that asks
first ([`RecordPage`](../loco-ui/src/blocks/record_page.rs)). The controller ends with a test of
`query()`'s SQL (`cargo test --lib`), and its flash messages and titles come from
`Text::Created`, `Saved`, `Deleted`, `New` and `Edit`, so they follow the visitor's language. The handlers take
`auth::JWT` unless you pass `--no-auth`. Needs: a `src/views/mod.rs`, a
`tests/models/mod.rs` (the model generator adds a test there), and `sea-orm-cli` 2 for the
entities step. Field kinds map to controls:

| Field | Control | Read by |
|---|---|---|
| `references` (`user:references`, except the owner above) | select of the parent's rows, labelled by `loco::label` (name, title or email) | `i64` |
| `bool` | checkbox | `loco::checkbox` |
| `date` | `type=date` | serde |
| `date_time`, `tstz` | `type=datetime-local` (a `tstz` without offset is UTC) | `loco::local` |
| `time` | `type=time` | `loco::local` |
| `decimal`, `money`, `float`, `double` | text with a number `pattern` | serde |
| `int`, `small_int`, `big_int` | `type=number` | serde |
| enums (`status:enum:todo,done`) | select | serde |
| `text` | textarea | serde |
| the rest | text | serde |

A reference is recognised by its column: an integer named `<parent>_id`, the parent's table
being the plural (`user_id` → `users`). A plain integer column with such a name is taken for
one too; rename it or edit `refs()` in the controller. The select lists every parent row; past
a few hundred, swap it for `ui.combobox(..)` with a search route. Array columns are not
supported. `examples/loco-app` scaffolds `task` with one field of each kind.

**Owned rows.** With auth on, a `user:references` field is the owner, not a choice. The form
has no `user_id` field. `create` sets it from the session (`auth.claims.pid`), and `list`,
`show`, `edit`, `update` and `delete` filter on it, so another user's row answers 404. Without
such a field, every signed-in user sees and changes every row, and the generated header says
so. `examples/loco-app`'s `tasks` are owned, and its page test checks that a second user gets
404 on the first user's task.

## Loco settings that affect pages

- `secure_headers` with the default `github` preset sends `script-src https:`: over plain
  `http://` in development the enhancement script is blocked and pages behave as without it.
- The `owasp` preset sends `Clear-Site-Data: "cache","cookies","storage"` on every response,
  which wipes the flash, the theme and the sign-in cookie on each page. Use `github`, or
  override that header.
- `cargo lui install` adds loco-ui's strict `csp` layer in `after_routes`
  (`axum::middleware::from_fn(loco_ui::enhance::csp)`). It keeps a policy an answer already
  has, so with `secure_headers` turned on (it is off unless configured) Loco's policy wins.

## Blocks and the 404 page

`loco_ui::blocks` has whole pages built from the components: `ui.app_shell(..)`,
`ui.auth_page(..)`, `ui.settings_page(..)`, `ui.record_page(..)`, `ui.dashboard_page(..)`,
`ui.pricing_page(..)`, `ui.inbox_page(..)`, `ui.invoice_page(..)` and `ui.error_page(status)` (the demo shows each under "Blocks"). The 404 page goes in as the
router's fallback, in `App::before_routes`:

```rust
async fn before_routes(_ctx: &AppContext) -> Result<axum::Router<AppContext>> {
    Ok(axum::Router::new().fallback(loco_ui::blocks::not_found))
}
```

Loco turns on its own fallback (a "Welcome to Loco!" page) outside production, and it wins;
switch it off in `config/*.yaml`:

```yaml
server:
  middlewares:
    fallback:
      enable: false
```

For a 500 from a handler, answer `ui.error_page(500).into_response()`.

## Languages

The components' own words ("Next", "Load more", month names) come from one table per
language, `loco_ui::i18n::Strings`; the request's language is the `lui-lang` cookie, else
`Accept-Language`, and `<html lang>` says it. With Loco's fluent-templates, add the keys
to each `assets/i18n/<lang>/main.ftl`:

```ftl
lui-next = Siguiente
lui-load-more = Cargar más
lui-rows-per-page = Filas por página
```

then build the tables once, in `App::boot` or an initializer (a key a file lacks stays
English):

```rust
let spanish = Strings::from_lookup("es", |key| LOCALES.try_lookup(&langid!("es"), key));
let tables: &'static [&'static Strings] = Box::leak(Box::new([&Strings::ENGLISH, spanish]));
loco_ui::i18n::languages(tables);
```

`Text::ALL` lists every key (`Text::key()`); a language switch posts to a handler that
answers `ui.redirect(back).lang("es")`. The form messages of `Valid` and `Submitted` ("This
field is required.") are English; put your own in `#[validate(message = ..)]`.

## Views: Maud, not Tera

Loco's generators write Tera templates under `assets/views/` and render them with
`format::render().view(&v, "notes/list.html", data!({..}))`. With loco-ui the views are Rust:
a `src/views/` module of plain functions that take `&Ui` and the data and return `Markup`. A
controller calls one and returns the page.

```rust
// src/views/notes.rs
use loco_rs::prelude::PagerMeta;
use loco_ui::{prelude::*, table::Row};
use crate::models::_entities::notes;

pub fn list(ui: &Ui, rows: &[notes::Model], meta: &PagerMeta) -> Markup {
    let rows = rows.iter().map(|n| Row::new([html! { a href={ "/notes/" (n.id) } { (n.title) } }]));
    lui! {
        Flash;
        Table("notes", "/notes") rows=(rows) paged_from=(meta) {
            column "title" "Title" sortable;
        }
        LinkButton("New note", "/notes/new");
    }
}

pub fn form(ui: &Ui, action: &str, title: &str, errors: &[(&str, &str)]) -> Markup {
    lui! {
        Form(action) errors=(errors) submit="Save" {
            text "title" "Title" required value=(title);
        }
    }
}
```

```rust
// src/controllers/notes.rs
async fn list(ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    let table = ui.table("notes", "");
    let wanted = query::PaginationQuery { page: table.page() as u64, page_size: table.per_page() as u64 };
    let found = query::fetch_page(&ctx.db, notes::Entity::find(), &wanted).await?;
    Ok(ui.page("Notes", views::notes::list(&ui, &found.page, &found.meta)))
}
```

Tera and Maud views live side by side: each controller picks how it renders, so an existing
app can move one controller at a time. The two need nothing from each other.

**No Tera bridge.** A Tera function such as `{{ lui_button(label="Save") }}` was considered
and left out:

- A component starts from `ui`: the browser's capabilities, the query and the `lui-ui` state
  (which tab is open, the page size, the flash). A Tera function gets only JSON arguments, so
  every template would have to pass that state through by hand, and a forgotten argument
  renders the wrong variant silently.
- Builders are typed chains (`.required()`, `.maxlength(80)`, `.column(..).sortable()`); as
  keyword arguments they lose the compiler's checks, and a typo is a runtime error on the page.
- Maud escapes by default and checks the markup at compile time; Tera output would have to be
  marked `| safe`.

A Tera page can still link to or embed a Maud-rendered fragment from its own route; there is
nothing to share at template level.
