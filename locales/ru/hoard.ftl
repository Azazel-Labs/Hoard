## Ошибки сервера

error-account-not-found = учётная запись не найдена
error-asset-gone = ассет не найден
error-asset-not-found = ассет { $id } не найден
error-blob-not-found = блоб { $hash } не найден
error-collection-not-found = коллекция { $id } не найдена
error-connection-not-found = подключение не найдено
error-derivative-not-found = производный файл { $kind } для { $id } не найден
error-file-in-pack-not-found = { $path } в паке { $id } не найден
error-site-not-a-page = страницами открываются только веб-страницы
error-site-link-expired = адрес этой страницы устарел; откройте страницу заново из Hoard
error-format-not-found = формат { $file } не найден
error-frame-sheet-aseprite-only = лист кадров есть только у файлов Aseprite
error-frame-sheet-frames = здесь один кадр или слишком много, чтобы их разложить
error-location-not-found = место хранения { $id } не найдено
error-maintenance-task-not-found = задача обслуживания { $id } не найдена
error-open-upload-not-found = открытая загрузка { $id } не найдена
error-original-not-found = исходный файл { $id } не найден
error-pack-not-found = пак { $id } не найден
error-preview-too-big = это изображение слишком большое для облегчённого превью
error-project-not-found = проект { $name } не найден
error-saved-search-not-found = сохранённый поиск { $id } не найден
error-sign-in-first = сначала войдите (или отправьте токен доступа)
error-those-assets-not-found = эти ассеты не найдены
error-thumbnail-not-found = миниатюра для { $id } не найдена
error-token-not-found = токен не найден
error-upload-not-found = загрузка { $id } не найдена
error-pack-needs-name = у пака должно быть название
error-no-library = пока нет места хранения для файлов библиотеки: добавьте управляемое в «Настройки → Места хранения»
error-upload-failed = загрузка не удалась: { $reason }
error-upload-no-files = в загрузке нет файлов
error-analyze-nothing-to-do = нечего делать: rules=false, а LLM выключена
error-too-many-assets = слишком много ассетов в одном запросе
error-change-not-found = этого изменения нет в истории этого ассета
error-bulk-by-look = поиск по виду нельзя изменить целиком: выберите ассеты вручную
error-too-many-items = слишком много элементов в одном запросе
error-not-a-hash = это не шестнадцатеричный хеш blake3
error-merge-into-tag = into должен быть тегом
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = «{ $text }» не является тегом
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } уже под { $tag }, поэтому { $tag } нельзя поместить под него
error-nothing-to-merge = нечего объединять
error-selection-size = укажите от 1 до { $max } id ассетов
error-collection-needs-name = у коллекции должно быть название
error-collection-exists = коллекция «{ $name }» уже есть
error-collection-empty = коллекция «{ $name }» пуста
error-conversion-failed = не удалось преобразовать в { $format }: { $reason }
error-max-size = max должен быть степенью двойки от 64 до 16384
error-replace-ext = ext должен быть коротким расширением файла
error-empty-file = пустой файл
error-imitation-silent = В записи ничего не выделяется на фоне. Попробуйте ещё раз ближе к микрофону.
error-replace-duplicate = преобразованный файл повторяет файл, который уже есть в паке; пакет объединён с ним
error-ai-address = Адрес ИИ должен начинаться с http:// или https://
error-hosted-key = Введите ключ API для { $provider }
error-hosted-address = Адрес сервиса поиска по картинкам должен начинаться с http:// или https://
error-local-llm-action = action — это list, download или delete, и для последних двух нужна модель
error-role = Роль — это view, edit или admin
error-admin-only = это может сделать только администратор
error-view-only = эта учётная запись может только смотреть, но ничего не менять: администратор может это изменить в «Настройки → Учётная запись»
error-portal-only = войдите в портал, чтобы управлять учётными записями и токенами
error-not-from-portal = этот запрос пришёл не из портала Hoard
error-account-name = Выберите имя длиной до 64 символов
error-password-length = Пароль должен быть не короче { $min } символов
error-too-many-codes = Слишком много неверных кодов: подождите { $seconds } с
error-already-set-up = Учётная запись уже есть: войдите
error-too-many-tries = Слишком много неверных попыток: подождите { $seconds } с
error-directory-unavailable = Каталог не смог проверить ваш пароль: { $reason }
error-password-elsewhere = Ваш пароль хранится в каталоге или у провайдера, через которого вы входите
error-current-password = Текущий пароль неверен
error-account-source = Учётная запись входит через Hoard (internal), каталог (ldap) или OAuth
error-account-exists = Учётная запись с таким именем уже есть
error-own-role = Вашу роль может изменить другой администратор
error-remove-self = Нельзя удалить свою учётную запись, пока вы в неё вошли
error-token-name = Назовите токен по тому, что будет его использовать, — до 80 символов
error-token-role = Токен не может делать больше, чем учётная запись, которая его создаёт
error-wrong-setup-code = Это не тот код настройки, что задан серверу в HOARD_SETUP_CODE
error-wrong-password = Имя и пароль не совпадают
error-folder-missing = { $path } не найден (диск не подключён?)
error-s3-path = место хранения S3 выглядит так: s3://bucket
error-s3-address = адрес S3 должен начинаться с http:// или https://
error-s3-header = дополнительный заголовок S3 выглядит как Name=value, а «{ $header }» — нет
error-keychain = не удалось сохранить в системном хранилище паролей: { $reason }
error-not-built = эта копия Hoard собрана без { $feature }
error-openai-auth-unreachable = не удалось связаться с auth.openai.com
error-openai-unexpected = неожиданный ответ от OpenAI
error-openai-answered = OpenAI ответил { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID-токен — не JWT
error-chatgpt-id-token-issuer = ID-токен выдан { $issuer }, а не { $expected }
error-chatgpt-id-token-audience = ID-токен предназначен для другого приложения
error-chatgpt-id-token-expired = срок действия ID-токена истёк
error-chatgpt-id-token-nonce = ID-токен не относится к этому входу
error-chatgpt-no-state = в адресе нет state; скопируйте адрес целиком
error-chatgpt-not-started = этот вход начат не здесь или больше 15 минут назад. Начните его заново в настройках.
error-chatgpt-canceled = вход отменён
error-chatgpt-refused = ChatGPT отклонил вход: { $reason }
error-chatgpt-no-code = в адресе нет кода входа; скопируйте адрес целиком
error-chatgpt-other-registration = ChatGPT ответил для другой регистрации этого сервера; начните вход заново
error-chatgpt-other-registration-desktop = ChatGPT ответил для другой копии Hoard; начните вход заново
error-chatgpt-not-registered = ChatGPT не завершил регистрацию Hoard; начните вход заново
error-chatgpt-code-used = код входа уже использован или устарел; войдите снова
error-chatgpt-refused-code = ChatGPT отклонил вход ({ $code })
error-chatgpt-no-id-token = ChatGPT не прислал ID-токен
error-chatgpt-plan-not-allowed = Вход выполнен, но использовать тариф не разрешено. Войдите снова и разрешите это или используйте ключ API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = вход в ChatGPT не выполнен; войдите в «Настройки → Теги от ИИ»
error-chatgpt-refresh-later = ChatGPT сейчас не смог обновить вход ({ $code }); попробуем позже
error-chatgpt-signed-out-problem = ChatGPT вывел этот сервер из системы ({ $code }). Войдите снова, чтобы и дальше ставить теги по своему тарифу.
error-chatgpt-signed-out-problem-desktop = ChatGPT вывел Hoard из системы ({ $code }). Войдите снова, чтобы и дальше ставить теги по своему тарифу.
error-chatgpt-signed-out = ChatGPT вывел этот сервер из системы ({ $code }); войдите снова в «Настройки → Теги от ИИ»
error-chatgpt-signed-out-desktop = ChatGPT вывел Hoard из системы ({ $code }); войдите снова в «Настройки → Теги от ИИ»
error-openai-api-unreachable = не удалось связаться с api.openai.com
error-openai-broke-off = ответ от OpenAI оборвался
error-chatgpt-kept-refusing = OpenAI раз за разом отклонял вход в ChatGPT; войдите снова в «Настройки → Теги от ИИ»
error-chatgpt-request-failed = запрос не удался
error-chatgpt-cut-off = ответ обрезан ({ $reason })
error-chatgpt-cut-off-no-reason = ответ обрезан (причина не указана)
error-openai-ended-early = ответ от OpenAI закончился, не будучи полным
error-chatgpt-usage-limit = лимит использования вашего тарифа ChatGPT для Hoard пока исчерпан. Посмотреть или изменить лимит можно на { $url }
error-chatgpt-not-eligible = эта учётная запись или рабочее пространство ChatGPT не может использовать свой тариф в других приложениях. Используйте ключ API.
error-chatgpt-usage-unavailable = ChatGPT сейчас не смог проверить использование тарифа; попробуем позже
error-chatgpt-unsupported = тариф ChatGPT не поддерживает { $param } с этой моделью
error-chatgpt-unsupported-images = тариф ChatGPT не поддерживает { $param } с этой моделью; выберите модель, которая принимает изображения, или отключите отправку миниатюр
error-chatgpt-unsupported-part = тариф ChatGPT не поддерживает часть запроса с этой моделью
error-chatgpt-port = port — это порт, по которому браузер обращается к Hoard
error-chatgpt-paste-address = вставьте адрес целиком, начиная с http://127.0.0.1
error-too-many-hashes = не больше { $max } хешей за раз
error-credits-pack = пак { $key }
error-credits-collection = коллекция { $key }
error-format-too-long = формат слишком длинный
error-not-started = сервер не запущен
error-not-started-desktop = Hoard не запущен
error-not-format-file = это не файл формата: { $name }
error-format-name = в названии нужна буква или цифра от A до Z или от 0 до 9
error-format-miss-name = файлы формата не включают { $file }
error-format-miss-store = формат не принимает { $file } для этого магазина (см. require)
error-format-miss-title = формат не находит названия в { $file }
error-cant-download-as = файлы .{ $ext } нельзя скачать в формате { $format }
error-only-images-resize = менять размер можно только у изображений
error-https-no-certificate = Сначала вставьте или загрузите сертификат и его ключ
error-https-no-cert-found = Сертификат не найден: вставьте текст, который начинается с -----BEGIN CERTIFICATE-----
error-https-no-key-found = Закрытый ключ не найден: вставьте текст, который начинается с -----BEGIN PRIVATE KEY-----
error-https-several-keys = Закрытых ключей больше одного: вставьте только тот, что подходит к сертификату
error-https-key-password = Ключ защищён паролем: дайте его без пароля (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Сертификат — не корректный PEM
error-https-key-not-pem = Закрытый ключ — не корректный PEM
error-https-key-mismatch = Ключ не относится к сертификату. Если вы дали цепочку, поставьте сертификат своего сервера первым
error-https-unusable = Сертификат или ключ нельзя использовать
error-https-expired = Срок действия этого сертификата истёк { $date }
error-https-not-yet-valid = Этот сертификат действует только с { $date }
error-https-unreadable = Hoard не может прочитать этот сертификат
error-https-bad-name = «{ $name }» — не тот адрес, который может быть в сертификате
error-https-test-timed-out = время ожидания тестового подключения истекло
error-https-test-failed = тестовое подключение не удалось
error-https-no-ca = Hoard ещё не создал свой центр сертификации
error-https-unavailable = HTTPS на этом сервере недоступен
error-https-certificate-mode = Сертификат указывается с вариантом «Мой сертификат»
error-https-test = Сертификат не сработал при тестовом подключении: { $reason }
error-nothing-to-retry = там нечего повторять
error-ai-unreachable = не удалось связаться с { $url }
error-ai-key-refused = { $url } отклонил ключ API ({ $status })
error-ai-answered = { $url } ответил { $status }: { $reason }
error-ai-not-json = { $url }/models ответил не в формате JSON
error-model-server-refused = сервер моделей: { $reason }
error-no-model-server = по адресу { $url } не отвечает сервер моделей
error-not-llama-router = { $url } — не сервер llama.cpp в режиме router, поэтому он не может скачивать модели
error-model-download = не удалось скачать { $model }: { $reason }
error-model-delete = не удалось удалить { $model }: { $reason }
error-unknown-kind = неизвестный вид ассета { $kind }
error-watched-is-folder = отслеживаемое место хранения — это папка на этом сервере
error-cant-reach = нет доступа к { $path }: { $reason }
error-folder-full-path = укажите полный путь к папке
error-cant-create = не удалось создать { $path }
error-cant-write = не удалось записать в { $path }
error-cant-read = не удалось прочитать { $path }
error-overlaps-location = это пересекается с местом хранения { $name } ({ $path })
error-overlaps-data = это пересекается с папкой данных Hoard ({ $path })
error-overlaps-cache = это пересекается с папкой кеша Hoard ({ $path })
error-not-watched = { $name } — не отслеживаемая папка
error-watched-no-kinds = отслеживаемая папка не принимает новые файлы, так что видов для изменения у неё нет
error-keep-catch-all = сделайте местом для всего остального другое место хранения
error-location-holds-files = в { $name } ещё есть файлы: сначала перенесите их в другие места хранения (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } недоступно, поэтому Hoard не может определить, остались ли там файлы
error-relocate-running = файлы уже переносятся между местами хранения; попробуйте снова, когда это закончится
error-watched-not-moved = { $name } — отслеживаемая папка: Hoard не переносит её файлы
error-no-other-location = нет другого места хранения, куда перенести файлы: сначала добавьте его
error-full-path = укажите полный путь
error-pick-folder = выберите папку
error-task-running = «{ $task }» уже выполняется
error-job-cant-stop = Задачу «{ $job }» нельзя остановить после запуска
error-job-not-found = эта задача уже не ждёт в очереди и не выполняется
error-task-cant-stop = Задачу «{ $task }» нельзя остановить на полпути
error-task-not-running = Задача «{ $task }» не выполняется
error-token-account-gone = учётной записи этого токена больше нет, поэтому у него нет избранного и поисков
error-favorites-too-many = не больше 10 000 за раз
error-saved-search-name = у сохранённого поиска должно быть название не длиннее 80 символов
error-saved-search-too-long = этот поиск слишком длинный, чтобы его сохранить
error-hoard-seen = клад может хранить не более 32 видов ассетов, каждый с названием из строчных букв
error-texture-pattern-map = В шаблоне названия текстуры нужен { "{" }map{ "}" }, иначе все карты материала получат одно и то же название
error-texture-pattern-fields = В шаблоне названия текстуры можно использовать { "{" }name{ "}" }, { "{" }map{ "}" } и { "{" }size{ "}" }
error-texture-name-chars = В названиях текстур не может быть < > : " / \ | ? или *
error-texture-map-type = Типа карты { $role } нет
server-role-base-color = цвета
server-role-normal = нормалей
server-role-normal-dx = нормалей (DirectX)
server-role-roughness = шероховатости
server-role-gloss = глянца
server-role-metalness = металличности
server-role-ao = фонового затенения
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = высот
server-role-bump = рельефа
server-role-cavity = впадин
server-role-specular = отражения
server-role-opacity = непрозрачности
server-role-emissive = свечения
server-role-translucency = просвечивания
error-texture-suffix-missing = Задайте суффикс для карт { $map }
error-texture-suffix-chars = В суффиксе для карт { $map } не может быть < > : " / \ | ? или *
error-texture-ending-missing = Для каждого своего окончания нужно указать, какое окончание искать
error-texture-ending-chars = В своих окончаниях не может быть < > : " / \ | ? или *
error-texture-ending-twice = { $ending } встречается в ваших окончаниях дважды
error-oauth-no-client-id = В этой ссылке нет client_id.
error-oauth-unknown-app = Это приложение здесь не зарегистрировано (больше не зарегистрировано). Начните подключение заново из приложения.
error-oauth-redirect = Приложение попросило вернуть его на адрес, который оно не регистрировало.
error-oauth-response-type = Hoard выдаёт только коды входа (response_type=code).
error-oauth-pkce = Чтобы войти в Hoard, приложение должно использовать PKCE с S256.
error-light-preview-images = облегчённое превью есть только у изображений
error-no-light-preview = нет облегчённого превью: { $reason }
error-no-frame-sheet = нет листа кадров: { $reason }
error-thumbnail-source = неизвестный источник миниатюры: { $source }
error-thumbnail-too-big = эта картинка слишком большая для миниатюры
error-thumbnail-models-only = миниатюру, нарисованную браузером, принимают только модели
error-not-a-picture = Hoard не может прочитать эту картинку: { $reason }
error-project-name = у проекта должно быть название не длиннее 200 символов
error-project-too-big = не больше миллиона ассетов в одном проекте
error-project-too-many-at-once = не больше 100 000 за раз
error-server-name = Выберите имя длиной до { $max } символов, в одну строку
error-tool-setting = { $tool }: выберите «выкл.», «автоматически» или укажите путь
error-ai-model = Выберите модель, которую должен использовать ИИ-сервис
error-public-address = Публичный адрес должен начинаться с http:// или https://
error-update-hook-address = Адрес для вызова при обновлениях должен начинаться с http:// или https://
error-update-hook-auth = В заголовке Authorization не может быть переносов строк и других управляющих символов
error-no-soundfont = На сервере нет SoundFont по пути { $path }
error-no-soundfont-desktop = Нет SoundFont по пути { $path }
error-soundfont-bad = { $name } — не SoundFont, с которым может играть MIDI: { $reason }
error-workers = Фоновые обработчики: от 1 до { $max }
error-ai-batch = Ассетов в одном запросе к ИИ: от 1 до 200
error-conversion-cache = Преобразованные загрузки: не меньше 100 МБ
error-backup-keep = Резервные копии по расписанию: хранить от 1 до { $max }
error-backup-copy-to = Резервные копии по расписанию: { $path } — не папка на сервере (укажите полный путь)
error-conversion-keep-days = Правила преобразования: оригиналы можно хранить от 1 до { $max } дней
error-conversion-target = Правила преобразования: Hoard не умеет преобразовывать файлы в { $format }
error-conversion-no-source = Правила преобразования: в каждом правиле нужен хотя бы один формат для преобразования
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Правила преобразования: файлы .{ $ext } нельзя преобразовать в { $format }
error-conversion-twice = Правила преобразования: .{ $ext } есть в двух правилах
error-conversion-cant-undo = Этот файл не был преобразован правилом, или его оригинал уже удалён.
error-conversion-path-taken = По пути { $path } теперь лежит другой файл, поэтому оригинал нельзя туда вернуть.
error-limits-zero = Ограничения не могут быть нулевыми
error-settings-object = ожидался JSON-объект с настройками
error-bad-setting = неверная настройка: { $reason }
error-auth-default-role = Роль по умолчанию — это view, edit или admin
error-auth-session-hours = Сеансы длятся от 1 часа до года
error-auth-settings = неверные настройки аутентификации: { $reason }
error-ldap-settings = неверные настройки LDAP: { $reason }
error-ldap-no-server = Добавьте адрес сервера каталога
error-ldap-base-dn = Укажите, где находятся записи пользователей (base DN)
error-ldap-username-attribute = Укажите, в каком атрибуте хранится имя для входа
error-ldap-bind-dn = Укажите учётную запись для поиска пользователей или ищите их анонимно
error-ldap-ca-cert = Центр сертификации — не сертификат PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Адрес сервера каталога пуст
error-ldap-server-form = { $server }: адрес сервера каталога — это host:port, ldap://… или ldaps://…
error-ldap-not-starttls = { $server }: это LDAPS, а не StartTLS
error-ldap-no-certificate = в нём нет сертификата
error-ldap-password-again = Снова укажите пароль учётной записи для поиска пользователей: сохранённый отправляется только на те серверы, для которых он был указан
error-ldap-unreachable = не удалось связаться ни с одним сервером каталога ({ $reasons })
error-ldap-no-anonymous = каталог не разрешает анонимный поиск пользователей ({ $reason })
error-ldap-bind-failed = учётная запись для поиска пользователей не смогла войти ({ $reason })
error-ldap-search = поиск { $name } в { $base }
error-ldap-group-search = поиск групп { $name } в { $base }
error-ldap-several =
    { $count ->
        [one] { $count } запись в каталоге соответствует { $name }: сузьте фильтр пользователей
        [few] { $count } записи в каталоге соответствуют { $name }: сузьте фильтр пользователей
        [many] { $count } записей в каталоге соответствуют { $name }: сузьте фильтр пользователей
       *[other] { $count } записи в каталоге соответствуют { $name }: сузьте фильтр пользователей
    }
error-ldap-password-check = каталог не стал проверять пароль { $name } ({ $reason })
error-ldap-wrong-password = Каталог не знает { $name } с таким паролем
error-oauth-client-id = Укажите client ID, который провайдер создал для Hoard
error-oauth-addresses = Адреса авторизации и токенов должны начинаться с https://
error-oauth-userinfo-address = Адрес сведений о пользователе должен начинаться с https://
error-oauth-username-claim = Укажите, в каком claim хранится имя пользователя
error-oauth-secret-again = Снова укажите client secret: сохранённый отправляется только на тот адрес токенов, для которого он был указан
error-oauth-issuer = Issuer должен начинаться с https://
error-oauth-discovery-unreachable = Не удалось связаться с { $url }: { $reason }
error-oauth-discovery-status = { $url } ответил { $status }
error-oauth-discovery-bad = { $url } — не документ обнаружения (discovery)
error-oauth-discovery-incomplete = { $url } не указывает адреса авторизации и токенов
error-signin-no-group = { $name } не состоит в группе, которой разрешено пользоваться Hoard
error-signin-other-source = Учётная запись Hoard с именем { $name } уже есть и входит другим способом
error-signin-other-subject = Учётная запись Hoard с именем { $name } уже есть, но у провайдера она принадлежит другому человеку
error-signin-no-account = У { $name } пока нет учётной записи Hoard: администратор может добавить её в «Настройки → Учётная запись»
error-signin-bad-name = Такое имя не может быть учётной записью Hoard (до 64 символов)
error-oauth-off = Вход через OAuth не включён
error-oauth-bad-authorization-address = адрес авторизации неверен
error-oauth-refused = Провайдер не выполнил вход: { $reason }
error-oauth-no-code = Провайдер вернул вас без входа
error-oauth-expired = Вход занял слишком много времени или был начат в другом месте: попробуйте снова
error-oauth-no-claim = Провайдер не сообщил, кто вы (нет claim { $claim })
error-oauth-no-sub = Провайдер не сообщил, кто вы (нет claim sub или id)
error-oauth-token-unreachable = не удалось связаться с адресом токенов провайдера
error-oauth-token-refused = Провайдер не стал завершать вход: { $reason }
error-oauth-no-access-token = провайдер не прислал токен доступа
error-oauth-userinfo-unreachable = не удалось связаться с адресом сведений о пользователе у провайдера
error-oauth-userinfo-refused = Провайдер не сообщил, кто вошёл ({ $status })
error-oauth-userinfo-not-json = сведения о пользователе от провайдера — не JSON
error-format-no-name = name пусто
error-format-no-files = files пусто: укажите, какие файлы читает формат, например ["*.json"]
error-format-empty-selector = пусто
error-format-unopened-bracket = закрывающая скобка без открывающей
error-format-unclosed-bracket = скобка не закрыта
error-format-empty-step-dots = пустой шаг (две точки подряд или точка в начале или в конце)
error-format-unclosed-except = !( не закрыта
error-format-unclosed-filter = [ не закрыта
error-format-after-filters = { $rest } после ключа и фильтров { $step }
error-format-except-needs-glob = !(…) ставится после ключа с * в нём
error-format-except-not-glob = !(…) ставится после ключа с * в нём, а не после { $key }
error-format-empty-step = пустой шаг
error-format-not-a-filter = [{ $filter }] — не фильтр: используйте [], [key=value|other], [array], [string], [number], [bool] или [object]
error-format-filter-needs-value = [{ $filter }] нужны ключ и значение
error-store-search-empty = нечего искать; введите название пака
error-update-hook-not-set = сначала задайте адрес для вызова
error-upload-stalled = загрузка { $path } застряла; отправьте эту часть снова
error-upload-busy = { $path } уже записывается
error-upload-incomplete =
    { $count ->
        [one] { $count } файл не загружен до конца, например { $files }
        [few] { $count } файла не загружены до конца, например { $files }
        [many] { $count } файлов не загружены до конца, например { $files }
       *[other] { $count } файла не загружены до конца, например { $files }
    }
error-upload-no-files-listed = нет файлов
error-upload-not-part = { $path } не входит в загрузку { $id }, или загрузка закрыта
error-bad-path = неверный путь
error-upload-part-offset = { $offset } — не начало части { $path }
error-upload-interrupted = загрузка прервана: { $reason }
error-upload-part-long = часть { $part } файла { $path } длиннее { $len } байт
error-upload-part-short = часть { $part } файла { $path }: получено { $got } байт, ожидалось { $len }
error-upload-file-long = { $path } больше заявленного размера
error-upload-file-short = { $path }: получено { $got } байт, ожидалось { $len }
error-upload-complete = загрузка уже завершена
error-already-imported = уже импортирован как «{ $name }»

## Текст сервера

server-store-place-readme = README
server-store-place-folders = папки
server-store-why-linked = Ссылка есть в паке ({ $place })
server-store-why-same-title = То же название, что в паке ({ $place })
server-store-why-by = Автор — { $author }, он указан в паке ({ $place })
server-store-why-meshes = Примерно столько же мешей, сколько в паке ({ $count })
server-store-why-textures = Примерно столько же текстур, сколько в паке ({ $count })
server-chatgpt-revoke-failed = Здесь выход выполнен, но сообщить об этом OpenAI не удалось. Для надёжности отключите Hoard в ChatGPT: Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Вход в ChatGPT выполнен
server-chatgpt-signed-in = Hoard вошёл в систему. Теперь теги от ИИ могут использовать ваш тариф ChatGPT.
server-chatgpt-signed-in-as = Hoard вошёл как { $email }. Теперь теги от ИИ могут использовать ваш тариф ChatGPT.
server-chatgpt-not-finished-title = Вход не завершён
server-chatgpt-other-computer = Если вы начали вход из Hoard на другом компьютере, скопируйте адрес этой страницы из адресной строки и вставьте его в настройки Hoard там.
server-back-to-settings = Назад в настройки
server-issues-pack = Паки, которые не удалось добавить
server-issues-pack-blurb = Удалите пак и добавьте его снова или откройте, чтобы посмотреть, что в нём есть.
server-issues-damaged = Повреждённые или пропавшие файлы в хранилище
server-issues-damaged-blurb = Найдены задачей «Проверить файлы на повреждения». Восстановите их из резервной копии или снова добавьте их пак, затем ещё раз запустите «Проверить файлы на повреждения».
server-issues-folder = Отслеживаемые папки, которые не удаётся прочитать
server-issues-folder-blurb = Их паки остаются как есть, пока папку снова не удастся прочитать.
server-issues-missing = Отслеживаемые паки, чья папка пропала
server-issues-missing-blurb = Каждый пак хранится 3 дня на случай, если его папка просто недоступна, а затем удаляется.
server-issues-job = Фоновые задачи, которые завершились ошибкой
server-issues-job-blurb = Повтор запускает задачу заново с самого начала.
server-issues-file = Файлы, которые не удалось открыть
server-issues-file-blurb = Hoard не смог их открыть или сохранить. Отслеживаемый пак попробует снова при следующем сканировании; файл, который уже был в библиотеке, сохранил прежнее содержимое.
server-issues-unpack = Архивы, которые не удалось распаковать
server-issues-unpack-blurb = Каждый хранится в своём паке как обычный файл, так что ничего из его содержимого в библиотеке нет.
server-issues-read = Файлы, в которых Hoard не разобрался
server-issues-read-blurb = Они есть в библиотеке, но без сведений, тегов и миниатюры, которые даёт их чтение.
server-issues-render = Модели, на которых сломался рендер миниатюр
server-issues-render-blurb = У них нет миниатюры. Открыть такую модель в просмотрщике всё же может получиться, и там можно сохранить миниатюру.
server-issues-convert = Модели, которые Blender не смог преобразовать
server-issues-convert-blurb = У них нет копии в GLB, поэтому их нельзя показать в 3D.
server-issues-unreal = Пакеты Unreal, которые не удалось преобразовать
server-issues-unreal-blurb = Остальные пакеты пака есть на его странице Unreal.
server-issues-reformat = Файлы, которые правило преобразования не смогло преобразовать
server-issues-reformat-blurb = Они оставлены как были.
server-issues-materials = Паки, в которых не удалось определить материалы
server-issues-materials-blurb = Все их текстуры на месте, просто они не сгруппированы в материалы.
server-issues-material-thumb = Материалы без миниатюры
server-issues-material-thumb-blurb = Материал найден, но его превью не удалось нарисовать.
server-issues-store-files = Файлы магазинов, которые не удалось прочитать
server-issues-store-files-blurb = Файлы из магазина, откуда пришёл пак (например, JSON от Megascans), которые дают ему название и теги.
server-issues-scan-again = Сканировать снова
server-issues-retry = Повторить
server-issues-reanalyze = Проанализировать пак заново
server-issues-convert-again = Преобразовать снова
server-issues-look-again = Поискать снова
server-issues-read-again = Прочитать снова
server-issues-blob-missing = Сохранённый файл пропал.
server-issues-blob-damaged = Сохранённый файл не совпадает с тем, что был сохранён: он повреждён.
server-issues-folder-gone = Папка «{ $folder }» не найдена.
server-job-a-pack = пак
server-job-ingest = Импорт: { $pack }
server-job-retag = Повторная расстановка тегов: { $pack }
server-job-llm-tag = Теги от ИИ: { $pack }
server-job-convert = Преобразование в Blender: { $pack }
server-job-unreal = Преобразование пакетов Unreal: { $pack }
server-job-reformat = Преобразование файлов по правилам преобразования: { $pack }
server-job-scan = Сканирование отслеживаемой папки
server-job-relocate = Перенос файлов между местами хранения
server-job-looks = Определение цветов и внешнего вида
server-job-sounds = Прослушивание звуков
server-job-embed = Распознавание того, что на картинках
server-job-hear = Распознавание того, как звучат звуки
server-job-materials = Поиск материалов: { $pack }
server-job-read-files = Чтение файлов: { $pack }
server-job-store-lookup = Поиск страницы в магазине
server-maintenance-backup = Создать резервную копию базы данных
server-maintenance-backup-blurb = Целостная копия базы данных (паки, теги, заметки, коллекции и настройки) в папке backups рядом с ней. Файлов библиотеки в ней нет: после сохранения они никогда не меняются, так что их резервирует любая синхронизация файлов.
server-maintenance-scrub = Проверить файлы на повреждения
server-maintenance-scrub-blurb = Перечитывает каждый сохранённый файл и сверяет его с отпечатком, чтобы найти файлы, которые испортились или пропали с диска или NAS. На большой библиотеке занимает время.
server-maintenance-reanalyze = Проанализировать все паки заново
server-maintenance-reanalyze-blurb = Заново читает каждый файл этой версией Hoard: миниатюры, превью, сведения и теги по правилам создаются заново. Ваши правки остаются; ИИ повторно не спрашивается.
server-maintenance-clear-cache = Очистить кеш преобразований
server-maintenance-clear-cache-blurb = Удаляет файлы, оставшиеся от преобразований «Скачать как» (они создадутся снова при следующем запросе), и то, что осталось от прерванной работы.
server-maintenance-shrink-thumbs = Сжать миниатюры
server-maintenance-shrink-thumbs-blurb = Сохраняет миниатюры и виды моделей, хранящиеся в PNG, так, как задано в «Настройки → Производительность → Миниатюры», — обычно это уменьшает их в несколько раз. Миниатюры с прозрачностью и пиксель-арт остаются в PNG.
server-maintenance-avif-thumbs = Преобразовать миниатюры в AVIF
server-maintenance-avif-thumbs-blurb = Преобразует все миниатюры сейчас, а не каждую при первом показе, когда миниатюры отправляются в AVIF.
server-maintenance-gc = Удалить неиспользуемые файлы
server-maintenance-gc-blurb = Удаляет сохранённые файлы, на которые больше ничто не ссылается, например файлы паков, удалённых навсегда.
server-update-no-address = адрес для вызова не задан
server-update-no-answer = нет ответа в течение 30 секунд

## Настройки: оформление

appearance-language = Язык
appearance-language-auto = Как в браузере ({ $name })

## Корень

root-unreachable = Не удалось связаться с сервером Hoard: { $error }
root-unreachable-desktop = Hoard не отвечает: { $error }
root-desktop-reopen = В этом окне выполнен выход. Закройте Hoard и откройте его снова, чтобы продолжить.
root-try-again = Попробовать снова
root-lazy-connect = подключение приложения
window-minimize = Свернуть
window-maximize = Развернуть
window-restore = Свернуть в окно
window-close = Закрыть

## Приложение: библиотека, верхняя панель и фильтры

app-lazy-settings = настройки
app-lazy-archive = архив
app-lazy-shortcuts = сочетания клавиш
app-lazy-setup = начальная настройка
app-theme-system = Как в системе
app-theme-light = Светлая тема
app-theme-dark = Тёмная тема
app-color-red = Красный
app-color-orange = Оранжевый
app-color-yellow = Жёлтый
app-color-green = Зелёный
app-color-teal = Бирюзовый
app-color-blue = Синий
app-color-purple = Фиолетовый
app-color-pink = Розовый
app-color-brown = Коричневый
app-color-sand = Песочный
app-color-gray = Серый
app-color-slate = Сланцевый
app-color-black = Чёрный
app-color-white = Белый
app-color-swatch = { $color }: ассеты, в которых много этого цвета
app-color-any = Любой цвет
app-color-pick = Выбрать любой цвет
app-color = Цвет
app-music = Музыка
app-tempo = BPM
app-tempo-min = от
app-tempo-max = до
app-tempo-min-title = Музыка в этом темпе или быстрее, в ударах в минуту (вдвое медленнее или быстрее тоже подходит)
app-tempo-max-title = Музыка в этом темпе или медленнее, в ударах в минуту
app-key = Тональность
app-key-any = Любая тональность
app-key-major = Мажор
app-key-minor = Минор
app-key-mix = И тональности, которые с ней сводятся
app-key-mix-title = Её параллельная мажорная или минорная тональность и тональности на квинту выше и ниже, с которыми её сводят диджеи
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Напишите звук так, как произнесли бы его по буквам: похожие звуки будут первыми
app-imitation-record = Изобразите звук в микрофон: похожие звуки будут первыми
app-imitation-stop = Остановить запись
app-imitation-heard = Ваш звук
app-imitation-mic-failed = Не удалось записать с микрофона: { $error }
app-retry = Повторить
app-undo = Отменить
app-search-failed = Поиск не удался: { $error }
app-unreachable = Не удалось связаться с сервером: { $error }
app-unreachable-desktop = Hoard не отвечает: { $error }
app-forget-project-title = Забыть { $name }?
app-forget-project-body =
    { $count ->
        [one] Hoard перестанет показывать его там, где используется его { $count } ассет, пока следующий hoard sync снова о нём не сообщит. В самом проекте ничего не изменится.
        [few] Hoard перестанет показывать его там, где используются его { $count } ассета, пока следующий hoard sync снова о нём не сообщит. В самом проекте ничего не изменится.
        [many] Hoard перестанет показывать его там, где используются его { $count } ассетов, пока следующий hoard sync снова о нём не сообщит. В самом проекте ничего не изменится.
       *[other] Hoard перестанет показывать его там, где используются его { $count } ассета, пока следующий hoard sync снова о нём не сообщит. В самом проекте ничего не изменится.
    }
app-forget-project-confirm = Забыть
app-forget-project-failed = Не удалось забыть { $name }: { $error }
app-forget-project-label = Забыть проект { $name }
app-forget-project = Забыть этот проект
app-save-name-like = похожие на { $name }
app-save-name-favorites = избранное
app-search-saved = Поиск «{ $name }» сохранён: он в фильтрах, в разделе «Ваше»
app-search-updated = Поиск «{ $name }» обновлён
app-rank-alike = Самые похожие
app-rank-color = Больше всего этого цвета
app-rank-match = Лучшее совпадение
app-rank-imitation = Больше всего похожие
app-sort = Сортировка
app-sort-name = По названию
app-sort-created = По дате создания
app-sort-updated = По дате изменения
app-sort-size = По размеру
app-sort-rating = Оценка
app-sort-newest-first = Сначала новые
app-sort-oldest-first = Сначала старые
app-sort-a-to-z = От А до Я
app-sort-z-to-a = От Я до А
app-sort-largest-first = Сначала большие
app-sort-smallest-first = Сначала маленькие
app-sort-best-rated-first = Сначала с высокой оценкой
app-sort-lowest-rated-first = Сначала с низкой оценкой
app-collection-added =
    { $count ->
        [one] В «{ $name }» добавлен { $count } ассет
        [few] В «{ $name }» добавлено { $count } ассета
        [many] В «{ $name }» добавлено { $count } ассетов
       *[other] В «{ $name }» добавлено { $count } ассета
    }
app-collection-add-failed = Не удалось добавить в коллекцию: { $error }
app-collection-removed = Убрано из «{ $name }»: { $count }
app-collection-remove-failed = Не удалось убрать из коллекции: { $error }
app-ns-subject = Объект
app-ns-style = Стиль
app-ns-theme = Тематика
app-ns-material = Материал
app-ns-map = Карта PBR
app-ns-color = Цвет
app-ns-mood = Настроение
app-ns-genre = Жанр
app-ns-license = Лицензия
app-ns-res = Разрешение
app-ns-duration = Длительность
app-ns-poly = Число полигонов
app-ns-format = Формат
app-ns-weight = Объём
app-ns-tech = Технические
app-ns-blender = Версия Blender
app-ns-depth = Разрядность
app-ns-ucs = Категория звука (UCS)
app-ns-bpm = Темп
app-ns-root = Основной тон
app-ns-level = Громкость
app-ns-ue = Класс Unreal
app-ns-engine = Движок
app-ns-extracted = Восстановленный исходник
app-ns-script = Письменность
app-bulk-failed =
    { $count ->
        [one] Не удалось изменить этот ассет: { $error }
        [few] Не удалось изменить эти { $count } ассета: { $error }
        [many] Не удалось изменить эти { $count } ассетов: { $error }
       *[other] Не удалось изменить эти { $count } ассета: { $error }
    }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] Скопирован { $count } тег
        [few] Скопировано { $count } тега
        [many] Скопировано { $count } тегов
       *[other] Скопировано { $count } тега
    }
app-tags-copy-none = У { $name } нет тегов для копирования
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Теги не скопированы. Нажмите C на ассете, чтобы скопировать его теги.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [1] Эти теги у него уже есть
       *[other] Эти теги уже есть у всех { $count }
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [1]
            { $count ->
                [one] Добавлен { $count } тег
                [few] Добавлено { $count } тега
                [many] Добавлено { $count } тегов
               *[other] Добавлено { $count } тега
            }
       *[other]
            { $count ->
                [one] Добавлен { $count } тег к { $assets } ассетам
                [few] Добавлено { $count } тега к { $assets } ассетам
                [many] Добавлено { $count } тегов к { $assets } ассетам
               *[other] Добавлено { $count } тега к { $assets } ассетам
            }
    }
app-bulk-untagged =
    { $count ->
        [one] Тег { $tag } снят с { $count } ассета
        [few] Тег { $tag } снят с { $count } ассетов
        [many] Тег { $tag } снят с { $count } ассетов
       *[other] Тег { $tag } снят с { $count } ассета
    }
app-bulk-tagged =
    { $count ->
        [one] Тег { $tag } добавлен { $count } ассету
        [few] Тег { $tag } добавлен { $count } ассетам
        [many] Тег { $tag } добавлен { $count } ассетам
       *[other] Тег { $tag } добавлен { $count } ассета
    }
app-bulk-review = Отметить просмотренными
app-bulk-review-title = Отметить выбранные ассеты как просмотренные: после этого правила и ИИ не трогают их теги
app-bulk-unreview = Снять отметку
app-bulk-unreview-title = Снять отметку «просмотрено» с выбранных ассетов
app-bulk-reviewed =
    Отмечено просмотренными: { $count ->
        [one] { $count } ассет
        [few] { $count } ассета
        [many] { $count } ассетов
       *[other] { $count } ассета
    }
app-bulk-unreviewed =
    Отметка «просмотрено» снята у { $count ->
        [one] { $count } ассета
        [few] { $count } ассетов
        [many] { $count } ассетов
       *[other] { $count } ассетов
    }
app-bulk-accept = Оставить догадки
app-bulk-accept-title = Оставить как свои все теги, которые правила и ИИ поставили выбранным ассетам
app-bulk-accepted =
    Догадки оставлены у { $count ->
        [one] { $count } ассета
        [few] { $count } ассетов
        [many] { $count } ассетов
       *[other] { $count } ассетов
    }
app-credits-failed = Не удалось составить список авторов: { $error }
app-archived-assets =
    { $count ->
        [one] В архив отправлен { $count } ассет
        [few] В архив отправлено { $count } ассета
        [many] В архив отправлено { $count } ассетов
       *[other] В архив отправлено { $count } ассета
    }
app-archived-one = { $name } отправлен в архив
app-open-failed = Не удалось открыть этот ассет: { $error }
app-unnamed-pack = пак
app-unnamed-collection = коллекция
app-unnamed-asset = ассет
app-home-version = Hoard { $version } — назад в библиотеку
app-home = Назад в библиотеку
app-search-label = Поиск ассетов
app-search-placeholder = Поиск по названиям, путям, тегам
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = скала or камень: одно из двух · -мох: исключить · "сухой мох": эти слова вместе
app-views = Виды
app-library = Библиотека
app-packs = Паки
app-packs-working = в работе: { $count }
app-update-title = Вышел Hoard { $version } (на этом сервере { $current }). Скачайте новый образ или бинарный файл и перезапустите; база данных обновится сама.
app-update = Обновить
app-theme = Тема
app-sign-in = Войти
app-archive-title = Архив: { $contents }
app-archive = Архив
app-settings = Настройки
app-palette-title = Палитра команд ({ $keys })
app-import-folder = Импортировать папку
app-add-files = Добавить файлы
app-add-files-short = Добавить
app-split-label = Как импортировать эту папку
app-split-before = { "" }
app-split-after =
    { $count ->
        [one] { " " }содержит { $count } папку или архив.
        [few] { " " }содержит { $count } папки или архива.
        [many] { " " }содержит { $count } папок или архивов.
       *[other] { " " }содержит { $count } папки или архива.
    }
app-split-each = Импортировать каждый элемент как отдельный пак ({ $count })
app-split-one = Импортировать как один пак
app-cancel = Отмена
app-filters = Фильтры
app-done = Готово
app-yours = Ваше
app-favorites = Избранное
app-needs-review = Не просмотрено
app-needs-review-title = Только ассеты, которые ещё никто не просматривал (сочетается с другими фильтрами)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Оценено
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } звезда и выше
        [few] { $count } звезды и выше
        [many] { $count } звёзд и выше
       *[other] { $count } звезды и выше
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Нажмите ещё раз, чтобы показать все оценки
app-forget-search-label = Забыть сохранённый поиск { $name }
app-forget-search = Забыть этот поиск
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Открывать библиотеку с этим поиском
app-default-search-on-title = Библиотека открывается с этим поиском. Нажмите, чтобы отключить.
app-default-search-label = Открывать библиотеку с поиском { $name }
app-rename-search = Переименовать
app-rename-search-label = Новое название для { $name }
app-projects-title = Проекты, которые выполняют hoard sync с этого сервера, и что было в их последней синхронизации
app-projects = Проекты
app-project-synced = Синхронизирован { $when }
app-project-synced-by = Синхронизирован { $when }, пользователь { $who }
app-collections = Коллекции
app-no-filters = Для этого поиска нет фильтров.
app-kind = Вид
app-facets-more = ещё { $count }
app-facets-fewer = меньше
app-searching = Поиск…
app-searching-live = Идёт поиск
app-asset-count =
    { $count ->
        [one] { $count } ассет
        [few] { $count } ассета
        [many] { $count } ассетов
       *[other] { $count } ассета
    }
