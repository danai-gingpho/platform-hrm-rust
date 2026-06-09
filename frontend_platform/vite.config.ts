import { sveltekit } from '@sveltejs/kit/vite';
import { rsvelte } from '@rsvelte/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		// Replace the standard svelte() plugin with rsvelte()
		rsvelte(),
		sveltekit()
	],
	server: {
		port: 3000,
		proxy: {
			'/api': {
				target: 'http://localhost:8000',
				changeOrigin: true
			}
		}
	}
});
