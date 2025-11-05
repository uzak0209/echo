use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set};
use crate::{
    domain::{
        entities::{Post, PostId},
        value_objects::{PostContent, DisplayCount},
        repositories::PostRepository,
        error::DomainError,
    },
    infrastructure::persistence::models::post,
};

pub struct PostRepositoryImpl {
    db: DatabaseConnection,
}

impl PostRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_entity(model: post::Model) -> Result<Post, DomainError> {
        Ok(Post {
            id: PostId(model.id),
            user_id: model.user_id,
            content: PostContent::new(model.content)?,
            image_url: model.image_url,
            display_count: model.display_count.into(),
            created_at: model.created_at.into(),
        })
    }

    fn entity_to_active_model(post: &Post) -> post::ActiveModel {
        post::ActiveModel {
            id: if post.id.0 == 0 {
                sea_orm::ActiveValue::NotSet
            } else {
                Set(post.id.0)
            },
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
    async fn find_by_id(&self, id: PostId) -> Result<Option<Post>, DomainError> {
        let model = post::Entity::find_by_id(id.0).one(&self.db).await?;

        match model {
            Some(m) => Ok(Some(Self::model_to_entity(m)?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<Post>, DomainError> {
        let models = post::Entity::find().all(&self.db).await?;

        models
            .into_iter()
            .map(Self::model_to_entity)
            .collect()
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

    async fn save(&self, post: &Post) -> Result<(), DomainError> {
        let active_model = Self::entity_to_active_model(post);

        if post.id.0 == 0 {
            active_model.insert(&self.db).await?;
        } else {
            active_model.update(&self.db).await?;
        }

        Ok(())
    }

    async fn delete(&self, id: PostId) -> Result<(), DomainError> {
        post::Entity::delete_by_id(id.0).exec(&self.db).await?;
        Ok(())
    }
}
