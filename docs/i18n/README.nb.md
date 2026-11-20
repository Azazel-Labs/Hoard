<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · Norsk bokmål · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

</div>

<h3 align="center">Hver ressurspakke du eier, søkbar, med forhåndsvisning og ett klikk unna spillet ditt.</h3>

Hoard er et selvhostet bibliotek for spillressurser. Slipp inn en zip-fil, en Unity-pakke, en Fab-nedlasting eller en
mappe med ti tusen filer, så setter Hoard i gang. Modeller roterer i 3D med teksturene sine, løse teksturkart blir
materialer, lyder tegner bølgeformene sine, spriteark spiller animasjonene sine, og alt blir tagget, så du finner det
ved å skrive `footsteps gravel` eller `rigged low-poly`. Deretter havner det i Godot, Unity eller Unreal med alt det
trenger.

Ingen abonnement, ingen opplasting til noens sky, ingen pris per bruker: ett Docker-image på din egen server, eller en
app på din egen datamaskin, og det er ditt. Tenk på det som en ressursbutikk uten kasse.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard-biblioteket: 3D-modeller i et rutenett, med en åpen skattekiste i 3D-forhåndsvisningen ved siden av taggene og nedlastingene" src="../img/hero-light.webp">
</picture>

### Se alt før du bruker det

Modeller åpnes i 3D med teksturer, materialer og animasjoner. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL og voksler leses
av Hoard selv, uten at noe 3D-program er installert; Blender-filer kommer inn via Blender uten grensesnitt, og
Unreal-pakker viser hva editoren lagret om dem, uten Unreal. Miniatyrbilder tegnes på serveren, beskåret likt hver gang,
så en hel pakke kan skummes med ett blikk.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="En lysekrone vist på hel side: en stor 3D-visning ved siden av fakta, tagger, koblede teksturer og nedlastinger" src="../img/page-light.webp">
</picture>

### Løse teksturkart blir materialer

`Rock_BaseColor.png`, `Rock_Normal.png` og `Rock_Roughness.png` blir ett Rock-materiale på en opplyst kule. Hoard
forstår navngivingen til Megascans, Poly Haven, ambientCG, Substance og Unreal, også pakkede kart, og leser Unitys
`.mat`-filer også. Fullskjermvisningen viser et materiale på en kule, kube eller flate, i virkelig skala, og hvert kart
kan slås av og på.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Et mursteinsmateriale i fullskjermvisningen, på en kule, med brytere for farge-, normal-, ruhets- og høydekartene" src="../img/material-light.webp">
</picture>

### Spriteark som spiller

Atlas fra TexturePacker, Starling, libGDX, Cocos2d og Godot blir ett spriteark som spiller animasjonene sine og lister
opp spritene sine. Last det ned pakket om for den motoren du bruker: ti formater, og hvert får bare det det kan romme.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Et spriteark for en figur vist på hel side, med én av animasjonene som spilles ved siden av taggene og nedlastingsformatene" src="../img/sheet-light.webp">
</picture>

### Lyder du kan se

Hver lyd får en bølgeform farget etter hvor lys den låter, så et skarpt slag og en lang ambiens ser forskjellige ut før
du spiller dem av. Metadata fra lydbiblioteker leses (UCS-kategorier, Soundminer- og BaseHead-felt, looppunkter, tempo,
grunntone), og MIDI- og trackermusikk rendres så den spilles som hvilken som helst annen lyd.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Et rutenett av lydeffekter tegnet som fargede bølgeformer, med én åpen som viser avspilleren, taggene og opptakene" src="../img/sound-light.webp">
</picture>

### Søk som vet hva ting er

