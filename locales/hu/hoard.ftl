## Szerverhibák

error-account-not-found = a fiók nem található
error-asset-gone = az asset nem található
error-asset-not-found = nem található asset: { $id }
error-blob-not-found = nem található blob: { $hash }
error-collection-not-found = nem található gyűjtemény: { $id }
error-connection-not-found = a kapcsolat nem található
error-derivative-not-found = nem található származtatott fájl ({ $kind }) ehhez: { $id }
error-file-in-pack-not-found = nem található a csomagban ({ $id }): { $path }
error-site-not-a-page = csak weboldalak nyílnak meg oldalként
error-site-link-expired = az oldal címe lejárt; nyissa meg újra a Hoardból
error-format-not-found = nem található formátum: { $file }
error-frame-sheet-aseprite-only = csak az Aseprite-fájloknak van képkockalapja
error-frame-sheet-frames = ennek egyetlen képkockája van, vagy túl sok ahhoz, hogy el lehessen rendezni
error-location-not-found = nem található tárolási hely: { $id }
error-maintenance-task-not-found = nem található karbantartási feladat: { $id }
error-open-upload-not-found = nem található nyitott feltöltés: { $id }
error-original-not-found = nem található az eredeti fájl ehhez: { $id }
error-pack-not-found = nem található csomag: { $id }
error-preview-too-big = ez a kép túl nagy ahhoz, hogy könnyű előnézet készüljön belőle
error-project-not-found = nem található projekt: { $name }
error-saved-search-not-found = nem található mentett keresés: { $id }
error-sign-in-first = előbb jelentkezzen be (vagy küldjön hozzáférési tokent)
error-those-assets-not-found = ezek az assetek nem találhatók
error-thumbnail-not-found = nem található bélyegkép ehhez: { $id }
error-token-not-found = a token nem található
error-upload-not-found = nem található feltöltés: { $id }
error-pack-needs-name = a csomagnak nevet kell adni
error-no-library = még nincs tárolási hely, ahová fájlokat lehetne menteni: adjon hozzá egy kezelt helyet a Beállítások → Tárolási helyek oldalon
error-upload-failed = a feltöltés sikertelen: { $reason }
error-upload-no-files = a feltöltésben nincsenek fájlok
error-analyze-nothing-to-do = nincs mit tenni: rules=false, és az LLM ki van kapcsolva
error-too-many-assets = túl sok asset egy kérésben
error-change-not-found = ez a változás nincs az asset előzményeiben
error-bulk-by-look = a megjelenés szerinti keresés nem szerkeszthető egyszerre: inkább jelöld ki az asseteket
error-too-many-items = túl sok elem egy kérésben
error-not-a-hash = nem blake3 hexadecimális hash
error-merge-into-tag = az into értéke csak címke lehet
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = A(z) „{ $text }” nem címke
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = A(z) { $parent } már a(z) { $tag } alá tartozik, ezért a(z) { $tag } nem rendelhető alá
error-nothing-to-merge = nincs mit összevonni
error-selection-size = adjon meg 1 és { $max } közötti számú asset-azonosítót
error-collection-needs-name = a gyűjteménynek nevet kell adni
error-collection-exists = már van ilyen nevű gyűjtemény: { $name }
error-collection-empty = a gyűjtemény üres: { $name }
error-conversion-failed = az átalakítás sikertelen ({ $format }): { $reason }
error-max-size = a max értéke 64 és 16384 közötti kettőhatvány lehet
error-replace-ext = az ext értéke rövid fájlkiterjesztés legyen
error-empty-file = üres fájl
error-imitation-silent = A felvételen semmi sem emelkedik ki a háttérből. Próbáld újra közelebb a mikrofonhoz.
error-replace-duplicate = az átalakított fájl megegyezik egy, a csomagban már meglévő fájllal; a kettőt összevontuk
error-ai-address = Az MI-szolgáltatás címe http:// vagy https:// kezdetű legyen
error-hosted-key = Adja meg a(z) { $provider } API-kulcsát
error-hosted-address = A képkereső szolgáltatás címe http:// vagy https:// kezdetű legyen
error-local-llm-action = az action értéke list, download vagy delete lehet, és az utóbbi kettőhöz modell is kell
error-role = A szerepkör lehet view, edit vagy admin
error-admin-only = ezt csak admin teheti meg
error-view-only = ez a fiók csak megtekinthet, módosítani semmit sem tud: ezt egy admin megváltoztathatja a Beállítások → Fiók oldalon
error-portal-only = a fiókok és tokenek kezeléséhez jelentkezzen be a portálra
error-not-from-portal = ez a kérés nem a Hoard portálról érkezett
error-account-name = Válasszon legfeljebb 64 karakteres nevet
error-password-length = Legalább { $min } karakteres jelszót használjon
error-too-many-codes = Túl sok hibás kód: várjon { $seconds } másodpercet
error-already-set-up = Már van fiók: jelentkezzen be
error-too-many-tries = Túl sok sikertelen próbálkozás: várjon { $seconds } másodpercet
error-directory-unavailable = A címtár nem tudta ellenőrizni a jelszavát: { $reason }
error-password-elsewhere = A jelszavát az a címtár vagy szolgáltató kezeli, amellyel bejelentkezik
error-current-password = A jelenlegi jelszava nem helyes
error-account-source = A fiók bejelentkezési módja: Hoard (internal), címtár (ldap) vagy OAuth
error-account-exists = Már van ilyen nevű fiók
error-own-role = A saját szerepkörét egy másik admin módosíthatja
error-remove-self = Nem törölheti a saját fiókját, amíg be van jelentkezve vele
error-token-name = Nevezze el a tokent arról, ami használni fogja (legfeljebb 80 karakter)
error-token-role = A token nem tehet többet, mint az a fiók, amely létrehozza
error-wrong-setup-code = Ez nem az a beállítási kód, amelyet a szerver a HOARD_SETUP_CODE változóban kapott
error-wrong-password = A név és a jelszó nem egyezik
error-folder-missing = { $path } nem található (talán nincs csatlakoztatva a lemez?)
error-s3-path = egy S3-hely így néz ki: s3://bucket
error-s3-address = az S3-cím http:// vagy https:// kezdetű legyen
error-s3-header = egy további S3-fejléc így néz ki: Name=value, a „{ $header }” viszont nem ilyen
error-keychain = nem sikerült megőrizni a rendszer jelszótárolójában: { $reason }
error-not-built = a Hoard ezen példánya { $feature } nélkül készült
error-openai-auth-unreachable = nem sikerült elérni: auth.openai.com
error-openai-unexpected = váratlan válasz az OpenAI-tól
error-openai-answered = az OpenAI válasza: { $status }: { $reason }
error-chatgpt-id-token-not-jwt = az ID token nem JWT
error-chatgpt-id-token-issuer = az ID token kiállítója { $issuer }, nem pedig { $expected }
error-chatgpt-id-token-audience = az ID token egy másik alkalmazásnak szól
error-chatgpt-id-token-expired = az ID token lejárt
error-chatgpt-id-token-nonce = az ID token nem ehhez a bejelentkezéshez tartozik
error-chatgpt-no-state = a címben nincs state; másolja ki a teljes címet
error-chatgpt-not-started = ezt a bejelentkezést nem itt indították, vagy több mint 15 perce indult. Indítsa újra a Beállításokban.
error-chatgpt-canceled = a bejelentkezést megszakították
error-chatgpt-refused = a ChatGPT elutasította a bejelentkezést: { $reason }
error-chatgpt-no-code = a címben nincs bejelentkezési kód; másolja ki a teljes címet
error-chatgpt-other-registration = a ChatGPT ennek a szervernek egy másik regisztrációjára válaszolt; indítsa újra a bejelentkezést
error-chatgpt-other-registration-desktop = a ChatGPT a Hoard egy másik példányának válaszolt; indítsa újra a bejelentkezést
error-chatgpt-not-registered = a ChatGPT nem fejezte be a Hoard regisztrálását; indítsa újra a bejelentkezést
error-chatgpt-code-used = a bejelentkezési kódot már felhasználták, vagy lejárt; jelentkezzen be újra
error-chatgpt-refused-code = a ChatGPT elutasította a bejelentkezést ({ $code })
error-chatgpt-no-id-token = a ChatGPT nem küldött ID tokent
error-chatgpt-plan-not-allowed = A bejelentkezés sikerült, de az előfizetés használatát nem engedélyezte. Jelentkezzen be újra, és engedélyezze, vagy használjon helyette API-kulcsot.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = nincs bejelentkezve a ChatGPT-be; jelentkezzen be a Beállítások → MI-címkézés oldalon
error-chatgpt-refresh-later = a ChatGPT most nem tudta frissíteni a bejelentkezést ({ $code }); később újra próbálkozunk
error-chatgpt-signed-out-problem = A ChatGPT kijelentkeztette ezt a szervert ({ $code }). Jelentkezzen be újra, hogy továbbra is az előfizetésével címkézhessen.
error-chatgpt-signed-out-problem-desktop = A ChatGPT kijelentkeztette a Hoardot ({ $code }). Jelentkezzen be újra, hogy továbbra is az előfizetésével címkézhessen.
error-chatgpt-signed-out = a ChatGPT kijelentkeztette ezt a szervert ({ $code }); jelentkezzen be újra a Beállítások → MI-címkézés oldalon
error-chatgpt-signed-out-desktop = a ChatGPT kijelentkeztette a Hoardot ({ $code }); jelentkezzen be újra a Beállítások → MI-címkézés oldalon
error-openai-api-unreachable = nem sikerült elérni: api.openai.com
error-openai-broke-off = az OpenAI válasza megszakadt
error-chatgpt-kept-refusing = az OpenAI újra és újra elutasította a ChatGPT-bejelentkezést; jelentkezzen be újra a Beállítások → MI-címkézés oldalon
error-chatgpt-request-failed = a kérés sikertelen
error-chatgpt-cut-off = a válasz félbeszakadt ({ $reason })
error-chatgpt-cut-off-no-reason = a válasz félbeszakadt (ok megadása nélkül)
error-openai-ended-early = az OpenAI válasza véget ért, mielőtt teljes lett volna
error-chatgpt-usage-limit = a ChatGPT-előfizetés Hoardra jutó kerete egyelőre elfogyott. A korlátot itt tekintheti meg vagy módosíthatja: { $url }
error-chatgpt-not-eligible = ez a ChatGPT-fiók vagy munkaterület nem használhatja az előfizetését más alkalmazásokban. Használjon helyette API-kulcsot.
error-chatgpt-usage-unavailable = a ChatGPT most nem tudta ellenőrizni az előfizetés használatát; később újra próbálkozunk
error-chatgpt-unsupported = ChatGPT-előfizetéssel ez a modell nem támogatja ezt: { $param }
error-chatgpt-unsupported-images = ChatGPT-előfizetéssel ez a modell nem támogatja ezt: { $param }; válasszon képeket fogadó modellt, vagy kapcsolja ki a bélyegképek küldését
error-chatgpt-unsupported-part = ChatGPT-előfizetéssel ez a modell nem támogatja a kérés egy részét
error-chatgpt-port = a port az, amelyen a böngésző eléri a Hoardot
error-chatgpt-paste-address = illessze be a teljes címet, amely így kezdődik: http://127.0.0.1
error-too-many-hashes = egyszerre legfeljebb { $max } hash
error-credits-pack = csomag: { $key }
error-credits-collection = gyűjtemény: { $key }
error-format-too-long = a formátum túl hosszú
error-not-started = a szerver nem indult el
error-not-started-desktop = A Hoard nem indult el
error-not-format-file = nem egy formátum fájlja: { $name }
error-format-name = a névben legyen legalább egy betű vagy számjegy (A–Z vagy 0–9)
error-format-miss-name = a formátum fájljai közé nem tartozik ez: { $file }
error-format-miss-store = a formátum ezt nem fogadja el az áruház fájljaként: { $file } (lásd: require)
error-format-miss-title = a formátum nem talál címet ebben: { $file }
error-cant-download-as = .{ $ext } fájlok nem tölthetők le ebben a formátumban: { $format }
error-only-images-resize = csak képek méretezhetők át
error-https-no-certificate = Előbb illessze be vagy töltse fel a tanúsítványt és a kulcsát
error-https-no-cert-found = Nem található tanúsítvány: illessze be a -----BEGIN CERTIFICATE----- kezdetű szöveget
error-https-no-key-found = Nem található privát kulcs: illessze be a -----BEGIN PRIVATE KEY----- kezdetű szöveget
error-https-several-keys = Több privát kulcs is van: csak azt illessze be, amelyik a tanúsítványhoz tartozik
error-https-key-password = A kulcs jelszóval védett: adja meg jelszó nélkül (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = A tanúsítvány nem érvényes PEM
error-https-key-not-pem = A privát kulcs nem érvényes PEM
error-https-key-mismatch = A kulcs nem ehhez a tanúsítványhoz tartozik. Ha láncot adott meg, a szervere tanúsítványa legyen az első
error-https-unusable = A tanúsítvány vagy a kulcs nem használható
error-https-expired = Ez a tanúsítvány lejárt (lejárat: { $date })
error-https-not-yet-valid = Ez a tanúsítvány még nem érvényes (érvényesség kezdete: { $date })
error-https-unreadable = Ez nem olyan tanúsítvány, amelyet a Hoard be tud olvasni
error-https-bad-name = „{ $name }” nem olyan cím, amelyet tanúsítvány tartalmazhat
error-https-test-timed-out = a tesztkapcsolat túllépte az időkorlátot
error-https-test-failed = a tesztkapcsolat sikertelen
error-https-no-ca = A Hoard még nem hozta létre a saját hitelesítésszolgáltatóját
error-https-unavailable = A HTTPS nem érhető el ezen a szerveren
error-https-certificate-mode = Tanúsítványt a „Saját tanúsítvány” beállítással lehet megadni
error-https-test = A tanúsítvány nem működött a tesztkapcsolatban: { $reason }
error-nothing-to-retry = itt nincs semmi, amit újra lehetne próbálni
error-ai-unreachable = nem sikerült elérni: { $url }
error-ai-key-refused = { $url } elutasította az API-kulcsot ({ $status })
error-ai-answered = { $url } válasza: { $status }: { $reason }
error-ai-not-json = { $url }/models nem JSON formátumban válaszolt
error-model-server-refused = a modellszerver: { $reason }
error-no-model-server = nem válaszol modellszerver ezen a címen: { $url }
error-not-llama-router = { $url } nem router módban futó llama.cpp szerver, ezért nem tud modelleket letölteni
error-model-download = a modell letöltése nem sikerült ({ $model }): { $reason }
error-model-delete = a modell törlése nem sikerült ({ $model }): { $reason }
error-unknown-kind = ismeretlen assettípus: { $kind }
error-watched-is-folder = a figyelt hely egy mappa ezen a szerveren
error-cant-reach = nem érhető el ({ $path }): { $reason }
error-folder-full-path = adja meg a mappa teljes elérési útját
error-cant-create = nem hozható létre: { $path }
error-cant-write = nem írható: { $path }
error-cant-read = nem olvasható: { $path }
error-overlaps-location = ez átfedésben van egy másik tárolási hellyel: { $name } ({ $path })
error-overlaps-data = ez átfedésben van a Hoard adatmappájával ({ $path })
error-overlaps-cache = ez átfedésben van a Hoard gyorsítótár-mappájával ({ $path })
error-not-watched = { $name } nem figyelt mappa
error-watched-no-kinds = a figyelt mappa nem fogad új fájlokat, így nincsenek módosítható típusai
error-keep-catch-all = előbb tegyen meg egy másik helyet gyűjtőhelynek
error-location-holds-files = { $name } még tartalmaz fájlokat: előbb helyezze át őket a többi helyre (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } nem érhető el, így a Hoard nem tudja megállapítani, tartalmaz-e még fájlokat
error-relocate-running = már folyamatban van fájlok áthelyezése a helyek között; próbálja újra, ha az befejeződött
error-watched-not-moved = { $name } figyelt mappa: a Hoard nem helyezi át a fájljait
error-no-other-location = nincs másik tárolási hely, ahová a fájlokat át lehetne helyezni: előbb adjon hozzá egyet
error-full-path = adjon meg teljes elérési utat
error-pick-folder = válasszon mappát
error-task-running = „{ $task }” már fut
error-job-cant-stop = „{ $job }” indítás után már nem állítható le
error-job-not-found = ez a feladat már nem vár és nem is fut
error-task-cant-stop = „{ $task }” nem állítható le menet közben
error-task-not-running = „{ $task }” nem fut
error-token-account-gone = ennek a tokennek a fiókja már nem létezik, így nincsenek kedvencei és keresései
error-favorites-too-many = egyszerre legfeljebb 10 000
error-saved-search-name = a mentett keresésnek legfeljebb 80 karakteres név kell
error-saved-search-too-long = ez a keresés túl hosszú a mentéshez
error-hoard-seen = a kincs legfeljebb 32 assetfajtát tarthat nyilván, mindegyiket kisbetűs névvel
error-texture-pattern-map = A textúranév-mintában szerepelnie kell ennek: { "{" }map{ "}" }, különben egy anyag minden térképe ugyanazt a nevet kapná
error-texture-pattern-fields = A textúranév-minta ezeket használhatja: { "{" }name{ "}" }, { "{" }map{ "}" } és { "{" }size{ "}" }
error-texture-name-chars = A textúranevek nem tartalmazhatják ezeket: < > : " / \ | ? vagy *
error-texture-map-type = Nincs ilyen térképtípus: { $role }
server-role-base-color = szín
server-role-normal = normál
server-role-normal-dx = normál (DirectX)
server-role-roughness = érdesség
server-role-gloss = fényesség
server-role-metalness = fémesség
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = magasság
server-role-bump = bump
server-role-cavity = üregesség
server-role-specular = spekuláris
server-role-opacity = átlátszatlanság
server-role-emissive = emisszió
server-role-translucency = áttetszőség
error-texture-suffix-missing = Adjon utótagot ennek a térképtípusnak: { $map }
error-texture-suffix-chars = A térképtípus utótagja ({ $map }) nem tartalmazhatja ezeket: < > : " / \ | ? vagy *
error-texture-ending-missing = Minden saját végződéshez meg kell adni a keresendő végződést
error-texture-ending-chars = A saját végződések nem tartalmazhatják ezeket: < > : " / \ | ? vagy *
error-texture-ending-twice = { $ending } kétszer szerepel a végződései között
error-oauth-no-client-id = Ebben a linkben nincs client_id.
error-oauth-unknown-app = Ez az alkalmazás nincs (már) regisztrálva itt. Kezdje újra a csatlakoztatást az alkalmazásból.
error-oauth-redirect = Az alkalmazás olyan címre kérte a visszairányítást, amelyet nem regisztrált.
error-oauth-response-type = A Hoard csak bejelentkezési kódokat ad ki (response_type=code).
error-oauth-pkce = Az alkalmazásnak S256-os PKCE-t kell használnia a Hoardba való bejelentkezéshez.
error-light-preview-images = csak képeknek van könnyű előnézete
error-no-light-preview = nincs könnyű előnézet: { $reason }
error-no-frame-sheet = nincs képkockalap: { $reason }
error-thumbnail-source = ismeretlen bélyegképforrás: { $source }
error-thumbnail-too-big = ez a kép túl nagy bélyegképnek
error-thumbnail-models-only = böngésző által rajzolt bélyegképet csak modellek kaphatnak
error-not-a-picture = nem olyan kép, amelyet a Hoard be tud olvasni: { $reason }
error-project-name = a projektnek legfeljebb 200 karakteres név kell
error-project-too-big = egy projektben legfeljebb egymillió asset lehet
error-project-too-many-at-once = egyszerre legfeljebb 100 000
error-server-name = Válasszon legfeljebb { $max } karakteres, egysoros nevet
error-tool-setting = { $tool }: válassza a Ki vagy az Automatikus lehetőséget, vagy adjon meg elérési utat
error-ai-model = Válassza ki, melyik modellt használja az MI-szolgáltatás
error-public-address = A nyilvános cím http:// vagy https:// kezdetű legyen
error-update-hook-address = A frissítésekkor hívandó cím http:// vagy https:// kezdetű legyen
error-update-hook-auth = Az Authorization fejléc nem tartalmazhat sortörést vagy más vezérlőkaraktert
error-no-soundfont = Nincs SoundFont a szerveren ezen az útvonalon: { $path }
error-no-soundfont-desktop = Nincs SoundFont ezen az útvonalon: { $path }
error-soundfont-bad = A(z) { $name } nem olyan SoundFont, amellyel a MIDI szólni tud: { $reason }
error-workers = Háttérfolyamatok: 1 és { $max } között
error-ai-batch = Assetek MI-kérésenként: 1 és 200 között
error-conversion-cache = Átalakított letöltések: legalább 100 MB
error-backup-keep = Ütemezett biztonsági mentések: 1–{ $max } megtartása
error-backup-copy-to = Ütemezett biztonsági mentések: { $path } nem mappa a szerveren (add meg a teljes útvonalát)
error-conversion-keep-days = Átalakítási szabályok: az eredetiek 1–{ $max } napig őrizhetők meg
error-conversion-target = Átalakítási szabályok: a Hoard nem tud fájlokat { $format } formátumra alakítani
error-conversion-no-source = Átalakítási szabályok: minden szabályhoz legalább egy átalakítandó formátum kell
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Átalakítási szabályok: a .{ $ext } fájlok nem alakíthatók { $format } formátumra
error-conversion-twice = Átalakítási szabályok: a .{ $ext } két szabályban is szerepel
error-conversion-cant-undo = Ezt a fájlt nem szabály alakította át, vagy az eredetijét már törölték.
error-conversion-path-taken = Már egy másik fájl van itt: { $path }, így az eredeti nem kerülhet vissza oda.
error-limits-zero = A korlátok nem lehetnek nullák
error-settings-object = a beállításoknak JSON-objektumban kell érkezniük
error-bad-setting = hibás beállítás: { $reason }
error-auth-default-role = Az alapértelmezett szerepkör view, edit vagy admin lehet
error-auth-session-hours = A munkamenet 1 óra és 1 év között tarthat
error-auth-settings = hibás hitelesítési beállítások: { $reason }
error-ldap-settings = hibás LDAP-beállítások: { $reason }
error-ldap-no-server = Adja meg a címtárszerver címét
error-ldap-base-dn = Adja meg, hol találhatók a felhasználók bejegyzései (base DN)
error-ldap-username-attribute = Adja meg, melyik attribútum tartalmazza a bejelentkezési nevet
error-ldap-bind-dn = Adja meg a felhasználókat kereső fiókot, vagy keressen névtelenül
error-ldap-ca-cert = A hitelesítésszolgáltató nem PEM-tanúsítvány (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Az egyik címtárszerver címe üres
error-ldap-server-form = { $server }: a címtárszerver címe host:port, ldap://… vagy ldaps://… alakú
error-ldap-not-starttls = { $server }: ez LDAPS, nem StartTLS
error-ldap-no-certificate = nincs benne tanúsítvány
error-ldap-password-again = Adja meg újra a felhasználókat kereső fiók jelszavát: a mentett jelszót csak azoknak a szervereknek küldjük el, amelyekhez megadták
error-ldap-unreachable = egyik címtárszerver sem érhető el ({ $reasons })
error-ldap-no-anonymous = a címtár nem engedi a felhasználók névtelen keresését ({ $reason })
error-ldap-bind-failed = a felhasználókat kereső fiók nem tudott bejelentkezni ({ $reason })
error-ldap-search = keresés: { $name }, itt: { $base }
error-ldap-group-search = csoportok keresése: { $name }, itt: { $base }
error-ldap-several = { $count } címtárbejegyzés egyezik erre a névre: { $name }; szűkítse a felhasználói szűrőt
error-ldap-password-check = a címtár nem ellenőrizte ennek a felhasználónak a jelszavát: { $name } ({ $reason })
error-ldap-wrong-password = A címtár nem ismer ilyen felhasználót ezzel a jelszóval: { $name }
error-oauth-client-id = Adja meg a kliensazonosítót (client ID), amelyet a szolgáltató a Hoardnak létrehozott
error-oauth-addresses = Az engedélyezési és a tokencím https:// kezdetű legyen
error-oauth-userinfo-address = A felhasználói adatok címe https:// kezdetű legyen
error-oauth-username-claim = Adja meg, melyik claim tartalmazza a felhasználó nevét
error-oauth-secret-again = Adja meg újra a kliens titkos kulcsát: a mentett kulcsot csak arra a tokencímre küldjük el, amelyhez megadták
error-oauth-issuer = A kiállító (issuer) https:// kezdetű legyen
error-oauth-discovery-unreachable = Nem sikerült elérni ({ $url }): { $reason }
error-oauth-discovery-status = { $url } válasza: { $status }
error-oauth-discovery-bad = { $url } nem discovery dokumentum
error-oauth-discovery-incomplete = { $url } nem adja meg az engedélyezési és a tokencímet
error-signin-no-group = { $name } nincs olyan csoportban, amely használhatja a Hoardot
error-signin-other-source = Már létezik ilyen nevű Hoard-fiók, amely másként jelentkezik be: { $name }
error-signin-other-subject = Már létezik ilyen nevű Hoard-fiók, amely a szolgáltatónál valaki máshoz tartozik: { $name }
error-signin-no-account = { $name } még nem rendelkezik Hoard-fiókkal: egy admin hozzáadhatja a Beállítások → Fiók oldalon
error-signin-bad-name = Ez a név nem lehet Hoard-fiók neve (legfeljebb 64 karakter)
error-oauth-off = Az OAuth-bejelentkezés nincs bekapcsolva
error-oauth-bad-authorization-address = az engedélyezési cím érvénytelen
error-oauth-refused = A szolgáltató nem jelentkeztette be: { $reason }
error-oauth-no-code = A szolgáltató bejelentkezés nélkül küldte vissza
error-oauth-expired = A bejelentkezés túl sokáig tartott, vagy máshol indították: próbálja újra
error-oauth-no-claim = A szolgáltató nem közölte, ki Ön (nincs { $claim } claim)
error-oauth-no-sub = A szolgáltató nem közölte, ki Ön (nincs sub vagy id claim)
error-oauth-token-unreachable = nem sikerült elérni a szolgáltató tokencímét
error-oauth-token-refused = A szolgáltató nem fejezte be a bejelentkezést: { $reason }
error-oauth-no-access-token = a szolgáltató nem küldött hozzáférési tokent
error-oauth-userinfo-unreachable = nem sikerült elérni a szolgáltató felhasználóiadat-címét
error-oauth-userinfo-refused = A szolgáltató nem közölte, ki jelentkezett be ({ $status })
error-oauth-userinfo-not-json = a szolgáltató felhasználói adatai nem JSON formátumúak
error-format-no-name = a name üres
error-format-no-files = a files üres: adja meg, mely fájlokat olvassa a formátum, például ["*.json"]
error-format-empty-selector = üres
error-format-unopened-bracket = záró zárójel nyitó zárójel nélkül
error-format-unclosed-bracket = egy zárójel nincs lezárva
error-format-empty-step-dots = üres lépés (két pont egymás után, vagy pont az elején vagy a végén)
error-format-unclosed-except = a !( nincs lezárva
error-format-unclosed-filter = a [ nincs lezárva
error-format-after-filters = fölösleges rész: { $rest }, ennek a kulcsa és szűrői után: { $step }
error-format-except-needs-glob = a !(…) csak *-ot tartalmazó kulcs után állhat
error-format-except-not-glob = a !(…) csak *-ot tartalmazó kulcs után állhat, ez pedig nem az: { $key }
error-format-empty-step = üres lépés
error-format-not-a-filter = [{ $filter }] nem szűrő: használja ezek egyikét: [], [key=value|other], [array], [string], [number], [bool] vagy [object]
error-format-filter-needs-value = [{ $filter }]: kulcs és érték is kell
error-store-search-empty = nincs mit keresni; írja be a csomag nevét
error-update-hook-not-set = előbb adja meg a hívandó címet
error-upload-stalled = elakadt a feltöltés ({ $path }); küldje el újra azt a részt
error-upload-busy = { $path } írása már folyamatban van
error-upload-incomplete = { $count } fájl hiányos, például: { $files }
error-upload-no-files-listed = nincsenek fájlok
error-upload-not-part = { $path } nem része ennek a feltöltésnek ({ $id }), vagy a feltöltés le van zárva
error-bad-path = hibás elérési út
error-upload-part-offset = { $path }: egyik rész sem ezen a pozíción kezdődik: { $offset }
error-upload-interrupted = a feltöltés megszakadt: { $reason }
error-upload-part-long = { $path }, { $part }. rész: hosszabb, mint { $len } bájt
error-upload-part-short = { $path }, { $part }. rész: { $got } bájt érkezett, { $len } volt várható
error-upload-file-long = { $path } nagyobb a megadottnál
error-upload-file-short = { $path }: { $got } bájt érkezett, { $len } volt várható
error-upload-complete = a feltöltés már befejeződött
error-already-imported = már importálva ezen a néven: „{ $name }”

## Szerverszövegek

server-store-place-readme = README
server-store-place-folders = mappák
server-store-why-linked = A csomag hivatkozik rá (hely: { $place })
server-store-why-same-title = Ugyanaz a cím, mint a csomagban (hely: { $place })
server-store-why-by = Szerző: { $author }, akit a csomag megnevez (hely: { $place })
server-store-why-meshes = Nagyjából annyi háló, mint a csomagban ({ $count })
server-store-why-textures = Nagyjából annyi textúra, mint a csomagban ({ $count })
server-chatgpt-revoke-failed = Itt kijelentkezett, de az OpenAI-t nem sikerült értesíteni. A biztonság kedvéért válassza le a Hoardot a ChatGPT-ben: Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Bejelentkezve a ChatGPT-be
server-chatgpt-signed-in = A Hoard bejelentkezett. Az MI-címkézés mostantól használhatja a ChatGPT-előfizetését.
server-chatgpt-signed-in-as = A Hoard bejelentkezett ezzel a fiókkal: { $email }. Az MI-címkézés mostantól használhatja a ChatGPT-előfizetését.
server-chatgpt-not-finished-title = A bejelentkezés nem fejeződött be
server-chatgpt-other-computer = Ha a bejelentkezést egy másik számítógépen futó Hoardból indította, másolja ki ennek az oldalnak a címét a címsorból, és illessze be ott a Hoard beállításaiba.
server-back-to-settings = Vissza a Beállításokhoz
server-issues-pack = Csomagok, amelyeket nem sikerült hozzáadni
server-issues-pack-blurb = Törölje a csomagot, és adja hozzá újra, vagy nyissa meg, hogy lássa, mi van benne.
server-issues-damaged = Sérült vagy eltűnt tárolt fájlok
server-issues-damaged-blurb = A „Fájlok sérülésének ellenőrzése” találta őket. Állítsa vissza őket biztonsági mentésből, vagy adja hozzá újra a csomagjukat, majd futtassa ismét az ellenőrzést.
server-issues-folder = Figyelt mappák, amelyek nem olvashatók
server-issues-folder-blurb = A csomagjaik változatlanok maradnak, amíg a mappa újra olvashatóvá nem válik.
server-issues-missing = Figyelt csomagok, amelyeknek eltűnt a mappája
server-issues-missing-blurb = Minden csomag 3 napig megmarad arra az esetre, ha a mappája csak offline lenne, utána törlődik.
server-issues-job = Sikertelen háttérfeladatok
server-issues-job-blurb = Az újrapróbálás elölről futtatja a feladatot.
server-issues-file = Fájlok, amelyeket nem sikerült megnyitni
server-issues-file-blurb = A Hoard nem tudta megnyitni vagy tárolni őket. A figyelt csomag a következő vizsgálatkor újra megpróbálja; a könyvtárban már meglévő fájl megtartotta a korábbi tartalmát.
server-issues-unpack = Tömörített fájlok, amelyeket nem sikerült kicsomagolni
server-issues-unpack-blurb = Mindegyik egyszerű fájlként marad a csomagjában, így a tartalmából semmi sincs a könyvtárban.
server-issues-read = Fájlok, amelyeket a Hoard nem tudott értelmezni
server-issues-read-blurb = Bekerültek a könyvtárba, de azok az adatok, címkék és bélyegkép nélkül, amelyeket a beolvasásuk adna.
server-issues-render = Modellek, amelyeknél a bélyegkép-renderelő hibát jelzett
server-issues-render-blurb = Nincs bélyegképük. A megjelenítőben megnyitva még működhetnek, és ott el is lehet menteni egyet.
server-issues-convert = Modellek, amelyeket a Blender nem tudott átalakítani
server-issues-convert-blurb = Nincs GLB-másolatuk, így nem jeleníthetők meg 3D-ben.
server-issues-unreal = Unreal-csomagfájlok, amelyeket nem sikerült átalakítani
server-issues-unreal-blurb = A csomag Unreal oldalán látható a többi csomagfájl.
server-issues-reformat = Fájlok, amelyeket egy átalakítási szabály nem tudott átalakítani
server-issues-reformat-blurb = Változatlanul maradtak.
server-issues-materials = Csomagok, amelyek anyagait nem sikerült kikövetkeztetni
server-issues-materials-blurb = A textúráik mind megvannak, csak nincsenek anyagokba csoportosítva.
server-issues-material-thumb = Anyagok bélyegkép nélkül
server-issues-material-thumb-blurb = Az anyag megvan, de az előnézetét nem sikerült megrajzolni.
server-issues-store-files = Áruházi fájlok, amelyeket nem sikerült beolvasni
server-issues-store-files-blurb = Annak az áruháznak a fájljai, ahonnan a csomag származik (például egy Megascans JSON), amelyek megnevezik és felcímkézik.
server-issues-scan-again = Újravizsgálás
server-issues-retry = Újrapróbálás
server-issues-reanalyze = Csomag újraelemzése
server-issues-convert-again = Átalakítás újra
server-issues-look-again = Újrakeresés
server-issues-read-again = Újraolvasás
server-issues-blob-missing = A tárolt fájl eltűnt.
server-issues-blob-damaged = A tárolt fájl nem egyezik azzal, amit eltároltunk: megsérült.
server-issues-folder-gone = A mappa nem található: „{ $folder }”.
server-job-a-pack = egy csomag
server-job-ingest = Feldolgozás: { $pack }
server-job-retag = Újracímkézés: { $pack }
server-job-llm-tag = Címkézés MI-vel: { $pack }
server-job-convert = Átalakítás Blenderrel: { $pack }
server-job-unreal = Unreal-csomagfájlok átalakítása: { $pack }
server-job-reformat = Fájlok átalakítása az átalakítási szabályok szerint: { $pack }
server-job-scan = Figyelt mappa vizsgálata
server-job-relocate = Fájlok áthelyezése a tárolási helyek között
server-job-looks = Színek és megjelenés elemzése
server-job-sounds = Hangok meghallgatása
server-job-embed = A képek tartalmának megtanulása
server-job-hear = A hangok hangzásának megtanulása
server-job-materials = Anyagok keresése: { $pack }
server-job-read-files = Fájlok beolvasása: { $pack }
server-job-store-lookup = Áruházi oldal keresése
server-maintenance-backup = Adatbázis biztonsági mentése
server-maintenance-backup-blurb = Konzisztens másolat az adatbázisról (csomagok, címkék, jegyzetek, gyűjtemények és beállítások) a mellette lévő backups mappában. A könyvtár fájljai nincsenek benne: tárolás után sosem változnak, így bármilyen fájlszinkronizálás menti őket.
server-maintenance-scrub = Fájlok sérülésének ellenőrzése
server-maintenance-scrub-blurb = Minden tárolt fájlt visszaolvas, és összeveti az ujjlenyomatával, hogy kiszűrje a lemezen vagy NAS-on megromlott vagy eltűnt fájlokat. Nagy könyvtárnál eltart egy ideig.
server-maintenance-reanalyze = Minden csomag újraelemzése
server-maintenance-reanalyze-blurb = A Hoard ezen verziójával minden fájlt újra beolvas: a bélyegképek, előnézetek, részletek és szabályalapú címkék újonnan készülnek el. A saját módosításai megmaradnak; az MI-t nem kérdezi meg újra.
server-maintenance-clear-cache = Gyorsítótárazott átalakítások törlése
server-maintenance-clear-cache-blurb = Eldobja a „Letöltés másként” átalakításokból megőrzött fájlokat (a következő kéréskor újra elkészülnek), és azt, amit a megszakadt munka hátrahagyott.
server-maintenance-shrink-thumbs = Bélyegképek zsugorítása
server-maintenance-shrink-thumbs-blurb = A PNG-ként tárolt bélyegképeket és modellnézeteket a Beállítások → Teljesítmény → Bélyegképek szerint tárolja, ami általában többszörösen kisebbé teszi őket. Az átlátszóságot tartalmazók és a pixel art PNG marad.
server-maintenance-avif-thumbs = Bélyegképek átalakítása AVIF-re
server-maintenance-avif-thumbs-blurb = Most alakítja át az összes bélyegképet, nem egyenként az első megjelenítéskor, ha a bélyegképek AVIF formátumban mennek ki.
server-maintenance-gc = Nem használt fájlok törlése
server-maintenance-gc-blurb = Törli azokat a tárolt fájlokat, amelyekre már semmi sem hivatkozik, például a végleg törölt csomagok fájljait.
server-update-no-address = nincs megadva hívandó cím
server-update-no-answer = 30 másodpercen belül nem érkezett válasz

## Beállítások: Megjelenés

appearance-language = Nyelv
appearance-language-auto = A böngészőé ({ $name })

## Gyökér

root-unreachable = Nem sikerült elérni a Hoard szervert: { $error }
root-unreachable-desktop = A Hoard nem válaszol: { $error }
root-desktop-reopen = Ez az ablak kijelentkezett. A folytatáshoz zárd be a Hoardot, majd nyisd meg újra.
root-try-again = Újra
root-lazy-connect = alkalmazáskapcsolat
window-minimize = Kis méret
window-maximize = Teljes méret
window-restore = Előző méret
window-close = Bezárás

## Alkalmazás: a könyvtár, a felső sáv és a szűrők

app-lazy-settings = beállítások
app-lazy-archive = archívum
app-lazy-shortcuts = billentyűparancsok
app-lazy-setup = kezdeti beállítás
app-theme-system = Rendszertéma követése
app-theme-light = Világos téma
app-theme-dark = Sötét téma
app-color-red = Piros
app-color-orange = Narancs
app-color-yellow = Sárga
app-color-green = Zöld
app-color-teal = Kékeszöld
app-color-blue = Kék
app-color-purple = Lila
app-color-pink = Rózsaszín
app-color-brown = Barna
app-color-sand = Homok
app-color-gray = Szürke
app-color-slate = Palaszürke
app-color-black = Fekete
app-color-white = Fehér
app-color-swatch = { $color }: sok { $lower } színt tartalmazó assetek
app-color-any = Bármilyen szín
app-color-pick = Tetszőleges szín választása
app-color = Szín
app-music = Zene
app-tempo = BPM
app-tempo-min = ettől
app-tempo-max = eddig
app-tempo-min-title = Ilyen vagy gyorsabb tempójú zene, percenkénti ütésben (a fele vagy a dupla is számít)
app-tempo-max-title = Ilyen vagy lassabb tempójú zene, percenkénti ütésben
app-key = Hangnem
app-key-any = Bármilyen hangnem
app-key-major = Dúr
app-key-minor = Moll
app-key-mix = És a vele keverhető hangnemek
app-key-mix-title = A párhuzamos dúr vagy moll hangneme és a kvinttel feljebb és lejjebb lévő hangnemek, amelyekkel a DJ-k keverik
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Írd le a hangot úgy, ahogy betűznéd: az ilyen hangzású hangok jönnek előre
app-imitation-record = Add ki a hangot a mikrofonba: az ilyen hangzású hangok jönnek előre
app-imitation-stop = Felvétel leállítása
app-imitation-heard = A te hangod
app-imitation-mic-failed = Nem sikerült felvenni a mikrofonról: { $error }
app-retry = Újra
app-undo = Visszavonás
app-search-failed = A keresés sikertelen: { $error }
app-unreachable = Nem sikerült elérni a szervert: { $error }
app-unreachable-desktop = A Hoard nem válaszol: { $error }
app-forget-project-title = Elfelejti ezt a projektet: { $name }?
app-forget-project-body = A Hoard nem jelzi tovább, hol használják a projekt { $count } assetét, amíg a következő hoard sync újra nem jelenti. A projektben semmi sem változik.
app-forget-project-confirm = Elfelejtés
app-forget-project-failed = Nem sikerült elfelejteni a projektet ({ $name }): { $error }
app-forget-project-label = Projekt elfelejtése: { $name }
app-forget-project = Projekt elfelejtése
app-save-name-like = hasonló: { $name }
app-save-name-favorites = kedvencek
app-search-saved = Mentve: „{ $name }”. A Szűrők között, a Saját alatt találja
app-search-updated = Frissítve: „{ $name }”
app-rank-alike = Leginkább hasonló
app-rank-color = Legtöbb ebből a színből
app-rank-match = Legjobb egyezés
app-rank-imitation = Leginkább így szól
app-sort = Rendezés
app-sort-name = Név
app-sort-created = Létrehozva
app-sort-updated = Frissítve
app-sort-size = Méret
app-sort-rating = Értékelés
app-sort-newest-first = Legújabb elöl
app-sort-oldest-first = Legrégebbi elöl
app-sort-a-to-z = A–Z
app-sort-z-to-a = Z–A
app-sort-largest-first = Legnagyobb elöl
app-sort-smallest-first = Legkisebb elöl
app-sort-best-rated-first = Legjobbra értékelt elöl
app-sort-lowest-rated-first = Legrosszabbra értékelt elöl
app-collection-added = { $count } asset hozzáadva ehhez a gyűjteményhez: { $name }
app-collection-add-failed = Nem sikerült hozzáadni a gyűjteményhez: { $error }
app-collection-removed = { $count } eltávolítva ebből a gyűjteményből: { $name }
app-collection-remove-failed = Nem sikerült eltávolítani a gyűjteményből: { $error }
app-ns-subject = Tárgy
app-ns-style = Stílus
app-ns-theme = Téma
app-ns-material = Anyag
app-ns-map = PBR-térkép
app-ns-color = Szín
app-ns-mood = Hangulat
app-ns-genre = Műfaj
app-ns-license = Licenc
app-ns-res = Felbontás
app-ns-duration = Hossz
app-ns-poly = Poligonszám
app-ns-format = Formátum
app-ns-weight = Súly
app-ns-tech = Technikai
app-ns-blender = Blender-verzió
app-ns-depth = Bitmélység
app-ns-ucs = Hangkategória (UCS)
app-ns-bpm = Tempó
app-ns-root = Alaphang
app-ns-level = Hangerő
app-ns-ue = Unreal-osztály
app-ns-engine = Motor
app-ns-extracted = Visszanyert forrás
app-ns-script = Írásrendszer
app-bulk-failed =
    { $count ->
        [one] Nem sikerült módosítani az assetet
       *[other] Nem sikerült módosítani { $count } assetet
    }: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] { $count } címke másolva
       *[other] { $count } címke másolva
    }
app-tags-copy-none = { $name } nem rendelkezik másolható címkével
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Nincs másolt címke. Nyomd meg a C-t egy asseten a címkéi másolásához.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [one] Már megvannak rajta ezek a címkék
       *[other] Mind a { $count } asseten megvannak már ezek a címkék
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [one]
            { $count ->
                [one] { $count } címke hozzáadva
               *[other] { $count } címke hozzáadva
            }
       *[other]
            { $count ->
                [one] { $count } címke hozzáadva { $assets } assethez
               *[other] { $count } címke hozzáadva { $assets } assethez
            }
    }
app-bulk-untagged = Címke eltávolítva { $count } assetről: { $tag }
app-bulk-tagged = { $count } asset felcímkézve ezzel: { $tag }
app-bulk-review = Megjelölés átnézettként
app-bulk-review-title = A kijelölt assetek megjelölése átnézettként: a szabályok és az MI ezután nem nyúlnak a címkéikhez
app-bulk-unreview = Jelölés levétele
app-bulk-unreview-title = Az átnézett jelölés levétele a kijelölt assetekről
app-bulk-reviewed =
    { $count ->
        [one] { $count } asset megjelölve átnézettként
       *[other] { $count } asset megjelölve átnézettként
    }
app-bulk-unreviewed =
    { $count ->
        [one] Átnézett jelölés levéve { $count } assetről
       *[other] Átnézett jelölés levéve { $count } assetről
    }
app-bulk-accept = Tippelt címkék megtartása
app-bulk-accept-title = A szabályok és az MI által a kijelölt assetekre tett összes címke megtartása saját címkeként
app-bulk-accepted =
    { $count ->
        [one] Tippelt címkék megtartva { $count } asseten
       *[other] Tippelt címkék megtartva { $count } asseten
    }
app-credits-failed = Nem sikerült elkészíteni a szerzők listáját: { $error }
app-archived-assets = { $count } asset archiválva
app-archived-one = Archiválva: { $name }
app-open-failed = Nem sikerült megnyitni az assetet: { $error }
app-unnamed-pack = csomag
app-unnamed-collection = gyűjtemény
app-unnamed-asset = asset
app-home-version = Hoard { $version } – vissza a könyvtárhoz
app-home = Vissza a könyvtárhoz
app-search-label = Assetek keresése
app-search-placeholder = Keresés név, elérési út, címke szerint
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = szikla or kő: az egyik a kettő közül · -moha: kihagyás · "száraz moha": ezek a szavak együtt · pack:erdő: egy csomag nevében
app-alt-leave-out = Alt-kattintással kihagyja
app-views = Nézetek
app-library = Könyvtár
app-packs = Csomagok
app-packs-working = { $count } folyamatban
app-update-title = Megjelent a Hoard { $version } (ez a szerver ezt futtatja: { $current }). Töltse le az új image-et vagy binárist, és indítsa újra; az adatbázis magától frissül.
app-update = Frissítés
app-theme = Téma
app-sign-in = Bejelentkezés
app-archive-title = Archívum: { $contents }
app-archive = Archívum
app-settings = Beállítások
app-palette-title = Parancspaletta ({ $keys })
app-import-folder = Mappa importálása
app-add-files = Fájlok hozzáadása
app-add-files-short = Hozzáadás
app-split-label = A mappa importálásának módja
app-split-before = { "" }
app-split-after = : { $count } mappa vagy tömörített fájl van benne.
app-split-each = Mindegyik külön csomagként ({ $count })
app-split-one = Egyetlen csomagként
app-cancel = Mégse
app-filters = Szűrők
app-done = Kész
app-yours = Saját
app-favorites = Kedvencek
app-needs-review = Átnézésre vár
app-needs-review-title = Csak azok az assetek, amelyeket még senki nem nézett át (a többi szűrővel együtt működik)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Értékelt
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] legalább { $count } csillag
       *[other] legalább { $count } csillag
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Kattintson újra az összes értékelés megjelenítéséhez
app-forget-search-label = Mentett keresés elfelejtése: { $name }
app-forget-search = Keresés elfelejtése
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = A könyvtár ezzel a kereséssel nyíljon meg
app-default-search-on-title = A könyvtár ezzel a kereséssel nyílik meg. Kattintson a kikapcsoláshoz.
app-default-search-label = A könyvtár nyíljon meg ezzel: { $name }
app-rename-search = Átnevezés
app-rename-search-label = Új név: { $name }
app-projects-title = Projektek, amelyek a hoard sync paranccsal innen szinkronizálnak, a legutóbbi szinkronizálás adataival
app-projects = Projektek
app-project-synced = Szinkronizálva: { $when }
app-project-synced-by = Szinkronizálta: { $who }, { $when }
app-collections = Gyűjtemények
app-no-filters = Ehhez a kereséshez nincsenek szűrők.
app-kind = Típus
app-facets-more = még { $count }
app-facets-fewer = kevesebb
app-searching = Keresés…
app-searching-live = Keresés
app-asset-count = { $count } asset
app-clear-all = Összes törlése
app-save-name-label = A keresés neve
app-save-name-placeholder = Adjon nevet
app-save = Mentés
app-save-search-title = A keresés megőrzése a Szűrők között, a Saját alatt
app-save-search = Keresés mentése
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Frissítés: „{ $name }”
app-update-search-title = A jelenlegi kijelölés megtartása ebben a mentett keresésben
app-save-search-new = Mentés újként
app-select-title = Több asset kijelölése együttes címkézéshez, kategorizáláshoz, gyűjteménybe tételhez, letöltéshez vagy archiváláshoz (X vagy Ctrl+kattintás)
app-select = Kijelölés
app-tile-size = Csempeméret
app-supporting-title = glTF-pufferek, anyagkönyvtárak, az anyagok térképei, csak egy modell által használt anyagok és a modelleket leíró áruházi fájlok. Velük együtt töltődnek le, amikor letölti az őket használó elemet. A sprite-lap oldalai és azon spritejai, amelyek a csomagban külön képként is megvannak, a lappal együtt jelennek meg. Egy elem, amely a csomagban több formátumban is megvan, egyszer jelenik meg, abban a formátumban, amelyet a Hoard a legjobban olvas.
app-supporting-hidden = { $count } kiegészítő fájl elrejtve
app-supporting-show = Megjelenítés
app-supporting-shown = Kiegészítő fájlokkal együtt
app-supporting-hide = Elrejtés
app-folders = Mappák
app-folders-title = A csomag böngészése mappánként, ahogy a lemezen volt
app-folders-label = A csomag mappái
app-welcome-title = Húzzon egy assetcsomagot bárhová erre az oldalra
app-welcome-before = Zip, tarball, mappa vagy különálló fájlok – mind működik. A Hoard kicsomagolja őket, minden fájlt egyszer tárol a hash-e alapján, beolvassa a méreteket, hosszakat és poligonszámokat, és mindent felcímkéz, így kereshet például ilyenekre:{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }vagy{ " " }
app-welcome-after = .
app-welcome-import-folder = Mappa importálása
app-welcome-choose-files = Fájlok kiválasztása
app-none-similar = Még semmi sem hasonlít igazán erre: { $name }.
app-none = Nincs találat.
app-none-query = Nincs találat erre: „{ $query }”.
app-none-imitation = Még egyik mért hang sem szól nagyon úgy, mint ez: „{ $word }”.
app-none-heard = Még egyik mért hang sem szól nagyon úgy, mint a te hangod.
app-none-filters = Ezekkel a szűrőkkel nincs találat.
app-none-query-filters = Ezekkel a szűrőkkel nincs találat erre: „{ $query }”.
app-clear-search-filters = Keresés és szűrők törlése
app-loading-more = További találatok betöltése
app-keys-hint = A nyílbillentyűk mozgatnak, az Enter megnyit, a Szóköz lejátssza a hangot, az X vagy a Ctrl+kattintás többet jelöl ki (érintőképernyőn hosszú nyomás).
app-resize-label = A részletek panel átméretezése
app-resize-title = Húzza az átméretezéshez · dupla kattintás az illeszkedő szélességhez
app-selected-label = Kijelölt assetek
app-selected = { $count } kijelölve
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Szerkesztés…
app-edit-selected-title = Az összes címkéjének és mezőjének módosítása (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] { $count } találat kijelölve
       *[other] Mind a(z) { $count } találat kijelölve
    }
