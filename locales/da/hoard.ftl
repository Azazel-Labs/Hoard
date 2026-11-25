## Serverfejl

error-account-not-found = kontoen blev ikke fundet
error-asset-gone = assettet blev ikke fundet
error-asset-not-found = assettet { $id } blev ikke fundet
error-blob-not-found = blob { $hash } blev ikke fundet
error-collection-not-found = samlingen { $id } blev ikke fundet
error-connection-not-found = forbindelsen blev ikke fundet
error-derivative-not-found = { $kind }-afledningen af { $id } blev ikke fundet
error-file-in-pack-not-found = { $path } i pakken { $id } blev ikke fundet
error-site-not-a-page = kun websider åbnes som sider
error-site-link-expired = denne sides adresse er udløbet; åbn siden igen fra Hoard
error-format-not-found = formatet { $file } blev ikke fundet
error-frame-sheet-aseprite-only = kun Aseprite-filer har et frame sheet
error-frame-sheet-frames = denne har ét billede, eller for mange til at stille op
error-location-not-found = placeringen { $id } blev ikke fundet
error-maintenance-task-not-found = vedligeholdelsesopgaven { $id } blev ikke fundet
error-open-upload-not-found = den åbne upload { $id } blev ikke fundet
error-original-not-found = den originale fil til { $id } blev ikke fundet
error-pack-not-found = pakken { $id } blev ikke fundet
error-preview-too-big = billedet er for stort til at lave en let forhåndsvisning af
error-project-not-found = projektet { $name } blev ikke fundet
error-saved-search-not-found = den gemte søgning { $id } blev ikke fundet
error-sign-in-first = log ind først (eller send et adgangstoken)
error-those-assets-not-found = de assets blev ikke fundet
error-thumbnail-not-found = miniaturen til { $id } blev ikke fundet
error-token-not-found = tokenet blev ikke fundet
error-upload-not-found = uploaden { $id } blev ikke fundet
error-pack-needs-name = en pakke skal have et navn
error-no-library = der er endnu ingen biblioteksplacering at gemme filer i: tilføj en administreret placering under Indstillinger → Biblioteksplaceringer
error-upload-failed = upload mislykkedes: { $reason }
error-upload-no-files = ingen filer i uploaden
error-analyze-nothing-to-do = intet at gøre: rules=false, og LLM er slået fra
error-too-many-assets = for mange assets i én forespørgsel
error-change-not-found = den ændring findes ikke i dette assets historik
error-bulk-by-look = en søgning efter udseende kan ikke redigeres samlet: vælg i stedet aktiverne
error-too-many-items = for mange elementer i én forespørgsel
error-not-a-hash = ikke en blake3-hash i hex
error-merge-into-tag = into skal være et tag
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = "{ $text }" er ikke et tag
error-not-a-character = kun en figur kan være den, dens skelets animationer afspilles på
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } er allerede under { $tag }, så { $tag } kan ikke komme under det
error-nothing-to-merge = intet at flette
error-selection-size = angiv mellem 1 og { $max } asset-id'er
error-collection-needs-name = en samling skal have et navn
error-collection-exists = der findes allerede en samling ved navn { $name }
error-collection-empty = samlingen { $name } er tom
error-conversion-failed = kunne ikke konvertere til { $format }: { $reason }
error-max-size = max skal være en potens af to fra 64 til 16384
error-replace-ext = ext skal være et kort filtypenavn
error-empty-file = tom fil
error-imitation-silent = Intet i optagelsen skiller sig ud fra baggrunden. Prøv igen tættere på mikrofonen.
error-replace-duplicate = den konverterede fil er en kopi af en, der allerede er i pakken; assettet blev flettet ind i den
error-ai-address = AI-adressen skal starte med http:// eller https://
error-hosted-key = Indtast API-nøglen til { $provider }
error-hosted-address = Billedsøgningstjenestens adresse skal starte med http:// eller https://
error-local-llm-action = action er list, download eller delete, og de to sidste kræver en model
error-role = En rolle er view, edit eller admin
error-admin-only = kun en administrator kan gøre dette
error-view-only = denne konto kan se, men ikke ændre noget: en administrator kan ændre det under Indstillinger → Konto
error-portal-only = log ind på portalen for at administrere konti og tokens
error-not-from-portal = denne forespørgsel kom ikke fra Hoard-portalen
error-account-name = Vælg et navn på højst 64 tegn
error-password-length = Brug en adgangskode på mindst { $min } tegn
error-too-many-codes = For mange forkerte koder: vent { $seconds } sekunder
error-already-set-up = Der findes allerede en konto: log ind
error-too-many-tries = For mange forkerte forsøg: vent { $seconds } sekunder
error-directory-unavailable = Kataloget kunne ikke kontrollere din adgangskode: { $reason }
error-password-elsewhere = Din adgangskode opbevares af det katalog eller den udbyder, du logger ind med
error-current-password = Din nuværende adgangskode er forkert
error-account-source = En konto logger ind med Hoard (internal), et katalog (ldap) eller OAuth
error-account-exists = Der findes allerede en konto med det navn
error-own-role = En anden administrator kan ændre din rolle
error-remove-self = Du kan ikke fjerne din egen konto, mens du er logget ind på den
error-token-name = Navngiv tokenet efter det, der skal bruge det, med højst 80 tegn
error-token-role = Et token kan ikke mere end den konto, der laver det
error-wrong-setup-code = Det er ikke den opsætningskode, serveren har fået i HOARD_SETUP_CODE
error-wrong-password = Navnet og adgangskoden passer ikke sammen
error-folder-missing = { $path } findes ikke (en disk, der ikke er monteret?)
error-s3-path = en S3-placering ser ud som s3://bucket
error-s3-address = S3-adressen skal starte med http:// eller https://
error-s3-header = en ekstra S3-header ser ud som Name=value, og »{ $header }« er ikke en
error-keychain = kunne ikke gemme det i systemets nøglering: { $reason }
error-not-built = denne kopi af Hoard er bygget uden { $feature }
error-openai-auth-unreachable = kunne ikke nå auth.openai.com
error-openai-unexpected = uventet svar fra OpenAI
error-openai-answered = OpenAI svarede { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID-tokenet er ikke en JWT
error-chatgpt-id-token-issuer = ID-tokenet er fra { $issuer }, ikke { $expected }
error-chatgpt-id-token-audience = ID-tokenet er til en anden app
error-chatgpt-id-token-expired = ID-tokenet er udløbet
error-chatgpt-id-token-nonce = ID-tokenet hører ikke til dette login
error-chatgpt-no-state = adressen har ingen state; kopiér hele adressen
error-chatgpt-not-started = dette login blev ikke startet her, eller det blev startet for mere end 15 minutter siden. Start det igen fra Indstillinger.
error-chatgpt-canceled = login blev annulleret
error-chatgpt-refused = ChatGPT afviste login: { $reason }
error-chatgpt-no-code = adressen har ingen login-kode; kopiér hele adressen
error-chatgpt-other-registration = ChatGPT svarede for en anden registrering af denne server; start login forfra
error-chatgpt-other-registration-desktop = ChatGPT svarede for en anden kopi af Hoard; start login forfra
error-chatgpt-not-registered = ChatGPT blev ikke færdig med at registrere Hoard; start login forfra
error-chatgpt-code-used = login-koden er allerede brugt eller udløbet; log ind igen
error-chatgpt-refused-code = ChatGPT afviste login ({ $code })
error-chatgpt-no-id-token = ChatGPT sendte intet ID-token
error-chatgpt-plan-not-allowed = Logget ind, men det blev ikke tilladt at bruge abonnementet. Log ind igen og tillad det, eller brug en API-nøgle i stedet.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = ikke logget ind på ChatGPT; log ind under Indstillinger → AI-tagging
error-chatgpt-refresh-later = ChatGPT kunne ikke forny login lige nu ({ $code }); det prøves igen senere
error-chatgpt-signed-out-problem = ChatGPT loggede denne server ud ({ $code }). Log ind igen for at blive ved med at tagge med dit abonnement.
error-chatgpt-signed-out-problem-desktop = ChatGPT loggede Hoard ud ({ $code }). Log ind igen for at blive ved med at tagge med dit abonnement.
error-chatgpt-signed-out = ChatGPT loggede denne server ud ({ $code }); log ind igen under Indstillinger → AI-tagging
error-chatgpt-signed-out-desktop = ChatGPT loggede Hoard ud ({ $code }); log ind igen under Indstillinger → AI-tagging
error-openai-api-unreachable = kunne ikke nå api.openai.com
error-openai-broke-off = svaret fra OpenAI blev afbrudt
error-chatgpt-kept-refusing = OpenAI blev ved med at afvise ChatGPT-login; log ind igen under Indstillinger → AI-tagging
error-chatgpt-request-failed = forespørgslen mislykkedes
error-chatgpt-cut-off = svaret blev afkortet ({ $reason })
error-chatgpt-cut-off-no-reason = svaret blev afkortet (ingen årsag angivet)
error-openai-ended-early = svaret fra OpenAI sluttede, før det var færdigt
error-chatgpt-usage-limit = dit ChatGPT-abonnements forbrug til Hoard er opbrugt for nu. Se eller ændr grænsen på { $url }
error-chatgpt-not-eligible = denne ChatGPT-konto eller -workspace kan ikke bruge sit abonnement i andre apps. Brug en API-nøgle i stedet.
error-chatgpt-usage-unavailable = ChatGPT kunne ikke kontrollere abonnementets forbrug lige nu; det prøves igen senere
error-chatgpt-unsupported = brug via ChatGPT-abonnement understøtter ikke { $param } med denne model
error-chatgpt-unsupported-images = brug via ChatGPT-abonnement understøtter ikke { $param } med denne model; vælg en model, der tager billeder, eller slå afsendelse af miniaturer fra
error-chatgpt-unsupported-part = brug via ChatGPT-abonnement understøtter ikke en del af forespørgslen med denne model
error-chatgpt-port = port er den, browseren når Hoard på
error-chatgpt-paste-address = indsæt hele adressen, der starter med http://127.0.0.1
error-too-many-hashes = højst { $max } hashes ad gangen
error-credits-pack = pakken { $key }
error-credits-collection = samlingen { $key }
error-format-too-long = formatet er for langt
error-not-started = serveren er ikke startet
error-not-started-desktop = Hoard er ikke startet
error-not-format-file = ikke en formatfil: { $name }
error-format-name = navnet skal have et bogstav eller ciffer fra A til Z eller 0 til 9
error-format-miss-name = formatets filer tager ikke { $file }
error-format-miss-store = formatet tager ikke { $file } for butikkens (se require)
error-format-miss-title = formatet finder ingen titel i { $file }
error-cant-download-as = .{ $ext }-filer kan ikke downloades som { $format }
error-only-images-resize = kun billeder kan skaleres
error-https-no-certificate = Indsæt eller upload først dit certifikat og dets nøgle
error-https-no-cert-found = Intet certifikat fundet: indsæt teksten, der starter med -----BEGIN CERTIFICATE-----
error-https-no-key-found = Ingen privat nøgle fundet: indsæt teksten, der starter med -----BEGIN PRIVATE KEY-----
error-https-several-keys = Der er mere end én privat nøgle: indsæt kun den, der hører til certifikatet
error-https-key-password = Nøglen er beskyttet med en adgangskode: angiv den uden (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Certifikatet er ikke gyldig PEM
error-https-key-not-pem = Den private nøgle er ikke gyldig PEM
error-https-key-mismatch = Nøglen hører ikke til certifikatet. Hvis du har angivet en kæde, så sæt din servers certifikat først
error-https-unusable = Certifikatet eller nøglen kan ikke bruges
error-https-expired = Dette certifikat udløb den { $date }
error-https-not-yet-valid = Dette certifikat er først gyldigt fra { $date }
error-https-unreadable = Det er ikke et certifikat, Hoard kan læse
error-https-bad-name = »{ $name }« er ikke en adresse, et certifikat kan indeholde
error-https-test-timed-out = testforbindelsen fik timeout
error-https-test-failed = testforbindelsen mislykkedes
error-https-no-ca = Hoard har endnu ikke lavet sin certifikatmyndighed
error-https-unavailable = HTTPS er ikke tilgængelig på denne server
error-https-certificate-mode = Et certifikat hører til »Mit certifikat«
error-https-test = Certifikatet virkede ikke i en testforbindelse: { $reason }
error-nothing-to-retry = der er intet at prøve igen
error-ai-unreachable = kunne ikke nå { $url }
error-ai-key-refused = { $url } afviste API-nøglen ({ $status })
error-ai-answered = { $url } svarede { $status }: { $reason }
error-ai-not-json = { $url }/models svarede ikke med JSON
error-model-server-refused = modelserveren: { $reason }
error-no-model-server = ingen modelserver svarer på { $url }
error-not-llama-router = { $url } er ikke en llama.cpp-server i router-tilstand, så den kan ikke downloade modeller
error-model-download = kunne ikke downloade { $model }: { $reason }
error-model-delete = kunne ikke slette { $model }: { $reason }
error-unknown-kind = ukendt assettype { $kind }
error-watched-is-folder = en overvåget placering er en mappe på denne server
error-cant-reach = kan ikke nå { $path }: { $reason }
error-folder-full-path = angiv mappens fulde sti
error-cant-create = kan ikke oprette { $path }
error-cant-write = kan ikke skrive til { $path }
error-cant-read = kan ikke læse { $path }
error-overlaps-location = det overlapper placeringen { $name } ({ $path })
error-overlaps-data = det overlapper Hoards datamappe ({ $path })
error-overlaps-cache = det overlapper Hoards cachemappe ({ $path })
error-not-watched = { $name } er ikke en overvåget mappe
error-watched-no-kinds = en overvåget mappe tager ikke imod nye filer, så den har ingen typer at ændre
error-keep-catch-all = gør i stedet en anden placering til opsamlingsstedet
error-location-holds-files = { $name } indeholder stadig filer: flyt dem først til de andre placeringer (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } kan ikke nås, så Hoard kan ikke se, om den stadig indeholder filer
error-relocate-running = der flyttes allerede filer mellem placeringer; prøv igen, når det er færdigt
error-watched-not-moved = { $name } er en overvåget mappe: Hoard flytter ikke dens filer
error-no-other-location = der er ingen anden biblioteksplacering at flytte filerne til: tilføj en først
error-full-path = angiv en fuld sti
error-pick-folder = vælg en mappe
error-task-running = »{ $task }« kører allerede
error-job-cant-stop = »{ $job }« kan ikke stoppes, når det først er gået i gang
error-job-not-found = det job venter eller kører ikke længere
error-task-cant-stop = »{ $task }« kan ikke stoppes halvvejs
error-task-not-running = »{ $task }« kører ikke
error-token-account-gone = dette tokens konto findes ikke længere, så det har ingen favoritter eller søgninger
error-favorites-too-many = højst 10.000 ad gangen
error-saved-search-name = en gemt søgning skal have et navn på højst 80 tegn
error-saved-search-too-long = den søgning er for lang til at gemme
error-hoard-seen = skatten kan højst registrere 32 slags assets, hver navngivet med små bogstaver
error-news-seen = nyheder kan højst registrere 256 sete ting, hver navngivet med små bogstaver, tal og bindestreger
error-texture-pattern-map = Mønsteret for teksturnavne skal indeholde { "{" }map{ "}" }, ellers ville alle et materiales maps få samme navn
error-texture-pattern-fields = Mønsteret for teksturnavne kan bruge { "{" }name{ "}" }, { "{" }map{ "}" } og { "{" }size{ "}" }
error-texture-name-chars = Teksturnavne må ikke indeholde < > : " / \ | ? eller *
error-texture-map-type = Der findes ingen map-type ved navn { $role }
server-role-base-color = farve
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = ruhed
server-role-gloss = glans
server-role-metalness = metal
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = højde
server-role-bump = bump
server-role-cavity = cavity
server-role-specular = specular
server-role-opacity = opacitet
server-role-emissive = emission
server-role-translucency = gennemskinnelighed
error-texture-suffix-missing = Giv maps af typen { $map } et suffiks
error-texture-suffix-chars = Suffikset for maps af typen { $map } må ikke indeholde < > : " / \ | ? eller *
error-texture-ending-missing = Hver af dine egne endelser skal have den endelse, der skal søges efter
error-texture-ending-chars = Dine egne endelser må ikke indeholde < > : " / \ | ? eller *
error-texture-ending-twice = { $ending } står to gange i dine endelser
error-oauth-no-client-id = Dette link har ingen client_id.
error-oauth-unknown-app = Denne app er ikke (længere) registreret her. Start forbindelsen forfra fra appen.
error-oauth-redirect = Appen bad om at blive sendt tilbage til en adresse, den ikke har registreret.
error-oauth-response-type = Hoard udleverer kun login-koder (response_type=code).
error-oauth-pkce = Appen skal bruge PKCE med S256 for at logge ind på Hoard.
error-light-preview-images = kun billeder har en let forhåndsvisning
error-no-light-preview = ingen let forhåndsvisning: { $reason }
error-no-frame-sheet = intet frame sheet: { $reason }
error-thumbnail-source = ukendt kilde til miniature: { $source }
error-thumbnail-too-big = det billede er for stort til en miniature
error-thumbnail-models-only = kun modeller kan få en miniature tegnet af en browser
error-not-a-picture = ikke et billede, Hoard kan læse: { $reason }
error-project-name = et projekt skal have et navn på højst 200 tegn
error-project-too-big = højst en million assets i ét projekt
error-project-too-many-at-once = højst 100.000 ad gangen
error-server-name = Vælg et navn på højst { $max } tegn, på én linje
error-tool-setting = { $tool }: vælg fra, automatisk, eller angiv en sti
error-ai-model = Vælg den model, AI-tjenesten skal bruge
error-public-address = Den offentlige adresse skal starte med http:// eller https://
error-update-hook-address = Adressen, der skal kaldes ved opdateringer, skal starte med http:// eller https://
error-update-hook-auth = Authorization-headeren må ikke indeholde linjeskift eller andre kontroltegn
error-no-soundfont = Der er ingen SoundFont på { $path } på serveren
error-no-soundfont-desktop = Der er ingen SoundFont på { $path }
error-soundfont-bad = { $name } er ikke en SoundFont, MIDI kan spille med: { $reason }
error-workers = Baggrundsarbejdere: mellem 1 og { $max }
error-ai-batch = Assets pr. AI-forespørgsel: mellem 1 og 200
error-conversion-cache = Konverterede downloads: mindst 100 MB
error-motion-fps = levende miniaturer afspilles med 12, 24 eller 30 billeder i sekundet
error-thumb-size = miniaturer er 128 eller 256 pixel brede
error-backup-keep = Planlagte sikkerhedskopier: behold 1 til { $max }
error-backup-copy-to = Planlagte sikkerhedskopier: { $path } er ikke en mappe på serveren (angiv den fulde sti)
error-conversion-keep-days = Konverteringsregler: originaler kan beholdes i 1 til { $max } dage
error-conversion-target = Konverteringsregler: Hoard kan ikke konvertere filer til { $format }
error-conversion-no-source = Konverteringsregler: hver regel skal have mindst ét format at konvertere
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Konverteringsregler: .{ $ext }-filer kan ikke konverteres til { $format }
error-conversion-twice = Konverteringsregler: .{ $ext } er med i to regler
error-conversion-cant-undo = Denne fil blev ikke konverteret af en regel, eller dens original er blevet slettet.
error-conversion-path-taken = Der ligger en anden fil på { $path } nu, så originalen kan ikke komme tilbage dertil.
error-limits-zero = Grænser kan ikke være nul
error-settings-object = forventede et JSON-objekt med indstillinger
error-bad-setting = ugyldig indstilling: { $reason }
error-auth-default-role = Standardrollen er view, edit eller admin
error-auth-session-hours = Sessioner varer mellem 1 time og et år
error-auth-settings = ugyldige godkendelsesindstillinger: { $reason }
error-ldap-settings = ugyldige LDAP-indstillinger: { $reason }
error-ldap-no-server = Tilføj katalogserverens adresse
error-ldap-base-dn = Angiv, hvor folks poster ligger (base DN)
error-ldap-username-attribute = Angiv, hvilken attribut der indeholder det navn, folk logger ind med
error-ldap-bind-dn = Angiv den konto, der slår folk op, eller slå dem op anonymt
error-ldap-ca-cert = Certifikatmyndigheden er ikke et PEM-certifikat (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = En katalogservers adresse er tom
error-ldap-server-form = { $server }: en katalogservers adresse er host:port, ldap://… eller ldaps://…
error-ldap-not-starttls = { $server }: det er LDAPS, ikke StartTLS
error-ldap-no-certificate = intet certifikat i den
error-ldap-password-again = Angiv igen adgangskoden til den konto, der slår folk op: den gemte sendes kun til de servere, den blev angivet til
error-ldap-unreachable = ingen katalogserver kunne nås ({ $reasons })
error-ldap-no-anonymous = kataloget tillader ikke at slå folk op anonymt ({ $reason })
error-ldap-bind-failed = den konto, der slår folk op, kunne ikke logge ind ({ $reason })
error-ldap-search = søgning efter { $name } under { $base }
error-ldap-group-search = søgning efter grupperne for { $name } under { $base }
error-ldap-several = { $count } poster i kataloget matcher { $name }: indsnævr brugerfilteret
error-ldap-password-check = kataloget ville ikke kontrollere adgangskoden for { $name } ({ $reason })
error-ldap-wrong-password = Kataloget kender ikke { $name } med den adgangskode
error-oauth-client-id = Angiv det klient-id, udbyderen har lavet til Hoard
error-oauth-addresses = Godkendelses- og token-adresserne skal starte med https://
error-oauth-userinfo-address = Adressen til brugeroplysninger skal starte med https://
error-oauth-username-claim = Angiv, hvilket claim der indeholder personens navn
error-oauth-secret-again = Angiv klienthemmeligheden igen: den gemte sendes kun til den token-adresse, den blev angivet til
error-oauth-issuer = Udstederen skal starte med https://
error-oauth-discovery-unreachable = Kunne ikke nå { $url }: { $reason }
error-oauth-discovery-status = { $url } svarede { $status }
error-oauth-discovery-bad = { $url } er ikke et discovery-dokument
error-oauth-discovery-incomplete = { $url } angiver ikke godkendelses- og token-adresserne
error-signin-no-group = { $name } er ikke i en gruppe, der må bruge Hoard
error-signin-other-source = Der findes allerede en Hoard-konto ved navn { $name }, der logger ind på en anden måde
error-signin-other-subject = Der findes allerede en Hoard-konto ved navn { $name } for en anden person hos udbyderen
error-signin-no-account = { $name } har endnu ingen Hoard-konto: en administrator kan tilføje den under Indstillinger → Konto
error-signin-bad-name = Det navn kan ikke være en Hoard-konto (højst 64 tegn)
error-oauth-off = Login med OAuth er ikke slået til
error-oauth-bad-authorization-address = godkendelsesadressen er ikke gyldig
error-oauth-refused = Udbyderen loggede dig ikke ind: { $reason }
error-oauth-no-code = Udbyderen sendte dig tilbage uden et login
error-oauth-expired = Det login tog for lang tid eller blev startet et andet sted: prøv igen
error-oauth-no-claim = Udbyderen sagde ikke, hvem du er (intet { $claim }-claim)
error-oauth-no-sub = Udbyderen sagde ikke, hvem du er (intet sub- eller id-claim)
error-oauth-token-unreachable = kunne ikke nå udbyderens token-adresse
error-oauth-token-refused = Udbyderen ville ikke fuldføre login: { $reason }
error-oauth-no-access-token = udbyderen sendte intet adgangstoken
error-oauth-userinfo-unreachable = kunne ikke nå udbyderens adresse til brugeroplysninger
error-oauth-userinfo-refused = Udbyderen ville ikke sige, hvem der loggede ind ({ $status })
error-oauth-userinfo-not-json = udbyderens brugeroplysninger er ikke JSON
error-format-no-name = name er tom
error-format-no-files = files er tom: angiv, hvilke filer formatet læser, f.eks. ["*.json"]
error-format-empty-selector = tom
error-format-unopened-bracket = en lukkende parentes uden en åbnende
error-format-unclosed-bracket = en parentes er ikke lukket
error-format-empty-step-dots = et tomt trin (to punktummer i træk, eller et punktum i en af enderne)
error-format-unclosed-except = !( er ikke lukket
error-format-unclosed-filter = [ er ikke lukket
error-format-after-filters = { $rest } efter nøglen og filtrene for { $step }
error-format-except-needs-glob = !(…) skal stå efter en nøgle med * i
error-format-except-not-glob = !(…) skal stå efter en nøgle med * i, ikke { $key }
error-format-empty-step = et tomt trin
error-format-not-a-filter = [{ $filter }] er ikke et filter: brug [], [key=value|other], [array], [string], [number], [bool] eller [object]
error-format-filter-needs-value = [{ $filter }] skal have en nøgle og en værdi
error-store-search-empty = intet at søge efter; skriv pakkens navn
error-update-hook-not-set = angiv først en adresse, der skal kaldes
error-upload-stalled = upload af { $path } gik i stå; send den del igen
error-upload-busy = { $path } er allerede ved at blive skrevet
error-upload-incomplete = { $count } filer er ufuldstændige, f.eks. { $files }
error-upload-no-files-listed = ingen filer
error-upload-not-part = { $path } er ikke en del af uploaden { $id }, eller uploaden er lukket
error-bad-path = ugyldig sti
error-upload-part-offset = { $offset } er ikke der, hvor en del af { $path } starter
error-upload-interrupted = upload afbrudt: { $reason }
error-upload-part-long = del { $part } af { $path } er længere end { $len } bytes
error-upload-part-short = del { $part } af { $path }: fik { $got } bytes, forventede { $len }
error-upload-file-long = { $path } er større end angivet
error-upload-file-short = { $path }: fik { $got } bytes, forventede { $len }
error-upload-complete = uploaden er allerede fuldført
error-already-imported = allerede importeret som »{ $name }«

## Servertekst

server-store-place-readme = README
server-store-place-folders = mapper
server-store-why-linked = Linket i pakkens { $place }
server-store-why-same-title = Samme titel som pakkens { $place }
server-store-why-by = Af { $author }, som er nævnt i pakkens { $place }
server-store-why-meshes = Omtrent lige så mange meshes som pakken ({ $count })
server-store-why-textures = Omtrent lige så mange teksturer som pakken ({ $count })
server-chatgpt-revoke-failed = Logget ud her, men OpenAI kunne ikke få besked. For at være sikker kan du afbryde forbindelsen til Hoard i ChatGPT under Indstillinger → Sikkerhed og login → Log ind med ChatGPT.
server-chatgpt-signed-in-title = Logget ind på ChatGPT
server-chatgpt-signed-in = Hoard er logget ind. AI-tagging kan nu bruge dit ChatGPT-abonnement.
server-chatgpt-signed-in-as = Hoard er logget ind som { $email }. AI-tagging kan nu bruge dit ChatGPT-abonnement.
server-chatgpt-not-finished-title = Login blev ikke fuldført
server-chatgpt-other-computer = Hvis du startede login fra Hoard på en anden computer, så kopiér denne sides adresse fra adresselinjen, og indsæt den i Hoards Indstillinger dér.
server-back-to-settings = Tilbage til Indstillinger
server-issues-pack = Pakker, der ikke kunne tilføjes
server-issues-pack-blurb = Slet pakken, og tilføj den igen, eller åbn den for at se, hvad den indeholder.
server-issues-damaged = Gemte filer, der er beskadigede eller forsvundet
server-issues-damaged-blurb = Fundet af »Kontrollér filer for skader«. Gendan dem fra en sikkerhedskopi, eller tilføj deres pakke igen, og kør derefter »Kontrollér filer for skader« igen.
server-issues-folder = Overvågede mapper, der ikke kan læses
server-issues-folder-blurb = Deres pakker bliver ladt, som de er, indtil mappen kan læses igen.
server-issues-missing = Overvågede pakker, hvis mappe er forsvundet
server-issues-missing-blurb = Hver pakke beholdes i 3 dage, hvis nu mappen bare er offline, og fjernes derefter.
server-issues-job = Baggrundsjob, der mislykkedes
server-issues-job-blurb = Prøv igen kører jobbet forfra.
server-issues-file = Filer, der ikke kunne åbnes
server-issues-file-blurb = Hoard kunne ikke åbne eller gemme dem. En overvåget pakke prøver dem igen ved næste scanning; en fil, der allerede er i biblioteket, beholdt det, den havde før.
server-issues-unpack = Arkiver, der ikke kunne pakkes ud
server-issues-unpack-blurb = Hvert arkiv beholdes i sin pakke som en almindelig fil, så intet i det er i biblioteket.
server-issues-read = Filer, Hoard ikke kunne forstå
server-issues-read-blurb = De er i biblioteket, men uden de oplysninger, tags eller den miniature, som læsningen af dem giver.
server-issues-render = Modeller, som miniature-rendereren fejlede på
server-issues-render-blurb = De har ingen miniature. Det kan stadig virke at åbne en i fremviseren, og det kan gemme en.
server-issues-convert = Modeller, Blender ikke kunne konvertere
server-issues-convert-blurb = De har ingen GLB-kopi, så de kan ikke vises i 3D.
server-issues-unreal = Unreal-pakker, der ikke kunne konverteres
server-issues-unreal-blurb = Pakkens Unreal-side har resten af dens Unreal-pakker.
server-issues-reformat = Filer, en konverteringsregel ikke kunne konvertere
server-issues-reformat-blurb = De er ladt, som de var.
server-issues-materials = Pakker, hvis materialer ikke kunne regnes ud
server-issues-materials-blurb = Deres teksturer er der alle sammen; de er bare ikke grupperet i materialer.
server-issues-material-thumb = Materialer uden miniature
server-issues-material-thumb-blurb = Materialet blev fundet, men dets forhåndsvisning kunne ikke tegnes.
server-issues-store-files = Butiksfiler, der ikke kunne læses
server-issues-store-files-blurb = Filer fra den butik, en pakke kom fra, f.eks. en Megascans-JSON, som navngiver og tagger den.
server-issues-scan-again = Scan igen
server-issues-retry = Prøv igen
server-issues-reanalyze = Analysér pakken igen
server-issues-convert-again = Konvertér igen
server-issues-look-again = Se igen
server-issues-read-again = Læs igen
server-issues-blob-missing = Den gemte fil er forsvundet.
server-issues-blob-damaged = Den gemte fil passer ikke med det, der blev gemt: den er beskadiget.
server-issues-folder-gone = Mappen »{ $folder }« blev ikke fundet.
server-job-a-pack = en pakke
server-job-ingest = Indlæser { $pack }
server-job-retag = Tagger { $pack } igen
server-job-llm-tag = Tagger { $pack } med AI
server-job-convert = Konverterer { $pack } med Blender
server-job-unreal = Konverterer Unreal-pakker i { $pack }
server-job-reformat = Konverterer filer i { $pack } efter konverteringsreglerne
server-job-scan = Scanner en overvåget mappe
server-job-relocate = Flytter filer mellem biblioteksplaceringer
server-job-looks = Finder farver og udseende
server-job-sounds = Lytter til lyde
server-job-embed = Lærer, hvad billeder viser
server-job-hear = Lærer, hvordan lyde lyder
server-job-materials = Finder materialer i { $pack }
server-job-read-files = Læser filerne i { $pack }
server-job-store-lookup = Slår en butiksside op
server-maintenance-backup = Sikkerhedskopiér databasen
server-maintenance-backup-blurb = En konsistent kopi af databasen (pakker, tags, noter, samlinger og Indstillinger) i backup-mappen ved siden af den. Bibliotekets filer er ikke med: de ændres aldrig, når de først er gemt, så enhver filsynkronisering sikkerhedskopierer dem.
server-maintenance-scrub = Kontrollér filer for skader
server-maintenance-scrub-blurb = Læser hver gemt fil igen og kontrollerer den mod dens fingeraftryk for at fange filer, der er rådnet eller forsvundet fra en disk eller NAS. Tager et stykke tid på et stort bibliotek.
server-maintenance-reanalyze = Analysér alle pakker igen
server-maintenance-reanalyze-blurb = Læser hver fil igen med denne version af Hoard: miniaturer, forhåndsvisninger, detaljer og regel-tags laves på ny. Dine egne ændringer bliver; AI'en spørges ikke igen.
server-maintenance-clear-cache = Ryd cachelagrede konverteringer
server-maintenance-clear-cache-blurb = Smider de filer væk, der er gemt fra »Download som«-konverteringer (de laves igen, næste gang der bedes om dem), og det, afbrudt arbejde har efterladt.
server-maintenance-gc = Slet ubrugte filer
server-maintenance-gc-blurb = Sletter gemte filer, som intet længere peger på, f.eks. filerne fra pakker, der er slettet for altid.
server-update-no-address = der er ikke angivet nogen adresse at kalde
server-update-no-answer = intet svar inden for 30 sekunder

## Indstillinger: Udseende

appearance-language = Sprog
appearance-language-auto = Browserens ({ $name })
appearance-scheme = Lyst eller mørkt
appearance-scheme-system = System
appearance-size-section = Tekst og afstand
appearance-text-size = Tekststørrelse
appearance-text-small = Lille
appearance-text-normal = Normal
appearance-text-large = Stor
appearance-density = Afstand
appearance-density-comfortable = Luftig
appearance-density-compact = Kompakt
appearance-tiles = Felter
appearance-tiles-what = Hvad hvert felt i biblioteket viser ud over billedet.
appearance-tile-names = Navne
appearance-tile-details = Type og størrelse (pixels, længde, trekanter)
appearance-tile-badges = Filtype på billedet
appearance-tile-motion = Afspil animationer, mens markøren er over dem
appearance-tile-motion-always = Afspil animationer hele tiden, ikke kun under markøren
appearance-top-bar = Topbjælke
appearance-top-bar-what = Udelad det, du ikke bruger. / går stadig til søgefeltet, og { $keys } åbner stadig kommandopaletten.
appearance-top-activity = Baggrundsarbejde
appearance-top-theme = Skift mellem lyst og mørkt
appearance-top-mic = Mikrofon i søgefeltet, til at finde lyde ved at lave en
appearance-top-slash = /-tippet i søgefeltet
appearance-top-palette = Knap til kommandopaletten ({ $keys })
appearance-news = Nyheder
appearance-news-what = Nye ting i Hoard får en glimtende guldprik, indtil du har set dem, og topbjælken samler dem ét sted. Det, du har set, følger din konto.
appearance-news-point-out = Gør opmærksom på nye ting

## Rod

root-unreachable = Kunne ikke nå Hoard-serveren: { $error }
root-unreachable-desktop = Hoard svarer ikke: { $error }
root-desktop-reopen = Dette vindue er logget ud. Luk Hoard, og åbn det igen for at fortsætte.
root-try-again = Prøv igen
root-lazy-connect = app-forbindelsen
window-minimize = Minimer
window-maximize = Maksimer
window-restore = Gendan
window-close = Luk

## App: biblioteket, dets topbjælke og filtre

app-lazy-settings = indstillinger
app-lazy-archive = arkivet
app-lazy-shortcuts = genvejene
app-lazy-setup = opsætningen
app-lazy-explorer = mesh-udforskeren
app-theme-system = Følg systemets tema
app-theme-light = Lyst tema
app-theme-dark = Mørkt tema
app-color-red = Rød
app-color-orange = Orange
app-color-yellow = Gul
app-color-green = Grøn
app-color-teal = Blågrøn
app-color-blue = Blå
app-color-purple = Lilla
app-color-pink = Lyserød
app-color-brown = Brun
app-color-sand = Sand
app-color-gray = Grå
app-color-slate = Skifer
app-color-black = Sort
app-color-white = Hvid
app-color-swatch = { $color }: assets med meget { $lower } i
app-color-any = Enhver farve
app-color-pick = Vælg en vilkårlig farve
app-color = Farve
app-music = Musik
app-tempo = BPM
app-tempo-min = fra
app-tempo-max = til
app-tempo-min-title = Musik i dette tempo eller hurtigere, i slag i minuttet (halvt eller dobbelt tæller også)
app-tempo-max-title = Musik i dette tempo eller langsommere, i slag i minuttet
app-key = Toneart
app-key-any = Enhver toneart
app-key-major = Dur
app-key-minor = Mol
app-key-mix = Og tonearter, der kan mixes med den
app-key-mix-title = Dens paralleltoneart i dur eller mol og tonearterne en kvint på hver side, som dj's mixer den med
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Skriv den lyd, noget siger, som du ville stave den: lyde, der ligner, kommer først
app-imitation-record = Lav lyden i mikrofonen: lyde, der ligner, kommer først
app-imitation-stop = Stop optagelsen
app-imitation-heard = Din lyd
app-imitation-mic-failed = Kunne ikke optage fra mikrofonen: { $error }
app-retry = Prøv igen
app-undo = Fortryd
app-search-failed = Søgningen mislykkedes: { $error }
app-unreachable = Kunne ikke nå serveren: { $error }
app-unreachable-desktop = Hoard svarer ikke: { $error }
app-forget-project-title = Glem { $name }?
app-forget-project-body = Hoard holder op med at vise det, hvor dets { $count } assets bruges, indtil næste hoard sync fortæller om det igen. Intet i projektet ændres.
app-forget-project-confirm = Glem det
app-forget-project-failed = Kunne ikke glemme { $name }: { $error }
app-forget-project-label = Glem projektet { $name }
app-forget-project = Glem dette projekt
app-save-name-like = ligner { $name }
app-save-name-favorites = favoritter
app-search-saved = Gemte »{ $name }«: den ligger i Filtre under Dine
app-search-updated = Opdaterede »{ $name }«
app-rank-alike = Mest ens
app-rank-color = Mest af farven
app-rank-match = Bedste match
app-rank-imitation = Lyder mest som det
app-sort = Sortér
app-sort-name = Navn
app-sort-created = Oprettet
app-sort-updated = Opdateret
app-sort-size = Størrelse
app-sort-rating = Vurdering
app-sort-newest-first = Nyeste først
app-sort-oldest-first = Ældste først
app-sort-a-to-z = A til Å
app-sort-z-to-a = Å til A
app-sort-largest-first = Største først
app-sort-smallest-first = Mindste først
app-sort-best-rated-first = Bedst vurderet først
app-sort-lowest-rated-first = Lavest vurderet først
app-collection-added =
    Føjede { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } til { $name }
app-collection-add-failed = Kunne ikke føje til samlingen: { $error }
app-collection-removed = Fjernede { $count } fra { $name }
app-collection-remove-failed = Kunne ikke fjerne fra samlingen: { $error }
app-ns-subject = Emne
app-ns-style = Stil
app-ns-theme = Tema
app-ns-material = Materiale
app-ns-map = PBR-map
app-ns-color = Farve
app-ns-mood = Stemning
app-ns-genre = Genre
app-ns-license = Licens
app-ns-res = Opløsning
app-ns-duration = Længde
app-ns-poly = Antal polygoner
app-ns-format = Format
app-ns-weight = Størrelse
app-ns-tech = Teknisk
app-ns-blender = Blender-version
app-ns-depth = Bitdybde
app-ns-ucs = Lydkategori (UCS)
app-ns-bpm = Tempo
app-ns-root = Grundtone
app-ns-level = Lydstyrke
app-ns-ue = Unreal-klasse
app-ns-engine = Motor
app-ns-extracted = Gendannet kilde
app-ns-script = Skriftsystem
app-bulk-failed =
    Kunne ikke ændre { $count ->
        [one] det asset
       *[other] de { $count } assets
    }: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] Kopierede { $count } tag
       *[other] Kopierede { $count } tags
    }
app-tags-copy-none = { $name } har ingen tags at kopiere
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Ingen tags kopieret. Tryk på C på et asset for at kopiere dets tags.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [one] Det har allerede de tags
       *[other] Alle { $count } har allerede de tags
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [one]
            { $count ->
                [one] Tilføjede { $count } tag
               *[other] Tilføjede { $count } tags
            }
       *[other]
            { $count ->
                [one] Tilføjede { $count } tag til { $assets } assets
               *[other] Tilføjede { $count } tags til { $assets } assets
            }
    }
app-bulk-untagged =
    Fjernede { $tag } fra { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-bulk-tagged =
    Taggede { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } med { $tag }
app-bulk-review = Markér som gennemgået
app-bulk-review-title = Markér de valgte assets som gennemgået: regler og AI lader deres tags være herefter
app-bulk-unreview = Markér som ikke gennemgået
app-bulk-unreview-title = Fjern gennemgået-mærket fra de valgte assets
app-bulk-reviewed =
    Markerede { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } som gennemgået
app-bulk-unreviewed =
    Fjernede gennemgået-mærket fra { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-bulk-accept = Behold gæt
app-bulk-accept-title = Behold alle tags, som reglerne og AI har sat på de valgte assets, som dine egne
app-bulk-accepted =
    Beholdt de gættede tags på { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-credits-failed = Kunne ikke lave krediteringerne: { $error }
app-archived-assets =
    Arkiverede { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-archived-one = Arkiverede { $name }
app-open-failed = Kunne ikke åbne det asset: { $error }
app-unnamed-pack = pakke
app-unnamed-collection = samling
app-unnamed-asset = asset
app-home-version = Hoard { $version } — tilbage til biblioteket
app-home = Tilbage til biblioteket
app-search-label = Søg i assets
app-search-placeholder = Søg i navne, stier, tags
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = klippe or sten: en af dem · -mos: udelad · "tør mos": disse ord sammen · pack:skov: i en pakkes navn
app-alt-leave-out = Alt-klik for at udelade det
app-views = Visninger
app-library = Bibliotek
app-packs = Pakker
app-packs-working = { $count } i gang
app-update-title = Hoard { $version } er udkommet (denne server kører { $current }). Hent det nye image eller program, og genstart; databasen opgraderer sig selv.
app-update = Opdatér
app-theme = Tema
app-sign-in = Log ind
app-archive-title = Arkiv: { $contents }
app-archive = Arkiv
app-settings = Indstillinger
app-palette-title = Kommandopalet ({ $keys })
app-import-folder = Importér mappe
app-add-files = Tilføj filer
app-add-files-short = Tilføj
app-split-label = Sådan importeres denne mappe
app-split-before = { "" }
app-split-after = { " " }indeholder { $count } mapper eller arkiver.
app-split-each = Importér hver som sin egen pakke ({ $count })
app-split-one = Importér som én pakke
app-cancel = Annullér
app-filters = Filtre
app-done = Færdig
app-yours = Dine
app-favorites = Favoritter
app-needs-review = Skal gennemgås
app-needs-review-title = Kun assets, som ingen har gennemgået endnu (virker sammen med de andre filtre)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Vurderet
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } stjerne eller flere
       *[other] { $count } stjerner eller flere
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Klik igen for at vise alle vurderinger
app-forget-search-label = Glem den gemte søgning { $name }
app-forget-search = Glem denne søgning
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Åbn biblioteket med denne søgning
app-default-search-on-title = Biblioteket åbner med denne søgning. Klik for at stoppe det.
app-default-search-label = Åbn biblioteket med { $name }
app-rename-search = Omdøb
app-rename-search-label = Nyt navn til { $name }
app-projects-title = Projekter, der kører hoard sync herfra, med det, deres seneste synkronisering havde
app-projects = Projekter
app-project-synced = Synkroniseret { $when }
app-project-synced-by = Synkroniseret { $when } af { $who }
app-collections = Samlinger
app-no-filters = Ingen filtre til denne søgning.
app-kind = Type
app-facets-more = { $count } mere
app-facets-fewer = færre
app-searching = Søger…
app-searching-live = Søger
app-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-clear-all = Ryd alt
app-save-name-label = Navn på denne søgning
app-save-name-placeholder = Giv den et navn
app-save = Gem
app-save-search-title = Behold denne søgning i Filtre under Dine
app-save-search = Gem søgning
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Opdater »{ $name }«
app-update-search-title = Behold det, der er valgt nu, i denne gemte søgning
app-save-search-new = Gem som ny
app-select-title = Vælg flere assets, der skal tagges, kategoriseres, samles, downloades eller arkiveres sammen (X eller Ctrl-klik)
app-select = Vælg
app-explore-title = Udforsk disse modeller i 3D
app-tile-size = Feltstørrelse
app-supporting-title = glTF-buffere, materialebiblioteker, materialers maps, materialer, som kun én model bærer, og de butiksfiler, der beskriver modeller. De kommer med, når du downloader det, der bruger dem. Et sprite-arks sider og de sprites, pakken også har som separate billeder, vises sammen med arket. Et asset, som pakken har i flere formater, vises én gang, i det format Hoard læser bedst.
app-supporting-hidden =
    { $count } { $count ->
        [one] understøttende fil
       *[other] understøttende filer
    } skjult
app-supporting-show = Vis
app-supporting-shown = Inklusive understøttende filer
app-supporting-hide = Skjul
app-folders = Mapper
app-folders-title = Se pakken, som den lå på disken, én mappe ad gangen
app-folders-label = Pakkens mapper
app-welcome-title = Slip en asset-pakke hvor som helst på denne side
app-welcome-before = Zip-filer, tarballs, mapper eller løse filer virker alle sammen. Hoard pakker dem ud, gemmer hver fil én gang efter dens hash, læser størrelser, længder og antal polygoner og tagger det hele, så du kan søge efter ting som{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }eller{ " " }
app-welcome-after = .
app-welcome-import-folder = Importér en mappe
app-welcome-choose-files = Vælg filer
app-none-similar = Intet andet ligner { $name } særlig meget endnu.
app-none = Intet matcher.
app-none-query = Intet matcher »{ $query }«.
app-none-imitation = Ingen målt lyd lyder endnu meget som »{ $word }«.
app-none-heard = Ingen målt lyd lyder endnu meget som din lyd.
app-none-filters = Intet matcher med disse filtre.
app-none-query-filters = Intet matcher »{ $query }« med disse filtre.
app-clear-search-filters = Ryd søgning og filtre
app-loading-more = Indlæser flere
app-keys-hint = Piletasterne flytter, Enter åbner, mellemrum afspiller en lyd, X eller Ctrl-klik vælger flere (langt tryk på touchskærm).
app-resize-label = Tilpas størrelsen på detaljepanelet
app-resize-title = Træk for at tilpasse · dobbeltklik for den bredde, der passer
app-selected-label = Valgte assets
app-selected = { $count } valgt
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Rediger…
app-edit-selected-title = Ændr tags og felter på dem alle (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] { $count } resultat valgt
       *[other] Alle { $count } resultater valgt
    }
app-pick-assets = Vælg assets
app-download-zip = Download zip
app-credits-title = CREDITS.md for de pakker, disse kommer fra: hvem der lavede dem, deres licenser og de krediteringslinjer, de beder om
app-credits = Krediteringer
app-prefer-sources-title = Unreal-SoundWaves og -teksturer, hvis originale fil Hoard har gendannet
app-prefer-sources = Gendannet WAV/PNG i stedet for .uasset ({ $count })
bag-clear = Ryd markeringen
bag-glb-heading = Modeller som GLB
bag-glb-text = Hver model som en .glb-fil, der står med Y op og vender fremad, med sine teksturer som separate filer i en textures-mappe, som modellerne deler. Alt andet kommer med, som det er.
bag-glb-download = Download GLB-zip
bag-plain-heading = Filer, som de er
bag-plain-text = Hver fil, som den blev importeret, med det, den skal bruge, i en mappe for hver pakke.
bag-remove = Fjern { $name } fra markeringen
bag-more = og { $count } mere
app-bulk-tag-label = Et tag, der skal føjes til eller fjernes fra de valgte assets
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Føj dette tag til alle valgte assets
app-bulk-tag-add = Tilføj
app-bulk-tag-remove-title = Fjern dette tag fra alle valgte assets
app-bulk-tag-remove = Fjern
app-remove-from-collection = Fjern fra { $name }
app-unfavorite = Fjern favorit
app-favorite = Favorit
app-select-all = Vælg alle { $count }
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] Vælg { $count } resultat
       *[other] Vælg alle { $count } resultater
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] Rediger { $count } asset
       *[other] Rediger { $count } assets
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] Rediger { $count } resultat
       *[other] Rediger alle { $count } resultater
    }