Hoard tagger alt selv, uten nett: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0` og en
kategori fra et tre for spillressurser. Søk med ord, med farge eller etter hva et bilde viser, og be enhver tekstur
eller lyd om «Mer som dette». Pek det mot en hvilken som helst OpenAI-kompatibel modell, lokal eller i skyen, så skriver
det navn og beskrivelser også.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Et søk etter wood: et trebord, bark, planker og en treoverflate sammen, med filtre etter type og tagg ved siden av" src="../img/search-light.webp">
</picture>

### Rett inn i motoren din

- **Motorutvidelser** for Godot 4, Unity, Unreal Engine 5 og Lumina: bla gjennom biblioteket inne i editoren og trykk Legg til i
  prosjekt. Modellen kommer med teksturene sine, på riktig sted, importert.
- **`hoard sync`** henter nøyaktig de filene et prosjekt navngir inn i prosjektet, låst i en låsefil, så hver lagkamerat
  og byggemaskin får de samme bytene.
- **Last ned som** GLB, PNG, WAV, MP3 eller et spritearkformat, konvertert fortløpende, med teksturnavn etter studioets
  konvensjon.
- **AI-agenter** søker i, undersøker og henter ressurser over MCP.

### Lagret én gang, holdt trygt

Hver fil lagres én gang etter BLAKE3-hashen sin, så den samme teksturen i fem pakker tar plassen til én, og korrupsjon
kan ikke gå ubemerket hen. Ha det på lokal disk eller i en hvilken som helst S3-bøtte. Mapper du peker Hoard mot, leses,
men endres aldri. Kontoer, skrivebeskyttede tokener, enkel pålogging, HTTPS og HTTP/3 er innebygd.

### Eller bare på datamaskinen din

Kjører du ikke en server? Skrivebordsappen for Windows, macOS og Linux er det samme biblioteket i et eget vindu.
Installer den, velg en mappe, og den virker uten nett: ingen kontoer, passord eller serverinnstillinger, ingenting
resten av nettverket kan nå, og nøklene du skriver inn lagres i systemets nøkkelring. Nye versjoner lastes ned i
bakgrunnen og installeres når du avslutter.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Hoard-skrivebordsappen i et Windows-vindu: bibliotekets modeller i et rutenett, med en lykt åpen i 3D-forhåndsvisningen ved siden av taggene og nedlastingene" src="../img/desktop-light.webp">
</picture>

<sub>Demobiblioteket er CC0-kunst: modeller fra [Poly Haven](https://polyhaven.com) (glTF og USD), overflater fra
[ambientCG](https://ambientcg.com), og lyder, spriteark og fonter fra [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` lager disse bildene og delingskortet.</sub>

## Installere skrivebordsappen

