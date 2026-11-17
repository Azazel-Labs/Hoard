# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · Polski · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Każda posiadana paczka zasobów: do przeszukania, do podejrzenia i o jedno kliknięcie od gry.</h3>

Hoard to samodzielnie hostowana biblioteka zasobów do gier. Wystarczy wrzucić plik zip, paczkę Unity, pobranie z Fab
albo folder z dziesięcioma tysiącami plików, a Hoard zabiera się do pracy. Modele obracają się w 3D razem
z teksturami, luźne mapy tekstur stają się materiałami, dźwięki rysują swoje przebiegi, arkusze sprite’ów odtwarzają
swoje animacje, a wszystko dostaje tagi, więc wystarczy wpisać `footsteps gravel` albo `rigged low-poly`, żeby to
znaleźć. Potem trafia to do Godota, Unity lub Unreala ze wszystkim, czego potrzebuje.

Bez abonamentu, bez wysyłania czegokolwiek do cudzej chmury, bez opłat za stanowisko: jeden obraz Dockera na własnym
serwerze albo aplikacja na własnym komputerze, i to na własność. To coś jak sklep z zasobami, tylko bez kasy.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Biblioteka Hoard: modele 3D w siatce, a obok otwarta skrzynia ze skarbem w podglądzie 3D, wraz z jej tagami i pobraniami" src="../img/hero-light.webp">
</picture>

### Wszystko widać, zanim się tego użyje

Modele otwierają się w 3D z teksturami, materiałami i animacjami. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL i woksele
Hoard czyta sam, bez instalowania żadnego programu 3D; pliki Blendera przechodzą przez Blendera w trybie headless,
a paczki Unreal pokazują to, co zapisał o nich edytor, bez Unreala. Miniatury są rysowane na serwerze i za każdym
razem kadrowane tak samo, więc całą paczkę da się przejrzeć jednym rzutem oka.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Żyrandol na całej stronie: duży widok 3D obok jego danych, tagów, powiązanych tekstur i pobrań" src="../img/page-light.webp">
</picture>

### Luźne mapy tekstur stają się materiałami

`Rock_BaseColor.png`, `Rock_Normal.png` i `Rock_Roughness.png` stają się jednym materiałem Rock na oświetlonej kuli.
Hoard rozumie nazewnictwo Megascans, Poly Haven, ambientCG, Substance i Unreal, łącznie ze spakowanymi mapami, a także
pliki `.mat` z Unity. Pełnoekranowa przeglądarka pokazuje materiał na kuli, sześcianie lub płaszczyźnie,
w rzeczywistej skali, a każdą mapę można włączyć i wyłączyć.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Materiał z cegieł w pełnoekranowej przeglądarce, na kuli, z przełącznikami map koloru, normalnych, chropowatości i wysokości" src="../img/material-light.webp">
</picture>

### Arkusze sprite’ów, które się odtwarzają

Atlasy z TexturePacker, Starling, libGDX, Cocos2d i Godot stają się jednym arkuszem sprite’ów, który odtwarza swoje
animacje i wymienia swoje sprite’y. Można go pobrać przepakowanego dla dowolnego silnika: dziesięć formatów, a każdy
dostaje tylko to, co potrafi pomieścić.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Arkusz sprite’ów postaci na całej stronie, z jedną z jego animacji odtwarzaną obok tagów i formatów pobierania" src="../img/sheet-light.webp">
</picture>

### Dźwięki, które widać

Każdy dźwięk dostaje przebieg pokolorowany według tego, jak jasno brzmi, więc ostre uderzenie i długi ambient wyglądają
inaczej, zanim się je odtworzy. Odczytywane są metadane bibliotek dźwięków (kategorie UCS, pola Soundminer i BaseHead,
punkty pętli, tempo, dźwięk podstawowy), a muzyka MIDI i trackerowa jest renderowana, więc odtwarza się jak każdy inny
dźwięk.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Siatka efektów dźwiękowych narysowanych jako kolorowe przebiegi, z jednym otwartym, pokazującym odtwarzacz, tagi i ujęcia" src="../img/sound-light.webp">
</picture>

### Wyszukiwanie, które wie, czym są rzeczy

