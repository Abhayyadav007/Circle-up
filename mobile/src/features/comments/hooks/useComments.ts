import {
  useInfiniteQuery,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";

import { queryKeys } from "@/api/queryClient";

import { commentApi } from "../api/commentApi";

export function useComments(postId: string) {
  const qc = useQueryClient();

  const list = useInfiniteQuery({
    queryKey: queryKeys.comments(postId),
    queryFn: ({ pageParam }) => commentApi.page(postId, pageParam),
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (last) => last.next_cursor ?? undefined,
  });

  const invalidate = () => qc.invalidateQueries({ queryKey: queryKeys.comments(postId) });

  const add = useMutation({
    mutationFn: (body: string) => commentApi.create(postId, body),
    onSuccess: invalidate,
  });

  const remove = useMutation({
    mutationFn: (commentId: string) => commentApi.remove(commentId),
    onSuccess: invalidate,
  });

  return {
    ...list,
    comments: list.data?.pages.flatMap((p) => p.items) ?? [],
    add,
    remove,
  };
}
