//! `cargo lui`: set up a Loco app for loco-ui, write its account pages, copy components into
//! it, or start a new app.
//!
//! ```sh
//! cargo install --git https://github.com/luis-serrano-l/loco-ui loco-ui --bin cargo-lui --features cli
//! cargo lui install                  # in the app's directory
//! cargo lui install path/to/app      # or name it
//! cargo lui auth                     # then the account pages
//! cargo lui add badge card           # copy components into src/ui/, yours to edit
//! cargo lui new my_app               # a new Axum app (`--loco` for a Loco one)
//! # from a loco-ui checkout, without installing:
//! cargo run -p loco-ui --features cli -- install path/to/app --dep-path "$PWD/loco-ui"
//! ```
//!
//! `install` writes, and prints one line per step:
//! - `.loco-templates/scaffold/api/{controller,dto}.t`, the scaffold that makes
//!   `cargo loco generate scaffold` write Maud views and HTML controllers;
//! - the initializer line in `src/app.rs` (`Box::new(loco_ui::loco::Initializer)`), and the
//!   `before_routes` / `after_routes` hooks: loco-ui's 404 page for every path no route
//!   answers, the app's look and the strict Content-Security-Policy on every page;
//! - `src/views/layout.rs`, a page with a header, and `src/views/look.rs`, the app's
//!   [`Look`](loco_ui::layout::Look), with their `pub mod` lines in `src/views/mod.rs`;
//! - `loco-ui` (git, pinned to this build's commit; or `--dep-path`) and `maud` under
//!   `[dependencies]` in `Cargo.toml`, edited with `toml_edit` so comments, CRLF line ends
//!   and dotted tables survive.
//!
//! `auth` writes the account pages (sign in, sign up, sign out, forgot and reset password,
//! email verification, magic link) as no-script forms on the starter's `users` model and
//! `AuthMailer` (`loco new` with a database):
//! - `src/controllers/account.rs` and `src/views/account.rs` (from `loco-templates/auth/`),
//!   their `pub mod account;` lines, and `.add_route(controllers::account::routes())` in
//!   `src/app.rs`;
//! - `Box::new(loco_ui::loco::SignIn("/signin"))` after loco-ui's initializer in `src/app.rs`,
//!   so a signed-out visit to a signed-in page goes to the sign-in form (not Loco's JSON 401)
//!   and back to that page after signing in;
//! - the links in the starter's mails (`src/mailers/auth/*/{html,text}.t`) pointed at those
//!   pages (`/verify/<token>`, `/reset/<token>`, `/magic-link/<token>`);
//! - `location: { from: Cookie, name: auth }` under `auth.jwt` in each `config/*.yaml`, so
//!   Loco's `auth::JWT` reads the token from the `HttpOnly` cookie the pages set.
//!
//! `add`, `list`, `info` and `eject` are in [`add`]; `new` in [`new`]; `scaffold` runs
//! `cargo loco generate scaffold` with the template options (`--parent`).
//!
//! Every command is idempotent: a second run changes nothing. A file that exists with other
//! content is left alone and reported (`--force` overwrites it), and `--dry-run` prints what
//! would change without writing.

mod add;
mod edit;
mod new;

use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

const CONTROLLER: &str = include_str!("../../../loco-templates/scaffold/api/controller.t");
const DTO: &str = include_str!("../../../loco-templates/scaffold/api/dto.t");
const ACCOUNT_CONTROLLER: &str = include_str!("../../../loco-templates/auth/controller.rs");
const ACCOUNT_VIEWS: &str = include_str!("../../../loco-templates/auth/views.rs");
const LAYOUT: &str = include_str!("../../../loco-templates/install/layout.rs");
const LOOK: &str = include_str!("../../../loco-templates/install/look.rs");
const GIT: &str = "https://github.com/luis-serrano-l/loco-ui";

const USAGE: &str = "cargo lui: loco-ui for Loco and Axum apps

usage: cargo lui install [APP_DIR] [--dep-path PATH] [--rev REV] [--force] [--dry-run]
       cargo lui auth [APP_DIR] [--force] [--dry-run]
       cargo lui add NAME.. [--app DIR] [--prefix WORD] [--force] [--dry-run] [--diff] [--view]
       cargo lui list
       cargo lui info NAME
       cargo lui eject [APP_DIR] [--dry-run]
       cargo lui new APP_DIR [--axum | --loco] [--from DIR] [--dep-path PATH] [--rev REV]
       cargo lui scaffold MODEL FIELD.. [--parent MODEL] [--app DIR] [-- GENERATOR_ARGS..]
       cargo lui --help | --version

