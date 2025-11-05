'use client';

import { useState } from 'react';
import { useMutation } from '@apollo/client';
import { CREATE_POST } from '@/lib/graphql/mutations';
import { GET_TIMELINE } from '@/lib/graphql/queries';
import { useAuth } from '@/lib/auth-context';
import { Button } from './ui/button';
import { Textarea } from './ui/textarea';
import { Card, CardContent, CardHeader, CardTitle } from './ui/card';

export function CreatePost() {
  const [content, setContent] = useState('');
  const { userId } = useAuth();
  const [createPost, { loading }] = useMutation(CREATE_POST, {
    refetchQueries: [{ query: GET_TIMELINE, variables: { limit: 10 } }],
  });

  const handleSubmit = async () => {
    if (!content.trim() || !userId) return;

    try {
      await createPost({
        variables: {
          content: content.trim(),
          imageUrl: null,
          userId: userId,
        },
      });

      // Clear the form
      setContent('');
    } catch (error) {
      console.error('Error creating post:', error);
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle>Create a Post</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <Textarea
          placeholder="What's on your mind? (Your post will disappear after 10 views)"
          value={content}
          onChange={(e) => setContent(e.target.value)}
          maxLength={1000}
          className="min-h-[120px]"
        />
        <div className="flex justify-between items-center">
          <p className="text-sm text-muted-foreground">
            {content.length}/1000 characters
          </p>
          <Button onClick={handleSubmit} disabled={loading || !content.trim()}>
            {loading ? 'Posting...' : 'Post'}
          </Button>
        </div>
        <p className="text-xs text-muted-foreground">
          Note: You won't be able to see your own post. It will be shown to others randomly.
        </p>
      </CardContent>
    </Card>
  );
}
