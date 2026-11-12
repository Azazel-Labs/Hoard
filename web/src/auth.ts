// Who this browser is signed in as. The session itself is an HttpOnly cookie the server sets; this only mirrors what
// `GET /api/auth/status` says, so the portal knows whether to show the library or the sign-in screen.
import { reactive } from 'vue'
import { api, whenSignInNeeded, type AuthStatus } from './api'

export const auth = reactive({
  status: null as AuthStatus | null,
  /** Couldn't reach the server to ask. */
  error: '',
  /** Someone browsing as a guest (or whose session ran out) tried something that needs an account. */
  prompt: false,
})

export async function refreshAuth(): Promise<void> {
  try {
    auth.status = await api.authStatus()
    auth.error = ''
  } catch (e) {
    auth.error = (e as Error).message
  }
}

/** The library can be shown: signed in, or anyone may browse. */
export function canBrowse(): boolean {
  const s = auth.status
  return !!s && !s.setup_needed && (s.signed_in || s.guest_browsing)
}

export function askToSignIn() {
  auth.prompt = true
}

export async function signOut(): Promise<void> {
  auth.prompt = false
  try {
    await api.logout()
  } finally {
    await refreshAuth()
  }
}

// Any request the server turns away for want of an account asks to sign in, then rechecks: a session that ran out
// while the library was open shows the sign-in screen rather than a string of errors.
whenSignInNeeded(() => {
  auth.prompt = true
  refreshAuth()
})
