## Serverfeil

error-account-not-found = fant ikke kontoen
error-asset-gone = fant ikke ressursen
error-asset-not-found = fant ikke ressursen { $id }
error-blob-not-found = fant ikke blob { $hash }
error-collection-not-found = fant ikke samlingen { $id }
error-connection-not-found = fant ikke tilkoblingen
error-derivative-not-found = fant ikke { $kind }-varianten av { $id }
error-file-in-pack-not-found = fant ikke { $path } i pakken { $id }
error-format-not-found = fant ikke formatet { $file }
error-frame-sheet-aseprite-only = bare Aseprite-filer har bildeark
error-frame-sheet-frames = denne har ett bilde, eller for mange til å legge ut
error-location-not-found = fant ikke plasseringen { $id }
error-maintenance-task-not-found = fant ikke vedlikeholdsoppgaven { $id }
error-open-upload-not-found = fant ikke den åpne opplastingen { $id }
error-original-not-found = fant ikke originalfilen til { $id }
error-pack-not-found = fant ikke pakken { $id }
error-preview-too-big = dette bildet er for stort til å lage en lett forhåndsvisning av
error-project-not-found = fant ikke prosjektet { $name }
error-saved-search-not-found = fant ikke det lagrede søket { $id }
error-sign-in-first = logg på først (eller send et tilgangstoken)
error-those-assets-not-found = fant ikke de ressursene
error-thumbnail-not-found = fant ikke miniatyrbildet til { $id }
error-token-not-found = fant ikke tokenet
error-upload-not-found = fant ikke opplastingen { $id }
error-pack-needs-name = en pakke må ha et navn
error-no-library = det finnes ingen biblioteksplassering å lagre filer i ennå: legg til en administrert plassering i Innstillinger → Biblioteksplasseringer
error-upload-failed = opplastingen mislyktes: { $reason }
error-upload-no-files = ingen filer i opplastingen
error-analyze-nothing-to-do = ingenting å gjøre: rules=false og LLM er av
error-too-many-assets = for mange ressurser i én forespørsel
error-too-many-items = for mange elementer i én forespørsel
error-not-a-hash = ikke en blake3-hash i heks
error-merge-into-tag = into må være en tagg
error-nothing-to-merge = ingenting å slå sammen
error-selection-size = oppgi mellom 1 og { $max } ressurs-ID-er
error-collection-needs-name = en samling må ha et navn
error-collection-exists = det finnes allerede en samling som heter { $name }
error-collection-empty = samlingen { $name } er tom
error-conversion-failed = kunne ikke konvertere til { $format }: { $reason }
error-max-size = max må være en toerpotens fra 64 til 16384
error-replace-ext = ext må være en kort filendelse
error-empty-file = tom fil
error-replace-duplicate = den konverterte filen er lik en som allerede finnes i pakken; den opprinnelige ressursen ble slått sammen med den
error-ai-address = AI-adressen må begynne med http:// eller https://
error-local-llm-action = action er list, download eller delete, og de to siste trenger en modell
error-role = En rolle er view, edit eller admin
error-admin-only = bare en administrator kan gjøre dette
error-view-only = denne kontoen kan se, men ikke endre noe: en administrator kan endre det i Innstillinger → Konto
error-portal-only = logg på portalen for å administrere kontoer og tokener
error-not-from-portal = denne forespørselen kom ikke fra Hoard-portalen
error-account-name = Velg et navn på opptil 64 tegn
error-password-length = Bruk et passord på minst { $min } tegn
error-too-many-codes = For mange feil koder: vent { $seconds } sekunder
error-already-set-up = Det finnes allerede en konto: logg på
error-too-many-tries = For mange feil forsøk: vent { $seconds } sekunder
error-directory-unavailable = Katalogen kunne ikke sjekke passordet ditt: { $reason }
error-password-elsewhere = Passordet ditt ligger hos katalogen eller leverandøren du logger på med
error-current-password = Det nåværende passordet ditt er feil
error-account-source = En konto logger på med Hoard (internal), en katalog (ldap) eller OAuth
error-account-exists = Det finnes allerede en konto med det navnet
error-own-role = En annen administrator kan endre rollen din
error-remove-self = Du kan ikke fjerne din egen konto mens du er logget på den
error-token-name = Gi tokenet navn etter det som skal bruke det, med opptil 80 tegn
error-token-role = Et token kan ikke gjøre mer enn kontoen som lager det
error-wrong-setup-code = Det er ikke oppsettkoden serveren fikk i HOARD_SETUP_CODE
error-wrong-password = Navnet og passordet stemmer ikke overens
error-folder-missing = { $path } finnes ikke (en disk som ikke er montert?)
error-s3-path = en S3-plassering ser ut som s3://bucket
error-not-built = denne kopien av Hoard er bygget uten { $feature }
error-openai-auth-unreachable = kunne ikke nå auth.openai.com
error-openai-unexpected = uventet svar fra OpenAI
error-openai-answered = OpenAI svarte { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID-tokenet er ikke en JWT
error-chatgpt-id-token-issuer = ID-tokenet er fra { $issuer }, ikke { $expected }
error-chatgpt-id-token-audience = ID-tokenet er for en annen app
error-chatgpt-id-token-expired = ID-tokenet har utløpt
error-chatgpt-id-token-nonce = ID-tokenet hører ikke til denne påloggingen
error-chatgpt-no-state = adressen har ingen state; kopier hele adressen
error-chatgpt-not-started = denne påloggingen ble ikke startet her, eller den ble startet for over 15 minutter siden. Start den på nytt fra Innstillinger.
error-chatgpt-canceled = påloggingen ble avbrutt
error-chatgpt-refused = ChatGPT avviste påloggingen: { $reason }
error-chatgpt-no-code = adressen har ingen påloggingskode; kopier hele adressen
error-chatgpt-other-registration = ChatGPT svarte for en annen registrering av denne serveren; start påloggingen på nytt
error-chatgpt-not-registered = ChatGPT fullførte ikke registreringen av Hoard; start påloggingen på nytt
error-chatgpt-code-used = påloggingskoden er allerede brukt eller har utløpt; logg på igjen
error-chatgpt-refused-code = ChatGPT avviste påloggingen ({ $code })
error-chatgpt-no-id-token = ChatGPT sendte ikke noe ID-token
error-chatgpt-plan-not-allowed = Pålogget, men bruk av abonnementet ble ikke tillatt. Logg på igjen og tillat det, eller bruk en API-nøkkel i stedet.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = ikke pålogget ChatGPT; logg på fra Innstillinger → AI-tagging
error-chatgpt-refresh-later = ChatGPT kunne ikke fornye påloggingen akkurat nå ({ $code }); det prøves igjen senere
error-chatgpt-signed-out-problem = ChatGPT logget av denne serveren ({ $code }). Logg på igjen for å fortsette å tagge med abonnementet ditt.
error-chatgpt-signed-out = ChatGPT logget av denne serveren ({ $code }); logg på igjen fra Innstillinger → AI-tagging
error-openai-api-unreachable = kunne ikke nå api.openai.com
error-openai-broke-off = svaret fra OpenAI ble brutt
error-chatgpt-kept-refusing = OpenAI fortsatte å avvise ChatGPT-påloggingen; logg på igjen fra Innstillinger → AI-tagging
error-chatgpt-request-failed = forespørselen mislyktes
error-chatgpt-cut-off = svaret ble kuttet ({ $reason })
error-chatgpt-cut-off-no-reason = svaret ble kuttet (ingen grunn oppgitt)
error-openai-ended-early = svaret fra OpenAI sluttet før det var ferdig
error-chatgpt-usage-limit = ChatGPT-abonnementets kvote for Hoard er brukt opp for nå. Se eller endre grensen på { $url }
error-chatgpt-not-eligible = denne ChatGPT-kontoen eller -arbeidsområdet kan ikke bruke abonnementet i andre apper. Bruk en API-nøkkel i stedet.
error-chatgpt-usage-unavailable = ChatGPT kunne ikke sjekke abonnementets forbruk akkurat nå; det prøves igjen senere
error-chatgpt-unsupported = Bruk av ChatGPT-abonnement støtter ikke { $param } med denne modellen
error-chatgpt-unsupported-images = Bruk av ChatGPT-abonnement støtter ikke { $param } med denne modellen; velg en modell som tar imot bilder, eller slå av sending av miniatyrbilder
error-chatgpt-unsupported-part = Bruk av ChatGPT-abonnement støtter ikke en del av forespørselen med denne modellen
error-chatgpt-port = port er den nettleseren når Hoard på
error-chatgpt-paste-address = lim inn hele adressen, som begynner med http://127.0.0.1
error-too-many-hashes = maks { $max } hasher om gangen
error-credits-pack = pakke { $key }
error-credits-collection = samling { $key }
error-format-too-long = formatet er for langt
error-not-started = serveren har ikke startet
error-not-format-file = ikke en formatfil: { $name }
error-format-name = navnet må ha en bokstav eller et siffer fra A til Z eller 0 til 9
error-format-miss-name = formatets filer tar ikke med { $file }
error-format-miss-store = formatet tar ikke med { $file } for butikkens (se require)
error-format-miss-title = formatet finner ingen tittel i { $file }
error-cant-download-as = .{ $ext }-filer kan ikke lastes ned som { $format }
error-only-images-resize = bare bilder kan endre størrelse
error-https-no-certificate = Lim inn eller last opp sertifikatet og nøkkelen først
error-https-no-cert-found = Fant ikke noe sertifikat: lim inn teksten som begynner med -----BEGIN CERTIFICATE-----
error-https-no-key-found = Fant ingen privat nøkkel: lim inn teksten som begynner med -----BEGIN PRIVATE KEY-----
error-https-several-keys = Det er mer enn én privat nøkkel: lim inn bare den som hører til sertifikatet
error-https-key-password = Nøkkelen er beskyttet med passord: oppgi den uten (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Sertifikatet er ikke gyldig PEM
error-https-key-not-pem = Den private nøkkelen er ikke gyldig PEM
error-https-key-mismatch = Nøkkelen hører ikke til sertifikatet. Hvis du oppga en kjede, legg serverens sertifikat først
error-https-unusable = Sertifikatet eller nøkkelen kan ikke brukes
error-https-expired = Dette sertifikatet utløp { $date }
error-https-not-yet-valid = Dette sertifikatet er ikke gyldig før { $date }
error-https-unreadable = Det er ikke et sertifikat Hoard kan lese
error-https-bad-name = «{ $name }» er ikke en adresse et sertifikat kan inneholde
error-https-test-timed-out = testtilkoblingen fikk tidsavbrudd
error-https-test-failed = testtilkoblingen mislyktes
error-https-no-ca = Hoard har ikke laget sin sertifiseringsinstans ennå
error-https-unavailable = HTTPS er ikke tilgjengelig på denne serveren
error-https-certificate-mode = Et sertifikat hører til «Mitt sertifikat»
error-https-test = Sertifikatet fungerte ikke i en testtilkobling: { $reason }
error-nothing-to-retry = ingenting der kan prøves igjen
error-ai-unreachable = kunne ikke nå { $url }
error-ai-key-refused = { $url } avviste API-nøkkelen ({ $status })
error-ai-answered = { $url } svarte { $status }: { $reason }
error-ai-not-json = { $url }/models svarte ikke med JSON
error-model-server-refused = modellserveren: { $reason }
error-no-model-server = ingen modellserver svarer på { $url }
error-not-llama-router = { $url } er ikke en llama.cpp-server i ruter-modus, så den kan ikke laste ned modeller
error-model-download = kunne ikke laste ned { $model }: { $reason }
error-model-delete = kunne ikke slette { $model }: { $reason }
error-unknown-kind = ukjent ressurstype { $kind }
error-watched-is-folder = en overvåket plassering er en mappe på denne serveren
error-cant-reach = kan ikke nå { $path }: { $reason }
error-folder-full-path = oppgi hele banen til mappen
error-cant-create = kan ikke opprette { $path }
error-cant-write = kan ikke skrive til { $path }
error-cant-read = kan ikke lese { $path }
error-overlaps-location = det overlapper plasseringen { $name } ({ $path })
error-overlaps-data = det overlapper Hoards datamappe ({ $path })
error-overlaps-cache = det overlapper Hoards hurtigbuffermappe ({ $path })
error-not-watched = { $name } er ikke en overvåket mappe
error-watched-no-kinds = en overvåket mappe tar ikke imot nye filer, så den har ingen typer å endre
error-keep-catch-all = gjør heller en annen plassering til oppsamlingsplassering
error-location-holds-files = { $name } inneholder fortsatt filer: flytt dem til de andre plasseringene først (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } kan ikke nås, så Hoard kan ikke vite om den fortsatt inneholder filer
error-relocate-running = filer flyttes allerede mellom plasseringer; prøv igjen når det er ferdig
error-watched-not-moved = { $name } er en overvåket mappe: Hoard flytter ikke filene i den
error-no-other-location = det finnes ingen annen biblioteksplassering å flytte filene til: legg til en først
error-full-path = oppgi en full bane
error-pick-folder = velg en mappe
error-task-running = «{ $task }» kjører allerede
error-token-account-gone = kontoen til dette tokenet finnes ikke lenger, så det har ingen favoritter eller søk
error-favorites-too-many = maks 10 000 om gangen
error-saved-search-name = et lagret søk må ha et navn på maks 80 tegn
error-saved-search-too-long = det søket er for langt til å lagres
error-texture-pattern-map = Mønsteret for teksturnavn må ha { "{" }map{ "}" }, ellers ville alle kartene til et materiale få samme navn
error-texture-pattern-fields = Mønsteret for teksturnavn kan bruke { "{" }name{ "}" }, { "{" }map{ "}" } og { "{" }size{ "}" }
error-texture-name-chars = Teksturnavn kan ikke inneholde < > : " / \ | ? eller *
error-texture-map-type = Det finnes ingen karttype som heter { $role }
server-role-base-color = farge
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = ruhet
server-role-gloss = glans
server-role-metalness = metallisk
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = høyde
server-role-bump = bump
server-role-cavity = cavity
server-role-specular = spekulær
server-role-opacity = opasitet
server-role-emissive = emissiv
server-role-translucency = gjennomskinnelighet
error-texture-suffix-missing = Gi teksturer av typen { $map } et suffiks
error-texture-suffix-chars = Suffikset for teksturer av typen { $map } kan ikke inneholde < > : " / \ | ? eller *
error-texture-ending-missing = Hver egen endelse trenger endelsen det skal ses etter
error-texture-ending-chars = Egne endelser kan ikke inneholde < > : " / \ | ? eller *
error-texture-ending-twice = { $ending } står to ganger blant endelsene dine
error-oauth-no-client-id = Denne lenken har ingen client_id.
error-oauth-unknown-app = Denne appen er ikke registrert her (lenger). Start tilkoblingen på nytt fra appen.
error-oauth-redirect = Appen ba om å bli sendt tilbake til en adresse den ikke har registrert.
error-oauth-response-type = Hoard deler bare ut påloggingskoder (response_type=code).
error-oauth-pkce = Appen må bruke PKCE med S256 for å logge på Hoard.
error-light-preview-images = bare bilder har lett forhåndsvisning
error-no-light-preview = ingen lett forhåndsvisning: { $reason }
error-no-frame-sheet = ikke noe bildeark: { $reason }
error-thumbnail-source = ukjent kilde for miniatyrbilde: { $source }
error-thumbnail-too-big = det bildet er for stort for et miniatyrbilde
error-thumbnail-models-only = bare modeller kan få et miniatyrbilde tegnet av en nettleser
error-not-a-picture = ikke et bilde Hoard kan lese: { $reason }
error-project-name = et prosjekt må ha et navn på maks 200 tegn
error-project-too-big = maks én million ressurser i ett prosjekt
error-project-too-many-at-once = maks 100 000 om gangen
error-server-name = Velg et navn på opptil { $max } tegn, på én linje
error-tool-setting = { $tool }: velg av, automatisk eller oppgi en bane
error-ai-model = Velg modellen AI-tjenesten skal bruke
error-public-address = Den offentlige adressen må begynne med http:// eller https://
error-update-hook-address = Adressen som skal kalles ved oppdateringer, må begynne med http:// eller https://
error-update-hook-auth = Authorization-headeren kan ikke inneholde linjeskift eller andre kontrolltegn
error-no-soundfont = Det finnes ingen SoundFont på { $path } på serveren
error-workers = Bakgrunnsarbeidere: mellom 1 og { $max }
error-ai-batch = Ressurser per AI-forespørsel: mellom 1 og 200
error-conversion-cache = Konverterte nedlastinger: minst 100 MB
error-conversion-keep-days = Konverteringsregler: behold originaler i 1 til { $max } dager
error-conversion-target = Konverteringsregler: Hoard kan ikke konvertere filer til { $format }
error-conversion-no-source = Konverteringsregler: hver regel trenger minst ett format å konvertere
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Konverteringsregler: .{ $ext }-filer kan ikke konverteres til { $format }
error-conversion-twice = Konverteringsregler: .{ $ext } er med i to regler
error-conversion-cant-undo = Denne filen ble ikke konvertert av en regel, eller originalen er slettet.
error-conversion-path-taken = Det ligger en annen fil på { $path } nå, så originalen kan ikke flyttes tilbake dit.
error-limits-zero = Grenser kan ikke være null
error-settings-object = forventet et JSON-objekt med innstillinger
error-bad-setting = ugyldig innstilling: { $reason }
error-auth-default-role = Standardrollen er view, edit eller admin
error-auth-session-hours = Økter varer mellom 1 time og ett år
error-auth-settings = ugyldige autentiseringsinnstillinger: { $reason }
error-ldap-settings = ugyldige LDAP-innstillinger: { $reason }
error-ldap-no-server = Legg til adressen til katalogserveren
error-ldap-base-dn = Oppgi hvor personoppføringene ligger (base-DN)
error-ldap-username-attribute = Oppgi hvilket attributt som inneholder navnet folk logger på med
error-ldap-bind-dn = Oppgi kontoen som slår opp personer, eller slå dem opp anonymt
error-ldap-ca-cert = Sertifiseringsinstansen er ikke et PEM-sertifikat (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Adressen til en katalogserver er tom
error-ldap-server-form = { $server }: adressen til en katalogserver er vert:port, ldap://… eller ldaps://…
error-ldap-not-starttls = { $server }: det er LDAPS, ikke StartTLS
error-ldap-no-certificate = ikke noe sertifikat i den
error-ldap-password-again = Oppgi passordet til kontoen som slår opp personer på nytt: det lagrede sendes bare til serverne det ble oppgitt for
error-ldap-unreachable = ingen katalogserver kunne nås ({ $reasons })
error-ldap-no-anonymous = katalogen tillater ikke anonyme oppslag av personer ({ $reason })
error-ldap-bind-failed = kontoen som slår opp personer, kunne ikke logge på ({ $reason })
error-ldap-search = leter etter { $name } under { $base }
error-ldap-group-search = leter etter gruppene til { $name } under { $base }
error-ldap-several = { $count } oppføringer i katalogen samsvarer med { $name }: snevre inn brukerfilteret
error-ldap-password-check = katalogen ville ikke sjekke passordet til { $name } ({ $reason })
error-ldap-wrong-password = Katalogen kjenner ikke { $name } med det passordet
error-oauth-client-id = Oppgi klient-ID-en leverandøren laget for Hoard
error-oauth-addresses = Autorisasjons- og tokenadressene må begynne med https://
error-oauth-userinfo-address = Adressen for brukerinformasjon må begynne med https://
error-oauth-username-claim = Oppgi hvilken claim som inneholder personens navn
error-oauth-secret-again = Oppgi klienthemmeligheten på nytt: den lagrede sendes bare til tokenadressen den ble oppgitt for
error-oauth-issuer = Utstederen (issuer) må begynne med https://
error-oauth-discovery-unreachable = Kunne ikke nå { $url }: { $reason }
error-oauth-discovery-status = { $url } svarte { $status }
error-oauth-discovery-bad = { $url } er ikke et discovery-dokument
error-oauth-discovery-incomplete = { $url } oppgir ikke autorisasjons- og tokenadressene
error-signin-no-group = { $name } er ikke i en gruppe som kan bruke Hoard
error-signin-other-source = Det finnes allerede en Hoard-konto som heter { $name }, og som logger på på en annen måte
error-signin-other-subject = Det finnes allerede en Hoard-konto som heter { $name }, for en annen person hos leverandøren
error-signin-no-account = { $name } har ingen Hoard-konto ennå: en administrator kan legge den til i Innstillinger → Konto
error-signin-bad-name = Det navnet kan ikke brukes for en Hoard-konto (opptil 64 tegn)
error-oauth-off = Pålogging med OAuth er ikke slått på
error-oauth-bad-authorization-address = autorisasjonsadressen er ikke gyldig
error-oauth-refused = Leverandøren logget deg ikke på: { $reason }
error-oauth-no-code = Leverandøren sendte deg tilbake uten pålogging
error-oauth-expired = Påloggingen tok for lang tid eller ble startet et annet sted: prøv igjen
error-oauth-no-claim = Leverandøren sa ikke hvem du er (ingen { $claim }-claim)
error-oauth-no-sub = Leverandøren sa ikke hvem du er (ingen sub- eller id-claim)
error-oauth-token-unreachable = kunne ikke nå leverandørens tokenadresse
error-oauth-token-refused = Leverandøren ville ikke fullføre påloggingen: { $reason }
error-oauth-no-access-token = leverandøren sendte ikke noe tilgangstoken
error-oauth-userinfo-unreachable = kunne ikke nå leverandørens adresse for brukerinformasjon
error-oauth-userinfo-refused = Leverandøren ville ikke si hvem som logget på ({ $status })
error-oauth-userinfo-not-json = leverandørens brukerinformasjon er ikke JSON
error-format-no-name = name er tomt
error-format-no-files = files er tomt: oppgi hvilke filer formatet leser, for eksempel ["*.json"]
error-format-empty-selector = tom
error-format-unopened-bracket = en lukkende parentes uten en åpnende
error-format-unclosed-bracket = en parentes er ikke lukket
error-format-empty-step-dots = et tomt steg (to punktum etter hverandre, eller et punktum i en av endene)
error-format-unclosed-except = !( er ikke lukket
error-format-unclosed-filter = [ er ikke lukket
error-format-after-filters = { $rest } etter nøkkelen og filtrene til { $step }
error-format-except-needs-glob = !(…) kommer etter en nøkkel med * i
error-format-except-not-glob = !(…) kommer etter en nøkkel med * i, ikke { $key }
error-format-empty-step = et tomt steg
error-format-not-a-filter = [{ $filter }] er ikke et filter: bruk [], [key=value|other], [array], [string], [number], [bool] eller [object]
error-format-filter-needs-value = [{ $filter }] trenger en nøkkel og en verdi
error-store-search-empty = ingenting å søke etter; skriv inn navnet på pakken
error-update-hook-not-set = angi først en adresse som skal kalles
error-upload-stalled = opplastingen av { $path } stoppet opp; send den delen på nytt
error-upload-busy = { $path } skrives allerede
error-upload-incomplete = { $count } filer er ufullstendige, f.eks. { $files }
error-upload-no-files-listed = ingen filer
error-upload-not-part = { $path } er ikke en del av opplastingen { $id }, eller opplastingen er lukket
error-bad-path = ugyldig bane
error-upload-part-offset = { $offset } er ikke der en del av { $path } begynner
error-upload-interrupted = opplastingen ble avbrutt: { $reason }
error-upload-part-long = del { $part } av { $path } er lengre enn { $len } byte
error-upload-part-short = del { $part } av { $path }: fikk { $got } byte, forventet { $len }
error-upload-file-long = { $path } er større enn oppgitt
error-upload-file-short = { $path }: fikk { $got } byte, forventet { $len }
error-upload-complete = opplastingen er allerede fullført
error-already-imported = allerede importert som «{ $name }»

## Servertekst

server-store-place-readme = README
server-store-place-folders = mapper
server-store-why-linked = Lenket til i pakkens { $place }
server-store-why-same-title = Samme tittel som pakkens { $place }
server-store-why-by = Av { $author }, nevnt i pakkens { $place }
server-store-why-meshes = Omtrent like mange mesher som pakken ({ $count })
server-store-why-textures = Omtrent like mange teksturer som pakken ({ $count })
server-chatgpt-revoke-failed = Logget av her, men OpenAI kunne ikke varsles. For å være sikker kan du koble fra Hoard i ChatGPT under Innstillinger → Sikkerhet og pålogging → Logg på med ChatGPT.
server-chatgpt-signed-in-title = Pålogget ChatGPT
server-chatgpt-signed-in = Hoard er pålogget. AI-tagging kan nå bruke ChatGPT-abonnementet ditt.
server-chatgpt-signed-in-as = Hoard er pålogget som { $email }. AI-tagging kan nå bruke ChatGPT-abonnementet ditt.
server-chatgpt-not-finished-title = Påloggingen er ikke fullført
server-chatgpt-other-computer = Hvis du startet påloggingen fra Hoard på en annen datamaskin, kopierer du adressen til denne siden fra adressefeltet og limer den inn i Hoards innstillinger der.
server-back-to-settings = Tilbake til Innstillinger
server-issues-pack = Pakker som ikke kunne legges til
server-issues-pack-blurb = Slett pakken og legg den til på nytt, eller åpne den for å se hva den har.
server-issues-damaged = Lagrede filer som er skadet eller borte
server-issues-damaged-blurb = Funnet av «Sjekk filer for skade». Gjenopprett dem fra en sikkerhetskopi eller legg til pakken på nytt, og kjør så «Sjekk filer for skade».
server-issues-folder = Overvåkede mapper som ikke kan leses
server-issues-folder-blurb = Pakkene deres blir latt være som de er til mappen kan leses igjen.
server-issues-missing = Overvåkede pakker der mappen er borte
server-issues-missing-blurb = Hver pakke beholdes i 3 dager i tilfelle mappen bare er frakoblet, og fjernes deretter.
server-issues-job = Bakgrunnsjobber som mislyktes
server-issues-job-blurb = Prøv igjen kjører jobben på nytt fra starten.
server-issues-file = Filer som ikke kunne åpnes
server-issues-file-blurb = Hoard kunne ikke åpne eller lagre dem. En overvåket pakke prøver dem igjen ved neste skanning; en fil som allerede er i biblioteket, beholdt det den hadde fra før.
server-issues-unpack = Arkiver som ikke kunne pakkes ut
server-issues-unpack-blurb = Hvert av dem beholdes i pakken som en vanlig fil, så ingenting i det er i biblioteket.
server-issues-read = Filer Hoard ikke fikk noe ut av
server-issues-read-blurb = De er i biblioteket, men uten faktaene, taggene eller miniatyrbildet som lesingen gir.
server-issues-render = Modeller miniatyrbilde-rendereren feilet på
server-issues-render-blurb = De har ikke noe miniatyrbilde. Å åpne en i visningen kan likevel fungere, og kan lagre et.
server-issues-convert = Modeller Blender ikke kunne konvertere
server-issues-convert-blurb = De har ingen GLB-kopi, så de kan ikke vises i 3D.
server-issues-unreal = Unreal-pakkefiler som ikke kunne konverteres
server-issues-unreal-blurb = Pakkens Unreal-side har resten av pakkefilene.
server-issues-reformat = Filer en konverteringsregel ikke kunne konvertere
server-issues-reformat-blurb = De er latt være som de var.
server-issues-materials = Pakker der materialene ikke kunne finnes ut av
server-issues-materials-blurb = Teksturene er der alle sammen; de er bare ikke gruppert i materialer.
server-issues-material-thumb = Materialer uten miniatyrbilde
server-issues-material-thumb-blurb = Materialet ble funnet, men forhåndsvisningen kunne ikke tegnes.
server-issues-store-files = Butikkfiler som ikke kunne leses
server-issues-store-files-blurb = Filer fra butikken en pakke kom fra, for eksempel en Megascans-JSON, som navngir og tagger den.
server-issues-scan-again = Skann igjen
server-issues-retry = Prøv igjen
server-issues-reanalyze = Analyser pakken på nytt
server-issues-convert-again = Konverter igjen
server-issues-look-again = Se igjen
server-issues-read-again = Les igjen
server-issues-blob-missing = Den lagrede filen er borte.
server-issues-blob-damaged = Den lagrede filen stemmer ikke med det som ble lagret: den er skadet.
server-issues-folder-gone = Fant ikke mappen «{ $folder }».
server-job-a-pack = en pakke
server-job-ingest = Tar inn { $pack }
server-job-retag = Tagger { $pack } på nytt
server-job-llm-tag = Tagger { $pack } med AI
server-job-convert = Konverterer { $pack } med Blender
server-job-unreal = Konverterer Unreal-pakkefiler i { $pack }
server-job-reformat = Konverterer filer i { $pack } etter konverteringsreglene
server-job-scan = Skanner en overvåket mappe
server-job-relocate = Flytter filer mellom biblioteksplasseringer
server-job-looks = Finner ut farger og utseende
server-job-sounds = Lytter til lyder
server-job-embed = Lærer hva bilder viser
server-job-materials = Finner materialer i { $pack }
server-job-read-files = Leser filene i { $pack }
server-job-store-lookup = Slår opp en butikkside
server-maintenance-backup = Sikkerhetskopier databasen
server-maintenance-backup-blurb = En konsistent kopi av databasen (pakker, tagger, notater, samlinger og innstillinger) i backups-mappen ved siden av den. Bibliotekets filer er ikke med: de endres aldri når de først er lagret, så enhver filsynkronisering sikkerhetskopierer dem.
server-maintenance-scrub = Sjekk filer for skade
server-maintenance-scrub-blurb = Leser tilbake hver lagrede fil og sjekker den mot fingeravtrykket, for å fange opp filer som har råtnet eller forsvunnet fra en disk eller NAS. Tar en stund på et stort bibliotek.
server-maintenance-reanalyze = Analyser alle pakker på nytt
server-maintenance-reanalyze-blurb = Leser hver fil på nytt med denne versjonen av Hoard: miniatyrbilder, forhåndsvisninger, detaljer og regeltagger lages på nytt. Dine egne endringer blir værende; AI-en spørres ikke igjen.
server-maintenance-clear-cache = Tøm hurtigbufrede konverteringer
server-maintenance-clear-cache-blurb = Fjerner filene som er beholdt fra «Last ned som»-konverteringer (de lages på nytt neste gang noen ber om dem) og det avbrutt arbeid har latt ligge igjen.
server-maintenance-shrink-thumbs = Krymp miniatyrbilder
server-maintenance-shrink-thumbs-blurb = Lagrer miniatyrbildene og modellvisningene som er lagret som PNG, slik Innstillinger → Ytelse → Miniatyrbilder angir, noe som vanligvis gjør dem flere ganger mindre. De med gjennomsiktighet, og pikselgrafikk, forblir PNG.
server-maintenance-gc = Slett ubrukte filer
server-maintenance-gc-blurb = Sletter lagrede filer som ingenting peker på lenger, for eksempel filene til pakker som er slettet for godt.
server-update-no-address = ingen adresse å kalle er angitt
server-update-no-answer = ikke noe svar innen 30 sekunder

## Innstillinger: Utseende

appearance-language = Språk
appearance-language-auto = Nettleserens ({ $name })

## Rot

root-unreachable = Kunne ikke nå Hoard-serveren: { $error }
root-try-again = Prøv igjen
root-lazy-connect = app-tilkoblingen

## App: biblioteket, topplinjen og filtrene

app-lazy-settings = innstillingene
app-lazy-archive = arkivet
app-lazy-shortcuts = snarveiene
app-lazy-setup = oppsettet
app-theme-system = Følg systemtemaet
app-theme-light = Lyst tema
app-theme-dark = Mørkt tema
app-color-red = Rød
app-color-orange = Oransje
app-color-yellow = Gul
app-color-green = Grønn
app-color-teal = Blågrønn
app-color-blue = Blå
app-color-purple = Lilla
app-color-pink = Rosa
app-color-brown = Brun
app-color-sand = Sand
app-color-gray = Grå
app-color-slate = Skifer
app-color-black = Svart
app-color-white = Hvit
app-color-swatch = { $color }: ressurser med mye { $lower } i seg
app-color-any = Hvilken som helst farge
app-color-pick = Velg en farge
app-color = Farge
app-retry = Prøv igjen
app-undo = Angre
app-search-failed = Søket mislyktes: { $error }
app-unreachable = Kunne ikke nå serveren: { $error }
app-forget-project-title = Glemme { $name }?
app-forget-project-body = Hoard slutter å vise det der de { $count } ressursene brukes, til neste hoard sync forteller det igjen. Ingenting i prosjektet endres.
app-forget-project-confirm = Glem det
app-forget-project-failed = Kunne ikke glemme { $name }: { $error }
app-forget-project-label = Glem prosjektet { $name }
app-forget-project = Glem dette prosjektet
app-save-name-like = som { $name }
app-save-name-favorites = favoritter
app-search-saved = Lagret «{ $name }»: det ligger i Filtre, under Dine
app-rank-alike = Mest lik
app-rank-color = Mest av fargen
app-rank-match = Beste treff
app-sort = Sorter
app-sort-newest = Nyeste
app-sort-name = Navn
app-sort-largest = Største
app-sort-oldest = Eldste
app-collection-added =
    La til { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    } i { $name }
app-collection-add-failed = Kunne ikke legge til i samlingen: { $error }
app-collection-removed = Fjernet { $count } fra { $name }
app-collection-remove-failed = Kunne ikke fjerne fra samlingen: { $error }
app-ns-subject = Motiv
app-ns-style = Stil
app-ns-theme = Tema
app-ns-material = Materiale
app-ns-map = PBR-kart
app-ns-color = Farge
app-ns-mood = Stemning
app-ns-genre = Sjanger
app-ns-license = Lisens
app-ns-res = Oppløsning
app-ns-duration = Lengde
app-ns-poly = Antall polygoner
app-ns-format = Format
app-ns-weight = Vekt
app-ns-tech = Teknisk
app-ns-blender = Blender-versjon
app-ns-depth = Bitdybde
app-ns-ucs = Lydkategori (UCS)
app-ns-bpm = Tempo
app-ns-root = Grunntone
app-ns-level = Lydstyrke
app-ns-ue = Unreal-klasse
app-ns-engine = Motor
app-ns-extracted = Gjenopprettet kilde
app-ns-script = Skriftsystem
app-bulk-failed =
    Kunne ikke endre { $count ->
        [one] den ressursen
       *[other] de { $count } ressursene
    }: { $error }
app-bulk-untagged =
    Fjernet { $tag } fra { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    }
app-bulk-tagged =
    Tagget { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    } med { $tag }
app-bulk-category-set =
    Satte kategorien for { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    } til { $category }
app-bulk-category-cleared =
    Fjernet kategorien for { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    }
app-credits-failed = Kunne ikke lage krediteringen: { $error }
app-archived-assets =
    Arkiverte { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    }
app-archived-one = Arkiverte { $name }
app-open-failed = Kunne ikke åpne den ressursen: { $error }
app-unnamed-pack = pakke
app-unnamed-collection = samling
app-unnamed-asset = ressurs
app-home-version = Hoard { $version } – tilbake til biblioteket
app-home = Tilbake til biblioteket
app-search-label = Søk i ressurser
app-search-placeholder = Søk i navn, baner, tagger
app-views = Visninger
app-library = Bibliotek
app-packs = Pakker
app-packs-working = { $count } i arbeid
app-update-title = Hoard { $version } er ute (denne serveren kjører { $current }). Hent det nye bildet eller den nye binærfilen og start på nytt; databasen oppgraderer seg selv.
app-update = Oppdater
app-theme = Tema
app-sign-in = Logg på
app-archive-title = Arkiv: { $contents }
app-archive = Arkiv
app-settings = Innstillinger
app-palette-title = Kommandopalett ({ $keys })
app-import-folder = Importer mappe
app-add-files = Legg til filer
app-add-files-short = Legg til
app-split-label = Slik importerer du denne mappen
app-split-before = { "" }
app-split-after = { " " }inneholder { $count } mapper eller arkiver.
app-split-each = Importer hver som en egen pakke ({ $count })
app-split-one = Importer som én pakke
app-cancel = Avbryt
app-filters = Filtre
app-done = Ferdig
app-yours = Dine
app-favorites = Favoritter
app-forget-search-label = Glem det lagrede søket { $name }
app-forget-search = Glem dette søket
app-projects-title = Prosjekter som kjører hoard sync herfra, med det siste synkronisering hadde
app-projects = Prosjekter
app-project-synced = Synkronisert { $when }
app-project-synced-by = Synkronisert { $when } av { $who }
app-collections = Samlinger
app-no-filters = Ingen filtre for dette søket.
app-kind = Type
app-facets-more = { $count } til
app-facets-fewer = færre
app-searching = Søker …
app-searching-live = Søker
app-asset-count =
    { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    }
app-clear-all = Fjern alle
app-save-name-label = Navn på dette søket
app-save-name-placeholder = Gi det et navn
app-save = Lagre
app-save-search-title = Behold dette søket i Filtre, under Dine
app-save-search = Lagre søk
app-select-title = Velg flere ressurser for å tagge, kategorisere, samle, laste ned eller arkivere dem samlet (X, eller Ctrl-klikk)
app-select = Velg
app-tile-size = Flisstørrelse
app-supporting-title = glTF-buffere, materialbiblioteker, kartene til materialer, materialer som bare én modell bruker, og butikkfilene som beskriver modeller. De følger med når du laster ned det som bruker dem.
app-supporting-hidden =
    { $count } { $count ->
        [one] støttefil
       *[other] støttefiler
    } skjult
app-supporting-show = Vis
app-supporting-shown = Inkluderer støttefiler
app-supporting-hide = Skjul
app-welcome-title = Slipp en ressurspakke hvor som helst på denne siden
app-welcome-before = Zip-filer, tarballer, mapper og løse filer fungerer alle. Hoard pakker dem ut, lagrer hver fil én gang etter hashen, leser størrelser, lengder og antall polygoner, og tagger alt slik at du kan søke etter ting som{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }eller{ " " }
app-welcome-after = .
app-welcome-import-folder = Importer en mappe
app-welcome-choose-files = Velg filer
app-none-similar = Ingenting annet ligner særlig på { $name } ennå.
app-none = Ingen treff.
app-none-query = Ingen treff for «{ $query }».
app-none-filters = Ingen treff med disse filtrene.
app-none-query-filters = Ingen treff for «{ $query }» med disse filtrene.
app-clear-search-filters = Fjern søk og filtre
app-loading-more = Laster flere
app-keys-hint = Piltastene flytter, Enter åpner, Mellomrom spiller av en lyd, X eller Ctrl-klikk velger flere (trykk og hold på berøringsskjerm).
app-resize-label = Endre størrelse på detaljpanelet
app-resize-title = Dra for å endre størrelse · dobbeltklikk for bredden som passer
app-selected-label = Valgte ressurser
app-selected = { $count } valgt
app-pick-assets = Velg ressurser
app-download-zip = Last ned zip
app-credits-title = CREDITS.md for pakkene disse kommer fra: hvem som laget dem, lisensene og krediteringslinjene de ber om
app-credits = Kreditering
app-prefer-sources-title = Unreal-SoundWaves og -teksturer der Hoard har gjenopprettet originalfilen
app-prefer-sources = Gjenopprettet WAV/PNG i stedet for .uasset ({ $count })
bag-clear = Tøm utvalget
bag-glb-heading = Modeller som GLB
bag-glb-text = Hver modell som en .glb-fil som står med Y opp og vender fremover, med teksturene som egne filer i en textures-mappe modellene deler. Alt annet blir med som det er.
bag-glb-download = Last ned GLB-zip
bag-plain-heading = Filer som de er
bag-plain-text = Hver fil slik den ble importert, med det den trenger, i en mappe for hver pakke.
bag-remove = Ta { $name } ut av utvalget
bag-more = og { $count } til
app-bulk-tag-label = En tagg som skal legges til eller fjernes fra de valgte ressursene
app-bulk-tag-placeholder = Tagg
app-bulk-tag-add-title = Legg til denne taggen på alle valgte ressurser
app-bulk-tag-add = Legg til
app-bulk-tag-remove-title = Fjern denne taggen fra alle valgte ressurser
app-bulk-tag-remove = Fjern
app-bulk-category-label = Kategori for de valgte ressursene (tom fjerner den)
app-bulk-category-placeholder = Kategori
app-bulk-category-set-title = Angi kategorien for alle valgte ressurser
app-bulk-category-clear-title = Fjern kategorien for alle valgte ressurser
app-bulk-category-set-button = Angi
app-bulk-category-clear-button = Fjern
app-remove-from-collection = Fjern fra { $name }
app-unfavorite = Fjern favoritt
app-favorite = Favoritt
app-select-all = Velg alle { $count }
app-archive-selected-title = Utenfor søk til de gjenopprettes eller slettes for godt, fra Arkiv
app-drop-items = Slipp { $count } elementer for å legge dem til
app-drop = Slipp for å legge til i Hoard
app-drop-hint = Hver fil eller mappe på øverste nivå blir sin egen pakke. Gigabyte går fint; store filer fortsetter hvis tilkoblingen brytes.

## Kommandopalett (CommandPalette.vue, med kommandoene fra App.vue)

palette-group-go-to = Gå til
palette-group-actions = Handlinger
palette-group-theme = Tema
palette-group-kinds = Typer
palette-group-packs = Pakker
palette-group-collections = Samlinger
palette-group-projects = Prosjekter
palette-group-saved = Lagrede søk
palette-group-assets = Ressurser
palette-hint-current = gjeldende
palette-library = Bibliotek
palette-library-keywords = ressurser rutenett hjem start assets grid home
palette-favorites = Favoritter
palette-favorites-keywords = stjerne stjernemerket likt starred stars liked
palette-packs = Pakker
palette-archive = Arkiv
palette-archive-keywords = papirkurv søppel slettet gjenopprett trash recycle bin deleted restore
palette-stats = Bibliotekstatistikk
palette-stats-keywords = statistikk tall størrelse diskplass antall statistics numbers size disk space count
palette-search = Søk
palette-add-files = Legg til filer …
palette-add-files-keywords = last opp opplasting importer upload import
palette-import-folder = Importer en mappe …
palette-import-folder-keywords = last opp pakke upload pack
palette-select-all = Velg alle { $count } som vises
palette-select-all-keywords = merk huk av check
palette-select-several = Velg flere …
palette-select-several-keywords = mange flere tagg arkiver bulk multiple tag archive
palette-archive-selected = Arkiver de { $count } valgte
palette-archive-selected-keywords = slett fjern papirkurv delete remove trash
palette-clear-selection = Fjern utvalget
palette-clear-filters = Fjern filtre
palette-clear-filters-keywords = nullstill tilbakestill reset
palette-hide-supporting = Skjul støttefiler
palette-show-supporting = Vis støttefiler
palette-supporting-keywords = normalkart ruhet masker normal maps roughness masks
palette-account = Konto og tilgangstokener
palette-account-keywords = passord pålogging bruker token api password login user
palette-sign-out = Logg av ({ $name })
palette-sign-out-keywords = logg ut avlogging log out logout
palette-sign-in = Logg på
palette-sign-in-keywords = logg inn pålogging konto passord log in login account password
palette-settings = Innstillinger
palette-settings-keywords = blender ffmpeg llm ai tagging unreal admin verktøy innstillinger preferanser tools preferences
palette-shortcuts = Hurtigtaster
palette-shortcuts-keywords = hjelp taster tastatur help keys
palette-theme-keywords = tema utseende modus theme appearance mode
palette-theme-variant-keywords = utseende farge appearance color
palette-light-theme = Lyst tema: { $name }
palette-dark-theme = Mørkt tema: { $name }
palette-kind-all = Alle typer (slutt å vise bare { $kind })
palette-kind-only = Bare { $kind }
palette-title = Kommandopalett
palette-placeholder = Søk i ressurser, gå til en pakke, kjør en kommando …
palette-results = Resultater
palette-searching = Søker …
palette-no-match = Ingen treff for «{ $query }»
palette-foot-move = flytt
palette-foot-run = kjør
palette-foot-shortcuts = alle hurtigtaster

## Hurtigtaster (ShortcutsDialog.vue)

shortcuts-title = Hurtigtaster
shortcuts-close = Lukk
shortcuts-anywhere = Hvor som helst
shortcuts-palette = Kommandopalett: søk, hopp, kjør
shortcuts-search = Søk
shortcuts-this-list = Denne listen
shortcuts-escape = Lukk, fjern utvalget eller lukk detaljene
shortcuts-library = Bibliotek
shortcuts-move = Flytt mellom fliser (eller J og K)
shortcuts-open = Åpne detaljene
shortcuts-play-sound = Spill av eller stopp en lyd
shortcuts-select = Velg eller fjern valg
shortcuts-favorite = Legg til i eller fjern fra favorittene dine
shortcuts-key-click = klikk
shortcuts-select-range = Velg et område
shortcuts-select-all = Velg alt som vises
shortcuts-sound-player = Lydspiller
shortcuts-play-pause = Spill av eller pause
shortcuts-skip = Hopp bakover eller fremover
shortcuts-start-end = Start eller slutt
shortcuts-image-inspector = Bildeinspektør
shortcuts-zoom = Zoom inn eller ut
shortcuts-fit = Tilpass til vinduet
shortcuts-actual-pixels = Faktiske piksler
shortcuts-tile = Flislegg 3×3 for å sjekke skjøtene
shortcuts-background = Neste bakgrunn
shortcuts-channel = Vis én kanal

## KindChips.vue

kinds-group = Ressurstyper

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = lagt til av en person
tag-source-rule = fra den lokale taggeren
tag-source-meta = målt fra filen
tag-source-ai = foreslått av AI
tag-remove = Fjern taggen { $tag }

## ActivityIndicator.vue

activity-label-idle = Bakgrunnsaktivitet: inaktiv
activity-label-running = Bakgrunnsaktivitet: { $running } kjører
activity-label-queued = Bakgrunnsaktivitet: { $running } kjører, { $queued } i kø
activity-title = Bakgrunnsaktivitet
activity-unreachable = Kan ikke nå serveren
activity-empty = Ingenting kjører. Importer, tagging, konverteringer og zip-nedlastinger vises her mens de pågår.
activity-queued = i kø
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } t { $min } min
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } trekanter
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Spill av { $name }
tile-pause = Sett { $name } på pause
tile-favorite = Gjør { $name } til favoritt
tile-in-favorites = I favorittene dine
tile-add-favorite = Legg til i favorittene dine (F)
tile-more-like = Mer som { $name }
tile-more-like-this = Mer som dette
tile-select = Velg { $name }

## CollectionPicker.vue

collections-add = Legg til i samling
collections-menu = Samlinger
collections-find-label = Finn eller opprett en samling
collections-find-placeholder = Finn eller gi navn til en ny samling
collections-create = Opprett
collections-empty = Ingen samlinger ennå. Skriv et navn for å starte en.

## FolderPicker.vue

folders-list = Mapper
folders-close = Lukk
folders-browse = Bla gjennom …
folders-up-title = Opp én mappe
folders-up = Opp
folders-this-computer = Denne datamaskinen
folders-none = Ingen mapper her.

## LocationForm.vue

locations-mode-question = Hva skal Hoard gjøre med denne mappen?
locations-mode-managed = Ha biblioteket mitt her
locations-mode-managed-hint = Pakker du laster opp eller importerer, kopieres inn i denne mappen. Hoard tar seg av den, så la innholdet være i fred.
locations-mode-watched = Indekser en mappe jeg allerede har
locations-mode-watched-hint = Hver mappe i den blir en pakke. Hoard leser filene der de ligger og endrer, flytter eller sletter dem aldri, og plukker opp nye mapper når de dukker opp.
locations-folder = Mappe på serveren
locations-folder-placeholder = Mappen pakkene dine ligger i
locations-folder-hint-managed = Hoard oppretter den hvis den ikke finnes. I Docker er dette en mappe du har montert inn i containeren.
locations-folder-hint-watched = I Docker monterer du først mappen inn i containeren (skrivebeskyttet går fint), og velger den så her.
locations-name = Navn
locations-name-placeholder-managed = Bibliotek
locations-name-placeholder-watched = Mine pakker
locations-files = Filer som havner her
locations-catch-all = Også alt ingen annen plassering tar
locations-kinds-hint = Filene i en ny pakke havner på plasseringen som har krysset av for typen deres. Resten havner på oppsamlingsplasseringen.
locations-files-all = Alle sammen, inntil videre. Legg til flere plasseringer senere fra Innstillinger for å sende noen typer (for eksempel musikk) til en annen disk.
locations-first-watched-note = Opplasting krever et bibliotek som Hoard tar vare på. Du kan legge til et senere fra Innstillinger → Biblioteksplasseringer.
locations-checking = Sjekker mappen …
locations-add = Legg til plassering
locations-cancel = Avbryt

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Kunne ikke laste biblioteksplasseringene
locations-takes-and-rest = { $kinds } og alt annet
locations-takes-rest = Alt de andre ikke tar
locations-takes-everything = Alt
locations-takes-nothing = Ingenting ennå
locations-save-failed = Kunne ikke lagre plasseringen
locations-unwatch-title = Slutte å overvåke { $name }?
locations-unwatch-body =
    { $count ->
        [one] Den { $count } pakken
       *[other] De { $count } pakkene
    } forsvinner fra Hoard, med tagger og notater. Filene i mappen røres ikke.
locations-unwatch-body-empty = Filene i mappen røres ikke.
locations-unwatch = Slutt å overvåke
locations-remove-title = Fjerne { $name }?
locations-remove-body = Eventuelle filer i den flyttes først til de andre plasseringene, etter type; den forsvinner når den er tom. Ingenting går tapt.
locations-remove-body-only = Hvis den inneholder filer, må du først legge til en annen plassering de kan flyttes til.
locations-remove = Fjern
locations-removed = Fjernet { $name }
locations-emptying = Flytter filene ut av { $name }; den fjernes når det er gjort
locations-offline-title = { $name } kan ikke nås
locations-offline-body = Filene kan ikke flyttes ut mens den er frakoblet. Vil du fjerne den likevel? Filer som fortsatt ligger der, vises som manglende.
locations-remove-anyway = Fjern likevel
locations-remove-failed = Kunne ikke fjerne { $name }
locations-rebalancing = Flytter lagrede filer til plasseringene som tar typen deres
locations-rebalance-failed = Kunne ikke flytte filene
locations-scanned-never = ikke skannet ennå
locations-scanned-now = skannet akkurat nå
locations-scanned-minutes = skannet for { $mins } min siden
locations-scanned-hours = skannet for { $hours } t siden
locations-scanned-date = skannet { $date }
locations-scan-failed = Kunne ikke skanne { $name }
locations-watching = Overvåker { $name }
locations-added = La til { $name }
locations-loading = Laster …
locations-managed-title = Biblioteker Hoard tar vare på
locations-watched-title = Overvåkede mapper
locations-managed-hint = Opplastinger og importer kopieres inn her. Hver plassering tar typene den har krysset av for; oppsamlingsplasseringen tar resten.
locations-watched-hint = Mapper med pakker som indekseres der de ligger. Hoard endrer aldri filene i dem, og ser etter nye og fjernede mapper hvert 15. minutt.
locations-managed-empty = Ingen ennå, så det finnes ikke noe sted å legge en opplasting. Legg til en nedenfor.
locations-reachable = Kan nås
locations-unreachable-now = Kan ikke nås akkurat nå
locations-catch-all-tag = oppsamling
locations-packs =
    { $count ->
        [one] { $count } pakke
       *[other] { $count } pakker
    }
locations-unreachable = kan ikke nås
locations-scanning = Skanner …
locations-scan = Skann nå
locations-edit = Rediger
locations-catch-all-locked = For å endre dette må du gjøre en annen plassering til oppsamlingsplassering
locations-edit-hint-before = Gjelder filer som legges til fra nå av. For å flytte det som allerede er lagret, bruk
locations-edit-hint-after = nedenfor.
locations-save = Lagre
locations-rebalance = Flytt lagrede filer så de stemmer
locations-rebalance-hint = Etter at du har endret hva en plassering tar: filer som allerede er lagret, flyttes til plasseringen typen deres nå hører til.
locations-add-title = Legg til en plassering
locations-add-hint = En annen disk for noen typer filer, eller en mappe med pakker du allerede har.
locations-add-more = Legg til en plassering …

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Kunne ikke laste Problemer
issues-started =
    { $count ->
        [one] Startet på nytt.
       *[other] Startet { $count } ting på nytt.
    }
issues-retry-failed = Kunne ikke prøve igjen
issues-dismiss-failed = Kunne ikke avvise
issues-dismiss-all-title = Avvise alle { $count }?
issues-dismiss-all-body = «{ $title }» tømmes. Alt som går galt igjen, listes opp igjen.
issues-dismiss-all = Avvis alle
issues-just-now = akkurat nå
issues-minutes-ago = for { $mins } min siden
issues-hours-ago = for { $hours } t siden
issues-loading = Laster …
issues-none = Ingenting har gått galt.
issues-action-all = { $action } (alle)
issues-open-asset = Åpne den
issues-open-pack = Vis pakken
issues-open-locations = Biblioteksplasseringer
issues-where-before = i
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } gang
       *[other] { $count } ganger
    }
issues-starting = Starter …
issues-dismiss-named = Avvis { $title }
issues-dismiss = Avvis
issues-show-fewer = Vis færre
issues-show-all = Vis alle { $count }
issues-not-listed = { $count } til er ikke listet opp.

## archive.ts

archive-anyway = Arkiver likevel
archive-failed = Kunne ikke arkivere: { $error }
archive-undo = Angre
archive-restore-failed = Kunne ikke gjenopprette: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    } og { $assets ->
        [one] { $assets } ressurs
       *[other] { $assets } ressurser
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } ressurs
       *[other] { $assets } ressurser
    }
