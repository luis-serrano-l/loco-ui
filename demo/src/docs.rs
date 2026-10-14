//! The docs on the site, rendered from the Markdown in the repository: `docs/start/*.md` at
//! `/start`, the guides in `docs/` at `/docs/<name>` and `CHANGELOG.md` at `/changelog`.
//! pulldown-cmark parses them (a dependency of the demo only) and the snippets' highlighter
//! colours their code blocks. Every heading gets the id GitHub would give it, and the h2 and
//! h3 headings make the table of contents beside the page. A link from one file to another
//! becomes a link to its page; a link to any other file in the repository goes to it on
//! GitHub. A test renders every `docs/*.md` and fails on a link to a file or a heading that
//! does not exist.

use crate::site::page;
use axum::{Router, routing::get};
use loco_ui::prelude::*;
use maud::PreEscaped;
use std::sync::LazyLock;

/// Where a link to a repository file that is not a page goes.
const REPO: &str = "https://github.com/luis-serrano-l/loco-ui/blob/main/";

/// One Markdown file served as a page.
pub(crate) struct Doc {
    /// Its path on the site.
    pub href: &'static str,
    /// Its path in the repository.
    pub file: &'static str,
    /// Its name in the sidebar and the search box (the page's title is its `# heading`).
    pub label: &'static str,
    /// The sidebar group it is listed under.
    pub group: &'static str,
    /// Its Markdown.
    pub source: &'static str,
}

/// Every page made from a Markdown file, in sidebar order. A new `docs/*.md` goes here, or in
/// `NOT_SERVED` with the reason.
pub(crate) const DOCS: &[Doc] = &[
    Doc {
        href: "/start",
        file: "docs/start/index.md",
        label: "Getting started",
        group: "Get started",
        source: include_str!("../../docs/start/index.md"),
    },
    Doc {
        href: "/start/axum",
        file: "docs/start/axum.md",
        label: "Axum",
        group: "Get started",
        source: include_str!("../../docs/start/axum.md"),
    },
    Doc {
        href: "/start/loco",
        file: "docs/start/loco.md",
        label: "Loco",
        group: "Get started",
        source: include_str!("../../docs/start/loco.md"),
    },
    Doc {
        href: "/start/any-server",
        file: "docs/start/any-server.md",
        label: "Any server",
        group: "Get started",
        source: include_str!("../../docs/start/any-server.md"),
    },
    Doc {
        href: "/docs/api",
        file: "docs/api.md",
        label: "API rules",
        group: "Guides",
        source: include_str!("../../docs/api.md"),
    },
    Doc {
        href: "/docs/components",
        file: "docs/components.md",
        label: "Write your own component",
        group: "Guides",
        source: include_str!("../../docs/components.md"),
    },
    Doc {
        href: "/docs/theming",
        file: "docs/theming.md",
        label: "Theming",
        group: "Guides",
        source: include_str!("../../docs/theming.md"),
    },
    Doc {
        href: "/docs/state",
        file: "docs/state.md",
        label: "State",
        group: "Guides",
        source: include_str!("../../docs/state.md"),
    },
    Doc {
        href: "/docs/caps",
        file: "docs/caps.md",
        label: "Capabilities",
        group: "Guides",
        source: include_str!("../../docs/caps.md"),
    },
    Doc {
        href: "/docs/loco",
        file: "docs/loco.md",
        label: "Loco guide",
        group: "Guides",
        source: include_str!("../../docs/loco.md"),
    },
    Doc {
        href: "/docs/ergonomics",
        file: "docs/ergonomics.md",
        label: "Ergonomics",
        group: "Guides",
        source: include_str!("../../docs/ergonomics.md"),
    },
    Doc {
        href: "/docs/latency",
        file: "docs/latency.md",
        label: "Latency",
        group: "Guides",
        source: include_str!("../../docs/latency.md"),
    },
    Doc {
        href: "/docs/comparison",
        file: "docs/comparison.md",
        label: "Comparison",
        group: "About",
        source: include_str!("../../docs/comparison.md"),
    },
    Doc {
        href: "/docs/audiences",
        file: "docs/audiences.md",
        label: "Who it is for",
        group: "About",
        source: include_str!("../../docs/audiences.md"),
    },
    Doc {
        href: "/changelog",
        file: "CHANGELOG.md",
        label: "Changelog",
        group: "About",
        source: include_str!("../../CHANGELOG.md"),
    },
];

