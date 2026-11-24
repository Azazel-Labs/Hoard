<div align="center">

# <img src="../../web/public/favicon.svg" alt="" height="32" align="absmiddle"> Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· Italiano&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">Ogni pacchetto di asset che possiedi, ricercabile, visualizzabile in anteprima e a un clic dal tuo gioco.</h3>

Hoard è una libreria self-hosted per gli asset dei videogiochi. Aggiungi uno zip, un pacchetto Unity, un download di Fab
o una cartella con diecimila file, e Hoard si mette al lavoro. I modelli ruotano in 3D con le loro texture, le mappe di
texture sparse diventano materiali, i suoni disegnano la loro forma d'onda, i fogli di sprite riproducono le loro
animazioni, e tutto riceve dei tag, così basta scrivere `footsteps gravel` o `rigged low-poly` per trovarlo. Poi arriva
in Godot, Unity o Unreal con tutto ciò che gli serve.

Nessun abbonamento, nessun caricamento sul cloud di qualcun altro, nessun prezzo per postazione: un'unica immagine
Docker sul tuo server, o un'app sul tuo computer, ed è tua. Pensalo come uno store di asset senza la cassa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="La libreria di Hoard: modelli 3D in una griglia, con un forziere del tesoro aperto nell'anteprima 3D accanto ai suoi tag e download" src="../img/hero-light.webp">
</picture>

### Guarda tutto prima di usarlo

I modelli si aprono in 3D con le loro texture, i materiali e le animazioni. FBX, glTF, OBJ, USD, SketchUp, Collada, X3D, VRM, 3MF, STL e i
voxel vengono letti direttamente da Hoard, senza alcuna applicazione 3D installata; i file Blender passano da Blender in
modalità headless, e i pacchetti Unreal mostrano ciò che l'editor ha salvato su di loro, senza Unreal. Le miniature
vengono disegnate sul server, inquadrate sempre allo stesso modo, così un intero pacchetto si scorre con un colpo
d'occhio.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Un lampadario mostrato a pagina intera: una grande vista 3D accanto alle sue informazioni, ai tag, alle texture collegate e ai download" src="../img/page-light.webp">
</picture>

### Le mappe di texture sparse diventano materiali

`Rock_BaseColor.png`, `Rock_Normal.png` e `Rock_Roughness.png` diventano un unico materiale Rock su una sfera
illuminata. Hoard riconosce le convenzioni di nomi di Megascans, Poly Haven, ambientCG, Substance e Unreal, mappe
impacchettate comprese, e legge anche i file `.mat` di Unity. Il visualizzatore a schermo intero mostra un materiale su
una sfera, un cubo o un piano, nella sua scala reale, attivando e disattivando ogni mappa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Un materiale di mattoni nel visualizzatore a schermo intero, su una sfera, con gli interruttori per le mappe di colore, normali, rugosità e altezza" src="../img/material-light.webp">
</picture>

### Fogli di sprite che si animano

Gli atlanti di TexturePacker, Starling, libGDX, Cocos2d e Godot diventano un unico foglio di sprite che riproduce le sue
animazioni ed elenca i suoi sprite. Scaricalo reimpacchettato per il motore che stai usando: dieci formati, a ciascuno
dei quali viene passato solo ciò che può contenere.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Il foglio di sprite di un personaggio a pagina intera, con una delle sue animazioni in riproduzione accanto ai tag e ai formati di download" src="../img/sheet-light.webp">
</picture>

### Suoni che puoi vedere

Ogni suono riceve una forma d'onda colorata in base a quanto suona brillante, così un colpo secco e un lungo suono
d'ambiente appaiono diversi prima ancora di riprodurli. Vengono letti i metadati delle librerie di suoni (categorie UCS,
campi Soundminer e BaseHead, punti di loop, tempo, nota fondamentale), e la musica MIDI e tracker viene renderizzata
così da suonare come qualsiasi altro suono.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Una griglia di effetti sonori disegnati come forme d'onda colorate, con uno aperto che mostra il lettore, i tag e le take" src="../img/sound-light.webp">
</picture>

### Una ricerca che sa cosa sono le cose

