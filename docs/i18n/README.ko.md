<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · 한국어 · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

</div>

<h3 align="center">가지고 있는 모든 에셋 팩을 검색하고, 미리 보고, 클릭 한 번으로 게임에 넣으세요.</h3>

Hoard는 게임 에셋을 위한 셀프 호스팅 라이브러리입니다. zip, Unity 패키지, Fab 다운로드, 파일 만 개가 든 폴더를 넣으면
Hoard가 일을 시작합니다. 모델은 텍스처와 함께 3D로 돌아가고, 흩어진 텍스처 맵은 머티리얼이 되고, 사운드는 파형을 그리고,
스프라이트 시트는 애니메이션을 재생하며, 모든 것에 태그가 붙으므로 `footsteps gravel`이나 `rigged low-poly`를 입력하면
찾을 수 있습니다. 그런 다음 필요한 모든 것을 갖춘 채 Godot, Unity, Unreal로 들어갑니다.

구독도, 누군가의 클라우드로 업로드하는 일도, 사용자당 요금도 없습니다. 내 서버의 Docker 이미지 하나, 또는 내 컴퓨터의 앱
하나만 있으면 온전히 내 것이 됩니다. 결제 절차가 없는 에셋 스토어라고 생각하면 됩니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard 라이브러리: 3D 모델이 그리드로 표시되고, 3D 미리보기에서 열린 보물 상자 옆에 태그와 다운로드가 보입니다" src="../img/hero-light.webp">
</picture>

### 쓰기 전에 모든 것을 확인하세요

모델은 텍스처, 머티리얼, 애니메이션과 함께 3D로 열립니다. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL과 복셀은 3D 앱을
설치하지 않아도 Hoard가 직접 읽습니다. Blender 파일은 헤드리스 Blender를 통해 들어오고, Unreal 패키지는 Unreal 없이도
에디터가 저장해 둔 정보를 보여 줍니다. 썸네일은 서버에서 매번 같은 구도로 그려지므로 팩 전체를 한눈에 훑어볼 수
있습니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="전체 페이지로 표시된 샹들리에: 큰 3D 뷰 옆에 정보, 태그, 연결된 텍스처, 다운로드가 있습니다" src="../img/page-light.webp">
</picture>

### 흩어진 텍스처 맵이 머티리얼로

`Rock_BaseColor.png`, `Rock_Normal.png`, `Rock_Roughness.png`는 조명을 받은 구 위의 Rock 머티리얼 하나가 됩니다. Hoard는
패킹된 맵을 포함해 Megascans, Poly Haven, ambientCG, Substance, Unreal의 이름 규칙을 읽고, Unity `.mat` 파일도 읽습니다.
전체 화면 뷰어는 머티리얼을 구, 큐브, 평면 위에 실제 크기로 보여 주며, 각 맵을 켜고 끌 수 있습니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="전체 화면 뷰어에서 구에 적용된 벽돌 머티리얼과 컬러, 노멀, 러프니스, 높이 맵 스위치" src="../img/material-light.webp">
</picture>

### 재생되는 스프라이트 시트

TexturePacker, Starling, libGDX, Cocos2d, Godot의 아틀라스는 애니메이션을 재생하고 스프라이트 목록을 보여 주는
스프라이트 시트 하나가 됩니다. 사용하는 엔진에 맞게 다시 패킹해서 다운로드하세요. 10가지 형식이 있으며, 각 형식에는 그
형식이 담을 수 있는 정보만 전달됩니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="전체 페이지로 표시된 캐릭터 스프라이트 시트. 애니메이션 하나가 재생되고 옆에 태그와 다운로드 형식이 있습니다" src="../img/sheet-light.webp">
</picture>

### 눈으로 보는 사운드

모든 사운드에는 소리가 얼마나 밝은지에 따라 색이 입혀진 파형이 붙으므로, 날카로운 타격음과 긴 앰비언스는 재생하기 전부터
다르게 보입니다. 사운드 라이브러리 메타데이터(UCS 카테고리, Soundminer와 BaseHead 필드, 루프 포인트, 템포, 루트 노트)를
읽고, MIDI와 트래커 음악은 렌더링해서 다른 사운드처럼 재생할 수 있습니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="색칠된 파형으로 그려진 효과음 그리드. 하나가 열려 플레이어, 태그, 테이크를 보여 줍니다" src="../img/sound-light.webp">
</picture>

### 무엇인지 아는 검색

Hoard는 모든 것에 오프라인으로 직접 태그를 붙입니다. `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0`, 그리고 게임 에셋 분류 트리의 카테고리까지요. 단어, 색상, 그림에 보이는 내용으로 검색하고, 어떤 텍스처나
사운드에서든 ‘비슷한 것 더 보기’를 요청하세요. 로컬이든 호스팅이든 OpenAI 호환 모델을 연결하면 이름과 설명도 써 줍니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="wood 검색: 나무 테이블, 나무껍질, 판자, 나무 표면이 함께 나오고 옆에 종류와 태그별 필터가 있습니다" src="../img/search-light.webp">
</picture>

### 엔진으로 바로

