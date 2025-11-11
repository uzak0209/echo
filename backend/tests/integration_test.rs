use async_trait::async_trait;
use echo_backend::application::usecases::{
    CreatePostUseCase, GetTimelineUseCase, IncrementDisplayCountUseCase,
};
use echo_backend::domain::entities::post::Post;
use echo_backend::domain::error::DomainError;
use echo_backend::domain::repositories::{PostRepository, UserRepository};
use echo_backend::domain::value_objects::DisplayName;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Mock PostRepository for testing
#[derive(Clone)]
struct MockPostRepository {
    posts: Arc<Mutex<Vec<Post>>>,
    users: Arc<Mutex<Vec<echo_backend::domain::entities::user::User>>>,
}

impl MockPostRepository {
    fn new() -> Self {
        Self {
            posts: Arc::new(Mutex::new(Vec::new())),
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn with_users(users: Arc<Mutex<Vec<echo_backend::domain::entities::user::User>>>) -> Self {
        Self {
            posts: Arc::new(Mutex::new(Vec::new())),
            users,
        }
    }

    // Helper method for tests
    fn find_all_sync(&self) -> Vec<Post> {
        self.posts.lock().unwrap().clone()
    }
}

#[async_trait]
impl PostRepository for MockPostRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
        let posts = self.posts.lock().unwrap();
        Ok(posts.iter().find(|p| p.id == id).cloned())
    }

    async fn find_available(
        &self,
        limit: usize,
        _exclude_user_id: Option<Uuid>,
    ) -> Result<Vec<Post>, DomainError> {
        let posts = self.posts.lock().unwrap();
        let available: Vec<Post> = posts
            .iter()
            .filter(|p| !p.is_expired())
            .take(limit)
            .cloned()
            .collect();
        Ok(available)
    }

    async fn find_available_with_users(
        &self,
        limit: usize,
        exclude_user_id: Option<Uuid>,
    ) -> Result<Vec<(Post, echo_backend::domain::entities::user::User)>, DomainError> {
        let posts = self.posts.lock().unwrap();
        let users = self.users.lock().unwrap();

        let mut results = Vec::new();
        for post in posts.iter() {
            if post.is_expired() {
                continue;
            }

            if let Some(excluded_id) = exclude_user_id {
                if post.user_id == excluded_id {
                    continue;
                }
            }

            if let Some(user) = users.iter().find(|u| u.id == post.user_id) {
                results.push((post.clone(), user.clone()));
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    async fn create(&self, post: &Post) -> Result<Post, DomainError> {
        let mut posts = self.posts.lock().unwrap();
        let new_post = post.clone();
        posts.push(new_post.clone());
        Ok(new_post)
    }

    async fn increment_display_count(&self, id: Uuid) -> Result<Post, DomainError> {
        let mut posts = self.posts.lock().unwrap();
        if let Some(post) = posts.iter_mut().find(|p| p.id == id) {
            post.increment_display();

            // If expired, remove from list (simulating deletion)
            let post_clone = post.clone();
            if post.is_expired() {
                drop(posts);
                let mut posts = self.posts.lock().unwrap();
                posts.retain(|p| p.id != id);
            }
            Ok(post_clone)
        } else {
            Err(DomainError::NotFound(format!(
                "Post with id {} not found",
                id
            )))
        }
    }
}

// Mock UserRepository for testing
#[derive(Clone)]
struct MockUserRepository {
    users: Arc<Mutex<Vec<echo_backend::domain::entities::user::User>>>,
}

impl MockUserRepository {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<echo_backend::domain::entities::user::User>, DomainError> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.id == id).cloned())
    }

    async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<echo_backend::domain::entities::user::User>, DomainError> {
        let users = self.users.lock().unwrap();
        Ok(users
            .iter()
            .find(|u| u.display_name.value() == username)
            .cloned())
    }

    async fn create_user(
        &self,
        display_name: String,
        avatar_url: Option<String>,
    ) -> Result<echo_backend::domain::entities::user::User, DomainError> {
        use echo_backend::domain::entities::user::User;

        let user = User::new(
            DisplayName::new(display_name),
            avatar_url.unwrap_or_else(|| "https://example.com/avatar.jpg".to_string()),
        );

        let mut users = self.users.lock().unwrap();
        users.push(user.clone());

        Ok(user)
    }

