use loco_app::models::{
    _entities::{note_tags, notes as note_rows, tags as rows},
    notes::Notes,
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter};
use serial_test::serial;

use super::{database, note, user};

/// Two users who type the same tag get a tag each, and each listing shows its owner's.
#[tokio::test]
#[serial]
async fn a_tag_belongs_to_one_user() {
    let db = database().await;
    let (ada, grace) = (user(&db, "Ada").await, user(&db, "Grace").await);
    let hers = note(&db, ada, "Ada's", None).await;
    let other = note(&db, grace, "Grace's", None).await;
    Notes::set_tags(&db, ada, hers.id, "rust").await.unwrap();
    Notes::set_tags(&db, grace, other.id, "rust, web")
        .await
        .unwrap();

    let named_rust = rows::Entity::find()
        .filter(rows::Column::Name.eq("rust"))
        .all(&db)
        .await
        .unwrap();
    let mut owners: Vec<i64> = named_rust.iter().map(|t| t.user_id).collect();
    owners.sort_unstable();
    assert_eq!(owners, [ada, grace]);
    let listed = Notes::listed(&db, ada, |all| all).await.unwrap();
    assert_eq!(listed[0].tags, ["rust"]);
}

/// Deleting a note removes its pairs and leaves the tags for the next note.
#[tokio::test]
#[serial]
async fn deleting_a_note_drops_its_pairs_and_keeps_the_tags() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    let first = note(&db, ada, "First", None).await;
    Notes::set_tags(&db, ada, first.id, "rust, web")
        .await
        .unwrap();
    assert_eq!(note_tags::Entity::find().count(&db).await.unwrap(), 2);

    note_rows::Entity::find_by_id(first.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the note")
        .delete(&db)
        .await
        .unwrap();
    assert_eq!(note_tags::Entity::find().count(&db).await.unwrap(), 0);
    assert_eq!(rows::Entity::find().count(&db).await.unwrap(), 2);
}
