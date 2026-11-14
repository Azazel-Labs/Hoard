# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · Bahasa Indonesia · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Pustaka aset self-hosted untuk pengembangan game. Masukkan paket aset utuh (zip, tarball, folder), dan Hoard
menyimpan setiap berkas sekali berdasarkan hash-nya, mengenali apa setiap berkas itu, memberinya tag, menautkan mesh
ke tekstur dan animasinya, dan membuat semuanya dapat dicari dari portal web, CLI, endpoint MCP untuk agen, dan
`hoard sync` berbasis lockfile yang menarik tepat berkas yang dibutuhkan proyek game Anda.

Anggap saja toko aset tanpa kasir.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Pustaka Hoard: model dan material dalam kisi, dengan peti harta terbuka di pratinjau 3D di samping tag dan unduhannya" src="../img/hero-light.webp">
</picture>

<sub>Pustaka demo berisi karya CC0: model dari [Poly Haven](https://polyhaven.com) (glTF dan USD) dan permukaan dari
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` membuat gambar-gambar ini dan kartu
media sosialnya.</sub>

## Memasang dengan Docker

Hoard dirilis sebagai satu image untuk amd64 dan arm64: `ghcr.io/azazel-labs/hoard`. Simpan ini sebagai `compose.yml`
(isinya sama dengan [deploy/compose.yml](../../deploy/compose.yml)):

```yaml
services:
  hoard:
    container_name: hoard
    image: ghcr.io/azazel-labs/hoard:latest
    restart: unless-stopped
    volumes:
      - hoard-config:/config      # database, Settings, backups: small, back it up
      - hoard-library:/library    # your files: big, put it on the disk with room
      - hoard-cache:/cache        # thumbnails, uploads in progress, Blender/ffmpeg: fast disk, can be rebuilt
    ports:
      - "4627:4627"

volumes:
  hoard-config:
  hoard-library:
  hoard-cache:
```

Setiap volume bisa juga berupa folder di disk mana pun yang cocok: `/mnt/big/hoard:/library` menaruh berkas Anda di
disk besar, sementara database tetap kecil dan mudah dicadangkan. Folder yang Anda mount tidak perlu chown: Hoard
mengambil alih `/config` dan `/cache` dan berjalan sebagai pemilik `/library` (atur `PUID` dan `PGID` untuk memilih
pengguna sendiri).

Lalu jalankan:

```sh
docker compose up -d        # open http://<host>:4627
```

Kunjungan pertama meminta Anda membuat akun, lalu membuka penyiapan singkat yang menanyakan di mana pustaka Anda
disimpan. Penyiapan itu menyarankan `/library`, volume di atas; pilih itu dan jatuhkan sebuah paket ke portal. Lokasi
lain bisa ditambahkan nanti, dari **Pengaturan → Lokasi pustaka**:

- **Pustaka yang disimpan Hoard** mendapat salinan dari apa yang Anda unggah atau impor. Masing-masing mencentang
  jenis berkas yang diterimanya (audio di satu disk, model di disk lain), dan satu menjadi lokasi penampung untuk
  sisanya. Mengubah apa yang diterima sebuah lokasi berlaku untuk berkas baru; **Pindahkan berkas tersimpan agar
  sesuai** memindahkan apa yang sudah tersimpan. **Hapus** memindahkan berkas sebuah lokasi ke lokasi lain lebih
  dulu, menurut jenisnya, dan menghapus lokasi itu setelah kosong. Setiap berkas diperiksa terhadap hash-nya sebelum
  salinan lama dihapus. Lokasi yang disknya sudah tidak ada tetap dapat dihapus, dan berkasnya lalu tampil hilang.
- **Folder pantauan** adalah folder berisi paket yang sudah Anda miliki. Setiap folder di dalamnya menjadi sebuah
  paket, diindeks di tempatnya. Hoard tidak pernah mengubah, memindahkan, atau menghapus apa pun di sana. Mount saja
  sebagai read-only jika mau. Hoard memeriksanya saat mulai dan setiap 15 menit setelahnya (atau saat Anda menekan
  **Pindai sekarang**), menangkap folder baru, berkas yang ditambahkan ke atau dihapus dari sebuah paket, dan paket
  yang foldernya sudah tidak ada. Folder yang tiba-tiba kosong atau tidak terjangkau dianggap sebagai disk yang
  tidak ter-mount, dan paketnya tetap disimpan. Paket yang foldernya hilang disimpan selama tiga hari, beserta tag
  dan catatannya, kalau-kalau foldernya kembali. Apa pun yang Anda hapus permanen dari paket pantauan tetap terhapus
  saat folder dipindai ulang, meskipun berkasnya masih ada.

Lokasi adalah folder apa pun yang dapat dilihat server, jadi di Docker, mount dulu folder itu ke dalam container.

**Memperbarui.** Bilah atas menampilkan lencana saat ada rilis yang lebih baru. Jalankan
`docker compose pull && docker compose up -d` untuk mengambilnya; Hoard meningkatkan database-nya saat mulai, dengan
menyimpan salinan di `/config/backups` lebih dulu. Ubah tag image ke `:0.8` untuk hanya mendapat rilis patch. Untuk
memperbarui otomatis tanpa memulai ulang Hoard di tengah unggahan, berikan alamat untuk dipanggil di
**Pengaturan → Pemeliharaan → Pembaruan**: Hoard memanggilnya begitu rilis keluar dan tidak ada yang sedang berjalan
(HTTP API Watchtower, webhook Portainer, skrip; lihat [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opsional).** Hoard merender gambar mini model 3D di GPU jika dapat melihatnya, dan dengan software jika tidak.
Halaman paket menunjukkan yang mana (`GPU: ...` atau `No GPU found`). Untuk meneruskan GPU, tambahkan overlay untuk
perangkat keras Anda:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Ambil kedua overlay dari [deploy/](../../deploy/). Keduanya juga menaruh server model lokal (penandaan AI) di GPU.
Berkas di NAS, penyimpanan S3, akses jarak jauh melalui Cloudflare atau Tailscale, dan pencadangan dibahas di
[docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Membangun dari sumber

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Atau jatuhkan zip-nya ke portal.

## Sisa README ini

Apa yang terjadi pada sebuah paket, memasukkan aset ke engine, paket Unreal, pencarian, kredit, MCP, penyimpanan,
HTTPS, konfigurasi, keamanan, akun, dan API dijelaskan di [README bahasa Inggris](../../README.md).

## Terjemahan

Pesan portal dan server Hoard tersedia dalam 31 bahasa, dipilih di Pengaturan → Tampilan (secara bawaan mengikuti
bahasa browser). Terjemahan pertama dibuat dengan bantuan mesin; koreksi dari penutur asli sangat diterima.
[docs/translations.md](../translations.md) menjelaskan caranya.

## Lisensi

[MIT No Attribution](../../LICENSE) (MIT-0): pakai, salin, ubah, dan distribusikan ulang Hoard untuk tujuan apa pun,
tanpa perlu mempertahankan pemberitahuan hak cipta. Paket aset yang Anda simpan di Hoard tetap memakai lisensinya
masing-masing.
