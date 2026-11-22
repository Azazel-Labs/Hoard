// Light, dark or follow the system, plus which theme to wear for each. The resolved scheme goes on <html> as
// `data-scheme`, which sets `color-scheme` in styles.css: every light-dark() color follows it, along with form
// controls and scrollbars. The chosen themes go on as `data-light` and `data-dark`, which styles.css turns into the
// --l-* and --d-* colors. index.html applies the saved choices before first paint; this keeps them current.
import { ref, watch } from 'vue'
import { t } from './i18n'
import { prefs } from './prefs'
import { withTransition } from './ui'

export interface ThemeInfo {
  id: string
  /** In the reader's language (a getter, so it follows a change of language). */
  readonly name: string
  /** A few words on the feel, for Settings. */
  readonly blurb: string
}

// The first of each list is the default. Keep in step with the [data-light] / [data-dark] blocks in styles.css.
export const LIGHT_THEMES: ThemeInfo[] = [
  { id: 'slate', get name() { return t('theme-slate') }, get blurb() { return t('theme-light-slate-blurb') } },
  { id: 'paper', get name() { return t('theme-paper') }, get blurb() { return t('theme-light-paper-blurb') } },
  { id: 'linen', get name() { return t('theme-linen') }, get blurb() { return t('theme-light-linen-blurb') } },
  { id: 'sage', get name() { return t('theme-sage') }, get blurb() { return t('theme-light-sage-blurb') } },
  { id: 'lilac', get name() { return t('theme-lilac') }, get blurb() { return t('theme-light-lilac-blurb') } },
  { id: 'blush', get name() { return t('theme-blush') }, get blurb() { return t('theme-light-blush-blurb') } },
]
export const DARK_THEMES: ThemeInfo[] = [
  { id: 'graphite', get name() { return t('theme-graphite') }, get blurb() { return t('theme-dark-graphite-blurb') } },
  { id: 'slate', get name() { return t('theme-slate') }, get blurb() { return t('theme-dark-slate-blurb') } },
  { id: 'obsidian', get name() { return t('theme-obsidian') }, get blurb() { return t('theme-dark-obsidian-blurb') } },
  { id: 'ember', get name() { return t('theme-ember') }, get blurb() { return t('theme-dark-ember-blurb') } },
  { id: 'moss', get name() { return t('theme-moss') }, get blurb() { return t('theme-dark-moss-blurb') } },
  { id: 'aubergine', get name() { return t('theme-aubergine') }, get blurb() { return t('theme-dark-aubergine-blurb') } },
  { id: 'oxblood', get name() { return t('theme-oxblood') }, get blurb() { return t('theme-dark-oxblood-blurb') } },
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
// Text size and density (Settings → Appearance) go on <html> too, for styles.css and the components to follow.
watch(
  () => [prefs.textSize, prefs.density] as const,
  ([text, density]) => Object.assign(document.documentElement.dataset, { text, density }),
  { immediate: true },
)

/** Light, dark, or whichever the system is. */
export const SCHEMES = [
  { value: 'system', label: () => t('app-theme-system') },
  { value: 'light', label: () => t('app-theme-light') },
  { value: 'dark', label: () => t('app-theme-dark') },
] as const

/** Change the theme with the new one spreading out in a circle from the button that was pressed. */
export function setTheme(value: typeof prefs.theme, e?: MouseEvent) {
  if (prefs.theme === value) return
  // From the palette there's no button: spread from the theme switcher's current button instead.
  const from = (e?.currentTarget as HTMLElement | undefined) ?? document.querySelector('.theme [aria-pressed="true"]')
  const r = from?.getBoundingClientRect() ?? new DOMRect(innerWidth / 2, innerHeight / 2, 0, 0)
  const x = r.left + r.width / 2
  const y = r.top + r.height / 2
  const reach = Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y))
  void withTransition(() => (prefs.theme = value), 'theme', () => {
    document.documentElement.animate(
      { clipPath: [`circle(0 at ${x}px ${y}px)`, `circle(${reach}px at ${x}px ${y}px)`] },
      { duration: 520, easing: 'cubic-bezier(0.65, 0, 0.35, 1)', pseudoElement: '::view-transition-new(root)' },
    )
  })
}
