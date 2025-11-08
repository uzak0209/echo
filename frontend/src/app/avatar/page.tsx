'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useQuery } from '@apollo/client';
import dynamic from 'next/dynamic';
import { useAuth } from '@/lib/auth-context';
import { useReaction } from '@/lib/reaction-context';
import { GET_USER_LATEST_REACTION } from '@/lib/graphql/queries';
import { Button } from '@/components/ui/button';
import { REACTION_EMOJIS, ReactionType } from '@/lib/types/reaction';

// Dynamic import to prevent SSR issues with Three.js
const MascotAvatar = dynamic(() => import('@/components/MascotAvatar').then(mod => ({ default: mod.MascotAvatar })), {
  ssr: false,
  loading: () => (
    <div className="w-full h-full flex items-center justify-center">
      <p className="text-muted-foreground">Loading your mascot...</p>
    </div>
  ),
});

export default function AvatarPage() {
  const { isAuthenticated, userId, displayName, avatarUrl } = useAuth();
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

  if (!isAuthenticated || !avatarUrl || !displayName) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  const emoji = getReactionEmoji(latestReaction);

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

          {/* 3D Mascot Avatar */}
          <div className="relative w-full max-w-md">
            <div className="w-full aspect-square bg-white/80 backdrop-blur-sm rounded-xl shadow-2xl overflow-hidden">
              <MascotAvatar userId={userId || ''} expression={latestReaction} />
            </div>

            {/* Reaction emoji overlay with animation */}
            {emoji && (
              <div
                className="absolute -bottom-4 -right-4 bg-white rounded-full shadow-xl p-6 text-6xl animate-bounce"
                style={{
                  animation: 'bounce 1s ease-in-out 3',
                }}
              >
                {emoji}
              </div>
            )}
          </div>

          {/* User info */}
          <div className="text-center space-y-2">
            <h2 className="text-2xl font-semibold">{displayName}</h2>
            {latestReaction ? (
              <p className="text-muted-foreground">
                最新のリアクション: <span className="font-semibold">{emoji}</span>
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
              リアクションを受け取ると、アバターの表情が変化します！
            </p>
          </div>
        </div>
      </div>

      <style jsx>{`
        @keyframes bounce {
          0%, 100% {
            transform: translateY(0) scale(1);
          }
          50% {
            transform: translateY(-20px) scale(1.1);
          }
        }
      `}</style>
    </div>
  );
}
