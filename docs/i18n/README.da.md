# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · Dansk · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Et selvhostet asset-bibliotek til spiludvikling. Smid hele asset-pakker ind (zip-filer, tarballs, mapper), så
gemmer Hoard hver fil én gang efter dens hash, finder ud af, hvad hver fil er, tagger den, kobler meshes til deres
teksturer og animationer og gør det hele søgbart fra en webportal, en CLI, et MCP-endpoint til agenter og en
lockfil-baseret `hoard sync`, der henter præcis de filer, dit spilprojekt har brug for.

Tænk på det som en asset-butik uden kasse.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard-biblioteket: modeller og materialer i et gitter, med en åben skattekiste i 3D-forhåndsvisningen ved siden af dens tags og downloads" src="../img/hero-light.webp">
</picture>

<sub>Demobiblioteket er CC0-grafik: modeller fra [Poly Haven](https://polyhaven.com) (glTF og USD) og overflader fra
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` laver disse billeder og det sociale kort.</sub>

## Installation med Docker

Hoard leveres som ét image til amd64 og arm64: `ghcr.io/azazel-labs/hoard`. Gem dette som `compose.yml` (det er
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

Hvert volume kan i stedet være en mappe på den disk, der passer bedst: `/mnt/big/hoard:/library` lægger dine filer på
den store disk, mens databasen forbliver lille og nem at sikkerhedskopiere. Mapper, du monterer, kræver ingen chown:
Hoard overtager `/config` og `/cache` og kører som den, der ejer `/library` (angiv `PUID` og `PGID` for selv at vælge
en bruger).

Start den derefter:

```sh
docker compose up -d        # open http://<host>:4627
```

Ved første besøg bliver du bedt om at oprette en konto, og derefter åbner en kort opsætning, der spørger, hvor dit bibliotek skal ligge. Den foreslår `/library`, volumet ovenfor; vælg
det, og slip en pakke på portalen. Flere placeringer kan tilføjes senere under **Indstillinger → Biblioteksplaceringer**:

- **Et bibliotek, som Hoard opbevarer**, får kopier af det, du uploader eller importerer. Hver af dem afkrydser de
  filtyper, den tager imod (lyd på én disk, modeller på en anden), og én er opsamlingssted for resten. Ændrer du, hvad
  en placering tager imod, gælder det nye filer; **Flyt gemte filer, så de passer** flytter det, der allerede er
  gemt. **Fjern** flytter først en placerings filer til de andre efter type og fjerner den, når den er tom. Hver fil
  kontrolleres mod sin hash, før den gamle kopi slettes. En placering, hvis disk er væk, kan fjernes alligevel, og dens
  filer vises så som manglende.
- **En overvåget mappe** er en mappe med pakker, du allerede har. Hver mappe i den bliver til en pakke, der indekseres,
  hvor den ligger. Hoard ændrer, flytter eller sletter aldrig noget der. Montér den skrivebeskyttet, hvis du vil. Hoard
  gennemgår den ved opstart og derefter hvert 15. minut (eller når du trykker på **Scan nu**) og opfanger nye mapper,
  filer, der er føjet til eller fjernet fra en pakke, og pakker, hvis mappe er væk. En mappe, der pludselig er tom
  eller ikke kan nås, opfattes som en disk, der ikke er monteret, og dens pakker beholdes. En pakke, hvis mappe
  mangler, beholdes i tre dage med sine tags og noter, hvis nu mappen kommer tilbage. Alt, hvad du sletter for altid
  fra en overvåget pakke, forbliver slettet, når mappen scannes igen, selvom filerne stadig ligger der.

En placering er en hvilken som helst mappe, serveren kan se, så i Docker skal du først montere den i containeren.

**Opdatering.** Topbjælken viser et mærke, når der findes en nyere udgivelse. Kør `docker compose pull && docker compose up -d`
for at hente den; Hoard opgraderer sin database ved start og gemmer først en kopi i `/config/backups`. Skift image-tagget til
`:1.2` for kun at få patch-udgivelser. For at opdatere automatisk uden at genstarte Hoard midt i en upload kan du give
**Indstillinger → Vedligeholdelse → Opdateringer** en adresse at kalde: Hoard kalder den, når en udgivelse er ude, og der ikke sker noget
(Watchtowers HTTP-API, en Portainer-webhook, et script; se [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (valgfri).** Hoard renderer miniaturer af 3D-modeller på GPU'en, når den kan se en, og i software, når den ikke kan.
Pakkesiden viser hvilken (`GPU: ...` eller `No GPU found`). For at give containeren adgang til GPU'en skal du tilføje
overlayet til din hardware:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Hent begge overlays fra [deploy/](../../deploy/). De lægger også den lokale modelserver (AI-tagging) på GPU'en. Filer på en NAS, S3-lager, fjernadgang via Cloudflare
eller Tailscale og sikkerhedskopier er beskrevet i [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Byg fra kildekoden

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Eller slip zip-filen på portalen.

## Resten af denne README

Hvad der sker med en pakke, hvordan du får assets ind i en motor, Unreal-pakker, søgning, krediteringer, MCP, lagring,
HTTPS, konfiguration, sikkerhed, konti og API'et er beskrevet i den [engelske README](../../README.md).

## Oversættelser

Hoards portal og servermeddelelser findes på 31 sprog og vælges under Indstillinger → Udseende (som standard browserens
sprog). De første oversættelser blev lavet med hjælp fra maskinoversættelse; rettelser fra modersmålstalende er velkomne. [docs/translations.md](../translations.md) forklarer hvordan.

## Licens

[MIT No Attribution](../../LICENSE) (MIT-0): brug, kopiér, ændr og videredistribuér Hoard til ethvert formål uden at
skulle bevare copyrightmeddelelsen. De asset-pakker, du gemmer i Hoard, beholder deres egne licenser.
