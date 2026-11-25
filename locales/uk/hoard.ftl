## Помилки сервера

error-account-not-found = обліковий запис не знайдено
error-asset-gone = ресурс не знайдено
error-asset-not-found = ресурс { $id } не знайдено
error-blob-not-found = блоб { $hash } не знайдено
error-collection-not-found = колекцію { $id } не знайдено
error-connection-not-found = підключення не знайдено
error-derivative-not-found = похідний файл { $kind } для { $id } не знайдено
error-file-in-pack-not-found = { $path } у пакеті { $id } не знайдено
error-site-not-a-page = сторінками відкриваються лише вебсторінки
error-site-link-expired = адреса цієї сторінки застаріла; відкрийте сторінку знову з Hoard
error-format-not-found = формат { $file } не знайдено
error-frame-sheet-aseprite-only = аркуш кадрів є лише у файлів Aseprite
error-frame-sheet-frames = тут лише один кадр або забагато кадрів, щоб їх розкласти
error-location-not-found = розташування { $id } не знайдено
error-maintenance-task-not-found = завдання обслуговування { $id } не знайдено
error-open-upload-not-found = відкрите вивантаження { $id } не знайдено
error-original-not-found = оригінальний файл { $id } не знайдено
error-pack-not-found = пакет { $id } не знайдено
error-preview-too-big = це зображення завелике, щоб зробити з нього полегшений перегляд
error-project-not-found = проєкт { $name } не знайдено
error-saved-search-not-found = збережений пошук { $id } не знайдено
error-sign-in-first = спершу увійдіть (або надішліть токен доступу)
error-those-assets-not-found = цих ресурсів не знайдено
error-thumbnail-not-found = мініатюру для { $id } не знайдено
error-token-not-found = токен не знайдено
error-upload-not-found = вивантаження { $id } не знайдено
error-pack-needs-name = пакету потрібна назва
error-no-library = ще немає розташування бібліотеки, де можна зберігати файли: додайте кероване в «Налаштування → Розташування бібліотеки»
error-upload-failed = вивантаження не вдалося: { $reason }
error-upload-no-files = у вивантаженні немає файлів
error-analyze-nothing-to-do = нічого робити: rules=false, а LLM вимкнено
error-too-many-assets = забагато ресурсів в одному запиті
error-change-not-found = цієї зміни немає в історії цього ассета
error-bulk-by-look = пошук за виглядом не можна змінити цілком: виберіть ассети вручну
error-too-many-items = забагато елементів в одному запиті
error-not-a-hash = це не хеш blake3 у шістнадцятковому вигляді
error-merge-into-tag = into має бути тегом
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = «{ $text }» не є тегом
error-not-a-character = анімації скелета можна відтворювати лише на персонажі
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } уже під { $tag }, тому { $tag } не можна помістити під нього
error-nothing-to-merge = немає чого об’єднувати
error-selection-size = вкажіть від 1 до { $max } ідентифікаторів ресурсів
error-collection-needs-name = колекції потрібна назва
error-collection-exists = колекція з назвою { $name } уже є
error-collection-empty = колекція { $name } порожня
error-conversion-failed = не вдалося перетворити на { $format }: { $reason }
error-max-size = max має бути степенем двійки від 64 до 16384
error-replace-ext = ext має бути коротким розширенням файлу
error-empty-file = порожній файл
error-imitation-silent = У записі ніщо не виділяється на тлі. Спробуйте ще раз ближче до мікрофона.
error-replace-duplicate = перетворений файл дублює файл, що вже є в пакеті; пакет об’єднано з ним
error-ai-address = Адреса ШІ має починатися з http:// або https://
error-hosted-key = Введіть ключ API для { $provider }
error-hosted-address = Адреса сервісу пошуку зображень має починатися з http:// або https://
error-local-llm-action = action — це list, download або delete, і для двох останніх потрібна модель
error-role = Роль — це view, edit або admin
error-admin-only = це може зробити лише адміністратор
error-view-only = цей обліковий запис може лише переглядати, але нічого не змінювати: адміністратор може змінити це в «Налаштування → Обліковий запис»
error-portal-only = увійдіть у портал, щоб керувати обліковими записами й токенами
error-not-from-portal = цей запит надійшов не з порталу Hoard
error-account-name = Виберіть ім’я довжиною до 64 символів
error-password-length = Використайте пароль щонайменше з { $min } символів
error-too-many-codes = Забагато неправильних кодів: зачекайте { $seconds } с
error-already-set-up = Обліковий запис уже є: увійдіть
error-too-many-tries = Забагато невдалих спроб: зачекайте { $seconds } с
error-directory-unavailable = Каталог не зміг перевірити ваш пароль: { $reason }
error-password-elsewhere = Вашим паролем керує каталог або постачальник, через якого ви входите
error-current-password = Поточний пароль неправильний
error-account-source = Обліковий запис входить через Hoard (internal), каталог (ldap) або OAuth
error-account-exists = Обліковий запис із таким ім’ям уже є
error-own-role = Вашу роль може змінити інший адміністратор
error-remove-self = Не можна видалити власний обліковий запис, поки ви в нього ввійшли
error-token-name = Назвіть токен за тим, що його використовуватиме, до 80 символів
error-token-role = Токен не може мати більше прав, ніж обліковий запис, який його створює
error-wrong-setup-code = Це не той код налаштування, який передано серверу в HOARD_SETUP_CODE
error-wrong-password = Ім’я та пароль не збігаються
error-folder-missing = { $path } не знайдено (диск не змонтовано?)
error-s3-path = розташування S3 має вигляд s3://bucket
error-s3-address = адреса S3 має починатися з http:// або https://
error-s3-header = додатковий заголовок S3 має вигляд Name=value, а «{ $header }» — ні
error-keychain = не вдалося зберегти в системному сховищі паролів: { $reason }
error-not-built = цю копію Hoard зібрано без { $feature }
error-openai-auth-unreachable = не вдалося зв’язатися з auth.openai.com
error-openai-unexpected = неочікувана відповідь від OpenAI
error-openai-answered = OpenAI відповів { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID-токен не є JWT
error-chatgpt-id-token-issuer = ID-токен видав { $issuer }, а не { $expected }
error-chatgpt-id-token-audience = ID-токен призначено для іншого застосунку
error-chatgpt-id-token-expired = термін дії ID-токена минув
error-chatgpt-id-token-nonce = ID-токен не належить до цього входу
error-chatgpt-no-state = в адресі немає state; скопіюйте всю адресу
error-chatgpt-not-started = цей вхід розпочато не тут або понад 15 хвилин тому. Почніть знову з налаштувань.
error-chatgpt-canceled = вхід скасовано
error-chatgpt-refused = ChatGPT відхилив вхід: { $reason }
error-chatgpt-no-code = в адресі немає коду входу; скопіюйте всю адресу
error-chatgpt-other-registration = ChatGPT відповів для іншої реєстрації цього сервера; почніть вхід знову
error-chatgpt-other-registration-desktop = ChatGPT відповів для іншої копії Hoard; почніть вхід знову
error-chatgpt-not-registered = ChatGPT не завершив реєстрацію Hoard; почніть вхід знову
error-chatgpt-code-used = код входу вже використано або термін його дії минув; увійдіть знову
error-chatgpt-refused-code = ChatGPT відхилив вхід ({ $code })
error-chatgpt-no-id-token = ChatGPT не надіслав ID-токен
error-chatgpt-plan-not-allowed = Вхід виконано, але користуватися планом не дозволено. Увійдіть знову й дозвольте це або використайте ключ API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = вхід у ChatGPT не виконано; увійдіть у «Налаштування → Тегування ШІ»
error-chatgpt-refresh-later = ChatGPT не зміг зараз оновити вхід ({ $code }); спробуємо знову пізніше
error-chatgpt-signed-out-problem = ChatGPT вийшов із цього сервера ({ $code }). Увійдіть знову, щоб і далі тегувати за своїм планом.
error-chatgpt-signed-out-problem-desktop = ChatGPT вийшов із Hoard ({ $code }). Увійдіть знову, щоб і далі тегувати за своїм планом.
error-chatgpt-signed-out = ChatGPT вийшов із цього сервера ({ $code }); увійдіть знову в «Налаштування → Тегування ШІ»
error-chatgpt-signed-out-desktop = ChatGPT вийшов із Hoard ({ $code }); увійдіть знову в «Налаштування → Тегування ШІ»
error-openai-api-unreachable = не вдалося зв’язатися з api.openai.com
error-openai-broke-off = відповідь від OpenAI обірвалася
error-chatgpt-kept-refusing = OpenAI постійно відхиляв вхід у ChatGPT; увійдіть знову в «Налаштування → Тегування ШІ»
error-chatgpt-request-failed = запит не вдався
error-chatgpt-cut-off = відповідь обрізано ({ $reason })
error-chatgpt-cut-off-no-reason = відповідь обрізано (без пояснення причини)
error-openai-ended-early = відповідь від OpenAI завершилася неповною
error-chatgpt-usage-limit = ліміт використання вашого плану ChatGPT для Hoard наразі вичерпано. Переглянути або змінити ліміт можна на { $url }
error-chatgpt-not-eligible = цей обліковий запис або робочий простір ChatGPT не може використовувати свій план в інших застосунках. Використайте ключ API.
error-chatgpt-usage-unavailable = ChatGPT не зміг зараз перевірити використання плану; спробуємо знову пізніше
error-chatgpt-unsupported = використання плану ChatGPT не підтримує { $param } із цією моделлю
error-chatgpt-unsupported-images = використання плану ChatGPT не підтримує { $param } із цією моделлю; виберіть модель, яка приймає зображення, або вимкніть надсилання мініатюр
error-chatgpt-unsupported-part = використання плану ChatGPT не підтримує частину запиту з цією моделлю
error-chatgpt-port = port — це порт, через який браузер звертається до Hoard
error-chatgpt-paste-address = вставте всю адресу, що починається з http://127.0.0.1
error-too-many-hashes = щонайбільше { $max } хешів за раз
error-credits-pack = пакет { $key }
error-credits-collection = колекція { $key }
error-format-too-long = формат задовгий
error-not-started = сервер не запущено
error-not-started-desktop = Hoard не запущено
error-not-format-file = це не файл формату: { $name }
error-format-name = у назві має бути літера або цифра від A до Z чи від 0 до 9
error-format-miss-name = файли формату не приймають { $file }
error-format-miss-store = формат не приймає { $file } для файлів магазину (див. require)
error-format-miss-title = формат не знаходить назви в { $file }
error-cant-download-as = файли .{ $ext } не можна завантажити як { $format }
error-only-images-resize = змінювати розмір можна лише в зображень
error-https-no-certificate = Спершу вставте або вивантажте сертифікат і його ключ
error-https-no-cert-found = Сертифікат не знайдено: вставте текст, що починається з -----BEGIN CERTIFICATE-----
error-https-no-key-found = Закритий ключ не знайдено: вставте текст, що починається з -----BEGIN PRIVATE KEY-----
error-https-several-keys = Тут більше ніж один закритий ключ: вставте лише той, що відповідає сертифікату
error-https-key-password = Ключ захищено паролем: надайте його без пароля (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Сертифікат не є дійсним PEM
error-https-key-not-pem = Закритий ключ не є дійсним PEM
error-https-key-mismatch = Ключ не відповідає сертифікату. Якщо ви надали ланцюжок, поставте сертифікат свого сервера першим
error-https-unusable = Сертифікат або ключ не можна використати
error-https-expired = Термін дії цього сертифіката минув { $date }
error-https-not-yet-valid = Цей сертифікат дійсний лише з { $date }
error-https-unreadable = Hoard не може прочитати цей сертифікат
error-https-bad-name = «{ $name }» не є адресою, яку може містити сертифікат
error-https-test-timed-out = час очікування тестового підключення минув
error-https-test-failed = тестове підключення не вдалося
error-https-no-ca = Hoard ще не створив свій центр сертифікації
error-https-unavailable = HTTPS недоступний на цьому сервері
error-https-certificate-mode = Сертифікат потрібен для варіанта «Мій сертифікат»
error-https-test = Сертифікат не спрацював у тестовому підключенні: { $reason }
error-nothing-to-retry = тут немає нічого, що можна повторити
error-ai-unreachable = не вдалося зв’язатися з { $url }
error-ai-key-refused = { $url } відхилив ключ API ({ $status })
error-ai-answered = { $url } відповів { $status }: { $reason }
error-ai-not-json = { $url }/models відповів не в форматі JSON
error-model-server-refused = сервер моделей: { $reason }
error-no-model-server = за адресою { $url } не відповідає жоден сервер моделей
error-not-llama-router = { $url } не є сервером llama.cpp у режимі маршрутизатора, тож він не може завантажувати моделі
error-model-download = не вдалося завантажити { $model }: { $reason }
error-model-delete = не вдалося видалити { $model }: { $reason }
error-unknown-kind = невідомий тип ресурсу { $kind }
error-watched-is-folder = відстежуване розташування — це тека на цьому сервері
error-cant-reach = немає доступу до { $path }: { $reason }
error-folder-full-path = вкажіть повний шлях до теки
error-cant-create = не вдалося створити { $path }
error-cant-write = не вдалося записати в { $path }
error-cant-read = не вдалося прочитати { $path }
error-overlaps-location = це перетинається з розташуванням { $name } ({ $path })
error-overlaps-data = це перетинається з текою даних Hoard ({ $path })
error-overlaps-cache = це перетинається з текою кешу Hoard ({ $path })
error-not-watched = { $name } не є відстежуваною текою
error-watched-no-kinds = відстежувана тека не приймає нових файлів, тож у неї немає типів, які можна змінити
error-keep-catch-all = натомість зробіть резервним інше розташування
error-location-holds-files = { $name } ще містить файли: спершу перемістіть їх в інші розташування (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } недоступне, тож Hoard не може визначити, чи там ще є файли
error-relocate-running = файли вже переміщуються між розташуваннями; спробуйте знову, коли це завершиться
error-watched-not-moved = { $name } — відстежувана тека: Hoard не переміщує її файли
error-no-other-location = немає іншого розташування бібліотеки, куди можна перемістити файли: спершу додайте його
error-full-path = вкажіть повний шлях
error-pick-folder = виберіть теку
error-task-running = «{ $task }» уже виконується
error-job-cant-stop = Завдання «{ $job }» не можна зупинити після запуску
error-job-not-found = це завдання вже не чекає в черзі й не виконується
error-task-cant-stop = Завдання «{ $task }» не можна зупинити на півдорозі
error-task-not-running = Завдання «{ $task }» не виконується
error-token-account-gone = обліковий запис цього токена більше не існує, тож у нього немає обраного чи пошуків
error-favorites-too-many = щонайбільше 10 000 за раз
error-saved-search-name = збереженому пошуку потрібна назва довжиною до 80 символів
error-saved-search-too-long = цей пошук задовгий, щоб його зберегти
error-hoard-seen = скарб може зберігати не більше 32 видів ресурсів, кожен із назвою з малих літер
error-news-seen = розділ «Що нового» може зберігати не більше 256 переглянутих новинок, кожна з назвою з малих літер, цифр і дефісів
error-texture-pattern-map = Шаблону назв текстур потрібен { "{" }map{ "}" }, інакше всі карти матеріалу отримають однакову назву
error-texture-pattern-fields = Шаблон назв текстур може використовувати { "{" }name{ "}" }, { "{" }map{ "}" } і { "{" }size{ "}" }
error-texture-name-chars = Назви текстур не можуть містити < > : " / \ | ? або *
error-texture-map-type = Типу карти з назвою { $role } немає
server-role-base-color = кольору
server-role-normal = нормалей
server-role-normal-dx = нормалей (DirectX)
server-role-roughness = шорсткості
server-role-gloss = глянцю
server-role-metalness = металевості
server-role-ao = фонового затінення
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = висот
server-role-bump = рельєфу
server-role-cavity = порожнин
server-role-specular = дзеркальності
server-role-opacity = непрозорості
server-role-emissive = світіння
server-role-translucency = просвічування
error-texture-suffix-missing = Задайте суфікс для карт { $map }
error-texture-suffix-chars = Суфікс для карт { $map } не може містити < > : " / \ | ? або *
error-texture-ending-missing = Кожне власне закінчення має містити закінчення, яке слід шукати
error-texture-ending-chars = Власні закінчення не можуть містити < > : " / \ | ? або *
error-texture-ending-twice = { $ending } двічі є серед ваших закінчень
error-oauth-no-client-id = У цьому посиланні немає client_id.
error-oauth-unknown-app = Цей застосунок тут не зареєстровано (або вже не зареєстровано). Почніть підключення знову із застосунку.
error-oauth-redirect = Застосунок попросив повернути його на адресу, яку він не реєстрував.
error-oauth-response-type = Hoard видає лише коди входу (response_type=code).
error-oauth-pkce = Щоб увійти в Hoard, застосунок має використовувати PKCE з S256.
error-light-preview-images = полегшений перегляд є лише в зображень
error-no-light-preview = немає полегшеного перегляду: { $reason }
error-no-frame-sheet = немає аркуша кадрів: { $reason }
error-thumbnail-source = невідоме джерело мініатюри: { $source }
error-thumbnail-too-big = це зображення завелике для мініатюри
error-thumbnail-models-only = мініатюру, намальовану браузером, приймають лише моделі
error-not-a-picture = Hoard не може прочитати це зображення: { $reason }
error-project-name = проєкту потрібна назва довжиною до 200 символів
error-project-too-big = щонайбільше мільйон ресурсів в одному проєкті
error-project-too-many-at-once = щонайбільше 100 000 за раз
error-server-name = Виберіть назву довжиною до { $max } символів в один рядок
error-tool-setting = { $tool }: виберіть «вимкнено», «автоматично» або вкажіть шлях
error-ai-model = Виберіть модель, яку має використовувати служба ШІ
error-public-address = Публічна адреса має починатися з http:// або https://
error-update-hook-address = Адреса для сповіщень про оновлення має починатися з http:// або https://
error-update-hook-auth = Заголовок Authorization не може містити розривів рядка чи інших керівних символів
error-no-soundfont = На сервері немає SoundFont за шляхом { $path }
error-no-soundfont-desktop = Немає SoundFont за шляхом { $path }
error-soundfont-bad = { $name } — не SoundFont, з яким може грати MIDI: { $reason }
error-workers = Фонові обробники: від 1 до { $max }
error-ai-batch = Ресурсів на запит до ШІ: від 1 до 200
error-conversion-cache = Перетворені завантаження: щонайменше 100 МБ
error-motion-fps = анімовані мініатюри відтворюються з частотою 12, 24 або 30 кадрів на секунду
error-thumb-size = мініатюри мають 128 або 256 пікселів завширшки
error-backup-keep = Резервні копії за розкладом: зберігати від 1 до { $max }
error-backup-copy-to = Резервні копії за розкладом: { $path } — не тека на сервері (вкажіть повний шлях)
error-conversion-keep-days = Правила перетворення: зберігати оригінали від 1 до { $max } днів
error-conversion-target = Правила перетворення: Hoard не вміє перетворювати файли на { $format }
error-conversion-no-source = Правила перетворення: кожному правилу потрібен принаймні один формат, який воно перетворює
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Правила перетворення: файли .{ $ext } не можна перетворити на { $format }
error-conversion-twice = Правила перетворення: .{ $ext } є у двох правилах
error-conversion-cant-undo = Цей файл не було перетворено правилом, або його оригінал уже видалено.
error-conversion-path-taken = За шляхом { $path } тепер інший файл, тож оригінал не можна туди повернути.
error-limits-zero = Обмеження не можуть дорівнювати нулю
error-settings-object = очікувався JSON-об’єкт налаштувань
error-bad-setting = неправильне налаштування: { $reason }
error-auth-default-role = Типова роль — view, edit або admin
error-auth-session-hours = Сеанси тривають від 1 години до року
error-auth-settings = неправильні налаштування автентифікації: { $reason }
error-ldap-settings = неправильні налаштування LDAP: { $reason }
error-ldap-no-server = Додайте адресу сервера каталогу
error-ldap-base-dn = Вкажіть, де розташовані записи людей (base DN)
error-ldap-username-attribute = Вкажіть, який атрибут містить ім’я, з яким люди входять
error-ldap-bind-dn = Вкажіть обліковий запис, який шукає людей, або шукайте анонімно
error-ldap-ca-cert = Центр сертифікації не є сертифікатом PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Адреса сервера каталогу порожня
error-ldap-server-form = { $server }: адреса сервера каталогу має вигляд host:port, ldap://… або ldaps://…
error-ldap-not-starttls = { $server }: це LDAPS, а не StartTLS
error-ldap-no-certificate = у ньому немає сертифіката
error-ldap-password-again = Знову вкажіть пароль облікового запису, який шукає людей: збережений пароль надсилається лише на ті сервери, для яких його надано
error-ldap-unreachable = не вдалося зв’язатися з жодним сервером каталогу ({ $reasons })
error-ldap-no-anonymous = каталог не дозволяє шукати людей анонімно ({ $reason })
error-ldap-bind-failed = обліковий запис, який шукає людей, не зміг увійти ({ $reason })
error-ldap-search = пошук { $name } у { $base }
error-ldap-group-search = пошук груп { $name } у { $base }
error-ldap-several =
    { $count ->
        [one] { $count } запис у каталозі відповідає { $name }: звузьте фільтр користувачів
        [few] { $count } записи в каталозі відповідають { $name }: звузьте фільтр користувачів
        [many] { $count } записів у каталозі відповідають { $name }: звузьте фільтр користувачів
       *[other] { $count } запису в каталозі відповідають { $name }: звузьте фільтр користувачів
    }
error-ldap-password-check = каталог не перевірив пароль { $name } ({ $reason })
error-ldap-wrong-password = Каталог не знає { $name } із таким паролем
error-oauth-client-id = Вкажіть client ID, який постачальник створив для Hoard
error-oauth-addresses = Адреси авторизації та токенів мають починатися з https://
error-oauth-userinfo-address = Адреса відомостей про користувача має починатися з https://
error-oauth-username-claim = Вкажіть, яке твердження (claim) містить ім’я людини
error-oauth-secret-again = Знову вкажіть секрет клієнта: збережений секрет надсилається лише на ту адресу токенів, для якої його надано
error-oauth-issuer = Видавець (issuer) має починатися з https://
error-oauth-discovery-unreachable = Не вдалося зв’язатися з { $url }: { $reason }
error-oauth-discovery-status = { $url } відповів { $status }
error-oauth-discovery-bad = { $url } не є документом виявлення (discovery)
error-oauth-discovery-incomplete = { $url } не вказує адрес авторизації та токенів
error-signin-no-group = { $name } не входить до групи, якій дозволено користуватися Hoard
error-signin-other-source = Обліковий запис Hoard з ім’ям { $name } уже є, і він входить в інший спосіб
error-signin-other-subject = Обліковий запис Hoard з ім’ям { $name } уже є, і він належить іншій людині в постачальника
error-signin-no-account = У { $name } ще немає облікового запису Hoard: адміністратор може додати його в «Налаштування → Обліковий запис»
error-signin-bad-name = Таке ім’я не може бути обліковим записом Hoard (до 64 символів)
error-oauth-off = Вхід через OAuth не ввімкнено
error-oauth-bad-authorization-address = адреса авторизації недійсна
error-oauth-refused = Постачальник не впустив вас: { $reason }
error-oauth-no-code = Постачальник повернув вас без входу
error-oauth-expired = Цей вхід тривав задовго або його розпочато деінде: спробуйте ще раз
error-oauth-no-claim = Постачальник не повідомив, хто ви (немає твердження { $claim })
error-oauth-no-sub = Постачальник не повідомив, хто ви (немає твердження sub чи id)
error-oauth-token-unreachable = не вдалося зв’язатися з адресою токенів постачальника
error-oauth-token-refused = Постачальник не завершив вхід: { $reason }
error-oauth-no-access-token = постачальник не надіслав токен доступу
error-oauth-userinfo-unreachable = не вдалося зв’язатися з адресою відомостей про користувача в постачальника
error-oauth-userinfo-refused = Постачальник не повідомив, хто ввійшов ({ $status })
error-oauth-userinfo-not-json = відомості про користувача від постачальника не в форматі JSON
error-format-no-name = name порожнє
error-format-no-files = files порожнє: вкажіть, які файли читає формат, наприклад ["*.json"]
error-format-empty-selector = порожньо
error-format-unopened-bracket = закривна дужка без відкривної
error-format-unclosed-bracket = дужку не закрито
error-format-empty-step-dots = порожній крок (дві крапки поспіль або крапка на початку чи в кінці)
error-format-unclosed-except = !( не закрито
error-format-unclosed-filter = [ не закрито
error-format-after-filters = { $rest } після ключа та фільтрів { $step }
error-format-except-needs-glob = !(…) ставиться після ключа, що містить *
error-format-except-not-glob = !(…) ставиться після ключа, що містить *, а не після { $key }
error-format-empty-step = порожній крок
error-format-not-a-filter = [{ $filter }] не є фільтром: використайте [], [key=value|other], [array], [string], [number], [bool] або [object]
error-format-filter-needs-value = [{ $filter }] потребує ключа та значення
error-store-search-empty = немає чого шукати; введіть назву пакета
error-update-hook-not-set = спершу вкажіть адресу для сповіщень
error-upload-stalled = вивантаження { $path } зависло; надішліть цю частину знову
error-upload-busy = { $path } уже записується
error-upload-incomplete =
    { $count ->
        [one] { $count } файл не завершено, наприклад { $files }
        [few] { $count } файли не завершено, наприклад { $files }
        [many] { $count } файлів не завершено, наприклад { $files }
       *[other] { $count } файлу не завершено, наприклад { $files }
    }
error-upload-no-files-listed = немає файлів
error-upload-not-part = { $path } не належить до вивантаження { $id } або вивантаження закрито
error-bad-path = неправильний шлях
error-upload-part-offset = { $offset } — це не початок частини { $path }
error-upload-interrupted = вивантаження перервано: { $reason }
error-upload-part-long = частина { $part } файлу { $path } довша за { $len } байтів
error-upload-part-short = частина { $part } файлу { $path }: отримано { $got } байтів, очікувалося { $len }
error-upload-file-long = { $path } більший, ніж заявлено
error-upload-file-short = { $path }: отримано { $got } байтів, очікувалося { $len }
error-upload-complete = вивантаження вже завершено
error-already-imported = уже імпортовано як «{ $name }»

## Текст сервера

server-store-place-readme = README
server-store-place-folders = теки
server-store-why-linked = Посилання є в пакеті: { $place }
server-store-why-same-title = Та сама назва, що й у пакеті: { $place }
server-store-why-by = Автор — { $author }, згаданий у пакеті: { $place }
server-store-why-meshes = Приблизно стільки ж мешів, як у пакеті ({ $count })
server-store-why-textures = Приблизно стільки ж текстур, як у пакеті ({ $count })
server-chatgpt-revoke-failed = Тут вихід виконано, але повідомити OpenAI не вдалося. Щоб напевно, відключіть Hoard у ChatGPT: Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Вхід у ChatGPT виконано
server-chatgpt-signed-in = Hoard увійшов. Тепер тегування ШІ може використовувати ваш план ChatGPT.
server-chatgpt-signed-in-as = Hoard увійшов як { $email }. Тепер тегування ШІ може використовувати ваш план ChatGPT.
server-chatgpt-not-finished-title = Вхід не завершено
server-chatgpt-other-computer = Якщо ви почали вхід із Hoard на іншому комп’ютері, скопіюйте адресу цієї сторінки з адресного рядка й вставте її в налаштування Hoard там.
server-back-to-settings = Назад до налаштувань
server-issues-pack = Пакети, які не вдалося додати
server-issues-pack-blurb = Видаліть пакет і додайте його знову або відкрийте, щоб побачити, що в ньому є.
server-issues-damaged = Збережені файли, які пошкоджено або втрачено
server-issues-damaged-blurb = Знайдено завданням «Перевірити файли на пошкодження». Відновіть їх із резервної копії або знову додайте їхній пакет, а потім ще раз запустіть «Перевірити файли на пошкодження».
server-issues-folder = Відстежувані теки, які не вдається прочитати
server-issues-folder-blurb = Їхні пакети залишаються як є, доки теку знову не вдасться прочитати.
server-issues-missing = Відстежувані пакети, тека яких зникла
server-issues-missing-blurb = Кожен пакет зберігається 3 дні на випадок, якщо його тека лише тимчасово недоступна, а потім видаляється.
server-issues-job = Фонові завдання, що завершилися помилкою
server-issues-job-blurb = Повторна спроба запускає завдання знову з початку.
server-issues-file = Файли, які не вдалося відкрити
server-issues-file-blurb = Hoard не зміг відкрити або зберегти їх. Відстежуваний пакет спробує знову під час наступного сканування; файл, що вже є в бібліотеці, зберіг те, що мав раніше.
server-issues-unpack = Архіви, які не вдалося розпакувати
server-issues-unpack-blurb = Кожен зберігається у своєму пакеті як звичайний файл, тож нічого з його вмісту немає в бібліотеці.
server-issues-read = Файли, які Hoard не зміг розібрати
server-issues-read-blurb = Вони є в бібліотеці, але без відомостей, тегів і мініатюри, які дає їх читання.
server-issues-render = Моделі, на яких зламався рендерер мініатюр
server-issues-render-blurb = У них немає мініатюри. Відкрити модель у переглядачі все ще може вдатися, і там можна зберегти мініатюру.
server-issues-convert = Моделі, які Blender не зміг перетворити
server-issues-convert-blurb = У них немає копії GLB, тож їх не можна показати в 3D.
server-issues-unreal = Пакунки Unreal, які не вдалося перетворити
server-issues-unreal-blurb = Решта пакунків є на сторінці Unreal цього пакета.
server-issues-reformat = Файли, які правило перетворення не змогло перетворити
server-issues-reformat-blurb = Їх залишено як були.
server-issues-materials = Пакети, матеріали яких не вдалося визначити
server-issues-materials-blurb = Усі їхні текстури на місці; їх просто не згруповано в матеріали.
server-issues-material-thumb = Матеріали без мініатюри
server-issues-material-thumb-blurb = Матеріал знайдено, але його попередній перегляд не вдалося намалювати.
server-issues-store-files = Файли магазинів, які не вдалося прочитати
server-issues-store-files-blurb = Файли з магазину, звідки походить пакет (наприклад, JSON від Megascans), які дають йому назву й теги.
server-issues-scan-again = Сканувати знову
server-issues-retry = Повторити
server-issues-reanalyze = Проаналізувати пакет знову
server-issues-convert-again = Перетворити знову
server-issues-look-again = Пошукати знову
server-issues-read-again = Прочитати знову
server-issues-blob-missing = Збережений файл зник.
server-issues-blob-damaged = Збережений файл не збігається з тим, що зберігали: його пошкоджено.
server-issues-folder-gone = Теку «{ $folder }» не знайдено.
server-job-a-pack = пакет
server-job-ingest = Імпорт: { $pack }
server-job-retag = Повторне тегування: { $pack }
server-job-llm-tag = Тегування за допомогою ШІ: { $pack }
server-job-convert = Перетворення в Blender: { $pack }
server-job-unreal = Перетворення пакунків Unreal: { $pack }
server-job-reformat = Перетворення файлів за правилами перетворення: { $pack }
server-job-scan = Сканування відстежуваної теки
server-job-relocate = Переміщення файлів між розташуваннями бібліотеки
server-job-looks = Визначення кольорів і вигляду
server-job-sounds = Прослуховування звуків
server-job-embed = Розпізнавання вмісту зображень
server-job-hear = Розпізнавання того, як звучать звуки
server-job-materials = Пошук матеріалів: { $pack }
server-job-read-files = Читання файлів: { $pack }
server-job-store-lookup = Пошук сторінки в магазині
server-maintenance-backup = Створити резервну копію бази даних
server-maintenance-backup-blurb = Узгоджена копія бази даних (пакети, теги, нотатки, колекції та налаштування) у теці backups поруч із нею. Файлів бібліотеки в ній немає: після збереження вони ніколи не змінюються, тож будь-яка синхронізація файлів зберігає їхні копії.
server-maintenance-scrub = Перевірити файли на пошкодження
server-maintenance-scrub-blurb = Перечитує кожен збережений файл і звіряє його з відбитком, щоб виявити файли, які зіпсувалися або зникли з диска чи NAS. На великій бібліотеці це триває довго.
server-maintenance-reanalyze = Проаналізувати всі пакети знову
server-maintenance-reanalyze-blurb = Знову читає кожен файл цією версією Hoard: мініатюри, попередні перегляди, відомості й теги за правилами створюються заново. Ваші власні зміни залишаються; ШІ повторно не запитується.
server-maintenance-clear-cache = Очистити кеш перетворень
server-maintenance-clear-cache-blurb = Видаляє файли, збережені після перетворень «Завантажити як» (їх буде створено знову за наступним запитом), і те, що залишила перервана робота.
server-maintenance-gc = Видалити невикористані файли
server-maintenance-gc-blurb = Видаляє збережені файли, на які вже ніщо не посилається, наприклад файли остаточно видалених пакетів.
server-update-no-address = адресу для сповіщень не задано
server-update-no-answer = немає відповіді протягом 30 секунд

## Налаштування: Вигляд

appearance-language = Мова
appearance-language-auto = Як у браузері ({ $name })
appearance-scheme = Світла чи темна
appearance-scheme-system = Як у системі
appearance-size-section = Текст і відступи
appearance-text-size = Розмір тексту
appearance-text-small = Дрібний
appearance-text-normal = Звичайний
appearance-text-large = Великий
appearance-density = Відступи
appearance-density-comfortable = Просторо
appearance-density-compact = Компактно
appearance-tiles = Плитки
appearance-tiles-what = Що кожна плитка бібліотеки показує, окрім зображення.
appearance-tile-names = Назви
appearance-tile-details = Тип і розмір (пікселі, тривалість, трикутники)
appearance-tile-badges = Тип файлу на зображенні
appearance-tile-motion = Відтворювати анімації, поки над ними вказівник
appearance-tile-motion-always = Відтворювати анімації весь час, а не лише під вказівником
appearance-top-bar = Верхня панель
appearance-top-bar-what = Приберіть те, чим не користуєтеся. / і далі переводить у поле пошуку, а { $keys } і далі відкриває палітру команд.
appearance-top-activity = Фонова робота
appearance-top-theme = Перемикач світлої та темної теми
appearance-top-mic = Мікрофон у полі пошуку, щоб шукати звуки, видавши схожий
appearance-top-slash = Підказка / у полі пошуку
appearance-top-palette = Кнопка палітри команд ({ $keys })
appearance-news = Що нового
appearance-news-what = Нове в Hoard позначається мерехтливою золотою крапкою, доки ви його не побачите, а верхня панель збирає все нове в одному місці. Що ви вже бачили, зберігається у вашому обліковому записі.
appearance-news-point-out = Позначати нове

## Корінь

root-unreachable = Не вдалося зв’язатися із сервером Hoard: { $error }
root-unreachable-desktop = Hoard не відповідає: { $error }
root-desktop-reopen = У цьому вікні виконано вихід. Закрийте Hoard і відкрийте його знову, щоб продовжити.
root-try-again = Спробувати знову
root-lazy-connect = підключення застосунку
window-minimize = Згорнути
window-maximize = Розгорнути
window-restore = Відновити
window-close = Закрити

## Застосунок: бібліотека, верхня панель і фільтри

app-lazy-settings = налаштування
app-lazy-archive = архів
app-lazy-shortcuts = комбінації клавіш
app-lazy-setup = початкове налаштування
app-lazy-explorer = оглядач мешів
app-theme-system = Як у системі
app-theme-light = Світла тема
app-theme-dark = Темна тема
app-color-red = Червоний
app-color-orange = Помаранчевий
app-color-yellow = Жовтий
app-color-green = Зелений
app-color-teal = Бірюзовий
app-color-blue = Синій
app-color-purple = Фіолетовий
app-color-pink = Рожевий
app-color-brown = Коричневий
app-color-sand = Пісочний
app-color-gray = Сірий
app-color-slate = Грифельний
app-color-black = Чорний
app-color-white = Білий
app-color-swatch = { $color }: ресурси, у яких багато цього кольору
app-color-any = Будь-який колір
app-color-pick = Вибрати будь-який колір
app-color = Колір
app-music = Музика
app-tempo = BPM
app-tempo-min = від
app-tempo-max = до
app-tempo-min-title = Музика в цьому темпі або швидше, в ударах за хвилину (удвічі повільніше чи швидше теж підходить)
app-tempo-max-title = Музика в цьому темпі або повільніше, в ударах за хвилину
app-key = Тональність
app-key-any = Будь-яка тональність
app-key-major = Мажор
app-key-minor = Мінор
app-key-mix = І тональності, що з нею зводяться
app-key-mix-title = Її паралельна мажорна чи мінорна тональність і тональності на квінту вище й нижче, з якими її зводять діджеї
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Напишіть звук так, як вимовили б його по літерах: схожі звуки будуть першими
app-imitation-record = Зобразіть звук у мікрофон: схожі звуки будуть першими
app-imitation-stop = Зупинити запис
app-imitation-heard = Ваш звук
app-imitation-mic-failed = Не вдалося записати з мікрофона: { $error }
app-retry = Повторити
app-undo = Скасувати
app-search-failed = Пошук не вдався: { $error }
app-unreachable = Не вдалося зв’язатися із сервером: { $error }
app-unreachable-desktop = Hoard не відповідає: { $error }
app-forget-project-title = Забути { $name }?
app-forget-project-body =
    Hoard перестане показувати його там, де використовуються { $count ->
        [one] його { $count } ресурс
        [few] його { $count } ресурси
        [many] його { $count } ресурсів
       *[other] його { $count } ресурсу
    }, доки наступний hoard sync не повідомить про нього знову. У самому проєкті нічого не зміниться.
app-forget-project-confirm = Забути
app-forget-project-failed = Не вдалося забути { $name }: { $error }
app-forget-project-label = Забути проєкт { $name }
app-forget-project = Забути цей проєкт
app-save-name-like = схоже на { $name }
app-save-name-favorites = обране
app-search-saved = Збережено «{ $name }»: шукайте у «Фільтри» → «Ваше»
app-search-updated = Оновлено «{ $name }»
app-rank-alike = Найбільш схожі
app-rank-color = Найбільше цього кольору
app-rank-match = Найкращий збіг
app-rank-imitation = Найбільш схожі
app-sort = Сортування
app-sort-name = Назва
app-sort-created = Створено
app-sort-updated = Оновлено
app-sort-size = Розмір
app-sort-rating = Оцінка
app-sort-newest-first = Спершу нові
app-sort-oldest-first = Спершу старі
app-sort-a-to-z = Від А до Я
app-sort-z-to-a = Від Я до А
app-sort-largest-first = Спершу найбільші
app-sort-smallest-first = Спершу найменші
app-sort-best-rated-first = Спершу найвище оцінені
app-sort-lowest-rated-first = Спершу найнижче оцінені
app-collection-added =
    { $count ->
        [one] Додано { $count } ресурс
        [few] Додано { $count } ресурси
        [many] Додано { $count } ресурсів
       *[other] Додано { $count } ресурсу
    } до { $name }
app-collection-add-failed = Не вдалося додати до колекції: { $error }
app-collection-removed = Вилучено з { $name }: { $count }
app-collection-remove-failed = Не вдалося вилучити з колекції: { $error }
app-ns-subject = Предмет
app-ns-style = Стиль
app-ns-theme = Тематика
app-ns-material = Матеріал
app-ns-map = Карта PBR
app-ns-color = Колір
app-ns-mood = Настрій
app-ns-genre = Жанр
app-ns-license = Ліцензія
app-ns-res = Роздільність
app-ns-duration = Тривалість
app-ns-poly = Кількість полігонів
app-ns-format = Формат
app-ns-weight = Вага
app-ns-tech = Технічне
app-ns-blender = Версія Blender
app-ns-depth = Бітова глибина
app-ns-ucs = Категорія звуку (UCS)
app-ns-bpm = Темп
app-ns-root = Тоніка
app-ns-level = Гучність
app-ns-ue = Клас Unreal
app-ns-engine = Рушій
app-ns-extracted = Відновлене джерело
app-ns-script = Писемність
app-bulk-failed =
    Не вдалося змінити { $count ->
        [1] цей ресурс
        [one] { $count } ресурс
        [few] ці { $count } ресурси
        [many] ці { $count } ресурсів
       *[other] ці { $count } ресурсу
    }: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] Скопійовано { $count } тег
        [few] Скопійовано { $count } теги
        [many] Скопійовано { $count } тегів
       *[other] Скопійовано { $count } тега
    }
app-tags-copy-none = У { $name } немає тегів для копіювання
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Теги не скопійовано. Натисніть C на ресурсі, щоб скопіювати його теги.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [1] Ці теги в нього вже є
       *[other] Ці теги вже є в усіх { $count }
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [1]
            { $count ->
                [one] Додано { $count } тег
                [few] Додано { $count } теги
                [many] Додано { $count } тегів
               *[other] Додано { $count } тега
            }
       *[other]
            { $count ->
                [one] Додано { $count } тег до { $assets } ресурсів
                [few] Додано { $count } теги до { $assets } ресурсів
                [many] Додано { $count } тегів до { $assets } ресурсів
               *[other] Додано { $count } тега до { $assets } ресурсів
            }
    }
app-bulk-untagged =
    Тег { $tag } знято з { $count ->
        [one] { $count } ресурсу
        [few] { $count } ресурсів
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
app-bulk-tagged =
    Тег { $tag } додано до { $count ->
        [one] { $count } ресурсу
        [few] { $count } ресурсів
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
app-bulk-review = Позначити переглянутим
app-bulk-review-title = Позначити вибрані ресурси як переглянуті: після цього правила та ШІ не чіпають їхніх тегів
app-bulk-unreview = Позначити непереглянутим
app-bulk-unreview-title = Зняти позначку «переглянуто» з вибраних ресурсів
app-bulk-reviewed =
    Позначено переглянутими { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурси
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
app-bulk-unreviewed =
    Позначку «переглянуто» знято з { $count ->
        [one] { $count } ресурсу
        [few] { $count } ресурсів
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
app-bulk-accept = Залишити припущення
app-bulk-accept-title = Залишити як власні всі теги, які правила та ШІ поставили вибраним ресурсам
app-bulk-accepted =
    Припущені теги залишено в { $count ->
        [one] { $count } ресурсі
        [few] { $count } ресурсах
        [many] { $count } ресурсах
       *[other] { $count } ресурсі
    }
app-credits-failed = Не вдалося скласти список авторів: { $error }
app-archived-assets =
    { $count ->
        [one] В архів перенесено { $count } ресурс
        [few] В архів перенесено { $count } ресурси
        [many] В архів перенесено { $count } ресурсів
       *[other] В архів перенесено { $count } ресурсу
    }
app-archived-one = { $name } перенесено в архів
app-open-failed = Не вдалося відкрити цей ресурс: { $error }
app-unnamed-pack = пакет
app-unnamed-collection = колекція
app-unnamed-asset = ресурс
app-home-version = Hoard { $version } — назад до бібліотеки
app-home = Назад до бібліотеки
app-search-label = Пошук ресурсів
app-search-placeholder = Шукайте за назвами, шляхами, тегами
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = скеля or камінь: одне з двох · -мох: виключити · "сухий мох": ці слова разом · pack:ліс: у назві пакета
app-alt-leave-out = Alt-клік, щоб виключити
app-views = Вигляди
app-library = Бібліотека
app-packs = Пакети
app-packs-working = в роботі: { $count }
app-update-title = Вийшов Hoard { $version } (на цьому сервері { $current }). Завантажте новий образ або бінарний файл і перезапустіть; база даних оновиться сама.
app-update = Оновити
app-theme = Тема
app-sign-in = Увійти
app-archive-title = Архів: { $contents }
app-archive = Архів
app-settings = Налаштування
app-palette-title = Палітра команд ({ $keys })
app-import-folder = Імпортувати теку
app-add-files = Додати файли
app-add-files-short = Додати
app-split-label = Як імпортувати цю теку
app-split-before = { "" }
app-split-after =
    { " " }містить { $count ->
        [one] { $count } теку або архів
        [few] { $count } теки або архіви
        [many] { $count } тек або архівів
       *[other] { $count } теки або архіву
    }.
app-split-each = Імпортувати кожну як окремий пакет ({ $count })
app-split-one = Імпортувати як один пакет
app-cancel = Скасувати
app-filters = Фільтри
app-done = Готово
app-yours = Ваше
app-favorites = Обране
app-needs-review = Потребує перегляду
app-needs-review-title = Лише ресурси, які ще ніхто не переглядав (працює разом з іншими фільтрами)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Оцінені
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } зірка і більше
        [few] { $count } зірки і більше
        [many] { $count } зірок і більше
       *[other] { $count } зірки і більше
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Клацніть ще раз, щоб показати всі оцінки
app-forget-search-label = Забути збережений пошук { $name }
app-forget-search = Забути цей пошук
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Відкривати бібліотеку з цим пошуком
app-default-search-on-title = Бібліотека відкривається з цим пошуком. Натисніть, щоб вимкнути.
app-default-search-label = Відкривати бібліотеку з пошуком { $name }
app-rename-search = Перейменувати
app-rename-search-label = Нова назва для { $name }
app-projects-title = Проєкти, які виконують hoard sync звідси, з тим, що було в їхній останній синхронізації
app-projects = Проєкти
app-project-synced = Синхронізовано { $when }
app-project-synced-by = Синхронізовано { $when }, автор: { $who }
app-collections = Колекції
app-no-filters = Для цього пошуку немає фільтрів.
app-kind = Тип
app-facets-more = ще { $count }
app-facets-fewer = менше
app-searching = Пошук…
app-searching-live = Пошук
app-asset-count =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурси
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
app-clear-all = Очистити все
app-save-name-label = Назва цього пошуку
app-save-name-placeholder = Назвіть його
app-save = Зберегти
app-save-search-title = Зберегти цей пошук у «Фільтри» → «Ваше»
app-save-search = Зберегти пошук
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Оновити «{ $name }»
app-update-search-title = Зберегти поточний вибір у цьому збереженому пошуку
app-save-search-new = Зберегти як новий
app-select-title = Виберіть кілька ресурсів, щоб разом додати теги, категорію, колекцію, завантажити або перенести в архів (X або Ctrl+клацання)
app-select = Вибрати
app-explore-title = Оглянути ці моделі в 3D
app-tile-size = Розмір плиток
app-supporting-title = Буфери glTF, бібліотеки матеріалів, карти матеріалів, матеріали лише однієї моделі та файли магазинів, що описують моделі. Вони завантажуються разом із тим, що їх використовує. Сторінки спрайт-аркуша та його спрайти, що є в наборі й окремими зображеннями, показуються разом з аркушем. Ресурс, що є в наборі в кількох форматах, показується один раз, у форматі, який Hoard читає найкраще.
app-supporting-hidden =
    { $count ->
        [one] Приховано { $count } допоміжний файл
        [few] Приховано { $count } допоміжні файли
        [many] Приховано { $count } допоміжних файлів
       *[other] Приховано { $count } допоміжного файлу
    }
app-supporting-show = Показати
app-supporting-shown = Разом із допоміжними файлами
app-supporting-hide = Сховати
app-folders = Теки
app-folders-title = Переглядати набір так, як він лежав на диску, теку за текою
app-folders-label = Теки набору
app-welcome-title = Перетягніть пакет ресурсів будь-куди на цій сторінці
app-welcome-before = Підходять zip, tar-архіви, теки чи окремі файли. Hoard розпаковує їх, зберігає кожен файл один раз за його хешем, зчитує розміри, тривалість і кількість полігонів та додає теги до всього, щоб ви могли шукати, наприклад,{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }або{ " " }
app-welcome-after = .
app-welcome-import-folder = Імпортувати теку
app-welcome-choose-files = Вибрати файли
app-none-similar = Поки що немає нічого схожого на { $name }.
app-none = Нічого не знайдено.
app-none-query = За запитом «{ $query }» нічого не знайдено.
app-none-imitation = Жоден виміряний звук поки не схожий на «{ $word }».
app-none-heard = Жоден виміряний звук поки не схожий на ваш.
app-none-filters = З цими фільтрами нічого не знайдено.
app-none-query-filters = За запитом «{ $query }» з цими фільтрами нічого не знайдено.
app-clear-search-filters = Очистити пошук і фільтри
app-loading-more = Завантаження…
app-keys-hint = Стрілки переміщують, Enter відкриває, пробіл відтворює звук, X або Ctrl+клацання вибирає кілька (на сенсорному екрані — довге натискання).
app-resize-label = Змінити розмір панелі відомостей
app-resize-title = Перетягніть, щоб змінити розмір · двічі клацніть, щоб підібрати ширину
app-selected-label = Вибрані ресурси
app-selected = Вибрано: { $count }
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Редагувати…
app-edit-selected-title = Змінити теги та поля всіх (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] Вибрано { $count } результат
        [few] Вибрано { $count } результати
        [many] Вибрано { $count } результатів
       *[other] Вибрано { $count } результату
    }
app-pick-assets = Виберіть ресурси
app-download-zip = Завантажити zip
app-credits-title = CREDITS.md для пакетів, з яких вони походять: хто їх створив, їхні ліцензії та рядки подяки, про які просять автори
app-credits = Автори
app-prefer-sources-title = SoundWave і текстури Unreal, оригінальний файл яких Hoard відновив
app-prefer-sources = Відновлені WAV/PNG замість .uasset ({ $count })
bag-clear = Очистити вибір
bag-glb-heading = Моделі як GLB
bag-glb-text = Кожна модель — файлом .glb, що стоїть віссю Y вгору й обернений уперед, а її текстури — окремими файлами в спільній для моделей теці textures. Усе інше потрапляє як є.
bag-glb-download = Завантажити zip із GLB
bag-plain-heading = Файли як є
bag-plain-text = Кожен файл у тому вигляді, в якому його імпортовано, з усім, що йому потрібно, у теці для кожного пакета.
bag-remove = Прибрати { $name } з вибору
bag-more = і ще { $count }
app-bulk-tag-label = Тег, який треба додати до вибраних ресурсів або зняти з них
app-bulk-tag-placeholder = Тег
app-bulk-tag-add-title = Додати цей тег до кожного вибраного ресурсу
app-bulk-tag-add = Додати
app-bulk-tag-remove-title = Зняти цей тег з кожного вибраного ресурсу
app-bulk-tag-remove = Зняти
app-remove-from-collection = Вилучити з { $name }
app-unfavorite = Прибрати з обраного
app-favorite = В обране
app-select-all = Вибрати всі ({ $count })
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] Вибрати { $count } результат
        [few] Вибрати всі { $count } результати
        [many] Вибрати всі { $count } результатів
       *[other] Вибрати всі { $count } результату
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] Редагувати { $count } ресурс
        [few] Редагувати { $count } ресурси
        [many] Редагувати { $count } ресурсів
       *[other] Редагувати { $count } ресурсу
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] Редагувати { $count } результат
        [few] Редагувати всі { $count } результати
        [many] Редагувати всі { $count } результатів
       *[other] Редагувати всі { $count } результату
    }
