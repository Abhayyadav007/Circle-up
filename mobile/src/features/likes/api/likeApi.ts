import { apiClient } from "@/api/client";

export type LikeResponse = { liked: boolean; like_count: number };

export const likeApi = {
  like: (postId: string) =>
    apiClient.put<LikeResponse>(`/posts/${postId}/like`).then((r) => r.data),
  unlike: (postId: string) =>
    apiClient.delete<LikeResponse>(`/posts/${postId}/like`).then((r) => r.data),
};
