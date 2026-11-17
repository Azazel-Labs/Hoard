# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · Svenska · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Ett resursbibliotek för spelutveckling som du driftar själv. Släpp in hela resurspaket (zip-filer, tarballs, mappar), så
lagrar Hoard varje fil en gång utifrån dess hash, listar ut vad varje fil är, taggar den, kopplar mesher till deras
texturer och animationer, och gör allt sökbart från en webbportal, ett CLI, en MCP-slutpunkt för agenter och en
låsfilsbaserad `hoard sync` som hämtar exakt de filer som ditt spelprojekt behöver.

Se det som en resursbutik utan kassa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard-biblioteket: modeller och material i ett rutnät, med en öppen skattkista i 3D-förhandsvisningen bredvid dess taggar och nedladdningar" src="../img/hero-light.webp">
</picture>

<sub>Demobiblioteket består av CC0-grafik: modeller från [Poly Haven](https://polyhaven.com) (glTF och USD) och ytor från
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` skapar de här bilderna och delningsbilden.</sub>

## Installera med Docker

Hoard levereras som en enda avbild för amd64 och arm64: `ghcr.io/azazel-labs/hoard`. Spara det här som `compose.yml` (det är
[deploy/compose.yml](../../deploy/compose.yml)):

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

Varje volym kan i stället vara en mapp på den disk som passar den bäst: `/mnt/big/hoard:/library` lägger dina filer på den stora
disken, medan databasen förblir liten och lätt att säkerhetskopiera. Mappar som du monterar behöver ingen chown: Hoard tar över
`/config` och `/cache` och körs som den som äger `/library` (ange `PUID` och `PGID` för att välja en användare själv).

Starta det sedan:

```sh
docker compose up -d        # open http://<host>:4627
```

Första besöket ber dig skapa ett konto och öppnar sedan en kort konfiguration som frågar var ditt bibliotek ska finnas. Den föreslår `/library`, volymen ovan; välj
den och släpp ett paket på portalen. Fler platser kan läggas till senare, i **Inställningar → Biblioteksplatser**:

- **Ett bibliotek som Hoard förvaltar** får kopior av det du laddar upp eller importerar. Varje sådant kryssar i vilka typer av filer det tar
  (ljud på en disk, modeller på en annan), och ett av dem är uppsamlingsplats för resten. Att ändra vad en plats tar
  gäller nya filer; **Flytta lagrade filer så att de stämmer** flyttar det som redan har lagrats. **Ta bort** flyttar först en plats
  filer till de andra, efter typ, och tar bort den när den är tom. Varje fil kontrolleras mot sin hash innan
  den gamla kopian raderas. En plats vars disk har försvunnit kan tas bort ändå, och dess filer visas då som saknade.
- **En bevakad mapp** är en mapp med paket som du redan har. Varje mapp i den blir ett paket, indexerat där det
  ligger. Hoard ändrar, flyttar eller raderar aldrig något där. Montera den skrivskyddad om du vill. Hoard söker igenom den
  vid start och därefter var 15:e minut (eller när du trycker på **Sök igenom nu**) och plockar upp nya mappar, filer som lagts till i eller
  tagits bort från ett paket, och paket vars mapp har försvunnit. En mapp som plötsligt är tom eller inte kan nås antas vara en
  disk som inte är monterad, och dess paket behålls. Ett paket vars mapp saknas behålls i tre dagar, med sina
  taggar och anteckningar, ifall mappen kommer tillbaka. Det du raderar för gott från ett bevakat paket förblir raderat när
  mappen söks igenom igen, även om filerna fortfarande finns kvar där.

En plats är vilken mapp som helst som servern kan se, så i Docker monterar du först in den i containern.

**Uppdatera.** Sidhuvudet visar en markering när det finns en nyare version. Kör `docker compose pull && docker compose up -d`
för att hämta den; Hoard uppgraderar sin databas vid start och sparar först en kopia i `/config/backups`. Ändra avbildens tagg till
`:1.2` för att bara få patchversioner. För att uppdatera automatiskt utan att Hoard startas om mitt i en uppladdning ger du
**Inställningar → Underhåll → Uppdateringar** en adress att anropa: Hoard anropar den när en version har kommit ut och inget pågår
(Watchtowers HTTP-API, en Portainer-webhook, ett skript; se [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (valfritt).** Hoard renderar miniatyrbilder av 3D-modeller på GPU:n när det ser en, och i mjukvara när det inte gör det.
Paketsidan visar vilket (`GPU: ...` eller `No GPU found`). För att skicka igenom GPU:n lägger du till tillägget för din
hårdvara:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Hämta båda tilläggen från [deploy/](../../deploy/). De lägger också den lokala modellservern (AI-taggning) på GPU:n. Filer på en NAS, S3-lagring, fjärråtkomst via Cloudflare
eller Tailscale, och säkerhetskopior beskrivs i [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Bygg från källkod

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Eller släpp zip-filen på portalen.

## Resten av den här README-filen

Vad som händer med ett paket, att få in resurser i en spelmotor, Unreal-paket, sökning, krediteringar, MCP, lagring, HTTPS,
konfiguration, säkerhet, konton och API:t beskrivs i den [engelska README-filen](../../README.md).

## Översättningar

Hoards portal och servermeddelanden finns på 31 språk, som väljs i Inställningar → Utseende (webbläsarens språk som
standard). De första översättningarna gjordes med maskinhjälp; rättelser från personer med språket som modersmål är välkomna. [docs/translations.md](../translations.md) förklarar hur.

## Licens

[MIT No Attribution](../../LICENSE) (MIT-0): använd, kopiera, ändra och distribuera Hoard i vilket syfte som helst, utan att behöva
behålla upphovsrättsmeddelandet. Resurspaketen som du lagrar i Hoard behåller sina egna licenser.
