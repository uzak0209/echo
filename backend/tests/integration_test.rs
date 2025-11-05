use echo_backend::application::error::AppError;
use echo_backend::application::usecases::{CreatePostUseCase, GetTimelineUseCase, IncrementDisplayCountUseCase};
use echo_backend::domain::entities::post::{Post, PostId};
use echo_backend::domain::entities::user::UserId;
use echo_backend::domain::error::DomainError;
use echo_backend::domain::repositories::{PostRepository, UserRepository};
use echo_backend::domain::value_objects::{DisplayCount, DisplayName, PostContent};
use async_trait::async_trait;
use rstest::*;
use std::sync::{Arc, Mutex};

// Mock PostRepository for testing
#[derive(Clone)]
struct MockPostRepository {
    posts: Arc<Mutex<Vec<Post>>>,
    next_id: Arc<Mutex<i32>>,
}

impl MockPostRepository {
    fn new() -> Self {
        Self {
            posts: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

#[async_trait]
impl PostRepository for MockPostRepository {
    async fn find_by_id(&self, id: PostId) -> Result<Option<Post>, DomainError> {
        let posts = self.posts.lock().unwrap();
        Ok(posts.iter().find(|p| p.id == id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Post>, DomainError> {
        let posts = self.posts.lock().unwrap();
        Ok(posts.clone())
    }

    async fn find_available(&self, limit: usize) -> Result<Vec<Post>, DomainError> {
        let posts = self.posts.lock().unwrap();
        let available: Vec<Post> = posts
            .iter()
            .filter(|p| !p.is_expired())
            .take(limit)
            .cloned()
            .collect();
        Ok(available)
    }

    async fn save(&self, post: &Post) -> Result<(), DomainError> {
        let mut posts = self.posts.lock().unwrap();

        // If post has ID 0, assign a new ID
        let post_to_save = if post.id.0 == 0 {
            let mut next_id = self.next_id.lock().unwrap();
            let new_id = *next_id;
            *next_id += 1;
            drop(next_id);

            Post {
                id: PostId(new_id),
                user_id: post.user_id,
                content: post.content.clone(),
                image_url: post.image_url.clone(),
                display_count: post.display_count,
                created_at: post.created_at,
            }
        } else {
            post.clone()
        };

        if let Some(index) = posts.iter().position(|p| p.id == post_to_save.id) {
            posts[index] = post_to_save;
        } else {
            posts.push(post_to_save);
        }
        Ok(())
    }

    async fn delete(&self, id: PostId) -> Result<(), DomainError> {
        let mut posts = self.posts.lock().unwrap();
        posts.retain(|p| p.id != id);
        Ok(())
    }
}

// Mock UserRepository for testing
#[derive(Clone)]
struct MockUserRepository;

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, _id: UserId) -> Result<Option<echo_backend::domain::entities::user::User>, DomainError> {
        Ok(None)
    }

    async fn find_all(&self) -> Result<Vec<echo_backend::domain::entities::user::User>, DomainError> {
        Ok(vec![])
    }

    async fn save(&self, _user: &echo_backend::domain::entities::user::User) -> Result<UserId, DomainError> {
        Ok(UserId(1))
    }

    async fn get_random(&self) -> Result<Option<echo_backend::domain::entities::user::User>, DomainError> {
        use echo_backend::domain::entities::user::User;

        Ok(Some(User::new(
            UserId(1),
            DisplayName::new("TestUser".to_string()),
            "https://example.com/avatar.jpg".to_string(),
        )))
    }
}

// Fixtures
#[fixture]
fn mock_post_repo() -> Arc<dyn PostRepository> {
    Arc::new(MockPostRepository::new())
}

#[fixture]
fn mock_user_repo() -> Arc<dyn UserRepository> {
    Arc::new(MockUserRepository)
}

// CreatePostUseCase tests
#[rstest]
#[tokio::test]
async fn test_create_post_success(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    let use_case = CreatePostUseCase::new(mock_post_repo.clone(), mock_user_repo);

    let result = use_case
        .execute("Test post content".to_string(), None)
        .await;

    assert!(result.is_ok());
    assert!(result.unwrap());

    // Verify post was saved
    let posts = mock_post_repo.find_all().await.unwrap();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].content.value(), "Test post content");
}

#[rstest]
#[tokio::test]
async fn test_create_post_with_image(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    let use_case = CreatePostUseCase::new(mock_post_repo.clone(), mock_user_repo);

    let result = use_case
        .execute(
            "Test post with image".to_string(),
            Some("https://example.com/image.jpg".to_string()),
        )
        .await;

    assert!(result.is_ok());

    let posts = mock_post_repo.find_all().await.unwrap();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].image_url, Some("https://example.com/image.jpg".to_string()));
}

