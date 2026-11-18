use loco_app::models::{
    _entities::{note_tags, notes as rows, users as user_rows},
    notes::{self, Notes, tag_names},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter,
};
use serial_test::serial;

use super::{database, note, notebook, user};

/// The tags field is split on commas; each name is trimmed, lowercased and kept once.
#[test]
fn tag_names_are_trimmed_lowercased_and_kept_once() {
    assert_eq!(tag_names(" Rust, ideas ,rust,, "), ["rust", "ideas"]);
    assert_eq!(tag_names(""), Vec::<String>::new());
    assert_eq!(tag_names("c#"), ["c#"]);
}

/// The listing is one user's notes, pinned ones first, each with its tags in name order and
/// its notebook; the filter narrows it.
#[tokio::test]
#[serial]
async fn listed_is_one_users_notes_with_tags_and_notebook() {
    let db = database().await;
    let (ada, grace) = (user(&db, "Ada").await, user(&db, "Grace").await);
    let ideas = notebook(&db, ada, "Ideas").await;
    let first = note(&db, ada, "First", Some(ideas.id)).await;
    let second = note(&db, ada, "Second", None).await;
    note(&db, grace, "Hers", None).await;
    Notes::set_tags(&db, ada, first.id, "web, rust")
        .await
        .unwrap();
    let mut pin = second.into_active_model();
    pin.pinned = Set(true);
    pin.update(&db).await.unwrap();

    let listed = Notes::listed(&db, ada, |all| all).await.unwrap();
    let titles: Vec<&str> = listed.iter().map(|l| l.note.title.as_str()).collect();
    assert_eq!(titles, ["Second", "First"], "pinned first, and not Grace's");
    assert_eq!(listed[1].tags, ["rust", "web"]);
    assert_eq!(listed[1].notebook, Some((ideas.id, "Ideas".to_string())));
    assert_eq!(listed[0].tags, Vec::<String>::new());
    assert_eq!(listed[0].notebook, None);

    let pinned = Notes::listed(&db, ada, |all| all.filter(notes::Column::Pinned.eq(true)))
        .await
        .unwrap();
    assert_eq!(pinned.len(), 1);
    assert_eq!(pinned[0].note.title, "Second");
}

/// Setting a note's tags replaces what it had, and a name the user already has is the same
/// tag, not a second one.
#[tokio::test]
#[serial]
async fn set_tags_replaces_the_tags_and_reuses_the_users_tag() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    let (first, second) = (
        note(&db, ada, "First", None).await,
        note(&db, ada, "Second", None).await,
    );
    Notes::set_tags(&db, ada, first.id, "rust, web")
        .await
        .unwrap();
    Notes::set_tags(&db, ada, second.id, "Rust").await.unwrap();
    Notes::set_tags(&db, ada, first.id, "rust").await.unwrap();

    let listed = Notes::listed(&db, ada, |all| all).await.unwrap();
    for l in &listed {
        assert_eq!(l.tags, ["rust"], "{}", l.note.title);
    }
    let pairs = note_tags::Entity::find().all(&db).await.unwrap();
    assert_eq!(pairs.len(), 2, "one pair a note");
    assert_eq!(pairs[0].tag_id, pairs[1].tag_id, "both notes share the tag");
}

/// Saving a change moves `updated_at`, which the listing sorts by.
#[tokio::test]
#[serial]
async fn saving_a_change_moves_updated_at() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    let before = note(&db, ada, "First", None).await;
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let mut change = before.clone().into_active_model();
    change.title = Set("Renamed".to_string());
    let after = change.update(&db).await.unwrap();
    assert_eq!(after.title, "Renamed");
    assert!(after.updated_at > before.updated_at);
    assert_eq!(after.created_at, before.created_at);
}

/// A note needs its owner: the foreign key refuses a user that does not exist, and a user's
/// notes go with the user.
#[tokio::test]
#[serial]
async fn a_note_belongs_to_a_user() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    note(&db, ada, "First", None).await;
    let orphan = rows::ActiveModel {
        title: Set("Nobody's".to_string()),
        user_id: Set(ada + 100),
        pinned: Set(false),
        archived: Set(false),
        ..Default::default()
    };
    assert!(orphan.insert(&db).await.is_err());

    user_rows::Entity::delete_many().exec(&db).await.unwrap();
    assert_eq!(rows::Entity::find().count(&db).await.unwrap(), 0);
}
