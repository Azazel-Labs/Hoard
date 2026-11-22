## Erori ale serverului

error-account-not-found = contul nu a fost găsit
error-asset-gone = resursa nu a fost găsită
error-asset-not-found = resursa { $id } nu a fost găsită
error-blob-not-found = blobul { $hash } nu a fost găsit
error-collection-not-found = colecția { $id } nu a fost găsită
error-connection-not-found = conexiunea nu a fost găsită
error-derivative-not-found = derivatul { $kind } al { $id } nu a fost găsit
error-file-in-pack-not-found = { $path } nu a fost găsit în pachetul { $id }
error-site-not-a-page = doar paginile web se deschid ca pagini
error-site-link-expired = adresa acestei pagini a expirat; deschideți din nou pagina din Hoard
error-format-not-found = formatul { $file } nu a fost găsit
error-frame-sheet-aseprite-only = doar fișierele Aseprite au o foaie de cadre
error-frame-sheet-frames = are un singur cadru sau prea multe pentru a fi așezate
error-location-not-found = locația { $id } nu a fost găsită
error-maintenance-task-not-found = sarcina de întreținere { $id } nu a fost găsită
error-open-upload-not-found = încărcarea deschisă { $id } nu a fost găsită
error-original-not-found = fișierul original al { $id } nu a fost găsit
error-pack-not-found = pachetul { $id } nu a fost găsit
error-preview-too-big = imaginea este prea mare pentru a-i face o previzualizare ușoară
error-project-not-found = proiectul { $name } nu a fost găsit
error-saved-search-not-found = căutarea salvată { $id } nu a fost găsită
error-sign-in-first = conectați-vă mai întâi (sau trimiteți un token de acces)
error-those-assets-not-found = resursele acelea nu au fost găsite
error-thumbnail-not-found = miniatura pentru { $id } nu a fost găsită
error-token-not-found = tokenul nu a fost găsit
error-upload-not-found = încărcarea { $id } nu a fost găsită
error-pack-needs-name = un pachet are nevoie de un nume
error-no-library = încă nu există o locație a bibliotecii în care să fie stocate fișierele: adăugați una gestionată în Setări → Locații ale bibliotecii
error-upload-failed = încărcarea a eșuat: { $reason }
error-upload-no-files = niciun fișier în încărcare
error-analyze-nothing-to-do = nimic de făcut: rules=false și LLM este oprit
error-too-many-assets = prea multe resurse într-o singură cerere
error-change-not-found = această modificare nu este în istoricul acestui asset
error-bulk-by-look = o căutare după aspect nu poate fi editată deodată: selectează asset-urile
error-too-many-items = prea multe elemente într-o singură cerere
error-not-a-hash = nu este un hash blake3 hexazecimal
error-merge-into-tag = into trebuie să fie o etichetă
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = „{ $text }” nu este o etichetă
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } este deja sub { $tag }, deci { $tag } nu poate fi plasată sub ea
error-nothing-to-merge = nimic de unit
error-selection-size = dați între 1 și { $max } ID-uri de resurse
error-collection-needs-name = o colecție are nevoie de un nume
error-collection-exists = există deja o colecție numită { $name }
error-collection-empty = colecția { $name } este goală
error-conversion-failed = conversia în { $format } nu a reușit: { $reason }
error-max-size = max trebuie să fie o putere a lui doi între 64 și 16384
error-replace-ext = ext trebuie să fie o extensie scurtă de fișier
error-empty-file = fișier gol
error-imitation-silent = Nimic din înregistrare nu iese în evidență față de fundal. Încearcă din nou mai aproape de microfon.
error-replace-duplicate = fișierul convertit dublează unul care există deja în pachet; pachetul a fost unit cu acesta
error-ai-address = Adresa AI trebuie să înceapă cu http:// sau https://
error-hosted-key = Introduceți cheia API pentru { $provider }
error-hosted-address = Adresa serviciului de căutare după imagine trebuie să înceapă cu http:// sau https://
error-local-llm-action = action este list, download sau delete, iar ultimele două au nevoie de un model
error-role = Un rol este view, edit sau admin
error-admin-only = doar un administrator poate face asta
error-view-only = acest cont poate vedea, dar nu poate schimba nimic: un administrator poate schimba asta în Setări → Cont
error-portal-only = conectați-vă la portal pentru a gestiona conturile și tokenurile
error-not-from-portal = această cerere nu a venit de la portalul Hoard
error-account-name = Alegeți un nume de cel mult 64 de caractere
error-password-length = Folosiți o parolă de cel puțin { $min } caractere
error-too-many-codes = Prea multe coduri greșite: așteptați { $seconds } secunde
error-already-set-up = Există deja un cont: conectați-vă
error-too-many-tries = Prea multe încercări greșite: așteptați { $seconds } secunde
error-directory-unavailable = Directorul nu v-a putut verifica parola: { $reason }
error-password-elsewhere = Parola dvs. este păstrată de directorul sau furnizorul prin care vă conectați
error-current-password = Parola actuală nu este corectă
error-account-source = Un cont se conectează prin Hoard (internal), printr-un director (ldap) sau prin OAuth
error-account-exists = Există deja un cont cu acest nume
error-own-role = Rolul dvs. îl poate schimba alt administrator
error-remove-self = Nu vă puteți șterge propriul cont cât timp sunteți conectat la el
error-token-name = Denumiți tokenul după ce îl va folosi, în cel mult 80 de caractere
error-token-role = Un token nu poate face mai mult decât contul care îl creează
error-wrong-setup-code = Acesta nu este codul de configurare dat serverului în HOARD_SETUP_CODE
error-wrong-password = Numele și parola nu se potrivesc
error-folder-missing = { $path } nu există (un disc care nu este montat?)
error-s3-path = o locație S3 arată așa: s3://bucket
error-s3-address = adresa S3 trebuie să înceapă cu http:// sau https://
error-s3-header = un antet S3 suplimentar arată așa: Name=value, iar „{ $header }” nu este unul
error-keychain = nu a putut fi păstrat în depozitul de chei al sistemului: { $reason }
error-not-built = această copie Hoard a fost compilată fără { $feature }
error-openai-auth-unreachable = auth.openai.com nu a putut fi accesat
error-openai-unexpected = răspuns neașteptat de la OpenAI
error-openai-answered = OpenAI a răspuns { $status }: { $reason }
error-chatgpt-id-token-not-jwt = tokenul de ID nu este un JWT
error-chatgpt-id-token-issuer = tokenul de ID este de la { $issuer }, nu de la { $expected }
error-chatgpt-id-token-audience = tokenul de ID este pentru altă aplicație
error-chatgpt-id-token-expired = tokenul de ID a expirat
error-chatgpt-id-token-nonce = tokenul de ID nu aparține acestei conectări
error-chatgpt-no-state = adresa nu are state; copiați adresa întreagă
error-chatgpt-not-started = această conectare nu a fost începută aici sau a fost începută acum mai mult de 15 minute. Începeți-o din nou din Setări.
error-chatgpt-canceled = conectarea a fost anulată
error-chatgpt-refused = ChatGPT a refuzat conectarea: { $reason }
error-chatgpt-no-code = adresa nu are un cod de conectare; copiați adresa întreagă
error-chatgpt-other-registration = ChatGPT a răspuns pentru o altă înregistrare a acestui server; începeți conectarea din nou
error-chatgpt-other-registration-desktop = ChatGPT a răspuns pentru o altă copie a lui Hoard; începeți conectarea din nou
error-chatgpt-not-registered = ChatGPT nu a terminat înregistrarea Hoard; începeți conectarea din nou
error-chatgpt-code-used = codul de conectare a fost deja folosit sau a expirat; conectați-vă din nou
error-chatgpt-refused-code = ChatGPT a refuzat conectarea ({ $code })
error-chatgpt-no-id-token = ChatGPT nu a trimis niciun token de ID
error-chatgpt-plan-not-allowed = Conectat, dar folosirea abonamentului nu a fost permisă. Conectați-vă din nou și permiteți-o sau folosiți în schimb o cheie API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = nu sunteți conectat la ChatGPT; conectați-vă din Setări → Etichetare AI
error-chatgpt-refresh-later = ChatGPT nu a putut reîmprospăta conectarea acum ({ $code }); se va încerca din nou mai târziu
error-chatgpt-signed-out-problem = ChatGPT a deconectat acest server ({ $code }). Conectați-vă din nou pentru a continua etichetarea cu abonamentul dvs.
error-chatgpt-signed-out-problem-desktop = ChatGPT a deconectat Hoard ({ $code }). Conectați-vă din nou pentru a continua etichetarea cu abonamentul dvs.
error-chatgpt-signed-out = ChatGPT a deconectat acest server ({ $code }); conectați-vă din nou din Setări → Etichetare AI
error-chatgpt-signed-out-desktop = ChatGPT a deconectat Hoard ({ $code }); conectați-vă din nou din Setări → Etichetare AI
error-openai-api-unreachable = api.openai.com nu a putut fi accesat
error-openai-broke-off = răspunsul de la OpenAI s-a întrerupt
error-chatgpt-kept-refusing = OpenAI a refuzat în continuare conectarea ChatGPT; conectați-vă din nou din Setări → Etichetare AI
error-chatgpt-request-failed = cererea a eșuat
error-chatgpt-cut-off = răspunsul a fost tăiat ({ $reason })
error-chatgpt-cut-off-no-reason = răspunsul a fost tăiat (fără niciun motiv dat)
error-openai-ended-early = răspunsul de la OpenAI s-a terminat înainte de a fi complet
error-chatgpt-usage-limit = utilizarea abonamentului ChatGPT pentru Hoard s-a epuizat deocamdată. Vedeți sau schimbați limita la { $url }
error-chatgpt-not-eligible = acest cont sau spațiu de lucru ChatGPT nu își poate folosi abonamentul în alte aplicații. Folosiți în schimb o cheie API.
error-chatgpt-usage-unavailable = ChatGPT nu a putut verifica acum utilizarea abonamentului; se va încerca din nou mai târziu
error-chatgpt-unsupported = utilizarea abonamentului ChatGPT nu acceptă { $param } cu acest model
error-chatgpt-unsupported-images = utilizarea abonamentului ChatGPT nu acceptă { $param } cu acest model; alegeți un model care acceptă imagini sau opriți trimiterea miniaturilor
error-chatgpt-unsupported-part = utilizarea abonamentului ChatGPT nu acceptă o parte a cererii cu acest model
error-chatgpt-port = port este cel pe care browserul accesează Hoard
error-chatgpt-paste-address = lipiți adresa întreagă, care începe cu http://127.0.0.1
error-too-many-hashes = cel mult { $max } hashuri odată
error-credits-pack = pachetul { $key }
error-credits-collection = colecția { $key }
error-format-too-long = formatul este prea lung
error-not-started = serverul nu a pornit
error-not-started-desktop = Hoard nu a pornit
error-not-format-file = nu este fișierul unui format: { $name }
error-format-name = numele are nevoie de o literă sau o cifră de la A la Z sau de la 0 la 9
error-format-miss-name = fișierele formatului nu includ { $file }
error-format-miss-store = formatul nu include { $file } pentru cele ale magazinului (vedeți require)
error-format-miss-title = formatul nu găsește niciun titlu în { $file }
error-cant-download-as = fișierele .{ $ext } nu pot fi descărcate ca { $format }
error-only-images-resize = doar imaginile pot fi redimensionate
error-https-no-certificate = Lipiți sau încărcați mai întâi certificatul și cheia lui
error-https-no-cert-found = Nu s-a găsit niciun certificat: lipiți textul care începe cu -----BEGIN CERTIFICATE-----
error-https-no-key-found = Nu s-a găsit nicio cheie privată: lipiți textul care începe cu -----BEGIN PRIVATE KEY-----
error-https-several-keys = Există mai multe chei private: lipiți doar pe cea care se potrivește cu certificatul
error-https-key-password = Cheia este protejată cu o parolă: dați-o fără parolă (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Certificatul nu este PEM valid
error-https-key-not-pem = Cheia privată nu este PEM validă
error-https-key-mismatch = Cheia nu aparține certificatului. Dacă ați dat un lanț, puneți primul certificatul serverului dvs.
error-https-unusable = Certificatul sau cheia nu pot fi folosite
error-https-expired = Acest certificat a expirat pe { $date }
error-https-not-yet-valid = Acest certificat nu este valabil până pe { $date }
error-https-unreadable = Acesta nu este un certificat pe care Hoard îl poate citi
error-https-bad-name = „{ $name }” nu este o adresă pe care o poate conține un certificat
error-https-test-timed-out = timpul conexiunii de test a expirat
error-https-test-failed = conexiunea de test a eșuat
error-https-no-ca = Hoard nu și-a creat încă autoritatea de certificare
error-https-unavailable = HTTPS nu este disponibil pe acest server
error-https-certificate-mode = Un certificat se folosește cu „Certificatul meu”
error-https-test = Certificatul nu a funcționat într-o conexiune de test: { $reason }
error-nothing-to-retry = nu există nimic ce poate fi reîncercat
error-ai-unreachable = { $url } nu a putut fi accesat
error-ai-key-refused = { $url } a refuzat cheia API ({ $status })
error-ai-answered = { $url } a răspuns { $status }: { $reason }
error-ai-not-json = { $url }/models nu a răspuns cu JSON
error-model-server-refused = serverul de modele: { $reason }
error-no-model-server = niciun server de modele nu răspunde la { $url }
error-not-llama-router = { $url } nu este un server llama.cpp în modul router, așa că nu poate descărca modele
error-model-download = { $model } nu a putut fi descărcat: { $reason }
error-model-delete = { $model } nu a putut fi șters: { $reason }
error-unknown-kind = tip de resursă necunoscut { $kind }
error-watched-is-folder = o locație urmărită este un dosar pe acest server
error-cant-reach = { $path } nu poate fi accesat: { $reason }
error-folder-full-path = dați calea completă a dosarului
error-cant-create = { $path } nu poate fi creat
error-cant-write = nu se poate scrie în { $path }
error-cant-read = { $path } nu poate fi citit
error-overlaps-location = se suprapune cu locația { $name } ({ $path })
error-overlaps-data = se suprapune cu dosarul de date al Hoard ({ $path })
error-overlaps-cache = se suprapune cu dosarul cache al Hoard ({ $path })
error-not-watched = { $name } nu este un dosar urmărit
error-watched-no-kinds = un dosar urmărit nu primește fișiere noi, deci nu are tipuri de schimbat
error-keep-catch-all = faceți în schimb altă locație implicită
error-location-holds-files = { $name } încă conține fișiere: mutați-le mai întâi în celelalte locații (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } nu poate fi accesată, așa că Hoard nu poate spune dacă încă mai conține fișiere
error-relocate-running = fișierele sunt deja mutate între locații; încercați din nou după ce se termină
error-watched-not-moved = { $name } este un dosar urmărit: Hoard nu îi mută fișierele
error-no-other-location = nu există altă locație a bibliotecii în care să fie mutate fișierele: adăugați mai întâi una
error-full-path = dați o cale completă
error-pick-folder = alegeți un dosar
error-task-running = „{ $task }” rulează deja
error-job-cant-stop = „{ $job }” nu poate fi oprit după ce a început
error-job-not-found = acea lucrare nu mai așteaptă și nici nu mai rulează
error-task-cant-stop = „{ $task }” nu poate fi oprit la jumătate
error-task-not-running = „{ $task }” nu rulează
error-token-account-gone = contul acestui token nu mai există, așa că nu are favorite sau căutări
error-favorites-too-many = cel mult 10.000 odată
error-saved-search-name = o căutare salvată are nevoie de un nume de cel mult 80 de caractere
error-saved-search-too-long = căutarea este prea lungă pentru a fi salvată
error-hoard-seen = comoara poate înregistra cel mult 32 de tipuri de resurse, fiecare numit cu litere mici
error-texture-pattern-map = Șablonul pentru numele texturilor are nevoie de { "{" }map{ "}" }, altfel toate hărțile unui material ar primi același nume
error-texture-pattern-fields = Șablonul pentru numele texturilor poate folosi { "{" }name{ "}" }, { "{" }map{ "}" } și { "{" }size{ "}" }
error-texture-name-chars = Numele texturilor nu pot conține < > : " / \ | ? sau *
error-texture-map-type = Nu există niciun tip de hartă numit { $role }
server-role-base-color = culoare
server-role-normal = normale
server-role-normal-dx = normale (DirectX)
server-role-roughness = rugozitate
server-role-gloss = luciu
server-role-metalness = metalicitate
server-role-ao = ocluzie ambientală
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = înălțime
server-role-bump = bump
server-role-cavity = cavitate
server-role-specular = specularitate
server-role-opacity = opacitate
server-role-emissive = emisie
server-role-translucency = translucență
error-texture-suffix-missing = Dați un sufix hărților de { $map }
error-texture-suffix-chars = Sufixul pentru hărțile de { $map } nu poate conține < > : " / \ | ? sau *
error-texture-ending-missing = Fiecare terminație proprie are nevoie de terminația de căutat
error-texture-ending-chars = Terminațiile proprii nu pot conține < > : " / \ | ? sau *
error-texture-ending-twice = { $ending } apare de două ori în terminațiile dvs.
error-oauth-no-client-id = Acest link nu are client_id.
error-oauth-unknown-app = Această aplicație nu este (mai) înregistrată aici. Începeți conectarea din nou din aplicație.
error-oauth-redirect = Aplicația a cerut să fie trimisă înapoi la o adresă pe care nu a înregistrat-o.
error-oauth-response-type = Hoard oferă doar coduri de conectare (response_type=code).
error-oauth-pkce = Aplicația trebuie să folosească PKCE cu S256 pentru a se conecta la Hoard.
error-light-preview-images = doar imaginile au o previzualizare ușoară
error-no-light-preview = nicio previzualizare ușoară: { $reason }
error-no-frame-sheet = nicio foaie de cadre: { $reason }
error-thumbnail-source = sursă de miniatură necunoscută: { $source }
error-thumbnail-too-big = imaginea este prea mare pentru o miniatură
error-thumbnail-models-only = doar modelele primesc o miniatură desenată de un browser
error-not-a-picture = nu este o imagine pe care Hoard o poate citi: { $reason }
error-project-name = un proiect are nevoie de un nume de cel mult 200 de caractere
error-project-too-big = cel mult un milion de resurse într-un proiect
error-project-too-many-at-once = cel mult 100.000 odată
error-server-name = Alegeți un nume de cel mult { $max } caractere, pe un singur rând
error-tool-setting = { $tool }: alegeți oprit, automat sau dați o cale
error-ai-model = Alegeți modelul pe care trebuie să-l folosească serviciul AI
error-public-address = Adresa publică trebuie să înceapă cu http:// sau https://
error-update-hook-address = Adresa apelată pentru actualizări trebuie să înceapă cu http:// sau https://
error-update-hook-auth = Antetul Authorization nu poate conține rânduri noi sau alte caractere de control
error-no-soundfont = Nu există niciun SoundFont la { $path } pe server
error-no-soundfont-desktop = Nu există niciun SoundFont la { $path }
error-soundfont-bad = { $name } nu e un SoundFont cu care MIDI poate cânta: { $reason }
error-workers = Procese de fundal: între 1 și { $max }
error-ai-batch = Resurse per cerere AI: între 1 și 200
error-conversion-cache = Descărcări convertite: cel puțin 100 MB
error-backup-keep = Copii de rezervă programate: păstrează între 1 și { $max }
error-backup-copy-to = Copii de rezervă programate: { $path } nu este un dosar pe server (dă calea completă)
error-conversion-keep-days = Reguli de conversie: originalele se păstrează între 1 și { $max } de zile
error-conversion-target = Reguli de conversie: Hoard nu poate converti fișiere în { $format }
error-conversion-no-source = Reguli de conversie: fiecare regulă are nevoie de cel puțin un format de convertit
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Reguli de conversie: fișierele .{ $ext } nu pot fi convertite în { $format }
error-conversion-twice = Reguli de conversie: .{ $ext } apare în două reguli
error-conversion-cant-undo = Acest fișier nu a fost convertit de o regulă sau originalul său a fost deja șters.
error-conversion-path-taken = Acum există alt fișier la { $path }, așa că originalul nu se poate întoarce acolo.
error-limits-zero = Limitele nu pot fi zero
error-settings-object = se aștepta un obiect JSON de setări
error-bad-setting = setare greșită: { $reason }
error-auth-default-role = Rolul implicit este view, edit sau admin
error-auth-session-hours = Sesiunile durează între o oră și un an
error-auth-settings = setări de autentificare greșite: { $reason }
error-ldap-settings = setări LDAP greșite: { $reason }
error-ldap-no-server = Adăugați adresa serverului de director
error-ldap-base-dn = Spuneți unde se află intrările persoanelor (base DN)
error-ldap-username-attribute = Spuneți ce atribut conține numele cu care se conectează persoanele
error-ldap-bind-dn = Dați contul care caută persoanele sau căutați-le anonim
error-ldap-ca-cert = Autoritatea de certificare nu este un certificat PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Adresa unui server de director este goală
error-ldap-server-form = { $server }: adresa unui server de director este host:port, ldap://… sau ldaps://…
error-ldap-not-starttls = { $server }: acesta este LDAPS, nu StartTLS
error-ldap-no-certificate = nu conține niciun certificat
error-ldap-password-again = Dați din nou parola contului care caută persoanele: cea salvată este trimisă doar serverelor pentru care a fost dată
error-ldap-unreachable = niciun server de director nu a putut fi accesat ({ $reasons })
error-ldap-no-anonymous = directorul nu permite căutarea anonimă a persoanelor ({ $reason })
error-ldap-bind-failed = contul care caută persoanele nu s-a putut conecta ({ $reason })
error-ldap-search = se caută { $name } în { $base }
error-ldap-group-search = se caută grupurile lui { $name } în { $base }
error-ldap-several =
    { $count ->
        [one] o intrare din director se potrivește cu { $name }: restrângeți filtrul de utilizatori
        [few] { $count } intrări din director se potrivesc cu { $name }: restrângeți filtrul de utilizatori
       *[other] { $count } de intrări din director se potrivesc cu { $name }: restrângeți filtrul de utilizatori
    }
error-ldap-password-check = directorul nu a vrut să verifice parola lui { $name } ({ $reason })
error-ldap-wrong-password = Directorul nu îl cunoaște pe { $name } cu această parolă
error-oauth-client-id = Dați ID-ul de client pe care furnizorul l-a creat pentru Hoard
error-oauth-addresses = Adresele de autorizare și de token trebuie să înceapă cu https://
error-oauth-userinfo-address = Adresa de informații despre utilizator trebuie să înceapă cu https://
error-oauth-username-claim = Spuneți ce claim conține numele persoanei
error-oauth-secret-again = Dați din nou secretul clientului: cel salvat este trimis doar adresei de token pentru care a fost dat
error-oauth-issuer = Emitentul (issuer) trebuie să înceapă cu https://
error-oauth-discovery-unreachable = { $url } nu a putut fi accesat: { $reason }
error-oauth-discovery-status = { $url } a răspuns { $status }
error-oauth-discovery-bad = { $url } nu este un document de descoperire
error-oauth-discovery-incomplete = { $url } nu indică adresele de autorizare și de token
error-signin-no-group = { $name } nu face parte dintr-un grup care poate folosi Hoard
error-signin-other-source = Există deja un cont Hoard numit { $name } care se conectează în alt mod
error-signin-other-subject = Există deja un cont Hoard numit { $name } pentru altcineva la furnizor
error-signin-no-account = { $name } nu are încă un cont Hoard: un administrator îl poate adăuga în Setări → Cont
error-signin-bad-name = Acest nume nu poate fi un cont Hoard (cel mult 64 de caractere)
error-oauth-off = Conectarea cu OAuth nu este activată
error-oauth-bad-authorization-address = adresa de autorizare nu este validă
error-oauth-refused = Furnizorul nu v-a conectat: { $reason }
error-oauth-no-code = Furnizorul v-a trimis înapoi fără o conectare
error-oauth-expired = Conectarea a durat prea mult sau a fost începută în altă parte: încercați din nou
error-oauth-no-claim = Furnizorul nu a spus cine sunteți (niciun claim { $claim })
error-oauth-no-sub = Furnizorul nu a spus cine sunteți (niciun claim sub sau id)
error-oauth-token-unreachable = adresa de token a furnizorului nu a putut fi accesată
error-oauth-token-refused = Furnizorul nu a vrut să finalizeze conectarea: { $reason }
error-oauth-no-access-token = furnizorul nu a trimis niciun token de acces
error-oauth-userinfo-unreachable = adresa de informații despre utilizator a furnizorului nu a putut fi accesată
error-oauth-userinfo-refused = Furnizorul nu a vrut să spună cine s-a conectat ({ $status })
error-oauth-userinfo-not-json = informațiile despre utilizator ale furnizorului nu sunt JSON
error-format-no-name = name este gol
error-format-no-files = files este gol: spuneți ce fișiere citește formatul, de exemplu ["*.json"]
error-format-empty-selector = gol
error-format-unopened-bracket = o paranteză de închidere fără una de deschidere
error-format-unclosed-bracket = o paranteză nu este închisă
error-format-empty-step-dots = un pas gol (două puncte la rând sau un punct la unul dintre capete)
error-format-unclosed-except = !( nu este închis
error-format-unclosed-filter = [ nu este închis
error-format-after-filters = { $rest } după cheia și filtrele lui { $step }
error-format-except-needs-glob = !(…) vine după o cheie care conține *
error-format-except-not-glob = !(…) vine după o cheie care conține *, nu după { $key }
error-format-empty-step = un pas gol
error-format-not-a-filter = [{ $filter }] nu este un filtru: folosiți [], [key=value|other], [array], [string], [number], [bool] sau [object]
error-format-filter-needs-value = [{ $filter }] are nevoie de o cheie și o valoare
error-store-search-empty = nimic de căutat; tastați numele pachetului
error-update-hook-not-set = setați mai întâi o adresă de apelat
error-upload-stalled = încărcarea { $path } s-a blocat; trimiteți din nou partea respectivă
error-upload-busy = { $path } este deja în curs de scriere
error-upload-incomplete =
    { $count ->
        [one] un fișier este incomplet: { $files }
        [few] { $count } fișiere sunt incomplete, de exemplu { $files }
       *[other] { $count } de fișiere sunt incomplete, de exemplu { $files }
    }
error-upload-no-files-listed = niciun fișier
error-upload-not-part = { $path } nu face parte din încărcarea { $id } sau încărcarea este închisă
error-bad-path = cale greșită
error-upload-part-offset = { $offset } nu este locul unde începe o parte a { $path }
error-upload-interrupted = încărcare întreruptă: { $reason }
error-upload-part-long = partea { $part } a { $path } este mai lungă de { $len } octeți
error-upload-part-short = partea { $part } a { $path }: s-au primit { $got } octeți, se așteptau { $len }
error-upload-file-long = { $path } este mai mare decât s-a declarat
error-upload-file-short = { $path }: s-au primit { $got } octeți, se așteptau { $len }
error-upload-complete = încărcarea este deja completă
error-already-imported = deja importat ca „{ $name }”

## Texte ale serverului

server-store-place-readme = README
server-store-place-folders = dosare
server-store-why-linked = Apare ca link în pachet ({ $place })
server-store-why-same-title = Același titlu ca în pachet ({ $place })
server-store-why-by = De { $author }, numit în pachet ({ $place })
server-store-why-meshes = Cam tot atâtea mesh-uri ca pachetul ({ $count })
server-store-why-textures = Cam tot atâtea texturi ca pachetul ({ $count })
server-chatgpt-revoke-failed = Deconectat aici, dar OpenAI nu a putut fi anunțat. Pentru siguranță, deconectați Hoard în ChatGPT, din Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Conectat la ChatGPT
server-chatgpt-signed-in = Hoard este conectat. Etichetarea AI vă poate folosi acum abonamentul ChatGPT.
server-chatgpt-signed-in-as = Hoard este conectat ca { $email }. Etichetarea AI vă poate folosi acum abonamentul ChatGPT.
server-chatgpt-not-finished-title = Conectarea nu s-a încheiat
server-chatgpt-other-computer = Dacă ați început conectarea din Hoard pe alt computer, copiați adresa acestei pagini din bara de adrese și lipiți-o acolo, în Setările Hoard.
server-back-to-settings = Înapoi la Setări
server-issues-pack = Pachete care nu au putut fi adăugate
server-issues-pack-blurb = Ștergeți pachetul și adăugați-l din nou sau deschideți-l pentru a vedea ce conține.
server-issues-damaged = Fișiere stocate deteriorate sau dispărute
server-issues-damaged-blurb = Găsite de „Verificare a fișierelor pentru deteriorări”. Restaurați-le dintr-o copie de rezervă sau adăugați din nou pachetul lor, apoi rulați din nou „Verificare a fișierelor pentru deteriorări”.
server-issues-folder = Dosare urmărite care nu pot fi citite
server-issues-folder-blurb = Pachetele lor rămân așa cum sunt până când dosarul poate fi citit din nou.
server-issues-missing = Pachete urmărite al căror dosar a dispărut
server-issues-missing-blurb = Fiecare pachet este păstrat 3 zile, în caz că dosarul său este doar offline, apoi este eliminat.
server-issues-job = Sarcini de fundal care au eșuat
server-issues-job-blurb = Reîncercarea rulează sarcina din nou de la început.
server-issues-file = Fișiere care nu au putut fi deschise
server-issues-file-blurb = Hoard nu le-a putut deschide sau stoca. Un pachet urmărit le încearcă din nou la următoarea scanare; un fișier aflat deja în bibliotecă și-a păstrat ce avea înainte.
server-issues-unpack = Arhive care nu au putut fi dezarhivate
server-issues-unpack-blurb = Fiecare este păstrată în pachetul ei ca fișier simplu, deci nimic din interiorul ei nu este în bibliotecă.
server-issues-read = Fișiere pe care Hoard nu le-a putut înțelege
server-issues-read-blurb = Sunt în bibliotecă, dar fără detaliile, etichetele sau miniatura pe care le aduce citirea lor.
server-issues-render = Modele la care randarea miniaturilor a eșuat
server-issues-render-blurb = Nu au miniatură. Deschiderea unuia în vizualizator poate funcționa totuși și poate salva una.
server-issues-convert = Modele pe care Blender nu le-a putut converti
server-issues-convert-blurb = Nu au o copie GLB, deci nu pot fi afișate în 3D.
server-issues-unreal = Pachete Unreal care nu au putut fi convertite
server-issues-unreal-blurb = Pagina Unreal a pachetului conține restul pachetelor sale Unreal.
server-issues-reformat = Fișiere pe care o regulă de conversie nu le-a putut converti
server-issues-reformat-blurb = Rămân așa cum erau.
server-issues-materials = Pachete ale căror materiale nu au putut fi deduse
server-issues-materials-blurb = Toate texturile lor sunt acolo; doar că nu sunt grupate în materiale.
server-issues-material-thumb = Materiale fără miniatură
server-issues-material-thumb-blurb = Materialul a fost găsit, dar previzualizarea lui nu a putut fi desenată.
server-issues-store-files = Fișiere de magazin care nu au putut fi citite
server-issues-store-files-blurb = Fișiere de la magazinul din care provine un pachet, cum ar fi un JSON Megascans, care îl denumesc și îl etichetează.
server-issues-scan-again = Scanați din nou
server-issues-retry = Reîncercați
server-issues-reanalyze = Reanalizați pachetul
server-issues-convert-again = Convertiți din nou
server-issues-look-again = Căutați din nou
server-issues-read-again = Citiți din nou
server-issues-blob-missing = Fișierul stocat a dispărut.
server-issues-blob-damaged = Fișierul stocat nu se potrivește cu ce a fost stocat: este deteriorat.
server-issues-folder-gone = Dosarul „{ $folder }” nu a fost găsit.
server-job-a-pack = un pachet
server-job-ingest = Se preia { $pack }
server-job-retag = Se reetichetează { $pack }
server-job-llm-tag = Se etichetează { $pack } cu AI
server-job-convert = Se convertește { $pack } cu Blender
server-job-unreal = Se convertesc pachetele Unreal din { $pack }
server-job-reformat = Se convertesc fișierele din { $pack } după regulile de conversie
server-job-scan = Se scanează un dosar urmărit
server-job-relocate = Se mută fișiere între locațiile bibliotecii
server-job-looks = Se determină culorile și aspectul
server-job-sounds = Se ascultă sunetele
server-job-embed = Se învață ce arată imaginile
server-job-hear = Se învață cum sună sunetele
server-job-materials = Se caută materialele din { $pack }
server-job-read-files = Se citesc fișierele din { $pack }
server-job-store-lookup = Se caută o pagină de magazin
server-maintenance-backup = Copie de rezervă a bazei de date
server-maintenance-backup-blurb = O copie consecventă a bazei de date (pachete, etichete, note, colecții și Setări) în dosarul de copii de rezervă de lângă ea. Fișierele bibliotecii nu sunt incluse: ele nu se mai schimbă după ce sunt stocate, așa că orice sincronizare de fișiere le salvează.
server-maintenance-scrub = Verificare a fișierelor pentru deteriorări
server-maintenance-scrub-blurb = Citește din nou fiecare fișier stocat și îl compară cu amprenta sa, pentru a găsi fișierele care s-au degradat sau au dispărut de pe un disc sau NAS. Durează ceva timp la o bibliotecă mare.
server-maintenance-reanalyze = Reanalizare a tuturor pachetelor
server-maintenance-reanalyze-blurb = Citește din nou fiecare fișier cu această versiune Hoard: miniaturile, previzualizările, detaliile și etichetele din reguli sunt refăcute. Modificările dvs. rămân; AI-ul nu este întrebat din nou.
server-maintenance-clear-cache = Golire a conversiilor din cache
server-maintenance-clear-cache-blurb = Elimină fișierele păstrate din conversiile „Descărcați ca” (sunt refăcute la următoarea cerere) și ce a lăsat în urmă munca întreruptă.
server-maintenance-shrink-thumbs = Micșorare a miniaturilor
server-maintenance-shrink-thumbs-blurb = Stochează miniaturile și vederile modelelor păstrate ca PNG așa cum cere Setări → Performanță → Miniaturi, ceea ce de obicei le face de câteva ori mai mici. Cele cu transparență și pixel art rămân PNG.
server-maintenance-avif-thumbs = Convertește miniaturile în AVIF
server-maintenance-avif-thumbs-blurb = Convertește acum toate miniaturile, în loc de fiecare la prima afișare, când miniaturile sunt trimise ca AVIF.
server-maintenance-gc = Ștergere a fișierelor nefolosite
server-maintenance-gc-blurb = Șterge fișierele stocate spre care nu mai indică nimic, cum ar fi fișierele pachetelor șterse definitiv.
server-update-no-address = nu este setată nicio adresă de apelat
server-update-no-answer = niciun răspuns în 30 de secunde

## Setări: Aspect

appearance-language = Limbă
appearance-language-auto = A browserului ({ $name })
appearance-scheme = Luminos sau întunecat
appearance-scheme-system = Sistem
appearance-size-section = Text și spațiere
appearance-text-size = Mărimea textului
appearance-text-small = Mică
appearance-text-normal = Normală
appearance-text-large = Mare
appearance-density = Spațiere
appearance-density-comfortable = Aerisită
appearance-density-compact = Compactă
appearance-tiles = Dale
appearance-tiles-what = Ce arată fiecare dală din bibliotecă pe lângă imagine.
appearance-tile-names = Nume
appearance-tile-details = Tip și mărime (pixeli, durată, triunghiuri)
appearance-tile-badges = Tipul fișierului pe imagine
appearance-tile-motion = Redă animațiile cât timp cursorul e deasupra lor
appearance-top-bar = Bara de sus
appearance-top-bar-what = Ascunde ce nu folosești. / duce în continuare la caseta de căutare, iar { $keys } deschide în continuare paleta de comenzi.
appearance-top-activity = Lucru în fundal
appearance-top-theme = Comutator luminos/întunecat
appearance-top-mic = Microfon în caseta de căutare, pentru a găsi sunete făcând unul
appearance-top-slash = Indiciul / din caseta de căutare
appearance-top-palette = Butonul paletei de comenzi ({ $keys })

## Rădăcină

root-unreachable = Serverul Hoard nu a putut fi accesat: { $error }
root-unreachable-desktop = Hoard nu răspunde: { $error }
root-desktop-reopen = Această fereastră a fost deconectată. Închide Hoard și deschide-l din nou pentru a continua.
root-try-again = Încercați din nou
root-lazy-connect = conexiunea aplicației
window-minimize = Minimizare
window-maximize = Maximizare
window-restore = Restabilire
window-close = Închidere

## Aplicația: biblioteca, bara de sus și filtrele

app-lazy-settings = setările
app-lazy-archive = arhiva
app-lazy-shortcuts = comenzile rapide
app-lazy-setup = configurarea
app-theme-system = Ca tema sistemului
app-theme-light = Temă luminoasă
app-theme-dark = Temă întunecată
app-color-red = Roșu
app-color-orange = Portocaliu
app-color-yellow = Galben
app-color-green = Verde
app-color-teal = Turcoaz
app-color-blue = Albastru
app-color-purple = Mov
app-color-pink = Roz
app-color-brown = Maro
app-color-sand = Nisipiu
app-color-gray = Gri
app-color-slate = Ardezie
app-color-black = Negru
app-color-white = Alb
app-color-swatch = { $color }: resurse cu mult { $lower }
app-color-any = Orice culoare
app-color-pick = Alegeți orice culoare
app-color = Culoare
app-music = Muzică
app-tempo = BPM
app-tempo-min = de la
app-tempo-max = până la
app-tempo-min-title = Muzică în acest tempo sau mai rapidă, în bătăi pe minut (contează și jumătatea sau dublul)
app-tempo-max-title = Muzică în acest tempo sau mai lentă, în bătăi pe minut
app-key = Tonalitate
app-key-any = Orice tonalitate
app-key-major = Major
app-key-minor = Minor
app-key-mix = Și tonalitățile care se mixează cu ea
app-key-mix-title = Relativa ei majoră sau minoră și tonalitățile aflate la o cvintă deasupra și dedesubt, pe care DJ-ii le mixează cu ea
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Scrie zgomotul pe care îl face un sunet așa cum l-ai silabisi: sunetele asemănătoare apar primele
app-imitation-record = Fă zgomotul în microfon: sunetele asemănătoare apar primele
app-imitation-stop = Oprește înregistrarea
app-imitation-heard = Zgomotul tău
app-imitation-mic-failed = Nu s-a putut înregistra de la microfon: { $error }
app-retry = Reîncercați
app-undo = Anulați
app-search-failed = Căutarea a eșuat: { $error }
app-unreachable = Serverul nu a putut fi accesat: { $error }
app-unreachable-desktop = Hoard nu răspunde: { $error }
app-forget-project-title = Uitați { $name }?
app-forget-project-body =
    { $count ->
        [one] Hoard nu îl mai afișează acolo unde este folosită { $count } resursă a sa, până când următorul hoard sync îl anunță din nou. Nimic din proiect nu se schimbă.
        [few] Hoard nu îl mai afișează acolo unde sunt folosite cele { $count } resurse ale sale, până când următorul hoard sync îl anunță din nou. Nimic din proiect nu se schimbă.
       *[other] Hoard nu îl mai afișează acolo unde sunt folosite cele { $count } de resurse ale sale, până când următorul hoard sync îl anunță din nou. Nimic din proiect nu se schimbă.
    }
app-forget-project-confirm = Uitați-l
app-forget-project-failed = { $name } nu a putut fi uitat: { $error }
app-forget-project-label = Uitați proiectul { $name }
app-forget-project = Uitați acest proiect
app-save-name-like = ca { $name }
app-save-name-favorites = favorite
app-search-saved = „{ $name }” a fost salvată: o găsiți în Filtre, la Ale dvs.
app-search-updated = „{ $name }” a fost actualizată
app-rank-alike = Cele mai asemănătoare
app-rank-color = Cel mai mult din culoare
app-rank-match = Cea mai bună potrivire
app-rank-imitation = Cele mai asemănătoare
app-sort = Sortare
app-sort-name = Nume
app-sort-created = Data creării
app-sort-updated = Data actualizării
app-sort-size = Dimensiune
app-sort-rating = Evaluare
app-sort-newest-first = Cele mai noi primele
app-sort-oldest-first = Cele mai vechi primele
app-sort-a-to-z = De la A la Z
app-sort-z-to-a = De la Z la A
app-sort-largest-first = Cele mai mari primele
app-sort-smallest-first = Cele mai mici primele
app-sort-best-rated-first = Cele mai bine evaluate primele
app-sort-lowest-rated-first = Cele mai slab evaluate primele
app-collection-added =
    { $count ->
        [one] S-a adăugat { $count } resursă în { $name }
        [few] S-au adăugat { $count } resurse în { $name }
       *[other] S-au adăugat { $count } de resurse în { $name }
    }
app-collection-add-failed = Nu s-a putut adăuga în colecție: { $error }
app-collection-removed =
    { $count ->
        [one] S-a eliminat { $count } resursă din { $name }
        [few] S-au eliminat { $count } resurse din { $name }
       *[other] S-au eliminat { $count } de resurse din { $name }
    }
app-collection-remove-failed = Nu s-a putut elimina din colecție: { $error }
app-ns-subject = Subiect
app-ns-style = Stil
app-ns-theme = Temă
app-ns-material = Material
app-ns-map = Hartă PBR
app-ns-color = Culoare
app-ns-mood = Atmosferă
app-ns-genre = Gen
app-ns-license = Licență
app-ns-res = Rezoluție
app-ns-duration = Durată
app-ns-poly = Număr de poligoane
app-ns-format = Format
app-ns-weight = Greutate
app-ns-tech = Tehnic
app-ns-blender = Versiune Blender
app-ns-depth = Adâncime de biți
app-ns-ucs = Categorie de sunet (UCS)
app-ns-bpm = Tempo
app-ns-root = Notă fundamentală
app-ns-level = Intensitate sonoră
app-ns-ue = Clasă Unreal
app-ns-engine = Motor
app-ns-extracted = Sursă recuperată
app-ns-script = Sistem de scriere
app-bulk-failed =
    { $count ->
        [one] Resursa nu a putut fi modificată: { $error }
        [few] Cele { $count } resurse nu au putut fi modificate: { $error }
       *[other] Cele { $count } de resurse nu au putut fi modificate: { $error }
    }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] S-a copiat { $count } etichetă
        [few] S-au copiat { $count } etichete
       *[other] S-au copiat { $count } de etichete
    }
app-tags-copy-none = { $name } nu are etichete de copiat
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Nicio etichetă copiată. Apasă C pe o resursă ca să-i copiezi etichetele.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [one] Are deja aceste etichete
       *[other] Toate cele { $count } au deja aceste etichete
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [one]
            { $count ->
                [one] S-a adăugat { $count } etichetă
                [few] S-au adăugat { $count } etichete
               *[other] S-au adăugat { $count } de etichete
            }
       *[other]
            { $count ->
                [one] S-a adăugat { $count } etichetă la { $assets } resurse
                [few] S-au adăugat { $count } etichete la { $assets } resurse
               *[other] S-au adăugat { $count } de etichete la { $assets } resurse
            }
    }
