'use client';

import { useEffect, useState, useCallback } from 'react';
import { useAuth } from '../auth-context';
import { useMutation } from '@apollo/client';
import { GENERATE_SSE_TOKEN } from '../graphql/mutations';

export interface PostEventNewPost {
	type: 'new_post';
	post_id: string;
	user_id: string;
	content: string;
	image_url: string | null;
	display_count: number;
	created_at: number;
	author_name: string;
	author_avatar: string;
}

export interface PostEventDisplayCountUpdated {
	type: 'display_count_updated';
	post_id: string;
	display_count: number;
}

export interface PostEventPostDeleted {
	type: 'post_deleted';
	post_id: string;
}

export type PostEvent =
	| PostEventNewPost
	| PostEventDisplayCountUpdated
	| PostEventPostDeleted;

/**
 * SSE Hook for real-time post updates
 * Automatically connects when user is authenticated
 * Uses SSE token for authentication
 */
export function usePostStream() {
	const { isAuthenticated } = useAuth();
	const [latestPost, setLatestPost] = useState<PostEventNewPost | null>(null);
	const [isConnected, setIsConnected] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [generateSSEToken] = useMutation(GENERATE_SSE_TOKEN);

	useEffect(() => {
		console.log('[usePostStream] isAuthenticated:', isAuthenticated);

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

				const url = `http://localhost:8000/api/posts/events?token=${sseToken}`;
				eventSource = new EventSource(url);

				eventSource.onopen = () => {
					console.log('[usePostStream] SSE connection opened');
					setIsConnected(true);
					setError(null);
				};

				eventSource.onmessage = (event) => {
					const parsed: PostEvent = JSON.parse(event.data);
					console.log('[usePostStream] Received event:', parsed);

					// 新規投稿イベントのみlatestPostにセット
					if (parsed.type === 'new_post') {
						setLatestPost(parsed);
					}
				};

				eventSource.onerror = (err) => {
					console.error('[usePostStream] SSE error:', err);
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
			console.log('[usePostStream] Cleaning up...');
			if (eventSource) eventSource.close();
		};
	}, [isAuthenticated, generateSSEToken]);

	const clearLatestPost = useCallback(() => {
		setLatestPost(null);
	}, []);

	return {
		latestPost,
		isConnected,
		error,
		clearLatestPost,
	};
}
