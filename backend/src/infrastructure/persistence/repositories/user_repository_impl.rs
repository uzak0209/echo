use crate::{
    domain::{
        entities::User, error::DomainError, repositories::UserRepository,
        value_objects::DisplayName,
    },
    infrastructure::persistence::models::user,
};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct UserRepositoryImpl {
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_entity(model: user::Model) -> User {
        User {
            id: model.id,
            display_name: DisplayName::new(model.display_name),
            avatar_url: model.avatar_url,
            password_hash: model.password_hash,
            refresh_token: model.refresh_token,
            created_at: model.created_at.into(),
        }
    }

    fn entity_to_active_model(user: &User) -> user::ActiveModel {
        user::ActiveModel {
            id: Set(user.id),
            display_name: Set(user.display_name.value().to_string()),
            avatar_url: Set(user.avatar_url.clone()),
            password_hash: Set(user.password_hash.clone()),
            refresh_token: Set(None),
            valid: Set(true),
            created_at: Set(user.created_at.into()),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let model = user::Entity::find_by_id(id).one(&self.db).await?;

        Ok(model.map(Self::model_to_entity))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let model = user::Entity::find()
            .filter(user::Column::DisplayName.eq(username))
            .one(&self.db)
            .await?;

        Ok(model.map(Self::model_to_entity))
    }

    async fn create_user(
        &self,
        display_name: String,
        avatar_url: Option<String>,
    ) -> Result<User, DomainError> {
        let display_name = DisplayName::new(display_name);
        let avatar_url = avatar_url.unwrap_or_else(|| "https://example.com/default-avatar.jpg".to_string());
        let user = User::new(display_name, avatar_url);
        let active_model = Self::entity_to_active_model(&user);
        let result = active_model.insert(&self.db).await?;
        Ok(Self::model_to_entity(result))
    }

    async fn create_user_with_credentials(
        &self,
        display_name: String,
        avatar_url: Option<String>,
        password_hash: String,
    ) -> Result<User, DomainError> {
        let display_name = DisplayName::new(display_name);
        let avatar_url = avatar_url.unwrap_or_else(|| "https://example.com/default-avatar.jpg".to_string());
        let user = User::new_with_credentials(display_name, avatar_url, password_hash);
        let active_model = Self::entity_to_active_model(&user);
        let result = active_model.insert(&self.db).await?;
        Ok(Self::model_to_entity(result))
    }

    async fn update_refresh_token(
        &self,
        user_id: Uuid,
        refresh_token: Option<String>,
    ) -> Result<(), DomainError> {
        let user_model = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".to_string()))?;

        let mut active_model: user::ActiveModel = user_model.into();
        active_model.refresh_token = Set(refresh_token);
        active_model.update(&self.db).await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        user::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
