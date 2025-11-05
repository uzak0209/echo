use crate::application::dto::PostDto;
use crate::application::usecases::{AuthTokens, RefreshedTokens};
use async_graphql::SimpleObject;

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