app-clear-all = Очистить всё
app-save-name-label = Название для этого поиска
app-save-name-placeholder = Назовите его
app-save = Сохранить
app-save-search-title = Сохранить этот поиск в фильтрах, в разделе «Ваше»
app-save-search = Сохранить поиск
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Обновить «{ $name }»
app-update-search-title = Сохранить текущий выбор в этом сохранённом поиске
app-save-search-new = Сохранить как новый
app-select-title = Выберите несколько ассетов, чтобы вместе добавить им теги, категорию, собрать в коллекцию, скачать или отправить в архив (X или Ctrl+щелчок)
app-select = Выбрать
app-tile-size = Размер плиток
app-supporting-title = Буферы glTF, библиотеки материалов, карты материалов, материалы, которые носит только одна модель, и файлы магазинов с описанием моделей. Они скачиваются вместе с тем, что их использует. Страницы спрайт-листа и его спрайты, которые есть в наборе и отдельными картинками, показываются вместе с листом. Ресурс, который есть в наборе в нескольких форматах, показывается один раз, в формате, который Hoard читает лучше всего.
app-supporting-hidden =
    { $count ->
        [one] Скрыт { $count } вспомогательный файл
        [few] Скрыто { $count } вспомогательных файла
        [many] Скрыто { $count } вспомогательных файлов
       *[other] Скрыто { $count } вспомогательного файла
    }
app-supporting-show = Показать
app-supporting-shown = Со вспомогательными файлами
app-supporting-hide = Скрыть
app-folders = Папки
app-folders-title = Смотреть набор так, как он лежал на диске, папка за папкой
app-folders-label = Папки набора
app-welcome-title = Перетащите пак ассетов в любое место этой страницы
app-welcome-before = Подойдут zip, tar-архивы, папки или отдельные файлы. Hoard распакует их, сохранит каждый файл один раз по его хешу, прочитает размеры, длительность и число полигонов и расставит теги, чтобы вы могли искать что-нибудь вроде{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }или{ " " }
app-welcome-after = .
app-welcome-import-folder = Импортировать папку
app-welcome-choose-files = Выбрать файлы
app-none-similar = Пока нет ничего похожего на { $name }.
app-none = Ничего не найдено.
app-none-query = По запросу «{ $query }» ничего не найдено.
app-none-imitation = Ни один измеренный звук пока не похож на «{ $word }».
app-none-heard = Ни один измеренный звук пока не похож на ваш.
app-none-filters = С этими фильтрами ничего не найдено.
app-none-query-filters = По запросу «{ $query }» с этими фильтрами ничего не найдено.
app-clear-search-filters = Сбросить поиск и фильтры
app-loading-more = Загружаем ещё
app-keys-hint = Стрелки — перемещение, Enter — открыть, пробел — проиграть звук, X или Ctrl+щелчок — выбрать несколько (на сенсорном экране — долгое нажатие).
app-resize-label = Изменить ширину панели сведений
app-resize-title = Перетащите, чтобы изменить ширину · двойной щелчок — подходящая ширина
app-selected-label = Выбранные ассеты
app-selected = Выбрано: { $count }
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Изменить…
app-edit-selected-title = Изменить теги и поля у всех (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] Выбран { $count } результат
        [few] Выбрано { $count } результата
        [many] Выбрано { $count } результатов
       *[other] Выбрано { $count } результата
    }
app-pick-assets = Выберите ассеты
app-download-zip = Скачать zip
app-credits-title = CREDITS.md для паков, из которых они взяты: кто их сделал, их лицензии и строки благодарности, которые они просят указать
app-credits = Авторы
app-prefer-sources-title = SoundWave и текстуры Unreal, исходный файл которых Hoard восстановил
app-prefer-sources = Восстановленные WAV/PNG вместо .uasset ({ $count })
bag-clear = Очистить выбор
bag-glb-heading = Модели в GLB
bag-glb-text = Каждая модель — файлом .glb, стоящим осью Y вверх и обращённым вперёд, а её текстуры — отдельными файлами в общей для моделей папке textures. Всё остальное попадает как есть.
bag-glb-download = Скачать zip с GLB
bag-plain-heading = Файлы как есть
bag-plain-text = Каждый файл в том виде, в каком его импортировали, со всем, что ему нужно, в папке для каждого пака.
bag-remove = Убрать { $name } из выбора
bag-more = и ещё { $count }
app-bulk-tag-label = Тег, который нужно добавить выбранным ассетам или снять с них
app-bulk-tag-placeholder = Тег
app-bulk-tag-add-title = Добавить этот тег каждому выбранному ассету
app-bulk-tag-add = Добавить
app-bulk-tag-remove-title = Снять этот тег с каждого выбранного ассета
app-bulk-tag-remove = Снять
app-remove-from-collection = Убрать из «{ $name }»
app-unfavorite = Убрать из избранного
app-favorite = В избранное
app-select-all = Выбрать все ({ $count })
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] Выбрать { $count } результат
        [few] Выбрать все { $count } результата
        [many] Выбрать все { $count } результатов
       *[other] Выбрать все { $count } результата
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] Изменить { $count } ассет
        [few] Изменить { $count } ассета
        [many] Изменить { $count } ассетов
       *[other] Изменить { $count } ассета
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] Изменить { $count } результат
        [few] Изменить все { $count } результата
        [many] Изменить все { $count } результатов
       *[other] Изменить все { $count } результата
    }
bulk-edit-close = Закрыть
bulk-edit-tags = Теги
bulk-edit-tag-add = Добавить тег всем
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } из { $count }
bulk-edit-untick = Снять все отметки
bulk-edit-untick-title = Убрать все теги, затем отметить те, что нужно оставить
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = Самые используемые теги среди них
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Разные значения, не меняются
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Не меняется
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Очищено у всех
bulk-edit-reset = Вернуть как было
bulk-edit-all-note = Это изменит все результаты, в том числе ещё не загруженные, и отменить это одним шагом нельзя.
bulk-edit-save =
    { $count ->
        [one] Сохранить { $count } изменение
        [few] Сохранить { $count } изменения
        [many] Сохранить { $count } изменений
       *[other] Сохранить { $count } изменения
    }
bulk-edit-nothing = Ничего не изменилось
bulk-edit-saved =
    { $count ->
        [one] Обновлён { $count } ассет
        [few] Обновлено { $count } ассета
        [many] Обновлено { $count } ассетов
       *[other] Обновлено { $count } ассета
    }
app-archive-selected-title = Не показываются в поиске, пока их не восстановят или не удалят навсегда из архива
app-drop-items =
    { $count ->
        [one] Отпустите, чтобы добавить { $count } элемент
        [few] Отпустите, чтобы добавить { $count } элемента
        [many] Отпустите, чтобы добавить { $count } элементов
       *[other] Отпустите, чтобы добавить { $count } элемента
    }
app-drop = Отпустите, чтобы добавить в Hoard
app-drop-hint = Каждый файл или папка верхнего уровня становится отдельным паком. Гигабайты — не проблема; большие файлы докачиваются, если соединение оборвётся.

## Палитра команд (CommandPalette.vue, с командами из App.vue)

