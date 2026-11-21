// The server itself, as it sends it: accounts and signing in, its settings, its upkeep and what it's doing.
// Types mirror crates/hoard-core.
import type { TextureNames } from '../texnames'
import type { AssetKind, ConversionRules } from './library'

/** One second of the server, for the profiler: processor use in percent of every core (Hoard's and the whole
 *  machine's), Hoard's RAM and the machine's (in use, and all it has) in bytes, requests in progress, and what was running. Times are Unix milliseconds. */
export interface ProfileSample {
  seq: number
  at: number
  hoard: number
  machine: number
  memory: number
  used: number
  total: number
  serving: number
  running: { kind: string; label: string }[]
}

/** One request served: its route (empty for the portal's own files), then its time in milliseconds, in all and in
 *  the database, the disk and sending. */
export interface ProfileServed {
  seq: number
  at: number
  method: string
  route: string
  status: number
  total: number
  db: number
  disk: number
  send: number
  bytes: number
}

/** The profiler's last ten minutes (`keep`), or what's new since the `seq` passed back. */
export interface Profile {
  seq: number
  now: number
  cores: number
  keep: number
  samples: ProfileSample[]
  served: ProfileServed[]
}

/** Where a library keeps files: managed (Hoard copies files in) or watched (indexed where they sit). */
export type LocationMode = 'managed' | 'watched'

export interface LibraryLocation {
  id: number
  name: string
  mode: LocationMode
  /** A folder on the server, or `s3://bucket`. */
  path: string
  /** The asset kinds new files of which go here (managed locations). */
  kinds: AssetKind[]
  /** Takes every kind no other location ticks. */
  catch_all: boolean
  scanned_at?: string | null
  created_at: string
  /** Whether the folder can be reached right now. */
  online: boolean
  packs: number
}

export interface LocationList {
  locations: LibraryLocation[]
  /** Where the setup suggests the first location goes. */
  suggested: string
  kinds: AssetKind[]
}

export interface NewLocation {
  name?: string
  mode: LocationMode
  path: string
  kinds: AssetKind[]
  catch_all: boolean
}

/** `GET /api/auth/status`: who this browser is signed in as, and what the server wants first. */
/** What an account or token may do: look; look and change the library; or that and Settings and accounts too. */
export type Role = 'view' | 'edit' | 'admin'

export interface AuthStatus {
  /** No account yet: the portal asks for the first one. */
  setup_needed: boolean
  /** The first account needs the code from `HOARD_SETUP_CODE`. */
  setup_code?: boolean
  signed_in: boolean
  user: { name: string; kind: 'user' | 'token'; role: Role; source?: AccountSource } | null
  /** People who aren't signed in may look around. */
  guest_browsing: boolean
  /** What this server is called (Settings → General): Hoard unless it was given a name. */
  name: string
  /** Ways to sign in besides Hoard's own accounts (Settings → Authentication). */
  /** `oauth` is the sign-in button's text, "" when it has none of its own (then it's the reader's "your account"). */
  sign_in?: { directory: string | null; oauth: string | null }
  /** The desktop app's own server: one person, signed in by the app, on this computer only. */
  desktop?: boolean
  /** The desktop app as sold on Steam: its Steam app id. */
  steam_app?: number | null
}

/** Who checks an account's sign-in: Hoard, a directory (LDAP or Active Directory) or an OAuth provider. */
export type AccountSource = 'internal' | 'ldap' | 'oauth'

export interface Account {
  id: number
  username: string
  created_at: string
  role: Role
  source: AccountSource
  you: boolean
}

export type AuthMethod = 'internal' | 'ldap' | 'ad' | 'oauth'
export type LdapTls = 'none' | 'starttls' | 'ldaps'

