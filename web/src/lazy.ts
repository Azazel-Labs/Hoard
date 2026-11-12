// Parts of the app most visits never open (settings, the archive, the first-run wizard, the 3D viewers) load the first
// time they're shown, keeping the first page load small. While one loads it says so in its place; if it can't load
// (the server was updated and the old file is gone), it says that, with a reload.
import { defineAsyncComponent, h, type Component } from 'vue'

export function lazy<T extends Component>(load: () => Promise<{ default: T }>, what: string, cls = ''): T {
  return defineAsyncComponent<T>({
    loader: load,
    delay: 150,
    loadingComponent: { render: () => h('p', { class: ['lazy-msg', cls], role: 'status' }, `Loading ${what}…`) },
    errorComponent: {
      render: () =>
        h('p', { class: ['lazy-msg', 'lazy-err', cls], role: 'alert' }, [
          `Couldn't load ${what}. `,
          h('button', { type: 'button', class: 'btn ghost', onClick: () => location.reload() }, 'Reload the page'),
        ]),
    },
  })
}
