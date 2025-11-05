'use client';

import { ApolloProvider } from '@apollo/client';
import apolloClient from '@/lib/apollo-client';
import { CreatePost } from '@/components/CreatePost';
import { Timeline } from '@/components/Timeline';

export default function Home() {
  return (
    <ApolloProvider client={apolloClient}>
      <main className="min-h-screen bg-background">
        <div className="container mx-auto px-4 py-8 max-w-2xl">
          <header className="mb-8">
            <h1 className="text-4xl font-bold mb-2">Echo</h1>
            <p className="text-muted-foreground">
              A validation-free social network. Post your thoughts without worrying about likes or follows.
            </p>
          </header>

          <div className="space-y-8">
            <CreatePost />
            <Timeline />
          </div>
        </div>
      </main>
    </ApolloProvider>
  );
}
