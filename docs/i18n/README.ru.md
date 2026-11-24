<div align="center">

# <img src="../../web/public/favicon.svg" alt="" height="32" align="absmiddle"> Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· Русский&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">Все ваши паки ассетов: с поиском, с превью и в одном клике от вашей игры.</h3>

Hoard — самостоятельно размещаемая библиотека игровых ассетов. Закиньте в неё zip, пакет Unity, загрузку с Fab или
папку с десятью тысячами файлов, и Hoard возьмётся за дело. Модели вращаются в 3D вместе с текстурами, разрозненные
текстурные карты становятся материалами, звуки рисуют свои волны, листы спрайтов проигрывают свои анимации, и всё
получает теги, так что по запросу `footsteps gravel` или `rigged low-poly` нужное находится сразу. Затем оно попадает
в Godot, Unity или Unreal со всем, что ему нужно.

Без подписки, без загрузки в чьё-то облако, без оплаты за каждое рабочее место: один Docker-образ на вашем собственном
сервере или приложение на вашем собственном компьютере, и всё это ваше. Это как магазин ассетов, только без кассы.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Библиотека Hoard: 3D-модели в сетке, а рядом в 3D-превью открытый сундук с сокровищами, его теги и варианты скачивания" src="../img/hero-light.webp">
</picture>

### Всё видно ещё до использования

Модели открываются в 3D с текстурами, материалами и анимациями. FBX, glTF, OBJ, USD, SketchUp, Collada, X3D, VRM, 3MF, STL и воксели
Hoard читает сам, без установленных 3D-программ; файлы Blender проходят через Blender в headless-режиме, а паки Unreal
показывают, что о них сохранил редактор, без самого Unreal. Миниатюры рисуются на сервере и каждый раз кадрируются
одинаково, так что весь пак можно окинуть одним взглядом.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Люстра на отдельной странице: большой 3D-вид рядом с её сведениями, тегами, связанными текстурами и вариантами скачивания" src="../img/page-light.webp">
</picture>

### Разрозненные текстурные карты становятся материалами

`Rock_BaseColor.png`, `Rock_Normal.png` и `Rock_Roughness.png` превращаются в один материал Rock на освещённой сфере.
Hoard понимает схемы именования Megascans, Poly Haven, ambientCG, Substance и Unreal, включая упакованные карты, а
также файлы Unity `.mat`. Полноэкранный просмотрщик показывает материал на сфере, кубе или плоскости в реальном
масштабе, и каждую карту можно включить и выключить.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Кирпичный материал в полноэкранном просмотрщике на сфере, с переключателями карт цвета, нормалей, шероховатости и высоты" src="../img/material-light.webp">
</picture>

### Листы спрайтов, которые проигрываются

Атласы из TexturePacker, Starling, libGDX, Cocos2d и Godot становятся одним листом спрайтов, который проигрывает свои
анимации и перечисляет свои спрайты. Скачайте его перепакованным под любой движок, которым вы пользуетесь: десять
форматов, и в каждый попадает только то, что он умеет хранить.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Лист спрайтов персонажа на отдельной странице: одна из его анимаций проигрывается рядом с тегами и форматами скачивания" src="../img/sheet-light.webp">
</picture>

### Звуки, которые видно

Каждый звук получает волну, окрашенную по тому, насколько ярко он звучит, так что резкий удар и долгий эмбиент
выглядят по-разному ещё до прослушивания. Читаются метаданные звуковых библиотек (категории UCS, поля Soundminer и
BaseHead, точки петли, темп, основная нота), а MIDI и трекерная музыка рендерятся, чтобы играть как любой другой звук.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Сетка звуковых эффектов, нарисованных цветными волнами; один из них открыт, видны его плеер, теги и дубли" src="../img/sound-light.webp">
</picture>

### Поиск, который знает, что есть что

