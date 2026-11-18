## Грешке сервера

error-account-not-found = налог није пронађен
error-asset-gone = ресурс није пронађен
error-asset-not-found = ресурс { $id } није пронађен
error-blob-not-found = блоб { $hash } није пронађен
error-collection-not-found = колекција { $id } није пронађена
error-connection-not-found = веза није пронађена
error-derivative-not-found = изведена датотека ({ $kind }) за { $id } није пронађена
error-file-in-pack-not-found = { $path } у пакету { $id } није пронађен
error-format-not-found = формат { $file } није пронађен
error-frame-sheet-aseprite-only = лист сличица имају само Aseprite датотеке
error-frame-sheet-frames = ово има једну сличицу или превише да би се распоредиле
error-location-not-found = локација { $id } није пронађена
error-maintenance-task-not-found = задатак одржавања { $id } није пронађен
error-open-upload-not-found = отворено отпремање { $id } није пронађено
error-original-not-found = оригинална датотека за { $id } није пронађена
error-pack-not-found = пакет { $id } није пронађен
error-preview-too-big = ова слика је превелика за лагани преглед
error-project-not-found = пројекат { $name } није пронађен
error-saved-search-not-found = сачувана претрага { $id } није пронађена
error-sign-in-first = прво се пријавите (или пошаљите приступни токен)
error-those-assets-not-found = ти ресурси нису пронађени
error-thumbnail-not-found = сличица за { $id } није пронађена
error-token-not-found = токен није пронађен
error-upload-not-found = отпремање { $id } није пронађено
error-pack-needs-name = пакет мора имати назив
error-no-library = још нема локације библиотеке за чување датотека: додајте управљану у Подешавања → Локације библиотеке
error-upload-failed = отпремање није успело: { $reason }
error-upload-no-files = у отпремању нема датотека
error-analyze-nothing-to-do = нема шта да се ради: rules=false, а LLM је искључен
error-too-many-assets = превише ресурса у једном захтеву
error-too-many-items = превише ставки у једном захтеву
error-not-a-hash = ово није blake3 хеш у хексадецималном облику
error-merge-into-tag = into мора бити ознака
error-nothing-to-merge = нема шта да се споји
error-selection-size = наведите између 1 и { $max } ID-ова ресурса
error-collection-needs-name = колекција мора имати назив
error-collection-exists = већ постоји колекција под називом { $name }
error-collection-empty = колекција { $name } је празна
error-conversion-failed = претварање у { $format } није успело: { $reason }
error-max-size = max мора бити степен двојке од 64 до 16384
error-replace-ext = ext мора бити кратка екстензија датотеке
error-empty-file = празна датотека
error-replace-duplicate = претворена датотека је дупликат једне која је већ у пакету; Unreal пакет је спојен с њом
error-ai-address = Адреса ВИ услуге треба да почиње са http:// или https://
error-local-llm-action = action је list, download или delete, а последња два захтевају модел
error-role = Улога је view, edit или admin
error-admin-only = ово може само администратор
error-view-only = овај налог може да гледа, али не и да мења: администратор то може променити у Подешавања → Налог
error-portal-only = пријавите се на портал да бисте управљали налозима и токенима
error-not-from-portal = овај захтев није стигао са Hoard портала
error-account-name = Изаберите име од највише 64 знака
error-password-length = Користите лозинку од најмање { $min } знакова
error-too-many-codes = Превише погрешних кодова: сачекајте { $seconds } с
error-already-set-up = Налог већ постоји: пријавите се
error-too-many-tries = Превише погрешних покушаја: сачекајте { $seconds } с
error-directory-unavailable = Директоријум није могао да провери вашу лозинку: { $reason }
error-password-elsewhere = Вашу лозинку чува директоријум или добављач преко којег се пријављујете
error-current-password = Ваша тренутна лозинка није тачна
error-account-source = Налог се пријављује преко Hoard-а (internal), директоријума (ldap) или OAuth-а
error-account-exists = Налог с тим именом већ постоји
error-own-role = Вашу улогу може да промени други администратор
error-remove-self = Не можете уклонити сопствени налог док сте пријављени на њега
error-token-name = Назовите токен по ономе што ће га користити, у највише 80 знакова
error-token-role = Токен не може да ради више од налога који га прави
error-wrong-setup-code = То није код за подешавање који је серверу дат у HOARD_SETUP_CODE
error-wrong-password = Име и лозинка се не поклапају
error-folder-missing = { $path } не постоји (диск који није монтиран?)
error-s3-path = S3 локација изгледа као s3://bucket
error-s3-address = S3 адреса мора да почиње са http:// или https://
error-s3-header = додатно S3 заглавље изгледа као Name=value, а „{ $header }“ није такво
error-keychain = није могуће сачувати у системском складишту лозинки: { $reason }
error-not-built = ова копија Hoard-а је направљена без { $feature }
error-openai-auth-unreachable = auth.openai.com није доступан
error-openai-unexpected = неочекиван одговор од OpenAI-ја
error-openai-answered = OpenAI је одговорио { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID токен није JWT
error-chatgpt-id-token-issuer = ID токен је издао { $issuer }, а не { $expected }
error-chatgpt-id-token-audience = ID токен је намењен другој апликацији
error-chatgpt-id-token-expired = ID токен је истекао
error-chatgpt-id-token-nonce = ID токен не припада овој пријави
error-chatgpt-no-state = адреса нема state; копирајте целу адресу
error-chatgpt-not-started = ова пријава није започета овде или је започета пре више од 15 минута. Започните је поново из Подешавања.
error-chatgpt-canceled = пријава је отказана
error-chatgpt-refused = ChatGPT је одбио пријаву: { $reason }
error-chatgpt-no-code = адреса нема код за пријаву; копирајте целу адресу
error-chatgpt-other-registration = ChatGPT је одговорио за другу регистрацију овог сервера; започните пријаву поново
error-chatgpt-other-registration-desktop = ChatGPT је одговорио за другу копију Hoard-а; започните пријаву поново
error-chatgpt-not-registered = ChatGPT није довршио регистрацију Hoard-а; започните пријаву поново
error-chatgpt-code-used = код за пријаву је већ искоришћен или је истекао; пријавите се поново
error-chatgpt-refused-code = ChatGPT је одбио пријаву ({ $code })
error-chatgpt-no-id-token = ChatGPT није послао ID токен
error-chatgpt-plan-not-allowed = Пријављени сте, али коришћење плана није дозвољено. Пријавите се поново и дозволите га или уместо тога користите API кључ.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = нисте пријављени на ChatGPT; пријавите се у Подешавања → Означавање помоћу ВИ
error-chatgpt-refresh-later = ChatGPT тренутно није могао да освежи пријаву ({ $code }); биће поново покушано касније
error-chatgpt-signed-out-problem = ChatGPT је одјавио овај сервер ({ $code }). Пријавите се поново да бисте наставили означавање помоћу свог плана.
error-chatgpt-signed-out-problem-desktop = ChatGPT је одјавио Hoard ({ $code }). Пријавите се поново да бисте наставили означавање помоћу свог плана.
error-chatgpt-signed-out = ChatGPT је одјавио овај сервер ({ $code }); пријавите се поново у Подешавања → Означавање помоћу ВИ
error-chatgpt-signed-out-desktop = ChatGPT је одјавио Hoard ({ $code }); пријавите се поново у Подешавања → Означавање помоћу ВИ
error-openai-api-unreachable = api.openai.com није доступан
error-openai-broke-off = одговор од OpenAI-ја је прекинут
error-chatgpt-kept-refusing = OpenAI је стално одбијао ChatGPT пријаву; пријавите се поново у Подешавања → Означавање помоћу ВИ
error-chatgpt-request-failed = захтев није успео
error-chatgpt-cut-off = одговор је пресечен ({ $reason })
error-chatgpt-cut-off-no-reason = одговор је пресечен (без навођења разлога)
error-openai-ended-early = одговор од OpenAI-ја се завршио пре него што је био потпун
error-chatgpt-usage-limit = потрошња вашег ChatGPT плана за Hoard је засад исцрпљена. Погледајте или промените ограничење на { $url }
error-chatgpt-not-eligible = овај ChatGPT налог или радни простор не може да користи свој план у другим апликацијама. Уместо тога користите API кључ.
error-chatgpt-usage-unavailable = ChatGPT тренутно није могао да провери потрошњу плана; биће поново покушано касније
error-chatgpt-unsupported = коришћење ChatGPT плана не подржава { $param } с овим моделом
error-chatgpt-unsupported-images = коришћење ChatGPT плана не подржава { $param } с овим моделом; изаберите модел који прима слике или искључите слање сличица
error-chatgpt-unsupported-part = коришћење ChatGPT плана не подржава део захтева с овим моделом
error-chatgpt-port = port је онај преко којег прегледач приступа Hoard-у
error-chatgpt-paste-address = налепите целу адресу, која почиње са http://127.0.0.1
error-too-many-hashes = највише { $max } хешева одједном
error-credits-pack = пакет { $key }
error-credits-collection = колекција { $key }
error-format-too-long = формат је предугачак
error-not-started = сервер није покренут
error-not-started-desktop = Hoard није покренут
error-not-format-file = није датотека формата: { $name }
error-format-name = назив мора садржати слово или цифру од A до Z или од 0 до 9
error-format-miss-name = датотеке формата не обухватају { $file }
error-format-miss-store = формат не обухвата { $file } за продавницу (видите require)
error-format-miss-title = формат не налази наслов у { $file }
error-cant-download-as = .{ $ext } датотеке не могу да се преузму као { $format }
error-only-images-resize = величина може да се мења само сликама
error-https-no-certificate = Прво налепите или отпремите свој сертификат и његов кључ
error-https-no-cert-found = Сертификат није пронађен: налепите текст који почиње са -----BEGIN CERTIFICATE-----
error-https-no-key-found = Приватни кључ није пронађен: налепите текст који почиње са -----BEGIN PRIVATE KEY-----
error-https-several-keys = Има више приватних кључева: налепите само онај који иде уз сертификат
error-https-key-password = Кључ је заштићен лозинком: дајте га без ње (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Сертификат није исправан PEM
error-https-key-not-pem = Приватни кључ није исправан PEM
error-https-key-mismatch = Кључ не припада сертификату. Ако сте дали ланац, ставите сертификат свог сервера на прво место
error-https-unusable = Сертификат или кључ не могу да се користе
error-https-expired = Овај сертификат је истекао { $date }
error-https-not-yet-valid = Овај сертификат важи тек од { $date }
error-https-unreadable = То није сертификат који Hoard може да прочита
error-https-bad-name = „{ $name }“ није адреса коју сертификат може да садржи
error-https-test-timed-out = време за пробну везу је истекло
error-https-test-failed = пробна веза није успела
error-https-no-ca = Hoard још није направио своје сертификационо тело
error-https-unavailable = HTTPS није доступан на овом серверу
error-https-certificate-mode = Сертификат иде уз опцију „Мој сертификат“
error-https-test = Сертификат није радио у пробној вези: { $reason }
error-nothing-to-retry = ту нема ничега што може поново да се покуша
error-ai-unreachable = { $url } није доступан
error-ai-key-refused = { $url } је одбио API кључ ({ $status })
error-ai-answered = { $url } је одговорио { $status }: { $reason }
error-ai-not-json = { $url }/models није одговорио у JSON облику
error-model-server-refused = сервер модела: { $reason }
error-no-model-server = ниједан сервер модела не одговара на { $url }
error-not-llama-router = { $url } није llama.cpp сервер у router режиму, па не може да преузима моделе
error-model-download = преузимање модела { $model } није успело: { $reason }
error-model-delete = брисање модела { $model } није успело: { $reason }
error-unknown-kind = непозната врста ресурса { $kind }
error-watched-is-folder = праћена локација је фасцикла на овом серверу
error-cant-reach = { $path } није доступан: { $reason }
error-folder-full-path = наведите пуну путању фасцикле
error-cant-create = не може да се направи { $path }
error-cant-write = не може да се пише у { $path }
error-cant-read = не може да се чита { $path }
error-overlaps-location = то се преклапа с локацијом { $name } ({ $path })
error-overlaps-data = то се преклапа с Hoard-овом фасциклом података ({ $path })
error-overlaps-cache = то се преклапа с Hoard-овом фасциклом кеша ({ $path })
error-not-watched = { $name } није праћена фасцикла
error-watched-no-kinds = праћена фасцикла не прима нове датотеке, па нема врсте које би се мењале
error-keep-catch-all = уместо тога одредите другу локацију за све остало
error-location-holds-files = { $name } још садржи датотеке: прво их преместите на друге локације (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } није доступна, па Hoard не може да утврди да ли још садржи датотеке
error-relocate-running = датотеке се већ премештају између локација; покушајте поново кад то буде готово
error-watched-not-moved = { $name } је праћена фасцикла: Hoard не премешта њене датотеке
error-no-other-location = нема друге локације библиотеке у коју би се датотеке преместиле: прво додајте једну
error-full-path = наведите пуну путању
error-pick-folder = изаберите фасциклу
error-task-running = „{ $task }“ је већ покренут
error-token-account-gone = налог овог токена више не постоји, па токен нема омиљене ни претраге
error-favorites-too-many = највише 10.000 одједном
error-saved-search-name = сачувана претрага мора имати назив од највише 80 знакова
error-saved-search-too-long = та претрага је предугачка да би се сачувала
error-texture-pattern-map = Шаблон назива текстуре мора да садржи { "{" }map{ "}" }, иначе би све мапе једног материјала добиле исти назив
error-texture-pattern-fields = Шаблон назива текстуре може да користи { "{" }name{ "}" }, { "{" }map{ "}" } и { "{" }size{ "}" }
error-texture-name-chars = Називи текстура не могу да садрже < > : " / \ | ? или *
error-texture-map-type = Не постоји тип мапе под називом { $role }
server-role-base-color = боја
server-role-normal = нормала
server-role-normal-dx = нормала (DirectX)
server-role-roughness = храпавост
server-role-gloss = сјај
server-role-metalness = металност
server-role-ao = амбијентална оклузија
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = висина
server-role-bump = рељеф
server-role-cavity = шупљине
server-role-specular = спекулар
server-role-opacity = непрозирност
server-role-emissive = емисија
server-role-translucency = провидност
error-texture-suffix-missing = Дајте суфикс мапама типа { $map }
error-texture-suffix-chars = Суфикс за мапе типа { $map } не може да садржи < > : " / \ | ? или *
error-texture-ending-missing = Сваки ваш завршетак мора да наведе завршетак који се тражи
error-texture-ending-chars = Ваши завршеци не могу да садрже < > : " / \ | ? или *
error-texture-ending-twice = { $ending } се двапут налази у вашим завршецима
error-oauth-no-client-id = Ова веза нема client_id.
error-oauth-unknown-app = Ова апликација није (више) регистрована овде. Почните повезивање поново из апликације.
error-oauth-redirect = Апликација је тражила да буде враћена на адресу коју није регистровала.
error-oauth-response-type = Hoard издаје само кодове за пријаву (response_type=code).
error-oauth-pkce = Апликација мора да користи PKCE са S256 да би се пријавила на Hoard.
error-light-preview-images = лагани преглед имају само слике
error-no-light-preview = нема лаганог прегледа: { $reason }
error-no-frame-sheet = нема листа сличица: { $reason }
error-thumbnail-source = непознат извор сличице: { $source }
error-thumbnail-too-big = та слика је превелика за сличицу
error-thumbnail-models-only = сличицу коју исцртава прегледач могу да имају само модели
error-not-a-picture = није слика коју Hoard може да прочита: { $reason }
error-project-name = пројекат мора имати назив од највише 200 знакова
error-project-too-big = највише милион ресурса у једном пројекту
error-project-too-many-at-once = највише 100.000 одједном
error-server-name = Изаберите назив од највише { $max } знакова, у једном реду
error-tool-setting = { $tool }: изаберите искључено, аутоматски или наведите путању
error-ai-model = Изаберите модел који ВИ услуга треба да користи
error-public-address = Јавна адреса треба да почиње са http:// или https://
error-update-hook-address = Адреса која се позива за ажурирања треба да почиње са http:// или https://
error-update-hook-auth = Заглавље Authorization не може да садржи преломе редова ни друге контролне знакове
error-no-soundfont = На серверу нема SoundFont-а на { $path }
error-no-soundfont-desktop = Нема SoundFont-а на { $path }
error-workers = Позадински радници: између 1 и { $max }
error-ai-batch = Ресурса по ВИ захтеву: између 1 и 200
error-conversion-cache = Претворена преузимања: најмање 100 MB
error-conversion-keep-days = Правила претварања: оригинали се чувају од 1 до { $max } дана
error-conversion-target = Правила претварања: Hoard не може да претвара датотеке у { $format }
error-conversion-no-source = Правила претварања: свако правило мора да има бар један формат за претварање
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Правила претварања: .{ $ext } датотеке не могу да се претворе у { $format }
error-conversion-twice = Правила претварања: .{ $ext } је у два правила
error-conversion-cant-undo = Ову датотеку није претворило правило или је њен оригинал већ обрисан.
error-conversion-path-taken = На { $path } је сада друга датотека, па оригинал не може да се врати тамо.
error-limits-zero = Ограничења не могу бити нула
error-settings-object = очекиван је JSON објекат с подешавањима
error-bad-setting = неисправно подешавање: { $reason }
error-auth-default-role = Подразумевана улога је view, edit или admin
error-auth-session-hours = Сесије трају између 1 сата и године дана
error-auth-settings = неисправна подешавања аутентификације: { $reason }
error-ldap-settings = неисправна LDAP подешавања: { $reason }
error-ldap-no-server = Додајте адресу сервера директоријума
error-ldap-base-dn = Наведите где се налазе уноси корисника (base DN)
error-ldap-username-attribute = Наведите који атрибут садржи име којим се корисници пријављују
error-ldap-bind-dn = Наведите налог који тражи кориснике или их тражите анонимно
error-ldap-ca-cert = Сертификационо тело није PEM сертификат (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Адреса сервера директоријума је празна
error-ldap-server-form = { $server }: адреса сервера директоријума је host:port, ldap://… или ldaps://…
error-ldap-not-starttls = { $server }: то је LDAPS, а не StartTLS
error-ldap-no-certificate = у њему нема сертификата
error-ldap-password-again = Поново наведите лозинку налога који тражи кориснике: сачувана се шаље само серверима за које је дата
error-ldap-unreachable = ниједан сервер директоријума није доступан ({ $reasons })
error-ldap-no-anonymous = директоријум не дозвољава анонимно тражење корисника ({ $reason })
error-ldap-bind-failed = налог који тражи кориснике није могао да се пријави ({ $reason })
error-ldap-search = тражење корисника { $name } у { $base }
error-ldap-group-search = тражење група корисника { $name } у { $base }
error-ldap-several =
    { $count ->
        [one] { $count } унос у директоријуму одговара имену { $name }: сузите филтер корисника
        [few] { $count } уноса у директоријуму одговарају имену { $name }: сузите филтер корисника
       *[other] { $count } уноса у директоријуму одговара имену { $name }: сузите филтер корисника
    }
error-ldap-password-check = директоријум није хтео да провери лозинку корисника { $name } ({ $reason })
error-ldap-wrong-password = Директоријум не познаје корисника { $name } с том лозинком
error-oauth-client-id = Наведите client ID који је добављач направио за Hoard
error-oauth-addresses = Адресе за ауторизацију и токене треба да почињу са https://
error-oauth-userinfo-address = Адреса за податке о кориснику треба да почиње са https://
error-oauth-username-claim = Наведите који claim садржи име особе
error-oauth-secret-again = Поново наведите client secret: сачувани се шаље само адреси за токене за коју је дат
error-oauth-issuer = Издавалац треба да почиње са https://
error-oauth-discovery-unreachable = { $url } није доступан: { $reason }
error-oauth-discovery-status = { $url } је одговорио { $status }
error-oauth-discovery-bad = { $url } није discovery документ
error-oauth-discovery-incomplete = { $url } не наводи адресе за ауторизацију и токене
error-signin-no-group = { $name } није у групи која сме да користи Hoard
error-signin-other-source = Већ постоји Hoard налог под именом { $name } који се пријављује на други начин
error-signin-other-subject = Већ постоји Hoard налог под именом { $name } за неког другог код добављача
error-signin-no-account = { $name } још нема Hoard налог: администратор може да га дода у Подешавања → Налог
error-signin-bad-name = То име не може бити Hoard налог (највише 64 знака)
error-oauth-off = Пријава преко OAuth-а није укључена
error-oauth-bad-authorization-address = адреса за ауторизацију није исправна
error-oauth-refused = Добављач вас није пријавио: { $reason }
error-oauth-no-code = Добављач вас је вратио без пријаве
error-oauth-expired = Та пријава је трајала предуго или је започета на другом месту: покушајте поново
error-oauth-no-claim = Добављач није рекао ко сте (нема claim-а { $claim })
error-oauth-no-sub = Добављач није рекао ко сте (нема claim-а sub ни id)
error-oauth-token-unreachable = адреса добављача за токене није доступна
error-oauth-token-refused = Добављач није хтео да доврши пријаву: { $reason }
error-oauth-no-access-token = добављач није послао приступни токен
error-oauth-userinfo-unreachable = адреса добављача за податке о кориснику није доступна
error-oauth-userinfo-refused = Добављач није хтео да каже ко се пријавио ({ $status })
error-oauth-userinfo-not-json = подаци добављача о кориснику нису JSON
error-format-no-name = name је празно
error-format-no-files = files је празно: наведите које датотеке формат чита, нпр. ["*.json"]
error-format-empty-selector = празно
error-format-unopened-bracket = затворена заграда без отворене
error-format-unclosed-bracket = заграда није затворена
error-format-empty-step-dots = празан корак (две тачке заредом или тачка на неком крају)
error-format-unclosed-except = !( није затворено
error-format-unclosed-filter = [ није затворено
error-format-after-filters = { $rest } после кључа и филтера корака { $step }
error-format-except-needs-glob = !(…) иде после кључа који садржи *
error-format-except-not-glob = !(…) иде после кључа који садржи *, а не после { $key }
error-format-empty-step = празан корак
error-format-not-a-filter = [{ $filter }] није филтер: користите [], [key=value|other], [array], [string], [number], [bool] или [object]
error-format-filter-needs-value = [{ $filter }] захтева кључ и вредност
error-store-search-empty = нема шта да се тражи; унесите назив пакета
error-update-hook-not-set = прво подесите адресу која се позива
error-upload-stalled = отпремање датотеке { $path } је застало; пошаљите тај део поново
error-upload-busy = { $path } се већ уписује
error-upload-incomplete =
    { $count ->
        [one] { $count } датотека није потпуна, нпр. { $files }
        [few] { $count } датотеке нису потпуне, нпр. { $files }
       *[other] { $count } датотека није потпуно, нпр. { $files }
    }
error-upload-no-files-listed = нема датотека
error-upload-not-part = { $path } није део отпремања { $id } или је отпремање затворено
error-bad-path = неисправна путања
error-upload-part-offset = { $offset } није место где почиње део датотеке { $path }
error-upload-interrupted = отпремање је прекинуто: { $reason }
error-upload-part-long = део { $part } датотеке { $path } је дужи од { $len } бајтова
error-upload-part-short = део { $part } датотеке { $path }: примљено { $got } бајтова, очекивано { $len }
error-upload-file-long = { $path } је већи од најављеног
error-upload-file-short = { $path }: примљено { $got } бајтова, очекивано { $len }
error-upload-complete = отпремање је већ завршено
error-already-imported = већ увезено као „{ $name }“

## Текст сервера

server-store-place-readme = README
server-store-place-folders = фасцикле
server-store-why-linked = Веза се налази у пакету ({ $place })
server-store-why-same-title = Исти наслов као у пакету ({ $place })
server-store-why-by = Аутор { $author }, наведен у пакету ({ $place })
server-store-why-meshes = Отприлике онолико мрежа колико има пакет ({ $count })
server-store-why-textures = Отприлике онолико текстура колико има пакет ({ $count })
server-chatgpt-revoke-failed = Одјављени сте овде, али OpenAI није могао да буде обавештен. Да бисте били сигурни, прекините везу с Hoard-ом у ChatGPT-у: Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Пријављени сте на ChatGPT
server-chatgpt-signed-in = Hoard је пријављен. Означавање помоћу ВИ сада може да користи ваш ChatGPT план.
server-chatgpt-signed-in-as = Hoard је пријављен као { $email }. Означавање помоћу ВИ сада може да користи ваш ChatGPT план.
server-chatgpt-not-finished-title = Пријава није довршена
server-chatgpt-other-computer = Ако сте пријаву започели из Hoard-а на другом рачунару, копирајте адресу ове странице из траке за адресу и налепите је тамо у Hoard-ова Подешавања.
server-back-to-settings = Назад на Подешавања
server-issues-pack = Пакети који нису могли да се додају
server-issues-pack-blurb = Обришите пакет и додајте га поново или га отворите да видите шта садржи.
server-issues-damaged = Сачуване датотеке које су оштећене или нестале
server-issues-damaged-blurb = Пронашао их је задатак „Провери датотеке на оштећења“. Вратите их из резервне копије или поново додајте њихов пакет, па поново покрените „Провери датотеке на оштећења“.
server-issues-folder = Праћене фасцикле које не могу да се читају
server-issues-folder-blurb = Њихови пакети остају какви јесу док фасцикла поново не постане читљива.
server-issues-missing = Праћени пакети чија је фасцикла нестала
server-issues-missing-blurb = Сваки пакет се чува 3 дана, за случај да је његова фасцикла само недоступна, а затим се уклања.
server-issues-job = Позадински послови који нису успели
server-issues-job-blurb = Поновни покушај покреће посао испочетка.
server-issues-file = Датотеке које нису могле да се отворе
server-issues-file-blurb = Hoard није могао да их отвори или сачува. Праћени пакет их поново покушава при следећем скенирању; датотека која је већ у библиотеци задржала је оно што је имала раније.
server-issues-unpack = Архиве које нису могле да се распакују
server-issues-unpack-blurb = Свака се чува у свом пакету као обична датотека, па ништа из ње није у библиотеци.
server-issues-read = Датотеке које Hoard није успео да протумачи
server-issues-read-blurb = У библиотеци су, али без података, ознака или сличице које би читање дало.
server-issues-render = Модели на којима исцртавање сличица није успело
server-issues-render-blurb = Немају сличицу. Отварање у прегледачу модела можда ипак ради и може да сачува сличицу.
server-issues-convert = Модели које Blender није успео да претвори
server-issues-convert-blurb = Немају GLB копију, па не могу да се прикажу у 3D-у.
server-issues-unreal = Unreal пакети који нису могли да се претворе
server-issues-unreal-blurb = Unreal страница пакета садржи остале његове Unreal пакете.
server-issues-reformat = Датотеке које правило претварања није могло да претвори
server-issues-reformat-blurb = Остављене су какве су биле.
server-issues-materials = Пакети чији материјали нису могли да се утврде
server-issues-materials-blurb = Све текстуре су ту; само нису груписане у материјале.
server-issues-material-thumb = Материјали без сличице
server-issues-material-thumb-blurb = Материјал је пронађен, али његов преглед није могао да се исцрта.
server-issues-store-files = Датотеке продавница које нису могле да се прочитају
server-issues-store-files-blurb = Датотеке из продавнице из које пакет потиче, као што је Megascans JSON, које му дају назив и ознаке.
server-issues-scan-again = Скенирај поново
server-issues-retry = Покушај поново
server-issues-reanalyze = Поново анализирај пакет
server-issues-convert-again = Претвори поново
server-issues-look-again = Потражи поново
server-issues-read-again = Прочитај поново
server-issues-blob-missing = Сачувана датотека је нестала.
server-issues-blob-damaged = Сачувана датотека се не поклапа с оним што је сачувано: оштећена је.
server-issues-folder-gone = Фасцикла „{ $folder }“ није пронађена.
server-job-a-pack = пакет
server-job-ingest = Увоз: { $pack }
server-job-retag = Поновно означавање: { $pack }
server-job-llm-tag = Означавање помоћу ВИ: { $pack }
server-job-convert = Претварање помоћу Blender-а: { $pack }
server-job-unreal = Претварање Unreal пакета: { $pack }
server-job-reformat = Претварање датотека по правилима претварања: { $pack }
server-job-scan = Скенирање праћене фасцикле
server-job-relocate = Премештање датотека између локација библиотеке
server-job-looks = Утврђивање боја и изгледа
server-job-sounds = Преслушавање звукова
server-job-embed = Учење шта слике приказују
server-job-hear = Учење како звуци звуче
server-job-materials = Тражење материјала: { $pack }
server-job-read-files = Читање датотека: { $pack }
server-job-store-lookup = Тражење странице у продавници
server-maintenance-backup = Направи резервну копију базе
server-maintenance-backup-blurb = Доследна копија базе података (пакети, ознаке, белешке, колекције и Подешавања) у фасцикли backups поред ње. Датотеке библиотеке нису у њој: оне се никад не мењају кад се сачувају, па их свака синхронизација датотека чува.
server-maintenance-scrub = Провери датотеке на оштећења
server-maintenance-scrub-blurb = Поново чита сваку сачувану датотеку и проверава је према њеном отиску, да би открио датотеке које су се оштетиле или нестале с диска или NAS-а. На великој библиотеци траје неко време.
server-maintenance-reanalyze = Поново анализирај сваки пакет
server-maintenance-reanalyze-blurb = Поново чита сваку датотеку овом верзијом Hoard-а: сличице, прегледи, детаљи и ознаке из правила праве се изнова. Ваше измене остају; ВИ се не пита поново.
server-maintenance-clear-cache = Обриши кеширана претварања
server-maintenance-clear-cache-blurb = Брише датотеке сачуване од претварања „Преузми као“ (поново се праве кад се следећи пут затраже) и оно што је остало од прекинутог рада.
server-maintenance-shrink-thumbs = Смањи сличице
server-maintenance-shrink-thumbs-blurb = Чува сличице и приказе модела сачуване као PNG онако како тражи Подешавања → Перформансе → Сличице, што их обично чини неколико пута мањим. Оне с провидношћу и пиксел-арт остају PNG.
server-maintenance-gc = Обриши некоришћене датотеке
server-maintenance-gc-blurb = Брише сачуване датотеке на које више ништа не упућује, као што су датотеке трајно обрисаних пакета.
server-update-no-address = није подешена адреса која се позива
server-update-no-answer = нема одговора у року од 30 секунди

## Подешавања: Изглед

appearance-language = Језик
appearance-language-auto = Као у прегледачу ({ $name })

## Корен

root-unreachable = Hoard сервер није доступан: { $error }
root-unreachable-desktop = Hoard не одговара: { $error }
root-desktop-reopen = Овај прозор је одјављен. Затворите Hoard и поново га отворите да бисте наставили.
root-try-again = Покушај поново
root-lazy-connect = веза апликације
window-minimize = Умањи
window-maximize = Увећај
window-restore = Врати
window-close = Затвори

## Апликација: библиотека, горња трака и филтери

app-lazy-settings = подешавања
app-lazy-archive = архива
app-lazy-shortcuts = пречице
app-lazy-setup = подешавање
app-theme-system = Као тема система
app-theme-light = Светла тема
app-theme-dark = Тамна тема
app-color-red = Црвена
app-color-orange = Наранџаста
app-color-yellow = Жута
app-color-green = Зелена
app-color-teal = Тиркизна
app-color-blue = Плава
app-color-purple = Љубичаста
app-color-pink = Розе
app-color-brown = Браон
app-color-sand = Пешчана
app-color-gray = Сива
app-color-slate = Шкриљаста
app-color-black = Црна
app-color-white = Бела
app-color-swatch = { $color }: ресурси у којима преовлађује { $lower } боја
app-color-any = Било која боја
app-color-pick = Изабери било коју боју
app-color = Боја
app-imitation = Звучи као
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Напишите звук онако како бисте га спеловали: први долазе звуци који му личе
app-retry = Покушај поново
app-undo = Опозови
app-search-failed = Претрага није успела: { $error }
app-unreachable = Сервер није доступан: { $error }
app-unreachable-desktop = Hoard не одговара: { $error }
app-forget-project-title = Заборавити { $name }?
app-forget-project-body = Hoard престаје да га наводи тамо где се користе његови ресурси ({ $count }), док га следећи hoard sync поново не пријави. У пројекту се ништа не мења.
app-forget-project-confirm = Заборави
app-forget-project-failed = Није могуће заборавити { $name }: { $error }
app-forget-project-label = Заборави пројекат { $name }
app-forget-project = Заборави овај пројекат
app-save-name-like = слично као { $name }
app-save-name-favorites = омиљени
app-search-saved = Сачувано „{ $name }“: налази се у Филтерима, под Ваше
app-rank-alike = Најсличније
app-rank-color = Највише те боје
app-rank-match = Најбоље подударање
app-rank-imitation = Најсличнији
app-sort = Сортирање
app-sort-newest = Најновије
app-sort-name = Назив
app-sort-largest = Највеће
app-sort-oldest = Најстарије
app-collection-added =
    Додато у { $name }: { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
app-collection-add-failed = Није могуће додати у колекцију: { $error }
app-collection-removed = Уклоњено из { $name }: { $count }
app-collection-remove-failed = Није могуће уклонити из колекције: { $error }
app-ns-subject = Предмет
app-ns-style = Стил
app-ns-theme = Тема
app-ns-material = Материјал
app-ns-map = PBR мапа
app-ns-color = Боја
app-ns-mood = Расположење
app-ns-genre = Жанр
app-ns-license = Лиценца
app-ns-res = Резолуција
app-ns-duration = Трајање
app-ns-poly = Број полигона
app-ns-format = Формат
app-ns-weight = Величина
app-ns-tech = Техничко
app-ns-blender = Верзија Blender-а
app-ns-depth = Дубина бита
app-ns-ucs = Категорија звука (UCS)
app-ns-bpm = Темпо
app-ns-root = Основни тон
app-ns-level = Гласноћа
app-ns-ue = Unreal класа
app-ns-engine = Енџин
app-ns-extracted = Враћени извор
app-ns-script = Писмо
app-bulk-failed =
    Није могуће изменити { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }: { $error }
app-bulk-untagged = Ознака { $tag } уклоњена је са { $count } ресурса
app-bulk-tagged =
    Ознака { $tag } додата је на { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
app-bulk-category-set =
    Категорија { $category } постављена је за { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
app-bulk-category-cleared = Категорија је уклоњена са { $count } ресурса
app-credits-failed = Није могуће направити заслуге: { $error }
app-archived-assets =
    Архивирано: { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
app-archived-one = Архивирано: { $name }
app-open-failed = Није могуће отворити тај ресурс: { $error }
app-unnamed-pack = пакет
app-unnamed-collection = колекција
app-unnamed-asset = ресурс
app-home-version = Hoard { $version } — назад у библиотеку
app-home = Назад у библиотеку
app-search-label = Претражи ресурсе
app-search-placeholder = Претражи називе, путање, ознаке
app-views = Прикази
app-library = Библиотека
app-packs = Пакети
app-packs-working = { $count } у обради
app-update-title = Изашао је Hoard { $version } (овај сервер користи { $current }). Преузмите нову слику или извршну датотеку и поново покрените; база података се сама надограђује.
app-update = Ажурирај
app-theme = Тема
app-sign-in = Пријави се
app-archive-title = Архива: { $contents }
app-archive = Архива
app-settings = Подешавања
app-palette-title = Палета команди ({ $keys })
app-import-folder = Увези фасциклу
app-add-files = Додај датотеке
app-add-files-short = Додај
app-split-label = Како увести ову фасциклу
app-split-before = { "" }
app-split-after =
    { " " }садржи { $count ->
        [one] { $count } фасциклу или архиву.
        [few] { $count } фасцикле или архиве.
       *[other] { $count } фасцикли или архива.
    }
app-split-each = Увези сваку као посебан пакет ({ $count })
app-split-one = Увези као један пакет
app-cancel = Откажи
app-filters = Филтери
app-done = Готово
app-yours = Ваше
app-favorites = Омиљени
app-forget-search-label = Заборави сачувану претрагу { $name }
app-forget-search = Заборави ову претрагу
app-projects-title = Пројекти који се одавде синхронизују помоћу hoard sync, са оним што је имала њихова последња синхронизација
app-projects = Пројекти
app-project-synced = Синхронизовано { $when }
app-project-synced-by = Синхронизовано { $when }, корисник { $who }
app-collections = Колекције
app-no-filters = Нема филтера за ову претрагу.
app-kind = Врста
app-facets-more = још { $count }
app-facets-fewer = мање
app-searching = Претраживање…
app-searching-live = Претраживање
app-asset-count =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
app-clear-all = Обриши све
app-save-name-label = Назив ове претраге
app-save-name-placeholder = Дајте назив
app-save = Сачувај
app-save-search-title = Задржи ову претрагу у Филтерима, под Ваше
app-save-search = Сачувај претрагу
app-select-title = Изаберите више ресурса да бисте их заједно означили, категорисали, додали у колекцију, преузели или архивирали (X или Ctrl-клик)
app-select = Изабери
app-tile-size = Величина плочица
app-supporting-title = glTF бафери, библиотеке материјала, мапе материјала, материјали које носи само један модел и датотеке продавница које описују моделе. Долазе уз оно што их користи када то преузмете. Странице листа спрајтова, и његови спрајтови које пакет има и као засебне слике, приказују се уз лист. Ресурс који пакет има у више формата приказује се једном, у формату који Hoard најбоље чита.
app-supporting-hidden =
    { $count ->
        [one] Скривена је { $count } пратећа датотека
        [few] Скривене су { $count } пратеће датотеке
       *[other] Скривено је { $count } пратећих датотека
    }
app-supporting-show = Прикажи
app-supporting-shown = Укључујући пратеће датотеке
app-supporting-hide = Сакриј
app-folders = Фасцикле
app-folders-title = Прегледај пакет онако како је био на диску, фасциклу по фасциклу
app-folders-label = Фасцикле пакета
app-welcome-title = Превуците пакет ресурса било где на ову страницу
app-welcome-before = Раде zip и tar архиве, фасцикле или појединачне датотеке. Hoard их распакује, сваку датотеку чува једном према њеном хешу, чита величине, трајања и број полигона и све означава, тако да можете тражити нпр.{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }или{ " " }
app-welcome-after = .
app-welcome-import-folder = Увези фасциклу
app-welcome-choose-files = Изабери датотеке
app-none-similar = Још ништа друго не личи много на { $name }.
app-none = Ништа се не подудара.
app-none-query = Ништа се не подудара с „{ $query }“.
app-none-imitation = Још ниједан измерени звук не звучи много као „{ $word }“.
app-none-filters = Ништа се не подудара с овим филтерима.
app-none-query-filters = Ништа се не подудара с „{ $query }“ уз ове филтере.
app-clear-search-filters = Обриши претрагу и филтере
app-loading-more = Учитавање још
app-keys-hint = Стрелице померају, Enter отвара, Space пушта звук, X или Ctrl-клик бира више (дуг притисак на екрану осетљивом на додир).
app-resize-label = Промени величину панела с детаљима
app-resize-title = Превуците да промените величину · двоклик за ширину која одговара
app-selected-label = Изабрани ресурси
app-selected = Изабрано: { $count }
app-pick-assets = Изаберите ресурсе
app-download-zip = Преузми zip
app-credits-title = CREDITS.md за пакете из којих ово потиче: ко их је направио, њихове лиценце и наводи заслуга које траже
app-credits = Заслуге
app-prefer-sources-title = Unreal SoundWave-ови и текстуре чију је оригиналну датотеку Hoard вратио
app-prefer-sources = Враћени WAV/PNG уместо .uasset ({ $count })
bag-clear = Очисти избор
bag-glb-heading = Модели као GLB
bag-glb-text = Сваки модел као .glb датотека која стоји са Y нагоре и окренута напред, а њене текстуре као засебне датотеке у фасцикли textures коју модели деле. Све остало иде онако какво јесте.
bag-glb-download = Преузми GLB zip
bag-plain-heading = Датотеке какве јесу
bag-plain-text = Свака датотека онако како је увезена, са оним што јој треба, у фасцикли за сваки пакет.
bag-remove = Уклони { $name } из избора
bag-more = и још { $count }
app-bulk-tag-label = Ознака која се додаје изабраним ресурсима или уклања с њих
app-bulk-tag-placeholder = Ознака
app-bulk-tag-add-title = Додај ову ознаку сваком изабраном ресурсу
app-bulk-tag-add = Додај
app-bulk-tag-remove-title = Уклони ову ознаку са сваког изабраног ресурса
app-bulk-tag-remove = Уклони
app-bulk-category-label = Категорија за изабране ресурсе (празно је брише)
app-bulk-category-placeholder = Категорија
app-bulk-category-set-title = Постави категорију сваког изабраног ресурса
app-bulk-category-clear-title = Обриши категорију сваког изабраног ресурса
app-bulk-category-set-button = Постави
app-bulk-category-clear-button = Обриши
app-remove-from-collection = Уклони из { $name }
app-unfavorite = Уклони из омиљених
app-favorite = Додај у омиљене
app-select-all = Изабери све ({ $count })
app-archive-selected-title = Ван претраге док се не врате или трајно обришу, из Архиве
app-drop-items = Отпустите да бисте додали ставке ({ $count })
app-drop = Отпустите да додате у Hoard
app-drop-hint = Свака датотека или фасцикла највишег нивоа постаје посебан пакет. Гигабајти нису проблем; велике датотеке се настављају ако веза пукне.

## Палета команди

palette-group-go-to = Иди на
palette-group-actions = Радње
palette-group-theme = Тема
palette-group-kinds = Врсте
palette-group-packs = Пакети
palette-group-collections = Колекције
palette-group-projects = Пројекти
palette-group-saved = Сачуване претраге
palette-group-assets = Ресурси
palette-hint-current = тренутна
palette-library = Библиотека
palette-library-keywords = ресурси мрежа почетна assets grid home
palette-favorites = Омиљени
palette-favorites-keywords = омиљено звездица свиђа starred stars liked
palette-packs = Пакети
palette-archive = Архива
palette-archive-keywords = отпад корпа обрисано врати trash recycle bin deleted restore
palette-stats = Статистика библиотеке
palette-stats-keywords = статистика бројеви величина диск простор број statistics numbers size disk space count
palette-search = Претрага
palette-add-files = Додај датотеке…
palette-add-files-keywords = отпреми увези upload import
palette-import-folder = Увези фасциклу…
palette-import-folder-keywords = отпреми пакет upload pack
palette-select-all = Изабери свих приказаних ({ $count })
palette-select-all-keywords = означи check
palette-select-several = Изабери више…
palette-select-several-keywords = групно више ознака архивирај bulk multiple tag archive
palette-archive-selected = Архивирај изабране ({ $count })
palette-archive-selected-keywords = обриши уклони отпад delete remove trash
palette-clear-selection = Поништи избор
palette-clear-filters = Обриши филтере
palette-clear-filters-keywords = ресетуј reset
palette-hide-supporting = Сакриј пратеће датотеке
palette-show-supporting = Прикажи пратеће датотеке
palette-supporting-keywords = нормал мапе храпавост маске normal maps roughness masks
palette-account = Налог и приступни токени
palette-account-keywords = лозинка пријава корисник токен password login user token api
palette-sign-out = Одјави се ({ $name })
palette-sign-out-keywords = одјава log out logout
palette-sign-in = Пријави се
palette-sign-in-keywords = пријава налог лозинка log in login account password
palette-settings = Подешавања
palette-settings-keywords = blender ffmpeg llm ai ви означавање unreal администратор алати поставке tagging admin tools preferences
palette-shortcuts = Пречице на тастатури
palette-shortcuts-keywords = помоћ тастери help keys
palette-theme-keywords = тема изглед режим theme appearance mode
palette-theme-variant-keywords = изглед боја appearance color
palette-light-theme = Светла тема: { $name }
palette-dark-theme = Тамна тема: { $name }
palette-kind-all = Све врсте (престани да приказујеш само { $kind })
palette-kind-only = Само { $kind }
palette-title = Палета команди
palette-placeholder = Претражи ресурсе, иди на пакет, покрени команду…
palette-results = Резултати
palette-searching = Претраживање…
palette-no-match = Ништа се не подудара с „{ $query }“
palette-foot-move = померање
palette-foot-run = покретање
palette-foot-shortcuts = све пречице

## Пречице на тастатури

shortcuts-title = Пречице на тастатури
shortcuts-close = Затвори
shortcuts-anywhere = Било где
shortcuts-palette = Палета команди: претрага, скок, покретање
shortcuts-search = Претрага
shortcuts-this-list = Овај списак
shortcuts-escape = Затвори, поништи избор или затвори детаље
shortcuts-library = Библиотека
shortcuts-move = Кретање између плочица (или J и K)
shortcuts-open = Отвори детаље
shortcuts-play-sound = Пусти или заустави звук
shortcuts-select = Изабери или поништи избор
shortcuts-favorite = Додај у омиљене или уклони из њих
shortcuts-key-click = клик
shortcuts-select-range = Изабери опсег
shortcuts-select-all = Изабери све приказано
shortcuts-sound-player = Плејер звука
shortcuts-play-pause = Пусти или паузирај
shortcuts-skip = Премотај уназад или унапред
shortcuts-start-end = Почетак или крај
shortcuts-image-inspector = Прегледач слика
shortcuts-zoom = Увећај или умањи
shortcuts-fit = Уклопи у прозор
shortcuts-actual-pixels = Стварни пиксели
shortcuts-tile = Поплочај 3×3 ради провере спојева
shortcuts-background = Следећа позадина
shortcuts-channel = Прикажи један канал

## KindChips.vue

kinds-group = Врсте ресурса

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = додала особа
tag-source-rule = од офлајн означивача
tag-source-meta = измерено из датотеке
tag-source-ai = предлог ВИ
tag-remove = Уклони ознаку { $tag }

## ActivityIndicator.vue

activity-label-idle = Позадинска активност: мирује
activity-label-running = Позадинска активност: { $running } у току
activity-label-queued = Позадинска активност: { $running } у току, { $queued } на чекању
activity-title = Позадинска активност
activity-unreachable = Сервер није доступан
activity-unreachable-desktop = Hoard не одговара
activity-empty = Ништа није у току. Увози, означавање, претварања и zip преузимања приказују се овде док раде.
activity-queued = на чекању
activity-seconds = { $s } с
activity-minutes = { $min } мин
activity-hours = { $h } ч { $min } мин
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles =
    { $count ->
        [one] { $count } троугао
        [few] { $count } троугла
       *[other] { $count } троуглова
    }
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Пусти { $name }
tile-pause = Паузирај { $name }
tile-favorite = Додај у омиљене: { $name }
tile-in-favorites = У вашим омиљенима
tile-add-favorite = Додај у омиљене (F)
tile-more-like = Још сличних као { $name }
tile-more-like-this = Још сличних
tile-select = Изабери { $name }

## CollectionPicker.vue

collections-add = Додај у колекцију
collections-menu = Колекције
collections-find-label = Пронађи или направи колекцију
collections-find-placeholder = Пронађите или дајте назив новој колекцији
collections-create = Направи
collections-empty = Још нема колекција. Унесите назив да бисте започели једну.

## FolderPicker.vue

folders-list = Фасцикле
folders-close = Затвори
folders-browse = Прегледај…
folders-up-title = Једна фасцикла навише
folders-up = Горе
folders-this-computer = Овај рачунар
folders-none = Овде нема фасцикли.

## LocationForm.vue

locations-mode-question = Шта Hoard треба да ради с овом фасциклом?
locations-mode-managed = Чувај моју библиотеку овде
locations-mode-managed-hint = Пакети које отпремите или увезете копирају се у ову фасциклу. Hoard се брине о њој, зато не дирајте њен садржај.
locations-mode-watched = Индексирај фасциклу коју већ имам
locations-mode-watched-hint = Свака фасцикла у њој постаје пакет. Hoard чита датотеке тамо где јесу и никад их не мења, не премешта нити брише, а нове фасцикле преузима чим се појаве.
locations-folder = Фасцикла на серверу
locations-folder-desktop = Фасцикла
locations-folder-placeholder = Фасцикла у којој су ваши пакети
locations-folder-hint-managed = Hoard је прави ако не постоји. У Docker-у, ово је фасцикла коју сте монтирали у контејнер.
locations-folder-hint-managed-desktop = Hoard је прави ако не постоји.
locations-folder-hint-watched = У Docker-у прво монтирајте фасциклу у контејнер (само за читање је у реду), па је изаберите овде.
locations-folder-hint-watched-desktop = Hoard је само чита: ништа у њој се не мења.
locations-folder-hint-cloud = Може и фасцикла коју синхронизује Dropbox, Google Drive, OneDrive или iCloud Drive, као и s3://bucket-name за бакет у S3 складишту.
locations-name = Назив
locations-name-placeholder-managed = Библиотека
locations-name-placeholder-watched = Моји пакети
locations-files = Датотеке које иду овде
locations-catch-all = И све што ниједна друга локација не прима
locations-kinds-hint = Датотеке новог пакета иду на локацију која има означену њихову врсту. Остале иду на локацију за све остало.
locations-files-all = Засад све. Касније из Подешавања додајте још локација да бисте неке врсте (на пример музику) слали на други диск.
locations-first-watched-note = За отпремање је потребна библиотека коју Hoard чува. Можете је додати касније у Подешавања → Локације библиотеке.
locations-checking = Провера фасцикле…
locations-add = Додај локацију
locations-cancel = Откажи

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Није могуће учитати локације библиотеке
locations-takes-and-rest = { $kinds } и све остало
locations-takes-rest = Све што друге не примају
locations-takes-everything = Све
locations-takes-nothing = Још ништа
locations-save-failed = Није могуће сачувати локацију
locations-unwatch-title = Престати с праћењем { $name }?
locations-unwatch-body = Пакети из ње ({ $count }) напуштају Hoard, заједно са својим ознакама и белешкама. Датотеке у фасцикли се не дирају.
locations-unwatch-body-empty = Датотеке у фасцикли се не дирају.
locations-unwatch = Престани с праћењем
locations-remove-title = Уклонити { $name }?
locations-remove-body = Све датотеке у њој прво се премештају на друге локације, према врсти; уклања се кад се испразни. Ништа се не губи.
locations-remove-body-only = Ако садржи датотеке, прво додајте другу локацију у коју ће се преместити.
locations-remove = Уклони
locations-removed = Уклоњено: { $name }
locations-emptying = Датотеке се премештају из { $name }; биће уклоњена кад се то заврши
locations-offline-title = { $name } није доступна
locations-offline-body = Њене датотеке не могу да се преместе док је недоступна. Ипак је уклонити? Све датотеке које су још на њој биће приказане као недостајуће.
locations-remove-anyway = Ипак уклони
locations-remove-failed = Није могуће уклонити { $name }
locations-rebalancing = Сачуване датотеке се премештају на локације које примају њихову врсту
locations-rebalance-failed = Није могуће преместити датотеке
locations-scanned-never = још није скенирано
locations-scanned-now = скенирано малопре
locations-scanned-minutes = скенирано пре { $mins } мин
locations-scanned-hours = скенирано пре { $hours } ч
locations-scanned-date = скенирано { $date }
locations-scan-failed = Није могуће скенирати { $name }
locations-watching = Прати се: { $name }
locations-added = Додато: { $name }
locations-loading = Учитавање…
locations-managed-title = Библиотеке које Hoard чува
locations-watched-title = Праћене фасцикле
locations-managed-hint = Отпремања и увози се копирају овде. Свака локација прима врсте које има означене; локација за све остало прима остатак.
locations-watched-hint = Фасцикле с пакетима индексиране тамо где јесу. Hoard никад не мења датотеке у њима и на сваких 15 минута тражи нове и уклоњене фасцикле.
locations-managed-empty = Још ниједна, па нема где да се смести отпремање. Додајте једну испод.
locations-reachable = Доступна
locations-unreachable-now = Тренутно није доступна
locations-catch-all-tag = за све остало
locations-packs =
    { $count ->
        [one] { $count } пакет
        [few] { $count } пакета
       *[other] { $count } пакета
    }
locations-unreachable = није доступна
locations-scanning = Скенирање…
locations-scan = Скенирај сада
locations-edit = Уреди
locations-catch-all-locked = Да бисте ово променили, одредите другу локацију за све остало
locations-edit-hint-before = Важи за датотеке додате од сада. Да бисте преместили оно што је већ сачувано, користите
locations-edit-hint-after = испод.
locations-save = Сачувај
locations-rebalance = Премести сачуване датотеке у складу с тим
locations-rebalance-hint = Након промене онога што локација прима: већ сачуване датотеке премештају се на локацију на коју њихова врста сада иде.
locations-add-title = Додај локацију
locations-add-hint = Још један диск за неке врсте датотека или фасцикла с пакетима коју већ имате.
locations-s3-title = S3 складиште
locations-s3-hint = Локација s3://bucket-name је бакет у овом сервису: AWS или било ком сервису компатибилном са S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage на NAS-у).
locations-s3-endpoint = Адреса
locations-s3-endpoint-placeholder = Празно за AWS; нпр. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Регион
locations-s3-access-key = ИД приступног кључа
locations-s3-secret-key = Тајни приступни кључ
locations-s3-headers = Додатна заглавља
locations-s3-headers-placeholder = Name=value, једно по реду (на пример за Cloudflare Access)
locations-s3-kept = Сачувано (куцајте да замените)
locations-s3-secrets = Кључеви и заглавља чувају се у подешавањима сервера и више се не приказују.
locations-s3-secrets-desktop = Кључеви и заглавља чувају се у складишту лозинки овог рачунара и више се не приказују.
locations-s3-saved = S3 складиште је сачувано
locations-s3-save-failed = Није могуће сачувати S3 складиште
locations-add-more = Додај локацију…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Није могуће учитати Проблеме
issues-started =
    { $count ->
        [1] Поново покренуто.
        [one] Поново покренута { $count } ставка.
        [few] Поново покренуте { $count } ставке.
       *[other] Поново покренуто { $count } ставки.
    }
issues-retry-failed = Поновни покушај није успео
issues-dismiss-failed = Одбацивање није успело
issues-dismiss-all-title = Одбацити све ({ $count })?
issues-dismiss-all-body = „{ $title }“ се брише. Све што поново пође наопако биће поново наведено.
issues-dismiss-all = Одбаци све
issues-just-now = малопре
issues-minutes-ago = пре { $mins } мин
issues-hours-ago = пре { $hours } ч
issues-loading = Учитавање…
issues-none = Ништа није пошло наопако.
issues-action-all = { $action } све
issues-open-asset = Отвори
issues-open-pack = Прикажи пакет
issues-open-locations = Локације библиотеке
issues-where-before = у
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } пут
        [few] { $count } пута
       *[other] { $count } пута
    }
issues-starting = Покретање…
issues-dismiss-named = Одбаци { $title }
issues-dismiss = Одбаци
issues-show-fewer = Прикажи мање
issues-show-all = Прикажи све ({ $count })
issues-not-listed = Још { $count } није на списку.

## archive.ts

archive-anyway = Ипак архивирај
archive-failed = Архивирање није успело: { $error }
archive-undo = Опозови
archive-restore-failed = Враћање није успело: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } пакет
        [few] { $packs } пакета
       *[other] { $packs } пакета
    } и { $assets ->
        [one] { $assets } ресурс
        [few] { $assets } ресурса
       *[other] { $assets } ресурса
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } пакет
        [few] { $packs } пакета
       *[other] { $packs } пакета
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } ресурс
        [few] { $assets } ресурса
       *[other] { $assets } ресурса
    }
archive-count-nothing = ништа

## ArchiveView.vue

archive-load-failed = Није могуће учитати архиву: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Враћено: { $what }
archive-purged = Трајно обрисано: { $what }
archive-purge-failed = Брисање није успело: { $error }
archive-purge-title = Трајно обрисати { $name }?
archive-purge-pack-body = Нестају и његови ресурси ({ $count }) и { $size } датотека, осим датотека које дели неки други пакет. Ово не може да се опозове.
archive-purge = Обриши заувек
archive-purge-asset-body = Његова датотека се ослобађа, осим ако је дели неки други ресурс. Ово не може да се опозове.
archive-empty-title = Испразнити архиву?
archive-empty-body = Трајно ће бити обрисано: { $what }, а њихове датотеке ослобођене. Ово не може да се опозове.
archive-empty = Испразни архиву
archive-title = Архива
archive-intro = Архивирани пакети и ресурси су ван претраге, бројања и списка ознака. Вратите их или их трајно обришите да бисте ослободили простор.
archive-loading = Учитавање…
archive-is-empty = Архива је празна
archive-is-empty-hint = Архивирајте пакет или ресурс и он ће чекати овде док га не вратите или трајно обришете.
archive-packs = Пакети
archive-assets = Ресурси
archive-asset-count =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
archive-archived = архивирано { $when }
archive-from = из { $pack }
archive-restore = Врати
archive-show-more = Прикажи још (преостало: { $count })

## mine.ts

projects-favorite-add-failed = Није могуће додати у омиљене: { $error }
projects-favorite-remove-failed = Није могуће уклонити из омиљених: { $error }
projects-save-search-failed = Није могуће сачувати претрагу: { $error }
projects-forget-search-failed = Није могуће уклонити { $name }: { $error }

## projects.ts

projects-list-and = { $items } и { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = још { $count }
projects-other =
    { $count ->
        [one] још { $count } пројекат
        [few] још { $count } пројекта
       *[other] још { $count } пројеката
    }
projects-usage-title-one = Ово користи { $name }
projects-usage-title =
    { $count ->
        [one] Ово користи { $count } пројекат
        [few] Ово користе { $count } пројекта
       *[other] Ово користи { $count } пројеката
    }
projects-usage-body-one = Користи се у: { $list }. Следећи hoard sync ће их уклонити оданде.
projects-usage-body = Користи се у: { $list }. Следећи hoard sync сваког од њих ће их уклонити оданде.

## uploader.ts

upload-network-error = грешка мреже
upload-gave-up = { $what }: { $error } после { $attempts } покушаја
upload-failed-plain = није успело
upload-already-imported = већ увезено
upload-part-failed = отпремање није успело ({ $status })
upload-batch = серија датотека ({ $count })
upload-batch-failed = серија није успела ({ $status })
upload-finish = завршетак отпремања

## UploadQueue.vue

upload-eta-seconds = преостало { $s } с
upload-eta-minutes = преостало { $min } мин
upload-eta-hours = преостало { $h } ч
upload-queue = Увози
upload-importing = Увоз пакета: { $count }
upload-bytes-of = { $sent } од { $total }
upload-rate = { $size }/s
upload-waiting = На чекању
upload-unpacking = Отпремљено, сада се распакује
upload-already = Већ је у библиотеци
upload-stopped = Заустављено
upload-files-of = { $done } од { $total } датотека
upload-resumed = настављено, { $size } је већ било ту
upload-skipped = Ове исте датотеке су већ увезене као „{ $name }“, па су прескочене.
upload-error = { $error }. Ваш напредак је сачуван: покушајте поново или касније поново превуците исте датотеке.
upload-retry = Покушај поново
upload-import-again = Ипак увези поново
upload-dismiss = Одбаци
upload-cancel = Откажи

## PacksView.vue

packs-rename-failed = Преименовање није успело: { $error }
packs-collection-deleted = Колекција { $name } је обрисана
packs-delete-failed = Брисање није успело: { $error }
packs-status-uploading = Отпремање (датотеке примљене до сада)
packs-status-ingesting = Распакивање и означавање
packs-status-analyzing = LLM означава
packs-status-ready = Спремно
packs-status-failed = Није успело
packs-retag-failed = Није могуће покренути поновно означавање: { $error }
packs-read-files-failed = Није могуће прочитати датотеке пакета: { $error }
packs-context-load-failed = Није могуће учитати податке о пакету из продавнице: { $error }
packs-undo-store-failed = Није могуће уклонити страницу продавнице: { $error }
packs-context-save-failed = Није могуће сачувати податке из продавнице: { $error }
packs-unreal-load-failed = Није могуће учитати стање Unreal претварања: { $error }
packs-unreal-discard-title = Одбацити оригинале?
packs-unreal-discard-body = Оригиналне .uasset датотеке у { $name } нестају након претварања. Подаци које разуме само Unreal, као што су графови материјала, трајно се губе.
packs-unreal-discard-confirm = Претвори и одбаци
packs-unreal-convert-failed = Није могуће покренути претварање: { $error }
packs-unreal-fact-converted = Претворено из Unreal-а: { $done }
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal пакет
        [few] { $left } Unreal пакета
       *[other] { $left } Unreal пакета
    }
packs-unreal-fact-both = Претворено из Unreal-а: { $done }, још у пакетима: { $left }
packs-thumbs-canceled = Отказано након { $done } од { $total }
packs-thumbs-drew =
    { $count ->
        [one] Исцртан { $count } модел
        [few] Исцртана { $count } модела
       *[other] Исцртано { $count } модела
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: замењена је { $improved } сличица.
        [few] { $lead }: замењене су { $improved } сличице.
       *[other] { $lead }: замењено је { $improved } сличица.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: замењена је { $improved } сличица, неисцртано: { $failed }.
        [few] { $lead }: замењене су { $improved } сличице, неисцртано: { $failed }.
       *[other] { $lead }: замењено је { $improved } сличица, неисцртано: { $failed }.
    }
packs-archived = Архивирано: { $name } и његови ресурси ({ $count })
packs-blobs-bad =
    { $count ->
        [one] { $count } датотека није прошла последњу проверу интегритета.
        [few] { $count } датотеке нису прошле последњу проверу интегритета.
       *[other] { $count } датотека није прошло последњу проверу интегритета.
    }
packs-collections = Колекције
packs-browse-named = Прегледај { $name }
packs-collection-name = Назив колекције
packs-save = Сачувај
packs-cancel = Откажи
packs-assets =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
packs-updated = Ажурирано { $date }
packs-browse = Прегледај
packs-collection-download = Преузми (.zip)
packs-collection-credits = Заслуге
packs-collection-credits-title = Ко је направио пакете из којих ови ресурси потичу, њихове лиценце и наводи заслуга које траже
packs-rename = Преименуј
packs-delete = Обриши
packs-packs = Пакети
packs-empty = Још нема пакета. Превуците zip или фасциклу било где на ову страницу да бисте га додали.
packs-find = Пронађи пакет
packs-find-placeholder = Пронађите пакет по називу, продавници, лиценци или ознаци
packs-sort = Редослед
packs-sort-new = Прво најновији
packs-sort-old = Прво најстарији
packs-sort-name = По називу
packs-sort-size = Прво највећи
packs-sort-assets = Прво они с највише ресурса
packs-with-kind = Пакети са
packs-state = Стање
packs-state-any = Било које стање
packs-state-working = У обради
packs-state-failed = Није успело
packs-state-untagged = Још нису описани
packs-store = Продавница
packs-store-any = Било која продавница
packs-store-from = Из продавнице { $store }
packs-store-none = Без странице продавнице
packs-shown-of = { $shown } од { $total }
packs-clear = Обриши
packs-no-match = Ниједан пакет се не подудара.
packs-clear-all = Обриши претрагу и филтере
packs-pack-name = Назив пакета
packs-thumbs-drawing = Исцртавање сличица у овом прегледачу…
packs-thumbs-drawing-count = Исцртавање сличица у овом прегледачу: { $done } од { $total }…
packs-thumbs-stopped = Заустављено: { $error }
packs-thumbs-none = Сваки модел овде већ има сличицу исцртану са свим његовим текстурама.
packs-dismiss = Одбаци
packs-license = Лиценца: { $license }
packs-license-tier = Лиценца: { $license } ({ $tier })
packs-license-not-found = није пронађена
packs-from-before = Извор:
packs-from-after = { "" }
packs-added = Додато { $date }
packs-download = Преузми пакет (.zip)
packs-store-details = Подаци из продавнице
packs-more-named = Још радњи за { $name }
packs-more = Још
packs-rename-more = Преименуј…
packs-retag-rules = Поново покрени офлајн означивач
packs-retag-llm = Поново покрени LLM означивач
packs-make-thumbs = Направи сличице у овом прегледачу
packs-make-thumbs-title = Отвара сваки модел у скривеном 3D прегледачу, један по један, и користи његову слику као сличицу где је боља од серверске
packs-make-thumbs-title-desktop = Отвара сваки модел у скривеном 3D прегледачу, један по један, и користи његову слику као сличицу где је боља од оне коју је направио Hoard
packs-read-files = Поново прочитај датотеке пакета
packs-credits = Преузми заслуге
packs-credits-title = CREDITS.md: ко је направио пакет, његова лиценца и навод заслуга који тражи
packs-unreal-open = Претвори Unreal садржај…
packs-archive = Архивирај пакет
packs-unreal-intro = Hoard сам чита Unreal пакете и оно што је у њима чува као обичне датотеке: текстуре као PNG или EXR, звукове као WAV, материјале као JSON. Претворена датотека заузима место пакета и задржава његове ознаке, везе и колекције.
packs-unreal-converted =
    { $count ->
        [one] претворен
        [few] претворена
       *[other] претворено
    }
packs-unreal-converted-kept =
    { $count ->
        [one] претворен, { $kept } уз задржан оригинал
        [few] претворена, { $kept } уз задржан оригинал
       *[other] претворено, { $kept } уз задржан оригинал
    }
packs-unreal-pending =
    { $count ->
        [one] још није прегледан
        [few] још нису прегледана
       *[other] још није прегледано
    }
packs-unreal-needs-editor =
    { $count ->
        [one] чека Unreal Editor
        [few] чекају Unreal Editor
       *[other] чека Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] без уобичајеног формата
        [few] без уобичајеног формата
       *[other] без уобичајеног формата
    }
packs-unreal-failed =
    { $count ->
        [one] није успео
        [few] нису успела
       *[other] није успело
    }
packs-unreal-reasons = Зашто су пакети преостали
packs-unreal-editor-before = За мреже и анимације потребан је Unreal Editor. На рачунару са инсталираним Unreal-ом покрените
packs-unreal-editor-command = hoard unreal-worker --editor <путања до UnrealEditor-Cmd>
packs-unreal-editor-after = ; он их извози као FBX и отпрема резултате овде.
packs-unreal-discard = Одбаци оригиналне .uasset датотеке након претварања
packs-unreal-discard-hint = Ослобађа простор. Подаци које разуме само Unreal, као што су графови материјала, губе се.
packs-unreal-drop = Обриши пакете без уобичајеног формата
packs-unreal-drop-hint = Blueprint-ови, нивои и други ресурси које разуме само енџин.
packs-unreal-convert = Претвори
packs-context-hint-before = Одакле пакет потиче и шта продавац каже о њему. LLM означивач ово чита уз сваку датотеку, па опис из продавнице као „120 чини леда и ватре“ помаже да именује
packs-context-hint-after = .
packs-store-auto = Пронађено аутоматски: сам пакет наводи ову страницу.
packs-store-not-this = Није ова
packs-lookup-label = Претражи продавнице за
packs-lookup-placeholder = Назив пакета у продавници
packs-lookup-searching = Претраживање…
packs-lookup-search = Претражи продавнице
packs-lookup-hide = Сакриј
packs-lookup-clues = Из датотека пакета: { $clues }.
packs-lookup-also = Претражено је и: { $queries }.
packs-lookup-quoted = „{ $query }“
packs-lookup-nothing = Ништа на Fab-у, itch.io, Kenney-ју или OpenGameArt-у не личи на њега. Пробајте с мање речи или с другим речима.
packs-lookup-use = Користи
packs-lookup-searched = Претражено { $date }.
packs-lookup-again = Претражи поново
packs-lookup-again-after = .
packs-lookup-unreachable = Овај пут није било могуће приступити: { $stores }.
packs-lookup-also-look = Потражите и на
packs-lookup-links-and = { " " }и{ " " }
packs-lookup-browser-only = (у њима може да се претражује само у прегледачу); налепите адресу странице испод.
packs-lookup-open = Пронађи у продавницама…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Продавац
packs-seller-placeholder = Ко га је направио
packs-license-label = Лиценца
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Ниво лиценце
packs-license-tier-placeholder = Лична, професионална…
packs-store-page = Страница продавнице
packs-store-text = Опис из продавнице
packs-store-text-placeholder = Налепите текст огласа или оставите празно да би се преузео са странице продавнице.
packs-store-text-fetch = Чување преузима опис са странице продавнице.
packs-readme = README пронађен у пакету
packs-save-retag = Сачувај и поново покрени LLM означивач

## AssetDetail.vue

asset-lazy-3d-viewer = 3D прегледач
asset-lazy-text = текст
asset-lazy-text-viewer = прегледач текста
asset-lazy-image-viewer = прегледач слика
asset-lazy-material-preview = преглед материјала
asset-lazy-material-viewer = прегледач материјала
asset-lazy-sprite-player = плејер спрајтова
asset-lazy-sheet-view = прегледач листова спрајтова
asset-rel-texture = текстура
asset-rel-buffer = подаци геометрије
asset-rel-material = датотека материјала
asset-rel-animation = анимација
asset-rel-library = повезана датотека
asset-rel-guess = вероватно иде уз
asset-rel-sidecar = подаци из продавнице
asset-rel-variant = варијанта
asset-rel-sprite = спрајт
asset-role-map = мапа типа { $role }
asset-maps-count =
    { $count ->
        [one] { $count } мапа
        [few] { $count } мапе
       *[other] { $count } мапа
    }
asset-needs = Потребно
asset-animations = Анимације
asset-sprites = Спрајтови
asset-used-by = Користи га
asset-loose-sprites = И као засебне датотеке
asset-in-sheet = У листу спрајтова
asset-other-formats = Други формати
asset-shown-as = У библиотеци приказан као
asset-conversion-failed = претварање није успело
asset-as-converting = Претварање…
asset-as-converting-percent = Претварање… { $percent }%
asset-as-queued = На чекању…
asset-download = Преузми
asset-as-label = Преузми као
asset-as-original = { $format } (оригинални формат)
asset-as-pack-files = Датотеке из пакета
asset-as-size = Величина
asset-as-full-size = Пуна величина
asset-as-up-to = До { $size } px
asset-as-hint = Изаберите величину или други формат
asset-meta-size = Величина
asset-meta-length = Трајање
asset-meta-sample-rate = Фреквенција узорковања
asset-meta-channels = Канали
asset-meta-mono = Моно
asset-meta-stereo = Стерео
asset-meta-bit-depth = Дубина бита
asset-meta-bits = { $bits } бита
asset-meta-level = Ниво
asset-meta-level-value = врх { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = врх { $peak } dBFS, RMS { $rms } dBFS, клипује
asset-meta-loop = Петља
asset-meta-loop-value = { $start } до { $end }
asset-meta-playback = Репродукција
asset-meta-one-shot = Једнократно
asset-meta-envelope = Анвелопа
asset-meta-envelope-times = атака { $attack }, реп { $tail }
asset-meta-envelope-hit = удар
asset-meta-envelope-swell = нарастање
asset-meta-envelope-sustained = одржан
asset-meta-tone = Боја тона
asset-meta-tone-value = осветљеност { $khz } kHz, шум { $noise } %
asset-meta-pitch = Висина тона
asset-meta-pitch-rises =
    { $octaves ->
        [one] расте за { $octaves } октаву
        [few] расте за { $octaves } октаве
       *[other] расте за { $octaves } октава
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] опада за { $octaves } октаву
        [few] опада за { $octaves } октаве
       *[other] опада за { $octaves } октава
    }
asset-meta-loops-cleanly = Понавља се без прекида
asset-meta-tempo = Темпо
asset-meta-root-note = Основни тон
asset-meta-cue-markers = Ознаке позиција
asset-meta-library = Библиотека
asset-meta-keywords = Кључне речи
asset-meta-title = Наслов
asset-meta-artist = Извођач
asset-meta-album = Албум
asset-meta-genre = Жанр
asset-meta-generator = Генератор
asset-meta-everything-with = Прикажи све са ознаком { $tag }
asset-meta-saved-with = Сачувано помоћу
asset-meta-color-mode = Режим боја
asset-meta-layers = Слојеви
asset-meta-range = Опсег
asset-meta-hdr = HDR (преглед је тонски мапиран)
asset-meta-format = Формат
asset-meta-large-document = Велики документ (PSB)
asset-meta-ai = ВИ
asset-meta-ai-made = Направљено генеративном ВИ
asset-meta-ai-edited = Уређено генеративном ВИ
asset-meta-ai-how-metadata = метаподаци
asset-meta-ai-how-generator = подешавања генератора
asset-meta-ai-how-iptc = IPTC тип дигиталног извора
asset-meta-ai-says = { $made }, према: { $how }
asset-meta-ai-by-says = { $made } ({ $by }), према: { $how }
asset-meta-provenance = Порекло
asset-meta-c2pa = Има C2PA Content Credentials
asset-meta-compression = Компресија
asset-meta-note = Белешка
asset-meta-maps = Мапе
asset-meta-sizes =
    { $count ->
        [one] Величина
        [few] Величине
       *[other] Величине
    }
asset-meta-triangles = Троуглови
asset-meta-vertices = Темена
asset-meta-meshes = Мреже
asset-meta-materials = Материјали
asset-meta-animations = Анимације
asset-meta-skins = Скинови
asset-meta-unreal-asset = Unreal ресурс
asset-meta-unreal-saved-with = { $class }, сачуван помоћу UE { $engine }
asset-meta-package = Пакет
asset-meta-cooked = Cooked (верзија игре): прегледи и извори су уклоњени
asset-meta-tiling = Поплочавање
asset-meta-seamless = Без спојева по обе осе
asset-meta-analysis-error = Ова датотека није могла у потпуности да се прочита: { $error }
asset-copy-blocked = Копирање није успело: прегледач га је блокирао
asset-save-error = Чување није успело: { $error }
asset-tag-removed = Уклоњено: { $tag }
asset-undo = Опозови
asset-play-error = Ова датотека не може да се пусти: { $error }
asset-copied-hash = Хеш је копиран
asset-copied-toml = toml је копиран
asset-copied-id = ID је копиран
asset-copied-link = Веза је копирана
asset-origin-before = Ова датотека је била
asset-origin-class-engine = , { $class } који је сачувао UE { $engine }.
asset-origin-class = , { $class }.
asset-origin-engine = { " " }коју је сачувао UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor ју је извезао { $date }.
asset-origin-editor-notes = Unreal Editor ју је извезао { $date } ({ $notes }).
asset-origin-hoard = Hoard ју је прочитао из пакета { $date }.
asset-origin-hoard-notes = Hoard ју је прочитао из пакета { $date } ({ $notes }).
asset-origin-download = Преузми оригинални .{ $ext }
asset-origin-discarded = Оригинални пакет је одбачен након претварања.
asset-details = Детаљи ресурса
asset-in-pack = у
asset-previous = Претходни ресурс
asset-next = Следећи ресурс
asset-favorite = Омиљено
asset-favorite-on-title = У вашим омиљенима (F га уклања)
asset-favorite-off-title = Додај у омиљене (F)
asset-share = Подели
asset-share-title = Подели везу до овог ресурса (ко је отвори, прво се пријављује у Hoard)
asset-layout-panel = Прикажи у бочном панелу
asset-layout-page = Прикажи преко целе странице
asset-layout-page-title = Прикажи преко целе странице: велики преглед поред детаља
asset-close = Затвори детаље
asset-rename-title = Кликните да преименујете
asset-name = Назив
asset-views-alt = Модел из четири правца
asset-inspect-aria = Прегледај { $name } преко целог екрана
asset-inspect-title = Прегледај: зумирање, поплочавање, канали
asset-playback-position = Позиција репродукције
asset-playback-value = { $current } од { $duration }
asset-pause = Паузирај
asset-play = Пусти
asset-waveform-drawing = Исцртавање таласног облика…
asset-waveform-skipped = Нема таласног облика за овако велике датотеке
asset-play-animation = Пусти као анимацију
asset-show-views = Прикажи лист с четири погледа који је видео означивач
asset-palette = Боје на слици
asset-swatch-title = { $hex }, { $percent }% слике. Пронађи ресурсе с овом бојом
asset-swatch-aria = Пронађи ресурсе са { $hex }, { $percent }% ове слике
asset-more-like-this = Још сличних
asset-desc-label = Ваш опис
asset-desc-placeholder = Шта је то, где бисте га користили…
asset-save = Сачувај
asset-cancel = Откажи
asset-desc-family = Чува се за све варијанте ({ $count })
asset-desc-edit = Уреди опис
asset-desc-write = Напиши опис
asset-desc-ai = Написала ВИ
asset-desc-file = Из датотеке
asset-desc-store-before = Са
asset-desc-store-page = странице { $store }
asset-desc-store-page-unnamed = странице продавнице
asset-desc-store-after = (цео пакет)
asset-less = Мање
asset-more = Више
asset-tags = Ознаке
asset-tag-add-label = Додај ознаку
asset-tag-add-placeholder = Додајте ознаку, нпр. mood:tense
asset-tag-add = Додај
asset-apply-family = Примени измене ознака, категорије и описа на све варијанте ({ $count })
asset-collections = Колекције
asset-collection-remove = Уклони из { $name }
asset-collection-add-another = Додај у другу
asset-collection-add = Додај у колекцију
asset-used-in = Користи се у
asset-used-in-title = Прикажи шта има { $name }
asset-used-in-synced = синхронизовано { $when }
asset-maps = Мапе
asset-show-more = Прикажи још { $count }
asset-materials-heading =
    { $count ->
        [one] Материјал
        [few] Материјали
       *[other] Материјали
    }
asset-material-maps-aria = { $name }: мапе
asset-variants = Варијанте (верзија: { $count })
asset-copies = Идентичне копије
asset-copies-note = Исти бајтови се чувају једном; ови пакети такође садрже ову датотеку.
asset-unreal-converted = Претворено из Unreal-а
asset-rule-converted = Претворено правилом
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Правило претварања направило је ову датотеку { $format } { $date } од
asset-rule-kept-until = Оригинал се чува до { $date }.
asset-rule-put-back = Врати оригинал
asset-rule-put-back-title = Вратити { $file }?
asset-rule-put-back-body = Претворена датотека се брише, а њено место заузима оригинал, с његовим ознакама, колекцијама и везама. Правила претварања га од тада не дирају.
asset-rule-put-back-done = Враћено: { $file }
asset-rule-let-go = Оригинал је обрисан кад је истекло време чувања оригинала задато у Подешавањима.
asset-unreal-package = Из Unreal пакета
asset-ue-needs-editor-before = Чека Unreal Editor: { $reason }. На рачунару са инсталираним Unreal-ом покрените
asset-ue-needs-editor-after = и вратиће се као FBX.
asset-ue-editor-path = путања до UnrealEditor-Cmd
asset-ue-unsupported = Задржано као пакет: { $reason }.
asset-ue-failed = Претварање није успело: { $reason }
asset-recovered = Оригинални .{ $ext } био је сачуван у овом пакету и Hoard га је извукао.
asset-recovered-download = Преузми .{ $ext }
asset-ue-meshes-before = Мреже и анимације се чувају у Unreal-овом сопственом формату. Претворите пакет (страница пакета, Још), па покрените
asset-ue-meshes-after = поред инсталације Unreal Editor-а да бисте добили FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } ознака регистра ресурса
        [few] { $count } ознаке регистра ресурса
       *[other] { $count } ознака регистра ресурса
    }