install    the scaffold templates, the initializer, the 404 page, the look, the CSP, the deps
auth       the account pages (sign in, sign up, reset, magic link) on Loco's users model
add        copy components into src/ui/ (as `<prefix>_<name>`, `app_` by default)
list       the components `add` can copy
info       a component's setters
eject      copy loco-ui itself into vendor/ and depend on that copy
new        a new app: app shell, settings, theme toggle, 404 page, CSP, a check script
scaffold   `cargo loco generate scaffold` with loco-ui's options (`--parent notebook`)

--dry-run  print what would change and write nothing
--force    overwrite files that were edited since they were written";

fn main() -> ExitCode {
    // `cargo lui ..` runs this binary as `cargo-lui lui ..`.
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("lui") {
        args.remove(0);
    }
    let options = match Options::parse(args) {
        Ok(Parsed::Run(options)) => options,
        Ok(Parsed::Print(text)) => {
            println!("{text}");
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("{e}\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let result = match options.command.as_str() {
        "install" => install(&options),
        "auth" => auth(&options),
        "add" => add::add(&options),
        "list" => add::list(),
        "info" => add::info(&options),
        "eject" => add::eject(&options),
        "new" => new::new(&options),
        "scaffold" => scaffold(&options),
        other => Err(format!("unknown command {other}")),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// What the command line asked for.
pub(crate) struct Options {
    pub command: String,
    /// The app's directory (`.` unless named).
    pub app: PathBuf,
    /// The positional arguments after the command that are not the app: component names
    /// for `add` and `info`, the model and fields for `scaffold`.
    pub names: Vec<String>,
    /// Arguments after `--`, passed on to the generator by `scaffold`.
    pub rest: Vec<String>,
    pub dep_path: Option<String>,
    pub rev: Option<String>,
    pub prefix: String,
    pub parent: Option<String>,
    pub from: Option<PathBuf>,
    pub loco: bool,
    pub diff: bool,
    pub view: bool,
    pub files: Files,
}

enum Parsed {
    Run(Options),
    Print(String),
}

impl Options {
    fn parse(args: Vec<String>) -> Result<Parsed, String> {
        let mut args = args.into_iter();
        let Some(command) = args.next() else {
            return Ok(Parsed::Print(USAGE.to_string()));
        };
        match command.as_str() {
            "-h" | "--help" | "help" => return Ok(Parsed::Print(USAGE.to_string())),
            "-V" | "--version" => return Ok(Parsed::Print(version())),
            _ => {}
        }
        let mut options = Options {
            command,
            app: PathBuf::from("."),
            names: Vec::new(),
            rest: Vec::new(),
            dep_path: None,
            rev: None,
            prefix: "app".to_string(),
            parent: None,
            from: None,
            loco: false,
            diff: false,
            view: false,
            files: Files {
                force: false,
                dry_run: false,
            },
        };
        // `install`, `auth`, `eject` and `new` take the app as their one positional argument;
        // `add`, `info` and `scaffold` take names and the app as `--app`.
        let app_positional = matches!(
            options.command.as_str(),
            "install" | "auth" | "eject" | "new"
        );
        let mut app_named = false;
        while let Some(arg) = args.next() {
            let mut value = |flag: &str| args.next().ok_or_else(|| format!("{flag} needs a value"));
            match arg.as_str() {
                "-h" | "--help" => return Ok(Parsed::Print(USAGE.to_string())),
                "-V" | "--version" => return Ok(Parsed::Print(version())),
                "--force" => options.files.force = true,
                "--dry-run" => options.files.dry_run = true,
                "--diff" => options.diff = true,
                "--view" => options.view = true,
                "--loco" => options.loco = true,
                "--axum" => options.loco = false,
                "--dep-path" => options.dep_path = Some(value("--dep-path")?),
                "--rev" => options.rev = Some(value("--rev")?),
                "--prefix" => options.prefix = value("--prefix")?,
                "--parent" => options.parent = Some(value("--parent")?),
                "--from" => options.from = Some(PathBuf::from(value("--from")?)),
                "--app" => {
                    options.app = PathBuf::from(value("--app")?);
                    app_named = true;
                }
                "--" => {
                    options.rest = args.by_ref().collect();
                }
                flag if flag.starts_with('-') => return Err(format!("unknown option {flag}")),
                _ if app_positional && !app_named => {
                    options.app = PathBuf::from(arg);
                    app_named = true;
                }
                _ => options.names.push(arg),
            }
        }
        if !options.names.is_empty() && app_positional {
            return Err(format!(
                "{} takes one directory, not {}",
                options.command,
                options.names.join(" ")
            ));
        }
        if !is_word(&options.prefix) {
            return Err(format!(
                "--prefix {}: lowercase letters, digits and `_` only",
                options.prefix
            ));
        }
        Ok(Parsed::Run(options))
    }

    /// The `loco-ui` line for the app's `[dependencies]`, with `features`.
    pub fn dependency(&self, features: &str) -> String {
        match (&self.dep_path, self.rev.clone().or_else(source_rev)) {
            (Some(p), _) => {
                format!(r#"{{ path = "{p}", features = [{features}] }}"#)
            }
            (None, Some(rev)) => {
                format!(r#"{{ git = "{GIT}", rev = "{rev}", features = [{features}] }}"#)
            }
            (None, None) => format!(r#"{{ git = "{GIT}", features = [{features}] }}"#),
        }
    }
}

/// A lowercase Rust identifier: what `--prefix` and a component name may be.
pub(crate) fn is_word(text: &str) -> bool {
    text.starts_with(|c: char| c.is_ascii_lowercase())
        && text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn version() -> String {
    match source_rev() {
        Some(rev) => format!("cargo-lui {} ({rev})", env!("CARGO_PKG_VERSION")),
        None => format!("cargo-lui {}", env!("CARGO_PKG_VERSION")),
    }
}

/// The commit this binary was built from, which `install` pins the git dependency to until
/// the crate is published: the source's `git rev-parse HEAD`, or the short commit that
/// `cargo install --git` names its checkout directory after
/// (`~/.cargo/git/checkouts/loco-ui-<hash>/<commit>/loco-ui`).
fn source_rev() -> Option<String> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    let head = Command::new("git")
        .arg("-C")
        .arg(source)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|rev| rev.trim().to_string())
        .filter(|rev| !rev.is_empty());
    head.or_else(|| {
        let checkout = source.parent()?;
        let in_checkouts = checkout.parent()?.parent()?.ends_with("git/checkouts");
        let name = checkout.file_name()?.to_str()?;
        (in_checkouts && name.chars().all(|c| c.is_ascii_hexdigit())).then(|| name.to_string())
    })
}

pub(crate) fn loco_app(app: &Path) -> Result<(), String> {
    if !app.join("src/app.rs").is_file() || !app.join("Cargo.toml").is_file() {
        return Err(format!(
            "{} is not a Loco app (no Cargo.toml and src/app.rs)",
            app.display()
        ));
    }
    Ok(())
}

fn install(options: &Options) -> Result<(), String> {
    let app = &options.app;
    let files = &options.files;
    loco_app(app)?;
    let templates = app.join(".loco-templates/scaffold/api");
    files.write(&templates.join("controller.t"), CONTROLLER)?;
    files.write(&templates.join("dto.t"), DTO)?;
    files.write(&app.join("src/views/layout.rs"), LAYOUT)?;
    files.write(&app.join("src/views/look.rs"), LOOK)?;
    files.edit(&app.join("src/views/mod.rs"), |s| {
        edit::add_line(&edit::add_line(s, "pub mod layout;")?, "pub mod look;")
    })?;
    files.edit(&app.join("src/app.rs"), |s| {
        edit::add_hooks(&edit::add_initializer(s, edit::INITIALIZER)?)
    })?;
    let dep = options.dependency(r#""loco""#);
    files.edit(&app.join("Cargo.toml"), |s| {
        let s = edit::add_dependency(s, "loco-ui", &dep)?;
        edit::add_dependency(&s, "maud", r#""0.27""#)
    })?;
    println!("done: `cargo loco generate scaffold <model> <fields..>` now writes Maud views");
    Ok(())
}

fn auth(options: &Options) -> Result<(), String> {
    let app = &options.app;
    let files = &options.files;
    loco_app(app)?;
    let manifest = fs::read_to_string(app.join("Cargo.toml")).unwrap_or_default();
    if !manifest.contains("loco-ui") {
        return Err("no loco-ui dependency: run `cargo lui install` first".into());
    }
    let users = fs::read_to_string(app.join("src/models/users.rs")).unwrap_or_default();
    if !users.contains("RegisterParams") || !app.join("src/mailers/auth.rs").is_file() {
        return Err(
            "needs the starter's users model and AuthMailer (`loco new` with a database)".into(),
        );
    }
    files.write(&app.join("src/controllers/account.rs"), ACCOUNT_CONTROLLER)?;
    files.write(&app.join("src/views/account.rs"), ACCOUNT_VIEWS)?;
    files.edit(&app.join("src/controllers/mod.rs"), |s| {
        edit::add_line(s, "pub mod account;")
    })?;
    files.edit(&app.join("src/views/mod.rs"), |s| {
        edit::add_line(s, "pub mod account;")
    })?;
    files.edit(&app.join("src/app.rs"), |s| {
        edit::add_initializer(&edit::add_account_route(s)?, edit::SIGN_IN)
    })?;
    for mail in ["welcome", "forgot", "magic_link"] {
        for part in ["html.t", "text.t"] {
            let path = app.join("src/mailers/auth").join(mail).join(part);
            if path.is_file() {
                files.edit(&path, |s| Ok(edit::point_mail_links(s)))?;
            }
        }
    }
    let config = app.join("config");
    let mut yaml: Vec<PathBuf> = fs::read_dir(&config)
        .map_err(|e| format!("{}: {e}", config.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "yaml"))
        .collect();
    yaml.sort();
    for path in yaml {
        files.edit(&path, edit::add_jwt_location)?;
    }
    println!("done: /signin, /signup, /forgot and /magic-link answer with pages");
    Ok(())
}

/// `cargo loco generate scaffold MODEL FIELD..` in the app, with the template options in the
/// environment the templates read them from (`get_env(name="LUI_PARENT")`).
fn scaffold(options: &Options) -> Result<(), String> {
    loco_app(&options.app)?;
    if options.names.is_empty() {
        return Err("scaffold needs a model and its fields".into());
    }
    let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
    command
        .args(["loco", "generate", "scaffold"])
        .args(&options.names)
        .args(&options.rest)
        .current_dir(&options.app);
    if let Some(parent) = &options.parent {
        if !is_word(parent) {
            return Err(format!("--parent {parent}: a model name in snake case"));
        }
        let field = format!("{parent}:references");
        if !options.names.iter().any(|f| f.starts_with(&field)) {
            return Err(format!(
                "--parent {parent} needs a `{parent}:references` field (the column it nests by)"
            ));
        }
        command.env("LUI_PARENT", parent);
    }
    if options.files.dry_run {
        println!("would run {command:?}");
        return Ok(());
    }
    let status = command
        .status()
        .map_err(|e| format!("cargo loco generate: {e}"))?;
    if !status.success() {
        return Err(format!("cargo loco generate scaffold failed ({status})"));
    }
    Ok(())
}

/// Writes and edits files, or only says what it would do (`--dry-run`).
pub(crate) struct Files {
    pub force: bool,
    pub dry_run: bool,
}

impl Files {
    /// Write `content` unless the file already holds it; other content is kept unless
    /// `--force`.
    pub fn write(&self, path: &Path, content: &str) -> Result<(), String> {
        match fs::read_to_string(path) {
            Ok(old) if old == content => println!("unchanged {}", path.display()),
            Ok(_) if !self.force => {
                println!("kept      {} (edited; --force overwrites)", path.display());
            }
            Err(e) if e.kind() != io::ErrorKind::NotFound => {
                return Err(format!("{}: {e}", path.display()));
            }
            _ if self.dry_run => println!("would write {}", path.display()),
            _ => {
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
                }
                fs::write(path, content).map_err(|e| format!("{}: {e}", path.display()))?;
                println!("wrote     {}", path.display());
            }
        }
        Ok(())
    }

    /// Apply `change` to the file (created empty when missing) and write it back only if it
    /// changed. A file with CRLF line ends is edited as LF and written back with CRLF.
    pub fn edit(
        &self,
        path: &Path,
        change: impl Fn(&str) -> Result<String, String>,
    ) -> Result<(), String> {
        let old = match fs::read_to_string(path) {
            Ok(old) => old,
            Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(format!("{}: {e}", path.display())),
        };
        let crlf = old.contains("\r\n");
        let lf = if crlf {
            old.replace("\r\n", "\n")
        } else {
            old.clone()
        };
        let new = change(&lf).map_err(|e| format!("{}: {e}", path.display()))?;
        let new = if crlf { new.replace('\n', "\r\n") } else { new };
        if new == old {
            println!("unchanged {}", path.display());
        } else if self.dry_run {
            println!("would edit {}", path.display());
        } else {
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            }
            fs::write(path, new).map_err(|e| format!("{}: {e}", path.display()))?;
            println!("edited    {}", path.display());
        }
        Ok(())
    }
}
