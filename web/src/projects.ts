// Where assets are used: the projects that `hoard sync` into. Each sync tells the server what its hoard.lock holds,
// so an asset's details list the projects that have it, the library can be filtered to one project's assets, and
// archiving or deleting something a project uses asks first, since that project's next sync would take it out.
import { ref } from 'vue'
import { api, type Project, type ProjectUse, type UsageItems } from './api'

export const projects = ref<Project[]>([])

export async function loadProjects(): Promise<void> {
  try {
    projects.value = (await api.projects()).projects
  } catch {
    // Kept as it was; the next poll tries again.
  }
}

/** "a, b and c". */
function listText(parts: string[]): string {
  return parts.length < 2 ? (parts[0] ?? '') : `${parts.slice(0, -1).join(', ')} and ${parts[parts.length - 1]}`
}

/** One project's share: "Forest (props/crate.obj and 2 more)". */
function useText(u: ProjectUse): string {
  const more = u.assets - u.examples.length
  const files = u.examples.map((e) => e.path)
  if (more > 0) files.push(`${more.toLocaleString()} more`)
  return `${u.name} (${listText(files)})`
}

/**
 * Which projects use any of these, as a warning for a confirm dialog: null if none do (or the server couldn't say,
 * which shouldn't stop anyone archiving).
 */
export async function usageWarning(items: UsageItems): Promise<{ title: string; body: string } | null> {
  let uses: ProjectUse[]
  try {
    uses = (await api.usage(items)).projects
  } catch {
    return null
  }
  if (!uses.length) return null
  const shown = uses.slice(0, 3).map(useText)
  const rest = uses.length - shown.length
  if (rest > 0) shown.push(`${rest} other project${rest === 1 ? '' : 's'}`)
  const title = uses.length === 1 ? `${uses[0].name} uses this` : `${uses.length} projects use this`
  const each = uses.length === 1 ? 'its' : 'each one’s'
  return { title, body: `Used in ${listText(shown)}: ${each} next hoard sync takes them out.` }
}