asset-file = Датотека
asset-bytes = Бајтови
asset-source = Извор
asset-path = Путања
asset-path-open-folder = Погледај шта још има у { $folder }
asset-download-maps =
    { $count ->
        [one] Преузми { $count } мапу
        [few] Преузми све { $count } мапе
       *[other] Преузми свих { $count } мапа
    }
asset-download-linked =
    { $count ->
        [one] Преузми уз { $count } повезану датотеку
        [few] Преузми уз { $count } повезане датотеке
       *[other] Преузми уз { $count } повезаних датотека
    }
asset-download-only-title = Само ова датотека, без датотека које користи
asset-download-only = Само ова датотека
asset-include-animations = Укључи анимације у .zip
asset-copy-toml = Копирај hoard.toml унос
asset-copy-id = Копирај ID
asset-teach = Научи Hoard ову датотеку
asset-teach-title = Опишите ову датотеку продавнице да би је Hoard читао (Подешавања → Датотеке продавница)
asset-archive = Архивирај
asset-archive-title = Ван претраге док се не врати или трајно обрише, из Архиве

## ImageViewer.vue

image-channel-red = Црвени канал
image-channel-green = Зелени канал
image-channel-blue = Плави канал
image-channel-alpha = Алфа канал
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Позадина шаховница (K мења)
image-bg-black-title = Црна позадина (K мења)
image-bg-gray-title = Сива позадина (K мења)
image-bg-white-title = Бела позадина (K мења)
image-bg-checker = позадина шаховница
image-bg-black = црна позадина
image-bg-gray = сива позадина
image-bg-white = бела позадина
image-inspect-aria = Прегледај { $name }
image-preview-size = преглед { $width }×{ $height }, { $bytes }
image-preview = преглед
image-resolution = Резолуција
image-full-title = Учитај целу слику { $width }×{ $height } (F); то чини и зумирање преко прегледа
image-full-loading = Учитавање пуне резолуције…
image-full-failed = Пуна резолуција се није учитала: покушајте поново
image-full = Пуна резолуција
image-zoom = Зум
image-zoom-out-title = Умањи (−)
image-zoom-out = Умањи
image-zoom-in-title = Увећај (+)
image-zoom-in = Увећај
image-fit-title = Уклопи (0)
image-fit = Уклопи
image-actual-title = Стварни пиксели (1)
image-view = Приказ
image-tile-title = Поплочај 3×3 ради провере спојева (T)
image-tile = Поплочај
image-channels = Канали
image-close = Затвори (Esc)
image-failed = Ова слика није могла да се учита.
image-loading = Учитавање…
image-key-gestures = Скролујте или скупите прсте за зум · превуците за померање · двоклик за увећање или уклапање
image-key-tile = поплочавање
image-key-background = позадина
image-key-full = пуна резолуција
image-key-channels = канали