app-bulk-untagged =
    { $count ->
        [one] S-a scos { $tag } de pe { $count } resursă
        [few] S-a scos { $tag } de pe { $count } resurse
       *[other] S-a scos { $tag } de pe { $count } de resurse
    }
app-bulk-tagged =
    { $count ->
        [one] S-a etichetat { $count } resursă cu { $tag }
        [few] S-au etichetat { $count } resurse cu { $tag }
       *[other] S-au etichetat { $count } de resurse cu { $tag }
    }
app-bulk-review = Marcați verificate
app-bulk-review-title = Marcați resursele selectate ca verificate: regulile și AI nu le mai ating etichetele după aceea
app-bulk-unreview = Marcați neverificate
app-bulk-unreview-title = Scoateți marcajul de verificat de pe resursele selectate
app-bulk-reviewed =
    { $count ->
        [one] A fost marcată ca verificată { $count } resursă
        [few] Au fost marcate ca verificate { $count } resurse
       *[other] Au fost marcate ca verificate { $count } de resurse
    }
app-bulk-unreviewed =
    { $count ->
        [one] S-a scos marcajul de verificat de pe { $count } resursă
        [few] S-a scos marcajul de verificat de pe { $count } resurse
       *[other] S-a scos marcajul de verificat de pe { $count } de resurse
    }
app-bulk-accept = Păstrați presupunerile
app-bulk-accept-title = Păstrați ca ale dvs. toate etichetele puse de reguli și de AI pe resursele selectate
app-bulk-accepted =
    { $count ->
        [one] S-a păstrat presupunerea pentru { $count } resursă
        [few] S-au păstrat presupunerile pentru { $count } resurse
       *[other] S-au păstrat presupunerile pentru { $count } de resurse
    }
app-credits-failed = Creditele nu au putut fi create: { $error }
app-archived-assets =
    { $count ->
        [one] S-a arhivat { $count } resursă
        [few] S-au arhivat { $count } resurse
       *[other] S-au arhivat { $count } de resurse
    }
app-archived-one = S-a arhivat { $name }
app-open-failed = Resursa nu a putut fi deschisă: { $error }
app-unnamed-pack = pachet
app-unnamed-collection = colecție
app-unnamed-asset = resursă
app-home-version = Hoard { $version } — înapoi la bibliotecă
app-home = Înapoi la bibliotecă
app-search-label = Căutați resurse
app-search-placeholder = Căutați nume, căi, etichete
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = stâncă or piatră: una dintre ele · -mușchi: omiteți · "mușchi uscat": aceste cuvinte împreună · pack:pădure: în numele unui pachet
app-alt-leave-out = Alt-clic pentru a-l omite
app-views = Vizualizări
app-library = Bibliotecă
app-packs = Pachete
app-packs-working = { $count } în lucru
app-update-title = A apărut Hoard { $version } (acest server rulează { $current }). Descărcați noua imagine sau noul binar și reporniți; baza de date se actualizează singură.
app-update = Actualizare
app-theme = Temă
app-sign-in = Conectare
app-archive-title = Arhivă: { $contents }
app-archive = Arhivă
app-settings = Setări
app-palette-title = Paleta de comenzi ({ $keys })
app-import-folder = Importați un dosar
app-add-files = Adăugați fișiere
app-add-files-short = Adăugați
app-split-label = Cum se importă acest dosar
app-split-before = { "" }
app-split-after =
    { $count ->
        [one] { " " }conține { $count } dosar sau arhivă.
        [few] { " " }conține { $count } dosare sau arhive.
       *[other] { " " }conține { $count } de dosare sau arhive.
    }
app-split-each = Importați fiecare ca pachet separat ({ $count })
app-split-one = Importați ca un singur pachet
app-cancel = Anulați
app-filters = Filtre
app-done = Gata
app-yours = Ale dvs.
app-favorites = Favorite
app-needs-review = De verificat
app-needs-review-title = Doar resursele pe care nu le-a verificat încă nimeni (merge împreună cu celelalte filtre)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Evaluate
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } stea sau mai multe
        [few] { $count } stele sau mai multe
       *[other] { $count } de stele sau mai multe
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Faceți din nou clic pentru a afișa toate evaluările
app-forget-search-label = Uitați căutarea salvată { $name }
app-forget-search = Uitați această căutare
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Deschideți biblioteca cu această căutare
app-default-search-on-title = Biblioteca se deschide cu această căutare. Faceți clic pentru a opri.
app-default-search-label = Deschideți biblioteca cu { $name }
app-rename-search = Redenumiți
app-rename-search-label = Nume nou pentru { $name }
app-projects-title = Proiectele care rulează hoard sync de aici, cu ce avea ultima lor sincronizare
app-projects = Proiecte
app-project-synced = Sincronizat { $when }
app-project-synced-by = Sincronizat { $when } de { $who }
app-collections = Colecții
app-no-filters = Niciun filtru pentru această căutare.
app-kind = Tip
app-facets-more = încă { $count }
app-facets-fewer = mai puține
app-searching = Se caută…
app-searching-live = Se caută
app-asset-count =
    { $count ->
        [one] { $count } resursă
        [few] { $count } resurse
       *[other] { $count } de resurse
    }
app-clear-all = Ștergeți tot
app-save-name-label = Numele acestei căutări
app-save-name-placeholder = Denumiți-o
app-save = Salvați
app-save-search-title = Păstrați această căutare în Filtre, la Ale dvs.
app-save-search = Salvați căutarea
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Actualizați „{ $name }”
app-update-search-title = Păstrați selecția curentă în această căutare salvată
app-save-search-new = Salvați ca nouă
app-select-title = Alegeți mai multe resurse pentru a le eticheta, categorisi, colecta, descărca sau arhiva împreună (X sau Ctrl-clic)
app-select = Selectați
app-tile-size = Mărimea dalelor
app-supporting-title = Buffere glTF, biblioteci de materiale, hărțile materialelor, materiale purtate de un singur model și fișierele de magazin care descriu modele. Vin împreună cu ce le folosește atunci când descărcați. Paginile unei foi de sprite-uri și sprite-urile ei pe care pachetul le are și ca imagini separate sunt afișate împreună cu foaia. Un element pe care pachetul îl are în mai multe formate este afișat o dată, în formatul pe care Hoard îl citește cel mai bine.
app-supporting-hidden =
    { $count ->
        [one] { $count } fișier auxiliar ascuns
        [few] { $count } fișiere auxiliare ascunse
       *[other] { $count } de fișiere auxiliare ascunse
    }
app-supporting-show = Afișați
app-supporting-shown = Inclusiv fișierele auxiliare
app-supporting-hide = Ascundeți
app-folders = Dosare
app-folders-title = Vezi pachetul așa cum era pe disc, câte un dosar
app-folders-label = Dosarele pachetului
app-welcome-title = Plasați un pachet de resurse oriunde pe această pagină
app-welcome-before = Merg arhive zip, tarballuri, dosare sau fișiere separate. Hoard le dezarhivează, stochează fiecare fișier o singură dată după hash, citește mărimi, durate și număr de poligoane și etichetează totul ca să puteți căuta lucruri precum{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }sau{ " " }
app-welcome-after = .
app-welcome-import-folder = Importați un dosar
app-welcome-choose-files = Alegeți fișiere
app-none-similar = Încă nimic nu seamănă prea mult cu { $name }.
app-none = Nicio potrivire.
app-none-query = Nimic nu se potrivește cu „{ $query }”.
app-none-imitation = Niciun sunet măsurat nu sună încă prea mult ca „{ $word }”.
app-none-heard = Niciun sunet măsurat nu sună încă prea mult ca zgomotul tău.
app-none-filters = Nimic nu se potrivește cu aceste filtre.
app-none-query-filters = Nimic nu se potrivește cu „{ $query }” cu aceste filtre.
app-clear-search-filters = Ștergeți căutarea și filtrele
app-loading-more = Se încarcă mai multe
app-keys-hint = Săgețile mută, Enter deschide, Space redă un sunet, X sau Ctrl-clic selectează mai multe (apăsare lungă pe ecrane tactile).
app-resize-label = Redimensionați panoul de detalii
app-resize-title = Trageți pentru a redimensiona · dublu clic pentru lățimea potrivită
app-selected-label = Resurse selectate
app-selected = { $count } selectate
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Editează…
app-edit-selected-title = Schimbă etichetele și câmpurile tuturor (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] { $count } rezultat selectat
        [few] { $count } rezultate selectate
       *[other] Toate cele { $count } de rezultate selectate
    }
app-pick-assets = Alegeți resurse
app-download-zip = Descărcați zip
app-credits-title = CREDITS.md pentru pachetele din care provin: cine le-a creat, licențele lor și mențiunile pe care le cer
app-credits = Credite
app-prefer-sources-title = SoundWave-uri și texturi Unreal al căror fișier original l-a recuperat Hoard
app-prefer-sources = WAV/PNG recuperat în loc de .uasset ({ $count })
bag-clear = Goliți selecția
bag-glb-heading = Modele ca GLB
bag-glb-text = Fiecare model ca fișier .glb, în picioare cu Y în sus și orientat înainte, cu texturile ca fișiere separate într-un dosar textures comun modelelor. Restul intră așa cum este.
bag-glb-download = Descărcați zip-ul GLB
bag-plain-heading = Fișiere așa cum sunt
bag-plain-text = Fiecare fișier așa cum a fost importat, cu ce îi trebuie, într-un dosar pentru fiecare pachet.
bag-remove = Scoateți { $name } din selecție
bag-more = și încă { $count }
app-bulk-tag-label = O etichetă de adăugat la resursele selectate sau de scos de pe ele
app-bulk-tag-placeholder = Etichetă
app-bulk-tag-add-title = Adăugați această etichetă la fiecare resursă selectată
app-bulk-tag-add = Adăugați
app-bulk-tag-remove-title = Scoateți această etichetă de pe fiecare resursă selectată
app-bulk-tag-remove = Eliminați
app-remove-from-collection = Eliminați din { $name }
app-unfavorite = Scoateți din favorite
app-favorite = Adăugați la favorite
app-select-all = Selectați toate ({ $count })
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] Selectează rezultatul ({ $count })
        [few] Selectează cele { $count } rezultate
       *[other] Selectează toate cele { $count } de rezultate
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] Editează { $count } resursă
        [few] Editează { $count } resurse
       *[other] Editează { $count } de resurse
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] Editează { $count } rezultat
        [few] Editează cele { $count } rezultate
       *[other] Editează toate cele { $count } de rezultate
    }
