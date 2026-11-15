## Errori del server

error-account-not-found = account non trovato
error-asset-gone = asset non trovato
error-asset-not-found = asset { $id } non trovato
error-blob-not-found = blob { $hash } non trovato
error-collection-not-found = raccolta { $id } non trovata
error-connection-not-found = connessione non trovata
error-derivative-not-found = derivato { $kind } di { $id } non trovato
error-file-in-pack-not-found = { $path } non trovato nel pacchetto { $id }
error-format-not-found = formato { $file } non trovato
error-frame-sheet-aseprite-only = solo i file Aseprite hanno un foglio di fotogrammi
error-frame-sheet-frames = ha un solo fotogramma, o troppi da disporre
error-location-not-found = posizione { $id } non trovata
error-maintenance-task-not-found = attività di manutenzione { $id } non trovata
error-open-upload-not-found = caricamento aperto { $id } non trovato
error-original-not-found = file originale di { $id } non trovato
error-pack-not-found = pacchetto { $id } non trovato
error-preview-too-big = questa immagine è troppo grande per farne un'anteprima leggera
error-project-not-found = progetto { $name } non trovato
error-saved-search-not-found = ricerca salvata { $id } non trovata
error-sign-in-first = accedi prima (o invia un token di accesso)
error-those-assets-not-found = asset non trovati
error-thumbnail-not-found = miniatura di { $id } non trovata
error-token-not-found = token non trovato
error-upload-not-found = caricamento { $id } non trovato
error-pack-needs-name = un pacchetto ha bisogno di un nome
error-no-library = non c'è ancora una posizione della libreria in cui salvare i file: aggiungine una gestita in Impostazioni → Posizioni della libreria
error-upload-failed = caricamento non riuscito: { $reason }
error-upload-no-files = nessun file nel caricamento
error-analyze-nothing-to-do = niente da fare: rules=false e l'LLM è disattivato
error-too-many-assets = troppi asset in una sola richiesta
error-too-many-items = troppi elementi in una sola richiesta
error-not-a-hash = non è un hash blake3 esadecimale
error-merge-into-tag = into deve essere un tag
error-nothing-to-merge = niente da unire
error-selection-size = indica da 1 a { $max } ID di asset
error-collection-needs-name = una raccolta ha bisogno di un nome
error-collection-exists = esiste già una raccolta chiamata { $name }
error-collection-empty = la raccolta { $name } è vuota
error-conversion-failed = impossibile convertire in { $format }: { $reason }
error-max-size = max deve essere una potenza di due da 64 a 16384
error-replace-ext = ext deve essere una breve estensione di file
error-empty-file = file vuoto
error-replace-duplicate = il file convertito è il duplicato di uno già presente nel pacchetto; il package è stato unito a quello
error-ai-address = L'indirizzo dell'IA deve iniziare con http:// o https://
error-local-llm-action = action è list, download o delete, e gli ultimi due richiedono un modello
error-role = Un ruolo è view, edit o admin
error-admin-only = solo un amministratore può farlo
error-view-only = questo account può guardare ma non modificare nulla: un amministratore può cambiarlo in Impostazioni → Account
error-portal-only = accedi al portale per gestire account e token
error-not-from-portal = questa richiesta non proviene dal portale di Hoard
error-account-name = Scegli un nome di massimo 64 caratteri
error-password-length = Usa una password di almeno { $min } caratteri
error-too-many-codes = Troppi codici errati: attendi { $seconds } secondi
error-already-set-up = Esiste già un account: accedi
error-too-many-tries = Troppi tentativi errati: attendi { $seconds } secondi
error-directory-unavailable = La directory non ha potuto verificare la tua password: { $reason }
error-password-elsewhere = La tua password è gestita dalla directory o dal provider con cui accedi
error-current-password = La password attuale non è corretta
error-account-source = Un account accede con Hoard (internal), una directory (ldap) o OAuth
error-account-exists = Esiste già un account con quel nome
error-own-role = Il tuo ruolo può cambiarlo un altro amministratore
error-remove-self = Non puoi rimuovere il tuo account mentre hai effettuato l'accesso con esso
error-token-name = Dai al token il nome di ciò che lo userà, in massimo 80 caratteri
error-token-role = Un token non può fare più dell'account che lo crea
error-wrong-setup-code = Non è il codice di configurazione dato al server in HOARD_SETUP_CODE
error-wrong-password = Nome e password non corrispondono
error-folder-missing = { $path } non c'è (un disco non montato?)
error-s3-path = una posizione S3 ha la forma s3://bucket
error-openai-auth-unreachable = impossibile raggiungere auth.openai.com
error-openai-unexpected = risposta inattesa da OpenAI
error-openai-answered = OpenAI ha risposto { $status }: { $reason }
error-chatgpt-id-token-not-jwt = l'ID token non è un JWT
error-chatgpt-id-token-issuer = l'ID token proviene da { $issuer }, non da { $expected }
error-chatgpt-id-token-audience = l'ID token è per un'altra app
error-chatgpt-id-token-expired = l'ID token è scaduto
error-chatgpt-id-token-nonce = l'ID token non appartiene a questo accesso
error-chatgpt-no-state = l'indirizzo non ha uno state; copia l'indirizzo completo
error-chatgpt-not-started = questo accesso non è stato avviato qui, o è stato avviato più di 15 minuti fa. Avvialo di nuovo dalle Impostazioni.
error-chatgpt-canceled = l'accesso è stato annullato
error-chatgpt-refused = ChatGPT ha rifiutato l'accesso: { $reason }
error-chatgpt-no-code = l'indirizzo non ha un codice di accesso; copia l'indirizzo completo
error-chatgpt-other-registration = ChatGPT ha risposto per un'altra registrazione di questo server; avvia di nuovo l'accesso
error-chatgpt-not-registered = ChatGPT non ha completato la registrazione di Hoard; avvia di nuovo l'accesso
error-chatgpt-code-used = il codice di accesso è già stato usato o è scaduto; accedi di nuovo
error-chatgpt-refused-code = ChatGPT ha rifiutato l'accesso ({ $code })
error-chatgpt-no-id-token = ChatGPT non ha inviato nessun ID token
error-chatgpt-plan-not-allowed = Accesso effettuato, ma l'uso del piano non è stato consentito. Accedi di nuovo e consentilo, oppure usa una chiave API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = accesso a ChatGPT non effettuato; accedi da Impostazioni → Tag con IA
error-chatgpt-refresh-later = ChatGPT non è riuscito a rinnovare l'accesso in questo momento ({ $code }); si riproverà più tardi
error-chatgpt-signed-out-problem = ChatGPT ha disconnesso questo server ({ $code }). Accedi di nuovo per continuare ad assegnare tag con il tuo piano.
error-chatgpt-signed-out = ChatGPT ha disconnesso questo server ({ $code }); accedi di nuovo da Impostazioni → Tag con IA
error-openai-api-unreachable = impossibile raggiungere api.openai.com
error-openai-broke-off = la risposta di OpenAI si è interrotta
error-chatgpt-kept-refusing = OpenAI ha continuato a rifiutare l'accesso a ChatGPT; accedi di nuovo da Impostazioni → Tag con IA
error-chatgpt-request-failed = la richiesta non è riuscita
error-chatgpt-cut-off = la risposta è stata troncata ({ $reason })
error-chatgpt-cut-off-no-reason = la risposta è stata troncata (nessun motivo indicato)
error-openai-ended-early = la risposta di OpenAI è finita prima di essere completa
error-chatgpt-usage-limit = l'utilizzo del tuo piano ChatGPT per Hoard è esaurito per ora. Controlla o modifica il limite su { $url }
error-chatgpt-not-eligible = questo account o workspace ChatGPT non può usare il suo piano in altre app. Usa invece una chiave API.
error-chatgpt-usage-unavailable = ChatGPT non è riuscito a verificare l'utilizzo del piano in questo momento; si riproverà più tardi
error-chatgpt-unsupported = l'utilizzo del piano ChatGPT non supporta { $param } con questo modello
error-chatgpt-unsupported-images = l'utilizzo del piano ChatGPT non supporta { $param } con questo modello; scegli un modello che accetta immagini, o disattiva l'invio delle miniature
error-chatgpt-unsupported-part = l'utilizzo del piano ChatGPT non supporta una parte della richiesta con questo modello
error-chatgpt-port = port è la porta su cui il browser raggiunge Hoard
error-chatgpt-paste-address = incolla l'indirizzo completo, che inizia con http://127.0.0.1
error-too-many-hashes = al massimo { $max } hash alla volta
error-credits-pack = pacchetto { $key }
error-credits-collection = raccolta { $key }
error-format-too-long = il formato è troppo lungo
error-not-started = il server non è stato avviato
error-not-format-file = non è un file di un formato: { $name }
error-format-name = il nome ha bisogno di una lettera o cifra da A a Z o da 0 a 9
error-format-miss-name = i file del formato non accettano { $file }
error-format-miss-store = il formato non accetta { $file } per quelli dello store (vedi require)
error-format-miss-title = il formato non trova alcun titolo in { $file }
error-cant-download-as = i file .{ $ext } non possono essere scaricati come { $format }
error-only-images-resize = solo le immagini possono essere ridimensionate
error-https-no-certificate = Prima incolla o carica il certificato e la sua chiave
error-https-no-cert-found = Nessun certificato trovato: incolla il testo che inizia con -----BEGIN CERTIFICATE-----
error-https-no-key-found = Nessuna chiave privata trovata: incolla il testo che inizia con -----BEGIN PRIVATE KEY-----
error-https-several-keys = C'è più di una chiave privata: incolla solo quella che va con il certificato
error-https-key-password = La chiave è protetta da password: forniscila senza (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Il certificato non è un PEM valido
error-https-key-not-pem = La chiave privata non è un PEM valido
error-https-key-mismatch = La chiave non appartiene al certificato. Se hai fornito una catena, metti per primo il certificato del tuo server
error-https-unusable = Il certificato o la chiave non possono essere usati
error-https-expired = Questo certificato è scaduto il { $date }
error-https-not-yet-valid = Questo certificato non è valido fino al { $date }
error-https-unreadable = Non è un certificato che Hoard sa leggere
error-https-bad-name = «{ $name }» non è un indirizzo che un certificato può contenere
error-https-test-timed-out = la connessione di prova è scaduta
error-https-test-failed = la connessione di prova non è riuscita
error-https-no-ca = Hoard non ha ancora creato la sua autorità di certificazione
error-https-unavailable = HTTPS non è disponibile su questo server
error-https-certificate-mode = Un certificato va con «Il mio certificato»
error-https-test = Il certificato non ha funzionato in una connessione di prova: { $reason }
error-nothing-to-retry = non c'è niente da riprovare
error-ai-unreachable = impossibile raggiungere { $url }
error-ai-key-refused = { $url } ha rifiutato la chiave API ({ $status })
error-ai-answered = { $url } ha risposto { $status }: { $reason }
error-ai-not-json = { $url }/models non ha risposto in JSON
error-model-server-refused = il server dei modelli: { $reason }
error-no-model-server = nessun server dei modelli risponde su { $url }
error-not-llama-router = { $url } non è un server llama.cpp in modalità router, quindi non può scaricare modelli
error-model-download = impossibile scaricare { $model }: { $reason }
error-model-delete = impossibile eliminare { $model }: { $reason }
error-unknown-kind = tipo di asset sconosciuto { $kind }
error-watched-is-folder = una posizione monitorata è una cartella su questo server
error-cant-reach = impossibile raggiungere { $path }: { $reason }
error-folder-full-path = indica il percorso completo della cartella
error-cant-create = impossibile creare { $path }
error-cant-write = impossibile scrivere in { $path }
error-cant-read = impossibile leggere { $path }
error-overlaps-location = si sovrappone alla posizione { $name } ({ $path })
error-overlaps-data = si sovrappone alla cartella dei dati di Hoard ({ $path })
error-overlaps-cache = si sovrappone alla cartella della cache di Hoard ({ $path })
error-not-watched = { $name } non è una cartella monitorata
error-watched-no-kinds = una cartella monitorata non accetta nuovi file, quindi non ha tipi da modificare
error-keep-catch-all = imposta invece un'altra posizione come destinazione predefinita
error-location-holds-files = { $name } contiene ancora file: spostali prima nelle altre posizioni (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } non è raggiungibile, quindi Hoard non può sapere se contiene ancora file
error-relocate-running = i file vengono già spostati tra le posizioni; riprova quando l'operazione è finita
error-watched-not-moved = { $name } è una cartella monitorata: Hoard non ne sposta i file
error-no-other-location = non c'è un'altra posizione della libreria in cui spostare i file: aggiungine prima una
error-full-path = indica un percorso completo
error-pick-folder = scegli una cartella
error-task-running = «{ $task }» è già in esecuzione
error-token-account-gone = l'account di questo token non esiste più, quindi non ha preferiti né ricerche
error-favorites-too-many = al massimo 10.000 alla volta
error-saved-search-name = una ricerca salvata ha bisogno di un nome di massimo 80 caratteri
error-saved-search-too-long = questa ricerca è troppo lunga per essere salvata
error-texture-pattern-map = Lo schema dei nomi delle texture ha bisogno di { "{" }map{ "}" }, altrimenti tutte le mappe di un materiale avrebbero lo stesso nome
error-texture-pattern-fields = Lo schema dei nomi delle texture può usare { "{" }name{ "}" }, { "{" }map{ "}" } e { "{" }size{ "}" }
error-texture-name-chars = I nomi delle texture non possono contenere < > : " / \ | ? o *
error-texture-map-type = Non esiste un tipo di mappa chiamato { $role }
server-role-base-color = colore
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = rugosità
server-role-gloss = lucidità
server-role-metalness = metallicità
server-role-ao = occlusione ambientale
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = altezza
server-role-bump = bump
server-role-cavity = cavità
server-role-specular = specularità
server-role-opacity = opacità
server-role-emissive = emissione
server-role-translucency = traslucenza
error-texture-suffix-missing = Assegna un suffisso alle mappe { $map }
error-texture-suffix-chars = Il suffisso delle mappe { $map } non può contenere < > : " / \ | ? o *
error-oauth-no-client-id = Questo link non ha un client_id.
error-oauth-unknown-app = Questa app non è registrata qui (o non lo è più). Ricomincia la connessione dall'app.
error-oauth-redirect = L'app ha chiesto di essere rimandata a un indirizzo che non ha registrato.
error-oauth-response-type = Hoard rilascia solo codici di accesso (response_type=code).
error-oauth-pkce = L'app deve usare PKCE con S256 per accedere a Hoard.
error-light-preview-images = solo le immagini hanno un'anteprima leggera
error-no-light-preview = nessuna anteprima leggera: { $reason }
error-no-frame-sheet = nessun foglio di fotogrammi: { $reason }
error-thumbnail-source = origine della miniatura sconosciuta: { $source }
error-thumbnail-too-big = quell'immagine è troppo grande per una miniatura
error-thumbnail-models-only = solo i modelli accettano una miniatura disegnata da un browser
error-not-a-picture = non è un'immagine che Hoard sa leggere: { $reason }
error-project-name = un progetto ha bisogno di un nome di massimo 200 caratteri
error-project-too-big = al massimo un milione di asset in un progetto
error-project-too-many-at-once = al massimo 100.000 alla volta
error-server-name = Scegli un nome di massimo { $max } caratteri, su una sola riga
error-tool-setting = { $tool }: scegli disattivato, automatico, o indica un percorso
error-ai-model = Scegli il modello che il servizio di IA deve usare
error-public-address = L'indirizzo pubblico deve iniziare con http:// o https://
error-update-hook-address = L'indirizzo da chiamare per gli aggiornamenti deve iniziare con http:// o https://
error-update-hook-auth = L'intestazione Authorization non può contenere interruzioni di riga o altri caratteri di controllo
error-no-soundfont = Non c'è nessun SoundFont in { $path } sul server
error-workers = Processi in background: da 1 a { $max }
error-ai-batch = Asset per richiesta all'IA: da 1 a 200
error-conversion-cache = Download convertiti: almeno 100 MB
error-limits-zero = I limiti non possono essere zero
error-settings-object = era atteso un oggetto JSON di impostazioni
error-bad-setting = impostazione non valida: { $reason }
error-auth-default-role = Il ruolo predefinito è view, edit o admin
error-auth-session-hours = Le sessioni durano da 1 ora a un anno
error-auth-settings = impostazioni di autenticazione non valide: { $reason }
error-ldap-settings = impostazioni LDAP non valide: { $reason }
error-ldap-no-server = Aggiungi l'indirizzo del server della directory
error-ldap-base-dn = Indica dove si trovano le voci delle persone (il base DN)
error-ldap-username-attribute = Indica quale attributo contiene il nome con cui le persone accedono
error-ldap-bind-dn = Indica l'account che cerca le persone, oppure cercale in modo anonimo
error-ldap-ca-cert = L'autorità di certificazione non è un certificato PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = L'indirizzo di un server della directory è vuoto
error-ldap-server-form = { $server }: l'indirizzo di un server della directory è host:port, ldap://… o ldaps://…
error-ldap-not-starttls = { $server }: questo è LDAPS, non StartTLS
error-ldap-no-certificate = non contiene alcun certificato
error-ldap-password-again = Inserisci di nuovo la password dell'account che cerca le persone: quella salvata viene inviata solo ai server per cui è stata data
error-ldap-unreachable = nessun server della directory è raggiungibile ({ $reasons })
error-ldap-no-anonymous = la directory non consente di cercare le persone in modo anonimo ({ $reason })
error-ldap-bind-failed = l'account che cerca le persone non è riuscito ad accedere ({ $reason })
error-ldap-search = ricerca di { $name } in { $base }
error-ldap-group-search = ricerca dei gruppi di { $name } in { $base }
error-ldap-several =
    { $count ->
        [one] { $count } voce della directory corrisponde
       *[other] { $count } voci della directory corrispondono
    } a { $name }: restringi il filtro utenti
error-ldap-password-check = la directory non ha voluto verificare la password di { $name } ({ $reason })
error-ldap-wrong-password = La directory non conosce { $name } con quella password
error-oauth-client-id = Indica il client ID che il provider ha creato per Hoard
error-oauth-addresses = Gli indirizzi di autorizzazione e del token devono iniziare con https://
error-oauth-userinfo-address = L'indirizzo delle informazioni utente deve iniziare con https://
error-oauth-username-claim = Indica quale claim contiene il nome della persona
error-oauth-secret-again = Inserisci di nuovo il client secret: quello salvato viene inviato solo all'indirizzo del token per cui è stato dato
error-oauth-issuer = L'issuer deve iniziare con https://
error-oauth-discovery-unreachable = Impossibile raggiungere { $url }: { $reason }
error-oauth-discovery-status = { $url } ha risposto { $status }
error-oauth-discovery-bad = { $url } non è un documento di discovery
error-oauth-discovery-incomplete = { $url } non indica gli indirizzi di autorizzazione e del token
error-signin-no-group = { $name } non è in un gruppo che può usare Hoard
error-signin-other-source = Esiste già un account Hoard chiamato { $name } che accede in un altro modo
error-signin-other-subject = Esiste già un account Hoard chiamato { $name } per un'altra persona presso il provider
error-signin-no-account = { $name } non ha ancora un account Hoard: un amministratore può aggiungerlo in Impostazioni → Account
error-signin-bad-name = Quel nome non può essere un account Hoard (massimo 64 caratteri)
error-oauth-off = L'accesso con OAuth non è attivato
error-oauth-bad-authorization-address = l'indirizzo di autorizzazione non è valido
error-oauth-refused = Il provider non ti ha fatto accedere: { $reason }
error-oauth-no-code = Il provider ti ha rimandato indietro senza un accesso
error-oauth-expired = L'accesso ha richiesto troppo tempo o è stato avviato altrove: riprova
error-oauth-no-claim = Il provider non ha detto chi sei (nessun claim { $claim })
error-oauth-no-sub = Il provider non ha detto chi sei (nessun claim sub o id)
error-oauth-token-unreachable = impossibile raggiungere l'indirizzo del token del provider
error-oauth-token-refused = Il provider non ha voluto completare l'accesso: { $reason }
error-oauth-no-access-token = il provider non ha inviato nessun token di accesso
error-oauth-userinfo-unreachable = impossibile raggiungere l'indirizzo delle informazioni utente del provider
error-oauth-userinfo-refused = Il provider non ha voluto dire chi ha effettuato l'accesso ({ $status })
error-oauth-userinfo-not-json = le informazioni utente del provider non sono JSON
error-format-no-name = name è vuoto
error-format-no-files = files è vuoto: indica quali file legge il formato, ad esempio ["*.json"]
error-format-empty-selector = vuoto
error-format-unopened-bracket = una parentesi chiusa senza quella aperta
error-format-unclosed-bracket = una parentesi non è chiusa
error-format-empty-step-dots = un passo vuoto (due punti di fila, o un punto a un'estremità)
error-format-unclosed-except = !( non è chiusa
error-format-unclosed-filter = [ non è chiusa
error-format-after-filters = { $rest } dopo la chiave e i filtri di { $step }
error-format-except-needs-glob = !(…) va dopo una chiave che contiene *
error-format-except-not-glob = !(…) va dopo una chiave che contiene *, non dopo { $key }
error-format-empty-step = un passo vuoto
error-format-not-a-filter = [{ $filter }] non è un filtro: usa [], [key=value|other], [array], [string], [number], [bool] o [object]
error-format-filter-needs-value = [{ $filter }] ha bisogno di una chiave e di un valore
error-store-search-empty = niente da cercare; scrivi il nome del pacchetto
error-update-hook-not-set = imposta prima un indirizzo da chiamare
error-upload-stalled = il caricamento di { $path } si è bloccato; invia di nuovo quella parte
error-upload-busy = { $path } è già in scrittura
error-upload-incomplete =
    { $count ->
        [one] { $count } file è incompleto
       *[other] { $count } file sono incompleti
    }, ad esempio { $files }
error-upload-no-files-listed = nessun file
error-upload-not-part = { $path } non fa parte del caricamento { $id }, oppure il caricamento è chiuso
error-bad-path = percorso non valido
error-upload-part-offset = { $offset } non è l'inizio di una parte di { $path }
error-upload-interrupted = caricamento interrotto: { $reason }
error-upload-part-long = la parte { $part } di { $path } è più lunga di { $len } byte
error-upload-part-short = parte { $part } di { $path }: ricevuti { $got } byte, attesi { $len }
error-upload-file-long = { $path } è più grande di quanto dichiarato
error-upload-file-short = { $path }: ricevuti { $got } byte, attesi { $len }
error-upload-complete = il caricamento è già completo
error-already-imported = già importato come «{ $name }»

## Testi del server

server-store-place-readme = README
server-store-place-folders = cartelle
server-store-why-linked = Linkato in { $place } del pacchetto
server-store-why-same-title = Stesso titolo di { $place } del pacchetto
server-store-why-by = Di { $author }, indicato in { $place } del pacchetto
server-store-why-meshes = Circa tante mesh quante il pacchetto ({ $count })
server-store-why-textures = Circa tante texture quante il pacchetto ({ $count })
server-chatgpt-revoke-failed = Disconnesso qui, ma non è stato possibile avvisare OpenAI. Per sicurezza, disconnetti Hoard in ChatGPT da Impostazioni → Sicurezza e accesso → Accedi con ChatGPT.
server-chatgpt-signed-in-title = Accesso a ChatGPT effettuato
server-chatgpt-signed-in = Hoard ha effettuato l'accesso. Ora i tag con IA possono usare il tuo piano ChatGPT.
server-chatgpt-signed-in-as = Hoard ha effettuato l'accesso come { $email }. Ora i tag con IA possono usare il tuo piano ChatGPT.
server-chatgpt-not-finished-title = Accesso non completato
server-chatgpt-other-computer = Se hai avviato l'accesso da Hoard su un altro computer, copia l'indirizzo di questa pagina dalla barra degli indirizzi e incollalo lì nelle Impostazioni di Hoard.
server-back-to-settings = Torna alle Impostazioni
server-issues-pack = Pacchetti che non è stato possibile aggiungere
server-issues-pack-blurb = Elimina il pacchetto e aggiungilo di nuovo, oppure aprilo per vedere cosa contiene.
server-issues-damaged = File salvati danneggiati o spariti
server-issues-damaged-blurb = Trovati da «Controlla i file per danni». Ripristinali da un backup o aggiungi di nuovo il loro pacchetto, poi esegui di nuovo «Controlla i file per danni».
server-issues-folder = Cartelle monitorate illeggibili
server-issues-folder-blurb = I loro pacchetti restano come sono finché la cartella non sarà di nuovo leggibile.
server-issues-missing = Pacchetti monitorati la cui cartella è sparita
server-issues-missing-blurb = Ogni pacchetto viene conservato per 3 giorni nel caso la cartella sia solo offline, poi viene rimosso.
server-issues-job = Processi in background non riusciti
server-issues-job-blurb = Riprovare esegue di nuovo il processo dall'inizio.
server-issues-file = File che non è stato possibile aprire
server-issues-file-blurb = Hoard non è riuscito ad aprirli o salvarli. Un pacchetto monitorato li riprova alla scansione successiva; un file già presente nella libreria conserva quello che aveva prima.
server-issues-unpack = Archivi che non è stato possibile estrarre
server-issues-unpack-blurb = Ognuno resta nel suo pacchetto come file semplice, quindi nulla di ciò che contiene è nella libreria.
server-issues-read = File che Hoard non è riuscito a interpretare
server-issues-read-blurb = Sono nella libreria, ma senza i dati, i tag o la miniatura che la loro lettura fornisce.
server-issues-render = Modelli su cui il generatore di miniature non è riuscito
server-issues-render-blurb = Non hanno una miniatura. Aprirne uno nel visualizzatore potrebbe comunque funzionare, e può salvarne una.
server-issues-convert = Modelli che Blender non è riuscito a convertire
server-issues-convert-blurb = Non hanno una copia GLB, quindi non possono essere mostrati in 3D.
server-issues-unreal = Package Unreal che non è stato possibile convertire
server-issues-unreal-blurb = La pagina Unreal del pacchetto ha il resto dei suoi package.
server-issues-materials = Pacchetti i cui materiali non è stato possibile ricostruire
server-issues-materials-blurb = Le loro texture ci sono tutte; semplicemente non sono raggruppate in materiali.
server-issues-material-thumb = Materiali senza miniatura
server-issues-material-thumb-blurb = Il materiale è stato trovato, ma non è stato possibile disegnarne l'anteprima.
server-issues-store-files = File degli store illeggibili
server-issues-store-files-blurb = File dello store da cui proviene un pacchetto, come un JSON di Megascans, che gli danno nome e tag.
server-issues-scan-again = Scansiona di nuovo
server-issues-retry = Riprova
server-issues-reanalyze = Rianalizza pacchetto
server-issues-convert-again = Converti di nuovo
server-issues-look-again = Cerca di nuovo
server-issues-read-again = Leggi di nuovo
server-issues-blob-missing = Il file salvato è sparito.
server-issues-blob-damaged = Il file salvato non corrisponde a ciò che era stato salvato: è danneggiato.
server-issues-folder-gone = La cartella «{ $folder }» non è stata trovata.
server-job-a-pack = un pacchetto
server-job-ingest = Importazione di { $pack }
server-job-retag = Nuovi tag per { $pack }
server-job-llm-tag = Tag con IA per { $pack }
server-job-convert = Conversione di { $pack } con Blender
server-job-unreal = Conversione dei package Unreal in { $pack }
server-job-scan = Scansione di una cartella monitorata
server-job-relocate = Spostamento dei file tra le posizioni della libreria
server-job-looks = Analisi di colori e aspetto
server-job-sounds = Ascolto dei suoni
server-job-embed = Riconoscimento del contenuto delle immagini
server-job-materials = Ricerca dei materiali in { $pack }
server-job-read-files = Lettura dei file di { $pack }
server-job-store-lookup = Ricerca di una pagina dello store
server-maintenance-backup = Esegui il backup del database
server-maintenance-backup-blurb = Una copia coerente del database (pacchetti, tag, note, raccolte e Impostazioni) nella cartella dei backup accanto ad esso. I file della libreria non ci sono: non cambiano mai una volta salvati, quindi qualsiasi sincronizzazione di file ne fa il backup.
server-maintenance-scrub = Controlla i file per danni
server-maintenance-scrub-blurb = Rilegge ogni file salvato e lo confronta con la sua impronta, per individuare file deteriorati o spariti da un disco o da un NAS. Su una libreria grande richiede un po' di tempo.
server-maintenance-reanalyze = Rianalizza tutti i pacchetti
server-maintenance-reanalyze-blurb = Rilegge ogni file con questa versione di Hoard: miniature, anteprime, dettagli e tag delle regole vengono rigenerati. Le tue modifiche restano; l'IA non viene interpellata di nuovo.
server-maintenance-clear-cache = Svuota le conversioni in cache
server-maintenance-clear-cache-blurb = Elimina i file conservati dalle conversioni «Scarica come» (vengono rifatti alla richiesta successiva) e ciò che è rimasto dei lavori interrotti.
server-maintenance-shrink-thumbs = Riduci le miniature
server-maintenance-shrink-thumbs-blurb = Salva le miniature e le viste dei modelli conservate come PNG nel modo indicato in Impostazioni → Prestazioni → Miniature, il che di solito le rende parecchie volte più piccole. Quelle con trasparenza, e la pixel art, restano PNG.
server-maintenance-gc = Elimina i file inutilizzati
server-maintenance-gc-blurb = Elimina i file salvati a cui non fa più riferimento nulla, come i file dei pacchetti eliminati definitivamente.
server-update-no-address = non è impostato nessun indirizzo da chiamare
server-update-no-answer = nessuna risposta entro 30 secondi

## Impostazioni: Aspetto

appearance-language = Lingua
appearance-language-auto = Del browser ({ $name })

## Radice

root-unreachable = Impossibile raggiungere il server di Hoard: { $error }
root-try-again = Riprova
root-lazy-connect = la connessione dell'app

## App: la libreria, la barra superiore e i filtri

app-lazy-settings = le impostazioni
app-lazy-archive = l'archivio
app-lazy-shortcuts = le scorciatoie
app-lazy-setup = la configurazione
app-theme-system = Segui il tema di sistema
app-theme-light = Tema chiaro
app-theme-dark = Tema scuro
app-color-red = Rosso
app-color-orange = Arancione
app-color-yellow = Giallo
app-color-green = Verde
app-color-teal = Verde acqua
app-color-blue = Blu
app-color-purple = Viola
app-color-pink = Rosa
app-color-brown = Marrone
app-color-sand = Sabbia
app-color-gray = Grigio
app-color-slate = Ardesia
app-color-black = Nero
app-color-white = Bianco
app-color-swatch = { $color }: asset con molto { $lower }
app-color-any = Qualsiasi colore
app-color-pick = Scegli un colore qualsiasi
app-color = Colore
app-retry = Riprova
app-undo = Annulla
app-search-failed = Ricerca non riuscita: { $error }
app-unreachable = Impossibile raggiungere il server: { $error }
app-forget-project-title = Dimenticare { $name }?
app-forget-project-body = Hoard smette di indicarlo dove vengono usati i suoi { $count } asset, finché il prossimo hoard sync non lo comunica di nuovo. Nel progetto non cambia nulla.
app-forget-project-confirm = Dimentica
app-forget-project-failed = Impossibile dimenticare { $name }: { $error }
app-forget-project-label = Dimentica il progetto { $name }
app-forget-project = Dimentica questo progetto
app-save-name-like = simili a { $name }
app-save-name-favorites = preferiti
app-search-saved = «{ $name }» salvata: si trova in Filtri, sotto I tuoi
app-rank-alike = Più simili
app-rank-color = Più del colore
app-rank-match = Più pertinenti
app-sort = Ordina
app-sort-newest = Più recenti
app-sort-name = Nome
app-sort-largest = Più grandi
app-sort-oldest = Meno recenti
app-collection-added =
    { $count ->
        [one] { $count } asset aggiunto
       *[other] { $count } asset aggiunti
    } a { $name }
app-collection-add-failed = Impossibile aggiungere alla raccolta: { $error }
app-collection-removed = { $count } rimossi da { $name }
app-collection-remove-failed = Impossibile rimuovere dalla raccolta: { $error }
app-ns-subject = Soggetto
app-ns-style = Stile
app-ns-theme = Tema
app-ns-material = Materiale
app-ns-map = Mappa PBR
app-ns-color = Colore
app-ns-mood = Atmosfera
app-ns-genre = Genere
app-ns-license = Licenza
app-ns-res = Risoluzione
app-ns-duration = Durata
app-ns-poly = Numero di poligoni
app-ns-format = Formato
app-ns-weight = Peso
app-ns-tech = Dati tecnici
app-ns-blender = Versione di Blender
app-ns-depth = Profondità di bit
app-ns-ucs = Categoria sonora (UCS)
app-ns-bpm = Tempo
app-ns-root = Nota fondamentale
app-ns-level = Intensità sonora
app-ns-ue = Classe Unreal
app-ns-engine = Motore
app-ns-extracted = Sorgente recuperata
app-ns-script = Sistema di scrittura
app-bulk-failed =
    Impossibile modificare { $count ->
        [one] quell'asset
       *[other] quei { $count } asset
    }: { $error }
app-bulk-untagged =
    Tag { $tag } tolto da { $count ->
        [one] { $count } asset
       *[other] { $count } asset
    }
app-bulk-tagged =
    Tag { $tag } aggiunto a { $count ->
        [one] { $count } asset
       *[other] { $count } asset
    }
app-bulk-category-set =
    Categoria di { $count ->
        [one] { $count } asset impostata
       *[other] { $count } asset impostata
    } su { $category }
app-bulk-category-cleared =
    Categoria di { $count ->
        [one] { $count } asset cancellata
       *[other] { $count } asset cancellata
    }
app-credits-failed = Impossibile creare i crediti: { $error }
app-archived-assets =
    { $count ->
        [one] { $count } asset archiviato
       *[other] { $count } asset archiviati
    }
app-archived-one = { $name } archiviato
app-open-failed = Impossibile aprire quell'asset: { $error }
app-unnamed-pack = pacchetto
app-unnamed-collection = raccolta
app-unnamed-asset = asset
app-home-version = Hoard { $version } — torna alla libreria
app-home = Torna alla libreria
app-search-label = Cerca asset
app-search-placeholder = Cerca nomi, percorsi, tag
app-views = Viste
app-library = Libreria
app-packs = Pacchetti
app-packs-working = { $count } in corso
app-update-title = È uscito Hoard { $version } (questo server esegue la { $current }). Scarica la nuova immagine o il nuovo binario e riavvia; il database si aggiorna da solo.
app-update = Aggiorna
app-theme = Tema
app-sign-in = Accedi
app-archive-title = Archivio: { $contents }
app-archive = Archivio
app-settings = Impostazioni
app-palette-title = Tavolozza dei comandi ({ $keys })
app-import-folder = Importa cartella
app-add-files = Aggiungi file
app-add-files-short = Aggiungi
app-split-label = Come importare questa cartella
app-split-before = { "" }
app-split-after = { " " }contiene { $count } cartelle o archivi.
app-split-each = Importa ciascuno come pacchetto a sé ({ $count })
app-split-one = Importa come un unico pacchetto
app-cancel = Annulla
app-filters = Filtri
app-done = Fatto
app-yours = I tuoi
app-favorites = Preferiti
app-forget-search-label = Dimentica la ricerca salvata { $name }
app-forget-search = Dimentica questa ricerca
app-projects-title = Progetti che eseguono hoard sync da qui, con ciò che conteneva la loro ultima sincronizzazione
app-projects = Progetti
app-project-synced = Sincronizzato { $when }
app-project-synced-by = Sincronizzato { $when } da { $who }
app-collections = Raccolte
app-no-filters = Nessun filtro per questa ricerca.
app-kind = Tipo
app-facets-more = Altri { $count }
app-facets-fewer = meno
app-searching = Ricerca in corso…
app-searching-live = Ricerca in corso
app-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } asset
    }
app-clear-all = Cancella tutto
app-save-name-label = Nome per questa ricerca
app-save-name-placeholder = Dagli un nome
app-save = Salva
app-save-search-title = Conserva questa ricerca in Filtri, sotto I tuoi
app-save-search = Salva ricerca
app-select-title = Scegli più asset per assegnare tag, categorizzarli, raccoglierli, scaricarli o archiviarli insieme (X, o Ctrl+clic)
app-select = Seleziona
app-tile-size = Dimensione riquadri
app-supporting-title = Buffer glTF, librerie di materiali, le mappe dei materiali, i materiali usati da un solo modello e i file degli store che descrivono i modelli. Vengono inclusi quando scarichi ciò che li usa.
app-supporting-hidden =
    { $count ->
        [one] { $count } file di supporto nascosto
       *[other] { $count } file di supporto nascosti
    }
app-supporting-show = Mostra
app-supporting-shown = File di supporto inclusi
app-supporting-hide = Nascondi
app-welcome-title = Trascina un pacchetto di asset in un punto qualsiasi di questa pagina
app-welcome-before = Vanno bene zip, tarball, cartelle o file sparsi. Hoard li estrae, salva ogni file una sola volta in base al suo hash, legge dimensioni, durate e numero di poligoni, e assegna tag a tutto così puoi cercare cose come{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }o{ " " }
app-welcome-after = .
app-welcome-import-folder = Importa una cartella
app-welcome-choose-files = Scegli file
app-none-similar = Ancora niente somiglia molto a { $name }.
app-none = Nessun risultato.
app-none-query = Nessun risultato per «{ $query }».
app-none-filters = Nessun risultato con questi filtri.
app-none-query-filters = Nessun risultato per «{ $query }» con questi filtri.
app-clear-search-filters = Cancella ricerca e filtri
app-loading-more = Caricamento di altri risultati
app-keys-hint = Le frecce si spostano, Invio apre, Spazio riproduce un suono, X o Ctrl+clic seleziona più elementi (pressione prolungata sul touch).
app-resize-label = Ridimensiona il pannello dei dettagli
app-resize-title = Trascina per ridimensionare · doppio clic per la larghezza adatta
app-selected-label = Asset selezionati
app-selected = { $count } selezionati
app-pick-assets = Scegli asset
app-download-zip = Scarica zip
app-credits-title = CREDITS.md per i pacchetti da cui provengono: chi li ha creati, le loro licenze e le righe di credito che richiedono
app-credits = Crediti
app-prefer-sources-title = SoundWave e texture Unreal di cui Hoard ha recuperato il file originale
app-prefer-sources = WAV/PNG recuperati invece di .uasset ({ $count })
app-bulk-tag-label = Un tag da aggiungere agli asset selezionati o da togliere
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Aggiungi questo tag a ogni asset selezionato
app-bulk-tag-add = Aggiungi
app-bulk-tag-remove-title = Togli questo tag da ogni asset selezionato
app-bulk-tag-remove = Rimuovi
app-bulk-category-label = Categoria per gli asset selezionati (vuota la cancella)
app-bulk-category-placeholder = Categoria
app-bulk-category-set-title = Imposta la categoria di ogni asset selezionato
app-bulk-category-clear-title = Cancella la categoria di ogni asset selezionato
app-bulk-category-set-button = Imposta
app-bulk-category-clear-button = Cancella
app-remove-from-collection = Rimuovi da { $name }
app-unfavorite = Togli dai preferiti
app-favorite = Aggiungi ai preferiti
app-select-all = Seleziona tutti e { $count }
app-archive-selected-title = Esclusi dalla ricerca finché non vengono ripristinati o eliminati definitivamente, dall'Archivio
app-drop-items = Rilascia { $count } elementi per aggiungerli
app-drop = Rilascia per aggiungere a Hoard
app-drop-hint = Ogni file o cartella di primo livello diventa un pacchetto a sé. I gigabyte vanno bene; i file grandi riprendono se la connessione cade.

## Tavolozza dei comandi (CommandPalette.vue, con i comandi da App.vue)

palette-group-go-to = Vai a
palette-group-actions = Azioni
palette-group-theme = Tema
palette-group-kinds = Tipi
palette-group-packs = Pacchetti
palette-group-collections = Raccolte
palette-group-projects = Progetti
palette-group-saved = Ricerche salvate
palette-group-assets = Asset
palette-hint-current = attuale
palette-library = Libreria
palette-library-keywords = libreria asset griglia home inizio grid
palette-favorites = Preferiti
palette-favorites-keywords = preferiti stella stelle mi piace starred stars liked
palette-packs = Pacchetti
palette-archive = Archivio
palette-archive-keywords = cestino eliminati ripristina ripristinare trash recycle bin deleted restore
palette-stats = Statistiche della libreria
palette-stats-keywords = statistiche numeri dimensione disco spazio conteggio statistics numbers size disk space count
palette-search = Cerca
palette-add-files = Aggiungi file…
palette-add-files-keywords = carica caricamento importa upload import
palette-import-folder = Importa una cartella…
palette-import-folder-keywords = carica pacchetto upload pack
palette-select-all = Seleziona tutti i { $count } mostrati
palette-select-all-keywords = spunta seleziona check
palette-select-several = Seleziona più elementi…
palette-select-several-keywords = multipla multipli gruppo tag archivia bulk multiple archive
palette-archive-selected = Archivia i { $count } selezionati
palette-archive-selected-keywords = elimina rimuovi cestino delete remove trash
palette-clear-selection = Annulla la selezione
palette-clear-filters = Cancella i filtri
palette-clear-filters-keywords = azzera reimposta reset
palette-hide-supporting = Nascondi i file di supporto
palette-show-supporting = Mostra i file di supporto
palette-supporting-keywords = mappe normal rugosità maschere normal maps roughness masks
palette-account = Account e token di accesso
palette-account-keywords = password accesso login utente token api
palette-sign-out = Esci ({ $name })
palette-sign-out-keywords = esci disconnetti uscita log out logout
palette-sign-in = Accedi
palette-sign-in-keywords = accedi accesso entra account password log in login
palette-settings = Impostazioni
palette-settings-keywords = impostazioni preferenze strumenti amministratore ia tag blender ffmpeg llm ai tagging unreal admin tools preferences
palette-shortcuts = Scorciatoie da tastiera
palette-shortcuts-keywords = aiuto tasti tastiera help keys
palette-theme-keywords = tema aspetto modalità theme appearance mode
palette-theme-variant-keywords = aspetto colore appearance color
palette-light-theme = Tema chiaro: { $name }
palette-dark-theme = Tema scuro: { $name }
palette-kind-all = Tutti i tipi (smetti di mostrare solo { $kind })
palette-kind-only = Solo { $kind }
palette-title = Tavolozza dei comandi
palette-placeholder = Cerca asset, vai a un pacchetto, esegui un comando…
palette-results = Risultati
palette-searching = Ricerca in corso…
palette-no-match = Nessun risultato per «{ $query }»
palette-foot-move = sposta
palette-foot-run = esegui
palette-foot-shortcuts = tutte le scorciatoie

## Scorciatoie da tastiera (ShortcutsDialog.vue)

shortcuts-title = Scorciatoie da tastiera
shortcuts-close = Chiudi
shortcuts-anywhere = Ovunque
shortcuts-palette = Tavolozza dei comandi: cerca, salta, esegui
shortcuts-search = Cerca
shortcuts-this-list = Questo elenco
shortcuts-escape = Chiudi, annulla la selezione o chiudi i dettagli
shortcuts-library = Libreria
shortcuts-move = Spostati tra i riquadri (o J e K)
shortcuts-open = Apri i dettagli
shortcuts-play-sound = Riproduci o ferma un suono
shortcuts-select = Seleziona o deseleziona
shortcuts-favorite = Aggiungi ai preferiti o togli dai preferiti
shortcuts-key-click = clic
shortcuts-select-range = Seleziona un intervallo
shortcuts-select-all = Seleziona tutto ciò che è mostrato
shortcuts-sound-player = Lettore audio
shortcuts-play-pause = Riproduci o metti in pausa
shortcuts-skip = Vai indietro o avanti
shortcuts-start-end = Inizio o fine
shortcuts-image-inspector = Ispettore immagini
shortcuts-zoom = Ingrandisci o riduci
shortcuts-fit = Adatta alla finestra
shortcuts-actual-pixels = Pixel reali
shortcuts-tile = Affianca 3×3 per controllare le giunture
shortcuts-background = Sfondo successivo
shortcuts-channel = Mostra un solo canale

## KindChips.vue

kinds-group = Tipi di asset

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = aggiunto da una persona
tag-source-rule = dal tagger offline
tag-source-meta = misurato dal file
tag-source-ai = suggerito dall'IA
tag-remove = Rimuovi il tag { $tag }

## ActivityIndicator.vue

activity-label-idle = Attività in background: inattiva
activity-label-running = Attività in background: { $running } in corso
activity-label-queued = Attività in background: { $running } in corso, { $queued } in coda
activity-title = Attività in background
activity-unreachable = Impossibile raggiungere il server
activity-empty = Niente in corso. Importazioni, assegnazione dei tag, conversioni e download di zip compaiono qui mentre sono in corso.
activity-queued = in coda
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } tri
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Riproduci { $name }
tile-pause = Metti in pausa { $name }
tile-favorite = Aggiungi { $name } ai preferiti
tile-in-favorites = Nei tuoi preferiti
tile-add-favorite = Aggiungi ai preferiti (F)
tile-more-like = Altri simili a { $name }
tile-more-like-this = Altri simili
tile-select = Seleziona { $name }

## CollectionPicker.vue

collections-add = Aggiungi alla raccolta
collections-menu = Raccolte
collections-find-label = Trova o crea una raccolta
collections-find-placeholder = Trova o dai un nome a una nuova raccolta
collections-create = Crea
collections-empty = Ancora nessuna raccolta. Scrivi un nome per crearne una.

## FolderPicker.vue

folders-list = Cartelle
folders-close = Chiudi
folders-browse = Sfoglia…
folders-up-title = Su di una cartella
folders-up = Su
folders-this-computer = Questo computer
folders-none = Nessuna cartella qui.

## LocationForm.vue

locations-mode-question = Cosa deve fare Hoard con questa cartella?
locations-mode-managed = Tieni qui la mia libreria
locations-mode-managed-hint = I pacchetti che carichi o importi vengono copiati in questa cartella. Se ne occupa Hoard, quindi non toccarne il contenuto.
locations-mode-watched = Indicizza una cartella che ho già
locations-mode-watched-hint = Ogni cartella al suo interno diventa un pacchetto. Hoard legge i file dove si trovano e non li modifica, sposta o elimina mai, e rileva le nuove cartelle man mano che compaiono.
locations-folder = Cartella sul server
locations-folder-placeholder = La cartella in cui si trovano i tuoi pacchetti
locations-folder-hint-managed = Hoard la crea se non esiste. In Docker, è una cartella che hai montato nel container.
locations-folder-hint-watched = In Docker, monta prima la cartella nel container (va bene in sola lettura), poi sceglila qui.
locations-name = Nome
locations-name-placeholder-managed = Libreria
locations-name-placeholder-watched = I miei pacchetti
locations-files = File che vanno qui
locations-catch-all = Anche tutto ciò che nessun'altra posizione accetta
locations-kinds-hint = I file di un nuovo pacchetto vanno nella posizione che ha spuntato il loro tipo. Il resto va nella posizione predefinita.
locations-files-all = Tutti, per ora. Aggiungi altre posizioni più avanti dalle Impostazioni per mandare alcuni tipi (la musica, ad esempio) su un altro disco.
locations-first-watched-note = Per caricare serve una libreria gestita da Hoard. Puoi aggiungerne una più avanti da Impostazioni → Posizioni della libreria.
locations-checking = Controllo della cartella…
locations-add = Aggiungi posizione
locations-cancel = Annulla

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Impossibile caricare le posizioni della libreria
locations-takes-and-rest = { $kinds } e tutto il resto
locations-takes-rest = Tutto ciò che le altre non accettano
locations-takes-everything = Tutto
locations-takes-nothing = Ancora niente
locations-save-failed = Impossibile salvare la posizione
locations-unwatch-title = Smettere di monitorare { $name }?
locations-unwatch-body =
    { $count ->
        [one] Il suo { $count } pacchetto esce
       *[other] I suoi { $count } pacchetti escono
    } da Hoard, con i relativi tag e note. I file nella cartella non vengono toccati.
locations-unwatch-body-empty = I file nella cartella non vengono toccati.
locations-unwatch = Smetti di monitorare
locations-remove-title = Rimuovere { $name }?
locations-remove-body = Gli eventuali file che contiene vengono prima spostati nelle altre posizioni, in base al tipo; viene rimossa una volta vuota. Non si perde nulla.
locations-remove-body-only = Se contiene file, aggiungi prima un'altra posizione in cui spostarli.
locations-remove = Rimuovi
locations-removed = { $name } rimossa
locations-emptying = Spostamento dei file fuori da { $name }; verrà rimossa al termine
locations-offline-title = { $name } non è raggiungibile
locations-offline-body = I suoi file non possono essere spostati mentre è offline. Rimuoverla comunque? I file ancora presenti risulteranno mancanti.
locations-remove-anyway = Rimuovi comunque
locations-remove-failed = Impossibile rimuovere { $name }
locations-rebalancing = Spostamento dei file salvati nelle posizioni che accettano il loro tipo
locations-rebalance-failed = Impossibile spostare i file
locations-scanned-never = non ancora scansionata
locations-scanned-now = scansionata proprio ora
locations-scanned-minutes = scansionata { $mins } min fa
locations-scanned-hours = scansionata { $hours } h fa
locations-scanned-date = scansionata il { $date }
locations-scan-failed = Impossibile scansionare { $name }
locations-watching = Monitoraggio di { $name } attivo
locations-added = { $name } aggiunta
locations-loading = Caricamento…
locations-managed-title = Librerie gestite da Hoard
locations-watched-title = Cartelle monitorate
locations-managed-hint = Caricamenti e importazioni vengono copiati qui. Ogni posizione accetta i tipi che ha spuntato; quella predefinita accetta il resto.
locations-watched-hint = Cartelle di pacchetti indicizzate dove si trovano. Hoard non modifica mai i loro file, e cerca cartelle nuove e rimosse ogni 15 minuti.
locations-managed-empty = Ancora nessuna, quindi non c'è dove mettere un caricamento. Aggiungine una qui sotto.
locations-reachable = Raggiungibile
locations-unreachable-now = Non raggiungibile in questo momento
locations-catch-all-tag = predefinita
locations-packs =
    { $count ->
        [one] { $count } pacchetto
       *[other] { $count } pacchetti
    }
locations-unreachable = non raggiungibile
locations-scanning = Scansione…
locations-scan = Scansiona ora
locations-edit = Modifica
locations-catch-all-locked = Per cambiarlo, imposta un'altra posizione come predefinita
locations-edit-hint-before = Vale per i file aggiunti d'ora in poi. Per spostare ciò che è già salvato, usa
locations-edit-hint-after = qui sotto.
locations-save = Salva
locations-rebalance = Sposta i file salvati di conseguenza
locations-rebalance-hint = Dopo aver cambiato ciò che una posizione accetta: i file già salvati vengono spostati nella posizione a cui ora va il loro tipo.
locations-add-title = Aggiungi una posizione
locations-add-hint = Un altro disco per alcuni tipi di file, o una cartella di pacchetti che hai già.
locations-add-more = Aggiungi una posizione…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Impossibile caricare i Problemi
issues-started =
    { $count ->
        [one] Riavviato.
       *[other] { $count } elementi riavviati.
    }
issues-retry-failed = Impossibile riprovare
issues-dismiss-failed = Impossibile ignorare
issues-dismiss-all-title = Ignorare tutti e { $count }?
issues-dismiss-all-body = «{ $title }» viene svuotato. Tutto ciò che andrà di nuovo storto verrà elencato di nuovo.
issues-dismiss-all = Ignora tutti
issues-just-now = proprio ora
issues-minutes-ago = { $mins } min fa
issues-hours-ago = { $hours } h fa
issues-loading = Caricamento…
issues-none = Non è andato storto niente.
issues-action-all = { $action } tutti
issues-open-asset = Aprilo
issues-open-pack = Mostra il pacchetto
issues-open-locations = Posizioni della libreria
issues-where-before = in
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } volta
       *[other] { $count } volte
    }
