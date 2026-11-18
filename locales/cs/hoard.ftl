## Chyby serveru

error-account-not-found = účet nebyl nalezen
error-asset-gone = asset nebyl nalezen
error-asset-not-found = asset { $id } nebyl nalezen
error-blob-not-found = blob { $hash } nebyl nalezen
error-collection-not-found = kolekce { $id } nebyla nalezena
error-connection-not-found = připojení nebylo nalezeno
error-derivative-not-found = odvozený soubor { $kind } assetu { $id } nebyl nalezen
error-file-in-pack-not-found = { $path } v balíčku { $id } nebyl nalezen
error-format-not-found = formát { $file } nebyl nalezen
error-frame-sheet-aseprite-only = list snímků mají jen soubory Aseprite
error-frame-sheet-frames = tento soubor má jen jeden snímek, nebo jich má příliš mnoho na rozložení
error-location-not-found = umístění { $id } nebylo nalezeno
error-maintenance-task-not-found = úloha údržby { $id } nebyla nalezena
error-open-upload-not-found = otevřené nahrávání { $id } nebylo nalezeno
error-original-not-found = původní soubor assetu { $id } nebyl nalezen
error-pack-not-found = balíček { $id } nebyl nalezen
error-preview-too-big = tento obrázek je příliš velký na vytvoření odlehčeného náhledu
error-project-not-found = projekt { $name } nebyl nalezen
error-saved-search-not-found = uložené hledání { $id } nebylo nalezeno
error-sign-in-first = nejprve se přihlaste (nebo pošlete přístupový token)
error-those-assets-not-found = tyto assety nebyly nalezeny
error-thumbnail-not-found = miniatura pro { $id } nebyla nalezena
error-token-not-found = token nebyl nalezen
error-upload-not-found = nahrávání { $id } nebylo nalezeno
error-pack-needs-name = balíček potřebuje název
error-no-library = zatím není žádné umístění knihovny, kam ukládat soubory: přidejte spravované v Nastavení → Umístění knihovny
error-upload-failed = nahrávání selhalo: { $reason }
error-upload-no-files = v nahrávání nejsou žádné soubory
error-analyze-nothing-to-do = není co dělat: rules=false a LLM je vypnutý
error-too-many-assets = příliš mnoho assetů v jednom požadavku
error-too-many-items = příliš mnoho položek v jednom požadavku
error-not-a-hash = toto není hexadecimální hash blake3
error-merge-into-tag = into musí být štítek
error-nothing-to-merge = není co sloučit
error-selection-size = zadejte 1 až { $max } ID assetů
error-collection-needs-name = kolekce potřebuje název
error-collection-exists = kolekce s názvem { $name } už existuje
error-collection-empty = kolekce { $name } je prázdná
error-conversion-failed = převod do { $format } se nezdařil: { $reason }
error-max-size = max musí být mocnina dvou od 64 do 16384
error-replace-ext = ext musí být krátká přípona souboru
error-empty-file = prázdný soubor
error-replace-duplicate = převedený soubor je stejný jako jiný, který už v balíčku je; původní soubor Unrealu byl sloučen do něj
error-ai-address = Adresa AI by měla začínat http:// nebo https://
error-local-llm-action = action je list, download nebo delete a poslední dvě potřebují model
error-role = Role je view, edit nebo admin
error-admin-only = tohle může udělat jen správce
error-view-only = tento účet si může vše prohlížet, ale nic měnit: správce to může změnit v Nastavení → Účet
error-portal-only = účty a tokeny spravujte po přihlášení do portálu
error-not-from-portal = tento požadavek nepřišel z portálu Hoard
error-account-name = Zvolte jméno o nejvýše 64 znacích
error-password-length = Použijte heslo o alespoň { $min } znacích
error-too-many-codes = Příliš mnoho špatných kódů: počkejte { $seconds } s
error-already-set-up = Účet už existuje: přihlaste se
error-too-many-tries = Příliš mnoho špatných pokusů: počkejte { $seconds } s
error-directory-unavailable = Adresář nemohl ověřit vaše heslo: { $reason }
error-password-elsewhere = Vaše heslo spravuje adresář nebo poskytovatel, přes kterého se přihlašujete
error-current-password = Vaše současné heslo není správné
error-account-source = Účet se přihlašuje přes Hoard (internal), adresář (ldap) nebo OAuth
error-account-exists = Účet s tímto jménem už existuje
error-own-role = Vaši roli může změnit jiný správce
error-remove-self = Nemůžete odstranit vlastní účet, když jste do něj přihlášeni
error-token-name = Pojmenujte token podle toho, co ho bude používat, nejvýše 80 znaky
error-token-role = Token nemůže víc než účet, který ho vytváří
error-wrong-setup-code = To není instalační kód zadaný serveru v HOARD_SETUP_CODE
error-wrong-password = Jméno a heslo k sobě nepatří
error-folder-missing = { $path } neexistuje (není připojený disk?)
error-s3-path = umístění S3 vypadá jako s3://bucket
error-s3-address = adresa S3 musí začínat http:// nebo https://
error-s3-header = další hlavička S3 vypadá jako Name=value a „{ $header }“ taková není
error-keychain = nepodařilo se to uložit do systémové klíčenky: { $reason }
error-not-built = tato kopie Hoardu byla sestavena bez { $feature }
error-openai-auth-unreachable = nepodařilo se spojit s auth.openai.com
error-openai-unexpected = neočekávaná odpověď od OpenAI
error-openai-answered = OpenAI odpověděl { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID token není JWT
error-chatgpt-id-token-issuer = ID token je od { $issuer }, ne od { $expected }
error-chatgpt-id-token-audience = ID token je pro jinou aplikaci
error-chatgpt-id-token-expired = platnost ID tokenu vypršela
error-chatgpt-id-token-nonce = ID token nepatří k tomuto přihlášení
error-chatgpt-no-state = adresa neobsahuje state; zkopírujte celou adresu
error-chatgpt-not-started = toto přihlášení nebylo zahájeno tady, nebo bylo zahájeno před více než 15 minutami. Zahajte ho znovu z Nastavení.
error-chatgpt-canceled = přihlášení bylo zrušeno
error-chatgpt-refused = ChatGPT přihlášení odmítl: { $reason }
error-chatgpt-no-code = adresa neobsahuje přihlašovací kód; zkopírujte celou adresu
error-chatgpt-other-registration = ChatGPT odpověděl pro jinou registraci tohoto serveru; zahajte přihlášení znovu
error-chatgpt-other-registration-desktop = ChatGPT odpověděl pro jinou kopii Hoardu; zahajte přihlášení znovu
error-chatgpt-not-registered = ChatGPT nedokončil registraci Hoardu; zahajte přihlášení znovu
error-chatgpt-code-used = přihlašovací kód už byl použit nebo vypršel; přihlaste se znovu
error-chatgpt-refused-code = ChatGPT přihlášení odmítl ({ $code })
error-chatgpt-no-id-token = ChatGPT neposlal ID token
error-chatgpt-plan-not-allowed = Přihlášení proběhlo, ale používání předplatného nebylo povoleno. Přihlaste se znovu a povolte ho, nebo místo toho použijte klíč API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = nejste přihlášeni do ChatGPT; přihlaste se v Nastavení → Štítkování pomocí AI
error-chatgpt-refresh-later = ChatGPT teď nemohl obnovit přihlášení ({ $code }); zkusí se to znovu později
error-chatgpt-signed-out-problem = ChatGPT tento server odhlásil ({ $code }). Přihlaste se znovu, abyste mohli dál štítkovat se svým předplatným.
error-chatgpt-signed-out-problem-desktop = ChatGPT Hoard odhlásil ({ $code }). Přihlaste se znovu, abyste mohli dál štítkovat se svým předplatným.
error-chatgpt-signed-out = ChatGPT tento server odhlásil ({ $code }); přihlaste se znovu v Nastavení → Štítkování pomocí AI
error-chatgpt-signed-out-desktop = ChatGPT Hoard odhlásil ({ $code }); přihlaste se znovu v Nastavení → Štítkování pomocí AI
error-openai-api-unreachable = nepodařilo se spojit s api.openai.com
error-openai-broke-off = odpověď od OpenAI se přerušila
error-chatgpt-kept-refusing = OpenAI opakovaně odmítal přihlášení do ChatGPT; přihlaste se znovu v Nastavení → Štítkování pomocí AI
error-chatgpt-request-failed = požadavek selhal
error-chatgpt-cut-off = odpověď byla useknuta ({ $reason })
error-chatgpt-cut-off-no-reason = odpověď byla useknuta (bez udání důvodu)
error-openai-ended-early = odpověď od OpenAI skončila dřív, než byla úplná
error-chatgpt-usage-limit = limit vašeho předplatného ChatGPT pro Hoard je zatím vyčerpán. Zobrazit nebo změnit ho můžete na { $url }
error-chatgpt-not-eligible = tento účet nebo pracovní prostor ChatGPT nemůže používat své předplatné v jiných aplikacích. Použijte místo toho klíč API.
error-chatgpt-usage-unavailable = ChatGPT teď nemohl zkontrolovat využití předplatného; zkusí se to znovu později
error-chatgpt-unsupported = využití předplatného ChatGPT nepodporuje u tohoto modelu { $param }
error-chatgpt-unsupported-images = využití předplatného ChatGPT nepodporuje u tohoto modelu { $param }; vyberte model, který přijímá obrázky, nebo vypněte posílání miniatur
error-chatgpt-unsupported-part = využití předplatného ChatGPT nepodporuje u tohoto modelu část požadavku
error-chatgpt-port = port je ten, na kterém prohlížeč Hoard otevírá
error-chatgpt-paste-address = vložte celou adresu začínající http://127.0.0.1
error-too-many-hashes = nejvýše { $max } hashů najednou
error-credits-pack = balíček { $key }
error-credits-collection = kolekce { $key }
error-format-too-long = formát je příliš dlouhý
error-not-started = server se ještě nespustil
error-not-started-desktop = Hoard se ještě nespustil
error-not-format-file = není to soubor formátu: { $name }
error-format-name = název potřebuje písmeno nebo číslici od A do Z nebo od 0 do 9
error-format-miss-name = soubory formátu nezahrnují { $file }
error-format-miss-store = formát nebere { $file } pro tento obchod (viz require)
error-format-miss-title = formát v { $file } nenachází žádný název
error-cant-download-as = soubory .{ $ext } nelze stáhnout jako { $format }
error-only-images-resize = měnit velikost lze jen u obrázků
error-https-no-certificate = Nejprve vložte nebo nahrajte svůj certifikát a jeho klíč
error-https-no-cert-found = Certifikát nebyl nalezen: vložte text, který začíná -----BEGIN CERTIFICATE-----
error-https-no-key-found = Soukromý klíč nebyl nalezen: vložte text, který začíná -----BEGIN PRIVATE KEY-----
error-https-several-keys = Je tu víc než jeden soukromý klíč: vložte jen ten, který patří k certifikátu
error-https-key-password = Klíč je chráněný heslem: zadejte ho bez hesla (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Certifikát není platný PEM
error-https-key-not-pem = Soukromý klíč není platný PEM
error-https-key-mismatch = Klíč nepatří k certifikátu. Pokud jste zadali řetězec, dejte certifikát svého serveru na začátek
error-https-unusable = Certifikát nebo klíč nelze použít
error-https-expired = Platnost tohoto certifikátu skončila { $date }
error-https-not-yet-valid = Tento certifikát je platný až od { $date }
error-https-unreadable = To není certifikát, který Hoard umí přečíst
error-https-bad-name = „{ $name }“ není adresa, kterou může certifikát obsahovat
error-https-test-timed-out = časový limit testovacího připojení vypršel
error-https-test-failed = testovací připojení selhalo
error-https-no-ca = Hoard ještě nevytvořil svou certifikační autoritu
error-https-unavailable = HTTPS není na tomto serveru dostupné
error-https-certificate-mode = Certifikát patří k volbě „Vlastní certifikát“
error-https-test = Certifikát v testovacím připojení nefungoval: { $reason }
error-nothing-to-retry = tady není nic, co by šlo zkusit znovu
error-ai-unreachable = nepodařilo se spojit s { $url }
error-ai-key-refused = { $url } odmítl klíč API ({ $status })
error-ai-answered = { $url } odpověděl { $status }: { $reason }
error-ai-not-json = { $url }/models neodpověděl ve formátu JSON
error-model-server-refused = server modelů: { $reason }
error-no-model-server = na { $url } neodpovídá žádný server modelů
error-not-llama-router = { $url } není server llama.cpp v režimu router, takže neumí stahovat modely
error-model-download = nepodařilo se stáhnout { $model }: { $reason }
error-model-delete = nepodařilo se smazat { $model }: { $reason }
error-unknown-kind = neznámý druh assetu { $kind }
error-watched-is-folder = sledované umístění je složka na tomto serveru
error-cant-reach = nelze se spojit s { $path }: { $reason }
error-folder-full-path = zadejte úplnou cestu ke složce
error-cant-create = nelze vytvořit { $path }
error-cant-write = nelze zapisovat do { $path }
error-cant-read = nelze číst { $path }
error-overlaps-location = to se překrývá s umístěním { $name } ({ $path })
error-overlaps-data = to se překrývá s datovou složkou Hoardu ({ $path })
error-overlaps-cache = to se překrývá se složkou mezipaměti Hoardu ({ $path })
error-not-watched = { $name } není sledovaná složka
error-watched-no-kinds = sledovaná složka nepřijímá nové soubory, takže nemá žádné druhy ke změně
error-keep-catch-all = místo toho udělejte záchytným umístěním jiné umístění
error-location-holds-files = { $name } stále obsahuje soubory: nejprve je přesuňte do ostatních umístění (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } není dostupné, takže Hoard nepozná, jestli stále obsahuje soubory
error-relocate-running = soubory se už přesouvají mezi umístěními; zkuste to znovu, až to skončí
error-watched-not-moved = { $name } je sledovaná složka: Hoard její soubory nepřesouvá
error-no-other-location = není žádné jiné umístění knihovny, kam soubory přesunout: nejprve nějaké přidejte
error-full-path = zadejte úplnou cestu
error-pick-folder = vyberte složku
error-task-running = „{ $task }“ už běží
error-token-account-gone = účet tohoto tokenu už neexistuje, takže nemá žádné oblíbené ani hledání
error-favorites-too-many = nejvýše 10 000 najednou
error-saved-search-name = uložené hledání potřebuje název o nejvýše 80 znacích
error-saved-search-too-long = toto hledání je příliš dlouhé na uložení
error-texture-pattern-map = Vzor názvu textury potřebuje { "{" }map{ "}" }, jinak by všechny mapy materiálu dostaly stejný název
error-texture-pattern-fields = Vzor názvu textury může používat { "{" }name{ "}" }, { "{" }map{ "}" } a { "{" }size{ "}" }
error-texture-name-chars = Názvy textur nemohou obsahovat < > : " / \ | ? ani *
error-texture-map-type = Typ mapy { $role } neexistuje
server-role-base-color = barva
server-role-normal = normála
server-role-normal-dx = normála (DirectX)
server-role-roughness = drsnost
server-role-gloss = lesk
server-role-metalness = kovovost
server-role-ao = ambientní okluze
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = výška
server-role-bump = bump
server-role-cavity = dutiny
server-role-specular = spekulární odraz
server-role-opacity = neprůhlednost
server-role-emissive = emise
server-role-translucency = průsvitnost
error-texture-suffix-missing = Zadejte příponu pro mapy typu { $map }
error-texture-suffix-chars = Přípona pro mapy typu { $map } nemůže obsahovat < > : " / \ | ? ani *
error-texture-ending-missing = Každá vlastní koncovka potřebuje koncovku, kterou má hledat
error-texture-ending-chars = Vlastní koncovky nemohou obsahovat < > : " / \ | ? ani *
error-texture-ending-twice = { $ending } je ve vašich koncovkách dvakrát
error-oauth-no-client-id = V tomto odkazu chybí client_id.
error-oauth-unknown-app = Tato aplikace tu není (už) registrovaná. Zahajte připojení znovu z aplikace.
error-oauth-redirect = Aplikace požádala o návrat na adresu, kterou nezaregistrovala.
error-oauth-response-type = Hoard vydává jen přihlašovací kódy (response_type=code).
error-oauth-pkce = Aplikace musí k přihlášení do Hoardu použít PKCE s S256.
error-light-preview-images = odlehčený náhled mají jen obrázky
error-no-light-preview = žádný odlehčený náhled: { $reason }
error-no-frame-sheet = žádný list snímků: { $reason }
error-thumbnail-source = neznámý zdroj miniatury: { $source }
error-thumbnail-too-big = ten obrázek je na miniaturu příliš velký
error-thumbnail-models-only = miniaturu vykreslenou prohlížečem mohou mít jen modely
error-not-a-picture = není to obrázek, který Hoard umí přečíst: { $reason }
error-project-name = projekt potřebuje název o nejvýše 200 znacích
error-project-too-big = nejvýše milion assetů v jednom projektu
error-project-too-many-at-once = nejvýše 100 000 najednou
error-server-name = Zvolte název o nejvýše { $max } znacích, na jednom řádku
error-tool-setting = { $tool }: zvolte vypnuto, automaticky, nebo zadejte cestu
error-ai-model = Zvolte model, který má služba AI používat
error-public-address = Veřejná adresa by měla začínat http:// nebo https://
error-update-hook-address = Adresa volaná při aktualizacích by měla začínat http:// nebo https://
error-update-hook-auth = Hlavička Authorization nemůže obsahovat zalomení řádků ani jiné řídicí znaky
error-no-soundfont = Na serveru není na { $path } žádný SoundFont
error-no-soundfont-desktop = Na { $path } není žádný SoundFont
error-workers = Úlohy na pozadí: 1 až { $max }
error-ai-batch = Assetů na jeden požadavek AI: 1 až 200
error-conversion-cache = Převedená stahování: alespoň 100 MB
error-conversion-keep-days = Pravidla převodu: originály lze uchovávat 1 až { $max } dní
error-conversion-target = Pravidla převodu: Hoard neumí převádět soubory do { $format }
error-conversion-no-source = Pravidla převodu: každé pravidlo potřebuje alespoň jeden formát k převodu
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Pravidla převodu: soubory .{ $ext } nelze převést do { $format }
error-conversion-twice = Pravidla převodu: .{ $ext } je ve dvou pravidlech
error-conversion-cant-undo = Tento soubor nepřevedlo pravidlo, nebo už byl jeho originál smazán.
error-conversion-path-taken = Na { $path } je teď jiný soubor, takže se tam originál nemůže vrátit.
error-limits-zero = Limity nemohou být nulové
error-settings-object = očekáván objekt JSON s nastavením
error-bad-setting = chybné nastavení: { $reason }
error-auth-default-role = Výchozí role je view, edit nebo admin
error-auth-session-hours = Relace trvají od 1 hodiny do jednoho roku
error-auth-settings = chybné nastavení ověřování: { $reason }
error-ldap-settings = chybné nastavení LDAP: { $reason }
error-ldap-no-server = Přidejte adresu adresářového serveru
error-ldap-base-dn = Uveďte, kde jsou záznamy lidí (base DN)
error-ldap-username-attribute = Uveďte, který atribut obsahuje jméno, kterým se lidé přihlašují
error-ldap-bind-dn = Zadejte účet, který lidi vyhledává, nebo je vyhledávejte anonymně
error-ldap-ca-cert = Certifikační autorita není certifikát PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Adresa adresářového serveru je prázdná
error-ldap-server-form = { $server }: adresa adresářového serveru má tvar host:port, ldap://… nebo ldaps://…
error-ldap-not-starttls = { $server }: to je LDAPS, ne StartTLS
error-ldap-no-certificate = neobsahuje žádný certifikát
error-ldap-password-again = Zadejte znovu heslo účtu, který lidi vyhledává: uložené heslo se posílá jen serverům, pro které bylo zadáno
error-ldap-unreachable = nepodařilo se spojit s žádným adresářovým serverem ({ $reasons })
error-ldap-no-anonymous = adresář nepovoluje anonymní vyhledávání lidí ({ $reason })
error-ldap-bind-failed = účet, který lidi vyhledává, se nemohl přihlásit ({ $reason })
error-ldap-search = hledání { $name } v { $base }
error-ldap-group-search = hledání skupin uživatele { $name } v { $base }
error-ldap-several =
    { $count ->
        [one] { $count } záznam v adresáři odpovídá
        [few] { $count } záznamy v adresáři odpovídají
       *[other] { $count } záznamů v adresáři odpovídá
    } { $name }: zužte filtr uživatelů
error-ldap-password-check = adresář odmítl ověřit heslo uživatele { $name } ({ $reason })
error-ldap-wrong-password = Adresář nezná uživatele { $name } s tímto heslem
error-oauth-client-id = Zadejte client ID, které poskytovatel vytvořil pro Hoard
error-oauth-addresses = Adresy autorizace a tokenu by měly začínat https://
error-oauth-userinfo-address = Adresa s informacemi o uživateli by měla začínat https://
error-oauth-username-claim = Uveďte, který claim obsahuje jméno člověka
error-oauth-secret-again = Zadejte znovu client secret: uložený se posílá jen na adresu tokenu, pro kterou byl zadán
error-oauth-issuer = Vydavatel (issuer) by měl začínat https://
error-oauth-discovery-unreachable = Nepodařilo se spojit s { $url }: { $reason }
error-oauth-discovery-status = { $url } odpověděl { $status }
error-oauth-discovery-bad = { $url } není dokument discovery
error-oauth-discovery-incomplete = { $url } neuvádí adresy autorizace a tokenu
error-signin-no-group = { $name } není ve skupině, která smí Hoard používat
error-signin-other-source = Účet Hoard se jménem { $name } už existuje a přihlašuje se jinak
error-signin-other-subject = Účet Hoard se jménem { $name } už existuje pro někoho jiného u poskytovatele
error-signin-no-account = { $name } zatím nemá účet Hoard: správce ho může přidat v Nastavení → Účet
error-signin-bad-name = Toto jméno nemůže být účtem Hoard (nejvýše 64 znaků)
error-oauth-off = Přihlašování přes OAuth není zapnuté
error-oauth-bad-authorization-address = adresa autorizace není platná
error-oauth-refused = Poskytovatel vás nepřihlásil: { $reason }
error-oauth-no-code = Poskytovatel vás poslal zpět bez přihlášení
error-oauth-expired = Přihlášení trvalo příliš dlouho nebo bylo zahájeno jinde: zkuste to znovu
error-oauth-no-claim = Poskytovatel neuvedl, kdo jste (chybí claim { $claim })
error-oauth-no-sub = Poskytovatel neuvedl, kdo jste (chybí claim sub nebo id)
error-oauth-token-unreachable = nepodařilo se spojit s adresou tokenu poskytovatele
error-oauth-token-refused = Poskytovatel nedokončil přihlášení: { $reason }
error-oauth-no-access-token = poskytovatel neposlal přístupový token
error-oauth-userinfo-unreachable = nepodařilo se spojit s adresou informací o uživateli u poskytovatele
error-oauth-userinfo-refused = Poskytovatel nesdělil, kdo se přihlásil ({ $status })
error-oauth-userinfo-not-json = informace o uživateli od poskytovatele nejsou JSON
error-format-no-name = name je prázdné
error-format-no-files = files je prázdné: uveďte, které soubory formát čte, například ["*.json"]
error-format-empty-selector = prázdné
error-format-unopened-bracket = uzavírací závorka bez otevírací
error-format-unclosed-bracket = závorka není uzavřená
error-format-empty-step-dots = prázdný krok (dvě tečky za sebou nebo tečka na začátku či konci)
error-format-unclosed-except = !( není uzavřené
error-format-unclosed-filter = [ není uzavřené
error-format-after-filters = { $rest } za klíčem a filtry kroku { $step }
error-format-except-needs-glob = !(…) patří za klíč obsahující *
error-format-except-not-glob = !(…) patří za klíč obsahující *, ne za { $key }
error-format-empty-step = prázdný krok
error-format-not-a-filter = [{ $filter }] není filtr: použijte [], [key=value|other], [array], [string], [number], [bool] nebo [object]
error-format-filter-needs-value = [{ $filter }] potřebuje klíč a hodnotu
error-store-search-empty = není co hledat; napište název balíčku
error-update-hook-not-set = nejprve nastavte adresu, která se má volat
error-upload-stalled = nahrávání { $path } se zaseklo; pošlete tu část znovu
error-upload-busy = { $path } se už zapisuje
error-upload-incomplete =
    { $count ->
        [one] { $count } soubor je neúplný
        [few] { $count } soubory jsou neúplné
       *[other] { $count } souborů je neúplných
    }, například { $files }
error-upload-no-files-listed = žádné soubory
error-upload-not-part = { $path } není součástí nahrávání { $id }, nebo je nahrávání uzavřené
error-bad-path = chybná cesta
error-upload-part-offset = na pozici { $offset } nezačíná žádná část { $path }
error-upload-interrupted = nahrávání přerušeno: { $reason }
error-upload-part-long = část { $part } souboru { $path } je delší než { $len } bajtů
error-upload-part-short = část { $part } souboru { $path }: přijato { $got } bajtů, očekáváno { $len }
error-upload-file-long = { $path } je větší, než bylo uvedeno
error-upload-file-short = { $path }: přijato { $got } bajtů, očekáváno { $len }
error-upload-complete = nahrávání je už dokončené
error-already-imported = už importováno jako „{ $name }“

## Text serveru

server-store-place-readme = README
server-store-place-folders = názvy složek
server-store-why-linked = Odkaz je uvedený v balíčku ({ $place })
server-store-why-same-title = Stejný název jako v balíčku ({ $place })
server-store-why-by = Od autora { $author }, kterého balíček uvádí ({ $place })
server-store-why-meshes = Přibližně stejně meshů jako v balíčku ({ $count })
server-store-why-textures = Přibližně stejně textur jako v balíčku ({ $count })
server-chatgpt-revoke-failed = Tady jste odhlášeni, ale OpenAI se to nepodařilo oznámit. Pro jistotu odpojte Hoard v ChatGPT v Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Přihlášeno do ChatGPT
server-chatgpt-signed-in = Hoard je přihlášený. Štítkování pomocí AI teď může používat vaše předplatné ChatGPT.
server-chatgpt-signed-in-as = Hoard je přihlášený jako { $email }. Štítkování pomocí AI teď může používat vaše předplatné ChatGPT.
server-chatgpt-not-finished-title = Přihlášení nebylo dokončeno
server-chatgpt-other-computer = Pokud jste přihlášení zahájili z Hoardu na jiném počítači, zkopírujte adresu této stránky z adresního řádku a vložte ji tam do Nastavení Hoardu.
server-back-to-settings = Zpět do Nastavení
server-issues-pack = Balíčky, které se nepodařilo přidat
server-issues-pack-blurb = Smažte balíček a přidejte ho znovu, nebo ho otevřete a podívejte se, co obsahuje.
server-issues-damaged = Uložené soubory, které jsou poškozené nebo chybí
server-issues-damaged-blurb = Nalezeno úlohou „Zkontrolovat poškození souborů“. Obnovte je ze zálohy nebo znovu přidejte jejich balíček a pak znovu spusťte „Zkontrolovat poškození souborů“.
server-issues-folder = Sledované složky, které nelze přečíst
server-issues-folder-blurb = Jejich balíčky zůstanou, jak jsou, dokud složku nepůjde znovu přečíst.
server-issues-missing = Sledované balíčky, jejichž složka zmizela
server-issues-missing-blurb = Každý balíček se ponechá 3 dny pro případ, že je složka jen offline, a pak se odstraní.
server-issues-job = Úlohy na pozadí, které selhaly
server-issues-job-blurb = Opakování spustí úlohu znovu od začátku.
server-issues-file = Soubory, které nešlo otevřít
server-issues-file-blurb = Hoard je nedokázal otevřít ani uložit. Sledovaný balíček je zkusí znovu při příštím prohledání; soubor, který už je v knihovně, si ponechal to, co měl předtím.
server-issues-unpack = Archivy, které nešlo rozbalit
server-issues-unpack-blurb = Každý zůstává ve svém balíčku jako obyčejný soubor, takže nic z jeho obsahu v knihovně není.
server-issues-read = Soubory, kterým Hoard neporozuměl
server-issues-read-blurb = Jsou v knihovně, ale bez údajů, štítků a miniatury, které by přineslo jejich přečtení.
server-issues-render = Modely, u kterých vykreslování miniatur selhalo
server-issues-render-blurb = Nemají miniaturu. Otevření v prohlížeči modelů může přesto fungovat a miniaturu uložit.
server-issues-convert = Modely, které Blender nedokázal převést
server-issues-convert-blurb = Nemají kopii GLB, takže je nelze zobrazit ve 3D.
server-issues-unreal = Soubory Unrealu, které nešlo převést
server-issues-unreal-blurb = Ostatní soubory Unrealu najdete na stránce Unreal daného balíčku.
server-issues-reformat = Soubory, které pravidlo převodu nedokázalo převést
server-issues-reformat-blurb = Zůstaly tak, jak byly.
server-issues-materials = Balíčky, jejichž materiály nešlo zjistit
server-issues-materials-blurb = Textury jsou všechny na místě; jen nejsou seskupené do materiálů.
server-issues-material-thumb = Materiály bez miniatury
server-issues-material-thumb-blurb = Materiál byl nalezen, ale jeho náhled nešlo vykreslit.
server-issues-store-files = Soubory obchodů, které nešlo přečíst
server-issues-store-files-blurb = Soubory z obchodu, ze kterého balíček pochází (například JSON z Megascans), které ho pojmenovávají a štítkují.
server-issues-scan-again = Prohledat znovu
server-issues-retry = Zkusit znovu
server-issues-reanalyze = Znovu analyzovat balíček
server-issues-convert-again = Převést znovu
server-issues-look-again = Podívat se znovu
server-issues-read-again = Přečíst znovu
server-issues-blob-missing = Uložený soubor zmizel.
server-issues-blob-damaged = Uložený soubor neodpovídá tomu, co bylo uloženo: je poškozený.
server-issues-folder-gone = Složka „{ $folder }“ nebyla nalezena.
server-job-a-pack = balíček
server-job-ingest = Přidávání: { $pack }
server-job-retag = Nové štítkování: { $pack }
server-job-llm-tag = Štítkování pomocí AI: { $pack }
server-job-convert = Převod v Blenderu: { $pack }
server-job-unreal = Převod souborů Unrealu: { $pack }
server-job-reformat = Převod souborů podle pravidel převodu: { $pack }
server-job-scan = Prohledávání sledované složky
server-job-relocate = Přesouvání souborů mezi umístěními knihovny
server-job-looks = Zjišťování barev a vzhledu
server-job-sounds = Poslech zvuků
server-job-embed = Učení, co zobrazují obrázky
server-job-materials = Hledání materiálů: { $pack }
server-job-read-files = Čtení souborů: { $pack }
server-job-store-lookup = Vyhledávání stránky v obchodě
server-maintenance-backup = Zálohovat databázi
server-maintenance-backup-blurb = Konzistentní kopie databáze (balíčky, štítky, poznámky, kolekce a Nastavení) ve složce backups vedle ní. Soubory knihovny v ní nejsou: po uložení se nikdy nemění, takže je zálohuje jakákoli synchronizace souborů.
server-maintenance-scrub = Zkontrolovat poškození souborů
server-maintenance-scrub-blurb = Přečte každý uložený soubor a porovná ho s jeho otiskem, aby odhalila soubory, které se na disku nebo NAS poškodily nebo zmizely. U velké knihovny to chvíli trvá.
server-maintenance-reanalyze = Znovu analyzovat všechny balíčky
server-maintenance-reanalyze-blurb = Znovu přečte každý soubor touto verzí Hoardu: miniatury, náhledy, podrobnosti a štítky z pravidel se vytvoří znovu. Vaše vlastní úpravy zůstanou; AI se znovu neptá.
server-maintenance-clear-cache = Vymazat převody v mezipaměti
server-maintenance-clear-cache-blurb = Zahodí soubory uchované z převodů „Stáhnout jako“ (při dalším požadavku se vytvoří znovu) a to, co zanechala přerušená práce.
server-maintenance-shrink-thumbs = Zmenšit miniatury
server-maintenance-shrink-thumbs-blurb = Uloží miniatury a pohledy na modely uchované jako PNG tak, jak určuje Nastavení → Výkon → Miniatury, což je obvykle několikanásobně zmenší. Ty s průhledností a pixel art zůstanou PNG.
server-maintenance-gc = Smazat nepoužívané soubory
server-maintenance-gc-blurb = Smaže uložené soubory, na které už nic neodkazuje, například soubory trvale smazaných balíčků.
server-update-no-address = není nastavená žádná adresa k volání
server-update-no-answer = žádná odpověď do 30 sekund

## Nastavení: Vzhled

appearance-language = Jazyk
appearance-language-auto = Podle prohlížeče ({ $name })

## Kořen

root-unreachable = Nepodařilo se spojit se serverem Hoard: { $error }
root-unreachable-desktop = Hoard neodpovídá: { $error }
root-desktop-reopen = Toto okno bylo odhlášeno. Zavřete Hoard a znovu ho otevřete, abyste mohli pokračovat.
root-try-again = Zkusit znovu
root-lazy-connect = připojení aplikace
window-minimize = Minimalizovat
window-maximize = Maximalizovat
window-restore = Obnovit
window-close = Zavřít

## Aplikace: knihovna, horní lišta a filtry

app-lazy-settings = nastavení
app-lazy-archive = archiv
app-lazy-shortcuts = klávesové zkratky
app-lazy-setup = průvodce nastavením
app-theme-system = Podle systému
app-theme-light = Světlý motiv
app-theme-dark = Tmavý motiv
app-color-red = Červená
app-color-orange = Oranžová
app-color-yellow = Žlutá
app-color-green = Zelená
app-color-teal = Tyrkysová
app-color-blue = Modrá
app-color-purple = Fialová
app-color-pink = Růžová
app-color-brown = Hnědá
app-color-sand = Písková
app-color-gray = Šedá
app-color-slate = Břidlicová
app-color-black = Černá
app-color-white = Bílá
app-color-swatch = { $color }: assety, ve kterých převládá { $lower }
app-color-any = Jakákoli barva
app-color-pick = Vybrat libovolnou barvu
app-color = Barva
app-imitation = Zní jako
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Napište zvuk tak, jak byste ho hláskovali: nejdřív se zobrazí zvuky, které se mu podobají
app-retry = Zkusit znovu
app-undo = Zpět
app-search-failed = Hledání selhalo: { $error }
app-unreachable = Nepodařilo se spojit se serverem: { $error }
app-unreachable-desktop = Hoard neodpovídá: { $error }
app-forget-project-title = Zapomenout { $name }?
app-forget-project-body =
    Hoard ho přestane uvádět u { $count ->
        [one] { $count } assetu, který používá
        [few] { $count } assetů, které používá
       *[other] { $count } assetů, které používá
    }, dokud mu to příští hoard sync znovu neoznámí. V projektu se nic nezmění.
app-forget-project-confirm = Zapomenout
app-forget-project-failed = { $name } se nepodařilo zapomenout: { $error }
app-forget-project-label = Zapomenout projekt { $name }
app-forget-project = Zapomenout tento projekt
app-save-name-like = podobné { $name }
app-save-name-favorites = oblíbené
app-search-saved = Uloženo „{ $name }“: najdete ho ve Filtrech v části Vaše
app-rank-alike = Nejpodobnější
app-rank-color = Nejvíc dané barvy
app-rank-match = Nejlepší shoda
app-rank-imitation = Nejvíc se podobá
app-sort = Řadit
app-sort-newest = Nejnovější
app-sort-name = Název
app-sort-largest = Největší
app-sort-oldest = Nejstarší
app-collection-added =
    { $count ->
        [one] Přidán { $count } asset
        [few] Přidány { $count } assety
       *[other] Přidáno { $count } assetů
    } do { $name }
app-collection-add-failed = Nepodařilo se přidat do kolekce: { $error }
app-collection-removed = Odebráno z { $name }: { $count }
app-collection-remove-failed = Nepodařilo se odebrat z kolekce: { $error }
app-ns-subject = Námět
app-ns-style = Styl
app-ns-theme = Téma
app-ns-material = Materiál
app-ns-map = Mapa PBR
app-ns-color = Barva
app-ns-mood = Nálada
app-ns-genre = Žánr
app-ns-license = Licence
app-ns-res = Rozlišení
app-ns-duration = Délka
app-ns-poly = Počet polygonů
app-ns-format = Formát
app-ns-weight = Velikost
app-ns-tech = Technické
app-ns-blender = Verze Blenderu
app-ns-depth = Bitová hloubka
app-ns-ucs = Kategorie zvuku (UCS)
app-ns-bpm = Tempo
app-ns-root = Základní tón
app-ns-level = Hlasitost
app-ns-ue = Třída Unrealu
app-ns-engine = Engine
app-ns-extracted = Obnovený zdroj
app-ns-script = Písmo
app-bulk-failed =
    { $count ->
        [one] Tento asset se nepodařilo změnit
        [few] Tyto { $count } assety se nepodařilo změnit
       *[other] Těchto { $count } assetů se nepodařilo změnit
    }: { $error }
app-bulk-untagged =
    Štítek { $tag } odebrán z { $count ->
        [one] { $count } assetu
        [few] { $count } assetů
       *[other] { $count } assetů
    }
app-bulk-tagged =
    Štítek { $tag } přidán { $count ->
        [one] { $count } assetu
        [few] { $count } assetům
       *[other] { $count } assetům
    }
app-bulk-category-set =
    Kategorie { $count ->
        [one] { $count } assetu
        [few] { $count } assetů
       *[other] { $count } assetů
    } nastavena na { $category }
app-bulk-category-cleared =
    Kategorie vymazána u { $count ->
        [one] { $count } assetu
        [few] { $count } assetů
       *[other] { $count } assetů
    }
app-credits-failed = Nepodařilo se vytvořit poděkování autorům: { $error }
app-archived-assets =
    { $count ->
        [one] Archivován { $count } asset
        [few] Archivovány { $count } assety
       *[other] Archivováno { $count } assetů
    }
app-archived-one = Archivováno: { $name }
app-open-failed = Tento asset se nepodařilo otevřít: { $error }
app-unnamed-pack = balíček
app-unnamed-collection = kolekce
app-unnamed-asset = asset
app-home-version = Hoard { $version } — zpět do knihovny
app-home = Zpět do knihovny
app-search-label = Hledat assety
app-search-placeholder = Hledat názvy, cesty, štítky
app-views = Zobrazení
app-library = Knihovna
app-packs = Balíčky
app-packs-working = Zpracovává se: { $count }
app-update-title = Je venku Hoard { $version } (tento server běží na { $current }). Stáhněte nový image nebo binárku a restartujte; databáze se aktualizuje sama.
app-update = Aktualizovat
app-theme = Motiv
app-sign-in = Přihlásit se
app-archive-title = Archiv: { $contents }
app-archive = Archiv
app-settings = Nastavení
app-palette-title = Paleta příkazů ({ $keys })
app-import-folder = Importovat složku
app-add-files = Přidat soubory
app-add-files-short = Přidat
app-split-label = Jak importovat tuto složku
app-split-before = Složka{ " " }
app-split-after =
    { " " }obsahuje { $count ->
        [one] { $count } složku nebo archiv.
        [few] { $count } složky nebo archivy.
       *[other] { $count } složek nebo archivů.
    }
app-split-each = Importovat každou jako samostatný balíček ({ $count })
app-split-one = Importovat jako jeden balíček
app-cancel = Zrušit
app-filters = Filtry
app-done = Hotovo
app-yours = Vaše
app-favorites = Oblíbené
app-forget-search-label = Zapomenout uložené hledání { $name }
app-forget-search = Zapomenout toto hledání
app-projects-title = Projekty, které odsud synchronizuje hoard sync, s tím, co měla jejich poslední synchronizace
app-projects = Projekty
app-project-synced = Synchronizováno { $when }
app-project-synced-by = Synchronizováno { $when }, { $who }
app-collections = Kolekce
app-no-filters = Pro toto hledání nejsou žádné filtry.
app-kind = Druh
app-facets-more = Další ({ $count })
app-facets-fewer = méně
app-searching = Hledám…
app-searching-live = Hledám
app-asset-count =
    { $count ->
        [one] { $count } asset
        [few] { $count } assety
       *[other] { $count } assetů
    }
app-clear-all = Vymazat vše
app-save-name-label = Název tohoto hledání
app-save-name-placeholder = Pojmenujte ho
app-save = Uložit
app-save-search-title = Uchovat toto hledání ve Filtrech v části Vaše
app-save-search = Uložit hledání
app-select-title = Vyberte několik assetů, které chcete společně oštítkovat, zařadit do kategorie, přidat do kolekce, stáhnout nebo archivovat (X nebo Ctrl+klik)
app-select = Vybrat
app-tile-size = Velikost dlaždic
app-supporting-title = Buffery glTF, knihovny materiálů, mapy materiálů, materiály, které nosí jen jeden model, a soubory obchodů popisující modely. Stáhnou se spolu s tím, co je používá. Stránky spritesheetu a jeho sprity, které balíček má i jako samostatné obrázky, se zobrazují u spritesheetu. Asset, který balíček má v několika formátech, se zobrazí jednou, ve formátu, který Hoard čte nejlépe.
app-supporting-hidden =
    { $count ->
        [one] { $count } podpůrný soubor je skrytý
        [few] { $count } podpůrné soubory jsou skryté
       *[other] { $count } podpůrných souborů je skrytých
    }
app-supporting-show = Zobrazit
app-supporting-shown = Včetně podpůrných souborů
app-supporting-hide = Skrýt
app-folders = Složky
app-folders-title = Procházet balíček tak, jak byl na disku, složku po složce
app-folders-label = Složky balíčku
app-welcome-title = Přetáhněte balíček assetů kamkoli na tuto stránku
app-welcome-before = Funguje zip, tarball, složka i jednotlivé soubory. Hoard je rozbalí, každý soubor uloží jen jednou podle jeho hashe, zjistí velikosti, délky a počty polygonů a všechno oštítkuje, takže můžete hledat třeba{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }nebo{ " " }
app-welcome-after = .
app-welcome-import-folder = Importovat složku
app-welcome-choose-files = Vybrat soubory
app-none-similar = Zatím se { $name } nic jiného moc nepodobá.
app-none = Nic neodpovídá.
app-none-query = „{ $query }“ nic neodpovídá.
app-none-imitation = Žádný změřený zvuk zatím nezní moc jako „{ $word }“.
app-none-filters = S těmito filtry nic neodpovídá.
app-none-query-filters = „{ $query }“ s těmito filtry nic neodpovídá.
app-clear-search-filters = Vymazat hledání a filtry
app-loading-more = Načítám další
app-keys-hint = Šipky přesouvají, Enter otevírá, mezerník přehraje zvuk, X nebo Ctrl+klik vybírá více položek (na dotykovém displeji dlouhé podržení).
app-resize-label = Změnit velikost panelu podrobností
app-resize-title = Táhnutím změníte velikost · dvojklik nastaví šířku podle obsahu
app-selected-label = Vybrané assety
app-selected = Vybráno: { $count }
app-pick-assets = Vyberte assety
app-download-zip = Stáhnout zip
app-credits-title = CREDITS.md pro balíčky, ze kterých pocházejí: kdo je vytvořil, jejich licence a řádky s poděkováním, o které žádají
app-credits = Poděkování
app-prefer-sources-title = SoundWaves a textury z Unrealu, jejichž původní soubor Hoard obnovil
app-prefer-sources = Obnovené WAV/PNG místo .uasset ({ $count })
bag-clear = Zrušit výběr
bag-glb-heading = Modely jako GLB
bag-glb-text = Každý model jako soubor .glb stojící osou Y nahoru a otočený dopředu, jeho textury jako samostatné soubory ve složce textures, kterou modely sdílejí. Vše ostatní jde dovnitř tak, jak je.
bag-glb-download = Stáhnout GLB zip
bag-plain-heading = Soubory tak, jak jsou
bag-plain-text = Každý soubor tak, jak byl importován, s tím, co potřebuje, ve složce pro každý balíček.
bag-remove = Odebrat { $name } z výběru
bag-more = a další ({ $count })
app-bulk-tag-label = Štítek, který chcete vybraným assetům přidat nebo odebrat
app-bulk-tag-placeholder = Štítek
app-bulk-tag-add-title = Přidat tento štítek všem vybraným assetům
app-bulk-tag-add = Přidat
app-bulk-tag-remove-title = Odebrat tento štítek všem vybraným assetům
app-bulk-tag-remove = Odebrat
app-bulk-category-label = Kategorie pro vybrané assety (prázdná ji vymaže)
app-bulk-category-placeholder = Kategorie
app-bulk-category-set-title = Nastavit kategorii všem vybraným assetům
app-bulk-category-clear-title = Vymazat kategorii všem vybraným assetům
app-bulk-category-set-button = Nastavit
app-bulk-category-clear-button = Vymazat
app-remove-from-collection = Odebrat z { $name }
app-unfavorite = Odebrat z oblíbených
app-favorite = Přidat do oblíbených
app-select-all = Vybrat vše ({ $count })
app-archive-selected-title = Mimo hledání, dokud je v Archivu neobnovíte nebo trvale nesmažete
app-drop-items =
    { $count ->
        [one] Pusťte { $count } položku a přidáte ji
        [few] Pusťte { $count } položky a přidáte je
       *[other] Pusťte { $count } položek a přidáte je
    }
app-drop = Pusťte a přidejte do Hoardu
app-drop-hint = Každý soubor nebo složka nejvyšší úrovně se stane samostatným balíčkem. Gigabajty nevadí; velké soubory pokračují, pokud spojení vypadne.

## Paleta příkazů

palette-group-go-to = Přejít na
palette-group-actions = Akce
palette-group-theme = Motiv
palette-group-kinds = Druhy
palette-group-packs = Balíčky
palette-group-collections = Kolekce
palette-group-projects = Projekty
palette-group-saved = Uložená hledání
palette-group-assets = Assety
palette-hint-current = aktuální
palette-library = Knihovna
palette-library-keywords = assety mřížka domů assets grid home
palette-favorites = Oblíbené
palette-favorites-keywords = hvězdička hvězdičky líbí starred stars liked
palette-packs = Balíčky
palette-archive = Archiv
palette-archive-keywords = koš smazané obnovit trash recycle bin deleted restore
palette-stats = Statistiky knihovny
palette-stats-keywords = statistiky čísla velikost místo disk počet statistics numbers size disk space count
palette-search = Hledat
palette-add-files = Přidat soubory…
palette-add-files-keywords = nahrát importovat upload import
palette-import-folder = Importovat složku…
palette-import-folder-keywords = nahrát balíček upload pack
palette-select-all = Vybrat všechny zobrazené ({ $count })
palette-select-all-keywords = označit check
palette-select-several = Vybrat několik…
palette-select-several-keywords = hromadně více štítek archivovat bulk multiple tag archive
palette-archive-selected = Archivovat vybrané ({ $count })
palette-archive-selected-keywords = smazat odstranit koš delete remove trash
palette-clear-selection = Zrušit výběr
palette-clear-filters = Vymazat filtry
palette-clear-filters-keywords = resetovat obnovit reset
palette-hide-supporting = Skrýt podpůrné soubory
palette-show-supporting = Zobrazit podpůrné soubory
palette-supporting-keywords = normálové mapy drsnost masky normal maps roughness masks
palette-account = Účet a přístupové tokeny
palette-account-keywords = heslo přihlášení uživatel token api password login user
palette-sign-out = Odhlásit se ({ $name })
palette-sign-out-keywords = odhlásit odhlášení log out logout
palette-sign-in = Přihlásit se
palette-sign-in-keywords = přihlásit přihlášení účet heslo log in login account password
palette-settings = Nastavení
palette-settings-keywords = blender ffmpeg llm ai štítkování unreal správce nástroje předvolby tagging admin tools preferences
palette-shortcuts = Klávesové zkratky
palette-shortcuts-keywords = nápověda klávesy help keys
palette-theme-keywords = motiv vzhled režim theme appearance mode
palette-theme-variant-keywords = vzhled barva appearance color
palette-light-theme = Světlý motiv: { $name }
palette-dark-theme = Tmavý motiv: { $name }
palette-kind-all = Všechny druhy (přestat zobrazovat jen { $kind })
palette-kind-only = Jen { $kind }
palette-title = Paleta příkazů
palette-placeholder = Hledat assety, přejít na balíček, spustit příkaz…
palette-results = Výsledky
palette-searching = Hledám…
palette-no-match = „{ $query }“ nic neodpovídá
palette-foot-move = pohyb
palette-foot-run = spustit
palette-foot-shortcuts = všechny zkratky

## Klávesové zkratky

shortcuts-title = Klávesové zkratky
shortcuts-close = Zavřít
shortcuts-anywhere = Kdekoli
shortcuts-palette = Paleta příkazů: hledat, přejít, spustit
shortcuts-search = Hledat
shortcuts-this-list = Tento seznam
shortcuts-escape = Zavřít, zrušit výběr nebo zavřít podrobnosti
shortcuts-library = Knihovna
shortcuts-move = Pohyb mezi dlaždicemi (nebo J a K)
shortcuts-open = Otevřít podrobnosti
shortcuts-play-sound = Přehrát nebo zastavit zvuk
shortcuts-select = Vybrat nebo zrušit výběr
shortcuts-favorite = Přidat do oblíbených nebo z nich odebrat
shortcuts-key-click = klik
shortcuts-select-range = Vybrat rozsah
shortcuts-select-all = Vybrat vše zobrazené
shortcuts-sound-player = Přehrávač zvuků
shortcuts-play-pause = Přehrát nebo pozastavit
shortcuts-skip = Skok zpět nebo dopředu
shortcuts-start-end = Začátek nebo konec
shortcuts-image-inspector = Prohlížeč obrázků
shortcuts-zoom = Přiblížit nebo oddálit
shortcuts-fit = Přizpůsobit oknu
shortcuts-actual-pixels = Skutečné pixely
shortcuts-tile = Dlaždice 3×3 pro kontrolu švů
shortcuts-background = Další pozadí
shortcuts-channel = Zobrazit jeden kanál

## KindChips.vue

kinds-group = Druhy assetů

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = přidal člověk
tag-source-rule = z offline štítkovače
tag-source-meta = změřeno ze souboru
tag-source-ai = navrženo AI
tag-remove = Odebrat štítek { $tag }

## ActivityIndicator.vue

activity-label-idle = Aktivita na pozadí: nečinná
activity-label-running = Aktivita na pozadí: běží { $running }
activity-label-queued = Aktivita na pozadí: běží { $running }, ve frontě { $queued }
activity-title = Aktivita na pozadí
activity-unreachable = Nelze se spojit se serverem
activity-unreachable-desktop = Hoard neodpovídá
activity-empty = Nic neběží. Importy, štítkování, převody a stahování zipů se tu zobrazují, dokud probíhají.
activity-queued = ve frontě
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } troj.
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Přehrát { $name }
tile-pause = Pozastavit { $name }
tile-favorite = Přidat { $name } do oblíbených
tile-in-favorites = V oblíbených
tile-add-favorite = Přidat do oblíbených (F)
tile-more-like = Další podobné jako { $name }
tile-more-like-this = Další podobné
tile-select = Vybrat { $name }

## CollectionPicker.vue

collections-add = Přidat do kolekce
collections-menu = Kolekce
collections-find-label = Najít nebo vytvořit kolekci
collections-find-placeholder = Najděte nebo pojmenujte novou kolekci
collections-create = Vytvořit
collections-empty = Zatím žádné kolekce. Začněte novou zadáním názvu.

## FolderPicker.vue

folders-list = Složky
folders-close = Zavřít
folders-browse = Procházet…
folders-up-title = O složku výš
folders-up = Nahoru
folders-this-computer = Tento počítač
folders-none = Nejsou tu žádné složky.

## LocationForm.vue

locations-mode-question = Co má Hoard s touto složkou dělat?
locations-mode-managed = Uchovávat tu mou knihovnu
locations-mode-managed-hint = Balíčky, které nahrajete nebo importujete, se zkopírují do této složky. Hoard se o ni stará, takže její obsah neměňte.
locations-mode-watched = Indexovat složku, kterou už mám
locations-mode-watched-hint = Každá složka uvnitř se stane balíčkem. Hoard čte soubory tam, kde jsou, nikdy je nemění, nepřesouvá ani nemaže a zachytí nové složky, jakmile se objeví.
locations-folder = Složka na serveru
locations-folder-desktop = Složka
locations-folder-placeholder = Složka, ve které jsou vaše balíčky
locations-folder-hint-managed = Hoard ji vytvoří, pokud neexistuje. V Dockeru je to složka, kterou jste připojili do kontejneru.
locations-folder-hint-managed-desktop = Hoard ji vytvoří, pokud neexistuje.
locations-folder-hint-watched = V Dockeru nejprve připojte složku do kontejneru (stačí jen pro čtení) a pak ji vyberte tady.
locations-folder-hint-watched-desktop = Hoard ji jen čte: nic v ní se nemění.
locations-folder-hint-cloud = Funguje i složka, kterou synchronizuje Dropbox, Google Drive, OneDrive nebo iCloud Drive, a také s3://bucket-name pro bucket v úložišti S3.
locations-name = Název
locations-name-placeholder-managed = Knihovna
locations-name-placeholder-watched = Moje balíčky
locations-files = Soubory, které sem patří
locations-catch-all = Také všechno, co nepřijme žádné jiné umístění
locations-kinds-hint = Soubory nového balíčku jdou do umístění, které má zaškrtnutý jejich druh. Zbytek jde do záchytného umístění.
locations-files-all = Zatím všechny. Později můžete v Nastavení přidat další umístění a posílat některé druhy (třeba hudbu) na jiný disk.
locations-first-watched-note = Nahrávání potřebuje knihovnu, kterou Hoard spravuje. Můžete ji přidat později v Nastavení → Umístění knihovny.
locations-checking = Kontroluji složku…
locations-add = Přidat umístění
locations-cancel = Zrušit

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Nepodařilo se načíst umístění knihovny
locations-takes-and-rest = { $kinds } a všechno ostatní
locations-takes-rest = Všechno, co nepřijmou ostatní
locations-takes-everything = Všechno
locations-takes-nothing = Zatím nic
locations-save-failed = Umístění se nepodařilo uložit
locations-unwatch-title = Přestat sledovat { $name }?
locations-unwatch-body =
    { $count ->
        [one] Jeho { $count } balíček odejde
        [few] Jeho { $count } balíčky odejdou
       *[other] Jeho { $count } balíčků odejde
    } z Hoardu i se štítky a poznámkami. Souborů ve složce se to nedotkne.
locations-unwatch-body-empty = Souborů ve složce se to nedotkne.
locations-unwatch = Přestat sledovat
locations-remove-title = Odebrat { $name }?
locations-remove-body = Soubory v něm se nejprve podle druhu přesunou do ostatních umístění; odebere se, jakmile bude prázdné. Nic se neztratí.
locations-remove-body-only = Pokud obsahuje soubory, nejprve přidejte jiné umístění, kam se přesunou.
locations-remove = Odebrat
locations-removed = Odebráno: { $name }
locations-emptying = Přesouvám soubory z { $name }; po dokončení se odebere
locations-offline-title = { $name } není dostupné
locations-offline-body = Dokud je offline, nelze z něj soubory přesunout. Přesto ho odebrat? Soubory, které na něm zůstanou, se budou zobrazovat jako chybějící.
locations-remove-anyway = Přesto odebrat
locations-remove-failed = { $name } se nepodařilo odebrat
locations-rebalancing = Přesouvám uložené soubory do umístění, která přijímají jejich druh
locations-rebalance-failed = Soubory se nepodařilo přesunout
locations-scanned-never = zatím neprohledáno
locations-scanned-now = prohledáno právě teď
locations-scanned-minutes = prohledáno před { $mins } min
locations-scanned-hours = prohledáno před { $hours } h
locations-scanned-date = prohledáno { $date }
locations-scan-failed = { $name } se nepodařilo prohledat
locations-watching = Sleduji { $name }
locations-added = Přidáno: { $name }
locations-loading = Načítám…
locations-managed-title = Knihovny, které Hoard spravuje
locations-watched-title = Sledované složky
locations-managed-hint = Nahrané a importované soubory se kopírují sem. Každé umístění přijímá zaškrtnuté druhy; záchytné umístění přijímá zbytek.
locations-watched-hint = Složky s balíčky indexované tam, kde jsou. Hoard v nich soubory nikdy nemění a každých 15 minut hledá nové a odebrané složky.
locations-managed-empty = Zatím žádné, takže nahrané soubory nemají kam jít. Přidejte nějaké níže.
locations-reachable = Dostupné
locations-unreachable-now = Momentálně nedostupné
locations-catch-all-tag = záchytné
locations-packs =
    { $count ->
        [one] { $count } balíček
        [few] { $count } balíčky
       *[other] { $count } balíčků
    }
locations-unreachable = nedostupné
locations-scanning = Prohledávám…
locations-scan = Prohledat teď
locations-edit = Upravit
locations-catch-all-locked = Chcete-li to změnit, udělejte záchytným jiné umístění
locations-edit-hint-before = Platí pro soubory přidané od teď. Chcete-li přesunout to, co už je uložené, použijte
locations-edit-hint-after = níže.
locations-save = Uložit
locations-rebalance = Přesunout uložené soubory podle nastavení
locations-rebalance-hint = Po změně toho, co umístění přijímá: už uložené soubory se přesunou do umístění, kam teď patří jejich druh.
locations-add-title = Přidat umístění
locations-add-hint = Další disk pro některé druhy souborů nebo složka s balíčky, které už máte.
locations-s3-title = Úložiště S3
locations-s3-hint = Umístění s3://bucket-name je bucket v této službě: AWS nebo jakékoli službě kompatibilní s S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage na NAS).
locations-s3-endpoint = Adresa
locations-s3-endpoint-placeholder = Pro AWS prázdné; např. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Region
locations-s3-access-key = ID přístupového klíče
locations-s3-secret-key = Tajný přístupový klíč
locations-s3-headers = Další hlavičky
locations-s3-headers-placeholder = Name=value, jedna na řádek (třeba pro Cloudflare Access)
locations-s3-kept = Uloženo (psaním nahradíte)
locations-s3-secrets = Klíče a hlavičky se ukládají s nastavením serveru a už se nikdy nezobrazí.
locations-s3-secrets-desktop = Klíče a hlavičky se ukládají do klíčenky tohoto počítače a už se nikdy nezobrazí.
locations-s3-saved = Úložiště S3 uloženo
locations-s3-save-failed = Úložiště S3 se nepodařilo uložit
locations-add-more = Přidat umístění…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Problémy se nepodařilo načíst
issues-started =
    { $count ->
        [one] Spuštěno znovu.
        [few] Znovu spuštěny { $count } položky.
       *[other] Znovu spuštěno { $count } položek.
    }
issues-retry-failed = Nepodařilo se zkusit znovu
issues-dismiss-failed = Nepodařilo se skrýt
issues-dismiss-all-title = Skrýt všechny ({ $count })?
issues-dismiss-all-body = Skupina „{ $title }“ se vyprázdní. Cokoli, co se znovu pokazí, se znovu zobrazí.
issues-dismiss-all = Skrýt vše
issues-just-now = právě teď
issues-minutes-ago = před { $mins } min
issues-hours-ago = před { $hours } h
issues-loading = Načítám…
issues-none = Nic se nepokazilo.
issues-action-all = { $action } vše
issues-open-asset = Otevřít
issues-open-pack = Zobrazit balíček
issues-open-locations = Umístění knihovny
issues-where-before = v
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count }krát
        [few] { $count }krát
       *[other] { $count }krát
    }
issues-starting = Spouštím…
issues-dismiss-named = Skrýt { $title }
issues-dismiss = Skrýt
issues-show-fewer = Zobrazit méně
issues-show-all = Zobrazit vše ({ $count })
issues-not-listed =
    { $count ->
        [one] Další { $count } není uvedený.
        [few] Další { $count } nejsou uvedené.
       *[other] Dalších { $count } není uvedeno.
    }

## archive.ts

archive-anyway = Přesto archivovat
archive-failed = Nepodařilo se archivovat: { $error }
archive-undo = Zpět
archive-restore-failed = Nepodařilo se obnovit: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } balíček
        [few] { $packs } balíčky
       *[other] { $packs } balíčků
    } a { $assets ->
        [one] { $assets } asset
        [few] { $assets } assety
       *[other] { $assets } assetů
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } balíček
        [few] { $packs } balíčky
       *[other] { $packs } balíčků
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } asset
        [few] { $assets } assety
       *[other] { $assets } assetů
    }
