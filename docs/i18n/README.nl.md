# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · Nederlands · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Elk assetpakket dat je hebt, doorzoekbaar, te bekijken en één klik van je game verwijderd.</h3>

Hoard is een assetbibliotheek voor games die je zelf host. Zet er een zip, een Unity-pakket, een Fab-download of een
map met tienduizend bestanden in, en Hoard gaat aan de slag. Modellen draaien in 3D met hun texturen, losse
texturemaps worden materialen, geluiden tekenen hun golfvorm, spritesheets spelen hun animaties af, en alles krijgt
tags, zodat je het vindt door `footsteps gravel` of `rigged low-poly` te typen. Daarna belandt het in Godot, Unity of
Unreal, met alles wat het nodig heeft.

Geen abonnement, geen upload naar andermans cloud, geen prijs per gebruiker: één Docker-image op je eigen server, of een
app op je eigen computer, en het is van jou. Zie het als een assetstore zonder kassa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="De Hoard-bibliotheek: 3D-modellen in een raster, met een geopende schatkist in het 3D-voorbeeld naast de tags en downloads ervan" src="../img/hero-light.webp">
</picture>

### Bekijk alles voordat je het gebruikt

Modellen openen in 3D met hun texturen, materialen en animaties. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL en voxels
leest Hoard zelf, zonder dat er een 3D-programma geïnstalleerd hoeft te zijn; Blender-bestanden komen binnen via
headless Blender, en Unreal-pakketten laten zien wat de editor erover heeft opgeslagen, zonder Unreal. Miniaturen
worden op de server getekend, elke keer op dezelfde manier in beeld gebracht, zodat je een heel pakket in één
oogopslag doorziet.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Een kroonluchter op een volledige pagina: een grote 3D-weergave naast de gegevens, tags, gekoppelde texturen en downloads ervan" src="../img/page-light.webp">
</picture>

### Losse texturemaps worden materialen

`Rock_BaseColor.png`, `Rock_Normal.png` en `Rock_Roughness.png` worden één Rock-materiaal op een belichte bol. Hoard
kent de naamgeving van Megascans, Poly Haven, ambientCG, Substance en Unreal, gecombineerde maps inbegrepen, en ook
Unity-`.mat`-bestanden. De schermvullende viewer toont een materiaal op een bol, kubus of vlak, op ware grootte, en
elke map kan los aan en uit.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Een bakstenen materiaal in de schermvullende viewer, op een bol, met schakelaars voor de kleur-, normal-, roughness- en hoogtemap ervan" src="../img/material-light.webp">
</picture>

### Spritesheets die afspelen

Atlassen uit TexturePacker, Starling, libGDX, Cocos2d en Godot worden één spritesheet die zijn animaties afspeelt en
zijn sprites opsomt. Download hem opnieuw verpakt voor welke engine je ook gebruikt: tien formaten, die elk alleen
krijgen wat ze kunnen bevatten.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Een spritesheet van een personage op een volledige pagina, met een van de animaties die afspeelt naast de tags en downloadformaten ervan" src="../img/sheet-light.webp">
</picture>

### Geluiden die je kunt zien

Elk geluid krijgt een golfvorm die gekleurd is naar hoe helder het klinkt, zodat een scherpe klap en een lange
ambience er anders uitzien voordat je ze afspeelt. Metadata van geluidsbibliotheken wordt gelezen (UCS-categorieën,
velden van Soundminer en BaseHead, looppunten, tempo, grondtoon), en MIDI- en trackermuziek wordt gerenderd zodat die
afspeelt zoals elk ander geluid.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Een raster van geluidseffecten getekend als gekleurde golfvormen, met één geopend dat zijn speler, tags en takes toont" src="../img/sound-light.webp">
</picture>

### Zoeken dat weet wat dingen zijn

