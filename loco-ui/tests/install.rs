//! `cargo lui` on a fresh Loco app. `tests/fresh-loco-app` is
//! `loco new -n fresh_app --db sqlite --bg blocking --assets none` (loco 1.2.0) trimmed to
//! what `cargo check`, the installer and `generate scaffold` read: `Cargo.toml`, `Cargo.lock`,
//! `.cargo/config.toml` (the `loco-tool` alias the generator runs), `config/`, `src/`,
//! `migration/` and `tests/models/mod.rs` (where the generator adds each model's test).
//!
//! The tests marked `#[ignore]` build what the commands write (Loco and SeaORM, a few minutes
//! the first time), so CI runs them with `--include-ignored`; they share one target directory.

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fresh-loco-app");
const SOURCE: &str = env!("CARGO_MANIFEST_DIR");

/// A copy of the fixture in `target/`, so every run starts from `loco new`'s files.
fn fresh_copy(name: &str) -> PathBuf {
    let dir = scratch(name);
    copy_dir(Path::new(FIXTURE), &dir);
    dir
}

/// An empty `target/lui-install/<name>`.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(SOURCE).join("../target/lui-install").join(name);
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// Run the installer on `app` against this checkout; its stdout.
fn install(app: &Path) -> String {
    lui(&["install"], app, &["--dep-path", SOURCE])
}

