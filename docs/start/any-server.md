# Any server

Without a framework feature the crate depends on Maud and its own two crates only. Every
part of the Axum glue is a plain function over strings, so any server, or a program that
writes static files, can do the same in a few lines.

## Install

```toml
[dependencies]
loco-ui = { git = "https://github.com/luis-serrano-l/loco-ui" }
maud = "0.27"
```

Add `features = ["http"]` for `http::Response` answers and streamed pages.

## A page from a request

`Ui::from_request` takes the path, the query string and the `Cookie:` header, and gives the
same `ui` an Axum handler gets. `into_string()` is the HTML to send; `set_cookies()` the
`Set-Cookie` values to send with it.

```rust
use loco_ui::prelude::*;

let ui = Ui::from_request("/", "tab.intro=1", "theme=dark");
let page = ui.page("Home", lui! {
    h1 { "Hello" }
    Tabs("intro") {
        tab "First" { p { "One" } }
        tab "Second" { p { "Two" } }
    }
});
let cookies: Vec<String> = page.set_cookies().to_vec();
let html = page.into_string();
assert!(html.starts_with("<!DOCTYPE html>"));
assert!(html.contains(r#"data-theme="dark""#));
assert!(cookies.iter().all(|c| c.contains('=')));
```

## A post and a redirect

`ui.redirect(to)` is a 303 with its flash and cookies: send `location()` as the `Location`
header and each of `set_cookies()` as a `Set-Cookie`.

```rust
use loco_ui::prelude::*;

let ui = Ui::from_request("/profile", "", "");
let redirect = ui.redirect("/profile").ok("Saved.");
assert_eq!(redirect.location(), "/profile");
assert!(!redirect.set_cookies().is_empty(), "the flash travels in a cookie");
```

## The two routes

- **`GET /lui/caps?flag=<name>`**: `caps::beacon_cookie(query, secure)` gives the
  `Set-Cookie` to answer with a 204, or `None` for a 404. Send `Cache-Control: no-store`.
- **`GET /lui/enhance.js`**: the body is `enhance::served()`, as `text/javascript`. Leave it
  out and every page still works; it only makes forms and links update in place.

`loco-ui/examples/hyper_server.rs` in the repository is all of this on raw hyper: three
components, the beacon route, a post answered with Post/Redirect/Get and the script served by
hand. To use the components inside another template engine, call `.render().into_string()` on
any of them.
