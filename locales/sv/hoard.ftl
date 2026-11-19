## Serverfel

error-account-not-found = kontot hittades inte
error-asset-gone = resursen hittades inte
error-asset-not-found = resursen { $id } hittades inte
error-blob-not-found = bloben { $hash } hittades inte
error-collection-not-found = samlingen { $id } hittades inte
error-connection-not-found = anslutningen hittades inte
error-derivative-not-found = härledd fil ({ $kind }) av { $id } hittades inte
error-file-in-pack-not-found = { $path } i paketet { $id } hittades inte
error-format-not-found = formatet { $file } hittades inte
error-frame-sheet-aseprite-only = bara Aseprite-filer har ett bildark
error-frame-sheet-frames = det här har en enda bildruta, eller för många för att lägga ut
error-location-not-found = platsen { $id } hittades inte
error-maintenance-task-not-found = underhållsuppgiften { $id } hittades inte
error-open-upload-not-found = den öppna uppladdningen { $id } hittades inte
error-original-not-found = originalfilen för { $id } hittades inte
error-pack-not-found = paketet { $id } hittades inte
error-preview-too-big = bilden är för stor för att göra en lätt förhandsvisning av
error-project-not-found = projektet { $name } hittades inte
error-saved-search-not-found = den sparade sökningen { $id } hittades inte
error-sign-in-first = logga in först (eller skicka en åtkomsttoken)
error-those-assets-not-found = de resurserna hittades inte
error-thumbnail-not-found = miniatyrbilden för { $id } hittades inte
error-token-not-found = token hittades inte
error-upload-not-found = uppladdningen { $id } hittades inte
error-pack-needs-name = ett paket behöver ett namn
error-no-library = det finns ingen biblioteksplats att lagra filer på än: lägg till en hanterad plats i Inställningar → Biblioteksplatser
error-upload-failed = uppladdningen misslyckades: { $reason }
error-upload-no-files = inga filer i uppladdningen
error-analyze-nothing-to-do = inget att göra: rules=false och LLM är avstängd
error-too-many-assets = för många resurser i en begäran
error-too-many-items = för många objekt i en begäran
error-not-a-hash = inte en blake3-hash i hex
error-merge-into-tag = into måste vara en tagg
error-nothing-to-merge = inget att slå ihop
error-selection-size = ange mellan 1 och { $max } resurs-id:n
error-collection-needs-name = en samling behöver ett namn
error-collection-exists = det finns redan en samling som heter { $name }
error-collection-empty = samlingen { $name } är tom
error-conversion-failed = kunde inte konvertera till { $format }: { $reason }
error-max-size = max måste vara en tvåpotens från 64 till 16384
error-replace-ext = ext måste vara en kort filändelse
error-empty-file = tom fil
error-imitation-silent = Inget i inspelningen sticker ut från bakgrunden. Försök igen närmare mikrofonen.
error-replace-duplicate = den konverterade filen är en dubblett av en som redan finns i paketet; paketfilen slogs ihop med den
error-ai-address = AI-adressen ska börja med http:// eller https://
error-hosted-key = Ange API-nyckeln för { $provider }
error-hosted-address = Bildsökningstjänstens adress ska börja med http:// eller https://
error-local-llm-action = action är list, download eller delete, och de två sista behöver en modell
error-role = En roll är view, edit eller admin
error-admin-only = bara en administratör kan göra det här
error-view-only = det här kontot kan titta men inte ändra något: en administratör kan ändra det i Inställningar → Konto
error-portal-only = logga in i portalen för att hantera konton och token
error-not-from-portal = den här begäran kom inte från Hoard-portalen
error-account-name = Välj ett namn på högst 64 tecken
error-password-length = Använd ett lösenord på minst { $min } tecken
error-too-many-codes = För många felaktiga koder: vänta { $seconds } sekunder
error-already-set-up = Det finns redan ett konto: logga in
error-too-many-tries = För många felaktiga försök: vänta { $seconds } sekunder
error-directory-unavailable = Katalogen kunde inte kontrollera ditt lösenord: { $reason }
error-password-elsewhere = Ditt lösenord hanteras av katalogen eller leverantören du loggar in med
error-current-password = Ditt nuvarande lösenord stämmer inte
error-account-source = Ett konto loggar in med Hoard (internal), en katalog (ldap) eller OAuth
error-account-exists = Det finns redan ett konto med det namnet
error-own-role = En annan administratör kan ändra din roll
error-remove-self = Du kan inte ta bort ditt eget konto medan du är inloggad på det
error-token-name = Döp token efter det som ska använda den, med högst 80 tecken
error-token-role = En token kan inte göra mer än kontot som skapar den
error-wrong-setup-code = Det är inte installationskoden som servern fick i HOARD_SETUP_CODE
error-wrong-password = Namnet och lösenordet stämmer inte överens
error-folder-missing = { $path } finns inte (en disk som inte är monterad?)
error-s3-path = en S3-plats ser ut som s3://bucket
error-s3-address = S3-adressen måste börja med http:// eller https://
error-s3-header = ett extra S3-huvud ser ut som Name=value, och ”{ $header }” är inte ett sådant
error-keychain = kunde inte spara det i systemets nyckelring: { $reason }
error-not-built = den här kopian av Hoard är byggd utan { $feature }
error-openai-auth-unreachable = kunde inte nå auth.openai.com
error-openai-unexpected = oväntat svar från OpenAI
error-openai-answered = OpenAI svarade { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID-token är inte en JWT
error-chatgpt-id-token-issuer = ID-token kommer från { $issuer }, inte { $expected }
error-chatgpt-id-token-audience = ID-token är avsedd för en annan app
error-chatgpt-id-token-expired = ID-token har gått ut
error-chatgpt-id-token-nonce = ID-token hör inte till den här inloggningen
error-chatgpt-no-state = adressen har inget state; kopiera hela adressen
error-chatgpt-not-started = den här inloggningen startades inte här, eller så startades den för mer än 15 minuter sedan. Starta den igen från Inställningar.
error-chatgpt-canceled = inloggningen avbröts
error-chatgpt-refused = ChatGPT nekade inloggningen: { $reason }
error-chatgpt-no-code = adressen har ingen inloggningskod; kopiera hela adressen
error-chatgpt-other-registration = ChatGPT svarade för en annan registrering av den här servern; starta inloggningen igen
error-chatgpt-other-registration-desktop = ChatGPT svarade för en annan kopia av Hoard; starta inloggningen igen
error-chatgpt-not-registered = ChatGPT slutförde inte registreringen av Hoard; starta inloggningen igen
error-chatgpt-code-used = inloggningskoden har redan använts eller har gått ut; logga in igen
error-chatgpt-refused-code = ChatGPT nekade inloggningen ({ $code })
error-chatgpt-no-id-token = ChatGPT skickade ingen ID-token
error-chatgpt-plan-not-allowed = Inloggad, men det var inte tillåtet att använda abonnemanget. Logga in igen och tillåt det, eller använd en API-nyckel i stället.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = inte inloggad på ChatGPT; logga in från Inställningar → AI-taggning
error-chatgpt-refresh-later = ChatGPT kunde inte förnya inloggningen just nu ({ $code }); det försöks igen senare
error-chatgpt-signed-out-problem = ChatGPT loggade ut den här servern ({ $code }). Logga in igen för att fortsätta tagga med ditt abonnemang.
error-chatgpt-signed-out-problem-desktop = ChatGPT loggade ut Hoard ({ $code }). Logga in igen för att fortsätta tagga med ditt abonnemang.
error-chatgpt-signed-out = ChatGPT loggade ut den här servern ({ $code }); logga in igen från Inställningar → AI-taggning
error-chatgpt-signed-out-desktop = ChatGPT loggade ut Hoard ({ $code }); logga in igen från Inställningar → AI-taggning
error-openai-api-unreachable = kunde inte nå api.openai.com
error-openai-broke-off = svaret från OpenAI bröts av
error-chatgpt-kept-refusing = OpenAI fortsatte att neka ChatGPT-inloggningen; logga in igen från Inställningar → AI-taggning
error-chatgpt-request-failed = begäran misslyckades
error-chatgpt-cut-off = svaret klipptes av ({ $reason })
error-chatgpt-cut-off-no-reason = svaret klipptes av (ingen orsak angavs)
error-openai-ended-early = svaret från OpenAI tog slut innan det var komplett
error-chatgpt-usage-limit = ditt ChatGPT-abonnemangs användning för Hoard är slut för tillfället. Se eller ändra gränsen på { $url }
error-chatgpt-not-eligible = det här ChatGPT-kontot eller den här arbetsytan kan inte använda sitt abonnemang i andra appar. Använd en API-nyckel i stället.
error-chatgpt-usage-unavailable = ChatGPT kunde inte kontrollera abonnemangets användning just nu; det försöks igen senare
error-chatgpt-unsupported = användning via ChatGPT-abonnemang stöder inte { $param } med den här modellen
error-chatgpt-unsupported-images = användning via ChatGPT-abonnemang stöder inte { $param } med den här modellen; välj en modell som tar emot bilder, eller stäng av att skicka miniatyrbilder
error-chatgpt-unsupported-part = användning via ChatGPT-abonnemang stöder inte en del av begäran med den här modellen
error-chatgpt-port = port är den som webbläsaren når Hoard på
error-chatgpt-paste-address = klistra in hela adressen, som börjar med http://127.0.0.1
error-too-many-hashes = högst { $max } hashar åt gången
error-credits-pack = paketet { $key }
error-credits-collection = samlingen { $key }
error-format-too-long = formatet är för långt
error-not-started = servern har inte startat
error-not-started-desktop = Hoard har inte startat
error-not-format-file = inte en fil för ett format: { $name }
error-format-name = namnet behöver en bokstav eller siffra från A till Z eller 0 till 9
error-format-miss-name = formatets filer tar inte med { $file }
error-format-miss-store = formatet tar inte med { $file } för butikens (se require)
error-format-miss-title = formatet hittar ingen titel i { $file }
error-cant-download-as = .{ $ext }-filer kan inte laddas ner som { $format }
error-only-images-resize = bara bilder kan skalas om
error-https-no-certificate = Klistra in eller ladda upp ditt certifikat och dess nyckel först
error-https-no-cert-found = Inget certifikat hittades: klistra in texten som börjar med -----BEGIN CERTIFICATE-----
error-https-no-key-found = Ingen privat nyckel hittades: klistra in texten som börjar med -----BEGIN PRIVATE KEY-----
error-https-several-keys = Det finns mer än en privat nyckel: klistra bara in den som hör till certifikatet
error-https-key-password = Nyckeln är skyddad med ett lösenord: ange den utan (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Certifikatet är inte giltig PEM
error-https-key-not-pem = Den privata nyckeln är inte giltig PEM
error-https-key-mismatch = Nyckeln hör inte till certifikatet. Om du angav en kedja ska serverns certifikat stå först
error-https-unusable = Certifikatet eller nyckeln kan inte användas
error-https-expired = Certifikatet gick ut { $date }
error-https-not-yet-valid = Certifikatet är inte giltigt förrän { $date }
error-https-unreadable = Det är inte ett certifikat som Hoard kan läsa
error-https-bad-name = ”{ $name }” är inte en adress som ett certifikat kan innehålla
error-https-test-timed-out = testanslutningen överskred tidsgränsen
error-https-test-failed = testanslutningen misslyckades
error-https-no-ca = Hoard har inte skapat sin certifikatutfärdare än
error-https-unavailable = HTTPS är inte tillgängligt på den här servern
error-https-certificate-mode = Ett certifikat hör ihop med ”Mitt certifikat”
error-https-test = Certifikatet fungerade inte i en testanslutning: { $reason }
error-nothing-to-retry = det finns inget där som kan försökas igen
error-ai-unreachable = kunde inte nå { $url }
error-ai-key-refused = { $url } nekade API-nyckeln ({ $status })
error-ai-answered = { $url } svarade { $status }: { $reason }
error-ai-not-json = { $url }/models svarade inte med JSON
error-model-server-refused = modellservern: { $reason }
error-no-model-server = ingen modellserver svarar på { $url }
error-not-llama-router = { $url } är inte en llama.cpp-server i routerläge, så den kan inte ladda ner modeller
error-model-download = kunde inte ladda ner { $model }: { $reason }
error-model-delete = kunde inte radera { $model }: { $reason }
error-unknown-kind = okänd resurstyp { $kind }
error-watched-is-folder = en bevakad plats är en mapp på den här servern
error-cant-reach = kan inte nå { $path }: { $reason }
error-folder-full-path = ange mappens fullständiga sökväg
error-cant-create = kan inte skapa { $path }
error-cant-write = kan inte skriva till { $path }
error-cant-read = kan inte läsa { $path }
error-overlaps-location = det överlappar platsen { $name } ({ $path })
error-overlaps-data = det överlappar Hoards datamapp ({ $path })
error-overlaps-cache = det överlappar Hoards cachemapp ({ $path })
error-not-watched = { $name } är inte en bevakad mapp
error-watched-no-kinds = en bevakad mapp tar inte emot nya filer, så den har inga typer att ändra
error-keep-catch-all = gör en annan plats till uppsamlingsplatsen i stället
error-location-holds-files = { $name } innehåller fortfarande filer: flytta dem till de andra platserna först (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } kan inte nås, så Hoard kan inte avgöra om den fortfarande innehåller filer
error-relocate-running = filer flyttas redan mellan platser; försök igen när det är klart
error-watched-not-moved = { $name } är en bevakad mapp: Hoard flyttar inte dess filer
error-no-other-location = det finns ingen annan biblioteksplats att flytta filerna till: lägg till en först
error-full-path = ange en fullständig sökväg
error-pick-folder = välj en mapp
error-task-running = ”{ $task }” körs redan
error-token-account-gone = kontot för den här token finns inte längre, så den har inga favoriter eller sökningar
error-favorites-too-many = högst 10 000 åt gången
error-saved-search-name = en sparad sökning behöver ett namn på högst 80 tecken
error-saved-search-too-long = sökningen är för lång för att sparas
error-texture-pattern-map = Mönstret för texturnamn behöver { "{" }map{ "}" }, annars skulle alla kartor i ett material få samma namn
error-texture-pattern-fields = Mönstret för texturnamn kan använda { "{" }name{ "}" }, { "{" }map{ "}" } och { "{" }size{ "}" }
error-texture-name-chars = Texturnamn får inte innehålla < > : " / \ | ? eller *
error-texture-map-type = Det finns ingen karttyp som heter { $role }
server-role-base-color = färg
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = grovhet
server-role-gloss = glans
server-role-metalness = metallighet
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = höjd
server-role-bump = bump
server-role-cavity = kavitet
server-role-specular = specular
server-role-opacity = opacitet
server-role-emissive = emission
server-role-translucency = genomskinlighet
error-texture-suffix-missing = Ge kartor av typen { $map } ett suffix
error-texture-suffix-chars = Suffixet för kartor av typen { $map } får inte innehålla < > : " / \ | ? eller *
error-texture-ending-missing = Varje egen ändelse behöver ändelsen som ska sökas efter
error-texture-ending-chars = Egna ändelser får inte innehålla < > : " / \ | ? eller *
error-texture-ending-twice = { $ending } finns två gånger bland dina ändelser
error-oauth-no-client-id = Länken har inget client_id.
error-oauth-unknown-app = Appen är inte registrerad här (längre). Börja ansluta igen från appen.
error-oauth-redirect = Appen bad om att skickas tillbaka till en adress som den inte har registrerat.
error-oauth-response-type = Hoard delar bara ut inloggningskoder (response_type=code).
error-oauth-pkce = Appen måste använda PKCE med S256 för att logga in på Hoard.
error-light-preview-images = bara bilder har en lätt förhandsvisning
error-no-light-preview = ingen lätt förhandsvisning: { $reason }
error-no-frame-sheet = inget bildark: { $reason }
error-thumbnail-source = okänd källa för miniatyrbild: { $source }
error-thumbnail-too-big = bilden är för stor för en miniatyrbild
error-thumbnail-models-only = bara modeller kan få en miniatyrbild som ritas av en webbläsare
error-not-a-picture = inte en bild som Hoard kan läsa: { $reason }
error-project-name = ett projekt behöver ett namn på högst 200 tecken
error-project-too-big = högst en miljon resurser i ett projekt
error-project-too-many-at-once = högst 100 000 åt gången
error-server-name = Välj ett namn på högst { $max } tecken, på en rad
error-tool-setting = { $tool }: välj av, automatiskt eller ange en sökväg
error-ai-model = Välj modellen som AI-tjänsten ska använda
error-public-address = Den offentliga adressen ska börja med http:// eller https://
error-update-hook-address = Adressen att anropa vid uppdateringar ska börja med http:// eller https://
error-update-hook-auth = Authorization-huvudet får inte innehålla radbrytningar eller andra styrtecken
error-no-soundfont = Det finns ingen SoundFont på { $path } på servern
error-no-soundfont-desktop = Det finns ingen SoundFont på { $path }
error-workers = Bakgrundsarbetare: mellan 1 och { $max }
error-ai-batch = Resurser per AI-begäran: mellan 1 och 200
error-conversion-cache = Konverterade nedladdningar: minst 100 MB
error-conversion-keep-days = Konverteringsregler: behåll originalen i 1 till { $max } dagar
error-conversion-target = Konverteringsregler: Hoard kan inte konvertera filer till { $format }
error-conversion-no-source = Konverteringsregler: varje regel behöver minst ett format att konvertera
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Konverteringsregler: .{ $ext }-filer kan inte konverteras till { $format }
error-conversion-twice = Konverteringsregler: .{ $ext } finns i två regler
error-conversion-cant-undo = Den här filen konverterades inte av en regel, eller så har dess original tagits bort.
error-conversion-path-taken = En annan fil ligger nu på { $path }, så originalet kan inte flyttas tillbaka dit.
error-limits-zero = Gränser kan inte vara noll
error-settings-object = förväntade ett JSON-objekt med inställningar
error-bad-setting = felaktig inställning: { $reason }
error-auth-default-role = Standardrollen är view, edit eller admin
error-auth-session-hours = Sessioner varar mellan 1 timme och ett år
error-auth-settings = felaktiga autentiseringsinställningar: { $reason }
error-ldap-settings = felaktiga LDAP-inställningar: { $reason }
error-ldap-no-server = Lägg till katalogserverns adress
error-ldap-base-dn = Ange var personernas poster finns (bas-DN)
error-ldap-username-attribute = Ange vilket attribut som innehåller namnet personer loggar in med
error-ldap-bind-dn = Ange kontot som slår upp personer, eller slå upp dem anonymt
error-ldap-ca-cert = Certifikatutfärdaren är inte ett PEM-certifikat (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = En katalogservers adress är tom
error-ldap-server-form = { $server }: en katalogservers adress är host:port, ldap://… eller ldaps://…
error-ldap-not-starttls = { $server }: det är LDAPS, inte StartTLS
error-ldap-no-certificate = inget certifikat i den
error-ldap-password-again = Ange lösenordet för kontot som slår upp personer igen: det sparade skickas bara till de servrar det angavs för
error-ldap-unreachable = ingen katalogserver kunde nås ({ $reasons })
error-ldap-no-anonymous = katalogen tillåter inte att personer slås upp anonymt ({ $reason })
error-ldap-bind-failed = kontot som slår upp personer kunde inte logga in ({ $reason })
error-ldap-search = söker efter { $name } under { $base }
error-ldap-group-search = söker efter grupperna för { $name } under { $base }
error-ldap-several = { $count } poster i katalogen matchar { $name }: snäva in användarfiltret
error-ldap-password-check = katalogen ville inte kontrollera lösenordet för { $name } ({ $reason })
error-ldap-wrong-password = Katalogen känner inte till { $name } med det lösenordet
error-oauth-client-id = Ange klient-ID:t som leverantören skapade för Hoard
error-oauth-addresses = Auktoriserings- och tokenadresserna ska börja med https://
error-oauth-userinfo-address = Adressen för användarinformation ska börja med https://
error-oauth-username-claim = Ange vilket claim som innehåller personens namn
error-oauth-secret-again = Ange klienthemligheten igen: den sparade skickas bara till tokenadressen den angavs för
error-oauth-issuer = Utfärdaren (issuer) ska börja med https://
error-oauth-discovery-unreachable = Kunde inte nå { $url }: { $reason }
error-oauth-discovery-status = { $url } svarade { $status }
error-oauth-discovery-bad = { $url } är inte ett discovery-dokument
error-oauth-discovery-incomplete = { $url } anger inte auktoriserings- och tokenadresserna
error-signin-no-group = { $name } är inte med i en grupp som får använda Hoard
error-signin-other-source = Det finns redan ett Hoard-konto som heter { $name } och som loggar in på ett annat sätt
error-signin-other-subject = Det finns redan ett Hoard-konto som heter { $name } för någon annan hos leverantören
error-signin-no-account = { $name } har inget Hoard-konto än: en administratör kan lägga till det i Inställningar → Konto
error-signin-bad-name = Det namnet kan inte vara ett Hoard-konto (högst 64 tecken)
error-oauth-off = Inloggning med OAuth är inte påslagen
error-oauth-bad-authorization-address = auktoriseringsadressen är inte giltig
error-oauth-refused = Leverantören loggade inte in dig: { $reason }
error-oauth-no-code = Leverantören skickade tillbaka dig utan en inloggning
error-oauth-expired = Inloggningen tog för lång tid eller startades någon annanstans: försök igen
error-oauth-no-claim = Leverantören sa inte vem du är (inget { $claim }-claim)
error-oauth-no-sub = Leverantören sa inte vem du är (inget sub- eller id-claim)
error-oauth-token-unreachable = kunde inte nå leverantörens tokenadress
error-oauth-token-refused = Leverantören ville inte slutföra inloggningen: { $reason }
error-oauth-no-access-token = leverantören skickade ingen åtkomsttoken
error-oauth-userinfo-unreachable = kunde inte nå leverantörens adress för användarinformation
error-oauth-userinfo-refused = Leverantören ville inte säga vem som loggade in ({ $status })
error-oauth-userinfo-not-json = leverantörens användarinformation är inte JSON
error-format-no-name = name är tomt
error-format-no-files = files är tomt: ange vilka filer formatet läser, till exempel ["*.json"]
error-format-empty-selector = tom
error-format-unopened-bracket = en avslutande hakparentes utan en inledande
error-format-unclosed-bracket = en hakparentes är inte stängd
error-format-empty-step-dots = ett tomt steg (två punkter i rad, eller en punkt i någon av ändarna)
error-format-unclosed-except = !( är inte stängd
error-format-unclosed-filter = [ är inte stängd
error-format-after-filters = { $rest } efter nyckeln och filtren för { $step }
error-format-except-needs-glob = !(…) ska stå efter en nyckel med * i
error-format-except-not-glob = !(…) ska stå efter en nyckel med * i, inte { $key }
error-format-empty-step = ett tomt steg
error-format-not-a-filter = [{ $filter }] är inte ett filter: använd [], [key=value|other], [array], [string], [number], [bool] eller [object]
error-format-filter-needs-value = [{ $filter }] behöver en nyckel och ett värde
error-store-search-empty = inget att söka efter; skriv paketets namn
error-update-hook-not-set = ange först en adress att anropa
error-upload-stalled = uppladdningen av { $path } har stannat; skicka den delen igen
error-upload-busy = { $path } skrivs redan
error-upload-incomplete = { $count } filer är ofullständiga, t.ex. { $files }
error-upload-no-files-listed = inga filer
error-upload-not-part = { $path } ingår inte i uppladdningen { $id }, eller så är uppladdningen stängd
error-bad-path = felaktig sökväg
error-upload-part-offset = { $offset } är inte där en del av { $path } börjar
error-upload-interrupted = uppladdningen avbröts: { $reason }
error-upload-part-long = del { $part } av { $path } är längre än { $len } byte
error-upload-part-short = del { $part } av { $path }: fick { $got } byte, förväntade { $len }
error-upload-file-long = { $path } är större än angivet
error-upload-file-short = { $path }: fick { $got } byte, förväntade { $len }
error-upload-complete = uppladdningen är redan klar
error-already-imported = redan importerat som ”{ $name }”

## Servertext

server-store-place-readme = README
server-store-place-folders = mappar
server-store-why-linked = Länkad i paketets { $place }
server-store-why-same-title = Samma titel som paketets { $place }
server-store-why-by = Av { $author }, som nämns i paketets { $place }
server-store-why-meshes = Ungefär lika många mesher som paketet ({ $count })
server-store-why-textures = Ungefär lika många texturer som paketet ({ $count })
server-chatgpt-revoke-failed = Utloggad här, men OpenAI kunde inte meddelas. För säkerhets skull kan du koppla från Hoard i ChatGPT under Inställningar → Säkerhet och inloggning → Logga in med ChatGPT.
server-chatgpt-signed-in-title = Inloggad på ChatGPT
server-chatgpt-signed-in = Hoard är inloggat. AI-taggningen kan nu använda ditt ChatGPT-abonnemang.
server-chatgpt-signed-in-as = Hoard är inloggat som { $email }. AI-taggningen kan nu använda ditt ChatGPT-abonnemang.
server-chatgpt-not-finished-title = Inloggningen slutfördes inte
server-chatgpt-other-computer = Om du startade inloggningen från Hoard på en annan dator kopierar du den här sidans adress från adressfältet och klistrar in den i Hoards inställningar där.
server-back-to-settings = Tillbaka till Inställningar
server-issues-pack = Paket som inte kunde läggas till
server-issues-pack-blurb = Radera paketet och lägg till det igen, eller öppna det för att se vad det innehåller.
server-issues-damaged = Lagrade filer som är skadade eller borta
server-issues-damaged-blurb = Hittade av Kontrollera filer efter skador. Återställ dem från en säkerhetskopia eller lägg till deras paket igen, och kör sedan Kontrollera filer efter skador.
server-issues-folder = Bevakade mappar som inte kan läsas
server-issues-folder-blurb = Deras paket lämnas som de är tills mappen kan läsas igen.
server-issues-missing = Bevakade paket vars mapp är borta
server-issues-missing-blurb = Varje paket behålls i 3 dagar ifall mappen bara är offline, och tas sedan bort.
server-issues-job = Bakgrundsjobb som misslyckades
server-issues-job-blurb = Ett nytt försök kör jobbet igen från början.
server-issues-file = Filer som inte kunde öppnas
server-issues-file-blurb = Hoard kunde inte öppna eller lagra dem. Ett bevakat paket försöker igen vid nästa genomsökning; en fil som redan fanns i biblioteket behöll det den hade innan.
server-issues-unpack = Arkiv som inte kunde packas upp
server-issues-unpack-blurb = Varje arkiv behålls i sitt paket som en vanlig fil, så inget i det finns i biblioteket.
server-issues-read = Filer som Hoard inte förstod sig på
server-issues-read-blurb = De finns i biblioteket, men utan de uppgifter, taggar eller den miniatyrbild som läsningen ger.
server-issues-render = Modeller som miniatyrrenderaren misslyckades med
server-issues-render-blurb = De har ingen miniatyrbild. Att öppna en i visaren kan ändå fungera, och kan spara en.
server-issues-convert = Modeller som Blender inte kunde konvertera
server-issues-convert-blurb = De har ingen GLB-kopia, så de kan inte visas i 3D.
server-issues-unreal = Unreal-paketfiler som inte kunde konverteras
server-issues-unreal-blurb = Paketets Unreal-sida har resten av dess paketfiler.
server-issues-reformat = Filer som en konverteringsregel inte kunde konvertera
server-issues-reformat-blurb = De lämnas som de var.
server-issues-materials = Paket vars material inte kunde räknas ut
server-issues-materials-blurb = Alla texturer finns där; de är bara inte grupperade i material.
server-issues-material-thumb = Material utan miniatyrbild
server-issues-material-thumb-blurb = Materialet hittades, men förhandsvisningen kunde inte ritas.
server-issues-store-files = Butiksfiler som inte kunde läsas
server-issues-store-files-blurb = Filer från butiken som ett paket kom från, till exempel en Megascans-JSON, som namnger och taggar det.
server-issues-scan-again = Sök igenom igen
server-issues-retry = Försök igen
server-issues-reanalyze = Analysera paketet igen
server-issues-convert-again = Konvertera igen
server-issues-look-again = Titta igen
server-issues-read-again = Läs igen
server-issues-blob-missing = Den lagrade filen är borta.
server-issues-blob-damaged = Den lagrade filen stämmer inte med det som lagrades: den är skadad.
server-issues-folder-gone = Mappen ”{ $folder }” hittades inte.
server-job-a-pack = ett paket
server-job-ingest = Tar in { $pack }
server-job-retag = Taggar om { $pack }
server-job-llm-tag = Taggar { $pack } med AI
server-job-convert = Konverterar { $pack } med Blender
server-job-unreal = Konverterar Unreal-paketfiler i { $pack }
server-job-reformat = Konverterar filer i { $pack } enligt konverteringsreglerna
server-job-scan = Söker igenom en bevakad mapp
server-job-relocate = Flyttar filer mellan biblioteksplatser
server-job-looks = Räknar ut färger och utseende
server-job-sounds = Lyssnar på ljud
server-job-embed = Lär sig vad bilder visar
server-job-hear = Lär sig hur ljud låter
server-job-materials = Letar efter material i { $pack }
server-job-read-files = Läser filerna i { $pack }
server-job-store-lookup = Slår upp en butikssida
server-maintenance-backup = Säkerhetskopiera databasen
server-maintenance-backup-blurb = En konsekvent kopia av databasen (paket, taggar, anteckningar, samlingar och inställningar) i mappen backups bredvid den. Bibliotekets filer ingår inte: de ändras aldrig när de väl har lagrats, så vilken filsynkronisering som helst säkerhetskopierar dem.
server-maintenance-scrub = Kontrollera filer efter skador
server-maintenance-scrub-blurb = Läser tillbaka varje lagrad fil och jämför den med dess fingeravtryck, för att hitta filer som har förstörts eller försvunnit från en disk eller NAS. Tar en stund med ett stort bibliotek.
server-maintenance-reanalyze = Analysera alla paket igen
server-maintenance-reanalyze-blurb = Läser alla filer igen med den här versionen av Hoard: miniatyrbilder, förhandsvisningar, detaljer och regeltaggar skapas på nytt. Dina egna ändringar finns kvar; AI:n tillfrågas inte igen.
server-maintenance-clear-cache = Rensa cachade konverteringar
server-maintenance-clear-cache-blurb = Tar bort filerna som sparats från konverteringar med ”Ladda ner som” (de skapas igen nästa gång de efterfrågas) och det som avbrutet arbete lämnat efter sig.
server-maintenance-shrink-thumbs = Krymp miniatyrbilder
server-maintenance-shrink-thumbs-blurb = Lagrar miniatyrbilderna och modellvyerna som sparats som PNG på det sätt som Inställningar → Prestanda → Miniatyrbilder anger, vilket oftast gör dem flera gånger mindre. De med genomskinlighet, och pixelgrafik, förblir PNG.
server-maintenance-gc = Radera oanvända filer
server-maintenance-gc-blurb = Raderar lagrade filer som inget längre pekar på, till exempel filerna i paket som raderats för gott.
server-update-no-address = ingen adress att anropa har angetts
server-update-no-answer = inget svar inom 30 sekunder

## Inställningar: Utseende

appearance-language = Språk
appearance-language-auto = Webbläsarens ({ $name })

## Rot

root-unreachable = Kunde inte nå Hoard-servern: { $error }
root-unreachable-desktop = Hoard svarar inte: { $error }
root-desktop-reopen = Det här fönstret har loggats ut. Stäng Hoard och öppna det igen för att fortsätta.
root-try-again = Försök igen
root-lazy-connect = appanslutningen
window-minimize = Minimera
window-maximize = Maximera
window-restore = Återställ
window-close = Stäng

## App: biblioteket, dess övre fält och filter

app-lazy-settings = inställningarna
app-lazy-archive = arkivet
app-lazy-shortcuts = kortkommandona
app-lazy-setup = installationen
app-theme-system = Följ systemets tema
app-theme-light = Ljust tema
app-theme-dark = Mörkt tema
app-color-red = Röd
app-color-orange = Orange
app-color-yellow = Gul
app-color-green = Grön
app-color-teal = Blågrön
app-color-blue = Blå
app-color-purple = Lila
app-color-pink = Rosa
app-color-brown = Brun
app-color-sand = Sand
app-color-gray = Grå
app-color-slate = Skiffer
app-color-black = Svart
app-color-white = Vit
app-color-swatch = { $color }: resurser med mycket av färgen { $lower }
app-color-any = Valfri färg
app-color-pick = Välj en färg
app-color = Färg
app-music = Musik
app-tempo = BPM
app-tempo-min = från
app-tempo-max = till
app-tempo-min-title = Musik i det här tempot eller snabbare, i slag per minut (halva eller dubbla räknas också)
app-tempo-max-title = Musik i det här tempot eller långsammare, i slag per minut
app-key = Tonart
app-key-any = Valfri tonart
app-key-major = Dur
app-key-minor = Moll
app-key-mix = Och tonarter som går att mixa med den
app-key-mix-title = Dess parallelltonart i dur eller moll och tonarterna en kvint åt vardera hållet, som DJ:er mixar den med
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Skriv ljudet som du skulle stava det: ljud som liknar det kommer först
app-imitation-record = Gör ljudet i mikrofonen: ljud som liknar det kommer först
app-imitation-stop = Sluta spela in
app-imitation-heard = Ditt ljud
app-imitation-mic-failed = Kunde inte spela in från mikrofonen: { $error }
app-retry = Försök igen
app-undo = Ångra
app-search-failed = Sökningen misslyckades: { $error }
app-unreachable = Kunde inte nå servern: { $error }
app-unreachable-desktop = Hoard svarar inte: { $error }
app-forget-project-title = Glöm { $name }?
app-forget-project-body = Hoard slutar visa projektet där dess { $count } resurser används, tills nästa hoard sync berättar om det igen. Inget i projektet ändras.
app-forget-project-confirm = Glöm det
app-forget-project-failed = Kunde inte glömma { $name }: { $error }
app-forget-project-label = Glöm projektet { $name }
app-forget-project = Glöm det här projektet
app-save-name-like = som { $name }
app-save-name-favorites = favoriter
app-search-saved = Sparade ”{ $name }”: den finns i Filter, under Ditt
app-rank-alike = Mest lika
app-rank-color = Mest av färgen
app-rank-match = Bäst matchning
app-rank-imitation = Låter mest så
app-sort = Sortera
app-sort-newest = Nyast
app-sort-name = Namn
app-sort-largest = Störst
app-sort-oldest = Äldst
app-collection-added =
    Lade till { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    } i { $name }
app-collection-add-failed = Kunde inte lägga till i samlingen: { $error }
app-collection-removed = Tog bort { $count } från { $name }
app-collection-remove-failed = Kunde inte ta bort från samlingen: { $error }
app-ns-subject = Motiv
app-ns-style = Stil
app-ns-theme = Tema
app-ns-material = Material
app-ns-map = PBR-karta
app-ns-color = Färg
app-ns-mood = Stämning
app-ns-genre = Genre
app-ns-license = Licens
app-ns-res = Upplösning
app-ns-duration = Längd
app-ns-poly = Polygonantal
app-ns-format = Format
app-ns-weight = Vikt
app-ns-tech = Teknik
app-ns-blender = Blender-version
app-ns-depth = Bitdjup
app-ns-ucs = Ljudkategori (UCS)
app-ns-bpm = Tempo
app-ns-root = Grundton
app-ns-level = Ljudstyrka
app-ns-ue = Unreal-klass
app-ns-engine = Motor
app-ns-extracted = Återvunnen källa
app-ns-script = Skriftsystem
app-bulk-failed =
    Kunde inte ändra { $count ->
        [one] den resursen
       *[other] de { $count } resurserna
    }: { $error }
app-bulk-untagged =
    Tog bort { $tag } från { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    }
app-bulk-tagged =
    Taggade { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    } med { $tag }
app-bulk-category-set =
    Satte kategorin för { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    } till { $category }
app-bulk-category-cleared =
    Rensade kategorin för { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    }
app-credits-failed = Kunde inte skapa krediteringarna: { $error }
app-archived-assets =
    Arkiverade { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    }
app-archived-one = Arkiverade { $name }
app-open-failed = Kunde inte öppna resursen: { $error }
app-unnamed-pack = paket
app-unnamed-collection = samling
app-unnamed-asset = resurs
app-home-version = Hoard { $version } – tillbaka till biblioteket
app-home = Tillbaka till biblioteket
app-search-label = Sök resurser
app-search-placeholder = Sök namn, sökvägar, taggar
app-views = Vyer
app-library = Bibliotek
app-packs = Paket
app-packs-working = { $count } pågår
app-update-title = Hoard { $version } har släppts (den här servern kör { $current }). Hämta den nya avbildningen eller binärfilen och starta om; databasen uppgraderar sig själv.
app-update = Uppdatera
app-theme = Tema
app-sign-in = Logga in
app-archive-title = Arkiv: { $contents }
app-archive = Arkiv
app-settings = Inställningar
app-palette-title = Kommandopalett ({ $keys })
app-import-folder = Importera mapp
app-add-files = Lägg till filer
app-add-files-short = Lägg till
app-split-label = Hur mappen ska importeras
app-split-before = { "" }
app-split-after = { " " }innehåller { $count } mappar eller arkiv.
app-split-each = Importera var och en som ett eget paket ({ $count })
app-split-one = Importera som ett paket
app-cancel = Avbryt
app-filters = Filter
app-done = Klar
app-yours = Ditt
app-favorites = Favoriter
app-forget-search-label = Glöm den sparade sökningen { $name }
app-forget-search = Glöm den här sökningen
app-projects-title = Projekt som kör hoard sync härifrån, med det som deras senaste synk hade
app-projects = Projekt
app-project-synced = Synkat { $when }
app-project-synced-by = Synkat { $when } av { $who }
app-collections = Samlingar
app-no-filters = Inga filter för den här sökningen.
app-kind = Typ
app-facets-more = { $count } till
app-facets-fewer = färre
app-searching = Söker…
app-searching-live = Söker
app-asset-count =
    { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    }
app-clear-all = Rensa alla
app-save-name-label = Namn på sökningen
app-save-name-placeholder = Namnge den
app-save = Spara
app-save-search-title = Behåll sökningen i Filter, under Ditt
app-save-search = Spara sökning
app-select-title = Välj flera resurser för att tagga, kategorisera, samla, ladda ner eller arkivera dem tillsammans (X, eller Ctrl-klick)
app-select = Välj
app-tile-size = Rutstorlek
app-supporting-title = glTF-buffertar, materialbibliotek, materialens kartor, material som bara en modell använder och butiksfiler som beskriver modeller. De följer med när du laddar ner det som använder dem. Ett spritearks sidor, och de sprites som paketet även har som separata bilder, visas tillsammans med arket. En tillgång som paketet har i flera format visas en gång, i det format Hoard läser bäst.
app-supporting-hidden =
    { $count } { $count ->
        [one] stödfil
       *[other] stödfiler
    } dolda
app-supporting-show = Visa
app-supporting-shown = Inklusive stödfiler
app-supporting-hide = Dölj
app-folders = Mappar
app-folders-title = Se paketet som det låg på disken, en mapp i taget
app-folders-label = Paketets mappar
app-welcome-title = Släpp ett resurspaket var som helst på sidan
app-welcome-before = Zip-filer, tar-arkiv, mappar eller lösa filer fungerar. Hoard packar upp dem, lagrar varje fil en gång efter dess hash, läser storlekar, längder och polygonantal och taggar allt så att du kan söka efter saker som{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }eller{ " " }
app-welcome-after = .
app-welcome-import-folder = Importera en mapp
app-welcome-choose-files = Välj filer
app-none-similar = Inget annat liknar { $name } särskilt mycket än.
app-none = Inget matchar.
app-none-query = Inget matchar ”{ $query }”.
app-none-imitation = Inget uppmätt ljud låter ännu särskilt likt ”{ $word }”.
app-none-heard = Inget uppmätt ljud låter ännu särskilt likt ditt ljud.
app-none-filters = Inget matchar med de här filtren.
app-none-query-filters = Inget matchar ”{ $query }” med de här filtren.
app-clear-search-filters = Rensa sökning och filter
app-loading-more = Läser in fler
app-keys-hint = Piltangenterna flyttar, Enter öppnar, blanksteg spelar upp ett ljud, X eller Ctrl-klick väljer flera (långt tryck på pekskärm).
app-resize-label = Ändra storlek på detaljpanelen
app-resize-title = Dra för att ändra storlek · dubbelklicka för den bredd som passar
app-selected-label = Valda resurser
app-selected = { $count } valda
app-pick-assets = Välj resurser
app-download-zip = Ladda ner zip
app-credits-title = CREDITS.md för paketen de kommer från: vem som gjorde dem, deras licenser och de krediteringsrader de ber om
app-credits = Krediteringar
app-prefer-sources-title = Unreal-SoundWaves och texturer vars originalfil Hoard har återvunnit
app-prefer-sources = Återvunnen WAV/PNG i stället för .uasset ({ $count })
bag-clear = Töm urvalet
bag-glb-heading = Modeller som GLB
bag-glb-text = Varje modell som en .glb-fil som står med Y upp och är vänd framåt, med sina texturer som egna filer i en textures-mapp som modellerna delar. Allt annat följer med som det är.
bag-glb-download = Ladda ner GLB-zip
bag-plain-heading = Filer som de är
bag-plain-text = Varje fil som den importerades, med det den behöver, i en mapp för varje paket.
bag-remove = Ta bort { $name } ur urvalet
bag-more = och { $count } till
app-bulk-tag-label = En tagg att lägga till på, eller ta bort från, de valda resurserna
app-bulk-tag-placeholder = Tagg
app-bulk-tag-add-title = Lägg till taggen på alla valda resurser
app-bulk-tag-add = Lägg till
app-bulk-tag-remove-title = Ta bort taggen från alla valda resurser
app-bulk-tag-remove = Ta bort
app-bulk-category-label = Kategori för de valda resurserna (tom rensar den)
app-bulk-category-placeholder = Kategori
app-bulk-category-set-title = Ange kategorin för alla valda resurser
app-bulk-category-clear-title = Rensa kategorin för alla valda resurser
app-bulk-category-set-button = Ange
app-bulk-category-clear-button = Rensa
app-remove-from-collection = Ta bort från { $name }
app-unfavorite = Ta bort favorit
app-favorite = Favorit
app-select-all = Välj alla { $count }
app-archive-selected-title = Borta från sökningen tills de återställs eller raderas för gott, från Arkiv
app-drop-items = Släpp { $count } objekt för att lägga till dem
app-drop = Släpp för att lägga till i Hoard
app-drop-hint = Varje fil eller mapp på översta nivån blir ett eget paket. Gigabyte går bra; stora filer fortsätter om anslutningen bryts.

## Kommandopalett (CommandPalette.vue, med kommandon från App.vue)

palette-group-go-to = Gå till
palette-group-actions = Åtgärder
palette-group-theme = Tema
palette-group-kinds = Typer
palette-group-packs = Paket
palette-group-collections = Samlingar
palette-group-projects = Projekt
palette-group-saved = Sparade sökningar
palette-group-assets = Resurser
palette-hint-current = aktuellt
palette-library = Bibliotek
palette-library-keywords = resurser rutnät hem start assets grid home
palette-favorites = Favoriter
palette-favorites-keywords = stjärnmärkta stjärnor gillade starred stars liked
palette-packs = Paket
palette-archive = Arkiv
palette-archive-keywords = papperskorg raderade återställ trash recycle bin deleted restore
palette-stats = Biblioteksstatistik
palette-stats-keywords = statistik siffror storlek diskutrymme antal statistics numbers size disk space count
palette-copies = Nästan-kopior
palette-copies-keywords = dubbletter samma liknande skalade konverterade slösat utrymme duplicates same similar resized converted wasted space dedupe
palette-search = Sök
palette-add-files = Lägg till filer…
palette-add-files-keywords = ladda upp importera upload import
palette-import-folder = Importera en mapp…
palette-import-folder-keywords = ladda upp paket upload pack
palette-select-all = Välj alla { $count } som visas
palette-select-all-keywords = markera check
palette-select-several = Välj flera…
palette-select-several-keywords = flera massändring tagga arkivera bulk multiple tag archive
palette-archive-selected = Arkivera de { $count } valda
palette-archive-selected-keywords = radera ta bort papperskorg delete remove trash
palette-clear-selection = Rensa markeringen
palette-clear-filters = Rensa filter
palette-clear-filters-keywords = återställ nollställ reset
palette-hide-supporting = Dölj stödfiler
palette-show-supporting = Visa stödfiler
palette-supporting-keywords = normalkartor grovhet masker normal maps roughness masks
palette-account = Konto och åtkomsttoken
palette-account-keywords = lösenord inloggning användare token api password login user
palette-sign-out = Logga ut ({ $name })
palette-sign-out-keywords = logga ut log out logout
palette-sign-in = Logga in
palette-sign-in-keywords = logga in konto lösenord log in login account password
palette-settings = Inställningar
palette-settings-keywords = blender ffmpeg llm ai taggning unreal admin verktyg inställningar tagging tools preferences
palette-shortcuts = Kortkommandon
palette-shortcuts-keywords = hjälp tangenter help keys
palette-theme-keywords = tema utseende läge theme appearance mode
palette-theme-variant-keywords = utseende färg appearance color
palette-light-theme = Ljust tema: { $name }
palette-dark-theme = Mörkt tema: { $name }
palette-kind-all = Alla typer (sluta visa bara { $kind })
palette-kind-only = Bara { $kind }
palette-title = Kommandopalett
palette-placeholder = Sök resurser, gå till ett paket, kör ett kommando…
palette-results = Resultat
palette-searching = Söker…
palette-no-match = Inget matchar ”{ $query }”
palette-foot-move = flytta
palette-foot-run = kör
palette-foot-shortcuts = alla kortkommandon

## Kortkommandon (ShortcutsDialog.vue)

shortcuts-title = Kortkommandon
shortcuts-close = Stäng
shortcuts-anywhere = Överallt
shortcuts-palette = Kommandopalett: sök, hoppa, kör
shortcuts-search = Sök
shortcuts-this-list = Den här listan
shortcuts-escape = Stäng, rensa markeringen eller stäng detaljerna
shortcuts-library = Bibliotek
shortcuts-move = Flytta mellan rutor (eller J och K)
shortcuts-open = Öppna detaljerna
shortcuts-play-sound = Spela upp eller stoppa ett ljud
shortcuts-select = Markera eller avmarkera
shortcuts-favorite = Lägg till i eller ta bort från dina favoriter
shortcuts-key-click = klick
shortcuts-select-range = Markera ett intervall
shortcuts-select-all = Markera allt som visas
shortcuts-sound-player = Ljudspelare
shortcuts-play-pause = Spela upp eller pausa
shortcuts-skip = Hoppa bakåt eller framåt
shortcuts-start-end = Början eller slutet
shortcuts-image-inspector = Bildgranskare
shortcuts-zoom = Zooma in eller ut
shortcuts-fit = Anpassa till fönstret
shortcuts-actual-pixels = Faktiska pixlar
shortcuts-tile = Upprepa 3×3 för att kontrollera skarvarna
shortcuts-background = Nästa bakgrund
shortcuts-channel = Visa en kanal

## KindChips.vue

kinds-group = Resurstyper

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = tillagd av en person
tag-source-rule = från offlinetaggaren
tag-source-meta = uppmätt från filen
tag-source-ai = föreslagen av AI
tag-remove = Ta bort taggen { $tag }

## ActivityIndicator.vue

activity-label-idle = Bakgrundsaktivitet: inaktiv
activity-label-running = Bakgrundsaktivitet: { $running } pågår
activity-label-queued = Bakgrundsaktivitet: { $running } pågår, { $queued } i kö
activity-title = Bakgrundsaktivitet
activity-unreachable = Kan inte nå servern
activity-unreachable-desktop = Hoard svarar inte
activity-empty = Inget pågår. Importer, taggning, konverteringar och zip-nedladdningar visas här medan de arbetar.
activity-queued = i kö
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } trianglar
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Spela upp { $name }
tile-pause = Pausa { $name }
tile-favorite = Favoritmarkera { $name }
tile-in-favorites = Bland dina favoriter
tile-add-favorite = Lägg till i dina favoriter (F)
tile-more-like = Fler som { $name }
tile-more-like-this = Fler som den här
tile-select = Välj { $name }

## CollectionPicker.vue

collections-add = Lägg till i samling
collections-menu = Samlingar
collections-find-label = Hitta eller skapa en samling
collections-find-placeholder = Hitta eller namnge en ny samling
collections-create = Skapa
collections-empty = Inga samlingar än. Skriv ett namn för att skapa en.

## FolderPicker.vue

folders-list = Mappar
folders-close = Stäng
folders-browse = Bläddra…
folders-up-title = En mapp upp
folders-up = Upp
folders-this-computer = Den här datorn
folders-none = Inga mappar här.

## LocationForm.vue

locations-mode-question = Vad ska Hoard göra med den här mappen?
locations-mode-managed = Göra anspråk på den: Hoard har mina paket här
locations-mode-managed-hint = Paket som du laddar upp eller importerar kopieras hit, och Hoard håller ordning på dem. Mappen är Hoards nu, så låt innehållet vara.
locations-mode-watched = Hålla koll på den: mina paket finns redan här
locations-mode-watched-hint = Varje mapp i den blir ett paket. Hoard läser filerna där de ligger och ändrar, flyttar eller raderar dem aldrig, och plockar upp nya mappar när de dyker upp.
locations-folder = Mapp på servern
locations-folder-desktop = Mapp
locations-folder-placeholder = Mappen som dina paket ligger i
locations-folder-hint-managed = Hoard skapar den om den inte finns. I Docker är det här en mapp som du har monterat i containern.
locations-folder-hint-managed-desktop = Hoard skapar den om den inte finns.
locations-folder-hint-watched = I Docker monterar du först mappen i containern (skrivskyddad går bra) och väljer den sedan här.
locations-folder-hint-watched-desktop = Hoard läser bara den: inget i den ändras.
locations-folder-hint-cloud = En mapp som Dropbox, Google Drive, OneDrive eller iCloud Drive synkroniserar fungerar också, och likaså s3://bucket-name för en bucket i S3-lagringen.
locations-name = Namn
locations-name-placeholder-managed = Bibliotek
locations-name-placeholder-watched = Mina paket
locations-files = Filer som hamnar här
locations-catch-all = Även allt som ingen annan plats tar
locations-kinds-hint = Ett nytt pakets filer hamnar på platsen som har deras typ ikryssad. Resten hamnar på uppsamlingsplatsen.
locations-files-all = Alla, tills vidare. Lägg till fler platser senare från Inställningar för att skicka vissa typer (till exempel musik) till en annan disk.
locations-first-watched-note = Uppladdning kräver ett bibliotek som Hoard sköter. Du kan lägga till ett senare från Inställningar → Biblioteksplatser.
locations-checking = Kontrollerar mappen…
locations-add = Lägg till plats
locations-cancel = Avbryt

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Kunde inte läsa in biblioteksplatserna
locations-takes-and-rest = { $kinds } och allt annat
locations-takes-rest = Allt som de andra inte tar
locations-takes-everything = Allt
locations-takes-nothing = Inget än
locations-save-failed = Kunde inte spara platsen
locations-unwatch-title = Sluta bevaka { $name }?
locations-unwatch-body =
    Dess { $count ->
        [one] { $count } paket
       *[other] { $count } paket
    } lämnar Hoard, tillsammans med sina taggar och anteckningar. Filerna i mappen rörs inte.
locations-unwatch-body-empty = Filerna i mappen rörs inte.
locations-unwatch = Sluta bevaka
locations-remove-title = Ta bort { $name }?
locations-remove-body = Alla filer i den flyttas först till de andra platserna, efter typ; den försvinner när den är tom. Inget går förlorat.
locations-remove-body-only = Om den innehåller filer måste du först lägga till en annan plats som de kan flyttas till.
locations-remove = Ta bort
locations-removed = Tog bort { $name }
locations-emptying = Flyttar ut filerna från { $name }; den tas bort när det är klart
locations-offline-title = { $name } kan inte nås
locations-offline-body = Filerna kan inte flyttas ut medan den är offline. Ta bort den ändå? Filer som fortfarande finns på den kommer att visas som saknade.
locations-remove-anyway = Ta bort ändå
locations-remove-failed = Kunde inte ta bort { $name }
locations-rebalancing = Flyttar lagrade filer till platserna som tar deras typ
locations-rebalance-failed = Kunde inte flytta filerna
locations-scanned-never = inte genomsökt än
locations-scanned-now = genomsökt nyss
locations-scanned-minutes = genomsökt för { $mins } min sedan
locations-scanned-hours = genomsökt för { $hours } h sedan
locations-scanned-date = genomsökt { $date }
locations-scan-failed = Kunde inte söka igenom { $name }
locations-watching = Bevakar { $name }
locations-added = Lade till { $name }
locations-loading = Läser in…
locations-managed-title = Bibliotek som Hoard sköter
locations-watched-title = Bevakade mappar
locations-managed-hint = Uppladdningar och importer kopieras hit. Varje plats tar de typer som den har ikryssade; uppsamlingsplatsen tar resten.
locations-watched-hint = Mappar med paket som indexeras där de ligger. Hoard ändrar aldrig filerna i dem, och letar efter nya och borttagna mappar var 15:e minut.
locations-managed-empty = Inga än, så det finns ingenstans att lägga en uppladdning. Lägg till en nedan.
locations-reachable = Kan nås
locations-unreachable-now = Kan inte nås just nu
locations-catch-all-tag = uppsamling
locations-packs =
    { $count ->
        [one] { $count } paket
       *[other] { $count } paket
    }
locations-unreachable = kan inte nås
locations-scanning = Söker igenom…
locations-scan = Sök igenom nu
locations-edit = Redigera
locations-catch-all-locked = Gör en annan plats till uppsamlingsplats för att ändra det här
locations-edit-hint-before = Gäller filer som läggs till från och med nu. Använd
locations-edit-hint-after = nedan för att flytta det som redan har lagrats.
locations-save = Spara
locations-rebalance = Flytta lagrade filer så att de stämmer
locations-rebalance-hint = Efter att du har ändrat vad en plats tar: filer som redan har lagrats flyttas till platsen som deras typ nu hör till.
locations-add-title = Lägg till en plats
locations-add-hint = En annan disk för vissa typer av filer, eller en mapp med paket som du redan har.
locations-s3-title = S3-lagring
locations-s3-hint = En plats på s3://bucket-name är en bucket i den här tjänsten: AWS eller någon S3-kompatibel tjänst (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage på en NAS).
locations-s3-endpoint = Adress
locations-s3-endpoint-placeholder = Tomt för AWS; t.ex. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Region
locations-s3-access-key = Åtkomstnyckel-ID
locations-s3-secret-key = Hemlig åtkomstnyckel
locations-s3-headers = Extra huvuden
locations-s3-headers-placeholder = Name=value, ett per rad (till exempel för Cloudflare Access)
locations-s3-kept = Sparad (skriv för att ersätta)
locations-s3-secrets = Nycklarna och huvudena sparas med serverns inställningar och visas aldrig igen.
locations-s3-secrets-desktop = Nycklarna och huvudena sparas i den här datorns nyckelring och visas aldrig igen.
locations-s3-saved = S3-lagringen har sparats
locations-s3-save-failed = Kunde inte spara S3-lagringen
locations-add-more = Lägg till en plats…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Kunde inte läsa in Problem
issues-started =
    { $count ->
        [one] Startade igen.
       *[other] Startade { $count } saker igen.
    }
issues-retry-failed = Kunde inte försöka igen
issues-dismiss-failed = Kunde inte avfärda
issues-dismiss-all-title = Avfärda alla { $count }?
issues-dismiss-all-body = ”{ $title }” rensas. Allt som går fel igen listas igen.
issues-dismiss-all = Avfärda alla
issues-just-now = nyss
issues-minutes-ago = för { $mins } min sedan
issues-hours-ago = för { $hours } h sedan
issues-loading = Läser in…
issues-none = Inget har gått fel.
issues-action-all = { $action } (alla)
issues-open-asset = Öppna den
issues-open-pack = Visa paketet
issues-open-locations = Biblioteksplatser
issues-where-before = i
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } gång
       *[other] { $count } gånger
    }
issues-starting = Startar…
issues-dismiss-named = Avfärda { $title }
issues-dismiss = Avfärda
issues-show-fewer = Visa färre
issues-show-all = Visa alla { $count }
issues-not-listed = { $count } till listas inte.

## archive.ts

archive-anyway = Arkivera ändå
archive-failed = Kunde inte arkivera: { $error }
archive-undo = Ångra
archive-restore-failed = Kunde inte återställa: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } paket
       *[other] { $packs } paket
    } och { $assets ->
        [one] { $assets } resurs
       *[other] { $assets } resurser
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } paket
       *[other] { $packs } paket
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } resurs
       *[other] { $assets } resurser
    }
