import { readFile } from 'node:fs/promises'
import vue from '@vitejs/plugin-vue'
import { type Plugin, defineConfig } from 'vite'

/** The Fluent catalogs without their comments, which are for translators: a fifth of the English one, which every
 *  page loads. A `#` starting a line is always a comment (a value's later lines are indented). */
function catalogsWithoutComments(): Plugin {
  return {
    name: 'catalogs-without-comments',
    enforce: 'pre',
    async load(id) {
      const [path, query] = id.split('?')
      if (!path.endsWith('.ftl') || query !== 'raw') return null
      const text = (await readFile(path, 'utf8'))
        .split(/\r?\n/)
        .filter((line) => !line.startsWith('#'))
        .join('\n')
        .replace(/\n{3,}/g, '\n\n')
      return `export default ${JSON.stringify(text)}`
    },
  }
}

// In dev, Vite serves the UI on :5173 and proxies the API to hoardd on :4627.
// In production, hoardd serves the built `dist/` itself, so there's no CORS to configure.
export default defineConfig({
  plugins: [catalogsWithoutComments(), vue()],
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
    // The catalogs live beside the server's, in ../locales.
    fs: { allow: ['..'] },
    proxy: {
      '/api': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
      '/mcp': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
      '/auth': process.env.HOARD_API ?? 'http://127.0.0.1:4627',
    },
  },
})
