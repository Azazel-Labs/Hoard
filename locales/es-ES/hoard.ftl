## Errores del servidor

error-account-not-found = no se ha encontrado la cuenta
error-asset-gone = no se ha encontrado el recurso
error-asset-not-found = no se ha encontrado el recurso { $id }
error-blob-not-found = no se ha encontrado el blob { $hash }
error-collection-not-found = no se ha encontrado la colección { $id }
error-connection-not-found = no se ha encontrado la conexión
error-derivative-not-found = no se ha encontrado el derivado { $kind } de { $id }
error-file-in-pack-not-found = no se ha encontrado { $path } en el paquete { $id }
error-site-not-a-page = solo las páginas web se abren como páginas
error-site-link-expired = la dirección de esta página ha caducado; vuelve a abrirla desde Hoard
error-format-not-found = no se ha encontrado el formato { $file }
error-frame-sheet-aseprite-only = solo los archivos de Aseprite tienen hoja de fotogramas
error-frame-sheet-frames = tiene un solo fotograma, o demasiados para colocarlos
error-location-not-found = no se ha encontrado la ubicación { $id }
error-maintenance-task-not-found = no se ha encontrado la tarea de mantenimiento { $id }
error-open-upload-not-found = no se ha encontrado la subida abierta { $id }
error-original-not-found = no se ha encontrado el archivo original de { $id }
error-pack-not-found = no se ha encontrado el paquete { $id }
error-preview-too-big = esta imagen es demasiado grande para hacer una vista previa ligera
error-project-not-found = no se ha encontrado el proyecto { $name }
error-saved-search-not-found = no se ha encontrado la búsqueda guardada { $id }
error-sign-in-first = inicia sesión primero (o envía un token de acceso)
error-those-assets-not-found = no se han encontrado esos recursos
error-thumbnail-not-found = no se ha encontrado la miniatura de { $id }
error-token-not-found = no se ha encontrado el token
error-upload-not-found = no se ha encontrado la subida { $id }
error-pack-needs-name = un paquete necesita un nombre
error-no-library = todavía no hay ninguna ubicación de la biblioteca donde guardar archivos: añade una gestionada en Ajustes → Ubicaciones de la biblioteca
error-upload-failed = la subida ha fallado: { $reason }
error-upload-no-files = la subida no tiene archivos
error-analyze-nothing-to-do = no hay nada que hacer: rules=false y el LLM está desactivado
error-too-many-assets = demasiados recursos en una sola petición
error-too-many-items = demasiados elementos en una sola petición
error-not-a-hash = no es un hash blake3 en hexadecimal
error-merge-into-tag = into debe ser una etiqueta
error-nothing-to-merge = no hay nada que fusionar
error-selection-size = indica entre 1 y { $max } ids de recursos
error-collection-needs-name = una colección necesita un nombre
error-collection-exists = ya hay una colección llamada { $name }
error-collection-empty = la colección { $name } está vacía
error-conversion-failed = no se ha podido convertir a { $format }: { $reason }
error-max-size = max debe ser una potencia de dos entre 64 y 16384
error-replace-ext = ext debe ser una extensión de archivo corta
error-empty-file = archivo vacío
error-imitation-silent = Nada en la grabación destaca sobre el fondo. Vuelve a intentarlo más cerca del micrófono.
error-replace-duplicate = el archivo convertido duplica uno que ya está en el paquete; el paquete se ha fusionado con él
error-ai-address = La dirección de la IA debe empezar por http:// o https://
error-hosted-key = Introduce la clave de API de { $provider }
error-hosted-address = La dirección del servicio de búsqueda de imágenes debe empezar por http:// o https://
error-local-llm-action = action es list, download o delete, y las dos últimas necesitan un modelo
error-role = Un rol es view, edit o admin
error-admin-only = solo un administrador puede hacer esto
error-view-only = esta cuenta puede mirar, pero no cambiar nada: un administrador puede cambiarlo en Ajustes → Cuenta
error-portal-only = inicia sesión en el portal para gestionar cuentas y tokens
error-not-from-portal = esta petición no viene del portal de Hoard
error-account-name = Elige un nombre de hasta 64 caracteres
error-password-length = Usa una contraseña de al menos { $min } caracteres
error-too-many-codes = Demasiados códigos incorrectos: espera { $seconds } segundos
error-already-set-up = Ya hay una cuenta: inicia sesión
error-too-many-tries = Demasiados intentos fallidos: espera { $seconds } segundos
error-directory-unavailable = El directorio no ha podido comprobar tu contraseña: { $reason }
error-password-elsewhere = Tu contraseña la guarda el directorio o el proveedor con el que inicias sesión
error-current-password = Tu contraseña actual no es correcta
error-account-source = Una cuenta inicia sesión con Hoard (internal), un directorio (ldap) u OAuth
error-account-exists = Ya hay una cuenta con ese nombre
error-own-role = Otro administrador puede cambiar tu rol
error-remove-self = No puedes eliminar tu propia cuenta mientras tienes la sesión iniciada con ella
error-token-name = Ponle al token el nombre de lo que lo va a usar, en un máximo de 80 caracteres
error-token-role = Un token no puede hacer más que la cuenta que lo crea
error-wrong-setup-code = Ese no es el código de configuración que se le ha dado al servidor en HOARD_SETUP_CODE
error-wrong-password = Ese nombre y esa contraseña no coinciden
error-folder-missing = { $path } no está (¿un disco que no está montado?)
error-s3-path = una ubicación S3 tiene el aspecto s3://bucket
error-s3-address = la dirección de S3 debe empezar por http:// o https://
error-s3-header = una cabecera S3 adicional tiene la forma Name=value, y «{ $header }» no lo es
error-keychain = no se ha podido guardar en el llavero del sistema: { $reason }
error-not-built = esta copia de Hoard se ha compilado sin { $feature }
error-openai-auth-unreachable = no se ha podido conectar con auth.openai.com
error-openai-unexpected = respuesta inesperada de OpenAI
error-openai-answered = OpenAI ha respondido { $status }: { $reason }
error-chatgpt-id-token-not-jwt = el token de ID no es un JWT
error-chatgpt-id-token-issuer = el token de ID es de { $issuer }, no de { $expected }
error-chatgpt-id-token-audience = el token de ID es para otra aplicación
error-chatgpt-id-token-expired = el token de ID ha caducado
error-chatgpt-id-token-nonce = el token de ID no pertenece a este inicio de sesión
error-chatgpt-no-state = la dirección no tiene state; copia la dirección completa
error-chatgpt-not-started = este inicio de sesión no se empezó aquí, o se empezó hace más de 15 minutos. Vuelve a empezarlo desde Ajustes.
error-chatgpt-canceled = se ha cancelado el inicio de sesión
error-chatgpt-refused = ChatGPT ha rechazado el inicio de sesión: { $reason }
error-chatgpt-no-code = la dirección no tiene código de inicio de sesión; copia la dirección completa
error-chatgpt-other-registration = ChatGPT ha respondido para otro registro de este servidor; vuelve a iniciar sesión
error-chatgpt-other-registration-desktop = ChatGPT ha respondido para otra copia de Hoard; vuelve a iniciar sesión
error-chatgpt-not-registered = ChatGPT no ha terminado de registrar Hoard; vuelve a iniciar sesión
error-chatgpt-code-used = el código de inicio de sesión ya se ha usado o ha caducado; vuelve a iniciar sesión
error-chatgpt-refused-code = ChatGPT ha rechazado el inicio de sesión ({ $code })
error-chatgpt-no-id-token = ChatGPT no ha enviado ningún token de ID
error-chatgpt-plan-not-allowed = Se ha iniciado sesión, pero no se ha permitido usar el plan. Vuelve a iniciar sesión y permítelo, o usa una clave de API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = no se ha iniciado sesión en ChatGPT; inicia sesión desde Ajustes → Etiquetado con IA
error-chatgpt-refresh-later = ChatGPT no ha podido renovar el inicio de sesión ahora mismo ({ $code }); se volverá a intentar más tarde
error-chatgpt-signed-out-problem = ChatGPT ha cerrado la sesión de este servidor ({ $code }). Vuelve a iniciar sesión para seguir etiquetando con tu plan.
error-chatgpt-signed-out-problem-desktop = ChatGPT ha cerrado la sesión de Hoard ({ $code }). Vuelve a iniciar sesión para seguir etiquetando con tu plan.
error-chatgpt-signed-out = ChatGPT ha cerrado la sesión de este servidor ({ $code }); vuelve a iniciar sesión desde Ajustes → Etiquetado con IA
error-chatgpt-signed-out-desktop = ChatGPT ha cerrado la sesión de Hoard ({ $code }); vuelve a iniciar sesión desde Ajustes → Etiquetado con IA
error-openai-api-unreachable = no se ha podido conectar con api.openai.com
error-openai-broke-off = la respuesta de OpenAI se ha interrumpido
error-chatgpt-kept-refusing = OpenAI ha seguido rechazando el inicio de sesión de ChatGPT; vuelve a iniciar sesión desde Ajustes → Etiquetado con IA
error-chatgpt-request-failed = la petición ha fallado
error-chatgpt-cut-off = la respuesta se ha cortado ({ $reason })
error-chatgpt-cut-off-no-reason = la respuesta se ha cortado (sin motivo)
error-openai-ended-early = la respuesta de OpenAI ha terminado antes de estar completa
error-chatgpt-usage-limit = el uso de Hoard con tu plan de ChatGPT se ha agotado por ahora. Consulta o cambia el límite en { $url }
error-chatgpt-not-eligible = esta cuenta o espacio de trabajo de ChatGPT no puede usar su plan en otras aplicaciones. Usa una clave de API.
error-chatgpt-usage-unavailable = ChatGPT no ha podido comprobar el uso del plan ahora mismo; se volverá a intentar más tarde
error-chatgpt-unsupported = el uso con plan de ChatGPT no admite { $param } con este modelo
error-chatgpt-unsupported-images = el uso con plan de ChatGPT no admite { $param } con este modelo; elige un modelo que acepte imágenes o desactiva el envío de miniaturas
error-chatgpt-unsupported-part = el uso con plan de ChatGPT no admite parte de la petición con este modelo
error-chatgpt-port = port es el puerto por el que el navegador llega a Hoard
error-chatgpt-paste-address = pega la dirección completa, que empieza por http://127.0.0.1
error-too-many-hashes = como máximo { $max } hashes a la vez
error-credits-pack = paquete { $key }
error-credits-collection = colección { $key }
error-format-too-long = el formato es demasiado largo
error-not-started = el servidor no se ha iniciado
error-not-started-desktop = Hoard no se ha iniciado
error-not-format-file = no es un archivo de un formato: { $name }
error-format-name = el nombre necesita una letra o un dígito de la A a la Z o del 0 al 9
error-format-miss-name = los archivos del formato no aceptan { $file }
error-format-miss-store = el formato no acepta { $file } como de la tienda (consulta require)
error-format-miss-title = el formato no encuentra ningún título en { $file }
error-cant-download-as = los archivos .{ $ext } no se pueden descargar como { $format }
error-only-images-resize = solo se pueden redimensionar imágenes
error-https-no-certificate = Primero pega o sube tu certificado y su clave
error-https-no-cert-found = No se ha encontrado ningún certificado: pega el texto que empieza por -----BEGIN CERTIFICATE-----
error-https-no-key-found = No se ha encontrado ninguna clave privada: pega el texto que empieza por -----BEGIN PRIVATE KEY-----
error-https-several-keys = Hay más de una clave privada: pega solo la que corresponde al certificado
error-https-key-password = La clave está protegida con contraseña: dala sin ella (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = El certificado no es un PEM válido
error-https-key-not-pem = La clave privada no es un PEM válido
error-https-key-mismatch = La clave no corresponde al certificado. Si has dado una cadena, pon primero el certificado de tu servidor
error-https-unusable = No se puede usar el certificado o la clave
error-https-expired = Este certificado caducó el { $date }
error-https-not-yet-valid = Este certificado no es válido hasta el { $date }
error-https-unreadable = Eso no es un certificado que Hoard pueda leer
error-https-bad-name = «{ $name }» no es una dirección que pueda llevar un certificado
error-https-test-timed-out = se ha agotado el tiempo de la conexión de prueba
error-https-test-failed = la conexión de prueba ha fallado
error-https-no-ca = Hoard todavía no ha creado su autoridad de certificación
error-https-unavailable = HTTPS no está disponible en este servidor
error-https-certificate-mode = Un certificado va con «Mi certificado»
error-https-test = El certificado no ha funcionado en una conexión de prueba: { $reason }
error-nothing-to-retry = no hay nada que se pueda reintentar
error-ai-unreachable = no se ha podido conectar con { $url }
error-ai-key-refused = { $url } ha rechazado la clave de API ({ $status })
error-ai-answered = { $url } ha respondido { $status }: { $reason }
error-ai-not-json = { $url }/models no ha respondido con JSON
error-model-server-refused = el servidor de modelos: { $reason }
error-no-model-server = no responde ningún servidor de modelos en { $url }
error-not-llama-router = { $url } no es un servidor llama.cpp en modo router, así que no puede descargar modelos
error-model-download = no se ha podido descargar { $model }: { $reason }
error-model-delete = no se ha podido eliminar { $model }: { $reason }
error-unknown-kind = tipo de recurso desconocido { $kind }
error-watched-is-folder = una ubicación vigilada es una carpeta de este servidor
error-cant-reach = no se puede acceder a { $path }: { $reason }
error-folder-full-path = indica la ruta completa de la carpeta
error-cant-create = no se puede crear { $path }
error-cant-write = no se puede escribir en { $path }
error-cant-read = no se puede leer { $path }
error-overlaps-location = se solapa con la ubicación { $name } ({ $path })
error-overlaps-data = se solapa con la carpeta de datos de Hoard ({ $path })
error-overlaps-cache = se solapa con la carpeta de caché de Hoard ({ $path })
error-not-watched = { $name } no es una carpeta vigilada
error-watched-no-kinds = una carpeta vigilada no acepta archivos nuevos, así que no tiene tipos que cambiar
error-keep-catch-all = haz que otra ubicación sea la de reserva para todo
error-location-holds-files = { $name } todavía contiene archivos: muévelos primero a las otras ubicaciones (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = no se puede acceder a { $name }, así que Hoard no puede saber si todavía contiene archivos
error-relocate-running = ya se están moviendo archivos entre ubicaciones; vuelve a intentarlo cuando termine
error-watched-not-moved = { $name } es una carpeta vigilada: Hoard no mueve sus archivos
error-no-other-location = no hay otra ubicación de la biblioteca a la que mover los archivos: añade una primero
error-full-path = indica una ruta completa
error-pick-folder = elige una carpeta
error-task-running = «{ $task }» ya se está ejecutando
error-token-account-gone = la cuenta de este token ya no existe, así que no tiene favoritos ni búsquedas
error-favorites-too-many = como máximo 10.000 a la vez
error-saved-search-name = una búsqueda guardada necesita un nombre de 80 caracteres como máximo
error-saved-search-too-long = esa búsqueda es demasiado larga para guardarla
error-hoard-seen = el tesoro puede registrar como máximo 32 tipos de recurso, cada uno con un nombre en minúsculas
error-texture-pattern-map = El patrón de nombres de texturas necesita { "{" }map{ "}" }, o todos los mapas de un material tendrían el mismo nombre
error-texture-pattern-fields = El patrón de nombres de texturas puede usar { "{" }name{ "}" }, { "{" }map{ "}" } y { "{" }size{ "}" }
error-texture-name-chars = Los nombres de texturas no pueden tener < > : " / \ | ? ni *
error-texture-map-type = No hay ningún tipo de mapa llamado { $role }
server-role-base-color = color
server-role-normal = normales
server-role-normal-dx = normales (DirectX)
server-role-roughness = rugosidad
server-role-gloss = brillo
server-role-metalness = metalicidad
server-role-ao = oclusión ambiental
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = altura
server-role-bump = relieve
server-role-cavity = cavidad
server-role-specular = especular
server-role-opacity = opacidad
server-role-emissive = emisión
server-role-translucency = translucidez
error-texture-suffix-missing = Asigna un sufijo a los mapas de { $map }
error-texture-suffix-chars = El sufijo de los mapas de { $map } no puede tener < > : " / \ | ? ni *
error-texture-ending-missing = Cada terminación tuya necesita la terminación que se va a buscar
error-texture-ending-chars = Tus terminaciones no pueden tener < > : " / \ | ? ni *
error-texture-ending-twice = { $ending } está dos veces en tus terminaciones
error-oauth-no-client-id = Este enlace no tiene client_id.
error-oauth-unknown-app = Esta aplicación no está registrada aquí (o ya no lo está). Vuelve a conectarla desde la aplicación.
error-oauth-redirect = La aplicación ha pedido volver a una dirección que no ha registrado.
error-oauth-response-type = Hoard solo entrega códigos de inicio de sesión (response_type=code).
error-oauth-pkce = La aplicación tiene que usar PKCE con S256 para iniciar sesión en Hoard.
error-light-preview-images = solo las imágenes tienen vista previa ligera
error-no-light-preview = no hay vista previa ligera: { $reason }
error-no-frame-sheet = no hay hoja de fotogramas: { $reason }
error-thumbnail-source = origen de miniatura desconocido: { $source }
error-thumbnail-too-big = esa imagen es demasiado grande para una miniatura
error-thumbnail-models-only = solo los modelos aceptan una miniatura dibujada por un navegador
error-not-a-picture = no es una imagen que Hoard pueda leer: { $reason }
error-project-name = un proyecto necesita un nombre de 200 caracteres como máximo
error-project-too-big = como máximo un millón de recursos en un proyecto
error-project-too-many-at-once = como máximo 100.000 a la vez
error-server-name = Elige un nombre de hasta { $max } caracteres, en una sola línea
error-tool-setting = { $tool }: elige desactivado, automático o indica una ruta
error-ai-model = Elige el modelo que debe usar el servicio de IA
error-public-address = La dirección pública debe empezar por http:// o https://
error-update-hook-address = La dirección a la que llamar con las actualizaciones debe empezar por http:// o https://
error-update-hook-auth = La cabecera Authorization no puede contener saltos de línea ni otros caracteres de control
error-no-soundfont = No hay ninguna SoundFont en { $path } en el servidor
error-no-soundfont-desktop = No hay ninguna SoundFont en { $path }
error-soundfont-bad = { $name } no es un SoundFont con el que pueda sonar el MIDI: { $reason }
error-workers = Procesos en segundo plano: entre 1 y { $max }
error-ai-batch = Recursos por petición a la IA: entre 1 y 200
error-conversion-cache = Descargas convertidas: al menos 100 MB
error-backup-keep = Copias de seguridad programadas: conservar de 1 a { $max }
error-backup-copy-to = Copias de seguridad programadas: { $path } no es una carpeta del servidor (indica su ruta completa)
error-conversion-keep-days = Reglas de conversión: los originales deben conservarse entre 1 y { $max } días
error-conversion-target = Reglas de conversión: Hoard no puede convertir archivos a { $format }
error-conversion-no-source = Reglas de conversión: cada regla necesita al menos un formato que convertir
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Reglas de conversión: los archivos .{ $ext } no se pueden convertir a { $format }
error-conversion-twice = Reglas de conversión: .{ $ext } está en dos reglas
error-conversion-cant-undo = Ninguna regla ha convertido este archivo, o ya se ha eliminado su original.
error-conversion-path-taken = Ahora hay otro archivo en { $path }, así que el original no puede volver ahí.
error-limits-zero = Los límites no pueden ser cero
error-settings-object = se esperaba un objeto JSON de ajustes
error-bad-setting = ajuste no válido: { $reason }
error-auth-default-role = El rol predeterminado es view, edit o admin
error-auth-session-hours = Las sesiones duran entre 1 hora y un año
error-auth-settings = ajustes de autenticación no válidos: { $reason }
error-ldap-settings = ajustes de LDAP no válidos: { $reason }
error-ldap-no-server = Añade la dirección del servidor de directorio
error-ldap-base-dn = Indica dónde están las entradas de las personas (el DN base)
error-ldap-username-attribute = Indica qué atributo contiene el nombre con el que se inicia sesión
error-ldap-bind-dn = Indica la cuenta que busca a las personas, o búscalas de forma anónima
error-ldap-ca-cert = La autoridad de certificación no es un certificado PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = La dirección de un servidor de directorio está vacía
error-ldap-server-form = { $server }: la dirección de un servidor de directorio es host:puerto, ldap://… o ldaps://…
error-ldap-not-starttls = { $server }: eso es LDAPS, no StartTLS
error-ldap-no-certificate = no contiene ningún certificado
error-ldap-password-again = Vuelve a dar la contraseña de la cuenta que busca a las personas: la guardada solo se envía a los servidores para los que se dio
error-ldap-unreachable = no se ha podido conectar con ningún servidor de directorio ({ $reasons })
error-ldap-no-anonymous = el directorio no permite buscar personas de forma anónima ({ $reason })
error-ldap-bind-failed = la cuenta que busca a las personas no ha podido iniciar sesión ({ $reason })
error-ldap-search = buscando a { $name } en { $base }
error-ldap-group-search = buscando los grupos de { $name } en { $base }
error-ldap-several =
    { $count ->
        [one] { $count } entrada del directorio coincide con { $name }: restringe el filtro de usuarios
       *[other] { $count } entradas del directorio coinciden con { $name }: restringe el filtro de usuarios
    }
error-ldap-password-check = el directorio no ha querido comprobar la contraseña de { $name } ({ $reason })
error-ldap-wrong-password = El directorio no conoce a { $name } con esa contraseña
error-oauth-client-id = Indica el ID de cliente que el proveedor ha creado para Hoard
error-oauth-addresses = Las direcciones de autorización y de token deben empezar por https://
error-oauth-userinfo-address = La dirección de información del usuario debe empezar por https://
error-oauth-username-claim = Indica qué claim contiene el nombre de la persona
error-oauth-secret-again = Vuelve a dar el secreto de cliente: el guardado solo se envía a la dirección de token para la que se dio
error-oauth-issuer = El emisor debe empezar por https://
error-oauth-discovery-unreachable = No se ha podido conectar con { $url }: { $reason }
error-oauth-discovery-status = { $url } ha respondido { $status }
error-oauth-discovery-bad = { $url } no es un documento de descubrimiento
error-oauth-discovery-incomplete = { $url } no indica las direcciones de autorización y de token
error-signin-no-group = { $name } no está en ningún grupo que pueda usar Hoard
error-signin-other-source = Ya hay una cuenta de Hoard llamada { $name } que inicia sesión de otra forma
error-signin-other-subject = Ya hay una cuenta de Hoard llamada { $name } para otra persona del proveedor
error-signin-no-account = { $name } todavía no tiene cuenta de Hoard: un administrador puede añadirla en Ajustes → Cuenta
error-signin-bad-name = Ese nombre no puede ser una cuenta de Hoard (hasta 64 caracteres)
error-oauth-off = El inicio de sesión con OAuth no está activado
error-oauth-bad-authorization-address = la dirección de autorización no es válida
error-oauth-refused = El proveedor no te ha dejado iniciar sesión: { $reason }
error-oauth-no-code = El proveedor te ha devuelto sin iniciar sesión
error-oauth-expired = Ese inicio de sesión ha tardado demasiado o se empezó en otro sitio: vuelve a intentarlo
error-oauth-no-claim = El proveedor no ha dicho quién eres (no hay claim { $claim })
error-oauth-no-sub = El proveedor no ha dicho quién eres (no hay claim sub ni id)
error-oauth-token-unreachable = no se ha podido conectar con la dirección de token del proveedor
error-oauth-token-refused = El proveedor no ha querido terminar el inicio de sesión: { $reason }
error-oauth-no-access-token = el proveedor no ha enviado ningún token de acceso
error-oauth-userinfo-unreachable = no se ha podido conectar con la dirección de información del usuario del proveedor
error-oauth-userinfo-refused = El proveedor no ha querido decir quién ha iniciado sesión ({ $status })
error-oauth-userinfo-not-json = la información del usuario del proveedor no es JSON
error-format-no-name = name está vacío
error-format-no-files = files está vacío: indica qué archivos lee el formato, como ["*.json"]
error-format-empty-selector = vacío
error-format-unopened-bracket = un corchete de cierre sin uno de apertura
error-format-unclosed-bracket = un corchete sin cerrar
error-format-empty-step-dots = un paso vacío (dos puntos seguidos, o un punto en un extremo)
error-format-unclosed-except = !( sin cerrar
error-format-unclosed-filter = [ sin cerrar
error-format-after-filters = { $rest } después de la clave y los filtros de { $step }
error-format-except-needs-glob = !(…) va después de una clave con * dentro
error-format-except-not-glob = !(…) va después de una clave con * dentro, no de { $key }
error-format-empty-step = un paso vacío
error-format-not-a-filter = [{ $filter }] no es un filtro: usa [], [key=value|other], [array], [string], [number], [bool] o [object]
error-format-filter-needs-value = [{ $filter }] necesita una clave y un valor
error-store-search-empty = no hay nada que buscar; escribe el nombre del paquete
error-update-hook-not-set = primero indica una dirección a la que llamar
error-upload-stalled = la subida de { $path } se ha atascado; vuelve a enviar esa parte
error-upload-busy = ya se está escribiendo { $path }
error-upload-incomplete =
    { $count ->
        [one] { $count } archivo está incompleto, p. ej. { $files }
       *[other] { $count } archivos están incompletos, p. ej. { $files }
    }
error-upload-no-files-listed = no hay archivos
error-upload-not-part = { $path } no forma parte de la subida { $id }, o la subida está cerrada
error-bad-path = ruta no válida
error-upload-part-offset = { $offset } no es donde empieza una parte de { $path }
error-upload-interrupted = subida interrumpida: { $reason }
error-upload-part-long = la parte { $part } de { $path } ocupa más de { $len } bytes
error-upload-part-short = parte { $part } de { $path }: se han recibido { $got } bytes y se esperaban { $len }
error-upload-file-long = { $path } es más grande de lo declarado
error-upload-file-short = { $path }: se han recibido { $got } bytes y se esperaban { $len }
error-upload-complete = la subida ya está completa
error-already-imported = ya importado como «{ $name }»

## Textos del servidor

server-store-place-readme = README
server-store-place-folders = carpetas
server-store-why-linked = Enlazado en el paquete ({ $place })
server-store-why-same-title = Mismo título que en el paquete ({ $place })
server-store-why-by = De { $author }, nombrado en el paquete ({ $place })
server-store-why-meshes = Más o menos tantas mallas como el paquete ({ $count })
server-store-why-textures = Más o menos tantas texturas como el paquete ({ $count })
server-chatgpt-revoke-failed = Se ha cerrado la sesión aquí, pero no se ha podido avisar a OpenAI. Para asegurarte, desconecta Hoard en ChatGPT, en Configuración → Seguridad e inicio de sesión → Iniciar sesión con ChatGPT.
server-chatgpt-signed-in-title = Sesión iniciada en ChatGPT
server-chatgpt-signed-in = Hoard ha iniciado sesión. El etiquetado con IA ya puede usar tu plan de ChatGPT.
server-chatgpt-signed-in-as = Hoard ha iniciado sesión como { $email }. El etiquetado con IA ya puede usar tu plan de ChatGPT.
server-chatgpt-not-finished-title = Inicio de sesión sin terminar
server-chatgpt-other-computer = Si empezaste el inicio de sesión desde Hoard en otro ordenador, copia la dirección de esta página de la barra de direcciones y pégala allí, en los Ajustes de Hoard.
server-back-to-settings = Volver a Ajustes
server-issues-pack = Paquetes que no se han podido añadir
server-issues-pack-blurb = Elimina el paquete y vuelve a añadirlo, o ábrelo para ver lo que contiene.
server-issues-damaged = Archivos guardados dañados o desaparecidos
server-issues-damaged-blurb = Encontrados por «Comprobar daños en los archivos». Restáuralos desde una copia de seguridad o vuelve a añadir su paquete, y luego ejecuta de nuevo «Comprobar daños en los archivos».
server-issues-folder = Carpetas vigiladas que no se pueden leer
server-issues-folder-blurb = Sus paquetes se quedan como están hasta que la carpeta se pueda volver a leer.
server-issues-missing = Paquetes vigilados cuya carpeta ha desaparecido
server-issues-missing-blurb = Cada paquete se conserva 3 días por si su carpeta solo está desconectada, y luego se elimina.
server-issues-job = Tareas en segundo plano que han fallado
server-issues-job-blurb = Al reintentar, la tarea vuelve a ejecutarse desde el principio.
server-issues-file = Archivos que no se han podido abrir
server-issues-file-blurb = Hoard no ha podido abrirlos ni guardarlos. Un paquete vigilado los vuelve a intentar en su siguiente escaneo; un archivo que ya estaba en la biblioteca conserva lo que tenía.
server-issues-unpack = Archivos comprimidos que no se han podido descomprimir
server-issues-unpack-blurb = Cada uno se conserva en su paquete como un archivo normal, así que nada de lo que contiene está en la biblioteca.
server-issues-read = Archivos que Hoard no ha sabido interpretar
server-issues-read-blurb = Están en la biblioteca, pero sin los datos, las etiquetas ni la miniatura que da leerlos.
server-issues-render = Modelos con los que ha fallado el generador de miniaturas
server-issues-render-blurb = No tienen miniatura. Puede que abrir uno en el visor funcione igualmente, y así se puede guardar una.
server-issues-convert = Modelos que Blender no ha podido convertir
server-issues-convert-blurb = No tienen copia en GLB, así que no se pueden mostrar en 3D.
server-issues-unreal = Paquetes de Unreal que no se han podido convertir
server-issues-unreal-blurb = La página de Unreal del paquete tiene el resto de sus paquetes.
server-issues-reformat = Archivos que una regla de conversión no ha podido convertir
server-issues-reformat-blurb = Se han dejado como estaban.
server-issues-materials = Paquetes cuyos materiales no se han podido deducir
server-issues-materials-blurb = Sus texturas están todas; simplemente no están agrupadas en materiales.
server-issues-material-thumb = Materiales sin miniatura
server-issues-material-thumb-blurb = Se ha encontrado el material, pero no se ha podido dibujar su vista previa.
server-issues-store-files = Archivos de tienda que no se han podido leer
server-issues-store-files-blurb = Archivos de la tienda de la que viene un paquete, como un JSON de Megascans, que le dan nombre y etiquetas.
server-issues-scan-again = Volver a escanear
server-issues-retry = Reintentar
server-issues-reanalyze = Volver a analizar el paquete
server-issues-convert-again = Volver a convertir
server-issues-look-again = Volver a buscar
server-issues-read-again = Volver a leer
server-issues-blob-missing = El archivo guardado ha desaparecido.
server-issues-blob-damaged = El archivo guardado no coincide con lo que se guardó: está dañado.
server-issues-folder-gone = No se ha encontrado la carpeta «{ $folder }».
server-job-a-pack = un paquete
server-job-ingest = Incorporando { $pack }
server-job-retag = Volviendo a etiquetar { $pack }
server-job-llm-tag = Etiquetando { $pack } con IA
server-job-convert = Convirtiendo { $pack } con Blender
server-job-unreal = Convirtiendo paquetes de Unreal en { $pack }
server-job-reformat = Convirtiendo los archivos de { $pack } según las reglas de conversión
server-job-scan = Escaneando una carpeta vigilada
server-job-relocate = Moviendo archivos entre ubicaciones de la biblioteca
server-job-looks = Analizando colores y aspecto
server-job-sounds = Escuchando sonidos
server-job-embed = Aprendiendo qué muestran las imágenes
server-job-hear = Aprendiendo cómo suenan los sonidos
server-job-materials = Buscando materiales en { $pack }
server-job-read-files = Leyendo los archivos de { $pack }
server-job-store-lookup = Consultando una página de tienda
server-maintenance-backup = Hacer copia de seguridad de la base de datos
server-maintenance-backup-blurb = Una copia coherente de la base de datos (paquetes, etiquetas, notas, colecciones y Ajustes) en la carpeta de copias de seguridad que hay junto a ella. Los archivos de la biblioteca no van en ella: nunca cambian una vez guardados, así que cualquier sincronización de archivos les hace copia de seguridad.
server-maintenance-scrub = Comprobar daños en los archivos
server-maintenance-scrub-blurb = Vuelve a leer cada archivo guardado y lo compara con su huella, para detectar archivos que se han estropeado o que han desaparecido de un disco o NAS. Tarda un rato en una biblioteca grande.
server-maintenance-reanalyze = Volver a analizar todos los paquetes
server-maintenance-reanalyze-blurb = Vuelve a leer cada archivo con esta versión de Hoard: las miniaturas, vistas previas, detalles y etiquetas de reglas se rehacen. Tus propios cambios se mantienen; no se vuelve a preguntar a la IA.
server-maintenance-clear-cache = Borrar conversiones en caché
server-maintenance-clear-cache-blurb = Elimina los archivos guardados de las conversiones de «Descargar como» (se vuelven a hacer la próxima vez que se pidan) y lo que haya dejado el trabajo interrumpido.
server-maintenance-shrink-thumbs = Reducir miniaturas
server-maintenance-shrink-thumbs-blurb = Guarda las miniaturas y vistas de modelos que están en PNG como pide Ajustes → Rendimiento → Miniaturas, lo que suele hacerlas varias veces más pequeñas. Las que tienen transparencia, y el pixel art, se quedan en PNG.
server-maintenance-gc = Eliminar archivos sin usar
server-maintenance-gc-blurb = Elimina los archivos guardados a los que ya no apunta nada, como los archivos de paquetes eliminados definitivamente.
server-update-no-address = no se ha indicado ninguna dirección a la que llamar
server-update-no-answer = sin respuesta en 30 segundos

## Ajustes: Apariencia

appearance-language = Idioma
appearance-language-auto = El del navegador ({ $name })

## Raíz

root-unreachable = No se ha podido conectar con el servidor de Hoard: { $error }
root-unreachable-desktop = Hoard no responde: { $error }
root-desktop-reopen = Se ha cerrado la sesión de esta ventana. Cierra Hoard y vuelve a abrirlo para continuar.
root-try-again = Volver a intentarlo
root-lazy-connect = la conexión de la aplicación
window-minimize = Minimizar
window-maximize = Maximizar
window-restore = Restaurar
window-close = Cerrar

## Aplicación: la biblioteca, su barra superior y los filtros

app-lazy-settings = los ajustes
app-lazy-archive = el Archivo
app-lazy-shortcuts = los atajos
app-lazy-setup = la configuración inicial
app-theme-system = Usar el tema del sistema
app-theme-light = Tema claro
app-theme-dark = Tema oscuro
app-color-red = Rojo
app-color-orange = Naranja
app-color-yellow = Amarillo
app-color-green = Verde
app-color-teal = Verde azulado
app-color-blue = Azul
app-color-purple = Morado
app-color-pink = Rosa
app-color-brown = Marrón
app-color-sand = Arena
app-color-gray = Gris
app-color-slate = Pizarra
app-color-black = Negro
app-color-white = Blanco
app-color-swatch = { $color }: recursos con mucho { $lower }
app-color-any = Cualquier color
app-color-pick = Elegir cualquier color
app-color = Color
app-music = Música
app-tempo = BPM
app-tempo-min = desde
app-tempo-max = hasta
app-tempo-min-title = Música a este tempo o más rápida, en pulsos por minuto (también cuenta la mitad o el doble)
app-tempo-max-title = Música a este tempo o más lenta, en pulsos por minuto
app-key = Tonalidad
app-key-any = Cualquier tonalidad
app-key-major = Mayor
app-key-minor = Menor
app-key-mix = Y tonalidades que se mezclan bien con ella
app-key-mix-title = Su relativa mayor o menor y las tonalidades a una quinta por encima y por debajo, que los DJ mezclan con ella
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Escribe el ruido que hace un sonido como lo deletrearías: primero salen los que se le parecen
app-imitation-record = Haz el ruido en el micrófono: primero salen los sonidos que se le parecen
app-imitation-stop = Detener la grabación
app-imitation-heard = Tu ruido
app-imitation-mic-failed = No se ha podido grabar desde el micrófono: { $error }
app-retry = Reintentar
app-undo = Deshacer
app-search-failed = La búsqueda ha fallado: { $error }
app-unreachable = No se ha podido conectar con el servidor: { $error }
app-unreachable-desktop = Hoard no responde: { $error }
app-forget-project-title = ¿Olvidar { $name }?
app-forget-project-body =
    Hoard deja de mostrarlo donde se usan { $count ->
        [one] su { $count } recurso
       *[other] sus { $count } recursos
    }, hasta que su próximo hoard sync se lo vuelva a indicar. No cambia nada en el proyecto.
app-forget-project-confirm = Olvidarlo
app-forget-project-failed = No se ha podido olvidar { $name }: { $error }
app-forget-project-label = Olvidar el proyecto { $name }
app-forget-project = Olvidar este proyecto
app-save-name-like = como { $name }
app-save-name-favorites = favoritos
app-search-saved = Se ha guardado «{ $name }»: está en Filtros, en Lo tuyo
app-rank-alike = Más parecidos
app-rank-color = Con más de ese color
app-rank-match = Mejor coincidencia
app-rank-imitation = Los que más se parecen
app-sort = Ordenar
app-sort-newest = Más recientes
app-sort-name = Nombre
app-sort-largest = Más grandes
app-sort-oldest = Más antiguos
app-collection-added =
    { $count ->
        [one] Se ha añadido { $count } recurso
       *[other] Se han añadido { $count } recursos
    } a { $name }
app-collection-add-failed = No se ha podido añadir a la colección: { $error }
app-collection-removed = Se han quitado { $count } de { $name }
app-collection-remove-failed = No se ha podido quitar de la colección: { $error }
app-ns-subject = Tema principal
app-ns-style = Estilo
app-ns-theme = Ambientación
app-ns-material = Material
app-ns-map = Mapa PBR
app-ns-color = Color
app-ns-mood = Atmósfera
app-ns-genre = Género
app-ns-license = Licencia
app-ns-res = Resolución
app-ns-duration = Duración
app-ns-poly = Número de polígonos
app-ns-format = Formato
app-ns-weight = Peso
app-ns-tech = Técnico
app-ns-blender = Versión de Blender
app-ns-depth = Profundidad de bits
app-ns-ucs = Categoría de sonido (UCS)
app-ns-bpm = Tempo
app-ns-root = Nota fundamental
app-ns-level = Sonoridad
app-ns-ue = Clase de Unreal
app-ns-engine = Motor
app-ns-extracted = Fuente recuperada
app-ns-script = Sistema de escritura
app-bulk-failed =
    No se ha podido cambiar { $count ->
        [one] ese recurso
       *[other] esos { $count } recursos
    }: { $error }
app-bulk-untagged =
    Se ha quitado { $tag } de { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
app-bulk-tagged =
    Se ha etiquetado { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    } con { $tag }
app-bulk-category-set =
    Se ha cambiado la categoría de { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    } a { $category }
app-bulk-category-cleared =
    Se ha quitado la categoría de { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
app-credits-failed = No se han podido generar los créditos: { $error }
app-archived-assets =
    { $count ->
        [one] Se ha archivado { $count } recurso
       *[other] Se han archivado { $count } recursos
    }
app-archived-one = Se ha archivado { $name }
app-open-failed = No se ha podido abrir ese recurso: { $error }
app-unnamed-pack = paquete
app-unnamed-collection = colección
app-unnamed-asset = recurso
app-home-version = Hoard { $version }: volver a la biblioteca
app-home = Volver a la biblioteca
app-search-label = Buscar recursos
app-search-placeholder = Busca nombres, rutas, etiquetas
app-views = Vistas
app-library = Biblioteca
app-packs = Paquetes
app-packs-working = { $count } en curso
app-update-title = Ha salido Hoard { $version } (este servidor ejecuta { $current }). Descarga la nueva imagen o el binario y reinicia; la base de datos se actualiza sola.
app-update = Actualizar
app-theme = Tema
app-sign-in = Iniciar sesión
app-archive-title = Archivo: { $contents }
app-archive = Archivo
app-settings = Ajustes
app-palette-title = Paleta de comandos ({ $keys })
app-import-folder = Importar carpeta
app-add-files = Añadir archivos
app-add-files-short = Añadir
app-split-label = Cómo importar esta carpeta
app-split-before = { "" }
app-split-after = { " " }contiene { $count } carpetas o archivos comprimidos.
app-split-each = Importar cada uno como un paquete ({ $count })
app-split-one = Importar como un solo paquete
app-cancel = Cancelar
app-filters = Filtros
app-done = Hecho
app-yours = Lo tuyo
app-favorites = Favoritos
app-forget-search-label = Olvidar la búsqueda guardada { $name }
app-forget-search = Olvidar esta búsqueda
app-projects-title = Proyectos que hacen hoard sync desde aquí, con lo que tenía su última sincronización
app-projects = Proyectos
app-project-synced = Sincronizado { $when }
app-project-synced-by = Sincronizado { $when } por { $who }
app-collections = Colecciones
app-no-filters = No hay filtros para esta búsqueda.
app-kind = Tipo
app-facets-more = { $count } más
app-facets-fewer = menos
app-searching = Buscando…
app-searching-live = Buscando
app-asset-count =
    { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
app-clear-all = Borrar todo
app-save-name-label = Nombre de esta búsqueda
app-save-name-placeholder = Ponle nombre
app-save = Guardar
app-save-search-title = Guardar esta búsqueda en Filtros, en Lo tuyo
app-save-search = Guardar búsqueda
app-select-title = Elige varios recursos para etiquetarlos, categorizarlos, añadirlos a una colección, descargarlos o archivarlos a la vez (X o Ctrl+clic)
app-select = Seleccionar
app-tile-size = Tamaño de mosaico
app-supporting-title = Búferes de glTF, bibliotecas de materiales, los mapas de los materiales, materiales que solo usa un modelo y los archivos de tienda que describen modelos. Se incluyen cuando descargas lo que los usa. Las páginas de una hoja de sprites, y los sprites que el paquete también tiene como imágenes sueltas, se muestran con la hoja. Un recurso que el paquete tiene en varios formatos se muestra una sola vez, en el formato que Hoard lee mejor.
app-supporting-hidden =
    { $count ->
        [one] { $count } archivo de apoyo oculto
       *[other] { $count } archivos de apoyo ocultos
    }
app-supporting-show = Mostrar
app-supporting-shown = Con archivos de apoyo
app-supporting-hide = Ocultar
app-folders = Carpetas
app-folders-title = Ver el paquete como estaba en el disco, carpeta por carpeta
app-folders-label = Carpetas del paquete
app-welcome-title = Suelta un paquete de recursos en cualquier parte de esta página
app-welcome-before = Sirven zips, tarballs, carpetas o archivos sueltos. Hoard los descomprime, guarda cada archivo una sola vez por su hash, lee tamaños, duraciones y número de polígonos, y lo etiqueta todo para que puedas buscar cosas como{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }o{ " " }
app-welcome-after = .
app-welcome-import-folder = Importar una carpeta
app-welcome-choose-files = Elegir archivos
app-none-similar = Todavía no hay nada que se parezca mucho a { $name }.
app-none = No hay coincidencias.
app-none-query = No hay coincidencias para «{ $query }».
app-none-imitation = Todavía ningún sonido medido se parece mucho a «{ $word }».
app-none-heard = Todavía ningún sonido medido se parece mucho a tu ruido.
app-none-filters = No hay coincidencias con estos filtros.
app-none-query-filters = No hay coincidencias para «{ $query }» con estos filtros.
app-clear-search-filters = Borrar búsqueda y filtros
app-loading-more = Cargando más
app-keys-hint = Las flechas mueven, Enter abre, la barra espaciadora reproduce un sonido, X o Ctrl+clic selecciona varios (pulsación larga en pantallas táctiles).
app-resize-label = Cambiar el tamaño del panel de detalles
app-resize-title = Arrastra para cambiar el tamaño · doble clic para el ancho justo
app-selected-label = Recursos seleccionados
app-selected =
    { $count ->
        [one] { $count } seleccionado
       *[other] { $count } seleccionados
    }
app-pick-assets = Elige recursos
app-download-zip = Descargar zip
app-credits-title = CREDITS.md de los paquetes de los que vienen: quién los hizo, sus licencias y las líneas de créditos que piden
app-credits = Créditos
app-prefer-sources-title = SoundWaves y texturas de Unreal cuyo archivo original ha recuperado Hoard
app-prefer-sources = WAV/PNG recuperado en lugar de .uasset ({ $count })
bag-clear = Borrar la selección
bag-glb-heading = Modelos como GLB
bag-glb-text = Cada modelo como archivo .glb, de pie con la Y hacia arriba y mirando al frente, con sus texturas como archivos aparte en una carpeta textures que comparten los modelos. Todo lo demás va tal cual.
bag-glb-download = Descargar zip de GLB
bag-plain-heading = Archivos tal cual
bag-plain-text = Cada archivo tal como se importó, con lo que necesita, en una carpeta por paquete.
bag-remove = Quitar { $name } de la selección
bag-more = y { $count } más
app-bulk-tag-label = Una etiqueta para añadir a los recursos seleccionados o quitársela
app-bulk-tag-placeholder = Etiqueta
app-bulk-tag-add-title = Añadir esta etiqueta a todos los recursos seleccionados
app-bulk-tag-add = Añadir
app-bulk-tag-remove-title = Quitar esta etiqueta de todos los recursos seleccionados
app-bulk-tag-remove = Quitar
app-bulk-category-label = Categoría de los recursos seleccionados (vacía la quita)
app-bulk-category-placeholder = Categoría
app-bulk-category-set-title = Poner esta categoría a todos los recursos seleccionados
app-bulk-category-clear-title = Quitar la categoría de todos los recursos seleccionados
app-bulk-category-set-button = Poner
app-bulk-category-clear-button = Quitar
app-remove-from-collection = Quitar de { $name }
app-unfavorite = Quitar de favoritos
app-favorite = Añadir a favoritos
app-select-all = Seleccionar todo ({ $count })
app-archive-selected-title = Fuera de las búsquedas hasta que se restauren o se eliminen definitivamente, desde el Archivo
app-drop-items = Suelta { $count } elementos para añadirlos
app-drop = Suelta para añadir a Hoard
app-drop-hint = Cada archivo o carpeta de primer nivel se convierte en su propio paquete. Los gigabytes no son problema; los archivos grandes se reanudan si se corta la conexión.

## Paleta de comandos (CommandPalette.vue, con sus comandos de App.vue)

palette-group-go-to = Ir a
palette-group-actions = Acciones
palette-group-theme = Tema
palette-group-kinds = Tipos
palette-group-packs = Paquetes
palette-group-collections = Colecciones
palette-group-projects = Proyectos
palette-group-saved = Búsquedas guardadas
palette-group-assets = Recursos
palette-hint-current = actual
palette-library = Biblioteca
palette-library-keywords = recursos cuadrícula inicio assets grid home
palette-favorites = Favoritos
palette-favorites-keywords = estrella destacados gustan favoritos starred stars liked
palette-packs = Paquetes
palette-archive = Archivo
palette-archive-keywords = papelera reciclaje eliminados borrados restaurar archivados trash recycle bin deleted restore
palette-stats = Estadísticas de la biblioteca
palette-stats-keywords = estadísticas números tamaño disco espacio recuento statistics numbers size disk space count
palette-copies = Casi copias
palette-copies-keywords = duplicados iguales parecidos redimensionados convertidos espacio desperdiciado duplicates same similar resized converted wasted space dedupe
palette-search = Buscar
palette-add-files = Añadir archivos…
palette-add-files-keywords = subir importar upload import
palette-import-folder = Importar una carpeta…
palette-import-folder-keywords = subir paquete upload pack
palette-select-all = Seleccionar todo lo mostrado ({ $count })
palette-select-all-keywords = marcar check
palette-select-several = Seleccionar varios…
palette-select-several-keywords = masivo varios múltiple etiquetar archivar bulk multiple tag archive
palette-archive-selected = Archivar los seleccionados ({ $count })
palette-archive-selected-keywords = eliminar quitar borrar papelera delete remove trash
palette-clear-selection = Borrar la selección
palette-clear-filters = Borrar filtros
palette-clear-filters-keywords = restablecer reiniciar reset
palette-hide-supporting = Ocultar archivos de apoyo
palette-show-supporting = Mostrar archivos de apoyo
palette-supporting-keywords = mapas normales rugosidad máscaras normal maps roughness masks
palette-account = Cuenta y tokens de acceso
palette-account-keywords = contraseña acceso usuario token api password login user
palette-sign-out = Cerrar sesión ({ $name })
palette-sign-out-keywords = salir desconectar log out logout
palette-sign-in = Iniciar sesión
palette-sign-in-keywords = entrar acceder cuenta contraseña log in login account password
palette-settings = Ajustes
palette-settings-keywords = blender ffmpeg llm ia etiquetado unreal administrador herramientas preferencias configuración ai tagging admin tools preferences settings
palette-shortcuts = Atajos de teclado
palette-shortcuts-keywords = ayuda teclas help keys
palette-theme-keywords = tema apariencia modo theme appearance mode
palette-theme-variant-keywords = apariencia color appearance
palette-light-theme = Tema claro: { $name }
palette-dark-theme = Tema oscuro: { $name }
palette-kind-all = Todos los tipos (dejar de mostrar solo { $kind })
palette-kind-only = Solo { $kind }
palette-title = Paleta de comandos
palette-placeholder = Busca recursos, ve a un paquete, ejecuta un comando…
palette-results = Resultados
palette-searching = Buscando…
palette-no-match = No hay coincidencias para «{ $query }»
palette-foot-move = mover
palette-foot-run = ejecutar
palette-foot-shortcuts = todos los atajos

## Atajos de teclado (ShortcutsDialog.vue)

shortcuts-title = Atajos de teclado
shortcuts-close = Cerrar
shortcuts-anywhere = En cualquier parte
shortcuts-palette = Paleta de comandos: buscar, saltar, ejecutar
shortcuts-search = Buscar
shortcuts-this-list = Esta lista
shortcuts-escape = Cerrar, borrar la selección o cerrar los detalles
shortcuts-library = Biblioteca
shortcuts-move = Moverse entre mosaicos (o J y K)
shortcuts-open = Abrir los detalles
shortcuts-play-sound = Reproducir o detener un sonido
shortcuts-select = Seleccionar o deseleccionar
shortcuts-favorite = Añadir a favoritos o quitar de favoritos
shortcuts-key-click = clic
shortcuts-select-range = Seleccionar un intervalo
shortcuts-select-all = Seleccionar todo lo mostrado
shortcuts-sound-player = Reproductor de sonido
shortcuts-play-pause = Reproducir o pausar
shortcuts-skip = Retroceder o avanzar
shortcuts-start-end = Inicio o final
shortcuts-image-inspector = Inspector de imágenes
shortcuts-zoom = Acercar o alejar
shortcuts-fit = Ajustar a la ventana
shortcuts-actual-pixels = Píxeles reales
shortcuts-tile = Repetir en mosaico 3×3 para comprobar las costuras
shortcuts-background = Siguiente fondo
shortcuts-channel = Mostrar un canal

## KindChips.vue

kinds-group = Tipos de recurso

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = añadida por una persona
tag-source-rule = del etiquetador sin conexión
tag-source-meta = medida a partir del archivo
tag-source-ai = sugerida por la IA
tag-remove = Quitar la etiqueta { $tag }

## ActivityIndicator.vue

activity-label-idle = Actividad en segundo plano: inactiva
activity-label-running = Actividad en segundo plano: { $running } en curso
activity-label-queued = Actividad en segundo plano: { $running } en curso, { $queued } en cola
activity-title = Actividad en segundo plano
activity-unreachable = No se puede conectar con el servidor
activity-unreachable-desktop = Hoard no responde
activity-empty = No hay nada en curso. Las importaciones, el etiquetado, las conversiones y las descargas de zip aparecen aquí mientras se procesan.
activity-queued = en cola
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } tris
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Reproducir { $name }
tile-pause = Pausar { $name }
tile-favorite = Añadir { $name } a favoritos
tile-in-favorites = En tus favoritos
tile-add-favorite = Añadir a tus favoritos (F)
tile-more-like = Más como { $name }
tile-more-like-this = Más como este
tile-select = Seleccionar { $name }

## CollectionPicker.vue

collections-add = Añadir a una colección
collections-menu = Colecciones
collections-find-label = Buscar o crear una colección
collections-find-placeholder = Busca o pon nombre a una colección nueva
collections-create = Crear
collections-empty = Todavía no hay colecciones. Escribe un nombre para empezar una.

## FolderPicker.vue

folders-list = Carpetas
folders-close = Cerrar
folders-browse = Examinar…
folders-up-title = Subir una carpeta
folders-up = Subir
folders-this-computer = Este ordenador
folders-none = No hay carpetas aquí.

## LocationForm.vue

locations-mode-question = ¿Qué debe hacer Hoard con esta carpeta?
locations-mode-managed = Reclamarla: Hoard guarda mis paquetes aquí
locations-mode-managed-hint = Los paquetes que subas o importes se copian aquí, y Hoard los mantiene en orden. Ahora la carpeta es de Hoard, así que no toques su contenido.
locations-mode-watched = Vigilarla: mis paquetes ya están aquí
locations-mode-watched-hint = Cada carpeta que contiene se convierte en un paquete. Hoard lee los archivos donde están y nunca los cambia, mueve ni elimina, y detecta las carpetas nuevas a medida que aparecen.
locations-folder-desktop = Carpeta
locations-folder-placeholder = La carpeta donde están tus paquetes
locations-folder-hint-managed = Hoard la crea si no existe. En Docker, es una carpeta que has montado en el contenedor.
locations-folder-hint-managed-desktop = Hoard la crea si no existe.
locations-folder-hint-watched = En Docker, primero monta la carpeta en el contenedor (de solo lectura vale) y luego elígela aquí.
locations-folder-hint-watched-desktop = Hoard solo la lee: no se cambia nada de lo que contiene.
locations-name = Nombre
locations-name-placeholder-managed = Biblioteca
locations-name-placeholder-watched = Mis paquetes
locations-files = Archivos que van aquí
locations-catch-all = También todo lo que no acepta ninguna otra ubicación
locations-kinds-hint = Los archivos de un paquete nuevo van a la ubicación que tiene marcado su tipo. El resto va a la de reserva.
locations-files-all = Todos, por ahora. Más adelante puedes añadir más ubicaciones desde Ajustes para enviar algunos tipos (la música, por ejemplo) a otro disco.
locations-first-watched-note = Para subir archivos hace falta una biblioteca que Hoard gestione. Puedes añadirla más adelante desde Ajustes → Ubicaciones de la biblioteca.
locations-checking = Comprobando la carpeta…
locations-add = Añadir ubicación
locations-cancel = Cancelar

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = No se han podido cargar las ubicaciones de la biblioteca
locations-takes-and-rest = { $kinds } y todo lo demás
locations-takes-rest = Todo lo que no aceptan las demás
locations-takes-everything = Todo
locations-takes-nothing = Nada todavía
locations-save-failed = No se ha podido guardar la ubicación
locations-unwatch-title = ¿Dejar de vigilar { $name }?
locations-unwatch-body =
    { $count ->
        [one] Su { $count } paquete sale de Hoard, con sus etiquetas y notas.
       *[other] Sus { $count } paquetes salen de Hoard, con sus etiquetas y notas.
    } Los archivos de la carpeta no se tocan.
locations-unwatch-body-empty = Los archivos de la carpeta no se tocan.
locations-unwatch = Dejar de vigilar
locations-remove-title = ¿Quitar { $name }?
locations-remove-body = Los archivos que contenga se mueven primero a las otras ubicaciones, según su tipo; se quita en cuanto esté vacía. No se pierde nada.
locations-remove-body-only = Si contiene archivos, añade primero otra ubicación a la que moverlos.
locations-remove = Quitar
locations-removed = Se ha quitado { $name }
locations-emptying = Sacando los archivos de { $name }; se quitará cuando termine
locations-offline-title = No se puede acceder a { $name }
locations-offline-body = Sus archivos no se pueden sacar mientras esté desconectada. ¿Quitarla de todos modos? Los archivos que sigan en ella aparecerán como desaparecidos.
locations-remove-anyway = Quitar de todos modos
locations-remove-failed = No se ha podido quitar { $name }
locations-rebalancing = Moviendo los archivos guardados a las ubicaciones que aceptan su tipo
locations-rebalance-failed = No se han podido mover los archivos
locations-scanned-never = sin escanear todavía
locations-scanned-now = escaneada hace un momento
locations-scanned-minutes = escaneada hace { $mins } min
locations-scanned-hours = escaneada hace { $hours } h
locations-scanned-date = escaneada el { $date }
locations-scan-failed = No se ha podido escanear { $name }
locations-watching = Vigilando { $name }
locations-added = Se ha añadido { $name }
locations-loading = Cargando…
locations-managed-title = Bibliotecas que gestiona Hoard
locations-watched-title = Carpetas vigiladas
locations-managed-hint = Las subidas y las importaciones se copian aquí. Cada ubicación acepta los tipos que tiene marcados; la de reserva acepta el resto.
locations-watched-hint = Carpetas de paquetes indexadas donde están. Hoard nunca cambia sus archivos, y busca carpetas nuevas y eliminadas cada 15 minutos.
locations-managed-empty = Todavía no hay ninguna, así que no hay dónde poner una subida. Añade una abajo.
locations-reachable = Accesible
locations-unreachable-now = No se puede acceder ahora mismo
locations-catch-all-tag = de reserva
locations-packs =
    { $count ->
        [one] { $count } paquete
       *[other] { $count } paquetes
    }
locations-unreachable = no accesible
locations-scanning = Escaneando…
locations-scan = Escanear ahora
locations-edit = Editar
locations-catch-all-locked = Para cambiarlo, haz que otra ubicación sea la de reserva
locations-edit-hint-before = Se aplica a los archivos que se añadan a partir de ahora. Para mover lo que ya está guardado, usa
locations-edit-hint-after = abajo.
locations-save = Guardar
locations-rebalance = Mover los archivos guardados para que coincidan
locations-rebalance-hint = Después de cambiar lo que acepta una ubicación: los archivos ya guardados se mueven a la ubicación a la que va ahora su tipo.
locations-add-title = Añadir una ubicación
locations-add-hint = Otro disco para algunos tipos de archivos, o una carpeta de paquetes que ya tienes.
locations-kind-question = ¿Qué tipo de ubicación es?
locations-kind-folder = Una carpeta
locations-kind-folder-hint = En un disco o un recurso compartido de un NAS montado en el contenedor, o una que Dropbox, Google Drive, OneDrive o iCloud Drive mantengan sincronizada. Hoard puede guardar archivos ahí o vigilar paquetes que ya tienes.
locations-kind-folder-hint-desktop = En este ordenador, otro disco o un recurso compartido de un NAS, o una que Dropbox, Google Drive, OneDrive o iCloud Drive mantengan sincronizada. Hoard puede guardar archivos ahí o vigilar paquetes que ya tienes.
locations-kind-s3-hint = Un bucket de Amazon S3 o de un servicio parecido: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard guarda archivos ahí.
locations-s3-title = Almacenamiento S3
locations-s3-hint = Una ubicación en s3://bucket-name es un bucket de este servicio: AWS o cualquiera compatible con S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage en un NAS).
locations-bucket = Bucket
locations-bucket-hint = El nombre de un bucket que ya exista. Las claves de arriba tienen que poder listarlo, leerlo y escribir en él.
locations-s3-endpoint = Dirección
locations-s3-endpoint-placeholder = Vacía para AWS; p. ej. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Región
locations-s3-access-key = ID de clave de acceso
locations-s3-secret-key = Clave de acceso secreta
locations-s3-headers = Cabeceras adicionales
locations-s3-headers-placeholder = Name=value, una por línea (por ejemplo, para Cloudflare Access)
locations-s3-kept = Guardado (escribe para sustituirlo)
locations-s3-secrets = Las claves y las cabeceras se guardan con los ajustes del servidor y no se vuelven a mostrar.
locations-s3-secrets-desktop = Las claves y las cabeceras se guardan en el llavero de este ordenador y no se vuelven a mostrar.
locations-s3-saved = Almacenamiento S3 guardado
locations-s3-save-failed = No se ha podido guardar el almacenamiento S3
locations-add-more = Añadir una ubicación…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = No se han podido cargar las incidencias
issues-started =
    { $count ->
        [one] Se ha vuelto a iniciar.
       *[other] Se han vuelto a iniciar { $count } cosas.
    }
issues-retry-failed = No se ha podido reintentar
issues-dismiss-failed = No se ha podido descartar
issues-dismiss-all-title = ¿Descartar todas ({ $count })?
issues-dismiss-all-body = Se vacía «{ $title }». Si algo vuelve a fallar, se volverá a mostrar.
issues-dismiss-all = Descartar todas
issues-just-now = hace un momento
issues-minutes-ago = hace { $mins } min
issues-hours-ago = hace { $hours } h
issues-loading = Cargando…
issues-none = No ha fallado nada.
issues-action-all = { $action } (todas)
issues-open-asset = Abrirlo
issues-open-pack = Mostrar el paquete
issues-open-locations = Ubicaciones de la biblioteca
issues-where-before = en
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } vez
       *[other] { $count } veces
    }
issues-starting = Iniciando…
issues-dismiss-named = Descartar { $title }
issues-dismiss = Descartar
issues-show-fewer = Mostrar menos
issues-show-all = Mostrar todas ({ $count })
issues-not-listed =
    { $count ->
        [one] { $count } más no aparece en la lista.
       *[other] { $count } más no aparecen en la lista.
    }

## archive.ts

archive-anyway = Archivar de todos modos
archive-failed = No se ha podido archivar: { $error }
archive-undo = Deshacer
archive-restore-failed = No se ha podido restaurar: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } paquete
       *[other] { $packs } paquetes
    } y { $assets ->
        [one] { $assets } recurso
       *[other] { $assets } recursos
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } paquete
       *[other] { $packs } paquetes
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } recurso
       *[other] { $assets } recursos
    }
