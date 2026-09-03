import { apiClient } from "@/api/client";
import type { Page, ProfileResponse, UserSummary } from "@/api/types";

export const profileApi = {
  me: () => apiClient.get<ProfileResponse>("/profiles/me").then((r) => r.data),

  byUsername: (username: string) =>
    apiClient.get<ProfileResponse>(`/profiles/${username}`).then((r) => r.data),

  updateMe: (patch: { display_name?: string | null; bio?: string; avatar_url?: string | null }) =>
    apiClient.patch<ProfileResponse>("/profiles/me", patch).then((r) => r.data),

  follow: (username: string) =>
    apiClient
      .put<{ following: boolean; followers_count: number }>(`/profiles/${username}/follow`)
      .then((r) => r.data),

  unfollow: (username: string) =>
    apiClient
      .delete<{ following: boolean; followers_count: number }>(`/profiles/${username}/follow`)
      .then((r) => r.data),

  followers: (username: string, cursor?: string) =>
    apiClient
      .get<Page<UserSummary>>(`/profiles/${username}/followers`, { params: { cursor } })
      .then((r) => r.data),

  following: (username: string, cursor?: string) =>
    apiClient
      .get<Page<UserSummary>>(`/profiles/${username}/following`, { params: { cursor } })
      .then((r) => r.data),
};
