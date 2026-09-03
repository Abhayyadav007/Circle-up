import * as FileSystem from "expo-file-system";

import { apiClient } from "@/api/client";
import type { Page, PostView, PresignedUpload } from "@/api/types";

export const postApi = {
  get: (id: string) => apiClient.get<PostView>(`/posts/${id}`).then((r) => r.data),

  delete: (id: string) => apiClient.delete(`/posts/${id}`),

  byUser: (username: string, cursor?: string) =>
    apiClient
      .get<Page<PostView>>(`/profiles/${username}/posts`, { params: { cursor, limit: 18 } })
      .then((r) => r.data),

  /** Step 1 of upload: ask the API for a presigned PUT URL. */
  createUploadUrl: (contentType: string) =>
    apiClient
      .post<PresignedUpload>("/posts/uploads", { content_type: contentType })
      .then((r) => r.data),

  /** Step 3 of upload: create the post row from the uploaded object key. */
  create: (input: { imageKey: string; caption: string }) =>
    apiClient
      .post<PostView>("/posts", { image_key: input.imageKey, caption: input.caption })
      .then((r) => r.data),
};

/**
 * Step 2 of upload: PUT the file bytes to the presigned URL. With R2 this goes
 * straight to Cloudflare; in local dev it hits this backend's `/media` route.
 *
 * Uses `expo-file-system` (streams the file) rather than `fetch` + `blob`, which
 * is unreliable for binary bodies on React Native.
 */
export async function uploadToStorage(
  presigned: PresignedUpload,
  fileUri: string,
  contentType: string,
): Promise<void> {
  const headers = Object.fromEntries([
    ...presigned.headers,
    ["Content-Type", contentType],
  ]);

  const res = await FileSystem.uploadAsync(presigned.url, fileUri, {
    httpMethod: "PUT",
    uploadType: FileSystem.FileSystemUploadType.BINARY_CONTENT,
    headers,
  });

  if (res.status < 200 || res.status >= 300) {
    throw new Error(`Upload failed (${res.status}): ${res.body?.slice(0, 200) ?? ""}`);
  }
}