palette-group-go-to = Перейти
palette-group-actions = Действия
palette-group-theme = Тема
palette-group-kinds = Виды
palette-group-packs = Паки
palette-group-collections = Коллекции
palette-group-projects = Проекты
palette-group-saved = Сохранённые поиски
palette-group-assets = Ассеты
palette-hint-current = текущая
palette-library = Библиотека
palette-library-keywords = ассеты сетка главная assets grid home
palette-favorites = Избранное
palette-favorites-keywords = звёзды звезда понравилось избранные starred stars liked
palette-packs = Паки
palette-archive = Архив
palette-archive-keywords = корзина удалённые восстановить trash recycle bin deleted restore
palette-stats = Статистика библиотеки
palette-stats-keywords = статистика числа размер диск место количество statistics numbers size disk space count
palette-copies = Почти копии
palette-copies-keywords = дубликаты одинаковые похожие уменьшенные сконвертированные потерянное место duplicates same similar resized converted wasted space dedupe
palette-search = Поиск
palette-add-files = Добавить файлы…
palette-add-files-keywords = загрузить импорт upload import
palette-import-folder = Импортировать папку…
palette-import-folder-keywords = загрузить пак upload pack
palette-select-all = Выбрать все показанные ({ $count })
palette-select-all-keywords = отметить check
palette-select-several = Выбрать несколько…
palette-select-several-keywords = массово несколько тег архив bulk multiple tag archive
palette-archive-selected = Отправить в архив выбранные ({ $count })
palette-archive-selected-keywords = удалить убрать корзина delete remove trash
palette-edit-selected = Изменить выбранные ({ $count })
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = массовое изменение теги категория лицензия рейтинг
palette-paste-tags =
    { $count ->
        [1] Вставить скопированные теги в выбранный ассет
       *[other] Вставить скопированные теги в { $count } выбранных
    }
palette-clear-selection = Снять выделение
palette-clear-filters = Сбросить фильтры
palette-clear-filters-keywords = сброс очистить reset
palette-hide-supporting = Скрыть вспомогательные файлы
palette-show-supporting = Показать вспомогательные файлы
palette-supporting-keywords = нормали шероховатость маски normal maps roughness masks
palette-account = Учётная запись и токены доступа
palette-account-keywords = пароль вход пользователь токен api password login user token
palette-sign-out = Выйти ({ $name })
palette-sign-out-keywords = выход выйти log out logout
palette-sign-in = Войти
palette-sign-in-keywords = вход войти учётная запись пароль log in login account password
palette-settings = Настройки
palette-settings-keywords = blender ffmpeg llm ии теги unreal администратор инструменты параметры ai tagging admin tools preferences
palette-shortcuts = Сочетания клавиш
palette-shortcuts-keywords = справка клавиши помощь help keys
palette-theme-keywords = тема оформление режим theme appearance mode
palette-theme-variant-keywords = оформление цвет appearance color
palette-light-theme = Светлая тема: { $name }
palette-dark-theme = Тёмная тема: { $name }
palette-kind-all = Все виды (показывать не только { $kind })
palette-kind-only = Только { $kind }
palette-title = Палитра команд
palette-placeholder = Искать ассеты, перейти к паку, выполнить команду…
palette-results = Результаты
palette-searching = Поиск…
palette-no-match = По запросу «{ $query }» ничего не найдено
palette-foot-move = перемещение
palette-foot-run = выполнить
palette-foot-shortcuts = все сочетания

## Сочетания клавиш (ShortcutsDialog.vue)

shortcuts-title = Сочетания клавиш
shortcuts-close = Закрыть
shortcuts-anywhere = Везде
shortcuts-palette = Палитра команд: поиск, переход, команды
shortcuts-search = Поиск
shortcuts-this-list = Этот список
shortcuts-escape = Закрыть, снять выделение или закрыть сведения
shortcuts-library = Библиотека
shortcuts-move = Перемещение между плитками (или J и K)
shortcuts-open = Открыть сведения
shortcuts-play-sound = Проиграть или остановить звук
shortcuts-select = Выбрать или снять выбор
shortcuts-favorite = Добавить в избранное или убрать из него
shortcuts-reviewed = Отметить как просмотренное или снять отметку
shortcuts-tag = Добавить тег
shortcuts-edit = Изменить выбранное
shortcuts-copy-tags = Скопировать его теги
shortcuts-paste-tags = Вставить скопированные теги в выбранное
shortcuts-key-click = щелчок
shortcuts-select-range = Выбрать диапазон
shortcuts-select-all = Выбрать всё показанное
shortcuts-sound-player = Проигрыватель звука
shortcuts-play-pause = Воспроизведение или пауза
shortcuts-skip = Перемотка назад или вперёд
shortcuts-start-end = В начало или в конец
shortcuts-image-inspector = Просмотр изображений
shortcuts-zoom = Увеличить или уменьшить
shortcuts-fit = По размеру окна
shortcuts-actual-pixels = Реальный размер
shortcuts-tile = Тайлинг 3×3, чтобы проверить швы
shortcuts-background = Следующий фон
shortcuts-channel = Показать один канал

## KindChips.vue

kinds-group = Виды ассетов

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = добавлен человеком
tag-source-rule = от офлайн-разметчика
tag-source-meta = измерен по файлу
tag-source-ai = предложен ИИ
tag-remove = Убрать тег { $tag }
tag-accept = Оставить тег { $tag }
tag-accept-title = Оставить эту догадку как свой тег
tag-reject-title = Убрать; теггер не вернёт его
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Новый тег
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Вы имели в виду
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = раньше { $alias }

## ActivityIndicator.vue

activity-label-idle = Фоновая активность: простой
activity-label-running = Фоновая активность: в работе { $running }
activity-label-queued = Фоновая активность: в работе { $running }, в очереди { $queued }
activity-title = Фоновая активность
activity-unreachable = Нет связи с сервером
activity-unreachable-desktop = Hoard не отвечает
activity-empty = Ничего не выполняется. Импорт, расстановка тегов, преобразования и скачивание zip показываются здесь, пока идут.
activity-queued = в очереди
activity-stop = Остановить «{ $what }»
activity-stop-tagging = Остановить все теги от ИИ ({ $count })
activity-seconds = { $s } с
activity-minutes = { $min } мин
activity-hours = { $h } ч { $min } мин
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } треуг.
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Проиграть { $name }
tile-pause = Пауза: { $name }
tile-favorite = В избранное: { $name }
tile-in-favorites = В вашем избранном
tile-add-favorite = Добавить в избранное (F)
tile-more-like = Похожие на { $name }
tile-more-like-this = Похожие
tile-select = Выбрать { $name }

## CollectionPicker.vue

collections-add = Добавить в коллекцию
collections-menu = Коллекции
collections-find-label = Найти или создать коллекцию
collections-find-placeholder = Найдите или назовите новую коллекцию
collections-create = Создать
collections-empty = Коллекций пока нет. Введите название, чтобы создать первую.

## FolderPicker.vue

folders-list = Папки
folders-close = Закрыть
folders-browse = Обзор…
folders-up-title = На папку выше
folders-up = Вверх
folders-this-computer = Этот компьютер
folders-none = Здесь нет папок.

## LocationForm.vue

locations-mode-question = Что Hoard должен делать с этой папкой?
locations-mode-managed = Забрать себе: Hoard хранит здесь мои паки
locations-mode-managed-hint = Паки, которые вы загружаете или импортируете, копируются сюда, и Hoard держит их в порядке. Теперь папка принадлежит Hoard, поэтому не трогайте её содержимое.
locations-mode-watched = Следить: мои паки уже здесь
locations-mode-watched-hint = Каждая папка внутри неё становится паком. Hoard читает файлы на месте и никогда не изменяет, не перемещает и не удаляет их, а новые папки подхватывает по мере появления.
locations-folder-desktop = Папка
locations-folder-placeholder = Папка, в которой лежат ваши паки
locations-folder-hint-managed = Hoard создаст её, если её нет. В Docker это папка, которую вы подключили в контейнер.
locations-folder-hint-managed-desktop = Hoard создаст её, если её нет.
locations-folder-hint-watched = В Docker сначала подключите папку в контейнер (можно только для чтения), затем выберите её здесь.
locations-folder-hint-watched-desktop = Hoard только читает её: ничего в ней не меняется.
locations-name = Название
locations-name-placeholder-managed = Библиотека
locations-name-placeholder-watched = Мои паки
locations-files = Какие файлы сюда идут
locations-catch-all = А также всё, что не берут другие места хранения
locations-kinds-hint = Файлы нового пака идут в место хранения, где отмечен их вид. Остальные — в место для всего остального.
locations-files-all = Пока все. Позже в настройках можно добавить места хранения и отправлять некоторые виды (скажем, музыку) на другой диск.
locations-first-watched-note = Для загрузки нужна библиотека, которую хранит Hoard. Её можно добавить позже в «Настройки → Места хранения».
locations-checking = Проверяем папку…
locations-add = Добавить место хранения
locations-cancel = Отмена

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Не удалось загрузить места хранения
locations-takes-and-rest = { $kinds } и всё остальное
locations-takes-rest = Всё, что не берут другие
locations-takes-everything = Всё
locations-takes-nothing = Пока ничего
locations-save-failed = Не удалось сохранить место хранения
locations-unwatch-title = Перестать отслеживать { $name }?
locations-unwatch-body =
    { $count ->
        [one] Его { $count } пак уйдёт из Hoard вместе с тегами и заметками. Файлы в папке не затрагиваются.
        [few] Его { $count } пака уйдут из Hoard вместе с тегами и заметками. Файлы в папке не затрагиваются.
        [many] Его { $count } паков уйдут из Hoard вместе с тегами и заметками. Файлы в папке не затрагиваются.
       *[other] Его { $count } пака уйдут из Hoard вместе с тегами и заметками. Файлы в папке не затрагиваются.
    }
locations-unwatch-body-empty = Файлы в папке не затрагиваются.
locations-unwatch = Перестать отслеживать
locations-remove-title = Удалить { $name }?
locations-remove-body = Все файлы в нём сначала переносятся в другие места хранения по видам; удалится, когда опустеет. Ничего не потеряется.
locations-remove-body-only = Если в нём есть файлы, сначала добавьте другое место хранения, куда их перенести.
locations-remove = Удалить
locations-removed = { $name } удалено
locations-emptying = Переносим файлы из { $name }; оно удалится, когда это закончится
locations-offline-title = { $name } недоступно
locations-offline-body = Пока оно недоступно, файлы из него нельзя перенести. Всё равно удалить? Файлы, оставшиеся на нём, будут показаны как пропавшие.
locations-remove-anyway = Всё равно удалить
locations-remove-failed = Не удалось удалить { $name }
locations-rebalancing = Переносим сохранённые файлы в места хранения, которые берут их вид
locations-rebalance-failed = Не удалось перенести файлы
locations-scanned-never = ещё не сканировалось
locations-scanned-now = только что просканировано
locations-scanned-minutes = просканировано { $mins } мин назад
locations-scanned-hours = просканировано { $hours } ч назад
locations-scanned-date = просканировано { $date }
locations-scan-failed = Не удалось просканировать { $name }
locations-watching = Отслеживается { $name }
locations-added = { $name } добавлено
locations-loading = Загрузка…
locations-managed-title = Библиотеки, которые хранит Hoard
locations-watched-title = Отслеживаемые папки
locations-managed-hint = Сюда копируются загрузки и импорт. Каждое место хранения берёт отмеченные в нём виды; место для всего остального берёт остальное.
locations-watched-hint = Папки с паками, проиндексированные на месте. Hoard никогда не изменяет файлы в них и каждые 15 минут ищет новые и удалённые папки.
locations-managed-empty = Пока ни одной, так что загрузки некуда класть. Добавьте место хранения ниже.
locations-reachable = Доступно
locations-unreachable-now = Сейчас недоступно
locations-catch-all-tag = для остального
locations-packs =
    { $count ->
        [one] { $count } пак
        [few] { $count } пака
        [many] { $count } паков
       *[other] { $count } пака
    }
locations-unreachable = недоступно
locations-scanning = Сканирование…
locations-scan = Сканировать
locations-edit = Изменить
locations-catch-all-locked = Чтобы это изменить, сделайте местом для всего остального другое место хранения
locations-edit-hint-before = Действует для файлов, добавленных с этого момента. Чтобы перенести уже сохранённые, используйте кнопку
locations-edit-hint-after = ниже.
locations-save = Сохранить
locations-rebalance = Перенести сохранённые файлы
locations-rebalance-hint = После изменения того, что берёт место хранения: уже сохранённые файлы переносятся туда, куда теперь идёт их вид.
locations-add-title = Добавить место хранения
locations-add-hint = Другой диск для некоторых видов файлов или уже существующая папка с паками.
locations-kind-question = Какое это место хранения?
locations-kind-folder = Папка
locations-kind-folder-hint = На диске или сетевой папке NAS, подключённой в контейнер, или папка, которую синхронизирует Dropbox, Google Drive, OneDrive или iCloud Drive. Hoard может хранить там файлы или следить за пакетами, которые у вас уже есть.
locations-kind-folder-hint-desktop = На этом компьютере, другом диске или сетевой папке NAS, или папка, которую синхронизирует Dropbox, Google Drive, OneDrive или iCloud Drive. Hoard может хранить там файлы или следить за пакетами, которые у вас уже есть.
locations-kind-s3-hint = Бакет в Amazon S3 или похожем сервисе: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard хранит там файлы.
locations-s3-title = Хранилище S3
locations-s3-hint = Место хранения s3://bucket-name — это бакет в этом сервисе: AWS или любом совместимом с S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage на NAS).
locations-bucket = Бакет
locations-bucket-hint = Имя бакета, который уже существует. Ключи выше должны позволять просматривать его список, читать и записывать в него.
locations-s3-endpoint = Адрес
locations-s3-endpoint-placeholder = Пусто для AWS; например, https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Регион
locations-s3-access-key = Идентификатор ключа доступа
locations-s3-secret-key = Секретный ключ доступа
locations-s3-headers = Дополнительные заголовки
locations-s3-headers-placeholder = Name=value, по одному на строку (например, для Cloudflare Access)
locations-s3-kept = Сохранено (введите, чтобы заменить)
locations-s3-secrets = Ключи и заголовки хранятся в настройках сервера и больше не показываются.
locations-s3-secrets-desktop = Ключи и заголовки хранятся в хранилище паролей этого компьютера и больше не показываются.
locations-s3-saved = Хранилище S3 сохранено
locations-s3-save-failed = Не удалось сохранить хранилище S3
locations-add-more = Добавить место хранения…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Не удалось загрузить проблемы
issues-started =
    { $count ->
        [one] Запущено снова.
        [few] Снова запущено: { $count }.
        [many] Снова запущено: { $count }.
       *[other] Снова запущено: { $count }.
    }
issues-retry-failed = Не удалось повторить
issues-dismiss-failed = Не удалось убрать
issues-dismiss-all-title = Убрать все ({ $count })?
issues-dismiss-all-body = Группа «{ $title }» будет очищена. Если что-то снова пойдёт не так, оно снова появится в списке.
issues-dismiss-all = Убрать все
issues-just-now = только что
issues-minutes-ago = { $mins } мин назад
issues-hours-ago = { $hours } ч назад
issues-loading = Загрузка…
issues-none = Всё в порядке.
issues-action-all = { $action } (все)
issues-open-asset = Открыть
issues-open-pack = Показать пак
issues-open-locations = Места хранения
issues-where-before = в
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } раз
        [few] { $count } раза
        [many] { $count } раз
       *[other] { $count } раза
    }
issues-starting = Запуск…
issues-dismiss-named = Убрать { $title }
issues-dismiss = Убрать
issues-show-fewer = Показать меньше
issues-show-all = Показать все ({ $count })
issues-not-listed = Не показано ещё: { $count }.

## archive.ts

archive-anyway = Всё равно отправить в архив
archive-failed = Не удалось отправить в архив: { $error }
archive-undo = Отменить
archive-restore-failed = Не удалось восстановить: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } пак
        [few] { $packs } пака
        [many] { $packs } паков
       *[other] { $packs } пака
    } и { $assets ->
        [one] { $assets } ассет
        [few] { $assets } ассета
        [many] { $assets } ассетов
       *[other] { $assets } ассета
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } пак
        [few] { $packs } пака
        [many] { $packs } паков
       *[other] { $packs } пака
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } ассет
        [few] { $assets } ассета
        [many] { $assets } ассетов
       *[other] { $assets } ассета
    }
archive-count-nothing = ничего

## ArchiveView.vue

archive-load-failed = Не удалось загрузить архив: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Восстановлено: { $what }
archive-purged = Удалено навсегда: { $what }
archive-purge-failed = Не удалось удалить: { $error }
archive-purge-title = Удалить { $name } навсегда?
archive-purge-pack-body =
    { $count ->
        [one] Вместе с ним удалятся его { $count } ассет и { $size } файлов, кроме файлов, общих с другим паком. Это нельзя отменить.
        [few] Вместе с ним удалятся его { $count } ассета и { $size } файлов, кроме файлов, общих с другим паком. Это нельзя отменить.
        [many] Вместе с ним удалятся его { $count } ассетов и { $size } файлов, кроме файлов, общих с другим паком. Это нельзя отменить.
       *[other] Вместе с ним удалятся его { $count } ассета и { $size } файлов, кроме файлов, общих с другим паком. Это нельзя отменить.
    }
archive-purge = Удалить навсегда
archive-purge-asset-body = Его файл будет удалён, если только его не использует другой ассет. Это нельзя отменить.
archive-empty-title = Очистить архив?
archive-empty-body = Навсегда будут удалены: { $what }, а их файлы освобождены. Это нельзя отменить.
archive-empty = Очистить архив
archive-title = Архив
archive-intro = Паки и ассеты в архиве не попадают в поиск, подсчёты и список тегов. Восстановите их или удалите навсегда, чтобы освободить место.
archive-loading = Загрузка…
archive-is-empty = Архив пуст
archive-is-empty-hint = Отправьте пак или ассет в архив, и он будет ждать здесь, пока вы его не восстановите или не удалите навсегда.
archive-packs = Паки
archive-assets = Ассеты
archive-asset-count =
    { $count ->
        [one] { $count } ассет
        [few] { $count } ассета
        [many] { $count } ассетов
       *[other] { $count } ассета
    }
archive-archived = отправлен в архив { $when }
archive-from = из { $pack }
archive-restore = Восстановить
archive-show-more = Показать ещё (осталось { $count })

## mine.ts

projects-favorite-add-failed = Не удалось добавить в избранное: { $error }
projects-favorite-remove-failed = Не удалось убрать из избранного: { $error }
projects-save-search-failed = Не удалось сохранить поиск: { $error }
projects-forget-search-failed = Не удалось удалить { $name }: { $error }
projects-change-search-failed = Не удалось изменить { $name }: { $error }

## projects.ts

projects-list-and = { $items } и { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = ещё { $count }
projects-other =
    { $count ->
        [one] ещё { $count } проект
        [few] ещё { $count } проекта
        [many] ещё { $count } проектов
       *[other] ещё { $count } проекта
    }
projects-usage-title-one = Это использует { $name }
projects-usage-title =
    { $count ->
        [one] Это использует { $count } проект
        [few] Это используют { $count } проекта
        [many] Это используют { $count } проектов
       *[other] Это используют { $count } проекта
    }
projects-usage-body-one = Используется в { $list }: при следующем hoard sync оно оттуда уберётся.
projects-usage-body = Используется в { $list }: при следующем hoard sync каждого из них оно оттуда уберётся.

## uploader.ts

upload-network-error = ошибка сети
upload-gave-up = { $what }: { $error } (попыток: { $attempts })
upload-failed-plain = не удалось
upload-already-imported = уже импортировано
upload-part-failed = загрузка не удалась ({ $status })
upload-batch =
    { $count ->
        [one] группа из { $count } файла
        [few] группа из { $count } файлов
        [many] группа из { $count } файлов
       *[other] группа из { $count } файла
    }
upload-batch-failed = группа не загрузилась ({ $status })
upload-finish = завершение загрузки

## UploadQueue.vue

upload-eta-seconds = осталось { $s } с
upload-eta-minutes = осталось { $min } мин
upload-eta-hours = осталось { $h } ч
upload-queue = Импорт
upload-importing = Импорт паков: { $count }
upload-bytes-of = { $sent } из { $total }
upload-rate = { $size }/с
upload-waiting = Ожидание
upload-unpacking = Загружено, распаковываем
upload-already = Уже в библиотеке
upload-stopped = Остановлено
upload-files-of =
    { $total ->
        [one] { $done } из { $total } файла
        [few] { $done } из { $total } файлов
        [many] { $done } из { $total } файлов
       *[other] { $done } из { $total } файла
    }
upload-resumed = докачка, { $size } уже было на месте
upload-skipped = Точно такие же файлы уже импортированы как «{ $name }», поэтому они пропущены.
upload-error = { $error }. Прогресс сохранён: повторите или перетащите те же файлы снова позже.
upload-retry = Повторить
# $name is the pack the library has already that these files go into.
upload-update = Обновить «{ $name }»
upload-copy = Импортировать как копию
# $name is the pack the library has already that the upload goes into.
upload-into = Добавляется в пак «{ $name }», который уже есть в библиотеке: новые файлы добавляются, изменённые заменяются.
upload-own-pack = Сделать отдельным паком
upload-dismiss = Скрыть
upload-cancel = Отмена
upload-minimize = Свернуть
upload-expand = Показать импорт
upload-pill = Импорт: { $count } · { $percent }%
upload-pill-stopped = Остановлено: { $count }
upload-interrupted = Прервано
upload-resume = Продолжить
upload-resume-all = Продолжить все
upload-resume-hint-folder = Выберите ту же папку снова или перетащите её на страницу, чтобы продолжить с того места, где импорт остановился.
upload-resume-hint-file = Выберите тот же файл снова или перетащите его на страницу, чтобы продолжить с того места, где импорт остановился.
upload-not-same = Это не те же файлы, поэтому ничего не продолжено.
upload-permission-denied = Браузеру не разрешили снова прочитать файлы. Выберите их вручную.

## PacksView.vue

packs-rename-failed = Не удалось переименовать: { $error }
packs-collection-deleted = Коллекция «{ $name }» удалена
packs-delete-failed = Не удалось удалить: { $error }
packs-status-uploading = Загрузка (получены не все файлы)
packs-status-ingesting = Распаковка и расстановка тегов
packs-status-analyzing = LLM расставляет теги
packs-status-ready = Готов
packs-status-failed = Ошибка
packs-retag-failed = Не удалось начать повторную расстановку тегов: { $error }
packs-read-files-failed = Не удалось прочитать файлы пака: { $error }
packs-context-load-failed = Не удалось загрузить сведения о паке из магазина: { $error }
packs-undo-store-failed = Не удалось убрать страницу магазина: { $error }
packs-context-save-failed = Не удалось сохранить сведения из магазина: { $error }
packs-unreal-load-failed = Не удалось загрузить состояние преобразования Unreal: { $error }
packs-unreal-discard-title = Удалить оригиналы?
packs-unreal-discard-body = Исходные файлы .uasset в { $name } удаляются после преобразования. Данные, которые есть только в Unreal (например, графы материалов), будут потеряны навсегда.
packs-unreal-discard-confirm = Преобразовать и удалить
packs-unreal-convert-failed = Не удалось начать преобразование: { $error }
packs-unreal-fact-converted = Преобразовано из Unreal: { $done }
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } пакет Unreal
        [few] { $left } пакета Unreal
        [many] { $left } пакетов Unreal
       *[other] { $left } пакета Unreal
    }
packs-unreal-fact-both = Преобразовано из Unreal: { $done }, осталось пакетов: { $left }
packs-thumbs-canceled = Отменено после { $done } из { $total }
packs-thumbs-drew =
    { $count ->
        [one] Нарисована { $count } модель
        [few] Нарисованы { $count } модели
        [many] Нарисовано { $count } моделей
       *[other] Нарисовано { $count } модели
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: заменена { $improved } миниатюра.
        [few] { $lead }: заменены { $improved } миниатюры.
        [many] { $lead }: заменено { $improved } миниатюр.
       *[other] { $lead }: заменено { $improved } миниатюры.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: заменена { $improved } миниатюра, не удалось нарисовать: { $failed }.
        [few] { $lead }: заменены { $improved } миниатюры, не удалось нарисовать: { $failed }.
        [many] { $lead }: заменено { $improved } миниатюр, не удалось нарисовать: { $failed }.
       *[other] { $lead }: заменено { $improved } миниатюры, не удалось нарисовать: { $failed }.
    }
packs-archived =
    { $count ->
        [one] { $name } и его { $count } ассет отправлены в архив
        [few] { $name } и его { $count } ассета отправлены в архив
        [many] { $name } и его { $count } ассетов отправлены в архив
       *[other] { $name } и его { $count } ассета отправлены в архив
    }
packs-blobs-bad =
    { $count ->
        [one] { $count } файл не прошёл последнюю проверку целостности.
        [few] { $count } файла не прошли последнюю проверку целостности.
        [many] { $count } файлов не прошли последнюю проверку целостности.
       *[other] { $count } файла не прошли последнюю проверку целостности.
    }
packs-collections = Коллекции
packs-browse-named = Открыть { $name }
packs-collection-name = Название коллекции
packs-save = Сохранить
packs-cancel = Отмена
packs-assets =
    { $count ->
        [one] { $count } ассет
        [few] { $count } ассета
        [many] { $count } ассетов
       *[other] { $count } ассета
    }