archive-count-nothing = inget

## ArchiveView.vue

archive-load-failed = Kunde inte läsa in arkivet: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Återställde { $what }
archive-purged = Raderade { $what } för gott
archive-purge-failed = Kunde inte radera: { $error }
archive-purge-title = Radera { $name } för gott?
archive-purge-pack-body =
    Dess { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    } och { $size } med filer försvinner också, utom filer som ett annat paket delar. Det går inte att ångra.
archive-purge = Radera för alltid
archive-purge-asset-body = Dess fil frigörs om ingen annan resurs delar den. Det går inte att ångra.
archive-empty-title = Töm arkivet?
archive-empty-body = { $what } raderas för gott, och deras filer frigörs. Det går inte att ångra.
archive-empty = Töm arkivet
archive-title = Arkiv
archive-intro = Arkiverade paket och resurser syns inte i sökningar, antal eller tagglistan. Återställ dem, eller radera dem för gott för att frigöra utrymmet.
archive-loading = Läser in…
archive-is-empty = Arkivet är tomt
archive-is-empty-hint = Arkivera ett paket eller en resurs så väntar det här tills du återställer det eller raderar det för gott.
archive-packs = Paket
archive-assets = Resurser
archive-asset-count =
    { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    }
archive-archived = arkiverad { $when }
archive-from = från { $pack }
archive-restore = Återställ
archive-show-more = Visa fler ({ $count } kvar)

