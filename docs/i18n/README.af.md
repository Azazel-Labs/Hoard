<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· Afrikaans&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">Elke batepak wat jy besit: deursoekbaar, met 'n voorskou, en net een klik van jou speletjie af.</h3>

Hoard is 'n selfgehuisveste biblioteek vir speletjiebates. Los 'n zip, 'n Unity-pakket, 'n Fab-aflaai of 'n vouer met
tienduisend lêers in, en Hoard spring aan die werk. Modelle draai in 3D met hul teksture, los tekstuurkaarte word
materiale, klanke teken hul golfvorms, sprite-blaaie speel hul animasies, en alles kry etikette, sodat jy dit vind deur
`footsteps gravel` of `rigged low-poly` te tik. Dan beland dit in Godot, Unity of Unreal met alles wat dit nodig het.

Geen intekening nie, geen oplaai na iemand se wolk nie, geen pryse per gebruiker nie: een Docker-beeld op jou eie
bediener, of 'n toep op jou eie rekenaar, en dit is joune. Dink daaraan as 'n batewinkel sonder 'n betaalpunt.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Die Hoard-biblioteek: 3D-modelle in 'n rooster, met 'n oop skatkis in die 3D-voorskou langs sy etikette en aflaaie" src="../img/hero-light.webp">
</picture>

### Sien alles voordat jy dit gebruik

Modelle maak in 3D oop met hul teksture, materiale en animasies. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL en voxels word
deur Hoard self gelees, sonder dat enige 3D-toepassing geïnstalleer hoef te wees; Blender-lêers kom deur 'n koplose
Blender in, en Unreal-pakke wys wat die redigeerder oor hulle gestoor het, sonder Unreal. Duimnaelskete word op die
bediener geteken, elke keer op dieselfde manier geraam, sodat jy 'n hele pak in een oogopslag kan deurblaai.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="'n Kandelaar op 'n volle bladsy: 'n groot 3D-aansig langs sy feite, etikette, gekoppelde teksture en aflaaie" src="../img/page-light.webp">
</picture>

### Los tekstuurkaarte word materiale

`Rock_BaseColor.png`, `Rock_Normal.png` en `Rock_Roughness.png` word een Rock-materiaal op 'n beligte sfeer. Hoard lees
die naamgewing van Megascans, Poly Haven, ambientCG, Substance en Unreal, gepakte kaarte ingesluit, en ook Unity se
`.mat`-lêers. Die volskermkyker wys 'n materiaal op 'n sfeer, kubus of vlak, op sy werklike skaal, en elke kaart kan
aan- en afgeskakel word.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="'n Baksteenmateriaal in die volskermkyker, op 'n sfeer, met skakelaars vir sy kleur-, normaal-, grofheid- en hoogtekaarte" src="../img/material-light.webp">
</picture>

### Sprite-blaaie wat speel

Atlasse van TexturePacker, Starling, libGDX, Cocos2d en Godot word een sprite-blad wat sy animasies speel en sy sprites
lys. Laai dit herverpak af vir watter enjin jy ook al gebruik: tien formate, en elkeen kry net wat dit kan hou.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="'n Karakter se sprite-blad op 'n volle bladsy, met een van sy animasies wat langs sy etikette en aflaaiformate speel" src="../img/sheet-light.webp">
</picture>

### Klanke wat jy kan sien

Elke klank kry 'n golfvorm wat gekleur is volgens hoe helder dit klink, sodat 'n skerp slag en 'n lang agtergrondklank
anders lyk voordat jy hulle speel. Klankbiblioteek-metadata word gelees (UCS-kategorieë, Soundminer- en BaseHead-velde,
luspunte, tempo, grondnoot), en MIDI- en tracker-musiek word weergegee sodat dit soos enige ander klank speel.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="'n Rooster klankeffekte wat as gekleurde golfvorms geteken is, met een oop wat sy speler, etikette en opnames wys" src="../img/sound-light.webp">
</picture>

### Soektog wat weet wat dinge is

