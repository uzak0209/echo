'use client';

import { useEffect, useState, useCallback } from 'react';
import { useAuth } from '../auth-context';
import { useMutation } from '@apollo/client';
import { GENERATE_SSE_TOKEN } from '../graphql/mutations';

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
	const [generateSSEToken] = useMutation(GENERATE_SSE_TOKEN);

	useEffect(() => {
		console.log('[useReactionStream] isAuthenticated:', isAuthenticated);

		// 未ログインならSSEを張らない
		if (!isAuthenticated) return;

		let eventSource: EventSource | null = null;

		// 非同期処理をラップする
		(async () => {
			try {
				const { data } = await generateSSEToken();
				const sseToken = data?.generateSseToken;
				if (!sseToken) {
					console.error('Failed to obtain SSE token');
					setError('トークンの生成に失敗しました');
					return;
				}

				const url = `http://localhost:8000/api/reactions/events?token=${sseToken}`;
				eventSource = new EventSource(url);

				eventSource.onopen = () => {
					console.log('[useReactionStream] SSE connection opened');
					setIsConnected(true);
					setError(null);
				};

				eventSource.onmessage = (event) => {
					const parsed = JSON.parse(event.data);
					setLatestReaction(parsed);
				};

				eventSource.onerror = (err) => {
					console.error('[useReactionStream] SSE error:', err);
					setIsConnected(false);
					setError('SSE 接続エラーが発生しました');
					eventSource?.close();
				};
			} catch (err) {
				console.error('Error initializing SSE:', err);
				setError('SSE 初期化中にエラー');
			}
		})();

		return () => {
			console.log('[useReactionStream] Cleaning up...');
			if (eventSource) eventSource.close();
		};
	}, [isAuthenticated, generateSSEToken]);

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