/// Markdown files under `docs/` that are not pages, and why (the test that keeps [`DOCS`]
/// complete reads it). The link check still reads them.
#[cfg(test)]
pub(crate) const NOT_SERVED: &[(&str, &str)] = &[(
    "docs/launch-post.md",
    "a draft for the owner to post (BLOCKED.md), not documentation",
)];

/// A heading of a rendered file: its level, its id and its text.
pub(crate) struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// A Markdown file as HTML: the text of its `# heading`, the body after it, its headings and
/// the relative links it makes (as written, for the link check).
pub(crate) struct Rendered {
    pub title: String,
    pub html: String,
    pub headings: Vec<Heading>,
    #[cfg_attr(not(test), expect(dead_code, reason = "the link check reads it"))]
    pub links: Vec<String>,
}

/// Every [`DOCS`] entry rendered once, in the same order.
pub(crate) static RENDERED: LazyLock<Vec<Rendered>> =
    LazyLock::new(|| DOCS.iter().map(|d| render(d.file, d.source)).collect());

/// One GET route per [`DOCS`] entry.
pub(crate) fn routes() -> Router {
    DOCS.iter()
        .enumerate()
        .fold(Router::new(), |router, (i, doc)| {
            router.route(
                doc.href,
                get(move |ui: Ui| async move { doc_page(&ui, doc, &RENDERED[i]) }),
            )
        })
}

/// A doc's page: its body beside the table of contents, and where its source is.
fn doc_page(ui: &Ui, doc: &Doc, rendered: &Rendered) -> Page {
    // A long file (the changelog) lists its h2 headings only.
    let toc: Vec<&Heading> = if rendered.headings.len() > 40 {
        rendered.headings.iter().filter(|h| h.level == 2).collect()
    } else {
        rendered.headings.iter().collect()
    };
    let body = html! {
        div class="lui-doc" {
            div class="lui-doc-grid" {
                @if toc.len() > 1 {
                    nav class="lui-doc-toc" aria-label="On this page" {
                        p { "On this page" }
                        ul { @for h in &toc {
                            li class=[(h.level == 3).then_some("lui-doc-toc-sub")] { a href={ "#" (h.id) } { (h.text) } }
                        } }
                    }
                }
                article class="lui-doc-body" { (PreEscaped(&rendered.html)) }
            }
            p class="lui-note lui-doc-source" { "This page is " a href={ (REPO) (doc.file) } { code { (doc.file) } } " in the repository." }
        }
    };
    page(ui, &rendered.title, body)
}

/// `source` (the file at `file`, a path from the repository root) as HTML.
pub(crate) fn render(file: &str, source: &str) -> Rendered {
    use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let events: Vec<Event> = Parser::new_ext(source, options).collect();
    let mut out = Vec::with_capacity(events.len());
    let (mut title, mut headings, mut links, mut ids) =
        (String::new(), Vec::new(), Vec::new(), Vec::<String>::new());
    let mut i = 0;
    while i < events.len() {
        match &events[i] {
            Event::Start(Tag::Heading { level, .. }) => {
                let end = (events[i..].iter())
                    .position(|e| matches!(e, Event::End(TagEnd::Heading(_))))
                    .map_or(events.len(), |p| i + p);
                let text: String = (events[i + 1..end].iter())
                    .filter_map(|e| match e {
                        Event::Text(t) | Event::Code(t) => Some(t.as_ref()),
                        _ => None,
                    })
                    .collect();
                if *level == HeadingLevel::H1 && title.is_empty() {
                    // The page's own title, which the shell shows as its h1.
                    title = text;
                    i = end + 1;
                    continue;
                }
                let id = unique(&mut ids, slug(&text));
                let level = *level as u8;
                if level == 2 || level == 3 {
                    headings.push(Heading {
                        level,
                        id: id.clone(),
                        text,
                    });
                }
                out.push(Event::Html(format!("<h{level} id=\"{id}\">").into()));
                out.extend(
                    events[i + 1..end]
                        .iter()
                        .cloned()
                        .map(|e| link(file, e, &mut links)),
                );
                out.push(Event::Html(format!("</h{level}>\n").into()));
                i = end + 1;
                continue;
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(info) => info.split(',').next().unwrap_or("").trim(),
                    CodeBlockKind::Indented => "",
                };
                let mut code = String::new();
                let mut j = i + 1;
                while let Some(Event::Text(t)) = events.get(j) {
                    code.push_str(t);
                    j += 1;
                }
                out.push(Event::Html(code_block(lang, &code).into_string().into()));
                i = j + 1;
                continue;
            }
            Event::Start(Tag::Table(_)) => {
                out.push(Event::Html(
                    r#"<div class="lui-doc-table" tabindex="0" role="region" aria-label="Table">"#
                        .into(),
                ));
                out.push(events[i].clone());
            }
            Event::End(TagEnd::Table) => {
                out.push(events[i].clone());
                out.push(Event::Html("</div>\n".into()));
            }
            e => out.push(link(file, e.clone(), &mut links)),
        }
        i += 1;
    }
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, out.into_iter());
    Rendered {
        title,
        html,
        headings,
        links,
    }
}

