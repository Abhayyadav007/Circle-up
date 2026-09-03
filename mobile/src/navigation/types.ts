import type { NavigatorScreenParams } from "@react-navigation/native";
import type { NativeStackScreenProps } from "@react-navigation/native-stack";

/** Bottom tabs shown once the user is authenticated. */
export type MainTabParamList = {
  Feed: undefined;
  Search: undefined;
  Create: undefined;
  Profile: { username?: string } | undefined;
};

/** The app-wide stack. */
export type RootStackParamList = {
  // Auth flow
  Login: undefined;
  Signup: undefined;
  // Main app
  Tabs: NavigatorScreenParams<MainTabParamList>;
  PostDetail: { postId: string };
  UserProfile: { username: string };
};

export type RootStackScreenProps<T extends keyof RootStackParamList> = NativeStackScreenProps<
  RootStackParamList,
  T
>;

declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace ReactNavigation {
    interface RootParamList extends RootStackParamList {}
  }
}
