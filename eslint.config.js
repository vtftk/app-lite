// @ts-check

import eslintPluginSvelte from "eslint-plugin-svelte";
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import globals from "globals";

import * as svelteParser from "svelte-eslint-parser";
import * as typescriptParser from "@typescript-eslint/parser";

export default tseslint.config(
  eslint.configs.recommended,
  tseslint.configs.recommended,
  ...eslintPluginSvelte.configs["flat/recommended"],
  ...eslintPluginSvelte.configs["flat/prettier"],
  {
    files: ["**/*.svelte"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: {
          // Specify a parser for each lang.
          ts: typescriptParser,
          typescript: typescriptParser,
        },
        project: "./tsconfig.json",
        extraFileExtensions: [".svelte"],
      },
    },
  },
  {
    languageOptions: {
      globals: {
        ...globals.browser,
      },
    },
  },
);
