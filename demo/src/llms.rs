//! The docs for AI tools: `llms.txt` (an index, in the [llmstxt.org](https://llmstxt.org)
//! shape), `llms-full.txt` (every component in one file) and one Markdown file per component,
//! made from `loco_ui::props()`, `loco_ui::spec::SPECS` and each module's `//!` header.
//! `cargo run -p demo -- spec write` writes them into `docs/` (a test keeps those in step),
//! the demo serves them at `/llms.txt`, `/llms-full.txt` and `/<component>.md`, a component
//! page asked for with `Accept: text/markdown` answers with its components' Markdown, and the
//! snapshot exports them.

use crate::site::COMPONENTS;
use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, header},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
};
use loco_ui::props::{Component, PropKind};
use loco_ui::spec::{ComponentSpec, NeedsJs, SPECS};
use std::sync::LazyLock;

include!(concat!(env!("OUT_DIR"), "/sources.rs"));

/// One documented module: its file under `loco-ui/src/` without `.rs`, the name its `.md`
/// goes by (the last part of the path), its spec entry and its builders.
struct Doc {
    module: &'static str,
    name: &'static str,
    spec: Option<&'static ComponentSpec>,
    builders: Vec<&'static Component>,
}

/// Every module with a spec entry, in README order, then the builders' modules the spec does
/// not list (the blocks).
static DOCS: LazyLock<Vec<Doc>> = LazyLock::new(|| {
    let mut modules: Vec<&'static str> = SPECS.iter().map(|s| s.module).collect();
    for c in loco_ui::props() {
        if !modules.contains(&c.module) {
            modules.push(c.module);
        }
    }
    (modules.into_iter())
        .map(|module| Doc {
            module,
            name: module.rsplit('/').next().unwrap_or(module),
            spec: SPECS.iter().find(|s| s.module == module),
            builders: (loco_ui::props().iter())
                .filter(|c| c.module == module)
                .collect(),
        })
        .collect()
});

/// The modules each demo page's snippet calls, by page href.
static PAGES: LazyLock<Vec<(&'static str, Vec<&'static str>)>> = LazyLock::new(|| {
    (COMPONENTS.iter())
        .map(|c| {
            let mut modules: Vec<&'static str> = Vec::new();
            for b in crate::code::called(&crate::code::snippet(c.href).1) {
                if !modules.contains(&b.module) {
                    modules.push(b.module);
                }
            }
            (c.href, modules)
        })
        .collect()
});

/// Every generated file, as `(path under docs/, text)`: `llms.txt`, `llms-full.txt`, then
/// `components/<name>.md` for each component.
pub fn files() -> Vec<(String, String)> {
    let mut files = vec![
        ("llms.txt".to_string(), index()),
        ("llms-full.txt".to_string(), full()),
    ];
    for doc in DOCS.iter() {
        files.push((format!("components/{}.md", doc.name), markdown(doc)));
    }
    files
}

/// `/llms.txt`, `/llms-full.txt` and `/<name>.md` for every component.
pub(crate) fn routes() -> Router {
    let text = |body: String, kind: &'static str| ([(header::CONTENT_TYPE, kind)], body);
    let mut router = Router::new()
        .route(
            "/llms.txt",
            get(move || async move { text(index(), "text/plain; charset=utf-8") }),
        )
        .route(
            "/llms-full.txt",
            get(move || async move { text(full(), "text/plain; charset=utf-8") }),
        );
    for doc in DOCS.iter() {
        router = router.route(
            &format!("/{}.md", doc.name),
            get(move || async move { text(markdown(doc), MARKDOWN) }),
        );
    }
    router
}

const MARKDOWN: &str = "text/markdown; charset=utf-8";

/// Content negotiation: a GET of a component page whose `Accept` puts `text/markdown` before
/// `text/html` (or names no HTML at all) gets the Markdown of the components the page shows.
/// Both answers carry `Vary: Accept`.
pub(crate) async fn negotiate(request: Request, next: Next) -> Response {
    let path = request.uri().path();
    let Some((_, modules)) = PAGES.iter().find(|p| p.0 == path && !p.1.is_empty()) else {
        return next.run(request).await;
    };
    let accept = (request.headers().get(header::ACCEPT))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let rank = |kind: &str| {
        (accept.split(',')).position(|t| t.split(';').next().is_some_and(|t| t.trim() == kind))
    };
    let wants = match (rank("text/markdown"), rank("text/html")) {
        (Some(md), Some(html)) => md < html,
        (Some(_), None) => true,
        _ => false,
    };
    let mut response = if wants && request.method() == axum::http::Method::GET {
        let body: Vec<String> = (modules.iter())
            .filter_map(|m| DOCS.iter().find(|d| d.module == *m))
            .map(markdown)
            .collect();
        ([(header::CONTENT_TYPE, MARKDOWN)], body.join("\n")).into_response()
    } else {
        next.run(request).await
    };
    (response.headers_mut()).append(header::VARY, HeaderValue::from_static("accept"));
    response
}

