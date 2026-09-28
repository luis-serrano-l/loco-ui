#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20260925_022143_notes;
mod m20260925_093049_tasks;
mod m20260928_201952_notebooks;
mod m20260928_202109_recreate_notes_per_user;
mod m20260928_202248_tags;
mod m20260928_202300_note_tags;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260925_022143_notes::Migration),
            Box::new(m20260925_093049_tasks::Migration),
            Box::new(m20260928_201952_notebooks::Migration),
            Box::new(m20260928_202109_recreate_notes_per_user::Migration),
            Box::new(m20260928_202248_tags::Migration),
            Box::new(m20260928_202300_note_tags::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
