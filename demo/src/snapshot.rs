//! The static snapshot for GitHub Pages: every GET page in [`PATHS`] rendered to a file, with
//! the enhancement script taken out, links rewritten to relative `.html` files and a banner
//! saying what needs the real server. `cargo run -p demo -- snapshot <dir>` (or
//! `scripts/snapshot.sh`) writes it; `<dialog>`, `popover`, `<details>` and tooltips keep
//! working there because they never needed the server.
//!
//! Each page is rendered twice, as a browser with every capability and as one with none. The
//! modern render is `<name>.html`; where the baseline render differs it is written beside it
//! as `<name>.baseline.html`, and the banner links the two.
//!
//! The whole set is exported again in Spanish under `es/`. The language buttons post to
//! `/lang` on the server; here each is a link to the same page in the other set.

use crate::{PATHS, router};
use axum::{body::Body, http::Request};
use loco_ui::caps::Cap;
use loco_ui::prelude::*;
use tower::ServiceExt;

/// The text every exported page carries (the test looks for it).
pub const BANNER: &str = "Static snapshot";

/// One exported file: its name under the output directory and its HTML (or text, for the
/// `llms` files).
pub struct File {
    pub name: String,
    pub html: String,
}

/// The languages of the snapshot: the tag, the button's text and the directory its pages
/// are written under. English is the root; the demo's other language sits beside it.
const LANGUAGES: [(&str, &str, &str); 2] = [("en", "English", ""), ("es", "Español", "es/")];

/// Every page of the snapshot: for each language, modern variants in [`PATHS`] order with a
/// baseline beside each one that differs; then `llms.txt`, `llms-full.txt` and each
/// component's `.md` ([`crate::llms::files`]).
pub async fn pages() -> Vec<File> {
    let modern = Cap::ALL
        .map(|c| format!("lui-cap-{}=1", c.name()))
        .join("; ");
    let names = names();
    let mut files = Vec::new();
    for (tag, _, dir) in LANGUAGES {
        let lang = format!("{}={tag}", loco_ui::i18n::LANG_COOKIE);
        for (path, name) in PATHS.iter().zip(&names) {
            let rich = render(path, &format!("{lang}; {modern}")).await;
            let plain = render(path, &lang).await;
            let baseline = (plain != rich).then(|| format!("{name}.baseline"));
            let other = baseline.as_deref();
            files.push(File {
                name: format!("{dir}{name}.html"),
                html: finish(&rich, &names, other, false, (tag, name)),
            });
            if let Some(b) = other {
                files.push(File {
                    name: format!("{dir}{b}.html"),
                    html: finish(&plain, &names, Some(name), true, (tag, name)),
                });
            }
        }
    }
    // The AI-readable docs beside the pages, at the paths the server answers them on.
    for (path, text) in crate::llms::files() {
        let name = path
            .strip_prefix("components/")
            .unwrap_or(&path)
            .to_string();
        files.push(File { name, html: text });
    }
    files
}

/// The file name (without `.html`) for each entry of [`PATHS`]: `/` is `index`, `/app/signin`
/// is `app-signin`, and a route that appears again with another query gets `-2`, `-3`.
pub fn names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for path in PATHS {
        let route = route(path);
        let base = match route.trim_matches('/') {
            "" => "index".to_string(),
            r => r.replace('/', "-"),
        };
        let seen = PATHS
            .iter()
            .take_while(|p| **p != path)
            .filter(|p| self::route(p) == route)
            .count();
        names.push(if seen == 0 {
            base
        } else {
            format!("{base}-{}", seen + 1)
        });
    }
    names
}

fn route(path: &str) -> &str {
    path.split(['?', '#']).next().unwrap_or(path)
}

async fn render(path: &str, cookie: &str) -> String {
    let request = Request::get(path)
        .header("cookie", cookie)
        .body(Body::empty())
        .unwrap();
    let response = router().oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let html = String::from_utf8(body.to_vec()).unwrap();
    // The capability beacons ask `/lui/caps` for a cookie; a static host has neither.
    let beacons = r#"<div class="lui-caps" aria-hidden="true">"#;
    match html.find(beacons) {
        Some(i) => {
            let end = i + html[i..].find("</div>").map_or(0, |j| j + "</div>".len());
            format!("{}{}", &html[..i], &html[end..])
        }
        None => html,
    }
}

/// Drop the script, rewrite `href`/`action`, turn the language buttons into links and put
/// the banner at the top of `<main>`. `page` is the page's language tag and file name.
fn finish(
    html: &str,
    names: &[String],
    other: Option<&str>,
    baseline: bool,
    page: (&str, &str),
) -> String {
    let up = LANGUAGES
        .iter()
        .find(|(tag, ..)| *tag == page.0)
        .map_or("", |(_, _, dir)| if dir.is_empty() { "" } else { "../" });
    let html = html.replace(&loco_ui::enhance::script_tag().into_string(), "");
    let html = rewrite(&html, "href=\"", names, up);
    let html = rewrite(&html, "action=\"", names, up);
    let html = rewrite(&html, "src=\"", names, up);
    let html = languages(&html, page, up);
    let at = html
        .find("<main")
        .or_else(|| html.find("<body"))
        .and_then(|i| html[i..].find('>').map(|j| i + j + 1))
        .unwrap_or(0);
    format!(
        "{}{}{}",
        &html[..at],
        banner(other, baseline).into_string(),
        &html[at..]
    )
}

