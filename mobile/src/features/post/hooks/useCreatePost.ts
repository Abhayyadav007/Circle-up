import { useMutation, useQueryClient } from "@tanstack/react-query";

import { queryKeys } from "@/api/queryClient";

import { postApi, uploadToStorage } from "../api/postApi";

type Input = {
  /** Local file URI from expo-image-picker. */
  fileUri: string;
  /** e.g. "image/jpeg". */
  contentType: string;
  caption: string;
};

/**
 * The full 3-step create flow:
 *   1. presign  → 2. PUT to R2  → 3. create post row
 */
export function useCreatePost() {
  const qc = useQueryClient();

  return useMutation({
    mutationFn: async ({ fileUri, contentType, caption }: Input) => {
      const presigned = await postApi.createUploadUrl(contentType);
      await uploadToStorage(presigned, fileUri, contentType);
      return postApi.create({ imageKey: presigned.key, caption });
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: queryKeys.feed("home") });
      qc.invalidateQueries({ queryKey: queryKeys.me() });
    },
  });
}
