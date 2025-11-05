'use client';

import { useAuth } from '@/lib/auth-context';
import { CreatePost } from '@/components/CreatePost';
import { Timeline } from '@/components/Timeline';
import { Login } from '@/components/Login';
import { Button } from '@/components/ui/button';

export default function Home() {
  const { isAuthenticated, logout } = useAuth();

  if (!isAuthenticated) {
    return <Login />;
  }

  return (
    <main className="min-h-screen bg-background">
      <div className="container mx-auto px-4 py-8 max-w-2xl">
        <header className="mb-8 flex justify-between items-start">
          <div>
            <h1 className="text-4xl font-bold mb-2">Echo</h1>
            <p className="text-muted-foreground">
              A validation-free social network. Post your thoughts without worrying about likes or follows.
            </p>
          </div>
          <Button variant="outline" onClick={logout}>
            Logout
          </Button>
        </header>

        <div className="space-y-8">
          <CreatePost />
          <Timeline />
        </div>
      </div>
    </main>
  );
}
