# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · Bahasa Indonesia · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Setiap paket aset yang Anda miliki, dapat dicari, dapat dipratinjau, dan hanya sekali klik dari game Anda.</h3>

Hoard adalah pustaka self-hosted untuk aset game. Masukkan zip, paket Unity, unduhan Fab, atau folder berisi sepuluh
ribu berkas, dan Hoard langsung bekerja. Model berputar dalam 3D dengan teksturnya, map tekstur yang lepas menjadi
material, suara menggambar bentuk gelombangnya, lembar sprite memutar animasinya, dan semuanya diberi tag, sehingga
mengetik `footsteps gravel` atau `rigged low-poly` langsung menemukannya. Lalu aset itu masuk ke Godot, Unity, atau
Unreal beserta semua yang dibutuhkannya.

Tanpa langganan, tanpa unggahan ke cloud siapa pun, tanpa harga per pengguna: satu image Docker di server Anda sendiri,
atau sebuah aplikasi di komputer Anda sendiri, dan itu milik Anda. Anggap saja toko aset tanpa kasir.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Pustaka Hoard: model 3D dalam kisi, dengan peti harta terbuka di pratinjau 3D di samping tag dan unduhannya" src="../img/hero-light.webp">
</picture>

### Lihat semuanya sebelum Anda memakainya

Model terbuka dalam 3D dengan tekstur, material, dan animasinya. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL, dan voxel
dibaca oleh Hoard sendiri, tanpa aplikasi 3D yang terpasang; berkas Blender masuk melalui Blender headless (tanpa
antarmuka), dan paket Unreal menampilkan apa yang disimpan editor tentangnya, tanpa Unreal. Thumbnail digambar di
server, dibingkai dengan cara yang sama setiap kali, sehingga seluruh paket bisa ditelusuri sekilas.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Sebuah lampu gantung ditampilkan satu halaman penuh: tampilan 3D besar di samping fakta, tag, tekstur tertaut, dan unduhannya" src="../img/page-light.webp">
</picture>

### Map tekstur yang lepas menjadi material

`Rock_BaseColor.png`, `Rock_Normal.png`, dan `Rock_Roughness.png` menjadi satu material Rock pada bola yang diterangi.
Hoard memahami penamaan Megascans, Poly Haven, ambientCG, Substance, dan Unreal, termasuk map yang dikemas, serta
membaca berkas `.mat` Unity juga. Penampil layar penuh menampilkan material pada bola, kubus, atau bidang, dalam skala
dunia nyatanya, dengan setiap map bisa dinyalakan dan dimatikan.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Material bata di penampil layar penuh, pada bola, dengan sakelar untuk map warna, normal, kekasaran, dan ketinggiannya" src="../img/material-light.webp">
</picture>

### Lembar sprite yang bisa diputar

Atlas dari TexturePacker, Starling, libGDX, Cocos2d, dan Godot menjadi satu lembar sprite yang memutar animasinya dan
mencantumkan sprite-nya. Unduh dalam bentuk dikemas ulang untuk engine apa pun yang Anda pakai: sepuluh format,
masing-masing hanya diberi apa yang bisa ditampungnya.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Lembar sprite karakter ditampilkan satu halaman penuh, salah satu animasinya diputar di samping tag dan format unduhannya" src="../img/sheet-light.webp">
</picture>

### Suara yang bisa Anda lihat

Setiap suara mendapat bentuk gelombang yang diwarnai menurut seberapa cerah bunyinya, sehingga pukulan tajam dan
ambience panjang terlihat berbeda sebelum Anda memutarnya. Metadata pustaka suara dibaca (kategori UCS, kolom Soundminer
dan BaseHead, titik loop, tempo, nada dasar), dan musik MIDI serta tracker dirender agar bisa diputar seperti suara
lainnya.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Kisi efek suara yang digambar sebagai bentuk gelombang berwarna, dengan satu yang terbuka menampilkan pemutar, tag, dan take-nya" src="../img/sound-light.webp">
</picture>

### Pencarian yang tahu apa itu apa

