// What the Settings pages share: the server's settings, saving a change to them, and the server options that save
// on their own when they're entered. SettingsPage provides it; a page shows only once the settings have loaded.
import { type ComputedRef, type InjectionKey, type Ref, inject } from 'vue'
import type { Settings, SettingsPatch } from '../api'
import { t } from '../i18n'

/** The server options typed into a field, saved when it's left (or on Enter). */
export interface Opts {
  workers: number
  max_upload_mb: number
  max_expand_mb: number
  conversion_cache_mb: number
  name: string
  public_url: string
  soundfont: string
}
export type OptKey = keyof Opts
export type AnyOpt = keyof Settings['options']

export interface SettingsContext {
  /** The settings as the server last sent them. */
  s: ComputedRef<Settings>
  /** A save is under way. */
  busy: Ref<boolean>
  save(patch: SettingsPatch, done?: string): Promise<void>
  /** Settings a panel saved itself. */
  take(next: Settings): void
  /** Run now and whenever the settings come back from the server, to put a page's form back to what's saved. */
  onTake(fn: (next: Settings) => void): void
  opts: Opts
  saveOpt(k: OptKey): void
  resetOpt(k: AnyOpt): Promise<void>
  /** Not at the server's default, so it gets a reset arrow. */
  changed(k: AnyOpt): boolean
  unauthorized(): void
}

export const SETTINGS: InjectionKey<SettingsContext> = Symbol('settings')

export function useSettings(): SettingsContext {
  const ctx = inject(SETTINGS)
  if (!ctx) throw new Error('a Settings page outside SettingsPage')
  return ctx
}

export const onOff = (v: boolean) => (v ? t('settings-default-on') : t('settings-default-off'))
export const trimUrl = (u: string) => u.trim().replace(/\/+$/, '')
