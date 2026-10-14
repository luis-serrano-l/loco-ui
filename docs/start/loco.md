# Loco

On [Loco](https://loco.rs), controllers are Axum handlers, so they take `ui: Ui` and return a
`Page` or a `Redirect` as on Axum. The installer does the setup in one command.

## With the installer

```sh
cargo install --git https://github.com/luis-serrano-l/loco-ui loco-ui --bin cargo-lui --features cli
cd my_app
cargo lui install
```

`cargo lui install` adds the dependency with the `loco` feature, the scaffold templates, a
layout view, the app's `Look`, the 404 page, the strict CSP layer and the initializer that
serves the script and the beacon route. Then:

```sh
cargo loco generate scaffold note title:string! body:text   # HTML controller and Maud views
cargo lui auth                                              # sign-in, sign-up, reset password
cargo lui add badge                                         # copy a component into src/ui/ to edit
```

## By hand

Add the dependency:

```toml
[dependencies]
loco-ui = { git = "https://github.com/luis-serrano-l/loco-ui", features = ["loco"] }
maud = "0.27"
```

Mount the script and the beacon route with the initializer:

```rust,ignore
async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
    Ok(vec![Box::new(loco_ui::loco::Initializer)])
}
```

A controller then reads like any Axum handler:

```rust,ignore
use loco_rs::prelude::*;
use loco_ui::prelude::*;

async fn index(ui: Ui, State(ctx): State<AppContext>) -> Result<Page> {
    let notes = notes::Entity::find().all(&ctx.db).await?;
    Ok(ui.page("Notes", lui! {
        h1 { "Notes" }
        ul { @for n in &notes { li { (n.title) } } }
    }))
}
```

## Next

[Use with Loco](../loco.md) covers the rest: `loco::Valid<T>` for forms that show their
mistakes again, paged tables from the database, the generator's templates, sign-in with
Loco's JWT in a cookie, and the Loco settings that affect pages.
[`examples/loco-app`](../../examples/loco-app) is a generated app with accounts, notebooks,
tagged notes and a task board.