archive-count-nothing = nada

## ArchiveView.vue

archive-load-failed = No se ha podido cargar el Archivo: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Restaurado: { $what }
archive-purged = Eliminado definitivamente: { $what }
archive-purge-failed = No se ha podido eliminar: { $error }
archive-purge-title = ¿Eliminar { $name } definitivamente?
archive-purge-pack-body =
    { $count ->
        [one] Su { $count } recurso
       *[other] Sus { $count } recursos
    } y { $size } de archivos también se eliminan, salvo los archivos que comparta otro paquete. No se puede deshacer.
archive-purge = Eliminar para siempre
archive-purge-asset-body = Su archivo se libera, salvo que otro recurso lo comparta. No se puede deshacer.
archive-empty-title = ¿Vaciar el Archivo?
archive-empty-body = Se eliminará definitivamente lo siguiente: { $what }, y se liberarán sus archivos. No se puede deshacer.
archive-empty = Vaciar el Archivo
archive-title = Archivo
archive-intro = Los paquetes y recursos archivados quedan fuera de las búsquedas, los recuentos y la lista de etiquetas. Restáuralos, o elimínalos definitivamente para liberar su espacio.
archive-loading = Cargando…
archive-is-empty = El Archivo está vacío
archive-is-empty-hint = Archiva un paquete o un recurso y esperará aquí hasta que lo restaures o lo elimines definitivamente.
archive-packs = Paquetes
archive-assets = Recursos
archive-asset-count =
    { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
archive-archived = archivado { $when }
archive-from = de { $pack }
archive-restore = Restaurar
archive-show-more = Mostrar más (quedan { $count })

## mine.ts

projects-favorite-add-failed = No se ha podido añadir a tus favoritos: { $error }
projects-favorite-remove-failed = No se ha podido quitar de tus favoritos: { $error }
projects-save-search-failed = No se ha podido guardar la búsqueda: { $error }
projects-forget-search-failed = No se ha podido quitar { $name }: { $error }

## projects.ts

projects-list-and = { $items } y { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } más
projects-other =
    { $count ->
        [one] { $count } proyecto más
       *[other] { $count } proyectos más
    }
projects-usage-title-one = { $name } usa esto
projects-usage-title = { $count } proyectos usan esto
projects-usage-body-one = Se usa en { $list }: su próximo hoard sync los quitará.
projects-usage-body = Se usa en { $list }: el próximo hoard sync de cada uno los quitará.

## uploader.ts

upload-network-error = error de red
upload-gave-up = { $what }: { $error } tras { $attempts } intentos
upload-failed-plain = ha fallado
upload-already-imported = ya importado
upload-part-failed = la subida ha fallado ({ $status })
upload-batch =
    { $count ->
        [one] lote de { $count } archivo
       *[other] lote de { $count } archivos
    }
upload-batch-failed = el lote ha fallado ({ $status })
upload-finish = terminar la subida

## UploadQueue.vue

upload-eta-seconds = quedan { $s } s
upload-eta-minutes = quedan { $min } min
upload-eta-hours = quedan { $h } h
upload-queue = Importaciones
upload-importing =
    { $count ->
        [one] Importando { $count } paquete
       *[other] Importando { $count } paquetes
    }
upload-bytes-of = { $sent } de { $total }
upload-rate = { $size }/s
upload-waiting = En espera
upload-unpacking = Subido, descomprimiendo
upload-already = Ya está en la biblioteca
upload-stopped = Detenido
upload-files-of = { $done } de { $total } archivos
upload-resumed = reanudado, { $size } ya estaban allí
upload-skipped = Estos mismos archivos ya se importaron como «{ $name }», así que se han omitido.
upload-error = { $error }. Tu progreso se conserva: reintenta o vuelve a soltar los mismos archivos más tarde.
upload-retry = Reintentar
# $name is the pack the library has already that these files go into.
upload-update = Actualizar «{ $name }»
upload-copy = Importar como copia
# $name is the pack the library has already that the upload goes into.
upload-into = Va a «{ $name }», que ya está en la biblioteca: se añaden los archivos nuevos y se sustituyen los que han cambiado.
upload-own-pack = Convertirlo en un paquete propio
upload-dismiss = Descartar
upload-cancel = Cancelar
upload-minimize = Minimizar
upload-expand = Mostrar importaciones
upload-pill = Importando { $count } · { $percent } %
upload-pill-stopped = Detenidas: { $count }
upload-interrupted = Interrumpida
upload-resume = Reanudar
upload-resume-all = Reanudar todas
upload-resume-hint-folder = Vuelve a elegir la misma carpeta o suéltala en la página para continuar donde se detuvo.
upload-resume-hint-file = Vuelve a elegir el mismo archivo o suéltalo en la página para continuar donde se detuvo.
upload-not-same = No son los mismos archivos, así que no se ha reanudado nada.
upload-permission-denied = El navegador no ha tenido permiso para volver a leer los archivos. Elígelos tú.

## PacksView.vue

packs-rename-failed = No se ha podido cambiar el nombre: { $error }
packs-collection-deleted = Se ha eliminado la colección { $name }
packs-delete-failed = No se ha podido eliminar: { $error }
packs-status-uploading = Subiendo (archivos recibidos hasta ahora)
packs-status-ingesting = Descomprimiendo y etiquetando
packs-status-analyzing = El LLM está etiquetando
packs-status-ready = Listo
packs-status-failed = Error
packs-retag-failed = No se ha podido empezar a volver a etiquetar: { $error }
packs-read-files-failed = No se han podido leer los archivos del paquete: { $error }
packs-context-load-failed = No se han podido cargar los datos de tienda del paquete: { $error }
packs-undo-store-failed = No se ha podido quitar la página de la tienda: { $error }
packs-context-save-failed = No se han podido guardar los datos de tienda: { $error }
packs-unreal-load-failed = No se ha podido cargar el estado de la conversión de Unreal: { $error }
packs-unreal-discard-title = ¿Descartar los originales?
packs-unreal-discard-body = Los archivos .uasset originales de { $name } se eliminan una vez convertidos. Los datos exclusivos de Unreal que contienen, como los grafos de materiales, se pierden para siempre.
packs-unreal-discard-confirm = Convertir y descartar
packs-unreal-convert-failed = No se ha podido iniciar la conversión: { $error }
packs-unreal-fact-converted =
    { $done ->
        [one] { $done } convertido desde Unreal
       *[other] { $done } convertidos desde Unreal
    }
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } paquete de Unreal
       *[other] { $left } paquetes de Unreal
    }
