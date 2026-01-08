import { defineConfig } from "vitest/config";
import path from "path";

export default defineConfig({
  test: {
    alias: {
      "taffy-js": path.resolve(__dirname, "./src/index.ts"),
    },
  },
});
