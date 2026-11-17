# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · Tiếng Việt · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Mọi gói tài nguyên bạn sở hữu, đều tìm kiếm được, xem trước được và chỉ cách game của bạn một cú nhấp.</h3>

Hoard là một thư viện tài nguyên game tự lưu trữ. Hãy thả vào một tệp zip, một gói Unity, một bản tải về từ Fab hay một
thư mục mười nghìn tệp, và Hoard sẽ bắt tay vào việc. Mô hình xoay trong 3D cùng texture của chúng, các map texture rời
trở thành vật liệu, âm thanh vẽ ra dạng sóng, bảng sprite phát hoạt ảnh của chúng, và mọi thứ đều được gắn thẻ, nên chỉ
cần gõ `footsteps gravel` hay `rigged low-poly` là tìm thấy. Rồi nó đi thẳng vào Godot, Unity hoặc Unreal cùng mọi thứ
nó cần.

Không thuê bao, không tải lên đám mây của ai cả, không tính phí theo từng người dùng: một image Docker trên máy của
chính bạn, và nó thuộc về bạn. Hãy nghĩ về nó như một cửa hàng tài nguyên không có quầy thanh toán.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Thư viện Hoard: các mô hình 3D xếp thành lưới, với một rương báu đang mở trong bản xem trước 3D bên cạnh thẻ và các lựa chọn tải xuống của nó" src="../img/hero-light.webp">
</picture>

### Xem mọi thứ trước khi dùng

Mô hình mở ra trong 3D cùng texture, vật liệu và hoạt ảnh của chúng. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL và voxel
được chính Hoard đọc, không cần cài ứng dụng 3D nào; tệp Blender đi qua Blender chạy headless (không giao diện), còn gói
Unreal hiển thị những gì trình biên tập đã lưu về chúng, mà không cần Unreal. Ảnh thu nhỏ được vẽ trên máy chủ, đóng
khung giống nhau mọi lần, nên có thể lướt qua cả một gói chỉ trong nháy mắt.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Một chiếc đèn chùm hiển thị toàn trang: khung xem 3D lớn bên cạnh thông tin, thẻ, texture liên kết và các lựa chọn tải xuống của nó" src="../img/page-light.webp">
</picture>

### Map texture rời trở thành vật liệu

`Rock_BaseColor.png`, `Rock_Normal.png` và `Rock_Roughness.png` trở thành một vật liệu Rock duy nhất trên một quả cầu
được chiếu sáng. Hoard hiểu cách đặt tên của Megascans, Poly Haven, ambientCG, Substance và Unreal, kể cả các map được
đóng gói chung, và cả tệp `.mat` của Unity. Trình xem toàn màn hình hiển thị vật liệu trên hình cầu, hình lập phương
hoặc mặt phẳng, theo tỉ lệ thực, và từng map có thể bật hoặc tắt.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Một vật liệu gạch trong trình xem toàn màn hình, trên hình cầu, với các công tắc cho map màu, normal, độ nhám và độ cao" src="../img/material-light.webp">
</picture>

### Bảng sprite phát được hoạt ảnh

Atlas từ TexturePacker, Starling, libGDX, Cocos2d và Godot trở thành một bảng sprite duy nhất phát hoạt ảnh và liệt kê
các sprite của nó. Tải xuống bản đóng gói lại cho engine bạn đang dùng: mười định dạng, mỗi định dạng chỉ nhận những gì
nó có thể chứa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Một bảng sprite nhân vật hiển thị toàn trang, một hoạt ảnh của nó đang phát bên cạnh thẻ và các định dạng tải xuống" src="../img/sheet-light.webp">
</picture>

### Âm thanh bạn nhìn thấy được

