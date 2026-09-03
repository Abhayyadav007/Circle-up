import { NavigationContainer } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";
import { ActivityIndicator, View } from "react-native";

import { LoginScreen } from "@/features/auth/screens/LoginScreen";
import { SignupScreen } from "@/features/auth/screens/SignupScreen";
import { PostDetailScreen } from "@/features/post/screens/PostDetailScreen";
import { ProfileScreen } from "@/features/profile/screens/ProfileScreen";
import { useAuthStore } from "@/store/authStore";
import { colors } from "@/theme";

import { TabNavigator } from "./TabNavigator";
import type { RootStackParamList } from "./types";

const Stack = createNativeStackNavigator<RootStackParamList>();

export function RootNavigator() {
  const hydrating = useAuthStore((s) => s.hydrating);
  const isAuthenticated = useAuthStore((s) => s.isAuthenticated);

  if (hydrating) {
    return (
      <View style={{ flex: 1, alignItems: "center", justifyContent: "center" }}>
        <ActivityIndicator color={colors.primary} size="large" />
      </View>
    );
  }

  return (
    <NavigationContainer>
      <Stack.Navigator screenOptions={{ headerTintColor: colors.textPrimary }}>
        {isAuthenticated ? (
          <>
            <Stack.Screen name="Tabs" component={TabNavigator} options={{ headerShown: false }} />
            <Stack.Screen
              name="PostDetail"
              component={PostDetailScreen}
              options={{ title: "Post" }}
            />
            <Stack.Screen
              name="UserProfile"
              component={ProfileScreen}
              options={{ title: "Profile" }}
            />
          </>
        ) : (
          <>
            <Stack.Screen name="Login" component={LoginScreen} options={{ headerShown: false }} />
            <Stack.Screen name="Signup" component={SignupScreen} options={{ title: "Create account" }} />
          </>
        )}
      </Stack.Navigator>
    </NavigationContainer>
  );
}
