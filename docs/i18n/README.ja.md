<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · 日本語 · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

</div>

<h3 align="center">持っているすべてのアセットパックを、検索でき、プレビューでき、ワンクリックでゲームへ。</h3>

Hoard はゲームアセットのためのセルフホスト型ライブラリです。zip、Unity パッケージ、Fab のダウンロード、
1 万個のファイルが入ったフォルダーを放り込めば、Hoard が作業に取りかかります。モデルはテクスチャ付きの 3D で回転し、
バラバラのテクスチャマップはマテリアルにまとまり、サウンドは波形を描き、スプライトシートはアニメーションを再生し、
すべてにタグが付くので、`footsteps gravel` や `rigged low-poly` と入力すれば見つかります。
そして必要なものをすべて揃えて Godot、Unity、Unreal に届きます。

サブスクリプションも、誰かのクラウドへのアップロードも、シート単位の料金もありません。
自分のサーバー上の Docker イメージ 1 つ、または自分のコンピューター上のアプリ 1 つだけで、それはあなたのものです。
チェックアウトのないアセットストアだと考えてください。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard のライブラリ: 3D モデルがグリッドに並び、3D プレビューには開いた宝箱と、そのタグとダウンロードが表示されています" src="../img/hero-light.webp">
</picture>

### 使う前にすべてを確認

モデルはテクスチャ、マテリアル、アニメーション付きの 3D で開きます。FBX、glTF、OBJ、USD、SketchUp、3MF、STL、
ボクセルは、3D アプリをインストールしなくても Hoard 自身が読み込みます。
Blender ファイルはヘッドレスの Blender を通して取り込まれ、Unreal パッケージは Unreal なしでも、
エディターが保存した情報を表示します。サムネイルはサーバー上で毎回同じ構図で描画されるので、
パック全体をひと目で見渡せます。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="シャンデリアの全ページ表示: 大きな 3D ビューの横に、その情報、タグ、リンクされたテクスチャ、ダウンロードが並んでいます" src="../img/page-light.webp">
</picture>

### バラバラのテクスチャマップがマテリアルに

`Rock_BaseColor.png`、`Rock_Normal.png`、`Rock_Roughness.png` は、
ライティングされた球の上の 1 つの Rock マテリアルになります。Hoard は Megascans、Poly Haven、ambientCG、Substance、
Unreal の命名規則をパック済みマップも含めて読み取り、Unity の `.mat` ファイルにも対応します。
フルスクリーンビューアーでは、マテリアルを球、立方体、平面の上に実寸のスケールで表示し、
各マップのオン・オフを切り替えられます。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="フルスクリーンビューアーで球に適用されたレンガのマテリアル。カラー、ノーマル、ラフネス、ハイトの各マップのスイッチ付き" src="../img/material-light.webp">
</picture>

### 再生できるスプライトシート

TexturePacker、Starling、libGDX、Cocos2d、Godot のアトラスは、アニメーションを再生し、
含まれるスプライトを一覧表示する 1 つのスプライトシートになります。
使っているエンジンに合わせて再パックしてダウンロードできます。形式は 10 種類で、
それぞれにはその形式が保持できる情報だけが渡されます。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="キャラクターのスプライトシートの全ページ表示。アニメーションの 1 つが再生され、その横にタグとダウンロード形式が並んでいます" src="../img/sheet-light.webp">
</picture>

### 目で見えるサウンド

すべてのサウンドには、音の明るさに応じて色付けされた波形が付くので、
鋭い打撃音と長い環境音は再生する前から見た目で区別できます。サウンドライブラリのメタデータ（UCS カテゴリ、
Soundminer と BaseHead のフィールド、ループポイント、テンポ、ルートノート）を読み取り、
MIDI やトラッカー音楽はレンダリングして、ほかのサウンドと同じように再生できるようにします。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="色付きの波形で描かれた効果音のグリッド。1 つが開かれ、プレーヤー、タグ、テイクが表示されています" src="../img/sound-light.webp">
</picture>

### 中身を理解する検索

Hoard はすべてのものにオフラインで自動的にタグを付けます。`tileable`、`res:2k`、`poly:low`、`rigged`、`loop`、
`bpm:120`、`license:cc0` といったタグと、ゲームアセットの分類ツリーに基づくカテゴリです。言葉、色、
画像に写っているもので検索でき、どのテクスチャやサウンドでも「これに似たもの」を探せます。ローカルでもホスト型でも、
OpenAI 互換のモデルを指定すれば、名前と説明も書いてくれます。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="wood の検索結果: 木のテーブル、樹皮、板、木の表面が一緒に並び、横に種類とタグのフィルターがあります" src="../img/search-light.webp">
</picture>

