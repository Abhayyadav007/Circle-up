import { useInfiniteQuery } from "@tanstack/react-query";

import { queryKeys } from "@/api/queryClient";

import { feedApi } from "../api/feedApi";

export function useFeed(scope: "home" | "explore" = "home") {
  const query = useInfiniteQuery({
    queryKey: queryKeys.feed(scope),
    queryFn: ({ pageParam }) => feedApi.page(scope, pageParam),
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (last) => last.next_cursor ?? undefined,
  });

  const posts = query.data?.pages.flatMap((p) => p.items) ?? [];

  return { ...query, posts };
}
