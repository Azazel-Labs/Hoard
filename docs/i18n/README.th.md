<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· ไ&#8288;ท&#8288;ย&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">แพ็กแอสเซ็ตทุกแพ็กที่คุณมี ค้นหาได้ ดูตัวอย่างได้ และห่างจากเกมของคุณเพียงคลิกเดียว</h3>

Hoard คือคลังแอสเซ็ตเกมที่คุณโฮสต์เอง วางไฟล์ zip, แพ็กเกจ Unity, ไฟล์ที่ดาวน์โหลดจาก Fab หรือโฟลเดอร์ที่มีไฟล์นับหมื่นลงไป แล้ว Hoard จะเริ่มทำงานทันที โมเดลหมุนดูได้ในแบบ 3 มิติพร้อมเท็กซ์เจอร์ แมปเท็กซ์เจอร์ที่กระจัดกระจายกลายเป็นแมทีเรียล เสียงวาดรูปคลื่นของตัวเอง แผ่นสไปรต์เล่นแอนิเมชันของตัวเอง และทุกอย่างถูกติดแท็ก เพียงพิมพ์ `footsteps gravel` หรือ `rigged low-poly` ก็เจอ จากนั้นแอสเซ็ตก็ส่งเข้า Godot, Unity หรือ Unreal พร้อมทุกอย่างที่ต้องใช้

ไม่มีค่าสมาชิก ไม่ต้องอัปโหลดขึ้นคลาวด์ของใคร ไม่คิดราคาต่อผู้ใช้: อิมเมจ Docker เพียงตัวเดียวบนเซิร์ฟเวอร์ของคุณเอง หรือแอปบนคอมพิวเตอร์ของคุณเอง แล้วมันก็เป็นของคุณ ลองนึกภาพว่าเป็นร้านค้าแอสเซ็ตที่ไม่ต้องชำระเงิน

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="คลังของ Hoard: โมเดล 3 มิติเรียงเป็นตาราง พร้อมหีบสมบัติที่เปิดอยู่ในตัวอย่าง 3 มิติ ข้างแท็กและการดาวน์โหลดของหีบนั้น" src="../img/hero-light.webp">
</picture>

### ดูทุกอย่างก่อนนำไปใช้

โมเดลเปิดในแบบ 3 มิติพร้อมเท็กซ์เจอร์ แมทีเรียล และแอนิเมชัน Hoard อ่านไฟล์ FBX, glTF, OBJ, USD, SketchUp, 3MF, STL และวอกเซลได้เอง โดยไม่ต้องติดตั้งแอป 3 มิติใดๆ ไฟล์ Blender นำเข้าผ่าน Blender แบบ headless ส่วนแพ็กเกจ Unreal จะแสดงสิ่งที่เอดิเตอร์บันทึกไว้เกี่ยวกับแพ็กเกจนั้นโดยไม่ต้องมี Unreal ภาพย่อวาดบนเซิร์ฟเวอร์และจัดเฟรมแบบเดียวกันทุกครั้ง จึงกวาดตาดูทั้งแพ็กได้ในพริบตา

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="โคมระย้าที่แสดงเต็มหน้า: มุมมอง 3 มิติขนาดใหญ่ข้างข้อมูล แท็ก เท็กซ์เจอร์ที่เชื่อมโยง และการดาวน์โหลดของโคมนั้น" src="../img/page-light.webp">
</picture>

### แมปเท็กซ์เจอร์ที่กระจัดกระจายกลายเป็นแมทีเรียล