Hoard сам расставляет теги на всё, офлайн: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0` и категорию из дерева игровых ассетов. Ищите по словам, по цвету или по тому, что изображено на картинке,
а у любой текстуры или звука можно запросить «Похожие». Подключите любую OpenAI-совместимую модель, локальную или
облачную, и Hoard будет писать ещё и названия с описаниями.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Поиск дерева: деревянный стол, кора, доски и деревянная поверхность вместе, а рядом фильтры по типу и тегу" src="../img/search-light.webp">
</picture>

### Прямо в ваш движок

- **Плагины для движков** Godot 4, Unity, Unreal Engine 5 и Lumina: просматривайте библиотеку прямо в редакторе и нажмите
  Добавить в проект. Модель приезжает вместе с текстурами, сразу на своё место и уже импортированная.
- **`hoard sync`** забирает в проект ровно те файлы, которые он называет, закреплённые в lock-файле, так что каждый
  участник команды и каждая сборочная машина получают одни и те же байты.
- **Скачать как** GLB, PNG, WAV, MP3 или в формате листа спрайтов, с конвертацией на лету и именами текстур по
  соглашениям вашей студии.
- **ИИ-агенты** ищут, изучают и забирают ассеты через MCP и могут делать всё остальное, что умеет портал, с
  правами учётной записи или токена, под которыми входят.

### Хранится один раз, под надёжной защитой

Каждый файл хранится один раз по его хешу BLAKE3, так что одна и та же текстура в пяти наборах занимает место одной, а
повреждение не может остаться незамеченным. Храните всё на локальном диске или в любом бакете S3. Папки, на которые вы
указали Hoard, только читаются и никогда не изменяются. Учётные записи, токены только для чтения, единый вход (single
sign-on), HTTPS и HTTP/3 встроены.

### Или просто на вашем компьютере

Нет своего сервера? Настольное приложение для Windows, macOS и Linux — это та же библиотека, только в собственном окне.
Установите его, выберите папку, и оно работает офлайн: никаких учётных записей, паролей и настроек сервера, ничего, до
чего могла бы добраться остальная сеть, а введённые вами ключи хранятся в системном хранилище паролей. Новые версии
скачиваются в фоне и устанавливаются, когда вы выходите из приложения.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Настольное приложение Hoard в окне Windows: модели библиотеки в сетке, а рядом в 3D-превью открытый фонарь, его теги и варианты скачивания" src="../img/desktop-light.webp">
</picture>

<sub>Демонстрационная библиотека состоит из работ под CC0: модели с [Poly Haven](https://polyhaven.com) (glTF и USD),
поверхности с [ambientCG](https://ambientcg.com), а также звуки, листы спрайтов и шрифты от [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` создаёт эти картинки и карточку для соцсетей.</sub>

## Установка настольного приложения

