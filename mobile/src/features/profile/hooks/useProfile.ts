import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { queryKeys } from "@/api/queryClient";

import { profileApi } from "../api/profileApi";

/** Pass `undefined` for the current user's own profile. */
export function useProfile(username?: string) {
  const key = username ? queryKeys.profile(username) : queryKeys.me();
  return useQuery({
    queryKey: key,
    queryFn: () => (username ? profileApi.byUsername(username) : profileApi.me()),
  });
}

export function useFollow(username: string) {
  const qc = useQueryClient();
  const invalidate = () => qc.invalidateQueries({ queryKey: queryKeys.profile(username) });

  const follow = useMutation({ mutationFn: () => profileApi.follow(username), onSuccess: invalidate });
  const unfollow = useMutation({
    mutationFn: () => profileApi.unfollow(username),
    onSuccess: invalidate,
  });

  return { follow, unfollow };
}
