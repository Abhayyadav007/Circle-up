import { Button } from "@/components/ui";

export function GoogleButton({
  onPress,
  loading,
}: {
  onPress: () => void;
  loading?: boolean;
}) {
  return (
    <Button
      label="Continue with Google"
      variant="outline"
      loading={loading}
      onPress={onPress}
      accessibilityLabel="Continue with Google"
    />
  );
}
