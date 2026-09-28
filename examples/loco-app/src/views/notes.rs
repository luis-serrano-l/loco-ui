//! Note pages: the list (cards or a table, filtered by tag), the reading view, and the form.
//! Started as scaffold output, then written by hand: notes are this app's main page.

use chrono::{DateTime, FixedOffset, Utc};
use loco_ui::prelude::*;

use crate::models::notes::{Listed, Model};

/// "3 hours ago", "yesterday", "12 Sep 2026".
pub fn ago(t: &DateTime<FixedOffset>) -> String {
    let secs = (Utc::now() - t.with_timezone(&Utc)).num_seconds().max(0);
    match secs {
        0..60 => "just now".into(),
        60..3_600 => format!("{} min ago", secs / 60),
        3_600..86_400 => format!("{} h ago", secs / 3_600),
        86_400..172_800 => "yesterday".into(),
        172_800..604_800 => format!("{} days ago", secs / 86_400),
        _ => t.format("%-d %b %Y").to_string(),
    }
}

/// The first lines of a note's body.
fn excerpt(m: &Model) -> String {
    let body = m.body.as_deref().unwrap_or("");
    let mut text: String = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() > 180 {
        text = text.chars().take(180).collect::<String>() + "…";
    }
    text
}

/// A note as a card: title (the card's link), a few lines, tags and when it changed.
pub fn card(ui: &Ui, it: &Listed) -> Markup {
    let m = &it.note;
    let hrefs: Vec<String> = it.tags.iter().map(|t| format!("/notes?sel={t}")).collect();
    let card = ui.card().body(html! {
            div class="notes-card-body" {
                h3 { a href={ "/notes/" (m.id) } { (m.title) } }
                @let text = excerpt(m);
                @if !text.is_empty() { p class="notes-excerpt" { (text) } }
                div class="notes-meta" {
                    @if m.pinned { (ui.badge("Pinned").solid()) }
                    @for (t, href) in it.tags.iter().zip(&hrefs) { (ui.badge(t).secondary().href(href)) }
                    span { (ago(&m.updated_at)) }
                }
            }
    });
    html! { div class="notes-card" { (card) } }
}

/// The list at `path` headed `heading`: a tag filter, cards or a table (`?view=list`).
pub fn list(ui: &Ui, heading: &str, path: &str, items: &[Listed], tags: &[String]) -> Markup {
    let view = ui.param("view").unwrap_or("grid");
    let typed = ui.param("tag").unwrap_or("").to_lowercase();
    let results: Vec<&str> = (tags.iter().map(String::as_str))
        .filter(|t| !typed.is_empty() && t.contains(&typed))
        .collect();
    let filter = ui
        .combobox("tag", path)
        .options(tags.iter().map(String::as_str))
        .results(results)
        .multiple()
        .label("Tags")
        .placeholder("Filter by tag");
    let chosen: Vec<&str> = ui.params("sel").collect();
    let switch = html! {
        form class="notes-view" method="get" action=(path) {
            @for s in &chosen { input type="hidden" name="sel" value=(s); }
            (ui.toggle_group("view", "View").option("grid", "Cards").option("list", "Table").value(view))
            (ui.button("Show").small().submit())
        }
    };
    let body = if items.is_empty() {
        ui.empty_state("No notes here")
            .icon(Icon::File)
            .body(html! { "Write one, or clear the filter." })
            .link("New note", "/notes/new")
            .render()
    } else if view == "list" {
        table(ui, path, items)
    } else {
        ui.grid("17rem")
            .body(html! { @for it in items { (card(ui, it)) } })
            .render()
    };
    html! {
        div class="notes-bar" {
            h1 { (heading) }
            (ui.link_button("New note", "/notes/new").primary())
        }
        div class="notes-filters" { (filter) (switch) }
        (body)
    }
}

