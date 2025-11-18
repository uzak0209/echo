'use client';

import { useQuery } from '@apollo/client';
import { GET_TIMELINE } from '@/lib/graphql/queries';
import { PostCard } from './PostCard';
import { Button } from './ui/button';

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

  const posts: Post[] = data?.timeline || [];

  console.log('Timeline data:', data, 'Posts:', posts);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center mb-2">
        <div className="flex items-center gap-3">
          <h2 className="text-2xl font-bold bg-gradient-to-r from-blue-400 to-pink-400 bg-clip-text text-transparent">
            フィード
          </h2>
        </div>
        <Button
          variant="ghost"
          onClick={() => refetch()}
          className="text-blue-500 hover:text-pink-500 hover:bg-blue-500/5 transition-all duration-300"
        >
          🔄 更新
        </Button>
      </div>

      {posts.length === 0 ? (
        <div className="flex flex-col justify-center items-center min-h-[400px] backdrop-blur-sm bg-card/20 rounded-lg border border-blue-500/20 p-12">
          <div className="text-6xl mb-4 opacity-30">✨</div>
          <p className="text-muted-foreground text-lg">まだ投稿がありません</p>
        </div>
      ) : (
        <div className="grid gap-8">
          {posts.map((post, index) => (
            <PostCard key={post.id} post={post} index={index} />
          ))}
        </div>
      )}
    </div>
  );
}