/// A link event with its destination resolved for the site (and noted when relative); any
/// other event as it is.
fn link<'a>(
    file: &str,
    event: pulldown_cmark::Event<'a>,
    links: &mut Vec<String>,
) -> pulldown_cmark::Event<'a> {
    use pulldown_cmark::{Event, Tag};
    match event {
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            if relative(&dest_url) {
                links.push(dest_url.to_string());
            }
            Event::Start(Tag::Link {
                link_type,
                dest_url: resolve(file, &dest_url).into(),
                title,
                id,
            })
        }
        e => e,
    }
}

/// A fenced block: highlighted (Rust, shell, CSS, HTML, YAML; anything else as plain text) in
/// a scrolling region the enhancement script gives a copy button. Rust's hidden doctest lines
/// (`# …`) are left out, as rustdoc does.
fn code_block(lang: &str, code: &str) -> Markup {
    let shown: String = if matches!(lang, "rust" | "rs") {
        code.lines()
            .filter(|l| !(l.trim() == "#" || l.trim_start().starts_with("# ")))
            .map(|l| format!("{l}\n"))
            .collect()
    } else {
        code.to_string()
    };
    let token = match lang {
        "rust" => "rs",
        "shell" | "bash" => "sh",
        other => other,
    };
    let label = if lang.is_empty() { "Code" } else { lang };
    html! {
        figure class="lui-doc-code" data-lui="copy" {
            div tabindex="0" role="region" aria-label={ (label) " example" } {
                pre { code { (crate::code::highlight_as(shown.trim_end(), token)) } }
            }
        }
    }
}

/// Whether a link points into the repository (not a URL, not a fragment of the same page).
fn relative(url: &str) -> bool {
    !(url.contains("://")
        || url.starts_with('#')
        || url.starts_with('/')
        || url.starts_with("mailto:"))
}

/// Where a link written in `file` to `url` goes on the site: another file's page, with its
/// fragment, or the file on GitHub. URLs and fragments stay as written.
pub(crate) fn resolve(file: &str, url: &str) -> String {
    if !relative(url) {
        return url.to_string();
    }
    let (path, fragment) = match url.split_once('#') {
        Some((p, f)) => (p, format!("#{f}")),
        None => (url, String::new()),
    };
    let target = join(file, path);
    match DOCS.iter().find(|d| d.file == target) {
        Some(doc) => format!("{}{fragment}", doc.href),
        None => format!("{REPO}{target}{fragment}"),
    }
}

/// `path` relative to the folder of `file`, both from the repository root, with `.` and `..`
/// worked out: `join("docs/start/axum.md", "../theming.md")` is `docs/theming.md`.
pub(crate) fn join(file: &str, path: &str) -> String {
    let mut parts: Vec<&str> = file.split('/').collect();
    parts.pop();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            p => parts.push(p),
        }
    }
    parts.join("/")
}

/// The id GitHub gives a heading: lower case, punctuation dropped, spaces as hyphens.
fn slug(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('-'),
            c if c.is_alphanumeric() || c == '-' || c == '_' => Some(c),
            _ => None,
        })
        .collect()
}

/// `id`, or `id-1`, `id-2`… when an earlier heading took it, as GitHub numbers repeats.
fn unique(seen: &mut Vec<String>, id: String) -> String {
    let base = if id.is_empty() {
        "section".to_string()
    } else {
        id
    };
    let mut id = base.clone();
    let mut n = 0;
    while seen.contains(&id) {
        n += 1;
        id = format!("{base}-{n}");
    }
    seen.push(id.clone());
    id
}