/// The table view: title, notebook, tags, when it changed, and a menu per row.
fn table(ui: &Ui, path: &str, items: &[Listed]) -> Markup {
    let links: Vec<[String; 5]> = (items.iter())
        .map(|it| {
            let id = it.note.id;
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
        .map(|(it, [key, show, edit, pin, archive])| {
            let m = &it.note;
            Row::new([
                html! { a href=(show) { (m.title) } },
                html! { @if let Some((_, name)) = &it.notebook { (name) } },
                html! { @for t in &it.tags { (ui.badge(t).secondary()) " " } },
                html! { (ago(&m.updated_at)) },
            ])
            .key(key)
            .menu([
                MenuItem::link("Edit", edit),
                MenuItem::action(if m.pinned { "Unpin" } else { "Pin" }, pin),
                MenuItem::action(if m.archived { "Unarchive" } else { "Archive" }, archive),
            ])
        });
    ui.table("notes", path)
        .column("title", "Title")
        .column("notebook", "Notebook")
        .column("tags", "Tags")
        .column("updated", "Updated")
        .hide_search()
        .rows(rows)
        .render()
}

/// One note to read: the text in a serif column, its details in a card beside it, and edit,
/// delete (behind a confirm dialog), pin and archive.
pub fn show(ui: &Ui, it: &Listed) -> Markup {
    let m = &it.note;
    let id = m.id;
    let [edit, pin, archive, delete] =
        ["edit", "pin", "archive", "delete"].map(|a| format!("/notes/{id}/{a}"));
    let book = it
        .notebook
        .as_ref()
        .map(|(bid, name)| (format!("/notebooks/{bid}"), name.as_str()));
    let crumbs = match &book {
        Some((href, name)) => ui.breadcrumbs().link("Notes", "/notes").link(name, href),
        None => ui.breadcrumbs().link("Notes", "/notes"),
    }
    .here(&m.title);
    let tag_hrefs: Vec<String> = it.tags.iter().map(|t| format!("/notes?sel={t}")).collect();
    let tags = html! { @for (t, href) in it.tags.iter().zip(&tag_hrefs) { (ui.badge(t).secondary().href(href)) " " } };
    let body = m.body.as_deref().unwrap_or("");
    let words = body.split_whitespace().count().to_string();
    let details = ui
        .description_list()
        .item(
            "Notebook",
            html! { @if let Some((href, name)) = &book { a href=(href) { (name) } } },
        )
        .item("Tags", tags.clone())
        .item(
            "Pinned",
            html! { @if m.pinned { (ui.badge("Pinned").solid()) } @else { "No" } },
        )
        .item(
            "Due",
            m.due
                .map(|d| d.format("%-d %b %Y").to_string())
                .unwrap_or_default(),
        )
        .item("Words", words.as_str())
        .item("Created", m.created_at.format("%-d %b %Y").to_string())
        .item("Updated", ago(&m.updated_at));
    let menu = ui
        .menu("More")
        .action(if m.pinned { "Unpin" } else { "Pin to the top" }, &pin)
        .action(
            if m.archived {
                "Move out of the archive"
            } else {
                "Archive"
            },
            &archive,
        )
        .align_end();
    let confirm = ui
        .dialog("Delete")
        .id("delete-note")
        .title("Delete this note?")
        .danger()
        .confirm("Delete", &delete)
        .body(html! { p { "“" (m.title) "” and its tags go for good." } });
    let article = html! {
        article class="notes-article" {
            h1 { (m.title) }
            div class="notes-meta" {
                span { "Edited " (ago(&m.updated_at)) }
                @if m.archived { (ui.badge("Archived").warn()) }
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
    let side = html! {
        aside class="notes-side" { (ui.card().title("Details").body(details.render())) }
    };
    html! {
        div class="notes-bar" {
            (crumbs)
            div class="notes-actions" {
                (ui.link_button("Edit", &edit))
                (confirm)
                (menu)
            }
        }
        (ui.split(side, article).side_end().side_width("17rem").gap(8))
    }
}

/// The note as the form's `(name, value)` pairs, for the edit page.
pub fn values(it: &Listed) -> Vec<(String, String)> {
    let m = &it.note;
    vec![
        ("title".into(), m.title.clone()),
        ("body".into(), m.body.clone().unwrap_or_default()),
        (
            "notebook_id".into(),
            m.notebook_id.map(|id| id.to_string()).unwrap_or_default(),
        ),
        ("tags".into(), it.tags.join(", ")),
        ("pinned".into(), m.pinned.to_string()),
        (
            "due".into(),
            m.due.map(|d| d.to_string()).unwrap_or_default(),
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
    let form = ui
        .form(action)
        .text("title", "Title")
        .required()
        .textarea("body", "Body", 14)
        .help("A blank line starts a new paragraph.")
        .select("notebook_id", "Notebook", choices)
        .text("tags", "Tags")
        .help("Comma separated, for example: rust, ideas")
        .date("due", "Due", "1900-01-01", "2100-12-31")
        .switch("pinned", "Pinned to the top")
        .values(values)
        .errors(errors)
        .submit("Save");
    html! {
        div class="notes-bar" { h1 { (title) } }
        div style="max-width: 44rem" { (form) }
    }
}
