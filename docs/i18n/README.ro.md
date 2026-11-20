<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · Română · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

</div>

<h3 align="center">Fiecare pachet de resurse pe care îl dețineți, căutabil, previzualizabil și la un clic de jocul dvs.</h3>

Hoard este o bibliotecă de resurse pentru jocuri, găzduită de dvs. Adăugați un zip, un pachet Unity, o descărcare de pe
Fab sau un dosar cu zece mii de fișiere, iar Hoard se apucă de treabă. Modelele se rotesc în 3D cu texturile lor,
hărțile de textură răzlețe devin materiale, sunetele își desenează forma de undă, foile de sprite-uri își redau
animațiile și totul primește etichete, așa că tastând `footsteps gravel` sau `rigged low-poly` găsiți ce căutați. Apoi
ajunge în Godot, Unity sau Unreal cu tot ce îi trebuie.

Fără abonament, fără încărcare în cloudul nimănui, fără preț pe utilizator: o singură imagine Docker pe propriul server
sau o aplicație pe propriul calculator și e a dvs. Gândiți-vă la el ca la un magazin de resurse fără casă de marcat.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Biblioteca Hoard: modele 3D într-o grilă, cu un cufăr cu comori deschis în previzualizarea 3D, lângă etichetele și descărcările sale" src="../img/hero-light.webp">
</picture>

### Vedeți totul înainte de a-l folosi

Modelele se deschid în 3D cu texturile, materialele și animațiile lor. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL și
voxelii sunt citiți chiar de Hoard, fără nicio aplicație 3D instalată; fișierele Blender intră prin Blender rulat
headless, iar pachetele Unreal arată ce a salvat editorul despre ele, fără Unreal. Miniaturile sunt desenate pe server,
încadrate de fiecare dată la fel, așa că un pachet întreg poate fi parcurs dintr-o privire.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Un candelabru afișat pe toată pagina: o vizualizare 3D mare lângă datele, etichetele, texturile legate și descărcările sale" src="../img/page-light.webp">
</picture>

### Hărțile de textură răzlețe devin materiale

`Rock_BaseColor.png`, `Rock_Normal.png` și `Rock_Roughness.png` devin un singur material Rock pe o sferă luminată. Hoard
înțelege denumirile din Megascans, Poly Haven, ambientCG, Substance și Unreal, inclusiv hărțile împachetate, precum și
fișierele `.mat` din Unity. Vizualizatorul pe tot ecranul arată un material pe o sferă, un cub sau un plan, la scara
reală, iar fiecare hartă poate fi pornită și oprită.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Un material de cărămidă în vizualizatorul pe tot ecranul, pe o sferă, cu comutatoare pentru hărțile de culoare, normale, rugozitate și înălțime" src="../img/material-light.webp">
</picture>

### Foi de sprite-uri care se redau

Atlasele din TexturePacker, Starling, libGDX, Cocos2d și Godot devin o singură foaie de sprite-uri care își redă
animațiile și își listează sprite-urile. Descărcați-o reîmpachetată pentru motorul de joc pe care îl folosiți: zece
formate, fiecare primind doar ce poate cuprinde.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="O foaie de sprite-uri a unui personaj afișată pe toată pagina, cu una dintre animații redându-se lângă etichetele și formatele de descărcare" src="../img/sheet-light.webp">
</picture>

### Sunete pe care le puteți vedea

Fiecare sunet primește o formă de undă colorată după cât de luminos sună, așa că o lovitură scurtă și o ambianță lungă
arată diferit înainte să le redați. Sunt citite metadatele bibliotecilor de sunete (categorii UCS, câmpuri Soundminer și
BaseHead, puncte de buclă, tempo, nota fundamentală), iar muzica MIDI și cea de tracker sunt randate, ca să se redea ca
orice alt sunet.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="O grilă de efecte sonore desenate ca forme de undă colorate, cu unul deschis care își arată playerul, etichetele și take-urile" src="../img/sound-light.webp">
</picture>

### O căutare care știe ce sunt lucrurile

