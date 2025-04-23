import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte()],
  server: {
    allowedHosts: ['in.finkegabriel.com'],
    host: true // enables access from network devices
  }
});