## SpritePlayer.vue

sprite-frame-aria = { $name }, сличица { $frame } од { $count }
sprite-failed = Ово не може да се пусти.
sprite-loading = Учитавање сличица…
sprite-previous = Претходна сличица
sprite-next = Следећа сличица
sprite-pause = Паузирај анимацију
sprite-play = Пусти анимацију
sprite-animation = Анимација
sprite-all-frames = Све сличице
sprite-grid-title = Како је лист исечен на сличице
sprite-columns = Колоне
sprite-rows = Редови
sprite-fps-aria = Сличица у секунди
sprite-fps = сл./с

## Прегледач листова спрајтова (SheetView.vue) и његови формати за преузимање (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } спрајт
        [few] { $count } спрајта
       *[other] { $count } спрајтова
    }
sheet-pages =
    { $count ->
        [one] { $count } страница
        [few] { $count } странице
       *[other] { $count } страница
    }
sheet-animations =
    { $count ->
        [one] { $count } анимација
        [few] { $count } анимације
       *[other] { $count } анимација
    }
# The box that narrows the list of sprites by name.
sheet-find = Пронађи спрајт
# Under the list of sprites when more match than it draws.
sheet-showing = Приказано { $shown } од { $total }
sheet-no-match = Ниједан спрајт нема то у имену.
sheet-failed = Овај лист не може да се прочита.
sheet-loading = Учитавање листа…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoard-ов сопствени формат: странице какве јесу, спрајтови окренути на било коју страну и опсечени, с анимацијама и трајањем сваке сличице.
sheet-note-texturepacker = Спрајтови окренути у смеру казаљке на сату и опсечени, с анимацијама по имену, онако како их читају PixiJS и Phaser.
sheet-note-clockwise = Спрајтови окренути у смеру казаљке на сату и опсечени. Не може да чува анимације: сличице се препознају по нумерисаним именима.
sheet-note-libgdx = Спрајтови окренути супротно од смера казаљке на сату и опсечени. Не може да чува анимације: сличице се препознају по нумерисаним именима.
sheet-note-godot-frames = Спрајтови никад окренути, оно што је опсечено чува се као маргине, с анимацијама и трајањем сваке сличице.
sheet-note-godot-tileset = Плочице никад окренуте ни опсечене, на мрежи величине плочице; слике које су већ на мрежи остају какве јесу.
sheet-note-grid = Сваки спрајт усправан, на целом свом платну, у ћелијама исте величине, за све што лист сече по мрежи.
sheet-note-frames = Сваки спрајт усправан, на целом свом платну, сваки као засебан PNG назван по њему.

