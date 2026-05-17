import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './jest.setup.js',
coverage: {
    provider: 'v8',
    reporter: ['text', 'json', 'html', 'lcov'],
    exclude: [
      'node_modules/',
      'src/main.tsx',
      'src/vite-env.d.ts',
      '**/*.d.ts',
      'src/**/__tests__/**',
      'src/ipc-contract.integration.test.ts',
    ],
    include: [
      'src/**/*.ts',
      'src/**/*.tsx',
    ],
  },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});