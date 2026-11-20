<script setup lang="ts">
// Adding a library location: what kind, where, and which files go there.
import { computed, ref, watch } from 'vue'
import { api, ApiError, type AssetKind, type LibraryLocation, type LocationMode } from '../api'
import FolderPicker from './FolderPicker.vue'
import KindChips from './KindChips.vue'
import claimArt from '../assets/dragon-claim.png'
import watchArt from '../assets/dragon-watch.png'
import { t } from '../i18n'

const props = defineProps<{
  /** Where the server suggests a first location goes. */
  suggested: string
  kinds: AssetKind[]
  /** The server has no location yet: this one takes everything. */
  first?: boolean
  /** Label for the button that adds it. */
  submitLabel?: string
  cancelable?: boolean
  /** A bucket in S3 storage rather than a folder: Hoard keeps files there, it can't watch one. */
  bucket?: boolean
  /** Runs before the location is added (saving the S3 storage it's in); false stops it. */
  beforeAdd?: () => Promise<boolean>
}>()
const emit = defineEmits<{ created: [LibraryLocation]; cancel: []; unauthorized: [] }>()

const mode = ref<LocationMode>('managed')
const folder = ref(props.bucket ? '' : props.suggested)
const bucketName = ref('')
/** What's sent: the folder, or the bucket as s3://name. */
const path = computed({
  get: () => (props.bucket ? (bucketName.value.trim() ? `s3://${bucketName.value.trim().replace(/^s3:\/\//, '')}` : '') : folder.value),
  set: (v: string) => (folder.value = v),
})
const name = ref('')
const picked = ref<AssetKind[]>([])
const catchAll = ref(!!props.first)
const busy = ref(false)
const error = ref('')

// The suggestion suits a managed folder; a watched one is somewhere the files already are.
watch(mode, (m, old) => {
  if (m === 'watched' && path.value === props.suggested) path.value = ''
  if (m === 'managed' && old === 'watched' && !path.value) path.value = props.suggested
  error.value = ''
})

const folderName = computed(() => path.value.trim().replace(/[\\/]+$/, '').split(/[\\/]/).pop() || '')
const ready = computed(() => !!path.value.trim() && (mode.value === 'watched' || props.first || catchAll.value || picked.value.length > 0))

