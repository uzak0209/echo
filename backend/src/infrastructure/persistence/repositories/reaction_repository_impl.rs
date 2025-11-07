use crate::{
    domain::{
        entities::{Reaction, ReactionType},
        error::DomainError,
        repositories::ReactionRepository,
    },
    infrastructure::persistence::models::reaction,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, Set,
};
use uuid::Uuid;

pub struct ReactionRepositoryImpl {
    db: DatabaseConnection,
}

impl ReactionRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_entity(model: reaction::Model) -> Result<Reaction, DomainError> {
        let reaction_type = ReactionType::from_str(&model.reaction_type)
            .ok_or_else(|| DomainError::validation(format!("Invalid reaction type: {}", model.reaction_type)))?;

        Ok(Reaction {
            id: model.id,
            post_id: model.post_id,
            user_id: model.user_id,
            reaction_type,
            created_at: model.created_at.into(),
        })
    }
}

#[async_trait]
impl ReactionRepository for ReactionRepositoryImpl {
    async fn add_reaction(
        &self,
        post_id: Uuid,
        user_id: Uuid,
        reaction_type: ReactionType,
    ) -> Result<Reaction, DomainError> {
        // First, check if this exact reaction already exists
        let existing = reaction::Entity::find()
            .filter(reaction::Column::PostId.eq(post_id))
            .filter(reaction::Column::UserId.eq(user_id))
            .filter(reaction::Column::ReactionType.eq(reaction_type.as_str()))
            .one(&self.db)
            .await?;

        if let Some(existing_model) = existing {
            return Self::model_to_entity(existing_model);
        }

        // Create new reaction
        let new_reaction = Reaction::new(post_id, user_id, reaction_type.clone());

        let active_model = reaction::ActiveModel {
            id: Set(new_reaction.id),
            post_id: Set(new_reaction.post_id),
            user_id: Set(new_reaction.user_id),
            reaction_type: Set(new_reaction.reaction_type.as_str().to_string()),
            created_at: Set(new_reaction.created_at.into()),
        };

        let result = active_model.insert(&self.db).await?;
        Self::model_to_entity(result)
    }

    async fn remove_reaction(
        &self,
        post_id: Uuid,
        user_id: Uuid,
        reaction_type: ReactionType,
    ) -> Result<(), DomainError> {
        reaction::Entity::delete_many()
            .filter(reaction::Column::PostId.eq(post_id))
            .filter(reaction::Column::UserId.eq(user_id))
            .filter(reaction::Column::ReactionType.eq(reaction_type.as_str()))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn find_by_post_id(&self, post_id: Uuid) -> Result<Vec<Reaction>, DomainError> {
        let models = reaction::Entity::find()
            .filter(reaction::Column::PostId.eq(post_id))
            .all(&self.db)
            .await?;

        models
            .into_iter()
            .map(Self::model_to_entity)
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get_reaction_counts(&self, post_id: Uuid) -> Result<Vec<(ReactionType, i64)>, DomainError> {
        let reactions = self.find_by_post_id(post_id).await?;

        let mut counts = std::collections::HashMap::new();
        for reaction in reactions {
            *counts.entry(reaction.reaction_type).or_insert(0) += 1;
        }

        Ok(counts.into_iter().collect())
    }

    async fn get_latest_reaction_for_post(&self, post_id: Uuid) -> Result<Option<Reaction>, DomainError> {
        let model = reaction::Entity::find()
            .filter(reaction::Column::PostId.eq(post_id))
            .order_by_desc(reaction::Column::CreatedAt)
            .one(&self.db)
            .await?;

        match model {
            Some(m) => Ok(Some(Self::model_to_entity(m)?)),
            None => Ok(None),
        }
    }

    async fn get_latest_reaction_for_user(&self, user_id: Uuid) -> Result<Option<Reaction>, DomainError> {
        use crate::infrastructure::persistence::models::post;
        use sea_orm::JoinType;

        // Join reactions with posts to find reactions on posts by this user
        // Then get the most recent one
        let model = reaction::Entity::find()
            .join(JoinType::InnerJoin, reaction::Relation::Post.def())
            .filter(post::Column::UserId.eq(user_id))
            .order_by_desc(reaction::Column::CreatedAt)
            .one(&self.db)
            .await?;

        match model {
            Some(m) => Ok(Some(Self::model_to_entity(m)?)),
            None => Ok(None),
        }
    }
}
