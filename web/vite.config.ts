import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

// In dev, Vite serves the UI on :5173 and proxies the API to hoardd on :4627.
// In production, hoardd serves the built `dist/` itself, so there's no CORS to configure.
export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/api': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
      '/mcp': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
      '/auth': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
    },
  },
})
