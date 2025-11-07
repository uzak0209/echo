'use client';

import { REACTION_EMOJIS, ReactionType } from '@/lib/types/reaction';

interface UserAvatarProps {
  avatarUrl: string;
  displayName: string;
  latestReaction?: string | null;
  size?: 'sm' | 'md' | 'lg';
}

const sizeClasses = {
  sm: 'w-10 h-10',
  md: 'w-16 h-16',
  lg: 'w-24 h-24',
};

const emojiSizeClasses = {
  sm: 'text-xs',
  md: 'text-sm',
  lg: 'text-base',
};

/**
 * UserAvatar component with reaction expression overlay
 * Shows the latest reaction received on any of the user's posts
 */
export function UserAvatar({
  avatarUrl,
  displayName,
  latestReaction,
  size = 'md'
}: UserAvatarProps) {
  const getReactionEmoji = (reaction: string | null | undefined): string | null => {
    if (!reaction) return null;

    // Convert backend string format (lowercase) to frontend enum
    const reactionMap: Record<string, ReactionType> = {
      'surprise': ReactionType.SURPRISE,
      'empathy': ReactionType.EMPATHY,
      'laugh': ReactionType.LAUGH,
      'sad': ReactionType.SAD,
      'confused': ReactionType.CONFUSED,
    };

    const reactionType = reactionMap[reaction.toLowerCase()];
    return reactionType ? REACTION_EMOJIS[reactionType] : null;
  };

  const emoji = getReactionEmoji(latestReaction);

  return (
    <div className="relative inline-block">
      <img
        src={avatarUrl}
        alt={displayName}
        className={`${sizeClasses[size]} rounded-full object-cover`}
      />
      {emoji && (
        <div
          className={`absolute -bottom-1 -right-1 bg-white rounded-full shadow-md p-1 ${emojiSizeClasses[size]}`}
          title="最新のリアクション"
        >
          {emoji}
        </div>
      )}
    </div>
  );
}
