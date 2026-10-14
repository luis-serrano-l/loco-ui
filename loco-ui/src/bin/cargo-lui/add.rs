//! `cargo lui add`, `list`, `info` and `eject`: copy loco-ui's components into an app.
//!
//! `add badge` copies `loco-ui/src/badge.rs` from the app's own loco-ui dependency (found with
//! `cargo metadata`, or the `path` the dependency names) into `src/ui/app_badge.rs`, the
//! whole file: its `//!` header, doctests, `PROPS` and `CSS`. What changes, so it compiles
//! inside the app:
//! - `crate::` paths become `loco_ui::`, and `#[cfg(feature = "axum" | "http" | "loco")]`
//!   lines go (the installed dependency has all three on);
//! - the builder takes the prefix (`Badge` → `AppBadge`, `ui.badge(..)` → `ui.app_badge(..)`,
//!   `lui-badge` classes → `app-badge`). Rust picks a type's own (inherent) method over a
//!   trait's, so a copy that kept the name would never be called: loco-ui's `ui.badge(..)`
//!   would win;
//! - `impl Ui { .. }` becomes the extension trait `AppBadgeExt` and its `impl AppBadgeExt
//!   for Ui`, the pattern `docs/components.md` describes for a component of the app's own;
//! - each doctest imports `<app>::ui::*`, and the unit tests (which test loco-ui's
//!   internals) are left out.
//!
//! `src/ui/mod.rs` gets `pub mod app_badge;` and `pub use app_badge::{AppBadge, AppBadgeExt};`,
//! so `use crate::ui::*;` brings every copied component; `src/lib.rs` gets `pub mod ui;`; and
//! the `css` list of the app's `Look` (`src/views/look.rs`) gets `crate::ui::app_badge::CSS`.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use loco_ui::props::Component;
use toml_edit::{DocumentMut, InlineTable, Item, Value};

use crate::{Options, edit::add_line, is_word};

/// The components a copy of builds on loco-ui's public API alone, so it compiles in an app.
/// The others call helpers loco-ui keeps to itself (the shared search box, `labelled`, the
/// menu placement, `gap_class`, a button's link and part setters behind the dialog, drawer
/// and tooltip triggers, the field internals the input shares with the select and range,
/// ..); copying one of those copies its use of them too, which does not compile outside the
/// crate. `flash` is left out too: its cookie is named `lui-flash` like its class, and the
/// copy's renamed class would rename the cookie the redirects set. `tests/install.rs` copies
/// every one of these into a fresh Loco app and builds it.
pub const COPYABLE: [&str; 22] = [
    "accordion",
    "avatar",
    "badge",
    "breadcrumbs",
    "card",
    "chart",
    "error_summary",
    "marquee",
    "meter",
    "progress",
    "separator",
    "skeleton",
    "stat",
    "tabs",
    "toast",
    "description_list",
    "input_otp",
    "blocks/auth_page",
    "blocks/dashboard_page",
    "blocks/error_page",
    "blocks/record_page",
    "blocks/settings_page",
];

/// The copy's names, from the module and the prefix.
struct Names<'a> {
    /// The module as `loco_ui::props()` names it: `badge`, `blocks/record_page`.
    module: &'a str,
    /// Its file name: `badge`, `record_page`.
    short: &'a str,
    prefix: &'a str,
    /// `app_badge`: the file, the module and the methods' prefix.
    file: String,
    /// `App`: the types' prefix.
    upper: String,
}

impl<'a> Names<'a> {
    fn new(module: &'a str, prefix: &'a str) -> Self {
        let short = module.rsplit('/').next().unwrap_or(module);
        Names {
            module,
            short,
            prefix,
            file: format!("{prefix}_{short}"),
            upper: upper_camel(prefix),
        }
    }
}

/// The module `name` stands for: a component's (`badge`) or a block's (`record_page` is
/// `blocks/record_page`).
fn resolve(name: &str) -> Result<&'static str, String> {
    let module = loco_ui::props()
        .iter()
        .map(|c| c.module)
        .find(|m| *m == name || m.strip_prefix("blocks/") == Some(name))
        .ok_or_else(|| unknown(name))?;
    copyable(module)?;
    Ok(module)
}

/// The builder a module is named after: the first one made from `ui` (`ui.menu(..)` in
/// `popover`, not `Row::new(..)` in `table`).
fn main_builder(module: &str) -> Option<&'static Component> {
    let mut all = components(module).peekable();
    let first = all.peek().copied();
    all.find(|c| c.calls.first().is_some_and(|call| call.starts_with("ui.")))
        .or(first)
}

