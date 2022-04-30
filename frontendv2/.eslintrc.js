module.exports = {
  root: true,
  extends: [
    'eslint:recommended',
    'plugin:react/recommended',
    'plugin:@typescript-eslint/recommended',
  ],
  parser: '@typescript-eslint/parser',
  plugins: ['@typescript-eslint', 'react'],
  ignorePatterns: ['src/**/*.test.ts', 'src/gen/*'],
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 'latest',
    sourceType: 'module',
  },
  rules: {
    'no-console': 'error',
    'prefer-const': 'error',
    'prefer-template': 'error',
    'no-duplicate-imports': 'error',
    'no-self-compare': 'error',
    'react/react-in-jsx-scope': 'off',
  },
  settings: {
    react: {
      version: '18.0',
    },
  },
};