/// The index: what the library is, then one line per component linking its `.md`.
fn index() -> String {
    let mut out = String::from(INTRO);
    out.push_str("\n## Components\n\n");
    for doc in DOCS.iter() {
        let lede = first_sentence(&header(doc.module));
        out.push_str(&format!("- [{}]({}.md): {lede}\n", title(doc), doc.name));
    }
    out.push_str("\n## Optional\n\n- [Every component in one file](llms-full.txt)\n");
    out
}

/// Every component's Markdown after the index's introduction.
fn full() -> String {
    let mut out = String::from(INTRO);
    for doc in DOCS.iter() {
        out.push('\n');
        out.push_str(&markdown(doc));
    }
    out
}

const INTRO: &str = "# loco-ui

> A component library for Rust servers (Axum + Maud) that needs no JavaScript. Every component is a builder reached from `ui` (`ui.dialog(\"Delete\")`) or written like an element inside `lui!`, rendering plain HTML; interactivity comes from the HTML and CSS platform and ordinary form posts. One optional script, `/lui/enhance.js`, updates marked regions in place.

Start a handler from `ui: Ui` (an Axum extractor, or `Ui::from_request(path, query, cookie)` on any server), build the page with `ui.page(title, html! { .. })` and return it. Required arguments go in the call; everything else is a chained setter. Each component below lists its constructors, its setters (props), the platform features it needs with the first Chrome, Firefox and Safari versions, its fallback, and its documentation.
";

/// One component's page: constructors, props, platform, then its `//!` header.
fn markdown(doc: &Doc) -> String {
    let body = header(doc.module);
    let mut out = format!(
        "# {}\n\n`loco_ui::{}`",
        title(doc),
        doc.module.replace('/', "::")
    );
    if let Some((href, _)) = PAGES.iter().find(|p| p.1.contains(&doc.module)) {
        out.push_str(&format!(" · demo: `{href}`"));
    }
    out.push_str("\n\n");
    if !doc.builders.is_empty() {
        out.push_str("## Constructors\n\n| `lui!` | Constructor | Status |\n|---|---|---|\n");
        for b in &doc.builders {
            for call in b.calls {
                out.push_str(&format!(
                    "| `{}` | `{call}` | {} |\n",
                    b.lui(),
                    b.status.as_str()
                ));
            }
        }
        out.push('\n');
    }
    for b in doc.builders.iter().filter(|b| !b.props.is_empty()) {
        out.push_str(&format!("## Props of `{}`\n\n", b.builder));
        out.push_str("| Prop | Kind | Arguments | Default | HTML | What it does |\n|---|---|---|---|---|---|\n");
        for p in b.props {
            let kind = match p.kind {
                PropKind::Choice { of } => format!("choice: {}", of.join(", ")),
                kind => kind.as_str().to_string(),
            };
            let code = |s: &str| {
                if s.is_empty() {
                    String::new()
                } else {
                    format!("`{s}`")
                }
            };
            out.push_str(&format!(
                "| `{}` | {kind} | {} | {} | {} | {} |\n",
                p.name,
                cell(&code(p.args)),
                cell(&code(p.default)),
                cell(&code(p.attr)),
                cell(p.doc)
            ));
        }
        out.push('\n');
    }
    for b in doc.builders.iter().filter(|b| !b.keys.is_empty()) {
        out.push_str(&format!("## Keyboard of `{}`\n\n", b.builder));
        out.push_str("| Keys | What they do | Needs the script |\n|---|---|---|\n");
        for k in b.keys {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                cell(&format!("`{}`", k.keys)),
                cell(k.does),
                if k.script { "yes" } else { "no" }
            ));
        }
        out.push('\n');
    }
    let parts = crate::reference::anatomy(doc.module);
    if !parts.is_empty() {
        out.push_str("## Anatomy\n\n| Class | Tokens it reads |\n|---|---|\n");
        for (class, tokens) in &parts {
            let tokens: Vec<String> = tokens.iter().map(|t| format!("`{t}`")).collect();
            out.push_str(&format!("| `.{class}` | {} |\n", cell(&tokens.join(", "))));
        }
        out.push('\n');
    }
    if let Some(spec) = doc.spec {
        out.push_str("## Platform\n\n");
        if !spec.features.is_empty() {
            out.push_str("| Feature | Chrome | Firefox | Safari |\n|---|---|---|---|\n");
            for f in spec.features {
                let b = f.baseline;
                out.push_str(&format!(
                    "| `{}` | {} | {} | {} |\n",
                    cell(f.name),
                    b.chrome,
                    b.firefox,
                    b.safari
                ));
            }
            out.push('\n');
        }
        let needs = match spec.needs_js {
            NeedsJs::No => "no".to_string(),
            NeedsJs::Partial(part) => format!("partly: {part}"),
        };
        out.push_str(&format!(
            "- Fallback: {}\n- Needs script: {needs}\n- Responsive by: {}\n- Look after: {}\n\n",
            spec.fallback,
            spec.responsive.as_str(),
            spec.look
        ));
    }
    if !body.trim().is_empty() {
        out.push_str("## Documentation\n\n");
        out.push_str(body.trim());
        out.push('\n');
    }
    out
}

