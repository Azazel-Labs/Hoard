# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · 繁體中文

一套自行架設的遊戲開發素材庫。直接放入整個素材包（zip、tarball、資料夾），Hoard 就會依雜湊值將每個檔案只儲存一次，判斷每個檔案是什麼、為它加上標籤、將網格連結到它們的貼圖與動畫，並讓這一切都能透過網頁入口、CLI、供代理程式使用的 MCP 端點，以及以鎖定檔為基礎、只拉取你的遊戲專案所需檔案的
`hoard sync` 進行搜尋。

你可以把它想成一個不用結帳的素材商店。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard 素材庫：格狀排列的模型與材質，旁邊的 3D 預覽中開啟著一個寶箱，並列出它的標籤與下載選項" src="../img/hero-light.webp">
</picture>

<sub>示範素材庫中的都是 CC0 作品：模型來自 [Poly Haven](https://polyhaven.com)（glTF 與 USD），表面材質來自
[ambientCG](https://ambientcg.com)。這些圖片與社群預覽卡是由 `node web/scripts/readme-images.mts` 產生的。</sub>

## 使用 Docker 安裝

Hoard 以單一映像檔發布，支援 amd64 與 arm64：`ghcr.io/azazel-labs/hoard`。將以下內容儲存為 `compose.yml`（也就是
[deploy/compose.yml](../../deploy/compose.yml)）：

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

每個磁碟區也可以改成放在最適合它的磁碟上的資料夾：`/mnt/big/hoard:/library` 會把你的檔案放在大容量磁碟上，而資料庫則維持小巧、容易備份。你掛載的資料夾不需要 chown：Hoard 會接管
`/config` 與 `/cache`，並以 `/library` 的擁有者身分執行（設定 `PUID` 與 `PGID` 可自行指定使用者）。

接著啟動它：

```sh
docker compose up -d        # open http://<host>:4627
```

第一次造訪時會請你建立帳號，然後開啟簡短的設定流程，詢問你的素材庫要放在哪裡。它會建議 `/library`，也就是上面的磁碟區；選擇它，然後把素材包拖放到入口網頁上即可。之後可以在 **設定 → 素材庫位置** 中新增更多位置：

- **由 Hoard 保管的素材庫** 會存放你上傳或匯入的檔案副本。每個素材庫會勾選它接收的檔案種類（例如音訊放在一個磁碟、模型放在另一個），其中一個是接收其餘所有檔案的位置。變更位置接收的種類只會套用到新檔案；**移動已儲存的檔案以符合設定** 則會移動已經儲存的檔案。**移除** 會先依種類將該位置的檔案移到其他位置，等它清空後再將它移除。每個檔案在刪除舊副本之前，都會先以雜湊值檢查。磁碟已經不在的位置仍然可以移除，之後它的檔案會顯示為遺失。
- **監看資料夾** 是你已經擁有的素材包資料夾。其中的每個資料夾都會成為一個素材包，並在原位置建立索引。Hoard 絕不會變更、移動或刪除其中的任何內容。你也可以用唯讀方式掛載它。Hoard 會在啟動時以及之後每 15 分鐘（或在你按下 **立即掃描** 時）檢查它，找出新的資料夾、加入素材包或從素材包移除的檔案，以及資料夾已經不在的素材包。突然變空或無法存取的資料夾會被視為尚未掛載的磁碟，其中的素材包會保留。資料夾遺失的素材包會連同標籤與筆記保留三天，以防資料夾再次出現。你從監看素材包中永久刪除的任何內容，在重新掃描資料夾時仍會維持刪除狀態，即使它的檔案仍在原處。

位置可以是伺服器能看到的任何資料夾，因此在 Docker 中，請先將它掛載到容器中。

**更新。** 有較新的版本時，頂端列會顯示標記。執行 `docker compose pull && docker compose up -d`
即可更新；Hoard 會在啟動時升級資料庫，並先在 `/config/backups` 中儲存一份副本。將映像檔標籤改為
`:1.2` 就只會取得修補版本。若要自動更新，又不想讓 Hoard 在上傳途中重新啟動，請在
**設定 → 維護 → 更新** 中提供一個要呼叫的網址：Hoard 會在新版本推出且沒有任何工作進行時呼叫它（Watchtower 的 HTTP API、Portainer webhook、指令碼；請參閱 [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)）。

**GPU（選用）。** Hoard 能看到 GPU 時會用 GPU 算繪 3D 模型縮圖，看不到時則使用軟體算繪。素材包頁面會顯示使用的是哪一種（`GPU: ...` 或 `No GPU found`）。若要將 GPU 傳遞給容器，請加上適合你硬體的覆疊檔：

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

兩個覆疊檔都可以從 [deploy/](../../deploy/) 取得。它們也會讓本機模型伺服器（AI 標記）使用 GPU。將檔案放在 NAS 或 S3 儲存空間、透過 Cloudflare
或 Tailscale 遠端存取，以及備份等內容，都在 [docs/DEPLOYMENT.md](../DEPLOYMENT.md) 中說明。

## 從原始碼建置

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

或者直接將 zip 拖放到入口網頁上。

## 本 README 的其餘內容

素材包會經過哪些處理、如何將素材匯入引擎、Unreal 素材包、搜尋、致謝名單、MCP、儲存空間、HTTPS、組態設定、安全性、帳號與 API 等內容，都在[英文版 README](../../README.md) 中說明。

## 翻譯

Hoard 的入口網頁與伺服器訊息提供 31 種語言，可在「設定 → 外觀」中選擇（預設使用瀏覽器的語言）。最初的翻譯由機器輔助完成；歡迎母語人士提供修正。[docs/translations.md](../translations.md) 說明了如何參與。

## 授權

[MIT No Attribution](../../LICENSE)（MIT-0）：你可以為任何目的使用、複製、修改與重新散布 Hoard，而且不需要保留著作權聲明。你存放在 Hoard 中的素材包仍各自保有其授權。
