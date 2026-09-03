import { env } from "@/config/env";

/**
 * `@react-native-google-signin/google-signin` is a **native module** — it is NOT
 * bundled in Expo Go. Importing it at startup crashes Expo Go with
 * "RNGoogleSignin could not be found".
 *
 * So we load it lazily and only when it's actually available:
 *   - Expo Go            -> module missing -> "Continue with Google" shows a friendly error
 *   - dev build / release -> module present -> real native flow
 *
 * To use Google Sign-In for real, run a development build:
 *   npx expo prebuild && npx expo run:ios   (or run:android / an EAS dev build)
 */

type GoogleSigninModule = typeof import("@react-native-google-signin/google-signin");

let cached: GoogleSigninModule | null | undefined;

function loadGoogleSignin(): GoogleSigninModule | null {
  if (cached !== undefined) return cached;
  try {
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    cached = require("@react-native-google-signin/google-signin") as GoogleSigninModule;
  } catch {
    cached = null;
  }
  return cached;
}

/** True when the native module is linked (dev/release build, not Expo Go). */
export function isGoogleSignInAvailable(): boolean {
  return loadGoogleSignin() != null && Boolean(env.googleWebClientId);
}

/** Call once on app start (see App.tsx). No-op if unavailable. */
export function configureGoogleSignIn(): void {
  const mod = loadGoogleSignin();
  if (!mod || !env.googleWebClientId) return;
  mod.GoogleSignin.configure({
    // The backend verifies ID tokens against THIS (web) client id.
    webClientId: env.googleWebClientId,
    iosClientId: env.googleIosClientId || undefined,
    offlineAccess: false,
  });
}

/**
 * Runs the native Google flow and returns the ID token to forward to
 * `POST /auth/oauth/google`.
 */
export async function getGoogleIdToken(): Promise<string> {
  const mod = loadGoogleSignin();
  if (!mod) {
    throw new Error(
      "Google Sign-In needs a development build (not Expo Go). Run `npx expo prebuild && npx expo run:ios`.",
    );
  }
  if (!env.googleWebClientId) {
    throw new Error("Google Sign-In is not configured (set googleWebClientId in app.json).");
  }

  await mod.GoogleSignin.hasPlayServices({ showPlayServicesUpdateDialog: true });
  const result = await mod.GoogleSignin.signIn();
  const idToken =
    // @ts-expect-error — support both v13 ({ data }) and older ({ idToken }) shapes
    result?.data?.idToken ?? result?.idToken;
  if (!idToken) {
    throw new Error("Google did not return an ID token.");
  }
  return idToken as string;
}

/** Best-effort native sign-out. Safe to call when the module is absent. */
export async function googleSignOut(): Promise<void> {
  const mod = loadGoogleSignin();
  if (!mod) return;
  try {
    await mod.GoogleSignin.signOut();
  } catch {
    // user may not have signed in with Google
  }
}
