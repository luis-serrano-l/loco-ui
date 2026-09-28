# loco-app

A notes app on [Loco](https://loco.rs) and loco-ui, every page working with script off:

- **Notes** in notebooks, with tags, pinned or archived, each user's own. The list is cards
  or a table, filtered by tag or searched; a note reads in a narrow column with its details
  beside it.
- **An overview**: counts, notes written per week, what is due, the latest notes.
- **Tasks** on a board (move a card to change its status) or in a table.
- **Accounts**: sign up, sign in, forgot and reset password, email verification, magic link.

```sh
cd examples/loco-app
cargo loco start          # http://localhost:5150, the sqlite file created and migrated
cargo dev-app             # the same, rebuilt and restarted on every change (needs cargo-watch)
cargo loco start -d       # the same in the background (log in target/debug/loco-app.log); `cargo loco stop` ends it
cargo test -p loco-app    # every page through Loco's router and Blitz
```

The first start in development, with no users yet, seeds a demo account: sign in as
`ada@example.com` / `analytical-engine`. `cargo loco db seed --reset` puts it back.
`src/seed.rs` writes it in Rust so the dates follow today.

What wrote what:

- **Generated, then edited**: notebooks and tasks, by `cargo loco generate scaffold` with the
  templates in `.loco-templates/` (a copy of `loco-ui/loco-templates/`, kept identical by a
  test in `loco-ui`). The edits put the pages in the app shell, make the task list a board,
  and list a notebook's notes on its page.

  ```sh
  cargo loco generate scaffold notebook name:string! user:references
  cargo loco generate model tag name:string! user:references
  ```

  `db entities` in that step needs `sea-orm-cli` 2 on `PATH`.
- **By hand**: notes (`src/controllers/notes.rs`, `src/views/notes.rs`, the migration that
  recreates the table), the overview, the shell (`src/views/shell.rs`), the look
  (`src/views/look.rs`, one layer in `App::after_routes`) and the seed.
- **`cargo lui auth`**: `src/controllers/account.rs` and `src/views/account.rs`, on the
  starter's `users` model and `AuthMailer` (`src/mailers/`); a test in `loco-ui` fails if they
  drift from the templates. Development and tests send no mail (`mailer.stub`); set
  `stub: false` in `config/development.yaml` and run Mailpit on 1025 to read the links.

The notes migration drops and recreates the `notes` table (SQLite cannot add a NOT NULL
`user_id` to a table with rows), so notes written before it are gone.
