import { ActivityIndicator, FlatList, StyleSheet, View } from "react-native";

import { Screen, Text } from "@/components/ui";
import { CommentComposer } from "@/features/comments/components/CommentComposer";
import { CommentItem } from "@/features/comments/components/CommentItem";
import { useComments } from "@/features/comments/hooks/useComments";
import { PostCard } from "@/features/feed/components/PostCard";
import type { RootStackScreenProps } from "@/navigation/types";
import { colors, spacing } from "@/theme";

import { usePost } from "../hooks/usePost";

export function PostDetailScreen({ route }: RootStackScreenProps<"PostDetail">) {
  const { postId } = route.params;
  const post = usePost(postId);
  const comments = useComments(postId);

  if (post.isLoading) {
    return (
      <Screen>
        <ActivityIndicator color={colors.primary} style={{ marginTop: spacing.xxl }} />
      </Screen>
    );
  }

  if (post.isError || !post.data) {
    return (
      <Screen>
        <Text color="danger">Couldn&apos;t load this post.</Text>
      </Screen>
    );
  }

  return (
    <Screen padded={false}>
      <FlatList
        data={comments.comments}
        keyExtractor={(c) => c.id}
        contentContainerStyle={styles.list}
        ListHeaderComponent={
          <View style={styles.header}>
            <PostCard post={post.data} />
            <Text variant="heading">Comments</Text>
          </View>
        }
        renderItem={({ item }) => (
          <CommentItem comment={item} onDelete={(id) => comments.remove.mutate(id)} />
        )}
        onEndReached={() => comments.hasNextPage && comments.fetchNextPage()}
        ListEmptyComponent={
          <Text color="textSecondary">No comments yet — say something nice.</Text>
        }
      />
      <View style={styles.composer}>
        <CommentComposer
          pending={comments.add.isPending}
          onSubmit={(body) => comments.add.mutate(body)}
        />
      </View>
    </Screen>
  );
}

const styles = StyleSheet.create({
  list: { padding: spacing.lg, gap: spacing.xs },
  header: { gap: spacing.lg, marginBottom: spacing.md },
  composer: {
    padding: spacing.md,
    borderTopWidth: StyleSheet.hairlineWidth,
    borderTopColor: colors.border,
    backgroundColor: colors.surface,
  },
});
