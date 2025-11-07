'use client';

import React, { createContext, useContext, useState, useCallback, useEffect } from 'react';
import { useMutation } from '@apollo/client';
import { SIGNUP, LOGIN, CREATE_USER, REFRESH_TOKEN } from './graphql/mutations';

interface AuthContextType {
  accessToken: string | null;
  userId: string | null;
  login: (username: string, password: string) => Promise<void>;
  signup: (username: string, password: string) => Promise<void>;
  createAnonymousUser: (displayName: string, avatarUrl?: string) => Promise<void>;
  refreshAccessToken: () => Promise<void>;
  logout: () => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [userId, setUserId] = useState<string | null>(null);

  const [signupMutation] = useMutation(SIGNUP);
  const [loginMutation] = useMutation(LOGIN);
  const [createUser] = useMutation(CREATE_USER);
  const [refreshToken] = useMutation(REFRESH_TOKEN);

  const signup = useCallback(
    async (username: string, password: string) => {
      try {
        const { data } = await signupMutation({
          variables: { username, password },
        });

        if (data?.signup) {
          setAccessToken(data.signup.accessToken);
          setUserId(data.signup.userId);

          // Store both accessToken and userId in localStorage for persistence
          if (typeof window !== 'undefined') {
            localStorage.setItem('accessToken', data.signup.accessToken);
            localStorage.setItem('userId', data.signup.userId);
          }
        }
      } catch (error) {
        console.error('Signup failed:', error);
        throw error;
      }
    },
    [signupMutation]
  );

  const login = useCallback(
    async (username: string, password: string) => {
      try {
        const { data } = await loginMutation({
          variables: { username, password },
        });

        if (data?.login) {
          setAccessToken(data.login.accessToken);
          setUserId(data.login.userId);

          // Store both accessToken and userId in localStorage for persistence
          if (typeof window !== 'undefined') {
            localStorage.setItem('accessToken', data.login.accessToken);
            localStorage.setItem('userId', data.login.userId);
          }
        }
      } catch (error) {
        console.error('Login failed:', error);
        throw error;
      }
    },
    [loginMutation]
  );

  const createAnonymousUser = useCallback(
    async (displayName: string, avatarUrl?: string) => {
      try {
        const { data } = await createUser({
          variables: { displayName, avatarUrl },
        });

        if (data?.createUser) {
          setAccessToken(data.createUser.accessToken);
          setUserId(data.createUser.userId);

          // Store both accessToken and userId in localStorage for persistence
          if (typeof window !== 'undefined') {
            localStorage.setItem('accessToken', data.createUser.accessToken);
            localStorage.setItem('userId', data.createUser.userId);
          }
        }
      } catch (error) {
        console.error('Create user failed:', error);
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

        // Store new access token in localStorage
        if (typeof window !== 'undefined') {
          localStorage.setItem('accessToken', data.refreshToken.accessToken);
        }
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

    // Clear both accessToken and userId from localStorage
    if (typeof window !== 'undefined') {
      localStorage.removeItem('accessToken');
      localStorage.removeItem('userId');
    }
  }, []);

  // Try to restore accessToken and userId from localStorage on mount
  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedAccessToken = localStorage.getItem('accessToken');
      const storedUserId = localStorage.getItem('userId');

      if (storedAccessToken && storedUserId) {
        setAccessToken(storedAccessToken);
        setUserId(storedUserId);
      } else if (storedUserId && !storedAccessToken) {
        // If we have userId but no access token, try to refresh
        setUserId(storedUserId);
        refreshAccessToken();
      }
    }
  }, []);

  return (
    <AuthContext.Provider
      value={{
        accessToken,
        userId,
        login,
        signup,
        createAnonymousUser,
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
