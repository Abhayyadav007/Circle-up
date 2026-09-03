import { useState } from "react";
import { StyleSheet, TextInput, View, type TextInputProps } from "react-native";

import { colors, radius, spacing, typography } from "@/theme";

import { Text } from "./Text";

export type InputProps = TextInputProps & {
  label?: string;
  error?: string;
};

export function Input({ label, error, style, onFocus, onBlur, ...rest }: InputProps) {
  const [focused, setFocused] = useState(false);

  return (
    <View style={styles.wrap}>
      {label ? (
        <Text variant="caption" color="textSecondary" style={styles.label}>
          {label}
        </Text>
      ) : null}
      <TextInput
        placeholderTextColor={colors.textSecondary}
        style={[
          styles.input,
          typography.body,
          { color: colors.textPrimary },
          focused && styles.focused,
          !!error && styles.errored,
          style,
        ]}
        onFocus={(e) => {
          setFocused(true);
          onFocus?.(e);
        }}
        onBlur={(e) => {
          setFocused(false);
          onBlur?.(e);
        }}
        {...rest}
      />
      {error ? (
        <Text variant="caption" color="danger" style={styles.error}>
          {error}
        </Text>
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  wrap: { alignSelf: "stretch", gap: spacing.xs },
  label: { marginLeft: spacing.xs },
  input: {
    minHeight: 48,
    paddingHorizontal: spacing.md,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: radius.md,
    backgroundColor: colors.surface,
  },
  focused: { borderColor: colors.primary },
  errored: { borderColor: colors.danger },
  error: { marginLeft: spacing.xs },
});
