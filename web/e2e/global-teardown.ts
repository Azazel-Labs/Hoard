// Stops the hoardd global setup started (by its PID, nothing else) and removes its temporary folders.
import { existsSync, readFileSync, rmSync } from 'node:fs'
import { RUN_FILE, type RunInfo } from './support/paths'

function alive(pid: number): boolean {
  try {
    process.kill(pid, 0)
    return true
  } catch {
    return false
  }
}

export default async function globalTeardown(): Promise<void> {
  if (!existsSync(RUN_FILE)) return
  const run = JSON.parse(readFileSync(RUN_FILE, 'utf8')) as RunInfo
  if (alive(run.pid)) {
    process.kill(run.pid)
    const deadline = Date.now() + 10_000
    while (alive(run.pid) && Date.now() < deadline) await new Promise((r) => setTimeout(r, 100))
  }
  // The server may hold files a moment after it's gone on Windows; a few tries, then leave it to the OS.
  for (let i = 0; i < 20; i++) {
    try {
      rmSync(run.root, { recursive: true, force: true })
      break
    } catch {
      await new Promise((r) => setTimeout(r, 250))
    }
  }
  rmSync(RUN_FILE, { force: true })
}
