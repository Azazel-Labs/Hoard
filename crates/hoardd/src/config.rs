use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum StorageKind {
    Local,
    S3,
}

/// What to do with Unreal packages after ingest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum UnrealConvert {
    /// Leave .uasset files as they are (convert from the portal or API on demand).
    Off,
    /// Convert what can be converted to common formats, and keep the original package alongside.
    Keep,
    /// Convert, then discard the original package once its content is out.
    Replace,
}

/// Hoard asset server. Every flag can also be set with the matching `HOARD_*` environment variable.
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
#[expect(clippy::struct_excessive_bools, reason = "each bool is an independent command-line switch")]
pub(crate) struct Config {
    /// Address to listen on.
    #[arg(long, env = "HOARD_LISTEN", default_value = "127.0.0.1:7878")]
    pub(crate) listen: SocketAddr,

    /// Where blobs, the database, staging and backups live.
    #[arg(long, env = "HOARD_DATA_DIR", default_value = "./hoard-data")]
    pub(crate) data_dir: PathBuf,

    /// Built web portal to serve at `/` (the `web/dist` folder). Skipped if it doesn't exist.
    #[arg(long, env = "HOARD_WEB_DIR", default_value = "./web/dist")]
    pub(crate) web_dir: PathBuf,

    /// TLS certificate chain (PEM). With `--tls-key`, serves HTTPS with HTTP/2. `tailscale cert <name>` or any ACME
    /// client gives you one browsers trust.
    #[arg(long, env = "HOARD_TLS_CERT")]
    pub(crate) tls_cert: Option<PathBuf>,

    /// TLS private key (PEM).
    #[arg(long, env = "HOARD_TLS_KEY")]
    pub(crate) tls_key: Option<PathBuf>,

    /// Serve HTTPS with a self-signed certificate generated on first start (kept in `--data-dir/tls`). Browsers will
    /// warn once; accept it and you get HTTP/2.
    #[arg(long, env = "HOARD_TLS_SELF_SIGNED", default_value_t = false, action = clap::ArgAction::Set)]
    pub(crate) tls_self_signed: bool,

    /// Extra host names/IPs for the self-signed certificate, comma-separated (e.g. `192.168.50.10,nas.local`).
    #[arg(long, env = "HOARD_TLS_NAMES", value_delimiter = ',')]
    pub(crate) tls_names: Vec<String>,

    /// Also serve HTTP/3 (QUIC, UDP on the same port) and advertise it with `Alt-Svc`. Needs TLS. Experimental.
    #[arg(long, env = "HOARD_HTTP3", default_value_t = false, action = clap::ArgAction::Set)]
    pub(crate) http3: bool,

    /// If set, uploads, edits and admin calls require `Authorization: Bearer <token>`. Reads stay open.
    #[arg(long, env = "HOARD_TOKEN", hide_env_values = true)]
    pub(crate) token: Option<String>,

    /// Check GitHub once a day for a newer Hoard release and say so in the portal. Sends nothing but the request
    /// (with a `hoard/<version>` user agent); `false` turns it off.
    #[arg(long, env = "HOARD_UPDATE_CHECK", default_value_t = true, action = clap::ArgAction::Set)]
    pub(crate) update_check: bool,

    /// Max size of one upload request, in MiB.
    #[arg(long, env = "HOARD_MAX_UPLOAD_MB", default_value_t = 16 * 1024)]
    pub(crate) max_upload_mb: u64,

    /// Max bytes an archive may expand to, in MiB (zip-bomb guard).
    #[arg(long, env = "HOARD_MAX_EXPAND_MB", default_value_t = 64 * 1024)]
    pub(crate) max_expand_mb: u64,

    /// Max number of files in one pack after unpacking.
    #[arg(long, env = "HOARD_MAX_FILES", default_value_t = 200_000)]
    pub(crate) max_files: usize,

    /// Render thumbnails for glTF/GLB/OBJ models with the built-in wgpu renderer. Uses a GPU when there is
    /// one, otherwise Mesa's lavapipe (software Vulkan, `mesa-vulkan-drivers` on Debian/Ubuntu).
    #[arg(long, env = "HOARD_RENDER", default_value_t = true, action = clap::ArgAction::Set)]
    pub(crate) render: bool,

    /// Blender, for converting .blend/.fbx/.dae/.usd/.ply/.stl/.3ds models to .glb in the background (stored next to
    /// the original), which gives them thumbnails and a browser preview: `auto` uses an installed Blender, otherwise
    /// downloads Blender 4.5 LTS into `<data dir>/tools` (Linux and Windows x64); a path uses that binary; `off`
    /// never runs it. Can be changed in the portal's Settings, which wins over this.
    #[arg(long, env = "HOARD_BLENDER", default_value = "off")]
    pub(crate) blender: String,

    /// Convert Unreal packages to common formats after ingest: textures to PNG/EXR, sounds to WAV, materials to
    /// JSON, all with Hoard's own reader. Meshes and animations wait for `hoard unreal-worker` (Unreal Editor).
    #[arg(long, env = "HOARD_UNREAL_CONVERT", value_enum, default_value_t = UnrealConvert::Keep)]
    pub(crate) unreal_convert: UnrealConvert,

