# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · 日本語 · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

ゲーム開発のためのセルフホスト型アセットライブラリです。アセットパックを丸ごと（zip、tarball、フォルダー）放り込むと、Hoard
はすべてのファイルをハッシュで 1 回だけ保存し、各ファイルが何であるかを判別してタグを付け、メッシュをテクスチャや
アニメーションと結び付けます。そのすべてを、Web ポータル、CLI、エージェント向けの MCP エンドポイント、そしてゲームプロジェクトに必要なファイルだけを正確に取得するロックファイルベースの
`hoard sync` から検索できるようにします。

チェックアウトのないアセットストアだと考えてください。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard のライブラリ: モデルとマテリアルがグリッドに並び、横の 3D プレビューには開いた宝箱と、そのタグとダウンロードが表示されています" src="../img/hero-light.webp">
</picture>

<sub>デモライブラリは CC0 のアートです。モデルは [Poly Haven](https://polyhaven.com)（glTF と USD）、サーフェスは
[ambientCG](https://ambientcg.com) のものです。これらの画像とソーシャルカードは `node web/scripts/readme-images.mts` で作成しています。</sub>

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
`:0.8` に変更してください。アップロードの途中で Hoard を再起動せずに自動で更新するには、
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
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
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
構成、セキュリティ、アカウント、API については、[英語の README](../../README.md) で説明しています。

## 翻訳

Hoard のポータルとサーバーのメッセージは 31 の言語で利用でき、設定 → 外観 で選べます（既定ではブラウザーの
言語）。最初の翻訳は機械の支援を受けて作成されました。ネイティブスピーカーによる修正を歓迎します。方法は [docs/translations.md](../translations.md) で説明しています。

## ライセンス

[MIT No Attribution](../../LICENSE)（MIT-0）: 著作権表示を残す必要なく、どんな目的でも Hoard を使用、コピー、変更、再配布できます。
Hoard に保存するアセットパックには、それぞれのライセンスがそのまま適用されます。
