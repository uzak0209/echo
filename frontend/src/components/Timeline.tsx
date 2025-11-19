'use client';

import { useQuery } from '@apollo/client';
import { GET_TIMELINE } from '@/lib/graphql/queries';
import { PostCard } from './PostCard';
import { Button } from './ui/button';
import { usePostStream } from '@/lib/hooks/usePostStream';
import { useEffect, useState } from 'react';

interface Post {
  id: string;
  content: string;
  imageUrl?: string | null;
  authorName: string;
  authorAvatar: string;
}

export function Timeline() {
  const { data, loading, error, refetch } = useQuery(GET_TIMELINE, {
    variables: { limit: 10 },
    fetchPolicy: 'network-only', // Always fetch from server, not cache
  });

  // リアルタイム投稿ストリーム
  const { latestPost, isConnected } = usePostStream();
  const [realtimePosts, setRealtimePosts] = useState<Post[]>([]);

  // 新規投稿が来たらリアルタイムで追加
  useEffect(() => {
    if (latestPost) {
      console.log('[Timeline] New post received:', latestPost);

      // SSEイベントからPost型に変換
      const newPost: Post = {
        id: latestPost.post_id,
        content: latestPost.content,
        imageUrl: latestPost.image_url,
        authorName: latestPost.author_name,
        authorAvatar: latestPost.author_avatar,
      };

      // 重複チェック（既に存在する投稿は追加しない）
      setRealtimePosts((prev) => {
        if (prev.some((p) => p.id === newPost.id)) {
          return prev;
        }
        return [newPost, ...prev];
      });
    }
  }, [latestPost]);

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-[200px]">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  if (error) {
    console.error('Timeline error:', error);
    return (
      <div className="flex justify-center items-center min-h-[200px]">
        <p className="text-destructive">Error loading timeline: {error.message}</p>
      </div>
    );
  }

  const serverPosts: Post[] = data?.timeline || [];

  // リアルタイム投稿 + サーバーから取得した投稿を統合
  // 重複を除外
  const allPosts = [...realtimePosts];
  const seenIds = new Set(realtimePosts.map((p) => p.id));

  for (const post of serverPosts) {
    if (!seenIds.has(post.id)) {
      allPosts.push(post);
      seenIds.add(post.id);
    }
  }

  console.log('Timeline - Realtime posts:', realtimePosts.length, 'Server posts:', serverPosts.length, 'Total:', allPosts.length);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center mb-2">
        <div className="flex items-center gap-3">
          <h2 className="text-2xl font-bold bg-gradient-to-r from-blue-400 to-pink-400 bg-clip-text text-transparent">
            フィード
          </h2>
          {/* SSE接続ステータス */}
          {isConnected && (
            <span className="text-xs text-green-500 flex items-center gap-1">
              <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
              リアルタイム
            </span>
          )}
        </div>
        <Button
          variant="ghost"
          onClick={() => refetch()}
          className="text-blue-500 hover:text-pink-500 hover:bg-blue-500/5 transition-all duration-300"
        >
          🔄 更新
        </Button>
      </div>

      {allPosts.length === 0 ? (
        <div className="flex flex-col justify-center items-center min-h-[400px] backdrop-blur-sm bg-card/20 rounded-lg border border-blue-500/20 p-12">
          <div className="text-6xl mb-4 opacity-30">✨</div>
          <p className="text-muted-foreground text-lg">まだ投稿がありません</p>
        </div>
      ) : (
        <div className="grid gap-8">
          {allPosts.map((post, index) => (
            <PostCard key={post.id} post={post} index={index} />
          ))}
        </div>
      )}
    </div>
  );
}
