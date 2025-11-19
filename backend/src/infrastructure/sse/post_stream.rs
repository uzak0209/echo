use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// 投稿イベントの種類
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PostEvent {
    /// 新規投稿
    NewPost {
        post_id: String,
        user_id: String,
        content: String,
        image_url: Option<String>,
        display_count: i32,
        created_at: i64,
        author_name: String,
        author_avatar: String,
    },
    /// 表示回数更新
    DisplayCountUpdated {
        post_id: String,
        display_count: i32,
    },
    /// 投稿削除（100回表示達成）
    PostDeleted {
        post_id: String,
    },
}

impl PostEvent {
    pub fn new_post(
        post_id: Uuid,
        user_id: Uuid,
        content: String,
        image_url: Option<String>,
        display_count: i32,
        created_at: chrono::DateTime<chrono::Utc>,
        author_name: String,
        author_avatar: String,
    ) -> Self {
        Self::NewPost {
            post_id: post_id.to_string(),
            user_id: user_id.to_string(),
            content,
            image_url,
            display_count,
            created_at: created_at.timestamp(),
            author_name,
            author_avatar,
        }
    }

    pub fn display_count_updated(post_id: Uuid, display_count: i32) -> Self {
        Self::DisplayCountUpdated {
            post_id: post_id.to_string(),
            display_count,
        }
    }

    pub fn post_deleted(post_id: Uuid) -> Self {
        Self::PostDeleted {
            post_id: post_id.to_string(),
        }
    }
}

/// SSE投稿ストリーム管理
/// 全ユーザーに対してグローバルにブロードキャストする
pub struct PostStreamManager {
    // グローバルなbroadcast sender
    sender: Arc<RwLock<broadcast::Sender<PostEvent>>>,
}

impl PostStreamManager {
    pub fn new() -> Self {
        // バッファサイズ1000のチャンネルを作成
        let (tx, _) = broadcast::channel(1000);
        Self {
            sender: Arc::new(RwLock::new(tx)),
        }
    }

    /// グローバルストリームにサブスクライブ
    pub async fn subscribe(&self) -> broadcast::Receiver<PostEvent> {
        let sender = self.sender.read().await;
        sender.subscribe()
    }

    /// 投稿イベントを全ユーザーにブロードキャスト
    pub async fn broadcast(&self, event: PostEvent) -> Result<(), String> {
        let sender = self.sender.read().await;

        // イベントを配信（受信者がいない場合はエラーを無視）
        let _ = sender.send(event);

        Ok(())
    }
}

impl Default for PostStreamManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscribe_and_broadcast() {
        let manager = PostStreamManager::new();
        let post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Subscribe
        let mut receiver = manager.subscribe().await;

        // Broadcast new post
        let event = PostEvent::new_post(
            post_id,
            user_id,
            "Test content".to_string(),
            None,
            0,
            chrono::Utc::now(),
            "TestUser".to_string(),
            "https://example.com/avatar.png".to_string(),
        );
        manager.broadcast(event.clone()).await.unwrap();

        // Receive
        let received = receiver.recv().await.unwrap();
        match received {
            PostEvent::NewPost { post_id: id, .. } => {
                assert_eq!(id, post_id.to_string());
            }
            _ => panic!("Expected NewPost event"),
        }
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let manager = PostStreamManager::new();
        let post_id = Uuid::new_v4();

        // Multiple subscribers
        let mut receiver1 = manager.subscribe().await;
        let mut receiver2 = manager.subscribe().await;

        // Broadcast
        let event = PostEvent::post_deleted(post_id);
        manager.broadcast(event).await.unwrap();

        // Both receive
        let event1 = receiver1.recv().await.unwrap();
        let event2 = receiver2.recv().await.unwrap();

        match (event1, event2) {
            (PostEvent::PostDeleted { post_id: id1 }, PostEvent::PostDeleted { post_id: id2 }) => {
                assert_eq!(id1, id2);
                assert_eq!(id1, post_id.to_string());
            }
            _ => panic!("Expected PostDeleted events"),
        }
    }
}
