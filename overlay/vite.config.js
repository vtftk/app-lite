import { defineConfig } from "vite";
import path from "path";

export default defineConfig(async () => ({
  resolve: {
    alias: {
      $shared: path.resolve(__dirname, "../types"),
    },
  },
}));