bulk-edit-close = Закрити
bulk-edit-tags = Теги
bulk-edit-tag-add = Додати тег усім
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } з { $count }
bulk-edit-untick = Зняти всі позначки
bulk-edit-untick-title = Прибрати всі теги, а потім позначити ті, що треба залишити
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = Найуживаніші теги серед них
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Різні значення, не змінюються
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Не змінюється
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Очищено в усіх
bulk-edit-reset = Повернути як було
bulk-edit-all-note = Це змінить усі результати, зокрема ще не завантажені, і скасувати це одним кроком не можна.
bulk-edit-save =
    { $count ->
        [one] Зберегти { $count } зміну
        [few] Зберегти { $count } зміни
        [many] Зберегти { $count } змін
       *[other] Зберегти { $count } зміни
    }
bulk-edit-nothing = Нічого не змінено
bulk-edit-saved =
    { $count ->
        [one] Оновлено { $count } ресурс
        [few] Оновлено { $count } ресурси
        [many] Оновлено { $count } ресурсів
       *[other] Оновлено { $count } ресурсу
    }
app-archive-selected-title = Зникнуть із пошуку, доки їх не відновлять з архіву або не видалять остаточно
app-drop-items =
    { $count ->
        [one] Відпустіть, щоб додати { $count } елемент
        [few] Відпустіть, щоб додати { $count } елементи
        [many] Відпустіть, щоб додати { $count } елементів
       *[other] Відпустіть, щоб додати { $count } елемента
    }
app-drop = Відпустіть, щоб додати до Hoard
app-drop-hint = Кожен файл або тека верхнього рівня стає окремим пакетом. Гігабайти — не проблема; великі файли продовжать вивантажуватися, якщо з’єднання обірветься.

## Палітра команд (CommandPalette.vue, з командами з App.vue)

palette-group-go-to = Перейти
palette-group-actions = Дії
palette-group-theme = Тема
palette-group-news = Що нового
palette-news-show = Показати, що нового
palette-news-mark-all = Позначити все нове як переглянуте
palette-news-keywords = нове новинки зміни функції можливості випуск оновлення переглянуте new news
palette-group-kinds = Типи
palette-group-packs = Пакети
palette-group-collections = Колекції
palette-group-projects = Проєкти
palette-group-saved = Збережені пошуки
palette-group-assets = Ресурси
palette-hint-current = поточна
palette-library = Бібліотека
palette-library-keywords = ресурси сітка головна assets grid home
palette-favorites = Обране
palette-favorites-keywords = зірочка зірки вподобане улюблене starred stars liked
palette-packs = Пакети
palette-archive = Архів
palette-archive-keywords = кошик смітник видалене відновити trash recycle bin deleted restore
palette-stats = Статистика бібліотеки
palette-stats-keywords = статистика числа розмір диск місце кількість statistics numbers size disk space count
palette-copies = Майже копії
palette-copies-keywords = дублікати однакові схожі зменшені конвертовані втрачене місце duplicates same similar resized converted wasted space dedupe
palette-search = Пошук
palette-add-files = Додати файли…
palette-add-files-keywords = вивантажити імпорт upload import
palette-import-folder = Імпортувати теку…
palette-import-folder-keywords = вивантажити пакет upload pack
palette-select-all =
    { $count ->
        [one] Вибрати { $count } показаний
        [few] Вибрати всі { $count } показані
        [many] Вибрати всі { $count } показаних
       *[other] Вибрати всі { $count } показаного
    }
palette-select-all-keywords = позначити check
palette-select-several = Вибрати кілька…
palette-select-several-keywords = масово кілька тег архів bulk multiple tag archive
palette-archive-selected =
    { $count ->
        [one] Перенести в архів { $count } вибраний
        [few] Перенести в архів { $count } вибрані
        [many] Перенести в архів { $count } вибраних
       *[other] Перенести в архів { $count } вибраного
    }