async function submit() {
  if (!ready.value || busy.value) return
  busy.value = true
  error.value = ''
  try {
    if (props.beforeAdd && !(await props.beforeAdd())) return
    const managed = mode.value === 'managed'
    const loc = await api.addLocation({
      name: name.value.trim() || undefined,
      mode: mode.value,
      path: path.value.trim(),
      kinds: managed ? picked.value : [],
      catch_all: managed && (catchAll.value || !!props.first),
    })
    emit('created', loc)
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    error.value = (e as Error).message
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <form class="locform" @submit.prevent="submit">
    <fieldset v-if="!bucket" class="choices">
      <legend class="lbl">{{ t('locations-mode-question') }}</legend>
      <!-- A dragon on its treasure, sorted into chests and racks; a dragon eyeing a heap it never sorted. -->
      <label :class="{ on: mode === 'managed' }">
        <img :src="claimArt" alt="" width="160" height="112" />
        <span class="pick">
          <input v-model="mode" type="radio" name="mode" value="managed" />
          <span>
            <strong>{{ t('locations-mode-managed') }}</strong>
            <small>{{ t('locations-mode-managed-hint') }}</small>
          </span>
        </span>
      </label>
      <label :class="{ on: mode === 'watched' }">
        <img :src="watchArt" alt="" width="160" height="112" />
        <span class="pick">
          <input v-model="mode" type="radio" name="mode" value="watched" />
          <span>
            <strong>{{ t('locations-mode-watched') }}</strong>
            <small>{{ t('locations-mode-watched-hint') }}</small>
          </span>
        </span>
      </label>
    </fieldset>

    <div class="grid">
      <template v-if="bucket">
        <label class="lbl" for="loc-bucket">{{ t('locations-bucket') }}</label>
        <div>
          <div class="s3url">
            <span class="mono" aria-hidden="true">s3://</span>
            <input id="loc-bucket" v-model="bucketName" class="field mono" type="text" spellcheck="false" autocomplete="off" maxlength="63" />
          </div>
          <p class="hint">{{ t('locations-bucket-hint') }}</p>
        </div>
      </template>
      <template v-else>
        <label class="lbl" for="loc-path">{{ t('locations-folder-desktop') }}</label>
        <div>
          <FolderPicker id="loc-path" v-model="folder" :placeholder="mode === 'managed' ? suggested : t('locations-folder-placeholder')" @unauthorized="emit('unauthorized')" />
          <p class="hint">
            <template v-if="mode === 'managed'">{{ t('locations-folder-hint-managed') }}</template>
            <template v-else>{{ t('locations-folder-hint-watched') }}</template>
          </p>
        </div>
      </template>

      <label class="lbl" for="loc-name">{{ t('locations-name') }}</label>
      <input id="loc-name" v-model="name" class="field" type="text" maxlength="80" :placeholder="folderName || (mode === 'managed' ? t('locations-name-placeholder-managed') : t('locations-name-placeholder-watched'))" />

      <template v-if="mode === 'managed' && !first">
        <span class="lbl">{{ t('locations-files') }}</span>
        <div>
          <KindChips v-model="picked" :kinds="kinds" />
          <label class="check">
            <input v-model="catchAll" type="checkbox" />
            {{ t('locations-catch-all') }}
          </label>
          <p class="hint">{{ t('locations-kinds-hint') }}</p>
        </div>
      </template>
      <template v-else-if="mode === 'managed'">
        <span class="lbl">{{ t('locations-files') }}</span>
        <p class="hint flat">{{ t('locations-files-all') }}</p>
      </template>
    </div>

    <p v-if="first && mode === 'watched'" class="hint note">
      {{ t('locations-first-watched-note') }}
    </p>
    <p v-if="error" class="err" role="alert">{{ error }}</p>

    <div class="acts">
      <button type="submit" class="btn primary" :disabled="!ready || busy" :aria-busy="busy">{{ busy ? t('locations-checking') : submitLabel || t('locations-add') }}</button>
      <button v-if="cancelable" type="button" class="btn ghost" @click="emit('cancel')">{{ t('locations-cancel') }}</button>
    </div>
  </form>
</template>

<style scoped>
.locform { display: grid; gap: 1rem; font-size: var(--step--1); }
fieldset { margin: 0; padding: 0; border: 0; min-width: 0; }
.lbl { color: var(--muted); }
legend.lbl { margin-bottom: 0.45rem; padding: 0; }
.choices { display: grid; grid-template-columns: repeat(auto-fit, minmax(13rem, 1fr)); gap: 0.6rem; }
.choices label {
  display: grid; gap: 0.6rem; align-content: start; padding: 0.6rem; border: 1px solid var(--line);
  border-radius: var(--r-chip); cursor: pointer; transition: border-color var(--dur-1), background-color var(--dur-1);
}
.choices label:hover { border-color: color-mix(in srgb, var(--muted) 60%, var(--line)); }
.choices label.on { border-color: var(--muted); background: var(--raised); }
/* Pixel art: drawn at 160×112 and scaled up whole, never smoothed. */
.choices img {
  display: block; width: 100%; height: auto; image-rendering: pixelated; border-radius: calc(var(--r-chip) - 2px);
  filter: saturate(0.55) brightness(0.85); transition: filter var(--dur-1);
}
.choices label:hover img, .choices label.on img { filter: none; }
.choices .pick { display: flex; gap: 0.6rem; align-items: flex-start; padding: 0 0.15rem 0.1rem; }
.choices input { margin: 0.2rem 0 0; }
.choices .pick > span { display: grid; gap: 0.15rem; }
.choices small { color: var(--muted); font-size: inherit; line-height: 1.45; }
.grid { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.7rem 0.9rem; align-items: baseline; }
.grid > .field { max-width: 22rem; }
.hint { margin: 0.4rem 0 0; color: var(--muted); line-height: 1.45; }
.hint.flat { margin: 0; }
.note { margin: 0; }
.check { display: inline-flex; gap: 0.45rem; align-items: center; }
.err { margin: 0; color: var(--danger); overflow-wrap: anywhere; }
.acts { display: flex; gap: 0.5rem; }
.s3url { display: flex; align-items: baseline; gap: 0.3rem; max-width: 22rem; }
.s3url > span { color: var(--muted); }
.s3url .field { flex: 1; min-width: 0; }
@media (max-width: 30rem) { .grid { grid-template-columns: minmax(0, 1fr); gap: 0.35rem; } .grid > .lbl:not(:first-child) { margin-top: 0.5rem; } }
</style>