Last den ned fra [siste versjon](https://github.com/Azazel-Labs/Hoard/releases/latest): `-setup.exe` for Windows, `.dmg`
for macOS på Mac med Apple-chip, `.AppImage` eller `.deb` for Linux. På Windows installeres den bare for deg, uten å be
om administratortilgang. Installasjonsprogrammene er ikke signert av Apple eller Microsoft ennå, så Windows spør før det
kjører installasjonsprogrammet (Mer informasjon → Kjør likevel), og macOS før appen åpnes første gang
(Systeminnstillinger → Personvern og sikkerhet → Åpne likevel).

Første gang den åpnes, spør den hvor biblioteket skal ligge. Alt annet ligger i Innstillinger, som på en server.

Etter det holder den seg oppdatert av seg selv, uten at du trenger å klikke på noe: en ny versjon lastes ned i
bakgrunnen og installeres når du avslutter, så neste gang du åpner Hoard, er det den nye versjonen. En `.deb` oppdateres
ved å installere den neste.

## Installere med Docker

Hoard leveres som ett image for amd64 og arm64: `ghcr.io/azazel-labs/hoard`. Lagre dette som `compose.yml` (det er
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

Hvert volum kan i stedet være en mappe på den disken som passer best: `/mnt/big/hoard:/library` legger filene dine på den
store disken, mens databasen forblir liten og enkel å sikkerhetskopiere. Mapper du monterer, trenger ingen chown: Hoard tar over
`/config` og `/cache` og kjører som den som eier `/library` (sett `PUID` og `PGID` for å velge en bruker selv).

Start den deretter:

```sh
docker compose up -d        # open http://<host>:4627
```

Det første besøket ber deg lage en konto, og åpner deretter et kort oppsett som spør hvor biblioteket ditt skal ligge. Det foreslår `/library`, volumet ovenfor; velg
det og slipp en pakke på portalen. Flere plasseringer kommer senere, fra **Innstillinger → Biblioteksplasseringer**:

- **Et bibliotek som Hoard holder** får kopier av det du laster opp eller importerer. Hvert av dem krysser av for filtypene det tar imot
  (lyd på én disk, modeller på en annen), og ett er oppsamlingsplasseringen for resten. Endringer i hva en plassering tar imot,
  gjelder nye filer; **Flytt lagrede filer så de stemmer** flytter det som allerede er lagret. **Fjern** flytter først filene til en plassering
  til de andre, etter type, og fjerner den når den er tom. Hver fil sjekkes mot hashen sin før
  den gamle kopien slettes. En plassering der disken er borte, kan fjernes likevel, og filene vises da som manglende.
- **En overvåket mappe** er en mappe med pakker du allerede har. Hver mappe inni den blir en pakke, indeksert der den
  ligger. Hoard endrer, flytter eller sletter aldri noe der. Monter den skrivebeskyttet hvis du vil. Hoard går gjennom den
  ved oppstart og hvert 15. minutt etterpå (eller når du trykker **Skann nå**), og fanger opp nye mapper, filer som er lagt til i eller
  fjernet fra en pakke, og pakker der mappen er borte. En mappe som plutselig er tom eller utilgjengelig, antas å være en
  disk som ikke er montert, og pakkene i den beholdes. En pakke der mappen mangler, beholdes i tre dager, med
  tagger og notater, i tilfelle mappen kommer tilbake. Alt du sletter for godt fra en overvåket pakke, forblir slettet når
  mappen skannes på nytt, selv om filene fortsatt ligger der.

En plassering er hvilken som helst mappe serveren kan se, så i Docker må du først montere den inn i containeren.

**Oppdatering.** Topplinjen viser et merke når en nyere versjon finnes. Kjør `docker compose pull && docker compose up -d`
for å ta den i bruk; Hoard oppgraderer databasen ved oppstart og lagrer først en kopi i `/config/backups`. Endre image-taggen til
`:1.3` for bare å få feilrettingsversjoner. For å oppdatere automatisk uten at Hoard startes på nytt midt i en opplasting, gi
**Innstillinger → Vedlikehold → Oppdateringer** en adresse å kalle: Hoard kaller den når en ny versjon er ute og ingenting pågår
(Watchtowers HTTP-API, en Portainer-webhook, et skript; se [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (valgfritt).** Hoard rendrer miniatyrbilder av 3D-modeller på GPU-en når den ser en, og i programvare når den ikke gjør det.
Pakkesiden viser hvilken (`GPU: ...` eller `No GPU found`). For å sende GPU-en gjennom legger du til overlegget for
maskinvaren din:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Hent begge overleggene fra [deploy/](../../deploy/). De legger også den lokale modellserveren (AI-tagging) på GPU-en. Filer på en NAS, S3-lagring, fjerntilgang via Cloudflare
eller Tailscale, og sikkerhetskopier er beskrevet i [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Bygge fra kildekoden

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Eller slipp zip-filen på portalen.

## Resten av denne README-en

Hva som skjer med en pakke, hvordan du får ressurser inn i en motor, Unreal-pakker, søk, kreditering, MCP, lagring, HTTPS,
konfigurasjon, sikkerhet, kontoer, API-et samt fakta og tall er beskrevet i den [engelske README-en](../../README.md).

## Oversettelser

Hoards portal og servermeldinger finnes på 31 språk, og velges under Innstillinger → Utseende (nettleserens språk som
standard). De første oversettelsene ble laget med maskinhjelp; rettelser fra morsmålsbrukere er velkomne. [docs/translations.md](../translations.md) forklarer hvordan.

## Lisens

[MIT No Attribution](../../LICENSE) (MIT-0): bruk, kopier, endre og videredistribuer Hoard til et hvilket som helst formål, uten at
opphavsrettsmerknaden må beholdes. Ressurspakkene du lagrer i Hoard, beholder sine egne lisenser.
