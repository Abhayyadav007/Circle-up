import { createBottomTabNavigator } from "@react-navigation/bottom-tabs";

import { FeedScreen } from "@/features/feed/screens/FeedScreen";
import { CreatePostScreen } from "@/features/post/screens/CreatePostScreen";
import { ProfileScreen } from "@/features/profile/screens/ProfileScreen";
import { SearchScreen } from "@/features/search/screens/SearchScreen";
import { colors } from "@/theme";

import type { MainTabParamList } from "./types";

const Tab = createBottomTabNavigator<MainTabParamList>();

export function TabNavigator() {
  return (
    <Tab.Navigator
      screenOptions={{
        headerShown: true,
        tabBarActiveTintColor: colors.primary,
        tabBarInactiveTintColor: colors.textSecondary,
        headerTintColor: colors.textPrimary,
      }}
    >
      <Tab.Screen name="Feed" component={FeedScreen} />
      <Tab.Screen name="Search" component={SearchScreen} />
      <Tab.Screen name="Create" component={CreatePostScreen} options={{ title: "New Post" }} />
      <Tab.Screen name="Profile" component={ProfileScreen} />
    </Tab.Navigator>
  );
}
