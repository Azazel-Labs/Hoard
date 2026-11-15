# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · 简体中文 · [繁體中文](README.zh-Hant.md)

一个面向游戏开发的自托管资源库。把整个资源包（zip、tar 包、文件夹）丢进来，Hoard 会按哈希值将每个文件只存储一次，
识别每个文件是什么，为它打标签，把网格与其纹理和动画关联起来，并让这一切都可以通过 Web 门户、CLI、供智能体使用的 MCP
端点进行搜索，还可以通过基于锁文件、能精确拉取你的游戏项目所需文件的 `hoard sync` 获取。

可以把它看作一个无需结账的资源商店。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard 资源库：模型和材质以网格排列，旁边的 3D 预览中打开着一个宝箱，并显示其标签和下载选项" src="../img/hero-light.webp">
</picture>

<sub>演示资源库中的都是 CC0 作品：模型来自 [Poly Haven](https://polyhaven.com)（glTF 和 USD），表面材质来自
[ambientCG](https://ambientcg.com)。这些图片和社交卡片由 `node web/scripts/readme-images.mts` 生成。</sub>

## 使用 Docker 安装

Hoard 以单个镜像发布，支持 amd64 和 arm64：`ghcr.io/azazel-labs/hoard`。将以下内容保存为 `compose.yml`（即
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

每个卷也可以改为放在最合适的磁盘上的某个文件夹：`/mnt/big/hoard:/library` 会把你的文件放到大容量磁盘上，
而数据库依然很小，便于备份。挂载的文件夹无需 chown：Hoard 会接管
`/config` 和 `/cache`，并以 `/library` 的所有者身份运行（设置 `PUID` 和 `PGID` 可自行指定用户）。

然后启动它：

```sh
docker compose up -d        # open http://<host>:4627
```

首次访问时会要求你创建一个账户，然后打开一个简短的设置向导，询问你的资源库要放在哪里。它会建议使用 `/library`，即上面的卷；选择
它，然后把一个资源包拖放到门户上即可。之后可以在 **设置 → 资源库位置** 中添加更多位置：

- **由 Hoard 管理的资源库** 会保存你上传或导入的内容的副本。每个资源库都勾选了它接收的文件类型
  （音频放在一块磁盘上，模型放在另一块上），其中一个是接收其余文件的兜底位置。修改位置接收的类型
  只对新文件生效；**移动已存储的文件以匹配** 会移动已经存储的文件。**移除** 会先按类型把该位置的
  文件移到其他位置，清空后再将其移除。每个文件在删除旧副本之前都会用其哈希值进行校验。
  磁盘已不存在的位置也可以直接移除，其中的文件随后会显示为缺失。
- **监视文件夹** 是你已有的资源包所在的文件夹。其中的每个子文件夹都会成为一个资源包，并在原处建立索引。
  Hoard 从不修改、移动或删除其中的任何内容。你可以以只读方式挂载它。Hoard 会在启动时
  以及之后每 15 分钟（或在你点击 **立即扫描** 时）浏览一遍，发现新的文件夹、资源包中新增或
  删除的文件，以及文件夹已不存在的资源包。突然变空或无法访问的文件夹会被视为
  未挂载的磁盘，其中的资源包会被保留。文件夹缺失的资源包会连同其标签和备注保留三天，
  以防文件夹再次出现。你从监视资源包中永久删除的任何内容，在重新扫描文件夹时仍保持删除状态，
  即使其文件仍在原处。

位置可以是服务器能看到的任何文件夹，因此在 Docker 中，请先将其挂载到容器中。

**更新。** 有新版本发布时，页眉会显示一个徽标。运行 `docker compose pull && docker compose up -d`
即可更新；Hoard 会在启动时升级其数据库，并先在 `/config/backups` 中保存一份副本。将镜像标签改为
`:0.8` 即可只获取补丁版本。如果想自动更新，又不希望 Hoard 在上传过程中重启，请在
**设置 → 维护 → 更新** 中填写一个要调用的地址：新版本发布且没有正在进行的任务时，Hoard 会调用它一次
（Watchtower 的 HTTP API、Portainer webhook、脚本均可；参见 [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)）。

**GPU（可选）。** 能检测到 GPU 时，Hoard 会用 GPU 渲染 3D 模型缩略图，否则使用软件渲染。
资源包页面会显示使用的是哪一种（`GPU: ...` 或 `No GPU found`）。要直通 GPU，请添加与你的
硬件对应的覆盖配置：

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

两个覆盖配置都可以从 [deploy/](../../deploy/) 获取。它们还会让本地模型服务器（AI 标注）使用 GPU。将文件放在 NAS 或 S3 存储上、通过 Cloudflare
或 Tailscale 远程访问以及备份，请参见 [docs/DEPLOYMENT.md](../DEPLOYMENT.md)。

## 从源代码构建

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

或者直接把 zip 拖放到门户上。

## 本 README 的其余部分

资源包的处理过程、将资源导入引擎、Unreal 资源包、搜索、致谢、MCP、存储、HTTPS、
配置、安全、账户和 API 均在[英文 README](../../README.md) 中介绍。

## 翻译

Hoard 的门户和服务器消息提供 31 种语言，可在“设置 → 外观”中选择（默认使用浏览器的
语言）。最初的翻译借助了机器翻译；欢迎母语使用者提出更正。[docs/translations.md](../translations.md) 介绍了具体方法。

## 许可证

[MIT No Attribution](../../LICENSE)（MIT-0）：可出于任何目的使用、复制、修改和再分发 Hoard，无需
保留版权声明。你存储在 Hoard 中的资源包仍遵循其各自的许可证。
