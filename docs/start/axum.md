# Axum

An Axum app with one page, a form that saves with Post/Redirect/Get, and the two routes
loco-ui needs: the capability beacons and the optional script.

## Install

```toml
[dependencies]
loco-ui = { git = "https://github.com/luis-serrano-l/loco-ui", features = ["axum"] }
maud = "0.27"
axum = "0.8"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

## A page

A handler takes `ui: Ui` and returns a `Page`. `ui.page(title, body)` writes the whole
document: the stylesheet once, the theme, the skip link and the script tag.

```rust
use loco_ui::prelude::*;

async fn home(ui: Ui) -> Page {
    ui.page("Home", lui! {
        h1 { "Hello" }
        Tabs("intro") {
            tab "First" { p { "The open tab is in the URL." } }
            tab "Second" { p { "Reload: it is still open." } }
        }
        Dialog("Say hi") { p { "Opened and closed by the browser." } }
    })
}
```

## A form

The form posts; the handler answers with a redirect and a flash, which the next page shows by
itself. A mistake answers with the same page, its message beside the field and a 422.

```rust
use axum::Form;
use loco_ui::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Profile { name: String }

fn profile(ui: &Ui, name: &str, errors: &[(&str, &str)]) -> Page {
    ui.page("Profile", lui! {
        Form("/profile") submit="Save" errors=(errors) {
            text "name" "Name" required value=(name);
        }
    })
}

async fn show(ui: Ui) -> Page {
    profile(&ui, "", &[])
}

async fn save(ui: Ui, Form(p): Form<Profile>) -> Result<Redirect, Page> {
    if p.name.trim().is_empty() {
        return Err(profile(&ui, &p.name, &[("name", "Say your name.")]).invalid());
    }
    Ok(ui.redirect("/profile").ok("Saved."))
}
```

## The router

`caps::router()` serves `/lui/caps`, the beacons that tell the server what the browser
supports; `enhance::router()` serves `/lui/enhance.js`, the optional script. The `csp` layer
sends a strict Content-Security-Policy that the pages already satisfy.

```rust
use axum::{Router, routing::get};
use loco_ui::{caps, enhance, prelude::*};

async fn home(ui: Ui) -> Page {
    ui.page("Home", html! { h1 { "Hello" } })
}

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .merge(caps::router())
        .merge(enhance::router())
        .layer(axum::middleware::from_fn(enhance::csp))
}

async fn serve() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
```

The first page a browser sees is the variant every browser can show, and the beacons set a
cookie per capability as it loads; from the next request on, each component sends the variant
that browser supports. `loco-ui/examples/axum_server.rs` in the repository is this app, ready to
run.

## Next

- Keep a value between requests with `Saved<T>` (a serde value in a cookie) and
  `redirect.save(&value)`; read posts by name with `Posted`, files included.
- Give the app its look once with a `Look` layer: [Theming](../theming.md).
- Every component page on this site shows the code that draws it and a table of its props.
