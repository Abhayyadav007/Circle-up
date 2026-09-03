import { useState } from "react";
import { StyleSheet, View } from "react-native";

import type { ApiError } from "@/api/client";
import { Button, Input, Screen, Text } from "@/components/ui";
import type { RootStackScreenProps } from "@/navigation/types";
import { spacing } from "@/theme";

import { GoogleButton } from "../components/GoogleButton";
import { useAuth } from "../hooks/useAuth";

export function LoginScreen({ navigation }: RootStackScreenProps<"Login">) {
  const { login, googleSignIn } = useAuth();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const error = (login.error ?? googleSignIn.error) as ApiError | Error | null;

  return (
    <Screen>
      <View style={styles.hero}>
        <Text variant="display" color="primary">
          Circleup
        </Text>
        <Text variant="body" color="textSecondary">
          Share your world, one circle at a time.
        </Text>
      </View>

      <View style={styles.form}>
        <Input
          label="Email"
          autoCapitalize="none"
          keyboardType="email-address"
          value={email}
          onChangeText={setEmail}
        />
        <Input label="Password" secureTextEntry value={password} onChangeText={setPassword} />

        {error ? (
          <Text variant="caption" color="danger">
            {"message" in error ? error.message : "Login failed"}
          </Text>
        ) : null}

        <Button
          label="Log in"
          loading={login.isPending}
          onPress={() => login.mutate({ email, password })}
        />
        <GoogleButton loading={googleSignIn.isPending} onPress={() => googleSignIn.mutate()} />
      </View>

      <View style={styles.footer}>
        <Text variant="caption" color="textSecondary">
          New to Circleup?
        </Text>
        <Button label="Create an account" variant="ghost" onPress={() => navigation.navigate("Signup")} />
      </View>
    </Screen>
  );
}

const styles = StyleSheet.create({
  hero: { gap: spacing.sm, marginTop: spacing.xxxl, marginBottom: spacing.xxl },
  form: { gap: spacing.md, flex: 1 },
  footer: { alignItems: "center", gap: spacing.xs },
});
