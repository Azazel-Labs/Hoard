# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · Català · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Una biblioteca de recursos autoallotjada per al desenvolupament de videojocs. Hi deixeu anar paquets de recursos sencers
(zips, tarballs, carpetes) i Hoard desa cada fitxer una sola vegada pel seu hash, esbrina què és cada fitxer, l'etiqueta,
enllaça les malles amb les seves textures i animacions, i fa que tot es pugui cercar des d'un portal web, una CLI, un
punt d'accés MCP per a agents i un `hoard sync` basat en un fitxer de bloqueig que baixa exactament els fitxers que
necessita el vostre projecte de joc.

Penseu-hi com una botiga de recursos sense caixa per pagar.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="La biblioteca de Hoard: models i materials en una graella, amb un cofre del tresor obert a la previsualització 3D al costat de les seves etiquetes i baixades" src="../img/hero-light.webp">
</picture>

<sub>La biblioteca de demostració és art CC0: models de [Poly Haven](https://polyhaven.com) (glTF i USD) i superfícies
d'[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` genera aquestes imatges i la targeta per a
xarxes socials.</sub>

## Instal·lació amb Docker

Hoard es distribueix com una sola imatge per a amd64 i arm64: `ghcr.io/azazel-labs/hoard`. Deseu això com a
`compose.yml` (és [deploy/compose.yml](../../deploy/compose.yml)):

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

Cada volum pot ser, en canvi, una carpeta al disc que més li convingui: `/mnt/big/hoard:/library` posa els vostres
fitxers al disc gran, mentre que la base de dades continua sent petita i fàcil de copiar. Les carpetes que munteu no
necessiten chown: Hoard s'apropia de `/config` i `/cache` i s'executa amb l'usuari propietari de `/library` (definiu
`PUID` i `PGID` per triar vosaltres l'usuari).

Després, inicieu-lo:

```sh
docker compose up -d        # open http://<host>:4627
```

La primera visita us demana que creeu un compte i després obre una configuració breu que us pregunta on és la vostra
biblioteca. Suggereix `/library`, el volum de més amunt; trieu-lo i deixeu anar un paquet al portal. Més endavant podeu
afegir més ubicacions, des de **Configuració → Ubicacions de la biblioteca**:

- **Una biblioteca que gestiona Hoard** rep còpies del que pugeu o importeu. Cadascuna marca els tipus de fitxer que
  accepta (l'àudio en un disc, els models en un altre), i una és la general per a la resta. Canviar el que accepta una
  ubicació s'aplica als fitxers nous; **Mou els fitxers desats perquè hi coincideixin** mou el que ja està desat.
  **Elimina** primer mou els fitxers d'una ubicació a les altres, segons el tipus, i l'elimina quan queda buida. Cada
  fitxer es comprova amb el seu hash abans de suprimir la còpia antiga. Una ubicació el disc de la qual ha desaparegut
  es pot eliminar igualment, i llavors els seus fitxers es mostren com a absents.
- **Una carpeta vigilada** és una carpeta de paquets que ja teniu. Cada carpeta que conté es converteix en un paquet,
  indexat on és. Hoard no hi canvia, mou ni suprimeix mai res. Si voleu, munteu-la només de lectura. Hoard la revisa en
  iniciar-se i després cada 15 minuts (o quan premeu **Escaneja ara**), i incorpora les carpetes noves, els fitxers
  afegits a un paquet o eliminats, i els paquets la carpeta dels quals ha desaparegut. Una carpeta que de cop és buida o
  inaccessible es considera un disc que no està muntat, i se'n conserven els paquets. Un paquet la carpeta del qual
  falta es conserva tres dies, amb les seves etiquetes i notes, per si la carpeta torna. Tot el que suprimiu
  definitivament d'un paquet vigilat continua suprimit quan es torna a escanejar la carpeta, encara que els fitxers hi
  siguin.

Una ubicació és qualsevol carpeta que el servidor pugui veure, així que a Docker primer l'heu de muntar al contenidor.

**Actualitzacions.** La capçalera mostra una insígnia quan hi ha una versió més nova. Executeu
`docker compose pull && docker compose up -d` per instal·lar-la; Hoard actualitza la seva base de dades en iniciar-se, i
abans en desa una còpia a `/config/backups`. Canvieu l'etiqueta de la imatge a `:0.8` per rebre només versions de
correcció. Per actualitzar automàticament sense reiniciar Hoard enmig d'una pujada, doneu a
**Configuració → Manteniment → Actualitzacions** una adreça a cridar: Hoard la crida quan surt una versió i no passa res
(l'API HTTP de Watchtower, un webhook de Portainer, un script; vegeu [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opcional).** Hoard renderitza les miniatures dels models 3D a la GPU quan en veu una, i per programari quan no.
La pàgina de paquets mostra quina de les dues (`GPU: ...` o `No GPU found`). Per passar-hi la GPU, afegiu la
superposició per al vostre maquinari:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Obteniu les dues superposicions de [deploy/](../../deploy/). També posen a la GPU el servidor de models local
(etiquetatge amb IA). Els fitxers en un NAS, l'emmagatzematge S3, l'accés remot a través de Cloudflare o Tailscale i les
còpies de seguretat s'expliquen a [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Compilació des del codi font

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

O deixeu anar el zip al portal.

## La resta d'aquest README

Què li passa a un paquet, com portar els recursos a un motor, els paquets d'Unreal, la cerca, els crèdits, MCP,
l'emmagatzematge, HTTPS, la configuració, la seguretat, els comptes i l'API es descriuen al
[README en anglès](../../README.md).

## Traduccions

Els missatges del portal i del servidor de Hoard estan disponibles en 31 idiomes, que es trien a Configuració → Aparença
(per defecte, l'idioma del navegador). Les primeres traduccions es van fer amb ajuda de traducció automàtica; les
correccions de parlants nadius són benvingudes. [docs/translations.md](../translations.md) explica com fer-ho.

## Llicència

[MIT No Attribution](../../LICENSE) (MIT-0): feu servir, copieu, modifiqueu i redistribuïu Hoard per a qualsevol
finalitat, sense haver de conservar l'avís de drets d'autor. Els paquets de recursos que deseu a Hoard conserven les
seves pròpies llicències.