## TextViewer.vue, TextPreview.vue и text.ts

text-lang-config = Конфигурација
text-lang-table = Табела
text-lang-code = Код
text-lang-script = Скрипта
text-lang-text = Текст
text-copy-blocked = Копирање није успело: прегледач га је блокирао
text-read-aria = Читај { $name }
text-lines =
    { $count ->
        [one] { $count } ред
        [few] { $count } реда
       *[other] { $count } редова
    }
text-show-as = Прикажи као
text-tree-title = Стабло које се склапа (T)
text-tree = Стабло
text-pretty-title = Поново увучено (P)
text-pretty = Уредно
text-raw-title = Како је написано (R)
text-raw = Сирово
text-document-title = Као документ (D)
text-document = Документ
text-source-title = Markdown како је написан (R)
text-source = Извор
text-fold = Склопи
text-expand-all-title = Расклопи све
text-expand-all = Прошири све
text-collapse-all-title = Склопи све
text-collapse-all = Скупи све
text-group-text = Текст
text-wrap-title = Преломи дугачке редове (W)
text-wrap = Преломи
text-copy-title = Копирај текст (C)
text-copied = Копирано
text-copy = Копирај
text-download-title = Преузми датотеку
text-download = Преузми
text-close = Затвори (Esc)
text-truncated = Приказује се првих { $shown } од { $total }.
text-truncated-json = Приказује се првих { $shown } од { $total } (стаблу је потребна цела датотека).
text-show-all = Прикажи све
text-json-error = Овај JSON не може да се рашчлани: { $error }
text-contents-aria = Садржај датотеке { $name }
text-loading = Учитавање…
text-load-error = Није могуће учитати ову датотеку: { $error }
text-binary = Ова датотека је бинарна, а не текстуална, па овде нема шта да се чита. Преузмите је да бисте је отворили у одговарајућем алату.
text-key-tree = стабло
text-key-pretty = уредно
text-key-raw = сирово
text-key-document = документ
text-key-source = извор
text-key-wrap = прелом
text-key-copy = копирај
text-key-find = пронађи
text-key-close = затвори
text-preview-aria = Читај { $name } преко целог екрана
text-preview-read = Читај

