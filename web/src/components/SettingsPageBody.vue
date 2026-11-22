<script setup lang="ts">
// What one Settings page shows below its heading: on its own when it's open, or one of several when settings are
// searched (SettingsPage hides the sections that don't match). The settings and saving are SettingsPage's
// (settingsContext).
import type { BackupEvery, Stats } from '../api'
import { isDesktop, may } from '../auth'
import { formatNumber, t } from '../i18n'
import AboutPanel from './AboutPanel.vue'
import AccountPanel from './AccountPanel.vue'
import AddingPanel from './AddingPanel.vue'
import AiPanel from './AiPanel.vue'
import AppearancePanel from './AppearancePanel.vue'
import AuthPanel from './AuthPanel.vue'
import CopiesPanel from './CopiesPanel.vue'
import HoardPanel from './HoardPanel.vue'
import HttpsPanel from './HttpsPanel.vue'
import IssuesPanel from './IssuesPanel.vue'
import LibraryStats from './LibraryStats.vue'
import LocationsPanel from './LocationsPanel.vue'
import MaintenancePanel from './MaintenancePanel.vue'
import NewPip from './NewPip.vue'
import PerformancePanel from './PerformancePanel.vue'
import ResetButton from './ResetButton.vue'
import SearchPanel from './SearchPanel.vue'
import StoreFormatsPanel from './StoreFormatsPanel.vue'
import TagsPanel from './TagsPanel.vue'
import TextureNamesPanel from './TextureNamesPanel.vue'
import ToolsPanel from './ToolsPanel.vue'
import UpdatesPanel from './UpdatesPanel.vue'
import { onOff, useSettings } from './settingsContext'

defineProps<{ id: string; stats: Stats | null }>()
const emit = defineEmits<{
  unauthorized: []
  tag: [string]
  pack: [{ id: string; name: string }]
  open: [string]
  archive: []
  changed: []
  /** Go to another page. */
  section: [string]
}>()
const { s: settings, ready, busy, save, take, opts, saveOpt, resetOpt, changed } = useSettings()
</script>