packs-updated = Обновлено { $date }
packs-browse = Открыть
packs-collection-download = Скачать (.zip)
packs-collection-credits = Авторы
packs-collection-credits-title = Кто сделал паки, из которых взяты эти ассеты, их лицензии и строки благодарности, которые они просят указать
packs-rename = Переименовать
packs-delete = Удалить
packs-packs = Паки
packs-empty = Паков пока нет. Перетащите zip или папку в любое место этой страницы, чтобы добавить пак.
packs-find = Найти пак
packs-find-placeholder = Найдите пак по названию, магазину, лицензии или тегу
packs-sort = Порядок
packs-sort-new = Сначала новые
packs-sort-old = Сначала старые
packs-sort-size = Сначала большие
packs-sort-assets = Больше всего ассетов
packs-sort-count = Количество ассетов
packs-sort-smallest = Сначала маленькие
packs-sort-fewest = Меньше всего ассетов
packs-with-kind = Паки, в которых есть
packs-state = Состояние
packs-state-any = Любое состояние
packs-state-working = В работе
packs-state-failed = С ошибкой
packs-state-untagged = Ещё без описания
packs-state-unlicensed = Лицензия не указана
packs-store = Магазин
packs-store-any = Любой магазин
packs-store-from = Из { $store }
packs-store-none = Без страницы магазина
packs-shown-of = { $shown } из { $total }
packs-clear = Сбросить
packs-no-match = Подходящих паков нет.
packs-clear-all = Сбросить поиск и фильтры
packs-pack-name = Название пака
packs-thumbs-drawing = Рисуем миниатюры в этом браузере…
packs-thumbs-drawing-count = Рисуем миниатюры в этом браузере: { $done } из { $total }…
packs-thumbs-stopped = Остановлено: { $error }
packs-thumbs-none = У каждой модели здесь уже есть миниатюра, нарисованная со всеми текстурами.
packs-dismiss = Скрыть
packs-license = Лицензия: { $license }
packs-license-tier = Лицензия: { $license } ({ $tier })
packs-license-not-found = не найдена
packs-from-before = Из
packs-from-after = { "" }
packs-added = Добавлен { $date }
pack-panel-label = Об этом паке
pack-panel-hide = Скрыть сведения о паке
pack-panel-kinds = Что внутри
pack-panel-all = Все паки
packs-download = Скачать пак (.zip)
packs-store-details = Сведения из магазина
packs-more-named = Другие действия для { $name }
packs-more = Ещё
packs-rename-more = Переименовать…
packs-retag-rules = Перезапустить офлайн-разметчик
packs-retag-llm = Перезапустить LLM-разметчик
packs-make-thumbs = Сделать миниатюры в этом браузере
packs-make-thumbs-title = Открыть каждую модель по очереди в скрытом 3D-просмотрщике и взять её изображение в качестве миниатюры там, где оно лучше серверного
packs-make-thumbs-title-desktop = Открыть каждую модель по очереди в скрытом 3D-просмотрщике и взять её изображение в качестве миниатюры там, где оно лучше той, что сделал Hoard
packs-read-files = Перечитать файлы пака
packs-credits = Скачать список авторов
packs-credits-title = CREDITS.md: кто сделал пак, его лицензия и строка благодарности, которую он просит указать
packs-unreal-open = Преобразовать контент Unreal…
packs-archive = Отправить пак в архив
packs-unreal-intro = Hoard сам читает пакеты Unreal и сохраняет их содержимое как обычные файлы: текстуры в PNG или EXR, звуки в WAV, материалы в JSON. Преобразованный файл занимает место пакета и сохраняет его теги, ссылки и коллекции.
packs-unreal-converted =
    { $count ->
        [one] преобразован
        [few] преобразованы
        [many] преобразовано
       *[other] преобразовано
    }
packs-unreal-converted-kept =
    { $count ->
        [one] преобразован, у { $kept } сохранён оригинал
        [few] преобразованы, у { $kept } сохранён оригинал
        [many] преобразовано, у { $kept } сохранён оригинал
       *[other] преобразовано, у { $kept } сохранён оригинал
    }
packs-unreal-pending =
    { $count ->
        [one] ещё не просмотрен
        [few] ещё не просмотрены
        [many] ещё не просмотрено
       *[other] ещё не просмотрено
    }
packs-unreal-needs-editor =
    { $count ->
        [one] ждёт Unreal Editor
        [few] ждут Unreal Editor
        [many] ждут Unreal Editor
       *[other] ждут Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] без распространённого формата
        [few] без распространённого формата
        [many] без распространённого формата
       *[other] без распространённого формата
    }
packs-unreal-failed =
    { $count ->
        [one] с ошибкой
        [few] с ошибкой
        [many] с ошибкой
       *[other] с ошибкой
    }
packs-unreal-reasons = Почему пакеты остались
packs-unreal-editor-before = Мешам и анимациям нужен Unreal Editor. На машине с установленным Unreal выполните
packs-unreal-editor-command = hoard unreal-worker --editor <путь к UnrealEditor-Cmd>
packs-unreal-editor-after = ; он экспортирует их в FBX и загрузит результат сюда.
packs-unreal-discard = Удалить исходные файлы .uasset после преобразования
packs-unreal-discard-hint = Освобождает место. Данные, которые есть только в Unreal (например, графы материалов), теряются.
packs-unreal-drop = Удалить пакеты без распространённого формата
packs-unreal-drop-hint = Блюпринты, уровни и другие ассеты, которые работают только в движке.
packs-unreal-convert = Преобразовать
packs-context-hint-before = Откуда пак и что о нём говорит продавец. LLM-разметчик читает это вместе с каждым файлом, так что описание из магазина вроде «120 заклинаний льда и огня» помогает ему назвать
packs-context-hint-after = .
packs-store-auto = Найдено автоматически: пак сам ссылается на эту страницу.
packs-store-not-this = Не эта
packs-lookup-label = Искать в магазинах
packs-lookup-placeholder = Название пака в магазине
packs-lookup-searching = Поиск…
packs-lookup-search = Искать в магазинах
packs-lookup-hide = Скрыть
packs-lookup-clues = Из файлов пака: { $clues }.
packs-lookup-also = Также искали { $queries }.
packs-lookup-quoted = «{ $query }»
packs-lookup-nothing = На Fab, itch.io, Kenney и OpenGameArt ничего похожего нет. Попробуйте меньше слов или другие слова.
packs-lookup-use = Выбрать
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Сейчас
packs-review-store = Из магазина
packs-review-same = Одинаково
packs-review-read-page = При сохранении будет прочитано со страницы
packs-review-take =
    { $count ->
        [one] Заполнить { $count } поле
        [few] Заполнить { $count } поля
        [many] Заполнить { $count } полей
       *[other] Заполнить { $count } поля
    }
packs-lookup-searched = Искали { $date }.
packs-lookup-again = Искать снова
packs-lookup-again-after = .
packs-lookup-unreachable = На этот раз не удалось связаться с { $stores }.
packs-lookup-also-look = Поищите также на
packs-lookup-links-and = { " " }и{ " " }
packs-lookup-browser-only = (там можно искать только в браузере); вставьте адрес страницы ниже.
packs-lookup-open = Найти в магазинах…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Продавец
packs-seller-placeholder = Кто его сделал
packs-license-label = Лицензия
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Уровень лицензии
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] У { $count } ассета своя лицензия: переключить на лицензию пака
        [few] У { $count } ассетов своя лицензия: переключить на лицензию пака
        [many] У { $count } ассетов своя лицензия: переключить на лицензию пака
       *[other] У { $count } ассета своя лицензия: переключить на лицензию пака
    }
packs-store-page = Страница в магазине
packs-store-text = Описание из магазина
packs-store-text-placeholder = Вставьте текст описания или оставьте пустым, чтобы загрузить его со страницы магазина.
packs-store-text-fetch = При сохранении описание загрузится со страницы магазина.
packs-readme = README, найденный в паке
packs-save-retag = Сохранить и перезапустить LLM-разметчик

## AssetDetail.vue

asset-lazy-3d-viewer = 3D-просмотрщик
asset-lazy-text = текст
asset-lazy-text-viewer = просмотрщик текста
asset-lazy-image-viewer = просмотрщик изображений
asset-lazy-material-preview = превью материала
asset-lazy-material-viewer = просмотрщик материалов
asset-lazy-sprite-player = проигрыватель спрайтов
asset-lazy-sheet-view = просмотрщик листов спрайтов
asset-rel-texture = текстура
asset-rel-buffer = данные геометрии
asset-rel-material = файл материала
asset-rel-animation = анимация
asset-rel-library = связанный файл
asset-rel-guess = вероятно, относится
asset-rel-sidecar = сведения из магазина
asset-rel-variant = вариант
asset-rel-sprite = спрайт
asset-rel-sample = сэмпл
asset-role-map = карта { $role }
asset-maps-count =
    { $count ->
        [one] { $count } карта
        [few] { $count } карты
        [many] { $count } карт
       *[other] { $count } карты
    }
asset-needs = Нужны
asset-animations = Анимации
asset-sprites = Спрайты
asset-used-by = Используется в
asset-loose-sprites = Также отдельными файлами
asset-in-sheet = В спрайт-листе
asset-other-formats = Другие форматы
asset-shown-as = В библиотеке показан как
asset-conversion-failed = преобразование не удалось
asset-as-converting = Преобразование…
asset-as-converting-percent = Преобразование… { $percent }%
asset-as-queued = В очереди…
asset-download = Скачать
asset-as-label = Скачать как
asset-as-original = { $format } (исходный формат)
asset-as-pack-files = Файлы из набора
asset-as-size = Размер
asset-as-full-size = Полный размер
asset-as-up-to = До { $size } пкс
asset-as-hint = Выберите размер или другой формат
asset-meta-size = Размер
asset-meta-length = Длительность
asset-meta-sample-rate = Частота дискретизации
asset-meta-channels = Каналы
asset-meta-mono = Моно
asset-meta-stereo = Стерео
asset-meta-bit-depth = Разрядность
asset-meta-bits = { $bits } бит
asset-meta-level = Уровень
asset-meta-level-value = пик { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = пик { $peak } dBFS, RMS { $rms } dBFS, есть клиппинг
asset-meta-loop = Петля
asset-meta-loop-value = от { $start } до { $end }
asset-meta-playback = Воспроизведение
asset-meta-one-shot = Однократно
asset-meta-envelope = Огибающая
asset-meta-envelope-times = атака { $attack }, затухание { $tail }
asset-meta-envelope-hit = удар
asset-meta-envelope-swell = нарастание
asset-meta-envelope-sustained = протяжный
asset-meta-tone = Тембр
asset-meta-tone-value = яркость { $khz } кГц, шум { $noise } %
asset-meta-pitch = Высота
asset-meta-pitch-rises =
    { $octaves ->
        [one] повышается на { $octaves } октаву
        [few] повышается на { $octaves } октавы
        [many] повышается на { $octaves } октав
       *[other] повышается на { $octaves } октавы
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] понижается на { $octaves } октаву
        [few] понижается на { $octaves } октавы
        [many] понижается на { $octaves } октав
       *[other] понижается на { $octaves } октавы
    }
asset-meta-loops-cleanly = Бесшовно зацикливается
asset-meta-tempo = Темп
asset-meta-tempo-heard = около { $bpm } BPM, на слух
asset-meta-tempo-heard-or = около { $bpm } BPM (или { $alt }), на слух
asset-meta-key = Тональность
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, на слух
music-key-major = { $note } мажор
music-key-minor = { $note } минор
asset-meta-root-note = Основной тон
asset-meta-cue-markers = Маркеры
asset-meta-library = Библиотека
asset-meta-keywords = Ключевые слова
asset-meta-title = Название
asset-meta-artist = Исполнитель
asset-meta-album = Альбом
asset-meta-genre = Жанр
asset-meta-generator = Генератор
asset-meta-everything-with = Показать всё с тегом { $tag }
asset-meta-saved-with = Сохранено в
asset-meta-color-mode = Цветовой режим
asset-meta-layers = Слои
asset-meta-range = Диапазон
asset-meta-hdr = HDR (в превью применена тональная компрессия)
asset-meta-format = Формат
asset-meta-large-document = Большой документ (PSB)
asset-meta-ai = ИИ
asset-meta-ai-made = Создано генеративным ИИ
asset-meta-ai-edited = Отредактировано генеративным ИИ
asset-meta-ai-how-metadata = метаданные
asset-meta-ai-how-generator = настройки генератора
asset-meta-ai-how-iptc = тип цифрового источника IPTC
asset-meta-ai-says = { $made } (источник: { $how })
asset-meta-ai-by-says = { $made } ({ $by }; источник: { $how })
asset-meta-provenance = Происхождение
asset-meta-c2pa = Есть C2PA Content Credentials
asset-meta-compression = Сжатие
asset-meta-note = Примечание
asset-meta-maps = Карты
asset-meta-sizes =
    { $count ->
        [one] Размер
        [few] Размеры
        [many] Размеры
       *[other] Размеры
    }
asset-meta-triangles = Треугольники
asset-meta-vertices = Вершины
asset-meta-meshes = Меши
asset-meta-materials = Материалы
asset-meta-animations = Анимации
asset-meta-skins = Скины
asset-meta-unreal-asset = Ассет Unreal
asset-meta-unreal-saved-with = { $class }, сохранён в UE { $engine }
asset-meta-package = Пакет
asset-meta-cooked = Cooked (сборка игры): превью и исходники вырезаны
asset-meta-tiling = Тайлинг
asset-meta-seamless = Бесшовная по обеим осям
asset-meta-analysis-error = Не удалось полностью прочитать этот файл: { $error }
asset-copy-blocked = Не удалось скопировать: браузер это запретил
asset-save-error = Не удалось сохранить: { $error }
asset-tag-removed = Тег { $tag } убран
asset-tag-restored = Тег { $tag } возвращён
asset-tags-added =
    Добавлено: { $count ->
        [one] { $count } тег
        [few] { $count } тега
        [many] { $count } тегов
       *[other] { $count } тега
    }
asset-tags-accepted =
    Оставлено: { $count ->
        [one] { $count } предложенный тег
        [few] { $count } предложенных тега
        [many] { $count } предложенных тегов
       *[other] { $count } предложенных тега
    }
asset-reviewed = Просмотрено
asset-reviewed-on-title = Просмотрено: правила и ИИ не трогают его теги. Нажмите, чтобы снять отметку (R)
asset-reviewed-off-title = Отметить как просмотренное, чтобы правила и ИИ не трогали его теги (R)
asset-reviewed-on = Отмечено как просмотренное
asset-reviewed-off = Отметка «просмотрено» снята
# $name is an asset's name.
asset-reviewed-on-named = { $name } отмечен как просмотренный
asset-reviewed-off-named = Отметка «просмотрено» снята у { $name }
asset-undo = Отменить
asset-play-error = Не удалось воспроизвести этот файл: { $error }
asset-copied-hash = Хеш скопирован
asset-copied-toml = toml скопирован
asset-copied-id = ID скопирован
asset-copied-link = Ссылка скопирована
asset-origin-before = Исходный файл:
asset-origin-class-engine = , { $class }, сохранённый в UE { $engine }.
asset-origin-class = , { $class }.
asset-origin-engine = , сохранённый в UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Экспортирован из Unreal Editor { $date }.
asset-origin-editor-notes = Экспортирован из Unreal Editor { $date } ({ $notes }).
asset-origin-hoard = Hoard извлёк его из пакета { $date }.
asset-origin-hoard-notes = Hoard извлёк его из пакета { $date } ({ $notes }).
asset-origin-download = Скачать исходный .{ $ext }
asset-origin-discarded = Исходный пакет удалён после преобразования.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = История
history-loading = Загрузка…
history-none = Здесь пока ничего не менялось.
history-load-failed = Не удалось загрузить историю: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = ИИ
history-by-rule = Правило
history-by-someone = Кто-то
history-by-app = { $who } через приложение
history-by-assistant = { $who } через ассистента
history-field-description = Описание
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Добавлен тег { $tag }
history-tag-removed = Снят тег { $tag }
history-tag-kept = Оставлен предложенный тег { $tag }
history-tag-changed = Изменён тег { $tag }
history-reviewed = Отмечено как проверенное
history-unreviewed = Отмечено как непроверенное
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: задано { $to }
history-cleared = { $field }: очищено (было { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Отменить
history-revert-title = Вернуть то, что изменило это изменение
history-reverted = Отменено: { $change }
history-revert-failed = Не удалось отменить: { $error }
asset-details = Сведения об ассете
asset-in-pack = из
asset-previous = Предыдущий ассет
asset-next = Следующий ассет
asset-favorite = В избранное
asset-favorite-on-title = В вашем избранном (F — убрать)
asset-favorite-off-title = Добавить в избранное (F)
asset-share = Поделиться
asset-share-title = Поделиться ссылкой на этот ассет (открывший её сначала войдёт в Hoard)
asset-layout-panel = Показать на боковой панели
asset-layout-page = Показать на всю страницу
asset-layout-page-title = Показать на всю страницу: большое превью рядом со сведениями
asset-close = Закрыть сведения
asset-rename-title = Нажмите, чтобы переименовать
asset-name = Название
asset-views-alt = Модель с четырёх сторон
asset-inspect-aria = Рассмотреть { $name } на весь экран
asset-inspect-title = Рассмотреть: масштаб, тайлинг, каналы
asset-playback-position = Позиция воспроизведения
asset-playback-value = { $current } из { $duration }
asset-pause = Пауза
asset-play = Воспроизвести
asset-waveform-drawing = Рисуем волну…
asset-waveform-skipped = Для таких больших файлов волна не рисуется
asset-play-animation = Воспроизвести как анимацию
asset-show-views = Показать лист с четырьмя видами, который видел разметчик
asset-palette = Цвета на картинке
asset-swatch-title = { $hex }, { $percent }% картинки. Найти ассеты с этим цветом
asset-swatch-aria = Найти ассеты с цветом { $hex }, { $percent }% этой картинки
asset-more-like-this = Похожие
asset-desc-label = Ваше описание
asset-desc-placeholder = Что это, где это пригодится…
asset-save = Сохранить
asset-cancel = Отмена
asset-desc-family =
    { $count ->
        [one] Сохраняется во всех вариантах ({ $count })
        [few] Сохраняется во всех { $count } вариантах
        [many] Сохраняется во всех { $count } вариантах
       *[other] Сохраняется во всех вариантах ({ $count })
    }
asset-desc-edit = Изменить описание
asset-desc-write = Написать описание
asset-desc-ai = Написано ИИ
asset-desc-file = Из файла
asset-desc-store-before = Со
asset-desc-store-page = страницы { $store }
asset-desc-store-page-unnamed = страницы магазина
asset-desc-store-after = (весь пак)
asset-less = Меньше
asset-more = Больше
asset-tags = Теги
asset-tag-add-label = Добавить тег
asset-tag-add-placeholder = Добавьте тег, например mood:tense
asset-apply-family = Применить изменения ко всем вариантам ({ $count })
asset-tags-accept-all =
    Оставить все: { $count ->
        [one] { $count } догадку
        [few] { $count } догадки
        [many] { $count } догадок
       *[other] { $count } догадки
    }
asset-tags-accept-all-title = Оставить как свои все теги, которые угадали правила и ИИ
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Отклонено:
asset-tag-restore-title = Вернуть { $tag }
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = У похожих есть:
asset-tag-suggest-title = Добавить { $tag }
asset-details-heading = Сведения
asset-field-category = Категория
asset-field-license = Лицензия
asset-field-source = Источник
asset-field-rating = Оценка
# Shown in place of an empty value, to fill it in.
asset-field-add = Добавить
asset-field-new = Добавить поле
asset-field-name = Название поля
asset-field-value = Значение
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = из пака
asset-field-license-placeholder = Например, CC0 или CC-BY 4.0
asset-field-remove-placeholder = Пусто — удалить
# $field is a field's name, like "License".
asset-field-edit-title = Изменить: { $field }
asset-field-saved = Сохранено: { $field }
asset-field-cleared = Очищено: { $field }
asset-rate =
    { $count ->
        [one] { $count } звезда
        [few] { $count } звезды
        [many] { $count } звёзд
       *[other] { $count } звезды
    }
asset-rated =
    Оценка: { $count ->
        [one] { $count } звезда
        [few] { $count } звезды
        [many] { $count } звёзд
       *[other] { $count } звезды
    }
asset-rating-clear-title = Нажмите ещё раз, чтобы убрать оценку
asset-rating-cleared = Оценка убрана
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Нажмите ещё раз: { $count } звезда
        [few] Нажмите ещё раз: { $count } звезды
        [many] Нажмите ещё раз: { $count } звёзд
       *[other] Нажмите ещё раз: { $count } звезды
    }
asset-collections = Коллекции
asset-collection-remove = Убрать из «{ $name }»
asset-collection-add-another = Добавить в другую
asset-collection-add = Добавить в коллекцию
asset-used-in = Используется в
asset-used-in-title = Показать, что есть в { $name }
asset-used-in-synced = синхронизирован { $when }
asset-maps = Карты
asset-show-more = Показать ещё { $count }
asset-materials-heading =
    { $count ->
        [one] Материал
        [few] Материалы
        [many] Материалы
       *[other] Материалы
    }
asset-material-maps-aria = { $name }: карты
asset-variants = Варианты (дублей: { $count })
asset-fit-animations = Подходящие анимации ({ $count })
asset-fit-characters = Подходящие персонажи ({ $count })
asset-copies = Идентичные копии
asset-copies-note = Одинаковые байты хранятся один раз; этот файл есть и в этих паках.
asset-near-copies = Почти копии
asset-near-copies-note = То же самое в другом размере, частоте дискретизации или формате. Лучшая копия отмечена.
asset-near-copies-best = То же самое в другом размере, частоте дискретизации или формате. Это лучшая копия.
asset-near-best = лучшая
asset-unreal-converted = Преобразован из Unreal
asset-rule-converted = Преобразован правилом
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Правило преобразования создало этот файл { $format } { $date } из
asset-rule-kept-until = Оригинал хранится до { $date }.
asset-rule-rebuilt = FLAC хранит всё, что было в WAV, так что WAV можно в любой момент скачать или вернуть точно таким, каким он был.
asset-rule-put-back = Вернуть оригинал
asset-rule-put-back-title = Вернуть { $file }?
asset-rule-put-back-body = Преобразованный файл удаляется, а его место занимает оригинал — с тегами, коллекциями и ссылками. После этого правила преобразования его больше не трогают.
asset-rule-put-back-done = { $file } возвращён
asset-rule-let-go = Оригинал удалён, когда истёк срок хранения оригиналов, заданный в настройках.
asset-unreal-package = Из пакета Unreal
asset-ue-needs-editor-before = Ждёт Unreal Editor: { $reason }. На машине с установленным Unreal выполните
asset-ue-needs-editor-after = — и он вернётся в формате FBX.
asset-ue-editor-path = путь к UnrealEditor-Cmd
asset-ue-unsupported = Оставлен как пакет: { $reason }.
asset-ue-failed = Преобразование не удалось: { $reason }
asset-recovered = Исходный .{ $ext } хранился внутри этого пакета, и Hoard его извлёк.
asset-recovered-download = Скачать .{ $ext }
asset-ue-meshes-before = Меши и анимации хранятся в собственном формате Unreal. Преобразуйте пак (страница пака, «Ещё»), затем выполните
asset-ue-meshes-after = рядом с установленным Unreal Editor, чтобы получить FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } тег реестра ассетов
        [few] { $count } тега реестра ассетов
        [many] { $count } тегов реестра ассетов
       *[other] { $count } тега реестра ассетов
    }
asset-file = Файл
asset-bytes = Байты
asset-source = Источник
asset-path = Путь
asset-path-open-folder = Что ещё лежит в { $folder }
asset-download-maps =
    { $count ->
        [one] Скачать { $count } карту
        [few] Скачать все { $count } карты
        [many] Скачать все { $count } карт
       *[other] Скачать все карты ({ $count })
    }
asset-download-linked =
    { $count ->
        [one] Скачать с { $count } связанным файлом
        [few] Скачать с { $count } связанными файлами
        [many] Скачать с { $count } связанными файлами
       *[other] Скачать со связанными файлами ({ $count })
    }
asset-download-only-title = Только этот файл, без файлов, которые он использует
asset-download-only = Только этот файл
asset-include-animations = Включить анимации в .zip
asset-include-character = Добавить персонажа { $name }
asset-copy-toml = Скопировать запись hoard.toml
asset-copy-id = Скопировать ID
asset-teach = Научить Hoard этому файлу
asset-teach-title = Описать этот файл магазина, чтобы Hoard умел его читать («Настройки → Файлы магазинов»)
asset-archive = В архив
asset-archive-title = Не показывается в поиске, пока его не восстановят или не удалят навсегда из архива

## ImageViewer.vue

image-channel-red = Красный канал
image-channel-green = Зелёный канал
image-channel-blue = Синий канал
image-channel-alpha = Альфа-канал
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Фон «шахматка» (K — переключить)
image-bg-black-title = Чёрный фон (K — переключить)
image-bg-gray-title = Серый фон (K — переключить)
image-bg-white-title = Белый фон (K — переключить)
image-bg-checker = фон «шахматка»
image-bg-black = чёрный фон
image-bg-gray = серый фон
image-bg-white = белый фон
image-inspect-aria = Рассмотреть { $name }
image-preview-size = превью { $width }×{ $height }, { $bytes }
image-preview = превью
image-resolution = Разрешение
image-full-title = Загрузить полную картинку { $width }×{ $height } (F); это происходит и при увеличении сверх превью
image-full-loading = Загружаем полное разрешение…
image-full-failed = Полное разрешение не загрузилось: повторить
image-full = Полное разрешение
image-zoom = Масштаб
image-zoom-out-title = Уменьшить (−)
image-zoom-out = Уменьшить
image-zoom-in-title = Увеличить (+)
image-zoom-in = Увеличить
image-fit-title = По размеру (0)
image-fit = По размеру
image-actual-title = Реальный размер (1)
image-view = Вид
image-tile-title = Тайлинг 3×3, чтобы проверить швы (T)
image-tile = Тайлинг
image-channels = Каналы
image-close = Закрыть (Esc)
image-failed = Не удалось загрузить это изображение.
image-loading = Загрузка…
image-key-gestures = Прокрутка или щипок — масштаб · перетаскивание — сдвиг · двойной щелчок — увеличить или вписать
image-key-tile = тайлинг
image-key-background = фон
image-key-full = полное разрешение
image-key-channels = каналы

