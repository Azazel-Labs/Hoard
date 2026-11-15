# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · 한국어 · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

게임 개발을 위한 셀프 호스팅 에셋 라이브러리입니다. 에셋 팩을 통째로(zip, tarball, 폴더) 넣으면 Hoard가 모든 파일을
해시 기준으로 한 번만 저장하고, 각 파일이 무엇인지 알아내 태그를 붙이고, 메시를 텍스처 및 애니메이션과 연결합니다.
그리고 이 모든 것을 웹 포털, CLI, 에이전트용 MCP 엔드포인트에서 검색할 수 있게 하며, lockfile 기반의
`hoard sync`로 게임 프로젝트에 필요한 파일만 정확히 받아 올 수 있습니다.

결제 절차가 없는 에셋 스토어라고 생각하면 됩니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard 라이브러리: 모델과 머티리얼이 그리드로 표시되고, 3D 미리보기에서 열린 보물 상자 옆에 태그와 다운로드가 보입니다" src="../img/hero-light.webp">
</picture>

<sub>데모 라이브러리는 CC0 작품입니다. 모델은 [Poly Haven](https://polyhaven.com)(glTF 및 USD), 표면은
[ambientCG](https://ambientcg.com)에서 가져왔습니다. `node web/scripts/readme-images.mts`가 이 그림들과 소셜 카드를 만듭니다.</sub>

## Docker로 설치

Hoard는 amd64와 arm64용 이미지 하나로 배포됩니다: `ghcr.io/azazel-labs/hoard`. 다음을 `compose.yml`로 저장하세요
([deploy/compose.yml](../../deploy/compose.yml)과 같습니다):

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

각 볼륨 대신 적당한 디스크의 폴더를 써도 됩니다. `/mnt/big/hoard:/library`로 지정하면 파일은 큰 디스크에 두고,
데이터베이스는 작고 백업하기 쉬운 상태로 유지됩니다. 마운트한 폴더에 chown을 할 필요는 없습니다. Hoard가
`/config`와 `/cache`를 넘겨받고 `/library`의 소유자로 실행됩니다(사용자를 직접 고르려면 `PUID`와 `PGID`를 설정하세요).

그런 다음 시작합니다:

```sh
docker compose up -d        # open http://<host>:4627
```

처음 접속하면 계정을 만들라고 하고, 이어서 라이브러리를 어디에 둘지 묻는 짧은 설정이 열립니다. 위의 볼륨인 `/library`를 제안하니,
그것을 고르고 포털에 팩을 놓으세요. 나중에 **설정 → 라이브러리 위치**에서 위치를 더 추가할 수 있습니다:

- **Hoard가 보관하는 라이브러리**에는 업로드하거나 가져온 파일의 사본이 들어갑니다. 각 위치는 받을 파일 종류에 체크하고
  (오디오는 이 디스크, 모델은 저 디스크), 하나는 나머지를 받는 기본 위치가 됩니다. 위치가 받는 종류를 바꾸면
  새 파일에 적용되며, **저장된 파일을 설정에 맞게 옮기기**는 이미 저장된 파일을 옮깁니다. **제거**는 먼저 위치의
  파일을 종류에 따라 다른 위치로 옮기고, 비고 나면 위치를 제거합니다. 이전 사본을 삭제하기 전에 각 파일을 해시와
  대조해 확인합니다. 디스크가 사라진 위치도 제거할 수 있으며, 그러면 그 파일은 없는 것으로 표시됩니다.
- **감시 폴더**는 이미 가지고 있는 팩 폴더입니다. 그 안의 각 폴더가 팩이 되며, 있는 자리에서 색인됩니다.
  Hoard는 그곳의 어떤 것도 변경, 이동, 삭제하지 않습니다. 원하면 읽기 전용으로 마운트하세요. Hoard는 시작할 때와
  그 뒤 15분마다(또는 **지금 검사**를 누르면) 폴더를 살펴보고, 새 폴더, 팩에 추가되거나 팩에서 제거된 파일, 폴더가
  사라진 팩을 찾아냅니다. 갑자기 비었거나 접근할 수 없는 폴더는 마운트되지 않은 디스크로 보고 그 팩을 유지합니다.
  폴더가 없어진 팩은 폴더가 돌아올 경우를 대비해 태그와 메모와 함께 3일 동안 유지됩니다. 감시 중인 팩에서
  영구 삭제한 것은 파일이 그대로 있더라도 폴더를 다시 검사할 때 삭제된 상태로 남습니다.

위치는 서버가 볼 수 있는 폴더라면 어디든 됩니다. 따라서 Docker에서는 먼저 컨테이너에 마운트하세요.

**업데이트.** 새 릴리스가 있으면 헤더에 배지가 표시됩니다. `docker compose pull && docker compose up -d`를 실행하면
적용되며, Hoard는 시작할 때 데이터베이스를 업그레이드하기 전에 `/config/backups`에 사본을 저장합니다. 패치 릴리스만
받으려면 이미지 태그를 `:0.8`로 바꾸세요. 업로드 도중에 Hoard를 다시 시작하지 않고 자동으로 업데이트하려면
**설정 → 유지 관리 → 업데이트**에 호출할 주소를 입력하세요. 릴리스가 나오고 진행 중인 작업이 없을 때 Hoard가
그 주소를 호출합니다(Watchtower의 HTTP API, Portainer 웹훅, 스크립트 등. [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating) 참고).

**GPU(선택 사항).** Hoard는 GPU가 보이면 GPU로, 보이지 않으면 소프트웨어로 3D 모델 썸네일을 렌더링합니다.
팩 페이지에 어느 쪽인지 표시됩니다(`GPU: ...` 또는 `No GPU found`). GPU를 넘겨주려면 하드웨어에 맞는 오버레이를
추가하세요:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

두 오버레이는 [deploy/](../../deploy/)에서 받을 수 있습니다. 로컬 모델 서버(AI 태깅)도 GPU에서 실행되게 합니다. NAS의 파일, S3 스토리지, Cloudflare나
Tailscale을 통한 원격 접속, 백업은 [docs/DEPLOYMENT.md](../DEPLOYMENT.md)에서 다룹니다.

## 소스에서 빌드

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

또는 zip을 포털에 놓으세요.

## 이 README의 나머지 내용

팩이 어떻게 처리되는지, 엔진으로 에셋 가져오기, Unreal 팩, 검색, 크레디트, MCP, 스토리지, HTTPS,
구성, 보안, 계정, API는 [영어 README](../../README.md)에 설명되어 있습니다.

## 번역

Hoard의 포털과 서버 메시지는 31개 언어로 제공되며, 설정 → 모양에서 고를 수 있습니다(기본값은 브라우저 언어).
첫 번역은 기계의 도움을 받았습니다. 원어민의 수정을 환영합니다. 방법은 [docs/translations.md](../translations.md)에 설명되어 있습니다.

## 라이선스

[MIT No Attribution](../../LICENSE)(MIT-0): 저작권 고지를 유지할 필요 없이 어떤 목적으로든 Hoard를 사용, 복사, 수정,
재배포할 수 있습니다. Hoard에 저장하는 에셋 팩은 각자의 라이선스를 따릅니다.
