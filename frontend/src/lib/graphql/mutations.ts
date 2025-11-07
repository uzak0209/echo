import { gql } from '@apollo/client';

export const CREATE_USER = gql`
  mutation CreateUser($displayName: String!, $avatarUrl: String) {
    createUser(displayName: $displayName, avatarUrl: $avatarUrl) {
      accessToken
      userId
    }
  }
`;

export const REFRESH_TOKEN = gql`
  mutation RefreshToken {
    refreshToken {
      accessToken
    }
  }
`;

export const CREATE_POST = gql`
  mutation CreatePost($content: String!, $imageUrl: String, $userId: String!) {
    createPost(content: $content, imageUrl: $imageUrl, userId: $userId)
  }
`;

export const INCREMENT_DISPLAY_COUNT = gql`
  mutation IncrementDisplayCount($postId: String!) {
    incrementDisplayCount(postId: $postId)
  }
`;

export const ADD_REACTION = gql`
  mutation AddReaction($postId: String!, $userId: String!, $reactionType: ReactionTypeGql!) {
    addReaction(postId: $postId, userId: $userId, reactionType: $reactionType)
  }
`;

export const REMOVE_REACTION = gql`
  mutation RemoveReaction($postId: String!, $userId: String!, $reactionType: ReactionTypeGql!) {
    removeReaction(postId: $postId, userId: $userId, reactionType: $reactionType)
  }
`;