palette-archive-selected-keywords = видалити прибрати кошик delete remove trash
palette-edit-selected = Редагувати вибрані ({ $count })
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = масова зміна теги категорія ліцензія оцінка
palette-paste-tags =
    { $count ->
        [1] Вставити скопійовані теги до вибраного ресурсу
       *[other] Вставити скопійовані теги до { $count } вибраних
    }
palette-clear-selection = Скасувати вибір
palette-clear-filters = Очистити фільтри
palette-clear-filters-keywords = скинути reset
palette-hide-supporting = Сховати допоміжні файли
palette-show-supporting = Показати допоміжні файли
palette-supporting-keywords = карти нормалей шорсткість маски normal maps roughness masks
palette-account = Обліковий запис і токени доступу
palette-account-keywords = пароль вхід користувач токен api password login user token
palette-sign-out = Вийти ({ $name })
palette-sign-out-keywords = вийти вихід log out logout
palette-sign-in = Увійти
palette-sign-in-keywords = увійти вхід обліковий запис пароль log in login account password
palette-settings = Налаштування
palette-settings-keywords = blender ffmpeg llm ші ai тегування tagging unreal адміністратор admin інструменти tools параметри preferences
palette-shortcuts = Комбінації клавіш
palette-shortcuts-keywords = довідка клавіші гарячі help keys
palette-theme-keywords = тема вигляд режим theme appearance mode
palette-theme-variant-keywords = вигляд колір appearance color
palette-light-theme = Світла тема: { $name }
palette-dark-theme = Темна тема: { $name }
palette-kind-all = Усі типи (не лише { $kind })
palette-kind-only = Лише { $kind }
palette-title = Палітра команд
palette-placeholder = Шукайте ресурси, переходьте до пакета, виконуйте команди…
palette-results = Результати
palette-searching = Пошук…
palette-no-match = За запитом «{ $query }» нічого не знайдено
palette-foot-move = перейти
palette-foot-run = виконати
palette-foot-shortcuts = усі комбінації

## Комбінації клавіш (ShortcutsDialog.vue)

shortcuts-title = Комбінації клавіш
shortcuts-close = Закрити
shortcuts-anywhere = Будь-де
shortcuts-palette = Палітра команд: пошук, перехід, виконання
shortcuts-search = Пошук
shortcuts-this-list = Цей список
shortcuts-escape = Закрити, скасувати вибір або закрити відомості
shortcuts-library = Бібліотека
shortcuts-move = Переміщення між плитками (або J і K)
shortcuts-open = Відкрити відомості
shortcuts-play-sound = Відтворити або зупинити звук
shortcuts-select = Вибрати або скасувати вибір
shortcuts-favorite = Додати в обране або прибрати з обраного
shortcuts-reviewed = Позначити як переглянутий або зняти позначку
shortcuts-tag = Додати тег
shortcuts-edit = Редагувати вибране
shortcuts-copy-tags = Скопіювати його теги
shortcuts-paste-tags = Вставити скопійовані теги до вибраного
shortcuts-key-click = клацання
shortcuts-select-range = Вибрати діапазон
shortcuts-select-all = Вибрати все показане
shortcuts-sound-player = Звуковий програвач
shortcuts-play-pause = Відтворити або призупинити
shortcuts-skip = Перемотати назад або вперед
shortcuts-start-end = На початок або в кінець
shortcuts-image-inspector = Перегляд зображень
shortcuts-zoom = Збільшити або зменшити
shortcuts-fit = Умістити у вікно
shortcuts-actual-pixels = Справжні пікселі
shortcuts-tile = Розкласти плиткою 3×3, щоб перевірити шви
shortcuts-background = Наступне тло
shortcuts-channel = Показати один канал

## KindChips.vue

kinds-group = Типи ресурсів

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = додано людиною
tag-source-rule = від офлайн-тегувальника
tag-source-meta = виміряно з файлу
tag-source-ai = запропоновано ШІ
tag-remove = Прибрати тег { $tag }
tag-accept = Залишити тег { $tag }
tag-accept-title = Залишити це припущення як власний тег
tag-reject-title = Прибрати; тегувальник не поверне його
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Новий тег
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Ви мали на увазі
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = раніше { $alias }

## ActivityIndicator.vue

activity-label-idle = Фонова активність: простій
activity-label-running = Фонова активність: виконується { $running }
activity-label-queued = Фонова активність: виконується { $running }, у черзі { $queued }
activity-title = Фонова активність
activity-unreachable = Не вдається зв’язатися із сервером
activity-unreachable-desktop = Hoard не відповідає
activity-empty = Нічого не виконується. Тут з’являються імпорт, тегування, перетворення й завантаження zip, поки вони тривають.
activity-queued = у черзі
activity-stop = Зупинити «{ $what }»
activity-stop-tagging = Зупинити все тегування ШІ ({ $count })
activity-seconds = { $s } с
activity-minutes = { $min } хв
activity-hours = { $h } год { $min } хв
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles =
    { $count ->
        [one] { $count } трикутник
        [few] { $count } трикутники
        [many] { $count } трикутників
       *[other] { $count } трикутника
    }
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Відтворити { $name }
tile-pause = Призупинити { $name }
tile-favorite = Додати { $name } в обране
tile-in-favorites = В обраному
tile-add-favorite = Додати в обране (F)
tile-more-like = Більше схожого на { $name }
tile-more-like-this = Більше схожого
tile-select = Вибрати { $name }

## CollectionPicker.vue

collections-add = Додати до колекції
collections-menu = Колекції
collections-find-label = Знайти або створити колекцію
collections-find-placeholder = Знайдіть колекцію або назвіть нову
collections-create = Створити
collections-empty = Колекцій ще немає. Введіть назву, щоб створити першу.

## FolderPicker.vue

folders-list = Теки
folders-close = Закрити
folders-browse = Огляд…
folders-up-title = На одну теку вгору
folders-up = Вгору
folders-this-computer = Цей комп’ютер
folders-none = Тут немає тек.

## LocationForm.vue

locations-mode-question = Що Hoard має робити з цією текою?
locations-mode-managed = Забрати собі: Hoard зберігає тут мої пакети
locations-mode-managed-hint = Пакети, які ви вивантажуєте або імпортуєте, копіюються сюди, і Hoard тримає їх у порядку. Тепер тека належить Hoard, тож не змінюйте її вміст.
locations-mode-watched = Стежити: мої пакети вже тут
locations-mode-watched-hint = Кожна тека всередині стає пакетом. Hoard читає файли там, де вони є, ніколи не змінює, не переміщує й не видаляє їх і підхоплює нові теки, щойно вони з’являються.
locations-folder-desktop = Тека
locations-folder-placeholder = Тека, де лежать ваші пакети
locations-folder-hint-managed = Hoard створить її, якщо її немає. У Docker це тека, яку ви змонтували в контейнер.
locations-folder-hint-managed-desktop = Hoard створить її, якщо її немає.
locations-folder-hint-watched = У Docker спершу змонтуйте теку в контейнер (можна лише для читання), а потім виберіть її тут.
locations-folder-hint-watched-desktop = Hoard лише читає її: нічого в ній не змінюється.
locations-name = Назва
locations-name-placeholder-managed = Бібліотека
locations-name-placeholder-watched = Мої пакети
locations-files = Файли, що потрапляють сюди
locations-catch-all = А також усе, чого не беруть інші розташування
locations-kinds-hint = Файли нового пакета потрапляють у розташування, де позначено їхній тип. Решта йде в резервне.
locations-files-all = Поки що всі. Пізніше в налаштуваннях можна додати інші розташування, щоб надсилати деякі типи (скажімо, музику) на інший диск.
locations-first-watched-note = Для вивантаження потрібна бібліотека, якою опікується Hoard. Її можна додати пізніше в «Налаштування → Розташування бібліотеки».
locations-checking = Перевірка теки…
locations-add = Додати розташування
locations-cancel = Скасувати

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Не вдалося завантажити розташування бібліотеки
locations-takes-and-rest = { $kinds } та все інше
locations-takes-rest = Усе, чого не беруть інші
locations-takes-everything = Усе
locations-takes-nothing = Поки нічого
locations-save-failed = Не вдалося зберегти розташування
locations-unwatch-title = Припинити відстежувати { $name }?
locations-unwatch-body =
    { $count ->
        [one] Її { $count } пакет зникне
        [few] Її { $count } пакети зникнуть
        [many] Її { $count } пакетів зникне
       *[other] Її { $count } пакета зникне
    } з Hoard разом із тегами й нотатками. Файлів у теці це не торкнеться.
locations-unwatch-body-empty = Файлів у теці це не торкнеться.
locations-unwatch = Припинити відстеження
locations-remove-title = Вилучити { $name }?
locations-remove-body = Усі файли з нього спершу переміщуються в інші розташування за типом; воно зникне, щойно спорожніє. Нічого не буде втрачено.
locations-remove-body-only = Якщо в ньому є файли, спершу додайте інше розташування, куди їх можна перемістити.
locations-remove = Вилучити
locations-removed = { $name } вилучено
locations-emptying = Файли переміщуються з { $name }; його буде вилучено, коли це завершиться
locations-offline-title = { $name } недоступне
locations-offline-body = Поки воно офлайн, файли з нього не можна перемістити. Усе одно вилучити? Файли, що залишаться на ньому, показуватимуться як відсутні.
locations-remove-anyway = Усе одно вилучити
locations-remove-failed = Не вдалося вилучити { $name }
locations-rebalancing = Збережені файли переміщуються в розташування, які беруть їхній тип
locations-rebalance-failed = Не вдалося перемістити файли
locations-scanned-never = ще не скановано
locations-scanned-now = щойно скановано
locations-scanned-minutes = скановано { $mins } хв тому
locations-scanned-hours = скановано { $hours } год тому
locations-scanned-date = скановано { $date }
locations-scan-failed = Не вдалося сканувати { $name }
locations-watching = Відстежується { $name }
locations-added = Додано { $name }
locations-loading = Завантаження…
locations-managed-title = Бібліотеки, якими опікується Hoard
locations-watched-title = Відстежувані теки
locations-managed-hint = Сюди копіюються вивантаження та імпорт. Кожне розташування бере позначені в ньому типи; резервне бере решту.
locations-watched-hint = Теки з пакетами, проіндексовані там, де вони є. Hoard ніколи не змінює файли в них і шукає нові та вилучені теки кожні 15 хвилин.
locations-managed-empty = Поки жодної, тож вивантаженням нікуди потрапити. Додайте бібліотеку нижче.
locations-reachable = Доступне
locations-unreachable-now = Зараз недоступне
locations-catch-all-tag = резервне
locations-packs =
    { $count ->
        [one] { $count } пакет
        [few] { $count } пакети
        [many] { $count } пакетів
       *[other] { $count } пакета
    }
locations-unreachable = недоступне
locations-scanning = Сканування…
locations-scan = Сканувати зараз
locations-edit = Змінити
locations-catch-all-locked = Щоб змінити це, зробіть резервним інше розташування
locations-edit-hint-before = Стосується файлів, доданих відтепер. Щоб перемістити вже збережене, скористайтеся кнопкою
locations-edit-hint-after = нижче.
locations-save = Зберегти
locations-rebalance = Перемістити збережені файли відповідно
locations-rebalance-hint = Після зміни того, що бере розташування: уже збережені файли переміщуються туди, куди тепер іде їхній тип.
locations-add-title = Додати розташування
locations-add-hint = Інший диск для деяких типів файлів або тека з пакетами, які у вас уже є.
locations-kind-question = Яке це розташування?
locations-kind-folder = Тека
locations-kind-folder-hint = На диску чи мережевій теці NAS, змонтованій у контейнер, або тека, яку синхронізує Dropbox, Google Drive, OneDrive чи iCloud Drive. Hoard може зберігати там файли або стежити за пакетами, які у вас уже є.
locations-kind-folder-hint-desktop = На цьому комп’ютері, іншому диску чи мережевій теці NAS, або тека, яку синхронізує Dropbox, Google Drive, OneDrive чи iCloud Drive. Hoard може зберігати там файли або стежити за пакетами, які у вас уже є.
locations-kind-s3-hint = Бакет в Amazon S3 або схожому сервісі: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard зберігає там файли.
locations-s3-title = Сховище S3
locations-s3-hint = Розташування s3://bucket-name — це бакет у цьому сервісі: AWS або будь-якому сумісному з S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage на NAS).
locations-bucket = Бакет
locations-bucket-hint = Назва бакета, який уже існує. Ключі вище мають дозволяти переглядати його вміст, читати й записувати в нього.
locations-s3-endpoint = Адреса
locations-s3-endpoint-placeholder = Порожньо для AWS; напр. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Регіон
locations-s3-access-key = Ідентифікатор ключа доступу
locations-s3-secret-key = Секретний ключ доступу
locations-s3-headers = Додаткові заголовки
locations-s3-headers-placeholder = Name=value, по одному в рядку (скажімо, для Cloudflare Access)
locations-s3-kept = Збережено (введіть, щоб замінити)
locations-s3-secrets = Ключі й заголовки зберігаються в налаштуваннях сервера й більше ніколи не показуються.
locations-s3-secrets-desktop = Ключі й заголовки зберігаються в сховищі паролів цього комп’ютера й більше ніколи не показуються.
locations-s3-saved = Сховище S3 збережено
locations-s3-save-failed = Не вдалося зберегти сховище S3
locations-add-more = Додати розташування…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Не вдалося завантажити проблеми
issues-started =
    { $count ->
        [1] Запущено знову.
        [one] Знову запущено { $count } завдання.
        [few] Знову запущено { $count } завдання.
        [many] Знову запущено { $count } завдань.
       *[other] Знову запущено { $count } завдання.
    }
issues-retry-failed = Не вдалося повторити
issues-dismiss-failed = Не вдалося відхилити
issues-dismiss-all-title =
    { $count ->
        [one] Відхилити { $count } проблему?
        [few] Відхилити всі { $count } проблеми?
        [many] Відхилити всі { $count } проблем?
       *[other] Відхилити всі { $count } проблеми?
    }
issues-dismiss-all-body = Групу «{ $title }» буде очищено. Усе, що знову піде не так, знову з’явиться в списку.
issues-dismiss-all = Відхилити всі
issues-just-now = щойно
issues-minutes-ago = { $mins } хв тому
issues-hours-ago = { $hours } год тому
issues-loading = Завантаження…
issues-none = Нічого не пішло не так.
issues-action-all = { $action } (усі)
issues-open-asset = Відкрити
issues-open-pack = Показати пакет
issues-open-locations = Розташування бібліотеки
issues-where-before = у
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } раз
        [few] { $count } рази
        [many] { $count } разів
       *[other] { $count } разу
    }
issues-starting = Запуск…
issues-dismiss-named = Відхилити { $title }
issues-dismiss = Відхилити
issues-show-fewer = Показати менше
issues-show-all = Показати всі ({ $count })
issues-not-listed = Ще { $count } не показано.

## archive.ts

archive-anyway = Усе одно перенести в архів
archive-failed = Не вдалося перенести в архів: { $error }
archive-undo = Скасувати
archive-restore-failed = Не вдалося відновити: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } пакет
        [few] { $packs } пакети
        [many] { $packs } пакетів
       *[other] { $packs } пакета
    } і { $assets ->
        [one] { $assets } ресурс
        [few] { $assets } ресурси
        [many] { $assets } ресурсів
       *[other] { $assets } ресурсу
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } пакет
        [few] { $packs } пакети
        [many] { $packs } пакетів
       *[other] { $packs } пакета
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } ресурс
        [few] { $assets } ресурси
        [many] { $assets } ресурсів
       *[other] { $assets } ресурсу
    }
archive-count-nothing = нічого

## ArchiveView.vue

archive-load-failed = Не вдалося завантажити архів: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Відновлено: { $what }
archive-purged = Остаточно видалено: { $what }
archive-purge-failed = Не вдалося видалити: { $error }
archive-purge-title = Остаточно видалити { $name }?
archive-purge-pack-body =
    { $count ->
        [one] Його { $count } ресурс
        [few] Його { $count } ресурси
        [many] Його { $count } ресурсів
       *[other] Його { $count } ресурсу
    } і { $size } файлів теж буде видалено, крім файлів, спільних з іншим пакетом. Це не можна скасувати.
archive-purge = Видалити назавжди
archive-purge-asset-body = Його файл буде звільнено, якщо ним не користується інший ресурс. Це не можна скасувати.
archive-empty-title = Очистити архів?
archive-empty-body = Буде остаточно видалено: { $what }, а їхні файли звільнено. Це не можна скасувати.
archive-empty = Очистити архів
archive-title = Архів
archive-intro = Пакети й ресурси в архіві не потрапляють у пошук, підрахунки та список тегів. Відновіть їх або остаточно видаліть, щоб звільнити місце.
archive-loading = Завантаження…
archive-is-empty = Архів порожній
archive-is-empty-hint = Перенесіть у архів пакет або ресурс, і він чекатиме тут, доки ви його не відновите або остаточно не видалите.
archive-packs = Пакети
archive-assets = Ресурси
archive-asset-count =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурси
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
archive-archived = в архіві { $when }
archive-from = з { $pack }
archive-restore = Відновити
archive-show-more = Показати ще (залишилося { $count })

## mine.ts

projects-favorite-add-failed = Не вдалося додати в обране: { $error }
projects-favorite-remove-failed = Не вдалося прибрати з обраного: { $error }
projects-save-search-failed = Не вдалося зберегти пошук: { $error }
projects-forget-search-failed = Не вдалося вилучити { $name }: { $error }
projects-change-search-failed = Не вдалося змінити { $name }: { $error }

## projects.ts

projects-list-and = { $items } і { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = ще { $count }
projects-other =
    { $count ->
        [one] ще { $count } проєкт
        [few] ще { $count } проєкти
        [many] ще { $count } проєктів
       *[other] ще { $count } проєкту
    }
projects-usage-title-one = { $name } використовує це
projects-usage-title =
    { $count ->
        [one] Це використовує { $count } проєкт
        [few] Це використовують { $count } проєкти
        [many] Це використовують { $count } проєктів
       *[other] Це використовують { $count } проєкту
    }
projects-usage-body-one = Використовується в { $list }: наступний hoard sync прибере їх звідти.
projects-usage-body = Використовується в { $list }: наступний hoard sync кожного з них прибере їх звідти.

## uploader.ts

upload-network-error = помилка мережі
upload-no-reply = немає відповіді { $seconds } с
upload-retrying = Повторна спроба: { $what } ({ $attempt } з { $attempts }): { $error }
upload-gave-up =
    { $what }: { $error } після { $attempts ->
        [one] { $attempts } спроби
        [few] { $attempts } спроб
        [many] { $attempts } спроб
       *[other] { $attempts } спроби
    }
upload-failed-plain = не вдалося
upload-already-imported = уже імпортовано
upload-part-failed = вивантаження не вдалося ({ $status })
upload-batch =
    { $count ->
        [one] партія з { $count } файлу
        [few] партія з { $count } файлів
        [many] партія з { $count } файлів
       *[other] партія з { $count } файлу
    }
upload-batch-failed = партія не вдалася ({ $status })
upload-finish = завершення вивантаження

## UploadQueue.vue

upload-eta-seconds = залишилося { $s } с
upload-eta-minutes = залишилося { $min } хв
upload-eta-hours = залишилося { $h } год
upload-queue = Імпорт
upload-importing =
    { $count ->
        [one] Імпорт { $count } пакета
        [few] Імпорт { $count } пакетів
        [many] Імпорт { $count } пакетів
       *[other] Імпорт { $count } пакета
    }
upload-bytes-of = { $sent } з { $total }
upload-rate = { $size }/с
upload-waiting = Очікування
upload-unpacking = Вивантажено, триває розпакування
upload-already = Уже є в бібліотеці
upload-stopped = Зупинено
upload-files-of = Файли: { $done } з { $total }
upload-resumed = продовжено, { $size } уже було на сервері
upload-skipped = Саме ці файли вже імпортовано як «{ $name }», тому їх пропущено.
upload-error = { $error }. Ваш прогрес збережено: повторіть спробу або пізніше знову перетягніть ті самі файли.
upload-retry = Повторити
# $name is the pack the library has already that these files go into.
upload-update = Оновити «{ $name }»
upload-copy = Імпортувати як копію
# $name is the pack the library has already that the upload goes into.
upload-into = Додається до пакета «{ $name }», який уже є в бібліотеці: нові файли додаються, змінені замінюються.
upload-own-pack = Зробити окремим пакетом
upload-dismiss = Закрити
upload-cancel = Скасувати
upload-minimize = Згорнути
upload-expand = Показати імпорт
upload-pill = Імпорт: { $count } · { $percent }%
upload-pill-stopped = Зупинено: { $count }
upload-interrupted = Перервано
upload-resume = Продовжити
upload-resume-all = Продовжити всі
upload-resume-hint-folder = Виберіть ту саму теку знову або перетягніть її на сторінку, щоб продовжити з місця, де імпорт зупинився.
upload-resume-hint-file = Виберіть той самий файл знову або перетягніть його на сторінку, щоб продовжити з місця, де імпорт зупинився.
upload-not-same = Це не ті самі файли, тож нічого не продовжено.
upload-permission-denied = Браузеру не дозволили знову прочитати файли. Виберіть їх вручну.

## PacksView.vue

packs-rename-failed = Не вдалося перейменувати: { $error }
packs-collection-deleted = Колекцію { $name } видалено
packs-delete-failed = Не вдалося видалити: { $error }
packs-status-uploading = Вивантаження (отримані досі файли)
packs-status-ingesting = Розпакування й тегування
packs-status-analyzing = LLM додає теги
packs-status-ready = Готово
packs-status-failed = Помилка
packs-retag-failed = Не вдалося почати повторне тегування: { $error }
packs-read-files-failed = Не вдалося прочитати файли пакета: { $error }
packs-context-load-failed = Не вдалося завантажити відомості про пакет із магазину: { $error }
packs-undo-store-failed = Не вдалося відв’язати сторінку магазину: { $error }
packs-context-save-failed = Не вдалося зберегти відомості з магазину: { $error }
packs-unreal-load-failed = Не вдалося завантажити стан перетворення Unreal: { $error }
packs-unreal-discard-title = Позбутися оригіналів?
packs-unreal-discard-body = Оригінальні файли .uasset у { $name } буде видалено після перетворення. Дані, які є лише в Unreal, як-от графи матеріалів, буде втрачено назавжди.
packs-unreal-discard-confirm = Перетворити й видалити
packs-unreal-convert-failed = Не вдалося почати перетворення: { $error }
packs-unreal-fact-converted = Перетворено з Unreal: { $done }
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } пакунок Unreal
        [few] { $left } пакунки Unreal
        [many] { $left } пакунків Unreal
       *[other] { $left } пакунка Unreal
    }
packs-unreal-fact-both = Перетворено з Unreal: { $done }, ще не перетворено: { $left }
packs-thumbs-canceled = Скасовано після { $done } з { $total }
packs-thumbs-drew =
    { $count ->
        [one] Намальовано { $count } модель
        [few] Намальовано { $count } моделі
        [many] Намальовано { $count } моделей
       *[other] Намальовано { $count } моделі
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: замінено { $improved } мініатюру.
        [few] { $lead }: замінено { $improved } мініатюри.
        [many] { $lead }: замінено { $improved } мініатюр.
       *[other] { $lead }: замінено { $improved } мініатюри.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: замінено { $improved } мініатюру, не вдалося намалювати: { $failed }.
        [few] { $lead }: замінено { $improved } мініатюри, не вдалося намалювати: { $failed }.
        [many] { $lead }: замінено { $improved } мініатюр, не вдалося намалювати: { $failed }.
       *[other] { $lead }: замінено { $improved } мініатюри, не вдалося намалювати: { $failed }.
    }
packs-archived =
    { $count ->
        [one] { $name } і його { $count } ресурс перенесено в архів
        [few] { $name } і його { $count } ресурси перенесено в архів
        [many] { $name } і його { $count } ресурсів перенесено в архів
       *[other] { $name } і його { $count } ресурсу перенесено в архів
    }
packs-blobs-bad =
    { $count ->
        [one] { $count } файл не пройшов останню перевірку цілісності.
        [few] { $count } файли не пройшли останню перевірку цілісності.
        [many] { $count } файлів не пройшли останню перевірку цілісності.
       *[other] { $count } файлу не пройшли останню перевірку цілісності.
    }
packs-collections = Колекції
packs-browse-named = Переглянути { $name }
packs-collection-name = Назва колекції
packs-save = Зберегти
packs-cancel = Скасувати
packs-assets =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурси
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
packs-updated = Оновлено { $date }
packs-browse = Переглянути
packs-collection-download = Завантажити (.zip)
packs-collection-credits = Автори
packs-collection-credits-title = Хто створив пакети, з яких походять ці ресурси, їхні ліцензії та рядки подяки, про які просять автори
packs-rename = Перейменувати
packs-delete = Видалити
packs-packs = Пакети
packs-empty = Пакетів ще немає. Перетягніть zip або теку будь-куди на цій сторінці, щоб додати пакет.
packs-find = Знайти пакет
packs-find-placeholder = Шукайте пакет за назвою, магазином, ліцензією чи тегом
packs-sort = Порядок
packs-sort-new = Спершу нові
packs-sort-old = Спершу старі
packs-sort-size = Спершу найбільші
packs-sort-assets = Спершу з найбільшою кількістю ресурсів
packs-sort-count = Кількість ресурсів
packs-sort-smallest = Спершу найменші
packs-sort-fewest = Спершу з найменшою кількістю ресурсів
packs-with-kind = Пакети з
packs-state = Стан
packs-state-any = Будь-який стан
packs-state-working = В обробці
packs-state-failed = З помилкою
packs-state-untagged = Ще не описані
packs-state-unlicensed = Ліцензію не вказано
packs-store = Магазин
packs-store-any = Будь-який магазин
packs-store-from = З { $store }
packs-store-none = Без сторінки магазину
packs-shown-of = { $shown } з { $total }
packs-clear = Очистити
packs-no-match = Немає відповідних пакетів.
packs-clear-all = Очистити пошук і фільтри
packs-pack-name = Назва пакета
packs-thumbs-drawing = Малювання мініатюр у цьому браузері…
packs-thumbs-drawing-count = Малювання мініатюр у цьому браузері: { $done } з { $total }…
packs-thumbs-stopped = Зупинено: { $error }
packs-thumbs-none = Кожна модель тут уже має мініатюру, намальовану з усіма текстурами.
packs-dismiss = Закрити
packs-license = Ліцензія: { $license }
packs-license-tier = Ліцензія: { $license } ({ $tier })
packs-license-not-found = не знайдено
packs-from-before = З
packs-from-after = { "" }
packs-added = Додано { $date }
pack-panel-label = Про цей пакет
pack-panel-hide = Сховати відомості про пакет
pack-cover-previous = Попереднє зображення
pack-cover-next = Наступне зображення
pack-cover-picture = Зображення { $n } з { $count }
pack-panel-kinds = Що всередині
pack-panel-all = Усі пакети
packs-download = Завантажити пакет (.zip)
packs-store-details = Відомості з магазину
packs-more-named = Інші дії з { $name }
packs-more = Більше
packs-rename-more = Перейменувати…
packs-retag-rules = Знову запустити офлайн-тегувальник
packs-retag-llm = Знову запустити LLM-тегувальник
packs-make-thumbs = Створити мініатюри в цьому браузері
packs-make-thumbs-title = Відкрити кожну модель у прихованому 3D-переглядачі по одній і взяти її зображення за мініатюру там, де воно краще за серверне
packs-make-thumbs-title-desktop = Відкрити кожну модель у прихованому 3D-переглядачі по одній і взяти її зображення за мініатюру там, де воно краще за ту, що створив Hoard
packs-read-files = Знову прочитати файли пакета
packs-credits = Завантажити список авторів
packs-credits-title = CREDITS.md: хто створив пакет, його ліцензія та рядок подяки, про який просить автор
packs-unreal-open = Перетворити вміст Unreal…
packs-archive = Перенести пакет в архів
packs-unreal-intro = Hoard сам читає пакунки Unreal і зберігає їхній вміст як звичайні файли: текстури як PNG чи EXR, звуки як WAV, матеріали як JSON. Перетворений файл займає місце пакунка й зберігає його теги, посилання та колекції.
packs-unreal-converted =
    { $count ->
        [one] перетворено
        [few] перетворено
        [many] перетворено
       *[other] перетворено
    }
