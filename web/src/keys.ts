import { t } from './i18n'

/** A musical key, as the server names and tags it (`E♭ major`, `key:e-flat-major`). */
export interface MusicKey {
  /** Its tag without `key:`: `a-minor`, `e-flat-major`. What the search sends. */
  value: string
  /** The note it's on, as written: `E♭`, `F♯`. */
  note: string
  minor: boolean
  /** Its place on the Camelot wheel, as DJs write keys: `8A` for A minor, `8B` for C major. */
  camelot: string
}

const MAJOR_NOTES = ['C', 'D♭', 'D', 'E♭', 'E', 'F', 'F♯', 'G', 'A♭', 'A', 'B♭', 'B']
const MINOR_NOTES = ['C', 'C♯', 'D', 'E♭', 'E', 'F', 'F♯', 'G', 'G♯', 'A', 'B♭', 'B']

function key(tonic: number, minor: boolean): MusicKey {
  const note = (minor ? MINOR_NOTES : MAJOR_NOTES)[tonic]
  const major = minor ? (tonic + 3) % 12 : tonic
  const value = `${note.toLowerCase().replace('♭', '-flat').replace('♯', '-sharp')}-${minor ? 'minor' : 'major'}`
  return { value, note, minor, camelot: `${((major * 7 + 7) % 12) + 1}${minor ? 'A' : 'B'}` }
}

/** Every key, the majors then the minors, each from C up. */
export const KEYS: MusicKey[] = [false, true].flatMap((minor) => MAJOR_NOTES.map((_, i) => key(i, minor)))

/** A key by its value (`a-minor`) or its name (`A minor`, as the server writes one in an asset's facts). */
export function findKey(s: string): MusicKey | undefined {
  const v = s.trim().toLowerCase().replace('♭', '-flat').replace('♯', '-sharp').replace(/\s+/g, '-')
  return KEYS.find((k) => k.value === v)
}

/** A key in words: "A minor". */
export const keyName = (k: MusicKey) => (k.minor ? t('music-key-minor', { note: k.note }) : t('music-key-major', { note: k.note }))