Hoard etiketteer alles self, vanlyn: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`, en 'n
kategorie uit 'n boom vir speletjiebates. Soek met woorde, volgens kleur of volgens wat 'n prent wys, en vra enige
tekstuur of klank vir “Meer soos hierdie”. Koppel dit aan enige OpenAI-versoenbare model, plaaslik of aanlyn, en dit
skryf ook name en beskrywings.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="'n Soektog na “wood”: 'n houttafel, bas, planke en 'n houtoppervlak saam, met filters volgens soort en etiket langsaan" src="../img/search-light.webp">
</picture>

### Reguit in jou enjin

- **Enjin-inproppe** vir Godot 4, Unity, Unreal Engine 5 en Lumina: blaai deur die biblioteek binne die redigeerder en druk Voeg
  by projek. Die model kom met sy teksture, op die regte plek, ingevoer.
- **`hoard sync`** trek presies die lêers wat 'n projek noem daarin in, vasgepen in 'n lockfile, sodat elke spanmaat en
  elke boumasjien dieselfde grepe kry.
- **Laai af as** GLB, PNG, WAV, MP3 of 'n sprite-bladformaat, terstond omgeskakel, met tekstuurname volgens jou ateljee
  se konvensie.
- **KI-agente** soek, ondersoek en haal bates oor MCP.

### Een keer gestoor, veilig bewaar

Elke lêer word een keer volgens sy BLAKE3-hash gestoor, sodat dieselfde tekstuur in vyf bundels net die spasie van een
opneem, en korrupsie nie ongemerk kan bly nie. Hou dit op 'n plaaslike skyf of in enige S3-houer. Vouers waarna jy Hoard
wys, word gelees, nooit verander nie. Rekeninge, leesalleen-tokens, enkelaanmelding, HTTPS en HTTP/3 is ingebou.

### Of net op jou rekenaar

Bestuur jy nie 'n bediener nie? Die rekenaartoep vir Windows, macOS en Linux is dieselfde biblioteek in 'n eie venster.
Installeer dit, kies 'n vouer, en dit werk vanlyn: geen rekeninge, wagwoorde of bedienerinstellings nie, niks wat die
res van die netwerk kan bereik nie, en die sleutels wat jy intik word in die stelsel se sleutelring bewaar. Nuwe
weergawes laai in die agtergrond af en word geïnstalleer wanneer dit toemaak.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Die Hoard-rekenaartoep in 'n Windows-venster: die biblioteek se modelle in 'n rooster, met 'n lantern oop in die 3D-voorskou langs sy etikette en aflaaie" src="../img/desktop-light.webp">
</picture>

<sub>Die demobiblioteek is CC0-kuns: modelle van [Poly Haven](https://polyhaven.com) (glTF en USD), oppervlakke van
[ambientCG](https://ambientcg.com), en klanke, sprite-blaaie en lettertipes van [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` maak hierdie prente en die sosiale kaart.</sub>

## Installeer die rekenaartoep

