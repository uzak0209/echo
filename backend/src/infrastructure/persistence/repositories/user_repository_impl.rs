use crate::{
    domain::{
        entities::User, error::DomainError, repositories::UserRepository,
        value_objects::DisplayName,
    },
    infrastructure::persistence::models::user,
};
use async_trait::async_trait;
use rand::seq::SliceRandom;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
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
            created_at: model.created_at.into(),
        }
    }

    fn entity_to_active_model(user: &User) -> user::ActiveModel {
        user::ActiveModel {
            id: Set(user.id),
            display_name: Set(user.display_name.value().to_string()),
            avatar_url: Set(user.avatar_url.clone()),
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

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        let models = user::Entity::find().all(&self.db).await?;

        Ok(models.into_iter().map(Self::model_to_entity).collect())
    }

    async fn save(&self, user: &User) -> Result<Uuid, DomainError> {
        let active_model = Self::entity_to_active_model(user);

        let result = active_model.insert(&self.db).await?;

        Ok(result.id)
    }

    async fn get_random(&self) -> Result<Option<User>, DomainError> {
        let users = self.find_all().await?;

        if users.is_empty() {
            return Ok(None);
        }

        let mut rng = rand::thread_rng();
        let user = users.choose(&mut rng).cloned();

        Ok(user)
    }
}
