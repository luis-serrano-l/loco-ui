//! `cargo lui install` and `cargo lui auth` on a fresh Loco app. `tests/fresh-loco-app` is
//! `loco new -n fresh_app --db sqlite --bg blocking --assets none` (loco 1.2.0) trimmed to
//! what `cargo check`, the installer and `generate scaffold` read: `Cargo.toml`, `Cargo.lock`,
//! `.cargo/config.toml` (the `loco-tool` alias the generator runs), `config/`, `src/`,
//! `migration/` and `tests/models/mod.rs` (where the generator adds each model's test).

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fresh-loco-app");

/// A copy of the fixture in `target/`, so every run starts from `loco new`'s files.
fn fresh_copy(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/lui-install")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    copy_dir(Path::new(FIXTURE), &dir);
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
    lui(
        &["install"],
        app,
        &["--dep-path", env!("CARGO_MANIFEST_DIR")],
    )
}

/// `cargo lui <command> <app> <rest..>`; its stdout, after checking it succeeded.
fn lui(command: &[&str], app: &Path, rest: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_cargo-lui"))
        .arg("lui")
        .args(command)
        .arg(app)
        .args(rest)
        .output()
        .unwrap();
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

#[test]
fn install_writes_every_piece_once() {
    let app = fresh_copy("once");
    let first = install(&app);
    assert_eq!(first.matches("wrote").count(), 3, "{first}");
    assert_eq!(first.matches("edited").count(), 3, "{first}");
    assert_eq!(
        read(&app, ".loco-templates/scaffold/api/controller.t"),
        include_str!("../loco-templates/scaffold/api/controller.t")
    );
    assert!(read(&app, "src/app.rs").contains("Ok(vec![Box::new(loco_ui::loco::Initializer)])"));
    assert!(read(&app, "src/views/mod.rs").contains("pub mod auth;\npub mod layout;\n"));
    assert!(read(&app, "src/views/layout.rs").contains("pub fn page(ui: &Ui"));
    let toml = read(&app, "Cargo.toml");
    assert!(toml.contains("\nloco-ui = { path = "), "{toml}");
    assert!(toml.contains("maud = \"0.27\""), "{toml}");

    let second = install(&app);
    assert!(
        !second.contains("wrote") && !second.contains("edited"),
        "{second}"
    );
    assert_eq!(second.matches("unchanged").count(), 6, "{second}");
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
    let out = Command::new(env!("CARGO_BIN_EXE_cargo-lui"))
        .arg("auth")
        .arg(&app)
        .output()
        .unwrap();
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
    let out = Command::new(env!("CARGO_BIN_EXE_cargo-lui"))
        .args(["install", env!("CARGO_MANIFEST_DIR")])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("not a Loco app"));
}

/// With the account pages and three scaffolds from the installed templates: one field of every
/// kind they tell apart (text, dates and times, numbers, a bool, enums required and optional,
/// the owner's `user_id`, which the form leaves out), then a required and an optional
/// reference to it. Builds Loco and SeaORM for the fixture (a few minutes the first time), so
/// it is run by CI with `--ignored` rather than by every `cargo test`.
/// The generator runs `sea-orm-cli` for the entities, so that must be installed.
#[test]
#[ignore]
fn the_installed_app_compiles() {
    let app = fresh_copy("check");
    install(&app);
    lui(&["auth"], &app, &[]);
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/lui-install-target");
    let cargo = |args: &[&str]| {
        let out = Command::new(env!("CARGO"))
            .args(args)
            .current_dir(&app)
            .env("CARGO_TARGET_DIR", &target)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "cargo {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
    };
    let scaffold = |fields: &[&str]| {
        let generate = ["run", "--quiet", "--", "generate", "scaffold"];
        cargo(&[&generate[..], fields].concat());
    };
    scaffold(&[
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
    ]);
    scaffold(&["widget", "title:string!", "gadget:references"]);
    scaffold(&["doodad", "title:string!", "gadget:references?"]);
    // `build`, not `check`: the generator's runs built everything already.
    cargo(&["build"]);
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
