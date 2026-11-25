## Errors del servidor

error-account-not-found = no s'ha trobat el compte
error-asset-gone = no s'ha trobat el recurs
error-asset-not-found = no s'ha trobat el recurs { $id }
error-blob-not-found = no s'ha trobat el blob { $hash }
error-collection-not-found = no s'ha trobat la col·lecció { $id }
error-connection-not-found = no s'ha trobat la connexió
error-derivative-not-found = no s'ha trobat el derivat { $kind } de { $id }
error-file-in-pack-not-found = no s'ha trobat { $path } al paquet { $id }
error-site-not-a-page = només les pàgines web s’obren com a pàgines
error-site-link-expired = l’adreça d’aquesta pàgina ha caducat; torna-la a obrir des del Hoard
error-format-not-found = no s'ha trobat el format { $file }
error-frame-sheet-aseprite-only = només els fitxers d'Aseprite tenen full de fotogrames
error-frame-sheet-frames = té un sol fotograma, o massa per disposar-los
error-location-not-found = no s'ha trobat la ubicació { $id }
error-maintenance-task-not-found = no s'ha trobat la tasca de manteniment { $id }
error-open-upload-not-found = no s'ha trobat la pujada oberta { $id }
error-original-not-found = no s'ha trobat el fitxer original de { $id }
error-pack-not-found = no s'ha trobat el paquet { $id }
error-preview-too-big = aquesta imatge és massa gran per fer-ne una previsualització lleugera
error-project-not-found = no s'ha trobat el projecte { $name }
error-saved-search-not-found = no s'ha trobat la cerca desada { $id }
error-sign-in-first = primer inicieu la sessió (o envieu un testimoni d'accés)
error-those-assets-not-found = no s'han trobat aquests recursos
error-thumbnail-not-found = no s'ha trobat la miniatura de { $id }
error-token-not-found = no s'ha trobat el testimoni
error-upload-not-found = no s'ha trobat la pujada { $id }
error-pack-needs-name = un paquet necessita un nom
error-no-library = encara no hi ha cap ubicació de la biblioteca on desar fitxers: afegiu-ne una de gestionada a Configuració → Ubicacions de la biblioteca
error-upload-failed = la pujada ha fallat: { $reason }
error-upload-no-files = la pujada no té fitxers
error-analyze-nothing-to-do = no hi ha res a fer: rules=false i l'LLM està desactivat
error-too-many-assets = massa recursos en una sola sol·licitud
error-change-not-found = aquest canvi no és a l'historial d'aquest recurs
error-bulk-by-look = una cerca per aspecte no es pot editar tota alhora: selecciona els recursos
error-too-many-items = massa elements en una sola sol·licitud
error-not-a-hash = no és un hash blake3 hexadecimal
error-merge-into-tag = into ha de ser una etiqueta
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = «{ $text }» no és una etiqueta
error-not-a-character = només un personatge pot ser on es reprodueixen les animacions del seu esquelet
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } ja és sota { $tag }, així que { $tag } no hi pot anar
error-nothing-to-merge = no hi ha res a fusionar
error-selection-size = indiqueu entre 1 i { $max } identificadors de recurs
error-collection-needs-name = una col·lecció necessita un nom
error-collection-exists = ja hi ha una col·lecció anomenada { $name }
error-collection-empty = la col·lecció { $name } és buida
error-conversion-failed = no s'ha pogut convertir a { $format }: { $reason }
error-max-size = max ha de ser una potència de dos entre 64 i 16384
error-replace-ext = ext ha de ser una extensió de fitxer curta
error-empty-file = fitxer buit
error-imitation-silent = No hi ha res a l'enregistrament que destaqui del soroll de fons. Torna-ho a provar més a prop del micròfon.
error-replace-duplicate = el fitxer convertit duplica un que ja és al paquet; el paquet s'hi ha fusionat
error-ai-address = L'adreça de la IA ha de començar per http:// o https://
error-hosted-key = Introduïu la clau d'API de { $provider }
error-hosted-address = L'adreça del servei de cerca d'imatges ha de començar per http:// o https://
error-local-llm-action = action és list, download o delete, i les dues últimes necessiten un model
error-role = Un rol és view, edit o admin
error-admin-only = només un administrador pot fer això
error-view-only = aquest compte pot mirar però no canviar res: un administrador ho pot canviar a Configuració → Compte
error-portal-only = inicieu la sessió al portal per gestionar comptes i testimonis
error-not-from-portal = aquesta sol·licitud no prové del portal de Hoard
error-account-name = Trieu un nom de fins a 64 caràcters
error-password-length = Feu servir una contrasenya d'almenys { $min } caràcters
error-too-many-codes = Massa codis incorrectes: espereu { $seconds } segons
error-already-set-up = Ja hi ha un compte: inicieu la sessió
error-too-many-tries = Massa intents incorrectes: espereu { $seconds } segons
error-directory-unavailable = El directori no ha pogut comprovar la contrasenya: { $reason }
error-password-elsewhere = La vostra contrasenya la guarda el directori o el proveïdor amb què inicieu la sessió
error-current-password = La contrasenya actual no és correcta
error-account-source = Un compte inicia la sessió amb Hoard (internal), un directori (ldap) o OAuth
error-account-exists = Ja hi ha un compte amb aquest nom
error-own-role = Un altre administrador pot canviar el vostre rol
error-remove-self = No podeu eliminar el vostre propi compte mentre hi teniu la sessió iniciada
error-token-name = Poseu al testimoni el nom del que l'utilitzarà, en 80 caràcters com a màxim
error-token-role = Un testimoni no pot fer més que el compte que el crea
error-wrong-setup-code = Aquest no és el codi de configuració donat al servidor a HOARD_SETUP_CODE
error-wrong-password = El nom i la contrasenya no coincideixen
error-folder-missing = { $path } no hi és (un disc que no està muntat?)
error-s3-path = una ubicació S3 té la forma s3://bucket
error-s3-address = l'adreça S3 ha de començar per http:// o https://
error-s3-header = una capçalera S3 addicional té la forma Name=value, i «{ $header }» no ho és
error-keychain = no s'ha pogut desar al clauer del sistema: { $reason }
error-not-built = aquesta còpia de Hoard s'ha compilat sense { $feature }
error-openai-auth-unreachable = no s'ha pogut connectar amb auth.openai.com
error-openai-unexpected = resposta inesperada d'OpenAI
error-openai-answered = OpenAI ha respost { $status }: { $reason }
error-chatgpt-id-token-not-jwt = el testimoni d'identitat no és un JWT
error-chatgpt-id-token-issuer = el testimoni d'identitat és de { $issuer }, no de { $expected }
error-chatgpt-id-token-audience = el testimoni d'identitat és per a una altra aplicació
error-chatgpt-id-token-expired = el testimoni d'identitat ha caducat
error-chatgpt-id-token-nonce = el testimoni d'identitat no pertany a aquest inici de sessió
error-chatgpt-no-state = l'adreça no té state; copieu l'adreça sencera
error-chatgpt-not-started = aquest inici de sessió no s'ha començat aquí, o fa més de 15 minuts que es va començar. Torneu-lo a començar des de Configuració.
error-chatgpt-canceled = s'ha cancel·lat l'inici de sessió
error-chatgpt-refused = ChatGPT ha rebutjat l'inici de sessió: { $reason }
error-chatgpt-no-code = l'adreça no té codi d'inici de sessió; copieu l'adreça sencera
error-chatgpt-other-registration = ChatGPT ha respost per a un altre registre d'aquest servidor; torneu a començar l'inici de sessió
error-chatgpt-other-registration-desktop = ChatGPT ha respost per a una altra còpia de Hoard; torneu a començar l'inici de sessió
error-chatgpt-not-registered = ChatGPT no ha acabat de registrar Hoard; torneu a començar l'inici de sessió
error-chatgpt-code-used = el codi d'inici de sessió ja s'ha fet servir o ha caducat; torneu a iniciar la sessió
error-chatgpt-refused-code = ChatGPT ha rebutjat l'inici de sessió ({ $code })
error-chatgpt-no-id-token = ChatGPT no ha enviat cap testimoni d'identitat
error-chatgpt-plan-not-allowed = S'ha iniciat la sessió, però no s'ha permès fer servir el pla. Torneu a iniciar la sessió i permeteu-ho, o feu servir una clau d'API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = no s'ha iniciat la sessió a ChatGPT; inicieu-la des de Configuració → Etiquetatge amb IA
error-chatgpt-refresh-later = ChatGPT no ha pogut renovar la sessió ara mateix ({ $code }); es tornarà a provar més tard
error-chatgpt-signed-out-problem = ChatGPT ha tancat la sessió d'aquest servidor ({ $code }). Torneu a iniciar la sessió per continuar etiquetant amb el vostre pla.
error-chatgpt-signed-out-problem-desktop = ChatGPT ha tancat la sessió de Hoard ({ $code }). Torneu a iniciar la sessió per continuar etiquetant amb el vostre pla.
error-chatgpt-signed-out = ChatGPT ha tancat la sessió d'aquest servidor ({ $code }); torneu a iniciar-la des de Configuració → Etiquetatge amb IA
error-chatgpt-signed-out-desktop = ChatGPT ha tancat la sessió de Hoard ({ $code }); torneu a iniciar-la des de Configuració → Etiquetatge amb IA
error-openai-api-unreachable = no s'ha pogut connectar amb api.openai.com
error-openai-broke-off = la resposta d'OpenAI s'ha interromput
error-chatgpt-kept-refusing = OpenAI ha continuat rebutjant l'inici de sessió de ChatGPT; torneu a iniciar la sessió des de Configuració → Etiquetatge amb IA
error-chatgpt-request-failed = la sol·licitud ha fallat
error-chatgpt-cut-off = la resposta s'ha tallat ({ $reason })
error-chatgpt-cut-off-no-reason = la resposta s'ha tallat (sense cap motiu indicat)
error-openai-ended-early = la resposta d'OpenAI ha acabat abans d'estar completa
error-chatgpt-usage-limit = l'ús del vostre pla de ChatGPT per a Hoard s'ha esgotat de moment. Podeu veure o canviar el límit a { $url }
error-chatgpt-not-eligible = aquest compte o espai de treball de ChatGPT no pot fer servir el pla en altres aplicacions. Feu servir una clau d'API.
error-chatgpt-usage-unavailable = ChatGPT no ha pogut comprovar l'ús del pla ara mateix; es tornarà a provar més tard
error-chatgpt-unsupported = l'ús del pla de ChatGPT no admet { $param } amb aquest model
error-chatgpt-unsupported-images = l'ús del pla de ChatGPT no admet { $param } amb aquest model; trieu un model que accepti imatges, o desactiveu l'enviament de miniatures
error-chatgpt-unsupported-part = l'ús del pla de ChatGPT no admet part de la sol·licitud amb aquest model
error-chatgpt-port = port és el port pel qual el navegador arriba a Hoard
error-chatgpt-paste-address = enganxeu l'adreça sencera, que comença per http://127.0.0.1
error-too-many-hashes = com a màxim { $max } hashes alhora
error-credits-pack = paquet { $key }
error-credits-collection = col·lecció { $key }
error-format-too-long = el format és massa llarg
error-not-started = el servidor no s'ha iniciat
error-not-started-desktop = Hoard no s'ha iniciat
error-not-format-file = no és un fitxer de format: { $name }
error-format-name = el nom necessita una lletra o una xifra de la A a la Z o del 0 al 9
error-format-miss-name = els fitxers del format no accepten { $file }
error-format-miss-store = el format no accepta { $file } per als de la botiga (vegeu require)
error-format-miss-title = el format no troba cap títol a { $file }
error-cant-download-as = els fitxers .{ $ext } no es poden baixar com a { $format }
error-only-images-resize = només es poden redimensionar imatges
error-https-no-certificate = Primer enganxeu o pugeu el certificat i la seva clau
error-https-no-cert-found = No s'ha trobat cap certificat: enganxeu el text que comença per -----BEGIN CERTIFICATE-----
error-https-no-key-found = No s'ha trobat cap clau privada: enganxeu el text que comença per -----BEGIN PRIVATE KEY-----
error-https-several-keys = Hi ha més d'una clau privada: enganxeu només la que correspon al certificat
error-https-key-password = La clau està protegida amb contrasenya: doneu-la sense (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = El certificat no és PEM vàlid
error-https-key-not-pem = La clau privada no és PEM vàlid
error-https-key-mismatch = La clau no pertany al certificat. Si heu donat una cadena, poseu primer el certificat del vostre servidor
error-https-unusable = El certificat o la clau no es poden fer servir
error-https-expired = Aquest certificat va caducar el { $date }
error-https-not-yet-valid = Aquest certificat no és vàlid fins al { $date }
error-https-unreadable = Això no és un certificat que Hoard pugui llegir
error-https-bad-name = «{ $name }» no és una adreça que pugui contenir un certificat
error-https-test-timed-out = la connexió de prova ha esgotat el temps d'espera
error-https-test-failed = la connexió de prova ha fallat
error-https-no-ca = Hoard encara no ha creat la seva autoritat de certificació
error-https-unavailable = HTTPS no està disponible en aquest servidor
error-https-certificate-mode = Un certificat va amb «El meu certificat»
error-https-test = El certificat no ha funcionat en una connexió de prova: { $reason }
error-nothing-to-retry = no hi ha res que es pugui tornar a provar
error-ai-unreachable = no s'ha pogut connectar amb { $url }
error-ai-key-refused = { $url } ha rebutjat la clau d'API ({ $status })
error-ai-answered = { $url } ha respost { $status }: { $reason }
error-ai-not-json = { $url }/models no ha respost amb JSON
error-model-server-refused = el servidor de models: { $reason }
error-no-model-server = no respon cap servidor de models a { $url }
error-not-llama-router = { $url } no és un servidor llama.cpp en mode router, així que no pot baixar models
error-model-download = no s'ha pogut baixar { $model }: { $reason }
error-model-delete = no s'ha pogut suprimir { $model }: { $reason }
error-unknown-kind = tipus de recurs desconegut { $kind }
error-watched-is-folder = una ubicació vigilada és una carpeta d'aquest servidor
error-cant-reach = no es pot accedir a { $path }: { $reason }
error-folder-full-path = indiqueu el camí complet de la carpeta
error-cant-create = no es pot crear { $path }
error-cant-write = no es pot escriure a { $path }
error-cant-read = no es pot llegir { $path }
error-overlaps-location = això se superposa amb la ubicació { $name } ({ $path })
error-overlaps-data = això se superposa amb la carpeta de dades de Hoard ({ $path })
error-overlaps-cache = això se superposa amb la carpeta de memòria cau de Hoard ({ $path })
error-not-watched = { $name } no és una carpeta vigilada
error-watched-no-kinds = una carpeta vigilada no accepta fitxers nous, així que no té tipus per canviar
error-keep-catch-all = feu que una altra ubicació sigui la general
error-location-holds-files = { $name } encara conté fitxers: primer moveu-los a les altres ubicacions (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = no es pot accedir a { $name }, així que Hoard no sap si encara conté fitxers
error-relocate-running = ja s'estan movent fitxers entre ubicacions; torneu-ho a provar quan acabi
error-watched-not-moved = { $name } és una carpeta vigilada: Hoard no en mou els fitxers
error-no-other-location = no hi ha cap altra ubicació de la biblioteca on moure els fitxers: primer afegiu-ne una
error-full-path = indiqueu un camí complet
error-pick-folder = trieu una carpeta
error-task-running = «{ $task }» ja s'està executant
error-job-cant-stop = «{ $job }» no es pot aturar un cop ha començat
error-job-not-found = aquesta tasca ja no està en espera ni en execució
error-task-cant-stop = «{ $task }» no es pot aturar a mig camí
error-task-not-running = «{ $task }» no s'està executant
error-token-account-gone = el compte d'aquest testimoni ja no existeix, així que no té preferits ni cerques
error-favorites-too-many = com a màxim 10.000 alhora
error-saved-search-name = una cerca desada necessita un nom de 80 caràcters com a màxim
error-saved-search-too-long = aquesta cerca és massa llarga per desar-la
error-hoard-seen = el tresor pot registrar com a màxim 32 tipus de recurs, cadascun amb un nom en minúscules
error-news-seen = les novetats poden registrar com a màxim 256 elements vistos, cadascun amb un nom en minúscules, xifres i guions
error-texture-pattern-map = El patró de nom de textura necessita { "{" }map{ "}" }, o tots els mapes d'un material tindrien el mateix nom
error-texture-pattern-fields = El patró de nom de textura pot fer servir { "{" }name{ "}" }, { "{" }map{ "}" } i { "{" }size{ "}" }
error-texture-name-chars = Els noms de textura no poden contenir < > : " / \ | ? ni *
error-texture-map-type = No hi ha cap tipus de mapa anomenat { $role }
server-role-base-color = color
server-role-normal = normals
server-role-normal-dx = normals (DirectX)
server-role-roughness = rugositat
server-role-gloss = brillantor
server-role-metalness = metal·licitat
server-role-ao = oclusió ambiental
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = alçada
server-role-bump = relleu
server-role-cavity = cavitat
server-role-specular = especular
server-role-opacity = opacitat
server-role-emissive = emissió
server-role-translucency = translucidesa
error-texture-suffix-missing = Doneu un sufix als mapes de tipus { $map }
error-texture-suffix-chars = El sufix dels mapes de tipus { $map } no pot contenir < > : " / \ | ? ni *
error-texture-ending-missing = Cada terminació pròpia necessita la terminació que cal buscar
error-texture-ending-chars = Les terminacions pròpies no poden contenir < > : " / \ | ? ni *
error-texture-ending-twice = { $ending } surt dues vegades a les vostres terminacions
error-oauth-no-client-id = Aquest enllaç no té client_id.
error-oauth-unknown-app = Aquesta aplicació no està registrada aquí (o ja no ho està). Torneu a començar la connexió des de l'aplicació.
error-oauth-redirect = L'aplicació ha demanat tornar a una adreça que no ha registrat.
error-oauth-response-type = Hoard només lliura codis d'inici de sessió (response_type=code).
error-oauth-pkce = L'aplicació ha de fer servir PKCE amb S256 per iniciar la sessió a Hoard.
error-light-preview-images = només les imatges tenen previsualització lleugera
error-no-light-preview = no hi ha previsualització lleugera: { $reason }
error-no-frame-sheet = no hi ha full de fotogrames: { $reason }
error-thumbnail-source = origen de miniatura desconegut: { $source }
error-thumbnail-too-big = aquesta imatge és massa gran per a una miniatura
error-thumbnail-models-only = només els models accepten una miniatura dibuixada per un navegador
error-not-a-picture = no és una imatge que Hoard pugui llegir: { $reason }
error-project-name = un projecte necessita un nom de 200 caràcters com a màxim
error-project-too-big = com a màxim un milió de recursos en un projecte
error-project-too-many-at-once = com a màxim 100.000 alhora
error-server-name = Trieu un nom de fins a { $max } caràcters, en una sola línia
error-tool-setting = { $tool }: trieu desactivat, automàtic o indiqueu un camí
error-ai-model = Trieu el model que ha de fer servir el servei d'IA
error-public-address = L'adreça pública ha de començar per http:// o https://
error-update-hook-address = L'adreça a cridar per a les actualitzacions ha de començar per http:// o https://
error-update-hook-auth = La capçalera Authorization no pot contenir salts de línia ni altres caràcters de control
error-no-soundfont = No hi ha cap SoundFont a { $path } al servidor
error-no-soundfont-desktop = No hi ha cap SoundFont a { $path }
error-soundfont-bad = { $name } no és un SoundFont amb què el MIDI pugui sonar: { $reason }
error-workers = Processos en segon pla: entre 1 i { $max }
error-ai-batch = Recursos per sol·licitud d'IA: entre 1 i 200
error-conversion-cache = Baixades convertides: almenys 100 MB
error-motion-fps = les miniatures animades van a 12, 24 o 30 fotogrames per segon
error-thumb-size = les miniatures fan 128 o 256 píxels d’amplada
error-backup-keep = Còpies de seguretat programades: conserva’n d’1 a { $max }
error-backup-copy-to = Còpies de seguretat programades: { $path } no és una carpeta al servidor (indica’n el camí complet)
error-conversion-keep-days = Regles de conversió: els originals s'han de conservar entre 1 i { $max } dies
error-conversion-target = Regles de conversió: Hoard no pot convertir fitxers a { $format }
error-conversion-no-source = Regles de conversió: cada regla necessita com a mínim un format per convertir
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Regles de conversió: els fitxers .{ $ext } no es poden convertir a { $format }
error-conversion-twice = Regles de conversió: .{ $ext } és a dues regles
error-conversion-cant-undo = Aquest fitxer no l'ha convertit cap regla, o se n'ha eliminat l'original.
error-conversion-path-taken = Ara hi ha un altre fitxer a { $path }, així que l'original no hi pot tornar.
error-limits-zero = Els límits no poden ser zero
error-settings-object = s'esperava un objecte JSON de configuració
error-bad-setting = configuració incorrecta: { $reason }
error-auth-default-role = El rol per defecte és view, edit o admin
error-auth-session-hours = Les sessions duren entre 1 hora i un any
error-auth-settings = configuració d'autenticació incorrecta: { $reason }
error-ldap-settings = configuració LDAP incorrecta: { $reason }
error-ldap-no-server = Afegiu l'adreça del servidor de directori
error-ldap-base-dn = Indiqueu on són les entrades de les persones (el DN base)
error-ldap-username-attribute = Indiqueu quin atribut conté el nom amb què les persones inicien la sessió
error-ldap-bind-dn = Indiqueu el compte que cerca les persones, o cerqueu-les de manera anònima
error-ldap-ca-cert = L'autoritat de certificació no és un certificat PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = L'adreça d'un servidor de directori és buida
error-ldap-server-form = { $server }: l'adreça d'un servidor de directori és host:port, ldap://… o ldaps://…
error-ldap-not-starttls = { $server }: això és LDAPS, no StartTLS
error-ldap-no-certificate = no hi ha cap certificat
error-ldap-password-again = Torneu a indicar la contrasenya del compte que cerca les persones: la desada només s'envia als servidors per als quals es va donar
error-ldap-unreachable = no s'ha pogut connectar amb cap servidor de directori ({ $reasons })
error-ldap-no-anonymous = el directori no permet cercar persones de manera anònima ({ $reason })
error-ldap-bind-failed = el compte que cerca les persones no ha pogut iniciar la sessió ({ $reason })
error-ldap-search = cercant { $name } a { $base }
error-ldap-group-search = cercant els grups de { $name } a { $base }
error-ldap-several = { $count } entrades del directori coincideixen amb { $name }: restringiu el filtre d'usuaris
error-ldap-password-check = el directori no ha volgut comprovar la contrasenya de { $name } ({ $reason })
error-ldap-wrong-password = El directori no coneix { $name } amb aquesta contrasenya
error-oauth-client-id = Indiqueu l'ID de client que el proveïdor ha creat per a Hoard
error-oauth-addresses = Les adreces d'autorització i de testimoni han de començar per https://
error-oauth-userinfo-address = L'adreça d'informació d'usuari ha de començar per https://
error-oauth-username-claim = Indiqueu quina claim conté el nom de la persona
error-oauth-secret-again = Torneu a indicar el secret de client: el desat només s'envia a l'adreça de testimoni per a la qual es va donar
error-oauth-issuer = L'emissor ha de començar per https://
error-oauth-discovery-unreachable = No s'ha pogut connectar amb { $url }: { $reason }
error-oauth-discovery-status = { $url } ha respost { $status }
error-oauth-discovery-bad = { $url } no és un document de descobriment
error-oauth-discovery-incomplete = { $url } no indica les adreces d'autorització i de testimoni
error-signin-no-group = { $name } no és a cap grup que pugui fer servir Hoard
error-signin-other-source = Ja hi ha un compte de Hoard anomenat { $name } que inicia la sessió d'una altra manera
error-signin-other-subject = Ja hi ha un compte de Hoard anomenat { $name } per a una altra persona del proveïdor
error-signin-no-account = { $name } encara no té compte de Hoard: un administrador el pot afegir a Configuració → Compte
error-signin-bad-name = Aquest nom no pot ser un compte de Hoard (fins a 64 caràcters)
error-oauth-off = L'inici de sessió amb OAuth no està activat
error-oauth-bad-authorization-address = l'adreça d'autorització no és vàlida
error-oauth-refused = El proveïdor no us ha iniciat la sessió: { $reason }
error-oauth-no-code = El proveïdor us ha retornat sense iniciar la sessió
error-oauth-expired = L'inici de sessió ha trigat massa o es va començar en un altre lloc: torneu-ho a provar
error-oauth-no-claim = El proveïdor no ha dit qui sou (no hi ha la claim { $claim })
error-oauth-no-sub = El proveïdor no ha dit qui sou (no hi ha cap claim sub ni id)
error-oauth-token-unreachable = no s'ha pogut connectar amb l'adreça de testimoni del proveïdor
error-oauth-token-refused = El proveïdor no ha volgut acabar l'inici de sessió: { $reason }
error-oauth-no-access-token = el proveïdor no ha enviat cap testimoni d'accés
error-oauth-userinfo-unreachable = no s'ha pogut connectar amb l'adreça d'informació d'usuari del proveïdor
error-oauth-userinfo-refused = El proveïdor no ha volgut dir qui ha iniciat la sessió ({ $status })
error-oauth-userinfo-not-json = la informació d'usuari del proveïdor no és JSON
error-format-no-name = name és buit
error-format-no-files = files és buit: indiqueu quins fitxers llegeix el format, com ara ["*.json"]
error-format-empty-selector = buit
error-format-unopened-bracket = un claudàtor de tancament sense el d'obertura
error-format-unclosed-bracket = un claudàtor no està tancat
error-format-empty-step-dots = un pas buit (dos punts seguits, o un punt en un extrem)
error-format-unclosed-except = !( no està tancat
error-format-unclosed-filter = [ no està tancat
error-format-after-filters = { $rest } després de la clau i els filtres de { $step }
error-format-except-needs-glob = !(…) va després d'una clau amb *
error-format-except-not-glob = !(…) va després d'una clau amb *, no de { $key }
error-format-empty-step = un pas buit
error-format-not-a-filter = [{ $filter }] no és un filtre: feu servir [], [key=value|other], [array], [string], [number], [bool] o [object]
error-format-filter-needs-value = [{ $filter }] necessita una clau i un valor
error-store-search-empty = no hi ha res a cercar; escriviu el nom del paquet
error-update-hook-not-set = primer definiu una adreça a cridar
error-upload-stalled = la pujada de { $path } s'ha encallat; torneu a enviar aquesta part
error-upload-busy = ja s'està escrivint { $path }
error-upload-incomplete = { $count } fitxers estan incomplets, p. ex. { $files }
error-upload-no-files-listed = no hi ha fitxers
error-upload-not-part = { $path } no forma part de la pujada { $id }, o la pujada està tancada
error-bad-path = camí incorrecte
error-upload-part-offset = { $offset } no és on comença una part de { $path }
error-upload-interrupted = pujada interrompuda: { $reason }
error-upload-part-long = la part { $part } de { $path } fa més de { $len } bytes
error-upload-part-short = part { $part } de { $path }: s'han rebut { $got } bytes, se n'esperaven { $len }
error-upload-file-long = { $path } és més gran del que s'havia declarat
error-upload-file-short = { $path }: s'han rebut { $got } bytes, se n'esperaven { $len }
error-upload-complete = la pujada ja és completa
error-already-imported = ja s'ha importat com a «{ $name }»

## Text del servidor

server-store-place-readme = README
server-store-place-folders = carpetes
server-store-why-linked = Enllaçat al paquet ({ $place })
server-store-why-same-title = El mateix títol que al paquet ({ $place })
server-store-why-by = De { $author }, que s'esmenta al paquet ({ $place })
server-store-why-meshes = Més o menys tantes malles com el paquet ({ $count })
server-store-why-textures = Més o menys tantes textures com el paquet ({ $count })
server-chatgpt-revoke-failed = S'ha tancat la sessió aquí, però no s'ha pogut avisar OpenAI. Per assegurar-vos-en, desconnecteu Hoard a ChatGPT: Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Sessió iniciada a ChatGPT
server-chatgpt-signed-in = Hoard ha iniciat la sessió. L'etiquetatge amb IA ja pot fer servir el vostre pla de ChatGPT.
server-chatgpt-signed-in-as = Hoard ha iniciat la sessió com a { $email }. L'etiquetatge amb IA ja pot fer servir el vostre pla de ChatGPT.
server-chatgpt-not-finished-title = L'inici de sessió no s'ha acabat
server-chatgpt-other-computer = Si heu començat l'inici de sessió des de Hoard en un altre ordinador, copieu l'adreça d'aquesta pàgina de la barra d'adreces i enganxeu-la a la Configuració de Hoard d'allà.
server-back-to-settings = Torna a la Configuració
server-issues-pack = Paquets que no s'han pogut afegir
server-issues-pack-blurb = Suprimiu el paquet i torneu-lo a afegir, o obriu-lo per veure què conté.
server-issues-damaged = Fitxers desats que estan malmesos o han desaparegut
server-issues-damaged-blurb = Detectats per «Comprova si els fitxers estan malmesos». Restaureu-los des d'una còpia de seguretat o torneu a afegir el seu paquet, i després torneu a executar «Comprova si els fitxers estan malmesos».
server-issues-folder = Carpetes vigilades que no es poden llegir
server-issues-folder-blurb = Els seus paquets es deixen tal com estan fins que la carpeta es pugui tornar a llegir.
server-issues-missing = Paquets vigilats la carpeta dels quals ha desaparegut
server-issues-missing-blurb = Cada paquet es conserva 3 dies per si la carpeta només està fora de línia, i després s'elimina.
server-issues-job = Tasques en segon pla que han fallat
server-issues-job-blurb = En tornar-ho a provar, la tasca s'executa de nou des del principi.
server-issues-file = Fitxers que no s'han pogut obrir
server-issues-file-blurb = Hoard no els ha pogut obrir ni desar. Un paquet vigilat els torna a provar en el següent escaneig; un fitxer que ja era a la biblioteca conserva el que tenia abans.
server-issues-unpack = Arxius comprimits que no s'han pogut descomprimir
server-issues-unpack-blurb = Cadascun es conserva al seu paquet com a fitxer normal, així que res del que conté és a la biblioteca.
server-issues-read = Fitxers que Hoard no ha sabut interpretar
server-issues-read-blurb = Són a la biblioteca, però sense les dades, les etiquetes ni la miniatura que en dona la lectura.
server-issues-render = Models en què ha fallat el renderitzador de miniatures
server-issues-render-blurb = No tenen miniatura. Obrir-ne un al visualitzador pot funcionar igualment, i en pot desar una.
server-issues-convert = Models que Blender no ha pogut convertir
server-issues-convert-blurb = No tenen còpia GLB, així que no es poden mostrar en 3D.
server-issues-unreal = Paquets d'Unreal que no s'han pogut convertir
server-issues-unreal-blurb = La pàgina d'Unreal del paquet té la resta dels seus paquets d'Unreal.
server-issues-reformat = Fitxers que una regla de conversió no ha pogut convertir
server-issues-reformat-blurb = S'han deixat tal com eren.
server-issues-materials = Paquets dels quals no s'han pogut deduir els materials
server-issues-materials-blurb = Totes les textures hi són; simplement no estan agrupades en materials.
server-issues-material-thumb = Materials sense miniatura
server-issues-material-thumb-blurb = S'ha trobat el material, però no se n'ha pogut dibuixar la previsualització.
server-issues-store-files = Fitxers de botiga que no s'han pogut llegir
server-issues-store-files-blurb = Fitxers de la botiga d'on prové un paquet, com ara un JSON de Megascans, que li donen nom i etiquetes.
server-issues-scan-again = Torna a escanejar
server-issues-retry = Torna-ho a provar
server-issues-reanalyze = Torna a analitzar el paquet
server-issues-convert-again = Torna a convertir
server-issues-look-again = Torna a cercar
server-issues-read-again = Torna a llegir
server-issues-blob-missing = El fitxer desat ha desaparegut.
server-issues-blob-damaged = El fitxer desat no coincideix amb el que es va desar: està malmès.
server-issues-folder-gone = No s'ha trobat la carpeta «{ $folder }».
server-job-a-pack = un paquet
server-job-ingest = Important { $pack }
server-job-retag = Tornant a etiquetar { $pack }
server-job-llm-tag = Etiquetant { $pack } amb IA
server-job-convert = Convertint { $pack } amb Blender
server-job-unreal = Convertint els paquets d'Unreal de { $pack }
server-job-reformat = Convertint els fitxers de { $pack } segons les regles de conversió
server-job-scan = Escanejant una carpeta vigilada
server-job-relocate = Movent fitxers entre ubicacions de la biblioteca
server-job-looks = Deduint colors i aspecte
server-job-sounds = Escoltant sons
server-job-embed = Aprenent què mostren les imatges
server-job-hear = Aprenent com sonen els sons
server-job-materials = Cercant materials a { $pack }
server-job-read-files = Llegint els fitxers de { $pack }
server-job-store-lookup = Consultant una pàgina de botiga
server-maintenance-backup = Fes una còpia de seguretat de la base de dades
server-maintenance-backup-blurb = Una còpia coherent de la base de dades (paquets, etiquetes, notes, col·leccions i Configuració) a la carpeta de còpies de seguretat que hi ha al costat. Els fitxers de la biblioteca no hi són: no canvien mai un cop desats, així que qualsevol sincronització de fitxers en fa còpia.
server-maintenance-scrub = Comprova si els fitxers estan malmesos
server-maintenance-scrub-blurb = Torna a llegir cada fitxer desat i el compara amb la seva empremta, per detectar fitxers que s'han degradat o han desaparegut d'un disc o NAS. Triga una estona en una biblioteca gran.
server-maintenance-reanalyze = Torna a analitzar tots els paquets
server-maintenance-reanalyze-blurb = Torna a llegir cada fitxer amb aquesta versió de Hoard: les miniatures, les previsualitzacions, els detalls i les etiquetes de les regles es fan de nou. Les vostres edicions es mantenen; no es torna a consultar la IA.
server-maintenance-clear-cache = Buida les conversions en memòria cau
server-maintenance-clear-cache-blurb = Elimina els fitxers conservats de les conversions de «Baixa com a» (es tornen a fer quan es demanen) i el que hagi deixat la feina interrompuda.
server-maintenance-gc = Suprimeix els fitxers no utilitzats
server-maintenance-gc-blurb = Suprimeix els fitxers desats als quals ja no apunta res, com ara els fitxers de paquets suprimits definitivament.
server-update-no-address = no s'ha definit cap adreça a cridar
server-update-no-answer = no hi ha hagut resposta en 30 segons

## Configuració: Aparença

appearance-language = Llengua
appearance-language-auto = La del navegador ({ $name })
appearance-scheme = Clar o fosc
appearance-scheme-system = Sistema
appearance-size-section = Text i espaiat
appearance-text-size = Mida del text
appearance-text-small = Petit
appearance-text-normal = Normal
appearance-text-large = Gran
appearance-density = Espaiat
appearance-density-comfortable = Còmode
appearance-density-compact = Compacte
appearance-tiles = Miniatures
appearance-tiles-what = El que mostra cada miniatura de la biblioteca a més de la imatge.
appearance-tile-names = Noms
appearance-tile-details = Tipus i mida (píxels, durada, triangles)
appearance-tile-badges = Tipus de fitxer sobre la imatge
appearance-tile-motion = Reprodueix les animacions mentre el punter hi és a sobre
appearance-tile-motion-always = Mantén les animacions en marxa tota l’estona, no només sota el punter
appearance-top-bar = Barra superior
appearance-top-bar-what = Amaga el que no facis servir. / continua anant al quadre de cerca i { $keys } continua obrint la paleta d'ordres.
appearance-top-activity = Feina en segon pla
appearance-top-theme = Commutador de clar i fosc
appearance-top-mic = Micròfon al quadre de cerca, per trobar sons fent-ne un
appearance-top-slash = La indicació / al quadre de cerca
appearance-top-palette = Botó de la paleta d'ordres ({ $keys })
appearance-news = Novetats
appearance-news-what = Les novetats del Hoard porten un punt daurat que brilla fins que les veus, i la barra superior les aplega en un sol lloc. El que has vist va lligat al teu compte.
appearance-news-point-out = Assenyala les novetats

## Arrel

root-unreachable = No s'ha pogut connectar amb el servidor de Hoard: { $error }
root-unreachable-desktop = Hoard no respon: { $error }
root-desktop-reopen = Aquesta finestra ha tancat la sessió. Tanqueu Hoard i torneu-lo a obrir per continuar.
root-try-again = Torna-ho a provar
root-lazy-connect = la connexió de l'aplicació
window-minimize = Minimitza
window-maximize = Maximitza
window-restore = Restaura
window-close = Tanca

## Aplicació: la biblioteca, la barra superior i els filtres

app-lazy-settings = la configuració
app-lazy-archive = l'arxiu
app-lazy-shortcuts = les dreceres
app-lazy-setup = la configuració inicial
app-lazy-explorer = l'explorador de malles
app-lazy-palette = la paleta d'ordres
app-lazy-bulk-edit = l'editor en bloc
app-lazy-news = les novetats
app-lazy-pack-details = els detalls del paquet
app-theme-system = Segueix el tema del sistema
app-theme-light = Tema clar
app-theme-dark = Tema fosc
app-color-red = Vermell
app-color-orange = Taronja
app-color-yellow = Groc
app-color-green = Verd
app-color-teal = Verd blavós
app-color-blue = Blau
app-color-purple = Lila
app-color-pink = Rosa
app-color-brown = Marró
app-color-sand = Sorra
app-color-gray = Gris
app-color-slate = Pissarra
app-color-black = Negre
app-color-white = Blanc
app-color-swatch = { $color }: recursos amb molt de { $lower }
app-color-any = Qualsevol color
app-color-pick = Tria qualsevol color
app-color = Color
app-music = Música
app-tempo = BPM
app-tempo-min = des de
app-tempo-max = fins a
app-tempo-min-title = Música a aquest tempo o més ràpida, en pulsacions per minut (també compta la meitat o el doble)
app-tempo-max-title = Música a aquest tempo o més lenta, en pulsacions per minut
app-key = Tonalitat
app-key-any = Qualsevol tonalitat
app-key-major = Major
app-key-minor = Menor
app-key-mix = I tonalitats que s'hi barregen bé
app-key-mix-title = La seva relativa major o menor i les tonalitats a una quinta per sobre i per sota, que els DJ hi barregen
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Escriu el soroll que fa un so tal com l'escriuries: primer surten els sons que s'hi assemblen
app-imitation-record = Fes el soroll al micròfon: primer surten els sons que s'hi assemblen
app-imitation-stop = Atura l'enregistrament
app-imitation-heard = El teu soroll
app-imitation-mic-failed = No s'ha pogut enregistrar del micròfon: { $error }
app-retry = Torna-ho a provar
app-undo = Desfés
app-search-failed = La cerca ha fallat: { $error }
app-unreachable = No s'ha pogut connectar amb el servidor: { $error }
app-unreachable-desktop = Hoard no respon: { $error }
app-forget-project-title = Voleu oblidar { $name }?
app-forget-project-body = Hoard deixarà de mostrar-lo allà on s'utilitzen els seus { $count } recursos, fins que el següent hoard sync li ho torni a dir. No canvia res al projecte.
app-forget-project-confirm = Oblida'l
app-forget-project-failed = No s'ha pogut oblidar { $name }: { $error }
app-forget-project-label = Oblida el projecte { $name }
app-forget-project = Oblida aquest projecte
app-save-name-like = com { $name }
app-save-name-favorites = preferits
app-search-saved = S'ha desat «{ $name }»: és a Filtres, a Vostres
app-search-updated = S'ha actualitzat «{ $name }»
app-rank-alike = Més semblants
app-rank-color = Amb més d'aquest color
app-rank-match = Millor coincidència
app-rank-imitation = Els que més hi sonen
app-sort = Ordena
app-sort-name = Nom
app-sort-created = Creació
app-sort-updated = Actualització
app-sort-size = Mida
app-sort-rating = Valoració
app-sort-newest-first = Primer els més recents
app-sort-oldest-first = Primer els més antics
app-sort-a-to-z = De la A a la Z
app-sort-z-to-a = De la Z a la A
app-sort-largest-first = Primer els més grans
app-sort-smallest-first = Primer els més petits
app-sort-best-rated-first = Primer les més ben valorades
app-sort-lowest-rated-first = Primer les pitjor valorades
app-collection-added =
    { $count ->
        [one] S'ha afegit { $count } recurs a { $name }
       *[other] S'han afegit { $count } recursos a { $name }
    }
app-collection-add-failed = No s'ha pogut afegir a la col·lecció: { $error }
app-collection-removed = S'han tret { $count } de { $name }
app-collection-remove-failed = No s'ha pogut treure de la col·lecció: { $error }
app-ns-subject = Tema
app-ns-style = Estil
app-ns-theme = Ambientació
app-ns-material = Material
app-ns-map = Mapa PBR
app-ns-color = Color
app-ns-mood = Estat d'ànim
app-ns-genre = Gènere
app-ns-license = Llicència
app-ns-res = Resolució
app-ns-duration = Durada
app-ns-poly = Nombre de polígons
app-ns-format = Format
app-ns-weight = Pes
app-ns-tech = Tècnic
app-ns-blender = Versió de Blender
app-ns-depth = Profunditat de bits
app-ns-ucs = Categoria de so (UCS)
app-ns-bpm = Tempo
app-ns-root = Nota fonamental
app-ns-level = Sonoritat
app-ns-ue = Classe d'Unreal
app-ns-engine = Motor
app-ns-extracted = Font recuperada
app-ns-script = Sistema d'escriptura
app-bulk-failed =
    { $count ->
        [one] No s'ha pogut canviar aquest recurs: { $error }
       *[other] No s'han pogut canviar aquests { $count } recursos: { $error }
    }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] S'ha copiat { $count } etiqueta
       *[other] S'han copiat { $count } etiquetes
    }
app-tags-copy-none = { $name } no té etiquetes per copiar
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = No s'ha copiat cap etiqueta. Prem C en un recurs per copiar-ne les etiquetes.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [one] Ja té aquestes etiquetes
       *[other] Els { $count } ja tenen aquestes etiquetes
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [one]
            { $count ->
                [one] S'ha afegit { $count } etiqueta
               *[other] S'han afegit { $count } etiquetes
            }
       *[other]
            { $count ->
                [one] S'ha afegit { $count } etiqueta a { $assets } recursos
               *[other] S'han afegit { $count } etiquetes a { $assets } recursos
            }
    }
app-bulk-untagged =
    { $count ->
        [one] S'ha tret { $tag } de { $count } recurs
       *[other] S'ha tret { $tag } de { $count } recursos
    }
app-bulk-tagged =
    { $count ->
        [one] S'ha etiquetat { $count } recurs amb { $tag }
       *[other] S'han etiquetat { $count } recursos amb { $tag }
    }
app-bulk-review = Marca com a revisats
app-bulk-review-title = Marca els recursos seleccionats com a revisats: les regles i la IA deixaran en pau les seves etiquetes
app-bulk-unreview = Marca com a no revisats
app-bulk-unreview-title = Treu la marca de revisat dels recursos seleccionats
app-bulk-reviewed =
    { $count ->
        [one] S'ha marcat com a revisat { $count } recurs
       *[other] S'han marcat com a revisats { $count } recursos
    }
app-bulk-unreviewed =
    { $count ->
        [one] S'ha tret la marca de revisat a { $count } recurs
       *[other] S'ha tret la marca de revisat a { $count } recursos
    }
app-bulk-accept = Conserva les proposades
app-bulk-accept-title = Conserva com a pròpies totes les etiquetes que les regles i la IA han posat als recursos seleccionats
app-bulk-accepted =
    { $count ->
        [one] S'han conservat les etiquetes proposades de { $count } recurs
       *[other] S'han conservat les etiquetes proposades de { $count } recursos
    }
app-credits-failed = No s'han pogut generar els crèdits: { $error }
app-archived-assets =
    { $count ->
        [one] S'ha arxivat { $count } recurs
       *[other] S'han arxivat { $count } recursos
    }
app-archived-one = S'ha arxivat { $name }
app-open-failed = No s'ha pogut obrir aquest recurs: { $error }
app-unnamed-pack = paquet
app-unnamed-collection = col·lecció
app-unnamed-asset = recurs
app-home-version = Hoard { $version } — torna a la biblioteca
app-home = Torna a la biblioteca
app-search-label = Cerca recursos
app-search-placeholder = Cerca noms, camins, etiquetes
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = roca or pedra: una de les dues · -molsa: exclou-la · "molsa seca": aquestes paraules juntes · pack:bosc: al nom d'un paquet
app-alt-leave-out = Alt+clic per excloure-ho
app-views = Vistes
app-library = Biblioteca
app-packs = Paquets
app-packs-working = { $count } en curs
app-update-title = Ha sortit Hoard { $version } (aquest servidor executa { $current }). Baixeu la imatge o el binari nou i reinicieu; la base de dades s'actualitza sola.
app-update = Actualitza
app-theme = Tema
app-sign-in = Inicia la sessió
app-archive-title = Arxiu: { $contents }
app-archive = Arxiu
app-settings = Configuració
app-palette-title = Paleta d'ordres ({ $keys })
app-import-folder = Importa una carpeta
app-add-files = Afegeix fitxers
app-add-files-short = Afegeix
app-split-label = Com s'ha d'importar aquesta carpeta
app-split-before = { "" }
app-split-after = { " " }conté { $count } carpetes o arxius comprimits.
app-split-each = Importa cadascun com a paquet propi ({ $count })
app-split-one = Importa com a un sol paquet
app-cancel = Cancel·la
app-filters = Filtres
app-done = Fet
app-yours = Vostres
app-favorites = Preferits
app-needs-review = Per revisar
app-needs-review-title = Només els recursos que ningú no ha revisat encara (funciona amb els altres filtres)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Valorats
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } estrella o més
       *[other] { $count } estrelles o més
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Feu clic un altre cop per mostrar totes les valoracions
app-forget-search-label = Oblida la cerca desada { $name }
app-forget-search = Oblida aquesta cerca
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Obre la biblioteca amb aquesta cerca
app-default-search-on-title = La biblioteca s'obre amb aquesta cerca. Fes clic per deixar-ho.
app-default-search-label = Obre la biblioteca amb { $name }
app-rename-search = Canvia el nom
app-rename-search-label = Nou nom per a { $name }
app-projects-title = Projectes que fan hoard sync des d'aquí, amb el que tenia la seva última sincronització
app-projects = Projectes
app-project-synced = Sincronitzat { $when }
app-project-synced-by = Sincronitzat { $when } per { $who }
app-collections = Col·leccions
app-no-filters = No hi ha filtres per a aquesta cerca.
app-kind = Tipus
app-facets-more = { $count } més
app-facets-fewer = menys
app-searching = S'està cercant…
app-searching-live = S'està cercant
app-asset-count =
    { $count ->
        [one] { $count } recurs
       *[other] { $count } recursos
    }
