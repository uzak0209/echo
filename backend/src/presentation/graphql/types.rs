use crate::application::dto::PostDto;
use crate::application::usecases::{AuthTokens, ExpressionState, LoginTokens, RefreshedTokens, SignupTokens};
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

/// Reaction count by type for expression state
#[derive(SimpleObject)]
pub struct ReactionCount {
    pub reaction_type: ReactionTypeGql,
    pub count: i64,
}

/// User's 3D model expression state based on reactions received
#[derive(SimpleObject)]
pub struct UserExpressionState {
    /// Dominant expression type based on most received reactions
    pub dominant_expression: Option<ReactionTypeGql>,
    /// Intensity level (0.0 to 1.0) based on total reaction count
    pub intensity: f32,
    /// Breakdown of reaction counts by type
    pub reaction_counts: Vec<ReactionCount>,
    /// Total number of reactions received
    pub total_reactions: i64,
}

impl From<ExpressionState> for UserExpressionState {
    fn from(state: ExpressionState) -> Self {
        Self {
            dominant_expression: state.dominant_expression.map(|r| r.into()),
            intensity: state.intensity,
            reaction_counts: state
                .reaction_counts
                .into_iter()
                .map(|(reaction_type, count)| ReactionCount {
                    reaction_type: reaction_type.into(),
                    count,
                })
                .collect(),
            total_reactions: state.total_reactions,
        }
    }
}