packs-unreal-fact-both = { $done } convertidos desde Unreal, { $left } siguen siendo paquetes
packs-thumbs-canceled = Cancelado tras { $done } de { $total }
packs-thumbs-drew =
    { $count ->
        [one] Se ha dibujado { $count } modelo
       *[other] Se han dibujado { $count } modelos
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniatura sustituida.
       *[other] { $lead }: { $improved } miniaturas sustituidas.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniatura sustituida, { $failed } no se han podido dibujar.
       *[other] { $lead }: { $improved } miniaturas sustituidas, { $failed } no se han podido dibujar.
    }
packs-archived =
    { $count ->
        [one] Se ha archivado { $name } y su { $count } recurso
       *[other] Se ha archivado { $name } y sus { $count } recursos
    }
packs-blobs-bad =
    { $count ->
        [one] { $count } archivo no ha superado su última comprobación de integridad.
       *[other] { $count } archivos no han superado su última comprobación de integridad.
    }
packs-collections = Colecciones
packs-browse-named = Explorar { $name }
packs-collection-name = Nombre de la colección
packs-save = Guardar
packs-cancel = Cancelar
packs-assets =
    { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
packs-updated = Actualizada el { $date }
packs-browse = Explorar
packs-collection-download = Descargar (.zip)
packs-collection-credits = Créditos
packs-collection-credits-title = Quién hizo los paquetes de los que vienen estos recursos, sus licencias y las líneas de créditos que piden
packs-rename = Cambiar nombre
packs-delete = Eliminar
packs-packs = Paquetes
packs-empty = Todavía no hay paquetes. Suelta un zip o una carpeta en cualquier parte de esta página para añadir uno.
packs-find = Buscar un paquete
packs-find-placeholder = Busca un paquete por nombre, tienda, licencia o etiqueta
packs-sort = Orden
packs-sort-new = Más recientes primero
packs-sort-old = Más antiguos primero
packs-sort-name = Por nombre
packs-sort-size = Más grandes primero
packs-sort-assets = Con más recursos primero
packs-with-kind = Paquetes con
packs-state = Estado
packs-state-any = Cualquier estado
packs-state-working = En proceso
packs-state-failed = Con errores
packs-state-untagged = Sin describir todavía
packs-state-unlicensed = Sin licencia indicada
packs-store = Tienda
packs-store-any = Cualquier tienda
packs-store-from = De { $store }
packs-store-none = Sin página de tienda
packs-shown-of = { $shown } de { $total }
packs-clear = Borrar
packs-no-match = No hay paquetes que coincidan.
packs-clear-all = Borrar la búsqueda y los filtros
packs-pack-name = Nombre del paquete
packs-thumbs-drawing = Dibujando miniaturas en este navegador…
packs-thumbs-drawing-count = Dibujando miniaturas en este navegador: { $done } de { $total }…
packs-thumbs-stopped = Detenido: { $error }
packs-thumbs-none = Todos los modelos de aquí ya tienen una miniatura dibujada con todas sus texturas.
packs-dismiss = Descartar
packs-license = Licencia: { $license }
packs-license-tier = Licencia: { $license } ({ $tier })
packs-license-not-found = no encontrada
packs-from-before = De
packs-from-after = { "" }
packs-added = Añadido el { $date }
pack-panel-label = Acerca de este paquete
pack-panel-hide = Ocultar los detalles del paquete
pack-panel-kinds = Qué contiene
pack-panel-all = Todos los paquetes
packs-download = Descargar paquete (.zip)
packs-store-details = Datos de tienda
packs-more-named = Más acciones para { $name }
packs-more = Más
packs-rename-more = Cambiar nombre…
packs-retag-rules = Volver a ejecutar el etiquetador sin conexión
packs-retag-llm = Volver a ejecutar el etiquetador LLM
packs-make-thumbs = Crear miniaturas en este navegador
packs-make-thumbs-title = Abre cada modelo en un visor 3D oculto, de uno en uno, y usa su imagen como miniatura cuando es mejor que la del servidor
packs-make-thumbs-title-desktop = Abre cada modelo en un visor 3D oculto, de uno en uno, y usa su imagen como miniatura cuando es mejor que la que ha hecho Hoard
packs-read-files = Volver a leer los archivos del paquete
packs-credits = Descargar créditos
packs-credits-title = CREDITS.md: quién hizo el paquete, su licencia y la línea de créditos que pide
packs-unreal-open = Convertir contenido de Unreal…
packs-archive = Archivar paquete
packs-unreal-intro = Hoard lee los paquetes de Unreal por sí mismo y guarda lo que contienen como archivos normales: texturas en PNG o EXR, sonidos en WAV, materiales en JSON. El archivo convertido ocupa el lugar del paquete y conserva sus etiquetas, enlaces y colecciones.
packs-unreal-converted =
    { $count ->
        [one] convertido
       *[other] convertidos
    }
packs-unreal-converted-kept =
    { $count ->
        [one] convertido, { $kept } con el original conservado
       *[other] convertidos, { $kept } con el original conservado
    }
packs-unreal-pending =
    { $count ->
       *[other] sin revisar todavía
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] a la espera de Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] sin ningún formato común
    }
