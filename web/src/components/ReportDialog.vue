<script setup lang="ts">
// Settings → About → Report a problem: what an issue on GitHub needs for the problem to be looked into (the build,
// where it runs, what's turned on, and the server's last log lines), gathered by the server with nothing private in
// it. Copy puts all of it on the clipboard; the GitHub button opens a new issue with it filled in.
import { computed, onMounted, ref } from 'vue'
import { api, type ProblemReport } from '../api'
import { t } from '../i18n'
import { copyText, toast } from '../ui'

const props = defineProps<{ repo: string }>()
const emit = defineEmits<{ close: [] }>()
const dlg = ref<HTMLDialogElement | null>(null)
const report = ref<ProblemReport | null>(null)
const failed = ref('')

/** GitHub turns away a new-issue address much longer than this. */
const MAX_URL = 7500

function markdown(r: ProblemReport, log: string[]): string {
  const facts = r.facts.map((f) => `- **${f.name}:** ${f.value}`).join('\n')
  const lines = log.length < r.log.length ? [`… (${r.log.length - log.length} earlier lines)`, ...log] : log
  return `${facts}\n\n<details><summary>Recent log</summary>\n\n\`\`\`\n${lines.join('\n')}\n\`\`\`\n</details>\n`
}

function issueUrl(r: ProblemReport, log: string[]): string {
  const q = new URLSearchParams({
    template: 'bug.yml',
    version: r.version,
    environment: r.environment,
    report: markdown(r, log),
  })
  return `${props.repo}/issues/new?${q}`
}

/** A new issue with the report in it: the whole log if it fits in the address, else its last lines. */
const github = computed(() => {
  const r = report.value
  if (!r) return ''
  let keep = r.log.length
  let url = issueUrl(r, r.log)
  while (url.length > MAX_URL && keep > 0) {
    keep = Math.max(0, keep - 10)
    url = issueUrl(r, r.log.slice(r.log.length - keep))
  }
  return url
})

async function copy() {
  if (!report.value) return
  if (await copyText(markdown(report.value, report.value.log))) toast(t('report-copied'))
  else toast(t('account-copy-failed'), { kind: 'error' })
}

onMounted(async () => {
  dlg.value?.showModal()
  try {
    report.value = await api.report()
  } catch (e) {
    failed.value = e instanceof Error ? e.message : String(e)
  }
})
</script>

<template>
  <dialog
    ref="dlg"
    class="report"
    aria-labelledby="report-title"
    data-testid="report"
    @close="emit('close')"
    @click.self="dlg?.close()"
  >
    <header>
      <h2 id="report-title">{{ t('report-title') }}</h2>
      <button type="button" class="btn ghost x" :aria-label="t('report-close')" @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>
    <p class="what">{{ t('report-what') }}</p>
    <p v-if="failed" class="what" role="alert">{{ t('report-failed', { error: failed }) }}</p>
    <p v-else-if="!report" class="what">{{ t('report-loading') }}</p>
    <template v-else>
      <dl>
        <template v-for="f in report.facts" :key="f.name">
          <dt>{{ f.name }}</dt>
          <dd>{{ f.value }}</dd>
        </template>
      </dl>
      <details class="log">
        <summary>{{ t('report-log') }} <small>{{ report.log.length }}</small></summary>
        <pre data-testid="report-log">{{ report.log.join('\n') }}</pre>
      </details>
      <div class="actions">
        <button type="button" class="btn" @click="copy">{{ t('report-copy') }}</button>
        <a class="btn primary" :href="github" target="_blank" rel="noopener" data-testid="report-github">
          {{ t('report-github') }}
        </a>
      </div>
      <p class="hint">{{ t('report-github-hint') }}</p>
    </template>
  </dialog>
</template>

<style scoped>
.report {
  width: min(44rem, calc(100vw - 2rem)); max-height: calc(100vh - 2rem); padding: 1.1rem 1.3rem 1.3rem;
  border: 1px solid var(--line); border-radius: 10px; background: var(--panel); color: var(--text); box-shadow: var(--elev-3);
}
.report[open] { animation: report-in var(--dur-3) var(--ease-pop); }
.report::backdrop { background: var(--scrim); animation: fade var(--dur-2) var(--ease-out); }
@keyframes report-in { from { opacity: 0; scale: 0.96; } }
@keyframes fade { from { opacity: 0; } }
header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.4rem; }
h2 { margin: 0; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.x { padding: 0.35em 0.45em; }
.what { margin: 0 0 0.8rem; color: var(--muted); font-size: var(--step--1); }
dl { display: grid; grid-template-columns: max-content 1fr; gap: 0.3rem 0.9rem; margin: 0 0 0.8rem; font-size: var(--step--1); }
dt { color: var(--muted); }
dd { margin: 0; overflow-wrap: anywhere; }
.log summary { cursor: pointer; font-size: var(--step--1); }
.log small { color: var(--faint); font-variant-numeric: tabular-nums; }
.log pre {
  max-height: 16rem; overflow: auto; margin: 0.5rem 0 0; padding: 0.6rem 0.7rem; border: 1px solid var(--line);
  border-radius: 6px; background: var(--raised); font-size: 0.72rem; font-variation-settings: 'MONO' 1; white-space: pre;
}
.actions { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 1rem; }
.actions a { text-decoration: none; }
.hint { margin: 0.5rem 0 0; color: var(--faint); font-size: var(--step--2); }
</style>
