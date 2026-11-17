<script setup lang="ts">
// Settings → Adding packs → Conversion rules: files Hoard keeps converted in place, such as every OBJ to GLB. Saving
// a change that matches files already in the library first says how many it would convert, and which, and asks
// whether to convert them now or only what's added from then on.
import { computed, nextTick, ref, watch } from 'vue'
import { api, ApiError, type ConversionPreview, type ConversionRules, type Settings } from '../api'
import { t } from '../i18n'
import { showError, showOk } from '../ui'

const props = defineProps<{ settings: Settings['conversion_rules']; busy: boolean }>()
const emit = defineEmits<{ saved: [Settings]; unauthorized: [] }>()

const copy = (s: ConversionRules): ConversionRules => ({
  rules: s.rules.map((r) => ({ ...r, from: [...r.from] })),
  keep_days: s.keep_days,
})
const draft = ref<ConversionRules>(copy(props.settings))
watch(() => props.settings, (s) => (draft.value = copy(s)))

const same = (a: ConversionRules, b: ConversionRules) => JSON.stringify(copy(a)) === JSON.stringify(copy(b))
const dirty = computed(() => !same(draft.value, props.settings))
const working = ref(false)
const disabled = computed(() => props.busy || working.value)

/** The formats a rule converting to `to` can take. */
const sourcesOf = (to: string) => props.settings.targets.find((x) => x.to === to)?.from ?? []
/** Another rule that's on converts this format already. */
const takenBy = (ext: string, i: number) => draft.value.rules.some((r, j) => j !== i && r.on && r.from.includes(ext))

function toggle(i: number, ext: string, on: boolean) {
  const from = draft.value.rules[i].from
  if (on && !from.includes(ext)) from.push(ext)
  if (!on) from.splice(from.indexOf(ext), 1)
}
function addRule() {
  const to = props.settings.targets[0]?.to ?? 'glb'
  const free = ['obj', 'fbx'].filter((e) => !draft.value.rules.some((r) => r.on && r.from.includes(e)))
  draft.value.rules.push({ on: true, from: free, to, upright: true })
}

// The question asked before saving rules that would change files already in the library.
const dlg = ref<HTMLDialogElement | null>(null)
const asking = ref<{ preview: ConversionPreview; save: boolean } | null>(null)
let answer: ((a: 'convert' | 'new' | null) => void) | null = null
function ask(preview: ConversionPreview, save: boolean): Promise<'convert' | 'new' | null> {
  asking.value = { preview, save }
  void nextTick(() => dlg.value?.showModal())
  return new Promise((resolve) => (answer = resolve))
}
function reply(a: 'convert' | 'new' | null) {
  dlg.value?.close()
  asking.value = null
  answer?.(a)
  answer = null
}

const SKIPS = ['watched', 'unity', 'used', 'beside', 'blender'] as const
const skipLabel = (k: (typeof SKIPS)[number], n: number) =>
  ({
    watched: () => t('settings-conv-skip-watched', { count: n }),
    unity: () => t('settings-conv-skip-unity', { count: n }),
    used: () => t('settings-conv-skip-used', { count: n }),
    beside: () => t('settings-conv-skip-beside', { count: n }),
    blender: () => t('settings-conv-skip-blender', { count: n }),
  })[k]()

function failed(e: unknown) {
  if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  showError((e as Error).message)
}

/** Save the rules, after saying what they'd convert that's in the library already. */
async function save() {
  working.value = true
  try {
    const rules = copy(draft.value)
    const preview = await api.previewConversions(rules)
    let choice: 'convert' | 'new' | null = 'new'
    if (preview.files > 0) choice = await ask(preview, true)
    if (!choice) return
    emit('saved', await api.saveSettings({ conversion_rules: rules }))
    if (choice === 'convert') {
      await api.runConversions()
      showOk(t('settings-conv-converting', { count: preview.files }))
    } else {
      showOk(t('settings-conv-saved'))
    }
  } catch (e) {
    failed(e)
  } finally {
    working.value = false
  }
}

/** Convert what the saved rules match among files already in the library. */
async function convertExisting() {
  working.value = true
  try {
    const preview = await api.previewConversions(copy(props.settings))
    if (preview.files === 0) {
      showOk(t('settings-conv-nothing'))
      return
    }
    if ((await ask(preview, false)) !== 'convert') return
    await api.runConversions()
    showOk(t('settings-conv-converting', { count: preview.files }))
  } catch (e) {
    failed(e)
  } finally {
    working.value = false
  }
}
</script>

