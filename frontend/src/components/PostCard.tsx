'use client';

import { useState } from 'react';
import { useMutation } from '@apollo/client';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { ADD_REACTION, REMOVE_REACTION } from '@/lib/graphql/mutations';
import { ReactionType, REACTION_EMOJIS } from '@/lib/types/reaction';
import { useAuth } from '@/lib/auth-context';

interface Post {
  id: string;
  content: string;
  imageUrl?: string | null;
  authorName: string;
  authorAvatar: string;
}

interface PostCardProps {
  post: Post;
}

export function PostCard({ post }: PostCardProps) {
  const { userId } = useAuth();
  const [selectedReaction, setSelectedReaction] = useState<ReactionType | null>(null);
  const [addReaction] = useMutation(ADD_REACTION);
  const [removeReaction] = useMutation(REMOVE_REACTION);

  const handleReactionClick = async (reactionType: ReactionType) => {
    if (!userId) {
      console.error('User not authenticated');
      return;
    }

    try {
      if (selectedReaction === reactionType) {
        // Remove reaction if clicking the same button
        await removeReaction({
          variables: {
            postId: post.id,
            reactionType,
          },
        });
        setSelectedReaction(null);
      } else {
        // Add new reaction (backend handles removing old one if exists)
        await addReaction({
          variables: {
            postId: post.id,
            reactionType,
          },
        });
        setSelectedReaction(reactionType);
      }
    } catch (error) {
      console.error('Failed to update reaction:', error);
    }
  };

  // Note: Display count is automatically incremented on the backend
  // when the timeline is fetched, so no need to increment here

  return (
    <Card className="w-full">
      <CardContent className="pt-6">
        <div className="flex items-center gap-3 mb-4">
          <img
            src={post.authorAvatar}
            alt={post.authorName}
            className="w-10 h-10 rounded-full object-cover"
          />
          <span className="font-semibold text-sm">{post.authorName}</span>
        </div>
        {post.imageUrl && (
          <img
            src={post.imageUrl}
            alt="Post"
            className="w-full h-48 object-cover rounded-md mb-4"
          />
        )}
        <p className="text-base mb-4">{post.content}</p>

        {/* Reaction Buttons */}
        <div className="flex gap-2 flex-wrap">
          {Object.values(ReactionType).map((reactionType) => (
            <Button
              key={reactionType}
              variant={selectedReaction === reactionType ? 'default' : 'outline'}
              size="sm"
              onClick={() => handleReactionClick(reactionType)}
              className="text-lg"
            >
              {REACTION_EMOJIS[reactionType]}
            </Button>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
