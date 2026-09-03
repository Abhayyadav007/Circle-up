import { apiClient } from "@/api/client";
import type { TokenPair } from "@/store/authStore";

type TokenPairWire = {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
};

const toPair = (w: TokenPairWire): TokenPair => ({
  accessToken: w.access_token,
  refreshToken: w.refresh_token,
});

export const authApi = {
  async signup(input: {
    email: string;
    username: string;
    password: string;
    displayName?: string;
  }): Promise<TokenPair> {
    const { data } = await apiClient.post<TokenPairWire>("/auth/signup", {
      email: input.email,
      username: input.username,
      password: input.password,
      display_name: input.displayName,
    });
    return toPair(data);
  },

  async login(input: { email: string; password: string }): Promise<TokenPair> {
    const { data } = await apiClient.post<TokenPairWire>("/auth/login", input);
    return toPair(data);
  },

  /** Exchange a Google ID token for a Circleup session. */
  async oauthGoogle(idToken: string): Promise<TokenPair> {
    const { data } = await apiClient.post<TokenPairWire>("/auth/oauth/google", {
      id_token: idToken,
    });
    return toPair(data);
  },
};
