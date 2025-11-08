use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Reaction types for posts
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ReactionType {
    Surprise, // 驚き
    Empathy,  // 共感
    Laugh,    // 笑い
    Sad,      // 悲しい
    Confused, // 首を傾げる
}

impl ReactionType {
    pub fn as_str(&self) -> &str {
        match self {
            ReactionType::Surprise => "surprise",
            ReactionType::Empathy => "empathy",
            ReactionType::Laugh => "laugh",
            ReactionType::Sad => "sad",
            ReactionType::Confused => "confused",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "surprise" => Some(ReactionType::Surprise),
            "empathy" => Some(ReactionType::Empathy),
            "laugh" => Some(ReactionType::Laugh),
            "sad" => Some(ReactionType::Sad),
            "confused" => Some(ReactionType::Confused),
            _ => None,
        }
    }

    /// Get all reaction types
    pub fn all() -> Vec<ReactionType> {
        vec![
            ReactionType::Surprise,
            ReactionType::Empathy,
            ReactionType::Laugh,
            ReactionType::Sad,
            ReactionType::Confused,
        ]
    }
}

/// Reaction domain entity
#[derive(Debug, Copy, Clone)]
pub struct Reaction {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub reaction_type: ReactionType,
    pub created_at: DateTime<Utc>,
}

impl Reaction {
    pub fn new(post_id: Uuid, user_id: Uuid, reaction_type: ReactionType) -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id,
            user_id,
            reaction_type,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_type_as_str() {
        assert_eq!(ReactionType::Surprise.as_str(), "surprise");
        assert_eq!(ReactionType::Empathy.as_str(), "empathy");
        assert_eq!(ReactionType::Laugh.as_str(), "laugh");
        assert_eq!(ReactionType::Sad.as_str(), "sad");
        assert_eq!(ReactionType::Confused.as_str(), "confused");
    }

    #[test]
    fn test_reaction_type_from_str() {
        assert_eq!(
            ReactionType::from_str("surprise"),
            Some(ReactionType::Surprise)
        );
        assert_eq!(
            ReactionType::from_str("empathy"),
            Some(ReactionType::Empathy)
        );
        assert_eq!(ReactionType::from_str("laugh"), Some(ReactionType::Laugh));
        assert_eq!(ReactionType::from_str("sad"), Some(ReactionType::Sad));
        assert_eq!(
            ReactionType::from_str("confused"),
            Some(ReactionType::Confused)
        );
        assert_eq!(ReactionType::from_str("invalid"), None);
    }

    #[test]
    fn test_new_reaction() {
        let post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let reaction = Reaction::new(post_id, user_id, ReactionType::Laugh);

        assert_eq!(reaction.post_id, post_id);
        assert_eq!(reaction.user_id, user_id);
        assert_eq!(reaction.reaction_type, ReactionType::Laugh);
    }
}
