<div align="center">

# <img src="../../web/public/favicon.svg" alt="" height="32" align="absmiddle"> Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· Українська&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">Кожен ваш пакет ресурсів: з пошуком, попереднім переглядом і за один клік від вашої гри.</h3>

Hoard — це бібліотека ігрових ресурсів, яку ви розгортаєте на власному сервері. Додайте zip, пакет Unity, завантаження з
Fab або теку з десятьма тисячами файлів, і Hoard візьметься до роботи. Моделі обертаються в 3D разом зі своїми
текстурами, розрізнені текстурні карти стають матеріалами, звуки малюють свої хвильові форми, аркуші спрайтів програють
свої анімації, а все разом отримує теги, тож щоб знайти потрібне, достатньо ввести `footsteps gravel` або
`rigged low-poly`. Потім ресурс потрапляє в Godot, Unity чи Unreal з усім необхідним.

Жодної підписки, жодного завантаження в чиюсь хмару, жодної оплати за робоче місце: один образ Docker на вашому власному
сервері або застосунок на вашому власному комп’ютері — і він ваш. Це як магазин ресурсів, тільки без каси.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Бібліотека Hoard: 3D-моделі в сітці, а поруч у 3D-перегляді відкрита скриня зі скарбами, її теги й варіанти завантаження" src="../img/hero-light.webp">
</picture>

### Побачте все, перш ніж використовувати

Моделі відкриваються в 3D разом із текстурами, матеріалами й анімаціями. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL і
воксельні моделі Hoard читає сам, без жодної встановленої 3D-програми; файли Blender проходять через Blender без
графічного інтерфейсу, а пакети Unreal показують те, що редактор про них зберіг, — без Unreal. Мініатюри малюються на
сервері й щоразу кадруються однаково, тож увесь пакет можна охопити одним поглядом.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Люстра на всю сторінку: великий 3D-перегляд поруч із її даними, тегами, пов’язаними текстурами й варіантами завантаження" src="../img/page-light.webp">
</picture>

### Розрізнені текстурні карти стають матеріалами

`Rock_BaseColor.png`, `Rock_Normal.png` і `Rock_Roughness.png` стають одним матеріалом Rock на освітленій сфері. Hoard
розуміє схеми іменування Megascans, Poly Haven, ambientCG, Substance і Unreal, зокрема впаковані карти, а також файли
Unity `.mat`. Повноекранний переглядач показує матеріал на сфері, кубі чи площині, у його реальному масштабі, і кожну
карту можна вмикати й вимикати.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Цегляний матеріал у повноекранному переглядачі, на сфері, з перемикачами для карт кольору, нормалей, шорсткості й висоти" src="../img/material-light.webp">
</picture>

### Аркуші спрайтів, що оживають

Атласи з TexturePacker, Starling, libGDX, Cocos2d і Godot стають одним аркушем спрайтів, який програє свої анімації й
показує список своїх спрайтів. Завантажте його перепакованим для рушія, яким користуєтеся: десять форматів, і кожному
передається лише те, що він здатен вмістити.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Аркуш спрайтів персонажа на всю сторінку: одна з його анімацій програється поруч із тегами й форматами завантаження" src="../img/sheet-light.webp">
</picture>

### Звуки, які видно

Кожен звук отримує хвильову форму, забарвлену залежно від того, наскільки яскраво він звучить, тож різкий удар і довгий
ембієнт виглядають по-різному ще до того, як ви їх увімкнете. Hoard читає метадані звукових бібліотек (категорії UCS,
поля Soundminer і BaseHead, точки циклу, темп, основну ноту), а MIDI й трекерну музику рендерить, щоб вона
відтворювалася, як будь-який інший звук.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Сітка звукових ефектів, намальованих кольоровими хвильовими формами; один із них відкритий і показує свій програвач, теги й дублі" src="../img/sound-light.webp">
</picture>

### Пошук, який знає, що є що

