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
          className="fixed bottom-6 right-6 h-14 w-14 rounded-full bg-green-500 hover:bg-green-600 shadow-lg z-50 p-0"
        >
          <Plus className="h-6 w-6" />
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[525px]">
        <DialogHeader>
          <DialogTitle>投稿を作成</DialogTitle>
          <DialogDescription>
            あなたの思いを共有しましょう。投稿は100回表示されると消えます。
          </DialogDescription>
        </DialogHeader>
        <div className="space-y-4 pt-4">
          <Textarea
            placeholder="いま何してる？"
            value={content}
            onChange={(e) => setContent(e.target.value)}
            maxLength={1000}
            className="min-h-[150px]"
          />
          <div className="flex justify-between items-center">
            <p className="text-sm text-muted-foreground">
              {content.length}/1000 文字
            </p>
            <Button onClick={handleSubmit} disabled={loading || !content.trim()}>
              {loading ? '投稿中...' : '投稿'}
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            注意: 自分の投稿は確認できません。他のユーザーにランダムで表示されます。
          </p>
        </div>
      </DialogContent>
    </Dialog>
  );
}
