'use client';

import { ApolloProvider } from '@apollo/client';
import apolloClient from './apollo-client';
import { AuthProvider } from './auth-context';
import { ReactionProvider } from './reaction-context';

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <ApolloProvider client={apolloClient}>
      <AuthProvider>
        <ReactionProvider>{children}</ReactionProvider>
      </AuthProvider>
    </ApolloProvider>
  );
}
