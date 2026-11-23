<div align="center">

# <img src="../../web/public/favicon.svg" alt="" height="32" align="absmiddle"> Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· 繁&#8288;體&#8288;中&#8288;文

</div>

<h3 align="center">你擁有的每一個素材包，都能搜尋、預覽，而且離你的遊戲只差一次點擊。</h3>

Hoard 是一套自行架設的遊戲素材庫。放入一個 zip、一個 Unity 套件、一份 Fab 下載內容，或一個裝著上萬個檔案的資料夾，Hoard 就會開始工作。模型帶著貼圖在 3D 中旋轉，零散的貼圖檔會組成材質，聲音會畫出自己的波形，精靈圖表會播放自己的動畫，而一切都會加上標籤，所以輸入 `footsteps gravel` 或 `rigged low-poly` 就能找到。接著它就會帶著所需的一切進入 Godot、Unity 或 Unreal。

不用訂閱，不必上傳到任何人的雲端，也不按席次計價：在你自己的伺服器上跑一個 Docker 映像檔，或在你自己的電腦上裝一個應用程式，它就完全屬於你。你可以把它想成一個不用結帳的素材商店。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard 素材庫：格狀排列的 3D 模型，旁邊的 3D 預覽中開啟著一個寶箱，並列出它的標籤與下載選項" src="../img/hero-light.webp">
</picture>

### 使用之前，先看個清楚

模型會連同貼圖、材質與動畫一起在 3D 中開啟。FBX、glTF、OBJ、USD、SketchUp、3MF、STL 與體素由 Hoard 自行讀取，不需安裝任何 3D 應用程式；Blender 檔案透過無介面（headless）的 Blender 匯入，Unreal 套件則不需要 Unreal，就能顯示編輯器為它們儲存的資訊。縮圖在伺服器上繪製，每次都以相同方式取景，整個素材包一眼就能瀏覽完畢。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="以整頁顯示的吊燈：大型 3D 檢視旁列出它的資訊、標籤、連結的貼圖與下載選項" src="../img/page-light.webp">
</picture>

### 零散的貼圖變成材質

`Rock_BaseColor.png`、`Rock_Normal.png` 與 `Rock_Roughness.png` 會合成一個 Rock 材質，呈現在受光照的球體上。Hoard 看得懂 Megascans、Poly Haven、ambientCG、Substance 與 Unreal 的命名方式（包括打包貼圖），也能讀取 Unity 的 `.mat` 檔案。全螢幕檢視器可將材質呈現在球體、立方體或平面上，以真實世界的比例顯示，每張貼圖都能個別開關。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="全螢幕檢視器中的磚牆材質，呈現在球體上，並附有顏色、法線、粗糙度與高度貼圖的開關" src="../img/material-light.webp">
</picture>

### 會動的精靈圖表

來自 TexturePacker、Starling、libGDX、Cocos2d 與 Godot 的圖集會變成一份精靈圖表，能播放其中的動畫並列出其中的精靈。你可以下載專為所用引擎重新打包的版本：共十種格式，每種格式只會寫入它能容納的內容。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="以整頁顯示的角色精靈圖表，其中一段動畫正在播放，旁邊是它的標籤與下載格式" src="../img/sheet-light.webp">
</picture>

### 看得見的聲音

每個聲音都有一條依音色明亮程度上色的波形，因此清脆的打擊聲和綿長的環境音在播放之前看起來就不一樣。Hoard 會讀取音效庫的中繼資料（UCS 分類、Soundminer 與 BaseHead 欄位、循環點、速度、根音），並將 MIDI 與 tracker 音樂算繪出來，讓它們像其他聲音一樣播放。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="以彩色波形呈現的音效格狀列表，其中一個已開啟，顯示它的播放器、標籤與各個錄音版本" src="../img/sound-light.webp">
</picture>

### 懂得東西是什麼的搜尋

Hoard 會離線自動為一切加上標籤：`tileable`、`res:2k`、`poly:low`、`rigged`、`loop`、`bpm:120`、`license:cc0`，以及來自遊戲素材分類樹的類別。可以依文字、依顏色或依圖片內容搜尋，也能請任何貼圖或聲音找出「更多類似項目」。接上任何與 OpenAI 相容的模型，無論是本機執行或雲端託管，它還能撰寫名稱與描述。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="搜尋木頭：木桌、樹皮、木板與木質表面一起出現，旁邊是依種類與標籤篩選的選項" src="../img/search-light.webp">
</picture>

