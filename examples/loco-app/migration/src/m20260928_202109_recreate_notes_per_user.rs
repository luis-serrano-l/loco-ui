//! Notes become the signed-in user's, in a notebook or none, pinned or archived. Written by
//! hand: SQLite cannot add a NOT NULL reference to a table that has rows, and a note's
//! notebook is optional (`ON DELETE SET NULL`), which Loco's `create_table` references are not.
//! So the table is dropped and made again: notes written before this migration are lost.

use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{
    prelude::*,
    schema::{big_integer, big_integer_null, boolean, date_null, pk_auto, string, text_null},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table("notes").to_owned())
            .await?;
        let notes = table_auto_tz("notes")
            .col(pk_auto("id"))
            .col(string("title"))
            .col(text_null("body"))
            .col(boolean("pinned").default(false))
            .col(boolean("archived").default(false))
            .col(date_null("due"))
            .col(big_integer("user_id"))
            .col(big_integer_null("notebook_id"))
            .foreign_key(
                ForeignKey::create()
                    .name("fk-notes-user_id-to-users")
                    .from("notes", "user_id")
                    .to("users", "id")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-notes-notebook_id-to-notebooks")
                    .from("notes", "notebook_id")
                    .to("notebooks", "id")
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();
        m.create_table(notes).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table("notes").to_owned())
            .await?;
        let notes = table_auto_tz("notes")
            .col(pk_auto("id"))
            .col(string("title"))
            .col(text_null("body"))
            .col(boolean("done"))
            .col(date_null("due"))
            .to_owned();
        m.create_table(notes).await
    }
}
