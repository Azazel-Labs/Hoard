<script setup lang="ts">
// Downloads: what a download's texture files are called, the pattern, each map's suffix and endings to swap.
import { computed, nextTick, reactive } from 'vue'
import { t } from '../i18n'
import { ROLE_LABEL } from '../materials'
import {
  ROLES,
  SHORT,
  SLOTS,
  SLOT_LABEL,
  problem as namesProblem,
  stem as mapStem,
  usedEndings,
  type Ending,
  type TextureNames,
} from '../texnames'
import ResetButton from './ResetButton.vue'
import { useSettings } from './settingsContext'

const { s, busy, save, onTake } = useSettings()

// Texture names: edited here, saved when a field is left (or Enter), checked first the way the server checks.
const names = reactive<TextureNames>({ on: true, pattern: '{name}_{map}', maps: {}, endings: [] })
onTake((next) => {
  const tn = next.texture_names
  const endings = tn.endings.map((e) => ({ ...e }))
  Object.assign(names, { on: tn.on, pattern: tn.pattern, maps: { ...tn.maps }, endings })
})
const namesError = computed(() => (names.on ? namesProblem(names) : null))
const endingsKey = (e: Ending[]) => JSON.stringify(usedEndings(e).map((x) => [x.from.trim(), x.to.trim()]))
const sameMaps = (a: TextureNames, b: Pick<TextureNames, 'pattern' | 'maps'>) =>
  a.pattern === b.pattern && [...ROLES, ...SLOTS].every((r) => (a.maps[r] ?? '').trim() === (b.maps[r] ?? '').trim())
const sameNames = (a: TextureNames, b: TextureNames) =>
  a.on === b.on && sameMaps(a, b) && endingsKey(a.endings) === endingsKey(b.endings)
/** Which preset the pattern and suffixes are, if they're one. */
const namesPreset = computed(() => {
  const d = s.value?.texture_names.default
  if (d && sameMaps(names, d)) return 'descriptive'
  if (sameMaps(names, SHORT)) return 'short'
  return 'custom'
})
const namesExample = computed(() =>
  [
    `${mapStem(names, 'Stand Wood Old 05', 'base_color')}.jpg`,
    `${mapStem(names, 'Stand Wood Old 05', 'normal')}.jpg`,
    `${mapStem(names, 'Stand Wood Old 05', 'roughness')}.jpg`,
    `${mapStem(names, 'Rock Moss', 'base_color', '4K')}.png`,
  ],
)
function saveNames() {
  const cur = s.value
  if (!cur || namesError.value) return
  if (sameNames(names, cur.texture_names)) return
  const endings = usedEndings(names.endings).map((e) => ({ from: e.from.trim(), to: e.to.trim() }))
  const next = { on: names.on, pattern: names.pattern.trim(), maps: { ...names.maps }, endings }
  void save({ texture_names: next }, t('settings-tn-saved'))
}
function addEnding() {
  names.endings.push({ from: '', to: '' })
  void nextTick(() => document.getElementById(`tn-from-${names.endings.length - 1}`)?.focus())
}
/** A row is saved once it's left (or on Enter), not between its two fields, so a save coming back can't undo what's
 * typed in the second. */
function leaveEnding(ev: FocusEvent) {
  const row = ev.currentTarget as HTMLElement
  if (!(ev.relatedTarget instanceof Node && row.contains(ev.relatedTarget))) saveNames()
}
function removeEnding(i: number) {
  names.endings.splice(i, 1)
  saveNames()
}
function useNames(preset: 'descriptive' | 'short') {
  const d = s.value?.texture_names.default
  if (!d) return
  Object.assign(names, preset === 'short' ? { pattern: SHORT.pattern, maps: { ...SHORT.maps } } : { pattern: d.pattern, maps: { ...d.maps } })
  saveNames()
}
</script>

