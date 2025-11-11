use crate::domain::entities::ReactionType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// リアクションイベントの種類
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionEvent {
    pub post_id: String,
    pub reactor_user_id: String,
    pub reaction_type: String,
    pub timestamp: i64,
    // 投稿者のアバターに表示する最新リアクション（このイベントのリアクション）
    pub latest_reaction_for_author: String,
}

impl ReactionEvent {
    pub fn new(post_id: Uuid, reactor_user_id: Uuid, reaction_type: ReactionType) -> Self {
        let reaction_str = reaction_type.as_str().to_string();
        Self {
            post_id: post_id.to_string(),
            reactor_user_id: reactor_user_id.to_string(),
            reaction_type: reaction_str.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            latest_reaction_for_author: reaction_str, // 最新のリアクションはこれ
        }
    }
}

/// SSEストリーム管理
/// 各投稿者ごとに専用のbroadcastチャンネルを持つ
pub struct ReactionStreamManager {
    // user_id -> broadcast sender
    streams: Arc<RwLock<HashMap<Uuid, broadcast::Sender<ReactionEvent>>>>,
}

impl ReactionStreamManager {
    pub fn new() -> Self {
        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// ユーザーのストリームを取得または作成
    pub async fn subscribe(&self, user_id: Uuid) -> broadcast::Receiver<ReactionEvent> {
        let mut streams = self.streams.write().await;

        let sender = streams.entry(user_id).or_insert_with(|| {
            // バッファサイズ100のチャンネルを作成
            let (tx, _) = broadcast::channel(100);
            tx
        });

        sender.subscribe()
    }

    /// リアクションイベントを配信
    /// post_idから投稿者を特定し、その投稿者のストリームに配信
    pub async fn broadcast_reaction(
        &self,
        post_author_id: Uuid,
        post_id: Uuid,
        reactor_user_id: Uuid,
        reaction_type: ReactionType,
    ) -> Result<(), String> {
        let streams = self.streams.read().await;

        if let Some(sender) = streams.get(&post_author_id) {
            let event = ReactionEvent::new(post_id, reactor_user_id, reaction_type);

            // イベントを配信（受信者がいない場合はエラーを無視）
            let _ = sender.send(event);
        }

        Ok(())
    }
}

impl Default for ReactionStreamManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscribe_and_broadcast() {
        let manager = ReactionStreamManager::new();
        let user_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        let reactor_id = Uuid::new_v4();

        // Subscribe
        let mut receiver = manager.subscribe(user_id).await;

        // Broadcast
        manager
            .broadcast_reaction(user_id, post_id, reactor_id, ReactionType::Laugh)
            .await
            .unwrap();

        // Receive
        let event = receiver.recv().await.unwrap();
        assert_eq!(event.post_id, post_id.to_string());
        assert_eq!(event.reaction_type, "laugh");
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let manager = ReactionStreamManager::new();
        let user_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        let reactor_id = Uuid::new_v4();

        // Multiple subscribers
        let mut receiver1 = manager.subscribe(user_id).await;
        let mut receiver2 = manager.subscribe(user_id).await;

        // Broadcast
        manager
            .broadcast_reaction(user_id, post_id, reactor_id, ReactionType::Surprise)
            .await
            .unwrap();

        // Both receive
        let event1 = receiver1.recv().await.unwrap();
        let event2 = receiver2.recv().await.unwrap();

        assert_eq!(event1.post_id, event2.post_id);
        assert_eq!(event1.reaction_type, "surprise");
    }
}
