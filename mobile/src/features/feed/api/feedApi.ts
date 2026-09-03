import { apiClient } from "@/api/client";
import type { Page, PostView } from "@/api/types";

export const feedApi = {
  async page(
    scope: "home" | "explore",
    cursor?: string,
    limit = 15,
  ): Promise<Page<PostView>> {
    const path = scope === "home" ? "/feed" : "/feed/explore";
    const { data } = await apiClient.get<Page<PostView>>(path, {
      params: { cursor, limit },
    });
    return data;
  },
};
