<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · 简体中文 · [繁體中文](README.zh-Hant.md)

</div>

<h3 align="center">你拥有的每一个资源包，都可搜索、可预览，离你的游戏只有一键之遥。</h3>

Hoard 是一个自托管的游戏资源库。丢进一个 zip、一个 Unity 包、一份 Fab 下载或一个装着上万个文件的文件夹，Hoard
就会开始工作。模型带着纹理在 3D 中旋转，零散的纹理贴图变成材质，声音画出自己的波形，精灵图集播放自己的动画，
一切都会被打上标签，所以输入 `footsteps gravel` 或 `rigged low-poly` 就能找到。然后它会带着所需的一切进入 Godot、Unity
或 Unreal。

没有订阅，不上传到任何人的云端，不按席位收费：在你自己的服务器上运行一个 Docker 镜像，或在你自己的电脑上运行一个应用，
它就完全属于你。可以把它看作一个无需结账的资源商店。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard 资源库：3D 模型以网格排列，旁边的 3D 预览中打开着一个宝箱，并显示其标签和下载选项" src="../img/hero-light.webp">
</picture>

### 用之前先看个清楚

模型在 3D 中打开，带有纹理、材质和动画。FBX、glTF、OBJ、USD、SketchUp、3MF、STL 和体素由 Hoard 自己读取，无需安装任何 3D
软件；Blender 文件通过无界面（headless）的 Blender 导入，Unreal 包则无需 Unreal 就能显示编辑器为它们保存的信息。
缩略图在服务器上绘制，每次都以同样的方式取景，因此整个资源包一眼就能浏览完。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="整页显示的一盏吊灯：大幅 3D 视图旁边是它的信息、标签、关联纹理和下载选项" src="../img/page-light.webp">
</picture>

### 零散的纹理贴图变成材质

`Rock_BaseColor.png`、`Rock_Normal.png` 和 `Rock_Roughness.png` 会合成一个 Rock 材质，显示在一个受光照的球体上。Hoard
能读懂 Megascans、Poly Haven、ambientCG、Substance 和 Unreal 的命名方式（包括打包贴图），也能读取 Unity 的 `.mat` 文件。
全屏查看器可将材质显示在球体、立方体或平面上，按真实世界尺寸呈现，每张贴图都可单独开关。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="全屏查看器中的一种砖块材质，显示在球体上，并带有颜色、法线、粗糙度和高度贴图的开关" src="../img/material-light.webp">
</picture>

### 会播放的精灵图集

来自 TexturePacker、Starling、libGDX、Cocos2d 和 Godot 的图集会变成一个精灵图集，可播放其中的动画并列出其中的精灵。
你可以为所用的引擎下载重新打包后的版本：共十种格式，每种格式只写入它能容纳的内容。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="整页显示的一个角色精灵图集，其中一段动画正在播放，旁边是它的标签和下载格式" src="../img/sheet-light.webp">
</picture>

### 看得见的声音

每个声音都有一条按音色明亮程度着色的波形，所以清脆的打击声和绵长的环境声在播放之前看起来就不一样。Hoard
会读取音效库元数据（UCS 分类、Soundminer 和 BaseHead 字段、循环点、速度、根音），并将 MIDI 和 tracker 音乐渲染出来，
让它们像其他声音一样播放。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="以彩色波形呈现的音效网格，其中一个已打开，显示其播放器、标签和各个录音版本" src="../img/sound-light.webp">
</picture>

### 知道内容是什么的搜索

Hoard 会离线自动为一切打标签：`tileable`、`res:2k`、`poly:low`、`rigged`、`loop`、`bpm:120`、`license:cc0`，
以及来自游戏资源分类树的类别。可以按文字、按颜色或按图片内容搜索，还能让任意纹理或声音给出“更多类似内容”。接入任何兼容
OpenAI 的模型，无论是本地的还是托管的，它还能撰写名称和描述。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="搜索木材：木桌、树皮、木板和木质表面同时出现，旁边是按种类和标签筛选的选项" src="../img/search-light.webp">
</picture>

