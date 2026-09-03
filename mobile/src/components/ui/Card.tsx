import { View, StyleSheet, type ViewProps } from "react-native";

import { colors, radius, spacing } from "@/theme";

export type CardProps = ViewProps & {
  padded?: boolean;
};

export function Card({ padded = true, style, ...rest }: CardProps) {
  return <View style={[styles.card, padded && styles.padded, style]} {...rest} />;
}

const styles = StyleSheet.create({
  card: {
    backgroundColor: colors.surface,
    borderRadius: radius.lg,
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: colors.border,
  },
  padded: { padding: spacing.lg },
});
