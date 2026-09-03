import { apiClient } from "@/api/client";
import type { PostView, UserSummary } from "@/api/types";

export const searchApi = {
  users: (q: string) =>
    apiClient.get<UserSummary[]>("/search/users", { params: { q } }).then((r) => r.data),

  /** Explore grid — recent global posts. */
  explore: (cursor?: string) =>
    apiClient
      .get<{ items: PostView[]; next_cursor: string | null }>("/feed/explore", {
        params: { cursor, limit: 21 },
      })
      .then((r) => r.data),
};
