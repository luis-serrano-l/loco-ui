//! Note pages: the list (cards or a table, filtered by tag), the reading view, and the form.
//! Started as scaffold output, then written by hand: notes are this app's main page.

use chrono::{DateTime, FixedOffset, Utc};
use loco_ui::prelude::*;

use crate::models::notes::{Listed, Model};

/// "3 hours ago", "yesterday", "12 Sep 2026".
pub fn ago(time: &DateTime<FixedOffset>) -> String {
    let secs = (Utc::now() - time.with_timezone(&Utc)).num_seconds().max(0);
    match secs {
        0..60 => "just now".into(),
        60..3_600 => format!("{} min ago", secs / 60),
        3_600..86_400 => format!("{} h ago", secs / 3_600),
        86_400..172_800 => "yesterday".into(),
        172_800..604_800 => format!("{} days ago", secs / 86_400),
        _ => time.format("%-d %b %Y").to_string(),
    }
}

/// The first lines of a note's body.
fn excerpt(note: &Model) -> String {
    let body = note.body.as_deref().unwrap_or("");
    let mut text: String = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() > 180 {
        text = text.chars().take(180).collect::<String>() + "…";
    }
    text
}

/// The list of the notes tagged `tag`, the tag encoded (`c#`, `r&d`).
fn tag_href(tag: &str) -> String {
    loco_ui::href("/notes", [("sel", tag)])
}

/// A note as a card: title (the card's link), a few lines, tags and when it changed.
pub fn card(ui: &Ui, listed: &Listed) -> Markup {
    let note = &listed.note;
    lui! {
        div class="notes-card" {
            Card {
                div class="notes-card-body" {
                    h3 { a href={ "/notes/" (note.id) } { (note.title) } }
                    @let text = excerpt(note);
                    @if !text.is_empty() { p class="notes-excerpt" { (text) } }
                    div class="notes-meta" {
                        @if note.pinned { Badge("Pinned") solid; }
                        @for tag in &listed.tags { @let href = tag_href(tag); Badge(tag) secondary href=(&href); }
                        span { (ago(&note.updated_at)) }
                    }
                }
            }
        }
    }
}

/// The list at `path` headed `heading`: a tag filter, cards or a table (`?view=list`).
pub fn list(ui: &Ui, heading: &str, path: &str, items: &[Listed], tags: &[String]) -> Markup {
    let view = ui.param("view").unwrap_or("grid");
    let typed = ui.param("tag").unwrap_or("").to_lowercase();
    let results: Vec<&str> = (tags.iter().map(String::as_str))
        .filter(|tag| !typed.is_empty() && tag.contains(&typed))
        .collect();
    let chosen: Vec<&str> = ui.params("sel").collect();
    lui! {
        div class="notes-bar" {
            h1 { (heading) }
            LinkButton("New note", "/notes/new") primary;
        }
        div class="notes-filters" {
            Combobox("tag", path) options=(tags.iter().map(String::as_str)) results=(results)
                multiple label="Tags" placeholder="Filter by tag" keep="q" keep="view";
            form class="notes-view" method="get" action=(path) {
                // The switch keeps the search and the chosen tags.
                @if let Some(query) = ui.param("q") { input type="hidden" name="q" value=(query); }
                @for tag in &chosen { input type="hidden" name="sel" value=(tag); }
                ToggleGroup("view", "View") value=(view) {
                    option "grid" "Cards";
                    option "list" "Table";
                }
                Button("Show") small submit;
            }
        }
        @if items.is_empty() {
            EmptyState("No notes here") icon=(Icon::File) {
                body { "Write one, or clear the filter." }
                link "New note" "/notes/new";
            }
        } @else if view == "list" {
            (table(ui, path, items))
        } @else {
            Grid("17rem") { @for listed in items { (card(ui, listed)) } }
        }
    }
}

/// The table view: title, notebook, tags, when it changed, and a menu per row.
fn table(ui: &Ui, path: &str, items: &[Listed]) -> Markup {
    let links: Vec<[String; 5]> = (items.iter())
        .map(|listed| {
            let id = listed.note.id;
            [
                id.to_string(),
                format!("/notes/{id}"),
                format!("/notes/{id}/edit"),
                format!("/notes/{id}/pin"),
                format!("/notes/{id}/archive"),
            ]
        })
        .collect();
    let rows = items
        .iter()
        .zip(&links)
        .map(|(listed, [key, show, edit, pin, archive])| {
            let note = &listed.note;
            Row::new([
                html! { a href=(show) { (note.title) } },
                html! { @if let Some((_, name)) = &listed.notebook { (name) } },
                lui! { @for tag in &listed.tags { Badge(tag) secondary; " " } },
                html! { (ago(&note.updated_at)) },
            ])
            .key(key)
            .menu([
                MenuItem::link("Edit", edit),
                MenuItem::action(if note.pinned { "Unpin" } else { "Pin" }, pin),
                MenuItem::action(
                    if note.archived {
                        "Unarchive"
                    } else {
                        "Archive"
                    },
                    archive,
                ),
            ])
        });
    lui! {
        Table("notes", path) hide_search rows=(rows) {
            column "title" "Title";
            column "notebook" "Notebook";
            column "tags" "Tags";
            column "updated" "Updated";
        }
    }
}

