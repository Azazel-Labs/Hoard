<script setup lang="ts">
// Adding to the library: looking packs up on their store's page, what happens to Unreal packages, and the
// conversion rules.
import type { UnrealConvert } from '../api'
import { t } from '../i18n'
import ConversionRules from './ConversionRules.vue'
import ResetButton from './ResetButton.vue'
import { onOff, useSettings } from './settingsContext'

const { s, busy, save, take, resetOpt, changed, unauthorized } = useSettings()

const UNREAL: { value: UnrealConvert; readonly label: string; readonly hint: string }[] = [
  { value: 'off', get label() { return t('settings-unreal-off') }, get hint() { return t('settings-unreal-off-hint') } },
  { value: 'keep', get label() { return t('settings-unreal-keep') }, get hint() { return t('settings-unreal-keep-hint') } },
  { value: 'replace', get label() { return t('settings-unreal-replace') }, get hint() { return t('settings-unreal-replace-hint') } },
]
</script>

<template>
  <section>
    <div class="sh"><h3>{{ t('settings-store-pages') }}</h3></div>
    <p class="what">{{ t('settings-store-pages-what') }}</p>
    <div class="switches">
      <div class="opt-row">
        <label class="check">
          <input
            type="checkbox"
            :checked="s.options.store_lookup.value"
            :disabled="busy"
            @change="save({ store_lookup: ($event.target as HTMLInputElement).checked })"
          />
          {{ t('settings-store-lookup') }}
        </label>
        <ResetButton v-if="changed('store_lookup')" :to="onOff(s.options.store_lookup.default)" :disabled="busy" @reset="resetOpt('store_lookup')" />
      </div>
    </div>
  </section>
  <section>
    <div class="sh">
      <h3>{{ t('settings-unreal') }}</h3>
    <ResetButton
      v-if="s.unreal_convert.value !== s.unreal_convert.default"
      :to="UNREAL.find((u) => u.value === s!.unreal_convert.default)?.label ?? s.unreal_convert.default"
      :disabled="busy"
      @reset="save({ unreal_convert: null }, t('settings-unreal-reset'))"
    />
    </div>
    <p class="what">{{ t('settings-unreal-what') }}</p>
  <div class="choices" role="radiogroup" :aria-label="t('settings-unreal')">
    <label v-for="u in UNREAL" :key="u.value" :class="{ on: s.unreal_convert.value === u.value }">
      <input
        type="radio"
        name="unreal"
        :checked="s.unreal_convert.value === u.value"
        :disabled="busy"
        @change="save({ unreal_convert: u.value })"
      />
      <span><strong>{{ u.label }}</strong><small>{{ u.hint }}</small></span>
    </label>
  </div>
  </section>
  <ConversionRules :settings="s.conversion_rules" :busy="busy" @saved="take" @unauthorized="unauthorized" />
</template>

<style scoped src="./settingsForm.css"></style>
