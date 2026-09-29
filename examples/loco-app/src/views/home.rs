//! The front page: what the app is for a visitor, the overview for a signed-in user.
use loco_ui::prelude::*;

use crate::{
    models::{_entities::tasks, notes::Listed},
    views::{notes::card, shell::Nav},
};

/// Signed out: what the app is, and the way in.
pub fn landing(ui: &Ui) -> Markup {
    lui! {
        Flash;
        div class="notes-landing" {
            Badge("Loco + loco-ui") surface;
            h1 { "Notes that live on your server." }
            p { "Notebooks, tags, pinned notes and a task board. Every page is plain HTML and works with JavaScript switched off." }
            Cluster {
                LinkButton("Sign in", "/signin") primary;
                LinkButton("Create an account", "/signup");
            }
        }
        div class="notes-features" {
            Grid("15rem") {
                Card title="Write" description="A title, a body, a notebook and a few tags. Pin what matters.";
                Card title="Find" description="Search every note, or narrow them by tag in one click.";
                Card title="Plan" description="Tasks on a board: move a card from To do to Done.";
            }
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
    let recent: Vec<&Listed> = notes
        .iter()
        .filter(|it| !it.note.archived)
        .take(6)
        .collect();
    let due = &due[..due.len().min(5)];
    let hrefs: Vec<String> = due.iter().map(|t| format!("/tasks/{}", t.id)).collect();
    let dates: Vec<String> = (due.iter())
        .map(|t| {
            t.due_on
                .map_or("Someday".into(), |d| d.format("%-d %b").to_string())
        })
        .collect();
    let first = nav.user.split_whitespace().next().unwrap_or("");
    let hello = format!("Good to see you, {first}.");
    let due_next = lui! {
        Card title="Due next" {
            @if due.is_empty() {
                "Nothing due."
            } @else {
                DescriptionList {
                    @for ((t, href), date) in due.iter().zip(&hrefs).zip(&dates) {
                        item (date) { a href=(href) { (t.title) } " " Badge(&t.status) secondary; }
                    }
                }
            }
        }
    };
    let chart = lui! {
        Chart("Notes per week") description="New notes in each of the last eight weeks"
            points=(weeks.iter().map(|(w, n)| (w.as_str(), *n)));
    };
    lui! {
        DashboardPage("Overview") description=(&hello) {
            stat (ui.stat("Notes", &total).href("/notes"));
            stat (ui.stat("Pinned", &pinned).href("/notes/pinned"));
            stat (ui.stat("Open tasks", &open).href("/tasks"));
            stat (ui.stat("Notebooks", &books).href("/notebooks"));
            body {
                div class="notes-section" {
                    Split(due_next, chart) side_end side_width="20rem" gap=6;
                }
                div class="notes-section" {
                    h2 { "Recent notes" }
                    Grid("17rem") { @for it in recent { (card(ui, it)) } }
                }
            }
        }
    }
}