bulk-edit-close = Închide
bulk-edit-tags = Etichete
bulk-edit-tag-add = Adaugă o etichetă la toate
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } din { $count }
bulk-edit-untick = Debifează tot
bulk-edit-untick-title = Scoate toate etichetele, apoi bifează-le pe cele de păstrat
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = Cele mai folosite etichete dintre ele
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Mixt, lăsat neschimbat
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Lăsat neschimbat
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Golit la toate
bulk-edit-reset = Revino la cum era
bulk-edit-all-note = Aceasta modifică toate rezultatele, inclusiv pe cele încă neîncărcate, și nu poate fi anulată dintr-un singur pas.
bulk-edit-save =
    { $count ->
        [one] Salvează { $count } modificare
        [few] Salvează { $count } modificări
       *[other] Salvează { $count } de modificări
    }
bulk-edit-nothing = Nimic modificat
bulk-edit-saved =
    { $count ->
        [one] { $count } resursă actualizată
        [few] { $count } resurse actualizate
       *[other] { $count } de resurse actualizate
    }
app-archive-selected-title = Scoase din căutare până când sunt restaurate sau șterse definitiv din Arhivă
app-drop-items =
    { $count ->
        [one] Plasați { $count } element pentru a-l adăuga
        [few] Plasați cele { $count } elemente pentru a le adăuga
       *[other] Plasați cele { $count } de elemente pentru a le adăuga
    }
app-drop = Plasați aici pentru a adăuga în Hoard
app-drop-hint = Fiecare fișier sau dosar de pe primul nivel devine un pachet separat. Gigaocteții nu sunt o problemă; fișierele mari se reiau dacă se întrerupe conexiunea.

## Paleta de comenzi (CommandPalette.vue, cu comenzile din App.vue)

palette-group-go-to = Mergeți la
palette-group-actions = Acțiuni
palette-group-theme = Temă
palette-group-kinds = Tipuri
palette-group-packs = Pachete
palette-group-collections = Colecții
palette-group-projects = Proiecte
palette-group-saved = Căutări salvate
palette-group-assets = Resurse
palette-hint-current = curentă
palette-library = Bibliotecă
palette-library-keywords = resurse grilă acasă bibliotecă assets grid home
palette-favorites = Favorite
palette-favorites-keywords = favorite stele steluțe apreciate starred stars liked
palette-packs = Pachete
palette-archive = Arhivă
palette-archive-keywords = coș gunoi șterse restaurare trash recycle bin deleted restore
palette-stats = Statisticile bibliotecii
palette-stats-keywords = statistici numere mărime spațiu disc număr statistics numbers size disk space count
palette-copies = Aproape copii
palette-copies-keywords = duplicate identice asemănătoare redimensionate convertite spațiu irosit duplicates same similar resized converted wasted space dedupe
palette-search = Căutare
palette-add-files = Adăugați fișiere…
palette-add-files-keywords = încărcare import upload
palette-import-folder = Importați un dosar…
palette-import-folder-keywords = încărcare pachet upload pack
palette-select-all = Selectați toate cele { $count } afișate
palette-select-all-keywords = bifare selectare check
palette-select-several = Selectați mai multe…
palette-select-several-keywords = grup multiple etichetă arhivă bulk multiple tag archive
palette-archive-selected = Arhivați cele { $count } selectate
palette-archive-selected-keywords = ștergere eliminare coș delete remove trash
palette-edit-selected = Editează cele { $count } selectate
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = modificare în masă etichete categorie licență evaluare
palette-paste-tags =
    { $count ->
        [one] Lipește etichetele copiate pe resursa selectată
       *[other] Lipește etichetele copiate pe cele { $count } selectate
    }
palette-clear-selection = Goliți selecția
palette-clear-filters = Ștergeți filtrele
palette-clear-filters-keywords = resetare reset
palette-hide-supporting = Ascundeți fișierele auxiliare
palette-show-supporting = Afișați fișierele auxiliare
palette-supporting-keywords = hărți normale rugozitate măști normal maps roughness masks
palette-account = Cont și tokenuri de acces
palette-account-keywords = parolă conectare utilizator token api password login user
palette-sign-out = Deconectare ({ $name })
palette-sign-out-keywords = deconectare ieșire log out logout
palette-sign-in = Conectare
palette-sign-in-keywords = conectare autentificare cont parolă log in login account password
palette-settings = Setări
palette-settings-keywords = blender ffmpeg llm ai etichetare unreal administrator instrumente preferințe tagging admin tools preferences
palette-shortcuts = Comenzi rapide de la tastatură
palette-shortcuts-keywords = ajutor taste help keys
palette-theme-keywords = temă aspect mod theme appearance mode
palette-theme-variant-keywords = aspect culoare appearance color
palette-light-theme = Temă luminoasă: { $name }
palette-dark-theme = Temă întunecată: { $name }
palette-kind-all = Toate tipurile (nu mai afișați doar { $kind })
palette-kind-only = Doar { $kind }
palette-title = Paleta de comenzi
palette-placeholder = Căutați resurse, mergeți la un pachet, rulați o comandă…
palette-results = Rezultate
palette-searching = Se caută…
palette-no-match = Nimic nu se potrivește cu „{ $query }”
palette-foot-move = mutare
palette-foot-run = rulare
palette-foot-shortcuts = toate comenzile rapide

## Comenzi rapide de la tastatură (ShortcutsDialog.vue)

shortcuts-title = Comenzi rapide de la tastatură
shortcuts-close = Închideți
shortcuts-anywhere = Oriunde
shortcuts-palette = Paleta de comenzi: căutare, salt, rulare
shortcuts-search = Căutare
shortcuts-this-list = Această listă
shortcuts-escape = Închidere, golirea selecției sau închiderea detaliilor
shortcuts-library = Bibliotecă
shortcuts-move = Deplasare între dale (sau J și K)
shortcuts-open = Deschiderea detaliilor
shortcuts-play-sound = Redarea sau oprirea unui sunet
shortcuts-select = Selectare sau deselectare
shortcuts-favorite = Adăugare la favorite sau scoatere din favorite
shortcuts-reviewed = Marcare ca verificat sau nu
shortcuts-tag = Adăugarea unei etichete
shortcuts-edit = Editează selecția
shortcuts-copy-tags = Copiază-i etichetele
shortcuts-paste-tags = Lipește etichetele copiate pe selecție
shortcuts-key-click = clic
shortcuts-select-range = Selectarea unui interval
shortcuts-select-all = Selectarea a tot ce este afișat
shortcuts-sound-player = Player de sunet
shortcuts-play-pause = Redare sau pauză
shortcuts-skip = Salt înapoi sau înainte
shortcuts-start-end = Început sau sfârșit
shortcuts-image-inspector = Inspectorul de imagini
shortcuts-zoom = Mărire sau micșorare
shortcuts-fit = Potrivire în fereastră
shortcuts-actual-pixels = Pixeli reali
shortcuts-tile = Repetare 3×3 pentru verificarea îmbinărilor
shortcuts-background = Fundalul următor
shortcuts-channel = Afișarea unui singur canal

## KindChips.vue

kinds-group = Tipuri de resurse

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = adăugată de o persoană
tag-source-rule = de la etichetatorul offline
tag-source-meta = măsurată din fișier
tag-source-ai = sugerată de AI
tag-remove = Eliminați eticheta { $tag }
tag-accept = Păstrați eticheta { $tag }
tag-accept-title = Păstrați această presupunere ca etichetă proprie
tag-reject-title = Eliminați-o; etichetatorul nu o va pune la loc
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Etichetă nouă
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Ați vrut să spuneți
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = înainte { $alias }

## ActivityIndicator.vue

activity-label-idle = Activitate de fundal: inactiv
activity-label-running = Activitate de fundal: { $running } în curs
activity-label-queued = Activitate de fundal: { $running } în curs, { $queued } în așteptare
activity-title = Activitate de fundal
activity-unreachable = Serverul nu poate fi accesat
activity-unreachable-desktop = Hoard nu răspunde
activity-empty = Nimic nu rulează. Importurile, etichetarea, conversiile și descărcările zip apar aici cât timp lucrează.
activity-queued = în așteptare
activity-stop = Oprește „{ $what }”
activity-stop-tagging = Oprește toată etichetarea AI ({ $count })
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } tri
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Redați { $name }
tile-pause = Puneți pe pauză { $name }
tile-favorite = Adăugați { $name } la favorite
tile-in-favorites = În favoritele dvs.
tile-add-favorite = Adăugați la favorite (F)
tile-more-like = Mai multe ca { $name }
tile-more-like-this = Mai multe ca aceasta
tile-select = Selectați { $name }

## CollectionPicker.vue

collections-add = Adăugați în colecție
collections-menu = Colecții
collections-find-label = Găsiți sau creați o colecție
collections-find-placeholder = Găsiți sau denumiți o colecție nouă
collections-create = Creați
collections-empty = Încă nicio colecție. Tastați un nume pentru a crea una.

## FolderPicker.vue

folders-list = Dosare
folders-close = Închideți
folders-browse = Răsfoiți…
folders-up-title = Un dosar mai sus
folders-up = Sus
folders-this-computer = Acest computer
folders-none = Niciun dosar aici.

## LocationForm.vue

locations-mode-question = Ce să facă Hoard cu acest dosar?
locations-mode-managed = Îl revendică: Hoard îmi păstrează pachetele aici
locations-mode-managed-hint = Pachetele pe care le încărcați sau le importați sunt copiate aici, iar Hoard le ține în ordine. Dosarul este acum al lui Hoard, așa că nu îi modificați conținutul.
locations-mode-watched = Îl urmărește: pachetele mele sunt deja aici
locations-mode-watched-hint = Fiecare dosar din interior devine un pachet. Hoard citește fișierele acolo unde sunt și nu le modifică, nu le mută și nu le șterge niciodată, iar dosarele noi sunt preluate pe măsură ce apar.
locations-folder-desktop = Dosar
locations-folder-placeholder = Dosarul în care sunt pachetele dvs.
locations-folder-hint-managed = Hoard îl creează dacă nu există. În Docker, acesta este un dosar pe care l-ați montat în container.
locations-folder-hint-managed-desktop = Hoard îl creează dacă nu există.
locations-folder-hint-watched = În Docker, montați mai întâi dosarul în container (doar citire este suficient), apoi alegeți-l aici.
locations-folder-hint-watched-desktop = Hoard doar îl citește: nimic din el nu este modificat.
locations-name = Nume
locations-name-placeholder-managed = Bibliotecă
locations-name-placeholder-watched = Pachetele mele
locations-files = Fișierele care merg aici
locations-catch-all = Și tot ce nu preia nicio altă locație
locations-kinds-hint = Fișierele unui pachet nou merg în locația care le bifează tipul. Restul merg în locația implicită.
locations-files-all = Toate, deocamdată. Adăugați mai târziu alte locații din Setări pentru a trimite unele tipuri (de exemplu muzica) pe alt disc.
locations-first-watched-note = Încărcarea are nevoie de o bibliotecă păstrată de Hoard. Puteți adăuga una mai târziu din Setări → Locații ale bibliotecii.
locations-checking = Se verifică dosarul…
locations-add = Adăugați locația
locations-cancel = Anulați

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Locațiile bibliotecii nu au putut fi încărcate
locations-takes-and-rest = { $kinds } și tot restul
locations-takes-rest = Tot ce nu preiau celelalte
locations-takes-everything = Tot
locations-takes-nothing = Încă nimic
locations-save-failed = Locația nu a putut fi salvată
locations-unwatch-title = Nu mai urmăriți { $name }?
locations-unwatch-body =
    { $count ->
        [one] Pachetul său ({ $count }) iese din Hoard, cu etichetele și notele lui. Fișierele din dosar nu sunt atinse.
        [few] Cele { $count } pachete ale sale ies din Hoard, cu etichetele și notele lor. Fișierele din dosar nu sunt atinse.
       *[other] Cele { $count } de pachete ale sale ies din Hoard, cu etichetele și notele lor. Fișierele din dosar nu sunt atinse.
    }
locations-unwatch-body-empty = Fișierele din dosar nu sunt atinse.
locations-unwatch = Nu mai urmăriți
locations-remove-title = Eliminați { $name }?
locations-remove-body = Fișierele din ea se mută mai întâi în celelalte locații, după tip; locația dispare după ce se golește. Nu se pierde nimic.
locations-remove-body-only = Dacă are fișiere, adăugați mai întâi altă locație în care să fie mutate.
locations-remove = Eliminați
locations-removed = { $name } a fost eliminată
locations-emptying = Se mută fișierele din { $name }; locația este eliminată după ce se termină
locations-offline-title = { $name } nu poate fi accesată
locations-offline-body = Fișierele ei nu pot fi mutate cât timp este offline. O eliminați oricum? Fișierele rămase pe ea vor apărea ca lipsă.
locations-remove-anyway = Eliminați oricum
locations-remove-failed = { $name } nu a putut fi eliminată
locations-rebalancing = Se mută fișierele stocate în locațiile care le preiau tipul
locations-rebalance-failed = Fișierele nu au putut fi mutate
locations-scanned-never = încă nescanată
locations-scanned-now = scanată chiar acum
locations-scanned-minutes = scanată acum { $mins } min
locations-scanned-hours = scanată acum { $hours } h
locations-scanned-date = scanată pe { $date }
locations-scan-failed = { $name } nu a putut fi scanată
locations-watching = Se urmărește { $name }
locations-added = { $name } a fost adăugată
locations-loading = Se încarcă…
locations-managed-title = Biblioteci păstrate de Hoard
locations-watched-title = Dosare urmărite
locations-managed-hint = Încărcările și importurile sunt copiate aici. Fiecare locație preia tipurile pe care le bifează; locația implicită preia restul.
locations-watched-hint = Dosare de pachete indexate acolo unde sunt. Hoard nu modifică niciodată fișierele din ele și caută dosare noi sau eliminate la fiecare 15 minute.
locations-managed-empty = Încă niciuna, deci nu există unde să fie pusă o încărcare. Adăugați una mai jos.
locations-reachable = Accesibilă
locations-unreachable-now = Nu poate fi accesată acum
locations-catch-all-tag = implicită
locations-packs =
    { $count ->
        [one] { $count } pachet
        [few] { $count } pachete
       *[other] { $count } de pachete
    }
locations-unreachable = nu poate fi accesată
locations-scanning = Se scanează…
locations-scan = Scanați acum
locations-edit = Editați
locations-catch-all-locked = Pentru a schimba asta, faceți altă locație implicită
locations-edit-hint-before = Se aplică fișierelor adăugate de acum înainte. Pentru a muta ce este deja stocat, folosiți
locations-edit-hint-after = mai jos.
locations-save = Salvați
locations-rebalance = Mutați fișierele stocate în consecință
locations-rebalance-hint = După ce schimbați ce preia o locație: fișierele deja stocate se mută în locația în care merge acum tipul lor.
locations-add-title = Adăugați o locație
locations-add-hint = Alt disc pentru unele tipuri de fișiere sau un dosar de pachete pe care îl aveți deja.
locations-kind-question = Ce fel de locație este?
locations-kind-folder = Un dosar
locations-kind-folder-hint = Pe un disc sau o partajare NAS montată în container, ori un dosar sincronizat de Dropbox, Google Drive, OneDrive sau iCloud Drive. Hoard poate păstra fișiere acolo sau poate urmări pachete pe care le aveți deja.
locations-kind-folder-hint-desktop = Pe acest computer, pe alt disc sau pe o partajare NAS, ori un dosar sincronizat de Dropbox, Google Drive, OneDrive sau iCloud Drive. Hoard poate păstra fișiere acolo sau poate urmări pachete pe care le aveți deja.
locations-kind-s3-hint = Un bucket în Amazon S3 sau într-un serviciu asemănător: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard păstrează fișiere acolo.
locations-s3-title = Stocare S3
locations-s3-hint = O locație s3://bucket-name este un bucket din acest serviciu: AWS sau orice serviciu compatibil S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage pe un NAS).
locations-bucket = Bucket
locations-bucket-hint = Numele unui bucket care există deja. Cheile de mai sus trebuie să-l poată lista, citi și scrie.
locations-s3-endpoint = Adresă
locations-s3-endpoint-placeholder = Goală pentru AWS; de ex. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Regiune
locations-s3-access-key = ID cheie de acces
locations-s3-secret-key = Cheie de acces secretă
locations-s3-headers = Anteturi suplimentare
locations-s3-headers-placeholder = Name=value, câte unul pe rând (de exemplu pentru Cloudflare Access)
locations-s3-kept = Salvat (tastați pentru a înlocui)
locations-s3-secrets = Cheile și anteturile sunt păstrate în setările serverului și nu mai sunt afișate niciodată.
locations-s3-secrets-desktop = Cheile și anteturile sunt păstrate în depozitul de chei al acestui computer și nu mai sunt afișate niciodată.
locations-s3-saved = Stocarea S3 a fost salvată
locations-s3-save-failed = Stocarea S3 nu a putut fi salvată
locations-add-more = Adăugați o locație…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Problemele nu au putut fi încărcate
issues-started =
    { $count ->
        [one] Repornit.
        [few] S-au repornit { $count } elemente.
       *[other] S-au repornit { $count } de elemente.
    }
issues-retry-failed = Nu s-a putut reîncerca
issues-dismiss-failed = Nu s-a putut ignora
issues-dismiss-all-title = Ignorați toate cele { $count }?
issues-dismiss-all-body = „{ $title }” este golit. Orice merge din nou prost este afișat din nou.
issues-dismiss-all = Ignorați toate
issues-just-now = chiar acum
issues-minutes-ago = acum { $mins } min
issues-hours-ago = acum { $hours } h
issues-loading = Se încarcă…
issues-none = Nu a mers nimic prost.
issues-action-all = { $action } (toate)
issues-open-asset = Deschideți
issues-open-pack = Afișați pachetul
issues-open-locations = Locații ale bibliotecii
issues-where-before = în
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] o dată
        [few] de { $count } ori
       *[other] de { $count } de ori
    }
issues-starting = Se pornește…
issues-dismiss-named = Ignorați { $title }
issues-dismiss = Ignorați
issues-show-fewer = Afișați mai puține
issues-show-all = Afișați toate ({ $count })
issues-not-listed =
    { $count ->
        [one] Încă { $count } nu este afișată.
        [few] Încă { $count } nu sunt afișate.
       *[other] Încă { $count } nu sunt afișate.
    }

## archive.ts

archive-anyway = Arhivați oricum
archive-failed = Nu s-a putut arhiva: { $error }
archive-undo = Anulați
archive-restore-failed = Nu s-a putut restaura: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pachet
        [few] { $packs } pachete
       *[other] { $packs } de pachete
    } și { $assets ->
        [one] { $assets } resursă
        [few] { $assets } resurse
       *[other] { $assets } de resurse
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pachet
        [few] { $packs } pachete
       *[other] { $packs } de pachete
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } resursă
        [few] { $assets } resurse
       *[other] { $assets } de resurse
    }
archive-count-nothing = nimic

## ArchiveView.vue

archive-load-failed = Arhiva nu a putut fi încărcată: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Restaurat: { $what }
archive-purged = Șters definitiv: { $what }
archive-purge-failed = Nu s-a putut șterge: { $error }
archive-purge-title = Ștergeți definitiv { $name }?
archive-purge-pack-body =
    { $count ->
        [one] Dispar și resursa sa ({ $count }) și { $size } de fișiere, cu excepția fișierelor pe care le are în comun cu alt pachet. Acest lucru nu poate fi anulat.
        [few] Dispar și cele { $count } resurse ale sale și { $size } de fișiere, cu excepția fișierelor pe care le are în comun cu alt pachet. Acest lucru nu poate fi anulat.
       *[other] Dispar și cele { $count } de resurse ale sale și { $size } de fișiere, cu excepția fișierelor pe care le are în comun cu alt pachet. Acest lucru nu poate fi anulat.
    }
archive-purge = Ștergeți definitiv
archive-purge-asset-body = Fișierul ei este eliberat, cu excepția cazului în care îl folosește și altă resursă. Acest lucru nu poate fi anulat.
archive-empty-title = Goliți arhiva?
archive-empty-body = Tot ce este în arhivă ({ $what }) va fi șters definitiv, iar fișierele vor fi eliberate. Acest lucru nu poate fi anulat.
archive-empty = Goliți arhiva
archive-title = Arhivă
archive-intro = Pachetele și resursele arhivate nu apar în căutare, în totaluri și în lista de etichete. Restaurați-le sau ștergeți-le definitiv pentru a le elibera spațiul.
archive-loading = Se încarcă…
archive-is-empty = Arhiva este goală
archive-is-empty-hint = Arhivați un pachet sau o resursă și va aștepta aici până când o restaurați sau o ștergeți definitiv.
archive-packs = Pachete
archive-assets = Resurse
archive-asset-count =
    { $count ->
        [one] { $count } resursă
        [few] { $count } resurse
       *[other] { $count } de resurse
    }
archive-archived = arhivat { $when }
archive-from = din { $pack }
archive-restore = Restaurați
archive-show-more = Afișați mai multe (au rămas { $count })

## mine.ts

projects-favorite-add-failed = Nu s-a putut adăuga la favorite: { $error }
projects-favorite-remove-failed = Nu s-a putut scoate din favorite: { $error }
projects-save-search-failed = Căutarea nu a putut fi salvată: { $error }
projects-forget-search-failed = { $name } nu a putut fi eliminată: { $error }
projects-change-search-failed = { $name } nu a putut fi modificată: { $error }

## projects.ts

projects-list-and = { $items } și { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = încă { $count }
projects-other =
    { $count ->
        [one] încă { $count } proiect
        [few] încă { $count } proiecte
       *[other] încă { $count } de proiecte
    }
projects-usage-title-one = { $name } folosește asta
projects-usage-title =
    { $count ->
        [one] { $count } proiect folosește asta
        [few] { $count } proiecte folosesc asta
       *[other] { $count } de proiecte folosesc asta
    }
projects-usage-body-one = Folosit în { $list }: următorul hoard sync le scoate.
projects-usage-body = Folosit în { $list }: următorul hoard sync al fiecăruia le scoate.

## uploader.ts

upload-network-error = eroare de rețea
upload-gave-up =
    { $attempts ->
        [one] { $what }: { $error } după { $attempts } încercare
        [few] { $what }: { $error } după { $attempts } încercări
       *[other] { $what }: { $error } după { $attempts } de încercări
    }
upload-failed-plain = eșuat
upload-already-imported = deja importat
upload-part-failed = încărcarea a eșuat ({ $status })
upload-batch =
    { $count ->
        [one] lot de { $count } fișier
        [few] lot de { $count } fișiere
       *[other] lot de { $count } de fișiere
    }
upload-batch-failed = lotul a eșuat ({ $status })
upload-finish = finalizarea încărcării

## UploadQueue.vue

upload-eta-seconds = au rămas { $s } s
upload-eta-minutes = au rămas { $min } min
upload-eta-hours = au rămas { $h } h
upload-queue = Importuri
upload-importing =
    { $count ->
        [one] Se importă { $count } pachet
        [few] Se importă { $count } pachete
       *[other] Se importă { $count } de pachete
    }
upload-bytes-of = { $sent } din { $total }
upload-rate = { $size }/s
upload-waiting = În așteptare
upload-unpacking = Încărcat, acum se dezarhivează
upload-already = Deja în bibliotecă
upload-stopped = Oprit
upload-files-of =
    { $total ->
        [one] { $done } din { $total } fișier
        [few] { $done } din { $total } fișiere
       *[other] { $done } din { $total } de fișiere
    }
upload-resumed = reluat, { $size } erau deja acolo
upload-skipped = Exact aceste fișiere au fost deja importate ca „{ $name }”, așa că au fost omise.
upload-error = { $error }. Progresul este păstrat: reîncercați sau plasați aceleași fișiere din nou mai târziu.
upload-retry = Reîncercați
# $name is the pack the library has already that these files go into.
upload-update = Actualizați „{ $name }”
upload-copy = Importați ca copie
# $name is the pack the library has already that the upload goes into.
upload-into = Intră în „{ $name }”, care este deja în bibliotecă: fișierele noi se adaugă, iar cele modificate se înlocuiesc.
upload-own-pack = Faceți-l un pachet separat
upload-dismiss = Închideți
upload-cancel = Anulați
upload-minimize = Minimizează
upload-expand = Arată importurile
upload-pill = Se importă { $count } · { $percent }%
upload-pill-stopped = Oprite: { $count }
upload-interrupted = Întrerupt
upload-resume = Reia
upload-resume-all = Reia tot
upload-resume-hint-folder = Alege din nou același dosar sau plasează-l pe pagină ca să continue de unde s-a oprit.
upload-resume-hint-file = Alege din nou același fișier sau plasează-l pe pagină ca să continue de unde s-a oprit.
upload-not-same = Nu sunt aceleași fișiere, așa că nu s-a reluat nimic.
upload-permission-denied = Browserul nu a avut voie să citească din nou fișierele. Alege-le tu.

## PacksView.vue

packs-rename-failed = Nu s-a putut redenumi: { $error }
packs-collection-deleted = Colecția { $name } a fost ștearsă
packs-delete-failed = Nu s-a putut șterge: { $error }
packs-status-uploading = Se încarcă (fișierele primite până acum)
packs-status-ingesting = Se dezarhivează și se etichetează
packs-status-analyzing = LLM-ul etichetează
packs-status-ready = Gata
packs-status-failed = Eșuat
packs-retag-failed = Reetichetarea nu a putut porni: { $error }
packs-read-files-failed = Fișierele pachetului nu au putut fi citite: { $error }
packs-context-load-failed = Detaliile de magazin ale pachetului nu au putut fi încărcate: { $error }
packs-undo-store-failed = Pagina de magazin nu a putut fi scoasă: { $error }
packs-context-save-failed = Detaliile de magazin nu au putut fi salvate: { $error }
packs-unreal-load-failed = Starea conversiei Unreal nu a putut fi încărcată: { $error }
packs-unreal-discard-title = Renunțați la originale?
packs-unreal-discard-body = Fișierele .uasset originale din { $name } dispar după conversie. Datele specifice Unreal din ele, cum ar fi grafurile de materiale, se pierd definitiv.
packs-unreal-discard-confirm = Convertiți și renunțați la ele
packs-unreal-convert-failed = Conversia nu a putut porni: { $error }
packs-unreal-fact-converted = { $done } convertite din Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } pachet Unreal
        [few] { $left } pachete Unreal
       *[other] { $left } de pachete Unreal
    }
