import { useState } from "react";
import { StyleSheet, View } from "react-native";

import { Button, Input } from "@/components/ui";
import { spacing } from "@/theme";

export function CommentComposer({
  onSubmit,
  pending,
}: {
  onSubmit: (body: string) => void;
  pending?: boolean;
}) {
  const [text, setText] = useState("");

  return (
    <View style={styles.row}>
      <View style={styles.input}>
        <Input placeholder="Add a comment…" value={text} onChangeText={setText} />
      </View>
      <Button
        label="Post"
        fullWidth={false}
        loading={pending}
        disabled={text.trim().length === 0}
        onPress={() => {
          onSubmit(text.trim());
          setText("");
        }}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  row: { flexDirection: "row", gap: spacing.sm, alignItems: "flex-start" },
  input: { flex: 1 },
});
