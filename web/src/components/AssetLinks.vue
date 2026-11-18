<script setup lang="ts">
// The other files this asset goes with: a material's maps, the materials it's in, what it needs and what uses it,
// its variants, and its copies elsewhere. Long lists show their first few.
import { computed, ref, watch } from 'vue'
import { thumbUrl, type Asset, type MaterialInfo, type MaterialMap } from '../api'
import { humanBytes } from '../format'
import { animsOf, formatsOf, linkLabel, needsOf } from '../assetLinks'
import { copyFacts } from '../copies'
import { t } from '../i18n'
import { isMaterialAsset, ROLE_LABEL, ROLE_NAME, sizeLabel } from '../materials'
import KindGlyph from './KindGlyph.vue'

const props = defineProps<{ asset: Asset }>()
defineEmits<{ open: [id: string] }>()

const material = computed(() => props.asset.material ?? null)
const partOf = computed(() => props.asset.materials ?? [])
const family = computed(() => props.asset.family ?? null)
const roleOfHere = (m: MaterialInfo) => {
  const own = m.maps.find((x) => x.id === props.asset.id)
  return own ? t('asset-role-map', { role: ROLE_NAME[own.role] }) : t('asset-maps-count', { count: m.maps.length })
}
/** A model wears these materials: each lists its maps under it by what they're for, not by their long file names. */
const wears = (m: MaterialInfo) => !m.maps.some((x) => x.id === props.asset.id)
/** Files shown under a material already, so "Needs" doesn't list them again. */
const inMaterials = computed(() => new Set(partOf.value.flatMap((m) => [m.id, ...m.maps.map((x) => x.id)])))
const mapSize = (m: MaterialMap) => (m.resolution ? sizeLabel(m.resolution) : m.width ? `${m.width}×${m.height}` : m.ext)
// The materials it belongs to have a section of their own.
const usedBy = computed(() =>
  props.asset.linked_from.filter((l) => l.rel !== 'sprite' && l.rel !== 'format' && !partOf.value.some((m) => m.id === l.id)),
)
// A sheet's sprites the pack has as loose images too, and on one of those, the sheet it's a sprite of.
const looseSprites = computed(() => props.asset.links.filter((l) => l.rel === 'sprite'))
const inSheets = computed(() => props.asset.linked_from.filter((l) => l.rel === 'sprite'))
// The same asset in the pack's other formats, and on one of those, the format it's shown as.
const otherFormats = computed(() => formatsOf(props.asset))
const shownAs = computed(() => props.asset.linked_from.filter((l) => l.rel === 'format'))
const linkGroups = computed(() =>
  [
    { key: 'needs', title: t('asset-needs'), items: isMaterialAsset(props.asset) ? [] : needsOf(props.asset).filter((l) => !inMaterials.value.has(l.id)) },
    { key: 'animations', title: t('asset-animations'), items: animsOf(props.asset) },
    { key: 'loose', title: t('asset-loose-sprites'), items: looseSprites.value },
    { key: 'in-sheet', title: t('asset-in-sheet'), items: inSheets.value },
    { key: 'formats', title: t('asset-other-formats'), items: otherFormats.value },
    { key: 'shown-as', title: t('asset-shown-as'), items: shownAs.value },
    { key: 'used-by', title: t('asset-used-by'), items: usedBy.value },
  ].filter((g) => g.items.length),
)
/** Long lists show their first few: the panel is about one asset, and a 40-texture "Needs" pushed the rest of it
 *  out of reach. A list only a couple longer than that shows whole, rather than hiding two behind a button. */
const LIST_PEEK = 6
const unfolded = ref(new Set<string>())
watch(() => props.asset.id, () => (unfolded.value = new Set()))
function peek<T>(key: string, items: T[]): T[] {
  return unfolded.value.has(key) || items.length <= LIST_PEEK + 2 ? items : items.slice(0, LIST_PEEK)
}
const unfold = (key: string) => (unfolded.value = new Set(unfolded.value).add(key))
const duplicates = computed(() => props.asset.duplicates ?? [])
const nearCopies = computed(() => props.asset.near_copies ?? [])
</script>