/// Markdown inline text (a component header's paragraph) as HTML, without the paragraph
/// around it. Rustdoc's intra-doc links (`` [`Palette::exact`] ``) become plain code.
pub(crate) fn inline(text: &str) -> Markup {
    let text = text.replace("[`", "`").replace("`]", "`");
    let rendered = render("loco-ui/src/lib.rs", &text).html;
    let rendered = rendered.trim();
    let rendered = rendered
        .strip_prefix("<p>")
        .and_then(|r| r.strip_suffix("</p>"))
        .unwrap_or(rendered);
    PreEscaped(rendered.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// The repository root, where every `file` path starts.
    fn root() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
    }

    /// Every Markdown file under `docs/` (and the changelog), as a path from the root.
    fn markdown() -> Vec<String> {
        let mut files = vec!["CHANGELOG.md".to_string()];
        for dir in ["docs", "docs/start"] {
            for entry in std::fs::read_dir(root().join(dir)).unwrap() {
                let name = entry.unwrap().file_name().into_string().unwrap();
                if name.ends_with(".md") {
                    files.push(format!("{dir}/{name}"));
                }
            }
        }
        files.sort();
        files
    }

    /// Every `docs/*.md` renders, and every relative link in it names a file that exists and,
    /// when it has a fragment into a Markdown file, a heading that file has.
    #[test]
    fn every_doc_renders_and_its_links_resolve() {
        let mut broken = Vec::new();
        for file in markdown() {
            let source = std::fs::read_to_string(root().join(&file)).unwrap();
            let rendered = render(&file, &source);
            assert!(!rendered.title.is_empty(), "{file}: no # heading");
            assert!(!rendered.html.contains("<h1"), "{file}: a second # heading");
            for url in &rendered.links {
                let (path, fragment) = url.split_once('#').unwrap_or((url, ""));
                let target = join(&file, path);
                if !root().join(&target).exists() {
                    broken.push(format!("{file}: {url} ({target} does not exist)"));
                    continue;
                }
                if !fragment.is_empty() && target.ends_with(".md") {
                    let text = std::fs::read_to_string(root().join(&target)).unwrap();
                    let ids = render(&target, &text).html;
                    if !ids.contains(&format!("id=\"{fragment}\"")) {
                        broken.push(format!("{file}: {url} (no heading #{fragment})"));
                    }
                }
            }
        }
        assert!(broken.is_empty(), "broken links:\n{}", broken.join("\n"));
    }

    /// Each Markdown file under `docs/` is a page or says why it is not, and each page's
    /// file is where it says.
    #[test]
    fn every_doc_is_served_or_listed_as_not() {
        for file in markdown() {
            assert!(
                DOCS.iter().any(|d| d.file == file) || NOT_SERVED.iter().any(|n| n.0 == file),
                "{file}: add it to docs::DOCS (or NOT_SERVED, with the reason)"
            );
        }
        for doc in DOCS {
            let on_disk = std::fs::read_to_string(root().join(doc.file)).unwrap();
            assert_eq!(
                on_disk, doc.source,
                "{}: include_str! of another file",
                doc.file
            );
            assert!(
                crate::PATHS.contains(&doc.href),
                "{}: not in PATHS",
                doc.href
            );
        }
    }

    #[test]
    fn links_become_pages_or_github_and_headings_get_github_ids() {
        assert_eq!(
            join("docs/start/axum.md", "../theming.md"),
            "docs/theming.md"
        );
        assert_eq!(
            resolve("docs/start/axum.md", "../theming.md#tokens"),
            "/docs/theming#tokens"
        );
        assert_eq!(resolve("docs/start/index.md", "axum.md"), "/start/axum");
        assert_eq!(resolve("docs/api.md", "../CHANGELOG.md"), "/changelog");
        assert_eq!(
            resolve("docs/loco.md", "../examples/loco-app"),
            format!("{REPO}examples/loco-app")
        );
        assert_eq!(
            resolve("docs/api.md", "https://x.dev/a.md"),
            "https://x.dev/a.md"
        );
        let r = render(
            "docs/x.md",
            "# Title\n\n## Use with `Loco`\n\n## Use with `Loco`\n\n```rust\n# hidden\nlet x = 1;\n```\n\n| a |\n|---|\n| b |\n",
        );
        assert_eq!(r.title, "Title");
        let ids: Vec<&str> = r.headings.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(ids, ["use-with-loco", "use-with-loco-1"]);
        assert!(
            r.html
                .contains(r#"<h2 id="use-with-loco">Use with <code>Loco</code></h2>"#),
            "{}",
            r.html
        );
        assert!(r.html.contains(r#"data-lui="copy""#) && !r.html.contains("hidden"));
        assert!(r.html.contains(r#"<div class="lui-doc-table""#));
        assert_eq!(
            inline("A [`Palette::exact`] and `code`.").into_string(),
            "A <code>Palette::exact</code> and <code>code</code>."
        );
    }
}
