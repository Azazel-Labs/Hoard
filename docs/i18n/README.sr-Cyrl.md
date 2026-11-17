# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · Српски · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

<h3 align="center">Сваки пакет ресурса који поседујете, претражив, с прегледом и један клик од ваше игре.</h3>

Hoard је самостално хостована библиотека ресурса за игре. Убаците zip, Unity пакет, преузимање са Fab-а или фасциклу с
десет хиљада датотека, и Hoard креће на посао. Модели се окрећу у 3D-у са својим текстурама, растурене мапе текстура
постају материјали, звуци исцртавају своје таласне облике, листови спрајтова пуштају своје анимације, а све добија
ознаке, па куцање `footsteps gravel` или `rigged low-poly` проналази оно што тражите. Затим све стиже у Godot, Unity
или Unreal са свиме што му треба.

Без претплате, без отпремања у било чији облак, без цене по кориснику: једна Docker слика на вашем серверу, или
апликација на вашем рачунару, и ваша је. Замислите је као продавницу ресурса без плаћања на каси.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard библиотека: 3D модели у мрежи, са отвореним ковчегом с благом у 3D прегледу поред његових ознака и преузимања" src="../img/hero-light.webp">
</picture>

### Видите све пре него што га употребите

Модели се отварају у 3D-у са својим текстурама, материјалима и анимацијама. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL и
вокселе Hoard чита сам, без инсталираног 3D програма; Blender датотеке улазе преко Blender-а покренутог без интерфејса
(headless), а Unreal пакети показују шта је едитор сачувао о њима, без Unreal-а. Сличице се цртају на серверу, сваки
пут уоквирене на исти начин, па се цео пакет може прегледати на први поглед.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Лустер приказан на целој страници: велики 3D приказ поред његових података, ознака, повезаних текстура и преузимања" src="../img/page-light.webp">
</picture>

### Растурене мапе текстура постају материјали

`Rock_BaseColor.png`, `Rock_Normal.png` и `Rock_Roughness.png` постају један Rock материјал на осветљеној сфери. Hoard
разуме именовање које користе Megascans, Poly Haven, ambientCG, Substance и Unreal, укључујући спаковане мапе, као и
Unity `.mat` датотеке. Прегледач преко целог екрана приказује материјал на сфери, коцки или равни, у стварној
величини, а свака мапа може да се укључи и искључи.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Материјал од цигле у прегледачу преко целог екрана, на сфери, са прекидачима за мапе боје, нормала, храпавости и висине" src="../img/material-light.webp">
</picture>

### Листови спрајтова који се пуштају

Атласи које праве TexturePacker, Starling, libGDX, Cocos2d и Godot постају један лист спрајтова који пушта своје
анимације и набраја своје спрајтове. Преузмите га препакованог за који год енџин користите: десет формата, а сваки
добија само оно што може да садржи.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Лист спрајтова лика приказан на целој страници, са једном од његових анимација која се пушта поред ознака и формата за преузимање" src="../img/sheet-light.webp">
</picture>

### Звуци које можете да видите

Сваки звук добија таласни облик обојен према томе колико светло звучи, па оштар ударац и дуга амбијентална подлога
изгледају различито пре него што их пустите. Читају се метаподаци звучних библиотека (UCS категорије, Soundminer и
BaseHead поља, тачке петље, темпо, основни тон), а MIDI и тракерска музика се рендерују како би се пуштале као сваки
други звук.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Мрежа звучних ефеката исцртаних као обојени таласни облици, са једним отвореним који приказује свој плејер, ознаке и снимке" src="../img/sound-light.webp">
</picture>

### Претрага која зна шта је шта