<template>
  <section v-if="material">
    <h3>{{ t('asset-maps') }}</h3>
    <ul class="links">
      <li v-for="m in peek('maps', material.maps)" :key="m.id">
        <button type="button" class="link" style="--kind: var(--k-texture)" :title="m.path" @click="$emit('open', m.id)">
          <span class="lthumb checker">
            <img v-if="m.has_thumb" :src="thumbUrl(m)" alt="" loading="lazy" />
            <KindGlyph v-else kind="texture" :size="22" />
          </span>
          <span class="lname">{{ ROLE_LABEL[m.role] }}</span>
          <span class="lrel mono">{{ mapSize(m) }}</span>
        </button>
      </li>
    </ul>
    <button v-if="peek('maps', material.maps).length < material.maps.length" type="button" class="linkish small showmore" @click="unfold('maps')">
      {{ t('asset-show-more', { count: material.maps.length - LIST_PEEK }) }}
    </button>
  </section>

  <section v-if="partOf.length">
    <h3>{{ t('asset-materials-heading', { count: partOf.length }) }}</h3>
    <ul class="links">
      <li v-for="m in partOf" :key="m.id">
        <button type="button" class="link" style="--kind: var(--k-material)" @click="$emit('open', m.id)">
          <span class="lthumb checker">
            <img v-if="m.has_thumb" :src="thumbUrl(m)" alt="" loading="lazy" />
            <KindGlyph v-else kind="material" :size="22" />
          </span>
          <span class="lname">{{ m.name }}</span>
          <span class="lrel">{{ roleOfHere(m) }}</span>
        </button>
        <ul v-if="wears(m)" class="links maps" :aria-label="t('asset-material-maps-aria', { name: m.name })">
          <li v-for="x in peek(`maps-${m.id}`, m.maps)" :key="x.id">
            <button type="button" class="link" style="--kind: var(--k-texture)" :title="x.path" @click="$emit('open', x.id)">
              <span class="lthumb checker">
                <img v-if="x.has_thumb" :src="thumbUrl(x)" alt="" loading="lazy" />
                <KindGlyph v-else kind="texture" :size="22" />
              </span>
              <span class="lname">{{ ROLE_LABEL[x.role] }}</span>
              <span class="lrel mono">{{ mapSize(x) }}</span>
            </button>
          </li>
        </ul>
        <button
          v-if="wears(m) && peek(`maps-${m.id}`, m.maps).length < m.maps.length"
          type="button"
          class="linkish small showmore"
          @click="unfold(`maps-${m.id}`)"
        >
          {{ t('asset-show-more', { count: m.maps.length - LIST_PEEK }) }}
        </button>
      </li>
    </ul>
  </section>

  <section v-for="g in linkGroups" :key="g.key">
    <h3>{{ g.title }}</h3>
    <ul class="links">
      <li v-for="l in peek(g.key, g.items)" :key="l.id + l.rel">
        <button type="button" class="link" :style="{ '--kind': `var(--k-${l.kind})` }" @click="$emit('open', l.id)">
          <span class="lthumb checker">
            <img v-if="l.has_thumb" :src="thumbUrl(l)" :class="{ glyphs: l.kind === 'font' }" alt="" loading="lazy" />
            <KindGlyph v-else :kind="l.kind" :size="22" />
          </span>
          <span class="lname">{{ l.name }}</span>
          <span class="lrel">{{ linkLabel(l) }}<template v-if="l.size != null"> · {{ humanBytes(l.size) }}</template></span>
        </button>
      </li>
    </ul>
    <button v-if="peek(g.key, g.items).length < g.items.length" type="button" class="linkish small showmore" @click="unfold(g.key)">
      {{ t('asset-show-more', { count: g.items.length - LIST_PEEK }) }}
    </button>
  </section>

  <section v-if="family">
    <h3>{{ t('asset-variants', { count: family.size }) }}</h3>
    <ul class="links">
      <li v-for="l in peek('variants', family.members)" :key="l.id">
        <button type="button" class="link" :style="{ '--kind': `var(--k-${l.kind})` }" @click="$emit('open', l.id)">
          <span class="lthumb checker">
            <img v-if="l.has_thumb" :src="thumbUrl(l)" :class="{ glyphs: l.kind === 'font' }" alt="" loading="lazy" />
            <KindGlyph v-else :kind="l.kind" :size="22" />
          </span>
          <span class="lname">{{ l.name }}</span>
          <span class="lrel">{{ l.path.split('/').pop() }}</span>
        </button>
      </li>
    </ul>
    <button v-if="peek('variants', family.members).length < family.members.length" type="button" class="linkish small showmore" @click="unfold('variants')">
      {{ t('asset-show-more', { count: family.members.length - LIST_PEEK }) }}
    </button>
  </section>

  <section v-if="duplicates.length">
    <h3>{{ t('asset-copies') }}</h3>
    <p class="legend dupnote">{{ t('asset-copies-note') }}</p>
    <ul class="links">
      <li v-for="d in peek('copies', duplicates)" :key="d.id">
        <button type="button" class="link dup" @click="$emit('open', d.id)">
          <span class="lname">{{ d.pack_name }}</span>
          <span class="lrel">{{ d.path }}</span>
        </button>
      </li>
    </ul>
    <button v-if="peek('copies', duplicates).length < duplicates.length" type="button" class="linkish small showmore" @click="unfold('copies')">
      {{ t('asset-show-more', { count: duplicates.length - LIST_PEEK }) }}
    </button>
  </section>

  <section v-if="nearCopies.length" data-testid="near-copies">
    <h3>{{ t('asset-near-copies') }}</h3>
    <p class="legend dupnote">{{ nearCopies.some((n) => n.best) ? t('asset-near-copies-note') : t('asset-near-copies-best') }}</p>
    <ul class="links">
      <li v-for="n in peek('near', nearCopies)" :key="n.id">
        <button type="button" class="link dup near" :title="n.path" @click="$emit('open', n.id)">
          <span class="lname">{{ n.pack_name }}<span v-if="n.best" class="best">{{ t('asset-near-best') }}</span></span>
          <span class="lrel">{{ copyFacts(n) }}</span>
        </button>
      </li>
    </ul>
    <button v-if="peek('near', nearCopies).length < nearCopies.length" type="button" class="linkish small showmore" @click="unfold('near')">
      {{ t('asset-show-more', { count: nearCopies.length - LIST_PEEK }) }}
    </button>
  </section>
