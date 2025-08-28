/// <reference types='vitest' />
import { defineConfig } from 'vite';
import webExtension from 'vite-plugin-web-extension';
import path from 'path';

// const isRemote = process.env.SSH_CLIENT || process.env.SSH_TTY;

// if (isRemote) {
//   console.log('Pre-warming Firefox for remote development...');
// }

// const args = isRemote ? [
//   // '--no-sandbox',
//   '--disable-gpu',
//   // '--disable-software-rasterizer',
//   // '--disable-background-timer-throttling',
//   // '--disable-backgrounding-occluded-windows',
//   // '--disable-renderer-backgrounding',
//   // '--disable-dev-shm-usage',
//   // '--disable-seccomp-filter-sandbox',
//   // '--no-first-run',  // Skip first-run setup
//   // '--no-default-browser-check',  // Skip browser check
//   '--disable-default-apps',  // Faster startup
//   '--disable-background-networking',  // Reduce startup load
// ] : [];


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
        target: 'firefox-desktop',
        // args,
        // keepProfileChanges: true,
      },
      // htmlViteConfig: {
      //   build: {
      //     watch: {
      //       include: ['src/menu/index.html', 'src/menu/index.ts', 'src/menu/styles.scss'],
      //     },
      //   }
      // },
      additionalInputs: [
        'src/menu/index.ts',
        'src/lib/index.ts',
        'src/menu/styles.scss',
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
  css: {
     preprocessorOptions: {
        scss: {
          silenceDeprecations: [
            'import',
            'mixed-decls',
            'color-functions',
            'global-builtin',
          ],
        },
     },
  },
});
