# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · Čeština · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Každý balíček assetů, který vlastníte: prohledávatelný, s náhledem a jedno kliknutí od vaší hry.</h3>

Hoard je knihovna herních assetů, kterou si provozujete sami. Vhoďte do ní zip, balíček pro Unity, soubor stažený z Fabu
nebo složku s deseti tisíci soubory a Hoard se pustí do práce. Modely se otáčejí ve 3D se svými texturami, z volných
texturových map se stanou materiály, zvuky vykreslí svůj průběh, listy spritů přehrají své animace a vše dostane štítky,
takže když napíšete `footsteps gravel` nebo `rigged low-poly`, najdete to. Pak to přistane v Godotu, Unity nebo Unrealu
se vším, co potřebuje.

Žádné předplatné, žádné nahrávání do cizího cloudu, žádné ceny za uživatele: jeden image pro Docker na vašem vlastním
serveru, nebo aplikace na vašem vlastním počítači, a patří vám. Představte si ho jako obchod s assety bez pokladny.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Knihovna Hoardu: 3D modely v mřížce, vedle toho otevřená truhla s pokladem ve 3D náhledu se svými štítky a stahováním" src="../img/hero-light.webp">
</picture>

### Uvidíte všechno dřív, než to použijete

Modely se otevírají ve 3D se svými texturami, materiály a animacemi. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL a voxely
čte Hoard sám, bez nainstalované 3D aplikace; soubory Blenderu přicházejí přes Blender spuštěný bez rozhraní a balíčky z
Unrealu ukazují, co o nich editor uložil, a to bez Unrealu. Náhledy se kreslí na serveru, pokaždé stejně zarámované,
takže celý balíček přelétnete jediným pohledem.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Lustr na celé stránce: velký 3D pohled vedle jeho údajů, štítků, propojených textur a stahování" src="../img/page-light.webp">
</picture>

### Z volných texturových map se stanou materiály

`Rock_BaseColor.png`, `Rock_Normal.png` a `Rock_Roughness.png` se stanou jedním materiálem Rock na osvětlené kouli.
Hoard rozumí pojmenování z Megascans, Poly Haven, ambientCG, Substance a Unrealu, včetně sloučených map, a také souborům
`.mat` z Unity. Celoobrazovkový prohlížeč ukáže materiál na kouli, krychli nebo rovině, ve skutečném měřítku, a každou
mapu lze zapnout a vypnout.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Cihlový materiál v celoobrazovkovém prohlížeči na kouli, s přepínači pro mapy barvy, normál, drsnosti a výšky" src="../img/material-light.webp">
</picture>

### Listy spritů, které se přehrávají

Atlasy z nástrojů TexturePacker, Starling, libGDX, Cocos2d a Godot se stanou jedním listem spritů, který přehrává své
animace a vypisuje své sprity. Stáhněte si ho znovu zabalený pro engine, který právě používáte: deset formátů a každému
se předá jen to, co dokáže pojmout.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="List spritů postavy na celé stránce, jedna z jeho animací se přehrává vedle jeho štítků a formátů ke stažení" src="../img/sheet-light.webp">
</picture>

### Zvuky, které uvidíte

Každý zvuk dostane průběh obarvený podle toho, jak jasně zní, takže ostrý úder a dlouhá atmosféra vypadají jinak ještě
předtím, než je přehrajete. Čtou se metadata zvukových knihoven (kategorie UCS, pole Soundminer a BaseHead, body smyčky,
tempo, základní tón) a hudba MIDI a z trackerů se vyrenderuje, takže se přehrává jako jakýkoli jiný zvuk.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Mřížka zvukových efektů vykreslených jako barevné průběhy, jeden z nich otevřený s přehrávačem, štítky a variantami nahrávky" src="../img/sound-light.webp">
</picture>

### Hledání, které ví, co věci jsou