</template>

<style scoped>
/* The sections sit straight in the panel's column, which mustn't squash them. */
section { flex-shrink: 0; }
.showmore { margin-top: 0.45rem; color: var(--muted); }
.showmore:hover { color: var(--text); }
.links { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 0.3rem; }
/* A material's maps, under it. */
.links.maps { margin: 0.3rem 0 0.2rem 1.4rem; padding-left: 0.7rem; border-left: 1px solid var(--line); }
.link {
  display: grid; grid-template-columns: 32px 1fr auto; gap: 0.6rem; align-items: center; width: 100%;
  padding: 0.25rem; border: 1px solid transparent; border-radius: var(--r-chip); background: none; cursor: pointer; text-align: left;
}
.link:hover { border-color: var(--line); background: var(--raised); }
.lthumb {
  width: 32px; height: 32px; border-radius: 3px; overflow: hidden; display: grid; place-items: center;
  box-shadow: inset 0 2px 0 var(--kind);
}
.lthumb img { width: 100%; height: 100%; object-fit: contain; }
.lname { font-size: var(--step--1); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.lrel { font-size: 0.75rem; color: var(--faint); }
.dupnote { margin: 0 0 0.4rem; }
.link.dup { grid-template-columns: 1fr auto; }
.link.dup .lrel { max-width: 22ch; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; }
.link.near .lrel { max-width: none; direction: ltr; }
.best {
  margin-left: 0.45rem; font-size: 0.85em; font-weight: 500; padding: 0 0.45rem; border-radius: 999px;
  background: color-mix(in srgb, var(--ok) 16%, transparent); color: var(--ok);
}
</style>
<style scoped src="./detailSection.css"></style>