archive-count-nothing = nic

## ArchiveView.vue

archive-load-failed = Archiv se nepodařilo načíst: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Obnoveno: { $what }
archive-purged = Trvale smazáno: { $what }
archive-purge-failed = Nepodařilo se smazat: { $error }
archive-purge-title = Trvale smazat { $name }?
archive-purge-pack-body =
    Smaže se i { $count ->
        [one] jeho { $count } asset
        [few] jeho { $count } assety
       *[other] jeho { $count } assetů
    } a { $size } souborů, kromě souborů, které sdílí jiný balíček. Tuto akci nelze vrátit zpět.
archive-purge = Smazat navždy
archive-purge-asset-body = Jeho soubor se uvolní, pokud ho nesdílí jiný asset. Tuto akci nelze vrátit zpět.
archive-empty-title = Vyprázdnit archiv?
archive-empty-body = Trvale se smaže: { $what }. Jejich soubory se uvolní. Tuto akci nelze vrátit zpět.
archive-empty = Vyprázdnit archiv
archive-title = Archiv
archive-intro = Archivované balíčky a assety jsou mimo hledání, počty i seznam štítků. Obnovte je, nebo je trvale smažte a uvolněte místo.
archive-loading = Načítám…
archive-is-empty = Archiv je prázdný
archive-is-empty-hint = Archivujte balíček nebo asset a počká tu, dokud ho neobnovíte nebo trvale nesmažete.
archive-packs = Balíčky
archive-assets = Assety
archive-asset-count =
    { $count ->
        [one] { $count } asset
        [few] { $count } assety
       *[other] { $count } assetů
    }
