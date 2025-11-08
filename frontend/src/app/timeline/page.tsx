'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/lib/auth-context';
import { CreatePost } from '@/components/CreatePost';
import { Timeline } from '@/components/Timeline';
import { Button } from '@/components/ui/button';
import { ReactionNotification } from '@/components/ReactionNotification';
import { UserProfile } from '@/components/UserProfile';

export default function TimelinePage() {
  const { isAuthenticated, logout } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  if (!isAuthenticated) {
    return null;
  }

  return (
    <main className="min-h-screen bg-background">
      {/* Real-time reaction notifications via SSE */}
      <ReactionNotification />

      <div className="container mx-auto px-4 py-8 max-w-2xl">
        <header className="mb-8 flex justify-between items-start">
          <div>
            <h1 className="text-4xl font-bold mb-2">Echo</h1>
            <p className="text-muted-foreground">
              A validation-free social network. Post your thoughts without worrying about likes or follows.
            </p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" onClick={() => router.push('/avatar')}>
              Avatar
            </Button>
            <Button variant="outline" onClick={logout}>
              Logout
            </Button>
          </div>
        </header>

        <div className="space-y-8">
          <UserProfile />
          <CreatePost />
          <Timeline />
        </div>
      </div>
    </main>
  );
}
