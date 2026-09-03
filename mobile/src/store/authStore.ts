import * as SecureStore from "expo-secure-store";
import { create } from "zustand";

/**
 * UI-facing auth state. Tokens are the source of truth for "am I logged in";
 * they're persisted in the device keychain (SecureStore), not AsyncStorage.
 */
const ACCESS_KEY = "circleup.accessToken";
const REFRESH_KEY = "circleup.refreshToken";

export type TokenPair = {
  accessToken: string;
  refreshToken: string;
};

type AuthState = {
  accessToken: string | null;
  refreshToken: string | null;
  /** True until we've checked SecureStore on app start. */
  hydrating: boolean;
  isAuthenticated: boolean;

  hydrate: () => Promise<void>;
  setTokens: (tokens: TokenPair) => Promise<void>;
  clear: () => Promise<void>;
};

export const useAuthStore = create<AuthState>((set) => ({
  accessToken: null,
  refreshToken: null,
  hydrating: true,
  isAuthenticated: false,

  hydrate: async () => {
    const [accessToken, refreshToken] = await Promise.all([
      SecureStore.getItemAsync(ACCESS_KEY),
      SecureStore.getItemAsync(REFRESH_KEY),
    ]);
    set({
      accessToken,
      refreshToken,
      isAuthenticated: Boolean(accessToken),
      hydrating: false,
    });
  },

  setTokens: async ({ accessToken, refreshToken }) => {
    await Promise.all([
      SecureStore.setItemAsync(ACCESS_KEY, accessToken),
      SecureStore.setItemAsync(REFRESH_KEY, refreshToken),
    ]);
    set({ accessToken, refreshToken, isAuthenticated: true });
  },

  clear: async () => {
    await Promise.all([
      SecureStore.deleteItemAsync(ACCESS_KEY),
      SecureStore.deleteItemAsync(REFRESH_KEY),
    ]);
    set({ accessToken: null, refreshToken: null, isAuthenticated: false });
  },
}));

/** Non-hook access for the Axios interceptor. */
export const authStore = {
  get: () => useAuthStore.getState(),
};