archive-count-nothing = ingenting

## ArchiveView.vue

archive-load-failed = Kunne ikke laste arkivet: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Gjenopprettet { $what }
archive-purged = Slettet { $what } for godt
archive-purge-failed = Kunne ikke slette: { $error }
archive-purge-title = Slette { $name } for godt?
archive-purge-pack-body =
    { $count ->
        [one] Den { $count } ressursen
       *[other] De { $count } ressursene
    } og { $size } med filer slettes også, unntatt filer som en annen pakke deler. Dette kan ikke angres.
archive-purge = Slett for alltid
archive-purge-asset-body = Filen frigjøres med mindre en annen ressurs deler den. Dette kan ikke angres.
archive-empty-title = Tømme arkivet?
archive-empty-body = { $what } slettes for godt, og filene deres frigjøres. Dette kan ikke angres.
archive-empty = Tøm arkivet
archive-title = Arkiv
archive-intro = Arkiverte pakker og ressurser er utenfor søk, tellinger og tagglisten. Gjenopprett dem, eller slett dem for godt for å frigjøre plassen.
archive-loading = Laster …
archive-is-empty = Arkivet er tomt
archive-is-empty-hint = Arkiver en pakke eller en ressurs, så venter den her til du gjenoppretter den eller sletter den for godt.
archive-packs = Pakker
archive-assets = Ressurser
archive-asset-count =
    { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    }
