import { StyleSheet, View, type ViewProps } from "react-native";
import { SafeAreaView, type Edge } from "react-native-safe-area-context";

import { colors, spacing } from "@/theme";

export type ScreenProps = ViewProps & {
  /** Add horizontal + vertical padding. Default true. */
  padded?: boolean;
  edges?: Edge[];
};

/** Standard screen container: safe-area aware, themed background. */
export function Screen({
  padded = true,
  edges = ["top", "bottom"],
  style,
  children,
  ...rest
}: ScreenProps) {
  return (
    <SafeAreaView style={styles.safe} edges={edges}>
      <View style={[styles.body, padded && styles.padded, style]} {...rest}>
        {children}
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.background },
  body: { flex: 1 },
  padded: { paddingHorizontal: spacing.lg, paddingVertical: spacing.md },
});
