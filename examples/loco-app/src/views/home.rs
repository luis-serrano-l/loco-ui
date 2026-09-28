//! The front page: what the app is for a visitor, the overview for a signed-in user.
use loco_ui::prelude::*;

use crate::{
    models::{_entities::tasks, notes::Listed},
    views::{notes::card, shell::Nav},
};

/// Signed out: what the app is, and the way in.
pub fn landing(ui: &Ui) -> Markup {
    let feature =
        |title: &'static str, text: &'static str| ui.card().title(title).description(text);
    html! {
        (ui.flash())
        div class="notes-landing" {
            (ui.badge("Loco + loco-ui").surface())
            h1 { "Notes that live on your server." }
            p { "Notebooks, tags, pinned notes and a task board. Every page is plain HTML and works with JavaScript switched off." }
            (ui.cluster().body(html! {
                (ui.link_button("Sign in", "/signin").primary())
                (ui.link_button("Create an account", "/signup"))
            }))
        }
        div class="notes-features" {
            (ui.grid("15rem").body(html! {
                (feature("Write", "A title, a body, a notebook and a few tags. Pin what matters."))
                (feature("Find", "Search every note, or narrow them by tag in one click."))
                (feature("Plan", "Tasks on a board: move a card from To do to Done."))
            }))
        }
    }
}

/// Signed in: counts, notes per week, the latest notes and the open tasks.
pub fn overview(
    ui: &Ui,
    nav: &Nav,
    notes: &[Listed],
    weeks: &[(String, f64)],
    due: &[tasks::Model],
) -> Markup {
    let [total, pinned, open, books] = [
        nav.notes,
        nav.pinned,
        nav.open_tasks,
        nav.notebooks.len() as u64,
    ]
    .map(|n| n.to_string());
    let chart = ui
        .chart("Notes per week")
        .description("New notes in each of the last eight weeks")
        .points(weeks.iter().map(|(w, n)| (w.as_str(), *n)));
    let recent: Vec<&Listed> = notes
        .iter()
        .filter(|it| !it.note.archived)
        .take(6)
        .collect();
    let hrefs: Vec<String> = due.iter().map(|t| format!("/tasks/{}", t.id)).collect();
    let dates: Vec<String> = (due.iter())
        .map(|t| {
            t.due_on
                .map_or("Someday".into(), |d| d.format("%-d %b").to_string())
        })
        .collect();
    let tasks = ui.description_list();
    let tasks = (due.iter().zip(&hrefs).zip(&dates).take(5)).fold(tasks, |l, ((t, href), date)| {
        l.item(
            date,
            html! { a href=(href) { (t.title) } " " (ui.badge(&t.status).secondary()) },
        )
    });
    let first = nav.user.split_whitespace().next().unwrap_or("");
    let hello = format!("Good to see you, {first}.");
    ui.dashboard_page("Overview")
        .description(&hello)
        .stat(ui.stat("Notes", &total).href("/notes"))
        .stat(ui.stat("Pinned", &pinned).href("/notes/pinned"))
        .stat(ui.stat("Open tasks", &open).href("/tasks"))
        .stat(ui.stat("Notebooks", &books).href("/notebooks"))
        .body(html! {
            div class="notes-section" {
                (ui.split(
                    ui.card().title("Due next").body(if due.is_empty() { html! { "Nothing due." } } else { tasks.render() }).render(),
                    chart.render(),
                ).side_end().side_width("20rem").gap(6))
            }
            div class="notes-section" {
                h2 { "Recent notes" }
                (ui.grid("17rem").body(html! { @for it in recent { (card(ui, it)) } }))
            }
        })
        .render()
}
