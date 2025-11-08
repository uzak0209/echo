import { ApolloClient, InMemoryCache, HttpLink, from } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';
import { onError } from '@apollo/client/link/error';

const httpLink = new HttpLink({
  uri: process.env.NEXT_PUBLIC_GRAPHQL_URL || 'http://localhost:8000/graphql',
  credentials: 'include', // Enable cookies for authentication
});

// Middleware to add Authorization header with access token
const authLink = setContext((_, { headers }) => {
  // Get the access token from wherever it's stored (will be set by AuthProvider)
  const token = typeof window !== 'undefined' ? localStorage.getItem('accessToken') : null;

  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  };
});

// Track if we're currently refreshing to prevent multiple simultaneous refresh attempts
let isRefreshing = false;
let refreshPromise: Promise<string | null> | null = null;

// Function to refresh the access token
const refreshAccessToken = async (): Promise<string | null> => {
  if (isRefreshing && refreshPromise) {
    // If already refreshing, wait for the existing refresh to complete
    return refreshPromise;
  }

  isRefreshing = true;
  refreshPromise = (async () => {
    try {
      console.log('[Apollo] Refreshing access token...');
      const response = await fetch(process.env.NEXT_PUBLIC_GRAPHQL_URL || 'http://localhost:8000/graphql', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include', // Include refresh token cookie
        body: JSON.stringify({
          query: `
            mutation RefreshToken {
              refreshToken {
                accessToken
              }
            }
          `,
        }),
      });

      const result = await response.json();

      if (result.data?.refreshToken?.accessToken) {
        const newToken = result.data.refreshToken.accessToken;
        if (typeof window !== 'undefined') {
          localStorage.setItem('accessToken', newToken);
        }
        console.log('[Apollo] Access token refreshed successfully');
        return newToken;
      } else {
        console.error('[Apollo] Failed to refresh token:', result.errors);
        // Clear invalid tokens and redirect to login
        if (typeof window !== 'undefined') {
          localStorage.removeItem('accessToken');
          localStorage.removeItem('userId');
        }
        return null;
      }
    } catch (error) {
      console.error('[Apollo] Token refresh error:', error);
      // Clear invalid tokens
      if (typeof window !== 'undefined') {
        localStorage.removeItem('accessToken');
        localStorage.removeItem('userId');
      }
      return null;
    } finally {
      isRefreshing = false;
      refreshPromise = null;
    }
  })();

  return refreshPromise;
};

// Error link to handle authentication errors
const errorLink = onError(({ graphQLErrors, networkError, operation, forward }) => {
  if (graphQLErrors) {
    for (const err of graphQLErrors) {
      // Check for unauthorized errors
      if (err.message.includes('Unauthorized') || err.message.includes('No valid access token')) {
        console.log('[Apollo] Detected expired token, attempting refresh...');

        // Try to refresh the token
        return new Observable((observer) => {
          refreshAccessToken()
            .then((newToken) => {
              if (newToken) {
                // Update the operation with the new token
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: {
                    ...oldHeaders,
                    authorization: `Bearer ${newToken}`,
                  },
                });

                // Retry the request with the new token
                const subscriber = {
                  next: observer.next.bind(observer),
                  error: observer.error.bind(observer),
                  complete: observer.complete.bind(observer),
                };

                forward(operation).subscribe(subscriber);
              } else {
                // Refresh failed, complete with error
                observer.error(err);
              }
            })
            .catch((error) => {
              observer.error(error);
            });
        });
      }
    }
  }

  if (networkError) {
    console.error('[Apollo] Network error:', networkError);
  }
});

// Import Observable for retry logic
import { Observable } from '@apollo/client/core';

const apolloClient = new ApolloClient({
  link: from([errorLink, authLink, httpLink]),
  cache: new InMemoryCache(),
});

export default apolloClient;