archive-archived = arkivert { $when }
archive-from = fra { $pack }
archive-restore = Gjenopprett
archive-show-more = Vis flere ({ $count } igjen)

## mine.ts

projects-favorite-add-failed = Kunne ikke legge til i favorittene dine: { $error }
projects-favorite-remove-failed = Kunne ikke fjerne fra favorittene dine: { $error }
projects-save-search-failed = Kunne ikke lagre søket: { $error }
projects-forget-search-failed = Kunne ikke fjerne { $name }: { $error }

## projects.ts

projects-list-and = { $items } og { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } til
projects-other =
    { $count ->
        [one] { $count } annet prosjekt
       *[other] { $count } andre prosjekter
    }
projects-usage-title-one = { $name } bruker dette
projects-usage-title = { $count } prosjekter bruker dette
projects-usage-body-one = Brukes i { $list }: neste hoard sync fjerner dem.
projects-usage-body = Brukes i { $list }: neste hoard sync i hvert av dem fjerner dem.

## uploader.ts

upload-network-error = nettverksfeil
upload-gave-up = { $what }: { $error } etter { $attempts } forsøk
upload-failed-plain = mislyktes
upload-already-imported = allerede importert
upload-part-failed = opplastingen mislyktes ({ $status })
upload-batch = parti med { $count } filer
upload-batch-failed = partiet mislyktes ({ $status })
upload-finish = fullfør opplastingen

## UploadQueue.vue

upload-eta-seconds = { $s } s igjen
upload-eta-minutes = { $min } min igjen
upload-eta-hours = { $h } t igjen
upload-queue = Importer
upload-importing = Importerer { $count } pakker
upload-bytes-of = { $sent } av { $total }
upload-rate = { $size }/s
upload-waiting = Venter
upload-unpacking = Lastet opp, pakker nå ut
upload-already = Allerede i biblioteket
upload-stopped = Stoppet
upload-files-of = { $done } av { $total } filer
upload-resumed = gjenopptatt, { $size } var allerede der
upload-skipped = Akkurat disse filene var allerede importert som «{ $name }», så de ble hoppet over.
upload-error = { $error }. Fremdriften din er tatt vare på: prøv igjen, eller slipp de samme filene igjen senere.
upload-retry = Prøv igjen
upload-import-again = Importer på nytt likevel
upload-dismiss = Avvis
upload-cancel = Avbryt

## PacksView.vue

packs-rename-failed = Kunne ikke gi nytt navn: { $error }
packs-collection-deleted = Slettet samlingen { $name }
packs-delete-failed = Kunne ikke slette: { $error }
packs-status-uploading = Laster opp (filer mottatt så langt)
packs-status-ingesting = Pakker ut og tagger
packs-status-analyzing = LLM tagger
packs-status-ready = Klar
packs-status-failed = Mislyktes
packs-retag-failed = Kunne ikke starte ny tagging: { $error }
packs-read-files-failed = Kunne ikke lese pakkens filer: { $error }
packs-context-load-failed = Kunne ikke laste pakkens butikkdetaljer: { $error }
packs-undo-store-failed = Kunne ikke fjerne butikksiden: { $error }
packs-context-save-failed = Kunne ikke lagre butikkdetaljene: { $error }
packs-unreal-load-failed = Kunne ikke laste status for Unreal-konverteringen: { $error }
packs-unreal-discard-title = Forkaste originalene?
packs-unreal-discard-body = De opprinnelige .uasset-filene i { $name } slettes når de er konvertert. Data som bare Unreal bruker, for eksempel materialgrafer, går tapt for godt.
packs-unreal-discard-confirm = Konverter og forkast
packs-unreal-convert-failed = Kunne ikke starte konverteringen: { $error }
packs-unreal-fact-converted = { $done } konvertert fra Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal-pakkefil
       *[other] { $left } Unreal-pakkefiler
    }
packs-unreal-fact-both = { $done } konvertert fra Unreal, { $left } fortsatt pakkefiler
packs-thumbs-canceled = Avbrutt etter { $done } av { $total }
packs-thumbs-drew =
    { $count ->
        [one] Tegnet { $count } modell
       *[other] Tegnet { $count } modeller
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniatyrbilde erstattet.
       *[other] { $lead }: { $improved } miniatyrbilder erstattet.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniatyrbilde erstattet, { $failed } kunne ikke tegnes.
       *[other] { $lead }: { $improved } miniatyrbilder erstattet, { $failed } kunne ikke tegnes.
    }
packs-archived = Arkiverte { $name } og de { $count } ressursene i den
packs-blobs-bad = { $count } filer besto ikke den siste integritetssjekken.
packs-collections = Samlinger
packs-browse-named = Bla gjennom { $name }
packs-collection-name = Navn på samlingen
packs-save = Lagre
packs-cancel = Avbryt
packs-assets =
    { $count ->
        [one] { $count } ressurs
       *[other] { $count } ressurser
    }
