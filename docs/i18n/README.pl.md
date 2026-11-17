# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · Polski · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Samodzielnie hostowana biblioteka zasobów do tworzenia gier. Wystarczy wrzucić całe paczki zasobów (pliki zip,
archiwa tar, foldery), a Hoard przechowa każdy plik raz, według jego skrótu, rozpozna, czym jest każdy plik, otaguje
go, powiąże siatki z ich teksturami i animacjami oraz udostępni wszystko do przeszukiwania z portalu WWW, z CLI,
z endpointu MCP dla agentów i przez opartą na pliku blokady komendę `hoard sync`, która pobiera dokładnie te pliki,
których potrzebuje projekt gry.

To coś jak sklep z zasobami, tylko bez kasy.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Biblioteka Hoard: modele i materiały w siatce, a obok otwarta skrzynia ze skarbem w podglądzie 3D, wraz z jej tagami i pobraniami" src="../img/hero-light.webp">
</picture>

<sub>Biblioteka demonstracyjna to grafiki na licencji CC0: modele z [Poly Haven](https://polyhaven.com) (glTF i USD)
i powierzchnie z [ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` tworzy te obrazy i kartę
do mediów społecznościowych.</sub>

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
przechowywanie, HTTPS, konfiguracja, bezpieczeństwo, konta i API są opisane w [angielskim README](../../README.md).

## Tłumaczenia

Komunikaty portalu i serwera Hoard są dostępne w 31 językach, do wyboru w Ustawienia → Wygląd (domyślnie język
przeglądarki). Pierwsze tłumaczenia powstały z pomocą maszynową; poprawki od rodzimych użytkowników języka są mile
widziane. [docs/translations.md](../translations.md) wyjaśnia, jak je zgłaszać.

## Licencja

[MIT No Attribution](../../LICENSE) (MIT-0): Hoard można używać, kopiować, modyfikować i rozpowszechniać w dowolnym
celu, bez obowiązku zachowania informacji o prawach autorskich. Paczki zasobów przechowywane w Hoard zachowują
własne licencje.