### 直接进入你的引擎

- **引擎插件**支持 Godot 4、Unity、Unreal Engine 5 和 Lumina：在编辑器中浏览资源库，按下“添加到项目”即可。
  模型会连同纹理一起放到正确的位置并完成导入。
- **`hoard sync`** 会把项目指定的文件精确拉取到项目中，并锁定在锁文件里，让每位队友和每台构建机器得到完全相同的字节。
- **下载为** GLB、PNG、WAV、MP3 或某种精灵图集格式，实时转换，纹理名称遵循你工作室的命名约定。
- **AI 智能体**可通过 MCP 搜索、检查和获取资源。

### 只存一次，安全保管

每个文件按其 BLAKE3 哈希只存储一次，因此五个资源包中的同一张纹理只占一份空间，而且任何损坏都不会被忽视。
可以存放在本地磁盘或任意 S3 存储桶中。你指向 Hoard 的文件夹只会被读取，绝不会被修改。账户、只读令牌、单点登录、HTTPS 和
HTTP/3 均为内置功能。

### 或者，就在你的电脑上

没有运行服务器？适用于 Windows、macOS 和 Linux 的桌面应用就是同一个资源库，只是在它自己的窗口里。安装它，选一个文件夹，
它就能离线工作：没有账户、密码或服务器设置，网络中的其他设备访问不到它，你输入的密钥也保存在系统钥匙串中。
新版本会在后台下载，并在你退出时安装。

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Windows 窗口中的 Hoard 桌面应用：资源库中的模型以网格排列，旁边的 3D 预览中打开着一盏灯笼，并显示其标签和下载选项" src="../img/desktop-light.webp">
</picture>

<sub>演示资源库中的都是 CC0 作品：模型来自 [Poly Haven](https://polyhaven.com)（glTF 和 USD），表面材质来自
[ambientCG](https://ambientcg.com)，声音、精灵图集和字体来自 [Kenney](https://kenney.nl)。这些图片和社交卡片由
`node web/scripts/readme-images.mts` 生成。</sub>

## 安装桌面应用

从[最新版本](https://github.com/Azazel-Labs/Hoard/releases/latest)下载：Windows 用 `-setup.exe`，Apple 芯片的 macOS 用
`.dmg`，Linux 用 `.AppImage` 或 `.deb`。在 Windows 上，它只为你一人安装，无需管理员权限。安装程序尚未经过 Apple 或
Microsoft 签名，因此 Windows 会在运行安装程序前询问（更多信息 → 仍要运行），macOS 会在首次打开应用前询问
（系统设置 → 隐私与安全性 → 仍要打开）。

首次打开时，它会询问资源库放在哪里。其他一切都在“设置”中，和服务器上一样。

此后它会自动保持最新，什么都不用点：新版本会在后台下载，并在你退出时安装，下次打开 Hoard 时就已经是新版本了。
`.deb` 则通过安装下一个版本来更新。

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
`:1.3` 即可只获取补丁版本。如果想自动更新，又不希望 Hoard 在上传过程中重启，请在
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
配置、安全、账户、API 以及概况与数据均在[英文 README](../../README.md) 中介绍。

## 翻译

Hoard 的门户和服务器消息提供 31 种语言，可在“设置 → 外观”中选择（默认使用浏览器的
语言）。最初的翻译借助了机器翻译；欢迎母语使用者提出更正。[docs/translations.md](../translations.md) 介绍了具体方法。

## 社区

有问题、想法，或者想展示点什么？欢迎来 [Azazel Labs 的 Discord](https://discord.gg/zf9ncS4EW) 打个招呼。Bug 和功能请求最好提交为 [GitHub issue](https://github.com/Azazel-Labs/Hoard/issues)，这样不会被遗漏。

## 许可证

[MIT No Attribution](../../LICENSE)（MIT-0）：可出于任何目的使用、复制、修改和再分发 Hoard，无需
保留版权声明。你存储在 Hoard 中的资源包仍遵循其各自的许可证。
