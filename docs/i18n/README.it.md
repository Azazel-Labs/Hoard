# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · Italiano · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Una libreria di asset self-hosted per lo sviluppo di videogiochi. Aggiungi interi pacchetti di asset (zip, tarball,
cartelle) e Hoard salva ogni file una sola volta in base al suo hash, capisce cos'è ogni file, gli assegna dei tag,
collega le mesh alle loro texture e animazioni, e rende tutto ricercabile da un portale web, da una CLI, da un endpoint
MCP per gli agenti e da un `hoard sync` basato su lockfile che scarica esattamente i file di cui il tuo progetto ha
bisogno.

Pensalo come uno store di asset senza la cassa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="La libreria di Hoard: modelli e materiali in una griglia, con un forziere del tesoro aperto nell'anteprima 3D accanto ai suoi tag e download" src="../img/hero-light.webp">
</picture>

<sub>La libreria di esempio è arte CC0: modelli da [Poly Haven](https://polyhaven.com) (glTF e USD) e superfici da
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` crea queste immagini e la social card.</sub>

## Installazione con Docker

Hoard è distribuito come un'unica immagine per amd64 e arm64: `ghcr.io/azazel-labs/hoard`. Salva questo come
`compose.yml` (è [deploy/compose.yml](../../deploy/compose.yml)):

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

Ogni volume può invece essere una cartella sul disco più adatto: `/mnt/big/hoard:/library` mette i tuoi file sul disco
grande, mentre il database resta piccolo e facile da salvare. Le cartelle che monti non richiedono chown: Hoard prende
possesso di `/config` e `/cache` e viene eseguito come il proprietario di `/library` (imposta `PUID` e `PGID` per
scegliere tu un utente).

Poi avvialo:

```sh
docker compose up -d        # open http://<host>:4627
```

La prima visita ti chiede di creare un account, poi apre una breve configurazione che chiede dove si trova la tua
libreria. Propone `/library`, il volume qui sopra; sceglilo e trascina un pacchetto sul portale. Altre posizioni si
aggiungono più avanti, da **Impostazioni → Posizioni della libreria**:

- **Una libreria gestita da Hoard** riceve copie di ciò che carichi o importi. Ognuna indica i tipi di file che
  accetta (l'audio su un disco, i modelli su un altro), e una è quella predefinita per tutto il resto. Cambiare ciò
  che una posizione accetta vale per i nuovi file; **Sposta i file salvati di conseguenza** sposta ciò che è già
  salvato. **Rimuovi** prima sposta i file di una posizione nelle altre, per tipo, e la rimuove quando è vuota. Ogni
  file viene verificato con il suo hash prima che la vecchia copia venga eliminata. Una posizione il cui disco non c'è
  più può essere rimossa comunque, e i suoi file risultano allora mancanti.
- **Una cartella monitorata** è una cartella di pacchetti che hai già. Ogni cartella al suo interno diventa un
  pacchetto, indicizzato dove si trova. Hoard non cambia, sposta né elimina mai niente lì dentro. Se vuoi, montala in
  sola lettura. Hoard la esamina all'avvio e poi ogni 15 minuti (o quando premi **Scansiona ora**), trovando le nuove
  cartelle, i file aggiunti o tolti da un pacchetto e i pacchetti la cui cartella non c'è più. Una cartella che
  all'improvviso è vuota o irraggiungibile viene considerata un disco non montato, e i suoi pacchetti vengono
  conservati. Un pacchetto la cui cartella manca viene conservato per tre giorni, con i suoi tag e le sue note, nel
  caso la cartella ritorni. Ciò che elimini definitivamente da un pacchetto monitorato resta eliminato quando la
  cartella viene riscansionata, anche se i suoi file sono ancora lì.

Una posizione è qualsiasi cartella che il server riesce a vedere, quindi in Docker montala prima nel container.

**Aggiornamenti.** L'intestazione mostra un badge quando esiste una versione più recente. Esegui
`docker compose pull && docker compose up -d` per installarla; Hoard aggiorna il suo database all'avvio, salvandone
prima una copia in `/config/backups`. Cambia il tag dell'immagine in `:1.2` per ricevere solo le versioni di patch. Per
aggiornare automaticamente senza riavviare Hoard a metà di un caricamento, dai a
**Impostazioni → Manutenzione → Aggiornamenti** un indirizzo da chiamare: Hoard lo chiama quando esce una versione e
non succede niente (l'API HTTP di Watchtower, un webhook di Portainer, uno script; vedi
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (facoltativa).** Hoard genera le miniature dei modelli 3D sulla GPU quando ne vede una, e via software quando
non la vede. La pagina dei pacchetti indica quale delle due (`GPU: ...` o `No GPU found`). Per passare la GPU al
container, aggiungi l'overlay per il tuo hardware:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Entrambi gli overlay sono in [deploy/](../../deploy/). Mettono sulla GPU anche il server dei modelli locali (tag con
IA). File su un NAS, archiviazione S3, accesso remoto tramite Cloudflare o Tailscale e backup sono trattati in
[docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Compilazione dai sorgenti

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Oppure trascina lo zip sul portale.

## Il resto di questo README

Cosa succede a un pacchetto, come portare gli asset in un motore, i pacchetti Unreal, la ricerca, i crediti, MCP,
l'archiviazione, HTTPS, la configurazione, la sicurezza, gli account e l'API sono descritti nel
[README in inglese](../../README.md).

## Traduzioni

Il portale e i messaggi del server di Hoard sono disponibili in 31 lingue, da scegliere in Impostazioni → Aspetto (per
impostazione predefinita, la lingua del browser). Le prime traduzioni sono state fatte con l'aiuto di una macchina; le
correzioni da parte di madrelingua sono benvenute. [docs/translations.md](../translations.md) spiega come.

## Licenza

[MIT No Attribution](../../LICENSE) (MIT-0): usa, copia, modifica e ridistribuisci Hoard per qualsiasi scopo, senza
bisogno di mantenere l'avviso di copyright. I pacchetti di asset che salvi in Hoard mantengono le proprie licenze.
