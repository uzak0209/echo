use crate::{
    domain::{
        entities::Post,
        error::DomainError,
        repositories::PostRepository,
        value_objects::{DisplayCount, PostContent},
    },
    infrastructure::persistence::models::post,
};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct PostRepositoryImpl {
    db: DatabaseConnection,
}

impl PostRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_entity(model: post::Model) -> Result<Post, DomainError> {
        Ok(Post {
            id: model.id,
            user_id: model.user_id,
            content: PostContent::new(model.content)?,
            image_url: model.image_url,
            display_count: model.display_count.into(),
            created_at: model.created_at.into(),
        })
    }

    fn entity_to_active_model(post: &Post) -> post::ActiveModel {
        post::ActiveModel {
            id: Set(post.id),
            user_id: Set(post.user_id),
            content: Set(post.content.value().to_string()),
            image_url: Set(post.image_url.clone()),
            display_count: Set(post.display_count.value()),
            created_at: Set(post.created_at.into()),
        }
    }
}

impl From<i32> for DisplayCount {
    fn from(value: i32) -> Self {
        DisplayCount::from_value(value)
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
        let model = post::Entity::find_by_id(id).one(&self.db).await?;

        match model {
            Some(m) => Ok(Some(Self::model_to_entity(m)?)),
            None => Ok(None),
        }
    }

    async fn find_available(&self, limit: usize) -> Result<Vec<Post>, DomainError> {
        let models = post::Entity::find()
            .filter(post::Column::DisplayCount.lt(10))
            .all(&self.db)
            .await?;

        let posts: Vec<Post> = models
            .into_iter()
            .map(Self::model_to_entity)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(posts.into_iter().take(limit).collect())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        post::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