archive-archived = archivováno { $when }
archive-from = z { $pack }
archive-restore = Obnovit
archive-show-more = Zobrazit další (zbývá { $count })

## mine.ts

projects-favorite-add-failed = Nepodařilo se přidat do oblíbených: { $error }
projects-favorite-remove-failed = Nepodařilo se odebrat z oblíbených: { $error }
projects-save-search-failed = Hledání se nepodařilo uložit: { $error }
projects-forget-search-failed = { $name } se nepodařilo odebrat: { $error }

## projects.ts

projects-list-and = { $items } a { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more =
    { $count ->
        [one] { $count } další
        [few] { $count } další
       *[other] { $count } dalších
    }
projects-other =
    { $count ->
        [one] { $count } další projekt
        [few] { $count } další projekty
       *[other] { $count } dalších projektů
    }
projects-usage-title-one = Toto používá { $name }
projects-usage-title =
    { $count ->
        [one] Toto používá { $count } projekt
        [few] Toto používají { $count } projekty
       *[other] Toto používá { $count } projektů
    }
projects-usage-body-one = Používá se v projektu { $list }: příští hoard sync je odtamtud odebere.
projects-usage-body = Používá se v projektech { $list }: příští hoard sync každého z nich je odtamtud odebere.

## uploader.ts

upload-network-error = chyba sítě
upload-gave-up =
    { $what }: { $error } po { $attempts ->
        [one] { $attempts } pokusu
        [few] { $attempts } pokusech
       *[other] { $attempts } pokusech
    }
upload-failed-plain = selhalo
upload-already-imported = už importováno
upload-part-failed = nahrávání selhalo ({ $status })
upload-batch =
    dávka { $count ->
        [one] { $count } souboru
        [few] { $count } souborů
       *[other] { $count } souborů
    }
upload-batch-failed = dávka selhala ({ $status })
upload-finish = dokončení nahrávání

## UploadQueue.vue

upload-eta-seconds = zbývá { $s } s
upload-eta-minutes = zbývá { $min } min
upload-eta-hours = zbývá { $h } h
upload-queue = Importy
upload-importing =
    { $count ->
        [one] Importuji { $count } balíček
        [few] Importuji { $count } balíčky
       *[other] Importuji { $count } balíčků
    }
upload-bytes-of = { $sent } z { $total }
upload-rate = { $size }/s
upload-waiting = Čeká
upload-unpacking = Nahráno, teď se rozbaluje
upload-already = Už je v knihovně
upload-stopped = Zastaveno
upload-files-of = { $done } z { $total } souborů
upload-resumed = obnoveno, { $size } už bylo na serveru
upload-skipped = Přesně tyto soubory už byly importovány jako „{ $name }“, takže byly přeskočeny.
upload-error = { $error }. Váš postup zůstává uložený: zkuste to znovu, nebo stejné soubory přetáhněte znovu později.
upload-retry = Zkusit znovu
upload-import-again = Přesto importovat znovu
upload-dismiss = Skrýt
upload-cancel = Zrušit

## PacksView.vue

packs-rename-failed = Nepodařilo se přejmenovat: { $error }
packs-collection-deleted = Kolekce { $name } byla smazána
packs-delete-failed = Nepodařilo se smazat: { $error }
packs-status-uploading = Nahrává se (zatím přijaté soubory)
packs-status-ingesting = Rozbalování a štítkování
packs-status-analyzing = LLM štítkuje
packs-status-ready = Připraveno
packs-status-failed = Selhalo
packs-retag-failed = Nové štítkování se nepodařilo spustit: { $error }
packs-read-files-failed = Soubory balíčku se nepodařilo přečíst: { $error }
packs-context-load-failed = Údaje z obchodu pro balíček se nepodařilo načíst: { $error }
packs-undo-store-failed = Stránku obchodu se nepodařilo odebrat: { $error }
packs-context-save-failed = Údaje z obchodu se nepodařilo uložit: { $error }
packs-unreal-load-failed = Stav převodu z Unrealu se nepodařilo načíst: { $error }
packs-unreal-discard-title = Zahodit originály?
packs-unreal-discard-body = Původní soubory .uasset v { $name } se po převodu smažou. Data, která zná jen Unreal (například grafy materiálů), se trvale ztratí.
packs-unreal-discard-confirm = Převést a zahodit
packs-unreal-convert-failed = Převod se nepodařilo spustit: { $error }
packs-unreal-fact-converted = Převedeno z Unrealu: { $done }
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } soubor Unrealu
        [few] { $left } soubory Unrealu
       *[other] { $left } souborů Unrealu
    }
packs-unreal-fact-both = Převedeno z Unrealu: { $done }, stále souborů Unrealu: { $left }
packs-thumbs-canceled = Zrušeno po { $done } z { $total }
packs-thumbs-drew =
    { $count ->
        [one] Vykreslen { $count } model
        [few] Vykresleny { $count } modely
       *[other] Vykresleno { $count } modelů
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: nahrazena { $improved } miniatura.
        [few] { $lead }: nahrazeny { $improved } miniatury.
       *[other] { $lead }: nahrazeno { $improved } miniatur.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: nahrazena { $improved } miniatura, nepodařilo se vykreslit: { $failed }.
        [few] { $lead }: nahrazeny { $improved } miniatury, nepodařilo se vykreslit: { $failed }.
       *[other] { $lead }: nahrazeno { $improved } miniatur, nepodařilo se vykreslit: { $failed }.
    }
packs-archived =
    { $count ->
        [one] Archivováno: { $name } a jeho { $count } asset
        [few] Archivováno: { $name } a jeho { $count } assety
       *[other] Archivováno: { $name } a jeho { $count } assetů
    }