packs-unreal-failed =
    { $count ->
        [one] fallido
       *[other] fallidos
    }
packs-unreal-reasons = Por qué quedan paquetes
packs-unreal-editor-before = Las mallas y las animaciones necesitan Unreal Editor. En una máquina con Unreal instalado, ejecuta
packs-unreal-editor-command = hoard unreal-worker --editor <ruta a UnrealEditor-Cmd>
packs-unreal-editor-after = ; los exporta como FBX y sube aquí los resultados.
packs-unreal-discard = Descartar los archivos .uasset originales una vez convertidos
packs-unreal-discard-hint = Libera el espacio. Los datos exclusivos de Unreal, como los grafos de materiales, se pierden.
packs-unreal-drop = Eliminar los paquetes sin ningún formato común
packs-unreal-drop-hint = Blueprints, niveles y otros recursos exclusivos del motor.
packs-unreal-convert = Convertir
packs-context-hint-before = De dónde viene el paquete y qué dice de él el vendedor. El etiquetador LLM lo lee con cada archivo, así que una descripción de la tienda como «120 hechizos de hielo y fuego» le ayuda a ponerle nombre a
packs-context-hint-after = .
packs-store-auto = Encontrada automáticamente: el propio paquete nombra esta página.
packs-store-not-this = No es esta
packs-lookup-label = Buscar en las tiendas
packs-lookup-placeholder = El nombre del paquete en la tienda
packs-lookup-searching = Buscando…
packs-lookup-search = Buscar en tiendas
packs-lookup-hide = Ocultar
packs-lookup-clues = De los archivos del paquete: { $clues }.
packs-lookup-also = También se ha buscado { $queries }.
packs-lookup-quoted = «{ $query }»
packs-lookup-nothing = No hay nada en Fab, itch.io, Kenney ni OpenGameArt que se le parezca. Prueba con menos palabras o con otras.
packs-lookup-use = Usar
packs-lookup-searched = Búsqueda del { $date }.
packs-lookup-again = Volver a buscar
packs-lookup-again-after = .
packs-lookup-unreachable = Esta vez no se ha podido conectar con { $stores }.
packs-lookup-also-look = Busca también en
packs-lookup-links-and = { " " }y{ " " }
packs-lookup-browser-only = (solo se puede buscar en ellas desde tu navegador); pega abajo la dirección de la página.
packs-lookup-open = Buscar en tiendas…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Vendedor
packs-seller-placeholder = Quién lo hizo
packs-license-label = Licencia
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Nivel de licencia
packs-license-tier-placeholder = Personal, Profesional…
packs-store-page = Página de la tienda
packs-store-text = Descripción de la tienda
packs-store-text-placeholder = Pega el texto del anuncio, o déjalo vacío para obtenerlo de la página de la tienda.
packs-store-text-fetch = Al guardar se obtiene la descripción de la página de la tienda.
packs-readme = README encontrado en el paquete
packs-save-retag = Guardar y volver a ejecutar el etiquetador LLM

## AssetDetail.vue

asset-lazy-3d-viewer = el visor 3D
asset-lazy-text = el texto
asset-lazy-text-viewer = el visor de texto
asset-lazy-image-viewer = el visor de imágenes
asset-lazy-material-preview = la vista previa del material
asset-lazy-material-viewer = el visor de materiales
asset-lazy-sprite-player = el reproductor de sprites
asset-lazy-sheet-view = el visor de hojas de sprites
asset-rel-texture = textura
asset-rel-buffer = datos de geometría
asset-rel-material = archivo de material
asset-rel-animation = animación
asset-rel-library = archivo vinculado
asset-rel-guess = probablemente va con
asset-rel-sidecar = datos de tienda
asset-rel-variant = variante
asset-rel-sprite = sprite
asset-rel-sample = muestra
asset-role-map = mapa de { $role }
asset-maps-count =
    { $count ->
        [one] { $count } mapa
       *[other] { $count } mapas
    }
asset-needs = Necesita
asset-animations = Animaciones
asset-sprites = Sprites
asset-used-by = Usado por
asset-loose-sprites = También como archivos separados
asset-in-sheet = En una hoja de sprites
asset-other-formats = Otros formatos
asset-shown-as = Se muestra en la biblioteca como
asset-conversion-failed = la conversión ha fallado
asset-as-converting = Convirtiendo…
asset-as-converting-percent = Convirtiendo… { $percent } %
asset-as-queued = En cola…
asset-download = Descargar
asset-as-label = Descargar como
asset-as-original = { $format } (formato original)
asset-as-pack-files = Los archivos del paquete
asset-as-size = Tamaño
asset-as-full-size = Tamaño completo
asset-as-up-to = Hasta { $size } px
asset-as-hint = Elige un tamaño u otro formato
asset-meta-size = Tamaño
asset-meta-length = Duración
asset-meta-sample-rate = Frecuencia de muestreo
asset-meta-channels = Canales
asset-meta-mono = Mono
asset-meta-stereo = Estéreo
asset-meta-bit-depth = Profundidad de bits
asset-meta-bits = { $bits } bits
asset-meta-level = Nivel
asset-meta-level-value = pico { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = pico { $peak } dBFS, RMS { $rms } dBFS, satura
asset-meta-loop = Bucle
asset-meta-loop-value = de { $start } a { $end }
asset-meta-playback = Reproducción
asset-meta-one-shot = Una sola vez
asset-meta-envelope = Envolvente
asset-meta-envelope-times = ataque { $attack }, cola { $tail }
asset-meta-envelope-hit = golpe
asset-meta-envelope-swell = crescendo
asset-meta-envelope-sustained = sostenido
asset-meta-tone = Timbre
asset-meta-tone-value = brillo { $khz } kHz, ruido { $noise } %
asset-meta-pitch = Altura
asset-meta-pitch-rises =
    { $octaves ->
        [one] sube { $octaves } octava
       *[other] sube { $octaves } octavas
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] baja { $octaves } octava
       *[other] baja { $octaves } octavas
    }
asset-meta-loops-cleanly = Se repite sin cortes
asset-meta-tempo = Tempo
asset-meta-tempo-heard = unos { $bpm } BPM, de oído
asset-meta-tempo-heard-or = unos { $bpm } BPM (o { $alt }), de oído
asset-meta-key = Tonalidad
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, de oído
music-key-major = { $note } mayor
music-key-minor = { $note } menor
asset-meta-root-note = Nota fundamental
asset-meta-cue-markers = Marcadores
asset-meta-library = Biblioteca
asset-meta-keywords = Palabras clave
asset-meta-title = Título
asset-meta-artist = Artista
asset-meta-album = Álbum
asset-meta-genre = Género
asset-meta-generator = Generador
asset-meta-everything-with = Mostrar todo lo que tiene la etiqueta { $tag }
asset-meta-saved-with = Guardado con
asset-meta-color-mode = Modo de color
asset-meta-layers = Capas
asset-meta-range = Rango
asset-meta-hdr = HDR (la vista previa tiene mapeo de tonos)
asset-meta-format = Formato
asset-meta-large-document = Documento grande (PSB)
asset-meta-ai = IA
asset-meta-ai-made = Hecho con IA generativa
asset-meta-ai-edited = Editado con IA generativa
asset-meta-ai-how-metadata = sus metadatos
asset-meta-ai-how-generator = la configuración del generador
asset-meta-ai-how-iptc = el tipo de fuente digital IPTC
asset-meta-ai-says = { $made }, según { $how }
asset-meta-ai-by-says = { $made } ({ $by }), según { $how }
asset-meta-provenance = Procedencia
asset-meta-c2pa = Tiene credenciales de contenido C2PA
asset-meta-compression = Compresión
asset-meta-note = Nota
asset-meta-maps = Mapas
asset-meta-sizes =
    { $count ->
        [one] Tamaño
       *[other] Tamaños
    }
asset-meta-triangles = Triángulos
asset-meta-vertices = Vértices
asset-meta-meshes = Mallas
asset-meta-materials = Materiales
asset-meta-animations = Animaciones
asset-meta-skins = Skins
asset-meta-unreal-asset = Recurso de Unreal
asset-meta-unreal-saved-with = { $class }, guardado con UE { $engine }
asset-meta-package = Paquete
asset-meta-cooked = Cocinado (build del juego): sin vistas previas ni fuentes
asset-meta-tiling = Repetición
asset-meta-seamless = Sin costuras en ambos ejes
asset-meta-analysis-error = No se ha podido leer este archivo por completo: { $error }
asset-copy-blocked = No se ha podido copiar: el navegador lo ha bloqueado
asset-save-error = No se ha podido guardar: { $error }
asset-tag-removed = Se ha quitado { $tag }
asset-undo = Deshacer
asset-play-error = No se ha podido reproducir este archivo: { $error }
asset-copied-hash = Hash copiado
asset-copied-toml = toml copiado
asset-copied-id = ID copiado
asset-copied-link = Enlace copiado
asset-origin-before = Este archivo era
asset-origin-class-engine = , un { $class } guardado por UE { $engine }.
asset-origin-class = , un { $class }.
asset-origin-engine = { " " }guardado por UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor lo exportó el { $date }.
asset-origin-editor-notes = Unreal Editor lo exportó el { $date } ({ $notes }).
asset-origin-hoard = Hoard lo extrajo del paquete el { $date }.
asset-origin-hoard-notes = Hoard lo extrajo del paquete el { $date } ({ $notes }).
asset-origin-download = Descargar el .{ $ext } original
asset-origin-discarded = El paquete original se descartó después de la conversión.
asset-details = Detalles del recurso
asset-in-pack = en
asset-previous = Recurso anterior
asset-next = Recurso siguiente
asset-favorite = Favorito
asset-favorite-on-title = En tus favoritos (F lo quita)
asset-favorite-off-title = Añadir a tus favoritos (F)
asset-share = Compartir
asset-share-title = Compartir un enlace a este recurso (quien lo abra inicia sesión en Hoard primero)
asset-layout-panel = Mostrar en un panel lateral
asset-layout-page = Mostrar a página completa
asset-layout-page-title = Mostrar a página completa: una vista previa grande junto a los detalles
asset-close = Cerrar detalles
asset-rename-title = Haz clic para cambiar el nombre
asset-name = Nombre
asset-views-alt = El modelo desde cuatro direcciones
asset-inspect-aria = Inspeccionar { $name } a pantalla completa
asset-inspect-title = Inspeccionar: zoom, mosaico, canales
asset-playback-position = Posición de reproducción
asset-playback-value = { $current } de { $duration }
asset-pause = Pausar
asset-play = Reproducir
asset-waveform-drawing = Dibujando la forma de onda…
asset-waveform-skipped = No hay forma de onda para archivos tan grandes
asset-play-animation = Reproducir como animación
asset-show-views = Mostrar la hoja de cuatro vistas que vio el etiquetador
asset-palette = Colores de la imagen
asset-swatch-title = { $hex }, { $percent } % de la imagen. Buscar recursos con este color
asset-swatch-aria = Buscar recursos con { $hex }, { $percent } % de esta imagen
asset-more-like-this = Más como este
asset-desc-label = Tu descripción
asset-desc-placeholder = Qué es, dónde lo usarías…
asset-save = Guardar
asset-cancel = Cancelar
asset-desc-family = Se guarda en las { $count } variantes
asset-desc-edit = Editar descripción
asset-desc-write = Escribir una descripción
asset-desc-ai = Escrita por IA
asset-desc-file = Del archivo
asset-desc-store-before = De la
asset-desc-store-page = página de { $store }
asset-desc-store-page-unnamed = página de la tienda
asset-desc-store-after = (todo el paquete)
asset-less = Menos
asset-more = Más
asset-tags = Etiquetas
asset-tag-add-label = Añadir una etiqueta
asset-tag-add-placeholder = Añade una etiqueta, p. ej. mood:tense
asset-tag-add = Añadir
asset-apply-family = Aplicar los cambios de etiquetas, categoría y descripción a las { $count } variantes
asset-collections = Colecciones
asset-collection-remove = Quitar de { $name }
asset-collection-add-another = Añadir a otra
asset-collection-add = Añadir a una colección
asset-used-in = Se usa en
asset-used-in-title = Mostrar lo que tiene { $name }
asset-used-in-synced = sincronizado { $when }
asset-maps = Mapas
asset-show-more = Mostrar { $count } más
asset-materials-heading =
    { $count ->
        [one] Material
       *[other] Materiales
    }
asset-material-maps-aria = { $name }: mapas
asset-variants = Variantes ({ $count } tomas)
asset-fit-animations = Animaciones compatibles ({ $count })
asset-fit-characters = Personajes compatibles ({ $count })
asset-copies = Copias idénticas
asset-copies-note = Los mismos bytes se guardan una sola vez; estos paquetes también incluyen este archivo.
asset-near-copies = Casi copias
asset-near-copies-note = Lo mismo en otro tamaño, frecuencia de muestreo o formato. La mejor copia está marcada.
asset-near-copies-best = Lo mismo en otro tamaño, frecuencia de muestreo o formato. Esta es la mejor copia.
asset-near-best = la mejor
asset-unreal-converted = Convertido desde Unreal
asset-rule-converted = Convertido por una regla
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Una regla de conversión creó este { $format } el { $date } a partir de
asset-rule-kept-until = El original se conserva hasta el { $date }.
asset-rule-rebuilt = El FLAC guarda todo lo que había en el WAV, así que el WAV se puede descargar o recuperar tal como era en cualquier momento.
asset-rule-put-back = Restaurar el original
asset-rule-put-back-title = ¿Restaurar { $file }?
asset-rule-put-back-body = El archivo convertido se elimina y el original ocupa su lugar, con sus etiquetas, colecciones y enlaces. A partir de entonces, las reglas de conversión no lo vuelven a tocar.
asset-rule-put-back-done = Se ha restaurado { $file }
asset-rule-let-go = El original se eliminó cuando venció el plazo durante el que se conservan los originales según Ajustes.
asset-unreal-package = Del paquete de Unreal
asset-ue-needs-editor-before = A la espera de Unreal Editor: { $reason }. En una máquina con Unreal instalado, ejecuta
asset-ue-needs-editor-after = y vuelve como FBX.
asset-ue-editor-path = ruta a UnrealEditor-Cmd
asset-ue-unsupported = Se conserva como paquete: { $reason }.
asset-ue-failed = La conversión ha fallado: { $reason }
asset-recovered = El .{ $ext } original estaba guardado dentro de este paquete, y Hoard lo ha extraído.
asset-recovered-download = Descargar .{ $ext }
asset-ue-meshes-before = Las mallas y las animaciones se guardan en el formato propio de Unreal. Convierte el paquete (página del paquete, Más) y luego ejecuta
asset-ue-meshes-after = junto a una instalación de Unreal Editor para obtener FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } etiqueta del registro de recursos
       *[other] { $count } etiquetas del registro de recursos
    }
asset-file = Archivo
asset-bytes = Bytes
asset-source = Origen
asset-path = Ruta
asset-path-open-folder = Ver qué más hay en { $folder }
asset-download-maps =
    { $count ->
        [one] Descargar { $count } mapa
       *[other] Descargar los { $count } mapas
    }
asset-download-linked =
    { $count ->
        [one] Descargar con { $count } archivo vinculado
       *[other] Descargar con { $count } archivos vinculados
    }
asset-download-only-title = Solo este archivo, sin los archivos que usa
asset-download-only = Solo este archivo
asset-include-animations = Incluir animaciones en el .zip
asset-include-character = Incluir el personaje { $name }
asset-copy-toml = Copiar entrada de hoard.toml
asset-copy-id = Copiar ID
asset-teach = Enseñar este archivo a Hoard
asset-teach-title = Describe el archivo de esta tienda para que Hoard lo lea (Ajustes → Archivos de tienda)
asset-archive = Archivar
asset-archive-title = Fuera de las búsquedas hasta que se restaure o se elimine definitivamente, desde el Archivo

## ImageViewer.vue

image-channel-red = Canal rojo
image-channel-green = Canal verde
image-channel-blue = Canal azul
image-channel-alpha = Canal alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Fondo de cuadros (K cambia)
image-bg-black-title = Fondo negro (K cambia)
image-bg-gray-title = Fondo gris (K cambia)
image-bg-white-title = Fondo blanco (K cambia)
image-bg-checker = fondo de cuadros
image-bg-black = fondo negro
image-bg-gray = fondo gris
image-bg-white = fondo blanco
image-inspect-aria = Inspeccionar { $name }
image-preview-size = vista previa { $width }×{ $height }, { $bytes }
image-preview = vista previa
image-resolution = Resolución
image-full-title = Cargar la imagen completa de { $width }×{ $height } (F); también se carga al ampliar más allá de la vista previa
image-full-loading = Cargando la resolución completa…
image-full-failed = La resolución completa no se ha cargado: reintentar
image-full = Resolución completa
image-zoom = Zoom
image-zoom-out-title = Alejar (−)
image-zoom-out = Alejar
image-zoom-in-title = Acercar (+)
image-zoom-in = Acercar
image-fit-title = Ajustar (0)
image-fit = Ajustar
image-actual-title = Píxeles reales (1)
image-view = Vista
image-tile-title = Repetir en mosaico 3×3 para comprobar las costuras (T)
image-tile = Mosaico
image-channels = Canales
image-close = Cerrar (Esc)
image-failed = No se ha podido cargar esta imagen.
image-loading = Cargando…
image-key-gestures = Desplaza o pellizca para hacer zoom · arrastra para mover · doble clic para acercar o ajustar
image-key-tile = mosaico
image-key-background = fondo
image-key-full = resolución completa
image-key-channels = canales

## SpritePlayer.vue

sprite-frame-aria = { $name }, fotograma { $frame } de { $count }
sprite-failed = Este no se puede reproducir.
sprite-loading = Cargando fotogramas…
sprite-previous = Fotograma anterior
sprite-next = Fotograma siguiente
sprite-pause = Pausar animación
sprite-play = Reproducir animación
sprite-animation = Animación
sprite-all-frames = Todos los fotogramas
sprite-grid-title = Cómo se corta la hoja en fotogramas
sprite-columns = Columnas
sprite-rows = Filas
sprite-fps-aria = Fotogramas por segundo
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
        [one] { $count } página
       *[other] { $count } páginas
    }
sheet-animations =
    { $count ->
        [one] { $count } animación
       *[other] { $count } animaciones
    }
# The box that narrows the list of sprites by name.
sheet-find = Buscar un sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Mostrando { $shown } de { $total }
sheet-no-match = Ningún sprite tiene eso en su nombre.
sheet-failed = Esta hoja no se puede leer.
sheet-loading = Cargando la hoja…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Formato propio de Hoard: las páginas tal cual, los sprites girados en cualquier sentido y recortados, con animaciones y la duración de cada fotograma.
sheet-note-texturepacker = Sprites girados en sentido horario y recortados, con animaciones por nombre, tal como los leen PixiJS y Phaser.
sheet-note-clockwise = Sprites girados en sentido horario y recortados. No admite animaciones: los fotogramas se identifican por sus nombres numerados.
sheet-note-libgdx = Sprites girados en sentido antihorario y recortados. No admite animaciones: los fotogramas se identifican por sus nombres numerados.
sheet-note-godot-frames = Los sprites nunca se giran y el recorte se conserva como márgenes, con animaciones y la duración de cada fotograma.
sheet-note-godot-tileset = Los tiles nunca se giran ni se recortan, en una cuadrícula del tamaño del tile; las imágenes que ya están en una se quedan como están.
sheet-note-grid = Cada sprite derecho sobre todo su lienzo, en celdas del mismo tamaño, para cualquier herramienta que corte una hoja en cuadrícula.
sheet-note-frames = Cada sprite derecho sobre todo su lienzo, cada uno en un PNG con su nombre.

## TextViewer.vue, TextPreview.vue y text.ts

text-lang-config = Configuración
text-lang-table = Tabla
text-lang-code = Código
text-lang-script = Script
text-lang-text = Texto
text-copy-blocked = No se ha podido copiar: el navegador lo ha bloqueado
text-read-aria = Leer { $name }
text-lines =
    { $count ->
        [one] { $count } línea
       *[other] { $count } líneas
    }
