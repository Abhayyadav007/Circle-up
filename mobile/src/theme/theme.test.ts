import { colors, palette, spacing } from "./theme";

describe("theme", () => {
  it("uses a blue primary", () => {
    expect(colors.primary).toBe(palette.blue600);
  });

  it("exposes a monotonic spacing scale", () => {
    const values = [spacing.xs, spacing.sm, spacing.md, spacing.lg, spacing.xl, spacing.xxl];
    const sorted = [...values].sort((a, b) => a - b);
    expect(values).toEqual(sorted);
  });
});