packs-blobs-bad =
    { $count ->
        [one] { $count } soubor neprošel poslední kontrolou integrity.
        [few] { $count } soubory neprošly poslední kontrolou integrity.
       *[other] { $count } souborů neprošlo poslední kontrolou integrity.
    }
packs-collections = Kolekce
packs-browse-named = Procházet { $name }
packs-collection-name = Název kolekce
packs-save = Uložit
packs-cancel = Zrušit
packs-assets =
    { $count ->
        [one] { $count } asset
        [few] { $count } assety
       *[other] { $count } assetů
    }
packs-updated = Aktualizováno { $date }
packs-browse = Procházet
packs-collection-download = Stáhnout (.zip)
packs-collection-credits = Poděkování
packs-collection-credits-title = Kdo vytvořil balíčky, ze kterých tyto assety pocházejí, jejich licence a řádky s poděkováním, o které žádají
packs-rename = Přejmenovat
packs-delete = Smazat
packs-packs = Balíčky
packs-empty = Zatím žádné balíčky. Přidáte ho přetažením zipu nebo složky kamkoli na tuto stránku.
packs-find = Najít balíček
packs-find-placeholder = Najít balíček podle názvu, obchodu, licence nebo štítku
packs-sort = Pořadí
packs-sort-new = Od nejnovějších
packs-sort-old = Od nejstarších
packs-sort-name = Podle názvu
packs-sort-size = Od největších
packs-sort-assets = Podle počtu assetů
packs-with-kind = Balíčky obsahující
packs-state = Stav
packs-state-any = Jakýkoli stav
packs-state-working = Zpracovávají se
packs-state-failed = Selhaly
packs-state-untagged = Zatím bez popisu
packs-store = Obchod
packs-store-any = Jakýkoli obchod
packs-store-from = Z { $store }
packs-store-none = Bez stránky v obchodě
packs-shown-of = { $shown } z { $total }
packs-clear = Vymazat
packs-no-match = Žádné balíčky neodpovídají.
packs-clear-all = Vymazat hledání a filtry
packs-pack-name = Název balíčku
packs-thumbs-drawing = Vykresluji miniatury v tomto prohlížeči…
packs-thumbs-drawing-count = Vykresluji miniatury v tomto prohlížeči: { $done } z { $total }…
packs-thumbs-stopped = Zastaveno: { $error }
packs-thumbs-none = Každý model tady už má miniaturu vykreslenou se všemi texturami.
packs-dismiss = Skrýt
packs-license = Licence: { $license }
packs-license-tier = Licence: { $license } ({ $tier })
packs-license-not-found = nenalezena
packs-from-before = Z
packs-from-after = { "" }
packs-added = Přidáno { $date }
packs-download = Stáhnout balíček (.zip)
packs-store-details = Údaje z obchodu
packs-more-named = Další akce pro { $name }
packs-more = Další
packs-rename-more = Přejmenovat…
packs-retag-rules = Znovu spustit offline štítkovač
packs-retag-llm = Znovu spustit štítkovač LLM
packs-make-thumbs = Vytvořit miniatury v tomto prohlížeči
packs-make-thumbs-title = Otevře každý model postupně ve skrytém 3D prohlížeči a jeho obrázek použije jako miniaturu tam, kde je lepší než ta ze serveru
packs-make-thumbs-title-desktop = Otevře každý model postupně ve skrytém 3D prohlížeči a jeho obrázek použije jako miniaturu tam, kde je lepší než ta, kterou vytvořil Hoard
packs-read-files = Znovu přečíst soubory balíčku
packs-credits = Stáhnout poděkování
packs-credits-title = CREDITS.md: kdo balíček vytvořil, jeho licence a řádek s poděkováním, o který žádá
packs-unreal-open = Převést obsah z Unrealu…
packs-archive = Archivovat balíček
packs-unreal-intro = Hoard čte soubory Unrealu sám a jejich obsah ukládá jako běžné soubory: textury jako PNG nebo EXR, zvuky jako WAV, materiály jako JSON. Převedený soubor nahradí původní soubor Unrealu a zachová jeho štítky, odkazy a kolekce.
packs-unreal-converted =
    { $count ->
        [one] převeden
        [few] převedeny
       *[other] převedeno
    }
packs-unreal-converted-kept =
    { $count ->
        [one] převeden, u { $kept } ponechán originál
        [few] převedeny, u { $kept } ponechán originál
       *[other] převedeno, u { $kept } ponechán originál
    }
packs-unreal-pending =
    { $count ->
        [one] zatím neprohlédnut
        [few] zatím neprohlédnuty
       *[other] zatím neprohlédnuto
    }
packs-unreal-needs-editor =
    { $count ->
        [one] čeká na Unreal Editor
        [few] čekají na Unreal Editor
       *[other] čeká na Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] bez běžného formátu
        [few] bez běžného formátu
       *[other] bez běžného formátu
    }
packs-unreal-failed =
    { $count ->
        [one] selhal
        [few] selhaly
       *[other] selhalo
    }
packs-unreal-reasons = Proč soubory zůstávají
packs-unreal-editor-before = Meshe a animace potřebují Unreal Editor. Na počítači s nainstalovaným Unrealem spusťte
packs-unreal-editor-command = hoard unreal-worker --editor <cesta k UnrealEditor-Cmd>
packs-unreal-editor-after = ; vyexportuje je jako FBX a výsledky nahraje sem.
packs-unreal-discard = Po převodu zahodit původní soubory .uasset
packs-unreal-discard-hint = Uvolní místo. Data, která zná jen Unreal (například grafy materiálů), se ztratí.
packs-unreal-drop = Smazat soubory bez běžného formátu
packs-unreal-drop-hint = Blueprinty, levely a další assety, které zná jen engine.
packs-unreal-convert = Převést
packs-context-hint-before = Odkud balíček pochází a co o něm říká prodejce. Štítkovač LLM to čte u každého souboru, takže popis z obchodu jako „120 kouzel ledu a ohně“ mu pomůže pojmenovat
packs-context-hint-after = .
packs-store-auto = Nalezeno automaticky: balíček sám uvádí tuto stránku.
packs-store-not-this = Tohle není ono
packs-lookup-label = Hledat v obchodech
packs-lookup-placeholder = Název balíčku v obchodě
packs-lookup-searching = Hledám…
packs-lookup-search = Hledat v obchodech
packs-lookup-hide = Skrýt
packs-lookup-clues = Ze souborů balíčku: { $clues }.
packs-lookup-also = Hledáno také: { $queries }.
packs-lookup-quoted = „{ $query }“
packs-lookup-nothing = Na Fab, itch.io, Kenney ani OpenGameArt nic podobného není. Zkuste méně slov nebo jiná slova.
packs-lookup-use = Použít
packs-lookup-searched = Hledáno { $date }.
packs-lookup-again = Hledat znovu
packs-lookup-again-after = .
packs-lookup-unreachable = Tentokrát se nepodařilo spojit s { $stores }.
packs-lookup-also-look = Podívejte se také na
packs-lookup-links-and = { " " }a{ " " }
packs-lookup-browser-only = (lze je prohledávat jen ve vašem prohlížeči); adresu stránky vložte níže.
packs-lookup-open = Najít v obchodech…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Prodejce
packs-seller-placeholder = Kdo ho vytvořil
packs-license-label = Licence
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Úroveň licence
packs-license-tier-placeholder = Personal, Professional…
packs-store-page = Stránka v obchodě
packs-store-text = Popis z obchodu
packs-store-text-placeholder = Vložte text nabídky, nebo nechte prázdné a načte se ze stránky obchodu.
packs-store-text-fetch = Uložení načte popis ze stránky obchodu.
packs-readme = README nalezené v balíčku
packs-save-retag = Uložit a znovu spustit štítkovač LLM

## AssetDetail.vue

asset-lazy-3d-viewer = 3D prohlížeč
asset-lazy-text = text
asset-lazy-text-viewer = prohlížeč textu
asset-lazy-image-viewer = prohlížeč obrázků
asset-lazy-material-preview = náhled materiálu
asset-lazy-material-viewer = prohlížeč materiálů
asset-lazy-sprite-player = přehrávač spritů
asset-lazy-sheet-view = prohlížeč listů spritů
asset-rel-texture = textura
asset-rel-buffer = data geometrie
asset-rel-material = soubor materiálu
asset-rel-animation = animace
asset-rel-library = propojený soubor
asset-rel-guess = nejspíš patří k
asset-rel-sidecar = údaje z obchodu
asset-rel-variant = varianta
asset-rel-sprite = sprite
asset-role-map = mapa typu { $role }
asset-maps-count =
    { $count ->
        [one] { $count } mapa
        [few] { $count } mapy
       *[other] { $count } map
    }
asset-needs = Potřebuje
asset-animations = Animace
asset-sprites = Sprity
asset-used-by = Používá ho
asset-loose-sprites = Také jako samostatné soubory
asset-in-sheet = Ve spritesheetu
asset-other-formats = Další formáty
asset-shown-as = V knihovně zobrazeno jako
asset-conversion-failed = převod selhal
asset-as-converting = Převádím…
asset-as-converting-percent = Převádím… { $percent } %
asset-as-queued = Ve frontě…
asset-download = Stáhnout
asset-as-label = Stáhnout jako
asset-as-original = { $format } (původní formát)
asset-as-pack-files = Vlastní soubory balíčku
asset-as-size = Velikost
asset-as-full-size = Plná velikost
asset-as-up-to = Až { $size } px
asset-as-hint = Vyberte velikost nebo jiný formát
asset-meta-size = Velikost
asset-meta-length = Délka
asset-meta-sample-rate = Vzorkovací frekvence
asset-meta-channels = Kanály
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bitová hloubka
asset-meta-bits = { $bits } bitů
asset-meta-level = Úroveň
asset-meta-level-value = špička { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = špička { $peak } dBFS, RMS { $rms } dBFS, přebuzení
asset-meta-loop = Smyčka
asset-meta-loop-value = { $start } až { $end }
asset-meta-playback = Přehrávání
asset-meta-one-shot = Jednorázově
asset-meta-envelope = Obálka
asset-meta-envelope-times = náběh { $attack }, doznívání { $tail }
asset-meta-envelope-hit = úder
asset-meta-envelope-swell = zesilující
asset-meta-envelope-sustained = držený
asset-meta-tone = Zabarvení
asset-meta-tone-value = jas { $khz } kHz, šum { $noise } %
asset-meta-pitch = Výška tónu
asset-meta-pitch-rises =
    { $octaves ->
        [one] stoupá o { $octaves } oktávu
        [few] stoupá o { $octaves } oktávy
        [many] stoupá o { $octaves } oktávy
       *[other] stoupá o { $octaves } oktáv
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] klesá o { $octaves } oktávu
        [few] klesá o { $octaves } oktávy
        [many] klesá o { $octaves } oktávy
       *[other] klesá o { $octaves } oktáv
    }
asset-meta-loops-cleanly = Plynule se opakuje
asset-meta-tempo = Tempo
asset-meta-root-note = Základní tón
asset-meta-cue-markers = Značky
asset-meta-library = Knihovna
asset-meta-keywords = Klíčová slova
asset-meta-title = Název
asset-meta-artist = Interpret
asset-meta-album = Album
asset-meta-genre = Žánr
asset-meta-generator = Generátor
asset-meta-everything-with = Zobrazit vše se štítkem { $tag }
asset-meta-saved-with = Uloženo v
asset-meta-color-mode = Barevný režim
asset-meta-layers = Vrstvy
asset-meta-range = Rozsah
asset-meta-hdr = HDR (náhled je tónově mapovaný)
asset-meta-format = Formát
asset-meta-large-document = Velký dokument (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Vytvořeno generativní AI
asset-meta-ai-edited = Upraveno generativní AI
asset-meta-ai-how-metadata = metadata
asset-meta-ai-how-generator = nastavení generátoru
asset-meta-ai-how-iptc = typ digitálního zdroje IPTC
asset-meta-ai-says = { $made } (zdroj: { $how })
asset-meta-ai-by-says = { $made } ({ $by }) (zdroj: { $how })
asset-meta-provenance = Původ
asset-meta-c2pa = Má Content Credentials C2PA
asset-meta-compression = Komprese
asset-meta-note = Poznámka
asset-meta-maps = Mapy
asset-meta-sizes =
    { $count ->
        [one] Velikost
        [few] Velikosti
       *[other] Velikosti
    }
asset-meta-triangles = Trojúhelníky
asset-meta-vertices = Vrcholy
asset-meta-meshes = Meshe
asset-meta-materials = Materiály
asset-meta-animations = Animace
asset-meta-skins = Skiny
asset-meta-unreal-asset = Asset Unrealu
asset-meta-unreal-saved-with = { $class }, uloženo v UE { $engine }
asset-meta-package = Soubor Unrealu
asset-meta-cooked = Cooked (herní build): náhledy a zdroje jsou odstraněné
asset-meta-tiling = Opakování
asset-meta-seamless = Bezešvé v obou osách
asset-meta-analysis-error = Tento soubor se nepodařilo celý přečíst: { $error }
asset-copy-blocked = Nepodařilo se zkopírovat: prohlížeč to zablokoval
asset-save-error = Nepodařilo se uložit: { $error }
asset-tag-removed = Odebráno: { $tag }
asset-undo = Zpět
asset-play-error = Tento soubor se nepodařilo přehrát: { $error }
asset-copied-hash = Hash zkopírován
asset-copied-toml = Toml zkopírován
asset-copied-id = ID zkopírováno
asset-copied-link = Odkaz zkopírován
asset-origin-before = Tento soubor byl původně
asset-origin-class-engine = , { $class } uložený v UE { $engine }.
asset-origin-class = , { $class }.
asset-origin-engine = { " " }uložený v UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor ho vyexportoval { $date }.
asset-origin-editor-notes = Unreal Editor ho vyexportoval { $date } ({ $notes }).
asset-origin-hoard = Hoard ho vyčetl ze souboru Unrealu { $date }.
asset-origin-hoard-notes = Hoard ho vyčetl ze souboru Unrealu { $date } ({ $notes }).
asset-origin-download = Stáhnout původní .{ $ext }
asset-origin-discarded = Původní soubor Unrealu byl po převodu zahozen.
asset-details = Podrobnosti assetu
asset-in-pack = v
asset-previous = Předchozí asset
asset-next = Další asset
asset-favorite = Oblíbené
asset-favorite-on-title = V oblíbených (F ho odebere)
asset-favorite-off-title = Přidat do oblíbených (F)
asset-share = Sdílet
asset-share-title = Sdílet odkaz na tento asset (kdo ho otevře, se nejdřív přihlásí do Hoardu)
asset-layout-panel = Zobrazit v bočním panelu
asset-layout-page = Zobrazit na celé stránce
asset-layout-page-title = Zobrazit na celé stránce: velký náhled vedle podrobností
asset-close = Zavřít podrobnosti
asset-rename-title = Kliknutím přejmenujete
asset-name = Název
asset-views-alt = Model ze čtyř stran
asset-inspect-aria = Prohlédnout { $name } na celé obrazovce
asset-inspect-title = Prohlédnout: přiblížení, dlaždice, kanály
asset-playback-position = Pozice přehrávání
asset-playback-value = { $current } z { $duration }
asset-pause = Pozastavit
asset-play = Přehrát
asset-waveform-drawing = Vykresluji průběh…
asset-waveform-skipped = U tak velkých souborů se průběh nezobrazuje
asset-play-animation = Přehrát jako animaci
asset-show-views = Zobrazit čtyřpohledový list, který viděl štítkovač
asset-palette = Barvy na obrázku
asset-swatch-title = { $hex }, { $percent } % obrázku. Najít assety s touto barvou
asset-swatch-aria = Najít assety s barvou { $hex }, { $percent } % tohoto obrázku
asset-more-like-this = Další podobné
asset-desc-label = Váš popis
asset-desc-placeholder = Co to je, kde byste to použili…
asset-save = Uložit
asset-cancel = Zrušit
asset-desc-family = Uloží se do všech variant ({ $count })
asset-desc-edit = Upravit popis
asset-desc-write = Napsat popis
asset-desc-ai = Napsala AI
asset-desc-file = Ze souboru
asset-desc-store-before = Ze
asset-desc-store-page = stránky { $store }
asset-desc-store-page-unnamed = stránky obchodu
asset-desc-store-after = (celý balíček)
asset-less = Méně
asset-more = Více
asset-tags = Štítky
asset-tag-add-label = Přidat štítek
asset-tag-add-placeholder = Přidejte štítek, např. mood:tense
asset-tag-add = Přidat
asset-apply-family = Použít změny štítků, kategorie a popisu na všechny varianty ({ $count })
asset-collections = Kolekce
asset-collection-remove = Odebrat z { $name }
asset-collection-add-another = Přidat do další
asset-collection-add = Přidat do kolekce
asset-used-in = Používá se v
asset-used-in-title = Zobrazit, co má { $name }
asset-used-in-synced = synchronizováno { $when }
asset-maps = Mapy
asset-show-more = Zobrazit další ({ $count })
asset-materials-heading =
    { $count ->
        [one] Materiál
        [few] Materiály
       *[other] Materiály
    }
asset-material-maps-aria = { $name }: mapy
asset-variants = Varianty (verzí: { $count })
asset-copies = Identické kopie
asset-copies-note = Stejné bajty se ukládají jen jednou; tento soubor obsahují i tyto balíčky.
asset-unreal-converted = Převedeno z Unrealu
asset-rule-converted = Převedeno pravidlem
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Toto { $format } vytvořilo pravidlo převodu { $date } ze souboru
asset-rule-kept-until = Originál se uchovává do { $date }.
asset-rule-put-back = Vrátit originál
asset-rule-put-back-title = Vrátit { $file }?
asset-rule-put-back-body = Převedený soubor se smaže a jeho místo zaujme originál i se svými štítky, kolekcemi a odkazy. Pravidla převodu ho od té doby nechají být.
asset-rule-put-back-done = { $file } je zpět
asset-rule-let-go = Originál byl smazán, jakmile uplynula doba, po kterou se podle Nastavení originály uchovávají.
asset-unreal-package = Ze souboru Unrealu
asset-ue-needs-editor-before = Čeká na Unreal Editor: { $reason }. Na počítači s nainstalovaným Unrealem spusťte
asset-ue-needs-editor-after = a vrátí se jako FBX.
asset-ue-editor-path = cesta k UnrealEditor-Cmd
asset-ue-unsupported = Ponecháno jako soubor Unrealu: { $reason }.
asset-ue-failed = Převod selhal: { $reason }
asset-recovered = Původní .{ $ext } byl uložen uvnitř tohoto souboru Unrealu a Hoard ho vytáhl.
asset-recovered-download = Stáhnout .{ $ext }
asset-ue-meshes-before = Meshe a animace jsou uložené ve vlastním formátu Unrealu. Převeďte balíček (stránka balíčku, Další) a pak spusťte
asset-ue-meshes-after = vedle instalace Unreal Editoru a získáte FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } štítek registru assetů
        [few] { $count } štítky registru assetů
       *[other] { $count } štítků registru assetů
    }
asset-file = Soubor
asset-bytes = Bajty
asset-source = Zdroj
asset-path = Cesta
asset-path-open-folder = Co dalšího je ve složce { $folder }
asset-download-maps =
    { $count ->
        [one] Stáhnout { $count } mapu
        [few] Stáhnout všechny { $count } mapy
       *[other] Stáhnout všech { $count } map
    }
asset-download-linked =
    { $count ->
        [one] Stáhnout s { $count } propojeným souborem
        [few] Stáhnout s { $count } propojenými soubory
       *[other] Stáhnout s { $count } propojenými soubory
    }
asset-download-only-title = Jen tento soubor, bez souborů, které používá
asset-download-only = Jen tento soubor
asset-include-animations = Zahrnout animace do .zip
asset-copy-toml = Kopírovat záznam do hoard.toml
asset-copy-id = Kopírovat ID
asset-teach = Naučit Hoard tento soubor
asset-teach-title = Popište tento soubor obchodu, aby ho Hoard uměl přečíst (Nastavení → Soubory obchodů)
asset-archive = Archivovat
asset-archive-title = Mimo hledání, dokud ho v Archivu neobnovíte nebo trvale nesmažete

## ImageViewer.vue

image-channel-red = Červený kanál
image-channel-green = Zelený kanál
image-channel-blue = Modrý kanál
image-channel-alpha = Alfa kanál
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Šachovnicové pozadí (K přepíná)
image-bg-black-title = Černé pozadí (K přepíná)
image-bg-gray-title = Šedé pozadí (K přepíná)
image-bg-white-title = Bílé pozadí (K přepíná)
image-bg-checker = šachovnicové pozadí
image-bg-black = černé pozadí
image-bg-gray = šedé pozadí
image-bg-white = bílé pozadí
image-inspect-aria = Prohlédnout { $name }
image-preview-size = náhled { $width }×{ $height }, { $bytes }
image-preview = náhled
image-resolution = Rozlišení
image-full-title = Načíst celý obrázek { $width }×{ $height } (F); stejně tak přiblížení nad rámec náhledu
image-full-loading = Načítám plné rozlišení…
image-full-failed = Plné rozlišení se nenačetlo: zkusit znovu
image-full = Plné rozlišení
image-zoom = Přiblížení
image-zoom-out-title = Oddálit (−)
image-zoom-out = Oddálit
image-zoom-in-title = Přiblížit (+)
image-zoom-in = Přiblížit
image-fit-title = Přizpůsobit (0)
image-fit = Přizpůsobit
image-actual-title = Skutečné pixely (1)
image-view = Zobrazení
image-tile-title = Dlaždice 3×3 pro kontrolu švů (T)
image-tile = Dlaždice
image-channels = Kanály
image-close = Zavřít (Esc)
image-failed = Tento obrázek se nepodařilo načíst.
image-loading = Načítám…
image-key-gestures = Kolečkem nebo sevřením přiblížíte · tažením posunete · dvojklikem přiblížíte nebo přizpůsobíte
image-key-tile = dlaždice
image-key-background = pozadí
image-key-full = plné rozlišení
image-key-channels = kanály

## SpritePlayer.vue

sprite-frame-aria = { $name }, snímek { $frame } z { $count }
sprite-failed = Tento soubor nelze přehrát.
sprite-loading = Načítám snímky…
sprite-previous = Předchozí snímek
sprite-next = Další snímek
sprite-pause = Pozastavit animaci
sprite-play = Přehrát animaci
sprite-animation = Animace
sprite-all-frames = Všechny snímky
sprite-grid-title = Jak je list rozřezán na snímky
sprite-columns = Sloupce
sprite-rows = Řádky
sprite-fps-aria = Snímků za sekundu
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
        [few] { $count } sprity
        [many] { $count } spritu
       *[other] { $count } spritů
    }