## SpritePlayer.vue

sprite-frame-aria = { $name }, кадр { $frame } из { $count }
sprite-failed = Это нельзя воспроизвести.
sprite-loading = Загружаем кадры…
sprite-previous = Предыдущий кадр
sprite-next = Следующий кадр
sprite-pause = Приостановить анимацию
sprite-play = Воспроизвести анимацию
sprite-animation = Анимация
sprite-all-frames = Все кадры
sprite-grid-title = Как лист разрезается на кадры
sprite-columns = Столбцы
sprite-rows = Строки
sprite-fps-aria = Кадров в секунду
sprite-fps = к/с

## Просмотрщик листов спрайтов (SheetView.vue) и его форматы скачивания (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } спрайт
        [few] { $count } спрайта
        [many] { $count } спрайтов
       *[other] { $count } спрайта
    }
sheet-pages =
    { $count ->
        [one] { $count } страница
        [few] { $count } страницы
        [many] { $count } страниц
       *[other] { $count } страницы
    }
sheet-animations =
    { $count ->
        [one] { $count } анимация
        [few] { $count } анимации
        [many] { $count } анимаций
       *[other] { $count } анимации
    }
# The box that narrows the list of sprites by name.
sheet-find = Найти спрайт
# Under the list of sprites when more match than it draws.
sheet-showing = Показано { $shown } из { $total }
sheet-no-match = Нет спрайтов с таким текстом в имени.
sheet-failed = Этот лист не удаётся прочитать.
sheet-loading = Загружаем лист…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Собственный формат Hoard: страницы как есть, спрайты повёрнуты как угодно и обрезаны, с анимациями и длительностью каждого кадра.
sheet-note-texturepacker = Спрайты повёрнуты по часовой стрелке и обрезаны, с анимациями по именам — так, как их читают PixiJS и Phaser.
sheet-note-clockwise = Спрайты повёрнуты по часовой стрелке и обрезаны. Анимаций в этом формате нет: кадры различаются по номерам в именах.
sheet-note-libgdx = Спрайты повёрнуты против часовой стрелки и обрезаны. Анимаций в этом формате нет: кадры различаются по номерам в именах.
sheet-note-godot-frames = Спрайты не поворачиваются, обрезанное сохраняется как поля, с анимациями и длительностью каждого кадра.
sheet-note-godot-tileset = Тайлы не поворачиваются и не обрезаются, лежат на сетке размером с тайл; изображения, которые уже на сетке, остаются как есть.
sheet-note-grid = Каждый спрайт без поворота, на всём своём холсте, в ячейках одного размера — для всего, что режет лист по сетке.
sheet-note-frames = Каждый спрайт без поворота, на всём своём холсте, отдельным PNG с его именем.

## TextViewer.vue, TextPreview.vue и text.ts

text-lang-config = Конфигурация
text-lang-table = Таблица
text-lang-code = Код
text-lang-script = Скрипт
text-lang-text = Текст
text-copy-blocked = Не удалось скопировать: браузер это запретил
text-read-aria = Читать { $name }
text-lines =
    { $count ->
        [one] { $count } строка
        [few] { $count } строки
        [many] { $count } строк
       *[other] { $count } строки
    }
text-show-as = Показать как
text-tree-title = Сворачиваемое дерево (T)
text-tree = Дерево
text-pretty-title = С новыми отступами (P)
text-pretty = Форматировать
text-raw-title = Как написано (R)
text-raw = Как есть
text-document-title = Как документ (D)
text-document = Документ
text-source-title = Markdown как написан (R)
text-source = Исходник
text-page-title = Как её показывает браузер из её папки, отдельно от Hoard (P)
text-page = Страница
text-page-source-title = Страница как она написана (R)
text-page-aria = { $name } как страница
text-page-error = Не удалось открыть эту страницу: { $error }
text-fold = Свернуть
text-expand-all-title = Развернуть всё
text-expand-all = Развернуть всё
text-collapse-all-title = Свернуть всё
text-collapse-all = Свернуть всё
text-group-text = Текст
text-wrap-title = Переносить длинные строки (W)
text-wrap = Перенос
text-copy-title = Скопировать текст (C)
text-copied = Скопировано
text-copy = Копировать
text-download-title = Скачать файл
text-download = Скачать
text-close = Закрыть (Esc)
text-truncated = Показаны первые { $shown } из { $total }.
text-truncated-json = Показаны первые { $shown } из { $total } (дереву нужен весь файл).
text-show-all = Показать всё
text-json-error = Этот JSON не разбирается: { $error }
text-contents-aria = Содержимое { $name }
text-loading = Загрузка…
text-load-error = Не удалось загрузить этот файл: { $error }
text-binary = Это двоичный файл, а не текст, так что читать здесь нечего. Скачайте его, чтобы открыть в подходящей программе.
text-key-tree = дерево
text-key-pretty = форматировать
text-key-raw = как есть
text-key-document = документ
text-key-source = исходник
text-key-page = страница
text-key-wrap = перенос
text-key-copy = копировать
text-key-find = найти
text-key-close = закрыть
text-preview-aria = Читать { $name } на весь экран
text-preview-read = Читать

## JsonTree.vue и JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } элемент
        [few] { $count } элемента
        [many] { $count } элементов
       *[other] { $count } элемента
    }
json-keys =
    { $count ->
        [one] { $count } ключ
        [few] { $count } ключа
        [many] { $count } ключей
       *[other] { $count } ключа
    }
json-fold = Свернуть
json-unfold = Развернуть
json-unfold-summary = Развернуть: { $summary }
json-show-more = Показать ещё { $count }
json-left = из оставшихся { $count }
json-picker-more = … ещё { $count }

## 3D-просмотрщики (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = 3D-просмотрщик не загрузился ({ $error }); может помочь перезагрузка страницы
model-loading-viewer = Загружаем 3D-просмотрщик…
model-loading = Загружаем модель…
model-show-failed = Не удалось показать эту модель: { $error }
model-preview-failed = Не удалось сделать превью этой модели: { $error }
model-error-converting = сервер ещё преобразует эту модель
model-error-converting-desktop = Hoard ещё преобразует эту модель
model-error-empty-collada = пустой файл Collada
model-error-no-loader = Браузер не умеет загружать .{ $ext }
model-dialog = Просмотр { $name } в 3D
model-title-stats =
    { $triangles } треуг. · { $meshes ->
        [one] { $meshes } меш
        [few] { $meshes } меша
        [many] { $meshes } мешей
       *[other] { $meshes } меша
    } · { $materials ->
        [one] { $materials } материал
        [few] { $materials } материала
        [many] { $materials } материалов
       *[other] { $materials } материала
    }
model-triangles =
    { $count ->
        [one] { $count } треугольник
        [few] { $count } треугольника
        [many] { $count } треугольников
       *[other] { $count } треугольника
    }
model-group-camera = Камера
model-group-shading = Затенение
model-group-background = Фон
model-group-animation = Анимация
model-group-view = Вид
model-group-show = Показ
model-frame-title = Показать модель целиком (F)
model-frame = Вписать
model-reset-title = Вернуться к начальному виду (R)
model-reset = Сброс
model-turn-title = Медленно вращать (A)
model-turn = Вращать
model-textures-title = Текстуры (T)
model-textures = Текстуры
model-full-res-title = Текстуры в полном разрешении (H): каждый файл текстуры как есть, каким бы большим он ни был; если выключено — облегчённые превью до 2048 пкс
model-full-res = Полное разр.
model-light-title = Освещение окружения (E)
model-light = Свет
model-grid-title = Сетка пола (G)
model-grid = Сетка
model-bg-checker = Фон «шахматка»
model-bg-dark = Тёмный фон
model-bg-gray = Серый фон
model-bg-light = Светлый фон
model-bg-env = Фон окружения
model-bg-title = { $background } (K — переключить)
model-pause-title = Пауза (пробел)
model-play-title = Воспроизвести (пробел)
model-pause = Пауза
model-play = Воспроизвести
model-animation-clip = Клип анимации
model-animation = Анимация
model-plays-on = Воспроизводить на
model-rest-pose = Поза покоя
model-close-title = Закрыть (Esc)
model-full-screen = На весь экран
model-open-full-screen = Открыть полноэкранный 3D-просмотрщик
model-menu-fold = Свернуть меню
model-menu-show = Показать меню
model-view-prefix = Вид:{ " " }
model-menu-render = Рендер
model-menu-maps = Карты
model-menu-inspect = Анализ
model-mode-final = Итоговый рендер
model-mode-raw = Без обработки
model-mode-color = Базовый цвет
model-mode-roughness = Шероховатость
model-mode-metalness = Металличность
model-mode-normal = Нормали
model-mode-ao = Затенение
model-mode-matcap = Matcap
model-mode-uv = Проверка UV
model-mode-geometry = Геометрии
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Каркас (W) поверх любого вида
model-wireframe = Каркас
model-info-title = Сведения о сцене (I)
model-info = Сведения о сцене
model-info-close = Закрыть сведения о сцене
model-info-close-title = Закрыть (I)
model-info-geometries = Геометрии
model-info-materials = Материалы
model-info-textures = Текстуры
model-info-triangles = Треугольники
model-info-vertices = Вершины
model-info-meters = glTF задаётся в метрах
model-info-no-units = Этот формат не указывает свои единицы измерения
model-info-height = Высота
model-info-length = Длина
model-info-width = Ширина
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } единица
        [few] { NUMBER($n, maximumSignificantDigits: 3) } единицы
        [many] { NUMBER($n, maximumSignificantDigits: 3) } единиц
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } единицы
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } м
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } см
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } мм
model-missing-one = Текстура не найдена: { $name }
model-missing-some =
    { $count ->
        [one] Не найдена { $count } текстура: { $names }
        [few] Не найдены { $count } текстуры: { $names }
        [many] Не найдено { $count } текстур: { $names }
       *[other] Не найдено { $count } текстуры: { $names }
    }
model-missing-more =
    { $count ->
        [one] Не найдена { $count } текстура: { $names }…
        [few] Не найдены { $count } текстуры: { $names }…
        [many] Не найдено { $count } текстур: { $names }…
       *[other] Не найдено { $count } текстуры: { $names }…
    }
model-missing-count =
    { $count ->
        [one] Не найдена { $count } текстура
        [few] Не найдены { $count } текстуры
        [many] Не найдено { $count } текстур
       *[other] Не найдено { $count } текстуры
    }
model-guessed = Текстуры подобраны по названию; сам файл их не называет.
model-group-facing = Ориентация
model-facing-turn = Повернуть на четверть
model-facing-turn-title = Повернуть модель на четверть оборота против часовой стрелки, если смотреть сверху
model-facing-tip = Наклонить на четверть
model-facing-tip-title = Наклонить верх модели на четверть оборота вперёд, чтобы поставить лежащую модель
model-facing-save = Применить к паку
model-facing-save-title = Повернуть так все модели в паке { $pack }. Их миниатюры будут перерисованы; загрузки останутся такими, какими были созданы.
model-facing-undo = Отменить
model-facing-undo-title = Вернуться к тому, как повёрнуты модели пака
model-facing-saved = Все модели в паке { $pack } теперь повёрнуты так. Их миниатюры перерисовываются.
model-facing-failed = Не удалось повернуть модели пака: { $error }
model-keys-mouse = Перетаскивание — вращение · правой кнопкой — сдвиг · прокрутка — масштаб · двойной щелчок — вписать
model-key-frame = вписать
model-key-reset = сброс
model-key-view = вид
model-key-wireframe = каркас
model-key-info = сведения о сцене
model-key-textures = текстуры
model-key-full-res = текстуры в полном разрешении
model-key-light = свет
model-key-grid = сетка
model-key-background = фон
model-key-play = воспроизвести

## Материалы (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Цвет
material-role-normal = Нормали
material-role-normal-dx = Нормали (DirectX)
material-role-roughness = Шероховатость
material-role-gloss = Глянец
material-role-metalness = Металличность
material-role-ao = Фоновое затенение
material-role-orm = Затенение, шероховатость, металл
material-role-mra = Металл, шероховатость, затенение
material-role-rma = Шероховатость, металл, затенение
material-role-height = Высота
material-role-bump = Рельеф
material-role-cavity = Впадины
material-role-specular = Отражение
material-role-opacity = Непрозрачность
material-role-emissive = Свечение
material-role-translucency = Просвечивание
material-role-base-color-name = цвета
material-role-normal-name = нормалей
material-role-normal-dx-name = нормалей (DirectX)
material-role-roughness-name = шероховатости
material-role-gloss-name = глянца
material-role-metalness-name = металличности
material-role-ao-name = фонового затенения
material-role-orm-name = затенения, шероховатости, металла
material-role-mra-name = металла, шероховатости, затенения
material-role-rma-name = шероховатости, металла, затенения
material-role-height-name = высот
material-role-bump-name = рельефа
material-role-cavity-name = впадин
material-role-specular-name = отражения
material-role-opacity-name = непрозрачности
material-role-emissive-name = свечения
material-role-translucency-name = просвечивания
material-maps =
    { $count ->
        [one] { $count } карта
        [few] { $count } карты
        [many] { $count } карт
       *[other] { $count } карты
    }
material-dialog = Просмотр материала { $name }
material-up-to = до { $size }
material-repeat-size = { $size } на повтор
material-shape-sphere = Сфера
material-shape-cube = Куб
material-shape-plane = Плоскость
material-shape-title = { $shape } ({ $key })
material-group-shape = Форма
material-group-tiling = Тайлинг
material-real-title = Реальный размер: один повтор — { $size }, сфера 2 м в диаметре (T — переключить)
material-repeat-title = Повтор { $n }× (T — переключить)
material-real = Реальный
material-repeat = { $n }×
material-height-title = Насколько карта высот выдавливает поверхность
material-height = Высота
material-height-strength = Сила высоты
material-loading = Загружаем материал…
material-show-failed = Не удалось показать этот материал: { $error }
material-preview-failed = Не удалось сделать превью этого материала: { $error }
material-group-maps = Карты
material-map-show = Показать карту { $map }
material-map-hide = Скрыть карту { $map }
material-map-unused = Не используется в превью
material-open-full-screen = Открыть полноэкранный просмотрщик материалов
material-keys-mouse = Перетаскивание — вращение · правой кнопкой — сдвиг · прокрутка — масштаб · щелчок по карте — выключить её
material-key-shape = форма
material-key-tiling = тайлинг
material-key-turn = вращать

## theme.ts

theme-slate = Сланец
theme-paper = Бумага
theme-linen = Лён
theme-sage = Шалфей
theme-lilac = Сирень
theme-blush = Румянец
theme-graphite = Графит
theme-obsidian = Обсидиан
theme-ember = Угли
theme-moss = Мох
theme-aubergine = Баклажан
theme-oxblood = Бордо
theme-light-slate-blurb = Холодный сине-серый
theme-light-paper-blurb = Простой нейтральный серый
theme-light-linen-blurb = Тёплый кремовый, янтарь
theme-light-sage-blurb = Мягкий зелёный
theme-light-lilac-blurb = Бледно-фиолетовый
theme-light-blush-blurb = Пыльная роза
theme-dark-graphite-blurb = Нейтральный серый без оттенка
theme-dark-slate-blurb = Сине-серый
theme-dark-obsidian-blurb = Почти чёрный, для OLED
theme-dark-ember-blurb = Тёплый угольный, янтарь
theme-dark-moss-blurb = Лесной зелёно-чёрный
theme-dark-aubergine-blurb = Сливово-чёрный, сирень
theme-dark-oxblood-blurb = Винно-чёрный, коралл

## texnames.ts

texnames-needs-map = В шаблоне нужен { "{" }map{ "}" }, иначе все карты материала получат одно и то же название
texnames-placeholders = В шаблоне можно использовать { "{" }name{ "}" }, { "{" }map{ "}" } и { "{" }size{ "}" }
texnames-bad-characters = В названиях не может быть < > : " / \ | ? или *
texnames-no-suffix = Задайте суффикс для карт { $role }
texnames-bad-suffix = В суффиксе для карт { $role } не может быть < > : " / \ | ? или *
texnames-slot-no-suffix = Задайте суффикс для «{ $slot }»
texnames-slot-bad-suffix = В суффиксе для «{ $slot }» не может быть < > : " / \ | ? или *
texnames-ending-missing = Для каждого своего окончания нужно указать, какое окончание искать
texnames-ending-bad = В своих окончаниях не может быть < > : " / \ | ? или *
texnames-ending-twice = { $ending } встречается в ваших окончаниях дважды
texnames-slot-metallic-roughness = Металл и шероховатость
texnames-slot-specular-color = Цвет отражения
texnames-slot-specular-glossiness = Отражение и глянец
texnames-slot-clearcoat = Лак
texnames-slot-clearcoat-roughness = Шероховатость лака
texnames-slot-clearcoat-normal = Нормали лака
texnames-slot-sheen-color = Цвет отлива
texnames-slot-sheen-roughness = Шероховатость отлива
texnames-slot-transmission = Пропускание
texnames-slot-thickness = Толщина
texnames-slot-diffuse-transmission = Диффузное пропускание
texnames-slot-diffuse-transmission-color = Цвет диффузного пропускания
texnames-slot-iridescence = Радужность
texnames-slot-iridescence-thickness = Толщина радужности
texnames-slot-anisotropy = Анизотропия

## storeFormats.ts

storefiles-field-title = Название
storefiles-field-title-hint = Точное название товара. Оно даёт имя паку, а без него файл пропускается.
storefiles-field-seller = Продавец
storefiles-field-seller-hint = Кто сделал или продаёт.
storefiles-field-license = Лицензия
storefiles-field-license-hint = Как написано в файле, например CC-BY-4.0.
storefiles-field-url = Страница в магазине
storefiles-field-url-hint = Адрес товара, начинается с http:// или https://.
storefiles-field-words = Теги
storefiles-field-words-hint = Слова для тегов. Значения через запятую разделяются.
storefiles-field-type = Вид
storefiles-field-type-hint = Что это, например «3D model» или «surface». Тоже становится тегом.
storefiles-field-colors = Цвета
storefiles-field-colors-hint = Его цвета в виде тегов color:.
storefiles-field-id = ID в магазине
storefiles-field-id-hint = ID товара в магазине: буквы, цифры, _ или -.
storefiles-field-origin = Где сделано
storefiles-field-origin-hint = Где это сделано или отсканировано, для описания.
storefiles-field-size = Размер
storefiles-field-size-hint = Насколько он большой, словами, для описания.
storefiles-field-title-name = название
storefiles-field-seller-name = продавец
storefiles-field-license-name = лицензия
storefiles-field-url-name = страница в магазине
storefiles-field-words-name = теги
storefiles-field-type-name = вид
storefiles-field-colors-name = цвета
storefiles-field-id-name = ID в магазине
storefiles-field-origin-name = где сделано
storefiles-field-size-name = размер

## StoreFormatsPanel.vue

storefiles-load-failed = Не удалось загрузить форматы файлов магазинов: { $error }
storefiles-remove-title = Удалить { $name }?
storefiles-remove-body = { $file } удаляется из папки данных. Паки, уже прочитанные с ним, сохраняют то, что он прочитал.
storefiles-remove = Удалить
storefiles-removed = { $name } удалён.
storefiles-remove-failed = Не удалось удалить { $name }: { $error }
storefiles-unread = Файлы, которые Hoard не читает
storefiles-loading = Загрузка…
storefiles-unread-none = Нет: ни один файл в библиотеке не похож на файл неизвестного магазина.
storefiles-unread-what = Файлы, которые есть больше чем в одном паке или названы как файлы магазинов, но которые не читает ни один формат.
storefiles-unread-packs =
    { $count ->
        [one] { $count } пак
        [few] { $count } пака
        [many] { $count } паков
       *[other] { $count } пака
    }
storefiles-unread-example = например, { $pack }
storefiles-teach = Научить Hoard этому файлу
storefiles-formats = Форматы
storefiles-formats-what = Ваши форматы пробуются первыми и заменяют встроенный формат с тем же названием.
storefiles-dir-before = Они хранятся в
storefiles-dir-after = .
storefiles-yours = Ваш
storefiles-built-in = Встроенный
storefiles-edit = Изменить
storefiles-start-from = Взять за основу

## TagsPanel.vue

tags-load-failed = Не удалось загрузить теги: { $error }
tags-rename-title = Переименовать { $tag }?
tags-merge-title =
    { $count ->
        [one] Объединить { $count } тег?
        [few] Объединить { $count } тега?
        [many] Объединить { $count } тегов?
       *[other] Объединить { $count } тега?
    }
tags-rename-body =
    { $count ->
        [one] Его { $count } ассет получит вместо него { $into }, а разметчики с этого момента будут использовать новое название.
        [few] Его { $count } ассета получат вместо него { $into }, а разметчики с этого момента будут использовать новое название.
        [many] Его { $count } ассетов получат вместо него { $into }, а разметчики с этого момента будут использовать новое название.
       *[other] Его { $count } ассета получат вместо него { $into }, а разметчики с этого момента будут использовать новое название.
    }
tags-merge-body =
    { $count ->
        [one] { $tags } станут { $into } не более чем у { $count } ассета. Разметчики с этого момента будут использовать { $into }.
        [few] { $tags } станут { $into } не более чем у { $count } ассетов. Разметчики с этого момента будут использовать { $into }.
        [many] { $tags } станут { $into } не более чем у { $count } ассетов. Разметчики с этого момента будут использовать { $into }.
       *[other] { $tags } станут { $into } не более чем у { $count } ассета. Разметчики с этого момента будут использовать { $into }.
    }
tags-rename = Переименовать
tags-merge = Объединить
tags-merged =
    { $count ->
        [one] Теперь { $count } ассет помечен тегом { $into }
        [few] Теперь { $count } ассета помечены тегом { $into }
        [many] Теперь { $count } ассетов помечены тегом { $into }
       *[other] Теперь { $count } ассета помечены тегом { $into }
    }
tags-merge-failed = Не удалось объединить: { $error }
tags-forget-failed = Не удалось это изменить: { $error }
tags-find = Найти теги
tags-find-placeholder = Найдите теги, например rock или set:
tags-count =
    { $count ->
        [one] { $count } тег
        [few] { $count } тега
        [many] { $count } тегов
       *[other] { $count } тега
    }
tags-count-more = { $count }+ тегов
tags-what = Выберите теги, которые означают одно и то же, и объедините их, или выберите один, чтобы переименовать.
tags-new-name = Новое название
tags-merge-into = Объединить в
tags-saved-as-before = Сохранён как
tags-saved-as-after = { "" }
tags-cancel = Отмена
tags-loading = Загрузка…
tags-none = Тегов нет.
tags-none-containing = Нет тегов, содержащих «{ $text }».
tags-browse-title = Открыть ассеты с тегом { $tag }
tags-browse = Открыть
tags-merged-heading = Объединённые теги
tags-merged-what = Когда разметчики придумывают одно из этих старых названий, ассет получает тег, с которым оно было объединено.
tags-allow-old = Снова разрешить старое название
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } ассет
        [few] { $count } ассета
        [many] { $count } ассетов
       *[other] { $count } ассета
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Старое название: { $names }
        [few] Старые названия: { $names }
        [many] Старые названия: { $names }
       *[other] Старые названия: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } объединён с { $into }.
