/// <reference types='vitest' />
import { defineConfig } from 'vite';
import webExtension from 'vite-plugin-web-extension';
import path from 'path';

export default defineConfig(() => ({
  root: __dirname,
  cacheDir: '../../node_modules/.vite/apps/browser-extension',
  server: {
    port: 4200,
    host: 'localhost',
  },
  preview: {
    port: 4300,
    host: 'localhost',
  },
  plugins: [
    webExtension({
      manifest: path.resolve(__dirname, 'manifest.json'),
      webExtConfig: {
        startUrl: ['about:debugging#/runtime/this-firefox'],
        target: 'firefox-desktop'
      }
    })
  ],
  build: {
    outDir: './dist',
    emptyOutDir: true,
    reportCompressedSize: true,
    rollupOptions: {
      input: path.resolve(__dirname, 'manifest.json'),
    },
  },
}));
