//! The edits `install` and `auth` make to files the app already has. Each takes the file's
//! text and returns the new text, the same text when the edit is already there.

use toml_edit::{DocumentMut, Item, Value};

pub const INITIALIZER: &str = "Box::new(loco_ui::loco::Initializer)";
/// Signed-out visits to signed-in pages go to the sign-in form (`auth`).
pub const SIGN_IN: &str = "Box::new(loco_ui::loco::SignIn(\"/signin\"))";
const ACCOUNT_ROUTE: &str = ".add_route(controllers::account::routes())";
/// The starter's mail links (to its JSON API and SPA) and where the account pages answer.
const MAIL_LINKS: [(&str, &str); 3] = [
    ("{{host}}/api/auth/verify/", "{{host}}/verify/"),
    ("{{host}}/reset#", "{{host}}/reset/"),
    ("{{host}}/api/auth/magic-link/", "{{host}}/magic-link/"),
];
const JWT_LOCATION: &str = "    # loco-ui's account pages keep the token in this cookie.
    location:
      from: Cookie
      name: auth
";
const NOT_FOUND: &str = "loco_ui::blocks::not_found";
const CSP: &str = "loco_ui::enhance::csp";
const BEFORE_ROUTES: &str =
    "    /// loco-ui: every path no route answers gets the 404 page, in the app's look.
    async fn before_routes(_ctx: &AppContext) -> Result<axum::Router<AppContext>> {
        Ok(axum::Router::new().fallback(loco_ui::blocks::not_found))
    }

";
const AFTER_ROUTES: &str =
    "    /// loco-ui: the app's look (`src/views/look.rs`) on every page, and the strict
    /// Content-Security-Policy on every HTML answer that has none of its own.
    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        Ok(router
            .layer(crate::views::look::LOOK.layer())
            .layer(axum::middleware::from_fn(loco_ui::enhance::csp)))
    }

";

/// Register the account routes right after `AppRoutes::..` in `fn routes`.
pub fn add_account_route(contents: &str) -> Result<String, String> {
    if contents.contains(ACCOUNT_ROUTE) {
        return Ok(contents.to_string());
    }
    let missing = || format!("no `AppRoutes::` in `fn routes`; add `{ACCOUNT_ROUTE}` by hand");
    let at = contents.find("fn routes").ok_or_else(missing)?;
    let at = at + contents[at..].find("AppRoutes::").ok_or_else(missing)?;
    let end = at + contents[at..].find('\n').ok_or_else(missing)? + 1;
    Ok(format!(
        "{}            {ACCOUNT_ROUTE}\n{}",
        &contents[..end],
        &contents[end..]
    ))
}

/// Point the starter's mail links at the account pages.
pub fn point_mail_links(contents: &str) -> String {
    MAIL_LINKS
        .iter()
        .fold(contents.to_string(), |text, (from, to)| {
            text.replace(from, to)
        })
}

/// Add the cookie location under `auth:` → `jwt:` unless that block names a location.
pub fn add_jwt_location(contents: &str) -> Result<String, String> {
    let lines: Vec<&str> = contents.split_inclusive('\n').collect();
    let Some(auth) = lines.iter().position(|l| l.trim_end() == "auth:") else {
        return Ok(contents.to_string());
    };
    // The block runs to the next line that starts in the first column (not a comment).
    let end = lines[auth + 1..]
        .iter()
        .position(|l| l.starts_with(|c: char| !c.is_whitespace() && c != '#'))
        .map_or(lines.len(), |i| auth + 1 + i);
    let block = &lines[auth..end];
    if block
        .iter()
        .any(|l| l.trim_start().starts_with("location:"))
    {
        return Ok(contents.to_string());
    }
    let Some(jwt) = block.iter().position(|l| l.trim_end() == "  jwt:") else {
        return Ok(contents.to_string());
    };
    let at = auth + jwt + 1;
    Ok(format!(
        "{}{JWT_LOCATION}{}",
        lines[..at].concat(),
        lines[at..].concat()
    ))
}

/// Append `line` unless a line reads exactly that.
pub fn add_line(contents: &str, line: &str) -> Result<String, String> {
    if contents.lines().any(|l| l.trim() == line) {
        return Ok(contents.to_string());
    }
    let mut out = contents.to_string();
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(line);
    out.push('\n');
    Ok(out)
}