/// The human name: the spec's, else the first builder's, else the module's.
fn title(doc: &Doc) -> &'static str {
    (doc.spec.map(|s| s.name))
        .or_else(|| doc.builders.first().map(|b| b.builder))
        .unwrap_or(doc.module)
}

/// A module's `//!` header as Markdown without its `# Title`, headings two levels down
/// (under the page's `##`), the lines rustdoc hides (`# use ..`) left out of Rust blocks.
fn header(module: &str) -> String {
    let source = (LIBRARY_SOURCES.iter())
        .find(|(m, _)| *m == module)
        .map_or("", |(_, s)| s);
    let mut lines = (source.lines())
        .map_while(|l| l.strip_prefix("//!"))
        .map(|l| l.strip_prefix(' ').unwrap_or(l));
    lines.next();
    // `Some(is_rust)` inside a fence.
    let mut fence: Option<bool> = None;
    let mut out = Vec::new();
    for line in lines {
        if let Some(lang) = line.trim_start().strip_prefix("```") {
            fence = match fence {
                Some(_) => None,
                None => Some(lang.is_empty() || lang.starts_with("rust")),
            };
            out.push(line.to_string());
        } else if fence == Some(true) && (line.trim() == "#" || line.trim_start().starts_with("# "))
        {
            continue;
        } else if fence.is_none() && line.starts_with('#') {
            out.push(format!("##{}", plain_links(line)));
        } else if fence.is_none() {
            out.push(plain_links(line));
        } else {
            out.push(line.to_string());
        }
    }
    out.join("\n")
}

/// `line` with rustdoc's links to items, which mean nothing outside rustdoc, as plain text:
/// `` [`Tokens`] `` is `` `Tokens` `` and `` [`Tone`](crate::variant::Tone) `` is `` `Tone` ``.
/// Links to a URL, a path or a fragment stay.
fn plain_links(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(open) = rest.find('[') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let Some(close) = after.find(']') else {
            out.push_str(&rest[open..]);
            return out;
        };
        let text = &after[..close];
        let tail = &after[close + 1..];
        if let Some(target) = tail.strip_prefix('(')
            && let Some(end) = target.find(')')
        {
            let url = &target[..end];
            if url.contains("::") || !url.contains(['/', '.', '#']) {
                out.push_str(text);
            } else {
                out.push_str(&rest[open..open + 1 + close + 1 + end + 2]);
            }
            rest = &target[end + 1..];
        } else if text.starts_with('`') && text.ends_with('`') {
            out.push_str(text);
            rest = tail;
        } else {
            out.push('[');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// The header's first sentence, on one line.
fn first_sentence(body: &str) -> String {
    let para: Vec<&str> = (body.lines())
        .skip_while(|l| l.trim().is_empty())
        .take_while(|l| !l.trim().is_empty())
        .map(str::trim)
        .collect();
    let para = para.join(" ");
    match para.find(". ") {
        Some(i) => para[..=i].to_string(),
        None => para,
    }
}

/// `text` safe inside a Markdown table cell.
fn cell(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}
