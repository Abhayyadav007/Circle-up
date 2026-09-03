import Constants from "expo-constants";

/**
 * Typed access to runtime config. Values come from `app.json` → `expo.extra`
 * (override per-environment with EAS build profiles or `app.config.ts`).
 */
type Extra = {
  apiBaseUrl: string;
  googleWebClientId: string;
  googleIosClientId: string;
};

const extra = (Constants.expoConfig?.extra ?? {}) as Partial<Extra>;

export const env = {
  /** Base URL of the Axum backend, including `/api/v1`. */
  apiBaseUrl: extra.apiBaseUrl ?? "http://localhost:8080/api/v1",
  /** Google OAuth "Web" client ID — the one the backend expects as `aud`. */
  googleWebClientId: extra.googleWebClientId ?? "",
  googleIosClientId: extra.googleIosClientId ?? "",
} as const;
