//! Which tags each note has. Loco's join-table migration (`CreateJoinTableNotesAndTags`),
//! written out by hand because the generator needs the app to compile first.

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_join_table(m, "note_tags", &[], &[("note", ""), ("tag", "")]).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "note_tags").await
    }
}
