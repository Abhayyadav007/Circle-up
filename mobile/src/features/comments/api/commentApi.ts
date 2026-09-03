import { apiClient } from "@/api/client";
import type { CommentView, Page } from "@/api/types";

export const commentApi = {
  async page(postId: string, cursor?: string): Promise<Page<CommentView>> {
    const { data } = await apiClient.get<Page<CommentView>>(`/posts/${postId}/comments`, {
      params: { cursor, limit: 30 },
    });
    return data;
  },

  async create(postId: string, body: string): Promise<CommentView> {
    const { data } = await apiClient.post<CommentView>(`/posts/${postId}/comments`, { body });
    return data;
  },

  remove: (commentId: string) => apiClient.delete(`/comments/${commentId}`),
};
