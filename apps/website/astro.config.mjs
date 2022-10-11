import { defineConfig } from 'astro/config';

import vue from "@astrojs/vue";
import tailwind from "@astrojs/tailwind";
import sitemap from "@astrojs/sitemap";

import { name, version } from './package.json';

// https://astro.build/config
export default defineConfig({
  integrations: [vue(), tailwind(), sitemap()],
  vite: {
    server: {
      port: 8002
    },
    root: 'src',
    publicDir: '../../packages/interface/src/assets',
    define: {
      pkgJson: { name, version }
    },
    build: {
      outDir: '../dist',
      assetsDir: '.'
    },
  },
});