Hoard sam taguje wszystko, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`
oraz kategorię z drzewa zasobów do gier. Można szukać po słowach, po kolorze albo po tym, co pokazuje obraz, a przy
każdej teksturze lub dźwięku poprosić o „Więcej podobnych”. Po wskazaniu dowolnego modelu zgodnego z OpenAI,
lokalnego lub hostowanego, Hoard pisze też nazwy i opisy.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Wyszukiwanie drewna: drewniany stół, kora, deski i drewniana powierzchnia razem, a obok filtry według rodzaju i tagu" src="../img/search-light.webp">
</picture>

### Prosto do silnika

- **Wtyczki do silników** Godot 4, Unity, Unreal Engine 5 i Lumina: bibliotekę przegląda się w edytorze i naciska Dodaj
  do projektu. Model trafia na miejsce razem z teksturami, od razu zaimportowany.
- **`hoard sync`** ściąga do projektu dokładnie te pliki, które on wymienia, przypięte w pliku blokady, więc każdy
  członek zespołu i każda maszyna budująca dostaje te same bajty.
- **Pobierz jako** GLB, PNG, WAV, MP3 lub format arkusza sprite’ów, z konwersją w locie i z nazwami tekstur zgodnymi
  z konwencją studia.
- **Agenci AI** wyszukują, sprawdzają i pobierają zasoby przez MCP.

### Zapisane raz, bezpiecznie przechowane

Każdy plik jest zapisywany raz, według skrótu BLAKE3, więc ta sama tekstura w pięciu pakietach zajmuje miejsce jednej,
a uszkodzenie nie może przejść niezauważone. Wszystko można trzymać na dysku lokalnym albo w dowolnym kubełku S3.
Foldery wskazane Hoardowi są tylko odczytywane, nigdy zmieniane. Konta, tokeny tylko do odczytu, logowanie
jednokrotne (SSO), HTTPS i HTTP/3 są wbudowane.

### Albo po prostu na komputerze

Bez serwera? Aplikacja desktopowa dla Windows, macOS i Linuksa to ta sama biblioteka we własnym oknie. Wystarczy ją
zainstalować i wybrać folder, a działa offline: bez kont, haseł i ustawień serwera, bez niczego, do czego mogłaby
dotrzeć reszta sieci, a wpisane klucze trafiają do systemowego pęku kluczy. Nowe wersje pobierają się w tle i instalują
przy zamykaniu aplikacji.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Aplikacja desktopowa Hoard w oknie Windows: modele z biblioteki w siatce, a obok otwarta w podglądzie 3D latarnia, wraz z jej tagami i pobraniami" src="../img/desktop-light.webp">
</picture>

<sub>Biblioteka demonstracyjna to grafiki na licencji CC0: modele z [Poly Haven](https://polyhaven.com) (glTF i USD),
powierzchnie z [ambientCG](https://ambientcg.com) oraz dźwięki, arkusze sprite’ów i fonty z [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` tworzy te obrazy i kartę do mediów społecznościowych.</sub>

## Instalacja aplikacji desktopowej