packs-updated = Oppdatert { $date }
packs-browse = Bla gjennom
packs-collection-download = Last ned (.zip)
packs-collection-credits = Kreditering
packs-collection-credits-title = Hvem som laget pakkene disse ressursene kommer fra, lisensene og krediteringslinjene de ber om
packs-rename = Gi nytt navn
packs-delete = Slett
packs-packs = Pakker
packs-empty = Ingen pakker ennå. Slipp en zip-fil eller en mappe hvor som helst på denne siden for å legge til en.
packs-find = Finn en pakke
packs-find-placeholder = Finn en pakke etter navn, butikk, lisens eller tagg
packs-sort = Rekkefølge
packs-sort-new = Nyeste først
packs-sort-old = Eldste først
packs-sort-name = Etter navn
packs-sort-size = Største først
packs-sort-assets = Flest ressurser først
packs-with-kind = Pakker med
packs-state = Status
packs-state-any = Alle statuser
packs-state-working = Under arbeid
packs-state-failed = Mislyktes
packs-state-untagged = Ikke beskrevet ennå
packs-store = Butikk
packs-store-any = Alle butikker
packs-store-from = Fra { $store }
packs-store-none = Ingen butikkside
packs-shown-of = { $shown } av { $total }
packs-clear = Fjern
packs-no-match = Ingen pakker passer.
packs-clear-all = Fjern søket og filtrene
packs-pack-name = Navn på pakken
packs-thumbs-drawing = Tegner miniatyrbilder i denne nettleseren …
packs-thumbs-drawing-count = Tegner miniatyrbilder i denne nettleseren: { $done } av { $total } …
packs-thumbs-stopped = Stoppet: { $error }
packs-thumbs-none = Alle modellene her har allerede et miniatyrbilde tegnet med alle teksturene sine.
packs-dismiss = Avvis
packs-license = Lisens: { $license }
packs-license-tier = Lisens: { $license } ({ $tier })
packs-license-not-found = ikke funnet
packs-from-before = Fra
packs-from-after = { "" }
packs-added = Lagt til { $date }
packs-download = Last ned pakken (.zip)
packs-store-details = Butikkdetaljer
packs-more-named = Flere handlinger for { $name }
packs-more = Mer
packs-rename-more = Gi nytt navn …
packs-retag-rules = Kjør den lokale taggeren på nytt
packs-retag-llm = Kjør LLM-taggeren på nytt
packs-make-thumbs = Lag miniatyrbilder i denne nettleseren
packs-make-thumbs-title = Åpne hver modell i en skjult 3D-visning, én om gangen, og bruk bildet som miniatyrbilde der det er bedre enn serverens
packs-read-files = Les pakkens filer på nytt
packs-credits = Last ned kreditering
packs-credits-title = CREDITS.md: hvem som laget pakken, lisensen og krediteringslinjen den ber om
packs-unreal-open = Konverter Unreal-innhold …
packs-archive = Arkiver pakken
packs-unreal-intro = Hoard leser Unreal-pakkefiler selv og lagrer innholdet som vanlige filer: teksturer som PNG eller EXR, lyder som WAV, materialer som JSON. Den konverterte filen tar pakkefilens plass og beholder tagger, lenker og samlinger.
packs-unreal-converted =
    { $count ->
        [one] konvertert
       *[other] konverterte
    }
packs-unreal-converted-kept =
    { $count ->
        [one] konvertert, { $kept } med originalen beholdt
       *[other] konverterte, { $kept } med originalen beholdt
    }
packs-unreal-pending =
    { $count ->
       *[other] ikke sett på ennå
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] venter på Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] uten et vanlig format
    }
packs-unreal-failed =
    { $count ->
       *[other] mislyktes
    }
packs-unreal-reasons = Hvorfor pakkefiler står igjen
packs-unreal-editor-before = Mesher og animasjoner krever Unreal Editor. På en maskin med Unreal installert kjører du
packs-unreal-editor-command = hoard unreal-worker --editor <bane til UnrealEditor-Cmd>
packs-unreal-editor-after = ; den eksporterer dem som FBX og laster opp resultatene hit.
packs-unreal-discard = Forkast de opprinnelige .uasset-filene når de er konvertert
packs-unreal-discard-hint = Frigjør plass. Data som bare Unreal bruker, for eksempel materialgrafer, går tapt.
packs-unreal-drop = Slett pakkefiler uten et vanlig format
packs-unreal-drop-hint = Blueprints, nivåer og andre ressurser som bare motoren kan bruke.
packs-unreal-convert = Konverter
packs-context-hint-before = Hvor pakken kom fra og hva selgeren sier om den. LLM-taggeren leser dette sammen med hver fil, så en butikktekst som «120 ice and fire spells» hjelper den med å navngi
packs-context-hint-after = .
packs-store-auto = Funnet automatisk: pakken nevner selv denne siden.
packs-store-not-this = Ikke denne
packs-lookup-label = Søk i butikkene etter
packs-lookup-placeholder = Pakkens navn i butikken
packs-lookup-searching = Søker …
packs-lookup-search = Søk i butikker
packs-lookup-hide = Skjul
packs-lookup-clues = Fra pakkens filer: { $clues }.
packs-lookup-also = Søkte også etter { $queries }.
packs-lookup-quoted = «{ $query }»
packs-lookup-nothing = Ingenting på Fab, itch.io, Kenney eller OpenGameArt ligner. Prøv færre eller andre ord.
packs-lookup-use = Bruk
packs-lookup-searched = Søkte { $date }.
packs-lookup-again = Søk igjen
packs-lookup-again-after = .
packs-lookup-unreachable = Kunne ikke nå { $stores } denne gangen.
packs-lookup-also-look = Se også på
packs-lookup-links-and = { " " }og{ " " }
packs-lookup-browser-only = (de kan bare søkes i fra nettleseren din); lim inn sidens adresse nedenfor.
packs-lookup-open = Finn i butikker …
packs-store-placeholder = Fab, itch.io, Humble Bundle …
packs-seller = Selger
packs-seller-placeholder = Hvem som laget den
packs-license-label = Lisens
packs-license-placeholder = Fab Standard License, CC BY 4.0 …
packs-license-tier-label = Lisensnivå
packs-license-tier-placeholder = Personal, Professional …
packs-store-page = Butikkside
packs-store-text = Beskrivelse fra butikken
packs-store-text-placeholder = Lim inn teksten fra oppføringen, eller la feltet stå tomt for å hente den fra butikksiden.
packs-store-text-fetch = Når du lagrer, hentes beskrivelsen fra butikksiden.
packs-readme = README funnet i pakken
packs-save-retag = Lagre og kjør LLM-taggeren på nytt

## AssetDetail.vue

asset-lazy-3d-viewer = 3D-visningen
asset-lazy-text = teksten
asset-lazy-text-viewer = tekstvisningen
asset-lazy-image-viewer = bildevisningen
asset-lazy-material-preview = materialforhåndsvisningen
asset-lazy-material-viewer = materialvisningen
asset-lazy-sprite-player = spritespilleren
asset-lazy-sheet-view = spritearkvisningen
asset-rel-texture = tekstur
asset-rel-buffer = geometridata
asset-rel-material = materialfil
asset-rel-animation = animasjon
asset-rel-library = lenket fil
asset-rel-guess = hører trolig til
asset-rel-sidecar = butikkdetaljer
asset-rel-variant = variant
asset-role-map = { $role }-kart
asset-maps-count =
    { $count ->
        [one] { $count } kart
       *[other] { $count } kart
    }
asset-needs = Trenger
asset-animations = Animasjoner
asset-sprites = Sprites
asset-used-by = Brukes av
asset-conversion-failed = konverteringen mislyktes
asset-as-converting = Konverterer …
asset-as-converting-percent = Konverterer … { $percent } %
asset-as-queued = I kø …
asset-download = Last ned
asset-as-label = Last ned som
asset-as-original = { $format } (opprinnelig format)
asset-as-size = Størrelse
asset-as-full-size = Full størrelse
asset-as-up-to = Opptil { $size } px
asset-as-hint = Velg en størrelse eller et annet format
asset-meta-size = Størrelse
asset-meta-length = Lengde
asset-meta-sample-rate = Samplingsfrekvens
asset-meta-channels = Kanaler
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bitdybde
asset-meta-bits = { $bits }-bit
asset-meta-level = Nivå
asset-meta-level-value = topp { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = topp { $peak } dBFS, RMS { $rms } dBFS, klipper
asset-meta-loop = Løkke
asset-meta-loop-value = { $start } til { $end }
asset-meta-playback = Avspilling
asset-meta-one-shot = Enkeltavspilling
asset-meta-tempo = Tempo
asset-meta-root-note = Grunntone
asset-meta-cue-markers = Cue-markører
asset-meta-library = Bibliotek
asset-meta-keywords = Nøkkelord
asset-meta-title = Tittel
asset-meta-artist = Artist
asset-meta-album = Album
asset-meta-genre = Sjanger
asset-meta-generator = Generator
asset-meta-everything-with = Vis alt med taggen { $tag }
asset-meta-saved-with = Lagret med
asset-meta-color-mode = Fargemodus
asset-meta-layers = Lag
asset-meta-range = Område
asset-meta-hdr = HDR (forhåndsvisningen er tonemappet)
asset-meta-format = Format
asset-meta-large-document = Stort dokument (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Laget med generativ AI
asset-meta-ai-edited = Redigert med generativ AI
asset-meta-ai-how-metadata = metadata
asset-meta-ai-how-generator = generatorinnstillinger
asset-meta-ai-how-iptc = IPTC digital kildetype
asset-meta-ai-says = { $made }, ifølge { $how }
asset-meta-ai-by-says = { $made } ({ $by }), ifølge { $how }
asset-meta-provenance = Opprinnelse
asset-meta-c2pa = Har C2PA Content Credentials
asset-meta-compression = Komprimering
asset-meta-note = Merknad
asset-meta-maps = Kart
asset-meta-sizes =
    { $count ->
        [one] Størrelse
       *[other] Størrelser
    }
asset-meta-triangles = Trekanter
asset-meta-vertices = Hjørnepunkter
asset-meta-meshes = Mesher
asset-meta-materials = Materialer
asset-meta-animations = Animasjoner
asset-meta-skins = Skins
asset-meta-unreal-asset = Unreal-ressurs
asset-meta-unreal-saved-with = { $class }, lagret med UE { $engine }
asset-meta-package = Pakkefil
asset-meta-cooked = Cooked (spillbygg): forhåndsvisninger og kilder er fjernet
asset-meta-tiling = Flislegging
asset-meta-seamless = Sømløs på begge akser
asset-meta-analysis-error = Kunne ikke lese hele denne filen: { $error }
asset-copy-blocked = Kunne ikke kopiere: nettleseren blokkerte det
asset-save-error = Kunne ikke lagre: { $error }
asset-tag-removed = Fjernet { $tag }
asset-undo = Angre
asset-play-error = Kunne ikke spille av denne filen: { $error }
asset-copied-hash = Kopierte hashen
asset-copied-toml = Kopierte toml
asset-copied-id = Kopierte ID-en
asset-copied-link = Kopierte lenken
asset-origin-before = Denne filen var
asset-origin-class-engine = , en { $class } lagret av UE { $engine }.
asset-origin-class = , en { $class }.
asset-origin-engine = { " " }lagret av UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor eksporterte den { $date }.
asset-origin-editor-notes = Unreal Editor eksporterte den { $date } ({ $notes }).
asset-origin-hoard = Hoard leste den ut av pakkefilen { $date }.
asset-origin-hoard-notes = Hoard leste den ut av pakkefilen { $date } ({ $notes }).
asset-origin-download = Last ned den opprinnelige .{ $ext }-filen
asset-origin-discarded = Den opprinnelige pakkefilen ble forkastet etter konverteringen.
asset-details = Ressursdetaljer
asset-in-pack = i
asset-previous = Forrige ressurs
asset-next = Neste ressurs
asset-favorite = Favoritt
asset-favorite-on-title = I favorittene dine (F fjerner den)
asset-favorite-off-title = Legg til i favorittene dine (F)
asset-share = Del
asset-share-title = Del en lenke til denne ressursen (den som åpner den, logger først inn i Hoard)
asset-layout-panel = Vis i et sidepanel
asset-layout-page = Vis som hel side
asset-layout-page-title = Vis som hel side: en stor forhåndsvisning ved siden av detaljene
asset-close = Lukk detaljene
asset-rename-title = Klikk for å gi nytt navn
asset-name = Navn
asset-views-alt = Modellen fra fire retninger
asset-inspect-aria = Inspiser { $name } i fullskjerm
asset-inspect-title = Inspiser: zoom, flislegging, kanaler
asset-playback-position = Avspillingsposisjon
asset-playback-value = { $current } av { $duration }
asset-pause = Pause
asset-play = Spill av
asset-waveform-drawing = Tegner bølgeform …
asset-waveform-skipped = Ingen bølgeform for så store filer
asset-play-animation = Spill av som animasjon
asset-show-views = Vis arket med fire visninger som taggeren så
asset-palette = Farger i bildet
asset-swatch-title = { $hex }, { $percent } % av bildet. Finn ressurser med denne fargen
asset-swatch-aria = Finn ressurser med { $hex }, { $percent } % av dette bildet
asset-more-like-this = Mer som dette
asset-desc-label = Din beskrivelse
asset-desc-placeholder = Hva det er, hvor du ville brukt det …
asset-save = Lagre
asset-cancel = Avbryt
asset-desc-family = Lagres for alle { $count } variantene
asset-desc-edit = Rediger beskrivelsen
asset-desc-write = Skriv en beskrivelse
asset-desc-ai = Skrevet av AI
asset-desc-file = Fra filen
asset-desc-store-before = Fra
asset-desc-store-page = { $store }-siden
asset-desc-store-page-unnamed = butikksiden
asset-desc-store-after = (hele pakken)
asset-less = Mindre
asset-more = Mer
asset-tags = Tagger
asset-tag-add-label = Legg til en tagg
asset-tag-add-placeholder = Legg til en tagg, f.eks. mood:tense
asset-tag-add = Legg til
asset-apply-family = Bruk endringer i tagger, kategori og beskrivelse på alle { $count } variantene
asset-collections = Samlinger
asset-collection-remove = Fjern fra { $name }
asset-collection-add-another = Legg til i en annen
asset-collection-add = Legg til i samling
asset-used-in = Brukes i
asset-used-in-title = Vis hva { $name } har
asset-used-in-synced = synkronisert { $when }
asset-maps = Kart
asset-show-more = Vis { $count } til
asset-materials-heading =
    { $count ->
        [one] Materiale
       *[other] Materialer
    }
asset-material-maps-aria = { $name }: kart
asset-variants = Varianter ({ $count } versjoner)
asset-copies = Identiske kopier
asset-copies-note = De samme bytene lagres én gang; disse pakkene inneholder også denne filen.
asset-unreal-converted = Konvertert fra Unreal
asset-rule-converted = Konvertert av en regel
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = En konverteringsregel laget denne { $format }-filen { $date } fra
asset-rule-kept-until = Originalen beholdes til { $date }.
asset-rule-put-back = Legg tilbake originalen
asset-rule-put-back-title = Legge tilbake { $file }?
asset-rule-put-back-body = Den konverterte filen slettes, og originalen tar plassen sin tilbake, med tagger, samlinger og lenker. Konverteringsreglene lar den være i fred fra da av.
asset-rule-put-back-done = { $file } er tilbake
asset-rule-let-go = Originalen ble slettet da tiden Innstillinger beholder originaler i, var ute.
asset-unreal-package = Fra Unreal-pakkefilen
asset-ue-needs-editor-before = Venter på Unreal Editor: { $reason }. På en maskin med Unreal installert kjører du
asset-ue-needs-editor-after = så kommer den tilbake som FBX.
asset-ue-editor-path = bane til UnrealEditor-Cmd
asset-ue-unsupported = Beholdt som pakkefil: { $reason }.
asset-ue-failed = Konverteringen mislyktes: { $reason }
asset-recovered = Den opprinnelige .{ $ext }-filen var lagret i denne pakkefilen, og Hoard hentet den ut.
asset-recovered-download = Last ned .{ $ext }
asset-ue-meshes-before = Mesher og animasjoner lagres i Unreals eget format. Konverter pakken (pakkesiden, Mer), og kjør deretter
asset-ue-meshes-after = ved siden av en Unreal Editor-installasjon for å få FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } asset registry-tagg
       *[other] { $count } asset registry-tagger
    }
asset-file = Fil
asset-bytes = Byte
asset-source = Kilde
asset-path = Bane
asset-download-maps =
    { $count ->
        [one] Last ned { $count } kart
       *[other] Last ned alle { $count } kart
    }
asset-download-linked =
    { $count ->
        [one] Last ned med { $count } lenket fil
       *[other] Last ned med { $count } lenkede filer
    }
asset-download-only-title = Bare denne filen, uten filene den bruker
asset-download-only = Bare denne filen
asset-include-animations = Ta med animasjoner i .zip-filen
asset-copy-toml = Kopier hoard.toml-oppføring
asset-copy-id = Kopier ID
asset-teach = Lær Hoard denne filen
asset-teach-title = Beskriv denne butikkens fil slik at Hoard leser den (Innstillinger → Butikkfiler)
asset-archive = Arkiver
asset-archive-title = Utenfor søk til den gjenopprettes eller slettes for godt, fra Arkiv

## ImageViewer.vue

image-channel-red = Rød kanal
image-channel-green = Grønn kanal
image-channel-blue = Blå kanal
image-channel-alpha = Alfakanal
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Rutete bakgrunn (K bytter)
image-bg-black-title = Svart bakgrunn (K bytter)
image-bg-gray-title = Grå bakgrunn (K bytter)
image-bg-white-title = Hvit bakgrunn (K bytter)
image-bg-checker = rutete bakgrunn
image-bg-black = svart bakgrunn
image-bg-gray = grå bakgrunn
image-bg-white = hvit bakgrunn
image-inspect-aria = Inspiser { $name }
image-preview-size = forhåndsvisning { $width }×{ $height }, { $bytes }
image-preview = forhåndsvisning
image-resolution = Oppløsning
image-full-title = Last inn hele bildet på { $width }×{ $height } (F); det gjør også det å zoome inn forbi forhåndsvisningen
image-full-loading = Laster full oppløsning …
image-full-failed = Full oppløsning ble ikke lastet: prøv igjen
image-full = Full oppløsning
image-zoom = Zoom
image-zoom-out-title = Zoom ut (−)
image-zoom-out = Zoom ut
image-zoom-in-title = Zoom inn (+)
image-zoom-in = Zoom inn
image-fit-title = Tilpass (0)
image-fit = Tilpass
image-actual-title = Faktiske piksler (1)
image-view = Visning
image-tile-title = Flislegg 3×3 for å sjekke skjøtene (T)
image-tile = Flislegg
image-channels = Kanaler
image-close = Lukk (Esc)
image-failed = Dette bildet kunne ikke lastes.
image-loading = Laster …
image-key-gestures = Rull eller knip for å zoome · dra for å panorere · dobbeltklikk for å zoome inn eller tilpasse
image-key-tile = flislegg
image-key-background = bakgrunn
image-key-full = full oppløsning
image-key-channels = kanaler

## SpritePlayer.vue

sprite-frame-aria = { $name }, bilde { $frame } av { $count }
sprite-failed = Denne kan ikke spilles av.
sprite-loading = Laster bilder …
sprite-previous = Forrige bilde
sprite-next = Neste bilde
sprite-pause = Sett animasjonen på pause
sprite-play = Spill av animasjonen
sprite-animation = Animasjon
sprite-all-frames = Alle bilder
sprite-grid-title = Hvordan arket deles opp i bilder
sprite-columns = Kolonner
sprite-rows = Rader
sprite-fps-aria = Bilder per sekund
sprite-fps = fps

## Spritearkvisningen (SheetView.vue) og nedlastingsformatene (AssetDetail.vue)

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
        [one] { $count } animasjon
       *[other] { $count } animasjoner
    }
