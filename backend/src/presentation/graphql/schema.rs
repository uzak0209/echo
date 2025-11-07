use async_graphql::{EmptySubscription, Schema};
use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::{
    application::usecases::{
        AddReactionUseCase, CreatePostUseCase, CreateUserUseCase, EchoPostUseCase,
        GetMyPostsUseCase, GetPostReactionsUseCase, GetTimelineUseCase,
        GetUserLatestReactionUseCase, IncrementDisplayCountUseCase, LoginUseCase,
        RefreshTokenUseCase, RemoveReactionUseCase, SignupUseCase,
    },
    infrastructure::{
        auth::JwtService,
        persistence::{PostRepositoryImpl, ReactionRepositoryImpl, UserRepositoryImpl},
    },
};

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(
    db: DatabaseConnection,
    jwt_secret: String,
    stream_manager: Arc<crate::infrastructure::sse::ReactionStreamManager>,
) -> AppSchema {
    // Create JWT service
    let jwt_service = Arc::new(JwtService::new(&jwt_secret));

    // Create repositories
    let post_repo = Arc::new(PostRepositoryImpl::new(db.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(db.clone()));
    let reaction_repo = Arc::new(ReactionRepositoryImpl::new(db.clone()));

    // Create use cases
    let get_timeline_use_case = Arc::new(GetTimelineUseCase::new(post_repo.clone(), user_repo.clone()));
    let create_post_use_case =
        Arc::new(CreatePostUseCase::new(post_repo.clone(), user_repo.clone()));
    let increment_display_count_use_case =
        Arc::new(IncrementDisplayCountUseCase::new(post_repo.clone()));
    let echo_post_use_case = Arc::new(EchoPostUseCase::new(post_repo.clone()));
    let create_user_use_case =
        Arc::new(CreateUserUseCase::new(user_repo.clone(), jwt_service.clone()));
    let refresh_token_use_case = Arc::new(RefreshTokenUseCase::new(jwt_service.clone()));
    let login_use_case = Arc::new(LoginUseCase::new(user_repo.clone(), jwt_service.clone()));
    let signup_use_case = Arc::new(SignupUseCase::new(user_repo.clone(), jwt_service.clone()));
    let add_reaction_use_case = Arc::new(AddReactionUseCase::new(
        reaction_repo.clone(),
        post_repo.clone(),
        stream_manager,
    ));
    let remove_reaction_use_case = Arc::new(RemoveReactionUseCase::new(reaction_repo.clone()));
    let get_post_reactions_use_case = Arc::new(GetPostReactionsUseCase::new(reaction_repo.clone()));
    let get_my_posts_use_case = Arc::new(GetMyPostsUseCase::new(
        post_repo.clone(),
        user_repo.clone(),
        reaction_repo.clone(),
    ));
    let get_user_latest_reaction_use_case = Arc::new(GetUserLatestReactionUseCase::new(reaction_repo.clone()));

    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(get_timeline_use_case)
        .data(create_post_use_case)
        .data(increment_display_count_use_case)
        .data(echo_post_use_case)
        .data(create_user_use_case)
        .data(refresh_token_use_case)
        .data(login_use_case)
        .data(signup_use_case)
        .data(add_reaction_use_case)
        .data(remove_reaction_use_case)
        .data(get_post_reactions_use_case)
        .data(get_my_posts_use_case)
        .data(get_user_latest_reaction_use_case)
        .finish()
}
