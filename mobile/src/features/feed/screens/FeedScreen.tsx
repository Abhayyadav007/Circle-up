import { useNavigation } from "@react-navigation/native";
import type { NativeStackNavigationProp } from "@react-navigation/native-stack";
import { ActivityIndicator, FlatList, RefreshControl, StyleSheet, View } from "react-native";

import { Screen, Text } from "@/components/ui";
import { PostCard } from "@/features/feed/components/PostCard";
import { useFeed } from "@/features/feed/hooks/useFeed";
import type { RootStackParamList } from "@/navigation/types";
import { colors, spacing } from "@/theme";

export function FeedScreen() {
  const nav = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const feed = useFeed("home");

  if (feed.isLoading) {
    return (
      <Screen>
        <ActivityIndicator color={colors.primary} style={{ marginTop: spacing.xxl }} />
      </Screen>
    );
  }

  return (
    <Screen padded={false}>
      <FlatList
        data={feed.posts}
        keyExtractor={(p) => p.id}
        contentContainerStyle={styles.list}
        ItemSeparatorComponent={() => <View style={{ height: spacing.lg }} />}
        renderItem={({ item }) => (
          <PostCard
            post={item}
            onPressComments={() => nav.navigate("PostDetail", { postId: item.id })}
            onPressAuthor={() => nav.navigate("UserProfile", { username: item.author.username })}
          />
        )}
        refreshControl={
          <RefreshControl refreshing={feed.isRefetching} onRefresh={feed.refetch} tintColor={colors.primary} />
        }
        onEndReachedThreshold={0.5}
        onEndReached={() => feed.hasNextPage && feed.fetchNextPage()}
        ListEmptyComponent={
          <Text variant="body" color="textSecondary" style={styles.empty}>
            Your feed is quiet. Follow some people or share your first post.
          </Text>
        }
        ListFooterComponent={
          feed.isFetchingNextPage ? <ActivityIndicator color={colors.primary} /> : null
        }
      />
    </Screen>
  );
}

const styles = StyleSheet.create({
  list: { padding: spacing.lg },
  empty: { textAlign: "center", marginTop: spacing.xxl },
});
