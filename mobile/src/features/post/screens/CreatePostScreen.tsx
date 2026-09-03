import { useNavigation } from "@react-navigation/native";
import * as ImagePicker from "expo-image-picker";
import { useState } from "react";
import { Image, Pressable, StyleSheet } from "react-native";

import { Button, Input, Screen, Text } from "@/components/ui";
import { colors, radius, spacing } from "@/theme";

import { useCreatePost } from "../hooks/useCreatePost";

export function CreatePostScreen() {
  const nav = useNavigation();
  const create = useCreatePost();
  const [asset, setAsset] = useState<ImagePicker.ImagePickerAsset | null>(null);
  const [caption, setCaption] = useState("");

  async function pickImage() {
    const result = await ImagePicker.launchImageLibraryAsync({
      mediaTypes: ImagePicker.MediaTypeOptions.Images,
      quality: 0.9,
      allowsEditing: true,
      aspect: [1, 1],
    });
    if (!result.canceled) {
      setAsset(result.assets[0] ?? null);
    }
  }

  function submit() {
    if (!asset) return;
    create.mutate(
      {
        fileUri: asset.uri,
        contentType: asset.mimeType ?? "image/jpeg",
        caption: caption.trim(),
      },
      {
        onSuccess: () => {
          setAsset(null);
          setCaption("");
          nav.goBack();
        },
      },
    );
  }

  return (
    <Screen>
      <Pressable style={styles.picker} onPress={pickImage}>
        {asset ? (
          <Image source={{ uri: asset.uri }} style={styles.preview} />
        ) : (
          <Text color="textSecondary">Tap to choose a photo</Text>
        )}
      </Pressable>

      <Input
        label="Caption"
        placeholder="Write a caption…"
        multiline
        value={caption}
        onChangeText={setCaption}
        style={styles.caption}
      />

      {create.isError ? (
        <Text variant="caption" color="danger">
          Couldn&apos;t share your post. Try again.
        </Text>
      ) : null}

      <Button
        label="Share"
        loading={create.isPending}
        disabled={!asset}
        onPress={submit}
      />
    </Screen>
  );
}

const styles = StyleSheet.create({
  picker: {
    aspectRatio: 1,
    borderRadius: radius.lg,
    borderWidth: 1,
    borderColor: colors.border,
    borderStyle: "dashed",
    alignItems: "center",
    justifyContent: "center",
    backgroundColor: colors.surfaceAlt,
    marginBottom: spacing.lg,
    overflow: "hidden",
  },
  preview: { width: "100%", height: "100%" },
  caption: { minHeight: 80, textAlignVertical: "top", paddingTop: spacing.sm },
});
