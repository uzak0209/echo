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
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-2xl font-bold">Timeline</h2>
        <Button variant="outline" onClick={() => refetch()}>
          Refresh
        </Button>
      </div>

      {posts.length === 0 ? (
        <div className="flex justify-center items-center min-h-[200px]">
          <p className="text-muted-foreground">No posts yet</p>
        </div>
      ) : (
        <div className="grid gap-4">
          {posts.map((post) => (
            <PostCard key={post.id} post={post} />
          ))}
        </div>
      )}
    </div>
  );
}