pub fn add(options: &Options) -> Result<(), String> {
    let app = &options.app;
    if options.names.is_empty() {
        return Err("add needs the components to copy: `cargo lui add badge card`".into());
    }
    let source = source_dir(app)?;
    let crate_name = app_crate(app)?;
    let files = &options.files;
    for name in &options.names {
        let module = resolve(name)?;
        let names = Names::new(module, &options.prefix);
        let original = component_source(&source, module)?;
        let copy = vendor(&original, &names, &crate_name)?;
        let path = app.join("src/ui").join(format!("{}.rs", names.file));
        if options.view {
            print!("{copy}");
            continue;
        }
        if options.diff {
            let mine = fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e} (nothing to diff)", path.display()))?;
            let changes = diff(&copy, &mine);
            if changes.is_empty() {
                println!("same      {} (loco-ui's copy, unedited)", path.display());
            } else {
                println!(
                    "--- loco-ui's {module}.rs, as `add` writes it\n+++ {}\n{changes}",
                    path.display()
                );
            }
            continue;
        }
        files.write(&path, &copy)?;
        let exports = exports(&names);
        files.edit(&app.join("src/ui/mod.rs"), |s| {
            let s = if s.is_empty() {
                MOD_HEADER.to_string()
            } else {
                s.to_string()
            };
            let s = add_line(&s, &format!("pub mod {};", names.file))?;
            add_line(&s, &format!("pub use {}::{{{exports}}};", names.file))
        })?;
        files.edit(&app.join("src/lib.rs"), |s| add_line(s, "pub mod ui;"))?;
        if copy.contains("pub const CSS") {
            let css = format!("crate::ui::{}::CSS", names.file);
            match look_file(app) {
                Some(look) => files.edit(&look, |s| add_css(s, &css))?,
                None => println!(
                    "note      no src/views/look.rs: add `.css({css})` to the pages that show it"
                ),
            }
        }
    }
    if !options.view && !options.diff {
        let first = Names::new(resolve(&options.names[0])?, &options.prefix);
        if let Some(method) = main_builder(first.module).and_then(method_of) {
            let new = format!("{}_{method}", first.prefix);
            println!(
                "done: `use crate::ui::*;`, then `ui.{new}(..)`, or `{}(..)` in `lui!`",
                upper_camel(&new)
            );
        }
    }
    Ok(())
}

const MOD_HEADER: &str =
    "//! Components copied from loco-ui by `cargo lui add`, yours to edit. `use crate::ui::*;`
//! brings each one's builder and the extension trait that puts it on `Ui`; its CSS is in
//! the `css` list of `src/views/look.rs`. `cargo lui add <name> --diff` shows what changed
//! in loco-ui's own copy since.

";

/// `AppBadge, AppBadgeExt`: what `src/ui/mod.rs` re-exports of the copy.
fn exports(names: &Names) -> String {
    let mut out: Vec<String> = components(names.module)
        .map(|c| format!("{}{}", names.upper, c.builder))
        .collect();
    out.dedup();
    if let Some(main) = main_builder(names.module).filter(|c| method_of(c).is_some()) {
        out.push(format!("{}{}Ext", names.upper, main.builder));
    }
    out.join(", ")
}

/// `badge` for a builder made by `ui.badge(..)`.
fn method_of(c: &Component) -> Option<&'static str> {
    let call = c.calls.first()?.strip_prefix("ui.")?;
    call.split('(').next()
}

pub fn list() -> Result<(), String> {
    let mut seen = BTreeSet::new();
    println!("{:<18} {:<16} call", "component", "builder");
    for c in loco_ui::props() {
        if copyable(c.module).is_err() || !seen.insert(c.module) {
            continue;
        }
        let call = c.calls.first().copied().unwrap_or("");
        println!("{:<18} {:<16} {call}", c.module, c.builder);
    }
    println!(
        "\n`cargo lui info <component>` lists its setters; `cargo lui add <component>` copies it."
    );
    Ok(())
}

pub fn info(options: &Options) -> Result<(), String> {
    let [name] = options.names.as_slice() else {
        return Err("info takes one component: `cargo lui info badge`".into());
    };
    let module = loco_ui::props()
        .iter()
        .map(|c| c.module)
        .find(|m| m == name || m.strip_prefix("blocks/") == Some(name.as_str()))
        .ok_or_else(|| unknown(name))?;
    let found: Vec<&Component> = components(module).collect();
    if found.is_empty() {
        return Err(unknown(module));
    }
    for c in found {
        println!(
            "{} ({}, `{}` in lui!)",
            c.builder,
            c.status.as_str(),
            c.lui()
        );
        for call in c.calls {
            println!("  {call}");
        }
        println!();
        println!(
            "  {:<16} {:<10} {:<34} {:<10} {:<12} doc",
            "setter", "kind", "arguments", "default", "attribute"
        );
        for p in c.props {
            println!(
                "  {:<16} {:<10} {:<34} {:<10} {:<12} {}",
                p.name,
                p.kind.as_str(),
                p.args,
                p.default,
                p.attr,
                p.doc
            );
        }
        println!();
    }
    Ok(())
}

fn components(module: &str) -> impl Iterator<Item = &'static Component> + '_ {
    loco_ui::props().iter().filter(move |c| c.module == module)
}

fn unknown(module: &str) -> String {
    format!("no component `{module}` in loco-ui: `cargo lui list` names them")
}

fn copyable(module: &str) -> Result<(), String> {
    if COPYABLE.contains(&module) {
        Ok(())
    } else {
        Err(format!(
            "`{module}` cannot be copied yet: it uses loco-ui helpers that are private to the \
             crate (`cargo lui list` names the ones that can); `cargo lui eject` copies all of loco-ui"
        ))
    }
}

/// The component's file in the dependency's source.
fn component_source(source: &Path, module: &str) -> Result<String, String> {
    let short = module.strip_prefix("blocks/").unwrap_or(module);
    if !is_word(short) {
        return Err(unknown(module));
    }
    let path = source.join("src").join(format!("{module}.rs"));
    fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))
}

/// The app's package name as a crate name, for the doctests' `use <app>::ui::*;`.
fn app_crate(app: &Path) -> Result<String, String> {
    let doc = manifest(app)?;
    let lib = doc.get("lib").and_then(|l| l.get("name"));
    let name = lib
        .or_else(|| doc.get("package").and_then(|p| p.get("name")))
        .and_then(Item::as_str)
        .ok_or("Cargo.toml has no [package] name")?;
    Ok(name.replace('-', "_"))
}