Скачайте его из [последнего выпуска](https://github.com/Azazel-Labs/Hoard/releases/latest): `-setup.exe` для Windows,
`.dmg` для macOS на Apple silicon, `.AppImage` или `.deb` для Linux. В Windows оно устанавливается только для вас и не
просит прав администратора. Установщики пока не подписаны Apple или Microsoft, поэтому Windows переспросит перед
запуском установщика (Подробнее → Выполнить в любом случае), а macOS — перед первым открытием приложения (Системные
настройки → Конфиденциальность и безопасность → Все равно открыть).

При первом запуске оно спросит, где будет лежать библиотека. Всё остальное — в «Настройках», как и на сервере.

Дальше оно обновляется само, ничего нажимать не нужно: новый выпуск скачивается в фоне и устанавливается, когда вы
закрываете приложение, так что в следующий раз Hoard откроется уже в новой версии. `.deb` обновляется установкой
следующего.

## Установка через Docker

Hoard поставляется одним образом для amd64 и arm64: `ghcr.io/azazel-labs/hoard`. Сохраните это как `compose.yml`
(это [deploy/compose.yml](../../deploy/compose.yml)):

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

Вместо тома каждый из них может быть папкой на подходящем диске: `/mnt/big/hoard:/library` кладёт ваши файлы на
большой диск, а база данных остаётся маленькой, и её легко резервировать. Подключаемым папкам не нужен chown: Hoard
берёт на себя `/config` и `/cache` и работает от имени владельца `/library` (задайте `PUID` и `PGID`, чтобы выбрать
пользователя самостоятельно).

Затем запустите его:

```sh
docker compose up -d        # open http://<host>:4627
```

При первом визите вас попросят создать учётную запись, а затем откроется короткая настройка, которая спросит, где будет храниться библиотека. Она предложит `/library` — том выше; выберите
его и перетащите пак на портал. Другие места хранения можно добавить позже в **«Настройки → Места хранения»**:

- **Библиотека, которую хранит Hoard**, получает копии того, что вы загружаете или импортируете. В каждой отмечены
  виды файлов, которые она берёт (аудио на одном диске, модели на другом), и одна из них — место для всего остального.
  Изменение того, что берёт место хранения, касается новых файлов; **«Перенести сохранённые файлы»** переносит уже
  сохранённое. **«Удалить»** сначала переносит файлы места хранения в остальные, по видам, и удаляет его, когда оно
  опустеет. Перед удалением старой копии каждый файл сверяется со своим хешем. Место хранения, диск которого пропал,
  всё равно можно удалить, и тогда его файлы будут показаны как отсутствующие.
- **Отслеживаемая папка** — это папка с уже имеющимися у вас паками. Каждая папка внутри неё становится паком и
  индексируется там, где лежит. Hoard никогда ничего в ней не меняет, не перемещает и не удаляет. Если хотите,
  подключите её только для чтения. Hoard просматривает её при запуске и затем каждые 15 минут (или когда вы нажимаете
  **«Сканировать»**), подхватывая новые папки, файлы, добавленные в пак или удалённые из него, и паки, папка которых
  пропала. Папка, которая вдруг стала пустой или недоступной, считается диском, который не подключён, и её паки
  сохраняются. Пак, папка которого пропала, хранится три дня вместе с тегами и заметками на случай, если папка
  вернётся. То, что вы навсегда удалили из отслеживаемого пака, остаётся удалённым при повторном сканировании папки,
  хотя его файлы по-прежнему там.

Место хранения — это любая папка, которую видит сервер, поэтому в Docker сначала подключите её в контейнер.

**Обновление.** Когда выходит новый выпуск, в заголовке появляется значок. Выполните
`docker compose pull && docker compose up -d`, чтобы его установить; при запуске Hoard обновляет свою базу данных,
сначала сохранив копию в `/config/backups`. Измените тег образа на `:1.3`, чтобы получать только патч-выпуски. Чтобы
обновляться автоматически, не перезапуская Hoard посреди загрузки, укажите в **«Настройки → Обслуживание →
Обновления»** адрес для вызова: Hoard вызовет его, когда выпуск выйдет и ничего не будет происходить (HTTP API
Watchtower, вебхук Portainer, скрипт; см. [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (необязательно).** Hoard рендерит миниатюры 3D-моделей на GPU, если видит его, а если нет — программно.
Страница паков показывает, как именно (`GPU: ...` или `No GPU found`). Чтобы пробросить GPU, добавьте оверлей для
своего оборудования:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Оба оверлея лежат в [deploy/](../../deploy/). Они также переносят на GPU локальный сервер моделей (теги от ИИ). Файлы на NAS, хранилище S3, удалённый доступ через Cloudflare
или Tailscale и резервное копирование описаны в [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Сборка из исходников

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Или перетащите zip на портал.

## Остальная часть этого README

Что происходит с паком, как перенести ассеты в движок, паки Unreal, поиск, авторы, MCP, хранение, HTTPS, настройка,
безопасность, учётные записи, API, а также факты и цифры описаны в [README на английском](../../README.md).

## Переводы

Сообщения портала и сервера Hoard доступны на 31 языке; язык выбирается в «Настройки → Оформление» (по умолчанию —
язык браузера). Первые переводы были сделаны с помощью машинного перевода; исправления от носителей языка
приветствуются. Как это сделать, объясняет [docs/translations.md](../translations.md).

## Сообщество

Есть вопросы, идеи или хочется что-то показать? Заходите поздороваться в [Discord Azazel Labs](https://discord.gg/zf9ncS4EW). Об ошибках и пожеланиях лучше писать в [issues на GitHub](https://github.com/Azazel-Labs/Hoard/issues): там они не потеряются.

## Лицензия

[MIT No Attribution](../../LICENSE) (MIT-0): используйте, копируйте, изменяйте и распространяйте Hoard для любых
целей, без необходимости сохранять уведомление об авторских правах. Паки ассетов, которые вы храните в Hoard,
сохраняют собственные лицензии.