## mine.ts

projects-favorite-add-failed = Kunde inte lägga till i dina favoriter: { $error }
projects-favorite-remove-failed = Kunde inte ta bort från dina favoriter: { $error }
projects-save-search-failed = Kunde inte spara sökningen: { $error }
projects-forget-search-failed = Kunde inte ta bort { $name }: { $error }

## projects.ts

projects-list-and = { $items } och { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } till
projects-other =
    { $count ->
        [one] { $count } annat projekt
       *[other] { $count } andra projekt
    }
projects-usage-title-one = { $name } använder det här
projects-usage-title = { $count } projekt använder det här
projects-usage-body-one = Används i { $list }: nästa hoard sync tar bort dem.
projects-usage-body = Används i { $list }: nästa hoard sync i vart och ett tar bort dem.

## uploader.ts

upload-network-error = nätverksfel
upload-gave-up = { $what }: { $error } efter { $attempts } försök
upload-failed-plain = misslyckades
upload-already-imported = redan importerat
upload-part-failed = uppladdningen misslyckades ({ $status })
upload-batch = omgång med { $count } filer
upload-batch-failed = omgången misslyckades ({ $status })
upload-finish = slutför uppladdningen

## UploadQueue.vue

upload-eta-seconds = { $s } s kvar
upload-eta-minutes = { $min } min kvar
upload-eta-hours = { $h } h kvar
upload-queue = Importer
upload-importing = Importerar { $count } paket
upload-bytes-of = { $sent } av { $total }
upload-rate = { $size }/s
upload-waiting = Väntar
upload-unpacking = Uppladdat, packar upp
upload-already = Finns redan i biblioteket
upload-stopped = Stoppad
upload-files-of = { $done } av { $total } filer
upload-resumed = återupptagen, { $size } fanns redan
upload-skipped = Exakt de här filerna har redan importerats som ”{ $name }”, så de hoppades över.
upload-error = { $error }. Dina framsteg sparas: försök igen, eller släpp samma filer igen senare.
upload-retry = Försök igen
upload-import-again = Importera igen ändå
upload-dismiss = Avfärda
upload-cancel = Avbryt

## PacksView.vue

packs-rename-failed = Kunde inte byta namn: { $error }
packs-collection-deleted = Raderade samlingen { $name }
packs-delete-failed = Kunde inte radera: { $error }
packs-status-uploading = Laddar upp (filer som tagits emot hittills)
packs-status-ingesting = Packar upp och taggar
packs-status-analyzing = LLM:en taggar
packs-status-ready = Klart
packs-status-failed = Misslyckades
packs-retag-failed = Kunde inte börja tagga om: { $error }
packs-read-files-failed = Kunde inte läsa paketets filer: { $error }
packs-context-load-failed = Kunde inte läsa in paketets butiksuppgifter: { $error }
packs-undo-store-failed = Kunde inte ta bort butikssidan: { $error }
packs-context-save-failed = Kunde inte spara butiksuppgifterna: { $error }
packs-unreal-load-failed = Kunde inte läsa in status för Unreal-konverteringen: { $error }
packs-unreal-discard-title = Kasta originalen?
packs-unreal-discard-body = De ursprungliga .uasset-filerna i { $name } försvinner när de har konverterats. Data som bara Unreal använder, till exempel materialgrafer, går förlorad för gott.
packs-unreal-discard-confirm = Konvertera och kasta
packs-unreal-convert-failed = Kunde inte starta konverteringen: { $error }
packs-unreal-fact-converted = { $done } konverterade från Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal-paketfil
       *[other] { $left } Unreal-paketfiler
    }
packs-unreal-fact-both = { $done } konverterade från Unreal, { $left } fortfarande paketfiler
packs-thumbs-canceled = Avbröts efter { $done } av { $total }
packs-thumbs-drew =
    { $count ->
        [one] Ritade { $count } modell
       *[other] Ritade { $count } modeller
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniatyrbild ersattes.
       *[other] { $lead }: { $improved } miniatyrbilder ersattes.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniatyrbild ersattes, { $failed } kunde inte ritas.
       *[other] { $lead }: { $improved } miniatyrbilder ersattes, { $failed } kunde inte ritas.
    }
packs-archived = Arkiverade { $name } och dess { $count } resurser
packs-blobs-bad = { $count } filer klarade inte sin senaste integritetskontroll.
packs-collections = Samlingar
packs-browse-named = Bläddra i { $name }
packs-collection-name = Samlingens namn
packs-save = Spara
packs-cancel = Avbryt
packs-assets =
    { $count ->
        [one] { $count } resurs
       *[other] { $count } resurser
    }
