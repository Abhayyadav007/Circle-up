import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  View,
  type PressableProps,
} from "react-native";

import { colors, radius, spacing, typography } from "@/theme";

import { Text } from "./Text";

type Variant = "primary" | "secondary" | "outline" | "ghost";

export type ButtonProps = Omit<PressableProps, "children"> & {
  label: string;
  variant?: Variant;
  loading?: boolean;
  fullWidth?: boolean;
  leftIcon?: React.ReactNode;
};

export function Button({
  label,
  variant = "primary",
  loading = false,
  fullWidth = true,
  leftIcon,
  disabled,
  style,
  ...rest
}: ButtonProps) {
  const isDisabled = disabled || loading;
  const v = VARIANTS[variant];

  return (
    <Pressable
      accessibilityRole="button"
      accessibilityState={{ disabled: isDisabled, busy: loading }}
      disabled={isDisabled}
      style={(state) => [
        styles.base,
        { backgroundColor: v.bg, borderColor: v.border },
        fullWidth && styles.fullWidth,
        state.pressed && !isDisabled && { backgroundColor: v.pressedBg },
        isDisabled && styles.disabled,
        typeof style === "function" ? style(state) : style,
      ]}
      {...rest}
    >
      {loading ? (
        <ActivityIndicator color={v.fg} />
      ) : (
        <View style={styles.content}>
          {leftIcon}
          <Text style={[typography.button, { color: v.fg }]}>{label}</Text>
        </View>
      )}
    </Pressable>
  );
}

const VARIANTS: Record<
  Variant,
  { bg: string; pressedBg: string; fg: string; border: string }
> = {
  primary: {
    bg: colors.primary,
    pressedBg: colors.primaryDark,
    fg: colors.textInverse,
    border: colors.primary,
  },
  secondary: {
    bg: colors.primaryLight,
    pressedBg: "#C7DBFE",
    fg: colors.primaryDark,
    border: colors.primaryLight,
  },
  outline: {
    bg: "transparent",
    pressedBg: colors.surfaceAlt,
    fg: colors.primary,
    border: colors.primary,
  },
  ghost: {
    bg: "transparent",
    pressedBg: colors.surfaceAlt,
    fg: colors.primary,
    border: "transparent",
  },
};

const styles = StyleSheet.create({
  base: {
    minHeight: 48,
    paddingHorizontal: spacing.lg,
    borderRadius: radius.md,
    borderWidth: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  fullWidth: { alignSelf: "stretch" },
  content: { flexDirection: "row", alignItems: "center", gap: spacing.sm },
  disabled: { opacity: 0.5 },
});