fn manifest(app: &Path) -> Result<DocumentMut, String> {
    let path = app.join("Cargo.toml");
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.parse().map_err(|e| format!("{}: {e}", path.display()))
}

/// Where the app's loco-ui dependency lives: the `path` it names, else what `cargo
/// metadata` resolved (a git checkout, or the registry copy).
pub fn source_dir(app: &Path) -> Result<PathBuf, String> {
    let doc = manifest(app)?;
    let dep = doc
        .get("dependencies")
        .and_then(|d| d.get("loco-ui"))
        .ok_or("no loco-ui dependency: run `cargo lui install` first")?;
    if let Some(path) = dep.get("path").and_then(Item::as_str) {
        return Ok(app.join(path));
    }
    package_dirs(app)?
        .into_iter()
        .find(|(name, _)| name == "loco-ui")
        .map(|(_, dir)| dir)
        .ok_or_else(|| "cargo metadata lists no loco-ui package".to_string())
}

/// Every package `cargo metadata` resolves for the app, as `(name, directory)`.
fn package_dirs(app: &Path) -> Result<Vec<(String, PathBuf)>, String> {
    let out = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg(app.join("Cargo.toml"))
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "cargo metadata: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("cargo metadata: {e}"))?;
    let packages = json["packages"].as_array().cloned().unwrap_or_default();
    Ok(packages
        .iter()
        .filter_map(|p| {
            let name = p["name"].as_str()?.to_string();
            let dir = Path::new(p["manifest_path"].as_str()?)
                .parent()?
                .to_path_buf();
            Some((name, dir))
        })
        .collect())
}

fn look_file(app: &Path) -> Option<PathBuf> {
    ["src/views/look.rs", "src/look.rs"]
        .iter()
        .map(|p| app.join(p))
        .find(|p| p.is_file())
}

/// Add `css` to the `css: &[..]` list of the app's `Look`, once.
fn add_css(contents: &str, css: &str) -> Result<String, String> {
    if contents.contains(css) {
        return Ok(contents.to_string());
    }
    let missing = || format!("no `css: &[` in the Look; add `{css}` to it by hand");
    let open = contents.find("css: &[").ok_or_else(missing)? + "css: &[".len();
    let close = open + contents[open..].find(']').ok_or_else(missing)?;
    let inside = contents[open..close].trim_end();
    let joined = if inside.trim().is_empty() {
        css.to_string()
    } else if inside.ends_with(',') {
        format!("{inside} {css}")
    } else {
        format!("{inside}, {css}")
    };
    Ok(format!(
        "{}{joined}{}",
        &contents[..open],
        &contents[close..]
    ))
}

/// `app_badge` → `AppBadge`.
fn upper_camel(snake: &str) -> String {
    snake
        .split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| {
                f.to_ascii_uppercase().to_string() + c.as_str()
            })
        })
        .collect()
}

/// The component file `source` rewritten to live in the app as `src/ui/<names.file>.rs`.
fn vendor(source: &str, names: &Names, crate_name: &str) -> Result<String, String> {
    let text = drop_unit_tests(source);
    let text: String = text
        .split_inclusive('\n')
        .filter(|l| {
            let l = l.trim();
            let feature = matches!(
                l,
                r#"#[cfg(feature = "axum")]"#
                    | r#"#[cfg(feature = "http")]"#
                    | r#"#[cfg(feature = "loco")]"#
            );
            let docsrs = l.starts_with("#[cfg_attr(docsrs") || l.starts_with("#![cfg_attr(docsrs");
            !feature && !docsrs
        })
        .collect();
    let (text, methods) = take_impl_ui(&text)?;
    let builders: Vec<&Component> = components(names.module).collect();
    let main = main_builder(names.module)
        .ok_or_else(|| unknown(names.module))?
        .builder;
    let ext = format!("{}{main}Ext", names.upper);

    let rename = |text: &str| -> String {
        let mut text = text.replace("crate::", "loco_ui::");
        for c in &builders {
            text = rename_word(&text, c.builder, &format!("{}{}", names.upper, c.builder));
            let lui = c.lui();
            if lui != c.builder {
                text = rename_word(&text, &lui, &format!("{}{lui}", names.upper));
            }
        }
        for m in &methods {
            let new = format!("{}_{}", names.prefix, m.name);
            text = text.replace(&format!("Ui::{}`", m.name), &format!("{ext}::{new}`"));
            text = rename_call(&text, &m.name, &new);
        }
        if let Some(class) = class_root(source, names.short) {
            text = rename_class(&text, &class, names.prefix);
        }
        text
    };
    let mut text = rename(&text);
    if !methods.is_empty() {
        // A module inside the file that imports the builder calls its constructor too.
        let builder = format!("use super::{}{main};", names.upper);
        let both = format!("use super::{{{}{main}, {ext}}};", names.upper);
        text = text.replace(&builder, &both);
    }
    if !methods.is_empty() {
        let mut declarations = String::new();
        let mut definitions = String::new();
        for m in &methods {
            let new = format!("{}_{}", names.prefix, m.name);
            let signature =
                rename(&m.signature).replacen(&format!("fn {}", m.name), &format!("fn {new}"), 1);
            let body = methods.iter().fold(rename(&m.body), |body, other| {
                let call = format!("self.{}(", other.name);
                body.replace(&call, &format!("self.{}_{}(", names.prefix, other.name))
            });
            declarations.push_str(&declaration(&signature));
            if !definitions.is_empty() {
                definitions.push('\n');
            }
            definitions.push_str(&definition(&signature, &body));
        }
        let lead = format!(
            "/// `ui.{prefix}_{first}(..)`: loco-ui's `ui.{first}(..)`, copied into the app.\n",
            prefix = names.prefix,
            first = methods[0].name
        );
        let block = format!(
            "{lead}pub trait {ext} {{\n{declarations}}}\n\nimpl {ext} for Ui {{\n{definitions}}}\n"
        );
        text = text.replacen(IMPL_MARK, &block, 1);
    }
    let text = doctests_import(&text, crate_name);
    Ok(allow_unused_helpers(&add_note(&text, names)))
}