packs-updated = Uppdaterad { $date }
packs-browse = Bläddra
packs-collection-download = Ladda ner (.zip)
packs-collection-credits = Krediteringar
packs-collection-credits-title = Vem som gjorde paketen som resurserna kommer från, deras licenser och de krediteringsrader de ber om
packs-rename = Byt namn
packs-delete = Radera
packs-packs = Paket
packs-empty = Inga paket än. Släpp en zip-fil eller en mapp var som helst på sidan för att lägga till ett.
packs-find = Hitta ett paket
packs-find-placeholder = Hitta ett paket efter namn, butik, licens eller tagg
packs-sort = Ordning
packs-sort-new = Nyast först
packs-sort-old = Äldst först
packs-sort-name = Efter namn
packs-sort-size = Störst först
packs-sort-assets = Flest resurser först
packs-with-kind = Paket med
packs-state = Status
packs-state-any = Valfri status
packs-state-working = Pågår
packs-state-failed = Misslyckades
packs-state-untagged = Inte beskrivna än
packs-store = Butik
packs-store-any = Valfri butik
packs-store-from = Från { $store }
packs-store-none = Ingen butikssida
packs-shown-of = { $shown } av { $total }
packs-clear = Rensa
packs-no-match = Inga paket matchar.
packs-clear-all = Rensa sökningen och filtren
packs-pack-name = Paketets namn
packs-thumbs-drawing = Ritar miniatyrbilder i den här webbläsaren…
packs-thumbs-drawing-count = Ritar miniatyrbilder i den här webbläsaren: { $done } av { $total }…
packs-thumbs-stopped = Stoppades: { $error }
packs-thumbs-none = Alla modeller här har redan en miniatyrbild som ritats med alla dess texturer.
packs-dismiss = Avfärda
packs-license = Licens: { $license }
packs-license-tier = Licens: { $license } ({ $tier })
packs-license-not-found = hittades inte
packs-from-before = Från
packs-from-after = { "" }
packs-added = Tillagt { $date }
packs-download = Ladda ner paketet (.zip)
packs-store-details = Butiksuppgifter
packs-more-named = Fler åtgärder för { $name }
packs-more = Mer
packs-rename-more = Byt namn…
packs-retag-rules = Kör offlinetaggaren igen
packs-retag-llm = Kör LLM-taggaren igen
packs-make-thumbs = Skapa miniatyrbilder i den här webbläsaren
packs-make-thumbs-title = Öppna varje modell i en dold 3D-visare, en i taget, och använd dess bild som miniatyrbild där den är bättre än serverns
packs-make-thumbs-title-desktop = Öppna varje modell i en dold 3D-visare, en i taget, och använd dess bild som miniatyrbild där den är bättre än den som Hoard gjorde
packs-read-files = Läs paketets filer igen
packs-credits = Ladda ner krediteringar
packs-credits-title = CREDITS.md: vem som gjorde paketet, dess licens och den krediteringsrad det ber om
packs-unreal-open = Konvertera Unreal-innehåll…
packs-archive = Arkivera paketet
packs-unreal-intro = Hoard läser själv Unreal-paketfiler och lagrar innehållet som vanliga filer: texturer som PNG eller EXR, ljud som WAV, material som JSON. Den konverterade filen tar paketfilens plats och behåller dess taggar, länkar och samlingar.
packs-unreal-converted =
    { $count ->
        [one] konverterad
       *[other] konverterade
    }
packs-unreal-converted-kept =
    { $count ->
        [one] konverterad, { $kept } med originalet kvar
       *[other] konverterade, { $kept } med originalet kvar
    }
packs-unreal-pending =
    { $count ->
        [one] inte granskad än
       *[other] inte granskade än
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] väntar på Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] utan något vanligt format
    }
packs-unreal-failed =
    { $count ->
       *[other] misslyckades
    }
packs-unreal-reasons = Varför paketfiler är kvar
packs-unreal-editor-before = Mesher och animationer kräver Unreal Editor. På en dator med Unreal installerat kör du
packs-unreal-editor-command = hoard unreal-worker --editor <sökväg till UnrealEditor-Cmd>
packs-unreal-editor-after = ; det exporterar dem som FBX och laddar upp resultaten hit.
packs-unreal-discard = Kasta de ursprungliga .uasset-filerna när de har konverterats
packs-unreal-discard-hint = Frigör utrymmet. Data som bara Unreal använder, till exempel materialgrafer, går förlorad.
packs-unreal-drop = Radera paketfiler utan något vanligt format
packs-unreal-drop-hint = Blueprints, nivåer och andra resurser som bara motorn kan använda.
packs-unreal-convert = Konvertera
packs-context-hint-before = Var paketet kom ifrån och vad säljaren säger om det. LLM-taggaren läser det här med varje fil, så en butikstext som ”120 is- och eldbesvärjelser” hjälper den att namnge
packs-context-hint-after = .
packs-store-auto = Hittades automatiskt: paketet nämner själv den här sidan.
packs-store-not-this = Inte den här
packs-lookup-label = Sök i butikerna efter
packs-lookup-placeholder = Paketets namn i butiken
packs-lookup-searching = Söker…
packs-lookup-search = Sök i butiker
packs-lookup-hide = Dölj
packs-lookup-clues = Från paketets filer: { $clues }.
packs-lookup-also = Sökte även efter { $queries }.
packs-lookup-quoted = ”{ $query }”
packs-lookup-nothing = Inget på Fab, itch.io, Kenney eller OpenGameArt liknar det. Försök med färre eller andra ord.
packs-lookup-use = Använd
packs-lookup-searched = Sökte { $date }.
packs-lookup-again = Sök igen
packs-lookup-again-after = .
packs-lookup-unreachable = Kunde inte nå { $stores } den här gången.
packs-lookup-also-look = Titta även på
packs-lookup-links-and = { " " }och{ " " }
packs-lookup-browser-only = (där kan du bara söka i din webbläsare); klistra in sidans adress nedan.
packs-lookup-open = Hitta i butiker…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Säljare
packs-seller-placeholder = Vem som gjorde det
packs-license-label = Licens
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Licensnivå
packs-license-tier-placeholder = Personal, Professional…
packs-store-page = Butikssida
packs-store-text = Beskrivning från butiken
packs-store-text-placeholder = Klistra in annonstexten, eller lämna tomt för att hämta den från butikssidan.
packs-store-text-fetch = När du sparar hämtas beskrivningen från butikssidan.
packs-readme = README som hittades i paketet
packs-save-retag = Spara och kör LLM-taggaren igen

## AssetDetail.vue

asset-lazy-3d-viewer = 3D-visaren
asset-lazy-text = texten
asset-lazy-text-viewer = textvisaren
asset-lazy-image-viewer = bildvisaren
asset-lazy-material-preview = förhandsvisningen av materialet
asset-lazy-material-viewer = materialvisaren
asset-lazy-sprite-player = spritespelaren
asset-lazy-sheet-view = visaren för spriteark
asset-rel-texture = textur
asset-rel-buffer = geometridata
asset-rel-material = materialfil
asset-rel-animation = animation
asset-rel-library = länkad fil
asset-rel-guess = hör troligen ihop med
asset-rel-sidecar = butiksuppgifter
asset-rel-variant = variant
asset-rel-sprite = sprite
asset-rel-sample = sampling
asset-role-map = karta för { $role }
asset-maps-count =
    { $count ->
        [one] { $count } karta
       *[other] { $count } kartor
    }
asset-needs = Behöver
asset-animations = Animationer
asset-sprites = Sprites
asset-used-by = Används av
asset-loose-sprites = Även som separata filer
asset-in-sheet = I ett spriteark
asset-other-formats = Andra format
asset-shown-as = Visas i biblioteket som
asset-conversion-failed = konverteringen misslyckades
asset-as-converting = Konverterar…
asset-as-converting-percent = Konverterar… { $percent } %
asset-as-queued = I kö…
asset-download = Ladda ner
asset-as-label = Ladda ner som
asset-as-original = { $format } (originalformat)
asset-as-pack-files = Paketets egna filer
asset-as-size = Storlek
asset-as-full-size = Full storlek
asset-as-up-to = Upp till { $size } px
asset-as-hint = Välj en storlek eller ett annat format
asset-meta-size = Storlek
asset-meta-length = Längd
asset-meta-sample-rate = Samplingsfrekvens
asset-meta-channels = Kanaler
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bitdjup
asset-meta-bits = { $bits } bitar
asset-meta-level = Nivå
asset-meta-level-value = topp { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = topp { $peak } dBFS, RMS { $rms } dBFS, överstyrs
asset-meta-loop = Loop
asset-meta-loop-value = { $start } till { $end }
asset-meta-playback = Uppspelning
asset-meta-one-shot = Engångsljud
asset-meta-envelope = Förlopp
asset-meta-envelope-times = attack { $attack }, avklingning { $tail }
asset-meta-envelope-hit = slag
asset-meta-envelope-swell = crescendo
asset-meta-envelope-sustained = ihållande
asset-meta-tone = Klang
asset-meta-tone-value = ljushet { $khz } kHz, brus { $noise } %
asset-meta-pitch = Tonhöjd
asset-meta-pitch-rises =
    { $octaves ->
        [one] stiger { $octaves } oktav
       *[other] stiger { $octaves } oktaver
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] sjunker { $octaves } oktav
       *[other] sjunker { $octaves } oktaver
    }
asset-meta-loops-cleanly = Loopar sömlöst
asset-meta-tempo = Tempo
asset-meta-tempo-heard = ungefär { $bpm } BPM, efter gehör
asset-meta-tempo-heard-or = ungefär { $bpm } BPM (eller { $alt }), efter gehör
asset-meta-key = Tonart
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, efter gehör
music-key-major = { $note }-dur
music-key-minor = { $note }-moll
asset-meta-root-note = Grundton
asset-meta-cue-markers = Cue-markörer
asset-meta-library = Bibliotek
asset-meta-keywords = Nyckelord
asset-meta-title = Titel
asset-meta-artist = Artist
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Generator
asset-meta-everything-with = Visa allt med taggen { $tag }
asset-meta-saved-with = Sparad med
asset-meta-color-mode = Färgläge
asset-meta-layers = Lager
asset-meta-range = Omfång
asset-meta-hdr = HDR (förhandsvisningen är tonmappad)
asset-meta-format = Format
asset-meta-large-document = Stort dokument (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Skapad med generativ AI
asset-meta-ai-edited = Redigerad med generativ AI
asset-meta-ai-how-metadata = metadata
asset-meta-ai-how-generator = generatorinställningar
asset-meta-ai-how-iptc = IPTC-typ av digital källa
asset-meta-ai-says = { $made }, enligt { $how }
asset-meta-ai-by-says = { $made } ({ $by }), enligt { $how }
asset-meta-provenance = Ursprung
asset-meta-c2pa = Har C2PA Content Credentials
asset-meta-compression = Komprimering
asset-meta-note = Anteckning
asset-meta-maps = Kartor
asset-meta-sizes =
    { $count ->
        [one] Storlek
       *[other] Storlekar
    }
asset-meta-triangles = Trianglar
asset-meta-vertices = Hörnpunkter
asset-meta-meshes = Mesher
asset-meta-materials = Material
asset-meta-animations = Animationer
asset-meta-skins = Skins
asset-meta-unreal-asset = Unreal-resurs
asset-meta-unreal-saved-with = { $class }, sparad med UE { $engine }
asset-meta-package = Paketfil
asset-meta-cooked = Cookad (spelbygge): förhandsvisningar och källor är borttagna
asset-meta-tiling = Upprepning
asset-meta-seamless = Sömlös på båda axlarna
asset-meta-analysis-error = Kunde inte läsa hela filen: { $error }
asset-copy-blocked = Kunde inte kopiera: webbläsaren blockerade det
asset-save-error = Kunde inte spara: { $error }
asset-tag-removed = Tog bort { $tag }
asset-undo = Ångra
asset-play-error = Kunde inte spela upp filen: { $error }
asset-copied-hash = Kopierade hash
asset-copied-toml = Kopierade toml
asset-copied-id = Kopierade id
asset-copied-link = Kopierade länk
asset-origin-before = Den här filen var
asset-origin-class-engine = , en { $class } sparad av UE { $engine }.
asset-origin-class = , en { $class }.
asset-origin-engine = { " " }sparad av UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor exporterade den { $date }.
asset-origin-editor-notes = Unreal Editor exporterade den { $date } ({ $notes }).
asset-origin-hoard = Hoard läste ut den ur paketfilen { $date }.
asset-origin-hoard-notes = Hoard läste ut den ur paketfilen { $date } ({ $notes }).
asset-origin-download = Ladda ner originalet .{ $ext }
asset-origin-discarded = Den ursprungliga paketfilen kastades efter konverteringen.
asset-details = Resursdetaljer
asset-in-pack = i
asset-previous = Föregående resurs
asset-next = Nästa resurs
asset-favorite = Favorit
asset-favorite-on-title = Bland dina favoriter (F tar bort den)
asset-favorite-off-title = Lägg till i dina favoriter (F)
asset-share = Dela
asset-share-title = Dela en länk till den här resursen (den som öppnar den loggar först in i Hoard)
asset-layout-panel = Visa i en sidopanel
asset-layout-page = Visa helsida
asset-layout-page-title = Visa helsida: en stor förhandsvisning bredvid detaljerna
asset-close = Stäng detaljerna
asset-rename-title = Klicka för att byta namn
asset-name = Namn
asset-views-alt = Modellen från fyra håll
asset-inspect-aria = Granska { $name } i helskärm
asset-inspect-title = Granska: zooma, upprepa, kanaler
asset-playback-position = Uppspelningsposition
asset-playback-value = { $current } av { $duration }
asset-pause = Pausa
asset-play = Spela upp
asset-waveform-drawing = Ritar vågform…
asset-waveform-skipped = Ingen vågform för så här stora filer
asset-play-animation = Spela upp som animation
asset-show-views = Visa arket med fyra vyer som taggaren såg
asset-palette = Färger i bilden
asset-swatch-title = { $hex }, { $percent } % av bilden. Hitta resurser med den här färgen
asset-swatch-aria = Hitta resurser med { $hex }, { $percent } % av den här bilden
asset-more-like-this = Fler som den här
asset-desc-label = Din beskrivning
asset-desc-placeholder = Vad det är, var du skulle använda det…
asset-save = Spara
asset-cancel = Avbryt
asset-desc-family = Sparas i alla { $count } varianter
asset-desc-edit = Redigera beskrivning
asset-desc-write = Skriv en beskrivning
asset-desc-ai = Skriven av AI
asset-desc-file = Från filen
asset-desc-store-before = Från
asset-desc-store-page = { $store }-sidan
asset-desc-store-page-unnamed = butikssidan
asset-desc-store-after = (hela paketet)
asset-less = Mindre
asset-more = Mer
asset-tags = Taggar
asset-tag-add-label = Lägg till en tagg
asset-tag-add-placeholder = Lägg till en tagg, t.ex. mood:tense
asset-tag-add = Lägg till
asset-apply-family = Tillämpa ändringar av taggar, kategori och beskrivning på alla { $count } varianter
asset-collections = Samlingar
asset-collection-remove = Ta bort från { $name }
asset-collection-add-another = Lägg till i en annan
asset-collection-add = Lägg till i samling
asset-used-in = Används i
asset-used-in-title = Visa vad { $name } har
asset-used-in-synced = synkat { $when }
asset-maps = Kartor
asset-show-more = Visa { $count } till
asset-materials-heading =
    { $count ->
        [one] Material
       *[other] Material
    }
asset-material-maps-aria = { $name }: kartor
asset-variants = Varianter ({ $count } tagningar)
asset-fit-animations = Animationer som passar ({ $count })
asset-fit-characters = Figurer den passar ({ $count })
asset-copies = Identiska kopior
asset-copies-note = Samma byte lagras en gång; de här paketen innehåller också filen.
asset-near-copies = Nästan-kopior
asset-near-copies-note = Samma sak i en annan storlek, samplingsfrekvens eller ett annat format. Den bästa kopian är markerad.
asset-near-copies-best = Samma sak i en annan storlek, samplingsfrekvens eller ett annat format. Den här är den bästa kopian.
asset-near-best = bäst
asset-unreal-converted = Konverterad från Unreal
asset-rule-converted = Konverterad av en regel
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = En konverteringsregel skapade denna { $format } den { $date } från
asset-rule-kept-until = Originalet sparas till { $date }.
asset-rule-put-back = Lägg tillbaka originalet
asset-rule-put-back-title = Lägga tillbaka { $file }?
asset-rule-put-back-body = Den konverterade filen tas bort och originalet tar dess plats, med sina taggar, samlingar och länkar. Konverteringsreglerna låter den vara därefter.
asset-rule-put-back-done = { $file } är tillbaka
asset-rule-let-go = Originalet togs bort när tiden som Inställningar sparar original hade gått ut.
asset-unreal-package = Från Unreal-paketfilen
asset-ue-needs-editor-before = Väntar på Unreal Editor: { $reason }. På en dator med Unreal installerat kör du
asset-ue-needs-editor-after = så kommer den tillbaka som FBX.
asset-ue-editor-path = sökväg till UnrealEditor-Cmd
asset-ue-unsupported = Behålls som paketfil: { $reason }.
asset-ue-failed = Konverteringen misslyckades: { $reason }
asset-recovered = Originalet .{ $ext } låg lagrat i den här paketfilen, och Hoard plockade ut det.
asset-recovered-download = Ladda ner .{ $ext }
asset-ue-meshes-before = Mesher och animationer lagras i Unreals eget format. Konvertera paketet (paketsidan, Mer) och kör sedan
asset-ue-meshes-after = bredvid en installation av Unreal Editor för att få FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } asset registry-tagg
       *[other] { $count } asset registry-taggar
    }
asset-file = Fil
asset-bytes = Byte
asset-source = Källa
asset-path = Sökväg
asset-path-open-folder = Se vad mer som finns i { $folder }
asset-download-maps =
    { $count ->
        [one] Ladda ner { $count } karta
       *[other] Ladda ner alla { $count } kartor
    }
asset-download-linked =
    { $count ->
        [one] Ladda ner med { $count } länkad fil
       *[other] Ladda ner med { $count } länkade filer
    }
asset-download-only-title = Bara den här filen, utan filerna den använder
asset-download-only = Bara den här filen
asset-include-animations = Ta med animationer i .zip-filen
asset-include-character = Ta med figuren { $name }
asset-copy-toml = Kopiera hoard.toml-post
asset-copy-id = Kopiera ID
asset-teach = Lär Hoard den här filen
asset-teach-title = Beskriv den här butikens fil så att Hoard läser den (Inställningar → Butiksfiler)
asset-archive = Arkivera
asset-archive-title = Borta från sökningen tills den återställs eller raderas för gott, från Arkiv

## ImageViewer.vue

image-channel-red = Röd kanal
image-channel-green = Grön kanal
image-channel-blue = Blå kanal
image-channel-alpha = Alfakanal
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Rutig bakgrund (K växlar)
image-bg-black-title = Svart bakgrund (K växlar)
image-bg-gray-title = Grå bakgrund (K växlar)
image-bg-white-title = Vit bakgrund (K växlar)
image-bg-checker = rutig bakgrund
image-bg-black = svart bakgrund
image-bg-gray = grå bakgrund
image-bg-white = vit bakgrund
image-inspect-aria = Granska { $name }
image-preview-size = förhandsvisning { $width }×{ $height }, { $bytes }
image-preview = förhandsvisning
image-resolution = Upplösning
image-full-title = Läs in hela bilden på { $width }×{ $height } (F); det gör även inzoomning förbi förhandsvisningen
image-full-loading = Läser in full upplösning…
image-full-failed = Full upplösning lästes inte in: försök igen
image-full = Full upplösning
image-zoom = Zoom
image-zoom-out-title = Zooma ut (−)
image-zoom-out = Zooma ut
image-zoom-in-title = Zooma in (+)
image-zoom-in = Zooma in
image-fit-title = Anpassa (0)
image-fit = Anpassa
image-actual-title = Faktiska pixlar (1)
image-view = Visa
image-tile-title = Upprepa 3×3 för att kontrollera skarvarna (T)
image-tile = Upprepa
image-channels = Kanaler
image-close = Stäng (Esc)
image-failed = Bilden kunde inte läsas in.
image-loading = Läser in…
image-key-gestures = Rulla eller nyp för att zooma · dra för att panorera · dubbelklicka för att zooma in eller anpassa
image-key-tile = upprepa
image-key-background = bakgrund
image-key-full = full upplösning
image-key-channels = kanaler

## SpritePlayer.vue

sprite-frame-aria = { $name }, bildruta { $frame } av { $count }
sprite-failed = Den här kan inte spelas upp.
sprite-loading = Läser in bildrutor…
sprite-previous = Föregående bildruta
sprite-next = Nästa bildruta
sprite-pause = Pausa animationen
sprite-play = Spela upp animationen
sprite-animation = Animation
sprite-all-frames = Alla bildrutor
sprite-grid-title = Hur arket delas upp i bildrutor
sprite-columns = Kolumner
sprite-rows = Rader
sprite-fps-aria = Bildrutor per sekund
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
        [one] { $count } sida
       *[other] { $count } sidor
    }
sheet-animations =
    { $count ->
        [one] { $count } animation
       *[other] { $count } animationer
    }
