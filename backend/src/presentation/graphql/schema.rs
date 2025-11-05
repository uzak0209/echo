use async_graphql::{EmptySubscription, Schema};
use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::{
    application::usecases::{CreatePostUseCase, GetTimelineUseCase, IncrementDisplayCountUseCase},
    infrastructure::persistence::{PostRepositoryImpl, UserRepositoryImpl},
};

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(db: DatabaseConnection) -> AppSchema {
    // Create repositories
    let post_repo = Arc::new(PostRepositoryImpl::new(db.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(db.clone()));

    // Create use cases
    let get_timeline_use_case = Arc::new(GetTimelineUseCase::new(post_repo.clone()));
    let create_post_use_case = Arc::new(CreatePostUseCase::new(post_repo.clone(), user_repo.clone()));
    let increment_display_count_use_case = Arc::new(IncrementDisplayCountUseCase::new(post_repo.clone()));

    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(get_timeline_use_case)
        .data(create_post_use_case)
        .data(increment_display_count_use_case)
        .finish()
}