/// `#![allow(dead_code)]` after the header: the copy keeps helpers loco-ui's other
/// components call (`pub(crate)` there), which nothing in the app may call.
fn allow_unused_helpers(text: &str) -> String {
    if !text.contains("pub(crate)") {
        return text.to_string();
    }
    let header = text
        .split_inclusive('\n')
        .take_while(|l| l.starts_with("//!"))
        .map(str::len)
        .sum::<usize>();
    format!(
        "{}\n#![allow(dead_code)] // helpers loco-ui's other components call\n{}",
        &text[..header],
        &text[header..]
    )
}

const IMPL_MARK: &str = "\u{0}impl Ui\u{0}\n";

/// One method of an `impl Ui` block.
struct Method {
    name: String,
    /// Its docs, attributes and `fn` line up to the body, `pub` dropped, indented as written.
    signature: String,
    /// `{ .. }`.
    body: String,
}

/// Take the top-level `impl Ui { .. }` blocks out of `source`: the first is replaced by
/// [`IMPL_MARK`], the others removed, and their methods returned.
fn take_impl_ui(source: &str) -> Result<(String, Vec<Method>), String> {
    let code = code_map(source);
    let mut out = String::new();
    let mut methods = Vec::new();
    let mut at = 0;
    let mut marked = false;
    while let Some(start) = ["impl Ui {", "impl crate::Ui {"]
        .iter()
        .filter_map(|p| find_line_start(&source[at..], p).map(|i| at + i))
        .filter(|&i| code[i])
        .min()
    {
        let open = start + source[start..].find('{').unwrap_or(0);
        let close =
            matching(source, &code, open).ok_or("an `impl Ui` block without its closing brace")?;
        methods.extend(methods_in(source, &code, open + 1, close)?);
        out.push_str(source[at..start].trim_end_matches('\n'));
        out.push_str("\n\n");
        if !marked {
            out.push_str(IMPL_MARK);
            out.push('\n');
            marked = true;
        }
        at = close + 1;
        at += source[at..].len() - source[at..].trim_start_matches('\n').len();
    }
    out.push_str(&source[at..]);
    Ok((out, methods))
}

/// The methods in `source[from..to]`, the body of an `impl Ui` block.
fn methods_in(source: &str, code: &[bool], from: usize, to: usize) -> Result<Vec<Method>, String> {
    let bytes = source.as_bytes();
    let not_a_method = || "an `impl Ui` item that is not a method".to_string();
    let mut methods = Vec::new();
    let mut at = from;
    while let Some(skip) = source[at..to].find(|c: char| !c.is_whitespace()) {
        let start = at + skip;
        let open = (start..to)
            .find(|&i| code[i] && bytes[i] == b'{')
            .ok_or_else(not_a_method)?;
        if (start..open).any(|i| code[i] && bytes[i] == b';') {
            return Err(not_a_method());
        }
        let close = matching(source, code, open).ok_or("a method without its closing brace")?;
        let fn_at = (start..open)
            .find(|&i| code[i] && source[i..].starts_with("fn "))
            .ok_or_else(not_a_method)?;
        let name: String = source[fn_at + 3..]
            .chars()
            .take_while(|&c| is_ident(c))
            .collect();
        let line = source[..start].rfind('\n').map_or(0, |i| i + 1);
        let fn_line = source[..fn_at].rfind('\n').map_or(0, |i| i + 1);
        let lead = &source[fn_line..fn_at];
        let indent = &lead[..lead.len() - lead.trim_start().len()];
        let qualifiers = lead.trim_start();
        let qualifiers = ["pub(crate) ", "pub(super) ", "pub "]
            .iter()
            .find_map(|v| qualifiers.strip_prefix(v))
            .unwrap_or(qualifiers);
        let signature = format!(
            "{}{indent}{qualifiers}{}",
            &source[line..fn_line],
            source[fn_at..open].trim_end()
        );
        methods.push(Method {
            name,
            signature,
            body: source[open..=close].to_string(),
        });
        at = close + 1;
    }
    Ok(methods)
}

/// The trait's declaration of a method: docs and `#[must_use]` kept, `#[inline]` dropped.
fn declaration(signature: &str) -> String {
    let lines: Vec<&str> = signature
        .lines()
        .filter(|l| !l.trim_start().starts_with("#[inline"))
        .collect();
    format!("{};\n", lines.join("\n"))
}

/// The impl's definition: docs and `#[must_use]` are the trait's, so they go.
fn definition(signature: &str, body: &str) -> String {
    let lines: Vec<&str> = signature
        .lines()
        .filter(|l| {
            let l = l.trim_start();
            !l.starts_with("///") && !l.starts_with("#[must_use") && !l.starts_with("#[doc")
        })
        .collect();
    format!("{} {body}\n", lines.join("\n"))
}

/// `source` without its top-level `#[cfg(test)] mod .. { .. }`: those test loco-ui itself.
fn drop_unit_tests(source: &str) -> String {
    let code = code_map(source);
    let mut out = String::new();
    let mut at = 0;
    while let Some(found) = find_line_start(&source[at..], "#[cfg(test)]\nmod ") {
        let start = at + found;
        let Some(open) = source[start..].find('{').map(|i| start + i) else {
            break;
        };
        let Some(close) = matching(source, &code, open) else {
            break;
        };
        out.push_str(source[at..start].trim_end_matches('\n'));
        out.push('\n');
        at = close + 1;
    }
    out.push_str(&source[at..]);
    out
}