issues-starting = Avvio…
issues-dismiss-named = Ignora { $title }
issues-dismiss = Ignora
issues-show-fewer = Mostra meno
issues-show-all = Mostra tutti e { $count }
issues-not-listed = Altri { $count } non sono elencati.

## archive.ts

archive-anyway = Archivia comunque
archive-failed = Impossibile archiviare: { $error }
archive-undo = Annulla
archive-restore-failed = Impossibile ripristinare: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pacchetto
       *[other] { $packs } pacchetti
    } e { $assets ->
        [one] { $assets } asset
       *[other] { $assets } asset
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pacchetto
       *[other] { $packs } pacchetti
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } asset
       *[other] { $assets } asset
    }
archive-count-nothing = niente

## ArchiveView.vue

archive-load-failed = Impossibile caricare l'archivio: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Hai ripristinato { $what }
archive-purged = Hai eliminato definitivamente { $what }
archive-purge-failed = Impossibile eliminare: { $error }
archive-purge-title = Eliminare definitivamente { $name }?
archive-purge-pack-body =
    { $count ->
        [one] Verranno eliminati anche il suo { $count } asset
       *[other] Verranno eliminati anche i suoi { $count } asset
    } e { $size } di file, tranne i file condivisi con un altro pacchetto. L'operazione non può essere annullata.
