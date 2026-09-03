import { useNavigation, useRoute } from "@react-navigation/native";
import type { NativeStackNavigationProp } from "@react-navigation/native-stack";
import { useInfiniteQuery } from "@tanstack/react-query";
import { ActivityIndicator, FlatList, Image, Pressable, StyleSheet, View } from "react-native";

import { queryKeys } from "@/api/queryClient";
import { Avatar, Button, Screen, Text } from "@/components/ui";
import { useAuth } from "@/features/auth/hooks/useAuth";
import { postApi } from "@/features/post/api/postApi";
import { useFollow, useProfile } from "@/features/profile/hooks/useProfile";
import type { RootStackParamList } from "@/navigation/types";
import { colors, spacing } from "@/theme";

/**
 * Serves both the "Profile" tab (no param → own profile) and the "UserProfile"
 * stack screen (`{ username }`).
 */
export function ProfileScreen() {
  const route = useRoute();
  const nav = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const username = (route.params as { username?: string } | undefined)?.username;

  const { logout } = useAuth();
  const profile = useProfile(username);
  const follow = useFollow(username ?? "");

  const grid = useInfiniteQuery({
    queryKey: queryKeys.userPosts(username ?? "me"),
    queryFn: ({ pageParam }) => postApi.byUser(username ?? profile.data!.username, pageParam),
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (last) => last.next_cursor ?? undefined,
    enabled: Boolean(username) || Boolean(profile.data),
  });

  if (profile.isLoading || !profile.data) {
    return (
      <Screen>
        <ActivityIndicator color={colors.primary} style={{ marginTop: spacing.xxl }} />
      </Screen>
    );
  }

  const p = profile.data;
  const posts = grid.data?.pages.flatMap((pg) => pg.items) ?? [];

  return (
    <Screen padded={false}>
      <FlatList
        data={posts}
        keyExtractor={(item) => item.id}
        numColumns={3}
        contentContainerStyle={styles.grid}
        ListHeaderComponent={
          <View style={styles.header}>
            <View style={styles.identity}>
              <Avatar uri={p.avatar_url} name={p.username} size={72} />
              <View style={styles.counts}>
                <Stat label="Posts" value={p.counts.posts} />
                <Stat label="Followers" value={p.counts.followers} />
                <Stat label="Following" value={p.counts.following} />
              </View>
            </View>

            <Text variant="bodyStrong">{p.display_name ?? p.username}</Text>
            {p.bio ? <Text color="textSecondary">{p.bio}</Text> : null}

            {p.is_me ? (
              <Button label="Log out" variant="outline" onPress={logout} />
            ) : (
              <Button
                label={p.is_following ? "Following" : "Follow"}
                variant={p.is_following ? "secondary" : "primary"}
                loading={follow.follow.isPending || follow.unfollow.isPending}
                onPress={() =>
                  p.is_following ? follow.unfollow.mutate() : follow.follow.mutate()
                }
              />
            )}
          </View>
        }
        renderItem={({ item }) => (
          <Pressable
            style={styles.cell}
            onPress={() => nav.navigate("PostDetail", { postId: item.id })}
          >
            <Image source={{ uri: item.image_url }} style={styles.cellImage} />
          </Pressable>
        )}
        onEndReached={() => grid.hasNextPage && grid.fetchNextPage()}
        ListEmptyComponent={
          <Text color="textSecondary" style={styles.empty}>
            No posts yet.
          </Text>
        }
      />
    </Screen>
  );
}

function Stat({ label, value }: { label: string; value: number }) {
  return (
    <View style={styles.stat}>
      <Text variant="bodyStrong">{value}</Text>
      <Text variant="caption" color="textSecondary">
        {label}
      </Text>
    </View>
  );
}

const styles = StyleSheet.create({
  grid: { padding: spacing.xs },
  header: { padding: spacing.lg, gap: spacing.md },
  identity: { flexDirection: "row", alignItems: "center", gap: spacing.lg },
  counts: { flexDirection: "row", flex: 1, justifyContent: "space-around" },
  stat: { alignItems: "center" },
  cell: { flex: 1 / 3, aspectRatio: 1, padding: spacing.xs },
  cellImage: { flex: 1, borderRadius: 4, backgroundColor: colors.surfaceAlt },
  empty: { textAlign: "center", marginTop: spacing.xxl },
});