app-pick-assets = Válasszon asseteket
app-download-zip = Letöltés zipként
app-credits-title = CREDITS.md azokhoz a csomagokhoz, amelyekből ezek származnak: kik készítették őket, milyen licenccel, és milyen forrásmegjelölést kérnek
app-credits = Szerzők
app-prefer-sources-title = Unreal SoundWave-ek és textúrák, amelyeknek az eredeti fájlját a Hoard visszanyerte
app-prefer-sources = Visszanyert WAV/PNG .uasset helyett ({ $count })
bag-clear = Kijelölés törlése
bag-glb-heading = Modellek GLB-ként
bag-glb-text = Minden modell .glb fájlként, Y tengellyel felfelé állva és előre nézve, a textúrái külön fájlként egy textures mappában, amelyen a modellek osztoznak. Minden más úgy kerül bele, ahogy van.
bag-glb-download = GLB zip letöltése
bag-plain-heading = Fájlok úgy, ahogy vannak
bag-plain-text = Minden fájl úgy, ahogy importálva lett, azzal, amire szüksége van, csomagonként egy mappában.
bag-remove = { $name } kivétele a kijelölésből
bag-more = és még { $count }
app-bulk-tag-label = Címke, amelyet hozzáad a kijelölt assetekhez, vagy eltávolít róluk
app-bulk-tag-placeholder = Címke
app-bulk-tag-add-title = A címke hozzáadása minden kijelölt assethez
app-bulk-tag-add = Hozzáadás
app-bulk-tag-remove-title = A címke eltávolítása minden kijelölt assetről
app-bulk-tag-remove = Eltávolítás
app-remove-from-collection = Eltávolítás innen: { $name }
app-unfavorite = Eltávolítás a kedvencekből
app-favorite = Kedvencekhez
app-select-all = Mind kijelölése ({ $count })
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] A(z) { $count } találat kijelölése
       *[other] Mind a(z) { $count } találat kijelölése
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] { $count } asset szerkesztése
       *[other] { $count } asset szerkesztése
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] { $count } találat szerkesztése
       *[other] Mind a(z) { $count } találat szerkesztése
    }