archive-purge = Elimina definitivamente
archive-purge-asset-body = Il suo file viene liberato, a meno che un altro asset non lo condivida. L'operazione non può essere annullata.
archive-empty-title = Svuotare l'archivio?
archive-empty-body = Hoard eliminerà definitivamente { $what } e ne libererà i file. L'operazione non può essere annullata.
archive-empty = Svuota archivio
archive-title = Archivio
archive-intro = Pacchetti e asset archiviati sono esclusi dalla ricerca, dai conteggi e dall'elenco dei tag. Ripristinali, oppure eliminali definitivamente per liberarne lo spazio.
archive-loading = Caricamento…
archive-is-empty = L'archivio è vuoto
archive-is-empty-hint = Archivia un pacchetto o un asset e resterà qui finché non lo ripristini o lo elimini definitivamente.
archive-packs = Pacchetti
archive-assets = Asset
archive-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } asset
    }
archive-archived = archiviato { $when }
archive-from = da { $pack }
archive-restore = Ripristina
archive-show-more = Mostra altri ({ $count } rimanenti)

## mine.ts

projects-favorite-add-failed = Impossibile aggiungere ai preferiti: { $error }
projects-favorite-remove-failed = Impossibile togliere dai preferiti: { $error }
projects-save-search-failed = Impossibile salvare la ricerca: { $error }
projects-forget-search-failed = Impossibile rimuovere { $name }: { $error }

## projects.ts