packs-unreal-fact-both = { $done } convertite din Unreal, { $left } încă pachete
packs-thumbs-canceled = Anulat după { $done } din { $total }
packs-thumbs-drew =
    { $count ->
        [one] S-a desenat { $count } model
        [few] S-au desenat { $count } modele
       *[other] S-au desenat { $count } de modele
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniatură înlocuită.
        [few] { $lead }: { $improved } miniaturi înlocuite.
       *[other] { $lead }: { $improved } de miniaturi înlocuite.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniatură înlocuită, { $failed } nu au putut fi desenate.
        [few] { $lead }: { $improved } miniaturi înlocuite, { $failed } nu au putut fi desenate.
       *[other] { $lead }: { $improved } de miniaturi înlocuite, { $failed } nu au putut fi desenate.
    }
packs-archived =
    { $count ->
        [one] { $name } a fost arhivat, împreună cu resursa sa ({ $count })
        [few] { $name } a fost arhivat, împreună cu cele { $count } resurse ale sale
       *[other] { $name } a fost arhivat, împreună cu cele { $count } de resurse ale sale
    }
packs-blobs-bad =
    { $count ->
        [one] { $count } fișier nu a trecut ultima verificare a integrității.
        [few] { $count } fișiere nu au trecut ultima verificare a integrității.
       *[other] { $count } de fișiere nu au trecut ultima verificare a integrității.
    }
packs-collections = Colecții
packs-browse-named = Răsfoiți { $name }
packs-collection-name = Numele colecției
packs-save = Salvați
packs-cancel = Anulați
packs-assets =
    { $count ->
        [one] { $count } resursă
        [few] { $count } resurse
       *[other] { $count } de resurse
    }
packs-updated = Actualizat { $date }
packs-browse = Răsfoiți
packs-collection-download = Descărcați (.zip)
packs-collection-credits = Credite
packs-collection-credits-title = Cine a creat pachetele din care provin aceste resurse, licențele lor și mențiunile pe care le cer
packs-rename = Redenumiți
packs-delete = Ștergeți
packs-packs = Pachete
packs-empty = Încă niciun pachet. Plasați un zip sau un dosar oriunde pe această pagină pentru a adăuga unul.
packs-find = Găsiți un pachet
packs-find-placeholder = Găsiți un pachet după nume, magazin, licență sau etichetă
packs-sort = Ordine
packs-sort-new = Cele mai noi primele
packs-sort-old = Cele mai vechi primele
packs-sort-size = Cele mai mari primele
packs-sort-assets = Cele mai multe resurse primele
packs-sort-count = Număr de resurse
packs-sort-smallest = Cele mai mici primele
packs-sort-fewest = Cele mai puține resurse primele
packs-with-kind = Pachete cu
packs-state = Stare
packs-state-any = Orice stare
packs-state-working = În lucru
packs-state-failed = Eșuate
packs-state-untagged = Încă nedescrise
packs-state-unlicensed = Fără licență notată
packs-store = Magazin
packs-store-any = Orice magazin
packs-store-from = De pe { $store }
packs-store-none = Fără pagină de magazin
packs-shown-of = { $shown } din { $total }
packs-clear = Ștergeți
packs-no-match = Niciun pachet nu se potrivește.
packs-clear-all = Ștergeți căutarea și filtrele
packs-pack-name = Numele pachetului
packs-thumbs-drawing = Se desenează miniaturile în acest browser…
packs-thumbs-drawing-count = Se desenează miniaturile în acest browser: { $done } din { $total }…
packs-thumbs-stopped = Oprit: { $error }
packs-thumbs-none = Fiecare model de aici are deja o miniatură desenată cu toate texturile sale.
packs-dismiss = Închideți
packs-license = Licență: { $license }
packs-license-tier = Licență: { $license } ({ $tier })
packs-license-not-found = negăsită
packs-from-before = De pe
packs-from-after = { "" }
packs-added = Adăugat { $date }
pack-panel-label = Despre acest pachet
pack-panel-hide = Ascunde detaliile pachetului
pack-panel-kinds = Ce conține
pack-panel-all = Toate pachetele
packs-download = Descărcați pachetul (.zip)
packs-store-details = Detalii de magazin
packs-more-named = Mai multe acțiuni pentru { $name }
packs-more = Mai multe
packs-rename-more = Redenumiți…
packs-retag-rules = Rulați din nou etichetatorul offline
packs-retag-llm = Rulați din nou etichetatorul LLM
packs-make-thumbs = Creați miniaturi în acest browser
packs-make-thumbs-title = Deschide fiecare model, pe rând, într-un vizualizator 3D ascuns și îi folosește imaginea ca miniatură acolo unde este mai bună decât cea a serverului
packs-make-thumbs-title-desktop = Deschide fiecare model, pe rând, într-un vizualizator 3D ascuns și îi folosește imaginea ca miniatură acolo unde este mai bună decât cea creată de Hoard
packs-read-files = Citiți din nou fișierele pachetului
packs-credits = Descărcați creditele
packs-credits-title = CREDITS.md: cine a creat pachetul, licența sa și mențiunea pe care o cere
packs-unreal-open = Convertiți conținutul Unreal…
packs-archive = Arhivați pachetul
packs-unreal-intro = Hoard citește singur pachetele Unreal și stochează conținutul lor ca fișiere obișnuite: texturile ca PNG sau EXR, sunetele ca WAV, materialele ca JSON. Fișierul convertit ia locul pachetului și îi păstrează etichetele, linkurile și colecțiile.
packs-unreal-converted =
    { $count ->
        [one] convertit
        [few] convertite
       *[other] convertite
    }
packs-unreal-converted-kept =
    { $count ->
        [one] convertit, { $kept } cu originalul păstrat
        [few] convertite, { $kept } cu originalul păstrat
       *[other] convertite, { $kept } cu originalul păstrat
    }
packs-unreal-pending =
    { $count ->
        [one] încă neanalizat
        [few] încă neanalizate
       *[other] încă neanalizate
    }
packs-unreal-needs-editor =
    { $count ->
        [one] așteaptă Unreal Editor
        [few] așteaptă Unreal Editor
       *[other] așteaptă Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] fără un format comun
        [few] fără un format comun
       *[other] fără un format comun
    }
packs-unreal-failed =
    { $count ->
        [one] eșuat
        [few] eșuate
       *[other] eșuate
    }
packs-unreal-reasons = De ce rămân pachete
packs-unreal-editor-before = Mesh-urile și animațiile au nevoie de Unreal Editor. Pe un computer cu Unreal instalat, rulați
packs-unreal-editor-command = hoard unreal-worker --editor <calea către UnrealEditor-Cmd>
packs-unreal-editor-after = ; le exportă ca FBX și încarcă rezultatele aici.
packs-unreal-discard = Renunțați la fișierele .uasset originale după conversie
packs-unreal-discard-hint = Eliberează spațiul. Datele specifice Unreal, cum ar fi grafurile de materiale, se pierd.
packs-unreal-drop = Ștergeți pachetele fără un format comun
packs-unreal-drop-hint = Blueprinturi, niveluri și alte resurse care există doar în motor.
packs-unreal-convert = Convertiți
packs-context-hint-before = De unde provine pachetul și ce spune vânzătorul despre el. Etichetatorul LLM citește asta cu fiecare fișier, așa că o descriere de magazin precum „120 de vrăji de gheață și foc” îl ajută să denumească
packs-context-hint-after = .
packs-store-auto = Găsită automat: pachetul însuși indică această pagină.
packs-store-not-this = Nu aceasta
packs-lookup-label = Căutați în magazine
packs-lookup-placeholder = Numele pachetului în magazin
packs-lookup-searching = Se caută…
packs-lookup-search = Căutați în magazine
packs-lookup-hide = Ascundeți
packs-lookup-clues = Din fișierele pachetului: { $clues }.
packs-lookup-also = S-a mai căutat și { $queries }.
packs-lookup-quoted = „{ $query }”
packs-lookup-nothing = Nimic de pe Fab, itch.io, Kenney sau OpenGameArt nu seamănă cu el. Încercați mai puține cuvinte sau altele.
packs-lookup-use = Folosiți
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Acum
packs-review-store = Din magazin
packs-review-same = La fel
packs-review-read-page = Se citește de pe pagină la salvare
packs-review-take =
    { $count ->
        [one] Completați { $count } câmp
        [few] Completați { $count } câmpuri
       *[other] Completați { $count } de câmpuri
    }
packs-lookup-searched = Căutat { $date }.
packs-lookup-again = Căutați din nou
packs-lookup-again-after = .
packs-lookup-unreachable = { $stores } nu a putut fi accesat de data aceasta.
packs-lookup-also-look = Căutați și pe
packs-lookup-links-and = { " " }și{ " " }
packs-lookup-browser-only = (în ele se poate căuta doar din browser); lipiți mai jos adresa paginii.
packs-lookup-open = Găsiți în magazine…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Vânzător
packs-seller-placeholder = Cine l-a creat
packs-license-label = Licență
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Nivelul licenței
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] { $count } resursă are licență proprie: aplicați-i și licența pachetului
        [few] { $count } resurse au licență proprie: aplicați-le și licența pachetului
       *[other] { $count } de resurse au licență proprie: aplicați-le și licența pachetului
    }
packs-store-page = Pagina de magazin
packs-store-text = Descrierea din magazin
packs-store-text-placeholder = Lipiți textul anunțului sau lăsați gol pentru a-l prelua de pe pagina de magazin.
packs-store-text-fetch = Salvarea preia descrierea de pe pagina de magazin.
packs-readme = README găsit în pachet
packs-save-retag = Salvați și rulați din nou etichetatorul LLM

## AssetDetail.vue

asset-lazy-3d-viewer = vizualizatorul 3D
asset-lazy-text = textul
asset-lazy-text-viewer = vizualizatorul de text
asset-lazy-image-viewer = vizualizatorul de imagini
asset-lazy-material-preview = previzualizarea materialului
asset-lazy-material-viewer = vizualizatorul de materiale
asset-lazy-sprite-player = playerul de sprite-uri
asset-lazy-sheet-view = vizualizatorul de foi de sprite-uri
asset-rel-texture = textură
asset-rel-buffer = date de geometrie
asset-rel-material = fișier de material
asset-rel-animation = animație
asset-rel-library = fișier legat
asset-rel-guess = probabil merge cu
asset-rel-sidecar = detalii de magazin
asset-rel-variant = variantă
asset-rel-sprite = sprite
asset-rel-sample = eșantion
asset-role-map = hartă de { $role }
asset-maps-count =
    { $count ->
        [one] { $count } hartă
        [few] { $count } hărți
       *[other] { $count } de hărți
    }
asset-needs = Are nevoie de
asset-animations = Animații
asset-sprites = Sprite-uri
asset-used-by = Folosit de
asset-loose-sprites = Și ca fișiere separate
asset-in-sheet = Într-o foaie de sprite-uri
asset-other-formats = Alte formate
asset-shown-as = Afișat în bibliotecă ca
asset-conversion-failed = conversia a eșuat
asset-as-converting = Se convertește…
asset-as-converting-percent = Se convertește… { $percent }%
asset-as-queued = În așteptare…
asset-download = Descărcați
asset-as-label = Descărcați ca
asset-as-original = { $format } (formatul original)
asset-as-pack-files = Fișierele pachetului
asset-as-size = Mărime
asset-as-full-size = Mărime completă
asset-as-up-to = Până la { $size } px
asset-as-hint = Alegeți o mărime sau alt format
asset-meta-size = Mărime
asset-meta-length = Durată
asset-meta-sample-rate = Rată de eșantionare
asset-meta-channels = Canale
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Adâncime de biți
asset-meta-bits =
    { $bits ->
        [one] { $bits } bit
        [few] { $bits } biți
       *[other] { $bits } de biți
    }
asset-meta-level = Nivel
asset-meta-level-value = vârf { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = vârf { $peak } dBFS, RMS { $rms } dBFS, saturează
asset-meta-loop = Buclă
asset-meta-loop-value = de la { $start } la { $end }
asset-meta-playback = Redare
asset-meta-one-shot = O singură dată
asset-meta-envelope = Anvelopă
asset-meta-envelope-times = atac { $attack }, coadă { $tail }
asset-meta-envelope-hit = lovitură
asset-meta-envelope-swell = crescendo
asset-meta-envelope-sustained = susținut
asset-meta-tone = Timbru
asset-meta-tone-value = strălucire { $khz } kHz, zgomot { $noise } %
asset-meta-pitch = Înălțime
asset-meta-pitch-rises =
    { $octaves ->
        [one] urcă { $octaves } octavă
        [few] urcă { $octaves } octave
       *[other] urcă { $octaves } de octave
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] coboară { $octaves } octavă
        [few] coboară { $octaves } octave
       *[other] coboară { $octaves } de octave
    }
asset-meta-loops-cleanly = Se repetă fără întrerupere
asset-meta-tempo = Tempo
asset-meta-tempo-heard = aproximativ { $bpm } BPM, după ureche
asset-meta-tempo-heard-or = aproximativ { $bpm } BPM (sau { $alt }), după ureche
asset-meta-key = Tonalitate
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, după ureche
music-key-major = { $note } major
music-key-minor = { $note } minor
asset-meta-root-note = Notă fundamentală
asset-meta-cue-markers = Marcaje cue
asset-meta-library = Bibliotecă
asset-meta-keywords = Cuvinte cheie
asset-meta-title = Titlu
asset-meta-artist = Artist
asset-meta-album = Album
asset-meta-genre = Gen
asset-meta-generator = Generator
asset-meta-everything-with = Arată tot ce are eticheta { $tag }
asset-meta-saved-with = Salvat cu
asset-meta-color-mode = Mod de culoare
asset-meta-layers = Straturi
asset-meta-range = Interval
asset-meta-hdr = HDR (previzualizarea are tone mapping)
asset-meta-format = Format
asset-meta-large-document = Document mare (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Creat cu AI generativ
asset-meta-ai-edited = Editat cu AI generativ
asset-meta-ai-how-metadata = metadatele
asset-meta-ai-how-generator = setările generatorului
asset-meta-ai-how-iptc = tipul de sursă digitală IPTC
asset-meta-ai-says = { $made } (sursa: { $how })
asset-meta-ai-by-says = { $made } ({ $by }; sursa: { $how })
asset-meta-provenance = Proveniență
asset-meta-c2pa = Are C2PA Content Credentials
asset-meta-compression = Compresie
asset-meta-note = Notă
asset-meta-maps = Hărți
asset-meta-sizes =
    { $count ->
        [one] Mărime
        [few] Mărimi
       *[other] Mărimi
    }
asset-meta-triangles = Triunghiuri
asset-meta-vertices = Vârfuri
asset-meta-meshes = Mesh-uri
asset-meta-materials = Materiale
asset-meta-animations = Animații
asset-meta-skins = Skinuri
asset-meta-unreal-asset = Resursă Unreal
asset-meta-unreal-saved-with = { $class }, salvat cu UE { $engine }
asset-meta-package = Pachet
asset-meta-cooked = Cooked (build de joc): previzualizările și sursele sunt eliminate
asset-meta-tiling = Repetare
asset-meta-seamless = Fără îmbinări vizibile pe ambele axe
asset-meta-analysis-error = Acest fișier nu a putut fi citit complet: { $error }
asset-copy-blocked = Nu s-a putut copia: browserul a blocat copierea
asset-save-error = Nu s-a putut salva: { $error }
asset-tag-removed = S-a eliminat { $tag }
asset-tag-restored = { $tag } a fost pusă la loc
asset-tags-added =
    Adăugate: { $count ->
        [one] { $count } etichetă
        [few] { $count } etichete
       *[other] { $count } de etichete
    }
asset-tags-accepted =
    { $count ->
        [one] S-a păstrat { $count } etichetă presupusă
        [few] S-au păstrat { $count } etichete presupuse
       *[other] S-au păstrat { $count } de etichete presupuse
    }
asset-reviewed = Verificat
asset-reviewed-on-title = Verificat: regulile și AI nu îi ating etichetele. Faceți clic pentru a scoate marcajul (R)
asset-reviewed-off-title = Marcați ca verificat, ca regulile și AI să nu îi atingă etichetele (R)
asset-reviewed-on = Marcat ca verificat
asset-reviewed-off = S-a scos marcajul de verificat
# $name is an asset's name.
asset-reviewed-on-named = { $name } a fost marcat ca verificat
asset-reviewed-off-named = S-a scos marcajul de verificat de pe { $name }
asset-undo = Anulați
asset-play-error = Acest fișier nu a putut fi redat: { $error }
asset-copied-hash = Hash copiat
asset-copied-toml = toml copiat
asset-copied-id = ID copiat
asset-copied-link = Link copiat
asset-origin-before = Acest fișier era
asset-origin-class-engine = , un { $class } salvat de UE { $engine }.
asset-origin-class = , un { $class }.
asset-origin-engine = { " " }salvat de UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor l-a exportat pe { $date }.
asset-origin-editor-notes = Unreal Editor l-a exportat pe { $date } ({ $notes }).
asset-origin-hoard = Hoard l-a extras din pachet pe { $date }.
asset-origin-hoard-notes = Hoard l-a extras din pachet pe { $date } ({ $notes }).
asset-origin-download = Descărcați originalul .{ $ext }
asset-origin-discarded = Pachetul original a fost eliminat după conversie.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Istoric
history-loading = Se încarcă…
history-none = Încă nu s-a modificat nimic aici.
history-load-failed = Istoricul nu a putut fi încărcat: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = AI
history-by-rule = O regulă
history-by-someone = Cineva
history-by-app = { $who } printr-o aplicație
history-by-assistant = { $who } printr-un asistent
history-field-description = Descriere
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Etichetă adăugată: { $tag }
history-tag-removed = Etichetă eliminată: { $tag }
history-tag-kept = Etichetă sugerată păstrată: { $tag }
history-tag-changed = Etichetă modificată: { $tag }
history-reviewed = Marcat ca verificat
history-unreviewed = Marcat ca neverificat
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: setat la { $to }
history-cleared = { $field }: golit (era { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Anulați
history-revert-title = Readuce ce a schimbat această modificare
history-reverted = Anulat: { $change }
history-revert-failed = Nu s-a putut anula: { $error }
asset-details = Detaliile resursei
asset-in-pack = în
asset-previous = Resursa anterioară
asset-next = Resursa următoare
asset-favorite = Favorit
asset-favorite-on-title = În favoritele dvs. (F o scoate)
asset-favorite-off-title = Adăugați la favorite (F)
asset-share = Partajați
asset-share-title = Partajați un link către această resursă (cine îl deschide se conectează mai întâi la Hoard)
asset-layout-panel = Afișați într-un panou lateral
asset-layout-page = Afișați pe toată pagina
asset-layout-page-title = Afișați pe toată pagina: o previzualizare mare lângă detalii
asset-close = Închideți detaliile
asset-rename-title = Faceți clic pentru a redenumi
asset-name = Nume
asset-views-alt = Modelul din patru direcții
asset-inspect-aria = Inspectați { $name } pe tot ecranul
asset-inspect-title = Inspectare: zoom, repetare, canale
asset-playback-position = Poziția de redare
asset-playback-value = { $current } din { $duration }
asset-pause = Pauză
asset-play = Redare
asset-waveform-drawing = Se desenează forma de undă…
asset-waveform-skipped = Nicio formă de undă pentru fișiere atât de mari
asset-play-animation = Redați ca animație
asset-show-views = Afișați foaia cu cele patru vederi pe care a văzut-o etichetatorul
asset-palette = Culorile din imagine
asset-swatch-title = { $hex }, { $percent }% din imagine. Găsiți resurse cu această culoare
asset-swatch-aria = Găsiți resurse cu { $hex }, { $percent }% din această imagine
asset-more-like-this = Mai multe ca aceasta
asset-desc-label = Descrierea dvs.
asset-desc-placeholder = Ce este, unde l-ați folosi…
asset-save = Salvați
asset-cancel = Anulați
asset-desc-family =
    { $count ->
        [one] Se salvează pentru { $count } variantă
        [few] Se salvează pentru toate cele { $count } variante
       *[other] Se salvează pentru toate cele { $count } de variante
    }
asset-desc-edit = Editați descrierea
asset-desc-write = Scrieți o descriere
asset-desc-ai = Scrisă de AI
asset-desc-file = Din fișier
asset-desc-store-before = Din
asset-desc-store-page = pagina { $store }
asset-desc-store-page-unnamed = pagina de magazin
asset-desc-store-after = (întregul pachet)
asset-less = Mai puțin
asset-more = Mai mult
asset-tags = Etichete
asset-tag-add-label = Adăugați o etichetă
asset-tag-add-placeholder = Adăugați o etichetă, de ex. mood:tense
asset-apply-family = Aplicați modificările la toate cele { $count } variante
asset-tags-accept-all =
    { $count ->
        [one] Păstrați { $count } presupunere
        [few] Păstrați toate cele { $count } presupuneri
       *[other] Păstrați toate cele { $count } de presupuneri
    }
asset-tags-accept-all-title = Păstrați ca ale dvs. toate etichetele presupuse de reguli și de AI
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Respinse:
asset-tag-restore-title = Puneți { $tag } la loc
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = Cele asemănătoare au:
asset-tag-suggest-title = Adaugă { $tag }
asset-details-heading = Detalii
asset-field-category = Categorie
asset-field-license = Licență
asset-field-source = Sursă
asset-field-rating = Evaluare
# Shown in place of an empty value, to fill it in.
asset-field-add = Adăugați
asset-field-new = Adăugați un câmp
asset-field-name = Numele câmpului
asset-field-value = Valoare
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = din pachet
asset-field-license-placeholder = De exemplu CC0 sau CC-BY 4.0
asset-field-remove-placeholder = Goliți pentru a elimina
# $field is a field's name, like "License".
asset-field-edit-title = Editați { $field }
asset-field-saved = { $field } a fost salvat
asset-field-cleared = { $field } a fost golit
asset-rate =
    { $count ->
        [one] { $count } stea
        [few] { $count } stele
       *[other] { $count } de stele
    }
asset-rated =
    { $count ->
        [one] Evaluat cu { $count } stea
        [few] Evaluat cu { $count } stele
       *[other] Evaluat cu { $count } de stele
    }
asset-rating-clear-title = Faceți din nou clic pentru a șterge evaluarea
asset-rating-cleared = Evaluarea a fost ștearsă
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Faceți din nou clic pentru { $count } stea
        [few] Faceți din nou clic pentru { $count } stele
       *[other] Faceți din nou clic pentru { $count } de stele
    }
asset-collections = Colecții
asset-collection-remove = Eliminați din { $name }
asset-collection-add-another = Adăugați în alta
asset-collection-add = Adăugați în colecție
asset-used-in = Folosit în
asset-used-in-title = Afișați ce conține { $name }
asset-used-in-synced = sincronizat { $when }
asset-maps = Hărți
asset-show-more = Afișați încă { $count }
asset-materials-heading =
    { $count ->
        [one] Material
        [few] Materiale
       *[other] Materiale
    }
asset-material-maps-aria = { $name }: hărți
asset-variants =
    { $count ->
        [one] Variante ({ $count } versiune)
        [few] Variante ({ $count } versiuni)
       *[other] Variante ({ $count } de versiuni)
    }
asset-fit-animations = Animații potrivite ({ $count })
asset-fit-characters = Personaje potrivite ({ $count })
asset-copies = Copii identice
asset-copies-note = Aceiași octeți sunt stocați o singură dată; și aceste pachete conțin acest fișier.
asset-near-copies = Aproape copii
asset-near-copies-note = Același lucru la altă dimensiune, frecvență de eșantionare sau în alt format. Cea mai bună copie este marcată.
asset-near-copies-best = Același lucru la altă dimensiune, frecvență de eșantionare sau în alt format. Aceasta este cea mai bună copie.
asset-near-best = cea mai bună
asset-unreal-converted = Convertit din Unreal
asset-rule-converted = Convertit de o regulă
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = O regulă de conversie a creat acest { $format } pe { $date } din
asset-rule-kept-until = Originalul este păstrat până pe { $date }.
asset-rule-rebuilt = FLAC-ul păstrează tot ce era în WAV, așa că WAV-ul poate fi descărcat sau pus înapoi exact cum era oricând.
asset-rule-put-back = Restaurați originalul
asset-rule-put-back-title = Restaurați { $file }?
asset-rule-put-back-body = Fișierul convertit este șters, iar originalul îi ia locul, cu etichetele, colecțiile și linkurile sale. De atunci încolo, regulile de conversie nu se mai ating de el.
asset-rule-put-back-done = { $file } a fost restaurat
asset-rule-let-go = Originalul a fost șters după ce a expirat perioada de păstrare a originalelor din Setări.
asset-unreal-package = Din pachetul Unreal
asset-ue-needs-editor-before = Se așteaptă Unreal Editor: { $reason }. Pe un computer cu Unreal instalat, rulați
asset-ue-needs-editor-after = și revine ca FBX.
asset-ue-editor-path = calea către UnrealEditor-Cmd
asset-ue-unsupported = Păstrat ca pachet: { $reason }.
asset-ue-failed = Conversia a eșuat: { $reason }
asset-recovered = Fișierul .{ $ext } original era stocat în acest pachet, iar Hoard l-a extras.
asset-recovered-download = Descărcați .{ $ext }
asset-ue-meshes-before = Mesh-urile și animațiile sunt stocate în formatul propriu Unreal. Convertiți pachetul (pagina pachetului, Mai multe), apoi rulați
asset-ue-meshes-after = lângă o instalare Unreal Editor pentru a obține FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } etichetă din registrul de resurse
        [few] { $count } etichete din registrul de resurse
       *[other] { $count } de etichete din registrul de resurse
    }
asset-file = Fișier
asset-bytes = Octeți
asset-source = Sursă
asset-path = Cale
asset-path-open-folder = Vezi ce mai e în { $folder }
asset-download-maps =
    { $count ->
        [one] Descărcați { $count } hartă
        [few] Descărcați toate cele { $count } hărți
       *[other] Descărcați toate cele { $count } de hărți
    }
asset-download-linked =
    { $count ->
        [one] Descărcați cu { $count } fișier legat
        [few] Descărcați cu { $count } fișiere legate
       *[other] Descărcați cu { $count } de fișiere legate
    }
asset-download-only-title = Doar acest fișier, fără fișierele pe care le folosește
asset-download-only = Doar acest fișier
asset-include-animations = Includeți animațiile în .zip
asset-include-character = Include personajul { $name }
asset-copy-toml = Copiați intrarea hoard.toml
asset-copy-id = Copiați ID-ul
asset-teach = Învățați Hoard acest fișier
asset-teach-title = Descrieți fișierul acestui magazin ca Hoard să-l poată citi (Setări → Fișiere de magazin)
asset-archive = Arhivați
asset-archive-title = Scoasă din căutare până când este restaurată sau ștearsă definitiv din Arhivă

## ImageViewer.vue

