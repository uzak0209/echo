import { gql } from '@apollo/client';

export const CREATE_POST = gql`
  mutation CreatePost($content: String!, $imageUrl: String) {
    createPost(content: $content, imageUrl: $imageUrl)
  }
`;

export const INCREMENT_DISPLAY_COUNT = gql`
  mutation IncrementDisplayCount($postId: Int!) {
    incrementDisplayCount(postId: $postId)
  }
`;
