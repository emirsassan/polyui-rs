import vue from '@vitejs/plugin-vue';
import { resolve } from 'path';
import { defineConfig } from 'vite';

import { name, version } from './package.json';

export default defineConfig({
	server: {
		port: 8001
	},
	plugins: [vue()],
	root: 'src',
	publicDir: '../../packages/interface/src/assets',
	define: {
		pkgJson: { name, version }
	},
	build: {
		outDir: '../dist',
		assetsDir: '.',
		target: ['es2021', 'chrome100', 'safari13'],
		minify: !!!process.env.TAURI_DEBUG,
		sourcemap: !!process.env.TAURI_DEBUG,
		emptyOutDir: true
	},
	clearScreen: false,
	envPrefix: ['VITE_', 'TAURI_']
});
