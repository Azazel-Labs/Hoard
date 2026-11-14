import '@fontsource-variable/recursive/full.css'
import './styles.css'
import './theme'
import { createApp, watch } from 'vue'
import { useLanguage } from './i18n'
import { prefs } from './prefs'
import Root from './Root.vue'

// The first screen is already in the reader's language: its catalog loads before anything is drawn.
await useLanguage(prefs.language)
watch(() => prefs.language, (pref) => void useLanguage(pref))
createApp(Root).mount('#app')
