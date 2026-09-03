import { QueryClient } from "@tanstack/react-query";

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 30_000,
      refetchOnWindowFocus: false,
    },
  },
});

/** Central query-key factory — keeps cache keys consistent across features. */
export const queryKeys = {
  feed: (scope: "home" | "explore") => ["feed", scope] as const,
  post: (id: string) => ["post", id] as const,
  comments: (postId: string) => ["comments", postId] as const,
  profile: (username: string) => ["profile", username] as const,
  me: () => ["profile", "me"] as const,
  userPosts: (username: string) => ["userPosts", username] as const,
  followers: (username: string) => ["followers", username] as const,
  following: (username: string) => ["following", username] as const,
  search: (term: string) => ["search", term] as const,
};