#[rstest]
#[tokio::test]
async fn test_create_post_with_empty_content(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    let use_case = CreatePostUseCase::new(mock_post_repo, mock_user_repo);

    let result = use_case.execute("".to_string(), None).await;

    assert!(result.is_err());
}

#[rstest]
#[tokio::test]
async fn test_create_post_with_too_long_content(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    let use_case = CreatePostUseCase::new(mock_post_repo, mock_user_repo);

    let long_content = "a".repeat(1001);
    let result = use_case.execute(long_content, None).await;

    assert!(result.is_err());
}

// GetTimelineUseCase tests
#[rstest]
#[tokio::test]
async fn test_get_timeline_empty(mock_post_repo: Arc<dyn PostRepository>) {
    let use_case = GetTimelineUseCase::new(mock_post_repo);

    let result = use_case.execute(10).await;

    assert!(result.is_ok());
    let timeline = result.unwrap();
    assert_eq!(timeline.len(), 0);
}

#[rstest]
#[tokio::test]
async fn test_get_timeline_with_posts(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    // Create some posts first
    let create_use_case = CreatePostUseCase::new(mock_post_repo.clone(), mock_user_repo);

    create_use_case.execute("Post 1".to_string(), None).await.unwrap();
    create_use_case.execute("Post 2".to_string(), None).await.unwrap();
    create_use_case.execute("Post 3".to_string(), None).await.unwrap();

    let get_timeline_use_case = GetTimelineUseCase::new(mock_post_repo);
    let result = get_timeline_use_case.execute(10).await;

    assert!(result.is_ok());
    let timeline = result.unwrap();
    assert_eq!(timeline.len(), 3);
}

#[rstest]
#[tokio::test]
async fn test_get_timeline_respects_limit(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    let create_use_case = CreatePostUseCase::new(mock_post_repo.clone(), mock_user_repo);

    for i in 0..5 {
        create_use_case
            .execute(format!("Post {}", i), None)
            .await
            .unwrap();
    }

    let get_timeline_use_case = GetTimelineUseCase::new(mock_post_repo);
    let result = get_timeline_use_case.execute(3).await;

    assert!(result.is_ok());
    let timeline = result.unwrap();
    assert_eq!(timeline.len(), 3);
}

// IncrementDisplayCountUseCase tests
#[rstest]
#[tokio::test]
async fn test_increment_display_count_success(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    // Create a post first
    let create_use_case = CreatePostUseCase::new(mock_post_repo.clone(), mock_user_repo);
    create_use_case.execute("Test post".to_string(), None).await.unwrap();

    let posts = mock_post_repo.find_all().await.unwrap();
    let post_id = posts[0].id;

    // Increment display count
    let increment_use_case = IncrementDisplayCountUseCase::new(mock_post_repo.clone());
    let result = increment_use_case.execute(post_id.0).await;

    assert!(result.is_ok());
    assert!(result.unwrap());

    // Verify count was incremented
    let post = mock_post_repo.find_by_id(post_id).await.unwrap().unwrap();
    assert_eq!(post.display_count.value(), 1);
}

#[rstest]
#[tokio::test]
async fn test_increment_display_count_deletes_after_10(
    mock_post_repo: Arc<dyn PostRepository>,
    mock_user_repo: Arc<dyn UserRepository>,
) {
    let create_use_case = CreatePostUseCase::new(mock_post_repo.clone(), mock_user_repo);
    create_use_case.execute("Test post".to_string(), None).await.unwrap();

    let posts = mock_post_repo.find_all().await.unwrap();
    let post_id = posts[0].id;

    let increment_use_case = IncrementDisplayCountUseCase::new(mock_post_repo.clone());

    // Increment 10 times
    for _ in 0..10 {
        increment_use_case.execute(post_id.0).await.unwrap();
    }

    // Post should be deleted
    let post = mock_post_repo.find_by_id(post_id).await.unwrap();
    assert!(post.is_none());
}

#[rstest]
#[tokio::test]
async fn test_increment_display_count_not_found(mock_post_repo: Arc<dyn PostRepository>) {
    let increment_use_case = IncrementDisplayCountUseCase::new(mock_post_repo);

    let result = increment_use_case.execute(999).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}
