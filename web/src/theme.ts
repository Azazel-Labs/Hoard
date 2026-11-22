// Light, dark or follow the system, plus which theme to wear for each. The resolved scheme goes on <html> as
// `data-scheme`, which sets `color-scheme` in styles.css: every light-dark() color follows it, along with form
// controls and scrollbars. The chosen themes go on as `data-light` and `data-dark`, which styles.css turns into the
// --l-* and --d-* colors. index.html applies the saved choices before first paint; this keeps them current.
import { ref, watch } from 'vue'
import { t } from './i18n'
import { prefs } from './prefs'
import { reducedMotion, withTransition } from './ui'

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
  { id: 'sky', get name() { return t('theme-sky') }, get blurb() { return t('theme-light-sky-blurb') } },
  { id: 'mint', get name() { return t('theme-mint') }, get blurb() { return t('theme-light-mint-blurb') } },
  { id: 'apricot', get name() { return t('theme-apricot') }, get blurb() { return t('theme-light-apricot-blurb') } },
  { id: 'lemon', get name() { return t('theme-lemon') }, get blurb() { return t('theme-light-lemon-blurb') } },
]
export const DARK_THEMES: ThemeInfo[] = [
  { id: 'graphite', get name() { return t('theme-graphite') }, get blurb() { return t('theme-dark-graphite-blurb') } },
  { id: 'slate', get name() { return t('theme-slate') }, get blurb() { return t('theme-dark-slate-blurb') } },
  { id: 'obsidian', get name() { return t('theme-obsidian') }, get blurb() { return t('theme-dark-obsidian-blurb') } },
  { id: 'ember', get name() { return t('theme-ember') }, get blurb() { return t('theme-dark-ember-blurb') } },
  { id: 'moss', get name() { return t('theme-moss') }, get blurb() { return t('theme-dark-moss-blurb') } },
  { id: 'aubergine', get name() { return t('theme-aubergine') }, get blurb() { return t('theme-dark-aubergine-blurb') } },
  { id: 'oxblood', get name() { return t('theme-oxblood') }, get blurb() { return t('theme-dark-oxblood-blurb') } },
  { id: 'midnight', get name() { return t('theme-midnight') }, get blurb() { return t('theme-dark-midnight-blurb') } },
  { id: 'lagoon', get name() { return t('theme-lagoon') }, get blurb() { return t('theme-dark-lagoon-blurb') } },
  { id: 'neon', get name() { return t('theme-neon') }, get blurb() { return t('theme-dark-neon-blurb') } },
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

/**
 * Run a theme change as a reveal: the new look spreads out in a circle from `from`, which pops as it lands, and a ring
 * of the new accent splashes out from it once the page settles. With reduced motion (or no view transitions) it just
 * changes.
 */
export function reveal(update: () => void, from?: Element | null) {
  const r = from?.getBoundingClientRect() ?? new DOMRect(innerWidth / 2, innerHeight / 2, 0, 0)
  const x = r.left + r.width / 2
  const y = r.top + r.height / 2
  const reach = Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y))
  // Named, the pressed thing gets its own layer above the circle, so styles.css can drop its old look and this can
  // bounce the new one.
  const pop = from instanceof HTMLElement ? from : null
  if (pop) pop.style.viewTransitionName = 'theme-pick'
  const root = document.documentElement
  void withTransition(update, 'theme', () => {
    root.animate(
      { clipPath: [`circle(0 at ${x}px ${y}px)`, `circle(${reach}px at ${x}px ${y}px)`] },
      { duration: 640, easing: 'cubic-bezier(0.7, 0, 0.2, 1)', pseudoElement: '::view-transition-new(root)' },
    )
    if (pop) {
      root.animate(
        { scale: [0.55, 1.22, 0.92, 1.05, 1], rotate: ['-8deg', '5deg', '-3deg', '1deg', '0deg'] },
        { duration: 720, easing: 'ease-out', pseudoElement: '::view-transition-new(theme-pick)' },
      )
    }
  }).then(() => {
    if (pop) pop.style.viewTransitionName = ''
    if (!pop || reducedMotion.matches || !pop.isConnected) return
    splash(pop)
  })
}

/** A ring of the accent that grows out of `el` and fades: the page's look landing. */
function splash(el: HTMLElement) {
  const r = el.getBoundingClientRect()
  const size = Math.max(r.width, r.height)
  const ring = document.createElement('div')
  ring.setAttribute('aria-hidden', 'true')
  Object.assign(ring.style, {
    position: 'fixed', left: `${r.left + r.width / 2 - size / 2}px`, top: `${r.top + r.height / 2 - size / 2}px`,
    width: `${size}px`, height: `${size}px`, borderRadius: '50%', border: '2px solid var(--accent)',
    pointerEvents: 'none', zIndex: '2147483647',
  })
  // Inside the open dialog when there is one: the top layer would cover a ring on the page.
  ;(el.closest('dialog[open]') ?? document.body).append(ring)
  ring
    .animate(
      { scale: [0.6, 3.2], opacity: [0.9, 0], borderWidth: ['3px', '1px'] },
      { duration: 620, easing: 'cubic-bezier(0.2, 0.8, 0.2, 1)' },
    )
    .finished.finally(() => ring.remove())
}

/** Change the theme with the new one spreading out in a circle from the button that was pressed. */
export function setTheme(value: typeof prefs.theme, e?: MouseEvent) {
  if (prefs.theme === value) return
  // From the palette there's no button: spread from the theme switcher's current button instead. A radio in
  // Settings is hidden inside its label, so the label is what shows the pop.
  const target = (e?.currentTarget as HTMLElement | undefined) ?? document.querySelector('.theme [aria-pressed="true"]')
  const from = target instanceof HTMLInputElement ? (target.closest('label') ?? target) : target
  reveal(() => (prefs.theme = value), from)
}