<template>
  <section data-testid="texture-names">
    <div class="sh">
      <h3>{{ t('settings-tn') }}</h3>
      <ResetButton
        v-if="s.texture_names.saved && !sameNames(s.texture_names, s.texture_names.default)"
        :to="t('settings-the-default')"
        :disabled="busy"
        @reset="save({ texture_names: null }, t('settings-tn-reset'))"
      />
    </div>
    <p class="what">{{ t('settings-tn-what') }}</p>
    <div class="switches">
      <div class="opt-row">
        <label class="check">
          <input
            type="checkbox"
            :checked="names.on"
            :disabled="busy"
            @change="names.on = ($event.target as HTMLInputElement).checked; saveNames()"
          />
          {{ t('settings-tn-toggle') }}
        </label>
      </div>
    </div>
    <template v-if="names.on">
      <div class="llm opts">
        <span class="label">{{ t('settings-tn-style') }}</span>
        <div class="modes" role="radiogroup" :aria-label="t('settings-tn-style-group')">
          <label><input type="radio" name="tn-style" :checked="namesPreset === 'descriptive'" :disabled="busy" @change="useNames('descriptive')" />{{ t('settings-tn-descriptive') }}</label>
          <label><input type="radio" name="tn-style" :checked="namesPreset === 'short'" :disabled="busy" @change="useNames('short')" />{{ t('settings-tn-short') }}</label>
          <label><input type="radio" name="tn-style" :checked="namesPreset === 'custom'" disabled />{{ t('settings-tn-custom') }}</label>
        </div>
        <label for="tn-pattern">{{ t('settings-tn-pattern') }}</label>
        <div class="key">
          <input
            id="tn-pattern"
            v-model="names.pattern"
            class="field mono"
            spellcheck="false"
            :disabled="busy"
            @change="saveNames"
            @keydown.enter.prevent="saveNames"
          />
        </div>
        <span />
        <p class="hint inline">
          <code>{name}</code>{{ t('settings-tn-hint-name') }}<code>{map}</code>{{ t('settings-tn-hint-map') }}<code>{size}</code>{{ t('settings-tn-hint-size') }}
        </p>
        <span class="label">{{ t('settings-tn-example') }}</span>
        <p class="tn-example mono" data-testid="texture-example">
          <span v-for="e in namesExample" :key="e">{{ e }}</span>
        </p>
      </div>
      <p v-if="namesError" class="status" data-state="unavailable" role="alert">
        <span class="dot" aria-hidden="true" /><span class="text">{{ namesError }}</span>
      </p>
      <details class="tn-maps">
        <summary>{{ t('settings-tn-suffixes') }}</summary>
        <div class="llm opts">
          <template v-for="r in ROLES" :key="r">
            <label :for="`tn-${r}`">{{ ROLE_LABEL[r] }}</label>
            <div class="key">
              <input
                :id="`tn-${r}`"
                v-model="names.maps[r]"
                class="field mono short"
                spellcheck="false"
                :disabled="busy"
                @change="saveNames"
                @keydown.enter.prevent="saveNames"
              />
            </div>
          </template>
        </div>
        <p class="what tn-slots">{{ t('settings-tn-slots') }}</p>
        <div class="llm opts">
          <template v-for="k in SLOTS" :key="k">
            <label :for="`tn-${k}`">{{ SLOT_LABEL[k] }}</label>
            <div class="key">
              <input
                :id="`tn-${k}`"
                v-model="names.maps[k]"
                class="field mono short"
                spellcheck="false"
                :disabled="busy"
                @change="saveNames"
                @keydown.enter.prevent="saveNames"
              />
            </div>
          </template>
        </div>
      </details>
      <div class="tn-endings" data-testid="texture-endings">
        <h4>{{ t('settings-tn-endings') }}</h4>
        <p class="what">{{ t('settings-tn-endings-what') }}</p>
        <ul v-if="names.endings.length">
          <li v-for="(e, i) in names.endings" :key="i" @focusout="leaveEnding">
            <input
              :id="`tn-from-${i}`"
              v-model="e.from"
              class="field mono"
              spellcheck="false"
              :aria-label="t('settings-tn-ending-from')"
              :placeholder="t('settings-tn-ending-from-example')"
              :disabled="busy"
              @keydown.enter.prevent="saveNames"
            />
            <span aria-hidden="true">→</span>
            <input
              v-model="e.to"
              class="field mono"
              spellcheck="false"
              :aria-label="t('settings-tn-ending-to')"
              :placeholder="t('settings-tn-ending-to-example')"
              :disabled="busy"
              @keydown.enter.prevent="saveNames"
            />
            <button
              type="button"
              class="btn ghost"
              :disabled="busy"
              :aria-label="t('settings-tn-ending-remove-label', { ending: e.from.trim() || '…' })"
              @click="removeEnding(i)"
            >
              {{ t('settings-tn-ending-remove') }}
            </button>
          </li>
        </ul>
        <button type="button" class="btn" :disabled="busy" @click="addEnding">
          {{ t('settings-tn-ending-add') }}
        </button>
      </div>
    </template>
  </section>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.tn-example {
  display: grid; gap: 0.1rem; margin: 0; padding: 0.5rem 0.75rem; border: 1px dashed var(--line); border-radius: 8px;
  background: var(--well); color: var(--text); overflow-wrap: anywhere;
}
.tn-maps { margin-top: 0.9rem; padding-top: 0.8rem; border-top: 1px solid var(--line); font-size: var(--step--1); }
.tn-maps summary { cursor: pointer; color: var(--muted); width: fit-content; }
.tn-maps summary:hover { color: var(--text); }
.tn-maps[open] summary { margin-bottom: 0.7rem; }
.llm .key .field.short { max-width: 10rem; }
.tn-slots { margin: 1rem 0 0.6rem; }
.tn-endings { margin-top: 0.9rem; padding-top: 0.8rem; border-top: 1px solid var(--line); }
.tn-endings h4 { margin: 0 0 0.3rem; font-size: var(--step--1); font-weight: 550; }
.tn-endings ul { list-style: none; margin: 0 0 0.6rem; padding: 0; display: grid; gap: 0.4rem; }
.tn-endings li { display: flex; gap: 0.5rem; align-items: center; }
.tn-endings li .field { flex: 1 1 0; min-width: 0; max-width: 12rem; }
.tn-endings li > span { color: var(--muted); }
</style>
