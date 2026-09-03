import axios, {
  type AxiosError,
  type AxiosInstance,
  type InternalAxiosRequestConfig,
} from "axios";

import { env } from "@/config/env";
import { useAuthStore } from "@/store/authStore";

/**
 * The one HTTP client. All feature `api/` modules import `apiClient` from here so
 * base URL, auth headers, and token-refresh live in exactly one place.
 */
export const apiClient: AxiosInstance = axios.create({
  baseURL: env.apiBaseUrl,
  timeout: 15_000,
});

// --- Request: attach the access token -------------------------------------
apiClient.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const token = useAuthStore.getState().accessToken;
  if (token) {
    config.headers.set("Authorization", `Bearer ${token}`);
  }
  return config;
});

// --- Response: refresh once on 401, then retry ----------------------------
type RetriableConfig = InternalAxiosRequestConfig & { _retried?: boolean };

let refreshInFlight: Promise<string | null> | null = null;

async function refreshAccessToken(): Promise<string | null> {
  const { refreshToken, setTokens, clear } = useAuthStore.getState();
  if (!refreshToken) return null;

  try {
    // Bare axios (not apiClient) to skip interceptors and avoid a loop.
    const res = await axios.post(`${env.apiBaseUrl}/auth/refresh`, {
      refresh_token: refreshToken,
    });
    const next = {
      accessToken: res.data.access_token as string,
      refreshToken: res.data.refresh_token as string,
    };
    await setTokens(next);
    return next.accessToken;
  } catch {
    await clear();
    return null;
  }
}

apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const original = error.config as RetriableConfig | undefined;
    const isAuthEndpoint = original?.url?.includes("/auth/");

    if (error.response?.status === 401 && original && !original._retried && !isAuthEndpoint) {
      original._retried = true;
      refreshInFlight ??= refreshAccessToken().finally(() => {
        refreshInFlight = null;
      });
      const newToken = await refreshInFlight;
      if (newToken) {
        original.headers.set("Authorization", `Bearer ${newToken}`);
        return apiClient(original);
      }
    }
    return Promise.reject(normalizeError(error));
  },
);

// --- Error normalisation -------------------------------------------------
export type ApiError = {
  status: number;
  code: string;
  message: string;
  fields?: { field: string; message: string }[];
};

function normalizeError(error: AxiosError): ApiError {
  const body = error.response?.data as
    | { error?: { code?: string; message?: string; fields?: ApiError["fields"] } }
    | undefined;
  return {
    status: error.response?.status ?? 0,
    code: body?.error?.code ?? "network_error",
    message: body?.error?.message ?? error.message ?? "Something went wrong",
    fields: body?.error?.fields,
  };
}
