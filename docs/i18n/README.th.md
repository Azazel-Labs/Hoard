# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · ไทย · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

คลังแอสเซ็ตสำหรับการพัฒนาเกมที่คุณโฮสต์เอง วางแพ็กแอสเซ็ตทั้งแพ็ก (ไฟล์ zip, tarball, โฟลเดอร์) ลงไป แล้ว Hoard จะจัดเก็บทุกไฟล์เพียงครั้งเดียวตามแฮชของไฟล์ ระบุว่าแต่ละไฟล์คืออะไร ติดแท็ก เชื่อมเมชเข้ากับเท็กซ์เจอร์และแอนิเมชันของเมชนั้น และทำให้ทุกอย่างค้นหาได้จากเว็บพอร์ทัล, CLI, ปลายทาง MCP สำหรับเอเจนต์ และ `hoard sync` ที่ทำงานด้วย lockfile ซึ่งดึงเฉพาะไฟล์ที่โปรเจกต์เกมของคุณต้องใช้

ลองนึกภาพว่าเป็นร้านค้าแอสเซ็ตที่ไม่ต้องชำระเงิน

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="คลังของ Hoard: โมเดลและแมทีเรียลเรียงเป็นตาราง พร้อมหีบสมบัติที่เปิดอยู่ในตัวอย่าง 3 มิติ ข้างแท็กและการดาวน์โหลดของหีบนั้น" src="../img/hero-light.webp">
</picture>

