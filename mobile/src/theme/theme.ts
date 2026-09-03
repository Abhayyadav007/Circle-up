/**
 * The single source of truth for Circleup's visual language.
 *
 * Every color, spacing value, radius, and text style used in the app should come
 * from here — never hard-code a hex value or a magic number in a component.
 * The palette is blue-forward (Circleup's brand).
 */

export const palette = {
  // --- Brand blues ---
  blue50: "#EFF6FF",
  blue100: "#DBEAFE",
  blue200: "#BFDBFE",
  blue300: "#93C5FD",
  blue400: "#60A5FA",
  blue500: "#3B82F6",
  blue600: "#2563EB", // primary
  blue700: "#1D4ED8", // primary pressed / brand
  blue800: "#1E40AF",
  blue900: "#1E3A8A",

  // --- Neutrals ---
  white: "#FFFFFF",
  gray50: "#F8FAFC",
  gray100: "#F1F5F9",
  gray200: "#E2E8F0",
  gray300: "#CBD5E1",
  gray400: "#94A3B8",
  gray500: "#64748B",
  gray600: "#475569",
  gray700: "#334155",
  gray800: "#1E293B",
  gray900: "#0F172A",
  black: "#000000",

  // --- Feedback ---
  danger: "#DC2626",
  success: "#16A34A",
  warning: "#D97706",
  like: "#EF4444",
} as const;

export const colors = {
  primary: palette.blue600,
  primaryDark: palette.blue700,
  primaryLight: palette.blue100,
  secondary: palette.blue400,
  accent: palette.blue500,

  background: palette.white,
  surface: palette.white,
  surfaceAlt: palette.gray50,
  border: palette.gray200,

  textPrimary: palette.gray900,
  textSecondary: palette.gray500,
  textInverse: palette.white,
  textLink: palette.blue600,

  danger: palette.danger,
  success: palette.success,
  warning: palette.warning,
  like: palette.like,
} as const;

export const spacing = {
  xs: 4,
  sm: 8,
  md: 12,
  lg: 16,
  xl: 24,
  xxl: 32,
  xxxl: 48,
} as const;

export const radius = {
  sm: 6,
  md: 10,
  lg: 16,
  pill: 999,
} as const;

export const typography = {
  display: { fontSize: 28, fontWeight: "700", lineHeight: 34 },
  title: { fontSize: 22, fontWeight: "700", lineHeight: 28 },
  heading: { fontSize: 18, fontWeight: "600", lineHeight: 24 },
  body: { fontSize: 15, fontWeight: "400", lineHeight: 21 },
  bodyStrong: { fontSize: 15, fontWeight: "600", lineHeight: 21 },
  caption: { fontSize: 13, fontWeight: "400", lineHeight: 18 },
  button: { fontSize: 15, fontWeight: "600", lineHeight: 20 },
} as const;

export const theme = {
  palette,
  colors,
  spacing,
  radius,
  typography,
} as const;

export type Theme = typeof theme;
export type ThemeColor = keyof typeof colors;
export type SpacingKey = keyof typeof spacing;