tag-page-show = Показать { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Напишите, что означает этот тег…
tag-page-describe-label = Что означает этот тег
tag-page-describe-placeholder = Для чего он нужен, что к нему относится. Enter сохраняет.
tag-page-describe-edit = Нажмите, чтобы изменить
tag-page-description-saved = Значение тега сохранено
# The tag's picture: one of its assets stands for it.
tag-page-picture = Выбрать картинку для этого тега
tag-page-picture-help = Выберите один из его ассетов, который будет его представлять
tag-page-picture-nothing = Ни у одного из показанных ассетов нет картинки.
tag-page-picture-none = Без картинки
tag-page-picture-saved = Картинка задана
tag-page-picture-removed = Картинка убрана
tag-page-rename = Переименовать…
tag-page-rename-title = Дайте ему новое название или название другого тега, чтобы объединить с ним
tag-page-rename-label = Новое название
tag-page-save-failed = Не удалось сохранить тег: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Под
tag-page-parent-add = Поместить под…
tag-page-parent-add-title = Поместите этот тег под более общий: при поиске общего найдутся и эти
tag-page-parent-placeholder = Более общий тег, например транспорт
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Убрать { $tag } из-под { $parent }
tag-page-parent-saved = { $tag } теперь под { $parent }
tag-page-parent-removed = { $tag } больше не под { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Более узкие:
tag-page-narrower-title = При поиске { $tag } найдутся и эти
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Общие и узкие
tags-tree-what = Теги, помещённые под более общие на своих страницах. При поиске тега находится и всё, что под ним.

## LibraryStats.vue

stats-source-human = Добавлены людьми
stats-source-ai = Предложены ИИ
stats-source-rule = Встроенные правила
stats-source-meta = Из файлов
stats-assets = Ассеты
stats-packs = Паки
stats-on-disk = На диске
stats-duplicates-title =
    { $count ->
        [one] { $count } файл — побайтная копия другого и хранится один раз
        [few] { $count } файла — побайтные копии других и хранятся один раз
        [many] { $count } файлов — побайтные копии других и хранятся один раз
       *[other] { $count } файла — побайтные копии других и хранятся один раз
    }
stats-saved-dedup = Сэкономлено дедупликацией
stats-distinct-tags = Разных тегов
stats-archive = В архиве
stats-load-failed = Не удалось загрузить сведения: { $error }
stats-counting = Подсчёт…
stats-as-of = По состоянию на { $time }
stats-updating = Обновляется…
stats-by-kind = Место по видам
stats-files-size =
    { $count ->
        [one] { $name }: { $count } файл, { $size }
        [few] { $name }: { $count } файла, { $size }
        [many] { $name }: { $count } файлов, { $size }
       *[other] { $name }: { $count } файла, { $size }
    }
stats-months = Добавлено за последний год
stats-month =
    { $count ->
        [one] { $month }: { $count } ассет, { $size }
        [few] { $month }: { $count } ассета, { $size }
        [many] { $month }: { $count } ассетов, { $size }
       *[other] { $month }: { $count } ассета, { $size }
    }
stats-tagging = Теги
stats-tagged = С тегами
stats-untagged = { $count } без тегов
stats-described = С описанием
stats-described-assets =
    { $count ->
        [one] { $count } ассет
        [few] { $count } ассета
        [many] { $count } ассетов
       *[other] { $count } ассета
    }
stats-human-edited = Изменены вручную
stats-human-edited-unit =
    { $count ->
        [one] ассет
        [few] ассета
        [many] ассетов
       *[other] ассета
    }
stats-source-different = { $count } разных
stats-most-used = Самые частые
stats-show-tag = Показать ассеты с тегом { $tag }
stats-file-types = Типы файлов
stats-biggest-packs = Самые большие паки
stats-browse-pack = Открыть { $name }
stats-largest-files = Самые большие файлы
stats-open-file = Открыть { $name }
stats-empty = В библиотеке пока ничего нет. Добавьте пак, и здесь появятся его цифры.

## HoardPanel.vue

hoard-title = Ваш клад
hoard-what = Каждый ассет в библиотеке — это предмет клада, и его вид определяет, какой. Соберите новое, чтобы добавить его в свою груду.
hoard-sign-in = Войдите, чтобы собирать собственный клад.
hoard-new = Новая добыча для сбора: { $count }
hoard-none-new = Собирать пока нечего.
hoard-collect = Собрать
hoard-unavailable = Этот браузер не может нарисовать клад: { $error }
hoard-collect-failed = Не удалось сохранить собранное: { $error }
hoard-arcade = Экран игрового автомата
hoard-full-screen = Во весь экран
hoard-leave-full-screen = Выйти из полноэкранного режима
hoard-label = Дракон охраняет свой клад. Сокровищ: { $count }
hoard-gold = Золото
hoard-gems = Самоцветы
hoard-books = Книги
hoard-weapons = Оружие
hoard-bones = Кости
hoard-tablets = Рунические таблички
hoard-paintings = Картины
hoard-potions = Зелья

## UpdatesPanel.vue

updates-saved = Hoard вызовет его, когда будет готово обновление
updates-saved-none = Адрес для вызова не задан
updates-save-failed = Не удалось сохранить
updates-call-answered = Ответ: { $result }
updates-call-refused = Запрос не принят: { $result }
updates-call-failed = Не удалось вызвать
updates-called = Вызван { $when } для { $version }: { $result }
updates-called-failed = Вызван { $when } для { $version }: { $result }. Hoard попробует снова через час.
updates-latest = Hoard { $version } — последний выпуск.
updates-out = Вышел Hoard { $version }.
updates-out-idle = Вышел Hoard { $version }; вызов будет в ближайшие пару минут, пока ничего не происходит.
updates-out-busy = Вышел Hoard { $version }; ждём, пока ничего не будет происходить ({ $work }).
updates-busy-more = ещё { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } задача в очереди
        [few] { $count } задачи в очереди
        [many] { $count } задач в очереди
       *[other] { $count } задачи в очереди
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } загрузка
        [few] { $count } загрузки
        [many] { $count } загрузок
       *[other] { $count } загрузки
    }
updates-list-separator = ,{ " " }
updates-what = Когда выходит новый выпуск, Hoard может вызвать адрес, как только ничего не происходит (нет загрузок, задач, преобразований или обслуживания), чтобы то, что обновляет его контейнер, не перезапустило его посреди загрузки. Он отправляет POST с выпуском в теле JSON. Для этого нужна проверка выпусков выше.
updates-url = Адрес для вызова
updates-url-placeholder = Нет (например, http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Сохранено (введите, чтобы заменить)
updates-auth-placeholder = Нет (например, Bearer my-token)
updates-remove = Удалить
updates-save = Сохранить
updates-call-now-title = Запустится то, к чему он подключён
updates-call-now = Вызвать сейчас
updates-what-calls = Что можно вызывать?
updates-watchtower = HTTP API Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , с{ " " }
updates-watchtower-token = { " " }и токеном из{ " " }
updates-watchtower-its = { " " }(эндпоинт{ " " }
updates-watchtower-endpoint = { " " }должен быть включён).
updates-portainer = Вебхук Portainer
updates-portainer-rest = { " " }на стеке или контейнере Hoard с включённым re-pull image. Токен — в URL.
updates-script = Скрипт на хосте
updates-script-listener = { " " }за слушателем вебхуков (например,{ " " }
updates-script-runs = ), который выполняет{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }или что угодно ещё, что принимает вебхуки.
updates-poll-before = Или обойдитесь без адреса: скрипт с токеном доступа может опрашивать{ " " }
updates-poll-until = , пока{ " " }
updates-poll-and = { " " }не задан, а{ " " }
updates-poll-after = { " " }не равен true, а затем скачать новую версию и перезапустить Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Не удалось загрузить обслуживание
maintenance-reanalyze-title = Проанализировать все паки заново?
maintenance-reanalyze-body = Каждый файл будет прочитан заново, и на большой библиотеке это может занять много времени; паки показываются как «в работе», пока не дойдёт их очередь. Ваши названия, теги и заметки сохранятся.
maintenance-reanalyze-confirm = Проанализировать заново
maintenance-gc-title = Удалить неиспользуемые файлы?
maintenance-gc-body = Сохранённые файлы, на которые ничто в библиотеке не ссылается, удаляются навсегда. Паки в архиве сохраняют свои файлы.
maintenance-gc-confirm = Удалить их
maintenance-start-failed = Не удалось запустить «{ $task }»
maintenance-just-now = только что
maintenance-minutes-ago = { $count } мин назад
maintenance-hours-ago = { $count } ч назад
maintenance-took-seconds = { $count } с
maintenance-took-minutes = { $count } мин
maintenance-took-hours = { $hours } ч
maintenance-loading = Загрузка…
maintenance-running = Выполняется…
maintenance-starting = Запуск…
maintenance-run = Запустить
maintenance-stop = Остановить
maintenance-stopping = Останавливается…
maintenance-stop-failed = Не удалось остановить «{ $task }»
maintenance-started = { $detail } · запущено { $when }
maintenance-starting-detail = Запуск
maintenance-finished = Завершено:{ " " }
maintenance-failed = Ошибка:{ " " }
maintenance-never-run = Ещё не запускалось
maintenance-history = Прошлые запуски
maintenance-col-task = Задача
maintenance-col-finished = Завершено
maintenance-col-took = Длительность
maintenance-col-result = Результат
maintenance-script-before = Резервное копирование и проверку на повреждения можно запускать и из скрипта:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }и{ " " }
maintenance-script-after = . Такие запуски тоже показываются здесь.

## SettingsPage.vue

settings-title = Настройки
settings-loading = Загрузка…
settings-find = Найти настройку
settings-find-none = Нет настроек, подходящих под «{ $query }».
settings-load-failed = Не удалось загрузить настройки: { $error }
settings-page-appearance = Оформление
settings-page-appearance-blurb = Темы, которые этот браузер использует в светлом и в тёмном режиме. Сохраняется только в этом браузере.
settings-page-dragon = Дракон
settings-page-dragon-blurb = Сокровищница вашего дракона: каждый ассет в библиотеке — часть его сокровищ.
settings-page-library = Библиотека
settings-page-library-blurb = Что есть в Hoard: какой он большой, из чего состоит, как рос и насколько хорошо размечен тегами. Содержимое архива не учитывается.
settings-page-copies = Почти копии
settings-page-copies-blurb = Файлы, которые являются тем же изображением, звуком или моделью в другом размере, частоте дискретизации или формате; сначала те, что занимают больше всего места. Лучшая в каждом наборе отмечена; ничего не удаляется.
copies-loading = Поиск почти копий…
copies-refreshing = Библиотека изменилась с тех пор, как они были найдены. Повторный поиск…
copies-none = Почти копий нет: каждое изображение, звук и модель здесь только один раз.
copies-load-failed = Не удалось показать почти копии: { $error }
copies-sum = Наборов почти копий: { $count }. Копии, которые не лучшие, занимают { $size }.
copies-set = Набор { $n }
copies-set-wasted = { $size } в копиях, которые не лучшие
copies-more = Показать ещё (осталось { $count })
settings-page-tags = Теги
settings-page-tags-blurb = Все теги библиотеки. Объединяйте теги с одинаковым значением или переименовывайте; разметчики с этого момента используют новое название.
settings-page-store-files = Файлы магазинов
settings-page-store-files-blurb = Файлы, которые магазины кладут рядом с контентом и которые Hoard читает, чтобы назвать паки и указать их авторов. Научите его тем, которых он не знает, и поделитесь тем, чему научили.
settings-page-locations = Места хранения
settings-page-locations-blurb = Папки на сервере, в которых лежит библиотека: те, где Hoard хранит файлы, и папки с паками, которые он индексирует на месте.
settings-page-locations-blurb-desktop = Папки, в которых лежит библиотека: те, где Hoard хранит файлы, и папки с паками, которые он индексирует на месте.
settings-page-tools = Инструменты
settings-page-tools-blurb = Внешние программы, которыми Hoard преобразует и читает файлы.
settings-page-ai = Теги от ИИ
settings-page-ai-blurb = ИИ-модель называет, описывает и размечает тегами новые паки в дополнение к встроенным правилам. Её теги помечаются как предложенные.
settings-page-search-blurb = Ищите картинки по тому, что на них изображено, а звуки — по тому, как они звучат, а не только по названиям, и находите похожие по сути, а не только по цвету.
settings-page-search-tab = Поиск
settings-page-adding = Добавление паков
settings-page-adding-blurb = Что Hoard делает с паком при добавлении, кроме чтения и расстановки тегов.
settings-page-downloads = Скачивание
settings-page-downloads-blurb = Как называются файлы в том, что отдаёт Hoard: при скачивании, в zip, в инструменте hoard и для ИИ-ассистентов. Файлы в библиотеке остаются как есть.
settings-page-account = Учётная запись
settings-page-account-blurb = Ваш пароль, другие люди, которые могут входить, и токены доступа для скриптов и других программ.
settings-page-auth = Аутентификация
settings-page-auth-blurb = Как входят пользователи: собственные учётные записи Hoard, каталог LDAP, Microsoft Active Directory или провайдер OAuth, сколько длятся сеансы и какие группы получают какую роль.
settings-page-connection = Подключение
settings-page-connection-blurb = Как браузеры и приложения обращаются к Hoard: адрес, по которому он известен, и HTTPS, который нужен некоторым функциям браузера.
settings-page-performance = Производительность
settings-page-performance-blurb = Насколько интенсивно работает сервер, как создаются и хранятся миниатюры и сколько он принимает максимум.
settings-page-performance-blurb-desktop = Насколько интенсивно работает Hoard, как создаются и хранятся миниатюры и сколько он принимает максимум.
settings-page-issues = Проблемы
settings-page-issues-blurb = Что пошло не так при открытии или обработке файлов: архивы, которые не распаковались, файлы, которые не удалось прочитать, задачи, завершившиеся ошибкой. Повторите их, когда причина устранена, или уберите.
settings-page-maintenance = Обслуживание
settings-page-maintenance-blurb = Резервное копирование, проверка на повреждения и уборка. Каждая задача выполняется в одном экземпляре, и пока они идут, эту страницу можно покинуть.
settings-page-about = О программе
settings-page-about-blurb = Что такое Hoard, какая это версия и чужой труд, из которого он сделан.
settings-the-default = значение по умолчанию
settings-default-on = вкл.
settings-default-off = выкл.
settings-default-auto = автоматически
settings-option-saved = { $name }: { $value }
settings-back-to-default = Вернуть по умолчанию
settings-save = Сохранить
settings-use = Использовать
settings-status-off = Выкл.

## SettingsPage.vue: оформление

settings-themes = Темы
settings-look-light = Светлая
settings-look-dark = Тёмная
settings-look-group =
    { $scheme ->
        [light] Светлая тема
       *[dark] Тёмная тема
    }
settings-preview =
    Показана { $preview ->
        [light] светлая
       *[dark] тёмная
    } тема, чтобы вы могли её посмотреть. Вы используете { $used ->
        [light] светлую
       *[dark] тёмную
    }{ $system ->
        [yes] { " " }(как в системе)
       *[other] { "" }
    }, и она вернётся, когда вы уйдёте с этой страницы.
settings-preview-back =
    Вернуться к { $used ->
        [light] светлой
       *[dark] тёмной
    }
settings-look-hint = Переключайтесь между светлой и тёмной темой (или следуйте системе) на верхней панели.

## SettingsPage.vue: инструменты

