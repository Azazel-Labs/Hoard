<script setup lang="ts">
// Appearance: this browser's language and its light and dark themes, saved here.
import { onBeforeUnmount } from 'vue'
import { LANGUAGES, languageName, preferredCatalog, t } from '../i18n'
import { prefs } from '../prefs'
import { DARK_THEMES, LIGHT_THEMES, previewScheme, themeFor, usedScheme } from '../theme'

const browserLanguages = navigator.languages

const LOOKS = [
  { scheme: 'light' as const, get label() { return t('settings-look-light') }, list: LIGHT_THEMES },
  { scheme: 'dark' as const, get label() { return t('settings-look-dark') }, list: DARK_THEMES },
]
/**
 * Pick a theme and wear it at once, even when it's for the scheme not in use (see `previewScheme`). Runs on click,
 * not change: clicking the theme that's already picked (back to light while previewing dark) has to work too.
 */
function setLook(scheme: 'light' | 'dark', id: string) {
  if (scheme === 'light') prefs.lightTheme = id
  else prefs.darkTheme = id
  previewScheme.value = scheme === usedScheme() ? null : scheme
}
const endPreview = () => (previewScheme.value = null)
onBeforeUnmount(endPreview)
</script>

<template>
  <section class="look" :aria-label="t('settings-themes')">
    <div class="themes">
      <label class="tlabel lang" for="language">{{ t('appearance-language') }}</label>
      <select id="language" v-model="prefs.language" class="field lang">
        <option value="auto">{{ t('appearance-language-auto', { name: languageName(preferredCatalog(browserLanguages, LANGUAGES) ?? 'en') }) }}</option>
        <option v-for="code in LANGUAGES" :key="code" :value="code" :lang="code">{{ languageName(code) }}</option>
      </select>
    </div>
    <div v-for="g in LOOKS" :key="g.scheme" class="themes">
      <span class="tlabel">{{ g.label }}</span>
      <div class="swatches" role="radiogroup" :aria-label="t('settings-look-group', { scheme: g.scheme })">
        <label
          v-for="theme in g.list"
          :key="theme.id"
          class="sw"
          :class="[g.scheme, { on: themeFor(g.scheme).id === theme.id }]"
          v-bind="{ [`data-${g.scheme}`]: theme.id }"
          :title="theme.blurb"
        >
          <input type="radio" :name="`theme-${g.scheme}`" :checked="themeFor(g.scheme).id === theme.id" @click="setLook(g.scheme, theme.id)" />
          <span class="mini" aria-hidden="true">
            <span class="bar" /><span class="ln" /><span class="ln short" /><span class="dot" />
          </span>
          <span class="nm">{{ theme.name }}</span>
          <small>{{ theme.blurb }}</small>
        </label>
      </div>
    </div>
    <p v-if="previewScheme" class="hint preview" role="status">
      {{ t('settings-preview', { preview: previewScheme, used: usedScheme(), system: prefs.theme === 'system' ? 'yes' : 'no' }) }}
      <button type="button" class="btn ghost" @click="endPreview">{{ t('settings-preview-back', { used: usedScheme() }) }}</button>
    </p>
    <p v-else class="hint">{{ t('settings-look-hint') }}</p>
  </section>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.hint.preview .btn { margin-left: 0.4rem; }

/* Theme swatches: each label carries data-light / data-dark, so styles.css gives it that theme's --l-* / --d-*. */
.themes { display: grid; grid-template-columns: 6rem minmax(0, 1fr); gap: 0.8rem; align-items: start; margin-bottom: 1.4rem; }
.themes + .themes { padding-top: 1.2rem; border-top: 1px solid var(--line); }
.tlabel { font-size: var(--step--1); font-weight: 550; color: var(--text); padding-top: 1.3rem; }
.tlabel.lang { padding-top: 0; align-self: center; }
.field.lang { justify-self: start; min-width: 14rem; }
.swatches { display: grid; grid-template-columns: repeat(auto-fill, minmax(6.5rem, 1fr)); gap: 0.6rem; }
.sw small { font-size: 0.68rem; color: var(--faint); font-weight: 400; line-height: 1.3; }
.sw.light { --s-bg: var(--l-bg); --s-panel: var(--l-panel); --s-line: var(--l-line); --s-text: var(--l-text); --s-muted: var(--l-muted); --s-accent: var(--l-accent); }
.sw.dark { --s-bg: var(--d-bg); --s-panel: var(--d-panel); --s-line: var(--d-line); --s-text: var(--d-text); --s-muted: var(--d-muted); --s-accent: var(--d-accent); }
.sw { display: grid; gap: 0.25rem; cursor: pointer; font-size: 0.75rem; color: var(--muted); }
.sw input { position: absolute; opacity: 0; pointer-events: none; }
.mini {
  position: relative; display: block; height: 3.6rem; border-radius: 6px; overflow: hidden;
  background: var(--s-bg); border: 1px solid var(--line);
  transition: border-color var(--dur-1), box-shadow var(--dur-2) var(--ease-out), scale var(--dur-2) var(--ease-pop);
}
.mini .bar { position: absolute; inset: 0 0 auto; height: 0.6rem; background: var(--s-panel); border-bottom: 1px solid var(--s-line); }
.mini .ln { position: absolute; left: 0.4rem; top: 1rem; width: 55%; height: 0.25rem; border-radius: 1px; background: var(--s-text); }
.mini .ln.short { top: 1.55rem; width: 35%; background: var(--s-muted); }
.mini .dot { position: absolute; right: 0.4rem; bottom: 0.4rem; width: 0.55rem; height: 0.55rem; border-radius: 50%; background: var(--s-accent); }
.sw:hover .mini { border-color: var(--muted); }
.sw:active .mini { scale: 0.95; }
.sw.on { color: var(--text); font-weight: 600; }
.sw.on .mini { border-color: var(--text); box-shadow: 0 0 0 1px var(--text); }
.sw:has(:focus-visible) .mini { outline: 2px solid var(--accent); outline-offset: 2px; }
@media (max-width: 30rem) { .themes { grid-template-columns: minmax(0, 1fr); gap: 0.3rem; } .tlabel { padding-top: 0; } }
</style>
