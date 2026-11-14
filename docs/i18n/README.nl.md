# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · Nederlands · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Een assetbibliotheek voor gameontwikkeling die je zelf host. Zet er complete assetpakketten in (zips, tarballs,
mappen), en Hoard slaat elk bestand één keer op aan de hand van zijn hash, zoekt uit wat elk bestand is, tagt het,
koppelt meshes aan hun texturen en animaties, en maakt alles doorzoekbaar via een webportaal, een CLI, een
MCP-endpoint voor agents, en een `hoard sync` op basis van een lockfile die precies de bestanden ophaalt die je
gameproject nodig heeft.

Zie het als een assetstore zonder kassa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="De Hoard-bibliotheek: modellen en materialen in een raster, met een geopende schatkist in het 3D-voorbeeld naast de tags en downloads ervan" src="../img/hero-light.webp">
</picture>

<sub>De demobibliotheek bestaat uit CC0-art: modellen van [Poly Haven](https://polyhaven.com) (glTF en USD) en
oppervlakken van [ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` maakt deze afbeeldingen en
de social card.</sub>

## Installeren met Docker

Hoard wordt geleverd als één image voor amd64 en arm64: `ghcr.io/azazel-labs/hoard`. Sla dit op als `compose.yml`
(het is [deploy/compose.yml](../../deploy/compose.yml)):

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

Elk volume kan in plaats daarvan een map zijn op de schijf die er het best bij past: `/mnt/big/hoard:/library` zet je
bestanden op de grote schijf, terwijl de database klein en makkelijk te back-uppen blijft. Mappen die je mount, hebben
geen chown nodig: Hoard neemt `/config` en `/cache` over en draait als de eigenaar van `/library` (stel `PUID` en
`PGID` in om zelf een gebruiker te kiezen).

Start het daarna:

```sh
docker compose up -d        # open http://<host>:4627
```

Bij het eerste bezoek word je gevraagd een account aan te maken, waarna een korte installatie opent die vraagt waar je bibliotheek komt. Die stelt `/library` voor, het volume hierboven; kies
dat en zet een pakket neer in het portaal. Meer locaties volgen later, via **Instellingen → Bibliotheeklocaties**:

- **Een bibliotheek die Hoard zelf bewaart** krijgt kopieën van wat je uploadt of importeert. Bij elke bibliotheek
  vink je aan welke soorten bestanden ze aanneemt (audio op de ene schijf, modellen op de andere), en één is de
  opvanglocatie voor de rest. Wijzigen wat een locatie aanneemt, geldt voor nieuwe bestanden; **Opgeslagen bestanden
  mee verplaatsen** verplaatst wat al is opgeslagen. **Verwijderen** verplaatst de bestanden van een locatie eerst
  per soort naar de andere, en verwijdert de locatie zodra ze leeg is. Elk bestand wordt aan zijn hash getoetst
  voordat de oude kopie wordt verwijderd. Een locatie waarvan de schijf weg is, kan toch worden verwijderd; de
  bestanden ervan worden dan als ontbrekend getoond.
- **Een bewaakte map** is een map met pakketten die je al hebt. Elke map daarin wordt een pakket, geïndexeerd waar
  het staat. Hoard wijzigt, verplaatst of verwijdert daar nooit iets. Mount hem gerust alleen-lezen. Hoard doorzoekt
  hem bij het opstarten en daarna elke 15 minuten (of als je op **Nu scannen** drukt), en pikt nieuwe mappen op,
  bestanden die aan een pakket zijn toegevoegd of eruit zijn verwijderd, en pakketten waarvan de map weg is. Een map
  die plotseling leeg of onbereikbaar is, wordt gezien als een schijf die niet is gemount, en de pakketten ervan
  blijven bewaard. Een pakket waarvan de map ontbreekt, blijft drie dagen bewaard, met zijn tags en notities, voor
  het geval de map terugkomt. Wat je definitief uit een bewaakt pakket verwijdert, blijft verwijderd als de map
  opnieuw wordt gescand, ook al staan de bestanden er nog.

Een locatie is elke map die de server kan zien, dus mount hem in Docker eerst in de container.

**Bijwerken.** De bovenbalk toont een badge als er een nieuwere release is. Voer
`docker compose pull && docker compose up -d` uit om die binnen te halen; Hoard werkt bij het starten zijn database
bij en bewaart eerst een kopie in `/config/backups`. Wijzig de image-tag in `:0.8` om alleen patchreleases te krijgen.
Om automatisch bij te werken zonder dat Hoard midden in een upload herstart, geef je
**Instellingen → Onderhoud → Updates** een adres om aan te roepen: Hoard roept het aan zodra er een release uit is en
er niets gebeurt (de HTTP-API van Watchtower, een Portainer-webhook, een script; zie
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (optioneel).** Hoard rendert miniaturen van 3D-modellen op de GPU als het er een ziet, en in software als dat
niet zo is. De pakkettenpagina laat zien welke van de twee (`GPU: ...` of `No GPU found`). Om de GPU door te geven,
voeg je de overlay voor je hardware toe:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Beide overlays vind je in [deploy/](../../deploy/). Ze zetten ook de lokale modelserver (AI-tagging) op de GPU.
Bestanden op een NAS, S3-opslag, toegang op afstand via Cloudflare of Tailscale, en back-ups worden behandeld in
[docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Bouwen vanaf de broncode

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Of zet de zip neer in het portaal.

## De rest van deze README

Wat er met een pakket gebeurt, assets in een engine krijgen, Unreal-pakketten, zoeken, vermeldingen, MCP, opslag,
HTTPS, configuratie, beveiliging, accounts en de API worden beschreven in de [Engelse README](../../README.md).

## Vertalingen

Het portaal en de serverberichten van Hoard zijn er in 31 talen, te kiezen in Instellingen → Weergave (standaard de
taal van de browser). De eerste vertalingen zijn met hulp van machines gemaakt; correcties van moedertaalsprekers zijn
welkom. [docs/translations.md](../translations.md) legt uit hoe.

## Licentie

[MIT No Attribution](../../LICENSE) (MIT-0): gebruik, kopieer, wijzig en verspreid Hoard voor elk doel, zonder dat je
de copyrightvermelding hoeft te behouden. De assetpakketten die je in Hoard opslaat, houden hun eigen licenties.