/// Where `pattern` starts a line in `text`.
fn find_line_start(text: &str, pattern: &str) -> Option<usize> {
    if text.starts_with(pattern) {
        return Some(0);
    }
    text.match_indices(&format!("\n{pattern}"))
        .next()
        .map(|(i, _)| i + 1)
}

/// For each byte of `src`, whether it is code: not in a comment, a string or a char literal.
fn code_map(src: &str) -> Vec<bool> {
    let b = src.as_bytes();
    let mut code = vec![true; b.len()];
    let ident = |i: usize| i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_');
    let mut i = 0;
    while i < b.len() {
        let start = i;
        if b[i] == b'/' && b.get(i + 1) == Some(&b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
        } else if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
            let mut depth = 0;
            while i < b.len() {
                if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
                    depth += 1;
                    i += 2;
                } else if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        } else if b[i] == b'r'
            && (i == 0 || !ident(i - 1) || (b[i - 1] == b'b' && (i < 2 || !ident(i - 2))))
            && matches!(b.get(i + 1), Some(b'"' | b'#'))
        {
            let hashes = b[i + 1..].iter().take_while(|&&c| c == b'#').count();
            if b.get(i + 1 + hashes) != Some(&b'"') {
                i += 1;
                continue;
            }
            let end: Vec<u8> = std::iter::once(b'"')
                .chain(std::iter::repeat_n(b'#', hashes))
                .collect();
            i += 2 + hashes;
            while i < b.len() && !b[i..].starts_with(&end) {
                i += 1;
            }
            i = (i + end.len()).min(b.len());
        } else if b[i] == b'"' {
            i += 1;
            while i < b.len() && b[i] != b'"' {
                i += if b[i] == b'\\' { 2 } else { 1 };
            }
            i = (i + 1).min(b.len());
        } else if b[i] == b'\'' {
            // A char literal ('x', '\n', 'é'), or else a lifetime, which is code.
            let close = if b.get(i + 1) == Some(&b'\\') {
                (i + 2..b.len()).find(|&k| b[k] == b'\'')
            } else {
                src[i + 1..]
                    .chars()
                    .next()
                    .map(|c| i + 1 + c.len_utf8())
                    .filter(|&k| b.get(k) == Some(&b'\''))
            };
            match close {
                Some(k) => i = k + 1,
                None => {
                    i += 1;
                    continue;
                }
            }
        } else {
            i += 1;
            continue;
        }
        code[start..i].iter_mut().for_each(|c| *c = false);
    }
    code
}