projects-list-and = { $items } e { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = altri { $count }
projects-other =
    { $count ->
        [one] { $count } altro progetto
       *[other] altri { $count } progetti
    }
projects-usage-title-one = In uso in { $name }
projects-usage-title = In uso in { $count } progetti
projects-usage-body-one = In uso in { $list }: il suo prossimo hoard sync li toglierà.
projects-usage-body = In uso in { $list }: il prossimo hoard sync di ciascuno li toglierà.

## uploader.ts

upload-network-error = errore di rete
upload-gave-up = { $what }: { $error } dopo { $attempts } tentativi
upload-failed-plain = non riuscito
upload-already-imported = già importato
upload-part-failed = caricamento non riuscito ({ $status })
upload-batch = gruppo di { $count } file
upload-batch-failed = gruppo non riuscito ({ $status })
upload-finish = completamento del caricamento

## UploadQueue.vue

upload-eta-seconds = { $s } s rimanenti
upload-eta-minutes = { $min } min rimanenti
upload-eta-hours = { $h } h rimanenti
upload-queue = Importazioni
upload-importing = Importazione di { $count } pacchetti
upload-bytes-of = { $sent } di { $total }
upload-rate = { $size }/s
upload-waiting = In attesa
upload-unpacking = Caricato, ora in estrazione
upload-already = Già nella libreria
upload-stopped = Interrotto
upload-files-of = { $done } di { $total } file
upload-resumed = ripreso, { $size } erano già presenti
upload-skipped = Questi stessi file erano già stati importati come «{ $name }», quindi sono stati saltati.
upload-error = { $error }. I tuoi progressi sono conservati: riprova, o trascina di nuovo gli stessi file più tardi.
upload-retry = Riprova
upload-import-again = Importa di nuovo comunque
upload-dismiss = Ignora
upload-cancel = Annulla

## PacksView.vue

packs-rename-failed = Impossibile rinominare: { $error }
packs-collection-deleted = Raccolta { $name } eliminata
packs-delete-failed = Impossibile eliminare: { $error }
packs-status-uploading = Caricamento (file ricevuti finora)
packs-status-ingesting = Estrazione e assegnazione dei tag
packs-status-analyzing = L'LLM sta assegnando i tag
packs-status-ready = Pronto
packs-status-failed = Non riuscito
packs-retag-failed = Impossibile avviare la nuova assegnazione dei tag: { $error }
packs-read-files-failed = Impossibile leggere i file del pacchetto: { $error }
packs-context-load-failed = Impossibile caricare i dettagli dello store del pacchetto: { $error }
packs-undo-store-failed = Impossibile togliere la pagina dello store: { $error }
packs-context-save-failed = Impossibile salvare i dettagli dello store: { $error }
packs-unreal-load-failed = Impossibile caricare lo stato della conversione Unreal: { $error }
packs-unreal-discard-title = Scartare gli originali?
packs-unreal-discard-body = I file .uasset originali in { $name } vengono eliminati una volta convertiti. I dati che solo Unreal usa, come i grafi dei materiali, andranno persi definitivamente.
packs-unreal-discard-confirm = Converti e scarta
packs-unreal-convert-failed = Impossibile avviare la conversione: { $error }
packs-unreal-fact-converted = { $done } convertiti da Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } package Unreal
       *[other] { $left } package Unreal
    }
packs-unreal-fact-both = { $done } convertiti da Unreal, { $left } ancora package
packs-thumbs-canceled = Annullato dopo { $done } di { $total }
packs-thumbs-drew =
    { $count ->
        [one] { $count } modello disegnato
       *[other] { $count } modelli disegnati
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniatura sostituita.
       *[other] { $lead }: { $improved } miniature sostituite.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniatura sostituita, { $failed } non disegnabili.
       *[other] { $lead }: { $improved } miniature sostituite, { $failed } non disegnabili.
    }
packs-archived = { $name } e i suoi { $count } asset archiviati
packs-blobs-bad = { $count } file non hanno superato l'ultimo controllo di integrità.
packs-collections = Raccolte
packs-browse-named = Sfoglia { $name }
packs-collection-name = Nome della raccolta
packs-save = Salva
packs-cancel = Annulla
packs-assets =
    { $count ->
        [one] { $count } asset
       *[other] { $count } asset
    }
packs-updated = Aggiornata il { $date }
packs-browse = Sfoglia
packs-collection-download = Scarica (.zip)
packs-collection-credits = Crediti
packs-collection-credits-title = Chi ha creato i pacchetti da cui provengono questi asset, le loro licenze e le righe di credito che richiedono
packs-rename = Rinomina
packs-delete = Elimina
packs-packs = Pacchetti
packs-empty = Ancora nessun pacchetto. Trascina uno zip o una cartella in un punto qualsiasi di questa pagina per aggiungerne uno.
packs-find = Trova un pacchetto
packs-find-placeholder = Trova un pacchetto per nome, store, licenza o tag
packs-sort = Ordine
packs-sort-new = Prima i più recenti
packs-sort-old = Prima i meno recenti
packs-sort-name = Per nome
packs-sort-size = Prima i più grandi
packs-sort-assets = Prima quelli con più asset
packs-with-kind = Pacchetti con
packs-state = Stato
packs-state-any = Qualsiasi stato
packs-state-working = In lavorazione
packs-state-failed = Non riusciti
packs-state-untagged = Non ancora descritti
packs-store = Store
packs-store-any = Qualsiasi store
packs-store-from = Da { $store }
packs-store-none = Nessuna pagina dello store
packs-shown-of = { $shown } di { $total }
packs-clear = Cancella
packs-no-match = Nessun pacchetto corrisponde.
packs-clear-all = Cancella la ricerca e i filtri
packs-pack-name = Nome del pacchetto
packs-thumbs-drawing = Disegno delle miniature in questo browser…
packs-thumbs-drawing-count = Disegno delle miniature in questo browser: { $done } di { $total }…
packs-thumbs-stopped = Interrotto: { $error }
packs-thumbs-none = Ogni modello qui ha già una miniatura disegnata con tutte le sue texture.
packs-dismiss = Ignora
packs-license = Licenza: { $license }
packs-license-tier = Licenza: { $license } ({ $tier })
packs-license-not-found = non trovata
packs-from-before = Da
packs-from-after = { "" }
packs-added = Aggiunto il { $date }
packs-download = Scarica pacchetto (.zip)
packs-store-details = Dettagli dello store
packs-more-named = Altre azioni per { $name }
packs-more = Altro
packs-rename-more = Rinomina…
packs-retag-rules = Riesegui il tagger offline
packs-retag-llm = Riesegui il tagger LLM
packs-make-thumbs = Crea miniature in questo browser
packs-make-thumbs-title = Apre ogni modello in un visualizzatore 3D nascosto, uno alla volta, e usa la sua immagine come miniatura quando è migliore di quella del server
packs-read-files = Rileggi i file del pacchetto
packs-credits = Scarica i crediti
packs-credits-title = CREDITS.md: chi ha creato il pacchetto, la sua licenza e la riga di credito che richiede
packs-unreal-open = Converti contenuti Unreal…
packs-archive = Archivia pacchetto
packs-unreal-intro = Hoard legge da sé i package Unreal e salva il loro contenuto come file normali: texture come PNG o EXR, suoni come WAV, materiali come JSON. Il file convertito prende il posto del package, mantenendo tag, collegamenti e raccolte.
packs-unreal-converted =
    { $count ->
        [one] convertito
       *[other] convertiti
    }
packs-unreal-converted-kept =
    { $count ->
        [one] convertito, { $kept } con l'originale conservato
       *[other] convertiti, { $kept } con l'originale conservato
    }
packs-unreal-pending =
    { $count ->
        [one] non ancora esaminato
       *[other] non ancora esaminati
    }
packs-unreal-needs-editor =
    { $count ->
        [one] in attesa di Unreal Editor
       *[other] in attesa di Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] senza un formato comune
       *[other] senza un formato comune
    }
packs-unreal-failed =
    { $count ->
        [one] non riuscito
       *[other] non riusciti
    }
packs-unreal-reasons = Perché restano dei package
packs-unreal-editor-before = Mesh e animazioni richiedono Unreal Editor. Su una macchina con Unreal installato, esegui
packs-unreal-editor-command = hoard unreal-worker --editor <percorso di UnrealEditor-Cmd>
packs-unreal-editor-after = ; li esporta come FBX e carica qui i risultati.
packs-unreal-discard = Scarta i file .uasset originali una volta convertiti
packs-unreal-discard-hint = Libera spazio. I dati che solo Unreal usa, come i grafi dei materiali, vanno persi.
packs-unreal-drop = Elimina i package senza un formato comune
packs-unreal-drop-hint = Blueprint, livelli e altri asset che solo il motore usa.
packs-unreal-convert = Converti
packs-context-hint-before = Da dove viene il pacchetto e cosa ne dice il venditore. Il tagger LLM lo legge insieme a ogni file, quindi una descrizione dello store come «120 incantesimi di ghiaccio e fuoco» lo aiuta a dare un nome a
packs-context-hint-after = .
packs-store-auto = Trovata automaticamente: è il pacchetto stesso a indicare questa pagina.
packs-store-not-this = Non è questa
packs-lookup-label = Cerca negli store
packs-lookup-placeholder = Il nome del pacchetto nello store
packs-lookup-searching = Ricerca in corso…
packs-lookup-search = Cerca negli store
packs-lookup-hide = Nascondi
packs-lookup-clues = Dai file del pacchetto: { $clues }.
packs-lookup-also = Cercato anche { $queries }.
packs-lookup-quoted = «{ $query }»
packs-lookup-nothing = Niente su Fab, itch.io, Kenney o OpenGameArt gli somiglia. Prova con meno parole o parole diverse.
packs-lookup-use = Usa
packs-lookup-searched = Cercato il { $date }.
packs-lookup-again = Cerca di nuovo
packs-lookup-again-after = .
packs-lookup-unreachable = Questa volta non è stato possibile raggiungere { $stores }.
packs-lookup-also-look = Cerca anche su
packs-lookup-links-and = { " " }e{ " " }
packs-lookup-browser-only = (si possono cercare solo nel tuo browser); incolla qui sotto l'indirizzo della pagina.
packs-lookup-open = Trova negli store…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Venditore
packs-seller-placeholder = Chi l'ha creato
packs-license-label = Licenza
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Livello di licenza
packs-license-tier-placeholder = Personale, Professionale…
packs-store-page = Pagina dello store
packs-store-text = Descrizione dello store
packs-store-text-placeholder = Incolla il testo dell'inserzione, o lascia vuoto per recuperarlo dalla pagina dello store.
packs-store-text-fetch = Salvando, la descrizione viene recuperata dalla pagina dello store.
packs-readme = README trovato nel pacchetto
packs-save-retag = Salva e riesegui il tagger LLM

## AssetDetail.vue

asset-lazy-3d-viewer = il visualizzatore 3D
asset-lazy-text = il testo
asset-lazy-text-viewer = il visualizzatore di testo
asset-lazy-image-viewer = il visualizzatore di immagini
asset-lazy-material-preview = l'anteprima del materiale
asset-lazy-material-viewer = il visualizzatore di materiali
asset-lazy-sprite-player = il lettore di sprite
asset-rel-texture = texture
asset-rel-buffer = dati geometrici
asset-rel-material = file del materiale
asset-rel-animation = animazione
asset-rel-library = file collegato
asset-rel-guess = probabilmente va con
asset-rel-sidecar = dettagli dello store
asset-rel-variant = variante
asset-role-map = mappa { $role }
asset-maps-count =
    { $count ->
        [one] { $count } mappa
       *[other] { $count } mappe
    }
