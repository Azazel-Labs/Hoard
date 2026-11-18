// What each kind of asset is called, and what "more like this" works for.
import { t } from './i18n'
import type { Asset, AssetKind } from './types/library'

/** Each kind's name, in the current language: getters, so `KIND_LABEL[k]` reads it when it's shown. */
export const KIND_LABEL: Record<AssetKind, string> = {
  get texture() { return t('api-kind-texture') },
  get vector() { return t('api-kind-vector') },
  get sheet() { return t('api-kind-sheet') },
  get audio() { return t('api-kind-audio') },
  get model() { return t('api-kind-model') },
  get animation() { return t('api-kind-animation') },
  get font() { return t('api-kind-font') },
  get shader() { return t('api-kind-shader') },
  get material() { return t('api-kind-material') },
  get video() { return t('api-kind-video') },
  get document() { return t('api-kind-document') },
  get data() { return t('api-kind-data') },
  get other() { return t('api-kind-other') },
}

/** Each kind's name as it reads mid-sentence (“Only textures”), in the current language. */
export const KIND_NAME: Record<AssetKind, string> = {
  get texture() { return t('api-kind-texture-name') },
  get vector() { return t('api-kind-vector-name') },
  get sheet() { return t('api-kind-sheet-name') },
  get audio() { return t('api-kind-audio-name') },
  get model() { return t('api-kind-model-name') },
  get animation() { return t('api-kind-animation-name') },
  get font() { return t('api-kind-font-name') },
  get shader() { return t('api-kind-shader-name') },
  get material() { return t('api-kind-material-name') },
  get video() { return t('api-kind-video-name') },
  get document() { return t('api-kind-document-name') },
  get data() { return t('api-kind-data-name') },
  get other() { return t('api-kind-other-name') },
}

/** Kinds whose thumbnails are pictures of the thing itself, so "looks like" means something (mirrors the server). */
const LOOK_KINDS = new Set<AssetKind>(['texture', 'vector', 'model', 'material', 'animation', 'video', 'other'])
/** Sound files the server listens to, for "more like this" by how they sound (mirrors the server's decoders). */
const SOUND_EXTS = new Set(['wav', 'wave', 'ogg', 'oga', 'flac', 'mp3', 'aif', 'aiff', 'aifc'])
/** Whether "More like this" can find anything for an asset. */
export const looksAlike = (a: Asset) =>
  (a.has_thumb && LOOK_KINDS.has(a.kind)) || (a.kind === 'audio' && SOUND_EXTS.has(a.ext.toLowerCase()))
