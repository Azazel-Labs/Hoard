import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const e2eDir = join(dirname(fileURLToPath(import.meta.url)), '..')

/** The signed-in browser state global setup saves. */
export const AUTH_FILE = join(e2eDir, '.auth', 'state.json')
/** What global teardown needs to stop the server and tidy up. */
export const RUN_FILE = join(e2eDir, '.auth', 'run.json')

export interface RunInfo {
  pid: number
  root: string
  url: string
  logFile: string
}

/** Ids of the seeded packs by name, and of their assets by `pack name/asset name`. */
export interface Seed {
  packs: Record<string, string>
  assets: Record<string, string>
}
