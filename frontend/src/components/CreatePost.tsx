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
        <Button
          size="lg"
          className="fixed bottom-8 right-8 h-16 w-16 rounded-full bg-gradient-to-br from-blue-500 to-pink-500 hover:from-blue-600 hover:to-pink-600 shadow-lg hover:shadow-xl z-50 p-0 transition-all duration-300 border-2 border-white/20"
        >
          <Plus className="h-8 w-8 text-white" />
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[600px] bg-card border border-border">
        <DialogHeader>
          <DialogTitle className="text-xl font-semibold">
            投稿を作成
          </DialogTitle>
        </DialogHeader>
        <div className="space-y-4 pt-4">
          <div className="relative">
            <Textarea
              placeholder="いま何してる？"
              value={content}
              onChange={(e) => setContent(e.target.value)}
              maxLength={1000}
              className="min-h-[180px] resize-none text-base"
            />
          </div>

          <div className="flex justify-between items-center">
            <span className="text-sm text-muted-foreground">
              {content.length}/1000
            </span>
            <Button
              onClick={handleSubmit}
              disabled={loading || !content.trim()}
              className="bg-gradient-to-r from-blue-500 to-pink-500 hover:from-blue-600 hover:to-pink-600 transition-all duration-300"
            >
              {loading ? (
                <span className="flex items-center gap-2">
                  <span className="inline-block w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  投稿中...
                </span>
              ) : (
                '投稿'
              )}
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
