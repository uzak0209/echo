import { gql } from '@apollo/client';

export const GET_TIMELINE = gql`
  query GetTimeline($limit: Int!) {
    timeline(limit: $limit) {
      id
      content
      imageUrl
      authorName
      authorAvatar
    }
  }
`;

export const GET_USER_LATEST_REACTION = gql`
  query GetUserLatestReaction($userId: String!) {
    userLatestReaction(userId: $userId)
  }
`;

export const GET_MY_POSTS = gql`
  query GetMyPosts {
    myPosts {
      id
      content
      imageUrl
      authorName
      authorAvatar
    }
  }
`;
