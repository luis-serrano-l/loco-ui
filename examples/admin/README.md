# admin: an admin panel on plain Axum and SQLite

Users, roles and an audit log, built from loco-ui's page blocks with no framework around the
router: Axum handlers, `sqlx` over one SQLite file, Argon2id for passwords. Every page works
with script off; the only script is loco-ui's optional one.

```sh
cargo run -p admin        # http://127.0.0.1:3002
```

The first start creates `admin.sqlite` beside where you ran it and seeds ten users and four
roles. Sign in as `ada@example.com` with the password `analytical-engine`. `PORT` and
`ADMIN_DB` (an sqlx URL, `sqlite::memory:` for a throwaway run) change the defaults.

## What is where

| File | Pages | loco-ui parts |
|------|-------|---------------|
| `src/auth.rs` | `/signin`, `POST /signout`; the `Admin` extractor | `auth_page`, `form` |
| `src/users.rs` | `/users`, `/users/new`, `/users/{id}`, `/users/{id}/edit` | `list_page` (search, filters with counts), `table`, `record_page` (Delete behind a dialog), `form` with a select and a switch |
| `src/roles.rs` | `/roles` | `list_page`, `table`, `dialog` holding the form, a confirm dialog per unused role |
| `src/audit.rs` | `/audit` | `list_page`, read-only |
| `src/settings.rs` | `/settings` | `settings_page`, two forms |
| `src/views.rs` | the shell around every signed-in page | `app_shell`, `badge`, `empty_state` |
| `src/db.rs` | the schema, the seed and every query | |

Each handler takes `ui: Ui`, `admin: Admin` and what it reads, and returns a `Page` or a
`Redirect`. A form with mistakes comes back as `Err(page.invalid())` (422) with what was
typed; a change that worked redirects with a toast (`ui.redirect(..).ok(..)`).

## How sign-in works

A password is stored as an Argon2id hash. Signing in writes a random token to the `sessions`
table and to an `HttpOnly`, `SameSite=Lax` cookie; `Admin` looks the token up on every
request and redirects to `/signin` when there is none. Suspending a user or changing a
password deletes their other sessions. A wrong password, an unknown address and a suspended
account get the same message.

Before putting something like this on the internet: add `Secure` to the cookie (left off so
the example works over `http://localhost`), limit sign-in attempts, expire old sessions, and
check roles in the handlers. Here every signed-in user may do everything; the roles are
data, not permissions.

## Tests

```sh
cargo test -p admin
```

`tests/pages.rs` drives the router over an in-memory database: signed-out visitors are
redirected, the list searches and filters, a user is created, edited, suspended and deleted,
roles and settings change, and every page ships one script and is laid out by Blitz into
`tests/shots/admin-*.png` (the pictures in the demo's examples gallery).
