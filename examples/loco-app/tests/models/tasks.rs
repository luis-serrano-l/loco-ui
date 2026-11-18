use loco_app::models::_entities::tasks as rows;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    EntityTrait, IntoActiveModel, PaginatorTrait,
    prelude::{Date, Decimal},
};
use serial_test::serial;

use super::{database, user};

/// Every field kind the scaffold wrote comes back as it went in: a date, a date-time
/// without a zone, a decimal, an integer, and the optional ones empty.
#[tokio::test]
#[serial]
async fn a_task_round_trips_every_field_kind() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    let day = Date::from_ymd_opt(2026, 10, 1).expect("a date");
    let full = rows::ActiveModel {
        title: Set("Ship".to_string()),
        done: Set(true),
        due_on: Set(Some(day)),
        starts_at: Set(day.and_hms_opt(9, 30, 0)),
        price: Set(Some(Decimal::new(1250, 2))),
        status: Set("doing".to_string()),
        size: Set(Some(3)),
        user_id: Set(ada),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    let read = rows::Entity::find_by_id(full.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the task");
    assert_eq!(
        (read.title.as_str(), read.done, read.status.as_str()),
        ("Ship", true, "doing")
    );
    assert_eq!(read.due_on, Some(day));
    assert_eq!(read.starts_at, day.and_hms_opt(9, 30, 0));
    assert_eq!(read.price, Some(Decimal::new(1250, 2)));
    assert_eq!(read.size, Some(3));
    assert_eq!(read.remind_at, None);

    let bare = rows::ActiveModel {
        title: Set("Later".to_string()),
        done: Set(false),
        status: Set("todo".to_string()),
        user_id: Set(ada),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    assert_eq!(
        (bare.due_on, bare.starts_at, bare.price, bare.size),
        (None, None, None, None)
    );
}

/// Saving a change moves `updated_at`; a user's tasks go with the user.
#[tokio::test]
#[serial]
async fn a_task_tracks_its_change_and_its_owner() {
    let db = database().await;
    let ada = user(&db, "Ada").await;
    let before = rows::ActiveModel {
        title: Set("Ship".to_string()),
        done: Set(false),
        status: Set("todo".to_string()),
        user_id: Set(ada),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let mut moved = before.clone().into_active_model();
    moved.status = Set("done".to_string());
    let after = moved.update(&db).await.unwrap();
    assert_eq!(after.status, "done");
    assert!(after.updated_at > before.updated_at);

    loco_app::models::_entities::users::Entity::delete_many()
        .exec(&db)
        .await
        .unwrap();
    assert_eq!(rows::Entity::find().count(&db).await.unwrap(), 0);
}