# The box that narrows the list of sprites by name.
sheet-find = Hitta en sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Visar { $shown } av { $total }
sheet-no-match = Ingen sprite har det i namnet.
sheet-failed = Det här arket kan inte läsas.
sheet-loading = Läser in arket…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoards eget format: sidorna som de är, sprites vridna åt vilket håll som helst och beskurna, med animationer och varje bildrutas tid.
sheet-note-texturepacker = Sprites vridna medurs och beskurna, med animationer efter namn, så som PixiJS och Phaser läser dem.
sheet-note-clockwise = Sprites vridna medurs och beskurna. Formatet rymmer inga animationer: bildrutorna känns igen på sina numrerade namn.
sheet-note-libgdx = Sprites vridna moturs och beskurna. Formatet rymmer inga animationer: bildrutorna känns igen på sina numrerade namn.
sheet-note-godot-frames = Sprites vrids aldrig och beskärningen sparas som marginaler, med animationer och varje bildrutas tid.
sheet-note-godot-tileset = Rutor som aldrig vrids eller beskärs, i ett rutnät med rutstorleken, och bilderna behålls som de är om de redan ligger i ett.
sheet-note-grid = Varje sprite upprätt på hela sin duk, i celler av samma storlek, för allt som delar upp ett ark i ett rutnät.
sheet-note-frames = Varje sprite upprätt på hela sin duk, var och en som en PNG med spritens namn.

## TextViewer.vue, TextPreview.vue och text.ts

text-lang-config = Konfiguration
text-lang-table = Tabell
text-lang-code = Kod
text-lang-script = Skript
text-lang-text = Text
text-copy-blocked = Kunde inte kopiera: webbläsaren blockerade det
text-read-aria = Läs { $name }
text-lines =
    { $count ->
        [one] { $count } rad
       *[other] { $count } rader
    }
text-show-as = Visa som
text-tree-title = Hopfällbart träd (T)
text-tree = Träd
text-pretty-title = Omindenterad (P)
text-pretty = Snygg
text-raw-title = Som den är skriven (R)
text-raw = Rå
text-document-title = Som ett dokument (D)
text-document = Dokument
text-source-title = Markdown-koden som den är skriven (R)
text-source = Källa
text-fold = Fäll ihop
text-expand-all-title = Fäll ut allt
text-expand-all = Expandera alla
text-collapse-all-title = Fäll ihop allt
text-collapse-all = Komprimera alla
text-group-text = Text
text-wrap-title = Radbryt långa rader (W)
text-wrap = Radbryt
text-copy-title = Kopiera texten (C)
text-copied = Kopierat
text-copy = Kopiera
text-download-title = Ladda ner filen
text-download = Ladda ner
text-close = Stäng (Esc)
text-truncated = Visar de första { $shown } av { $total }.
text-truncated-json = Visar de första { $shown } av { $total } (trädet behöver hela filen).
text-show-all = Visa allt
text-json-error = JSON-koden kan inte tolkas: { $error }
text-contents-aria = Innehållet i { $name }
text-loading = Läser in…
text-load-error = Kunde inte läsa in filen: { $error }
text-binary = Filen är binär, inte text, så det finns inget att läsa här. Ladda ner den för att öppna den i sitt eget verktyg.
text-key-tree = träd
text-key-pretty = snygg
text-key-raw = rå
text-key-document = dokument
text-key-source = källa
text-key-wrap = radbryt
text-key-copy = kopiera
text-key-find = sök
text-key-close = stäng
text-preview-aria = Läs { $name } i helskärm
text-preview-read = Läs

## JsonTree.vue och JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } objekt
       *[other] { $count } objekt
    }
json-keys =
    { $count ->
        [one] { $count } nyckel
       *[other] { $count } nycklar
    }
json-fold = Fäll ihop
json-unfold = Fäll ut
json-unfold-summary = Fäll ut { $summary }
json-show-more = Visa { $count } till
json-left = av { $count } kvar
json-picker-more = … { $count } till

## 3D-visare (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = 3D-visaren lästes inte in ({ $error }); det kan hjälpa att läsa in sidan igen
model-loading-viewer = Läser in 3D-visaren…
model-loading = Läser in modellen…
model-show-failed = Kunde inte visa modellen: { $error }
model-preview-failed = Kunde inte förhandsvisa modellen: { $error }
model-error-converting = servern konverterar fortfarande modellen
model-error-converting-desktop = Hoard konverterar fortfarande modellen
model-error-empty-collada = tom Collada-fil
model-error-no-loader = Ingen inläsare i webbläsaren för .{ $ext }
model-dialog = Visa { $name } i 3D
model-title-stats =
    { $triangles } trianglar · { $meshes ->
        [one] { $meshes } mesh
       *[other] { $meshes } mesher
    } · { $materials ->
        [one] { $materials } material
       *[other] { $materials } material
    }
model-triangles =
    { $count ->
        [one] { $count } triangel
       *[other] { $count } trianglar
    }
model-group-camera = Kamera
model-group-shading = Skuggning
model-group-background = Bakgrund
model-group-animation = Animation
model-group-view = Vy
model-group-show = Visa
model-frame-title = Rama in modellen (F)
model-frame = Rama in
model-reset-title = Tillbaka till startvyn (R)
model-reset = Återställ
model-turn-title = Snurra långsamt (A)
model-turn = Snurra
model-textures-title = Texturer (T)
model-textures = Texturer
model-full-res-title = Texturer i full upplösning (H): varje texturfil som den är, hur stor den än är; av ger lätta förhandsvisningar på högst 2048 px
model-full-res = Full uppl.
model-light-title = Miljöljus (E)
model-light = Ljus
model-grid-title = Golvrutnät (G)
model-grid = Rutnät
model-bg-checker = Rutig bakgrund
model-bg-dark = Mörk bakgrund
model-bg-gray = Grå bakgrund
model-bg-light = Ljus bakgrund
model-bg-env = Miljöbakgrund
model-bg-title = { $background } (K växlar)
model-pause-title = Pausa (blanksteg)
model-play-title = Spela upp (blanksteg)
model-pause = Pausa
model-play = Spela upp
model-animation-clip = Animationsklipp
model-animation = Animation
model-plays-on = Spelas på
model-rest-pose = Vilopose
model-close-title = Stäng (Esc)
model-full-screen = Helskärm
model-open-full-screen = Öppna 3D-visaren i helskärm
model-menu-fold = Fäll undan menyn
model-menu-show = Visa menyn
model-view-prefix = Vy:{ " " }
model-menu-render = Rendering
model-menu-maps = Kartor
model-menu-inspect = Granska
model-mode-final = Slutlig rendering
model-mode-raw = Rå rendering
model-mode-color = Grundfärg
model-mode-roughness = Grovhet
model-mode-metalness = Metallighet
model-mode-normal = Normal
model-mode-ao = Ocklusion
model-mode-matcap = Matcap
model-mode-uv = UV-rutmönster
model-mode-geometry = Geometrier
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Trådmodell (W), ovanpå vilken vy som helst
model-wireframe = Trådmodell
model-info-title = Sceninformation (I)
model-info = Sceninformation
model-info-close = Stäng sceninformationen
model-info-close-title = Stäng (I)
model-info-geometries = Geometrier
model-info-materials = Material
model-info-textures = Texturer
model-info-triangles = Trianglar
model-info-vertices = Hörnpunkter
model-info-meters = glTF anges i meter
model-info-no-units = Det här formatet anger inte vilka enheter det använder
model-info-height = Höjd
model-info-length = Längd
model-info-width = Bredd
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } enhet
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } enheter
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Texturen hittades inte: { $name }
model-missing-some = { $count } texturer hittades inte: { $names }
model-missing-more = { $count } texturer hittades inte: { $names }…
model-missing-count = { $count } texturer hittades inte
model-guessed = Texturerna matchades efter namn; filen namnger inga.
model-group-facing = Riktning
model-facing-turn = Vrid en kvart
model-facing-turn-title = Vrid modellen ett kvarts varv moturs, sett ovanifrån
model-facing-tip = Tippa en kvart
model-facing-tip-title = Tippa modellens ovansida ett kvarts varv framåt, för att resa upp en som ligger ner
model-facing-save = Använd för paketet
model-facing-save-title = Vrid alla modeller i { $pack } så här. Deras miniatyrbilder ritas om; nedladdningar förblir som de gjordes.
model-facing-undo = Ångra
model-facing-undo-title = Tillbaka till hur paketets modeller är vridna
model-facing-saved = Alla modeller i { $pack } är nu vända så här. Deras miniatyrbilder ritas om.
model-facing-failed = Kunde inte vrida paketets modeller: { $error }
model-keys-mouse = Dra för att kretsa · högerdra för att panorera · rulla för att zooma · dubbelklicka för att rama in
model-key-frame = rama in
model-key-reset = återställ
model-key-view = vy
model-key-wireframe = trådmodell
model-key-info = sceninformation
model-key-textures = texturer
model-key-full-res = texturer i full upplösning
model-key-light = ljus
model-key-grid = rutnät
model-key-background = bakgrund
model-key-play = spela upp

## Material (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Färg
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Grovhet
material-role-gloss = Glans
material-role-metalness = Metallighet
material-role-ao = Ambient occlusion
material-role-orm = Ocklusion, grovhet, metall
material-role-mra = Metall, grovhet, ocklusion
material-role-rma = Grovhet, metall, ocklusion
material-role-height = Höjd
material-role-bump = Bump
material-role-cavity = Kavitet
material-role-specular = Specular
material-role-opacity = Opacitet
material-role-emissive = Emission
material-role-translucency = Genomskinlighet
material-role-base-color-name = färg
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = grovhet
material-role-gloss-name = glans
material-role-metalness-name = metallighet
material-role-ao-name = ambient occlusion
material-role-orm-name = ocklusion, grovhet, metall
material-role-mra-name = metall, grovhet, ocklusion
material-role-rma-name = grovhet, metall, ocklusion
material-role-height-name = höjd
material-role-bump-name = bump
material-role-cavity-name = kavitet
material-role-specular-name = specular
material-role-opacity-name = opacitet
material-role-emissive-name = emission
material-role-translucency-name = genomskinlighet
material-maps =
    { $count ->
        [one] { $count } karta
       *[other] { $count } kartor
    }
material-dialog = Visa materialet { $name }
material-up-to = upp till { $size }
material-repeat-size = { $size } per upprepning
material-shape-sphere = Sfär
material-shape-cube = Kub
material-shape-plane = Plan
material-shape-title = { $shape } ({ $key })
material-group-shape = Form
material-group-tiling = Upprepning
material-real-title = Verklig storlek: en upprepning är { $size }, sfären är 2 m i diameter (T växlar)
material-repeat-title = Upprepa { $n }× (T växlar)
material-real = Verklig
material-repeat = { $n }×
material-height-title = Hur långt höjdkartan trycker ut ytan
material-height = Höjd
material-height-strength = Höjdstyrka
material-loading = Läser in materialet…
material-show-failed = Kunde inte visa materialet: { $error }
material-preview-failed = Kunde inte förhandsvisa materialet: { $error }
material-group-maps = Kartor
material-map-show = Visa kartan för { $map }
material-map-hide = Dölj kartan för { $map }
material-map-unused = Används inte av förhandsvisningen
material-open-full-screen = Öppna materialvisaren i helskärm
material-keys-mouse = Dra för att kretsa · högerdra för att panorera · rulla för att zooma · klicka på en karta för att stänga av den
material-key-shape = form
material-key-tiling = upprepning
material-key-turn = snurra

## theme.ts

theme-slate = Skiffer
theme-paper = Papper
theme-linen = Linne
theme-sage = Salvia
theme-lilac = Syren
theme-blush = Rodnad
theme-graphite = Grafit
theme-obsidian = Obsidian
theme-ember = Glöd
theme-moss = Mossa
theme-aubergine = Aubergine
theme-oxblood = Oxblod
theme-light-slate-blurb = Sval blågrå
theme-light-paper-blurb = Enkel neutral grå
theme-light-linen-blurb = Varm gräddvit, bärnsten
theme-light-sage-blurb = Mjuk grön
theme-light-lilac-blurb = Blek violett
theme-light-blush-blurb = Dämpad rosa
theme-dark-graphite-blurb = Neutral grå, ingen ton
theme-dark-slate-blurb = Blågrå
theme-dark-obsidian-blurb = Nästan svart, för OLED
theme-dark-ember-blurb = Varm kolgrå, bärnsten
theme-dark-moss-blurb = Skogsgrön-svart
theme-dark-aubergine-blurb = Plommonsvart, syren
theme-dark-oxblood-blurb = Vinsvart, korall

## texnames.ts

texnames-needs-map = Mönstret behöver { "{" }map{ "}" }, annars skulle alla kartor i ett material få samma namn
texnames-placeholders = Mönstret kan använda { "{" }name{ "}" }, { "{" }map{ "}" } och { "{" }size{ "}" }
texnames-bad-characters = Namn får inte innehålla < > : " / \ | ? eller *
texnames-no-suffix = Ge kartor av typen { $role } ett suffix
texnames-bad-suffix = Suffixet för kartor av typen { $role } får inte innehålla < > : " / \ | ? eller *
texnames-slot-no-suffix = Ge ”{ $slot }” ett suffix
texnames-slot-bad-suffix = Suffixet för ”{ $slot }” får inte innehålla < > : " / \ | ? eller *
texnames-ending-missing = Varje egen ändelse behöver ändelsen som ska sökas efter
texnames-ending-bad = Egna ändelser får inte innehålla < > : " / \ | ? eller *
texnames-ending-twice = { $ending } finns två gånger bland dina ändelser
texnames-slot-metallic-roughness = Metall och grovhet
texnames-slot-specular-color = Specularfärg
texnames-slot-specular-glossiness = Specular och glans
texnames-slot-clearcoat = Klarlack
texnames-slot-clearcoat-roughness = Klarlacksgrovhet
texnames-slot-clearcoat-normal = Klarlacksnormal
texnames-slot-sheen-color = Lysterfärg
texnames-slot-sheen-roughness = Lystergrovhet
texnames-slot-transmission = Transmission
texnames-slot-thickness = Tjocklek
texnames-slot-diffuse-transmission = Diffus transmission
texnames-slot-diffuse-transmission-color = Diffus transmissionsfärg
texnames-slot-iridescence = Iridescens
texnames-slot-iridescence-thickness = Iridescenstjocklek
texnames-slot-anisotropy = Anisotropi

## storeFormats.ts

storefiles-field-title = Titel
storefiles-field-title-hint = Annonsens exakta titel. Den namnger paketet, och utan den hoppas filen över.
storefiles-field-seller = Säljare
storefiles-field-seller-hint = Vem som gjorde eller säljer det.
storefiles-field-license = Licens
storefiles-field-license-hint = Som filen skriver den, till exempel CC-BY-4.0.
storefiles-field-url = Butikssida
storefiles-field-url-hint = Annonsens adress, som börjar med http:// eller https://.
storefiles-field-words = Taggar
storefiles-field-words-hint = Ord att tagga det med. Kommaseparerade värden delas upp.
storefiles-field-type = Typ
storefiles-field-type-hint = Vad det är, till exempel ”3D model” eller ”surface”. Blir också en tagg.
storefiles-field-colors = Färger
storefiles-field-colors-hint = Dess färger, som color:-taggar.
storefiles-field-id = Butiks-id
storefiles-field-id-hint = Butikens id för det: bokstäver, siffror, _ eller -.
storefiles-field-origin = Tillverkad i
storefiles-field-origin-hint = Var det tillverkades eller skannades, för beskrivningen.
storefiles-field-size = Storlek
storefiles-field-size-hint = Hur stort det är, i ord, för beskrivningen.
storefiles-field-title-name = titeln
storefiles-field-seller-name = säljaren
storefiles-field-license-name = licensen
storefiles-field-url-name = butikssidan
storefiles-field-words-name = taggarna
storefiles-field-type-name = typen
storefiles-field-colors-name = färgerna
storefiles-field-id-name = butiks-id:t
storefiles-field-origin-name = tillverkningsplatsen
storefiles-field-size-name = storleken

## StoreFormatsPanel.vue

storefiles-load-failed = Kunde inte läsa in butiksfilformaten: { $error }
storefiles-remove-title = Ta bort { $name }?
storefiles-remove-body = { $file } raderas från datamappen. Paket som redan har lästs med det behåller det som lästes.
storefiles-remove = Ta bort
storefiles-removed = Tog bort { $name }.
storefiles-remove-failed = Kunde inte ta bort { $name }: { $error }
storefiles-unread = Filer som Hoard inte läser
storefiles-loading = Läser in…
storefiles-unread-none = Inga: ingen fil i biblioteket ser ut som en oläst butiks fil.
storefiles-unread-what = Filer som finns i mer än ett paket, eller som heter som en butiks filer, och som inget format läser.
storefiles-unread-packs =
    { $count ->
        [one] { $count } paket
       *[other] { $count } paket
    }
storefiles-unread-example = t.ex. { $pack }
storefiles-teach = Lär Hoard den här filen
storefiles-formats = Format
storefiles-formats-what = Dina egna provas först och ersätter ett inbyggt format med samma namn.
storefiles-dir-before = De sparas i
storefiles-dir-after = .
storefiles-yours = Ditt
storefiles-built-in = Inbyggt
storefiles-edit = Redigera
storefiles-start-from = Utgå från det här

## TagsPanel.vue

tags-load-failed = Kunde inte läsa in taggarna: { $error }
tags-rename-title = Byt namn på { $tag }?
tags-merge-title = Slå ihop { $count } taggar?
tags-rename-body = Dess { $count } resurser får { $into } i stället, och taggningen använder det nya namnet från och med nu.
tags-merge-body = { $tags } blir { $into } på upp till { $count } resurser. Taggningen använder { $into } från och med nu.
tags-rename = Byt namn
tags-merge = Slå ihop
tags-merged =
    { $count ->
        [one] { $count } resurs är nu taggad { $into }
       *[other] { $count } resurser är nu taggade { $into }
    }
tags-merge-failed = Kunde inte slå ihop: { $error }
tags-forget-failed = Kunde inte ändra det: { $error }
tags-find = Hitta taggar
tags-find-placeholder = Hitta taggar, t.ex. rock eller set:
tags-count =
    { $count ->
        [one] { $count } tagg
       *[other] { $count } taggar
    }
tags-count-more = { $count }+ taggar
tags-what = Välj taggar som betyder samma sak och slå ihop dem, eller välj en för att byta namn på den.
tags-new-name = Nytt namn
tags-merge-into = Slå ihop till
tags-saved-as-before = Sparad som
tags-saved-as-after = { "" }
tags-cancel = Avbryt
tags-loading = Läser in…
tags-none = Inga taggar.
tags-none-containing = Inga taggar som innehåller ”{ $text }”.
tags-browse-title = Bläddra bland resurser taggade { $tag }
tags-browse = Bläddra
tags-merged-heading = Sammanslagna taggar
tags-merged-what = När taggningen kommer på något av de här gamla namnen får resursen taggen som det slogs ihop till.
tags-allow-old = Tillåt det gamla namnet igen

## LibraryStats.vue

stats-source-human = Tillagda av personer
stats-source-ai = Föreslagna av AI
stats-source-rule = Inbyggda regler
stats-source-meta = Från filerna
stats-assets = Resurser
stats-packs = Paket
stats-on-disk = På disk
stats-duplicates-title = { $count } filer är byte-identiska kopior av andra och lagras en gång
stats-saved-dedup = Sparat genom deduplicering
stats-distinct-tags = Olika taggar
stats-archive = I arkivet
stats-load-failed = Kunde inte läsa in detaljerna: { $error }
stats-counting = Räknar…
stats-by-kind = Utrymme per typ
stats-files-size = { $name }: { $count } filer, { $size }
stats-months = Tillagt det senaste året
stats-month = { $month }: { $count } resurser, { $size }
stats-tagging = Taggning
stats-tagged = Taggade
stats-untagged = { $count } utan taggar
stats-described = Beskrivna
stats-described-assets = { $count } resurser
stats-human-edited = Redigerade för hand
stats-human-edited-unit =
    { $count ->
        [one] resurs
       *[other] resurser
    }
stats-source-different = { $count } olika
stats-most-used = Mest använda
stats-show-tag = Visa resurserna med { $tag }
stats-file-types = Filtyper
stats-biggest-packs = Största paketen
stats-browse-pack = Bläddra i { $name }
stats-largest-files = Största filerna
stats-open-file = Öppna { $name }
stats-empty = Inget i biblioteket än. Lägg till ett paket så visas siffrorna här.

## UpdatesPanel.vue

updates-saved = Hoard anropar den när en uppdatering är klar
updates-saved-none = Ingen adress att anropa
updates-save-failed = Kunde inte spara
updates-call-answered = Den svarade { $result }
updates-call-refused = Den tog inte emot begäran: { $result }
updates-call-failed = Kunde inte anropa den
updates-called = Anropade { $when } för { $version }: { $result }
updates-called-failed = Anropade { $when } för { $version }: { $result }. Hoard försöker igen om en timme.
updates-latest = Hoard { $version } är den senaste versionen.
updates-out = Hoard { $version } har släppts.
updates-out-idle = Hoard { $version } har släppts; anropar inom ett par minuter, medan inget pågår.
updates-out-busy = Hoard { $version } har släppts; väntar tills inget pågår ({ $work }).
updates-busy-more = { $count } till
updates-busy-jobs =
    { $count ->
        [one] { $count } jobb väntar
       *[other] { $count } jobb väntar
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } uppladdning
       *[other] { $count } uppladdningar
    }