packs-unreal-converted-kept =
    { $count ->
        [one] перетворено, з них { $kept } зі збереженим оригіналом
        [few] перетворено, з них { $kept } зі збереженим оригіналом
        [many] перетворено, з них { $kept } зі збереженим оригіналом
       *[other] перетворено, з них { $kept } зі збереженим оригіналом
    }
packs-unreal-pending =
    { $count ->
        [one] ще не переглянуто
        [few] ще не переглянуто
        [many] ще не переглянуто
       *[other] ще не переглянуто
    }
packs-unreal-needs-editor =
    { $count ->
        [one] чекає на Unreal Editor
        [few] чекають на Unreal Editor
        [many] чекають на Unreal Editor
       *[other] чекають на Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] не має поширеного формату
        [few] не мають поширеного формату
        [many] не мають поширеного формату
       *[other] не мають поширеного формату
    }
packs-unreal-failed =
    { $count ->
        [one] з помилкою
        [few] з помилкою
        [many] з помилкою
       *[other] з помилкою
    }
packs-unreal-reasons = Чому пакунки залишилися
packs-unreal-editor-before = Мешам і анімаціям потрібен Unreal Editor. На комп’ютері з установленим Unreal виконайте
packs-unreal-editor-command = hoard unreal-worker --editor <шлях до UnrealEditor-Cmd>
packs-unreal-editor-after = ; команда експортує їх у FBX і вивантажить результати сюди.
packs-unreal-discard = Видалити оригінальні файли .uasset після перетворення
packs-unreal-discard-hint = Звільняє місце. Дані, які є лише в Unreal, як-от графи матеріалів, буде втрачено.
packs-unreal-drop = Видалити пакунки без поширеного формату
packs-unreal-drop-hint = Blueprint, рівні та інші ресурси, що існують лише в рушії.
packs-unreal-convert = Перетворити
packs-context-hint-before = Звідки пакет і що про нього каже продавець. LLM-тегувальник читає це разом із кожним файлом, тож опис із магазину на кшталт «120 заклинань льоду й вогню» допомагає йому назвати
packs-context-hint-after = .
packs-store-auto = Знайдено автоматично: пакет сам згадує цю сторінку.
packs-store-not-this = Не ця
packs-lookup-label = Шукати в магазинах
packs-lookup-placeholder = Назва пакета в магазині
packs-lookup-searching = Пошук…
packs-lookup-search = Шукати в магазинах
packs-lookup-hide = Сховати
packs-lookup-clues = З файлів пакета: { $clues }.
packs-lookup-also = Також шукали { $queries }.
packs-lookup-quoted = «{ $query }»
packs-lookup-nothing = На Fab, itch.io, Kenney та OpenGameArt немає нічого схожого. Спробуйте менше слів або інші.
packs-lookup-use = Використати
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Зараз
packs-review-store = З магазину
packs-review-same = Однаково
packs-review-read-page = Під час збереження буде прочитано зі сторінки
packs-review-take =
    { $count ->
        [one] Заповнити { $count } поле
        [few] Заповнити { $count } поля
        [many] Заповнити { $count } полів
       *[other] Заповнити { $count } поля
    }
packs-lookup-searched = Шукали { $date }.
packs-lookup-again = Шукати знову
packs-lookup-again-after = .
packs-lookup-unreachable = Цього разу не вдалося зв’язатися з { $stores }.
packs-lookup-also-look = Також пошукайте на
packs-lookup-links-and = { " " }і{ " " }
packs-lookup-browser-only = (там можна шукати лише у вашому браузері); вставте адресу сторінки нижче.
packs-lookup-open = Знайти в магазинах…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Продавець
packs-seller-placeholder = Хто створив
packs-license-label = Ліцензія
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Рівень ліцензії
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] У { $count } ресурсу своя ліцензія: перемкнути на ліцензію пакета
        [few] У { $count } ресурсів своя ліцензія: перемкнути на ліцензію пакета
        [many] У { $count } ресурсів своя ліцензія: перемкнути на ліцензію пакета
       *[other] У { $count } ресурсу своя ліцензія: перемкнути на ліцензію пакета
    }
packs-store-page = Сторінка магазину
packs-store-text = Опис із магазину
packs-store-text-placeholder = Вставте текст оголошення або залиште порожнім, щоб отримати його зі сторінки магазину.
packs-store-text-fetch = Під час збереження опис буде отримано зі сторінки магазину.
packs-readme = README, знайдений у пакеті
packs-save-retag = Зберегти й знову запустити LLM-тегувальник

## AssetDetail.vue

asset-lazy-3d-viewer = 3D-переглядач
asset-lazy-text = текст
asset-lazy-text-viewer = переглядач тексту
asset-lazy-image-viewer = переглядач зображень
asset-lazy-material-preview = попередній перегляд матеріалу
asset-lazy-material-viewer = переглядач матеріалів
asset-lazy-sprite-player = програвач спрайтів
asset-lazy-sheet-view = переглядач аркушів спрайтів
asset-rel-texture = текстура
asset-rel-buffer = дані геометрії
asset-rel-material = файл матеріалу
asset-rel-animation = анімація
asset-rel-library = пов’язаний файл
asset-rel-guess = імовірно, пов’язаний
asset-rel-sidecar = відомості з магазину
asset-rel-variant = варіант
asset-rel-sprite = спрайт
asset-rel-sample = семпл
asset-role-map = карта { $role }
asset-maps-count =
    { $count ->
        [one] { $count } карта
        [few] { $count } карти
        [many] { $count } карт
       *[other] { $count } карти
    }
asset-needs = Потребує
asset-animations = Анімації
asset-sprites = Спрайти
asset-used-by = Використовується в
asset-loose-sprites = Також окремими файлами
asset-in-sheet = У спрайт-аркуші
asset-other-formats = Інші формати
asset-shown-as = У бібліотеці показано як
asset-conversion-failed = перетворення не вдалося
asset-as-converting = Перетворення…
asset-as-converting-percent = Перетворення… { $percent }%
asset-as-queued = У черзі…
asset-download = Завантажити
asset-as-label = Завантажити як
asset-as-original = { $format } (оригінальний формат)
asset-as-pack-files = Файли з набору
asset-as-size = Розмір
asset-as-full-size = Повний розмір
asset-as-up-to = До { $size } пікс.
asset-as-hint = Виберіть розмір або інший формат
asset-meta-size = Розмір
asset-meta-length = Тривалість
asset-meta-sample-rate = Частота дискретизації
asset-meta-channels = Канали
asset-meta-mono = Моно
asset-meta-stereo = Стерео
asset-meta-bit-depth = Бітова глибина
asset-meta-bits = { $bits } біт
asset-meta-level = Рівень
asset-meta-level-value = пік { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = пік { $peak } dBFS, RMS { $rms } dBFS, є перевантаження
asset-meta-loop = Цикл
asset-meta-loop-value = від { $start } до { $end }
asset-meta-playback = Відтворення
asset-meta-one-shot = Одноразове
asset-meta-envelope = Обвідна
asset-meta-envelope-times = атака { $attack }, згасання { $tail }
asset-meta-envelope-hit = удар
asset-meta-envelope-swell = наростання
asset-meta-envelope-sustained = протяжний
asset-meta-tone = Тембр
asset-meta-tone-value = яскравість { $khz } кГц, шум { $noise } %
asset-meta-pitch = Висота
asset-meta-pitch-rises =
    { $octaves ->
        [one] підвищується на { $octaves } октаву
        [few] підвищується на { $octaves } октави
        [many] підвищується на { $octaves } октав
       *[other] підвищується на { $octaves } октави
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] знижується на { $octaves } октаву
        [few] знижується на { $octaves } октави
        [many] знижується на { $octaves } октав
       *[other] знижується на { $octaves } октави
    }
asset-meta-loops-cleanly = Безшовно зациклюється
asset-meta-tempo = Темп
asset-meta-tempo-heard = близько { $bpm } BPM, на слух
asset-meta-tempo-heard-or = близько { $bpm } BPM (або { $alt }), на слух
asset-meta-key = Тональність
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, на слух
music-key-major = { $note } мажор
music-key-minor = { $note } мінор
asset-meta-root-note = Тоніка
asset-meta-cue-markers = Маркери
asset-meta-library = Бібліотека
asset-meta-keywords = Ключові слова
asset-meta-title = Назва
asset-meta-artist = Виконавець
asset-meta-album = Альбом
asset-meta-genre = Жанр
asset-meta-generator = Генератор
asset-meta-everything-with = Показати все з тегом { $tag }
asset-meta-saved-with = Збережено в
asset-meta-color-mode = Колірний режим
asset-meta-layers = Шари
asset-meta-range = Діапазон
asset-meta-hdr = HDR (попередній перегляд із тональною компресією)
asset-meta-format = Формат
asset-meta-large-document = Великий документ (PSB)
asset-meta-ai = ШІ
asset-meta-ai-made = Створено генеративним ШІ
asset-meta-ai-edited = Відредаговано генеративним ШІ
asset-meta-ai-how-metadata = метадані
asset-meta-ai-how-generator = налаштування генератора
asset-meta-ai-how-iptc = тип цифрового джерела IPTC
asset-meta-ai-says = { $made } (джерело: { $how })
asset-meta-ai-by-says = { $made } ({ $by }; джерело: { $how })
asset-meta-provenance = Походження
asset-meta-c2pa = Має облікові дані вмісту C2PA
asset-meta-compression = Стиснення
asset-meta-note = Примітка
asset-meta-maps = Карти
asset-meta-sizes =
    { $count ->
        [one] Розмір
        [few] Розміри
        [many] Розміри
       *[other] Розміри
    }
asset-meta-triangles = Трикутники
asset-meta-vertices = Вершини
asset-meta-meshes = Меші
asset-meta-materials = Матеріали
asset-meta-animations = Анімації
asset-meta-skins = Скіни
asset-meta-unreal-asset = Ресурс Unreal
asset-meta-unreal-saved-with = { $class }, збережено в UE { $engine }
asset-meta-package = Пакунок
asset-meta-cooked = Скомпільований (збірка гри): попередні перегляди й джерела вилучено
asset-meta-tiling = Тайлінг
asset-meta-seamless = Безшовний по обох осях
asset-meta-analysis-error = Не вдалося повністю прочитати цей файл: { $error }
asset-copy-blocked = Не вдалося скопіювати: браузер заблокував це
asset-save-error = Не вдалося зберегти: { $error }
asset-tag-removed = Тег { $tag } прибрано
asset-tag-restored = Повернуто { $tag }
asset-tags-added =
    Додано: { $count ->
        [one] { $count } тег
        [few] { $count } теги
        [many] { $count } тегів
       *[other] { $count } тегу
    }
asset-tags-accepted =
    Залишено { $count ->
        [one] { $count } припущений тег
        [few] { $count } припущені теги
        [many] { $count } припущених тегів
       *[other] { $count } припущеного тега
    }
asset-reviewed = Переглянуто
asset-reviewed-on-title = Переглянуто: правила та ШІ не чіпають його тегів. Клацніть, щоб зняти позначку (R)
asset-reviewed-off-title = Позначити як переглянутий, щоб правила та ШІ не чіпали його тегів (R)
asset-reviewed-on = Позначено переглянутим
asset-reviewed-off = Позначку «переглянуто» знято
# $name is an asset's name.
asset-reviewed-on-named = Ресурс { $name } позначено переглянутим
asset-reviewed-off-named = З ресурсу { $name } знято позначку «переглянуто»
asset-undo = Скасувати
asset-play-error = Не вдалося відтворити цей файл: { $error }
asset-copied-hash = Хеш скопійовано
asset-copied-toml = toml скопійовано
asset-copied-id = ID скопійовано
asset-copied-link = Посилання скопійовано
asset-origin-before = Раніше цей файл був
asset-origin-class-engine = , { $class }, збережений UE { $engine }.
asset-origin-class = , { $class }.
asset-origin-engine = , збережений UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor експортував його { $date }.
asset-origin-editor-notes = Unreal Editor експортував його { $date } ({ $notes }).
asset-origin-hoard = Hoard витягнув його з пакунка { $date }.
asset-origin-hoard-notes = Hoard витягнув його з пакунка { $date } ({ $notes }).
asset-origin-download = Завантажити оригінальний .{ $ext }
asset-origin-discarded = Оригінальний пакунок видалено після перетворення.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Історія
history-loading = Завантаження…
history-none = Тут ще нічого не змінювали.
history-load-failed = Не вдалося завантажити історію: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = ШІ
history-by-rule = Правило
history-by-someone = Хтось
history-by-app = { $who } через застосунок
history-by-assistant = { $who } через асистента
history-field-description = Опис
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Додано тег { $tag }
history-tag-removed = Знято тег { $tag }
history-tag-kept = Залишено запропонований тег { $tag }
history-tag-changed = Змінено тег { $tag }
history-reviewed = Позначено як перевірене
history-unreviewed = Позначено як неперевірене
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: встановлено { $to }
history-cleared = { $field }: очищено (було { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Скасувати
history-revert-title = Повернути те, що змінила ця зміна
history-reverted = Скасовано: { $change }
history-revert-failed = Не вдалося скасувати: { $error }
asset-details = Відомості про ресурс
asset-in-pack = у
asset-previous = Попередній ресурс
asset-next = Наступний ресурс
asset-favorite = В обране
asset-favorite-on-title = В обраному (F прибирає звідти)
asset-favorite-off-title = Додати в обране (F)
asset-share = Поділитися
asset-share-title = Поділитися посиланням на цей ресурс (той, хто його відкриє, спершу увійде в Hoard)
asset-layout-panel = Показати на бічній панелі
asset-layout-page = Показати на всю сторінку
asset-layout-page-title = Показати на всю сторінку: великий попередній перегляд поруч із відомостями
asset-close = Закрити відомості
asset-rename-title = Клацніть, щоб перейменувати
asset-name = Назва
asset-views-alt = Модель із чотирьох боків
asset-inspect-aria = Роздивитися { $name } на весь екран
asset-inspect-title = Роздивитися: масштаб, плитка, канали
asset-playback-position = Позиція відтворення
asset-playback-value = { $current } з { $duration }
asset-pause = Призупинити
asset-play = Відтворити
asset-waveform-drawing = Малювання хвильової форми…
asset-waveform-skipped = Для таких великих файлів хвильова форма не малюється
asset-play-animation = Відтворити як анімацію
asset-show-views = Показати аркуш із чотирма виглядами, який бачив тегувальник
asset-palette = Кольори на зображенні
asset-swatch-title = { $hex }, { $percent }% зображення. Знайти ресурси з цим кольором
asset-swatch-aria = Знайти ресурси з { $hex }, { $percent }% цього зображення
asset-more-like-this = Більше схожого
asset-desc-label = Ваш опис
asset-desc-placeholder = Що це, де ви б це використали…
asset-save = Зберегти
asset-cancel = Скасувати
asset-desc-family =
    { $count ->
        [one] Зберігається для { $count } варіанта
        [few] Зберігається для всіх { $count } варіантів
        [many] Зберігається для всіх { $count } варіантів
       *[other] Зберігається для всіх { $count } варіанта
    }
asset-desc-edit = Змінити опис
asset-desc-write = Написати опис
asset-desc-ai = Написано ШІ
asset-desc-file = З файлу
asset-desc-store-before = Зі
asset-desc-store-page = сторінки { $store }
asset-desc-store-page-unnamed = сторінки магазину
asset-desc-store-after = (увесь пакет)
asset-less = Менше
asset-more = Більше
asset-tags = Теги
asset-tag-add-label = Додати тег
asset-tag-add-placeholder = Додайте тег, напр. mood:tense
asset-apply-family =
    { $count ->
        [one] Застосувати зміни до { $count } варіанта
        [few] Застосувати зміни до всіх { $count } варіантів
        [many] Застосувати зміни до всіх { $count } варіантів
       *[other] Застосувати зміни до всіх { $count } варіанта
    }
asset-tags-accept-all =
    Залишити { $count ->
        [one] { $count } припущення
        [few] { $count } припущення
        [many] { $count } припущень
       *[other] { $count } припущення
    }
asset-tags-accept-all-title = Залишити як власні всі теги, які припустили правила та ШІ
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Відхилені:
asset-tag-restore-title = Повернути { $tag }
asset-tags-also-under = Також входить до:
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = У схожих є:
asset-tag-suggest-title = Додати { $tag }
asset-details-heading = Подробиці
asset-field-category = Категорія
asset-field-license = Ліцензія
asset-field-source = Джерело
asset-field-rating = Оцінка
# Shown in place of an empty value, to fill it in.
asset-field-add = Додати
asset-field-new = Додати поле
asset-field-name = Назва поля
asset-field-value = Значення
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = з пакета
asset-field-license-placeholder = Наприклад, CC0 або CC-BY 4.0
asset-field-remove-placeholder = Порожнє поле видаляє його
# $field is a field's name, like "License".
asset-field-edit-title = Змінити { $field }
asset-field-saved = Збережено { $field }
asset-field-cleared = Очищено { $field }
asset-rate =
    { $count ->
        [one] { $count } зірка
        [few] { $count } зірки
        [many] { $count } зірок
       *[other] { $count } зірки
    }
asset-rated =
    Оцінено на { $count ->
        [one] { $count } зірку
        [few] { $count } зірки
        [many] { $count } зірок
       *[other] { $count } зірки
    }
asset-rating-clear-title = Клацніть ще раз, щоб очистити оцінку
asset-rating-cleared = Оцінку очищено
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Клацніть ще раз: { $count } зірка
        [few] Клацніть ще раз: { $count } зірки
        [many] Клацніть ще раз: { $count } зірок
       *[other] Клацніть ще раз: { $count } зірки
    }
asset-collections = Колекції
asset-collection-remove = Вилучити з { $name }
asset-collection-add-another = Додати до іншої
asset-collection-add = Додати до колекції
asset-used-in = Використовується в
asset-used-in-title = Показати, що є в { $name }
asset-used-in-synced = синхронізовано { $when }
asset-maps = Карти
asset-show-more = Показати ще { $count }
asset-materials-heading =
    { $count ->
        [one] Матеріал
        [few] Матеріали
        [many] Матеріали
       *[other] Матеріали
    }
asset-material-maps-aria = { $name }: карти
asset-variants = Варіанти ({ $count })
asset-fit-animations = Анімації, що підходять ({ $count })
asset-fit-characters = Персонажі, яким підходить ({ $count })
asset-copies = Ідентичні копії
asset-copies-note = Однакові байти зберігаються один раз; цей файл є і в цих пакетах.
asset-near-copies = Майже копії
asset-near-copies-note = Те саме в іншому розмірі, частоті дискретизації або форматі. Найкращу копію позначено.
asset-near-copies-best = Те саме в іншому розмірі, частоті дискретизації або форматі. Це найкраща копія.
asset-near-best = найкраща
asset-unreal-converted = Перетворено з Unreal
asset-rule-converted = Перетворено правилом
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Правило перетворення створило цей { $format } { $date } з файлу
asset-rule-kept-until = Оригінал зберігається до { $date }.
asset-rule-rebuilt = FLAC зберігає все, що було у WAV, тож WAV можна будь-коли завантажити або повернути точно таким, яким він був.
asset-rule-put-back = Повернути оригінал
asset-rule-put-back-title = Повернути { $file }?
asset-rule-put-back-body = Перетворений файл видаляється, а оригінал займає його місце разом зі своїми тегами, колекціями та посиланнями. Відтепер правила перетворення його не чіпають.
asset-rule-put-back-done = { $file } повернуто
asset-rule-let-go = Оригінал видалено, коли минув строк зберігання оригіналів, заданий у «Налаштуваннях».
asset-unreal-package = З пакунка Unreal
asset-ue-needs-editor-before = Чекає на Unreal Editor: { $reason }. На комп’ютері з установленим Unreal виконайте
asset-ue-needs-editor-after = і файл повернеться як FBX.
asset-ue-editor-path = шлях до UnrealEditor-Cmd
asset-ue-unsupported = Залишено як пакунок: { $reason }.
asset-ue-failed = Перетворення не вдалося: { $reason }
asset-recovered = Оригінальний .{ $ext } зберігався всередині цього пакунка, і Hoard витягнув його.
asset-recovered-download = Завантажити .{ $ext }
asset-ue-meshes-before = Меші й анімації зберігаються у власному форматі Unreal. Перетворіть пакет (сторінка пакета, «Більше»), а потім виконайте
asset-ue-meshes-after = поруч з установленим Unreal Editor, щоб отримати FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } тег реєстру ресурсів
        [few] { $count } теги реєстру ресурсів
        [many] { $count } тегів реєстру ресурсів
       *[other] { $count } тегу реєстру ресурсів
    }
asset-file = Файл
asset-bytes = Байти
asset-source = Джерело
asset-path = Шлях
asset-path-open-folder = Що ще лежить у { $folder }
asset-download-maps =
    { $count ->
        [one] Завантажити { $count } карту
        [few] Завантажити всі { $count } карти
        [many] Завантажити всі { $count } карт
       *[other] Завантажити всі { $count } карти
    }
asset-download-linked =
    { $count ->
        [one] Завантажити з { $count } пов’язаним файлом
        [few] Завантажити з { $count } пов’язаними файлами
        [many] Завантажити з { $count } пов’язаними файлами
       *[other] Завантажити з { $count } пов’язаного файлу
    }
asset-download-only-title = Лише цей файл, без файлів, які він використовує
asset-download-only = Лише цей файл
asset-include-animations = Додати анімації в .zip
asset-include-character = Додати персонажа { $name }
asset-copy-toml = Копіювати запис hoard.toml
asset-copy-id = Копіювати ID
asset-teach = Навчити Hoard цього файлу
asset-teach-title = Описати файл цього магазину, щоб Hoard умів його читати («Налаштування → Файли магазинів»)
asset-archive = Перенести в архів
asset-archive-title = Зникне з пошуку, доки його не відновлять з архіву або не видалять остаточно

## ImageViewer.vue

image-channel-red = Червоний канал
image-channel-green = Зелений канал
image-channel-blue = Синій канал
image-channel-alpha = Альфа-канал
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Тло «шахівниця» (K перемикає)
image-bg-black-title = Чорне тло (K перемикає)
image-bg-gray-title = Сіре тло (K перемикає)
image-bg-white-title = Біле тло (K перемикає)
image-bg-checker = тло «шахівниця»
image-bg-black = чорне тло
image-bg-gray = сіре тло
image-bg-white = біле тло
image-inspect-aria = Роздивитися { $name }
image-preview-size = попередній перегляд { $width }×{ $height }, { $bytes }
image-preview = попередній перегляд
image-resolution = Роздільність
image-full-title = Завантажити повне зображення { $width }×{ $height } (F); те саме станеться, якщо збільшити масштаб понад попередній перегляд
image-full-loading = Завантаження повної роздільності…
image-full-failed = Повна роздільність не завантажилася: повторіть
image-full = Повна роздільність
image-zoom = Масштаб
image-zoom-out-title = Зменшити (−)
image-zoom-out = Зменшити
image-zoom-in-title = Збільшити (+)
image-zoom-in = Збільшити
image-fit-title = Умістити (0)
image-fit = Умістити
image-actual-title = Справжні пікселі (1)
image-view = Вигляд
image-tile-title = Розкласти плиткою 3×3, щоб перевірити шви (T)
image-tile = Плитка
image-channels = Канали
image-close = Закрити (Esc)
image-failed = Не вдалося завантажити це зображення.
image-loading = Завантаження…
image-key-gestures = Прокручування або зведення пальців — масштаб · перетягування — панорама · подвійне клацання — збільшити або вмістити
image-key-tile = плитка
image-key-background = тло
image-key-full = повна роздільність
image-key-channels = канали

## SpritePlayer.vue

sprite-frame-aria = { $name }, кадр { $frame } з { $count }
sprite-failed = Це не вдається відтворити.
sprite-loading = Завантаження кадрів…
sprite-previous = Попередній кадр
sprite-next = Наступний кадр
sprite-pause = Призупинити анімацію
sprite-play = Відтворити анімацію
sprite-animation = Анімація
sprite-all-frames = Усі кадри
sprite-grid-title = Як аркуш розрізано на кадри
sprite-columns = Стовпці
sprite-rows = Рядки
sprite-fps-aria = Кадрів за секунду
sprite-fps = кадр/с

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } спрайт
        [few] { $count } спрайти
        [many] { $count } спрайтів
       *[other] { $count } спрайта
    }
sheet-pages =
    { $count ->
        [one] { $count } сторінка
        [few] { $count } сторінки
        [many] { $count } сторінок
       *[other] { $count } сторінки
    }
sheet-animations =
    { $count ->
        [one] { $count } анімація
        [few] { $count } анімації
        [many] { $count } анімацій
       *[other] { $count } анімації
    }
# The box that narrows the list of sprites by name.
sheet-find = Знайти спрайт
# Under the list of sprites when more match than it draws.
sheet-showing = Показано { $shown } з { $total }
sheet-no-match = Жоден спрайт не має цього в назві.
sheet-failed = Цей аркуш не вдається прочитати.
sheet-loading = Завантаження аркуша…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Власний формат Hoard: сторінки як є, спрайти повернуто будь-як і обрізано, з анімаціями й тривалістю кожного кадру.
sheet-note-texturepacker = Спрайти повернуто за годинниковою стрілкою й обрізано, анімації — за назвами, як їх читають PixiJS і Phaser.
sheet-note-clockwise = Спрайти повернуто за годинниковою стрілкою й обрізано. Анімацій формат не зберігає: кадри впізнаються за нумерованими назвами.
sheet-note-libgdx = Спрайти повернуто проти годинникової стрілки й обрізано. Анімацій формат не зберігає: кадри впізнаються за нумерованими назвами.
sheet-note-godot-frames = Спрайти ніколи не повертаються, а обрізане зберігається як поля, з анімаціями й тривалістю кожного кадру.
sheet-note-godot-tileset = Плитки ніколи не повертаються й не обрізаються, лягають у сітку за розміром плитки, а зображення, що вже лежать у сітці, лишаються як є.
sheet-note-grid = Кожен спрайт стоїть рівно на всьому своєму полотні, у клітинках однакового розміру, — для будь-чого, що ріже аркуш на сітку.
sheet-note-frames = Кожен спрайт стоїть рівно на всьому своєму полотні, кожен — окремий PNG з його назвою.

## TextViewer.vue, TextPreview.vue і text.ts

text-lang-config = Конфігурація
text-lang-table = Таблиця
text-lang-code = Код
text-lang-script = Скрипт
text-lang-text = Текст
text-copy-blocked = Не вдалося скопіювати: браузер заблокував це
text-read-aria = Читати { $name }
text-lines =
    { $count ->
        [one] { $count } рядок
        [few] { $count } рядки
        [many] { $count } рядків
       *[other] { $count } рядка
    }
