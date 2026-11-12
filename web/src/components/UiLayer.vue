<script setup lang="ts">
// Renders ui.ts: the toast stack and the confirm dialog.
import { nextTick, ref, watch } from 'vue'
import { act, answerDialog, dialog, dismiss, pause, resume, toasts } from '../ui'

const dlg = ref<HTMLDialogElement | null>(null)
const confirmBtn = ref<HTMLButtonElement | null>(null)
watch(() => dialog.open, async (o) => {
  await nextTick()
  if (o && dlg.value && !dlg.value.open) {
    dlg.value.showModal()
    // Destructive questions start on Cancel, so a stray Enter doesn't delete anything.
    if (!o.danger) confirmBtn.value?.focus()
  } else if (!o && dlg.value?.open) {
    dlg.value.close()
  }
})
function onCancel(e: Event) {
  e.preventDefault()
  answerDialog(false)
}
// Clicks on the backdrop land on the <dialog> itself.
function onDialogClick(e: MouseEvent) {
  if (e.target === dlg.value) answerDialog(false)
}
</script>

<template>
  <TransitionGroup tag="ol" name="toast" class="toasts" aria-label="Notifications">
    <li
      v-for="t in toasts"
      :key="t.id"
      class="toast"
      :class="[t.kind, { paused: t.paused }]"
      :role="t.kind === 'error' ? 'alert' : 'status'"
      @mouseenter="pause(t.id)"
      @mouseleave="resume(t.id)"
      @focusin="pause(t.id)"
      @focusout="resume(t.id)"
    >
      <span :key="t.repeat" class="icon" :class="{ nudge: t.repeat }" aria-hidden="true">
        <svg v-if="t.kind === 'ok'" viewBox="0 0 16 16" width="14" height="14"><path d="M3.5 8.5 6.5 11.5 12.5 4.5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" /></svg>
        <svg v-else-if="t.kind === 'error'" viewBox="0 0 16 16" width="14" height="14"><path d="M8 4.5v4.5M8 11.6v.1" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" /></svg>
        <svg v-else viewBox="0 0 16 16" width="14" height="14"><path d="M3 4.5h10M6.5 4.5V3h3v1.5M4.5 4.5l.6 8.5h5.8l.6-8.5" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
      </span>
      <span class="text">{{ t.text }}</span>
      <button v-if="t.action" type="button" class="act" @click="act(t.id)">{{ t.action.label }}</button>
      <button type="button" class="x" aria-label="Dismiss" @click="dismiss(t.id)">
        <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
      <span :key="`${t.repeat}-timer`" class="timer" :style="{ animationDuration: `${t.duration}ms` }" aria-hidden="true" />
    </li>
  </TransitionGroup>

  <dialog ref="dlg" class="confirm" :aria-label="dialog.open?.title" @cancel="onCancel" @click="onDialogClick">
    <form v-if="dialog.open" method="dialog" @submit.prevent="answerDialog(true)">
      <h2>{{ dialog.open.title }}</h2>
      <p v-if="dialog.open.body">{{ dialog.open.body }}</p>
      <div class="row">
        <button type="button" class="btn ghost" autofocus @click="answerDialog(false)">{{ dialog.open.cancel ?? 'Cancel' }}</button>
        <button ref="confirmBtn" type="submit" class="btn" :class="dialog.open.danger ? 'danger-solid' : 'primary'">{{ dialog.open.confirm }}</button>
      </div>
    </form>
  </dialog>
</template>

