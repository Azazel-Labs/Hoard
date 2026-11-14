# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · Tiếng Việt · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Một thư viện tài nguyên tự lưu trữ cho phát triển game. Hãy thả vào cả gói tài nguyên (tệp zip, tarball, thư mục), và
Hoard sẽ lưu mỗi tệp đúng một lần theo mã băm, nhận biết từng tệp là gì, gắn thẻ cho nó, liên kết mesh với texture và
hoạt ảnh của chúng, và cho phép tìm kiếm tất cả từ một cổng web, một CLI, một endpoint MCP cho agent, và lệnh
`hoard sync` dựa trên lockfile, kéo về đúng những tệp mà dự án game của bạn cần.

Hãy nghĩ về nó như một cửa hàng tài nguyên không có quầy thanh toán.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Thư viện Hoard: mô hình và vật liệu xếp thành lưới, với một rương báu đang mở trong bản xem trước 3D bên cạnh thẻ và các lựa chọn tải xuống của nó" src="../img/hero-light.webp">
</picture>

<sub>Thư viện demo là tác phẩm CC0: mô hình từ [Poly Haven](https://polyhaven.com) (glTF và USD) và bề mặt từ
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` tạo ra những hình này và thẻ mạng xã hội.</sub>

## Cài đặt bằng Docker

Hoard được phát hành dưới dạng một image cho amd64 và arm64: `ghcr.io/azazel-labs/hoard`. Hãy lưu nội dung này thành
`compose.yml` (đây là [deploy/compose.yml](../../deploy/compose.yml)):

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

Thay vào đó, mỗi volume có thể là một thư mục trên bất kỳ ổ đĩa nào phù hợp với nó: `/mnt/big/hoard:/library` đặt
tệp của bạn lên ổ đĩa lớn, còn cơ sở dữ liệu vẫn nhỏ và dễ sao lưu. Các thư mục bạn gắn vào không cần chown: Hoard tiếp
quản `/config` và `/cache` và chạy dưới quyền chủ sở hữu của `/library` (đặt `PUID` và `PGID` để tự chọn người dùng).

Sau đó khởi động nó:

```sh
docker compose up -d        # open http://<host>:4627
```

Lần truy cập đầu tiên sẽ yêu cầu bạn tạo tài khoản, rồi mở một bước thiết lập ngắn hỏi thư viện của bạn đặt ở đâu. Nó
gợi ý `/library`, volume ở trên; hãy chọn nó và thả một gói vào cổng web. Có thể thêm vị trí khác sau, từ
**Cài đặt → Vị trí thư viện**:

- **Thư viện do Hoard quản lý** nhận bản sao của những gì bạn tải lên hoặc nhập vào. Mỗi thư viện đánh dấu các loại tệp
  nó nhận (âm thanh trên một ổ đĩa, mô hình trên ổ khác), và một thư viện là vị trí chứa mọi loại cho phần còn lại. Thay
  đổi những gì một vị trí nhận sẽ áp dụng cho tệp mới; **Chuyển tệp đã lưu cho khớp** chuyển những gì đã được lưu.
  **Gỡ** chuyển tệp của một vị trí sang các vị trí khác trước, theo loại, và gỡ nó khi đã trống. Mỗi tệp được kiểm tra
  với mã băm của nó trước khi bản cũ bị xóa. Một vị trí mà ổ đĩa đã mất vẫn có thể gỡ được, và khi đó các tệp của nó
  hiện là bị thiếu.
- **Thư mục theo dõi** là một thư mục chứa các gói bạn đã có. Mỗi thư mục bên trong nó trở thành một gói, được lập chỉ
  mục tại chỗ. Hoard không bao giờ thay đổi, di chuyển hay xóa bất cứ thứ gì ở đó. Bạn có thể gắn nó ở chế độ chỉ đọc
  nếu muốn. Hoard xem qua nó khi khởi động và mỗi 15 phút sau đó (hoặc khi bạn nhấn **Quét ngay**), tìm các thư mục
  mới, các tệp được thêm vào hoặc gỡ khỏi một gói, và các gói mà thư mục đã mất. Một thư mục đột nhiên trống hoặc không
  truy cập được sẽ được coi là ổ đĩa chưa được gắn, và các gói của nó được giữ lại. Một gói bị thiếu thư mục được giữ
  trong ba ngày, cùng thẻ và ghi chú của nó, phòng khi thư mục quay lại. Bất cứ thứ gì bạn xóa vĩnh viễn khỏi một gói
  được theo dõi sẽ vẫn bị xóa khi thư mục được quét lại, dù các tệp của nó vẫn còn đó.

Vị trí là bất kỳ thư mục nào máy chủ nhìn thấy được, nên trong Docker, hãy gắn nó vào container trước.

**Cập nhật.** Thanh trên cùng hiện một huy hiệu khi có bản phát hành mới hơn. Chạy
`docker compose pull && docker compose up -d` để cập nhật; Hoard nâng cấp cơ sở dữ liệu khi khởi động, sau khi lưu một
bản sao vào `/config/backups`. Đổi tag của image thành `:0.8` để chỉ nhận các bản vá. Để tự động cập nhật mà không khởi
động lại Hoard giữa lúc đang tải lên, hãy đưa cho **Cài đặt → Bảo trì → Cập nhật** một địa chỉ để gọi: Hoard gọi nó khi
có bản phát hành và không có việc gì đang chạy (HTTP API của Watchtower, một webhook Portainer, một script; xem
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (tùy chọn).** Hoard kết xuất ảnh thu nhỏ mô hình 3D trên GPU khi nó thấy GPU, và bằng phần mềm khi không thấy.
Trang gói cho biết cách nào đang được dùng (`GPU: ...` hoặc `No GPU found`). Để chuyển GPU vào container, hãy thêm
tệp overlay cho phần cứng của bạn:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Lấy cả hai overlay từ [deploy/](../../deploy/). Chúng cũng đưa máy chủ mô hình cục bộ (gắn thẻ AI) lên GPU. Tệp trên
NAS, lưu trữ S3, truy cập từ xa qua Cloudflare hoặc Tailscale, và sao lưu được trình bày trong
[docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Build từ mã nguồn

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Hoặc thả tệp zip vào cổng web.

## Phần còn lại của README này

Những gì xảy ra với một gói, đưa tài nguyên vào engine, gói Unreal, tìm kiếm, ghi công, MCP, lưu trữ, HTTPS, cấu hình,
bảo mật, tài khoản và API được mô tả trong [README tiếng Anh](../../README.md).

## Bản dịch

Thông báo của cổng web và máy chủ Hoard có ở 31 ngôn ngữ, chọn trong Cài đặt → Giao diện (mặc định là ngôn ngữ của
trình duyệt). Các bản dịch đầu tiên có sự hỗ trợ của máy; chúng tôi hoan nghênh người bản ngữ góp ý sửa lỗi.
[docs/translations.md](../translations.md) giải thích cách làm.

## Giấy phép

[MIT No Attribution](../../LICENSE) (MIT-0): dùng, sao chép, sửa đổi và phân phối lại Hoard cho bất kỳ mục đích nào,
không cần giữ thông báo bản quyền. Các gói tài nguyên bạn lưu trong Hoard giữ giấy phép riêng của chúng.