/** A directory to sign in against: LDAP (a custom server or OpenLDAP) or Active Directory. */
export interface LdapConfig {
  server_type: string
  servers: string[]
  tls: LdapTls
  skip_verify: boolean
  ca_cert: string
  anonymous: boolean
  bind_dn: string
  /** Always empty from the server: see `bind_password_set`. Left empty, the saved one is kept. */
  bind_password: string
  bind_password_set?: boolean
  clear_bind_password?: boolean
  base_dn: string
  username_attribute: string
  user_filter: string
  group_base_dn: string
  group_filter: string
}

export interface OAuthConfig {
  provider: string
  client_id: string
  /** Always empty from the server: see `client_secret_set`. Left empty, the saved one is kept. */
  client_secret: string
  client_secret_set?: boolean
  clear_client_secret?: boolean
  authorization_url: string
  token_url: string
  userinfo_url: string
  scopes: string
  username_claim: string
  groups_claim: string
  button: string
}

/** Settings → Authentication. */
export interface AuthConfig {
  method: AuthMethod
  session_hours: number | null
  provision: boolean
  default_role: Role
  roles: { admin: string[]; edit: string[]; view: string[] }
  ldap: LdapConfig
  ad: LdapConfig
  oauth: OAuthConfig
}

export interface AuthSettings {
  config: AuthConfig
  /** Where the OAuth provider sends people back: give the provider this. */
  callback_url: string
}

export interface LdapTestResult {
  servers: { server: string; ok: boolean; error?: string }[]
  user: { ok: boolean; error?: string; name?: string; dn?: string; groups?: string[]; role?: Role | 'default' | null } | null
}

export interface Connection {
  id: number
  app: string
  user: string
  created_at: string
  last_used_at: string | null
}

/** An app that asked the MCP server something lately (Settings → Account). */
export interface LiveApp {
  name: string
  user: string | null
  client: string | null
  last: string
  seconds_ago: number
  calls: number
}

export interface AccessToken {
  id: number
  name: string
  created_at: string
  last_used_at: string | null
  created_by: string | null
  role: Role
  /** The account that made it: the most it can be given, and the most it can do whatever it was given. */
  maker_role: Role
}

/** A finished (or interrupted) run of a maintenance task. */
export interface MaintenanceRun {
  id: number
  task: string
  started_at: string
  finished_at: string | null
  ok: boolean | null
  summary: string | null
}

export interface MaintenanceTask {
  id: string
  label: string
  blurb: string
  running: { run: number; started_at: string; progress?: number; detail?: string } | null
  last: MaintenanceRun | null
}

/** Settings → Maintenance (`GET /api/admin/maintenance`). */
export interface Maintenance {
  tasks: MaintenanceTask[]
  history: MaintenanceRun[]
}

/** Something that went wrong opening or processing the library, in Settings → Issues. */
export interface Issue {
  /** Opaque: pass it back to retry or dismiss. */
  id: string
  title: string
  message: string
  pack_id: string | null
  pack_name: string | null
  asset_id: string | null
  path: string | null
  location_id: number | null
  location_name: string | null
  /** How many times it's happened. */
  count: number
  at: string | null
  retry: boolean
}

/** One kind of issue. `retry` is the retry button's label, when it has one; at most 200 `items` of `total`. */
export interface IssueGroup {
  kind: string
  title: string
  blurb: string
  retry: string | null
  dismiss: boolean
  total: number
  items: Issue[]
}

/** `GET /api/admin/issues`: only groups that have issues, most serious first. */
export interface Issues {
  total: number
  groups: IssueGroup[]
}

/** Which issues an action is for: some, or a whole group. */
export type IssuePick = { ids: string[] } | { kind: string }

/** An outside program Hoard can use, as the Settings page shows it. */
export interface ToolSetting {
  /** `off`, `auto` or a path. */
  value: string
  /** The built-in default, used when nothing is saved. */
  default: string
  saved: boolean
  state: 'off' | 'looking' | 'downloading' | 'ready' | 'unavailable'
  /** The version when ready, why when unavailable. */
  detail: string
  can_download: boolean
}

