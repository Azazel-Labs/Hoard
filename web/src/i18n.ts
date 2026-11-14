// The portal in the reader's language, from the same Fluent catalogs as the server (`locales/<code>/hoard.ftl`).
// English is built in and is what any message a catalog lacks falls back to; the others load when picked.
// `t('message-id', { name: value })` is the only way text reaches the page, so `cargo xtask translations check` can
// find every message the portal uses and make sure the catalogs have it with the same variables.
import { FluentBundle, FluentResource, type FluentVariable } from '@fluent/bundle'
import { shallowRef } from 'vue'
import english from '../../locales/en/hoard.ftl?raw'

const loaders = import.meta.glob<string>('../../locales/*/hoard.ftl', { query: '?raw', import: 'default' })
const codeOf = (path: string) => path.split('/').at(-2) ?? 'en'

/** Every language there's a catalog for, by code. */
export const LANGUAGES: string[] = Object.keys(loaders).map(codeOf).sort()

function bundle(code: string, text: string): FluentBundle {
  // Every catalog is left-to-right; bidi isolation marks would only show up as stray characters.
  const b = new FluentBundle(code, { useIsolating: false })
  const errors = b.addResource(new FluentResource(text))
  if (errors.length) console.warn(`the ${code} catalog has errors`, errors)
  return b
}

const englishBundle = bundle('en', english)
const current = shallowRef({ code: 'en', bundle: englishBundle })

/** A message in the current language. Reactive: text shown with it changes when the language does. */
export function t(id: string, args?: Record<string, FluentVariable>): string {
  for (const b of [current.value.bundle, englishBundle]) {
    const message = b.getMessage(id)
    if (message?.value) return b.formatPattern(message.value, args)
  }
  return id
}

/** The language the portal is in, as a code (`en`, `pt-BR`…). */
export function language(): string {
  return current.value.code
}

/** Switch to a language: `auto` follows the browser's, or a code from LANGUAGES. */
export async function useLanguage(pref: string): Promise<void> {
  const code = pref === 'auto' ? (preferredCatalog(navigator.languages, LANGUAGES) ?? 'en') : LANGUAGES.includes(pref) ? pref : 'en'
  if (code !== current.value.code) {
    const text = code === 'en' ? english : await loaders[`../../locales/${code}/hoard.ftl`]()
    current.value = { code, bundle: code === 'en' ? englishBundle : bundle(code, text) }
  }
  document.documentElement.lang = code
}

/** A language's name in itself (`Deutsch`, `日本語`), for the picker. */
export function languageName(code: string): string {
  try {
    const name = new Intl.DisplayNames([code], { type: 'language' }).of(code) ?? code
    return name.charAt(0).toLocaleUpperCase(code) + name.slice(1)
  } catch {
    return code
  }
}

/** A number written the current language's way (`1,234` or `1 234` or `1.234`). */
export function formatNumber(n: number, options?: Intl.NumberFormatOptions): string {
  return new Intl.NumberFormat(language(), options).format(n)
}

/** A date and time written the current language's way. */
export function formatDate(date: Date | number | string, options?: Intl.DateTimeFormatOptions): string {
  return new Intl.DateTimeFormat(language(), options).format(typeof date === 'string' ? new Date(date) : date)
}

// CLDR parents, checked after an exact match and before truncation (the same table as the server's `i18n.rs`).
const PARENTS: [string, string[]][] = [
  ['zh-Hans', ['zh-cn', 'zh-sg']],
  ['zh-Hant', ['zh-tw', 'zh-hk', 'zh-mo']],
  ['es-419', ['es-ar', 'es-bo', 'es-br', 'es-bz', 'es-cl', 'es-co', 'es-cr', 'es-cu', 'es-do', 'es-ec', 'es-gt', 'es-hn', 'es-jp', 'es-mx', 'es-ni', 'es-pa', 'es-pe', 'es-pr', 'es-py', 'es-sv', 'es-us', 'es-uy', 'es-ve']],
  ['pt-PT', ['pt-ao', 'pt-ch', 'pt-cv', 'pt-fr', 'pt-gq', 'pt-gw', 'pt-lu', 'pt-mo', 'pt-mz', 'pt-st', 'pt-tl']],
  ['nb', ['no']],
  ['es-ES', ['es']],
  ['pt-BR', ['pt']],
]
const SERBIAN_CYRILLIC = ['sr', 'sr-rs', 'sr-ba', 'sr-xk']

/** The first catalog the preferences can read, letting fr-CA use fr; never guesses between sibling variants. */
export function preferredCatalog(preferences: readonly string[], available: readonly string[]): string | undefined {
  const find = (code: string) => available.find((c) => c.toLowerCase() === code)
  for (const preference of preferences) {
    let code = preference.toLowerCase().replaceAll('_', '-')
    const serbianLatin = code.includes('latn') || code.split('-')[1] === 'me'
    for (;;) {
      const parent = PARENTS.find(([, children]) => children.includes(code))?.[0] ?? (!serbianLatin && SERBIAN_CYRILLIC.includes(code) ? 'sr-cyrl' : undefined)
      const found = find(code) ?? (parent ? find(parent.toLowerCase()) : undefined)
      if (found) return found
      const end = code.lastIndexOf('-')
      if (end < 0) break
      code = code.slice(0, end)
    }
  }
  return undefined
}
