import { useState } from "react";
import { StyleSheet, View } from "react-native";

import type { ApiError } from "@/api/client";
import { Button, Input, Screen, Text } from "@/components/ui";
import type { RootStackScreenProps } from "@/navigation/types";
import { spacing } from "@/theme";

import { GoogleButton } from "../components/GoogleButton";
import { useAuth } from "../hooks/useAuth";

export function SignupScreen(_props: RootStackScreenProps<"Signup">) {
  const { signup, googleSignIn } = useAuth();
  const [email, setEmail] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const err = signup.error as ApiError | null;
  const fieldError = (name: string) => err?.fields?.find((f) => f.field === name)?.message;

  return (
    <Screen>
      <View style={styles.form}>
        <Input
          label="Email"
          autoCapitalize="none"
          keyboardType="email-address"
          value={email}
          onChangeText={setEmail}
          error={fieldError("email")}
        />
        <Input
          label="Username"
          autoCapitalize="none"
          value={username}
          onChangeText={setUsername}
          error={fieldError("username")}
        />
        <Input
          label="Password"
          secureTextEntry
          value={password}
          onChangeText={setPassword}
          error={fieldError("password")}
        />

        {err && !err.fields ? (
          <Text variant="caption" color="danger">
            {err.message}
          </Text>
        ) : null}

        <Button
          label="Sign up"
          loading={signup.isPending}
          onPress={() => signup.mutate({ email, username, password })}
        />
        <GoogleButton loading={googleSignIn.isPending} onPress={() => googleSignIn.mutate()} />
      </View>
    </Screen>
  );
}

const styles = StyleSheet.create({
  form: { gap: spacing.md, marginTop: spacing.lg },
});