<template>
  <template v-if="id === 'appearance'">
    <!-- What the server is called, for an admin: the first thing anyone sees of it. -->
    <div v-if="ready && may('admin')" class="body named">
      <section>
        <div class="sh"><h3>{{ t('settings-name') }}</h3></div>
        <p class="what">{{ t('settings-name-what') }}</p>
        <div class="llm opts">
          <label for="opt-name">{{ t('settings-name') }}</label>
          <div class="key">
            <input
              id="opt-name"
              v-model="opts.name"
              class="field"
              maxlength="40"
              :placeholder="settings.options.name.default"
              :disabled="busy"
              @change="saveOpt('name')"
              @keydown.enter.prevent="saveOpt('name')"
            />
            <ResetButton v-if="changed('name')" :to="settings.options.name.default" :disabled="busy" @reset="resetOpt('name')" />
          </div>
        </div>
      </section>
    </div>
    <AppearancePanel />
  </template>

  <LibraryStats
    v-else-if="id === 'library'"
    :stats="stats"
    @tag="(t) => emit('tag', t)"
    @pack="(p) => emit('pack', p)"
    @open="(id) => emit('open', id)"
    @archive="emit('archive')"
  />

  <LocationsPanel v-else-if="id === 'locations'" @unauthorized="emit('unauthorized')" @changed="emit('changed')" />
  <IssuesPanel
    v-else-if="id === 'issues'"
    @unauthorized="emit('unauthorized')"
    @pack="(p) => emit('pack', p)"
    @open="(id) => emit('open', id)"
    @section="(id) => emit('section', id)"
  />
  <AccountPanel v-else-if="id === 'account'" />
  <AboutPanel v-else-if="id === 'about'" :stats="stats" />
  <AuthPanel v-else-if="id === 'auth'" />
  <CopiesPanel v-else-if="id === 'copies'" @open="(id) => emit('open', id)" @unauthorized="emit('unauthorized')" />
  <TagsPanel v-else-if="id === 'tags'" @tag="(t) => emit('tag', t)" @changed="emit('changed')" />
  <StoreFormatsPanel v-else-if="id === 'store-files'" />
  <HoardPanel v-else-if="id === 'dragon'" @unauthorized="emit('unauthorized')" />

  <p v-else-if="!ready" class="loading">{{ t('settings-loading') }}</p>

  <div v-else class="body">
    <ToolsPanel v-if="id === 'tools'" />
    <SearchPanel v-else-if="id === 'search'" />
    <AiPanel v-else-if="id === 'ai'" />
    <AddingPanel v-else-if="id === 'adding'" />
    <TextureNamesPanel v-else-if="id === 'downloads'" />

    <template v-else-if="id === 'connection'">
      <section>
        <div class="sh"><h3>{{ t('settings-address') }}</h3></div>
        <p class="what">{{ t('settings-address-what') }}</p>
        <div class="llm opts">
          <label for="opt-public">{{ t('settings-public-url') }}</label>
          <div class="key">
            <input
              id="opt-public"
              v-model="opts.public_url"
              class="field mono"
              type="url"
              spellcheck="false"
              :placeholder="t('settings-public-url-placeholder')"
              :disabled="busy"
              @change="saveOpt('public_url')"
              @keydown.enter.prevent="saveOpt('public_url')"
            />
            <ResetButton
              v-if="changed('public_url')"
              :to="settings.options.public_url.default || t('settings-public-url-default')"
              :disabled="busy"
              @reset="resetOpt('public_url')"
            />
          </div>
        </div>
      </section>
      <HttpsPanel />
    </template>

    <PerformancePanel v-else-if="id === 'performance'" :stats="stats" />

    <template v-else-if="id === 'maintenance'">
      <section>
        <div class="sh"><h3>{{ t('settings-updates') }}</h3></div>
        <p v-if="stats?.version" class="what">{{ t('settings-version', { version: stats.version }) }}</p>
        <p v-if="isDesktop()" class="what">{{ t('settings-desktop-updates') }}</p>
        <div v-else class="switches">
          <div class="opt-row">
            <label class="check">
              <input
                type="checkbox"
                :checked="settings.options.update_check.value"
                :disabled="busy"
                @change="save({ update_check: ($event.target as HTMLInputElement).checked })"
              />
              {{ t('settings-update-check') }}
            </label>
            <ResetButton
              v-if="changed('update_check')"
              :to="onOff(settings.options.update_check.default)"
              :disabled="busy"
              @reset="resetOpt('update_check')"
            />
          </div>
        </div>
        <UpdatesPanel v-if="!isDesktop()" :settings="settings" @saved="take" @unauthorized="emit('unauthorized')" />
      </section>
      <section>
        <div class="sh"><h3>{{ t('settings-backups') }}<NewPip id="backups" /></h3></div>
        <p class="what">{{ t('settings-backups-what') }}</p>
        <div class="llm opts">
          <label for="opt-backup-every">{{ t('settings-backup-every') }}</label>
          <div class="key">
            <select
              id="opt-backup-every"
              class="field"
              :value="settings.options.backup_every.value"
              :disabled="busy"
              @change="save({ backup_every: ($event.target as HTMLSelectElement).value as BackupEvery })"
            >
              <option value="off">{{ t('settings-backup-off') }}</option>
              <option value="daily">{{ t('settings-backup-daily') }}</option>
              <option value="weekly">{{ t('settings-backup-weekly') }}</option>
            </select>
            <ResetButton v-if="changed('backup_every')" :to="t('settings-backup-off')" :disabled="busy" @reset="resetOpt('backup_every')" />
          </div>
          <label for="opt-backup-keep">{{ t('settings-backup-keep') }}</label>
          <div class="key">
            <input
              id="opt-backup-keep"
              v-model.number="opts.backup_keep"
              class="field num"
              type="number"
              min="1"
              max="365"
              :disabled="busy"
              @change="saveOpt('backup_keep')"
              @keydown.enter.prevent="saveOpt('backup_keep')"
            />
            <span class="unit">{{ t('settings-backup-keep-unit') }}</span>
            <ResetButton
              v-if="changed('backup_keep')"
              :to="formatNumber(settings.options.backup_keep.default)"
              :disabled="busy"
              @reset="resetOpt('backup_keep')"
            />
          </div>
          <label for="opt-backup-copy-to">{{ t('settings-backup-copy-to') }}</label>
          <div class="key">
            <input
              id="opt-backup-copy-to"
              v-model="opts.backup_copy_to"
              class="field"
              :placeholder="t('settings-backup-copy-to-none')"
              :disabled="busy"
              spellcheck="false"
              @change="saveOpt('backup_copy_to')"
              @keydown.enter.prevent="saveOpt('backup_copy_to')"
            />
            <ResetButton v-if="changed('backup_copy_to')" :to="t('settings-backup-copy-to-none')" :disabled="busy" @reset="resetOpt('backup_copy_to')" />
          </div>
        </div>
        <p class="what">{{ t('settings-backups-kept') }}</p>
      </section>
      <MaintenancePanel @unauthorized="emit('unauthorized')" />
    </template>
  </div>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.loading { color: var(--muted); font-size: var(--step--1); }
.body { display: grid; gap: 1rem; }
/* Appearance: the server's name, then the looks the same gap below it. */
.body.named { margin-bottom: 1rem; }
</style>