# The box that narrows the list of sprites by name.
sheet-find = Finn en sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Viser { $shown } av { $total }
sheet-no-match = Ingen sprite har det i navnet.
sheet-failed = Dette arket kan ikke leses.
sheet-loading = Laster arket …
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoards eget format: sidene som de er, sprites rotert hvilken vei som helst og beskåret, med animasjoner og varigheten til hvert bilde.
sheet-note-texturepacker = Sprites rotert med klokken og beskåret, med animasjoner etter navn, slik PixiJS og Phaser leser dem.
sheet-note-clockwise = Sprites rotert med klokken og beskåret. Det kan ikke inneholde animasjoner: bildene følger de nummererte navnene sine.
sheet-note-libgdx = Sprites rotert mot klokken og beskåret. Det kan ikke inneholde animasjoner: bildene følger de nummererte navnene sine.
sheet-note-godot-frames = Sprites roteres aldri, beskjæringen beholdes som marger, med animasjoner og varigheten til hvert bilde.
sheet-note-godot-tileset = Fliser roteres og beskjæres aldri, de legges på et rutenett i flisstørrelsen, og bildene beholdes som de er når de allerede ligger på et.
sheet-note-grid = Hver sprite oppreist på hele lerretet sitt, i celler som alle er like store, for alt som deler et ark opp i et rutenett.
sheet-note-frames = Hver sprite oppreist på hele lerretet sitt, hver som en PNG med spritens navn.

## TextViewer.vue, TextPreview.vue og text.ts

text-lang-config = Konfigurasjon
text-lang-table = Tabell
text-lang-code = Kode
text-lang-script = Skript
text-lang-text = Tekst
text-copy-blocked = Kunne ikke kopiere: nettleseren blokkerte det
text-read-aria = Les { $name }
text-lines =
    { $count ->
        [one] { $count } linje
       *[other] { $count } linjer
    }
text-show-as = Vis som
text-tree-title = Sammenleggbart tre (T)
text-tree = Tre
text-pretty-title = Nytt innrykk (P)
text-pretty = Pen
text-raw-title = Slik det er skrevet (R)
text-raw = Rå
text-document-title = Som dokument (D)
text-document = Dokument
text-source-title = Markdown slik det er skrevet (R)
text-source = Kilde
text-fold = Legg sammen
text-expand-all-title = Fold ut alt
text-expand-all = Utvid alle
text-collapse-all-title = Legg sammen alt
text-collapse-all = Skjul alle
text-group-text = Tekst
text-wrap-title = Bryt lange linjer (W)
text-wrap = Bryt
text-copy-title = Kopier teksten (C)
text-copied = Kopiert
text-copy = Kopier
text-download-title = Last ned filen
text-download = Last ned
text-close = Lukk (Esc)
text-truncated = Viser de første { $shown } av { $total }.
text-truncated-json = Viser de første { $shown } av { $total } (treet trenger hele filen).
text-show-all = Vis alt
text-json-error = Denne JSON-en kan ikke tolkes: { $error }
text-contents-aria = Innholdet i { $name }
text-loading = Laster …
text-load-error = Kunne ikke laste denne filen: { $error }
text-binary = Denne filen er binær, ikke tekst, så det er ingenting å lese her. Last den ned for å åpne den i sitt eget verktøy.
text-key-tree = tre
text-key-pretty = pen
text-key-raw = rå
text-key-document = dokument
text-key-source = kilde
text-key-wrap = bryt
text-key-copy = kopier
text-key-find = finn
text-key-close = lukk
text-preview-aria = Les { $name } i fullskjerm
text-preview-read = Les

## JsonTree.vue og JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } element
       *[other] { $count } elementer
    }
json-keys =
    { $count ->
        [one] { $count } nøkkel
       *[other] { $count } nøkler
    }
json-fold = Legg sammen
json-unfold = Fold ut
json-unfold-summary = Fold ut { $summary }
json-show-more = Vis { $count } til
json-left = av { $count } igjen
json-picker-more = … { $count } til

## 3D-modellvisninger (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = 3D-visningen ble ikke lastet ({ $error }); det kan hjelpe å laste siden på nytt
model-loading-viewer = Laster 3D-visningen …
model-loading = Laster modellen …
model-show-failed = Kunne ikke vise denne modellen: { $error }
model-preview-failed = Kunne ikke forhåndsvise denne modellen: { $error }
model-error-converting = serveren konverterer fortsatt denne modellen
model-error-empty-collada = tom Collada-fil
model-error-no-loader = Ingen nettleserlaster for .{ $ext }
model-dialog = Vis { $name } i 3D
model-title-stats =
    { $triangles } trekanter · { $meshes ->
        [one] { $meshes } mesh
       *[other] { $meshes } mesher
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
model-group-shading = Skyggelegging
model-group-background = Bakgrunn
model-group-animation = Animasjon
model-group-view = Visning
model-group-show = Vis
model-frame-title = Ramme inn modellen (F)
model-frame = Ramme inn
model-reset-title = Tilbake til startvisningen (R)
model-reset = Tilbakestill
model-turn-title = Roter sakte (A)
model-turn = Roter
model-textures-title = Teksturer (T)
model-textures = Teksturer
model-full-res-title = Teksturer i full oppløsning (H): hver teksturfil som den er, uansett størrelse; av gir lette forhåndsvisninger på maks 2048 px
model-full-res = Full oppl.
model-light-title = Miljølys (E)
model-light = Lys
model-grid-title = Gulvrutenett (G)
model-grid = Rutenett
model-bg-checker = Rutete bakgrunn
model-bg-dark = Mørk bakgrunn
model-bg-gray = Grå bakgrunn
model-bg-light = Lys bakgrunn
model-bg-env = Miljøbakgrunn
model-bg-title = { $background } (K bytter)
model-pause-title = Pause (Mellomrom)
model-play-title = Spill av (Mellomrom)
model-pause = Pause
model-play = Spill av
model-animation-clip = Animasjonsklipp
model-animation = Animasjon
model-plays-on = Spilles av på
model-close-title = Lukk (Esc)
model-full-screen = Fullskjerm
model-open-full-screen = Åpne 3D-visningen i fullskjerm
model-menu-fold = Legg bort menyen
model-menu-show = Vis menyen
model-view-prefix = Visning:{ " " }
model-menu-render = Rendering
model-menu-maps = Kart
model-menu-inspect = Inspiser
model-mode-final = Endelig rendering
model-mode-raw = Rå rendering
model-mode-color = Grunnfarge
model-mode-roughness = Ruhet
model-mode-metalness = Metallisk
model-mode-normal = Normal
model-mode-ao = Okklusjon
model-mode-matcap = Matcap
model-mode-uv = UV-rutemønster
model-mode-geometry = Geometrier
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Trådmodell (W), over hvilken som helst visning
model-wireframe = Trådmodell
model-info-title = Sceneinformasjon (I)
model-info = Sceneinformasjon
model-info-close = Lukk sceneinformasjonen
model-info-close-title = Lukk (I)
model-info-geometries = Geometrier
model-info-materials = Materialer
model-info-textures = Teksturer
model-info-triangles = Trekanter
model-info-vertices = Hjørnepunkter
model-info-meters = glTF er i meter
model-info-no-units = Dette formatet sier ikke hvilke enheter det bruker
model-info-height = Høyde
model-info-length = Lengde
model-info-width = Bredde
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } enhet
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } enheter
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Fant ikke teksturen: { $name }
model-missing-some = Fant ikke { $count } teksturer: { $names }
model-missing-more = Fant ikke { $count } teksturer: { $names } …
model-missing-count = Fant ikke { $count } teksturer
model-guessed = Teksturene er matchet etter navn; filen nevner ingen.
model-group-facing = Retning
model-facing-turn = Roter en kvart
model-facing-turn-title = Roter modellen en kvart omdreining mot klokken, sett ovenfra
model-facing-tip = Vipp en kvart
model-facing-tip-title = Vipp toppen av modellen en kvart omdreining fremover, for å reise opp en som ligger
model-facing-save = Bruk for pakken
model-facing-save-title = Roter alle modellene i { $pack } slik. Miniatyrbildene deres tegnes på nytt; nedlastinger blir som de ble laget.
model-facing-undo = Angre
model-facing-undo-title = Tilbake til slik pakkens modeller er rotert
model-facing-saved = Alle modellene i { $pack } vender nå slik. Miniatyrbildene deres tegnes på nytt.
model-facing-failed = Kunne ikke rotere pakkens modeller: { $error }
model-keys-mouse = Dra for å bane rundt · høyredra for å panorere · rull for å zoome · dobbeltklikk for å ramme inn
model-key-frame = ramme inn
model-key-reset = tilbakestill
model-key-view = visning
model-key-wireframe = trådmodell
model-key-info = sceneinformasjon
model-key-textures = teksturer
model-key-full-res = teksturer i full oppløsning
model-key-light = lys
model-key-grid = rutenett
model-key-background = bakgrunn
model-key-play = spill av

## Materialer (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Farge
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Ruhet
material-role-gloss = Glans
material-role-metalness = Metallisk
material-role-ao = Ambient occlusion
material-role-orm = Okklusjon, ruhet, metall
material-role-mra = Metall, ruhet, okklusjon
material-role-rma = Ruhet, metall, okklusjon
material-role-height = Høyde
material-role-bump = Bump
material-role-cavity = Cavity
material-role-specular = Spekulær
material-role-opacity = Opasitet
material-role-emissive = Emissiv
material-role-translucency = Gjennomskinnelighet
material-role-base-color-name = farge
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = ruhet
material-role-gloss-name = glans
material-role-metalness-name = metallisk
material-role-ao-name = ambient occlusion
material-role-orm-name = okklusjon, ruhet, metall
material-role-mra-name = metall, ruhet, okklusjon
material-role-rma-name = ruhet, metall, okklusjon
material-role-height-name = høyde
material-role-bump-name = bump
material-role-cavity-name = cavity
material-role-specular-name = spekulær
material-role-opacity-name = opasitet
material-role-emissive-name = emissiv
material-role-translucency-name = gjennomskinnelighet
material-maps =
    { $count ->
        [one] { $count } kart
       *[other] { $count } kart
    }
material-dialog = Vis materialet { $name }
material-up-to = opptil { $size }
material-repeat-size = { $size } per repetisjon
material-shape-sphere = Kule
material-shape-cube = Kube
material-shape-plane = Plan
material-shape-title = { $shape } ({ $key })
material-group-shape = Form
material-group-tiling = Flislegging
material-real-title = Virkelig størrelse: én repetisjon er { $size }, kulen er 2 m i diameter (T bytter)
material-repeat-title = Gjenta { $n }× (T bytter)
material-real = Virkelig
material-repeat = { $n }×
material-height-title = Hvor langt høydekartet skyver overflaten
material-height = Høyde
material-height-strength = Høydestyrke
material-loading = Laster materialet …
material-show-failed = Kunne ikke vise dette materialet: { $error }
material-preview-failed = Kunne ikke forhåndsvise dette materialet: { $error }
material-group-maps = Kart
material-map-show = Vis kartet for { $map }
material-map-hide = Skjul kartet for { $map }
material-map-unused = Brukes ikke av forhåndsvisningen
material-open-full-screen = Åpne materialvisningen i fullskjerm
material-keys-mouse = Dra for å bane rundt · høyredra for å panorere · rull for å zoome · klikk på et kart for å slå det av
material-key-shape = form
material-key-tiling = flislegging
material-key-turn = roter

## theme.ts

theme-slate = Skifer
theme-paper = Papir
theme-linen = Lin
theme-sage = Salvie
theme-lilac = Syrin
theme-blush = Rødme
theme-graphite = Grafitt
theme-obsidian = Obsidian
theme-ember = Glør
theme-moss = Mose
theme-aubergine = Aubergine
theme-oxblood = Oksblod
theme-light-slate-blurb = Kjølig blågrå
theme-light-paper-blurb = Enkel nøytral grå
theme-light-linen-blurb = Varm krem, rav
theme-light-sage-blurb = Myk grønn
theme-light-lilac-blurb = Blek fiolett
theme-light-blush-blurb = Støvete rosa
theme-dark-graphite-blurb = Nøytral grå, uten fargetone
theme-dark-slate-blurb = Blågrå
theme-dark-obsidian-blurb = Nesten svart, for OLED
theme-dark-ember-blurb = Varm kullgrå, rav
theme-dark-moss-blurb = Skoggrønn-svart
theme-dark-aubergine-blurb = Plommesvart, syrin
theme-dark-oxblood-blurb = Vinsvart, korall

## texnames.ts

texnames-needs-map = Mønsteret må ha { "{" }map{ "}" }, ellers ville alle kartene til et materiale få samme navn
texnames-placeholders = Mønsteret kan bruke { "{" }name{ "}" }, { "{" }map{ "}" } og { "{" }size{ "}" }
texnames-bad-characters = Navn kan ikke inneholde < > : " / \ | ? eller *
texnames-no-suffix = Gi teksturer av typen { $role } et suffiks
texnames-bad-suffix = Suffikset for teksturer av typen { $role } kan ikke inneholde < > : " / \ | ? eller *
texnames-slot-no-suffix = Gi «{ $slot }» et suffiks
texnames-slot-bad-suffix = Suffikset for «{ $slot }» kan ikke inneholde < > : " / \ | ? eller *
texnames-ending-missing = Hver egen endelse trenger endelsen det skal ses etter
texnames-ending-bad = Egne endelser kan ikke inneholde < > : " / \ | ? eller *
texnames-ending-twice = { $ending } står to ganger blant endelsene dine
texnames-slot-metallic-roughness = Metall og ruhet
texnames-slot-specular-color = Spekulær farge
texnames-slot-specular-glossiness = Spekulær og glans
texnames-slot-clearcoat = Klarlakk
texnames-slot-clearcoat-roughness = Klarlakkruhet
texnames-slot-clearcoat-normal = Klarlakknormal
texnames-slot-sheen-color = Sheen-farge
texnames-slot-sheen-roughness = Sheen-ruhet
texnames-slot-transmission = Transmisjon
texnames-slot-thickness = Tykkelse
texnames-slot-diffuse-transmission = Diffus transmisjon
texnames-slot-diffuse-transmission-color = Diffus transmisjonsfarge
texnames-slot-iridescence = Iridescens
texnames-slot-iridescence-thickness = Iridescenstykkelse
texnames-slot-anisotropy = Anisotropi

## storeFormats.ts

storefiles-field-title = Tittel
storefiles-field-title-hint = Den nøyaktige tittelen på oppføringen. Den gir pakken navn, og uten den hoppes filen over.
storefiles-field-seller = Selger
storefiles-field-seller-hint = Hvem som har laget eller selger den.
storefiles-field-license = Lisens
storefiles-field-license-hint = Slik filen skriver den, for eksempel CC-BY-4.0.
storefiles-field-url = Butikkside
storefiles-field-url-hint = Adressen til oppføringen, som begynner med http:// eller https://.
storefiles-field-words = Tagger
storefiles-field-words-hint = Ord den skal tagges med. Kommadelte verdier deles opp.
storefiles-field-type = Type
storefiles-field-type-hint = Hva det er, for eksempel «3D model» eller «surface». Også en tagg.
storefiles-field-colors = Farger
storefiles-field-colors-hint = Fargene, som color:-tagger.
storefiles-field-id = Butikk-ID
storefiles-field-id-hint = Butikkens ID for den: bokstaver, sifre, _ eller -.
storefiles-field-origin = Laget i
storefiles-field-origin-hint = Hvor den ble laget eller skannet, for beskrivelsen.
storefiles-field-size = Størrelse
storefiles-field-size-hint = Hvor stor den er, i ord, for beskrivelsen.
storefiles-field-title-name = tittelen
storefiles-field-seller-name = selgeren
storefiles-field-license-name = lisensen
storefiles-field-url-name = butikksiden
storefiles-field-words-name = taggene
storefiles-field-type-name = typen
storefiles-field-colors-name = fargene
storefiles-field-id-name = butikk-ID-en
storefiles-field-origin-name = opprinnelsesstedet
storefiles-field-size-name = størrelsen

## StoreFormatsPanel.vue

storefiles-load-failed = Kunne ikke laste butikkfilformatene: { $error }
storefiles-remove-title = Fjerne { $name }?
storefiles-remove-body = { $file } slettes fra datamappen. Pakker som allerede er lest med det, beholder det som ble lest.
storefiles-remove = Fjern
storefiles-removed = Fjernet { $name }.
storefiles-remove-failed = Kunne ikke fjerne { $name }: { $error }
storefiles-unread = Filer Hoard ikke leser
storefiles-loading = Laster …
storefiles-unread-none = Ingen: ingen fil i biblioteket ser ut som en ulest butikkfil.
storefiles-unread-what = Filer i mer enn én pakke, eller med navn som en butikkfil, som ikke noe format leser.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pakke
       *[other] { $count } pakker
    }
storefiles-unread-example = f.eks. { $pack }
storefiles-teach = Lær Hoard denne filen
storefiles-formats = Formater
storefiles-formats-what = Dine prøves først og erstatter et innebygd format med samme navn.
storefiles-dir-before = De ligger i
storefiles-dir-after = .
storefiles-yours = Dine
storefiles-built-in = Innebygd
storefiles-edit = Rediger
storefiles-start-from = Start fra denne

## TagsPanel.vue

tags-load-failed = Kunne ikke laste taggene: { $error }
tags-rename-title = Gi { $tag } nytt navn?
tags-merge-title = Slå sammen { $count } tagger?
tags-rename-body = De { $count } ressursene får { $into } i stedet, og taggerne bruker det nye navnet fra nå av.
tags-merge-body = { $tags } blir til { $into } på opptil { $count } ressurser. Taggerne bruker { $into } fra nå av.
tags-rename = Gi nytt navn
tags-merge = Slå sammen
tags-merged =
    { $count ->
        [one] { $count } ressurs er nå tagget { $into }
       *[other] { $count } ressurser er nå tagget { $into }
    }
tags-merge-failed = Kunne ikke slå sammen: { $error }
tags-forget-failed = Kunne ikke endre det: { $error }
tags-find = Finn tagger
tags-find-placeholder = Finn tagger, f.eks. rock eller set:
tags-count =
    { $count ->
        [one] { $count } tagg
       *[other] { $count } tagger
    }
tags-count-more = { $count }+ tagger
tags-what = Velg tagger som betyr det samme og slå dem sammen, eller velg én for å gi den nytt navn.
tags-new-name = Nytt navn
tags-merge-into = Slå sammen til
tags-saved-as-before = Lagret som
tags-saved-as-after = { "" }
tags-cancel = Avbryt
tags-loading = Laster …
tags-none = Ingen tagger.
tags-none-containing = Ingen tagger som inneholder «{ $text }».
tags-browse-title = Bla gjennom ressurser tagget { $tag }
tags-browse = Bla gjennom
tags-merged-heading = Sammenslåtte tagger
tags-merged-what = Når taggerne kommer opp med et av disse gamle navnene, får ressursen taggen den ble slått sammen til.
tags-allow-old = Tillat det gamle navnet igjen

## LibraryStats.vue

stats-source-human = Lagt til av personer
stats-source-ai = Foreslått av AI
stats-source-rule = Innebygde regler
stats-source-meta = Fra filene
stats-assets = Ressurser
stats-packs = Pakker
stats-on-disk = På disk
stats-duplicates-title = { $count } filer er byte-identiske kopier av andre og lagres én gang
stats-saved-dedup = Spart med deduplisering
stats-distinct-tags = Ulike tagger
stats-archive = I arkivet
stats-load-failed = Kunne ikke laste detaljene: { $error }
stats-counting = Teller …
stats-by-kind = Plass etter type
stats-files-size = { $name }: { $count } filer, { $size }
stats-months = Lagt til det siste året
stats-month = { $month }: { $count } ressurser, { $size }
stats-tagging = Tagging
stats-tagged = Tagget
stats-untagged = { $count } uten tagger
stats-described = Beskrevet
stats-described-assets = { $count } ressurser
stats-human-edited = Redigert for hånd
stats-human-edited-unit =
    { $count ->
        [one] ressurs
       *[other] ressurser
    }
stats-source-different = { $count } ulike
stats-most-used = Mest brukt
stats-show-tag = Vis ressursene med { $tag }
stats-file-types = Filtyper
stats-biggest-packs = Største pakker
stats-browse-pack = Bla gjennom { $name }
stats-largest-files = Største filer
stats-open-file = Åpne { $name }
stats-empty = Ingenting i biblioteket ennå. Legg til en pakke, så dukker tallene opp her.