<style scoped>
.toasts {
  position: fixed; top: 4.2rem; left: 50%; translate: -50% 0; z-index: 60;
  display: flex; flex-direction: column; align-items: center; gap: 0.5rem;
  width: min(560px, calc(100vw - 2rem)); margin: 0; padding: 0; list-style: none; pointer-events: none;
}
.toast {
  --tone: var(--muted);
  position: relative; overflow: hidden; pointer-events: auto; max-width: 100%;
  display: flex; align-items: center; gap: 0.6rem; padding: 0.55rem 0.5rem 0.6rem 0.75rem;
  background: var(--raised); border: 1px solid var(--line); border-radius: 8px;
  box-shadow: var(--elev-3); font-size: var(--step--1);
}
.toast.ok { --tone: var(--ok); }
.toast.error { --tone: var(--danger); border-color: color-mix(in srgb, var(--danger) 55%, var(--line)); background: color-mix(in srgb, var(--danger) 8%, var(--raised)); }
.icon {
  flex: none; display: grid; place-items: center; width: 1.35rem; height: 1.35rem; border-radius: 50%;
  color: var(--tone); background: color-mix(in srgb, var(--tone) 16%, transparent);
}
.icon.nudge { animation: nudge var(--dur-3) var(--ease-pop); }
@keyframes nudge { 30% { scale: 1.3; } }
.text { flex: 1; min-width: 0; overflow-wrap: anywhere; }
.act {
  flex: none; border: 0; border-radius: var(--r-chip); padding: 0.3em 0.65em; cursor: pointer;
  background: color-mix(in srgb, var(--text) 10%, transparent); font-weight: 650; font-size: var(--step--1);
  transition: background-color var(--dur-1), scale var(--dur-1) var(--ease-out);
}
.act:hover { background: color-mix(in srgb, var(--text) 18%, transparent); }
.act:active { scale: 0.95; }
.x {
  flex: none; display: grid; place-items: center; width: 1.6rem; height: 1.6rem; padding: 0; border: 0; border-radius: 50%;
  background: none; color: var(--faint); cursor: pointer; transition: background-color var(--dur-1), color var(--dur-1);
}
.x:hover { color: var(--text); background: color-mix(in srgb, var(--text) 10%, transparent); }
/* Time left before the toast goes: drains left to right, holds while hovered. */
.timer {
  position: absolute; left: 0; right: 0; bottom: 0; height: 2px; background: var(--tone); opacity: 0.55;
  transform-origin: left; animation: drain linear forwards;
}
.paused .timer { animation-play-state: paused; }
@keyframes drain { to { scale: 0 1; } }

.toast-enter-active { transition: opacity var(--dur-2) var(--ease-out), translate var(--dur-3) var(--ease-pop), scale var(--dur-3) var(--ease-pop); }
.toast-leave-active { transition: opacity var(--dur-2) var(--ease-out), scale var(--dur-2) var(--ease-out); position: absolute; }
.toast-enter-from { opacity: 0; translate: 0 -14px; scale: 0.94; }
.toast-leave-to { opacity: 0; scale: 0.94; }
.toast-move { transition: translate var(--dur-3) var(--ease-out); }

.confirm {
  width: min(28rem, calc(100vw - 2rem)); padding: 0; border: 1px solid var(--line); border-radius: 12px;
  background: var(--raised); color: var(--text); box-shadow: 0 24px 64px var(--shadow);
}
.confirm[open] { animation: dialog-in var(--dur-3) var(--ease-pop); }
.confirm::backdrop { background: var(--scrim); backdrop-filter: blur(3px); animation: fade-in var(--dur-2) var(--ease-out); }
@keyframes dialog-in { from { opacity: 0; scale: 0.94; translate: 0 8px; } }
@keyframes fade-in { from { opacity: 0; } }
.confirm form { display: flex; flex-direction: column; gap: 0.6rem; padding: 1.25rem 1.25rem 1rem; }
.confirm h2 { margin: 0; font-size: var(--step-1); font-weight: 650; font-variation-settings: 'CASL' 0.4; }
.confirm p { margin: 0; color: var(--muted); font-size: var(--step--1); }
.row { display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 0.6rem; }
.danger-solid { background: var(--danger); border-color: var(--danger); color: var(--bg); font-weight: 650; }
.danger-solid:hover:not(:disabled) { background: color-mix(in srgb, var(--danger) 85%, var(--text)); }
</style>
