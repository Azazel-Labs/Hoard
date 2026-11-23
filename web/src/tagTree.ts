import { shallowRef } from 'vue'
import { api } from './api'

/** Each tag that goes under broader ones, with them. Loaded once when first wanted, and again when the tree changes. */
const parents = shallowRef(new Map<string, string[]>())
let loaded = false

/** Read the tag tree again: after putting a tag under others, say. */
export async function reloadTagTree(): Promise<void> {
  loaded = true
  try {
    const next = new Map<string, string[]>()
    for (const { tag, parent } of await api.tagTree()) next.set(tag, [...(next.get(tag) ?? []), parent])
    parents.value = next
  } catch {
    // Without the tree, nothing shows what it goes under.
  }
}

/** Put a tag under broader ones, in place of those it went under before, and read the tree again. */
export async function setTagParents(tag: string, under: string[]) {
  const got = await api.setTagParents(tag, under)
  await reloadTagTree()
  return got
}

/** Every tag `tag` goes under, however high, nearest first. Reactive: it fills in once the tree has loaded. */
export function broaderTags(tag: string): string[] {
  if (!loaded) void reloadTagTree()
  const tree = parents.value
  const out: string[] = []
  const next = [...(tree.get(tag) ?? [])]
  while (next.length) {
    const t = next.shift() as string
    if (t === tag || out.includes(t)) continue
    out.push(t)
    next.push(...(tree.get(t) ?? []))
  }
  return out
}
