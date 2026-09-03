import { useNavigation } from "@react-navigation/native";
import type { NativeStackNavigationProp } from "@react-navigation/native-stack";
import { useState } from "react";
import { FlatList, Pressable, StyleSheet, View } from "react-native";

import { Avatar, Input, Screen, Text } from "@/components/ui";
import type { RootStackParamList } from "@/navigation/types";
import { spacing } from "@/theme";

import { useUserSearch } from "../hooks/useSearch";

export function SearchScreen() {
  const nav = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const [term, setTerm] = useState("");
  const results = useUserSearch(term);

  return (
    <Screen>
      <Input
        placeholder="Search people…"
        autoCapitalize="none"
        value={term}
        onChangeText={setTerm}
      />
      <FlatList
        data={results.data ?? []}
        keyExtractor={(u) => u.id}
        contentContainerStyle={styles.list}
        keyboardShouldPersistTaps="handled"
        renderItem={({ item }) => (
          <Pressable
            style={styles.row}
            onPress={() => nav.navigate("UserProfile", { username: item.username })}
          >
            <Avatar uri={item.avatar_url} name={item.username} size={40} />
            <View>
              <Text variant="bodyStrong">{item.username}</Text>
              {item.display_name ? (
                <Text variant="caption" color="textSecondary">
                  {item.display_name}
                </Text>
              ) : null}
            </View>
          </Pressable>
        )}
        ListEmptyComponent={
          term.trim().length >= 2 && !results.isFetching ? (
            <Text color="textSecondary" style={styles.empty}>
              No people found for “{term}”.
            </Text>
          ) : null
        }
      />
    </Screen>
  );
}

const styles = StyleSheet.create({
  list: { paddingTop: spacing.md, gap: spacing.xs },
  row: { flexDirection: "row", alignItems: "center", gap: spacing.md, paddingVertical: spacing.sm },
  empty: { textAlign: "center", marginTop: spacing.xl },
});
