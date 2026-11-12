import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

// In dev, Vite serves the UI on :5173 and proxies the API to hoardd on :4627.
// In production, hoardd serves the built `dist/` itself, so there's no CORS to configure.
export default defineConfig({
  plugins: [vue()],
  build: {
    rolldownOptions: {
      output: {
        // Vue itself, shared by the first page and the parts that load later, gets a name of its own.
        codeSplitting: { groups: [{ name: 'vue', test: /node_modules[\\/]@?vue[\\/]/ }] },
      },
    },
    // three.js with its WebGPU renderer is one ~650 kB chunk, loaded only when a 3D viewer opens.
    chunkSizeWarningLimit: 700,
  },
  server: {
    proxy: {
      '/api': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
      '/mcp': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
      '/auth': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
    },
  },
})