`Rock_BaseColor.png`, `Rock_Normal.png` และ `Rock_Roughness.png` กลายเป็นแมทีเรียล Rock หนึ่งเดียวบนทรงกลมที่มีแสงส่อง Hoard อ่านรูปแบบการตั้งชื่อของ Megascans, Poly Haven, ambientCG, Substance และ Unreal ได้ รวมถึงแมปที่แพ็กรวมกัน และอ่านไฟล์ `.mat` ของ Unity ได้ด้วย ตัวดูแบบเต็มจอแสดงแมทีเรียลบนทรงกลม ลูกบาศก์ หรือระนาบ ในสเกลจริง โดยเปิดและปิดแต่ละแมปได้

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="แมทีเรียลอิฐในตัวดูแบบเต็มจอ บนทรงกลม พร้อมสวิตช์สำหรับแมปสี นอร์มัล ความหยาบ และความสูง" src="../img/material-light.webp">
</picture>

### แผ่นสไปรต์ที่เล่นได้

แอตลาสจาก TexturePacker, Starling, libGDX, Cocos2d และ Godot กลายเป็นแผ่นสไปรต์หนึ่งเดียวที่เล่นแอนิเมชันและแสดงรายการสไปรต์ของตัวเอง ดาวน์โหลดแบบแพ็กใหม่สำหรับเอนจินที่คุณใช้อยู่ได้: สิบรูปแบบ แต่ละรูปแบบได้รับเฉพาะสิ่งที่รูปแบบนั้นรองรับได้

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="แผ่นสไปรต์ตัวละครที่แสดงเต็มหน้า โดยมีแอนิเมชันหนึ่งกำลังเล่นอยู่ข้างแท็กและรูปแบบการดาวน์โหลด" src="../img/sheet-light.webp">
</picture>

### เสียงที่มองเห็นได้

เสียงทุกเสียงได้รูปคลื่นที่ลงสีตามความสว่างของเสียง เสียงกระแทกที่คมและเสียงบรรยากาศยาวๆ จึงดูต่างกันตั้งแต่ก่อนกดเล่น Hoard อ่านเมทาดาทาของคลังเสียง (หมวดหมู่ UCS, ฟิลด์ของ Soundminer และ BaseHead, จุดวนซ้ำ, เทมโป, โน้ตราก) และเรนเดอร์เพลง MIDI และเพลง tracker ให้เล่นได้เหมือนเสียงอื่นๆ

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="ตารางเอฟเฟกต์เสียงที่วาดเป็นรูปคลื่นสีต่างๆ โดยมีเสียงหนึ่งเปิดอยู่ แสดงตัวเล่น แท็ก และเทคต่างๆ ของเสียงนั้น" src="../img/sound-light.webp">
</picture>

### การค้นหาที่รู้ว่าแต่ละอย่างคืออะไร

