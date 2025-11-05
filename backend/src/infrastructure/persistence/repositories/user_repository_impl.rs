use crate::{
    domain::{
        entities::{DisplayName, User, UserId},
        repositories::UserRepository,
    },
    infrastructure::persistence::models::user,
};
use async_trait::async_trait;
use rand::seq::SliceRandom;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub struct UserRepositoryImpl {
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_entity(model: user::Model) -> User {
        User {
            id: UserId(model.id),
            display_name: DisplayName::new(model.display_name),
            avatar_url: model.avatar_url,
            created_at: model.created_at.into(),
        }
    }

    fn entity_to_active_model(user: &User) -> user::ActiveModel {
        user::ActiveModel {
            id: if user.id.0 == 0 {
                sea_orm::ActiveValue::NotSet
            } else {
                Set(user.id.0)
            },
            display_name: Set(user.display_name.value().to_string()),
            avatar_url: Set(user.avatar_url.clone()),
            created_at: Set(user.created_at.into()),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(
        &self,
        id: UserId,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let model = user::Entity::find_by_id(id.0).one(&self.db).await?;

        Ok(model.map(Self::model_to_entity))
    }

    async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let models = user::Entity::find().all(&self.db).await?;

        Ok(models.into_iter().map(Self::model_to_entity).collect())
    }

    async fn save(&self, user: &User) -> Result<UserId, Box<dyn std::error::Error + Send + Sync>> {
        let active_model = Self::entity_to_active_model(user);

        let result = if user.id.0 == 0 {
            active_model.insert(&self.db).await?
        } else {
            active_model.update(&self.db).await?
        };

        Ok(UserId(result.id))
    }

    async fn get_random(&self) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let users = self.find_all().await?;

        if users.is_empty() {
            return Ok(None);
        }

        let mut rng = rand::thread_rng();
        let user = users.choose(&mut rng).cloned();

        Ok(user)
    }
}
