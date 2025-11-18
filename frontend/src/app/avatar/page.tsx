'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useQuery } from '@apollo/client';
import { useAuth } from '@/lib/auth-context';
import { useReaction } from '@/lib/reaction-context';
import { GET_USER_LATEST_REACTION } from '@/lib/graphql/queries';
import { Button } from '@/components/ui/button';
import { REACTION_EMOJIS, ReactionType } from '@/lib/types/reaction';
import { MascotAvatar } from '@/components/MascotAvatar';

export default function AvatarPage() {
  const { isAuthenticated, userId, displayName } = useAuth();
  const router = useRouter();
  const [latestReaction, setLatestReaction] = useState<string | null>(null);
  const { latestReaction: sseReaction } = useReaction();

  // Fetch initial latest reaction
  const { data: reactionData } = useQuery(GET_USER_LATEST_REACTION, {
    variables: { userId: userId || '' },
    skip: !userId,
  });

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  // Update from initial query
  useEffect(() => {
    if (reactionData?.userLatestReaction) {
      setLatestReaction(reactionData.userLatestReaction);
    }
  }, [reactionData]);

  // Update from SSE events
  useEffect(() => {
    if (sseReaction?.latest_reaction_for_author) {
      setLatestReaction(sseReaction.latest_reaction_for_author);
    }
  }, [sseReaction]);

  const getReactionEmoji = (reaction: string | null): string | null => {
    if (!reaction) return null;

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

  if (!isAuthenticated || !userId || !displayName) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-50 via-pink-50 to-blue-50 p-4">
      <div className="container mx-auto max-w-2xl min-h-screen flex flex-col justify-center items-center">
        {/* Back button */}
        <div className="absolute top-4 left-4">
          <Button variant="outline" onClick={() => router.push('/timeline')}>
            ← Back to Timeline
          </Button>
        </div>

        {/* Avatar display */}
        <div className="flex flex-col items-center gap-8">
          <h1 className="text-4xl font-bold text-center bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
            Your Mascot
          </h1>

          {/* Mascot Avatar */}
          <div className="w-96 h-96 border-8 border-white shadow-2xl rounded-lg overflow-hidden">
            <MascotAvatar userId={userId} expression={latestReaction} />
          </div>

          {/* User info */}
          <div className="text-center space-y-2">
            <h2 className="text-2xl font-semibold">{displayName}</h2>
            {latestReaction ? (
              <p className="text-muted-foreground">
                最新のリアクション: <span className="font-semibold">{latestReaction}</span>
              </p>
            ) : (
              <p className="text-muted-foreground">
                まだリアクションはありません
              </p>
            )}
          </div>

          {/* Expression info */}
          <div className="mt-4 p-6 bg-white/80 backdrop-blur-sm rounded-xl shadow-lg max-w-md">
            <h3 className="font-semibold mb-2 text-center">表情について</h3>
            <p className="text-sm text-muted-foreground text-center">
              あなたの投稿に対する最新のリアクションが表情として表示されます。
              リアクションを受け取ると、マスコットの表情が変化します！
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