Hoard memberi tag pada semuanya sendiri, secara offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0`, dan sebuah kategori dari pohon aset game. Cari dengan kata, dengan warna, atau dengan apa yang
ditampilkan sebuah gambar, dan minta “Lainnya yang mirip ini” untuk tekstur atau suara apa pun. Arahkan ke model apa pun
yang kompatibel dengan OpenAI, lokal atau dihosting, dan Hoard juga menuliskan nama dan deskripsi.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Pencarian wood: meja kayu, kulit kayu, papan, dan permukaan kayu bersama-sama, dengan filter jenis dan tag di sampingnya" src="../img/search-light.webp">
</picture>

### Langsung ke engine Anda

- **Plugin engine** untuk Godot 4, Unity, Unreal Engine 5, dan Lumina: jelajahi pustaka di dalam editor lalu tekan Tambahkan ke
  proyek. Model datang dengan teksturnya, di tempat yang tepat, sudah diimpor.
- **`hoard sync`** menarik tepat berkas yang disebutkan sebuah proyek ke dalam proyek itu, dikunci dalam lockfile,
  sehingga setiap rekan tim dan mesin build mendapat byte yang sama.
- **Unduh sebagai** GLB, PNG, WAV, MP3, atau format lembar sprite, dikonversi seketika, dengan nama tekstur mengikuti
  konvensi studio Anda.
- **Agen AI** mencari, memeriksa, dan mengambil aset melalui MCP.

### Disimpan sekali, dijaga tetap aman

Setiap berkas disimpan sekali berdasarkan hash BLAKE3-nya, sehingga tekstur yang sama di lima bundel hanya memakan ruang
satu tekstur, dan kerusakan data tidak mungkin luput dari perhatian. Simpan di disk lokal atau bucket S3 mana pun.
Folder yang Anda tunjukkan ke Hoard hanya dibaca, tidak pernah diubah. Akun, token hanya-baca, single sign-on, HTTPS,
dan HTTP/3 sudah tersedia bawaan.

### Atau cukup di komputer Anda

Tidak menjalankan server? Aplikasi desktop untuk Windows, macOS, dan Linux adalah pustaka yang sama dalam jendelanya
sendiri. Pasang, pilih sebuah folder, dan aplikasi itu bekerja secara offline: tanpa akun, kata sandi, atau pengaturan
server, tidak ada yang bisa dijangkau dari bagian lain jaringan, dan kunci yang Anda ketikkan disimpan di keychain
sistem. Versi baru diunduh di latar belakang dan dipasang saat aplikasi ditutup.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Aplikasi desktop Hoard di jendela Windows: model-model pustaka dalam kisi, dengan lentera yang dibuka di pratinjau 3D di samping tag dan unduhannya" src="../img/desktop-light.webp">
</picture>

<sub>Pustaka demo berisi karya CC0: model dari [Poly Haven](https://polyhaven.com) (glTF dan USD), permukaan dari
[ambientCG](https://ambientcg.com), serta suara, lembar sprite, dan font dari [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` membuat gambar-gambar ini dan kartu media sosialnya.</sub>

## Memasang aplikasi desktop

Unduh dari [rilis terbaru](https://github.com/Azazel-Labs/Hoard/releases/latest): `-setup.exe` untuk Windows, `.dmg`
untuk macOS di Apple silicon, `.AppImage` atau `.deb` untuk Linux. Di Windows, aplikasi ini terpasang hanya untuk Anda,
tanpa meminta akses administrator. Penginstalnya belum ditandatangani oleh Apple atau Microsoft, jadi Windows akan
bertanya sebelum menjalankan penginstal (More info → Run anyway) dan macOS sebelum membuka aplikasinya untuk pertama
kali (System Settings → Privacy & Security → Open Anyway).

Saat pertama kali dibuka, aplikasi akan menanyakan di mana pustaka disimpan. Semua hal lainnya ada di Pengaturan, sama
seperti di server.

Setelah itu, aplikasi akan memperbarui dirinya sendiri tanpa perlu Anda mengeklik apa pun: rilis baru diunduh di latar
belakang dan dipasang saat Anda keluar, jadi saat berikutnya Anda membuka Hoard, versinya sudah yang baru. `.deb`
diperbarui dengan memasang versi berikutnya.

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
menyimpan salinan di `/config/backups` lebih dulu. Ubah tag image ke `:1.2` untuk hanya mendapat rilis patch. Untuk
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
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
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
HTTPS, konfigurasi, keamanan, akun, API, serta fakta dan angka dijelaskan di [README bahasa Inggris](../../README.md).

## Terjemahan

Pesan portal dan server Hoard tersedia dalam 31 bahasa, dipilih di Pengaturan → Tampilan (secara bawaan mengikuti
bahasa browser). Terjemahan pertama dibuat dengan bantuan mesin; koreksi dari penutur asli sangat diterima.
[docs/translations.md](../translations.md) menjelaskan caranya.

## Lisensi

[MIT No Attribution](../../LICENSE) (MIT-0): pakai, salin, ubah, dan distribusikan ulang Hoard untuk tujuan apa pun,
tanpa perlu mempertahankan pemberitahuan hak cipta. Paket aset yang Anda simpan di Hoard tetap memakai lisensinya
masing-masing.
