//! `cargo lui new APP_DIR [--axum | --loco]`: a new app with the app shell, a settings page
//! with the theme toggle, the 404 page, the strict CSP, the app's `Look` and a
//! `scripts/check.sh` (format, clippy, tests, no `<script>`), from `loco-templates/new/`.
//!
//! `--axum` (the default) writes a whole Axum app. `--loco` starts from a Loco app: `loco new`
//! (SQLite, blocking workers, no assets), or a copy of `--from DIR`; then runs `install` and
//! `auth`, and adds the pages and a demo account seeded on the first start in development.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{Files, Options, edit};

const AXUM: [(&str, &str); 7] = [
    (
        "Cargo.toml",
        include_str!("../../../loco-templates/new/axum/Cargo.toml"),
    ),
    (
        "src/main.rs",
        include_str!("../../../loco-templates/new/axum/src/main.rs"),
    ),
    (
        "src/lib.rs",
        include_str!("../../../loco-templates/new/axum/src/lib.rs"),
    ),
    (
        "src/pages.rs",
        include_str!("../../../loco-templates/new/axum/src/pages.rs"),
    ),
    (
        "src/look.rs",
        include_str!("../../../loco-templates/new/axum/src/look.rs"),
    ),
    (
        "tests/pages.rs",
        include_str!("../../../loco-templates/new/axum/tests/pages.rs"),
    ),
    (
        "scripts/check.sh",
        include_str!("../../../loco-templates/new/scripts/check.sh"),
    ),
];

const LOCO: [(&str, &str); 4] = [
    (
        "src/controllers/home.rs",
        include_str!("../../../loco-templates/new/loco/controllers/home.rs"),
    ),
    (
        "src/views/home.rs",
        include_str!("../../../loco-templates/new/loco/views/home.rs"),
    ),
    (
        "src/views/look.rs",
        include_str!("../../../loco-templates/new/loco/views/look.rs"),
    ),
    (
        "scripts/check.sh",
        include_str!("../../../loco-templates/new/scripts/check.sh"),
    ),
];

const HOME_ROUTE: &str = ".add_route(controllers::home::routes())";

const SEED: &str = "    /// loco-ui starter: the first start in development makes the demo account
    /// (demo@example.com / demo-password), so the sign-in form has someone to sign in.
    async fn before_run(ctx: &AppContext) -> Result<()> {
        use loco_rs::prelude::{EntityTrait, IntoActiveModel, PaginatorTrait};
        if ctx.environment == Environment::Development
            && users::Entity::find().count(&ctx.db).await? == 0
        {
            let demo = crate::models::users::RegisterParams {
                email: \"demo@example.com\".into(),
                password: \"demo-password\".into(),
                name: \"Demo\".into(),
            };
            let user = crate::models::users::Model::create_with_password(&ctx.db, &demo).await?;
            user.into_active_model().verified(&ctx.db).await?;
        }
        Ok(())
    }

";