text-show-as = Mostrar como
text-tree-title = Árbol plegable (T)
text-tree = Árbol
text-pretty-title = Con nueva sangría (P)
text-pretty = Formateado
text-raw-title = Tal como está escrito (R)
text-raw = Sin formato
text-document-title = Como documento (D)
text-document = Documento
text-source-title = El markdown tal como está escrito (R)
text-source = Código fuente
text-page-title = Como la muestra un navegador desde su carpeta, aparte de Hoard (P)
text-page = Página
text-page-source-title = La página tal como está escrita (R)
text-page-aria = { $name } como página
text-page-error = No se ha podido abrir esta página: { $error }
text-fold = Plegar
text-expand-all-title = Desplegarlo todo
text-expand-all = Desplegar todo
text-collapse-all-title = Plegarlo todo
text-collapse-all = Plegar todo
text-group-text = Texto
text-wrap-title = Ajustar las líneas largas (W)
text-wrap = Ajustar
text-copy-title = Copiar el texto (C)
text-copied = Copiado
text-copy = Copiar
text-download-title = Descargar el archivo
text-download = Descargar
text-close = Cerrar (Esc)
text-truncated = Se muestra el principio: { $shown } de { $total }.
text-truncated-json = Se muestra el principio: { $shown } de { $total } (el árbol necesita el archivo completo).
text-show-all = Mostrar todo
text-json-error = Este JSON no se puede analizar: { $error }
text-contents-aria = Contenido de { $name }
text-loading = Cargando…
text-load-error = No se ha podido cargar este archivo: { $error }
text-binary = Este archivo es binario, no de texto, así que aquí no hay nada que leer. Descárgalo para abrirlo con su propia herramienta.
text-key-tree = árbol
text-key-pretty = formateado
text-key-raw = sin formato
text-key-document = documento
text-key-source = código fuente
text-key-page = página
text-key-wrap = ajustar
text-key-copy = copiar
text-key-find = buscar
text-key-close = cerrar
text-preview-aria = Leer { $name } a pantalla completa
text-preview-read = Leer

## JsonTree.vue y JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementos
    }
json-keys =
    { $count ->
        [one] { $count } clave
       *[other] { $count } claves
    }
json-fold = Plegar
json-unfold = Desplegar
json-unfold-summary = Desplegar { $summary }
json-show-more = Mostrar { $count } más
json-left = de { $count } restantes
json-picker-more = … { $count } más

## Visores de modelos 3D (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = el visor 3D no se ha cargado ({ $error }); puede que recargar la página ayude
model-loading-viewer = Cargando el visor 3D…
model-loading = Cargando el modelo…
model-show-failed = No se ha podido mostrar este modelo: { $error }
model-preview-failed = No se ha podido previsualizar este modelo: { $error }
model-error-converting = el servidor todavía está convirtiendo este modelo
model-error-converting-desktop = Hoard todavía está convirtiendo este modelo
model-error-empty-collada = archivo Collada vacío
model-error-no-loader = El navegador no tiene cargador para .{ $ext }
model-dialog = Ver { $name } en 3D
model-title-stats =
    { $triangles } tris · { $meshes ->
        [one] { $meshes } malla
       *[other] { $meshes } mallas
    } · { $materials ->
        [one] { $materials } material
       *[other] { $materials } materiales
    }
model-triangles =
    { $count ->
        [one] { $count } triángulo
       *[other] { $count } triángulos
    }
model-group-camera = Cámara
model-group-shading = Sombreado
model-group-background = Fondo
model-group-animation = Animación
model-group-view = Vista
model-group-show = Mostrar
model-frame-title = Encuadrar el modelo (F)
model-frame = Encuadrar
model-reset-title = Volver a la vista inicial (R)
model-reset = Restablecer
model-turn-title = Girar despacio (A)
model-turn = Girar
model-textures-title = Texturas (T)
model-textures = Texturas
model-full-res-title = Texturas a resolución completa (H): cada archivo de textura tal cual, por grande que sea; desactivado, vistas previas ligeras de 2048 px como máximo
model-full-res = Res. completa
model-light-title = Luz del entorno (E)
model-light = Luz
model-grid-title = Cuadrícula del suelo (G)
model-grid = Cuadrícula
model-bg-checker = Fondo de cuadros
model-bg-dark = Fondo oscuro
model-bg-gray = Fondo gris
model-bg-light = Fondo claro
model-bg-env = Fondo del entorno
model-bg-title = { $background } (K cambia)
model-pause-title = Pausar (Espacio)
model-play-title = Reproducir (Espacio)
model-pause = Pausar
model-play = Reproducir
model-animation-clip = Clip de animación
model-animation = Animación
model-plays-on = Se reproduce en
model-rest-pose = Pose de reposo
model-close-title = Cerrar (Esc)
model-full-screen = Pantalla completa
model-open-full-screen = Abrir el visor 3D a pantalla completa
model-menu-fold = Recoger el menú
model-menu-show = Mostrar el menú
model-view-prefix = Vista:{ " " }
model-menu-render = Renderizado
model-menu-maps = Mapas
model-menu-inspect = Inspeccionar
model-mode-final = Renderizado final
model-mode-raw = Renderizado sin procesar
model-mode-color = Color base
model-mode-roughness = Rugosidad
model-mode-metalness = Metalicidad
model-mode-normal = Normales
model-mode-ao = Oclusión
model-mode-matcap = Matcap
model-mode-uv = Comprobador de UV
model-mode-geometry = Geometrías
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Malla de alambre (W), sobre cualquier vista
model-wireframe = Malla de alambre
model-info-title = Información de la escena (I)
model-info = Información de la escena
model-info-close = Cerrar la información de la escena
model-info-close-title = Cerrar (I)
model-info-geometries = Geometrías
model-info-materials = Materiales
model-info-textures = Texturas
model-info-triangles = Triángulos
model-info-vertices = Vértices
model-info-meters = glTF está en metros
model-info-no-units = Este formato no indica en qué unidades está
model-info-height = Alto
model-info-length = Largo
model-info-width = Ancho
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } unidad
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } unidades
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = No se ha encontrado la textura: { $name }
model-missing-some = No se han encontrado { $count } texturas: { $names }
model-missing-more = No se han encontrado { $count } texturas: { $names }…
model-missing-count = No se han encontrado { $count } texturas
model-guessed = Texturas emparejadas por nombre; el archivo no nombra ninguna.
model-group-facing = Orientación
model-facing-turn = Girar un cuarto
model-facing-turn-title = Girar el modelo un cuarto de vuelta en sentido antihorario, visto desde arriba
model-facing-tip = Inclinar un cuarto
model-facing-tip-title = Inclinar la parte superior del modelo un cuarto de vuelta hacia delante, para poner de pie uno que está tumbado
model-facing-save = Usar en este paquete
model-facing-save-title = Girar así todos los modelos de { $pack }. Sus miniaturas se vuelven a dibujar; las descargas se quedan como se hicieron.
model-facing-undo = Deshacer
model-facing-undo-title = Volver a como están girados los modelos del paquete
model-facing-saved = Todos los modelos de { $pack } están ahora orientados así. Sus miniaturas se están volviendo a dibujar.
model-facing-failed = No se han podido girar los modelos del paquete: { $error }
model-keys-mouse = Arrastra para orbitar · arrastra con el botón derecho para mover · desplaza para hacer zoom · doble clic para encuadrar
model-key-frame = encuadrar
model-key-reset = restablecer
model-key-view = vista
model-key-wireframe = malla de alambre
model-key-info = información de la escena
model-key-textures = texturas
model-key-full-res = texturas a resolución completa
model-key-light = luz
model-key-grid = cuadrícula
model-key-background = fondo
model-key-play = reproducir

## Materiales (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Color
material-role-normal = Normales
material-role-normal-dx = Normales (DirectX)
material-role-roughness = Rugosidad
material-role-gloss = Brillo
material-role-metalness = Metalicidad
material-role-ao = Oclusión ambiental
material-role-orm = Oclusión, rugosidad, metal
material-role-mra = Metal, rugosidad, oclusión
material-role-rma = Rugosidad, metal, oclusión
material-role-height = Altura
material-role-bump = Relieve
material-role-cavity = Cavidad
material-role-specular = Especular
material-role-opacity = Opacidad
material-role-emissive = Emisión
material-role-translucency = Translucidez
material-role-base-color-name = color
material-role-normal-name = normales
material-role-normal-dx-name = normales (DirectX)
material-role-roughness-name = rugosidad
material-role-gloss-name = brillo
material-role-metalness-name = metalicidad
material-role-ao-name = oclusión ambiental
material-role-orm-name = oclusión, rugosidad y metal
material-role-mra-name = metal, rugosidad y oclusión
material-role-rma-name = rugosidad, metal y oclusión
material-role-height-name = altura
material-role-bump-name = relieve
material-role-cavity-name = cavidad
material-role-specular-name = especular
material-role-opacity-name = opacidad
material-role-emissive-name = emisión
material-role-translucency-name = translucidez
material-maps =
    { $count ->
        [one] { $count } mapa
       *[other] { $count } mapas
    }
material-dialog = Ver el material { $name }
material-up-to = hasta { $size }
material-repeat-size = { $size } por repetición
material-shape-sphere = Esfera
material-shape-cube = Cubo
material-shape-plane = Plano
material-shape-title = { $shape } ({ $key })
material-group-shape = Forma
material-group-tiling = Repetición
material-real-title = Tamaño real: una repetición mide { $size }, la esfera 2 m de diámetro (T cambia)
material-repeat-title = Repetir { $n }× (T cambia)
material-real = Real
material-repeat = { $n }×
material-height-title = Cuánto desplaza la superficie el mapa de altura
material-height = Altura
material-height-strength = Intensidad de la altura
material-loading = Cargando el material…
material-show-failed = No se ha podido mostrar este material: { $error }
material-preview-failed = No se ha podido previsualizar este material: { $error }
material-group-maps = Mapas
material-map-show = Mostrar el mapa de { $map }
material-map-hide = Ocultar el mapa de { $map }
material-map-unused = La vista previa no lo usa
material-open-full-screen = Abrir el visor de materiales a pantalla completa
material-keys-mouse = Arrastra para orbitar · arrastra con el botón derecho para mover · desplaza para hacer zoom · haz clic en un mapa para desactivarlo
material-key-shape = forma
material-key-tiling = repetición
material-key-turn = girar

## theme.ts

theme-slate = Pizarra
theme-paper = Papel
theme-linen = Lino
theme-sage = Salvia
theme-lilac = Lila
theme-blush = Rubor
theme-graphite = Grafito
theme-obsidian = Obsidiana
theme-ember = Ascua
theme-moss = Musgo
theme-aubergine = Berenjena
theme-oxblood = Granate
theme-light-slate-blurb = Gris azulado frío
theme-light-paper-blurb = Gris neutro sencillo
theme-light-linen-blurb = Crema cálido, ámbar
theme-light-sage-blurb = Verde suave
theme-light-lilac-blurb = Violeta pálido
theme-light-blush-blurb = Rosa empolvado
theme-dark-graphite-blurb = Gris neutro, sin matiz
theme-dark-slate-blurb = Gris azulado
theme-dark-obsidian-blurb = Casi negro, para OLED
theme-dark-ember-blurb = Carbón cálido, ámbar
theme-dark-moss-blurb = Negro verdoso de bosque
theme-dark-aubergine-blurb = Negro ciruela, lila
theme-dark-oxblood-blurb = Negro vino, coral

## texnames.ts

texnames-needs-map = El patrón necesita { "{" }map{ "}" }, o todos los mapas de un material tendrían el mismo nombre
texnames-placeholders = El patrón puede usar { "{" }name{ "}" }, { "{" }map{ "}" } y { "{" }size{ "}" }
texnames-bad-characters = Los nombres no pueden tener < > : " / \ | ? ni *
texnames-no-suffix = Asigna un sufijo a los mapas de { $role }
texnames-bad-suffix = El sufijo de los mapas de { $role } no puede tener < > : " / \ | ? ni *
texnames-slot-no-suffix = Asigna un sufijo a «{ $slot }»
texnames-slot-bad-suffix = El sufijo de «{ $slot }» no puede tener < > : " / \ | ? ni *
texnames-ending-missing = Cada terminación tuya necesita la terminación que se va a buscar
texnames-ending-bad = Tus terminaciones no pueden tener < > : " / \ | ? ni *
texnames-ending-twice = { $ending } está dos veces en tus terminaciones
texnames-slot-metallic-roughness = Metal y rugosidad
texnames-slot-specular-color = Color especular
texnames-slot-specular-glossiness = Especular y brillo
texnames-slot-clearcoat = Barniz
texnames-slot-clearcoat-roughness = Rugosidad del barniz
texnames-slot-clearcoat-normal = Normales del barniz
texnames-slot-sheen-color = Color del satinado
texnames-slot-sheen-roughness = Rugosidad del satinado
texnames-slot-transmission = Transmisión
texnames-slot-thickness = Grosor
texnames-slot-diffuse-transmission = Transmisión difusa
texnames-slot-diffuse-transmission-color = Color de la transmisión difusa
texnames-slot-iridescence = Iridiscencia
texnames-slot-iridescence-thickness = Grosor de la iridiscencia
texnames-slot-anisotropy = Anisotropía

## storeFormats.ts

storefiles-field-title = Título
storefiles-field-title-hint = El título exacto del anuncio. Da nombre al paquete, y sin él el archivo se omite.
storefiles-field-seller = Vendedor
storefiles-field-seller-hint = Quién lo hizo o lo vende.
storefiles-field-license = Licencia
storefiles-field-license-hint = Tal como lo escribe el archivo, como CC-BY-4.0.
storefiles-field-url = Página de la tienda
storefiles-field-url-hint = La dirección del anuncio, que empieza por http:// o https://.
storefiles-field-words = Etiquetas
storefiles-field-words-hint = Palabras con las que etiquetarlo. Los valores separados por comas se dividen.
storefiles-field-type = Tipo
storefiles-field-type-hint = Qué es, como «3D model» o «surface». También es una etiqueta.
storefiles-field-colors = Colores
storefiles-field-colors-hint = Sus colores, como etiquetas color:.
storefiles-field-id = ID en la tienda
storefiles-field-id-hint = El ID que le da la tienda: letras, dígitos, _ o -.
storefiles-field-origin = Origen
storefiles-field-origin-hint = Dónde se hizo o se escaneó, para la descripción.
storefiles-field-size = Tamaño
storefiles-field-size-hint = Lo grande que es, en palabras, para la descripción.
storefiles-field-title-name = el título
storefiles-field-seller-name = el vendedor
storefiles-field-license-name = la licencia
storefiles-field-url-name = la página de la tienda
storefiles-field-words-name = las etiquetas
storefiles-field-type-name = el tipo
storefiles-field-colors-name = los colores
storefiles-field-id-name = el ID en la tienda
storefiles-field-origin-name = el origen
storefiles-field-size-name = el tamaño

## StoreFormatsPanel.vue

storefiles-load-failed = No se han podido cargar los formatos de archivos de tienda: { $error }
storefiles-remove-title = ¿Quitar { $name }?
storefiles-remove-body = { $file } se elimina de la carpeta de datos. Los paquetes que ya se leyeron con él conservan lo que leyó.
storefiles-remove = Quitar
storefiles-removed = Se ha quitado { $name }.
storefiles-remove-failed = No se ha podido quitar { $name }: { $error }
storefiles-unread = Archivos que Hoard no lee
storefiles-loading = Cargando…
storefiles-unread-none = Ninguno: ningún archivo de la biblioteca parece de una tienda sin leer.
storefiles-unread-what = Archivos que están en más de un paquete, o con nombre de archivo de tienda, que ningún formato lee.
storefiles-unread-packs =
    { $count ->
        [one] { $count } paquete
       *[other] { $count } paquetes
    }
storefiles-unread-example = p. ej. { $pack }
storefiles-teach = Enseñar este archivo a Hoard
storefiles-formats = Formatos
storefiles-formats-what = Los tuyos se prueban primero y sustituyen a uno integrado con el mismo nombre.
storefiles-dir-before = Se guardan en
storefiles-dir-after = .
storefiles-yours = Tuyo
storefiles-built-in = Integrado
storefiles-edit = Editar
storefiles-start-from = Partir de este

## TagsPanel.vue

tags-load-failed = No se han podido cargar las etiquetas: { $error }
tags-rename-title = ¿Cambiar el nombre de { $tag }?
tags-merge-title = ¿Fusionar { $count } etiquetas?
tags-rename-body =
    { $count ->
        [one] Su { $count } recurso pasa a tener { $into }, y los etiquetadores usan el nuevo nombre a partir de ahora.
       *[other] Sus { $count } recursos pasan a tener { $into }, y los etiquetadores usan el nuevo nombre a partir de ahora.
    }
tags-merge-body = { $tags } pasan a ser { $into } en hasta { $count } recursos. Los etiquetadores usan { $into } a partir de ahora.
tags-rename = Cambiar nombre
tags-merge = Fusionar
tags-merged =
    { $count ->
        [one] { $count } recurso etiquetado ahora con { $into }
       *[other] { $count } recursos etiquetados ahora con { $into }
    }
tags-merge-failed = No se ha podido fusionar: { $error }
tags-forget-failed = No se ha podido cambiar: { $error }
tags-find = Buscar etiquetas
tags-find-placeholder = Busca etiquetas, p. ej. rock o set:
tags-count =
    { $count ->
        [one] { $count } etiqueta
       *[other] { $count } etiquetas
    }
tags-count-more = Más de { $count } etiquetas
tags-what = Elige etiquetas que signifiquen lo mismo y fusiónalas, o elige una para cambiarle el nombre.
tags-new-name = Nombre nuevo
tags-merge-into = Fusionar en
tags-saved-as-before = Guardada como
tags-saved-as-after = { "" }
tags-cancel = Cancelar
tags-loading = Cargando…
tags-none = No hay etiquetas.
tags-none-containing = No hay etiquetas que contengan «{ $text }».
tags-browse-title = Explorar los recursos etiquetados con { $tag }
tags-browse = Explorar
tags-merged-heading = Etiquetas fusionadas
tags-merged-what = Cuando los etiquetadores proponen uno de estos nombres antiguos, el recurso recibe la etiqueta con la que se fusionó.
tags-allow-old = Volver a permitir el nombre antiguo

## LibraryStats.vue

stats-source-human = Añadidas por personas
stats-source-ai = Sugeridas por la IA
stats-source-rule = Reglas integradas
stats-source-meta = De los archivos
stats-assets = Recursos
stats-packs = Paquetes
stats-on-disk = En disco
stats-duplicates-title = { $count } archivos son copias idénticas byte a byte de otros y se guardan una sola vez
stats-saved-dedup = Ahorrado con la deduplicación
stats-distinct-tags = Etiquetas distintas
stats-archive = En el Archivo
stats-load-failed = No se han podido cargar los detalles: { $error }
stats-counting = Contando…
stats-as-of = A { $time }
stats-updating = Actualizando…
stats-by-kind = Espacio por tipo
stats-files-size = { $name }: { $count } archivos, { $size }
stats-months = Añadido en el último año
stats-month = { $month }: { $count } recursos, { $size }
stats-tagging = Etiquetado
stats-tagged = Etiquetados
stats-untagged = { $count } sin etiquetas
stats-described = Descritos
stats-described-assets =
    { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
stats-human-edited = Editados a mano
stats-human-edited-unit =
    { $count ->
        [one] recurso
       *[other] recursos
    }
stats-source-different = { $count } distintas
stats-most-used = Más usadas
stats-show-tag = Mostrar los recursos con { $tag }
stats-file-types = Tipos de archivo
stats-biggest-packs = Paquetes más grandes
stats-browse-pack = Explorar { $name }
stats-largest-files = Archivos más grandes
stats-open-file = Abrir { $name }
stats-empty = Todavía no hay nada en la biblioteca. Añade un paquete y sus cifras aparecerán aquí.

## HoardPanel.vue

hoard-title = Tu tesoro
hoard-what = Cada recurso de la biblioteca es una pieza del tesoro, y su tipo decide cuál. Recoge lo nuevo para añadirlo a tu montón.
hoard-sign-in = Inicia sesión para reunir un tesoro propio.
hoard-new = Botín nuevo por recoger: { $count }
hoard-none-new = No hay nada nuevo por recoger.
hoard-collect = Recoger
hoard-unavailable = Este navegador no puede dibujar el tesoro: { $error }
hoard-collect-failed = No se ha podido guardar lo que has recogido: { $error }
hoard-arcade = Pantalla de recreativa
hoard-full-screen = Pantalla completa
hoard-leave-full-screen = Salir de pantalla completa
hoard-label = Un dragón que custodia su tesoro. Piezas: { $count }
hoard-gold = Oro
hoard-gems = Gemas
hoard-books = Libros
hoard-weapons = Armas
hoard-bones = Huesos
hoard-tablets = Tablillas rúnicas
hoard-paintings = Cuadros
hoard-potions = Pociones

## UpdatesPanel.vue

updates-saved = Hoard llamará a esa dirección cuando haya una actualización lista
updates-saved-none = Ninguna dirección a la que llamar
updates-save-failed = No se ha podido guardar
updates-call-answered = Ha respondido { $result }
updates-call-refused = No ha aceptado la petición: { $result }
updates-call-failed = No se ha podido llamar
updates-called = Llamada el { $when } para { $version }: { $result }
updates-called-failed = Llamada el { $when } para { $version }: { $result }. Hoard volverá a intentarlo dentro de una hora.
updates-latest = Hoard { $version } es la última versión.
updates-out = Ha salido Hoard { $version }.
updates-out-idle = Ha salido Hoard { $version }; se llamará en un par de minutos, mientras no haya nada en curso.
updates-out-busy = Ha salido Hoard { $version }; esperando a que no haya nada en curso ({ $work }).
updates-busy-more = { $count } más
updates-busy-jobs =
    { $count ->
        [one] { $count } tarea en espera
       *[other] { $count } tareas en espera
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } subida
       *[other] { $count } subidas
    }
