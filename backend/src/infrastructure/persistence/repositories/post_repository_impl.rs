use crate::{
    domain::{
        entities::{Post, User},
        error::DomainError,
        repositories::PostRepository,
        value_objects::{DisplayCount, DisplayName, PostContent},
    },
    infrastructure::persistence::models::{post, user},
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

    fn user_model_to_entity(model: user::Model) -> User {
        User {
            id: model.id,
            display_name: DisplayName::new(model.display_name),
            avatar_url: model.avatar_url,
            password_hash: model.password_hash,
            created_at: model.created_at.into(),
        }
    }

    fn entity_to_active_model(post: &Post) -> post::ActiveModel {
        post::ActiveModel {
            id: Set(post.id),
            user_id: Set(post.user_id),
            content: Set(post.content.value().to_string()),
            image_url: Set(post.image_url.clone()),
            valid: Set(true),
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

    async fn find_available(&self, limit: usize, exclude_user_id: Option<Uuid>) -> Result<Vec<Post>, DomainError> {
        let mut query = post::Entity::find()
            .filter(post::Column::Valid.eq(true))
            .filter(post::Column::DisplayCount.lt(10));

        // Exclude posts from specific user (don't show own posts)
        if let Some(user_id) = exclude_user_id {
            query = query.filter(post::Column::UserId.ne(user_id));
        }

        let models = query.all(&self.db).await?;

        let posts: Vec<Post> = models
            .into_iter()
            .map(Self::model_to_entity)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(posts.into_iter().take(limit).collect())
    }

    async fn find_available_with_users(&self, limit: usize, exclude_user_id: Option<Uuid>) -> Result<Vec<(Post, User)>, DomainError> {
        let mut query = post::Entity::find()
            .find_also_related(user::Entity)
            .filter(post::Column::Valid.eq(true))
            .filter(post::Column::DisplayCount.lt(10));

        // Exclude posts from specific user (don't show own posts)
        if let Some(user_id) = exclude_user_id {
            query = query.filter(post::Column::UserId.ne(user_id));
        }

        let models = query.all(&self.db).await?;

        let results: Vec<(Post, User)> = models
            .into_iter()
            .filter_map(|(post_model, user_model_opt)| {
                let user_model = user_model_opt?;
                let post = Self::model_to_entity(post_model).ok()?;
                let user = Self::user_model_to_entity(user_model);
                Some((post, user))
            })
            .take(limit)
            .collect();

        Ok(results)
    }

    async fn create(&self, post: &Post) -> Result<Post, DomainError> {
        let active_model = Self::entity_to_active_model(post);
        let result = active_model.insert(&self.db).await?;
        Self::model_to_entity(result)
    }

    async fn increment_display_count(&self, id: Uuid) -> Result<Post, DomainError> {
        let model = post::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| DomainError::NotFound("Post not found".to_string()))?;

        let mut active_model: post::ActiveModel = model.into();
        let new_count = active_model.display_count.clone().unwrap() + 1;
        active_model.display_count = Set(new_count);

        // If display count reaches 10, mark as invalid (expired)
        if new_count >= 10 {
            active_model.valid = Set(false);
        }

        let updated = active_model.update(&self.db).await?;
        Self::model_to_entity(updated)
    }
}