app-clear-all = Esborra-ho tot
app-save-name-label = Nom per a aquesta cerca
app-save-name-placeholder = Poseu-hi un nom
app-save = Desa
app-save-search-title = Conserva aquesta cerca a Filtres, a Vostres
app-save-search = Desa la cerca
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Actualitza «{ $name }»
app-update-search-title = Conserva el que està triat ara en aquesta cerca desada
app-save-search-new = Desa com a nova
app-select-title = Trieu diversos recursos per etiquetar-los, categoritzar-los, afegir-los a una col·lecció, baixar-los o arxivar-los alhora (X, o Ctrl+clic)
app-select = Selecciona
app-explore-title = Explora aquests models en 3D
app-tile-size = Mida de les miniatures
app-supporting-title = Memòries intermèdies de glTF, biblioteques de materials, els mapes dels materials, materials que només porta un model i els fitxers de botiga que descriuen models. S'inclouen quan baixeu el que els fa servir. Les pàgines d’un full de sprites, i els sprites que el paquet també té com a imatges soltes, es mostren amb el full. Un recurs que el paquet té en diversos formats es mostra un sol cop, en el format que Hoard llegeix millor.
app-supporting-hidden =
    { $count ->
        [one] { $count } fitxer auxiliar amagat
       *[other] { $count } fitxers auxiliars amagats
    }
app-supporting-show = Mostra
app-supporting-shown = S'inclouen els fitxers auxiliars
app-supporting-hide = Amaga
app-folders = Carpetes
app-folders-title = Mostra el paquet com era al disc, carpeta per carpeta
app-folders-label = Carpetes del paquet
app-welcome-title = Deixeu anar un paquet de recursos a qualsevol lloc d'aquesta pàgina
app-welcome-before = Funcionen zips, tarballs, carpetes o fitxers solts. Hoard els descomprimeix, desa cada fitxer un sol cop pel seu hash, llegeix mides, durades i nombres de polígons, i ho etiqueta tot perquè pugueu cercar coses com ara{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }o{ " " }
app-welcome-after = .
app-welcome-import-folder = Importa una carpeta
app-welcome-choose-files = Tria fitxers
app-none-similar = Encara no hi ha res més que s'assembli gaire a { $name }.
app-none = No hi ha cap coincidència.
app-none-query = No hi ha cap coincidència per a «{ $query }».
app-none-imitation = Encara no hi ha cap so mesurat que soni gaire com «{ $word }».
app-none-heard = Encara no hi ha cap so mesurat que soni gaire com el teu soroll.
app-none-filters = No hi ha cap coincidència amb aquests filtres.
app-none-query-filters = No hi ha cap coincidència per a «{ $query }» amb aquests filtres.
app-clear-search-filters = Esborra la cerca i els filtres
app-loading-more = S'està carregant més
app-keys-hint = Les fletxes es mouen, Enter obre, Espai reprodueix un so, X o Ctrl+clic en selecciona diversos (pulsació llarga en pantalles tàctils).
app-resize-label = Redimensiona el tauler de detalls
app-resize-title = Arrossegueu per redimensionar · doble clic per a l'amplada que s'ajusta
app-selected-label = Recursos seleccionats
app-selected = { $count } seleccionats
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Edita…
app-edit-selected-title = Canvia les etiquetes i els camps de tots (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] { $count } resultat seleccionat
       *[other] Tots { $count } resultats seleccionats
    }
app-pick-assets = Trieu recursos
app-download-zip = Baixa un zip
app-credits-title = CREDITS.md dels paquets d'on provenen: qui els ha fet, les seves llicències i les línies de crèdit que demanen
app-credits = Crèdits
app-prefer-sources-title = SoundWaves i textures d'Unreal el fitxer original dels quals ha recuperat Hoard
app-prefer-sources = WAV/PNG recuperat en lloc de .uasset ({ $count })
bag-clear = Buida la selecció
bag-glb-heading = Models com a GLB
bag-glb-text = Cada model com a fitxer .glb dret amb la Y amunt i mirant endavant, amb les textures com a fitxers a part en una carpeta textures que comparteixen els models. Tota la resta hi va tal com és.
bag-glb-download = Baixa el zip GLB
bag-plain-heading = Fitxers tal com són
bag-plain-text = Cada fitxer tal com es va importar, amb el que necessita, en una carpeta per a cada paquet.
bag-remove = Treu { $name } de la selecció
bag-more = i { $count } més
app-bulk-tag-label = Una etiqueta per afegir o treure dels recursos seleccionats
app-bulk-tag-placeholder = Etiqueta
app-bulk-tag-add-title = Afegeix aquesta etiqueta a tots els recursos seleccionats
app-bulk-tag-add = Afegeix
app-bulk-tag-remove-title = Treu aquesta etiqueta de tots els recursos seleccionats
app-bulk-tag-remove = Treu
app-remove-from-collection = Treu de { $name }
app-unfavorite = Treu de preferits
app-favorite = Afegeix a preferits
app-select-all = Selecciona'ls tots ({ $count })
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] Selecciona el resultat ({ $count })
       *[other] Selecciona els { $count } resultats
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] Edita { $count } recurs
       *[other] Edita { $count } recursos
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] Edita { $count } resultat
       *[other] Edita els { $count } resultats
    }
bulk-edit-close = Tanca
bulk-edit-tags = Etiquetes
bulk-edit-tag-add = Afegeix una etiqueta a tots
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } de { $count }
bulk-edit-untick = Desmarca-ho tot
bulk-edit-untick-title = Treu totes les etiquetes i marca després les que vulguis conservar
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = Les etiquetes més usades entre ells
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Barrejat, es deixa tal com està
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Es deixa tal com està
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Esborrat a tots
bulk-edit-reset = Torna a deixar-ho com estava
bulk-edit-all-note = Això canvia tots els resultats, també els que encara no s'han carregat, i no es pot desfer d'un sol cop.
bulk-edit-save =
    { $count ->
        [one] Desa { $count } canvi
       *[other] Desa { $count } canvis
    }
bulk-edit-nothing = No ha canviat res
bulk-edit-saved =
    { $count ->
        [one] S'ha actualitzat { $count } recurs
       *[other] S'han actualitzat { $count } recursos
    }
app-archive-selected-title = Fora de la cerca fins que es restaurin o se suprimeixin definitivament, des de l'Arxiu
app-drop-items = Deixeu anar { $count } elements per afegir-los
app-drop = Deixeu anar per afegir-ho a Hoard
app-drop-hint = Cada fitxer o carpeta de primer nivell es converteix en un paquet propi. Els gigabytes no són cap problema; els fitxers grans es reprenen si cau la connexió.

