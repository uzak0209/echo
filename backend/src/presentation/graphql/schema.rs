use async_graphql::{EmptySubscription, Schema};
use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::{
    application::usecases::{
        CreatePostUseCase, CreateUserUseCase, GetTimelineUseCase, IncrementDisplayCountUseCase,
        RefreshTokenUseCase,
    },
    infrastructure::{
        auth::JwtService,
        persistence::{PostRepositoryImpl, UserRepositoryImpl},
    },
};

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(db: DatabaseConnection, jwt_secret: String) -> AppSchema {
    // Create JWT service
    let jwt_service = Arc::new(JwtService::new(&jwt_secret));

    // Create repositories
    let post_repo = Arc::new(PostRepositoryImpl::new(db.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(db.clone()));

    // Create use cases
    let get_timeline_use_case = Arc::new(GetTimelineUseCase::new(post_repo.clone()));
    let create_post_use_case =
        Arc::new(CreatePostUseCase::new(post_repo.clone(), user_repo.clone()));
    let increment_display_count_use_case =
        Arc::new(IncrementDisplayCountUseCase::new(post_repo.clone()));
    let create_user_use_case =
        Arc::new(CreateUserUseCase::new(user_repo.clone(), jwt_service.clone()));
    let refresh_token_use_case = Arc::new(RefreshTokenUseCase::new(jwt_service.clone()));

    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(get_timeline_use_case)
        .data(create_post_use_case)
        .data(increment_display_count_use_case)
        .data(create_user_use_case)
        .data(refresh_token_use_case)
        .finish()
}
