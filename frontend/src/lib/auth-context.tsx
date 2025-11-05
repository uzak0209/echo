'use client';

import React, { createContext, useContext, useState, useCallback, useEffect } from 'react';
import { useMutation } from '@apollo/client';
import { CREATE_USER, REFRESH_TOKEN } from './graphql/mutations';

interface AuthContextType {
  accessToken: string | null;
  userId: string | null;
  login: (displayName: string, avatarUrl?: string) => Promise<void>;
  refreshAccessToken: () => Promise<void>;
  logout: () => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [userId, setUserId] = useState<string | null>(null);

  const [createUser] = useMutation(CREATE_USER);
  const [refreshToken] = useMutation(REFRESH_TOKEN);

  const login = useCallback(
    async (displayName: string, avatarUrl?: string) => {
      try {
        const { data } = await createUser({
          variables: { displayName, avatarUrl },
        });

        if (data?.createUser) {
          setAccessToken(data.createUser.accessToken);
          setUserId(data.createUser.userId);

          // Store userId in localStorage for persistence
          if (typeof window !== 'undefined') {
            localStorage.setItem('userId', data.createUser.userId);
          }
        }
      } catch (error) {
        console.error('Login failed:', error);
        throw error;
      }
    },
    [createUser]
  );

  const refreshAccessToken = useCallback(async () => {
    try {
      const { data } = await refreshToken();

      if (data?.refreshToken) {
        setAccessToken(data.refreshToken.accessToken);
      }
    } catch (error) {
      console.error('Token refresh failed:', error);
      // If refresh fails, log the user out
      logout();
    }
  }, [refreshToken]);

  const logout = useCallback(() => {
    setAccessToken(null);
    setUserId(null);

    // Clear localStorage
    if (typeof window !== 'undefined') {
      localStorage.removeItem('userId');
    }
  }, []);

  // Try to refresh token on mount if userId exists in localStorage
  useEffect(() => {
    const storedUserId = typeof window !== 'undefined' ? localStorage.getItem('userId') : null;

    if (storedUserId && !accessToken) {
      setUserId(storedUserId);
      refreshAccessToken();
    }
  }, []);

  return (
    <AuthContext.Provider
      value={{
        accessToken,
        userId,
        login,
        refreshAccessToken,
        logout,
        isAuthenticated: !!accessToken && !!userId,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
