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
