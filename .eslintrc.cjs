/* eslint-env node */
require('@rushstack/eslint-patch/modern-module-resolution');

module.exports = {
  root: true,
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser',
    ecmaVersion: 'latest',
  },
  extends: ['plugin:vue/vue3-essential', 'eslint:recommended'],
  plugins: ['@typescript-eslint'],
};