/// One note to read: the text in a reading column, its details in a card beside it, and edit,
/// delete (behind a confirm dialog), pin and archive.
pub fn show(ui: &Ui, listed: &Listed) -> Markup {
    let note = &listed.note;
    let id = note.id;
    let [edit, pin, archive, delete] =
        ["edit", "pin", "archive", "delete"].map(|a| format!("/notes/{id}/{a}"));
    let book = listed
        .notebook
        .as_ref()
        .map(|(bid, name)| (format!("/notebooks/{bid}"), name.as_str()));
    let tags = lui! { @for tag in &listed.tags { @let href = tag_href(tag); Badge(tag) secondary href=(&href); " " } };
    let body = note.body.as_deref().unwrap_or("");
    let words = body.split_whitespace().count().to_string();
    let article = lui! {
        article class="notes-article" {
            h1 { (note.title) }
            div class="notes-meta" {
                span { "Edited " (ago(&note.updated_at)) }
                @if note.archived { Badge("Archived") warn; }
                (tags)
            }
            div class="notes-prose" {
                @for para in body.split("\n\n").map(str::trim).filter(|p| !p.is_empty()) {
                    p { @for (i, line) in para.lines().enumerate() { @if i > 0 { br; } (line) } }
                }
                @if body.trim().is_empty() { p class="notes-excerpt" { "This note is empty." } }
            }
        }
    };
    let due = note
        .due
        .map(|date| date.format("%-d %b %Y").to_string())
        .unwrap_or_default();
    let side = lui! {
        aside class="notes-side" {
            Card title="Details" {
                DescriptionList {
                    item "Notebook" { @if let Some((href, name)) = &book { a href=(href) { (name) } } }
                    item "Tags" (&tags);
                    item "Pinned" { @if note.pinned { Badge("Pinned") solid; } @else { "No" } }
                    item "Due" (due);
                    item "Words" (words.as_str());
                    item "Created" (note.created_at.format("%-d %b %Y").to_string());
                    item "Updated" (ago(&note.updated_at));
                }
            }
        }
    };
    lui! {
        div class="notes-bar" {
            Breadcrumbs here=(&note.title) {
                link "Notes" "/notes";
                @if let Some((href, name)) = &book { link (name) (href); }
            }
            div class="notes-actions" {
                LinkButton("Edit", &edit);
                Dialog("Delete") id="delete-note" title="Delete this note?" danger confirm=("Delete", &delete) {
                    p { "“" (note.title) "” and its tags go for good." }
                }
                Menu("More") align_end {
                    action (if note.pinned { "Unpin" } else { "Pin to the top" }) (&pin);
                    action (if note.archived { "Move out of the archive" } else { "Archive" }) (&archive);
                }
            }
        }
        Split(side, article) side_end side_width="17rem" gap=8;
    }
}

/// The note as the form's `(name, value)` pairs, for the edit page.
pub fn values(listed: &Listed) -> Vec<(String, String)> {
    let note = &listed.note;
    vec![
        ("title".into(), note.title.clone()),
        ("body".into(), note.body.clone().unwrap_or_default()),
        (
            "notebook_id".into(),
            note.notebook_id
                .map(|id| id.to_string())
                .unwrap_or_default(),
        ),
        ("tags".into(), listed.tags.join(", ")),
        ("pinned".into(), note.pinned.to_string()),
        (
            "due".into(),
            note.due.map(|date| date.to_string()).unwrap_or_default(),
        ),
    ]
}

/// The form for new (`action` = the list) and edit (`action` = the note), with what was
/// posted and the messages when it comes back. `notebooks` are `(id, name)`.
pub fn form(
    ui: &Ui,
    title: &str,
    action: &str,
    values: &[(String, String)],
    errors: &[(&str, &str)],
    notebooks: &[(i64, String)],
) -> Markup {
    let ids: Vec<String> = notebooks.iter().map(|(id, _)| id.to_string()).collect();
    let choices = std::iter::once(("", "No notebook")).chain(
        ids.iter()
            .zip(notebooks)
            .map(|(id, (_, name))| (id.as_str(), name.as_str())),
    );
    lui! {
        div class="notes-bar" { h1 { (title) } }
        div style="max-width: 44rem" {
            Form(action) values=(values) errors=(errors) submit="Save" {
                text "title" "Title" required;
                textarea "body" "Body" 14 help="A blank line starts a new paragraph.";
                select "notebook_id" "Notebook" (choices);
                text "tags" "Tags" help="Comma separated, for example: rust, ideas";
                date "due" "Due" "1900-01-01" "2100-12-31";
                switch "pinned" "Pinned to the top";
            }
        }
    }
}