### 直接送進你的引擎

- **引擎外掛**支援 Godot 4、Unity、Unreal Engine 5 與 Lumina：在編輯器中瀏覽素材庫，按下「加入專案」即可。模型會連同貼圖一起放到正確的位置，並完成匯入。
- **`hoard sync`** 會將專案指定的檔案精確拉取到專案中，並固定在鎖定檔裡，讓每位隊友與每台建置機器都拿到完全相同的位元組。
- **下載為** GLB、PNG、WAV、MP3 或某種精靈圖表格式，即時轉換，貼圖名稱也會依照你工作室的命名慣例。
- **AI 代理程式**可透過 MCP 搜尋、檢視與取得素材，還能以其登入所用帳號或權杖的權限，完成入口網站能做的其他任何操作。

### 只存一次，妥善保管

每個檔案依其 BLAKE3 雜湊值只儲存一次，因此五個素材包中的同一張貼圖只佔一份空間，任何損毀也不會被忽略。可以存放在本機磁碟或任何 S3 儲存貯體中。你指定給 Hoard 的資料夾只會被讀取，絕不會被修改。帳號、唯讀權杖、單一登入、HTTPS 與 HTTP/3 都已內建。

### 或者，就在你的電腦上

沒有架設伺服器？適用於 Windows、macOS 與 Linux 的桌面應用程式就是同一個素材庫，只是有自己的視窗。安裝後選一個資料夾，就能離線使用：沒有帳號、密碼或伺服器設定，網路上的其他裝置都存取不到它，你輸入的金鑰也會存放在系統鑰匙圈中。新版本會在背景下載，並在你結束程式時安裝。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Windows 視窗中的 Hoard 桌面應用程式：格狀排列的素材庫模型，旁邊的 3D 預覽中開啟著一盞燈籠，並列出它的標籤與下載選項" src="../img/desktop-light.webp">
</picture>

<sub>示範素材庫中的都是 CC0 作品：模型來自 [Poly Haven](https://polyhaven.com)（glTF 與 USD），表面材質來自 [ambientCG](https://ambientcg.com)，聲音、精靈圖表與字型來自 [Kenney](https://kenney.nl)。這些圖片與社群預覽卡是由 `node web/scripts/readme-images.mts` 產生的。</sub>

## 安裝桌面應用程式

從[最新版本](https://github.com/Azazel-Labs/Hoard/releases/latest)下載：Windows 用 `-setup.exe`，Apple 晶片的 macOS 用 `.dmg`，Linux 用 `.AppImage` 或 `.deb`。在 Windows 上，它只會為你一個人安裝，不需要系統管理員權限。安裝程式尚未經過 Apple 或 Microsoft 簽署，因此 Windows 會在執行安裝程式前詢問（其他資訊 → 仍要執行），macOS 則會在第一次開啟應用程式前詢問（系統設定 → 隱私權與安全性 → 強制打開）。

第一次開啟時，它會詢問素材庫要放在哪裡。其他一切都在「設定」中，就像在伺服器上一樣。

之後它會自動保持最新，什麼都不用點：新版本會在背景下載，並在你結束應用程式時安裝，下次開啟 Hoard 時就已經是新版本了。`.deb` 則要安裝下一版來更新。

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
`:1.3` 就只會取得修補版本。若要自動更新，又不想讓 Hoard 在上傳途中重新啟動，請在
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

素材包會經過哪些處理、如何將素材匯入引擎、Unreal 素材包、搜尋、致謝名單、MCP、儲存空間、HTTPS、組態設定、安全性、帳號、API 與概況數據等內容，都在[英文版 README](../../README.md) 中說明。

## 翻譯

Hoard 的入口網頁與伺服器訊息提供 31 種語言，可在「設定 → 外觀」中選擇（預設使用瀏覽器的語言）。最初的翻譯由機器輔助完成；歡迎母語人士提供修正。[docs/translations.md](../translations.md) 說明了如何參與。

## 社群

有問題、想法，或想展示些什麼？歡迎來 [Azazel Labs 的 Discord](https://discord.gg/zf9ncS4EW) 打聲招呼。Bug 和功能請求最好提交為 [GitHub issue](https://github.com/Azazel-Labs/Hoard/issues)，這樣不會被遺漏。

## 授權

[MIT No Attribution](../../LICENSE)（MIT-0）：你可以為任何目的使用、複製、修改與重新散布 Hoard，而且不需要保留著作權聲明。你存放在 Hoard 中的素材包仍各自保有其授權。