## JsonTree.vue и JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } ставка
        [few] { $count } ставке
       *[other] { $count } ставки
    }
json-keys =
    { $count ->
        [one] { $count } кључ
        [few] { $count } кључа
       *[other] { $count } кључева
    }
json-fold = Склопи
json-unfold = Расклопи
json-unfold-summary = Расклопи { $summary }
json-show-more = Прикажи још { $count }
json-left = од преосталих { $count }
json-picker-more = … још { $count }

## 3D прегледачи модела

model-viewer-failed = 3D прегледач се није учитао ({ $error }); поновно учитавање странице може да помогне
model-loading-viewer = Учитавање 3D прегледача…
model-loading = Учитавање модела…
model-show-failed = Није могуће приказати овај модел: { $error }
model-preview-failed = Није могуће прегледати овај модел: { $error }
model-error-converting = сервер још претвара овај модел
model-error-converting-desktop = Hoard још претвара овај модел
model-error-empty-collada = празна Collada датотека
model-error-no-loader = Прегледач нема учитавач за .{ $ext }
model-dialog = Прикажи { $name } у 3D-у
model-title-stats =
    { $triangles ->
        [one] { $triangles } троугао
        [few] { $triangles } троугла
       *[other] { $triangles } троуглова
    } · { $meshes ->
        [one] { $meshes } мрежа
        [few] { $meshes } мреже
       *[other] { $meshes } мрежа
    } · { $materials ->
        [one] { $materials } материјал
        [few] { $materials } материјала
       *[other] { $materials } материјала
    }
model-triangles =
    { $count ->
        [one] { $count } троугао
        [few] { $count } троугла
       *[other] { $count } троуглова
    }
model-group-camera = Камера
model-group-shading = Сенчење
model-group-background = Позадина
model-group-animation = Анимација
model-group-view = Приказ
model-group-show = Прикажи
model-frame-title = Уоквири модел (F)
model-frame = Уоквири
model-reset-title = Назад на почетни приказ (R)
model-reset = Ресетуј
model-turn-title = Окрећи полако (A)
model-turn = Окрећи
model-textures-title = Текстуре (T)
model-textures = Текстуре
model-full-res-title = Текстуре пуне резолуције (H): свака датотека текстуре таква каква јесте, ма колико велика; искључено, лагани прегледи од највише 2048 px
model-full-res = Пуна рез.
model-light-title = Светло окружења (E)
model-light = Светло
model-grid-title = Мрежа пода (G)
model-grid = Мрежа пода
model-bg-checker = Позадина шаховница
model-bg-dark = Тамна позадина
model-bg-gray = Сива позадина
model-bg-light = Светла позадина
model-bg-env = Позадина окружења
model-bg-title = { $background } (K мења)
model-pause-title = Паузирај (Space)
model-play-title = Пусти (Space)
model-pause = Паузирај
model-play = Пусти
model-animation-clip = Анимациони клип
model-animation = Анимација
model-plays-on = Пусти на
model-close-title = Затвори (Esc)
model-full-screen = Цео екран
model-open-full-screen = Отвори 3D прегледач преко целог екрана
model-menu-fold = Склопи мени
model-menu-show = Прикажи мени
model-view-prefix = Приказ:{ " " }
model-menu-render = Рендер
model-menu-maps = Мапе
model-menu-inspect = Преглед
model-mode-final = Коначни рендер
model-mode-raw = Сирови рендер
model-mode-color = Основна боја
model-mode-roughness = Храпавост
model-mode-metalness = Металност
model-mode-normal = Нормала
model-mode-ao = Оклузија
model-mode-matcap = Matcap
model-mode-uv = UV провера
model-mode-geometry = Геометрије
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Жичани модел (W), преко било ког приказа
model-wireframe = Жичани модел
model-info-title = Подаци о сцени (I)
model-info = Подаци о сцени
model-info-close = Затвори податке о сцени
model-info-close-title = Затвори (I)
model-info-geometries = Геометрије
model-info-materials = Материјали
model-info-textures = Текстуре
model-info-triangles = Троуглови
model-info-vertices = Темена
model-info-meters = glTF је у метрима
model-info-no-units = Овај формат не наводи своје јединице
model-info-height = Висина
model-info-length = Дужина
model-info-width = Ширина
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } јединица
        [few] { NUMBER($n, maximumSignificantDigits: 3) } јединице
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } јединица
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Текстура није пронађена: { $name }
model-missing-some = Текстуре нису пронађене ({ $count }): { $names }
model-missing-more = Текстуре нису пронађене ({ $count }): { $names }…
model-missing-count = Текстуре нису пронађене: { $count }
model-guessed = Текстуре су упарене по називу; датотека не наводи ниједну.
model-group-facing = Оријентација
model-facing-turn = Окрени за четвртину
model-facing-turn-title = Окрени модел за четвртину круга супротно од казаљке на сату, гледано одозго
model-facing-tip = Нагни за четвртину
model-facing-tip-title = Нагни врх модела за четвртину круга унапред, да би модел који лежи стао усправно
model-facing-save = Користи за пакет
model-facing-save-title = Окрени овако све моделе у пакету { $pack }. Њихове сличице се поново исцртавају; преузимања остају каква су направљена.
model-facing-undo = Опозови
model-facing-undo-title = Назад на то како су модели пакета окренути
model-facing-saved = Сви модели у пакету { $pack } сада су овако окренути. Њихове сличице се поново исцртавају.
model-facing-failed = Није могуће окренути моделе пакета: { $error }
model-keys-mouse = Превуците за кружење · десно превлачење за померање · скролујте за зум · двоклик за уоквиравање
model-key-frame = уоквири
model-key-reset = ресетуј
model-key-view = приказ
model-key-wireframe = жичани модел
model-key-info = подаци о сцени
model-key-textures = текстуре
model-key-full-res = текстуре пуне резолуције
model-key-light = светло
model-key-grid = мрежа пода
model-key-background = позадина
model-key-play = пусти

## Материјали

material-role-base-color = Боја
material-role-normal = Нормала
material-role-normal-dx = Нормала (DirectX)
material-role-roughness = Храпавост
material-role-gloss = Сјај
material-role-metalness = Металност
material-role-ao = Амбијентална оклузија
material-role-orm = Оклузија, храпавост, металност
material-role-mra = Металност, храпавост, оклузија
material-role-rma = Храпавост, металност, оклузија
material-role-height = Висина
material-role-bump = Рељеф
material-role-cavity = Шупљине
material-role-specular = Спекулар
material-role-opacity = Непрозирност
material-role-emissive = Емисија
material-role-translucency = Провидност
material-role-base-color-name = боја
material-role-normal-name = нормала
material-role-normal-dx-name = нормала (DirectX)
material-role-roughness-name = храпавост
material-role-gloss-name = сјај
material-role-metalness-name = металност
material-role-ao-name = амбијентална оклузија
material-role-orm-name = оклузија, храпавост, металност
material-role-mra-name = металност, храпавост, оклузија
material-role-rma-name = храпавост, металност, оклузија
material-role-height-name = висина
material-role-bump-name = рељеф
material-role-cavity-name = шупљине
material-role-specular-name = спекулар
material-role-opacity-name = непрозирност
material-role-emissive-name = емисија
material-role-translucency-name = провидност
material-maps =
    { $count ->
        [one] { $count } мапа
        [few] { $count } мапе
       *[other] { $count } мапа
    }
material-dialog = Прикажи материјал { $name }
material-up-to = до { $size }
material-repeat-size = { $size } по понављању
material-shape-sphere = Сфера
material-shape-cube = Коцка
material-shape-plane = Раван
material-shape-title = { $shape } ({ $key })
material-group-shape = Облик
material-group-tiling = Поплочавање
material-real-title = Стварна величина: једно понављање је { $size }, сфера је пречника 2 m (T мења)
material-repeat-title = Понови { $n }× (T мења)
material-real = Стварна
material-repeat = { $n }×
material-height-title = Колико мапа висине издиже површину
material-height = Висина
material-height-strength = Јачина висине
material-loading = Учитавање материјала…
material-show-failed = Није могуће приказати овај материјал: { $error }
material-preview-failed = Није могуће прегледати овај материјал: { $error }
material-group-maps = Мапе
material-map-show = Прикажи мапу типа { $map }
material-map-hide = Сакриј мапу типа { $map }
material-map-unused = Преглед је не користи
material-open-full-screen = Отвори прегледач материјала преко целог екрана
material-keys-mouse = Превуците за кружење · десно превлачење за померање · скролујте за зум · кликните на мапу да је искључите
material-key-shape = облик
material-key-tiling = поплочавање
material-key-turn = окретање

## theme.ts

theme-slate = Шкриљац
theme-paper = Папир
theme-linen = Лан
theme-sage = Жалфија
theme-lilac = Јоргован
theme-blush = Руменило
theme-graphite = Графит
theme-obsidian = Опсидијан
theme-ember = Жар
theme-moss = Маховина
theme-aubergine = Патлиџан
theme-oxblood = Бордо
theme-light-slate-blurb = Хладна плавосива
theme-light-paper-blurb = Обична неутрална сива
theme-light-linen-blurb = Топла крем, ћилибар
theme-light-sage-blurb = Мека зелена
theme-light-lilac-blurb = Бледа љубичаста
theme-light-blush-blurb = Пепељаста ружичаста
theme-dark-graphite-blurb = Неутрална сива, без нијансе
theme-dark-slate-blurb = Плавосива
theme-dark-obsidian-blurb = Скоро црна, за OLED
theme-dark-ember-blurb = Топли угаљ, ћилибар
theme-dark-moss-blurb = Шумска зеленоцрна
theme-dark-aubergine-blurb = Шљивоцрна, јоргован
theme-dark-oxblood-blurb = Винскоцрна, корал

## texnames.ts

texnames-needs-map = Шаблон мора да садржи { "{" }map{ "}" }, иначе би све мапе једног материјала добиле исти назив
texnames-placeholders = Шаблон може да користи { "{" }name{ "}" }, { "{" }map{ "}" } и { "{" }size{ "}" }
texnames-bad-characters = Називи не могу да садрже < > : " / \ | ? или *
texnames-no-suffix = Дајте суфикс мапама типа { $role }
texnames-bad-suffix = Суфикс за мапе типа { $role } не може да садржи < > : " / \ | ? или *
texnames-slot-no-suffix = Дајте суфикс за „{ $slot }“
texnames-slot-bad-suffix = Суфикс за „{ $slot }“ не може да садржи < > : " / \ | ? или *
texnames-ending-missing = Сваки ваш завршетак мора да наведе завршетак који се тражи
texnames-ending-bad = Ваши завршеци не могу да садрже < > : " / \ | ? или *
texnames-ending-twice = { $ending } се двапут налази у вашим завршецима
texnames-slot-metallic-roughness = Металност и храпавост
texnames-slot-specular-color = Боја спекулара
texnames-slot-specular-glossiness = Спекулар и сјај
texnames-slot-clearcoat = Лак
texnames-slot-clearcoat-roughness = Храпавост лака
texnames-slot-clearcoat-normal = Нормала лака
texnames-slot-sheen-color = Боја сатенског сјаја
texnames-slot-sheen-roughness = Храпавост сатенског сјаја
texnames-slot-transmission = Трансмисија
texnames-slot-thickness = Дебљина
texnames-slot-diffuse-transmission = Дифузна трансмисија
texnames-slot-diffuse-transmission-color = Боја дифузне трансмисије
texnames-slot-iridescence = Иридесценција
texnames-slot-iridescence-thickness = Дебљина иридесценције
texnames-slot-anisotropy = Анизотропија

## storeFormats.ts

storefiles-field-title = Наслов
storefiles-field-title-hint = Тачан наслов огласа. Даје назив пакету, а без њега се датотека прескаче.
storefiles-field-seller = Продавац
storefiles-field-seller-hint = Ко га је направио или га продаје.
storefiles-field-license = Лиценца
storefiles-field-license-hint = Онако како је датотека пише, нпр. CC-BY-4.0.
storefiles-field-url = Страница продавнице
storefiles-field-url-hint = Адреса огласа, која почиње са http:// или https://.
storefiles-field-words = Ознаке
storefiles-field-words-hint = Речи којима се означава. Вредности одвојене зарезима се раздвајају.
storefiles-field-type = Врста
storefiles-field-type-hint = Шта је то, нпр. „3D модел“ или „површина“. Такође постаје ознака.
storefiles-field-colors = Боје
storefiles-field-colors-hint = Његове боје, као ознаке color:.
storefiles-field-id = ID продавнице
storefiles-field-id-hint = ID који му продавница даје: слова, цифре, _ или -.
storefiles-field-origin = Место израде
storefiles-field-origin-hint = Где је направљен или скениран, за опис.
storefiles-field-size = Величина
storefiles-field-size-hint = Колико је велик, речима, за опис.
storefiles-field-title-name = наслов
storefiles-field-seller-name = продавац
storefiles-field-license-name = лиценца
storefiles-field-url-name = страница продавнице
storefiles-field-words-name = ознаке
storefiles-field-type-name = врста
storefiles-field-colors-name = боје
storefiles-field-id-name = ID продавнице
storefiles-field-origin-name = место израде
storefiles-field-size-name = величина

## StoreFormatsPanel.vue

storefiles-load-failed = Није могуће учитати формате датотека продавница: { $error }
storefiles-remove-title = Уклонити { $name }?
storefiles-remove-body = { $file } се брише из фасцикле података. Пакети већ прочитани помоћу њега задржавају оно што је прочитао.
storefiles-remove = Уклони
storefiles-removed = Уклоњено: { $name }.
storefiles-remove-failed = Није могуће уклонити { $name }: { $error }
storefiles-unread = Датотеке које Hoard не чита
storefiles-loading = Учитавање…
storefiles-unread-none = Нема их: ниједна датотека у библиотеци не личи на датотеку непрочитане продавнице.
storefiles-unread-what = Датотеке у више пакета или назване као датотеке продавница, које ниједан формат не чита.
storefiles-unread-packs =
    { $count ->
        [one] { $count } пакет
        [few] { $count } пакета
       *[other] { $count } пакета
    }
storefiles-unread-example = нпр. { $pack }
storefiles-teach = Научи Hoard ову датотеку
storefiles-formats = Формати
storefiles-formats-what = Ваши се пробају први и замењују уграђени формат истог назива.
storefiles-dir-before = Чувају се у
storefiles-dir-after = .
storefiles-yours = Ваш
storefiles-built-in = Уграђен
storefiles-edit = Уреди
storefiles-start-from = Почни од овог

## TagsPanel.vue

tags-load-failed = Није могуће учитати ознаке: { $error }
tags-rename-title = Преименовати { $tag }?
tags-merge-title = Спојити ознаке ({ $count })?
tags-rename-body = Њени ресурси ({ $count }) уместо ње добијају { $into }, а означивачи од сада користе нови назив.
tags-merge-body = { $tags } постају { $into } на ресурсима (највише { $count }). Означивачи од сада користе { $into }.
tags-rename = Преименуј
tags-merge = Споји
tags-merged =
    { $count ->
        [one] { $count } ресурс сада има ознаку { $into }
        [few] { $count } ресурса сада имају ознаку { $into }
       *[other] { $count } ресурса сада има ознаку { $into }
    }
tags-merge-failed = Спајање није успело: { $error }
tags-forget-failed = Није могуће то променити: { $error }
tags-find = Пронађи ознаке
tags-find-placeholder = Пронађите ознаке, нпр. rock или set:
tags-count =
    { $count ->
        [one] { $count } ознака
        [few] { $count } ознаке
       *[other] { $count } ознака
    }
tags-count-more = { $count }+ ознака
tags-what = Изаберите ознаке које значе исто и спојите их или изаберите једну да је преименујете.
tags-new-name = Нови назив
tags-merge-into = Споји у
tags-saved-as-before = Сачувано као
tags-saved-as-after = { "" }
tags-cancel = Откажи
tags-loading = Учитавање…
tags-none = Нема ознака.
tags-none-containing = Нема ознака које садрже „{ $text }“.
tags-browse-title = Прегледај ресурсе означене са { $tag }
tags-browse = Прегледај
tags-merged-heading = Спојене ознаке
tags-merged-what = Када означивачи предложе неки од ових старих назива, ресурс добија ознаку у коју је стари назив спојен.
tags-allow-old = Поново дозволи стари назив

## LibraryStats.vue

stats-source-human = Додали људи
stats-source-ai = Предложила ВИ
stats-source-rule = Уграђена правила
stats-source-meta = Из датотека
stats-assets = Ресурси
stats-packs = Пакети
stats-on-disk = На диску
stats-duplicates-title =
    { $count ->
        [one] { $count } датотека је копија друге, бајт по бајт, и чува се једном
        [few] { $count } датотеке су копије других, бајт по бајт, и чувају се једном
       *[other] { $count } датотека су копије других, бајт по бајт, и чувају се једном
    }
stats-saved-dedup = Уштеђено уклањањем дупликата
stats-distinct-tags = Различите ознаке
stats-archive = У архиви
stats-load-failed = Није могуће учитати детаље: { $error }
stats-counting = Бројање…
stats-by-kind = Простор по врсти
stats-files-size =
    { $count ->
        [one] { $name }: { $count } датотека, { $size }
        [few] { $name }: { $count } датотеке, { $size }
       *[other] { $name }: { $count } датотека, { $size }
    }
stats-months = Додато у последњих годину дана
stats-month =
    { $count ->
        [one] { $month }: { $count } ресурс, { $size }
        [few] { $month }: { $count } ресурса, { $size }
       *[other] { $month }: { $count } ресурса, { $size }
    }
stats-tagging = Означавање
stats-tagged = Означено
stats-untagged = { $count } без ознака
stats-described = Описано
stats-described-assets =
    { $count ->
        [one] { $count } ресурс
        [few] { $count } ресурса
       *[other] { $count } ресурса
    }
stats-human-edited = Ручно уређено
stats-human-edited-unit =
    { $count ->
        [one] ресурс
        [few] ресурса
       *[other] ресурса
    }
stats-source-different =
    { $count ->
        [one] { $count } различита
        [few] { $count } различите
       *[other] { $count } различитих
    }