updates-list-separator = ,{ " " }
updates-what = När en ny version har släppts kan Hoard anropa en adress så snart inget pågår (inga uppladdningar, jobb, konverteringar eller underhåll), så att det som uppdaterar containern inte startar om den mitt i en uppladdning. Den skickar en POST med versionen i en JSON-kropp. Det här kräver versionskontrollen ovan.
updates-url = Adress att anropa
updates-url-placeholder = Ingen (t.ex. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Sparad (skriv för att ersätta)
updates-auth-placeholder = Ingen (t.ex. Bearer my-token)
updates-remove = Ta bort
updates-save = Spara
updates-call-now-title = Det som den är kopplad till körs
updates-call-now = Anropa nu
updates-what-calls = Vad kan den anropa?
updates-watchtower = Watchtowers HTTP-API
updates-watchtower-at = :{ " " }
updates-watchtower-with = , med{ " " }
updates-watchtower-token = { " " }och token från{ " " }
updates-watchtower-its = { " " }(med slutpunkten{ " " }
updates-watchtower-endpoint = { " " }påslagen).
updates-portainer = En Portainer-webhook
updates-portainer-rest = { " " }på Hoards stack eller container, med re-pull image påslaget. Token finns i URL:en.
updates-script = Ett skript på värddatorn
updates-script-listener = { " " }bakom en webhook-lyssnare (till exempel{ " " }
updates-script-runs = ) som kör{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }eller något annat som tar emot en webhook.
updates-poll-before = Eller hoppa över adressen: ett skript med en åtkomsttoken kan fråga{ " " }
updates-poll-until = { " " }tills{ " " }
updates-poll-and = { " " }är satt och{ " " }
updates-poll-after = { " " }är true, och sedan hämta och starta om Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Kunde inte läsa in Underhåll
maintenance-reanalyze-title = Analysera alla paket igen?
maintenance-reanalyze-body = Alla filer läses igen, vilket kan ta lång tid med ett stort bibliotek; paket visas som pågående tills det är deras tur. Dina egna namn, taggar och anteckningar finns kvar.
maintenance-reanalyze-confirm = Analysera igen
maintenance-gc-title = Radera oanvända filer?
maintenance-gc-body = Lagrade filer som inget i biblioteket pekar på raderas för gott. Paket i arkivet behåller sina.
maintenance-gc-confirm = Radera dem
maintenance-start-failed = Kunde inte starta ”{ $task }”
maintenance-just-now = nyss
maintenance-minutes-ago = för { $count } min sedan
maintenance-hours-ago = för { $count } h sedan
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Läser in…
maintenance-running = Körs…
maintenance-starting = Startar…
maintenance-run = Kör
maintenance-started = { $detail } · startade { $when }
maintenance-starting-detail = Startar
maintenance-finished = Klar:{ " " }
maintenance-failed = Misslyckades:{ " " }
maintenance-never-run = Har aldrig körts
maintenance-history = Tidigare körningar
maintenance-col-task = Uppgift
maintenance-col-finished = Klar
maintenance-col-took = Tog
maintenance-col-result = Resultat
maintenance-script-before = Säkerhetskopior och skadekontrollen kan också köras från ett skript:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }och{ " " }
maintenance-script-after = . De körningarna visas också här.

## SettingsPage.vue

settings-title = Inställningar
settings-loading = Läser in…
settings-load-failed = Kunde inte läsa in inställningarna: { $error }
settings-page-general = Allmänt
settings-page-general-blurb = Vad den här servern heter, för alla som använder den.
settings-page-general-blurb-desktop = Vad det här biblioteket heter.
settings-page-appearance = Utseende
settings-page-appearance-blurb = Teman som den här webbläsaren använder när den är ljus och när den är mörk. Sparas bara i den här webbläsaren.
settings-page-library = Bibliotek
settings-page-library-blurb = Vad som finns i Hoard: hur stort det är, vad det består av, hur det har vuxit och hur väl det är taggat. Arkiverade saker räknas inte.
settings-page-copies = Nästan-kopior
settings-page-copies-blurb = Filer som är samma bild, ljud eller modell i en annan storlek, samplingsfrekvens eller ett annat format, de som tar mest plats först. Den bästa i varje uppsättning är markerad; inget raderas.
copies-loading = Letar efter nästan-kopior…
copies-none = Inga nästan-kopior: varje bild, ljud och modell finns här bara en gång.
copies-load-failed = Kunde inte lista nästan-kopior: { $error }
copies-sum = Uppsättningar nästan-kopior: { $count }. Kopiorna som inte är bäst tar { $size }.
copies-set = Uppsättning { $n }
copies-set-wasted = { $size } i kopiorna som inte är bäst
copies-more = Visa fler ({ $count } kvar)
settings-page-tags = Taggar
settings-page-tags-blurb = Alla taggar i biblioteket. Slå ihop sådana som betyder samma sak, eller byt namn på en; taggningen använder det nya namnet från och med då.
settings-page-store-files = Butiksfiler
settings-page-store-files-blurb = Filerna som butiker lägger bredvid sina resurser, som Hoard läser för att namnge paket och kreditera deras skapare. Lär det filer som det inte känner till, och dela med dig av det du har lärt det.
settings-page-locations = Biblioteksplatser
settings-page-locations-blurb = Mapparna på servern som rymmer biblioteket: sådana som Hoard förvarar filer i, och mappar med paket som det indexerar där de ligger.
settings-page-locations-blurb-desktop = Mapparna som rymmer biblioteket: sådana som Hoard förvarar filer i, och mappar med paket som det indexerar där de ligger.
settings-page-tools = Verktyg
settings-page-tools-blurb = Externa program som Hoard använder för att konvertera och läsa filer.
settings-page-ai = AI-taggning
settings-page-ai-blurb = En AI-modell namnger, beskriver och taggar nya paket, utöver de inbyggda reglerna. Dess taggar markeras som föreslagna.
settings-page-search = Bild- och ljudsökning
settings-page-search-blurb = Sök bilder efter vad de visar och ljud efter hur de låter, inte bara efter namn, och hitta saker som liknar varandra till sin art, inte bara till färgen.
settings-page-adding = Lägga till paket
settings-page-adding-blurb = Vad Hoard gör med ett paket när det läggs till, förutom att läsa och tagga det.
settings-page-downloads = Nedladdningar
settings-page-downloads-blurb = Hur filer namnges i det som Hoard skickar: nedladdningar, zip-filer, verktyget hoard och AI-assistenter. Filerna i biblioteket förblir som de är.
settings-page-account = Konto
settings-page-account-blurb = Ditt lösenord, de andra personerna som kan logga in och åtkomsttoken för skript och andra program.
settings-page-auth = Autentisering
settings-page-auth-blurb = Hur personer loggar in: Hoards egna konton, en LDAP-katalog, Microsoft Active Directory eller en OAuth-leverantör, hur länge sessioner varar och vilka grupper som får vilken roll.
settings-page-connection = Anslutning
settings-page-connection-blurb = Hur webbläsare och appar når Hoard: adressen det är känt under, och HTTPS, som vissa webbläsarfunktioner kräver.
settings-page-performance = Prestanda
settings-page-performance-blurb = Hur hårt servern arbetar, hur miniatyrbilder skapas och sparas, och det mesta den tar emot.
settings-page-performance-blurb-desktop = Hur hårt Hoard arbetar, hur miniatyrbilder skapas och sparas, och det mesta det tar emot.
settings-page-issues = Problem
settings-page-issues-blurb = Det som gick fel när filer öppnades eller bearbetades: arkiv som inte gick att packa upp, filer som inte kunde läsas, jobb som misslyckades. Försök igen när orsaken är åtgärdad, eller avfärda dem.
settings-page-maintenance = Underhåll
settings-page-maintenance-blurb = Säkerhetskopiera, kontrollera efter skador och städa. En av varje körs åt gången, och du kan lämna sidan medan de körs.
settings-the-default = standardvärdet
settings-default-on = på
settings-default-off = av
settings-default-auto = automatiskt
settings-option-saved = { $name }: { $value }
settings-back-to-default = Tillbaka till standardvärdet
settings-save = Spara
settings-use = Använd
settings-status-off = Av

## SettingsPage.vue: Utseende

settings-themes = Teman
settings-look-light = Ljust
settings-look-dark = Mörkt
settings-look-group =
    { $scheme ->
        [light] Ljust tema
       *[dark] Mörkt tema
    }
settings-preview =
    Visar det { $preview ->
        [light] ljusa
       *[dark] mörka
    } temat så att du kan se det. Du använder { $used ->
        [light] ljust
       *[dark] mörkt
    }{ $system ->
        [yes] { " " }(följer systemet)
       *[other] { "" }
    }, och det kommer tillbaka när du lämnar sidan.
settings-preview-back =
    Tillbaka till { $used ->
        [light] ljust
       *[dark] mörkt
    }
settings-look-hint = Växla mellan ljust och mörkt (eller följ systemet) i det övre fältet.

## SettingsPage.vue: Verktyg

settings-blender-what = Konverterar .blend-, .dae-, .abc- och .x3d-modeller till glTF i bakgrunden, så att de får miniatyrbilder och en 3D-förhandsvisning. Paket som lades till medan det var avstängt konverteras när det är klart.
settings-blender-download = Använder en installerad Blender, eller laddar ner Blender 4.5 LTS (cirka 400 MB) från en spegel av blender.org.
settings-ffmpeg-what = Kodar MP3-, OGG- och Opus-nedladdningar, och läser ljud som Hoard inte kan avkoda själv (AAC, M4A, WMA, …).
settings-ffmpeg-download = Använder en ffmpeg i systemet med rätt kodare, eller laddar ner en statisk version (cirka 80 MB).
settings-tool-reset = { $tool }: tillbaka till standardvärdet
settings-tool-no-download = Använder en installerad { $tool }. Det finns ingen version att ladda ner för den här datorn, så installera det eller ange dess sökväg.
settings-tool-path = Sökväg till programmet { $tool }
settings-mode-off = Av
settings-mode-auto = Automatiskt
settings-mode-path = Det här programmet
settings-tool-looking = Letar…
settings-tool-downloading = Laddar ner… förloppet visas i aktivitetsindikatorn
settings-tool-ready = Klart: { $detail }
settings-soundfont = MIDI-SoundFont
settings-soundfont-what = Instrumenten som MIDI-filer spelas upp och konverteras med.
settings-soundfont-file = SoundFont-fil
settings-soundfont-placeholder = Hittad på servern, eller inbyggd
settings-soundfont-placeholder-desktop = Hittad på den här datorn, eller inbyggd
settings-soundfont-default = hittad på servern, eller inbyggd
settings-soundfont-default-desktop = hittad på den här datorn, eller inbyggd

## SettingsPage.vue: Bildsökning

settings-clip-heading = Sök efter vad bilder visar
settings-clip-reset = Bildsökning: tillbaka till standardvärdet
settings-clip-saved-on = Bildsökning: på
settings-clip-saved-off = Bildsökning: av
settings-clip-what = Skriv vad du letar efter (”mossy rock”, ”sci-fi door”, ”wooden barrel”) så dyker bilder som visar det upp bredvid namn som innehåller orden. ”Fler som den här” hittar saker av samma slag, inte bara med samma färger.
settings-clip-toggle = Slå på bildsökning
settings-clip-hint = Använder Googles SigLIP 2-modell, som körs på den här serverns processor: inget lämnar den. När du slår på den laddas modellen ner ({ $size } MB) från Hugging Face, och sedan lär den sig varje bild i bakgrunden med låg prioritet. Den tar 350 till 750 MB minne medan den arbetar eller någon söker. När du stänger av den raderas modellen men det den har lärt sig behålls, till nästa gång.
settings-clip-hint-desktop = Använder Googles SigLIP 2-modell, som körs på den här datorns processor: inget lämnar den. När du slår på den laddas modellen ner ({ $size } MB) från Hugging Face, och sedan lär den sig varje bild i bakgrunden med låg prioritet. Den tar 350 till 750 MB minne medan den arbetar eller du söker. När du stänger av den raderas modellen men det den har lärt sig behålls, till nästa gång.
settings-clip-looking = Letar efter modellen…
settings-clip-downloading = Laddar ner modellen… förloppet visas i aktivitetsindikatorn
settings-clip-ready-empty = Klar. Bilder lärs in när paket läggs till.
settings-clip-ready-all =
    { $count ->
        [one] Klar: känner till { $count } bild
       *[other] Klar: känner till alla { $count } bilder
    }
settings-clip-learning = Klar: lär sig bilder, { $done } av { $total } hittills
settings-clip-relearning = Tills den nya modellen har lärt sig varje bild använder sökningar det den gamla lärde sig.
settings-clip-unavailable = Kunde inte hämta modellen: { $detail }. Försöker igen varje timme.
settings-clip-where = Körs på
settings-clip-here = Den här servern: inget lämnar den
settings-clip-here-desktop = Den här datorn: inget lämnar den
settings-clip-hosted = En värdbaserad tjänst
settings-clip-hosted-warning = Bilder lämnar den här servern. En miniatyr av varje bild i biblioteket och orden i varje bildsökning skickas till { $service }, som tar betalt för dem och behåller dem enligt sina villkor.
settings-clip-hosted-warning-desktop = Bilder lämnar den här datorn. En miniatyr av varje bild i biblioteket och orden i varje bildsökning skickas till { $service }, som tar betalt för dem och behåller dem enligt sina villkor.
settings-clip-hosted-hint = Inget laddas ned eller körs här: tjänsten lär sig varje bild, en omgång i taget. En annan tjänst eller modell lär sig alla igen, och det varje har lärt sig sparas, så att du kan byta tillbaka.
settings-clip-saved-here = Bildsökning: körs på den här servern
settings-clip-saved-here-desktop = Bildsökning: körs på den här datorn
settings-clip-saved-hosted = Bildsökning: körs hos { $service }
settings-clip-hosted-unavailable = Kunde inte använda { $service }: { $detail }. Försöker igen varje timme.
settings-clap-heading = Sök efter hur ljud låter
settings-clap-reset = Ljudsökning: tillbaka till standardvärdet
settings-clap-saved-on = Ljudsökning: på
settings-clap-saved-off = Ljudsökning: av
settings-clap-what = Skriv vad du letar efter (”distant thunder”, ”glassy UI blip”, ”footsteps on gravel”) så dyker ljud som låter så upp bredvid namn som innehåller orden.
settings-clap-toggle = Slå på ljudsökning
settings-clap-hint = Använder LAION:s CLAP-modell, som körs på den här serverns processor: inget lämnar den. När du slår på den laddas modellen ner ({ $size } MB) från Hugging Face, och sedan lyssnar den på början av varje ljud i bakgrunden med låg prioritet. Den tar upp till ungefär 1 GB minne medan den arbetar eller någon söker. När du stänger av den raderas modellen men det den har lärt sig behålls, till nästa gång.
settings-clap-hint-desktop = Använder LAION:s CLAP-modell, som körs på den här datorns processor: inget lämnar den. När du slår på den laddas modellen ner ({ $size } MB) från Hugging Face, och sedan lyssnar den på början av varje ljud i bakgrunden med låg prioritet. Den tar upp till ungefär 1 GB minne medan den arbetar eller du söker. När du stänger av den raderas modellen men det den har lärt sig behålls, till nästa gång.
settings-clap-ready-empty = Klar. Ljud lärs in när paket läggs till.
settings-clap-ready-all =
    { $count ->
        [one] Klar: känner till { $count } ljud
       *[other] Klar: känner till alla { $count } ljud
    }
settings-clap-learning = Klar: lär sig ljud, { $done } av { $total } hittills

## SettingsPage.vue: AI-taggning

settings-ai-service = Tjänst
settings-ai-off = Av
settings-ai-off-hint = Paket taggas bara med de inbyggda reglerna.
settings-ai-local = Lokal modell
settings-ai-local-hint = Tjänsten llm i Hoards compose-fil (eller cargo xtask llm). Ladda ner en modell nedan; den körs på den här datorn och inget lämnar den.
settings-ai-chatgpt = ChatGPT-abonnemang
settings-ai-chatgpt-hint = Använder ditt ChatGPT Go-, Plus-, Pro- eller Business-abonnemang i stället för en API-nyckel.
settings-ai-openrouter-hint = En nyckel för hundratals molnmodeller, vissa av dem gratis. Modeller som tar emot bilder är markerade.
settings-ai-openai-hint = Drivs av OpenAI.
settings-ai-anthropic-hint = Drivs av Anthropic, via dess OpenAI-kompatibla slutpunkt.
settings-ai-gemini-hint = Drivs av Google, via dess OpenAI-kompatibla slutpunkt.
settings-ai-ollama-hint = En Ollama-server som du kör. Ändra adressen om det inte är en container som heter ollama.
settings-ai-custom = Annan (OpenAI-kompatibel)
settings-ai-custom-hint = Allt som talar OpenAI:s chatt-API: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Ansluter…
settings-ai-connected =
    { $count ->
        [one] Ansluten · { $count } modell
       *[other] Ansluten · { $count } modeller
    }
settings-ai-connected-none = Ansluten, men den listar inga modeller; skriv modellens namn
settings-chatgpt-connected-none = Ansluten; skriv modellens namn
settings-ai-takes-images = tar emot bilder
settings-ai-saved = AI-taggning: { $service }
settings-ai-saved-off = AI-taggning är avstängd
settings-ai-reset = AI-taggning: tillbaka till standardvärdet
settings-ai-on = På: { $service } · { $model }
settings-ai-url = Adress
settings-ai-key = API-nyckel
settings-ai-key-saved = Sparad (skriv för att ersätta)
settings-ai-key-paste = Klistra in din nyckel
settings-ai-key-optional = Ingen (valfritt)
settings-ai-key-remove = Ta bort
settings-ai-get-key = Skaffa en nyckel
settings-ai-model = Modell
settings-ai-model-pick = Välj eller skriv en modell
settings-ai-model-name = Modellnamn
settings-ai-refresh = Uppdatera
settings-ai-connect = Anslut
settings-ai-need-key = Lägg till API-nyckeln för att se dess modeller
settings-ai-vision = Skicka även miniatyrbilder (kräver en modell som tar emot bilder)
settings-ai-batch = Per begäran
settings-ai-batch-unit = resurser (färre för små lokala modeller, fler för stora molnmodeller)

## SettingsPage.vue: ChatGPT-inloggning

settings-chatgpt-account = Konto
settings-chatgpt-checking = Kontrollerar…
settings-chatgpt-status-as = Inloggad som { $email }
settings-chatgpt-status = Inloggad
settings-chatgpt-signed-in-as = Inloggad på ChatGPT som { $email }
settings-chatgpt-signed-in = Inloggad på ChatGPT
settings-chatgpt-signed-out = Utloggad från ChatGPT
settings-chatgpt-continue = Fortsätt med ChatGPT
settings-chatgpt-sign-out = Logga ut
settings-chatgpt-sign-out-title = Logga ut från ChatGPT?
settings-chatgpt-sign-out-in-use = AI-taggningen använder ditt ChatGPT-abonnemang, så den stannar tills du loggar in igen eller väljer en annan tjänst.
settings-chatgpt-sign-out-body = Hoard glömmer inloggningen och ber OpenAI att avsluta den.
settings-chatgpt-manage = Hantera användning
settings-chatgpt-finish-before = Slutför inloggningen i fönstret som öppnades (
settings-chatgpt-finish-link = öppna det igen
settings-chatgpt-finish-middle = ). Om det slutar på en sida som inte läses in, på en adress som börjar med{ " " }
settings-chatgpt-finish-after = , kopierar du adressen och klistrar in den här:
settings-chatgpt-paste = Adressen som inloggningen slutade på
settings-chatgpt-finish = Slutför
settings-chatgpt-usage = Taggningsbegäranden räknas mot ditt ChatGPT-abonnemangs användning. Hantera användning visar vad Hoard har använt och låter dig begränsa det. Om du vill koppla från Hoard i själva ChatGPT går du till Inställningar → Säkerhet och inloggning → Logga in med ChatGPT på chatgpt.com.

## SettingsPage.vue: Lokala modeller

settings-local-downloading-progress = Laddar ner { $percent } % · { $done } av { $total }
settings-local-downloading = Laddar ner…
settings-local-incomplete = Nedladdningen stoppades
settings-local-failed = Kunde inte starta förra gången
settings-local-in-use-running = Används · körs
settings-local-in-use = Används
settings-local-starting = Startar…
settings-local-running = Körs
settings-local-ready-images = Klar · tar emot bilder
settings-local-ready-text = Klar · bara text
settings-local-delete-title = Radera { $name }?
settings-local-delete-in-use = AI-taggningen använder den här modellen, så den stannar tills du väljer en annan. Modellens filer raderas från modellservern.
settings-local-delete-downloading = Nedladdningen stoppas och det som hittills har laddats ner raderas.
settings-local-delete-body = Modellens filer raderas från modellservern. Du kan ladda ner den igen senare.
settings-local-delete = Radera
settings-local-deleting = Raderar…
settings-local-cancel = Avbryt
settings-local-resume = Återuppta
settings-local-download = Ladda ner
settings-local-start-before = Starta den med{ " " }
settings-local-start-or = { " " }(eller{ " " }
settings-local-start-developing = { " " }under utveckling) och{ " " }
settings-local-start-retry = försök igen
settings-local-start-after = . En server som bara kör en modell fungerar också: ange modellens namn nedan.
settings-local-list = På modellservern
settings-local-none = Inget har laddats ner än. Välj en nedan; den sparas på servern, och du kan byta när du vill.
settings-local-catalog = Modeller att ladda ner
settings-local-size = { $size } GB · { $license }
settings-local-other = En annan modell från Hugging Face
settings-local-other-placeholder = En annan från Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Lägga till paket