Laai dit af van die [nuutste vrystelling](https://github.com/Azazel-Labs/Hoard/releases/latest): die `-setup.exe` vir
Windows, die `.dmg` vir macOS op Apple silicon, die `.AppImage` of `.deb` vir Linux. Op Windows installeer dit net vir
jou, sonder om 'n administrateur te vra. Die installeerders is nog nie deur Apple of Microsoft onderteken nie, so
Windows vra voordat dit die installeerder laat loop (More info → Run anyway) en macOS voordat dit die toep die eerste
keer oopmaak (System Settings → Privacy & Security → Open Anyway).

Die eerste keer dat dit oopmaak, vra dit waar die biblioteek moet kom. Alles anders is in Instellings, soos op 'n
bediener.

Daarna hou dit homself op datum, sonder dat jy iets hoef te klik: 'n nuwe vrystelling word in die agtergrond afgelaai en
geïnstalleer wanneer jy die toep toemaak, so die volgende keer dat jy Hoard oopmaak, is dit die nuwe weergawe. 'n `.deb`
word bygewerk deur die volgende een te installeer.

## Installeer met Docker

Hoard kom as een beeld vir amd64 en arm64: `ghcr.io/azazel-labs/hoard`. Stoor dit as `compose.yml` (dit is
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

Elke volume kan eerder 'n vouer wees op watter skyf ook al daarby pas: `/mnt/big/hoard:/library` sit jou lêers op die
groot skyf, terwyl die databasis klein en maklik bly om te rugsteun. Vouers wat jy koppel, het geen chown nodig nie:
Hoard neem `/config` en `/cache` oor en loop as wie ook al `/library` besit (stel `PUID` en `PGID` om self 'n gebruiker
te kies).

Begin dit dan:

```sh
docker compose up -d        # open http://<host>:4627
```

Die eerste besoek vra jou om 'n rekening te maak, en maak dan 'n kort opstelling oop wat vra waar jou biblioteek woon. Dit stel `/library` voor, die volume hierbo; kies
dit en los 'n pak op die portaal. Meer liggings kom later, vanaf **Instellings → Biblioteekliggings**:

- **'n Biblioteek wat Hoard hou** kry kopieë van wat jy oplaai of invoer. Elkeen merk die soorte lêers wat dit neem
  (klank op een skyf, modelle op 'n ander), en een is die vang-alles vir die res. Om te verander wat 'n ligging neem,
  geld vir nuwe lêers; **Skuif gestoorde lêers om te pas** skuif wat reeds gestoor is. **Verwyder** skuif eers 'n
  ligging se lêers na die ander, volgens soort, en verwyder dit sodra dit leeg is. Elke lêer word teen sy hash
  nagegaan voordat die ou kopie uitgevee word. 'n Ligging waarvan die skyf weg is, kan in elk geval verwyder word, en
  sy lêers wys dan as vermis.
- **'n Gemonitorde vouer** is 'n vouer met pakke wat jy reeds het. Elke vouer daarin word 'n pak, geïndekseer waar dit
  is. Hoard verander, skuif of vee nooit enigiets daar uit nie. Koppel dit net-lees as jy wil. Hoard kyk daardeur by
  die opstart en elke 15 minute daarna (of wanneer jy **Skandeer nou** druk), en tel nuwe vouers op, lêers wat by 'n
  pak gevoeg of daaruit verwyder is, en pakke waarvan die vouer weg is. 'n Vouer wat skielik leeg of onbereikbaar is,
  word aangeneem as 'n skyf wat nie gekoppel is nie, en sy pakke word gehou. 'n Pak waarvan die vouer vermis is, word
  drie dae lank gehou, met sy etikette en notas, ingeval die vouer terugkom. Enigiets wat jy vir altyd uit 'n
  gemonitorde pak uitvee, bly uitgevee wanneer die vouer weer geskandeer word, al is sy lêers nog daar.

'n Ligging is enige vouer wat die bediener kan sien, so in Docker, koppel dit eers in die houer.

**Bywerk.** Die kopstrook wys 'n kenteken wanneer daar 'n nuwer weergawe is. Voer `docker compose pull && docker compose up -d`
uit om dit te kry; Hoard gradeer sy databasis by die begin op, en stoor eers 'n kopie in `/config/backups`. Verander die
beeld-etiket na `:1.3` om net reggemaakte weergawes te kry. Om outomaties by te werk sonder om Hoard in die middel van
'n oplaai te herbegin, gee **Instellings → Instandhouding → Bywerkings** 'n adres om te roep: Hoard roep dit sodra 'n
weergawe uit is en niks aan die gang is nie (Watchtower se HTTP API, 'n Portainer-webhook, 'n skrip; sien
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opsioneel).** Hoard lewer duimnaels van 3D-modelle op die GPU wanneer dit een kan sien, en in sagteware wanneer
nie. Die pakke-bladsy wys watter (`GPU: ...` of `No GPU found`). Om die GPU deur te gee, voeg die oorlegger vir jou
hardeware by:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Kry albei oorleggers van [deploy/](../../deploy/). Hulle sit ook die plaaslike modelbediener (KI-etikettering) op die GPU. Lêers op 'n NAS, S3-berging, afstandtoegang deur Cloudflare
of Tailscale, en rugsteune word in [docs/DEPLOYMENT.md](../DEPLOYMENT.md) behandel.

## Bou vanaf bronkode

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Of los die zip op die portaal.

## Die res van hierdie README

Wat met 'n pak gebeur, bates in 'n enjin kry, Unreal-pakke, soektog, erkennings, MCP, berging, HTTPS,
konfigurasie, sekuriteit, rekeninge, die API, en feite en syfers word in die [Engelse README](../../README.md)
beskryf.

## Vertalings

Hoard se portaal- en bedienerboodskappe is in 31 tale beskikbaar, gekies in Instellings → Voorkoms (by verstek die
blaaier se taal). Die eerste vertalings is met masjienhulp gemaak; regstellings deur moedertaalsprekers is welkom. [docs/translations.md](../translations.md) verduidelik hoe.

## Gemeenskap

Vrae, idees of iets om te wys? Kom groet ons op die [Azazel Labs Discord](https://discord.gg/zf9ncS4EW). Foute en versoeke vir nuwe funksies word die beste as [GitHub-issues](https://github.com/Azazel-Labs/Hoard/issues) aangemeld, waar hulle nie wegraak nie.

## Lisensie

[MIT No Attribution](../../LICENSE) (MIT-0): gebruik, kopieer, wysig en herversprei Hoard vir enige doel, sonder dat
die kopieregkennisgewing gehou hoef te word. Die batepakke wat jy in Hoard stoor, hou hul eie lisensies.