sheet-pages =
    { $count ->
        [one] { $count } stránka
        [few] { $count } stránky
        [many] { $count } stránky
       *[other] { $count } stránek
    }
sheet-animations =
    { $count ->
        [one] { $count } animace
        [few] { $count } animace
        [many] { $count } animace
       *[other] { $count } animací
    }
# The box that narrows the list of sprites by name.
sheet-find = Najít sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Zobrazeno { $shown } z { $total }
sheet-no-match = Žádný sprite tohle v názvu nemá.
sheet-failed = Tento list nelze přečíst.
sheet-loading = Načítám list…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Vlastní formát Hoardu: stránky tak, jak jsou, sprity libovolně otočené a oříznuté, s animacemi a délkou každého snímku.
sheet-note-texturepacker = Sprity otočené po směru hodinových ručiček a oříznuté, s animacemi podle názvu, tak jak je čtou PixiJS a Phaser.
sheet-note-clockwise = Sprity otočené po směru hodinových ručiček a oříznuté. Animace v něm být nemohou: snímky se řídí svými číslovanými názvy.
sheet-note-libgdx = Sprity otočené proti směru hodinových ručiček a oříznuté. Animace v něm být nemohou: snímky se řídí svými číslovanými názvy.
sheet-note-godot-frames = Sprity se nikdy neotáčejí a oříznutá část zůstane jako okraje, s animacemi a délkou každého snímku.
sheet-note-godot-tileset = Dlaždice se nikdy neotáčejí ani neořezávají, v mřížce o velikosti dlaždice; obrázky, které už v mřížce jsou, zůstanou, jak jsou.
sheet-note-grid = Každý sprite vzpřímeně na celém svém plátně, v buňkách stejné velikosti, pro cokoli, co list rozřezává do mřížky.
sheet-note-frames = Každý sprite vzpřímeně na celém svém plátně, každý jako PNG pojmenované po něm.

## TextViewer.vue, TextPreview.vue a text.ts

text-lang-config = Konfigurace
text-lang-table = Tabulka
text-lang-code = Kód
text-lang-script = Skript
text-lang-text = Text
text-copy-blocked = Nepodařilo se zkopírovat: prohlížeč to zablokoval
text-read-aria = Číst { $name }
text-lines =
    { $count ->
        [one] { $count } řádek
        [few] { $count } řádky
       *[other] { $count } řádků
    }
text-show-as = Zobrazit jako
text-tree-title = Rozbalovací strom (T)
text-tree = Strom
text-pretty-title = Nově odsazené (P)
text-pretty = Formátované
text-raw-title = Tak, jak je napsáno (R)
text-raw = Původní
text-document-title = Jako dokument (D)
text-document = Dokument
text-source-title = Markdown tak, jak je napsaný (R)
text-source = Zdroj
text-fold = Sbalit
text-expand-all-title = Rozbalit vše
text-expand-all = Rozbalit vše
text-collapse-all-title = Sbalit vše
text-collapse-all = Sbalit vše
text-group-text = Text
text-wrap-title = Zalamovat dlouhé řádky (W)
text-wrap = Zalamovat
text-copy-title = Kopírovat text (C)
text-copied = Zkopírováno
text-copy = Kopírovat
text-download-title = Stáhnout soubor
text-download = Stáhnout
text-close = Zavřít (Esc)
text-truncated = Zobrazuje se prvních { $shown } z { $total }.
text-truncated-json = Zobrazuje se prvních { $shown } z { $total } (strom potřebuje celý soubor).
text-show-all = Zobrazit vše
text-json-error = Tento JSON nejde zpracovat: { $error }
text-contents-aria = Obsah { $name }
text-loading = Načítám…
text-load-error = Tento soubor se nepodařilo načíst: { $error }
text-binary = Tento soubor je binární, ne textový, takže tu není co číst. Stáhněte ho a otevřete v nástroji, který k němu patří.
text-key-tree = strom
text-key-pretty = formátované
text-key-raw = původní
text-key-document = dokument
text-key-source = zdroj
text-key-wrap = zalamování
text-key-copy = kopírovat
text-key-find = hledat
text-key-close = zavřít
text-preview-aria = Číst { $name } na celé obrazovce
text-preview-read = Číst

## JsonTree.vue a JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } položka
        [few] { $count } položky
       *[other] { $count } položek
    }
json-keys =
    { $count ->
        [one] { $count } klíč
        [few] { $count } klíče
       *[other] { $count } klíčů
    }
json-fold = Sbalit
json-unfold = Rozbalit
json-unfold-summary = Rozbalit ({ $summary })
json-show-more =
    { $count ->
        [one] Zobrazit další { $count }
        [few] Zobrazit další { $count }
       *[other] Zobrazit dalších { $count }
    }
json-left = ze zbývajících { $count }
json-picker-more =
    { $count ->
        [one] … a { $count } další
        [few] … a { $count } další
       *[other] … a { $count } dalších
    }

## 3D prohlížeče modelů

model-viewer-failed = 3D prohlížeč se nenačetl ({ $error }); může pomoct znovu načíst stránku
model-loading-viewer = Načítám 3D prohlížeč…
model-loading = Načítám model…
model-show-failed = Tento model se nepodařilo zobrazit: { $error }
model-preview-failed = Náhled tohoto modelu se nepodařilo zobrazit: { $error }
model-error-converting = server tento model ještě převádí
model-error-converting-desktop = Hoard tento model ještě převádí
model-error-empty-collada = prázdný soubor Collada
model-error-no-loader = Prohlížeč nemá zavaděč pro .{ $ext }
model-dialog = Zobrazit { $name } ve 3D
model-title-stats =
    { $triangles } troj. · { $meshes ->
        [one] { $meshes } mesh
        [few] { $meshes } meshe
       *[other] { $meshes } meshů
    } · { $materials ->
        [one] { $materials } materiál
        [few] { $materials } materiály
       *[other] { $materials } materiálů
    }
model-triangles =
    { $count ->
        [one] { $count } trojúhelník
        [few] { $count } trojúhelníky
       *[other] { $count } trojúhelníků
    }
model-group-camera = Kamera
model-group-shading = Stínování
model-group-background = Pozadí
model-group-animation = Animace
model-group-view = Zobrazení
model-group-show = Zobrazit
model-frame-title = Zaměřit model (F)
model-frame = Zaměřit
model-reset-title = Zpět na výchozí pohled (R)
model-reset = Obnovit
model-turn-title = Pomalu otáčet (A)
model-turn = Otáčet
model-textures-title = Textury (T)
model-textures = Textury
model-full-res-title = Textury v plném rozlišení (H): každý soubor textury tak, jak je, jakkoli velký; vypnuto: odlehčené náhledy o nejvýše 2048 px
model-full-res = Plné rozl.
model-light-title = Osvětlení prostředím (E)
model-light = Světlo
model-grid-title = Mřížka podlahy (G)
model-grid = Mřížka
model-bg-checker = Šachovnicové pozadí
model-bg-dark = Tmavé pozadí
model-bg-gray = Šedé pozadí
model-bg-light = Světlé pozadí
model-bg-env = Pozadí z prostředí
model-bg-title = { $background } (K přepíná)
model-pause-title = Pozastavit (mezerník)
model-play-title = Přehrát (mezerník)
model-pause = Pozastavit
model-play = Přehrát
model-animation-clip = Klip animace
model-animation = Animace
model-plays-on = Přehrát na
model-close-title = Zavřít (Esc)
model-full-screen = Celá obrazovka
model-open-full-screen = Otevřít 3D prohlížeč na celé obrazovce
model-menu-fold = Skrýt nabídku
model-menu-show = Zobrazit nabídku
model-view-prefix = Zobrazení:{ " " }
model-menu-render = Render
model-menu-maps = Mapy
model-menu-inspect = Prohlídka
model-mode-final = Výsledný render
model-mode-raw = Surový render
model-mode-color = Základní barva
model-mode-roughness = Drsnost
model-mode-metalness = Kovovost
model-mode-normal = Normála
model-mode-ao = Okluze
model-mode-matcap = Matcap
model-mode-uv = Kontrola UV
model-mode-geometry = Geometrie
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Drátový model (W), přes jakékoli zobrazení
model-wireframe = Drátový model
model-info-title = Informace o scéně (I)
model-info = Informace o scéně
model-info-close = Zavřít informace o scéně
model-info-close-title = Zavřít (I)
model-info-geometries = Geometrie
model-info-materials = Materiály
model-info-textures = Textury
model-info-triangles = Trojúhelníky
model-info-vertices = Vrcholy
model-info-meters = glTF je v metrech
model-info-no-units = Tento formát neuvádí, v jakých je jednotkách
model-info-height = Výška
model-info-length = Délka
model-info-width = Šířka
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } jednotka
        [few] { NUMBER($n, maximumSignificantDigits: 3) } jednotky
        [many] { NUMBER($n, maximumSignificantDigits: 3) } jednotky
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } jednotek
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Textura nenalezena: { $name }
model-missing-some =
    { $count ->
        [one] { $count } textura nenalezena: { $names }
        [few] { $count } textury nenalezeny: { $names }
       *[other] { $count } textur nenalezeno: { $names }
    }
model-missing-more =
    { $count ->
        [one] { $count } textura nenalezena: { $names }…
        [few] { $count } textury nenalezeny: { $names }…
       *[other] { $count } textur nenalezeno: { $names }…
    }
model-missing-count =
    { $count ->
        [one] { $count } textura nenalezena
        [few] { $count } textury nenalezeny
       *[other] { $count } textur nenalezeno
    }
model-guessed = Textury přiřazené podle názvu; soubor žádné neuvádí.
model-group-facing = Natočení
model-facing-turn = Otočit o čtvrt
model-facing-turn-title = Otočit model o čtvrt otáčky proti směru hodinových ručiček při pohledu shora
model-facing-tip = Naklonit o čtvrt
model-facing-tip-title = Naklonit vršek modelu o čtvrt otáčky dopředu, aby ležící model stál
model-facing-save = Použít pro balíček
model-facing-save-title = Takto otočit všechny modely v balíčku { $pack }. Jejich miniatury se vykreslí znovu; stahování zůstanou tak, jak byla vytvořena.
model-facing-undo = Zpět
model-facing-undo-title = Zpět k tomu, jak jsou modely balíčku otočené
model-facing-saved = Všechny modely v balíčku { $pack } jsou teď natočené takto. Jejich miniatury se vykreslují znovu.
model-facing-failed = Modely balíčku se nepodařilo otočit: { $error }
model-keys-mouse = Tažením otáčíte · tažením pravým tlačítkem posouváte · kolečkem přibližujete · dvojklikem zaměříte
model-key-frame = zaměřit
model-key-reset = obnovit
model-key-view = zobrazení
model-key-wireframe = drátový model
model-key-info = informace o scéně
model-key-textures = textury
model-key-full-res = textury v plném rozlišení
model-key-light = světlo
model-key-grid = mřížka
model-key-background = pozadí
model-key-play = přehrát

## Materiály

material-role-base-color = Barva
material-role-normal = Normála
material-role-normal-dx = Normála (DirectX)
material-role-roughness = Drsnost
material-role-gloss = Lesk
material-role-metalness = Kovovost
material-role-ao = Ambientní okluze
material-role-orm = Okluze, drsnost, kov
material-role-mra = Kov, drsnost, okluze
material-role-rma = Drsnost, kov, okluze
material-role-height = Výška
material-role-bump = Bump
material-role-cavity = Dutiny
material-role-specular = Spekulární odraz
material-role-opacity = Neprůhlednost
material-role-emissive = Emise
material-role-translucency = Průsvitnost
material-role-base-color-name = barva
material-role-normal-name = normála
material-role-normal-dx-name = normála (DirectX)
material-role-roughness-name = drsnost
material-role-gloss-name = lesk
material-role-metalness-name = kovovost
material-role-ao-name = ambientní okluze
material-role-orm-name = okluze, drsnost, kov
material-role-mra-name = kov, drsnost, okluze
material-role-rma-name = drsnost, kov, okluze
material-role-height-name = výška
material-role-bump-name = bump
material-role-cavity-name = dutiny
material-role-specular-name = spekulární odraz
material-role-opacity-name = neprůhlednost
material-role-emissive-name = emise
material-role-translucency-name = průsvitnost
material-maps =
    { $count ->
        [one] { $count } mapa
        [few] { $count } mapy
       *[other] { $count } map
    }
material-dialog = Zobrazit materiál { $name }
material-up-to = až { $size }
material-repeat-size = { $size } na opakování
material-shape-sphere = Koule
material-shape-cube = Krychle
material-shape-plane = Rovina
material-shape-title = { $shape } ({ $key })
material-group-shape = Tvar
material-group-tiling = Opakování
material-real-title = Skutečná velikost: jedno opakování je { $size }, koule má průměr 2 m (T přepíná)
material-repeat-title = Opakovat { $n }× (T přepíná)
material-real = Skutečná
material-repeat = { $n }×
material-height-title = Jak moc výšková mapa vytlačuje povrch
material-height = Výška
material-height-strength = Síla výšky
material-loading = Načítám materiál…
material-show-failed = Tento materiál se nepodařilo zobrazit: { $error }
material-preview-failed = Náhled tohoto materiálu se nepodařilo zobrazit: { $error }
material-group-maps = Mapy
material-map-show = Zobrazit mapu typu { $map }
material-map-hide = Skrýt mapu typu { $map }
material-map-unused = Náhled ji nepoužívá
material-open-full-screen = Otevřít prohlížeč materiálů na celé obrazovce
material-keys-mouse = Tažením otáčíte · tažením pravým tlačítkem posouváte · kolečkem přibližujete · kliknutím na mapu ji vypnete
material-key-shape = tvar
material-key-tiling = opakování
material-key-turn = otáčení

## theme.ts

theme-slate = Břidlice
theme-paper = Papír
theme-linen = Len
theme-sage = Šalvěj
theme-lilac = Šeřík
theme-blush = Ruměnec
theme-graphite = Grafit
theme-obsidian = Obsidián
theme-ember = Žár
theme-moss = Mech
theme-aubergine = Lilek
theme-oxblood = Bordó
theme-light-slate-blurb = Chladná modrošedá
theme-light-paper-blurb = Prostá neutrální šedá
theme-light-linen-blurb = Teplá krémová, jantarová
theme-light-sage-blurb = Jemná zelená
theme-light-lilac-blurb = Bledě fialová
theme-light-blush-blurb = Prachově růžová
theme-dark-graphite-blurb = Neutrální šedá bez nádechu
theme-dark-slate-blurb = Modrošedá
theme-dark-obsidian-blurb = Téměř černá, pro OLED
theme-dark-ember-blurb = Teplá antracitová, jantarová
theme-dark-moss-blurb = Lesně zelená až černá
theme-dark-aubergine-blurb = Švestkově černá, šeříková
theme-dark-oxblood-blurb = Vínově černá, korálová

## texnames.ts

texnames-needs-map = Vzor potřebuje { "{" }map{ "}" }, jinak by všechny mapy materiálu dostaly stejný název
texnames-placeholders = Vzor může používat { "{" }name{ "}" }, { "{" }map{ "}" } a { "{" }size{ "}" }
texnames-bad-characters = Názvy nemohou obsahovat < > : " / \ | ? ani *
texnames-no-suffix = Zadejte příponu pro mapy typu { $role }
texnames-bad-suffix = Přípona pro mapy typu { $role } nemůže obsahovat < > : " / \ | ? ani *
texnames-slot-no-suffix = Zadejte příponu pro „{ $slot }“
texnames-slot-bad-suffix = Přípona pro „{ $slot }“ nemůže obsahovat < > : " / \ | ? ani *
texnames-ending-missing = Každá vlastní koncovka potřebuje koncovku, kterou má hledat
texnames-ending-bad = Vlastní koncovky nemohou obsahovat < > : " / \ | ? ani *
texnames-ending-twice = { $ending } je ve vašich koncovkách dvakrát
texnames-slot-metallic-roughness = Kov a drsnost
texnames-slot-specular-color = Barva spekulárního odrazu
texnames-slot-specular-glossiness = Spekulární odraz a lesk
texnames-slot-clearcoat = Lak
texnames-slot-clearcoat-roughness = Drsnost laku
texnames-slot-clearcoat-normal = Normála laku
texnames-slot-sheen-color = Barva sametového lesku
texnames-slot-sheen-roughness = Drsnost sametového lesku
texnames-slot-transmission = Propustnost
texnames-slot-thickness = Tloušťka
texnames-slot-diffuse-transmission = Difuzní propustnost
texnames-slot-diffuse-transmission-color = Barva difuzní propustnosti
texnames-slot-iridescence = Irizace
texnames-slot-iridescence-thickness = Tloušťka irizace
texnames-slot-anisotropy = Anizotropie

## storeFormats.ts

storefiles-field-title = Název
storefiles-field-title-hint = Přesný název nabídky. Pojmenovává balíček a bez něj se soubor přeskočí.
storefiles-field-seller = Prodejce
storefiles-field-seller-hint = Kdo ho vytvořil nebo prodává.
storefiles-field-license = Licence
storefiles-field-license-hint = Tak, jak ji uvádí soubor, například CC-BY-4.0.
storefiles-field-url = Stránka v obchodě
storefiles-field-url-hint = Adresa nabídky, začínající http:// nebo https://.
storefiles-field-words = Štítky
storefiles-field-words-hint = Slova, kterými se má oštítkovat. Hodnoty oddělené čárkami se rozdělí.
storefiles-field-type = Druh
storefiles-field-type-hint = Co to je, například „3D model“ nebo „povrch“. Také štítek.
storefiles-field-colors = Barvy
storefiles-field-colors-hint = Jeho barvy jako štítky color:.
storefiles-field-id = ID v obchodě
storefiles-field-id-hint = ID, které mu dal obchod: písmena, číslice, _ nebo -.
storefiles-field-origin = Místo vzniku
storefiles-field-origin-hint = Kde byl vytvořen nebo naskenován, pro popis.
storefiles-field-size = Velikost
storefiles-field-size-hint = Jak je velký, slovy, pro popis.
storefiles-field-title-name = název
storefiles-field-seller-name = prodejce
storefiles-field-license-name = licenci
storefiles-field-url-name = stránku v obchodě
storefiles-field-words-name = štítky
storefiles-field-type-name = druh
storefiles-field-colors-name = barvy
storefiles-field-id-name = ID v obchodě
storefiles-field-origin-name = místo vzniku
storefiles-field-size-name = velikost

## StoreFormatsPanel.vue

storefiles-load-failed = Formáty souborů obchodů se nepodařilo načíst: { $error }
storefiles-remove-title = Odebrat { $name }?
storefiles-remove-body = { $file } se smaže z datové složky. Balíčky, které už jím byly přečtené, si ponechají, co přečetl.
storefiles-remove = Odebrat
storefiles-removed = Odebráno: { $name }.
storefiles-remove-failed = { $name } se nepodařilo odebrat: { $error }
storefiles-unread = Soubory, které Hoard nečte
storefiles-loading = Načítám…
storefiles-unread-none = Žádné: žádný soubor v knihovně nevypadá jako nepřečtený soubor obchodu.
storefiles-unread-what = Soubory ve více než jednom balíčku nebo pojmenované jako soubory obchodů, které nečte žádný formát.
storefiles-unread-packs =
    { $count ->
        [one] { $count } balíček
        [few] { $count } balíčky
       *[other] { $count } balíčků
    }
storefiles-unread-example = např. { $pack }
storefiles-teach = Naučit Hoard tento soubor
storefiles-formats = Formáty
storefiles-formats-what = Vaše se zkoušejí jako první a nahrazují vestavěný formát se stejným názvem.
storefiles-dir-before = Jsou uložené v
storefiles-dir-after = .
storefiles-yours = Vlastní
storefiles-built-in = Vestavěný
storefiles-edit = Upravit
storefiles-start-from = Začít z tohoto

## TagsPanel.vue

tags-load-failed = Štítky se nepodařilo načíst: { $error }
tags-rename-title = Přejmenovat { $tag }?
tags-merge-title = Sloučit štítky ({ $count })?
tags-rename-body =
    { $count ->
        [one] Jeho { $count } asset dostane
        [few] Jeho { $count } assety dostanou
       *[other] Jeho { $count } assetů dostane
    } místo něj { $into } a štítkovače od teď používají nový název.
tags-merge-body =
    { $tags } se změní na { $into } u nejvýše { $count ->
        [one] { $count } assetu
        [few] { $count } assetů
       *[other] { $count } assetů
    }. Štítkovače od teď používají { $into }.
tags-rename = Přejmenovat
tags-merge = Sloučit
tags-merged =
    { $count ->
        [one] { $count } asset má teď štítek { $into }
        [few] { $count } assety mají teď štítek { $into }
       *[other] { $count } assetů má teď štítek { $into }
    }
tags-merge-failed = Nepodařilo se sloučit: { $error }
tags-forget-failed = Nepodařilo se to změnit: { $error }
tags-find = Najít štítky
tags-find-placeholder = Najít štítky, např. rock nebo set:
tags-count =
    { $count ->
        [one] { $count } štítek
        [few] { $count } štítky
       *[other] { $count } štítků
    }
tags-count-more = { $count }+ štítků
tags-what = Vyberte štítky, které znamenají totéž, a slučte je, nebo vyberte jeden a přejmenujte ho.
tags-new-name = Nový název
tags-merge-into = Sloučit do
tags-saved-as-before = Uloženo jako
tags-saved-as-after = { "" }
tags-cancel = Zrušit
tags-loading = Načítám…
tags-none = Žádné štítky.
tags-none-containing = Žádné štítky obsahující „{ $text }“.
tags-browse-title = Procházet assety se štítkem { $tag }
tags-browse = Procházet
tags-merged-heading = Sloučené štítky
tags-merged-what = Když štítkovače přijdou s některým z těchto starých názvů, asset dostane štítek, do kterého byl sloučen.
tags-allow-old = Znovu povolit starý název

## LibraryStats.vue

stats-source-human = Přidané lidmi
stats-source-ai = Navržené AI
stats-source-rule = Vestavěná pravidla
stats-source-meta = Ze souborů
stats-assets = Assety
stats-packs = Balíčky
stats-on-disk = Na disku
stats-duplicates-title =
    { $count ->
        [one] { $count } soubor je bajtově identickou kopií jiného a ukládá se jen jednou
        [few] { $count } soubory jsou bajtově identickými kopiemi jiných a ukládají se jen jednou
       *[other] { $count } souborů je bajtově identickými kopiemi jiných a ukládají se jen jednou
    }