## Paleta d'ordres

palette-group-go-to = Ves a
palette-group-actions = Accions
palette-group-theme = Tema
palette-group-news = Novetats
palette-news-show = Mostra les novetats
palette-news-mark-all = Marca totes les novetats com a vistes
palette-news-keywords = nou novetats canvis funcions versió vist new news
palette-group-kinds = Tipus
palette-group-packs = Paquets
palette-group-collections = Col·leccions
palette-group-projects = Projectes
palette-group-saved = Cerques desades
palette-group-assets = Recursos
palette-hint-current = actual
palette-library = Biblioteca
palette-library-keywords = recursos graella inici assets grid home
palette-favorites = Preferits
palette-favorites-keywords = destacats estrelles m'agrada favorits starred stars liked
palette-packs = Paquets
palette-archive = Arxiu
palette-archive-keywords = paperera reciclatge suprimits eliminats restaura trash recycle bin deleted restore
palette-stats = Estadístiques de la biblioteca
palette-stats-keywords = estadístiques xifres mida disc espai recompte statistics size disk space count
palette-copies = Còpies gairebé idèntiques
palette-copies-keywords = duplicats iguals semblants redimensionats convertits espai malgastat duplicates same similar resized converted wasted space dedupe
palette-search = Cerca
palette-add-files = Afegeix fitxers…
palette-add-files-keywords = puja pujar importa upload import
palette-import-folder = Importa una carpeta…
palette-import-folder-keywords = puja paquet upload pack
palette-select-all = Selecciona els { $count } que es mostren
palette-select-all-keywords = marca check
palette-select-several = Selecciona'n diversos…
palette-select-several-keywords = massiu múltiple etiqueta arxiva bulk multiple tag archive
palette-archive-selected = Arxiva els { $count } seleccionats
palette-archive-selected-keywords = suprimeix elimina treu paperera delete remove trash
palette-edit-selected = Edita els { $count } seleccionats
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = canvi massiu etiquetes categoria llicència valoració
palette-paste-tags =
    { $count ->
        [one] Enganxa les etiquetes copiades al recurs seleccionat
       *[other] Enganxa les etiquetes copiades als { $count } seleccionats
    }
palette-clear-selection = Esborra la selecció
palette-clear-filters = Esborra els filtres
palette-clear-filters-keywords = restableix reinicia reset
palette-hide-supporting = Amaga els fitxers auxiliars
palette-show-supporting = Mostra els fitxers auxiliars
palette-supporting-keywords = mapes normals rugositat màscares normal maps roughness masks
palette-account = Compte i testimonis d'accés
palette-account-keywords = contrasenya inici sessió usuari testimoni api password login user token
palette-sign-out = Tanca la sessió ({ $name })
palette-sign-out-keywords = tanca sortir desconnecta log out logout
palette-sign-in = Inicia la sessió
palette-sign-in-keywords = inicia entra compte contrasenya log in login account password
palette-settings = Configuració
palette-settings-keywords = blender ffmpeg llm ia etiquetatge unreal administrador eines preferències ai tagging admin tools preferences
palette-shortcuts = Dreceres de teclat
palette-shortcuts-keywords = ajuda tecles help keys
palette-theme-keywords = tema aparença mode theme appearance
palette-theme-variant-keywords = aparença color appearance
palette-light-theme = Tema clar: { $name }
palette-dark-theme = Tema fosc: { $name }
palette-kind-all = Tots els tipus (deixa de mostrar només { $kind })
palette-kind-only = Només { $kind }
palette-title = Paleta d'ordres
palette-placeholder = Cerqueu recursos, aneu a un paquet, executeu una ordre…
palette-results = Resultats
palette-searching = S'està cercant…
palette-no-match = No hi ha cap coincidència per a «{ $query }»
palette-foot-move = mou
palette-foot-run = executa
palette-foot-shortcuts = totes les dreceres

## Dreceres de teclat

shortcuts-title = Dreceres de teclat
shortcuts-close = Tanca
shortcuts-anywhere = A qualsevol lloc
shortcuts-palette = Paleta d'ordres: cerca, salta, executa
shortcuts-search = Cerca
shortcuts-this-list = Aquesta llista
shortcuts-escape = Tanca, esborra la selecció o tanca els detalls
shortcuts-library = Biblioteca
shortcuts-move = Mou entre miniatures (o J i K)
shortcuts-open = Obre els detalls
shortcuts-play-sound = Reprodueix o atura un so
shortcuts-select = Selecciona o desselecciona
shortcuts-favorite = Afegeix als preferits o treu-ne
shortcuts-reviewed = Marca com a revisat o no
shortcuts-tag = Afegeix una etiqueta
shortcuts-edit = Edita la selecció
shortcuts-copy-tags = Copia'n les etiquetes
shortcuts-paste-tags = Enganxa les etiquetes copiades a la selecció
shortcuts-key-click = clic
shortcuts-select-range = Selecciona un interval
shortcuts-select-all = Selecciona tot el que es mostra
shortcuts-sound-player = Reproductor de so
shortcuts-play-pause = Reprodueix o fa una pausa
shortcuts-skip = Salta enrere o endavant
shortcuts-start-end = Inici o final
shortcuts-image-inspector = Inspector d'imatges
shortcuts-zoom = Amplia o redueix
shortcuts-fit = Ajusta a la finestra
shortcuts-actual-pixels = Píxels reals
shortcuts-tile = Mosaic de 3×3 per comprovar les juntures
shortcuts-background = Fons següent
shortcuts-channel = Mostra un sol canal

## KindChips.vue

kinds-group = Tipus de recurs

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = afegida per una persona
tag-source-rule = de l'etiquetador fora de línia
tag-source-meta = mesurada a partir del fitxer
tag-source-ai = suggerida per la IA
tag-remove = Treu l'etiqueta { $tag }
tag-accept = Conserva l'etiqueta { $tag }
tag-accept-title = Conserva aquesta proposta com a etiqueta pròpia
tag-reject-title = Treu-la; l'etiquetador no la tornarà a posar
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Etiqueta nova
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Volies dir
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = abans { $alias }

## ActivityIndicator.vue

activity-label-idle = Activitat en segon pla: inactiva
activity-label-running = Activitat en segon pla: { $running } en curs
activity-label-queued = Activitat en segon pla: { $running } en curs, { $queued } en cua
activity-title = Activitat en segon pla
activity-unreachable = No es pot connectar amb el servidor
activity-unreachable-desktop = Hoard no respon
activity-empty = No hi ha res en curs. Les importacions, l'etiquetatge, les conversions i les baixades de zip apareixen aquí mentre funcionen.
activity-queued = en cua
activity-stop = Atura «{ $what }»
activity-stop-tagging = Atura tot l'etiquetatge amb IA ({ $count })
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } triangles
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Reprodueix { $name }
tile-pause = Posa en pausa { $name }
tile-favorite = Afegeix { $name } als preferits
tile-in-favorites = Als vostres preferits
tile-add-favorite = Afegeix als preferits (F)
tile-more-like = Més com { $name }
tile-more-like-this = Més com aquest
tile-select = Selecciona { $name }

## CollectionPicker.vue

collections-add = Afegeix a una col·lecció
collections-menu = Col·leccions
collections-find-label = Cerqueu o creeu una col·lecció
collections-find-placeholder = Cerqueu o anomeneu una col·lecció nova
collections-create = Crea
collections-empty = Encara no hi ha col·leccions. Escriviu un nom per començar-ne una.

## FolderPicker.vue

folders-list = Carpetes
folders-close = Tanca
folders-browse = Navega…
folders-up-title = Puja una carpeta
folders-up = Amunt
folders-this-computer = Aquest ordinador
folders-none = No hi ha cap carpeta aquí.

## LocationForm.vue

locations-mode-question = Què ha de fer Hoard amb aquesta carpeta?
locations-mode-managed = Reclama-la: Hoard hi guarda els meus paquets
locations-mode-managed-hint = Els paquets que pugeu o importeu s'hi copien, i Hoard els manté en ordre. Ara la carpeta és de Hoard, així que no en toqueu el contingut.
locations-mode-watched = Vigila-la: els meus paquets ja hi són
locations-mode-watched-hint = Cada carpeta que conté es converteix en un paquet. Hoard llegeix els fitxers on són i no els canvia, mou ni suprimeix mai, i incorpora les carpetes noves a mesura que apareixen.
locations-folder-desktop = Carpeta
locations-folder-placeholder = La carpeta on són els vostres paquets
locations-folder-hint-managed = Hoard la crea si no existeix. A Docker, és una carpeta que heu muntat al contenidor.
locations-folder-hint-managed-desktop = Hoard la crea si no existeix.
locations-folder-hint-watched = A Docker, primer munteu la carpeta al contenidor (només de lectura ja va bé) i després trieu-la aquí.
locations-folder-hint-watched-desktop = Hoard només la llegeix: no s'hi canvia res.
locations-name = Nom
locations-name-placeholder-managed = Biblioteca
locations-name-placeholder-watched = Els meus paquets
locations-files = Fitxers que van aquí
locations-catch-all = També tot el que no accepta cap altra ubicació
locations-kinds-hint = Els fitxers d'un paquet nou van a la ubicació que té marcat el seu tipus. La resta van a la general.
locations-files-all = Tots, de moment. Més endavant podeu afegir més ubicacions des de Configuració per enviar alguns tipus (la música, per exemple) a un altre disc.
locations-first-watched-note = Per pujar cal una biblioteca que gestioni Hoard. En podeu afegir una més endavant des de Configuració → Ubicacions de la biblioteca.
locations-checking = S'està comprovant la carpeta…
locations-add = Afegeix la ubicació
locations-cancel = Cancel·la

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = No s'han pogut carregar les ubicacions de la biblioteca
locations-takes-and-rest = { $kinds } i tota la resta
locations-takes-rest = Tot el que no accepten les altres
locations-takes-everything = Tot
locations-takes-nothing = Encara res
locations-save-failed = No s'ha pogut desar la ubicació
locations-unwatch-title = Voleu deixar de vigilar { $name }?
locations-unwatch-body =
    { $count ->
        [one] El seu únic paquet surt de Hoard, amb les etiquetes i les notes. Els fitxers de la carpeta no es toquen.
       *[other] Els seus { $count } paquets surten de Hoard, amb les etiquetes i les notes. Els fitxers de la carpeta no es toquen.
    }
locations-unwatch-body-empty = Els fitxers de la carpeta no es toquen.
locations-unwatch = Deixa de vigilar
locations-remove-title = Voleu eliminar { $name }?
locations-remove-body = Primer, els fitxers que hi hagi es mouen a les altres ubicacions, segons el tipus; s'elimina quan queda buida. No es perd res.
locations-remove-body-only = Si conté fitxers, primer afegiu una altra ubicació on es puguin moure.
locations-remove = Elimina
locations-removed = S'ha eliminat { $name }
locations-emptying = S'estan traient els fitxers de { $name }; s'eliminarà quan acabi
locations-offline-title = No es pot accedir a { $name }
locations-offline-body = No se'n poden treure els fitxers mentre està fora de línia. Voleu eliminar-la igualment? Els fitxers que encara hi hagi es mostraran com a absents.
locations-remove-anyway = Elimina igualment
locations-remove-failed = No s'ha pogut eliminar { $name }
locations-rebalancing = S'estan movent els fitxers desats a les ubicacions que accepten el seu tipus
locations-rebalance-failed = No s'han pogut moure els fitxers
locations-scanned-never = encara no s'ha escanejat
locations-scanned-now = escanejada fa un moment
locations-scanned-minutes = escanejada fa { $mins } min
locations-scanned-hours = escanejada fa { $hours } h
locations-scanned-date = escanejada el { $date }
locations-scan-failed = No s'ha pogut escanejar { $name }
locations-watching = Vigilant { $name }
locations-added = S'ha afegit { $name }
locations-loading = S'està carregant…
locations-managed-title = Biblioteques que gestiona Hoard
locations-watched-title = Carpetes vigilades
locations-managed-hint = Les pujades i importacions es copien aquí. Cada ubicació accepta els tipus que té marcats; la general accepta la resta.
locations-watched-hint = Carpetes de paquets indexades on són. Hoard no canvia mai els fitxers que contenen, i cerca carpetes noves i eliminades cada 15 minuts.
locations-managed-empty = Encara no n'hi ha cap, així que no hi ha on posar una pujada. Afegiu-ne una a sota.
locations-reachable = Accessible
locations-unreachable-now = Ara mateix no és accessible
locations-catch-all-tag = general
locations-packs =
    { $count ->
        [one] { $count } paquet
       *[other] { $count } paquets
    }
locations-unreachable = no és accessible
locations-scanning = S'està escanejant…
locations-scan = Escaneja ara
locations-edit = Edita
locations-catch-all-locked = Per canviar-ho, feu que una altra ubicació sigui la general
locations-edit-hint-before = S'aplica als fitxers que s'afegeixin a partir d'ara. Per moure el que ja està desat, feu servir
locations-edit-hint-after = a sota.
locations-save = Desa
locations-rebalance = Mou els fitxers desats perquè hi coincideixin
locations-rebalance-hint = Després de canviar el que accepta una ubicació: els fitxers ja desats es mouen a la ubicació on va ara el seu tipus.
locations-add-title = Afegeix una ubicació
locations-add-hint = Un altre disc per a alguns tipus de fitxers, o una carpeta de paquets que ja teniu.
locations-kind-question = Quin tipus d’ubicació és?
locations-kind-folder = Una carpeta
locations-kind-folder-hint = En un disc o una unitat compartida d’un NAS muntada al contenidor, o una que Dropbox, Google Drive, OneDrive o iCloud Drive mantinguin sincronitzada. Hoard hi pot desar fitxers o vigilar paquets que ja teniu.
locations-kind-folder-hint-desktop = En aquest ordinador, un altre disc o una unitat compartida d’un NAS, o una que Dropbox, Google Drive, OneDrive o iCloud Drive mantinguin sincronitzada. Hoard hi pot desar fitxers o vigilar paquets que ja teniu.
locations-kind-s3-hint = Un bucket d’Amazon S3 o d’un servei semblant: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard hi desa fitxers.
locations-s3-title = Emmagatzematge S3
locations-s3-hint = Una ubicació s3://bucket-name és un bucket d'aquest servei: AWS o qualsevol de compatible amb S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage en un NAS).
locations-bucket = Bucket
locations-bucket-hint = El nom d’un bucket que ja existeixi. Les claus de dalt l’han de poder llistar, llegir i escriure.
locations-s3-endpoint = Adreça
locations-s3-endpoint-placeholder = Buida per a AWS; p. ex. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Regió
locations-s3-access-key = ID de la clau d'accés
locations-s3-secret-key = Clau d'accés secreta
locations-s3-headers = Capçaleres addicionals
locations-s3-headers-placeholder = Name=value, una per línia (per a Cloudflare Access, per exemple)
locations-s3-kept = Desat (escriviu per substituir-lo)
locations-s3-secrets = Les claus i les capçaleres es guarden amb la configuració del servidor i no es tornen a mostrar mai.
locations-s3-secrets-desktop = Les claus i les capçaleres es guarden al clauer d'aquest ordinador i no es tornen a mostrar mai.
locations-s3-saved = S'ha desat l'emmagatzematge S3
locations-s3-save-failed = No s'ha pogut desar l'emmagatzematge S3
locations-add-more = Afegeix una ubicació…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = No s'han pogut carregar les incidències
issues-started =
    { $count ->
        [one] S'ha tornat a començar.
       *[other] S'han tornat a començar { $count } coses.
    }
issues-retry-failed = No s'ha pogut tornar a provar
issues-dismiss-failed = No s'ha pogut descartar
issues-dismiss-all-title = Voleu descartar-les totes ({ $count })?
issues-dismiss-all-body = S'esborra «{ $title }». Tot el que torni a fallar es tornarà a mostrar.
issues-dismiss-all = Descarta-les totes
issues-just-now = fa un moment
issues-minutes-ago = fa { $mins } min
issues-hours-ago = fa { $hours } h
issues-loading = S'està carregant…
issues-none = No ha fallat res.
issues-action-all = { $action } (totes)
issues-open-asset = Obre'l
issues-open-pack = Mostra el paquet
issues-open-locations = Ubicacions de la biblioteca
issues-where-before = a
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } vegada
       *[other] { $count } vegades
    }
issues-starting = S'està començant…
issues-dismiss-named = Descarta { $title }
issues-dismiss = Descarta
issues-show-fewer = Mostra'n menys
issues-show-all = Mostra-les totes ({ $count })
issues-not-listed = N'hi ha { $count } més que no es mostren.

## archive.ts

archive-anyway = Arxiva igualment
archive-failed = No s'ha pogut arxivar: { $error }
archive-undo = Desfés
archive-restore-failed = No s'ha pogut restaurar: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } paquet
       *[other] { $packs } paquets
    } i { $assets ->
        [one] { $assets } recurs
       *[other] { $assets } recursos
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } paquet
       *[other] { $packs } paquets
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } recurs
       *[other] { $assets } recursos
    }
archive-count-nothing = res

## ArchiveView.vue

archive-load-failed = No s'ha pogut carregar l'arxiu: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = S'ha restaurat: { $what }
archive-purged = S'ha suprimit definitivament: { $what }
archive-purge-failed = No s'ha pogut suprimir: { $error }
archive-purge-title = Voleu suprimir { $name } definitivament?
archive-purge-pack-body =
    { $count ->
        [one] També se'n van el seu únic recurs i { $size } de fitxers, excepte els fitxers que comparteixi un altre paquet. No es pot desfer.
       *[other] També se'n van els seus { $count } recursos i { $size } de fitxers, excepte els fitxers que comparteixi un altre paquet. No es pot desfer.
    }
archive-purge = Suprimeix per sempre
archive-purge-asset-body = El seu fitxer s'allibera, tret que un altre recurs el comparteixi. No es pot desfer.
archive-empty-title = Voleu buidar l'arxiu?
archive-empty-body = Se suprimirà definitivament: { $what }, i se n'alliberaran els fitxers. No es pot desfer.
archive-empty = Buida l'arxiu
archive-title = Arxiu
archive-intro = Els paquets i recursos arxivats queden fora de la cerca, dels recomptes i de la llista d'etiquetes. Restaureu-los, o suprimiu-los definitivament per alliberar-ne l'espai.
archive-loading = S'està carregant…
archive-is-empty = L'arxiu és buit
archive-is-empty-hint = Arxiveu un paquet o un recurs i s'esperarà aquí fins que el restaureu o el suprimiu definitivament.
archive-packs = Paquets
archive-assets = Recursos
archive-asset-count =
    { $count ->
        [one] { $count } recurs
       *[other] { $count } recursos
    }
archive-archived = arxivat { $when }
archive-from = de { $pack }
archive-restore = Restaura
archive-show-more = Mostra'n més (en queden { $count })

## mine.ts

projects-favorite-add-failed = No s'ha pogut afegir als preferits: { $error }
projects-favorite-remove-failed = No s'ha pogut treure dels preferits: { $error }
projects-save-search-failed = No s'ha pogut desar la cerca: { $error }
projects-forget-search-failed = No s'ha pogut eliminar { $name }: { $error }
projects-change-search-failed = No s'ha pogut canviar { $name }: { $error }

## projects.ts

projects-list-and = { $items } i { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } més
projects-other =
    { $count ->
        [one] { $count } projecte més
       *[other] { $count } projectes més
    }
projects-usage-title-one = { $name } fa servir això
projects-usage-title = { $count } projectes fan servir això
projects-usage-body-one = Es fa servir a { $list }: el seu proper hoard sync els traurà.
projects-usage-body = Es fa servir a { $list }: el proper hoard sync de cadascun els traurà.

## uploader.ts

upload-network-error = error de xarxa
upload-no-reply = cap resposta en { $seconds } s
upload-retrying = Es torna a provar { $what } ({ $attempt } de { $attempts }): { $error }
upload-gave-up = { $what }: { $error } després de { $attempts } intents
upload-failed-plain = ha fallat
upload-already-imported = ja s'havia importat
upload-part-failed = la pujada ha fallat ({ $status })
upload-batch = lot de { $count } fitxers
upload-batch-failed = el lot ha fallat ({ $status })
upload-finish = acabar la pujada

## UploadQueue.vue

upload-eta-seconds = queden { $s } s
upload-eta-minutes = queden { $min } min
upload-eta-hours = queden { $h } h
upload-queue = Importacions
upload-importing = S'estan important { $count } paquets
upload-bytes-of = { $sent } de { $total }
upload-rate = { $size }/s
upload-waiting = En espera
upload-unpacking = Pujat, ara s'està descomprimint
upload-already = Ja és a la biblioteca
upload-stopped = Aturat
upload-files-of = { $done } de { $total } fitxers
upload-resumed = represa, { $size } ja hi eren
upload-skipped = Aquests mateixos fitxers ja s'havien importat com a «{ $name }», així que s'han omès.
upload-error = { $error }. El progrés es conserva: torneu-ho a provar, o torneu a deixar anar els mateixos fitxers més tard.
upload-retry = Torna-ho a provar
# $name is the pack the library has already that these files go into.
upload-update = Actualitza «{ $name }»
upload-copy = Importa com a còpia
# $name is the pack the library has already that the upload goes into.
upload-into = Va a «{ $name }», que ja és a la biblioteca: s'hi afegeixen els fitxers nous i se substitueixen els que han canviat.
upload-own-pack = Fes-ne un paquet propi
upload-dismiss = Descarta
upload-cancel = Cancel·la
upload-minimize = Minimitza
upload-expand = Mostra les importacions
upload-pill = S’estan important { $count } · { $percent } %
upload-pill-stopped = Aturades: { $count }
upload-interrupted = Interrompuda
upload-resume = Reprèn
upload-resume-all = Reprèn-les totes
upload-resume-hint-folder = Torneu a triar la mateixa carpeta, o deixeu-la anar a la pàgina, per continuar on es va aturar.
upload-resume-hint-file = Torneu a triar el mateix fitxer, o deixeu-lo anar a la pàgina, per continuar on es va aturar.
upload-not-same = No són els mateixos fitxers, així que no s’ha reprès res.
upload-permission-denied = El navegador no ha tingut permís per tornar a llegir els fitxers. Trieu-los vosaltres.

## PacksView.vue

packs-rename-failed = No s'ha pogut canviar el nom: { $error }
packs-collection-deleted = S'ha suprimit la col·lecció { $name }
packs-delete-failed = No s'ha pogut suprimir: { $error }
packs-status-uploading = S'està pujant (fitxers rebuts fins ara)
packs-status-ingesting = S'està descomprimint i etiquetant
packs-status-analyzing = L'LLM està etiquetant
packs-status-ready = Llest
packs-status-failed = Ha fallat
packs-retag-failed = No s'ha pogut començar a tornar a etiquetar: { $error }
packs-read-files-failed = No s'han pogut llegir els fitxers del paquet: { $error }
packs-context-load-failed = No s'han pogut carregar els detalls de botiga del paquet: { $error }
packs-undo-store-failed = No s'ha pogut treure la pàgina de botiga: { $error }
packs-context-save-failed = No s'han pogut desar els detalls de botiga: { $error }
packs-unreal-load-failed = No s'ha pogut carregar l'estat de la conversió d'Unreal: { $error }
packs-unreal-discard-title = Voleu descartar els originals?
packs-unreal-discard-body = Els fitxers .uasset originals de { $name } s'eliminen un cop convertits. Les dades exclusives d'Unreal que contenen, com ara els grafs de materials, es perden definitivament.
packs-unreal-discard-confirm = Converteix i descarta
packs-unreal-convert-failed = No s'ha pogut començar la conversió: { $error }
packs-unreal-fact-converted = { $done } convertits des d'Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } paquet d'Unreal
       *[other] { $left } paquets d'Unreal
    }
packs-unreal-fact-both = { $done } convertits des d'Unreal, { $left } encara són paquets
packs-thumbs-canceled = S'ha cancel·lat després de { $done } de { $total }
packs-thumbs-drew =
    { $count ->
        [one] S'ha dibuixat { $count } model
       *[other] S'han dibuixat { $count } models
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: s'ha substituït { $improved } miniatura.
       *[other] { $lead }: s'han substituït { $improved } miniatures.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: s'ha substituït { $improved } miniatura, { $failed } no s'han pogut dibuixar.
       *[other] { $lead }: s'han substituït { $improved } miniatures, { $failed } no s'han pogut dibuixar.
    }
packs-archived = S'ha arxivat { $name } i els seus { $count } recursos
packs-blobs-bad = { $count } fitxers no han superat l'última comprovació d'integritat.
packs-collections = Col·leccions
packs-browse-named = Explora { $name }
packs-collection-name = Nom de la col·lecció
packs-save = Desa
packs-cancel = Cancel·la
packs-assets =
    { $count ->
        [one] { $count } recurs
       *[other] { $count } recursos
    }
packs-updated = Última actualització: { $date }
packs-browse = Explora
packs-collection-download = Baixa (.zip)
packs-collection-credits = Crèdits
packs-collection-credits-title = Qui ha fet els paquets d'on provenen aquests recursos, les seves llicències i les línies de crèdit que demanen
packs-rename = Canvia el nom
packs-delete = Suprimeix
packs-packs = Paquets
packs-empty = Encara no hi ha paquets. Deixeu anar un zip o una carpeta a qualsevol lloc d'aquesta pàgina per afegir-ne un.
packs-find = Cerca un paquet
packs-find-placeholder = Cerqueu un paquet per nom, botiga, llicència o etiqueta
packs-sort = Ordre
packs-sort-new = Primer els més recents
packs-sort-old = Primer els més antics
packs-sort-size = Primer els més grans
packs-sort-assets = Primer els que tenen més recursos
packs-sort-count = Nombre de recursos
packs-sort-smallest = Primer els més petits
packs-sort-fewest = Primer els que tenen menys recursos
packs-with-kind = Paquets amb
packs-state = Estat
packs-state-any = Qualsevol estat
packs-state-working = En curs
packs-state-failed = Han fallat
packs-state-untagged = Encara sense descripció
packs-state-unlicensed = Sense llicència indicada
packs-store = Botiga
packs-store-any = Qualsevol botiga
packs-store-from = De { $store }
packs-store-none = Sense pàgina de botiga
packs-shown-of = { $shown } de { $total }
packs-clear = Esborra
packs-no-match = No hi ha cap paquet que hi coincideixi.
packs-clear-all = Esborra la cerca i els filtres
packs-pack-name = Nom del paquet
packs-thumbs-drawing = S'estan dibuixant miniatures en aquest navegador…
packs-thumbs-drawing-count = S'estan dibuixant miniatures en aquest navegador: { $done } de { $total }…
packs-thumbs-stopped = S'ha aturat: { $error }
packs-thumbs-none = Tots els models d'aquí ja tenen una miniatura dibuixada amb totes les textures.
packs-dismiss = Descarta
packs-license = Llicència: { $license }
packs-license-tier = Llicència: { $license } ({ $tier })
packs-license-not-found = no s'ha trobat
packs-from-before = De
packs-from-after = { "" }
packs-added = Afegit el { $date }
pack-panel-label = Sobre aquest paquet
pack-panel-hide = Amaga els detalls del paquet
pack-cover-previous = Imatge anterior
pack-cover-next = Imatge següent
pack-cover-picture = Imatge { $n } de { $count }
pack-panel-kinds = Què conté
pack-panel-all = Tots els paquets
packs-download = Baixa el paquet (.zip)
packs-store-details = Detalls de botiga
packs-more-named = Més accions per a { $name }
packs-more = Més
packs-rename-more = Canvia el nom…
packs-retag-rules = Torna a executar l'etiquetador fora de línia
packs-retag-llm = Torna a executar l'etiquetador LLM
packs-make-thumbs = Fes miniatures en aquest navegador
packs-make-thumbs-title = Obre cada model en un visualitzador 3D ocult, d'un en un, i fa servir la seva imatge com a miniatura quan és millor que la del servidor
packs-make-thumbs-title-desktop = Obre cada model en un visualitzador 3D ocult, d'un en un, i fa servir la seva imatge com a miniatura quan és millor que la que ha fet Hoard
packs-read-files = Torna a llegir els fitxers del paquet
packs-credits = Baixa els crèdits
packs-credits-title = CREDITS.md: qui ha fet el paquet, la seva llicència i la línia de crèdit que demana
packs-unreal-open = Converteix el contingut d'Unreal…
packs-archive = Arxiva el paquet
packs-unreal-intro = Hoard llegeix els paquets d'Unreal per si mateix i desa el que contenen com a fitxers normals: les textures en PNG o EXR, els sons en WAV, els materials en JSON. El fitxer convertit ocupa el lloc del paquet d'Unreal i en conserva les etiquetes, els enllaços i les col·leccions.
packs-unreal-converted =
    { $count ->
        [one] convertit
       *[other] convertits
    }