settings-store-pages = Butikssidor
settings-store-pages-what = Nya paket slås upp i butikerna i bakgrunden, så att de får sin butikssida, säljare och licens.
settings-store-lookup = Slå upp butikssidor automatiskt
settings-unreal = Unreal-paketfiler
settings-unreal-reset = Unreal-paketfiler: tillbaka till standardvärdet
settings-unreal-what = Vad som ska göras med .uasset-filer.
settings-conv = Konverteringsregler
settings-conv-what = Konvertera filer som Hoard lagrar till ett format, till exempel alla OBJ och FBX till GLB, så att hela biblioteket blir enhetligt. Den konverterade filen tar originalets plats, med dess namn, taggar, samlingar och länkar. Filer i bevakade mappar ändras aldrig.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = På
settings-conv-to = Konvertera till
# Heading over the file formats a rule converts.
settings-conv-from = Från
settings-conv-remove = Ta bort
settings-conv-remove-label = Ta bort regel { $n }
settings-conv-taken = En annan regel konverterar det här formatet
settings-conv-upright = Vänd modeller upprätt när de konverteras (Y upp, vända mot +Z), så som paketets modeller visas
settings-conv-add = Lägg till en regel
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Behåll originalen i
settings-conv-keep-days =
    { $count ->
        [one] dag
       *[other] dagar
    }
settings-conv-keep-what = Till dess kan en konverterad fils sida lägga tillbaka originalet. Texturer kopieras inte in i en konverterad modell: den pekar på bildfilerna bredvid den. En modells .mtl- eller .bin-filer ligger kvar.
settings-conv-save = Spara regler
settings-conv-revert = Ångra ändringar
settings-conv-existing = Konvertera filer som redan finns i biblioteket
settings-conv-saved = Konverteringsreglerna har sparats. De gäller för paket som läggs till från och med nu.
settings-conv-converting =
    { $count ->
        [one] Konverterar en fil i bakgrunden
       *[other] Konverterar { $count } filer i bakgrunden
    }
settings-conv-nothing = Inget i biblioteket behöver konverteras.
settings-conv-ask-title =
    { $count ->
        [one] Konvertera en fil som redan finns i biblioteket?
       *[other] Konvertera { $count } filer som redan finns i biblioteket?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Den finns i ett paket.
       *[other] De finns i { $packs } paket.
    } Varje original sparas i { $days } dagar, och till dess kan det läggas tillbaka från sin sida.
settings-conv-ask-busy =
    { $count ->
        [one] En finns i ett paket som fortfarande läggs till; den konverteras när det är klart.
       *[other] { $count } finns i paket som fortfarande läggs till; de konverteras när det är klart.
    }
settings-conv-ask-list = Filer som skulle konverteras
settings-conv-ask-more = …och { $count } till
settings-conv-ask-skipped = Lämnas som de är:
settings-conv-skip-watched =
    { $count ->
        [one] en fil i en bevakad mapp
       *[other] { $count } filer i bevakade mappar
    }
settings-conv-skip-unity =
    { $count ->
        [one] en fil som ett Unity-projekt hänvisar till med namn
       *[other] { $count } filer som ett Unity-projekt hänvisar till med namn
    }
settings-conv-skip-used =
    { $count ->
        [one] en fil som en scen eller en annan fil nämner
       *[other] { $count } filer som en scen eller en annan fil nämner
    }
settings-conv-skip-beside =
    { $count ->
        [one] en fil vars konverterade kopia redan finns i dess paket
       *[other] { $count } filer vars konverterade kopior redan finns i deras paket
    }
settings-conv-skip-blender =
    { $count ->
        [one] en fil som Blender inte har läst än (Inställningar → Verktyg)
       *[other] { $count } filer som Blender inte har läst än (Inställningar → Verktyg)
    }
settings-conv-ask-new = Spara, bara nya paket
settings-conv-ask-save-convert =
    { $count ->
        [one] Spara och konvertera den
       *[other] Spara och konvertera { $count } filer
    }
settings-conv-ask-convert =
    { $count ->
        [one] Konvertera den
       *[other] Konvertera { $count } filer
    }
settings-unreal-off = Lämna som de är
settings-unreal-off-hint = Konvertera från ett pakets sida när du vill.
settings-unreal-keep = Konvertera, behåll originalen
settings-unreal-keep-hint = Texturer, ljud och material kommer ut i vanliga format; .uasset-filen finns också kvar.
settings-unreal-replace = Konvertera, kasta originalen
settings-unreal-replace-hint = Sparar utrymme när innehållet har plockats ut. Mesher väntar fortfarande på Unreal-arbetaren.

## SettingsPage.vue: Nedladdningar

settings-tn = Texturnamn
settings-tn-saved = Texturnamnen har sparats
settings-tn-reset = Texturnamn: tillbaka till standardvärdet
settings-tn-what = Ett materials kartor skickas ut med namn efter materialet och vad varje karta är till för, oavsett vad filerna heter: Rock_Normal.png, inte rock_moss_xeokah0_raw_8K_Normal.png. Filerna i biblioteket rörs inte, och en karta som namnges av en modell eller MTL som skickas med behåller det namnet, så att modellen fortfarande hittar den.
settings-tn-toggle = Namnge texturkartor efter deras material
settings-tn-style = Stil
settings-tn-style-group = Namngivningsstil
settings-tn-descriptive = Beskrivande
settings-tn-short = Kort
settings-tn-custom = Egen
settings-tn-pattern = Mönster
settings-tn-hint-name = { " " }materialet,{ " " }
settings-tn-hint-map = { " " }vad kartan är till för,{ " " }
settings-tn-hint-size = { " " }dess storlek när uppsättningen finns i mer än en (läggs till i slutet om mönstret utelämnar den).
settings-tn-example = Exempel
settings-tn-suffixes = Suffix för varje typ av karta
settings-tn-slots = I GLB-nedladdningar namnges en textur som är packad inuti en modell utan egen fil efter materialet och det fack den fyller, och de facken har också suffix:
settings-tn-endings = Egna ändelser
settings-tn-endings-what = En textur som inte är en av ett materials kartor behåller sitt eget namn, om den inte slutar på någon av dessa: _diff → _BC skickar rock_diff.png som rock_BC.png. Lämna den andra tom för att ta bort ändelsen.
settings-tn-ending-from = Ändelse
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Byt mot
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Lägg till en ändelse
settings-tn-ending-remove = Ta bort
settings-tn-ending-remove-label = Ta bort { $ending }

## SettingsPage.vue: Allmänt och Anslutning