bulk-edit-close = Luk
bulk-edit-tags = Tags
bulk-edit-tag-add = Tilføj et tag til dem alle
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } af { $count }
bulk-edit-untick = Fravælg alle
bulk-edit-untick-title = Fjern alle tags, og markér så dem, der skal beholdes
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = De mest brugte tags blandt dem
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Blandet, uændret
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Uændret
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Ryddet på alle
bulk-edit-reset = Sæt tilbage som det var
bulk-edit-all-note = Det ændrer alle resultater, også dem der ikke er indlæst endnu, og kan ikke fortrydes i ét trin.
bulk-edit-save =
    { $count ->
        [one] Gem { $count } ændring
       *[other] Gem { $count } ændringer
    }
bulk-edit-nothing = Intet ændret
bulk-edit-saved =
    { $count ->
        [one] { $count } asset opdateret
       *[other] { $count } assets opdateret
    }
app-archive-selected-title = Ude af søgningen, indtil de gendannes eller slettes for altid fra Arkivet
app-drop-items = Slip { $count } elementer for at tilføje dem
app-drop = Slip for at føje til Hoard
app-drop-hint = Hver fil eller mappe på øverste niveau bliver sin egen pakke. Gigabytes er fint; store filer genoptages, hvis forbindelsen afbrydes.

## Kommandopalet