/// The `}` that closes the `{` at `open`, counting only braces in code.
fn matching(src: &str, code: &[bool], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (i, &byte) in src.as_bytes().iter().enumerate().skip(open) {
        if !code[i] {
            continue;
        }
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Rename the word `from` (a type, or a `lui!` name) to `to`, but not a path's last segment
/// (`loco_ui::badge::Badge` stays loco-ui's; `super::Badge` is the copy's).
fn rename_word(text: &str, from: &str, to: &str) -> String {
    replace_where(text, from, to, |before, after| {
        let path = before.ends_with("::") && !before.ends_with("super::");
        !before.ends_with(is_ident) && !path && !after.starts_with(is_ident)
    })
}

/// `ui.badge(` → `ui.app_badge(`, for any receiver named `ui`.
fn rename_call(text: &str, method: &str, new: &str) -> String {
    replace_where(
        text,
        &format!("ui.{method}("),
        &format!("ui.{new}("),
        |before, _| !before.ends_with(is_ident),
    )
}

/// `lui-badge`, `lui-badge-ok` → `app-badge`, `app-badge-ok`; not `--lui-badge` (a token)
/// nor `lui-badges`.
fn rename_class(text: &str, class: &str, prefix: &str) -> String {
    let prefix = prefix.replace('_', "-");
    replace_where(
        text,
        &format!("lui-{class}"),
        &format!("{prefix}-{class}"),
        |before, after| {
            !before.ends_with(|c: char| is_ident(c) || c == '-') && !after.starts_with(is_ident)
        },
    )
}

/// Replace each `from` in `text` for which `keep(before, after)` holds.
fn replace_where(text: &str, from: &str, to: &str, keep: impl Fn(&str, &str) -> bool) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for (at, _) in text.match_indices(from) {
        if at < last {
            continue;
        }
        let end = at + from.len();
        if keep(&text[..at], &text[end..]) {
            out.push_str(&text[last..at]);
            out.push_str(to);
            last = end;
        }
    }
    out.push_str(&text[last..]);
    out
}

/// The class the component's CSS is written under: `lui-<module>` in kebab case, or the
/// module's last word (`input_otp` is `lui-otp`).
fn class_root(source: &str, module: &str) -> Option<String> {
    let kebab = module.replace('_', "-");
    let last = module.rsplit('_').next().unwrap_or(module).to_string();
    [kebab, last]
        .into_iter()
        .find(|c| source.contains(&format!(".lui-{c}")))
}

/// Each doctest imports the app's copies (`# use <app>::ui::*;`).
fn doctests_import(text: &str, crate_name: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut open = false;
    for line in text.split_inclusive('\n') {
        out.push_str(line);
        let trimmed = line.trim_start();
        let Some(doc) = ["//! ", "/// ", "//!", "///"]
            .iter()
            .find_map(|p| trimmed.strip_prefix(p).map(|rest| (p, rest)))
        else {
            continue;
        };
        let (marker, rest) = doc;
        let Some(lang) = rest.trim_end().strip_prefix("```") else {
            continue;
        };
        if open {
            open = false;
            continue;
        }
        open = true;
        let rust = lang.is_empty() || lang == "rust" || lang.starts_with("rust,");
        if rust && !lang.contains("ignore") {
            let indent = &line[..line.len() - trimmed.len()];
            let marker = marker.trim_end();
            out.push_str(&format!("{indent}{marker} # use {crate_name}::ui::*;\n"));
        }
    }
    out
}

/// A paragraph under the header's title saying where the file came from.
fn add_note(text: &str, names: &Names) -> String {
    let method = main_builder(names.module).and_then(method_of);
    let call = match method {
        Some(m) => format!(
            "\n//! It is `ui.{}_{m}(..)` (`{}(..)` in `lui!`) after `use crate::ui::*;`; loco-ui's own\n//! `ui.{m}(..)` is unchanged.",
            names.prefix,
            upper_camel(&format!("{}_{m}", names.prefix))
        ),
        None => String::new(),
    };
    let note = format!(
        "//!\n//! Copied from loco-ui {} by `cargo lui add {}`: yours to edit.{call}\n//! `cargo lui add {} --diff` shows how it differs from loco-ui's copy.\n",
        env!("CARGO_PKG_VERSION"),
        names.short,
        names.short
    );
    match text.split_once('\n') {
        Some((title, rest)) if title.starts_with("//! #") => format!("{title}\n{note}{rest}"),
        _ => format!("{}{text}", note.trim_start_matches("//!\n")),
    }
}

/// The lines that differ between `old` and `new`, as `-` and `+` lines with two lines of
/// context and an `@@ -line +line @@` heading per hunk; empty when they are the same.
fn diff(old: &str, new: &str) -> String {
    let a: Vec<&str> = old.lines().collect();
    let b: Vec<&str> = new.lines().collect();
    // Longest common subsequence, from the end.
    let mut lcs = vec![vec![0u32; b.len() + 1]; a.len() + 1];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    // (kind, old line, new line, text): ' ' same, '-' only in old, '+' only in new.
    let mut ops: Vec<(char, usize, usize, &str)> = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            ops.push((' ', i, j, a[i]));
            i += 1;
            j += 1;
        } else if j < b.len() && (i == a.len() || lcs[i][j + 1] > lcs[i + 1][j]) {
            ops.push(('+', i, j, b[j]));
            j += 1;
        } else {
            ops.push(('-', i, j, a[i]));
            i += 1;
        }
    }
    let near = |k: usize| {
        ops[k.saturating_sub(2)..(k + 3).min(ops.len())]
            .iter()
            .any(|op| op.0 != ' ')
    };
    let mut out = String::new();
    let mut in_hunk = false;
    for (k, &(kind, i, j, line)) in ops.iter().enumerate() {
        if !near(k) {
            in_hunk = false;
            continue;
        }
        if !in_hunk {
            out.push_str(&format!("@@ -{} +{} @@\n", i + 1, j + 1));
            in_hunk = true;
        }
        out.push(kind);
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// `cargo lui eject`: copy loco-ui, loco-ui-caps and loco-ui-macros into `vendor/` and point
/// the app's dependency at that copy, so every file of the library is the app's to edit.
pub fn eject(options: &Options) -> Result<(), String> {
    let app = &options.app;
    let files = &options.files;
    let source = source_dir(app)?;
    let workspace = source.parent().map(Path::to_path_buf);
    let mut found: Vec<(String, PathBuf)> = Vec::new();
    for name in ["loco-ui", "loco-ui-caps", "loco-ui-macros"] {
        let sibling = workspace.as_ref().map(|w| w.join(name));
        let dir = match sibling.filter(|d| d.join("Cargo.toml").is_file()) {
            Some(dir) => dir,
            None => package_dirs(app)?
                .into_iter()
                .find(|(n, _)| n == name)
                .map(|(_, d)| d)
                .ok_or_else(|| format!("cannot find the {name} source"))?,
        };
        found.push((name.to_string(), dir));
    }
    let root_deps = workspace
        .as_ref()
        .and_then(|w| fs::read_to_string(w.join("Cargo.toml")).ok())
        .and_then(|t| t.parse::<DocumentMut>().ok())
        .and_then(|d| {
            d.get("workspace")
                .and_then(|w| w.get("dependencies"))
                .and_then(Item::as_table_like)
                .map(|t| {
                    t.iter()
                        .map(|(k, v)| (k.to_string(), v.clone()))
                        .collect::<Vec<_>>()
                })
        })
        .unwrap_or_default();
    for (name, dir) in &found {
        let target = app.join("vendor").join(name);
        copy_tree(dir, &target, files)?;
        let manifest = fs::read_to_string(dir.join("Cargo.toml"))
            .map_err(|e| format!("{}: {e}", dir.display()))?;
        let standalone = standalone_manifest(&manifest, &root_deps)?;
        files.write(&target.join("Cargo.toml"), &standalone)?;
    }
    files.edit(&app.join("Cargo.toml"), point_at_vendor)?;
    println!("done: loco-ui is vendor/loco-ui now; edit it like the app's own code");
    Ok(())
}

/// Copy `from` into `to`, leaving out `target/` and the installer's test app.
fn copy_tree(from: &Path, to: &Path, files: &crate::Files) -> Result<(), String> {
    let entries = fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", from.display()))?;
        let name = entry.file_name();
        let path = entry.path();
        if name == "target" || name == ".git" || path.ends_with("tests/fresh-loco-app") {
            continue;
        }
        if path.is_dir() {
            copy_tree(&path, &to.join(&name), files)?;
        } else if name != "Cargo.toml" || !from.join("src").is_dir() {
            match fs::read_to_string(&path) {
                Ok(text) => files.write(&to.join(&name), &text)?,
                Err(_) if !files.dry_run => {
                    fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
                    fs::copy(&path, to.join(&name))
                        .map_err(|e| format!("{}: {e}", path.display()))?;
                }
                Err(_) => println!("would write {}", to.join(&name).display()),
            }
        }
    }
    Ok(())
}

