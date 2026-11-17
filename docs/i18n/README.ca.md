# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · Català · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Tots els paquets de recursos que teniu, cercables, previsualitzables i a un clic del vostre joc.</h3>

Hoard és una biblioteca autoallotjada de recursos per a videojocs. Hi deixeu anar un zip, un paquet d'Unity, una baixada
de Fab o una carpeta amb deu mil fitxers, i Hoard es posa a treballar. Els models giren en 3D amb les seves textures,
els mapes de textura solts es converteixen en materials, els sons dibuixen les seves formes d'ona, els fulls de sprites
reprodueixen les seves animacions i tot queda etiquetat, de manera que només cal escriure `footsteps gravel` o
`rigged low-poly` per trobar-ho. Després arriba a Godot, Unity o Unreal amb tot el que necessita.

Sense subscripció, sense pujar res al núvol de ningú, sense preus per usuari: una imatge de Docker a la vostra pròpia
màquina, i és vostra. Penseu-hi com una botiga de recursos sense caixa per pagar.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="La biblioteca de Hoard: models 3D en una graella, amb un cofre del tresor obert a la previsualització 3D al costat de les seves etiquetes i baixades" src="../img/hero-light.webp">
</picture>

### Vegeu-ho tot abans de fer-ho servir

Els models s'obren en 3D amb les seves textures, materials i animacions. Hoard mateix llegeix FBX, glTF, OBJ, USD,
SketchUp, 3MF, STL i vòxels, sense cap aplicació 3D instal·lada; els fitxers de Blender entren a través d'un Blender
sense interfície, i els paquets d'Unreal mostren el que l'editor en va desar, sense Unreal. Les miniatures es dibuixen
al servidor, enquadrades sempre de la mateixa manera, de manera que es pot repassar tot un paquet d'un cop d'ull.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Una aranya de llums a pàgina completa: una vista 3D gran al costat de les seves dades, etiquetes, textures enllaçades i baixades" src="../img/page-light.webp">
</picture>

### Els mapes de textura solts es converteixen en materials

`Rock_BaseColor.png`, `Rock_Normal.png` i `Rock_Roughness.png` es converteixen en un sol material Rock sobre una esfera
il·luminada. Hoard entén la nomenclatura de Megascans, Poly Haven, ambientCG, Substance i Unreal, inclosos els mapes
empaquetats, i també els fitxers `.mat` d'Unity. El visualitzador a pantalla completa mostra un material sobre una
esfera, un cub o un pla, a la seva escala real, i cada mapa es pot activar i desactivar.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Un material de maó al visualitzador a pantalla completa, sobre una esfera, amb interruptors per als seus mapes de color, normals, rugositat i alçada" src="../img/material-light.webp">
</picture>

### Fulls de sprites que es reprodueixen

Els atles de TexturePacker, Starling, libGDX, Cocos2d i Godot es converteixen en un sol full de sprites que reprodueix
les seves animacions i llista els seus sprites. Baixeu-lo reempaquetat per al motor que feu servir: deu formats, i a
cadascun només se li dona el que pot contenir.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="El full de sprites d'un personatge a pàgina completa, amb una de les seves animacions reproduint-se al costat de les seves etiquetes i formats de baixada" src="../img/sheet-light.webp">
</picture>

### Sons que es poden veure

Cada so té una forma d'ona acolorida segons com de brillant sona, de manera que un cop sec i un ambient llarg es veuen
diferents abans de reproduir-los. Es llegeixen les metadades de les biblioteques de so (categories UCS, camps de
Soundminer i BaseHead, punts de bucle, tempo, nota fonamental), i la música MIDI i de tracker es renderitza perquè es
reprodueixi com qualsevol altre so.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Una graella d'efectes de so dibuixats com a formes d'ona de colors, amb un d'ells obert que mostra el seu reproductor, etiquetes i preses" src="../img/sound-light.webp">
</picture>

### Una cerca que sap què és cada cosa

Hoard ho etiqueta tot ell mateix, sense connexió: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0`, i una categoria d'un arbre de recursos per a videojocs. Cerqueu per paraules, per color o pel que mostra
una imatge, i demaneu a qualsevol textura o so «Més com aquest». Connecteu-lo a qualsevol model compatible amb OpenAI,
local o allotjat, i també escriurà noms i descripcions.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Una cerca de «wood»: una taula de fusta, escorça, taulons i una superfície de fusta junts, amb filtres per tipus i etiqueta al costat" src="../img/search-light.webp">
</picture>

### Directament al vostre motor

- **Complements de motor** per a Godot 4, Unity, Unreal Engine 5 i Lumina: navegueu per la biblioteca dins de l'editor i premeu
  Afegeix al projecte. El model arriba amb les seves textures, al lloc correcte, importat.
- **`hoard sync`** porta a un projecte exactament els fitxers que aquest indica, fixats en un fitxer de bloqueig, de
  manera que tots els companys d'equip i totes les màquines de compilació reben els mateixos bytes.
- **Baixa com a** GLB, PNG, WAV, MP3 o un format de full de sprites, convertit al moment, amb noms de textura segons la
  convenció del vostre estudi.
- **Els agents d'IA** cerquen, inspeccionen i obtenen recursos per MCP.

### Desat una vegada, ben protegit

Cada fitxer es desa una sola vegada pel seu hash BLAKE3, de manera que la mateixa textura en cinc paquets ocupa l'espai
d'una sola, i cap corrupció no pot passar desapercebuda. Guardeu-ho en un disc local o en qualsevol bucket S3. Les
carpetes que indiqueu a Hoard es llegeixen, però no es modifiquen mai. Els comptes, els tokens de només lectura, l'inici
de sessió únic, HTTPS i HTTP/3 hi són integrats.

<sub>La biblioteca de demostració és art CC0: models de [Poly Haven](https://polyhaven.com) (glTF i USD), superfícies
d'[ambientCG](https://ambientcg.com), i sons, fulls de sprites i tipus de lletra de [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` genera aquestes imatges i la targeta per a xarxes socials.</sub>

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
abans en desa una còpia a `/config/backups`. Canvieu l'etiqueta de la imatge a `:1.2` per rebre només versions de
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
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
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
l'emmagatzematge, HTTPS, la configuració, la seguretat, els comptes, l'API i les dades i xifres es descriuen al
[README en anglès](../../README.md).

## Traduccions

Els missatges del portal i del servidor de Hoard estan disponibles en 31 idiomes, que es trien a Configuració → Aparença
(per defecte, l'idioma del navegador). Les primeres traduccions es van fer amb ajuda de traducció automàtica; les
correccions de parlants nadius són benvingudes. [docs/translations.md](../translations.md) explica com fer-ho.

## Llicència

[MIT No Attribution](../../LICENSE) (MIT-0): feu servir, copieu, modifiqueu i redistribuïu Hoard per a qualsevol
finalitat, sense haver de conservar l'avís de drets d'autor. Els paquets de recursos que deseu a Hoard conserven les
seves pròpies llicències.