palette-group-go-to = Gå til
palette-group-actions = Handlinger
palette-group-theme = Tema
palette-group-news = Nyheder
palette-news-show = Vis nyheder
palette-news-mark-all = Markér alt nyt som set
palette-news-keywords = ny nyt nyheder ændringer funktioner udgivelse set new news
palette-group-kinds = Typer
palette-group-packs = Pakker
palette-group-collections = Samlinger
palette-group-projects = Projekter
palette-group-saved = Gemte søgninger
palette-group-assets = Assets
palette-hint-current = aktuelt
palette-library = Bibliotek
palette-library-keywords = assets gitter hjem forside grid home
palette-favorites = Favoritter
palette-favorites-keywords = stjerne stjernemarkeret synes godt om starred stars liked
palette-packs = Pakker
palette-archive = Arkiv
palette-archive-keywords = papirkurv skraldespand slettet gendan trash recycle bin deleted restore
palette-stats = Biblioteksstatistik
palette-stats-keywords = statistik tal størrelse diskplads antal statistics numbers size disk space count
palette-copies = Næsten-kopier
palette-copies-keywords = dubletter samme lignende skaleret konverteret spildt plads duplicates same similar resized converted wasted space dedupe
palette-search = Søg
palette-add-files = Tilføj filer…
palette-add-files-keywords = upload uploade importér import
palette-import-folder = Importér en mappe…
palette-import-folder-keywords = upload uploade pakke pack
palette-select-all = Vælg alle { $count } viste
palette-select-all-keywords = markér afkryds check
palette-select-several = Vælg flere…
palette-select-several-keywords = flere mange tag arkivér masse bulk multiple archive
palette-archive-selected = Arkivér de { $count } valgte
palette-archive-selected-keywords = slet fjern papirkurv delete remove trash
palette-edit-selected = Rediger de { $count } valgte
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = masseændring tags kategori licens vurdering
palette-paste-tags =
    { $count ->
        [one] Indsæt de kopierede tags på det valgte asset
       *[other] Indsæt de kopierede tags på de { $count } valgte
    }
palette-clear-selection = Ryd markeringen
palette-clear-filters = Ryd filtre
palette-clear-filters-keywords = nulstil reset
palette-hide-supporting = Skjul understøttende filer
palette-show-supporting = Vis understøttende filer
palette-supporting-keywords = normal maps ruhed masker normal maps roughness masks
palette-account = Konto og adgangstokens
palette-account-keywords = adgangskode login bruger token api password user
palette-sign-out = Log ud ({ $name })
palette-sign-out-keywords = log ud logout log out
palette-sign-in = Log ind
palette-sign-in-keywords = log ind login konto adgangskode account password
palette-settings = Indstillinger
palette-settings-keywords = blender ffmpeg llm ai tagging unreal admin værktøjer præferencer tools preferences settings
palette-shortcuts = Tastaturgenveje
palette-shortcuts-keywords = hjælp taster genveje help keys
palette-theme-keywords = tema udseende tilstand theme appearance mode
palette-theme-variant-keywords = udseende farve appearance color
palette-light-theme = Lyst tema: { $name }
palette-dark-theme = Mørkt tema: { $name }
palette-kind-all = Alle typer (vis ikke kun { $kind })
palette-kind-only = Kun { $kind }
palette-title = Kommandopalet
palette-placeholder = Søg i assets, gå til en pakke, kør en kommando…
palette-results = Resultater
palette-searching = Søger…
palette-no-match = Intet matcher »{ $query }«
palette-foot-move = flyt
palette-foot-run = kør
palette-foot-shortcuts = alle genveje

## Tastaturgenveje

shortcuts-title = Tastaturgenveje
shortcuts-close = Luk
shortcuts-anywhere = Overalt
shortcuts-palette = Kommandopalet: søg, hop, kør
shortcuts-search = Søg
shortcuts-this-list = Denne liste
shortcuts-escape = Luk, ryd markeringen, eller luk detaljerne
shortcuts-library = Bibliotek
shortcuts-move = Flyt mellem felter (eller J og K)
shortcuts-open = Åbn detaljerne
shortcuts-play-sound = Afspil eller stop en lyd
shortcuts-select = Vælg eller fravælg
shortcuts-favorite = Føj til eller fjern fra dine favoritter
shortcuts-reviewed = Markér som gennemgået, eller ej
shortcuts-tag = Tilføj et tag
shortcuts-edit = Rediger det valgte
shortcuts-copy-tags = Kopiér dets tags
shortcuts-paste-tags = Indsæt kopierede tags på det valgte
shortcuts-key-click = klik
shortcuts-select-range = Vælg et område
shortcuts-select-all = Vælg alt viste
shortcuts-sound-player = Lydafspiller
shortcuts-play-pause = Afspil eller sæt på pause
shortcuts-skip = Spring tilbage eller frem
shortcuts-start-end = Start eller slut
shortcuts-image-inspector = Billedinspektør
shortcuts-zoom = Zoom ind eller ud
shortcuts-fit = Tilpas til vinduet
shortcuts-actual-pixels = Faktiske pixels
shortcuts-tile = Gentag 3×3 for at tjekke sømmene
shortcuts-background = Næste baggrund
shortcuts-channel = Vis én kanal

## KindChips.vue

kinds-group = Assettyper

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = tilføjet af en person
tag-source-rule = fra offline-taggeren
tag-source-meta = målt fra filen
tag-source-ai = foreslået af AI
tag-remove = Fjern tagget { $tag }
tag-accept = Behold tagget { $tag }
tag-accept-title = Behold dette gæt som dit eget tag
tag-reject-title = Fjern det; taggeren sætter det ikke tilbage
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Nyt tag
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Mente du
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = var { $alias }

## ActivityIndicator.vue

activity-label-idle = Baggrundsaktivitet: inaktiv
activity-label-running = Baggrundsaktivitet: { $running } kører
activity-label-queued = Baggrundsaktivitet: { $running } kører, { $queued } i kø
activity-title = Baggrundsaktivitet
activity-unreachable = Kan ikke nå serveren
activity-unreachable-desktop = Hoard svarer ikke
activity-empty = Intet kører. Importer, tagging, konverteringer og zip-downloads vises her, mens de arbejder.
activity-queued = i kø
activity-stop = Stop »{ $what }«
activity-stop-tagging = Stop al AI-tagging ({ $count })
activity-seconds = { $s } s
activity-minutes = { $min } min.
activity-hours = { $h } t { $min } min.
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } trekanter
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Afspil { $name }
tile-pause = Sæt { $name } på pause
tile-favorite = Gør { $name } til favorit
tile-in-favorites = I dine favoritter
tile-add-favorite = Føj til dine favoritter (F)
tile-more-like = Mere som { $name }
tile-more-like-this = Mere som dette
tile-select = Vælg { $name }

## CollectionPicker.vue

collections-add = Føj til samling
collections-menu = Samlinger
collections-find-label = Find eller opret en samling
collections-find-placeholder = Find eller navngiv en ny samling
collections-create = Opret
collections-empty = Ingen samlinger endnu. Skriv et navn for at starte en.

## FolderPicker.vue

folders-list = Mapper
folders-close = Luk
folders-browse = Gennemse…
folders-up-title = Én mappe op
folders-up = Op
folders-this-computer = Denne computer
folders-none = Ingen mapper herinde.

## LocationForm.vue

locations-mode-question = Hvad skal Hoard gøre med denne mappe?
locations-mode-managed = Gør krav på den: Hoard har mine pakker her
locations-mode-managed-hint = Pakker, du uploader eller importerer, kopieres herind, og Hoard holder orden på dem. Mappen er Hoards nu, så lad dens indhold være.
locations-mode-watched = Hold øje med den: mine pakker er her allerede
locations-mode-watched-hint = Hver mappe i den bliver en pakke. Hoard læser filerne, hvor de ligger, og ændrer, flytter eller sletter dem aldrig, og opfanger nye mapper, efterhånden som de dukker op.
locations-folder-desktop = Mappe
locations-folder-placeholder = Den mappe, dine pakker ligger i
locations-folder-hint-managed = Hoard opretter den, hvis den ikke findes. I Docker er det en mappe, du har monteret i containeren.
locations-folder-hint-managed-desktop = Hoard opretter den, hvis den ikke findes.
locations-folder-hint-watched = I Docker skal du først montere mappen i containeren (skrivebeskyttet er fint) og så vælge den her.
locations-folder-hint-watched-desktop = Hoard læser den kun: intet i den bliver ændret.
locations-name = Navn
locations-name-placeholder-managed = Bibliotek
locations-name-placeholder-watched = Mine pakker
locations-files = Filer, der skal hertil
locations-catch-all = Også alt, som ingen anden placering tager
locations-kinds-hint = En ny pakkes filer går til den placering, der har deres type afkrydset. Resten går til opsamlingsstedet.
locations-files-all = Dem alle sammen, indtil videre. Tilføj flere placeringer senere under Indstillinger for at sende nogle typer (f.eks. musik) til en anden disk.
locations-first-watched-note = Upload kræver et bibliotek, som Hoard passer. Du kan tilføje et senere under Indstillinger → Biblioteksplaceringer.
locations-checking = Kontrollerer mappen…
locations-add = Tilføj placering
locations-cancel = Annullér

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Kunne ikke indlæse biblioteksplaceringerne
locations-takes-and-rest = { $kinds } og alt andet
locations-takes-rest = Alt, som de andre ikke tager
locations-takes-everything = Alt
locations-takes-nothing = Intet endnu
locations-save-failed = Kunne ikke gemme placeringen
locations-unwatch-title = Stop overvågning af { $name }?
locations-unwatch-body =
    Dens { $count ->
        [one] { $count } pakke
       *[other] { $count } pakker
    } forsvinder fra Hoard sammen med deres tags og noter. Filerne i mappen røres ikke.
locations-unwatch-body-empty = Filerne i mappen røres ikke.
locations-unwatch = Stop overvågning
locations-remove-title = Fjern { $name }?
locations-remove-body = Eventuelle filer i den flyttes først til de andre placeringer efter type; den forsvinder, når den er tom. Intet går tabt.
locations-remove-body-only = Hvis den indeholder filer, så tilføj først en anden placering, de kan flyttes til.
locations-remove = Fjern
locations-removed = Fjernede { $name }
locations-emptying = Flytter filerne ud af { $name }; den fjernes, når det er færdigt
locations-offline-title = { $name } kan ikke nås
locations-offline-body = Dens filer kan ikke flyttes ud, mens den er offline. Vil du fjerne den alligevel? Filer, der stadig ligger på den, vises som manglende.
locations-remove-anyway = Fjern alligevel
locations-remove-failed = Kunne ikke fjerne { $name }
locations-rebalancing = Flytter gemte filer til de placeringer, der tager deres type
locations-rebalance-failed = Kunne ikke flytte filerne
locations-scanned-never = ikke scannet endnu
locations-scanned-now = scannet lige nu
locations-scanned-minutes = scannet for { $mins } min. siden
locations-scanned-hours = scannet for { $hours } t siden
locations-scanned-date = scannet { $date }
locations-scan-failed = Kunne ikke scanne { $name }
locations-watching = Overvåger { $name }
locations-added = Tilføjede { $name }
locations-loading = Indlæser…
locations-managed-title = Biblioteker, Hoard passer
locations-watched-title = Overvågede mapper
locations-managed-hint = Uploads og importer kopieres herind. Hver placering tager de typer, den har afkrydset; opsamlingsstedet tager resten.
locations-watched-hint = Mapper med pakker, der indekseres, hvor de ligger. Hoard ændrer aldrig filerne i dem og kigger efter nye og fjernede mapper hvert 15. minut.
locations-managed-empty = Ingen endnu, så der er ikke noget sted at lægge en upload. Tilføj en nedenfor.
locations-reachable = Kan nås
locations-unreachable-now = Kan ikke nås lige nu
locations-catch-all-tag = opsamling
locations-packs =
    { $count ->
        [one] { $count } pakke
       *[other] { $count } pakker
    }
locations-unreachable = kan ikke nås
locations-scanning = Scanner…
locations-scan = Scan nu
locations-edit = Redigér
locations-catch-all-locked = Gør en anden placering til opsamlingsstedet for at ændre dette
locations-edit-hint-before = Gælder for filer, der tilføjes fra nu af. For at flytte det, der allerede er gemt, skal du bruge
locations-edit-hint-after = nedenfor.
locations-save = Gem
locations-rebalance = Flyt gemte filer, så de passer
locations-rebalance-hint = Efter du har ændret, hvad en placering tager: filer, der allerede er gemt, flyttes til den placering, deres type nu hører til.
locations-add-title = Tilføj en placering
locations-add-hint = En anden disk til nogle typer filer, eller en mappe med pakker, du allerede har.
locations-kind-question = Hvilken slags placering er det?
locations-kind-folder = En mappe
locations-kind-folder-hint = På en disk eller et NAS-share, der er monteret i containeren, eller en, som Dropbox, Google Drive, OneDrive eller iCloud Drive holder synkroniseret. Hoard kan gemme filer der eller holde øje med pakker, du allerede har.
locations-kind-folder-hint-desktop = På denne computer, en anden disk eller et NAS-share, eller en, som Dropbox, Google Drive, OneDrive eller iCloud Drive holder synkroniseret. Hoard kan gemme filer der eller holde øje med pakker, du allerede har.
locations-kind-s3-hint = En bucket i Amazon S3 eller en lignende tjeneste: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard gemmer filer der.
locations-s3-title = S3-lager
locations-s3-hint = En placering på s3://bucket-name er en bucket i denne tjeneste: AWS eller en hvilken som helst S3-kompatibel (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage på en NAS).
locations-bucket = Bucket
locations-bucket-hint = Navnet på en bucket, der allerede findes. Nøglerne ovenfor skal kunne liste, læse og skrive i den.
locations-s3-endpoint = Adresse
locations-s3-endpoint-placeholder = Tom for AWS; f.eks. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Region
locations-s3-access-key = Adgangsnøgle-id
locations-s3-secret-key = Hemmelig adgangsnøgle
locations-s3-headers = Ekstra headere
locations-s3-headers-placeholder = Name=value, én pr. linje (f.eks. til Cloudflare Access)
locations-s3-kept = Gemt (skriv for at erstatte)
locations-s3-secrets = Nøglerne og headerne gemmes sammen med serverens indstillinger og vises aldrig igen.
locations-s3-secrets-desktop = Nøglerne og headerne gemmes i denne computers nøglering og vises aldrig igen.
locations-s3-saved = S3-lager gemt
locations-s3-save-failed = Kunne ikke gemme S3-lager
locations-add-more = Tilføj en placering…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Kunne ikke indlæse Problemer
issues-started =
    { $count ->
        [one] Startet igen.
       *[other] Startede { $count } ting igen.
    }
issues-retry-failed = Kunne ikke prøve igen
issues-dismiss-failed = Kunne ikke afvise
issues-dismiss-all-title = Afvis alle { $count }?
issues-dismiss-all-body = »{ $title }« ryddes. Alt, der går galt igen, bliver vist igen.
issues-dismiss-all = Afvis alle
issues-just-now = lige nu
issues-minutes-ago = for { $mins } min. siden
issues-hours-ago = for { $hours } t siden
issues-loading = Indlæser…
issues-none = Intet er gået galt.
issues-action-all = { $action } alle
issues-open-asset = Åbn det
issues-open-pack = Vis pakken
issues-open-locations = Biblioteksplaceringer
issues-where-before = i
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } gang
       *[other] { $count } gange
    }
issues-starting = Starter…
issues-dismiss-named = Afvis { $title }
issues-dismiss = Afvis
issues-show-fewer = Vis færre
issues-show-all = Vis alle { $count }
issues-not-listed = { $count } mere er ikke vist.

## archive.ts

archive-anyway = Arkivér alligevel
archive-failed = Kunne ikke arkivere: { $error }
archive-undo = Fortryd
archive-restore-failed = Kunne ikke gendanne: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    } og { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-nothing = intet

## ArchiveView.vue

archive-load-failed = Kunne ikke indlæse arkivet: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Gendannede { $what }
archive-purged = Slettede { $what } for altid
archive-purge-failed = Kunne ikke slette: { $error }
archive-purge-title = Slet { $name } for altid?
archive-purge-pack-body =
    Dens { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } og { $size } filer slettes også, undtagen filer, som en anden pakke deler. Dette kan ikke fortrydes.
archive-purge = Slet for altid
archive-purge-asset-body = Dets fil frigives, medmindre et andet asset deler den. Dette kan ikke fortrydes.
archive-empty-title = Tøm arkivet?
archive-empty-body = { $what } slettes for altid, og deres filer frigives. Dette kan ikke fortrydes.
archive-empty = Tøm arkivet
archive-title = Arkiv
archive-intro = Arkiverede pakker og assets er ude af søgningen, optællingerne og listen over tags. Gendan dem, eller slet dem for altid for at frigøre deres plads.
archive-loading = Indlæser…
archive-is-empty = Arkivet er tomt
archive-is-empty-hint = Arkivér en pakke eller et asset, så venter det her, indtil du gendanner det eller sletter det for altid.
archive-packs = Pakker
archive-assets = Assets
archive-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
archive-archived = arkiveret { $when }
archive-from = fra { $pack }
archive-restore = Gendan
archive-show-more = Vis flere ({ $count } tilbage)

## mine.ts

projects-favorite-add-failed = Kunne ikke føje til dine favoritter: { $error }
projects-favorite-remove-failed = Kunne ikke fjerne fra dine favoritter: { $error }
projects-save-search-failed = Kunne ikke gemme søgningen: { $error }
projects-forget-search-failed = Kunne ikke fjerne { $name }: { $error }
projects-change-search-failed = Kunne ikke ændre { $name }: { $error }

## projects.ts

projects-list-and = { $items } og { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } mere
projects-other =
    { $count ->
        [one] { $count } andet projekt
       *[other] { $count } andre projekter
    }
projects-usage-title-one = { $name } bruger dette
projects-usage-title = { $count } projekter bruger dette
projects-usage-body-one = Bruges i { $list }: næste hoard sync fjerner dem.
projects-usage-body = Bruges i { $list }: hvert projekts næste hoard sync fjerner dem.

## uploader.ts

upload-network-error = netværksfejl
upload-no-reply = intet svar i { $seconds } s
upload-retrying = Prøver { $what } igen ({ $attempt } af { $attempts }): { $error }
upload-gave-up = { $what }: { $error } efter { $attempts } forsøg
upload-failed-plain = mislykkedes
upload-already-imported = allerede importeret
upload-part-failed = upload mislykkedes ({ $status })
upload-batch = parti med { $count } filer
upload-batch-failed = partiet mislykkedes ({ $status })
upload-finish = afslut upload

## UploadQueue.vue

upload-eta-seconds = { $s } s tilbage
upload-eta-minutes = { $min } min. tilbage
upload-eta-hours = { $h } t tilbage
upload-queue = Importer
upload-importing = Importerer { $count } pakker
upload-bytes-of = { $sent } af { $total }
upload-rate = { $size }/s
upload-waiting = Venter
upload-unpacking = Uploadet, pakker nu ud
upload-already = Allerede i biblioteket
upload-stopped = Stoppet
upload-files-of = { $done } af { $total } filer
upload-resumed = genoptaget, { $size } var der allerede
upload-skipped = Præcis disse filer var allerede importeret som »{ $name }«, så de blev sprunget over.
upload-error = { $error }. Dine fremskridt er gemt: prøv igen, eller slip de samme filer igen senere.
upload-retry = Prøv igen
# $name is the pack the library has already that these files go into.
upload-update = Opdatér »{ $name }«
upload-copy = Importér som kopi
# $name is the pack the library has already that the upload goes into.
upload-into = Går ind i »{ $name }«, som allerede er i biblioteket: nye filer tilføjes, og ændrede erstattes.
upload-own-pack = Gør det til en selvstændig pakke
upload-dismiss = Afvis
upload-cancel = Annullér
upload-minimize = Minimer
upload-expand = Vis importer
upload-pill = Importerer { $count } · { $percent } %
upload-pill-stopped = Stoppet: { $count }
upload-interrupted = Afbrudt
upload-resume = Genoptag
upload-resume-all = Genoptag alle
upload-resume-hint-folder = Vælg den samme mappe igen, eller slip den på siden, for at fortsætte, hvor den stoppede.
upload-resume-hint-file = Vælg den samme fil igen, eller slip den på siden, for at fortsætte, hvor den stoppede.
upload-not-same = Det er ikke de samme filer, så intet blev genoptaget.
upload-permission-denied = Browseren fik ikke lov til at læse filerne igen. Vælg dem i stedet.

## PacksView.vue

packs-rename-failed = Kunne ikke omdøbe: { $error }
packs-collection-deleted = Slettede samlingen { $name }
packs-delete-failed = Kunne ikke slette: { $error }
packs-status-uploading = Uploader (de filer, der er modtaget indtil nu)
packs-status-ingesting = Pakker ud og tagger
packs-status-analyzing = LLM tagger
packs-status-ready = Klar
packs-status-failed = Mislykkedes
packs-retag-failed = Kunne ikke starte ny tagging: { $error }
packs-read-files-failed = Kunne ikke læse pakkens filer: { $error }
packs-context-load-failed = Kunne ikke indlæse pakkens butiksoplysninger: { $error }
packs-undo-store-failed = Kunne ikke fjerne butikssiden: { $error }
packs-context-save-failed = Kunne ikke gemme butiksoplysningerne: { $error }
packs-unreal-load-failed = Kunne ikke indlæse status for Unreal-konverteringen: { $error }
packs-unreal-discard-title = Kassér originalerne?
packs-unreal-discard-body = De originale .uasset-filer i { $name } slettes, når de er konverteret. Data i dem, som kun Unreal bruger, f.eks. materialegrafer, går tabt for altid.
packs-unreal-discard-confirm = Konvertér og kassér
packs-unreal-convert-failed = Kunne ikke starte konverteringen: { $error }
packs-unreal-fact-converted = { $done } konverteret fra Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal-pakke
       *[other] { $left } Unreal-pakker
    }
packs-unreal-fact-both = { $done } konverteret fra Unreal, { $left } stadig Unreal-pakker
packs-thumbs-canceled = Annulleret efter { $done } af { $total }
packs-thumbs-drew =
    { $count ->
        [one] Tegnede { $count } model
       *[other] Tegnede { $count } modeller
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniature erstattet.
       *[other] { $lead }: { $improved } miniaturer erstattet.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniature erstattet, { $failed } kunne ikke tegnes.
       *[other] { $lead }: { $improved } miniaturer erstattet, { $failed } kunne ikke tegnes.
    }
packs-archived = Arkiverede { $name } og dens { $count } assets
packs-blobs-bad = { $count } filer bestod ikke deres seneste integritetskontrol.
packs-collections = Samlinger
packs-browse-named = Gennemse { $name }
packs-collection-name = Samlingens navn
packs-save = Gem
packs-cancel = Annullér
packs-assets =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
packs-updated = Opdateret { $date }
packs-browse = Gennemse
packs-collection-download = Download (.zip)
packs-collection-credits = Krediteringer
packs-collection-credits-title = Hvem der lavede de pakker, disse assets kommer fra, deres licenser og de krediteringslinjer, de beder om
packs-rename = Omdøb
packs-delete = Slet
packs-packs = Pakker
packs-empty = Ingen pakker endnu. Slip en zip-fil eller en mappe hvor som helst på denne side for at tilføje en.
packs-find = Find en pakke
packs-find-placeholder = Find en pakke efter navn, butik, licens eller tag
packs-sort = Rækkefølge
packs-sort-new = Nyeste først
packs-sort-old = Ældste først
packs-sort-size = Største først
packs-sort-assets = Flest assets først
packs-sort-count = Antal assets
packs-sort-smallest = Mindste først
packs-sort-fewest = Færrest assets først
packs-with-kind = Pakker med
packs-state = Status
packs-state-any = Enhver status
packs-state-working = I gang
packs-state-failed = Mislykkedes
packs-state-untagged = Ikke beskrevet endnu
packs-state-unlicensed = Ingen licens angivet
packs-store = Butik
packs-store-any = Enhver butik
packs-store-from = Fra { $store }
packs-store-none = Ingen butiksside
packs-shown-of = { $shown } af { $total }
packs-clear = Ryd
packs-no-match = Ingen pakker matcher.
packs-clear-all = Ryd søgningen og filtrene
packs-pack-name = Pakkens navn
packs-thumbs-drawing = Tegner miniaturer i denne browser…
packs-thumbs-drawing-count = Tegner miniaturer i denne browser: { $done } af { $total }…
packs-thumbs-stopped = Stoppet: { $error }
packs-thumbs-none = Alle modeller her har allerede en miniature tegnet med alle deres teksturer.
packs-dismiss = Afvis
packs-license = Licens: { $license }
packs-license-tier = Licens: { $license } ({ $tier })
packs-license-not-found = ikke fundet
packs-from-before = Fra
packs-from-after = { "" }
packs-added = Tilføjet { $date }
pack-panel-label = Om denne pakke
pack-panel-hide = Skjul pakkens detaljer
pack-cover-previous = Forrige billede
pack-cover-next = Næste billede
pack-cover-picture = Billede { $n } af { $count }
pack-panel-kinds = Hvad den indeholder
pack-panel-all = Alle pakker
packs-download = Download pakke (.zip)
packs-store-details = Butiksoplysninger
packs-more-named = Flere handlinger for { $name }
packs-more = Mere
packs-rename-more = Omdøb…
packs-retag-rules = Kør offline-taggeren igen
packs-retag-llm = Kør LLM-taggeren igen
packs-make-thumbs = Lav miniaturer i denne browser
packs-make-thumbs-title = Åbn hver model i en skjult 3D-fremviser, én ad gangen, og brug dens billede som miniature, hvor det er bedre end serverens
packs-make-thumbs-title-desktop = Åbn hver model i en skjult 3D-fremviser, én ad gangen, og brug dens billede som miniature, hvor det er bedre end det, Hoard lavede
packs-read-files = Læs pakkens filer igen
packs-credits = Download krediteringer
packs-credits-title = CREDITS.md: hvem der lavede pakken, dens licens og den krediteringslinje, den beder om
packs-unreal-open = Konvertér Unreal-indhold…
packs-archive = Arkivér pakke
packs-unreal-intro = Hoard læser selv Unreal-pakker og gemmer indholdet som almindelige filer: teksturer som PNG eller EXR, lyde som WAV, materialer som JSON. Den konverterede fil tager Unreal-pakkens plads og beholder dens tags, links og samlinger.
packs-unreal-converted =
    { $count ->
        [one] konverteret
       *[other] konverterede
    }