/// Put `init` in the `vec![..]` that `fn initializers` returns: right after loco-ui's own
/// initializer when that is there, else first.
pub fn add_initializer(contents: &str, init: &str) -> Result<String, String> {
    if contents.contains(init) {
        return Ok(contents.to_string());
    }
    if let Some(at) = contents.find(INITIALIZER) {
        let at = at + INITIALIZER.len();
        return Ok(format!("{}, {init}{}", &contents[..at], &contents[at..]));
    }
    let missing = || format!("no `vec![` in `fn initializers`; add `{init}` to it by hand");
    let at = contents.find("fn initializers").ok_or_else(missing)?;
    let open = at + contents[at..].find("vec![").ok_or_else(missing)? + "vec![".len();
    let empty = contents[open..].trim_start().starts_with(']');
    let insert = if empty {
        init.to_string()
    } else {
        format!("{init}, ")
    };
    Ok(format!(
        "{}{insert}{}",
        &contents[..open],
        &contents[open..]
    ))
}

/// Add the `before_routes` hook (the 404 fallback) and the `after_routes` hook (the look and
/// the CSP layer) to `impl Hooks for App`, before `fn routes`. A hook the app already defines
/// is left alone, with a note saying what to add to it.
pub fn add_hooks(contents: &str) -> Result<String, String> {
    let mut out = contents.to_string();
    for (hook, text, needs) in [
        ("fn before_routes", BEFORE_ROUTES, NOT_FOUND),
        ("fn after_routes", AFTER_ROUTES, CSP),
    ] {
        if out.contains(needs) {
            continue;
        }
        if out.contains(hook) {
            eprintln!("note: `{hook}` is the app's own; add `{needs}` to it by hand");
            continue;
        }
        let missing = || format!("no `fn routes` in `impl Hooks`; add `{hook}` by hand");
        let at = out.find("fn routes(").ok_or_else(missing)?;
        let line = out[..at].rfind('\n').map_or(0, |i| i + 1);
        out.insert_str(line, text);
    }
    Ok(out)
}

