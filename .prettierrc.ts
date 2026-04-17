import type { Config } from "prettier";

/** @see https://prettier.io/docs/configuration */
const config: Config = {
  tabWidth: 2,
  printWidth: 120,
  semi: true,
  overrides: [
    {
      files: "*.vue",
      options: { tabWidth: 4 },
    },
  ],
};

export default config;