packs-unreal-converted-kept =
    { $count ->
        [one] konverteret, { $kept } med originalen beholdt
       *[other] konverterede, { $kept } med originalen beholdt
    }
packs-unreal-pending =
    { $count ->
       *[other] ikke kigget på endnu
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] venter på Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] uden et almindeligt format
    }
packs-unreal-failed =
    { $count ->
       *[other] mislykkedes
    }
packs-unreal-reasons = Hvorfor Unreal-pakker er tilbage
packs-unreal-editor-before = Meshes og animationer kræver Unreal Editor. På en maskine med Unreal installeret skal du køre
packs-unreal-editor-command = hoard unreal-worker --editor <sti til UnrealEditor-Cmd>
packs-unreal-editor-after = ; den eksporterer dem som FBX og uploader resultaterne hertil.
packs-unreal-discard = Kassér de originale .uasset-filer, når de er konverteret
packs-unreal-discard-hint = Frigør pladsen. Data, som kun Unreal bruger, f.eks. materialegrafer, går tabt.
packs-unreal-drop = Slet Unreal-pakker uden et almindeligt format
packs-unreal-drop-hint = Blueprints, levels og andre assets, som kun motoren bruger.
packs-unreal-convert = Konvertér
packs-context-hint-before = Hvor pakken kom fra, og hvad sælgeren siger om den. LLM-taggeren læser dette sammen med hver fil, så en butikstekst som »120 ice and fire spells« hjælper den med at navngive
packs-context-hint-after = .
packs-store-auto = Fundet automatisk: pakken nævner selv denne side.
packs-store-not-this = Ikke denne
packs-lookup-label = Søg i butikkerne efter
packs-lookup-placeholder = Pakkens navn i butikken
packs-lookup-searching = Søger…
packs-lookup-search = Søg i butikker
packs-lookup-hide = Skjul
packs-lookup-clues = Fra pakkens filer: { $clues }.
packs-lookup-also = Søgte også efter { $queries }.
packs-lookup-quoted = »{ $query }«
packs-lookup-nothing = Intet på Fab, itch.io, Kenney eller OpenGameArt ligner den. Prøv færre eller andre ord.
packs-lookup-use = Brug
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Nu
packs-review-store = Fra butikken
packs-review-same = Ens
packs-review-read-page = Læses fra siden, når du gemmer
packs-review-take =
    { $count ->
        [one] Udfyld { $count } felt
       *[other] Udfyld { $count } felter
    }
packs-lookup-searched = Søgt { $date }.
packs-lookup-again = Søg igen
packs-lookup-again-after = .
packs-lookup-unreachable = Kunne ikke nå { $stores } denne gang.
packs-lookup-also-look = Kig også på
packs-lookup-links-and = { " " }og{ " " }
packs-lookup-browser-only = (de kan kun søges i din browser); indsæt sidens adresse nedenfor.
packs-lookup-open = Find i butikker…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Sælger
packs-seller-placeholder = Hvem der lavede den
packs-license-label = Licens
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Licensniveau
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] { $count } asset har sin egen licens: lad den også følge pakkens
       *[other] { $count } assets har deres egen licens: lad dem også følge pakkens
    }
packs-store-page = Butiksside
packs-store-text = Beskrivelse fra butikken
packs-store-text-placeholder = Indsæt teksten fra butikken, eller lad feltet være tomt for at hente den fra butikssiden.
packs-store-text-fetch = Når du gemmer, hentes beskrivelsen fra butikssiden.
packs-readme = README fundet i pakken
packs-save-retag = Gem, og kør LLM-taggeren igen

## AssetDetail.vue

asset-lazy-3d-viewer = 3D-fremviseren
asset-lazy-text = teksten
asset-lazy-text-viewer = tekstfremviseren
asset-lazy-image-viewer = billedfremviseren
asset-lazy-material-preview = materialeforhåndsvisningen
asset-lazy-material-viewer = materialefremviseren
asset-lazy-sprite-player = sprite-afspilleren
asset-lazy-sheet-view = fremviseren til sprite-ark
asset-rel-texture = tekstur
asset-rel-buffer = geometridata
asset-rel-material = materialefil
asset-rel-animation = animation
asset-rel-library = linket fil
asset-rel-guess = hører nok sammen med
asset-rel-sidecar = butiksoplysninger
asset-rel-variant = variant
asset-rel-sprite = sprite
asset-rel-sample = sample
asset-role-map = map for { $role }
asset-maps-count =
    { $count ->
        [one] { $count } map
       *[other] { $count } maps
    }
asset-needs = Kræver
asset-animations = Animationer
asset-sprites = Sprites
asset-used-by = Bruges af
asset-loose-sprites = Også som separate filer
asset-in-sheet = I et spriteark
asset-other-formats = Andre formater
asset-shown-as = Vist i biblioteket som
asset-conversion-failed = konverteringen mislykkedes
asset-as-converting = Konverterer…
asset-as-converting-percent = Konverterer… { $percent } %
asset-as-queued = I kø…
asset-download = Download
asset-as-label = Download som
asset-as-original = { $format } (originalt format)
asset-as-pack-files = Pakkens egne filer
asset-as-size = Størrelse
asset-as-full-size = Fuld størrelse
asset-as-up-to = Op til { $size } px
asset-as-hint = Vælg en størrelse eller et andet format
asset-meta-size = Størrelse
asset-meta-length = Længde
asset-meta-sample-rate = Samplerate
asset-meta-channels = Kanaler
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bitdybde
asset-meta-bits = { $bits }-bit
asset-meta-level = Niveau
asset-meta-level-value = top { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = top { $peak } dBFS, RMS { $rms } dBFS, klipper
asset-meta-loop = Loop
asset-meta-loop-value = { $start } til { $end }
asset-meta-playback = Afspilning
asset-meta-one-shot = One-shot
asset-meta-envelope = Forløb
asset-meta-envelope-times = anslag { $attack }, efterklang { $tail }
asset-meta-envelope-hit = slag
asset-meta-envelope-swell = opbygning
asset-meta-envelope-sustained = vedvarende
asset-meta-tone = Klang
asset-meta-tone-value = lysstyrke { $khz } kHz, støj { $noise } %
asset-meta-pitch = Tonehøjde
asset-meta-pitch-rises =
    { $octaves ->
        [one] stiger { $octaves } oktav
       *[other] stiger { $octaves } oktaver
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] falder { $octaves } oktav
       *[other] falder { $octaves } oktaver
    }
asset-meta-loops-cleanly = Looper sømløst
asset-meta-tempo = Tempo
asset-meta-tempo-heard = cirka { $bpm } BPM, efter gehør
asset-meta-tempo-heard-or = cirka { $bpm } BPM (eller { $alt }), efter gehør
asset-meta-key = Toneart
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, efter gehør
music-key-major = { $note }-dur
music-key-minor = { $note }-mol
asset-meta-root-note = Grundtone
asset-meta-cue-markers = Cue-markører
asset-meta-library = Bibliotek
asset-meta-keywords = Nøgleord
asset-meta-title = Titel
asset-meta-artist = Kunstner
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Generator
asset-meta-everything-with = Vis alt med tagget { $tag }
asset-meta-saved-with = Gemt med
asset-meta-color-mode = Farvetilstand
asset-meta-layers = Lag
asset-meta-range = Område
asset-meta-hdr = HDR (forhåndsvisningen er tone-mappet)
asset-meta-format = Format
asset-meta-large-document = Stort dokument (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Lavet med generativ AI
asset-meta-ai-edited = Redigeret med generativ AI
asset-meta-ai-how-metadata = metadata
asset-meta-ai-how-generator = generatorindstillinger
asset-meta-ai-how-iptc = IPTC-type for digital kilde
asset-meta-ai-says = { $made }, ifølge filens { $how }
asset-meta-ai-by-says = { $made } ({ $by }), ifølge filens { $how }
asset-meta-provenance = Oprindelse
asset-meta-c2pa = Har C2PA Content Credentials
asset-meta-compression = Komprimering
asset-meta-note = Note
asset-meta-maps = Maps
asset-meta-sizes =
    { $count ->
        [one] Størrelse
       *[other] Størrelser
    }
asset-meta-triangles = Trekanter
asset-meta-vertices = Hjørnepunkter
asset-meta-meshes = Meshes
asset-meta-materials = Materialer
asset-meta-animations = Animationer
asset-meta-skins = Skins
asset-meta-unreal-asset = Unreal-asset
asset-meta-unreal-saved-with = { $class }, gemt med UE { $engine }
asset-meta-package = Unreal-pakke
asset-meta-cooked = Cooked (spil-build): forhåndsvisninger og kilder er fjernet
asset-meta-tiling = Gentagelse
asset-meta-seamless = Sømløs på begge akser
asset-meta-analysis-error = Kunne ikke læse hele denne fil: { $error }
asset-copy-blocked = Kunne ikke kopiere: browseren blokerede det
asset-save-error = Kunne ikke gemme: { $error }
asset-tag-removed = Fjernede { $tag }
asset-tag-restored = Satte { $tag } tilbage
asset-tags-added =
    Tilføjede { $count ->
        [one] { $count } tag
       *[other] { $count } tags
    }
asset-tags-accepted =
    Beholdt { $count ->
        [one] { $count } gættet tag
       *[other] { $count } gættede tags
    }
asset-reviewed = Gennemgået
asset-reviewed-on-title = Gennemgået: regler og AI lader dets tags være. Klik for at fjerne mærket (R)
asset-reviewed-off-title = Markér som gennemgået, så regler og AI lader dets tags være (R)
asset-reviewed-on = Markeret som gennemgået
asset-reviewed-off = Fjernede gennemgået-mærket
# $name is an asset's name.
asset-reviewed-on-named = Markerede { $name } som gennemgået
asset-reviewed-off-named = Fjernede gennemgået-mærket fra { $name }
asset-undo = Fortryd
asset-play-error = Kunne ikke afspille denne fil: { $error }
asset-copied-hash = Kopierede hash
asset-copied-toml = Kopierede toml
asset-copied-id = Kopierede id
asset-copied-link = Kopierede link
asset-origin-before = Denne fil var
asset-origin-class-engine = , en { $class } gemt af UE { $engine }.
asset-origin-class = , en { $class }.
asset-origin-engine = { " " }gemt af UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor eksporterede den den { $date }.
asset-origin-editor-notes = Unreal Editor eksporterede den den { $date } ({ $notes }).
asset-origin-hoard = Hoard læste den ud af Unreal-pakken den { $date }.
asset-origin-hoard-notes = Hoard læste den ud af Unreal-pakken den { $date } ({ $notes }).
asset-origin-download = Download den originale .{ $ext }
asset-origin-discarded = Den originale Unreal-pakke blev kasseret efter konverteringen.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Historik
history-loading = Indlæser…
history-none = Der er ikke ændret noget her endnu.
history-load-failed = Kunne ikke indlæse historikken: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = AI
history-by-rule = En regel
history-by-someone = Nogen
history-by-app = { $who } via en app
history-by-assistant = { $who } via en assistent
history-field-description = Beskrivelse
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Tagget { $tag }
history-tag-removed = Fjernede { $tag }
history-tag-kept = Beholdt det foreslåede tag { $tag }
history-tag-changed = Ændrede tagget { $tag }
history-reviewed = Markeret som gennemgået
history-unreviewed = Markeret som ikke gennemgået
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: sat til { $to }
history-cleared = { $field }: ryddet (var { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Fortryd
history-revert-title = Sæt det tilbage, som denne ændring ændrede
history-reverted = Fortrudt: { $change }
history-revert-failed = Kunne ikke fortryde det: { $error }
asset-details = Asset-detaljer
asset-in-pack = i
asset-previous = Forrige asset
asset-next = Næste asset
asset-favorite = Favorit
asset-favorite-on-title = I dine favoritter (F fjerner det)
asset-favorite-off-title = Føj til dine favoritter (F)
asset-share = Del
asset-share-title = Del et link til dette asset (den, der åbner det, logger først ind i Hoard)
asset-layout-panel = Vis i et sidepanel
asset-layout-page = Vis som hel side
asset-layout-page-title = Vis som hel side: en stor forhåndsvisning ved siden af detaljerne
asset-close = Luk detaljer
asset-rename-title = Klik for at omdøbe
asset-name = Navn
asset-views-alt = Modellen fra fire retninger
asset-inspect-aria = Undersøg { $name } i fuld skærm
asset-inspect-title = Undersøg: zoom, gentagelse, kanaler
asset-playback-position = Afspilningsposition
asset-playback-value = { $current } af { $duration }
asset-pause = Pause
asset-play = Afspil
asset-waveform-drawing = Tegner bølgeform…
asset-waveform-skipped = Ingen bølgeform for så store filer
asset-play-animation = Afspil som animation
asset-show-views = Vis arket med de fire visninger, som taggeren så
asset-palette = Farver i billedet
asset-swatch-title = { $hex }, { $percent } % af billedet. Find assets med denne farve
asset-swatch-aria = Find assets med { $hex }, { $percent } % af dette billede
asset-more-like-this = Mere som dette
asset-desc-label = Din beskrivelse
asset-desc-placeholder = Hvad det er, hvor du ville bruge det…
asset-save = Gem
asset-cancel = Annullér
asset-desc-family = Gemmes på alle { $count } varianter
asset-desc-edit = Redigér beskrivelse
asset-desc-write = Skriv en beskrivelse
asset-desc-ai = Skrevet af AI
asset-desc-file = Fra filen
asset-desc-store-before = Fra
asset-desc-store-page = { $store }-siden
asset-desc-store-page-unnamed = butikssiden
asset-desc-store-after = (hele pakken)
asset-less = Mindre
asset-more = Mere
asset-tags = Tags
asset-tag-add-label = Tilføj et tag
asset-tag-add-placeholder = Tilføj et tag, f.eks. mood:tense
asset-apply-family = Anvend ændringer på alle { $count } varianter
asset-tags-accept-all =
    { $count ->
        [one] Behold { $count } gæt
       *[other] Behold alle { $count } gæt
    }
asset-tags-accept-all-title = Behold alle tags, som reglerne og AI har gættet, som dine egne
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Afvist:
asset-tag-restore-title = Sæt { $tag } tilbage
asset-tags-also-under = Hører også under:
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = Lignende har:
asset-tag-suggest-title = Tilføj { $tag }
asset-details-heading = Detaljer
asset-field-category = Kategori
asset-field-license = Licens
asset-field-source = Kilde
asset-field-rating = Vurdering
# Shown in place of an empty value, to fill it in.
asset-field-add = Tilføj
asset-field-new = Tilføj et felt
asset-field-name = Feltnavn
asset-field-value = Værdi
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = fra pakken
asset-field-license-placeholder = Som CC0 eller CC-BY 4.0
asset-field-remove-placeholder = Tomt for at fjerne det
# $field is a field's name, like "License".
asset-field-edit-title = Redigér { $field }
asset-field-saved = Gemte { $field }
asset-field-cleared = Ryddede { $field }
asset-rate =
    { $count ->
        [one] { $count } stjerne
       *[other] { $count } stjerner
    }
asset-rated =
    Vurderede til { $count ->
        [one] { $count } stjerne
       *[other] { $count } stjerner
    }
asset-rating-clear-title = Klik igen for at rydde vurderingen
asset-rating-cleared = Ryddede vurderingen
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Klik igen for { $count } stjerne
       *[other] Klik igen for { $count } stjerner
    }
asset-collections = Samlinger
asset-collection-remove = Fjern fra { $name }
asset-collection-add-another = Føj til en anden
asset-collection-add = Føj til samling
asset-used-in = Bruges i
asset-used-in-title = Vis, hvad { $name } har
asset-used-in-synced = synkroniseret { $when }
asset-maps = Maps
asset-show-more = Vis { $count } mere
asset-materials-heading =
    { $count ->
        [one] Materiale
       *[other] Materialer
    }
asset-material-maps-aria = { $name }: maps
asset-variants = Varianter ({ $count } versioner)
asset-fit-animations = Animationer der passer ({ $count })
asset-fit-characters = Figurer den passer til ({ $count })
asset-copies = Identiske kopier
asset-copies-note = De samme bytes gemmes én gang; disse pakker indeholder også denne fil.
asset-near-copies = Næsten-kopier
asset-near-copies-note = Det samme i en anden størrelse, samplingfrekvens eller et andet format. Den bedste kopi er markeret.
asset-near-copies-best = Det samme i en anden størrelse, samplingfrekvens eller et andet format. Denne er den bedste kopi.
asset-near-best = bedst
asset-unreal-converted = Konverteret fra Unreal
asset-rule-converted = Konverteret af en regel
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = En konverteringsregel lavede denne { $format } den { $date } ud fra
asset-rule-kept-until = Originalen gemmes indtil { $date }.
asset-rule-rebuilt = FLAC-filen gemmer alt, hvad der var i WAV-filen, så WAV-filen til enhver tid kan hentes eller sættes tilbage præcis, som den var.
asset-rule-put-back = Gendan originalen
asset-rule-put-back-title = Gendan { $file }?
asset-rule-put-back-body = Den konverterede fil slettes, og originalen tager dens plads med sine tags, samlinger og links. Konverteringsreglerne lader den være fremover.
asset-rule-put-back-done = { $file } er tilbage
asset-rule-let-go = Originalen blev slettet, da den tid, originaler gemmes ifølge Indstillinger, var gået.
asset-unreal-package = Fra Unreal-pakken
asset-ue-needs-editor-before = Venter på Unreal Editor: { $reason }. På en maskine med Unreal installeret skal du køre
asset-ue-needs-editor-after = så kommer den tilbage som FBX.
asset-ue-editor-path = sti til UnrealEditor-Cmd
asset-ue-unsupported = Beholdt som Unreal-pakke: { $reason }.
asset-ue-failed = Konverteringen mislykkedes: { $reason }
asset-recovered = Den originale .{ $ext } lå gemt i denne Unreal-pakke, og Hoard trak den ud.
asset-recovered-download = Download .{ $ext }
asset-ue-meshes-before = Meshes og animationer er gemt i Unreals eget format. Konvertér pakken (pakkesiden, Mere), og kør så
asset-ue-meshes-after = ved siden af en Unreal Editor-installation for at få FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } asset registry-tag
       *[other] { $count } asset registry-tags
    }
asset-file = Fil
asset-bytes = Bytes
asset-source = Kilde
asset-path = Sti
asset-path-open-folder = Se, hvad der ellers er i { $folder }
asset-download-maps =
    { $count ->
        [one] Download { $count } map
       *[other] Download alle { $count } maps
    }
asset-download-linked =
    { $count ->
        [one] Download med { $count } linket fil
       *[other] Download med { $count } linkede filer
    }
asset-download-only-title = Kun denne fil, uden de filer, den bruger
asset-download-only = Kun denne fil
asset-include-animations = Medtag animationer i .zip-filen
asset-include-character = Medtag figuren { $name }
asset-copy-toml = Kopiér hoard.toml-post
asset-copy-id = Kopiér ID
asset-teach = Lær Hoard denne fil
asset-teach-title = Beskriv denne butiks fil, så Hoard kan læse den (Indstillinger → Butiksfiler)
asset-archive = Arkivér
asset-archive-title = Ude af søgningen, indtil det gendannes eller slettes for altid fra Arkivet

## ImageViewer.vue

image-channel-red = Rød kanal
image-channel-green = Grøn kanal
image-channel-blue = Blå kanal
image-channel-alpha = Alfakanal
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Ternet baggrund (K skifter)
image-bg-black-title = Sort baggrund (K skifter)
image-bg-gray-title = Grå baggrund (K skifter)
image-bg-white-title = Hvid baggrund (K skifter)
image-bg-checker = ternet baggrund
image-bg-black = sort baggrund
image-bg-gray = grå baggrund
image-bg-white = hvid baggrund
image-inspect-aria = Undersøg { $name }
image-preview-size = forhåndsvisning { $width }×{ $height }, { $bytes }
image-preview = forhåndsvisning
image-resolution = Opløsning
image-full-title = Indlæs hele billedet på { $width }×{ $height } (F); det sker også, hvis du zoomer længere ind end forhåndsvisningen
image-full-loading = Indlæser fuld opløsning…
image-full-failed = Fuld opløsning blev ikke indlæst: prøv igen
image-full = Fuld opløsning
image-zoom = Zoom
image-zoom-out-title = Zoom ud (−)
image-zoom-out = Zoom ud
image-zoom-in-title = Zoom ind (+)
image-zoom-in = Zoom ind
image-fit-title = Tilpas (0)
image-fit = Tilpas
image-actual-title = Faktiske pixels (1)
image-view = Visning
image-tile-title = Gentag 3×3 for at tjekke sømmene (T)
image-tile = Gentag
image-channels = Kanaler
image-close = Luk (Esc)
image-failed = Dette billede kunne ikke indlæses.
image-loading = Indlæser…
image-key-gestures = Rul eller knib for at zoome · træk for at panorere · dobbeltklik for at zoome ind eller tilpasse
image-key-tile = gentag
image-key-background = baggrund
image-key-full = fuld opløsning
image-key-channels = kanaler

## SpritePlayer.vue

sprite-frame-aria = { $name }, billede { $frame } af { $count }
sprite-failed = Denne kan ikke afspilles.
sprite-loading = Indlæser billeder…
sprite-previous = Forrige billede
sprite-next = Næste billede
sprite-pause = Sæt animation på pause
sprite-play = Afspil animation
sprite-animation = Animation
sprite-all-frames = Alle billeder
sprite-grid-title = Hvordan arket skæres op i billeder
sprite-columns = Kolonner
sprite-rows = Rækker
sprite-fps-aria = Billeder pr. sekund
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
        [one] { $count } side
       *[other] { $count } sider
    }
sheet-animations =
    { $count ->
        [one] { $count } animation
       *[other] { $count } animationer
    }
# The box that narrows the list of sprites by name.
sheet-find = Find en sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Viser { $shown } af { $total }
sheet-no-match = Ingen sprite har det i sit navn.
sheet-failed = Dette ark kan ikke læses.
sheet-loading = Indlæser arket…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoards eget format: siderne som de er, sprites drejet vilkårligt og beskåret, med animationer og hvert billedes varighed.
sheet-note-texturepacker = Sprites drejet med uret og beskåret, med animationer efter navn, sådan som PixiJS og Phaser læser dem.
sheet-note-clockwise = Sprites drejet med uret og beskåret. Det kan ikke rumme animationer: billederne følger deres nummererede navne.
sheet-note-libgdx = Sprites drejet mod uret og beskåret. Det kan ikke rumme animationer: billederne følger deres nummererede navne.
sheet-note-godot-frames = Sprites drejes aldrig, og det beskårne bevares som margener, med animationer og hvert billedes varighed.
sheet-note-godot-tileset = Fliser drejes eller beskæres aldrig, i et gitter på flisestørrelsen, og billederne beholdes, hvis de allerede ligger i et.
sheet-note-grid = Hver sprite opret på hele sit lærred, i celler af samme størrelse, til alt, der skærer et ark op i et gitter.
sheet-note-frames = Hver sprite opret på hele sit lærred, hver som en PNG opkaldt efter den.

## TextViewer.vue, TextPreview.vue og text.ts

text-lang-config = Konfiguration
text-lang-table = Tabel
text-lang-code = Kode
text-lang-script = Script
text-lang-text = Tekst
text-copy-blocked = Kunne ikke kopiere: browseren blokerede det
text-read-aria = Læs { $name }
text-lines =
    { $count ->
        [one] { $count } linje
       *[other] { $count } linjer
    }
