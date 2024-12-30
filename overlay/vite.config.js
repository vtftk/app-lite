import { defineConfig } from "vite";
import path from "path";
import { viteSingleFile } from "vite-plugin-singlefile";

export default defineConfig(async () => ({
  plugins: [viteSingleFile()],
  resolve: {
    alias: {
      // eslint-disable-next-line no-undef
      $shared: path.resolve(__dirname, "../types"),
    },
  },
}));
