//! Search over the site: every component page, every prop of the builders each page calls,
//! and every docs page and heading, in one index. `/find?q=` lists what matches, or goes
//! straight to the page whose name is the query; the site header's command palette is the
//! search box, fed from the same index, so the palette demonstrates itself on every page.

use crate::code::CODE;
use crate::docs::{DOCS, RENDERED};
use crate::site::{COMPONENTS, page};
use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
};
use loco_ui::prelude::*;
use std::sync::LazyLock;

pub(crate) fn routes() -> Router {
    Router::new().route("/find", get(search_page))
}

/// One thing search can find: what it is, its name, where it is and the text that finds it.
pub(crate) struct Hit {
    pub kind: Kind,
    pub title: String,
    pub href: String,
    pub text: String,
}

/// What a [`Hit`] is, in the order the results list them.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum Kind {
    Component,
    Doc,
    Prop,
}

impl Kind {
    fn heading(self) -> &'static str {
        match self {
            Kind::Component => "Components",
            Kind::Doc => "Docs",
            Kind::Prop => "Props",
        }
    }
}

/// Every component page, every docs page and heading, and every prop (on the first page
/// whose code calls its builder), built once.
pub(crate) static INDEX: LazyLock<Vec<Hit>> = LazyLock::new(|| {
    let mut hits = Vec::new();
    for c in COMPONENTS {
        hits.push(Hit {
            kind: Kind::Component,
            title: c.title.to_string(),
            href: c.href.to_string(),
            text: format!("{} {} {}", c.what, c.built_on, c.when),
        });
    }
    for (doc, rendered) in DOCS.iter().zip(RENDERED.iter()) {
        hits.push(Hit {
            kind: Kind::Doc,
            title: doc.label.to_string(),
            href: doc.href.to_string(),
            text: rendered.title.clone(),
        });
        for h in &rendered.headings {
            hits.push(Hit {
                kind: Kind::Doc,
                title: format!("{} › {}", doc.label, h.text),
                href: format!("{}#{}", doc.href, h.id),
                text: String::new(),
            });
        }
    }
    let mut seen: Vec<&str> = Vec::new();
    for (href, _, _, builders) in CODE.iter() {
        for b in builders.iter() {
            if seen.contains(&b.builder) {
                continue;
            }
            seen.push(b.builder);
            for p in b.props {
                hits.push(Hit {
                    kind: Kind::Prop,
                    title: format!("{}.{}()", b.builder, p.name),
                    href: format!("{href}#props-{}", b.builder.to_lowercase()),
                    text: p.doc.to_string(),
                });
            }
        }
    }
    hits
});

/// The hits whose name or text holds every word of `query`, names that start with it first,
/// then names that hold it, then the rest, each group in index order.
pub(crate) fn search(query: &str) -> Vec<&'static Hit> {
    let query = query.trim().to_lowercase();
    let words: Vec<&str> = query.split_whitespace().collect();
    if words.is_empty() {
        return Vec::new();
    }
    let mut found: Vec<(u8, &Hit)> = INDEX
        .iter()
        .filter_map(|hit| {
            let title = hit.title.to_lowercase();
            let hay = format!("{title} {}", hit.text.to_lowercase());
            words.iter().all(|w| hay.contains(w)).then(|| {
                let rank = if title.starts_with(&query) {
                    0
                } else if title.contains(&query) {
                    1
                } else {
                    2
                };
                (rank, hit)
            })
        })
        .collect();
    found.sort_by_key(|(rank, hit)| (hit.kind, *rank));
    found.into_iter().map(|(_, hit)| hit).collect()
}

/// The page whose name is `query`, ignoring case: a component or a docs page.
pub(crate) fn exact(query: &str) -> Option<&'static str> {
    let query = query.trim();
    INDEX
        .iter()
        .filter(|h| h.kind != Kind::Prop)
        .find(|h| h.title.eq_ignore_ascii_case(query))
        .map(|h| h.href.as_str())
}

/// The header's search box: the command palette over the component and docs pages. It is
/// drawn from a `Ui` without the request's query, so a page's own `?q=` (the combobox's, the
/// palette demo's) never fills it.
pub(crate) fn palette(ui: &Ui) -> Markup {
    let mut quiet = Ui::from(ui.caps);
    quiet.strings = ui.strings;
    let palette = quiet
        .palette("/find")
        .id("site-search")
        .label("Search the docs")
        .group("Components")
        .links(COMPONENTS.iter().map(|c| (c.title, c.href)))
        .group("Docs")
        .links(DOCS.iter().map(|d| (d.label, d.href)));
    palette.render()
}

/// An exact name goes to its page; anything else lists the matches by kind.
async fn search_page(ui: Ui) -> Response {
    let query = ui.param("q").unwrap_or("").trim();
    if let Some(href) = exact(query) {
        return ui.redirect(href).into_response();
    }
    let hits = search(query);
    let kinds = [Kind::Component, Kind::Doc, Kind::Prop];
    let body = html! {
        search {
            form method="get" action="/find" class="lui-find" {
                (ui.input("q", "Search components, props and docs").search().value(query).hide_label().placeholder("Search components, props and docs"))
                (ui.button("Search").primary())
            }
        }
        @if query.is_empty() {
            p class="lui-note" { "Type a component, a prop (" code { "closedby" } ", " code { "maxlength" } ") or a topic (" code { "contrast" } "). An exact name goes straight to its page." }
        } @else if hits.is_empty() {
            p { "Nothing matches “" (query) "”." }
        } @else {
            p class="lui-note" { (hits.len()) @if hits.len() == 1 { " result" } @else { " results" } " for “" (query) "”." }
            @for kind in kinds {
                @let these: Vec<&&Hit> = hits.iter().filter(|h| h.kind == kind).collect();
                @if !these.is_empty() {
                    section class="lui-find-results" aria-labelledby={ "results-" (kind.heading().to_lowercase()) } {
                        h2 id={ "results-" (kind.heading().to_lowercase()) } { (kind.heading()) " " (ui.badge(these.len()).secondary()) }
                        ul { @for hit in these.iter().take(40) {
                            li { a href=(hit.href) { (hit.title) } @if !hit.text.is_empty() && kind != Kind::Component { p class="lui-note" { (excerpt(&hit.text)) } } }
                        } }
                    }
                }
            }
        }
    };
    page(&ui, "Search", body).into_response()
}

/// The first 160 characters of `text`, cut at a word.
fn excerpt(text: &str) -> String {
    if text.chars().count() <= 160 {
        return text.to_string();
    }
    let cut: String = text.chars().take(160).collect();
    match cut.rfind(' ') {
        Some(i) => format!("{}…", &cut[..i]),
        None => format!("{cut}…"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_components_props_and_headings() {
        let first = |q: &str| search(q).first().map(|h| h.href.clone());
        assert_eq!(first("dialog").as_deref(), Some("/dialog"));
        assert!(
            search("closedby")
                .iter()
                .any(|h| h.kind == Kind::Prop && h.href.starts_with("/dialog#props-"))
        );
        assert!(
            search("both schemes").iter().any(|h| h.kind == Kind::Doc
                && h.href == "/docs/theming#both-schemes-in-one-declaration")
        );
        assert_eq!(exact("Command palette"), Some("/palette"));
        assert_eq!(exact("theming"), Some("/docs/theming"));
        assert_eq!(exact("closedby"), None);
        assert!(search("   ").is_empty());
    }
}
