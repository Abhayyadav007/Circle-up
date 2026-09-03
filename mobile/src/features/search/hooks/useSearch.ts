import { keepPreviousData, useQuery } from "@tanstack/react-query";

import { queryKeys } from "@/api/queryClient";

import { searchApi } from "../api/searchApi";

export function useUserSearch(term: string) {
  const q = term.trim();
  return useQuery({
    queryKey: queryKeys.search(q),
    queryFn: () => searchApi.users(q),
    enabled: q.length >= 2,
    placeholderData: keepPreviousData,
  });
}
