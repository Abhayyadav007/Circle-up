import { useMutation } from "@tanstack/react-query";

import { queryClient } from "@/api/queryClient";
import { useAuthStore } from "@/store/authStore";

import { authApi } from "../api/authApi";
import { getGoogleIdToken, googleSignOut } from "../api/googleSignIn";

/** Login / signup / Google / logout, wired to the auth store + query cache. */
export function useAuth() {
  const setTokens = useAuthStore((s) => s.setTokens);
  const clear = useAuthStore((s) => s.clear);

  const login = useMutation({
    mutationFn: authApi.login,
    onSuccess: setTokens,
  });

  const signup = useMutation({
    mutationFn: authApi.signup,
    onSuccess: setTokens,
  });

  const googleSignIn = useMutation({
    mutationFn: async () => {
      const idToken = await getGoogleIdToken();
      return authApi.oauthGoogle(idToken);
    },
    onSuccess: setTokens,
  });

  async function logout() {
    await googleSignOut();
    await clear();
    queryClient.clear();
  }

  return { login, signup, googleSignIn, logout };
}