## UpdatesPanel.vue

updates-saved = Hoard kaller den når en oppdatering er klar
updates-saved-none = Ingen adresse å kalle
updates-save-failed = Kunne ikke lagre
updates-call-answered = Den svarte { $result }
updates-call-refused = Den tok ikke imot forespørselen: { $result }
updates-call-failed = Kunne ikke kalle den
updates-called = Kalte { $when } for { $version }: { $result }
updates-called-failed = Kalte { $when } for { $version }: { $result }. Hoard prøver igjen om en time.
updates-latest = Hoard { $version } er den nyeste versjonen.
updates-out = Hoard { $version } er ute.
updates-out-idle = Hoard { $version } er ute; kaller innen et par minutter, mens ingenting pågår.
updates-out-busy = Hoard { $version } er ute; venter til ingenting pågår ({ $work }).
updates-busy-more = { $count } til
updates-busy-jobs =
    { $count ->
        [one] { $count } jobb venter
       *[other] { $count } jobber venter
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } opplasting
       *[other] { $count } opplastinger
    }
updates-list-separator = ,{ " " }
updates-what = Når en ny versjon er ute, kan Hoard kalle en adresse så snart ingenting pågår (ingen opplastinger, jobber, konverteringer eller vedlikehold), slik at det som oppdaterer containeren, ikke starter den på nytt midt i en opplasting. Den sender en POST med versjonen i en JSON-kropp. Dette krever versjonssjekken ovenfor.
updates-url = Adresse som skal kalles
updates-url-placeholder = Ingen (f.eks. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Lagret (skriv for å erstatte)
updates-auth-placeholder = Ingen (f.eks. Bearer my-token)
updates-remove = Fjern
updates-save = Lagre
updates-call-now-title = Det den er koblet til, kjører
updates-call-now = Kall den nå
updates-what-calls = Hva kan den kalle?
updates-watchtower = Watchtowers HTTP-API
updates-watchtower-at = :{ " " }
updates-watchtower-with = , med{ " " }
updates-watchtower-token = { " " }og tokenet fra{ " " }
updates-watchtower-its = { " " }(med endepunktet{ " " }
updates-watchtower-endpoint = { " " }slått på).
updates-portainer = En Portainer-webhook
updates-portainer-rest = { " " }på Hoards stack eller container, med re-pull image slått på. Tokenet ligger i URL-en.
updates-script = Et skript på verten
updates-script-listener = { " " }bak en webhook-lytter (for eksempel{ " " }
updates-script-runs = ) som kjører{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }eller noe annet som tar imot en webhook.
updates-poll-before = Eller hopp over adressen: et skript med et tilgangstoken kan spørre{ " " }
updates-poll-until = { " " }til{ " " }
updates-poll-and = { " " }er satt og{ " " }
updates-poll-after = { " " }er true, og deretter hente og starte Hoard på nytt.

## MaintenancePanel.vue

maintenance-load-failed = Kunne ikke laste Vedlikehold
maintenance-reanalyze-title = Analysere alle pakker på nytt?
maintenance-reanalyze-body = Hver fil leses på nytt, noe som kan ta lang tid på et stort bibliotek; pakker vises som under arbeid til det er deres tur. Dine egne navn, tagger og notater blir værende.
maintenance-reanalyze-confirm = Analyser på nytt
maintenance-gc-title = Slette ubrukte filer?
maintenance-gc-body = Lagrede filer som ingenting i biblioteket peker på, slettes for godt. Pakker i arkivet beholder sine.
maintenance-gc-confirm = Slett dem
maintenance-start-failed = Kunne ikke starte «{ $task }»
maintenance-just-now = akkurat nå
maintenance-minutes-ago = for { $count } min siden
maintenance-hours-ago = for { $count } t siden
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } t
maintenance-loading = Laster …
maintenance-running = Kjører …
maintenance-starting = Starter …
maintenance-run = Kjør
maintenance-started = { $detail } · startet { $when }
maintenance-starting-detail = Starter
maintenance-finished = Fullført:{ " " }
maintenance-failed = Mislyktes:{ " " }
maintenance-never-run = Aldri kjørt
maintenance-history = Tidligere kjøringer
maintenance-col-task = Oppgave
maintenance-col-finished = Fullført
maintenance-col-took = Tok
maintenance-col-result = Resultat
maintenance-script-before = Sikkerhetskopiering og skadesjekken kan også kjøres fra et skript:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }og{ " " }
maintenance-script-after = . Disse kjøringene vises også her.

## SettingsPage.vue

settings-title = Innstillinger
settings-loading = Laster …
settings-load-failed = Kunne ikke laste innstillingene: { $error }
settings-page-general = Generelt
settings-page-general-blurb = Hva denne serveren heter, for alle som bruker den.
settings-page-appearance = Utseende
settings-page-appearance-blurb = Temaene denne nettleseren bruker når den er lys og når den er mørk. Lagres bare i denne nettleseren.
settings-page-library = Bibliotek
settings-page-library-blurb = Hva som er i Hoard: hvor stort det er, hva det består av, hvordan det har vokst og hvor godt det er tagget. Arkiverte ting telles ikke med.
settings-page-tags = Tagger
settings-page-tags-blurb = Alle taggene i biblioteket. Slå sammen de som betyr det samme, eller gi en nytt navn; taggerne bruker det nye navnet fra da av.
settings-page-store-files = Butikkfiler
settings-page-store-files-blurb = Filene butikker legger ved siden av innholdet sitt, som Hoard leser for å gi pakker navn og kreditere skaperne. Lær den opp på filer den ikke kjenner, og del det du har lært den.
settings-page-locations = Biblioteksplasseringer
settings-page-locations-blurb = Mappene på serveren som inneholder biblioteket: de Hoard lagrer filer i, og mapper med pakker den indekserer der de ligger.
settings-page-tools = Verktøy
settings-page-tools-blurb = Eksterne programmer Hoard bruker til å konvertere og lese filer.
settings-page-ai = AI-tagging
settings-page-ai-blurb = En AI-modell navngir, beskriver og tagger nye pakker, i tillegg til de innebygde reglene. Taggene den gir, merkes som forslag.
settings-page-search = Bildesøk
settings-page-search-blurb = Søk i bilder etter hva de viser, ikke bare etter navnene, og finn ting som ligner i art, ikke bare i farge.
settings-page-adding = Legge til pakker
settings-page-adding-blurb = Hva Hoard gjør med en pakke når den legges til, i tillegg til å lese og tagge den.
settings-page-downloads = Nedlastinger
settings-page-downloads-blurb = Hvordan filer navngis i det Hoard sender: nedlastinger, zip-filer, hoard-verktøyet og AI-assistenter. Filene i biblioteket forblir som de er.
settings-page-account = Konto
settings-page-account-blurb = Passordet ditt, de andre som kan logge på, og tilgangstokener for skript og andre programmer.
settings-page-auth = Autentisering
settings-page-auth-blurb = Hvordan folk logger på: Hoards egne kontoer, en LDAP-katalog, Microsoft Active Directory eller en OAuth-leverandør, hvor lenge økter varer, og hvilke grupper som får hvilken rolle.
settings-page-connection = Tilkobling
settings-page-connection-blurb = Hvordan nettlesere og apper når Hoard: adressen den er kjent under, og HTTPS, som noen nettleserfunksjoner krever.
settings-page-performance = Ytelse
settings-page-performance-blurb = Hvor hardt serveren jobber, hvordan miniatyrbilder lages og lagres, og det meste den tar imot.
settings-page-issues = Problemer
settings-page-issues-blurb = Hva som gikk galt ved åpning eller behandling av filer: arkiver som ikke ville pakkes ut, filer som ikke kunne leses, jobber som mislyktes. Prøv dem igjen når årsaken er rettet, eller avvis dem.
settings-page-maintenance = Vedlikehold
settings-page-maintenance-blurb = Sikkerhetskopier, sjekk for skade og rydd opp. Én av hver kjører om gangen, og du kan forlate denne siden mens de kjører.
settings-the-default = standardverdien
settings-default-on = på
settings-default-off = av
settings-default-auto = automatisk
settings-option-saved = { $name }: { $value }
settings-back-to-default = Tilbake til standard
settings-save = Lagre
settings-use = Bruk
settings-status-off = Av

## SettingsPage.vue: Utseende

settings-themes = Temaer
settings-look-light = Lys
settings-look-dark = Mørk
settings-look-group =
    { $scheme ->
        [light] Lyst tema
       *[dark] Mørkt tema
    }
settings-preview =
    Viser det { $preview ->
        [light] lyse
       *[dark] mørke
    } temaet så du kan se det. Du bruker { $used ->
        [light] lyst
       *[dark] mørkt
    }{ $system ->
        [yes] { " " }(følger systemet)
       *[other] { "" }
    }, og det kommer tilbake når du forlater denne siden.
settings-preview-back =
    Tilbake til { $used ->
        [light] lyst
       *[dark] mørkt
    }
settings-look-hint = Bytt mellom lyst og mørkt (eller følg systemet) fra topplinjen.

## SettingsPage.vue: Verktøy

settings-blender-what = Konverterer .blend-, .dae-, .abc- og .x3d-modeller til glTF i bakgrunnen, så de får miniatyrbilder og en 3D-forhåndsvisning. Pakker som ble lagt til mens det var av, konverteres når det er klart.
settings-blender-download = Bruker en installert Blender, eller laster ned Blender 4.5 LTS (ca. 400 MB) fra et blender.org-speil.
settings-ffmpeg-what = Koder MP3-, OGG- og Opus-nedlastinger, og leser lyd Hoard ikke kan dekode selv (AAC, M4A, WMA, …).
settings-ffmpeg-download = Bruker en ffmpeg på systemet med de riktige koderne, eller laster ned en statisk versjon (ca. 80 MB).
settings-tool-reset = { $tool }: tilbake til standard
settings-tool-no-download = Bruker en installert { $tool }. Det finnes ingen versjon å laste ned for denne maskinen, så installer det eller oppgi banen.
settings-tool-path = Programbane for { $tool }
settings-mode-off = Av
settings-mode-auto = Automatisk
settings-mode-path = Dette programmet
settings-tool-looking = Leter …
settings-tool-downloading = Laster ned … fremdriften vises i aktivitetsindikatoren
settings-tool-ready = Klar: { $detail }
settings-soundfont = MIDI-SoundFont
settings-soundfont-what = Instrumentene MIDI-filer spilles av og konverteres med.
settings-soundfont-file = SoundFont-fil
settings-soundfont-placeholder = Funnet på serveren, eller innebygd
settings-soundfont-default = funnet på serveren, eller innebygd

## SettingsPage.vue: Bildesøk

settings-clip-heading = Søk etter hva bilder viser
settings-clip-reset = Bildesøk: tilbake til standard
settings-clip-saved-on = Bildesøk: på
settings-clip-saved-off = Bildesøk: av
settings-clip-what = Skriv hva du er ute etter («mossy rock», «sci-fi door», «wooden barrel»), så dukker bilder som viser det opp ved siden av navn som har ordene. «Mer som dette» finner ting av samme slag, ikke bare de samme fargene.
settings-clip-toggle = Slå på bildesøk
settings-clip-hint = Bruker OpenAIs CLIP-modell, kjørt på prosessoren til denne serveren: ingenting forlater den. Når du slår det på, lastes modellen ned ({ $size } MB) fra Hugging Face, og deretter lærer den alle bildene i bakgrunnen med lav prioritet. Den bruker 300 til 600 MB minne mens den jobber eller noen søker. Når du slår det av, slettes modellen, og det den har lært, beholdes til neste gang.
settings-clip-looking = Leter etter modellen …
settings-clip-downloading = Laster ned modellen … fremdriften vises i aktivitetsindikatoren
settings-clip-ready-empty = Klar. Bilder læres etter hvert som pakker legges til.
settings-clip-ready-all =
    { $count ->
        [one] Klar: kjenner { $count } bilde
       *[other] Klar: kjenner alle { $count } bildene
    }
settings-clip-learning = Klar: lærer bilder, { $done } av { $total } så langt
settings-clip-unavailable = Kunne ikke hente modellen: { $detail }. Prøver igjen hver time.

## SettingsPage.vue: AI-tagging

settings-ai-service = Tjeneste
settings-ai-off = Av
settings-ai-off-hint = Pakker tagges bare med de innebygde reglene.
settings-ai-local = Lokal modell
settings-ai-local-hint = llm-tjenesten i Hoards compose-fil (eller cargo xtask llm). Last ned en modell nedenfor; den kjører på denne maskinen, og ingenting forlater den.
settings-ai-chatgpt = ChatGPT-abonnement
settings-ai-chatgpt-hint = Bruker ChatGPT Go-, Plus-, Pro- eller Business-abonnementet ditt i stedet for en API-nøkkel.
settings-ai-openrouter-hint = Én nøkkel for hundrevis av hostede modeller, noen av dem gratis. Modeller som tar imot bilder, er merket.
settings-ai-openai-hint = Hostet av OpenAI.
settings-ai-anthropic-hint = Hostet av Anthropic, via det OpenAI-kompatible endepunktet.
settings-ai-gemini-hint = Hostet av Google, via det OpenAI-kompatible endepunktet.
settings-ai-ollama-hint = En Ollama-server du kjører selv. Endre adressen hvis det ikke er en container som heter ollama.
settings-ai-custom = Annet (OpenAI-kompatibelt)
settings-ai-custom-hint = Alt som snakker OpenAIs chat-API: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Kobler til …
settings-ai-connected =
    { $count ->
        [one] Tilkoblet · { $count } modell
       *[other] Tilkoblet · { $count } modeller
    }
settings-ai-connected-none = Tilkoblet, men den viser ingen modeller; skriv inn navnet på modellen
settings-chatgpt-connected-none = Tilkoblet; skriv inn navnet på modellen
settings-ai-takes-images = tar imot bilder
settings-ai-saved = AI-tagging: { $service }
settings-ai-saved-off = AI-tagging er av
settings-ai-reset = AI-tagging: tilbake til standard
settings-ai-on = På: { $service } · { $model }
settings-ai-url = Adresse
settings-ai-key = API-nøkkel
settings-ai-key-saved = Lagret (skriv for å erstatte)
settings-ai-key-paste = Lim inn nøkkelen din
settings-ai-key-optional = Ingen (valgfritt)
settings-ai-key-remove = Fjern
settings-ai-get-key = Skaff en nøkkel
settings-ai-model = Modell
settings-ai-model-pick = Velg eller skriv inn en modell
settings-ai-model-name = Modellnavn
settings-ai-refresh = Oppdater
settings-ai-connect = Koble til
settings-ai-need-key = Legg til API-nøkkelen for å se modellene
settings-ai-vision = Send også miniatyrbilder (krever en modell som tar imot bilder)
settings-ai-batch = Per forespørsel
settings-ai-batch-unit = ressurser (færre for små lokale modeller, flere for store hostede)

## SettingsPage.vue: ChatGPT-pålogging

settings-chatgpt-account = Konto
settings-chatgpt-checking = Sjekker …
settings-chatgpt-status-as = Pålogget som { $email }
settings-chatgpt-status = Pålogget
settings-chatgpt-signed-in-as = Pålogget ChatGPT som { $email }
settings-chatgpt-signed-in = Pålogget ChatGPT
settings-chatgpt-signed-out = Logget av ChatGPT
settings-chatgpt-continue = Fortsett med ChatGPT
settings-chatgpt-sign-out = Logg av
settings-chatgpt-sign-out-title = Logge av ChatGPT?
settings-chatgpt-sign-out-in-use = AI-tagging bruker ChatGPT-abonnementet ditt, så den stopper til du logger på igjen eller velger en annen tjeneste.
settings-chatgpt-sign-out-body = Hoard glemmer påloggingen og ber OpenAI avslutte den.
settings-chatgpt-manage = Administrer forbruk
settings-chatgpt-finish-before = Fullfør påloggingen i vinduet som åpnet seg (
settings-chatgpt-finish-link = åpne det igjen
settings-chatgpt-finish-middle = ). Hvis det ender på en side som ikke lastes, på en adresse som begynner med{ " " }
settings-chatgpt-finish-after = , kopierer du den adressen og limer den inn her:
settings-chatgpt-paste = Adressen påloggingen endte på
settings-chatgpt-finish = Fullfør
settings-chatgpt-usage = Taggeforespørsler teller mot forbruket i ChatGPT-abonnementet ditt. Administrer forbruk viser hva Hoard har brukt, og lar deg begrense det. For å koble Hoard fra selve ChatGPT går du til Innstillinger → Sikkerhet og pålogging → Logg på med ChatGPT på chatgpt.com.

## SettingsPage.vue: Lokale modeller

settings-local-downloading-progress = Laster ned { $percent } % · { $done } av { $total }
settings-local-downloading = Laster ned …
settings-local-incomplete = Nedlastingen stoppet
settings-local-failed = Kunne ikke starte forrige gang
settings-local-in-use-running = I bruk · kjører
settings-local-in-use = I bruk
settings-local-starting = Starter …
settings-local-running = Kjører
settings-local-ready-images = Klar · tar imot bilder
settings-local-ready-text = Klar · bare tekst
settings-local-delete-title = Slette { $name }?
settings-local-delete-in-use = AI-tagging bruker denne modellen, så den stopper til du velger en annen. Filene slettes fra modellserveren.
settings-local-delete-downloading = Nedlastingen stopper, og det som er lastet ned så langt, slettes.
settings-local-delete-body = Filene slettes fra modellserveren. Du kan laste den ned igjen senere.
settings-local-delete = Slett
settings-local-deleting = Sletter …
settings-local-cancel = Avbryt
settings-local-resume = Fortsett
settings-local-download = Last ned
settings-local-start-before = Start den med{ " " }
settings-local-start-or = { " " }(eller{ " " }
settings-local-start-developing = { " " }under utvikling), og{ " " }
settings-local-start-retry = prøv igjen
settings-local-start-after = . En server som bare kjører én modell, fungerer også: oppgi modellens navn nedenfor.
settings-local-list = På modellserveren
settings-local-none = Ingenting lastet ned ennå. Velg en nedenfor; den lagres på serveren, og du kan bytte når som helst.
settings-local-catalog = Modeller å laste ned
settings-local-size = { $size } GB · { $license }
settings-local-other = En annen modell fra Hugging Face
settings-local-other-placeholder = En annen fra Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Legge til pakker

settings-store-pages = Butikksider
settings-store-pages-what = Nye pakker slås opp i butikkene i bakgrunnen, så de får butikkside, selger og lisens.
settings-store-lookup = Slå opp butikksider automatisk
settings-unreal = Unreal-pakkefiler
settings-unreal-reset = Unreal-pakkefiler: tilbake til standard
settings-unreal-what = Hva som skal gjøres med .uasset-filer.
settings-conv = Konverteringsregler
settings-conv-what = Konverter filer Hoard tar vare på til ett format, for eksempel alle OBJ og FBX til GLB, så biblioteket blir ensartet. Den konverterte filen tar originalens plass, med navn, tagger, samlinger og lenker. Filer i overvåkede mapper endres aldri.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = På
settings-conv-to = Konverter til
# Heading over the file formats a rule converts.
settings-conv-from = Fra
settings-conv-remove = Fjern
settings-conv-remove-label = Fjern regel { $n }
settings-conv-taken = En annen regel konverterer dette formatet
settings-conv-upright = Snu modellene opprett når de konverteres (Y opp, vendt mot +Z), slik pakkens modeller vises
settings-conv-add = Legg til en regel
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Behold originaler i
settings-conv-keep-days =
    { $count ->
        [one] dag
       *[other] dager
    }
settings-conv-keep-what = Fram til da kan siden til en konvertert fil legge tilbake originalen. Teksturer kopieres ikke inn i en konvertert modell: den peker på bildefilene ved siden av seg. En modells .mtl- eller .bin-filer blir liggende.
settings-conv-save = Lagre regler
settings-conv-revert = Angre endringer
settings-conv-existing = Konverter filer som allerede er i biblioteket
settings-conv-saved = Konverteringsregler lagret. De gjelder for pakker som legges til fra nå av.
settings-conv-converting =
    { $count ->
        [one] Konverterer én fil i bakgrunnen
       *[other] Konverterer { $count } filer i bakgrunnen
    }
