<script setup lang="ts">
// Adding a library location: what kind, where, and which files go there.
import { computed, ref, watch } from 'vue'
import { api, ApiError, type AssetKind, type LibraryLocation, type LocationMode } from '../api'
import FolderPicker from './FolderPicker.vue'
import KindChips from './KindChips.vue'

const props = defineProps<{
  /** Where the server suggests a first location goes. */
  suggested: string
  kinds: AssetKind[]
  /** The server has no location yet: this one takes everything. */
  first?: boolean
  /** Label for the button that adds it. */
  submitLabel?: string
  cancellable?: boolean
}>()
const emit = defineEmits<{ created: [LibraryLocation]; cancel: []; unauthorized: [] }>()

const mode = ref<LocationMode>('managed')
const path = ref(props.suggested)
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
    <fieldset class="choices">
      <legend class="lbl">What should Hoard do with this folder?</legend>
      <label :class="{ on: mode === 'managed' }">
        <input v-model="mode" type="radio" name="mode" value="managed" />
        <span>
          <strong>Keep my library here</strong>
          <small>Packs you upload or import are copied into this folder. Hoard looks after it, so leave its contents alone.</small>
        </span>
      </label>
      <label :class="{ on: mode === 'watched' }">
        <input v-model="mode" type="radio" name="mode" value="watched" />
        <span>
          <strong>Index a folder I already have</strong>
          <small>
            Each folder inside it becomes a pack. Hoard reads the files where they are and never changes, moves or deletes them, and picks up
            new folders as they appear.
          </small>
        </span>
      </label>
    </fieldset>

    <div class="grid">
      <label class="lbl" for="loc-path">Folder on the server</label>
      <div>
        <FolderPicker id="loc-path" v-model="path" :placeholder="mode === 'managed' ? suggested : 'The folder your packs are in'" @unauthorized="emit('unauthorized')" />
        <p class="hint">
          <template v-if="mode === 'managed'">Hoard creates it if it isn't there. In Docker, this is a folder you've mounted into the container.</template>
          <template v-else>In Docker, mount the folder into the container first (read-only is fine), then pick it here.</template>
        </p>
      </div>

      <label class="lbl" for="loc-name">Name</label>
      <input id="loc-name" v-model="name" class="field" type="text" maxlength="80" :placeholder="folderName || (mode === 'managed' ? 'Library' : 'My packs')" />

      <template v-if="mode === 'managed' && !first">
        <span class="lbl">Files that go here</span>
        <div>
          <KindChips v-model="picked" :kinds="kinds" />
          <label class="check">
            <input v-model="catchAll" type="checkbox" />
            Also everything no other location takes
          </label>
          <p class="hint">A new pack's files go to the location that ticks their kind. The rest go to the catch-all.</p>
        </div>
      </template>
      <template v-else-if="mode === 'managed'">
        <span class="lbl">Files that go here</span>
        <p class="hint flat">All of them, for now. Add more locations later from Settings to send some kinds (music, say) to another disk.</p>
      </template>
    </div>

    <p v-if="first && mode === 'watched'" class="hint note">
      Uploading needs a library Hoard keeps. You can add one later from Settings → Library locations.
    </p>
    <p v-if="error" class="err" role="alert">{{ error }}</p>

    <div class="acts">
      <button type="submit" class="btn primary" :disabled="!ready || busy" :aria-busy="busy">{{ busy ? 'Checking the folder…' : submitLabel || 'Add location' }}</button>
      <button v-if="cancellable" type="button" class="btn ghost" @click="emit('cancel')">Cancel</button>
    </div>
  </form>
</template>

<style scoped>
.locform { display: grid; gap: 1rem; font-size: var(--step--1); }
fieldset { margin: 0; padding: 0; border: 0; min-width: 0; }
.lbl { color: var(--muted); }
legend.lbl { margin-bottom: 0.45rem; padding: 0; }
.choices { display: grid; gap: 0.4rem; }
.choices label {
  display: flex; gap: 0.6rem; align-items: flex-start; padding: 0.6rem 0.75rem; border: 1px solid var(--line);
  border-radius: var(--r-chip); cursor: pointer; transition: border-color var(--dur-1), background-color var(--dur-1);
}
.choices label:hover { border-color: color-mix(in srgb, var(--muted) 60%, var(--line)); }
.choices label.on { border-color: var(--muted); background: var(--raised); }
.choices input { margin: 0.2rem 0 0; }
.choices span { display: grid; gap: 0.15rem; }
.choices small { color: var(--muted); font-size: inherit; line-height: 1.45; }
.grid { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.7rem 0.9rem; align-items: baseline; }
.grid > .field { max-width: 22rem; }
.hint { margin: 0.4rem 0 0; color: var(--muted); line-height: 1.45; }
.hint.flat { margin: 0; }
.note { margin: 0; }
.check { display: inline-flex; gap: 0.45rem; align-items: center; }
.err { margin: 0; color: var(--danger); overflow-wrap: anywhere; }
.acts { display: flex; gap: 0.5rem; }
@media (max-width: 30rem) { .grid { grid-template-columns: minmax(0, 1fr); gap: 0.35rem; } .grid > .lbl:not(:first-child) { margin-top: 0.5rem; } }
</style>
