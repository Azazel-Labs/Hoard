# Hoard web portal

Vue 3 + TypeScript + Vite. `npm run dev` serves on :5173 and proxies `/api` and `/mcp` to `hoardd` on :4627
(override with `HOARD_API=http://host:port`). `npm run build` writes `dist/`, which `hoardd` serves itself.

The three.js model viewer (`src/components/ModelViewer.vue`) is lazy-loaded the first time a model is opened.
