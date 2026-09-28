pub use super::_entities::notes::{ActiveModel, Column, Entity, Model};
use std::collections::HashMap;

use sea_orm::{ActiveValue::Set, QueryOrder, entity::prelude::*};

use super::_entities::{note_tags, notebooks, tags};
pub type Notes = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

/// A note as the pages show it: with its tag names and its notebook.
#[derive(Clone, Debug)]
pub struct Listed {
    pub note: Model,
    pub tags: Vec<String>,
    /// `(id, name)`
    pub notebook: Option<(i64, String)>,
}

/// Tag names as typed in the form: split on commas, trimmed, lowercased, each once.
pub fn tag_names(text: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for name in text.split(',').map(|t| t.trim().to_lowercase()) {
        if !name.is_empty() && !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

impl Entity {
    /// `user`'s notes that `filter` keeps, newest first, each with its tags and notebook.
    pub async fn listed(
        db: &DatabaseConnection,
        user: i64,
        filter: impl FnOnce(Select<Entity>) -> Select<Entity>,
    ) -> Result<Vec<Listed>, DbErr> {
        let notes = filter(Entity::find().filter(Column::UserId.eq(user)))
            .order_by_desc(Column::Pinned)
            .order_by_desc(Column::UpdatedAt)
            .all(db)
            .await?;
        let mut tags_of: HashMap<i64, Vec<String>> = HashMap::new();
        let pairs = note_tags::Entity::find()
            .find_also_related(tags::Entity)
            .filter(tags::Column::UserId.eq(user))
            .order_by_asc(tags::Column::Name)
            .all(db)
            .await?;
        for (pair, tag) in pairs {
            if let Some(tag) = tag {
                tags_of.entry(pair.note_id).or_default().push(tag.name);
            }
        }
        let books: HashMap<i64, String> = notebooks::Entity::find()
            .filter(notebooks::Column::UserId.eq(user))
            .all(db)
            .await?
            .into_iter()
            .map(|b| (b.id, b.name))
            .collect();
        Ok(notes
            .into_iter()
            .map(|note| Listed {
                tags: tags_of.remove(&note.id).unwrap_or_default(),
                notebook: (note.notebook_id).and_then(|id| Some((id, books.get(&id)?.clone()))),
                note,
            })
            .collect())
    }

    /// Give note `note` of `user` exactly the tags named in `text`, making the missing ones.
    pub async fn set_tags(
        db: &DatabaseConnection,
        user: i64,
        note: i64,
        text: &str,
    ) -> Result<(), DbErr> {
        note_tags::Entity::delete_many()
            .filter(note_tags::Column::NoteId.eq(note))
            .exec(db)
            .await?;
        for name in tag_names(text) {
            let found = tags::Entity::find()
                .filter(tags::Column::UserId.eq(user))
                .filter(tags::Column::Name.eq(&name))
                .one(db)
                .await?;
            let tag = match found {
                Some(tag) => tag,
                None => {
                    tags::ActiveModel {
                        name: Set(name),
                        user_id: Set(user),
                        ..Default::default()
                    }
                    .insert(db)
                    .await?
                }
            };
            note_tags::ActiveModel {
                note_id: Set(note),
                tag_id: Set(tag.id),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        Ok(())
    }
}