text-show-as = Vis som
text-tree-title = Foldbart træ (T)
text-tree = Træ
text-pretty-title = Indrykket på ny (P)
text-pretty = Pæn
text-raw-title = Som skrevet (R)
text-raw = Rå
text-document-title = Som et dokument (D)
text-document = Dokument
text-source-title = Markdown som skrevet (R)
text-source = Kilde
text-page-title = Som en browser viser den fra dens mappe, holdt adskilt fra Hoard (P)
text-page = Side
text-page-source-title = Siden, som den er skrevet (R)
text-page-aria = { $name } som side
text-page-error = Kunne ikke åbne denne side: { $error }
text-fold = Fold
text-expand-all-title = Fold alt ud
text-expand-all = Udvid alle
text-collapse-all-title = Fold alt sammen
text-collapse-all = Skjul alle
text-group-text = Tekst
text-wrap-title = Ombryd lange linjer (W)
text-wrap = Ombryd
text-copy-title = Kopiér teksten (C)
text-copied = Kopieret
text-copy = Kopiér
text-download-title = Download filen
text-download = Download
text-close = Luk (Esc)
text-truncated = Viser de første { $shown } af { $total }.
text-truncated-json = Viser de første { $shown } af { $total } (træet kræver hele filen).
text-show-all = Vis alt
text-json-error = Denne JSON kan ikke fortolkes: { $error }
text-contents-aria = Indholdet af { $name }
text-loading = Indlæser…
text-load-error = Kunne ikke indlæse denne fil: { $error }
text-binary = Denne fil er binær, ikke tekst, så der er intet at læse her. Download den for at åbne den i sit eget værktøj.
text-key-tree = træ
text-key-pretty = pæn
text-key-raw = rå
text-key-document = dokument
text-key-source = kilde
text-key-page = side
text-key-wrap = ombryd
text-key-copy = kopiér
text-key-find = find
text-key-close = luk
text-preview-aria = Læs { $name } i fuld skærm
text-preview-read = Læs

## JsonTree.vue og JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } element
       *[other] { $count } elementer
    }
json-keys =
    { $count ->
        [one] { $count } nøgle
       *[other] { $count } nøgler
    }
json-fold = Fold sammen
json-unfold = Fold ud
json-unfold-summary = Fold { $summary } ud
json-show-more = Vis { $count } mere
json-left = af { $count } tilbage
json-picker-more = … { $count } mere

## 3D-modelfremvisere

model-viewer-failed = 3D-fremviseren blev ikke indlæst ({ $error }); det kan hjælpe at genindlæse siden
model-loading-viewer = Indlæser 3D-fremviseren…
model-loading = Indlæser model…
model-show-failed = Kunne ikke vise denne model: { $error }
model-preview-failed = Kunne ikke vise en forhåndsvisning af denne model: { $error }
model-error-converting = serveren er stadig ved at konvertere denne model
model-error-converting-desktop = Hoard er stadig ved at konvertere denne model
model-error-no-loader = Ingen browserindlæser til .{ $ext }
model-dialog = Vis { $name } i 3D
model-title-stats =
    { $triangles } trekanter · { $meshes ->
        [one] { $meshes } mesh
       *[other] { $meshes } meshes
    } · { $materials ->
        [one] { $materials } materiale
       *[other] { $materials } materialer
    }
model-triangles =
    { $count ->
        [one] { $count } trekant
       *[other] { $count } trekanter
    }
model-group-camera = Kamera
model-group-shading = Skygge
model-group-background = Baggrund
model-group-animation = Animation
model-group-view = Visning
model-group-show = Vis
model-frame-title = Indram modellen (F)
model-frame = Indram
model-reset-title = Tilbage til startvisningen (R)
model-reset = Nulstil
model-turn-title = Drej langsomt (A)
model-turn = Drej
model-textures-title = Teksturer (T)
model-textures = Teksturer
model-full-res-title = Teksturer i fuld opløsning (H): hver teksturfil, som den er, uanset størrelse; slået fra: lette forhåndsvisninger på højst 2048 px
model-full-res = Fuld opl.
model-light-title = Omgivelseslys (E)
model-light = Lys
model-grid-title = Gulvgitter (G)
model-grid = Gitter
model-bg-checker = Ternet baggrund
model-bg-dark = Mørk baggrund
model-bg-gray = Grå baggrund
model-bg-light = Lys baggrund
model-bg-env = Omgivelsesbaggrund
model-bg-title = { $background } (K skifter)
model-pause-title = Pause (mellemrum)
model-play-title = Afspil (mellemrum)
model-pause = Pause
model-play = Afspil
model-animation-clip = Animationsklip
model-animation = Animation
model-plays-on = Afspilles på
model-plays-everywhere = Brug overalt
model-plays-everywhere-title = Afspil alle animationer med dette skelet på { $name }, også i deres miniaturer
model-plays-everywhere-done = Animationer med dette skelet afspilles nu på { $name }. Deres miniaturer tegnes igen.
model-plays-everywhere-failed = Kunne ikke ændre den figur, de afspilles på: { $error }
model-rest-pose = Hvilestilling
model-close-title = Luk (Esc)
model-full-screen = Fuld skærm
model-open-full-screen = Åbn 3D-fremviseren i fuld skærm
model-menu-fold = Fold menuen væk
model-menu-show = Vis menuen
model-view-prefix = Visning:{ " " }
model-menu-render = Render
model-menu-maps = Maps
model-menu-inspect = Undersøg
model-mode-final = Endelig render
model-mode-raw = Rå render
model-mode-color = Grundfarve
model-mode-roughness = Ruhed
model-mode-metalness = Metal
model-mode-normal = Normal
model-mode-ao = Okklusion
model-mode-matcap = Matcap
model-mode-uv = UV-tjek
model-mode-geometry = Geometrier
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Trådmodel (W), oven på enhver visning
model-wireframe = Trådmodel
model-info-title = Sceneoplysninger (I)
model-info = Sceneoplysninger
model-info-close = Luk sceneoplysninger
model-info-close-title = Luk (I)
model-info-geometries = Geometrier
model-info-materials = Materialer
model-info-textures = Teksturer
model-info-triangles = Trekanter
model-info-vertices = Hjørnepunkter
model-info-units = Enheder
model-info-in-meters = Størrelse i meter ud fra de enheder, filen angiver
model-units-m = Meter
model-units-cm = Centimeter
model-units-mm = Millimeter
model-units-in = Tommer
model-units-ft = Fod
model-units-other = 1 enhed = { NUMBER($n, maximumSignificantDigits: 3) } m
model-units-unknown = Ikke angivet
model-info-no-units = Dette format angiver ikke, hvilke enheder det bruger
model-info-height = Højde
model-info-length = Længde
model-info-width = Bredde
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } enhed
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } enheder
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Tekstur ikke fundet: { $name }
model-missing-some = { $count } teksturer ikke fundet: { $names }
model-missing-more = { $count } teksturer ikke fundet: { $names }…
model-missing-count = { $count } teksturer ikke fundet
model-guessed = Teksturer matchet efter navn; filen nævner ingen.
model-group-facing = Retning
model-facing-turn = Drej en kvart
model-facing-turn-title = Drej modellen en kvart omgang mod uret, set oppefra
model-facing-tip = Vip en kvart
model-facing-tip-title = Vip modellens top en kvart omgang frem, så en liggende model kommer til at stå op
model-facing-save = Brug til pakken
model-facing-save-title = Drej alle modeller i { $pack } sådan. Deres miniaturer tegnes igen; downloads forbliver, som de blev lavet.
model-facing-undo = Fortryd
model-facing-undo-title = Tilbage til, hvordan pakkens modeller er drejet
model-facing-saved = Alle modeller i { $pack } vender nu sådan. Deres miniaturer tegnes igen.
model-facing-failed = Kunne ikke dreje pakkens modeller: { $error }
model-keys-mouse = Træk for at kredse · højretræk for at panorere · rul for at zoome · dobbeltklik for at indramme
model-key-frame = indram
model-key-reset = nulstil
model-key-view = visning
model-key-wireframe = trådmodel
model-key-info = sceneoplysninger
model-key-textures = teksturer
model-key-full-res = teksturer i fuld opløsning
model-key-light = lys
model-key-grid = gitter
model-key-background = baggrund
model-key-play = afspil
explorer-dialog = Mesh-udforsker: flyv rundt blandt modellerne i disse resultater
explorer-title = Mesh-udforsker
explorer-group-view = Visning
explorer-true-size = Faktisk størrelse
explorer-true-size-title = Vis hver model i dens faktiske størrelse i meter, op til 12 m, i stedet for tilpasset dens felt (T)
explorer-animate = Animer
explorer-animate-title = Afspil hver animeret models første klip (Mellemrum)
explorer-orbit = Kredsløb
explorer-orbit-title = Træk for at dreje rundt om den valgte model i stedet for at se dig omkring (O)
explorer-open = Åbn
explorer-open-title = Åbn denne models detaljer (Enter)
explorer-size = { $x } × { $y } × { $z } m
explorer-empty = Ingen af disse resultater kan vises i 3D.
explorer-hud-loaded =
    { $total ->
        [one] { $loaded } af { $total } model indlæst
       *[other] { $loaded } af { $total } modeller indlæst
    }
explorer-hud-loading = { $count } indlæses
explorer-hud-memory = ~{ $mb } MB
explorer-hud-speed = Fart { $speed } m/s
explorer-hud-fps = { $fps } fps
explorer-keys-mouse = Klik for at vælge · træk for at se dig om · træk med midterknappen for at panorere · Alt-træk for at kredse · rul for at bevæge dig, med højre knap holdt nede for at ændre farten · dobbeltklik for at tilpasse
explorer-key-fly = flyv
explorer-key-down-up = ned, op
explorer-key-faster = hurtigere
explorer-key-open = åbn
explorer-key-true-size = faktisk størrelse
explorer-key-orbit = kredsløb
explorer-key-animate = animer
explorer-key-close = luk

## Materialer

material-role-base-color = Farve
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Ruhed
material-role-gloss = Glans
material-role-metalness = Metal
material-role-ao = Ambient occlusion
material-role-orm = Okklusion, ruhed, metal
material-role-mra = Metal, ruhed, okklusion
material-role-rma = Ruhed, metal, okklusion
material-role-height = Højde
material-role-bump = Bump
material-role-cavity = Cavity
material-role-specular = Specular
material-role-opacity = Opacitet
material-role-emissive = Emission
material-role-translucency = Gennemskinnelighed
material-role-base-color-name = farve
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = ruhed
material-role-gloss-name = glans
material-role-metalness-name = metal
material-role-ao-name = ambient occlusion
material-role-orm-name = okklusion, ruhed, metal
material-role-mra-name = metal, ruhed, okklusion
material-role-rma-name = ruhed, metal, okklusion
material-role-height-name = højde
material-role-bump-name = bump
material-role-cavity-name = cavity
material-role-specular-name = specular
material-role-opacity-name = opacitet
material-role-emissive-name = emission
material-role-translucency-name = gennemskinnelighed
material-maps =
    { $count ->
        [one] { $count } map
       *[other] { $count } maps
    }
material-dialog = Vis materialet { $name }
material-up-to = op til { $size }
material-repeat-size = { $size } pr. gentagelse
material-shape-sphere = Kugle
material-shape-cube = Terning
material-shape-plane = Plan
material-shape-title = { $shape } ({ $key })
material-group-shape = Form
material-group-tiling = Gentagelse
material-real-title = Virkelig størrelse: én gentagelse er { $size }, kuglen er 2 m i diameter (T skifter)
material-repeat-title = Gentag { $n }× (T skifter)
material-real = Virkelig
material-repeat = { $n }×
material-height-title = Hvor langt højde-mappet skubber overfladen
material-height = Højde
material-height-strength = Højdestyrke
material-loading = Indlæser materiale…
material-show-failed = Kunne ikke vise dette materiale: { $error }
material-preview-failed = Kunne ikke vise en forhåndsvisning af dette materiale: { $error }
material-group-maps = Maps
material-map-show = Vis mappet for { $map }
material-map-hide = Skjul mappet for { $map }
material-map-unused = Bruges ikke af forhåndsvisningen
material-open-full-screen = Åbn materialefremviseren i fuld skærm
material-keys-mouse = Træk for at kredse · højretræk for at panorere · rul for at zoome · klik på et map for at slå det fra
material-key-shape = form
material-key-tiling = gentagelse
material-key-turn = drej

## theme.ts

theme-slate = Skifer
theme-paper = Papir
theme-linen = Hør
theme-sage = Salvie
theme-lilac = Syren
theme-blush = Rosa
theme-graphite = Grafit
theme-obsidian = Obsidian
theme-ember = Glød
theme-moss = Mos
theme-aubergine = Aubergine
theme-oxblood = Okseblod
theme-sky = Himmel
theme-mint = Mynte
theme-apricot = Abrikos
theme-lemon = Citron
theme-midnight = Midnat
theme-lagoon = Lagune
theme-neon = Neon
theme-light-slate-blurb = Kølig blågrå
theme-light-paper-blurb = Enkel neutral grå
theme-light-linen-blurb = Varm creme, rav
theme-light-sage-blurb = Blød grøn
theme-light-lilac-blurb = Bleg violet
theme-light-blush-blurb = Støvet rosa
theme-light-sky-blurb = Klar himmelblå
theme-light-mint-blurb = Kølig akvamynte
theme-light-apricot-blurb = Blød fersken, orange
theme-light-lemon-blurb = Lysegul, okker
theme-dark-graphite-blurb = Neutral grå, uden farvetone
theme-dark-slate-blurb = Blågrå
theme-dark-obsidian-blurb = Næsten sort, til OLED
theme-dark-ember-blurb = Varm koksgrå, rav
theme-dark-moss-blurb = Skovgrøn-sort
theme-dark-aubergine-blurb = Blommesort, syren
theme-dark-oxblood-blurb = Vinsort, koral
theme-dark-midnight-blurb = Dyb marineblå, himmelblå
theme-dark-lagoon-blurb = Dyb blågrøn, akva
theme-dark-neon-blurb = Næsten sort, knaldpink

## texnames.ts

texnames-needs-map = Mønsteret skal indeholde { "{" }map{ "}" }, ellers ville alle et materiales maps få samme navn
texnames-placeholders = Mønsteret kan bruge { "{" }name{ "}" }, { "{" }map{ "}" } og { "{" }size{ "}" }
texnames-bad-characters = Navne må ikke indeholde < > : " / \ | ? eller *
texnames-no-suffix = Giv maps af typen { $role } et suffiks
texnames-bad-suffix = Suffikset for maps af typen { $role } må ikke indeholde < > : " / \ | ? eller *
texnames-slot-no-suffix = Giv »{ $slot }« et suffiks
texnames-slot-bad-suffix = Suffikset for »{ $slot }« må ikke indeholde < > : " / \ | ? eller *
texnames-ending-missing = Hver af dine egne endelser skal have den endelse, der skal søges efter
texnames-ending-bad = Dine egne endelser må ikke indeholde < > : " / \ | ? eller *
texnames-ending-twice = { $ending } står to gange i dine endelser
texnames-slot-metallic-roughness = Metal og ruhed
texnames-slot-specular-color = Specular-farve
texnames-slot-specular-glossiness = Specular og glans
texnames-slot-clearcoat = Clearcoat
texnames-slot-clearcoat-roughness = Clearcoat-ruhed
texnames-slot-clearcoat-normal = Clearcoat-normal
texnames-slot-sheen-color = Sheen-farve
texnames-slot-sheen-roughness = Sheen-ruhed
texnames-slot-transmission = Transmission
texnames-slot-thickness = Tykkelse
texnames-slot-diffuse-transmission = Diffus transmission
texnames-slot-diffuse-transmission-color = Farve for diffus transmission
texnames-slot-iridescence = Iridescens
texnames-slot-iridescence-thickness = Iridescens-tykkelse
texnames-slot-anisotropy = Anisotropi

## storeFormats.ts

storefiles-field-title = Titel
storefiles-field-title-hint = Butikkens præcise titel. Den navngiver pakken, og uden den springes filen over.
storefiles-field-seller = Sælger
storefiles-field-seller-hint = Hvem der har lavet eller sælger den.
storefiles-field-license = Licens
storefiles-field-license-hint = Som filen skriver den, f.eks. CC-BY-4.0.
storefiles-field-url = Butiksside
storefiles-field-url-hint = Butikssidens adresse, der starter med http:// eller https://.
storefiles-field-words = Tags
storefiles-field-words-hint = Ord at tagge den med. Kommaseparerede værdier deles op.
storefiles-field-type = Type
storefiles-field-type-hint = Hvad det er, f.eks. »3D model« eller »surface«. Også et tag.
storefiles-field-colors = Farver
storefiles-field-colors-hint = Dens farver, som color:-tags.
storefiles-field-id = Butiks-id
storefiles-field-id-hint = Butikkens id for den: bogstaver, cifre, _ eller -.
storefiles-field-origin = Lavet i
storefiles-field-origin-hint = Hvor den blev lavet eller scannet, til beskrivelsen.
storefiles-field-size = Størrelse
storefiles-field-size-hint = Hvor stor den er, i ord, til beskrivelsen.
storefiles-field-title-name = titlen
storefiles-field-seller-name = sælgeren
storefiles-field-license-name = licensen
storefiles-field-url-name = butikssiden
storefiles-field-words-name = taggene
storefiles-field-type-name = typen
storefiles-field-colors-name = farverne
storefiles-field-id-name = butiks-id'et
storefiles-field-origin-name = stedet, den blev lavet
storefiles-field-size-name = størrelsen

## StoreFormatsPanel.vue

storefiles-load-failed = Kunne ikke indlæse butiksfilformaterne: { $error }
storefiles-remove-title = Fjern { $name }?
storefiles-remove-body = { $file } slettes fra datamappen. Pakker, der allerede er læst med det, beholder det, der blev læst.
storefiles-remove = Fjern
storefiles-removed = Fjernede { $name }.
storefiles-remove-failed = Kunne ikke fjerne { $name }: { $error }
storefiles-unread = Filer, Hoard ikke læser
storefiles-loading = Indlæser…
storefiles-unread-none = Ingen: ingen fil i biblioteket ligner en fil fra en butik, der ikke læses.
storefiles-unread-what = Filer i mere end én pakke, eller navngivet som en butiks filer, som intet format læser.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pakke
       *[other] { $count } pakker
    }
storefiles-unread-example = f.eks. { $pack }
storefiles-teach = Lær Hoard denne fil
storefiles-formats = Formater
storefiles-formats-what = Dine prøves først og træder i stedet for et indbygget format med samme navn.
storefiles-dir-before = De opbevares i
storefiles-dir-after = .
storefiles-yours = Dine
storefiles-built-in = Indbygget
storefiles-edit = Redigér
storefiles-start-from = Start ud fra dette

## TagsPanel.vue

tags-load-failed = Kunne ikke indlæse taggene: { $error }
tags-rename-title = Omdøb { $tag }?
tags-merge-title = Flet { $count } tags?
tags-rename-body = Dets { $count } assets får { $into } i stedet, og taggerne bruger det nye navn fra nu af.
tags-merge-body = { $tags } bliver til { $into } på op til { $count } assets. Taggerne bruger { $into } fra nu af.
tags-rename = Omdøb
tags-merge = Flet
tags-merged =
    { $count ->
        [one] { $count } asset er nu tagget { $into }
       *[other] { $count } assets er nu tagget { $into }
    }
tags-merge-failed = Kunne ikke flette: { $error }
tags-forget-failed = Kunne ikke ændre det: { $error }
tags-find = Find tags
tags-find-placeholder = Find tags, f.eks. rock eller set:
tags-sort = Rækkefølge
tags-sort-count = Mest brugt
tags-sort-name = A–Å
tags-count =
    { $count ->
        [one] { $count } tag
       *[other] { $count } tags
    }
tags-count-more = { $count }+ tags
tags-what = Vælg tags, der betyder det samme, og flet dem, eller vælg ét for at omdøbe det.
tags-new-name = Nyt navn
tags-merge-into = Flet ind i
tags-saved-as-before = Gemt som
tags-saved-as-after = { "" }
tags-cancel = Annullér
tags-loading = Indlæser…
tags-none = Ingen tags.
tags-none-containing = Ingen tags, der indeholder »{ $text }«.
tags-browse-title = Gennemse assets tagget { $tag }
tags-browse = Gennemse
tags-merged-heading = Flettede tags
tags-merged-what = Når taggerne finder på et af disse gamle navne, får assettet det tag, det blev flettet ind i.
tags-allow-old = Tillad det gamle navn igen
tags-lookalike-heading = Mærker, der ligner hinanden
tags-lookalike-what = En flertalsform, de samme ord skrevet sammen, ét bogstav forkert eller et ord, der også er et mærke med et navnerum (red og color:red). Ved sammenlægning beholdes det mest brugte navn eller det med navnerum, og mærkerne bruger det fremover.
tags-lookalike-merge-title = Læg { $other } sammen med { $keep }
tags-lookalike-apart = Forskellige
tags-lookalike-apart-title = { $a } og { $b } betyder forskellige ting: hold op med at foreslå at lægge dem sammen
tag-page-alias-remove = Glem det gamle navn { $alias }: mærkerne kan give det igen i stedet for { $tag }
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Gammelt navn: { $names }
       *[other] Gamle navne: { $names }
    }