Hoard сам, без мережі, позначає все тегами: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`,
а також призначає категорію з дерева ігрових ресурсів. Шукайте словами, за кольором або за тим, що зображено на картинці, і
попросіть будь-яку текстуру чи звук показати «Більше схожого». Під’єднайте будь-яку OpenAI-сумісну модель, локальну чи
хмарну, і вона також писатиме назви й описи.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Пошук за словом «дерево»: дерев’яний стіл, кора, дошки й дерев’яна поверхня разом, а поруч фільтри за типом і тегом" src="../img/search-light.webp">
</picture>

### Прямо у ваш рушій

- **Плагіни для рушіїв** Godot 4, Unity, Unreal Engine 5 і Lumina: переглядайте бібліотеку просто в редакторі й натисніть
  «Додати до проєкту». Модель прибуде разом із текстурами, у потрібне місце, уже імпортована.
- **`hoard sync`** завантажує в проєкт саме ті файли, які він називає, закріплені в lock-файлі, тож кожен учасник
  команди й кожна машина збирання отримують ті самі байти.
- **Завантажити як** GLB, PNG, WAV, MP3 або у форматі аркуша спрайтів, з перетворенням на льоту й назвами текстур за
  правилами вашої студії.
- **ШІ-агенти** шукають, переглядають і отримують ресурси через MCP.

### Зберігається один раз, лишається в безпеці

Кожен файл зберігається один раз за своїм хешем BLAKE3, тож та сама текстура в п’яти наборах займає місце однієї, а
пошкодження не залишиться непоміченим. Тримайте все на локальному диску або в будь-якому бакеті S3. Теки, на які ви
вказуєте Hoard, лише читаються й ніколи не змінюються. Облікові записи, токени лише для читання, єдиний вхід (SSO),
HTTPS і HTTP/3 вбудовано.

### Або просто на вашому комп’ютері

Не маєте сервера? Настільний застосунок для Windows, macOS і Linux — це та сама бібліотека, тільки у власному вікні.
Встановіть його, виберіть теку, і він працює офлайн: жодних облікових записів, паролів і налаштувань сервера, нічого, до
чого могла б дістатися решта мережі, а введені вами ключі зберігаються в системному сховищі паролів. Нові версії
завантажуються у фоні й встановлюються, коли ви виходите із застосунку.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="Настільний застосунок Hoard у вікні Windows: моделі бібліотеки в сітці, а поруч у 3D-перегляді відкритий ліхтар, його теги й варіанти завантаження" src="../img/desktop-light.webp">
</picture>

<sub>Демонстраційна бібліотека складається з робіт під CC0: моделі з [Poly Haven](https://polyhaven.com) (glTF і USD),
поверхні з [ambientCG](https://ambientcg.com), а звуки, аркуші спрайтів і шрифти з [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` створює ці зображення й картку для соцмереж.</sub>

## Встановлення настільного застосунку

Завантажте його з [найновішого випуску](https://github.com/Azazel-Labs/Hoard/releases/latest): `-setup.exe` для Windows,
`.dmg` для macOS на Apple silicon, `.AppImage` або `.deb` для Linux. У Windows він встановлюється лише для вас, без
запиту прав адміністратора. Інсталятори поки не підписані Apple чи Microsoft, тому Windows перепитає перед запуском
інсталятора (Докладніше → Усе одно запустити), а macOS — перед першим відкриттям застосунку (Системні параметри →
Приватність і безпека → Усе одно відкрити).

Під час першого запуску він запитає, де буде бібліотека. Усе інше — в налаштуваннях, як і на сервері.

Далі він оновлюється сам, нічого натискати не треба: новий випуск завантажується у фоні й встановлюється, коли ви
закриваєте застосунок, тож наступного разу Hoard відкриється вже в новій версії. `.deb` оновлюється встановленням
наступного.

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
спершу зберігши копію в `/config/backups`. Змініть тег образу на `:1.3`, щоб отримувати лише випуски з виправленнями.
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
конфігурацію, безпеку, облікові записи, API та факти й цифри описано в [англійському README](../../README.md).

## Переклади

Повідомлення порталу й сервера Hoard доступні 31 мовою; мову можна вибрати в «Налаштування → Вигляд» (типово — мова
браузера). Перші переклади зроблено за допомогою машинного перекладу; виправлення від носіїв мови вітаються.
[docs/translations.md](../translations.md) пояснює, як це зробити.

## Спільнота

Маєте запитання, ідеї чи хочете щось показати? Заходьте привітатися в [Discord Azazel Labs](https://discord.gg/zf9ncS4EW). Про помилки й побажання краще писати в [issues на GitHub](https://github.com/Azazel-Labs/Hoard/issues): там вони не загубляться.

## Ліцензія

[MIT No Attribution](../../LICENSE) (MIT-0): використовуйте, копіюйте, змінюйте й поширюйте Hoard для будь-якої мети без
потреби зберігати повідомлення про авторські права. Пакети ресурсів, які ви зберігаєте в Hoard, мають власні ліцензії.
