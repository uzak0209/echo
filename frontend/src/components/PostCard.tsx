'use client';

import { Card, CardContent } from '@/components/ui/card';
import { useEffect } from 'react';
import { useMutation } from '@apollo/client';
import { INCREMENT_DISPLAY_COUNT } from '@/lib/graphql/mutations';

interface Post {
  id: number;
  content: string;
  imageUrl?: string | null;
}

interface PostCardProps {
  post: Post;
}

export function PostCard({ post }: PostCardProps) {
  const [incrementDisplayCount] = useMutation(INCREMENT_DISPLAY_COUNT);

  useEffect(() => {
    // Increment display count when post is viewed
    incrementDisplayCount({
      variables: { postId: post.id },
    });
  }, [post.id, incrementDisplayCount]);

  return (
    <Card className="w-full">
      <CardContent className="pt-6">
        {post.imageUrl && (
          <img
            src={post.imageUrl}
            alt="Post"
            className="w-full h-48 object-cover rounded-md mb-4"
          />
        )}
        <p className="text-base">{post.content}</p>
      </CardContent>
    </Card>
  );
}