asset-needs = Richiede
asset-animations = Animazioni
asset-used-by = Usato da
asset-conversion-failed = conversione non riuscita
asset-as-converting = Conversione…
asset-as-converting-percent = Conversione… { $percent }%
asset-as-queued = In coda…
asset-download = Scarica
asset-as-label = Scarica come
asset-as-original = { $format } (formato originale)
asset-as-size = Dimensione
asset-as-full-size = Dimensione piena
asset-as-up-to = Fino a { $size } px
asset-as-hint = Scegli una dimensione o un altro formato
asset-meta-size = Dimensione
asset-meta-length = Durata
asset-meta-sample-rate = Frequenza di campionamento
asset-meta-channels = Canali
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Profondità di bit
asset-meta-bits = { $bits } bit
asset-meta-level = Livello
asset-meta-level-value = picco { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = picco { $peak } dBFS, RMS { $rms } dBFS, con clipping
asset-meta-loop = Loop
asset-meta-loop-value = da { $start } a { $end }
asset-meta-playback = Riproduzione
asset-meta-one-shot = One-shot
asset-meta-tempo = Tempo
asset-meta-root-note = Nota fondamentale
asset-meta-cue-markers = Marcatori cue
asset-meta-library = Libreria
asset-meta-keywords = Parole chiave
asset-meta-title = Titolo
asset-meta-artist = Artista
asset-meta-album = Album
asset-meta-genre = Genere
asset-meta-generator = Generatore
asset-meta-everything-with = Mostra tutto ciò che ha il tag { $tag }
asset-meta-saved-with = Salvato con
asset-meta-color-mode = Modalità colore
asset-meta-layers = Livelli
asset-meta-range = Gamma
asset-meta-hdr = HDR (l'anteprima usa il tone mapping)
asset-meta-format = Formato
asset-meta-large-document = Documento grande (PSB)
asset-meta-ai = IA
asset-meta-ai-made = Creato con IA generativa
asset-meta-ai-edited = Modificato con IA generativa
asset-meta-ai-how-metadata = metadati
asset-meta-ai-how-generator = impostazioni del generatore
asset-meta-ai-how-iptc = tipo di origine digitale IPTC
asset-meta-ai-says = { $made } (fonte: { $how })
asset-meta-ai-by-says = { $made } ({ $by }) (fonte: { $how })
asset-meta-provenance = Provenienza
asset-meta-c2pa = Ha le credenziali di contenuto C2PA
asset-meta-compression = Compressione
asset-meta-note = Nota
asset-meta-maps = Mappe
asset-meta-sizes =
    { $count ->
        [one] Dimensione
       *[other] Dimensioni
    }
asset-meta-triangles = Triangoli
asset-meta-vertices = Vertici
asset-meta-meshes = Mesh
asset-meta-materials = Materiali
asset-meta-animations = Animazioni
asset-meta-skins = Skin
asset-meta-unreal-asset = Asset Unreal
asset-meta-unreal-saved-with = { $class }, salvato con UE { $engine }
asset-meta-package = Package
asset-meta-cooked = Cotto (build del gioco): anteprime e sorgenti sono rimosse
asset-meta-tiling = Ripetizione
asset-meta-seamless = Senza giunture su entrambi gli assi
asset-meta-analysis-error = Impossibile leggere del tutto questo file: { $error }
asset-copy-blocked = Impossibile copiare: il browser l'ha bloccato
asset-save-error = Impossibile salvare: { $error }
asset-tag-removed = { $tag } rimosso
asset-undo = Annulla
asset-play-error = Impossibile riprodurre questo file: { $error }
asset-copied-hash = Hash copiato
asset-copied-toml = Toml copiato
asset-copied-id = ID copiato
asset-origin-before = Questo file era
asset-origin-class-engine = , un { $class } salvato da UE { $engine }.
asset-origin-class = , un { $class }.
asset-origin-engine = { " " }salvato da UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor l'ha esportato il { $date }.
asset-origin-editor-notes = Unreal Editor l'ha esportato il { $date } ({ $notes }).
asset-origin-hoard = Hoard l'ha estratto dal package il { $date }.
asset-origin-hoard-notes = Hoard l'ha estratto dal package il { $date } ({ $notes }).
asset-origin-download = Scarica il file .{ $ext } originale
asset-origin-discarded = Il package originale è stato scartato dopo la conversione.
asset-details = Dettagli dell'asset
asset-in-pack = in
asset-previous = Asset precedente
asset-next = Asset successivo
asset-favorite = Preferito
asset-favorite-on-title = Nei tuoi preferiti (F lo toglie)
asset-favorite-off-title = Aggiungi ai preferiti (F)
asset-layout-panel = Mostra in un pannello laterale
asset-layout-page = Mostra a pagina intera
asset-layout-page-title = Mostra a pagina intera: un'anteprima grande accanto ai dettagli
asset-close = Chiudi i dettagli
asset-rename-title = Clicca per rinominare
asset-name = Nome
asset-views-alt = Il modello da quattro direzioni
asset-inspect-aria = Ispeziona { $name } a schermo intero
asset-inspect-title = Ispeziona: zoom, ripetizione, canali
asset-playback-position = Posizione di riproduzione
asset-playback-value = { $current } di { $duration }
asset-pause = Pausa
asset-play = Riproduci
asset-waveform-drawing = Disegno della forma d'onda…
asset-waveform-skipped = Nessuna forma d'onda per file così grandi
asset-play-animation = Riproduci come animazione
asset-show-views = Mostra il foglio a quattro viste visto dal tagger
asset-palette = Colori dell'immagine
asset-swatch-title = { $hex }, { $percent }% dell'immagine. Trova asset con questo colore
asset-swatch-aria = Trova asset con { $hex }, { $percent }% di questa immagine
asset-more-like-this = Altri simili
asset-desc-label = La tua descrizione
asset-desc-placeholder = Cos'è, dove lo useresti…
asset-save = Salva
asset-cancel = Annulla
asset-desc-family = Viene salvata in tutte le { $count } varianti
asset-desc-edit = Modifica descrizione
asset-desc-write = Scrivi una descrizione
asset-desc-ai = Scritta dall'IA
asset-desc-file = Dal file
asset-desc-store-before = Dalla
asset-desc-store-page = pagina di { $store }
asset-desc-store-page-unnamed = pagina dello store
asset-desc-store-after = (l'intero pacchetto)
asset-less = Meno
asset-more = Altro
asset-tags = Tag
asset-tag-add-label = Aggiungi un tag
asset-tag-add-placeholder = Aggiungi un tag, ad es. mood:tense
asset-tag-add = Aggiungi
asset-apply-family = Applica le modifiche a tag, categoria e descrizione a tutte le { $count } varianti
asset-collections = Raccolte
asset-collection-remove = Rimuovi da { $name }
asset-collection-add-another = Aggiungi a un'altra
asset-collection-add = Aggiungi alla raccolta
asset-used-in = Usato in
asset-used-in-title = Mostra cosa contiene { $name }
asset-used-in-synced = sincronizzato { $when }
asset-maps = Mappe
asset-show-more = Mostra altri { $count }
asset-materials-heading =
    { $count ->
        [one] Materiale
       *[other] Materiali
    }
asset-material-maps-aria = { $name }: mappe
asset-variants = Varianti ({ $count } versioni)
asset-copies = Copie identiche
asset-copies-note = Gli stessi byte sono salvati una volta sola; anche questi pacchetti includono questo file.
asset-unreal-converted = Convertito da Unreal
asset-unreal-package = Dal package Unreal
asset-ue-needs-editor-before = In attesa di Unreal Editor: { $reason }. Su una macchina con Unreal installato, esegui
asset-ue-needs-editor-after = e tornerà come FBX.
asset-ue-editor-path = percorso di UnrealEditor-Cmd
asset-ue-unsupported = Conservato come package: { $reason }.
asset-ue-failed = Conversione non riuscita: { $reason }
asset-recovered = Il file .{ $ext } originale era salvato dentro questo package, e Hoard l'ha estratto.
asset-recovered-download = Scarica .{ $ext }
asset-ue-meshes-before = Mesh e animazioni sono salvate nel formato proprio di Unreal. Converti il pacchetto (pagina del pacchetto, Altro), poi esegui
asset-ue-meshes-after = accanto a un'installazione di Unreal Editor per ottenere FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } tag dell'asset registry
       *[other] { $count } tag dell'asset registry
    }
asset-file = File
asset-bytes = Byte
asset-source = Origine
asset-path = Percorso
asset-download-maps =
    { $count ->
        [one] Scarica { $count } mappa
       *[other] Scarica tutte le { $count } mappe
    }
asset-download-linked =
    { $count ->
        [one] Scarica con { $count } file collegato
       *[other] Scarica con { $count } file collegati
    }
asset-download-only-title = Solo questo file, senza i file che usa
asset-download-only = Solo questo file
asset-include-animations = Includi le animazioni nello .zip
asset-copy-toml = Copia la voce di hoard.toml
asset-copy-id = Copia ID
asset-teach = Insegna questo file a Hoard
asset-teach-title = Descrivi il file di questo store così Hoard sa leggerlo (Impostazioni → File degli store)
asset-archive = Archivia
asset-archive-title = Escluso dalla ricerca finché non viene ripristinato o eliminato definitivamente, dall'Archivio

## ImageViewer.vue

image-channel-red = Canale rosso
image-channel-green = Canale verde
image-channel-blue = Canale blu
image-channel-alpha = Canale alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Sfondo a scacchi (K per cambiare)
image-bg-black-title = Sfondo nero (K per cambiare)
image-bg-gray-title = Sfondo grigio (K per cambiare)
image-bg-white-title = Sfondo bianco (K per cambiare)
image-bg-checker = sfondo a scacchi
image-bg-black = sfondo nero
image-bg-gray = sfondo grigio
image-bg-white = sfondo bianco
image-inspect-aria = Ispeziona { $name }
image-preview-size = anteprima { $width }×{ $height }, { $bytes }
image-preview = anteprima
image-resolution = Risoluzione
image-full-title = Carica l'immagine completa { $width }×{ $height } (F); succede anche ingrandendo oltre l'anteprima
image-full-loading = Caricamento della risoluzione piena…
image-full-failed = La risoluzione piena non si è caricata: riprova
image-full = Risoluzione piena
image-zoom = Zoom
image-zoom-out-title = Riduci (−)
image-zoom-out = Riduci
image-zoom-in-title = Ingrandisci (+)
image-zoom-in = Ingrandisci
image-fit-title = Adatta (0)
image-fit = Adatta
image-actual-title = Pixel reali (1)
image-view = Vista
image-tile-title = Affianca 3×3 per controllare le giunture (T)
image-tile = Affianca
image-channels = Canali
image-close = Chiudi (Esc)
image-failed = Impossibile caricare questa immagine.
image-loading = Caricamento…
image-key-gestures = Scorri o pizzica per lo zoom · trascina per spostarti · doppio clic per ingrandire o adattare
image-key-tile = affianca
image-key-background = sfondo
image-key-full = risoluzione piena
image-key-channels = canali

## SpritePlayer.vue

sprite-frame-aria = { $name }, fotogramma { $frame } di { $count }
sprite-failed = Questo non si può riprodurre.
sprite-loading = Caricamento dei fotogrammi…
sprite-previous = Fotogramma precedente
sprite-next = Fotogramma successivo
sprite-pause = Metti in pausa l'animazione
sprite-play = Riproduci l'animazione
sprite-animation = Animazione
sprite-all-frames = Tutti i fotogrammi
sprite-grid-title = Come il foglio è diviso in fotogrammi
sprite-columns = Colonne
sprite-rows = Righe
sprite-fps-aria = Fotogrammi al secondo
sprite-fps = fps

## TextViewer.vue, TextPreview.vue e text.ts

text-lang-config = Configurazione
text-lang-table = Tabella
text-lang-code = Codice
text-lang-script = Script
text-lang-text = Testo
text-copy-blocked = Impossibile copiare: il browser l'ha bloccato
text-read-aria = Leggi { $name }
text-lines =
    { $count ->
        [one] { $count } riga
       *[other] { $count } righe
    }
text-show-as = Mostra come
text-tree-title = Albero richiudibile (T)
text-tree = Albero
text-pretty-title = Reindentato (P)
text-pretty = Formattato
text-raw-title = Come è scritto (R)
text-raw = Grezzo
text-document-title = Come documento (D)
text-document = Documento
text-source-title = Il markdown come è scritto (R)
text-source = Sorgente
text-fold = Chiudi
text-expand-all-title = Espandi tutto
text-expand-all = Espandi tutto
text-collapse-all-title = Comprimi tutto
text-collapse-all = Comprimi tutto
text-group-text = Testo
text-wrap-title = A capo per le righe lunghe (W)
text-wrap = A capo
text-copy-title = Copia il testo (C)
text-copied = Copiato
text-copy = Copia
text-download-title = Scarica il file
text-download = Scarica
text-close = Chiudi (Esc)
text-truncated = Mostrati i primi { $shown } di { $total }.
text-truncated-json = Mostrati i primi { $shown } di { $total } (l'albero richiede il file intero).
text-show-all = Mostra tutto
text-json-error = Questo JSON non è valido: { $error }
text-contents-aria = Contenuto di { $name }
text-loading = Caricamento…
text-load-error = Impossibile caricare questo file: { $error }
text-binary = Questo file è binario, non testo, quindi qui non c'è niente da leggere. Scaricalo per aprirlo con il suo strumento.
text-key-tree = albero
text-key-pretty = formattato
text-key-raw = grezzo
text-key-document = documento
text-key-source = sorgente
text-key-wrap = a capo
text-key-copy = copia
text-key-find = trova
text-key-close = chiudi
text-preview-aria = Leggi { $name } a schermo intero
text-preview-read = Leggi

## JsonTree.vue e JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementi
    }
json-keys =
    { $count ->
        [one] { $count } chiave
       *[other] { $count } chiavi
    }
json-fold = Chiudi
json-unfold = Apri
json-unfold-summary = Apri { $summary }
json-show-more = Mostra altri { $count }
json-left = su { $count } rimanenti
json-picker-more = … altri { $count }

## Visualizzatori di modelli 3D (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = il visualizzatore 3D non si è caricato ({ $error }); ricaricare la pagina può aiutare
model-loading-viewer = Caricamento del visualizzatore 3D…
model-loading = Caricamento del modello…
model-show-failed = Impossibile mostrare questo modello: { $error }
model-preview-failed = Impossibile visualizzare l'anteprima di questo modello: { $error }
model-error-converting = il server sta ancora convertendo questo modello
model-error-empty-collada = file Collada vuoto
model-error-no-loader = Nessun caricatore del browser per .{ $ext }
model-dialog = Visualizza { $name } in 3D
model-title-stats =
    { $triangles } tri · { $meshes ->
        [one] { $meshes } mesh
       *[other] { $meshes } mesh
    } · { $materials ->
        [one] { $materials } materiale
       *[other] { $materials } materiali
    }
model-triangles =
    { $count ->
        [one] { $count } triangolo
       *[other] { $count } triangoli
    }
model-group-camera = Telecamera
model-group-shading = Ombreggiatura
model-group-background = Sfondo
model-group-animation = Animazione
model-group-view = Vista
model-group-show = Mostra
model-frame-title = Inquadra il modello (F)
model-frame = Inquadra
model-reset-title = Torna alla vista iniziale (R)
model-reset = Reimposta
model-turn-title = Ruota lentamente (A)
model-turn = Ruota
model-textures-title = Texture (T)
model-textures = Texture
model-full-res-title = Texture a risoluzione piena (H): ogni file di texture così com'è, per quanto grande; se disattivato, anteprime leggere di massimo 2048 px
model-full-res = Ris. piena
model-light-title = Luce ambientale (E)
model-light = Luce
model-grid-title = Griglia del pavimento (G)
model-grid = Griglia
model-bg-checker = Sfondo a scacchi
model-bg-dark = Sfondo scuro
model-bg-gray = Sfondo grigio
model-bg-light = Sfondo chiaro
model-bg-env = Sfondo dell'ambiente
model-bg-title = { $background } (K per cambiare)
model-pause-title = Pausa (Spazio)
model-play-title = Riproduci (Spazio)
model-pause = Pausa
model-play = Riproduci
model-animation-clip = Clip di animazione
model-animation = Animazione
model-close-title = Chiudi (Esc)
model-full-screen = Schermo intero
model-open-full-screen = Apri il visualizzatore 3D a schermo intero
model-menu-fold = Chiudi il menu
model-menu-show = Mostra il menu
model-view-prefix = Vista:{ " " }
model-menu-render = Render
model-menu-maps = Mappe
model-menu-inspect = Ispeziona
model-mode-final = Render finale
model-mode-raw = Render grezzo
model-mode-color = Colore di base
model-mode-roughness = Rugosità
model-mode-metalness = Metallicità
model-mode-normal = Normal
model-mode-ao = Occlusione
model-mode-matcap = Matcap
model-mode-uv = Controllo UV
model-mode-geometry = Geometrie
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Wireframe (W), sopra qualsiasi vista
model-wireframe = Wireframe
model-info-title = Informazioni sulla scena (I)
model-info = Informazioni sulla scena
model-info-close = Chiudi le informazioni sulla scena
model-info-close-title = Chiudi (I)
model-info-geometries = Geometrie
model-info-materials = Materiali
model-info-textures = Texture
model-info-triangles = Triangoli
model-info-vertices = Vertici
model-info-meters = glTF è in metri
model-info-no-units = Questo formato non indica le sue unità di misura
model-info-height = Altezza
model-info-length = Lunghezza
model-info-width = Larghezza
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } unità
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } unità
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Texture non trovata: { $name }
model-missing-some = { $count } texture non trovate: { $names }
model-missing-more = { $count } texture non trovate: { $names }…
model-missing-count = { $count } texture non trovate
model-guessed = Texture abbinate in base al nome; il file non ne indica nessuna.
model-keys-mouse = Trascina per orbitare · trascina con il tasto destro per spostarti · scorri per lo zoom · doppio clic per inquadrare
model-key-frame = inquadra
model-key-reset = reimposta
model-key-view = vista
model-key-wireframe = wireframe
model-key-info = informazioni sulla scena
model-key-textures = texture
model-key-full-res = texture a risoluzione piena
model-key-light = luce
model-key-grid = griglia
model-key-background = sfondo
model-key-play = riproduci

## Materiali (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Colore
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Rugosità
material-role-gloss = Lucidità
material-role-metalness = Metallicità
material-role-ao = Occlusione ambientale
material-role-orm = Occlusione, rugosità, metallo
material-role-mra = Metallo, rugosità, occlusione
material-role-rma = Rugosità, metallo, occlusione
material-role-height = Altezza
material-role-bump = Bump
material-role-cavity = Cavità
material-role-specular = Specularità
material-role-opacity = Opacità
material-role-emissive = Emissione
material-role-translucency = Traslucenza
material-role-base-color-name = colore
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = rugosità
material-role-gloss-name = lucidità
material-role-metalness-name = metallicità
material-role-ao-name = occlusione ambientale
material-role-orm-name = occlusione, rugosità, metallo
material-role-mra-name = metallo, rugosità, occlusione
material-role-rma-name = rugosità, metallo, occlusione
material-role-height-name = altezza
material-role-bump-name = bump
material-role-cavity-name = cavità
material-role-specular-name = specularità
material-role-opacity-name = opacità
material-role-emissive-name = emissione
material-role-translucency-name = traslucenza
material-maps =
    { $count ->
        [one] { $count } mappa
       *[other] { $count } mappe
    }
material-dialog = Visualizza il materiale { $name }
material-up-to = fino a { $size }
material-repeat-size = { $size } per ripetizione
material-shape-sphere = Sfera
material-shape-cube = Cubo
material-shape-plane = Piano
material-shape-title = { $shape } ({ $key })
material-group-shape = Forma
material-group-tiling = Ripetizione
material-real-title = Dimensione reale: una ripetizione misura { $size }, la sfera 2 m di diametro (T per cambiare)
material-repeat-title = Ripeti { $n }× (T per cambiare)
material-real = Reale
material-repeat = { $n }×
material-height-title = Quanto la mappa di altezza spinge la superficie
material-height = Altezza
material-height-strength = Intensità dell'altezza
material-loading = Caricamento del materiale…
material-show-failed = Impossibile mostrare questo materiale: { $error }
material-preview-failed = Impossibile visualizzare l'anteprima di questo materiale: { $error }
material-group-maps = Mappe
material-map-show = Mostra la mappa { $map }
material-map-hide = Nascondi la mappa { $map }
material-map-unused = Non usata dall'anteprima
material-open-full-screen = Apri il visualizzatore di materiali a schermo intero
material-keys-mouse = Trascina per orbitare · trascina con il tasto destro per spostarti · scorri per lo zoom · clicca su una mappa per disattivarla
material-key-shape = forma
material-key-tiling = ripetizione
material-key-turn = ruota

## theme.ts

theme-slate = Ardesia
theme-paper = Carta
theme-linen = Lino
theme-sage = Salvia
theme-lilac = Lilla
theme-blush = Cipria
theme-graphite = Grafite
theme-obsidian = Ossidiana
theme-ember = Brace
theme-moss = Muschio
theme-aubergine = Melanzana
theme-oxblood = Vinaccia
theme-light-slate-blurb = Grigio-blu freddo
theme-light-paper-blurb = Grigio neutro semplice
theme-light-linen-blurb = Crema caldo, ambra
theme-light-sage-blurb = Verde tenue
theme-light-lilac-blurb = Viola pallido
theme-light-blush-blurb = Rosa antico
theme-dark-graphite-blurb = Grigio neutro, senza tinta
theme-dark-slate-blurb = Grigio-blu
theme-dark-obsidian-blurb = Quasi nero, per OLED
theme-dark-ember-blurb = Carbone caldo, ambra
theme-dark-moss-blurb = Verde bosco quasi nero
theme-dark-aubergine-blurb = Nero prugna, lilla
theme-dark-oxblood-blurb = Nero vino, corallo

## texnames.ts

texnames-needs-map = Lo schema ha bisogno di { "{" }map{ "}" }, altrimenti tutte le mappe di un materiale avrebbero lo stesso nome
texnames-placeholders = Lo schema può usare { "{" }name{ "}" }, { "{" }map{ "}" } e { "{" }size{ "}" }
texnames-bad-characters = I nomi non possono contenere < > : " / \ | ? o *
texnames-no-suffix = Assegna un suffisso alle mappe { $role }
texnames-bad-suffix = Il suffisso delle mappe { $role } non può contenere < > : " / \ | ? o *

## storeFormats.ts

storefiles-field-title = Titolo
storefiles-field-title-hint = Il titolo esatto dell'inserzione. Dà il nome al pacchetto, e senza di esso il file viene saltato.
storefiles-field-seller = Venditore
storefiles-field-seller-hint = Chi l'ha creato o lo vende.
storefiles-field-license = Licenza
storefiles-field-license-hint = Come la scrive il file, ad esempio CC-BY-4.0.
storefiles-field-url = Pagina dello store
storefiles-field-url-hint = L'indirizzo dell'inserzione, che inizia con http:// o https://.
storefiles-field-words = Tag
storefiles-field-words-hint = Parole con cui assegnargli i tag. I valori separati da virgole vengono divisi.
storefiles-field-type = Tipo
storefiles-field-type-hint = Cos'è, ad esempio «modello 3D» o «superficie». È anche un tag.
storefiles-field-colors = Colori
storefiles-field-colors-hint = I suoi colori, come tag color:.
storefiles-field-id = ID dello store
storefiles-field-id-hint = L'ID che gli dà lo store: lettere, cifre, _ o -.
storefiles-field-origin = Luogo di creazione
storefiles-field-origin-hint = Dove è stato creato o scansionato, per la descrizione.
storefiles-field-size = Dimensione
storefiles-field-size-hint = Quanto è grande, a parole, per la descrizione.
storefiles-field-title-name = titolo
storefiles-field-seller-name = venditore
storefiles-field-license-name = licenza
storefiles-field-url-name = pagina dello store
storefiles-field-words-name = tag
storefiles-field-type-name = tipo
storefiles-field-colors-name = colori
storefiles-field-id-name = ID dello store
storefiles-field-origin-name = luogo di creazione
storefiles-field-size-name = dimensione

## StoreFormatsPanel.vue

storefiles-load-failed = Impossibile caricare i formati dei file degli store: { $error }
storefiles-remove-title = Rimuovere { $name }?
storefiles-remove-body = { $file } viene eliminato dalla cartella dei dati. I pacchetti già letti con esso conservano ciò che ha letto.
storefiles-remove = Rimuovi
storefiles-removed = { $name } rimosso.
storefiles-remove-failed = Impossibile rimuovere { $name }: { $error }
storefiles-unread = File che Hoard non legge
storefiles-loading = Caricamento…
storefiles-unread-none = Nessuno: nessun file nella libreria sembra quello di uno store non letto.
storefiles-unread-what = File presenti in più di un pacchetto, o con un nome da file di store, che nessun formato legge.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pacchetto
       *[other] { $count } pacchetti
    }
storefiles-unread-example = ad es. { $pack }
storefiles-teach = Insegna questo file a Hoard
storefiles-formats = Formati
storefiles-formats-what = I tuoi vengono provati per primi e sostituiscono quello integrato con lo stesso nome.
storefiles-dir-before = Si trovano in
storefiles-dir-after = .
storefiles-yours = Tuo
storefiles-built-in = Integrato
storefiles-edit = Modifica
storefiles-start-from = Parti da questo

## TagsPanel.vue

tags-load-failed = Impossibile caricare i tag: { $error }
tags-rename-title = Rinominare { $tag }?
tags-merge-title = Unire { $count } tag?
tags-rename-body = I suoi { $count } asset ricevono invece { $into }, e d'ora in poi i tagger useranno il nuovo nome.
tags-merge-body = { $tags } diventano { $into } su un massimo di { $count } asset. D'ora in poi i tagger useranno { $into }.
tags-rename = Rinomina
tags-merge = Unisci
tags-merged =
    { $count ->
        [one] { $count } asset ora ha il tag { $into }
       *[other] { $count } asset ora hanno il tag { $into }
    }
tags-merge-failed = Impossibile unire: { $error }
tags-forget-failed = Impossibile modificarlo: { $error }
tags-find = Trova tag
tags-find-placeholder = Trova tag, ad es. rock o set:
tags-count =
    { $count ->
        [one] { $count } tag
       *[other] { $count } tag
    }
tags-count-more = Oltre { $count } tag
tags-what = Scegli i tag che significano la stessa cosa e uniscili, oppure scegline uno per rinominarlo.
tags-new-name = Nuovo nome
tags-merge-into = Unisci in
tags-saved-as-before = Salvato come
tags-saved-as-after = { "" }
tags-cancel = Annulla
tags-loading = Caricamento…
tags-none = Nessun tag.
tags-none-containing = Nessun tag che contiene «{ $text }».
tags-browse-title = Sfoglia gli asset con il tag { $tag }
tags-browse = Sfoglia
tags-merged-heading = Tag uniti
tags-merged-what = Quando i tagger propongono uno di questi vecchi nomi, l'asset riceve il tag in cui è stato unito.
tags-allow-old = Consenti di nuovo il vecchio nome

