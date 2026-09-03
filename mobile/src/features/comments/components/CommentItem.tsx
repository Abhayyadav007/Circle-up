import { Pressable, StyleSheet, View } from "react-native";

import type { CommentView } from "@/api/types";
import { Avatar, Text } from "@/components/ui";
import { spacing } from "@/theme";

export function CommentItem({
  comment,
  onDelete,
}: {
  comment: CommentView;
  onDelete?: (id: string) => void;
}) {
  return (
    <View style={styles.row}>
      <Avatar uri={comment.author.avatar_url} name={comment.author.username} size={32} />
      <View style={styles.body}>
        <Text>
          <Text variant="bodyStrong">{comment.author.username} </Text>
          {comment.body}
        </Text>
      </View>
      {comment.can_delete && onDelete ? (
        <Pressable onPress={() => onDelete(comment.id)} accessibilityLabel="Delete comment">
          <Text variant="caption" color="danger">
            Delete
          </Text>
        </Pressable>
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  row: { flexDirection: "row", gap: spacing.sm, paddingVertical: spacing.sm },
  body: { flex: 1 },
});