text-show-as = Показати як
text-tree-title = Дерево, що згортається (T)
text-tree = Дерево
text-pretty-title = З новими відступами (P)
text-pretty = Форматований
text-raw-title = Як написано (R)
text-raw = Сирий
text-document-title = Як документ (D)
text-document = Документ
text-source-title = Markdown як написано (R)
text-source = Джерело
text-page-title = Як її показує браузер з її теки, окремо від Hoard (P)
text-page = Сторінка
text-page-source-title = Сторінка, як її написано (R)
text-page-aria = { $name } як сторінка
text-page-error = Не вдалося відкрити цю сторінку: { $error }
text-fold = Згортання
text-expand-all-title = Розгорнути все
text-expand-all = Розгорнути все
text-collapse-all-title = Згорнути все
text-collapse-all = Згорнути все
text-group-text = Текст
text-wrap-title = Переносити довгі рядки (W)
text-wrap = Перенесення
text-copy-title = Копіювати текст (C)
text-copied = Скопійовано
text-copy = Копіювати
text-download-title = Завантажити файл
text-download = Завантажити
text-close = Закрити (Esc)
text-truncated = Показано перші { $shown } з { $total }.
text-truncated-json = Показано перші { $shown } з { $total } (для дерева потрібен увесь файл).
text-show-all = Показати все
text-json-error = Цей JSON не розбирається: { $error }
text-contents-aria = Вміст { $name }
text-loading = Завантаження…
text-load-error = Не вдалося завантажити цей файл: { $error }
text-binary = Це двійковий файл, а не текст, тож тут нічого читати. Завантажте його, щоб відкрити у відповідній програмі.
text-key-tree = дерево
text-key-pretty = форматований
text-key-raw = сирий
text-key-document = документ
text-key-source = джерело
text-key-page = сторінка
text-key-wrap = перенесення
text-key-copy = копіювати
text-key-find = знайти
text-key-close = закрити
text-preview-aria = Читати { $name } на весь екран
text-preview-read = Читати

## JsonTree.vue і JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } елемент
        [few] { $count } елементи
        [many] { $count } елементів
       *[other] { $count } елемента
    }
json-keys =
    { $count ->
        [one] { $count } ключ
        [few] { $count } ключі
        [many] { $count } ключів
       *[other] { $count } ключа
    }
json-fold = Згорнути
json-unfold = Розгорнути
json-unfold-summary = Розгорнути: { $summary }
json-show-more = Показати ще { $count }
json-left = (усього залишилося { $count })
json-picker-more = … ще { $count }

## 3D-переглядачі моделей (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = 3D-переглядач не завантажився ({ $error }); може допомогти перезавантаження сторінки
model-loading-viewer = Завантаження 3D-переглядача…
model-loading = Завантаження моделі…
model-show-failed = Не вдалося показати цю модель: { $error }
model-preview-failed = Не вдалося показати попередній перегляд цієї моделі: { $error }
model-error-converting = сервер ще перетворює цю модель
model-error-converting-desktop = Hoard ще перетворює цю модель
model-error-no-loader = Браузер не має завантажувача для .{ $ext }
model-dialog = Перегляд { $name } у 3D
model-title-stats =
    { $triangles ->
        [one] { $triangles } трикутник
        [few] { $triangles } трикутники
        [many] { $triangles } трикутників
       *[other] { $triangles } трикутника
    } · { $meshes ->
        [one] { $meshes } меш
        [few] { $meshes } меші
        [many] { $meshes } мешів
       *[other] { $meshes } меша
    } · { $materials ->
        [one] { $materials } матеріал
        [few] { $materials } матеріали
        [many] { $materials } матеріалів
       *[other] { $materials } матеріалу
    }
model-triangles =
    { $count ->
        [one] { $count } трикутник
        [few] { $count } трикутники
        [many] { $count } трикутників
       *[other] { $count } трикутника
    }
model-group-camera = Камера
model-group-shading = Затінення
model-group-background = Тло
model-group-animation = Анімація
model-group-view = Вигляд
model-group-show = Показ
model-frame-title = Вмістити модель у кадр (F)
model-frame = У кадр
model-reset-title = Повернутися до початкового вигляду (R)
model-reset = Скинути
model-turn-title = Повільно обертати (A)
model-turn = Обертати
model-textures-title = Текстури (T)
model-textures = Текстури
model-full-res-title = Текстури в повній роздільності (H): кожен файл текстури як є, хоч би яким великим він був; якщо вимкнено — полегшені перегляди до 2048 пікс.
model-full-res = Повна розд.
model-light-title = Освітлення оточення (E)
model-light = Світло
model-grid-title = Сітка підлоги (G)
model-grid = Сітка
model-bg-checker = Тло «шахівниця»
model-bg-dark = Темне тло
model-bg-gray = Сіре тло
model-bg-light = Світле тло
model-bg-env = Тло оточення
model-bg-title = { $background } (K перемикає)
model-pause-title = Призупинити (пробіл)
model-play-title = Відтворити (пробіл)
model-pause = Призупинити
model-play = Відтворити
model-animation-clip = Кліп анімації
model-animation = Анімація
model-plays-on = Відтворювати на
model-plays-everywhere = Використовувати всюди
model-plays-everywhere-title = Відтворювати всі анімації з цим скелетом на персонажі { $name }, зокрема в мініатюрах
model-plays-everywhere-done = Анімації з цим скелетом тепер відтворюються на персонажі { $name }. Їхні мініатюри малюються заново.
model-plays-everywhere-failed = Не вдалося змінити персонажа, на якому вони відтворюються: { $error }
model-rest-pose = Поза спокою
model-close-title = Закрити (Esc)
model-full-screen = На весь екран
model-open-full-screen = Відкрити 3D-переглядач на весь екран
model-menu-fold = Згорнути меню
model-menu-show = Показати меню
model-view-prefix = Вигляд:{ " " }
model-menu-render = Рендер
model-menu-maps = Карти
model-menu-inspect = Аналіз
model-mode-final = Фінальний рендер
model-mode-raw = Сирий рендер
model-mode-color = Базовий колір
model-mode-roughness = Шорсткість
model-mode-metalness = Металевість
model-mode-normal = Нормалі
model-mode-ao = Затінення
model-mode-matcap = Matcap
model-mode-uv = UV-шахівниця
model-mode-geometry = Геометрії
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Каркас (W) поверх будь-якого вигляду
model-wireframe = Каркас
model-info-title = Відомості про сцену (I)
model-info = Відомості про сцену
model-info-close = Закрити відомості про сцену
model-info-close-title = Закрити (I)
model-info-geometries = Геометрії
model-info-materials = Матеріали
model-info-textures = Текстури
model-info-triangles = Трикутники
model-info-vertices = Вершини
model-info-units = Одиниці
model-info-in-meters = Розміри в метрах, за одиницями, зазначеними у файлі
model-units-m = Метри
model-units-cm = Сантиметри
model-units-mm = Міліметри
model-units-in = Дюйми
model-units-ft = Фути
model-units-other = 1 одиниця = { NUMBER($n, maximumSignificantDigits: 3) } м
model-units-unknown = Не зазначено
model-info-no-units = Цей формат не вказує своїх одиниць виміру
model-info-height = Висота
model-info-length = Довжина
model-info-width = Ширина
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } одиниця
        [few] { NUMBER($n, maximumSignificantDigits: 3) } одиниці
        [many] { NUMBER($n, maximumSignificantDigits: 3) } одиниць
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } одиниці
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } м
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } см
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } мм
model-missing-one = Текстуру не знайдено: { $name }
model-missing-some =
    { $count ->
        [one] Не знайдено { $count } текстуру: { $names }
        [few] Не знайдено { $count } текстури: { $names }
        [many] Не знайдено { $count } текстур: { $names }
       *[other] Не знайдено { $count } текстури: { $names }
    }
model-missing-more =
    { $count ->
        [one] Не знайдено { $count } текстуру: { $names }…
        [few] Не знайдено { $count } текстури: { $names }…
        [many] Не знайдено { $count } текстур: { $names }…
       *[other] Не знайдено { $count } текстури: { $names }…
    }
model-missing-count =
    { $count ->
        [one] Не знайдено { $count } текстуру
        [few] Не знайдено { $count } текстури
        [many] Не знайдено { $count } текстур
       *[other] Не знайдено { $count } текстури
    }
model-guessed = Текстури підібрано за назвою; файл не називає жодної.
model-group-facing = Орієнтація
model-facing-turn = Повернути на чверть
model-facing-turn-title = Повернути модель на чверть оберту проти годинникової стрілки, якщо дивитися згори
model-facing-tip = Нахилити на чверть
model-facing-tip-title = Нахилити верх моделі на чверть оберту вперед, щоб поставити модель, яка лежить
model-facing-save = Застосувати до пакета
model-facing-save-title = Повернути так усі моделі в пакеті { $pack }. Їхні мініатюри буде намальовано знову; завантаження залишаться такими, якими їх створено.
model-facing-undo = Скасувати
model-facing-undo-title = Повернутися до того, як повернуто моделі пакета
model-facing-saved = Усі моделі в пакеті { $pack } тепер повернуто так. Їхні мініатюри малюються знову.
model-facing-failed = Не вдалося повернути моделі пакета: { $error }
model-keys-mouse = Перетягування — обертання · правою кнопкою — панорама · прокручування — масштаб · подвійне клацання — у кадр
model-key-frame = у кадр
model-key-reset = скинути
model-key-view = вигляд
model-key-wireframe = каркас
model-key-info = відомості про сцену
model-key-textures = текстури
model-key-full-res = текстури в повній роздільності
model-key-light = світло
model-key-grid = сітка
model-key-background = тло
model-key-play = відтворення
explorer-dialog = Оглядач мешів: літайте серед моделей із цих результатів
explorer-title = Оглядач мешів
explorer-group-view = Вигляд
explorer-true-size = Справжній розмір
explorer-true-size-title = Показувати кожну модель у справжньому розмірі в метрах, до 12 м, а не вписувати її в клітинку (T)
explorer-animate = Анімація
explorer-animate-title = Відтворювати перший кліп кожної анімованої моделі (Пробіл)
explorer-orbit = Орбіта
explorer-orbit-title = Перетягування обертає навколо вибраної моделі, а не роззирається (O)
explorer-open = Відкрити
explorer-open-title = Відкрити відомості про цю модель (Enter)
explorer-size = { $x } × { $y } × { $z } м
explorer-empty = Жоден із цих результатів не можна показати в 3D.
explorer-hud-loaded =
    { $total ->
        [one] Завантажено { $loaded } з { $total } моделі
        [few] Завантажено { $loaded } з { $total } моделей
        [many] Завантажено { $loaded } з { $total } моделей
       *[other] Завантажено { $loaded } з { $total } моделі
    }
explorer-hud-loading = Завантажується: { $count }
explorer-hud-memory = ~{ $mb } МБ
explorer-hud-speed = Швидкість { $speed } м/с
explorer-hud-fps = { $fps } кадр/с
explorer-keys-mouse = Клацання вибирає · перетягування — роззирнутися · перетягування середньою кнопкою зсуває · Alt і перетягування — орбіта · прокручування переміщує, із затиснутою правою кнопкою змінює швидкість · подвійне клацання — показати повністю
explorer-key-fly = політ
explorer-key-down-up = вниз, вгору
explorer-key-faster = швидше
explorer-key-open = відкрити
explorer-key-true-size = справжній розмір
explorer-key-orbit = орбіта
explorer-key-animate = анімація
explorer-key-close = закрити

## Матеріали (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Колір
material-role-normal = Нормалі
material-role-normal-dx = Нормалі (DirectX)
material-role-roughness = Шорсткість
material-role-gloss = Глянець
material-role-metalness = Металевість
material-role-ao = Фонове затінення
material-role-orm = Затінення, шорсткість, металевість
material-role-mra = Металевість, шорсткість, затінення
material-role-rma = Шорсткість, металевість, затінення
material-role-height = Висоти
material-role-bump = Рельєф
material-role-cavity = Порожнини
material-role-specular = Дзеркальність
material-role-opacity = Непрозорість
material-role-emissive = Світіння
material-role-translucency = Просвічування
material-role-base-color-name = кольору
material-role-normal-name = нормалей
material-role-normal-dx-name = нормалей (DirectX)
material-role-roughness-name = шорсткості
material-role-gloss-name = глянцю
material-role-metalness-name = металевості
material-role-ao-name = фонового затінення
material-role-orm-name = затінення, шорсткості, металевості
material-role-mra-name = металевості, шорсткості, затінення
material-role-rma-name = шорсткості, металевості, затінення
material-role-height-name = висот
material-role-bump-name = рельєфу
material-role-cavity-name = порожнин
material-role-specular-name = дзеркальності
material-role-opacity-name = непрозорості
material-role-emissive-name = світіння
material-role-translucency-name = просвічування
material-maps =
    { $count ->
        [one] { $count } карта
        [few] { $count } карти
        [many] { $count } карт
       *[other] { $count } карти
    }
material-dialog = Перегляд матеріалу { $name }
material-up-to = до { $size }
material-repeat-size = { $size } на повтор
material-shape-sphere = Сфера
material-shape-cube = Куб
material-shape-plane = Площина
material-shape-title = { $shape } ({ $key })
material-group-shape = Форма
material-group-tiling = Тайлінг
material-real-title = Реальний розмір: один повтор — { $size }, сфера діаметром 2 м (T перемикає)
material-repeat-title = Повторити { $n }× (T перемикає)
material-real = Реальний
material-repeat = { $n }×
material-height-title = Наскільки карта висот зміщує поверхню
material-height = Висота
material-height-strength = Сила висоти
material-loading = Завантаження матеріалу…
material-show-failed = Не вдалося показати цей матеріал: { $error }
material-preview-failed = Не вдалося показати попередній перегляд цього матеріалу: { $error }
material-group-maps = Карти
material-map-show = Показати карту { $map }
material-map-hide = Сховати карту { $map }
material-map-unused = Не використовується в попередньому перегляді
material-open-full-screen = Відкрити переглядач матеріалів на весь екран
material-keys-mouse = Перетягування — обертання · правою кнопкою — панорама · прокручування — масштаб · клацання на карті вимикає її
material-key-shape = форма
material-key-tiling = тайлінг
material-key-turn = обертання

## theme.ts

theme-slate = Грифель
theme-paper = Папір
theme-linen = Льон
theme-sage = Шавлія
theme-lilac = Бузок
theme-blush = Рум’янець
theme-graphite = Графіт
theme-obsidian = Обсидіан
theme-ember = Жар
theme-moss = Мох
theme-aubergine = Баклажан
theme-oxblood = Бордо
theme-sky = Небо
theme-mint = М’ята
theme-apricot = Абрикос
theme-lemon = Лимон
theme-midnight = Північ
theme-lagoon = Лагуна
theme-neon = Неон
theme-light-slate-blurb = Прохолодний синьо-сірий
theme-light-paper-blurb = Простий нейтральний сірий
theme-light-linen-blurb = Теплий кремовий, бурштин
theme-light-sage-blurb = М’який зелений
theme-light-lilac-blurb = Блідо-фіолетовий
theme-light-blush-blurb = Припорошена троянда
theme-light-sky-blurb = Чистий небесно-блакитний
theme-light-mint-blurb = Прохолодна аквамаринова м’ята
theme-light-apricot-blurb = Ніжний персик, помаранчевий
theme-light-lemon-blurb = Блідо-жовтий, вохра
theme-dark-graphite-blurb = Нейтральний сірий, без відтінку
theme-dark-slate-blurb = Синьо-сірий
theme-dark-obsidian-blurb = Майже чорний, для OLED
theme-dark-ember-blurb = Теплий вугільний, бурштин
theme-dark-moss-blurb = Лісовий зелено-чорний
theme-dark-aubergine-blurb = Сливово-чорний, бузок
theme-dark-oxblood-blurb = Винно-чорний, корал
theme-dark-midnight-blurb = Глибокий темно-синій, блакитний
theme-dark-lagoon-blurb = Глибокий синьо-зелений, аквамарин
theme-dark-neon-blurb = Майже чорний, яскраво-рожевий

## texnames.ts

texnames-needs-map = Шаблону потрібен { "{" }map{ "}" }, інакше всі карти матеріалу отримають однакову назву
texnames-placeholders = Шаблон може використовувати { "{" }name{ "}" }, { "{" }map{ "}" } і { "{" }size{ "}" }
texnames-bad-characters = Назви не можуть містити < > : " / \ | ? або *
texnames-no-suffix = Задайте суфікс для карт { $role }
texnames-bad-suffix = Суфікс для карт { $role } не може містити < > : " / \ | ? або *
texnames-slot-no-suffix = Задайте суфікс для «{ $slot }»
texnames-slot-bad-suffix = Суфікс для «{ $slot }» не може містити < > : " / \ | ? або *
texnames-ending-missing = Кожне власне закінчення має містити закінчення, яке слід шукати
texnames-ending-bad = Власні закінчення не можуть містити < > : " / \ | ? або *
texnames-ending-twice = { $ending } двічі є серед ваших закінчень
texnames-slot-metallic-roughness = Металевість і шорсткість
texnames-slot-specular-color = Колір дзеркальності
texnames-slot-specular-glossiness = Дзеркальність і глянець
texnames-slot-clearcoat = Лак
texnames-slot-clearcoat-roughness = Шорсткість лаку
texnames-slot-clearcoat-normal = Нормалі лаку
texnames-slot-sheen-color = Колір відливу
texnames-slot-sheen-roughness = Шорсткість відливу
texnames-slot-transmission = Пропускання
texnames-slot-thickness = Товщина
texnames-slot-diffuse-transmission = Розсіяне пропускання
texnames-slot-diffuse-transmission-color = Колір розсіяного пропускання
texnames-slot-iridescence = Райдужність
texnames-slot-iridescence-thickness = Товщина райдужності
texnames-slot-anisotropy = Анізотропія

## storeFormats.ts

storefiles-field-title = Назва
storefiles-field-title-hint = Точна назва оголошення. Вона дає назву пакету, і без неї файл пропускається.
storefiles-field-seller = Продавець
storefiles-field-seller-hint = Хто створив або продає.
storefiles-field-license = Ліцензія
storefiles-field-license-hint = Як записано у файлі, наприклад CC-BY-4.0.
storefiles-field-url = Сторінка магазину
storefiles-field-url-hint = Адреса оголошення, що починається з http:// або https://.
storefiles-field-words = Теги
storefiles-field-words-hint = Слова для тегів. Значення через кому розділяються.
storefiles-field-type = Тип
storefiles-field-type-hint = Що це, наприклад «3D-модель» чи «поверхня». Теж стає тегом.
storefiles-field-colors = Кольори
storefiles-field-colors-hint = Його кольори, як теги color:.
storefiles-field-id = ID у магазині
storefiles-field-id-hint = Ідентифікатор у магазині: літери, цифри, _ або -.
storefiles-field-origin = Місце створення
storefiles-field-origin-hint = Де створено або відскановано, для опису.
storefiles-field-size = Розмір
storefiles-field-size-hint = Наскільки великий, словами, для опису.
storefiles-field-title-name = назва
storefiles-field-seller-name = продавець
storefiles-field-license-name = ліцензія
storefiles-field-url-name = сторінка магазину
storefiles-field-words-name = теги
storefiles-field-type-name = тип
storefiles-field-colors-name = кольори
storefiles-field-id-name = ID у магазині
storefiles-field-origin-name = місце створення
storefiles-field-size-name = розмір

## StoreFormatsPanel.vue

storefiles-load-failed = Не вдалося завантажити формати файлів магазинів: { $error }
storefiles-remove-title = Вилучити { $name }?
storefiles-remove-body = { $file } буде видалено з теки даних. Пакети, уже прочитані з ним, збережуть прочитане.
storefiles-remove = Вилучити
storefiles-removed = { $name } вилучено.
storefiles-remove-failed = Не вдалося вилучити { $name }: { $error }
storefiles-unread = Файли, які Hoard не читає
storefiles-loading = Завантаження…
storefiles-unread-none = Немає: жоден файл у бібліотеці не схожий на файл непрочитаного магазину.
storefiles-unread-what = Файли, що є в кількох пакетах або названі як файли магазину, але жоден формат їх не читає.
storefiles-unread-packs =
    { $count ->
        [one] { $count } пакет
        [few] { $count } пакети
        [many] { $count } пакетів
       *[other] { $count } пакета
    }
storefiles-unread-example = напр. { $pack }
storefiles-teach = Навчити Hoard цього файлу
storefiles-formats = Формати
storefiles-formats-what = Ваші формати пробуються першими й замінюють вбудований формат із такою самою назвою.
storefiles-dir-before = Вони зберігаються в
storefiles-dir-after = .
storefiles-yours = Ваш
storefiles-built-in = Вбудований
storefiles-edit = Змінити
storefiles-start-from = Почати з цього

## TagsPanel.vue

tags-load-failed = Не вдалося завантажити теги: { $error }
tags-rename-title = Перейменувати { $tag }?
tags-merge-title =
    { $count ->
        [one] Об’єднати { $count } тег?
        [few] Об’єднати { $count } теги?
        [many] Об’єднати { $count } тегів?
       *[other] Об’єднати { $count } тегу?
    }
tags-rename-body =
    { $count ->
        [one] Його { $count } ресурс отримає натомість { $into }
        [few] Його { $count } ресурси отримають натомість { $into }
        [many] Його { $count } ресурсів отримають натомість { $into }
       *[other] Його { $count } ресурсу отримають натомість { $into }
    }, а тегувальники відтепер використовуватимуть нову назву.
tags-merge-body =
    { $tags } стануть { $into } { $count ->
        [one] щонайбільше в { $count } ресурсі
        [few] щонайбільше в { $count } ресурсах
        [many] щонайбільше в { $count } ресурсах
       *[other] щонайбільше в { $count } ресурсу
    }. Тегувальники відтепер використовуватимуть { $into }.
tags-rename = Перейменувати
tags-merge = Об’єднати
tags-merged =
    { $count ->
        [one] { $count } ресурс тепер має тег { $into }
        [few] { $count } ресурси тепер мають тег { $into }
        [many] { $count } ресурсів тепер мають тег { $into }
       *[other] { $count } ресурсу тепер мають тег { $into }
    }
tags-merge-failed = Не вдалося об’єднати: { $error }
tags-forget-failed = Не вдалося це змінити: { $error }
tags-find = Знайти теги
tags-find-placeholder = Шукайте теги, напр. rock або set:
tags-sort = Порядок
tags-sort-count = Найуживаніші
tags-sort-name = А–Я
tags-count =
    { $count ->
        [one] { $count } тег
        [few] { $count } теги
        [many] { $count } тегів
       *[other] { $count } тегу
    }
tags-count-more = { $count }+ тегів
tags-what = Виберіть теги, що означають те саме, і об’єднайте їх або виберіть один, щоб перейменувати.
tags-new-name = Нова назва
tags-merge-into = Об’єднати в
tags-saved-as-before = Збережено як
tags-saved-as-after = { "" }
tags-cancel = Скасувати
tags-loading = Завантаження…
tags-none = Тегів немає.
tags-none-containing = Немає тегів, що містять «{ $text }».
tags-browse-title = Переглянути ресурси з тегом { $tag }
tags-browse = Переглянути
tags-merged-heading = Об’єднані теги
tags-merged-what = Коли тегувальники пропонують одну з цих старих назв, ресурс отримує тег, у який її об’єднано.
tags-allow-old = Знову дозволити стару назву
tags-lookalike-heading = Схожі теги
tags-lookalike-what = Множина, ті самі слова разом, одна літера не та або слово, яке є й тегом із простором імен (red і color:red). Під час об’єднання лишається частіше вживана назва або та, що з простором імен, і надалі теги ставляться з нею.
tags-lookalike-merge-title = Об’єднати { $other } з { $keep }
tags-lookalike-apart = Різні
tags-lookalike-apart-title = { $a } і { $b } означають різне: більше не пропонувати їх об’єднати
tag-page-alias-remove = Забути стару назву { $alias }: її знову можуть ставити замість { $tag }
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурси
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Стара назва: { $names }
        [few] Старі назви: { $names }
        [many] Старі назви: { $names }
       *[other] Старі назви: { $names }
    }
tag-page-aliases-label =
    { $count ->
        [one] Стара назва:
        [few] Старі назви:
        [many] Старі назви:
       *[other] Старі назви:
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } об’єднано з { $into }.
tag-page-show = Показати { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Напишіть, що означає цей тег…
tag-page-describe-label = Що означає цей тег
tag-page-describe-placeholder = Для чого він, що до нього належить. Enter зберігає.
tag-page-describe-edit = Натисніть, щоб змінити
tag-page-description-saved = Значення тега збережено
# The tag's picture: one of its assets stands for it.
tag-page-picture = Вибрати зображення для цього тега
tag-page-picture-help = Виберіть один з його ресурсів, який його представлятиме
tag-page-picture-nothing = Жоден із показаних ресурсів не має зображення.
tag-page-picture-none = Без зображення
tag-page-picture-saved = Зображення встановлено
tag-page-picture-removed = Зображення прибрано
tag-page-rename = Перейменувати…
tag-page-rename-title = Дайте йому нову назву або назву іншого тега, щоб об’єднати з ним
tag-page-rename-label = Нова назва
tag-page-save-failed = Не вдалося зберегти тег: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Під
tag-page-parent-add = Помістити під…
tag-page-parent-add-title = Помістіть цей тег під ширший: при пошуку ширшого знайдуться й ці
tag-page-parent-placeholder = Ширший тег, наприклад транспорт
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Прибрати { $tag } з-під { $parent }
tag-page-parent-saved = { $tag } тепер під { $parent }
tag-page-parent-removed = { $tag } більше не під { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Вужчі:
tag-page-narrower-title = При пошуку { $tag } знайдуться й ці
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Ширші й вужчі
tags-tree-what = Помістіть тег під ширший тут або на сторінці тегу, і пошук за ширшим тегом знайде все, що під ним.
tags-tree-none = Поки нічого ні під чим немає. Спробуйте помістити «автомобіль» під «транспорт».
tags-tree-tag-label = Тег, який помістити під ширший
tags-tree-tag-placeholder = Тег, наприклад автомобіль
tags-tree-goes-under = входить до
tags-tree-parent-label = Ширший тег
tags-tree-put = Помістити

## LibraryStats.vue

stats-source-human = Додано людьми
stats-source-ai = Запропоновано ШІ
stats-source-rule = Вбудовані правила
stats-source-meta = З файлів
stats-assets = Ресурси
stats-packs = Пакети
stats-on-disk = На диску
stats-duplicates-title =
    { $count ->
        [one] { $count } файл є побайтовою копією іншого й зберігається один раз
        [few] { $count } файли є побайтовими копіями інших і зберігаються один раз
        [many] { $count } файлів є побайтовими копіями інших і зберігаються один раз
       *[other] { $count } файлу є побайтовими копіями інших і зберігаються один раз
    }
stats-saved-dedup = Заощаджено дедуплікацією
stats-distinct-tags = Різні теги
stats-archive = В архіві
stats-load-failed = Не вдалося завантажити відомості: { $error }
stats-counting = Підрахунок…
stats-as-of = Станом на { $time }
stats-updating = Оновлюється…
stats-by-kind = Місце за типами
stats-files-size =
    { $count ->
        [one] { $name }: { $count } файл, { $size }
        [few] { $name }: { $count } файли, { $size }
        [many] { $name }: { $count } файлів, { $size }
       *[other] { $name }: { $count } файлу, { $size }
    }
stats-months = Додано за останній рік
stats-month =
    { $count ->
        [one] { $month }: { $count } ресурс, { $size }
        [few] { $month }: { $count } ресурси, { $size }
        [many] { $month }: { $count } ресурсів, { $size }
       *[other] { $month }: { $count } ресурсу, { $size }
    }
stats-tagging = Тегування
stats-tagged = З тегами
stats-untagged = Без тегів: { $count }
stats-described = З описом
stats-described-assets =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурси
        [many] { $count } ресурсів
       *[other] { $count } ресурсу
    }
stats-human-edited = Змінено вручну
stats-human-edited-unit =
    { $count ->
        [one] ресурс
        [few] ресурси
        [many] ресурсів
       *[other] ресурсу
    }
stats-source-different =
    { $count ->
        [one] { $count } різний
        [few] { $count } різні
        [many] { $count } різних
       *[other] { $count } різного
    }
stats-most-used = Найуживаніші
stats-show-tag = Показати ресурси з тегом { $tag }
stats-file-types = Типи файлів
stats-biggest-packs = Найбільші пакети
stats-browse-pack = Переглянути { $name }
stats-largest-files = Найбільші файли
stats-open-file = Відкрити { $name }
stats-empty = У бібліотеці ще нічого немає. Додайте пакет, і тут з’являться його числа.

## HoardPanel.vue

hoard-title = Ваш скарб
hoard-what = Кожен ресурс у бібліотеці — це частина скарбу, і його вид визначає, яка саме. Зберіть нове, щоб додати його до своєї купи.
hoard-sign-in = Увійдіть, щоб збирати власний скарб.
hoard-new = Нова здобич для збору: { $count }
hoard-none-new = Збирати поки нічого.
hoard-collect = Зібрати
hoard-unavailable = Цей браузер не може намалювати скарб: { $error }
hoard-collect-failed = Не вдалося зберегти зібране: { $error }
hoard-arcade = Екран ігрового автомата
hoard-full-screen = На весь екран
hoard-leave-full-screen = Вийти з повноекранного режиму
hoard-label = Дракон стереже свій скарб. Скарбів: { $count }
hoard-gold = Золото
hoard-gems = Самоцвіти
hoard-books = Книги
hoard-weapons = Зброя
hoard-bones = Кістки
hoard-tablets = Рунічні таблички
hoard-paintings = Картини
hoard-potions = Зілля

## UpdatesPanel.vue

updates-saved = Hoard звернеться за нею, коли буде готове оновлення
updates-saved-none = Адресу для сповіщень не задано
updates-save-failed = Не вдалося зберегти
updates-call-answered = Відповідь: { $result }
updates-call-refused = Запит не прийнято: { $result }
updates-call-failed = Не вдалося звернутися
updates-called = Звернення { $when } щодо { $version }: { $result }
updates-called-failed = Звернення { $when } щодо { $version }: { $result }. Hoard спробує знову за годину.
updates-latest = Hoard { $version } — найновіший випуск.
updates-out = Вийшов Hoard { $version }.
updates-out-idle = Вийшов Hoard { $version }; звернення відбудеться протягом кількох хвилин, поки нічого не відбувається.
updates-out-busy = Вийшов Hoard { $version }; чекаємо, доки нічого не відбуватиметься ({ $work }).
updates-busy-more = ще { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } завдання в черзі
        [few] { $count } завдання в черзі
        [many] { $count } завдань у черзі
       *[other] { $count } завдання в черзі
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } вивантаження
        [few] { $count } вивантаження
        [many] { $count } вивантажень
       *[other] { $count } вивантаження
    }