### エンジンへ直接

- **エンジンプラグイン** は Godot 4、Unity、Unreal Engine 5、Lumina に対応しています。エディター内でライブラリを閲覧し、
  「プロジェクトに追加」を押すと、モデルがテクスチャと一緒に正しい場所へインポートされます。
- **`hoard sync`** は、プロジェクトが指定したファイルだけを正確に取り込み、ロックファイルで固定するので、
  チームメンバー全員とビルドマシンが同じバイト列を受け取ります。
- **形式を指定してダウンロード**: GLB、PNG、WAV、MP3、スプライトシート形式にその場で変換し、
  テクスチャ名はスタジオの命名規則に合わせられます。
- **AI エージェント** は MCP を通じてアセットを検索、調査、取得できます。

### 一度だけ保存し、安全に保管

すべてのファイルは BLAKE3 ハッシュで 1 回だけ保存されるので、
5 つのバンドルに入った同じテクスチャも 1 つ分の容量しか使わず、データの破損も見逃されません。
保存先はローカルディスクでも、任意の S3 バケットでもかまいません。Hoard に指定したフォルダーは読み取るだけで、
決して変更しません。アカウント、読み取り専用トークン、シングルサインオン、HTTPS、HTTP/3 を標準で備えています。

### あるいは自分のコンピューターだけで

サーバーを運用していなくても大丈夫です。
Windows、macOS、Linux 向けのデスクトップアプリは、同じライブラリを専用のウィンドウで開きます。
インストールしてフォルダーを選べば、オフラインで動作します。
アカウントもパスワードもサーバー設定も不要で、ネットワーク上のほかの機器からは何もアクセスできず、
入力したキーはシステムのキーチェーンに保管されます。
新しいバージョンはバックグラウンドでダウンロードされ、終了するときにインストールされます。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Windows のウィンドウで開いた Hoard のデスクトップアプリ: ライブラリのモデルがグリッドに並び、3D プレビューにはランタンが開かれ、そのタグとダウンロードが表示されています" src="../img/desktop-light.webp">
</picture>