/// `cargo lui <command> <app> <rest..>`; its stdout, after checking it succeeded.
fn lui(command: &[&str], app: &Path, rest: &[&str]) -> String {
    let out = run(command, Some(app), rest);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn run(command: &[&str], app: Option<&Path>, rest: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-lui"))
        .arg("lui")
        .args(command)
        .args(app)
        .args(rest)
        .env("CARGO_TARGET_DIR", target())
        .output()
        .unwrap()
}

/// `cargo lui add <names..> --app <app> <rest..>`; its stdout.
fn add(app: &Path, names: &[&str], rest: &[&str]) -> String {
    let app = app.to_str().unwrap();
    let out = run(
        &[&["add"], names, &["--app", app], rest].concat(),
        None,
        &[],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn read(app: &Path, file: &str) -> String {
    fs::read_to_string(app.join(file)).unwrap()
}

fn target() -> PathBuf {
    Path::new(SOURCE).join("../target/lui-install-target")
}

/// `cargo <args>` in `app`, checked.
fn cargo(app: &Path, args: &[&str]) -> String {
    let out = Command::new(env!("CARGO"))
        .args(args)
        .current_dir(app)
        .env("CARGO_TARGET_DIR", target())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "cargo {}: {}{}",
        args.join(" "),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn install_writes_every_piece_once() {
    let app = fresh_copy("once");
    let first = install(&app);
    assert_eq!(first.matches("wrote").count(), 4, "{first}");
    assert_eq!(first.matches("edited").count(), 3, "{first}");
    assert_eq!(
        read(&app, ".loco-templates/scaffold/api/controller.t"),
        include_str!("../loco-templates/scaffold/api/controller.t")
    );
    let routes = read(&app, "src/app.rs");
    assert!(routes.contains("Ok(vec![Box::new(loco_ui::loco::Initializer)])"));
    assert!(
        routes.contains(".fallback(loco_ui::blocks::not_found)"),
        "{routes}"
    );
    assert!(
        routes.contains(".layer(axum::middleware::from_fn(loco_ui::enhance::csp))"),
        "{routes}"
    );
    assert!(
        read(&app, "src/views/mod.rs").contains("pub mod auth;\npub mod layout;\npub mod look;\n")
    );
    assert!(read(&app, "src/views/layout.rs").contains("pub fn page(ui: &Ui"));
    assert!(read(&app, "src/views/look.rs").contains("pub static LOOK: Look"));
    let toml = read(&app, "Cargo.toml");
    assert!(toml.contains("\nloco-ui = { path = "), "{toml}");
    assert!(toml.contains("maud = \"0.27\""), "{toml}");

    let second = install(&app);
    assert!(
        !second.contains("wrote") && !second.contains("edited"),
        "{second}"
    );
    assert_eq!(second.matches("unchanged").count(), 7, "{second}");
}

#[test]
fn a_dry_run_writes_nothing() {
    let app = fresh_copy("dry");
    let before = read(&app, "src/app.rs");
    let out = lui(&["install"], &app, &["--dep-path", SOURCE, "--dry-run"]);
    assert_eq!(out.matches("would write").count(), 4, "{out}");
    assert_eq!(out.matches("would edit").count(), 3, "{out}");
    assert_eq!(read(&app, "src/app.rs"), before);
    assert!(!app.join("src/views/look.rs").exists());
}

#[test]
fn a_manifest_with_crlf_and_dotted_tables_keeps_them() {
    let app = fresh_copy("crlf");
    let toml =
        "[package]\r\nname = \"fresh_app\"\r\n\r\n[dependencies.loco-rs]\r\nversion = \"1.2\"\r\n";
    fs::write(app.join("Cargo.toml"), toml).unwrap();
    install(&app);
    let after = read(&app, "Cargo.toml");
    assert!(!after.replace("\r\n", "").contains('\n'), "{after:?}");
    assert!(
        after.contains("[dependencies]\r\nloco-ui = { path = "),
        "{after:?}"
    );
    assert!(
        after.contains("[dependencies.loco-rs]\r\nversion = \"1.2\"\r\n"),
        "{after:?}"
    );
}

#[test]
fn help_and_version() {
    let help = run(&["--help"], None, &[]);
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(
        help.contains("cargo lui add NAME..") && help.contains("--dry-run"),
        "{help}"
    );
    let version = String::from_utf8(run(&["--version"], None, &[]).stdout).unwrap();
    assert!(
        version.starts_with(concat!("cargo-lui ", env!("CARGO_PKG_VERSION"))),
        "{version}"
    );
    let unknown = run(&["install", "--nope"], None, &[]);
    assert_eq!(unknown.status.code(), Some(2));
}

#[test]
fn the_git_dependency_is_pinned_to_a_commit() {
    let app = fresh_copy("rev");
    lui(&["install"], &app, &["--rev", "0123abc"]);
    let toml = read(&app, "Cargo.toml");
    assert!(
        toml.contains(r#"loco-ui = { git = "https://github.com/luis-serrano-l/loco-ui", rev = "0123abc", features = ["loco"] }"#),
        "{toml}"
    );
}

#[test]
fn auth_writes_the_account_pages_once() {
    let app = fresh_copy("auth");
    install(&app);
    let first = lui(&["auth"], &app, &[]);
    assert_eq!(first.matches("wrote").count(), 2, "{first}");
    assert_eq!(
        read(&app, "src/controllers/account.rs"),
        include_str!("../loco-templates/auth/controller.rs")
    );
    assert!(read(&app, "src/controllers/mod.rs").contains("pub mod account;"));
    assert!(read(&app, "src/views/mod.rs").contains("pub mod account;"));
    let routes = read(&app, "src/app.rs");
    assert!(
        routes.contains("below\n            .add_route(controllers::account::routes())\n"),
        "{routes}"
    );
    assert!(
        routes.contains(r#"vec![Box::new(loco_ui::loco::Initializer), Box::new(loco_ui::loco::SignIn("/signin"))]"#),
        "{routes}"
    );
    let welcome = read(&app, "src/mailers/auth/welcome/text.t");
    assert!(
        welcome.contains("{{host}}/verify/{{verifyToken}}"),
        "{welcome}"
    );
    assert!(read(&app, "src/mailers/auth/forgot/html.t").contains("{{host}}/reset/{{resetToken}}"));
    for config in ["development", "production", "test"] {
        let yaml = read(&app, &format!("config/{config}.yaml"));
        assert!(yaml.contains("  jwt:\n    # loco-ui's account pages keep the token in this cookie.\n    location:\n      from: Cookie\n      name: auth\n"), "{config}");
    }

    let second = lui(&["auth"], &app, &[]);
    assert!(
        !second.contains("wrote") && !second.contains("edited"),
        "{second}"
    );
}

#[test]
fn auth_needs_the_dependency_first() {
    let app = fresh_copy("auth-first");
    let out = run(&["auth"], Some(&app), &[]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("run `cargo lui install` first"));
}

#[test]
fn an_edited_file_is_kept() {
    let app = fresh_copy("edited");
    install(&app);
    fs::write(app.join("src/views/layout.rs"), "// mine\n").unwrap();
    assert!(install(&app).contains("kept"));
    assert_eq!(read(&app, "src/views/layout.rs"), "// mine\n");
}

#[test]
fn a_directory_that_is_not_a_loco_app_is_refused() {
    let out = run(&["install", SOURCE], None, &[]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("not a Loco app"));
}

/// `add` copies the file whole under the prefix, registers it once, and `--diff` shows an
/// edit against loco-ui's copy.
#[test]
fn add_copies_a_component_into_the_app() {
    let app = fresh_copy("add");
    install(&app);
    let dry = add(&app, &["badge"], &["--dry-run"]);
    assert!(
        dry.contains("would write") && !app.join("src/ui").exists(),
        "{dry}"
    );
    let out = add(&app, &["badge", "record_page"], &[]);
    assert!(out.contains("ui.app_badge(..)"), "{out}");
    let copy = read(&app, "src/ui/app_badge.rs");
    assert!(
        copy.starts_with("//! # AppBadge\n//!\n//! Copied from loco-ui"),
        "{copy}"
    );
    assert!(copy.contains("//! # use fresh_app::ui::*;"));
    assert!(copy.contains("pub trait AppBadgeExt {") && copy.contains("impl AppBadgeExt for Ui {"));
    assert!(copy.contains("pub const PROPS") && copy.contains(".app-badge {"));
    let blocks = read(&app, "src/ui/app_record_page.rs");
    assert!(blocks.contains("pub trait AppRecordPageExt"));
    let modules = read(&app, "src/ui/mod.rs");
    assert!(
        modules.contains("pub mod app_badge;\npub use app_badge::{AppBadge, AppBadgeExt};\n"),
        "{modules}"
    );
    assert!(read(&app, "src/lib.rs").contains("pub mod ui;"));
    let look = read(&app, "src/views/look.rs");
    assert!(
        look.contains("css: &[crate::ui::app_badge::CSS, crate::ui::app_record_page::CSS],"),
        "{look}"
    );

    let again = add(&app, &["badge"], &[]);
    assert!(
        !again.contains("wrote") && !again.contains("edited"),
        "{again}"
    );
    assert!(add(&app, &["badge"], &["--diff"]).contains("same"));
    fs::write(
        app.join("src/ui/app_badge.rs"),
        copy.replace("A short label", "A label"),
    )
    .unwrap();
    let diff = add(&app, &["badge"], &["--diff"]);
    assert!(
        diff.contains("-//! A short label") && diff.contains("+//! A label"),
        "{diff}"
    );
    assert!(add(&app, &["badge"], &[]).contains("kept"));
    assert!(add(&app, &["badge"], &["--view"]).starts_with("//! # AppBadge"));

    let refused = run(&["add", "table", "--app", app.to_str().unwrap()], None, &[]);
    assert!(String::from_utf8_lossy(&refused.stderr).contains("cannot be copied yet"));
}

#[test]
fn list_and_info() {
    let list = String::from_utf8(run(&["list"], None, &[]).stdout).unwrap();
    assert!(
        list.contains("badge") && !list.contains("enhance"),
        "{list}"
    );
    let info = String::from_utf8(run(&["info", "badge"], None, &[]).stdout).unwrap();
    assert!(
        info.contains("Badge (stable, `Badge` in lui!)") && info.contains("shimmer"),
        "{info}"
    );
}

#[test]
fn eject_copies_the_library_into_vendor() {
    let app = fresh_copy("eject-files");
    install(&app);
    lui(&["eject"], &app, &[]);
    let toml = read(&app, "Cargo.toml");
    assert!(
        toml.contains(r#"loco-ui = { path = "vendor/loco-ui", features = ["loco"] }"#),
        "{toml}"
    );
    assert!(toml.contains("exclude = [\"vendor\"]"), "{toml}");
    let lib = read(&app, "vendor/loco-ui/Cargo.toml");
    assert!(
        lib.contains("maud = { version = \"0.27\", features = [\"axum\"] }"),
        "{lib}"
    );
    assert!(app.join("vendor/loco-ui/src/badge.rs").is_file());
    assert!(app.join("vendor/loco-ui-macros/src/lib.rs").is_file());
    assert!(!app.join("vendor/loco-ui/tests/fresh-loco-app").exists());
}

#[test]
fn new_writes_an_axum_starter() {
    let app = scratch("new-axum");
    let out = lui(&["new"], &app, &["--dep-path", SOURCE]);
    assert!(out.contains("cargo run"), "{out}");
    assert!(read(&app, "Cargo.toml").contains("name = \"new_axum\""));
    assert!(read(&app, "src/main.rs").contains("new_axum::router()"));
    assert!(read(&app, "src/lib.rs").contains(".layer(axum::middleware::from_fn(enhance::csp))"));
    let refused = run(&["new"], Some(&app), &[]);
    assert!(String::from_utf8_lossy(&refused.stderr).contains("not empty"));
}

/// With the account pages, every component `add` can copy, and three scaffolds from the
/// installed templates: one field of every kind they tell apart (text, dates and times,
/// numbers, a bool, enums required and optional, the owner's `user_id`, which the form leaves
/// out), then a required reference nested under its parent (`--parent gadget`) and an
/// optional one. `cargo test --lib` runs each controller's generated test (the search and
/// sort SQL), `--doc` the copied components' doctests.
/// The generator runs `sea-orm-cli` for the entities, so that must be installed.
#[test]
#[ignore]
fn the_installed_app_compiles() {
    let app = fresh_copy("check");
    install(&app);
    lui(&["auth"], &app, &[]);
    let copyable: Vec<&str> = include_str!("../src/bin/cargo-lui/add.rs")
        .split("pub const COPYABLE")
        .nth(1)
        .and_then(|s| s.split("];").next())
        .unwrap()
        .lines()
        .filter_map(|l| l.trim().strip_prefix('"')?.strip_suffix("\","))
        .map(|m| m.trim_start_matches("blocks/"))
        .collect();
    assert!(copyable.len() > 20, "{copyable:?}");
    add(&app, &copyable, &[]);
    let scaffold = |fields: &[&str], rest: &[&str]| {
        let mut args = vec!["scaffold"];
        args.extend(fields);
        args.extend(["--app", app.to_str().unwrap()]);
        args.extend(rest);
        let out = run(&args, None, &[]);
        assert!(
            out.status.success(),
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    };
    scaffold(
        &[
            "gadget",
            "name:string!",
            "body:text",
            "due:date!",
            "at:tstz",
            "starts:tstz!",
            "opens:time",
            "price:float",
            "qty:int!",
            "size:small_int",
            "done:bool!",
            "status:enum:draft,published!",
            "kind:enum:small,large",
            "user:references",
        ],
        &[],
    );
    scaffold(
        &["widget", "title:string!", "gadget:references"],
        &["--parent", "gadget"],
    );
    scaffold(&["doodad", "title:string!", "gadget:references?"], &[]);
    let widgets = read(&app, "src/controllers/widgets.rs");
    assert!(
        widgets.contains(".prefix(\"/gadgets/{parent}/widgets\")"),
        "{widgets}"
    );
    assert!(read(&app, "src/app.rs").contains("controllers::widgets::nested_routes()"));
    // `build`, not `check`: the generator's runs built everything already.
    cargo(&app, &["build"]);
    let tests = cargo(&app, &["test", "--lib"]);
    assert!(
        tests.contains("the_list_searches_and_sorts ... ok"),
        "{tests}"
    );
    cargo(&app, &["test", "--doc"]);
}

/// `new --loco` from the fixture (in place of `loco new`): install, auth, the starter's pages
/// and the demo account, built.
#[test]
#[ignore]
fn the_loco_starter_compiles() {
    let app = scratch("new_loco");
    lui(
        &["new"],
        &app,
        &["--loco", "--from", FIXTURE, "--dep-path", SOURCE],
    );
    let routes = read(&app, "src/app.rs");
    assert!(
        routes.contains(".add_route(controllers::home::routes())"),
        "{routes}"
    );
    assert!(
        routes.contains("async fn before_run") && routes.contains("SignIn"),
        "{routes}"
    );
    cargo(&app, &["build"]);
}

/// `new` (Axum): its own tests pass, and its check script's clippy is clean.
#[test]
#[ignore]
fn the_axum_starter_passes_its_tests() {
    let app = scratch("new_axum_built");
    lui(&["new"], &app, &["--dep-path", SOURCE]);
    cargo(&app, &["fmt", "--check"]);
    cargo(&app, &["clippy", "--all-targets", "--", "-D", "warnings"]);
    cargo(&app, &["test"]);
}

/// After `eject`, the app builds on its own copy of the library.
#[test]
#[ignore]
fn an_ejected_app_compiles() {
    let app = fresh_copy("eject");
    install(&app);
    lui(&["eject"], &app, &[]);
    cargo(&app, &["check"]);
}

/// `examples/loco-app` generates its scaffolds from its own copy of the templates; that copy
/// is the library's, byte for byte, so what the example shows is what `cargo lui install` writes.
#[test]
fn the_example_app_scaffolds_from_the_library_templates() {
    let copy = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/loco-app/.loco-templates/scaffold/api"
    );
    for (file, template) in [
        (
            "controller.t",
            include_str!("../loco-templates/scaffold/api/controller.t"),
        ),
        (
            "dto.t",
            include_str!("../loco-templates/scaffold/api/dto.t"),
        ),
    ] {
        let written = fs::read_to_string(Path::new(copy).join(file)).unwrap();
        assert!(
            written == template,
            "examples/loco-app/.loco-templates/scaffold/api/{file} differs from the library's"
        );
    }
}

/// `examples/loco-app`'s account pages are the templates as written, so its tests (every page
/// through Loco's router and Blitz) test what `cargo lui auth` writes.
#[test]
fn the_example_app_runs_the_generated_account_pages() {
    let example = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/loco-app/src");
    for (file, template) in [
        (
            "controllers/account.rs",
            include_str!("../loco-templates/auth/controller.rs"),
        ),
        (
            "views/account.rs",
            include_str!("../loco-templates/auth/views.rs"),
        ),
    ] {
        let written = fs::read_to_string(Path::new(example).join(file)).unwrap();
        assert!(
            written == template,
            "{file} differs from its template: rerun `cargo lui auth`"
        );
    }
}