    async fn create_user_with_credentials(
        &self,
        display_name: String,
        avatar_url: Option<String>,
        password_hash: String,
    ) -> Result<echo_backend::domain::entities::user::User, DomainError> {
        use echo_backend::domain::entities::user::User;

        let user = User::new_with_credentials(
            DisplayName::new(display_name),
            avatar_url.unwrap_or_else(|| "https://example.com/avatar.jpg".to_string()),
            password_hash,
        );

        let mut users = self.users.lock().unwrap();
        users.push(user.clone());

        Ok(user)
    }

    async fn update_refresh_token(
        &self,
        user_id: Uuid,
        refresh_token: Option<String>,
    ) -> Result<(), DomainError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
            user.refresh_token = refresh_token;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let mut users = self.users.lock().unwrap();
        users.retain(|u| u.id != id);
        Ok(())
    }
}

// CreatePostUseCase tests
#[tokio::test]
async fn test_create_post_success() {
    let mock_post_repo = Arc::new(MockPostRepository::new());
    let mock_user_repo = Arc::new(MockUserRepository::new());

    // Create a test user first
    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    let use_case = CreatePostUseCase::new(
        mock_post_repo.clone() as Arc<dyn PostRepository>,
        mock_user_repo.clone() as Arc<dyn UserRepository>,
    );

    let result = use_case
        .execute("Test post content".to_string(), None, user.id)
        .await;

    assert!(result.is_ok());
    assert!(result.unwrap());

    // Verify post was saved
    let posts = mock_post_repo.find_all_sync();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].content.value(), "Test post content");
}

#[tokio::test]
async fn test_create_post_with_image() {
    let mock_post_repo = Arc::new(MockPostRepository::new());
    let mock_user_repo = Arc::new(MockUserRepository::new());

    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    let use_case = CreatePostUseCase::new(
        mock_post_repo.clone() as Arc<dyn PostRepository>,
        mock_user_repo.clone() as Arc<dyn UserRepository>,
    );

    let result = use_case
        .execute(
            "Test post with image".to_string(),
            Some("https://example.com/image.jpg".to_string()),
            user.id,
        )
        .await;

    assert!(result.is_ok());

    let posts = mock_post_repo.find_all_sync();
    assert_eq!(posts.len(), 1);
    assert_eq!(
        posts[0].image_url,
        Some("https://example.com/image.jpg".to_string())
    );
}

