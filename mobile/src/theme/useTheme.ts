import { theme, type Theme } from "./theme";

/**
 * Hook accessor for the theme. Today it returns the static object; keeping it a
 * hook means a future light/dark switch is a one-file change (return a different
 * palette based on `useColorScheme()`), not an app-wide refactor.
 */
export function useTheme(): Theme {
  return theme;
}
