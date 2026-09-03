import { Text as RNText, type TextProps as RNTextProps } from "react-native";

import { colors, typography } from "@/theme";

type Variant = keyof typeof typography;

export type TextProps = RNTextProps & {
  variant?: Variant;
  color?: keyof typeof colors;
};

/** Themed text. Always use this instead of react-native's `Text`. */
export function Text({ variant = "body", color = "textPrimary", style, ...rest }: TextProps) {
  return <RNText style={[typography[variant], { color: colors[color] }, style]} {...rest} />;
}