updates-list-separator = ,{ " " }
updates-what = Коли виходить новий випуск, Hoard може звернутися за адресою, щойно нічого не відбуватиметься (немає вивантажень, завдань, перетворень чи обслуговування), щоб те, що оновлює його контейнер, не перезапустило його посеред вивантаження. Він надсилає POST із випуском у тілі JSON. Для цього потрібна перевірка випусків, описана вище.
updates-url = Адреса для сповіщень
updates-url-placeholder = Немає (напр. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Збережено (введіть, щоб замінити)
updates-auth-placeholder = Немає (напр. Bearer my-token)
updates-remove = Вилучити
updates-save = Зберегти
updates-call-now-title = Запуститься те, що підключено до цієї адреси
updates-call-now = Звернутися зараз
updates-what-calls = За якою адресою можна звертатися?
updates-watchtower = HTTP API Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , з{ " " }
updates-watchtower-token = { " " }і токеном із{ " " }
updates-watchtower-its = { " " }(з увімкненою кінцевою точкою{ " " }
updates-watchtower-endpoint = ).
updates-portainer = Вебхук Portainer
updates-portainer-rest = { " " }для стека або контейнера Hoard з увімкненим повторним завантаженням образу (re-pull image). Токен міститься в URL.
updates-script = Скрипт на хості
updates-script-listener = { " " }за обробником вебхуків (наприклад,{ " " }
updates-script-runs = ), який виконує{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }або будь-що інше, що приймає вебхуки.
updates-poll-before = Або обійдіться без адреси: скрипт із токеном доступу може опитувати{ " " }
updates-poll-until = , доки{ " " }
updates-poll-and = { " " }не буде задано, а{ " " }
updates-poll-after = { " " }не дорівнюватиме true, а потім завантажити оновлення й перезапустити Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Не вдалося завантажити обслуговування
maintenance-reanalyze-title = Проаналізувати всі пакети знову?
maintenance-reanalyze-body = Кожен файл буде прочитано знову, що на великій бібліотеці може тривати довго; пакети показуються як «в обробці», доки не настане їхня черга. Ваші власні назви, теги й нотатки залишаться.
maintenance-reanalyze-confirm = Проаналізувати знову
maintenance-gc-title = Видалити невикористані файли?
maintenance-gc-body = Збережені файли, на які ніщо в бібліотеці не посилається, буде остаточно видалено. Пакети в архіві збережуть свої.
maintenance-gc-confirm = Видалити їх
maintenance-start-failed = Не вдалося запустити «{ $task }»
maintenance-just-now = щойно
maintenance-minutes-ago = { $count } хв тому
maintenance-hours-ago = { $count } год тому
maintenance-took-seconds = { $count } с
maintenance-took-minutes = { $count } хв
maintenance-took-hours = { $hours } год
maintenance-loading = Завантаження…
maintenance-running = Виконується…
maintenance-starting = Запуск…
maintenance-run = Запустити
maintenance-stop = Зупинити
maintenance-stopping = Зупиняється…
maintenance-stop-failed = Не вдалося зупинити «{ $task }»
maintenance-started = { $detail } · запущено { $when }
maintenance-starting-detail = Запуск
maintenance-finished = Завершено:{ " " }
maintenance-failed = Помилка:{ " " }
maintenance-never-run = Ще не запускалося
maintenance-history = Попередні запуски
maintenance-col-task = Завдання
maintenance-col-finished = Завершено
maintenance-col-took = Тривалість
maintenance-col-result = Результат
maintenance-script-before = Резервне копіювання й перевірку на пошкодження також можна запускати зі скрипту:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }і{ " " }
maintenance-script-after = . Ці запуски теж з’являються тут.

## SettingsPage.vue

settings-title = Налаштування
settings-loading = Завантаження…
settings-find = Знайти налаштування
settings-find-none = Немає налаштувань, що відповідають «{ $query }».
settings-load-failed = Не вдалося завантажити налаштування: { $error }
settings-page-appearance = Вигляд
settings-page-appearance-blurb = Мова, теми, розмір тексту й відступи, а також що показують плитки й верхня панель. Зберігається лише в цьому браузері.
settings-page-dragon = Дракон
settings-page-dragon-blurb = Скарбниця вашого дракона: кожен ресурс у бібліотеці — частина його скарбу.
settings-page-library = Бібліотека
settings-page-library-blurb = Що є в Hoard: який він великий, із чого складається, як ріс і наскільки добре позначений тегами. Вміст архіву не враховується.
settings-page-copies = Майже копії
settings-page-copies-blurb = Файли, що є тим самим зображенням, звуком або моделлю в іншому розмірі, частоті дискретизації або форматі; спершу ті, що займають найбільше місця. Найкращу в кожному наборі позначено; нічого не видаляється.
copies-loading = Пошук майже копій…
copies-refreshing = Бібліотека змінилася відтоді, як їх знайшли. Повторний пошук…
copies-none = Майже копій немає: кожне зображення, звук і модель тут лише один раз.
copies-load-failed = Не вдалося показати майже копії: { $error }
copies-sum = Наборів майже копій: { $count }. Копії, які не найкращі, займають { $size }.
copies-set = Набір { $n }
copies-set-wasted = { $size } у копіях, які не найкращі
copies-more = Показати ще (залишилось { $count })
settings-page-tags = Теги
settings-page-tags-blurb = Усі теги бібліотеки. Об’єднуйте ті, що означають те саме, або перейменовуйте; тегувальники відтоді використовуватимуть нову назву.
settings-page-store-files = Файли магазинів
settings-page-store-files-blurb = Файли, які магазини кладуть поруч зі своїми ресурсами і які Hoard читає, щоб назвати пакети й зазначити їхніх авторів. Навчіть його тих, яких він не знає, і поділіться тим, чого навчили.
settings-page-locations = Розташування бібліотеки
settings-page-locations-blurb = Теки на сервері, що містять бібліотеку: ті, де Hoard зберігає файли, і теки з пакетами, які він індексує на місці.
settings-page-locations-blurb-desktop = Теки, що містять бібліотеку: ті, де Hoard зберігає файли, і теки з пакетами, які він індексує на місці.
settings-page-tools = Інструменти
settings-page-tools-blurb = Зовнішні програми, якими Hoard перетворює й читає файли.
settings-page-ai = Тегування ШІ
settings-page-ai-blurb = Модель ШІ дає назви, описи й теги новим пакетам на додачу до вбудованих правил. Її теги позначено як запропоновані.
settings-page-search-blurb = Шукайте зображення за тим, що на них зображено, а звуки — за тим, як вони звучать, а не лише за назвами, і знаходьте речі, схожі за суттю, а не лише за кольором.
settings-page-search-tab = Пошук
settings-page-adding = Додавання пакетів
settings-page-adding-blurb = Що Hoard робить із пакетом, коли його додано, окрім читання й тегування.
settings-page-downloads = Завантаження
settings-page-downloads-blurb = Як називаються файли в тому, що надсилає Hoard: завантаженнях, zip-архівах, інструменті hoard і для ШІ-асистентів. Файли в бібліотеці залишаються як є.
settings-page-account = Обліковий запис
settings-page-account-blurb = Ваш пароль, інші люди, які можуть входити, і токени доступу для скриптів та інших програм.
settings-page-auth = Автентифікація
settings-page-auth-blurb = Як люди входять: власні облікові записи Hoard, каталог LDAP, Microsoft Active Directory або постачальник OAuth, скільки тривають сеанси і які групи отримують яку роль.
settings-page-connection = Підключення
settings-page-connection-blurb = Як браузери й застосунки звертаються до Hoard: адреса, за якою його знають, і HTTPS, потрібний для деяких функцій браузера.
settings-page-performance = Продуктивність
settings-page-performance-blurb = Наскільки інтенсивно працює сервер, як створюються й зберігаються мініатюри і скільки він щонайбільше приймає.
settings-page-performance-blurb-desktop = Наскільки інтенсивно працює Hoard, як створюються й зберігаються мініатюри і скільки він щонайбільше приймає.
settings-page-issues = Проблеми
settings-page-issues-blurb = Що пішло не так під час відкриття чи обробки файлів: архіви, що не розпакувалися, файли, які не вдалося прочитати, завдання, що завершилися помилкою. Спробуйте знову, коли причину усунено, або відхиліть їх.
settings-page-maintenance = Обслуговування
settings-page-maintenance-blurb = Резервне копіювання, перевірка на пошкодження й прибирання. Кожне завдання виконується в одному екземплярі, і поки вони тривають, цю сторінку можна залишити.
settings-page-about = Про програму
settings-page-about-blurb = Що таке Hoard, яка це версія і чужа праця, з якої його зроблено.
settings-the-default = типове значення
settings-default-on = увімкнено
settings-default-off = вимкнено
settings-default-auto = автоматично
settings-option-saved = { $name }: { $value }
settings-back-to-default = Повернути типове значення
settings-save = Зберегти
settings-use = Використати
settings-status-off = Вимкнено

## SettingsPage.vue: Вигляд

settings-themes = Теми
settings-look-light = Світла
settings-look-dark = Темна
settings-look-group =
    { $scheme ->
        [light] Світла тема
       *[dark] Темна тема
    }
settings-preview =
    Для перегляду показано { $preview ->
        [light] світлу
       *[dark] темну
    } тему. Ви використовуєте { $used ->
        [light] світлу
       *[dark] темну
    }{ $system ->
        [yes] { " " }(як у системі)
       *[other] { "" }
    }, і вона повернеться, коли ви залишите цю сторінку.
settings-preview-back =
    Повернутися до { $used ->
        [light] світлої
       *[dark] темної
    }
settings-look-hint = Перемикайтеся між світлою й темною темами (або вибирайте «як у системі») на верхній панелі.

## SettingsPage.vue: Інструменти