/// A member's manifest made to build outside its workspace: each `x.workspace = true`
/// dependency spelled out from the workspace's `[workspace.dependencies]`, and the sibling
/// crates found by path (`../loco-ui-caps`).
fn standalone_manifest(manifest: &str, root: &[(String, Item)]) -> Result<String, String> {
    let mut doc: DocumentMut = manifest.parse().map_err(|e| format!("{e}"))?;
    if let Some(package) = doc.get_mut("package").and_then(Item::as_table_like_mut) {
        // The README is the workspace's, one directory up.
        package.remove("readme");
    }
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(deps) = doc.get_mut(section).and_then(Item::as_table_like_mut) else {
            continue;
        };
        let keys: Vec<String> = deps.iter().map(|(k, _)| k.to_string()).collect();
        for key in keys {
            let Some(item) = deps.get(&key) else { continue };
            let inherits = item
                .get("workspace")
                .and_then(Item::as_bool)
                .unwrap_or(false);
            let mut spelled = InlineTable::new();
            if inherits {
                match root.iter().find(|(k, _)| *k == key).map(|(_, v)| v) {
                    Some(Item::Value(Value::String(v))) => {
                        spelled.insert("version", Value::from(v.value().as_str()));
                    }
                    Some(other) => {
                        if let Some(t) = other.as_table_like() {
                            for (k, v) in t.iter() {
                                if let Some(v) = v.as_value() {
                                    spelled.insert(k, v.clone());
                                }
                            }
                        }
                    }
                    None => return Err(format!("{key}.workspace = true, but no workspace has it")),
                }
            }
            let Some(local) = item.as_table_like() else {
                continue;
            };
            if !inherits && !key.starts_with("loco-ui-") {
                continue;
            }
            for (k, v) in local.iter() {
                if k == "workspace" {
                    continue;
                }
                let Some(v) = v.as_value() else { continue };
                if k == "features" {
                    let mut all: Vec<String> = spelled
                        .get("features")
                        .and_then(Value::as_array)
                        .map(|a| {
                            a.iter()
                                .filter_map(|f| f.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    for f in v.as_array().into_iter().flatten().filter_map(Value::as_str) {
                        if !all.iter().any(|x| x == f) {
                            all.push(f.to_string());
                        }
                    }
                    spelled.insert(
                        "features",
                        Value::Array(all.iter().map(String::as_str).collect()),
                    );
                } else {
                    spelled.insert(k, v.clone());
                }
            }
            if key.starts_with("loco-ui-") {
                spelled.insert("path", Value::from(format!("../{key}")));
            }
            deps.insert(&key, Item::Value(Value::InlineTable(spelled)));
        }
    }
    Ok(doc.to_string())
}

/// The app's `loco-ui` dependency becomes `{ path = "vendor/loco-ui", features = [..] }`, its
/// features kept; and a `[workspace]` the app has leaves `vendor/` out, so the copy is built
/// as a dependency, not tested as a member.
fn point_at_vendor(contents: &str) -> Result<String, String> {
    let mut doc: DocumentMut = contents.parse().map_err(|e| format!("{e}"))?;
    let deps = doc
        .get_mut("dependencies")
        .and_then(Item::as_table_like_mut)
        .ok_or("no [dependencies]")?;
    let features = deps
        .get("loco-ui")
        .and_then(|d| d.get("features"))
        .and_then(Item::as_array)
        .cloned();
    let mut dep = InlineTable::new();
    dep.insert("path", Value::from("vendor/loco-ui"));
    if let Some(features) = features {
        dep.insert("features", Value::Array(features));
    }
    deps.insert("loco-ui", Item::Value(Value::InlineTable(dep)));
    if let Some(workspace) = doc.get_mut("workspace").and_then(Item::as_table_like_mut) {
        let exclude = workspace
            .entry("exclude")
            .or_insert(Item::Value(Value::Array(Default::default())));
        if let Some(list) = exclude.as_array_mut()
            && !list.iter().any(|v| v.as_str() == Some("vendor"))
        {
            list.push("vendor");
        }
    }
    Ok(doc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r##"//! # Badge
//!
//! A label.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! assert_eq!(ui.badge("New").render().into_string(), r#"<span class="lui-badge">New</span>"#);
//! assert_eq!(lui! { Badge("New"); }.into_string(), ui.badge("New").render().into_string());
//! ```

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A badge, made by [`Ui::badge`].
pub struct Badge<'a> {
    text: &'a str,
}

impl Ui {
    /// A badge reading `text`.
    #[must_use]
    pub fn badge<'a>(&self, text: &'a str) -> Badge<'a> {
        let _ = '{';
        Badge { text }
    }
}

impl Render for Badge<'_> {
    fn render(&self) -> Markup {
        html! { span class="lui-badge" { (self.text) } }
    }
}

#[cfg(feature = "loco")]
pub fn paged() {}

