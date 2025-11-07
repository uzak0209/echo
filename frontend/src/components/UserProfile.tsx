'use client';

import { useEffect, useState } from 'react';
import { useQuery } from '@apollo/client';
import { useAuth } from '@/lib/auth-context';
import { useReactionStream } from '@/lib/hooks/useReactionStream';
import { GET_USER_LATEST_REACTION } from '@/lib/graphql/queries';
import { UserAvatar } from './UserAvatar';
import { Card, CardContent } from './ui/card';

/**
 * User profile component showing avatar with latest reaction expression
 * Updates in real-time via SSE when new reactions are received
 */
export function UserProfile() {
  const { userId } = useAuth();
  const { latestReaction: sseReaction } = useReactionStream();
  const [latestReaction, setLatestReaction] = useState<string | null>(null);

  // Fetch initial latest reaction
  const { data } = useQuery(GET_USER_LATEST_REACTION, {
    variables: { userId: userId || '' },
    skip: !userId,
  });

  // Update from initial query
  useEffect(() => {
    if (data?.userLatestReaction) {
      setLatestReaction(data.userLatestReaction);
    }
  }, [data]);

  // Update from SSE events
  useEffect(() => {
    if (sseReaction?.latest_reaction_for_author) {
      setLatestReaction(sseReaction.latest_reaction_for_author);
    }
  }, [sseReaction]);

  if (!userId) {
    return null;
  }

  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex items-center gap-4">
          <UserAvatar
            avatarUrl="https://api.dicebear.com/7.x/avataaars/svg?seed=user"
            displayName="あなた"
            latestReaction={latestReaction}
            size="lg"
          />
          <div>
            <h3 className="font-semibold text-lg">あなたのアバター</h3>
            <p className="text-sm text-muted-foreground">
              {latestReaction
                ? '最新のリアクションが表示されています'
                : 'まだリアクションはありません'}
            </p>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