<sub>คลังตัวอย่างเป็นงานศิลป์ CC0: โมเดลจาก [Poly Haven](https://polyhaven.com) (glTF และ USD) และพื้นผิวจาก [ambientCG](https://ambientcg.com) คำสั่ง `node web/scripts/readme-images.mts` ใช้สร้างภาพเหล่านี้และการ์ดโซเชียล</sub>

## ติดตั้งด้วย Docker

Hoard มาเป็นอิมเมจเดียวสำหรับ amd64 และ arm64: `ghcr.io/azazel-labs/hoard` บันทึกไฟล์นี้เป็น `compose.yml` (คือไฟล์ [deploy/compose.yml](../../deploy/compose.yml)):

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

แต่ละวอลุ่มจะเป็นโฟลเดอร์บนดิสก์ที่เหมาะกับวอลุ่มนั้นแทนก็ได้: `/mnt/big/hoard:/library` จะวางไฟล์ของคุณไว้บนดิสก์ขนาดใหญ่ ขณะที่ฐานข้อมูลยังเล็กและสำรองได้ง่าย โฟลเดอร์ที่คุณเมานต์ไม่ต้อง chown: Hoard จะดูแล `/config` และ `/cache` เอง และทำงานในนามของเจ้าของ `/library` (ตั้ง `PUID` และ `PGID` เพื่อเลือกผู้ใช้เอง)

จากนั้นเริ่มการทำงาน:

```sh
docker compose up -d        # open http://<host>:4627
```

การเข้าใช้ครั้งแรกจะให้คุณสร้างบัญชี แล้วเปิดการตั้งค่าเริ่มต้นสั้น ๆ ที่ถามว่าคลังของคุณจะอยู่ที่ใด โดยจะแนะนำ `/library` ซึ่งคือวอลุ่มด้านบน เลือกวอลุ่มนั้นแล้ววางแพ็กลงบนพอร์ทัล ตำแหน่งอื่นเพิ่มได้ภายหลังจาก **การตั้งค่า → ตำแหน่งคลัง**:

- **คลังที่ Hoard ดูแล** จะได้รับสำเนาของสิ่งที่คุณอัปโหลดหรือนำเข้า แต่ละคลังจะติ๊กประเภทไฟล์ที่รับ (เสียงไว้บนดิสก์หนึ่ง โมเดลไว้บนอีกดิสก์หนึ่ง) และมีคลังหนึ่งเป็นที่รับทุกอย่างที่เหลือ การเปลี่ยนสิ่งที่ตำแหน่งรับมีผลกับไฟล์ใหม่ ส่วน **ย้ายไฟล์ที่จัดเก็บไว้ให้ตรงกัน** จะย้ายสิ่งที่จัดเก็บไว้แล้ว **นำออก** จะย้ายไฟล์ของตำแหน่งนั้นไปยังตำแหน่งอื่นตามประเภทก่อน แล้วนำตำแหน่งออกเมื่อว่างแล้ว แต่ละไฟล์จะถูกตรวจสอบกับแฮชก่อนลบสำเนาเดิม ตำแหน่งที่ดิสก์หายไปแล้วก็นำออกได้ และไฟล์ของตำแหน่งนั้นจะแสดงว่าหายไป
- **โฟลเดอร์ที่เฝ้าดู** คือโฟลเดอร์ของแพ็กที่คุณมีอยู่แล้ว แต่ละโฟลเดอร์ข้างในจะกลายเป็นแพ็กหนึ่งแพ็ก ซึ่งจัดทำดัชนีไว้ในที่ที่อยู่เดิม Hoard จะไม่เปลี่ยน ย้าย หรือลบสิ่งใดในนั้นเลย จะเมานต์แบบอ่านอย่างเดียวก็ได้ Hoard จะตรวจดูโฟลเดอร์นั้นเมื่อเริ่มทำงานและทุก 15 นาทีหลังจากนั้น (หรือเมื่อคุณกด **สแกนเดี๋ยวนี้**) เพื่อรับโฟลเดอร์ใหม่ ไฟล์ที่เพิ่มเข้าหรือนำออกจากแพ็ก และแพ็กที่โฟลเดอร์หายไป โฟลเดอร์ที่ว่างเปล่าหรือเข้าถึงไม่ได้อย่างกะทันหันจะถือว่าเป็นดิสก์ที่ไม่ได้เมานต์ และจะเก็บแพ็กของโฟลเดอร์นั้นไว้ แพ็กที่โฟลเดอร์หายไปจะถูกเก็บไว้สามวัน พร้อมแท็กและโน้ต เผื่อโฟลเดอร์กลับมา สิ่งใดที่คุณลบถาวรจากแพ็กที่เฝ้าดูจะยังคงถูกลบเมื่อสแกนโฟลเดอร์ใหม่ แม้ว่าไฟล์จะยังอยู่ก็ตาม

ตำแหน่งคือโฟลเดอร์ใดก็ได้ที่เซิร์ฟเวอร์มองเห็น ดังนั้นใน Docker ให้เมานต์โฟลเดอร์นั้นเข้าไปในคอนเทนเนอร์ก่อน

**การอัปเดต** ส่วนหัวจะแสดงป้ายเมื่อมีรุ่นที่ใหม่กว่า เรียกใช้ `docker compose pull && docker compose up -d` เพื่ออัปเดต Hoard จะอัปเกรดฐานข้อมูลเมื่อเริ่มทำงาน โดยบันทึกสำเนาไว้ใน `/config/backups` ก่อน เปลี่ยนแท็กของอิมเมจเป็น `:1.2` เพื่อรับเฉพาะรุ่นแพตช์ หากต้องการอัปเดตอัตโนมัติโดยไม่ให้ Hoard รีสตาร์ตกลางการอัปโหลด ให้ระบุที่อยู่ที่จะเรียกใน **การตั้งค่า → การบำรุงรักษา → การอัปเดต**: Hoard จะเรียกที่อยู่นั้นเมื่อมีรุ่นใหม่ออกและไม่มีงานใดทำอยู่ (HTTP API ของ Watchtower, webhook ของ Portainer, สคริปต์ ดู [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating))

**GPU (ไม่บังคับ)** Hoard เรนเดอร์ภาพย่อของโมเดล 3 มิติบน GPU เมื่อมองเห็น GPU และใช้ซอฟต์แวร์เมื่อมองไม่เห็น หน้าแพ็กจะแสดงว่าใช้แบบใด (`GPU: ...` หรือ `No GPU found`) หากต้องการส่ง GPU เข้าไป ให้เพิ่มโอเวอร์เลย์สำหรับฮาร์ดแวร์ของคุณ:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

ดาวน์โหลดโอเวอร์เลย์ทั้งสองได้จาก [deploy/](../../deploy/) โอเวอร์เลย์เหล่านี้ยังให้เซิร์ฟเวอร์โมเดลในเครื่อง (การติดแท็กด้วย AI) ทำงานบน GPU ด้วย ไฟล์บน NAS, ที่เก็บข้อมูล S3, การเข้าถึงระยะไกลผ่าน Cloudflare หรือ Tailscale และการสำรองข้อมูล มีอธิบายไว้ใน [docs/DEPLOYMENT.md](../DEPLOYMENT.md)

## บิลด์จากซอร์สโค้ด

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

หรือวางไฟล์ zip ลงบนพอร์ทัล

## ส่วนที่เหลือของ README นี้

สิ่งที่เกิดขึ้นกับแพ็ก การนำแอสเซ็ตเข้าเอนจิน แพ็ก Unreal การค้นหา เครดิต MCP การจัดเก็บ HTTPS การกำหนดค่า ความปลอดภัย บัญชี และ API มีอธิบายไว้ใน [README ภาษาอังกฤษ](../../README.md)

## คำแปล

ข้อความของพอร์ทัลและเซิร์ฟเวอร์ของ Hoard มีให้เลือก 31 ภาษา โดยเลือกได้ที่ การตั้งค่า → รูปลักษณ์ (ค่าเริ่มต้นคือภาษาของเบราว์เซอร์) คำแปลชุดแรกทำโดยมีเครื่องช่วย ยินดีรับการแก้ไขจากเจ้าของภาษา [docs/translations.md](../translations.md) อธิบายวิธีการไว้

## สัญญาอนุญาต

[MIT No Attribution](../../LICENSE) (MIT-0): ใช้ คัดลอก แก้ไข และแจกจ่าย Hoard ซ้ำได้เพื่อวัตถุประสงค์ใดก็ได้ โดยไม่จำเป็นต้องคงประกาศลิขสิทธิ์ไว้ แพ็กแอสเซ็ตที่คุณจัดเก็บใน Hoard ยังคงใช้สัญญาอนุญาตของตัวเอง
