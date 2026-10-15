//! The examples gallery at `/examples`: pages of `examples/loco-app`, the Loco notes app built
//! on loco-ui, each as the screenshot its own test takes (`examples/loco-app/tests/pages.rs`
//! lays every page out with Blitz into `tests/shots/loco-*.png`) beside a link to the file
//! that renders it. The shots are served from the binary at `/examples/<name>.png`, so the
//! gallery always shows what the last test run drew. Blitz has no container queries, so they
//! show each page's narrow layout at 1000px.

use crate::site::page;
use axum::{Router, extract::Path, http::header, response::IntoResponse, routing::get};
use loco_ui::prelude::*;

/// Where the source links go.
const REPO: &str = "https://github.com/luis-serrano-l/loco-ui/blob/main/examples/loco-app/";

/// One page of the app in the gallery.
pub(crate) struct Example {
    /// The shot's name: `tests/shots/loco-<name>.png`, served at `/examples/<name>.png`.
    pub name: &'static str,
    /// What the page is.
    pub title: &'static str,
    /// What it shows of loco-ui.
    pub note: &'static str,
    /// The file that renders it, from the app's root.
    pub source: &'static str,
    /// The shot.
    pub png: &'static [u8],
}

/// The gallery, in the order a visitor meets the app.
pub(crate) const EXAMPLES: &[Example] = &[
    Example {
        name: "landing",
        title: "Landing page",
        note: "The signed-out front page: what the app is, and the way in.",
        source: "src/views/home.rs",
        png: include_bytes!("../../tests/shots/loco-landing.png"),
    },
    Example {
        name: "signin",
        title: "Sign in",
        note: "The auth block, as cargo lui auth writes it: a centred card with the form.",
        source: "src/views/account.rs",
        png: include_bytes!("../../tests/shots/loco-signin.png"),
    },
    Example {
        name: "index",
        title: "Overview",
        note: "The dashboard block: counts, notes per week, the latest notes and the open tasks.",
        source: "src/views/home.rs",
        png: include_bytes!("../../tests/shots/loco-index.png"),
    },
    Example {
        name: "notes",
        title: "Notes",
        note: "The app shell's sidebar and top bar, a tag filter, and the notes as cards.",
        source: "src/views/notes.rs",
        png: include_bytes!("../../tests/shots/loco-notes.png"),
    },
    Example {
        name: "notes-table",
        title: "Notes as a table",
        note: "The same list as a table: title, notebook, tags, when it changed, a menu per row.",
        source: "src/views/notes.rs",
        png: include_bytes!("../../tests/shots/loco-notes-table.png"),
    },
    Example {
        name: "notes-show",
        title: "A note",
        note: "The text in a reading column, its details in a card, and delete behind a confirm dialog.",
        source: "src/views/notes.rs",
        png: include_bytes!("../../tests/shots/loco-notes-show.png"),
    },
    Example {
        name: "notes-edit",
        title: "Editing a note",
        note: "The note form, refilled: a notebook select and a pinned switch.",
        source: "src/views/notes.rs",
        png: include_bytes!("../../tests/shots/loco-notes-edit.png"),
    },
    Example {
        name: "notebooks-show",
        title: "A notebook",
        note: "The record block from the scaffold templates: the fields in a card, then its notes.",
        source: "src/views/notebooks.rs",
        png: include_bytes!("../../tests/shots/loco-notebooks-show.png"),
    },
    Example {
        name: "tasks",
        title: "Task board",
        note: "Tabs of a Kanban board, whose cards move by form posts, and a table.",
        source: "src/views/tasks.rs",
        png: include_bytes!("../../tests/shots/loco-tasks.png"),
    },
    Example {
        name: "link-expired",
        title: "Expired link",
        note: "The auth block for a spent link, pointing to the page that sends a new one.",
        source: "src/views/account.rs",
        png: include_bytes!("../../tests/shots/loco-link-expired.png"),
    },
];

/// The gallery page and one route for the shots.
/// The shots as the static snapshot writes them: `examples/<name>.png` and the bytes.
pub fn images() -> impl Iterator<Item = (String, &'static [u8])> {
    EXAMPLES
        .iter()
        .map(|e| (format!("examples/{}.png", e.name), e.png))
}

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/examples", get(|ui: Ui| async move { gallery(&ui) }))
        .route("/examples/{file}", get(shot))
}

fn gallery(ui: &Ui) -> Page {
    let body = html! {
        p class="lui-note" {
            "Pages of " a href={ (REPO) "README.md" } { code { "examples/loco-app" } }
            ", a Loco notes app with accounts, notebooks, tagged notes and a task board. Each picture is the screenshot its test takes without script, at the narrow layout."
        }
        (ui.grid("18rem").body(html! {
            @for e in EXAMPLES {
                (ui.card().title(e.title).description(e.note).body(html! {
                    a href={ "/examples/" (e.name) ".png" } {
                        img src={ "/examples/" (e.name) ".png" } alt={ "Screenshot: " (e.title) } width="1000" height="700" loading="lazy" class="lui-example-shot";
                    }
                }).footer(html! {
                    a href={ (REPO) (e.source) } { code { (e.source) } }
                }))
            }
        }))
    };
    page(ui, "Examples", body)
}

async fn shot(Path(file): Path<String>) -> axum::response::Response {
    let found = file
        .strip_suffix(".png")
        .and_then(|name| EXAMPLES.iter().find(|e| e.name == name));
    match found {
        Some(e) => ([(header::CONTENT_TYPE, "image/png")], e.png).into_response(),
        None => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_example_links_a_file_that_exists_and_a_png() {
        let app = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../examples/loco-app");
        for e in EXAMPLES {
            assert!(app.join(e.source).is_file(), "{}: no {}", e.name, e.source);
            assert!(e.png.starts_with(b"\x89PNG"), "{}", e.name);
        }
    }
}
