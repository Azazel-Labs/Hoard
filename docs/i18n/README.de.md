# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · Deutsch · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Jedes Asset-Paket, das Sie besitzen: durchsuchbar, mit Vorschau und nur einen Klick von Ihrem Spiel entfernt.</h3>

Hoard ist eine selbst gehostete Bibliothek für Spiele-Assets. Legen Sie ein Zip, ein Unity-Paket, einen Fab-Download
oder einen Ordner mit zehntausend Dateien ab, und Hoard macht sich an die Arbeit. Modelle drehen sich in 3D mit ihren
Texturen, lose Textur-Maps werden zu Materialien, Sounds zeichnen ihre Wellenformen, Spritesheets spielen ihre
Animationen ab, und alles bekommt Tags, sodass Sie es finden, wenn Sie `footsteps gravel` oder `rigged low-poly`
eintippen. Danach landet es mit allem, was es braucht, in Godot, Unity oder Unreal.

Kein Abo, kein Upload in irgendjemandes Cloud, keine Preise pro Arbeitsplatz: ein Docker-Image auf Ihrem eigenen
Rechner, und es gehört Ihnen. Stellen Sie es sich als Asset-Store ohne Kasse vor.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Die Hoard-Bibliothek: 3D-Modelle in einem Raster, daneben eine geöffnete Schatztruhe in der 3D-Vorschau mit ihren Tags und Downloads" src="../img/hero-light.webp">
</picture>

### Alles sehen, bevor Sie es verwenden

Modelle öffnen sich in 3D mit ihren Texturen, Materialien und Animationen. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL und
Voxel liest Hoard selbst, ganz ohne installierte 3D-Software; Blender-Dateien kommen über ein Headless-Blender herein,
und Unreal-Pakete zeigen, was der Editor über sie gespeichert hat, ohne Unreal. Vorschaubilder werden auf dem Server
gezeichnet, jedes Mal gleich gerahmt, sodass sich ein ganzes Paket auf einen Blick überfliegen lässt.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Ein Kronleuchter auf einer ganzen Seite: eine große 3D-Ansicht neben seinen Eckdaten, Tags, verknüpften Texturen und Downloads" src="../img/page-light.webp">
</picture>

### Lose Textur-Maps werden zu Materialien

`Rock_BaseColor.png`, `Rock_Normal.png` und `Rock_Roughness.png` werden zu einem einzigen Rock-Material auf einer
beleuchteten Kugel. Hoard versteht die Benennung von Megascans, Poly Haven, ambientCG, Substance und Unreal, gepackte
Maps eingeschlossen, und auch `.mat`-Dateien aus Unity. Der Vollbild-Viewer zeigt ein Material auf einer Kugel, einem
Würfel oder einer Ebene, in seiner realen Größe, und jede Map lässt sich ein- und ausschalten.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Ein Ziegelmaterial im Vollbild-Viewer auf einer Kugel, mit Schaltern für seine Farb-, Normal-, Roughness- und Height-Map" src="../img/material-light.webp">
</picture>

### Spritesheets, die sich abspielen

Atlanten aus TexturePacker, Starling, libGDX, Cocos2d und Godot werden zu einem Spritesheet, das seine Animationen
abspielt und seine Sprites auflistet. Laden Sie es neu gepackt für die Engine herunter, die Sie gerade verwenden: zehn
Formate, und jedes bekommt nur, was es auch aufnehmen kann.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Ein Charakter-Spritesheet auf einer ganzen Seite, eine seiner Animationen läuft neben seinen Tags und Download-Formaten" src="../img/sheet-light.webp">
</picture>

### Sounds, die man sehen kann

Jeder Sound bekommt eine Wellenform, die danach eingefärbt ist, wie hell er klingt, sodass ein harter Schlag und eine
lange Atmo unterschiedlich aussehen, bevor Sie sie abspielen. Metadaten von Soundbibliotheken werden gelesen
(UCS-Kategorien, Soundminer- und BaseHead-Felder, Loop-Punkte, Tempo, Grundton), und MIDI- und Tracker-Musik wird
gerendert, sodass sie sich wie jeder andere Sound abspielen lässt.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Ein Raster aus Soundeffekten, als farbige Wellenformen gezeichnet, einer davon geöffnet mit seinem Player, seinen Tags und Takes" src="../img/sound-light.webp">
</picture>

