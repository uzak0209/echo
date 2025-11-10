'use client';

import { useState } from 'react';
import { useMutation } from '@apollo/client';
import { CREATE_POST } from '@/lib/graphql/mutations';
import { GET_TIMELINE } from '@/lib/graphql/queries';
import { useAuth } from '@/lib/auth-context';
import { Button } from './ui/button';
import { Textarea } from './ui/textarea';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from './ui/dialog';
import { Plus } from 'lucide-react';

export function CreatePost() {
  const [content, setContent] = useState('');
  const [open, setOpen] = useState(false);
  const { userId } = useAuth();
  const [createPost, { loading }] = useMutation(CREATE_POST, {
    refetchQueries: [{ query: GET_TIMELINE, variables: { limit: 10 } }],
  });

  const handleSubmit = async () => {
    if (!content.trim() || !userId) return;

    try {
      await createPost({
        variables: {
          input: {
            content: content.trim(),
            imageUrl: null,
          },
        },
      });

      // Clear the form and close dialog
      setContent('');
      setOpen(false);
    } catch (error) {
      console.error('Error creating post:', error);
    }
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button size="lg" className="gap-2">
          <Plus className="h-5 w-5" />
          Post
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[525px]">
        <DialogHeader>
          <DialogTitle>Create a Post</DialogTitle>
          <DialogDescription>
            Share your thoughts. Your post will disappear after 10 views.
          </DialogDescription>
        </DialogHeader>
        <div className="space-y-4 pt-4">
          <Textarea
            placeholder="What's on your mind?"
            value={content}
            onChange={(e) => setContent(e.target.value)}
            maxLength={1000}
            className="min-h-[150px]"
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
        </div>
      </DialogContent>
    </Dialog>
  );
}
