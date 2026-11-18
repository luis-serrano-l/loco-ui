use loco_app::models::_entities::{notebooks as rows, notes as note_rows};
use sea_orm::{EntityTrait, ModelTrait, PaginatorTrait};
use serial_test::serial;

use super::{database, note, notebook, user};

/// Deleting a notebook keeps its notes: they are in no notebook afterwards.
#[tokio::test]
#[serial]
async fn deleting_a_notebook_keeps_its_notes() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    let ideas = notebook(&db, ada, "Ideas").await;
    let first = note(&db, ada, "First", Some(ideas.id)).await;
    assert_eq!(first.notebook_id, Some(ideas.id));

    ideas.delete(&db).await.unwrap();
    let first = note_rows::Entity::find_by_id(first.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the note is still there");
    assert_eq!(first.notebook_id, None);
}

/// A notebook's notes are found through the relation, and only its own.
#[tokio::test]
#[serial]
async fn a_notebook_finds_its_notes() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    let (ideas, work) = (
        notebook(&db, ada, "Ideas").await,
        notebook(&db, ada, "Work").await,
    );
    note(&db, ada, "First", Some(ideas.id)).await;
    note(&db, ada, "Second", Some(ideas.id)).await;
    note(&db, ada, "Loose", None).await;

    let in_ideas = ideas
        .find_related(note_rows::Entity)
        .all(&db)
        .await
        .unwrap();
    let mut titles: Vec<&str> = in_ideas.iter().map(|n| n.title.as_str()).collect();
    titles.sort_unstable();
    assert_eq!(titles, ["First", "Second"]);
    assert_eq!(
        work.find_related(note_rows::Entity)
            .count(&db)
            .await
            .unwrap(),
        0
    );
    assert_eq!(rows::Entity::find().count(&db).await.unwrap(), 2);
}