## LibraryStats.vue

stats-source-human = Aggiunti da persone
stats-source-ai = Suggeriti dall'IA
stats-source-rule = Regole integrate
stats-source-meta = Dai file
stats-assets = Asset
stats-packs = Pacchetti
stats-on-disk = Su disco
stats-duplicates-title = { $count } file sono copie identiche byte per byte di altri e vengono salvati una volta sola
stats-saved-dedup = Risparmiato con la deduplicazione
stats-distinct-tags = Tag diversi
stats-archive = Nell'archivio
stats-load-failed = Impossibile caricare i dettagli: { $error }
stats-counting = Conteggio…
stats-by-kind = Spazio per tipo
stats-files-size = { $name }: { $count } file, { $size }
stats-months = Aggiunti nell'ultimo anno
stats-month = { $month }: { $count } asset, { $size }
stats-tagging = Tag
stats-tagged = Con tag
stats-untagged = { $count } senza tag
stats-described = Descritti
stats-described-assets = { $count } asset
stats-human-edited = Modificati a mano
stats-human-edited-unit =
    { $count ->
       *[other] asset
    }
stats-source-different = { $count } diversi
stats-most-used = I più usati
stats-show-tag = Mostra gli asset con il tag { $tag }
stats-file-types = Tipi di file
stats-biggest-packs = Pacchetti più grandi
stats-browse-pack = Sfoglia { $name }
stats-largest-files = File più grandi
stats-open-file = Apri { $name }
stats-empty = Ancora niente nella libreria. Aggiungi un pacchetto e i suoi numeri compariranno qui.

## UpdatesPanel.vue

updates-saved = Hoard lo chiamerà quando un aggiornamento sarà pronto
updates-saved-none = Nessun indirizzo da chiamare
updates-save-failed = Impossibile salvare
updates-call-answered = Ha risposto { $result }
updates-call-refused = Non ha accettato la richiesta: { $result }
updates-call-failed = Impossibile chiamarlo
updates-called = Chiamato il { $when } per la { $version }: { $result }
updates-called-failed = Chiamato il { $when } per la { $version }: { $result }. Hoard riproverà tra un'ora.
updates-latest = Hoard { $version } è la versione più recente.
updates-out = È uscito Hoard { $version }.
updates-out-idle = È uscito Hoard { $version }; la chiamata partirà entro un paio di minuti, mentre non succede niente.
updates-out-busy = È uscito Hoard { $version }; in attesa che non succeda niente ({ $work }).
updates-busy-more = altri { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } processo in attesa
       *[other] { $count } processi in attesa
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } caricamento
       *[other] { $count } caricamenti
    }
updates-list-separator = ,{ " " }
updates-what = Quando esce una nuova versione, Hoard può chiamare un indirizzo appena non succede niente (nessun caricamento, processo, conversione o manutenzione), così ciò che aggiorna il suo container non lo riavvia a metà di un caricamento. Invia una POST con la versione in un corpo JSON. Richiede il controllo delle versioni qui sopra.
updates-url = Indirizzo da chiamare
updates-url-placeholder = Nessuno (ad es. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Salvato (scrivi per sostituirlo)
updates-auth-placeholder = Nessuno (ad es. Bearer my-token)
updates-remove = Rimuovi
updates-save = Salva
updates-call-now-title = Viene eseguito ciò a cui è collegato
updates-call-now = Chiamalo ora
updates-what-calls = Cosa può chiamare?
updates-watchtower = L'API HTTP di Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , con{ " " }
updates-watchtower-token = { " " }e il token di{ " " }
updates-watchtower-its = { " " }(con il suo endpoint{ " " }
updates-watchtower-endpoint = { " " }attivato).
updates-portainer = Un webhook di Portainer
updates-portainer-rest = { " " }sullo stack o sul container di Hoard, con il nuovo pull dell'immagine attivato. Il token è nell'URL.
updates-script = Uno script sull'host
updates-script-listener = { " " }dietro un listener di webhook (come{ " " }
updates-script-runs = ) che esegue{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }o qualsiasi altra cosa che accetti un webhook.
updates-poll-before = Oppure fai a meno dell'indirizzo: uno script con un token di accesso può interrogare{ " " }
updates-poll-until = { " " }finché{ " " }
updates-poll-and = { " " }non è impostato e{ " " }
updates-poll-after = { " " }non è true, poi scaricare la nuova versione e riavviare Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Impossibile caricare la Manutenzione
maintenance-reanalyze-title = Rianalizzare tutti i pacchetti?
maintenance-reanalyze-body = Ogni file viene riletto, il che può richiedere molto tempo su una libreria grande; i pacchetti risultano in lavorazione finché non arriva il loro turno. I tuoi nomi, tag e note restano.
maintenance-reanalyze-confirm = Rianalizza
maintenance-gc-title = Eliminare i file inutilizzati?
maintenance-gc-body = I file salvati a cui niente nella libreria fa riferimento vengono eliminati definitivamente. I pacchetti nell'archivio conservano i propri.
maintenance-gc-confirm = Eliminali
maintenance-start-failed = Impossibile avviare «{ $task }»
maintenance-just-now = proprio ora
maintenance-minutes-ago = { $count } min fa
maintenance-hours-ago = { $count } h fa
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Caricamento…
maintenance-running = In esecuzione…
maintenance-starting = Avvio…
maintenance-run = Esegui
maintenance-started = { $detail } · avviata { $when }
maintenance-starting-detail = Avvio
maintenance-finished = Completata:{ " " }
maintenance-failed = Non riuscita:{ " " }
maintenance-never-run = Mai eseguita
maintenance-history = Esecuzioni precedenti
maintenance-col-task = Attività
maintenance-col-finished = Completata
maintenance-col-took = Durata
maintenance-col-result = Risultato
maintenance-script-before = Anche i backup e il controllo dei danni possono essere eseguiti da uno script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }e{ " " }
maintenance-script-after = . Anche quelle esecuzioni compaiono qui.

## SettingsPage.vue

settings-title = Impostazioni
settings-loading = Caricamento…
settings-load-failed = Impossibile caricare le impostazioni: { $error }
settings-page-general = Generali
settings-page-general-blurb = Come si chiama questo server, per tutti quelli che lo usano.
settings-page-appearance = Aspetto
settings-page-appearance-blurb = I temi che questo browser usa quando è chiaro e quando è scuro. Salvati solo in questo browser.
settings-page-library = Libreria
settings-page-library-blurb = Cosa c'è in Hoard: quanto è grande, di cosa è fatto, come è cresciuto e quanto bene è taggato. Gli elementi archiviati non vengono contati.
settings-page-tags = Tag
settings-page-tags-blurb = Tutti i tag della libreria. Unisci quelli che significano la stessa cosa, o rinominane uno; da quel momento i tagger useranno il nuovo nome.
settings-page-store-files = File degli store
settings-page-store-files-blurb = I file che gli store mettono accanto alle loro opere, che Hoard legge per dare un nome ai pacchetti e attribuirne i crediti agli autori. Insegnagli quelli che non conosce, e condividi ciò che gli hai insegnato.
settings-page-locations = Posizioni della libreria
settings-page-locations-blurb = Le cartelle sul server che contengono la libreria: quelle in cui Hoard conserva i file, e le cartelle di pacchetti che indicizza dove si trovano.
settings-page-tools = Strumenti
settings-page-tools-blurb = Programmi esterni che Hoard usa per convertire e leggere i file.
settings-page-ai = Tag con IA
settings-page-ai-blurb = Un modello di IA dà un nome, descrive e assegna tag ai nuovi pacchetti, oltre alle regole integrate. I suoi tag sono contrassegnati come suggeriti.
settings-page-search = Ricerca per immagini
settings-page-search-blurb = Cerca le immagini in base a ciò che mostrano, non solo ai loro nomi, e trova cose simili per tipo, non solo per colore.
settings-page-adding = Aggiunta di pacchetti
settings-page-adding-blurb = Cosa fa Hoard con un pacchetto quando viene aggiunto, oltre a leggerlo e assegnargli i tag.
settings-page-downloads = Download
settings-page-downloads-blurb = Come vengono chiamati i file in ciò che Hoard invia: download, zip, lo strumento hoard e gli assistenti IA. I file nella libreria restano come sono.
settings-page-account = Account
settings-page-account-blurb = La tua password, le altre persone che possono accedere e i token di accesso per script e altri programmi.
settings-page-auth = Autenticazione
settings-page-auth-blurb = Come accedono le persone: gli account di Hoard, una directory LDAP, Microsoft Active Directory o un provider OAuth, quanto durano le sessioni e quali gruppi ricevono quale ruolo.
settings-page-connection = Connessione
settings-page-connection-blurb = Come browser e app raggiungono Hoard: l'indirizzo con cui è conosciuto, e HTTPS, che serve ad alcune funzioni del browser.
settings-page-performance = Prestazioni
settings-page-performance-blurb = Quanto lavora il server, come vengono create e conservate le miniature, e il massimo che accetta.
settings-page-issues = Problemi
settings-page-issues-blurb = Cosa è andato storto aprendo o elaborando i file: archivi che non si sono estratti, file illeggibili, processi non riusciti. Riprovali una volta risolta la causa, oppure ignorali.
settings-page-maintenance = Manutenzione
settings-page-maintenance-blurb = Backup, controllo dei danni e pulizia. Ne viene eseguita una per tipo alla volta, e puoi lasciare questa pagina mentre sono in corso.
settings-the-default = il valore predefinito
settings-default-on = attivo
settings-default-off = disattivato
settings-default-auto = automatico
settings-option-saved = { $name }: { $value }
settings-back-to-default = Ripristina il predefinito
settings-save = Salva
settings-use = Usa
settings-status-off = Disattivato

## SettingsPage.vue: Aspetto

settings-themes = Temi
settings-look-light = Chiaro
settings-look-dark = Scuro
settings-look-group =
    { $scheme ->
        [light] Tema chiaro
       *[dark] Tema scuro
    }
settings-preview =
    Viene mostrato il tema { $preview ->
        [light] chiaro
       *[dark] scuro
    } così puoi vederlo. Stai usando quello { $used ->
        [light] chiaro
       *[dark] scuro
    }{ $system ->
        [yes] { " " }(segue il sistema)
       *[other] { "" }
    }, che torna quando lasci questa pagina.
settings-preview-back =
    Torna al tema { $used ->
        [light] chiaro
       *[dark] scuro
    }
settings-look-hint = Passa da chiaro a scuro (o segui il sistema) dalla barra superiore.

## SettingsPage.vue: Strumenti

settings-blender-what = Converte i modelli .blend, .fbx, .dae, .usd, .ply, .stl e .3ds in glTF in background, così ricevono miniature e un'anteprima 3D. I pacchetti aggiunti mentre era disattivato vengono convertiti appena è pronto.
settings-blender-download = Usa un Blender installato, oppure scarica Blender 4.5 LTS (circa 400 MB) da un mirror di blender.org.
settings-ffmpeg-what = Codifica i download in MP3, OGG e Opus, e legge l'audio che Hoard non sa decodificare da sé (AAC, M4A, WMA, …).
settings-ffmpeg-download = Usa un ffmpeg di sistema con gli encoder giusti, oppure scarica una build statica (circa 80 MB).
settings-tool-reset = { $tool }: ripristina il predefinito
settings-tool-no-download = Usa un { $tool } installato. Non c'è una build da scaricare per questa macchina, quindi installalo o indica il suo percorso.
settings-tool-path = Percorso del programma { $tool }
settings-mode-off = Disattivato
settings-mode-auto = Automatico
settings-mode-path = Questo programma
settings-tool-looking = Ricerca…
settings-tool-downloading = Download… l'avanzamento è nell'indicatore delle attività
settings-tool-ready = Pronto: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Gli strumenti con cui i file MIDI vengono riprodotti e convertiti.
settings-soundfont-file = File SoundFont
settings-soundfont-placeholder = Trovato sul server, o integrato
settings-soundfont-default = trovato sul server, o integrato

## SettingsPage.vue: Ricerca per immagini

settings-clip-heading = Cerca in base a ciò che mostrano le immagini
settings-clip-reset = Ricerca per immagini: ripristina il predefinito
settings-clip-saved-on = Ricerca per immagini: attiva
settings-clip-saved-off = Ricerca per immagini: disattivata
settings-clip-what = Scrivi ciò che cerchi («roccia muschiosa», «porta fantascientifica», «barile di legno») e le immagini che lo mostrano compaiono accanto ai nomi che contengono quelle parole. «Altri simili» trova cose dello stesso tipo, non solo degli stessi colori.
settings-clip-toggle = Attiva la ricerca per immagini
settings-clip-hint = Usa il modello CLIP di OpenAI, eseguito sul processore di questo server: non esce niente. Attivandola, il modello ({ $size } MB) viene scaricato da Hugging Face, poi impara ogni immagine in background a bassa priorità. Occupa da 300 a 600 MB di memoria mentre lavora o qualcuno cerca. Disattivandola, il modello viene eliminato ma ciò che ha imparato resta, per la volta successiva.
settings-clip-looking = Ricerca del modello…
settings-clip-downloading = Download del modello… l'avanzamento è nell'indicatore delle attività
settings-clip-ready-empty = Pronto. Le immagini vengono imparate man mano che si aggiungono pacchetti.
settings-clip-ready-all =
    { $count ->
        [one] Pronto: conosce { $count } immagine
       *[other] Pronto: conosce tutte le { $count } immagini
    }
settings-clip-learning = Pronto: sta imparando le immagini, { $done } di { $total } finora
settings-clip-unavailable = Impossibile ottenere il modello: { $detail }. Nuovo tentativo ogni ora.

## SettingsPage.vue: Tag con IA

settings-ai-service = Servizio
settings-ai-off = Disattivato
settings-ai-off-hint = Ai pacchetti vengono assegnati tag solo con le regole integrate.
settings-ai-local = Modello locale
settings-ai-local-hint = Il servizio llm nel file compose di Hoard (o cargo xtask llm). Scarica un modello qui sotto; viene eseguito su questa macchina e non esce niente.
settings-ai-chatgpt = Piano ChatGPT
settings-ai-chatgpt-hint = Usa il tuo piano ChatGPT Go, Plus, Pro o Business invece di una chiave API.
settings-ai-openrouter-hint = Una sola chiave per centinaia di modelli ospitati, alcuni gratuiti. I modelli che accettano immagini sono contrassegnati.
settings-ai-openai-hint = Ospitato da OpenAI.
settings-ai-anthropic-hint = Ospitato da Anthropic, tramite il suo endpoint compatibile con OpenAI.
settings-ai-gemini-hint = Ospitato da Google, tramite il suo endpoint compatibile con OpenAI.
settings-ai-ollama-hint = Un server Ollama gestito da te. Cambia l'indirizzo se non è un container chiamato ollama.
settings-ai-custom = Altro (compatibile con OpenAI)
settings-ai-custom-hint = Qualsiasi cosa che parli l'API chat di OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Connessione…
settings-ai-connected =
    { $count ->
        [one] Connesso · { $count } modello
       *[other] Connesso · { $count } modelli
    }
settings-ai-connected-none = Connesso, ma non elenca alcun modello; scrivi il nome del modello
settings-chatgpt-connected-none = Connesso; scrivi il nome del modello
settings-ai-takes-images = accetta immagini
settings-ai-saved = Tag con IA: { $service }
settings-ai-saved-off = I tag con IA sono disattivati
settings-ai-reset = Tag con IA: ripristina il predefinito
settings-ai-on = Attivo: { $service } · { $model }
settings-ai-url = Indirizzo
settings-ai-key = Chiave API
settings-ai-key-saved = Salvata (scrivi per sostituirla)
settings-ai-key-paste = Incolla la tua chiave
settings-ai-key-optional = Nessuna (facoltativa)
settings-ai-key-remove = Rimuovi
settings-ai-get-key = Ottieni una chiave
settings-ai-model = Modello
settings-ai-model-pick = Scegli o scrivi un modello
settings-ai-model-name = Nome del modello
settings-ai-refresh = Aggiorna
settings-ai-connect = Connetti
settings-ai-need-key = Aggiungi la chiave API per vederne i modelli
settings-ai-vision = Invia anche le miniature (richiede un modello che accetta immagini)
settings-ai-batch = Per richiesta
settings-ai-batch-unit = asset (meno per i piccoli modelli locali, di più per i grandi modelli ospitati)

## SettingsPage.vue: accesso a ChatGPT

settings-chatgpt-account = Account
settings-chatgpt-checking = Verifica…
settings-chatgpt-status-as = Accesso effettuato come { $email }
settings-chatgpt-status = Accesso effettuato
settings-chatgpt-signed-in-as = Accesso a ChatGPT effettuato come { $email }
settings-chatgpt-signed-in = Accesso a ChatGPT effettuato
settings-chatgpt-signed-out = Disconnesso da ChatGPT
settings-chatgpt-continue = Continua con ChatGPT
settings-chatgpt-sign-out = Esci
settings-chatgpt-sign-out-title = Uscire da ChatGPT?
settings-chatgpt-sign-out-in-use = I tag con IA usano il tuo piano ChatGPT, quindi si fermano finché non accedi di nuovo o scegli un altro servizio.
settings-chatgpt-sign-out-body = Hoard dimentica l'accesso e chiede a OpenAI di terminarlo.
settings-chatgpt-manage = Gestisci utilizzo
settings-chatgpt-finish-before = Completa l'accesso nella finestra che si è aperta (
settings-chatgpt-finish-link = aprila di nuovo
settings-chatgpt-finish-middle = ). Se finisce su una pagina che non si carica, a un indirizzo che inizia con{ " " }
settings-chatgpt-finish-after = , copia quell'indirizzo e incollalo qui:
settings-chatgpt-paste = L'indirizzo su cui è terminato l'accesso
settings-chatgpt-finish = Completa
settings-chatgpt-usage = Le richieste di tag vengono conteggiate nell'utilizzo del tuo piano ChatGPT. Gestisci utilizzo mostra quanto ha usato Hoard e ti permette di limitarlo. Per disconnettere Hoard da ChatGPT stesso, vai su chatgpt.com in Impostazioni → Sicurezza e accesso → Accedi con ChatGPT.

## SettingsPage.vue: Modelli locali

