'use client';

import { useEffect, useState, useCallback } from 'react';
import { useAuth } from '../auth-context';

export interface ReactionEvent {
  post_id: string;
  reactor_user_id: string;
  reaction_type: string;
  timestamp: number;
  latest_reaction_for_author: string;
}

/**
 * SSE Hook for real-time reaction updates
 * Automatically connects when user is authenticated
 * Uses refresh_token from cookie for authentication
 */
export function useReactionStream() {
  const { isAuthenticated } = useAuth();
  const [latestReaction, setLatestReaction] = useState<ReactionEvent | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    console.log('[useReactionStream] isAuthenticated:', isAuthenticated);

    if (!isAuthenticated) {
      console.log('[useReactionStream] Not authenticated, skipping SSE connection');
      setIsConnected(false);
      return;
    }

    console.log('[useReactionStream] Attempting to connect to SSE...');

    // Create EventSource connection
    // refresh_token cookie is automatically sent by the browser
    const eventSource = new EventSource('http://localhost:8000/api/reactions/events', {
      withCredentials: true,
    });

    eventSource.onopen = () => {
      console.log('SSE connection established');
      setIsConnected(true);
      setError(null);
    };

    eventSource.onmessage = (event) => {
      try {
        const data: ReactionEvent = JSON.parse(event.data);
        console.log('Received reaction event:', data);
        setLatestReaction(data);
      } catch (err) {
        console.error('Failed to parse SSE event:', err);
      }
    };

    eventSource.onerror = (err) => {
      console.error('SSE connection error:', err);
      setIsConnected(false);
      setError('SSE connection failed');
      eventSource.close();
    };

    // Cleanup on unmount
    return () => {
      console.log('Closing SSE connection');
      eventSource.close();
    };
  }, [isAuthenticated]);

  const clearLatestReaction = useCallback(() => {
    setLatestReaction(null);
  }, []);

  return {
    latestReaction,
    isConnected,
    error,
    clearLatestReaction,
  };
}