export type UnrealConvert = 'off' | 'keep' | 'replace'

/** A server option: what's in effect, what the flags give, and whether the portal changed it. */
export interface OptionSetting<T> {
  value: T
  default: T
  saved: boolean
}

/** A model an AI service offers (`POST /api/admin/llm/models`). */
export interface ModelInfo {
  id: string
  name?: string
  /** Whether it takes images, when the service says. */
  vision?: boolean
}

/** A certificate, as Settings → Connection shows it. Times are Unix seconds. */
export interface CertInfo {
  names: string[]
  subject: string
  issuer: string
  not_before: number
  not_after: number
  self_signed: boolean
  fingerprint: string
}

export type HttpsMode = 'off' | 'own' | 'custom'

/** Settings → Connection (`GET /api/admin/https`). */
export interface HttpsStatus {
  available: boolean
  mode: HttpsMode
  /** Addresses added for Hoard's own certificate. */
  names: string[]
  http3: boolean
  redirect: boolean
  /** HTTPS is being served now. */
  on: boolean
  /** Why it isn't, though it's chosen. */
  error: string | null
  http3_running: boolean
  http3_error: string | null
  /** Addresses Hoard found by itself. */
  found: string[]
  covers_this_address: boolean | null
  /** The certificate being served. */
  cert: CertInfo | null
  /** The uploaded certificate, if there is one. */
  custom: CertInfo | null
  /** Hoard's own certificate authority, once made. */
  ca: CertInfo | null
  port: number
  this_address: string | null
}

export interface HttpsChange {
  mode?: HttpsMode
  names?: string[]
  http3?: boolean
  redirect?: boolean
  cert?: string
  key?: string
}

export interface CertCheck {
  cert: CertInfo
  problem: string | null
  covers_this_address: boolean | null
  this_address: string | null
}

/** The ChatGPT account the server tags through, if it's signed in (`GET /api/admin/chatgpt`). */
export interface ChatGptStatus {
  signed_in: boolean
  email?: string | null
  /** Why tagging can't use the account right now. */
  problem?: string | null
  /** A sign-in was started and hasn't finished. */
  waiting: boolean
  /** Where to see or limit what Hoard uses of the plan. */
  manage_url: string
  /** After signing out: anything the person should still do. */
  note?: string | null
}

/** A model the local model server has, or is downloading (`POST /api/admin/llm/local`). */
export interface LocalModel {
  /** Its Hugging Face reference, which is also the name to use it by. */
  id: string
  /** `downloading`, `incomplete`, `unloaded` (ready), `loading`, `loaded`, `sleeping`. */
  status: string
  /** Bytes done and expected, while downloading. */
  progress?: [number, number]
  vision: boolean
  failed: boolean
}

/** A model Hoard suggests for the local model server. */
export interface SuggestedModel {
  id: string
  name: string
  size_gb: number
  license: string
  note: string
}

export interface LocalModels {
  reachable: boolean
  error?: string
  models: LocalModel[]
  suggested: SuggestedModel[]
}