Hoard tagt alles zelf, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`, en een
categorie uit een boom voor game-assets. Zoek op woorden, op kleur of op wat een afbeelding laat zien, en vraag bij
elke textuur of elk geluid om “Meer zoals dit”. Wijs het een OpenAI-compatibel model aan, lokaal of gehost, en het
schrijft ook namen en beschrijvingen.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Een zoekopdracht naar hout: een houten tafel, schors, planken en een houtoppervlak bij elkaar, met filters op soort en tag ernaast" src="../img/search-light.webp">
</picture>

### Rechtstreeks in je engine

- **Engine-plugins** voor Godot 4, Unity, Unreal Engine 5 en Lumina: blader door de bibliotheek in de editor en druk op
  Toevoegen aan project. Het model komt mee met zijn texturen, op de juiste plek, geïmporteerd.
- **`hoard sync`** haalt precies de bestanden binnen die een project noemt, vastgelegd in een lockfile, zodat elke
  teamgenoot en elke buildmachine dezelfde bytes krijgt.
- **Downloaden als** GLB, PNG, WAV, MP3 of een spritesheetformaat, ter plekke geconverteerd, met texturenamen volgens
  de conventie van je studio.
- **AI-agents** zoeken, inspecteren en halen assets op via MCP.

### Eén keer opgeslagen, veilig bewaard

Elk bestand wordt één keer opgeslagen aan de hand van zijn BLAKE3-hash, zodat dezelfde textuur in vijf bundels de
ruimte van één inneemt, en beschadiging niet onopgemerkt kan blijven. Bewaar het op een lokale schijf of in een
willekeurige S3-bucket. Mappen waar je Hoard op richt worden gelezen, nooit gewijzigd. Accounts, alleen-lezentokens,
single sign-on, HTTPS en HTTP/3 zijn ingebouwd.

### Of gewoon op je computer

Draai je geen server? De desktop-app voor Windows, macOS en Linux is dezelfde bibliotheek in een eigen venster.
Installeer hem, kies een map en hij werkt offline: geen accounts, wachtwoorden of serverinstellingen, niets wat de rest
van het netwerk kan bereiken, en de sleutels die je invoert bewaard in de sleutelhanger van het systeem. Nieuwe versies
worden op de achtergrond gedownload en geïnstalleerd wanneer je de app afsluit.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="De Hoard-desktop-app in een Windows-venster: de modellen uit de bibliotheek in een raster, met een lantaarn geopend in het 3D-voorbeeld naast de tags en downloads ervan" src="../img/desktop-light.webp">
</picture>

<sub>De demobibliotheek bestaat uit CC0-art: modellen van [Poly Haven](https://polyhaven.com) (glTF en USD),
oppervlakken van [ambientCG](https://ambientcg.com), en geluiden, spritesheets en lettertypen van
[Kenney](https://kenney.nl). `node web/scripts/readme-images.mts` maakt deze afbeeldingen en de social card.</sub>

## De desktop-app installeren

Download hem van de [nieuwste release](https://github.com/Azazel-Labs/Hoard/releases/latest): de `-setup.exe` voor
Windows, de `.dmg` voor macOS op een Mac met Apple-chip, de `.AppImage` of `.deb` voor Linux. Op Windows installeert hij
alleen voor jou, zonder om beheerdersrechten te vragen. De installatieprogramma's zijn nog niet door Apple of Microsoft
ondertekend, dus Windows vraagt het eerst voordat het installatieprogramma wordt uitgevoerd (Meer informatie → Toch
uitvoeren) en macOS voordat de app de eerste keer opent (Systeeminstellingen → Privacy en beveiliging → Open toch).

De eerste keer dat hij opent, vraagt hij waar de bibliotheek moet komen. Al het andere staat in Instellingen, net als op
een server.

Daarna houdt hij zichzelf up-to-date, zonder dat je ergens op hoeft te klikken: een nieuwe release wordt op de
achtergrond gedownload en geïnstalleerd zodra je hem afsluit, dus de volgende keer dat je Hoard opent, is het de nieuwe
versie. Een `.deb` werk je bij door de volgende te installeren.

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
bij en bewaart eerst een kopie in `/config/backups`. Wijzig de image-tag in `:1.2` om alleen patchreleases te krijgen.
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
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
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
HTTPS, configuratie, beveiliging, accounts, de API en feiten en cijfers worden beschreven in de
[Engelse README](../../README.md).

## Vertalingen

Het portaal en de serverberichten van Hoard zijn er in 31 talen, te kiezen in Instellingen → Weergave (standaard de
taal van de browser). De eerste vertalingen zijn met hulp van machines gemaakt; correcties van moedertaalsprekers zijn
welkom. [docs/translations.md](../translations.md) legt uit hoe.

## Licentie

[MIT No Attribution](../../LICENSE) (MIT-0): gebruik, kopieer, wijzig en verspreid Hoard voor elk doel, zonder dat je
de copyrightvermelding hoeft te behouden. De assetpakketten die je in Hoard opslaat, houden hun eigen licenties.