image-channel-red = Canalul roșu
image-channel-green = Canalul verde
image-channel-blue = Canalul albastru
image-channel-alpha = Canalul alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Fundal în carouri (K schimbă)
image-bg-black-title = Fundal negru (K schimbă)
image-bg-gray-title = Fundal gri (K schimbă)
image-bg-white-title = Fundal alb (K schimbă)
image-bg-checker = fundal în carouri
image-bg-black = fundal negru
image-bg-gray = fundal gri
image-bg-white = fundal alb
image-inspect-aria = Inspectați { $name }
image-preview-size = previzualizare { $width }×{ $height }, { $bytes }
image-preview = previzualizare
image-resolution = Rezoluție
image-full-title = Încărcați imaginea completă de { $width }×{ $height } (F); la fel se întâmplă dacă măriți dincolo de previzualizare
image-full-loading = Se încarcă rezoluția completă…
image-full-failed = Rezoluția completă nu s-a încărcat: reîncercați
image-full = Rezoluție completă
image-zoom = Zoom
image-zoom-out-title = Micșorați (−)
image-zoom-out = Micșorați
image-zoom-in-title = Măriți (+)
image-zoom-in = Măriți
image-fit-title = Potriviți (0)
image-fit = Potriviți
image-actual-title = Pixeli reali (1)
image-view = Vizualizare
image-tile-title = Repetare 3×3 pentru verificarea îmbinărilor (T)
image-tile = Repetare
image-channels = Canale
image-close = Închideți (Esc)
image-failed = Această imagine nu a putut fi încărcată.
image-loading = Se încarcă…
image-key-gestures = Derulați sau ciupiți pentru zoom · trageți pentru deplasare · dublu clic pentru mărire sau potrivire
image-key-tile = repetare
image-key-background = fundal
image-key-full = rezoluție completă
image-key-channels = canale

## SpritePlayer.vue

sprite-frame-aria = { $name }, cadrul { $frame } din { $count }
sprite-failed = Acesta nu poate fi redat.
sprite-loading = Se încarcă cadrele…
sprite-previous = Cadrul anterior
sprite-next = Cadrul următor
sprite-pause = Puneți animația pe pauză
sprite-play = Redați animația
sprite-animation = Animație
sprite-all-frames = Toate cadrele
sprite-grid-title = Cum este tăiată foaia în cadre
sprite-columns = Coloane
sprite-rows = Rânduri
sprite-fps-aria = Cadre pe secundă
sprite-fps = fps

## Vizualizatorul de foi de sprite-uri (SheetView.vue) și formatele sale de descărcare (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
        [few] { $count } sprite-uri
       *[other] { $count } de sprite-uri
    }
sheet-pages =
    { $count ->
        [one] { $count } pagină
        [few] { $count } pagini
       *[other] { $count } de pagini
    }
sheet-animations =
    { $count ->
        [one] { $count } animație
        [few] { $count } animații
       *[other] { $count } de animații
    }
# The box that narrows the list of sprites by name.
sheet-find = Căutați un sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Se afișează { $shown } din { $total }
sheet-no-match = Niciun sprite nu are asta în nume.
sheet-failed = Această foaie nu poate fi citită.
sheet-loading = Se încarcă foaia…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Formatul propriu Hoard: paginile așa cum sunt, sprite-uri rotite în orice sens și decupate, cu animații și durata fiecărui cadru.
sheet-note-texturepacker = Sprite-uri rotite în sensul acelor de ceasornic și decupate, cu animații după nume, așa cum le citesc PixiJS și Phaser.
sheet-note-clockwise = Sprite-uri rotite în sensul acelor de ceasornic și decupate. Nu poate păstra animații: cadrele se recunosc după numele numerotate.
sheet-note-libgdx = Sprite-uri rotite în sens invers acelor de ceasornic și decupate. Nu poate păstra animații: cadrele se recunosc după numele numerotate.
sheet-note-godot-frames = Sprite-uri niciodată rotite, cu decuparea păstrată ca margini, cu animații și durata fiecărui cadru.
sheet-note-godot-tileset = Tile-uri niciodată rotite sau decupate, pe o grilă de mărimea unui tile, păstrând imaginile neschimbate când sunt deja pe o grilă.
sheet-note-grid = Fiecare sprite drept, pe întreaga sa pânză, în celule de aceeași mărime, pentru orice program care taie o foaie după o grilă.
sheet-note-frames = Fiecare sprite drept, pe întreaga sa pânză, fiecare ca PNG cu numele lui.

## TextViewer.vue, TextPreview.vue și text.ts

text-lang-config = Configurație
text-lang-table = Tabel
text-lang-code = Cod
text-lang-script = Script
text-lang-text = Text
text-copy-blocked = Nu s-a putut copia: browserul a blocat copierea
text-read-aria = Citiți { $name }
text-lines =
    { $count ->
        [one] { $count } rând
        [few] { $count } rânduri
       *[other] { $count } de rânduri
    }
text-show-as = Afișați ca
text-tree-title = Arbore pliabil (T)
text-tree = Arbore
text-pretty-title = Reindentat (P)
text-pretty = Formatat
text-raw-title = Așa cum este scris (R)
text-raw = Brut
text-document-title = Ca document (D)
text-document = Document
text-source-title = Markdown-ul așa cum este scris (R)
text-source = Sursă
text-page-title = Așa cum o arată un browser din dosarul ei, separat de Hoard (P)
text-page = Pagină
text-page-source-title = Pagina așa cum e scrisă (R)
text-page-aria = { $name } ca pagină
text-page-error = Pagina nu a putut fi deschisă: { $error }
text-fold = Pliați
text-expand-all-title = Depliați tot
text-expand-all = Extindeți tot
text-collapse-all-title = Pliați tot
text-collapse-all = Restrângeți tot
text-group-text = Text
text-wrap-title = Încadrați rândurile lungi (W)
text-wrap = Încadrare
text-copy-title = Copiați textul (C)
text-copied = Copiat
text-copy = Copiați
text-download-title = Descărcați fișierul
text-download = Descărcați
text-close = Închideți (Esc)
text-truncated = Se afișează primii { $shown } din { $total }.
text-truncated-json = Se afișează primii { $shown } din { $total } (arborele are nevoie de tot fișierul).
text-show-all = Afișați tot
text-json-error = Acest JSON nu poate fi analizat: { $error }
text-contents-aria = Conținutul { $name }
text-loading = Se încarcă…
text-load-error = Acest fișier nu a putut fi încărcat: { $error }
text-binary = Acest fișier este binar, nu text, așa că nu există nimic de citit aici. Descărcați-l pentru a-l deschide în programul potrivit.
text-key-tree = arbore
text-key-pretty = formatat
text-key-raw = brut
text-key-document = document
text-key-source = sursă
text-key-page = pagină
text-key-wrap = încadrare
text-key-copy = copiere
text-key-find = căutare
text-key-close = închidere
text-preview-aria = Citiți { $name } pe tot ecranul
text-preview-read = Citiți

## JsonTree.vue și JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } element
        [few] { $count } elemente
       *[other] { $count } de elemente
    }
json-keys =
    { $count ->
        [one] { $count } cheie
        [few] { $count } chei
       *[other] { $count } de chei
    }
json-fold = Pliați
json-unfold = Depliați
json-unfold-summary = Depliați { $summary }
json-show-more = Afișați încă { $count }
json-left = din { $count } rămase
json-picker-more = … încă { $count }

## Vizualizatoare de modele 3D (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = vizualizatorul 3D nu s-a încărcat ({ $error }); reîncărcarea paginii poate ajuta
model-loading-viewer = Se încarcă vizualizatorul 3D…
model-loading = Se încarcă modelul…
model-show-failed = Acest model nu a putut fi afișat: { $error }
model-preview-failed = Acest model nu a putut fi previzualizat: { $error }
model-error-converting = serverul încă convertește acest model
model-error-converting-desktop = Hoard încă convertește acest model
model-error-empty-collada = fișier Collada gol
model-error-no-loader = Niciun încărcător de browser pentru .{ $ext }
model-dialog = Vizualizați { $name } în 3D
model-title-stats =
    { $triangles } tri · { $meshes ->
        [one] { $meshes } mesh
        [few] { $meshes } mesh-uri
       *[other] { $meshes } de mesh-uri
    } · { $materials ->
        [one] { $materials } material
        [few] { $materials } materiale
       *[other] { $materials } de materiale
    }
model-triangles =
    { $count ->
        [one] { $count } triunghi
        [few] { $count } triunghiuri
       *[other] { $count } de triunghiuri
    }
model-group-camera = Cameră
model-group-shading = Umbrire
model-group-background = Fundal
model-group-animation = Animație
model-group-view = Vizualizare
model-group-show = Afișare
model-frame-title = Încadrați modelul (F)
model-frame = Încadrare
model-reset-title = Înapoi la vederea inițială (R)
model-reset = Resetare
model-turn-title = Rotire lentă (A)
model-turn = Rotire
model-textures-title = Texturi (T)
model-textures = Texturi
model-full-res-title = Texturi la rezoluție completă (H): fiecare fișier de textură așa cum este, oricât de mare; dezactivat, previzualizări ușoare de cel mult 2048 px
model-full-res = Rez. completă
model-light-title = Lumina mediului (E)
model-light = Lumină
model-grid-title = Grila podelei (G)
model-grid = Grilă
model-bg-checker = Fundal în carouri
model-bg-dark = Fundal întunecat
model-bg-gray = Fundal gri
model-bg-light = Fundal luminos
model-bg-env = Fundalul mediului
model-bg-title = { $background } (K schimbă)
model-pause-title = Pauză (Space)
model-play-title = Redare (Space)
model-pause = Pauză
model-play = Redare
model-animation-clip = Clip de animație
model-animation = Animație
model-plays-on = Redă pe
model-rest-pose = Poză de repaus
model-close-title = Închideți (Esc)
model-full-screen = Ecran complet
model-open-full-screen = Deschideți vizualizatorul 3D pe tot ecranul
model-menu-fold = Ascundeți meniul
model-menu-show = Afișați meniul
model-view-prefix = Vizualizare:{ " " }
model-menu-render = Randare
model-menu-maps = Hărți
model-menu-inspect = Inspectare
model-mode-final = Randare finală
model-mode-raw = Randare brută
model-mode-color = Culoare de bază
model-mode-roughness = Rugozitate
model-mode-metalness = Metalicitate
model-mode-normal = Normale
model-mode-ao = Ocluzie
model-mode-matcap = Matcap
model-mode-uv = Verificare UV
model-mode-geometry = Geometrii
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Wireframe (W), peste orice vizualizare
model-wireframe = Wireframe
model-info-title = Informații despre scenă (I)
model-info = Informații despre scenă
model-info-close = Închideți informațiile despre scenă
model-info-close-title = Închideți (I)
model-info-geometries = Geometrii
model-info-materials = Materiale
model-info-textures = Texturi
model-info-triangles = Triunghiuri
model-info-vertices = Vârfuri
model-info-meters = glTF este în metri
model-info-no-units = Acest format nu spune ce unități folosește
model-info-height = Înălțime
model-info-length = Lungime
model-info-width = Lățime
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } unitate
        [few] { NUMBER($n, maximumSignificantDigits: 3) } unități
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } de unități
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Textură negăsită: { $name }
model-missing-some =
    { $count ->
        [one] { $count } textură negăsită: { $names }
        [few] { $count } texturi negăsite: { $names }
       *[other] { $count } de texturi negăsite: { $names }
    }
model-missing-more =
    { $count ->
        [one] { $count } textură negăsită: { $names }…
        [few] { $count } texturi negăsite: { $names }…
       *[other] { $count } de texturi negăsite: { $names }…
    }
model-missing-count =
    { $count ->
        [one] { $count } textură negăsită
        [few] { $count } texturi negăsite
       *[other] { $count } de texturi negăsite
    }
model-guessed = Texturi potrivite după nume; fișierul nu numește niciuna.
model-group-facing = Orientare
model-facing-turn = Rotire cu un sfert
model-facing-turn-title = Rotiți modelul cu un sfert de tură în sens antiorar, privit de sus
model-facing-tip = Înclinare cu un sfert
model-facing-tip-title = Înclinați partea de sus a modelului cu un sfert de tură spre față, pentru a ridica în picioare un model culcat
model-facing-save = Aplicare la pachet
model-facing-save-title = Rotiți astfel toate modelele din { $pack }. Miniaturile lor sunt desenate din nou; descărcările rămân cum au fost făcute.
model-facing-undo = Anulați
model-facing-undo-title = Înapoi la felul în care sunt rotite modelele pachetului
model-facing-saved = Toate modelele din { $pack } sunt acum orientate așa. Miniaturile lor sunt desenate din nou.
model-facing-failed = Modelele pachetului nu au putut fi rotite: { $error }
model-keys-mouse = Trageți pentru orbitare · trageți cu clic dreapta pentru deplasare · derulați pentru zoom · dublu clic pentru încadrare
model-key-frame = încadrare
model-key-reset = resetare
model-key-view = vizualizare
model-key-wireframe = wireframe
model-key-info = informații despre scenă
model-key-textures = texturi
model-key-full-res = texturi la rezoluție completă
model-key-light = lumină
model-key-grid = grilă
model-key-background = fundal
model-key-play = redare

## Materiale (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Culoare
material-role-normal = Normale
material-role-normal-dx = Normale (DirectX)
material-role-roughness = Rugozitate
material-role-gloss = Luciu
material-role-metalness = Metalicitate
material-role-ao = Ocluzie ambientală
material-role-orm = Ocluzie, rugozitate, metal
material-role-mra = Metal, rugozitate, ocluzie
material-role-rma = Rugozitate, metal, ocluzie
material-role-height = Înălțime
material-role-bump = Bump
material-role-cavity = Cavitate
material-role-specular = Specularitate
material-role-opacity = Opacitate
material-role-emissive = Emisie
material-role-translucency = Translucență
material-role-base-color-name = culoare
material-role-normal-name = normale
material-role-normal-dx-name = normale (DirectX)
material-role-roughness-name = rugozitate
material-role-gloss-name = luciu
material-role-metalness-name = metalicitate
material-role-ao-name = ocluzie ambientală
material-role-orm-name = ocluzie, rugozitate, metal
material-role-mra-name = metal, rugozitate, ocluzie
material-role-rma-name = rugozitate, metal, ocluzie
material-role-height-name = înălțime
material-role-bump-name = bump
material-role-cavity-name = cavitate
material-role-specular-name = specularitate
material-role-opacity-name = opacitate
material-role-emissive-name = emisie
material-role-translucency-name = translucență
material-maps =
    { $count ->
        [one] { $count } hartă
        [few] { $count } hărți
       *[other] { $count } de hărți
    }
material-dialog = Vizualizați materialul { $name }
material-up-to = până la { $size }
material-repeat-size = { $size } per repetare
material-shape-sphere = Sferă
material-shape-cube = Cub
material-shape-plane = Plan
material-shape-title = { $shape } ({ $key })
material-group-shape = Formă
material-group-tiling = Repetare
material-real-title = Mărime reală: o repetare are { $size }, sfera are 2 m în diametru (T schimbă)
material-repeat-title = Repetare de { $n }× (T schimbă)
material-real = Real
material-repeat = { $n }×
material-height-title = Cât de mult împinge harta de înălțime suprafața
material-height = Înălțime
material-height-strength = Intensitatea înălțimii
material-loading = Se încarcă materialul…
material-show-failed = Acest material nu a putut fi afișat: { $error }
material-preview-failed = Acest material nu a putut fi previzualizat: { $error }
material-group-maps = Hărți
material-map-show = Afișați harta de { $map }
material-map-hide = Ascundeți harta de { $map }
material-map-unused = Nefolosită de previzualizare
material-open-full-screen = Deschideți vizualizatorul de materiale pe tot ecranul
material-keys-mouse = Trageți pentru orbitare · trageți cu clic dreapta pentru deplasare · derulați pentru zoom · faceți clic pe o hartă pentru a o dezactiva
material-key-shape = formă
material-key-tiling = repetare
material-key-turn = rotire

## theme.ts

theme-slate = Ardezie
theme-paper = Hârtie
theme-linen = In
theme-sage = Salvie
theme-lilac = Liliac
theme-blush = Roz pudrat
theme-graphite = Grafit
theme-obsidian = Obsidian
theme-ember = Jar
theme-moss = Mușchi
theme-aubergine = Vânătă
theme-oxblood = Bordo
theme-light-slate-blurb = Gri-albastru rece
theme-light-paper-blurb = Gri neutru simplu
theme-light-linen-blurb = Crem cald, chihlimbar
theme-light-sage-blurb = Verde moale
theme-light-lilac-blurb = Violet pal
theme-light-blush-blurb = Roz prăfuit
theme-dark-graphite-blurb = Gri neutru, fără nuanță
theme-dark-slate-blurb = Gri-albastru
theme-dark-obsidian-blurb = Aproape negru, pentru OLED
theme-dark-ember-blurb = Cărbune cald, chihlimbar
theme-dark-moss-blurb = Verde de pădure spre negru
theme-dark-aubergine-blurb = Prună spre negru, liliac
theme-dark-oxblood-blurb = Vin spre negru, coral

## texnames.ts

texnames-needs-map = Șablonul are nevoie de { "{" }map{ "}" }, altfel toate hărțile unui material ar primi același nume
texnames-placeholders = Șablonul poate folosi { "{" }name{ "}" }, { "{" }map{ "}" } și { "{" }size{ "}" }
texnames-bad-characters = Numele nu pot conține < > : " / \ | ? sau *
texnames-no-suffix = Dați un sufix hărților de { $role }
texnames-bad-suffix = Sufixul pentru hărțile de { $role } nu poate conține < > : " / \ | ? sau *
texnames-slot-no-suffix = Dați un sufix pentru „{ $slot }”
texnames-slot-bad-suffix = Sufixul pentru „{ $slot }” nu poate conține < > : " / \ | ? sau *
texnames-ending-missing = Fiecare terminație proprie are nevoie de terminația de căutat
texnames-ending-bad = Terminațiile proprii nu pot conține < > : " / \ | ? sau *
texnames-ending-twice = { $ending } apare de două ori în terminațiile dvs.
texnames-slot-metallic-roughness = Metal și rugozitate
texnames-slot-specular-color = Culoare speculară
texnames-slot-specular-glossiness = Specularitate și luciu
texnames-slot-clearcoat = Strat de lac
texnames-slot-clearcoat-roughness = Rugozitatea stratului de lac
texnames-slot-clearcoat-normal = Normalele stratului de lac
texnames-slot-sheen-color = Culoarea satinării
texnames-slot-sheen-roughness = Rugozitatea satinării
texnames-slot-transmission = Transmisie
texnames-slot-thickness = Grosime
texnames-slot-diffuse-transmission = Transmisie difuză
texnames-slot-diffuse-transmission-color = Culoarea transmisiei difuze
texnames-slot-iridescence = Irizare
texnames-slot-iridescence-thickness = Grosimea irizării
texnames-slot-anisotropy = Anizotropie

## storeFormats.ts

storefiles-field-title = Titlu
storefiles-field-title-hint = Titlul exact al anunțului. El denumește pachetul, iar fără el fișierul este omis.
storefiles-field-seller = Vânzător
storefiles-field-seller-hint = Cine l-a creat sau îl vinde.
storefiles-field-license = Licență
storefiles-field-license-hint = Așa cum o scrie fișierul, de exemplu CC-BY-4.0.
storefiles-field-url = Pagina de magazin
storefiles-field-url-hint = Adresa anunțului, care începe cu http:// sau https://.
storefiles-field-words = Etichete
storefiles-field-words-hint = Cuvinte cu care să fie etichetat. Valorile separate prin virgulă sunt despărțite.
storefiles-field-type = Tip
storefiles-field-type-hint = Ce este, de exemplu „3D model” sau „surface”. Este și etichetă.
storefiles-field-colors = Culori
storefiles-field-colors-hint = Culorile sale, ca etichete color:.
storefiles-field-id = ID de magazin
storefiles-field-id-hint = ID-ul dat de magazin: litere, cifre, _ sau -.
storefiles-field-origin = Creat în
storefiles-field-origin-hint = Unde a fost creat sau scanat, pentru descriere.
storefiles-field-size = Mărime
storefiles-field-size-hint = Cât de mare este, în cuvinte, pentru descriere.
storefiles-field-title-name = titlul
storefiles-field-seller-name = vânzătorul
storefiles-field-license-name = licența
storefiles-field-url-name = pagina de magazin
storefiles-field-words-name = etichetele
storefiles-field-type-name = tipul
storefiles-field-colors-name = culorile
storefiles-field-id-name = ID-ul de magazin
storefiles-field-origin-name = locul de creare
storefiles-field-size-name = mărimea

## StoreFormatsPanel.vue

storefiles-load-failed = Formatele fișierelor de magazin nu au putut fi încărcate: { $error }
storefiles-remove-title = Eliminați { $name }?
storefiles-remove-body = { $file } este șters din dosarul de date. Pachetele deja citite cu el păstrează ce a citit.
storefiles-remove = Eliminați
storefiles-removed = { $name } a fost eliminat.
storefiles-remove-failed = { $name } nu a putut fi eliminat: { $error }
storefiles-unread = Fișiere pe care Hoard nu le citește
storefiles-loading = Se încarcă…
storefiles-unread-none = Niciunul: niciun fișier din bibliotecă nu arată ca fișierul unui magazin necitit.
storefiles-unread-what = Fișiere aflate în mai multe pachete sau denumite ca ale unui magazin, pe care nu le citește niciun format.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pachet
        [few] { $count } pachete
       *[other] { $count } de pachete
    }
storefiles-unread-example = de ex. { $pack }
storefiles-teach = Învățați Hoard acest fișier
storefiles-formats = Formate
storefiles-formats-what = Ale dvs. sunt încercate primele și iau locul unui format integrat cu același nume.
storefiles-dir-before = Sunt păstrate în
storefiles-dir-after = .
storefiles-yours = Ale dvs.
storefiles-built-in = Integrat
storefiles-edit = Editați
storefiles-start-from = Porniți de la acesta

## TagsPanel.vue

tags-load-failed = Etichetele nu au putut fi încărcate: { $error }
tags-rename-title = Redenumiți { $tag }?
tags-merge-title =
    { $count ->
        [one] Uniți { $count } etichetă?
        [few] Uniți { $count } etichete?
       *[other] Uniți { $count } de etichete?
    }
tags-rename-body =
    { $count ->
        [one] Resursa sa ({ $count }) primește în schimb { $into }, iar etichetatorii folosesc noul nume de acum înainte.
        [few] Cele { $count } resurse ale sale primesc în schimb { $into }, iar etichetatorii folosesc noul nume de acum înainte.
       *[other] Cele { $count } de resurse ale sale primesc în schimb { $into }, iar etichetatorii folosesc noul nume de acum înainte.
    }
tags-merge-body =
    { $count ->
        [one] { $tags } devin { $into } pe cel mult { $count } resursă. Etichetatorii folosesc { $into } de acum înainte.
        [few] { $tags } devin { $into } pe cel mult { $count } resurse. Etichetatorii folosesc { $into } de acum înainte.
       *[other] { $tags } devin { $into } pe cel mult { $count } de resurse. Etichetatorii folosesc { $into } de acum înainte.
    }
tags-rename = Redenumiți
tags-merge = Uniți
tags-merged =
    { $count ->
        [one] { $count } resursă are acum eticheta { $into }
        [few] { $count } resurse au acum eticheta { $into }
       *[other] { $count } de resurse au acum eticheta { $into }
    }
tags-merge-failed = Nu s-a putut uni: { $error }
tags-forget-failed = Nu s-a putut face modificarea: { $error }
tags-find = Găsiți etichete
tags-find-placeholder = Găsiți etichete, de ex. rock sau set:
tags-sort = Ordine
tags-sort-count = Cele mai folosite
tags-sort-name = A–Z
tags-count =
    { $count ->
        [one] { $count } etichetă
        [few] { $count } etichete
       *[other] { $count } de etichete
    }
tags-count-more =
    { $count ->
        [one] { $count }+ etichete
        [few] { $count }+ etichete
       *[other] { $count }+ de etichete
    }
tags-what = Alegeți etichete care înseamnă același lucru și uniți-le sau alegeți una pentru a o redenumi.
tags-new-name = Nume nou
tags-merge-into = Uniți în
tags-saved-as-before = Salvată ca
tags-saved-as-after = { "" }
tags-cancel = Anulați
tags-loading = Se încarcă…
tags-none = Nicio etichetă.
tags-none-containing = Nicio etichetă care să conțină „{ $text }”.
tags-browse-title = Răsfoiți resursele etichetate cu { $tag }
tags-browse = Răsfoiți
tags-merged-heading = Etichete unite
tags-merged-what = Când etichetatorii propun unul dintre aceste nume vechi, resursa primește eticheta în care a fost unit.
tags-allow-old = Permiteți din nou numele vechi
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } resursă
        [few] { $count } resurse
       *[other] { $count } de resurse
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Nume vechi: { $names }
        [few] Nume vechi: { $names }
       *[other] Nume vechi: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } a fost unită cu { $into }.