Hoard ติดแท็กทุกอย่างเองแบบออฟไลน์: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0` และหมวดหมู่จากแผนผังหมวดหมู่แอสเซ็ตเกม ค้นหาด้วยคำ ด้วยสี หรือด้วยสิ่งที่อยู่ในภาพ และขอ “เพิ่มเติมที่คล้ายกัน” จากเท็กซ์เจอร์หรือเสียงใดก็ได้ เชื่อม Hoard กับโมเดลที่เข้ากันได้กับ OpenAI ตัวใดก็ได้ ทั้งที่รันในเครื่องหรือบนบริการโฮสต์ แล้วมันจะเขียนชื่อและคำอธิบายให้ด้วย

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="การค้นหาไม้: โต๊ะไม้ เปลือกไม้ แผ่นไม้กระดาน และพื้นผิวไม้อยู่ด้วยกัน พร้อมตัวกรองตามชนิดและแท็กอยู่ข้างๆ" src="../img/search-light.webp">
</picture>

### ส่งตรงเข้าเอนจินของคุณ

- **ปลั๊กอินเอนจิน** สำหรับ Godot 4, Unity, Unreal Engine 5 และ Lumina: เรียกดูคลังได้ในเอดิเตอร์แล้วกด “เพิ่มลงในโปรเจกต์” โมเดลจะมาพร้อมเท็กซ์เจอร์ วางไว้ในตำแหน่งที่ถูกต้อง และนำเข้าเรียบร้อย
- **`hoard sync`** ดึงเฉพาะไฟล์ที่โปรเจกต์ระบุไว้เข้าสู่โปรเจกต์นั้น โดยตรึงไว้ใน lockfile เพื่อให้เพื่อนร่วมทีมและเครื่องบิลด์ทุกเครื่องได้ข้อมูลไบต์เดียวกัน
- **ดาวน์โหลดเป็น** GLB, PNG, WAV, MP3 หรือรูปแบบแผ่นสไปรต์ แปลงให้ทันที พร้อมตั้งชื่อเท็กซ์เจอร์ตามแบบแผนของสตูดิโอคุณ
- **เอเจนต์ AI** ค้นหา ตรวจดู และดึงแอสเซ็ตผ่าน MCP

### จัดเก็บครั้งเดียว ปลอดภัยเสมอ

ทุกไฟล์จัดเก็บเพียงครั้งเดียวตามแฮช BLAKE3 เท็กซ์เจอร์เดียวกันในห้าบันเดิลจึงกินพื้นที่เท่าหนึ่งไฟล์ และความเสียหายของข้อมูลจะไม่มีทางรอดพ้นการตรวจพบ เก็บไว้บนดิสก์ในเครื่องหรือบัคเก็ต S3 ใดก็ได้ โฟลเดอร์ที่คุณชี้ให้ Hoard ใช้จะถูกอ่านเท่านั้น ไม่มีการแก้ไขเลย บัญชี โทเค็นแบบอ่านอย่างเดียว การลงชื่อเข้าใช้ครั้งเดียว (SSO), HTTPS และ HTTP/3 มีมาให้ในตัว

### หรือใช้แค่บนคอมพิวเตอร์ของคุณ

ไม่ได้รันเซิร์ฟเวอร์ใช่ไหม แอปเดสก์ท็อปสำหรับ Windows, macOS และ Linux คือคลังเดียวกันในหน้าต่างของตัวเอง ติดตั้ง เลือกโฟลเดอร์ แล้วใช้งานแบบออฟไลน์ได้เลย ไม่มีบัญชี รหัสผ่าน หรือการตั้งค่าเซิร์ฟเวอร์ ไม่มีอะไรที่เครื่องอื่นในเครือข่ายเข้าถึงได้ และคีย์ที่คุณพิมพ์ลงไปจะเก็บไว้ในที่เก็บรหัสผ่านของระบบ รุ่นใหม่จะดาวน์โหลดไว้เบื้องหลังและติดตั้งเมื่อคุณปิดโปรแกรม

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="แอปเดสก์ท็อป Hoard ในหน้าต่าง Windows: โมเดลในคลังเรียงเป็นตาราง พร้อมตะเกียงที่เปิดอยู่ในตัวอย่าง 3 มิติ ข้างแท็กและการดาวน์โหลดของตะเกียงนั้น" src="../img/desktop-light.webp">
</picture>

<sub>คลังตัวอย่างเป็นงานศิลป์ CC0: โมเดลจาก [Poly Haven](https://polyhaven.com) (glTF และ USD) พื้นผิวจาก [ambientCG](https://ambientcg.com) และเสียง แผ่นสไปรต์ และฟอนต์จาก [Kenney](https://kenney.nl) คำสั่ง `node web/scripts/readme-images.mts` ใช้สร้างภาพเหล่านี้และการ์ดโซเชียล</sub>

## ติดตั้งแอปเดสก์ท็อป

ดาวน์โหลดได้จาก[รุ่นล่าสุด](https://github.com/Azazel-Labs/Hoard/releases/latest): ไฟล์ `-setup.exe` สำหรับ Windows, `.dmg` สำหรับ macOS บน Apple silicon และ `.AppImage` หรือ `.deb` สำหรับ Linux บน Windows แอปจะติดตั้งให้เฉพาะคุณคนเดียว โดยไม่ต้องขอสิทธิ์ผู้ดูแลระบบ ตัวติดตั้งยังไม่ได้เซ็นโดย Apple หรือ Microsoft ดังนั้น Windows จะถามก่อนเรียกใช้ตัวติดตั้ง (More info → Run anyway) และ macOS จะถามก่อนเปิดแอปครั้งแรก (System Settings → Privacy & Security → Open Anyway)

เมื่อเปิดครั้งแรก แอปจะถามว่าจะให้คลังอยู่ที่ไหน ส่วนอย่างอื่นทั้งหมดอยู่ในการตั้งค่า เหมือนบนเซิร์ฟเวอร์

หลังจากนั้นแอปจะอัปเดตตัวเองโดยที่คุณไม่ต้องกดอะไรเลย รุ่นใหม่จะดาวน์โหลดอยู่เบื้องหลังและติดตั้งเมื่อคุณปิดแอป ครั้งต่อไปที่คุณเปิด Hoard ก็จะเป็นรุ่นใหม่แล้ว ส่วน `.deb` อัปเดตได้ด้วยการติดตั้งไฟล์รุ่นถัดไป

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

**การอัปเดต** ส่วนหัวจะแสดงป้ายเมื่อมีรุ่นที่ใหม่กว่า เรียกใช้ `docker compose pull && docker compose up -d` เพื่ออัปเดต Hoard จะอัปเกรดฐานข้อมูลเมื่อเริ่มทำงาน โดยบันทึกสำเนาไว้ใน `/config/backups` ก่อน เปลี่ยนแท็กของอิมเมจเป็น `:1.3` เพื่อรับเฉพาะรุ่นแพตช์ หากต้องการอัปเดตอัตโนมัติโดยไม่ให้ Hoard รีสตาร์ตกลางการอัปโหลด ให้ระบุที่อยู่ที่จะเรียกใน **การตั้งค่า → การบำรุงรักษา → การอัปเดต**: Hoard จะเรียกที่อยู่นั้นเมื่อมีรุ่นใหม่ออกและไม่มีงานใดทำอยู่ (HTTP API ของ Watchtower, webhook ของ Portainer, สคริปต์ ดู [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating))

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

สิ่งที่เกิดขึ้นกับแพ็ก การนำแอสเซ็ตเข้าเอนจิน แพ็ก Unreal การค้นหา เครดิต MCP การจัดเก็บ HTTPS การกำหนดค่า ความปลอดภัย บัญชี API รวมถึงข้อเท็จจริงและตัวเลข มีอธิบายไว้ใน [README ภาษาอังกฤษ](../../README.md)

## คำแปล

ข้อความของพอร์ทัลและเซิร์ฟเวอร์ของ Hoard มีให้เลือก 31 ภาษา โดยเลือกได้ที่ การตั้งค่า → รูปลักษณ์ (ค่าเริ่มต้นคือภาษาของเบราว์เซอร์) คำแปลชุดแรกทำโดยมีเครื่องช่วย ยินดีรับการแก้ไขจากเจ้าของภาษา [docs/translations.md](../translations.md) อธิบายวิธีการไว้

## ชุมชน

มีคำถาม ไอเดีย หรืออยากอวดผลงาน? แวะมาทักทายกันได้ที่ [Discord ของ Azazel Labs](https://discord.gg/zf9ncS4EW) ส่วนบั๊กและคำขอฟีเจอร์ แจ้งเป็น [issue บน GitHub](https://github.com/Azazel-Labs/Hoard/issues) จะดีที่สุด จะได้ไม่ตกหล่น

## สัญญาอนุญาต

[MIT No Attribution](../../LICENSE) (MIT-0): ใช้ คัดลอก แก้ไข และแจกจ่าย Hoard ซ้ำได้เพื่อวัตถุประสงค์ใดก็ได้ โดยไม่จำเป็นต้องคงประกาศลิขสิทธิ์ไว้ แพ็กแอสเซ็ตที่คุณจัดเก็บใน Hoard ยังคงใช้สัญญาอนุญาตของตัวเอง
