import { defineConfig } from 'astro/config';

// Ponytail: hello world, static output, mismo puerto que ya espera Tauri.
export default defineConfig({
  output: 'static',
  server: { port: 1420, host: true },
});