tag-page-show = Afișați { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Spuneți ce înseamnă această etichetă…
tag-page-describe-label = Ce înseamnă această etichetă
tag-page-describe-placeholder = Pentru ce e, ce se încadrează aici. Enter salvează.
tag-page-describe-edit = Faceți clic pentru a schimba
tag-page-description-saved = Semnificația etichetei a fost salvată
# The tag's picture: one of its assets stands for it.
tag-page-picture = Alegeți o imagine pentru această etichetă
tag-page-picture-help = Alegeți una dintre resursele ei care s-o reprezinte
tag-page-picture-nothing = Niciuna dintre resursele afișate nu are imagine.
tag-page-picture-none = Fără imagine
tag-page-picture-saved = Imaginea a fost setată
tag-page-picture-removed = Imaginea a fost scoasă
tag-page-rename = Redenumiți…
tag-page-rename-title = Dați-i un nume nou sau numele altei etichete, ca să fie unită cu aceasta
tag-page-rename-label = Nume nou
tag-page-save-failed = Eticheta nu a putut fi salvată: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Sub
tag-page-parent-add = Plasați sub…
tag-page-parent-add-title = Plasați această etichetă sub una mai largă: dacă o căutați pe cea largă, le găsiți și pe acestea
tag-page-parent-placeholder = O etichetă mai largă, de exemplu vehicul
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Scoateți { $tag } de sub { $parent }
tag-page-parent-saved = { $tag } este acum sub { $parent }
tag-page-parent-removed = { $tag } nu mai este sub { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Mai înguste:
tag-page-narrower-title = Căutarea după { $tag } le găsește și pe acestea
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Mai largi și mai înguste
tags-tree-what = Etichete plasate sub etichete mai largi din paginile lor. Căutarea după o etichetă găsește și tot ce este sub ea.

## LibraryStats.vue

stats-source-human = Adăugate de oameni
stats-source-ai = Sugerate de AI
stats-source-rule = Reguli integrate
stats-source-meta = Din fișiere
stats-assets = Resurse
stats-packs = Pachete
stats-on-disk = Pe disc
stats-duplicates-title =
    { $count ->
        [one] { $count } fișier este o copie identică, octet cu octet, a altuia și este stocat o singură dată
        [few] { $count } fișiere sunt copii identice, octet cu octet, ale altora și sunt stocate o singură dată
       *[other] { $count } de fișiere sunt copii identice, octet cu octet, ale altora și sunt stocate o singură dată
    }
stats-saved-dedup = Economisit prin deduplicare
stats-distinct-tags = Etichete diferite
stats-archive = În arhivă
stats-load-failed = Detaliile nu au putut fi încărcate: { $error }
stats-counting = Se numără…
stats-as-of = La { $time }
stats-updating = Se actualizează…
stats-by-kind = Spațiu după tip
stats-files-size =
    { $count ->
        [one] { $name }: { $count } fișier, { $size }
        [few] { $name }: { $count } fișiere, { $size }
       *[other] { $name }: { $count } de fișiere, { $size }
    }
stats-months = Adăugate în ultimul an
stats-month =
    { $count ->
        [one] { $month }: { $count } resursă, { $size }
        [few] { $month }: { $count } resurse, { $size }
       *[other] { $month }: { $count } de resurse, { $size }
    }
stats-tagging = Etichetare
stats-tagged = Etichetate
stats-untagged = { $count } fără etichete
stats-described = Descrise
stats-described-assets =
    { $count ->
        [one] { $count } resursă
        [few] { $count } resurse
       *[other] { $count } de resurse
    }
stats-human-edited = Editate manual
stats-human-edited-unit =
    { $count ->
        [one] resursă
        [few] resurse
       *[other] de resurse
    }
stats-source-different = { $count } diferite
stats-most-used = Cele mai folosite
stats-show-tag = Afișați resursele { $tag }
stats-file-types = Tipuri de fișiere
stats-biggest-packs = Cele mai mari pachete
stats-browse-pack = Răsfoiți { $name }
stats-largest-files = Cele mai mari fișiere
stats-open-file = Deschideți { $name }
stats-empty = Încă nimic în bibliotecă. Adăugați un pachet și cifrele lui apar aici.

## HoardPanel.vue

hoard-title = Comoara dumneavoastră
hoard-what = Fiecare resursă din bibliotecă este o bucată de comoară, iar tipul ei decide care. Adunați ce e nou ca să adăugați la grămada dumneavoastră.
hoard-sign-in = Conectați-vă ca să adunați o comoară proprie.
hoard-new = Pradă nouă de adunat: { $count }
hoard-none-new = Nimic nou de adunat.
hoard-collect = Adună
hoard-unavailable = Acest browser nu poate desena comoara: { $error }
hoard-collect-failed = Nu s-a putut înregistra ce ați adunat: { $error }
hoard-arcade = Ecran de arcade
hoard-full-screen = Ecran complet
hoard-leave-full-screen = Ieșiți din ecranul complet
hoard-label = Un dragon care își păzește comoara. Bucăți: { $count }
hoard-gold = Aur
hoard-gems = Pietre prețioase
hoard-books = Cărți
hoard-weapons = Arme
hoard-bones = Oase
hoard-tablets = Tăblițe runice
hoard-paintings = Picturi
hoard-potions = Poțiuni

## UpdatesPanel.vue

updates-saved = Hoard o va apela când este gata o actualizare
updates-saved-none = Nicio adresă de apelat
updates-save-failed = Nu s-a putut salva
updates-call-answered = A răspuns { $result }
updates-call-refused = Nu a acceptat cererea: { $result }
updates-call-failed = Nu s-a putut apela
updates-called = Apelată pe { $when } pentru { $version }: { $result }
updates-called-failed = Apelată pe { $when } pentru { $version }: { $result }. Hoard încearcă din nou peste o oră.
updates-latest = Hoard { $version } este cea mai nouă versiune.
updates-out = A apărut Hoard { $version }.
updates-out-idle = A apărut Hoard { $version }; apelul se face în câteva minute, cât timp nu se întâmplă nimic.
updates-out-busy = A apărut Hoard { $version }; se așteaptă până când nu se mai întâmplă nimic ({ $work }).
updates-busy-more = încă { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } sarcină în așteptare
        [few] { $count } sarcini în așteptare
       *[other] { $count } de sarcini în așteptare
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } încărcare
        [few] { $count } încărcări
       *[other] { $count } de încărcări
    }
updates-list-separator = ,{ " " }
updates-what = Când apare o versiune nouă, Hoard poate apela o adresă după ce nu se mai întâmplă nimic (nicio încărcare, sarcină, conversie sau întreținere), astfel încât ce îi actualizează containerul să nu-l repornească în mijlocul unei încărcări. Trimite un POST cu versiunea într-un corp JSON. Este nevoie de verificarea versiunilor de mai sus.
updates-url = Adresa de apelat
updates-url-placeholder = Niciuna (de ex. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Salvat (tastați pentru a înlocui)
updates-auth-placeholder = Niciuna (de ex. Bearer my-token)
updates-remove = Eliminați
updates-save = Salvați
updates-call-now-title = Rulează ce este conectat la ea
updates-call-now = Apelați acum
updates-what-calls = Ce poate apela?
updates-watchtower = API-ul HTTP al Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , cu{ " " }
updates-watchtower-token = { " " }și tokenul din{ " " }
updates-watchtower-its = { " " }(cu endpointul{ " " }
updates-watchtower-endpoint = { " " }activat).
updates-portainer = Un webhook Portainer
updates-portainer-rest = { " " }pe stiva sau containerul Hoard, cu re-pull image activat. Tokenul este în URL.
updates-script = Un script pe gazdă
updates-script-listener = { " " }în spatele unui listener de webhook (cum ar fi{ " " }
updates-script-runs = ) care rulează{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }sau orice altceva care acceptă un webhook.
updates-poll-before = Sau renunțați la adresă: un script cu un token de acces poate interoga{ " " }
updates-poll-until = { " " }până când{ " " }
updates-poll-and = { " " }este setat și{ " " }
updates-poll-after = { " " }este true, apoi descarcă versiunea nouă și repornește Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Întreținerea nu a putut fi încărcată
maintenance-reanalyze-title = Reanalizați toate pachetele?
maintenance-reanalyze-body = Fiecare fișier este citit din nou, ceea ce poate dura mult la o bibliotecă mare; pachetele apar ca fiind în lucru până le vine rândul. Numele, etichetele și notele dvs. rămân.
maintenance-reanalyze-confirm = Reanalizați
maintenance-gc-title = Ștergeți fișierele nefolosite?
maintenance-gc-body = Fișierele stocate spre care nu mai indică nimic din bibliotecă sunt șterse definitiv. Pachetele din arhivă le păstrează pe ale lor.
maintenance-gc-confirm = Ștergeți-le
maintenance-start-failed = „{ $task }” nu a putut porni
maintenance-just-now = chiar acum
maintenance-minutes-ago = acum { $count } min
maintenance-hours-ago = acum { $count } h
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Se încarcă…
maintenance-running = Rulează…
maintenance-starting = Se pornește…
maintenance-run = Rulați
maintenance-stop = Oprește
maintenance-stopping = Se oprește…
maintenance-stop-failed = „{ $task }” nu a putut fi oprit
maintenance-started = { $detail } · pornită { $when }
maintenance-starting-detail = Se pornește
maintenance-finished = Terminată:{ " " }
maintenance-failed = Eșuată:{ " " }
maintenance-never-run = Nu a rulat niciodată
maintenance-history = Rulări anterioare
maintenance-col-task = Sarcină
maintenance-col-finished = Terminată
maintenance-col-took = Durată
maintenance-col-result = Rezultat
maintenance-script-before = Copiile de rezervă și verificarea deteriorărilor pot fi rulate și dintr-un script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }și{ " " }
maintenance-script-after = . Și acele rulări apar aici.

## SettingsPage.vue

settings-title = Setări
settings-loading = Se încarcă…
settings-find = Găsește o setare
settings-find-none = Nicio setare nu se potrivește cu „{ $query }”.
settings-load-failed = Setările nu au putut fi încărcate: { $error }
settings-page-appearance = Aspect
settings-page-appearance-blurb = Limbă, teme, mărimea textului și spațiere, plus ce arată dalele și bara de sus. Salvat doar în acest browser.
settings-page-dragon = Dragon
settings-page-dragon-blurb = Comoara dragonului tău: fiecare resursă din bibliotecă e o bucată din ea.
settings-page-library = Bibliotecă
settings-page-library-blurb = Ce este în Hoard: cât de mare este, din ce este alcătuit, cum a crescut și cât de bine este etichetat. Elementele arhivate nu sunt numărate.
settings-page-copies = Aproape copii
settings-page-copies-blurb = Fișiere care sunt aceeași imagine, același sunet sau model la altă dimensiune, frecvență de eșantionare sau în alt format, întâi cele care ocupă cel mai mult spațiu. Cea mai bună din fiecare set este marcată; nu se șterge nimic.
copies-loading = Se caută aproape copii…
copies-refreshing = Biblioteca s-a schimbat de când au fost găsite. Se caută din nou…
copies-none = Nicio aproape copie: fiecare imagine, sunet și model este aici o singură dată.
copies-load-failed = Nu s-au putut lista aproape copiile: { $error }
copies-sum = Seturi de aproape copii: { $count }. Copiile care nu sunt cele mai bune ocupă { $size }.
copies-set = Setul { $n }
copies-set-wasted = { $size } în copiile care nu sunt cele mai bune
copies-more = Arată mai multe (mai sunt { $count })
settings-page-tags = Etichete
settings-page-tags-blurb = Toate etichetele din bibliotecă. Uniți-le pe cele care înseamnă același lucru sau redenumiți una; etichetatorii folosesc noul nume de atunci înainte.
settings-page-store-files = Fișiere de magazin
settings-page-store-files-blurb = Fișierele pe care magazinele le pun lângă grafica lor și pe care Hoard le citește pentru a denumi pachetele și a-i menționa pe autori. Învățați-l pe cele pe care nu le cunoaște și împărtășiți ce l-ați învățat.
settings-page-locations = Locații ale bibliotecii
settings-page-locations-blurb = Dosarele de pe server care conțin biblioteca: cele în care Hoard păstrează fișiere și dosarele de pachete pe care le indexează acolo unde sunt.
settings-page-locations-blurb-desktop = Dosarele care conțin biblioteca: cele în care Hoard păstrează fișiere și dosarele de pachete pe care le indexează acolo unde sunt.
settings-page-tools = Instrumente
settings-page-tools-blurb = Programe externe pe care Hoard le folosește pentru a converti și a citi fișiere.
settings-page-ai = Etichetare AI
settings-page-ai-blurb = Un model AI denumește, descrie și etichetează pachetele noi, pe lângă regulile integrate. Etichetele lui sunt marcate ca sugerate.
settings-page-search-blurb = Căutați imagini după ce arată și sunete după cum sună, nu doar după nume, și găsiți lucruri asemănătoare ca fel, nu doar ca culoare.
settings-page-search-tab = Căutare
settings-page-adding = Adăugarea pachetelor
settings-page-adding-blurb = Ce face Hoard cu un pachet când este adăugat, pe lângă citirea și etichetarea lui.
settings-page-downloads = Descărcări
settings-page-downloads-blurb = Cum sunt denumite fișierele în ce trimite Hoard: descărcări, arhive zip, instrumentul hoard și asistenții AI. Fișierele din bibliotecă rămân cum sunt.
settings-page-account = Cont
settings-page-account-blurb = Parola dvs., celelalte persoane care se pot conecta și tokenurile de acces pentru scripturi și alte programe.
settings-page-auth = Autentificare
settings-page-auth-blurb = Cum se conectează persoanele: conturile proprii Hoard, un director LDAP, Microsoft Active Directory sau un furnizor OAuth, cât durează sesiunile și ce grupuri primesc ce rol.
settings-page-connection = Conexiune
settings-page-connection-blurb = Cum ajung browserele și aplicațiile la Hoard: adresa sub care este cunoscut și HTTPS, de care au nevoie unele funcții ale browserului.
settings-page-performance = Performanță
settings-page-performance-blurb = Cât de intens lucrează serverul, cum sunt create și păstrate miniaturile și cât poate primi cel mult.
settings-page-performance-blurb-desktop = Cât de intens lucrează Hoard, cum sunt create și păstrate miniaturile și cât poate primi cel mult.
settings-page-issues = Probleme
settings-page-issues-blurb = Ce a mers prost la deschiderea sau procesarea fișierelor: arhive care nu s-au dezarhivat, fișiere care nu au putut fi citite, sarcini care au eșuat. Reîncercați-le după ce cauza este remediată sau ignorați-le.
settings-page-maintenance = Întreținere
settings-page-maintenance-blurb = Copii de rezervă, verificarea deteriorărilor și curățenie. Rulează câte una din fiecare odată și puteți părăsi această pagină cât timp rulează.
settings-page-about = Despre
settings-page-about-blurb = Ce este Hoard, ce versiune este aceasta și munca altora din care este făcut.
settings-the-default = valoarea implicită
settings-default-on = activat
settings-default-off = dezactivat
settings-default-auto = automat
settings-option-saved = { $name }: { $value }
settings-back-to-default = Înapoi la valoarea implicită
settings-save = Salvați
settings-use = Folosiți
settings-status-off = Dezactivat

## SettingsPage.vue: Aspect

settings-themes = Teme
settings-look-light = Luminos
settings-look-dark = Întunecat
settings-look-group =
    { $scheme ->
        [light] Temă luminoasă
       *[dark] Temă întunecată
    }
settings-preview =
    Se afișează tema { $preview ->
        [light] luminoasă
       *[dark] întunecată
    } ca să o puteți vedea. Folosiți tema { $used ->
        [light] luminoasă
       *[dark] întunecată
    }{ $system ->
        [yes] { " " }(după sistem)
       *[other] { "" }
    }, care revine când părăsiți această pagină.
settings-preview-back =
    Înapoi la tema { $used ->
        [light] luminoasă
       *[dark] întunecată
    }
settings-look-hint = Comutați între luminos și întunecat (sau urmați sistemul) din bara de sus.

## SettingsPage.vue: Instrumente

settings-blender-what = Convertește în fundal modelele .blend, .dae, .abc și .x3d în glTF, ca să primească miniaturi și o previzualizare 3D. Pachetele adăugate cât timp era dezactivat sunt convertite după ce este gata.
settings-blender-download = Folosește un Blender instalat sau descarcă Blender 4.5 LTS (aproximativ 400 MB) de pe o oglindă blender.org.
settings-ffmpeg-what = Codifică descărcările MP3, OGG și Opus și citește sunetul pe care Hoard nu îl poate decoda singur (AAC, M4A, WMA, …).
settings-ffmpeg-download = Folosește un ffmpeg de sistem cu codificatoarele potrivite sau descarcă un build static (aproximativ 80 MB).
settings-tool-reset = { $tool }: înapoi la valoarea implicită
settings-tool-no-download = Folosește un { $tool } instalat. Nu există un build de descărcat pentru acest computer, așa că instalați-l sau dați calea către el.
settings-tool-path = Calea programului { $tool }
settings-mode-off = Dezactivat
settings-mode-auto = Automat
settings-mode-path = Acest program
settings-tool-looking = Se caută…
settings-tool-downloading = Se descarcă… progresul apare în indicatorul de activitate
settings-tool-ready = Gata: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Instrumentele cu care sunt redate și convertite fișierele MIDI.
settings-soundfont-file = Fișier SoundFont
settings-soundfont-placeholder = Găsit pe server sau integrat
settings-soundfont-placeholder-desktop = Găsit pe acest computer sau integrat
settings-soundfont-default = găsit pe server sau integrat
settings-soundfont-default-desktop = găsit pe acest computer sau integrat
settings-soundfont-send = Alege un fișier…
settings-soundfont-sent = MIDI cântă acum cu { $name }

## SettingsPage.vue: Căutare după imagine

settings-clip-heading = Căutare după ce arată imaginile
settings-clip-reset = Căutare după imagine: înapoi la valoarea implicită
settings-clip-saved-on = Căutare după imagine: activată
settings-clip-saved-off = Căutare după imagine: dezactivată
settings-clip-what = Tastați ce căutați („mossy rock”, „sci-fi door”, „wooden barrel”), iar imaginile care arată asta apar lângă numele care conțin cuvintele. „Mai multe ca aceasta” găsește lucruri de același fel, nu doar de aceleași culori.
settings-clip-toggle = Activați căutarea după imagine
settings-clip-hint = Folosește modelul SigLIP 2 de la Google, rulat pe procesorul acestui server: nimic nu pleacă de pe el. Activarea descarcă modelul ({ $size } MB) de pe Hugging Face, apoi învață fiecare imagine în fundal, cu prioritate scăzută. Ocupă între 350 și 750 MB de memorie cât timp lucrează sau cât timp caută cineva. Dezactivarea șterge modelul și păstrează ce a învățat, pentru data viitoare.
settings-clip-hint-desktop = Folosește modelul SigLIP 2 de la Google, rulat pe procesorul acestui computer: nimic nu pleacă de pe el. Activarea descarcă modelul ({ $size } MB) de pe Hugging Face, apoi învață fiecare imagine în fundal, cu prioritate scăzută. Ocupă între 350 și 750 MB de memorie cât timp lucrează sau cât timp căutați. Dezactivarea șterge modelul și păstrează ce a învățat, pentru data viitoare.
settings-clip-looking = Se caută modelul…
settings-clip-downloading = Se descarcă modelul… progresul apare în indicatorul de activitate
settings-clip-ready-empty = Gata. Imaginile sunt învățate pe măsură ce se adaugă pachete.
settings-clip-ready-all =
    { $count ->
        [one] Gata: cunoaște { $count } imagine
        [few] Gata: cunoaște toate cele { $count } imagini
       *[other] Gata: cunoaște toate cele { $count } de imagini
    }
settings-clip-learning = Gata: se învață imaginile, { $done } din { $total } până acum
settings-clip-relearning = Până când noul model învață fiecare imagine, căutările folosesc ce a învățat cel vechi.
settings-clip-unavailable = Modelul nu a putut fi obținut: { $detail }. Se încearcă din nou în fiecare oră.
settings-clip-where = Rulează pe
settings-clip-here = Acest server: nimic nu iese de pe el
settings-clip-here-desktop = Acest computer: nimic nu iese de pe el
settings-clip-hosted = Un serviciu găzduit
settings-clip-hosted-warning = Imaginile ies de pe acest server. O miniatură a fiecărei imagini din bibliotecă și cuvintele fiecărei căutări după imagine sunt trimise la { $service }, care le taxează și le păstrează după cum spun condițiile sale.
settings-clip-hosted-warning-desktop = Imaginile ies de pe acest computer. O miniatură a fiecărei imagini din bibliotecă și cuvintele fiecărei căutări după imagine sunt trimise la { $service }, care le taxează și le păstrează după cum spun condițiile sale.
settings-clip-hosted-hint = Nimic nu se descarcă și nu rulează aici: serviciul învață fiecare imagine, pe loturi. Alt serviciu sau model le învață din nou pe toate, iar ce a învățat fiecare se păstrează, pentru a putea reveni.
settings-clip-saved-here = Căutare după imagine: rulează pe acest server
settings-clip-saved-here-desktop = Căutare după imagine: rulează pe acest computer
settings-clip-saved-hosted = Căutare după imagine: rulează la { $service }
settings-clip-hosted-unavailable = { $service } nu a putut fi folosit: { $detail }. Se încearcă din nou în fiecare oră.
settings-clap-heading = Căutare după cum sună sunetele
settings-clap-reset = Căutare după sunet: înapoi la valoarea implicită
settings-clap-saved-on = Căutare după sunet: activată
settings-clap-saved-off = Căutare după sunet: dezactivată
settings-clap-what = Tastați ce căutați („distant thunder”, „glassy UI blip”, „footsteps on gravel”), iar sunetele care sună așa apar lângă numele care conțin cuvintele.
settings-clap-toggle = Activați căutarea după sunet
settings-clap-hint = Folosește modelul CLAP de la LAION, rulat pe procesorul acestui server: nimic nu pleacă de pe el. Activarea descarcă modelul ({ $size } MB) de pe Hugging Face, apoi ascultă începutul fiecărui sunet în fundal, cu prioritate scăzută. Ocupă până la aproximativ 1 GB de memorie cât timp lucrează sau cât timp caută cineva. Dezactivarea șterge modelul și păstrează ce a învățat, pentru data viitoare.
settings-clap-hint-desktop = Folosește modelul CLAP de la LAION, rulat pe procesorul acestui computer: nimic nu pleacă de pe el. Activarea descarcă modelul ({ $size } MB) de pe Hugging Face, apoi ascultă începutul fiecărui sunet în fundal, cu prioritate scăzută. Ocupă până la aproximativ 1 GB de memorie cât timp lucrează sau cât timp căutați. Dezactivarea șterge modelul și păstrează ce a învățat, pentru data viitoare.
settings-clap-ready-empty = Gata. Sunetele sunt învățate pe măsură ce se adaugă pachete.
settings-clap-ready-all =
    { $count ->
        [one] Gata: cunoaște { $count } sunet
        [few] Gata: cunoaște toate cele { $count } sunete
       *[other] Gata: cunoaște toate cele { $count } de sunete
    }
settings-clap-learning = Gata: se învață sunetele, { $done } din { $total } până acum

## SettingsPage.vue: Etichetare AI

settings-ai-service = Serviciu
settings-ai-off = Dezactivat
settings-ai-off-hint = Pachetele sunt etichetate doar cu regulile integrate.
settings-ai-local = Model local
settings-ai-local-hint = Serviciul llm din fișierul compose al Hoard (sau cargo xtask llm). Descărcați un model mai jos; rulează pe acest computer și nimic nu pleacă de pe el.
settings-ai-chatgpt = Abonament ChatGPT
settings-ai-chatgpt-hint = Folosește abonamentul dvs. ChatGPT Go, Plus, Pro sau Business în loc de o cheie API.
settings-ai-openrouter-hint = O singură cheie pentru sute de modele găzduite, unele gratuite. Modelele care acceptă imagini sunt marcate.
settings-ai-openai-hint = Găzduit de OpenAI.
settings-ai-anthropic-hint = Găzduit de Anthropic, prin endpointul său compatibil OpenAI.
settings-ai-gemini-hint = Găzduit de Google, prin endpointul său compatibil OpenAI.
settings-ai-ollama-hint = Un server Ollama pe care îl rulați dvs. Schimbați adresa dacă nu este un container numit ollama.
settings-ai-custom = Altul (compatibil OpenAI)
settings-ai-custom-hint = Orice înțelege API-ul de chat OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Se conectează…
settings-ai-connected =
    { $count ->
        [one] Conectat · { $count } model
        [few] Conectat · { $count } modele
       *[other] Conectat · { $count } de modele
    }
settings-ai-connected-none = Conectat, dar nu listează niciun model; tastați numele modelului
settings-chatgpt-connected-none = Conectat; tastați numele modelului
settings-ai-takes-images = acceptă imagini
settings-ai-saved = Etichetare AI: { $service }
settings-ai-saved-off = Etichetarea AI este dezactivată
settings-ai-reset = Etichetare AI: înapoi la valoarea implicită
settings-ai-on = Activată: { $service } · { $model }
settings-ai-url = Adresă
settings-ai-key = Cheie API
settings-ai-key-saved = Salvată (tastați pentru a înlocui)
settings-ai-key-paste = Lipiți cheia
settings-ai-key-optional = Niciuna (opțional)
settings-ai-key-remove = Eliminați
settings-ai-get-key = Obțineți o cheie
settings-ai-model = Model
settings-ai-model-pick = Alegeți sau tastați un model
settings-ai-model-name = Numele modelului
settings-ai-refresh = Reîmprospătați
settings-ai-connect = Conectați
settings-ai-need-key = Adăugați cheia API pentru a-i vedea modelele
settings-ai-vision = Trimiteți și miniaturi (necesită un model care acceptă imagini)
settings-ai-batch = Per cerere
settings-ai-batch-unit = resurse (mai puține pentru modelele locale mici, mai multe pentru cele mari găzduite)

## SettingsPage.vue: Conectarea la ChatGPT

settings-chatgpt-account = Cont
settings-chatgpt-checking = Se verifică…
settings-chatgpt-status-as = Conectat ca { $email }
settings-chatgpt-status = Conectat
settings-chatgpt-signed-in-as = Conectat la ChatGPT ca { $email }
settings-chatgpt-signed-in = Conectat la ChatGPT
settings-chatgpt-signed-out = Deconectat de la ChatGPT
settings-chatgpt-continue = Continuați cu ChatGPT
settings-chatgpt-sign-out = Deconectați-vă
settings-chatgpt-sign-out-title = Vă deconectați de la ChatGPT?
settings-chatgpt-sign-out-in-use = Etichetarea AI folosește abonamentul dvs. ChatGPT, așa că se oprește până când vă conectați din nou sau alegeți alt serviciu.
settings-chatgpt-sign-out-body = Hoard uită conectarea și îi cere lui OpenAI să o încheie.
settings-chatgpt-manage = Gestionați utilizarea
settings-chatgpt-finish-before = Finalizați conectarea în fereastra care s-a deschis (
settings-chatgpt-finish-link = deschideți-o din nou
settings-chatgpt-finish-middle = ). Dacă se termină pe o pagină care nu se încarcă, la o adresă care începe cu{ " " }
settings-chatgpt-finish-after = , copiați adresa respectivă și lipiți-o aici:
settings-chatgpt-paste = Adresa la care s-a terminat conectarea
settings-chatgpt-finish = Finalizați
settings-chatgpt-usage = Cererile de etichetare se socotesc în utilizarea abonamentului dvs. ChatGPT. Gestionați utilizarea arată ce a folosit Hoard și vă permite să limitați. Pentru a deconecta Hoard din ChatGPT însuși, accesați Settings → Security and login → Sign in with ChatGPT pe chatgpt.com.

## SettingsPage.vue: Modele locale