#[tokio::test]
async fn test_create_post_with_empty_content() {
    let mock_post_repo = Arc::new(MockPostRepository::new());
    let mock_user_repo = Arc::new(MockUserRepository::new());

    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    let use_case = CreatePostUseCase::new(
        mock_post_repo as Arc<dyn PostRepository>,
        mock_user_repo as Arc<dyn UserRepository>,
    );

    let result = use_case.execute("".to_string(), None, user.id).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_post_with_too_long_content() {
    let mock_post_repo = Arc::new(MockPostRepository::new());
    let mock_user_repo = Arc::new(MockUserRepository::new());

    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    let use_case = CreatePostUseCase::new(
        mock_post_repo as Arc<dyn PostRepository>,
        mock_user_repo as Arc<dyn UserRepository>,
    );

    let long_content = "a".repeat(1001);
    let result = use_case.execute(long_content, None, user.id).await;

    assert!(result.is_err());
}

// GetTimelineUseCase tests
#[tokio::test]
async fn test_get_timeline_empty() {
    let mock_post_repo = Arc::new(MockPostRepository::new());
    let _mock_user_repo = Arc::new(MockUserRepository::new());

    let use_case = GetTimelineUseCase::new(
        mock_post_repo as Arc<dyn PostRepository>,
    );

    let result = use_case.execute(10, None).await;

    assert!(result.is_ok());
    let timeline = result.unwrap();
    assert_eq!(timeline.len(), 0);
}

#[tokio::test]
async fn test_get_timeline_with_posts() {
    let mock_user_repo = Arc::new(MockUserRepository::new());
    let mock_post_repo = Arc::new(MockPostRepository::with_users(mock_user_repo.users.clone()));

    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    // Create some posts first
    let create_use_case = CreatePostUseCase::new(
        mock_post_repo.clone() as Arc<dyn PostRepository>,
        mock_user_repo.clone() as Arc<dyn UserRepository>,
    );

    create_use_case
        .execute("Post 1".to_string(), None, user.id)
        .await
        .unwrap();
    create_use_case
        .execute("Post 2".to_string(), None, user.id)
        .await
        .unwrap();
    create_use_case
        .execute("Post 3".to_string(), None, user.id)
        .await
        .unwrap();

    let get_timeline_use_case = GetTimelineUseCase::new(
        mock_post_repo as Arc<dyn PostRepository>,
    );
    let result = get_timeline_use_case.execute(10, None).await;

    assert!(result.is_ok());
    let timeline = result.unwrap();
    assert_eq!(timeline.len(), 3);
}

#[tokio::test]
async fn test_get_timeline_respects_limit() {
    let mock_user_repo = Arc::new(MockUserRepository::new());
    let mock_post_repo = Arc::new(MockPostRepository::with_users(mock_user_repo.users.clone()));

    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    let create_use_case = CreatePostUseCase::new(
        mock_post_repo.clone() as Arc<dyn PostRepository>,
        mock_user_repo.clone() as Arc<dyn UserRepository>,
    );

    for i in 0..5 {
        create_use_case
            .execute(format!("Post {}", i), None, user.id)
            .await
            .unwrap();
    }

    let get_timeline_use_case = GetTimelineUseCase::new(
        mock_post_repo as Arc<dyn PostRepository>,
    );
    let result = get_timeline_use_case.execute(3, None).await;

    assert!(result.is_ok());
    let timeline = result.unwrap();
    assert_eq!(timeline.len(), 3);
}

// IncrementDisplayCountUseCase tests
#[tokio::test]
async fn test_increment_display_count_success() {
    let mock_post_repo = Arc::new(MockPostRepository::new());
    let mock_user_repo = Arc::new(MockUserRepository::new());

    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    // Create a post first
    let create_use_case = CreatePostUseCase::new(
        mock_post_repo.clone() as Arc<dyn PostRepository>,
        mock_user_repo.clone() as Arc<dyn UserRepository>,
    );
    create_use_case
        .execute("Test post".to_string(), None, user.id)
        .await
        .unwrap();

    let posts = mock_post_repo.find_all_sync();
    let post_id = posts[0].id;

    // Increment display count
    let increment_use_case =
        IncrementDisplayCountUseCase::new(mock_post_repo.clone() as Arc<dyn PostRepository>);
    let result = increment_use_case.execute(post_id).await;

    assert!(result.is_ok());
    assert!(result.unwrap());

    // Verify count was incremented
    let post = (mock_post_repo.clone() as Arc<dyn PostRepository>)
        .find_by_id(post_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(post.display_count.value(), 1);
}

#[tokio::test]
async fn test_increment_display_count_deletes_after_100() {
    let mock_post_repo = Arc::new(MockPostRepository::new());
    let mock_user_repo = Arc::new(MockUserRepository::new());

    let user = mock_user_repo
        .create_user("TestUser".to_string(), None)
        .await
        .unwrap();

    let create_use_case = CreatePostUseCase::new(
        mock_post_repo.clone() as Arc<dyn PostRepository>,
        mock_user_repo.clone() as Arc<dyn UserRepository>,
    );
    create_use_case
        .execute("Test post".to_string(), None, user.id)
        .await
        .unwrap();

    let posts = mock_post_repo.find_all_sync();
    let post_id = posts[0].id;

    let increment_use_case =
        IncrementDisplayCountUseCase::new(mock_post_repo.clone() as Arc<dyn PostRepository>);

    // Increment 100 times
    for _ in 0..100 {
        increment_use_case.execute(post_id).await.unwrap();
    }

    // Post should be deleted (removed from mock repo when expired)
    let post = (mock_post_repo.clone() as Arc<dyn PostRepository>)
        .find_by_id(post_id)
        .await
        .unwrap();
    assert!(post.is_none());
}

#[tokio::test]
async fn test_increment_display_count_not_found() {
    let mock_post_repo = Arc::new(MockPostRepository::new());

    let increment_use_case =
        IncrementDisplayCountUseCase::new(mock_post_repo as Arc<dyn PostRepository>);

    let result = increment_use_case.execute(Uuid::new_v4()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}
