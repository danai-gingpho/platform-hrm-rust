import { sveltekit } from '@sveltejs/kit/vite';
import { rsvelte } from '@rsvelte/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		rsvelte(),
		sveltekit()
	],
	server: {
		port: 3001, // Different port from frontend_platform
		proxy: {
			'/api': {
				target: 'http://localhost:8000',
				changeOrigin: true
			}
		}
	}
});
