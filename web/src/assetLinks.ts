// An asset's links to other files, sorted the ways the detail panel shows them and the zip of it takes them along.
import type { Asset, AssetLink, LinkRel } from './api'
import { t } from './i18n'

/** A file's extension, upper case (`PNG`); `''` for none. */
export const extOf = (path: string) => (path.includes('.') ? path.slice(path.lastIndexOf('.') + 1).toUpperCase() : '')

/** What it needs to work (its textures, buffers, library…): what the zip of it takes along. */
export const needsOf = (a: Asset) => a.links.filter((l) => l.rel !== 'animation' && l.rel !== 'sprite' && l.rel !== 'format')
/** Animations for it, which the zip takes along when asked. */
export const animsOf = (a: Asset) => a.links.filter((l) => l.rel === 'animation')
/** The same asset in the pack's other formats. */
export const formatsOf = (a: Asset) => a.links.filter((l) => l.rel === 'format')

// Getters, so the words are read at render time in the current language.
const REL_LABEL: Record<Exclude<LinkRel, 'format'>, string> = {
  get texture() { return t('asset-rel-texture') },
  get buffer() { return t('asset-rel-buffer') },
  get material() { return t('asset-rel-material') },
  get animation() { return t('asset-rel-animation') },
  get library() { return t('asset-rel-library') },
  get guess() { return t('asset-rel-guess') },
  get sidecar() { return t('asset-rel-sidecar') },
  get variant() { return t('asset-rel-variant') },
  get sprite() { return t('asset-rel-sprite') },
  get sample() { return t('asset-rel-sample') },
}

/** How a linked file relates to this one, in words (another format says which). */
export const linkLabel = (l: AssetLink) => (l.rel === 'format' ? extOf(l.path) : REL_LABEL[l.rel])
