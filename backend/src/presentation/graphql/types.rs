use crate::application::dto::PostDto;
use crate::application::usecases::{AuthTokens, LoginTokens, RefreshedTokens, SignupTokens};
use crate::domain::entities::ReactionType;
use async_graphql::{Enum, InputObject, SimpleObject};

/// GraphQL output type for Post (response)
#[derive(SimpleObject)]
pub struct Post {
    // Expose the UUID as a string in the GraphQL layer.
    pub id: String,
    pub content: String,
    pub image_url: Option<String>,
    pub author_name: String,
    pub author_avatar: String,
}

impl From<PostDto> for Post {
    fn from(dto: PostDto) -> Self {
        Self {
            id: dto.id.to_string(),
            content: dto.content,
            image_url: dto.image_url,
            author_name: dto.author_name,
            author_avatar: dto.author_avatar,
        }
    }
}

/// GraphQL input type for creating a Post (request)
#[derive(InputObject)]
pub struct CreatePostInput {
    pub content: String,
    pub image_url: Option<String>,
}

#[derive(SimpleObject)]
pub struct AuthResponse {
    pub access_token: String,
    pub user_id: String,
}

impl From<AuthTokens> for AuthResponse {
    fn from(tokens: AuthTokens) -> Self {
        Self {
            access_token: tokens.access_token,
            user_id: tokens.user_id,
        }
    }
}

impl From<LoginTokens> for AuthResponse {
    fn from(tokens: LoginTokens) -> Self {
        Self {
            access_token: tokens.access_token,
            user_id: tokens.user_id,
        }
    }
}

impl From<SignupTokens> for AuthResponse {
    fn from(tokens: SignupTokens) -> Self {
        Self {
            access_token: tokens.access_token,
            user_id: tokens.user_id,
        }
    }
}

#[derive(SimpleObject)]
pub struct RefreshResponse {
    pub access_token: String,
}

impl From<RefreshedTokens> for RefreshResponse {
    fn from(tokens: RefreshedTokens) -> Self {
        Self {
            access_token: tokens.access_token,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ReactionTypeGql {
    Surprise,
    Empathy,
    Laugh,
    Sad,
    Confused,
}

impl From<ReactionTypeGql> for ReactionType {
    fn from(gql: ReactionTypeGql) -> Self {
        match gql {
            ReactionTypeGql::Surprise => ReactionType::Surprise,
            ReactionTypeGql::Empathy => ReactionType::Empathy,
            ReactionTypeGql::Laugh => ReactionType::Laugh,
            ReactionTypeGql::Sad => ReactionType::Sad,
            ReactionTypeGql::Confused => ReactionType::Confused,
        }
    }
}

impl From<ReactionType> for ReactionTypeGql {
    fn from(domain: ReactionType) -> Self {
        match domain {
            ReactionType::Surprise => ReactionTypeGql::Surprise,
            ReactionType::Empathy => ReactionTypeGql::Empathy,
            ReactionType::Laugh => ReactionTypeGql::Laugh,
            ReactionType::Sad => ReactionTypeGql::Sad,
            ReactionType::Confused => ReactionTypeGql::Confused,
        }
    }
}
