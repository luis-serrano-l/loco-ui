//! The frame of every signed-in page: the app shell's sidebar (notes, pinned, archive, each
//! notebook with its count, tasks), a top bar with the search and the theme, then the page.

use loco_ui::prelude::*;

/// What the sidebar lists for the signed-in user, loaded by `controllers::session::nav`.
#[derive(Debug, Default)]
pub struct Nav {
    pub user: String,
    pub notes: u64,
    pub pinned: u64,
    pub archived: u64,
    pub open_tasks: u64,
    /// `(id, name, notes in it)`, by name.
    pub notebooks: Vec<(i64, String, u64)>,
}

/// `body` in the app shell, as a whole page titled `title`.
pub fn page(ui: &Ui, nav: &Nav, title: &str, body: Markup) -> Page {
    let hrefs: Vec<String> = (nav.notebooks.iter())
        .map(|(id, _, _)| format!("/notebooks/{id}"))
        .collect();
    let shell = ui
        .app_shell("Notes")
        .link("Overview", "/")
        .icon(Icon::House)
        .link("All notes", "/notes")
        .icon(Icon::File)
        .badge(nav.notes)
        .link("Pinned", "/notes/pinned")
        .icon("📌")
        .badge(nav.pinned)
        .link("Archive", "/notes/archive")
        .icon(Icon::Download)
        .badge(nav.archived)
        .link("Tasks", "/tasks")
        .icon(Icon::CircleCheck)
        .badge(nav.open_tasks)
        .group("Notebooks");
    let shell = (nav.notebooks.iter().zip(&hrefs))
        .fold(shell, |s, ((_, name, n), href)| s.link(name, href).badge(n))
        .link("All notebooks", "/notebooks")
        .icon(Icon::Menu)
        .user(&nav.user, "/signout");
    let q = ui.param("q").unwrap_or_default();
    let body = html! {
        div class="notes-top" {
            form class="notes-search" method="get" action="/notes" role="search" {
                (ui.input("q", "Search notes").hide_label().placeholder("Search notes…").value(q).leading(Icon::Search))
            }
            (ui.theme_toggle("/theme"))
        }
        (ui.flash())
        (body)
    };
    ui.page(title, shell.body(body).render())
}