settings-conv-nothing = Ingenting i biblioteket trenger å konverteres.
settings-conv-ask-title =
    { $count ->
        [one] Konvertere én fil som allerede er i biblioteket?
       *[other] Konvertere { $count } filer som allerede er i biblioteket?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Den er i én pakke.
       *[other] De er i { $packs } pakker.
    } Hver original beholdes i { $days } dager, og fram til da kan den legges tilbake fra siden sin.
settings-conv-ask-busy =
    { $count ->
        [one] Én er i en pakke som fortsatt legges til; den konverteres når det er ferdig.
       *[other] { $count } er i pakker som fortsatt legges til; de konverteres når det er ferdig.
    }
settings-conv-ask-list = Filer som ville blitt konvertert
settings-conv-ask-more = …og { $count } til
settings-conv-ask-skipped = Latt være som de er:
settings-conv-skip-watched =
    { $count ->
        [one] én fil i en overvåket mappe
       *[other] { $count } filer i overvåkede mapper
    }
settings-conv-skip-unity =
    { $count ->
        [one] én fil et Unity-prosjekt kjenner ved navn
       *[other] { $count } filer et Unity-prosjekt kjenner ved navn
    }
settings-conv-skip-used =
    { $count ->
        [one] én fil en scene eller en annen fil nevner ved navn
       *[other] { $count } filer en scene eller en annen fil nevner ved navn
    }
settings-conv-skip-beside =
    { $count ->
        [one] én fil der den konverterte kopien allerede er i pakken
       *[other] { $count } filer der de konverterte kopiene allerede er i pakkene
    }
settings-conv-skip-blender =
    { $count ->
        [one] én fil Blender ikke har lest ennå (Innstillinger → Verktøy)
       *[other] { $count } filer Blender ikke har lest ennå (Innstillinger → Verktøy)
    }
settings-conv-ask-new = Lagre, bare nye pakker
settings-conv-ask-save-convert =
    { $count ->
        [one] Lagre og konverter den
       *[other] Lagre og konverter { $count } filer
    }
settings-conv-ask-convert =
    { $count ->
        [one] Konverter den
       *[other] Konverter { $count } filer
    }
settings-unreal-off = La være som de er
settings-unreal-off-hint = Konverter fra en pakkes side når du vil.
settings-unreal-keep = Konverter, behold originalene
settings-unreal-keep-hint = Teksturer, lyder og materialer kommer ut i vanlige formater; .uasset-filen blir også værende.
settings-unreal-replace = Konverter, forkast originalene
settings-unreal-replace-hint = Sparer plass når innholdet er hentet ut. Mesher venter fortsatt på Unreal-arbeideren.

## SettingsPage.vue: Nedlastinger

settings-tn = Teksturnavn
settings-tn-saved = Teksturnavn lagret
settings-tn-reset = Teksturnavn: tilbake til standard
settings-tn-what = Kartene til et materiale sendes ut med navn etter materialet og hva hvert kart er til, uansett hva filene heter: Rock_Normal.png, ikke rock_moss_xeokah0_raw_8K_Normal.png. Filene i biblioteket røres ikke, og et kart som en modell eller MTL-fil sendt med den, navngir, beholder det navnet, så modellen fortsatt finner det.
settings-tn-toggle = Navngi teksturkart etter materialet
settings-tn-style = Stil
settings-tn-style-group = Navnestil
settings-tn-descriptive = Beskrivende
settings-tn-short = Kort
settings-tn-custom = Egen
settings-tn-pattern = Mønster
settings-tn-hint-name = { " " }materialet,{ " " }
settings-tn-hint-map = { " " }hva kartet er til,{ " " }
settings-tn-hint-size = { " " }størrelsen når settet finnes i mer enn én (legges til på slutten hvis mønsteret utelater den).
settings-tn-example = Eksempel
settings-tn-suffixes = Suffikser for hver type kart
settings-tn-slots = I GLB-nedlastinger får en tekstur som er pakket inn i en modell uten egen fil, navn etter materialet og sporet den fyller, og disse sporene har også suffikser:
settings-tn-endings = Egne endelser
settings-tn-endings-what = En tekstur som ikke er et av kartene til et materiale, beholder sitt eget navn, med mindre det slutter på en av disse: _diff → _BC sender rock_diff.png som rock_BC.png. La det andre feltet stå tomt for å fjerne endelsen.
settings-tn-ending-from = Endelse
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Bytt med
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Legg til en endelse
settings-tn-ending-remove = Fjern
settings-tn-ending-remove-label = Fjern { $ending }

## SettingsPage.vue: Generelt og Tilkobling