Hoard сам означава све, ван мреже: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`, и
категорију из стабла ресурса за игре. Претражујте по речима, по боји или по ономе што слика приказује, и тражите од
било које текстуре или звука „Још сличних“. Усмерите га на било који модел компатибилан са OpenAI-јем, локални или
хостовани, и писаће и називе и описе.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Претрага за дрво: дрвени сто, кора, даске и дрвена површина заједно, са филтерима по врсти и ознаци поред њих" src="../img/search-light.webp">
</picture>

### Право у ваш енџин

- **Додаци за енџине** Godot 4, Unity, Unreal Engine 5 и Lumina: прегледајте библиотеку унутар едитора и притисните Додај у
  пројекат. Модел стиже са својим текстурама, на правом месту, већ увезен.
- **`hoard sync`** повлачи у пројекат тачно оне датотеке које он наводи, закључане у lockfile датотеци, тако да сваки
  члан тима и свака машина за изградњу добијају исте бајтове.
- **Преузми као** GLB, PNG, WAV, MP3 или формат листа спрајтова, уз конверзију у ходу и називе текстура по конвенцији
  вашег студија.
- **AI агенти** претражују, прегледају и преузимају ресурсе преко MCP-а.

### Сачувано једном, чувано безбедно

Свака датотека се чува једном, по свом BLAKE3 хешу, па иста текстура у пет пакета заузима простор једне, а оштећење не
може проћи непримећено. Држите све на локалном диску или у било ком S3 бакету. Фасцикле на које усмерите Hoard се
читају, а никада не мењају. Налози, токени само за читање, јединствена пријава (single sign-on), HTTPS и HTTP/3 су
уграђени.

### Или само на вашем рачунару

Не покрећете сервер? Десктоп апликација за Windows, macOS и Linux је иста библиотека, у сопственом прозору. Инсталирајте
је, изаберите фасциклу и ради ван мреже: без налога, лозинки и подешавања сервера, ништа до чега остатак мреже може да
допре, а кључеви које унесете чувају се у системском складишту лозинки. Нова издања се преузимају у позадини и
инсталирају када изађете из апликације.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Hoard десктоп апликација у Windows прозору: модели из библиотеке у мрежи, са отвореним фењером у 3D прегледу поред његових ознака и преузимања" src="../img/desktop-light.webp">
</picture>

<sub>Демо библиотека је CC0 уметност: модели са [Poly Haven](https://polyhaven.com)-а (glTF и USD), површине са
[ambientCG](https://ambientcg.com)-а, и звуци, листови спрајтова и фонтови са [Kenney](https://kenney.nl)-ја.
`node web/scripts/readme-images.mts` прави ове слике и картицу за друштвене мреже.</sub>

## Инсталација десктоп апликације

Преузмите је из [најновијег издања](https://github.com/Azazel-Labs/Hoard/releases/latest): `.msi` за Windows, `.dmg` за
macOS на Apple silicon рачунарима, `.AppImage` или `.deb` за Linux. Инсталационе програме Apple и Microsoft још нису
потписали, па Windows пита пре покретања `.msi` датотеке (More info → Run anyway), а macOS пре првог отварања апликације
(System Settings → Privacy & Security → Open Anyway).

Када се први пут отвори, пита где ће бити библиотека. Све остало је у Подешавањима, као на серверу.

## Инсталација помоћу Docker-а

Hoard се испоручује као једна слика за amd64 и arm64: `ghcr.io/azazel-labs/hoard`. Сачувајте ово као `compose.yml` (то је
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

Сваки волумен уместо тога може бити фасцикла на диску који му највише одговара: `/mnt/big/hoard:/library` ставља ваше датотеке на
велики диск, док база података остаје мала и лака за резервне копије. Фасциклама које монтирате није потребан chown: Hoard преузима
`/config` и `/cache` и ради као власник фасцикле `/library` (подесите `PUID` и `PGID` ако сами желите да изаберете корисника).

Затим га покрените:

```sh
docker compose up -d        # open http://<host>:4627
```

При првој посети тражи се да направите налог, а затим се отвара кратко подешавање које пита где ће бити ваша библиотека. Предлаже
`/library`, волумен изнад; изаберите га и превуците пакет на портал. Касније можете да додате још локација, у
**Подешавања → Локације библиотеке**:

- **Библиотека коју Hoard чува** добија копије онога што отпремате или увозите. Свака има означене врсте датотека које прима
  (звук на једном диску, модели на другом), а једна је локација за све остало. Промена онога што локација прима
  важи за нове датотеке; **Премести сачуване датотеке у складу с тим** премешта оно што је већ сачувано. **Уклони** прво
  премешта датотеке локације на остале, према врсти, и уклања је кад се испразни. Свака датотека се проверава према свом хешу пре
  него што се стара копија обрише. Локација чији је диск нестао може ипак да се уклони, а њене датотеке се тада приказују као
  недостајуће.
- **Праћена фасцикла** је фасцикла с пакетима које већ имате. Свака фасцикла у њој постаје пакет, индексиран тамо где
  јесте. Hoard никад ништа не мења, премешта ни брише тамо. Ако желите, монтирајте је само за читање. Hoard је прегледа
  при покретању и затим на сваких 15 минута (или кад притиснете **Скенирај сада**) и проналази нове фасцикле, датотеке додате у
  пакет или уклоњене из њега и пакете чија је фасцикла нестала. Фасцикла која је изненада празна или недоступна сматра се
  диском који није монтиран, и њени пакети се задржавају. Пакет чија фасцикла недостаје задржава се три дана, с ознакама и
  белешкама, за случај да се фасцикла врати. Све што трајно обришете из праћеног пакета остаје обрисано кад се фасцикла
  поново скенира, иако су његове датотеке и даље тамо.

Локација је било која фасцикла коју сервер види, па је у Docker-у прво монтирајте у контејнер.

**Ажурирање.** Горња трака приказује ознаку кад постоји новије издање. Покрените `docker compose pull && docker compose up -d`
да бисте га преузели; Hoard надограђује своју базу података при покретању, а пре тога чува копију у `/config/backups`. Промените
ознаку слике у `:1.2` да бисте добијали само закрпе. Да бисте ажурирали аутоматски, а да се Hoard не поново покреће усред
отпремања, у **Подешавања → Одржавање → Ажурирања** унесите адресу за позивање: Hoard је позива кад изађе издање и кад се ништа
не дешава (HTTP API Watchtower-а, Portainer вебхук, скрипта; погледајте [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (опционо).** Hoard исцртава сличице 3D модела на GPU-у кад га види, а софтверски кад га не види.
Страница пакета показује шта се користи (`GPU: ...` или `No GPU found`). Да бисте проследили GPU, додајте overlay датотеку за свој
хардвер:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Обе overlay датотеке преузмите из [deploy/](../../deploy/). Оне на GPU стављају и локални сервер модела (означавање помоћу ВИ).
Датотеке на NAS-у, S3 складиште, даљински приступ преко Cloudflare-а или Tailscale-а и резервне копије описани су у
[docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Изградња из изворног кода

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Или превуците zip на портал.

## Остатак овог README-а

Шта се дешава с пакетом, пребацивање ресурса у енџин, Unreal пакети, претрага, заслуге, MCP, складиште, HTTPS,
конфигурација, безбедност, налози, API, као и чињенице и бројке описани су у [README-у на енглеском](../../README.md).

## Преводи

Поруке Hoard-овог портала и сервера доступне су на 31 језику, а бирају се у Подешавања → Изглед (подразумевано језик
прегледача). Први преводи су настали уз помоћ машинског превођења; исправке изворних говорника су добродошле.
[docs/translations.md](../translations.md) објашњава како.

## Лиценца

[MIT No Attribution](../../LICENSE) (MIT-0): користите, копирајте, мењајте и поново дистрибуирајте Hoard у било коју сврху, без
обавезе да задржите обавештење о ауторским правима. Пакети ресурса које чувате у Hoard-у задржавају своје лиценце.