settings-blender-what = У фоновому режимі перетворює моделі .blend і .abc на glTF, щоб вони отримали мініатюри й 3D-перегляд. Пакети, додані, поки він був вимкнений, перетворюються, щойно він буде готовий.
settings-blender-download = Використовує встановлений Blender або завантажує Blender 4.5 LTS (близько 400 МБ) із дзеркала blender.org.
settings-ffmpeg-what = Кодує завантаження в MP3, OGG та Opus і читає аудіо, яке Hoard не може декодувати сам (AAC, M4A, WMA, …).
settings-ffmpeg-download = Використовує системний ffmpeg із потрібними кодувальниками або завантажує статичну збірку (близько 80 МБ).
settings-tool-reset = { $tool }: повернуто типове значення
settings-tool-no-download = Використовує встановлений { $tool }. Для цієї машини немає збірки для завантаження, тож установіть його або вкажіть шлях.
settings-tool-path = Шлях до програми { $tool }
settings-mode-off = Вимкнено
settings-mode-auto = Автоматично
settings-mode-path = Ця програма
settings-tool-looking = Пошук…
settings-tool-downloading = Завантаження… прогрес видно в індикаторі активності
settings-tool-ready = Готово: { $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = Інструменти, якими відтворюються й перетворюються MIDI-файли.
settings-soundfont-file = Файл SoundFont
settings-soundfont-placeholder = Знайдений на сервері або вбудований
settings-soundfont-placeholder-desktop = Знайдений на цьому комп’ютері або вбудований
settings-soundfont-default = знайдений на сервері або вбудований
settings-soundfont-default-desktop = знайдений на цьому комп’ютері або вбудований
settings-soundfont-send = Вибрати файл…
settings-soundfont-sent = Тепер MIDI грає з { $name }

## SettingsPage.vue: Пошук зображень

settings-clip-heading = Пошук за тим, що зображено
settings-clip-reset = Пошук зображень: повернуто типове значення
settings-clip-saved-on = Пошук зображень: увімкнено
settings-clip-saved-off = Пошук зображень: вимкнено
settings-clip-what = Введіть, що шукаєте («mossy rock», «sci-fi door», «wooden barrel»), і зображення, на яких це є, з’являться поруч із назвами, що містять ці слова. «Більше схожого» знаходить речі того самого типу, а не лише тих самих кольорів.
settings-clip-toggle = Увімкнути пошук зображень
settings-clip-hint = Використовує модель SigLIP 2 від Google, що працює на процесорі цього сервера: нічого за його межі не виходить. Увімкнення завантажує модель ({ $size } МБ) з Hugging Face, а потім у фоновому режимі з низьким пріоритетом вивчає кожне зображення. Поки модель працює або хтось шукає, вона займає від 350 до 750 МБ пам’яті. Вимкнення видаляє модель і зберігає вивчене на наступний раз.
settings-clip-hint-desktop = Використовує модель SigLIP 2 від Google, що працює на процесорі цього комп’ютера: нічого за його межі не виходить. Увімкнення завантажує модель ({ $size } МБ) з Hugging Face, а потім у фоновому режимі з низьким пріоритетом вивчає кожне зображення. Поки модель працює або ви шукаєте, вона займає від 350 до 750 МБ пам’яті. Вимкнення видаляє модель і зберігає вивчене на наступний раз.
settings-clip-looking = Пошук моделі…
settings-clip-downloading = Завантаження моделі… прогрес видно в індикаторі активності
settings-clip-ready-empty = Готово. Зображення вивчаються в міру додавання пакетів.
settings-clip-ready-all =
    { $count ->
        [one] Готово: знає { $count } зображення
        [few] Готово: знає всі { $count } зображення
        [many] Готово: знає всі { $count } зображень
       *[other] Готово: знає всі { $count } зображення
    }
settings-clip-learning = Готово: вивчає зображення, поки що { $done } з { $total }
settings-clip-relearning = Доки нова модель не вивчить усі зображення, пошук використовує те, що вивчила стара.
settings-clip-unavailable = Не вдалося отримати модель: { $detail }. Повторна спроба щогодини.
settings-clip-where = Де працює
settings-clip-here = На цьому сервері: нічого не виходить назовні
settings-clip-here-desktop = На цьому комп’ютері: нічого не виходить назовні
settings-clip-hosted = У зовнішньому сервісі
settings-clip-hosted-warning = Зображення залишають цей сервер. Мініатюра кожного зображення в бібліотеці та слова кожного пошуку зображень надсилаються до { $service }, який бере за них плату й зберігає їх так, як сказано в його умовах.
settings-clip-hosted-warning-desktop = Зображення залишають цей комп’ютер. Мініатюра кожного зображення в бібліотеці та слова кожного пошуку зображень надсилаються до { $service }, який бере за них плату й зберігає їх так, як сказано в його умовах.
settings-clip-hosted-hint = Тут нічого не завантажується й не запускається: сервіс вивчає кожне зображення партіями. Інший сервіс або модель вивчає їх усі заново, а те, що вивчив кожен, зберігається, щоб можна було повернутися.
settings-clip-saved-here = Пошук зображень: працює на цьому сервері
settings-clip-saved-here-desktop = Пошук зображень: працює на цьому комп’ютері
settings-clip-saved-hosted = Пошук зображень: працює в { $service }
settings-clip-hosted-unavailable = Не вдалося скористатися { $service }: { $detail }. Повторна спроба щогодини.
settings-clap-heading = Пошук за звучанням
settings-clap-reset = Пошук звуків: повернуто типове значення
settings-clap-saved-on = Пошук звуків: увімкнено
settings-clap-saved-off = Пошук звуків: вимкнено
settings-clap-what = Введіть, що шукаєте («distant thunder», «glassy UI blip», «footsteps on gravel»), і звуки, які так звучать, з’являться поруч із назвами, що містять ці слова.
settings-clap-toggle = Увімкнути пошук звуків
settings-clap-hint = Використовує модель CLAP від LAION, що працює на процесорі цього сервера: нічого за його межі не виходить. Увімкнення завантажує модель ({ $size } МБ) з Hugging Face, а потім у фоновому режимі з низьким пріоритетом прослуховує початок кожного звуку. Поки модель працює або хтось шукає, вона займає приблизно до 1 ГБ пам’яті. Вимкнення видаляє модель і зберігає вивчене на наступний раз.
settings-clap-hint-desktop = Використовує модель CLAP від LAION, що працює на процесорі цього комп’ютера: нічого за його межі не виходить. Увімкнення завантажує модель ({ $size } МБ) з Hugging Face, а потім у фоновому режимі з низьким пріоритетом прослуховує початок кожного звуку. Поки модель працює або ви шукаєте, вона займає приблизно до 1 ГБ пам’яті. Вимкнення видаляє модель і зберігає вивчене на наступний раз.
settings-clap-ready-empty = Готово. Звуки вивчаються в міру додавання пакетів.
settings-clap-ready-all =
    { $count ->
        [one] Готово: знає { $count } звук
        [few] Готово: знає всі { $count } звуки
        [many] Готово: знає всі { $count } звуків
       *[other] Готово: знає всі { $count } звуку
    }
settings-clap-learning = Готово: вивчає звуки, поки що { $done } з { $total }

## SettingsPage.vue: Тегування ШІ

settings-ai-service = Служба
settings-ai-off = Вимкнено
settings-ai-off-hint = Пакети отримують теги лише за вбудованими правилами.
settings-ai-local = Локальна модель
settings-ai-local-hint = Служба llm у файлі compose для Hoard (або cargo xtask llm). Завантажте модель нижче; вона працює на цій машині, і нічого за її межі не виходить.
settings-ai-chatgpt = План ChatGPT
settings-ai-chatgpt-hint = Використовує ваш план ChatGPT Go, Plus, Pro чи Business замість ключа API.
settings-ai-openrouter-hint = Один ключ для сотень хмарних моделей, деякі з них безкоштовні. Моделі, що приймають зображення, позначено.
settings-ai-openai-hint = Розміщено в OpenAI.
settings-ai-anthropic-hint = Розміщено в Anthropic, через її OpenAI-сумісну кінцеву точку.
settings-ai-gemini-hint = Розміщено в Google, через його OpenAI-сумісну кінцеву точку.
settings-ai-ollama-hint = Сервер Ollama, який ви запускаєте. Змініть адресу, якщо це не контейнер з назвою ollama.
settings-ai-custom = Інше (OpenAI-сумісне)
settings-ai-custom-hint = Будь-що, що підтримує OpenAI chat API: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Підключення…
settings-ai-connected =
    { $count ->
        [one] Підключено · { $count } модель
        [few] Підключено · { $count } моделі
        [many] Підключено · { $count } моделей
       *[other] Підключено · { $count } моделі
    }
settings-ai-connected-none = Підключено, але служба не показує жодної моделі; введіть назву моделі
settings-chatgpt-connected-none = Підключено; введіть назву моделі
settings-ai-takes-images = приймає зображення
settings-ai-saved = Тегування ШІ: { $service }
settings-ai-saved-off = Тегування ШІ вимкнено
settings-ai-reset = Тегування ШІ: повернуто типове значення
settings-ai-on = Увімкнено: { $service } · { $model }
settings-ai-url = Адреса
settings-ai-key = Ключ API
settings-ai-key-saved = Збережено (введіть, щоб замінити)
settings-ai-key-paste = Вставте свій ключ
settings-ai-key-optional = Немає (необов’язково)
settings-ai-key-remove = Вилучити
settings-ai-get-key = Отримати ключ
settings-ai-model = Модель
settings-ai-model-pick = Виберіть або введіть модель
settings-ai-model-name = Назва моделі
settings-ai-refresh = Оновити
settings-ai-connect = Підключити
settings-ai-need-key = Додайте ключ API, щоб побачити моделі
settings-ai-vision = Надсилати й мініатюри (потрібна модель, що приймає зображення)
settings-ai-batch = На запит
settings-ai-batch-unit = ресурсів (менше для малих локальних моделей, більше для великих хмарних)

## SettingsPage.vue: Вхід у ChatGPT

settings-chatgpt-account = Обліковий запис
settings-chatgpt-checking = Перевірка…
settings-chatgpt-status-as = Вхід виконано як { $email }
settings-chatgpt-status = Вхід виконано
settings-chatgpt-signed-in-as = Вхід у ChatGPT виконано як { $email }
settings-chatgpt-signed-in = Вхід у ChatGPT виконано
settings-chatgpt-signed-out = Вихід із ChatGPT виконано
settings-chatgpt-continue = Продовжити з ChatGPT
settings-chatgpt-sign-out = Вийти
settings-chatgpt-sign-out-title = Вийти з ChatGPT?
settings-chatgpt-sign-out-in-use = Тегування ШІ використовує ваш план ChatGPT, тож воно зупиниться, доки ви не ввійдете знову або не виберете іншу службу.
settings-chatgpt-sign-out-body = Hoard забуде вхід і попросить OpenAI завершити його.
settings-chatgpt-manage = Керувати використанням
settings-chatgpt-finish-before = Завершіть вхід у вікні, що відкрилося (
settings-chatgpt-finish-link = відкрити знову
settings-chatgpt-finish-middle = ). Якщо все закінчиться сторінкою, що не завантажується, з адресою, яка починається з{ " " }
settings-chatgpt-finish-after = , скопіюйте цю адресу й вставте сюди:
settings-chatgpt-paste = Адреса, на якій завершився вхід
settings-chatgpt-finish = Завершити
settings-chatgpt-usage = Запити на тегування враховуються у використанні вашого плану ChatGPT. «Керувати використанням» показує, скільки використав Hoard, і дає змогу це обмежити. Щоб відключити Hoard від самого ChatGPT, перейдіть на chatgpt.com у Settings → Security and login → Sign in with ChatGPT.

## SettingsPage.vue: Локальні моделі

settings-local-downloading-progress = Завантаження { $percent }% · { $done } з { $total }
settings-local-downloading = Завантаження…
settings-local-incomplete = Завантаження зупинено
settings-local-failed = Минулого разу не вдалося запустити
settings-local-in-use-running = Використовується · працює
settings-local-in-use = Використовується
settings-local-starting = Запуск…
settings-local-running = Працює
settings-local-ready-images = Готово · приймає зображення
settings-local-ready-text = Готово · лише текст
settings-local-delete-title = Видалити { $name }?
settings-local-delete-in-use = Тегування ШІ використовує цю модель, тож воно зупиниться, доки ви не виберете іншу. Її файли буде видалено із сервера моделей.
settings-local-delete-downloading = Завантаження зупиниться, а вже завантажене буде видалено.
settings-local-delete-body = Її файли буде видалено із сервера моделей. Пізніше її можна завантажити знову.
settings-local-delete = Видалити
settings-local-deleting = Видалення…
settings-local-cancel = Скасувати
settings-local-resume = Продовжити
settings-local-download = Завантажити
settings-local-start-before = Запустіть його командою{ " " }
settings-local-start-or = { " " }(або{ " " }
settings-local-start-developing = { " " }під час розробки), а потім{ " " }
settings-local-start-retry = спробуйте знову
settings-local-start-after = . Сервер, що запускає лише одну модель, теж підійде: вкажіть назву його моделі нижче.
settings-local-list = На сервері моделей
settings-local-none = Ще нічого не завантажено. Виберіть модель нижче; вона зберігатиметься на сервері, і її можна будь-коли змінити.
settings-local-catalog = Моделі для завантаження
settings-local-size = { $size } ГБ · { $license }
settings-local-other = Інша модель із Hugging Face
settings-local-other-placeholder = Інша з Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Додавання пакетів

settings-store-pages = Сторінки магазинів
settings-store-pages-what = Нові пакети у фоновому режимі шукаються в магазинах, щоб отримати сторінку магазину, продавця й ліцензію.
settings-store-lookup = Автоматично шукати сторінки магазинів
settings-unreal = Пакунки Unreal
settings-unreal-reset = Пакунки Unreal: повернуто типове значення
settings-unreal-what = Що робити з файлами .uasset.
settings-conv = Правила перетворення
settings-conv-what = Перетворюйте файли, які зберігає Hoard, на один формат, наприклад усі OBJ і FBX на GLB, щоб уся бібліотека була однорідною. Перетворений файл займає місце оригіналу разом з його назвою, тегами, колекціями та посиланнями. Файли у відстежуваних теках ніколи не змінюються.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Увімкнено
settings-conv-to = Перетворювати на
# Heading over the file formats a rule converts.
settings-conv-from = З форматів
settings-conv-remove = Вилучити
settings-conv-remove-label = Вилучити правило { $n }
settings-conv-taken = Цей формат уже перетворює інше правило
settings-conv-upright = Ставити моделі вертикально під час перетворення (Y вгору, обличчям до +Z), як показано моделі пакета
settings-conv-flac-what = Звук залишається точно таким самим, а місця займає приблизно вдвічі менше, і все інше з WAV (точки петлі, маркери cue, теги) зберігається у FLAC, тож WAV можна будь-коли повернути точно таким, яким він був. WAV із семплами з рухомою комою або 32-бітними семплами залишаються WAV.
settings-conv-add = Додати правило
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Зберігати оригінали
settings-conv-keep-days =
    { $count ->
        [one] день
        [few] дні
        [many] днів
       *[other] дня
    }
settings-conv-keep-what = До того часу на сторінці перетвореного файлу можна повернути оригінал. Текстури не копіюються в перетворену модель: вона посилається на файли зображень поруч із нею. Файли .mtl чи .bin моделі залишаються.
settings-conv-save = Зберегти правила
settings-conv-revert = Скасувати зміни
settings-conv-existing = Перетворити файли, що вже є в бібліотеці
settings-conv-saved = Правила перетворення збережено. Вони застосовуються до пакетів, доданих відтепер.
settings-conv-converting =
    { $count ->
        [one] У фоновому режимі перетворюється { $count } файл
        [few] У фоновому режимі перетворюються { $count } файли
        [many] У фоновому режимі перетворюється { $count } файлів
       *[other] У фоновому режимі перетворюється { $count } файлу
    }
settings-conv-nothing = У бібліотеці немає нічого, що треба перетворити.
settings-conv-ask-title =
    { $count ->
        [one] Перетворити { $count } файл, що вже є в бібліотеці?
        [few] Перетворити { $count } файли, що вже є в бібліотеці?
        [many] Перетворити { $count } файлів, що вже є в бібліотеці?
       *[other] Перетворити { $count } файлу, що вже є в бібліотеці?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Це стосується { $packs } пакета.
        [few] Це стосується { $packs } пакетів.
        [many] Це стосується { $packs } пакетів.
       *[other] Це стосується { $packs } пакета.
    } Кожен оригінал зберігається { $days } дн., і до того часу його можна повернути з його сторінки.
settings-conv-ask-busy =
    { $count ->
        [one] { $count } з них у пакеті, який ще додається; його перетворять, щойно це завершиться.
        [few] { $count } з них у пакетах, які ще додаються; їх перетворять, щойно це завершиться.
        [many] { $count } з них у пакетах, які ще додаються; їх перетворять, щойно це завершиться.
       *[other] { $count } з них у пакетах, які ще додаються; їх перетворять, щойно це завершиться.
    }
settings-conv-ask-list = Файли, які буде перетворено
settings-conv-ask-more = …і ще { $count }
settings-conv-ask-skipped = Залишаться як є:
settings-conv-skip-watched =
    { $count ->
        [one] { $count } файл у відстежуваній теці
        [few] { $count } файли у відстежуваних теках
        [many] { $count } файлів у відстежуваних теках
       *[other] { $count } файлу у відстежуваних теках
    }
settings-conv-skip-unity =
    { $count ->
        [one] { $count } файл, який проєкт Unity знає за назвою
        [few] { $count } файли, які проєкт Unity знає за назвами
        [many] { $count } файлів, які проєкт Unity знає за назвами
       *[other] { $count } файлу, які проєкт Unity знає за назвами
    }
settings-conv-skip-used =
    { $count ->
        [one] { $count } файл, на який за назвою посилається сцена чи інший файл
        [few] { $count } файли, на які за назвою посилається сцена чи інший файл
        [many] { $count } файлів, на які за назвою посилається сцена чи інший файл
       *[other] { $count } файлу, на які за назвою посилається сцена чи інший файл
    }
settings-conv-skip-beside =
    { $count ->
        [one] { $count } файл, перетворена копія якого вже є в його пакеті
        [few] { $count } файли, перетворені копії яких уже є в їхніх пакетах
        [many] { $count } файлів, перетворені копії яких уже є в їхніх пакетах
       *[other] { $count } файлу, перетворені копії яких уже є в їхніх пакетах
    }
settings-conv-skip-blender =
    { $count ->
        [one] { $count } файл, який Blender ще не прочитав («Налаштування → Інструменти»)
        [few] { $count } файли, які Blender ще не прочитав («Налаштування → Інструменти»)
        [many] { $count } файлів, які Blender ще не прочитав («Налаштування → Інструменти»)
       *[other] { $count } файлу, які Blender ще не прочитав («Налаштування → Інструменти»)
    }
settings-conv-ask-new = Зберегти, лише нові пакети
settings-conv-ask-save-convert =
    { $count ->
        [one] Зберегти й перетворити { $count } файл
        [few] Зберегти й перетворити { $count } файли
        [many] Зберегти й перетворити { $count } файлів
       *[other] Зберегти й перетворити { $count } файлу
    }
settings-conv-ask-convert =
    { $count ->
        [one] Перетворити { $count } файл
        [few] Перетворити { $count } файли
        [many] Перетворити { $count } файлів
       *[other] Перетворити { $count } файлу
    }
settings-unreal-off = Залишати як є
settings-unreal-off-hint = Перетворюйте зі сторінки пакета, коли захочете.
settings-unreal-keep = Перетворювати, зберігати оригінали
settings-unreal-keep-hint = Текстури, звуки й матеріали виходять у поширених форматах; .uasset теж залишається.
settings-unreal-replace = Перетворювати, видаляти оригінали
settings-unreal-replace-hint = Заощаджує місце, щойно вміст витягнуто. Меші все одно чекають на обробник Unreal.

## SettingsPage.vue: Завантаження

settings-tn = Назви текстур
settings-tn-saved = Назви текстур збережено
settings-tn-reset = Назви текстур: повернуто типове значення
settings-tn-what = Карти матеріалу надсилаються з назвою матеріалу та призначенням кожної карти, хоч би як називалися файли: Rock_Normal.png, а не rock_moss_xeokah0_raw_8K_Normal.png. Файлів у бібліотеці це не торкається, а карта, яку називає надіслана з нею модель або MTL, зберігає цю назву, щоб модель і далі її знаходила.
settings-tn-toggle = Називати карти текстур за їхнім матеріалом
settings-tn-style = Стиль
settings-tn-style-group = Стиль назв
settings-tn-descriptive = Описовий
settings-tn-short = Короткий
settings-tn-custom = Власний
settings-tn-pattern = Шаблон
settings-tn-hint-name = { " " }— матеріал,{ " " }
settings-tn-hint-map = { " " }— призначення карти,{ " " }
settings-tn-hint-size = { " " }— її розмір, коли набір має кілька розмірів (додається в кінці, якщо шаблон його не містить).
settings-tn-example = Приклад
settings-tn-suffixes = Суфікси для кожного типу карт
settings-tn-slots = У завантаженнях GLB текстура, упакована всередину моделі без власного файлу, отримує назву за матеріалом і слотом, який вона займає, і ці слоти теж мають суфікси:
settings-tn-endings = Власні закінчення
settings-tn-endings-what = Текстура, яка не є однією з карт матеріалу, зберігає власну назву, якщо тільки та не закінчується на одне з цих: _diff → _BC надсилає rock_diff.png як rock_BC.png. Залиште друге поле порожнім, щоб прибрати закінчення.
settings-tn-ending-from = Закінчення
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Замінити на
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Додати закінчення
settings-tn-ending-remove = Прибрати
settings-tn-ending-remove-label = Прибрати { $ending }

## SettingsPage.vue: Загальні та Підключення

settings-name = Назва
settings-name-what = Як цей сервер називається на верхній панелі, у вкладці браузера та на сторінці входу, щоб за кількох серверів Hoard було видно, який є який.
settings-name-what-desktop = Як ця бібліотека називається на верхній панелі.
settings-address = Адреса
settings-address-what = Адреса, якою люди й застосунки звертаються до Hoard, якщо вона відрізняється від тієї, на якій цей браузер: адреса зворотного проксі, тунелю чи імені в tailnet, наприклад https://hoard.example.com. Її використовують посилання, які видає Hoard (для Claude та інших застосунків, під час входу, у своєму сертифікаті).
settings-public-url = Публічна адреса
settings-public-url-placeholder = З адреси браузера (напр. https://hoard.example.com)
settings-public-url-default = з адреси браузера

## ProfilerPanel.vue: профілювальник угорі сторінки «Налаштування → Продуктивність»

profiler = Профілювальник
profiler-what = Що Hoard робив за останні десять хвилин; оновлюється кожні дві секунди, поки ви дивитеся. Зберігається лише в пам’яті й починається заново після перезапуску Hoard.
profiler-starting = Перші показники з’являться за мить.
profiler-failed = Не вдалося прочитати профілювальник: { $error }
profiler-cpu = Завантаження процесора
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Уся машина
profiler-cpu-summary = Hoard використовує { $hoard } процесорів, а вся машина — { $machine }.
profiler-ram = ОЗП
profiler-ram-peak = Пік
profiler-ram-of = { $used } з { $total }
profiler-ram-summary = Hoard використовує { $hoard } ОЗП, а за останні десять хвилин — щонайбільше { $peak }.
profiler-serving = Запити в роботі
profiler-working = У роботі
profiler-working-none = У фоні нічого не виконувалося.
profiler-working-more =
    { $count ->
        [one] І ще { $count }.
        [few] І ще { $count }.
        [many] І ще { $count }.
       *[other] І ще { $count }.
    }
profiler-requests = Час обслуговування запитів
profiler-requests-what = Кожен стовпчик — це десять секунд запитів, а їхній час розподілено за тим, на що він пішов.
profiler-part-db = База даних
profiler-part-db-title = Очікування бази даних і виконання її запитів
profiler-part-disk = Диск
profiler-part-disk-title = Читання збережених файлів і стиснення того, що надсилається
profiler-part-send = Надсилання
profiler-part-send-title = Очікування, поки мережа прийме надіслане
profiler-part-other = Інше
profiler-part-other-title = Підготовка відповіді
profiler-request-count =
    { $count ->
        [one] { $count } запит
        [few] { $count } запити
        [many] { $count } запитів
       *[other] { $count } запиту
    }
profiler-requests-none = За останні десять хвилин запитів не було.
profiler-slowest = Найповільніший: { $route }, { $time }
profiler-routes = За маршрутами
profiler-route = Маршрут
profiler-count = Запити
profiler-average = Середнє
profiler-p95 = Найповільніші 5%
profiler-split = Куди пішов час
profiler-sent = Надіслано
profiler-portal = власні файли інтерфейсу

## SettingsPage.vue: Продуктивність

settings-render = 3D-мініатюри
settings-render-toggle = Рендерити мініатюри для 3D-моделей
settings-gpu-on = На GPU: { $renderer }
settings-gpu-software = GPU не знайдено, тому програмно ({ $renderer })
settings-gpu-none = Не знайдено ні GPU, ні програмного рендерера, тож моделі не отримують мініатюр
settings-thumbs = Мініатюри
settings-thumbs-what = Зберігаються у WebP, стиснутими або точними — що вийде меншим. Маленьке зображення, яке саме по собі менше, є власною мініатюрою. Зміна тут створює наявні мініатюри заново у фоні.
settings-previews-size = Попередні перегляди: { $size }
settings-thumbs-saved = Мініатюри: { $quality }
settings-thumbs-size = Розмір
settings-thumbs-size-option = { $px } px
settings-thumbs-quality = Якість { $quality }.
settings-thumbs-small = Менші
settings-thumbs-small-hint = Добре в сітці; трохи розмито зблизька.
settings-thumbs-balanced = Збалансовані
settings-thumbs-balanced-hint = У розмірі мініатюри важко відрізнити від оригіналу.
settings-thumbs-sharp = Чіткіші
settings-thumbs-sharp-hint = Приблизно вдвічі більші за збалансовані.
settings-motion-thumbs = Рухомі мініатюри анімацій
settings-motion-thumbs-what = Кожна анімація також отримує короткий зациклений ролик, що грає, поки ви наводите вказівник на її плитку. Кожен займає близько 40 КБ, а для великої бібліотеки їх створення триває якийсь час. Вимкнення їх видаляє.
settings-motion-fps = Частота кадрів
settings-motion-fps-option = { $fps } кадр/с
settings-motion-fps-what = Плавніші ролики довше створюються й займають більше місця: за 24 кадр/с приблизно вдвічі більше, ніж за 12, а за 30 — ще більше. Після зміни всі вони створюються заново.
settings-limits = Робота й обмеження
settings-limits-what = Кожне значення зберігається, коли ви залишаєте поле або натискаєте Enter. Розміри вказуються з одиницями: 100 GB, 512 MB, 1.5 TB; просте число означає ГБ.
settings-workers = Фонові обробники
settings-workers-unit = завдань одночасно (імпорт, тегування, перетворення)
settings-max-upload = Найбільше вивантаження
settings-max-upload-unit = на файл
settings-max-expand = Найбільший пакет
settings-max-expand-unit = після розпакування
settings-conversion-cache = Перетворені завантаження
settings-conversion-cache-unit-now = зберігається щонайбільше, зараз { $size }; першими видаляються ті, що давно не використовувалися
settings-conversion-cache-unit = зберігається щонайбільше; першими видаляються ті, що давно не використовувалися

## SettingsPage.vue: Обслуговування

settings-updates = Оновлення
settings-version = Це Hoard { $version }.
settings-update-check = Перевіряти GitHub на нові випуски раз на день
settings-desktop-updates = Hoard шукає нову версію під час кожного запуску, завантажує її у фоні та встановлює, коли ви виходите.
settings-backups = Резервні копії за розкладом
settings-backups-what = База даних сама робить резервні копії; кожна з’являється в історії нижче.
settings-backup-every = Резервна копія
settings-backup-off = Ніколи
settings-backup-daily = Щодня
settings-backup-weekly = Щотижня
settings-backup-keep = Зберігати
settings-backup-keep-unit = останніх копій за розкладом
settings-backup-copy-to = Також копіювати в
settings-backup-copy-to-none = Без іншої теки
settings-backups-kept = Кожна зберігається в теці резервних копій як hoard-auto-…db і, якщо вказано, в іншій теці (інший диск або тека, яку зберігає застосунок синхронізації), заданій повним шляхом на сервері. Видаляються лише старіші копії за розкладом; зроблені вручну залишаються.
about-what = Hoard зберігає ігрові ресурси (моделі, текстури, звуки, музику, спрайти тощо) в одній бібліотеці, у якій можна шукати, переглядати й звідки можна брати потрібне, спільній для всіх, кому ви надали доступ.
about-version = Версія
about-edition = Редакція
about-edition-server = Сервер
about-edition-desktop = Застосунок для комп’ютера
about-edition-steam = Застосунок для комп’ютера зі Steam
about-made-by = Розробник
about-source = Вихідний код на GitHub
about-issues = Повідомити про проблему
about-discord = Поспілкуватися з нами в Discord
about-steam = Hoard у Steam
about-license = Ліцензія
about-license-what = Hoard — відкрите ПЗ за ліцензією MIT No Attribution: використовуйте, змінюйте й поширюйте його як завгодно.
about-downloaded = Завантажується за потреби
about-downloaded-what = Вони не входять до складу Hoard. Кожен із них завантажується, коли вперше знадобиться, і кожен має власну ліцензію.
about-blender = Перетворює 3D-моделі та створює їхні мініатюри.
about-ffmpeg = Перетворює звук і відео та створює їхні попередні перегляди.
about-siglip = Знаходить зображення за тим, що на них.
about-clap = Знаходить звуки за тим, як вони звучать.
about-models = На моделі, які ви обираєте для локального ШІ-сервера, поширюються ліцензії їхніх авторів.
about-built-with = Створено за допомогою
about-built-with-what = Бібліотеки з відкритим кодом, з яких зроблено Hoard, з їхніми ліцензіями.
about-steamworks = Версія для Steam також містить Steamworks SDK, © Valve Corporation.
about-licenses-loading = Завантаження списку…
about-licenses-failed = Не вдалося завантажити список.
about-filter = Фільтр за назвою або ліцензією
about-count = { $shown } з { $total }
about-no-license = Ліцензію не вказано
about-report = Повідомити про проблему…
report-title = Повідомити про проблему
report-what = Це те, що потрібно issue на GitHub, щоб проблему можна було дослідити. Нічого приватного тут немає: токени доступу, паролі, ключі, адреси електронної пошти та ваше ім’я в шляхах до тек вилучаються. Перечитайте перед надсиланням.
report-loading = Збирання звіту…
report-failed = Не вдалося зібрати звіт: { $error }
report-log = Що сервер нещодавно записав у журнал
report-copy = Копіювати
report-copied = Звіт скопійовано.
report-github = Відкрити issue на GitHub
report-github-hint = Новий issue заповнюється цим. Якщо журнал задовгий, туди потрапляють лише останні рядки; у «Копіювати» є все.
report-close = Закрити

## Asset kinds (api.ts)

api-kind-texture = Текстури
api-kind-vector = Векторна графіка
api-kind-sheet = Аркуші спрайтів
api-kind-audio = Аудіо
api-kind-model = Моделі
api-kind-animation = Анімації
api-kind-font = Шрифти
api-kind-shader = Шейдери
api-kind-material = Матеріали
api-kind-video = Відео
api-kind-document = Документи
api-kind-data = Дані
api-kind-other = Інше
api-kind-texture-name = текстури
api-kind-vector-name = векторна графіка
api-kind-sheet-name = аркуші спрайтів
api-kind-audio-name = аудіо
api-kind-model-name = моделі
api-kind-animation-name = анімації
api-kind-font-name = шрифти
api-kind-shader-name = шейдери
api-kind-material-name = матеріали
api-kind-video-name = відео
api-kind-document-name = документи
api-kind-data-name = дані
api-kind-other-name = інше

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

news-button = Нове: { $count }
news-button-title =
    { $count ->
        [one] { $count } новинка в Hoard
        [few] { $count } новинки в Hoard
        [many] { $count } новинок в Hoard
       *[other] { $count } новинки в Hoard
    }
news-pip =
    { $count ->
        [1] Нове в Hoard
        [one] Тут { $count } новинка
        [few] Тут { $count } новинки
        [many] Тут { $count } новинок
       *[other] Тут { $count } новинки
    }
news-title = Нове в Hoard
news-to-find =
    { $count ->
        [one] Залишилося знайти: { $count }
        [few] Залишилося знайти: { $count }
        [many] Залишилося знайти: { $count }
       *[other] Залишилося знайти: { $count }
    }
news-close = Закрити
news-got-it = Зрозуміло
news-take-me = Перейти
news-look-again = Подивитися ще раз
news-none = Поки нічого нового.
news-seen-lately = Нещодавно переглянуте
news-stop-pointing = Не позначати нове
news-point-out = Знову позначати нове
news-mark-all = Позначити все як переглянуте
news-marked-seen =
    { $count ->
        [one] { $count } новинку позначено як переглянуту
        [few] { $count } новинки позначено як переглянуті
        [many] { $count } новинок позначено як переглянуті
       *[other] { $count } новинки позначено як переглянуті
    }
news-pack-covers-title = Пакети показують власні зображення
news-pack-covers-body = Пакет із прев’ю чи знімками екрана використовує їх як обкладинку й гортає їх, поки вказівник над ним у списку пакетів, а також на своїй панелі.
news-models-read-title = Більше 3D-форматів, без Blender
news-models-read-body = Моделі Collada, X3D, VRML, OFF і VRM, анімації BVH і гаусові сплати тепер отримують мініатюри, відомості й 3D-перегляд без Blender, і їх можна завантажити як GLB.
news-mesh-explorer-title = Літайте серед моделей із пошуку
news-mesh-explorer-body = Кнопка з кубом над результатами пошуку з 3D-моделями відкриває оглядач мешів: усі знайдені моделі стоять рядами на безкінечній підлозі, і між ними можна літати.
news-tag-lookalikes-title = Об’єднуйте схожі теги
news-tag-lookalikes-body = Теги, що виглядають як той самий, як-от множина, слова разом, пропущена літера чи просте слово та його близнюк із простором імен, показуються списком, щоб їх об’єднати або позначити як різні.
news-mcp-everything-title = Застосунки ШІ можуть те саме, що й ви
news-mcp-everything-body = Claude та інші застосунки ШІ, під’єднані через MCP, тепер можуть робити все, що робить вебзастосунок Hoard, рівно з тими правами, що має обліковий запис або токен, з яким вони входять.
news-themes-title = Десять світлих і десять темних тем
news-themes-body = До світлих тем додалися Sky, Mint, Apricot і Lemon, до темних — Midnight, Lagoon і Neon, а новий вигляд розходиться від плитки, яку ви вибрали.
news-tiles-move-title = Плитки в русі
news-tiles-move-body = Анімації та анімовані зображення на плитках бібліотеки можуть відтворюватися весь час, а не лише під вказівником.
news-motion-trails-title = Траєкторії руху в анімаціях
news-motion-trails-body = Мініатюра анімації прокреслює шлях персонажа, а крапка показує, де він зараз.
news-tags-under-title = Тег знаходить усе, що під ним
news-tags-under-body = Тег у полі пошуку тепер знаходить і все, що під ним, мінус перед ним виключає все це, а панель ресурсу показує ширші теги, під які потрапляють його власні.
news-motion-fps-title = Частота кадрів для рухомих мініатюр
news-motion-fps-body = Мініатюри анімацій тепер можна робити з частотою 12, 24 або 30 кадрів за секунду.
news-text-size-title = Розмір тексту й відступи
news-text-size-body = Зробіть текст більшим або меншим, сторінку — просторішою або компактнішою, і виберіть, що показують плитки й верхня панель.
news-tag-tree-title = Теги в тегах
news-tag-tree-body = Помістіть тег під ширший, і пошук за «транспорт» знайде також «автомобіль» і «седан». Вкладати теги можна в «Налаштування → Теги» або на сторінці тегу.
news-bulk-edit-title = Редагування багатьох одразу
news-bulk-edit-body = Виберіть кілька ресурсів і натисніть E, щоб разом змінити їхні теги, ліцензію, оцінку та інше — з можливістю скасування.
news-backups-title = Резервні копії за розкладом
news-backups-body = Робіть резервну копію бази даних щодня або щотижня, зберігайте кілька найновіших і копіюйте кожну в іншу теку.
ui-undo = Скасувати
ui-today = сьогодні
ui-yesterday = учора
ui-days-ago =
    { $days ->
        [one] { $days } день тому
        [few] { $days } дні тому
        [many] { $days } днів тому
       *[other] { $days } дня тому
    }
ui-notifications = Сповіщення
ui-dismiss = Закрити
ui-cancel = Скасувати
ui-lazy-loading = Завантаження: { $what }…
ui-lazy-failed = Не вдалося завантажити: { $what }.
ui-lazy-reload = Перезавантажити сторінку

## ResetButton.vue

reset-title = Повернути типове значення: { $value }
reset-label = Повернути типове значення, { $value }

## SizeInput.vue

size-invalid = Розмір на зразок 100 GB, 512 MB чи 1.5 TB
size-hint = Розмір на зразок 100 GB, 512 MB чи 1.5 TB; просте число означає ГБ

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = Не вдалося завантажити налаштування автентифікації: { $error }
auth-saved = Збережено. Діятиме з наступного входу.
auth-method-internal = Власні
auth-method-internal-hint = Імена й паролі, які зберігає Hoard; додаються в «Налаштування → Обліковий запис».
auth-method-ldap-hint = Каталог перевіряє ім’я та пароль, введені на сторінці входу: OpenLDAP, FreeIPA, LLDAP чи інший сервер LDAP.
auth-method-ad-hint = Облікові записи домену Windows входять зі своїм звичайним ім’ям і паролем.
auth-method-oauth-hint = Кнопка «Увійти через…», що веде до Google, Microsoft, GitHub, Keycloak, Authentik чи іншого постачальника й назад.
auth-session-unused = Після 30 днів без використання
auth-session-1h = Через 1 годину після входу
auth-session-8h = Через 8 годин після входу
auth-session-day = Через день після входу
auth-session-week = Через тиждень після входу
auth-session-30d = Через 30 днів після входу
auth-session-hours =
    { $hours ->
        [one] Через { $hours } годину після входу
        [few] Через { $hours } години після входу
        [many] Через { $hours } годин після входу
       *[other] Через { $hours } години після входу
    }
auth-role-view = Може переглядати
auth-role-edit = Може змінювати
auth-role-admin = Адміністратор
auth-role-word-view = може переглядати
auth-role-word-edit = може змінювати
auth-role-word-admin = адміністратор
auth-role-word-default = { $role } (типова роль)
auth-tls-starttls-hint = Звичайний LDAP (порт 389), що переходить на TLS, перш ніж щось надіслати.
auth-tls-ldaps-hint = TLS від самого початку (порт 636).
auth-tls-none = Немає
auth-tls-none-hint = Паролі передаються мережею як є. Лише в мережі, якій ви довіряєте.
auth-test-failed = Не вдалося перевірити: { $error }
auth-preset-custom = Інше (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel та інші: вкажіть видавця, і «Виявити» заповнить адреси.
auth-preset-google-note = Створіть клієнт OAuth (Web application) у Google Cloud Console → APIs & Services → Credentials. Google не надсилає груп, тож призначайте ролі в «Налаштування → Обліковий запис» і вимкніть створення облікових записів під час першого входу, якщо не кожен, хто має обліковий запис Google, має отримати доступ.
auth-preset-microsoft-note = Зареєструйте застосунок у Entra ID → App registrations (з переспрямуванням Web на адресу нижче). Замініть «organizations» на ID свого тенанта, щоб пускати лише вашу організацію. Щоб ролі бралися з груп, додайте claim groups у Token configuration: він містить ID груп.
auth-preset-github-note = Створіть OAuth app у GitHub → Settings → Developer settings. GitHub не надсилає груп, тож призначайте ролі в «Налаштування → Обліковий запис» і вимкніть створення облікових записів під час першого входу, якщо не кожен, хто має обліковий запис GitHub, має отримати доступ.
auth-preset-gitlab-note = Створіть застосунок у GitLab → Preferences → Applications (або в адмінській частині свого екземпляра: замініть gitlab.com в адресах).
auth-discovered = Адреси постачальника заповнено.
auth-copied = Скопійовано
auth-copy-failed = Не вдалося скопіювати: виділіть і скопіюйте самостійно
auth-sign-in-with = Увійти через
auth-own-accounts-still-work = Власні облікові записи Hoard і далі входять зі своїми паролями, тож каталог чи постачальник, що не працює або неправильно налаштований, не заблокує вам доступ.
auth-sessions = Сеанси
auth-sign-out-after = Виходити з облікових записів
auth-ldap-server = Сервер LDAP
auth-server-type = Тип сервера
auth-server-type-custom = Власний
auth-servers = Сервери
auth-server-n = Сервер { $n }
auth-remove-server-n = Вилучити сервер { $n }
auth-remove = Вилучити
auth-add-server = Додати сервер
auth-servers-hint = Коли один недоступний, пробується наступний.
auth-security = Безпека
auth-connection-security = Захист з’єднання
auth-skip-verify = Не перевіряти сертифікат сервера (лише для випробування)
auth-ca = Центр сертифікації
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (порожньо: ті, яким довіряє система)
auth-lookup = Пошук людей
auth-anonymously = Анонімно
auth-reader-account = Обліковий запис для читання
auth-reader-password = Його пароль
auth-saved-type-to-replace = Збережено: введіть, щоб замінити
auth-people-under = Люди знаходяться в
auth-name-attribute = Атрибут імені
auth-only-matching = Лише записи, що відповідають
auth-groups-under = Групи знаходяться в
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (порожньо: лише memberOf)
auth-their-groups = Їхні групи
auth-group-filter-hint = { "{" }dn{ "}" } — запис людини, { "{" }user{ "}" } — ім’я, з яким вона ввійшла.
auth-test = Перевірка
auth-test-hint = Звертається до кожного сервера й входить під обліковим записом для читання. Якщо вказано ім’я та пароль, входить і під ними та показує, що з цього зробить Hoard. Нічого не зберігається.
auth-test-user = Під ким увійти
auth-test-user-placeholder = Під ким увійти (необов’язково)
auth-test-password = Пароль цієї людини
auth-testing = Перевірка…
auth-test-connectivity = Перевірити з’єднання
auth-test-server-ok = доступний, вхід під обліковим записом для читання виконано
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }— вхід виконано (
auth-test-signed-in-after = ).
auth-test-role = Роль: { $role }.
auth-test-no-role = Ця людина не входить до жодної групи з розділу «Ролі», тож їй буде відмовлено.
auth-test-groups = Групи: { $groups }
auth-test-no-groups = Груп не знайдено.
auth-provider = Постачальник
auth-redirect = Адреса переспрямування
auth-copy = Копіювати
auth-redirect-hint = Передайте її постачальнику, коли створюватимете клієнт для Hoard. Вона береться з публічної адреси в «Налаштування → Підключення» або з адреси, якою ви користуєтеся.
auth-issuer = Видавець
auth-discover = Виявити
auth-client-id = ID клієнта
auth-client-secret = Секрет клієнта
auth-authorization-url = Адреса авторизації
auth-token-url = Адреса токенів
auth-userinfo-url = Адреса відомостей про користувача
auth-userinfo-placeholder = (порожньо: хто це, каже ID-токен)
auth-scopes = Області доступу
auth-name-from = Ім’я з
auth-groups-from = Групи з
auth-groups-from-placeholder = (немає)
auth-button = Кнопка
auth-button-placeholder = свій обліковий запис
auth-accounts = Облікові записи
auth-provision = Створювати обліковий запис Hoard під час першого входу людини
auth-provision-on-hint = Інакше адміністратор спершу додає кожну людину в «Налаштування → Обліковий запис».
auth-provision-off-hint = Адміністратор спершу додає кожну людину в «Налаштування → Обліковий запис», вибираючи, як вона входитиме.
auth-open-to-anyone = Увійти може будь-хто з обліковим записом { $provider }. Додайте групи для перегляду нижче або вимкніть це й додавайте людей самостійно.
auth-new-accounts = Нові облікові записи
auth-roles-from-groups = Ролі з груп
auth-roles-hint = Групи, учасники яких отримують кожну роль, по одній на рядок, за назвою чи DN. Перемагає найвища відповідна роль; вона перевіряється під час кожного входу. Якщо груп не вказано, нові облікові записи отримують роль, вибрану вище, а адміністратор змінює її згодом. Якщо в «Може переглядати» вказано групи, входити можуть лише учасники вказаних груп.
auth-roles-hint-oauth = Групи, учасники яких отримують кожну роль, по одній на рядок, за назвою чи DN (або за тим, що містить claim груп). Перемагає найвища відповідна роль; вона перевіряється під час кожного входу. Якщо груп не вказано, нові облікові записи отримують роль, вибрану вище, а адміністратор змінює її згодом. Якщо в «Може переглядати» вказано групи, входити можуть лише учасники вказаних груп.
auth-saving = Збереження…
auth-save = Зберегти

## HttpsPanel.vue (Settings → Connection)

https-load-failed = Не вдалося завантажити HTTPS: { $error }
https-off-redirecting = HTTPS вимкнено. Перехід на http://
https-now-on = HTTPS увімкнено
https-saved = Збережено. Нові з’єднання використовують новий сертифікат.
https-now-off = HTTPS вимкнено
https-http3-failed-toast = Не вдалося запустити HTTP/3: { $error }
https-copied = Скопійовано
https-copy-failed = Не вдалося скопіювати: виділіть і скопіюйте самостійно
https-mode-off = Вимкнено
https-mode-off-hint = Лише звичайний http://.
https-mode-own = Власний сертифікат Hoard
https-mode-own-hint = Hoard створює його для адрес цього сервера. Один раз установіть центр сертифікації Hoard на кожному пристрої, і його браузери довірятимуть сертифікату.
https-mode-custom = Мій сертифікат
https-mode-custom-hint = Той, що у вас уже є, напр. з tailscale cert чи Let’s Encrypt. Браузери довіряють йому без жодних установлень.
https-untrusted-own = Цей браузер поки не довіряє йому для { $host }: установіть центр сертифікації Hoard нижче. Ви побачите попередження, яке можна пропустити.
https-untrusted-uncovered = Цей браузер поки не довіряє йому для { $host }: сертифікат не охоплює цієї адреси. Ви побачите попередження, яке можна пропустити.
https-untrusted = Цей браузер поки не довіряє йому для { $host }. Ви побачите попередження, яке можна пропустити.
https-now = Зараз
https-start-failed = HTTPS налаштовано, але не вдалося запустити: { $error }
https-on-port = HTTPS увімкнено на порту { $port }, як і http://.
https-using-it = Ви зараз ним користуєтеся.
https-open = Відкрити { $url }
https-trusted = Цей браузер йому довіряє.
https-cert-for-before = Сертифікат для{ " " }
https-cert-for-after = , дійсний до { $date }.
https-cert-for-after-own = , дійсний до { $date } (оновлюється сам).
https-not-covered-before = Він не охоплює{ " " }
https-not-covered-after = , адреси, якою ви користуєтеся.
https-is-off = HTTPS вимкнено: сторінки передаються звичайним http://.
https-addresses = Адреси
https-addresses-hint = Сертифікат охоплює адреси, які знайшов Hoard, і ті, що ви додасте (ім’я від вашого роутера, ім’я в Tailscale).
https-remove-name = Вилучити { $name }
https-another-address = Інша адреса
https-another-address-placeholder = Інша адреса, напр. nas.local
https-add = Додати
https-your-cert = Ваш сертифікат
https-saved-cert-before = Збережено:{ " " }
https-saved-cert-after = { " " }від { $issuer }, дійсний до { $date }. Вставте новий, щоб замінити.
https-pem-before = Сертифікат (з ланцюжком) і його закритий ключ як текст PEM: файли, що починаються з{ " " }
https-pem-after = .
https-certificate = Сертифікат
https-choose-file = Вибрати файл…
https-private-key = Закритий ключ
https-checked-before = Для{ " " }
https-checked-after = , від { $issuer }, дійсний до { $date }.
https-checked-after-days =
    , від { $issuer }, дійсний до { $date } ({ $days ->
        [one] { $days } день
        [few] { $days } дні
        [many] { $days } днів
       *[other] { $days } дня
    }).
https-self-signed = Він самопідписаний: браузери попереджатимуть про нього.
https-key-matches = Ключ відповідає.
https-where-to-get = Де його взяти
https-get-tailscale-before = увімкніть HTTPS для свого tailnet, потім виконайте{ " " }
https-get-tailscale-after = { " " }на сервері й вивантажте два файли. Вони дійсні 90 днів: до того часу виконайте команду знову й вивантажте їх ще раз.
https-get-acme-title = Let’s Encrypt чи інший клієнт ACME
https-get-acme-before = (certbot, acme.sh, lego) для вашого власного домену: вивантажте{ " " }
https-get-acme-and = { " " }і{ " " }
https-get-acme-after = .
https-get-proxy-title = Зворотний проксі
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), що має сертифікат: залиште HTTPS тут вимкненим, і нехай проксі надсилає до Hoard звичайний http://.
https-trying = Перевірка…
https-apply = Застосувати
https-apply-hint = Hoard спершу пробує нові налаштування й залишає попередні, якщо вони не спрацюють.
https-trust-title = Довіра до Hoard на ваших пристроях
https-download-ca = Завантажити hoard-ca.crt
https-trust-hint = Один раз установіть центр сертифікації Hoard на кожному пристрої, і його браузери довірятимуть цьому серверу (і будь-якому сертифікату, який Hoard створить для нього пізніше). Він може ручатися лише за цей Hoard. Відбиток (SHA-256):
https-ca-windows = Відкрийте файл → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Перезапустіть браузер.
https-ca-macos = Відкрийте файл; він потрапить у Keychain Access. Двічі клацніть там на «{ $subject }» → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone та iPad
https-ca-ios = Завантажте його в Safari й дозвольте. Settings → Profile Downloaded → Install. Потім Settings → General → About → Certificate Trust Settings → увімкніть «{ $subject }».
https-ca-android = Налаштування → Безпека → Інші налаштування безпеки → Шифрування й облікові дані → Установити сертифікат → Сертифікат CA, і виберіть файл. (Назви трохи різняться залежно від телефона; пошукайте в налаштуваннях «сертифікат».)
https-ca-linux-1 = Debian і Ubuntu:{ " " }
https-ca-linux-2 = . Fedora й Arch:{ " " }
https-ca-linux-3 = . Chrome у Linux має власний список: Settings → Privacy and security → Security → Manage certificates → Import.
https-ca-firefox = Firefox має власний список: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import, і позначте «Trust this CA to identify websites».
https-ca-cli-title = Інструмент командного рядка hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , або вкажіть шлях у{ " " }
https-ca-cli-after = .
https-options = Параметри
https-redirect = Переспрямовувати браузери на інших пристроях з http:// на https://
https-redirect-hint = Переспрямовуються лише завантаження сторінок. Інструмент командного рядка hoard, обробник Unreal, скрипти й застосунки ШІ і далі працюють через http://, як і http://localhost на самому сервері.
https-http3-hint-before = Швидше на нестабільному Wi-Fi і телефонах. Потребує HTTPS і відкритого UDP-порту { $port } на додачу до TCP (у Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Працює.
https-http3-waits = Запуститься, коли HTTPS буде ввімкнено.
https-http3-failed = Не вдалося запустити HTTP/3: { $error }
https-lockout-title = Якщо ви втратили доступ
https-lockout-1 = { "" }
https-lockout-2 = { " " }на самому сервері завжди працює через звичайний http://. Або вимкніть HTTPS з командного рядка сервера й запустіть Hoard знову:{ " " }
https-lockout-3 = { " " }(у Docker:{ " " }
https-lockout-4 = , потім{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Може переглядати
account-role-view-hint = Пошук, попередній перегляд і завантаження
account-role-edit = Може змінювати
account-role-edit-hint = А також додавання, зміна, тегування, колекції й архів
account-role-admin = Адміністратор
account-role-admin-hint = А також налаштування, облікові записи, обслуговування й остаточне видалення
account-source-internal = Пароль Hoard
account-source-ldap = Пароль { $directory }
account-source-oauth = Вхід через { $provider }
account-source-word-directory = каталог
account-load-failed = Не вдалося завантажити облікові записи: { $error }
account-failed = { $what }: { $error }
account-password-failed = Не вдалося змінити пароль
account-password-changed = Пароль змінено. На інших ваших пристроях виконано вихід.
account-add-failed = Не вдалося додати { $name }
account-added = { $name } тепер може входити.
account-role-failed = Не вдалося змінити права { $name }
account-remove-title = Вилучити { $name }?
account-remove-body = Цю людину буде виведено з усіх сеансів, і вона більше не зможе ввійти. Нічого з того, що вона додала до бібліотеки, не буде видалено.
account-remove = Вилучити
account-remove-failed = Не вдалося вилучити { $name }
account-token-failed = Не вдалося створити токен
account-copied = Скопійовано
account-copy-token-failed = Не вдалося скопіювати: виділіть токен і скопіюйте самостійно
account-copy-failed = Не вдалося скопіювати: виділіть і скопіюйте самостійно
account-revoke-title = Відкликати «{ $name }»?
account-revoke-body = Відтепер усе, що досі його використовує, отримуватиме відмову.
account-revoke = Відкликати
account-revoke-failed = Не вдалося відкликати «{ $name }»
account-disconnect-title = Відключити { $app }?
account-disconnect-body = Відтепер він отримуватиме відмову, і щоб користуватися Hoard, його доведеться підключити знову.
account-disconnect = Відключити
account-disconnect-failed = Не вдалося відключити { $app }
account-guest-failed = Не вдалося це змінити
account-never = ніколи
account-you = Ви
account-sign-out = Вийти
account-signed-in-as-before = Ви ввійшли як{ " " }
account-signed-in-as-admin = , адміністратор.
account-signed-in-as-edit = , з правом змінювати бібліотеку.
account-signed-in-as-view = , з правом лише переглядати, без змін.
account-app-signin = Застосунок Hoard сам виконує вхід, тому пароля немає. Створіть нижче токен доступу для плагінів рушіїв і команди hoard.
account-password-at-provider = Ваш пароль зберігає постачальник, через якого ви входите, тож змінюйте його там.
account-password-at-directory = Ваш пароль зберігає каталог, через який ви входите, тож змінюйте його там.
account-current-password = Поточний пароль
account-new-password = Новий пароль
account-new-password-again = Новий пароль ще раз
account-change-password = Змінити пароль
account-password-mismatch = Нові паролі не збігаються.
account-password-rule = Щонайменше 8 символів.
account-people = Люди
account-people-hint = Кожна людина може переглядати (пошук, попередній перегляд і завантаження), також змінювати бібліотеку (додавання, редагування, тегування, колекції й архів) або ще й бути адміністратором (налаштування, облікові записи, обслуговування й остаточне видалення).
account-you-tag = (ви)
account-signs-in-oauth = Входить через постачальника OAuth
account-signs-in-directory = Входить через каталог
account-since = з { $date }
account-what-can-do = Права { $name }
account-name = Ім’я
account-how-sign-in = Спосіб входу
account-password-label = Пароль (щонайменше 8 символів)
account-password-placeholder = Пароль (від 8 символів)
account-what-they-can-do = Права
account-add = Додати
account-ai-apps = Claude та інші застосунки ШІ
account-ai-apps-hint = Підключіть Claude (або будь-який MCP-застосунок), і він зможе шукати й отримувати для вас ресурси, а також описувати й позначати тегами те, що Hoard ще не позначив. Під час першого звернення він входить у Hoard; дозвіл ви даєте тут, у браузері.
account-mcp-address = MCP-адреса Hoard
account-copy = Копіювати
account-claude-code-1 = виконайте{ " " }
account-claude-code-2 = , потім{ " " }
account-claude-code-3 = , щоб увійти.{ " " }
account-claude-code-4 = { " " }описує пакет.
account-claude-ai = Settings → Connectors → Add custom connector, вставте адресу, потім Connect. Вони підключаються із серверів Anthropic, тож Hoard має бути доступним з інтернету через HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, вставте адресу, потім Connect. Вони підключаються із серверів Anthropic, тож Hoard має бути доступним з інтернету через HTTPS (ця адреса недоступна: задайте публічну адресу в «Налаштування → Підключення»).
account-connection-meta = для { $user } з { $since } · востаннє використано: { $used }
account-no-apps = Підключених застосунків ще немає.
account-live = Зараз користуються Hoard
account-live-hint = Застосунки, які зверталися до Hoard за останні 10 хвилин. Список оновлюється сам.
account-live-none = За останні 10 хвилин до Hoard ніхто не звертався.
account-live-last = останнє: { $what } · { $when } · запитів: { $calls }
account-tokens = Токени доступу
account-tokens-hint-1 = Для інструмента командного рядка{ " " }
account-tokens-hint-2 = , скриптів, MCP-клієнтів і обробника Unreal: вони надсилають його як{ " " }
account-tokens-hint-3 = . Токен може робити те, що ви виберете, у межах ваших прав, але ніколи не керує обліковими записами чи токенами. Токена з правом лише переглядати досить для пошуку, завантаження й синхронізації проєкту.
account-tokens-admin = Ви бачите токени й підключені застосунки всіх; інші бачать лише свої.
account-made-before = { "" }
account-made-after = : скопіюйте його зараз. Більше він не показуватиметься.
account-new-token = Новий токен
account-cli-1 = Для CLI:{ " " }
account-cli-2 = , або вкажіть його в{ " " }
account-cli-3 = .
account-done = Готово
account-token-meta = створено: { $made } · востаннє використано: { $used }
account-token-meta-by = створено: { $made }, автор: { $by } · востаннє використано: { $used }
account-no-tokens = Токенів ще немає.
account-token-name = Що його використовуватиме
account-token-name-placeholder = Що його використовуватиме, напр. «обробник Unreal»
account-token-role = Права токена
account-make-token = Створити токен
account-guests = Гості
account-guests-allow = Дозволити будь-кому, хто може дістатися до сервера, переглядати без входу
account-guests-hint = Вони можуть шукати, переглядати й завантажувати, але не можуть нічого додавати, змінювати чи видаляти або бачити налаштування.

## SignIn.vue

signin-create-title = Створіть свій обліковий запис
signin-title = Вхід
signin-setup-lede = На цьому сервері ще немає облікових записів. Той, що ви створите тут, може все; облікові записи для інших людей можна додати згодом у «Налаштування → Обліковий запис».
signin-overlay-lede = Переглядати тут може кожен, але для змін потрібен обліковий запис.
signin-with = Увійти через { $provider }
signin-or-password = або за іменем і паролем
signin-username = Ім’я користувача
signin-name = Ім’я
signin-new-password = Новий пароль
signin-password = Пароль
signin-password-rule = Щонайменше 8 символів.
signin-confirm-password = Підтвердіть пароль
signin-mismatch = Паролі не збігаються.
signin-setup-code = Код налаштування
signin-setup-code-hint-before = Код, заданий цьому серверу в{ " " }
signin-setup-code-hint-after = .
signin-creating = Створення…
signin-signing-in = Вхід…
signin-create = Створити обліковий запис
signin-submit = Увійти
signin-keep-browsing = Продовжити перегляд
signin-directory-hint = Використовуйте своє ім’я та пароль { $directory }.
signin-forgot-1 = Забули пароль Hoard? Щоб отримати новий, виконайте на сервері{ " " }
signin-forgot-2 = { " " }(у Docker:{ " " }
signin-forgot-3 = ).

## SetupWizard.vue (first visit to a new server)

setup-title = Де зберігатиметься ваша бібліотека?
setup-lede = Виберіть на сервері теку, де Hoard зберігатиме ваші пакети, або вкажіть теку з пакетами, які у вас уже є. Згодом у налаштуваннях можна додати інші розташування, зокрема на інших дисках.
setup-lede-desktop = Виберіть теку, де Hoard зберігатиме ваші пакети, або вкажіть теку з пакетами, які у вас уже є. Згодом у налаштуваннях можна додати інші розташування, зокрема на інших дисках.
setup-unreachable = Не вдалося зв’язатися із сервером: { $error }
setup-unreachable-desktop = Hoard не відповідає: { $error }
setup-try-again = Спробувати знову
setup-loading = Завантаження…
setup-start = Створити бібліотеку
setup-ready = Ваша бібліотека готова
setup-managed-before = Нові пакети потрапляють у{ " " }
setup-managed-after = . Перетягніть файли, теки чи архіви будь-куди на сторінці, щоб додати перші.
setup-watched-before = Hoard відстежує{ " " }
setup-watched-after = ; пакети з’являтимуться в міру того, як він їх знаходитиме. Щоб також вивантажувати, додайте бібліотеку, яку зберігає Hoard, у «Налаштування → Розташування бібліотеки».
setup-open = Відкрити бібліотеку

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = Підключити { $app }?
connect-lede-1 = { " " }зможе шукати, завантажувати, описувати й позначати тегами все в цьому Hoard від імені{ " " }
connect-lede-2 = . Керувати обліковими записами чи токенами доступу він не зможе.
connect-back-before = Ви повернетеся на{ " " }
connect-back-after = . Відключити його можна будь-коли в «Налаштування → Обліковий запис».
connect-allow = Дозволити
connect-cancel = Скасувати
connect-failed = Не вдається підключити цей застосунок
connect-go-library = Перейти до бібліотеки
connect-checking = Перевірка…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = Це звичайний http://, а деяким функціям потрібен HTTPS: 3D з WebGPU, сучасний буфер обміну, установлення Hoard як застосунку та конектори Claude.
insecure-switch = Перейти на https://
insecure-set-up = Налаштувати HTTPS
insecure-dismiss = Більше не показувати

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Цей файл завеликий, щоб читати його як файл магазину.
formatwizard-not-json = Цей файл не є коректним JSON, тому показаний як текст.
formatwizard-read-failed = Не вдалося прочитати файл: { $error }
formatwizard-back-title = Повернутися до форми?
formatwizard-back-body = Зміни, внесені в TOML, буде втрачено: форма знову запише формат на основі своїх даних.
formatwizard-back-to-form = Назад до форми
formatwizard-miss-name = Його файли не приймають назви { $file }. Додайте її в «Файли».
formatwizard-miss-not-the-stores = Того, за чим він розпізнає файли магазину, у цьому файлі немає. Змініть «Розпізнавати за».
formatwizard-miss-no-title = У цьому файлі він не знаходить назви. Вкажіть назву.
formatwizard-miss-short-name = не та назва
formatwizard-miss-short-not-the-stores = не цього магазину
formatwizard-miss-short-no-title = немає назви
formatwizard-saved-packs =
    { $file } збережено. { $packs ->
        [one] { $packs } пакет читається знову.
        [few] { $packs } пакети читаються знову.
        [many] { $packs } пакетів читаються знову.
       *[other] { $packs } пакета читаються знову.
    }
formatwizard-saved = { $file } збережено.
formatwizard-save-failed = Не вдалося зберегти формат: { $error }
formatwizard-zip-failed = Не вдалося створити zip: { $error }
formatwizard-stop-title = Припинити навчання для цього файлу?
formatwizard-stop-body = Формат не збережено.
formatwizard-stop = Припинити
formatwizard-label = Майстер форматів
formatwizard-changing = Зміна формату { $name }
formatwizard-starting-from = На основі { $name }
formatwizard-title = Навчіть Hoard файлу магазину
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }у{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Редагувати як TOML
formatwizard-file-reference = Файл для довідки.
formatwizard-click-json = Клацніть у файлі на поле «{ $field }».
formatwizard-click-text = Клацніть у файлі на поле «{ $field }» або виділіть його.
formatwizard-pick-first = Виберіть поле праворуч, а потім клацніть тут на його значення.
formatwizard-loading = Завантаження…
formatwizard-the-format = Формат
formatwizard-syntax-before = Синтаксис описано в{ " " }
formatwizard-syntax-after = .
formatwizard-store = Магазин
formatwizard-store-placeholder = Назва магазину, напр. Kenney
formatwizard-files = Файли
formatwizard-recognize = Розпізнавати за
formatwizard-recognize-json = Ключ, який є лише у файлах магазину
formatwizard-recognize-text = Рядок, який є лише у файлах магазину
formatwizard-recognize-hint = По одному варіанту на рядок; підійде будь-який. Потрібно, коли інші файли мають таку саму назву.
formatwizard-seller = Продавець
formatwizard-seller-placeholder = Якщо у файлі не вказано
formatwizard-tags = Теги
formatwizard-tags-placeholder = Для всього, що він описує, через кому
formatwizard-fields = Поля
formatwizard-selector = Селектор поля «{ $field }»
formatwizard-clear = Очистити «{ $field }»
formatwizard-pick = Вибрати
formatwizard-what-it-reads = Що він читає
formatwizard-read-title = Назва
formatwizard-read-seller = Продавець
formatwizard-read-license = Ліцензія
formatwizard-read-url = Сторінка магазину
formatwizard-read-kind = Тип
formatwizard-read-tags = Теги
formatwizard-read-origin = Місце створення
formatwizard-read-size = Розмір
formatwizard-also = { $format } теж читає цей файл. Цей формат пробуватиметься першим.
formatwizard-library =
    Файлів із назвою, яку він приймає: { $files } у { $packs ->
        [one] { $packs } пакеті
        [few] { $packs } пакетах
        [many] { $packs } пакетах
       *[other] { $packs } пакета
    }. З { $tried } перевірених він читає { $read }.
formatwizard-library-none = Жоден файл у бібліотеці не має назви, яку він приймає.
formatwizard-share = Поділитися
formatwizard-share-hint = Zip-архів із форматом, цим файлом як зразком і тим, що зі зразка прочитано, розкладений як у репозиторії Hoard, щоб надіслати його й щоб він дістався всім. Спершу приберіть зі зразка все, чим не слід ділитися.
formatwizard-sample = Зразок для поширення
formatwizard-download = Завантажити { $file }
formatwizard-close = Закрити
formatwizard-use = Використовувати тут
formatwizard-cancel = Скасувати
settings-unreal-editor-what = Встановлює Unreal Editor для процесу експорту. Для експорту ресурсів запустіть його окремо.
settings-unreal-download = Одноразово завантажує редактор Epic для Linux x86-64 у теку інструментів Hoard. Він займає десятки ГБ на диску й потребує облікового запису GitHub, допущеного Epic.
settings-unreal-github-step-link-go = Як їх пов’язати
settings-unreal-github-step-token = Створіть токен GitHub (classic), позначивши лише read:packages.
settings-unreal-github-step-token-go = Створити токен
settings-unreal-github-user = Ім’я користувача GitHub
settings-unreal-github-token = Токен
settings-unreal-github-token-kept = Збережено (введіть новий, щоб замінити його)
settings-unreal-github-save = Зберегти вхід
settings-unreal-github-forget = Відключити
settings-unreal-github-saved = Вхід у GitHub збережено; повторюємо завантаження
settings-unreal-github-forgotten = GitHub відключено
settings-unreal-github-from-server = Цьому серверу також надано власний вхід (GHCR_TOKEN_FILE або DOCKER_CONFIG); він використовується, якщо тут нічого не збережено.
settings-unreal-github-connect = Підключити GitHub…
settings-unreal-github-connect-title = Підключити GitHub
settings-unreal-github-connect-why = Epic відкриває Unreal Editor обліковим записам GitHub, пов’язаним з Epic Games. Hoard просить лише читання пакетів.
settings-unreal-github-sign-in = Увійти через GitHub
settings-unreal-github-enter-code = Після натискання GitHub відкриється в новій вкладці. Введіть там цей код.
settings-unreal-github-waiting = Чекаємо на GitHub…
settings-unreal-github-expired = Термін дії коду минув.
settings-unreal-github-denied = Вхід скасовано на GitHub.
settings-unreal-github-failed = Не вдалося отримати код від GitHub.
settings-unreal-github-again = Отримати новий код
settings-unreal-github-before = Перш ніж почати
settings-unreal-github-before-what = Пов’яжіть GitHub зі своїм обліковим записом Epic Games і прийміть запрошення EpicGames, яке GitHub надішле на пошту.
settings-unreal-github-use-token = Використати токен
settings-unreal-github-use-sign-in = Увійти через GitHub
settings-unreal-github-no-password = Жоден пароль не потрапляє до Hoard
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = Підключено як { $user }; повторюємо завантаження
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = Підключено як { $user }
settings-unreal-github-as-what = Unreal Editor завантажується від імені цього облікового запису GitHub.
settings-unreal-github-none = GitHub не підключено
settings-unreal-github-none-what = Потрібно для завантаження редактора.
settings-unreal-github-change = Змінити…
settings-unreal-image = Версія або образ Unreal
settings-editor-path = Вкажіть шлях до виконуваного файлу встановленого { $tool }. Автоматичне встановлення доступне лише в Docker.
error-unreal-image = Вкажіть версію Unreal 5, тег версії Epic dev/dev-slim або посилання на образ Epic із SHA-256.
error-github-no-app = На цьому сервері немає застосунку GitHub для входу; використайте токен.
error-github-unreachable = Не вдається зв’язатися з GitHub.
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub відхилив вхід: { $reason }