settings-local-downloading-progress = Download { $percent }% · { $done } di { $total }
settings-local-downloading = Download…
settings-local-incomplete = Download interrotto
settings-local-failed = L'ultima volta non si è avviato
settings-local-in-use-running = In uso · in esecuzione
settings-local-in-use = In uso
settings-local-starting = Avvio…
settings-local-running = In esecuzione
settings-local-ready-images = Pronto · accetta immagini
settings-local-ready-text = Pronto · solo testo
settings-local-delete-title = Eliminare { $name }?
settings-local-delete-in-use = I tag con IA usano questo modello, quindi si fermano finché non ne scegli un altro. I suoi file vengono eliminati dal server dei modelli.
settings-local-delete-downloading = Il download si interrompe e ciò che è stato scaricato finora viene eliminato.
settings-local-delete-body = I suoi file vengono eliminati dal server dei modelli. Puoi scaricarlo di nuovo più tardi.
settings-local-delete = Elimina
settings-local-deleting = Eliminazione…
settings-local-cancel = Annulla
settings-local-resume = Riprendi
settings-local-download = Scarica
settings-local-start-before = Avvialo con{ " " }
settings-local-start-or = { " " }(o{ " " }
settings-local-start-developing = { " " }durante lo sviluppo), poi{ " " }
settings-local-start-retry = riprova
settings-local-start-after = . Va bene anche un server che esegue un solo modello: indica qui sotto il nome del suo modello.
settings-local-list = Sul server dei modelli
settings-local-none = Ancora niente scaricato. Scegline uno qui sotto; resta sul server e puoi cambiarlo in qualsiasi momento.
settings-local-catalog = Modelli da scaricare
settings-local-size = { $size } GB · { $license }
settings-local-other = Un altro modello da Hugging Face
settings-local-other-placeholder = Un altro da Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Aggiunta di pacchetti

settings-store-pages = Pagine degli store
settings-store-pages-what = I nuovi pacchetti vengono cercati negli store in background, così ricevono la pagina dello store, il venditore e la licenza.
settings-store-lookup = Cerca automaticamente le pagine degli store
settings-unreal = Package Unreal
settings-unreal-reset = Package Unreal: ripristina il predefinito
settings-unreal-what = Cosa fare con i file .uasset.
settings-unreal-off = Lascia così
settings-unreal-off-hint = Converti dalla pagina di un pacchetto quando vuoi.
settings-unreal-keep = Converti, conserva gli originali
settings-unreal-keep-hint = Texture, suoni e materiali escono in formati comuni; resta anche il file .uasset.
settings-unreal-replace = Converti, elimina gli originali
settings-unreal-replace-hint = Risparmia spazio una volta estratto il contenuto. Le mesh aspettano comunque il worker Unreal.

## SettingsPage.vue: Download

settings-tn = Nomi delle texture
settings-tn-saved = Nomi delle texture salvati
settings-tn-reset = Nomi delle texture: ripristina il predefinito
settings-tn-what = Le mappe di un materiale escono con il nome del materiale e lo scopo di ogni mappa, comunque si chiamino i file: Rock_Normal.png, non rock_moss_xeokah0_raw_8K_Normal.png. I file nella libreria non vengono toccati, e una mappa nominata da un modello o MTL inviato insieme mantiene quel nome, così il modello la trova comunque.
settings-tn-toggle = Dai alle mappe delle texture il nome del loro materiale
settings-tn-style = Stile
settings-tn-style-group = Stile dei nomi
settings-tn-descriptive = Descrittivo
settings-tn-short = Breve
settings-tn-custom = Personalizzato
settings-tn-pattern = Schema
settings-tn-hint-name = { " " }il materiale,{ " " }
settings-tn-hint-map = { " " }lo scopo della mappa,{ " " }
settings-tn-hint-size = { " " }la sua dimensione quando il set ne ha più di una (aggiunta alla fine se lo schema non la include).
settings-tn-example = Esempio
settings-tn-suffixes = Suffissi per ogni tipo di mappa

## SettingsPage.vue: Generali e Connessione

settings-name = Nome
settings-name-what = Come si chiama questo server nella barra superiore, nella scheda del browser e nella pagina di accesso, così con più di un Hoard puoi distinguerli.
settings-address = Indirizzo
settings-address-what = L'indirizzo che persone e app usano per raggiungere Hoard, quando non è quello su cui si trova questo browser: quello di un reverse proxy, di un tunnel o un nome della tailnet, come https://hoard.example.com. I link che Hoard fornisce (a Claude e ad altre app, nell'accesso, nel suo certificato) lo usano.
settings-public-url = Indirizzo pubblico
settings-public-url-placeholder = Dall'indirizzo del browser (ad es. https://hoard.example.com)
settings-public-url-default = dall'indirizzo del browser

## SettingsPage.vue: Prestazioni

settings-render = Miniature 3D
settings-render-toggle = Genera le miniature dei modelli 3D
settings-gpu-on = Sulla GPU: { $renderer }
settings-gpu-software = Nessuna GPU trovata, quindi via software ({ $renderer })
settings-gpu-none = Nessuna GPU né renderer software trovati, quindi i modelli non hanno miniature
settings-thumbs = Miniature
settings-thumbs-what = Come vengono salvate le nuove miniature e viste dei modelli. Quelle con trasparenza, e la pixel art, sono sempre PNG. Per ridurre quelle che hai già, esegui Riduci le miniature in Manutenzione.
settings-previews-size = Ora le anteprime occupano { $size }.
settings-thumbs-saved = Miniature: { $quality }
settings-thumbs-small = Più piccole
settings-thumbs-small-hint = JPEG, qualità 70. Vanno bene nella griglia; un po' morbide da vicino.
settings-thumbs-balanced = Bilanciate
settings-thumbs-balanced-hint = JPEG, qualità 82. Difficili da distinguere dall'originale alle dimensioni di una miniatura.
settings-thumbs-sharp = Più nitide
settings-thumbs-sharp-hint = JPEG, qualità 92. Circa il doppio di Bilanciate.
settings-thumbs-lossless = Senza perdita
settings-thumbs-lossless-hint = PNG, esattamente come generate. Parecchie volte più grandi.
settings-limits = Lavoro e limiti
settings-limits-what = Ognuno viene salvato quando esci dal campo o premi Invio. Le dimensioni accettano unità: 100 GB, 512 MB, 1,5 TB; un numero semplice è in GB.
settings-workers = Processi in background
settings-workers-unit = processi alla volta (importazione, tag, conversioni)
settings-max-upload = Caricamento massimo
settings-max-upload-unit = per file
settings-max-expand = Pacchetto massimo
settings-max-expand-unit = una volta estratto
settings-conversion-cache = Download convertiti
settings-conversion-cache-unit-now = conservati al massimo, { $size } ora; i meno usati di recente vanno via per primi
settings-conversion-cache-unit = conservati al massimo; i meno usati di recente vanno via per primi

## SettingsPage.vue: Manutenzione

settings-updates = Aggiornamenti
settings-version = Questo è Hoard { $version }.
settings-update-check = Controlla GitHub ogni poche ore per una nuova versione

## Tipi di asset (api.ts)

api-kind-texture = Texture
api-kind-vector = Grafica vettoriale
api-kind-audio = Audio
api-kind-model = Modelli
api-kind-animation = Animazioni
api-kind-font = Font
api-kind-shader = Shader
api-kind-material = Materiali
api-kind-video = Video
api-kind-document = Documenti
api-kind-data = Dati
api-kind-other = Altro
api-kind-texture-name = texture
api-kind-vector-name = grafica vettoriale
api-kind-audio-name = audio
api-kind-model-name = modelli
api-kind-animation-name = animazioni
api-kind-font-name = font
api-kind-shader-name = shader
api-kind-material-name = materiali
api-kind-video-name = video
api-kind-document-name = documenti
api-kind-data-name = dati
api-kind-other-name = altro

## Notifiche, finestre e caricamento (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Annulla
ui-today = oggi
ui-yesterday = ieri
ui-days-ago =
    { $days ->
        [one] { $days } giorno fa
       *[other] { $days } giorni fa
    }
ui-notifications = Notifiche
ui-dismiss = Ignora
ui-cancel = Annulla
ui-lazy-loading = Caricamento di { $what }…
ui-lazy-failed = Impossibile caricare { $what }.
ui-lazy-reload = Ricarica la pagina

## ResetButton.vue

reset-title = Ripristina il predefinito: { $value }
reset-label = Ripristina il predefinito, { $value }

## SizeInput.vue

size-invalid = Una dimensione come 100 GB, 512 MB o 1,5 TB
size-hint = Una dimensione come 100 GB, 512 MB o 1,5 TB; un numero semplice è in GB

## AuthPanel.vue (Impostazioni → Autenticazione)

auth-load-failed = Impossibile caricare le impostazioni di autenticazione: { $error }
auth-saved = Salvato. Vale dal prossimo accesso.
auth-method-internal = Interni
auth-method-internal-hint = Nomi e password conservati da Hoard, aggiunti in Impostazioni → Account.
auth-method-ldap-hint = Una directory verifica il nome e la password scritti nella pagina di accesso: OpenLDAP, FreeIPA, LLDAP o un altro server LDAP.
auth-method-ad-hint = Gli account del dominio Windows accedono con il nome e la password di sempre.
auth-method-oauth-hint = Un pulsante «Accedi con…» che porta a Google, Microsoft, GitHub, Keycloak, Authentik o un altro provider e ritorna.
auth-session-unused = Dopo 30 giorni di inutilizzo
auth-session-1h = 1 ora dopo l'accesso
auth-session-8h = 8 ore dopo l'accesso
auth-session-day = Un giorno dopo l'accesso
auth-session-week = Una settimana dopo l'accesso
auth-session-30d = 30 giorni dopo l'accesso
auth-session-hours =
    { $hours ->
        [one] { $hours } ora dopo l'accesso
       *[other] { $hours } ore dopo l'accesso
    }