Hoard vše oštítkuje sám, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0` a
kategorii ze stromu herních assetů. Hledejte podle slov, podle barvy nebo podle toho, co obrázek ukazuje, a u kterékoli
textury nebo zvuku si řekněte o „Další podobné“. Napojte ho na libovolný model kompatibilní s OpenAI, místní nebo
hostovaný, a bude psát i názvy a popisy.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Hledání „wood“: dřevěný stůl, kůra, prkna a dřevěný povrch pohromadě, vedle nich filtry podle druhu a štítku" src="../img/search-light.webp">
</picture>

### Přímo do vašeho enginu

- **Pluginy pro enginy** Godot 4, Unity, Unreal Engine 5 a Lumina: procházejte knihovnu přímo v editoru a stiskněte Přidat do
  projektu. Model dorazí se svými texturami, na správné místo a naimportovaný.
- **`hoard sync`** stáhne do projektu přesně ty soubory, které projekt uvádí, zafixované v lockfilu, takže každý člen
  týmu i každý build server dostane tytéž bajty.
- **Stáhnout jako** GLB, PNG, WAV, MP3 nebo formát listu spritů, převedené za běhu, s názvy textur podle konvence vašeho
  studia.
- **AI agenti** hledají, prohlížejí a stahují assety přes MCP.

### Uloženo jednou, v bezpečí

Každý soubor se uloží jen jednou podle svého hashe BLAKE3, takže stejná textura v pěti balících zabere místo jedné a
poškození nemůže zůstat nepovšimnuto. Ukládejte na místní disk nebo do libovolného bucketu S3. Složky, na které Hoard
nasměrujete, se jen čtou, nikdy nemění. Účty, tokeny jen pro čtení, jednotné přihlášení, HTTPS a HTTP/3 jsou vestavěné.

### Nebo prostě na vašem počítači

Neprovozujete server? Desktopová aplikace pro Windows, macOS a Linux je stejná knihovna ve vlastním okně. Nainstalujte
ji, vyberte složku a funguje offline: žádné účty, hesla ani nastavení serveru, nic, na co by dosáhl zbytek sítě, a
klíče, které zadáte, zůstávají v systémové klíčence. Nové verze se stahují na pozadí a instalují se, když aplikaci
ukončíte.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Desktopová aplikace Hoard v okně Windows: modely knihovny v mřížce, vedle toho otevřená lucerna ve 3D náhledu se svými štítky a stahováním" src="../img/desktop-light.webp">
</picture>

<sub>Ukázková knihovna je grafika pod CC0: modely z [Poly Haven](https://polyhaven.com) (glTF a USD), povrchy z
[ambientCG](https://ambientcg.com) a zvuky, listy spritů a písma od [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` vytváří tyto obrázky i kartu pro sociální sítě.</sub>

## Instalace desktopové aplikace

Stáhněte si ji z [nejnovějšího vydání](https://github.com/Azazel-Labs/Hoard/releases/latest): `-setup.exe` pro Windows,
`.dmg` pro macOS na Apple silicon, `.AppImage` nebo `.deb` pro Linux. Ve Windows se nainstaluje jen pro vás a o
oprávnění správce nežádá. Instalátory zatím nejsou podepsané společnostmi Apple ani Microsoft, takže Windows se před
spuštěním instalátoru zeptají (Další informace → Přesto spustit) a macOS před prvním otevřením aplikace (Nastavení
systému → Soukromí a zabezpečení → Přesto otevřít).

Při prvním spuštění se zeptá, kam knihovnu uložit. Všechno ostatní najdete v Nastavení, stejně jako na serveru.

Pak se sama udržuje aktuální a nemusíte na nic klikat: nová verze se stáhne na pozadí a nainstaluje se, když aplikaci
ukončíte, takže až Hoard příště otevřete, poběží už nová verze. Balíček `.deb` aktualizujete instalací dalšího.

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
image na `:1.2`, chcete-li jen opravné verze. Chcete-li aktualizovat automaticky, aniž by se Hoard restartoval uprostřed
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
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
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
konfiguraci, zabezpečení, účty, API a fakta a čísla popisuje [anglické README](../../README.md).

## Překlady

Portál a zprávy serveru Hoardu jsou k dispozici v 31 jazycích, které vybíráte v Nastavení → Vzhled (ve výchozím stavu
jazyk prohlížeče). První překlady vznikly s pomocí strojového překladu; opravy od rodilých mluvčích jsou vítány. Jak na to, vysvětluje [docs/translations.md](../translations.md).

## Licence

[MIT No Attribution](../../LICENSE) (MIT-0): Hoard můžete používat, kopírovat, upravovat a dále šířit k jakémukoli účelu,
aniž byste museli zachovat upozornění na autorská práva. Balíčky assetů, které v Hoardu ukládáte, si ponechávají své vlastní licence.
