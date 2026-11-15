# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · Српски · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Самостално хостована библиотека ресурса за развој игара. Убаците читаве пакете ресурса (zip, tarball архиве, фасцикле), а Hoard
чува сваку датотеку једном, по њеном хешу, утврђује шта је која датотека, означава је, повезује мреже с њиховим текстурама и
анимацијама и све то чини претраживим преко веб портала, CLI-ја, MCP крајње тачке за агенте и команде `hoard sync`, засноване на
lockfile датотеци, која повлачи тачно оне датотеке које су потребне вашем пројекту игре.

Замислите је као продавницу ресурса без плаћања на каси.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard библиотека: модели и материјали у мрежи, са отвореним ковчегом с благом у 3D прегледу поред његових ознака и преузимања" src="../img/hero-light.webp">
</picture>

<sub>Демо библиотека је CC0 уметност: модели са [Poly Haven](https://polyhaven.com)-а (glTF и USD) и површине са
[ambientCG](https://ambientcg.com)-а. `node web/scripts/readme-images.mts` прави ове слике и картицу за друштвене мреже.</sub>

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
ознаку слике у `:0.8` да бисте добијали само закрпе. Да бисте ажурирали аутоматски, а да се Hoard не поново покреће усред
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
конфигурација, безбедност, налози и API описани су у [README-у на енглеском](../../README.md).

## Преводи

Поруке Hoard-овог портала и сервера доступне су на 31 језику, а бирају се у Подешавања → Изглед (подразумевано језик
прегледача). Први преводи су настали уз помоћ машинског превођења; исправке изворних говорника су добродошле.
[docs/translations.md](../translations.md) објашњава како.

## Лиценца

[MIT No Attribution](../../LICENSE) (MIT-0): користите, копирајте, мењајте и поново дистрибуирајте Hoard у било коју сврху, без
обавезе да задржите обавештење о ауторским правима. Пакети ресурса које чувате у Hoard-у задржавају своје лиценце.
