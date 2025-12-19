import pluginVue from "eslint-plugin-vue";
import globals from "globals";

export default [
  // add more generic rulesets here, such as:
  // js.configs.recommended,
  ...pluginVue.configs["flat/recommended"],
  {
    rules: {
      "vue/no-unused-vars": "warn",
    },
    languageOptions: {
      sourceType: "module",
      globals: {
        ...globals.browser,
      },
    },
  },
];