settings-local-downloading-progress = Se descarcă { $percent }% · { $done } din { $total }
settings-local-downloading = Se descarcă…
settings-local-incomplete = Descărcarea s-a oprit
settings-local-failed = Nu a pornit data trecută
settings-local-in-use-running = În uz · rulează
settings-local-in-use = În uz
settings-local-starting = Se pornește…
settings-local-running = Rulează
settings-local-ready-images = Gata · acceptă imagini
settings-local-ready-text = Gata · doar text
settings-local-delete-title = Ștergeți { $name }?
settings-local-delete-in-use = Etichetarea AI folosește acest model, așa că se oprește până când alegeți altul. Fișierele lui sunt șterse de pe serverul de modele.
settings-local-delete-downloading = Descărcarea se oprește, iar ce s-a descărcat până acum este șters.
settings-local-delete-body = Fișierele lui sunt șterse de pe serverul de modele. Îl puteți descărca din nou mai târziu.
settings-local-delete = Ștergeți
settings-local-deleting = Se șterge…
settings-local-cancel = Anulați
settings-local-resume = Reluați
settings-local-download = Descărcați
settings-local-start-before = Porniți-l cu{ " " }
settings-local-start-or = { " " }(sau{ " " }
settings-local-start-developing = { " " }în dezvoltare), apoi{ " " }
settings-local-start-retry = încercați din nou
settings-local-start-after = . Funcționează și un server care rulează un singur model: dați mai jos numele modelului său.
settings-local-list = Pe serverul de modele
settings-local-none = Încă nimic descărcat. Alegeți unul mai jos; este păstrat pe server și puteți schimba oricând.
settings-local-catalog = Modele de descărcat
settings-local-size = { $size } GB · { $license }
settings-local-other = Alt model de pe Hugging Face
settings-local-other-placeholder = Altul de pe Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Adăugarea pachetelor

settings-store-pages = Pagini de magazin
settings-store-pages-what = Pachetele noi sunt căutate în magazine în fundal, ca să primească pagina de magazin, vânzătorul și licența.
settings-store-lookup = Căutați automat paginile de magazin
settings-unreal = Pachete Unreal
settings-unreal-reset = Pachete Unreal: înapoi la valoarea implicită
settings-unreal-what = Ce să se facă cu fișierele .uasset.
settings-conv = Reguli de conversie
settings-conv-what = Convertiți fișierele păstrate de Hoard într-un singur format, de exemplu toate OBJ și FBX în GLB, ca biblioteca să fie uniformă. Fișierul convertit ia locul originalului, cu numele, etichetele, colecțiile și linkurile sale. Fișierele din dosarele urmărite nu sunt modificate niciodată.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Activă
settings-conv-to = Convertiți în
# Heading over the file formats a rule converts.
settings-conv-from = Din
settings-conv-remove = Eliminați
settings-conv-remove-label = Eliminați regula { $n }
settings-conv-taken = O altă regulă convertește deja acest format
settings-conv-upright = Îndreptați modelele la conversie (Y în sus, cu fața spre +Z), așa cum sunt afișate modelele pachetului
settings-conv-flac-what = Sunetul rămâne exact la fel, în aproximativ jumătate din spațiu, iar tot restul din WAV (puncte de buclă, marcaje cue, etichete) este păstrat în FLAC, așa că WAV-ul poate fi pus înapoi exact cum era oricând. Fișierele WAV cu eșantioane în virgulă mobilă sau pe 32 de biți rămân WAV.
settings-conv-add = Adăugați o regulă
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Păstrați originalele timp de
settings-conv-keep-days =
    { $count ->
        [one] zi
        [few] zile
       *[other] de zile
    }
settings-conv-keep-what = Până atunci, pagina unui fișier convertit poate restaura originalul. Texturile nu sunt copiate în modelul convertit: acesta trimite la fișierele de imagine de lângă el. Fișierele .mtl sau .bin ale unui model rămân.
settings-conv-save = Salvați regulile
settings-conv-revert = Anulați modificările
settings-conv-existing = Convertiți fișierele aflate deja în bibliotecă
settings-conv-saved = Regulile de conversie au fost salvate. Se aplică pachetelor adăugate de acum înainte.
settings-conv-converting =
    { $count ->
        [one] Se convertește un fișier în fundal
        [few] Se convertesc { $count } fișiere în fundal
       *[other] Se convertesc { $count } de fișiere în fundal
    }
settings-conv-nothing = Nimic din bibliotecă nu trebuie convertit.
settings-conv-ask-title =
    { $count ->
        [one] Convertiți un fișier aflat deja în bibliotecă?
        [few] Convertiți { $count } fișiere aflate deja în bibliotecă?
       *[other] Convertiți { $count } de fișiere aflate deja în bibliotecă?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Se află într-un singur pachet.
        [few] Se află în { $packs } pachete.
       *[other] Se află în { $packs } de pachete.
    } Fiecare original este păstrat { $days ->
        [one] { $days } zi
        [few] { $days } zile
       *[other] { $days } de zile
    }, iar până atunci poate fi restaurat din pagina sa.
settings-conv-ask-busy =
    { $count ->
        [one] Unul se află într-un pachet care încă se adaugă; va fi convertit după ce se termină.
        [few] { $count } se află în pachete care încă se adaugă; vor fi convertite după ce se termină.
       *[other] { $count } se află în pachete care încă se adaugă; vor fi convertite după ce se termină.
    }
settings-conv-ask-list = Fișiere care ar fi convertite
settings-conv-ask-more = …și încă { $count }
settings-conv-ask-skipped = Rămân așa cum sunt:
settings-conv-skip-watched =
    { $count ->
        [one] un fișier dintr-un dosar urmărit
        [few] { $count } fișiere din dosare urmărite
       *[other] { $count } de fișiere din dosare urmărite
    }
settings-conv-skip-unity =
    { $count ->
        [one] un fișier pe care un proiect Unity îl cunoaște după nume
        [few] { $count } fișiere pe care un proiect Unity le cunoaște după nume
       *[other] { $count } de fișiere pe care un proiect Unity le cunoaște după nume
    }
settings-conv-skip-used =
    { $count ->
        [one] un fișier menționat de o scenă sau de alt fișier
        [few] { $count } fișiere menționate de o scenă sau de alt fișier
       *[other] { $count } de fișiere menționate de o scenă sau de alt fișier
    }
settings-conv-skip-beside =
    { $count ->
        [one] un fișier a cărui copie convertită se află deja în pachetul său
        [few] { $count } fișiere ale căror copii convertite se află deja în pachetele lor
       *[other] { $count } de fișiere ale căror copii convertite se află deja în pachetele lor
    }
settings-conv-skip-blender =
    { $count ->
        [one] un fișier pe care Blender nu l-a citit încă (Setări → Instrumente)
        [few] { $count } fișiere pe care Blender nu le-a citit încă (Setări → Instrumente)
       *[other] { $count } de fișiere pe care Blender nu le-a citit încă (Setări → Instrumente)
    }
settings-conv-ask-new = Salvați, doar pachete noi
settings-conv-ask-save-convert =
    { $count ->
        [one] Salvați și convertiți-l
        [few] Salvați și convertiți { $count } fișiere
       *[other] Salvați și convertiți { $count } de fișiere
    }
settings-conv-ask-convert =
    { $count ->
        [one] Convertiți-l
        [few] Convertiți { $count } fișiere
       *[other] Convertiți { $count } de fișiere
    }
settings-unreal-off = Lăsați-le așa
settings-unreal-off-hint = Convertiți din pagina unui pachet când doriți.
settings-unreal-keep = Convertiți, păstrați originalele
settings-unreal-keep-hint = Texturile, sunetele și materialele ies în formate comune; rămâne și .uasset.
settings-unreal-replace = Convertiți, renunțați la originale
settings-unreal-replace-hint = Economisește spațiu după ce conținutul a fost extras. Mesh-urile tot așteaptă workerul Unreal.

## SettingsPage.vue: Descărcări

settings-tn = Numele texturilor
settings-tn-saved = Numele texturilor au fost salvate
settings-tn-reset = Numele texturilor: înapoi la valoarea implicită
settings-tn-what = Hărțile unui material pleacă denumite după el și după rolul fiecărei hărți, oricum ar fi denumite fișierele: Rock_Normal.png, nu rock_moss_xeokah0_raw_8K_Normal.png. Fișierele din bibliotecă nu sunt atinse, iar o hartă pe care o numește un model sau un MTL trimis împreună cu ea își păstrează numele, ca modelul să o găsească în continuare.
settings-tn-toggle = Denumiți hărțile de textură după materialul lor
settings-tn-style = Stil
settings-tn-style-group = Stilul de denumire
settings-tn-descriptive = Descriptiv
settings-tn-short = Scurt
settings-tn-custom = Propriul dvs.
settings-tn-pattern = Șablon
settings-tn-hint-name = { " " }materialul,{ " " }
settings-tn-hint-map = { " " }rolul hărții,{ " " }
settings-tn-hint-size = { " " }mărimea ei când setul vine în mai multe mărimi (adăugată la sfârșit dacă șablonul o omite).
settings-tn-example = Exemplu
settings-tn-suffixes = Sufixe pentru fiecare tip de hartă
settings-tn-slots = În descărcările GLB, o textură împachetată într-un model, fără un fișier propriu, primește numele materialului și al slotului pe care îl ocupă, iar aceste sloturi au și ele sufixe:
settings-tn-endings = Terminații proprii
settings-tn-endings-what = O textură care nu este una dintre hărțile unui material își păstrează numele, cu excepția cazului în care se termină cu una dintre acestea: _diff → _BC trimite rock_diff.png ca rock_BC.png. Lăsați a doua goală pentru a elimina terminația.
settings-tn-ending-from = Terminație
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Înlocuiți cu
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Adăugați o terminație
settings-tn-ending-remove = Eliminați
settings-tn-ending-remove-label = Eliminați { $ending }

## SettingsPage.vue: General și Conexiune