stats-most-used = Најкоришћеније
stats-show-tag = Прикажи ресурсе с ознаком { $tag }
stats-file-types = Типови датотека
stats-biggest-packs = Највећи пакети
stats-browse-pack = Прегледај { $name }
stats-largest-files = Највеће датотеке
stats-open-file = Отвори { $name }
stats-empty = У библиотеци још нема ничега. Додајте пакет и његови бројеви ће се појавити овде.

## UpdatesPanel.vue

updates-saved = Hoard ће је позвати кад ажурирање буде спремно
updates-saved-none = Нема адресе за позивање
updates-save-failed = Чување није успело
updates-call-answered = Одговорила је { $result }
updates-call-refused = Није прихватила захтев: { $result }
updates-call-failed = Позивање није успело
updates-called = Позвано { $when } за { $version }: { $result }
updates-called-failed = Позвано { $when } за { $version }: { $result }. Hoard покушава поново за сат времена.
updates-latest = Hoard { $version } је најновије издање.
updates-out = Изашао је Hoard { $version }.
updates-out-idle = Изашао је Hoard { $version }; позив следи за неколико минута, док се ништа не дешава.
updates-out-busy = Изашао је Hoard { $version }; чека се да се ништа не дешава ({ $work }).
updates-busy-more = још { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } посао чека
        [few] { $count } посла чекају
       *[other] { $count } послова чека
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } отпремање
        [few] { $count } отпремања
       *[other] { $count } отпремања
    }
updates-list-separator = ,{ " " }
updates-what = Кад изађе ново издање, Hoard може да позове адресу чим се ништа не дешава (нема отпремања, послова, претварања ни одржавања), тако да оно што ажурира његов контејнер не ресетује Hoard усред отпремања. Шаље POST са издањем у JSON телу. За ово је потребна провера издања изнад.
updates-url = Адреса за позивање
updates-url-placeholder = Нема (нпр. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Сачувано (куцајте да замените)
updates-auth-placeholder = Нема (нпр. Bearer my-token)
updates-remove = Уклони
updates-save = Сачувај
updates-call-now-title = Покреће се оно на шта је повезана
updates-call-now = Позови сада
updates-what-calls = Шта може да позове?
updates-watchtower = HTTP API Watchtower-а
updates-watchtower-at = :{ " " }
updates-watchtower-with = , са{ " " }
updates-watchtower-token = { " " }и токеном из{ " " }
updates-watchtower-its = { " " }(са укљученом{ " " }
updates-watchtower-endpoint = { " " }крајњом тачком).
updates-portainer = Portainer вебхук
updates-portainer-rest = { " " }на Hoard-овом стеку или контејнеру, с укљученим поновним повлачењем слике. Токен је у URL-у.
updates-script = Скрипта на хосту
updates-script-listener = { " " }иза вебхук слушаоца (као што је{ " " }
updates-script-runs = ) која покреће{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }или било шта друго што прима вебхук.
updates-poll-before = Или прескочите адресу: скрипта с приступним токеном може периодично да проверава{ " " }
updates-poll-until = { " " }док{ " " }
updates-poll-and = { " " }не буде постављено и{ " " }
updates-poll-after = { " " }не буде true, а затим да повуче нову верзију и поново покрене Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Није могуће учитати Одржавање
maintenance-reanalyze-title = Поново анализирати сваки пакет?
maintenance-reanalyze-body = Свака датотека се поново чита, што на великој библиотеци може дуго да траје; пакети су приказани као у обради док не дођу на ред. Ваши називи, ознаке и белешке остају.
maintenance-reanalyze-confirm = Поново анализирај
maintenance-gc-title = Обрисати некоришћене датотеке?
maintenance-gc-body = Сачуване датотеке на које ништа у библиотеци не упућује трајно се бришу. Пакети у архиви задржавају своје.
maintenance-gc-confirm = Обриши их
maintenance-start-failed = Није могуће покренути „{ $task }“
maintenance-just-now = малопре
maintenance-minutes-ago = пре { $count } мин
maintenance-hours-ago = пре { $count } ч
maintenance-took-seconds = { $count } с
maintenance-took-minutes = { $count } мин
maintenance-took-hours = { $hours } ч
maintenance-loading = Учитавање…
maintenance-running = Покреће се…
maintenance-starting = Покретање…
maintenance-run = Покрени
maintenance-started = { $detail } · покренуто { $when }
maintenance-starting-detail = Покретање
maintenance-finished = Завршено:{ " " }
maintenance-failed = Није успело:{ " " }
maintenance-never-run = Никад покренуто
maintenance-history = Претходна покретања
maintenance-col-task = Задатак
maintenance-col-finished = Завршено
maintenance-col-took = Трајало
maintenance-col-result = Резултат
maintenance-script-before = Резервне копије и провера оштећења могу да се покрену и из скрипте:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }и{ " " }
maintenance-script-after = . И та покретања се приказују овде.

## SettingsPage.vue

settings-title = Подешавања
settings-loading = Учитавање…
settings-load-failed = Није могуће учитати подешавања: { $error }
settings-page-general = Опште
settings-page-general-blurb = Како се овај сервер зове, за све који га користе.
settings-page-general-blurb-desktop = Како се ова библиотека зове.
settings-page-appearance = Изглед
settings-page-appearance-blurb = Теме које овај прегледач носи кад је светао и кад је таман. Чува се само у овом прегледачу.
settings-page-library = Библиотека
settings-page-library-blurb = Шта је у збирци: колико је велика, од чега је, како је расла и колико је добро означена. Архивиране ствари се не рачунају.
settings-page-tags = Ознаке
settings-page-tags-blurb = Свака ознака у библиотеци. Спојите оне које значе исто или преименујте неку; означивачи од тада користе нови назив.
settings-page-store-files = Датотеке продавница
settings-page-store-files-blurb = Датотеке које продавнице стављају уз своје радове, а које Hoard чита да би дао назив пакетима и навео њихове ауторе. Научите га оне које не познаје и поделите оно што сте га научили.
settings-page-locations = Локације библиотеке
settings-page-locations-blurb = Фасцикле на серверу које садрже библиотеку: оне у којима Hoard чува датотеке и фасцикле с пакетима које индексира тамо где јесу.
settings-page-locations-blurb-desktop = Фасцикле које садрже библиотеку: оне у којима Hoard чува датотеке и фасцикле с пакетима које индексира тамо где јесу.
settings-page-tools = Алати
settings-page-tools-blurb = Спољни програми које Hoard користи за претварање и читање датотека.
settings-page-ai = Означавање помоћу ВИ
settings-page-ai-blurb = ВИ модел даје назив, описује и означава нове пакете, поврх уграђених правила. Његове ознаке су означене као предложене.
settings-page-search = Претрага слика и звукова
settings-page-search-blurb = Претражујте слике по ономе што приказују и звукове по томе како звуче, а не само по називима, и проналазите ствари сличне по врсти, а не само по боји.
settings-page-adding = Додавање пакета
settings-page-adding-blurb = Шта Hoard ради с пакетом кад се дода, осим што га чита и означава.
settings-page-downloads = Преузимања
settings-page-downloads-blurb = Како се датотеке зову у ономе што Hoard шаље: преузимања, zip архиве, алат hoard и ВИ асистенти. Датотеке у библиотеци остају какве јесу.
settings-page-account = Налог
settings-page-account-blurb = Ваша лозинка, остали људи који могу да се пријаве и приступни токени за скрипте и друге програме.
settings-page-auth = Аутентификација
settings-page-auth-blurb = Како се људи пријављују: Hoard-ови сопствени налози, LDAP директоријум, Microsoft Active Directory или OAuth добављач, колико трају сесије и које групе добијају коју улогу.
settings-page-connection = Веза
settings-page-connection-blurb = Како прегледачи и апликације приступају Hoard-у: адреса по којој је познат и HTTPS, који је потребан неким функцијама прегледача.
settings-page-performance = Перформансе
settings-page-performance-blurb = Колико се сервер напреже, како се праве и чувају сличице и колико највише прима.
settings-page-performance-blurb-desktop = Колико се Hoard напреже, како се праве и чувају сличице и колико највише прима.
settings-page-issues = Проблеми
settings-page-issues-blurb = Шта је пошло наопако при отварању или обради датотека: архиве које нису хтеле да се распакују, датотеке које нису могле да се прочитају, послови који нису успели. Покушајте поново кад отклоните узрок или их одбаците.
settings-page-maintenance = Одржавање
settings-page-maintenance-blurb = Резервне копије, провера оштећења и чишћење. Истовремено ради по један задатак сваке врсте, а ову страницу можете напустити док раде.
settings-the-default = подразумевано
settings-default-on = укључено
settings-default-off = искључено
settings-default-auto = аутоматски
settings-option-saved = { $name }: { $value }
settings-back-to-default = Врати на подразумевано
settings-save = Сачувај
settings-use = Користи
settings-status-off = Искључено

## SettingsPage.vue: Изглед

settings-themes = Теме
settings-look-light = Светла
settings-look-dark = Тамна
settings-look-group =
    { $scheme ->
        [light] Светла тема
       *[dark] Тамна тема
    }
settings-preview =
    Приказује се { $preview ->
        [light] светла
       *[dark] тамна
    } тема да бисте је видели. Користите { $used ->
        [light] светлу
       *[dark] тамну
    }{ $system ->
        [yes] { " " }(према систему)
       *[other] { "" }
    }, а она се враћа кад напустите ову страницу.
settings-preview-back =
    Назад на { $used ->
        [light] светлу
       *[dark] тамну
    }
settings-look-hint = Пребацујте између светле и тамне (или пратите систем) из горње траке.

## SettingsPage.vue: Алати

