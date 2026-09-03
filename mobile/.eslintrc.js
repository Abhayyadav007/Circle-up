module.exports = {
  root: true,
  extends: ["expo"],
  ignorePatterns: ["dist/", "node_modules/", "*.config.js"],
  rules: {
    "import/order": [
      "warn",
      {
        groups: ["builtin", "external", "internal", "parent", "sibling"],
        "newlines-between": "always",
        alphabetize: { order: "asc" },
      },
    ],
  },
};