settings-name = Nume
settings-name-what = Cum se numește acest server în bara de sus, în fila browserului și pe pagina de conectare, ca să le puteți deosebi dacă aveți mai multe servere Hoard.
settings-name-what-desktop = Cum se numește această bibliotecă în bara de sus.
settings-address = Adresă
settings-address-what = Adresa pe care o folosesc persoanele și aplicațiile pentru a ajunge la Hoard, când nu este cea pe care se află acest browser: a unui reverse proxy, a unui tunel sau un nume din tailnet, de exemplu https://hoard.example.com. Linkurile pe care le oferă Hoard (către Claude și alte aplicații, la conectare, în certificatul său) o folosesc.
settings-public-url = Adresă publică
settings-public-url-placeholder = Din adresa browserului (de ex. https://hoard.example.com)
settings-public-url-default = din adresa browserului

## ProfilerPanel.vue: profilerul din partea de sus a Setări → Performanță

profiler = Profiler
profiler-what = Ce a făcut Hoard în ultimele zece minute, actualizat la fiecare două secunde cât timp priviți. Se păstrează doar în memorie și o ia de la capăt când Hoard repornește.
profiler-starting = Primele citiri sosesc într-o clipă.
profiler-failed = Profilerul nu a putut fi citit: { $error }
profiler-cpu = Utilizarea procesorului
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Întreaga mașină
profiler-cpu-summary = Hoard folosește { $hoard } din procesoare, iar întreaga mașină { $machine }.
profiler-ram = RAM
profiler-ram-peak = Vârf
profiler-ram-of = { $used } din { $total }
profiler-ram-summary = Hoard folosește { $hoard } RAM și a folosit cel mult { $peak } în ultimele zece minute.
profiler-serving = Cereri în curs
profiler-working = Lucrează la
profiler-working-none = Nimic nu a rulat în fundal.
profiler-working-more =
    { $count ->
        [one] Și încă una.
        [few] Și încă { $count }.
       *[other] Și încă { $count }.
    }
profiler-requests = Timp petrecut servind cereri
profiler-requests-what = Fiecare bară înseamnă zece secunde de cereri, cu timpul împărțit după unde s-a dus.
profiler-part-db = Bază de date
profiler-part-db-title = Așteptarea bazei de date și rularea interogărilor ei
profiler-part-disk = Disc
profiler-part-disk-title = Citirea fișierelor stocate și comprimarea a ceea ce se trimite
profiler-part-send = Trimitere
profiler-part-send-title = Așteptarea cât timp rețeaua preia ce s-a trimis
profiler-part-other = Altele
profiler-part-other-title = Elaborarea răspunsului
profiler-request-count =
    { $count ->
        [one] { $count } cerere
        [few] { $count } cereri
       *[other] { $count } de cereri
    }
profiler-requests-none = Nicio cerere în ultimele zece minute.
profiler-slowest = Cea mai lentă: { $route }, { $time }
profiler-routes = Pe rută
profiler-route = Rută
profiler-count = Cereri
profiler-average = Medie
profiler-p95 = Cele mai lente 5%
profiler-split = Unde s-a dus timpul
profiler-sent = Trimis
profiler-portal = fișierele proprii ale interfeței

## SettingsPage.vue: Performanță

settings-render = Miniaturi 3D
settings-render-toggle = Randați miniaturi pentru modelele 3D
settings-gpu-on = Pe GPU: { $renderer }
settings-gpu-software = Nu s-a găsit niciun GPU, deci în software ({ $renderer })
settings-gpu-none = Nu s-a găsit niciun GPU sau randare software, deci modelele nu primesc miniaturi
settings-thumbs = Miniaturi
settings-thumbs-what = Pentru miniaturi și vederi de modele noi. Cele transparente și pixel art rămân PNG. Micșorare a miniaturilor din Întreținere le refac pe cele existente.
settings-previews-size = Previzualizări: { $size }
settings-thumbs-saved = Miniaturi: { $quality }
settings-thumbs-format = Format
settings-thumbs-quality = Calitate { $quality }.
settings-thumbs-small = Mai mici
settings-thumbs-small-hint = Bune în grilă; puțin neclare de aproape.
settings-thumbs-balanced = Echilibrate
settings-thumbs-balanced-hint = Greu de deosebit de original la mărimea unei miniaturi.
settings-thumbs-sharp = Mai clare
settings-thumbs-sharp-hint = Cam de două ori mai mari decât Echilibrate.
settings-thumbs-lossless = Fără pierderi
settings-thumbs-lossless-hint = PNG, exact cum au fost create. De câteva ori mai mari. Nu sunt trimise niciodată ca AVIF.
settings-thumbs-avif-what = AVIF are adesea jumătate din dimensiune. Fiecare miniatură este convertită prima dată când este afișată.
settings-thumbs-avif-now = Convertește-le pe toate acum
settings-thumbs-avif-converting = Se convertește…
settings-motion-thumbs = Miniaturi în mișcare pentru animații
settings-motion-thumbs-what = Fiecare animație primește și un film scurt în buclă, redat cât timp indici dala ei. Ocupă cam 40 KB fiecare și durează un timp să fie create pentru o bibliotecă mare. Dezactivarea le șterge.
settings-limits = Lucru și limite
settings-limits-what = Fiecare se salvează când părăsiți câmpul sau apăsați Enter. Mărimile au unități: 100 GB, 512 MB, 1,5 TB; un număr simplu înseamnă GB.
settings-workers = Procese de fundal
settings-workers-unit = sarcini simultan (preluare, etichetare, conversii)
settings-max-upload = Cea mai mare încărcare
settings-max-upload-unit = per fișier
settings-max-expand = Cel mai mare pachet
settings-max-expand-unit = după dezarhivare
settings-conversion-cache = Descărcări convertite
settings-conversion-cache-unit-now = păstrate cel mult, acum { $size }; cele folosite cel mai demult pleacă primele
settings-conversion-cache-unit = păstrate cel mult; cele folosite cel mai demult pleacă primele

## SettingsPage.vue: Întreținere

settings-updates = Actualizări
settings-version = Aceasta este versiunea Hoard { $version }.
settings-update-check = Verificați GitHub la câteva ore pentru o versiune nouă
settings-desktop-updates = Hoard caută o versiune nouă la fiecare pornire, o descarcă în fundal și o instalează când îl închizi.
settings-backups = Copii de rezervă programate
settings-backups-what = Baza de date își face singură copii de rezervă; fiecare apare în istoricul de mai jos.
settings-backup-every = Fă copie
settings-backup-off = Niciodată
settings-backup-daily = În fiecare zi
settings-backup-weekly = În fiecare săptămână
settings-backup-keep = Păstrează
settings-backup-keep-unit = cele mai noi copii programate
settings-backup-copy-to = Copiază și în
settings-backup-copy-to-none = Niciun alt dosar
settings-backups-kept = Fiecare ajunge în dosarul de copii ca hoard-auto-…db și, dacă există, în celălalt dosar (alt disc sau unul pe care îl salvează o aplicație de sincronizare), dat prin calea completă de pe server. Se șterg doar copiile programate mai vechi; cele făcute de mână rămân.
about-what = Hoard păstrează resursele pentru jocuri (modele, texturi, sunete, muzică, sprite-uri și altele) într-o singură bibliotecă în care puteți căuta, previzualiza și din care puteți lua, partajată cu toți cei cărora le dați acces.
about-version = Versiune
about-edition = Ediție
about-edition-server = Server
about-edition-desktop = Aplicație desktop
about-edition-steam = Aplicație desktop de pe Steam
about-made-by = Creat de
about-source = Codul sursă pe GitHub
about-issues = Raportați o problemă
about-discord = Vorbește cu noi pe Discord
about-steam = Hoard pe Steam
about-license = Licență
about-license-what = Hoard este open source sub licența MIT No Attribution: îl puteți folosi, modifica și distribui cum doriți.
about-downloaded = Descărcate la nevoie
about-downloaded-what = Hoard nu vine cu acestea. Descarcă fiecare dintre ele prima dată când ceva are nevoie de el, iar fiecare are propria licență.
about-blender = Convertește modele 3D și le creează miniaturile.
about-ffmpeg = Convertește sunet și video și le creează previzualizările.
about-siglip = Găsește imagini după ce se vede în ele.
about-clap = Găsește sunete după cum sună.
about-models = Modelele pe care le alegeți pentru serverul AI local sunt supuse licențelor autorilor lor.
about-built-with = Construit cu
about-built-with-what = Bibliotecile open source din care este făcut Hoard, cu licențele lor.
about-steamworks = Versiunea Steam include și Steamworks SDK, © Valve Corporation.
about-licenses-loading = Se încarcă lista…
about-licenses-failed = Lista nu a putut fi încărcată.
about-filter = Filtrați după nume sau licență
about-count = { $shown } din { $total }
about-no-license = Nicio licență indicată
about-report = Raportează o problemă…
report-title = Raportează o problemă
report-what = Iată de ce are nevoie un issue pe GitHub ca problema să poată fi cercetată. Nu conține nimic privat: tokenurile de acces, parolele, cheile, adresele de e-mail și numele tău din căile folderelor sunt eliminate. Citește-l înainte să-l trimiți.
report-loading = Se adună raportul…
report-failed = Raportul nu a putut fi adunat: { $error }
report-log = Ce a înregistrat serverul în ultima vreme
report-copy = Copiază
report-copied = Raportul a fost copiat.
report-github = Deschide un issue pe GitHub
report-github-hint = Noul issue se completează cu acestea. Dacă jurnalul e prea lung, intră doar ultimele lui rânduri; Copiază le are pe toate.
report-close = Închide

## Asset kinds (api.ts)

api-kind-texture = Texturi
api-kind-vector = Grafică vectorială
api-kind-sheet = Foi de sprite-uri
api-kind-audio = Audio
api-kind-model = Modele
api-kind-animation = Animații
api-kind-font = Fonturi
api-kind-shader = Shadere
api-kind-material = Materiale
api-kind-video = Video
api-kind-document = Documente
api-kind-data = Date
api-kind-other = Altele
api-kind-texture-name = texturi
api-kind-vector-name = grafică vectorială
api-kind-sheet-name = foi de sprite-uri
api-kind-audio-name = audio
api-kind-model-name = modele
api-kind-animation-name = animații
api-kind-font-name = fonturi
api-kind-shader-name = shadere
api-kind-material-name = materiale
api-kind-video-name = video
api-kind-document-name = documente
api-kind-data-name = date
api-kind-other-name = altele

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Anulați
ui-today = azi
ui-yesterday = ieri
ui-days-ago =
    { $days ->
        [one] acum { $days } zi
        [few] acum { $days } zile
       *[other] acum { $days } de zile
    }
ui-notifications = Notificări
ui-dismiss = Închideți
ui-cancel = Anulați
ui-lazy-loading = Se încarcă { $what }…
ui-lazy-failed = Nu s-a putut încărca { $what }.
ui-lazy-reload = Reîncărcați pagina

## ResetButton.vue

reset-title = Înapoi la valoarea implicită: { $value }
reset-label = Înapoi la valoarea implicită, { $value }

## SizeInput.vue

size-invalid = O mărime precum 100 GB, 512 MB sau 1,5 TB
size-hint = O mărime precum 100 GB, 512 MB sau 1,5 TB; un număr simplu înseamnă GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = Setările de autentificare nu au putut fi încărcate: { $error }
auth-saved = Salvat. Se aplică de la următoarea conectare.
auth-method-internal = Intern
auth-method-internal-hint = Nume și parole păstrate de Hoard, adăugate în Setări → Cont.
auth-method-ldap-hint = Un director verifică numele și parola tastate pe pagina de conectare: OpenLDAP, FreeIPA, LLDAP sau alt server LDAP.
auth-method-ad-hint = Conturile din domeniul Windows se conectează cu numele și parola obișnuite.
auth-method-oauth-hint = Un buton „Conectați-vă cu…” care duce la Google, Microsoft, GitHub, Keycloak, Authentik sau alt furnizor și înapoi.
auth-session-unused = După 30 de zile de nefolosire
auth-session-1h = La o oră după conectare
auth-session-8h = La 8 ore după conectare
auth-session-day = La o zi după conectare
auth-session-week = La o săptămână după conectare
auth-session-30d = La 30 de zile după conectare
auth-session-hours =
    { $hours ->
        [one] La { $hours } oră după conectare
        [few] La { $hours } ore după conectare
       *[other] La { $hours } de ore după conectare
    }
auth-role-view = Poate vedea
auth-role-edit = Poate modifica
auth-role-admin = Administrator
auth-role-word-view = poate vedea
auth-role-word-edit = poate modifica
auth-role-word-admin = administrator
auth-role-word-default = { $role } (rolul implicit)
auth-tls-starttls-hint = LDAP simplu (portul 389), trecut la TLS înainte de a se trimite ceva.
auth-tls-ldaps-hint = TLS de la început (portul 636).
auth-tls-none = Niciuna
auth-tls-none-hint = Parolele trec prin rețea așa cum sunt. Doar într-o rețea în care aveți încredere.
auth-test-failed = Nu s-a putut testa: { $error }
auth-preset-custom = Altul (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel și altele: dați emitentul, iar Descoperiți completează adresele.
auth-preset-google-note = Creați un client OAuth (Web application) în Google Cloud Console → APIs & Services → Credentials. Google nu trimite grupuri, așa că dați rolurile în Setări → Cont și dezactivați crearea conturilor la prima conectare, dacă nu trebuie să intre oricine are un cont Google.
auth-preset-microsoft-note = Înregistrați o aplicație în Entra ID → App registrations (cu o redirecționare Web către adresa de mai jos). Puneți ID-ul tenantului dvs. în locul lui „organizations” ca să intre doar organizația dvs. Pentru roluri din grupuri, adăugați claimul groups în Token configuration: listează ID-urile grupurilor.
auth-preset-github-note = Creați o aplicație OAuth în GitHub → Settings → Developer settings. GitHub nu trimite grupuri, așa că dați rolurile în Setări → Cont și dezactivați crearea conturilor la prima conectare, dacă nu trebuie să intre oricine are un cont GitHub.
auth-preset-gitlab-note = Creați o aplicație în GitLab → Preferences → Applications (sau în zona de administrare a instanței dvs.: schimbați gitlab.com în adrese).
auth-discovered = Adresele furnizorului au fost completate.
auth-copied = Copiat
auth-copy-failed = Nu s-a putut copia: selectați textul și copiați-l singur
auth-sign-in-with = Conectați-vă cu
auth-own-accounts-still-work = Conturile proprii Hoard se conectează în continuare cu parolele lor, așa că un director sau un furnizor care nu funcționează sau este configurat greșit nu vă poate bloca accesul.
auth-sessions = Sesiuni
auth-sign-out-after = Deconectați persoanele
auth-ldap-server = Server LDAP
auth-server-type = Tipul serverului
auth-server-type-custom = Personalizat
auth-servers = Servere
auth-server-n = Serverul { $n }
auth-remove-server-n = Eliminați serverul { $n }
auth-remove = Eliminați
auth-add-server = Adăugați un server
auth-servers-hint = Următorul este încercat când unul nu poate fi accesat.
auth-security = Securitate
auth-connection-security = Securitatea conexiunii
auth-skip-verify = Nu verificați certificatul serverului (doar pentru încercări)
auth-ca = Autoritate de certificare
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (gol: cele în care are încredere sistemul)
auth-lookup = Căutarea persoanelor
auth-anonymously = Anonim
auth-reader-account = Cont de citire
auth-reader-password = Parola lui
auth-saved-type-to-replace = Salvată: tastați pentru a înlocui
auth-people-under = Persoanele sunt sub
auth-name-attribute = Atributul numelui
auth-only-matching = Doar intrările care se potrivesc cu
auth-groups-under = Grupurile sunt sub
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (gol: doar memberOf)
auth-their-groups = Grupurile lor
auth-group-filter-hint = { "{" }dn{ "}" } este intrarea persoanei, { "{" }user{ "}" } numele cu care s-a conectat.
auth-test = Test
auth-test-hint = Accesează fiecare server și se conectează cu contul de citire. Cu un nume și o parolă, se conectează și ca acea persoană și arată ce ar înțelege Hoard din asta. Nu se salvează nimic.
auth-test-user = Cineva ca care să vă conectați
auth-test-user-placeholder = Cineva ca care să vă conectați (opțional)
auth-test-password = Parola acelei persoane
auth-testing = Se testează…
auth-test-connectivity = Testați conectivitatea
auth-test-server-ok = accesat, iar contul de citire s-a conectat
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }s-a conectat (
auth-test-signed-in-after = ).
auth-test-role = Ar fi: { $role }.
auth-test-no-role = Nu face parte din niciunul dintre grupurile listate la Roluri, așa că ar fi refuzată.
auth-test-groups = Grupuri: { $groups }
auth-test-no-groups = Nu s-a găsit niciun grup.
auth-provider = Furnizor
auth-redirect = Adresa de redirecționare
auth-copy = Copiați
auth-redirect-hint = Dați-i asta furnizorului când creați clientul Hoard. Provine din adresa publică din Setări → Conexiune sau din adresa pe care o folosiți.
auth-issuer = Emitent
auth-discover = Descoperiți
auth-client-id = ID client
auth-client-secret = Secret client
auth-authorization-url = Adresa de autorizare
auth-token-url = Adresa tokenului
auth-userinfo-url = Adresa informațiilor despre utilizator
auth-userinfo-placeholder = (gol: tokenul ID spune cine este)
auth-scopes = Scopes
auth-name-from = Numele din
auth-groups-from = Grupurile din
auth-groups-from-placeholder = (niciunul)
auth-button = Buton
auth-button-placeholder = contul dvs.
auth-accounts = Conturi
auth-provision = Creați un cont Hoard prima dată când se conectează cineva
auth-provision-on-hint = Altfel, un administrator adaugă mai întâi fiecare persoană, în Setări → Cont.
auth-provision-off-hint = Un administrator adaugă mai întâi fiecare persoană, în Setări → Cont, alegând cum se conectează.
auth-open-to-anyone = Poate intra oricine are un cont { $provider }. Adăugați mai jos grupuri de vizualizatori sau dezactivați asta și adăugați persoanele singur.
auth-new-accounts = Conturi noi
auth-roles-from-groups = Roluri din grupuri
auth-roles-hint = Grupurile ai căror membri primesc fiecare rol, câte unul pe rând, după nume sau DN. Câștigă cel mai înalt care se potrivește și se verifică la fiecare conectare. Dacă nu este listat niciunul, conturile noi primesc rolul de mai sus, iar un administrator îl schimbă apoi. Dacă sunt listate grupuri la Poate vedea, se pot conecta doar membrii unui grup listat.
auth-roles-hint-oauth = Grupurile ai căror membri primesc fiecare rol, câte unul pe rând, după nume sau DN (sau ce listează claimul groups). Câștigă cel mai înalt care se potrivește și se verifică la fiecare conectare. Dacă nu este listat niciunul, conturile noi primesc rolul de mai sus, iar un administrator îl schimbă apoi. Dacă sunt listate grupuri la Poate vedea, se pot conecta doar membrii unui grup listat.
auth-saving = Se salvează…
auth-save = Salvați

## HttpsPanel.vue (Settings → Connection)

https-load-failed = HTTPS nu a putut fi încărcat: { $error }
https-off-redirecting = HTTPS este dezactivat. Vă ducem la http://
https-now-on = HTTPS este activat
https-saved = Salvat. Conexiunile noi folosesc certificatul nou.
https-now-off = HTTPS este dezactivat
https-http3-failed-toast = HTTP/3 nu a putut porni: { $error }
https-copied = Copiat
https-copy-failed = Nu s-a putut copia: selectați textul și copiați-l singur
https-mode-off = Dezactivat
https-mode-off-hint = Doar http:// simplu.
https-mode-own = Certificatul propriu Hoard
https-mode-own-hint = Hoard creează unul pentru adresele acestui server. Instalați o singură dată autoritatea de certificare Hoard pe fiecare dispozitiv, iar browserele lui vor avea încredere în el.
https-mode-custom = Certificatul meu
https-mode-custom-hint = Unul pe care îl aveți deja, de ex. de la tailscale cert sau Let’s Encrypt. Browserele au încredere în el fără să instalați nimic.
https-untrusted-own = Acest browser încă nu are încredere în el pentru { $host }: instalați mai jos autoritatea de certificare Hoard. Veți vedea un avertisment pe care îl puteți trece.
https-untrusted-uncovered = Acest browser încă nu are încredere în el pentru { $host }: certificatul nu acoperă această adresă. Veți vedea un avertisment pe care îl puteți trece.
https-untrusted = Acest browser încă nu are încredere în el pentru { $host }. Veți vedea un avertisment pe care îl puteți trece.
https-now = Acum
https-start-failed = HTTPS este configurat, dar nu a putut porni: { $error }
https-on-port = HTTPS este activat, pe portul { $port }, la fel ca http://.
https-using-it = Îl folosiți acum.
https-open = Deschideți { $url }
https-trusted = Acest browser are încredere în el.
https-cert-for-before = Certificat pentru{ " " }
https-cert-for-after = , valabil până la { $date }.
https-cert-for-after-own = , valabil până la { $date } (se reînnoiește singur).
https-not-covered-before = Nu acoperă{ " " }
https-not-covered-after = , adresa pe care o folosiți.
https-is-off = HTTPS este dezactivat: paginile sunt servite prin http:// simplu.
https-addresses = Adrese
https-addresses-hint = Certificatul acoperă adresele găsite de Hoard și pe cele pe care le adăugați (un nume dat de router, un nume Tailscale).
https-remove-name = Eliminați { $name }
https-another-address = Altă adresă
https-another-address-placeholder = Altă adresă, de ex. nas.local
https-add = Adăugați
https-your-cert = Certificatul dvs.
https-saved-cert-before = Salvat:{ " " }
https-saved-cert-after = { " " }de la { $issuer }, valabil până la { $date }. Lipiți unul nou pentru a-l înlocui.
https-pem-before = Certificatul (cu lanțul său) și cheia sa privată, ca text PEM: fișierele care încep cu{ " " }
https-pem-after = .
https-certificate = Certificat
https-choose-file = Alegeți fișierul…
https-private-key = Cheie privată
https-checked-before = Pentru{ " " }
https-checked-after = , de la { $issuer }, valabil până la { $date }.
https-checked-after-days =
    , de la { $issuer }, valabil până la { $date } ({ $days ->
        [one] { $days } zi
        [few] { $days } zile
       *[other] { $days } de zile
    }).
https-self-signed = Este autosemnat: browserele vor avertiza în privința lui.
https-key-matches = Cheia se potrivește.
https-where-to-get = De unde obțineți unul
https-get-tailscale-before = activați HTTPS pentru tailnetul dvs., apoi rulați{ " " }
https-get-tailscale-after = { " " }pe server și încărcați cele două fișiere. Sunt valabile 90 de zile: rulați comanda din nou și încărcați-le din nou înainte de atunci.
https-get-acme-title = Let’s Encrypt sau alt client ACME
https-get-acme-before = (certbot, acme.sh, lego) pentru un domeniu pe care îl dețineți: încărcați{ " " }
https-get-acme-and = { " " }și{ " " }
https-get-acme-after = .
https-get-proxy-title = Un reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) care are certificatul: lăsați HTTPS dezactivat aici și lăsați proxy-ul să trimită http:// simplu către Hoard.
https-trying = Se încearcă…
https-apply = Aplicați
https-apply-hint = Hoard încearcă mai întâi configurarea nouă și o păstrează pe cea pe care o are dacă nu funcționează.
https-trust-title = Aveți încredere în Hoard pe dispozitivele dvs.
https-download-ca = Descărcați hoard-ca.crt
https-trust-hint = Instalați o singură dată autoritatea de certificare Hoard pe fiecare dispozitiv, iar browserele lui vor avea încredere în acest server (și în orice certificat pe care Hoard îl creează pentru el mai târziu). Poate garanta doar pentru acest Hoard. Amprentă (SHA-256):
https-ca-windows = Deschideți fișierul → Instalare certificat → Utilizator curent → Plasați toate certificatele în următorul depozit → Răsfoire → Autorități de certificare rădăcină de încredere → Terminare. Reporniți browserul.
https-ca-macos = Deschideți fișierul; ajunge în Keychain Access. Faceți dublu clic acolo pe „{ $subject }” → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone și iPad
https-ca-ios = Descărcați-l în Safari și permiteți-l. Configurări → Profil descărcat → Instalează. Apoi Configurări → General → Despre → Configurări încredere certificate → activați „{ $subject }”.
https-ca-android = Setări → Securitate → Mai multe setări de securitate → Criptare și date de conectare → Instalați un certificat → Certificat CA și alegeți fișierul. (Denumirile diferă puțin de la un telefon la altul; căutați „certificat” în Setări.)
https-ca-linux-1 = Debian și Ubuntu:{ " " }
https-ca-linux-2 = . Fedora și Arch:{ " " }
https-ca-linux-3 = . Chrome pe Linux are propria listă: Setări → Confidențialitate și securitate → Securitate → Gestionați certificatele → Importați.
https-ca-firefox = Firefox are propria listă: Setări → Confidențialitate și securitate → Certificate → Vezi certificatele → Autorități → Importă și bifați „Această autoritate de certificare poate identifica site-uri web”.
https-ca-cli-title = Instrumentul de linie de comandă hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , sau puneți calea în{ " " }
https-ca-cli-after = .
https-options = Opțiuni
https-redirect = Trimiteți browserele de pe alte dispozitive de la http:// la https://
https-redirect-hint = Doar încărcările de pagini sunt redirecționate. Instrumentul de linie de comandă hoard, workerul Unreal, scripturile și aplicațiile AI funcționează în continuare prin http://, la fel și http://localhost pe serverul însuși.
https-http3-hint-before = Mai rapid pe Wi-Fi instabil și pe telefoane. Necesită HTTPS și portul UDP { $port } deschis pe lângă TCP (în Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Rulează.
https-http3-waits = Pornește când HTTPS este activat.
https-http3-failed = HTTP/3 nu a putut porni: { $error }
https-lockout-title = Dacă rămâneți fără acces
https-lockout-1 = { "" }
https-lockout-2 = { " " }pe serverul însuși funcționează întotdeauna prin http:// simplu. Sau dezactivați HTTPS din linia de comandă a serverului și porniți din nou Hoard:{ " " }
https-lockout-3 = { " " }(în Docker:{ " " }
https-lockout-4 = , apoi{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Poate vedea
account-role-view-hint = Caută, previzualizează și descarcă
account-role-edit = Poate modifica
account-role-edit-hint = De asemenea adaugă, modifică, etichetează, adună în colecții și arhivează
account-role-admin = Administrator
account-role-admin-hint = De asemenea Setări, conturi și întreținere, precum și ștergerea definitivă
account-source-internal = O parolă Hoard
account-source-ldap = Parola sa { $directory }
account-source-oauth = Conectare cu { $provider }
account-source-word-directory = director
account-load-failed = Conturile nu au putut fi încărcate: { $error }
account-failed = { $what }: { $error }
account-password-failed = Parola dvs. nu a putut fi schimbată
account-password-changed = Parola a fost schimbată. Celelalte dispozitive ale dvs. sunt deconectate.
account-add-failed = { $name } nu a putut fi adăugat
account-added = { $name } se poate conecta acum.
account-role-failed = Nu s-a putut schimba ce poate face { $name }
account-remove-title = Eliminați { $name }?
account-remove-body = Este deconectat peste tot și nu se mai poate conecta. Nimic din ce a adăugat în bibliotecă nu este eliminat.
account-remove = Eliminați
account-remove-failed = { $name } nu a putut fi eliminat
account-token-failed = Tokenul nu a putut fi creat
account-copied = Copiat
account-copy-token-failed = Nu s-a putut copia: selectați tokenul și copiați-l singur
account-copy-failed = Nu s-a putut copia: selectați textul și copiați-l singur
account-revoke-title = Revocați „{ $name }”?
account-revoke-body = Orice îl mai folosește este refuzat de acum înainte.
account-revoke = Revocați
account-revoke-failed = „{ $name }” nu a putut fi revocat
account-disconnect-title = Deconectați { $app }?
account-disconnect-body = Este refuzată de acum înainte și trebuie conectată din nou pentru a folosi Hoard.
account-disconnect = Deconectați
account-disconnect-failed = { $app } nu a putut fi deconectată
account-guest-failed = Nu s-a putut schimba asta
account-never = niciodată
account-you = Dvs.
account-sign-out = Deconectați-vă
account-signed-in-as-before = Conectat ca{ " " }
account-signed-in-as-admin = , care este administrator.
account-signed-in-as-edit = , care poate modifica biblioteca.
account-signed-in-as-view = , care poate vedea, dar nu poate modifica nimic.
account-app-signin = Aplicația Hoard te conectează singură, așa că nu există parolă. Creează mai jos un token de acces pentru pluginurile motoarelor și comanda hoard.
account-password-at-provider = Parola dvs. este păstrată de furnizorul cu care vă conectați, așa că o schimbați acolo.
account-password-at-directory = Parola dvs. este păstrată de directorul cu care vă conectați, așa că o schimbați acolo.
account-current-password = Parola actuală
account-new-password = Parola nouă
account-new-password-again = Parola nouă din nou
account-change-password = Schimbați parola
account-password-mismatch = Parolele noi nu se potrivesc.
account-password-rule = Cel puțin 8 caractere.
account-people = Persoane
account-people-hint = Fiecare persoană poate vedea (căutare, previzualizare și descărcare), poate și modifica biblioteca (adăugare, editare, etichetare, colecții și arhivare) sau poate fi și administrator (Setări, conturi și întreținere, precum și ștergerea definitivă).
account-you-tag = (dvs.)
account-signs-in-oauth = Se conectează prin furnizorul OAuth
account-signs-in-directory = Se conectează prin director
account-since = din { $date }
account-what-can-do = Ce poate face { $name }
account-name = Nume
account-how-sign-in = Cum se conectează
account-password-label = Parolă (cel puțin 8 caractere)
account-password-placeholder = Parolă (8+ caractere)
account-what-they-can-do = Ce poate face
account-add = Adăugați
account-ai-apps = Claude și alte aplicații AI
account-ai-apps-hint = Conectați Claude (sau orice aplicație MCP) și va putea căuta și prelua resurse pentru dvs., precum și descrie și eticheta ce Hoard nu a făcut încă. Se conectează la Hoard prima dată; dvs. îi permiteți accesul aici, în browser.
account-mcp-address = Adresa MCP a Hoard
account-copy = Copiați
account-claude-code-1 = rulați{ " " }
account-claude-code-2 = , apoi{ " " }
account-claude-code-3 = { " " }pentru a vă conecta.{ " " }
account-claude-code-4 = { " " }descrie un pachet.
account-claude-ai = Settings → Connectors → Add custom connector, lipiți adresa, apoi Connect. Acestea se conectează de pe serverele Anthropic, așa că Hoard trebuie să fie accesibil din internet prin HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, lipiți adresa, apoi Connect. Acestea se conectează de pe serverele Anthropic, așa că Hoard trebuie să fie accesibil din internet prin HTTPS (această adresă nu este: setați Adresa publică în Setări → Conexiune).
account-connection-meta = pentru { $user } din { $since } · folosită ultima dată: { $used }
account-no-apps = Încă nicio aplicație conectată.
account-live = Folosesc Hoard acum
account-live-hint = Aplicațiile care au cerut ceva de la Hoard în ultimele 10 minute. Se actualizează singură.
account-live-none = Nimic nu a cerut nimic de la Hoard în ultimele 10 minute.
account-live-last = ultima: { $what } · { $when } · cereri: { $calls }
account-tokens = Tokenuri de acces
account-tokens-hint-1 = Pentru instrumentul de linie de comandă{ " " }
account-tokens-hint-2 = , scripturi, clienți MCP și workerul Unreal: îl trimit ca{ " " }
account-tokens-hint-3 = . Un token poate face ce alegeți, până la ce puteți face dvs., dar niciodată nu poate gestiona conturi sau tokenuri. Unul care doar vede este suficient pentru căutare și descărcare și pentru sincronizarea unui proiect.
account-tokens-admin = Vedeți tokenurile și aplicațiile conectate ale tuturor; ceilalți le văd doar pe ale lor.
account-made-before = { "" }
account-made-after = : copiați-l acum. Nu va mai fi afișat.
account-new-token = Tokenul nou
account-cli-1 = Pentru CLI:{ " " }
account-cli-2 = , sau puneți-l în{ " " }
account-cli-3 = .
account-done = Gata
account-token-meta = creat { $made } · folosit ultima dată: { $used }
account-token-meta-by = creat { $made } de { $by } · folosit ultima dată: { $used }
account-no-tokens = Încă niciun token.
account-token-name = Ce îl va folosi
account-token-name-placeholder = Ce îl va folosi, de ex. „Worker Unreal”
account-token-role = Ce poate face
account-make-token = Creați un token
account-guests = Vizitatori
account-guests-allow = Permiteți oricui poate accesa serverul să răsfoiască fără să se conecteze
account-guests-hint = Pot căuta, previzualiza și descărca, dar nu pot adăuga, modifica sau șterge nimic și nu pot vedea Setările.

## SignIn.vue

signin-create-title = Creați-vă contul
signin-title = Conectare
signin-setup-lede = Acest server nu are încă niciun cont. Cel pe care îl creați aici poate face orice; adăugați mai târziu conturi pentru alte persoane din Setări → Cont.
signin-overlay-lede = Aici oricine poate răsfoi, dar pentru modificări este nevoie de un cont.
signin-with = Conectați-vă cu { $provider }
signin-or-password = sau cu un nume și o parolă
signin-username = Nume de utilizator
signin-name = Nume
signin-new-password = Parolă nouă
signin-password = Parolă
signin-password-rule = Cel puțin 8 caractere.
signin-confirm-password = Confirmați parola
signin-mismatch = Cele două parole nu se potrivesc.
signin-setup-code = Cod de configurare
signin-setup-code-hint-before = Codul primit de acest server în{ " " }
signin-setup-code-hint-after = .
signin-creating = Se creează…
signin-signing-in = Se conectează…
signin-create = Creați contul
signin-submit = Conectați-vă
signin-keep-browsing = Continuați să răsfoiți
signin-directory-hint = Folosiți numele și parola dvs. { $directory }.
signin-forgot-1 = Ați uitat o parolă Hoard? Pe server, rulați{ " " }
signin-forgot-2 = { " " }(în Docker,{ " " }
signin-forgot-3 = ) pentru una nouă.

## SetupWizard.vue (first visit to a new server)

setup-title = Unde să stea biblioteca dvs.?
setup-lede = Alegeți un dosar pe server în care Hoard să vă păstreze pachetele sau indicați-i un dosar cu pachete pe care le aveți deja. Puteți adăuga mai târziu alte locații, pe alte discuri, din Setări.
setup-lede-desktop = Alegeți un dosar în care Hoard să vă păstreze pachetele sau indicați-i un dosar cu pachete pe care le aveți deja. Puteți adăuga mai târziu alte locații, pe alte discuri, din Setări.
setup-unreachable = Serverul nu a putut fi accesat: { $error }
setup-unreachable-desktop = Hoard nu răspunde: { $error }
setup-try-again = Încercați din nou
setup-loading = Se încarcă…
setup-start = Porniți biblioteca
setup-ready = Biblioteca dvs. este gata
setup-managed-before = Pachetele noi ajung în{ " " }
setup-managed-after = . Plasați fișiere, dosare sau arhive oriunde pe pagină pentru a le adăuga pe primele.
setup-watched-before = Hoard urmărește{ " " }
setup-watched-after = ; pachetele apar pe măsură ce le găsește. Pentru a putea și încărca, adăugați o bibliotecă păstrată de Hoard din Setări → Locații ale bibliotecii.
setup-open = Deschideți biblioteca

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = Conectați { $app }?
connect-lede-1 = { " " }va putea să caute, să descarce, să descrie și să eticheteze tot ce este în acest Hoard, acționând în numele contului{ " " }
connect-lede-2 = . Nu poate gestiona conturi sau tokenuri de acces.
connect-back-before = Veți reveni la{ " " }
connect-back-after = . O puteți deconecta oricând din Setări → Cont.
connect-allow = Permiteți
connect-cancel = Anulați
connect-failed = Această aplicație nu poate fi conectată
connect-go-library = Mergeți la bibliotecă
connect-checking = Se verifică…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = Aceasta este o conexiune http:// simplă, iar câteva lucruri au nevoie de HTTPS: 3D cu WebGPU, clipboardul modern, instalarea Hoard ca aplicație și conectorii Claude.
insecure-switch = Treceți la https://
insecure-set-up = Configurați HTTPS
insecure-dismiss = Nu mai afișați asta

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Acest fișier este prea mare pentru a fi citit ca fișier de magazin.
formatwizard-not-json = Acest fișier nu este JSON valid, așa că este afișat ca text.
formatwizard-read-failed = Fișierul nu a putut fi citit: { $error }
formatwizard-back-title = Reveniți la formular?
formatwizard-back-body = Modificările făcute în TOML se pierd: formularul scrie din nou formatul din ce are.
formatwizard-back-to-form = Înapoi la formular
formatwizard-miss-name = Fișierele sale nu acceptă numele { $file }. Adăugați-l la Fișiere.
formatwizard-miss-not-the-stores = Elementul după care recunoaște fișierele magazinului nu este în acest fișier. Schimbați Recunoaștere după.
formatwizard-miss-no-title = Nu găsește niciun titlu în acest fișier. Alegeți titlul.
formatwizard-miss-short-name = nu are numele potrivit
formatwizard-miss-short-not-the-stores = nu este al magazinului
formatwizard-miss-short-no-title = fără titlu
formatwizard-saved-packs =
    S-a salvat { $file }. { $packs ->
        [one] { $packs } pachet este citit din nou.
        [few] { $packs } pachete sunt citite din nou.
       *[other] { $packs } de pachete sunt citite din nou.
    }
formatwizard-saved = S-a salvat { $file }.
formatwizard-save-failed = Formatul nu a putut fi salvat: { $error }
formatwizard-zip-failed = Arhiva zip nu a putut fi creată: { $error }
formatwizard-stop-title = Opriți învățarea acestui fișier?
formatwizard-stop-body = Formatul nu este salvat.
formatwizard-stop = Opriți
formatwizard-label = Asistent pentru formate
formatwizard-changing = Se modifică { $name }
formatwizard-starting-from = Pornind de la { $name }
formatwizard-title = Învățați Hoard fișierul unui magazin
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }în{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Editați ca TOML
formatwizard-file-reference = Fișierul, ca referință.
formatwizard-click-json = Faceți clic în fișier pe valoarea pentru { $field }.
formatwizard-click-text = Faceți clic în fișier pe valoarea pentru { $field } sau selectați-o.
formatwizard-pick-first = Alegeți un câmp în dreapta, apoi faceți clic aici pe valoarea lui.
formatwizard-loading = Se încarcă…
formatwizard-the-format = Formatul
formatwizard-syntax-before = Sintaxa este descrisă în{ " " }
formatwizard-syntax-after = .
formatwizard-store = Magazin
formatwizard-store-placeholder = Numele magazinului, de ex. Kenney
formatwizard-files = Fișiere
formatwizard-recognize = Recunoaștere după
formatwizard-recognize-json = O cheie pe care o au doar fișierele magazinului
formatwizard-recognize-text = Un rând pe care îl au doar fișierele magazinului
formatwizard-recognize-hint = Câte un criteriu pe rând; oricare dintre ele este suficient. Necesar când alte fișiere au același nume.
formatwizard-seller = Vânzător
formatwizard-seller-placeholder = Când fișierul nu precizează
formatwizard-tags = Etichete
formatwizard-tags-placeholder = Pentru tot ce descrie, separate prin virgulă
formatwizard-fields = Câmpuri
formatwizard-selector = Selector pentru { $field }
formatwizard-clear = Goliți { $field }
formatwizard-pick = Alegeți
formatwizard-what-it-reads = Ce citește
formatwizard-read-title = Titlu
formatwizard-read-seller = Vânzător
formatwizard-read-license = Licență
formatwizard-read-url = Pagina de magazin
formatwizard-read-kind = Tip
formatwizard-read-tags = Etichete
formatwizard-read-origin = Creat în
formatwizard-read-size = Mărime
formatwizard-also = Și { $format } citește acest fișier. Acest format ar fi încercat primul.
formatwizard-library =
    Cu un nume pe care îl acceptă: { $files ->
        [one] { $files } fișier
        [few] { $files } fișiere
       *[other] { $files } de fișiere
    } din { $packs ->
        [one] { $packs } pachet
        [few] { $packs } pachete
       *[other] { $packs } de pachete
    }. Din { $tried } încercate, citește { $read }.
formatwizard-library-none = Niciun fișier din bibliotecă nu are un nume pe care să-l accepte.
formatwizard-share = Împărtășiți-l
formatwizard-share-hint = O arhivă zip cu formatul, acest fișier ca exemplu și ce se citește din exemplu, organizată ca depozitul Hoard, pe care o puteți trimite ca să o primească toată lumea. Scoateți mai întâi din exemplu tot ce nu ar trebui împărtășit.
formatwizard-sample = Exemplu de împărtășit
formatwizard-download = Descărcați { $file }
formatwizard-close = Închideți
formatwizard-use = Folosiți-l aici
formatwizard-cancel = Anulați
settings-unreal-editor-what = Instalează Unreal Editor pentru procesul de export. Rulează-l separat pentru a exporta resurse.
settings-unreal-download = Descarcă o singură dată editorul Linux x86-64 al Epic în dosarul de instrumente Hoard. Ocupă zeci de GB pe disc și necesită un cont GitHub acceptat de Epic.
settings-unreal-github-step-link-go = Cum le asociați
settings-unreal-github-step-token = Creați un token GitHub (classic) cu doar read:packages bifat.
settings-unreal-github-step-token-go = Creați un token
settings-unreal-github-user = Nume de utilizator GitHub
settings-unreal-github-token = Token
settings-unreal-github-token-kept = Salvat (tastați unul nou pentru a-l înlocui)
settings-unreal-github-save = Salvați conectarea
settings-unreal-github-forget = Deconectează
settings-unreal-github-saved = Conectarea GitHub a fost salvată; se reîncearcă descărcarea
settings-unreal-github-forgotten = GitHub deconectat
settings-unreal-github-from-server = Acest server a primit și o conectare proprie (GHCR_TOKEN_FILE sau DOCKER_CONFIG); aceasta se folosește când aici nu este salvată niciuna.
settings-unreal-github-connect = Conectează GitHub…
settings-unreal-github-connect-title = Conectează GitHub
settings-unreal-github-connect-why = Epic oferă Unreal Editor conturilor GitHub legate de Epic Games. Hoard cere doar citirea pachetelor.
settings-unreal-github-sign-in = Conectează-te cu GitHub
settings-unreal-github-enter-code = După clic, GitHub se deschide într-o filă nouă. Introdu acolo acest cod.
settings-unreal-github-waiting = Se așteaptă GitHub…
settings-unreal-github-expired = Codul a expirat.
settings-unreal-github-denied = Conectarea a fost anulată pe GitHub.
settings-unreal-github-failed = Nu s-a putut obține un cod de la GitHub.
settings-unreal-github-again = Obține un cod nou
settings-unreal-github-before = Înainte de a începe
settings-unreal-github-before-what = Leagă GitHub de contul tău Epic Games și acceptă invitația EpicGames pe care ți-o trimite GitHub pe e-mail.
settings-unreal-github-use-token = Folosește un token
settings-unreal-github-use-sign-in = Conectează-te cu GitHub
settings-unreal-github-no-password = Nicio parolă nu ajunge la Hoard
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = Conectat ca { $user }; se reîncearcă descărcarea
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = Conectat ca { $user }
settings-unreal-github-as-what = Unreal Editor se descarcă folosind acest cont GitHub.
settings-unreal-github-none = GitHub nu este conectat
settings-unreal-github-none-what = Necesar pentru descărcarea editorului.
settings-unreal-github-change = Schimbă…
settings-unreal-image = Versiune sau imagine Unreal
settings-editor-path = Introdu calea executabilului pentru { $tool } instalat. Instalarea automată este disponibilă doar în Docker.
error-unreal-image = Introdu o versiune Unreal 5, o etichetă de versiune Epic dev/dev-slim sau o referință de imagine Epic SHA-256.
error-github-no-app = Acest server nu are o aplicație GitHub prin care să te conectezi; folosește un token.
error-github-unreachable = GitHub nu poate fi accesat.
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub a refuzat conectarea: { $reason }
