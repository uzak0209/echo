'use client';

import React, { createContext, useContext } from 'react';
import { useReactionStream } from '@/lib/hooks/useReactionStream';

type ReactionContextType = ReturnType<typeof useReactionStream>;

const ReactionContext = createContext<ReactionContextType | undefined>(undefined);

export function ReactionProvider({ children }: { children: React.ReactNode }) {
  const reaction = useReactionStream();

  return (
    <ReactionContext.Provider value={reaction}>
      {children}
    </ReactionContext.Provider>
  );
}

export function useReaction() {
  const ctx = useContext(ReactionContext);
  if (!ctx) throw new Error('useReaction must be used within a ReactionProvider');
  return ctx;
}