pub const CSS: &str = ".lui-badge { color: var(--lui-badge-fg); } .lui-badges {}";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inside() { let s = "}"; }
}
"##;

    #[test]
    fn a_copy_takes_the_prefix_and_an_extension_trait() {
        let names = Names::new("badge", "app");
        let copy = vendor(SOURCE, &names, "my_app").unwrap();
        assert!(
            copy.starts_with("//! # AppBadge\n//!\n//! Copied from loco-ui"),
            "{copy}"
        );
        assert!(
            copy.contains("//! ```rust\n//! # use my_app::ui::*;\n//! use loco_ui::prelude::*;"),
            "{copy}"
        );
        assert!(
            copy.contains(
                r#"ui.app_badge("New").render().into_string(), r#"<span class="app-badge">"#
            ),
            "{copy}"
        );
        assert!(copy.contains(r#"lui! { AppBadge("New"); }"#), "{copy}");
        assert!(
            copy.contains("use loco_ui::Ui;\nuse loco_ui::props::{Prop, PropKind};"),
            "{copy}"
        );
        assert!(
            copy.contains(
                "/// A badge, made by [`AppBadgeExt::app_badge`].\npub struct AppBadge<'a> {"
            ),
            "{copy}"
        );
        assert!(copy.contains("pub trait AppBadgeExt {\n    /// A badge reading `text`.\n    #[must_use]\n    fn app_badge<'a>(&self, text: &'a str) -> AppBadge<'a>;\n}"), "{copy}");
        assert!(copy.contains("impl AppBadgeExt for Ui {\n    fn app_badge<'a>(&self, text: &'a str) -> AppBadge<'a> {\n        let _ = '{';\n        AppBadge { text }\n    }\n}"), "{copy}");
        assert!(!copy.contains("impl Ui"), "{copy}");
        assert!(copy.contains("impl Render for AppBadge<'_>"));
        assert!(
            copy.contains("\npub fn paged() {}") && !copy.contains("cfg(feature"),
            "{copy}"
        );
        assert!(
            copy.contains(r#"".app-badge { color: var(--lui-badge-fg); } .lui-badges {}""#),
            "{copy}"
        );
        assert!(
            !copy.contains("mod tests") && !copy.contains("fn inside"),
            "{copy}"
        );
    }

    #[test]
    fn the_code_map_skips_strings_comments_and_chars() {
        let src = r##"a { "}" r#"}"# '}' 'a // } "##;
        let code = code_map(src);
        let open = src.find('{').unwrap();
        assert_eq!(matching(src, &code, open), None);
        let src = "fn f<'a>(x: &'a str) { /* } */ let c = '\\''; }";
        let code = code_map(src);
        assert_eq!(
            matching(src, &code, src.find('{').unwrap()),
            Some(src.len() - 1)
        );
    }

    #[test]
    fn the_css_joins_the_look_once() {
        let look = "static LOOK: Look = Look {\n    css: &[],\n};";
        let one = add_css(look, "crate::ui::app_badge::CSS").unwrap();
        assert!(one.contains("css: &[crate::ui::app_badge::CSS],"));
        let two = add_css(&one, "crate::ui::app_card::CSS").unwrap();
        assert!(two.contains("css: &[crate::ui::app_badge::CSS, crate::ui::app_card::CSS],"));
        assert_eq!(add_css(&two, "crate::ui::app_card::CSS").unwrap(), two);
    }

    #[test]
    fn a_diff_shows_the_changed_lines() {
        assert_eq!(diff("a\nb\nc\n", "a\nb\nc\n"), "");
        let d = diff("a\nb\nc\nd\ne\nf\ng\n", "a\nb\nc\nD\ne\nf\ng\n");
        assert_eq!(d, "@@ -2 +2 @@\n b\n c\n-d\n+D\n e\n f\n");
    }

    #[test]
    fn workspace_dependencies_are_spelled_out() {
        let member = "[package]\nname = \"loco-ui\"\nreadme = \"../README.md\"\n\n[dependencies]\nmaud.workspace = true\naxum = { workspace = true, optional = true, features = [\"multipart\"] }\nloco-ui-caps = { path = \"../loco-ui-caps\", version = \"0.1.0\" }\nhttp = \"1\"\n";
        let root: DocumentMut = "[workspace.dependencies]\nmaud = { version = \"0.27\", features = [\"axum\"] }\naxum = \"0.8\"\n".parse().unwrap();
        let root: Vec<(String, Item)> = root["workspace"]["dependencies"]
            .as_table_like()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        let out = standalone_manifest(member, &root).unwrap();
        assert!(
            out.contains(r#"maud = { version = "0.27", features = ["axum"] }"#),
            "{out}"
        );
        assert!(
            out.contains(
                r#"axum = { version = "0.8", optional = true, features = ["multipart"] }"#
            ),
            "{out}"
        );
        assert!(
            out.contains(r#"loco-ui-caps = { path = "../loco-ui-caps", version = "0.1.0" }"#),
            "{out}"
        );
        assert!(
            out.contains("http = \"1\"") && !out.contains("readme"),
            "{out}"
        );
    }

    #[test]
    fn eject_points_the_dependency_at_vendor() {
        let app = "[workspace]\n\n[dependencies]\nloco-ui = { git = \"x\", rev = \"y\", features = [\"loco\"] }\n";
        let out = point_at_vendor(app).unwrap();
        assert!(
            out.contains(r#"loco-ui = { path = "vendor/loco-ui", features = ["loco"] }"#),
            "{out}"
        );
        assert!(out.contains("exclude = [\"vendor\"]"), "{out}");
        assert_eq!(point_at_vendor(&out).unwrap(), out);
    }
}
