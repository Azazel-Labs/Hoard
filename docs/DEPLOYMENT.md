# Deploying Hoard

Hoard has two kinds of data with very different costs:

- **Metadata** (SQLite + FTS5): small (megabytes per hundred thousand assets), hot, read on every search.
- **Blobs** (the files): large, cold, immutable, read only on download.

So the natural split is metadata near the web server and blobs wherever storage is cheap: usually a NAS at home.
`HOARD_STORAGE=s3` is the seam: the server talks S3 to whatever holds the bytes, and streams downloads through
itself with Range support.

## Choosing the NAS storage server

MinIO's community edition is no longer maintained (the repository was archived in 2026), so use one of:

| | Good for | Notes |
|---|---|---|
| **Garage** | single node or a few nodes, low RAM | Rust, AGPL-3.0. v2.3+ bootstraps a single node with a bucket and key from env vars. Path-style URLs, region `garage`. Used in the compose files here. |
| **RustFS** | closest MinIO replacement, broader S3 coverage | Rust, Apache-2.0, younger project. |
| **SeaweedFS** | huge file counts, also exposes FUSE/WebDAV | more moving parts |
| **VersityGW** | an existing NAS share where files must stay plain files | S3 gateway over a POSIX directory, so NAS snapshots/backups see normal files |

Any of them works. Hoard only needs PUT, GET (with Range), HEAD and DELETE.

## Private access: everything on the NAS, reachable only through Cloudflare One (recommended)

`deploy/compose.private.yml`: Hoard + Garage + `cloudflared`, with **no public hostname**.

```
laptop (Cloudflare One client) ──> Cloudflare ──tunnel──> cloudflared ──LAN──> hoard :7878 ──> garage
```

Hoard never appears on the public internet. Your devices reach the NAS's LAN address from anywhere, as if they were
at home, and nothing else can. Because this is private network routing (raw TCP through the tunnel), not the HTTP
proxy that serves public hostnames, the 100 MB request-body cap on Cloudflare's Free and Pro plans shouldn't apply.
Cloudflare's docs don't say this explicitly, so try one upload over 100 MB once it's set up.

1. **Give the NAS a fixed LAN address** in an uncommon subnet if you can (e.g. `192.168.50.10` instead of
   `192.168.1.x`). Hotel and café networks are often `192.168.1.0/24` or `192.168.0.0/24`, and if your home subnet
   matches the network you're sitting on, your laptop will look for the NAS locally.
2. **Create a tunnel.** Zero Trust dashboard → Networks → Tunnels → create a `cloudflared` tunnel and copy its
   token into `deploy/.env` as `CLOUDFLARE_TUNNEL_TOKEN`. Skip the public hostname step.
3. **Route the NAS's address into the tunnel.** In the tunnel's private network (CIDR) routes, add the NAS's
   address as a `/32` (e.g. `192.168.50.10/32`). That's narrower and safer than routing your whole LAN.
4. **Fix Split Tunnels.** The client's default split-tunnel *exclude* list contains the private ranges
   (`192.168.0.0/16`, `10.0.0.0/8`, `172.16.0.0/12`), which would keep traffic to your NAS off the tunnel. In the
   device profile's Split Tunnels settings, remove the range containing your NAS, or exclude only the parts you
   don't use.
5. **Enroll your devices.** Allow your email under device enrollment, install the Cloudflare One client (formerly
   WARP), and log in to your team. Optionally add a Gateway network policy so only your identity can reach
   `192.168.50.10:7878`.
6. **Start it:** `docker compose -f deploy/compose.private.yml --env-file deploy/.env up -d`, then open
   `https://192.168.50.10:7878` from anywhere with the client connected. The compose file turns on a self-signed
   certificate (set `HOARD_TLS_NAMES` to the address you type), so the browser warns once; accept it and you get
   HTTP/2. For the `hoard` CLI, copy `tls/self-signed.crt` from Hoard's data folder and pass it with
   `--ca-cert` (or `HOARD_CA_CERT`). Point `hoard sync` (`server = ...`) and MCP
   clients at the same address, and set `HOARD_PUBLIC_URL` to it so MCP links resolve.

Keep `HOARD_TOKEN` set even here. The network is private, but the token keeps other devices on your LAN or
Zero Trust org read-only.

**HTTP/3.** `HOARD_HTTP3=true` adds QUIC on UDP 7878 and advertises it with `Alt-Svc`; browsers switch after the
first page load. Cloudflare's private routing carries UDP as well as TCP, but if HTTP/3 never kicks in over the
tunnel, HTTP/2 is doing the work and nothing is lost. Browsers only use HTTP/3 when they trust the certificate, so
with a self-signed one, expect HTTP/2 unless you install the certificate on the device.

