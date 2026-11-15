# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · Deutsch · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Eine selbst gehostete Asset-Bibliothek für die Spieleentwicklung. Legen Sie ganze Asset-Pakete ab (Zips, Tarballs,
Ordner), und Hoard speichert jede Datei einmal anhand ihres Hashes, erkennt, was jede Datei ist, taggt sie, verknüpft
Meshes mit ihren Texturen und Animationen und macht alles durchsuchbar: über ein Webportal, eine CLI, einen
MCP-Endpunkt für Agents und ein Lockfile-basiertes `hoard sync`, das genau die Dateien holt, die Ihr Spielprojekt
braucht.

Stellen Sie es sich als Asset-Store ohne Kasse vor.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Die Hoard-Bibliothek: Modelle und Materialien in einem Raster, daneben eine geöffnete Schatztruhe in der 3D-Vorschau mit ihren Tags und Downloads" src="../img/hero-light.webp">
</picture>

<sub>Die Demo-Bibliothek besteht aus CC0-Kunst: Modelle von [Poly Haven](https://polyhaven.com) (glTF und USD) und Oberflächen von
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` erstellt diese Bilder und die Social-Media-Karte.</sub>

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
Start und legt vorher eine Kopie in `/config/backups` ab. Ändern Sie den Image-Tag auf `:0.8`, um nur Patch-Releases
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
Konfiguration, Sicherheit, Konten und die API werden im [englischen README](../../README.md) beschrieben.

## Übersetzungen

Das Portal und die Servermeldungen von Hoard gibt es in 31 Sprachen, auswählbar unter Einstellungen → Darstellung
(standardmäßig die Sprache des Browsers). Die ersten Übersetzungen entstanden maschinengestützt; Korrekturen von
Muttersprachlern sind willkommen. [docs/translations.md](../translations.md) erklärt, wie es geht.

## Lizenz

[MIT No Attribution](../../LICENSE) (MIT-0): Hoard darf für jeden Zweck verwendet, kopiert, verändert und
weitergegeben werden, ohne dass der Copyright-Hinweis erhalten bleiben muss. Die Asset-Pakete, die Sie in Hoard
speichern, behalten ihre eigenen Lizenzen.
