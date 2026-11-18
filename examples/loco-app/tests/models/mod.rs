//! The models against the database the migrations make: what the app's own code adds to the
//! generated entities (tags, the listing, `updated_at`) and what the foreign keys do when a
//! row goes. Each test boots on an empty database (`dangerously_recreate` in `test.yaml`).

mod notebooks;
mod notes;
mod tags;
mod tasks;

use loco_app::{
    app::App,
    models::{
        _entities::{notebooks as notebook_rows, notes as note_rows},
        users::{self, RegisterParams},
    },
};
use loco_rs::testing::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

/// An empty database with every migration run.
async fn database() -> DatabaseConnection {
    boot_test::<App>().await.unwrap().app_context.db
}

/// A new user's id.
async fn user(db: &DatabaseConnection, name: &str) -> i64 {
    let params = RegisterParams {
        email: format!("{}@example.com", name.to_lowercase()),
        password: "secret".to_string(),
        name: name.to_string(),
    };
    users::Model::create_with_password(db, &params)
        .await
        .unwrap()
        .id
}

/// A note of `user` with this title, in `notebook` if one is given.
async fn note(
    db: &DatabaseConnection,
    user: i64,
    title: &str,
    notebook: Option<i64>,
) -> note_rows::Model {
    note_rows::ActiveModel {
        title: Set(title.to_string()),
        user_id: Set(user),
        notebook_id: Set(notebook),
        pinned: Set(false),
        archived: Set(false),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
}

/// A notebook of `user`.
async fn notebook(db: &DatabaseConnection, user: i64, name: &str) -> notebook_rows::Model {
    notebook_rows::ActiveModel {
        name: Set(name.to_string()),
        user_id: Set(user),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
}