packs-unreal-converted-kept =
    { $count ->
        [one] convertit, { $kept } amb l'original conservat
       *[other] convertits, { $kept } amb l'original conservat
    }
packs-unreal-pending =
    { $count ->
        [one] encara no revisat
       *[other] encara no revisats
    }
packs-unreal-needs-editor =
    { $count ->
        [one] a l'espera d'Unreal Editor
       *[other] a l'espera d'Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] sense cap format comú
       *[other] sense cap format comú
    }
packs-unreal-failed =
    { $count ->
        [one] ha fallat
       *[other] han fallat
    }
packs-unreal-reasons = Per què queden paquets
packs-unreal-editor-before = Les malles i les animacions necessiten Unreal Editor. En una màquina amb Unreal instal·lat, executeu
packs-unreal-editor-command = hoard unreal-worker --editor <camí a UnrealEditor-Cmd>
packs-unreal-editor-after = ; els exporta en FBX i en puja els resultats aquí.
packs-unreal-discard = Descarta els fitxers .uasset originals un cop convertits
packs-unreal-discard-hint = Allibera l'espai. Les dades exclusives d'Unreal, com ara els grafs de materials, es perden.
packs-unreal-drop = Suprimeix els paquets sense cap format comú
packs-unreal-drop-hint = Blueprints, nivells i altres recursos exclusius del motor.
packs-unreal-convert = Converteix
packs-context-hint-before = D'on prové el paquet i què en diu el venedor. L'etiquetador LLM ho llegeix amb cada fitxer, així que una descripció de botiga com «120 encanteris de gel i foc» l'ajuda a anomenar
packs-context-hint-after = .
packs-store-auto = Trobada automàticament: el mateix paquet esmenta aquesta pàgina.
packs-store-not-this = No és aquesta
packs-lookup-label = Cerca a les botigues
packs-lookup-placeholder = El nom del paquet a la botiga
packs-lookup-searching = S'està cercant…
packs-lookup-search = Cerca a les botigues
packs-lookup-hide = Amaga
packs-lookup-clues = Dels fitxers del paquet: { $clues }.
packs-lookup-also = També s'ha cercat { $queries }.
packs-lookup-quoted = «{ $query }»
packs-lookup-nothing = No hi ha res a Fab, itch.io, Kenney ni OpenGameArt que s'hi assembli. Proveu amb menys paraules o amb unes altres.
packs-lookup-use = Fes-la servir
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Ara
packs-review-store = De la botiga
packs-review-same = Igual
packs-review-read-page = Es llegirà de la pàgina en desar
packs-review-take =
    { $count ->
        [one] Omple { $count } camp
       *[other] Omple { $count } camps
    }
packs-lookup-searched = S'ha cercat el { $date }.
packs-lookup-again = Torna a cercar
packs-lookup-again-after = .
packs-lookup-unreachable = Aquest cop no s'ha pogut connectar amb { $stores }.
packs-lookup-also-look = Mireu també a
packs-lookup-links-and = { " " }i{ " " }
packs-lookup-browser-only = (només s'hi pot cercar des del navegador); enganxeu l'adreça de la pàgina a sota.
packs-lookup-open = Cerca a les botigues…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Venedor
packs-seller-placeholder = Qui l'ha fet
packs-license-label = Llicència
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Nivell de llicència
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] { $count } recurs té llicència pròpia: fes que segueixi també la del paquet
       *[other] { $count } recursos tenen llicència pròpia: fes que segueixin també la del paquet
    }
packs-store-page = Pàgina de botiga
packs-store-text = Descripció de la botiga
packs-store-text-placeholder = Enganxeu el text de l'anunci, o deixeu-ho buit per obtenir-lo de la pàgina de botiga.
packs-store-text-fetch = En desar, s'obté la descripció de la pàgina de botiga.
packs-readme = README trobat al paquet
packs-save-retag = Desa i torna a executar l'etiquetador LLM

## AssetDetail.vue

asset-lazy-3d-viewer = el visualitzador 3D
asset-lazy-text = el text
asset-lazy-text-viewer = el visualitzador de text
asset-lazy-image-viewer = el visualitzador d'imatges
asset-lazy-material-preview = la previsualització del material
asset-lazy-material-viewer = el visualitzador de materials
asset-lazy-sprite-player = el reproductor de sprites
asset-lazy-sheet-view = el visualitzador de fulls de sprites
asset-rel-texture = textura
asset-rel-buffer = dades de geometria
asset-rel-material = fitxer de material
asset-rel-animation = animació
asset-rel-library = fitxer enllaçat
asset-rel-guess = probablement va amb
asset-rel-sidecar = detalls de botiga
asset-rel-variant = variant
asset-rel-sprite = sprite
asset-rel-sample = mostra
asset-role-map = mapa: { $role }
asset-maps-count =
    { $count ->
        [one] { $count } mapa
       *[other] { $count } mapes
    }
asset-needs = Necessita
asset-animations = Animacions
asset-sprites = Sprites
asset-used-by = Utilitzat per
asset-loose-sprites = També com a fitxers separats
asset-in-sheet = En un full de sprites
asset-other-formats = Altres formats
asset-shown-as = Es mostra a la biblioteca com a
asset-conversion-failed = la conversió ha fallat
asset-as-converting = S'està convertint…
asset-as-converting-percent = S'està convertint… { $percent } %
asset-as-queued = En cua…
asset-download = Baixa
asset-as-label = Baixa com a
asset-as-original = { $format } (format original)
asset-as-pack-files = Els fitxers del paquet
asset-as-size = Mida
asset-as-full-size = Mida completa
asset-as-up-to = Fins a { $size } px
asset-as-hint = Trieu una mida o un altre format
asset-meta-size = Mida
asset-meta-length = Durada
asset-meta-sample-rate = Freqüència de mostreig
asset-meta-channels = Canals
asset-meta-mono = Mono
asset-meta-stereo = Estèreo
asset-meta-bit-depth = Profunditat de bits
asset-meta-bits = { $bits } bits
asset-meta-level = Nivell
asset-meta-level-value = pic { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = pic { $peak } dBFS, RMS { $rms } dBFS, satura
asset-meta-loop = Bucle
asset-meta-loop-value = de { $start } a { $end }
asset-meta-playback = Reproducció
asset-meta-one-shot = D'un sol cop
asset-meta-envelope = Envolupant
asset-meta-envelope-times = atac { $attack }, cua { $tail }
asset-meta-envelope-hit = cop
asset-meta-envelope-swell = creixent
asset-meta-envelope-sustained = sostingut
asset-meta-tone = Timbre
asset-meta-tone-value = brillantor { $khz } kHz, soroll { $noise } %
asset-meta-pitch = Altura
asset-meta-pitch-rises =
    { $octaves ->
        [one] puja { $octaves } octava
       *[other] puja { $octaves } octaves
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] baixa { $octaves } octava
       *[other] baixa { $octaves } octaves
    }
asset-meta-loops-cleanly = Es repeteix sense talls
asset-meta-tempo = Tempo
asset-meta-tempo-heard = uns { $bpm } BPM, d'oïda
asset-meta-tempo-heard-or = uns { $bpm } BPM (o { $alt }), d'oïda
asset-meta-key = Tonalitat
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, d'oïda
music-key-major = { $note } major
music-key-minor = { $note } menor
asset-meta-root-note = Nota fonamental
asset-meta-cue-markers = Marcadors
asset-meta-library = Biblioteca
asset-meta-keywords = Paraules clau
asset-meta-title = Títol
asset-meta-artist = Artista
asset-meta-album = Àlbum
asset-meta-genre = Gènere
asset-meta-generator = Generador
asset-meta-everything-with = Mostra tot el que té l’etiqueta { $tag }
asset-meta-saved-with = Desat amb
asset-meta-color-mode = Mode de color
asset-meta-layers = Capes
asset-meta-range = Rang
asset-meta-hdr = HDR (la previsualització té mapatge de tons)
asset-meta-format = Format
asset-meta-large-document = Document gran (PSB)
asset-meta-ai = IA
asset-meta-ai-made = Creat amb IA generativa
asset-meta-ai-edited = Editat amb IA generativa
asset-meta-ai-how-metadata = metadades
asset-meta-ai-how-generator = configuració del generador
asset-meta-ai-how-iptc = tipus de font digital IPTC
asset-meta-ai-says = { $made }, segons { $how }
asset-meta-ai-by-says = { $made } ({ $by }), segons { $how }
asset-meta-provenance = Procedència
asset-meta-c2pa = Té credencials de contingut C2PA
asset-meta-compression = Compressió
asset-meta-note = Nota
asset-meta-maps = Mapes
asset-meta-sizes =
    { $count ->
        [one] Mida
       *[other] Mides
    }
asset-meta-triangles = Triangles
asset-meta-vertices = Vèrtexs
asset-meta-meshes = Malles
asset-meta-materials = Materials
asset-meta-animations = Animacions
asset-meta-skins = Skins
asset-meta-unreal-asset = Recurs d'Unreal
asset-meta-unreal-saved-with = { $class }, desat amb UE { $engine }
asset-meta-package = Paquet
asset-meta-cooked = Cuinat (versió de joc): no té previsualitzacions ni fonts
asset-meta-tiling = Mosaic
asset-meta-seamless = Sense juntures en tots dos eixos
asset-meta-analysis-error = No s'ha pogut llegir del tot aquest fitxer: { $error }
asset-copy-blocked = No s'ha pogut copiar: el navegador ho ha bloquejat
asset-save-error = No s'ha pogut desar: { $error }
asset-tag-removed = S'ha tret { $tag }
asset-tag-restored = S'ha tornat a posar { $tag }
asset-tags-added =
    S'han afegit { $count ->
        [one] { $count } etiqueta
       *[other] { $count } etiquetes
    }
asset-tags-accepted =
    { $count ->
        [one] S'ha conservat { $count } etiqueta proposada
       *[other] S'han conservat { $count } etiquetes proposades
    }
asset-reviewed = Revisat
asset-reviewed-on-title = Revisat: les regles i la IA deixen en pau les seves etiquetes. Feu clic per treure la marca (R)
asset-reviewed-off-title = Marca com a revisat perquè les regles i la IA deixin en pau les seves etiquetes (R)
asset-reviewed-on = Marcat com a revisat
asset-reviewed-off = S'ha tret la marca de revisat
# $name is an asset's name.
asset-reviewed-on-named = S'ha marcat { $name } com a revisat
asset-reviewed-off-named = S'ha tret la marca de revisat a { $name }
asset-undo = Desfés
asset-play-error = No s'ha pogut reproduir aquest fitxer: { $error }
asset-copied-hash = S'ha copiat el hash
asset-copied-toml = S'ha copiat el toml
asset-copied-id = S'ha copiat l'ID
asset-copied-link = S'ha copiat l'enllaç
asset-origin-before = Aquest fitxer era
asset-origin-class-engine = , un { $class } desat per UE { $engine }.
asset-origin-class = , un { $class }.
asset-origin-engine = { " " }desat per UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor el va exportar el { $date }.
asset-origin-editor-notes = Unreal Editor el va exportar el { $date } ({ $notes }).
asset-origin-hoard = Hoard el va extreure del paquet el { $date }.
asset-origin-hoard-notes = Hoard el va extreure del paquet el { $date } ({ $notes }).
asset-origin-download = Baixa el .{ $ext } original
asset-origin-discarded = El paquet original es va descartar després de la conversió.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Historial
history-loading = S'està carregant…
history-none = Encara no s'hi ha canviat res.
history-load-failed = No s'ha pogut carregar l'historial: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = IA
history-by-rule = Una regla
history-by-someone = Algú
history-by-app = { $who } mitjançant una aplicació
history-by-assistant = { $who } mitjançant un assistent
history-field-description = Descripció
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Etiqueta { $tag } afegida
history-tag-removed = Etiqueta { $tag } treta
history-tag-kept = Etiqueta suggerida { $tag } conservada
history-tag-changed = Etiqueta { $tag } canviada
history-reviewed = Marcat com a revisat
history-unreviewed = Marcat com a no revisat
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: establert a { $to }
history-cleared = { $field }: esborrat (era { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Desfés
history-revert-title = Torna a deixar el que aquest canvi va modificar
history-reverted = S'ha desfet: { $change }
history-revert-failed = No s'ha pogut desfer: { $error }
asset-details = Detalls del recurs
asset-in-pack = a
asset-previous = Recurs anterior
asset-next = Recurs següent
asset-favorite = Preferit
asset-favorite-on-title = Als vostres preferits (F el treu)
asset-favorite-off-title = Afegeix als preferits (F)
asset-share = Comparteix
asset-share-title = Comparteix un enllaç a aquest recurs (qui l'obri inicia la sessió a Hoard primer)
asset-layout-panel = Mostra en un tauler lateral
asset-layout-page = Mostra a pàgina completa
asset-layout-page-title = Mostra a pàgina completa: una previsualització gran al costat dels detalls
asset-close = Tanca els detalls
asset-rename-title = Feu clic per canviar el nom
asset-name = Nom
asset-views-alt = El model des de quatre direccions
asset-inspect-aria = Inspecciona { $name } a pantalla completa
asset-inspect-title = Inspecciona: zoom, mosaic, canals
asset-playback-position = Posició de reproducció
asset-playback-value = { $current } de { $duration }
asset-pause = Pausa
asset-play = Reprodueix
asset-waveform-drawing = S'està dibuixant la forma d'ona…
asset-waveform-skipped = No hi ha forma d'ona per a fitxers tan grans
asset-play-animation = Reprodueix com a animació
asset-show-views = Mostra el full de quatre vistes que ha vist l'etiquetador
asset-palette = Colors de la imatge
asset-swatch-title = { $hex }, { $percent } % de la imatge. Cerca recursos amb aquest color
asset-swatch-aria = Cerca recursos amb { $hex }, { $percent } % d'aquesta imatge
asset-more-like-this = Més com aquest
asset-desc-label = La vostra descripció
asset-desc-placeholder = Què és, on el faríeu servir…
asset-save = Desa
asset-cancel = Cancel·la
asset-desc-family = Es desa a totes les { $count } variants
asset-desc-edit = Edita la descripció
asset-desc-write = Escriu una descripció
asset-desc-ai = Escrita per la IA
asset-desc-file = Del fitxer
asset-desc-store-before = De la
asset-desc-store-page = pàgina de { $store }
asset-desc-store-page-unnamed = pàgina de botiga
asset-desc-store-after = (tot el paquet)
asset-less = Menys
asset-more = Més
asset-tags = Etiquetes
asset-tag-add-label = Afegeix una etiqueta
asset-tag-add-placeholder = Afegiu una etiqueta, p. ex. mood:tense
asset-apply-family = Aplica els canvis a totes les { $count } variants
asset-tags-accept-all =
    { $count ->
        [one] Conserva { $count } proposta
       *[other] Conserva les { $count } propostes
    }
asset-tags-accept-all-title = Conserva com a pròpies totes les etiquetes proposades per les regles i la IA
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Rebutjades:
asset-tag-restore-title = Torna a posar { $tag }
asset-tags-also-under = També dins de:
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = Els semblants tenen:
asset-tag-suggest-title = Afegeix { $tag }
asset-details-heading = Detalls
asset-field-category = Categoria
asset-field-license = Llicència
asset-field-source = Origen
asset-field-rating = Valoració
# Shown in place of an empty value, to fill it in.
asset-field-add = Afegeix
asset-field-new = Afegeix un camp
asset-field-name = Nom del camp
asset-field-value = Valor
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = del paquet
asset-field-license-placeholder = Com CC0 o CC-BY 4.0
asset-field-remove-placeholder = Buit per eliminar-lo
# $field is a field's name, like "License".
asset-field-edit-title = Edita { $field }
asset-field-saved = S'ha desat { $field }
asset-field-cleared = S'ha esborrat { $field }
asset-rate =
    { $count ->
        [one] { $count } estrella
       *[other] { $count } estrelles
    }
asset-rated =
    Valorat amb { $count ->
        [one] { $count } estrella
       *[other] { $count } estrelles
    }
asset-rating-clear-title = Feu clic un altre cop per esborrar la valoració
asset-rating-cleared = S'ha esborrat la valoració
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Feu clic un altre cop per a { $count } estrella
       *[other] Feu clic un altre cop per a { $count } estrelles
    }
asset-collections = Col·leccions
asset-collection-remove = Treu de { $name }
asset-collection-add-another = Afegeix a una altra
asset-collection-add = Afegeix a una col·lecció
asset-used-in = Utilitzat a
asset-used-in-title = Mostra què té { $name }
asset-used-in-synced = sincronitzat { $when }
asset-maps = Mapes
asset-show-more = Mostra'n { $count } més
asset-materials-heading =
    { $count ->
        [one] Material
       *[other] Materials
    }
asset-material-maps-aria = { $name }: mapes
asset-variants = Variants ({ $count } preses)
asset-fit-animations = Animacions que hi encaixen ({ $count })
asset-fit-characters = Personatges on encaixa ({ $count })
asset-copies = Còpies idèntiques
asset-copies-note = Els mateixos bytes es desen un sol cop; aquests paquets també inclouen aquest fitxer.
asset-near-copies = Còpies gairebé idèntiques
asset-near-copies-note = El mateix en una altra mida, freqüència de mostreig o format. La millor còpia està marcada.
asset-near-copies-best = El mateix en una altra mida, freqüència de mostreig o format. Aquesta és la millor còpia.
asset-near-best = la millor
asset-unreal-converted = Convertit des d'Unreal
asset-rule-converted = Convertit per una regla
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Una regla de conversió va crear aquest { $format } el { $date } a partir de
asset-rule-kept-until = L'original es conserva fins al { $date }.
asset-rule-rebuilt = El FLAC conserva tot el que hi havia al WAV, de manera que el WAV es pot baixar o recuperar tal com era en qualsevol moment.
asset-rule-put-back = Restaura l'original
asset-rule-put-back-title = Voleu restaurar { $file }?
asset-rule-put-back-body = El fitxer convertit s'elimina i l'original n'ocupa el lloc, amb les seves etiquetes, col·leccions i enllaços. A partir d'aleshores, les regles de conversió ja no el toquen.
asset-rule-put-back-done = S'ha restaurat { $file }
asset-rule-let-go = L'original es va eliminar quan es va acabar el temps durant el qual es conserven els originals segons la Configuració.
asset-unreal-package = Del paquet d'Unreal
asset-ue-needs-editor-before = A l'espera d'Unreal Editor: { $reason }. En una màquina amb Unreal instal·lat, executeu
asset-ue-needs-editor-after = i tornarà en FBX.
asset-ue-editor-path = camí a UnrealEditor-Cmd
asset-ue-unsupported = Es conserva com a paquet: { $reason }.
asset-ue-failed = La conversió ha fallat: { $reason }
asset-recovered = El .{ $ext } original estava desat dins d'aquest paquet, i Hoard l'ha extret.
asset-recovered-download = Baixa el .{ $ext }
asset-ue-meshes-before = Les malles i les animacions es desen en el format propi d'Unreal. Convertiu el paquet (pàgina del paquet, Més) i després executeu
asset-ue-meshes-after = al costat d'una instal·lació d'Unreal Editor per obtenir FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } etiqueta del registre de recursos
       *[other] { $count } etiquetes del registre de recursos
    }
asset-file = Fitxer
asset-bytes = Bytes
asset-source = Origen
asset-path = Camí
asset-path-open-folder = Mostra què més hi ha a { $folder }
asset-download-maps =
    { $count ->
        [one] Baixa { $count } mapa
       *[other] Baixa tots els { $count } mapes
    }
asset-download-linked =
    { $count ->
        [one] Baixa amb { $count } fitxer enllaçat
       *[other] Baixa amb { $count } fitxers enllaçats
    }
asset-download-only-title = Només aquest fitxer, sense els fitxers que fa servir
asset-download-only = Només aquest fitxer
asset-include-animations = Inclou les animacions al .zip
asset-include-character = Inclou el personatge { $name }
asset-copy-toml = Copia l'entrada de hoard.toml
asset-copy-id = Copia l'ID
asset-teach = Ensenya aquest fitxer a Hoard
asset-teach-title = Descriviu el fitxer d'aquesta botiga perquè Hoard el llegeixi (Configuració → Fitxers de botiga)
asset-archive = Arxiva
asset-archive-title = Fora de la cerca fins que es restauri o se suprimeixi definitivament, des de l'Arxiu

## ImageViewer.vue

image-channel-red = Canal vermell
image-channel-green = Canal verd
image-channel-blue = Canal blau
image-channel-alpha = Canal alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Fons d'escaquer (K canvia)
image-bg-black-title = Fons negre (K canvia)
image-bg-gray-title = Fons gris (K canvia)
image-bg-white-title = Fons blanc (K canvia)
image-bg-checker = fons d'escaquer
image-bg-black = fons negre
image-bg-gray = fons gris
image-bg-white = fons blanc
image-inspect-aria = Inspecciona { $name }
image-preview-size = previsualització { $width }×{ $height }, { $bytes }
image-preview = previsualització
image-resolution = Resolució
image-full-title = Carrega la imatge completa de { $width }×{ $height } (F); ampliar més enllà de la previsualització també ho fa
image-full-loading = S'està carregant la resolució completa…
image-full-failed = No s'ha carregat la resolució completa: torneu-ho a provar
image-full = Resolució completa
image-zoom = Zoom
image-zoom-out-title = Redueix (−)
image-zoom-out = Redueix
image-zoom-in-title = Amplia (+)
image-zoom-in = Amplia
image-fit-title = Ajusta (0)
image-fit = Ajusta
image-actual-title = Píxels reals (1)
image-view = Vista
image-tile-title = Mosaic de 3×3 per comprovar les juntures (T)
image-tile = Mosaic
image-channels = Canals
image-close = Tanca (Esc)
image-failed = No s'ha pogut carregar aquesta imatge.
image-loading = S'està carregant…
image-key-gestures = Desplaceu o pessigueu per fer zoom · arrossegueu per moure · doble clic per ampliar o ajustar
image-key-tile = mosaic
image-key-background = fons
image-key-full = resolució completa
image-key-channels = canals

## SpritePlayer.vue

sprite-frame-aria = { $name }, fotograma { $frame } de { $count }
sprite-failed = No es pot reproduir.
sprite-loading = S'estan carregant els fotogrames…
sprite-previous = Fotograma anterior
sprite-next = Fotograma següent
sprite-pause = Posa en pausa l'animació
sprite-play = Reprodueix l'animació
sprite-animation = Animació
sprite-all-frames = Tots els fotogrames
sprite-grid-title = Com es talla el full en fotogrames
sprite-columns = Columnes
sprite-rows = Files
sprite-fps-aria = Fotogrames per segon
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
       *[other] { $count } sprites
    }
sheet-pages =
    { $count ->
        [one] { $count } pàgina
       *[other] { $count } pàgines
    }
sheet-animations =
    { $count ->
        [one] { $count } animació
       *[other] { $count } animacions
    }
# The box that narrows the list of sprites by name.
sheet-find = Cerca un sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Es mostren { $shown } de { $total }
sheet-no-match = Cap sprite no té això al nom.
sheet-failed = Aquest full no es pot llegir.
sheet-loading = S'està carregant el full…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Format propi de Hoard: les pàgines tal com són, els sprites girats en qualsevol sentit i retallats, amb animacions i la durada de cada fotograma.
sheet-note-texturepacker = Sprites girats en sentit horari i retallats, amb animacions per nom, tal com els llegeixen PixiJS i Phaser.
sheet-note-clockwise = Sprites girats en sentit horari i retallats. No pot contenir animacions: els fotogrames es reconeixen pels seus noms numerats.
sheet-note-libgdx = Sprites girats en sentit antihorari i retallats. No pot contenir animacions: els fotogrames es reconeixen pels seus noms numerats.
sheet-note-godot-frames = Els sprites no es giren mai i el retall es conserva com a marges, amb animacions i la durada de cada fotograma.
sheet-note-godot-tileset = Les tessel·les no es giren ni es retallen mai, en una quadrícula de la mida de la tessel·la; les imatges que ja hi són es conserven tal com són.
sheet-note-grid = Cada sprite dret sobre tot el seu llenç, en cel·les de la mateixa mida, per a qualsevol eina que talli un full en una quadrícula.
sheet-note-frames = Cada sprite dret sobre tot el seu llenç, cadascun en un PNG amb el seu nom.

## TextViewer.vue, TextPreview.vue i text.ts

text-lang-config = Configuració
text-lang-table = Taula
text-lang-code = Codi
text-lang-script = Script
text-lang-text = Text
text-copy-blocked = No s'ha pogut copiar: el navegador ho ha bloquejat
text-read-aria = Llegeix { $name }
text-lines =
    { $count ->
        [one] { $count } línia
       *[other] { $count } línies
    }
