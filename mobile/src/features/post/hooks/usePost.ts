import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "@/api/queryClient";

import { postApi } from "../api/postApi";

export function usePost(postId: string) {
  return useQuery({
    queryKey: queryKeys.post(postId),
    queryFn: () => postApi.get(postId),
  });
}