### Eine Suche, die weiß, was Dinge sind

Hoard taggt alles selbst, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0` und eine
Kategorie aus einem Baum für Spiele-Assets. Suchen Sie nach Wörtern, nach Farbe oder danach, was ein Bild zeigt, und
fragen Sie jede Textur und jeden Sound nach „Mehr davon“. Verbinden Sie es mit einem beliebigen OpenAI-kompatiblen
Modell, lokal oder gehostet, und es schreibt auch Namen und Beschreibungen.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Eine Suche nach „wood“: ein Holztisch, Rinde, Planken und eine Holzoberfläche beieinander, daneben Filter nach Art und Tag" src="../img/search-light.webp">
</picture>

### Direkt in Ihre Engine

- **Engine-Plugins** für Godot 4, Unity und Unreal Engine 5: Durchsuchen Sie die Bibliothek im Editor und drücken Sie
  Zum Projekt hinzufügen. Das Modell kommt mit seinen Texturen, am richtigen Ort, importiert.
- **`hoard sync`** holt genau die Dateien in ein Projekt, die es benennt, festgehalten in einem Lockfile, sodass jedes
  Teammitglied und jede Build-Maschine dieselben Bytes bekommt.
- **Herunterladen als** GLB, PNG, WAV, MP3 oder ein Spritesheet-Format, im Handumdrehen konvertiert, mit Texturnamen
  nach der Konvention Ihres Studios.
- **KI-Agents** suchen, untersuchen und holen Assets über MCP.

### Einmal gespeichert, sicher aufbewahrt

Jede Datei wird einmal anhand ihres BLAKE3-Hashes gespeichert, sodass dieselbe Textur in fünf Bundles nur den Platz
einer einzigen belegt und Beschädigungen nicht unbemerkt bleiben können. Bewahren Sie alles auf einer lokalen Festplatte
oder in einem beliebigen S3-Bucket auf. Ordner, auf die Sie Hoard ansetzen, werden gelesen, nie verändert. Konten,
schreibgeschützte Tokens, Single Sign-on, HTTPS und HTTP/3 sind eingebaut.

<sub>Die Demo-Bibliothek besteht aus CC0-Kunst: Modelle von [Poly Haven](https://polyhaven.com) (glTF und USD),
Oberflächen von [ambientCG](https://ambientcg.com) sowie Sounds, Spritesheets und Schriften von
[Kenney](https://kenney.nl). `node web/scripts/readme-images.mts` erstellt diese Bilder und die
Social-Media-Karte.</sub>

## Installation mit Docker

Hoard wird als ein Image für amd64 und arm64 ausgeliefert: `ghcr.io/azazel-labs/hoard`. Speichern Sie dies als
`compose.yml` (es ist [deploy/compose.yml](../../deploy/compose.yml)):

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

Jedes Volume kann stattdessen ein Ordner auf dem jeweils passenden Datenträger sein: `/mnt/big/hoard:/library` legt
Ihre Dateien auf den großen Datenträger, während die Datenbank klein und leicht zu sichern bleibt. Eingebundene Ordner
brauchen kein chown: Hoard übernimmt `/config` und `/cache` und läuft als der Benutzer, dem `/library` gehört
(setzen Sie `PUID` und `PGID`, um selbst einen Benutzer zu wählen).

Dann starten Sie es:

```sh
docker compose up -d        # open http://<host>:4627
```

Beim ersten Besuch werden Sie gebeten, ein Konto anzulegen; danach öffnet sich eine kurze Einrichtung, die fragt, wo
Ihre Bibliothek liegen soll. Sie schlägt `/library` vor, das Volume von oben; wählen Sie es und legen Sie ein Paket im
Portal ab. Weitere Speicherorte kommen später hinzu, unter **Einstellungen → Speicherorte**:

- **Eine Bibliothek, die Hoard verwaltet**, erhält Kopien dessen, was Sie hochladen oder importieren. Bei jeder sind
  die Dateiarten angehakt, die sie aufnimmt (Audio auf einem Datenträger, Modelle auf einem anderen), und eine ist der
  Auffangort für den Rest. Eine Änderung dessen, was ein Speicherort aufnimmt, gilt für neue Dateien; **Gespeicherte
  Dateien entsprechend verschieben** verschiebt, was bereits gespeichert ist. **Entfernen** verschiebt die Dateien eines
  Speicherorts zuerst nach Art auf die anderen und entfernt ihn, sobald er leer ist. Jede Datei wird anhand ihres
  Hashes geprüft, bevor die alte Kopie gelöscht wird. Ein Speicherort, dessen Datenträger fehlt, lässt sich trotzdem
  entfernen; seine Dateien werden dann als fehlend angezeigt.
- **Ein überwachter Ordner** ist ein Ordner mit Paketen, die Sie bereits haben. Jeder Ordner darin wird zu einem
  Paket, das dort indiziert wird, wo es liegt. Hoard ändert, verschiebt oder löscht dort niemals etwas. Binden Sie ihn
  gern schreibgeschützt ein. Hoard scannt ihn beim Start und danach alle 15 Minuten (oder wenn Sie
  **Jetzt scannen** drücken) und erfasst neue Ordner, Dateien, die einem Paket hinzugefügt oder daraus entfernt
  wurden, und Pakete, deren Ordner verschwunden ist. Ein Ordner, der plötzlich leer oder nicht erreichbar ist, gilt als
  nicht eingebundener Datenträger, und seine Pakete bleiben erhalten. Ein Paket, dessen Ordner fehlt, wird samt Tags
  und Notizen drei Tage lang behalten, falls der Ordner zurückkehrt. Was Sie aus einem überwachten Paket endgültig
  löschen, bleibt gelöscht, wenn der Ordner erneut gescannt wird, auch wenn seine Dateien noch dort liegen.

Ein Speicherort ist jeder Ordner, den der Server sehen kann; binden Sie ihn unter Docker also zuerst in den Container
ein.

**Aktualisieren.** In der Kopfzeile erscheint ein Hinweis, wenn es eine neuere Version gibt. Führen Sie
`docker compose pull && docker compose up -d` aus, um sie zu übernehmen; Hoard aktualisiert seine Datenbank beim
Start und legt vorher eine Kopie in `/config/backups` ab. Ändern Sie den Image-Tag auf `:1.2`, um nur Patch-Releases
zu bekommen. Um automatisch zu aktualisieren, ohne Hoard mitten in einem Upload neu zu starten, geben Sie unter
**Einstellungen → Wartung → Updates** eine Adresse zum Aufrufen an: Hoard ruft sie auf, sobald eine Version
erschienen ist und gerade nichts läuft (die HTTP-API von Watchtower, ein Portainer-Webhook, ein Skript; siehe
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (optional).** Hoard rendert Vorschaubilder von 3D-Modellen auf der GPU, wenn es eine sieht, und andernfalls per
Software. Die Paketseite zeigt, welches von beiden (`GPU: ...` oder `No GPU found`). Um die GPU durchzureichen, fügen
Sie das Overlay für Ihre Hardware hinzu:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Beide Overlays finden Sie unter [deploy/](../../deploy/). Sie legen auch den lokalen Modellserver (KI-Tagging) auf die
GPU. Dateien auf einem NAS, S3-Speicher, Fernzugriff über Cloudflare oder Tailscale sowie Backups werden in
[docs/DEPLOYMENT.md](../DEPLOYMENT.md) behandelt.

## Aus dem Quellcode bauen

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Oder legen Sie das Zip im Portal ab.

## Der Rest dieses README

Was mit einem Paket geschieht, wie Assets in eine Engine kommen, Unreal-Pakete, Suche, Credits, MCP, Speicher, HTTPS,
Konfiguration, Sicherheit, Konten, die API sowie Zahlen und Fakten werden im [englischen README](../../README.md)
beschrieben.

## Übersetzungen

Das Portal und die Servermeldungen von Hoard gibt es in 31 Sprachen, auswählbar unter Einstellungen → Darstellung
(standardmäßig die Sprache des Browsers). Die ersten Übersetzungen entstanden maschinengestützt; Korrekturen von
Muttersprachlern sind willkommen. [docs/translations.md](../translations.md) erklärt, wie es geht.

## Lizenz

[MIT No Attribution](../../LICENSE) (MIT-0): Hoard darf für jeden Zweck verwendet, kopiert, verändert und
weitergegeben werden, ohne dass der Copyright-Hinweis erhalten bleiben muss. Die Asset-Pakete, die Sie in Hoard
speichern, behalten ihre eigenen Lizenzen.
