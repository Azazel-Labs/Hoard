// Light, dark or follow the system. The resolved scheme goes on <html> as `data-scheme`, which sets `color-scheme`
// in styles.css: every light-dark() color follows it, along with form controls and scrollbars. index.html applies
// the saved choice before first paint; this keeps it current.
import { watch } from 'vue'
import { prefs } from './prefs'

const lightQuery = window.matchMedia('(prefers-color-scheme: light)')

function apply() {
  const scheme = prefs.theme === 'light' || prefs.theme === 'dark' ? prefs.theme : lightQuery.matches ? 'light' : 'dark'
  document.documentElement.dataset.scheme = scheme
}

watch(() => prefs.theme, apply, { immediate: true })
lightQuery.addEventListener('change', apply)