Można ją pobrać z [najnowszego wydania](https://github.com/Azazel-Labs/Hoard/releases/latest): plik `-setup.exe` dla
Windows, `.dmg` dla macOS na Macach z układem Apple, `.AppImage` lub `.deb` dla Linuksa. W Windows instaluje się tylko
dla bieżącego użytkownika, bez proszenia o uprawnienia administratora. Instalatory nie są jeszcze podpisane przez Apple
ani Microsoft, więc Windows pyta przed uruchomieniem instalatora (Więcej informacji → Uruchom mimo to), a macOS przed
pierwszym otwarciem aplikacji (Ustawienia systemowe → Prywatność i ochrona → Otwórz mimo to).

Przy pierwszym otwarciu aplikacja pyta, gdzie ma trafić biblioteka. Wszystko inne jest w Ustawieniach, tak jak na
serwerze.

Później aplikacja sama dba o aktualizacje i nie trzeba niczego klikać: nowa wersja pobiera się w tle i instaluje przy
zamknięciu, więc przy następnym otwarciu Hoard działa już w nowej wersji. Pakiet `.deb` aktualizuje się, instalując
kolejny.

## Instalacja przez Dockera

Hoard jest dostarczany jako jeden obraz dla amd64 i arm64: `ghcr.io/azazel-labs/hoard`. Należy zapisać poniższą
treść jako `compose.yml` (to plik [deploy/compose.yml](../../deploy/compose.yml)):

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

Każdy wolumin może zamiast tego być folderem na dowolnym, najlepiej pasującym dysku: `/mnt/big/hoard:/library`
umieszcza pliki na dużym dysku, a baza danych pozostaje mała i łatwa do skopiowania. Montowane foldery nie wymagają
chown: Hoard przejmuje `/config` i `/cache` i działa jako właściciel `/library` (aby samodzielnie wybrać użytkownika,
należy ustawić `PUID` i `PGID`).

Następnie należy go uruchomić:

```sh
docker compose up -d        # open http://<host>:4627
```

Przy pierwszej wizycie pojawia się prośba o utworzenie konta, a potem krótka konfiguracja z pytaniem, gdzie ma się
znajdować biblioteka. Proponuje `/library`, czyli powyższy wolumin; wystarczy go wybrać i upuścić paczkę na portal.
Kolejne lokalizacje można dodać później w **Ustawienia → Lokalizacje biblioteki**:

- **Biblioteka zarządzana przez Hoard** otrzymuje kopie przesyłanych lub importowanych plików. Każda ma zaznaczone
  rodzaje plików, które przyjmuje (dźwięk na jednym dysku, modele na innym), a jedna jest lokalizacją domyślną dla
  reszty. Zmiana tego, co przyjmuje lokalizacja, dotyczy nowych plików; **Przenieś zapisane pliki zgodnie
  z ustawieniami** przenosi to, co już zapisano. **Usuń** najpierw przenosi pliki lokalizacji do pozostałych, według
  rodzaju, i usuwa ją, gdy jest pusta. Przed usunięciem starej kopii każdy plik jest sprawdzany względem swojego
  skrótu. Lokalizację, której dysk zniknął, można mimo to usunąć, a jej pliki będą wtedy widoczne jako brakujące.
- **Obserwowany folder** to folder z już posiadanymi paczkami. Każdy folder w nim staje się paczką, indeksowaną tam,
  gdzie leży. Hoard nigdy niczego w nim nie zmienia, nie przenosi ani nie usuwa. Można go zamontować tylko do
  odczytu. Hoard przegląda go przy uruchomieniu, a potem co 15 minut (lub po naciśnięciu **Skanuj teraz**),
  wychwytując nowe foldery, pliki dodane do paczki lub z niej usunięte oraz paczki, których folder zniknął. Folder,
  który nagle jest pusty lub niedostępny, jest uznawany za niezamontowany dysk, a jego paczki zostają zachowane.
  Paczka, której folderu brakuje, jest przechowywana przez trzy dni, wraz z tagami i notatkami, na wypadek gdyby
  folder wrócił. Wszystko, co zostanie trwale usunięte z obserwowanej paczki, pozostaje usunięte po ponownym
  skanowaniu folderu, nawet jeśli jego pliki nadal tam są.

Lokalizacją może być dowolny folder widoczny dla serwera, więc w Dockerze należy go najpierw zamontować w kontenerze.

**Aktualizacja.** Gdy dostępna jest nowsza wersja, w nagłówku pojawia się plakietka. Aby ją zainstalować, należy
uruchomić `docker compose pull && docker compose up -d`; Hoard aktualizuje bazę danych przy uruchomieniu, najpierw
zapisując jej kopię w `/config/backups`. Aby otrzymywać tylko wydania poprawkowe, należy zmienić tag obrazu na
`:1.2`. Aby aktualizować automatycznie bez ponownego uruchamiania Hoard w trakcie przesyłania, należy podać w
**Ustawienia → Konserwacja → Aktualizacje** adres do wywołania: Hoard wywoła go, gdy pojawi się nowa wersja i nic
nie będzie się działo (API HTTP Watchtower, webhook Portainer, skrypt; zob. [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opcjonalnie).** Hoard renderuje miniatury modeli 3D na GPU, jeśli je widzi, a programowo, jeśli nie.
Strona paczek pokazuje, który sposób jest używany (`GPU: ...` lub `No GPU found`). Aby przekazać GPU do kontenera,
należy dodać nakładkę odpowiednią dla sprzętu:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Obie nakładki są dostępne w [deploy/](../../deploy/). Przenoszą one na GPU także lokalny serwer modeli (tagowanie AI). Pliki na NAS, magazyn S3, zdalny dostęp przez Cloudflare
lub Tailscale oraz kopie zapasowe opisuje [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Kompilacja ze źródeł

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Można też upuścić plik zip na portal.

## Reszta tego README

Co dzieje się z paczką, przenoszenie zasobów do silnika, paczki Unreal, wyszukiwanie, lista autorów, MCP,
przechowywanie, HTTPS, konfiguracja, bezpieczeństwo, konta, API oraz fakty i liczby są opisane w
[angielskim README](../../README.md).

## Tłumaczenia

Komunikaty portalu i serwera Hoard są dostępne w 31 językach, do wyboru w Ustawienia → Wygląd (domyślnie język
przeglądarki). Pierwsze tłumaczenia powstały z pomocą maszynową; poprawki od rodzimych użytkowników języka są mile
widziane. [docs/translations.md](../translations.md) wyjaśnia, jak je zgłaszać.

## Licencja

[MIT No Attribution](../../LICENSE) (MIT-0): Hoard można używać, kopiować, modyfikować i rozpowszechniać w dowolnym
celu, bez obowiązku zachowania informacji o prawach autorskich. Paczki zasobów przechowywane w Hoard zachowują
własne licencje.