**Tailscale instead?** Same model and equally private (`compose.nas-storage.yml` shows the sidecar pattern).
Tailscale usually connects your laptop *directly* to the NAS, while Cloudflare relays every byte through its edge,
so for multi-GB source files Tailscale is often faster. Use whichever you already run.

## Topology A: everything on the NAS, published on a public hostname via Cloudflare Tunnel

`deploy/compose.all-in-one.yml`: Hoard + Garage + `cloudflared`.

```
browser ──https──> Cloudflare edge ──tunnel──> cloudflared ─> hoard ─> garage   (all on the NAS)
```

1. Create a tunnel in Cloudflare Zero Trust, add a public hostname `hoard.example.com → http://hoard:7878`, and put
   the token in `deploy/.env`.
2. Protect the hostname with a Cloudflare Access application (email OTP or your IdP). For MCP clients and
   `hoard sync` from CI, add an Access **service token** and send its two headers.
3. Add a cache rule: cache everything for `/api/blobs/*`, `/api/assets/*/thumb` and `/api/assets/*/derivative/*`,
   respecting origin headers. Those responses are `immutable` and keyed by content hash, so the edge can keep them
   indefinitely and your home uplink only serves each file once per edge location.

Simple, but public: anyone can reach the login page, and uploads through a public hostname are capped at
100 MB per request on Free and Pro plans, which large source files will hit. Prefer "Private access" above
unless you need to share with people who can't install a client.

## Topology B: web on a VPS, blobs at home over Tailscale

`deploy/compose.nas-storage.yml` on the NAS, `deploy/compose.web.yml` on a small VPS.

```
browser ─> (Cloudflare) ─> VPS: hoard + SQLite ──tailscale──> NAS: garage
```

- The portal, search and MCP stay up and fast when the NAS sleeps; only downloads and ingest need it.
- Garage's S3 port is only reachable on the tailnet (`network_mode: service:tailscale`).
- Uploads land on the VPS and are pushed to the NAS. Size the VPS disk for the largest pack you'll upload at once
  (staging), not for the library.
- Back up the VPS's `/data/hoard.db` with `POST /api/admin/backup` (cron + `curl`), and the NAS bucket with the NAS's
  own snapshots.

Instead of Tailscale you can publish Garage through a Cloudflare Tunnel too (`s3.example.com → http://garage:3900`),
protected by Access, and give Hoard the service token:

```
HOARD_S3_ENDPOINT=https://s3.example.com
HOARD_S3_HEADERS=CF-Access-Client-Id=<id>.access,CF-Access-Client-Secret=<secret>
```

## Topology C: Cloudflare-native (plan, not built yet)

What you described: metadata in Cloudflare, bytes at home, Cloudflare serving the site. Here is how it would map,
and what has to change.

```
browser ─> Worker (portal + search API + MCP) ─> D1 (metadata, FTS5)
                │
                └─ downloads: Cache API ─miss─> Tunnel + Access ─> NAS: blob service
NAS: hoardd in "ingest node" mode ─ writes metadata to D1 (REST API), blobs to local Garage
```

- **D1 is SQLite and supports FTS5**, so the schema and most queries carry over unchanged.
- **Workers can't do ingest.** Decoding 8K textures, decoding audio for waveforms, rendering thumbnails and running
  Blender need real CPU, memory and a filesystem. Ingest stays on the NAS (or any box), as a `hoardd` mode that writes
  metadata to D1 through Cloudflare's REST API instead of local SQLite.
- **The Worker** serves the built portal as static assets, answers search/detail/MCP from D1, and proxies blob
  downloads to the NAS through the tunnel with a service token, caching them at the edge with the Cache API (they're
  immutable). Rust can target Workers via `workers-rs` (wasm32); the read-path handlers and `hoard-core` types port
  over, and the SQL moves to the D1 binding.
- **Engineering required:** put the DB layer behind a trait (SQLite today, D1-over-HTTP for the ingest node, D1
  binding in the Worker), split the read-path handlers into a crate that compiles for both native and wasm32, and
  add a Worker crate with `wrangler.toml`.

Topology B gets most of the benefit today (site up while the NAS sleeps, edge caching for downloads) with no new
code; C removes the VPS. Pick B now and move to C if the VPS starts to feel like overhead.

## Updating

Every compose file here runs the published image `ghcr.io/nickdarnell/hoard` and a
[Watchtower](https://github.com/nicholas-fedor/watchtower) container. (The original containrrr/watchtower was archived
in 2025; this fork is maintained and works the same way.) Watchtower checks for a new image at 04:00 every day,
replaces the Hoard container, and removes the old image. It only touches containers labelled with the `hoard` scope,
so other containers on the host are left alone.