settings-blender-what = У позадини претвара моделе .blend, .dae, .abc и .x3d у glTF, тако да добију сличице и 3D преглед. Пакети додати док је био искључен претварају се чим буде спреман.
settings-blender-download = Користи инсталирани Blender или преузима Blender 4.5 LTS (око 400 MB) са огледала blender.org.
settings-ffmpeg-what = Кодира MP3, OGG и Opus преузимања и чита звук који Hoard не може сам да декодира (AAC, M4A, WMA, …).
settings-ffmpeg-download = Користи системски ffmpeg с одговарајућим кодерима или преузима статичку верзију (око 80 MB).
settings-tool-reset = { $tool }: врати на подразумевано
settings-tool-no-download = Користи инсталирани { $tool }. За овај рачунар нема верзије за преузимање, зато га инсталирајте или наведите његову путању.
settings-tool-path = Путања програма { $tool }
settings-mode-off = Искључено
settings-mode-auto = Аутоматски
settings-mode-path = Овај програм
settings-tool-looking = Тражење…
settings-tool-downloading = Преузимање… напредак је у индикатору активности
settings-tool-ready = Спремно: { $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = Инструменти којима се MIDI датотеке пуштају и претварају.
settings-soundfont-file = SoundFont датотека
settings-soundfont-placeholder = Пронађен на серверу или уграђен
settings-soundfont-placeholder-desktop = Пронађен на овом рачунару или уграђен
settings-soundfont-default = пронађен на серверу или уграђен
settings-soundfont-default-desktop = пронађен на овом рачунару или уграђен

## SettingsPage.vue: Претрага слика

settings-clip-heading = Претрага по ономе што слике приказују
settings-clip-reset = Претрага слика: врати на подразумевано
settings-clip-saved-on = Претрага слика: укључена
settings-clip-saved-off = Претрага слика: искључена
settings-clip-what = Унесите шта тражите („mossy rock“, „sci-fi door“, „wooden barrel“) и слике које то приказују појавиће се поред назива који садрже те речи. „Још сличних“ проналази ствари исте врсте, а не само истих боја.
settings-clip-toggle = Укључи претрагу слика
settings-clip-hint = Користи OpenAI-јев модел CLIP, који ради на процесору овог сервера: ништа не напушта сервер. Укључивање преузима модел ({ $size } MB) са Hugging Face-а, а затим у позадини, ниским приоритетом, учи сваку слику. Заузима 300 до 600 MB меморије док ради или док неко претражује. Искључивање брише модел и чува оно што је научио, за следећи пут.
settings-clip-hint-desktop = Користи OpenAI-јев модел CLIP, који ради на процесору овог рачунара: ништа не напушта рачунар. Укључивање преузима модел ({ $size } MB) са Hugging Face-а, а затим у позадини, ниским приоритетом, учи сваку слику. Заузима 300 до 600 MB меморије док ради или док претражујете. Искључивање брише модел и чува оно што је научио, за следећи пут.
settings-clip-looking = Тражење модела…
settings-clip-downloading = Преузимање модела… напредак је у индикатору активности
settings-clip-ready-empty = Спремно. Слике се уче како се пакети додају.
settings-clip-ready-all =
    { $count ->
        [one] Спремно: познаје { $count } слику
        [few] Спремно: познаје све { $count } слике
       *[other] Спремно: познаје свих { $count } слика
    }
settings-clip-learning = Спремно: учи слике, до сада { $done } од { $total }
settings-clip-unavailable = Није могуће добити модел: { $detail }. Покушава се поново на сваких сат времена.
settings-clap-heading = Претрага по томе како звуци звуче
settings-clap-reset = Претрага звукова: врати на подразумевано
settings-clap-saved-on = Претрага звукова: укључена
settings-clap-saved-off = Претрага звукова: искључена
settings-clap-what = Унесите шта тражите („distant thunder“, „glassy UI blip“, „footsteps on gravel“) и звуци који тако звуче појавиће се поред назива који садрже те речи.
settings-clap-toggle = Укључи претрагу звукова
settings-clap-hint = Користи LAION-ов модел CLAP, који ради на процесору овог сервера: ништа не напушта сервер. Укључивање преузима модел ({ $size } MB) са Hugging Face-а, а затим у позадини, ниским приоритетом, преслушава почетак сваког звука. Заузима до око 1 GB меморије док ради или док неко претражује. Искључивање брише модел и чува оно што је научио, за следећи пут.
settings-clap-hint-desktop = Користи LAION-ов модел CLAP, који ради на процесору овог рачунара: ништа не напушта рачунар. Укључивање преузима модел ({ $size } MB) са Hugging Face-а, а затим у позадини, ниским приоритетом, преслушава почетак сваког звука. Заузима до око 1 GB меморије док ради или док претражујете. Искључивање брише модел и чува оно што је научио, за следећи пут.
settings-clap-ready-empty = Спремно. Звуци се уче како се пакети додају.
settings-clap-ready-all =
    { $count ->
        [one] Спремно: познаје { $count } звук
        [few] Спремно: познаје сва { $count } звука
       *[other] Спремно: познаје свих { $count } звукова
    }
settings-clap-learning = Спремно: учи звукове, до сада { $done } од { $total }

## SettingsPage.vue: Означавање помоћу ВИ

settings-ai-service = Услуга
settings-ai-off = Искључено
settings-ai-off-hint = Пакети се означавају само уграђеним правилима.
settings-ai-local = Локални модел
settings-ai-local-hint = Услуга llm у Hoard-овој compose датотеци (или cargo xtask llm). Преузмите модел испод; ради на овом рачунару и ништа не напушта рачунар.
settings-ai-chatgpt = ChatGPT план
settings-ai-chatgpt-hint = Користи ваш ChatGPT Go, Plus, Pro или Business план уместо API кључа.
settings-ai-openrouter-hint = Један кључ за стотине хостованих модела, од којих су неки бесплатни. Модели који примају слике су означени.
settings-ai-openai-hint = Хостује OpenAI.
settings-ai-anthropic-hint = Хостује Anthropic, преко своје крајње тачке компатибилне са OpenAI-јем.
settings-ai-gemini-hint = Хостује Google, преко своје крајње тачке компатибилне са OpenAI-јем.
settings-ai-ollama-hint = Ollama сервер који сами покрећете. Промените адресу ако то није контејнер под именом ollama.
settings-ai-custom = Друго (компатибилно са OpenAI-јем)
settings-ai-custom-hint = Било шта што говори OpenAI chat API: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Повезивање…
settings-ai-connected =
    { $count ->
        [one] Повезано · { $count } модел
        [few] Повезано · { $count } модела
       *[other] Повезано · { $count } модела
    }
settings-ai-connected-none = Повезано, али не наводи ниједан модел; унесите назив модела
settings-chatgpt-connected-none = Повезано; унесите назив модела
settings-ai-takes-images = прима слике
settings-ai-saved = Означавање помоћу ВИ: { $service }
settings-ai-saved-off = Означавање помоћу ВИ је искључено
settings-ai-reset = Означавање помоћу ВИ: врати на подразумевано
settings-ai-on = Укључено: { $service } · { $model }
settings-ai-url = Адреса
settings-ai-key = API кључ
settings-ai-key-saved = Сачуван (куцајте да замените)
settings-ai-key-paste = Налепите свој кључ
settings-ai-key-optional = Нема (опционо)
settings-ai-key-remove = Уклони
settings-ai-get-key = Набави кључ
settings-ai-model = Модел
settings-ai-model-pick = Изаберите или унесите модел
settings-ai-model-name = Назив модела
settings-ai-refresh = Освежи
settings-ai-connect = Повежи
settings-ai-need-key = Додајте API кључ да бисте видели његове моделе
settings-ai-vision = Шаљи и сличице (потребан је модел који прима слике)
settings-ai-batch = По захтеву
settings-ai-batch-unit = ресурса (мање за мале локалне моделе, више за велике хостоване)

## SettingsPage.vue: ChatGPT пријава

settings-chatgpt-account = Налог
settings-chatgpt-checking = Провера…
settings-chatgpt-status-as = Пријављени сте као { $email }
settings-chatgpt-status = Пријављени сте
settings-chatgpt-signed-in-as = Пријављени сте на ChatGPT као { $email }
settings-chatgpt-signed-in = Пријављени сте на ChatGPT
settings-chatgpt-signed-out = Одјављени сте са ChatGPT-а
settings-chatgpt-continue = Настави помоћу ChatGPT-а
settings-chatgpt-sign-out = Одјави се
settings-chatgpt-sign-out-title = Одјавити се са ChatGPT-а?
settings-chatgpt-sign-out-in-use = Означавање помоћу ВИ користи ваш ChatGPT план, па се зауставља док се поново не пријавите или не изаберете другу услугу.
settings-chatgpt-sign-out-body = Hoard заборавља пријаву и тражи од OpenAI-ја да је оконча.
settings-chatgpt-manage = Управљај потрошњом
settings-chatgpt-finish-before = Довршите пријаву у прозору који се отворио (
settings-chatgpt-finish-link = отворите га поново
settings-chatgpt-finish-middle = ). Ако се заврши на страници која не може да се учита, на адреси која почиње са{ " " }
settings-chatgpt-finish-after = , копирајте ту адресу и налепите је овде:
settings-chatgpt-paste = Адреса на којој се пријава завршила
settings-chatgpt-finish = Заврши
settings-chatgpt-usage = Захтеви за означавање рачунају се у потрошњу вашег ChatGPT плана. „Управљај потрошњом“ показује колико је Hoard потрошио и омогућава вам да то ограничите. Да бисте Hoard искључили из самог ChatGPT-а, идите на Settings → Security and login → Sign in with ChatGPT на chatgpt.com.

## SettingsPage.vue: Локални модели

settings-local-downloading-progress = Преузимање { $percent }% · { $done } од { $total }
settings-local-downloading = Преузимање…
settings-local-incomplete = Преузимање је заустављено
settings-local-failed = Прошли пут није успео да се покрене
settings-local-in-use-running = У употреби · ради
settings-local-in-use = У употреби
settings-local-starting = Покретање…
settings-local-running = Ради
settings-local-ready-images = Спреман · прима слике
settings-local-ready-text = Спреман · само текст
settings-local-delete-title = Обрисати { $name }?
settings-local-delete-in-use = Означавање помоћу ВИ користи овај модел, па се зауставља док не изаберете други. Његове датотеке се бришу са сервера модела.
settings-local-delete-downloading = Преузимање се зауставља, а оно што је до сада преузето брише се.
settings-local-delete-body = Његове датотеке се бришу са сервера модела. Касније можете поново да га преузмете.
settings-local-delete = Обриши
settings-local-deleting = Брисање…
settings-local-cancel = Откажи
settings-local-resume = Настави
settings-local-download = Преузми
settings-local-start-before = Покрените га помоћу{ " " }
settings-local-start-or = { " " }(или{ " " }
settings-local-start-developing = { " " }током развоја), а затим{ " " }
settings-local-start-retry = покушајте поново
settings-local-start-after = . Ради и сервер који покреће само један модел: наведите назив његовог модела испод.
settings-local-list = На серверу модела
settings-local-none = Још ништа није преузето. Изаберите модел испод; чува се на серверу, а можете га заменити кад год желите.
settings-local-catalog = Модели за преузимање
settings-local-size = { $size } GB · { $license }
settings-local-other = Други модел са Hugging Face-а
settings-local-other-placeholder = Други са Hugging Face-а: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Додавање пакета

settings-store-pages = Странице продавница
settings-store-pages-what = Нови пакети се у позадини траже у продавницама, тако да добију своју страницу продавнице, продавца и лиценцу.
settings-store-lookup = Аутоматски тражи странице продавница
settings-unreal = Unreal пакети
settings-unreal-reset = Unreal пакети: врати на подразумевано
settings-unreal-what = Шта радити с .uasset датотекама.
settings-conv = Правила претварања
settings-conv-what = Претвара датотеке које Hoard чува у један формат, нпр. сваки OBJ и FBX у GLB, тако да цела библиотека буде уједначена. Претворена датотека заузима место оригинала, с његовим именом, ознакама, колекцијама и везама. Датотеке у праћеним фасциклама се никад не мењају.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Укључено
settings-conv-to = Претвори у
# Heading over the file formats a rule converts.
settings-conv-from = Из
settings-conv-remove = Уклони
settings-conv-remove-label = Уклони правило { $n }
settings-conv-taken = Овај формат већ претвара друго правило
settings-conv-upright = Усправи моделе при претварању (Y нагоре, окренути ка +Z), онако како се приказују модели пакета
settings-conv-add = Додај правило
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Чувај оригинале
settings-conv-keep-days =
    { $count ->
        [one] дан
        [few] дана
       *[other] дана
    }
settings-conv-keep-what = До тада се оригинал може вратити са странице претворене датотеке. Текстуре се не копирају у претворени модел: он упућује на датотеке слика поред себе. Датотеке .mtl или .bin модела остају.
settings-conv-save = Сачувај правила
settings-conv-revert = Опозови измене
settings-conv-existing = Претвори датотеке које су већ у библиотеци
settings-conv-saved = Правила претварања су сачувана. Примењују се на пакете додате од сада.
settings-conv-converting =
    { $count ->
        [one] У позадини се претвара { $count } датотека
        [few] У позадини се претварају { $count } датотеке
       *[other] У позадини се претвара { $count } датотека
    }
settings-conv-nothing = Ништа у библиотеци не треба претварати.
settings-conv-ask-title =
    { $count ->
        [one] Претворити { $count } датотеку која је већ у библиотеци?
        [few] Претворити { $count } датотеке које су већ у библиотеци?
       *[other] Претворити { $count } датотека које су већ у библиотеци?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Све је у { $packs } пакету.
        [few] Све је у { $packs } пакета.
       *[other] Све је у { $packs } пакета.
    } Сваки оригинал се чува { $days ->
        [one] { $days } дан
        [few] { $days } дана
       *[other] { $days } дана
    } и до тада може да се врати са своје странице.
settings-conv-ask-busy =
    { $count ->
        [one] { $count } датотека је у пакету који се још додаје; претвориће се кад то буде готово.
        [few] { $count } датотеке су у пакетима који се још додају; претвориће се кад то буде готово.
       *[other] { $count } датотека је у пакетима који се још додају; претвориће се кад то буде готово.
    }
settings-conv-ask-list = Датотеке које би биле претворене
settings-conv-ask-more = …и још { $count }
settings-conv-ask-skipped = Остају какве јесу:
settings-conv-skip-watched =
    { $count ->
        [one] { $count } датотека у праћеној фасцикли
        [few] { $count } датотеке у праћеним фасциклама
       *[other] { $count } датотека у праћеним фасциклама
    }
settings-conv-skip-unity =
    { $count ->
        [one] { $count } датотека коју Unity пројекат зна по имену
        [few] { $count } датотеке које Unity пројекат зна по имену
       *[other] { $count } датотека које Unity пројекат зна по имену
    }
settings-conv-skip-used =
    { $count ->
        [one] { $count } датотека коју помиње сцена или друга датотека
        [few] { $count } датотеке које помиње сцена или друга датотека
       *[other] { $count } датотека које помиње сцена или друга датотека
    }
settings-conv-skip-beside =
    { $count ->
        [one] { $count } датотека чија је претворена копија већ у њеном пакету
        [few] { $count } датотеке чије су претворене копије већ у њиховим пакетима
       *[other] { $count } датотека чије су претворене копије већ у њиховим пакетима
    }
settings-conv-skip-blender =
    { $count ->
        [one] { $count } датотека коју Blender још није прочитао (Подешавања → Алати)
        [few] { $count } датотеке које Blender још није прочитао (Подешавања → Алати)
       *[other] { $count } датотека које Blender још није прочитао (Подешавања → Алати)
    }
settings-conv-ask-new = Сачувај, само нови пакети
settings-conv-ask-save-convert =
    { $count ->
        [one] Сачувај и претвори { $count } датотеку
        [few] Сачувај и претвори { $count } датотеке
       *[other] Сачувај и претвори { $count } датотека
    }
settings-conv-ask-convert =
    { $count ->
        [one] Претвори { $count } датотеку
        [few] Претвори { $count } датотеке
       *[other] Претвори { $count } датотека
    }
settings-unreal-off = Остави какве јесу
settings-unreal-off-hint = Претварајте са странице пакета кад пожелите.
settings-unreal-keep = Претвори, задржи оригинале
settings-unreal-keep-hint = Текстуре, звукови и материјали излазе у уобичајеним форматима; .uasset такође остаје.
settings-unreal-replace = Претвори, одбаци оригинале
settings-unreal-replace-hint = Штеди простор кад се садржај извуче. Мреже и даље чекају Unreal радника.

## SettingsPage.vue: Преузимања

settings-tn = Називи текстура
settings-tn-saved = Називи текстура су сачувани
settings-tn-reset = Називи текстура: врати на подразумевано
settings-tn-what = Мапе материјала излазе назване по материјалу и по намени сваке мапе, ма како се датотеке звале: Rock_Normal.png, а не rock_moss_xeokah0_raw_8K_Normal.png. Датотеке у библиотеци се не дирају, а мапа коју наводи модел или MTL послат уз њу задржава тај назив, тако да је модел и даље проналази.
settings-tn-toggle = Дај мапама текстура називе по материјалу
settings-tn-style = Стил
settings-tn-style-group = Стил именовања
settings-tn-descriptive = Описни
settings-tn-short = Кратки
settings-tn-custom = Ваш
settings-tn-pattern = Шаблон
settings-tn-hint-name = { " " }материјал,{ " " }
settings-tn-hint-map = { " " }намена мапе,{ " " }
settings-tn-hint-size = { " " }њена величина кад сет долази у више величина (додаје се на крај ако је шаблон изостави).
settings-tn-example = Пример
settings-tn-suffixes = Суфикси за сваку врсту мапе
settings-tn-slots = У GLB преузимањима, текстура упакована у модел без сопствене датотеке добија назив по материјалу и слоту који попуњава, а и ти слотови имају суфиксе:
settings-tn-endings = Ваши завршеци
settings-tn-endings-what = Текстура која није једна од мапа материјала задржава свој назив, осим ако се завршава једним од ових: _diff → _BC шаље rock_diff.png као rock_BC.png. Оставите друго поље празно да бисте уклонили завршетак.
settings-tn-ending-from = Завршетак
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Замени са
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Додај завршетак
settings-tn-ending-remove = Уклони
settings-tn-ending-remove-label = Уклони { $ending }

## SettingsPage.vue: Опште и Веза

settings-name = Назив
settings-name-what = Како се овај сервер зове у горњој траци, на картици прегледача и на страници за пријаву, да бисте, ако имате више Hoard сервера, знали који је који.
settings-name-what-desktop = Како се ова библиотека зове у горњој траци.
settings-address = Адреса
settings-address-what = Адреса коју људи и апликације користе да приступе Hoard-у, кад није она на којој је овај прегледач: адреса обрнутог проксија, тунела или назив у tailnet-у, нпр. https://hoard.example.com. Користе је везе које Hoard даје (Claude-у и другим апликацијама, при пријави, на свом сертификату).
settings-public-url = Јавна адреса
settings-public-url-placeholder = Из адресе прегледача (нпр. https://hoard.example.com)
settings-public-url-default = из адресе прегледача

## ProfilerPanel.vue: профилер на врху Подешавања → Перформансе

profiler = Профилер
profiler-what = Шта је Hoard радио у последњих десет минута, освежено на сваке две секунде док гледате. Чува се само у меморији и почиње изнова кад се Hoard поново покрене.
profiler-starting = Прва мерења стижу за тренутак.
profiler-failed = Није могуће прочитати профилер: { $error }
profiler-cpu = Коришћење процесора
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Цела машина
profiler-cpu-summary = Hoard користи { $hoard } процесора, а цела машина { $machine }.
profiler-memory = Меморија
profiler-serving = Захтеви у току
profiler-working = Ради на
profiler-working-none = Ништа није радило у позадини.
profiler-working-more =
    { $count ->
        [one] И још { $count }.
        [few] И још { $count }.
       *[other] И још { $count }.
    }
profiler-requests = Време опслуживања захтева
profiler-requests-what = Свака трака је десет секунди захтева, а њихово време је подељено према томе где је отишло.
profiler-part-db = База података
profiler-part-db-title = Чекање на базу података и извршавање њених упита
profiler-part-disk = Диск
profiler-part-disk-title = Читање сачуваних датотека и компресовање онога што се шаље
profiler-part-send = Слање
profiler-part-send-title = Чекање док мрежа преузме оно што је послато
profiler-part-other = Остало
profiler-part-other-title = Припрема одговора
profiler-request-count =
    { $count ->
        [one] { $count } захтев
        [few] { $count } захтева
       *[other] { $count } захтева
    }
profiler-requests-none = Нема захтева у последњих десет минута.
profiler-slowest = Најспорији: { $route }, { $time }
profiler-routes = По рути
profiler-route = Рута
profiler-count = Захтеви
profiler-average = Просек
profiler-p95 = Најспоријих 5%
profiler-split = Где је отишло време
profiler-sent = Послато
profiler-portal = сопствене датотеке интерфејса

## SettingsPage.vue: Перформансе

settings-render = 3D сличице
settings-render-toggle = Исцртавај сличице за 3D моделе
settings-gpu-on = На GPU-у: { $renderer }
settings-gpu-software = GPU није пронађен, па софтверски ({ $renderer })
settings-gpu-none = Није пронађен ни GPU ни софтверски рендерер, па модели не добијају сличице
settings-thumbs = Сличице
settings-thumbs-what = Како се чувају нове сличице и прикази модела. Оне с провидношћу и пиксел-арт су увек PNG. Да бисте смањили постојеће, покрените „Смањи сличице“ у Одржавању.
settings-previews-size = Прегледи сада заузимају { $size }.
settings-thumbs-saved = Сличице: { $quality }
settings-thumbs-small = Мање
settings-thumbs-small-hint = JPEG, квалитет 70. Добро у мрежи; мало мутно изблиза.
settings-thumbs-balanced = Уравнотежено
settings-thumbs-balanced-hint = JPEG, квалитет 82. Тешко се разликује од оригинала у величини сличице.
settings-thumbs-sharp = Оштрије
settings-thumbs-sharp-hint = JPEG, квалитет 92. Отприлике двоструко веће од уравнотеженог.
settings-thumbs-lossless = Без губитака
settings-thumbs-lossless-hint = PNG, тачно онако како је направљено. Неколико пута веће.
settings-motion-thumbs = Покретне сличице за анимације
settings-motion-thumbs-what = Свака анимација добија и кратак филм у петљи, који се пушта док показујеш на њену плочицу. Сваки заузима око 40 KB, а за велику библиотеку прављење потраје. Искључивањем се бришу.
settings-limits = Рад и ограничења
settings-limits-what = Свако се чува кад напустите поље или притиснете Enter. Величине прихватају јединице: 100 GB, 512 MB, 1.5 TB; број без јединице је GB.
settings-workers = Позадински радници
settings-workers-unit = послова истовремено (увоз, означавање, претварања)
settings-max-upload = Највеће отпремање
settings-max-upload-unit = по датотеци
settings-max-expand = Највећи пакет
settings-max-expand-unit = кад се распакује
settings-conversion-cache = Претворена преузимања
settings-conversion-cache-unit-now = чува се највише, сада { $size }; најдуже некоришћена иду прва
settings-conversion-cache-unit = чува се највише; најдуже некоришћена иду прва

## SettingsPage.vue: Одржавање

settings-updates = Ажурирања
settings-version = Ово је Hoard { $version }.
settings-update-check = Проверавај GitHub на сваких неколико сати за ново издање
settings-desktop-updates = Hoard при сваком покретању тражи ново издање, преузима га у позадини и инсталира када изађете.

## Врсте ресурса (api.ts)

api-kind-texture = Текстуре
api-kind-vector = Векторска графика
api-kind-sheet = Листови спрајтова
api-kind-audio = Звук
api-kind-model = Модели
api-kind-animation = Анимације
api-kind-font = Фонтови
api-kind-shader = Шејдери
api-kind-material = Материјали
api-kind-video = Видео
api-kind-document = Документи
api-kind-data = Подаци
api-kind-other = Остало
api-kind-texture-name = текстуре
api-kind-vector-name = векторска графика
api-kind-sheet-name = листови спрајтова
api-kind-audio-name = звук
api-kind-model-name = модели
api-kind-animation-name = анимације
api-kind-font-name = фонтови
api-kind-shader-name = шејдери
api-kind-material-name = материјали
api-kind-video-name = видео
api-kind-document-name = документи
api-kind-data-name = подаци
api-kind-other-name = остало

## Обавештења, дијалози и учитавање (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Опозови
ui-today = данас
ui-yesterday = јуче
ui-days-ago =
    { $days ->
        [one] пре { $days } дан
        [few] пре { $days } дана
       *[other] пре { $days } дана
    }
ui-notifications = Обавештења
ui-dismiss = Одбаци
ui-cancel = Откажи
ui-lazy-loading = Учитавање: { $what }…
ui-lazy-failed = Није могуће учитати: { $what }.
ui-lazy-reload = Поново учитај страницу

## ResetButton.vue

reset-title = Врати на подразумевано: { $value }
reset-label = Врати на подразумевано, { $value }

## SizeInput.vue

size-invalid = Величина као 100 GB, 512 MB или 1.5 TB
size-hint = Величина као 100 GB, 512 MB или 1.5 TB; број без јединице је GB

## AuthPanel.vue (Подешавања → Аутентификација)

auth-load-failed = Није могуће учитати подешавања аутентификације: { $error }
auth-saved = Сачувано. Примењује се при следећој пријави.
auth-method-internal = Интерни
auth-method-internal-hint = Имена и лозинке које чува Hoard, додати у Подешавања → Налог.
auth-method-ldap-hint = Директоријум проверава име и лозинку унете на страници за пријаву: OpenLDAP, FreeIPA, LLDAP или неки други LDAP сервер.
auth-method-ad-hint = Налози Windows домена пријављују се својим уобичајеним именом и лозинком.
auth-method-oauth-hint = Дугме „Пријава помоћу…“ које води до Google-а, Microsoft-а, GitHub-а, Keycloak-а, Authentik-а или другог добављача и назад.
auth-session-unused = После 30 дана некоришћења
auth-session-1h = 1 сат након пријаве
auth-session-8h = 8 сати након пријаве
auth-session-day = Дан након пријаве
auth-session-week = Недељу дана након пријаве
auth-session-30d = 30 дана након пријаве
auth-session-hours =
    { $hours ->
        [one] { $hours } сат након пријаве
        [few] { $hours } сата након пријаве
       *[other] { $hours } сати након пријаве
    }
auth-role-view = Може да гледа
auth-role-edit = Може да мења
auth-role-admin = Администратор
auth-role-word-view = може да гледа
auth-role-word-edit = може да мења
auth-role-word-admin = администратор
auth-role-word-default = { $role } (подразумевана улога)
auth-tls-starttls-hint = Обичан LDAP (порт 389) који прелази на TLS пре него што се било шта пошаље.
auth-tls-ldaps-hint = TLS од почетка (порт 636).
auth-tls-none = Без заштите
auth-tls-none-hint = Лозинке иду мрежом онакве какве јесу. Само на мрежи којој верујете.
auth-test-failed = Није могуће тестирати: { $error }
auth-preset-custom = Друго (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel и други: унесите издаваоца и „Откриј“ попуњава адресе.
auth-preset-google-note = Направите OAuth клијент (Web application) у Google Cloud Console → APIs & Services → Credentials. Google не шаље групе, па улоге доделите у Подешавања → Налог и искључите прављење налога при првој пријави, осим ако треба да уђе свако ко има Google налог.
auth-preset-microsoft-note = Региструјте апликацију у Entra ID → App registrations (Web преусмеравање на адресу испод). Уместо „organizations“ ставите ID свог закупца (tenant) да бисте пустили само своју организацију. За улоге из група додајте groups claim под Token configuration: он наводи ID-јеве група.
auth-preset-github-note = Направите OAuth апликацију у GitHub → Settings → Developer settings. GitHub не шаље групе, па улоге доделите у Подешавања → Налог и искључите прављење налога при првој пријави, осим ако треба да уђе свако ко има GitHub налог.
auth-preset-gitlab-note = Направите апликацију у GitLab → Preferences → Applications (или у администраторском делу своје инстанце: промените gitlab.com у адресама).
auth-discovered = Адресе добављача су попуњене.
auth-copied = Копирано
auth-copy-failed = Копирање није успело: изаберите текст и сами га копирајте
auth-sign-in-with = Пријава помоћу
auth-own-accounts-still-work = Hoard-ови сопствени налози и даље се пријављују својим лозинкама, тако да вас директоријум или добављач који не ради или је погрешно подешен не може закључати напољу.
auth-sessions = Сесије
auth-sign-out-after = Одјављуј људе
auth-ldap-server = LDAP сервер
auth-server-type = Врста сервера
auth-server-type-custom = Прилагођено
auth-servers = Сервери
auth-server-n = Сервер { $n }
auth-remove-server-n = Уклони сервер { $n }
auth-remove = Уклони
auth-add-server = Додај сервер
auth-servers-hint = Кад један није доступан, покушава се са следећим.
auth-security = Безбедност
auth-connection-security = Заштита везе
auth-skip-verify = Не проверавај сертификат сервера (само за испробавање)
auth-ca = Сертификационо тело
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (празно: она којима систем верује)
auth-lookup = Тражење људи
auth-anonymously = Анонимно
auth-reader-account = Налог за читање
auth-reader-password = Његова лозинка
auth-saved-type-to-replace = Сачувано: куцајте да замените
auth-people-under = Људи су под
auth-name-attribute = Атрибут имена
auth-only-matching = Само уноси који одговарају
auth-groups-under = Групе су под
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (празно: само memberOf)
auth-their-groups = Њихове групе
auth-group-filter-hint = { "{" }dn{ "}" } је унос те особе, { "{" }user{ "}" } име којим се пријавила.
auth-test = Тест
auth-test-hint = Приступа сваком серверу и пријављује се као налог за читање. С именом и лозинком пријављује се и као та особа и показује шта би Hoard из тога закључио. Ништа се не чува.
auth-test-user = Неко као ко ће се пријавити
auth-test-user-placeholder = Неко као ко ће се пријавити (опционо)
auth-test-password = Његова лозинка
auth-testing = Тестирање…
auth-test-connectivity = Тестирај везу
auth-test-server-ok = доступан, а налог за читање се пријавио
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = : пријава је успела (
auth-test-signed-in-after = ).
auth-test-role = Улога би била: { $role }.
auth-test-no-role = Није ни у једној групи наведеној под Улогама, па би био одбијен.
auth-test-groups = Групе: { $groups }
auth-test-no-groups = Нису пронађене групе.
auth-provider = Добављач
auth-redirect = Адреса за преусмеравање
auth-copy = Копирај
auth-redirect-hint = Дајте ово добављачу кад правите Hoard-ов клијент. Долази из јавне адресе у Подешавања → Веза или из адресе коју користите.
auth-issuer = Издавалац
auth-discover = Откриј
auth-client-id = ID клијента
auth-client-secret = Тајна клијента
auth-authorization-url = Адреса за ауторизацију
auth-token-url = Адреса токена
auth-userinfo-url = Адреса података о кориснику
auth-userinfo-placeholder = (празно: ID токен каже ко је то)
auth-scopes = Опсези
auth-name-from = Име из
auth-groups-from = Групе из
auth-groups-from-placeholder = (нема)
auth-button = Дугме
auth-button-placeholder = вашег налога
auth-accounts = Налози
auth-provision = Направи Hoard налог кад се неко први пут пријави
auth-provision-on-hint = У супротном администратор прво додаје сваку особу, у Подешавања → Налог.
auth-provision-off-hint = Администратор прво додаје сваку особу, у Подешавања → Налог, и бира како се пријављује.
auth-open-to-anyone = Свако ко има { $provider } налог може да уђе. Додајте групе за гледање испод или ово искључите и сами додајте људе.
auth-new-accounts = Нови налози
auth-roles-from-groups = Улоге из група
auth-roles-hint = Групе чији чланови добијају сваку улогу, по једна у реду, по имену или DN-у. Побеђује највиша која одговара и проверава се при свакој пријави. Ако ниједна није наведена, нови налози добијају улогу изнад, а администратор је после мења. Ако су групе наведене под „Може да гледа“, пријавити се могу само чланови наведене групе.
auth-roles-hint-oauth = Групе чији чланови добијају сваку улогу, по једна у реду, по имену или DN-у (или шта год groups claim наводи). Побеђује највиша која одговара и проверава се при свакој пријави. Ако ниједна није наведена, нови налози добијају улогу изнад, а администратор је после мења. Ако су групе наведене под „Може да гледа“, пријавити се могу само чланови наведене групе.
auth-saving = Чување…
auth-save = Сачувај

## HttpsPanel.vue (Подешавања → Веза)

https-load-failed = Није могуће учитати HTTPS: { $error }
https-off-redirecting = HTTPS је искључен. Преусмеравање на http://
https-now-on = HTTPS је укључен
https-saved = Сачувано. Нове везе користе нови сертификат.
https-now-off = HTTPS је искључен
https-http3-failed-toast = HTTP/3 није могао да се покрене: { $error }
https-copied = Копирано
https-copy-failed = Копирање није успело: изаберите текст и сами га копирајте
https-mode-off = Искључено
https-mode-off-hint = Само обичан http://.
https-mode-own = Hoard-ов сопствени сертификат
https-mode-own-hint = Hoard прави сертификат за адресе овог сервера. Једном инсталирајте Hoard-ово сертификационо тело на сваки уређај и његови прегледачи ће му веровати.
https-mode-custom = Мој сертификат
https-mode-custom-hint = Онај који већ имате, нпр. од tailscale cert или Let’s Encrypt. Прегледачи му верују без инсталирања било чега.
https-untrusted-own = Овај прегледач му још не верује за { $host }: инсталирајте Hoard-ово сертификационо тело испод. Видећете упозорење које можете да прескочите.
https-untrusted-uncovered = Овај прегледач му још не верује за { $host }: сертификат не покрива ову адресу. Видећете упозорење које можете да прескочите.
https-untrusted = Овај прегледач му још не верује за { $host }. Видећете упозорење које можете да прескочите.
https-now = Сада
https-start-failed = HTTPS је подешен, али није могао да се покрене: { $error }
https-on-port = HTTPS је укључен, на порту { $port } као и http://.
https-using-it = Сада га користите.
https-open = Отвори { $url }
https-trusted = Овај прегледач му верује.
https-cert-for-before = Сертификат за{ " " }
https-cert-for-after = , важи до { $date }.
https-cert-for-after-own = , важи до { $date } (сам се обнавља).
https-not-covered-before = Не покрива{ " " }
https-not-covered-after = , адресу коју користите.
https-is-off = HTTPS је искључен: странице се служе преко обичног http://.
https-addresses = Адресе
https-addresses-hint = Сертификат покрива адресе које је Hoard пронашао и све које додате (назив који му даје рутер, Tailscale назив).
https-remove-name = Уклони { $name }
https-another-address = Друга адреса
https-another-address-placeholder = Друга адреса, нпр. nas.local
https-add = Додај
https-your-cert = Ваш сертификат
https-saved-cert-before = Сачувано:{ " " }
https-saved-cert-after = { " " }од { $issuer }, важи до { $date }. Налепите нови да бисте га заменили.
https-pem-before = Сертификат (с ланцем) и његов приватни кључ, као PEM текст: датотеке које почињу са{ " " }
https-pem-after = .
https-certificate = Сертификат
https-choose-file = Изабери датотеку…
https-private-key = Приватни кључ
https-checked-before = За{ " " }
https-checked-after = , од { $issuer }, важи до { $date }.
https-checked-after-days =
    , од { $issuer }, важи до { $date } ({ $days ->
        [one] { $days } дан
        [few] { $days } дана
       *[other] { $days } дана
    }).
https-self-signed = Самопотписан је: прегледачи ће упозоравати на њега.
https-key-matches = Кључ одговара.
https-where-to-get = Где га набавити
https-get-tailscale-before = укључите HTTPS за свој tailnet, затим покрените{ " " }
https-get-tailscale-after = { " " }на серверу и отпремите те две датотеке. Важе 90 дана: пре тога покрените команду поново и поново их отпремите.
https-get-acme-title = Let’s Encrypt или други ACME клијент
https-get-acme-before = (certbot, acme.sh, lego) за домен који поседујете: отпремите{ " " }
https-get-acme-and = { " " }и{ " " }
https-get-acme-after = .
https-get-proxy-title = Обрнути прокси
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) који има сертификат: овде оставите HTTPS искључен и нека прокси шаље Hoard-у обичан http://.
https-trying = Испробавање…
https-apply = Примени
https-apply-hint = Hoard прво испробава ново подешавање и задржава постојеће ако оно не ради.
https-trust-title = Верујте Hoard-у на својим уређајима
https-download-ca = Преузми hoard-ca.crt
https-trust-hint = Једном инсталирајте Hoard-ово сертификационо тело на сваки уређај и његови прегледачи ће веровати овом серверу (и сваком сертификату који Hoard касније направи за њега). Оно може да јемчи само за овај Hoard. Отисак (SHA-256):
https-ca-windows = Отворите датотеку → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Поново покрените прегледач.
https-ca-macos = Отворите датотеку; она иде у Keychain Access. Тамо двапут кликните на „{ $subject }“ → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone и iPad
https-ca-ios = Преузмите је у Safari-ју и дозволите. Settings → Profile Downloaded → Install. Затим Settings → General → About → Certificate Trust Settings → укључите „{ $subject }“.
https-ca-android = Settings → Security → More security settings → Encryption & credentials → Install a certificate → CA certificate и изаберите датотеку. (Називи се помало разликују од телефона до телефона; у Settings потражите „certificate“.)
https-ca-linux-1 = Debian и Ubuntu:{ " " }
https-ca-linux-2 = . Fedora и Arch:{ " " }
https-ca-linux-3 = . Chrome на Linux-у има сопствену листу: Settings → Privacy and security → Security → Manage certificates → Import.
https-ca-firefox = Firefox има сопствену листу: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import и означите „Trust this CA to identify websites“.
https-ca-cli-title = Алат командне линије hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , или ставите путању у{ " " }
https-ca-cli-after = .
https-options = Опције
https-redirect = Шаљи прегледаче на другим уређајима са http:// на https://
https-redirect-hint = Преусмеравају се само учитавања страница. Алат командне линије hoard, Unreal радник, скрипте и ВИ апликације и даље раде преко http://, као и http://localhost на самом серверу.
https-http3-hint-before = Брже на нестабилном Wi-Fi-ју и на телефонима. Потребни су HTTPS и отворен UDP порт { $port } поред TCP-а (у Docker-у:{ " " }
https-http3-hint-after = ).
https-http3-running = Ради.
https-http3-waits = Покреће се кад је HTTPS укључен.
https-http3-failed = HTTP/3 није могао да се покрене: { $error }
https-lockout-title = Ако останете закључани напољу
https-lockout-1 = { "" }
https-lockout-2 = { " " }на самом серверу увек ради преко обичног http://. Или искључите HTTPS из командне линије сервера и поново покрените Hoard:{ " " }
https-lockout-3 = { " " }(у Docker-у:{ " " }
https-lockout-4 = , затим{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Подешавања → Налог)

account-role-view = Може да гледа
account-role-view-hint = Претрага, преглед и преузимање
account-role-edit = Може да мења
account-role-edit-hint = Уз то додавање, мењање, означавање, сакупљање у колекције и архивирање
account-role-admin = Администратор
account-role-admin-hint = Уз то Подешавања, налози, одржавање и трајно брисање
account-source-internal = Hoard лозинка
account-source-ldap = Лозинка за { $directory }
account-source-oauth = Пријава помоћу { $provider }
account-source-word-directory = директоријум
account-load-failed = Није могуће учитати налоге: { $error }
account-failed = { $what }: { $error }
account-password-failed = Није могуће променити лозинку
account-password-changed = Лозинка је промењена. Ваши остали уређаји су одјављени.
account-add-failed = Није могуће додати { $name }
account-added = { $name } сада може да се пријави.
account-role-failed = Није могуће променити шта { $name } може да ради
account-remove-title = Уклонити { $name }?
account-remove-body = Одјављује се свуда и више не може да се пријави. Ништа од онога што је додато у библиотеку не уклања се.
account-remove = Уклони
account-remove-failed = Није могуће уклонити { $name }
account-token-failed = Није могуће направити токен
account-copied = Копирано
account-copy-token-failed = Копирање није успело: изаберите токен и сами га копирајте
account-copy-failed = Копирање није успело: изаберите текст и сами га копирајте
account-revoke-title = Опозвати „{ $name }“?
account-revoke-body = Све што га још користи од сада се одбија.
account-revoke = Опозови
account-revoke-failed = Није могуће опозвати „{ $name }“
account-disconnect-title = Прекинути везу са { $app }?
account-disconnect-body = Од сада се одбија и мора поново да се повеже да би користио Hoard.
account-disconnect = Прекини везу
account-disconnect-failed = Није могуће прекинути везу са { $app }
account-guest-failed = Није могуће променити то
account-never = никад
account-you = Ви
account-sign-out = Одјави се
account-signed-in-as-before = Пријављени сте као{ " " }
account-signed-in-as-admin = , улога: администратор.
account-signed-in-as-edit = , улога: може да мења библиотеку.
account-signed-in-as-view = , улога: може да гледа, али не и да мења било шта.
account-app-signin = Апликација Hoard вас сама пријављује, па лозинке нема. Испод направите приступни токен за прикључке погона и наредбу hoard.
account-password-at-provider = Вашу лозинку чува добављач преко ког се пријављујете, па је мењате тамо.
account-password-at-directory = Вашу лозинку чува директоријум преко ког се пријављујете, па је мењате тамо.
account-current-password = Тренутна лозинка
account-new-password = Нова лозинка
account-new-password-again = Поновите нову лозинку
account-change-password = Промени лозинку
account-password-mismatch = Нове лозинке се не подударају.
account-password-rule = Најмање 8 знакова.
account-people = Људи
account-people-hint = Свака особа може да гледа (претрага, преглед и преузимање), уз то и да мења библиотеку (додавање, уређивање, означавање, сакупљање у колекције и архивирање) или да буде и администратор (Подешавања, налози, одржавање и трајно брисање).
account-you-tag = (ви)
account-signs-in-oauth = Пријављује се преко OAuth добављача
account-signs-in-directory = Пријављује се преко директоријума
account-since = од { $date }
account-what-can-do = Шта { $name } може да ради
account-name = Име
account-how-sign-in = Како се пријављује
account-password-label = Лозинка (најмање 8 знакова)
account-password-placeholder = Лозинка (8+ знакова)
account-what-they-can-do = Шта може да ради
account-add = Додај
account-ai-apps = Claude и друге ВИ апликације
account-ai-apps-hint = Повежите Claude (или било коју MCP апликацију) и он може да претражује и доноси ресурсе за вас, као и да описује и означава оно што Hoard још није. Први пут се пријављује у Hoard; ви то дозвољавате овде у прегледачу.
account-mcp-address = Hoard-ова MCP адреса
account-copy = Копирај
account-claude-code-1 = покрените{ " " }
account-claude-code-2 = , затим{ " " }
account-claude-code-3 = { " " }да бисте се пријавили.{ " " }
account-claude-code-4 = { " " }описује пакет.
account-claude-ai = Settings → Connectors → Add custom connector, налепите адресу, затим Connect. Они се повезују са Anthropic-ових сервера, тако да Hoard мора бити доступан са интернета преко HTTPS-а.
account-claude-ai-local = Settings → Connectors → Add custom connector, налепите адресу, затим Connect. Они се повезују са Anthropic-ових сервера, тако да Hoard мора бити доступан са интернета преко HTTPS-а (ова адреса није: подесите јавну адресу у Подешавања → Веза).
account-connection-meta = за { $user } од { $since } · последњи пут коришћено: { $used }
account-no-apps = Још нема повезаних апликација.
account-tokens = Приступни токени
account-tokens-hint-1 = За{ " " }
account-tokens-hint-2 = { " " }(алат командне линије), скрипте, MCP клијенте и Unreal радника: шаљу га као{ " " }
account-tokens-hint-3 = . Токен може да ради оно што изаберете, до онога што ви можете, али никад не може да управља налозима ни токенима. Токен који само гледа довољан је за претрагу, преузимање и синхронизацију пројекта.
account-tokens-admin = Видите токене и повезане апликације свих корисника; остали виде само своје.
account-made-before = { "" }
account-made-after = : копирајте га сада. Више се неће приказати.
account-new-token = Нови токен
account-cli-1 = За CLI:{ " " }
account-cli-2 = , или га ставите у{ " " }
account-cli-3 = .
account-done = Готово
account-token-meta = направљен { $made } · последњи пут коришћен: { $used }
account-token-meta-by = направљен { $made }, аутор: { $by } · последњи пут коришћен: { $used }
account-no-tokens = Још нема токена.
account-token-name = Шта ће га користити
account-token-name-placeholder = Шта ће га користити, нпр. „Unreal радник“
account-token-role = Шта може да ради
account-make-token = Направи токен
account-guests = Гости
account-guests-allow = Дозволи свакоме ко може да приступи серверу да прегледа без пријаве
account-guests-hint = Могу да претражују, прегледају и преузимају, али не и да додају, мењају или бришу било шта, нити да виде Подешавања.

## SignIn.vue

signin-create-title = Направите свој налог
signin-title = Пријава
signin-setup-lede = Овај сервер још нема ниједан налог. Налог који овде направите може да ради све; налоге за друге људе касније додајте у Подешавања → Налог.
signin-overlay-lede = Прегледање је овде отворено за све, али за измене је потребан налог.
signin-with = Пријава помоћу { $provider }
signin-or-password = или именом и лозинком
signin-username = Корисничко име
signin-name = Име
signin-new-password = Нова лозинка
signin-password = Лозинка
signin-password-rule = Најмање 8 знакова.
signin-confirm-password = Потврдите лозинку
signin-mismatch = Лозинке се не подударају.
signin-setup-code = Код за подешавање
signin-setup-code-hint-before = Код који је овај сервер добио у{ " " }
signin-setup-code-hint-after = .
signin-creating = Прављење…
signin-signing-in = Пријављивање…
signin-create = Направи налог
signin-submit = Пријави се
signin-keep-browsing = Настави прегледање
signin-directory-hint = Користите своје { $directory } име и лозинку.
signin-forgot-1 = Заборавили сте Hoard лозинку? На серверу покрените{ " " }
signin-forgot-2 = { " " }(у Docker-у{ " " }
signin-forgot-3 = ) да бисте добили нову.

## SetupWizard.vue (прва посета новом серверу)

setup-title = Где ће бити ваша библиотека?
setup-lede = Изаберите фасциклу на серверу у којој ће Hoard чувати ваше пакете или га усмерите на фасциклу с пакетима коју већ имате. Касније можете да додате још локација, на другим дисковима, у Подешавањима.
setup-lede-desktop = Изаберите фасциклу у којој ће Hoard чувати ваше пакете или га усмерите на фасциклу с пакетима коју већ имате. Касније можете да додате још локација, на другим дисковима, у Подешавањима.
setup-unreachable = Није могуће приступити серверу: { $error }
setup-unreachable-desktop = Hoard не одговара: { $error }
setup-try-again = Покушај поново
setup-loading = Учитавање…
setup-start = Покрени библиотеку
setup-ready = Ваша библиотека је спремна
setup-managed-before = Нови пакети иду у{ " " }
setup-managed-after = . Превуците датотеке, фасцикле или архиве било где на страницу да бисте додали прве.
setup-watched-before = Hoard прати{ " " }
setup-watched-after = ; пакети се појављују како их пронађе. Да бисте могли и да отпремате, додајте библиотеку коју Hoard чува у Подешавања → Локације библиотеке.
setup-open = Отвори библиотеку

## ConnectApp.vue (апликација као што је Claude тражи пријаву)

connect-title = Повезати { $app }?
connect-lede-1 = { " " }ће моћи да претражује, преузима, описује и означава све у овом Hoard-у, у име корисника{ " " }
connect-lede-2 = . Не може да управља налозима ни приступним токенима.
connect-back-before = Вратићете се на{ " " }
connect-back-after = . Везу можете прекинути кад год желите у Подешавања → Налог.
connect-allow = Дозволи
connect-cancel = Откажи
connect-failed = Није могуће повезати ову апликацију
connect-go-library = Иди у библиотеку
connect-checking = Провера…

## InsecureNotice.vue (ред приказан преко обичног http://)

insecure-text = Ово је обичан http://, а неколико ствари захтева HTTPS: 3D помоћу WebGPU-а, савремени привремени меморијски простор, инсталирање Hoard-а као апликације и Claude конектори.
insecure-switch = Пређи на https://
insecure-set-up = Подеси HTTPS
insecure-dismiss = Не приказуј ово поново

## FormatWizard.vue (Подешавања → Датотеке продавница → учење Hoard-а датотеци продавнице)

formatwizard-too-big = Ова датотека је превелика да би се читала као датотека продавнице.
formatwizard-not-json = Ова датотека није исправан JSON, па се приказује као текст.
formatwizard-read-failed = Није могуће прочитати датотеку: { $error }
formatwizard-back-title = Вратити се на образац?
formatwizard-back-body = Измене у TOML-у се губе: образац поново пише формат на основу онога што има.
formatwizard-back-to-form = Назад на образац
formatwizard-miss-name = Његове датотеке не прихватају назив { $file }. Додајте га у Датотеке.
formatwizard-miss-not-the-stores = Оно по чему препознаје датотеке продавнице није у овој датотеци. Промените „Препознај по“.
formatwizard-miss-no-title = Не проналази наслов у овој датотеци. Изаберите наслов.
formatwizard-miss-short-name = није његов назив
formatwizard-miss-short-not-the-stores = није од продавнице
formatwizard-miss-short-no-title = нема наслова
formatwizard-saved-packs =
    Сачувано: { $file }. { $packs ->
        [one] { $packs } пакет се поново чита.
        [few] { $packs } пакета се поново читају.
       *[other] { $packs } пакета се поново чита.
    }
formatwizard-saved = Сачувано: { $file }.
formatwizard-save-failed = Није могуће сачувати формат: { $error }
formatwizard-zip-failed = Није могуће направити zip: { $error }
formatwizard-stop-title = Престати с учењем ове датотеке?
formatwizard-stop-body = Формат није сачуван.
formatwizard-stop = Престани
formatwizard-label = Чаробњак за формате
formatwizard-changing = Мењање: { $name }
formatwizard-starting-from = Полазиште: { $name }
formatwizard-title = Научите Hoard датотеци продавнице
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }из пакета{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Уреди као TOML
formatwizard-file-reference = Датотека, ради поређења.
formatwizard-click-json = Кликните на поље „{ $field }“ у датотеци.
formatwizard-click-text = Кликните на поље „{ $field }“ у датотеци или га изаберите.
formatwizard-pick-first = Изаберите поље десно, а затим овде кликните на његову вредност.
formatwizard-loading = Учитавање…
formatwizard-the-format = Формат
formatwizard-syntax-before = Синтакса је описана у{ " " }
formatwizard-syntax-after = .
formatwizard-store = Продавница
formatwizard-store-placeholder = Назив продавнице, као Kenney
formatwizard-files = Датотеке
formatwizard-recognize = Препознај по
formatwizard-recognize-json = Кључ који имају само датотеке продавнице
formatwizard-recognize-text = Ред који имају само датотеке продавнице
formatwizard-recognize-hint = Један начин по реду; било који од њих је довољан. Потребно кад друге датотеке имају исти назив.
formatwizard-seller = Продавац
formatwizard-seller-placeholder = Кад датотека то не наводи
formatwizard-tags = Ознаке
formatwizard-tags-placeholder = За све што описује, раздвојене зарезима
formatwizard-fields = Поља
formatwizard-selector = Селектор: { $field }
formatwizard-clear = Обриши: { $field }
formatwizard-pick = Изабери
formatwizard-what-it-reads = Шта чита
formatwizard-read-title = Наслов
formatwizard-read-seller = Продавац
formatwizard-read-license = Лиценца
formatwizard-read-url = Страница продавнице
formatwizard-read-kind = Врста
formatwizard-read-tags = Ознаке
formatwizard-read-origin = Место израде
formatwizard-read-size = Величина
formatwizard-also = И { $format } чита ову датотеку. Овај формат би се испробао први.
formatwizard-library =
    Назив који прихвата има { $files ->
        [one] { $files } датотека
        [few] { $files } датотеке
       *[other] { $files } датотека
    } у { $packs ->
        [one] { $packs } пакету
        [few] { $packs } пакета
       *[other] { $packs } пакета
    }. Од испробаних ({ $tried }) чита { $read }.
formatwizard-library-none = Ниједна датотека у библиотеци нема назив који прихвата.
formatwizard-share = Поделите га
formatwizard-share-hint = Zip формата, ове датотеке као његовог примера и онога како се пример чита, распоређен као у Hoard-овом репозиторијуму, да га пошаљете и да га сви добију. Прво уклоните из примера све што не би требало делити.
formatwizard-sample = Пример за дељење
formatwizard-download = Преузми { $file }
formatwizard-close = Затвори
formatwizard-use = Користи га овде
formatwizard-cancel = Откажи
