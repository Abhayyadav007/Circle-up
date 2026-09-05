import { useMutation, useQueryClient } from "@tanstack/react-query";

import { queryKeys } from "@/api/queryClient";

import { postApi, uploadToStorage } from "../api/postApi";

/** Tags which of the 3 create-post steps threw, so the UI can show it. */
async function step<T>(label: string, fn: () => Promise<T>): Promise<T> {
  try {
    return await fn();
  } catch (err) {
    const message = err && typeof err === "object" && "message" in err ? String(err.message) : String(err);
    throw new Error(`[${label}] ${message}`);
  }
}

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
      const presigned = await step("requesting upload URL", () =>
        postApi.createUploadUrl(contentType),
      );
      await step("uploading image", () => uploadToStorage(presigned, fileUri, contentType));
      return step("saving post", () => postApi.create({ imageKey: presigned.key, caption }));
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: queryKeys.feed("home") });
      qc.invalidateQueries({ queryKey: queryKeys.me() });
    },
  });
}
