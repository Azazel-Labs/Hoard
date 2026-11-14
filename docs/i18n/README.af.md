# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · Afrikaans · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

'n Selfgehuisveste batebiblioteek vir speletjie-ontwikkeling. Los hele batepakke in (zips, tarballs, vouers), en Hoard
stoor elke lêer een keer volgens sy hash, werk uit wat elke lêer is, etiketteer dit, koppel mase aan hul teksture en
animasies, en maak alles deursoekbaar vanaf 'n webportaal, 'n CLI, 'n MCP-eindpunt vir agente, en 'n `hoard sync` wat
op 'n lockfile gebaseer is en presies die lêers aftrek wat jou speletjieprojek nodig het.

Dink daaraan as 'n batewinkel sonder 'n betaalpunt.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Die Hoard-biblioteek: modelle en materiale in 'n rooster, met 'n oop skatkis in die 3D-voorskou langs sy etikette en aflaaie" src="../img/hero-light.webp">
</picture>

<sub>Die demobiblioteek is CC0-kuns: modelle van [Poly Haven](https://polyhaven.com) (glTF en USD) en oppervlakke van
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` maak hierdie prente en die sosiale kaart.</sub>

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
beeld-etiket na `:0.8` om net reggemaakte weergawes te kry. Om outomaties by te werk sonder om Hoard in die middel van
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
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
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
konfigurasie, sekuriteit, rekeninge en die API word in die [Engelse README](../../README.md) beskryf.

## Vertalings

Hoard se portaal- en bedienerboodskappe is in 31 tale beskikbaar, gekies in Instellings → Voorkoms (by verstek die
blaaier se taal). Die eerste vertalings is met masjienhulp gemaak; regstellings deur moedertaalsprekers is welkom. [docs/translations.md](../translations.md) verduidelik hoe.

## Lisensie

[MIT No Attribution](../../LICENSE) (MIT-0): gebruik, kopieer, wysig en herversprei Hoard vir enige doel, sonder dat
die kopieregkennisgewing gehou hoef te word. Die batepakke wat jy in Hoard stoor, hou hul eie lisensies.