Hoard assegna i tag a tutto da solo, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0`, e una categoria presa da un albero di asset per videogiochi. Cerca per parole, per colore o per ciò che
mostra un'immagine, e chiedi “Altri simili” a qualsiasi texture o suono. Collegalo a qualsiasi modello compatibile con
OpenAI, locale o in hosting, e scriverà anche nomi e descrizioni.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Una ricerca di wood: un tavolo di legno, corteccia, assi e una superficie di legno insieme, con accanto i filtri per tipo e per tag" src="../img/search-light.webp">
</picture>

### Dritto nel tuo motore

- **Plugin per i motori** Godot 4, Unity, Unreal Engine 5 e Lumina: sfoglia la libreria dentro l'editor e premi Aggiungi al
  progetto. Il modello arriva con le sue texture, nel posto giusto, già importato.
- **`hoard sync`** porta in un progetto esattamente i file che quel progetto indica, fissati in un lockfile, così ogni
  membro del team e ogni macchina di build riceve gli stessi byte.
- **Scarica come** GLB, PNG, WAV, MP3 o un formato di fogli di sprite, convertito al volo, con i nomi delle texture
  secondo la convenzione del tuo studio.
- **Gli agenti IA** cercano, esaminano e scaricano asset tramite MCP, e possono fare qualsiasi altra cosa che fa il
  portale, con i permessi dell'account o del token con cui accedono.

### Salvato una volta, al sicuro

Ogni file viene salvato una sola volta in base al suo hash BLAKE3, così la stessa texture in cinque pacchetti occupa lo
spazio di una sola, e nessuna corruzione può passare inosservata. Tienilo su disco locale o in qualsiasi bucket S3. Le
cartelle che indichi a Hoard vengono lette, mai modificate. Account, token di sola lettura, single sign-on, HTTPS e
HTTP/3 sono integrati.

### Oppure solo sul tuo computer

Non hai un server? L'app desktop per Windows, macOS e Linux è la stessa libreria in una finestra tutta sua. Installala,
scegli una cartella e funziona offline: niente account, password o impostazioni del server, niente che il resto della
rete possa raggiungere, e le chiavi che inserisci conservate nel portachiavi di sistema. Le nuove versioni si scaricano
in background e si installano quando la chiudi.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="L'app desktop di Hoard in una finestra di Windows: i modelli della libreria in una griglia, con una lanterna aperta nell'anteprima 3D accanto ai suoi tag e download" src="../img/desktop-light.webp">
</picture>

<sub>La libreria di esempio è arte CC0: modelli da [Poly Haven](https://polyhaven.com) (glTF e USD), superfici da
[ambientCG](https://ambientcg.com), e suoni, fogli di sprite e font da [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` crea queste immagini e la social card.</sub>

## Installazione dell'app desktop

Scaricala dall'[ultima versione](https://github.com/Azazel-Labs/Hoard/releases/latest): il file `-setup.exe` per
Windows, il `.dmg` per macOS su Apple silicon, l'`.AppImage` o il `.deb` per Linux. Su Windows si installa solo per te,
senza chiedere i permessi di amministratore. Gli installer non sono ancora firmati da Apple o Microsoft, quindi Windows
chiede conferma prima di eseguire l'installer (Ulteriori informazioni → Esegui comunque) e macOS prima di aprire l'app
la prima volta (Impostazioni di Sistema → Privacy e sicurezza → Apri comunque).

La prima volta che si apre, chiede dove mettere la libreria. Tutto il resto è nelle Impostazioni, come su un server.

Da lì in poi si tiene aggiornata da sola, senza che tu debba cliccare nulla: una nuova versione si scarica in background
e si installa quando la chiudi, così la prossima volta che apri Hoard è già la versione nuova. Un `.deb` si aggiorna
installando il successivo.

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
prima una copia in `/config/backups`. Cambia il tag dell'immagine in `:1.3` per ricevere solo le versioni di patch. Per
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
l'archiviazione, HTTPS, la configurazione, la sicurezza, gli account, l'API, i fatti e le cifre sono descritti nel
[README in inglese](../../README.md).

## Traduzioni

Il portale e i messaggi del server di Hoard sono disponibili in 31 lingue, da scegliere in Impostazioni → Aspetto (per
impostazione predefinita, la lingua del browser). Le prime traduzioni sono state fatte con l'aiuto di una macchina; le
correzioni da parte di madrelingua sono benvenute. [docs/translations.md](../translations.md) spiega come.

## Community

Domande, idee o qualcosa da mostrare? Passa a salutare sul [Discord di Azazel Labs](https://discord.gg/zf9ncS4EW). Bug e richieste di funzionalità meglio come [issue su GitHub](https://github.com/Azazel-Labs/Hoard/issues), dove non vanno persi.

## Licenza

[MIT No Attribution](../../LICENSE) (MIT-0): usa, copia, modifica e ridistribuisci Hoard per qualsiasi scopo, senza
bisogno di mantenere l'avviso di copyright. I pacchetti di asset che salvi in Hoard mantengono le proprie licenze.
