import { gql } from '@apollo/client';

export const GENERATE_SSE_TOKEN = gql`
    mutation GenerateSseToken {
      generateSseToken
    }
`;

export const SIGNUP = gql`
  mutation Signup($username: String!, $password: String!, $avatarUrl: String) {
    signup(username: $username, password: $password, avatarUrl: $avatarUrl) {
      accessToken
      userId
    }
  }
`;

export const LOGIN = gql`
  mutation Login($username: String!, $password: String!) {
    login(username: $username, password: $password) {
      accessToken
      userId
    }
  }
`;

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

export const LOGOUT = gql`
  mutation Logout {
    logout
  }
`;

export const CREATE_POST = gql`
  mutation CreatePost($input: CreatePostInput!) {
    createPost(input: $input)
  }
`;

export const INCREMENT_DISPLAY_COUNT = gql`
  mutation IncrementDisplayCount($postId: String!) {
    incrementDisplayCount(postId: $postId)
  }
`;

export const ADD_REACTION = gql`
  mutation AddReaction($postId: String!, $reactionType: ReactionTypeGql!) {
    addReaction(postId: $postId, reactionType: $reactionType)
  }
`;

export const REMOVE_REACTION = gql`
  mutation RemoveReaction($postId: String!, $reactionType: ReactionTypeGql!) {
    removeReaction(postId: $postId, reactionType: $reactionType)
  }
`;

export const UPDATE_AVATAR = gql`
  mutation UpdateAvatar($avatarUrl: String!) {
    updateAvatar(avatarUrl: $avatarUrl)
  }
`;
