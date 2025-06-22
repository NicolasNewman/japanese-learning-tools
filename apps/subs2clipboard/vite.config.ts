/// <reference types='vitest' />
import { defineConfig } from 'vite';
import webExtension from 'vite-plugin-web-extension';
import path from 'path';

export default defineConfig({
  root: __dirname,
  cacheDir: '../../node_modules/.vite/apps/subs2clipboard',
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
      },
      additionalInputs: [
        'src/menu/index.ts',
      ]
    })
  ],
  build: {
    outDir: '../../dist/subs2clipboard',
    emptyOutDir: true,
    // reportCompressedSize: true,
    // rollupOptions: {
    //   input: path.resolve(__dirname, 'manifest.json'),
    // },
  },
});