    /// General MIDI SoundFont (`.sf2`) for rendering MIDI previews and downloads. Without one, common system paths
    /// are tried (`/usr/share/sounds/sf2/…`), then a simple built-in synthesizer is used.
    #[arg(long, env = "HOARD_SOUNDFONT")]
    pub(crate) soundfont: Option<PathBuf>,

    /// ffmpeg, for MP3/OGG/Opus encoding and for reading audio Hoard can't decode itself (AAC/M4A, Opus, WMA, …):
    /// `auto` uses a system ffmpeg that has LAME, libopus and libvorbis, otherwise downloads a static build into
    /// `<data dir>/tools` in the background; a path uses that binary; `off` never runs ffmpeg. Can be changed in the
    /// portal's Settings.
    #[arg(long, env = "HOARD_FFMPEG", default_value = "auto")]
    pub(crate) ffmpeg: String,

    /// Public base URL of this server (e.g. `https://hoard.example.com`), used for absolute links handed to
    /// MCP clients. Defaults to the request's Host header.
    #[arg(long, env = "HOARD_PUBLIC_URL")]
    pub(crate) public_url: Option<String>,

    /// Background job workers.
    #[arg(long, env = "HOARD_WORKERS", default_value_t = 2)]
    pub(crate) workers: usize,

    /// Where file bytes live: `local` (under `--data-dir/blobs`) or `s3` (any S3-compatible bucket,
    /// e.g. Garage or `RustFS` on a home NAS). Metadata always stays in this server's SQLite database.
    #[arg(long, env = "HOARD_STORAGE", value_enum, default_value_t = StorageKind::Local)]
    pub(crate) storage: StorageKind,

    /// S3 endpoint, e.g. `http://nas.tailnet-name.ts.net:3900`. Omit for AWS itself.
    #[arg(long, env = "HOARD_S3_ENDPOINT")]
    pub(crate) s3_endpoint: Option<String>,

    #[arg(long, env = "HOARD_S3_BUCKET")]
    pub(crate) s3_bucket: Option<String>,

    /// Garage uses `garage` by default; most others accept anything.
    #[arg(long, env = "HOARD_S3_REGION", default_value = "us-east-1")]
    pub(crate) s3_region: String,

    #[arg(long, env = "HOARD_S3_ACCESS_KEY", hide_env_values = true)]
    pub(crate) s3_access_key: Option<String>,

    #[arg(long, env = "HOARD_S3_SECRET_KEY", hide_env_values = true)]
    pub(crate) s3_secret_key: Option<String>,

    /// Extra headers on every storage request, `Name=value`, comma-separated in the env var. Use this when the
    /// NAS's S3 endpoint sits behind Cloudflare Access:
    /// `HOARD_S3_HEADERS="CF-Access-Client-Id=xxx.access,CF-Access-Client-Secret=yyy"`.
    #[arg(long = "s3-header", env = "HOARD_S3_HEADERS", value_delimiter = ',', hide_env_values = true)]
    pub(crate) s3_headers: Vec<String>,

    /// OpenAI-compatible base URL for the LLM tagger, e.g. `https://api.openai.com/v1` or
    /// `http://localhost:11434/v1` (Ollama). Leave unset (or empty) to run fully offline with the rule tagger only.
    /// Can be changed in the portal's Settings.
    #[arg(long, env = "HOARD_LLM_URL")]
    pub(crate) llm_url: Option<String>,

    /// Model name for the LLM tagger, as the endpoint knows it (needed with `--llm-url`; without it the tagger
    /// stays off).
    #[arg(long, env = "HOARD_LLM_MODEL")]
    pub(crate) llm_model: Option<String>,

    #[arg(long, env = "HOARD_LLM_API_KEY", hide_env_values = true)]
    pub(crate) llm_api_key: Option<String>,

    /// Send texture thumbnails to the LLM (needs a vision-capable model).
    #[arg(long, env = "HOARD_LLM_VISION", default_value_t = false)]
    pub(crate) llm_vision: bool,

    /// Assets per LLM request.
    #[arg(long, env = "HOARD_LLM_BATCH", default_value_t = 40)]
    pub(crate) llm_batch: usize,
}

impl Config {
    pub(crate) fn blobs_dir(&self) -> PathBuf {
        self.data_dir.join("blobs")
    }
    pub(crate) fn staging_dir(&self) -> PathBuf {
        self.data_dir.join("staging")
    }
    pub(crate) fn tmp_dir(&self) -> PathBuf {
        self.data_dir.join("tmp")
    }
    pub(crate) fn tools_dir(&self) -> PathBuf {
        self.data_dir.join("tools")
    }
    pub(crate) fn backups_dir(&self) -> PathBuf {
        self.data_dir.join("backups")
    }
    pub(crate) fn db_path(&self) -> PathBuf {
        self.data_dir.join("hoard.db")
    }
}