auth-role-view = Può vedere
auth-role-edit = Può modificare
auth-role-admin = Amministratore
auth-role-word-view = può vedere
auth-role-word-edit = può modificare
auth-role-word-admin = amministratore
auth-role-word-default = { $role } (il ruolo predefinito)
auth-tls-starttls-hint = LDAP semplice (porta 389) passato a TLS prima di inviare qualsiasi cosa.
auth-tls-ldaps-hint = TLS fin dall'inizio (porta 636).
auth-tls-none = Nessuna
auth-tls-none-hint = Le password attraversano la rete così come sono. Solo su una rete di cui ti fidi.
auth-test-failed = Impossibile eseguire la prova: { $error }
auth-preset-custom = Altro (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel e altri: indica l'issuer e Rileva compila gli indirizzi.
auth-preset-google-note = Crea un client OAuth (Applicazione web) in Google Cloud Console → API e servizi → Credenziali. Google non invia gruppi, quindi assegna i ruoli in Impostazioni → Account, e disattiva la creazione degli account al primo accesso, a meno che debba entrare chiunque abbia un account Google.
auth-preset-microsoft-note = Registra un'app in Entra ID → Registrazioni app (con un reindirizzamento Web all'indirizzo qui sotto). Metti l'ID del tuo tenant al posto di «organizations» per far entrare solo la tua organizzazione. Per i ruoli dai gruppi, aggiungi l'attestazione dei gruppi in Configurazione token: elenca gli ID dei gruppi.
auth-preset-github-note = Crea un'app OAuth in GitHub → Settings → Developer settings. GitHub non invia gruppi, quindi assegna i ruoli in Impostazioni → Account, e disattiva la creazione degli account al primo accesso, a meno che debba entrare chiunque abbia un account GitHub.
auth-preset-gitlab-note = Crea un'applicazione in GitLab → Preferences → Applications (o nell'area di amministrazione della tua istanza: cambia gitlab.com negli indirizzi).
auth-discovered = Indirizzi del provider compilati.
auth-copied = Copiato
auth-copy-failed = Impossibile copiare: selezionalo e copialo tu
auth-sign-in-with = Accedi con
auth-own-accounts-still-work = Gli account di Hoard accedono comunque con le proprie password, così una directory o un provider non raggiungibile o configurato male non può chiuderti fuori.
auth-sessions = Sessioni
auth-sign-out-after = Disconnetti le persone
auth-ldap-server = Server LDAP
auth-server-type = Tipo di server
auth-server-type-custom = Personalizzato
auth-servers = Server
auth-server-n = Server { $n }
auth-remove-server-n = Rimuovi il server { $n }
auth-remove = Rimuovi
auth-add-server = Aggiungi un server
auth-servers-hint = Quando uno non è raggiungibile, si prova il successivo.
auth-security = Sicurezza
auth-connection-security = Sicurezza della connessione
auth-skip-verify = Non verificare il certificato del server (solo per fare prove)
auth-ca = Autorità di certificazione
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (vuoto: quelle di cui si fida il sistema)
auth-lookup = Ricerca delle persone
auth-anonymously = In forma anonima
auth-reader-account = Account di lettura
auth-reader-password = La sua password
auth-saved-type-to-replace = Salvata: scrivi per sostituirla
auth-people-under = Le persone sono sotto
auth-name-attribute = Attributo del nome
auth-only-matching = Solo le voci che corrispondono a
auth-groups-under = I gruppi sono sotto
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (vuoto: solo memberOf)
auth-their-groups = I loro gruppi
auth-group-filter-hint = { "{" }dn{ "}" } è la voce della persona, { "{" }user{ "}" } il nome con cui ha effettuato l'accesso.
auth-test = Prova
auth-test-hint = Raggiunge ogni server e accede con l'account di lettura. Con un nome e una password, accede anche come quella persona e mostra cosa ne ricaverebbe Hoard. Non viene salvato niente.
auth-test-user = Una persona con cui accedere
auth-test-user-placeholder = Una persona con cui accedere (facoltativo)
auth-test-password = La sua password
auth-testing = Prova in corso…
auth-test-connectivity = Prova la connessione
auth-test-server-ok = raggiunto, e l'account di lettura ha effettuato l'accesso
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }ha effettuato l'accesso (
auth-test-signed-in-after = ).
auth-test-role = Avrebbe il ruolo: { $role }.
auth-test-no-role = Non è in nessuno dei gruppi elencati in Ruoli, quindi verrebbe respinto.
auth-test-groups = Gruppi: { $groups }
auth-test-no-groups = Nessun gruppo trovato.
auth-provider = Provider
auth-redirect = Indirizzo di reindirizzamento
auth-copy = Copia
auth-redirect-hint = Dallo al provider quando crei il client di Hoard. Deriva dall'indirizzo pubblico in Impostazioni → Connessione, o dall'indirizzo che stai usando.
auth-issuer = Issuer
auth-discover = Rileva
auth-client-id = ID client
auth-client-secret = Segreto client
auth-authorization-url = Indirizzo di autorizzazione
auth-token-url = Indirizzo del token
auth-userinfo-url = Indirizzo delle informazioni utente
auth-userinfo-placeholder = (vuoto: è l'ID token a dire chi è)
auth-scopes = Scope
auth-name-from = Nome da
auth-groups-from = Gruppi da
auth-groups-from-placeholder = (nessuno)
auth-button = Pulsante
auth-button-placeholder = il tuo account
auth-accounts = Account
auth-provision = Crea un account Hoard la prima volta che qualcuno accede
auth-provision-on-hint = Altrimenti un amministratore aggiunge prima ogni persona, in Impostazioni → Account.
auth-provision-off-hint = Un amministratore aggiunge prima ogni persona, in Impostazioni → Account, scegliendo come accede.
auth-open-to-anyone = Chiunque abbia un account { $provider } può entrare. Aggiungi qui sotto dei gruppi di visualizzatori, oppure disattiva questa opzione e aggiungi tu le persone.
auth-new-accounts = Nuovi account
auth-roles-from-groups = Ruoli dai gruppi
auth-roles-hint = I gruppi i cui membri ricevono ciascun ruolo, uno per riga, per nome o DN. Vince il più alto che corrisponde, e viene verificato a ogni accesso. Se non ne è elencato nessuno, i nuovi account ricevono il ruolo qui sopra e un amministratore lo cambia in seguito. Con dei gruppi elencati in Può vedere, possono accedere solo i membri di un gruppo elencato.
auth-roles-hint-oauth = I gruppi i cui membri ricevono ciascun ruolo, uno per riga, per nome o DN (o qualunque cosa elenchi l'attestazione dei gruppi). Vince il più alto che corrisponde, e viene verificato a ogni accesso. Se non ne è elencato nessuno, i nuovi account ricevono il ruolo qui sopra e un amministratore lo cambia in seguito. Con dei gruppi elencati in Può vedere, possono accedere solo i membri di un gruppo elencato.
auth-saving = Salvataggio…
auth-save = Salva

## HttpsPanel.vue (Impostazioni → Connessione)

https-load-failed = Impossibile caricare HTTPS: { $error }
https-off-redirecting = HTTPS è disattivato. Ti porto su http://
https-now-on = HTTPS è attivo
https-saved = Salvato. Le nuove connessioni usano il nuovo certificato.
https-now-off = HTTPS è disattivato
https-http3-failed-toast = Impossibile avviare HTTP/3: { $error }
https-copied = Copiato
https-copy-failed = Impossibile copiare: selezionalo e copialo tu
https-mode-off = Disattivato
https-mode-off-hint = Solo http:// semplice.
https-mode-own = Il certificato di Hoard
https-mode-own-hint = Hoard ne crea uno per gli indirizzi di questo server. Installa una volta l'autorità di certificazione di Hoard su ogni dispositivo e i suoi browser se ne fidano.
https-mode-custom = Il mio certificato
https-mode-custom-hint = Uno che hai già, ad es. da tailscale cert o Let's Encrypt. I browser se ne fidano senza installare niente.
https-untrusted-own = Questo browser non se ne fida ancora per { $host }: installa qui sotto l'autorità di certificazione di Hoard. Vedrai un avviso che puoi superare.
https-untrusted-uncovered = Questo browser non se ne fida ancora per { $host }: il certificato non copre questo indirizzo. Vedrai un avviso che puoi superare.
https-untrusted = Questo browser non se ne fida ancora per { $host }. Vedrai un avviso che puoi superare.
https-now = Adesso
https-start-failed = HTTPS è configurato ma non è riuscito ad avviarsi: { $error }
https-on-port = HTTPS è attivo, sulla porta { $port } come http://.
https-using-it = Lo stai usando ora.
https-open = Apri { $url }
https-trusted = Questo browser se ne fida.
https-cert-for-before = Certificato per{ " " }
https-cert-for-after = , valido fino al { $date }.
https-cert-for-after-own = , valido fino al { $date } (si rinnova da sé).
https-not-covered-before = Non copre{ " " }
https-not-covered-after = , l'indirizzo che stai usando.
https-is-off = HTTPS è disattivato: le pagine vengono servite in chiaro, su http://.
https-addresses = Indirizzi
https-addresses-hint = Il certificato copre gli indirizzi trovati da Hoard e quelli che aggiungi tu (un nome assegnato dal router, un nome Tailscale).
https-remove-name = Rimuovi { $name }
https-another-address = Un altro indirizzo
https-another-address-placeholder = Un altro indirizzo, ad es. nas.local
https-add = Aggiungi
https-your-cert = Il tuo certificato
https-saved-cert-before = Salvato:{ " " }
https-saved-cert-after = { " " }da { $issuer }, valido fino al { $date }. Incollane uno nuovo per sostituirlo.
https-pem-before = Il certificato (con la sua catena) e la sua chiave privata, come testo PEM: i file che iniziano con{ " " }
https-pem-after = .
https-certificate = Certificato
https-choose-file = Scegli file…
https-private-key = Chiave privata
https-checked-before = Per{ " " }
https-checked-after = , da { $issuer }, valido fino al { $date }.
https-checked-after-days =
    , da { $issuer }, valido fino al { $date } ({ $days ->
        [one] { $days } giorno
       *[other] { $days } giorni
    }).
https-self-signed = È autofirmato: i browser lo segnaleranno.
https-key-matches = La chiave corrisponde.
https-where-to-get = Dove ottenerne uno
https-get-tailscale-before = attiva HTTPS per la tua tailnet, poi esegui{ " " }
https-get-tailscale-after = { " " }sul server e carica i due file. Durano 90 giorni: eseguilo di nuovo e ricaricali prima della scadenza.
https-get-acme-title = Let's Encrypt o un altro client ACME
https-get-acme-before = (certbot, acme.sh, lego) per un dominio di tua proprietà: carica{ " " }
https-get-acme-and = { " " }e{ " " }
https-get-acme-after = .
https-get-proxy-title = Un reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) che ha il certificato: lascia HTTPS disattivato qui e fai inviare al proxy http:// semplice a Hoard.
https-trying = Prova in corso…
https-apply = Applica
https-apply-hint = Hoard prova prima la nuova configurazione, e mantiene quella che ha se non funziona.
https-trust-title = Fidati di Hoard sui tuoi dispositivi
https-download-ca = Scarica hoard-ca.crt
https-trust-hint = Installa una volta l'autorità di certificazione di Hoard su ogni dispositivo, e i suoi browser si fidano di questo server (e di qualsiasi certificato che Hoard crei per esso in futuro). Può garantire solo per questo Hoard. Impronta digitale (SHA-256):
https-ca-windows = Apri il file → Installa certificato → Utente corrente → Colloca tutti i certificati nel seguente archivio → Sfoglia → Autorità di certificazione radice attendibili → Fine. Riavvia il browser.
https-ca-macos = Apri il file; finisce in Accesso Portachiavi. Fai doppio clic su «{ $subject }» → Fidati → Quando si utilizza questo certificato: Fidati sempre.
https-ca-ios-title = iPhone e iPad
https-ca-ios = Scaricalo in Safari e consentilo. Impostazioni → Profilo scaricato → Installa. Poi Impostazioni → Generali → Info → Impostazioni attendibilità certificati → attiva «{ $subject }».
https-ca-android = Impostazioni → Sicurezza → Altre impostazioni di sicurezza → Crittografia e credenziali → Installa un certificato → Certificato CA, e scegli il file. (I nomi cambiano un po' da telefono a telefono; cerca «certificato» nelle Impostazioni.)
https-ca-linux-1 = Debian e Ubuntu:{ " " }
https-ca-linux-2 = . Fedora e Arch:{ " " }
https-ca-linux-3 = . Chrome su Linux ha un proprio elenco: Impostazioni → Privacy e sicurezza → Sicurezza → Gestisci certificati → Importa.
https-ca-firefox = Firefox ha un proprio elenco: Impostazioni → Privacy e sicurezza → Certificati → Mostra certificati → Autorità → Importa, e spunta «Considera attendibile questa CA per l'identificazione di siti web».
https-ca-cli-title = Lo strumento da riga di comando hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , oppure metti il percorso in{ " " }
https-ca-cli-after = .
https-options = Opzioni
https-redirect = Manda i browser degli altri dispositivi da http:// a https://
https-redirect-hint = Vengono reindirizzati solo i caricamenti delle pagine. Lo strumento da riga di comando hoard, il worker Unreal, gli script e le app di IA continuano a funzionare su http://, e così http://localhost sul server stesso.
https-http3-hint-before = Più veloce su Wi-Fi instabili e sui telefoni. Richiede HTTPS, e la porta UDP { $port } aperta oltre a quella TCP (in Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = In esecuzione.
https-http3-waits = Si avvia quando HTTPS è attivo.
https-http3-failed = Impossibile avviare HTTP/3: { $error }
https-lockout-title = Se resti chiuso fuori
https-lockout-1 = { "" }
https-lockout-2 = { " " }sul server stesso funziona sempre in chiaro, su http://. Oppure disattiva HTTPS dalla riga di comando del server e riavvia Hoard:{ " " }
https-lockout-3 = { " " }(in Docker:{ " " }
https-lockout-4 = , poi{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Impostazioni → Account)

account-role-view = Può vedere
account-role-view-hint = Cercare, vedere in anteprima e scaricare
account-role-edit = Può modificare
account-role-edit-hint = Anche aggiungere, modificare, assegnare tag, raccogliere e archiviare
account-role-admin = Amministratore
account-role-admin-hint = Anche Impostazioni, account e manutenzione, ed eliminare definitivamente
account-source-internal = Una password di Hoard
account-source-ldap = La password di { $directory }
account-source-oauth = Accesso con { $provider }
account-source-word-directory = directory
account-load-failed = Impossibile caricare gli account: { $error }
account-failed = { $what }: { $error }
account-password-failed = Impossibile cambiare la tua password
account-password-changed = Password cambiata. Gli altri tuoi dispositivi sono stati disconnessi.
account-add-failed = Impossibile aggiungere { $name }
account-added = Ora { $name } può accedere.
account-role-failed = Impossibile cambiare cosa può fare { $name }
account-remove-title = Rimuovere { $name }?
account-remove-body = Viene disconnesso ovunque e non può più accedere. Niente di ciò che ha aggiunto alla libreria viene rimosso.
account-remove = Rimuovi
account-remove-failed = Impossibile rimuovere { $name }
account-token-failed = Impossibile creare il token
account-copied = Copiato
account-copy-token-failed = Impossibile copiare: seleziona il token e copialo tu
account-copy-failed = Impossibile copiare: selezionalo e copialo tu
account-revoke-title = Revocare «{ $name }»?
account-revoke-body = Da ora in poi, qualsiasi cosa lo usi ancora viene respinta.
account-revoke = Revoca
account-revoke-failed = Impossibile revocare «{ $name }»
account-disconnect-title = Disconnettere { $app }?
account-disconnect-body = Da ora in poi viene respinto, e deve essere connesso di nuovo per usare Hoard.
account-disconnect = Disconnetti
account-disconnect-failed = Impossibile disconnettere { $app }
account-guest-failed = Impossibile modificarlo
account-never = mai
account-you = Tu
account-sign-out = Esci
account-signed-in-as-before = Accesso effettuato come{ " " }
account-signed-in-as-admin = , che è amministratore.
account-signed-in-as-edit = , che può modificare la libreria.
account-signed-in-as-view = , che può vedere ma non modificare niente.
account-password-at-provider = La tua password è conservata dal provider con cui accedi, quindi va cambiata lì.
account-password-at-directory = La tua password è conservata dalla directory con cui accedi, quindi va cambiata lì.
account-current-password = Password attuale
account-new-password = Nuova password
account-new-password-again = Ripeti la nuova password
account-change-password = Cambia password
account-password-mismatch = Le nuove password non corrispondono.
account-password-rule = Almeno 8 caratteri.
account-people = Persone
account-people-hint = Ogni persona può vedere (cercare, vedere in anteprima e scaricare), anche modificare la libreria (aggiungere, modificare, assegnare tag, raccogliere e archiviare), o essere anche amministratore (Impostazioni, account e manutenzione, ed eliminare definitivamente).
account-you-tag = (tu)
account-signs-in-oauth = Accede con il provider OAuth
account-signs-in-directory = Accede con la directory
account-since = dal { $date }
account-what-can-do = Cosa può fare { $name }
account-name = Nome
account-how-sign-in = Come accede
account-password-label = Password (almeno 8 caratteri)
account-password-placeholder = Password (8+ caratteri)
account-what-they-can-do = Cosa può fare
account-add = Aggiungi
account-ai-apps = Claude e altre app di IA
account-ai-apps-hint = Connetti Claude (o qualsiasi app MCP) e potrà cercare e recuperare asset per te, e descrivere e assegnare tag a ciò che Hoard non ha ancora fatto. La prima volta accede a Hoard; tu lo autorizzi qui nel browser.
account-mcp-address = Indirizzo MCP di Hoard
account-copy = Copia
account-claude-code-1 = esegui{ " " }
account-claude-code-2 = , poi{ " " }
account-claude-code-3 = { " " }per accedere.{ " " }
account-claude-code-4 = { " " }descrive un pacchetto.
account-claude-ai = Impostazioni → Connettori → Aggiungi connettore personalizzato, incolla l'indirizzo, poi Connetti. Questi si connettono dai server di Anthropic, quindi Hoard deve essere raggiungibile da internet tramite HTTPS.
account-claude-ai-local = Impostazioni → Connettori → Aggiungi connettore personalizzato, incolla l'indirizzo, poi Connetti. Questi si connettono dai server di Anthropic, quindi Hoard deve essere raggiungibile da internet tramite HTTPS (questo indirizzo non lo è: imposta l'Indirizzo pubblico in Impostazioni → Connessione).
account-connection-meta = per { $user } dal { $since } · ultimo uso: { $used }
account-no-apps = Ancora nessuna app connessa.
account-tokens = Token di accesso
account-tokens-hint-1 = Per lo strumento da riga di comando{ " " }
account-tokens-hint-2 = , gli script, i client MCP e il worker Unreal: lo inviano come{ " " }
account-tokens-hint-3 = . Un token può fare ciò che scegli, fino a ciò che puoi fare tu, ma mai gestire account o token. Uno che può solo vedere basta per cercare e scaricare, e per sincronizzare un progetto.
account-tokens-admin = Tu vedi i token e le app connesse di tutti; gli altri vedono i propri.
account-made-before = { "" }
account-made-after = : copialo ora. Non verrà mostrato di nuovo.
account-new-token = Il nuovo token
account-cli-1 = Per la CLI:{ " " }
account-cli-2 = , oppure mettilo in{ " " }
account-cli-3 = .
account-done = Fatto
account-token-meta = creato: { $made } · ultimo uso: { $used }
account-token-meta-by = creato: { $made } da { $by } · ultimo uso: { $used }
account-no-tokens = Ancora nessun token.
account-token-name = Cosa lo userà
account-token-name-placeholder = Cosa lo userà, ad es. «Worker Unreal»
account-token-role = Cosa può fare
account-make-token = Crea un token
account-guests = Ospiti
account-guests-allow = Consenti a chiunque raggiunga il server di sfogliare senza accedere
account-guests-hint = Possono cercare, vedere in anteprima e scaricare, ma non aggiungere, modificare o eliminare niente, né vedere le Impostazioni.

## SignIn.vue

signin-create-title = Crea il tuo account
signin-title = Accedi
signin-setup-lede = Questo server non ha ancora un account. Quello che crei qui può fare tutto; più avanti potrai aggiungere account per altre persone da Impostazioni → Account.
signin-overlay-lede = Qui chiunque può sfogliare, ma per modificare serve un account.
signin-with = Accedi con { $provider }
signin-or-password = oppure con nome e password
signin-username = Nome utente
signin-name = Nome
signin-new-password = Nuova password
signin-password = Password
signin-password-rule = Almeno 8 caratteri.
signin-confirm-password = Conferma password
signin-mismatch = Le due password non corrispondono.
signin-setup-code = Codice di configurazione
signin-setup-code-hint-before = Il codice dato a questo server in{ " " }
signin-setup-code-hint-after = .
signin-creating = Creazione…
signin-signing-in = Accesso…
signin-create = Crea account
signin-submit = Accedi
signin-keep-browsing = Continua a sfogliare
signin-directory-hint = Usa il nome e la password di { $directory }.
signin-forgot-1 = Hai dimenticato una password di Hoard? Sul server, esegui{ " " }
signin-forgot-2 = { " " }(in Docker,{ " " }
signin-forgot-3 = ) per averne una nuova.

## SetupWizard.vue (prima visita a un nuovo server)

setup-title = Dove deve stare la tua libreria?
setup-lede = Scegli una cartella sul server in cui Hoard conserverà i tuoi pacchetti, oppure indicagli una cartella di pacchetti che hai già. Più avanti potrai aggiungere altre posizioni, su altri dischi, dalle Impostazioni.
setup-unreachable = Impossibile raggiungere il server: { $error }
setup-try-again = Riprova
setup-loading = Caricamento…
setup-start = Avvia la libreria
setup-ready = La tua libreria è pronta
setup-managed-before = I nuovi pacchetti vanno in{ " " }
setup-managed-after = . Trascina file, cartelle o archivi in un punto qualsiasi della pagina per aggiungere i primi.
setup-watched-before = Hoard sta monitorando{ " " }
setup-watched-after = ; i pacchetti compaiono man mano che li trova. Per poter anche caricare, aggiungi una libreria gestita da Hoard da Impostazioni → Posizioni della libreria.
setup-open = Apri la libreria

## ConnectApp.vue (un'app come Claude chiede di accedere)

connect-title = Connettere { $app }?
connect-lede-1 = { " " }potrà cercare, scaricare, descrivere e assegnare tag a tutto ciò che c'è in questo Hoard, agendo come{ " " }
connect-lede-2 = . Non può gestire account o token di accesso.
connect-back-before = Tornerai a{ " " }
connect-back-after = . Puoi disconnetterlo in qualsiasi momento in Impostazioni → Account.
connect-allow = Consenti
connect-cancel = Annulla
connect-failed = Impossibile connettere questa app
connect-go-library = Vai alla libreria
connect-checking = Verifica…

## InsecureNotice.vue (la riga mostrata sopra http:// semplice)

insecure-text = Questa pagina è in chiaro su http://, quindi alcune cose richiedono HTTPS: il 3D con WebGPU, gli appunti moderni, l'installazione di Hoard come app e i connettori di Claude.
insecure-switch = Passa a https://
insecure-set-up = Configura HTTPS
insecure-dismiss = Non mostrarlo più

## FormatWizard.vue (Impostazioni → File degli store → insegnare a Hoard il file di uno store)

formatwizard-too-big = Questo file è troppo grande per essere letto come file di uno store.
formatwizard-not-json = Questo file non è JSON valido, quindi viene mostrato come testo.
formatwizard-read-failed = Impossibile leggere il file: { $error }
formatwizard-back-title = Tornare al modulo?
formatwizard-back-body = Le modifiche fatte nel TOML vanno perse: il modulo riscrive il formato da ciò che ha.
formatwizard-back-to-form = Torna al modulo
formatwizard-miss-name = Il nome { $file } non è tra i suoi file. Aggiungilo in File.
formatwizard-miss-not-the-stores = Ciò da cui riconosce i file dello store non è in questo file. Cambia Riconosci da.
formatwizard-miss-no-title = Non trova un titolo in questo file. Scegli il titolo.
formatwizard-miss-short-name = non è il suo nome
formatwizard-miss-short-not-the-stores = non è dello store
formatwizard-miss-short-no-title = nessun titolo
formatwizard-saved-packs =
    { $file } salvato. { $packs ->
        [one] { $packs } pacchetto viene riletto.
       *[other] { $packs } pacchetti vengono riletti.
    }
formatwizard-saved = { $file } salvato.
formatwizard-save-failed = Impossibile salvare il formato: { $error }
formatwizard-zip-failed = Impossibile creare lo zip: { $error }
formatwizard-stop-title = Smettere di insegnare questo file?
formatwizard-stop-body = Il formato non viene salvato.
formatwizard-stop = Smetti
formatwizard-label = Procedura guidata dei formati
formatwizard-changing = Modifica di { $name }
formatwizard-starting-from = A partire da { $name }
formatwizard-title = Insegna a Hoard il file di uno store
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }in{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Modifica come TOML
formatwizard-file-reference = Il file, come riferimento.
formatwizard-click-json = Nel file, clicca sul campo { $field }.
formatwizard-click-text = Nel file, clicca sul campo { $field }, oppure selezionalo.
formatwizard-pick-first = Scegli un campo a destra, poi clicca qui sul suo valore.
formatwizard-loading = Caricamento…
formatwizard-the-format = Il formato
formatwizard-syntax-before = La sintassi è in{ " " }
formatwizard-syntax-after = .
formatwizard-store = Store
formatwizard-store-placeholder = Il nome dello store, ad es. Kenney
formatwizard-files = File
formatwizard-recognize = Riconosci da
formatwizard-recognize-json = Una chiave che hanno solo i file dello store
formatwizard-recognize-text = Una riga che hanno solo i file dello store
formatwizard-recognize-hint = Uno per riga; ne basta uno qualsiasi. Serve quando altri file hanno lo stesso nome.
formatwizard-seller = Venditore
formatwizard-seller-placeholder = Quando il file non lo dice
formatwizard-tags = Tag
formatwizard-tags-placeholder = Per tutto ciò che descrive, separati da virgole
formatwizard-fields = Campi
formatwizard-selector = Selettore per { $field }
formatwizard-clear = Cancella { $field }
formatwizard-pick = Scegli
formatwizard-what-it-reads = Cosa legge
formatwizard-read-title = Titolo
formatwizard-read-seller = Venditore
formatwizard-read-license = Licenza
formatwizard-read-url = Pagina dello store
formatwizard-read-kind = Tipo
formatwizard-read-tags = Tag
formatwizard-read-origin = Luogo di creazione
formatwizard-read-size = Dimensione
formatwizard-also = Anche { $format } legge questo file. Questo formato verrebbe provato per primo.
formatwizard-library =
    { $files ->
        [one] { $files } file
       *[other] { $files } file
    } in { $packs ->
        [one] { $packs } pacchetto
       *[other] { $packs } pacchetti
    } { $files ->
        [one] ha
       *[other] hanno
    } un nome che riconosce. Di { $tried } provati, ne legge { $read }.
formatwizard-library-none = Nessun file nella libreria ha un nome che riconosce.
formatwizard-share = Condividilo
formatwizard-share-hint = Uno zip del formato, con questo file come esempio e ciò che se ne legge, organizzato come il repository di Hoard, da inviare perché tutti lo ricevano. Prima togli dall'esempio tutto ciò che non va condiviso.
formatwizard-sample = Esempio da condividere
formatwizard-download = Scarica { $file }
formatwizard-close = Chiudi
formatwizard-use = Usalo qui
formatwizard-cancel = Annulla
