'use client';

import { useEffect, useState } from 'react';
import { useReaction } from '@/lib/reaction-context';
import { Card, CardContent } from '@/components/ui/card';
import { REACTION_EMOJIS, ReactionType } from '@/lib/types/reaction';

/**
 * Displays real-time reaction notifications via SSE
 * Shows a toast-like notification when a reaction is received
 */
export function ReactionNotification() {
  const { latestReaction, isConnected, error } = useReaction();
  const [showNotification, setShowNotification] = useState(false);

  useEffect(() => {
    if (latestReaction) {
      setShowNotification(true);

      // Auto-hide after 5 seconds
      const timer = setTimeout(() => {
        setShowNotification(false);
      }, 5000);

      return () => clearTimeout(timer);
    }
  }, [latestReaction]);

  const getReactionEmoji = (reactionType: string): string => {
    const reactionMap: Record<string, ReactionType> = {
      'surprise': ReactionType.SURPRISE,
      'empathy': ReactionType.EMPATHY,
      'laugh': ReactionType.LAUGH,
      'sad': ReactionType.SAD,
      'confused': ReactionType.CONFUSED,
    };

    const type = reactionMap[reactionType.toLowerCase()];
    return type ? REACTION_EMOJIS[type] : '👍';
  };

  if (error) {
    console.error('SSE Error:', error);
  }

  return (
    <>
      {/* Connection Status Indicator (optional, for debugging) */}
      <div className="fixed top-4 left-4 text-xs text-muted-foreground">
        {isConnected ? '🟢 リアルタイム接続中' : '🔴 未接続'}
      </div>

      {/* Notification Toast */}
      {showNotification && latestReaction && (
        <div className="fixed top-4 right-4 z-50 animate-in slide-in-from-top-5">
          <Card className="w-80 shadow-lg">
            <CardContent className="pt-6">
              <div className="flex items-center gap-3">
                <div className="text-4xl">
                  {getReactionEmoji(latestReaction.reaction_type)}
                </div>
                <div className="flex-1">
                  <p className="font-semibold">新しいリアクション！</p>
                  <p className="text-sm text-muted-foreground">
                    あなたの投稿にリアクションがつきました
                  </p>
                </div>
                <button
                  onClick={() => setShowNotification(false)}
                  className="text-muted-foreground hover:text-foreground"
                  aria-label="Close"
                >
                  ✕
                </button>
              </div>
            </CardContent>
          </Card>
        </div>
      )}
    </>
  );
}