stats-saved-dedup = Ušetřeno deduplikací
stats-distinct-tags = Různé štítky
stats-archive = V archivu
stats-load-failed = Podrobnosti se nepodařilo načíst: { $error }
stats-counting = Počítám…
stats-by-kind = Místo podle druhu
stats-files-size =
    { $count ->
        [one] { $name }: { $count } soubor, { $size }
        [few] { $name }: { $count } soubory, { $size }
       *[other] { $name }: { $count } souborů, { $size }
    }
stats-months = Přidáno za poslední rok
stats-month =
    { $count ->
        [one] { $month }: { $count } asset, { $size }
        [few] { $month }: { $count } assety, { $size }
       *[other] { $month }: { $count } assetů, { $size }
    }
stats-tagging = Štítkování
stats-tagged = Oštítkováno
stats-untagged = Bez štítků: { $count }
stats-described = S popisem
stats-described-assets =
    { $count ->
        [one] { $count } asset
        [few] { $count } assety
       *[other] { $count } assetů
    }
stats-human-edited = Upraveno ručně
stats-human-edited-unit =
    { $count ->
        [one] asset
        [few] assety
       *[other] assetů
    }
stats-source-different =
    { $count ->
        [one] { $count } různý
        [few] { $count } různé
       *[other] { $count } různých
    }
stats-most-used = Nejpoužívanější
stats-show-tag = Zobrazit assety se štítkem { $tag }
stats-file-types = Typy souborů
stats-biggest-packs = Největší balíčky
stats-browse-pack = Procházet { $name }
stats-largest-files = Největší soubory
stats-open-file = Otevřít { $name }
stats-empty = Knihovna je zatím prázdná. Přidejte balíček a jeho čísla se objeví tady.

## UpdatesPanel.vue

updates-saved = Hoard ji zavolá, až bude připravená aktualizace
updates-saved-none = Žádná adresa k volání
updates-save-failed = Nepodařilo se uložit
updates-call-answered = Odpověděla { $result }
updates-call-refused = Požadavek nepřijala: { $result }
updates-call-failed = Nepodařilo se ji zavolat
updates-called = Voláno { $when } pro { $version }: { $result }
updates-called-failed = Voláno { $when } pro { $version }: { $result }. Hoard to zkusí znovu za hodinu.
updates-latest = Hoard { $version } je nejnovější verze.
updates-out = Je venku Hoard { $version }.
updates-out-idle = Je venku Hoard { $version }; volání proběhne během pár minut, dokud se nic neděje.
updates-out-busy = Je venku Hoard { $version }; čeká se, až se nebude nic dít ({ $work }).
updates-busy-more =
    { $count ->
        [one] { $count } další
        [few] { $count } další
       *[other] { $count } dalších
    }
updates-busy-jobs =
    { $count ->
        [one] { $count } úloha čeká
        [few] { $count } úlohy čekají
       *[other] { $count } úloh čeká
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } nahrávání
        [few] { $count } nahrávání
       *[other] { $count } nahrávání
    }
updates-list-separator = ,{ " " }
updates-what = Když vyjde nová verze, Hoard může zavolat adresu, jakmile se nic neděje (žádná nahrávání, úlohy, převody ani údržba), takže to, co aktualizuje jeho kontejner, ho nerestartuje uprostřed nahrávání. Pošle POST s verzí v těle JSON. Potřebuje k tomu kontrolu verzí výše.
updates-url = Adresa k volání
updates-url-placeholder = Žádná (např. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Uloženo (psaním nahradíte)
updates-auth-placeholder = Žádná (např. Bearer my-token)
updates-remove = Odebrat
updates-save = Uložit
updates-call-now-title = Spustí se to, na co je napojená
updates-call-now = Zavolat teď
updates-what-calls = Co může volat?
updates-watchtower = HTTP API Watchtoweru
updates-watchtower-at = :{ " " }
updates-watchtower-with = , s{ " " }
updates-watchtower-token = { " " }a tokenem z{ " " }
updates-watchtower-its = { " " }(endpoint{ " " }
updates-watchtower-endpoint = { " " }musí být zapnutý).
updates-portainer = Webhook Portaineru
updates-portainer-rest = { " " }na stacku nebo kontejneru Hoardu se zapnutým re-pull image. Token je v URL.
updates-script = Skript na hostiteli
updates-script-listener = { " " }za posluchačem webhooků (například{ " " }
updates-script-runs = ), který spouští{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }nebo cokoli jiného, co přijímá webhook.
updates-poll-before = Nebo adresu vynechte: skript s přístupovým tokenem může dotazovat{ " " }
updates-poll-until = { " " }tak dlouho, dokud{ " " }
updates-poll-and = { " " }není nastavené a{ " " }
updates-poll-after = { " " }není true, a pak stáhnout novou verzi a restartovat Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Údržbu se nepodařilo načíst
maintenance-reanalyze-title = Znovu analyzovat všechny balíčky?
maintenance-reanalyze-body = Každý soubor se znovu přečte, což u velké knihovny může trvat dlouho; balíčky se zobrazují jako zpracovávané, dokud na ně nepřijde řada. Vaše vlastní názvy, štítky a poznámky zůstanou.
maintenance-reanalyze-confirm = Znovu analyzovat
maintenance-gc-title = Smazat nepoužívané soubory?
maintenance-gc-body = Uložené soubory, na které nic v knihovně neodkazuje, se trvale smažou. Balíčky v archivu si své soubory ponechají.
maintenance-gc-confirm = Smazat je
maintenance-start-failed = „{ $task }“ se nepodařilo spustit
maintenance-just-now = právě teď
maintenance-minutes-ago = před { $count } min
maintenance-hours-ago = před { $count } h
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Načítám…
maintenance-running = Běží…
maintenance-starting = Spouštím…
maintenance-run = Spustit
maintenance-started = { $detail } · spuštěno { $when }
maintenance-starting-detail = Spouští se
maintenance-finished = Dokončeno:{ " " }
maintenance-failed = Selhalo:{ " " }
maintenance-never-run = Nikdy nespuštěno
maintenance-history = Minulá spuštění
maintenance-col-task = Úloha
maintenance-col-finished = Dokončeno
maintenance-col-took = Trvalo
maintenance-col-result = Výsledek
maintenance-script-before = Zálohy a kontrolu poškození lze spouštět i ze skriptu:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }a{ " " }
maintenance-script-after = . I tato spuštění se zobrazují tady.

## SettingsPage.vue

settings-title = Nastavení
settings-loading = Načítám…
settings-load-failed = Nastavení se nepodařilo načíst: { $error }
settings-page-general = Obecné
settings-page-general-blurb = Jak se tento server jmenuje, pro všechny, kdo ho používají.
settings-page-general-blurb-desktop = Jak se tato knihovna jmenuje.
settings-page-appearance = Vzhled
settings-page-appearance-blurb = Motivy, které tento prohlížeč používá, když je světlý a když je tmavý. Ukládá se jen v tomto prohlížeči.
settings-page-library = Knihovna
settings-page-library-blurb = Co je v Hoardu: jak je velký, z čeho se skládá, jak rostl a jak dobře je oštítkovaný. Archivované věci se nepočítají.
settings-page-tags = Štítky
settings-page-tags-blurb = Všechny štítky v knihovně. Slučte ty, které znamenají totéž, nebo některý přejmenujte; štítkovače od té doby používají nový název.
settings-page-store-files = Soubory obchodů
settings-page-store-files-blurb = Soubory, které obchody přikládají ke svým assetům a které Hoard čte, aby pojmenoval balíčky a uvedl jejich autory. Naučte ho ty, které nezná, a sdílejte, co jste ho naučili.
settings-page-locations = Umístění knihovny
settings-page-locations-blurb = Složky na serveru, ve kterých je knihovna: ty, do kterých Hoard ukládá soubory, a složky s balíčky, které indexuje tam, kde jsou.
settings-page-locations-blurb-desktop = Složky, ve kterých je knihovna: ty, do kterých Hoard ukládá soubory, a složky s balíčky, které indexuje tam, kde jsou.
settings-page-tools = Nástroje
settings-page-tools-blurb = Externí programy, které Hoard používá k převodu a čtení souborů.
settings-page-ai = Štítkování pomocí AI
settings-page-ai-blurb = Model AI pojmenovává, popisuje a štítkuje nové balíčky navíc k vestavěným pravidlům. Jeho štítky jsou označené jako navržené.
settings-page-search = Hledání obrázků
settings-page-search-blurb = Hledejte obrázky podle toho, co zobrazují, nejen podle názvů, a nacházejte věci, které jsou si podobné druhem, nejen barvou.
settings-page-adding = Přidávání balíčků
settings-page-adding-blurb = Co Hoard s balíčkem při přidání dělá kromě čtení a štítkování.
settings-page-downloads = Stahování
settings-page-downloads-blurb = Jak se jmenují soubory v tom, co Hoard posílá: stahování, zipy, nástroj hoard a AI asistenti. Soubory v knihovně zůstávají, jak jsou.
settings-page-account = Účet
settings-page-account-blurb = Vaše heslo, další lidé, kteří se mohou přihlásit, a přístupové tokeny pro skripty a jiné programy.
settings-page-auth = Ověřování
settings-page-auth-blurb = Jak se lidé přihlašují: vlastní účty Hoardu, adresář LDAP, Microsoft Active Directory nebo poskytovatel OAuth, jak dlouho relace trvají a které skupiny dostanou jakou roli.
settings-page-connection = Připojení
settings-page-connection-blurb = Jak se prohlížeče a aplikace dostanou k Hoardu: adresa, pod kterou je známý, a HTTPS, které některé funkce prohlížeče potřebují.
settings-page-performance = Výkon
settings-page-performance-blurb = Jak usilovně server pracuje, jak se vytvářejí a uchovávají miniatury a kolik nejvýše přijme.
settings-page-performance-blurb-desktop = Jak usilovně Hoard pracuje, jak se vytvářejí a uchovávají miniatury a kolik nejvýše přijme.
settings-page-issues = Problémy
settings-page-issues-blurb = Co se pokazilo při otevírání nebo zpracování souborů: archivy, které nešly rozbalit, soubory, které nešly přečíst, úlohy, které selhaly. Až odstraníte příčinu, zkuste je znovu, nebo je skryjte.
settings-page-maintenance = Údržba
settings-page-maintenance-blurb = Zálohování, kontrola poškození a úklid. Od každého běží najednou jen jedno a během toho můžete tuto stránku opustit.
settings-the-default = výchozí hodnota
settings-default-on = zapnuto
settings-default-off = vypnuto
settings-default-auto = automaticky
settings-option-saved = { $name }: { $value }
settings-back-to-default = Zpět na výchozí
settings-save = Uložit
settings-use = Použít
settings-status-off = Vypnuto

## SettingsPage.vue: Vzhled

settings-themes = Motivy
settings-look-light = Světlý
settings-look-dark = Tmavý
settings-look-group =
    { $scheme ->
        [light] Světlý motiv
       *[dark] Tmavý motiv
    }
settings-preview =
    Zobrazuje se { $preview ->
        [light] světlý
       *[dark] tmavý
    } motiv, abyste ho viděli. Používáte { $used ->
        [light] světlý
       *[dark] tmavý
    }{ $system ->
        [yes] { " " }(podle systému)
       *[other] { "" }
    } a vrátí se, až tuto stránku opustíte.
settings-preview-back =
    Zpět na { $used ->
        [light] světlý
       *[dark] tmavý
    }
settings-look-hint = Mezi světlým a tmavým (nebo podle systému) přepínáte v horní liště.

## SettingsPage.vue: Nástroje