text-show-as = Mostra com a
text-tree-title = Arbre plegable (T)
text-tree = Arbre
text-pretty-title = Amb el sagnat refet (P)
text-pretty = Formatat
text-raw-title = Tal com està escrit (R)
text-raw = En brut
text-document-title = Com a document (D)
text-document = Document
text-source-title = El markdown tal com està escrit (R)
text-source = Font
text-page-title = Com la mostra un navegador des de la seva carpeta, separada del Hoard (P)
text-page = Pàgina
text-page-source-title = La pàgina tal com està escrita (R)
text-page-aria = { $name } com a pàgina
text-page-error = No s’ha pogut obrir aquesta pàgina: { $error }
text-fold = Plega
text-expand-all-title = Desplega-ho tot
text-expand-all = Desplega-ho tot
text-collapse-all-title = Plega-ho tot
text-collapse-all = Plega-ho tot
text-group-text = Text
text-wrap-title = Ajusta les línies llargues (W)
text-wrap = Ajusta
text-copy-title = Copia el text (C)
text-copied = S'ha copiat
text-copy = Copia
text-download-title = Baixa el fitxer
text-download = Baixa
text-close = Tanca (Esc)
text-truncated = Es mostren els primers { $shown } de { $total }.
text-truncated-json = Es mostren els primers { $shown } de { $total } (l'arbre necessita el fitxer sencer).
text-show-all = Mostra-ho tot
text-json-error = Aquest JSON no es pot analitzar: { $error }
text-contents-aria = Contingut de { $name }
text-loading = S'està carregant…
text-load-error = No s'ha pogut carregar aquest fitxer: { $error }
text-binary = Aquest fitxer és binari, no text, així que aquí no hi ha res per llegir. Baixeu-lo per obrir-lo amb la seva eina.
text-key-tree = arbre
text-key-pretty = formatat
text-key-raw = en brut
text-key-document = document
text-key-source = font
text-key-page = pàgina
text-key-wrap = ajusta
text-key-copy = copia
text-key-find = cerca
text-key-close = tanca
text-preview-aria = Llegeix { $name } a pantalla completa
text-preview-read = Llegeix

## JsonTree.vue i JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } element
       *[other] { $count } elements
    }
json-keys =
    { $count ->
        [one] { $count } clau
       *[other] { $count } claus
    }
json-fold = Plega
json-unfold = Desplega
json-unfold-summary = Desplega { $summary }
json-show-more = Mostra'n { $count } més
json-left = de { $count } que en queden
json-picker-more = … { $count } més

## Visualitzadors de models 3D

model-viewer-failed = el visualitzador 3D no s'ha carregat ({ $error }); potser ajuda tornar a carregar la pàgina
model-loading-viewer = S'està carregant el visualitzador 3D…
model-loading = S'està carregant el model…
model-show-failed = No s'ha pogut mostrar aquest model: { $error }
model-preview-failed = No s'ha pogut previsualitzar aquest model: { $error }
model-error-converting = el servidor encara està convertint aquest model
model-error-converting-desktop = Hoard encara està convertint aquest model
model-error-no-loader = No hi ha cap carregador del navegador per a .{ $ext }
model-dialog = Mostra { $name } en 3D
model-title-stats =
    { $triangles } triangles · { $meshes ->
        [one] { $meshes } malla
       *[other] { $meshes } malles
    } · { $materials ->
        [one] { $materials } material
       *[other] { $materials } materials
    }
model-triangles =
    { $count ->
        [one] { $count } triangle
       *[other] { $count } triangles
    }
model-group-camera = Càmera
model-group-shading = Ombrejat
model-group-background = Fons
model-group-animation = Animació
model-group-view = Vista
model-group-show = Mostra
model-frame-title = Enquadra el model (F)
model-frame = Enquadra
model-reset-title = Torna a la vista inicial (R)
model-reset = Restableix
model-turn-title = Gira lentament (A)
model-turn = Gira
model-textures-title = Textures (T)
model-textures = Textures
model-full-res-title = Textures a resolució completa (H): cada fitxer de textura tal com és, per gran que sigui; desactivat, previsualitzacions lleugeres de 2048 px com a màxim
model-full-res = Res. completa
model-light-title = Llum de l'entorn (E)
model-light = Llum
model-grid-title = Graella del terra (G)
model-grid = Graella
model-bg-checker = Fons d'escaquer
model-bg-dark = Fons fosc
model-bg-gray = Fons gris
model-bg-light = Fons clar
model-bg-env = Fons de l'entorn
model-bg-title = { $background } (K canvia)
model-pause-title = Pausa (Espai)
model-play-title = Reprodueix (Espai)
model-pause = Pausa
model-play = Reprodueix
model-animation-clip = Clip d'animació
model-animation = Animació
model-plays-on = Es reprodueix en
model-plays-everywhere = Fes-lo servir arreu
model-plays-everywhere-title = Reprodueix totes les animacions amb aquest esquelet en { $name }, també a les miniatures
model-plays-everywhere-done = Les animacions amb aquest esquelet ara es reprodueixen en { $name }. Se’n tornen a dibuixar les miniatures.
model-plays-everywhere-failed = No s’ha pogut canviar el personatge on es reprodueixen: { $error }
model-rest-pose = Postura de repòs
model-close-title = Tanca (Esc)
model-full-screen = Pantalla completa
model-open-full-screen = Obre el visualitzador 3D a pantalla completa
model-menu-fold = Plega el menú
model-menu-show = Mostra el menú
model-view-prefix = Vista:{ " " }
model-menu-render = Renderització
model-menu-maps = Mapes
model-menu-inspect = Inspecció
model-mode-final = Renderització final
model-mode-raw = Renderització en brut
model-mode-color = Color base
model-mode-roughness = Rugositat
model-mode-metalness = Metal·licitat
model-mode-normal = Normals
model-mode-ao = Oclusió
model-mode-matcap = Matcap
model-mode-uv = Comprovació d'UV
model-mode-geometry = Geometries
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Malla de filferro (W), sobre qualsevol vista
model-wireframe = Filferro
model-info-title = Informació de l'escena (I)
model-info = Informació de l'escena
model-info-close = Tanca la informació de l'escena
model-info-close-title = Tanca (I)
model-info-geometries = Geometries
model-info-materials = Materials
model-info-textures = Textures
model-info-triangles = Triangles
model-info-vertices = Vèrtexs
model-info-units = Unitats
model-info-in-meters = Mida en metres, segons les unitats que indica el fitxer
model-units-m = Metres
model-units-cm = Centímetres
model-units-mm = Mil·límetres
model-units-in = Polzades
model-units-ft = Peus
model-units-other = 1 unitat = { NUMBER($n, maximumSignificantDigits: 3) } m
model-units-unknown = No s’indica
model-info-no-units = Aquest format no indica quines són les seves unitats
model-info-height = Alçada
model-info-length = Llargada
model-info-width = Amplada
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } unitat
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } unitats
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = No s'ha trobat la textura: { $name }
model-missing-some = No s'han trobat { $count } textures: { $names }
model-missing-more = No s'han trobat { $count } textures: { $names }…
model-missing-count = No s'han trobat { $count } textures
model-guessed = Textures associades pel nom; el fitxer no n'esmenta cap.
model-group-facing = Orientació
model-facing-turn = Gira un quart
model-facing-turn-title = Gira el model un quart de volta en sentit antihorari, vist des de dalt
model-facing-tip = Inclina un quart
model-facing-tip-title = Inclina la part de dalt del model un quart de volta cap endavant, per posar dret un model ajagut
model-facing-save = Aplica al paquet
model-facing-save-title = Gira així tots els models de { $pack }. Les miniatures es tornen a dibuixar; les baixades es queden com es van fer.
model-facing-undo = Desfés
model-facing-undo-title = Torna a com estan girats els models del paquet
model-facing-saved = Ara tots els models de { $pack } estan orientats així. S'estan tornant a dibuixar les miniatures.
model-facing-failed = No s'han pogut girar els models del paquet: { $error }
model-keys-mouse = Arrossegueu per orbitar · arrossegueu amb el botó dret per moure · desplaceu per fer zoom · doble clic per enquadrar
model-key-frame = enquadra
model-key-reset = restableix
model-key-view = vista
model-key-wireframe = filferro
model-key-info = informació de l'escena
model-key-textures = textures
model-key-full-res = textures a resolució completa
model-key-light = llum
model-key-grid = graella
model-key-background = fons
model-key-play = reprodueix
explorer-dialog = Explorador de malles: vola entre els models d'aquests resultats
explorer-title = Explorador de malles
explorer-group-view = Vista
explorer-true-size = Mida real
explorer-true-size-title = Mostra cada model a la seva mida real en metres, fins a 12 m, en lloc d'ajustar-lo al seu quadrat (T)
explorer-animate = Anima
explorer-animate-title = Reprodueix el primer clip de cada model animat (Espai)
explorer-orbit = Òrbita
explorer-orbit-title = Arrossega per girar al voltant del model seleccionat en lloc de mirar al voltant (O)
explorer-open = Obre
explorer-open-title = Obre els detalls d'aquest model (Retorn)
explorer-size = { $x } × { $y } × { $z } m
explorer-empty = Cap d'aquests resultats es pot mostrar en 3D.
explorer-hud-loaded =
    { $total ->
        [one] { $loaded } de { $total } model carregat
       *[other] { $loaded } de { $total } models carregats
    }
explorer-hud-loading = { $count } carregant-se
explorer-hud-memory = ~{ $mb } MB
explorer-hud-speed = Velocitat { $speed } m/s
explorer-hud-fps = { $fps } fps
explorer-keys-mouse = Fes clic per seleccionar · arrossega per mirar · arrossega amb el botó del mig per desplaçar · Alt i arrossega per orbitar · desplaça la roda per moure't, amb el botó dret premut per canviar la velocitat · doble clic per enquadrar
explorer-key-fly = volar
explorer-key-down-up = baixar, pujar
explorer-key-faster = més ràpid
explorer-key-open = obrir
explorer-key-true-size = mida real
explorer-key-orbit = òrbita
explorer-key-animate = animar
explorer-key-close = tancar

## Materials

material-role-base-color = Color
material-role-normal = Normals
material-role-normal-dx = Normals (DirectX)
material-role-roughness = Rugositat
material-role-gloss = Brillantor
material-role-metalness = Metal·licitat
material-role-ao = Oclusió ambiental
material-role-orm = Oclusió, rugositat, metall
material-role-mra = Metall, rugositat, oclusió
material-role-rma = Rugositat, metall, oclusió
material-role-height = Alçada
material-role-bump = Relleu
material-role-cavity = Cavitat
material-role-specular = Especular
material-role-opacity = Opacitat
material-role-emissive = Emissió
material-role-translucency = Translucidesa
material-role-base-color-name = color
material-role-normal-name = normals
material-role-normal-dx-name = normals (DirectX)
material-role-roughness-name = rugositat
material-role-gloss-name = brillantor
material-role-metalness-name = metal·licitat
material-role-ao-name = oclusió ambiental
material-role-orm-name = oclusió, rugositat, metall
material-role-mra-name = metall, rugositat, oclusió
material-role-rma-name = rugositat, metall, oclusió
material-role-height-name = alçada
material-role-bump-name = relleu
material-role-cavity-name = cavitat
material-role-specular-name = especular
material-role-opacity-name = opacitat
material-role-emissive-name = emissió
material-role-translucency-name = translucidesa
material-maps =
    { $count ->
        [one] { $count } mapa
       *[other] { $count } mapes
    }
material-dialog = Mostra el material { $name }
material-up-to = fins a { $size }
material-repeat-size = { $size } per repetició
material-shape-sphere = Esfera
material-shape-cube = Cub
material-shape-plane = Pla
material-shape-title = { $shape } ({ $key })
material-group-shape = Forma
material-group-tiling = Mosaic
material-real-title = Mida real: una repetició fa { $size }, l'esfera fa 2 m de diàmetre (T canvia)
material-repeat-title = Repeteix { $n }× (T canvia)
material-real = Real
material-repeat = { $n }×
material-height-title = Quant desplaça la superfície el mapa d'alçada
material-height = Alçada
material-height-strength = Intensitat de l'alçada
material-loading = S'està carregant el material…
material-show-failed = No s'ha pogut mostrar aquest material: { $error }
material-preview-failed = No s'ha pogut previsualitzar aquest material: { $error }
material-group-maps = Mapes
material-map-show = Mostra el mapa de tipus { $map }
material-map-hide = Amaga el mapa de tipus { $map }
material-map-unused = La previsualització no el fa servir
material-open-full-screen = Obre el visualitzador de materials a pantalla completa
material-keys-mouse = Arrossegueu per orbitar · arrossegueu amb el botó dret per moure · desplaceu per fer zoom · feu clic en un mapa per desactivar-lo
material-key-shape = forma
material-key-tiling = mosaic
material-key-turn = gira

## theme.ts

theme-slate = Pissarra
theme-paper = Paper
theme-linen = Lli
theme-sage = Sàlvia
theme-lilac = Lila
theme-blush = Rosat
theme-graphite = Grafit
theme-obsidian = Obsidiana
theme-ember = Brasa
theme-moss = Molsa
theme-aubergine = Albergínia
theme-oxblood = Granat
theme-sky = Cel
theme-mint = Menta
theme-apricot = Albercoc
theme-lemon = Llimona
theme-midnight = Mitjanit
theme-lagoon = Llacuna
theme-neon = Neó
theme-light-slate-blurb = Gris blavós fred
theme-light-paper-blurb = Gris neutre senzill
theme-light-linen-blurb = Crema càlid, ambre
theme-light-sage-blurb = Verd suau
theme-light-lilac-blurb = Violeta pàl·lid
theme-light-blush-blurb = Rosa empolsat
theme-light-sky-blurb = Blau cel clar
theme-light-mint-blurb = Menta aiguamarina fresca
theme-light-apricot-blurb = Préssec suau, taronja
theme-light-lemon-blurb = Groc pàl·lid, ocre
theme-dark-graphite-blurb = Gris neutre, sense tint
theme-dark-slate-blurb = Gris blavós
theme-dark-obsidian-blurb = Gairebé negre, per a OLED
theme-dark-ember-blurb = Carbó càlid, ambre
theme-dark-moss-blurb = Negre verd bosc
theme-dark-aubergine-blurb = Negre pruna, lila
theme-dark-oxblood-blurb = Negre vi, corall
theme-dark-midnight-blurb = Blau marí fosc, blau cel
theme-dark-lagoon-blurb = Verd blavós fosc, aiguamarina
theme-dark-neon-blurb = Gairebé negre, rosa intens

## texnames.ts

texnames-needs-map = El patró necessita { "{" }map{ "}" }, o tots els mapes d'un material tindrien el mateix nom
texnames-placeholders = El patró pot fer servir { "{" }name{ "}" }, { "{" }map{ "}" } i { "{" }size{ "}" }
texnames-bad-characters = Els noms no poden contenir < > : " / \ | ? ni *
texnames-no-suffix = Doneu un sufix als mapes de tipus { $role }
texnames-bad-suffix = El sufix dels mapes de tipus { $role } no pot contenir < > : " / \ | ? ni *
texnames-slot-no-suffix = Doneu un sufix a «{ $slot }»
texnames-slot-bad-suffix = El sufix de «{ $slot }» no pot contenir < > : " / \ | ? ni *
texnames-ending-missing = Cada terminació pròpia necessita la terminació que cal buscar
texnames-ending-bad = Les terminacions pròpies no poden contenir < > : " / \ | ? ni *
texnames-ending-twice = { $ending } surt dues vegades a les vostres terminacions
texnames-slot-metallic-roughness = Metall i rugositat
texnames-slot-specular-color = Color especular
texnames-slot-specular-glossiness = Especular i brillantor
texnames-slot-clearcoat = Vernís
texnames-slot-clearcoat-roughness = Rugositat del vernís
texnames-slot-clearcoat-normal = Normals del vernís
texnames-slot-sheen-color = Color del setinat
texnames-slot-sheen-roughness = Rugositat del setinat
texnames-slot-transmission = Transmissió
texnames-slot-thickness = Gruix
texnames-slot-diffuse-transmission = Transmissió difusa
texnames-slot-diffuse-transmission-color = Color de la transmissió difusa
texnames-slot-iridescence = Iridescència
texnames-slot-iridescence-thickness = Gruix de la iridescència
texnames-slot-anisotropy = Anisotropia

## storeFormats.ts

storefiles-field-title = Títol
storefiles-field-title-hint = El títol exacte de l'anunci. Dona nom al paquet, i sense ell s'omet el fitxer.
storefiles-field-seller = Venedor
storefiles-field-seller-hint = Qui l'ha fet o el ven.
storefiles-field-license = Llicència
storefiles-field-license-hint = Tal com l'escriu el fitxer, com ara CC-BY-4.0.
storefiles-field-url = Pàgina de botiga
storefiles-field-url-hint = L'adreça de l'anunci, que comença per http:// o https://.
storefiles-field-words = Etiquetes
storefiles-field-words-hint = Paraules amb què etiquetar-lo. Els valors separats per comes es divideixen.
storefiles-field-type = Tipus
storefiles-field-type-hint = Què és, com ara «3D model» o «surface». També és una etiqueta.
storefiles-field-colors = Colors
storefiles-field-colors-hint = Els seus colors, com a etiquetes color:.
storefiles-field-id = ID de botiga
storefiles-field-id-hint = L'ID que hi dona la botiga: lletres, xifres, _ o -.
storefiles-field-origin = Lloc de creació
storefiles-field-origin-hint = On es va fer o escanejar, per a la descripció.
storefiles-field-size = Mida
storefiles-field-size-hint = Com és de gran, en paraules, per a la descripció.
storefiles-field-title-name = el títol
storefiles-field-seller-name = el venedor
storefiles-field-license-name = la llicència
storefiles-field-url-name = la pàgina de botiga
storefiles-field-words-name = les etiquetes
storefiles-field-type-name = el tipus
storefiles-field-colors-name = els colors
storefiles-field-id-name = l'ID de botiga
storefiles-field-origin-name = el lloc de creació
storefiles-field-size-name = la mida

## StoreFormatsPanel.vue

storefiles-load-failed = No s'han pogut carregar els formats de fitxers de botiga: { $error }
storefiles-remove-title = Voleu eliminar { $name }?
storefiles-remove-body = { $file } se suprimeix de la carpeta de dades. Els paquets que ja s'han llegit amb aquest format conserven el que se n'ha llegit.
storefiles-remove = Elimina
storefiles-removed = S'ha eliminat { $name }.
storefiles-remove-failed = No s'ha pogut eliminar { $name }: { $error }
storefiles-unread = Fitxers que Hoard no llegeix
storefiles-loading = S'està carregant…
storefiles-unread-none = Cap: no hi ha cap fitxer a la biblioteca que sembli d'una botiga que no es llegeix.
storefiles-unread-what = Fitxers que són a més d'un paquet, o amb nom de fitxer de botiga, que cap format no llegeix.
storefiles-unread-packs =
    { $count ->
        [one] { $count } paquet
       *[other] { $count } paquets
    }
storefiles-unread-example = p. ex. { $pack }
storefiles-teach = Ensenya aquest fitxer a Hoard
storefiles-formats = Formats
storefiles-formats-what = Els vostres es proven primer i substitueixen un d'integrat que tingui el mateix nom.
storefiles-dir-before = Es desen a
storefiles-dir-after = .
storefiles-yours = Vostre
storefiles-built-in = Integrat
storefiles-edit = Edita
storefiles-start-from = Parteix d'aquest

## TagsPanel.vue

tags-load-failed = No s'han pogut carregar les etiquetes: { $error }
tags-rename-title = Voleu canviar el nom de { $tag }?
tags-merge-title = Voleu fusionar { $count } etiquetes?
tags-rename-body = Els seus { $count } recursos passen a tenir { $into }, i els etiquetadors fan servir el nom nou a partir d'ara.
tags-merge-body = { $tags } passen a ser { $into } en un màxim de { $count } recursos. Els etiquetadors fan servir { $into } a partir d'ara.
tags-rename = Canvia el nom
tags-merge = Fusiona
tags-merged =
    { $count ->
        [one] { $count } recurs ara té l'etiqueta { $into }
       *[other] { $count } recursos ara tenen l'etiqueta { $into }
    }
tags-merge-failed = No s'ha pogut fusionar: { $error }
tags-forget-failed = No s'ha pogut canviar: { $error }
tags-find = Cerca etiquetes
tags-find-placeholder = Cerqueu etiquetes, p. ex. rock o set:
tags-sort = Ordre
tags-sort-count = Més usades
tags-sort-name = A–Z
tags-count =
    { $count ->
        [one] { $count } etiqueta
       *[other] { $count } etiquetes
    }
tags-count-more = Més de { $count } etiquetes
tags-what = Trieu etiquetes que vulguin dir el mateix i fusioneu-les, o trieu-ne una per canviar-li el nom.
tags-new-name = Nom nou
tags-merge-into = Fusiona a
tags-saved-as-before = S'ha desat com a
tags-saved-as-after = { "" }
tags-cancel = Cancel·la
tags-loading = S'està carregant…
tags-none = No hi ha etiquetes.
tags-none-containing = No hi ha cap etiqueta que contingui «{ $text }».
tags-browse-title = Explora els recursos amb l'etiqueta { $tag }
tags-browse = Explora
tags-merged-heading = Etiquetes fusionades
tags-merged-what = Quan els etiquetadors proposen un d'aquests noms antics, el recurs rep l'etiqueta a la qual es va fusionar.
tags-allow-old = Torna a permetre el nom antic
tags-lookalike-heading = Etiquetes que s’assemblen
tags-lookalike-what = Un plural, les mateixes paraules juntes, una lletra de diferència o una paraula que també és una etiqueta amb espai de noms (red i color:red). En fusionar-les es manté el nom més utilitzat, o el que té espai de noms, i els etiquetadors el fan servir a partir d’aleshores.
tags-lookalike-merge-title = Fusiona { $other } amb { $keep }
tags-lookalike-apart = Diferents
tags-lookalike-apart-title = { $a } i { $b } volen dir coses diferents: deixa d’oferir fusionar-les
tag-page-alias-remove = Oblida el nom antic { $alias }: els etiquetadors el poden tornar a posar en lloc de { $tag }
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } recurs
       *[other] { $count } recursos
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Nom antic: { $names }
       *[other] Noms antics: { $names }
    }
