use crate::application::dto::PostDto;
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Post {
    // Expose the UUID as a string in the GraphQL layer.
    pub id: String,
    pub content: String,
    pub image_url: Option<String>,
}

impl From<PostDto> for Post {
    fn from(dto: PostDto) -> Self {
        Self {
            id: dto.id.to_string(),
            content: dto.content,
            image_url: dto.image_url,
        }
    }
}
