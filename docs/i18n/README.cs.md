# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · Čeština · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Knihovna assetů pro vývoj her, kterou si provozujete sami. Vhoďte do ní celé balíčky assetů (zipy, tarbally, složky)
a Hoard uloží každý soubor jen jednou podle jeho hashe, zjistí, co který soubor je, oštítkuje ho, propojí meshe s jejich
texturami a animacemi a vše zpřístupní k hledání z webového portálu, z CLI, přes endpoint MCP pro agenty a pomocí
`hoard sync` založeného na lockfilu, který stáhne přesně ty soubory, které váš herní projekt potřebuje.

Představte si ho jako obchod s assety bez pokladny.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Knihovna Hoardu: modely a materiály v mřížce, vedle toho otevřená truhla s pokladem ve 3D náhledu se svými štítky a stahováním" src="../img/hero-light.webp">
</picture>

<sub>Ukázková knihovna je grafika pod CC0: modely z [Poly Haven](https://polyhaven.com) (glTF a USD) a povrchy z
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` vytváří tyto obrázky i kartu pro sociální sítě.</sub>

## Instalace přes Docker

Hoard se dodává jako jeden image pro amd64 a arm64: `ghcr.io/azazel-labs/hoard`. Uložte toto jako `compose.yml` (je to
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

Každý svazek může být místo toho složka na disku, který se mu hodí: `/mnt/big/hoard:/library` dá vaše soubory na velký
disk, zatímco databáze zůstane malá a snadno se zálohuje. Připojené složky nepotřebují chown: Hoard převezme
`/config` a `/cache` a běží jako vlastník `/library` (uživatele si sami vyberete nastavením `PUID` a `PGID`).

Pak ho spusťte:

```sh
docker compose up -d        # open http://<host>:4627
```

První návštěva vás požádá o vytvoření účtu a pak otevře krátké nastavení, které se zeptá, kde má být vaše knihovna. Navrhne `/library`, svazek výše; vyberte
ho a přetáhněte na portál balíček. Další umístění přidáte později v **Nastavení → Umístění knihovny**:

- **Knihovna, kterou Hoard spravuje**, dostává kopie toho, co nahrajete nebo importujete. Každá má zaškrtnuté druhy
  souborů, které přijímá (zvuk na jednom disku, modely na jiném), a jedna je záchytné umístění pro zbytek. Změna toho,
  co umístění přijímá, platí pro nové soubory; **Přesunout uložené soubory podle nastavení** přesune to, co už je uložené.
  **Odebrat** nejdřív přesune soubory umístění podle druhu do ostatních a umístění odebere, jakmile je prázdné. Každý
  soubor se před smazáním staré kopie ověří podle svého hashe. Umístění, jehož disk zmizel, lze odebrat i tak a jeho
  soubory se pak zobrazí jako chybějící.
- **Sledovaná složka** je složka s balíčky, které už máte. Každá složka v ní se stane balíčkem indexovaným tam, kde je.
  Hoard v ní nikdy nic nemění, nepřesouvá ani nemaže. Klidně ji připojte jen pro čtení. Hoard ji prochází při startu
  a pak každých 15 minut (nebo když stisknete **Prohledat teď**) a zachytí nové složky, soubory přidané do balíčku nebo
  z něj odebrané a balíčky, jejichž složka zmizela. Složka, která je náhle prázdná nebo nedostupná, se považuje za
  nepřipojený disk a její balíčky zůstanou. Balíček, jehož složka chybí, se i se štítky a poznámkami uchovává tři dny,
  kdyby se složka vrátila. Cokoli trvale smažete ze sledovaného balíčku, zůstane smazané i po novém prohledání složky,
  přestože jeho soubory tam stále jsou.

Umístění je jakákoli složka, kterou server vidí, takže v Dockeru ji nejdřív připojte do kontejneru.

**Aktualizace.** Když existuje novější verze, horní lišta zobrazí odznak. Spusťte `docker compose pull && docker compose up -d`
a nainstalujete ji; Hoard při startu aktualizuje svou databázi a předtím uloží kopii do `/config/backups`. Změňte tag
image na `:0.8`, chcete-li jen opravné verze. Chcete-li aktualizovat automaticky, aniž by se Hoard restartoval uprostřed
nahrávání, zadejte v **Nastavení → Údržba → Aktualizace** adresu k volání: Hoard ji zavolá, jakmile vyjde nová verze
a nic se neděje (HTTP API Watchtoweru, webhook Portaineru, skript; viz [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (volitelné).** Hoard vykresluje miniatury 3D modelů na GPU, když nějakou vidí, a softwarově, když ne.
Stránka balíčků ukazuje, jak to je (`GPU: ...` nebo `No GPU found`). Chcete-li GPU zpřístupnit, přidejte overlay pro svůj
hardware:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Oba overlaye najdete v [deploy/](../../deploy/). Na GPU přesunou také místní server modelů (štítkování pomocí AI). Soubory na NAS, úložiště S3, vzdálený přístup přes Cloudflare
nebo Tailscale a zálohy popisuje [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Sestavení ze zdrojového kódu

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Nebo zip přetáhněte na portál.

## Zbytek tohoto README

Co se s balíčkem děje, jak dostat assety do enginu, balíčky z Unrealu, hledání, poděkování, MCP, úložiště, HTTPS,
konfiguraci, zabezpečení, účty a API popisuje [anglické README](../../README.md).

## Překlady

Portál a zprávy serveru Hoardu jsou k dispozici v 31 jazycích, které vybíráte v Nastavení → Vzhled (ve výchozím stavu
jazyk prohlížeče). První překlady vznikly s pomocí strojového překladu; opravy od rodilých mluvčích jsou vítány. Jak na to, vysvětluje [docs/translations.md](../translations.md).

## Licence

[MIT No Attribution](../../LICENSE) (MIT-0): Hoard můžete používat, kopírovat, upravovat a dále šířit k jakémukoli účelu,
aniž byste museli zachovat upozornění na autorská práva. Balíčky assetů, které v Hoardu ukládáte, si ponechávají své vlastní licence.
