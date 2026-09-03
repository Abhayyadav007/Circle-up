import { Image, StyleSheet, View } from "react-native";

import { colors, palette, radius } from "@/theme";

import { Text } from "./Text";

export type AvatarProps = {
  uri?: string | null;
  name?: string | null;
  size?: number;
};

export function Avatar({ uri, name, size = 40 }: AvatarProps) {
  const dimensions = { width: size, height: size, borderRadius: radius.pill };

  if (uri) {
    return <Image source={{ uri }} style={[styles.base, dimensions]} />;
  }

  const initial = (name?.trim()?.[0] ?? "?").toUpperCase();
  return (
    <View style={[styles.base, styles.fallback, dimensions]}>
      <Text style={{ color: colors.textInverse, fontSize: size * 0.4, fontWeight: "700" }}>
        {initial}
      </Text>
    </View>
  );
}

const styles = StyleSheet.create({
  base: { backgroundColor: colors.surfaceAlt },
  fallback: {
    backgroundColor: palette.blue500,
    alignItems: "center",
    justifyContent: "center",
  },
});