settings-blender-what = Na pozadí převádí modely .blend, .dae, .abc a .x3d do glTF, aby dostaly miniatury a 3D náhled. Balíčky přidané, když byl vypnutý, se převedou, jakmile bude připravený.
settings-blender-download = Použije nainstalovaný Blender, nebo stáhne Blender 4.5 LTS (asi 400 MB) ze zrcadla blender.org.
settings-ffmpeg-what = Kóduje stahování v MP3, OGG a Opus a čte zvuk, který Hoard sám dekódovat neumí (AAC, M4A, WMA, …).
settings-ffmpeg-download = Použije systémový ffmpeg se správnými enkodéry, nebo stáhne statický build (asi 80 MB).
settings-tool-reset = { $tool }: zpět na výchozí
settings-tool-no-download = Použije nainstalovaný { $tool }. Pro tento počítač není build ke stažení, takže ho nainstalujte nebo zadejte jeho cestu.
settings-tool-path = Cesta k programu { $tool }
settings-mode-off = Vypnuto
settings-mode-auto = Automaticky
settings-mode-path = Tento program
settings-tool-looking = Hledám…
settings-tool-downloading = Stahuji… průběh je v indikátoru aktivity
settings-tool-ready = Připraveno: { $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = Nástroje, kterými se MIDI soubory přehrávají a převádějí.
settings-soundfont-file = Soubor SoundFont
settings-soundfont-placeholder = Nalezený na serveru, nebo vestavěný
settings-soundfont-placeholder-desktop = Nalezený v tomto počítači, nebo vestavěný
settings-soundfont-default = nalezený na serveru, nebo vestavěný
settings-soundfont-default-desktop = nalezený v tomto počítači, nebo vestavěný

## SettingsPage.vue: Hledání obrázků

settings-clip-heading = Hledání podle toho, co obrázky zobrazují
settings-clip-reset = Hledání obrázků: zpět na výchozí
settings-clip-saved-on = Hledání obrázků: zapnuto
settings-clip-saved-off = Hledání obrázků: vypnuto
settings-clip-what = Napište, co hledáte („mossy rock“, „sci-fi door“, „wooden barrel“), a obrázky, které to zobrazují, se objeví vedle názvů obsahujících tato slova. „Další podobné“ najde věci stejného druhu, nejen stejných barev.
settings-clip-toggle = Zapnout hledání obrázků
settings-clip-hint = Používá model CLIP od OpenAI spuštěný na procesoru tohoto serveru: nic ho neopouští. Zapnutí stáhne model ({ $size } MB) z Hugging Face a pak se na pozadí s nízkou prioritou naučí každý obrázek. Během práce nebo hledání zabere 300 až 600 MB paměti. Vypnutí model smaže a to, co se naučil, si ponechá na příště.
settings-clip-hint-desktop = Používá model CLIP od OpenAI spuštěný na procesoru tohoto počítače: nic ho neopouští. Zapnutí stáhne model ({ $size } MB) z Hugging Face a pak se na pozadí s nízkou prioritou naučí každý obrázek. Zatímco pracuje nebo když hledáte, zabere 300 až 600 MB paměti. Vypnutí model smaže a to, co se naučil, si ponechá na příště.
settings-clip-looking = Hledám model…
settings-clip-downloading = Stahuji model… průběh je v indikátoru aktivity
settings-clip-ready-empty = Připraveno. Obrázky se učí, jak se přidávají balíčky.
settings-clip-ready-all =
    { $count ->
        [one] Připraveno: zná { $count } obrázek
        [few] Připraveno: zná všechny { $count } obrázky
       *[other] Připraveno: zná všech { $count } obrázků
    }
settings-clip-learning = Připraveno: učí se obrázky, zatím { $done } z { $total }
settings-clip-unavailable = Model se nepodařilo získat: { $detail }. Zkouší se to znovu každou hodinu.

## SettingsPage.vue: Štítkování pomocí AI

settings-ai-service = Služba
settings-ai-off = Vypnuto
settings-ai-off-hint = Balíčky se štítkují jen vestavěnými pravidly.
settings-ai-local = Místní model
settings-ai-local-hint = Služba llm v souboru compose Hoardu (nebo cargo xtask llm). Níže stáhněte model; běží na tomto počítači a nic ho neopouští.
settings-ai-chatgpt = Předplatné ChatGPT
settings-ai-chatgpt-hint = Používá vaše předplatné ChatGPT Go, Plus, Pro nebo Business místo klíče API.
settings-ai-openrouter-hint = Jeden klíč pro stovky hostovaných modelů, některé zdarma. Modely, které přijímají obrázky, jsou označené.
settings-ai-openai-hint = Hostuje OpenAI.
settings-ai-anthropic-hint = Hostuje Anthropic, přes svůj endpoint kompatibilní s OpenAI.
settings-ai-gemini-hint = Hostuje Google, přes svůj endpoint kompatibilní s OpenAI.
settings-ai-ollama-hint = Server Ollama, který provozujete. Změňte adresu, pokud to není kontejner s názvem ollama.
settings-ai-custom = Jiná (kompatibilní s OpenAI)
settings-ai-custom-hint = Cokoli, co mluví chat API OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Připojuji…
settings-ai-connected =
    { $count ->
        [one] Připojeno · { $count } model
        [few] Připojeno · { $count } modely
       *[other] Připojeno · { $count } modelů
    }
settings-ai-connected-none = Připojeno, ale nenabízí žádné modely; napište název modelu
settings-chatgpt-connected-none = Připojeno; napište název modelu
settings-ai-takes-images = přijímá obrázky
settings-ai-saved = Štítkování pomocí AI: { $service }
settings-ai-saved-off = Štítkování pomocí AI je vypnuté
settings-ai-reset = Štítkování pomocí AI: zpět na výchozí
settings-ai-on = Zapnuto: { $service } · { $model }
settings-ai-url = Adresa
settings-ai-key = Klíč API
settings-ai-key-saved = Uloženo (psaním nahradíte)
settings-ai-key-paste = Vložte svůj klíč
settings-ai-key-optional = Žádný (volitelné)
settings-ai-key-remove = Odebrat
settings-ai-get-key = Získat klíč
settings-ai-model = Model
settings-ai-model-pick = Vyberte nebo napište model
settings-ai-model-name = Název modelu
settings-ai-refresh = Obnovit
settings-ai-connect = Připojit
settings-ai-need-key = Přidejte klíč API, abyste viděli jeho modely
settings-ai-vision = Posílat i miniatury (potřebuje model, který přijímá obrázky)
settings-ai-batch = Na požadavek
settings-ai-batch-unit = assetů (méně pro malé místní modely, více pro velké hostované)

## SettingsPage.vue: Přihlášení do ChatGPT

settings-chatgpt-account = Účet
settings-chatgpt-checking = Kontroluji…
settings-chatgpt-status-as = Přihlášeno jako { $email }
settings-chatgpt-status = Přihlášeno
settings-chatgpt-signed-in-as = Přihlášeno do ChatGPT jako { $email }
settings-chatgpt-signed-in = Přihlášeno do ChatGPT
settings-chatgpt-signed-out = Odhlášeno z ChatGPT
settings-chatgpt-continue = Pokračovat s ChatGPT
settings-chatgpt-sign-out = Odhlásit
settings-chatgpt-sign-out-title = Odhlásit z ChatGPT?
settings-chatgpt-sign-out-in-use = Štítkování pomocí AI používá vaše předplatné ChatGPT, takže se zastaví, dokud se znovu nepřihlásíte nebo nevyberete jinou službu.
settings-chatgpt-sign-out-body = Hoard přihlášení zapomene a požádá OpenAI, aby ho ukončil.
settings-chatgpt-manage = Spravovat využití
settings-chatgpt-finish-before = Dokončete přihlášení v okně, které se otevřelo (
settings-chatgpt-finish-link = otevřít znovu
settings-chatgpt-finish-middle = ). Pokud skončí na stránce, která se nenačte, na adrese začínající{ " " }
settings-chatgpt-finish-after = , zkopírujte tuto adresu a vložte ji sem:
settings-chatgpt-paste = Adresa, na které přihlášení skončilo
settings-chatgpt-finish = Dokončit
settings-chatgpt-usage = Požadavky na štítkování se počítají do využití vašeho předplatného ChatGPT. Spravovat využití ukazuje, kolik Hoard spotřeboval, a umožňuje to omezit. Chcete-li Hoard odpojit přímo v ChatGPT, přejděte na chatgpt.com do Settings → Security and login → Sign in with ChatGPT.

## SettingsPage.vue: Místní modely

settings-local-downloading-progress = Stahuji { $percent } % · { $done } z { $total }
settings-local-downloading = Stahuji…
settings-local-incomplete = Stahování se zastavilo
settings-local-failed = Minule se nepodařilo spustit
settings-local-in-use-running = Používá se · běží
settings-local-in-use = Používá se
settings-local-starting = Spouští se…
settings-local-running = Běží
settings-local-ready-images = Připraveno · přijímá obrázky
settings-local-ready-text = Připraveno · jen text
settings-local-delete-title = Smazat { $name }?
settings-local-delete-in-use = Štítkování pomocí AI používá tento model, takže se zastaví, dokud nevyberete jiný. Jeho soubory se smažou ze serveru modelů.
settings-local-delete-downloading = Stahování se zastaví a to, co se zatím stáhlo, se smaže.
settings-local-delete-body = Jeho soubory se smažou ze serveru modelů. Později ho můžete stáhnout znovu.
settings-local-delete = Smazat
settings-local-deleting = Mažu…
settings-local-cancel = Zrušit
settings-local-resume = Pokračovat
settings-local-download = Stáhnout
settings-local-start-before = Spusťte ho pomocí{ " " }
settings-local-start-or = { " " }(nebo{ " " }
settings-local-start-developing = { " " }při vývoji) a pak{ " " }
settings-local-start-retry = to zkuste znovu
settings-local-start-after = . Funguje i server, který spouští jen jeden model: zadejte níže název jeho modelu.
settings-local-list = Na serveru modelů
settings-local-none = Zatím nic staženého. Vyberte níže; model zůstane na serveru a kdykoli ho můžete vyměnit.
settings-local-catalog = Modely ke stažení
settings-local-size = { $size } GB · { $license }
settings-local-other = Jiný model z Hugging Face
settings-local-other-placeholder = Jiný z Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Přidávání balíčků

settings-store-pages = Stránky v obchodech
settings-store-pages-what = Nové balíčky se na pozadí vyhledávají v obchodech, aby dostaly svou stránku v obchodě, prodejce a licenci.
settings-store-lookup = Automaticky vyhledávat stránky v obchodech
settings-unreal = Soubory Unrealu
settings-unreal-reset = Soubory Unrealu: zpět na výchozí
settings-unreal-what = Co dělat se soubory .uasset.
settings-conv = Pravidla převodu
settings-conv-what = Převádí soubory, které Hoard uchovává, do jednoho formátu, například všechny OBJ a FBX do GLB, aby byla knihovna jednotná. Převedený soubor zaujme místo originálu i s jeho názvem, štítky, kolekcemi a odkazy. Soubory ve sledovaných složkách se nikdy nemění.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Zapnuto
settings-conv-to = Převést do
# Heading over the file formats a rule converts.
settings-conv-from = Z
settings-conv-remove = Odebrat
settings-conv-remove-label = Odebrat pravidlo { $n }
settings-conv-taken = Tento formát už převádí jiné pravidlo
settings-conv-upright = Při převodu postavit modely nastojato (osou Y nahoru, otočené k +Z), tak jak se zobrazují modely balíčku
settings-conv-add = Přidat pravidlo
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Uchovávat originály
settings-conv-keep-days =
    { $count ->
        [one] den
        [few] dny
        [many] dne
       *[other] dní
    }
settings-conv-keep-what = Do té doby lze na stránce převedeného souboru vrátit originál. Textury se do převedeného modelu nekopírují: model odkazuje na obrázky vedle sebe. Soubory .mtl nebo .bin modelu zůstávají.
settings-conv-save = Uložit pravidla
settings-conv-revert = Vrátit změny
settings-conv-existing = Převést soubory, které už jsou v knihovně
settings-conv-saved = Pravidla převodu uložena. Platí pro balíčky přidané od teď.
settings-conv-converting =
    { $count ->
        [one] Na pozadí se převádí jeden soubor
        [few] Na pozadí se převádějí { $count } soubory
        [many] Na pozadí se převádí { $count } souboru
       *[other] Na pozadí se převádí { $count } souborů
    }
settings-conv-nothing = V knihovně není nic, co by bylo potřeba převést.
settings-conv-ask-title =
    { $count ->
        [one] Převést jeden soubor, který už je v knihovně?
        [few] Převést { $count } soubory, které už jsou v knihovně?
        [many] Převést { $count } souboru, které už jsou v knihovně?
       *[other] Převést { $count } souborů, které už jsou v knihovně?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Je v jednom balíčku.
        [few] Jsou v { $packs } balíčcích.
        [many] Jsou v { $packs } balíčku.
       *[other] Jsou v { $packs } balíčcích.
    } Každý originál se uchovává { $days } dní a do té doby ho lze vrátit z jeho stránky.
settings-conv-ask-busy =
    { $count ->
        [one] Jeden je v balíčku, který se ještě přidává; převede se, až to bude hotové.
        [few] { $count } jsou v balíčcích, které se ještě přidávají; převedou se, až to bude hotové.
        [many] { $count } je v balíčcích, které se ještě přidávají; převedou se, až to bude hotové.
       *[other] { $count } je v balíčcích, které se ještě přidávají; převedou se, až to bude hotové.
    }
settings-conv-ask-list = Soubory, které by se převedly
settings-conv-ask-more = …a ještě { $count }
settings-conv-ask-skipped = Zůstanou tak, jak jsou:
settings-conv-skip-watched =
    { $count ->
        [one] jeden soubor ve sledované složce
        [few] { $count } soubory ve sledovaných složkách
        [many] { $count } souboru ve sledovaných složkách
       *[other] { $count } souborů ve sledovaných složkách
    }
settings-conv-skip-unity =
    { $count ->
        [one] jeden soubor, který projekt Unity zná podle názvu
        [few] { $count } soubory, které projekt Unity zná podle názvů
        [many] { $count } souboru, které projekt Unity zná podle názvů
       *[other] { $count } souborů, které projekt Unity zná podle názvů
    }
settings-conv-skip-used =
    { $count ->
        [one] jeden soubor, na který odkazuje scéna nebo jiný soubor
        [few] { $count } soubory, na které odkazuje scéna nebo jiný soubor
        [many] { $count } souboru, na které odkazuje scéna nebo jiný soubor
       *[other] { $count } souborů, na které odkazuje scéna nebo jiný soubor
    }
settings-conv-skip-beside =
    { $count ->
        [one] jeden soubor, jehož převedená kopie už je v jeho balíčku
        [few] { $count } soubory, jejichž převedené kopie už jsou v jejich balíčcích
        [many] { $count } souboru, jejichž převedené kopie už jsou v jejich balíčcích
       *[other] { $count } souborů, jejichž převedené kopie už jsou v jejich balíčcích
    }
settings-conv-skip-blender =
    { $count ->
        [one] jeden soubor, který Blender ještě nenačetl (Nastavení → Nástroje)
        [few] { $count } soubory, které Blender ještě nenačetl (Nastavení → Nástroje)
        [many] { $count } souboru, které Blender ještě nenačetl (Nastavení → Nástroje)
       *[other] { $count } souborů, které Blender ještě nenačetl (Nastavení → Nástroje)
    }
settings-conv-ask-new = Uložit, jen nové balíčky
settings-conv-ask-save-convert =
    { $count ->
        [one] Uložit a převést ho
        [few] Uložit a převést { $count } soubory
        [many] Uložit a převést { $count } souboru
       *[other] Uložit a převést { $count } souborů
    }
settings-conv-ask-convert =
    { $count ->
        [one] Převést ho
        [few] Převést { $count } soubory
        [many] Převést { $count } souboru
       *[other] Převést { $count } souborů
    }
settings-unreal-off = Nechat, jak jsou
settings-unreal-off-hint = Převeďte je ze stránky balíčku, až budete chtít.
settings-unreal-keep = Převést, ponechat originály
settings-unreal-keep-hint = Textury, zvuky a materiály se převedou do běžných formátů; .uasset také zůstane.
settings-unreal-replace = Převést, zahodit originály
settings-unreal-replace-hint = Ušetří místo, jakmile je obsah venku. Meshe dál čekají na Unreal worker.

## SettingsPage.vue: Stahování

settings-tn = Názvy textur
settings-tn-saved = Názvy textur uloženy
settings-tn-reset = Názvy textur: zpět na výchozí
settings-tn-what = Mapy materiálu se odesílají pojmenované podle něj a podle toho, k čemu každá mapa slouží, ať se soubory jmenují jakkoli: Rock_Normal.png, ne rock_moss_xeokah0_raw_8K_Normal.png. Souborů v knihovně se to nedotkne a mapa, kterou uvádí model nebo MTL odeslaný s ní, si svůj název ponechá, aby ji model stále našel.
settings-tn-toggle = Pojmenovat mapy textur podle jejich materiálu
settings-tn-style = Styl
settings-tn-style-group = Styl pojmenování
settings-tn-descriptive = Popisný
settings-tn-short = Krátký
settings-tn-custom = Vlastní
settings-tn-pattern = Vzor
settings-tn-hint-name = { " " }materiál,{ " " }
settings-tn-hint-map = { " " }k čemu mapa slouží,{ " " }
settings-tn-hint-size = { " " }její velikost, když sada existuje ve více velikostech (přidá se na konec, pokud ji vzor vynechá).
settings-tn-example = Příklad
settings-tn-suffixes = Přípony pro jednotlivé druhy map
settings-tn-slots = Při stahování GLB dostane textura zabalená uvnitř modelu bez vlastního souboru název podle materiálu a slotu, který vyplňuje, a i tyto sloty mají přípony:
settings-tn-endings = Vlastní koncovky
settings-tn-endings-what = Textura, která není žádnou z map materiálu, si ponechá svůj název, pokud nekončí jednou z těchto koncovek: _diff → _BC odešle rock_diff.png jako rock_BC.png. Nechte druhé pole prázdné, chcete-li koncovku odstranit.
settings-tn-ending-from = Koncovka
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Nahradit za
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Přidat koncovku
settings-tn-ending-remove = Odebrat
settings-tn-ending-remove-label = Odebrat { $ending }

## SettingsPage.vue: Obecné a Připojení

settings-name = Název
settings-name-what = Jak se tento server jmenuje v horní liště, na kartě prohlížeče a na přihlašovací stránce, abyste při více instalacích Hoardu poznali, která je která.
settings-name-what-desktop = Jak se tato knihovna jmenuje v horní liště.
settings-address = Adresa
settings-address-what = Adresa, kterou lidé a aplikace používají k přístupu k Hoardu, pokud to není ta, na které je tento prohlížeč: adresa reverzní proxy, tunelu nebo název v tailnetu, například https://hoard.example.com. Používají ji odkazy, které Hoard rozdává (Claudovi a jiným aplikacím, při přihlášení, ve svém certifikátu).
settings-public-url = Veřejná adresa
settings-public-url-placeholder = Z adresy prohlížeče (např. https://hoard.example.com)
settings-public-url-default = z adresy prohlížeče

## ProfilerPanel.vue: profiler v horní části Nastavení → Výkon

profiler = Profiler
profiler-what = Co Hoard dělal za posledních deset minut, aktualizováno každé dvě sekundy, dokud se díváte. Uchovává se jen v paměti a po restartu Hoardu začíná znovu.
profiler-starting = První měření dorazí za okamžik.
profiler-failed = Profiler se nepodařilo přečíst: { $error }
profiler-cpu = Využití procesoru
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Celý počítač
profiler-cpu-summary = Hoard využívá { $hoard } procesorů a celý počítač { $machine }.
profiler-memory = Paměť
profiler-serving = Probíhající požadavky
profiler-working = Pracuje na
profiler-working-none = Na pozadí nic neběželo.
profiler-working-more =
    { $count ->
        [one] A ještě jedna.
        [few] A ještě { $count }.
        [many] A ještě { $count }.
       *[other] A ještě { $count }.
    }
profiler-requests = Čas obsluhy požadavků
profiler-requests-what = Každý sloupec představuje deset sekund požadavků, jejichž čas je rozdělený podle toho, kam šel.
profiler-part-db = Databáze
profiler-part-db-title = Čekání na databázi a provádění jejích dotazů
profiler-part-disk = Disk
profiler-part-disk-title = Čtení uložených souborů a komprese toho, co se odesílá
profiler-part-send = Odesílání
profiler-part-send-title = Čekání, než síť převezme to, co se odeslalo
profiler-part-other = Ostatní
profiler-part-other-title = Sestavování odpovědi
profiler-request-count =
    { $count ->
        [one] { $count } požadavek
        [few] { $count } požadavky
        [many] { $count } požadavku
       *[other] { $count } požadavků
    }
profiler-requests-none = Za posledních deset minut žádné požadavky.
profiler-slowest = Nejpomalejší: { $route }, { $time }
profiler-routes = Podle cesty
profiler-route = Cesta
profiler-count = Požadavky
profiler-average = Průměr
profiler-p95 = Nejpomalejších 5 %
profiler-split = Kam šel čas
profiler-sent = Odesláno
profiler-portal = vlastní soubory rozhraní

## SettingsPage.vue: Výkon

settings-render = 3D miniatury
settings-render-toggle = Vykreslovat miniatury 3D modelů
settings-gpu-on = Na GPU: { $renderer }
settings-gpu-software = Nenalezena žádná GPU, takže softwarově ({ $renderer })
settings-gpu-none = Nenalezena žádná GPU ani softwarový renderer, takže modely nedostanou miniatury
settings-thumbs = Miniatury
settings-thumbs-what = Jak se ukládají nové miniatury a pohledy na modely. Ty s průhledností a pixel art jsou vždy PNG. Chcete-li zmenšit ty, které už máte, spusťte v Údržbě Zmenšit miniatury.
settings-previews-size = Náhledy teď zabírají { $size }.
settings-thumbs-saved = Miniatury: { $quality }
settings-thumbs-small = Menší
settings-thumbs-small-hint = JPEG, kvalita 70. V mřížce v pořádku; zblízka trochu měkké.
settings-thumbs-balanced = Vyvážené
settings-thumbs-balanced-hint = JPEG, kvalita 82. Ve velikosti miniatury těžko rozeznatelné od originálu.
settings-thumbs-sharp = Ostřejší
settings-thumbs-sharp-hint = JPEG, kvalita 92. Asi dvakrát větší než Vyvážené.
settings-thumbs-lossless = Bezeztrátové
settings-thumbs-lossless-hint = PNG, přesně tak, jak vzniklo. Několikanásobně větší.
settings-motion-thumbs = Pohyblivé náhledy animací
settings-motion-thumbs-what = Každá animace dostane i krátké smyčkové video, které se přehraje, když na její dlaždici najedete. Každé zabere asi 40 KB a u velké knihovny chvíli trvá je vytvořit. Vypnutím se smažou.
settings-limits = Práce a limity
settings-limits-what = Každé pole se uloží, když ho opustíte nebo stisknete Enter. Velikosti berou jednotky: 100 GB, 512 MB, 1,5 TB; samotné číslo znamená GB.
settings-workers = Úlohy na pozadí
settings-workers-unit = úloh najednou (přidávání, štítkování, převody)
settings-max-upload = Největší nahrávaný soubor
settings-max-upload-unit = na soubor
settings-max-expand = Největší balíček
settings-max-expand-unit = po rozbalení
settings-conversion-cache = Převedená stahování
settings-conversion-cache-unit-now = uchovává se nejvýše, teď { $size }; nejdéle nepoužívané jdou první
settings-conversion-cache-unit = uchovává se nejvýše; nejdéle nepoužívané jdou první

## SettingsPage.vue: Údržba

settings-updates = Aktualizace
settings-version = Toto je Hoard { $version }.
settings-update-check = Každých pár hodin kontrolovat na GitHubu novou verzi
settings-desktop-updates = Hoard při každém spuštění hledá novou verzi, stáhne ji na pozadí a nainstaluje ji, když ho ukončíte.

## Druhy assetů (api.ts)

api-kind-texture = Textury
api-kind-vector = Vektorová grafika
api-kind-sheet = Listy spritů
api-kind-audio = Zvuk
api-kind-model = Modely
api-kind-animation = Animace
api-kind-font = Písma
api-kind-shader = Shadery
api-kind-material = Materiály
api-kind-video = Video
api-kind-document = Dokumenty
api-kind-data = Data
api-kind-other = Ostatní
api-kind-texture-name = textury
api-kind-vector-name = vektorová grafika
api-kind-sheet-name = listy spritů
api-kind-audio-name = zvuk
api-kind-model-name = modely
api-kind-animation-name = animace
api-kind-font-name = písma
api-kind-shader-name = shadery
api-kind-material-name = materiály
api-kind-video-name = video
api-kind-document-name = dokumenty
api-kind-data-name = data
api-kind-other-name = ostatní

## Oznámení, dialogy a načítání (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Zpět
ui-today = dnes
ui-yesterday = včera
ui-days-ago =
    { $days ->
        [one] před { $days } dnem
        [few] před { $days } dny
       *[other] před { $days } dny
    }
ui-notifications = Oznámení
ui-dismiss = Skrýt
ui-cancel = Zrušit
ui-lazy-loading = Načítám { $what }…
ui-lazy-failed = Nepodařilo se načíst { $what }.
ui-lazy-reload = Znovu načíst stránku

## ResetButton.vue

reset-title = Vrátit na výchozí hodnotu: { $value }
reset-label = Vrátit na výchozí hodnotu, { $value }

## SizeInput.vue

size-invalid = Velikost jako 100 GB, 512 MB nebo 1,5 TB
size-hint = Velikost jako 100 GB, 512 MB nebo 1,5 TB; samotné číslo znamená GB

## AuthPanel.vue (Nastavení → Ověřování)

auth-load-failed = Nastavení ověřování se nepodařilo načíst: { $error }
auth-saved = Uloženo. Platí od příštího přihlášení.
auth-method-internal = Vlastní účty
auth-method-internal-hint = Jména a hesla uchovává Hoard; přidávají se v Nastavení → Účet.
auth-method-ldap-hint = Jméno a heslo zadané na přihlašovací stránce ověří adresář: OpenLDAP, FreeIPA, LLDAP nebo jiný server LDAP.
auth-method-ad-hint = Účty domény Windows se přihlašují svým obvyklým jménem a heslem.
auth-method-oauth-hint = Tlačítko „Přihlásit se přes…“, které vede na Google, Microsoft, GitHub, Keycloak, Authentik nebo jiného poskytovatele a zpět.
auth-session-unused = Po 30 dnech nepoužívání
auth-session-1h = 1 hodinu po přihlášení
auth-session-8h = 8 hodin po přihlášení
auth-session-day = Den po přihlášení
auth-session-week = Týden po přihlášení
auth-session-30d = 30 dní po přihlášení
auth-session-hours =
    { $hours ->
        [one] { $hours } hodinu po přihlášení
        [few] { $hours } hodiny po přihlášení
       *[other] { $hours } hodin po přihlášení
    }
auth-role-view = Může prohlížet
auth-role-edit = Může upravovat
auth-role-admin = Správce
auth-role-word-view = prohlížení
auth-role-word-edit = úpravy
auth-role-word-admin = správce
auth-role-word-default = { $role } (výchozí role)
auth-tls-starttls-hint = Obyčejné LDAP (port 389) přepnuté na TLS dřív, než se cokoli odešle.
auth-tls-ldaps-hint = TLS od začátku (port 636).
auth-tls-none = Žádné
auth-tls-none-hint = Hesla jdou sítí tak, jak jsou. Jen v síti, které důvěřujete.
auth-test-failed = Nepodařilo se otestovat: { $error }
auth-preset-custom = Jiný (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel a další: zadejte vydavatele (issuer) a Zjistit doplní adresy.
auth-preset-google-note = Vytvořte klienta OAuth (Web application) v Google Cloud Console → APIs & Services → Credentials. Google neposílá žádné skupiny, takže role přidělte v Nastavení → Účet a vytváření účtů při prvním přihlášení vypněte, pokud se nemá dostat dovnitř kdokoli s účtem Google.
auth-preset-microsoft-note = Zaregistrujte aplikaci v Entra ID → App registrations (přesměrování typu Web na adresu níže). Chcete-li pustit jen svou organizaci, nahraďte „organizations“ ID svého tenanta. Pro role ze skupin přidejte v Token configuration deklaraci groups: uvádí ID skupin.
auth-preset-github-note = Vytvořte aplikaci OAuth v GitHub → Settings → Developer settings. GitHub neposílá žádné skupiny, takže role přidělte v Nastavení → Účet a vytváření účtů při prvním přihlášení vypněte, pokud se nemá dostat dovnitř kdokoli s účtem GitHub.
auth-preset-gitlab-note = Vytvořte aplikaci v GitLab → Preferences → Applications (nebo v administraci své instance: v adresách změňte gitlab.com).
auth-discovered = Adresy poskytovatele jsou doplněné.
auth-copied = Zkopírováno
auth-copy-failed = Nepodařilo se zkopírovat: označte to a zkopírujte sami
auth-sign-in-with = Přihlásit se přes
auth-own-accounts-still-work = Vlastní účty Hoardu se dál přihlašují svými hesly, takže adresář nebo poskytovatel, který nefunguje nebo je špatně nastavený, vás nemůže zamknout venku.
auth-sessions = Relace
auth-sign-out-after = Odhlásit lidi
auth-ldap-server = Server LDAP
auth-server-type = Typ serveru
auth-server-type-custom = Vlastní
auth-servers = Servery
auth-server-n = Server { $n }
auth-remove-server-n = Odebrat server { $n }
auth-remove = Odebrat
auth-add-server = Přidat server
auth-servers-hint = Když jeden není dostupný, zkusí se další.
auth-security = Zabezpečení
auth-connection-security = Zabezpečení připojení
auth-skip-verify = Nekontrolovat certifikát serveru (jen na vyzkoušení)
auth-ca = Certifikační autorita
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (prázdné: ty, kterým důvěřuje systém)
auth-lookup = Vyhledávání lidí
auth-anonymously = Anonymně
auth-reader-account = Účet pro čtení
auth-reader-password = Jeho heslo
auth-saved-type-to-replace = Uloženo: psaním nahradíte
auth-people-under = Lidé jsou pod
auth-name-attribute = Atribut jména
auth-only-matching = Jen záznamy odpovídající
auth-groups-under = Skupiny jsou pod
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (prázdné: jen memberOf)
auth-their-groups = Jejich skupiny
auth-group-filter-hint = { "{" }dn{ "}" } je záznam daného člověka, { "{" }user{ "}" } jméno, kterým se přihlásil.
auth-test = Test
auth-test-hint = Spojí se s každým serverem a přihlásí se jako účet pro čtení. Se jménem a heslem se přihlásí i jako daný člověk a ukáže, co by z toho Hoard vyvodil. Nic se neukládá.
auth-test-user = Koho přihlásit
auth-test-user-placeholder = Koho přihlásit (volitelné)
auth-test-password = Jeho heslo
auth-testing = Testuji…
auth-test-connectivity = Otestovat spojení
auth-test-server-ok = dostupný, účet pro čtení se přihlásil
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = : přihlášení proběhlo (
auth-test-signed-in-after = ).
auth-test-role = Role: { $role }.
auth-test-no-role = Uživatel není v žádné ze skupin uvedených v části Role ze skupin, takže by byl odmítnut.
auth-test-groups = Skupiny: { $groups }
auth-test-no-groups = Nenalezeny žádné skupiny.
auth-provider = Poskytovatel
auth-redirect = Adresa přesměrování
auth-copy = Kopírovat
auth-redirect-hint = Tuto adresu dejte poskytovateli, když vytváříte klienta pro Hoard. Pochází z veřejné adresy v Nastavení → Připojení, nebo z adresy, kterou používáte.
auth-issuer = Vydavatel (issuer)
auth-discover = Zjistit
auth-client-id = ID klienta
auth-client-secret = Tajný klíč klienta
auth-authorization-url = Adresa autorizace
auth-token-url = Adresa tokenu
auth-userinfo-url = Adresa informací o uživateli
auth-userinfo-placeholder = (prázdné: kdo to je, řekne ID token)
auth-scopes = Rozsahy (scopes)
auth-name-from = Jméno z
auth-groups-from = Skupiny z
auth-groups-from-placeholder = (žádné)
auth-button = Tlačítko
auth-button-placeholder = váš účet
auth-accounts = Účty
auth-provision = Vytvořit účet Hoard, když se někdo přihlásí poprvé
auth-provision-on-hint = Jinak správce každého nejdřív přidá v Nastavení → Účet.
auth-provision-off-hint = Správce každého nejdřív přidá v Nastavení → Účet a zvolí, jak se bude přihlašovat.
auth-open-to-anyone = Dovnitř se dostane kdokoli s účtem { $provider }. Níže přidejte skupiny pro prohlížení, nebo tuto volbu vypněte a lidi přidávejte sami.
auth-new-accounts = Nové účty
auth-roles-from-groups = Role ze skupin
auth-roles-hint = Skupiny, jejichž členové dostanou jednotlivé role, jedna na řádek, podle názvu nebo DN. Vyhraje nejvyšší, která odpovídá, a kontroluje se při každém přihlášení. Když nejsou uvedené žádné, nové účty dostanou roli výše a správce ji pak změní. Když jsou skupiny uvedené u Může prohlížet, přihlásit se mohou jen členové uvedené skupiny.
auth-roles-hint-oauth = Skupiny, jejichž členové dostanou jednotlivé role, jedna na řádek, podle názvu nebo DN (nebo čehokoli, co uvádí deklarace groups). Vyhraje nejvyšší, která odpovídá, a kontroluje se při každém přihlášení. Když nejsou uvedené žádné, nové účty dostanou roli výše a správce ji pak změní. Když jsou skupiny uvedené u Může prohlížet, přihlásit se mohou jen členové uvedené skupiny.
auth-saving = Ukládám…
auth-save = Uložit

## HttpsPanel.vue (Nastavení → Připojení)

https-load-failed = HTTPS se nepodařilo načíst: { $error }
https-off-redirecting = HTTPS je vypnuté. Přecházím na http://
https-now-on = HTTPS je zapnuté
https-saved = Uloženo. Nová připojení používají nový certifikát.
https-now-off = HTTPS je vypnuté
https-http3-failed-toast = HTTP/3 se nepodařilo spustit: { $error }
https-copied = Zkopírováno
https-copy-failed = Nepodařilo se zkopírovat: označte to a zkopírujte sami
https-mode-off = Vypnuto
https-mode-off-hint = Jen obyčejné http://.
https-mode-own = Certifikát Hoardu
https-mode-own-hint = Hoard ho vytvoří pro adresy tohoto serveru. Na každé zařízení jednou nainstalujte certifikační autoritu Hoardu a jeho prohlížeče mu budou důvěřovat.
https-mode-custom = Vlastní certifikát
https-mode-custom-hint = Takový, který už máte, např. z tailscale cert nebo Let’s Encrypt. Prohlížeče mu důvěřují, aniž by se cokoli instalovalo.
https-untrusted-own = Tento prohlížeč mu pro { $host } zatím nedůvěřuje: níže nainstalujte certifikační autoritu Hoardu. Uvidíte varování, které lze přeskočit.
https-untrusted-uncovered = Tento prohlížeč mu pro { $host } zatím nedůvěřuje: certifikát tuto adresu nepokrývá. Uvidíte varování, které lze přeskočit.
https-untrusted = Tento prohlížeč mu pro { $host } zatím nedůvěřuje. Uvidíte varování, které lze přeskočit.
https-now = Teď
https-start-failed = HTTPS je nastavené, ale nepodařilo se ho spustit: { $error }
https-on-port = HTTPS je zapnuté, na portu { $port } stejně jako http://.
https-using-it = Právě ho používáte.
https-open = Otevřít { $url }
https-trusted = Tento prohlížeč mu důvěřuje.
https-cert-for-before = Certifikát pro{ " " }
https-cert-for-after = , platný do { $date }.
https-cert-for-after-own = , platný do { $date } (obnovuje se sám).
https-not-covered-before = Nepokrývá{ " " }
https-not-covered-after = , adresu, kterou používáte.
https-is-off = HTTPS je vypnuté: stránky se posílají přes obyčejné http://.
https-addresses = Adresy
https-addresses-hint = Certifikát pokrývá adresy, které Hoard našel, a všechny, které přidáte (název, který mu dává váš router, název v Tailscale).
https-remove-name = Odebrat { $name }
https-another-address = Další adresa
https-another-address-placeholder = Další adresa, např. nas.local
https-add = Přidat
https-your-cert = Váš certifikát
https-saved-cert-before = Uložený certifikát:{ " " }
https-saved-cert-after = { " " }od vydavatele { $issuer }, platný do { $date }. Vložením nového ho nahradíte.
https-pem-before = Certifikát (s řetězcem) a jeho soukromý klíč jako text PEM: soubory, které začínají{ " " }
https-pem-after = .
https-certificate = Certifikát
https-choose-file = Vybrat soubor…
https-private-key = Soukromý klíč
https-checked-before = Pro{ " " }
https-checked-after = , vydavatel { $issuer }, platný do { $date }.
https-checked-after-days =
    , vydavatel { $issuer }, platný do { $date } ({ $days ->
        [one] { $days } den
        [few] { $days } dny
       *[other] { $days } dní
    }).
https-self-signed = Je podepsaný sám sebou: prohlížeče na něj budou upozorňovat.
https-key-matches = Klíč odpovídá.
https-where-to-get = Kde ho získat
https-get-tailscale-before = zapněte HTTPS pro svůj tailnet, pak na serveru spusťte{ " " }
https-get-tailscale-after = { " " }a nahrajte oba soubory. Platí 90 dní: před jejich vypršením příkaz spusťte znovu a soubory nahrajte znovu.
https-get-acme-title = Let’s Encrypt nebo jiný klient ACME
https-get-acme-before = (certbot, acme.sh, lego) pro doménu, kterou vlastníte: nahrajte{ " " }
https-get-acme-and = { " " }a{ " " }
https-get-acme-after = .
https-get-proxy-title = Reverzní proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), která certifikát má: HTTPS tady nechte vypnuté a proxy ať posílá Hoardu obyčejné http://.
https-trying = Zkouším…
https-apply = Použít
https-apply-hint = Hoard nové nastavení nejdřív vyzkouší, a když nefunguje, ponechá si to stávající.
https-trust-title = Důvěřujte Hoardu na svých zařízeních
https-download-ca = Stáhnout hoard-ca.crt
https-trust-hint = Na každé zařízení jednou nainstalujte certifikační autoritu Hoardu a jeho prohlížeče budou tomuto serveru důvěřovat (i každému certifikátu, který pro něj Hoard později vytvoří). Může ručit jen za tento Hoard. Otisk (SHA-256):
https-ca-windows = Otevřete soubor → Nainstalovat certifikát → Aktuální uživatel → Všechny certifikáty umístit v následujícím úložišti → Procházet → Důvěryhodné kořenové certifikační autority → Dokončit. Restartujte prohlížeč.
https-ca-macos = Otevřete soubor; přidá se do aplikace Klíčenka. Tam dvakrát klikněte na „{ $subject }“ → Důvěřovat → Při použití tohoto certifikátu: Vždy důvěřovat.
https-ca-ios-title = iPhone a iPad
https-ca-ios = Stáhněte ho v Safari a povolte. Nastavení → Profil stažen → Instalovat. Pak Nastavení → Obecné → Informace → Nastavení důvěryhodnosti certifikátů → zapněte „{ $subject }“.
https-ca-android = Nastavení → Zabezpečení → Další nastavení zabezpečení → Šifrování a pověření → Instalovat certifikát → Certifikát CA a vyberte soubor. (Názvy se mezi telefony trochu liší; v Nastavení vyhledejte „certifikát“.)
https-ca-linux-1 = Debian a Ubuntu:{ " " }
https-ca-linux-2 = . Fedora a Arch:{ " " }
https-ca-linux-3 = . Chrome v Linuxu má vlastní seznam: Nastavení → Ochrana soukromí a zabezpečení → Zabezpečení → Spravovat certifikáty → Importovat.
https-ca-firefox = Firefox má vlastní seznam: Nastavení → Soukromí a zabezpečení → Certifikáty → Zobrazit certifikáty → Certifikační autority → Importovat a zaškrtněte „Důvěřovat této certifikační autoritě při identifikaci webových serverů“.
https-ca-cli-title = Nástroj příkazové řádky hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , nebo cestu zadejte do{ " " }
https-ca-cli-after = .
https-options = Možnosti
https-redirect = Přesměrovat prohlížeče na jiných zařízeních z http:// na https://
https-redirect-hint = Přesměrovává se jen načítání stránek. Nástroj příkazové řádky hoard, Unreal worker, skripty a aplikace AI dál fungují přes http://, stejně jako http://localhost přímo na serveru.
https-http3-hint-before = Rychlejší na nestabilní Wi-Fi a v telefonech. Potřebuje HTTPS a kromě TCP i otevřený port UDP { $port } (v Dockeru:{ " " }
https-http3-hint-after = ).
https-http3-running = Běží.
https-http3-waits = Spustí se, až bude HTTPS zapnuté.
https-http3-failed = HTTP/3 se nepodařilo spustit: { $error }
https-lockout-title = Když se zamknete venku
https-lockout-1 = { "" }
https-lockout-2 = { " " }přímo na serveru vždy funguje přes obyčejné http://. Nebo HTTPS vypněte z příkazové řádky serveru a spusťte Hoard znovu:{ " " }
https-lockout-3 = { " " }(v Dockeru:{ " " }
https-lockout-4 = , pak{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Nastavení → Účet)

account-role-view = Může prohlížet
account-role-view-hint = Hledání, náhledy a stahování
account-role-edit = Může upravovat
account-role-edit-hint = Navíc přidávání, úpravy, štítkování, kolekce a archivace
account-role-admin = Správce
account-role-admin-hint = Navíc Nastavení, účty a údržba a trvalé mazání
account-source-internal = Heslo Hoardu
account-source-ldap = Heslo z adresáře { $directory }
account-source-oauth = Přihlášení přes { $provider }
account-source-word-directory = adresář
account-load-failed = Účty se nepodařilo načíst: { $error }
account-failed = { $what }: { $error }
account-password-failed = Heslo se nepodařilo změnit
account-password-changed = Heslo změněno. Na ostatních zařízeních jste odhlášeni.
account-add-failed = { $name } se nepodařilo přidat
account-added = { $name } se teď může přihlásit.
account-role-failed = Nepodařilo se změnit, co smí { $name }
account-remove-title = Odebrat { $name }?
account-remove-body = Účet bude všude odhlášen a znovu se nepřihlásí. Nic z toho, co do knihovny přidal, se neodebere.
account-remove = Odebrat
account-remove-failed = { $name } se nepodařilo odebrat
account-token-failed = Token se nepodařilo vytvořit
account-copied = Zkopírováno
account-copy-token-failed = Nepodařilo se zkopírovat: označte token a zkopírujte ho sami
account-copy-failed = Nepodařilo se zkopírovat: označte to a zkopírujte sami
account-revoke-title = Zrušit „{ $name }“?
account-revoke-body = Cokoli, co ho ještě používá, bude od teď odmítnuto.
account-revoke = Zrušit platnost
account-revoke-failed = „{ $name }“ se nepodařilo zrušit
account-disconnect-title = Odpojit { $app }?
account-disconnect-body = Aplikace bude od teď odmítnuta, a aby mohla Hoard používat, musí se připojit znovu.
account-disconnect = Odpojit
account-disconnect-failed = { $app } se nepodařilo odpojit
account-guest-failed = Nepodařilo se to změnit
account-never = nikdy
account-you = Vy
account-sign-out = Odhlásit
account-signed-in-as-before = Přihlášený účet:{ " " }
account-signed-in-as-admin = { " " }(správce).
account-signed-in-as-edit = { " " }(může měnit knihovnu).
account-signed-in-as-view = { " " }(může prohlížet, ale nic neměnit).
account-app-signin = Aplikace Hoard vás přihlásí sama, takže žádné heslo nemáte. Níže si vytvořte přístupový token pro zásuvné moduly enginů a příkaz hoard.
account-password-at-provider = Vaše heslo uchovává poskytovatel, přes kterého se přihlašujete, takže ho změňte tam.
account-password-at-directory = Vaše heslo uchovává adresář, přes který se přihlašujete, takže ho změňte tam.
account-current-password = Současné heslo
account-new-password = Nové heslo
account-new-password-again = Nové heslo znovu
account-change-password = Změnit heslo
account-password-mismatch = Nová hesla se neshodují.
account-password-rule = Alespoň 8 znaků.
account-people = Lidé
account-people-hint = Každý může prohlížet (hledat, zobrazovat náhledy a stahovat), navíc měnit knihovnu (přidávat, upravovat, štítkovat, tvořit kolekce a archivovat), nebo být i správcem (Nastavení, účty a údržba a trvalé mazání).
account-you-tag = (vy)
account-signs-in-oauth = Přihlašuje se přes poskytovatele OAuth
account-signs-in-directory = Přihlašuje se přes adresář
account-since = od { $date }
account-what-can-do = Co smí { $name }
account-name = Jméno
account-how-sign-in = Jak se přihlašuje
account-password-label = Heslo (alespoň 8 znaků)
account-password-placeholder = Heslo (8+ znaků)
account-what-they-can-do = Co smí
account-add = Přidat
account-ai-apps = Claude a další aplikace AI
account-ai-apps-hint = Připojte Clauda (nebo jakoukoli aplikaci MCP) a bude za vás hledat a stahovat assety a popisovat a štítkovat to, co Hoard ještě nestihl. Poprvé se do Hoardu přihlásí; povolíte to tady v prohlížeči.
account-mcp-address = Adresa MCP Hoardu
account-copy = Kopírovat
account-claude-code-1 = spusťte{ " " }
account-claude-code-2 = , pak{ " " }
account-claude-code-3 = { " " }pro přihlášení.{ " " }
account-claude-code-4 = { " " }popíše balíček.
account-claude-ai = Settings → Connectors → Add custom connector, vložte adresu a pak Connect. Tyto aplikace se připojují ze serverů Anthropicu, takže Hoard musí být dostupný z internetu přes HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, vložte adresu a pak Connect. Tyto aplikace se připojují ze serverů Anthropicu, takže Hoard musí být dostupný z internetu přes HTTPS (tato adresa není: nastavte Veřejnou adresu v Nastavení → Připojení).
account-connection-meta = pro { $user } od { $since } · poslední použití: { $used }
account-no-apps = Zatím nejsou připojené žádné aplikace.
account-tokens = Přístupové tokeny
account-tokens-hint-1 = Pro nástroj příkazové řádky{ " " }
account-tokens-hint-2 = , skripty, klienty MCP a Unreal worker: posílají ho jako{ " " }
account-tokens-hint-3 = . Token může to, co zvolíte, až do rozsahu toho, co můžete vy, ale nikdy nespravuje účty ani tokeny. Token, který jen prohlíží, stačí k hledání a stahování a k synchronizaci projektu.
account-tokens-admin = Vidíte tokeny a připojené aplikace všech; ostatní vidí jen své.
account-made-before = { "" }
account-made-after = : zkopírujte ho hned. Znovu se nezobrazí.
account-new-token = Nový token
account-cli-1 = Pro CLI:{ " " }
account-cli-2 = , nebo ho uložte do{ " " }
account-cli-3 = .
account-done = Hotovo
account-token-meta = vytvořen { $made } · poslední použití: { $used }
account-token-meta-by = vytvořen { $made }, vytvořil { $by } · poslední použití: { $used }
account-no-tokens = Zatím žádné tokeny.
account-token-name = Co ho bude používat
account-token-name-placeholder = Co ho bude používat, např. „Unreal worker“
account-token-role = Co smí
account-make-token = Vytvořit token
account-guests = Hosté
account-guests-allow = Povolit procházení bez přihlášení komukoli, kdo se dostane na server
account-guests-hint = Mohou hledat, zobrazovat náhledy a stahovat, ale nemohou nic přidávat, měnit ani mazat a nevidí Nastavení.

## SignIn.vue

signin-create-title = Vytvořte si účet
signin-title = Přihlášení
signin-setup-lede = Tento server zatím nemá žádný účet. Ten, který tu vytvoříte, smí všechno; účty pro další lidi přidáte později v Nastavení → Účet.
signin-overlay-lede = Procházení je tu otevřené všem, ale změny vyžadují účet.
signin-with = Přihlásit se přes { $provider }
signin-or-password = nebo jménem a heslem
signin-username = Uživatelské jméno
signin-name = Jméno
signin-new-password = Nové heslo
signin-password = Heslo
signin-password-rule = Alespoň 8 znaků.
signin-confirm-password = Potvrzení hesla
signin-mismatch = Hesla se neshodují.
signin-setup-code = Instalační kód
signin-setup-code-hint-before = Kód, který tento server dostal v{ " " }
signin-setup-code-hint-after = .
signin-creating = Vytvářím…
signin-signing-in = Přihlašuji…
signin-create = Vytvořit účet
signin-submit = Přihlásit se
signin-keep-browsing = Pokračovat v procházení
signin-directory-hint = Použijte své jméno a heslo z adresáře { $directory }.
signin-forgot-1 = Zapomněli jste heslo k Hoardu? Na serveru spusťte{ " " }
signin-forgot-2 = { " " }(v Dockeru{ " " }
signin-forgot-3 = ) a získáte nové.

## SetupWizard.vue (první návštěva nového serveru)

setup-title = Kde má být vaše knihovna?
setup-lede = Vyberte na serveru složku, do které bude Hoard ukládat vaše balíčky, nebo ho nasměrujte na složku s balíčky, které už máte. Další umístění, i na jiných discích, můžete přidat později v Nastavení.
setup-lede-desktop = Vyberte složku, do které bude Hoard ukládat vaše balíčky, nebo ho nasměrujte na složku s balíčky, které už máte. Další umístění, i na jiných discích, můžete přidat později v Nastavení.
setup-unreachable = Nepodařilo se spojit se serverem: { $error }
setup-unreachable-desktop = Hoard neodpovídá: { $error }
setup-try-again = Zkusit znovu
setup-loading = Načítám…
setup-start = Založit knihovnu
setup-ready = Vaše knihovna je připravená
setup-managed-before = Nové balíčky se ukládají do{ " " }
setup-managed-after = . První přidáte přetažením souborů, složek nebo archivů kamkoli na stránku.
setup-watched-before = Hoard sleduje{ " " }
setup-watched-after = ; balíčky se zobrazují, jak je nachází. Chcete-li také nahrávat, přidejte v Nastavení → Umístění knihovny knihovnu, kterou Hoard spravuje.
setup-open = Otevřít knihovnu

## ConnectApp.vue (aplikace jako Claude žádá o přihlášení)

connect-title = Připojit { $app }?
connect-lede-1 = { " " }bude moci prohledávat, stahovat, popisovat a štítkovat vše v tomto Hoardu jako{ " " }
connect-lede-2 = . Nemůže spravovat účty ani přístupové tokeny.
connect-back-before = Vrátíte se na{ " " }
connect-back-after = . Odpojit ji můžete kdykoli v Nastavení → Účet.
connect-allow = Povolit
connect-cancel = Zrušit
connect-failed = Tuto aplikaci nelze připojit
connect-go-library = Přejít do knihovny
connect-checking = Kontroluji…

## InsecureNotice.vue (řádek zobrazený nad obyčejným http://)

insecure-text = Tohle je obyčejné http://, a pár věcí potřebuje HTTPS: 3D s WebGPU, moderní schránka, instalace Hoardu jako aplikace a konektory Claude.
insecure-switch = Přepnout na https://
insecure-set-up = Nastavit HTTPS
insecure-dismiss = Už nezobrazovat

## FormatWizard.vue (Nastavení → Soubory obchodů → učení Hoardu soubor obchodu)

formatwizard-too-big = Tento soubor je příliš velký na to, aby se četl jako soubor obchodu.
formatwizard-not-json = Tento soubor není platný JSON, takže se zobrazuje jako text.
formatwizard-read-failed = Soubor se nepodařilo přečíst: { $error }
formatwizard-back-title = Vrátit se k formuláři?
formatwizard-back-body = Změny provedené v TOML se ztratí: formulář formát napíše znovu z toho, co má.
formatwizard-back-to-form = Zpět k formuláři
formatwizard-miss-name = Formát nepřijímá soubory s názvem { $file }. Přidejte ho do Souborů.
formatwizard-miss-not-the-stores = To, podle čeho formát soubory obchodu poznává, v tomto souboru není. Změňte Rozpoznat podle.
formatwizard-miss-no-title = V tomto souboru nenachází žádný název. Vyberte název.
formatwizard-miss-short-name = jiný název
formatwizard-miss-short-not-the-stores = není z obchodu
formatwizard-miss-short-no-title = bez názvu
formatwizard-saved-packs =
    Uloženo: { $file }. { $packs ->
        [one] { $packs } balíček se znovu načítá.
        [few] { $packs } balíčky se znovu načítají.
       *[other] { $packs } balíčků se znovu načítá.
    }
formatwizard-saved = Uloženo: { $file }.
formatwizard-save-failed = Formát se nepodařilo uložit: { $error }
formatwizard-zip-failed = Zip se nepodařilo vytvořit: { $error }
formatwizard-stop-title = Přestat učit tento soubor?
formatwizard-stop-body = Formát se neuloží.
formatwizard-stop = Přestat
formatwizard-label = Průvodce formátem
formatwizard-changing = Úprava formátu { $name }
formatwizard-starting-from = Vychází z formátu { $name }
formatwizard-title = Naučte Hoard soubor obchodu
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }v{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Upravit jako TOML
formatwizard-file-reference = Soubor pro orientaci.
formatwizard-click-json = Klikněte v souboru na { $field }.
formatwizard-click-text = Klikněte v souboru na { $field }, nebo označte text.
formatwizard-pick-first = Vpravo vyberte pole a pak tady klikněte na jeho hodnotu.
formatwizard-loading = Načítám…
formatwizard-the-format = Formát
formatwizard-syntax-before = Syntaxe je popsaná v{ " " }
formatwizard-syntax-after = .
formatwizard-store = Obchod
formatwizard-store-placeholder = Název obchodu, například Kenney
formatwizard-files = Soubory
formatwizard-recognize = Rozpoznat podle
formatwizard-recognize-json = Klíč, který mají jen soubory obchodu
formatwizard-recognize-text = Řádek, který mají jen soubory obchodu
formatwizard-recognize-hint = Jeden způsob na řádek; stačí kterýkoli z nich. Potřeba, když mají stejný název i jiné soubory.
formatwizard-seller = Prodejce
formatwizard-seller-placeholder = Když ho soubor neuvádí
formatwizard-tags = Štítky
formatwizard-tags-placeholder = Pro vše, co popisuje, oddělené čárkami
formatwizard-fields = Pole
formatwizard-selector = Selektor pole { $field }
formatwizard-clear = Vymazat pole { $field }
formatwizard-pick = Vybrat
formatwizard-what-it-reads = Co přečte
formatwizard-read-title = Název
formatwizard-read-seller = Prodejce
formatwizard-read-license = Licence
formatwizard-read-url = Stránka v obchodě
formatwizard-read-kind = Druh
formatwizard-read-tags = Štítky
formatwizard-read-origin = Místo vzniku
formatwizard-read-size = Velikost
formatwizard-also = Tento soubor čte i { $format }. Tento formát by se zkusil jako první.
formatwizard-library =
    { $files ->
        [one] { $files } soubor
        [few] { $files } soubory
       *[other] { $files } souborů
    } v { $packs ->
        [one] { $packs } balíčku
        [few] { $packs } balíčcích
       *[other] { $packs } balíčcích
    } { $files ->
        [one] má
        [few] mají
       *[other] má
    } název, který přijímá. Vyzkoušeno: { $tried }, přečteno: { $read }.
formatwizard-library-none = Žádný soubor v knihovně nemá název, který přijímá.
formatwizard-share = Sdílet ho
formatwizard-share-hint = Zip s formátem, tímto souborem jako ukázkou a tím, co se z ukázky přečte, uspořádaný jako repozitář Hoardu, abyste ho mohli poslat a dostali ho všichni. Nejdřív z ukázky odstraňte vše, co by se sdílet nemělo.
formatwizard-sample = Ukázka ke sdílení
formatwizard-download = Stáhnout { $file }
formatwizard-close = Zavřít
formatwizard-use = Použít tady
formatwizard-cancel = Zrušit
