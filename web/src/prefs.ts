// Per-browser view preferences (tile size, collapsed filter groups…). Stored in localStorage when it's available;
// private windows and locked-down browsers just get the defaults for the session.
import { reactive, watch } from 'vue'

export interface Prefs {
  /** Tile width in px for the library grid. */
  tile: number
  /** Show glTF buffers, .mtl files and other supporting files in the grid. */
  showSupporting: boolean
  /** Filter groups the user collapsed, by namespace. */
  collapsed: string[]
  /** Filter groups the user expanded past the first few values. */
  expanded: string[]
  /** Color theme: follow the system, or always light or dark. */
  theme: 'system' | 'light' | 'dark'
  /** Theme worn when light, by id (see LIGHT_THEMES in theme.ts). */
  lightTheme: string
  /** Theme worn when dark, by id (see DARK_THEMES in theme.ts). */
  darkTheme: string
}

const KEY = 'hoard.prefs'
const DEFAULTS: Prefs = { tile: 156, showSupporting: false, collapsed: ['license', 'format', 'blender'], expanded: [], theme: 'system', lightTheme: 'slate', darkTheme: 'graphite' }

function load(): Prefs {
  try {
    const raw = localStorage.getItem(KEY)
    return raw ? { ...DEFAULTS, ...(JSON.parse(raw) as Partial<Prefs>) } : { ...DEFAULTS }
  } catch {
    return { ...DEFAULTS }
  }
}

export const prefs = reactive<Prefs>(load())

watch(
  prefs,
  (p) => {
    try {
      localStorage.setItem(KEY, JSON.stringify(p))
    } catch {
      // Storage unavailable or full: preferences last until the tab closes.
    }
  },
  { deep: true },
)

export function toggleIn(list: string[], value: string): string[] {
  return list.includes(value) ? list.filter((v) => v !== value) : [...list, value]
}
