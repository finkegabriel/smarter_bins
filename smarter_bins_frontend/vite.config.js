import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
  optimizeDeps: {
    exclude: ['<problematic-package-name>'],
  },
  server: {
    allowedHosts: ['in.finkegabriel.com'],
    host: true // enables access from network devices
  }
});
