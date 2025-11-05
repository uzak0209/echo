use async_graphql::SimpleObject;
use crate::application::dto::PostDto;

#[derive(SimpleObject)]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub image_url: Option<String>,
}

impl From<PostDto> for Post {
    fn from(dto: PostDto) -> Self {
        Self {
            id: dto.id,
            content: dto.content,
            image_url: dto.image_url,
        }
    }
}