settings-name = Namn
settings-name-what = Vad den här servern heter i det övre fältet, på webbläsarfliken och på inloggningssidan, så att du kan skilja dem åt om du har mer än en Hoard.
settings-name-what-desktop = Vad det här biblioteket heter i det övre fältet.
settings-address = Adress
settings-address-what = Adressen som personer och appar använder för att nå Hoard, när den inte är den som den här webbläsaren använder: en omvänd proxys, en tunnels eller ett tailnet-namn, till exempel https://hoard.example.com. Länkar som Hoard delar ut (till Claude och andra appar, vid inloggning, i sitt certifikat) använder den.
settings-public-url = Offentlig adress
settings-public-url-placeholder = Från webbläsarens adress (t.ex. https://hoard.example.com)
settings-public-url-default = från webbläsarens adress

## ProfilerPanel.vue: profileraren överst i Inställningar → Prestanda

profiler = Profilerare
profiler-what = Vad Hoard har gjort de senaste tio minuterna, uppdaterat varannan sekund medan du tittar. Det sparas bara i minnet och börjar om när Hoard startar om.
profiler-starting = De första mätvärdena kommer om en stund.
profiler-failed = Kunde inte läsa profileraren: { $error }
profiler-cpu = Processoranvändning
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Hela maskinen
profiler-cpu-summary = Hoard använder { $hoard } av processorerna, och hela maskinen { $machine }.
profiler-memory = Minne
profiler-serving = Pågående begäranden
profiler-working = Arbetar med
profiler-working-none = Inget kördes i bakgrunden.
profiler-working-more =
    { $count ->
        [one] Och en till.
       *[other] Och { $count } till.
    }
profiler-requests = Tid för att hantera begäranden
profiler-requests-what = Varje stapel är tio sekunders begäranden, med tiden uppdelad efter vart den gick.
profiler-part-db = Databas
profiler-part-db-title = Väntar på databasen och kör dess frågor
profiler-part-disk = Disk
profiler-part-disk-title = Läser lagrade filer och komprimerar det som skickas
profiler-part-send = Sändning
profiler-part-send-title = Väntar medan nätverket tar emot det som skickats
profiler-part-other = Övrigt
profiler-part-other-title = Räknar ut svaret
profiler-request-count =
    { $count ->
        [one] { $count } begäran
       *[other] { $count } begäranden
    }
profiler-requests-none = Inga begäranden de senaste tio minuterna.
profiler-slowest = Långsammast: { $route }, { $time }
profiler-routes = Per rutt
profiler-route = Rutt
profiler-count = Begäranden
profiler-average = Medel
profiler-p95 = Långsammaste 5 %
profiler-split = Vart tiden gick
profiler-sent = Skickat
profiler-portal = gränssnittets egna filer

## SettingsPage.vue: Prestanda

settings-render = 3D-miniatyrbilder
settings-render-toggle = Rendera miniatyrbilder för 3D-modeller
settings-gpu-on = På grafikprocessorn: { $renderer }
settings-gpu-software = Ingen grafikprocessor hittades, så i mjukvara ({ $renderer })
settings-gpu-none = Ingen grafikprocessor eller mjukvarurenderare hittades, så modeller får inga miniatyrbilder
settings-thumbs = Miniatyrbilder
settings-thumbs-what = Hur nya miniatyrbilder och modellvyer lagras. De med genomskinlighet, och pixelgrafik, är alltid PNG. Om du vill krympa de du redan har kör du Krymp miniatyrbilder under Underhåll.
settings-previews-size = Förhandsvisningar tar { $size } nu.
settings-thumbs-saved = Miniatyrbilder: { $quality }
settings-thumbs-small = Mindre
settings-thumbs-small-hint = JPEG, kvalitet 70. Bra i rutnätet; lite mjuk på nära håll.
settings-thumbs-balanced = Balanserad
settings-thumbs-balanced-hint = JPEG, kvalitet 82. Svår att skilja från originalet i miniatyrstorlek.
settings-thumbs-sharp = Skarpare
settings-thumbs-sharp-hint = JPEG, kvalitet 92. Ungefär dubbelt så stor som Balanserad.
settings-thumbs-lossless = Förlustfri
settings-thumbs-lossless-hint = PNG, exakt som den skapades. Flera gånger större.
settings-motion-thumbs = Rörliga miniatyrer för animationer
settings-motion-thumbs-what = Varje animation får också en kort film i loop som spelas medan du pekar på dess ruta. De tar ungefär 40 kB var och en stund att göra för ett stort bibliotek. Stänger du av det tas de bort.
settings-limits = Arbete och gränser
settings-limits-what = Varje värde sparas när du lämnar fältet eller trycker på Enter. Storlekar anges med enhet: 100 GB, 512 MB, 1,5 TB; ett tal utan enhet är GB.
settings-workers = Bakgrundsarbetare
settings-workers-unit = jobb samtidigt (intag, taggning, konverteringar)
settings-max-upload = Största uppladdning
settings-max-upload-unit = per fil
settings-max-expand = Största paket
settings-max-expand-unit = när det har packats upp
settings-conversion-cache = Konverterade nedladdningar
settings-conversion-cache-unit-now = sparas som mest, { $size } nu; de som använts minst nyligen tas bort först
settings-conversion-cache-unit = sparas som mest; de som använts minst nyligen tas bort först

## SettingsPage.vue: Underhåll

settings-updates = Uppdateringar
settings-version = Det här är Hoard { $version }.
settings-update-check = Kolla GitHub med några timmars mellanrum efter en ny version
settings-desktop-updates = Hoard letar efter en ny version varje gång det startar, laddar ned den i bakgrunden och installerar den när du avslutar.

## Resurstyper (api.ts)

api-kind-texture = Texturer
api-kind-vector = Vektorgrafik
api-kind-sheet = Spriteark
api-kind-audio = Ljud
api-kind-model = Modeller
api-kind-animation = Animationer
api-kind-font = Typsnitt
api-kind-shader = Shaders
api-kind-material = Material
api-kind-video = Video
api-kind-document = Dokument
api-kind-data = Data
api-kind-other = Övrigt
api-kind-texture-name = texturer
api-kind-vector-name = vektorgrafik
api-kind-sheet-name = spriteark
api-kind-audio-name = ljud
api-kind-model-name = modeller
api-kind-animation-name = animationer
api-kind-font-name = typsnitt
api-kind-shader-name = shaders
api-kind-material-name = material
api-kind-video-name = video
api-kind-document-name = dokument
api-kind-data-name = data
api-kind-other-name = övrigt

## Aviseringar, dialogrutor och inläsning (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Ångra
ui-today = i dag
ui-yesterday = i går
ui-days-ago =
    { $days ->
        [one] för { $days } dag sedan
       *[other] för { $days } dagar sedan
    }
ui-notifications = Aviseringar
ui-dismiss = Avfärda
ui-cancel = Avbryt
ui-lazy-loading = Läser in { $what }…
ui-lazy-failed = Kunde inte läsa in { $what }.
ui-lazy-reload = Läs in sidan igen

## ResetButton.vue

reset-title = Återställ till standardvärdet: { $value }
reset-label = Återställ till standardvärdet, { $value }

## SizeInput.vue

size-invalid = En storlek som 100 GB, 512 MB eller 1,5 TB
size-hint = En storlek som 100 GB, 512 MB eller 1,5 TB; ett tal utan enhet är GB

## AuthPanel.vue (Inställningar → Autentisering)

auth-load-failed = Kunde inte läsa in autentiseringsinställningarna: { $error }
auth-saved = Sparat. Det gäller från nästa inloggning.
auth-method-internal = Intern
auth-method-internal-hint = Namn och lösenord som Hoard sparar, tillagda i Inställningar → Konto.
auth-method-ldap-hint = En katalog kontrollerar namnet och lösenordet som skrivs på inloggningssidan: OpenLDAP, FreeIPA, LLDAP eller en annan LDAP-server.
auth-method-ad-hint = Windows-domänkonton loggar in med sitt vanliga namn och lösenord.
auth-method-oauth-hint = En knapp ”Logga in med…” som går till Google, Microsoft, GitHub, Keycloak, Authentik eller en annan leverantör och tillbaka.
auth-session-unused = Efter 30 dagar utan användning
auth-session-1h = 1 timme efter inloggning
auth-session-8h = 8 timmar efter inloggning
auth-session-day = En dag efter inloggning
auth-session-week = En vecka efter inloggning
auth-session-30d = 30 dagar efter inloggning
auth-session-hours =
    { $hours ->
        [one] { $hours } timme efter inloggning
       *[other] { $hours } timmar efter inloggning
    }
auth-role-view = Kan titta
auth-role-edit = Kan ändra
auth-role-admin = Admin
auth-role-word-view = kan titta
auth-role-word-edit = kan ändra
auth-role-word-admin = admin
auth-role-word-default = { $role } (standardrollen)
auth-tls-starttls-hint = Vanlig LDAP (port 389) som växlar till TLS innan något skickas.
auth-tls-ldaps-hint = TLS från början (port 636).
auth-tls-none = Ingen
auth-tls-none-hint = Lösenord skickas över nätverket som de är. Bara på ett nätverk du litar på.
auth-test-failed = Kunde inte testa: { $error }
auth-preset-custom = Annan (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel och andra: ange utfärdaren så fyller Upptäck i adresserna.
auth-preset-google-note = Skapa en OAuth-klient (webbapplikation) i Google Cloud Console → APIs & Services → Credentials. Google skickar inga grupper, så ge roller i Inställningar → Konto, och stäng av att konton skapas vid första inloggningen om inte alla med ett Google-konto ska komma in.
auth-preset-microsoft-note = Registrera en app i Entra ID → App registrations (en Web-omdirigering till adressen nedan). Ersätt ”organizations” med ditt klient-ID (tenant ID) för att bara släppa in din organisation. För roller från grupper lägger du till groups-claimet under Token configuration: det listar grupp-ID:n.
auth-preset-github-note = Skapa en OAuth-app i GitHub → Settings → Developer settings. GitHub skickar inga grupper, så ge roller i Inställningar → Konto, och stäng av att konton skapas vid första inloggningen om inte alla med ett GitHub-konto ska komma in.
auth-preset-gitlab-note = Skapa en applikation i GitLab → Preferences → Applications (eller i din instans adminområde: ändra gitlab.com i adresserna).
auth-discovered = Fyllde i leverantörens adresser.
auth-copied = Kopierat
auth-copy-failed = Kunde inte kopiera: markera och kopiera det själv
auth-sign-in-with = Logga in med
auth-own-accounts-still-work = Hoards egna konton loggar fortfarande in med sina egna lösenord, så en katalog eller leverantör som ligger nere eller är felkonfigurerad kan inte låsa ute dig.
auth-sessions = Sessioner
auth-sign-out-after = Logga ut personer
auth-ldap-server = LDAP-server
auth-server-type = Servertyp
auth-server-type-custom = Anpassad
auth-servers = Servrar
auth-server-n = Server { $n }
auth-remove-server-n = Ta bort server { $n }
auth-remove = Ta bort
auth-add-server = Lägg till en server
auth-servers-hint = Nästa server provas när en inte kan nås.
auth-security = Säkerhet
auth-connection-security = Anslutningssäkerhet
auth-skip-verify = Kontrollera inte serverns certifikat (bara för att testa)
auth-ca = Certifikatutfärdare
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (tomt: de som systemet litar på)
auth-lookup = Slå upp personer
auth-anonymously = Anonymt
auth-reader-account = Läskonto
auth-reader-password = Dess lösenord
auth-saved-type-to-replace = Sparat: skriv för att ersätta
auth-people-under = Personer finns under
auth-name-attribute = Namnattribut
auth-only-matching = Bara poster som matchar
auth-groups-under = Grupper finns under
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (tomt: bara memberOf)
auth-their-groups = Deras grupper
auth-group-filter-hint = { "{" }dn{ "}" } är personens post, { "{" }user{ "}" } namnet de loggade in med.
auth-test = Testa
auth-test-hint = Når varje server och loggar in som läskontot. Med ett namn och lösenord loggar den också in som den personen och visar vad Hoard skulle göra av det. Inget sparas.
auth-test-user = Någon att logga in som
auth-test-user-placeholder = Någon att logga in som (valfritt)
auth-test-password = Deras lösenord
auth-testing = Testar…
auth-test-connectivity = Testa anslutningen
auth-test-server-ok = nåddes, och läskontot loggade in
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }loggade in (
auth-test-signed-in-after = ).
auth-test-role = De skulle få: { $role }.
auth-test-no-role = De finns inte i någon av grupperna under Roller, så de skulle nekas.
auth-test-groups = Grupper: { $groups }
auth-test-no-groups = Inga grupper hittades.
auth-provider = Leverantör
auth-redirect = Omdirigeringsadress
auth-copy = Kopiera
auth-redirect-hint = Ge leverantören den här när du skapar Hoards klient. Den kommer från den offentliga adressen i Inställningar → Anslutning, eller från adressen du använder.
auth-issuer = Utfärdare
auth-discover = Upptäck
auth-client-id = Klient-ID
auth-client-secret = Klienthemlighet
auth-authorization-url = Auktoriseringsadress
auth-token-url = Tokenadress
auth-userinfo-url = Adress för användarinformation
auth-userinfo-placeholder = (tomt: ID-token säger vem det är)
auth-scopes = Omfång (scopes)
auth-name-from = Namn från
auth-groups-from = Grupper från
auth-groups-from-placeholder = (inga)
auth-button = Knapp
auth-button-placeholder = ditt konto
auth-accounts = Konton
auth-provision = Skapa ett Hoard-konto första gången någon loggar in
auth-provision-on-hint = Annars lägger en administratör först till varje person, i Inställningar → Konto.
auth-provision-off-hint = En administratör lägger först till varje person, i Inställningar → Konto, och väljer hur de loggar in.
auth-open-to-anyone = Alla med ett { $provider }-konto kan komma in. Lägg till visningsgrupper nedan, eller stäng av det här och lägg till personer själv.
auth-new-accounts = Nya konton
auth-roles-from-groups = Roller från grupper
auth-roles-hint = Grupper vars medlemmar får varje roll, en per rad, efter namn eller DN. Den högsta som matchar vinner, och det kontrolleras vid varje inloggning. Om inga listas får nya konton rollen ovan och en administratör ändrar den efteråt. Om grupper listas under Kan titta får bara medlemmar i en listad grupp logga in.
auth-roles-hint-oauth = Grupper vars medlemmar får varje roll, en per rad, efter namn eller DN (eller det som groups-claimet listar). Den högsta som matchar vinner, och det kontrolleras vid varje inloggning. Om inga listas får nya konton rollen ovan och en administratör ändrar den efteråt. Om grupper listas under Kan titta får bara medlemmar i en listad grupp logga in.
auth-saving = Sparar…
auth-save = Spara

## HttpsPanel.vue (Inställningar → Anslutning)

https-load-failed = Kunde inte läsa in HTTPS: { $error }
https-off-redirecting = HTTPS är avstängt. Skickar dig till http://
https-now-on = HTTPS är påslaget
https-saved = Sparat. Nya anslutningar använder det nya certifikatet.
https-now-off = HTTPS är avstängt
https-http3-failed-toast = HTTP/3 kunde inte starta: { $error }
https-copied = Kopierat
https-copy-failed = Kunde inte kopiera: markera och kopiera det själv
https-mode-off = Av
https-mode-off-hint = Bara vanlig http://.
https-mode-own = Hoards eget certifikat
https-mode-own-hint = Hoard skapar ett för den här serverns adresser. Installera Hoards certifikatutfärdare en gång på varje enhet så litar dess webbläsare på det.
https-mode-custom = Mitt certifikat
https-mode-custom-hint = Ett som du redan har, t.ex. från tailscale cert eller Let’s Encrypt. Webbläsare litar på det utan att något behöver installeras.
https-untrusted-own = Den här webbläsaren litar inte på det för { $host } än: installera Hoards certifikatutfärdare nedan. Du får se en varning som du kan klicka dig förbi.
https-untrusted-uncovered = Den här webbläsaren litar inte på det för { $host } än: certifikatet täcker inte den här adressen. Du får se en varning som du kan klicka dig förbi.
https-untrusted = Den här webbläsaren litar inte på det för { $host } än. Du får se en varning som du kan klicka dig förbi.
https-now = Nu
https-start-failed = HTTPS är konfigurerat men kunde inte starta: { $error }
https-on-port = HTTPS är påslaget, på port { $port } precis som http://.
https-using-it = Du använder det nu.
https-open = Öppna { $url }
https-trusted = Den här webbläsaren litar på det.
https-cert-for-before = Certifikat för{ " " }
https-cert-for-after = , till och med { $date }.
https-cert-for-after-own = , till och med { $date } (förnyas av sig självt).
https-not-covered-before = Det täcker inte{ " " }
https-not-covered-after = , adressen du använder.
https-is-off = HTTPS är avstängt: sidor skickas över vanlig http://.
https-addresses = Adresser
https-addresses-hint = Certifikatet täcker adresserna som Hoard hittade, och alla du lägger till (ett namn som din router ger det, ett Tailscale-namn).
https-remove-name = Ta bort { $name }
https-another-address = En annan adress
https-another-address-placeholder = En annan adress, t.ex. nas.local
https-add = Lägg till
https-your-cert = Ditt certifikat
https-saved-cert-before = Sparat:{ " " }
https-saved-cert-after = { " " }från { $issuer }, till och med { $date }. Klistra in ett nytt för att ersätta det.
https-pem-before = Certifikatet (med sin kedja) och dess privata nyckel, som PEM-text: filerna som börjar med{ " " }
https-pem-after = .
https-certificate = Certifikat
https-choose-file = Välj fil…
https-private-key = Privat nyckel
https-checked-before = För{ " " }
https-checked-after = , från { $issuer }, till och med { $date }.
https-checked-after-days =
    , från { $issuer }, till och med { $date } ({ $days ->
        [one] { $days } dag
       *[other] { $days } dagar
    }).
https-self-signed = Det är självsignerat: webbläsare kommer att varna för det.
https-key-matches = Nyckeln matchar.
https-where-to-get = Var du skaffar ett
https-get-tailscale-before = slå på HTTPS för ditt tailnet och kör sedan{ " " }
https-get-tailscale-after = { " " }på servern och ladda upp de två filerna. De gäller i 90 dagar: kör kommandot igen och ladda upp dem igen innan dess.
https-get-acme-title = Let’s Encrypt eller en annan ACME-klient
https-get-acme-before = (certbot, acme.sh, lego) för en domän du äger: ladda upp{ " " }
https-get-acme-and = { " " }och{ " " }
https-get-acme-after = .
https-get-proxy-title = En omvänd proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) som har certifikatet: låt HTTPS vara avstängt här och låt proxyn skicka vanlig http:// till Hoard.
https-trying = Provar…
https-apply = Tillämpa
https-apply-hint = Hoard provar den nya konfigurationen först, och behåller det som finns om den inte fungerar.
https-trust-title = Lita på Hoard på dina enheter
https-download-ca = Ladda ner hoard-ca.crt
https-trust-hint = Installera Hoards certifikatutfärdare en gång på varje enhet, så litar dess webbläsare på den här servern (och på alla certifikat som Hoard skapar för den senare). Den kan bara gå i god för den här Hoard-servern. Fingeravtryck (SHA-256):
https-ca-windows = Öppna filen → Installera certifikat → Aktuell användare → Placera alla certifikat i följande arkiv → Bläddra → Betrodda rotcertifikatutfärdare → Slutför. Starta om webbläsaren.
https-ca-macos = Öppna filen; den hamnar i Nyckelhanterare. Dubbelklicka på ”{ $subject }” där → Lita på → När detta certifikat används: Lita alltid på.
https-ca-ios-title = iPhone och iPad
https-ca-ios = Ladda ner den i Safari och tillåt det. Inställningar → Profil hämtad → Installera. Sedan Inställningar → Allmänt → Om → Certifikatförtroendeinställningar → slå på ”{ $subject }”.
https-ca-android = Inställningar → Säkerhet → Fler säkerhetsinställningar → Kryptering och inloggningsuppgifter → Installera ett certifikat → CA-certifikat, och välj filen. (Namnen varierar lite mellan telefoner; sök efter ”certifikat” i Inställningar.)
https-ca-linux-1 = Debian och Ubuntu:{ " " }
https-ca-linux-2 = . Fedora och Arch:{ " " }
https-ca-linux-3 = . Chrome på Linux har en egen lista: Inställningar → Sekretess och säkerhet → Säkerhet → Hantera certifikat → Importera.
https-ca-firefox = Firefox har en egen lista: Inställningar → Integritet och säkerhet → Certifikat → Visa certifikat → Certifikatutfärdare → Importera, och kryssa i ”Lita på denna CA för att identifiera webbplatser”.
https-ca-cli-title = Kommandoradsverktyget hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , eller ange sökvägen i{ " " }
https-ca-cli-after = .
https-options = Alternativ
https-redirect = Skicka webbläsare på andra enheter från http:// till https://
https-redirect-hint = Bara sidinläsningar skickas vidare. Kommandoradsverktyget hoard, Unreal-arbetaren, skript och AI-appar fortsätter att fungera över http://, och det gör även http://localhost på själva servern.
https-http3-hint-before = Snabbare på ostabilt wifi och i telefoner. Kräver HTTPS, och att UDP-port { $port } är öppen förutom TCP (i Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Körs.
https-http3-waits = Det startar när HTTPS är påslaget.
https-http3-failed = HTTP/3 kunde inte starta: { $error }
https-lockout-title = Om du blir utelåst
https-lockout-1 = { "" }
https-lockout-2 = { " " }på själva servern fungerar alltid över vanlig http://. Eller stäng av HTTPS från serverns kommandorad och starta Hoard igen:{ " " }
https-lockout-3 = { " " }(i Docker:{ " " }
https-lockout-4 = , och sedan{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Inställningar → Konto)

account-role-view = Kan titta
account-role-view-hint = Söka, förhandsvisa och ladda ner
account-role-edit = Kan ändra
account-role-edit-hint = Även lägga till, ändra, tagga, samla och arkivera
account-role-admin = Admin
account-role-admin-hint = Även Inställningar, konton och underhåll, och att radera för gott
account-source-internal = Ett Hoard-lösenord
account-source-ldap = Sitt { $directory }-lösenord
account-source-oauth = Loggar in med { $provider }
account-source-word-directory = katalog
account-load-failed = Kunde inte läsa in kontona: { $error }
account-failed = { $what }: { $error }
account-password-failed = Kunde inte byta ditt lösenord
account-password-changed = Lösenordet har bytts. Dina andra enheter har loggats ut.
account-add-failed = Kunde inte lägga till { $name }
account-added = { $name } kan logga in nu.
account-role-failed = Kunde inte ändra vad { $name } kan göra
account-remove-title = Ta bort { $name }?
account-remove-body = De loggas ut överallt och kan inte logga in igen. Inget som de har lagt till i biblioteket tas bort.
account-remove = Ta bort
account-remove-failed = Kunde inte ta bort { $name }
account-token-failed = Kunde inte skapa token
account-copied = Kopierat
account-copy-token-failed = Kunde inte kopiera: markera token och kopiera den själv
account-copy-failed = Kunde inte kopiera: markera och kopiera det själv
account-revoke-title = Återkalla ”{ $name }”?
account-revoke-body = Allt som fortfarande använder den nekas från och med nu.
account-revoke = Återkalla
account-revoke-failed = Kunde inte återkalla ”{ $name }”
account-disconnect-title = Koppla från { $app }?
account-disconnect-body = Den nekas från och med nu och måste anslutas igen för att använda Hoard.
account-disconnect = Koppla från
account-disconnect-failed = Kunde inte koppla från { $app }
account-guest-failed = Kunde inte ändra det
account-never = aldrig
account-you = Du
account-sign-out = Logga ut
account-signed-in-as-before = Inloggad som{ " " }
account-signed-in-as-admin = , som är administratör.
account-signed-in-as-edit = , som kan ändra biblioteket.
account-signed-in-as-view = , som kan titta men inte ändra något.
account-app-signin = Hoard-appen loggar in dig själv, så det finns inget lösenord. Skapa en åtkomsttoken nedan för motortilläggen och kommandot hoard.
account-password-at-provider = Ditt lösenord hanteras av leverantören du loggar in med, så du byter det där.
account-password-at-directory = Ditt lösenord hanteras av katalogen du loggar in med, så du byter det där.
account-current-password = Nuvarande lösenord
account-new-password = Nytt lösenord
account-new-password-again = Nytt lösenord igen
account-change-password = Byt lösenord
account-password-mismatch = De nya lösenorden stämmer inte överens.
account-password-rule = Minst 8 tecken.
account-people = Personer
account-people-hint = Varje person kan titta (söka, förhandsvisa och ladda ner), även ändra biblioteket (lägga till, redigera, tagga, samla och arkivera) eller dessutom vara administratör (Inställningar, konton och underhåll, och att radera för gott).
account-you-tag = (du)
account-signs-in-oauth = Loggar in med OAuth-leverantören
account-signs-in-directory = Loggar in med katalogen
account-since = sedan { $date }
account-what-can-do = Vad { $name } kan göra
account-name = Namn
account-how-sign-in = Hur de loggar in
account-password-label = Lösenord (minst 8 tecken)
account-password-placeholder = Lösenord (8+ tecken)
account-what-they-can-do = Vad de kan göra
account-add = Lägg till
account-ai-apps = Claude och andra AI-appar
account-ai-apps-hint = Anslut Claude (eller valfri MCP-app) så kan den söka efter och hämta resurser åt dig, och beskriva och tagga det som Hoard inte har hunnit med än. Den loggar in i Hoard första gången; du godkänner det här i webbläsaren.
account-mcp-address = Hoards MCP-adress
account-copy = Kopiera
account-claude-code-1 = kör{ " " }
account-claude-code-2 = , sedan{ " " }
account-claude-code-3 = { " " }för att logga in.{ " " }
account-claude-code-4 = { " " }beskriver ett paket.
account-claude-ai = Settings → Connectors → Add custom connector, klistra in adressen och välj sedan Connect. De ansluter från Anthropics servrar, så Hoard måste kunna nås från internet över HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, klistra in adressen och välj sedan Connect. De ansluter från Anthropics servrar, så Hoard måste kunna nås från internet över HTTPS (den här adressen kan inte det: ange Offentlig adress i Inställningar → Anslutning).
account-connection-meta = för { $user } sedan { $since } · senast använd { $used }
account-no-apps = Inga appar anslutna än.
account-tokens = Åtkomsttoken
account-tokens-hint-1 = För kommandoradsverktyget{ " " }
account-tokens-hint-2 = , skript, MCP-klienter och Unreal-arbetaren: de skickar den som{ " " }
account-tokens-hint-3 = . En token kan göra det du väljer, upp till det du själv kan, men aldrig hantera konton eller token. En som bara kan titta räcker för att söka och ladda ner, och för att synka ett projekt.
account-tokens-admin = Du ser allas token och anslutna appar; andra ser sina egna.
account-made-before = { "" }
account-made-after = : kopiera den nu. Den visas inte igen.
account-new-token = Den nya token
account-cli-1 = För kommandoraden:{ " " }
account-cli-2 = , eller lägg den i{ " " }
account-cli-3 = .
account-done = Klar
account-token-meta = skapad { $made } · senast använd { $used }
account-token-meta-by = skapad { $made } av { $by } · senast använd { $used }
account-no-tokens = Inga token än.
account-token-name = Vad som ska använda den
account-token-name-placeholder = Vad som ska använda den, t.ex. ”Unreal-arbetaren”
account-token-role = Vad den kan göra
account-make-token = Skapa en token
account-guests = Gäster
account-guests-allow = Låt alla som kan nå servern bläddra utan att logga in
account-guests-hint = De kan söka, förhandsvisa och ladda ner, men inte lägga till, ändra eller radera något, eller se Inställningar.

## SignIn.vue

signin-create-title = Skapa ditt konto
signin-title = Logga in
signin-setup-lede = Den här servern har inget konto än. Det du skapar här kan göra allt; lägg till konton för andra personer i Inställningar → Konto senare.
signin-overlay-lede = Alla kan bläddra här, men för att ändra något behövs ett konto.
signin-with = Logga in med { $provider }
signin-or-password = eller med namn och lösenord
signin-username = Användarnamn
signin-name = Namn
signin-new-password = Nytt lösenord
signin-password = Lösenord
signin-password-rule = Minst 8 tecken.
signin-confirm-password = Bekräfta lösenordet
signin-mismatch = De två lösenorden stämmer inte överens.
signin-setup-code = Konfigurationskod
signin-setup-code-hint-before = Koden som den här servern fick i{ " " }
signin-setup-code-hint-after = .
signin-creating = Skapar…
signin-signing-in = Loggar in…
signin-create = Skapa konto
signin-submit = Logga in
signin-keep-browsing = Fortsätt bläddra
signin-directory-hint = Använd ditt namn och lösenord för { $directory }.
signin-forgot-1 = Glömt ett Hoard-lösenord? Kör{ " " }
signin-forgot-2 = { " " }på servern (i Docker{ " " }
signin-forgot-3 = ) för att få ett nytt.

## SetupWizard.vue (första besöket på en ny server)

setup-title = Var ska ditt bibliotek finnas?
setup-lede = Välj en mapp på servern där Hoard ska förvara dina paket, eller peka på en mapp med paket som du redan har. Du kan lägga till fler platser, på andra diskar, i Inställningar senare.
setup-lede-desktop = Välj en mapp där Hoard ska förvara dina paket, eller peka på en mapp med paket som du redan har. Du kan lägga till fler platser, på andra diskar, i Inställningar senare.
setup-unreachable = Kunde inte nå servern: { $error }
setup-unreachable-desktop = Hoard svarar inte: { $error }
setup-try-again = Försök igen
setup-loading = Läser in…
setup-start = Starta biblioteket
setup-ready = Ditt bibliotek är klart
setup-managed-before = Nya paket hamnar i{ " " }
setup-managed-after = . Släpp filer, mappar eller arkiv var som helst på sidan för att lägga till de första.
setup-watched-before = Hoard bevakar{ " " }
setup-watched-after = ; paketen dyker upp allteftersom de hittas. Om du även vill ladda upp lägger du till ett bibliotek som Hoard förvaltar i Inställningar → Biblioteksplatser.
setup-open = Öppna biblioteket

## ConnectApp.vue (en app som Claude ber om att logga in)

connect-title = Ansluta { $app }?
connect-lede-1 = { " " }kommer att kunna söka, ladda ner, beskriva och tagga allt i det här Hoard, och agerar då som{ " " }
connect-lede-2 = . Den kan inte hantera konton eller åtkomsttoken.
connect-back-before = Du skickas tillbaka till{ " " }
connect-back-after = . Koppla från den när som helst i Inställningar → Konto.
connect-allow = Tillåt
connect-cancel = Avbryt
connect-failed = Det går inte att ansluta den här appen
connect-go-library = Gå till biblioteket
connect-checking = Kontrollerar…

## InsecureNotice.vue (raden som visas över vanlig http://)

insecure-text = Det här är vanlig http://, så några saker kräver HTTPS: 3D med WebGPU, det moderna urklippet, att installera Hoard som app och Claude-kopplingar.
insecure-switch = Byt till https://
insecure-set-up = Konfigurera HTTPS
insecure-dismiss = Visa inte det här igen

## FormatWizard.vue (Inställningar → Butiksfiler → lära Hoard en butiks fil)

formatwizard-too-big = Den här filen är för stor för att läsas som en butiks fil.
formatwizard-not-json = Den här filen är inte giltig JSON, så den visas som text.
formatwizard-read-failed = Kunde inte läsa filen: { $error }
formatwizard-back-title = Gå tillbaka till formuläret?
formatwizard-back-body = Ändringar som gjorts i TOML-koden går förlorade: formuläret skriver formatet igen utifrån det det har.
formatwizard-back-to-form = Tillbaka till formuläret
formatwizard-miss-name = Dess filer heter inte { $file }. Lägg till det under Filer.
formatwizard-miss-not-the-stores = Det som butikens filer känns igen på finns inte i den här filen. Ändra Känn igen på.
formatwizard-miss-no-title = Det hittar ingen titel i den här filen. Välj titeln.
formatwizard-miss-short-name = inte dess namn
formatwizard-miss-short-not-the-stores = inte butikens
formatwizard-miss-short-no-title = ingen titel
formatwizard-saved-packs =
    Sparade { $file }. { $packs ->
        [one] { $packs } paket läses
       *[other] { $packs } paket läses
    } in igen.
formatwizard-saved = Sparade { $file }.
formatwizard-save-failed = Kunde inte spara formatet: { $error }
formatwizard-zip-failed = Kunde inte skapa zip-filen: { $error }
formatwizard-stop-title = Sluta lära in den här filen?
formatwizard-stop-body = Formatet sparas inte.
formatwizard-stop = Sluta
formatwizard-label = Formatguide
formatwizard-changing = Ändrar { $name }
formatwizard-starting-from = Utgår från { $name }
formatwizard-title = Lär Hoard en butiks fil
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }i{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Redigera som TOML
formatwizard-file-reference = Filen, som referens.
formatwizard-click-json = Klicka på { $field } i filen.
formatwizard-click-text = Klicka på { $field } i filen, eller markera det.
formatwizard-pick-first = Välj ett fält till höger och klicka sedan på dess värde här.
formatwizard-loading = Läser in…
formatwizard-the-format = Formatet
formatwizard-syntax-before = Syntaxen beskrivs i{ " " }
formatwizard-syntax-after = .
formatwizard-store = Butik
formatwizard-store-placeholder = Butikens namn, t.ex. Kenney
formatwizard-files = Filer
formatwizard-recognize = Känn igen på
formatwizard-recognize-json = En nyckel som bara butikens filer har
formatwizard-recognize-text = En rad som bara butikens filer har
formatwizard-recognize-hint = Ett sätt per rad; vilket som helst av dem räcker. Behövs när andra filer har samma namn.
formatwizard-seller = Säljare
formatwizard-seller-placeholder = När filen inte säger det
formatwizard-tags = Taggar
formatwizard-tags-placeholder = För allt den beskriver, kommaseparerade
formatwizard-fields = Fält
formatwizard-selector = { $field }-väljare
formatwizard-clear = Rensa { $field }
formatwizard-pick = Välj
formatwizard-what-it-reads = Vad det läser
formatwizard-read-title = Titel
formatwizard-read-seller = Säljare
formatwizard-read-license = Licens
formatwizard-read-url = Butikssida
formatwizard-read-kind = Typ
formatwizard-read-tags = Taggar
formatwizard-read-origin = Tillverkad i
formatwizard-read-size = Storlek
formatwizard-also = { $format } läser också den här filen. Det här formatet skulle provas först.
formatwizard-library =
    { $files ->
        [one] { $files } fil
       *[other] { $files } filer
    } i { $packs ->
        [one] { $packs } paket
       *[other] { $packs } paket
    } har ett namn som det tar. Av { $tried } provade läser det { $read }.
formatwizard-library-none = Ingen fil i biblioteket har ett namn som det tar.
formatwizard-share = Dela det
formatwizard-share-hint = En zip-fil med formatet, den här filen som exempel och vad exemplet läses som, ordnade som i Hoards kodförråd, att skicka in så att alla får det. Ta först bort allt i exemplet som inte ska delas.
formatwizard-sample = Exempel att dela
formatwizard-download = Ladda ner { $file }
formatwizard-close = Stäng
formatwizard-use = Använd här
formatwizard-cancel = Avbryt