- **엔진 플러그인**은 Godot 4, Unity, Unreal Engine 5, Lumina용입니다. 에디터 안에서 라이브러리를 둘러보고 ‘프로젝트에 추가’를
  누르세요. 모델이 텍스처와 함께 올바른 위치에 임포트되어 들어옵니다.
- **`hoard sync`** 명령은 프로젝트가 지정한 파일만 정확히 가져와 lockfile에 고정하므로, 모든 팀원과 빌드 머신이 같은
  바이트를 받습니다.
- **다른 형식으로 다운로드**: GLB, PNG, WAV, MP3 또는 스프라이트 시트 형식으로 즉석에서 변환되며, 텍스처 이름은
  스튜디오의 규칙을 따릅니다.
- **AI 에이전트**는 MCP로 에셋을 검색하고, 살펴보고, 가져옵니다.

### 한 번 저장하고, 안전하게 보관

모든 파일은 BLAKE3 해시 기준으로 한 번만 저장되므로, 번들 다섯 개에 든 같은 텍스처도 하나만큼의 공간만 차지하고, 손상이
생겨도 놓치지 않습니다. 로컬 디스크나 어떤 S3 버킷에든 보관할 수 있습니다. Hoard에 지정한 폴더는 읽기만 하고 절대
변경하지 않습니다. 계정, 읽기 전용 토큰, 싱글 사인온, HTTPS, HTTP/3가 기본으로 들어 있습니다.

### 아니면 그냥 내 컴퓨터에서

서버를 운영하지 않나요? Windows, macOS, Linux용 데스크톱 앱은 같은 라이브러리를 전용 창에 띄웁니다. 설치하고 폴더를
고르면 오프라인으로 동작합니다. 계정도, 비밀번호도, 서버 설정도 없고, 네트워크의 다른 기기에서는 아무것도 접근할 수
없으며, 입력한 키는 시스템 키체인에 보관됩니다. 새 버전은 백그라운드에서 내려받고 앱을 종료할 때 설치됩니다.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Windows 창에서 실행한 Hoard 데스크톱 앱: 라이브러리의 모델이 그리드로 표시되고, 3D 미리보기에서 열린 랜턴 옆에 태그와 다운로드가 보입니다" src="../img/desktop-light.webp">
</picture>

<sub>데모 라이브러리는 CC0 작품입니다. 모델은 [Poly Haven](https://polyhaven.com)(glTF 및 USD), 표면은
[ambientCG](https://ambientcg.com), 사운드, 스프라이트 시트, 글꼴은 [Kenney](https://kenney.nl)에서 가져왔습니다.
`node web/scripts/readme-images.mts`가 이 그림들과 소셜 카드를 만듭니다.</sub>

## 데스크톱 앱 설치

[최신 릴리스](https://github.com/Azazel-Labs/Hoard/releases/latest)에서 내려받으세요. Windows는 `-setup.exe`, Apple
실리콘 Mac의 macOS는 `.dmg`, Linux는 `.AppImage` 또는 `.deb`입니다. Windows에서는 관리자 권한을 요구하지 않고 현재
사용자에게만 설치됩니다. 설치 프로그램은 아직 Apple이나 Microsoft의 서명을 받지 않았으므로, Windows는 설치 프로그램을
실행하기 전에(추가 정보 → 실행), macOS는 앱을 처음 열기 전에(시스템 설정 → 개인정보 보호 및 보안 → 그래도 열기) 확인을
요청합니다.

처음 열면 라이브러리를 어디에 둘지 묻습니다. 나머지는 서버에서와 마찬가지로 모두 설정에 있습니다.

그 뒤로는 아무것도 누를 필요 없이 스스로 최신 상태를 유지합니다. 새 릴리스는 백그라운드에서 내려받고 앱을 종료할 때
설치되므로, 다음에 Hoard를 열면 새 버전이 실행됩니다. `.deb`는 다음 버전을 설치하면 업데이트됩니다.

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
받으려면 이미지 태그를 `:1.3`로 바꾸세요. 업로드 도중에 Hoard를 다시 시작하지 않고 자동으로 업데이트하려면
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
구성, 보안, 계정, API, 사실과 수치는 [영어 README](../../README.md)에 설명되어 있습니다.

## 번역

Hoard의 포털과 서버 메시지는 31개 언어로 제공되며, 설정 → 모양에서 고를 수 있습니다(기본값은 브라우저 언어).
첫 번역은 기계의 도움을 받았습니다. 원어민의 수정을 환영합니다. 방법은 [docs/translations.md](../translations.md)에 설명되어 있습니다.

## 커뮤니티

질문이나 아이디어, 자랑하고 싶은 것이 있나요? [Azazel Labs Discord](https://discord.gg/zf9ncS4EW)에 들러 인사해 주세요. 버그와 기능 요청은 묻히지 않도록 [GitHub 이슈](https://github.com/Azazel-Labs/Hoard/issues)로 올려 주세요.

## 라이선스

[MIT No Attribution](../../LICENSE)(MIT-0): 저작권 고지를 유지할 필요 없이 어떤 목적으로든 Hoard를 사용, 복사, 수정,
재배포할 수 있습니다. Hoard에 저장하는 에셋 팩은 각자의 라이선스를 따릅니다.