/// Add `name = value` to `[dependencies]` unless `name` is already a dependency there. The
/// manifest is edited with `toml_edit`, so comments, layout and dotted keys are kept.
pub fn add_dependency(contents: &str, name: &str, value: &str) -> Result<String, String> {
    let mut doc: DocumentMut = contents.parse().map_err(|e| format!("{e}"))?;
    let value: Value = value
        .parse()
        .map_err(|e| format!("{name} = {value}: {e}"))?;
    let deps = doc
        .entry("dependencies")
        .or_insert_with(toml_edit::table)
        .as_table_like_mut()
        .ok_or("[dependencies] is not a table")?;
    if deps.contains_key(name) {
        return Ok(contents.to_string());
    }
    deps.insert(name, Item::Value(value));
    // Only `[dependencies.x]` tables before: the parent needs its own header now.
    if let Some(table) = doc.get_mut("dependencies").and_then(Item::as_table_mut) {
        table.set_implicit(false);
    }
    Ok(doc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_initializer_goes_first_once() {
        let fresh = "async fn initializers(_ctx: &AppContext) -> Result<..> {\n    Ok(vec![])\n}";
        let once = add_initializer(fresh, INITIALIZER).unwrap();
        assert!(once.contains("Ok(vec![Box::new(loco_ui::loco::Initializer)])"));
        assert_eq!(add_initializer(&once, INITIALIZER).unwrap(), once);
        let other = "fn initializers() { Ok(vec![Box::new(Mine)]) }";
        let added = add_initializer(other, INITIALIZER).unwrap();
        assert!(added.contains("vec![Box::new(loco_ui::loco::Initializer), Box::new(Mine)]"));
        assert!(add_initializer("fn routes() {}", INITIALIZER).is_err());
        // `auth`'s sign-in initializer goes right after it.
        let signed = add_initializer(&added, SIGN_IN).unwrap();
        assert!(signed.contains(r#"vec![Box::new(loco_ui::loco::Initializer), Box::new(loco_ui::loco::SignIn("/signin")), Box::new(Mine)]"#));
        assert_eq!(add_initializer(&signed, SIGN_IN).unwrap(), signed);
    }

    #[test]
    fn a_dependency_is_added_once_and_only_under_dependencies() {
        let toml = "[workspace.dependencies]\nmaud = \"1\"\n\n[dependencies]\n# keep me\nserde = \"1\"\n\n[dev-dependencies]\n";
        let added = add_dependency(toml, "maud", r#""0.27""#).unwrap();
        assert!(
            added.contains("[dependencies]\n# keep me\nserde = \"1\"\nmaud = \"0.27\"\n"),
            "{added}"
        );
        assert_eq!(add_dependency(&added, "maud", r#""0.27""#).unwrap(), added);
        // Dotted keys and tables count as the dependency being there.
        let dotted = "[dependencies]\nloco-ui.path = \"x\"\n";
        assert_eq!(add_dependency(dotted, "loco-ui", "\"1\"").unwrap(), dotted);
        let table = "[dependencies.serde]\nversion = \"1\"\n";
        let added = add_dependency(table, "maud", r#""0.27""#).unwrap();
        assert!(
            added.contains("[dependencies]\nmaud = \"0.27\"\n"),
            "{added}"
        );
        assert!(
            added.contains("[dependencies.serde]\nversion = \"1\"\n"),
            "{added}"
        );
        let inline = add_dependency(
            "[package]\nname = \"a\"\n",
            "loco-ui",
            r#"{ path = "../loco-ui", features = ["loco"] }"#,
        )
        .unwrap();
        assert!(
            inline.ends_with(
                "[dependencies]\nloco-ui = { path = \"../loco-ui\", features = [\"loco\"] }\n"
            ),
            "{inline}"
        );
        assert!(add_dependency("[dependencies\n", "maud", "\"1\"").is_err());
    }

    #[test]
    fn the_jwt_reads_the_cookie() {
        let yaml =
            "server:\n  port: 1\nauth:\n  # JWT\n  jwt:\n    secret: x\n\ndatabase:\n  uri: y\n";
        let added = add_jwt_location(yaml).unwrap();
        assert!(added.contains("  jwt:\n    # loco-ui's account pages keep the token in this cookie.\n    location:\n      from: Cookie\n      name: auth\n    secret: x"));
        assert_eq!(add_jwt_location(&added).unwrap(), added);
        assert_eq!(add_jwt_location("server: {}\n").unwrap(), "server: {}\n");
    }

    #[test]
    fn the_account_route_follows_app_routes() {
        let app = "fn routes(_ctx: &AppContext) -> AppRoutes {\n        AppRoutes::with_default_routes() // controller routes below\n            .add_route(controllers::auth::routes())\n    }";
        let added = add_account_route(app).unwrap();
        assert!(added.contains("below\n            .add_route(controllers::account::routes())\n            .add_route(controllers::auth"));
        assert_eq!(add_account_route(&added).unwrap(), added);
    }

    #[test]
    fn the_hooks_go_before_routes_once() {
        let app = "impl Hooks for App {\n    fn app_name() {}\n\n    fn routes(_ctx: &AppContext) -> AppRoutes {\n    }\n}\n";
        let added = add_hooks(app).unwrap();
        assert!(
            added.contains("fn app_name() {}\n\n    /// loco-ui: every path"),
            "{added}"
        );
        assert!(added.contains(".fallback(loco_ui::blocks::not_found)"));
        assert!(added.contains("    }\n\n    fn routes(_ctx"), "{added}");
        assert!(added.find("before_routes") < added.find("after_routes"));
        assert_eq!(add_hooks(&added).unwrap(), added);
        // The app's own hook stays as it is.
        let own = app.replace(
            "fn routes",
            "async fn after_routes(r: Router) {}\n    fn routes",
        );
        let added = add_hooks(&own).unwrap();
        assert!(added.contains("before_routes") && !added.contains(CSP));
    }

    #[test]
    fn mail_links_point_at_the_pages() {
        let text = "{{host}}/api/auth/verify/{{verifyToken}} {{host}}/reset#{{resetToken}}";
        assert_eq!(
            point_mail_links(text),
            "{{host}}/verify/{{verifyToken}} {{host}}/reset/{{resetToken}}"
        );
    }

    #[test]
    fn a_module_line_is_appended_once() {
        let once = add_line("pub mod auth;", "pub mod layout;").unwrap();
        assert_eq!(once, "pub mod auth;\npub mod layout;\n");
        assert_eq!(add_line(&once, "pub mod layout;").unwrap(), once);
    }
}
