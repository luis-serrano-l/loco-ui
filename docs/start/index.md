# Getting started

loco-ui is a library of server-rendered components for Rust web servers. A handler builds a
page from components and returns HTML; the browser does the interactive work with
`<dialog>`, `popover`, `<details>` and forms that post and redirect. Every page works with
JavaScript off, and one optional script makes the same forms and links update the page in
place.

## What you need

- Rust 1.88 or newer (1.94 for the `loco` feature, which loco-rs 1.2 asks for).
- [Maud](https://maud.lambda.xyz), the template macro the components render into. `lui!` and
  `html!` expand to `maud::` paths, so your crate depends on `maud` too.
- A server. Pick the guide for yours:

| Your server | Guide | Feature |
|---|---|---|
| Axum | [Axum](axum.md) | `axum` |
| Loco | [Loco](loco.md) | `loco` |
| Anything else (hyper, actix-web, a static site generator) | [Any server](any-server.md) | none, or `http` |

## Install

loco-ui is not on crates.io yet. Until the 0.2.0 release, depend on the repository:

```toml
[dependencies]
loco-ui = { git = "https://github.com/luis-serrano-l/loco-ui", features = ["axum"] }
maud = "0.27"
```

Swap the feature for the one your guide names. After the release, `cargo add loco-ui
--features axum` does the same.

## The three ideas

**`Ui` is the request.** A handler gets one `Ui` value: what the browser supports (`Caps`),
the theme, the UI state in the query and cookies, and the visitor's language. Every component
starts from it, `ui.button("Save")`, `ui.dialog("Delete account")`, and reads its own state
from it, so a handler passes only what differs.

**Components are builders.** Required arguments go in the call, everything else is a chained
setter: `ui.dialog("Delete").danger().confirm("Delete", "/delete")`. Inside `lui!` the same
call is written like an element: `Dialog("Delete") danger confirm=("Delete", "/delete")`.

**State lives on the server.** An open tab, a page number or a sort order is in the URL or a
cookie; a change is a form post answered with a redirect (Post/Redirect/Get). That is why a
page works with script off, and why a reload or a shared link shows the same thing.

## Next

- [Write your own component](../components.md) from the same primitives.
- [Theming](../theming.md): every colour, size and corner is a `--lui-*` token.
- [State](../state.md): what travels in the URL, what in cookies.
- [API rules](../api.md): how setters are named and why.
- [Capabilities](../caps.md): how the server learns what the browser supports, with no script.