Hoard etichetează totul singur, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`
și o categorie dintr-un arbore de resurse pentru jocuri. Căutați după cuvinte, după culoare sau după ce arată o imagine
și cereți oricărei texturi sau oricărui sunet „Mai multe ca aceasta”. Îndreptați-l spre orice model compatibil cu
OpenAI, local sau găzduit, și va scrie, de asemenea, nume și descrieri.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="O căutare după lemn: o masă de lemn, scoarță, scânduri și o suprafață de lemn împreună, cu filtre după tip și etichetă alături" src="../img/search-light.webp">
</picture>

### Direct în motorul de joc

- **Pluginuri de motor** pentru Godot 4, Unity, Unreal Engine 5 și Lumina: răsfoiți biblioteca din editor și apăsați Adăugați
  la proiect. Modelul vine cu texturile sale, la locul potrivit, importat.
- **`hoard sync`** aduce într-un proiect exact fișierele pe care acesta le numește, fixate într-un lockfile, ca fiecare
  coleg de echipă și fiecare mașină de build să primească aceiași octeți.
- **Descărcați ca** GLB, PNG, WAV, MP3 sau un format de foaie de sprite-uri, convertit pe loc, cu numele texturilor
  după convenția studioului dvs.
- **Agenții AI** caută, inspectează și preiau resurse prin MCP.

### Stocat o dată, păstrat în siguranță

Fiecare fișier este stocat o singură dată, după hash-ul său BLAKE3, așa că aceeași textură din cinci pachete ocupă
spațiul uneia singure, iar coruperea nu poate trece neobservată. Păstrați totul pe discul local sau în orice bucket S3.
Dosarele spre care îndreptați Hoard sunt citite, niciodată modificate. Conturile, tokenurile doar pentru citire,
autentificarea unică (single sign-on), HTTPS și HTTP/3 sunt incluse.

### Sau doar pe calculatorul dvs.

Nu rulați un server? Aplicația desktop pentru Windows, macOS și Linux este aceeași bibliotecă, într-o fereastră proprie.
Instalați-o, alegeți un dosar și funcționează offline: fără conturi, parole sau setări de server, nimic la care să poată
ajunge restul rețelei, iar cheile pe care le introduceți sunt păstrate în depozitul de chei al sistemului. Versiunile
noi se descarcă în fundal și se instalează când închideți aplicația.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Aplicația desktop Hoard într-o fereastră Windows: modelele bibliotecii într-o grilă, cu un felinar deschis în previzualizarea 3D, lângă etichetele și descărcările sale" src="../img/desktop-light.webp">
</picture>

<sub>Biblioteca demonstrativă conține grafică CC0: modele de pe [Poly Haven](https://polyhaven.com) (glTF și USD),
suprafețe de pe [ambientCG](https://ambientcg.com) și sunete, foi de sprite-uri și fonturi de la
[Kenney](https://kenney.nl). `node web/scripts/readme-images.mts` creează aceste imagini și cardul pentru rețelele
sociale.</sub>

## Instalarea aplicației desktop

Descărcați-o din [cea mai nouă versiune](https://github.com/Azazel-Labs/Hoard/releases/latest): `-setup.exe` pentru
Windows, `.dmg` pentru macOS pe Apple silicon, `.AppImage` sau `.deb` pentru Linux. Pe Windows se instalează doar pentru
dumneavoastră, fără să ceară drepturi de administrator. Programele de instalare nu sunt încă semnate de Apple sau
Microsoft, așa că Windows întreabă înainte de a rula programul de instalare (More info → Run anyway), iar macOS înainte
de a deschide aplicația prima dată (Configurări sistem → Confidențialitate și securitate → Deschide oricum).

La prima deschidere, vă întreabă unde să fie biblioteca. Tot restul este în Setări, ca pe un server.

După aceea se actualizează singură, fără să fie nevoie să apăsați nimic: o versiune nouă se descarcă în fundal și se
instalează când închideți aplicația, așa că data viitoare când deschideți Hoard, rulează deja versiunea nouă. Un `.deb`
se actualizează instalându-l pe următorul.

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
salvând mai întâi o copie în `/config/backups`. Schimbați eticheta imaginii în `:1.3` pentru a primi doar versiunile
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
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
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
stocarea, HTTPS, configurarea, securitatea, conturile, API-ul, precum și datele și cifrele sunt descrise în
[README-ul în engleză](../../README.md).

## Traduceri

Portalul și mesajele serverului Hoard sunt disponibile în 31 de limbi, alese în Setări → Aspect (implicit, limba
browserului). Primele traduceri au fost făcute cu ajutorul traducerii automate; corecturile de la vorbitori nativi sunt
binevenite. [docs/translations.md](../translations.md) explică cum.

## Licență

[MIT No Attribution](../../LICENSE) (MIT-0): folosiți, copiați, modificați și redistribuiți Hoard în orice scop, fără
a fi nevoie să păstrați notificarea privind drepturile de autor. Pachetele de resurse pe care le stocați în Hoard își
păstrează propriile licențe.