bulk-edit-close = Bezárás
bulk-edit-tags = Címkék
bulk-edit-tag-add = Címke hozzáadása mindegyikhez
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } / { $count }
bulk-edit-untick = Minden jelölés törlése
bulk-edit-untick-title = Az összes címke eltávolítása, majd a megtartandók bejelölése
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = A legtöbbet használt címkék közöttük
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Vegyes, változatlan marad
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Változatlan marad
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Mindegyiknél törölve
bulk-edit-reset = Visszaállítás
bulk-edit-all-note = Ez minden találatot módosít, a még be nem töltötteket is, és egy lépésben nem vonható vissza.
bulk-edit-save =
    { $count ->
        [one] { $count } módosítás mentése
       *[other] { $count } módosítás mentése
    }
bulk-edit-nothing = Nincs változás
bulk-edit-saved =
    { $count ->
        [one] { $count } asset frissítve
       *[other] { $count } asset frissítve
    }
app-archive-selected-title = Kikerülnek a keresésből, amíg vissza nem állítja vagy végleg nem törli őket az Archívumból
app-drop-items = Engedje el, és { $count } elem hozzáadódik
app-drop = Engedje el, és bekerül a Hoardba
app-drop-hint = Minden legfelső szintű fájl vagy mappa külön csomag lesz. Több gigabájt sem gond; a nagy fájlok feltöltése folytatódik, ha megszakad a kapcsolat.

## Parancspaletta

palette-group-go-to = Ugrás
palette-group-actions = Műveletek
palette-group-theme = Téma
palette-group-kinds = Típusok
palette-group-packs = Csomagok
palette-group-collections = Gyűjtemények
palette-group-projects = Projektek
palette-group-saved = Mentett keresések
palette-group-assets = Assetek
palette-hint-current = jelenlegi
palette-library = Könyvtár
palette-library-keywords = assetek rács kezdőlap assets grid home
palette-favorites = Kedvencek
palette-favorites-keywords = csillag csillagozott kedvelt starred stars liked
palette-packs = Csomagok
palette-archive = Archívum
palette-archive-keywords = kuka lomtár törölt visszaállítás trash recycle bin deleted restore
palette-stats = Könyvtárstatisztika
palette-stats-keywords = statisztika számok méret lemezterület darabszám statistics numbers size disk space count
palette-copies = Majdnem azonos másolatok
palette-copies-keywords = duplikátumok azonos hasonló átméretezett konvertált pazarolt hely duplicates same similar resized converted wasted space dedupe
palette-search = Keresés
palette-add-files = Fájlok hozzáadása…
palette-add-files-keywords = feltöltés importálás upload import
palette-import-folder = Mappa importálása…
palette-import-folder-keywords = feltöltés csomag upload pack
palette-select-all = Az összes látható kijelölése ({ $count })
palette-select-all-keywords = kijelölés bejelölés check
palette-select-several = Több kijelölése…
palette-select-several-keywords = tömeges több címke archiválás bulk multiple tag archive
palette-archive-selected = A kijelöltek archiválása ({ $count })
palette-archive-selected-keywords = törlés eltávolítás kuka delete remove trash
palette-edit-selected = A kijelölt { $count } szerkesztése
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = tömeges módosítás címkék kategória licenc értékelés
palette-paste-tags =
    { $count ->
        [one] A másolt címkék beillesztése a kijelölt assetre
       *[other] A másolt címkék beillesztése a { $count } kijelölt assetre
    }
palette-clear-selection = Kijelölés megszüntetése
palette-clear-filters = Szűrők törlése
palette-clear-filters-keywords = visszaállítás alaphelyzet reset
palette-hide-supporting = Kiegészítő fájlok elrejtése
palette-show-supporting = Kiegészítő fájlok megjelenítése
palette-supporting-keywords = normáltérkép érdesség maszk normal maps roughness masks
palette-account = Fiók és hozzáférési tokenek
palette-account-keywords = jelszó bejelentkezés felhasználó token api password login user
palette-sign-out = Kijelentkezés ({ $name })
palette-sign-out-keywords = kijelentkezés log out logout
palette-sign-in = Bejelentkezés
palette-sign-in-keywords = bejelentkezés fiók jelszó log in login account password
palette-settings = Beállítások
palette-settings-keywords = blender ffmpeg llm mi ai címkézés tagging unreal admin eszközök tools preferences
palette-shortcuts = Billentyűparancsok
palette-shortcuts-keywords = súgó billentyűk help keys
palette-theme-keywords = téma megjelenés mód theme appearance mode
palette-theme-variant-keywords = megjelenés szín appearance color
palette-light-theme = Világos téma: { $name }
palette-dark-theme = Sötét téma: { $name }
palette-kind-all = Minden típus (ne csak ezek: { $kind })
palette-kind-only = Csak { $kind }
palette-title = Parancspaletta
palette-placeholder = Assetek keresése, ugrás csomagra, parancs futtatása…
palette-results = Találatok
palette-searching = Keresés…
palette-no-match = Nincs találat erre: „{ $query }”
palette-foot-move = mozgás
palette-foot-run = futtatás
palette-foot-shortcuts = összes billentyűparancs

## Billentyűparancsok

shortcuts-title = Billentyűparancsok
shortcuts-close = Bezárás
shortcuts-anywhere = Bárhol
shortcuts-palette = Parancspaletta: keresés, ugrás, futtatás
shortcuts-search = Keresés
shortcuts-this-list = Ez a lista
shortcuts-escape = Bezárás, a kijelölés megszüntetése vagy a részletek bezárása
shortcuts-library = Könyvtár
shortcuts-move = Mozgás a csempék között (vagy J és K)
shortcuts-open = Részletek megnyitása
shortcuts-play-sound = Hang lejátszása vagy leállítása
shortcuts-select = Kijelölés vagy kijelölés megszüntetése
shortcuts-favorite = Hozzáadás a kedvencekhez vagy eltávolítás onnan
shortcuts-reviewed = Megjelölés átnézettként, vagy a jelölés levétele
shortcuts-tag = Címke hozzáadása
shortcuts-edit = A kijelöltek szerkesztése
shortcuts-copy-tags = A címkéinek másolása
shortcuts-paste-tags = A másolt címkék beillesztése a kijelöltekre
shortcuts-key-click = kattintás
shortcuts-select-range = Tartomány kijelölése
shortcuts-select-all = Minden látható kijelölése
shortcuts-sound-player = Hanglejátszó
shortcuts-play-pause = Lejátszás vagy szünet
shortcuts-skip = Ugrás hátra vagy előre
shortcuts-start-end = Eleje vagy vége
shortcuts-image-inspector = Képvizsgáló
shortcuts-zoom = Nagyítás vagy kicsinyítés
shortcuts-fit = Igazítás az ablakhoz
shortcuts-actual-pixels = Tényleges képpontok
shortcuts-tile = 3×3-as csempézés a varratok ellenőrzéséhez
shortcuts-background = Következő háttér
shortcuts-channel = Egy csatorna megjelenítése

## Assettípusok

kinds-group = Assettípusok

## Címkék

tag-title-with-source = { $tag } ({ $source })
tag-source-human = egy ember adta hozzá
tag-source-rule = az offline címkézőtől
tag-source-meta = a fájlból mérve
tag-source-ai = MI által javasolt
tag-remove = Címke eltávolítása: { $tag }
tag-accept = A(z) { $tag } címke megtartása
tag-accept-title = A tipp megtartása saját címkeként
tag-reject-title = Eltávolítás; a címkéző nem teszi vissza
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Új címke
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Erre gondolt
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = korábban { $alias }

## Tevékenységjelző

activity-label-idle = Háttértevékenység: tétlen
activity-label-running = Háttértevékenység: { $running } fut
activity-label-queued = Háttértevékenység: { $running } fut, { $queued } várakozik
activity-title = Háttértevékenység
activity-unreachable = A szerver nem érhető el
activity-unreachable-desktop = A Hoard nem válaszol
activity-empty = Semmi sem fut. Az importálások, a címkézés, az átalakítások és a zip-letöltések itt jelennek meg, amíg dolgoznak.
activity-queued = várakozik
activity-stop = „{ $what }” leállítása
activity-stop-tagging = Az összes MI-címkézés leállítása ({ $count })
activity-seconds = { $s } mp
activity-minutes = { $min } perc
activity-hours = { $h } ó { $min } perc
activity-percent = { $pct }%

## Assetcsempe

tile-triangles = { $count } háromszög
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Lejátszás: { $name }
tile-pause = Szünet: { $name }
tile-favorite = Kedvenc: { $name }
tile-in-favorites = A kedvencei között
tile-add-favorite = Hozzáadás a kedvencekhez (F)
tile-more-like = Több ehhez hasonló: { $name }
tile-more-like-this = Több ilyen
tile-select = Kijelölés: { $name }

## Gyűjteményválasztó

collections-add = Hozzáadás gyűjteményhez
collections-menu = Gyűjtemények
collections-find-label = Gyűjtemény keresése vagy létrehozása
collections-find-placeholder = Keressen, vagy adja meg egy új gyűjtemény nevét
collections-create = Létrehozás
collections-empty = Még nincsenek gyűjtemények. Írjon be egy nevet az elsőhöz.

## Mappaválasztó

folders-list = Mappák
folders-close = Bezárás
folders-browse = Tallózás…
folders-up-title = Egy mappával feljebb
folders-up = Fel
folders-this-computer = Ez a számítógép
folders-none = Itt nincsenek mappák.

## Tárolási hely űrlap

locations-mode-question = Mit kezdjen a Hoard ezzel a mappával?
locations-mode-managed = Lefoglalja: a Hoard itt tárolja a csomagjaimat
locations-mode-managed-hint = A feltöltött vagy importált csomagok ide másolódnak, és a Hoard rendben tartja őket. A mappa mostantól a Hoardé, ezért ne nyúljon a tartalmához.
locations-mode-watched = Figyeli: a csomagjaim már itt vannak
locations-mode-watched-hint = A benne lévő minden mappa külön csomag lesz. A Hoard a helyükön olvassa a fájlokat, és soha nem módosítja, helyezi át vagy törli őket, az új mappákat pedig megjelenésükkor felveszi.
locations-folder-desktop = Mappa
locations-folder-placeholder = A mappa, amelyben a csomagjai vannak
locations-folder-hint-managed = Ha nem létezik, a Hoard létrehozza. Dockerben ez egy olyan mappa, amelyet a konténerbe csatolt.
locations-folder-hint-managed-desktop = Ha nem létezik, a Hoard létrehozza.
locations-folder-hint-watched = Dockerben előbb csatolja a mappát a konténerbe (csak olvashatóként is jó), majd válassza ki itt.
locations-folder-hint-watched-desktop = A Hoard csak olvassa: semmi sem változik benne.
locations-name = Név
locations-name-placeholder-managed = Könyvtár
locations-name-placeholder-watched = Saját csomagok
locations-files = Ide kerülő fájlok
locations-catch-all = Minden egyéb is, amit más hely nem fogad
locations-kinds-hint = Egy új csomag fájljai arra a helyre kerülnek, amelyiken be van jelölve a típusuk. A többi a gyűjtőhelyre kerül.
locations-files-all = Egyelőre mind. Később a Beállításokban további helyeket adhat hozzá, hogy bizonyos típusokat (például a zenét) másik lemezre küldjön.
locations-first-watched-note = A feltöltéshez olyan könyvtár kell, amelyet a Hoard kezel. Később hozzáadhat egyet a Beállítások → Tárolási helyek oldalon.
locations-checking = A mappa ellenőrzése…
locations-add = Hely hozzáadása
locations-cancel = Mégse

## Tárolási helyek panel

locations-failed = { $what }: { $error }
locations-load-failed = Nem sikerült betölteni a tárolási helyeket
locations-takes-and-rest = { $kinds } és minden egyéb
locations-takes-rest = Minden, amit a többi nem fogad
locations-takes-everything = Minden
locations-takes-nothing = Egyelőre semmi
locations-save-failed = Nem sikerült menteni a helyet
locations-unwatch-title = Leállítja ennek a mappának a figyelését: { $name }?
locations-unwatch-body = A hozzá tartozó { $count } csomag a címkéivel és jegyzeteivel együtt kikerül a Hoardból. A mappában lévő fájlokhoz nem nyúl.
locations-unwatch-body-empty = A mappában lévő fájlokhoz nem nyúl.
locations-unwatch = Figyelés leállítása
locations-remove-title = Eltávolítja ezt a helyet: { $name }?
locations-remove-body = A benne lévő fájlok előbb típus szerint átkerülnek a többi helyre; a hely akkor tűnik el, amikor kiürült. Semmi sem vész el.
locations-remove-body-only = Ha fájlokat tartalmaz, előbb adjon hozzá egy másik helyet, ahová átkerülhetnek.
locations-remove = Eltávolítás
locations-removed = Eltávolítva: { $name }
locations-emptying = Fájlok áthelyezése innen: { $name }; ha elkészült, a hely törlődik
locations-offline-title = { $name } nem érhető el
locations-offline-body = A fájljai nem helyezhetők át, amíg offline. Mégis eltávolítja? A rajta maradt fájlok hiányzóként jelennek meg.
locations-remove-anyway = Eltávolítás mégis
locations-remove-failed = Nem sikerült eltávolítani: { $name }
locations-rebalancing = A tárolt fájlok áthelyezése a típusukat fogadó helyekre
locations-rebalance-failed = Nem sikerült áthelyezni a fájlokat
locations-scanned-never = még nem volt vizsgálva
locations-scanned-now = épp most vizsgálva
locations-scanned-minutes = vizsgálva { $mins } perce
locations-scanned-hours = vizsgálva { $hours } órája
locations-scanned-date = vizsgálva: { $date }
locations-scan-failed = Nem sikerült megvizsgálni: { $name }
locations-watching = Figyelés: { $name }
locations-added = Hozzáadva: { $name }
locations-loading = Betöltés…
locations-managed-title = A Hoard által kezelt könyvtárak
locations-watched-title = Figyelt mappák
locations-managed-hint = A feltöltések és importálások ide másolódnak. Minden hely a bejelölt típusokat fogadja; a gyűjtőhely a többit.
locations-watched-hint = Csomagokat tartalmazó mappák, a helyükön indexelve. A Hoard soha nem módosítja a bennük lévő fájlokat, és 15 percenként keres új és eltávolított mappákat.
locations-managed-empty = Még nincs ilyen, így a feltöltéseknek nincs hová kerülniük. Adjon hozzá egyet lent.
locations-reachable = Elérhető
locations-unreachable-now = Jelenleg nem érhető el
locations-catch-all-tag = gyűjtőhely
locations-packs = { $count } csomag
locations-unreachable = nem érhető el
locations-scanning = Vizsgálat…
locations-scan = Vizsgálat most
locations-edit = Szerkesztés
locations-catch-all-locked = Ennek módosításához tegyen meg egy másik helyet gyűjtőhelynek
locations-edit-hint-before = A mostantól hozzáadott fájlokra vonatkozik. A már tárolt fájlok áthelyezéséhez használja a lenti
locations-edit-hint-after = gombot.
locations-save = Mentés
locations-rebalance = Tárolt fájlok áthelyezése
locations-rebalance-hint = Miután módosította, mit fogad egy hely: a már tárolt fájlok átkerülnek arra a helyre, ahová a típusuk most tartozik.
locations-add-title = Hely hozzáadása
locations-add-hint = Egy másik lemez bizonyos fájltípusokhoz, vagy egy már meglévő, csomagokat tartalmazó mappa.
locations-kind-question = Milyen hely ez?
locations-kind-folder = Egy mappa
locations-kind-folder-hint = Egy lemezen vagy a konténerbe csatolt NAS-megosztáson, vagy egy mappa, amelyet a Dropbox, a Google Drive, a OneDrive vagy az iCloud Drive szinkronban tart. A Hoard fájlokat tárolhat benne, vagy figyelheti a már meglévő csomagjait.
locations-kind-folder-hint-desktop = Ezen a számítógépen, egy másik lemezen vagy NAS-megosztáson, vagy egy mappa, amelyet a Dropbox, a Google Drive, a OneDrive vagy az iCloud Drive szinkronban tart. A Hoard fájlokat tárolhat benne, vagy figyelheti a már meglévő csomagjait.
locations-kind-s3-hint = Egy bucket az Amazon S3-ban vagy egy hasonló szolgáltatásban: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. A Hoard fájlokat tárol benne.
locations-s3-title = S3-tárhely
locations-s3-hint = Az s3://bucket-name helyek ennek a szolgáltatásnak a bucketjei: az AWS-é vagy bármely S3-kompatibilis szolgáltatásé (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage egy NAS-on).
locations-bucket = Bucket
locations-bucket-hint = Egy már létező bucket neve. A fenti kulcsoknak listázniuk, olvasniuk és írniuk kell tudniuk.
locations-s3-endpoint = Cím
locations-s3-endpoint-placeholder = AWS esetén üres; pl. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Régió
locations-s3-access-key = Hozzáférési kulcs azonosítója
locations-s3-secret-key = Titkos hozzáférési kulcs
locations-s3-headers = További fejlécek
locations-s3-headers-placeholder = Name=value, soronként egy (például a Cloudflare Accesshez)
locations-s3-kept = Mentve (gépeljen a cseréhez)
locations-s3-secrets = A kulcsok és a fejlécek a szerver beállításai között tárolódnak, és többé nem jelennek meg.
locations-s3-secrets-desktop = A kulcsok és a fejlécek ennek a számítógépnek a jelszótárolójában tárolódnak, és többé nem jelennek meg.
locations-s3-saved = S3-tárhely mentve
locations-s3-save-failed = Nem sikerült menteni az S3-tárhelyet
locations-add-more = Hely hozzáadása…

## Problémák panel

issues-failed = { $what }: { $error }
issues-load-failed = Nem sikerült betölteni a Problémákat
issues-started =
    { $count ->
        [one] Újraindítva.
       *[other] { $count } elem újraindítva.
    }
issues-retry-failed = Nem sikerült újrapróbálni
issues-dismiss-failed = Nem sikerült elvetni
issues-dismiss-all-title = Elveti mindet ({ $count })?
issues-dismiss-all-body = A(z) „{ $title }” csoport kiürül. Ami újra hibát okoz, az ismét megjelenik.
issues-dismiss-all = Mind elvetése
issues-just-now = épp most
issues-minutes-ago = { $mins } perce
issues-hours-ago = { $hours } órája
issues-loading = Betöltés…
issues-none = Semmi sem romlott el.
issues-action-all = { $action } (mind)
issues-open-asset = Megnyitás
issues-open-pack = Csomag megjelenítése
issues-open-locations = Tárolási helyek
issues-where-before = hely:
issues-where-after = { "" }
issues-times = { $count } alkalommal
issues-starting = Indítás…
issues-dismiss-named = Elvetés: { $title }
issues-dismiss = Elvetés
issues-show-fewer = Kevesebb
issues-show-all = Összes megjelenítése ({ $count })
issues-not-listed = További { $count } nincs felsorolva.

## Archiválás

archive-anyway = Archiválás mégis
archive-failed = Nem sikerült archiválni: { $error }
archive-undo = Visszavonás
archive-restore-failed = Nem sikerült visszaállítani: { $error }
archive-count-packs-and-assets = { $packs } csomag és { $assets } asset
archive-count-packs = { $packs } csomag
archive-count-assets = { $assets } asset
archive-count-nothing = semmi

## Archívum nézet

archive-load-failed = Nem sikerült betölteni az archívumot: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Visszaállítva: { $what }
archive-purged = Véglegesen törölve: { $what }
archive-purge-failed = Nem sikerült törölni: { $error }
archive-purge-title = Véglegesen törli: { $name }?
archive-purge-pack-body = Vele együtt törlődik { $count } assete és { $size } fájlja is, kivéve azokat a fájlokat, amelyeken egy másik csomag is osztozik. Ez nem vonható vissza.
archive-purge = Végleges törlés
archive-purge-asset-body = A fájlja felszabadul, hacsak egy másik asset nem osztozik rajta. Ez nem vonható vissza.
archive-empty-title = Kiüríti az archívumot?
archive-empty-body = Véglegesen törlődik: { $what }, a fájljaik pedig felszabadulnak. Ez nem vonható vissza.
archive-empty = Archívum kiürítése
archive-title = Archívum
archive-intro = Az archivált csomagok és assetek kikerülnek a keresésből, a számlálókból és a címkelistából. Állítsa vissza őket, vagy törölje véglegesen a helyük felszabadításához.
archive-loading = Betöltés…
archive-is-empty = Az archívum üres
archive-is-empty-hint = Archiváljon egy csomagot vagy assetet, és itt várakozik, amíg vissza nem állítja vagy véglegesen nem törli.
archive-packs = Csomagok
archive-assets = Assetek
archive-asset-count = { $count } asset
archive-archived = archiválva: { $when }
archive-from = csomag: { $pack }
archive-restore = Visszaállítás
archive-show-more = Továbbiak ({ $count } maradt)

## Saját elemek

projects-favorite-add-failed = Nem sikerült hozzáadni a kedvencekhez: { $error }
projects-favorite-remove-failed = Nem sikerült eltávolítani a kedvencek közül: { $error }
projects-save-search-failed = Nem sikerült menteni a keresést: { $error }
projects-forget-search-failed = Nem sikerült eltávolítani: { $name }: { $error }
projects-change-search-failed = Nem sikerült módosítani: { $name }: { $error }

## Projektek

projects-list-and = { $items } és { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = további { $count }
projects-other = { $count } másik projekt
projects-usage-title-one = { $name } használja ezt
projects-usage-title = { $count } projekt használja ezt
projects-usage-body-one = Használja: { $list }. A következő hoard sync eltávolítja őket onnan.
projects-usage-body = Használják: { $list }. Mindegyiknél a következő hoard sync eltávolítja őket.

## Feltöltő

upload-network-error = hálózati hiba
upload-gave-up = { $what }: { $error }, { $attempts } próbálkozás után
upload-failed-plain = sikertelen
upload-already-imported = már importálva
upload-part-failed = a feltöltés sikertelen ({ $status })
upload-batch = { $count } fájlból álló köteg
upload-batch-failed = a köteg sikertelen ({ $status })
upload-finish = feltöltés befejezése

## Feltöltési sor

upload-eta-seconds = még { $s } mp
upload-eta-minutes = még { $min } perc
upload-eta-hours = még { $h } ó
upload-queue = Importálások
upload-importing = { $count } csomag importálása
upload-bytes-of = { $sent } / { $total }
upload-rate = { $size }/s
upload-waiting = Várakozás
upload-unpacking = Feltöltve, kicsomagolás folyamatban
upload-already = Már a könyvtárban van
upload-stopped = Leállítva
upload-files-of = { $done } / { $total } fájl
upload-resumed = folytatva, { $size } már megvolt
upload-skipped = Pontosan ezeket a fájlokat már importálták „{ $name }” néven, ezért kimaradtak.
upload-error = { $error }. A haladás megmarad: próbálja újra, vagy húzza ide később ugyanazokat a fájlokat.
upload-retry = Újra
# $name is the pack the library has already that these files go into.
upload-update = „{ $name }” frissítése
upload-copy = Importálás másolatként
# $name is the pack the library has already that the upload goes into.
upload-into = A már a könyvtárban lévő „{ $name }” csomagba kerül: az új fájlok hozzáadódnak, a módosultak lecserélődnek.
upload-own-pack = Legyen külön csomag
upload-dismiss = Elvetés
upload-cancel = Mégse
upload-minimize = Kis méret
upload-expand = Importálások megjelenítése
upload-pill = { $count } importálása · { $percent }%
upload-pill-stopped = Leállt: { $count }
upload-interrupted = Megszakadt
upload-resume = Folytatás
upload-resume-all = Mind folytatása
upload-resume-hint-folder = Válaszd ki újra ugyanazt a mappát, vagy húzd az oldalra, és onnan folytatódik, ahol abbamaradt.
upload-resume-hint-file = Válaszd ki újra ugyanazt a fájlt, vagy húzd az oldalra, és onnan folytatódik, ahol abbamaradt.
upload-not-same = Ezek nem ugyanazok a fájlok, így semmi sem folytatódott.
upload-permission-denied = A böngésző nem kapott engedélyt a fájlok újbóli olvasására. Válaszd ki őket inkább.

## Csomagok nézet

packs-rename-failed = Nem sikerült átnevezni: { $error }
packs-collection-deleted = Gyűjtemény törölve: { $name }
packs-delete-failed = Nem sikerült törölni: { $error }
packs-status-uploading = Feltöltés (az eddig megérkezett fájlok)
packs-status-ingesting = Kicsomagolás és címkézés
packs-status-analyzing = Az LLM címkéz
packs-status-ready = Kész
packs-status-failed = Sikertelen
packs-retag-failed = Nem sikerült elindítani az újracímkézést: { $error }
packs-read-files-failed = Nem sikerült beolvasni a csomag fájljait: { $error }
packs-context-load-failed = Nem sikerült betölteni a csomag áruházi adatait: { $error }
packs-undo-store-failed = Nem sikerült leválasztani az áruházi oldalt: { $error }
packs-context-save-failed = Nem sikerült menteni az áruházi adatokat: { $error }
packs-unreal-load-failed = Nem sikerült betölteni az Unreal-átalakítás állapotát: { $error }
packs-unreal-discard-title = Elveti az eredetieket?
packs-unreal-discard-body = Az eredeti .uasset fájlok ebben a csomagban ({ $name }) az átalakítás után törlődnek. A bennük lévő, csak az Unreal által használt adatok, például az anyaggráfok, végleg elvesznek.
packs-unreal-discard-confirm = Átalakítás és elvetés
packs-unreal-convert-failed = Nem sikerült elindítani az átalakítást: { $error }
packs-unreal-fact-converted = { $done } átalakítva Unrealből
packs-unreal-fact-packages = { $left } Unreal-csomagfájl
packs-unreal-fact-both = { $done } átalakítva Unrealből, { $left } még csomagfájl
packs-thumbs-canceled = Megszakítva ennyi után: { $done } / { $total }
packs-thumbs-drew = { $count } modell megrajzolva
packs-thumbs-result = { $lead }: { $improved } bélyegkép lecserélve.
packs-thumbs-result-failed = { $lead }: { $improved } bélyegkép lecserélve, { $failed } nem rajzolható meg.
packs-archived = Archiválva: { $name } és { $count } assete
packs-blobs-bad = { $count } fájl nem ment át a legutóbbi sértetlenség-ellenőrzésen.
packs-collections = Gyűjtemények
packs-browse-named = Böngészés: { $name }
packs-collection-name = Gyűjtemény neve
packs-save = Mentés
packs-cancel = Mégse
packs-assets = { $count } asset
packs-updated = Frissítve: { $date }
packs-browse = Böngészés
packs-collection-download = Letöltés (.zip)
packs-collection-credits = Szerzők
packs-collection-credits-title = Kik készítették azokat a csomagokat, amelyekből ezek az assetek származnak, milyen licenccel, és milyen forrásmegjelölést kérnek
packs-rename = Átnevezés
packs-delete = Törlés
packs-packs = Csomagok
packs-empty = Még nincsenek csomagok. Húzzon egy zipet vagy egy mappát bárhová erre az oldalra a hozzáadáshoz.
packs-find = Csomag keresése
packs-find-placeholder = Csomag keresése név, áruház, licenc vagy címke szerint
packs-sort = Sorrend
packs-sort-new = Legújabb elöl
packs-sort-old = Legrégebbi elöl
packs-sort-size = Legnagyobb elöl
packs-sort-assets = Legtöbb asset elöl
packs-sort-count = Assetek száma
packs-sort-smallest = Legkisebb elöl
packs-sort-fewest = Legkevesebb asset elöl
packs-with-kind = Csomagok, amelyekben van
packs-state = Állapot
packs-state-any = Bármilyen állapot
packs-state-working = Folyamatban
packs-state-failed = Sikertelen
packs-state-untagged = Még nincs leírva
packs-state-unlicensed = Nincs megadott licenc
packs-store = Áruház
packs-store-any = Bármelyik áruház
packs-store-from = Forrás: { $store }
packs-store-none = Nincs áruházi oldal
packs-shown-of = { $shown } / { $total }
packs-clear = Törlés
packs-no-match = Nincs egyező csomag.
packs-clear-all = Keresés és szűrők törlése
packs-pack-name = Csomag neve
packs-thumbs-drawing = Bélyegképek rajzolása ebben a böngészőben…
packs-thumbs-drawing-count = Bélyegképek rajzolása ebben a böngészőben: { $done } / { $total }…
packs-thumbs-stopped = Leállt: { $error }
packs-thumbs-none = Itt már minden modellnek van az összes textúrájával rajzolt bélyegképe.
packs-dismiss = Elvetés
packs-license = Licenc: { $license }
packs-license-tier = Licenc: { $license } ({ $tier })
packs-license-not-found = nem található
packs-from-before = Forrás:
packs-from-after = { "" }
packs-added = Hozzáadva: { $date }
pack-panel-label = A csomagról
pack-panel-hide = A csomag részleteinek elrejtése
pack-panel-kinds = Mi van benne
pack-panel-all = Összes csomag
packs-download = Csomag letöltése (.zip)
packs-store-details = Áruházi adatok
packs-more-named = További műveletek: { $name }
packs-more = Továbbiak
packs-rename-more = Átnevezés…
packs-retag-rules = Offline címkéző újrafuttatása
packs-retag-llm = LLM-címkéző újrafuttatása
packs-make-thumbs = Bélyegképek készítése ebben a böngészőben
packs-make-thumbs-title = Minden modellt egyenként megnyit egy rejtett 3D-megjelenítőben, és a képét használja bélyegképként, ahol az jobb a szerverénél
packs-make-thumbs-title-desktop = Minden modellt egyenként megnyit egy rejtett 3D-megjelenítőben, és a képét használja bélyegképként, ahol az jobb a Hoard által készítettnél
packs-read-files = A csomag fájljainak újraolvasása
packs-credits = Szerzők letöltése
packs-credits-title = CREDITS.md: ki készítette a csomagot, milyen licenccel, és milyen forrásmegjelölést kér
packs-unreal-open = Unreal-tartalom átalakítása…
packs-archive = Csomag archiválása
packs-unreal-intro = A Hoard maga olvassa be az Unreal-csomagfájlokat, és a tartalmukat közönséges fájlokként tárolja: a textúrákat PNG-ként vagy EXR-ként, a hangokat WAV-ként, az anyagokat JSON-ként. Az átalakított fájl átveszi a csomagfájl helyét, megtartva a címkéit, hivatkozásait és gyűjteményeit.
packs-unreal-converted =
    { $count ->
       *[other] átalakítva
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] átalakítva, ebből { $kept } megőrzött eredetivel
    }
