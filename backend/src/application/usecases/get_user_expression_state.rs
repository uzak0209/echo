use crate::{
    application::error::AppError,
    domain::{entities::ReactionType, repositories::ReactionRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserExpressionStateUseCase {
    reaction_repository: Arc<dyn ReactionRepository>,
}

/// User's 3D model expression state based on reactions received
#[derive(Debug, Clone)]
pub struct ExpressionState {
    /// Dominant expression type based on most received reactions
    pub dominant_expression: Option<ReactionType>,
    /// Intensity level (0.0 to 1.0) based on total reaction count
    pub intensity: f32,
    /// Breakdown of reaction counts by type
    pub reaction_counts: Vec<(ReactionType, i64)>,
    /// Total number of reactions received
    pub total_reactions: i64,
}

impl GetUserExpressionStateUseCase {
    pub fn new(reaction_repository: Arc<dyn ReactionRepository>) -> Self {
        Self {
            reaction_repository,
        }
    }

    /// Get expression state for a user based on cumulative reactions received
    pub async fn execute(&self, user_id: Uuid) -> Result<ExpressionState, AppError> {
        let reaction_counts = self
            .reaction_repository
            .get_user_received_reaction_counts(user_id)
            .await?;

        // Calculate total reactions
        let total_reactions: i64 = reaction_counts.iter().map(|(_, count)| count).sum();

        // Find dominant expression (most received reaction type)
        let dominant_expression = reaction_counts
            .iter()
            .max_by_key(|(_, count)| count)
            .map(|(reaction_type, _)| *reaction_type);

        // Calculate intensity (0.0 to 1.0 based on total reactions)
        // Cap at 100 reactions for max intensity
        let intensity = (total_reactions as f32 / 100.0).min(1.0);

        Ok(ExpressionState {
            dominant_expression,
            intensity,
            reaction_counts,
            total_reactions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_state_calculation() {
        let reaction_counts = vec![
            (ReactionType::Laugh, 15),
            (ReactionType::Empathy, 8),
            (ReactionType::Surprise, 3),
        ];

        let total: i64 = reaction_counts.iter().map(|(_, count)| count).sum();
        assert_eq!(total, 26);

        let dominant = reaction_counts
            .iter()
            .max_by_key(|(_, count)| count)
            .map(|(rt, _)| *rt);
        assert_eq!(dominant, Some(ReactionType::Laugh));

        let intensity = (total as f32 / 100.0).min(1.0);
        assert_eq!(intensity, 0.26);
    }
}
