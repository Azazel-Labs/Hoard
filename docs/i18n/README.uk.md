# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · Українська · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Бібліотека ресурсів для розробки ігор, яку ви розгортаєте на власному сервері. Додайте цілі пакети ресурсів (zip,
tar-архіви, теки), і Hoard збереже кожен файл один раз за його хешем, визначить, що це за файл, позначить його тегами,
пов’яже меші з їхніми текстурами й анімаціями та зробить усе це доступним для пошуку через вебпортал, CLI, кінцеву
точку MCP для агентів і команду `hoard sync` на основі lock-файлу, яка отримує саме ті файли, що потрібні вашому
ігровому проєкту.

Це як магазин ресурсів, тільки без каси.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Бібліотека Hoard: моделі й матеріали в сітці, а поруч у 3D-перегляді відкрита скриня зі скарбами, її теги й варіанти завантаження" src="../img/hero-light.webp">
</picture>

<sub>Демонстраційна бібліотека складається з робіт під CC0: моделі з [Poly Haven](https://polyhaven.com) (glTF і USD)
і поверхні з [ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` створює ці зображення й картку
для соцмереж.</sub>

## Встановлення через Docker

Hoard постачається одним образом для amd64 і arm64: `ghcr.io/azazel-labs/hoard`. Збережіть це як `compose.yml`
(це [deploy/compose.yml](../../deploy/compose.yml)):

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

Натомість кожен том може бути текою на тому диску, який йому найкраще підходить: `/mnt/big/hoard:/library` кладе ваші
файли на великий диск, а база даних залишається маленькою, і її легко зберігати в резервній копії. Теки, які ви
монтуєте, не потребують chown: Hoard бере на себе `/config` і `/cache` і працює від імені власника `/library` (задайте
`PUID` і `PGID`, щоб вибрати користувача самостійно).

Потім запустіть його:

```sh
docker compose up -d        # open http://<host>:4627
```

Під час першого відвідування вам запропонують створити обліковий запис, а потім відкриється коротке налаштування, яке
запитає, де зберігатиметься ваша бібліотека. Воно пропонує `/library`, тобто том вище; виберіть його й перетягніть
пакет на портал. Інші розташування можна додати згодом у **Налаштування → Розташування бібліотеки**:

- **Бібліотека, якою опікується Hoard**, отримує копії того, що ви вивантажуєте чи імпортуєте. У кожній позначено
  типи файлів, які вона приймає (аудіо на одному диску, моделі на іншому), а одна є резервною для всієї решти. Зміна
  того, що приймає розташування, стосується нових файлів; **Перемістити збережені файли відповідно** переносить те, що
  вже збережено. **Вилучити** спершу переносить файли розташування до інших, за типами, і вилучає його, щойно воно
  спорожніє. Перед видаленням старої копії кожен файл звіряється зі своїм хешем. Розташування, диск якого зник, можна
  вилучити й так, і тоді його файли показуються як відсутні.
- **Відстежувана тека** — це тека з пакетами, які у вас уже є. Кожна тека всередині неї стає пакетом, що індексується
  там, де лежить. Hoard ніколи нічого там не змінює, не переміщує й не видаляє. За бажання змонтуйте її лише для
  читання. Hoard переглядає її під час запуску і далі кожні 15 хвилин (або коли ви натискаєте **Сканувати зараз**),
  підхоплюючи нові теки, файли, додані до пакета чи вилучені з нього, і пакети, тека яких зникла. Теку, що раптом
  стала порожньою чи недоступною, вважають диском, який не змонтовано, і її пакети зберігаються. Пакет, тека якого
  відсутня, зберігається три дні разом із тегами й нотатками на випадок, якщо тека повернеться. Усе, що ви остаточно
  видалили з відстежуваного пакета, залишається видаленим після повторного сканування теки, хоча його файли досі там.

Розташування — це будь-яка тека, яку бачить сервер, тож у Docker спершу змонтуйте її в контейнер.

**Оновлення.** Коли виходить новіший випуск, у заголовку з’являється значок. Виконайте
`docker compose pull && docker compose up -d`, щоб його встановити; Hoard оновлює свою базу даних під час запуску,
спершу зберігши копію в `/config/backups`. Змініть тег образу на `:1.2`, щоб отримувати лише випуски з виправленнями.
Щоб оновлюватися автоматично й не перезапускати Hoard посеред вивантаження, вкажіть у
**Налаштування → Обслуговування → Оновлення** адресу для звернення: Hoard звертається до неї, щойно вийшов випуск і
нічого не відбувається (HTTP API Watchtower, вебхук Portainer, скрипт; див. [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (необов’язково).** Hoard рендерить мініатюри 3D-моделей на GPU, коли бачить його, а коли ні — програмно.
Сторінка пакетів показує, як саме (`GPU: ...` або `No GPU found`). Щоб передати GPU в контейнер, додайте накладку
для свого обладнання:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Обидві накладки є в [deploy/](../../deploy/). Вони також переносять на GPU локальний сервер моделей (тегування ШІ).
Файли на NAS, сховище S3, віддалений доступ через Cloudflare чи Tailscale і резервні копії описано в
[docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Збирання з вихідного коду

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Або перетягніть zip на портал.

## Решта цього README

Що відбувається з пакетом, як перенести ресурси в рушій, пакети Unreal, пошук, список авторів, MCP, сховище, HTTPS,
конфігурацію, безпеку, облікові записи та API описано в [англійському README](../../README.md).

## Переклади

Повідомлення порталу й сервера Hoard доступні 31 мовою; мову можна вибрати в «Налаштування → Вигляд» (типово — мова
браузера). Перші переклади зроблено за допомогою машинного перекладу; виправлення від носіїв мови вітаються.
[docs/translations.md](../translations.md) пояснює, як це зробити.

## Ліцензія

[MIT No Attribution](../../LICENSE) (MIT-0): використовуйте, копіюйте, змінюйте й поширюйте Hoard для будь-якої мети без
потреби зберігати повідомлення про авторські права. Пакети ресурсів, які ви зберігаєте в Hoard, мають власні ліцензії.
