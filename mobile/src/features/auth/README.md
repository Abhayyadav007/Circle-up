# feature: auth (mobile)

Mirrors backend `features/auth`. Owns login, signup, and "Continue with Google",
plus the session store.

| path                    | responsibility                                          |
|-------------------------|--------------------------------------------------------|
| `api/authApi.ts`        | typed calls to `/auth/signup|login|refresh|oauth/google` |
| `api/googleSignIn.ts`   | native Google flow → ID token                           |
| `hooks/useAuth.ts`      | React Query mutations + auth-store wiring               |
| `screens/`              | `LoginScreen`, `SignupScreen`                           |
| `components/`           | `GoogleButton`                                          |

Tokens live in `src/store/authStore.ts` (Zustand + `expo-secure-store`). The
Axios refresh interceptor is in `src/api/client.ts`. `RootNavigator` swaps the
auth stack for the tab navigator based on `isAuthenticated`.

## Google Sign-In needs a development build

`@react-native-google-signin/google-signin` is a native module and is **not in
Expo Go**. `api/googleSignIn.ts` lazy-loads it, so the app still runs in Expo Go
— but "Continue with Google" will show an error until you use a dev build:

```bash
npx expo prebuild            # generates ios/ + android/
npx expo run:ios             # or: run:android  (or an EAS dev build)
```

Email/password auth works everywhere, including Expo Go.