settings-name = Navn
settings-name-what = Hva denne serveren heter i topplinjen, i nettleserfanen og på påloggingssiden, så du kan skille dem fra hverandre hvis du har mer enn én Hoard.
settings-address = Adresse
settings-address-what = Adressen folk og apper bruker for å nå Hoard, når det ikke er den denne nettleseren er på: en omvendt proxy, en tunnel eller et tailnet-navn, for eksempel https://hoard.example.com. Lenker Hoard deler ut (til Claude og andre apper, ved pålogging, på sertifikatet), bruker den.
settings-public-url = Offentlig adresse
settings-public-url-placeholder = Fra nettleserens adresse (f.eks. https://hoard.example.com)
settings-public-url-default = fra nettleserens adresse

## ProfilerPanel.vue: profileringen øverst i Innstillinger → Ytelse

profiler = Profilering
profiler-what = Hva Hoard har gjort de siste ti minuttene, oppdatert hvert andre sekund mens du ser på. Det holdes bare i minnet og nullstilles når Hoard starter på nytt.
profiler-starting = De første målingene kommer om et øyeblikk.
profiler-failed = Kunne ikke lese profileringen: { $error }
profiler-cpu = Prosessorbruk
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Hele maskinen
profiler-cpu-summary = Hoard bruker { $hoard } av prosessorene, og hele maskinen { $machine }.
profiler-memory = Minne
profiler-serving = Forespørsler som pågår
profiler-working = Jobber med
profiler-working-none = Ingenting kjørte i bakgrunnen.
profiler-working-more =
    { $count ->
        [one] Og én til.
       *[other] Og { $count } til.
    }
profiler-requests = Tid brukt på forespørsler
profiler-requests-what = Hver stolpe er ti sekunder med forespørsler, med tiden fordelt etter hvor den gikk.
profiler-part-db = Database
profiler-part-db-title = Venter på databasen og kjører spørringene dens
profiler-part-disk = Disk
profiler-part-disk-title = Leser lagrede filer og komprimerer det som sendes
profiler-part-send = Sending
profiler-part-send-title = Venter mens nettverket tar imot det som ble sendt
profiler-part-other = Annet
profiler-part-other-title = Finner fram til svaret
profiler-request-count =
    { $count ->
        [one] { $count } forespørsel
       *[other] { $count } forespørsler
    }
profiler-requests-none = Ingen forespørsler de siste ti minuttene.
profiler-slowest = Tregest: { $route }, { $time }
profiler-routes = Etter rute
profiler-route = Rute
profiler-count = Forespørsler
profiler-average = Gjennomsnitt
profiler-p95 = Tregeste 5 %
profiler-split = Hvor tiden gikk
profiler-sent = Sendt
profiler-portal = grensesnittets egne filer

## SettingsPage.vue: Ytelse

settings-render = 3D-miniatyrbilder
settings-render-toggle = Render miniatyrbilder for 3D-modeller
settings-gpu-on = På GPU-en: { $renderer }
settings-gpu-software = Fant ingen GPU, så i programvare ({ $renderer })
settings-gpu-none = Fant ingen GPU eller programvarerenderer, så modeller får ikke miniatyrbilder
settings-thumbs = Miniatyrbilder
settings-thumbs-what = Hvordan nye miniatyrbilder og modellvisninger lagres. De med gjennomsiktighet, og pikselgrafikk, er alltid PNG. For å krympe de du allerede har, kjør «Krymp miniatyrbilder» i Vedlikehold.
settings-previews-size = Forhåndsvisninger tar { $size } nå.
settings-thumbs-saved = Miniatyrbilder: { $quality }
settings-thumbs-small = Mindre
settings-thumbs-small-hint = JPEG, kvalitet 70. Greit i rutenettet; litt mykt på nært hold.
settings-thumbs-balanced = Balansert
settings-thumbs-balanced-hint = JPEG, kvalitet 82. Vanskelig å skille fra originalen i miniatyrstørrelse.
settings-thumbs-sharp = Skarpere
settings-thumbs-sharp-hint = JPEG, kvalitet 92. Omtrent dobbelt så stor som Balansert.
settings-thumbs-lossless = Tapsfri
settings-thumbs-lossless-hint = PNG, nøyaktig slik det ble laget. Flere ganger større.
settings-motion-thumbs = Levende miniatyrbilder for animasjoner
settings-motion-thumbs-what = Hver animasjon får også en kort film i løkke som spilles mens du peker på flisen. De tar omtrent 40 kB hver og litt tid å lage for et stort bibliotek. Slår du dette av, slettes de.
settings-limits = Arbeid og grenser
settings-limits-what = Hver av dem lagres når du forlater feltet eller trykker Enter. Størrelser tar enheter: 100 GB, 512 MB, 1,5 TB; et tall alene er GB.
settings-workers = Bakgrunnsarbeidere
settings-workers-unit = jobber samtidig (inntak, tagging, konverteringer)
settings-max-upload = Største opplasting
settings-max-upload-unit = per fil
settings-max-expand = Største pakke
settings-max-expand-unit = etter utpakking
settings-conversion-cache = Konverterte nedlastinger
settings-conversion-cache-unit-now = beholdes høyst, { $size } nå; de som er brukt minst nylig, går først
settings-conversion-cache-unit = beholdes høyst; de som er brukt minst nylig, går først

## SettingsPage.vue: Vedlikehold

settings-updates = Oppdateringer
settings-version = Dette er Hoard { $version }.
settings-update-check = Sjekk GitHub med noen timers mellomrom etter en ny versjon

## Ressurstyper (api.ts)

api-kind-texture = Teksturer
api-kind-vector = Vektorgrafikk
api-kind-sheet = Spriteark
api-kind-audio = Lyd
api-kind-model = Modeller
api-kind-animation = Animasjoner
api-kind-font = Skrifter
api-kind-shader = Shadere
api-kind-material = Materialer
api-kind-video = Video
api-kind-document = Dokumenter
api-kind-data = Data
api-kind-other = Annet
api-kind-texture-name = teksturer
api-kind-vector-name = vektorgrafikk
api-kind-sheet-name = spriteark
api-kind-audio-name = lyd
api-kind-model-name = modeller
api-kind-animation-name = animasjoner
api-kind-font-name = skrifter
api-kind-shader-name = shadere
api-kind-material-name = materialer
api-kind-video-name = video
api-kind-document-name = dokumenter
api-kind-data-name = data
api-kind-other-name = annet

## Varsler, dialoger og lasting (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Angre
ui-today = i dag
ui-yesterday = i går
ui-days-ago =
    { $days ->
        [one] for { $days } dag siden
       *[other] for { $days } dager siden
    }
ui-notifications = Varsler
ui-dismiss = Avvis
ui-cancel = Avbryt
ui-lazy-loading = Laster { $what } …
ui-lazy-failed = Kunne ikke laste { $what }.
ui-lazy-reload = Last inn siden på nytt

## ResetButton.vue

reset-title = Tilbake til standard: { $value }
reset-label = Tilbake til standard, { $value }

## SizeInput.vue

size-invalid = En størrelse som 100 GB, 512 MB eller 1,5 TB
size-hint = En størrelse som 100 GB, 512 MB eller 1,5 TB; et tall alene er GB

## AuthPanel.vue (Innstillinger → Autentisering)

auth-load-failed = Kunne ikke laste autentiseringsinnstillingene: { $error }
auth-saved = Lagret. Det gjelder fra neste pålogging.
auth-method-internal = Intern
auth-method-internal-hint = Navn og passord som Hoard holder selv, lagt til under Innstillinger → Konto.
auth-method-ldap-hint = En katalog sjekker navnet og passordet som skrives inn på påloggingssiden: OpenLDAP, FreeIPA, LLDAP eller en annen LDAP-server.
auth-method-ad-hint = Windows-domenekontoer logger på med sitt vanlige navn og passord.
auth-method-oauth-hint = En «Logg på med …»-knapp som går til Google, Microsoft, GitHub, Keycloak, Authentik eller en annen leverandør og tilbake.
auth-session-unused = Etter 30 dager uten bruk
auth-session-1h = 1 time etter pålogging
auth-session-8h = 8 timer etter pålogging
auth-session-day = Et døgn etter pålogging
auth-session-week = En uke etter pålogging
auth-session-30d = 30 dager etter pålogging
auth-session-hours =
    { $hours ->
        [one] { $hours } time etter pålogging
       *[other] { $hours } timer etter pålogging
    }
auth-role-view = Kan se
auth-role-edit = Kan endre
auth-role-admin = Administrator
auth-role-word-view = kan se
auth-role-word-edit = kan endre
auth-role-word-admin = administrator
auth-role-word-default = { $role } (standardrollen)
auth-tls-starttls-hint = Vanlig LDAP (port 389) som bytter til TLS før noe sendes.
auth-tls-ldaps-hint = TLS fra starten av (port 636).
auth-tls-none = Ingen
auth-tls-none-hint = Passord går over nettverket som de er. Bare på et nettverk du stoler på.
auth-test-failed = Kunne ikke teste: { $error }
auth-preset-custom = Annet (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel og andre: oppgi utstederen, så fyller «Finn» inn adressene.
auth-preset-google-note = Lag en OAuth-klient (Web application) i Google Cloud Console → APIs & Services → Credentials. Google sender ingen grupper, så gi roller under Innstillinger → Konto, og slå av opprettelse av kontoer ved første pålogging med mindre alle med en Google-konto skal slippe inn.
auth-preset-microsoft-note = Registrer en app i Entra ID → App registrations (med en Web-omdirigering til adressen nedenfor). Sett tenant-ID-en din i stedet for «organizations» for å bare slippe inn din egen organisasjon. For roller fra grupper legger du til groups-kravet under Token configuration: det viser gruppe-ID-er.
auth-preset-github-note = Lag en OAuth-app i GitHub → Settings → Developer settings. GitHub sender ingen grupper, så gi roller under Innstillinger → Konto, og slå av opprettelse av kontoer ved første pålogging med mindre alle med en GitHub-konto skal slippe inn.
auth-preset-gitlab-note = Lag en applikasjon i GitLab → Preferences → Applications (eller i administrasjonsområdet til din egen instans: bytt ut gitlab.com i adressene).
auth-discovered = Fylte inn leverandørens adresser.
auth-copied = Kopiert
auth-copy-failed = Kunne ikke kopiere: merk teksten og kopier den selv
auth-sign-in-with = Logg på med
auth-own-accounts-still-work = Hoards egne kontoer logger fortsatt på med egne passord, så en katalog eller leverandør som er nede eller feilkonfigurert, ikke kan stenge deg ute.
auth-sessions = Økter
auth-sign-out-after = Logg folk av
auth-ldap-server = LDAP-server
auth-server-type = Servertype
auth-server-type-custom = Egendefinert
auth-servers = Servere
auth-server-n = Server { $n }
auth-remove-server-n = Fjern server { $n }
auth-remove = Fjern
auth-add-server = Legg til en server
auth-servers-hint = Den neste prøves når en ikke kan nås.
auth-security = Sikkerhet
auth-connection-security = Tilkoblingssikkerhet
auth-skip-verify = Ikke sjekk serverens sertifikat (bare for å prøve ting ut)
auth-ca = Sertifiseringsinstans
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (tomt: de systemet stoler på)
auth-lookup = Slå opp folk
auth-anonymously = Anonymt
auth-reader-account = Lesekonto
auth-reader-password = Passordet til den
auth-saved-type-to-replace = Lagret: skriv for å erstatte
auth-people-under = Folk ligger under
auth-name-attribute = Navneattributt
auth-only-matching = Bare oppføringer som samsvarer med
auth-groups-under = Grupper ligger under
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (tomt: bare memberOf)
auth-their-groups = Gruppene deres
auth-group-filter-hint = { "{" }dn{ "}" } er personens oppføring, { "{" }user{ "}" } navnet personen logget på med.
auth-test = Test
auth-test-hint = Når hver server og logger på som lesekontoen. Med navn og passord logger den også på som den personen og viser hva Hoard ville gjort ut av det. Ingenting lagres.
auth-test-user = Noen å logge på som
auth-test-user-placeholder = Noen å logge på som (valgfritt)
auth-test-password = Passordet deres
auth-testing = Tester …
auth-test-connectivity = Test tilkoblingen
auth-test-server-ok = nådd, og lesekontoen logget på
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }logget på (
auth-test-signed-in-after = ).
auth-test-role = Vedkommende ville fått rollen: { $role }.
auth-test-no-role = Vedkommende er ikke med i noen av gruppene under Roller, og ville derfor blitt avvist.
auth-test-groups = Grupper: { $groups }
auth-test-no-groups = Fant ingen grupper.
auth-provider = Leverandør
auth-redirect = Omdirigeringsadresse
auth-copy = Kopier
auth-redirect-hint = Gi leverandøren denne når du lager Hoards klient. Den kommer fra den offentlige adressen under Innstillinger → Tilkobling, eller adressen du bruker nå.
auth-issuer = Utsteder
auth-discover = Finn
auth-client-id = Klient-ID
auth-client-secret = Klienthemmelighet
auth-authorization-url = Autorisasjonsadresse
auth-token-url = Token-adresse
auth-userinfo-url = Adresse for brukerinfo
auth-userinfo-placeholder = (tomt: ID-tokenet sier hvem det er)
auth-scopes = Omfang (scopes)
auth-name-from = Navn fra
auth-groups-from = Grupper fra
auth-groups-from-placeholder = (ingen)
auth-button = Knapp
auth-button-placeholder = kontoen din
auth-accounts = Kontoer
auth-provision = Lag en Hoard-konto første gang noen logger på
auth-provision-on-hint = Ellers legger en administrator til hver person først, under Innstillinger → Konto.
auth-provision-off-hint = En administrator legger til hver person først, under Innstillinger → Konto, og velger hvordan de logger på.
auth-open-to-anyone = Alle med en { $provider }-konto kan komme inn. Legg til seergrupper nedenfor, eller slå dette av og legg til folk selv.
auth-new-accounts = Nye kontoer
auth-roles-from-groups = Roller fra grupper
auth-roles-hint = Grupper der medlemmene får hver rolle, én per linje, etter navn eller DN. Den høyeste som passer, vinner, og det sjekkes ved hver pålogging. Uten noen grupper får nye kontoer rollen ovenfor, og en administrator endrer den etterpå. Med grupper under Kan se kan bare medlemmer av en oppført gruppe logge på.
auth-roles-hint-oauth = Grupper der medlemmene får hver rolle, én per linje, etter navn eller DN (eller hva groups-kravet viser). Den høyeste som passer, vinner, og det sjekkes ved hver pålogging. Uten noen grupper får nye kontoer rollen ovenfor, og en administrator endrer den etterpå. Med grupper under Kan se kan bare medlemmer av en oppført gruppe logge på.
auth-saving = Lagrer …
auth-save = Lagre

## HttpsPanel.vue (Innstillinger → Tilkobling)

https-load-failed = Kunne ikke laste HTTPS: { $error }
https-off-redirecting = HTTPS er av. Sender deg til http://
https-now-on = HTTPS er på
https-saved = Lagret. Nye tilkoblinger bruker det nye sertifikatet.
https-now-off = HTTPS er av
https-http3-failed-toast = HTTP/3 kunne ikke starte: { $error }
https-copied = Kopiert
https-copy-failed = Kunne ikke kopiere: merk teksten og kopier den selv
https-mode-off = Av
https-mode-off-hint = Bare vanlig http://.
https-mode-own = Hoards eget sertifikat
https-mode-own-hint = Hoard lager et for denne serverens adresser. Installer Hoards sertifiseringsinstans én gang på hver enhet, så stoler nettleserne der på det.
https-mode-custom = Mitt sertifikat
https-mode-custom-hint = Et du allerede har, f.eks. fra tailscale cert eller Let’s Encrypt. Nettlesere stoler på det uten at noe må installeres.
https-untrusted-own = Denne nettleseren stoler ikke på det for { $host } ennå: installer Hoards sertifiseringsinstans nedenfor. Du får en advarsel du kan klikke deg forbi.
https-untrusted-uncovered = Denne nettleseren stoler ikke på det for { $host } ennå: sertifikatet dekker ikke denne adressen. Du får en advarsel du kan klikke deg forbi.
https-untrusted = Denne nettleseren stoler ikke på det for { $host } ennå. Du får en advarsel du kan klikke deg forbi.
https-now = Nå
https-start-failed = HTTPS er satt opp, men kunne ikke starte: { $error }
https-on-port = HTTPS er på, på port { $port } i likhet med http://.
https-using-it = Du bruker det nå.
https-open = Åpne { $url }
https-trusted = Denne nettleseren stoler på det.
https-cert-for-before = Sertifikat for{ " " }
https-cert-for-after = , til { $date }.
https-cert-for-after-own = , til { $date } (fornyes av seg selv).
https-not-covered-before = Det dekker ikke{ " " }
https-not-covered-after = , adressen du bruker.
https-is-off = HTTPS er av: sidene sendes over vanlig http://.
https-addresses = Adresser
https-addresses-hint = Sertifikatet dekker adressene Hoard fant, og de du legger til (et navn ruteren gir den, et Tailscale-navn).
https-remove-name = Fjern { $name }
https-another-address = En annen adresse
https-another-address-placeholder = En annen adresse, f.eks. nas.local
https-add = Legg til
https-your-cert = Ditt sertifikat
https-saved-cert-before = Lagret:{ " " }
https-saved-cert-after = { " " }fra { $issuer }, til { $date }. Lim inn et nytt for å erstatte det.
https-pem-before = Sertifikatet (med kjeden) og den private nøkkelen, som PEM-tekst: filene som begynner med{ " " }
https-pem-after = .
https-certificate = Sertifikat
https-choose-file = Velg fil …
https-private-key = Privat nøkkel
https-checked-before = For{ " " }
https-checked-after = , fra { $issuer }, til { $date }.
https-checked-after-days =
    , fra { $issuer }, til { $date } ({ $days ->
        [one] { $days } dag
       *[other] { $days } dager
    }).
https-self-signed = Det er selvsignert: nettlesere vil advare om det.
https-key-matches = Nøkkelen passer.
https-where-to-get = Hvor du får et
https-get-tailscale-before = slå på HTTPS for tailnettet ditt, og kjør deretter{ " " }
https-get-tailscale-after = { " " }på serveren og last opp de to filene. De varer i 90 dager: kjør kommandoen igjen og last dem opp på nytt før det.
https-get-acme-title = Let’s Encrypt eller en annen ACME-klient
https-get-acme-before = (certbot, acme.sh, lego) for et domene du eier: last opp{ " " }
https-get-acme-and = { " " }og{ " " }
https-get-acme-after = .
https-get-proxy-title = En omvendt proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) som har sertifikatet: la HTTPS være av her, og la proxyen sende vanlig http:// til Hoard.
https-trying = Prøver det …
https-apply = Bruk
https-apply-hint = Hoard prøver det nye oppsettet først, og beholder det den har hvis det ikke fungerer.
https-trust-title = Stol på Hoard på enhetene dine
https-download-ca = Last ned hoard-ca.crt
https-trust-hint = Installer Hoards sertifiseringsinstans én gang på hver enhet, så stoler nettleserne der på denne serveren (og på alle sertifikater Hoard lager for den senere). Den kan bare gå god for denne Hoard. Fingeravtrykk (SHA-256):
https-ca-windows = Åpne filen → Installer sertifikat → Gjeldende bruker → Plasser alle sertifikater i følgende lager → Bla gjennom → Klarerte rotsertifiseringsinstanser → Fullfør. Start nettleseren på nytt.
https-ca-macos = Åpne filen; den havner i Nøkkelringtilgang. Dobbeltklikk på «{ $subject }» der → Klarering → Ved bruk av dette sertifikatet: Klarer alltid.
https-ca-ios-title = iPhone og iPad
https-ca-ios = Last den ned i Safari og tillat den. Innstillinger → Profil lastet ned → Installer. Deretter Innstillinger → Generelt → Om → Innstillinger for sertifikatklarering → slå på «{ $subject }».
https-ca-android = Innstillinger → Sikkerhet → Flere sikkerhetsinnstillinger → Kryptering og legitimasjon → Installer et sertifikat → CA-sertifikat, og velg filen. (Navnene varierer litt mellom telefoner; søk etter «sertifikat» i Innstillinger.)
https-ca-linux-1 = Debian og Ubuntu:{ " " }
https-ca-linux-2 = . Fedora og Arch:{ " " }
https-ca-linux-3 = . Chrome på Linux har sin egen liste: Innstillinger → Personvern og sikkerhet → Sikkerhet → Administrer sertifikater → Importer.
https-ca-firefox = Firefox har sin egen liste: Innstillinger → Personvern og sikkerhet → Sertifikater → Vis sertifikater → Sertifiseringsinstanser → Importer, og kryss av for «Stol på at denne CA-en kan identifisere nettsteder».
https-ca-cli-title = Kommandolinjeverktøyet hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , eller legg banen i{ " " }
https-ca-cli-after = .
https-options = Valg
https-redirect = Send nettlesere på andre enheter fra http:// til https://
https-redirect-hint = Bare sidelastinger sendes videre. Kommandolinjeverktøyet hoard, Unreal-arbeideren, skript og AI-apper fortsetter å fungere på http://, og det gjør også http://localhost på selve serveren.
https-http3-hint-before = Raskere på ustabilt Wi-Fi og på telefoner. Krever HTTPS, og at UDP-port { $port } er åpen i tillegg til TCP (i Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Kjører.
https-http3-waits = Det starter når HTTPS er på.
https-http3-failed = HTTP/3 kunne ikke starte: { $error }
https-lockout-title = Hvis du blir stengt ute
https-lockout-1 = { "" }
https-lockout-2 = { " " }på selve serveren fungerer alltid over vanlig http://. Eller slå av HTTPS fra serverens kommandolinje og start Hoard på nytt:{ " " }
https-lockout-3 = { " " }(i Docker:{ " " }
https-lockout-4 = , deretter{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Innstillinger → Konto)

account-role-view = Kan se
account-role-view-hint = Søke, forhåndsvise og laste ned
account-role-edit = Kan endre
account-role-edit-hint = I tillegg legge til, endre, tagge, samle og arkivere
account-role-admin = Administrator
account-role-admin-hint = I tillegg Innstillinger, kontoer og vedlikehold, og å slette for godt
account-source-internal = Et Hoard-passord
account-source-ldap = Passordet deres i { $directory }
account-source-oauth = Pålogging med { $provider }
account-source-word-directory = katalog
account-load-failed = Kunne ikke laste kontoene: { $error }
account-failed = { $what }: { $error }
account-password-failed = Kunne ikke endre passordet ditt
account-password-changed = Passordet er endret. De andre enhetene dine er logget av.
account-add-failed = Kunne ikke legge til { $name }
account-added = { $name } kan logge på nå.
account-role-failed = Kunne ikke endre hva { $name } kan gjøre
account-remove-title = Fjerne { $name }?
account-remove-body = Vedkommende logges av overalt og kan ikke logge på igjen. Ingenting vedkommende har lagt til i biblioteket, fjernes.
account-remove = Fjern
account-remove-failed = Kunne ikke fjerne { $name }
account-token-failed = Kunne ikke lage tokenet
account-copied = Kopiert
account-copy-token-failed = Kunne ikke kopiere: merk tokenet og kopier det selv
account-copy-failed = Kunne ikke kopiere: merk teksten og kopier den selv
account-revoke-title = Tilbakekalle «{ $name }»?
account-revoke-body = Alt som fortsatt bruker det, blir avvist fra nå av.
account-revoke = Tilbakekall
account-revoke-failed = Kunne ikke tilbakekalle «{ $name }»
account-disconnect-title = Koble fra { $app }?
account-disconnect-body = Den blir avvist fra nå av, og må kobles til igjen for å bruke Hoard.
account-disconnect = Koble fra
account-disconnect-failed = Kunne ikke koble fra { $app }
account-guest-failed = Kunne ikke endre det
account-never = aldri
account-you = Du
account-sign-out = Logg av
account-signed-in-as-before = Pålogget som{ " " }
account-signed-in-as-admin = , som er administrator.
account-signed-in-as-edit = , som kan endre biblioteket.
account-signed-in-as-view = , som kan se, men ikke endre noe.
account-password-at-provider = Passordet ditt holdes av leverandøren du logger på med, så du endrer det der.
account-password-at-directory = Passordet ditt holdes av katalogen du logger på med, så du endrer det der.
account-current-password = Nåværende passord
account-new-password = Nytt passord
account-new-password-again = Nytt passord igjen
account-change-password = Endre passord
account-password-mismatch = De nye passordene er ikke like.
account-password-rule = Minst 8 tegn.
account-people = Personer
account-people-hint = Hver person kan se (søke, forhåndsvise og laste ned), også endre biblioteket (legge til, redigere, tagge, samle og arkivere), eller i tillegg være administrator (Innstillinger, kontoer og vedlikehold, og å slette for godt).
account-you-tag = (deg)
account-signs-in-oauth = Logger på med OAuth-leverandøren
account-signs-in-directory = Logger på med katalogen
account-since = siden { $date }
account-what-can-do = Hva { $name } kan gjøre
account-name = Navn
account-how-sign-in = Hvordan vedkommende logger på
account-password-label = Passord (minst 8 tegn)
account-password-placeholder = Passord (8+ tegn)
account-what-they-can-do = Hva vedkommende kan gjøre
account-add = Legg til
account-ai-apps = Claude og andre AI-apper
account-ai-apps-hint = Koble til Claude (eller en annen MCP-app), så kan den søke etter og hente ressurser for deg, og beskrive og tagge det Hoard ikke har gjort ennå. Den logger på Hoard første gang; du gir den lov her i nettleseren.
account-mcp-address = Hoards MCP-adresse
account-copy = Kopier
account-claude-code-1 = kjør{ " " }
account-claude-code-2 = , og deretter{ " " }
account-claude-code-3 = { " " }for å logge på.{ " " }
account-claude-code-4 = { " " }beskriver en pakke.
account-claude-ai = Settings → Connectors → Add custom connector, lim inn adressen, og velg deretter Connect. Disse kobler til fra Anthropics servere, så Hoard må kunne nås fra internett over HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, lim inn adressen, og velg deretter Connect. Disse kobler til fra Anthropics servere, så Hoard må kunne nås fra internett over HTTPS (denne adressen kan ikke det: angi Offentlig adresse under Innstillinger → Tilkobling).
account-connection-meta = for { $user } siden { $since } · sist brukt { $used }
account-no-apps = Ingen apper er koblet til ennå.
account-tokens = Tilgangstokener
account-tokens-hint-1 = For kommandolinjeverktøyet{ " " }
account-tokens-hint-2 = , skript, MCP-klienter og Unreal-arbeideren: de sender det som{ " " }
account-tokens-hint-3 = . Et token kan gjøre det du velger, opptil det du selv kan, men aldri administrere kontoer eller tokener. Et som bare kan se, er nok til å søke og laste ned, og til å synkronisere et prosjekt.
account-tokens-admin = Du ser alles tokener og tilkoblede apper; andre ser sine egne.
account-made-before = { "" }
account-made-after = : kopier det nå. Det vises ikke igjen.
account-new-token = Det nye tokenet
account-cli-1 = For CLI-en:{ " " }
account-cli-2 = , eller legg det i{ " " }
account-cli-3 = .
account-done = Ferdig
account-token-meta = laget { $made } · sist brukt { $used }
account-token-meta-by = laget { $made } av { $by } · sist brukt { $used }
account-no-tokens = Ingen tokener ennå.
account-token-name = Hva som skal bruke det
account-token-name-placeholder = Hva som skal bruke det, f.eks. «Unreal-arbeider»
account-token-role = Hva det kan gjøre
account-make-token = Lag et token
account-guests = Gjester
account-guests-allow = La alle som kan nå serveren, bla uten å logge på
account-guests-hint = De kan søke, forhåndsvise og laste ned, men ikke legge til, endre eller slette noe, eller se Innstillinger.

## SignIn.vue

signin-create-title = Opprett kontoen din
signin-title = Logg på
signin-setup-lede = Denne serveren har ingen konto ennå. Den du lager her, kan gjøre alt; legg til kontoer for andre senere under Innstillinger → Konto.
signin-overlay-lede = Her kan alle bla, men endringer krever en konto.
signin-with = Logg på med { $provider }
signin-or-password = eller med navn og passord
signin-username = Brukernavn
signin-name = Navn
signin-new-password = Nytt passord
signin-password = Passord
signin-password-rule = Minst 8 tegn.
signin-confirm-password = Bekreft passordet
signin-mismatch = De to passordene er ikke like.
signin-setup-code = Oppsettskode
signin-setup-code-hint-before = Koden denne serveren fikk i{ " " }
signin-setup-code-hint-after = .
signin-creating = Oppretter …
signin-signing-in = Logger på …
signin-create = Opprett konto
signin-submit = Logg på
signin-keep-browsing = Fortsett å bla
signin-directory-hint = Bruk navnet og passordet ditt i { $directory }.
signin-forgot-1 = Glemt et Hoard-passord? Kjør{ " " }
signin-forgot-2 = { " " }på serveren (i Docker{ " " }
signin-forgot-3 = ) for å få et nytt.

## SetupWizard.vue (første besøk på en ny server)

setup-title = Hvor skal biblioteket ditt ligge?
setup-lede = Velg en mappe på serveren der Hoard skal oppbevare pakkene dine, eller pek den til en mappe med pakker du allerede har. Du kan legge til flere plasseringer, på andre disker, fra Innstillinger senere.
setup-unreachable = Kunne ikke nå serveren: { $error }
setup-try-again = Prøv igjen
setup-loading = Laster …
setup-start = Start biblioteket
setup-ready = Biblioteket ditt er klart
setup-managed-before = Nye pakker legges i{ " " }
setup-managed-after = . Slipp filer, mapper eller arkiver hvor som helst på siden for å legge til de første.
setup-watched-before = Hoard overvåker{ " " }
setup-watched-after = ; pakker dukker opp etter hvert som den finner dem. For også å kunne laste opp legger du til et bibliotek som Hoard holder, under Innstillinger → Biblioteksplasseringer.
setup-open = Åpne biblioteket

## ConnectApp.vue (en app som Claude ber om å logge på)

connect-title = Koble til { $app }?
connect-lede-1 = { " " }vil kunne søke i, laste ned, beskrive og tagge alt i denne Hoard, på vegne av{ " " }
connect-lede-2 = . Den kan ikke administrere kontoer eller tilgangstokener.
connect-back-before = Du sendes tilbake til{ " " }
connect-back-after = . Koble den fra når som helst under Innstillinger → Konto.
connect-allow = Tillat
connect-cancel = Avbryt
connect-failed = Kan ikke koble til denne appen
connect-go-library = Gå til biblioteket
connect-checking = Sjekker …

## InsecureNotice.vue (linjen som vises over vanlig http://)

insecure-text = Dette er vanlig http://, så noen ting krever HTTPS: 3D med WebGPU, det moderne utklippstavle-API-et, å installere Hoard som app, og Claude-koblinger.
insecure-switch = Bytt til https://
insecure-set-up = Sett opp HTTPS
insecure-dismiss = Ikke vis dette igjen

## FormatWizard.vue (Innstillinger → Butikkfiler → lære Hoard en butikks fil)

formatwizard-too-big = Denne filen er for stor til å leses som en butikkfil.
formatwizard-not-json = Denne filen er ikke gyldig JSON, så den vises som tekst.
formatwizard-read-failed = Kunne ikke lese filen: { $error }
formatwizard-back-title = Gå tilbake til skjemaet?
formatwizard-back-body = Endringer gjort i TOML-en går tapt: skjemaet skriver formatet på nytt ut fra det det har.
formatwizard-back-to-form = Tilbake til skjemaet
formatwizard-miss-name = Filene til formatet heter ikke { $file }. Legg navnet til under Filer.
formatwizard-miss-not-the-stores = Det formatet kjenner igjen butikkens filer på, finnes ikke i denne filen. Endre Kjenn igjen på.
formatwizard-miss-no-title = Det finner ingen tittel i denne filen. Velg tittelen.
formatwizard-miss-short-name = ikke navnet
formatwizard-miss-short-not-the-stores = ikke butikkens
formatwizard-miss-short-no-title = ingen tittel
formatwizard-saved-packs =
    Lagret { $file }. { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    } leses på nytt.
formatwizard-saved = Lagret { $file }.
formatwizard-save-failed = Kunne ikke lagre formatet: { $error }
formatwizard-zip-failed = Kunne ikke lage zip-filen: { $error }
formatwizard-stop-title = Slutte å lære denne filen?
formatwizard-stop-body = Formatet blir ikke lagret.
formatwizard-stop = Slutt
formatwizard-label = Formatveiviser
formatwizard-changing = Endrer { $name }
formatwizard-starting-from = Tar utgangspunkt i { $name }
formatwizard-title = Lær Hoard en butikks fil
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }i{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Rediger som TOML
formatwizard-file-reference = Filen, til referanse.
formatwizard-click-json = Klikk på { $field } i filen.
formatwizard-click-text = Klikk på { $field } i filen, eller merk den.
formatwizard-pick-first = Velg et felt til høyre, og klikk deretter på verdien her.
formatwizard-loading = Laster …
formatwizard-the-format = Formatet
formatwizard-syntax-before = Syntaksen er beskrevet i{ " " }
formatwizard-syntax-after = .
formatwizard-store = Butikk
formatwizard-store-placeholder = Navnet på butikken, f.eks. Kenney
formatwizard-files = Filer
formatwizard-recognize = Kjenn igjen på
formatwizard-recognize-json = En nøkkel bare butikkens filer har
formatwizard-recognize-text = En linje bare butikkens filer har
formatwizard-recognize-hint = Én måte per linje; hvilken som helst av dem holder. Trengs når andre filer har samme navn.
formatwizard-seller = Selger
formatwizard-seller-placeholder = Når filen ikke sier det
formatwizard-tags = Tagger
formatwizard-tags-placeholder = For alt den beskriver, kommadelt
formatwizard-fields = Felter
formatwizard-selector = Velger for { $field }
formatwizard-clear = Tøm { $field }
formatwizard-pick = Velg
formatwizard-what-it-reads = Hva det leser
formatwizard-read-title = Tittel
formatwizard-read-seller = Selger
formatwizard-read-license = Lisens
formatwizard-read-url = Butikkside
formatwizard-read-kind = Type
formatwizard-read-tags = Tagger
formatwizard-read-origin = Laget i
formatwizard-read-size = Størrelse
formatwizard-also = { $format } leser også denne filen. Dette formatet ville blitt prøvd først.
formatwizard-library =
    { $files ->
        [one] { $files } fil
       *[other] { $files } filer
    } i { $packs ->
        [one] { $packs } pakke
       *[other] { $packs } pakker
    } har et navn det godtar. Av { $tried } prøvde leser det { $read }.
formatwizard-library-none = Ingen fil i biblioteket har et navn det godtar.
formatwizard-share = Del det
formatwizard-share-hint = En zip-fil med formatet, denne filen som eksempel og hva eksemplet leses som, lagt opp som i Hoards repository, til å sende inn så alle får det. Fjern først alt i eksemplet som ikke bør deles.
formatwizard-sample = Eksempel som skal deles
formatwizard-download = Last ned { $file }
formatwizard-close = Lukk
formatwizard-use = Bruk det her
formatwizard-cancel = Avbryt
