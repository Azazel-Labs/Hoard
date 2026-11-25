// Features still being worked on, off until someone turns them on in Settings → Experimental. Each is saved in this
// browser (`prefs.experiments`), so trying one changes nothing for anyone else. When a feature is good enough to be
// on for everyone, take it out of EXPERIMENTS and drop its `trying` checks.
import { t } from './i18n'
import { prefs, toggleIn } from './prefs'

export interface Experiment {
  id: string
  readonly label: string
  readonly what: string
}

export const EXPERIMENTS: Experiment[] = [
  { id: 'far-copies', get label() { return t('experiment-far-copies') }, get what() { return t('experiment-far-copies-what') } },
]

/** Whether this browser has turned an experiment on. */
export const trying = (id: string) => prefs.experiments.includes(id)

export const setTrying = (id: string, on: boolean) => {
  if (on !== trying(id)) prefs.experiments = toggleIn(prefs.experiments, id)
}
