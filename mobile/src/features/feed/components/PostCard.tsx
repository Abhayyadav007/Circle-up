import { Image, Pressable, StyleSheet, View } from "react-native";

import type { PostView } from "@/api/types";
import { Avatar, Card, Text } from "@/components/ui";
import { useLike } from "@/features/likes/hooks/useLike";
import { colors, spacing } from "@/theme";

export function PostCard({
  post,
  onPressComments,
  onPressAuthor,
}: {
  post: PostView;
  onPressComments?: () => void;
  onPressAuthor?: () => void;
}) {
  const like = useLike(post.id, { liked: post.liked_by_me, count: post.like_count });

  return (
    <Card padded={false} style={styles.card}>
      <Pressable style={styles.header} onPress={onPressAuthor}>
        <Avatar uri={post.author.avatar_url} name={post.author.username} size={36} />
        <Text variant="bodyStrong">{post.author.username}</Text>
      </Pressable>

      <Image source={{ uri: post.image_url }} style={styles.image} resizeMode="cover" />

      <View style={styles.actions}>
        <Pressable onPress={like.toggle} accessibilityRole="button">
          <Text variant="bodyStrong" style={{ color: like.liked ? colors.like : colors.textPrimary }}>
            {like.liked ? "♥" : "♡"} {like.count}
          </Text>
        </Pressable>
        <Pressable onPress={onPressComments} accessibilityRole="button">
          <Text variant="bodyStrong">💬 {post.comment_count}</Text>
        </Pressable>
      </View>

      {post.caption ? (
        <Text style={styles.caption}>
          <Text variant="bodyStrong">{post.author.username} </Text>
          {post.caption}
        </Text>
      ) : null}
    </Card>
  );
}

const styles = StyleSheet.create({
  card: { overflow: "hidden" },
  header: {
    flexDirection: "row",
    alignItems: "center",
    gap: spacing.sm,
    padding: spacing.md,
  },
  image: { width: "100%", aspectRatio: 1, backgroundColor: colors.surfaceAlt },
  actions: { flexDirection: "row", gap: spacing.lg, padding: spacing.md },
  caption: { paddingHorizontal: spacing.md, paddingBottom: spacing.md },
});