pub fn new(options: &Options) -> Result<(), String> {
    let app = &options.app;
    if app == Path::new(".") {
        return Err("new needs the directory to create: `cargo lui new my_app`".into());
    }
    let name = app
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("the app's directory needs a name")?
        .replace('-', "_");
    if !crate::is_word(&name) {
        return Err(format!(
            "{name}: an app name is lowercase letters, digits and `_`"
        ));
    }
    if app.exists()
        && fs::read_dir(app)
            .map(|mut d| d.next().is_some())
            .unwrap_or(true)
    {
        return Err(format!("{} exists and is not empty", app.display()));
    }
    let files = Files {
        force: true,
        dry_run: options.files.dry_run,
    };
    let fill = |text: &str| {
        text.replace("{{name}}", &name)
            .replace("{{crate}}", &name)
            .replace("{{dep}}", &options.dependency(r#""axum""#))
    };
    if !options.loco {
        for (path, text) in AXUM {
            files.write(&app.join(path), &fill(text))?;
        }
        executable(&app.join("scripts/check.sh"), files.dry_run)?;
        println!(
            "done: cd {} && cargo run, then open http://127.0.0.1:3000",
            app.display()
        );
        return Ok(());
    }
    start_loco(app, &name, options)?;
    if files.dry_run {
        println!("would run install and auth on {}", app.display());
    } else {
        crate::install(options)?;
        crate::auth(options)?;
    }
    for (path, text) in LOCO {
        files.write(&app.join(path), &fill(text))?;
    }
    executable(&app.join("scripts/check.sh"), files.dry_run)?;
    files.edit(&app.join("src/controllers/mod.rs"), |s| {
        edit::add_line(s, "pub mod home;")
    })?;
    files.edit(&app.join("src/views/mod.rs"), |s| {
        edit::add_line(s, "pub mod home;")
    })?;
    files.edit(&app.join("src/app.rs"), |s| add_seed(&add_home_route(s)?))?;
    println!(
        "done: cd {} && cargo loco start, then open http://localhost:5150",
        app.display()
    );
    Ok(())
}

/// `chmod +x` the check script, so `scripts/check.sh` runs as written.
fn executable(path: &Path, dry_run: bool) -> Result<(), String> {
    #[cfg(unix)]
    if !dry_run {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("{}: {e}", path.display()))?;
    }
    #[cfg(not(unix))]
    let _ = (path, dry_run);
    Ok(())
}

/// The Loco app to start from: a copy of `--from`, or `loco new`.
fn start_loco(app: &Path, name: &str, options: &Options) -> Result<(), String> {
    if let Some(from) = &options.from {
        if options.files.dry_run {
            println!("would copy {} to {}", from.display(), app.display());
            return Ok(());
        }
        return copy_dir(from, app);
    }
    let parent = app
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    let mut command = Command::new("loco");
    command
        .args([
            "new", "-n", name, "--db", "sqlite", "--bg", "blocking", "--assets", "none",
        ])
        .current_dir(&parent);
    if options.files.dry_run {
        println!("would run {command:?}");
        return Ok(());
    }
    let status = command.status().map_err(|e| {
        format!("`loco new`: {e}. Install it (`cargo install loco`), or start from an app with --from DIR")
    })?;
    if !status.success() {
        return Err(format!("`loco new` failed ({status})"));
    }
    let made = parent.join(name);
    if made != *app {
        fs::rename(&made, app).map_err(|e| format!("{}: {e}", made.display()))?;
    }
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    let entries = fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", from.display()))?;
        let target = to.join(entry.file_name());
        if entry.file_name() == "target" {
            continue;
        }
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target).map_err(|e| format!("{}: {e}", target.display()))?;
        }
    }
    Ok(())
}

/// The home routes right after `AppRoutes::..` in `fn routes`.
fn add_home_route(contents: &str) -> Result<String, String> {
    if contents.contains(HOME_ROUTE) {
        return Ok(contents.to_string());
    }
    let missing = || format!("no `AppRoutes::` in `fn routes`; add `{HOME_ROUTE}` by hand");
    let at = contents.find("fn routes").ok_or_else(missing)?;
    let at = at + contents[at..].find("AppRoutes::").ok_or_else(missing)?;
    let end = at + contents[at..].find('\n').ok_or_else(missing)? + 1;
    Ok(format!(
        "{}            {HOME_ROUTE}\n{}",
        &contents[..end],
        &contents[end..]
    ))
}

/// The `before_run` hook that seeds the demo account, before `fn routes`.
fn add_seed(contents: &str) -> Result<String, String> {
    if contents.contains("fn before_run") {
        return Ok(contents.to_string());
    }
    let at = contents
        .find("fn routes(")
        .ok_or("no `fn routes` in `impl Hooks`; add `before_run` by hand")?;
    let line = contents[..at].rfind('\n').map_or(0, |i| i + 1);
    Ok(format!("{}{SEED}{}", &contents[..line], &contents[line..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_starter_routes_and_seed_go_in_once() {
        let app = "impl Hooks for App {\n    fn routes(_ctx: &AppContext) -> AppRoutes {\n        AppRoutes::with_default_routes()\n    }\n}\n";
        let added = add_seed(&add_home_route(app).unwrap()).unwrap();
        assert!(
            added.contains(
                "with_default_routes()\n            .add_route(controllers::home::routes())\n"
            ),
            "{added}"
        );
        assert!(
            added.contains("    async fn before_run(ctx: &AppContext)"),
            "{added}"
        );
        assert_eq!(add_seed(&add_home_route(&added).unwrap()).unwrap(), added);
    }
}
