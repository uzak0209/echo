import { ApolloClient, InMemoryCache, HttpLink, ApolloLink } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';

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

const apolloClient = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache(),
});

export default apolloClient;