tag-page-aliases-label =
    { $count ->
        [one] Gammelt navn:
       *[other] Gamle navne:
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } blev flettet ind i { $into }.
tag-page-show = Vis { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Skriv, hvad dette tag betyder…
tag-page-describe-label = Hvad dette tag betyder
tag-page-describe-placeholder = Hvad det er til, og hvad der hører under det. Enter gemmer.
tag-page-describe-edit = Klik for at ændre
tag-page-description-saved = Gemte, hvad tagget betyder
# The tag's picture: one of its assets stands for it.
tag-page-picture = Vælg et billede til dette tag
tag-page-picture-help = Vælg en af dets assets til at repræsentere det
tag-page-picture-nothing = Ingen af de viste assets har et billede.
tag-page-picture-none = Intet billede
tag-page-picture-saved = Billede indstillet
tag-page-picture-removed = Billede fjernet
tag-page-rename = Omdøb…
tag-page-rename-title = Giv det et nyt navn, eller navnet på et andet tag for at flette det ind i det
tag-page-rename-label = Nyt navn
tag-page-save-failed = Kunne ikke gemme tagget: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Under
tag-page-parent-add = Sæt under…
tag-page-parent-add-title = Sæt dette tag under et bredere: når du søger efter det, finder du også disse
tag-page-parent-placeholder = Et bredere tag, fx køretøj
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Tag { $tag } ud fra under { $parent }
tag-page-parent-saved = { $tag } er nu under { $parent }
tag-page-parent-removed = { $tag } er ikke længere under { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Snævrere:
tag-page-narrower-title = Når du søger efter { $tag }, finder du også disse
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Bredere og snævrere
tags-tree-what = Sæt et tag under et bredere, her eller på dets tag-side, så finder en søgning på det bredere også alt under det.
tags-tree-none = Intet er under noget endnu. Prøv at sætte bil under køretøj.
tags-tree-tag-label = Tag, der skal under et bredere
tags-tree-tag-placeholder = Et tag, fx bil
tags-tree-goes-under = hører under
tags-tree-parent-label = Bredere tag
tags-tree-put = Sæt under

## LibraryStats.vue

stats-source-human = Tilføjet af personer
stats-source-ai = Foreslået af AI
stats-source-rule = Indbyggede regler
stats-source-meta = Fra filerne
stats-assets = Assets
stats-packs = Pakker
stats-on-disk = På disken
stats-duplicates-title = { $count } filer er byte-identiske kopier af andre og gemmes én gang
stats-saved-dedup = Sparet ved deduplikering
stats-distinct-tags = Forskellige tags
stats-archive = I arkivet
stats-load-failed = Kunne ikke indlæse detaljerne: { $error }
stats-counting = Tæller…
stats-as-of = Pr. { $time }
stats-updating = Opdaterer…
stats-by-kind = Plads efter type
stats-files-size = { $name }: { $count } filer, { $size }
stats-months = Tilføjet det seneste år
stats-month = { $month }: { $count } assets, { $size }
stats-tagging = Tagging
stats-tagged = Tagget
stats-untagged = { $count } uden tags
stats-described = Beskrevet
stats-described-assets = { $count } assets
stats-human-edited = Redigeret manuelt
stats-human-edited-unit =
    { $count ->
       *[other] assets
    }
stats-source-different = { $count } forskellige
stats-most-used = Mest brugt
stats-show-tag = Vis assets med { $tag }
stats-file-types = Filtyper
stats-biggest-packs = Største pakker
stats-browse-pack = Gennemse { $name }
stats-largest-files = Største filer
stats-open-file = Åbn { $name }
stats-empty = Intet i biblioteket endnu. Tilføj en pakke, så dukker dens tal op her.

## HoardPanel.vue

hoard-title = Din skat
hoard-what = Hvert asset i biblioteket er et stykke skat, og dets art afgør hvilket. Saml det nye op for at lægge det på din bunke.
hoard-sign-in = Log ind for at samle din egen skat.
hoard-new = Nyt bytte at samle: { $count }
hoard-none-new = Intet nyt at samle.
hoard-collect = Saml
hoard-unavailable = Denne browser kan ikke tegne skatten: { $error }
hoard-collect-failed = Kunne ikke gemme det, du samlede: { $error }
hoard-arcade = Arkadeskærm
hoard-full-screen = Fuld skærm
hoard-leave-full-screen = Afslut fuld skærm
hoard-label = En drage, der vogter sin skat. Skatte: { $count }
hoard-gold = Guld
hoard-gems = Ædelsten
hoard-books = Bøger
hoard-weapons = Våben
hoard-bones = Knogler
hoard-tablets = Runetavler
hoard-paintings = Malerier
hoard-potions = Eliksirer

## UpdatesPanel.vue

updates-saved = Hoard kalder den, når en opdatering er klar
updates-saved-none = Ingen adresse at kalde
updates-save-failed = Kunne ikke gemme
updates-call-answered = Den svarede { $result }
updates-call-refused = Den tog ikke imod forespørgslen: { $result }
updates-call-failed = Kunne ikke kalde den
updates-called = Kaldt { $when } for { $version }: { $result }
updates-called-failed = Kaldt { $when } for { $version }: { $result }. Hoard prøver igen om en time.
updates-latest = Hoard { $version } er den nyeste udgivelse.
updates-out = Hoard { $version } er udkommet.
updates-out-idle = Hoard { $version } er udkommet; kalder inden for et par minutter, mens der ikke sker noget.
updates-out-busy = Hoard { $version } er udkommet; venter, til der ikke sker noget ({ $work }).
updates-busy-more = { $count } mere
updates-busy-jobs =
    { $count ->
        [one] { $count } job venter
       *[other] { $count } job venter
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } upload
       *[other] { $count } uploads
    }
updates-list-separator = ,{ " " }
updates-what = Når en ny udgivelse er ude, kan Hoard kalde en adresse, så snart der ikke sker noget (ingen uploads, job, konverteringer eller vedligeholdelse), så det, der opdaterer containeren, ikke genstarter den midt i en upload. Den sender en POST med udgivelsen i en JSON-body. Dette kræver udgivelsestjekket ovenfor.
updates-url = Adresse at kalde
updates-url-placeholder = Ingen (f.eks. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Gemt (skriv for at erstatte)
updates-auth-placeholder = Ingen (f.eks. Bearer my-token)
updates-remove = Fjern
updates-save = Gem
updates-call-now-title = Det, den er koblet til, kører
updates-call-now = Kald den nu
updates-what-calls = Hvad kan den kalde?
updates-watchtower = Watchtowers HTTP-API
updates-watchtower-at = :{ " " }
updates-watchtower-with = , med{ " " }
updates-watchtower-token = { " " }og tokenet fra{ " " }
updates-watchtower-its = { " " }(med dens{ " " }
updates-watchtower-endpoint = -endpoint slået til).
updates-portainer = En Portainer-webhook
updates-portainer-rest = { " " }på Hoards stack eller container, med re-pull image slået til. Tokenet står i URL'en.
updates-script = Et script på værten
updates-script-listener = { " " }bag en webhook-lytter (f.eks.{ " " }
updates-script-runs = ), der kører{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }eller alt andet, der tager imod en webhook.
updates-poll-before = Eller spring adressen over: et script med et adgangstoken kan polle{ " " }
updates-poll-until = { " " }indtil{ " " }
updates-poll-and = { " " }er sat, og{ " " }
updates-poll-after = { " " }er true, og derefter hente og genstarte Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Kunne ikke indlæse Vedligeholdelse
maintenance-reanalyze-title = Analysér alle pakker igen?
maintenance-reanalyze-body = Hver fil læses igen, hvilket kan tage lang tid på et stort bibliotek; pakker vises som i gang, indtil det er deres tur. Dine egne navne, tags og noter bliver.
maintenance-reanalyze-confirm = Analysér igen
maintenance-gc-title = Slet ubrugte filer?
maintenance-gc-body = Gemte filer, som intet i biblioteket peger på, slettes for altid. Pakker i arkivet beholder deres.
maintenance-gc-confirm = Slet dem
maintenance-start-failed = Kunne ikke starte »{ $task }«
maintenance-just-now = lige nu
maintenance-minutes-ago = for { $count } min. siden
maintenance-hours-ago = for { $count } t siden
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min.
maintenance-took-hours = { $hours } t
maintenance-loading = Indlæser…
maintenance-running = Kører…
maintenance-starting = Starter…
maintenance-run = Kør
maintenance-stop = Stop
maintenance-stopping = Stopper…
maintenance-stop-failed = Kunne ikke stoppe »{ $task }«
maintenance-started = { $detail } · startet { $when }
maintenance-starting-detail = Starter
maintenance-finished = Færdig:{ " " }
maintenance-failed = Mislykkedes:{ " " }
maintenance-never-run = Aldrig kørt
maintenance-history = Tidligere kørsler
maintenance-col-task = Opgave
maintenance-col-finished = Færdig
maintenance-col-took = Varighed
maintenance-col-result = Resultat
maintenance-script-before = Sikkerhedskopier og skadeskontrollen kan også køres fra et script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }og{ " " }
maintenance-script-after = . De kørsler vises også her.

## SettingsPage.vue

settings-title = Indstillinger
settings-loading = Indlæser…
settings-find = Find en indstilling
settings-find-none = Ingen indstilling passer til “{ $query }”.
settings-load-failed = Kunne ikke indlæse indstillingerne: { $error }
settings-page-appearance = Udseende
settings-page-appearance-blurb = Sprog, temaer, tekststørrelse og afstand, og hvad felterne og topbjælken viser. Gemmes kun i denne browser.
settings-page-dragon = Drage
settings-page-dragon-blurb = Din drages skat: hvert asset i biblioteket er et stykke af den.
settings-page-library = Bibliotek
settings-page-library-blurb = Hvad der er i Hoard: hvor stort det er, hvad det består af, hvordan det er vokset, og hvor godt det er tagget. Arkiverede ting tælles ikke med.
settings-page-copies = Næsten-kopier
settings-page-copies-blurb = Filer, der er det samme billede, den samme lyd eller model i en anden størrelse, samplingfrekvens eller et andet format, dem der fylder mest først. Den bedste i hvert sæt er markeret; intet slettes.
copies-loading = Leder efter næsten-kopier…
copies-refreshing = Biblioteket er ændret, siden disse blev fundet. Leder igen…
copies-none = Ingen næsten-kopier: hvert billede, hver lyd og hver model findes kun én gang.
copies-load-failed = Kunne ikke vise næsten-kopier: { $error }
copies-sum = Sæt af næsten-kopier: { $count }. Kopierne, der ikke er de bedste, fylder { $size }.
copies-set = Sæt { $n }
copies-set-wasted = { $size } i kopierne, der ikke er de bedste
copies-more = Vis flere ({ $count } tilbage)
settings-page-tags = Tags
settings-page-tags-blurb = Alle tags i biblioteket. Flet dem, der betyder det samme, eller omdøb et; taggerne bruger derefter det nye navn.
settings-page-store-files = Butiksfiler
settings-page-store-files-blurb = De filer, butikker lægger ved siden af deres grafik, og som Hoard læser for at navngive pakker og kreditere deres skabere. Lær den dem, den ikke kender, og del det, du har lært den.
settings-page-locations = Biblioteksplaceringer
settings-page-locations-blurb = De mapper på serveren, der indeholder biblioteket: dem, Hoard opbevarer filer i, og mapper med pakker, som den indekserer, hvor de ligger.
settings-page-locations-blurb-desktop = De mapper, der indeholder biblioteket: dem, Hoard opbevarer filer i, og mapper med pakker, som den indekserer, hvor de ligger.
settings-page-tools = Værktøjer
settings-page-tools-blurb = Eksterne programmer, Hoard bruger til at konvertere og læse filer.
settings-page-ai = AI-tagging
settings-page-ai-blurb = En AI-model navngiver, beskriver og tagger nye pakker oven i de indbyggede regler. Dens tags er markeret som forslag.
settings-page-search-blurb = Søg i billeder efter det, de viser, og i lyde efter, hvordan de lyder, ikke kun efter deres navne, og find ting, der ligner hinanden i art, ikke kun i farve.
settings-page-search-tab = Søgning
settings-page-adding = Tilføjelse af pakker
settings-page-adding-blurb = Hvad Hoard gør med en pakke, når den tilføjes, ud over at læse og tagge den.
settings-page-downloads = Downloads
settings-page-downloads-blurb = Hvordan filer navngives i det, Hoard sender: downloads, zip-filer, hoard-værktøjet og AI-assistenter. Filerne i biblioteket forbliver, som de er.
settings-page-account = Konto
settings-page-account-blurb = Din adgangskode, de andre personer, der kan logge ind, og adgangstokens til scripts og andre programmer.
settings-page-auth = Godkendelse
settings-page-auth-blurb = Hvordan folk logger ind: Hoards egne konti, et LDAP-katalog, Microsoft Active Directory eller en OAuth-udbyder, hvor længe sessioner varer, og hvilke grupper der får hvilken rolle.
settings-page-connection = Forbindelse
settings-page-connection-blurb = Hvordan browsere og apps når Hoard: den adresse, den er kendt under, og HTTPS, som nogle browserfunktioner kræver.
settings-page-performance = Ydeevne
settings-page-performance-blurb = Hvor hårdt serveren arbejder, hvordan miniaturer laves og opbevares, og hvor meget den højst tager imod.
settings-page-performance-blurb-desktop = Hvor hårdt Hoard arbejder, hvordan miniaturer laves og opbevares, og hvor meget den højst tager imod.
settings-page-issues = Problemer
settings-page-issues-blurb = Hvad der gik galt ved åbning eller behandling af filer: arkiver, der ikke ville pakkes ud, filer, der ikke kunne læses, job, der mislykkedes. Prøv dem igen, når årsagen er rettet, eller afvis dem.
settings-page-maintenance = Vedligeholdelse
settings-page-maintenance-blurb = Sikkerhedskopiér, kontrollér for skader, og ryd op. Én af hver slags kører ad gangen, og du kan forlade siden, mens de kører.
settings-page-about = Om
settings-page-about-blurb = Hvad Hoard er, hvilken version det er, og andres arbejde, det er lavet med.
settings-the-default = standardværdien
settings-default-on = til
settings-default-off = fra
settings-default-auto = automatisk
settings-option-saved = { $name }: { $value }
settings-back-to-default = Tilbage til standard
settings-save = Gem
settings-use = Brug
settings-status-off = Fra

## SettingsPage.vue: Udseende

settings-themes = Temaer
settings-look-light = Lyst
settings-look-dark = Mørkt
settings-look-group =
    { $scheme ->
        [light] Lyst tema
       *[dark] Mørkt tema
    }
settings-preview =
    Viser det { $preview ->
        [light] lyse
       *[dark] mørke
    } tema, så du kan se det. Du bruger det { $used ->
        [light] lyse
       *[dark] mørke
    }{ $system ->
        [yes] { " " }(følger systemet)
       *[other] { "" }
    }, og det vender tilbage, når du forlader denne side.
settings-preview-back =
    Tilbage til { $used ->
        [light] lyst tema
       *[dark] mørkt tema
    }
settings-look-hint = Skift mellem lyst og mørkt (eller følg systemet) fra topbjælken.

## SettingsPage.vue: Værktøjer

settings-blender-what = Konverterer .blend-, .dae-, .abc- og .x3d-modeller til glTF i baggrunden, så de får miniaturer og en 3D-forhåndsvisning. Pakker, der blev tilføjet, mens den var slået fra, konverteres, når den er klar.
settings-blender-download = Bruger en installeret Blender eller downloader Blender 4.5 LTS (ca. 400 MB) fra et blender.org-spejl.
settings-ffmpeg-what = Koder MP3-, OGG- og Opus-downloads og læser lyd, som Hoard ikke selv kan afkode (AAC, M4A, WMA, …).
settings-ffmpeg-download = Bruger en ffmpeg på systemet med de rette encodere eller downloader et statisk build (ca. 80 MB).
settings-tool-reset = { $tool }: tilbage til standard
settings-tool-no-download = Bruger en installeret { $tool }. Der er ikke noget build at downloade til denne maskine, så installér det, eller angiv dets sti.
settings-tool-path = Sti til { $tool }-programmet
settings-mode-off = Fra
settings-mode-auto = Automatisk
settings-mode-path = Dette program
settings-tool-looking = Leder…
settings-tool-downloading = Downloader… fremskridtet vises i aktivitetsindikatoren
settings-tool-ready = Klar: { $detail }
settings-soundfont = MIDI-SoundFont
settings-soundfont-what = De instrumenter, MIDI-filer afspilles og konverteres med.
settings-soundfont-file = SoundFont-fil
settings-soundfont-placeholder = Fundet på serveren, eller indbygget
settings-soundfont-placeholder-desktop = Fundet på denne computer, eller indbygget
settings-soundfont-default = fundet på serveren, eller indbygget
settings-soundfont-default-desktop = fundet på denne computer, eller indbygget
settings-soundfont-send = Vælg en fil…
settings-soundfont-sent = MIDI spiller nu med { $name }

## SettingsPage.vue: Billedsøgning

settings-clip-heading = Søg efter det, billeder viser
settings-clip-reset = Billedsøgning: tilbage til standard
settings-clip-saved-on = Billedsøgning: til
settings-clip-saved-off = Billedsøgning: fra
settings-clip-what = Skriv, hvad du leder efter (»mossy rock«, »sci-fi door«, »wooden barrel«), så dukker billeder, der viser det, op ved siden af navne, der indeholder ordene. »Mere som dette« finder ting af samme slags, ikke kun med de samme farver.
settings-clip-toggle = Slå billedsøgning til
settings-clip-hint = Bruger Googles SigLIP 2-model, som køres på denne servers processor: intet forlader den. Når du slår det til, downloades modellen ({ $size } MB) fra Hugging Face, og derefter lærer den alle billeder i baggrunden med lav prioritet. Den bruger 350 til 750 MB hukommelse, mens den arbejder, eller nogen søger. Når du slår det fra, slettes modellen, men det, den har lært, beholdes til næste gang.
settings-clip-hint-desktop = Bruger Googles SigLIP 2-model, som køres på denne computers processor: intet forlader den. Når du slår det til, downloades modellen ({ $size } MB) fra Hugging Face, og derefter lærer den alle billeder i baggrunden med lav prioritet. Den bruger 350 til 750 MB hukommelse, mens den arbejder, eller du søger. Når du slår det fra, slettes modellen, men det, den har lært, beholdes til næste gang.
settings-clip-looking = Leder efter modellen…
settings-clip-downloading = Downloader modellen… fremskridtet vises i aktivitetsindikatoren
settings-clip-ready-empty = Klar. Billeder læres, efterhånden som pakker tilføjes.
settings-clip-ready-all =
    { $count ->
       *[other] Klar: kender alle { $count } billeder
    }
settings-clip-learning = Klar: lærer billeder, { $done } af { $total } indtil nu
settings-clip-relearning = Indtil den nye model har lært alle billeder, bruger søgninger det, den gamle lærte.
settings-clip-unavailable = Kunne ikke hente modellen: { $detail }. Prøver igen hver time.
settings-clip-where = Kører på
settings-clip-here = Denne server: intet forlader den
settings-clip-here-desktop = Denne computer: intet forlader den
settings-clip-hosted = En hostet tjeneste
settings-clip-hosted-warning = Billeder forlader denne server. Et miniaturebillede af hvert billede i biblioteket og ordene i hver billedsøgning sendes til { $service }, som tager betaling for dem og gemmer dem, som dens vilkår siger.
settings-clip-hosted-warning-desktop = Billeder forlader denne computer. Et miniaturebillede af hvert billede i biblioteket og ordene i hver billedsøgning sendes til { $service }, som tager betaling for dem og gemmer dem, som dens vilkår siger.
settings-clip-hosted-hint = Intet hentes eller køres her: tjenesten lærer hvert billede, et parti ad gangen. En anden tjeneste eller model lærer dem alle igen, og det, hver af dem har lært, gemmes, så du kan skifte tilbage.
settings-clip-saved-here = Billedsøgning: kører på denne server
settings-clip-saved-here-desktop = Billedsøgning: kører på denne computer
settings-clip-saved-hosted = Billedsøgning: kører på { $service }
settings-clip-hosted-unavailable = Kunne ikke bruge { $service }: { $detail }. Prøver igen hver time.
settings-clap-heading = Søg efter, hvordan lyde lyder
settings-clap-reset = Lydsøgning: tilbage til standard
settings-clap-saved-on = Lydsøgning: til
settings-clap-saved-off = Lydsøgning: fra
settings-clap-what = Skriv, hvad du leder efter (»distant thunder«, »glassy UI blip«, »footsteps on gravel«), så dukker lyde, der lyder sådan, op ved siden af navne, der indeholder ordene.
settings-clap-toggle = Slå lydsøgning til
settings-clap-hint = Bruger LAIONs CLAP-model, som køres på denne servers processor: intet forlader den. Når du slår det til, downloades modellen ({ $size } MB) fra Hugging Face, og derefter lytter den til begyndelsen af hver lyd i baggrunden med lav prioritet. Den bruger op til omkring 1 GB hukommelse, mens den arbejder, eller nogen søger. Når du slår det fra, slettes modellen, men det, den har lært, beholdes til næste gang.
settings-clap-hint-desktop = Bruger LAIONs CLAP-model, som køres på denne computers processor: intet forlader den. Når du slår det til, downloades modellen ({ $size } MB) fra Hugging Face, og derefter lytter den til begyndelsen af hver lyd i baggrunden med lav prioritet. Den bruger op til omkring 1 GB hukommelse, mens den arbejder, eller du søger. Når du slår det fra, slettes modellen, men det, den har lært, beholdes til næste gang.
settings-clap-ready-empty = Klar. Lyde læres, efterhånden som pakker tilføjes.
settings-clap-ready-all =
    { $count ->
       *[other] Klar: kender alle { $count } lyde
    }
settings-clap-learning = Klar: lærer lyde, { $done } af { $total } indtil nu

## SettingsPage.vue: AI-tagging

settings-ai-service = Tjeneste
settings-ai-off = Fra
settings-ai-off-hint = Pakker tagges kun med de indbyggede regler.
settings-ai-local = Lokal model
settings-ai-local-hint = llm-tjenesten i Hoards compose-fil (eller cargo xtask llm). Download en model nedenfor; den kører på denne maskine, og intet forlader den.
settings-ai-chatgpt = ChatGPT-abonnement
settings-ai-chatgpt-hint = Bruger dit ChatGPT Go-, Plus-, Pro- eller Business-abonnement i stedet for en API-nøgle.
settings-ai-openrouter-hint = Én nøgle til hundredvis af hostede modeller, nogle af dem gratis. Modeller, der tager billeder, er markeret.
settings-ai-openai-hint = Hostet af OpenAI.
settings-ai-anthropic-hint = Hostet af Anthropic via dets OpenAI-kompatible endpoint.
settings-ai-gemini-hint = Hostet af Google via dets OpenAI-kompatible endpoint.
settings-ai-ollama-hint = En Ollama-server, du selv kører. Skift adressen, hvis det ikke er en container ved navn ollama.
settings-ai-custom = Andet (OpenAI-kompatibelt)
settings-ai-custom-hint = Alt, der taler OpenAIs chat-API: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Forbinder…
settings-ai-connected =
    { $count ->
        [one] Forbundet · { $count } model
       *[other] Forbundet · { $count } modeller
    }
settings-ai-connected-none = Forbundet, men den viser ingen modeller; skriv modellens navn
settings-chatgpt-connected-none = Forbundet; skriv modellens navn
settings-ai-takes-images = tager billeder
settings-ai-saved = AI-tagging: { $service }
settings-ai-saved-off = AI-tagging er slået fra
settings-ai-reset = AI-tagging: tilbage til standard
settings-ai-on = Til: { $service } · { $model }
settings-ai-url = Adresse
settings-ai-key = API-nøgle
settings-ai-key-saved = Gemt (skriv for at erstatte)
settings-ai-key-paste = Indsæt din nøgle
settings-ai-key-optional = Ingen (valgfri)
settings-ai-key-remove = Fjern
settings-ai-get-key = Hent en nøgle
settings-ai-model = Model
settings-ai-model-pick = Vælg eller skriv en model
settings-ai-model-name = Modelnavn
settings-ai-refresh = Opdatér
settings-ai-connect = Forbind
settings-ai-need-key = Tilføj API-nøglen for at se dens modeller
settings-ai-vision = Send også miniaturer (kræver en model, der tager billeder)
settings-ai-batch = Pr. forespørgsel
settings-ai-batch-unit = assets (færre til små lokale modeller, flere til store hostede)

## SettingsPage.vue: ChatGPT-login

settings-chatgpt-account = Konto
settings-chatgpt-checking = Kontrollerer…
settings-chatgpt-status-as = Logget ind som { $email }
settings-chatgpt-status = Logget ind
settings-chatgpt-signed-in-as = Logget ind på ChatGPT som { $email }
settings-chatgpt-signed-in = Logget ind på ChatGPT
settings-chatgpt-signed-out = Logget ud af ChatGPT
settings-chatgpt-continue = Fortsæt med ChatGPT
settings-chatgpt-sign-out = Log ud
settings-chatgpt-sign-out-title = Log ud af ChatGPT?
settings-chatgpt-sign-out-in-use = AI-tagging bruger dit ChatGPT-abonnement, så den stopper, indtil du logger ind igen eller vælger en anden tjeneste.
settings-chatgpt-sign-out-body = Hoard glemmer login og beder OpenAI om at afslutte det.
settings-chatgpt-manage = Administrér forbrug
settings-chatgpt-finish-before = Gør login færdigt i det vindue, der åbnede (
settings-chatgpt-finish-link = åbn det igen
settings-chatgpt-finish-middle = ). Hvis det ender på en side, der ikke vil indlæses, på en adresse, der starter med{ " " }
settings-chatgpt-finish-after = , så kopiér den adresse, og indsæt den her:
settings-chatgpt-paste = Adressen, login endte på
settings-chatgpt-finish = Afslut
settings-chatgpt-usage = Tagging-forespørgsler tæller med i dit ChatGPT-abonnements forbrug. Administrér forbrug viser, hvad Hoard har brugt, og lader dig begrænse det. For at afbryde forbindelsen mellem Hoard og selve ChatGPT skal du gå til Indstillinger → Sikkerhed og login → Log ind med ChatGPT på chatgpt.com.

## SettingsPage.vue: Lokale modeller

settings-local-downloading-progress = Downloader { $percent } % · { $done } af { $total }
settings-local-downloading = Downloader…
settings-local-incomplete = Download stoppet
settings-local-failed = Kunne ikke starte sidste gang
settings-local-in-use-running = I brug · kører
settings-local-in-use = I brug
settings-local-starting = Starter…
settings-local-running = Kører
settings-local-ready-images = Klar · tager billeder
settings-local-ready-text = Klar · kun tekst
settings-local-delete-title = Slet { $name }?
settings-local-delete-in-use = AI-tagging bruger denne model, så den stopper, indtil du vælger en anden. Dens filer slettes fra modelserveren.
settings-local-delete-downloading = Downloaden stopper, og det, der er hentet indtil nu, slettes.
settings-local-delete-body = Dens filer slettes fra modelserveren. Du kan downloade den igen senere.
settings-local-delete = Slet
settings-local-deleting = Sletter…
settings-local-cancel = Annullér
settings-local-resume = Genoptag
settings-local-download = Download
settings-local-start-before = Start den med{ " " }
settings-local-start-or = { " " }(eller{ " " }
settings-local-start-developing = { " " }under udvikling), og{ " " }
settings-local-start-retry = prøv igen
settings-local-start-after = . En server, der kun kører én model, virker også: angiv dens models navn nedenfor.
settings-local-list = På modelserveren
settings-local-none = Intet downloadet endnu. Vælg en nedenfor; den gemmes på serveren, og du kan skifte når som helst.
settings-local-catalog = Modeller at downloade
settings-local-size = { $size } GB · { $license }
settings-local-other = En anden model fra Hugging Face
settings-local-other-placeholder = En anden fra Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Tilføjelse af pakker

settings-store-pages = Butikssider
settings-store-pages-what = Nye pakker slås op i butikkerne i baggrunden, så de får deres butiksside, sælger og licens.
settings-store-lookup = Slå butikssider op automatisk
settings-unreal = Unreal-pakker
settings-unreal-reset = Unreal-pakker: tilbage til standard
settings-unreal-what = Hvad der skal ske med .uasset-filer.
settings-conv = Konverteringsregler
settings-conv-what = Konverterer filer, som Hoard opbevarer, til ét format, f.eks. alle OBJ og FBX til GLB, så biblioteket er ensartet. Den konverterede fil tager originalens plads med dens navn, tags, samlinger og links. Filer i overvågede mapper ændres aldrig.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Til
settings-conv-to = Konvertér til
# Heading over the file formats a rule converts.
settings-conv-from = Fra
settings-conv-remove = Fjern
settings-conv-remove-label = Fjern regel { $n }
settings-conv-taken = En anden regel konverterer allerede dette format
settings-conv-upright = Rejs modellerne op, når de konverteres (Y op, vender mod +Z), sådan som pakkens modeller vises
settings-conv-flac-what = Lyden forbliver nøjagtig den samme, på cirka halvdelen af pladsen, og alt andet i WAV-filen (looppunkter, cue-markører, tags) gemmes i FLAC-filen, så WAV-filen til enhver tid kan sættes tilbage præcis, som den var. WAV-filer med flydende komma- eller 32-bit-samples forbliver WAV.
settings-conv-add = Tilføj en regel
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Behold originaler i
settings-conv-keep-days =
    { $count ->
        [one] dag
       *[other] dage
    }
settings-conv-keep-what = Indtil da kan originalen gendannes fra den konverterede fils side. Teksturer kopieres ikke ind i en konverteret model: den peger på billedfilerne ved siden af. En models .mtl- eller .bin-filer bliver liggende.
settings-conv-save = Gem regler
settings-conv-revert = Fortryd ændringer
settings-conv-existing = Konvertér filer, der allerede er i biblioteket
settings-conv-saved = Konverteringsregler gemt. De gælder for pakker, der tilføjes fra nu af.
settings-conv-converting =
    { $count ->
        [one] Konverterer én fil i baggrunden
       *[other] Konverterer { $count } filer i baggrunden
    }
settings-conv-nothing = Intet i biblioteket skal konverteres.
settings-conv-ask-title =
    { $count ->
        [one] Konvertér én fil, der allerede er i biblioteket?
       *[other] Konvertér { $count } filer, der allerede er i biblioteket?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Den ligger i én pakke.
       *[other] De ligger i { $packs } pakker.
    } Hver original gemmes i { $days } dage, og indtil da kan den gendannes fra sin side.
settings-conv-ask-busy =
    { $count ->
        [one] Én ligger i en pakke, der stadig er ved at blive tilføjet; den konverteres, når det er færdigt.
       *[other] { $count } ligger i pakker, der stadig er ved at blive tilføjet; de konverteres, når det er færdigt.
    }
settings-conv-ask-list = Filer, der ville blive konverteret
settings-conv-ask-more = …og { $count } til
settings-conv-ask-skipped = Bliver, som de er:
settings-conv-skip-watched =
    { $count ->
        [one] én fil i en overvåget mappe
       *[other] { $count } filer i overvågede mapper
    }
settings-conv-skip-unity =
    { $count ->
        [one] én fil, som et Unity-projekt kender ved navn
       *[other] { $count } filer, som et Unity-projekt kender ved navn
    }
settings-conv-skip-used =
    { $count ->
        [one] én fil, som en scene eller en anden fil henviser til
       *[other] { $count } filer, som en scene eller en anden fil henviser til
    }
settings-conv-skip-beside =
    { $count ->
        [one] én fil, hvis konverterede kopi allerede er i dens pakke
       *[other] { $count } filer, hvis konverterede kopier allerede er i deres pakker
    }
settings-conv-skip-blender =
    { $count ->
        [one] én fil, som Blender ikke har læst endnu (Indstillinger → Værktøjer)
       *[other] { $count } filer, som Blender ikke har læst endnu (Indstillinger → Værktøjer)
    }
settings-conv-ask-new = Gem, kun nye pakker
settings-conv-ask-save-convert =
    { $count ->
        [one] Gem og konvertér den
       *[other] Gem og konvertér { $count } filer
    }
settings-conv-ask-convert =
    { $count ->
        [one] Konvertér den
       *[other] Konvertér { $count } filer
    }
settings-unreal-off = Lad dem være
settings-unreal-off-hint = Konvertér fra en pakkes side, når du vil.
settings-unreal-keep = Konvertér, behold originaler
settings-unreal-keep-hint = Teksturer, lyde og materialer kommer ud i almindelige formater; .uasset-filen bliver også.
settings-unreal-replace = Konvertér, kassér originaler
settings-unreal-replace-hint = Sparer plads, når indholdet er trukket ud. Meshes venter stadig på Unreal-workeren.

## SettingsPage.vue: Downloads

settings-tn = Teksturnavne
settings-tn-saved = Teksturnavne gemt
settings-tn-reset = Teksturnavne: tilbage til standard
settings-tn-what = Et materiales maps sendes ud navngivet efter materialet og efter, hvad hvert map er til, uanset hvad filerne hedder: Rock_Normal.png, ikke rock_moss_xeokah0_raw_8K_Normal.png. Filerne i biblioteket røres ikke, og et map, som en model eller MTL-fil, der sendes med, nævner ved navn, beholder det navn, så modellen stadig finder det.
settings-tn-toggle = Navngiv tekstur-maps efter deres materiale
settings-tn-style = Stil
settings-tn-style-group = Navngivningsstil
settings-tn-descriptive = Beskrivende
settings-tn-short = Kort
settings-tn-custom = Din egen
settings-tn-pattern = Mønster
settings-tn-hint-name = { " " }materialet,{ " " }
settings-tn-hint-map = { " " }hvad mappet er til,{ " " }
settings-tn-hint-size = { " " }dets størrelse, når sættet findes i mere end én (tilføjes til sidst, hvis mønsteret udelader det).
settings-tn-example = Eksempel
settings-tn-suffixes = Suffikser for hver type map
settings-tn-slots = I GLB-downloads navngives en tekstur, der er pakket ind i en model uden sin egen fil, efter materialet og den plads, den udfylder, og disse pladser har også suffikser:
settings-tn-endings = Dine egne endelser
settings-tn-endings-what = En tekstur, der ikke er et af et materiales maps, beholder sit eget navn, medmindre det ender på en af disse: _diff → _BC sender rock_diff.png som rock_BC.png. Lad den anden være tom for at fjerne endelsen.
settings-tn-ending-from = Endelse
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Erstat med
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Tilføj en endelse
settings-tn-ending-remove = Fjern
settings-tn-ending-remove-label = Fjern { $ending }

## SettingsPage.vue: Generelt og Forbindelse

settings-name = Navn
settings-name-what = Hvad denne server hedder i topbjælken, i browserfanen og på login-siden, så du kan kende forskel, hvis du har mere end én Hoard.
settings-name-what-desktop = Hvad dette bibliotek hedder i topbjælken.
settings-address = Adresse
settings-address-what = Den adresse, folk og apps bruger til at nå Hoard, når det ikke er den, denne browser er på: en reverse proxys, en tunnels eller et tailnet-navn, f.eks. https://hoard.example.com. Links, som Hoard udleverer (til Claude og andre apps, ved login, på sit certifikat), bruger den.
settings-public-url = Offentlig adresse
settings-public-url-placeholder = Fra browserens adresse (f.eks. https://hoard.example.com)
settings-public-url-default = fra browserens adresse

## ProfilerPanel.vue: profileringen øverst i Indstillinger → Ydeevne

profiler = Profilering
profiler-what = Hvad Hoard har lavet de seneste ti minutter, opdateret hvert andet sekund, mens du ser på. Det gemmes kun i hukommelsen og starter forfra, når Hoard genstarter.
profiler-starting = De første målinger kommer om et øjeblik.
profiler-failed = Kunne ikke læse profileringen: { $error }
profiler-cpu = Processorforbrug
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Hele maskinen
profiler-cpu-summary = Hoard bruger { $hoard } af processorerne, og hele maskinen { $machine }.
profiler-ram = RAM
profiler-ram-peak = Højest
profiler-ram-of = { $used } af { $total }
profiler-ram-summary = Hoard bruger { $hoard } RAM og har højst brugt { $peak } de sidste ti minutter.
profiler-serving = Forespørgsler i gang
profiler-working = Arbejder på
profiler-working-none = Intet kørte i baggrunden.
profiler-working-more =
    { $count ->
        [one] Og én mere.
       *[other] Og { $count } mere.
    }
profiler-requests = Tid brugt på forespørgsler
profiler-requests-what = Hver søjle er ti sekunders forespørgsler, med tiden fordelt efter, hvor den gik hen.
profiler-part-db = Database
profiler-part-db-title = Venter på databasen og kører dens opslag
profiler-part-disk = Disk
profiler-part-disk-title = Læser gemte filer og komprimerer det, der sendes
profiler-part-send = Afsendelse
profiler-part-send-title = Venter, mens netværket tager imod det sendte
profiler-part-other = Andet
profiler-part-other-title = Regner svaret ud
profiler-request-count =
    { $count ->
        [one] { $count } forespørgsel
       *[other] { $count } forespørgsler
    }
profiler-requests-none = Ingen forespørgsler de seneste ti minutter.
profiler-slowest = Langsomst: { $route }, { $time }
profiler-routes = Efter rute
profiler-route = Rute
profiler-count = Forespørgsler
profiler-average = Gennemsnit
profiler-p95 = Langsomste 5 %
profiler-split = Hvor tiden gik hen
profiler-sent = Sendt
profiler-portal = grænsefladens egne filer

## SettingsPage.vue: Ydeevne

settings-render = 3D-miniaturer
settings-render-toggle = Render miniaturer til 3D-modeller
settings-gpu-on = På GPU'en: { $renderer }
settings-gpu-software = Ingen GPU fundet, så i software ({ $renderer })
settings-gpu-none = Ingen GPU eller software-renderer fundet, så modeller får ingen miniaturer
settings-thumbs = Miniaturer
settings-thumbs-what = Gemmes som WebP, komprimeret eller nøjagtigt, alt efter hvad der fylder mindst. Et lille billede, der fylder mindre, som det er, er sin egen miniature. En ændring her laver de miniaturer, du har, om i baggrunden.
settings-previews-size = Forhåndsvisninger: { $size }
settings-thumbs-saved = Miniaturer: { $quality }
settings-thumbs-size = Størrelse
settings-thumbs-size-option = { $px } px
settings-thumbs-quality = Kvalitet { $quality }.
settings-thumbs-small = Mindre
settings-thumbs-small-hint = Fint i gitteret; lidt blødt tæt på.
settings-thumbs-balanced = Balanceret
settings-thumbs-balanced-hint = Svær at skelne fra originalen i miniaturestørrelse.
settings-thumbs-sharp = Skarpere
settings-thumbs-sharp-hint = Omtrent dobbelt så stor som Balanceret.
settings-motion-thumbs = Levende miniaturer for animationer
settings-motion-thumbs-what = Hver animation får også en kort film i loop, der afspilles, mens du peger på dens felt. De fylder omkring 40 KB hver og tager et stykke tid at lave for et stort bibliotek. Slår du det fra, slettes de.
settings-motion-fps = Billedhastighed
settings-motion-fps-option = { $fps } fps
settings-motion-fps-what = Glattere film tager længere tid at lave og fylder mere: omkring dobbelt så meget ved 24 fps som ved 12, og endnu mere ved 30. Ændrer du den, laves de alle igen.
settings-limits = Arbejde og grænser
settings-limits-what = Hver af dem gemmes, når du forlader feltet eller trykker på Enter. Størrelser tager enheder: 100 GB, 512 MB, 1,5 TB; et tal alene er GB.
settings-workers = Baggrundsarbejdere
settings-workers-unit = job ad gangen (indlæsning, tagging, konverteringer)
settings-max-upload = Største upload
settings-max-upload-unit = pr. fil
settings-max-expand = Største pakke
settings-max-expand-unit = når den er pakket ud
settings-conversion-cache = Konverterede downloads
settings-conversion-cache-unit-now = højst beholdt, { $size } nu; de mindst brugte ryger først
settings-conversion-cache-unit = højst beholdt; de mindst brugte ryger først

## SettingsPage.vue: Vedligeholdelse

settings-updates = Opdateringer
settings-version = Dette er Hoard { $version }.
settings-update-check = Tjek GitHub for en ny udgivelse én gang om dagen
settings-desktop-updates = Hoard leder efter en ny version, hver gang det starter, henter den i baggrunden og installerer den, når du afslutter.
settings-backups = Planlagte sikkerhedskopier
settings-backups-what = Databasen tager selv sikkerhedskopier; hver kopi vises i historikken nedenfor.
settings-backup-every = Tag kopi
settings-backup-off = Aldrig
settings-backup-daily = Hver dag
settings-backup-weekly = Hver uge
settings-backup-keep = Behold
settings-backup-keep-unit = nyeste planlagte kopier
settings-backup-copy-to = Kopiér også til
settings-backup-copy-to-none = Ingen anden mappe
settings-backups-kept = Hver kopi lægges i backupmappen som hoard-auto-…db, og i den anden mappe (en anden disk, eller en som en synkroniseringsapp sikkerhedskopierer), hvis der er en, angivet med dens fulde sti på serveren. Kun ældre planlagte kopier fjernes; dem, der er lavet manuelt, bliver.
about-what = Hoard samler spilaktiver (modeller, teksturer, lyde, musik, sprites og mere) i ét bibliotek, du kan søge i, forhåndsvise og tage fra, delt med alle, du giver adgang.
about-version = Version
about-edition = Udgave
about-edition-server = Server
about-edition-desktop = Skrivebordsapp
about-edition-steam = Skrivebordsapp fra Steam
about-made-by = Lavet af
about-source = Kildekode på GitHub
about-issues = Rapportér et problem
about-discord = Chat med os på Discord
about-steam = Hoard på Steam
about-license = Licens
about-license-what = Hoard er open source under MIT No Attribution-licensen: brug den, ret i den og del den, som du vil.
about-downloaded = Hentes efter behov
about-downloaded-what = Hoard følger ikke med disse. Den henter hver enkelt første gang, noget har brug for den, og hver har sin egen licens.
about-blender = Konverterer 3D-modeller og laver deres miniaturer.
about-ffmpeg = Konverterer lyd og video og laver deres forhåndsvisninger.
about-siglip = Finder billeder ud fra, hvad der er på dem.
about-clap = Finder lyde ud fra, hvordan de lyder.
about-models = Modeller, du vælger til den lokale AI-server, er underlagt deres skaberes licenser.
about-built-with = Bygget med
about-built-with-what = De open source-biblioteker, Hoard er lavet af, med deres licenser.
about-steamworks = Steam-udgaven indeholder også Steamworks SDK, © Valve Corporation.
about-licenses-loading = Indlæser listen…
about-licenses-failed = Listen kunne ikke indlæses.
about-filter = Filtrér efter navn eller licens
about-count = { $shown } af { $total }
about-no-license = Ingen licens angivet
about-report = Rapportér et problem…
report-title = Rapportér et problem
report-what = Det her er, hvad et issue på GitHub skal bruge, for at problemet kan undersøges. Der er intet privat i det: adgangstokens, adgangskoder, nøgler, e-mailadresser og dit navn i mappestier er fjernet. Læs det igennem, før du sender det.
report-loading = Samler rapporten…
report-failed = Rapporten kunne ikke samles: { $error }
report-log = Hvad serveren har logget for nylig
report-copy = Kopiér
report-copied = Rapporten er kopieret.
report-github = Opret et issue på GitHub
report-github-hint = Det nye issue udfyldes med dette. Er loggen for lang til det, kommer kun de sidste linjer med; Kopiér har det hele.
report-close = Luk

## Asset kinds (api.ts)

api-kind-texture = Teksturer
api-kind-vector = Vektorgrafik
api-kind-sheet = Sprite-ark
api-kind-audio = Lyd
api-kind-model = Modeller
api-kind-animation = Animationer
api-kind-font = Skrifttyper
api-kind-shader = Shadere
api-kind-material = Materialer
api-kind-video = Video
api-kind-document = Dokumenter
api-kind-data = Data
api-kind-other = Andet
api-kind-texture-name = teksturer
api-kind-vector-name = vektorgrafik
api-kind-sheet-name = sprite-ark
api-kind-audio-name = lyd
api-kind-model-name = modeller
api-kind-animation-name = animationer
api-kind-font-name = skrifttyper
api-kind-shader-name = shadere
api-kind-material-name = materialer
api-kind-video-name = video
api-kind-document-name = dokumenter
api-kind-data-name = data
api-kind-other-name = andet

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

news-button =
    { $count ->
        [one] { $count } ny
       *[other] { $count } nye
    }
news-button-title =
    { $count ->
        [one] { $count } ny ting i Hoard
       *[other] { $count } nye ting i Hoard
    }
news-pip =
    { $count ->
        [one] Nyt i Hoard
       *[other] { $count } nye ting her
    }
news-title = Nyt i Hoard
news-to-find =
    { $count ->
        [one] { $count } at finde
       *[other] { $count } at finde
    }
news-close = Luk
news-got-it = Forstået
news-take-me = Tag mig derhen
news-look-again = Se igen
news-none = Der er intet nyt at finde lige nu.
news-seen-lately = Set for nylig
news-stop-pointing = Stop med at gøre opmærksom på nye ting
news-point-out = Gør opmærksom på nye ting igen
news-mark-all = Markér alt som set
news-marked-seen =
    { $count ->
        [one] { $count } ting markeret som set
       *[other] { $count } ting markeret som set
    }
news-themes-title = Ti lyse og ti mørke temaer
news-themes-body = Himmel, Mynte, Abrikos og Citron slutter sig til de lyse temaer og Midnat, Lagune og Neon til de mørke, og det nye udseende breder sig ud fra det felt, du vælger.
news-tiles-move-title = Hold felterne i bevægelse
news-tiles-move-body = Animationer og animerede billeder på bibliotekets felter kan afspilles hele tiden, ikke kun under markøren.
news-motion-trails-title = Bevægelsesspor på animationer
news-motion-trails-body = En animations miniature tegner den vej, figuren tager, med en prik der, hvor den er lige nu.
news-text-size-title = Tekststørrelse og afstand
news-text-size-body = Gør skriften større eller mindre og siden mere luftig eller mere kompakt, og vælg, hvad felterne og topbjælken viser.
news-tag-tree-title = Tags inden i tags
news-tag-tree-body = Sæt et tag under et bredere, så en søgning på køretøj også finder bil og sedan. Indlejr dem under Indstillinger → Tags eller på et tags egen side.
news-bulk-edit-title = Rediger mange på én gang
news-bulk-edit-body = Vælg nogle assets, og tryk på E for at ændre deres tags, licens, vurdering og mere på én gang, med Fortryd.
news-backups-title = Planlagte sikkerhedskopier
news-backups-body = Sikkerhedskopiér databasen hver dag eller hver uge, behold de nyeste og kopiér hver enkelt til en anden mappe.
ui-undo = Fortryd
ui-today = i dag
ui-yesterday = i går
ui-days-ago =
    { $days ->
        [one] for { $days } dag siden
       *[other] for { $days } dage siden
    }
ui-notifications = Notifikationer
ui-dismiss = Afvis
ui-cancel = Annullér
ui-lazy-loading = Indlæser { $what }…
ui-lazy-failed = Kunne ikke indlæse { $what }.
ui-lazy-reload = Genindlæs siden

## ResetButton.vue

reset-title = Nulstil til standard: { $value }
reset-label = Nulstil til standard, { $value }

## SizeInput.vue

size-invalid = En størrelse som 100 GB, 512 MB eller 1,5 TB
size-hint = En størrelse som 100 GB, 512 MB eller 1,5 TB; et tal alene er GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = Kunne ikke indlæse godkendelsesindstillingerne: { $error }
auth-saved = Gemt. Det gælder fra næste login.
auth-method-internal = Intern
auth-method-internal-hint = Navne og adgangskoder, som Hoard opbevarer, tilføjet under Indstillinger → Konto.
auth-method-ldap-hint = Et katalog kontrollerer det navn og den adgangskode, der skrives på login-siden: OpenLDAP, FreeIPA, LLDAP eller en anden LDAP-server.
auth-method-ad-hint = Windows-domænekonti logger ind med deres sædvanlige navn og adgangskode.
auth-method-oauth-hint = En »Log ind med…«-knap, der går til Google, Microsoft, GitHub, Keycloak, Authentik eller en anden udbyder og tilbage igen.
auth-session-unused = Efter 30 dage uden brug
auth-session-1h = 1 time efter login
auth-session-8h = 8 timer efter login
auth-session-day = En dag efter login
auth-session-week = En uge efter login
auth-session-30d = 30 dage efter login
auth-session-hours =
    { $hours ->
        [one] { $hours } time efter login
       *[other] { $hours } timer efter login
    }
auth-role-view = Kan se
auth-role-edit = Kan ændre
auth-role-admin = Administrator
auth-role-word-view = kan se
auth-role-word-edit = kan ændre
auth-role-word-admin = administrator
auth-role-word-default = { $role } (standardrollen)
auth-tls-starttls-hint = Almindelig LDAP (port 389), der skifter til TLS, før noget sendes.
auth-tls-ldaps-hint = TLS fra starten (port 636).
auth-tls-none = Ingen
auth-tls-none-hint = Adgangskoder sendes over netværket, som de er. Kun på et netværk, du stoler på.
auth-test-failed = Kunne ikke teste: { $error }
auth-preset-custom = Andet (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel og andre: angiv udstederen, så udfylder Find adresserne.
auth-preset-google-note = Opret en OAuth-klient (webapplikation) i Google Cloud Console → APIs & Services → Credentials. Google sender ingen grupper, så tildel roller under Indstillinger → Konto, og slå oprettelse af konti ved første login fra, medmindre alle med en Google-konto skal kunne komme ind.
auth-preset-microsoft-note = Registrér en app i Entra ID → App registrations (en web-omdirigering til adressen nedenfor). Skriv dit tenant-id i stedet for »organizations« for kun at lukke din organisation ind. For at få roller fra grupper skal du tilføje groups-claimet under Token configuration: det angiver gruppe-id'er.
auth-preset-github-note = Opret en OAuth-app i GitHub → Settings → Developer settings. GitHub sender ingen grupper, så tildel roller under Indstillinger → Konto, og slå oprettelse af konti ved første login fra, medmindre alle med en GitHub-konto skal kunne komme ind.
auth-preset-gitlab-note = Opret en applikation i GitLab → Preferences → Applications (eller i din instans' administrationsområde: ret gitlab.com i adresserne).
auth-discovered = Udfyldte udbyderens adresser.
auth-copied = Kopieret
auth-copy-failed = Kunne ikke kopiere: markér og kopiér det selv
auth-sign-in-with = Log ind med
auth-own-accounts-still-work = Hoards egne konti logger stadig ind med deres egne adgangskoder, så et katalog eller en udbyder, der er nede eller sat forkert op, ikke kan lukke dig ude.
auth-sessions = Sessioner
auth-sign-out-after = Log folk ud
auth-ldap-server = LDAP-server
auth-server-type = Servertype
auth-server-type-custom = Tilpasset
auth-servers = Servere
auth-server-n = Server { $n }
auth-remove-server-n = Fjern server { $n }
auth-remove = Fjern
auth-add-server = Tilføj en server
auth-servers-hint = Den næste prøves, når en ikke kan nås.
auth-security = Sikkerhed
auth-connection-security = Forbindelsessikkerhed
auth-skip-verify = Kontrollér ikke serverens certifikat (kun til afprøvning)
auth-ca = Certifikatudsteder
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (tom: dem, systemet stoler på)
auth-lookup = Opslag af personer
auth-anonymously = Anonymt
auth-reader-account = Læsekonto
auth-reader-password = Dens adgangskode
auth-saved-type-to-replace = Gemt: skriv for at erstatte
auth-people-under = Personer findes under
auth-name-attribute = Navneattribut
auth-only-matching = Kun poster, der matcher
auth-groups-under = Grupper findes under
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (tom: kun memberOf)
auth-their-groups = Deres grupper
auth-group-filter-hint = { "{" }dn{ "}" } er personens post, { "{" }user{ "}" } det navn, vedkommende loggede ind med.
auth-test = Test
auth-test-hint = Kontakter hver server og logger ind som læsekontoen. Med et navn og en adgangskode logges der også ind som dem, og du ser, hvad Hoard ville få ud af det. Intet gemmes.
auth-test-user = En at logge ind som
auth-test-user-placeholder = En at logge ind som (valgfri)
auth-test-password = Vedkommendes adgangskode
auth-testing = Tester…
auth-test-connectivity = Test forbindelsen
auth-test-server-ok = nået, og læsekontoen loggede ind
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }loggede ind (
auth-test-signed-in-after = ).
auth-test-role = Vedkommende ville få: { $role }.
auth-test-no-role = Vedkommende er ikke i nogen af grupperne under Roller og ville derfor blive afvist.
auth-test-groups = Grupper: { $groups }
auth-test-no-groups = Ingen grupper fundet.
auth-provider = Udbyder
auth-redirect = Omdirigeringsadresse
auth-copy = Kopiér
auth-redirect-hint = Giv udbyderen denne, når du opretter Hoards klient. Den kommer fra den offentlige adresse under Indstillinger → Forbindelse eller fra den adresse, du bruger.
auth-issuer = Udsteder
auth-discover = Find
auth-client-id = Klient-id
auth-client-secret = Klienthemmelighed
auth-authorization-url = Godkendelsesadresse
auth-token-url = Token-adresse
auth-userinfo-url = Adresse til brugeroplysninger
auth-userinfo-placeholder = (tom: ID-tokenet siger, hvem det er)
auth-scopes = Scopes
auth-name-from = Navn fra
auth-groups-from = Grupper fra
auth-groups-from-placeholder = (ingen)
auth-button = Knap
auth-button-placeholder = din konto
auth-accounts = Konti
auth-provision = Opret en Hoard-konto, første gang nogen logger ind
auth-provision-on-hint = Ellers tilføjer en administrator hver person først under Indstillinger → Konto.
auth-provision-off-hint = En administrator tilføjer hver person først under Indstillinger → Konto og vælger, hvordan de logger ind.
auth-open-to-anyone = Alle med en { $provider }-konto kan komme ind. Tilføj fremvisergrupper nedenfor, eller slå dette fra, og tilføj selv folk.
auth-new-accounts = Nye konti
auth-roles-from-groups = Roller fra grupper
auth-roles-hint = Grupper, hvis medlemmer får hver rolle, én pr. linje, efter navn eller DN. Den højeste, der matcher, vinder, og det kontrolleres ved hvert login. Hvis der ikke står nogen, får nye konti rollen ovenfor, og en administrator ændrer den bagefter. Hvis der står grupper under Kan se, kan kun medlemmer af en af dem logge ind.
auth-roles-hint-oauth = Grupper, hvis medlemmer får hver rolle, én pr. linje, efter navn eller DN (eller hvad groups-claimet nu angiver). Den højeste, der matcher, vinder, og det kontrolleres ved hvert login. Hvis der ikke står nogen, får nye konti rollen ovenfor, og en administrator ændrer den bagefter. Hvis der står grupper under Kan se, kan kun medlemmer af en af dem logge ind.
auth-saving = Gemmer…
auth-save = Gem

## HttpsPanel.vue (Settings → Connection)

https-load-failed = Kunne ikke indlæse HTTPS: { $error }
https-off-redirecting = HTTPS er slået fra. Sender dig til http://
https-now-on = HTTPS er slået til
https-saved = Gemt. Nye forbindelser bruger det nye certifikat.
https-now-off = HTTPS er slået fra
https-http3-failed-toast = HTTP/3 kunne ikke starte: { $error }
https-copied = Kopieret
https-copy-failed = Kunne ikke kopiere: markér og kopiér det selv
https-mode-off = Fra
https-mode-off-hint = Kun almindelig http://.
https-mode-own = Hoards eget certifikat
https-mode-own-hint = Hoard laver et til denne servers adresser. Installér Hoards certifikatudsteder én gang på hver enhed, så stoler dens browsere på det.
https-mode-custom = Mit certifikat
https-mode-custom-hint = Et, du allerede har, f.eks. fra tailscale cert eller Let’s Encrypt. Browsere stoler på det uden at installere noget.
https-untrusted-own = Denne browser stoler endnu ikke på det for { $host }: installér Hoards certifikatudsteder nedenfor. Du får en advarsel, som du kan klikke dig forbi.
https-untrusted-uncovered = Denne browser stoler endnu ikke på det for { $host }: certifikatet dækker ikke denne adresse. Du får en advarsel, som du kan klikke dig forbi.
https-untrusted = Denne browser stoler endnu ikke på det for { $host }. Du får en advarsel, som du kan klikke dig forbi.
https-now = Nu
https-start-failed = HTTPS er sat op, men kunne ikke starte: { $error }
https-on-port = HTTPS er slået til på port { $port }, ligesom http://.
https-using-it = Du bruger det nu.
https-open = Åbn { $url }
https-trusted = Denne browser stoler på det.
https-cert-for-before = Certifikat for{ " " }
https-cert-for-after = , gyldigt til { $date }.
https-cert-for-after-own = , gyldigt til { $date } (fornys af sig selv).
https-not-covered-before = Det dækker ikke{ " " }
https-not-covered-after = , den adresse, du bruger.
https-is-off = HTTPS er slået fra: sider sendes over almindelig http://.
https-addresses = Adresser
https-addresses-hint = Certifikatet dækker de adresser, Hoard fandt, og dem, du tilføjer (et navn, din router giver den, et Tailscale-navn).
https-remove-name = Fjern { $name }
https-another-address = En anden adresse
https-another-address-placeholder = En anden adresse, f.eks. nas.local
https-add = Tilføj
https-your-cert = Dit certifikat
https-saved-cert-before = Gemt:{ " " }
https-saved-cert-after = { " " }fra { $issuer }, gyldigt til { $date }. Indsæt et nyt for at erstatte det.
https-pem-before = Certifikatet (med sin kæde) og dets private nøgle som PEM-tekst: de filer, der starter med{ " " }
https-pem-after = .
https-certificate = Certifikat
https-choose-file = Vælg fil…
https-private-key = Privat nøgle
https-checked-before = For{ " " }
https-checked-after = , fra { $issuer }, gyldigt til { $date }.
https-checked-after-days =
    , fra { $issuer }, gyldigt til { $date } ({ $days ->
        [one] { $days } dag
       *[other] { $days } dage
    }).
https-self-signed = Det er selvsigneret: browsere vil advare om det.
https-key-matches = Nøglen passer.
https-where-to-get = Hvor du får et
https-get-tailscale-before = slå HTTPS til for dit tailnet, og kør derefter{ " " }
https-get-tailscale-after = { " " }på serveren, og upload de to filer. De holder i 90 dage: kør det igen, og upload dem igen inden da.
https-get-acme-title = Let’s Encrypt eller en anden ACME-klient
https-get-acme-before = (certbot, acme.sh, lego) til et domæne, du ejer: upload{ " " }
https-get-acme-and = { " " }og{ " " }
https-get-acme-after = .
https-get-proxy-title = En reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), der har certifikatet: lad HTTPS være slået fra her, og lad proxyen sende almindelig http:// til Hoard.
https-trying = Prøver det…
https-apply = Anvend
https-apply-hint = Hoard prøver den nye opsætning først og beholder den nuværende, hvis den ikke virker.
https-trust-title = Stol på Hoard på dine enheder
https-download-ca = Download hoard-ca.crt
https-trust-hint = Installér Hoards certifikatudsteder én gang på hver enhed, så stoler dens browsere på denne server (og på ethvert certifikat, Hoard laver til den senere). Den kan kun gå i god for denne Hoard. Fingeraftryk (SHA-256):
https-ca-windows = Åbn filen → Installer certifikat → Aktuel bruger → Placer alle certifikater i følgende lager → Gennemse → Rodnøglecentre, der er tillid til → Udfør. Genstart browseren.
https-ca-macos = Åbn filen; den havner i Nøglering. Dobbeltklik på »{ $subject }« der → Tillid → Ved brug af dette certifikat: Hav altid tillid.
https-ca-ios-title = iPhone og iPad
https-ca-ios = Download den i Safari, og tillad den. Indstillinger → Profil hentet → Installer. Derefter Indstillinger → Generelt → Om → Indstillinger for certifikattillid → slå »{ $subject }« til.
https-ca-android = Indstillinger → Sikkerhed → Flere sikkerhedsindstillinger → Kryptering og loginoplysninger → Installer et certifikat → CA-certifikat, og vælg filen. (Navnene varierer lidt fra telefon til telefon; søg efter »certifikat« i Indstillinger.)
https-ca-linux-1 = Debian og Ubuntu:{ " " }
https-ca-linux-2 = . Fedora og Arch:{ " " }
https-ca-linux-3 = . Chrome på Linux har sin egen liste: Indstillinger → Privatliv og sikkerhed → Sikkerhed → Administrer certifikater → Importér.
https-ca-firefox = Firefox har sin egen liste: Indstillinger → Privatliv & sikkerhed → Certifikater → Vis certifikater → Autoriteter → Importer, og sæt flueben ved »Stol på denne CA ved identificering af websteder«.
https-ca-cli-title = Kommandolinjeværktøjet hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , eller angiv stien i{ " " }
https-ca-cli-after = .
https-options = Indstillinger
https-redirect = Send browsere på andre enheder fra http:// til https://
https-redirect-hint = Kun sideindlæsninger sendes videre. Kommandolinjeværktøjet hoard, Unreal-workeren, scripts og AI-apps bliver ved med at virke på http://, og det gør http://localhost på selve serveren også.
https-http3-hint-before = Hurtigere på ustabilt Wi-Fi og på telefoner. Kræver HTTPS og UDP-port { $port } åben ud over TCP (i Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Kører.
https-http3-waits = Det starter, når HTTPS er slået til.
https-http3-failed = HTTP/3 kunne ikke starte: { $error }
https-lockout-title = Hvis du bliver lukket ude
https-lockout-1 = { "" }
https-lockout-2 = { " " }på selve serveren virker altid over almindelig http://. Eller slå HTTPS fra på serverens kommandolinje, og start Hoard igen:{ " " }
https-lockout-3 = { " " }(i Docker:{ " " }
https-lockout-4 = , derefter{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Kan se
account-role-view-hint = Søge, se forhåndsvisninger og downloade
account-role-edit = Kan ændre
account-role-edit-hint = Også tilføje, ændre, tagge, samle og arkivere
account-role-admin = Administrator
account-role-admin-hint = Også Indstillinger, konti og vedligeholdelse samt sletning for altid
account-source-internal = En Hoard-adgangskode
account-source-ldap = Vedkommendes { $directory }-adgangskode
account-source-oauth = Login med { $provider }
account-source-word-directory = katalog
account-load-failed = Kunne ikke indlæse konti: { $error }
account-failed = { $what }: { $error }
account-password-failed = Kunne ikke ændre din adgangskode
account-password-changed = Adgangskoden er ændret. Dine andre enheder er logget ud.
account-add-failed = Kunne ikke tilføje { $name }
account-added = { $name } kan logge ind nu.
account-role-failed = Kunne ikke ændre, hvad { $name } kan
account-remove-title = Fjern { $name }?
account-remove-body = Vedkommende logges ud overalt og kan ikke logge ind igen. Intet af det, vedkommende har tilføjet til biblioteket, fjernes.
account-remove = Fjern
account-remove-failed = Kunne ikke fjerne { $name }
account-token-failed = Kunne ikke lave tokenet
account-copied = Kopieret
account-copy-token-failed = Kunne ikke kopiere: markér tokenet, og kopiér det selv
account-copy-failed = Kunne ikke kopiere: markér og kopiér det selv
account-revoke-title = Tilbagekald »{ $name }«?
account-revoke-body = Alt, der stadig bruger det, bliver afvist fra nu af.
account-revoke = Tilbagekald
account-revoke-failed = Kunne ikke tilbagekalde »{ $name }«
account-disconnect-title = Afbryd forbindelsen til { $app }?
account-disconnect-body = Den bliver afvist fra nu af og skal forbindes igen for at bruge Hoard.
account-disconnect = Afbryd forbindelsen
account-disconnect-failed = Kunne ikke afbryde forbindelsen til { $app }
account-guest-failed = Kunne ikke ændre det
account-never = aldrig
account-you = Dig
account-sign-out = Log ud
account-signed-in-as-before = Logget ind som{ " " }
account-signed-in-as-admin = , som er administrator.
account-signed-in-as-edit = , som kan ændre biblioteket.
account-signed-in-as-view = , som kan se, men ikke ændre noget.
account-app-signin = Hoard-appen logger dig selv ind, så der er ingen adgangskode. Opret et adgangstoken herunder til motorernes plugins og hoard-kommandoen.
account-password-at-provider = Din adgangskode opbevares af den udbyder, du logger ind med, så du ændrer den der.
account-password-at-directory = Din adgangskode opbevares af det katalog, du logger ind med, så du ændrer den der.
account-current-password = Nuværende adgangskode
account-new-password = Ny adgangskode
account-new-password-again = Ny adgangskode igen
account-change-password = Skift adgangskode
account-password-mismatch = De nye adgangskoder er ikke ens.
account-password-rule = Mindst 8 tegn.
account-people = Personer
account-people-hint = Hver person kan se (søge, se forhåndsvisninger og downloade), også ændre biblioteket (tilføje, redigere, tagge, samle og arkivere) eller desuden være administrator (Indstillinger, konti og vedligeholdelse samt sletning for altid).
account-you-tag = (dig)
account-signs-in-oauth = Logger ind med OAuth-udbyderen
account-signs-in-directory = Logger ind med kataloget
account-since = siden { $date }
account-what-can-do = Hvad { $name } kan
account-name = Navn
account-how-sign-in = Hvordan vedkommende logger ind
account-password-label = Adgangskode (mindst 8 tegn)
account-password-placeholder = Adgangskode (8+ tegn)
account-what-they-can-do = Hvad vedkommende kan
account-add = Tilføj
account-ai-apps = Claude og andre AI-apps
account-ai-apps-hint = Forbind Claude (eller enhver MCP-app), så kan den søge efter og hente assets for dig og beskrive og tagge det, Hoard ikke har nået endnu. Den logger ind på Hoard første gang; du giver den lov her i browseren.
account-mcp-address = Hoards MCP-adresse
account-copy = Kopiér
account-claude-code-1 = kør{ " " }
account-claude-code-2 = , og derefter{ " " }
account-claude-code-3 = { " " }for at logge ind.{ " " }
account-claude-code-4 = { " " }beskriver en pakke.
account-claude-ai = Settings → Connectors → Add custom connector, indsæt adressen, og klik derefter på Connect. De forbinder fra Anthropics servere, så Hoard skal kunne nås fra internettet over HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, indsæt adressen, og klik derefter på Connect. De forbinder fra Anthropics servere, så Hoard skal kunne nås fra internettet over HTTPS (det kan denne adresse ikke: angiv den offentlige adresse under Indstillinger → Forbindelse).
account-connection-meta = for { $user } siden { $since } · sidst brugt { $used }
account-no-apps = Ingen apps forbundet endnu.
account-live = Bruger Hoard nu
account-live-hint = Apps, der har spurgt Hoard om noget inden for de seneste 10 minutter. Den holder sig selv opdateret.
account-live-none = Intet har spurgt Hoard om noget inden for de seneste 10 minutter.
account-live-last = senest: { $what } · { $when } · forespørgsler: { $calls }
account-tokens = Adgangstokens
account-tokens-hint-1 = Til kommandolinjeværktøjet{ " " }
account-tokens-hint-2 = , scripts, MCP-klienter og Unreal-workeren: de sender det som{ " " }
account-tokens-hint-3 = . Et token kan det, du vælger, op til det, du selv kan, men aldrig administrere konti eller tokens. Et, der kun kan se, er nok til at søge og downloade og til at synkronisere et projekt.
account-tokens-admin = Du ser alles tokens og forbundne apps; andre ser deres egne.
account-made-before = { "" }
account-made-after = : kopiér det nu. Det vises ikke igen.
account-new-token = Det nye token
account-cli-1 = Til CLI'en:{ " " }
account-cli-2 = , eller læg det i{ " " }
account-cli-3 = .
account-done = Færdig
account-token-meta = lavet { $made } · sidst brugt { $used }
account-token-meta-by = lavet { $made } af { $by } · sidst brugt { $used }
account-no-tokens = Ingen tokens endnu.
account-token-name = Hvad der skal bruge det
account-token-name-placeholder = Hvad der skal bruge det, f.eks. »Unreal-worker«
account-token-role = Hvad det kan
account-make-token = Lav et token
account-guests = Gæster
account-guests-allow = Lad alle, der kan nå serveren, gennemse uden at logge ind
account-guests-hint = De kan søge, se forhåndsvisninger og downloade, men ikke tilføje, ændre eller slette noget eller se Indstillinger.

## SignIn.vue

signin-create-title = Opret din konto
signin-title = Log ind
signin-setup-lede = Denne server har ikke nogen konto endnu. Den, du opretter her, kan alt; tilføj konti til andre senere under Indstillinger → Konto.
signin-overlay-lede = Alle kan gennemse her, men ændringer kræver en konto.
signin-with = Log ind med { $provider }
signin-or-password = eller med navn og adgangskode
signin-username = Brugernavn
signin-name = Navn
signin-new-password = Ny adgangskode
signin-password = Adgangskode
signin-password-rule = Mindst 8 tegn.
signin-confirm-password = Bekræft adgangskode
signin-mismatch = De to adgangskoder er ikke ens.
signin-setup-code = Opsætningskode
signin-setup-code-hint-before = Den kode, serveren fik i{ " " }
signin-setup-code-hint-after = .
signin-creating = Opretter…
signin-signing-in = Logger ind…
signin-create = Opret konto
signin-submit = Log ind
signin-keep-browsing = Fortsæt med at gennemse
signin-directory-hint = Brug dit { $directory }-navn og din adgangskode.
signin-forgot-1 = Har du glemt en Hoard-adgangskode? Kør{ " " }
signin-forgot-2 = { " " }på serveren (i Docker{ " " }
signin-forgot-3 = ) for at få en ny.

## SetupWizard.vue (first visit to a new server)

setup-title = Hvor skal dit bibliotek ligge?
setup-lede = Vælg en mappe på serveren, som Hoard skal opbevare dine pakker i, eller peg den på en mappe med pakker, du allerede har. Du kan senere tilføje flere placeringer, også på andre diske, under Indstillinger.
setup-lede-desktop = Vælg en mappe, som Hoard skal opbevare dine pakker i, eller peg den på en mappe med pakker, du allerede har. Du kan senere tilføje flere placeringer, også på andre diske, under Indstillinger.
setup-unreachable = Kunne ikke nå serveren: { $error }
setup-unreachable-desktop = Hoard svarer ikke: { $error }
setup-try-again = Prøv igen
setup-loading = Indlæser…
setup-start = Start biblioteket
setup-ready = Dit bibliotek er klar
setup-managed-before = Nye pakker lægges i{ " " }
setup-managed-after = . Slip filer, mapper eller arkiver hvor som helst på siden for at tilføje de første.
setup-watched-before = Hoard overvåger{ " " }
setup-watched-after = ; pakker dukker op, efterhånden som den finder dem. Hvis du også vil uploade, kan du tilføje et bibliotek, som Hoard opbevarer, under Indstillinger → Biblioteksplaceringer.
setup-open = Åbn biblioteket

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = Forbind { $app }?
connect-lede-1 = { " " }vil kunne søge i, downloade, beskrive og tagge alt i denne Hoard og handle som{ " " }
connect-lede-2 = . Den kan ikke administrere konti eller adgangstokens.
connect-back-before = Du sendes tilbage til{ " " }
connect-back-after = . Afbryd forbindelsen når som helst under Indstillinger → Konto.
connect-allow = Tillad
connect-cancel = Annullér
connect-failed = Kan ikke forbinde denne app
connect-go-library = Gå til biblioteket
connect-checking = Kontrollerer…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = Dette er almindelig http://, så nogle ting kræver HTTPS: 3D med WebGPU, den moderne udklipsholder, installation af Hoard som app og Claude-connectors.
insecure-switch = Skift til https://
insecure-set-up = Konfigurer HTTPS
insecure-dismiss = Vis ikke dette igen

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Denne fil er for stor til at blive læst som en butiks fil.
formatwizard-not-json = Denne fil er ikke gyldig JSON, så den vises som tekst.
formatwizard-read-failed = Kunne ikke læse filen: { $error }
formatwizard-back-title = Gå tilbage til formularen?
formatwizard-back-body = Ændringer i TOML'en går tabt: formularen skriver formatet igen ud fra det, den har.
formatwizard-back-to-form = Tilbage til formularen
formatwizard-miss-name = Dets filer tager ikke navnet { $file }. Tilføj det under Filer.
formatwizard-miss-not-the-stores = Det, formatet genkender butikkens filer på, findes ikke i denne fil. Ret Genkend på.
formatwizard-miss-no-title = Det finder ingen titel i denne fil. Vælg titlen.
formatwizard-miss-short-name = ikke dets navn
formatwizard-miss-short-not-the-stores = ikke butikkens
formatwizard-miss-short-no-title = ingen titel
formatwizard-saved-packs =
    Gemte { $file }. { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    } læses igen.
formatwizard-saved = Gemte { $file }.
formatwizard-save-failed = Kunne ikke gemme formatet: { $error }
formatwizard-zip-failed = Kunne ikke lave zip-filen: { $error }
formatwizard-stop-title = Stop med at lære denne fil?
formatwizard-stop-body = Formatet er ikke gemt.
formatwizard-stop = Stop
formatwizard-label = Formatguide
formatwizard-changing = Ændrer { $name }
formatwizard-starting-from = Tager udgangspunkt i { $name }
formatwizard-title = Lær Hoard en butiks fil
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }i{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Redigér som TOML
formatwizard-file-reference = Filen, til reference.
formatwizard-click-json = Klik på { $field } i filen.
formatwizard-click-text = Klik på { $field } i filen, eller markér teksten.
formatwizard-pick-first = Vælg et felt til højre, og klik derefter på dets værdi her.
formatwizard-loading = Indlæser…
formatwizard-the-format = Formatet
formatwizard-syntax-before = Syntaksen står i{ " " }
formatwizard-syntax-after = .
formatwizard-store = Butik
formatwizard-store-placeholder = Butikkens navn, f.eks. Kenney
formatwizard-files = Filer
formatwizard-recognize = Genkend på
formatwizard-recognize-json = En nøgle, som kun butikkens filer har
formatwizard-recognize-text = En linje, som kun butikkens filer har
formatwizard-recognize-hint = Én mulighed pr. linje; enhver af dem er nok. Kræves, når andre filer har samme navn.
formatwizard-seller = Sælger
formatwizard-seller-placeholder = Når filen ikke siger det
formatwizard-tags = Tags
formatwizard-tags-placeholder = Til alt, den beskriver, adskilt med kommaer
formatwizard-fields = Felter
formatwizard-selector = Selektor for { $field }
formatwizard-clear = Ryd { $field }
formatwizard-pick = Vælg
formatwizard-what-it-reads = Hvad det læser
formatwizard-read-title = Titel
formatwizard-read-seller = Sælger
formatwizard-read-license = Licens
formatwizard-read-url = Butiksside
formatwizard-read-kind = Type
formatwizard-read-tags = Tags
formatwizard-read-origin = Lavet i
formatwizard-read-size = Størrelse
formatwizard-also = { $format } læser også denne fil. Dette format ville blive prøvet først.
formatwizard-library =
    { $files ->
        [one] { $files } fil
       *[other] { $files } filer
    } i { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    } har et navn, det tager. Af { $tried } afprøvede læser det { $read }.
formatwizard-library-none = Ingen fil i biblioteket har et navn, det tager.
formatwizard-share = Del det
formatwizard-share-hint = En zip-fil med formatet, denne fil som eksempel og det, eksemplet læses som, organiseret som Hoards repository, som du kan indsende, så alle får det. Fjern først alt i eksemplet, der ikke skal deles.
formatwizard-sample = Eksempel til deling
formatwizard-download = Download { $file }
formatwizard-close = Luk
formatwizard-use = Brug det her
formatwizard-cancel = Annullér
settings-unreal-editor-what = Installerer Unreal Editor til eksportprocessen. Kør processen separat for at eksportere aktiver.
settings-unreal-download = Downloader Epics Linux x86-64-editor én gang til Hoards værktøjsmappe. Den fylder flere titals GB, og kræver en GitHub-konto, som Epic har lukket ind.
settings-unreal-github-step-link-go = Sådan knytter du dem sammen
settings-unreal-github-step-token = Opret et GitHub-token (classic), hvor kun read:packages er markeret.
settings-unreal-github-step-token-go = Opret et token
settings-unreal-github-user = GitHub-brugernavn
settings-unreal-github-token = Token
settings-unreal-github-token-kept = Gemt (skriv et nyt for at erstatte det)
settings-unreal-github-save = Gem login
settings-unreal-github-forget = Afbryd forbindelsen
settings-unreal-github-saved = GitHub-login gemt; prøver at hente igen
settings-unreal-github-forgotten = GitHub afbrudt
settings-unreal-github-from-server = Denne server har også fået sit eget login (GHCR_TOKEN_FILE eller DOCKER_CONFIG); det bruges, når der ikke er gemt et her.
settings-unreal-github-connect = Forbind GitHub…
settings-unreal-github-connect-title = Forbind GitHub
settings-unreal-github-connect-why = Epic deler Unreal Editor med GitHub-konti, der er knyttet til Epic Games. Hoard beder kun om at læse pakker.
settings-unreal-github-sign-in = Log ind med GitHub
settings-unreal-github-enter-code = Når du klikker, åbner GitHub i en ny fane. Indtast koden der.
settings-unreal-github-waiting = Venter på GitHub…
settings-unreal-github-expired = Koden er udløbet.
settings-unreal-github-denied = Login blev annulleret på GitHub.
settings-unreal-github-failed = Kunne ikke få en kode fra GitHub.
settings-unreal-github-again = Få en ny kode
settings-unreal-github-before = Før du starter
settings-unreal-github-before-what = Knyt GitHub til din Epic Games-konto, og accepter invitationen fra EpicGames, som GitHub sender dig på mail.
settings-unreal-github-use-token = Brug et token i stedet
settings-unreal-github-use-sign-in = Log ind med GitHub i stedet
settings-unreal-github-no-password = Ingen adgangskode når frem til Hoard
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = Forbundet som { $user }; prøver at downloade igen
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = Forbundet som { $user }
settings-unreal-github-as-what = Unreal Editor downloades som denne GitHub-konto.
settings-unreal-github-none = GitHub er ikke forbundet
settings-unreal-github-none-what = Kræves for at downloade editoren.
settings-unreal-github-change = Skift…
settings-unreal-image = Unreal-version eller -image
settings-editor-path = Angiv stien til den eksekverbare fil for din installerede { $tool }. Automatisk installation er kun tilgængelig i Docker.
error-unreal-image = Angiv en Unreal 5-version, et Epic dev/dev-slim-versionstag eller en Epic SHA-256-imagereference.
error-github-no-app = Denne server har ingen GitHub-app at logge ind igennem; brug et token i stedet.
error-github-unreachable = Kan ikke nå GitHub.
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub afviste login: { $reason }