| Setting (in `.env`) | Effect |
|---|---|
| `HOARD_VERSION=latest` (default) | every release |
| `HOARD_VERSION=0.8` | patch releases of 0.8 only; change it by hand for 0.9 |
| `HOARD_VERSION=0.8.1` | exactly that version: Watchtower never changes it |
| `HOARD_VERSION=edge` | every green commit on `main`, between releases |
| `HOARD_UPDATE_SCHEDULE=0 30 3 * * 0` | when Watchtower checks (cron with a leading seconds field) |
| `HOARD_UPDATE_CHECK=false` | stop the server from asking GitHub once a day whether a newer release exists |

To update by hand instead, delete the `watchtower` service and run `docker compose pull && docker compose up -d`.
If you build from a checkout, replace the `image:` line with `build: ..`.

**Database upgrades.** On start, Hoard compares the database's schema version with its own. If the database is
older, Hoard saves a copy (`<data dir>/backups/hoard-pre-upgrade-v<old>-<time>.db`, newest five kept) and then applies
each upgrade step in its own transaction, so a failed step leaves the database as it was. If the database is *newer*
than the server, because you rolled the image back, Hoard refuses to start rather than run against a schema it
doesn't know. To roll back, stop Hoard, copy the matching `hoard-pre-upgrade-*` file over `hoard.db` (anything added
since the upgrade is lost), and start the older image.

When a newer release exists, the portal header shows a green badge linking to its release notes. In `GET /api/stats`
it's the `update` field.

## GPU

Hoard uses a GPU for 3D model thumbnails (wgpu over Vulkan) when the container can see one. Otherwise it falls back
to Mesa's software renderer (lavapipe), which is already in the image, so a GPU is optional and only makes model
thumbnails faster. The packs page shows what Hoard found: `GPU: <adapter> (vulkan)`, `No GPU found: model thumbnails
render in software`, or no renderer at all. The same text is in `GET /api/stats` (`renderer`) and in the startup
log.

Layer the overlay for your hardware on any compose file that runs `hoard`:

**NVIDIA**: [compose.gpu-nvidia.yml](../deploy/compose.gpu-nvidia.yml). Install the NVIDIA driver and the
[NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
on the host, run `sudo nvidia-ctk runtime configure --runtime=docker`, and restart Docker. Then:

```sh
docker compose -f deploy/compose.yml -f deploy/compose.gpu-nvidia.yml up -d
```

The overlay asks for the `graphics` driver capability as well as `compute`. Without it the toolkit doesn't mount the
Vulkan driver, and Hoard falls back to software rendering even though `nvidia-smi` works.

**Intel or AMD**, including the integrated graphics in most NAS and mini-PC CPUs:
[compose.gpu-dri.yml](../deploy/compose.gpu-dri.yml). The host needs `/dev/dri/renderD128`. Hoard runs as an
unprivileged user, so the overlay adds it to the host's `render` group:

```sh
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> deploy/.env
docker compose -f deploy/compose.yml -f deploy/compose.gpu-dri.yml --env-file deploy/.env up -d
```

If the host has no `render` group, use the group that owns the device: `stat -c %g /dev/dri/renderD128`.

**The local LLM.** Both overlays also move the bundled tagger model (`--profile llm` in `compose.private.yml`) onto
the GPU. NVIDIA swaps in the llama.cpp `server-cuda` image and Intel or AMD the `server-vulkan` image, and both
offload every layer:

```sh
docker compose -f deploy/compose.private.yml -f deploy/compose.gpu-nvidia.yml --profile llm --env-file deploy/.env up -d
```

Docker Desktop on Windows passes NVIDIA GPUs through via WSL 2, so the NVIDIA overlay works there. On macOS Docker
can't reach the GPU, so run `hoardd` natively instead, where it uses Metal.

## Model conversion with Blender

Set `HOARD_BLENDER=/usr/bin/blender` (or build the image with `--build-arg WITH_BLENDER=1`). After each ingest, a
`convert` job exports .blend/.fbx/.dae/.usd*/.ply/.stl/.3ds/.abc/.x3d to GLB with the asset's linked textures laid out
beside it, stores the GLB as a derivative, and renders a thumbnail from it. Conversion runs with a 3-minute timeout per
file, and failures are logged without blocking anything. The originals are never modified.

## Unreal content

Packs of `.uasset` files are converted after ingest by default (`HOARD_UNREAL_CONVERT=keep`): textures become
PNG/EXR, sounds WAV, materials and data tables JSON, read by Hoard's own code with nothing extra installed. The
original packages stay downloadable; set `HOARD_UNREAL_CONVERT=replace` to discard them once converted and save the
space, or `off` to convert only when asked from the pack page.

Meshes, animations and textures imported from uncompressed formats (which UE5 stores Oodle-compressed) wait for
Unreal Editor. Run `hoard unreal-worker --editor <UnrealEditor-Cmd>` on a
workstation with Unreal installed and network access to the server (with `HOARD_TOKEN` set if writes need one); it
can run as a service and polls every minute. The server never needs Unreal.