updates-list-separator = ,{ " " }
updates-what = Cuando sale una versión nueva, Hoard puede llamar a una dirección en cuanto no haya nada en curso (ni subidas, ni tareas, ni conversiones, ni mantenimiento), para que lo que actualice su contenedor no lo reinicie en mitad de una subida. Envía un POST con la versión en un cuerpo JSON. Necesita la comprobación de versiones de arriba.
updates-url = Dirección a la que llamar
updates-url-placeholder = Ninguna (p. ej. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Guardada (escribe para sustituirla)
updates-auth-placeholder = Ninguna (p. ej. Bearer my-token)
updates-remove = Quitar
updates-save = Guardar
updates-call-now-title = Se ejecuta lo que esté conectado a ella
updates-call-now = Llamar ahora
updates-what-calls = ¿A qué puede llamar?
updates-watchtower = La API HTTP de Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , con{ " " }
updates-watchtower-token = { " " }y el token de{ " " }
updates-watchtower-its = { " " }(con su endpoint{ " " }
updates-watchtower-endpoint = { " " }activado).
updates-portainer = Un webhook de Portainer
updates-portainer-rest = { " " }en el stack o el contenedor de Hoard, con «re-pull image» activado. El token va en la URL.
updates-script = Un script en el host
updates-script-listener = { " " }detrás de un receptor de webhooks (como{ " " }
updates-script-runs = ) que ejecute{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }o cualquier otra cosa que acepte un webhook.
updates-poll-before = O prescinde de la dirección: un script con un token de acceso puede consultar{ " " }
updates-poll-until = { " " }hasta que{ " " }
updates-poll-and = { " " }esté definido y{ " " }
updates-poll-after = { " " }sea true, y entonces descargar la nueva versión y reiniciar Hoard.

## MaintenancePanel.vue

maintenance-load-failed = No se ha podido cargar Mantenimiento
maintenance-reanalyze-title = ¿Volver a analizar todos los paquetes?
maintenance-reanalyze-body = Se vuelve a leer cada archivo, lo que puede tardar mucho en una biblioteca grande; los paquetes aparecen como en proceso hasta que les toca. Tus propios nombres, etiquetas y notas se mantienen.
maintenance-reanalyze-confirm = Volver a analizar
maintenance-gc-title = ¿Eliminar los archivos sin usar?
maintenance-gc-body = Los archivos guardados a los que no apunta nada de la biblioteca se eliminan definitivamente. Los paquetes del Archivo conservan los suyos.
maintenance-gc-confirm = Eliminarlos
maintenance-start-failed = No se ha podido iniciar «{ $task }»
maintenance-just-now = hace un momento
maintenance-minutes-ago = hace { $count } min
maintenance-hours-ago = hace { $count } h
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Cargando…
maintenance-running = En ejecución…
maintenance-starting = Iniciando…
maintenance-run = Ejecutar
maintenance-started = { $detail } · iniciado { $when }
maintenance-starting-detail = Iniciando
maintenance-finished = Terminado:{ " " }
maintenance-failed = Ha fallado:{ " " }
maintenance-never-run = No se ha ejecutado nunca
maintenance-history = Ejecuciones anteriores
maintenance-col-task = Tarea
maintenance-col-finished = Terminada
maintenance-col-took = Duración
maintenance-col-result = Resultado
maintenance-script-before = Las copias de seguridad y la comprobación de daños también se pueden ejecutar desde un script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }y{ " " }
maintenance-script-after = . Esas ejecuciones también aparecen aquí.

## SettingsPage.vue

settings-title = Ajustes
settings-loading = Cargando…
settings-find = Buscar un ajuste
settings-find-none = Ningún ajuste coincide con «{ $query }».
settings-load-failed = No se han podido cargar los ajustes: { $error }
settings-page-appearance = Apariencia
settings-page-appearance-blurb = Los temas que usa este navegador cuando está en modo claro y en modo oscuro. Se guarda solo en este navegador.
settings-page-dragon = Dragón
settings-page-dragon-blurb = El tesoro de tu dragón: cada recurso de la biblioteca es una pieza de él.
settings-page-library = Biblioteca
settings-page-library-blurb = Lo que hay en Hoard: cuánto ocupa, de qué está hecho, cómo ha crecido y lo bien etiquetado que está. Lo archivado no se cuenta.
settings-page-copies = Casi copias
settings-page-copies-blurb = Archivos que son la misma imagen, sonido o modelo en otro tamaño, frecuencia de muestreo o formato, primero los que más espacio ocupan. La mejor de cada grupo está marcada; no se elimina nada.
copies-loading = Buscando casi copias…
copies-refreshing = La biblioteca ha cambiado desde que se encontraron. Buscando de nuevo…
copies-none = No hay casi copias: cada imagen, sonido y modelo está aquí una sola vez.
copies-load-failed = No se han podido mostrar las casi copias: { $error }
copies-sum = Grupos de casi copias: { $count }. Las copias que no son la mejor ocupan { $size }.
copies-set = Grupo { $n }
copies-set-wasted = { $size } en las copias que no son la mejor
copies-more = Mostrar más (quedan { $count })
settings-page-tags = Etiquetas
settings-page-tags-blurb = Todas las etiquetas de la biblioteca. Fusiona las que signifiquen lo mismo o cambia el nombre de una; los etiquetadores usan el nuevo nombre a partir de entonces.
settings-page-store-files = Archivos de tienda
settings-page-store-files-blurb = Los archivos que las tiendas ponen junto a su contenido, que Hoard lee para dar nombre a los paquetes y dar crédito a sus autores. Enséñale los que no conoce y comparte lo que le has enseñado.
settings-page-locations = Ubicaciones de la biblioteca
settings-page-locations-blurb = Las carpetas del servidor que contienen la biblioteca: unas en las que Hoard guarda archivos y carpetas de paquetes que indexa donde están.
settings-page-locations-blurb-desktop = Las carpetas que contienen la biblioteca: unas en las que Hoard guarda archivos y carpetas de paquetes que indexa donde están.
settings-page-tools = Herramientas
settings-page-tools-blurb = Programas externos que Hoard usa para convertir y leer archivos.
settings-page-ai = Etiquetado con IA
settings-page-ai-blurb = Un modelo de IA pone nombre, describe y etiqueta los paquetes nuevos, además de las reglas integradas. Sus etiquetas se marcan como sugeridas.
settings-page-search-blurb = Busca imágenes por lo que muestran y sonidos por cómo suenan, no solo por su nombre, y encuentra cosas parecidas en su naturaleza, no solo en su color.
settings-page-search-tab = Búsqueda
settings-page-adding = Añadir paquetes
settings-page-adding-blurb = Lo que hace Hoard con un paquete cuando se añade, además de leerlo y etiquetarlo.
settings-page-downloads = Descargas
settings-page-downloads-blurb = Cómo se nombran los archivos en lo que envía Hoard: descargas, zips, la herramienta hoard y los asistentes de IA. Los archivos de la biblioteca se quedan como están.
settings-page-account = Cuenta
settings-page-account-blurb = Tu contraseña, las demás personas que pueden iniciar sesión y los tokens de acceso para scripts y otros programas.
settings-page-auth = Autenticación
settings-page-auth-blurb = Cómo se inicia sesión: las cuentas propias de Hoard, un directorio LDAP, Microsoft Active Directory o un proveedor OAuth, cuánto duran las sesiones y qué grupos reciben qué rol.
settings-page-connection = Conexión
settings-page-connection-blurb = Cómo llegan a Hoard los navegadores y las aplicaciones: la dirección por la que se le conoce y HTTPS, que algunas funciones del navegador necesitan.
settings-page-performance = Rendimiento
settings-page-performance-blurb = Cuánto trabaja el servidor, cómo se crean y se guardan las miniaturas y lo máximo que acepta.
settings-page-performance-blurb-desktop = Cuánto trabaja Hoard, cómo se crean y se guardan las miniaturas y lo máximo que acepta.
settings-page-issues = Incidencias
settings-page-issues-blurb = Lo que ha fallado al abrir o procesar archivos: archivos comprimidos que no se han podido descomprimir, archivos que no se han podido leer, tareas que han fallado. Vuelve a intentarlos cuando se haya resuelto la causa, o descártalos.
settings-page-maintenance = Mantenimiento
settings-page-maintenance-blurb = Haz copias de seguridad, busca daños y pon orden. Se ejecuta una tarea de cada tipo a la vez, y puedes salir de esta página mientras tanto.
settings-page-about = Acerca de
settings-page-about-blurb = Qué es Hoard, qué versión es esta y el trabajo de otros con el que está hecho.
settings-the-default = el valor predeterminado
settings-default-on = activado
settings-default-off = desactivado
settings-default-auto = automático
settings-option-saved = { $name }: { $value }
settings-back-to-default = Volver al valor predeterminado
settings-save = Guardar
settings-use = Usar
settings-status-off = Desactivado

## SettingsPage.vue: Apariencia

settings-themes = Temas
settings-look-light = Claro
settings-look-dark = Oscuro
settings-look-group =
    { $scheme ->
        [light] Tema claro
       *[dark] Tema oscuro
    }
settings-preview =
    Se muestra el tema { $preview ->
        [light] claro
       *[dark] oscuro
    } para que lo veas. Estás usando el { $used ->
        [light] claro
       *[dark] oscuro
    }{ $system ->
        [yes] { " " }(según el sistema)
       *[other] { "" }
    }, que volverá cuando salgas de esta página.
settings-preview-back =
    Volver al { $used ->
        [light] claro
       *[dark] oscuro
    }
settings-look-hint = Cambia entre claro y oscuro (o usa el del sistema) desde la barra superior.

## SettingsPage.vue: Herramientas

settings-blender-what = Convierte modelos .blend, .dae, .abc y .x3d a glTF en segundo plano, para que tengan miniaturas y vista previa 3D. Los paquetes añadidos mientras estaba desactivado se convierten cuando esté listo.
settings-blender-download = Usa un Blender instalado, o descarga Blender 4.5 LTS (unos 400 MB) de un servidor espejo de blender.org.
settings-ffmpeg-what = Codifica las descargas en MP3, OGG y Opus, y lee el audio que Hoard no sabe decodificar por sí mismo (AAC, M4A, WMA, …).
settings-ffmpeg-download = Usa un ffmpeg del sistema con los codificadores adecuados, o descarga una compilación estática (unos 80 MB).
settings-tool-reset = { $tool }: se ha vuelto al valor predeterminado
settings-tool-no-download = Usa un { $tool } instalado. No hay ninguna compilación que descargar para esta máquina, así que instálalo o indica su ruta.
settings-tool-path = Ruta del programa { $tool }
settings-mode-off = Desactivado
settings-mode-auto = Automático
settings-mode-path = Este programa
settings-tool-looking = Buscando…
settings-tool-downloading = Descargando… el progreso está en el indicador de actividad
settings-tool-ready = Listo: { $detail }
settings-soundfont = SoundFont para MIDI
settings-soundfont-what = Los instrumentos con los que se reproducen y convierten los archivos MIDI.
settings-soundfont-file = Archivo SoundFont
settings-soundfont-placeholder = El que haya en el servidor, o el integrado
settings-soundfont-placeholder-desktop = El que haya en este ordenador, o el integrado
settings-soundfont-default = el que haya en el servidor, o el integrado
settings-soundfont-default-desktop = el que haya en este ordenador, o el integrado
settings-soundfont-send = Elegir un archivo…
settings-soundfont-sent = Ahora el MIDI suena con { $name }

## SettingsPage.vue: Búsqueda de imágenes

settings-clip-heading = Buscar por lo que muestran las imágenes
settings-clip-reset = Búsqueda de imágenes: se ha vuelto al valor predeterminado
settings-clip-saved-on = Búsqueda de imágenes: activada
settings-clip-saved-off = Búsqueda de imágenes: desactivada
settings-clip-what = Escribe lo que buscas («mossy rock», «sci-fi door», «wooden barrel») y las imágenes que lo muestran aparecen junto a los nombres que tienen esas palabras. «Más como este» encuentra cosas del mismo tipo, no solo de los mismos colores.
settings-clip-toggle = Activar la búsqueda de imágenes
settings-clip-hint = Usa el modelo SigLIP 2 de Google, ejecutado en el procesador de este servidor: no sale nada de él. Al activarla se descarga el modelo ({ $size } MB) de Hugging Face y luego aprende cada imagen en segundo plano con baja prioridad. Ocupa entre 350 y 750 MB de memoria mientras trabaja o alguien busca. Al desactivarla se elimina el modelo y se conserva lo aprendido para la próxima vez.
settings-clip-hint-desktop = Usa el modelo SigLIP 2 de Google, ejecutado en el procesador de este ordenador: no sale nada de él. Al activarla se descarga el modelo ({ $size } MB) de Hugging Face y luego aprende cada imagen en segundo plano con baja prioridad. Ocupa entre 350 y 750 MB de memoria mientras trabaja o mientras buscas. Al desactivarla se elimina el modelo y se conserva lo aprendido para la próxima vez.
settings-clip-looking = Buscando el modelo…
settings-clip-downloading = Descargando el modelo… el progreso está en el indicador de actividad
settings-clip-ready-empty = Lista. Las imágenes se aprenden a medida que se añaden paquetes.
settings-clip-ready-all =
    { $count ->
        [one] Lista: conoce { $count } imagen
       *[other] Lista: conoce las { $count } imágenes
    }
settings-clip-learning = Lista: aprendiendo imágenes, { $done } de { $total } hasta ahora
settings-clip-relearning = Hasta que el modelo nuevo haya aprendido todas las imágenes, las búsquedas usan lo que aprendió el anterior.
settings-clip-unavailable = No se ha podido obtener el modelo: { $detail }. Se vuelve a intentar cada hora.
settings-clip-where = Se ejecuta en
settings-clip-here = Este servidor: nada sale de él
settings-clip-here-desktop = Este ordenador: nada sale de él
settings-clip-hosted = Un servicio alojado
settings-clip-hosted-warning = Las imágenes salen de este servidor. Una miniatura de cada imagen de la biblioteca y las palabras de cada búsqueda de imágenes se envían a { $service }, que cobra por ellas y las conserva según sus condiciones.
settings-clip-hosted-warning-desktop = Las imágenes salen de este ordenador. Una miniatura de cada imagen de la biblioteca y las palabras de cada búsqueda de imágenes se envían a { $service }, que cobra por ellas y las conserva según sus condiciones.
settings-clip-hosted-hint = Aquí no se descarga ni se ejecuta nada: el servicio aprende cada imagen, por lotes. Otro servicio u otro modelo las vuelve a aprender todas, y lo que ha aprendido cada uno se conserva, por si vuelves.
settings-clip-saved-here = Búsqueda de imágenes: se ejecuta en este servidor
settings-clip-saved-here-desktop = Búsqueda de imágenes: se ejecuta en este ordenador
settings-clip-saved-hosted = Búsqueda de imágenes: se ejecuta en { $service }
settings-clip-hosted-unavailable = No se ha podido usar { $service }: { $detail }. Se vuelve a intentar cada hora.
settings-clap-heading = Buscar por cómo suenan los sonidos
settings-clap-reset = Búsqueda de sonidos: se ha vuelto al valor predeterminado
settings-clap-saved-on = Búsqueda de sonidos: activada
settings-clap-saved-off = Búsqueda de sonidos: desactivada
settings-clap-what = Escribe lo que buscas («distant thunder», «glassy UI blip», «footsteps on gravel») y los sonidos que suenan así aparecen junto a los nombres que tienen esas palabras.
settings-clap-toggle = Activar la búsqueda de sonidos
settings-clap-hint = Usa el modelo CLAP de LAION, ejecutado en el procesador de este servidor: no sale nada de él. Al activarla se descarga el modelo ({ $size } MB) de Hugging Face y luego escucha el principio de cada sonido en segundo plano con baja prioridad. Ocupa hasta cerca de 1 GB de memoria mientras trabaja o alguien busca. Al desactivarla se elimina el modelo y se conserva lo aprendido para la próxima vez.
settings-clap-hint-desktop = Usa el modelo CLAP de LAION, ejecutado en el procesador de este ordenador: no sale nada de él. Al activarla se descarga el modelo ({ $size } MB) de Hugging Face y luego escucha el principio de cada sonido en segundo plano con baja prioridad. Ocupa hasta cerca de 1 GB de memoria mientras trabaja o mientras buscas. Al desactivarla se elimina el modelo y se conserva lo aprendido para la próxima vez.
settings-clap-ready-empty = Lista. Los sonidos se aprenden a medida que se añaden paquetes.
settings-clap-ready-all =
    { $count ->
        [one] Lista: conoce { $count } sonido
       *[other] Lista: conoce los { $count } sonidos
    }
settings-clap-learning = Lista: aprendiendo sonidos, { $done } de { $total } hasta ahora

## SettingsPage.vue: Etiquetado con IA

settings-ai-service = Servicio
settings-ai-off = Desactivado
settings-ai-off-hint = Los paquetes se etiquetan solo con las reglas integradas.
settings-ai-local = Modelo local
settings-ai-local-hint = El servicio llm del archivo compose de Hoard (o cargo xtask llm). Descarga un modelo abajo; se ejecuta en esta máquina y no sale nada de ella.
settings-ai-chatgpt = Plan de ChatGPT
settings-ai-chatgpt-hint = Usa tu plan ChatGPT Go, Plus, Pro o Business en lugar de una clave de API.
settings-ai-openrouter-hint = Una sola clave para cientos de modelos alojados, algunos gratuitos. Los modelos que aceptan imágenes están marcados.
settings-ai-openai-hint = Alojado por OpenAI.
settings-ai-anthropic-hint = Alojado por Anthropic, a través de su endpoint compatible con OpenAI.
settings-ai-gemini-hint = Alojado por Google, a través de su endpoint compatible con OpenAI.
settings-ai-ollama-hint = Un servidor de Ollama que gestionas tú. Cambia la dirección si no es un contenedor llamado ollama.
settings-ai-custom = Otro (compatible con OpenAI)
settings-ai-custom-hint = Cualquier cosa que hable la API de chat de OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Conectando…
settings-ai-connected =
    { $count ->
        [one] Conectado · { $count } modelo
       *[other] Conectado · { $count } modelos
    }
settings-ai-connected-none = Conectado, pero no ofrece ningún modelo; escribe el nombre del modelo
settings-chatgpt-connected-none = Conectado; escribe el nombre del modelo
settings-ai-takes-images = acepta imágenes
settings-ai-saved = Etiquetado con IA: { $service }
settings-ai-saved-off = El etiquetado con IA está desactivado
settings-ai-reset = Etiquetado con IA: se ha vuelto al valor predeterminado
settings-ai-on = Activado: { $service } · { $model }
settings-ai-url = Dirección
settings-ai-key = Clave de API
settings-ai-key-saved = Guardada (escribe para sustituirla)
settings-ai-key-paste = Pega tu clave
settings-ai-key-optional = Ninguna (opcional)
settings-ai-key-remove = Quitar
settings-ai-get-key = Obtener una clave
settings-ai-model = Modelo
settings-ai-model-pick = Elige o escribe un modelo
settings-ai-model-name = Nombre del modelo
settings-ai-refresh = Actualizar
settings-ai-connect = Conectar
settings-ai-need-key = Añade la clave de API para ver sus modelos
settings-ai-vision = Enviar también miniaturas (necesita un modelo que acepte imágenes)
settings-ai-batch = Por petición
settings-ai-batch-unit = recursos (menos para modelos locales pequeños, más para los grandes alojados)

## SettingsPage.vue: Inicio de sesión de ChatGPT

settings-chatgpt-account = Cuenta
settings-chatgpt-checking = Comprobando…
settings-chatgpt-status-as = Sesión iniciada como { $email }
settings-chatgpt-status = Sesión iniciada
settings-chatgpt-signed-in-as = Sesión iniciada en ChatGPT como { $email }
settings-chatgpt-signed-in = Sesión iniciada en ChatGPT
settings-chatgpt-signed-out = Sesión de ChatGPT cerrada
settings-chatgpt-continue = Continuar con ChatGPT
settings-chatgpt-sign-out = Cerrar sesión
settings-chatgpt-sign-out-title = ¿Cerrar la sesión de ChatGPT?
settings-chatgpt-sign-out-in-use = El etiquetado con IA usa tu plan de ChatGPT, así que se detendrá hasta que vuelvas a iniciar sesión o elijas otro servicio.
settings-chatgpt-sign-out-body = Hoard olvida el inicio de sesión y le pide a OpenAI que lo termine.
settings-chatgpt-manage = Gestionar el uso
settings-chatgpt-finish-before = Termina de iniciar sesión en la ventana que se ha abierto (
settings-chatgpt-finish-link = vuelve a abrirla
settings-chatgpt-finish-middle = ). Si acaba en una página que no carga, en una dirección que empieza por{ " " }
settings-chatgpt-finish-after = , copia esa dirección y pégala aquí:
settings-chatgpt-paste = La dirección en la que ha terminado el inicio de sesión
settings-chatgpt-finish = Terminar
settings-chatgpt-usage = Las peticiones de etiquetado cuentan para el uso de tu plan de ChatGPT. Gestionar el uso muestra lo que ha usado Hoard y te permite limitarlo. Para desconectar Hoard del propio ChatGPT, ve a Configuración → Seguridad e inicio de sesión → Iniciar sesión con ChatGPT en chatgpt.com.

## SettingsPage.vue: Modelos locales

settings-local-downloading-progress = Descargando { $percent } % · { $done } de { $total }
settings-local-downloading = Descargando…
settings-local-incomplete = Descarga detenida
settings-local-failed = No se pudo iniciar la última vez
settings-local-in-use-running = En uso · en ejecución
settings-local-in-use = En uso
settings-local-starting = Iniciando…
settings-local-running = En ejecución
settings-local-ready-images = Listo · acepta imágenes
settings-local-ready-text = Listo · solo texto
settings-local-delete-title = ¿Eliminar { $name }?
settings-local-delete-in-use = El etiquetado con IA usa este modelo, así que se detendrá hasta que elijas otro. Sus archivos se eliminan del servidor de modelos.
settings-local-delete-downloading = La descarga se detiene y se elimina lo descargado hasta ahora.
settings-local-delete-body = Sus archivos se eliminan del servidor de modelos. Puedes volver a descargarlo más adelante.
settings-local-delete = Eliminar
settings-local-deleting = Eliminando…
settings-local-cancel = Cancelar
settings-local-resume = Reanudar
settings-local-download = Descargar
settings-local-start-before = Inícialo con{ " " }
settings-local-start-or = { " " }(o{ " " }
settings-local-start-developing = { " " }durante el desarrollo) y luego{ " " }
settings-local-start-retry = vuelve a intentarlo
settings-local-start-after = . También sirve un servidor que ejecute un solo modelo: indica abajo el nombre de su modelo.
settings-local-list = En el servidor de modelos
settings-local-none = Todavía no se ha descargado nada. Elige uno abajo; se guarda en el servidor y puedes cambiarlo cuando quieras.
settings-local-catalog = Modelos para descargar
settings-local-size = { $size } GB · { $license }
settings-local-other = Otro modelo de Hugging Face
settings-local-other-placeholder = Otro de Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Añadir paquetes

settings-store-pages = Páginas de tienda
settings-store-pages-what = Los paquetes nuevos se buscan en las tiendas en segundo plano, para que tengan su página de tienda, su vendedor y su licencia.
settings-store-lookup = Buscar páginas de tienda automáticamente
settings-unreal = Paquetes de Unreal
settings-unreal-reset = Paquetes de Unreal: se ha vuelto al valor predeterminado
settings-unreal-what = Qué hacer con los archivos .uasset.
settings-conv = Reglas de conversión
settings-conv-what = Convierte los archivos que guarda Hoard a un solo formato, como todos los OBJ y FBX a GLB, para que la biblioteca sea uniforme. El archivo convertido ocupa el lugar del original y conserva su nombre, etiquetas, colecciones y enlaces. Los archivos de las carpetas vigiladas nunca se modifican.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Activada
settings-conv-to = Convertir a
# Heading over the file formats a rule converts.
settings-conv-from = De
settings-conv-remove = Quitar
settings-conv-remove-label = Quitar la regla { $n }
settings-conv-taken = Otra regla ya convierte este formato
settings-conv-upright = Poner los modelos de pie al convertirlos (con la Y hacia arriba y mirando hacia +Z), como se muestran los modelos del paquete
settings-conv-flac-what = El sonido queda exactamente igual, en más o menos la mitad de espacio, y todo lo demás del WAV (puntos de bucle, marcadores de cue, etiquetas) se guarda en el FLAC, así que el WAV se puede recuperar tal como era en cualquier momento. Los WAV con muestras de coma flotante o de 32 bits se quedan como WAV.
settings-conv-add = Añadir una regla
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Conservar los originales durante
settings-conv-keep-days =
    { $count ->
        [one] día
       *[other] días
    }