/// Point every root-relative URL that names an exported page (or a gallery shot) at its file: the exact path and
/// query if [`PATHS`] has it, else the first export of that route. Anything else (POST
/// targets, `/lui/*`, `/table.csv`) stays as it is and fails on a static host, as the banner
/// says.
fn rewrite(html: &str, attr: &str, names: &[String], up: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(i) = rest.find(attr) {
        let start = i + attr.len();
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let end = rest.find('"').unwrap_or(rest.len());
        let url = &rest[..end];
        out.push_str(&target(url, names, up).unwrap_or_else(|| url.to_string()));
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// `up` is the way from the page's directory to the snapshot's root (`../` under `es/`):
/// pages link to their own language's files, the shots are shared.
fn target(url: &str, names: &[String], up: &str) -> Option<String> {
    if !url.starts_with('/') || url.starts_with("//") {
        return None;
    }
    // The gallery's shots, written beside the pages by `crate::examples::images`.
    if url.starts_with("/examples/") && url.ends_with(".png") {
        return Some(format!("{up}{}", &url[1..]));
    }
    let url = url.replace("&amp;", "&");
    let (without_fragment, fragment) = match url.split_once('#') {
        Some((path, fragment)) => (path, format!("#{fragment}")),
        None => (url.as_str(), String::new()),
    };
    let index = PATHS
        .iter()
        .position(|p| *p == without_fragment)
        .or_else(|| {
            PATHS
                .iter()
                .position(|p| route(p) == route(without_fragment))
        })?;
    Some(format!("{}.html{fragment}", names[index]))
}

/// The language form (`site::header`) as links: each language's copy of this page, the
/// current one marked. A static host cannot keep the `lui-lang` cookie the form sets.
fn languages(html: &str, (current, name): (&str, &str), up: &str) -> String {
    let open = r#"<form method="post" action="/lang" class="lui-lang">"#;
    let Some(start) = html.find(open) else {
        return html.to_string();
    };
    let Some(end) = html[start..]
        .find("</form>")
        .map(|i| start + i + "</form>".len())
    else {
        return html.to_string();
    };
    let ui = Ui::default();
    let links = html! {
        div class="lui-lang" {
            @for (tag, text, dir) in LANGUAGES {
                @let href = format!("{up}{dir}{name}.html");
                @if tag == current { (ui.link_button(text, &href).small().soft().current(true)) }
                @else { (ui.link_button(text, &href).small().ghost()) }
            }
        }
    };
    format!("{}{}{}", &html[..start], links.into_string(), &html[end..])
}

fn banner(other: Option<&str>, baseline: bool) -> Markup {
    let ui = Ui::default();
    let body = html! {
        p { "Forms, cookies and paging need the real server: clone the repository and run "
            code { "cargo run -p demo" } ". Dialogs, popovers, disclosures and tooltips work here." }
        @if let Some(other) = other {
            p { a href={ (other) ".html" } {
                @if baseline { "See this page as a current browser gets it." }
                @else { "See this page as a browser without the newer CSS features gets it." }
            } }
        }
    };
    html! { div style="margin-block-end: var(--lui-space-4)" { (ui.alert(BANNER).warn().body(body)) } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn every_exported_page_has_the_banner_and_no_script() {
        let pages = pages().await;
        assert!(pages.len() >= PATHS.len());
        let names: Vec<&str> = pages.iter().map(|p| p.name.as_str()).collect();
        // The pages; `llms.txt` and the `.md` files are text.
        for page in pages.iter().filter(|p| p.name.ends_with(".html")) {
            assert!(page.html.contains(BANNER), "{}: no banner", page.name);
            assert!(
                !page.html.contains(r#"action="/lang""#),
                "{}: the language form posts to a server",
                page.name
            );
            assert_eq!(page.html.matches("<script").count(), 0, "{}", page.name);
            assert!(
                !page.html.contains(r#"class="lui-caps""#),
                "{}: beacons",
                page.name
            );
            // Every link to a page is to a file that exists.
            for (i, _) in page.html.match_indices("href=\"") {
                let url = &page.html[i + 6..];
                let url = &url[..url.find('"').unwrap()];
                let file = url.split('#').next().unwrap();
                if file.ends_with(".html") {
                    // Relative to the page's own directory (`es/` for the Spanish set).
                    let dir = page.name.rsplit_once('/').map_or("", |(dir, _)| dir);
                    let file = match (file.strip_prefix("../"), dir) {
                        (Some(rest), _) => rest.to_string(),
                        (None, "") => file.to_string(),
                        (None, dir) => format!("{dir}/{file}"),
                    };
                    assert!(
                        names.contains(&file.as_str()),
                        "{}: {url} is not exported",
                        page.name
                    );
                }
            }
        }
    }

    /// The Spanish set sits under `es/`, in Spanish, and each set links the other's copy of
    /// the same page.
    #[tokio::test]
    async fn the_snapshot_switches_language_with_links() {
        let pages = pages().await;
        let page = |name: &str| &pages.iter().find(|p| p.name == name).unwrap().html;
        let (english, spanish) = (page("calendar.html"), page("es/calendar.html"));
        assert!(english.contains(r#"<html lang="en""#) && spanish.contains(r#"<html lang="es""#));
        assert!(english.contains(r#"href="es/calendar.html""#));
        assert!(spanish.contains(r#"href="../calendar.html""#));
        assert!(spanish.contains(r#"href="../es/calendar.html" aria-current="page""#));
        assert!(spanish.contains("Mes anterior") && english.contains("Previous month"));
        // The shots are shared, one directory up from the Spanish set.
        assert!(page("es/examples.html").contains(r#"src="../examples/"#));
        let per_language = pages.iter().filter(|p| p.name.ends_with(".html")).count();
        assert_eq!(per_language % 2, 0, "both sets hold the same pages");
    }
}
