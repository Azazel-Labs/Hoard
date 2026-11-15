# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · Magyar · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Saját szerveren futó assetkönyvtár játékfejlesztéshez. Dobjon be teljes assetcsomagokat (zipeket, tarballokat,
mappákat), és a Hoard minden fájlt egyszer tárol a hash-e alapján, kideríti, mi az egyes fájl, felcímkézi, a hálókat
összekapcsolja a textúráikkal és animációikkal, és mindezt kereshetővé teszi egy webes portálon, egy CLI-n, egy
ügynököknek szóló MCP-végponton és egy lockfájl-alapú `hoard sync` paranccsal, amely pontosan azokat a fájlokat
tölti le, amelyekre a játékprojektjének szüksége van.

Olyan, mint egy assetáruház, csak pénztár nélkül.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="A Hoard könyvtára: modellek és anyagok rácsban, a 3D-előnézetben egy nyitott kincsesláda a címkéi és letöltései mellett" src="../img/hero-light.webp">
</picture>

<sub>A bemutatókönyvtár CC0 grafikákból áll: modellek a [Poly Havenről](https://polyhaven.com) (glTF és USD) és
felületek az [ambientCG-ről](https://ambientcg.com). A `node web/scripts/readme-images.mts` készíti ezeket a képeket
és a közösségimédia-kártyát.</sub>

## Telepítés Dockerrel

A Hoard egyetlen képfájlként érhető el amd64-re és arm64-re: `ghcr.io/azazel-labs/hoard`. Mentse ezt `compose.yml`
néven (ez a [deploy/compose.yml](../../deploy/compose.yml)):

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

Bármelyik kötet lehet helyette egy mappa azon a lemezen, amelyik illik hozzá: a `/mnt/big/hoard:/library` a nagy
lemezre teszi a fájljait, míg az adatbázis kicsi marad, és könnyen menthető. A csatolt mappákhoz nem kell chown: a
Hoard átveszi a `/config` és a `/cache` mappát, és a `/library` tulajdonosaként fut (a `PUID` és a `PGID` beállításával
maga is választhat felhasználót).

Ezután indítsa el:

```sh
docker compose up -d        # open http://<host>:4627
```

Az első látogatáskor létre kell hoznia egy fiókot, majd egy rövid beállítás következik, amely megkérdezi, hol legyen a könyvtára. A `/library` mappát javasolja, a fenti kötetet; válassza
ezt, és dobjon egy csomagot a portálra. További helyeket később adhat hozzá a **Beállítások → Tárolási helyek** oldalon:

- **A Hoard által kezelt könyvtár** másolatot kap arról, amit feltölt vagy importál. Mindegyiknél be van jelölve,
  milyen fájltípusokat fogad (a hangokat az egyik lemezre, a modelleket a másikra), és egy közülük a gyűjtőhely a
  többinek. Ha módosítja, mit fogad egy hely, az az új fájlokra vonatkozik; a **Tárolt fájlok áthelyezése** a már
  tároltakat is áthelyezi. Az **Eltávolítás** előbb típus szerint átviszi a hely fájljait a többire, és akkor
  távolítja el, amikor kiürült. A régi másolat törlése előtt minden fájl ellenőrződik a hash-e alapján. Az a hely,
  amelynek a lemeze eltűnt, így is eltávolítható, és a fájljai ezután hiányzóként jelennek meg.
- **A figyelt mappa** egy meglévő csomagokat tartalmazó mappa. A benne lévő minden mappa egy csomag lesz, amelyet a
  Hoard a helyén indexel. A Hoard soha semmit nem módosít, helyez át vagy töröl benne. Csatolhatja csak olvashatóként
  is. A Hoard indításkor, majd 15 percenként (vagy amikor a **Vizsgálat most** gombra kattint) átnézi, és észreveszi
  az új mappákat, a csomagokhoz hozzáadott vagy belőlük eltávolított fájlokat, valamint azokat a csomagokat, amelyek
  mappája eltűnt. Egy hirtelen üressé vagy elérhetetlenné vált mappát nem csatolt lemeznek tekint, és megtartja a
  csomagjait. Azt a csomagot, amelynek mappája hiányzik, három napig megtartja a címkéivel és jegyzeteivel együtt,
  arra az esetre, ha a mappa visszakerülne. Amit egy figyelt csomagból véglegesen töröl, az a mappa újravizsgálatakor
  is törölve marad, pedig a fájljai még ott vannak.

Hely bármely mappa lehet, amelyet a szerver lát, ezért Dockerben előbb csatolja a konténerbe.

**Frissítés.** A fejléc jelvényt mutat, ha újabb kiadás érhető el. A frissítéshez futtassa a
`docker compose pull && docker compose up -d` parancsot; a Hoard induláskor frissíti az adatbázisát, és előtte
másolatot ment róla a `/config/backups` mappába. Ha csak javítókiadásokat szeretne, változtassa a képcímkét `:0.8`
értékre. Ha automatikusan szeretne frissíteni anélkül, hogy a Hoard feltöltés közben újraindulna, adjon meg egy
meghívandó címet a **Beállítások → Karbantartás → Frissítések** alatt: a Hoard akkor hívja meg, amikor megjelent egy
kiadás, és semmi sincs folyamatban (a Watchtower HTTP API-ja, egy Portainer-webhook, egy szkript; lásd:
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (nem kötelező).** A Hoard a GPU-n rendereli a 3D-modellek bélyegképeit, ha lát egyet, és szoftveresen, ha nem.
A csomagok oldala megmutatja, melyik történik (`GPU: ...` vagy `No GPU found`). A GPU átadásához adja hozzá a
hardverének megfelelő kiegészítő compose-fájlt:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Mindkét kiegészítő fájl megtalálható itt: [deploy/](../../deploy/). Ezek a helyi modellszervert (MI-címkézés) is a GPU-ra teszik. A NAS-on tárolt fájlokról, az S3-tárolásról, a Cloudflare-en
vagy Tailscale-en keresztüli távoli elérésről és a biztonsági mentésekről a [docs/DEPLOYMENT.md](../DEPLOYMENT.md) szól.

## Fordítás forrásból

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Vagy dobja a zipet a portálra.

## A README többi része

Hogy mi történik egy csomaggal, hogyan kerülnek az assetek egy játékmotorba, valamint az Unreal-csomagok, a keresés,
a forrásmegjelölés, az MCP, a tárolás, a HTTPS, a konfiguráció, a biztonság, a fiókok és az API leírása az
[angol README-ben](../../README.md) található.

## Fordítások

A Hoard portálja és szerverüzenetei 31 nyelven érhetők el, a nyelv a Beállítások → Megjelenés alatt választható
(alapértelmezés szerint a böngésző nyelve). Az első fordítások gépi segítséggel készültek; az anyanyelvi beszélők
javításait szívesen fogadjuk. Hogy hogyan, azt a [docs/translations.md](../translations.md) írja le.

## Licenc

[MIT No Attribution](../../LICENSE) (MIT-0): a Hoard bármilyen célra használható, másolható, módosítható és
továbbterjeszthető, a szerzői jogi megjegyzés megtartása nélkül is. A Hoardban tárolt assetcsomagokra a saját
licencük vonatkozik.