packs-unreal-pending =
    { $count ->
       *[other] még nincs megvizsgálva
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] Unreal Editorra vár
    }
packs-unreal-unsupported =
    { $count ->
       *[other] általános formátum nélkül
    }
packs-unreal-failed =
    { $count ->
       *[other] sikertelen
    }
packs-unreal-reasons = Miért maradtak csomagfájlok
packs-unreal-editor-before = A hálókhoz és animációkhoz Unreal Editor kell. Egy olyan gépen, ahol telepítve van az Unreal, futtassa ezt:
packs-unreal-editor-command = hoard unreal-worker --editor <az UnrealEditor-Cmd elérési útja>
packs-unreal-editor-after = ; FBX-ként exportálja őket, és ide tölti fel az eredményt.
packs-unreal-discard = Az eredeti .uasset fájlok elvetése az átalakítás után
packs-unreal-discard-hint = Felszabadítja a helyet. A csak az Unreal által használt adatok, például az anyaggráfok, elvesznek.
packs-unreal-drop = Általános formátum nélküli csomagfájlok törlése
packs-unreal-drop-hint = Blueprintek, pályák és más, csak a motorban használható assetek.
packs-unreal-convert = Átalakítás
packs-context-hint-before = Honnan származik a csomag, és mit mond róla az eladó. Az LLM-címkéző minden fájllal együtt elolvassa ezt, így egy olyan áruházi leírás, mint „120 ice and fire spells”, segít neki elnevezni például ezt:
packs-context-hint-after = .
packs-store-auto = Automatikusan megtalálva: maga a csomag nevezi meg ezt az oldalt.
packs-store-not-this = Nem ez az
packs-lookup-label = Keresés az áruházakban
packs-lookup-placeholder = A csomag neve az áruházban
packs-lookup-searching = Keresés…
packs-lookup-search = Keresés az áruházakban
packs-lookup-hide = Elrejtés
packs-lookup-clues = A csomag fájljaiból: { $clues }.
packs-lookup-also = Keresett még: { $queries }.
packs-lookup-quoted = „{ $query }”
packs-lookup-nothing = A Fabon, az itch.io-n, a Kenney-n és az OpenGameArton semmi sem hasonlít rá. Próbálja kevesebb vagy más szavakkal.
packs-lookup-use = Használat
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Most
packs-review-store = Az áruházból
packs-review-same = Azonos
packs-review-read-page = Mentéskor beolvassa az oldalról
packs-review-take =
    { $count ->
       *[other] { $count } mező kitöltése
    }
packs-lookup-searched = Keresve: { $date }.
packs-lookup-again = Keresés újra
packs-lookup-again-after = .
packs-lookup-unreachable = Ezúttal nem sikerült elérni: { $stores }.
packs-lookup-also-look = Nézze meg itt is:
packs-lookup-links-and = { " " }és{ " " }
packs-lookup-browser-only = (ezekben csak a böngészőben lehet keresni); az oldal címét illessze be lent.
packs-lookup-open = Keresés az áruházakban…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Eladó
packs-seller-placeholder = Ki készítette
packs-license-label = Licenc
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Licencszint
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
       *[other] { $count } asset saját licenccel rendelkezik: kövesse a csomag licencét is
    }
packs-store-page = Áruházi oldal
packs-store-text = Leírás az áruházból
packs-store-text-placeholder = Illessze be a hirdetés szövegét, vagy hagyja üresen, hogy az áruházi oldalról töltődjön le.
packs-store-text-fetch = Mentéskor a leírás letöltődik az áruházi oldalról.
packs-readme = A csomagban talált README
packs-save-retag = Mentés és az LLM-címkéző újrafuttatása

## Asset részletei

