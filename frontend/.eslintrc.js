module.exports = {
  env: {
    browser: true,
    es2021: true
  },
  extends: [
    "airbnb",
    "airbnb/hooks",
    "airbnb-typescript",
    "prettier"
  ],
  overrides: [
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: 'tsconfig.json'
  },
  plugins: [
    'react',
    'prettier'
  ],
  rules: {
    "@typescript-eslint/explicit-function-return-type": "off",
  },
  ignorePatterns: [
    ".eslintrc.js"
  ]
};