<template>
  <section data-testid="conversion-rules">
    <div class="sh"><h3>{{ t('settings-conv') }}</h3></div>
    <p class="what">{{ t('settings-conv-what') }}</p>
    <ol v-if="draft.rules.length" class="rules">
      <li v-for="(rule, i) in draft.rules" :key="i" class="rule" :class="{ off: !rule.on }" :data-testid="`conv-rule-${i}`">
        <div class="head">
          <label class="check">
            <input v-model="rule.on" type="checkbox" :disabled="disabled" />
            {{ t('settings-conv-rule-on') }}
          </label>
          <label class="to">
            {{ t('settings-conv-to') }}
            <select v-model="rule.to" class="field" :disabled="disabled">
              <option v-for="x in settings.targets" :key="x.to" :value="x.to">{{ x.to.toUpperCase() }}</option>
            </select>
          </label>
          <button
            type="button"
            class="btn ghost"
            :disabled="disabled"
            :aria-label="t('settings-conv-remove-label', { n: i + 1 })"
            @click="draft.rules.splice(i, 1)"
          >
            {{ t('settings-conv-remove') }}
          </button>
        </div>
        <fieldset class="formats">
          <legend>{{ t('settings-conv-from') }}</legend>
          <label v-for="ext in sourcesOf(rule.to)" :key="ext" :class="{ on: rule.from.includes(ext) }" :title="takenBy(ext, i) ? t('settings-conv-taken') : undefined">
            <input
              type="checkbox"
              :checked="rule.from.includes(ext)"
              :disabled="disabled || (rule.on && takenBy(ext, i))"
              @change="toggle(i, ext, ($event.target as HTMLInputElement).checked)"
            />
            <span class="mono">.{{ ext }}</span>
          </label>
        </fieldset>
        <label class="check">
          <input v-model="rule.upright" type="checkbox" :disabled="disabled" />
          {{ t('settings-conv-upright') }}
        </label>
      </li>
    </ol>
    <div class="bar">
      <button type="button" class="btn" :disabled="disabled" data-testid="conv-add" @click="addRule">{{ t('settings-conv-add') }}</button>
      <label class="keep">
        {{ t('settings-conv-keep') }}
        <input v-model.number="draft.keep_days" class="field num" type="number" min="1" max="365" :disabled="disabled" />
        {{ t('settings-conv-keep-days', { count: draft.keep_days || 0 }) }}
      </label>
    </div>
    <p class="what">{{ t('settings-conv-keep-what') }}</p>
    <div class="bar">
      <button type="button" class="btn primary" :disabled="disabled || !dirty" data-testid="conv-save" @click="save">{{ t('settings-conv-save') }}</button>
      <button v-if="dirty" type="button" class="btn ghost" :disabled="disabled" @click="draft = copy(settings)">{{ t('settings-conv-revert') }}</button>
      <button
        v-else-if="settings.rules.some((r) => r.on)"
        type="button"
        class="btn"
        :disabled="disabled"
        data-testid="conv-existing"
        @click="convertExisting"
      >
        {{ t('settings-conv-existing') }}
      </button>
    </div>

    <dialog ref="dlg" class="ask" data-testid="conv-dialog" @cancel.prevent="reply(null)" @click="$event.target === dlg && reply(null)">
      <form v-if="asking" method="dialog" @submit.prevent="reply('convert')">
        <h2>{{ t('settings-conv-ask-title', { count: asking.preview.files }) }}</h2>
        <p>{{ t('settings-conv-ask-body', { days: draft.keep_days, packs: asking.preview.packs }) }}</p>
        <p v-if="asking.preview.busy">{{ t('settings-conv-ask-busy', { count: asking.preview.busy }) }}</p>
        <ul class="sample" :aria-label="t('settings-conv-ask-list')">
          <li v-for="f in asking.preview.sample" :key="f.id">
            <span class="pack">{{ f.pack }}</span>
            <span class="mono">{{ f.path }}</span>
            <span class="mono arrow">→ {{ f.to.split('/').pop() }}</span>
          </li>
          <li v-if="asking.preview.files > asking.preview.sample.length" class="more">
            {{ t('settings-conv-ask-more', { count: asking.preview.files - asking.preview.sample.length }) }}
          </li>
        </ul>
        <template v-if="SKIPS.some((k) => asking!.preview.skipped[k])">
          <p>{{ t('settings-conv-ask-skipped') }}</p>
          <ul class="skips">
            <template v-for="k in SKIPS" :key="k">
              <li v-if="asking.preview.skipped[k]">{{ skipLabel(k, asking.preview.skipped[k]!) }}</li>
            </template>
          </ul>
        </template>
        <div class="row">
          <button type="button" class="btn ghost" autofocus @click="reply(null)">{{ t('ui-cancel') }}</button>
          <button v-if="asking.save" type="button" class="btn" data-testid="conv-new-only" @click="reply('new')">{{ t('settings-conv-ask-new') }}</button>
          <button type="submit" class="btn primary" data-testid="conv-convert">
            <template v-if="asking.save">{{ t('settings-conv-ask-save-convert', { count: asking.preview.files }) }}</template>
            <template v-else>{{ t('settings-conv-ask-convert', { count: asking.preview.files }) }}</template>
          </button>
        </div>
      </form>
    </dialog>
  </section>
