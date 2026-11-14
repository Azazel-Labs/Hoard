# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · Română · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

O bibliotecă de resurse găzduită de dvs., pentru dezvoltarea de jocuri. Adăugați pachete întregi de resurse (zip-uri,
tarballuri, dosare), iar Hoard stochează fiecare fișier o singură dată, după hash, își dă seama ce este fiecare fișier,
îl etichetează, leagă mesh-urile de texturile și animațiile lor și face totul căutabil dintr-un portal web, dintr-un
CLI, dintr-un endpoint MCP pentru agenți și prin `hoard sync`, bazat pe un lockfile, care aduce exact fișierele de care
are nevoie proiectul dvs. de joc.

Gândiți-vă la el ca la un magazin de resurse fără casă de marcat.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Biblioteca Hoard: modele și materiale într-o grilă, cu un cufăr cu comori deschis în previzualizarea 3D, lângă etichetele și descărcările sale" src="../img/hero-light.webp">
</picture>

<sub>Biblioteca demonstrativă conține grafică CC0: modele de pe [Poly Haven](https://polyhaven.com) (glTF și USD) și
suprafețe de pe [ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` creează aceste imagini și
cardul pentru rețelele sociale.</sub>

## Instalare cu Docker

Hoard vine ca o singură imagine pentru amd64 și arm64: `ghcr.io/azazel-labs/hoard`. Salvați asta ca `compose.yml` (este
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

Fiecare volum poate fi în schimb un dosar pe discul care i se potrivește: `/mnt/big/hoard:/library` vă pune fișierele
pe discul mare, în timp ce baza de date rămâne mică și ușor de salvat. Dosarele pe care le montați nu au nevoie de chown:
Hoard preia `/config` și `/cache` și rulează ca proprietarul lui `/library` (setați `PUID` și `PGID` pentru a alege
singur un utilizator).

Apoi porniți-l:

```sh
docker compose up -d        # open http://<host>:4627
```

La prima vizită vi se cere să creați un cont, apoi se deschide o configurare scurtă care vă întreabă unde stă biblioteca
dvs. Vă sugerează `/library`, volumul de mai sus; alegeți-l și plasați un pachet în portal. Alte locații vin mai
târziu, din **Setări → Locații ale bibliotecii**:

- **O bibliotecă păstrată de Hoard** primește copii ale celor încărcate sau importate. Fiecare bifează tipurile de
  fișiere pe care le acceptă (audio pe un disc, modele pe altul), iar una este locația implicită pentru restul.
  Schimbarea a ceea ce acceptă o locație se aplică fișierelor noi; **Mutați fișierele stocate în consecință** mută ce
  este deja stocat. **Eliminați** mută mai întâi fișierele unei locații în celelalte, după tip, și o elimină după ce
  se golește. Fiecare fișier este verificat după hash înainte ca vechea copie să fie ștearsă. O locație al cărei disc
  a dispărut poate fi eliminată oricum, iar fișierele ei apar apoi ca lipsă.
- **Un dosar urmărit** este un dosar cu pachete pe care le aveți deja. Fiecare dosar din el devine un pachet, indexat
  acolo unde se află. Hoard nu schimbă, nu mută și nu șterge niciodată nimic acolo. Dacă doriți, montați-l doar
  pentru citire. Hoard îl parcurge la pornire și apoi la fiecare 15 minute (sau când apăsați **Scanați acum**),
  preluând dosarele noi, fișierele adăugate în sau eliminate dintr-un pachet și pachetele al căror dosar a dispărut.
  Un dosar care devine brusc gol sau inaccesibil este considerat un disc nemontat, iar pachetele lui sunt păstrate.
  Un pachet al cărui dosar lipsește este păstrat trei zile, cu etichetele și notele sale, în caz că dosarul revine.
  Tot ce ștergeți definitiv dintr-un pachet urmărit rămâne șters când dosarul este scanat din nou, chiar dacă
  fișierele lui sunt încă acolo.

O locație este orice dosar pe care îl poate vedea serverul, așa că în Docker montați-l mai întâi în container.

**Actualizare.** Bara de sus afișează o insignă când există o versiune mai nouă. Rulați
`docker compose pull && docker compose up -d` pentru a o instala; Hoard își actualizează baza de date la pornire,
salvând mai întâi o copie în `/config/backups`. Schimbați eticheta imaginii în `:0.8` pentru a primi doar versiunile
de corecție. Pentru actualizare automată fără ca Hoard să fie repornit în mijlocul unei încărcări, dați în
**Setări → Întreținere → Actualizări** o adresă de apelat: Hoard o apelează după ce apare o versiune și nu se întâmplă
nimic (API-ul HTTP al Watchtower, un webhook Portainer, un script; vedeți
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opțional).** Hoard randează miniaturile modelelor 3D pe GPU când vede unul și în software când nu vede.
Pagina de pachete arată care dintre ele (`GPU: ...` sau `No GPU found`). Pentru a transmite GPU-ul în container,
adăugați overlayul pentru hardware-ul dvs.:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Ambele overlayuri se găsesc în [deploy/](../../deploy/). Ele pun pe GPU și serverul de modele locale (Etichetare AI).
Fișierele pe un NAS, stocarea S3, accesul de la distanță prin Cloudflare sau Tailscale și copiile de rezervă sunt
descrise în [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Compilare din sursă

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Sau plasați arhiva zip în portal.

## Restul acestui README

Ce se întâmplă cu un pachet, cum ajung resursele într-un motor de joc, pachetele Unreal, căutarea, creditele, MCP,
stocarea, HTTPS, configurarea, securitatea, conturile și API-ul sunt descrise în [README-ul în engleză](../../README.md).

## Traduceri

Portalul și mesajele serverului Hoard sunt disponibile în 31 de limbi, alese în Setări → Aspect (implicit, limba
browserului). Primele traduceri au fost făcute cu ajutorul traducerii automate; corecturile de la vorbitori nativi sunt
binevenite. [docs/translations.md](../translations.md) explică cum.

## Licență

[MIT No Attribution](../../LICENSE) (MIT-0): folosiți, copiați, modificați și redistribuiți Hoard în orice scop, fără
a fi nevoie să păstrați notificarea privind drepturile de autor. Pachetele de resurse pe care le stocați în Hoard își
păstrează propriile licențe.
