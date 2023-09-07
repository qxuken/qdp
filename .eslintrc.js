module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/stylistic',
    'plugin:prettier/recommended',
  ],
  ignorePatterns: ['/*', '!/src'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    project: true,
  },

  plugins: ['@typescript-eslint', 'prettier'],
  rules: {
    'prefer-const': 'off',
    '@typescript-eslint/consistent-type-imports': 'error',
    '@typescript-eslint/array-type': ['error', 'generic'],
  },
};
