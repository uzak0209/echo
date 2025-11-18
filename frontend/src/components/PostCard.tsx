'use client';

import { useState } from 'react';
import { useMutation } from '@apollo/client';
import { motion } from 'framer-motion';
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
  index: number;
}

export function PostCard({ post, index }: PostCardProps) {
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

  // 偶数は左から、奇数は右から
  const isFromLeft = index % 2 === 0;

  return (
    <motion.div
      className="w-full relative"
      initial={{
        opacity: 0,
        x: isFromLeft ? -150 : 150,
      }}
      animate={{
        opacity: 1,
        x: 0,
      }}
      transition={{
        duration: 0.8,
        delay: index * 0.2,
        ease: [0.25, 0.46, 0.45, 0.94],
      }}
    >
      {/* Speech bubble tail - 左右に配置 */}
      <div
        className={`absolute top-1/2 -translate-y-1/2 w-4 h-4 bg-card backdrop-blur-sm rotate-45 border-blue-500/30 ${
          isFromLeft
            ? '-left-2 border-l-2 border-b-2'
            : '-right-2 border-r-2 border-t-2'
        }`}
      ></div>

      <Card className="relative bg-card border border-blue-500/10 hover:border-blue-500/30 transition-all duration-300 hover:shadow-md overflow-hidden group rounded-xl">
        <CardContent className="pt-6 relative">
          {/* Author section */}
          <div className="flex items-center gap-3 mb-4">
            <img
              src={post.authorAvatar}
              alt={post.authorName}
              className="w-10 h-10 rounded-full object-cover border border-blue-500/20"
            />
            <span className="font-medium text-sm text-foreground/80">
              {post.authorName}
            </span>
          </div>

          {/* Post image */}
          {post.imageUrl && (
            <div className="relative mb-4 overflow-hidden rounded-lg">
              <img
                src={post.imageUrl}
                alt="Post"
                className="w-full h-64 object-cover"
              />
            </div>
          )}

          {/* Post content */}
          <p className="text-base mb-4 leading-relaxed text-foreground/90">
            {post.content}
          </p>

          {/* Reaction Buttons */}
          <div className="flex gap-2 flex-wrap pt-3 border-t border-border/50">
            {Object.values(ReactionType).map((reactionType) => {
              const isSelected = selectedReaction === reactionType;
              return (
                <Button
                  key={reactionType}
                  variant={isSelected ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => handleReactionClick(reactionType)}
                  className={`text-base transition-all duration-300 ${
                    isSelected
                      ? 'bg-gradient-to-r from-blue-500 to-pink-500 border-none'
                      : 'border-border hover:border-blue-500/50 hover:bg-blue-500/5'
                  }`}
                >
                  {REACTION_EMOJIS[reactionType]}
                </Button>
              );
            })}
          </div>
        </CardContent>
      </Card>
    </motion.div>
  );
}