Mỗi âm thanh có một dạng sóng được tô màu theo độ sáng của âm, nên một cú đánh sắc gọn và một đoạn âm thanh nền dài
trông khác nhau trước cả khi bạn phát chúng. Siêu dữ liệu của thư viện âm thanh được đọc (danh mục UCS, các trường của
Soundminer và BaseHead, điểm lặp, tempo, nốt gốc), còn nhạc MIDI và tracker được kết xuất để phát như mọi âm thanh khác.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Một lưới hiệu ứng âm thanh vẽ thành các dạng sóng có màu, với một hiệu ứng đang mở hiển thị trình phát, thẻ và các bản thu của nó" src="../img/sound-light.webp">
</picture>

### Tìm kiếm hiểu mọi thứ là gì

Hoard tự gắn thẻ mọi thứ, ngoại tuyến: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`, và
một danh mục từ cây phân loại tài nguyên game. Tìm theo từ khóa, theo màu sắc hoặc theo những gì một bức ảnh thể hiện,
và yêu cầu bất kỳ texture hay âm thanh nào cho “Thêm nữa giống thế này”. Trỏ Hoard tới bất kỳ mô hình AI tương thích
OpenAI nào, chạy cục bộ hay được lưu trữ, và nó cũng sẽ viết tên và mô tả.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Tìm kiếm gỗ: một chiếc bàn gỗ, vỏ cây, ván gỗ và một bề mặt gỗ cùng hiện ra, với bộ lọc theo loại và thẻ bên cạnh" src="../img/search-light.webp">
</picture>

### Thẳng vào engine của bạn

- **Plugin engine** cho Godot 4, Unity, Unreal Engine 5 và Lumina: duyệt thư viện ngay trong trình biên tập và nhấn Thêm vào dự
  án. Mô hình đến cùng texture của nó, đúng chỗ, đã được nhập.
- **`hoard sync`** kéo đúng những tệp mà một dự án chỉ định vào dự án đó, được ghim trong lockfile, để mọi đồng đội và
  máy build đều nhận được đúng những byte giống nhau.
- **Tải xuống dưới dạng** GLB, PNG, WAV, MP3 hoặc một định dạng bảng sprite, chuyển đổi tức thì, với tên texture theo
  quy ước của studio bạn.
- **Agent AI** tìm kiếm, kiểm tra và lấy tài nguyên qua MCP.

### Lưu một lần, giữ an toàn

Mỗi tệp được lưu một lần theo mã băm BLAKE3, nên cùng một texture trong năm gói chỉ chiếm chỗ của một, và dữ liệu hỏng
không thể lọt qua mà không bị phát hiện. Hãy lưu trên ổ đĩa cục bộ hoặc bất kỳ bucket S3 nào. Các thư mục bạn trỏ Hoard
tới chỉ được đọc, không bao giờ bị thay đổi. Tài khoản, token chỉ đọc, đăng nhập một lần (SSO), HTTPS và HTTP/3 đều có
sẵn.

<sub>Thư viện demo là tác phẩm CC0: mô hình từ [Poly Haven](https://polyhaven.com) (glTF và USD), bề mặt từ
[ambientCG](https://ambientcg.com), và âm thanh, bảng sprite cùng phông chữ từ [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` tạo ra những hình này và thẻ mạng xã hội.</sub>

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
bản sao vào `/config/backups`. Đổi tag của image thành `:1.2` để chỉ nhận các bản vá. Để tự động cập nhật mà không khởi
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
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
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
bảo mật, tài khoản, API cùng thông tin và số liệu được mô tả trong [README tiếng Anh](../../README.md).

## Bản dịch

Thông báo của cổng web và máy chủ Hoard có ở 31 ngôn ngữ, chọn trong Cài đặt → Giao diện (mặc định là ngôn ngữ của
trình duyệt). Các bản dịch đầu tiên có sự hỗ trợ của máy; chúng tôi hoan nghênh người bản ngữ góp ý sửa lỗi.
[docs/translations.md](../translations.md) giải thích cách làm.

## Giấy phép

[MIT No Attribution](../../LICENSE) (MIT-0): dùng, sao chép, sửa đổi và phân phối lại Hoard cho bất kỳ mục đích nào,
không cần giữ thông báo bản quyền. Các gói tài nguyên bạn lưu trong Hoard giữ giấy phép riêng của chúng.