export interface Settings {
  blender: ToolSetting
  unreal: ToolSetting
  unreal_image: OptionSetting<string>
  /** Who Epic's private Unreal Editor images are downloaded as; the token itself is never sent back. */
  unreal_github: { user: string; has_token: boolean; from_server: boolean }
  ffmpeg: ToolSetting
  unreal_convert: { value: UnrealConvert; default: UnrealConvert; saved: boolean }
  llm: {
    url: string
    model: string
    has_key: boolean
    vision: boolean
    /** Tagging goes through the ChatGPT account signed in on the server. */
    chatgpt: boolean
    batch: OptionSetting<number>
    enabled: boolean
    saved: boolean
    default: { url: string; model: string; has_key: boolean; vision: boolean }
    /** Where the compose files' local model server listens. */
    local_url: string
  }
  options: {
    workers: OptionSetting<number>
    max_upload_mb: OptionSetting<number>
    max_expand_mb: OptionSetting<number>
    render: OptionSetting<boolean>
    update_check: OptionSetting<boolean>
    guest_browsing: OptionSetting<boolean>
    /** Search the stores for packs' pages in the background. */
    store_lookup: OptionSetting<boolean>
    thumb_quality: OptionSetting<ThumbQuality>
    thumb_avif: OptionSetting<boolean>
    /** Make a short looping movie of each animation for its tile. Off by default: it takes space and time. */
    motion_thumbs: OptionSetting<boolean>
    /** Most space cached "Download as" conversions may take, in MiB. */
    conversion_cache_mb: OptionSetting<number>
    /** How often the database backs itself up. */
    backup_every: OptionSetting<BackupEvery>
    /** How many scheduled backups are kept. */
    backup_keep: OptionSetting<number>
    /** A folder on the server each scheduled backup is also copied to. Empty: none. */
    backup_copy_to: OptionSetting<string>
    /** What the portal calls this server. */
    name: OptionSetting<string>
    /** Empty: from the request's Host header. */
    public_url: OptionSetting<string>
    /** Empty: look in the usual places. */
    soundfont: OptionSetting<string>
  }
  max_workers: number
  clip: SearchModelSetting
  clap: SearchModelSetting
  /** Called once a newer release is out and the server is idle. The Authorization header is never sent back. */
  update_hook: { url: string; has_auth: boolean }
  s3: S3Setting
  /** How texture maps are named in downloads, zips and manifests. `maps` has every role's suffix. */
  texture_names: TextureNames & { default: TextureNames; saved: boolean }
  conversion_rules: ConversionRules & { default_keep_days: number; targets: { to: string; from: string[] }[] }
}

/** A store file format: built in, or one of the data folder's (`custom`), which can be changed or removed. */
export interface StoreFormat { name: string; file: string; custom: boolean; toml: string }
/** A file in the library. */
export interface FilePlace { asset_id: string; pack_id: string; pack_name: string; path: string }
/** Files of one name that may be a store's, which no format reads. */
export interface UnreadFile { name: string; packs: number; files: number; example: FilePlace }
/** Settings → Store files (`GET /api/admin/store-formats`). */
export interface StoreFormats { formats: StoreFormat[]; dir: string | null }
/** The files no format reads (`GET /api/admin/store-formats/unread`), as last worked out, and whether a newer list is
 *  being worked out. */
export interface UnreadFiles { unread: UnreadFile[]; refreshing: boolean }
/** Why a format didn't read a file: its `files` don't take the name, its `require` says it isn't the store's, or it
 *  finds no title. */
export type FormatMiss = 'name' | 'not_the_stores' | 'no_title'
/** What a format read in a store's file. */
export interface StoreFileRead {
  path: string
  id: string | null
  title: string
  format: string
  source: string
  seller: string | null
  license: string | null
  url: string | null
  asset_type: string | null
  tags: string[]
  origin: string | null
  dims: [string, string][]
  size: string | null
  texel_density: string | null
  maps: { file: string; kind: string }[]
  lods: { number: number; tris: number; files: string[] }[]
  physical_size: [number, number] | null
  displacement: number | null
}
/** `POST /api/admin/store-formats/try`: what a format makes of a file, and of the library's files like it. */
export interface FormatTrial {
  error?: string
  example?: { read: StoreFileRead | null; miss: FormatMiss | null; also: string | null } | null
  library?: { files: number; packs: number; tried: { place: FilePlace; title: string | null; miss: FormatMiss | null }[] }
}
export interface ExampleFile { path: string; text: string }

/** GET /api/admin/update: whether a newer release is out and what's keeping the server busy. */
/** Settings → About → Report a problem: the build, where it runs and what’s on, and the server’s last log lines. */
export interface ProblemReport {
  version: string
  environment: string
  facts: { name: string; value: string }[]
  log: string[]
}