<sub>デモライブラリは CC0 のアートです。モデルは [Poly Haven](https://polyhaven.com)（glTF と USD）、
サーフェスは [ambientCG](https://ambientcg.com)、サウンド、スプライトシート、
フォントは [Kenney](https://kenney.nl) のものです。
これらの画像とソーシャルカードは `node web/scripts/readme-images.mts` で作成しています。</sub>

## デスクトップアプリをインストール

[最新リリース](https://github.com/Azazel-Labs/Hoard/releases/latest) からダウンロードしてください。
Windows は `-setup.exe`、Apple シリコン搭載 Mac の macOS は `.dmg`、Linux は `.AppImage` または `.deb` です。
Windows では今のユーザーだけにインストールされ、管理者の許可は求められません。
インストーラーはまだ Apple や Microsoft の署名を受けていないため、
Windows ではインストーラーを実行する前に確認が表示され（詳細情報 → 実行）、
macOS ではアプリを初めて開く前に確認が表示されます（システム設定 → プライバシーとセキュリティ → このまま開く）。

初めて開いたときに、ライブラリの保存場所を尋ねられます。
それ以外はすべて、サーバーと同じく設定にあります。

その後は何もクリックしなくても、自動で最新の状態に保たれます。
新しいリリースはバックグラウンドでダウンロードされ、終了するときにインストールされるので、
次に Hoard を開いたときには新しいバージョンになっています。
`.deb` は次のバージョンをインストールすると更新されます。

## Docker でインストール

Hoard は amd64 と arm64 向けの 1 つのイメージ `ghcr.io/azazel-labs/hoard` として提供されています。次の内容を `compose.yml` として保存してください（
[deploy/compose.yml](../../deploy/compose.yml) と同じものです）:

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

各ボリュームは、代わりにそれぞれに合ったディスク上のフォルダーにすることもできます。`/mnt/big/hoard:/library` とすればファイルは大きな
ディスクに置かれ、データベースは小さくバックアップしやすいままです。マウントするフォルダーに chown は不要です。Hoard は
`/config` と `/cache` を引き受け、`/library` の所有者として動作します（ユーザーを自分で選ぶには `PUID` と `PGID` を設定します）。

それから起動します:

```sh
docker compose up -d        # open http://<host>:4627
```

最初にアクセスするとアカウントの作成を求められ、続いてライブラリの置き場所を尋ねる短いセットアップが開きます。上のボリュームである `/library` が提案されるので、
それを選び、ポータルにパックをドロップしてください。保存場所は後から **設定 → ライブラリの保存場所** で追加できます:

- **Hoard が管理するライブラリ** には、アップロードやインポートしたもののコピーが入ります。それぞれ受け取るファイルの種類にチェックを入れ
  （オーディオはこのディスク、モデルはあのディスク）、1 つが残りを受け取る受け皿になります。保存場所が受け取る種類の変更は
  新しいファイルに適用されます。**保存済みのファイルを合わせて移動** を押すと、すでに保存されているものが移動します。**削除** は、まず保存場所の
  ファイルを種類ごとにほかの保存場所へ移し、空になったら削除します。古いコピーを削除する前に、各ファイルをハッシュで
  確認します。ディスクがなくなった保存場所もそのまま削除でき、その場合ファイルは見つからないものとして表示されます。
- **監視フォルダー** は、すでに持っているパックのフォルダーです。中の各フォルダーがパックになり、その場で
  インデックスされます。Hoard はそこにあるものを一切変更、移動、削除しません。読み取り専用でマウントしてもかまいません。Hoard は起動時と
  その後 15 分ごとに（または **今すぐスキャン** を押したときに）中を調べ、新しいフォルダー、パックに追加された、またはパックから
  削除されたファイル、フォルダーがなくなったパックを拾います。突然空になったフォルダーや到達できなくなったフォルダーは、
  マウントされていないディスクとみなされ、そのパックは保持されます。フォルダーが見つからないパックは、フォルダーが戻ってくる場合に備えて、
  タグやメモとともに 3 日間保持されます。監視パックから完全に削除したものは、ファイルがまだ残っていても、
  フォルダーを再スキャンしたときに削除されたままになります。

保存場所はサーバーから見える任意のフォルダーです。そのため Docker では、まずコンテナーにマウントしてください。

**更新。** 新しいリリースがあると、ヘッダーにバッジが表示されます。`docker compose pull && docker compose up -d`
を実行すると更新できます。Hoard は起動時にデータベースをアップグレードし、その前に `/config/backups` にコピーを保存します。パッチリリースだけを受け取るには、イメージのタグを
`:1.3` に変更してください。アップロードの途中で Hoard を再起動せずに自動で更新するには、
**設定 → メンテナンス → 更新** に呼び出すアドレスを指定してください。リリースが出て何も処理していないときに、Hoard がそれを呼び出します
（Watchtower の HTTP API、Portainer の Webhook、スクリプトなど。[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating) を参照）。

**GPU（任意）。** Hoard は、GPU が見えるときは GPU で、見えないときはソフトウェアで 3D モデルのサムネイルを描画します。
どちらを使っているかはパックのページに表示されます（`GPU: ...` または `No GPU found`）。GPU を渡すには、お使いの
ハードウェア用のオーバーレイを追加します:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

どちらのオーバーレイも [deploy/](../../deploy/) から入手できます。これらはローカルのモデルサーバー（AI タグ付け）も GPU で動かします。NAS 上のファイル、S3 ストレージ、Cloudflare
や Tailscale を通したリモートアクセス、バックアップについては [docs/DEPLOYMENT.md](../DEPLOYMENT.md) で説明しています。

## ソースからビルド

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

または、zip をポータルにドロップしてください。

## この README の続き

パックに起きること、アセットをエンジンに取り込む方法、Unreal パック、検索、クレジット、MCP、ストレージ、HTTPS、
構成、セキュリティ、アカウント、API、ファクトと数値については、[英語の README](../../README.md) で説明しています。

## 翻訳

Hoard のポータルとサーバーのメッセージは 31 の言語で利用でき、設定 → 外観 で選べます（既定ではブラウザーの
言語）。最初の翻訳は機械の支援を受けて作成されました。ネイティブスピーカーによる修正を歓迎します。方法は [docs/translations.md](../translations.md) で説明しています。

## ライセンス

[MIT No Attribution](../../LICENSE)（MIT-0）: 著作権表示を残す必要なく、どんな目的でも Hoard を使用、コピー、変更、再配布できます。
Hoard に保存するアセットパックには、それぞれのライセンスがそのまま適用されます。
