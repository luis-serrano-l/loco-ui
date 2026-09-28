//! The demo account and its notes, notebooks, tags and tasks, so a fresh app has something to
//! show. Written in Rust rather than as YAML fixtures because the dates are relative to today:
//! the overview's "notes per week" chart always has eight weeks of notes.
//!
//! `App::seed` loads it (`cargo loco db seed`), and `App::before_run` does too the first time
//! the app starts in development with no users.

use chrono::{Duration, Utc};
use loco_rs::prelude::*;
use sea_orm::ActiveValue::Set;

use crate::models::{
    _entities::{notebooks, tasks},
    notes, users,
};

/// Sign in with these.
pub const EMAIL: &str = "ada@example.com";
pub const PASSWORD: &str = "analytical-engine";

/// `(notebook, title, body, tags, days ago, pinned, archived)`
const NOTES: &[(&str, &str, &str, &str, i64, bool, bool)] = &[
    (
        "Ideas",
        "A notes app with no JavaScript",
        "Every page is a form or a link. The server keeps the state: the theme in a cookie, the filter in the query.\n\nThe script is optional: it swaps parts of the page in place, and nothing breaks without it.",
        "rust, ideas",
        1,
        true,
        false,
    ),
    (
        "Reading",
        "Notes on the Analytical Engine",
        "The engine weaves algebraic patterns just as the Jacquard loom weaves flowers and leaves.\n\nIt might act upon other things besides number, were objects found whose mutual relations could be expressed by those of the abstract science of operations.",
        "history, reading",
        3,
        true,
        false,
    ),
    (
        "Work",
        "Release checklist",
        "Run the whole test suite, then clippy.\nUpdate the changelog.\nTag the release and write the notes.",
        "work",
        4,
        false,
        false,
    ),
    (
        "Ideas",
        "Server-held UI state",
        "Tabs, open panels and table sorting live in the URL or a cookie. A link can share exactly what you see.",
        "ideas, web",
        6,
        false,
        false,
    ),
    (
        "Reading",
        "How to take smart notes",
        "Write in your own words. Link each note to the ones it answers or questions.\n\nA note that is never read again was not worth writing.",
        "reading",
        9,
        false,
        false,
    ),
    (
        "Work",
        "Weekly review",
        "What moved, what is stuck, what to drop. Keep it to one page.",
        "work, habits",
        12,
        false,
        false,
    ),
    (
        "Ideas",
        "Container queries everywhere",
        "Components adapt to the box they are in, not the window. A card in a sidebar and the same card in the main column each pick their own layout.",
        "web, css",
        16,
        false,
        false,
    ),
    (
        "Reading",
        "The Mythical Man-Month",
        "Adding people to a late project makes it later. Plan to throw one away; you will anyhow.",
        "reading, work",
        20,
        false,
        false,
    ),
    (
        "Work",
        "Database migrations",
        "SQLite cannot add a NOT NULL reference to a table with rows. Recreate the table, or add the column nullable and backfill.",
        "rust, work",
        25,
        false,
        false,
    ),
    (
        "Ideas",
        "Keyboard shortcuts without script",
        "accesskey gives a key to a link or button. The browser decides the modifier.",
        "web, ideas",
        31,
        false,
        false,
    ),
    (
        "Reading",
        "Maud templates",
        "html! checks the markup at compile time and escapes every splice.",
        "rust, reading",
        38,
        false,
        false,
    ),
    (
        "",
        "Groceries",
        "Oats, coffee, lemons, olive oil.",
        "",
        44,
        false,
        false,
    ),
    (
        "Work",
        "Old meeting notes",
        "Decided to move the launch to October.",
        "work",
        50,
        false,
        true,
    ),
    (
        "Ideas",
        "Abandoned: a mobile app",
        "The web app works on a phone; a second codebase is not worth it.",
        "ideas",
        55,
        false,
        true,
    ),
];

/// `(title, status, due in days, size)`
const TASKS: &[(&str, &str, Option<i64>, Option<i64>)] = &[
    ("Write the release notes", "doing", Some(1), Some(2)),
    ("Review the notes page design", "todo", Some(2), Some(3)),
    ("Fix the migration on SQLite", "done", None, Some(1)),
    ("Read chapter 4", "todo", Some(5), Some(1)),
    ("Plan next quarter", "todo", Some(9), Some(5)),
    ("Clean up old notebooks", "doing", None, Some(2)),
    ("Ship the tag filter", "done", None, Some(3)),
    ("Answer Charles", "todo", Some(0), None),
];

/// The demo account and everything in it; does nothing when the account exists.
pub async fn demo(db: &DatabaseConnection) -> Result<()> {
    if users::Model::find_by_email(db, EMAIL).await.is_ok() {
        return Ok(());
    }
    let params = users::RegisterParams {
        email: EMAIL.into(),
        password: PASSWORD.into(),
        name: "Ada Lovelace".into(),
    };
    let me = users::Model::create_with_password(db, &params).await?;
    let me = me.into_active_model().verified(db).await?;
    let now = Utc::now();
    let mut books = Vec::new();
    for name in ["Ideas", "Reading", "Work"] {
        let book = notebooks::ActiveModel {
            name: Set(name.into()),
            user_id: Set(me.id),
            ..Default::default()
        }
        .insert(db)
        .await?;
        books.push((name, book.id));
    }
    for &(book, title, body, tags, days, pinned, archived) in NOTES {
        let at = (now - Duration::days(days) - Duration::hours(days % 7 * 3)).into();
        let note = notes::ActiveModel {
            title: Set(title.into()),
            body: Set(Some(body.into())),
            pinned: Set(pinned),
            archived: Set(archived),
            user_id: Set(me.id),
            notebook_id: Set(books.iter().find(|(n, _)| *n == book).map(|(_, id)| *id)),
            created_at: Set(at),
            updated_at: Set(at),
            ..Default::default()
        }
        .insert(db)
        .await?;
        notes::Entity::set_tags(db, me.id, note.id, tags).await?;
    }
    for &(title, status, due, size) in TASKS {
        tasks::ActiveModel {
            title: Set(title.into()),
            status: Set(status.into()),
            done: Set(status == "done"),
            due_on: Set(due.map(|d| (now + Duration::days(d)).date_naive())),
            size: Set(size),
            user_id: Set(me.id),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }
    Ok(())
}
