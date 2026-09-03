# Circleup mobile

React Native (Expo) + TypeScript. **Feature folders mirror the backend slices.**

```
src/
├── theme/          theme.ts — the ONLY source of colors / spacing / type (blue palette)
├── config/env.ts   typed runtime config from app.json -> expo.extra
├── api/
│   ├── client.ts     single Axios instance: base URL + auth + refresh interceptor
│   ├── queryClient.ts React Query client + queryKeys factory
│   └── types.ts       wire types mirroring backend dto.rs
├── store/authStore.ts Zustand + expo-secure-store (tokens in the keychain)
├── navigation/       RootNavigator (auth vs app) + TabNavigator + typed params
├── components/ui/    shared kit: Button, Input, Avatar, Card, Text, Screen
└── features/         auth / feed / profile / post / comments / likes / search
        <feature>/
        ├── api/       typed calls
        ├── hooks/     React Query wrappers
        ├── screens/
        └── components/
```

## Commands

```bash
npm install
npx expo start            # then press i (iOS) / a (Android)
npm run lint              # eslint, --max-warnings 0
npm run typecheck         # tsc --noEmit --strict
npm test                  # jest
```

## Config

The app reads `app.json` → `expo.extra` (see `src/config/env.ts`). Set:

- `apiBaseUrl` — e.g. `http://localhost:8080/api/v1` (simulator),
  `http://10.0.2.2:8080/api/v1` (Android emulator), or your LAN IP (device).
- `googleWebClientId` / `googleIosClientId` — see the root README's
  "Google Sign-In setup".

## State management

- **Server state:** React Query (`@tanstack/react-query`). Keys come from
  `queryKeys` in `src/api/queryClient.ts`.
- **Auth/session state:** Zustand (`src/store/authStore.ts`). Tokens persist in
  `expo-secure-store`; the Axios interceptor refreshes them once on 401.
- **Local UI state:** component `useState`.