settings-conv-keep-what = Hasta entonces, desde la página de un archivo convertido se puede restaurar el original. Las texturas no se copian dentro de un modelo convertido: este apunta a los archivos de imagen que tiene al lado. Los archivos .mtl o .bin de un modelo se quedan.
settings-conv-save = Guardar reglas
settings-conv-revert = Deshacer los cambios
settings-conv-existing = Convertir los archivos que ya están en la biblioteca
settings-conv-saved = Reglas de conversión guardadas. Se aplican a los paquetes que se añadan a partir de ahora.
settings-conv-converting =
    { $count ->
        [one] Convirtiendo un archivo en segundo plano
       *[other] Convirtiendo { $count } archivos en segundo plano
    }
settings-conv-nothing = No hay nada en la biblioteca que necesite convertirse.
settings-conv-ask-title =
    { $count ->
        [one] ¿Convertir un archivo que ya está en la biblioteca?
       *[other] ¿Convertir { $count } archivos que ya están en la biblioteca?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Está en un paquete.
       *[other] Están en { $packs } paquetes.
    } Cada original se conserva { $days } días y, hasta entonces, se puede restaurar desde su página.
settings-conv-ask-busy =
    { $count ->
        [one] Uno está en un paquete que todavía se está añadiendo; se convertirá cuando termine.
       *[other] { $count } están en paquetes que todavía se están añadiendo; se convertirán cuando terminen.
    }
settings-conv-ask-list = Archivos que se convertirían
settings-conv-ask-more = …y { $count } más
settings-conv-ask-skipped = Se dejan como están:
settings-conv-skip-watched =
    { $count ->
        [one] un archivo en una carpeta vigilada
       *[other] { $count } archivos en carpetas vigiladas
    }
settings-conv-skip-unity =
    { $count ->
        [one] un archivo que un proyecto de Unity conoce por su nombre
       *[other] { $count } archivos que un proyecto de Unity conoce por sus nombres
    }
settings-conv-skip-used =
    { $count ->
        [one] un archivo al que hace referencia una escena u otro archivo
       *[other] { $count } archivos a los que hace referencia una escena u otro archivo
    }
settings-conv-skip-beside =
    { $count ->
        [one] un archivo cuya copia convertida ya está en su paquete
       *[other] { $count } archivos cuyas copias convertidas ya están en sus paquetes
    }
settings-conv-skip-blender =
    { $count ->
        [one] un archivo que Blender aún no ha leído (Ajustes → Herramientas)
       *[other] { $count } archivos que Blender aún no ha leído (Ajustes → Herramientas)
    }
settings-conv-ask-new = Guardar, solo paquetes nuevos
settings-conv-ask-save-convert =
    { $count ->
        [one] Guardar y convertirlo
       *[other] Guardar y convertir { $count } archivos
    }
settings-conv-ask-convert =
    { $count ->
        [one] Convertirlo
       *[other] Convertir { $count } archivos
    }
settings-unreal-off = Dejarlos como están
settings-unreal-off-hint = Conviértelos desde la página de un paquete cuando quieras.
settings-unreal-keep = Convertir y conservar los originales
settings-unreal-keep-hint = Las texturas, los sonidos y los materiales salen en formatos comunes; el .uasset también se queda.
settings-unreal-replace = Convertir y descartar los originales
settings-unreal-replace-hint = Ahorra espacio una vez extraído el contenido. Las mallas siguen esperando al worker de Unreal.

## SettingsPage.vue: Descargas

settings-tn = Nombres de texturas
settings-tn-saved = Nombres de texturas guardados
settings-tn-reset = Nombres de texturas: se ha vuelto al valor predeterminado
settings-tn-what = Los mapas de un material salen con el nombre del material y para qué sirve cada mapa, se llamen como se llamen los archivos: Rock_Normal.png, no rock_moss_xeokah0_raw_8K_Normal.png. Los archivos de la biblioteca no se tocan, y un mapa al que nombra el modelo o el MTL con el que se envía conserva ese nombre, para que el modelo lo siga encontrando.
settings-tn-toggle = Nombrar los mapas de texturas según su material
settings-tn-style = Estilo
settings-tn-style-group = Estilo de nombres
settings-tn-descriptive = Descriptivo
settings-tn-short = Corto
settings-tn-custom = Personalizado
settings-tn-pattern = Patrón
settings-tn-hint-name = { " " }el material,{ " " }
settings-tn-hint-map = { " " }para qué sirve el mapa,{ " " }
settings-tn-hint-size = { " " }su tamaño cuando el conjunto viene en más de uno (se añade al final si el patrón no lo incluye).
settings-tn-example = Ejemplo
settings-tn-suffixes = Sufijos para cada tipo de mapa
settings-tn-slots = En las descargas GLB, una textura empaquetada dentro de un modelo sin archivo propio se nombra según el material y la ranura que ocupa, y estas ranuras también tienen sufijos:
settings-tn-endings = Tus terminaciones
settings-tn-endings-what = Una textura que no es uno de los mapas de un material conserva su nombre, a menos que termine en una de estas: _diff → _BC envía rock_diff.png como rock_BC.png. Deja vacía la segunda para quitar la terminación.
settings-tn-ending-from = Terminación
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Cambiar por
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Añadir una terminación
settings-tn-ending-remove = Quitar
settings-tn-ending-remove-label = Quitar { $ending }

## SettingsPage.vue: General y Conexión