asset-lazy-3d-viewer = 3D-megjelenítő
asset-lazy-text = szöveg
asset-lazy-text-viewer = szövegmegjelenítő
asset-lazy-image-viewer = képmegjelenítő
asset-lazy-material-preview = anyagelőnézet
asset-lazy-material-viewer = anyagmegjelenítő
asset-lazy-sprite-player = sprite-lejátszó
asset-lazy-sheet-view = spritelap-megjelenítő
asset-rel-texture = textúra
asset-rel-buffer = geometriaadatok
asset-rel-material = anyagfájl
asset-rel-animation = animáció
asset-rel-library = kapcsolt fájl
asset-rel-guess = valószínűleg hozzá tartozik
asset-rel-sidecar = áruházi adatok
asset-rel-variant = változat
asset-rel-sprite = sprite
asset-rel-sample = minta
asset-role-map = térkép: { $role }
asset-maps-count = { $count } térkép
asset-needs = Igényli
asset-animations = Animációk
asset-sprites = Sprite-ok
asset-used-by = Használja
asset-loose-sprites = Külön fájlokként is
asset-in-sheet = Sprite-lapon
asset-other-formats = Más formátumok
asset-shown-as = A könyvtárban így látszik
asset-conversion-failed = az átalakítás sikertelen
asset-as-converting = Átalakítás…
asset-as-converting-percent = Átalakítás… { $percent }%
asset-as-queued = Sorban áll…
asset-download = Letöltés
asset-as-label = Letöltés másként
asset-as-original = { $format } (eredeti formátum)
asset-as-pack-files = A csomag saját fájljai
asset-as-size = Méret
asset-as-full-size = Teljes méret
asset-as-up-to = Legfeljebb { $size } px
asset-as-hint = Válasszon méretet vagy más formátumot
asset-meta-size = Méret
asset-meta-length = Hossz
asset-meta-sample-rate = Mintavételi frekvencia
asset-meta-channels = Csatornák
asset-meta-mono = Mono
asset-meta-stereo = Sztereó
asset-meta-bit-depth = Bitmélység
asset-meta-bits = { $bits } bites
asset-meta-level = Szint
asset-meta-level-value = csúcs { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = csúcs { $peak } dBFS, RMS { $rms } dBFS, túlvezérelt
asset-meta-loop = Hurok
asset-meta-loop-value = { $start } – { $end }
asset-meta-playback = Lejátszás
asset-meta-one-shot = Egyszeri
asset-meta-envelope = Burkológörbe
asset-meta-envelope-times = felfutás { $attack }, lecsengés { $tail }
asset-meta-envelope-hit = ütés
asset-meta-envelope-swell = erősödő
asset-meta-envelope-sustained = kitartott
asset-meta-tone = Hangszín
asset-meta-tone-value = fényesség { $khz } kHz, zaj { $noise }%
asset-meta-pitch = Hangmagasság
asset-meta-pitch-rises = { $octaves } oktávval emelkedik
asset-meta-pitch-falls = { $octaves } oktávval süllyed
asset-meta-loops-cleanly = Hézagmentesen ismételhető
asset-meta-tempo = Tempó
asset-meta-tempo-heard = kb. { $bpm } BPM, hallás alapján
asset-meta-tempo-heard-or = kb. { $bpm } BPM (vagy { $alt }), hallás alapján
asset-meta-key = Hangnem
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, hallás alapján
music-key-major = { $note }-dúr
music-key-minor = { $note }-moll
asset-meta-root-note = Alaphang
asset-meta-cue-markers = Jelölők
asset-meta-library = Könyvtár
asset-meta-keywords = Kulcsszavak
asset-meta-title = Cím
asset-meta-artist = Előadó
asset-meta-album = Album
asset-meta-genre = Műfaj
asset-meta-generator = Generátor
asset-meta-everything-with = Minden { $tag } címkéjű elem megjelenítése
asset-meta-saved-with = Mentve ezzel
asset-meta-color-mode = Színmód
asset-meta-layers = Rétegek
asset-meta-range = Tartomány
asset-meta-hdr = HDR (az előnézet tónusleképezett)
asset-meta-format = Formátum
asset-meta-large-document = Nagy dokumentum (PSB)
asset-meta-ai = MI
asset-meta-ai-made = Generatív MI-vel készült
asset-meta-ai-edited = Generatív MI-vel szerkesztve
asset-meta-ai-how-metadata = metaadatok
asset-meta-ai-how-generator = generátorbeállítások
asset-meta-ai-how-iptc = IPTC digitális forrástípus
asset-meta-ai-says = { $made } (forrás: { $how })
asset-meta-ai-by-says = { $made } ({ $by }; forrás: { $how })
asset-meta-provenance = Eredet
asset-meta-c2pa = C2PA tartalomhitelesítő adatokkal (Content Credentials)
asset-meta-compression = Tömörítés
asset-meta-note = Megjegyzés
asset-meta-maps = Térképek
asset-meta-sizes =
    { $count ->
        [one] Méret
       *[other] Méretek
    }
asset-meta-triangles = Háromszögek
asset-meta-vertices = Csúcspontok
asset-meta-meshes = Hálók
asset-meta-materials = Anyagok
asset-meta-animations = Animációk
asset-meta-skins = Skinek
asset-meta-unreal-asset = Unreal-asset
asset-meta-unreal-saved-with = { $class }, mentve: UE { $engine }
asset-meta-package = Csomagfájl
asset-meta-cooked = Cookolt (játékbuild): az előnézetek és a források el vannak távolítva
asset-meta-tiling = Csempézés
asset-meta-seamless = Mindkét tengelyen varratmentes
asset-meta-analysis-error = Nem sikerült teljesen beolvasni ezt a fájlt: { $error }
asset-copy-blocked = Nem sikerült másolni: a böngésző letiltotta
asset-save-error = Nem sikerült menteni: { $error }
asset-tag-removed = Eltávolítva: { $tag }
asset-tag-restored = Visszatéve: { $tag }
asset-tags-added =
    Hozzáadva: { $count ->
        [one] { $count } címke
       *[other] { $count } címke
    }
asset-tags-accepted =
    { $count ->
        [one] { $count } tippelt címke megtartva
       *[other] { $count } tippelt címke megtartva
    }
asset-reviewed = Átnézve
asset-reviewed-on-title = Átnézve: a szabályok és az MI nem nyúlnak a címkéihez. Kattintson a jelölés levételéhez (R)
asset-reviewed-off-title = Megjelölés átnézettként, hogy a szabályok és az MI ne nyúljanak a címkéihez (R)
asset-reviewed-on = Megjelölve átnézettként
asset-reviewed-off = Átnézett jelölés levéve
# $name is an asset's name.
asset-reviewed-on-named = { $name } megjelölve átnézettként
asset-reviewed-off-named = Átnézett jelölés levéve: { $name }
asset-undo = Visszavonás
asset-play-error = Nem sikerült lejátszani ezt a fájlt: { $error }
asset-copied-hash = Hash másolva
asset-copied-toml = toml másolva
asset-copied-id = Azonosító másolva
asset-copied-link = Hivatkozás másolva
asset-origin-before = Ez a fájl eredetileg:
asset-origin-class-engine = , típusa: { $class }, mentve: UE { $engine }.
asset-origin-class = , típusa: { $class }.
asset-origin-engine = { " " }(mentve: UE { $engine }).
asset-origin-end = { "." }
asset-origin-editor = Az Unreal Editor exportálta ekkor: { $date }.
asset-origin-editor-notes = Az Unreal Editor exportálta ekkor: { $date } ({ $notes }).
asset-origin-hoard = A Hoard olvasta ki a csomagfájlból ekkor: { $date }.
asset-origin-hoard-notes = A Hoard olvasta ki a csomagfájlból ekkor: { $date } ({ $notes }).
asset-origin-download = Az eredeti .{ $ext } letöltése
asset-origin-discarded = Az eredeti csomagfájl az átalakítás után el lett vetve.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Előzmények
history-loading = Betöltés…
history-none = Itt még nem változott semmi.
history-load-failed = Nem sikerült betölteni az előzményeket: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = MI
history-by-rule = Egy szabály
history-by-someone = Valaki
history-by-app = { $who } egy alkalmazáson keresztül
history-by-assistant = { $who } egy asszisztensen keresztül
history-field-description = Leírás
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Címke hozzáadva: { $tag }
history-tag-removed = Címke eltávolítva: { $tag }
history-tag-kept = Javasolt címke megtartva: { $tag }
history-tag-changed = Címke módosítva: { $tag }
history-reviewed = Átnézettnek jelölve
history-unreviewed = Nem átnézettnek jelölve
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: beállítva erre: { $to }
history-cleared = { $field }: törölve (ez volt: { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Visszavonás
history-revert-title = A módosítás előtti állapot visszaállítása
history-reverted = Visszavonva: { $change }
history-revert-failed = Nem sikerült visszavonni: { $error }
asset-details = Asset részletei
asset-in-pack = ·
asset-previous = Előző asset
asset-next = Következő asset
asset-favorite = Kedvenc
asset-favorite-on-title = A kedvencei között (az F eltávolítja)
asset-favorite-off-title = Hozzáadás a kedvencekhez (F)
asset-share = Megosztás
asset-share-title = Hivatkozás megosztása erre az assetre (aki megnyitja, előbb bejelentkezik a Hoardba)
asset-layout-panel = Megjelenítés oldalpanelen
asset-layout-page = Megjelenítés teljes oldalon
asset-layout-page-title = Megjelenítés teljes oldalon: nagy előnézet a részletek mellett
asset-close = Részletek bezárása
asset-rename-title = Kattintson az átnevezéshez
asset-name = Név
asset-views-alt = A modell négy irányból
asset-inspect-aria = Teljes képernyős vizsgálat: { $name }
asset-inspect-title = Vizsgálat: nagyítás, csempézés, csatornák
asset-playback-position = Lejátszási pozíció
asset-playback-value = { $current } / { $duration }
asset-pause = Szünet
asset-play = Lejátszás
asset-waveform-drawing = Hullámforma rajzolása…
asset-waveform-skipped = Ilyen nagy fájlokhoz nincs hullámforma
asset-play-animation = Lejátszás animációként
asset-show-views = A címkéző által látott négynézetes lap megjelenítése
asset-palette = A kép színei
asset-swatch-title = { $hex }, a képből { $percent }%. Ilyen színű assetek keresése
asset-swatch-aria = Assetek keresése ezzel a színnel: { $hex } (a képből { $percent }%)
asset-more-like-this = Több ilyen
asset-desc-label = Saját leírás
asset-desc-placeholder = Mi ez, mire használná…
asset-save = Mentés
asset-cancel = Mégse
asset-desc-family = Mindegyik változatba mentődik ({ $count })
asset-desc-edit = Leírás szerkesztése
asset-desc-write = Leírás írása
asset-desc-ai = MI írta
asset-desc-file = A fájlból
asset-desc-store-before = Forrás:
asset-desc-store-page = { $store } áruházi oldal
asset-desc-store-page-unnamed = áruházi oldal
asset-desc-store-after = (a teljes csomagra vonatkozik)
asset-less = Kevesebb
asset-more = Több
asset-tags = Címkék
asset-tag-add-label = Címke hozzáadása
asset-tag-add-placeholder = Címke hozzáadása, pl. mood:tense
asset-apply-family = A módosítások alkalmazása mindegyik változatra ({ $count })
asset-tags-accept-all =
    { $count ->
        [one] { $count } tipp megtartása
       *[other] Mind a(z) { $count } tipp megtartása
    }
asset-tags-accept-all-title = A szabályok és az MI által tippelt összes címke megtartása saját címkeként
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Elutasítva:
asset-tag-restore-title = Visszatétel: { $tag }
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = A hasonlókon ezek vannak:
asset-tag-suggest-title = { $tag } hozzáadása
asset-details-heading = Részletek
asset-field-category = Kategória
asset-field-license = Licenc
asset-field-source = Forrás
asset-field-rating = Értékelés
# Shown in place of an empty value, to fill it in.
asset-field-add = Hozzáadás
asset-field-new = Mező hozzáadása
asset-field-name = Mező neve
asset-field-value = Érték
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = a csomagból
asset-field-license-placeholder = Például CC0 vagy CC-BY 4.0
asset-field-remove-placeholder = Üresen hagyva törli
# $field is a field's name, like "License".
asset-field-edit-title = { $field } szerkesztése
asset-field-saved = Mentve: { $field }
asset-field-cleared = Törölve: { $field }
asset-rate =
    { $count ->
        [one] { $count } csillag
       *[other] { $count } csillag
    }
asset-rated =
    { $count ->
        [one] Értékelés: { $count } csillag
       *[other] Értékelés: { $count } csillag
    }
asset-rating-clear-title = Kattintson újra az értékelés törléséhez
asset-rating-cleared = Értékelés törölve
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Kattintson újra: { $count } csillag
       *[other] Kattintson újra: { $count } csillag
    }
asset-collections = Gyűjtemények
asset-collection-remove = Eltávolítás innen: { $name }
asset-collection-add-another = Hozzáadás máshoz
asset-collection-add = Hozzáadás gyűjteményhez
asset-used-in = Használat helye
asset-used-in-title = A projekt tartalmának megjelenítése: { $name }
asset-used-in-synced = szinkronizálva: { $when }
asset-maps = Térképek
asset-show-more = További { $count } megjelenítése
asset-materials-heading =
    { $count ->
        [one] Anyag
       *[other] Anyagok
    }
asset-material-maps-aria = { $name }: térképek
asset-variants = Változatok ({ $count } verzió)
asset-fit-animations = Illeszkedő animációk ({ $count })
asset-fit-characters = Karakterek, amelyekre illik ({ $count })
asset-copies = Azonos másolatok
asset-copies-note = Az azonos bájtok egyszer vannak tárolva; ezek a csomagok is tartalmazzák ezt a fájlt.
asset-near-copies = Majdnem azonos másolatok
asset-near-copies-note = Ugyanaz más méretben, mintavételezési frekvenciával vagy formátumban. A legjobb másolat meg van jelölve.
asset-near-copies-best = Ugyanaz más méretben, mintavételezési frekvenciával vagy formátumban. Ez a legjobb másolat.
asset-near-best = legjobb
asset-unreal-converted = Unrealből átalakítva
asset-rule-converted = Szabály alakította át
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Egy átalakítási szabály készítette ezt a { $format } fájlt ({ $date }) ebből:
asset-rule-kept-until = Az eredeti megmarad eddig: { $date }.
asset-rule-rebuilt = A FLAC mindent megőriz, ami a WAV-ban volt, így a WAV bármikor pontosan az eredeti formájában letölthető vagy visszaállítható.
asset-rule-put-back = Az eredeti visszaállítása
asset-rule-put-back-title = Visszaállítja ezt: { $file }?
asset-rule-put-back-body = Az átalakított fájl törlődik, és az eredeti visszakerül a helyére a címkéivel, gyűjteményeivel és hivatkozásaival együtt. Az átalakítási szabályok ezután nem nyúlnak hozzá.
asset-rule-put-back-done = { $file } visszaállítva
asset-rule-let-go = Az eredeti törlődött, amikor lejárt a Beállításokban megadott megőrzési idő.
asset-unreal-package = Az Unreal-csomagfájlból
asset-ue-needs-editor-before = Unreal Editorra vár: { $reason }. Egy olyan gépen, ahol telepítve van az Unreal, futtassa ezt:
asset-ue-needs-editor-after = – így FBX-ként érkezik vissza.
asset-ue-editor-path = az UnrealEditor-Cmd elérési útja
asset-ue-unsupported = Csomagfájlként megtartva: { $reason }.
asset-ue-failed = Az átalakítás sikertelen: { $reason }
asset-recovered = Az eredeti .{ $ext } fájl ebben a csomagfájlban volt tárolva, és a Hoard kinyerte.
asset-recovered-download = .{ $ext } letöltése
asset-ue-meshes-before = A hálók és az animációk az Unreal saját formátumában vannak tárolva. Alakítsa át a csomagot (csomagoldal, Továbbiak), majd egy Unreal Editor-telepítés mellett futtassa ezt:
asset-ue-meshes-after = – így FBX-et kap.
asset-ue-registry-tags = { $count } asset registry címke
asset-file = Fájl
asset-bytes = Bájt
asset-source = Forrás
asset-path = Elérési út
asset-path-open-folder = Mi van még ebben: { $folder }
asset-download-maps = Összes térkép letöltése ({ $count })
asset-download-linked = Letöltés { $count } kapcsolt fájllal
asset-download-only-title = Csak ez a fájl, az általa használt fájlok nélkül
asset-download-only = Csak ez a fájl
asset-include-animations = Animációk hozzáadása a .zip fájlhoz
asset-include-character = A(z) { $name } karakter hozzáadása
asset-copy-toml = hoard.toml-bejegyzés másolása
asset-copy-id = Azonosító másolása
asset-teach = A fájl megtanítása a Hoardnak
asset-teach-title = Írja le ennek az áruháznak a fájlját, hogy a Hoard be tudja olvasni (Beállítások → Áruházi fájlok)
asset-archive = Archiválás
asset-archive-title = Kikerül a keresésből, amíg vissza nem állítja vagy végleg nem törli az Archívumból

## Képmegjelenítő

image-channel-red = Vörös csatorna
image-channel-green = Zöld csatorna
image-channel-blue = Kék csatorna
image-channel-alpha = Alfa csatorna
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Sakktábla háttér (K: váltás)
image-bg-black-title = Fekete háttér (K: váltás)
image-bg-gray-title = Szürke háttér (K: váltás)
image-bg-white-title = Fehér háttér (K: váltás)
image-bg-checker = sakktábla háttér
image-bg-black = fekete háttér
image-bg-gray = szürke háttér
image-bg-white = fehér háttér
image-inspect-aria = Vizsgálat: { $name }
image-preview-size = előnézet { $width }×{ $height }, { $bytes }
image-preview = előnézet
image-resolution = Felbontás
image-full-title = A teljes, { $width }×{ $height } méretű kép betöltése (F); az előnézeten túli nagyítás is betölti
image-full-loading = Teljes felbontás betöltése…
image-full-failed = A teljes felbontás nem töltődött be: újrapróbálás
image-full = Teljes felbontás
image-zoom = Nagyítás
image-zoom-out-title = Kicsinyítés (−)
image-zoom-out = Kicsinyítés
image-zoom-in-title = Nagyítás (+)
image-zoom-in = Nagyítás
image-fit-title = Igazítás (0)
image-fit = Igazítás
image-actual-title = Tényleges képpontok (1)
image-view = Nézet
image-tile-title = 3×3-as csempézés a varratok ellenőrzéséhez (T)
image-tile = Csempézés
image-channels = Csatornák
image-close = Bezárás (Esc)
image-failed = Ezt a képet nem sikerült betölteni.
image-loading = Betöltés…
image-key-gestures = Görgetés vagy csippentés: nagyítás · húzás: mozgatás · dupla kattintás: nagyítás vagy igazítás
image-key-tile = csempézés
image-key-background = háttér
image-key-full = teljes felbontás
image-key-channels = csatornák

## Sprite-lejátszó

sprite-frame-aria = { $name }, képkocka: { $frame } / { $count }
sprite-failed = Ez nem játszható le.
sprite-loading = Képkockák betöltése…
sprite-previous = Előző képkocka
sprite-next = Következő képkocka
sprite-pause = Animáció szüneteltetése
sprite-play = Animáció lejátszása
sprite-animation = Animáció
sprite-all-frames = Összes képkocka
sprite-grid-title = Hogyan van a lap képkockákra vágva
sprite-columns = Oszlopok
sprite-rows = Sorok
sprite-fps-aria = Képkocka másodpercenként
sprite-fps = fps

## Spritelap-megjelenítő (SheetView.vue) és letöltési formátumai (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
       *[other] { $count } sprite
    }
sheet-pages =
    { $count ->
        [one] { $count } oldal
       *[other] { $count } oldal
    }
sheet-animations =
    { $count ->
        [one] { $count } animáció
       *[other] { $count } animáció
    }
# The box that narrows the list of sprites by name.
sheet-find = Sprite keresése
# Under the list of sprites when more match than it draws.
sheet-showing = Megjelenítve: { $shown } / { $total }
sheet-no-match = Egyik sprite nevében sincs ilyen.
sheet-failed = Ez a lap nem olvasható.
sheet-loading = Lap betöltése…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = A Hoard saját formátuma: az oldalak úgy, ahogy vannak, a sprite-ok bármilyen irányba elforgatva és körbevágva, animációkkal és minden képkocka idejével.
sheet-note-texturepacker = A sprite-ok az óramutató járásával megegyezően elforgatva és körbevágva, név szerinti animációkkal, ahogy a PixiJS és a Phaser olvassa őket.
sheet-note-clockwise = A sprite-ok az óramutató járásával megegyezően elforgatva és körbevágva. Animációkat nem tud tárolni: a képkockákat a számozott nevük azonosítja.
sheet-note-libgdx = A sprite-ok az óramutató járásával ellentétesen elforgatva és körbevágva. Animációkat nem tud tárolni: a képkockákat a számozott nevük azonosítja.
sheet-note-godot-frames = A sprite-ok sosincsenek elforgatva, a levágott szélük margóként megmarad, animációkkal és minden képkocka idejével.
sheet-note-godot-tileset = A csempék sosincsenek elforgatva vagy körbevágva, csempeméretű rácson állnak, és a képek változatlanok maradnak, ha már rácson vannak.
sheet-note-grid = Minden sprite állítva, a teljes vásznával, egyforma méretű cellákban, bármihez, ami rácsra vágja a lapot.
sheet-note-frames = Minden sprite állítva, a teljes vásznával, mindegyik egy saját nevét viselő PNG-fájlban.

## Szövegmegjelenítő

text-lang-config = Konfiguráció
text-lang-table = Táblázat
text-lang-code = Kód
text-lang-script = Szkript
text-lang-text = Szöveg
text-copy-blocked = Nem sikerült másolni: a böngésző letiltotta
text-read-aria = Olvasás: { $name }
text-lines = { $count } sor
text-show-as = Megjelenítés
text-tree-title = Összecsukható fa (T)
text-tree = Fa
text-pretty-title = Újratördelve (P)
text-pretty = Formázott
text-raw-title = Ahogy meg van írva (R)
text-raw = Nyers
text-document-title = Dokumentumként (D)
text-document = Dokumentum
text-source-title = A markdown, ahogy meg van írva (R)
text-source = Forrás
text-page-title = Ahogy egy böngésző mutatja a mappájából, a Hoardtól elkülönítve (P)
text-page = Oldal
text-page-source-title = Az oldal, ahogy meg van írva (R)
text-page-aria = { $name } oldalként
text-page-error = Nem sikerült megnyitni ezt az oldalt: { $error }
text-fold = Összecsukás
text-expand-all-title = Minden kibontása
text-expand-all = Mind kibontása
text-collapse-all-title = Minden összecsukása
text-collapse-all = Mind összecsukása
text-group-text = Szöveg
text-wrap-title = Hosszú sorok tördelése (W)
text-wrap = Tördelés
text-copy-title = A szöveg másolása (C)
text-copied = Másolva
text-copy = Másolás
text-download-title = A fájl letöltése
text-download = Letöltés
text-close = Bezárás (Esc)
text-truncated = Az első { $shown } látható (összesen { $total }).
text-truncated-json = Az első { $shown } látható (összesen { $total }; a fához a teljes fájl kell).
text-show-all = Összes megjelenítése
text-json-error = Ez a JSON nem értelmezhető: { $error }
text-contents-aria = Tartalom: { $name }
text-loading = Betöltés…
text-load-error = Nem sikerült betölteni ezt a fájlt: { $error }
text-binary = Ez a fájl bináris, nem szöveg, így itt nincs mit olvasni. Töltse le, és nyissa meg a hozzá való programmal.
text-key-tree = fa
text-key-pretty = formázott
text-key-raw = nyers
text-key-document = dokumentum
text-key-source = forrás
text-key-page = oldal
text-key-wrap = tördelés
text-key-copy = másolás
text-key-find = keresés
text-key-close = bezárás
text-preview-aria = Olvasás teljes képernyőn: { $name }
text-preview-read = Olvasás

## JSON-fa és JSON-választó

json-items = { $count } elem
json-keys = { $count } kulcs
json-fold = Összecsukás
json-unfold = Kibontás
json-unfold-summary = Kibontás: { $summary }
json-show-more = További { $count } megjelenítése
json-left = (összesen { $count } van hátra)
json-picker-more = … még { $count }

## 3D-modellmegjelenítők

model-viewer-failed = a 3D-megjelenítő nem töltődött be ({ $error }); az oldal újratöltése segíthet
model-loading-viewer = A 3D-megjelenítő betöltése…
model-loading = Modell betöltése…
model-show-failed = Nem sikerült megjeleníteni ezt a modellt: { $error }
model-preview-failed = Nem sikerült előnézetet készíteni ehhez a modellhez: { $error }
model-error-converting = a szerver még átalakítja ezt a modellt
model-error-converting-desktop = A Hoard még átalakítja ezt a modellt
model-error-empty-collada = üres Collada-fájl
model-error-no-loader = Nincs böngészős betöltő ehhez: .{ $ext }
model-dialog = 3D-megtekintés: { $name }
model-title-stats = { $triangles } háromszög · { $meshes } háló · { $materials } anyag
model-triangles = { $count } háromszög
model-group-camera = Kamera
model-group-shading = Árnyalás
model-group-background = Háttér
model-group-animation = Animáció
model-group-view = Nézet
model-group-show = Megjelenítés
model-frame-title = A modell keretezése (F)
model-frame = Keretezés
model-reset-title = Vissza a kezdő nézethez (R)
model-reset = Alaphelyzet
model-turn-title = Lassú forgatás (A)
model-turn = Forgatás
model-textures-title = Textúrák (T)
model-textures = Textúrák
model-full-res-title = Teljes felbontású textúrák (H): minden textúrafájl eredeti méretben, bármekkora is; kikapcsolva legfeljebb 2048 px-es könnyű előnézetek
model-full-res = Teljes felb.
model-light-title = Környezeti fény (E)
model-light = Fény
model-grid-title = Padlórács (G)
model-grid = Rács
model-bg-checker = Sakktábla háttér
model-bg-dark = Sötét háttér
model-bg-gray = Szürke háttér
model-bg-light = Világos háttér
model-bg-env = Környezeti háttér
model-bg-title = { $background } (K: váltás)
model-pause-title = Szünet (Szóköz)
model-play-title = Lejátszás (Szóköz)
model-pause = Szünet
model-play = Lejátszás
model-animation-clip = Animációs klip
model-animation = Animáció
model-plays-on = Lejátszás ezen
model-rest-pose = Alappóz
model-close-title = Bezárás (Esc)
model-full-screen = Teljes képernyő
model-open-full-screen = A teljes képernyős 3D-megjelenítő megnyitása
model-menu-fold = A menü összecsukása
model-menu-show = A menü megjelenítése
model-view-prefix = Nézet:{ " " }
model-menu-render = Renderelés
model-menu-maps = Térképek
model-menu-inspect = Vizsgálat
model-mode-final = Végső render
model-mode-raw = Nyers render
model-mode-color = Alapszín
model-mode-roughness = Érdesség
model-mode-metalness = Fémesség
model-mode-normal = Normál
model-mode-ao = Takarás
model-mode-matcap = Matcap
model-mode-uv = UV-ellenőrző
model-mode-geometry = Geometriák
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Drótváz (W), bármely nézet fölött
model-wireframe = Drótváz
model-info-title = Jelenetinformáció (I)
model-info = Jelenetinformáció
model-info-close = Jelenetinformáció bezárása
model-info-close-title = Bezárás (I)
model-info-geometries = Geometriák
model-info-materials = Anyagok
model-info-textures = Textúrák
model-info-triangles = Háromszögek
model-info-vertices = Csúcspontok
model-info-meters = A glTF méterben van
model-info-no-units = Ez a formátum nem adja meg a mértékegységét
model-info-height = Magasság
model-info-length = Hosszúság
model-info-width = Szélesség
model-length-units = { NUMBER($n, maximumSignificantDigits: 3) } egység
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Nem található textúra: { $name }
model-missing-some = { $count } textúra nem található: { $names }
model-missing-more = { $count } textúra nem található: { $names }…
model-missing-count = { $count } textúra nem található
model-guessed = A textúrák név alapján lettek párosítva; a fájl egyet sem nevez meg.
model-group-facing = Tájolás
model-facing-turn = Negyed forgatás
model-facing-turn-title = A modell elforgatása negyed fordulattal az óramutató járásával ellentétesen, felülről nézve
model-facing-tip = Negyed billentés
model-facing-tip-title = A modell tetejének előrebillentése negyed fordulattal, hogy egy fekvő modell felálljon
model-facing-save = Alkalmazás a csomagra
model-facing-save-title = Minden modell elforgatása így ebben a csomagban: { $pack }. A bélyegképeik újra elkészülnek; a letöltések úgy maradnak, ahogy elkészültek.
model-facing-undo = Visszavonás
model-facing-undo-title = Vissza ahhoz, ahogy a csomag modelljei el vannak forgatva
model-facing-saved = Most már minden modell így áll ebben a csomagban: { $pack }. A bélyegképeik újra elkészülnek.
model-facing-failed = Nem sikerült elforgatni a csomag modelljeit: { $error }
model-keys-mouse = Húzás: körbeforgatás · jobb gombos húzás: mozgatás · görgetés: nagyítás · dupla kattintás: keretezés
model-key-frame = keretezés
model-key-reset = alaphelyzet
model-key-view = nézet
model-key-wireframe = drótváz
model-key-info = jelenetinformáció
model-key-textures = textúrák
model-key-full-res = teljes felbontású textúrák
model-key-light = fény
model-key-grid = rács
model-key-background = háttér
model-key-play = lejátszás

## Anyagok

material-role-base-color = Szín
material-role-normal = Normál
material-role-normal-dx = Normál (DirectX)
material-role-roughness = Érdesség
material-role-gloss = Fényesség
material-role-metalness = Fémesség
material-role-ao = Ambient occlusion
material-role-orm = Takarás, érdesség, fémesség
material-role-mra = Fémesség, érdesség, takarás
material-role-rma = Érdesség, fémesség, takarás
material-role-height = Magasság
material-role-bump = Bump
material-role-cavity = Üregesség
material-role-specular = Spekuláris
material-role-opacity = Átlátszatlanság
material-role-emissive = Emisszió
material-role-translucency = Áttetszőség
material-role-base-color-name = szín
material-role-normal-name = normál
material-role-normal-dx-name = normál (DirectX)
material-role-roughness-name = érdesség
material-role-gloss-name = fényesség
material-role-metalness-name = fémesség
material-role-ao-name = ambient occlusion
material-role-orm-name = takarás, érdesség, fémesség
material-role-mra-name = fémesség, érdesség, takarás
material-role-rma-name = érdesség, fémesség, takarás
material-role-height-name = magasság
material-role-bump-name = bump
material-role-cavity-name = üregesség
material-role-specular-name = spekuláris
material-role-opacity-name = átlátszatlanság
material-role-emissive-name = emisszió
material-role-translucency-name = áttetszőség
material-maps = { $count } térkép
material-dialog = Anyag megtekintése: { $name }
material-up-to = legfeljebb { $size }
material-repeat-size = { $size } ismétlésenként
material-shape-sphere = Gömb
material-shape-cube = Kocka
material-shape-plane = Sík
material-shape-title = { $shape } ({ $key })
material-group-shape = Alakzat
material-group-tiling = Csempézés
material-real-title = Valós méret: egy ismétlés { $size }, a gömb átmérője 2 m (T: váltás)
material-repeat-title = { $n }× ismétlés (T: váltás)
material-real = Valós
material-repeat = { $n }×
material-height-title = Mennyire tolja ki a felületet a magasságtérkép
material-height = Magasság
material-height-strength = Magasság erőssége
material-loading = Anyag betöltése…
material-show-failed = Nem sikerült megjeleníteni ezt az anyagot: { $error }
material-preview-failed = Nem sikerült előnézetet készíteni ehhez az anyaghoz: { $error }
material-group-maps = Térképek
material-map-show = Térkép megjelenítése: { $map }
material-map-hide = Térkép elrejtése: { $map }
material-map-unused = Az előnézet nem használja
material-open-full-screen = A teljes képernyős anyagmegjelenítő megnyitása
material-keys-mouse = Húzás: körbeforgatás · jobb gombos húzás: mozgatás · görgetés: nagyítás · kattintás egy térképre: kikapcsolás
material-key-shape = alakzat
material-key-tiling = csempézés
material-key-turn = forgatás

## Témák

theme-slate = Pala
theme-paper = Papír
theme-linen = Len
theme-sage = Zsálya
theme-lilac = Orgona
theme-blush = Pír
theme-graphite = Grafit
theme-obsidian = Obszidián
theme-ember = Parázs
theme-moss = Moha
theme-aubergine = Padlizsán
theme-oxblood = Bordó
theme-light-slate-blurb = Hűvös kékesszürke
theme-light-paper-blurb = Egyszerű semleges szürke
theme-light-linen-blurb = Meleg krém, borostyán
theme-light-sage-blurb = Lágy zöld
theme-light-lilac-blurb = Halvány ibolya
theme-light-blush-blurb = Fakó rózsa
theme-dark-graphite-blurb = Semleges szürke, árnyalat nélkül
theme-dark-slate-blurb = Kékesszürke
theme-dark-obsidian-blurb = Majdnem fekete, OLED-hez
theme-dark-ember-blurb = Meleg szénszürke, borostyán
theme-dark-moss-blurb = Erdőzöld-fekete
theme-dark-aubergine-blurb = Szilvafekete, orgona
theme-dark-oxblood-blurb = Borfekete, korall

## Textúranevek

texnames-needs-map = A mintában szerepelnie kell ennek: { "{" }map{ "}" }, különben egy anyag minden térképe ugyanazt a nevet kapná
texnames-placeholders = A minta ezeket használhatja: { "{" }name{ "}" }, { "{" }map{ "}" } és { "{" }size{ "}" }
texnames-bad-characters = A nevek nem tartalmazhatják ezeket: < > : " / \ | ? vagy *
texnames-no-suffix = Adjon utótagot ennek a térképtípusnak: { $role }
texnames-bad-suffix = A térképtípus utótagja ({ $role }) nem tartalmazhatja ezeket: < > : " / \ | ? vagy *
texnames-slot-no-suffix = Adjon utótagot ennek a helynek: „{ $slot }”
texnames-slot-bad-suffix = A hely utótagja („{ $slot }”) nem tartalmazhatja ezeket: < > : " / \ | ? vagy *
texnames-ending-missing = Minden saját végződéshez meg kell adni a keresendő végződést
texnames-ending-bad = A saját végződések nem tartalmazhatják ezeket: < > : " / \ | ? vagy *
texnames-ending-twice = { $ending } kétszer szerepel a végződései között
texnames-slot-metallic-roughness = Fémesség és érdesség
texnames-slot-specular-color = Spekuláris szín
texnames-slot-specular-glossiness = Spekuláris és fényesség
texnames-slot-clearcoat = Lakkréteg
texnames-slot-clearcoat-roughness = Lakkréteg érdessége
texnames-slot-clearcoat-normal = Lakkréteg normálja
texnames-slot-sheen-color = Selyemfény színe
texnames-slot-sheen-roughness = Selyemfény érdessége
texnames-slot-transmission = Áteresztés
texnames-slot-thickness = Vastagság
texnames-slot-diffuse-transmission = Szórt áteresztés
texnames-slot-diffuse-transmission-color = Szórt áteresztés színe
texnames-slot-iridescence = Irizálás
texnames-slot-iridescence-thickness = Irizálás vastagsága
texnames-slot-anisotropy = Anizotrópia

## Áruházi formátumok

storefiles-field-title = Cím
storefiles-field-title-hint = A hirdetés pontos címe. Ez adja a csomag nevét; nélküle a fájl kimarad.
storefiles-field-seller = Eladó
storefiles-field-seller-hint = Ki készítette vagy árulja.
storefiles-field-license = Licenc
storefiles-field-license-hint = Ahogy a fájl írja, például CC-BY-4.0.
storefiles-field-url = Áruházi oldal
storefiles-field-url-hint = A hirdetés címe, http:// vagy https:// kezdettel.
storefiles-field-words = Címkék
storefiles-field-words-hint = Szavak, amelyekkel fel kell címkézni. A vesszővel elválasztott értékek szétválnak.
storefiles-field-type = Típus
storefiles-field-type-hint = Mi ez, például „3D model” vagy „surface”. Címke is lesz belőle.
storefiles-field-colors = Színek
storefiles-field-colors-hint = A színei, color: címkékként.
storefiles-field-id = Áruházi azonosító
storefiles-field-id-hint = Az áruház azonosítója hozzá: betűk, számjegyek, _ vagy -.
storefiles-field-origin = Készült
storefiles-field-origin-hint = Hol készült vagy szkennelték, a leíráshoz.
storefiles-field-size = Méret
storefiles-field-size-hint = Mekkora, szavakkal, a leíráshoz.
storefiles-field-title-name = cím
storefiles-field-seller-name = eladó
storefiles-field-license-name = licenc
storefiles-field-url-name = áruházi oldal
storefiles-field-words-name = címkék
storefiles-field-type-name = típus
storefiles-field-colors-name = színek
storefiles-field-id-name = áruházi azonosító
storefiles-field-origin-name = készítés helye
storefiles-field-size-name = méret

## Áruházi formátumok panel

storefiles-load-failed = Nem sikerült betölteni az áruházi fájlformátumokat: { $error }
storefiles-remove-title = Eltávolítja ezt a formátumot: { $name }?
storefiles-remove-body = { $file } törlődik az adatmappából. A vele már beolvasott csomagok megtartják, amit beolvasott belőlük.
storefiles-remove = Eltávolítás
storefiles-removed = Eltávolítva: { $name }.
storefiles-remove-failed = Nem sikerült eltávolítani ({ $name }): { $error }
storefiles-unread = Fájlok, amelyeket a Hoard nem olvas be
storefiles-loading = Betöltés…
storefiles-unread-none = Nincs ilyen: a könyvtárban egyetlen fájl sem hasonlít be nem olvasott áruházi fájlra.
storefiles-unread-what = Több csomagban is előforduló vagy áruházi fájlhoz hasonló nevű fájlok, amelyeket egyik formátum sem olvas be.
storefiles-unread-packs = { $count } csomag
storefiles-unread-example = pl. { $pack }
storefiles-teach = A fájl megtanítása a Hoardnak
storefiles-formats = Formátumok
storefiles-formats-what = A sajátjait próbálja először, és ezek lépnek az azonos nevű beépítettek helyébe.
storefiles-dir-before = Tárolási helyük:
storefiles-dir-after = .
storefiles-yours = Saját
storefiles-built-in = Beépített
storefiles-edit = Szerkesztés
storefiles-start-from = Kiindulás ebből

## Címkék panel

tags-load-failed = Nem sikerült betölteni a címkéket: { $error }
tags-rename-title = Átnevezi ezt a címkét: { $tag }?
tags-merge-title = Összevon { $count } címkét?
tags-rename-body = A címkével rendelkező { $count } asset helyette ezt kapja: { $into }, és a címkézők mostantól az új nevet használják.
tags-merge-body = Ezek a címkék: { $tags } – legfeljebb { $count } asseten erre változnak: { $into }. A címkézők mostantól ezt használják: { $into }.
tags-rename = Átnevezés
tags-merge = Összevonás
tags-merged = { $count } asset címkéje mostantól: { $into }
tags-merge-failed = Nem sikerült összevonni: { $error }
tags-forget-failed = Nem sikerült módosítani: { $error }
tags-find = Címkék keresése
tags-find-placeholder = Címkék keresése, pl. rock vagy set:
tags-sort = Sorrend
tags-sort-count = Leggyakoribb
tags-sort-name = A–Z
tags-count = { $count } címke
tags-count-more = { $count }+ címke
tags-what = Jelölje ki az azonos jelentésű címkéket, és vonja össze őket, vagy jelöljön ki egyet az átnevezéséhez.
tags-new-name = Új név
tags-merge-into = Összevonás ebbe
tags-saved-as-before = Mentve ezen a néven:
tags-saved-as-after = { "" }
tags-cancel = Mégse
tags-loading = Betöltés…
tags-none = Nincsenek címkék.
tags-none-containing = Nincs ezt tartalmazó címke: „{ $text }”.
tags-browse-title = Ezzel a címkével ellátott assetek böngészése: { $tag }
tags-browse = Böngészés
tags-merged-heading = Összevont címkék
tags-merged-what = Ha a címkézők valamelyik régi nevet adnák, az asset azt a címkét kapja, amelybe összevonták.
tags-allow-old = A régi név újbóli engedélyezése
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count = { $count } asset
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
       *[other] Régi név: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = A(z) { $tag } összevonásra került ide: { $into }.
tag-page-show = { $tag } megjelenítése
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Írja le, mit jelent ez a címke…
tag-page-describe-label = Mit jelent ez a címke
tag-page-describe-placeholder = Mire való, mi tartozik alá. Az Enter ment.
tag-page-describe-edit = Kattintson a módosításhoz
tag-page-description-saved = A címke jelentése elmentve
# The tag's picture: one of its assets stands for it.
tag-page-picture = Válasszon képet ehhez a címkéhez
tag-page-picture-help = Válassza ki az egyik assetjét, amely képviseli
tag-page-picture-nothing = A megjelenített assetek egyikének sincs képe.
tag-page-picture-none = Nincs kép
tag-page-picture-saved = Kép beállítva
tag-page-picture-removed = Kép eltávolítva
tag-page-rename = Átnevezés…
tag-page-rename-title = Adjon neki új nevet, vagy egy másik címke nevét, hogy abba összevonja
tag-page-rename-label = Új név
tag-page-save-failed = Nem sikerült menteni a címkét: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Alá tartozik
tag-page-parent-add = Alárendelés…
tag-page-parent-add-title = Rendelje ezt a címkét egy tágabb alá: ha azt keresi, ezeket is megtalálja
tag-page-parent-placeholder = Egy tágabb címke, például jármű
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = A(z) { $tag } kivétele a(z) { $parent } alól
tag-page-parent-saved = A(z) { $tag } mostantól a(z) { $parent } alá tartozik
tag-page-parent-removed = A(z) { $tag } már nem tartozik a(z) { $parent } alá
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Szűkebbek:
tag-page-narrower-title = A(z) { $tag } keresése ezeket is megtalálja
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Tágabb és szűkebb
tags-tree-what = A címkeoldalukon tágabb címkék alá rendelt címkék. Ha egy címkére keres, mindent megtalál, ami alatta van.

## Könyvtárstatisztika

stats-source-human = Emberek adták hozzá
stats-source-ai = MI javasolta
stats-source-rule = Beépített szabályok
stats-source-meta = A fájlokból
stats-assets = Assetek
stats-packs = Csomagok
stats-on-disk = A lemezen
stats-duplicates-title = { $count } fájl bájtra azonos másolata másoknak, ezért egyszer van tárolva
stats-saved-dedup = Deduplikációval megtakarítva
stats-distinct-tags = Különböző címkék
stats-archive = Az archívumban
stats-load-failed = Nem sikerült betölteni a részleteket: { $error }
stats-counting = Számolás…
stats-as-of = Állapot: { $time }
stats-updating = Frissítés…
stats-by-kind = Hely típusonként
stats-files-size = { $name }: { $count } fájl, { $size }
stats-months = Az elmúlt évben hozzáadva
stats-month = { $month }: { $count } asset, { $size }
stats-tagging = Címkézés
stats-tagged = Címkézett
stats-untagged = { $count } címke nélkül
stats-described = Leírással
stats-described-assets = { $count } asset
stats-human-edited = Kézzel szerkesztve
stats-human-edited-unit =
    { $count ->
       *[other] asset
    }
stats-source-different = { $count } különböző
stats-most-used = Leggyakoribb
stats-show-tag = Assetek ezzel a címkével: { $tag }
stats-file-types = Fájltípusok
stats-biggest-packs = Legnagyobb csomagok
stats-browse-pack = Böngészés: { $name }
stats-largest-files = Legnagyobb fájlok
stats-open-file = Megnyitás: { $name }
stats-empty = A könyvtár még üres. Adjon hozzá egy csomagot, és itt megjelennek a számai.

## HoardPanel.vue

hoard-title = Az Ön kincse
hoard-what = A könyvtár minden assetje egy kincsdarab, és a fajtája dönti el, melyik. Gyűjtse be az újakat, hogy a kupacához kerüljenek.
hoard-sign-in = Jelentkezzen be, hogy saját kincset gyűjthessen.
hoard-new = Begyűjthető új zsákmány: { $count }
hoard-none-new = Nincs új begyűjtenivaló.
hoard-collect = Begyűjtés
hoard-unavailable = Ez a böngésző nem tudja megrajzolni a kincset: { $error }
hoard-collect-failed = Nem sikerült menteni, amit begyűjtött: { $error }
hoard-arcade = Játéktermi képernyő
hoard-full-screen = Teljes képernyő
hoard-leave-full-screen = Kilépés a teljes képernyőből
hoard-label = Egy sárkány őrzi a kincsét. Kincsdarabok: { $count }
hoard-gold = Arany
hoard-gems = Drágakövek
hoard-books = Könyvek
hoard-weapons = Fegyverek
hoard-bones = Csontok
hoard-tablets = Rúnatáblák
hoard-paintings = Festmények
hoard-potions = Bájitalok

## UpdatesPanel.vue

updates-saved = A Hoard meghívja, amikor frissítés érhető el
updates-saved-none = Nincs meghívandó cím
updates-save-failed = Nem sikerült menteni
updates-call-answered = A válasza: { $result }
updates-call-refused = Nem fogadta el a kérést: { $result }
updates-call-failed = Nem sikerült meghívni
updates-called = Utolsó hívás: { $when } ({ $version }) – válasz: { $result }
updates-called-failed = Utolsó hívás: { $when } ({ $version }) – válasz: { $result }. A Hoard egy óra múlva újrapróbálja.
updates-latest = A Hoard { $version } a legújabb kiadás.
updates-out = Megjelent a Hoard { $version }.
updates-out-idle = Megjelent a Hoard { $version }; a hívás néhány percen belül megtörténik, amikor épp semmi sincs folyamatban.
updates-out-busy = Megjelent a Hoard { $version }; a hívás megvárja, amíg semmi sincs folyamatban ({ $work }).
updates-busy-more = még { $count }
updates-busy-jobs = { $count } feladat vár
updates-busy-uploads = { $count } feltöltés
updates-list-separator = ,{ " " }
updates-what = Ha megjelenik egy új kiadás, a Hoard meghívhat egy címet, amint semmi sincs folyamatban (nincs feltöltés, feladat, átalakítás vagy karbantartás), így a konténerét frissítő eszköz nem indítja újra feltöltés közben. POST kérést küld, a kiadással egy JSON-törzsben. Ehhez a fenti kiadásellenőrzés szükséges.
updates-url = Meghívandó cím
updates-url-placeholder = Nincs (pl. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization-fejléc
updates-auth-saved = Mentve (gépeljen a cseréhez)
updates-auth-placeholder = Nincs (pl. Bearer my-token)
updates-remove = Eltávolítás
updates-save = Mentés
updates-call-now-title = Lefut, amihez csatlakoztatva van
updates-call-now = Meghívás most
updates-what-calls = Mit hívhat meg?
updates-watchtower = A Watchtower HTTP API-ja
updates-watchtower-at = :{ " " }
updates-watchtower-with = , fejléc:{ " " }
updates-watchtower-token = { " " }és a token innen:{ " " }
updates-watchtower-its = { " " }(az{ " " }
updates-watchtower-endpoint = { " " }végpontja legyen bekapcsolva).
updates-portainer = Egy Portainer-webhook
updates-portainer-rest = { " " }a Hoard stackjén vagy konténerén, bekapcsolt „Re-pull image” beállítással. A token az URL-ben van.
updates-script = Egy szkript a gazdagépen
updates-script-listener = { " " }egy webhook-figyelő mögött (például{ " " }
updates-script-runs = ), amely ezt futtatja:{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }vagy bármi más, ami webhookot fogad.
updates-poll-before = Vagy cím nélkül: egy hozzáférési tokennel rendelkező szkript rendszeresen lekérdezheti ezt:{ " " }
updates-poll-until = { " " }– addig, amíg az{ " " }
updates-poll-and = { " " }mező be nem áll, és az{ " " }
updates-poll-after = { " " }értéke true nem lesz –, majd lehúzhatja és újraindíthatja a Hoardot.

## MaintenancePanel.vue

maintenance-load-failed = Nem sikerült betölteni a Karbantartást
maintenance-reanalyze-title = Újraelemzi az összes csomagot?
maintenance-reanalyze-body = Minden fájl újra beolvasódik, ami egy nagy könyvtárnál sokáig tarthat; a csomagok folyamatban lévőként látszanak, amíg sorra nem kerülnek. A saját nevei, címkéi és jegyzetei megmaradnak.
maintenance-reanalyze-confirm = Újraelemzés
maintenance-gc-title = Törli a nem használt fájlokat?
maintenance-gc-body = Azok a tárolt fájlok, amelyekre semmi sem hivatkozik a könyvtárban, végleg törlődnek. Az archivált csomagok megtartják a sajátjukat.
maintenance-gc-confirm = Törlés
maintenance-start-failed = „{ $task }” nem indult el
maintenance-just-now = épp most
maintenance-minutes-ago = { $count } perce
maintenance-hours-ago = { $count } órája
maintenance-took-seconds = { $count } mp
maintenance-took-minutes = { $count } perc
maintenance-took-hours = { $hours } ó
maintenance-loading = Betöltés…
maintenance-running = Fut…
maintenance-starting = Indítás…
maintenance-run = Futtatás
maintenance-stop = Leállítás
maintenance-stopping = Leállítás folyamatban…
maintenance-stop-failed = „{ $task }” nem állítható le
maintenance-started = { $detail } · indult: { $when }
maintenance-starting-detail = Indítás
maintenance-finished = Befejeződött:{ " " }
maintenance-failed = Sikertelen:{ " " }
maintenance-never-run = Még nem futott
maintenance-history = Korábbi futások
maintenance-col-task = Feladat
maintenance-col-finished = Befejeződött
maintenance-col-took = Időtartam
maintenance-col-result = Eredmény
maintenance-script-before = A biztonsági mentések és a sérülésellenőrzés szkriptből is futtathatók:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }és{ " " }
maintenance-script-after = . Ezek a futások is itt jelennek meg.

## SettingsPage.vue

settings-title = Beállítások
settings-loading = Betöltés…
settings-find = Beállítás keresése
settings-find-none = Egy beállítás sem felel meg ennek: „{ $query }”.
settings-load-failed = Nem sikerült betölteni a beállításokat: { $error }
settings-page-appearance = Megjelenés
settings-page-appearance-blurb = A témák, amelyeket ez a böngésző világos és sötét módban használ. Csak ebben a böngészőben mentődnek.
settings-page-dragon = Sárkány
settings-page-dragon-blurb = A sárkányod kincse: a könyvtár minden eleme egy darabja.
settings-page-library = Könyvtár
settings-page-library-blurb = Mi van a hoardban: mekkora, miből áll, hogyan nőtt, és mennyire jól van felcímkézve. Az archivált elemek nem számítanak bele.
settings-page-copies = Majdnem azonos másolatok
settings-page-copies-blurb = Fájlok, amelyek ugyanaz a kép, hang vagy modell más méretben, mintavételezési frekvenciával vagy formátumban; elöl azok, amelyek a legtöbb helyet foglalják. Minden csoport legjobbja meg van jelölve; semmi sem törlődik.
copies-loading = Majdnem azonos másolatok keresése…
copies-refreshing = A könyvtár megváltozott, mióta ezeket megtaláltuk. Újrakeresés…
copies-none = Nincsenek majdnem azonos másolatok: minden kép, hang és modell csak egyszer van itt.
copies-load-failed = Nem sikerült listázni a majdnem azonos másolatokat: { $error }
copies-sum = Majdnem azonos másolatok csoportjai: { $count }. A nem legjobb másolatok { $size } helyet foglalnak.
copies-set = { $n }. csoport
copies-set-wasted = { $size } a nem legjobb másolatokban
copies-more = Több mutatása (még { $count })
settings-page-tags = Címkék
settings-page-tags-blurb = A könyvtár összes címkéje. Vonja össze az azonos jelentésűeket, vagy nevezzen át egyet; a címkézők ezután az új nevet használják.
settings-page-store-files = Áruházi fájlok
settings-page-store-files-blurb = Az áruházak által a grafikák mellé tett fájlok, amelyekből a Hoard kiolvassa a csomagok nevét és készítőit. Tanítsa meg neki azokat, amelyeket nem ismer, és ossza meg, amit megtanított.
settings-page-locations = Tárolási helyek
settings-page-locations-blurb = A szerver mappái, amelyekben a könyvtár van: amelyekben a Hoard fájlokat tárol, és csomagmappák, amelyeket a helyükön indexel.
settings-page-locations-blurb-desktop = A mappák, amelyekben a könyvtár van: amelyekben a Hoard fájlokat tárol, és csomagmappák, amelyeket a helyükön indexel.
settings-page-tools = Eszközök
settings-page-tools-blurb = Külső programok, amelyekkel a Hoard fájlokat alakít át és olvas be.
settings-page-ai = MI-címkézés
settings-page-ai-blurb = Egy MI-modell elnevezi, leírja és felcímkézi az új csomagokat a beépített szabályokon felül. A címkéi javasoltként vannak megjelölve.
settings-page-search-blurb = Keressen a képek között aszerint, hogy mit ábrázolnak, a hangok között pedig aszerint, hogyan szólnak, ne csak a nevük szerint, és találjon fajtájukban hasonló dolgokat, ne csak színükben hasonlókat.
settings-page-search-tab = Keresés
settings-page-adding = Csomagok hozzáadása
settings-page-adding-blurb = Mit csinál a Hoard egy csomaggal hozzáadáskor, a beolvasásán és felcímkézésén kívül.
settings-page-downloads = Letöltések
settings-page-downloads-blurb = Hogyan nevezi el a Hoard a fájlokat abban, amit küld: letöltésekben, zipekben, a hoard eszközben és az MI-asszisztenseknek. A könyvtár fájljai változatlanok maradnak.
settings-page-account = Fiók
settings-page-account-blurb = A jelszava, a többi bejelentkezni tudó személy, valamint hozzáférési tokenek szkriptekhez és más programokhoz.
settings-page-auth = Hitelesítés
settings-page-auth-blurb = Hogyan jelentkeznek be az emberek: a Hoard saját fiókjaival, LDAP-címtárral, Microsoft Active Directoryval vagy OAuth-szolgáltatóval; meddig tartanak a munkamenetek, és melyik csoport melyik szerepkört kapja.
settings-page-connection = Kapcsolat
settings-page-connection-blurb = Hogyan érik el a böngészők és az alkalmazások a Hoardot: a cím, amelyen ismert, és a HTTPS, amely egyes böngészőfunkciókhoz szükséges.
settings-page-performance = Teljesítmény
settings-page-performance-blurb = Mennyire dolgozik keményen a szerver, hogyan készülnek és tárolódnak a bélyegképek, és legfeljebb mennyit fogad be.
settings-page-performance-blurb-desktop = Mennyire dolgozik keményen a Hoard, hogyan készülnek és tárolódnak a bélyegképek, és legfeljebb mennyit fogad be.
settings-page-issues = Problémák
settings-page-issues-blurb = Mi ment félre a fájlok megnyitásakor vagy feldolgozásakor: ki nem csomagolható tömörített fájlok, be nem olvasható fájlok, sikertelen feladatok. Próbálja újra őket, ha az ok megszűnt, vagy vesse el őket.
settings-page-maintenance = Karbantartás
settings-page-maintenance-blurb = Biztonsági mentés, sérülésellenőrzés és rendrakás. Mindegyikből egyszerre egy fut, és elhagyhatja ezt az oldalt, amíg futnak.
settings-page-about = Névjegy
settings-page-about-blurb = Mi a Hoard, melyik verzió ez, és mások munkája, amelyből készült.
settings-the-default = alapértelmezett
settings-default-on = bekapcsolva
settings-default-off = kikapcsolva
settings-default-auto = automatikus
settings-option-saved = { $name }: { $value }
settings-back-to-default = Vissza az alapértelmezéshez
settings-save = Mentés
settings-use = Használat
settings-status-off = Ki

## SettingsPage.vue: Megjelenés

settings-themes = Témák
settings-look-light = Világos
settings-look-dark = Sötét
settings-look-group =
    { $scheme ->
        [light] Világos téma
       *[dark] Sötét téma
    }
settings-preview =
    A { $preview ->
        [light] világos
       *[dark] sötét
    } téma látható, hogy megnézhesse. Ön a { $used ->
        [light] világosat
       *[dark] sötétet
    } használja{ $system ->
        [yes] { " " }(a rendszert követve)
       *[other] { "" }
    }, és ez tér vissza, amikor elhagyja ezt az oldalt.
settings-preview-back =
    Vissza a { $used ->
        [light] világos
       *[dark] sötét
    } témához
settings-look-hint = A világos és a sötét mód (vagy a rendszer követése) között a felső sávban válthat.

## SettingsPage.vue: Eszközök

settings-blender-what = A .blend, .dae, .abc és .x3d modelleket a háttérben glTF-re alakítja, így bélyegképet és 3D-előnézetet kapnak. A kikapcsolt állapotában hozzáadott csomagok átalakítása megtörténik, amint elkészül.
settings-blender-download = Telepített Blendert használ, vagy letölti a Blender 4.5 LTS-t (kb. 400 MB) egy blender.org-tükörről.
settings-ffmpeg-what = MP3-, OGG- és Opus-letöltéseket kódol, és beolvassa azokat a hangokat, amelyeket a Hoard maga nem tud dekódolni (AAC, M4A, WMA, …).
settings-ffmpeg-download = A rendszer megfelelő kódolókkal rendelkező ffmpeg-jét használja, vagy letölt egy statikus buildet (kb. 80 MB).
settings-tool-reset = { $tool }: vissza az alapértelmezéshez
settings-tool-no-download = Telepített programot használ: { $tool }. Ehhez a géphez nincs letölthető build, ezért telepítse, vagy adja meg az elérési útját.
settings-tool-path = { $tool } – a program elérési útja
settings-mode-off = Ki
settings-mode-auto = Automatikus
settings-mode-path = Ez a program
settings-tool-looking = Keresés…
settings-tool-downloading = Letöltés… a folyamat az aktivitásjelzőn követhető
settings-tool-ready = Kész: { $detail }
settings-soundfont = MIDI-hangkészlet (SoundFont)
settings-soundfont-what = A hangszerek, amelyekkel a MIDI-fájlok lejátszása és átalakítása történik.
settings-soundfont-file = SoundFont-fájl
settings-soundfont-placeholder = A szerveren talált vagy a beépített
settings-soundfont-placeholder-desktop = Ezen a számítógépen talált vagy a beépített
settings-soundfont-default = a szerveren talált vagy a beépített
settings-soundfont-default-desktop = ezen a számítógépen talált vagy a beépített
settings-soundfont-send = Fájl kiválasztása…
settings-soundfont-sent = A MIDI mostantól ezzel szól: { $name }

## SettingsPage.vue: Képkeresés

settings-clip-heading = Keresés a képek tartalma szerint
settings-clip-reset = Képkeresés: vissza az alapértelmezéshez
settings-clip-saved-on = Képkeresés: bekapcsolva
settings-clip-saved-off = Képkeresés: kikapcsolva
settings-clip-what = Írja be, amit keres (angolul, pl. „mossy rock”, „sci-fi door”, „wooden barrel”), és az ezt ábrázoló képek megjelennek a szavakat tartalmazó nevek mellett. A „Több ilyen” ugyanolyan fajtájú dolgokat talál, nem csak ugyanolyan színűeket.
settings-clip-toggle = Képkeresés bekapcsolása
settings-clip-hint = A Google SigLIP 2 modelljét használja, a szerver processzorán futtatva: semmi sem hagyja el a szervert. Bekapcsoláskor letölti a modellt ({ $size } MB) a Hugging Face-ről, majd a háttérben, alacsony prioritással megtanul minden képet. Munka vagy keresés közben 350–750 MB memóriát foglal. Kikapcsoláskor törli a modellt, de megtartja, amit megtanult, a következő alkalomra.
settings-clip-hint-desktop = A Google SigLIP 2 modelljét használja, ennek a számítógépnek a processzorán futtatva: semmi sem hagyja el a gépet. Bekapcsoláskor letölti a modellt ({ $size } MB) a Hugging Face-ről, majd a háttérben, alacsony prioritással megtanul minden képet. Munka vagy keresés közben 350–750 MB memóriát foglal. Kikapcsoláskor törli a modellt, de megtartja, amit megtanult, a következő alkalomra.
settings-clip-looking = A modell keresése…
settings-clip-downloading = A modell letöltése… a folyamat az aktivitásjelzőn követhető
settings-clip-ready-empty = Kész. A képeket a csomagok hozzáadásakor tanulja meg.
settings-clip-ready-all =
    { $count ->
       *[other] Kész: mind a { $count } képet ismeri
    }
settings-clip-learning = Kész: képek tanulása, eddig { $done } / { $total }
settings-clip-relearning = Amíg az új modell meg nem tanul minden képet, a keresések azt használják, amit a régi tanult.
settings-clip-unavailable = Nem sikerült beszerezni a modellt: { $detail }. Óránként újrapróbálja.
settings-clip-where = Hol fut
settings-clip-here = Ezen a szerveren: semmi sem hagyja el
settings-clip-here-desktop = Ezen a számítógépen: semmi sem hagyja el
settings-clip-hosted = Egy hosztolt szolgáltatásnál
settings-clip-hosted-warning = A képek elhagyják ezt a szervert. A könyvtár minden képének bélyegképe és minden képkeresés szavai a(z) { $service } szolgáltatáshoz kerülnek, amely díjat számít fel értük, és a feltételei szerint őrzi meg őket.
settings-clip-hosted-warning-desktop = A képek elhagyják ezt a számítógépet. A könyvtár minden képének bélyegképe és minden képkeresés szavai a(z) { $service } szolgáltatáshoz kerülnek, amely díjat számít fel értük, és a feltételei szerint őrzi meg őket.
settings-clip-hosted-hint = Itt semmi sem töltődik le és nem fut: a szolgáltatás minden képet megtanul, kötegenként. Másik szolgáltatás vagy modell újra megtanulja mindet, és amit mindegyik megtanult, megmarad a visszaváltáshoz.
settings-clip-saved-here = Képkeresés: ezen a szerveren fut
settings-clip-saved-here-desktop = Képkeresés: ezen a számítógépen fut
settings-clip-saved-hosted = Képkeresés: a(z) { $service } szolgáltatásnál fut
settings-clip-hosted-unavailable = Nem sikerült használni a(z) { $service } szolgáltatást: { $detail }. Óránként újrapróbálja.
settings-clap-heading = Keresés a hangok hangzása szerint
settings-clap-reset = Hangkeresés: vissza az alapértelmezéshez
settings-clap-saved-on = Hangkeresés: bekapcsolva
settings-clap-saved-off = Hangkeresés: kikapcsolva
settings-clap-what = Írja be, amit keres (angolul, pl. „distant thunder”, „glassy UI blip”, „footsteps on gravel”), és az így szóló hangok megjelennek a szavakat tartalmazó nevek mellett.
settings-clap-toggle = Hangkeresés bekapcsolása
settings-clap-hint = A LAION CLAP modelljét használja, a szerver processzorán futtatva: semmi sem hagyja el a szervert. Bekapcsoláskor letölti a modellt ({ $size } MB) a Hugging Face-ről, majd a háttérben, alacsony prioritással meghallgatja minden hang elejét. Munka vagy keresés közben legfeljebb nagyjából 1 GB memóriát foglal. Kikapcsoláskor törli a modellt, de megtartja, amit megtanult, a következő alkalomra.
settings-clap-hint-desktop = A LAION CLAP modelljét használja, ennek a számítógépnek a processzorán futtatva: semmi sem hagyja el a gépet. Bekapcsoláskor letölti a modellt ({ $size } MB) a Hugging Face-ről, majd a háttérben, alacsony prioritással meghallgatja minden hang elejét. Munka vagy keresés közben legfeljebb nagyjából 1 GB memóriát foglal. Kikapcsoláskor törli a modellt, de megtartja, amit megtanult, a következő alkalomra.
settings-clap-ready-empty = Kész. A hangokat a csomagok hozzáadásakor tanulja meg.
settings-clap-ready-all =
    { $count ->
       *[other] Kész: mind a { $count } hangot ismeri
    }
settings-clap-learning = Kész: hangok tanulása, eddig { $done } / { $total }

## SettingsPage.vue: MI-címkézés

settings-ai-service = Szolgáltatás
settings-ai-off = Ki
settings-ai-off-hint = A csomagokat csak a beépített szabályok címkézik.
settings-ai-local = Helyi modell
settings-ai-local-hint = Az llm szolgáltatás a Hoard compose-fájljában (vagy cargo xtask llm). Töltsön le lent egy modellt; ezen a gépen fut, és semmi sem hagyja el.
settings-ai-chatgpt = ChatGPT-előfizetés
settings-ai-chatgpt-hint = A ChatGPT Go, Plus, Pro vagy Business előfizetését használja API-kulcs helyett.
settings-ai-openrouter-hint = Egy kulcs több száz hosztolt modellhez, közülük néhány ingyenes. A képeket fogadó modellek meg vannak jelölve.
settings-ai-openai-hint = Az OpenAI hosztolja.
settings-ai-anthropic-hint = Az Anthropic hosztolja, az OpenAI-kompatibilis végpontján keresztül.
settings-ai-gemini-hint = A Google hosztolja, az OpenAI-kompatibilis végpontján keresztül.
settings-ai-ollama-hint = Egy saját futtatású Ollama-szerver. Módosítsa a címet, ha nem egy ollama nevű konténer.
settings-ai-custom = Egyéb (OpenAI-kompatibilis)
settings-ai-custom-hint = Bármi, ami az OpenAI chat API-ját beszéli: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Kapcsolódás…
settings-ai-connected = Kapcsolódva · { $count } modell
settings-ai-connected-none = Kapcsolódva, de nem sorol fel modelleket; írja be a modell nevét
settings-chatgpt-connected-none = Kapcsolódva; írja be a modell nevét
settings-ai-takes-images = képeket fogad
settings-ai-saved = MI-címkézés: { $service }
settings-ai-saved-off = Az MI-címkézés ki van kapcsolva
settings-ai-reset = MI-címkézés: vissza az alapértelmezéshez
settings-ai-on = Be: { $service } · { $model }
settings-ai-url = Cím
settings-ai-key = API-kulcs
settings-ai-key-saved = Mentve (gépeljen a cseréhez)
settings-ai-key-paste = Illessze be a kulcsát
settings-ai-key-optional = Nincs (nem kötelező)
settings-ai-key-remove = Eltávolítás
settings-ai-get-key = Kulcs beszerzése
settings-ai-model = Modell
settings-ai-model-pick = Válasszon vagy írjon be egy modellt
settings-ai-model-name = Modell neve
settings-ai-refresh = Frissítés
settings-ai-connect = Kapcsolódás
settings-ai-need-key = Adja meg az API-kulcsot a modellek megjelenítéséhez
settings-ai-vision = Bélyegképek küldése is (képeket fogadó modell kell hozzá)
settings-ai-batch = Kérésenként
settings-ai-batch-unit = asset (kis helyi modellekhez kevesebb, nagy hosztoltakhoz több)

## SettingsPage.vue: ChatGPT-bejelentkezés

settings-chatgpt-account = Fiók
settings-chatgpt-checking = Ellenőrzés…
settings-chatgpt-status-as = Bejelentkezve: { $email }
settings-chatgpt-status = Bejelentkezve
settings-chatgpt-signed-in-as = Bejelentkezve a ChatGPT-be: { $email }
settings-chatgpt-signed-in = Bejelentkezve a ChatGPT-be
settings-chatgpt-signed-out = Kijelentkezve a ChatGPT-ből
settings-chatgpt-continue = Folytatás a ChatGPT-vel
settings-chatgpt-sign-out = Kijelentkezés
settings-chatgpt-sign-out-title = Kijelentkezik a ChatGPT-ből?
settings-chatgpt-sign-out-in-use = Az MI-címkézés a ChatGPT-előfizetését használja, ezért leáll, amíg újra be nem jelentkezik, vagy másik szolgáltatást nem választ.
settings-chatgpt-sign-out-body = A Hoard elfelejti a bejelentkezést, és kéri az OpenAI-t, hogy szüntesse meg.
settings-chatgpt-manage = Használat kezelése
settings-chatgpt-finish-before = Fejezze be a bejelentkezést a megnyílt ablakban (
settings-chatgpt-finish-link = nyissa meg újra
settings-chatgpt-finish-middle = ). Ha egy be nem töltődő oldalon ér véget, amelynek címe így kezdődik:{ " " }
settings-chatgpt-finish-after = , másolja ki ezt a címet, és illessze be ide:
settings-chatgpt-paste = A cím, amelyen a bejelentkezés véget ért
settings-chatgpt-finish = Befejezés
settings-chatgpt-usage = A címkézési kérések beleszámítanak a ChatGPT-előfizetése használatába. A Használat kezelése megmutatja, mennyit használt a Hoard, és ott korlátozhatja is. Ha magában a ChatGPT-ben szeretné leválasztani a Hoardot, a chatgpt.com oldalon lépjen ide: Settings → Security and login → Sign in with ChatGPT.

## SettingsPage.vue: Helyi modellek

settings-local-downloading-progress = Letöltés: { $percent }% · { $done } / { $total }
settings-local-downloading = Letöltés…
settings-local-incomplete = A letöltés megszakadt
settings-local-failed = Legutóbb nem sikerült elindítani
settings-local-in-use-running = Használatban · fut
settings-local-in-use = Használatban
settings-local-starting = Indítás…
settings-local-running = Fut
settings-local-ready-images = Kész · képeket fogad
settings-local-ready-text = Kész · csak szöveg
settings-local-delete-title = Törli ezt a modellt: { $name }?
settings-local-delete-in-use = Az MI-címkézés ezt a modellt használja, ezért leáll, amíg másikat nem választ. A fájljai törlődnek a modellszerverről.
settings-local-delete-downloading = A letöltés leáll, és az eddig letöltött rész törlődik.
settings-local-delete-body = A fájljai törlődnek a modellszerverről. Később újra letöltheti.
settings-local-delete = Törlés
settings-local-deleting = Törlés…
settings-local-cancel = Mégse
settings-local-resume = Folytatás
settings-local-download = Letöltés
settings-local-start-before = Indítsa el ezzel:{ " " }
settings-local-start-or = { " " }(fejlesztéskor ezzel:{ " " }
settings-local-start-developing = ), majd{ " " }
settings-local-start-retry = próbálja újra
settings-local-start-after = . Az egyetlen modellt futtató szerver is megfelel: adja meg lent a modellje nevét.
settings-local-list = A modellszerveren
settings-local-none = Még nincs letöltve semmi. Válasszon egyet lent; a szerveren marad, és bármikor válthat.
settings-local-catalog = Letölthető modellek
settings-local-size = { $size } GB · { $license }
settings-local-other = Másik modell a Hugging Face-ről
settings-local-other-placeholder = Másik a Hugging Face-ről: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Csomagok hozzáadása

settings-store-pages = Áruházi oldalak
settings-store-pages-what = Az új csomagokat a háttérben megkeresi az áruházakban, így megkapják az áruházi oldalukat, az eladójukat és a licencüket.
settings-store-lookup = Áruházi oldalak automatikus keresése
settings-unreal = Unreal-csomagfájlok
settings-unreal-reset = Unreal-csomagfájlok: vissza az alapértelmezéshez
settings-unreal-what = Mi történjen a .uasset fájlokkal.
settings-conv = Átalakítási szabályok
settings-conv-what = A Hoard által tárolt fájlok átalakítása egyetlen formátumra, például minden OBJ és FBX fájlé GLB-re, hogy a könyvtár egységes legyen. Az átalakított fájl átveszi az eredeti helyét, megtartva a nevét, címkéit, gyűjteményeit és hivatkozásait. A figyelt mappákban lévő fájlok soha nem változnak.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Bekapcsolva
settings-conv-to = Célformátum
# Heading over the file formats a rule converts.
settings-conv-from = Forrásformátumok
settings-conv-remove = Eltávolítás
settings-conv-remove-label = { $n }. szabály eltávolítása
settings-conv-taken = Ezt a formátumot egy másik szabály alakítja át
settings-conv-upright = A modellek felállítása átalakításkor (Y felfelé, előre +Z irányba), ahogy a csomag modelljei megjelennek
settings-conv-flac-what = A hang pontosan ugyanaz marad, nagyjából feleakkora helyen, és a WAV minden más része (hurokpontok, cue-jelölők, címkék) megmarad a FLAC-ban, így a WAV bármikor pontosan az eredeti formájában visszaállítható. A lebegőpontos vagy 32 bites mintákat tartalmazó WAV-ok WAV-ok maradnak.
settings-conv-add = Szabály hozzáadása
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Az eredetiek megőrzése
settings-conv-keep-days =
    { $count ->
        [one] napig
       *[other] napig
    }
settings-conv-keep-what = Addig az átalakított fájl oldaláról visszaállítható az eredeti. A textúrák nem másolódnak bele az átalakított modellbe: az a mellette lévő képfájlokra hivatkozik. A modell .mtl vagy .bin fájljai megmaradnak.
settings-conv-save = Szabályok mentése
settings-conv-revert = Módosítások visszavonása
settings-conv-existing = A könyvtárban már meglévő fájlok átalakítása
settings-conv-saved = Átalakítási szabályok mentve. A mostantól hozzáadott csomagokra vonatkoznak.
settings-conv-converting =
    { $count ->
        [one] Egy fájl átalakítása a háttérben
       *[other] { $count } fájl átalakítása a háttérben
    }
settings-conv-nothing = A könyvtárban semmit sem kell átalakítani.
settings-conv-ask-title =
    { $count ->
        [one] Átalakít egy fájlt, amely már a könyvtárban van?
       *[other] Átalakít { $count } fájlt, amely már a könyvtárban van?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Egy csomagban van.
       *[other] { $packs } csomagban vannak.
    } Minden eredeti { $days } napig megmarad, és addig az oldaláról visszaállítható.
settings-conv-ask-busy =
    { $count ->
        [one] Egy még hozzáadás alatt álló csomagban van; akkor alakul át, amikor ez befejeződik.
       *[other] { $count } még hozzáadás alatt álló csomagokban van; akkor alakulnak át, amikor ez befejeződik.
    }
settings-conv-ask-list = Az átalakítandó fájlok
settings-conv-ask-more = …és még { $count }
settings-conv-ask-skipped = Változatlanul maradnak:
settings-conv-skip-watched =
    { $count ->
        [one] egy fájl figyelt mappában
       *[other] { $count } fájl figyelt mappákban
    }
settings-conv-skip-unity =
    { $count ->
        [one] egy fájl, amelyet egy Unity-projekt a nevéről ismer
       *[other] { $count } fájl, amelyet egy Unity-projekt a nevéről ismer
    }
settings-conv-skip-used =
    { $count ->
        [one] egy fájl, amelyet egy jelenet vagy egy másik fájl megnevez
       *[other] { $count } fájl, amelyet egy jelenet vagy egy másik fájl megnevez
    }
settings-conv-skip-beside =
    { $count ->
        [one] egy fájl, amelynek átalakított másolata már a csomagjában van
       *[other] { $count } fájl, amelynek átalakított másolata már a csomagjában van
    }
settings-conv-skip-blender =
    { $count ->
        [one] egy fájl, amelyet a Blender még nem olvasott be (Beállítások → Eszközök)
       *[other] { $count } fájl, amelyet a Blender még nem olvasott be (Beállítások → Eszközök)
    }
settings-conv-ask-new = Mentés, csak az új csomagokra
settings-conv-ask-save-convert =
    { $count ->
        [one] Mentés és átalakítás
       *[other] Mentés és { $count } fájl átalakítása
    }
settings-conv-ask-convert =
    { $count ->
        [one] Átalakítás
       *[other] { $count } fájl átalakítása
    }
settings-unreal-off = Maradjanak, ahogy vannak
settings-unreal-off-hint = Átalakítás a csomag oldaláról, amikor szeretné.
settings-unreal-keep = Átalakítás, az eredetiek megtartásával
settings-unreal-keep-hint = A textúrák, hangok és anyagok általános formátumokba kerülnek; a .uasset is megmarad.
settings-unreal-replace = Átalakítás, az eredetiek elvetésével
settings-unreal-replace-hint = Helyet takarít meg, miután a tartalom kikerült. A hálók továbbra is az Unreal workerre várnak.

## SettingsPage.vue: Letöltések

settings-tn = Textúranevek
settings-tn-saved = Textúranevek mentve
settings-tn-reset = Textúranevek: vissza az alapértelmezéshez
settings-tn-what = Egy anyag térképei az anyag nevével és a térkép szerepével kerülnek ki, bárhogy is nevezik a fájlokat: Rock_Normal.png, nem rock_moss_xeokah0_raw_8K_Normal.png. A könyvtár fájljai érintetlenek maradnak, és az a térkép, amelyet egy vele küldött modell vagy MTL megnevez, megtartja a nevét, hogy a modell továbbra is megtalálja.
settings-tn-toggle = Textúratérképek elnevezése az anyaguk szerint
settings-tn-style = Stílus
settings-tn-style-group = Elnevezési stílus
settings-tn-descriptive = Leíró
settings-tn-short = Rövid
settings-tn-custom = Saját
settings-tn-pattern = Minta
settings-tn-hint-name = { " " }az anyag,{ " " }
settings-tn-hint-map = { " " }a térkép szerepe,{ " " }
settings-tn-hint-size = { " " }a mérete, ha a készlet több méretben érkezik (a végére kerül, ha a minta kihagyja).
settings-tn-example = Példa
settings-tn-suffixes = Utótagok térképtípusonként
settings-tn-slots = A GLB-letöltésekben a modellbe csomagolt, saját fájl nélküli textúra az anyagról és az általa kitöltött helyről kapja a nevét, és ezeknek a helyeknek is van utótagja:
settings-tn-endings = Saját végződések
settings-tn-endings-what = Az a textúra, amely nem egy anyag térképe, megtartja a saját nevét, hacsak nem ezek egyikére végződik: a _diff → _BC a rock_diff.png fájlt rock_BC.png néven küldi. Ha a másodikat üresen hagyja, a végződés elmarad.
settings-tn-ending-from = Végződés
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Csere erre
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Végződés hozzáadása
settings-tn-ending-remove = Eltávolítás
settings-tn-ending-remove-label = Végződés eltávolítása: { $ending }

## SettingsPage.vue: Általános és Kapcsolat

settings-name = Név
settings-name-what = A szerver neve a felső sávban, a böngészőlapon és a bejelentkezési oldalon, hogy több hoard esetén meg lehessen különböztetni őket.
settings-name-what-desktop = A könyvtár neve a felső sávban.
settings-address = Cím
settings-address-what = A cím, amelyen az emberek és az alkalmazások elérik a Hoardot, ha nem az, amelyen ez a böngésző van: egy fordított proxyé, egy alagúté vagy egy tailnet-név, például https://hoard.example.com. A Hoard által kiadott hivatkozások (a Claude-nak és más alkalmazásoknak, a bejelentkezésben, a tanúsítványán) ezt használják.
settings-public-url = Nyilvános cím
settings-public-url-placeholder = A böngésző címéből (pl. https://hoard.example.com)
settings-public-url-default = a böngésző címéből

## ProfilerPanel.vue: a profilozó a Beállítások → Teljesítmény tetején

profiler = Profilozó
profiler-what = Mit csinált a Hoard az elmúlt tíz percben, kétmásodpercenként frissítve, amíg nézi. Csak a memóriában tárolódik, és a Hoard újraindulásakor elölről kezdődik.
profiler-starting = Az első mérések egy pillanat múlva érkeznek.
profiler-failed = Nem sikerült beolvasni a profilozót: { $error }
profiler-cpu = Processzorhasználat
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Az egész gép
profiler-cpu-summary = A Hoard processzorhasználata { $hoard }, az egész gépé { $machine }.
profiler-ram = RAM
profiler-ram-peak = Csúcs
profiler-ram-of = { $used } / { $total }
profiler-ram-summary = A Hoard { $hoard } RAM-ot használ, az elmúlt tíz percben legfeljebb { $peak }-ot.
profiler-serving = Folyamatban lévő kérések
profiler-working = Folyamatban
profiler-working-none = A háttérben semmi sem futott.
profiler-working-more =
    { $count ->
        [one] És még egy.
       *[other] És még { $count }.
    }
profiler-requests = Kérések kiszolgálásával töltött idő
profiler-requests-what = Minden oszlop tíz másodpercnyi kérés, az idejük aszerint felosztva, hogy mire ment el.
profiler-part-db = Adatbázis
profiler-part-db-title = Várakozás az adatbázisra és a lekérdezései futtatása
profiler-part-disk = Lemez
profiler-part-disk-title = A tárolt fájlok olvasása és az elküldöttek tömörítése
profiler-part-send = Küldés
profiler-part-send-title = Várakozás, amíg a hálózat átveszi az elküldötteket
profiler-part-other = Egyéb
profiler-part-other-title = A válasz kidolgozása
profiler-request-count =
    { $count ->
        [one] { $count } kérés
       *[other] { $count } kérés
    }
profiler-requests-none = Nem volt kérés az elmúlt tíz percben.
profiler-slowest = Leglassabb: { $route }, { $time }
profiler-routes = Útvonalanként
profiler-route = Útvonal
profiler-count = Kérések
profiler-average = Átlag
profiler-p95 = Leglassabb 5%
profiler-split = Mire ment el az idő
profiler-sent = Elküldve
profiler-portal = a felület saját fájljai

## SettingsPage.vue: Teljesítmény

settings-render = 3D-bélyegképek
settings-render-toggle = Bélyegképek renderelése a 3D-modellekhez
settings-gpu-on = A GPU-n: { $renderer }
settings-gpu-software = Nincs GPU, ezért szoftveresen ({ $renderer })
settings-gpu-none = Nem található sem GPU, sem szoftveres renderelő, így a modellek nem kapnak bélyegképet
settings-thumbs = Bélyegképek
settings-thumbs-what = Új bélyegképekhez és modellnézetekhez. Az átlátszók és a pixelgrafikák PNG-k maradnak. A Karbantartás Bélyegképek zsugorítása feladata újrakészíti a meglévőket.
settings-previews-size = Előnézetek: { $size }
settings-thumbs-saved = Bélyegképek: { $quality }
settings-thumbs-format = Formátum
settings-thumbs-quality = Minőség: { $quality }.
settings-thumbs-small = Kisebb
settings-thumbs-small-hint = A rácsban megfelel; közelről kissé lágy.
settings-thumbs-balanced = Kiegyensúlyozott
settings-thumbs-balanced-hint = Bélyegképméretben alig különböztethető meg az eredetitől.
settings-thumbs-sharp = Élesebb
settings-thumbs-sharp-hint = Nagyjából kétszer akkora, mint a Kiegyensúlyozott.
settings-thumbs-lossless = Veszteségmentes
settings-thumbs-lossless-hint = PNG, pontosan úgy, ahogy elkészült. Többszörösen nagyobb. Soha nem mennek ki AVIF-ként.
settings-thumbs-avif-what = Az AVIF gyakran feleakkora. Minden bélyegkép az első megjelenítéskor alakul át.
settings-thumbs-avif-now = Összes átalakítása most
settings-thumbs-avif-converting = Átalakítás…
settings-motion-thumbs = Mozgó bélyegképek az animációkhoz
settings-motion-thumbs-what = Minden animáció kap egy rövid, ismétlődő mozgóképet is, amely akkor játszódik, amikor a csempéjére mutatsz. Darabonként kb. 40 KB-ot foglalnak, és nagy könyvtárnál eltart egy ideig az elkészítésük. Kikapcsoláskor törlődnek.
settings-limits = Munka és korlátok
settings-limits-what = Mindegyik mentődik, amikor elhagyja a mezőt vagy Entert nyom. A méretekhez mértékegység kell: 100 GB, 512 MB, 1,5 TB; a puszta szám GB-ot jelent.
settings-workers = Háttérfeldolgozók
settings-workers-unit = feladat egyszerre (beolvasás, címkézés, átalakítás)
settings-max-upload = Legnagyobb feltöltés
settings-max-upload-unit = fájlonként
settings-max-expand = Legnagyobb csomag
settings-max-expand-unit = kicsomagolva
settings-conversion-cache = Átalakított letöltések
settings-conversion-cache-unit-now = legfeljebb ennyi marad meg, jelenleg { $size }; a legrégebben használtak törlődnek először
settings-conversion-cache-unit = legfeljebb ennyi marad meg; a legrégebben használtak törlődnek először

## SettingsPage.vue: Karbantartás

settings-updates = Frissítések
settings-version = Ez a Hoard { $version }.
settings-update-check = Új kiadás keresése a GitHubon néhány óránként
settings-desktop-updates = A Hoard minden indításkor új kiadást keres, a háttérben letölti, és kilépéskor telepíti.
settings-backups = Ütemezett biztonsági mentések
settings-backups-what = Az adatbázis magától készít biztonsági mentést; mindegyik megjelenik lent az előzményekben.
settings-backup-every = Mentés
settings-backup-off = Soha
settings-backup-daily = Minden nap
settings-backup-weekly = Minden héten
settings-backup-keep = Megtartás
settings-backup-keep-unit = legújabb ütemezett mentés
settings-backup-copy-to = Másolás ide is
settings-backup-copy-to-none = Nincs másik mappa
settings-backups-kept = Mindegyik hoard-auto-…db néven a mentések mappájába kerül, és ha van, a másik mappába is (másik lemez, vagy egy mappa, amit egy szinkronizáló alkalmazás ment), a szerveren lévő teljes útvonalával megadva. Csak a régebbi ütemezett mentések törlődnek; a kézzel készítettek maradnak.
about-what = A Hoard egyetlen könyvtárban tárolja a játékokhoz való asseteket (modelleket, textúrákat, hangokat, zenét, sprite-okat és egyebeket), amelyben kereshet, előnézetet nézhet és amelyből vehet, megosztva mindenkivel, akinek hozzáférést ad.
about-version = Verzió
about-edition = Kiadás
about-edition-server = Szerver
about-edition-desktop = Asztali alkalmazás
about-edition-steam = Asztali alkalmazás a Steamről
about-made-by = Készítette
about-source = Forráskód a GitHubon
about-issues = Hiba bejelentése
about-discord = Beszélgess velünk a Discordon
about-steam = A Hoard a Steamen
about-license = Licenc
about-license-what = A Hoard nyílt forráskódú, MIT No Attribution licenccel: használhatja, módosíthatja és megoszthatja, ahogy csak szeretné.
about-downloaded = Szükség esetén letöltve
about-downloaded-what = Ezek nem részei a Hoardnak. Mindegyiket akkor tölti le, amikor először szükség van rá, és mindegyikre a saját licence vonatkozik.
about-blender = 3D-modelleket alakít át és bélyegképeket készít róluk.
about-ffmpeg = Hangot és videót alakít át és előnézetet készít róluk.
about-siglip = Képeket keres aszerint, hogy mi van rajtuk.
about-clap = Hangokat keres aszerint, hogy milyennek hangzanak.
about-models = A helyi MI-szerverhez választott modellekre készítőik licence vonatkozik.
about-built-with = Felhasznált összetevők
about-built-with-what = A nyílt forráskódú könyvtárak, amelyekből a Hoard készült, a licenceikkel.
about-steamworks = A Steames kiadás a Steamworks SDK-t is tartalmazza, © Valve Corporation.
about-licenses-loading = A lista betöltése…
about-licenses-failed = A listát nem sikerült betölteni.
about-filter = Szűrés név vagy licenc szerint
about-count = { $shown } / { $total }
about-no-license = Nincs megadott licenc
about-report = Probléma jelentése…
report-title = Probléma jelentése
report-what = Ennyi kell egy GitHub-issue-hoz, hogy a problémát ki lehessen vizsgálni. Semmi személyes nincs benne: a hozzáférési tokeneket, jelszavakat, kulcsokat, e-mail-címeket és a nevedet a mappaútvonalakban eltávolítjuk. Küldés előtt olvasd át.
report-loading = A jelentés összeállítása…
report-failed = A jelentést nem sikerült összeállítani: { $error }
report-log = Amit a szerver az utóbbi időben naplózott
report-copy = Másolás
report-copied = A jelentés kimásolva.
report-github = Issue nyitása a GitHubon
report-github-hint = Az új issue ezzel lesz kitöltve. Ha a napló ehhez túl hosszú, csak az utolsó sorai kerülnek bele; a Másolás mindent tartalmaz.
report-close = Bezárás

## Assettípusok (api.ts)

api-kind-texture = Textúrák
api-kind-vector = Vektorgrafikák
api-kind-sheet = Spritelapok
api-kind-audio = Hang
api-kind-model = Modellek
api-kind-animation = Animációk
api-kind-font = Betűtípusok
api-kind-shader = Shaderek
api-kind-material = Anyagok
api-kind-video = Videó
api-kind-document = Dokumentumok
api-kind-data = Adatok
api-kind-other = Egyéb
api-kind-texture-name = textúrák
api-kind-vector-name = vektorgrafikák
api-kind-sheet-name = spritelapok
api-kind-audio-name = hangok
api-kind-model-name = modellek
api-kind-animation-name = animációk
api-kind-font-name = betűtípusok
api-kind-shader-name = shaderek
api-kind-material-name = anyagok
api-kind-video-name = videók
api-kind-document-name = dokumentumok
api-kind-data-name = adatok
api-kind-other-name = egyebek

## Értesítések, párbeszédablakok és betöltés (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Visszavonás
ui-today = ma
ui-yesterday = tegnap
ui-days-ago = { $days } napja
ui-notifications = Értesítések
ui-dismiss = Elvetés
ui-cancel = Mégse
ui-lazy-loading = Betöltés: { $what }…
ui-lazy-failed = Nem sikerült betölteni: { $what }.
ui-lazy-reload = Az oldal újratöltése

## ResetButton.vue

reset-title = Visszaállítás az alapértékre: { $value }
reset-label = Visszaállítás az alapértékre, { $value }

## SizeInput.vue

size-invalid = Egy méret, például 100 GB, 512 MB vagy 1,5 TB
size-hint = Egy méret, például 100 GB, 512 MB vagy 1,5 TB; a puszta szám GB-ot jelent

## AuthPanel.vue (Beállítások → Hitelesítés)

auth-load-failed = Nem sikerült betölteni a hitelesítési beállításokat: { $error }
auth-saved = Mentve. A következő bejelentkezéstől érvényes.
auth-method-internal = Belső
auth-method-internal-hint = A Hoard által tárolt nevek és jelszavak, amelyeket a Beállítások → Fiók oldalon lehet felvenni.
auth-method-ldap-hint = Egy címtár ellenőrzi a bejelentkezési oldalon beírt nevet és jelszót: OpenLDAP, FreeIPA, LLDAP vagy más LDAP-szerver.
auth-method-ad-hint = A Windows-tartományi fiókok a szokásos nevükkel és jelszavukkal jelentkeznek be.
auth-method-oauth-hint = Egy „Bejelentkezés ezzel…” gomb, amely a Google-hoz, a Microsofthoz, a GitHubhoz, a Keycloakhoz, az Authentikhez vagy más szolgáltatóhoz visz, majd vissza.
auth-session-unused = 30 nap használaton kívül után
auth-session-1h = 1 órával a bejelentkezés után
auth-session-8h = 8 órával a bejelentkezés után
auth-session-day = Egy nappal a bejelentkezés után
auth-session-week = Egy héttel a bejelentkezés után
auth-session-30d = 30 nappal a bejelentkezés után
auth-session-hours = { $hours } órával a bejelentkezés után
auth-role-view = Megtekintő
auth-role-edit = Szerkesztő
auth-role-admin = Admin
auth-role-word-view = megtekintő
auth-role-word-edit = szerkesztő
auth-role-word-admin = admin
auth-role-word-default = { $role } (az alapértelmezett szerepkör)
auth-tls-starttls-hint = Sima LDAP (389-es port), amely bármi elküldése előtt TLS-re vált.
auth-tls-ldaps-hint = TLS az elejétől (636-os port).
auth-tls-none = Nincs
auth-tls-none-hint = A jelszavak titkosítás nélkül mennek át a hálózaton. Csak megbízható hálózaton.
auth-test-failed = Nem sikerült tesztelni: { $error }
auth-preset-custom = Egyéb (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel és mások: adja meg a kiállítót, és a Felderítés kitölti a címeket.
auth-preset-google-note = Hozzon létre egy OAuth-klienst (Web application) a Google Cloud Console → APIs & Services → Credentials alatt. A Google nem küld csoportokat, ezért a szerepköröket a Beállítások → Fiók oldalon adja meg, és kapcsolja ki a fiókok első bejelentkezéskor történő létrehozását, hacsak nem juthat be bárki, akinek Google-fiókja van.
auth-preset-microsoft-note = Regisztráljon egy alkalmazást az Entra ID → App registrations alatt (Web átirányítással az alábbi címre). Az „organizations” helyére írja a bérlőazonosítóját, hogy csak a saját szervezete jusson be. A csoportokból származó szerepkörökhöz adja hozzá a groups claimet a Token configuration alatt: ez csoportazonosítókat sorol fel.
auth-preset-github-note = Hozzon létre egy OAuth-alkalmazást a GitHub → Settings → Developer settings alatt. A GitHub nem küld csoportokat, ezért a szerepköröket a Beállítások → Fiók oldalon adja meg, és kapcsolja ki a fiókok első bejelentkezéskor történő létrehozását, hacsak nem juthat be bárki, akinek GitHub-fiókja van.
auth-preset-gitlab-note = Hozzon létre egy alkalmazást a GitLab → Preferences → Applications alatt (vagy a saját példánya adminfelületén: a címekben cserélje ki a gitlab.com részt).
auth-discovered = A szolgáltató címei kitöltve.
auth-copied = Másolva
auth-copy-failed = Nem sikerült másolni: jelölje ki, és másolja ki saját maga
auth-sign-in-with = Bejelentkezés ezzel
auth-own-accounts-still-work = A Hoard saját fiókjai továbbra is a saját jelszavukkal jelentkeznek be, így egy elérhetetlen vagy rosszul beállított címtár vagy szolgáltató nem zárhatja ki Önt.
auth-sessions = Munkamenetek
auth-sign-out-after = Kijelentkeztetés
auth-ldap-server = LDAP-szerver
auth-server-type = Szervertípus
auth-server-type-custom = Egyéni
auth-servers = Szerverek
auth-server-n = { $n }. szerver
auth-remove-server-n = { $n }. szerver eltávolítása
auth-remove = Eltávolítás
auth-add-server = Szerver hozzáadása
auth-servers-hint = Ha az egyik nem érhető el, a következővel próbálkozik.
auth-security = Biztonság
auth-connection-security = Kapcsolat biztonsága
auth-skip-verify = Ne ellenőrizze a szerver tanúsítványát (csak kipróbáláshoz)
auth-ca = Hitelesítésszolgáltató
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (üresen: amelyekben a rendszer megbízik)
auth-lookup = Személyek keresése
auth-anonymously = Névtelenül
auth-reader-account = Olvasófiók
auth-reader-password = A jelszava
auth-saved-type-to-replace = Mentve: gépeljen a cseréhez
auth-people-under = A személyek helye
auth-name-attribute = Névattribútum
auth-only-matching = Csak az ennek megfelelő bejegyzések
auth-groups-under = A csoportok helye
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (üresen: csak memberOf)
auth-their-groups = A csoportjaik
auth-group-filter-hint = { "{" }dn{ "}" } a személy bejegyzése, { "{" }user{ "}" } a név, amellyel bejelentkezett.
auth-test = Teszt
auth-test-hint = Elér minden szervert, és bejelentkezik az olvasófiókkal. Névvel és jelszóval az adott személyként is bejelentkezik, és megmutatja, mit kezdene vele a Hoard. Semmi sem mentődik.
auth-test-user = Akiként bejelentkezik
auth-test-user-placeholder = Akiként bejelentkezik (nem kötelező)
auth-test-password = A jelszava
auth-testing = Tesztelés…
auth-test-connectivity = Kapcsolat tesztelése
auth-test-server-ok = elérhető, és az olvasófiók bejelentkezett
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }bejelentkezett (
auth-test-signed-in-after = ).
auth-test-role = Szerepköre ez lenne: { $role }.
auth-test-no-role = Egyik csoportban sincs benne a Szerepkörök alatt felsoroltak közül, így elutasítanák.
auth-test-groups = Csoportok: { $groups }
auth-test-no-groups = Nem található csoport.
auth-provider = Szolgáltató
auth-redirect = Átirányítási cím
auth-copy = Másolás
auth-redirect-hint = Ezt adja meg a szolgáltatónak, amikor létrehozza a Hoard kliensét. A Beállítások → Kapcsolat oldalon megadott nyilvános címből származik, vagy abból a címből, amelyet éppen használ.
auth-issuer = Kiállító
auth-discover = Felderítés
auth-client-id = Kliensazonosító
auth-client-secret = Klienstitok
auth-authorization-url = Engedélyezési cím
auth-token-url = Tokencím
auth-userinfo-url = Felhasználóiadat-cím
auth-userinfo-placeholder = (üresen: az azonosítótoken mondja meg, ki az)
auth-scopes = Hatókörök
auth-name-from = Név forrása
auth-groups-from = Csoportok forrása
auth-groups-from-placeholder = (nincs)
auth-button = Gomb
auth-button-placeholder = saját fiók
auth-accounts = Fiókok
auth-provision = Hoard-fiók létrehozása, amikor valaki először bejelentkezik
auth-provision-on-hint = Ellenkező esetben egy admin előbb minden személyt felvesz a Beállítások → Fiók oldalon.
auth-provision-off-hint = Egy admin előbb minden személyt felvesz a Beállítások → Fiók oldalon, és kiválasztja, hogyan jelentkezik be.
auth-open-to-anyone = Bárki bejuthat, akinek fiókja van itt: { $provider }. Adjon meg lent megtekintői csoportokat, vagy kapcsolja ki ezt, és vegye fel saját maga a személyeket.
auth-new-accounts = Új fiókok
auth-roles-from-groups = Szerepkörök csoportokból
auth-roles-hint = Azok a csoportok, amelyek tagjai az egyes szerepköröket kapják, soronként egy, név vagy DN szerint. A legmagasabb egyező nyer, és ez minden bejelentkezéskor ellenőrződik. Ha nincs megadva egy sem, az új fiókok a fenti szerepkört kapják, és egy admin később módosítja. Ha a Megtekintő alatt csoportok vannak megadva, csak a felsorolt csoportok tagjai jelentkezhetnek be.
auth-roles-hint-oauth = Azok a csoportok, amelyek tagjai az egyes szerepköröket kapják, soronként egy, név vagy DN szerint (vagy ahogy a groups claim felsorolja őket). A legmagasabb egyező nyer, és ez minden bejelentkezéskor ellenőrződik. Ha nincs megadva egy sem, az új fiókok a fenti szerepkört kapják, és egy admin később módosítja. Ha a Megtekintő alatt csoportok vannak megadva, csak a felsorolt csoportok tagjai jelentkezhetnek be.
auth-saving = Mentés…
auth-save = Mentés

## HttpsPanel.vue (Beállítások → Kapcsolat)

https-load-failed = Nem sikerült betölteni a HTTPS-beállításokat: { $error }
https-off-redirecting = A HTTPS ki van kapcsolva. Átirányítás ide: http://
https-now-on = A HTTPS be van kapcsolva
https-saved = Mentve. Az új kapcsolatok az új tanúsítványt használják.
https-now-off = A HTTPS ki van kapcsolva
https-http3-failed-toast = A HTTP/3 nem tudott elindulni: { $error }
https-copied = Másolva
https-copy-failed = Nem sikerült másolni: jelölje ki, és másolja ki saját maga
https-mode-off = Ki
https-mode-off-hint = Csak sima http://.
https-mode-own = A Hoard saját tanúsítványa
https-mode-own-hint = A Hoard készít egyet a szerver címeihez. Telepítse egyszer a Hoard hitelesítésszolgáltatóját minden eszközre, és a böngészői megbíznak benne.
https-mode-custom = Saját tanúsítvány
https-mode-custom-hint = Egy meglévő, például a tailscale cert vagy a Let’s Encrypt által kiállított. A böngészők semmi telepítése nélkül megbíznak benne.
https-untrusted-own = Ez a böngésző még nem bízik meg benne ehhez a címhez: { $host }. Telepítse lent a Hoard hitelesítésszolgáltatóját. Egy figyelmeztetést fog látni, amelyen továbbkattinthat.
https-untrusted-uncovered = Ez a böngésző még nem bízik meg benne ehhez a címhez: { $host }. A tanúsítvány nem terjed ki erre a címre. Egy figyelmeztetést fog látni, amelyen továbbkattinthat.
https-untrusted = Ez a böngésző még nem bízik meg benne ehhez a címhez: { $host }. Egy figyelmeztetést fog látni, amelyen továbbkattinthat.
https-now = Jelenleg
https-start-failed = A HTTPS be van állítva, de nem tudott elindulni: { $error }
https-on-port = A HTTPS be van kapcsolva, ugyanazon a porton, mint a http:// ({ $port }).
https-using-it = Most is ezt használja.
https-open = Megnyitás: { $url }
https-trusted = Ez a böngésző megbízik benne.
https-cert-for-before = Tanúsítvány ezekhez:{ " " }
https-cert-for-after = , érvényes eddig: { $date }.
https-cert-for-after-own = , érvényes eddig: { $date } (magától megújul).
https-not-covered-before = Nem terjed ki erre:{ " " }
https-not-covered-after = , pedig ezt a címet használja.
https-is-off = A HTTPS ki van kapcsolva: az oldalak sima http://-n keresztül érkeznek.
https-addresses = Címek
https-addresses-hint = A tanúsítvány kiterjed a Hoard által talált címekre és az Ön által hozzáadottakra (a routere által adott név, egy Tailscale-név).
https-remove-name = Eltávolítás: { $name }
https-another-address = Másik cím
https-another-address-placeholder = Másik cím, pl. nas.local
https-add = Hozzáadás
https-your-cert = Az Ön tanúsítványa
https-saved-cert-before = Mentve:{ " " }
https-saved-cert-after = { " " }– kiállító: { $issuer }, érvényes eddig: { $date }. A cseréhez illesszen be egy újat.
https-pem-before = A tanúsítvány (a láncával) és a privát kulcsa PEM-szövegként: azok a fájlok, amelyek így kezdődnek:{ " " }
https-pem-after = .
https-certificate = Tanúsítvány
https-choose-file = Fájl kiválasztása…
https-private-key = Privát kulcs
https-checked-before = Ezekhez:{ " " }
https-checked-after = , kiállító: { $issuer }, érvényes eddig: { $date }.
https-checked-after-days = , kiállító: { $issuer }, érvényes eddig: { $date } ({ $days } nap).
https-self-signed = Önaláírt: a böngészők figyelmeztetni fognak miatta.
https-key-matches = A kulcs egyezik.
https-where-to-get = Honnan szerezhető
https-get-tailscale-before = kapcsolja be a HTTPS-t a tailnetjén, majd futtassa ezt:{ " " }
https-get-tailscale-after = { " " }a szerveren, és töltse fel a két fájlt. 90 napig érvényesek: előtte futtassa újra, és töltse fel őket ismét.
https-get-acme-title = Let’s Encrypt vagy más ACME-kliens
https-get-acme-before = (certbot, acme.sh, lego) egy saját domainhez: töltse fel ezeket:{ " " }
https-get-acme-and = { " " }és{ " " }
https-get-acme-after = .
https-get-proxy-title = Fordított proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), amelyen a tanúsítvány van: hagyja itt kikapcsolva a HTTPS-t, és a proxy sima http://-n továbbítson a Hoardnak.
https-trying = Kipróbálás…
https-apply = Alkalmaz
https-apply-hint = A Hoard először kipróbálja az új beállítást, és ha nem működik, megtartja a meglévőt.
https-trust-title = A Hoard megbízhatóvá tétele az eszközein
https-download-ca = A hoard-ca.crt letöltése
https-trust-hint = Telepítse egyszer a Hoard hitelesítésszolgáltatóját minden eszközre, és a böngészői megbíznak ebben a szerverben (és minden tanúsítványban, amelyet a Hoard később készít hozzá). Csak ezért a Hoardért tud kezeskedni. Ujjlenyomat (SHA-256):
https-ca-windows = Nyissa meg a fájlt → Tanúsítvány telepítése → Aktuális felhasználó → Minden tanúsítvány tárolása ebben a tárolóban → Tallózás → Megbízható legfelső szintű hitelesítésszolgáltatók → Befejezés. Indítsa újra a böngészőt.
https-ca-macos = Nyissa meg a fájlt; a Kulcskarikába kerül. Ott kattintson duplán erre: „{ $subject }” → Megbízhatóság → A tanúsítvány használatakor: Mindig megbízható.
https-ca-ios-title = iPhone és iPad
https-ca-ios = Töltse le Safariban, és engedélyezze. Beállítások → Profil letöltve → Telepítés. Ezután Beállítások → Általános → Névjegy → Tanúsítványmegbízhatósági beállítások → kapcsolja be ezt: „{ $subject }”.
https-ca-android = Beállítások → Biztonság → További biztonsági beállítások → Titkosítás és hitelesítési adatok → Tanúsítvány telepítése → CA-tanúsítvány, és válassza ki a fájlt. (A nevek telefononként kissé eltérnek; keressen rá a Beállításokban arra, hogy „tanúsítvány”.)
https-ca-linux-1 = Debian és Ubuntu:{ " " }
https-ca-linux-2 = . Fedora és Arch:{ " " }
https-ca-linux-3 = . A linuxos Chrome saját listát vezet: Beállítások → Adatvédelem és biztonság → Biztonság → Tanúsítványok kezelése → Importálás.
https-ca-firefox = A Firefox saját listát vezet: Beállítások → Adatvédelem és biztonság → Tanúsítványok → Tanúsítványok megtekintése → Hitelesítésszolgáltatók → Importálás, és jelölje be a weboldalak azonosítására vonatkozó megbízhatóságot („Trust this CA to identify websites”).
https-ca-cli-title = A hoard parancssori eszköz
https-ca-cli-before = { "" }
https-ca-cli-middle = , vagy adja meg az elérési utat itt:{ " " }
https-ca-cli-after = .
https-options = Beállítások
https-redirect = A más eszközökön futó böngészők átirányítása http://-ről https://-re
https-redirect-hint = Csak az oldalbetöltések lesznek átirányítva. A hoard parancssori eszköz, az Unreal worker, a szkriptek és az MI-alkalmazások továbbra is működnek http://-n, ahogy a szerveren a http://localhost is.
https-http3-hint-before = Gyorsabb akadozó Wi-Fin és telefonokon. HTTPS kell hozzá, és a TCP mellett az UDP-portnak is nyitva kell lennie ({ $port }; Dockerben:{ " " }
https-http3-hint-after = ).
https-http3-running = Fut.
https-http3-waits = Akkor indul el, amikor a HTTPS be van kapcsolva.
https-http3-failed = A HTTP/3 nem tudott elindulni: { $error }
https-lockout-title = Ha kizárta magát
https-lockout-1 = { "" }
https-lockout-2 = { " " }magán a szerveren mindig működik sima http://-n. Vagy kapcsolja ki a HTTPS-t a szerver parancssorából, és indítsa újra a Hoardot:{ " " }
https-lockout-3 = { " " }(Dockerben:{ " " }
https-lockout-4 = , majd{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Beállítások → Fiók)

account-role-view = Megtekintő
account-role-view-hint = Keresés, előnézet és letöltés
account-role-edit = Szerkesztő
account-role-edit-hint = Hozzáadás, módosítás, címkézés, gyűjteménybe rendezés és archiválás is
account-role-admin = Admin
account-role-admin-hint = Beállítások, fiókok és karbantartás, valamint végleges törlés is
account-source-internal = Hoard-jelszó
account-source-ldap = Jelszó a címtárból ({ $directory })
account-source-oauth = Bejelentkezés: { $provider }
account-source-word-directory = címtár
account-load-failed = Nem sikerült betölteni a fiókokat: { $error }
account-failed = { $what }: { $error }
account-password-failed = Nem sikerült módosítani a jelszavát
account-password-changed = A jelszó módosítva. A többi eszközén ki lett jelentkeztetve.
account-add-failed = Nem sikerült felvenni: { $name }
account-added = { $name } mostantól bejelentkezhet.
account-role-failed = Nem sikerült módosítani a jogosultságait: { $name }
account-remove-title = Eltávolítja ezt a személyt: { $name }?
account-remove-body = Mindenhol kijelentkeztetik, és nem jelentkezhet be újra. Semmi sem törlődik abból, amit a könyvtárhoz hozzáadott.
account-remove = Eltávolítás
account-remove-failed = Nem sikerült eltávolítani: { $name }
account-token-failed = Nem sikerült létrehozni a tokent
account-copied = Másolva
account-copy-token-failed = Nem sikerült másolni: jelölje ki a tokent, és másolja ki saját maga
account-copy-failed = Nem sikerült másolni: jelölje ki, és másolja ki saját maga
account-revoke-title = Visszavonja ezt: „{ $name }”?
account-revoke-body = Ami még használja, azt mostantól elutasítja.
account-revoke = Visszavonás
account-revoke-failed = Nem sikerült visszavonni: „{ $name }”
account-disconnect-title = Leválasztja ezt: { $app }?
account-disconnect-body = Mostantól elutasítja, és újra csatlakoztatni kell, hogy használhassa a Hoardot.
account-disconnect = Leválasztás
account-disconnect-failed = Nem sikerült leválasztani: { $app }
account-guest-failed = Nem sikerült módosítani
account-never = soha
account-you = Ön
account-sign-out = Kijelentkezés
account-signed-in-as-before = Bejelentkezett felhasználó:{ " " }
account-signed-in-as-admin = , aki admin.
account-signed-in-as-edit = , aki módosíthatja a könyvtárat.
account-signed-in-as-view = , aki megtekinthet mindent, de semmit sem módosíthat.
account-app-signin = A Hoard alkalmazás magától bejelentkeztet, ezért nincs jelszó. Alább hozz létre hozzáférési tokent a motorbővítményekhez és a hoard parancshoz.
account-password-at-provider = A jelszavát a bejelentkezéshez használt szolgáltató tárolja, ezért ott módosíthatja.
account-password-at-directory = A jelszavát a bejelentkezéshez használt címtár tárolja, ezért ott módosíthatja.
account-current-password = Jelenlegi jelszó
account-new-password = Új jelszó
account-new-password-again = Új jelszó még egyszer
account-change-password = Jelszó módosítása
account-password-mismatch = Az új jelszavak nem egyeznek.
account-password-rule = Legalább 8 karakter.
account-people = Személyek
account-people-hint = Minden személy lehet megtekintő (keresés, előnézet és letöltés), szerkesztő (hozzáadás, szerkesztés, címkézés, gyűjteménybe rendezés és archiválás is), vagy admin (Beállítások, fiókok és karbantartás, valamint végleges törlés is).
account-you-tag = (Ön)
account-signs-in-oauth = Az OAuth-szolgáltatóval jelentkezik be
account-signs-in-directory = A címtárral jelentkezik be
account-since = { $date } óta
account-what-can-do = Jogosultság: { $name }
account-name = Név
account-how-sign-in = Bejelentkezés módja
account-password-label = Jelszó (legalább 8 karakter)
account-password-placeholder = Jelszó (8+ karakter)
account-what-they-can-do = Jogosultság
account-add = Hozzáadás
account-ai-apps = A Claude és más MI-alkalmazások
account-ai-apps-hint = Csatlakoztassa a Claude-ot (vagy bármely MCP-alkalmazást), és assetekre kereshet és letöltheti őket Ön helyett, valamint leírhatja és felcímkézheti, amit a Hoard még nem. Első alkalommal bejelentkezik a Hoardba; ezt itt, a böngészőben engedélyezi.
account-mcp-address = A Hoard MCP-címe
account-copy = Másolás
account-claude-code-1 = futtassa ezt:{ " " }
account-claude-code-2 = , majd ezt:{ " " }
account-claude-code-3 = { " " }a bejelentkezéshez.{ " " }
account-claude-code-4 = { " " }leír egy csomagot.
account-claude-ai = Settings → Connectors → Add custom connector, illessze be a címet, majd Connect. Ezek az Anthropic szervereiről kapcsolódnak, ezért a Hoardnak HTTPS-en elérhetőnek kell lennie az internetről.
account-claude-ai-local = Settings → Connectors → Add custom connector, illessze be a címet, majd Connect. Ezek az Anthropic szervereiről kapcsolódnak, ezért a Hoardnak HTTPS-en elérhetőnek kell lennie az internetről (ez a cím nem az: állítsa be a Nyilvános címet a Beállítások → Kapcsolat oldalon).
account-connection-meta = felhasználó: { $user } · csatlakoztatva: { $since } · utoljára használva: { $used }
account-no-apps = Még nincs csatlakoztatott alkalmazás.
account-live = Most használják a Hoardot
account-live-hint = Alkalmazások, amelyek az elmúlt 10 percben kértek valamit a Hoardtól. Magától frissül.
account-live-none = Az elmúlt 10 percben semmi nem kért semmit a Hoardtól.
account-live-last = utoljára: { $what } · { $when } · kérések: { $calls }
account-tokens = Hozzáférési tokenek
account-tokens-hint-1 = A{ " " }
account-tokens-hint-2 = { " " }parancssori eszközhöz, szkriptekhez, MCP-kliensekhez és az Unreal workerhez: így küldik el:{ " " }
account-tokens-hint-3 = . Egy token azt teheti, amit kiválaszt, legfeljebb annyit, amennyit Ön, de fiókokat és tokeneket soha nem kezelhet. A csak megtekintő token elég a kereséshez, a letöltéshez és egy projekt szinkronizálásához.
account-tokens-admin = Ön mindenki tokenjeit és csatlakoztatott alkalmazásait látja; mások csak a sajátjukat.
account-made-before = { "" }
account-made-after = : másolja ki most. Többé nem jelenik meg.
account-new-token = Az új token
account-cli-1 = A CLI-hez:{ " " }
account-cli-2 = , vagy adja meg itt:{ " " }
account-cli-3 = .
account-done = Kész
account-token-meta = létrehozva: { $made } · utoljára használva: { $used }
account-token-meta-by = létrehozva: { $made }, létrehozó: { $by } · utoljára használva: { $used }
account-no-tokens = Még nincsenek tokenek.
account-token-name = Mi fogja használni
account-token-name-placeholder = Mi fogja használni, pl. „Unreal worker”
account-token-role = Mit tehet
account-make-token = Token létrehozása
account-guests = Vendégek
account-guests-allow = Bárki böngészhet bejelentkezés nélkül, aki eléri a szervert
account-guests-hint = Kereshetnek, előnézetet nézhetnek és letölthetnek, de semmit sem adhatnak hozzá, módosíthatnak vagy törölhetnek, és a Beállításokat sem láthatják.

## SignIn.vue

signin-create-title = Hozza létre a fiókját
signin-title = Bejelentkezés
signin-setup-lede = Ezen a szerveren még nincs fiók. Az itt létrehozott fiók mindenre jogosult; mások számára később a Beállítások → Fiók oldalon vehet fel fiókokat.
signin-overlay-lede = A böngészés itt mindenki számára nyitott, de a módosításokhoz fiók kell.
signin-with = Bejelentkezés: { $provider }
signin-or-password = vagy névvel és jelszóval
signin-username = Felhasználónév
signin-name = Név
signin-new-password = Új jelszó
signin-password = Jelszó
signin-password-rule = Legalább 8 karakter.
signin-confirm-password = Jelszó megerősítése
signin-mismatch = A két jelszó nem egyezik.
signin-setup-code = Beállítási kód
signin-setup-code-hint-before = A kód, amelyet a szerver itt kapott meg:{ " " }
signin-setup-code-hint-after = .
signin-creating = Létrehozás…
signin-signing-in = Bejelentkezés…
signin-create = Fiók létrehozása
signin-submit = Bejelentkezés
signin-keep-browsing = Böngészés folytatása
signin-directory-hint = Használja a címtárbeli ({ $directory }) nevét és jelszavát.
signin-forgot-1 = Elfelejtette a Hoard-jelszavát? Futtassa a szerveren ezt:{ " " }
signin-forgot-2 = { " " }(Dockerben:{ " " }
signin-forgot-3 = ), hogy újat kapjon.

## SetupWizard.vue (első látogatás egy új szerveren)

setup-title = Hol legyen a könyvtára?
setup-lede = Válasszon egy mappát a szerveren, ahol a Hoard a csomagjait tárolja, vagy mutasson egy meglévő csomagmappára. Később a Beállításokban további helyeket adhat hozzá, más lemezeken is.
setup-lede-desktop = Válasszon egy mappát, ahol a Hoard a csomagjait tárolja, vagy mutasson egy meglévő csomagmappára. Később a Beállításokban további helyeket adhat hozzá, más lemezeken is.
setup-unreachable = Nem sikerült elérni a szervert: { $error }
setup-unreachable-desktop = A Hoard nem válaszol: { $error }
setup-try-again = Újra
setup-loading = Betöltés…
setup-start = A könyvtár indítása
setup-ready = A könyvtára elkészült
setup-managed-before = Az új csomagok ide kerülnek:{ " " }
setup-managed-after = . Az első csomagok hozzáadásához húzzon fájlokat, mappákat vagy tömörített fájlokat bárhová az oldalon.
setup-watched-before = A Hoard ezt a mappát figyeli:{ " " }
setup-watched-after = ; a csomagok megjelennek, ahogy megtalálja őket. A feltöltéshez adjon hozzá egy a Hoard által kezelt könyvtárat a Beállítások → Tárolási helyek oldalon.
setup-open = A könyvtár megnyitása

## ConnectApp.vue (egy alkalmazás, például a Claude, bejelentkezést kér)

connect-title = Csatlakoztatja ezt: { $app }?
connect-lede-1 = { " " }kereshet, letölthet, leírhat és felcímkézhet mindent ebben a Hoardban, a következő nevében:{ " " }
connect-lede-2 = . Fiókokat és hozzáférési tokeneket nem kezelhet.
connect-back-before = Ezután visszakerül ide:{ " " }
connect-back-after = . Bármikor leválaszthatja a Beállítások → Fiók oldalon.
connect-allow = Engedélyezés
connect-cancel = Mégse
connect-failed = Ez az alkalmazás nem csatlakoztatható
connect-go-library = Ugrás a könyvtárhoz
connect-checking = Ellenőrzés…

## InsecureNotice.vue (a sima http:// fölött megjelenő sor)

insecure-text = Ez sima http://, pedig néhány dologhoz HTTPS kell: 3D WebGPU-val, a modern vágólap, a Hoard alkalmazásként való telepítése és a Claude-összekötők.
insecure-switch = Váltás https://-re
insecure-set-up = HTTPS beállítása
insecure-dismiss = Ne jelenjen meg többé

## FormatWizard.vue (Beállítások → Áruházi fájlok → áruházi fájl megtanítása)

formatwizard-too-big = Ez a fájl túl nagy ahhoz, hogy áruházi fájlként beolvasható legyen.
formatwizard-not-json = Ez a fájl nem érvényes JSON, ezért szövegként jelenik meg.
formatwizard-read-failed = Nem sikerült beolvasni a fájlt: { $error }
formatwizard-back-title = Vissza az űrlaphoz?
formatwizard-back-body = A TOML-ben végzett módosítások elvesznek: az űrlap a meglévő adataiból újraírja a formátumot.
formatwizard-back-to-form = Vissza az űrlaphoz
formatwizard-miss-name = A Fájlok listája nem illeszkedik erre a névre: { $file }. Adja hozzá a Fájlokhoz.
formatwizard-miss-not-the-stores = Ami alapján az áruház fájljait felismeri, az nincs ebben a fájlban. Módosítsa a Felismerés alapja mezőt.
formatwizard-miss-no-title = Nem talál címet ebben a fájlban. Válassza ki a címet.
formatwizard-miss-short-name = nem illeszkedő név
formatwizard-miss-short-not-the-stores = nem az áruházé
formatwizard-miss-short-no-title = nincs cím
formatwizard-saved-packs = Mentve: { $file }. { $packs } csomag újraolvasása folyamatban.
formatwizard-saved = Mentve: { $file }.
formatwizard-save-failed = Nem sikerült menteni a formátumot: { $error }
formatwizard-zip-failed = Nem sikerült elkészíteni a zipet: { $error }
formatwizard-stop-title = Abbahagyja a fájl megtanítását?
formatwizard-stop-body = A formátum nincs mentve.
formatwizard-stop = Abbahagyás
formatwizard-label = Formátumvarázsló
formatwizard-changing = Módosítás: { $name }
formatwizard-starting-from = Kiindulás: { $name }
formatwizard-title = Áruházi fájl megtanítása a Hoardnak
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }– csomag:{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Szerkesztés TOML-ként
formatwizard-file-reference = A fájl, tájékoztatásul.
formatwizard-click-json = Keresse meg a fájlban, és kattintson rá: { $field }.
formatwizard-click-text = Keresse meg a fájlban, és kattintson rá vagy jelölje ki: { $field }.
formatwizard-pick-first = Válasszon egy mezőt jobb oldalt, majd kattintson itt az értékére.
formatwizard-loading = Betöltés…
formatwizard-the-format = A formátum
formatwizard-syntax-before = A szintaxis leírása itt található:{ " " }
formatwizard-syntax-after = .
formatwizard-store = Áruház
formatwizard-store-placeholder = Az áruház neve, pl. Kenney
formatwizard-files = Fájlok
formatwizard-recognize = Felismerés alapja
formatwizard-recognize-json = Egy kulcs, amely csak az áruház fájljaiban van
formatwizard-recognize-text = Egy sor, amely csak az áruház fájljaiban van
formatwizard-recognize-hint = Soronként egy módszer; bármelyik elég. Akkor kell, ha más fájloknak is ugyanez a neve.
formatwizard-seller = Eladó
formatwizard-seller-placeholder = Ha a fájl nem adja meg
formatwizard-tags = Címkék
formatwizard-tags-placeholder = Mindenhez, amit leír, vesszővel elválasztva
formatwizard-fields = Mezők
formatwizard-selector = { $field } – kiválasztó
formatwizard-clear = Törlés: { $field }
formatwizard-pick = Kijelölés
formatwizard-what-it-reads = Amit beolvas
formatwizard-read-title = Cím
formatwizard-read-seller = Eladó
formatwizard-read-license = Licenc
formatwizard-read-url = Áruházi oldal
formatwizard-read-kind = Típus
formatwizard-read-tags = Címkék
formatwizard-read-origin = Készült
formatwizard-read-size = Méret
formatwizard-also = { $format } is beolvassa ezt a fájlt. Ezt a formátumot próbálná először.
formatwizard-library = { $files } fájl neve illeszkedik ({ $packs } csomagban). A kipróbált { $tried } fájlból { $read } beolvasható.
formatwizard-library-none = A könyvtárban egyetlen fájl neve sem illeszkedik.
formatwizard-share = Megosztás
formatwizard-share-hint = A formátum zipje, ezzel a fájllal mintaként és azzal, amit a mintából beolvas, a Hoard tárolójának szerkezetében, hogy beküldje, és mindenki megkapja. Előbb vegyen ki a mintából mindent, amit nem kellene megosztani.
formatwizard-sample = Megosztandó minta
formatwizard-download = Letöltés: { $file }
formatwizard-close = Bezárás
formatwizard-use = Használat itt
formatwizard-cancel = Mégse
settings-unreal-editor-what = Telepíti az Unreal Editort az exportáló folyamathoz. Az elemek exportálásához külön indítsa el a folyamatot.
settings-unreal-download = Egyszer letölti az Epic Linux x86-64 szerkesztőjét a Hoard eszközmappájába. Több tíz GB lemezterületet foglal, és olyan GitHub-fiókot igényel, amelyet az Epic beengedett (lásd: GitHub-bejelentkezés).
settings-unreal-github = GitHub-bejelentkezés
settings-unreal-github-why = Az Epic a Linux-szerkesztőt privát konténerképként teszi közzé a GitHub csomagregiszterében (ghcr.io), és a Hoard onnan tölti le a crane segítségével, amely a Hoarddal együtt érkezik. A GitHub csak olyan fiókoknak adja ki, amelyeket az Epic beengedett, ezért a Hoardnak szüksége van egy ilyen fiókra a letöltéshez. A Hoard a tokent a többi beállításával együtt tárolja, és semmi másra nem használja.
settings-unreal-github-step-link = Kapcsolja össze GitHub-fiókját az Epic Games-fiókjával.
settings-unreal-github-step-link-go = Az összekapcsolás módja
settings-unreal-github-step-invite = Fogadja el az EpicGames szervezetbe szóló meghívást, amelyet a GitHub e-mailben küld (néhány percbe telhet, mire megérkezik).
settings-unreal-github-step-token = Hozzon létre egy GitHub-tokent (classic), amelyben csak a read:packages van bejelölve.
settings-unreal-github-step-token-go = Token létrehozása
settings-unreal-github-step-enter = Adja meg itt GitHub-felhasználónevét és a tokent, majd válassza az „Automatikus” lehetőséget.
settings-unreal-github-user = GitHub-felhasználónév
settings-unreal-github-token = Token
settings-unreal-github-token-kept = Mentve (cseréjéhez írjon be egy újat)
settings-unreal-github-save = Bejelentkezés mentése
settings-unreal-github-forget = Elfelejtés
settings-unreal-github-saved = GitHub-bejelentkezés mentve; a letöltés újrapróbálása folyamatban
settings-unreal-github-forgotten = GitHub-bejelentkezés elfelejtve
settings-unreal-github-from-server = Ez a szerver saját bejelentkezést is kapott (GHCR_TOKEN_FILE vagy DOCKER_CONFIG); ezt használja, ha itt nincs mentve egy sem.
settings-unreal-image = Unreal-verzió vagy konténerkép
settings-editor-path = Adja meg a telepített { $tool } futtatható fájljának útvonalát. Az automatikus telepítés csak Dockerben érhető el.
error-unreal-image = Adjon meg Unreal 5-verziót, Epic dev/dev-slim verziócímkét vagy Epic SHA-256 képhivatkozást.