tag-page-aliases-label =
    { $count ->
        [one] Nom antic:
       *[other] Noms antics:
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } s'ha fusionat amb { $into }.
tag-page-show = Mostra { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Digueu què vol dir aquesta etiqueta…
tag-page-describe-label = Què vol dir aquesta etiqueta
tag-page-describe-placeholder = Per a què serveix i què hi pertany. Enter desa.
tag-page-describe-edit = Feu clic per canviar-ho
tag-page-description-saved = S'ha desat què vol dir l'etiqueta
# The tag's picture: one of its assets stands for it.
tag-page-picture = Trieu una imatge per a aquesta etiqueta
tag-page-picture-help = Trieu un dels seus recursos perquè la representi
tag-page-picture-nothing = Cap dels recursos mostrats té imatge.
tag-page-picture-none = Sense imatge
tag-page-picture-saved = Imatge definida
tag-page-picture-removed = Imatge treta
tag-page-rename = Canvia el nom…
tag-page-rename-title = Poseu-li un nom nou, o el nom d'una altra etiqueta per fusionar-la amb aquella
tag-page-rename-label = Nom nou
tag-page-save-failed = No s'ha pogut desar l'etiqueta: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Sota
tag-page-parent-add = Posa sota…
tag-page-parent-add-title = Poseu aquesta etiqueta sota una de més àmplia: en cercar-la, també es trobaran aquestes
tag-page-parent-placeholder = Una etiqueta més àmplia, com ara vehicle
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Treu { $tag } de sota { $parent }
tag-page-parent-saved = { $tag } ara és sota { $parent }
tag-page-parent-removed = { $tag } ja no és sota { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Més específiques:
tag-page-narrower-title = En cercar { $tag }, també es troben aquestes
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Més àmplies i més específiques
tags-tree-what = Posa una etiqueta sota una de més àmplia, aquí o a la seva pàgina, i en cercar la més àmplia també trobaràs tot el que hi ha a sota.
tags-tree-none = Encara no hi ha res sota res. Prova de posar cotxe sota vehicle.
tags-tree-tag-label = Etiqueta per posar sota una de més àmplia
tags-tree-tag-placeholder = Una etiqueta, com ara cotxe
tags-tree-goes-under = va sota
tags-tree-parent-label = Etiqueta més àmplia
tags-tree-put = Posa-la a sota

## LibraryStats.vue

stats-source-human = Afegides per persones
stats-source-ai = Suggerides per la IA
stats-source-rule = Regles integrades
stats-source-meta = Dels fitxers
stats-assets = Recursos
stats-packs = Paquets
stats-on-disk = Al disc
stats-duplicates-title = { $count } fitxers són còpies idèntiques byte a byte d'altres i es desen un sol cop
stats-saved-dedup = Estalviat per la deduplicació
stats-distinct-tags = Etiquetes diferents
stats-archive = A l'arxiu
stats-load-failed = No s'han pogut carregar els detalls: { $error }
stats-counting = S'està comptant…
stats-as-of = A { $time }
stats-updating = S’està actualitzant…
stats-by-kind = Espai per tipus
stats-files-size = { $name }: { $count } fitxers, { $size }
stats-months = Afegits l'últim any
stats-month = { $month }: { $count } recursos, { $size }
stats-tagging = Etiquetatge
stats-tagged = Etiquetats
stats-untagged = { $count } sense etiquetes
stats-described = Descrits
stats-described-assets = { $count } recursos
stats-human-edited = Editats a mà
stats-human-edited-unit =
    { $count ->
        [one] recurs
       *[other] recursos
    }
stats-source-different = { $count } de diferents
stats-most-used = Les més utilitzades
stats-show-tag = Mostra els recursos amb { $tag }
stats-file-types = Tipus de fitxer
stats-biggest-packs = Paquets més grans
stats-browse-pack = Explora { $name }
stats-largest-files = Fitxers més grans
stats-open-file = Obre { $name }
stats-empty = Encara no hi ha res a la biblioteca. Afegiu un paquet i les xifres apareixeran aquí.

## HoardPanel.vue

hoard-title = El vostre tresor
hoard-what = Cada recurs de la biblioteca és una peça del tresor, i el seu tipus decideix quina. Recolliu el que és nou per afegir-ho a la vostra pila.
hoard-sign-in = Inicieu la sessió per reunir un tresor propi.
hoard-new = Botí nou per recollir: { $count }
hoard-none-new = No hi ha res de nou per recollir.
hoard-collect = Recull
hoard-unavailable = Aquest navegador no pot dibuixar el tresor: { $error }
hoard-collect-failed = No s’ha pogut desar el que heu recollit: { $error }
hoard-arcade = Pantalla de recreativa
hoard-full-screen = Pantalla completa
hoard-leave-full-screen = Surt de la pantalla completa
hoard-label = Un drac que guarda el seu tresor. Peces: { $count }
hoard-gold = Or
hoard-gems = Gemmes
hoard-books = Llibres
hoard-weapons = Armes
hoard-bones = Ossos
hoard-tablets = Tauletes rúniques
hoard-paintings = Quadres
hoard-potions = Pocions

## UpdatesPanel.vue

updates-saved = Hoard la cridarà quan hi hagi una actualització a punt
updates-saved-none = No hi ha cap adreça a cridar
updates-save-failed = No s'ha pogut desar
updates-call-answered = Ha respost { $result }
updates-call-refused = No ha acceptat la sol·licitud: { $result }
updates-call-failed = No s'ha pogut cridar
updates-called = Cridada el { $when } per a { $version }: { $result }
updates-called-failed = Cridada el { $when } per a { $version }: { $result }. Hoard ho tornarà a provar d'aquí a una hora.
updates-latest = Hoard { $version } és la versió més recent.
updates-out = Ha sortit Hoard { $version }.
updates-out-idle = Ha sortit Hoard { $version }; es cridarà d'aquí a un parell de minuts, mentre no passi res.
updates-out-busy = Ha sortit Hoard { $version }; s'espera fins que no passi res ({ $work }).
updates-busy-more = { $count } més
updates-busy-jobs =
    { $count ->
        [one] { $count } tasca en espera
       *[other] { $count } tasques en espera
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } pujada
       *[other] { $count } pujades
    }
updates-list-separator = ,{ " " }
updates-what = Quan surt una versió nova, Hoard pot cridar una adreça un cop no passi res (cap pujada, tasca, conversió ni manteniment), perquè el que actualitzi el seu contenidor no el reiniciï enmig d'una pujada. Envia un POST amb la versió en un cos JSON. Cal la comprovació de versions de més amunt.
updates-url = Adreça a cridar
updates-url-placeholder = Cap (p. ex. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Desada (escriviu per substituir-la)
updates-auth-placeholder = Cap (p. ex. Bearer my-token)
updates-remove = Elimina
updates-save = Desa
updates-call-now-title = S'executa allò a què estigui connectada
updates-call-now = Crida-la ara
updates-what-calls = Què pot cridar?
updates-watchtower = L'API HTTP de Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , amb{ " " }
updates-watchtower-token = { " " }i el testimoni de{ " " }
updates-watchtower-its = { " " }(amb el punt d'accés{ " " }
updates-watchtower-endpoint = { " " }activat).
updates-portainer = Un webhook de Portainer
updates-portainer-rest = { " " }a la pila o al contenidor de Hoard, amb «re-pull image» activat. El testimoni és a l'URL.
updates-script = Un script a l'amfitrió
updates-script-listener = { " " }darrere d'un receptor de webhooks (com ara{ " " }
updates-script-runs = ) que executi{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }o qualsevol altra cosa que accepti un webhook.
updates-poll-before = O no feu servir l'adreça: un script amb un testimoni d'accés pot consultar{ " " }
updates-poll-until = { " " }fins que{ " " }
updates-poll-and = { " " }estigui definit i{ " " }
updates-poll-after = { " " }sigui true, i llavors baixar la versió nova i reiniciar Hoard.

## MaintenancePanel.vue

maintenance-load-failed = No s'ha pogut carregar el manteniment
maintenance-reanalyze-title = Voleu tornar a analitzar tots els paquets?
maintenance-reanalyze-body = Es torna a llegir cada fitxer, cosa que pot trigar molt en una biblioteca gran; els paquets es mostren en curs fins que els toca. Els vostres noms, etiquetes i notes es mantenen.
maintenance-reanalyze-confirm = Torna a analitzar
maintenance-gc-title = Voleu suprimir els fitxers no utilitzats?
maintenance-gc-body = Els fitxers desats als quals no apunta res de la biblioteca se suprimeixen definitivament. Els paquets de l'arxiu conserven els seus.
maintenance-gc-confirm = Suprimeix-los
maintenance-start-failed = No s'ha pogut començar «{ $task }»
maintenance-just-now = fa un moment
maintenance-minutes-ago = fa { $count } min
maintenance-hours-ago = fa { $count } h
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = S'està carregant…
maintenance-running = S'està executant…
maintenance-starting = S'està començant…
maintenance-run = Executa
maintenance-stop = Atura
maintenance-stopping = S'està aturant…
maintenance-stop-failed = No s'ha pogut aturar «{ $task }»
maintenance-started = { $detail } · començada { $when }
maintenance-starting-detail = Començant
maintenance-finished = Acabada:{ " " }
maintenance-failed = Ha fallat:{ " " }
maintenance-never-run = No s'ha executat mai
maintenance-history = Execucions anteriors
maintenance-col-task = Tasca
maintenance-col-finished = Acabada
maintenance-col-took = Durada
maintenance-col-result = Resultat
maintenance-script-before = Les còpies de seguretat i la comprovació de danys també es poden executar des d'un script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }i{ " " }
maintenance-script-after = . Aquestes execucions també apareixen aquí.

## SettingsPage.vue

settings-title = Configuració
settings-loading = S'està carregant…
settings-find = Troba un paràmetre
settings-find-none = Cap paràmetre coincideix amb «{ $query }».
settings-load-failed = No s'ha pogut carregar la configuració: { $error }
settings-page-appearance = Aparença
settings-page-appearance-blurb = Idioma, temes, mida del text i espaiat, i el que mostren les miniatures i la barra superior. Només es desa en aquest navegador.
settings-page-dragon = Drac
settings-page-dragon-blurb = El tresor del teu drac: cada recurs de la biblioteca n’és una peça.
settings-page-library = Biblioteca
settings-page-library-blurb = Què hi ha a Hoard: com és de gran, de què està fet, com ha crescut i com d'etiquetat està. No es compta el que està arxivat.
settings-page-copies = Còpies gairebé idèntiques
settings-page-copies-blurb = Fitxers que són la mateixa imatge, so o model en una altra mida, freqüència de mostreig o format, primer els que ocupen més espai. La millor de cada conjunt està marcada; no s’esborra res.
copies-loading = Cercant còpies gairebé idèntiques…
copies-refreshing = La biblioteca ha canviat des que es van trobar. Tornant a cercar…
copies-none = No hi ha còpies gairebé idèntiques: cada imatge, so i model només hi és una vegada.
copies-load-failed = No s’han pogut llistar les còpies gairebé idèntiques: { $error }
copies-sum = Conjunts de còpies gairebé idèntiques: { $count }. Les còpies que no són la millor ocupen { $size }.
copies-set = Conjunt { $n }
copies-set-wasted = { $size } en les còpies que no són la millor
copies-more = Mostra’n més (en queden { $count })
settings-page-tags = Etiquetes
settings-page-tags-blurb = Totes les etiquetes de la biblioteca. Fusioneu les que volen dir el mateix, o canvieu-ne el nom d'una; a partir d'aleshores els etiquetadors fan servir el nom nou.
settings-page-store-files = Fitxers de botiga
settings-page-store-files-blurb = Els fitxers que les botigues posen al costat del contingut, que Hoard llegeix per donar nom als paquets i acreditar-ne els autors. Ensenyeu-li els que no coneix, i compartiu el que li heu ensenyat.
settings-page-locations = Ubicacions de la biblioteca
settings-page-locations-blurb = Les carpetes del servidor que contenen la biblioteca: les que Hoard fa servir per desar fitxers, i carpetes de paquets que indexa on són.
settings-page-locations-blurb-desktop = Les carpetes que contenen la biblioteca: les que Hoard fa servir per desar fitxers, i carpetes de paquets que indexa on són.
settings-page-tools = Eines
settings-page-tools-blurb = Programes externs que Hoard fa servir per convertir i llegir fitxers.
settings-page-ai = Etiquetatge amb IA
settings-page-ai-blurb = Un model d'IA posa nom, descriu i etiqueta els paquets nous, a més de les regles integrades. Les seves etiquetes es marquen com a suggerides.
settings-page-search-blurb = Cerqueu imatges pel que mostren i sons per com sonen, no només pel nom, i trobeu coses semblants pel tipus, no només pel color.
settings-page-search-tab = Cerca
settings-page-adding = Addició de paquets
settings-page-adding-blurb = Què fa Hoard amb un paquet quan s'afegeix, a més de llegir-lo i etiquetar-lo.
settings-page-downloads = Baixades
settings-page-downloads-blurb = Com s'anomenen els fitxers del que envia Hoard: baixades, zips, l'eina hoard i els assistents d'IA. Els fitxers de la biblioteca es queden com estan.
settings-page-account = Compte
settings-page-account-blurb = La vostra contrasenya, les altres persones que poden iniciar la sessió i els testimonis d'accés per a scripts i altres programes.
settings-page-auth = Autenticació
settings-page-auth-blurb = Com inicien la sessió les persones: els comptes propis de Hoard, un directori LDAP, Microsoft Active Directory o un proveïdor OAuth, quant duren les sessions i quin rol té cada grup.
settings-page-connection = Connexió
settings-page-connection-blurb = Com arriben a Hoard els navegadors i les aplicacions: l'adreça amb què se'l coneix, i HTTPS, que algunes funcions del navegador necessiten.
settings-page-performance = Rendiment
settings-page-performance-blurb = Com de dur treballa el servidor, com es fan i es conserven les miniatures, i el màxim que accepta.
settings-page-performance-blurb-desktop = Com de dur treballa Hoard, com es fan i es conserven les miniatures, i el màxim que accepta.
settings-page-issues = Incidències
settings-page-issues-blurb = Què ha fallat en obrir o processar fitxers: arxius comprimits que no s'han pogut descomprimir, fitxers que no s'han pogut llegir, tasques que han fallat. Torneu-les a provar un cop resolta la causa, o descarteu-les.
settings-page-maintenance = Manteniment
settings-page-maintenance-blurb = Feu còpies de seguretat, comproveu si hi ha danys i feu neteja. Se n'executa una de cada tipus alhora, i podeu sortir d'aquesta pàgina mentre s'executen.
settings-page-about = Quant a
settings-page-about-blurb = Què és Hoard, quina versió és aquesta i la feina d’altres amb què està fet.
settings-the-default = el valor per defecte
settings-default-on = activat
settings-default-off = desactivat
settings-default-auto = automàtic
settings-option-saved = { $name }: { $value }
settings-back-to-default = Torna al valor per defecte
settings-save = Desa
settings-use = Fes servir
settings-status-off = Desactivat

## SettingsPage.vue: Aparença

settings-themes = Temes
settings-look-light = Clar
settings-look-dark = Fosc
settings-look-group =
    { $scheme ->
        [light] Tema clar
       *[dark] Tema fosc
    }
settings-preview =
    Es mostra el tema { $preview ->
        [light] clar
       *[dark] fosc
    } perquè el pugueu veure. Feu servir el { $used ->
        [light] clar
       *[dark] fosc
    }{ $system ->
        [yes] { " " }(segons el sistema)
       *[other] { "" }
    }, i tornarà quan sortiu d'aquesta pàgina.
settings-preview-back =
    Torna al { $used ->
        [light] clar
       *[dark] fosc
    }
settings-look-hint = Canvieu entre clar i fosc (o seguiu el sistema) des de la barra superior.

## SettingsPage.vue: Eines

settings-blender-what = Converteix models .blend i .abc a glTF en segon pla, perquè tinguin miniatures i previsualització 3D. Els paquets afegits mentre estava desactivat es converteixen quan estigui a punt.
settings-blender-download = Fa servir un Blender instal·lat, o baixa Blender 4.5 LTS (uns 400 MB) d'un mirall de blender.org.
settings-ffmpeg-what = Codifica les baixades en MP3, OGG i Opus, i llegeix l'àudio que Hoard no pot descodificar per si mateix (AAC, M4A, WMA…).
settings-ffmpeg-download = Fa servir un ffmpeg del sistema amb els codificadors adequats, o baixa una versió estàtica (uns 80 MB).
settings-tool-reset = { $tool }: torna al valor per defecte
settings-tool-no-download = Fa servir un { $tool } instal·lat. No hi ha cap versió per baixar per a aquesta màquina, així que instal·leu-lo o indiqueu-ne el camí.
settings-tool-path = Camí del programa { $tool }
settings-mode-off = Desactivat
settings-mode-auto = Automàtic
settings-mode-path = Aquest programa
settings-tool-looking = S'està cercant…
settings-tool-downloading = S'està baixant… el progrés es mostra a l'indicador d'activitat
settings-tool-ready = A punt: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Els instruments amb què es reprodueixen i es converteixen els fitxers MIDI.
settings-soundfont-file = Fitxer SoundFont
settings-soundfont-placeholder = El que es trobi al servidor, o l'integrat
settings-soundfont-placeholder-desktop = El que es trobi en aquest ordinador, o l'integrat
settings-soundfont-default = el que es trobi al servidor, o l'integrat
settings-soundfont-default-desktop = el que es trobi en aquest ordinador, o l'integrat
settings-soundfont-send = Tria un fitxer…
settings-soundfont-sent = Ara el MIDI sona amb { $name }

## SettingsPage.vue: Cerca d'imatges

settings-clip-heading = Cerca pel que mostren les imatges
settings-clip-reset = Cerca d'imatges: torna al valor per defecte
settings-clip-saved-on = Cerca d'imatges: activada
settings-clip-saved-off = Cerca d'imatges: desactivada
settings-clip-what = Escriviu el que busqueu («roca molsosa», «porta de ciència-ficció», «barril de fusta») i les imatges que ho mostren apareixen al costat dels noms que contenen les paraules. «Més com aquest» troba coses del mateix tipus, no només dels mateixos colors.
settings-clip-toggle = Activa la cerca d'imatges
settings-clip-hint = Fa servir el model SigLIP 2 de Google, executat al processador d'aquest servidor: no en surt res. En activar-la, es baixa el model ({ $size } MB) de Hugging Face i després aprèn cada imatge en segon pla amb prioritat baixa. Ocupa de 350 a 750 MB de memòria mentre treballa o algú cerca. En desactivar-la, se suprimeix el model i es conserva el que ha après, per a la propera vegada.
settings-clip-hint-desktop = Fa servir el model SigLIP 2 de Google, executat al processador d'aquest ordinador: no en surt res. En activar-la, es baixa el model ({ $size } MB) de Hugging Face i després aprèn cada imatge en segon pla amb prioritat baixa. Ocupa de 350 a 750 MB de memòria mentre treballa o mentre cerqueu. En desactivar-la, se suprimeix el model i es conserva el que ha après, per a la propera vegada.
settings-clip-looking = S'està cercant el model…
settings-clip-downloading = S'està baixant el model… el progrés es mostra a l'indicador d'activitat
settings-clip-ready-empty = A punt. Les imatges s'aprenen a mesura que s'afegeixen paquets.
settings-clip-ready-all =
    { $count ->
       *[other] A punt: coneix totes les { $count } imatges
    }
settings-clip-learning = A punt: aprenent imatges, { $done } de { $total } fins ara
settings-clip-relearning = Fins que el model nou hagi après totes les imatges, les cerques fan servir el que va aprendre l’antic.
settings-clip-unavailable = No s'ha pogut obtenir el model: { $detail }. Es torna a provar cada hora.
settings-clip-where = S'executa a
settings-clip-here = Aquest servidor: no en surt res
settings-clip-here-desktop = Aquest ordinador: no en surt res
settings-clip-hosted = Un servei allotjat
settings-clip-hosted-warning = Les imatges surten d'aquest servidor. Una miniatura de cada imatge de la biblioteca, i les paraules de cada cerca d'imatges, s'envien a { $service }, que en cobra l'ús i les conserva com diuen les seves condicions.
settings-clip-hosted-warning-desktop = Les imatges surten d'aquest ordinador. Una miniatura de cada imatge de la biblioteca, i les paraules de cada cerca d'imatges, s'envien a { $service }, que en cobra l'ús i les conserva com diuen les seves condicions.
settings-clip-hosted-hint = Aquí no es baixa ni s'executa res: el servei aprèn cada imatge, per lots. Un altre servei o model les torna a aprendre totes, i el que ha après cadascun es conserva, per si hi torneu.
settings-clip-saved-here = Cerca d'imatges: s'executa en aquest servidor
settings-clip-saved-here-desktop = Cerca d'imatges: s'executa en aquest ordinador
settings-clip-saved-hosted = Cerca d'imatges: s'executa a { $service }
settings-clip-hosted-unavailable = No s'ha pogut fer servir { $service }: { $detail }. Es torna a provar cada hora.
settings-clap-heading = Cerca per com sonen els sons
settings-clap-reset = Cerca de sons: torna al valor per defecte
settings-clap-saved-on = Cerca de sons: activada
settings-clap-saved-off = Cerca de sons: desactivada
settings-clap-what = Escriviu el que busqueu («tro llunyà», «bip d'interfície vidriós», «passes sobre grava») i els sons que s'hi assemblen apareixen al costat dels noms que contenen les paraules.
settings-clap-toggle = Activa la cerca de sons
settings-clap-hint = Fa servir el model CLAP de LAION, executat al processador d'aquest servidor: no en surt res. En activar-la, es baixa el model ({ $size } MB) de Hugging Face i després escolta l'inici de cada so en segon pla amb prioritat baixa. Ocupa fins a prop d'1 GB de memòria mentre treballa o algú cerca. En desactivar-la, se suprimeix el model i es conserva el que ha après, per a la propera vegada.
settings-clap-hint-desktop = Fa servir el model CLAP de LAION, executat al processador d'aquest ordinador: no en surt res. En activar-la, es baixa el model ({ $size } MB) de Hugging Face i després escolta l'inici de cada so en segon pla amb prioritat baixa. Ocupa fins a prop d'1 GB de memòria mentre treballa o mentre cerqueu. En desactivar-la, se suprimeix el model i es conserva el que ha après, per a la propera vegada.
settings-clap-ready-empty = A punt. Els sons s'aprenen a mesura que s'afegeixen paquets.
settings-clap-ready-all =
    { $count ->
       *[other] A punt: coneix tots els { $count } sons
    }
settings-clap-learning = A punt: aprenent sons, { $done } de { $total } fins ara

## SettingsPage.vue: Etiquetatge amb IA

settings-ai-service = Servei
settings-ai-off = Desactivat
settings-ai-off-hint = Els paquets només s'etiqueten amb les regles integrades.
settings-ai-local = Model local
settings-ai-local-hint = El servei llm del fitxer compose de Hoard (o cargo xtask llm). Baixeu un model a sota; s'executa en aquesta màquina i no en surt res.
settings-ai-chatgpt = Pla de ChatGPT
settings-ai-chatgpt-hint = Fa servir el vostre pla ChatGPT Go, Plus, Pro o Business en lloc d'una clau d'API.
settings-ai-openrouter-hint = Una sola clau per a centenars de models allotjats, alguns de franc. Els models que accepten imatges estan marcats.
settings-ai-openai-hint = Allotjat per OpenAI.
settings-ai-anthropic-hint = Allotjat per Anthropic, a través del seu punt d'accés compatible amb OpenAI.
settings-ai-gemini-hint = Allotjat per Google, a través del seu punt d'accés compatible amb OpenAI.
settings-ai-ollama-hint = Un servidor Ollama que executeu vosaltres. Canvieu l'adreça si no és un contenidor anomenat ollama.
settings-ai-custom = Un altre (compatible amb OpenAI)
settings-ai-custom-hint = Qualsevol cosa que parli l'API de xat d'OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM…
settings-ai-connecting = S'està connectant…
settings-ai-connected =
    { $count ->
        [one] Connectat · { $count } model
       *[other] Connectat · { $count } models
    }
settings-ai-connected-none = Connectat, però no mostra cap model; escriviu el nom del model
settings-chatgpt-connected-none = Connectat; escriviu el nom del model
settings-ai-takes-images = accepta imatges
settings-ai-saved = Etiquetatge amb IA: { $service }
settings-ai-saved-off = L'etiquetatge amb IA està desactivat
settings-ai-reset = Etiquetatge amb IA: torna al valor per defecte
settings-ai-on = Activat: { $service } · { $model }
settings-ai-url = Adreça
settings-ai-key = Clau d'API
settings-ai-key-saved = Desada (escriviu per substituir-la)
settings-ai-key-paste = Enganxeu la vostra clau
settings-ai-key-optional = Cap (opcional)
settings-ai-key-remove = Elimina
settings-ai-get-key = Obteniu una clau
settings-ai-model = Model
settings-ai-model-pick = Trieu o escriviu un model
settings-ai-model-name = Nom del model
settings-ai-refresh = Actualitza
settings-ai-connect = Connecta
settings-ai-need-key = Afegiu la clau d'API per veure'n els models
settings-ai-vision = Envia també les miniatures (cal un model que accepti imatges)
settings-ai-batch = Per sol·licitud
settings-ai-batch-unit = recursos (menys per a models locals petits, més per a models allotjats grans)

## SettingsPage.vue: inici de sessió de ChatGPT

settings-chatgpt-account = Compte
settings-chatgpt-checking = S'està comprovant…
settings-chatgpt-status-as = Sessió iniciada com a { $email }
settings-chatgpt-status = Sessió iniciada
settings-chatgpt-signed-in-as = Sessió iniciada a ChatGPT com a { $email }
settings-chatgpt-signed-in = Sessió iniciada a ChatGPT
settings-chatgpt-signed-out = Sessió de ChatGPT tancada
settings-chatgpt-continue = Continua amb ChatGPT
settings-chatgpt-sign-out = Tanca la sessió
settings-chatgpt-sign-out-title = Voleu tancar la sessió de ChatGPT?
settings-chatgpt-sign-out-in-use = L'etiquetatge amb IA fa servir el vostre pla de ChatGPT, així que s'aturarà fins que torneu a iniciar la sessió o trieu un altre servei.
settings-chatgpt-sign-out-body = Hoard oblida l'inici de sessió i demana a OpenAI que el tanqui.
settings-chatgpt-manage = Gestiona l'ús
settings-chatgpt-finish-before = Acabeu d'iniciar la sessió a la finestra que s'ha obert (
settings-chatgpt-finish-link = torneu-la a obrir
settings-chatgpt-finish-middle = ). Si acaba en una pàgina que no es carrega, en una adreça que comença per{ " " }
settings-chatgpt-finish-after = , copieu aquesta adreça i enganxeu-la aquí:
settings-chatgpt-paste = L'adreça on ha acabat l'inici de sessió
settings-chatgpt-finish = Acaba
settings-chatgpt-usage = Les sol·licituds d'etiquetatge compten per a l'ús del vostre pla de ChatGPT. Gestiona l'ús mostra el que ha fet servir Hoard i us permet limitar-ho. Per desconnectar Hoard del mateix ChatGPT, aneu a Settings → Security and login → Sign in with ChatGPT a chatgpt.com.

## SettingsPage.vue: models locals

settings-local-downloading-progress = S'està baixant { $percent } % · { $done } de { $total }
settings-local-downloading = S'està baixant…
settings-local-incomplete = La baixada s'ha aturat
settings-local-failed = No es va poder iniciar l'última vegada
settings-local-in-use-running = En ús · en execució
settings-local-in-use = En ús
settings-local-starting = S'està iniciant…
settings-local-running = En execució
settings-local-ready-images = A punt · accepta imatges
settings-local-ready-text = A punt · només text
settings-local-delete-title = Voleu suprimir { $name }?
settings-local-delete-in-use = L'etiquetatge amb IA fa servir aquest model, així que s'aturarà fins que en trieu un altre. Els seus fitxers se suprimeixen del servidor de models.
settings-local-delete-downloading = La baixada s'atura i se suprimeix el que s'hagi baixat fins ara.
settings-local-delete-body = Els seus fitxers se suprimeixen del servidor de models. El podeu tornar a baixar més endavant.
settings-local-delete = Suprimeix
settings-local-deleting = S'està suprimint…
settings-local-cancel = Cancel·la
settings-local-resume = Reprèn
settings-local-download = Baixa
settings-local-start-before = Inicieu-lo amb{ " " }
settings-local-start-or = { " " }(o{ " " }
settings-local-start-developing = { " " }durant el desenvolupament) i després{ " " }
settings-local-start-retry = torneu-ho a provar
settings-local-start-after = . També funciona un servidor que executa un sol model: indiqueu el nom del seu model a sota.
settings-local-list = Al servidor de models
settings-local-none = Encara no s'ha baixat res. Trieu-ne un a sota; es conserva al servidor, i el podeu canviar quan vulgueu.
settings-local-catalog = Models per baixar
settings-local-size = { $size } GB · { $license }
settings-local-other = Un altre model de Hugging Face
settings-local-other-placeholder = Un altre de Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Addició de paquets

settings-store-pages = Pàgines de botiga
settings-store-pages-what = Els paquets nous es cerquen a les botigues en segon pla, perquè tinguin la seva pàgina de botiga, el venedor i la llicència.
settings-store-lookup = Cerca automàticament les pàgines de botiga
settings-unreal = Paquets d'Unreal
settings-unreal-reset = Paquets d'Unreal: torna al valor per defecte
settings-unreal-what = Què s'ha de fer amb els fitxers .uasset.
settings-conv = Regles de conversió
settings-conv-what = Converteix els fitxers que desa Hoard a un sol format, com ara tots els OBJ i FBX a GLB, perquè la biblioteca sigui homogènia. El fitxer convertit ocupa el lloc de l'original i en conserva el nom, les etiquetes, les col·leccions i els enllaços. Els fitxers de les carpetes vigilades no es modifiquen mai.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Activa
settings-conv-to = Converteix a
# Heading over the file formats a rule converts.
settings-conv-from = De
settings-conv-remove = Elimina
settings-conv-remove-label = Elimina la regla { $n }
settings-conv-taken = Una altra regla ja converteix aquest format
settings-conv-upright = Posa els models drets en convertir-los (amb la Y amunt i mirant cap a +Z), tal com es mostren els models del paquet
settings-conv-flac-what = El so queda exactament igual, en més o menys la meitat d’espai, i tota la resta del WAV (punts de bucle, marques de cue, etiquetes) es conserva dins el FLAC, de manera que el WAV es pot recuperar tal com era en qualsevol moment. Els WAV amb mostres de coma flotant o de 32 bits es queden en WAV.
settings-conv-add = Afegeix una regla
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Conserva els originals durant
settings-conv-keep-days =
    { $count ->
        [one] dia
       *[other] dies
    }
settings-conv-keep-what = Fins aleshores, des de la pàgina d'un fitxer convertit es pot restaurar l'original. Les textures no es copien dins d'un model convertit: el model fa referència als fitxers d'imatge que té al costat. Els fitxers .mtl o .bin d'un model es mantenen.
settings-conv-save = Desa les regles
settings-conv-revert = Desfés els canvis
settings-conv-existing = Converteix els fitxers que ja són a la biblioteca
settings-conv-saved = S'han desat les regles de conversió. S'apliquen als paquets que s'afegeixin a partir d'ara.
settings-conv-converting =
    { $count ->
        [one] Convertint un fitxer en segon pla
       *[other] Convertint { $count } fitxers en segon pla
    }
settings-conv-nothing = No cal convertir res de la biblioteca.
settings-conv-ask-title =
    { $count ->
        [one] Voleu convertir un fitxer que ja és a la biblioteca?
       *[other] Voleu convertir { $count } fitxers que ja són a la biblioteca?
    }
settings-conv-ask-body =
    { $packs ->
        [one] És en un paquet.
       *[other] Són en { $packs } paquets.
    } Cada original es conserva { $days } dies i, fins aleshores, es pot restaurar des de la seva pàgina.
settings-conv-ask-busy =
    { $count ->
        [one] N'hi ha un en un paquet que encara s'està afegint; es convertirà quan acabi.
       *[other] N'hi ha { $count } en paquets que encara s'estan afegint; es convertiran quan acabin.
    }
settings-conv-ask-list = Fitxers que es convertirien
settings-conv-ask-more = …i { $count } més
settings-conv-ask-skipped = Es deixen tal com són:
settings-conv-skip-watched =
    { $count ->
        [one] un fitxer en una carpeta vigilada
       *[other] { $count } fitxers en carpetes vigilades
    }
settings-conv-skip-unity =
    { $count ->
        [one] un fitxer que un projecte d'Unity coneix pel nom
       *[other] { $count } fitxers que un projecte d'Unity coneix pel nom
    }
settings-conv-skip-used =
    { $count ->
        [one] un fitxer al qual fa referència una escena o un altre fitxer
       *[other] { $count } fitxers als quals fa referència una escena o un altre fitxer
    }
settings-conv-skip-beside =
    { $count ->
        [one] un fitxer que ja té la còpia convertida al seu paquet
       *[other] { $count } fitxers que ja tenen la còpia convertida al seu paquet
    }
settings-conv-skip-blender =
    { $count ->
        [one] un fitxer que Blender encara no ha llegit (Configuració → Eines)
       *[other] { $count } fitxers que Blender encara no ha llegit (Configuració → Eines)
    }
settings-conv-ask-new = Desa, només paquets nous
settings-conv-ask-save-convert =
    { $count ->
        [one] Desa i converteix-lo
       *[other] Desa i converteix { $count } fitxers
    }
settings-conv-ask-convert =
    { $count ->
        [one] Converteix-lo
       *[other] Converteix { $count } fitxers
    }
settings-unreal-off = Deixa'ls com estan
settings-unreal-off-hint = Convertiu-los des de la pàgina d'un paquet quan vulgueu.
settings-unreal-keep = Converteix i conserva els originals
settings-unreal-keep-hint = Les textures, els sons i els materials surten en formats comuns; el .uasset també es queda.
settings-unreal-replace = Converteix i descarta els originals
settings-unreal-replace-hint = Estalvia espai un cop extret el contingut. Les malles continuen esperant el treballador d'Unreal.

## SettingsPage.vue: Baixades

settings-tn = Noms de textura
settings-tn-saved = S'han desat els noms de textura
settings-tn-reset = Noms de textura: torna al valor per defecte
settings-tn-what = Els mapes d'un material surten amb el nom del material i per a què serveix cada mapa, tinguin el nom que tinguin els fitxers: Rock_Normal.png, no rock_moss_xeokah0_raw_8K_Normal.png. Els fitxers de la biblioteca no es toquen, i un mapa que esmenta un model o un MTL enviat amb ell conserva aquest nom, perquè el model el continuï trobant.
settings-tn-toggle = Anomena els mapes de textura segons el seu material
settings-tn-style = Estil
settings-tn-style-group = Estil de nom
settings-tn-descriptive = Descriptiu
settings-tn-short = Curt
settings-tn-custom = El vostre
settings-tn-pattern = Patró
settings-tn-hint-name = { " " }el material,{ " " }
settings-tn-hint-map = { " " }per a què serveix el mapa,{ " " }
settings-tn-hint-size = { " " }la seva mida quan el joc ve en més d'una (s'afegeix al final si el patró no la inclou).
settings-tn-example = Exemple
settings-tn-suffixes = Sufixos per a cada tipus de mapa
settings-tn-slots = A les baixades GLB, una textura empaquetada dins d'un model sense fitxer propi rep el nom del material i de la ranura que ocupa, i aquestes ranures també tenen sufixos:
settings-tn-endings = Les vostres terminacions
settings-tn-endings-what = Una textura que no és cap dels mapes d'un material conserva el seu nom, tret que acabi en una d'aquestes: _diff → _BC envia rock_diff.png com a rock_BC.png. Deixeu buida la segona per treure la terminació.
settings-tn-ending-from = Terminació
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Canvia per
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Afegeix una terminació
settings-tn-ending-remove = Treu
settings-tn-ending-remove-label = Treu { $ending }

## SettingsPage.vue: General i Connexió

settings-name = Nom
settings-name-what = Com s'anomena aquest servidor a la barra superior, a la pestanya del navegador i a la pàgina d'inici de sessió, perquè si teniu més d'un Hoard pugueu distingir-los.
settings-name-what-desktop = Com s'anomena aquesta biblioteca a la barra superior.
settings-address = Adreça
settings-address-what = L'adreça que fan servir les persones i les aplicacions per arribar a Hoard, quan no és la d'aquest navegador: la d'un servidor intermediari invers, la d'un túnel o un nom de tailnet, com ara https://hoard.example.com. Els enllaços que dona Hoard (a Claude i altres aplicacions, en l'inici de sessió, al seu certificat) la fan servir.
settings-public-url = Adreça pública
settings-public-url-placeholder = De l'adreça del navegador (p. ex. https://hoard.example.com)
settings-public-url-default = de l'adreça del navegador

## ProfilerPanel.vue: el perfilador a dalt de Configuració → Rendiment

profiler = Perfilador
profiler-what = El que ha fet Hoard els darrers deu minuts, actualitzat cada dos segons mentre ho mireu. Només es guarda a la memòria i torna a començar quan Hoard es reinicia.
profiler-starting = Les primeres lectures arriben d'aquí a un moment.
profiler-failed = No s'ha pogut llegir el perfilador: { $error }
profiler-cpu = Ús del processador
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Tota la màquina
profiler-cpu-summary = Hoard fa servir el { $hoard } dels processadors, i tota la màquina, el { $machine }.
profiler-ram = RAM
profiler-ram-peak = Màxim
profiler-ram-of = { $used } de { $total }
profiler-ram-summary = Hoard fa servir { $hoard } de RAM, i n’ha fet servir com a màxim { $peak } en els darrers deu minuts.
profiler-serving = Sol·licituds en curs
profiler-working = Treballant en
profiler-working-none = No s'ha executat res en segon pla.
profiler-working-more =
    { $count ->
        [one] I una més.
       *[other] I { $count } més.
    }
profiler-requests = Temps atenent sol·licituds
profiler-requests-what = Cada barra són deu segons de sol·licituds, amb el temps repartit segons on ha anat.
profiler-part-db = Base de dades
profiler-part-db-title = Esperant la base de dades i executant-ne les consultes
profiler-part-disk = Disc
profiler-part-disk-title = Llegint fitxers desats i comprimint el que s'envia
profiler-part-send = Enviament
profiler-part-send-title = Esperant mentre la xarxa recull el que s'ha enviat
profiler-part-other = Altres
profiler-part-other-title = Elaborant la resposta
profiler-request-count =
    { $count ->
        [one] { $count } sol·licitud
       *[other] { $count } sol·licituds
    }
profiler-requests-none = Cap sol·licitud en els darrers deu minuts.
profiler-slowest = La més lenta: { $route }, { $time }
profiler-routes = Per ruta
profiler-route = Ruta
profiler-count = Sol·licituds
profiler-average = Mitjana
profiler-p95 = El 5 % més lent
profiler-split = On ha anat el temps
profiler-sent = Enviat
profiler-portal = els fitxers de la mateixa interfície

## SettingsPage.vue: Rendiment

settings-render = Miniatures 3D
settings-render-toggle = Renderitza miniatures per als models 3D
settings-gpu-on = A la GPU: { $renderer }
settings-gpu-software = No s'ha trobat cap GPU, així que per programari ({ $renderer })
settings-gpu-none = No s'ha trobat cap GPU ni renderitzador per programari, així que els models no tenen miniatures
settings-thumbs = Miniatures
settings-thumbs-what = Es desen en WebP, comprimides o exactes, el que ocupi menys. Una imatge petita que ja ocupa menys tal com és fa de miniatura d’ella mateixa. Si canvieu això, les miniatures que ja teniu es refan en segon pla.
settings-previews-size = Previsualitzacions: { $size }
settings-thumbs-saved = Miniatures: { $quality }
settings-thumbs-size = Mida
settings-thumbs-size-option = { $px } px
settings-thumbs-quality = Qualitat { $quality }.
settings-thumbs-small = Més petites
settings-thumbs-small-hint = Bé a la graella; una mica borroses de prop.
settings-thumbs-balanced = Equilibrades
settings-thumbs-balanced-hint = Difícils de distingir de l'original a mida de miniatura.
settings-thumbs-sharp = Més nítides
settings-thumbs-sharp-hint = Aproximadament el doble de mida que Equilibrades.
settings-motion-thumbs = Miniatures animades per a les animacions
settings-motion-thumbs-what = Cada animació també té una pel·lícula curta en bucle que la mostra en moviment, quan poses el punter sobre la seva fitxa. Ocupen uns 40 KB cadascuna i triguen una estona a fer-se en una biblioteca gran. Si ho desactives, s'esborren.
settings-motion-fps = Fotogrames per segon
settings-motion-fps-option = { $fps } fps
settings-motion-fps-what = Les animacions més fluides triguen més a fer-se i ocupen més espai: aproximadament el doble a 24 fps que a 12, i encara més a 30. Si el canvies, es tornen a fer totes.
settings-limits = Feina i límits
settings-limits-what = Cadascun es desa quan sortiu del camp o premeu Enter. Les mides accepten unitats: 100 GB, 512 MB, 1,5 TB; un número sol són GB.
settings-workers = Processos en segon pla
settings-workers-unit = tasques alhora (importació, etiquetatge, conversions)
settings-max-upload = Pujada més gran
settings-max-upload-unit = per fitxer
settings-max-expand = Paquet més gran
settings-max-expand-unit = un cop descomprimit
settings-conversion-cache = Baixades convertides
settings-conversion-cache-unit-now = com a màxim, ara { $size }; primer se'n van les que fa més temps que no es fan servir
settings-conversion-cache-unit = com a màxim; primer se'n van les que fa més temps que no es fan servir

## SettingsPage.vue: Manteniment

settings-updates = Actualitzacions
settings-version = Aquesta és la versió { $version } de Hoard.
settings-update-check = Comprova GitHub un cop al dia per si hi ha una versió nova
settings-desktop-updates = Hoard busca una versió nova cada vegada que s’inicia, la baixa en segon pla i la instal·la quan el tanqueu.
settings-backups = Còpies de seguretat programades
settings-backups-what = La base de dades en fa còpies de seguretat tota sola; cada còpia surt a l’historial de sota.
settings-backup-every = Fes una còpia
settings-backup-off = Mai
settings-backup-daily = Cada dia
settings-backup-weekly = Cada setmana
settings-backup-keep = Conserva
settings-backup-keep-unit = còpies programades més recents
settings-backup-copy-to = Copia també a
settings-backup-copy-to-none = Cap altra carpeta
settings-backups-kept = Cada còpia va a la carpeta de còpies com a hoard-auto-…db, i a l’altra carpeta (un altre disc, o una que una aplicació de sincronització desa) si n’hi ha, indicada amb el camí complet al servidor. Només s’eliminen les còpies programades més antigues; les fetes a mà es queden.
about-what = Hoard guarda recursos de jocs (models, textures, sons, música, sprites i més) en una biblioteca on podeu cercar, previsualitzar i agafar el que calgui, compartida amb tothom a qui doneu accés.
about-version = Versió
about-edition = Edició
about-edition-server = Servidor
about-edition-desktop = Aplicació d’escriptori
about-edition-steam = Aplicació d’escriptori de Steam
about-made-by = Fet per
about-source = Codi font a GitHub
about-issues = Informeu d’un problema
about-discord = Xateja amb nosaltres a Discord
about-steam = Hoard a Steam
about-license = Llicència
about-license-what = Hoard és de codi obert amb la llicència MIT No Attribution: feu-lo servir, canvieu-lo i compartiu-lo com vulgueu.
about-downloaded = Baixat quan cal
about-downloaded-what = Hoard no inclou aquests programes. Baixa cadascun la primera vegada que alguna cosa el necessita, i cadascun té la seva pròpia llicència.
about-blender = Converteix models 3D i en fa les miniatures.
about-ffmpeg = Converteix so i vídeo i en fa les previsualitzacions.
about-siglip = Troba imatges pel que hi surt.
about-clap = Troba sons per com sonen.
about-models = Els models que trieu per al servidor d’IA local tenen les llicències dels seus creadors.
about-built-with = Fet amb
about-built-with-what = Les biblioteques de codi obert amb què està fet Hoard, amb les seves llicències.
about-steamworks = La versió de Steam també inclou l’SDK de Steamworks, © Valve Corporation.
about-licenses-loading = S’està carregant la llista…
about-licenses-failed = No s’ha pogut carregar la llista.
about-filter = Filtreu per nom o llicència
about-count = { $shown } de { $total }
about-no-license = Sense llicència indicada
about-report = Informa d’un problema…
report-title = Informa d’un problema
report-what = Això és el que necessita una incidència a GitHub perquè s’hi pugui investigar el problema. No hi ha res privat: se’n treuen els testimonis d’accés, les contrasenyes, les claus, les adreces electròniques i el vostre nom als camins de carpeta. Repasseu-ho abans d’enviar-ho.
report-loading = S’està recollint l’informe…
report-failed = No s’ha pogut recollir l’informe: { $error }
report-log = El que el servidor ha registrat darrerament
report-copy = Copia
report-copied = S’ha copiat l’informe.
report-github = Obre una incidència a GitHub
report-github-hint = La incidència nova s’omple amb això. Si el registre hi és massa llarg, només s’hi posen les darreres línies; Copia ho té tot.
report-close = Tanca

## Asset kinds (api.ts)

api-kind-texture = Textures
api-kind-vector = Art vectorial
api-kind-sheet = Fulls de sprites
api-kind-audio = Àudio
api-kind-model = Models
api-kind-animation = Animacions
api-kind-font = Tipografies
api-kind-shader = Shaders
api-kind-material = Materials
api-kind-video = Vídeo
api-kind-document = Documents
api-kind-data = Dades
api-kind-other = Altres
api-kind-texture-name = textures
api-kind-vector-name = art vectorial
api-kind-sheet-name = fulls de sprites
api-kind-audio-name = àudio
api-kind-model-name = models
api-kind-animation-name = animacions
api-kind-font-name = tipografies
api-kind-shader-name = shaders
api-kind-material-name = materials
api-kind-video-name = vídeo
api-kind-document-name = documents
api-kind-data-name = dades
api-kind-other-name = altres

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

news-button =
    { $count ->
        [one] { $count } novetat
       *[other] { $count } novetats
    }
news-button-title =
    { $count ->
        [one] { $count } novetat al Hoard
       *[other] { $count } novetats al Hoard
    }
news-pip =
    { $count ->
        [one] Nou al Hoard
       *[other] { $count } novetats aquí
    }
news-title = Novetats al Hoard
news-to-find =
    { $count ->
        [one] { $count } per descobrir
       *[other] { $count } per descobrir
    }
news-close = Tanca
news-got-it = Entesos
news-take-me = Porta-m’hi
news-look-again = Torna-ho a veure
news-none = Ara mateix no hi ha res de nou per descobrir.
news-seen-lately = Vist fa poc
news-stop-pointing = Deixa d’assenyalar les novetats
news-point-out = Torna a assenyalar les novetats
news-mark-all = Marca-ho tot com a vist
news-marked-seen =
    { $count ->
        [one] S’ha marcat { $count } novetat com a vista
       *[other] S’han marcat { $count } novetats com a vistes
    }
news-pack-covers-title = Els paquets mostren les seves pròpies imatges
news-pack-covers-body = Un paquet que inclou previsualitzacions o captures de pantalla les fa servir de portada i les va passant mentre hi tens el punter a la llista de paquets, i també al seu panell.
news-models-read-title = Més formats 3D, sense Blender
news-models-read-body = Els models Collada, X3D, VRML, OFF i VRM, les animacions BVH i els splats gaussians ara tenen miniatures, dades i el visor 3D sense Blender, i es poden baixar com a GLB.
news-mesh-explorer-title = Vola entre els models d’una cerca
news-mesh-explorer-body = El botó del cub sobre els resultats que contenen models 3D obre l’explorador de malles: tots els models que ha trobat la cerca, en files sobre un terra infinit per volar-hi entre ells.
news-tag-lookalikes-title = Fusiona etiquetes que s’assemblen
news-tag-lookalikes-body = Les etiquetes que semblen la mateixa, com un plural, paraules juntes, una lletra de menys o una paraula sola i la seva bessona amb espai de noms, es llisten per fusionar-les o marcar-les com a diferents.
news-mcp-everything-title = Les aplicacions d’IA poden fer el que fas tu
news-mcp-everything-body = Claude i altres aplicacions d’IA connectades per MCP ara poden fer tot el que fa l’aplicació web de Hoard, amb exactament els permisos del compte o el testimoni amb què inicien la sessió.
news-themes-title = Deu temes clars i deu de foscos
news-themes-body = Cel, Menta, Albercoc i Llimona s’afegeixen als temes clars, i Mitjanit, Llacuna i Neó als foscos; el nou aspecte s’escampa des de la miniatura que triïs.
news-tiles-move-title = Miniatures sempre en moviment
news-tiles-move-body = Les animacions i les imatges animades de les miniatures de la biblioteca es poden reproduir tota l’estona, no només sota el punter.
news-motion-trails-title = Trajectòries a les animacions
news-motion-trails-body = La miniatura d’una animació traça el camí que fa el seu personatge, amb un punt on és ara mateix.
news-tags-under-title = Una etiqueta troba tot el que té a sota
news-tags-under-body = Escriure una etiqueta al quadre de cerca també troba tot el que té a sota, un signe menys al davant ho deixa tot fora, i el panell d’un recurs mostra les etiquetes més àmplies sota les quals van les seves.
news-motion-fps-title = Fotogrames per segon per a les miniatures mòbils
news-motion-fps-body = Les miniatures de les animacions ara es poden fer a 12, 24 o 30 fotogrames per segon.
news-text-size-title = Mida del text i espaiat
news-text-size-body = Fes la lletra més gran o més petita i la pàgina més espaiosa o més compacta, i tria què mostren les miniatures i la barra superior.
news-tag-tree-title = Etiquetes dins d’etiquetes
news-tag-tree-body = Posa una etiqueta sota una de més àmplia, i en cercar vehicle també trobaràs cotxe i berlina. Niua-les a Configuració → Etiquetes o a la pàgina de cada etiqueta.
news-bulk-edit-title = Edita’n molts alhora
news-bulk-edit-body = Selecciona uns quants recursos i prem E per canviar-ne alhora les etiquetes, la llicència, la valoració i més, amb Desfés.
news-backups-title = Còpies de seguretat programades
news-backups-body = Fes una còpia de seguretat de la base de dades cada dia o cada setmana, conserva les més recents i copia cadascuna a una altra carpeta.
ui-undo = Desfés
ui-today = avui
ui-yesterday = ahir
ui-days-ago =
    { $days ->
        [one] fa { $days } dia
       *[other] fa { $days } dies
    }
ui-notifications = Notificacions
ui-dismiss = Descarta
ui-cancel = Cancel·la
ui-lazy-loading = S'està carregant { $what }…
ui-lazy-failed = No s'ha pogut carregar { $what }.
ui-lazy-reload = Torna a carregar la pàgina

## ResetButton.vue

reset-title = Torna al valor per defecte: { $value }
reset-label = Torna al valor per defecte, { $value }

## SizeInput.vue

size-invalid = Una mida com ara 100 GB, 512 MB o 1,5 TB
size-hint = Una mida com ara 100 GB, 512 MB o 1,5 TB; un número sol són GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = No s'ha pogut carregar la configuració d'autenticació: { $error }
auth-saved = S'ha desat. S'aplica al proper inici de sessió.
auth-method-internal = Interna
auth-method-internal-hint = Noms i contrasenyes que guarda Hoard, afegits a Configuració → Compte.
auth-method-ldap-hint = Un directori comprova el nom i la contrasenya escrits a la pàgina d'inici de sessió: OpenLDAP, FreeIPA, LLDAP o un altre servidor LDAP.
auth-method-ad-hint = Els comptes del domini de Windows inicien la sessió amb el nom i la contrasenya de sempre.
auth-method-oauth-hint = Un botó «Inicia la sessió amb…» que va a Google, Microsoft, GitHub, Keycloak, Authentik o un altre proveïdor i torna.
auth-session-unused = Després de 30 dies sense fer-se servir
auth-session-1h = 1 hora després d'iniciar la sessió
auth-session-8h = 8 hores després d'iniciar la sessió
auth-session-day = Un dia després d'iniciar la sessió
auth-session-week = Una setmana després d'iniciar la sessió
auth-session-30d = 30 dies després d'iniciar la sessió
auth-session-hours =
    { $hours ->
        [one] { $hours } hora després d'iniciar la sessió
       *[other] { $hours } hores després d'iniciar la sessió
    }
auth-role-view = Pot veure
auth-role-edit = Pot canviar
auth-role-admin = Administrador
auth-role-word-view = pot veure
auth-role-word-edit = pot canviar
auth-role-word-admin = administrador
auth-role-word-default = { $role } (el rol per defecte)
auth-tls-starttls-hint = LDAP sense xifrar (port 389) que passa a TLS abans d'enviar res.
auth-tls-ldaps-hint = TLS des del principi (port 636).
auth-tls-none = Cap
auth-tls-none-hint = Les contrasenyes travessen la xarxa tal com són. Només en una xarxa de confiança.
auth-test-failed = No s'ha pogut provar: { $error }
auth-preset-custom = Un altre (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel i altres: indiqueu l'emissor i Descobreix omple les adreces.
auth-preset-google-note = Creeu un client OAuth (Web application) a Google Cloud Console → APIs & Services → Credentials. Google no envia grups, així que doneu els rols a Configuració → Compte, i desactiveu la creació de comptes al primer inici de sessió tret que qualsevol persona amb un compte de Google hagi de poder entrar.
auth-preset-microsoft-note = Registreu una aplicació a Entra ID → App registrations (amb una redirecció Web a l'adreça de sota). Poseu l'ID del vostre inquilí en lloc de «organizations» perquè només hi pugui entrar la vostra organització. Per obtenir els rols dels grups, afegiu el claim groups a Token configuration: llista els ID dels grups.
auth-preset-github-note = Creeu una aplicació OAuth a GitHub → Settings → Developer settings. GitHub no envia grups, així que doneu els rols a Configuració → Compte, i desactiveu la creació de comptes al primer inici de sessió tret que qualsevol persona amb un compte de GitHub hagi de poder entrar.
auth-preset-gitlab-note = Creeu una aplicació a GitLab → Preferences → Applications (o a l'àrea d'administració de la vostra instància: canvieu gitlab.com a les adreces).
auth-discovered = S'han omplert les adreces del proveïdor.
auth-copied = Copiat
auth-copy-failed = No s'ha pogut copiar: seleccioneu-ho i copieu-ho vosaltres
auth-sign-in-with = Inicia la sessió amb
auth-own-accounts-still-work = Els comptes propis de Hoard continuen iniciant la sessió amb les seves contrasenyes, perquè un directori o un proveïdor que no funcioni o estigui mal configurat no us pugui deixar fora.
auth-sessions = Sessions
auth-sign-out-after = Tanca la sessió de les persones
auth-ldap-server = Servidor LDAP
auth-server-type = Tipus de servidor
auth-server-type-custom = Personalitzat
auth-servers = Servidors
auth-server-n = Servidor { $n }
auth-remove-server-n = Elimina el servidor { $n }
auth-remove = Elimina
auth-add-server = Afegeix un servidor
auth-servers-hint = Quan no es pot arribar a un, es prova el següent.
auth-security = Seguretat
auth-connection-security = Seguretat de la connexió
auth-skip-verify = No comprovis el certificat del servidor (només per fer proves)
auth-ca = Autoritat de certificació
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (buit: les que el sistema considera de confiança)
auth-lookup = Cerca de persones
auth-anonymously = De manera anònima
auth-reader-account = Compte de lectura
auth-reader-password = La seva contrasenya
auth-saved-type-to-replace = Desada: escriviu per substituir-la
auth-people-under = Les persones són a
auth-name-attribute = Atribut del nom
auth-only-matching = Només les entrades que coincideixen amb
auth-groups-under = Els grups són a
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (buit: només memberOf)
auth-their-groups = Els seus grups
auth-group-filter-hint = { "{" }dn{ "}" } és l'entrada de la persona, { "{" }user{ "}" } el nom amb què ha iniciat la sessió.
auth-test = Prova
auth-test-hint = Es connecta a cada servidor i inicia la sessió amb el compte de lectura. Amb un nom i una contrasenya, també inicia la sessió com a aquella persona i mostra què en faria Hoard. No es desa res.
auth-test-user = Algú amb qui iniciar la sessió
auth-test-user-placeholder = Algú amb qui iniciar la sessió (opcional)
auth-test-password = La seva contrasenya
auth-testing = S'està provant…
auth-test-connectivity = Prova la connexió
auth-test-server-ok = s'hi ha arribat, i el compte de lectura ha iniciat la sessió
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }ha iniciat la sessió (
auth-test-signed-in-after = ).
auth-test-role = Rol que tindria: { $role }.
auth-test-no-role = No és a cap dels grups indicats a Rols, així que se li denegaria l'accés.
auth-test-groups = Grups: { $groups }
auth-test-no-groups = No s'ha trobat cap grup.
auth-provider = Proveïdor
auth-redirect = Adreça de redirecció
auth-copy = Copia
auth-redirect-hint = Doneu-la al proveïdor quan creeu el client de Hoard. Prové de l'adreça pública de Configuració → Connexió, o de l'adreça que esteu fent servir.
auth-issuer = Emissor
auth-discover = Descobreix
auth-client-id = ID de client
auth-client-secret = Secret de client
auth-authorization-url = Adreça d'autorització
auth-token-url = Adreça del testimoni
auth-userinfo-url = Adreça de la informació d'usuari
auth-userinfo-placeholder = (buit: el testimoni d'identitat diu qui és)
auth-scopes = Àmbits
auth-name-from = Nom de
auth-groups-from = Grups de
auth-groups-from-placeholder = (cap)
auth-button = Botó
auth-button-placeholder = el vostre compte
auth-accounts = Comptes
auth-provision = Crea un compte de Hoard el primer cop que algú inicia la sessió
auth-provision-on-hint = Si no, un administrador afegeix primer cada persona, a Configuració → Compte.
auth-provision-off-hint = Un administrador afegeix primer cada persona, a Configuració → Compte, i tria com inicia la sessió.
auth-open-to-anyone = Qualsevol persona amb un compte de { $provider } hi pot entrar. Afegiu grups de lectors a sota, o desactiveu-ho i afegiu les persones vosaltres.
auth-new-accounts = Comptes nous
auth-roles-from-groups = Rols a partir de grups
auth-roles-hint = Els grups els membres dels quals obtenen cada rol, un per línia, pel nom o el DN. Guanya el més alt que coincideixi, i es comprova a cada inici de sessió. Si no n'hi ha cap, els comptes nous obtenen el rol de més amunt i un administrador el canvia després. Si hi ha grups a Pot veure, només hi poden iniciar la sessió els membres d'un grup de la llista.
auth-roles-hint-oauth = Els grups els membres dels quals obtenen cada rol, un per línia, pel nom o el DN (o el que llisti el claim de grups). Guanya el més alt que coincideixi, i es comprova a cada inici de sessió. Si no n'hi ha cap, els comptes nous obtenen el rol de més amunt i un administrador el canvia després. Si hi ha grups a Pot veure, només hi poden iniciar la sessió els membres d'un grup de la llista.
auth-saving = S'està desant…
auth-save = Desa

## HttpsPanel.vue (Settings → Connection)

https-load-failed = No s'ha pogut carregar HTTPS: { $error }
https-off-redirecting = HTTPS està desactivat. Us portem a http://
https-now-on = HTTPS està activat
https-saved = S'ha desat. Les connexions noves fan servir el certificat nou.
https-now-off = HTTPS està desactivat
https-http3-failed-toast = No s'ha pogut iniciar HTTP/3: { $error }
https-copied = Copiat
https-copy-failed = No s'ha pogut copiar: seleccioneu-ho i copieu-ho vosaltres
https-mode-off = Desactivat
https-mode-off-hint = Només http:// sense xifrar.
https-mode-own = El certificat propi de Hoard
https-mode-own-hint = Hoard en crea un per a les adreces d'aquest servidor. Instal·leu l'autoritat de certificació de Hoard un cop a cada dispositiu i els seus navegadors hi confiaran.
https-mode-custom = El meu certificat
https-mode-custom-hint = Un que ja tingueu, p. ex. de tailscale cert o de Let’s Encrypt. Els navegadors hi confien sense instal·lar res.
https-untrusted-own = Aquest navegador encara no hi confia per a { $host }: instal·leu l'autoritat de certificació de Hoard de sota. Veureu un avís que podeu ometre.
https-untrusted-uncovered = Aquest navegador encara no hi confia per a { $host }: el certificat no cobreix aquesta adreça. Veureu un avís que podeu ometre.
https-untrusted = Aquest navegador encara no hi confia per a { $host }. Veureu un avís que podeu ometre.
https-now = Ara
https-start-failed = HTTPS està configurat però no s'ha pogut iniciar: { $error }
https-on-port = HTTPS està activat, al port { $port } com http://.
https-using-it = Ara l'esteu fent servir.
https-open = Obre { $url }
https-trusted = Aquest navegador hi confia.
https-cert-for-before = Certificat per a{ " " }
https-cert-for-after = , fins al { $date }.
https-cert-for-after-own = , fins al { $date } (es renova sol).
https-not-covered-before = No cobreix{ " " }
https-not-covered-after = , l'adreça que esteu fent servir.
https-is-off = HTTPS està desactivat: les pàgines se serveixen per http:// sense xifrar.
https-addresses = Adreces
https-addresses-hint = El certificat cobreix les adreces que ha trobat Hoard i les que hi afegiu (un nom que li doni l'encaminador, un nom de Tailscale).
https-remove-name = Elimina { $name }
https-another-address = Una altra adreça
https-another-address-placeholder = Una altra adreça, p. ex. nas.local
https-add = Afegeix
https-your-cert = El vostre certificat
https-saved-cert-before = Desat:{ " " }
https-saved-cert-after = { " " }emès per { $issuer }, fins al { $date }. Enganxeu-ne un de nou per substituir-lo.
https-pem-before = El certificat (amb la seva cadena) i la seva clau privada, com a text PEM: els fitxers que comencen per{ " " }
https-pem-after = .
https-certificate = Certificat
https-choose-file = Tria un fitxer…
https-private-key = Clau privada
https-checked-before = Per a{ " " }
https-checked-after = , emès per { $issuer }, fins al { $date }.
https-checked-after-days =
    , emès per { $issuer }, fins al { $date } ({ $days ->
        [one] { $days } dia
       *[other] { $days } dies
    }).
https-self-signed = És autosignat: els navegadors n'avisaran.
https-key-matches = La clau coincideix.
https-where-to-get = On obtenir-ne un
https-get-tailscale-before = activeu HTTPS a la vostra tailnet, després executeu{ " " }
https-get-tailscale-after = { " " }al servidor i pugeu els dos fitxers. Duren 90 dies: torneu-lo a executar i torneu-los a pujar abans que caduquin.
https-get-acme-title = Let’s Encrypt o un altre client ACME
https-get-acme-before = (certbot, acme.sh, lego) per a un domini vostre: pugeu{ " " }
https-get-acme-and = { " " }i{ " " }
https-get-acme-after = .
https-get-proxy-title = Un servidor intermediari invers
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) que tingui el certificat: deixeu HTTPS desactivat aquí i feu que el servidor intermediari enviï http:// sense xifrar a Hoard.
https-trying = S'està provant…
https-apply = Aplica
https-apply-hint = Hoard prova primer la configuració nova, i manté la que té si no funciona.
https-trust-title = Confieu en Hoard als vostres dispositius
https-download-ca = Baixa hoard-ca.crt
https-trust-hint = Instal·leu l'autoritat de certificació de Hoard un cop a cada dispositiu, i els seus navegadors confiaran en aquest servidor (i en qualsevol certificat que Hoard en creï més endavant). Només pot respondre per aquest Hoard. Empremta digital (SHA-256):
https-ca-windows = Obriu el fitxer → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Reinicieu el navegador.
https-ca-macos = Obriu el fitxer; s'afegeix a Keychain Access. Feu-hi doble clic a «{ $subject }» → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone i iPad
https-ca-ios = Baixeu-lo a Safari i permeteu-ho. Settings → Profile Downloaded → Install. Després Settings → General → About → Certificate Trust Settings → activeu «{ $subject }».
https-ca-android = Settings → Security → More security settings → Encryption & credentials → Install a certificate → CA certificate, i trieu el fitxer. (Els noms varien una mica d'un telèfon a un altre; cerqueu «certificat» a la configuració.)
https-ca-linux-1 = Debian i Ubuntu:{ " " }
https-ca-linux-2 = . Fedora i Arch:{ " " }
https-ca-linux-3 = . Chrome a Linux té la seva pròpia llista: Settings → Privacy and security → Security → Manage certificates → Import.
https-ca-firefox = Firefox té la seva pròpia llista: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import, i marqueu «Trust this CA to identify websites».
https-ca-cli-title = L'eina de línia d'ordres hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , o poseu el camí a{ " " }
https-ca-cli-after = .
https-options = Opcions
https-redirect = Envia els navegadors d'altres dispositius de http:// a https://
https-redirect-hint = Només es redirigeixen les càrregues de pàgina. L'eina de línia d'ordres hoard, el treballador d'Unreal, els scripts i les aplicacions d'IA continuen funcionant per http://, igual que http://localhost al mateix servidor.
https-http3-hint-before = Més ràpid amb una Wi-Fi inestable i als telèfons. Cal HTTPS, i el port UDP { $port } obert a més del TCP (a Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = En execució.
https-http3-waits = S'inicia quan HTTPS està activat.
https-http3-failed = No s'ha pogut iniciar HTTP/3: { $error }
https-lockout-title = Si us quedeu sense accés
https-lockout-1 = { "" }
https-lockout-2 = { " " }al mateix servidor sempre funciona per http:// sense xifrar. O bé desactiveu HTTPS des de la línia d'ordres del servidor i torneu a iniciar Hoard:{ " " }
https-lockout-3 = { " " }(a Docker:{ " " }
https-lockout-4 = , i després{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Pot veure
account-role-view-hint = Cercar, previsualitzar i baixar
account-role-edit = Pot canviar
account-role-edit-hint = També afegir, canviar, etiquetar, agrupar en col·leccions i arxivar
account-role-admin = Administrador
account-role-admin-hint = També la configuració, els comptes, el manteniment i suprimir definitivament
account-source-internal = Una contrasenya de Hoard
account-source-ldap = La seva contrasenya de { $directory }
account-source-oauth = Iniciant la sessió amb { $provider }
account-source-word-directory = directori
account-load-failed = No s'han pogut carregar els comptes: { $error }
account-failed = { $what }: { $error }
account-password-failed = No s'ha pogut canviar la contrasenya
account-password-changed = S'ha canviat la contrasenya. S'ha tancat la sessió dels vostres altres dispositius.
account-add-failed = No s'ha pogut afegir { $name }
account-added = { $name } ja pot iniciar la sessió.
account-role-failed = No s'ha pogut canviar el que pot fer { $name }
account-remove-title = Voleu eliminar { $name }?
account-remove-body = Se li tanca la sessió a tot arreu i no la pot tornar a iniciar. No s'elimina res del que hagi afegit a la biblioteca.
account-remove = Elimina
account-remove-failed = No s'ha pogut eliminar { $name }
account-token-failed = No s'ha pogut crear el testimoni
account-copied = Copiat
account-copy-token-failed = No s'ha pogut copiar: seleccioneu el testimoni i copieu-lo vosaltres
account-copy-failed = No s'ha pogut copiar: seleccioneu-ho i copieu-ho vosaltres
account-revoke-title = Voleu revocar «{ $name }»?
account-revoke-body = Tot el que encara el faci servir serà rebutjat a partir d'ara.
account-revoke = Revoca
account-revoke-failed = No s'ha pogut revocar «{ $name }»
account-disconnect-title = Voleu desconnectar { $app }?
account-disconnect-body = Serà rebutjada a partir d'ara, i s'haurà de tornar a connectar per fer servir Hoard.
account-disconnect = Desconnecta
account-disconnect-failed = No s'ha pogut desconnectar { $app }
account-guest-failed = No s'ha pogut canviar
account-never = mai
account-you = Vós
account-sign-out = Tanca la sessió
account-signed-in-as-before = Sessió iniciada com a{ " " }
account-signed-in-as-admin = , que és administrador.
account-signed-in-as-edit = , que pot canviar la biblioteca.
account-signed-in-as-view = , que pot veure-ho tot però no canviar res.
account-app-signin = L’aplicació Hoard inicia la sessió per vosaltres, així que no hi ha contrasenya. Creeu un testimoni d’accés a continuació per als connectors dels motors i l’ordre hoard.
account-password-at-provider = La vostra contrasenya la guarda el proveïdor amb què inicieu la sessió, així que l'heu de canviar allà.
account-password-at-directory = La vostra contrasenya la guarda el directori amb què inicieu la sessió, així que l'heu de canviar allà.
account-current-password = Contrasenya actual
account-new-password = Contrasenya nova
account-new-password-again = Torneu a escriure la contrasenya nova
account-change-password = Canvia la contrasenya
account-password-mismatch = Les contrasenyes noves no coincideixen.
account-password-rule = Com a mínim 8 caràcters.
account-people = Persones
account-people-hint = Cada persona pot veure (cercar, previsualitzar i baixar), també canviar la biblioteca (afegir, editar, etiquetar, agrupar en col·leccions i arxivar), o a més ser administradora (la configuració, els comptes, el manteniment i suprimir definitivament).
account-you-tag = (vós)
account-signs-in-oauth = Inicia la sessió amb el proveïdor OAuth
account-signs-in-directory = Inicia la sessió amb el directori
account-since = des del { $date }
account-what-can-do = Què pot fer { $name }
account-name = Nom
account-how-sign-in = Com inicia la sessió
account-password-label = Contrasenya (com a mínim 8 caràcters)
account-password-placeholder = Contrasenya (8 caràcters o més)
account-what-they-can-do = Què pot fer
account-add = Afegeix
account-ai-apps = Claude i altres aplicacions d'IA
account-ai-apps-hint = Connecteu Claude (o qualsevol aplicació MCP) i podrà cercar i obtenir recursos per vosaltres, i descriure i etiquetar el que Hoard encara no ha fet. Inicia la sessió a Hoard el primer cop; li doneu permís aquí, al navegador.
account-mcp-address = Adreça MCP de Hoard
account-copy = Copia
account-claude-code-1 = executeu{ " " }
account-claude-code-2 = , després{ " " }
account-claude-code-3 = { " " }per iniciar la sessió.{ " " }
account-claude-code-4 = { " " }descriu un paquet.
account-claude-ai = Settings → Connectors → Add custom connector, enganxeu l'adreça i després Connect. Es connecten des dels servidors d'Anthropic, així que Hoard ha de ser accessible des d'internet per HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, enganxeu l'adreça i després Connect. Es connecten des dels servidors d'Anthropic, així que Hoard ha de ser accessible des d'internet per HTTPS (aquesta adreça no ho és: definiu l'adreça pública a Configuració → Connexió).
account-connection-meta = per a { $user } · connexió: { $since } · últim ús: { $used }
account-no-apps = Encara no hi ha cap aplicació connectada.
account-live = Fan servir Hoard ara
account-live-hint = Aplicacions que han demanat alguna cosa a Hoard els darrers 10 minuts. S’actualitza tot sol.
account-live-none = Res no ha demanat res a Hoard els darrers 10 minuts.
account-live-last = darrer: { $what } · { $when } · peticions: { $calls }
account-tokens = Testimonis d'accés
account-tokens-hint-1 = Per a l'eina de línia d'ordres{ " " }
account-tokens-hint-2 = , els scripts, els clients MCP i el treballador d'Unreal: l'envien com a{ " " }
account-tokens-hint-3 = . Un testimoni pot fer el que trieu, fins al que podeu fer vosaltres, però mai no pot gestionar comptes ni testimonis. N'hi ha prou amb un que només pugui veure per cercar i baixar, i per sincronitzar un projecte.
account-tokens-admin = Veieu els testimonis i les aplicacions connectades de tothom; els altres veuen els seus.
account-made-before = { "" }
account-made-after = : copieu-lo ara. No es tornarà a mostrar.
account-new-token = El testimoni nou
account-cli-1 = Per a la CLI:{ " " }
account-cli-2 = , o poseu-lo a{ " " }
account-cli-3 = .
account-done = Fet
account-token-meta = creat: { $made } · últim ús: { $used }
account-token-meta-by = creat per { $by }: { $made } · últim ús: { $used }
account-no-tokens = Encara no hi ha cap testimoni.
account-token-name = Què el farà servir
account-token-name-placeholder = Què el farà servir, p. ex. «Treballador d'Unreal»
account-token-role = Què pot fer
account-make-token = Crea un testimoni
account-guests = Convidats
account-guests-allow = Permet que qualsevol que pugui arribar al servidor navegui sense iniciar la sessió
account-guests-hint = Poden cercar, previsualitzar i baixar, però no afegir, canviar ni suprimir res, ni veure la configuració.

## SignIn.vue

signin-create-title = Creeu el vostre compte
signin-title = Inicia la sessió
signin-setup-lede = Aquest servidor encara no té cap compte. El que creeu aquí ho pot fer tot; més endavant podeu afegir comptes per a altres persones des de Configuració → Compte.
signin-overlay-lede = Aquí tothom pot navegar, però per fer canvis cal un compte.
signin-with = Inicia la sessió amb { $provider }
signin-or-password = o amb un nom i una contrasenya
signin-username = Nom d'usuari
signin-name = Nom
signin-new-password = Contrasenya nova
signin-password = Contrasenya
signin-password-rule = Com a mínim 8 caràcters.
signin-confirm-password = Confirmeu la contrasenya
signin-mismatch = Les dues contrasenyes no coincideixen.
signin-setup-code = Codi de configuració
signin-setup-code-hint-before = El codi que es va donar a aquest servidor a{ " " }
signin-setup-code-hint-after = .
signin-creating = S'està creant…
signin-signing-in = S'està iniciant la sessió…
signin-create = Crea el compte
signin-submit = Inicia la sessió
signin-keep-browsing = Continua navegant
signin-directory-hint = Feu servir el vostre nom i la vostra contrasenya de { $directory }.
signin-forgot-1 = Heu oblidat una contrasenya de Hoard? Al servidor, executeu{ " " }
signin-forgot-2 = { " " }(a Docker,{ " " }
signin-forgot-3 = ) per obtenir-ne una de nova.

## SetupWizard.vue (first visit to a new server)

setup-title = On voleu tenir la biblioteca?
setup-lede = Trieu una carpeta del servidor on Hoard guardi els vostres paquets, o indiqueu-li una carpeta de paquets que ja tingueu. Més endavant podeu afegir més ubicacions, en altres discos, des de Configuració.
setup-lede-desktop = Trieu una carpeta on Hoard guardi els vostres paquets, o indiqueu-li una carpeta de paquets que ja tingueu. Més endavant podeu afegir més ubicacions, en altres discos, des de Configuració.
setup-unreachable = No s'ha pogut connectar amb el servidor: { $error }
setup-unreachable-desktop = Hoard no respon: { $error }
setup-try-again = Torna-ho a provar
setup-loading = S'està carregant…
setup-start = Comença la biblioteca
setup-ready = La vostra biblioteca està a punt
setup-managed-before = Els paquets nous van a{ " " }
setup-managed-after = . Deixeu anar fitxers, carpetes o arxius comprimits a qualsevol lloc de la pàgina per afegir els primers.
setup-watched-before = Hoard està vigilant{ " " }
setup-watched-after = ; els paquets apareixen a mesura que els troba. Per poder-ne pujar també, afegiu una biblioteca que guardi Hoard des de Configuració → Ubicacions de la biblioteca.
setup-open = Obre la biblioteca

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = Voleu connectar { $app }?
connect-lede-1 = { " " }podrà cercar, baixar, descriure i etiquetar tot el que hi ha en aquest Hoard, actuant com a{ " " }
connect-lede-2 = . No pot gestionar comptes ni testimonis d'accés.
connect-back-before = Tornareu a{ " " }
connect-back-after = . Podeu desconnectar-la quan vulgueu a Configuració → Compte.
connect-allow = Permet
connect-cancel = Cancel·la
connect-failed = No es pot connectar aquesta aplicació
connect-go-library = Ves a la biblioteca
connect-checking = S'està comprovant…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = Això és http:// sense xifrar, així que algunes coses necessiten HTTPS: el 3D amb WebGPU, el porta-retalls modern, instal·lar Hoard com a aplicació i els connectors de Claude.
insecure-switch = Canvia a https://
insecure-set-up = Configura HTTPS
insecure-dismiss = No ho tornis a mostrar

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Aquest fitxer és massa gran per llegir-lo com a fitxer de botiga.
formatwizard-not-json = Aquest fitxer no és JSON vàlid, així que es mostra com a text.
formatwizard-read-failed = No s'ha pogut llegir el fitxer: { $error }
formatwizard-back-title = Voleu tornar al formulari?
formatwizard-back-body = Els canvis fets al TOML es perden: el formulari torna a escriure el format a partir del que té.
formatwizard-back-to-form = Torna al formulari
formatwizard-miss-name = Els noms de fitxer que admet no inclouen { $file }. Afegiu-lo a Fitxers.
formatwizard-miss-not-the-stores = Allò pel qual reconeix els fitxers de la botiga no és en aquest fitxer. Canvieu «Es reconeix per».
formatwizard-miss-no-title = No troba cap títol en aquest fitxer. Trieu el títol.
formatwizard-miss-short-name = no és el seu nom
formatwizard-miss-short-not-the-stores = no és de la botiga
formatwizard-miss-short-no-title = sense títol
formatwizard-saved-packs =
    S'ha desat { $file }. { $packs ->
        [one] S'està tornant a llegir { $packs } paquet.
       *[other] S'estan tornant a llegir { $packs } paquets.
    }
formatwizard-saved = S'ha desat { $file }.
formatwizard-save-failed = No s'ha pogut desar el format: { $error }
formatwizard-zip-failed = No s'ha pogut crear el zip: { $error }
formatwizard-stop-title = Voleu deixar d'ensenyar aquest fitxer?
formatwizard-stop-body = El format no s'ha desat.
formatwizard-stop = Atura
formatwizard-label = Assistent de formats
formatwizard-changing = S'està canviant { $name }
formatwizard-starting-from = A partir de { $name }
formatwizard-title = Ensenyeu a Hoard un fitxer de botiga
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }a{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Edita com a TOML
formatwizard-file-reference = El fitxer, com a referència.
formatwizard-click-json = Feu clic al fitxer on hi ha { $field }.
formatwizard-click-text = Feu clic al fitxer on hi ha { $field }, o seleccioneu-ho.
formatwizard-pick-first = Trieu un camp a la dreta i després feu clic aquí al seu valor.
formatwizard-loading = S'està carregant…
formatwizard-the-format = El format
formatwizard-syntax-before = La sintaxi és a{ " " }
formatwizard-syntax-after = .
formatwizard-store = Botiga
formatwizard-store-placeholder = El nom de la botiga, com ara Kenney
formatwizard-files = Fitxers
formatwizard-recognize = Es reconeix per
formatwizard-recognize-json = Una clau que només tenen els fitxers de la botiga
formatwizard-recognize-text = Una línia que només tenen els fitxers de la botiga
formatwizard-recognize-hint = Una manera per línia; n'hi ha prou amb qualsevol. Cal quan altres fitxers tenen el mateix nom.
formatwizard-seller = Venedor
formatwizard-seller-placeholder = Quan el fitxer no ho diu
formatwizard-tags = Etiquetes
formatwizard-tags-placeholder = Per a tot el que descriu, separades per comes
formatwizard-fields = Camps
formatwizard-selector = Selector del camp { $field }
formatwizard-clear = Esborra el camp { $field }
formatwizard-pick = Tria
formatwizard-what-it-reads = Què llegeix
formatwizard-read-title = Títol
formatwizard-read-seller = Venedor
formatwizard-read-license = Llicència
formatwizard-read-url = Pàgina de botiga
formatwizard-read-kind = Tipus
formatwizard-read-tags = Etiquetes
formatwizard-read-origin = Lloc de creació
formatwizard-read-size = Mida
formatwizard-also = { $format } també llegeix aquest fitxer. Aquest format es provaria primer.
formatwizard-library =
    Tenen un nom que admet: { $files ->
        [one] { $files } fitxer
       *[other] { $files } fitxers
    } de { $packs ->
        [one] { $packs } paquet
       *[other] { $packs } paquets
    }. Dels { $tried } provats, en llegeix { $read }.
formatwizard-library-none = No hi ha cap fitxer a la biblioteca amb un nom que admeti.
formatwizard-share = Compartiu-lo
formatwizard-share-hint = Un zip del format, amb aquest fitxer com a mostra i el que se'n llegeix, organitzat com el repositori de Hoard, per enviar-lo i que tothom el tingui. Abans, traieu de la mostra tot el que no s'hagi de compartir.
formatwizard-sample = Mostra per compartir
formatwizard-download = Baixa { $file }
formatwizard-close = Tanca
formatwizard-use = Fes-lo servir aquí
formatwizard-cancel = Cancel·la
settings-unreal-editor-what = Instal·la Unreal Editor per al procés d’exportació. Executeu-lo per separat per exportar recursos.
settings-unreal-download = Baixa l’editor Linux x86-64 d’Epic, una sola vegada, a la carpeta d’eines de Hoard. Ocupa desenes de GB al disc i requereix un compte de GitHub que Epic hagi admès.
settings-unreal-github-step-link-go = Com vincular-los
settings-unreal-github-step-token = Creeu un testimoni de GitHub (classic) amb només read:packages marcat.
settings-unreal-github-step-token-go = Crea un testimoni
settings-unreal-github-user = Nom d’usuari de GitHub
settings-unreal-github-token = Testimoni
settings-unreal-github-token-kept = Desat (escriviu-ne un de nou per substituir-lo)
settings-unreal-github-save = Desa l’inici de sessió
settings-unreal-github-forget = Desconnecta
settings-unreal-github-saved = S’ha desat l’inici de sessió a GitHub; es torna a provar la baixada
settings-unreal-github-forgotten = GitHub desconnectat
settings-unreal-github-from-server = Aquest servidor també té un inici de sessió propi (GHCR_TOKEN_FILE o DOCKER_CONFIG); s’utilitza quan no n’hi ha cap de desat aquí.
settings-unreal-github-connect = Connecta GitHub…
settings-unreal-github-connect-title = Connecta GitHub
settings-unreal-github-connect-why = Epic comparteix l’Unreal Editor amb comptes de GitHub vinculats a Epic Games. Hoard només demana llegir paquets.
settings-unreal-github-sign-in = Inicia la sessió amb GitHub
settings-unreal-github-enter-code = En fer clic, GitHub s’obre en una pestanya nova. Introduïu-hi aquest codi.
settings-unreal-github-waiting = S’està esperant GitHub…
settings-unreal-github-expired = Aquest codi ha caducat.
settings-unreal-github-denied = L’inici de sessió s’ha cancel·lat a GitHub.
settings-unreal-github-failed = No s’ha pogut obtenir un codi de GitHub.
settings-unreal-github-again = Obtén un codi nou
settings-unreal-github-before = Abans de començar
settings-unreal-github-before-what = Vinculeu GitHub al vostre compte d’Epic Games i accepteu la invitació d’EpicGames que GitHub us envia per correu.
settings-unreal-github-use-token = Fes servir un testimoni
settings-unreal-github-use-sign-in = Inicia la sessió amb GitHub
settings-unreal-github-no-password = Cap contrasenya arriba a Hoard
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = Connectat com a { $user }; es torna a provar la baixada
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = Connectat com a { $user }
settings-unreal-github-as-what = L’Unreal Editor es baixa amb aquest compte de GitHub.
settings-unreal-github-none = GitHub no està connectat
settings-unreal-github-none-what = Cal per baixar l’editor.
settings-unreal-github-change = Canvia…
settings-unreal-image = Versió o imatge d’Unreal
settings-editor-path = Introduïu el camí de l’executable de { $tool } instal·lat. La instal·lació automàtica només està disponible a Docker.
error-unreal-image = Introduïu una versió d’Unreal 5, una etiqueta de versió dev/dev-slim d’Epic o una referència d’imatge SHA-256 d’Epic.
error-github-no-app = Aquest servidor no té cap aplicació de GitHub per iniciar la sessió; feu servir un testimoni.
error-github-unreachable = No es pot connectar amb GitHub.
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub ha rebutjat l’inici de sessió: { $reason }