settings-name = Nombre
settings-name-what = Cómo se llama este servidor en la barra superior, en la pestaña del navegador y en la página de inicio de sesión, para que, si tienes más de un Hoard, sepas cuál es cuál.
settings-name-what-desktop = Cómo se llama esta biblioteca en la barra superior.
settings-address = Dirección
settings-address-what = La dirección que usan las personas y las aplicaciones para llegar a Hoard, cuando no es la de este navegador: la de un proxy inverso, la de un túnel o un nombre de tailnet, como https://hoard.example.com. La usan los enlaces que reparte Hoard (a Claude y a otras aplicaciones, al iniciar sesión, en su certificado).
settings-public-url = Dirección pública
settings-public-url-placeholder = La de la barra del navegador (p. ej. https://hoard.example.com)
settings-public-url-default = la de la barra del navegador

## ProfilerPanel.vue: el perfilador en la parte superior de Ajustes → Rendimiento

profiler = Perfilador
profiler-what = Lo que ha hecho Hoard en los últimos diez minutos, actualizado cada dos segundos mientras lo miras. Solo se guarda en memoria y vuelve a empezar cuando Hoard se reinicia.
profiler-starting = Las primeras lecturas llegan en un momento.
profiler-failed = No se ha podido leer el perfilador: { $error }
profiler-cpu = Uso del procesador
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Toda la máquina
profiler-cpu-summary = Hoard está usando el { $hoard } de los procesadores, y toda la máquina, el { $machine }.
profiler-memory = Memoria
profiler-serving = Peticiones en curso
profiler-working = Trabajando en
profiler-working-none = No se ha ejecutado nada en segundo plano.
profiler-working-more =
    { $count ->
        [one] Y una más.
       *[other] Y { $count } más.
    }
profiler-requests = Tiempo atendiendo peticiones
profiler-requests-what = Cada barra son diez segundos de peticiones, con su tiempo repartido según en qué se ha ido.
profiler-part-db = Base de datos
profiler-part-db-title = Esperando a la base de datos y ejecutando sus consultas
profiler-part-disk = Disco
profiler-part-disk-title = Leyendo archivos guardados y comprimiendo lo que se envía
profiler-part-send = Envío
profiler-part-send-title = Esperando mientras la red recoge lo enviado
profiler-part-other = Otros
profiler-part-other-title = Preparando la respuesta
profiler-request-count =
    { $count ->
        [one] { $count } petición
       *[other] { $count } peticiones
    }
profiler-requests-none = Ninguna petición en los últimos diez minutos.
profiler-slowest = La más lenta: { $route }, { $time }
profiler-routes = Por ruta
profiler-route = Ruta
profiler-count = Peticiones
profiler-average = Media
profiler-p95 = El 5 % más lento
profiler-split = En qué se ha ido el tiempo
profiler-sent = Enviado
profiler-portal = los archivos de la propia interfaz

## SettingsPage.vue: Rendimiento

settings-render = Miniaturas 3D
settings-render-toggle = Renderizar miniaturas de los modelos 3D
settings-gpu-on = En la GPU: { $renderer }
settings-gpu-software = No se ha encontrado ninguna GPU, así que por software ({ $renderer })
settings-gpu-none = No se ha encontrado ninguna GPU ni renderizador por software, así que los modelos no tienen miniatura
settings-thumbs = Miniaturas
settings-thumbs-what = Cómo se guardan las miniaturas y vistas de modelos nuevas. Las que tienen transparencia, y el pixel art, son siempre PNG. Para reducir las que ya tienes, ejecuta Reducir miniaturas en Mantenimiento.
settings-previews-size = Las vistas previas ocupan ahora { $size }.
settings-thumbs-saved = Miniaturas: { $quality }
settings-thumbs-small = Más pequeñas
settings-thumbs-small-hint = JPEG, calidad 70. Bien en la cuadrícula; algo borrosas de cerca.
settings-thumbs-balanced = Equilibradas
settings-thumbs-balanced-hint = JPEG, calidad 82. Difícil de distinguir del original a tamaño de miniatura.
settings-thumbs-sharp = Más nítidas
settings-thumbs-sharp-hint = JPEG, calidad 92. Más o menos el doble de tamaño que Equilibradas.
settings-thumbs-lossless = Sin pérdida
settings-thumbs-lossless-hint = PNG, exactamente como se crean. Varias veces más grandes.
settings-thumbs-avif = Enviar las miniaturas como AVIF
settings-thumbs-avif-what = Los navegadores que muestran AVIF reciben miniaturas más pequeñas, a menudo de menos de la mitad del tamaño. Cada una se crea a partir de la miniatura guardada la primera vez que se muestra y se guarda con las conversiones; hasta entonces se envía la guardada. Crearlas mantiene ocupados hasta dos núcleos mientras se recorre por primera vez una biblioteca grande. Las miniaturas sin pérdida y el pixel art se envían tal cual.
settings-motion-thumbs = Miniaturas en movimiento para animaciones
settings-motion-thumbs-what = Cada animación recibe también una película corta en bucle que se reproduce mientras apuntas a su mosaico. Ocupan unos 40 KB cada una y tardan un rato en hacerse en una biblioteca grande. Al desactivarlo, se borran.
settings-limits = Trabajo y límites
settings-limits-what = Cada uno se guarda al salir del campo o pulsar Enter. Los tamaños llevan unidades: 100 GB, 512 MB, 1,5 TB; un número solo son GB.
settings-workers = Procesos en segundo plano
settings-workers-unit = tareas a la vez (incorporación, etiquetado, conversiones)
settings-max-upload = Subida máxima
settings-max-upload-unit = por archivo
settings-max-expand = Paquete máximo
settings-max-expand-unit = una vez descomprimido
settings-conversion-cache = Descargas convertidas
settings-conversion-cache-unit-now = como máximo, ahora { $size }; las menos usadas recientemente se eliminan primero
settings-conversion-cache-unit = como máximo; las menos usadas recientemente se eliminan primero

## SettingsPage.vue: Mantenimiento

settings-updates = Actualizaciones
settings-version = Esta es la versión { $version } de Hoard.
settings-update-check = Buscar una versión nueva en GitHub cada pocas horas
settings-desktop-updates = Hoard busca una versión nueva cada vez que se inicia, la descarga en segundo plano y la instala cuando lo cierras.
settings-backups = Copias de seguridad programadas
settings-backups-what = La base de datos hace sus propias copias de seguridad; cada una aparece en el historial de abajo.
settings-backup-every = Hacer copia
settings-backup-off = Nunca
settings-backup-daily = Cada día
settings-backup-weekly = Cada semana
settings-backup-keep = Conservar
settings-backup-keep-unit = copias programadas más recientes
settings-backup-copy-to = Copiar también en
settings-backup-copy-to-none = Ninguna otra carpeta
settings-backups-kept = Cada una se guarda en la carpeta de copias como hoard-auto-…db, y en la otra carpeta (otro disco, o una que respalda una aplicación de sincronización) si la hay, indicada con su ruta completa en el servidor. Solo se eliminan las copias programadas más antiguas; las hechas a mano se quedan.
about-what = Hoard guarda recursos de juegos (modelos, texturas, sonidos, música, sprites y más) en una biblioteca en la que puedes buscar, previsualizar y coger lo que necesites, compartida con todas las personas a las que des acceso.
about-version = Versión
about-edition = Edición
about-edition-server = Servidor
about-edition-desktop = Aplicación de escritorio
about-edition-steam = Aplicación de escritorio de Steam
about-made-by = Hecho por
about-source = Código fuente en GitHub
about-issues = Informar de un problema
about-discord = Chatea con nosotros en Discord
about-steam = Hoard en Steam
about-license = Licencia
about-license-what = Hoard es de código abierto con la licencia MIT No Attribution: úsalo, modifícalo y compártelo como quieras.
about-downloaded = Se descarga cuando hace falta
about-downloaded-what = Hoard no los incluye. Descarga cada uno la primera vez que algo lo necesita, y cada uno tiene su propia licencia.
about-blender = Convierte modelos 3D y crea sus miniaturas.
about-ffmpeg = Convierte audio y vídeo y crea sus vistas previas.
about-siglip = Encuentra imágenes por lo que aparece en ellas.
about-clap = Encuentra sonidos por cómo suenan.
about-models = Los modelos que elijas para el servidor de IA local tienen las licencias de sus creadores.
about-built-with = Hecho con
about-built-with-what = Las bibliotecas de código abierto con las que está hecho Hoard, con sus licencias.
about-steamworks = La versión de Steam también incluye el SDK de Steamworks, © Valve Corporation.
about-licenses-loading = Cargando la lista…
about-licenses-failed = No se ha podido cargar la lista.
about-filter = Filtrar por nombre o licencia
about-count = { $shown } de { $total }
about-no-license = Sin licencia indicada
about-report = Informar de un problema…
report-title = Informar de un problema
report-what = Esto es lo que necesita una incidencia en GitHub para poder investigar el problema. No contiene nada privado: se quitan los tokens de acceso, las contraseñas, las claves, las direcciones de correo y tu nombre en las rutas de carpetas. Revísalo antes de enviarlo.
report-loading = Reuniendo el informe…
report-failed = No se ha podido reunir el informe: { $error }
report-log = Lo que el servidor ha registrado últimamente
report-copy = Copiar
report-copied = Informe copiado.
report-github = Abrir una incidencia en GitHub
report-github-hint = La incidencia nueva se rellena con esto. Si el registro es demasiado largo, solo van sus últimas líneas; Copiar lo tiene todo.
report-close = Cerrar

## Asset kinds (api.ts)

api-kind-texture = Texturas
api-kind-vector = Arte vectorial
api-kind-sheet = Hojas de sprites
api-kind-audio = Audio
api-kind-model = Modelos
api-kind-animation = Animaciones
api-kind-font = Fuentes
api-kind-shader = Shaders
api-kind-material = Materiales
api-kind-video = Vídeo
api-kind-document = Documentos
api-kind-data = Datos
api-kind-other = Otros
api-kind-texture-name = texturas
api-kind-vector-name = arte vectorial
api-kind-sheet-name = hojas de sprites
api-kind-audio-name = audio
api-kind-model-name = modelos
api-kind-animation-name = animaciones
api-kind-font-name = fuentes
api-kind-shader-name = shaders
api-kind-material-name = materiales
api-kind-video-name = vídeo
api-kind-document-name = documentos
api-kind-data-name = datos
api-kind-other-name = otros

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Deshacer
ui-today = hoy
ui-yesterday = ayer
ui-days-ago =
    { $days ->
        [one] hace { $days } día
       *[other] hace { $days } días
    }
ui-notifications = Notificaciones
ui-dismiss = Descartar
ui-cancel = Cancelar
ui-lazy-loading = Cargando { $what }…
ui-lazy-failed = No se ha podido cargar { $what }.
ui-lazy-reload = Recargar la página

## ResetButton.vue

reset-title = Volver al valor predeterminado: { $value }
reset-label = Volver al valor predeterminado, { $value }

## SizeInput.vue

size-invalid = Un tamaño como 100 GB, 512 MB o 1,5 TB
size-hint = Un tamaño como 100 GB, 512 MB o 1,5 TB; un número solo son GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = No se han podido cargar los ajustes de autenticación: { $error }
auth-saved = Guardado. Se aplica en el próximo inicio de sesión.
auth-method-internal = Interna
auth-method-internal-hint = Nombres y contraseñas que guarda Hoard, añadidos en Ajustes → Cuenta.
auth-method-ldap-hint = Un directorio comprueba el nombre y la contraseña escritos en la página de inicio de sesión: OpenLDAP, FreeIPA, LLDAP u otro servidor LDAP.
auth-method-ad-hint = Las cuentas de dominio de Windows inician sesión con su nombre y contraseña de siempre.
auth-method-oauth-hint = Un botón «Iniciar sesión con…» que va a Google, Microsoft, GitHub, Keycloak, Authentik u otro proveedor y vuelve.
auth-session-unused = Tras 30 días sin usarse
auth-session-1h = 1 hora después de iniciar sesión
auth-session-8h = 8 horas después de iniciar sesión
auth-session-day = Un día después de iniciar sesión
auth-session-week = Una semana después de iniciar sesión
auth-session-30d = 30 días después de iniciar sesión
auth-session-hours =
    { $hours ->
        [one] { $hours } hora después de iniciar sesión
       *[other] { $hours } horas después de iniciar sesión
    }
auth-role-view = Puede ver
auth-role-edit = Puede cambiar
auth-role-admin = Administrador
auth-role-word-view = puede ver
auth-role-word-edit = puede cambiar
auth-role-word-admin = administrador
auth-role-word-default = { $role } (el rol predeterminado)
auth-tls-starttls-hint = LDAP sin cifrar (puerto 389) que pasa a TLS antes de enviar nada.
auth-tls-ldaps-hint = TLS desde el principio (puerto 636).
auth-tls-none = Ninguna
auth-tls-none-hint = Las contraseñas cruzan la red tal cual. Solo en una red de confianza.
auth-test-failed = No se ha podido probar: { $error }
auth-preset-custom = Otro (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel y otros: indica el emisor y Descubrir rellena las direcciones.
auth-preset-google-note = Crea un cliente OAuth (Aplicación web) en Google Cloud Console → APIs y servicios → Credenciales. Google no envía grupos, así que asigna los roles en Ajustes → Cuenta, y desactiva la creación de cuentas en el primer inicio de sesión salvo que deba poder entrar cualquiera con una cuenta de Google.
auth-preset-microsoft-note = Registra una aplicación en Entra ID → Registros de aplicaciones (con una redirección web a la dirección de abajo). Pon el ID de tu inquilino en lugar de «organizations» para que solo entre tu organización. Para obtener roles de los grupos, añade la notificación de grupos en Configuración de token: enumera los ID de los grupos.
auth-preset-github-note = Crea una OAuth app en GitHub → Settings → Developer settings. GitHub no envía grupos, así que asigna los roles en Ajustes → Cuenta, y desactiva la creación de cuentas en el primer inicio de sesión salvo que deba poder entrar cualquiera con una cuenta de GitHub.
auth-preset-gitlab-note = Crea una aplicación en GitLab → Preferences → Applications (o en el área de administración de tu instancia: cambia gitlab.com en las direcciones).
auth-discovered = Se han rellenado las direcciones del proveedor.
auth-copied = Copiado
auth-copy-failed = No se ha podido copiar: selecciónalo y cópialo tú
auth-sign-in-with = Iniciar sesión con
auth-own-accounts-still-work = Las cuentas propias de Hoard siguen iniciando sesión con sus propias contraseñas, así que un directorio o proveedor caído o mal configurado no puede dejarte fuera.
auth-sessions = Sesiones
auth-sign-out-after = Cerrar la sesión de las personas
auth-ldap-server = Servidor LDAP
auth-server-type = Tipo de servidor
auth-server-type-custom = Personalizado
auth-servers = Servidores
auth-server-n = Servidor { $n }
auth-remove-server-n = Quitar el servidor { $n }
auth-remove = Quitar
auth-add-server = Añadir un servidor
auth-servers-hint = Cuando no se puede conectar con uno, se prueba el siguiente.
auth-security = Seguridad
auth-connection-security = Seguridad de la conexión
auth-skip-verify = No comprobar el certificado del servidor (solo para hacer pruebas)
auth-ca = Entidad de certificación
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (vacío: las que acepta el sistema)
auth-lookup = Búsqueda de personas
auth-anonymously = De forma anónima
auth-reader-account = Cuenta de lectura
auth-reader-password = Su contraseña
auth-saved-type-to-replace = Guardada: escribe para sustituirla
auth-people-under = Las personas están en
auth-name-attribute = Atributo del nombre
auth-only-matching = Solo las entradas que coincidan con
auth-groups-under = Los grupos están en
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (vacío: solo memberOf)
auth-their-groups = Sus grupos
auth-group-filter-hint = { "{" }dn{ "}" } es la entrada de la persona, { "{" }user{ "}" } el nombre con el que ha iniciado sesión.
auth-test = Prueba
auth-test-hint = Conecta con cada servidor e inicia sesión con la cuenta de lectura. Con un nombre y una contraseña, inicia sesión también con ellos y muestra cómo lo interpretaría Hoard. No se guarda nada.
auth-test-user = Alguien con quien iniciar sesión
auth-test-user-placeholder = Alguien con quien iniciar sesión (opcional)
auth-test-password = Su contraseña
auth-testing = Probando…
auth-test-connectivity = Probar la conexión
auth-test-server-ok = conectado, y la cuenta de lectura ha iniciado sesión
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }ha iniciado sesión (
auth-test-signed-in-after = ).
auth-test-role = Tendría el rol: { $role }.
auth-test-no-role = No está en ninguno de los grupos de Roles, así que se le denegaría el acceso.
auth-test-groups = Grupos: { $groups }
auth-test-no-groups = No se ha encontrado ningún grupo.
auth-provider = Proveedor
auth-redirect = Dirección de redirección
auth-copy = Copiar
auth-redirect-hint = Dásela al proveedor cuando crees el cliente de Hoard. Sale de la dirección pública de Ajustes → Conexión, o de la dirección que estás usando.
auth-issuer = Emisor
auth-discover = Descubrir
auth-client-id = ID de cliente
auth-client-secret = Secreto de cliente
auth-authorization-url = Dirección de autorización
auth-token-url = Dirección del token
auth-userinfo-url = Dirección de información del usuario
auth-userinfo-placeholder = (vacío: el token de ID dice quién es)
auth-scopes = Ámbitos
auth-name-from = Nombre desde
auth-groups-from = Grupos desde
auth-groups-from-placeholder = (ninguno)
auth-button = Botón
auth-button-placeholder = tu cuenta
auth-accounts = Cuentas
auth-provision = Crear una cuenta de Hoard la primera vez que alguien inicia sesión
auth-provision-on-hint = Si no, un administrador añade antes a cada persona, en Ajustes → Cuenta.
auth-provision-off-hint = Un administrador añade antes a cada persona, en Ajustes → Cuenta, y elige cómo inicia sesión.
auth-open-to-anyone = Cualquiera con una cuenta de { $provider } puede entrar. Añade grupos de lectores abajo, o desactiva esto y añade tú a las personas.
auth-new-accounts = Cuentas nuevas
auth-roles-from-groups = Roles a partir de grupos
auth-roles-hint = Los grupos cuyos miembros reciben cada rol, uno por línea, por nombre o DN. Gana el más alto que coincida, y se comprueba en cada inicio de sesión. Si no hay ninguno, las cuentas nuevas reciben el rol de arriba y un administrador lo cambia después. Si hay grupos en Puede ver, solo pueden iniciar sesión los miembros de alguno de ellos.
auth-roles-hint-oauth = Los grupos cuyos miembros reciben cada rol, uno por línea, por nombre o DN (o lo que enumere la notificación de grupos). Gana el más alto que coincida, y se comprueba en cada inicio de sesión. Si no hay ninguno, las cuentas nuevas reciben el rol de arriba y un administrador lo cambia después. Si hay grupos en Puede ver, solo pueden iniciar sesión los miembros de alguno de ellos.
auth-saving = Guardando…
auth-save = Guardar

## HttpsPanel.vue (Settings → Connection)

https-load-failed = No se ha podido cargar HTTPS: { $error }
https-off-redirecting = HTTPS está desactivado. Llevándote a http://
https-now-on = HTTPS está activado
https-saved = Guardado. Las conexiones nuevas usan el certificado nuevo.
https-now-off = HTTPS está desactivado
https-http3-failed-toast = No se ha podido iniciar HTTP/3: { $error }
https-copied = Copiado
https-copy-failed = No se ha podido copiar: selecciónalo y cópialo tú
https-mode-off = Desactivado
https-mode-off-hint = Solo http:// sin cifrar.
https-mode-own = El certificado propio de Hoard
https-mode-own-hint = Hoard crea uno para las direcciones de este servidor. Instala una vez la entidad de certificación de Hoard en cada dispositivo y sus navegadores confiarán en él.
https-mode-custom = Mi certificado
https-mode-custom-hint = Uno que ya tengas, p. ej. de tailscale cert o de Let’s Encrypt. Los navegadores confían en él sin instalar nada.
https-untrusted-own = Este navegador todavía no confía en él para { $host }: instala abajo la entidad de certificación de Hoard. Verás una advertencia que puedes saltarte.
https-untrusted-uncovered = Este navegador todavía no confía en él para { $host }: el certificado no cubre esta dirección. Verás una advertencia que puedes saltarte.
https-untrusted = Este navegador todavía no confía en él para { $host }. Verás una advertencia que puedes saltarte.
https-now = Ahora
https-start-failed = HTTPS está configurado, pero no se ha podido iniciar: { $error }
https-on-port = HTTPS está activado, en el puerto { $port }, como http://.
https-using-it = Lo estás usando ahora.
https-open = Abrir { $url }
https-trusted = Este navegador confía en él.
https-cert-for-before = Certificado para{ " " }
https-cert-for-after = , hasta el { $date }.
https-cert-for-after-own = , hasta el { $date } (se renueva solo).
https-not-covered-before = No cubre{ " " }
https-not-covered-after = , la dirección que estás usando.
https-is-off = HTTPS está desactivado: las páginas se sirven sin cifrar, por http://.
https-addresses = Direcciones
https-addresses-hint = El certificado cubre las direcciones que ha encontrado Hoard y las que añadas (un nombre que le dé tu router, un nombre de Tailscale).
https-remove-name = Quitar { $name }
https-another-address = Otra dirección
https-another-address-placeholder = Otra dirección, p. ej. nas.local
https-add = Añadir
https-your-cert = Tu certificado
https-saved-cert-before = Guardado:{ " " }
https-saved-cert-after = { " " }de { $issuer }, hasta el { $date }. Pega uno nuevo para sustituirlo.
https-pem-before = El certificado (con su cadena) y su clave privada, como texto PEM: los archivos que empiezan por{ " " }
https-pem-after = .
https-certificate = Certificado
https-choose-file = Elegir archivo…
https-private-key = Clave privada
https-checked-before = Para{ " " }
https-checked-after = , de { $issuer }, hasta el { $date }.
https-checked-after-days =
    , de { $issuer }, hasta el { $date } ({ $days ->
        [one] { $days } día
       *[other] { $days } días
    }).
https-self-signed = Está autofirmado: los navegadores avisarán.
https-key-matches = La clave coincide.
https-where-to-get = Dónde conseguir uno
https-get-tailscale-before = activa HTTPS en tu tailnet, luego ejecuta{ " " }
https-get-tailscale-after = { " " }en el servidor y sube los dos archivos. Duran 90 días: vuelve a ejecutarlo y súbelos de nuevo antes de que caduquen.
https-get-acme-title = Let’s Encrypt u otro cliente ACME
https-get-acme-before = (certbot, acme.sh, lego) para un dominio tuyo: sube{ " " }
https-get-acme-and = { " " }y{ " " }
https-get-acme-after = .
https-get-proxy-title = Un proxy inverso
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) que tenga el certificado: deja HTTPS desactivado aquí y deja que el proxy envíe http:// sin cifrar a Hoard.
https-trying = Probándolo…
https-apply = Aplicar
https-apply-hint = Hoard prueba primero la nueva configuración, y mantiene la que tiene si no funciona.
https-trust-title = Confiar en Hoard en tus dispositivos
https-download-ca = Descargar hoard-ca.crt
https-trust-hint = Instala una vez la entidad de certificación de Hoard en cada dispositivo, y sus navegadores confiarán en este servidor (y en cualquier certificado que Hoard cree para él más adelante). Solo puede responder por este Hoard. Huella digital (SHA-256):
https-ca-windows = Abre el archivo → Instalar certificado → Usuario actual → Colocar todos los certificados en el siguiente almacén → Examinar → Entidades de certificación raíz de confianza → Finalizar. Reinicia el navegador.
https-ca-macos = Abre el archivo; se añade a Acceso a Llaveros. Haz doble clic en «{ $subject }» → Confiar → Al utilizar este certificado: Confiar siempre.
https-ca-ios-title = iPhone y iPad
https-ca-ios = Descárgalo en Safari y permítelo. Ajustes → Perfil descargado → Instalar. Luego Ajustes → General → Información → Ajustes de confianza de certificados → activa «{ $subject }».
https-ca-android = Ajustes → Seguridad → Más ajustes de seguridad → Cifrado y credenciales → Instalar un certificado → Certificado de CA, y elige el archivo. (Los nombres cambian un poco de un teléfono a otro; busca «certificado» en Ajustes).
https-ca-linux-1 = Debian y Ubuntu:{ " " }
https-ca-linux-2 = . Fedora y Arch:{ " " }
https-ca-linux-3 = . Chrome en Linux tiene su propia lista: Configuración → Privacidad y seguridad → Seguridad → Gestionar certificados → Importar.
https-ca-firefox = Firefox tiene su propia lista: Ajustes → Privacidad & Seguridad → Certificados → Ver certificados → Autoridades → Importar, y marca «Confiar en esta CA para identificar sitios web».
https-ca-cli-title = La herramienta de línea de comandos hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , o pon la ruta en{ " " }
https-ca-cli-after = .
https-options = Opciones
https-redirect = Enviar los navegadores de otros dispositivos de http:// a https://
https-redirect-hint = Solo se redirigen las cargas de páginas. La herramienta de línea de comandos hoard, el worker de Unreal, los scripts y las aplicaciones de IA siguen funcionando por http://, igual que http://localhost en el propio servidor.
https-http3-hint-before = Más rápido con wifi inestable y en teléfonos. Necesita HTTPS, y el puerto UDP { $port } abierto además del TCP (en Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = En funcionamiento.
https-http3-waits = Se inicia cuando HTTPS está activado.
https-http3-failed = No se ha podido iniciar HTTP/3: { $error }
https-lockout-title = Si te quedas fuera
https-lockout-1 = { "" }
https-lockout-2 = { " " }en el propio servidor siempre funciona sin cifrar, por http://. O desactiva HTTPS desde la línea de comandos del servidor y vuelve a iniciar Hoard:{ " " }
https-lockout-3 = { " " }(en Docker:{ " " }
https-lockout-4 = , luego{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Puede ver
account-role-view-hint = Buscar, ver la vista previa y descargar
account-role-edit = Puede cambiar
account-role-edit-hint = Además, añadir, cambiar, etiquetar, coleccionar y archivar
account-role-admin = Administrador
account-role-admin-hint = Además, Ajustes, cuentas y mantenimiento, y eliminar definitivamente
account-source-internal = Una contraseña de Hoard
account-source-ldap = Su contraseña de { $directory }
account-source-oauth = Iniciando sesión con { $provider }
account-source-word-directory = directorio
account-load-failed = No se han podido cargar las cuentas: { $error }
account-failed = { $what }: { $error }
account-password-failed = No se ha podido cambiar tu contraseña
account-password-changed = Contraseña cambiada. Se ha cerrado la sesión en tus otros dispositivos.
account-add-failed = No se ha podido añadir a { $name }
account-added = { $name } ya puede iniciar sesión.
account-role-failed = No se ha podido cambiar lo que puede hacer { $name }
account-remove-title = ¿Quitar a { $name }?
account-remove-body = Se cierra su sesión en todas partes y no puede volver a iniciarla. No se quita nada de lo que haya añadido a la biblioteca.
account-remove = Quitar
account-remove-failed = No se ha podido quitar a { $name }
account-token-failed = No se ha podido crear el token
account-copied = Copiado
account-copy-token-failed = No se ha podido copiar: selecciona el token y cópialo tú
account-copy-failed = No se ha podido copiar: selecciónalo y cópialo tú
account-revoke-title = ¿Revocar «{ $name }»?
account-revoke-body = A partir de ahora se rechaza todo lo que aún lo use.
account-revoke = Revocar
account-revoke-failed = No se ha podido revocar «{ $name }»
account-disconnect-title = ¿Desconectar { $app }?
account-disconnect-body = A partir de ahora se rechaza, y hay que volver a conectarla para que use Hoard.
account-disconnect = Desconectar
account-disconnect-failed = No se ha podido desconectar { $app }
account-guest-failed = No se ha podido cambiar
account-never = nunca
account-you = Tú
account-sign-out = Cerrar sesión
account-signed-in-as-before = Sesión iniciada como{ " " }
account-signed-in-as-admin = , que es administrador.
account-signed-in-as-edit = , que puede cambiar la biblioteca.
account-signed-in-as-view = , que puede ver pero no cambiar nada.
account-app-signin = La aplicación Hoard inicia sesión por ti, así que no hay contraseña. Crea abajo un token de acceso para los complementos de los motores y el comando hoard.
account-password-at-provider = Tu contraseña la guarda el proveedor con el que inicias sesión, así que se cambia allí.
account-password-at-directory = Tu contraseña la guarda el directorio con el que inicias sesión, así que se cambia allí.
account-current-password = Contraseña actual
account-new-password = Contraseña nueva
account-new-password-again = Repite la contraseña nueva
account-change-password = Cambiar contraseña
account-password-mismatch = Las contraseñas nuevas no coinciden.
account-password-rule = Al menos 8 caracteres.
account-people = Personas
account-people-hint = Cada persona puede ver (buscar, ver la vista previa y descargar), también cambiar la biblioteca (añadir, editar, etiquetar, coleccionar y archivar) o, además, ser administrador (Ajustes, cuentas y mantenimiento, y eliminar definitivamente).
account-you-tag = (tú)
account-signs-in-oauth = Inicia sesión con el proveedor OAuth
account-signs-in-directory = Inicia sesión con el directorio
account-since = desde el { $date }
account-what-can-do = Lo que puede hacer { $name }
account-name = Nombre
account-how-sign-in = Cómo inicia sesión
account-password-label = Contraseña (al menos 8 caracteres)
account-password-placeholder = Contraseña (8 caracteres o más)
account-what-they-can-do = Lo que puede hacer
account-add = Añadir
account-ai-apps = Claude y otras aplicaciones de IA
account-ai-apps-hint = Conecta Claude (o cualquier aplicación MCP) y podrá buscar y obtener recursos por ti, y describir y etiquetar lo que Hoard aún no ha hecho. Inicia sesión en Hoard la primera vez; tú lo permites aquí, en el navegador.
account-mcp-address = Dirección MCP de Hoard
account-copy = Copiar
account-claude-code-1 = ejecuta{ " " }
account-claude-code-2 = , luego{ " " }
account-claude-code-3 = { " " }para iniciar sesión.{ " " }
account-claude-code-4 = { " " }describe un paquete.
account-claude-ai = Configuración → Conectores → Añadir conector personalizado, pega la dirección y luego Conectar. Se conectan desde los servidores de Anthropic, así que Hoard tiene que ser accesible desde internet por HTTPS.
account-claude-ai-local = Configuración → Conectores → Añadir conector personalizado, pega la dirección y luego Conectar. Se conectan desde los servidores de Anthropic, así que Hoard tiene que ser accesible desde internet por HTTPS (esta dirección no lo es: indica la Dirección pública en Ajustes → Conexión).
account-connection-meta = de { $user } desde { $since } · último uso: { $used }
account-no-apps = Todavía no hay aplicaciones conectadas.
account-live = Usando Hoard ahora
account-live-hint = Aplicaciones que han pedido algo a Hoard en los últimos 10 minutos. Se actualiza solo.
account-live-none = Nada ha pedido nada a Hoard en los últimos 10 minutos.
account-live-last = último: { $what } · { $when } · peticiones: { $calls }
account-tokens = Tokens de acceso
account-tokens-hint-1 = Para la herramienta de línea de comandos{ " " }
account-tokens-hint-2 = , los scripts, los clientes MCP y el worker de Unreal: lo envían como{ " " }
account-tokens-hint-3 = . Un token puede hacer lo que elijas, hasta lo que puedes hacer tú, pero nunca gestionar cuentas ni tokens. Con uno que solo puede ver basta para buscar y descargar, y para sincronizar un proyecto.
account-tokens-admin = Ves los tokens y las aplicaciones conectadas de todos; los demás ven los suyos.
account-made-before = { "" }
account-made-after = : cópialo ahora. No se volverá a mostrar.
account-new-token = El token nuevo
account-cli-1 = Para la CLI:{ " " }
account-cli-2 = , o ponlo en{ " " }
account-cli-3 = .
account-done = Hecho
account-token-meta = creado { $made } · último uso: { $used }
account-token-meta-by = creado { $made } por { $by } · último uso: { $used }
account-no-tokens = Todavía no hay tokens.
account-token-name = Qué lo va a usar
account-token-name-placeholder = Qué lo va a usar, p. ej. «Worker de Unreal»
account-token-role = Lo que puede hacer
account-make-token = Crear un token
account-guests = Invitados
account-guests-allow = Permitir que cualquiera que llegue al servidor explore sin iniciar sesión
account-guests-hint = Pueden buscar, ver la vista previa y descargar, pero no añadir, cambiar ni eliminar nada, ni ver los Ajustes.

## SignIn.vue

signin-create-title = Crea tu cuenta
signin-title = Iniciar sesión
signin-setup-lede = Este servidor todavía no tiene ninguna cuenta. La que crees aquí puede hacerlo todo; más adelante, añade cuentas para otras personas desde Ajustes → Cuenta.
signin-overlay-lede = Aquí cualquiera puede explorar, pero para hacer cambios hace falta una cuenta.
signin-with = Iniciar sesión con { $provider }
signin-or-password = o con un nombre y una contraseña
signin-username = Nombre de usuario
signin-name = Nombre
signin-new-password = Contraseña nueva
signin-password = Contraseña
signin-password-rule = Al menos 8 caracteres.
signin-confirm-password = Confirmar contraseña
signin-mismatch = Las dos contraseñas no coinciden.
signin-setup-code = Código de configuración
signin-setup-code-hint-before = El código que se le dio a este servidor en{ " " }
signin-setup-code-hint-after = .
signin-creating = Creando…
signin-signing-in = Iniciando sesión…
signin-create = Crear cuenta
signin-submit = Iniciar sesión
signin-keep-browsing = Seguir explorando
signin-directory-hint = Usa tu nombre y tu contraseña de { $directory }.
signin-forgot-1 = ¿Has olvidado una contraseña de Hoard? En el servidor, ejecuta{ " " }
signin-forgot-2 = { " " }(en Docker,{ " " }
signin-forgot-3 = ) para obtener una nueva.

## SetupWizard.vue (first visit to a new server)

setup-title = ¿Dónde quieres guardar tu biblioteca?
setup-lede = Elige una carpeta del servidor en la que Hoard guarde tus paquetes, o indícale una carpeta con paquetes que ya tengas. Más adelante puedes añadir más ubicaciones, en otros discos, desde Ajustes.
setup-lede-desktop = Elige una carpeta en la que Hoard guarde tus paquetes, o indícale una carpeta con paquetes que ya tengas. Más adelante puedes añadir más ubicaciones, en otros discos, desde Ajustes.
setup-unreachable = No se ha podido conectar con el servidor: { $error }
setup-unreachable-desktop = Hoard no responde: { $error }
setup-try-again = Volver a intentarlo
setup-loading = Cargando…
setup-start = Crear la biblioteca
setup-ready = Tu biblioteca está lista
setup-managed-before = Los paquetes nuevos van a{ " " }
setup-managed-after = . Suelta archivos, carpetas o archivos comprimidos en cualquier parte de la página para añadir los primeros.
setup-watched-before = Hoard está vigilando{ " " }
setup-watched-after = ; los paquetes aparecen a medida que los encuentra. Para poder subir también, añade una biblioteca que gestione Hoard desde Ajustes → Ubicaciones de la biblioteca.
setup-open = Abrir la biblioteca

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = ¿Conectar { $app }?
connect-lede-1 = { " " }podrá buscar, descargar, describir y etiquetar todo lo que hay en este Hoard, en nombre de{ " " }
connect-lede-2 = . No puede gestionar cuentas ni tokens de acceso.
connect-back-before = Volverás a{ " " }
connect-back-after = . Puedes desconectarla cuando quieras en Ajustes → Cuenta.
connect-allow = Permitir
connect-cancel = Cancelar
connect-failed = No se puede conectar esta aplicación
connect-go-library = Ir a la biblioteca
connect-checking = Comprobando…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = Esto es http://, sin cifrar, así que algunas cosas necesitan HTTPS: el 3D con WebGPU, el portapapeles moderno, instalar Hoard como aplicación y los conectores de Claude.
insecure-switch = Cambiar a https://
insecure-set-up = Configurar HTTPS
insecure-dismiss = No volver a mostrar

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Este archivo es demasiado grande para leerlo como archivo de tienda.
formatwizard-not-json = Este archivo no es un JSON válido, así que se muestra como texto.
formatwizard-read-failed = No se ha podido leer el archivo: { $error }
formatwizard-back-title = ¿Volver al formulario?
formatwizard-back-body = Se pierden los cambios hechos en el TOML: el formulario vuelve a escribir el formato a partir de lo que tiene.
formatwizard-back-to-form = Volver al formulario
formatwizard-miss-name = Sus archivos no aceptan el nombre { $file }. Añádelo en Archivos.
formatwizard-miss-not-the-stores = Lo que usa para reconocer los archivos de la tienda no está en este archivo. Cambia Reconocer por.
formatwizard-miss-no-title = No encuentra ningún título en este archivo. Elige el título.
formatwizard-miss-short-name = no es su nombre
formatwizard-miss-short-not-the-stores = no es de la tienda
formatwizard-miss-short-no-title = sin título
formatwizard-saved-packs =
    Se ha guardado { $file }. { $packs ->
        [one] Se está volviendo a leer { $packs } paquete.
       *[other] Se están volviendo a leer { $packs } paquetes.
    }
formatwizard-saved = Se ha guardado { $file }.
formatwizard-save-failed = No se ha podido guardar el formato: { $error }
formatwizard-zip-failed = No se ha podido crear el zip: { $error }
formatwizard-stop-title = ¿Dejar de enseñar este archivo?
formatwizard-stop-body = El formato no se guarda.
formatwizard-stop = Dejar
formatwizard-label = Asistente de formatos
formatwizard-changing = Cambiando { $name }
formatwizard-starting-from = A partir de { $name }
formatwizard-title = Enseña a Hoard el archivo de una tienda
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }en{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Editar como TOML
formatwizard-file-reference = El archivo, como referencia.
formatwizard-click-json = Haz clic en { $field } en el archivo.
formatwizard-click-text = Haz clic en { $field } en el archivo, o selecciónalo.
formatwizard-pick-first = Elige un campo a la derecha y luego haz clic aquí en su valor.
formatwizard-loading = Cargando…
formatwizard-the-format = El formato
formatwizard-syntax-before = La sintaxis está en{ " " }
formatwizard-syntax-after = .
formatwizard-store = Tienda
formatwizard-store-placeholder = El nombre de la tienda, como Kenney
formatwizard-files = Archivos
formatwizard-recognize = Reconocer por
formatwizard-recognize-json = Una clave que solo tienen los archivos de la tienda
formatwizard-recognize-text = Una línea que solo tienen los archivos de la tienda
formatwizard-recognize-hint = Una forma por línea; vale cualquiera de ellas. Hace falta cuando otros archivos tienen el mismo nombre.
formatwizard-seller = Vendedor
formatwizard-seller-placeholder = Cuando el archivo no lo dice
formatwizard-tags = Etiquetas
formatwizard-tags-placeholder = Para todo lo que describe, separadas por comas
formatwizard-fields = Campos
formatwizard-selector = Selector del campo { $field }
formatwizard-clear = Borrar el campo { $field }
formatwizard-pick = Elegir
formatwizard-what-it-reads = Lo que lee
formatwizard-read-title = Título
formatwizard-read-seller = Vendedor
formatwizard-read-license = Licencia
formatwizard-read-url = Página de la tienda
formatwizard-read-kind = Tipo
formatwizard-read-tags = Etiquetas
formatwizard-read-origin = Hecho en
formatwizard-read-size = Tamaño
formatwizard-also = { $format } también lee este archivo. Este formato se probaría primero.
formatwizard-library =
    { $files ->
        [one] Tiene un nombre que acepta { $files } archivo
       *[other] Tienen un nombre que acepta { $files } archivos
    } en { $packs ->
        [one] { $packs } paquete
       *[other] { $packs } paquetes
    }. De { $tried } probados, lee { $read }.
formatwizard-library-none = Ningún archivo de la biblioteca tiene un nombre que acepte.
formatwizard-share = Compartirlo
formatwizard-share-hint = Un zip con el formato, este archivo como muestra y lo que se lee de la muestra, organizado como el repositorio de Hoard, para enviarlo y que lo tenga todo el mundo. Antes, quita de la muestra lo que no se deba compartir.
formatwizard-sample = Muestra para compartir
formatwizard-download = Descargar { $file }
formatwizard-close = Cerrar
formatwizard-use = Usarlo aquí
formatwizard-cancel = Cancelar
settings-unreal-editor-what = Instala Unreal Editor para el proceso de exportación. Ejecútalo por separado para exportar recursos.
settings-unreal-download = Descarga el editor Linux x86-64 de Epic. Requiere acceso vinculado a GitHub y decenas de GB en disco.
settings-unreal-image = Versión o imagen de Unreal
settings-editor-path = Introduce la ruta del ejecutable de { $tool } instalado. La instalación automática solo está disponible en Docker.
error-unreal-image = Introduce una versión de Unreal 5, una etiqueta de versión dev/dev-slim de Epic o una referencia de imagen SHA-256 de Epic.