export interface UpdateStatus {
  version: string
  update: { version: string; url: string } | null
  idle: boolean
  busy: { work: string[]; queued: number; uploads: number }
  hook: { set: boolean; last: UpdateHookCall | null }
}

export interface UpdateHookCall {
  version: string
  at: string
  ok: boolean
  /** The status that came back, or why nothing did. */
  result: string
}

/** Image search (SigLIP 2) or sound search (CLAP): its switch, its model, and how many pictures or sounds it has learned. */
export interface SearchModelSetting {
  value: boolean
  default: boolean
  saved: boolean
  state: 'off' | 'looking' | 'downloading' | 'ready' | 'unavailable'
  /** Why, when unavailable. */
  detail: string
  download_bytes: number
  /** Pictures or sounds with an embedding, of all that could have one. */
  done: number
  total: number
  /** Image search only: searches use what the model before learned until every picture is learned again. */
  relearning?: boolean
  /** Image search only: a hosted service doing the model's part, when `provider` isn't empty. The key is never sent. */
  hosted?: HostedSearch
}

export interface HostedSearch {
  provider: string
  model: string
  /** Empty for the service's own address. */
  url: string
  has_key: boolean
  providers: { id: string; name: string; model: string; url: string }[]
}

/** The S3 service `s3://bucket` locations are in. The secret key and the headers are never sent back. */
export interface S3Setting {
  /** Empty: AWS itself. */
  endpoint: string
  region: string
  access_key: string
  has_secret_key: boolean
  has_headers: boolean
  saved: boolean
  /** This copy of Hoard can reach S3 at all. */
  built: boolean
}

/** How thumbnails are stored: JPEG at three qualities, or PNG. Transparent ones and pixel art are always PNG. */
export type ThumbQuality = 'small' | 'balanced' | 'sharp' | 'lossless'
export type BackupEvery = 'off' | 'daily' | 'weekly'

/** Settings to change; `null` goes back to the server's default. */
export interface SettingsPatch {
  blender?: string | null
  unreal?: string | null
  unreal_image?: string | null
  ghcr_user?: string | null
  ghcr_token?: string | null
  ffmpeg?: string | null
  unreal_convert?: UnrealConvert | null
  llm_url?: string | null
  llm_model?: string | null
  llm_api_key?: string | null
  llm_vision?: boolean | null
  llm_chatgpt?: boolean | null
  llm_batch?: number | null
  workers?: number | null
  max_upload_mb?: number | null
  max_expand_mb?: number | null
  render?: boolean | null
  update_check?: boolean | null
  update_hook_url?: string | null
  update_hook_auth?: string | null
  guest_browsing?: boolean | null
  store_lookup?: boolean | null
  thumb_quality?: ThumbQuality | null
  thumb_avif?: boolean | null
  motion_thumbs?: boolean | null
  conversion_cache_mb?: number | null
  backup_every?: BackupEvery | null
  backup_keep?: number | null
  backup_copy_to?: string | null
  name?: string | null
  public_url?: string | null
  soundfont?: string | null
  clip?: boolean | null
  /** A hosted service for image search (voyage, cohere, jina), or empty for the model here; with its model, key and address. */
  clip_hosted?: string | null
  clip_hosted_model?: string | null
  clip_hosted_key?: string | null
  clip_hosted_url?: string | null
  clap?: boolean | null
  texture_names?: TextureNames | null
  conversion_rules?: ConversionRules | null
  s3_endpoint?: string | null
  s3_region?: string | null
  s3_access_key?: string | null
  s3_secret_key?: string | null
  /** One `Name=value` a line. */
  s3_headers?: string | null
}

/** Background work the server is doing (see `GET /api/activity`). */
export interface ActivityItem {
  id: number
  kind: string
  label: string
  detail?: string
  /** 0..1, when the work can tell. */
  progress?: number
  state: 'running' | 'queued'
  since: string
  pack_id?: string
}