settings-blender-what = Преобразует модели .blend, .dae, .abc и .x3d в glTF в фоне, чтобы у них были миниатюры и 3D-превью. Паки, добавленные, пока он был выключен, преобразуются, как только он будет готов.
settings-blender-download = Использует установленный Blender или скачивает Blender 4.5 LTS (около 400 МБ) с зеркала blender.org.
settings-ffmpeg-what = Кодирует скачиваемые MP3, OGG и Opus и читает аудио, которое Hoard не может декодировать сам (AAC, M4A, WMA, …).
settings-ffmpeg-download = Использует системный ffmpeg с нужными кодировщиками или скачивает статическую сборку (около 80 МБ).
settings-tool-reset = { $tool }: вернуть по умолчанию
settings-tool-no-download = Использует установленный { $tool }. Для этой машины нет сборки для скачивания, поэтому установите его или укажите путь.
settings-tool-path = Путь к программе { $tool }
settings-mode-off = Выкл.
settings-mode-auto = Автоматически
settings-mode-path = Эта программа
settings-tool-looking = Ищем…
settings-tool-downloading = Скачиваем… прогресс — в индикаторе активности
settings-tool-ready = Готово: { $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = Инструменты, которыми проигрываются и преобразуются файлы MIDI.
settings-soundfont-file = Файл SoundFont
settings-soundfont-placeholder = Найденный на сервере или встроенный
settings-soundfont-placeholder-desktop = Найденный на этом компьютере или встроенный
settings-soundfont-default = найденный на сервере или встроенный
settings-soundfont-default-desktop = найденный на этом компьютере или встроенный
settings-soundfont-send = Выбрать файл…
settings-soundfont-sent = Теперь MIDI играет с { $name }

## SettingsPage.vue: поиск по картинкам

settings-clip-heading = Поиск по содержимому картинок
settings-clip-reset = Поиск по картинкам: вернуть по умолчанию
settings-clip-saved-on = Поиск по картинкам: вкл.
settings-clip-saved-off = Поиск по картинкам: выкл.
settings-clip-what = Введите, что ищете («mossy rock», «sci-fi door», «wooden barrel»), и картинки, на которых это есть, появятся рядом с названиями, где есть эти слова. «Похожие» находит вещи того же рода, а не только тех же цветов.
settings-clip-toggle = Включить поиск по картинкам
settings-clip-hint = Использует модель SigLIP 2 от Google, которая работает на процессоре этого сервера: ничего наружу не уходит. При включении модель ({ $size } МБ) скачивается с Hugging Face, а затем в фоне с низким приоритетом изучает каждую картинку. Во время работы или поиска она занимает 350–750 МБ памяти. При выключении модель удаляется, а то, что она изучила, сохраняется на следующий раз.
settings-clip-hint-desktop = Использует модель SigLIP 2 от Google, которая работает на процессоре этого компьютера: ничего наружу не уходит. При включении модель ({ $size } МБ) скачивается с Hugging Face, а затем в фоне с низким приоритетом изучает каждую картинку. Пока она работает или вы ищете, она занимает 350–750 МБ памяти. При выключении модель удаляется, а то, что она изучила, сохраняется на следующий раз.
settings-clip-looking = Ищем модель…
settings-clip-downloading = Скачиваем модель… прогресс — в индикаторе активности
settings-clip-ready-empty = Готово. Картинки изучаются по мере добавления паков.
settings-clip-ready-all =
    { $count ->
        [one] Готово: изучена { $count } картинка
        [few] Готово: изучены все { $count } картинки
        [many] Готово: изучены все { $count } картинок
       *[other] Готово: изучены все картинки ({ $count })
    }
settings-clip-learning = Готово: изучаем картинки, пока { $done } из { $total }
settings-clip-relearning = Пока новая модель не изучит все изображения, поиск использует то, что изучила старая.
settings-clip-unavailable = Не удалось получить модель: { $detail }. Пробуем снова каждый час.
settings-clip-where = Где работает
settings-clip-here = На этом сервере: ничего не уходит наружу
settings-clip-here-desktop = На этом компьютере: ничего не уходит наружу
settings-clip-hosted = Во внешнем сервисе
settings-clip-hosted-warning = Картинки покидают этот сервер. Миниатюра каждой картинки в библиотеке и слова каждого поиска по картинкам отправляются в { $service }, который берёт за них плату и хранит их так, как сказано в его условиях.
settings-clip-hosted-warning-desktop = Картинки покидают этот компьютер. Миниатюра каждой картинки в библиотеке и слова каждого поиска по картинкам отправляются в { $service }, который берёт за них плату и хранит их так, как сказано в его условиях.
settings-clip-hosted-hint = Здесь ничего не скачивается и не запускается: сервис изучает каждую картинку партиями. Другой сервис или модель изучает их все заново, а то, что изучил каждый, сохраняется, чтобы можно было вернуться.
settings-clip-saved-here = Поиск по картинкам: работает на этом сервере
settings-clip-saved-here-desktop = Поиск по картинкам: работает на этом компьютере
settings-clip-saved-hosted = Поиск по картинкам: работает в { $service }
settings-clip-hosted-unavailable = Не удалось воспользоваться { $service }: { $detail }. Пробуем снова каждый час.
settings-clap-heading = Поиск по звучанию
settings-clap-reset = Поиск по звукам: вернуть по умолчанию
settings-clap-saved-on = Поиск по звукам: вкл.
settings-clap-saved-off = Поиск по звукам: выкл.
settings-clap-what = Введите, что ищете («distant thunder», «glassy UI blip», «footsteps on gravel»), и звуки, которые так звучат, появятся рядом с названиями, где есть эти слова.
settings-clap-toggle = Включить поиск по звукам
settings-clap-hint = Использует модель CLAP от LAION, которая работает на процессоре этого сервера: ничего наружу не уходит. При включении модель ({ $size } МБ) скачивается с Hugging Face, а затем в фоне с низким приоритетом прослушивает начало каждого звука. Во время работы или поиска она занимает примерно до 1 ГБ памяти. При выключении модель удаляется, а то, что она изучила, сохраняется на следующий раз.
settings-clap-hint-desktop = Использует модель CLAP от LAION, которая работает на процессоре этого компьютера: ничего наружу не уходит. При включении модель ({ $size } МБ) скачивается с Hugging Face, а затем в фоне с низким приоритетом прослушивает начало каждого звука. Пока она работает или вы ищете, она занимает примерно до 1 ГБ памяти. При выключении модель удаляется, а то, что она изучила, сохраняется на следующий раз.
settings-clap-ready-empty = Готово. Звуки изучаются по мере добавления паков.
settings-clap-ready-all =
    { $count ->
        [one] Готово: изучен { $count } звук
        [few] Готово: изучены все { $count } звука
        [many] Готово: изучены все { $count } звуков
       *[other] Готово: изучены все звуки ({ $count })
    }
settings-clap-learning = Готово: изучаем звуки, пока { $done } из { $total }

## SettingsPage.vue: теги от ИИ

settings-ai-service = Сервис
settings-ai-off = Выкл.
settings-ai-off-hint = Паки размечаются только встроенными правилами.
settings-ai-local = Локальная модель
settings-ai-local-hint = Сервис llm в compose-файле Hoard (или cargo xtask llm). Скачайте модель ниже; она работает на этой машине, и ничего наружу не уходит.
settings-ai-chatgpt = Тариф ChatGPT
settings-ai-chatgpt-hint = Использует ваш тариф ChatGPT Go, Plus, Pro или Business вместо ключа API.
settings-ai-openrouter-hint = Один ключ для сотен размещённых моделей, некоторые из них бесплатные. Модели, принимающие изображения, отмечены.
settings-ai-openai-hint = Размещается OpenAI.
settings-ai-anthropic-hint = Размещается Anthropic, через его OpenAI-совместимый эндпоинт.
settings-ai-gemini-hint = Размещается Google, через его OpenAI-совместимый эндпоинт.
settings-ai-ollama-hint = Ваш собственный сервер Ollama. Измените адрес, если это не контейнер с именем ollama.
settings-ai-custom = Другой (OpenAI-совместимый)
settings-ai-custom-hint = Всё, что понимает OpenAI chat API: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Подключение…
settings-ai-connected =
    { $count ->
        [one] Подключено · { $count } модель
        [few] Подключено · { $count } модели
        [many] Подключено · { $count } моделей
       *[other] Подключено · { $count } модели
    }
settings-ai-connected-none = Подключено, но моделей в списке нет; введите название модели
settings-chatgpt-connected-none = Подключено; введите название модели
settings-ai-takes-images = принимает изображения
settings-ai-saved = Теги от ИИ: { $service }
settings-ai-saved-off = Теги от ИИ выключены
settings-ai-reset = Теги от ИИ: вернуть по умолчанию
settings-ai-on = Вкл.: { $service } · { $model }
settings-ai-url = Адрес
settings-ai-key = Ключ API
settings-ai-key-saved = Сохранён (введите, чтобы заменить)
settings-ai-key-paste = Вставьте ключ
settings-ai-key-optional = Нет (необязательно)
settings-ai-key-remove = Удалить
settings-ai-get-key = Получить ключ
settings-ai-model = Модель
settings-ai-model-pick = Выберите или введите модель
settings-ai-model-name = Название модели
settings-ai-refresh = Обновить
settings-ai-connect = Подключить
settings-ai-need-key = Добавьте ключ API, чтобы увидеть модели
settings-ai-vision = Отправлять и миниатюры (нужна модель, принимающая изображения)
settings-ai-batch = За запрос
settings-ai-batch-unit = ассетов (меньше для маленьких локальных моделей, больше для крупных размещённых)

## SettingsPage.vue: вход в ChatGPT

settings-chatgpt-account = Учётная запись
settings-chatgpt-checking = Проверка…
settings-chatgpt-status-as = Вход выполнен как { $email }
settings-chatgpt-status = Вход выполнен
settings-chatgpt-signed-in-as = Вход в ChatGPT выполнен как { $email }
settings-chatgpt-signed-in = Вход в ChatGPT выполнен
settings-chatgpt-signed-out = Выход из ChatGPT выполнен
settings-chatgpt-continue = Продолжить с ChatGPT
settings-chatgpt-sign-out = Выйти
settings-chatgpt-sign-out-title = Выйти из ChatGPT?
settings-chatgpt-sign-out-in-use = Теги от ИИ используют ваш тариф ChatGPT, поэтому они остановятся, пока вы снова не войдёте или не выберете другой сервис.
settings-chatgpt-sign-out-body = Hoard забудет вход и попросит OpenAI его завершить.
settings-chatgpt-manage = Управлять использованием
settings-chatgpt-finish-before = Завершите вход в открывшемся окне (
settings-chatgpt-finish-link = открыть снова
settings-chatgpt-finish-middle = ). Если вход закончится страницей, которая не загружается, по адресу, начинающемуся с{ " " }
settings-chatgpt-finish-after = , скопируйте этот адрес и вставьте сюда:
settings-chatgpt-paste = Адрес, на котором закончился вход
settings-chatgpt-finish = Завершить
settings-chatgpt-usage = Запросы на расстановку тегов расходуют лимит вашего тарифа ChatGPT. «Управлять использованием» показывает, сколько потратил Hoard, и позволяет это ограничить. Чтобы отключить Hoard в самом ChatGPT, откройте на chatgpt.com Settings → Security and login → Sign in with ChatGPT.

## SettingsPage.vue: локальные модели

settings-local-downloading-progress = Скачивание { $percent }% · { $done } из { $total }
settings-local-downloading = Скачивание…
settings-local-incomplete = Скачивание остановлено
settings-local-failed = В прошлый раз не запустилась
settings-local-in-use-running = Используется · работает
settings-local-in-use = Используется
settings-local-starting = Запуск…
settings-local-running = Работает
settings-local-ready-images = Готова · принимает изображения
settings-local-ready-text = Готова · только текст
settings-local-delete-title = Удалить { $name }?
settings-local-delete-in-use = Теги от ИИ используют эту модель, поэтому они остановятся, пока вы не выберете другую. Её файлы удаляются с сервера моделей.
settings-local-delete-downloading = Скачивание остановится, а уже скачанное будет удалено.
settings-local-delete-body = Её файлы удаляются с сервера моделей. Позже её можно скачать снова.
settings-local-delete = Удалить
settings-local-deleting = Удаление…
settings-local-cancel = Отмена
settings-local-resume = Продолжить
settings-local-download = Скачать
settings-local-start-before = Запустите его командой{ " " }
settings-local-start-or = { " " }(или{ " " }
settings-local-start-developing = { " " }при разработке), затем{ " " }
settings-local-start-retry = попробуйте снова
settings-local-start-after = . Подойдёт и сервер, на котором работает только одна модель: укажите её название ниже.
settings-local-list = На сервере моделей
settings-local-none = Пока ничего не скачано. Выберите модель ниже; она хранится на сервере, и переключиться можно в любой момент.
settings-local-catalog = Модели для скачивания
settings-local-size = { $size } ГБ · { $license }
settings-local-other = Другая модель с Hugging Face
settings-local-other-placeholder = Другая с Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: добавление паков

settings-store-pages = Страницы магазинов
settings-store-pages-what = Новые паки ищутся в магазинах в фоне, чтобы у них появились страница магазина, продавец и лицензия.
settings-store-lookup = Искать страницы магазинов автоматически
settings-unreal = Пакеты Unreal
settings-unreal-reset = Пакеты Unreal: вернуть по умолчанию
settings-unreal-what = Что делать с файлами .uasset.
settings-conv = Правила преобразования
settings-conv-what = Преобразует файлы, которые хранит Hoard, в один формат — например, все OBJ и FBX в GLB, — чтобы библиотека была единообразной. Преобразованный файл занимает место оригинала и сохраняет его имя, теги, коллекции и ссылки. Файлы в отслеживаемых папках никогда не изменяются.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Включено
settings-conv-to = Преобразовывать в
# Heading over the file formats a rule converts.
settings-conv-from = Из
settings-conv-remove = Удалить
settings-conv-remove-label = Удалить правило { $n }
settings-conv-taken = Этот формат уже преобразует другое правило
settings-conv-upright = Ставить модели вертикально при преобразовании (Y вверх, лицом к +Z), так, как показываются модели пака
settings-conv-flac-what = Звук остаётся точно таким же, а места занимает примерно вдвое меньше, и всё остальное из WAV (точки петли, маркеры cue, теги) сохраняется во FLAC, так что WAV можно в любой момент вернуть точно таким, каким он был. WAV с сэмплами с плавающей точкой или 32-битными сэмплами остаются WAV.
settings-conv-add = Добавить правило
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Хранить оригиналы
settings-conv-keep-days =
    { $count ->
        [one] день
        [few] дня
        [many] дней
       *[other] дня
    }
settings-conv-keep-what = До тех пор на странице преобразованного файла можно вернуть оригинал. Текстуры не копируются в преобразованную модель: она ссылается на файлы изображений рядом с ней. Файлы .mtl и .bin модели остаются.
settings-conv-save = Сохранить правила
settings-conv-revert = Отменить изменения
settings-conv-existing = Преобразовать файлы, которые уже в библиотеке
settings-conv-saved = Правила преобразования сохранены. Они применяются к пакам, добавленным с этого момента.
settings-conv-converting =
    { $count ->
        [one] В фоне преобразуется { $count } файл
        [few] В фоне преобразуются { $count } файла
        [many] В фоне преобразуются { $count } файлов
       *[other] В фоне преобразуются { $count } файла
    }
settings-conv-nothing = В библиотеке нечего преобразовывать.
settings-conv-ask-title =
    { $count ->
        [one] Преобразовать { $count } файл, который уже в библиотеке?
        [few] Преобразовать { $count } файла, которые уже в библиотеке?
        [many] Преобразовать { $count } файлов, которые уже в библиотеке?
       *[other] Преобразовать { $count } файла, которые уже в библиотеке?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Всё это в { $packs } паке.
        [few] Всё это в { $packs } паках.
        [many] Всё это в { $packs } паках.
       *[other] Всё это в { $packs } пака.
    } Каждый оригинал хранится { $days ->
        [one] { $days } день
        [few] { $days } дня
        [many] { $days } дней
       *[other] { $days } дня
    }, и до тех пор его можно вернуть с его страницы.
settings-conv-ask-busy =
    { $count ->
        [one] { $count } файл — в паке, который ещё добавляется; он будет преобразован, когда это закончится.
        [few] { $count } файла — в паках, которые ещё добавляются; они будут преобразованы, когда это закончится.
        [many] { $count } файлов — в паках, которые ещё добавляются; они будут преобразованы, когда это закончится.
       *[other] { $count } файла — в паках, которые ещё добавляются; они будут преобразованы, когда это закончится.
    }
settings-conv-ask-list = Файлы, которые будут преобразованы
settings-conv-ask-more = …и ещё { $count }
settings-conv-ask-skipped = Останутся как есть:
settings-conv-skip-watched =
    { $count ->
        [one] { $count } файл в отслеживаемой папке
        [few] { $count } файла в отслеживаемых папках
        [many] { $count } файлов в отслеживаемых папках
       *[other] { $count } файла в отслеживаемых папках
    }
settings-conv-skip-unity =
    { $count ->
        [one] { $count } файл, который проект Unity знает по имени
        [few] { $count } файла, которые проект Unity знает по имени
        [many] { $count } файлов, которые проект Unity знает по имени
       *[other] { $count } файла, которые проект Unity знает по имени
    }
settings-conv-skip-used =
    { $count ->
        [one] { $count } файл, на который ссылается сцена или другой файл
        [few] { $count } файла, на которые ссылается сцена или другой файл
        [many] { $count } файлов, на которые ссылается сцена или другой файл
       *[other] { $count } файла, на которые ссылается сцена или другой файл
    }
settings-conv-skip-beside =
    { $count ->
        [one] { $count } файл, преобразованная копия которого уже есть в его паке
        [few] { $count } файла, преобразованные копии которых уже есть в их паках
        [many] { $count } файлов, преобразованные копии которых уже есть в их паках
       *[other] { $count } файла, преобразованные копии которых уже есть в их паках
    }
settings-conv-skip-blender =
    { $count ->
        [one] { $count } файл, который Blender ещё не прочитал («Настройки → Инструменты»)
        [few] { $count } файла, которые Blender ещё не прочитал («Настройки → Инструменты»)
        [many] { $count } файлов, которые Blender ещё не прочитал («Настройки → Инструменты»)
       *[other] { $count } файла, которые Blender ещё не прочитал («Настройки → Инструменты»)
    }
settings-conv-ask-new = Сохранить, только новые паки
settings-conv-ask-save-convert =
    { $count ->
        [one] Сохранить и преобразовать { $count } файл
        [few] Сохранить и преобразовать { $count } файла
        [many] Сохранить и преобразовать { $count } файлов
       *[other] Сохранить и преобразовать { $count } файла
    }
settings-conv-ask-convert =
    { $count ->
        [one] Преобразовать { $count } файл
        [few] Преобразовать { $count } файла
        [many] Преобразовать { $count } файлов
       *[other] Преобразовать { $count } файла
    }
settings-unreal-off = Оставить как есть
settings-unreal-off-hint = Преобразуйте на странице пака, когда захотите.
settings-unreal-keep = Преобразовать, оставить оригиналы
settings-unreal-keep-hint = Текстуры, звуки и материалы извлекаются в распространённых форматах; .uasset тоже остаётся.
settings-unreal-replace = Преобразовать, удалить оригиналы
settings-unreal-replace-hint = Экономит место после извлечения контента. Меши всё равно ждут обработчика Unreal.

## SettingsPage.vue: скачивание

settings-tn = Названия текстур
settings-tn-saved = Названия текстур сохранены
settings-tn-reset = Названия текстур: вернуть по умолчанию
settings-tn-what = Карты материала отдаются с названиями по материалу и назначению каждой карты, как бы ни назывались файлы: Rock_Normal.png, а не rock_moss_xeokah0_raw_8K_Normal.png. Файлы в библиотеке не затрагиваются, а карта, которую называет модель или MTL, сохраняет это название, чтобы модель по-прежнему её находила.
settings-tn-toggle = Называть карты текстур по материалу
settings-tn-style = Стиль
settings-tn-style-group = Стиль названий
settings-tn-descriptive = Описательный
settings-tn-short = Короткий
settings-tn-custom = Свой
settings-tn-pattern = Шаблон
settings-tn-hint-name = { " " }— материал,{ " " }
settings-tn-hint-map = { " " }— назначение карты,{ " " }
settings-tn-hint-size = { " " }— её размер, если набор есть в нескольких размерах (добавляется в конце, если шаблон его не содержит).
settings-tn-example = Пример
settings-tn-suffixes = Суффиксы для каждого вида карт
settings-tn-slots = При скачивании GLB текстура, упакованная внутрь модели без собственного файла, получает название по материалу и слоту, который она занимает, и у этих слотов тоже есть суффиксы:
settings-tn-endings = Свои окончания
settings-tn-endings-what = Текстура, которая не относится к картам материала, сохраняет своё название, если только оно не оканчивается на одно из этих окончаний: _diff → _BC отдаёт rock_diff.png как rock_BC.png. Оставьте второе поле пустым, чтобы убрать окончание.
settings-tn-ending-from = Окончание
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Заменить на
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Добавить окончание
settings-tn-ending-remove = Убрать
settings-tn-ending-remove-label = Убрать { $ending }

## SettingsPage.vue: общие и подключение

settings-name = Название
settings-name-what = Как этот сервер называется на верхней панели, во вкладке браузера и на странице входа, чтобы при нескольких серверах Hoard можно было их различать.
settings-name-what-desktop = Как эта библиотека называется на верхней панели.
settings-address = Адрес
settings-address-what = Адрес, по которому люди и приложения обращаются к Hoard, если он отличается от адреса в этом браузере: адрес обратного прокси, туннеля или имя в tailnet, например https://hoard.example.com. Его используют ссылки, которые выдаёт Hoard (для Claude и других приложений, при входе, в его сертификате).
settings-public-url = Публичный адрес
settings-public-url-placeholder = Из адреса браузера (например, https://hoard.example.com)
settings-public-url-default = из адреса браузера

## ProfilerPanel.vue: профилировщик вверху страницы «Настройки → Производительность»

profiler = Профилировщик
profiler-what = Что Hoard делал за последние десять минут; обновляется каждые две секунды, пока вы смотрите. Хранится только в памяти и начинается заново после перезапуска Hoard.
profiler-starting = Первые показания появятся через мгновение.
profiler-failed = Не удалось прочитать профилировщик: { $error }
profiler-cpu = Загрузка процессора
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Вся машина
profiler-cpu-summary = Hoard использует { $hoard } процессоров, а вся машина — { $machine }.
profiler-ram = ОЗУ
profiler-ram-peak = Пик
profiler-ram-of = { $used } из { $total }
profiler-ram-summary = Hoard использует { $hoard } ОЗУ, а за последние десять минут — не больше { $peak }.
profiler-serving = Запросы в работе
profiler-working = В работе
profiler-working-none = В фоне ничего не выполнялось.
profiler-working-more =
    { $count ->
        [one] И ещё { $count }.
        [few] И ещё { $count }.
        [many] И ещё { $count }.
       *[other] И ещё { $count }.
    }
profiler-requests = Время обслуживания запросов
profiler-requests-what = Каждый столбец — это десять секунд запросов, а их время разбито по тому, на что оно ушло.
profiler-part-db = База данных
profiler-part-db-title = Ожидание базы данных и выполнение её запросов
profiler-part-disk = Диск
profiler-part-disk-title = Чтение сохранённых файлов и сжатие отправляемого
profiler-part-send = Отправка
profiler-part-send-title = Ожидание, пока сеть принимает отправленное
profiler-part-other = Прочее
profiler-part-other-title = Подготовка ответа
profiler-request-count =
    { $count ->
        [one] { $count } запрос
        [few] { $count } запроса
        [many] { $count } запросов
       *[other] { $count } запроса
    }
profiler-requests-none = За последние десять минут запросов не было.
profiler-slowest = Самый медленный: { $route }, { $time }
profiler-routes = По маршрутам
profiler-route = Маршрут
profiler-count = Запросы
profiler-average = Среднее
profiler-p95 = Самые медленные 5%
profiler-split = Куда ушло время
profiler-sent = Отправлено
profiler-portal = собственные файлы интерфейса

## SettingsPage.vue: производительность

settings-render = 3D-миниатюры
settings-render-toggle = Рендерить миниатюры для 3D-моделей
settings-gpu-on = На GPU: { $renderer }
settings-gpu-software = GPU не найден, поэтому программно ({ $renderer })
settings-gpu-none = Не найдено ни GPU, ни программного рендерера, поэтому у моделей не будет миниатюр
settings-thumbs = Миниатюры
settings-thumbs-what = Как хранятся новые миниатюры и виды моделей. Миниатюры с прозрачностью и пиксель-арт всегда в PNG. Чтобы сжать уже имеющиеся, запустите «Сжать миниатюры» в разделе «Обслуживание».
settings-previews-size = Сейчас превью занимают { $size }.
settings-thumbs-saved = Миниатюры: { $quality }
settings-thumbs-format = Формат
settings-thumbs-quality = Качество { $quality }.
settings-thumbs-small = Меньше
settings-thumbs-small-hint = В сетке хорошо; вблизи немного мягко.
settings-thumbs-balanced = Сбалансированно
settings-thumbs-balanced-hint = В размере миниатюры трудно отличить от оригинала.
settings-thumbs-sharp = Чётче
settings-thumbs-sharp-hint = Примерно вдвое больше, чем «Сбалансированно».
settings-thumbs-lossless = Без потерь
settings-thumbs-lossless-hint = PNG, в точности как создано. В несколько раз больше. Никогда не отправляются в AVIF.
settings-thumbs-avif-what = AVIF часто вдвое меньше. Каждая миниатюра преобразуется при первом показе.
settings-thumbs-avif-now = Преобразовать все сейчас
settings-thumbs-avif-converting = Преобразование…
settings-motion-thumbs = Движущиеся миниатюры анимаций
settings-motion-thumbs-what = Каждая анимация также получает короткий зацикленный ролик, который играет, пока вы наводите указатель на её плитку. Каждый занимает около 40 КБ, а для большой библиотеки их создание занимает время. При выключении они удаляются.
settings-limits = Работа и ограничения
settings-limits-what = Каждое значение сохраняется, когда вы уходите из поля или нажимаете Enter. Размеры указываются с единицами: 100 GB, 512 MB, 1.5 TB; число без единиц — это GB.
settings-workers = Фоновые обработчики
settings-workers-unit = задач одновременно (импорт, теги, преобразования)
settings-max-upload = Самая большая загрузка
settings-max-upload-unit = на файл
settings-max-expand = Самый большой пак
settings-max-expand-unit = после распаковки
settings-conversion-cache = Преобразованные загрузки
settings-conversion-cache-unit-now = храним не больше, сейчас { $size }; первыми удаляются давно не используемые
settings-conversion-cache-unit = храним не больше; первыми удаляются давно не используемые

## SettingsPage.vue: обслуживание

settings-updates = Обновления
settings-version = Это Hoard { $version }.
settings-update-check = Проверять GitHub каждые несколько часов на наличие нового выпуска
settings-desktop-updates = Hoard ищет новую версию при каждом запуске, скачивает её в фоне и устанавливает, когда вы выходите.
settings-backups = Резервные копии по расписанию
settings-backups-what = База данных сама делает резервные копии; каждая появляется в истории ниже.
settings-backup-every = Резервная копия
settings-backup-off = Никогда
settings-backup-daily = Каждый день
settings-backup-weekly = Каждую неделю
settings-backup-keep = Хранить
settings-backup-keep-unit = последних копий по расписанию
settings-backup-copy-to = Также копировать в
settings-backup-copy-to-none = Без другой папки
settings-backups-kept = Каждая сохраняется в папку резервных копий как hoard-auto-…db и, если указана, в другую папку (другой диск или папку, которую сохраняет приложение синхронизации), заданную полным путём на сервере. Удаляются только старые копии по расписанию; сделанные вручную остаются.
about-what = Hoard хранит игровые ассеты (модели, текстуры, звуки, музыку, спрайты и многое другое) в одной библиотеке, в которой можно искать, просматривать и откуда можно брать нужное, общей для всех, кому вы дали доступ.
about-version = Версия
about-edition = Редакция
about-edition-server = Сервер
about-edition-desktop = Настольное приложение
about-edition-steam = Настольное приложение из Steam
about-made-by = Разработчик
about-source = Исходный код на GitHub
about-issues = Сообщить о проблеме
about-discord = Пообщаться с нами в Discord
about-steam = Hoard в Steam
about-license = Лицензия
about-license-what = Hoard — открытое ПО под лицензией MIT No Attribution: используйте, изменяйте и распространяйте его как угодно.
about-downloaded = Скачивается при необходимости
about-downloaded-what = Они не входят в состав Hoard. Каждый из них скачивается, когда впервые понадобится, и у каждого своя лицензия.
about-blender = Преобразует 3D-модели и создаёт их миниатюры.
about-ffmpeg = Преобразует звук и видео и создаёт их превью.
about-siglip = Находит изображения по тому, что на них.
about-clap = Находит звуки по тому, как они звучат.
about-models = На модели, которые вы выбираете для локального ИИ-сервера, распространяются лицензии их авторов.
about-built-with = Создано с помощью
about-built-with-what = Библиотеки с открытым исходным кодом, из которых сделан Hoard, с их лицензиями.
about-steamworks = Версия для Steam также включает Steamworks SDK, © Valve Corporation.
about-licenses-loading = Загрузка списка…
about-licenses-failed = Не удалось загрузить список.
about-filter = Фильтр по названию или лицензии
about-count = { $shown } из { $total }
about-no-license = Лицензия не указана
about-report = Сообщить о проблеме…
report-title = Сообщить о проблеме
report-what = Это то, что нужно issue на GitHub, чтобы проблему можно было изучить. Ничего личного здесь нет: токены доступа, пароли, ключи, адреса электронной почты и ваше имя в путях к папкам удаляются. Прочитайте перед отправкой.
report-loading = Сбор отчёта…
report-failed = Не удалось собрать отчёт: { $error }
report-log = Что сервер записал в журнал недавно
report-copy = Копировать
report-copied = Отчёт скопирован.
report-github = Открыть issue на GitHub
report-github-hint = Новый issue заполняется этим. Если журнал для этого слишком длинный, туда попадают только последние строки; в «Копировать» есть всё.
report-close = Закрыть

## Виды ассетов (api.ts)

api-kind-texture = Текстуры
api-kind-vector = Векторная графика
api-kind-sheet = Листы спрайтов
api-kind-audio = Аудио
api-kind-model = Модели
api-kind-animation = Анимации
api-kind-font = Шрифты
api-kind-shader = Шейдеры
api-kind-material = Материалы
api-kind-video = Видео
api-kind-document = Документы
api-kind-data = Данные
api-kind-other = Другое
api-kind-texture-name = текстуры
api-kind-vector-name = векторная графика
api-kind-sheet-name = листы спрайтов
api-kind-audio-name = аудио
api-kind-model-name = модели
api-kind-animation-name = анимации
api-kind-font-name = шрифты
api-kind-shader-name = шейдеры
api-kind-material-name = материалы
api-kind-video-name = видео
api-kind-document-name = документы
api-kind-data-name = данные
api-kind-other-name = другое

## Уведомления, диалоги и загрузка (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Отменить
ui-today = сегодня
ui-yesterday = вчера
ui-days-ago =
    { $days ->
        [one] { $days } день назад
        [few] { $days } дня назад
        [many] { $days } дней назад
       *[other] { $days } дня назад
    }
ui-notifications = Уведомления
ui-dismiss = Скрыть
ui-cancel = Отмена
ui-lazy-loading = Загрузка: { $what }…
ui-lazy-failed = Не удалось загрузить: { $what }.
ui-lazy-reload = Перезагрузить страницу

## ResetButton.vue

reset-title = Вернуть значение по умолчанию: { $value }
reset-label = Вернуть значение по умолчанию, { $value }

## SizeInput.vue

size-invalid = Размер вроде 100 GB, 512 MB или 1.5 TB
size-hint = Размер вроде 100 GB, 512 MB или 1.5 TB; число без единиц — это GB

## AuthPanel.vue («Настройки → Аутентификация»)

auth-load-failed = Не удалось загрузить настройки аутентификации: { $error }
auth-saved = Сохранено. Действует со следующего входа.
auth-method-internal = Встроенные
auth-method-internal-hint = Имена и пароли, которые хранит Hoard; добавляются в «Настройки → Учётная запись».
auth-method-ldap-hint = Имя и пароль, введённые на странице входа, проверяет каталог: OpenLDAP, FreeIPA, LLDAP или другой LDAP-сервер.
auth-method-ad-hint = Учётные записи домена Windows входят со своими обычными именем и паролем.
auth-method-oauth-hint = Кнопка «Войти через…», которая ведёт в Google, Microsoft, GitHub, Keycloak, Authentik или к другому провайдеру и обратно.
auth-session-unused = Через 30 дней без использования
auth-session-1h = Через 1 час после входа
auth-session-8h = Через 8 часов после входа
auth-session-day = Через день после входа
auth-session-week = Через неделю после входа
auth-session-30d = Через 30 дней после входа
auth-session-hours =
    { $hours ->
        [one] Через { $hours } час после входа
        [few] Через { $hours } часа после входа
        [many] Через { $hours } часов после входа
       *[other] Через { $hours } часа после входа
    }
auth-role-view = Может смотреть
auth-role-edit = Может изменять
auth-role-admin = Администратор
auth-role-word-view = может смотреть
auth-role-word-edit = может изменять
auth-role-word-admin = администратор
auth-role-word-default = { $role } (роль по умолчанию)
auth-tls-starttls-hint = Обычный LDAP (порт 389), переключаемый на TLS до отправки чего-либо.
auth-tls-ldaps-hint = TLS с самого начала (порт 636).
auth-tls-none = Нет
auth-tls-none-hint = Пароли передаются по сети как есть. Только в сети, которой вы доверяете.
auth-test-failed = Не удалось проверить: { $error }
auth-preset-custom = Другой (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel и другие: укажите издателя (issuer), и «Найти» заполнит адреса.
auth-preset-google-note = Создайте OAuth-клиент (Web application) в Google Cloud Console → APIs & Services → Credentials. Google не передаёт группы, поэтому назначайте роли в «Настройки → Учётная запись» и выключите создание учётных записей при первом входе, если только не любой обладатель учётной записи Google должен получить доступ.
auth-preset-microsoft-note = Зарегистрируйте приложение в Entra ID → App registrations (с перенаправлением Web на адрес ниже). Подставьте ID своего клиента (tenant) вместо «organizations», чтобы впускать только свою организацию. Чтобы получать роли из групп, добавьте утверждение groups в Token configuration: в нём перечислены ID групп.
auth-preset-github-note = Создайте OAuth-приложение в GitHub → Settings → Developer settings. GitHub не передаёт группы, поэтому назначайте роли в «Настройки → Учётная запись» и выключите создание учётных записей при первом входе, если только не любой обладатель учётной записи GitHub должен получить доступ.
auth-preset-gitlab-note = Создайте приложение в GitLab → Preferences → Applications (или в админке своего экземпляра: тогда замените gitlab.com в адресах).
auth-discovered = Адреса провайдера заполнены.
auth-copied = Скопировано
auth-copy-failed = Не удалось скопировать: выделите и скопируйте сами
auth-sign-in-with = Войти через
auth-own-accounts-still-work = Собственные учётные записи Hoard по-прежнему входят со своими паролями, так что недоступный или неправильно настроенный каталог или провайдер не закроет вам доступ.
auth-sessions = Сеансы
auth-sign-out-after = Завершать сеансы
auth-ldap-server = LDAP-сервер
auth-server-type = Тип сервера
auth-server-type-custom = Другой
auth-servers = Серверы
auth-server-n = Сервер { $n }
auth-remove-server-n = Удалить сервер { $n }
auth-remove = Удалить
auth-add-server = Добавить сервер
auth-servers-hint = Если один недоступен, пробуется следующий.
auth-security = Безопасность
auth-connection-security = Защита соединения
auth-skip-verify = Не проверять сертификат сервера (только чтобы попробовать)
auth-ca = Центр сертификации
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (пусто: те, которым доверяет система)
auth-lookup = Поиск людей
auth-anonymously = Анонимно
auth-reader-account = Учётная запись для чтения
auth-reader-password = Её пароль
auth-saved-type-to-replace = Сохранено: введите, чтобы заменить
auth-people-under = Люди находятся в
auth-name-attribute = Атрибут имени
auth-only-matching = Только записи, подходящие под
auth-groups-under = Группы находятся в
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (пусто: только memberOf)
auth-their-groups = Их группы
auth-group-filter-hint = { "{" }dn{ "}" } — запись человека, { "{" }user{ "}" } — имя, с которым он вошёл.
auth-test = Проверка
auth-test-hint = Подключается к каждому серверу и входит под учётной записью для чтения. Если указаны имя и пароль, входит и под ними и показывает, что из этого сделает Hoard. Ничего не сохраняется.
auth-test-user = От чьего имени войти
auth-test-user-placeholder = От чьего имени войти (необязательно)
auth-test-password = Пароль
auth-testing = Проверка…
auth-test-connectivity = Проверить подключение
auth-test-server-ok = доступен, вход под учётной записью для чтения выполнен
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = : вход выполнен (
auth-test-signed-in-after = ).
auth-test-role = Роль: { $role }.
auth-test-no-role = Этого человека нет ни в одной из групп, указанных в разделе «Роли», поэтому ему будет отказано во входе.
auth-test-groups = Группы: { $groups }
auth-test-no-groups = Группы не найдены.
auth-provider = Провайдер
auth-redirect = Адрес перенаправления
auth-copy = Копировать
auth-redirect-hint = Укажите его провайдеру, когда будете создавать клиент для Hoard. Он берётся из публичного адреса в «Настройки → Подключение» или из адреса, который вы используете.
auth-issuer = Издатель (issuer)
auth-discover = Найти
auth-client-id = Client ID
auth-client-secret = Client secret
auth-authorization-url = Адрес авторизации
auth-token-url = Адрес токенов
auth-userinfo-url = Адрес сведений о пользователе
auth-userinfo-placeholder = (пусто: кто это, говорит ID-токен)
auth-scopes = Области (scopes)
auth-name-from = Имя из
auth-groups-from = Группы из
auth-groups-from-placeholder = (нет)
auth-button = Кнопка
auth-button-placeholder = вашу учётную запись
auth-accounts = Учётные записи
auth-provision = Создавать учётную запись Hoard при первом входе человека
auth-provision-on-hint = Иначе администратор сначала добавляет каждого человека в «Настройки → Учётная запись».
auth-provision-off-hint = Администратор сначала добавляет каждого человека в «Настройки → Учётная запись» и выбирает, как тот будет входить.
auth-open-to-anyone = Войти может любой, у кого есть учётная запись { $provider }. Добавьте группы зрителей ниже или выключите это и добавляйте людей сами.
auth-new-accounts = Новые учётные записи
auth-roles-from-groups = Роли из групп
auth-roles-hint = Группы, участники которых получают каждую роль, по одной на строку, по имени или DN. Побеждает самая высокая подходящая роль, и она проверяется при каждом входе. Если групп нет, новые учётные записи получают роль выше, а администратор меняет её потом. Если указаны группы в «Может смотреть», входить могут только участники указанных групп.
auth-roles-hint-oauth = Группы, участники которых получают каждую роль, по одной на строку, по имени или DN (или как их перечисляет утверждение groups). Побеждает самая высокая подходящая роль, и она проверяется при каждом входе. Если групп нет, новые учётные записи получают роль выше, а администратор меняет её потом. Если указаны группы в «Может смотреть», входить могут только участники указанных групп.
auth-saving = Сохранение…
auth-save = Сохранить

## HttpsPanel.vue («Настройки → Подключение»)

https-load-failed = Не удалось загрузить HTTPS: { $error }
https-off-redirecting = HTTPS выключен. Переходим на http://
https-now-on = HTTPS включён
https-saved = Сохранено. Новые подключения используют новый сертификат.
https-now-off = HTTPS выключен
https-http3-failed-toast = Не удалось запустить HTTP/3: { $error }
https-copied = Скопировано
https-copy-failed = Не удалось скопировать: выделите и скопируйте сами
https-mode-off = Выкл.
https-mode-off-hint = Только обычный http://.
https-mode-own = Собственный сертификат Hoard
https-mode-own-hint = Hoard создаёт его для адресов этого сервера. Установите центр сертификации Hoard один раз на каждое устройство, и его браузеры будут ему доверять.
https-mode-custom = Мой сертификат
https-mode-custom-hint = Уже имеющийся у вас, например от tailscale cert или Let’s Encrypt. Браузеры доверяют ему без установки чего-либо.
https-untrusted-own = Этот браузер пока не доверяет ему для { $host }: установите центр сертификации Hoard ниже. Вы увидите предупреждение, которое можно пропустить.
https-untrusted-uncovered = Этот браузер пока не доверяет ему для { $host }: сертификат не охватывает этот адрес. Вы увидите предупреждение, которое можно пропустить.
https-untrusted = Этот браузер пока не доверяет ему для { $host }. Вы увидите предупреждение, которое можно пропустить.
https-now = Сейчас
https-start-failed = HTTPS настроен, но не запустился: { $error }
https-on-port = HTTPS включён, на порту { $port }, как и http://.
https-using-it = Вы сейчас им пользуетесь.
https-open = Открыть { $url }
https-trusted = Этот браузер ему доверяет.
https-cert-for-before = Сертификат для{ " " }
https-cert-for-after = , до { $date }.
https-cert-for-after-own = , до { $date } (продлевается сам).
https-not-covered-before = Он не охватывает{ " " }
https-not-covered-after = { " " }— адрес, который вы используете.
https-is-off = HTTPS выключен: страницы отдаются по обычному http://.
https-addresses = Адреса
https-addresses-hint = Сертификат охватывает адреса, которые нашёл Hoard, и все, что вы добавите (имя от вашего роутера, имя в Tailscale).
https-remove-name = Удалить { $name }
https-another-address = Другой адрес
https-another-address-placeholder = Другой адрес, например nas.local
https-add = Добавить
https-your-cert = Ваш сертификат
https-saved-cert-before = Сохранён:{ " " }
https-saved-cert-after = { " " }от { $issuer }, до { $date }. Вставьте новый, чтобы заменить его.
https-pem-before = Сертификат (с цепочкой) и его закрытый ключ в виде текста PEM: файлы, которые начинаются с{ " " }
https-pem-after = .
https-certificate = Сертификат
https-choose-file = Выбрать файл…
https-private-key = Закрытый ключ
https-checked-before = Для{ " " }
https-checked-after = , от { $issuer }, до { $date }.
https-checked-after-days =
    , от { $issuer }, до { $date } ({ $days ->
        [one] { $days } день
        [few] { $days } дня
        [many] { $days } дней
       *[other] { $days } дня
    }).
https-self-signed = Он самоподписанный: браузеры будут о нём предупреждать.
https-key-matches = Ключ подходит.
https-where-to-get = Где его взять
https-get-tailscale-before = включите HTTPS для своего tailnet, затем выполните{ " " }
https-get-tailscale-after = { " " }на сервере и загрузите оба файла. Они действуют 90 дней: до этого выполните команду снова и загрузите их ещё раз.
https-get-acme-title = Let’s Encrypt или другой клиент ACME
https-get-acme-before = (certbot, acme.sh, lego) для вашего собственного домена: загрузите{ " " }
https-get-acme-and = { " " }и{ " " }
https-get-acme-after = .
https-get-proxy-title = Обратный прокси
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), у которого есть сертификат: оставьте HTTPS здесь выключенным, и пусть прокси отправляет в Hoard обычный http://.
https-trying = Пробуем…
https-apply = Применить
https-apply-hint = Hoard сначала пробует новую настройку и оставляет текущую, если та не работает.
https-trust-title = Доверие к Hoard на ваших устройствах
https-download-ca = Скачать hoard-ca.crt
https-trust-hint = Установите центр сертификации Hoard один раз на каждое устройство, и его браузеры будут доверять этому серверу (и любому сертификату, который Hoard создаст для него позже). Он может ручаться только за этот Hoard. Отпечаток (SHA-256):
https-ca-windows = Откройте файл → «Установить сертификат» → «Текущий пользователь» → «Поместить все сертификаты в следующее хранилище» → «Обзор» → «Доверенные корневые центры сертификации» → «Готово». Перезапустите браузер.
https-ca-macos = Откройте файл; он попадёт в «Связку ключей». Дважды щёлкните там «{ $subject }» → «Доверие» → «При использовании этого сертификата»: «Всегда доверять».
https-ca-ios-title = iPhone и iPad
https-ca-ios = Скачайте его в Safari и разрешите загрузку. «Настройки» → «Профиль загружен» → «Установить». Затем «Настройки» → «Основные» → «Об этом устройстве» → «Доверие сертификатам» → включите «{ $subject }».
https-ca-android = «Настройки» → «Безопасность» → «Другие настройки безопасности» → «Шифрование и учётные данные» → «Установить сертификат» → «Сертификат ЦС» и выберите файл. (Названия немного различаются на разных телефонах; поищите в настройках «сертификат».)
https-ca-linux-1 = Debian и Ubuntu:{ " " }
https-ca-linux-2 = . Fedora и Arch:{ " " }
https-ca-linux-3 = . Chrome в Linux ведёт собственный список: «Настройки» → «Конфиденциальность и безопасность» → «Безопасность» → «Настроить сертификаты» → «Импорт».
https-ca-firefox = Firefox ведёт собственный список: «Настройки» → «Приватность и защита» → «Сертификаты» → «Просмотр сертификатов» → «Центры сертификации» → «Импортировать», и отметьте «Доверять при идентификации веб-сайтов».
https-ca-cli-title = Инструмент командной строки hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , или укажите путь в{ " " }
https-ca-cli-after = .
https-options = Параметры
https-redirect = Перенаправлять браузеры на других устройствах с http:// на https://
https-redirect-hint = Перенаправляются только загрузки страниц. Инструмент командной строки hoard, обработчик Unreal, скрипты и ИИ-приложения продолжают работать по http://, как и http://localhost на самом сервере.
https-http3-hint-before = Быстрее на нестабильном Wi-Fi и на телефонах. Нужны HTTPS и открытый UDP-порт { $port } вдобавок к TCP (в Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Работает.
https-http3-waits = Запустится, когда HTTPS будет включён.
https-http3-failed = Не удалось запустить HTTP/3: { $error }
https-lockout-title = Если вы потеряли доступ
https-lockout-1 = { "" }
https-lockout-2 = { " " }на самом сервере всегда работает по обычному http://. Или выключите HTTPS из командной строки сервера и запустите Hoard снова:{ " " }
https-lockout-3 = { " " }(в Docker:{ " " }
https-lockout-4 = , затем{ " " }
https-lockout-5 = ).

## AccountPanel.vue («Настройки → Учётная запись»)

account-role-view = Может смотреть
account-role-view-hint = Искать, смотреть превью и скачивать
account-role-edit = Может изменять
account-role-edit-hint = А также добавлять, изменять, размечать тегами, собирать в коллекции и отправлять в архив
account-role-admin = Администратор
account-role-admin-hint = А также настройки, учётные записи, обслуживание и удаление навсегда
account-source-internal = Пароль Hoard
account-source-ldap = Пароль { $directory }
account-source-oauth = Вход через { $provider }
account-source-word-directory = каталог
account-load-failed = Не удалось загрузить учётные записи: { $error }
account-failed = { $what }: { $error }
account-password-failed = Не удалось сменить пароль
account-password-changed = Пароль изменён. На других ваших устройствах выполнен выход.
account-add-failed = Не удалось добавить { $name }
account-added = { $name } теперь может входить.
account-role-failed = Не удалось изменить права { $name }
account-remove-title = Удалить { $name }?
account-remove-body = Выход будет выполнен везде, и снова войти будет нельзя. Ничего из добавленного в библиотеку не удаляется.
account-remove = Удалить
account-remove-failed = Не удалось удалить { $name }
account-token-failed = Не удалось создать токен
account-copied = Скопировано
account-copy-token-failed = Не удалось скопировать: выделите токен и скопируйте сами
account-copy-failed = Не удалось скопировать: выделите и скопируйте сами
account-revoke-title = Отозвать «{ $name }»?
account-revoke-body = Всё, что ещё его использует, с этого момента получит отказ.
account-revoke = Отозвать
account-revoke-failed = Не удалось отозвать «{ $name }»
account-disconnect-title = Отключить { $app }?
account-disconnect-body = С этого момента оно получит отказ, и чтобы пользоваться Hoard, его придётся подключить снова.
account-disconnect = Отключить
account-disconnect-failed = Не удалось отключить { $app }
account-guest-failed = Не удалось это изменить
account-never = никогда
account-you = Вы
account-sign-out = Выйти
account-signed-in-as-before = Вход выполнен как{ " " }
account-signed-in-as-admin = . Вы администратор.
account-signed-in-as-edit = . Вы можете изменять библиотеку.
account-signed-in-as-view = . Вы можете смотреть, но ничего не менять.
account-app-signin = Приложение Hoard само выполняет вход, поэтому пароля нет. Создайте ниже токен доступа для плагинов движков и команды hoard.
account-password-at-provider = Ваш пароль хранит провайдер, через которого вы входите, поэтому меняйте его там.
account-password-at-directory = Ваш пароль хранит каталог, через который вы входите, поэтому меняйте его там.
account-current-password = Текущий пароль
account-new-password = Новый пароль
account-new-password-again = Новый пароль ещё раз
account-change-password = Сменить пароль
account-password-mismatch = Новые пароли не совпадают.
account-password-rule = Не меньше 8 символов.
account-people = Люди
account-people-hint = Каждый человек может смотреть (искать, смотреть превью и скачивать), ещё и изменять библиотеку (добавлять, редактировать, размечать тегами, собирать в коллекции и отправлять в архив) или вдобавок быть администратором (настройки, учётные записи, обслуживание и удаление навсегда).
account-you-tag = (вы)
account-signs-in-oauth = Входит через провайдера OAuth
account-signs-in-directory = Входит через каталог
account-since = с { $date }
account-what-can-do = Права { $name }
account-name = Имя
account-how-sign-in = Как входит
account-password-label = Пароль (не меньше 8 символов)
account-password-placeholder = Пароль (от 8 символов)
account-what-they-can-do = Права
account-add = Добавить
account-ai-apps = Claude и другие ИИ-приложения
account-ai-apps-hint = Подключите Claude (или любое MCP-приложение), и оно сможет искать и получать для вас ассеты, а также описывать и размечать тегами то, до чего Hoard ещё не добрался. При первом подключении оно входит в Hoard; вы разрешаете это здесь, в браузере.
account-mcp-address = MCP-адрес Hoard
account-copy = Копировать
account-claude-code-1 = выполните{ " " }
account-claude-code-2 = , затем{ " " }
account-claude-code-3 = { " " }для входа.{ " " }
account-claude-code-4 = { " " }описывает пак.
account-claude-ai = Settings → Connectors → Add custom connector, вставьте адрес, затем Connect. Они подключаются с серверов Anthropic, поэтому Hoard должен быть доступен из интернета по HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, вставьте адрес, затем Connect. Они подключаются с серверов Anthropic, поэтому Hoard должен быть доступен из интернета по HTTPS (этот адрес недоступен: задайте публичный адрес в «Настройки → Подключение»).
account-connection-meta = для { $user } · подключено: { $since } · использовалось: { $used }
account-no-apps = Подключённых приложений пока нет.
account-live = Сейчас используют Hoard
account-live-hint = Приложения, которые обращались к Hoard за последние 10 минут. Список обновляется сам.
account-live-none = За последние 10 минут к Hoard никто не обращался.
account-live-last = последнее: { $what } · { $when } · запросов: { $calls }
account-tokens = Токены доступа
account-tokens-hint-1 = Для инструмента командной строки{ " " }
account-tokens-hint-2 = , скриптов, MCP-клиентов и обработчика Unreal: они отправляют его как{ " " }
account-tokens-hint-3 = . Токен может делать то, что вы выберете, в пределах ваших прав, но никогда не управляет учётными записями и токенами. Токена, который может только смотреть, хватает, чтобы искать, скачивать и синхронизировать проект.
account-tokens-admin = Вы видите токены и подключённые приложения всех; остальные видят только свои.
account-made-before = { "" }
account-made-after = : скопируйте его сейчас. Больше он показан не будет.
account-new-token = Новый токен
account-cli-1 = Для CLI:{ " " }
account-cli-2 = , или поместите его в{ " " }
account-cli-3 = .
account-done = Готово
account-token-meta = создан: { $made } · использовался: { $used }
account-token-meta-by = создан: { $made }, создатель: { $by } · использовался: { $used }
account-no-tokens = Токенов пока нет.
account-token-name = Что будет его использовать
account-token-name-placeholder = Что будет его использовать, например «Обработчик Unreal»
account-token-role = Права
account-make-token = Создать токен
account-guests = Гости
account-guests-allow = Разрешить всем, кто может обратиться к серверу, просматривать без входа
account-guests-hint = Они могут искать, смотреть превью и скачивать, но не могут ничего добавлять, изменять или удалять и не видят настройки.

## SignIn.vue

signin-create-title = Создайте учётную запись
signin-title = Вход
signin-setup-lede = На этом сервере ещё нет учётных записей. Та, что вы создадите здесь, может всё; учётные записи для других людей можно добавить позже в «Настройки → Учётная запись».
signin-overlay-lede = Просматривать здесь может любой, но для изменений нужна учётная запись.
signin-with = Войти через { $provider }
signin-or-password = или по имени и паролю
signin-username = Имя пользователя
signin-name = Имя
signin-new-password = Новый пароль
signin-password = Пароль
signin-password-rule = Не меньше 8 символов.
signin-confirm-password = Подтвердите пароль
signin-mismatch = Пароли не совпадают.
signin-setup-code = Код настройки
signin-setup-code-hint-before = Код, заданный этому серверу в{ " " }
signin-setup-code-hint-after = .
signin-creating = Создание…
signin-signing-in = Вход…
signin-create = Создать учётную запись
signin-submit = Войти
signin-keep-browsing = Продолжить просмотр
signin-directory-hint = Используйте имя и пароль { $directory }.
signin-forgot-1 = Забыли пароль Hoard? На сервере выполните{ " " }
signin-forgot-2 = { " " }(в Docker —{ " " }
signin-forgot-3 = ), чтобы задать новый.

## SetupWizard.vue (первый визит на новый сервер)

setup-title = Где будет храниться ваша библиотека?
setup-lede = Выберите на сервере папку, в которой Hoard будет хранить ваши паки, или укажите папку с уже имеющимися паками. Позже в настройках можно добавить и другие места хранения, на других дисках.
setup-lede-desktop = Выберите папку, в которой Hoard будет хранить ваши паки, или укажите папку с уже имеющимися паками. Позже в настройках можно добавить и другие места хранения, на других дисках.
setup-unreachable = Не удалось связаться с сервером: { $error }
setup-unreachable-desktop = Hoard не отвечает: { $error }
setup-try-again = Попробовать снова
setup-loading = Загрузка…
setup-start = Создать библиотеку
setup-ready = Ваша библиотека готова
setup-managed-before = Новые паки попадают в{ " " }
setup-managed-after = . Перетащите файлы, папки или архивы в любое место страницы, чтобы добавить первые.
setup-watched-before = Hoard отслеживает{ " " }
setup-watched-after = ; паки появляются по мере того, как он их находит. Чтобы ещё и загружать, добавьте место, где хранит файлы сам Hoard, в «Настройки → Места хранения».
setup-open = Открыть библиотеку

## ConnectApp.vue (приложение вроде Claude просит войти)

connect-title = Подключить { $app }?
connect-lede-1 = { " " }сможет искать, скачивать, описывать и размечать тегами всё в этом Hoard под учётной записью{ " " }
connect-lede-2 = . Управлять учётными записями и токенами доступа оно не сможет.
connect-back-before = Вы вернётесь на{ " " }
connect-back-after = . Отключить его можно в любой момент в «Настройки → Учётная запись».
connect-allow = Разрешить
connect-cancel = Отмена
connect-failed = Не удаётся подключить это приложение
connect-go-library = Перейти к библиотеке
connect-checking = Проверка…

## InsecureNotice.vue (строка поверх обычного http://)

insecure-text = Это обычный http://, а некоторым функциям нужен HTTPS: 3D на WebGPU, современный буфер обмена, установка Hoard как приложения и коннекторы Claude.
insecure-switch = Перейти на https://
insecure-set-up = Настроить HTTPS
insecure-dismiss = Больше не показывать

## FormatWizard.vue («Настройки → Файлы магазинов» → обучение Hoard файлу магазина)

formatwizard-too-big = Этот файл слишком большой, чтобы читать его как файл магазина.
formatwizard-not-json = Этот файл не является корректным JSON, поэтому показан как текст.
formatwizard-read-failed = Не удалось прочитать файл: { $error }
formatwizard-back-title = Вернуться к форме?
formatwizard-back-body = Изменения в TOML будут потеряны: форма заново запишет формат из того, что в ней есть.
formatwizard-back-to-form = Вернуться к форме
formatwizard-miss-name = Он не берёт файлы с именем { $file }. Добавьте его в «Файлы».
formatwizard-miss-not-the-stores = В этом файле нет того, по чему он распознаёт файлы магазина. Измените «Распознавать по».
formatwizard-miss-no-title = В этом файле он не находит названия. Выберите название.
formatwizard-miss-short-name = не то имя
formatwizard-miss-short-not-the-stores = не этого магазина
formatwizard-miss-short-no-title = нет названия
formatwizard-saved-packs =
    { $file } сохранён. { $packs ->
        [one] { $packs } пак будет прочитан заново.
        [few] { $packs } пака будут прочитаны заново.
        [many] { $packs } паков будут прочитаны заново.
       *[other] { $packs } пака будут прочитаны заново.
    }
formatwizard-saved = { $file } сохранён.
formatwizard-save-failed = Не удалось сохранить формат: { $error }
formatwizard-zip-failed = Не удалось создать zip: { $error }
formatwizard-stop-title = Прекратить обучение этому файлу?
formatwizard-stop-body = Формат не сохранён.
formatwizard-stop = Прекратить
formatwizard-label = Мастер форматов
formatwizard-changing = Изменение: { $name }
formatwizard-starting-from = На основе: { $name }
formatwizard-title = Научить Hoard файлу магазина
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }из пака{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Редактировать как TOML
formatwizard-file-reference = Файл, для справки.
formatwizard-click-json = Щёлкните в файле поле «{ $field }».
formatwizard-click-text = Щёлкните в файле поле «{ $field }» или выделите его.
formatwizard-pick-first = Выберите поле справа, затем щёлкните здесь его значение.
formatwizard-loading = Загрузка…
formatwizard-the-format = Формат
formatwizard-syntax-before = Синтаксис описан в{ " " }
formatwizard-syntax-after = .
formatwizard-store = Магазин
formatwizard-store-placeholder = Название магазина, например Kenney
formatwizard-files = Файлы
formatwizard-recognize = Распознавать по
formatwizard-recognize-json = Ключ, который есть только в файлах магазина
formatwizard-recognize-text = Строка, которая есть только в файлах магазина
formatwizard-recognize-hint = По одному способу на строку; подойдёт любой. Нужно, когда у других файлов такое же имя.
formatwizard-seller = Продавец
formatwizard-seller-placeholder = Когда в файле не указан
formatwizard-tags = Теги
formatwizard-tags-placeholder = Для всего, что он описывает, через запятую
formatwizard-fields = Поля
formatwizard-selector = Селектор поля «{ $field }»
formatwizard-clear = Очистить поле «{ $field }»
formatwizard-pick = Выбрать
formatwizard-what-it-reads = Что он читает
formatwizard-read-title = Название
formatwizard-read-seller = Продавец
formatwizard-read-license = Лицензия
formatwizard-read-url = Страница в магазине
formatwizard-read-kind = Вид
formatwizard-read-tags = Теги
formatwizard-read-origin = Где сделано
formatwizard-read-size = Размер
formatwizard-also = { $format } тоже читает этот файл. Этот формат будет пробоваться первым.
formatwizard-library =
    { $files ->
        [one] { $files } файл
        [few] { $files } файла
        [many] { $files } файлов
       *[other] { $files } файла
    } в { $packs ->
        [one] { $packs } паке
        [few] { $packs } паках
        [many] { $packs } паках
       *[other] { $packs } паках
    } — с подходящим именем. Проверено: { $tried }, прочитано: { $read }.
formatwizard-library-none = Ни у одного файла в библиотеке нет подходящего имени.
formatwizard-share = Поделиться
formatwizard-share-hint = Zip с форматом, этим файлом в качестве образца и тем, как читается образец, разложенный как в репозитории Hoard, чтобы прислать его и все его получили. Сначала уберите из образца всё, чем не стоит делиться.
formatwizard-sample = Образец, которым вы делитесь
formatwizard-download = Скачать { $file }
formatwizard-close = Закрыть
formatwizard-use = Использовать здесь
formatwizard-cancel = Отмена
settings-unreal-editor-what = Устанавливает Unreal Editor для процесса экспорта. Для экспорта ресурсов запустите его отдельно.
settings-unreal-download = Однократно загружает редактор Linux x86-64 от Epic в папку инструментов Hoard. Занимает десятки ГБ на диске и требует учётной записи GitHub, получившей доступ от Epic (см. «Вход в GitHub»).
settings-unreal-github = Вход в GitHub
settings-unreal-github-why = Epic публикует редактор для Linux как закрытый образ в реестре пакетов GitHub (ghcr.io), и Hoard загружает его оттуда с помощью crane, который поставляется вместе с Hoard. GitHub выдаёт его только учётным записям, получившим доступ от Epic, поэтому Hoard нужна учётная запись, от имени которой он будет его загружать. Hoard хранит токен вместе с остальными настройками и больше ни для чего его не использует.
settings-unreal-github-step-link = Свяжите свою учётную запись GitHub с учётной записью Epic Games.
settings-unreal-github-step-link-go = Как их связать
settings-unreal-github-step-invite = Примите приглашение в организацию EpicGames, которое GitHub пришлёт вам по почте (оно может прийти через несколько минут).
settings-unreal-github-step-token = Создайте токен GitHub (classic), отметив только read:packages.
settings-unreal-github-step-token-go = Создать токен
settings-unreal-github-step-enter = Введите здесь имя пользователя GitHub и токен, затем выберите «Автоматически».
settings-unreal-github-user = Имя пользователя GitHub
settings-unreal-github-token = Токен
settings-unreal-github-token-kept = Сохранён (введите новый, чтобы заменить его)
settings-unreal-github-save = Сохранить вход
settings-unreal-github-forget = Забыть
settings-unreal-github-saved = Вход в GitHub сохранён; повторяем загрузку
settings-unreal-github-forgotten = Вход в GitHub забыт
settings-unreal-github-from-server = Этому серверу также задан собственный вход (GHCR_TOKEN_FILE или DOCKER_CONFIG); он используется, если здесь ничего не сохранено.
settings-unreal-image = Версия или образ Unreal
settings-editor-path = Укажите путь к исполняемому файлу установленного { $tool }. Автоматическая установка доступна только в Docker.
error-unreal-image = Укажите версию Unreal 5, тег версии Epic dev/dev-slim или ссылку на образ Epic с SHA-256.
