import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";
import vueTsConfig from "@vue/eslint-config-typescript";
import vuePrettierConfig from "@vue/eslint-config-prettier";

export default [
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs["flat/recommended"],
  ...vueTsConfig(),
  vuePrettierConfig,
  { rules: { "no-var": "error" } },
];