</template>

<style scoped>
.sh { display: flex; align-items: center; gap: 0.4rem; min-height: 1.75rem; margin-bottom: 0.2rem; }
.sh h3 { margin: 0; font-size: var(--step-0); }
.what { margin: 0 0 0.55rem; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.rules { display: grid; gap: 0.6rem; margin: 0 0 0.8rem; padding: 0; list-style: none; }
.rule { display: grid; gap: 0.5rem; padding: 0.7rem 0.8rem; border: 1px solid var(--line); border-radius: var(--r-card, 10px); }
.rule.off { opacity: 0.7; }
.head { display: flex; flex-wrap: wrap; align-items: center; gap: 0.6rem; }
.head .to { display: inline-flex; align-items: center; gap: 0.4rem; font-size: var(--step--1); margin-inline-start: auto; }
.check { display: inline-flex; gap: 0.6rem; align-items: center; font-size: var(--step--1); font-weight: 550; cursor: pointer; }
.formats { display: flex; flex-wrap: wrap; gap: 0.35rem; margin: 0; padding: 0; border: 0; }
.formats legend { margin-bottom: 0.3rem; padding: 0; font-size: var(--step--1); color: var(--muted); }
.formats label {
  display: inline-flex; align-items: center; gap: 0.3rem; padding: 0.15rem 0.5rem; border: 1px solid var(--line);
  border-radius: var(--r-chip); font-size: var(--step--1); cursor: pointer;
}
.formats label.on { border-color: var(--card-on-line); background: var(--card-on); }
.bar { display: flex; flex-wrap: wrap; align-items: center; gap: 0.6rem; margin-bottom: 0.6rem; }
.keep { display: inline-flex; align-items: center; gap: 0.4rem; font-size: var(--step--1); margin-inline-start: auto; }
.keep .num { width: 5rem; }
.mono { font-family: var(--mono); }

.ask {
  width: min(36rem, calc(100vw - 2rem)); padding: 0; border: 1px solid var(--line); border-radius: 12px;
  background: var(--raised); color: var(--text); box-shadow: 0 24px 64px var(--shadow);
}
.ask::backdrop { background: var(--scrim); backdrop-filter: blur(3px); }
.ask form { display: flex; flex-direction: column; gap: 0.6rem; padding: 1.25rem 1.25rem 1rem; }
.ask h2 { margin: 0; font-size: var(--step-1); font-weight: 650; }
.ask p { margin: 0; color: var(--muted); font-size: var(--step--1); }
.sample {
  display: grid; gap: 0.15rem; max-height: 16rem; overflow: auto; margin: 0; padding: 0.4rem 0.6rem; list-style: none;
  border: 1px solid var(--line); border-radius: 8px; font-size: var(--step--1);
}
.sample li { display: flex; flex-wrap: wrap; gap: 0.4rem; overflow-wrap: anywhere; }
.sample .pack { color: var(--muted); }
.sample .arrow { color: var(--muted); }
.sample .more { color: var(--muted); font-style: italic; }
.skips { margin: 0; padding-inline-start: 1.2rem; font-size: var(--step--1); color: var(--muted); }
.row { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: 0.5rem; margin-top: 0.6rem; }
</style>
