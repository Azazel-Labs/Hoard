// Light, dark or follow the system, plus which theme to wear for each. The resolved scheme goes on <html> as
// `data-scheme`, which sets `color-scheme` in styles.css: every light-dark() color follows it, along with form
// controls and scrollbars. The chosen themes go on as `data-light` and `data-dark`, which styles.css turns into the
// --l-* and --d-* colors. index.html applies the saved choices before first paint; this keeps them current.
import { ref, watch } from 'vue'
import { prefs } from './prefs'

export interface ThemeInfo {
  id: string
  name: string
  /** A few words on the feel, for Settings. */
  blurb: string
}

// The first of each list is the default. Keep in step with the [data-light] / [data-dark] blocks in styles.css.
export const LIGHT_THEMES: ThemeInfo[] = [
  { id: 'slate', name: 'Slate', blurb: 'Cool blue-grey' },
  { id: 'paper', name: 'Paper', blurb: 'Plain neutral grey' },
  { id: 'linen', name: 'Linen', blurb: 'Warm cream, amber' },
  { id: 'sage', name: 'Sage', blurb: 'Soft green' },
  { id: 'lilac', name: 'Lilac', blurb: 'Pale violet' },
  { id: 'blush', name: 'Blush', blurb: 'Dusty rose' },
]
export const DARK_THEMES: ThemeInfo[] = [
  { id: 'graphite', name: 'Graphite', blurb: 'Neutral grey, no tint' },
  { id: 'slate', name: 'Slate', blurb: 'Blue-grey' },
  { id: 'obsidian', name: 'Obsidian', blurb: 'Near black, for OLED' },
  { id: 'ember', name: 'Ember', blurb: 'Warm charcoal, amber' },
  { id: 'moss', name: 'Moss', blurb: 'Forest green-black' },
  { id: 'aubergine', name: 'Aubergine', blurb: 'Plum-black, lilac' },
  { id: 'oxblood', name: 'Oxblood', blurb: 'Wine-black, coral' },
]

/** The theme in use for a scheme: the saved one, or the default when it's unknown (renamed, removed). */
export function themeFor(scheme: 'light' | 'dark'): ThemeInfo {
  const list = scheme === 'light' ? LIGHT_THEMES : DARK_THEMES
  const id = scheme === 'light' ? prefs.lightTheme : prefs.darkTheme
  return list.find((t) => t.id === id) ?? list[0]
}

const lightQuery = window.matchMedia('(prefers-color-scheme: light)')

/** Light or dark as chosen in the top bar, the system's answer when following it. */
export function usedScheme(): 'light' | 'dark' {
  return prefs.theme === 'light' || prefs.theme === 'dark' ? prefs.theme : lightQuery.matches ? 'light' : 'dark'
}

/**
 * Worn instead of the scheme in use while Settings shows off a theme: pick a light theme while it's dark and the
 * page turns light so you can see it. Settings clears it when you leave, and the page goes back.
 */
export const previewScheme = ref<'light' | 'dark' | null>(null)

function apply() {
  const root = document.documentElement
  root.dataset.scheme = previewScheme.value ?? usedScheme()
  root.dataset.light = themeFor('light').id
  root.dataset.dark = themeFor('dark').id
}

// Choosing light, dark or system in the top bar ends a preview.
watch(() => prefs.theme, () => (previewScheme.value = null))
watch(() => [prefs.theme, prefs.lightTheme, prefs.darkTheme, previewScheme.value], apply, { immediate: true })
lightQuery.addEventListener('change', apply)
