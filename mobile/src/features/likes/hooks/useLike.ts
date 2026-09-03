import { useMutation } from "@tanstack/react-query";
import { useState } from "react";

import { likeApi } from "../api/likeApi";

/**
 * Optimistic like toggle for a single post. The caller passes the current state;
 * this hook flips it immediately and reconciles with the server response.
 */
export function useLike(postId: string, initial: { liked: boolean; count: number }) {
  const [liked, setLiked] = useState(initial.liked);
  const [count, setCount] = useState(initial.count);

  const mutation = useMutation({
    mutationFn: () => (liked ? likeApi.unlike(postId) : likeApi.like(postId)),
    onMutate: () => {
      setLiked((v) => !v);
      setCount((c) => c + (liked ? -1 : 1));
    },
    onSuccess: (res) => {
      setLiked(res.liked);
      setCount(res.like_count);
    },
    onError: () => {
      setLiked(initial.liked);
      setCount(initial.count);
    },
  });

  return { liked, count, toggle: () => mutation.mutate() };
}
