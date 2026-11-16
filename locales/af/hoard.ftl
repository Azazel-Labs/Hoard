## Bedienerfoute

error-account-not-found = rekening nie gevind nie
error-asset-gone = bate nie gevind nie
error-asset-not-found = bate { $id } nie gevind nie
error-blob-not-found = blob { $hash } nie gevind nie
error-collection-not-found = versameling { $id } nie gevind nie
error-connection-not-found = verbinding nie gevind nie
error-derivative-not-found = { $kind }-afgeleide van { $id } nie gevind nie
error-file-in-pack-not-found = { $path } in pak { $id } nie gevind nie
error-format-not-found = formaat { $file } nie gevind nie
error-frame-sheet-aseprite-only = net Aseprite-lêers het 'n raamblad
error-frame-sheet-frames = dit het een raam, of te veel om uit te lê
error-location-not-found = ligging { $id } nie gevind nie
error-maintenance-task-not-found = instandhoudingstaak { $id } nie gevind nie
error-open-upload-not-found = oop oplaai { $id } nie gevind nie
error-original-not-found = oorspronklike lêer van { $id } nie gevind nie
error-pack-not-found = pak { $id } nie gevind nie
error-preview-too-big = hierdie prent is te groot om 'n ligte voorskou van te maak
error-project-not-found = projek { $name } nie gevind nie
error-saved-search-not-found = gestoorde soektog { $id } nie gevind nie
error-sign-in-first = meld eers aan (of stuur 'n toegangstoken)
error-those-assets-not-found = daardie bates nie gevind nie
error-thumbnail-not-found = duimnael vir { $id } nie gevind nie
error-token-not-found = token nie gevind nie
error-upload-not-found = oplaai { $id } nie gevind nie
error-pack-needs-name = 'n pak het 'n naam nodig
error-no-library = daar is nog geen biblioteekligging om lêers in te stoor nie: voeg 'n bestuurde een by in Instellings → Biblioteekliggings
error-upload-failed = oplaai het misluk: { $reason }
error-upload-no-files = geen lêers in oplaai nie
error-analyze-nothing-to-do = niks om te doen nie: rules=false en LLM is af
error-too-many-assets = te veel bates in een versoek
error-too-many-items = te veel items in een versoek
error-not-a-hash = nie 'n blake3-hekshash nie
error-merge-into-tag = into moet 'n etiket wees
error-nothing-to-merge = niks om saam te voeg nie
error-selection-size = gee tussen 1 en { $max } bate-ID's
error-collection-needs-name = 'n versameling het 'n naam nodig
error-collection-exists = daar is reeds 'n versameling met die naam { $name }
error-collection-empty = versameling { $name } is leeg
error-conversion-failed = kon nie na { $format } omskakel nie: { $reason }
error-max-size = max moet 'n mag van twee van 64 tot 16384 wees
error-replace-ext = ext moet 'n kort lêeruitbreiding wees
error-empty-file = leë lêer
error-replace-duplicate = die omgeskakelde lêer is 'n duplikaat van een wat reeds in die pak is; die pakket is daarmee saamgevoeg
error-ai-address = Die KI-adres moet met http:// of https:// begin
error-local-llm-action = action is list, download of delete, en die laaste twee het 'n model nodig
error-role = 'n Rol is view, edit of admin
error-admin-only = net 'n admin kan dit doen
error-view-only = hierdie rekening kan kyk maar niks verander nie: 'n admin kan dit verander in Instellings → Rekening
error-portal-only = meld by die portaal aan om rekeninge en tokens te bestuur
error-not-from-portal = hierdie versoek kom nie van die Hoard-portaal af nie
error-account-name = Kies 'n naam van hoogstens 64 karakters
error-password-length = Gebruik 'n wagwoord van minstens { $min } karakters
error-too-many-codes = Te veel verkeerde kodes: wag { $seconds } sekondes
error-already-set-up = Daar is reeds 'n rekening: meld aan
error-too-many-tries = Te veel verkeerde pogings: wag { $seconds } sekondes
error-directory-unavailable = Die gids kon nie jou wagwoord nagaan nie: { $reason }
error-password-elsewhere = Jou wagwoord word gehou deur die gids of verskaffer waarmee jy aanmeld
error-current-password = Jou huidige wagwoord is nie reg nie
error-account-source = 'n Rekening meld aan met Hoard (internal), 'n gids (ldap) of OAuth
error-account-exists = Daar is reeds 'n rekening met daardie naam
error-own-role = 'n Ander admin kan jou rol verander
error-remove-self = Jy kan nie jou eie rekening verwyder terwyl jy daarby aangemeld is nie
error-token-name = Noem die token na wat dit gaan gebruik, in hoogstens 80 karakters
error-token-role = 'n Token kan nie meer doen as die rekening wat dit maak nie
error-wrong-setup-code = Dit is nie die opstelkode wat in HOARD_SETUP_CODE aan die bediener gegee is nie
error-wrong-password = Daardie naam en wagwoord pas nie bymekaar nie
error-folder-missing = { $path } is nie daar nie ('n skyf wat nie gekoppel is nie?)
error-s3-path = 'n S3-ligging lyk soos s3://bucket
error-openai-auth-unreachable = kon nie auth.openai.com bereik nie
error-openai-unexpected = onverwagte antwoord van OpenAI
error-openai-answered = OpenAI het { $status } geantwoord: { $reason }
error-chatgpt-id-token-not-jwt = die ID-token is nie 'n JWT nie
error-chatgpt-id-token-issuer = die ID-token is van { $issuer }, nie { $expected } nie
error-chatgpt-id-token-audience = die ID-token is vir 'n ander toep
error-chatgpt-id-token-expired = die ID-token het verval
error-chatgpt-id-token-nonce = die ID-token behoort nie aan hierdie aanmelding nie
error-chatgpt-no-state = die adres het geen state nie; kopieer die hele adres
error-chatgpt-not-started = hierdie aanmelding is nie hier begin nie, of dit is meer as 15 minute gelede begin. Begin dit weer vanuit Instellings.
error-chatgpt-canceled = die aanmelding is gekanselleer
error-chatgpt-refused = ChatGPT het die aanmelding geweier: { $reason }
error-chatgpt-no-code = die adres het geen aanmeldkode nie; kopieer die hele adres
error-chatgpt-other-registration = ChatGPT het vir 'n ander registrasie van hierdie bediener geantwoord; begin die aanmelding weer
error-chatgpt-not-registered = ChatGPT het nie klaar Hoard geregistreer nie; begin die aanmelding weer
error-chatgpt-code-used = die aanmeldkode is reeds gebruik of het verval; meld weer aan
error-chatgpt-refused-code = ChatGPT het die aanmelding geweier ({ $code })
error-chatgpt-no-id-token = ChatGPT het geen ID-token gestuur nie
error-chatgpt-plan-not-allowed = Aangemeld, maar die gebruik van die plan is nie toegelaat nie. Meld weer aan en laat dit toe, of gebruik eerder 'n API-sleutel.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = nie by ChatGPT aangemeld nie; meld aan vanuit Instellings → KI-etikettering
error-chatgpt-refresh-later = ChatGPT kon nie nou die aanmelding verfris nie ({ $code }); dit sal later weer probeer word
error-chatgpt-signed-out-problem = ChatGPT het hierdie bediener afgemeld ({ $code }). Meld weer aan om aan te hou etiketteer met jou plan.
error-chatgpt-signed-out = ChatGPT het hierdie bediener afgemeld ({ $code }); meld weer aan vanuit Instellings → KI-etikettering
error-openai-api-unreachable = kon nie api.openai.com bereik nie
error-openai-broke-off = die antwoord van OpenAI het afgebreek
error-chatgpt-kept-refusing = OpenAI het aangehou om die ChatGPT-aanmelding te weier; meld weer aan vanuit Instellings → KI-etikettering
error-chatgpt-request-failed = die versoek het misluk
error-chatgpt-cut-off = die antwoord is afgesny ({ $reason })
error-chatgpt-cut-off-no-reason = die antwoord is afgesny (geen rede gegee nie)
error-openai-ended-early = die antwoord van OpenAI het geëindig voordat dit volledig was
error-chatgpt-usage-limit = jou ChatGPT-plan se gebruik vir Hoard is vir eers op. Sien of verander die limiet by { $url }
error-chatgpt-not-eligible = hierdie ChatGPT-rekening of -werkruimte kan nie sy plan in ander toepe gebruik nie. Gebruik eerder 'n API-sleutel.
error-chatgpt-usage-unavailable = ChatGPT kon nie nou die plan se gebruik nagaan nie; dit sal later weer probeer word
error-chatgpt-unsupported = ChatGPT-plangebruik ondersteun nie { $param } met hierdie model nie
error-chatgpt-unsupported-images = ChatGPT-plangebruik ondersteun nie { $param } met hierdie model nie; kies 'n model wat prente aanvaar, of skakel die stuur van duimnaels af
error-chatgpt-unsupported-part = ChatGPT-plangebruik ondersteun nie 'n deel van die versoek met hierdie model nie
error-chatgpt-port = port is die een waarop die blaaier Hoard bereik
error-chatgpt-paste-address = plak die hele adres, wat met http://127.0.0.1 begin
error-too-many-hashes = hoogstens { $max } hashes op 'n slag
error-credits-pack = pak { $key }
error-credits-collection = versameling { $key }
error-format-too-long = die formaat is te lank
error-not-started = die bediener het nog nie begin nie
error-not-format-file = nie 'n formaat se lêer nie: { $name }
error-format-name = die naam het 'n letter of syfer van A tot Z of 0 tot 9 nodig
error-format-miss-name = die formaat se lêers aanvaar nie { $file } nie
error-format-miss-store = die formaat aanvaar nie { $file } vir dié van die winkel nie (sien require)
error-format-miss-title = die formaat vind geen titel in { $file } nie
error-cant-download-as = .{ $ext }-lêers kan nie as { $format } afgelaai word nie
error-only-images-resize = net prente kan van grootte verander word
error-https-no-certificate = Plak of laai eers jou sertifikaat en sy sleutel op
error-https-no-cert-found = Geen sertifikaat gevind nie: plak die teks wat met -----BEGIN CERTIFICATE----- begin
error-https-no-key-found = Geen privaat sleutel gevind nie: plak die teks wat met -----BEGIN PRIVATE KEY----- begin
error-https-several-keys = Daar is meer as een privaat sleutel: plak net die een wat by die sertifikaat hoort
error-https-key-password = Die sleutel is met 'n wagwoord beskerm: gee dit sonder een (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Die sertifikaat is nie geldige PEM nie
error-https-key-not-pem = Die privaat sleutel is nie geldige PEM nie
error-https-key-mismatch = Die sleutel behoort nie aan die sertifikaat nie. As jy 'n ketting gegee het, sit jou bediener se sertifikaat eerste
error-https-unusable = Die sertifikaat of sleutel kan nie gebruik word nie
error-https-expired = Hierdie sertifikaat het op { $date } verval
error-https-not-yet-valid = Hierdie sertifikaat is eers vanaf { $date } geldig
error-https-unreadable = Dit is nie 'n sertifikaat wat Hoard kan lees nie
error-https-bad-name = “{ $name }” is nie 'n adres wat 'n sertifikaat kan bevat nie
error-https-test-timed-out = die toetsverbinding het uitgetel
error-https-test-failed = die toetsverbinding het misluk
error-https-no-ca = Hoard het nog nie sy sertifikaatowerheid gemaak nie
error-https-unavailable = HTTPS is nie op hierdie bediener beskikbaar nie
error-https-certificate-mode = 'n Sertifikaat hoort by “My sertifikaat”
error-https-test = Die sertifikaat het nie in 'n toetsverbinding gewerk nie: { $reason }
error-nothing-to-retry = niks daar kan weer probeer word nie
error-ai-unreachable = kon nie { $url } bereik nie
error-ai-key-refused = { $url } het die API-sleutel geweier ({ $status })
error-ai-answered = { $url } het { $status } geantwoord: { $reason }
error-ai-not-json = { $url }/models het nie met JSON geantwoord nie
error-model-server-refused = die modelbediener: { $reason }
error-no-model-server = geen modelbediener antwoord by { $url } nie
error-not-llama-router = { $url } is nie 'n llama.cpp-bediener in router-modus nie, dus kan dit nie modelle aflaai nie
error-model-download = kon nie { $model } aflaai nie: { $reason }
error-model-delete = kon nie { $model } uitvee nie: { $reason }
error-unknown-kind = onbekende soort bate { $kind }
error-watched-is-folder = 'n gemonitorde ligging is 'n vouer op hierdie bediener
error-cant-reach = kan nie { $path } bereik nie: { $reason }
error-folder-full-path = gee die vouer se volle pad
error-cant-create = kan nie { $path } skep nie
error-cant-write = kan nie na { $path } skryf nie
error-cant-read = kan nie { $path } lees nie
error-overlaps-location = dit oorvleuel met die ligging { $name } ({ $path })
error-overlaps-data = dit oorvleuel met Hoard se datavouer ({ $path })
error-overlaps-cache = dit oorvleuel met Hoard se kasvouer ({ $path })
error-not-watched = { $name } is nie 'n gemonitorde vouer nie
error-watched-no-kinds = 'n gemonitorde vouer neem nie nuwe lêers aan nie, dus het dit geen soorte om te verander nie
error-keep-catch-all = maak eerder 'n ander ligging die vang-alles
error-location-holds-files = { $name } bevat nog lêers: skuif hulle eers na die ander liggings (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } kan nie bereik word nie, dus kan Hoard nie sê of dit nog lêers bevat nie
error-relocate-running = lêers word reeds tussen liggings geskuif; probeer weer wanneer dit klaar is
error-watched-not-moved = { $name } is 'n gemonitorde vouer: Hoard skuif nie sy lêers nie
error-no-other-location = daar is geen ander biblioteekligging om die lêers na te skuif nie: voeg eers een by
error-full-path = gee 'n volle pad
error-pick-folder = kies 'n vouer
error-task-running = “{ $task }” loop reeds
error-token-account-gone = hierdie token se rekening bestaan nie meer nie, dus het dit geen gunstelinge of soektogte nie
error-favorites-too-many = hoogstens 10 000 op 'n slag
error-saved-search-name = 'n gestoorde soektog het 'n naam van hoogstens 80 karakters nodig
error-saved-search-too-long = daardie soektog is te lank om te stoor
error-texture-pattern-map = Die tekstuurnaampatroon het { "{" }map{ "}" } nodig, anders kry elke kaart van 'n materiaal dieselfde naam
error-texture-pattern-fields = Die tekstuurnaampatroon kan { "{" }name{ "}" }, { "{" }map{ "}" } en { "{" }size{ "}" } gebruik
error-texture-name-chars = Tekstuurname kan nie < > : " / \ | ? of * bevat nie
error-texture-map-type = Daar is geen kaarttipe met die naam { $role } nie
server-role-base-color = kleur
server-role-normal = normaal
server-role-normal-dx = normaal (DirectX)
server-role-roughness = grofheid
server-role-gloss = glans
server-role-metalness = metaalagtigheid
server-role-ao = omgewingsokklusie
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = hoogte
server-role-bump = bult
server-role-cavity = holte
server-role-specular = spekulêr
server-role-opacity = ondeursigtigheid
server-role-emissive = emissie
server-role-translucency = deurskynendheid
error-texture-suffix-missing = Gee kaarte vir { $map } 'n agtervoegsel
error-texture-suffix-chars = Die agtervoegsel van kaarte vir { $map } kan nie < > : " / \ | ? of * bevat nie
error-oauth-no-client-id = Hierdie skakel het geen client_id nie.
error-oauth-unknown-app = Hierdie toep is nie (meer) hier geregistreer nie. Begin weer vanuit die toep koppel.
error-oauth-redirect = Die toep het gevra om teruggestuur te word na 'n adres wat dit nie geregistreer het nie.
error-oauth-response-type = Hoard deel net aanmeldkodes uit (response_type=code).
error-oauth-pkce = Die toep moet PKCE met S256 gebruik om by Hoard aan te meld.
error-light-preview-images = net prente het 'n ligte voorskou
error-no-light-preview = geen ligte voorskou nie: { $reason }
error-no-frame-sheet = geen raamblad nie: { $reason }
error-thumbnail-source = onbekende duimnaelbron: { $source }
error-thumbnail-too-big = daardie prent is te groot vir 'n duimnael
error-thumbnail-models-only = net modelle neem 'n duimnael aan wat deur 'n blaaier geteken is
error-not-a-picture = nie 'n prent wat Hoard kan lees nie: { $reason }
error-project-name = 'n projek het 'n naam van hoogstens 200 karakters nodig
error-project-too-big = hoogstens 'n miljoen bates in een projek
error-project-too-many-at-once = hoogstens 100 000 op 'n slag
error-server-name = Kies 'n naam van hoogstens { $max } karakters, op een reël
error-tool-setting = { $tool }: kies af, outomaties, of gee 'n pad
error-ai-model = Kies die model wat die KI-diens moet gebruik
error-public-address = Die publieke adres moet met http:// of https:// begin
error-update-hook-address = Die adres om vir bywerkings te roep moet met http:// of https:// begin
error-update-hook-auth = Die Authorization-kopreël kan nie reëlbreuke of ander beheerkarakters bevat nie
error-no-soundfont = Daar is geen SoundFont by { $path } op die bediener nie
error-workers = Agtergrondwerkers: tussen 1 en { $max }
error-ai-batch = Bates per KI-versoek: tussen 1 en 200
error-conversion-cache = Omgeskakelde aflaaie: minstens 100 MB
error-limits-zero = Limiete kan nie nul wees nie
error-settings-object = 'n JSON-objek met instellings is verwag
error-bad-setting = foutiewe instelling: { $reason }
error-auth-default-role = Die verstekrol is view, edit of admin
error-auth-session-hours = Sessies duur tussen 1 uur en 'n jaar
error-auth-settings = foutiewe verifikasie-instellings: { $reason }
error-ldap-settings = foutiewe LDAP-instellings: { $reason }
error-ldap-no-server = Voeg die gidsbediener se adres by
error-ldap-base-dn = Sê waar mense se inskrywings is (die basis-DN)
error-ldap-username-attribute = Sê watter attribuut die naam bevat waarmee mense aanmeld
error-ldap-bind-dn = Gee die rekening wat mense opsoek, of soek hulle anoniem op
error-ldap-ca-cert = Die sertifikaatowerheid is nie 'n PEM-sertifikaat nie (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = 'n Gidsbediener se adres is leeg
error-ldap-server-form = { $server }: 'n gidsbediener se adres is host:port, ldap://… of ldaps://…
error-ldap-not-starttls = { $server }: dit is LDAPS, nie StartTLS nie
error-ldap-no-certificate = geen sertifikaat daarin nie
error-ldap-password-again = Gee weer die wagwoord van die rekening wat mense opsoek: die gestoorde een word net gestuur na die bedieners waarvoor dit gegee is
error-ldap-unreachable = geen gidsbediener kon bereik word nie ({ $reasons })
error-ldap-no-anonymous = die gids laat nie toe dat mense anoniem opgesoek word nie ({ $reason })
error-ldap-bind-failed = die rekening wat mense opsoek kon nie aanmeld nie ({ $reason })
error-ldap-search = soek { $name } onder { $base }
error-ldap-group-search = soek { $name } se groepe onder { $base }
error-ldap-several = { $count } inskrywings in die gids pas by { $name }: vernou die gebruikerfilter
error-ldap-password-check = die gids wou nie { $name } se wagwoord nagaan nie ({ $reason })
error-ldap-wrong-password = Die gids ken nie { $name } met daardie wagwoord nie
error-oauth-client-id = Gee die client ID wat die verskaffer vir Hoard gemaak het
error-oauth-addresses = Die magtigings- en token-adresse moet met https:// begin
error-oauth-userinfo-address = Die gebruikersinligting-adres moet met https:// begin
error-oauth-username-claim = Sê watter claim die persoon se naam bevat
error-oauth-secret-again = Gee weer die client secret: die gestoorde een word net gestuur na die token-adres waarvoor dit gegee is
error-oauth-issuer = Die issuer moet met https:// begin
error-oauth-discovery-unreachable = Kon nie { $url } bereik nie: { $reason }
error-oauth-discovery-status = { $url } het { $status } geantwoord
error-oauth-discovery-bad = { $url } is nie 'n discovery-dokument nie
error-oauth-discovery-incomplete = { $url } noem nie die magtigings- en token-adresse nie
error-signin-no-group = { $name } is nie in 'n groep wat Hoard mag gebruik nie
error-signin-other-source = Daar is reeds 'n Hoard-rekening met die naam { $name } wat op 'n ander manier aanmeld
error-signin-other-subject = Daar is reeds 'n Hoard-rekening met die naam { $name } vir iemand anders by die verskaffer
error-signin-no-account = { $name } het nog geen Hoard-rekening nie: 'n admin kan dit byvoeg in Instellings → Rekening
error-signin-bad-name = Daardie naam kan nie 'n Hoard-rekening wees nie (hoogstens 64 karakters)
error-oauth-off = Aanmelding met OAuth is nie aangeskakel nie
error-oauth-bad-authorization-address = die magtigingsadres is nie geldig nie
error-oauth-refused = Die verskaffer het jou nie aangemeld nie: { $reason }
error-oauth-no-code = Die verskaffer het jou sonder 'n aanmelding teruggestuur
error-oauth-expired = Daardie aanmelding het te lank geneem of is elders begin: probeer weer
error-oauth-no-claim = Die verskaffer het nie gesê wie jy is nie (geen { $claim }-claim nie)
error-oauth-no-sub = Die verskaffer het nie gesê wie jy is nie (geen sub- of id-claim nie)
error-oauth-token-unreachable = kon nie die verskaffer se token-adres bereik nie
error-oauth-token-refused = Die verskaffer wou nie die aanmelding voltooi nie: { $reason }
error-oauth-no-access-token = die verskaffer het geen toegangstoken gestuur nie
error-oauth-userinfo-unreachable = kon nie die verskaffer se gebruikersinligting-adres bereik nie
error-oauth-userinfo-refused = Die verskaffer wou nie sê wie aangemeld het nie ({ $status })
error-oauth-userinfo-not-json = die verskaffer se gebruikersinligting is nie JSON nie
error-format-no-name = name is leeg
error-format-no-files = files is leeg: sê watter lêers die formaat lees, soos ["*.json"]
error-format-empty-selector = leeg
error-format-unopened-bracket = 'n sluithakie sonder 'n openingshakie
error-format-unclosed-bracket = 'n hakie is nie gesluit nie
error-format-empty-step-dots = 'n leë stap (twee punte agter mekaar, of 'n punt aan enige kant)
error-format-unclosed-except = !( is nie gesluit nie
error-format-unclosed-filter = [ is nie gesluit nie
error-format-after-filters = { $rest } ná { $step } se sleutel en filters
error-format-except-needs-glob = !(…) kom ná 'n sleutel met * daarin
error-format-except-not-glob = !(…) kom ná 'n sleutel met * daarin, nie { $key } nie
error-format-empty-step = 'n leë stap
error-format-not-a-filter = [{ $filter }] is nie 'n filter nie: gebruik [], [key=value|other], [array], [string], [number], [bool] of [object]
error-format-filter-needs-value = [{ $filter }] het 'n sleutel en 'n waarde nodig
error-store-search-empty = niks om na te soek nie; tik die pak se naam
error-update-hook-not-set = stel eers 'n adres om te roep
error-upload-stalled = oplaai van { $path } het vasgeval; stuur daardie deel weer
error-upload-busy = { $path } word reeds geskryf
error-upload-incomplete = { $count } lêers is onvolledig, bv. { $files }
error-upload-no-files-listed = geen lêers nie
error-upload-not-part = { $path } is nie deel van oplaai { $id } nie, of die oplaai is gesluit
error-bad-path = foutiewe pad
error-upload-part-offset = { $offset } is nie waar 'n deel van { $path } begin nie
error-upload-interrupted = oplaai onderbreek: { $reason }
error-upload-part-long = deel { $part } van { $path } is langer as { $len } grepe
error-upload-part-short = deel { $part } van { $path }: { $got } grepe ontvang, { $len } verwag
error-upload-file-long = { $path } is groter as wat aangegee is
error-upload-file-short = { $path }: { $got } grepe ontvang, { $len } verwag
error-upload-complete = oplaai is reeds voltooi
error-already-imported = reeds ingevoer as “{ $name }”

## Bedienerteks

server-store-place-readme = README
server-store-place-folders = vouers
server-store-why-linked = Gekoppel in die pak se { $place }
server-store-why-same-title = Dieselfde titel as die pak se { $place }
server-store-why-by = Deur { $author }, genoem in die pak se { $place }
server-store-why-meshes = Omtrent soveel mase as die pak ({ $count })
server-store-why-textures = Omtrent soveel teksture as die pak ({ $count })
server-chatgpt-revoke-failed = Hier afgemeld, maar OpenAI kon nie ingelig word nie. Om seker te wees, ontkoppel Hoard in ChatGPT se Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = By ChatGPT aangemeld
server-chatgpt-signed-in = Hoard is aangemeld. KI-etikettering kan nou jou ChatGPT-plan gebruik.
server-chatgpt-signed-in-as = Hoard is aangemeld as { $email }. KI-etikettering kan nou jou ChatGPT-plan gebruik.
server-chatgpt-not-finished-title = Aanmelding nie voltooi nie
server-chatgpt-other-computer = As jy die aanmelding vanuit Hoard op 'n ander rekenaar begin het, kopieer hierdie bladsy se adres uit die adresbalk en plak dit daar in Hoard se Instellings.
server-back-to-settings = Terug na Instellings
server-issues-pack = Pakke wat nie bygevoeg kon word nie
server-issues-pack-blurb = Vee die pak uit en voeg dit weer by, of maak dit oop om te sien wat dit bevat.
server-issues-damaged = Gestoorde lêers wat beskadig of weg is
server-issues-damaged-blurb = Gevind deur “Gaan lêers na vir skade”. Herstel hulle uit 'n rugsteun of voeg hul pak weer by, en laat loop dan “Gaan lêers na vir skade” weer.
server-issues-folder = Gemonitorde vouers wat nie gelees kan word nie
server-issues-folder-blurb = Hul pakke word gelaat soos hulle is totdat die vouer weer gelees kan word.
server-issues-missing = Gemonitorde pakke waarvan die vouer weg is
server-issues-missing-blurb = Elke pak word 3 dae gehou ingeval sy vouer net vanlyn is, en dan verwyder.
server-issues-job = Agtergrondtake wat misluk het
server-issues-job-blurb = Weer probeer laat die taak weer van voor af loop.
server-issues-file = Lêers wat nie oopgemaak kon word nie
server-issues-file-blurb = Hoard kon hulle nie oopmaak of stoor nie. 'n Gemonitorde pak probeer hulle weer by sy volgende skandering; 'n lêer wat reeds in die biblioteek is, het behou wat dit voorheen gehad het.
server-issues-unpack = Argiewe wat nie uitgepak kon word nie
server-issues-unpack-blurb = Elkeen word as 'n gewone lêer in sy pak gehou, dus is niks daarbinne in die biblioteek nie.
server-issues-read = Lêers waaruit Hoard nie wys kon word nie
server-issues-read-blurb = Hulle is in die biblioteek, maar sonder die feite, etikette of duimnael wat die lees daarvan gee.
server-issues-render = Modelle waarop die duimnaelweergewer misluk het
server-issues-render-blurb = Hulle het geen duimnael nie. Om een in die kyker oop te maak, kan dalk nog werk, en kan een stoor.
server-issues-convert = Modelle wat Blender nie kon omskakel nie
server-issues-convert-blurb = Hulle het geen GLB-kopie nie, dus kan hulle nie in 3D gewys word nie.
server-issues-unreal = Unreal-pakkette wat nie omgeskakel kon word nie
server-issues-unreal-blurb = Die pak se Unreal-bladsy het die res van sy pakkette.
server-issues-materials = Pakke waarvan die materiale nie uitgewerk kon word nie
server-issues-materials-blurb = Hul teksture is almal daar; hulle is net nie in materiale gegroepeer nie.
server-issues-material-thumb = Materiale sonder 'n duimnael
server-issues-material-thumb-blurb = Die materiaal is gevind, maar sy voorskou kon nie geteken word nie.
server-issues-store-files = Winkellêers wat nie gelees kon word nie
server-issues-store-files-blurb = Lêers van die winkel waar 'n pak vandaan kom, soos 'n Megascans-JSON, wat dit benoem en etiketteer.
server-issues-scan-again = Skandeer weer
server-issues-retry = Probeer weer
server-issues-reanalyze = Heranaliseer pak
server-issues-convert-again = Skakel weer om
server-issues-look-again = Kyk weer
server-issues-read-again = Lees weer
server-issues-blob-missing = Die gestoorde lêer is weg.
server-issues-blob-damaged = Die gestoorde lêer stem nie ooreen met wat gestoor is nie: dit is beskadig.
server-issues-folder-gone = Die vouer “{ $folder }” is nie gevind nie.
server-job-a-pack = 'n pak
server-job-ingest = Voer { $pack } in
server-job-retag = Her-etiketteer { $pack }
server-job-llm-tag = Etiketteer { $pack } met KI
server-job-convert = Skakel { $pack } om met Blender
server-job-unreal = Skakel Unreal-pakkette in { $pack } om
server-job-scan = Skandeer 'n gemonitorde vouer
server-job-relocate = Skuif lêers tussen biblioteekliggings
server-job-looks = Werk kleure en voorkoms uit
server-job-sounds = Luister na klanke
server-job-embed = Leer wat prente wys
server-job-materials = Soek materiale in { $pack }
server-job-read-files = Lees die lêers van { $pack }
server-job-store-lookup = Soek 'n winkelbladsy op
server-maintenance-backup = Rugsteun die databasis
server-maintenance-backup-blurb = 'n Konsekwente kopie van die databasis (pakke, etikette, notas, versamelings en Instellings) in die rugsteunvouer langsaan. Die biblioteek se lêers is nie daarin nie: hulle verander nooit nadat hulle gestoor is nie, dus rugsteun enige lêersinkronisering hulle.
server-maintenance-scrub = Gaan lêers na vir skade
server-maintenance-scrub-blurb = Lees elke gestoorde lêer terug en vergelyk dit met sy vingerafdruk, om lêers te vang wat verrot het of van 'n skyf of NAS verdwyn het. Neem 'n ruk op 'n groot biblioteek.
server-maintenance-reanalyze = Heranaliseer elke pak
server-maintenance-reanalyze-blurb = Lees elke lêer weer met hierdie weergawe van Hoard: duimnaels, voorskoue, besonderhede en reëletikette word opnuut gemaak. Jou eie wysigings bly; die KI word nie weer gevra nie.
server-maintenance-clear-cache = Maak gekasde omskakelings skoon
server-maintenance-clear-cache-blurb = Gooi die lêers weg wat van “Laai af as”-omskakelings gehou is (hulle word weer gemaak wanneer hulle volgende keer gevra word) en wat onderbreekte werk agtergelaat het.
server-maintenance-shrink-thumbs = Verklein duimnaels
server-maintenance-shrink-thumbs-blurb = Stoor die duimnaels en modelaansigte wat as PNG gehou word soos Instellings → Werkverrigting → Duimnaels vra, wat hulle gewoonlik etlike kere kleiner maak. Dié met deursigtigheid, en pixelkuns, bly PNG.
server-maintenance-gc = Vee ongebruikte lêers uit
server-maintenance-gc-blurb = Vee gestoorde lêers uit waarna niks meer verwys nie, soos die lêers van pakke wat vir altyd uitgevee is.
server-update-no-address = geen adres om te roep is gestel nie
server-update-no-answer = geen antwoord binne 30 sekondes nie

## Instellings: Voorkoms

appearance-language = Taal
appearance-language-auto = Blaaier s'n ({ $name })

## Wortel

root-unreachable = Kon nie die Hoard-bediener bereik nie: { $error }
root-try-again = Probeer weer
root-lazy-connect = die toepverbinding

## Toep: die biblioteek, sy boonste balk en filters

app-lazy-settings = instellings
app-lazy-archive = die argief
app-lazy-shortcuts = die kortpaaie
app-lazy-setup = opstelling
app-theme-system = Volg stelseltema
app-theme-light = Ligte tema
app-theme-dark = Donker tema
app-color-red = Rooi
app-color-orange = Oranje
app-color-yellow = Geel
app-color-green = Groen
app-color-teal = Blougroen
app-color-blue = Blou
app-color-purple = Pers
app-color-pink = Pienk
app-color-brown = Bruin
app-color-sand = Sand
app-color-gray = Grys
app-color-slate = Leiklip
app-color-black = Swart
app-color-white = Wit
app-color-swatch = { $color }: bates met baie { $lower } daarin
app-color-any = Enige kleur
app-color-pick = Kies enige kleur
app-color = Kleur
app-retry = Probeer weer
app-undo = Ontdoen
app-search-failed = Soektog het misluk: { $error }
app-unreachable = Kon nie die bediener bereik nie: { $error }
app-forget-project-title = Vergeet { $name }?
app-forget-project-body = Hoard hou op om dit te lys waar sy { $count } bates gebruik word, totdat sy volgende hoard sync dit weer vertel. Niks in die projek verander nie.
app-forget-project-confirm = Vergeet dit
app-forget-project-failed = Kon nie { $name } vergeet nie: { $error }
app-forget-project-label = Vergeet die projek { $name }
app-forget-project = Vergeet hierdie projek
app-save-name-like = soos { $name }
app-save-name-favorites = gunstelinge
app-search-saved = “{ $name }” gestoor: dit is in Filters, onder Joune
app-rank-alike = Mees eenders
app-rank-color = Meeste van die kleur
app-rank-match = Beste passing
app-sort = Sorteer
app-sort-newest = Nuutste
app-sort-name = Naam
app-sort-largest = Grootste
app-sort-oldest = Oudste
app-collection-added =
    { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    } by { $name } gevoeg
app-collection-add-failed = Kon nie by die versameling voeg nie: { $error }
app-collection-removed = { $count } uit { $name } verwyder
app-collection-remove-failed = Kon nie uit die versameling verwyder nie: { $error }
app-ns-subject = Onderwerp
app-ns-style = Styl
app-ns-theme = Tema
app-ns-material = Materiaal
app-ns-map = PBR-kaart
app-ns-color = Kleur
app-ns-mood = Stemming
app-ns-genre = Genre
app-ns-license = Lisensie
app-ns-res = Resolusie
app-ns-duration = Lengte
app-ns-poly = Poligoontelling
app-ns-format = Formaat
app-ns-weight = Gewig
app-ns-tech = Tegnies
app-ns-blender = Blender-weergawe
app-ns-depth = Bisdiepte
app-ns-ucs = Klankkategorie (UCS)
app-ns-bpm = Tempo
app-ns-root = Grondnoot
app-ns-level = Luidheid
app-ns-ue = Unreal-klas
app-ns-engine = Enjin
app-ns-extracted = Herwonne bron
app-ns-script = Skrifstelsel
app-bulk-failed =
    Kon nie { $count ->
        [one] daardie bate
       *[other] daardie { $count } bates
    } verander nie: { $error }
app-bulk-untagged =
    { $tag } van { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    } afgehaal
app-bulk-tagged =
    { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    } met { $tag } geëtiketteer
app-bulk-category-set =
    Die kategorie van { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    } op { $category } gestel
app-bulk-category-cleared =
    Die kategorie van { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    } skoongemaak
app-credits-failed = Kon nie die erkennings maak nie: { $error }
app-archived-assets =
    { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    } geargiveer
app-archived-one = { $name } geargiveer
app-open-failed = Kon nie daardie bate oopmaak nie: { $error }
app-unnamed-pack = pak
app-unnamed-collection = versameling
app-unnamed-asset = bate
app-home-version = Hoard { $version } — terug na die biblioteek
app-home = Terug na die biblioteek
app-search-label = Soek bates
app-search-placeholder = Soek name, paaie, etikette
app-views = Aansigte
app-library = Biblioteek
app-packs = Pakke
app-packs-working = { $count } besig
app-update-title = Hoard { $version } is uit (hierdie bediener loop { $current }). Trek die nuwe image of binêre lêer en herbegin; die databasis gradeer homself op.
app-update = Bywerk
app-theme = Tema
app-sign-in = Meld aan
app-archive-title = Argief: { $contents }
app-archive = Argief
app-settings = Instellings
app-palette-title = Opdragpalet ({ $keys })
app-import-folder = Voer vouer in
app-add-files = Voeg lêers by
app-add-files-short = Voeg by
app-split-label = Hoe om hierdie vouer in te voer
app-split-before = { "" }
app-split-after = { " " }bevat { $count } vouers of argiewe.
app-split-each = Voer elkeen as sy eie pak in ({ $count })
app-split-one = Voer in as een pak
app-cancel = Kanselleer
app-filters = Filters
app-done = Klaar
app-yours = Joune
app-favorites = Gunstelinge
app-forget-search-label = Vergeet die gestoorde soektog { $name }
app-forget-search = Vergeet hierdie soektog
app-projects-title = Projekte wat van hier af hoard sync, met wat hul laaste sinkronisering gehad het
app-projects = Projekte
app-project-synced = { $when } gesinkroniseer
app-project-synced-by = { $when } deur { $who } gesinkroniseer
app-collections = Versamelings
app-no-filters = Geen filters vir hierdie soektog nie.
app-kind = Soort
app-facets-more = Nog { $count }
app-facets-fewer = minder
app-searching = Soek tans…
app-searching-live = Soek tans
app-asset-count =
    { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    }
app-clear-all = Maak alles skoon
app-save-name-label = Naam vir hierdie soektog
app-save-name-placeholder = Gee dit 'n naam
app-save = Stoor
app-save-search-title = Hou hierdie soektog in Filters, onder Joune
app-save-search = Stoor soektog
app-select-title = Kies etlike bates om saam te etiketteer, te kategoriseer, te versamel, af te laai of te argiveer (X, of Ctrl-klik)
app-select = Kies
app-tile-size = Teëlgrootte
app-supporting-title = glTF-buffers, materiaalbiblioteke, die kaarte van materiale, materiale wat net een model dra en die winkellêers wat modelle beskryf. Hulle kom saam wanneer jy aflaai wat hulle gebruik.
app-supporting-hidden =
    { $count } ondersteunende { $count ->
        [one] lêer
       *[other] lêers
    } versteek
app-supporting-show = Wys
app-supporting-shown = Ondersteunende lêers ingesluit
app-supporting-hide = Versteek
app-welcome-title = Laat val 'n batepak enige plek op hierdie bladsy
app-welcome-before = Zips, tarballs, vouers of los lêers werk almal. Hoard pak hulle uit, stoor elke lêer een keer volgens sy hash, lees groottes, lengtes en poligoontellings, en etiketteer alles sodat jy kan soek na dinge soos{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }of{ " " }
app-welcome-after = .
app-welcome-import-folder = Voer 'n vouer in
app-welcome-choose-files = Kies lêers
app-none-similar = Niks anders lyk nog baie soos { $name } nie.
app-none = Niks pas nie.
app-none-query = Niks pas by “{ $query }” nie.
app-none-filters = Niks pas met hierdie filters nie.
app-none-query-filters = Niks pas by “{ $query }” met hierdie filters nie.
app-clear-search-filters = Maak soektog en filters skoon
app-loading-more = Laai meer
app-keys-hint = Pyltjiesleutels skuif, Enter maak oop, Spasie speel 'n klank, X of Ctrl-klik kies etlike (druk lank op raakskerms).
app-resize-label = Verander die grootte van die besonderhedepaneel
app-resize-title = Sleep om grootte te verander · dubbelklik vir die breedte wat pas
app-selected-label = Gekose bates
app-selected = { $count } gekies
app-pick-assets = Kies bates
app-download-zip = Laai zip af
app-credits-title = CREDITS.md vir die pakke waaruit hierdie kom: wie hulle gemaak het, hul lisensies en die erkenningsreëls waarvoor hulle vra
app-credits = Erkennings
app-prefer-sources-title = Unreal-SoundWaves en -teksture waarvan Hoard die oorspronklike lêer herwin het
app-prefer-sources = Herwonne WAV/PNG in plaas van .uasset ({ $count })
app-bulk-tag-label = 'n Etiket om by die gekose bates te voeg, of daarvan af te haal
app-bulk-tag-placeholder = Etiket
app-bulk-tag-add-title = Voeg hierdie etiket by elke gekose bate
app-bulk-tag-add = Voeg by
app-bulk-tag-remove-title = Haal hierdie etiket van elke gekose bate af
app-bulk-tag-remove = Verwyder
app-bulk-category-label = Kategorie vir die gekose bates (leeg maak dit skoon)
app-bulk-category-placeholder = Kategorie
app-bulk-category-set-title = Stel die kategorie van elke gekose bate
app-bulk-category-clear-title = Maak die kategorie van elke gekose bate skoon
app-bulk-category-set-button = Stel
app-bulk-category-clear-button = Maak skoon
app-remove-from-collection = Verwyder uit { $name }
app-unfavorite = Haal uit gunstelinge
app-favorite = Gunsteling
app-select-all = Kies al { $count }
app-archive-selected-title = Uit soektogte totdat hulle herstel of vir altyd uitgevee word, vanuit die Argief
app-drop-items = Laat val { $count } items om hulle by te voeg
app-drop = Laat val om by die hoard te voeg
app-drop-hint = Elke lêer of vouer op die boonste vlak word sy eie pak. Gigagrepe is geen probleem nie; groot lêers gaan voort as die verbinding wegval.

## Opdragpalet (CommandPalette.vue, met sy opdragte uit App.vue)

palette-group-go-to = Gaan na
palette-group-actions = Aksies
palette-group-theme = Tema
palette-group-kinds = Soorte
palette-group-packs = Pakke
palette-group-collections = Versamelings
palette-group-projects = Projekte
palette-group-saved = Gestoorde soektogte
palette-group-assets = Bates
palette-hint-current = huidige
palette-library = Biblioteek
palette-library-keywords = bates rooster tuis assets grid home
palette-favorites = Gunstelinge
palette-favorites-keywords = ster sterre gehou starred stars liked
palette-packs = Pakke
palette-archive = Argief
palette-archive-keywords = asblik herwin uitgevee herstel trash recycle bin deleted restore
palette-stats = Biblioteekstatistieke
palette-stats-keywords = statistiek getalle grootte skyfspasie telling statistics numbers size disk space count
palette-search = Soek
palette-add-files = Voeg lêers by…
palette-add-files-keywords = oplaai invoer upload import
palette-import-folder = Voer 'n vouer in…
palette-import-folder-keywords = oplaai pak upload pack
palette-select-all = Kies al { $count } wat gewys word
palette-select-all-keywords = merk check
palette-select-several = Kies etlike…
palette-select-several-keywords = grootmaat veelvuldig etiket argiveer bulk multiple tag archive
palette-archive-selected = Argiveer die { $count } gekose
palette-archive-selected-keywords = uitvee verwyder asblik delete remove trash
palette-clear-selection = Maak die keuse skoon
palette-clear-filters = Maak filters skoon
palette-clear-filters-keywords = terugstel reset
palette-hide-supporting = Versteek ondersteunende lêers
palette-show-supporting = Wys ondersteunende lêers
palette-supporting-keywords = normaalkaarte grofheid maskers normal maps roughness masks
palette-account = Rekening en toegangstokens
palette-account-keywords = wagwoord aanmeld gebruiker token api password login user
palette-sign-out = Meld af ({ $name })
palette-sign-out-keywords = afmeld uitteken log out logout
palette-sign-in = Meld aan
palette-sign-in-keywords = aanmeld rekening wagwoord log in login account password
palette-settings = Instellings
palette-settings-keywords = blender ffmpeg llm ki etikettering unreal admin gereedskap voorkeure ai tagging tools preferences
palette-shortcuts = Sleutelbordkortpaaie
palette-shortcuts-keywords = hulp sleutels help keys
palette-theme-keywords = tema voorkoms modus theme appearance mode
palette-theme-variant-keywords = voorkoms kleur appearance color
palette-light-theme = Ligte tema: { $name }
palette-dark-theme = Donker tema: { $name }
palette-kind-all = Alle soorte (hou op om net { $kind } te wys)
palette-kind-only = Net { $kind }
palette-title = Opdragpalet
palette-placeholder = Soek bates, gaan na 'n pak, voer 'n opdrag uit…
palette-results = Resultate
palette-searching = Soek tans…
palette-no-match = Niks pas by “{ $query }” nie
palette-foot-move = skuif
palette-foot-run = voer uit
palette-foot-shortcuts = alle kortpaaie

## Sleutelbordkortpaaie (ShortcutsDialog.vue)

shortcuts-title = Sleutelbordkortpaaie
shortcuts-close = Sluit
shortcuts-anywhere = Oral
shortcuts-palette = Opdragpalet: soek, spring, voer uit
shortcuts-search = Soek
shortcuts-this-list = Hierdie lys
shortcuts-escape = Sluit, maak die keuse skoon, of sluit die besonderhede
shortcuts-library = Biblioteek
shortcuts-move = Skuif tussen teëls (of J en K)
shortcuts-open = Maak die besonderhede oop
shortcuts-play-sound = Speel of stop 'n klank
shortcuts-select = Kies of ontkies
shortcuts-favorite = Voeg by of haal uit jou gunstelinge
shortcuts-key-click = klik
shortcuts-select-range = Kies 'n reeks
shortcuts-select-all = Kies alles wat gewys word
shortcuts-sound-player = Klankspeler
shortcuts-play-pause = Speel of pouseer
shortcuts-skip = Spring terug of vorentoe
shortcuts-start-end = Begin of einde
shortcuts-image-inspector = Prentinspekteur
shortcuts-zoom = Zoem in of uit
shortcuts-fit = Pas in die venster
shortcuts-actual-pixels = Werklike pixels
shortcuts-tile = Teël 3×3 om die nate na te gaan
shortcuts-background = Volgende agtergrond
shortcuts-channel = Wys een kanaal

## KindChips.vue

kinds-group = Batesoorte

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = deur 'n persoon bygevoeg
tag-source-rule = van die vanlyn etiketteerder
tag-source-meta = uit die lêer gemeet
tag-source-ai = deur KI voorgestel
tag-remove = Verwyder etiket { $tag }

## ActivityIndicator.vue

activity-label-idle = Agtergrondaktiwiteit: ledig
activity-label-running = Agtergrondaktiwiteit: { $running } besig
activity-label-queued = Agtergrondaktiwiteit: { $running } besig, { $queued } in die tou
activity-title = Agtergrondaktiwiteit
activity-unreachable = Kan nie die bediener bereik nie
activity-empty = Niks loop nie. Invoere, etikettering, omskakelings en zip-aflaaie verskyn hier terwyl hulle werk.
activity-queued = in die tou
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } driehoeke
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Speel { $name }
tile-pause = Pouseer { $name }
tile-favorite = Maak { $name } 'n gunsteling
tile-in-favorites = In jou gunstelinge
tile-add-favorite = Voeg by jou gunstelinge (F)
tile-more-like = Meer soos { $name }
tile-more-like-this = Meer soos hierdie
tile-select = Kies { $name }

## CollectionPicker.vue

collections-add = Voeg by versameling
collections-menu = Versamelings
collections-find-label = Vind of skep 'n versameling
collections-find-placeholder = Vind of benoem 'n nuwe versameling
collections-create = Skep
collections-empty = Nog geen versamelings nie. Tik 'n naam om een te begin.

## FolderPicker.vue

folders-list = Vouers
folders-close = Sluit
folders-browse = Blaai…
folders-up-title = Een vouer op
folders-up = Op
folders-this-computer = Hierdie rekenaar
folders-none = Geen vouers hierin nie.

## LocationForm.vue

locations-mode-question = Wat moet Hoard met hierdie vouer doen?
locations-mode-managed = Hou my biblioteek hier
locations-mode-managed-hint = Pakke wat jy oplaai of invoer, word na hierdie vouer gekopieer. Hoard sorg daarvoor, so los die inhoud daarvan uit.
locations-mode-watched = Indekseer 'n vouer wat ek reeds het
locations-mode-watched-hint = Elke vouer daarbinne word 'n pak. Hoard lees die lêers waar hulle is en verander, skuif of vee hulle nooit uit nie, en tel nuwe vouers op soos hulle verskyn.
locations-folder = Vouer op die bediener
locations-folder-placeholder = Die vouer waarin jou pakke is
locations-folder-hint-managed = Hoard skep dit as dit nie daar is nie. In Docker is dit 'n vouer wat jy in die houer gekoppel het.
locations-folder-hint-watched = In Docker, koppel eers die vouer in die houer (net-lees is goed), en kies dit dan hier.
locations-name = Naam
locations-name-placeholder-managed = Biblioteek
locations-name-placeholder-watched = My pakke
locations-files = Lêers wat hierheen gaan
locations-catch-all = Ook alles wat geen ander ligging neem nie
locations-kinds-hint = 'n Nuwe pak se lêers gaan na die ligging wat hul soort aanmerk. Die res gaan na die vang-alles.
locations-files-all = Almal, vir eers. Voeg later meer liggings by vanuit Instellings om sommige soorte (musiek, byvoorbeeld) na 'n ander skyf te stuur.
locations-first-watched-note = Oplaai het 'n biblioteek nodig wat Hoard hou. Jy kan later een byvoeg vanuit Instellings → Biblioteekliggings.
locations-checking = Gaan die vouer na…
locations-add = Voeg ligging by
locations-cancel = Kanselleer

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Kon nie die biblioteekliggings laai nie
locations-takes-and-rest = { $kinds }, en alles anders
locations-takes-rest = Alles wat die ander nie neem nie
locations-takes-everything = Alles
locations-takes-nothing = Nog niks nie
locations-save-failed = Kon nie die ligging stoor nie
locations-unwatch-title = Hou op om { $name } te monitor?
locations-unwatch-body =
    Sy { $count ->
        [one] { $count } pak
       *[other] { $count } pakke
    } verlaat die hoard, saam met hul etikette en notas. Die lêers in die vouer word nie aangeraak nie.
locations-unwatch-body-empty = Die lêers in die vouer word nie aangeraak nie.
locations-unwatch = Hou op monitor
locations-remove-title = Verwyder { $name }?
locations-remove-body = Enige lêers daarin skuif eers na die ander liggings, volgens soort; dit verdwyn sodra dit leeg is. Niks gaan verlore nie.
locations-remove-body-only = As dit lêers bevat, voeg eers 'n ander ligging by waarheen hulle kan skuif.
locations-remove = Verwyder
locations-removed = { $name } verwyder
locations-emptying = Skuif die lêers uit { $name }; dit word verwyder wanneer dit klaar is
locations-offline-title = { $name } kan nie bereik word nie
locations-offline-body = Sy lêers kan nie uitgeskuif word terwyl dit vanlyn is nie. Verwyder dit in elk geval? Enige lêers wat nog daarop is, sal as vermis verskyn.
locations-remove-anyway = Verwyder in elk geval
locations-remove-failed = Kon nie { $name } verwyder nie
locations-rebalancing = Skuif gestoorde lêers na die liggings wat hul soort neem
locations-rebalance-failed = Kon nie die lêers skuif nie
locations-scanned-never = nog nie geskandeer nie
locations-scanned-now = so pas geskandeer
locations-scanned-minutes = { $mins } min gelede geskandeer
locations-scanned-hours = { $hours } h gelede geskandeer
locations-scanned-date = { $date } geskandeer
locations-scan-failed = Kon nie { $name } skandeer nie
locations-watching = Monitor { $name }
locations-added = { $name } bygevoeg
locations-loading = Laai…
locations-managed-title = Biblioteke wat Hoard hou
locations-watched-title = Gemonitorde vouers
locations-managed-hint = Oplaaie en invoere word hierheen gekopieer. Elke ligging neem die soorte wat dit aanmerk; die vang-alles neem die res.
locations-watched-hint = Vouers met pakke wat geïndekseer word waar hulle is. Hoard verander nooit die lêers daarin nie, en soek elke 15 minute na nuwe en verwyderde vouers.
locations-managed-empty = Nog geen nie, dus is daar nêrens om 'n oplaai te sit nie. Voeg hieronder een by.
locations-reachable = Bereikbaar
locations-unreachable-now = Kan nou nie bereik word nie
locations-catch-all-tag = vang-alles
locations-packs =
    { $count ->
        [one] { $count } pak
       *[other] { $count } pakke
    }
locations-unreachable = kan nie bereik word nie
locations-scanning = Skandeer…
locations-scan = Skandeer nou
locations-edit = Wysig
locations-catch-all-locked = Om dit te verander, maak 'n ander ligging die vang-alles
locations-edit-hint-before = Geld vir lêers wat van nou af bygevoeg word. Om te skuif wat reeds gestoor is, gebruik
locations-edit-hint-after = hieronder.
locations-save = Stoor
locations-rebalance = Skuif gestoorde lêers om te pas
locations-rebalance-hint = Nadat jy verander het wat 'n ligging neem: lêers wat reeds gestoor is, skuif na die ligging waarheen hul soort nou gaan.
locations-add-title = Voeg 'n ligging by
locations-add-hint = Nog 'n skyf vir sommige soorte lêers, of 'n vouer met pakke wat jy reeds het.
locations-add-more = Voeg 'n ligging by…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Kon nie Probleme laai nie
issues-started =
    { $count ->
        [one] Weer begin.
       *[other] { $count } dinge weer begin.
    }
issues-retry-failed = Kon nie weer probeer nie
issues-dismiss-failed = Kon nie wegwys nie
issues-dismiss-all-title = Wys al { $count } weg?
issues-dismiss-all-body = “{ $title }” word skoongemaak. Enigiets wat weer verkeerd loop, word weer gelys.
issues-dismiss-all = Wys almal weg
issues-just-now = so pas
issues-minutes-ago = { $mins } min gelede
issues-hours-ago = { $hours } h gelede
issues-loading = Laai…
issues-none = Niks het verkeerd geloop nie.
issues-action-all = { $action } almal
issues-open-asset = Maak dit oop
issues-open-pack = Wys die pak
issues-open-locations = Biblioteekliggings
issues-where-before = in
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } keer
       *[other] { $count } keer
    }
issues-starting = Begin…
issues-dismiss-named = Wys { $title } weg
issues-dismiss = Wys weg
issues-show-fewer = Wys minder
issues-show-all = Wys al { $count }
issues-not-listed = Nog { $count } word nie gelys nie.

## archive.ts

archive-anyway = Argiveer in elk geval
archive-failed = Kon nie argiveer nie: { $error }
archive-undo = Ontdoen
archive-restore-failed = Kon nie herstel nie: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pak
       *[other] { $packs } pakke
    } en { $assets ->
        [one] { $assets } bate
       *[other] { $assets } bates
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pak
       *[other] { $packs } pakke
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } bate
       *[other] { $assets } bates
    }
archive-count-nothing = niks

## ArchiveView.vue

archive-load-failed = Kon nie die argief laai nie: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = { $what } herstel
archive-purged = { $what } vir altyd uitgevee
archive-purge-failed = Kon nie uitvee nie: { $error }
archive-purge-title = Vee { $name } vir altyd uit?
archive-purge-pack-body =
    Sy { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    } en { $size } se lêers gaan ook weg, behalwe lêers wat 'n ander pak deel. Dit kan nie ontdoen word nie.
archive-purge = Vee vir altyd uit
archive-purge-asset-body = Sy lêer word vrygestel tensy 'n ander bate dit deel. Dit kan nie ontdoen word nie.
archive-empty-title = Maak die argief leeg?
archive-empty-body = { $what } sal vir altyd uitgevee word, en hul lêers vrygestel. Dit kan nie ontdoen word nie.
archive-empty = Maak argief leeg
archive-title = Argief
archive-intro = Geargiveerde pakke en bates is uit soektogte, die tellings en die etiketlys. Herstel hulle, of vee hulle vir altyd uit om hul spasie vry te stel.
archive-loading = Laai…
archive-is-empty = Die argief is leeg
archive-is-empty-hint = Argiveer 'n pak of 'n bate en dit wag hier totdat jy dit herstel of vir altyd uitvee.
archive-packs = Pakke
archive-assets = Bates
archive-asset-count =
    { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    }
archive-archived = { $when } geargiveer
archive-from = uit { $pack }
archive-restore = Herstel
archive-show-more = Wys meer ({ $count } oor)

## mine.ts

projects-favorite-add-failed = Kon nie by jou gunstelinge voeg nie: { $error }
projects-favorite-remove-failed = Kon nie uit jou gunstelinge haal nie: { $error }
projects-save-search-failed = Kon nie die soektog stoor nie: { $error }
projects-forget-search-failed = Kon nie { $name } verwyder nie: { $error }

## projects.ts

projects-list-and = { $items } en { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = nog { $count }
projects-other =
    { $count ->
        [one] { $count } ander projek
       *[other] { $count } ander projekte
    }
projects-usage-title-one = { $name } gebruik dit
projects-usage-title = { $count } projekte gebruik dit
projects-usage-body-one = Gebruik in { $list }: sy volgende hoard sync haal hulle uit.
projects-usage-body = Gebruik in { $list }: elkeen se volgende hoard sync haal hulle uit.

## uploader.ts

upload-network-error = netwerkfout
upload-gave-up = { $what }: { $error } ná { $attempts } pogings
upload-failed-plain = misluk
upload-already-imported = reeds ingevoer
upload-part-failed = oplaai het misluk ({ $status })
upload-batch = bondel van { $count } lêers
upload-batch-failed = bondel het misluk ({ $status })
upload-finish = voltooi oplaai

## UploadQueue.vue

upload-eta-seconds = { $s } s oor
upload-eta-minutes = { $min } min oor
upload-eta-hours = { $h } h oor
upload-queue = Invoere
upload-importing = Voer { $count } pakke in
upload-bytes-of = { $sent } van { $total }
upload-rate = { $size }/s
upload-waiting = Wag
upload-unpacking = Opgelaai, pak nou uit
upload-already = Reeds in die biblioteek
upload-stopped = Gestop
upload-files-of = { $done } van { $total } lêers
upload-resumed = hervat, { $size } was reeds daar
upload-skipped = Presies hierdie lêers is reeds as “{ $name }” ingevoer, dus is hulle oorgeslaan.
upload-error = { $error }. Jou vordering word gehou: probeer weer, of laat later dieselfde lêers weer val.
upload-retry = Probeer weer
upload-import-again = Voer in elk geval weer in
upload-dismiss = Wys weg
upload-cancel = Kanselleer

## PacksView.vue

packs-rename-failed = Kon nie hernoem nie: { $error }
packs-collection-deleted = Versameling { $name } uitgevee
packs-delete-failed = Kon nie uitvee nie: { $error }
packs-status-uploading = Laai op (lêers tot dusver ontvang)
packs-status-ingesting = Pak uit en etiketteer
packs-status-analyzing = LLM etiketteer
packs-status-ready = Gereed
packs-status-failed = Misluk
packs-retag-failed = Kon nie her-etikettering begin nie: { $error }
packs-read-files-failed = Kon nie die pak se lêers lees nie: { $error }
packs-context-load-failed = Kon nie die pak se winkelbesonderhede laai nie: { $error }
packs-undo-store-failed = Kon nie die winkelbladsy afhaal nie: { $error }
packs-context-save-failed = Kon nie winkelbesonderhede stoor nie: { $error }
packs-unreal-load-failed = Kon nie die Unreal-omskakelingstatus laai nie: { $error }
packs-unreal-discard-title = Gooi die oorspronklikes weg?
packs-unreal-discard-body = Die oorspronklike .uasset-lêers in { $name } verdwyn sodra hulle omgeskakel is. Data wat net Unreal gebruik, soos materiaalgrafieke, gaan vir altyd verlore.
packs-unreal-discard-confirm = Skakel om en gooi weg
packs-unreal-convert-failed = Kon nie die omskakeling begin nie: { $error }
packs-unreal-fact-converted = { $done } uit Unreal omgeskakel
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal-pakket
       *[other] { $left } Unreal-pakkette
    }
packs-unreal-fact-both = { $done } uit Unreal omgeskakel, { $left } steeds pakkette
packs-thumbs-canceled = Gekanselleer ná { $done } van { $total }
packs-thumbs-drew =
    { $count ->
        [one] { $count } model geteken
       *[other] { $count } modelle geteken
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } duimnael vervang.
       *[other] { $lead }: { $improved } duimnaels vervang.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } duimnael vervang, { $failed } kon nie geteken word nie.
       *[other] { $lead }: { $improved } duimnaels vervang, { $failed } kon nie geteken word nie.
    }
packs-archived = { $name } en sy { $count } bates geargiveer
packs-blobs-bad = { $count } lêers het hul laaste integriteitstoets gedruip.
packs-collections = Versamelings
packs-browse-named = Blaai deur { $name }
packs-collection-name = Versamelingnaam
packs-save = Stoor
packs-cancel = Kanselleer
packs-assets =
    { $count ->
        [one] { $count } bate
       *[other] { $count } bates
    }
packs-updated = Bygewerk { $date }
packs-browse = Blaai
packs-collection-download = Laai af (.zip)
packs-collection-credits = Erkennings
packs-collection-credits-title = Wie die pakke gemaak het waaruit hierdie bates kom, hul lisensies en die erkenningsreëls waarvoor hulle vra
packs-rename = Hernoem
packs-delete = Vee uit
packs-packs = Pakke
packs-empty = Nog geen pakke nie. Laat val 'n zip of 'n vouer enige plek op hierdie bladsy om een by te voeg.
packs-find = Vind 'n pak
packs-find-placeholder = Vind 'n pak volgens naam, winkel, lisensie of etiket
packs-sort = Volgorde
packs-sort-new = Nuutste eerste
packs-sort-old = Oudste eerste
packs-sort-name = Volgens naam
packs-sort-size = Grootste eerste
packs-sort-assets = Meeste bates eerste
packs-with-kind = Pakke met
packs-state = Status
packs-state-any = Enige status
packs-state-working = Word aan gewerk
packs-state-failed = Misluk
packs-state-untagged = Nog nie beskryf nie
packs-store = Winkel
packs-store-any = Enige winkel
packs-store-from = Van { $store }
packs-store-none = Geen winkelbladsy nie
packs-shown-of = { $shown } van { $total }
packs-clear = Maak skoon
packs-no-match = Geen pakke pas nie.
packs-clear-all = Maak die soektog en filters skoon
packs-pack-name = Paknaam
packs-thumbs-drawing = Teken duimnaels in hierdie blaaier…
packs-thumbs-drawing-count = Teken duimnaels in hierdie blaaier: { $done } van { $total }…
packs-thumbs-stopped = Gestop: { $error }
packs-thumbs-none = Elke model hier het reeds 'n duimnael wat met al sy teksture geteken is.
packs-dismiss = Wys weg
packs-license = Lisensie: { $license }
packs-license-tier = Lisensie: { $license } ({ $tier })
packs-license-not-found = nie gevind nie
packs-from-before = Van
packs-from-after = { "" }
packs-added = Bygevoeg { $date }
packs-download = Laai pak af (.zip)
packs-store-details = Winkelbesonderhede
packs-more-named = Meer aksies vir { $name }
packs-more = Meer
packs-rename-more = Hernoem…
packs-retag-rules = Laat vanlyn etiketteerder weer loop
packs-retag-llm = Laat LLM-etiketteerder weer loop
packs-make-thumbs = Maak duimnaels in hierdie blaaier
packs-make-thumbs-title = Maak elke model een vir een buite sig in 'n 3D-kyker oop, en gebruik sy prent as die duimnael waar dit beter is as die bediener s'n
packs-read-files = Lees die pak se lêers weer
packs-credits = Laai erkennings af
packs-credits-title = CREDITS.md: wie die pak gemaak het, sy lisensie en die erkenningsreël waarvoor dit vra
packs-unreal-open = Skakel Unreal-inhoud om…
packs-archive = Argiveer pak
packs-unreal-intro = Hoard lees Unreal-pakkette self en stoor wat daarin is as gewone lêers: teksture as PNG of EXR, klanke as WAV, materiale as JSON. Die omgeskakelde lêer neem die pakket se plek in, en behou sy etikette, skakels en versamelings.
packs-unreal-converted =
    { $count ->
       *[other] omgeskakel
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] omgeskakel, { $kept } met die oorspronklike behou
    }
packs-unreal-pending =
    { $count ->
       *[other] nog nie na gekyk nie
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] wag vir Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] sonder 'n algemene formaat
    }
packs-unreal-failed =
    { $count ->
       *[other] misluk
    }
packs-unreal-reasons = Waarom pakkette oorbly
packs-unreal-editor-before = Mase en animasies het Unreal Editor nodig. Op 'n masjien met Unreal geïnstalleer, voer uit
packs-unreal-editor-command = hoard unreal-worker --editor <pad na UnrealEditor-Cmd>
packs-unreal-editor-after = ; dit voer hulle as FBX uit en laai die resultate hier op.
packs-unreal-discard = Gooi die oorspronklike .uasset-lêers weg sodra hulle omgeskakel is
packs-unreal-discard-hint = Stel die spasie vry. Data wat net Unreal gebruik, soos materiaalgrafieke, gaan verlore.
packs-unreal-drop = Vee pakkette sonder 'n algemene formaat uit
packs-unreal-drop-hint = Blueprints, vlakke en ander bates wat net die enjin gebruik.
packs-unreal-convert = Skakel om
packs-context-hint-before = Waar die pak vandaan kom en wat die verkoper daaroor sê. Die LLM-etiketteerder lees dit saam met elke lêer, so 'n winkelbeskrywing soos "120 ys- en vuurspreuke" help dit om
packs-context-hint-after = { " " }te benoem.
packs-store-auto = Outomaties gevind: die pak noem self hierdie bladsy.
packs-store-not-this = Nie hierdie een nie
packs-lookup-label = Soek in die winkels na
packs-lookup-placeholder = Die pak se naam in die winkel
packs-lookup-searching = Soek tans…
packs-lookup-search = Soek winkels
packs-lookup-hide = Versteek
packs-lookup-clues = Uit die pak se lêers: { $clues }.
packs-lookup-also = Het ook gesoek na { $queries }.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Niks op Fab, itch.io, Kenney of OpenGameArt lyk daarna nie. Probeer minder of ander woorde.
packs-lookup-use = Gebruik
packs-lookup-searched = Gesoek { $date }.
packs-lookup-again = Soek weer
packs-lookup-again-after = .
packs-lookup-unreachable = Kon hierdie keer nie { $stores } bereik nie.
packs-lookup-also-look = Kyk ook op
packs-lookup-links-and = { " " }en{ " " }
packs-lookup-browser-only = (hulle kan net in jou blaaier deursoek word); plak die bladsy se adres hieronder.
packs-lookup-open = Vind in winkels…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Verkoper
packs-seller-placeholder = Wie dit gemaak het
packs-license-label = Lisensie
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Lisensievlak
packs-license-tier-placeholder = Persoonlik, Professioneel…
packs-store-page = Winkelbladsy
packs-store-text = Beskrywing van die winkel
packs-store-text-placeholder = Plak die lysteks, of laat leeg om dit van die winkelbladsy af te haal.
packs-store-text-fetch = Stoor haal die beskrywing van die winkelbladsy af.
packs-readme = README in die pak gevind
packs-save-retag = Stoor en laat LLM-etiketteerder weer loop

## AssetDetail.vue

asset-lazy-3d-viewer = die 3D-kyker
asset-lazy-text = die teks
asset-lazy-text-viewer = die tekskyker
asset-lazy-image-viewer = die prentkyker
asset-lazy-material-preview = die materiaalvoorskou
asset-lazy-material-viewer = die materiaalkyker
asset-lazy-sprite-player = die sprite-speler
asset-rel-texture = tekstuur
asset-rel-buffer = geometriedata
asset-rel-material = materiaallêer
asset-rel-animation = animasie
asset-rel-library = gekoppelde lêer
asset-rel-guess = hoort waarskynlik by
asset-rel-sidecar = winkelbesonderhede
asset-rel-variant = variant
asset-rel-sprite = sprite
asset-role-map = kaart vir { $role }
asset-maps-count =
    { $count ->
        [one] { $count } kaart
       *[other] { $count } kaarte
    }
asset-needs = Benodig
asset-animations = Animasies
asset-sprites = Sprites
asset-in-atlas = In atlas
asset-used-by = Gebruik deur
asset-conversion-failed = omskakeling het misluk
asset-as-converting = Skakel om…
asset-as-converting-percent = Skakel om… { $percent }%
asset-as-queued = In die tou…
asset-download = Laai af
asset-as-label = Laai af as
asset-as-original = { $format } (oorspronklike formaat)
asset-as-size = Grootte
asset-as-full-size = Volle grootte
asset-as-up-to = Tot { $size } px
asset-as-hint = Kies 'n grootte of 'n ander formaat
asset-meta-size = Grootte
asset-meta-length = Lengte
asset-meta-sample-rate = Monstertempo
asset-meta-channels = Kanale
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bisdiepte
asset-meta-bits = { $bits }-bis
asset-meta-level = Vlak
asset-meta-level-value = piek { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = piek { $peak } dBFS, RMS { $rms } dBFS, knip
asset-meta-loop = Lus
asset-meta-loop-value = { $start } tot { $end }
asset-meta-playback = Afspeel
asset-meta-one-shot = Eenmalig
asset-meta-tempo = Tempo
asset-meta-root-note = Grondnoot
asset-meta-cue-markers = Merkers
asset-meta-library = Biblioteek
asset-meta-keywords = Sleutelwoorde
asset-meta-title = Titel
asset-meta-artist = Kunstenaar
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Generator
asset-meta-everything-with = Wys alles met die etiket { $tag }
asset-meta-saved-with = Gestoor met
asset-meta-color-mode = Kleurmodus
asset-meta-layers = Lae
asset-meta-range = Omvang
asset-meta-hdr = HDR (voorskou is toonverwerk)
asset-meta-format = Formaat
asset-meta-large-document = Groot dokument (PSB)
asset-meta-ai = KI
asset-meta-ai-made = Met generatiewe KI gemaak
asset-meta-ai-edited = Met generatiewe KI gewysig
asset-meta-ai-how-metadata = metadata
asset-meta-ai-how-generator = generatorinstellings
asset-meta-ai-how-iptc = IPTC-digitalebrontipe
asset-meta-ai-says = { $made }, volgens sy { $how }
asset-meta-ai-by-says = { $made } ({ $by }), volgens sy { $how }
asset-meta-provenance = Herkoms
asset-meta-c2pa = Het C2PA Content Credentials
asset-meta-compression = Kompressie
asset-meta-note = Nota
asset-meta-maps = Kaarte
asset-meta-sizes =
    { $count ->
        [one] Grootte
       *[other] Groottes
    }
asset-meta-triangles = Driehoeke
asset-meta-vertices = Hoekpunte
asset-meta-meshes = Mase
asset-meta-materials = Materiale
asset-meta-animations = Animasies
asset-meta-skins = Velle
asset-meta-unreal-asset = Unreal-bate
asset-meta-unreal-saved-with = { $class }, gestoor met UE { $engine }
asset-meta-package = Pakket
asset-meta-cooked = Gekook (speelbou): voorskoue en bronne is verwyder
asset-meta-tiling = Teëling
asset-meta-seamless = Naatloos op albei asse
asset-meta-analysis-error = Kon nie hierdie lêer heeltemal lees nie: { $error }
asset-copy-blocked = Kon nie kopieer nie: die blaaier het dit geblokkeer
asset-save-error = Kon nie stoor nie: { $error }
asset-tag-removed = { $tag } verwyder
asset-undo = Ontdoen
asset-play-error = Kon nie hierdie lêer speel nie: { $error }
asset-copied-hash = Hash gekopieer
asset-copied-toml = toml gekopieer
asset-copied-id = ID gekopieer
asset-origin-before = Hierdie lêer was
asset-origin-class-engine = , 'n { $class } wat deur UE { $engine } gestoor is.
asset-origin-class = , 'n { $class }.
asset-origin-engine = { " " }en is deur UE { $engine } gestoor.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor het dit op { $date } uitgevoer.
asset-origin-editor-notes = Unreal Editor het dit op { $date } uitgevoer ({ $notes }).
asset-origin-hoard = Hoard het dit op { $date } uit die pakket gelees.
asset-origin-hoard-notes = Hoard het dit op { $date } uit die pakket gelees ({ $notes }).
asset-origin-download = Laai die oorspronklike .{ $ext } af
asset-origin-discarded = Die oorspronklike pakket is ná omskakeling weggegooi.
asset-details = Batebesonderhede
asset-in-pack = in
asset-previous = Vorige bate
asset-next = Volgende bate
asset-favorite = Gunsteling
asset-favorite-on-title = In jou gunstelinge (F haal dit uit)
asset-favorite-off-title = Voeg by jou gunstelinge (F)
asset-layout-panel = Wys in 'n sypaneel
asset-layout-page = Wys volle bladsy
asset-layout-page-title = Wys volle bladsy: 'n groot voorskou langs die besonderhede
asset-close = Sluit besonderhede
asset-rename-title = Klik om te hernoem
asset-name = Naam
asset-views-alt = Die model uit vier rigtings
asset-inspect-aria = Inspekteer { $name } op volle skerm
asset-inspect-title = Inspekteer: zoem, teël, kanale
asset-playback-position = Afspeelposisie
asset-playback-value = { $current } van { $duration }
asset-pause = Pouseer
asset-play = Speel
asset-waveform-drawing = Teken golfvorm…
asset-waveform-skipped = Geen golfvorm vir lêers wat so groot is nie
asset-play-animation = Speel as animasie
asset-show-views = Wys die vier-aansig-blad wat die etiketteerder gesien het
asset-palette = Kleure in die prent
asset-swatch-title = { $hex }, { $percent }% van die prent. Vind bates met hierdie kleur
asset-swatch-aria = Vind bates met { $hex }, { $percent }% van hierdie prent
asset-more-like-this = Meer soos hierdie
asset-desc-label = Jou beskrywing
asset-desc-placeholder = Wat dit is, waar jy dit sou gebruik…
asset-save = Stoor
asset-cancel = Kanselleer
asset-desc-family = Stoor na al { $count } variante
asset-desc-edit = Wysig beskrywing
asset-desc-write = Skryf 'n beskrywing
asset-desc-ai = Deur KI geskryf
asset-desc-file = Uit die lêer
asset-desc-store-before = Van die
asset-desc-store-page = { $store }-bladsy
asset-desc-store-page-unnamed = winkelbladsy
asset-desc-store-after = (die hele pak)
asset-less = Minder
asset-more = Meer
asset-tags = Etikette
asset-tag-add-label = Voeg 'n etiket by
asset-tag-add-placeholder = Voeg 'n etiket by, bv. mood:tense
asset-tag-add = Voeg by
asset-apply-family = Pas etiket-, kategorie- en beskrywingveranderings op al { $count } variante toe
asset-collections = Versamelings
asset-collection-remove = Verwyder uit { $name }
asset-collection-add-another = Voeg by nog een
asset-collection-add = Voeg by versameling
asset-used-in = Gebruik in
asset-used-in-title = Wys wat { $name } het
asset-used-in-synced = { $when } gesinkroniseer
asset-maps = Kaarte
asset-show-more = Wys nog { $count }
asset-materials-heading =
    { $count ->
        [one] Materiaal
       *[other] Materiale
    }
asset-material-maps-aria = { $name }: kaarte
asset-variants = Variante ({ $count } weergawes)
asset-copies = Identiese kopieë
asset-copies-note = Dieselfde grepe word een keer gestoor; hierdie pakke bevat ook hierdie lêer.
asset-unreal-converted = Uit Unreal omgeskakel
asset-unreal-package = Uit die Unreal-pakket
asset-ue-needs-editor-before = Wag vir Unreal Editor: { $reason }. Op 'n masjien met Unreal geïnstalleer, voer
asset-ue-needs-editor-after = uit, en dit kom as FBX terug.
asset-ue-editor-path = pad na UnrealEditor-Cmd
asset-ue-unsupported = As 'n pakket gehou: { $reason }.
asset-ue-failed = Omskakeling het misluk: { $reason }
asset-recovered = Die oorspronklike .{ $ext } is binne hierdie pakket gestoor, en Hoard het dit uitgehaal.
asset-recovered-download = Laai .{ $ext } af
asset-ue-meshes-before = Mase en animasies word in Unreal se eie formaat gestoor. Skakel die pak om (pakbladsy, Meer), en voer dan
asset-ue-meshes-after = uit langs 'n Unreal Editor-installasie om FBX te kry.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } bateregister-etiket
       *[other] { $count } bateregister-etikette
    }
asset-file = Lêer
asset-bytes = Grepe
asset-source = Bron
asset-path = Pad
asset-download-maps =
    { $count ->
        [one] Laai die { $count } kaart af
       *[other] Laai al { $count } kaarte af
    }
asset-download-linked =
    { $count ->
        [one] Laai af met { $count } gekoppelde lêer
       *[other] Laai af met { $count } gekoppelde lêers
    }
asset-download-only-title = Net hierdie lêer, sonder die lêers wat dit gebruik
asset-download-only = Net hierdie lêer
asset-include-animations = Sluit animasies by die .zip in
asset-copy-toml = Kopieer hoard.toml-inskrywing
asset-copy-id = Kopieer ID
asset-teach = Leer Hoard hierdie lêer
asset-teach-title = Beskryf hierdie winkel se lêer sodat Hoard dit lees (Instellings → Winkellêers)
asset-archive = Argiveer
asset-archive-title = Uit soektogte totdat dit herstel of vir altyd uitgevee word, vanuit die Argief

## ImageViewer.vue

image-channel-red = Rooi kanaal
image-channel-green = Groen kanaal
image-channel-blue = Blou kanaal
image-channel-alpha = Alfakanaal
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Skaakbordagtergrond (K wissel)
image-bg-black-title = Swart agtergrond (K wissel)
image-bg-gray-title = Grys agtergrond (K wissel)
image-bg-white-title = Wit agtergrond (K wissel)
image-bg-checker = skaakbordagtergrond
image-bg-black = swart agtergrond
image-bg-gray = grys agtergrond
image-bg-white = wit agtergrond
image-inspect-aria = Inspekteer { $name }
image-preview-size = voorskou { $width }×{ $height }, { $bytes }
image-preview = voorskou
image-resolution = Resolusie
image-full-title = Laai die volle { $width }×{ $height }-prent (F); om verby die voorskou in te zoem doen dit ook
image-full-loading = Laai volle resolusie…
image-full-failed = Volle resolusie het nie gelaai nie: probeer weer
image-full = Volle resolusie
image-zoom = Zoem
image-zoom-out-title = Zoem uit (−)
image-zoom-out = Zoem uit
image-zoom-in-title = Zoem in (+)
image-zoom-in = Zoem in
image-fit-title = Pas (0)
image-fit = Pas
image-actual-title = Werklike pixels (1)
image-view = Aansig
image-tile-title = Teël 3×3 om die nate na te gaan (T)
image-tile = Teël
image-channels = Kanale
image-close = Sluit (Esc)
image-failed = Hierdie prent kon nie gelaai word nie.
image-loading = Laai…
image-key-gestures = Rol of knyp om te zoem · sleep om te skuif · dubbelklik om in te zoem of te pas
image-key-tile = teël
image-key-background = agtergrond
image-key-full = volle resolusie
image-key-channels = kanale

## SpritePlayer.vue

sprite-frame-aria = { $name }, raam { $frame } van { $count }
sprite-failed = Hierdie een kan nie gespeel word nie.
sprite-loading = Laai rame…
sprite-previous = Vorige raam
sprite-next = Volgende raam
sprite-pause = Pouseer animasie
sprite-play = Speel animasie
sprite-animation = Animasie
sprite-all-frames = Alle rame
sprite-grid-title = Hoe die blad in rame gesny word
sprite-columns = Kolomme
sprite-rows = Rye
sprite-fps-aria = Rame per sekonde
sprite-fps = fps

## TextViewer.vue, TextPreview.vue en text.ts

text-lang-config = Konfigurasie
text-lang-table = Tabel
text-lang-code = Kode
text-lang-script = Skrip
text-lang-text = Teks
text-copy-blocked = Kon nie kopieer nie: die blaaier het dit geblokkeer
text-read-aria = Lees { $name }
text-lines =
    { $count ->
        [one] { $count } reël
       *[other] { $count } reëls
    }
text-show-as = Wys as
text-tree-title = Opvoubare boom (T)
text-tree = Boom
text-pretty-title = Herinspring (P)
text-pretty = Netjies
text-raw-title = Soos geskryf (R)
text-raw = Rou
text-document-title = As 'n dokument (D)
text-document = Dokument
text-source-title = Die markdown soos geskryf (R)
text-source = Bron
text-fold = Vou
text-expand-all-title = Vou alles oop
text-expand-all = Vou alles oop
text-collapse-all-title = Vou alles toe
text-collapse-all = Vou alles toe
text-group-text = Teks
text-wrap-title = Vou lang reëls om (W)
text-wrap = Omvou
text-copy-title = Kopieer die teks (C)
text-copied = Gekopieer
text-copy = Kopieer
text-download-title = Laai die lêer af
text-download = Laai af
text-close = Sluit (Esc)
text-truncated = Wys die eerste { $shown } van { $total }.
text-truncated-json = Wys die eerste { $shown } van { $total } (die boom het die hele lêer nodig).
text-show-all = Wys alles
text-json-error = Hierdie JSON kan nie ontleed word nie: { $error }
text-contents-aria = Inhoud van { $name }
text-loading = Laai…
text-load-error = Kon nie hierdie lêer laai nie: { $error }
text-binary = Hierdie lêer is binêr, nie teks nie, dus is daar niks om hier te lees nie. Laai dit af om dit in sy eie program oop te maak.
text-key-tree = boom
text-key-pretty = netjies
text-key-raw = rou
text-key-document = dokument
text-key-source = bron
text-key-wrap = omvou
text-key-copy = kopieer
text-key-find = vind
text-key-close = sluit
text-preview-aria = Lees { $name } op volle skerm
text-preview-read = Lees

## JsonTree.vue en JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
json-keys =
    { $count ->
        [one] { $count } sleutel
       *[other] { $count } sleutels
    }
json-fold = Vou toe
json-unfold = Vou oop
json-unfold-summary = Vou { $summary } oop
json-show-more = Wys nog { $count }
json-left = van { $count } oor
json-picker-more = … nog { $count }

## 3D-modelkykers (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = die 3D-kyker het nie gelaai nie ({ $error }); om die bladsy te herlaai kan dalk help
model-loading-viewer = Laai die 3D-kyker…
model-loading = Laai model…
model-show-failed = Kon nie hierdie model wys nie: { $error }
model-preview-failed = Kon nie 'n voorskou van hierdie model wys nie: { $error }
model-error-converting = die bediener skakel hierdie model nog om
model-error-empty-collada = leë Collada-lêer
model-error-no-loader = Geen blaaierlaaier vir .{ $ext } nie
model-dialog = Bekyk { $name } in 3D
model-title-stats =
    { $triangles } driehoeke · { $meshes ->
        [one] { $meshes } maas
       *[other] { $meshes } mase
    } · { $materials ->
        [one] { $materials } materiaal
       *[other] { $materials } materiale
    }
model-triangles =
    { $count ->
        [one] { $count } driehoek
       *[other] { $count } driehoeke
    }
model-group-camera = Kamera
model-group-shading = Skakering
model-group-background = Agtergrond
model-group-animation = Animasie
model-group-view = Aansig
model-group-show = Wys
model-frame-title = Raam die model in (F)
model-frame = Raam in
model-reset-title = Terug na die beginaansig (R)
model-reset = Terugstel
model-turn-title = Draai stadig (A)
model-turn = Draai
model-textures-title = Teksture (T)
model-textures = Teksture
model-full-res-title = Volresolusie-teksture (H): elke tekstuurlêer soos dit is, hoe groot ook al; af: ligte voorskoue van hoogstens 2048 px
model-full-res = Volres
model-light-title = Omgewingslig (E)
model-light = Lig
model-grid-title = Vloerrooster (G)
model-grid = Rooster
model-bg-checker = Skaakbordagtergrond
model-bg-dark = Donker agtergrond
model-bg-gray = Grys agtergrond
model-bg-light = Ligte agtergrond
model-bg-env = Omgewingsagtergrond
model-bg-title = { $background } (K wissel)
model-pause-title = Pouseer (Spasie)
model-play-title = Speel (Spasie)
model-pause = Pouseer
model-play = Speel
model-animation-clip = Animasiegreep
model-animation = Animasie
model-close-title = Sluit (Esc)
model-full-screen = Volle skerm
model-open-full-screen = Maak die volskerm-3D-kyker oop
model-menu-fold = Vou die kieslys weg
model-menu-show = Wys die kieslys
model-view-prefix = Aansig:{ " " }
model-menu-render = Weergawe
model-menu-maps = Kaarte
model-menu-inspect = Inspekteer
model-mode-final = Finale weergawe
model-mode-raw = Rou weergawe
model-mode-color = Basiskleur
model-mode-roughness = Grofheid
model-mode-metalness = Metaalagtigheid
model-mode-normal = Normaal
model-mode-ao = Okklusie
model-mode-matcap = Matcap
model-mode-uv = UV-toetser
model-mode-geometry = Geometrieë
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Draadraam (W), oor enige aansig
model-wireframe = Draadraam
model-info-title = Toneelinligting (I)
model-info = Toneelinligting
model-info-close = Sluit toneelinligting
model-info-close-title = Sluit (I)
model-info-geometries = Geometrieë
model-info-materials = Materiale
model-info-textures = Teksture
model-info-triangles = Driehoeke
model-info-vertices = Hoekpunte
model-info-meters = glTF is in meter
model-info-no-units = Hierdie formaat sê nie wat sy eenhede is nie
model-info-height = Hoogte
model-info-length = Lengte
model-info-width = Breedte
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } eenheid
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } eenhede
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Tekstuur nie gevind nie: { $name }
model-missing-some = { $count } teksture nie gevind nie: { $names }
model-missing-more = { $count } teksture nie gevind nie: { $names }…
model-missing-count = { $count } teksture nie gevind nie
model-guessed = Teksture volgens naam gepas; die lêer noem geen nie.
model-keys-mouse = Sleep om te wentel · regs-sleep om te skuif · rol om te zoem · dubbelklik om in te raam
model-key-frame = raam in
model-key-reset = terugstel
model-key-view = aansig
model-key-wireframe = draadraam
model-key-info = toneelinligting
model-key-textures = teksture
model-key-full-res = volres-teksture
model-key-light = lig
model-key-grid = rooster
model-key-background = agtergrond
model-key-play = speel

## Materiale (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Kleur
material-role-normal = Normaal
material-role-normal-dx = Normaal (DirectX)
material-role-roughness = Grofheid
material-role-gloss = Glans
material-role-metalness = Metaalagtigheid
material-role-ao = Omgewingsokklusie
material-role-orm = Okklusie, grofheid, metaal
material-role-mra = Metaal, grofheid, okklusie
material-role-rma = Grofheid, metaal, okklusie
material-role-height = Hoogte
material-role-bump = Bult
material-role-cavity = Holte
material-role-specular = Spekulêr
material-role-opacity = Ondeursigtigheid
material-role-emissive = Emissie
material-role-translucency = Deurskynendheid
material-role-base-color-name = kleur
material-role-normal-name = normaal
material-role-normal-dx-name = normaal (DirectX)
material-role-roughness-name = grofheid
material-role-gloss-name = glans
material-role-metalness-name = metaalagtigheid
material-role-ao-name = omgewingsokklusie
material-role-orm-name = okklusie, grofheid, metaal
material-role-mra-name = metaal, grofheid, okklusie
material-role-rma-name = grofheid, metaal, okklusie
material-role-height-name = hoogte
material-role-bump-name = bult
material-role-cavity-name = holte
material-role-specular-name = spekulêr
material-role-opacity-name = ondeursigtigheid
material-role-emissive-name = emissie
material-role-translucency-name = deurskynendheid
material-maps =
    { $count ->
        [one] { $count } kaart
       *[other] { $count } kaarte
    }
material-dialog = Bekyk die materiaal { $name }
material-up-to = tot { $size }
material-repeat-size = { $size } per herhaling
material-shape-sphere = Sfeer
material-shape-cube = Kubus
material-shape-plane = Vlak
material-shape-title = { $shape } ({ $key })
material-group-shape = Vorm
material-group-tiling = Teëling
material-real-title = Werklike grootte: een herhaling is { $size }, die sfeer 2 m in deursnee (T wissel)
material-repeat-title = Herhaal { $n }× (T wissel)
material-real = Werklik
material-repeat = { $n }×
material-height-title = Hoe ver die hoogtekaart die oppervlak uitdruk
material-height = Hoogte
material-height-strength = Hoogtesterkte
material-loading = Laai materiaal…
material-show-failed = Kon nie hierdie materiaal wys nie: { $error }
material-preview-failed = Kon nie 'n voorskou van hierdie materiaal wys nie: { $error }
material-group-maps = Kaarte
material-map-show = Wys die kaart vir { $map }
material-map-hide = Versteek die kaart vir { $map }
material-map-unused = Nie deur die voorskou gebruik nie
material-open-full-screen = Maak die volskerm-materiaalkyker oop
material-keys-mouse = Sleep om te wentel · regs-sleep om te skuif · rol om te zoem · klik op 'n kaart om dit af te skakel
material-key-shape = vorm
material-key-tiling = teëling
material-key-turn = draai

## theme.ts

theme-slate = Leiklip
theme-paper = Papier
theme-linen = Linne
theme-sage = Salie
theme-lilac = Sering
theme-blush = Bloos
theme-graphite = Grafiet
theme-obsidian = Obsidiaan
theme-ember = Gloed
theme-moss = Mos
theme-aubergine = Eiervrug
theme-oxblood = Osbloed
theme-light-slate-blurb = Koel blougrys
theme-light-paper-blurb = Eenvoudige neutrale grys
theme-light-linen-blurb = Warm room, amber
theme-light-sage-blurb = Sagte groen
theme-light-lilac-blurb = Ligte violet
theme-light-blush-blurb = Dowwe roos
theme-dark-graphite-blurb = Neutrale grys, geen tint nie
theme-dark-slate-blurb = Blougrys
theme-dark-obsidian-blurb = Amper swart, vir OLED
theme-dark-ember-blurb = Warm houtskool, amber
theme-dark-moss-blurb = Woudgroen-swart
theme-dark-aubergine-blurb = Pruimswart, sering
theme-dark-oxblood-blurb = Wynswart, koraal

## texnames.ts

texnames-needs-map = Die patroon het { "{" }map{ "}" } nodig, anders kry elke kaart van 'n materiaal dieselfde naam
texnames-placeholders = Die patroon kan { "{" }name{ "}" }, { "{" }map{ "}" } en { "{" }size{ "}" } gebruik
texnames-bad-characters = Name kan nie < > : " / \ | ? of * bevat nie
texnames-no-suffix = Gee kaarte vir { $role } 'n agtervoegsel
texnames-bad-suffix = Die agtervoegsel van kaarte vir { $role } kan nie < > : " / \ | ? of * bevat nie

## storeFormats.ts

storefiles-field-title = Titel
storefiles-field-title-hint = Die lysinskrywing se presiese titel. Dit benoem die pak, en sonder dit word die lêer oorgeslaan.
storefiles-field-seller = Verkoper
storefiles-field-seller-hint = Wie dit gemaak het of verkoop.
storefiles-field-license = Lisensie
storefiles-field-license-hint = Soos die lêer dit skryf, bv. CC-BY-4.0.
storefiles-field-url = Winkelbladsy
storefiles-field-url-hint = Die lysinskrywing se adres, wat met http:// of https:// begin.
storefiles-field-words = Etikette
storefiles-field-words-hint = Woorde om dit mee te etiketteer. Kommageskeide waardes word verdeel.
storefiles-field-type = Soort
storefiles-field-type-hint = Wat dit is, soos “3D model” of “surface”. Ook 'n etiket.
storefiles-field-colors = Kleure
storefiles-field-colors-hint = Sy kleure, as color:-etikette.
storefiles-field-id = Winkel-ID
storefiles-field-id-hint = Die winkel se ID daarvoor: letters, syfers, _ of -.
storefiles-field-origin = Herkoms
storefiles-field-origin-hint = Waar dit gemaak of geskandeer is, vir die beskrywing.
storefiles-field-size = Grootte
storefiles-field-size-hint = Hoe groot dit is, in woorde, vir die beskrywing.
storefiles-field-title-name = titel
storefiles-field-seller-name = verkoper
storefiles-field-license-name = lisensie
storefiles-field-url-name = winkelbladsy
storefiles-field-words-name = etikette
storefiles-field-type-name = soort
storefiles-field-colors-name = kleure
storefiles-field-id-name = winkel-ID
storefiles-field-origin-name = herkoms
storefiles-field-size-name = grootte

## StoreFormatsPanel.vue

storefiles-load-failed = Kon nie die winkellêerformate laai nie: { $error }
storefiles-remove-title = Verwyder { $name }?
storefiles-remove-body = { $file } word uit die datavouer uitgevee. Pakke wat reeds daarmee gelees is, behou wat dit gelees het.
storefiles-remove = Verwyder
storefiles-removed = { $name } verwyder.
storefiles-remove-failed = Kon nie { $name } verwyder nie: { $error }
storefiles-unread = Lêers wat Hoard nie lees nie
storefiles-loading = Laai…
storefiles-unread-none = Geen: geen lêer in die biblioteek lyk soos een van 'n ongeleesde winkel nie.
storefiles-unread-what = Lêers in meer as een pak, of benoem soos 'n winkel s'n, wat geen formaat lees nie.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pak
       *[other] { $count } pakke
    }
storefiles-unread-example = bv. { $pack }
storefiles-teach = Leer Hoard hierdie lêer
storefiles-formats = Formate
storefiles-formats-what = Joune word eerste probeer en neem die plek in van 'n ingeboude een met dieselfde naam.
storefiles-dir-before = Hulle word gehou in
storefiles-dir-after = .
storefiles-yours = Joune
storefiles-built-in = Ingebou
storefiles-edit = Wysig
storefiles-start-from = Begin hiervan

## TagsPanel.vue

tags-load-failed = Kon nie die etikette laai nie: { $error }
tags-rename-title = Hernoem { $tag }?
tags-merge-title = Voeg { $count } etikette saam?
tags-rename-body = Sy { $count } bates kry eerder { $into }, en die etiketteerders gebruik die nuwe naam van nou af.
tags-merge-body = { $tags } word { $into } op tot { $count } bates. Die etiketteerders gebruik { $into } van nou af.
tags-rename = Hernoem
tags-merge = Voeg saam
tags-merged =
    { $count ->
        [one] { $count } bate nou met { $into } geëtiketteer
       *[other] { $count } bates nou met { $into } geëtiketteer
    }
tags-merge-failed = Kon nie saamvoeg nie: { $error }
tags-forget-failed = Kon dit nie verander nie: { $error }
tags-find = Vind etikette
tags-find-placeholder = Vind etikette, bv. rock of set:
tags-count =
    { $count ->
        [one] { $count } etiket
       *[other] { $count } etikette
    }
tags-count-more = { $count }+ etikette
tags-what = Kies etikette wat dieselfde beteken en voeg hulle saam, of kies een om dit te hernoem.
tags-new-name = Nuwe naam
tags-merge-into = Voeg saam in
tags-saved-as-before = Gestoor as
tags-saved-as-after = { "" }
tags-cancel = Kanselleer
tags-loading = Laai…
tags-none = Geen etikette nie.
tags-none-containing = Geen etikette wat “{ $text }” bevat nie.
tags-browse-title = Blaai deur bates met die etiket { $tag }
tags-browse = Blaai
tags-merged-heading = Saamgevoegde etikette
tags-merged-what = Wanneer die etiketteerders met een van hierdie ou name vorendag kom, kry die bate die etiket waarin dit saamgevoeg is.
tags-allow-old = Laat die ou naam weer toe

## LibraryStats.vue

stats-source-human = Deur mense bygevoeg
stats-source-ai = Deur KI voorgestel
stats-source-rule = Ingeboude reëls
stats-source-meta = Uit die lêers
stats-assets = Bates
stats-packs = Pakke
stats-on-disk = Op skyf
stats-duplicates-title = { $count } lêers is greep-identiese kopieë van ander en word een keer gestoor
stats-saved-dedup = Bespaar deur ontdubbeling
stats-distinct-tags = Verskillende etikette
stats-archive = In die argief
stats-load-failed = Kon nie die besonderhede laai nie: { $error }
stats-counting = Tel…
stats-by-kind = Spasie volgens soort
stats-files-size = { $name }: { $count } lêers, { $size }
stats-months = Bygevoeg in die afgelope jaar
stats-month = { $month }: { $count } bates, { $size }
stats-tagging = Etikettering
stats-tagged = Geëtiketteer
stats-untagged = { $count } sonder etikette
stats-described = Beskryf
stats-described-assets = { $count } bates
stats-human-edited = Met die hand gewysig
stats-human-edited-unit =
    { $count ->
       *[other] bates
    }
stats-source-different = { $count } verskillende
stats-most-used = Meeste gebruik
stats-show-tag = Wys die { $tag }-bates
stats-file-types = Lêertipes
stats-biggest-packs = Grootste pakke
stats-browse-pack = Blaai deur { $name }
stats-largest-files = Grootste lêers
stats-open-file = Maak { $name } oop
stats-empty = Nog niks in die biblioteek nie. Voeg 'n pak by en sy syfers verskyn hier.

## UpdatesPanel.vue

updates-saved = Hoard sal dit roep wanneer 'n bywerking gereed is
updates-saved-none = Geen adres om te roep nie
updates-save-failed = Kon nie stoor nie
updates-call-answered = Dit het { $result } geantwoord
updates-call-refused = Dit het nie die versoek aanvaar nie: { $result }
updates-call-failed = Kon dit nie roep nie
updates-called = { $when } geroep vir { $version }: { $result }
updates-called-failed = { $when } geroep vir { $version }: { $result }. Hoard probeer oor 'n uur weer.
updates-latest = Hoard { $version } is die nuutste weergawe.
updates-out = Hoard { $version } is uit.
updates-out-idle = Hoard { $version } is uit; roep binne 'n paar minute, terwyl niks aan die gang is nie.
updates-out-busy = Hoard { $version } is uit; wag totdat niks aan die gang is nie ({ $work }).
updates-busy-more = nog { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } taak wag
       *[other] { $count } take wag
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } oplaai
       *[other] { $count } oplaaie
    }
updates-list-separator = ,{ " " }
updates-what = Wanneer 'n nuwe weergawe uit is, kan Hoard 'n adres roep sodra niks aan die gang is nie (geen oplaaie, take, omskakelings of instandhouding nie), sodat wat ook al sy houer bywerk dit nie halfpad deur 'n oplaai herbegin nie. Dit stuur 'n POST met die weergawe in 'n JSON-liggaam. Dit het die weergawekontrole hierbo nodig.
updates-url = Adres om te roep
updates-url-placeholder = Geen (bv. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Gestoor (tik om te vervang)
updates-auth-placeholder = Geen (bv. Bearer my-token)
updates-remove = Verwyder
updates-save = Stoor
updates-call-now-title = Wat ook al daaraan gekoppel is, loop
updates-call-now = Roep dit nou
updates-what-calls = Wat kan dit roep?
updates-watchtower = Watchtower se HTTP-API
updates-watchtower-at = :{ " " }
updates-watchtower-with = , met{ " " }
updates-watchtower-token = { " " }en die token uit{ " " }
updates-watchtower-its = { " " }(met sy{ " " }
updates-watchtower-endpoint = { " " }eindpunt aangeskakel).
updates-portainer = 'n Portainer-webhook
updates-portainer-rest = { " " }op Hoard se stack of houer, met re-pull image aan. Die token is in die URL.
updates-script = 'n Skrip op die gasheer
updates-script-listener = { " " }agter 'n webhook-luisteraar (soos{ " " }
updates-script-runs = ) wat{ " " }
updates-script-end = { " " }uitvoer.
updates-others = Home Assistant, n8n
updates-others-rest = { " " }of enigiets anders wat 'n webhook aanvaar.
updates-poll-before = Of slaan die adres oor: 'n skrip met 'n toegangstoken kan herhaaldelik{ " " }
updates-poll-until = { " " }opvra totdat{ " " }
updates-poll-and = { " " }gestel is en{ " " }
updates-poll-after = { " " }waar is, en dan Hoard trek en herbegin.

## MaintenancePanel.vue

maintenance-load-failed = Kon nie Instandhouding laai nie
maintenance-reanalyze-title = Heranaliseer elke pak?
maintenance-reanalyze-body = Elke lêer word weer gelees, wat lank kan neem op 'n groot biblioteek; pakke wys as besig totdat dit hul beurt is. Jou eie name, etikette en notas bly.
maintenance-reanalyze-confirm = Heranaliseer
maintenance-gc-title = Vee ongebruikte lêers uit?
maintenance-gc-body = Gestoorde lêers waarna niks in die biblioteek verwys nie, word vir altyd uitgevee. Pakke in die argief behou hulle s'n.
maintenance-gc-confirm = Vee hulle uit
maintenance-start-failed = Kon nie “{ $task }” begin nie
maintenance-just-now = so pas
maintenance-minutes-ago = { $count } min gelede
maintenance-hours-ago = { $count } h gelede
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Laai…
maintenance-running = Loop…
maintenance-starting = Begin…
maintenance-run = Laat loop
maintenance-started = { $detail } · { $when } begin
maintenance-starting-detail = Begin
maintenance-finished = Voltooi:{ " " }
maintenance-failed = Misluk:{ " " }
maintenance-never-run = Nog nooit geloop nie
maintenance-history = Vorige lopies
maintenance-col-task = Taak
maintenance-col-finished = Voltooi
maintenance-col-took = Duur
maintenance-col-result = Resultaat
maintenance-script-before = Rugsteune en die skadetoets kan ook vanuit 'n skrip geloop word:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }en{ " " }
maintenance-script-after = . Daardie lopies verskyn ook hier.

## SettingsPage.vue

settings-title = Instellings
settings-loading = Laai…
settings-load-failed = Kon nie die instellings laai nie: { $error }
settings-page-general = Algemeen
settings-page-general-blurb = Wat hierdie bediener genoem word, vir almal wat dit gebruik.
settings-page-appearance = Voorkoms
settings-page-appearance-blurb = Die temas wat hierdie blaaier dra wanneer dit lig en wanneer dit donker is. Net in hierdie blaaier gestoor.
settings-page-library = Biblioteek
settings-page-library-blurb = Wat in die hoard is: hoe groot dit is, waarvan dit gemaak is, hoe dit gegroei het en hoe goed dit geëtiketteer is. Geargiveerde dinge word nie getel nie.
settings-page-tags = Etikette
settings-page-tags-blurb = Elke etiket in die biblioteek. Voeg dié saam wat dieselfde beteken, of hernoem een; die etiketteerders gebruik van dan af die nuwe naam.
settings-page-store-files = Winkellêers
settings-page-store-files-blurb = Die lêers wat winkels langs hul kuns sit, wat Hoard lees om pakke te benoem en hul makers erkenning te gee. Leer dit dié wat dit nie ken nie, en deel wat jy dit geleer het.
settings-page-locations = Biblioteekliggings
settings-page-locations-blurb = Die vouers op die bediener wat die biblioteek bevat: dié waarin Hoard lêers hou, en vouers met pakke wat dit indekseer waar hulle is.
settings-page-tools = Gereedskap
settings-page-tools-blurb = Eksterne programme wat Hoard gebruik om lêers om te skakel en te lees.
settings-page-ai = KI-etikettering
settings-page-ai-blurb = 'n KI-model benoem, beskryf en etiketteer nuwe pakke, bo en behalwe die ingeboude reëls. Sy etikette word as voorgestel gemerk.
settings-page-search = Beeldsoektog
settings-page-search-blurb = Soek prente volgens wat hulle wys, nie net volgens hul name nie, en vind dinge wat in soort eenders is, nie net in kleur nie.
settings-page-adding = Pakke byvoeg
settings-page-adding-blurb = Wat Hoard met 'n pak doen wanneer dit bygevoeg word, behalwe om dit te lees en te etiketteer.
settings-page-downloads = Aflaaie
settings-page-downloads-blurb = Hoe lêers benoem word in wat Hoard stuur: aflaaie, zips, die hoard-hulpmiddel en KI-assistente. Lêers in die biblioteek bly soos hulle is.
settings-page-account = Rekening
settings-page-account-blurb = Jou wagwoord, die ander mense wat kan aanmeld, en toegangstokens vir skrips en ander programme.
settings-page-auth = Verifikasie
settings-page-auth-blurb = Hoe mense aanmeld: Hoard se eie rekeninge, 'n LDAP-gids, Microsoft Active Directory of 'n OAuth-verskaffer, hoe lank sessies duur, en watter groepe watter rol kry.
settings-page-connection = Verbinding
settings-page-connection-blurb = Hoe blaaiers en toepe Hoard bereik: die adres waarby dit bekend is, en HTTPS, wat sommige blaaierfunksies nodig het.
settings-page-performance = Werkverrigting
settings-page-performance-blurb = Hoe hard die bediener werk, hoe duimnaels gemaak en gehou word, en die meeste wat dit inneem.
settings-page-issues = Probleme
settings-page-issues-blurb = Wat verkeerd geloop het met die oopmaak of verwerking van lêers: argiewe wat nie wou uitpak nie, lêers wat nie gelees kon word nie, take wat misluk het. Probeer hulle weer sodra die oorsaak reggestel is, of wys hulle weg.
settings-page-maintenance = Instandhouding
settings-page-maintenance-blurb = Rugsteun, gaan na vir skade en ruim op. Een van elk loop op 'n slag, en jy kan hierdie bladsy verlaat terwyl hulle loop.
settings-the-default = die verstek
settings-default-on = aan
settings-default-off = af
settings-default-auto = outomaties
settings-option-saved = { $name }: { $value }
settings-back-to-default = Terug na die verstek
settings-save = Stoor
settings-use = Gebruik
settings-status-off = Af

## SettingsPage.vue: Voorkoms

settings-themes = Temas
settings-look-light = Lig
settings-look-dark = Donker
settings-look-group =
    { $scheme ->
        [light] Ligte tema
       *[dark] Donker tema
    }
settings-preview =
    Wys die { $preview ->
        [light] ligte
       *[dark] donker
    } tema sodat jy dit kan sien. Jy gebruik die { $used ->
        [light] ligte
       *[dark] donker
    } tema{ $system ->
        [yes] { " " }(volg die stelsel)
       *[other] { "" }
    }, en dit kom terug wanneer jy hierdie bladsy verlaat.
settings-preview-back =
    Terug na { $used ->
        [light] lig
       *[dark] donker
    }
settings-look-hint = Wissel tussen lig en donker (of volg die stelsel) vanuit die boonste balk.

## SettingsPage.vue: Gereedskap

settings-blender-what = Skakel .blend-, .dae-, .abc- en .x3d-modelle in die agtergrond na glTF om, sodat hulle duimnaels en 'n 3D-voorskou kry. Pakke wat bygevoeg is terwyl dit af was, word omgeskakel sodra dit gereed is.
settings-blender-download = Gebruik 'n geïnstalleerde Blender, of laai Blender 4.5 LTS (omtrent 400 MB) van 'n blender.org-spieël af.
settings-ffmpeg-what = Enkodeer MP3-, OGG- en Opus-aflaaie, en lees klank wat Hoard nie self kan dekodeer nie (AAC, M4A, WMA, …).
settings-ffmpeg-download = Gebruik 'n stelsel-ffmpeg met die regte enkodeerders, of laai 'n statiese bou (omtrent 80 MB) af.
settings-tool-reset = { $tool }: terug na die verstek
settings-tool-no-download = Gebruik 'n geïnstalleerde { $tool }. Daar is geen bou om vir hierdie masjien af te laai nie, so installeer dit of gee sy pad.
settings-tool-path = { $tool }-programpad
settings-mode-off = Af
settings-mode-auto = Outomaties
settings-mode-path = Hierdie program
settings-tool-looking = Soek…
settings-tool-downloading = Laai af… vordering is in die aktiwiteitsaanwyser
settings-tool-ready = Gereed: { $detail }
settings-soundfont = MIDI-SoundFont
settings-soundfont-what = Die instrumente waarmee MIDI-lêers gespeel en omgeskakel word.
settings-soundfont-file = SoundFont-lêer
settings-soundfont-placeholder = Op die bediener gevind, of ingebou
settings-soundfont-default = op die bediener gevind, of ingebou

## SettingsPage.vue: Beeldsoektog

settings-clip-heading = Soek volgens wat prente wys
settings-clip-reset = Beeldsoektog: terug na die verstek
settings-clip-saved-on = Beeldsoektog: aan
settings-clip-saved-off = Beeldsoektog: af
settings-clip-what = Tik waarna jy soek (“mossy rock”, “sci-fi door”, “wooden barrel”) en prente wat dit wys, verskyn langs name wat die woorde bevat. “Meer soos hierdie” vind dinge van dieselfde soort, nie net dieselfde kleure nie.
settings-clip-toggle = Skakel beeldsoektog aan
settings-clip-hint = Gebruik OpenAI se CLIP-model, wat op hierdie bediener se verwerker loop: niks verlaat dit nie. Om dit aan te skakel laai die model ({ $size } MB) van Hugging Face af, en leer dan elke prent in die agtergrond teen lae prioriteit. Dit neem 300 tot 600 MB geheue terwyl dit werk of iemand soek. Om dit af te skakel vee die model uit en hou wat dit geleer het, vir volgende keer.
settings-clip-looking = Soek die model…
settings-clip-downloading = Laai die model af… vordering is in die aktiwiteitsaanwyser
settings-clip-ready-empty = Gereed. Prente word geleer soos pakke bygevoeg word.
settings-clip-ready-all =
    { $count ->
       *[other] Gereed: ken al { $count } prente
    }
settings-clip-learning = Gereed: leer prente, { $done } van { $total } tot dusver
settings-clip-unavailable = Kon nie die model kry nie: { $detail }. Probeer elke uur weer.

## SettingsPage.vue: KI-etikettering

settings-ai-service = Diens
settings-ai-off = Af
settings-ai-off-hint = Pakke word net met die ingeboude reëls geëtiketteer.
settings-ai-local = Plaaslike model
settings-ai-local-hint = Die llm-diens in Hoard se compose-lêer (of cargo xtask llm). Laai hieronder 'n model af; dit loop op hierdie masjien en niks verlaat dit nie.
settings-ai-chatgpt = ChatGPT-plan
settings-ai-chatgpt-hint = Gebruik jou ChatGPT Go-, Plus-, Pro- of Business-plan in plaas van 'n API-sleutel.
settings-ai-openrouter-hint = Een sleutel vir honderde gehuisveste modelle, sommige daarvan gratis. Modelle wat prente aanvaar, is gemerk.
settings-ai-openai-hint = Gehuisves deur OpenAI.
settings-ai-anthropic-hint = Gehuisves deur Anthropic, deur sy OpenAI-versoenbare eindpunt.
settings-ai-gemini-hint = Gehuisves deur Google, deur sy OpenAI-versoenbare eindpunt.
settings-ai-ollama-hint = 'n Ollama-bediener wat jy self laat loop. Verander die adres as dit nie 'n houer met die naam ollama is nie.
settings-ai-custom = Ander (OpenAI-versoenbaar)
settings-ai-custom-hint = Enigiets wat die OpenAI-klets-API praat: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Koppel…
settings-ai-connected =
    { $count ->
        [one] Gekoppel · { $count } model
       *[other] Gekoppel · { $count } modelle
    }
settings-ai-connected-none = Gekoppel, maar dit lys geen modelle nie; tik die model se naam
settings-chatgpt-connected-none = Gekoppel; tik die model se naam
settings-ai-takes-images = aanvaar prente
settings-ai-saved = KI-etikettering: { $service }
settings-ai-saved-off = KI-etikettering is af
settings-ai-reset = KI-etikettering: terug na die verstek
settings-ai-on = Aan: { $service } · { $model }
settings-ai-url = Adres
settings-ai-key = API-sleutel
settings-ai-key-saved = Gestoor (tik om te vervang)
settings-ai-key-paste = Plak jou sleutel
settings-ai-key-optional = Geen (opsioneel)
settings-ai-key-remove = Verwyder
settings-ai-get-key = Kry 'n sleutel
settings-ai-model = Model
settings-ai-model-pick = Kies of tik 'n model
settings-ai-model-name = Modelnaam
settings-ai-refresh = Verfris
settings-ai-connect = Koppel
settings-ai-need-key = Voeg die API-sleutel by om sy modelle te sien
settings-ai-vision = Stuur ook duimnaels (het 'n model nodig wat prente aanvaar)
settings-ai-batch = Per versoek
settings-ai-batch-unit = bates (minder vir klein plaaslike modelle, meer vir groot gehuisveste modelle)

## SettingsPage.vue: ChatGPT-aanmelding

settings-chatgpt-account = Rekening
settings-chatgpt-checking = Gaan na…
settings-chatgpt-status-as = Aangemeld as { $email }
settings-chatgpt-status = Aangemeld
settings-chatgpt-signed-in-as = By ChatGPT aangemeld as { $email }
settings-chatgpt-signed-in = By ChatGPT aangemeld
settings-chatgpt-signed-out = Van ChatGPT afgemeld
settings-chatgpt-continue = Gaan voort met ChatGPT
settings-chatgpt-sign-out = Meld af
settings-chatgpt-sign-out-title = Meld van ChatGPT af?
settings-chatgpt-sign-out-in-use = KI-etikettering gebruik jou ChatGPT-plan, dus stop dit totdat jy weer aanmeld of 'n ander diens kies.
settings-chatgpt-sign-out-body = Hoard vergeet die aanmelding en vra OpenAI om dit te beëindig.
settings-chatgpt-manage = Bestuur gebruik
settings-chatgpt-finish-before = Voltooi die aanmelding in die venster wat oopgegaan het (
settings-chatgpt-finish-link = maak dit weer oop
settings-chatgpt-finish-middle = ). As dit eindig op 'n bladsy wat nie wil laai nie, by 'n adres wat begin met{ " " }
settings-chatgpt-finish-after = , kopieer daardie adres en plak dit hier:
settings-chatgpt-paste = Die adres waarop die aanmelding geëindig het
settings-chatgpt-finish = Voltooi
settings-chatgpt-usage = Etiketteringsversoeke tel teen jou ChatGPT-plan se gebruik. Bestuur gebruik wys wat Hoard gebruik het en laat jou toe om dit te beperk. Om Hoard van ChatGPT self te ontkoppel, gaan na Settings → Security and login → Sign in with ChatGPT op chatgpt.com.

## SettingsPage.vue: Plaaslike modelle

settings-local-downloading-progress = Laai af { $percent }% · { $done } van { $total }
settings-local-downloading = Laai af…
settings-local-incomplete = Aflaai het gestop
settings-local-failed = Kon laas nie begin nie
settings-local-in-use-running = In gebruik · loop
settings-local-in-use = In gebruik
settings-local-starting = Begin…
settings-local-running = Loop
settings-local-ready-images = Gereed · aanvaar prente
settings-local-ready-text = Gereed · net teks
settings-local-delete-title = Vee { $name } uit?
settings-local-delete-in-use = KI-etikettering gebruik hierdie model, dus stop dit totdat jy 'n ander kies. Sy lêers word van die modelbediener uitgevee.
settings-local-delete-downloading = Die aflaai stop en wat tot dusver afgekom het, word uitgevee.
settings-local-delete-body = Sy lêers word van die modelbediener uitgevee. Jy kan dit later weer aflaai.
settings-local-delete = Vee uit
settings-local-deleting = Vee uit…
settings-local-cancel = Kanselleer
settings-local-resume = Hervat
settings-local-download = Laai af
settings-local-start-before = Begin dit met{ " " }
settings-local-start-or = { " " }(of{ " " }
settings-local-start-developing = { " " }wanneer jy ontwikkel), en{ " " }
settings-local-start-retry = probeer weer
settings-local-start-after = . 'n Bediener wat net een model laat loop, werk ook: gee sy model se naam hieronder.
settings-local-list = Op die modelbediener
settings-local-none = Nog niks afgelaai nie. Kies een hieronder; dit word op die bediener gehou, en jy kan enige tyd oorskakel.
settings-local-catalog = Modelle om af te laai
settings-local-size = { $size } GB · { $license }
settings-local-other = Nog 'n model van Hugging Face
settings-local-other-placeholder = Nog een van Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Pakke byvoeg

settings-store-pages = Winkelbladsye
settings-store-pages-what = Nuwe pakke word in die agtergrond in die winkels opgesoek, sodat hulle hul winkelbladsy, verkoper en lisensie kry.
settings-store-lookup = Soek winkelbladsye outomaties op
settings-unreal = Unreal-pakkette
settings-unreal-reset = Unreal-pakkette: terug na die verstek
settings-unreal-what = Wat om met .uasset-lêers te doen.
settings-unreal-off = Los soos dit is
settings-unreal-off-hint = Skakel om vanaf 'n pak se bladsy wanneer jy wil.
settings-unreal-keep = Skakel om, hou oorspronklikes
settings-unreal-keep-hint = Teksture, klanke en materiale kom in algemene formate uit; die .uasset bly ook.
settings-unreal-replace = Skakel om, gooi oorspronklikes weg
settings-unreal-replace-hint = Bespaar spasie sodra die inhoud uit is. Mase wag steeds vir die Unreal-werker.

## SettingsPage.vue: Aflaaie

settings-tn = Tekstuurname
settings-tn-saved = Tekstuurname gestoor
settings-tn-reset = Tekstuurname: terug na die verstek
settings-tn-what = 'n Materiaal se kaarte gaan uit benoem na hom en waarvoor elke kaart is, hoe die lêers ook al benoem is: Rock_Normal.png, nie rock_moss_xeokah0_raw_8K_Normal.png nie. Die lêers in die biblioteek word nie aangeraak nie, en 'n kaart wat 'n model of MTL wat daarmee saam gestuur is noem, behou daardie naam, sodat die model dit steeds vind.
settings-tn-toggle = Benoem tekstuurkaarte na hul materiaal
settings-tn-style = Styl
settings-tn-style-group = Benoemingstyl
settings-tn-descriptive = Beskrywend
settings-tn-short = Kort
settings-tn-custom = Jou eie
settings-tn-pattern = Patroon
settings-tn-hint-name = { " " }die materiaal,{ " " }
settings-tn-hint-map = { " " }waarvoor die kaart is,{ " " }
settings-tn-hint-size = { " " }sy grootte wanneer die stel in meer as een kom (aan die einde bygevoeg as die patroon dit weglaat).
settings-tn-example = Voorbeeld
settings-tn-suffixes = Agtervoegsels vir elke soort kaart

## SettingsPage.vue: Algemeen en Verbinding

settings-name = Naam
settings-name-what = Wat hierdie bediener in die boonste balk, die blaaieroortjie en op die aanmeldbladsy genoem word, sodat jy met meer as een hoard kan sien watter een watter is.
settings-address = Adres
settings-address-what = Die adres wat mense en toepe gebruik om Hoard te bereik, wanneer dit nie die een is waarop hierdie blaaier is nie: dié van 'n omgekeerde instaanbediener, 'n tonnel of 'n tailnet-naam, soos https://hoard.example.com. Skakels wat Hoard uitdeel (aan Claude en ander toepe, by aanmelding, op sy sertifikaat) gebruik dit.
settings-public-url = Publieke adres
settings-public-url-placeholder = Uit die blaaier se adres (bv. https://hoard.example.com)
settings-public-url-default = uit die blaaier se adres

## SettingsPage.vue: Werkverrigting

settings-render = 3D-duimnaels
settings-render-toggle = Lewer duimnaels vir 3D-modelle
settings-gpu-on = Op die GPU: { $renderer }
settings-gpu-software = Geen GPU gevind nie, dus in sagteware ({ $renderer })
settings-gpu-none = Geen GPU of sagteware-weergewer gevind nie, dus kry modelle geen duimnaels nie
settings-thumbs = Duimnaels
settings-thumbs-what = Hoe nuwe duimnaels en modelaansigte gestoor word. Dié met deursigtigheid, en pixelkuns, is altyd PNG. Om dié wat jy reeds het te verklein, laat loop Verklein duimnaels in Instandhouding.
settings-previews-size = Voorskoue neem nou { $size } op.
settings-thumbs-saved = Duimnaels: { $quality }
settings-thumbs-small = Kleiner
settings-thumbs-small-hint = JPEG, kwaliteit 70. Goed in die rooster; effens sag van naby.
settings-thumbs-balanced = Gebalanseer
settings-thumbs-balanced-hint = JPEG, kwaliteit 82. Moeilik om van die oorspronklike te onderskei op duimnaelgrootte.
settings-thumbs-sharp = Skerper
settings-thumbs-sharp-hint = JPEG, kwaliteit 92. Omtrent twee keer so groot as Gebalanseer.
settings-thumbs-lossless = Verliesloos
settings-thumbs-lossless-hint = PNG, presies soos gemaak. Etlike kere groter.
settings-limits = Werk en limiete
settings-limits-what = Elkeen stoor wanneer jy die veld verlaat of Enter druk. Groottes neem eenhede: 100 GB, 512 MB, 1,5 TB; 'n gewone getal is GB.
settings-workers = Agtergrondwerkers
settings-workers-unit = take gelyktydig (invoer, etikettering, omskakelings)
settings-max-upload = Grootste oplaai
settings-max-upload-unit = per lêer
settings-max-expand = Grootste pak
settings-max-expand-unit = sodra uitgepak
settings-conversion-cache = Omgeskakelde aflaaie
settings-conversion-cache-unit-now = hoogstens gehou, nou { $size }; die minste onlangs gebruikte gaan eerste
settings-conversion-cache-unit = hoogstens gehou; die minste onlangs gebruikte gaan eerste

## SettingsPage.vue: Instandhouding

settings-updates = Bywerkings
settings-version = Dit is Hoard { $version }.
settings-update-check = Kyk elke paar uur op GitHub vir 'n nuwe weergawe

## Batesoorte (api.ts)

api-kind-texture = Teksture
api-kind-vector = Vektorkuns
api-kind-audio = Klank
api-kind-model = Modelle
api-kind-animation = Animasies
api-kind-font = Lettertipes
api-kind-shader = Shaders
api-kind-material = Materiale
api-kind-video = Video
api-kind-document = Dokumente
api-kind-data = Data
api-kind-other = Ander
api-kind-texture-name = teksture
api-kind-vector-name = vektorkuns
api-kind-audio-name = klank
api-kind-model-name = modelle
api-kind-animation-name = animasies
api-kind-font-name = lettertipes
api-kind-shader-name = shaders
api-kind-material-name = materiale
api-kind-video-name = video
api-kind-document-name = dokumente
api-kind-data-name = data
api-kind-other-name = ander

## Kennisgewings, dialoë en laai (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Ontdoen
ui-today = vandag
ui-yesterday = gister
ui-days-ago =
    { $days ->
        [one] { $days } dag gelede
       *[other] { $days } dae gelede
    }
ui-notifications = Kennisgewings
ui-dismiss = Wys weg
ui-cancel = Kanselleer
ui-lazy-loading = Laai { $what }…
ui-lazy-failed = Kon nie { $what } laai nie.
ui-lazy-reload = Herlaai die bladsy

## ResetButton.vue

reset-title = Terugstel na die verstek: { $value }
reset-label = Terugstel na die verstek, { $value }

## SizeInput.vue

size-invalid = 'n Grootte soos 100 GB, 512 MB of 1,5 TB
size-hint = 'n Grootte soos 100 GB, 512 MB of 1,5 TB; 'n gewone getal is GB

## AuthPanel.vue (Instellings → Verifikasie)

auth-load-failed = Kon nie die verifikasie-instellings laai nie: { $error }
auth-saved = Gestoor. Dit geld vir die volgende aanmelding.
auth-method-internal = Intern
auth-method-internal-hint = Name en wagwoorde wat Hoard hou, bygevoeg in Instellings → Rekening.
auth-method-ldap-hint = 'n Gids gaan die naam en wagwoord na wat op die aanmeldbladsy getik is: OpenLDAP, FreeIPA, LLDAP, of 'n ander LDAP-bediener.
auth-method-ad-hint = Windows-domeinrekeninge meld aan met hul gewone naam en wagwoord.
auth-method-oauth-hint = 'n “Meld aan met…”-knoppie wat na Google, Microsoft, GitHub, Keycloak, Authentik of 'n ander verskaffer gaan en terug.
auth-session-unused = Ná 30 dae ongebruik
auth-session-1h = 1 uur ná aanmelding
auth-session-8h = 8 uur ná aanmelding
auth-session-day = 'n Dag ná aanmelding
auth-session-week = 'n Week ná aanmelding
auth-session-30d = 30 dae ná aanmelding
auth-session-hours =
    { $hours ->
        [one] { $hours } uur ná aanmelding
       *[other] { $hours } uur ná aanmelding
    }
auth-role-view = Kan kyk
auth-role-edit = Kan verander
auth-role-admin = Admin
auth-role-word-view = kan kyk
auth-role-word-edit = kan verander
auth-role-word-admin = admin
auth-role-word-default = { $role } (die verstekrol)
auth-tls-starttls-hint = Gewone LDAP (poort 389) wat na TLS oorskakel voordat enigiets gestuur word.
auth-tls-ldaps-hint = TLS van die begin af (poort 636).
auth-tls-none = Geen
auth-tls-none-hint = Wagwoorde gaan oor die netwerk soos hulle is. Net op 'n netwerk wat jy vertrou.
auth-test-failed = Kon nie toets nie: { $error }
auth-preset-custom = Ander (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel en ander: gee die issuer en Ontdek vul die adresse in.
auth-preset-google-note = Maak 'n OAuth-kliënt (Web application) in Google Cloud Console → APIs & Services → Credentials. Google stuur geen groepe nie, so gee rolle in Instellings → Rekening, en skakel die maak van rekeninge by die eerste aanmelding af tensy enigiemand met 'n Google-rekening moet kan inkom.
auth-preset-microsoft-note = Registreer 'n toep in Entra ID → App registrations ('n Web-herleiding na die adres hieronder). Sit jou tenant-ID in die plek van “organizations” om net jou organisasie in te laat. Vir rolle uit groepe, voeg die groups-claim by onder Token configuration: dit lys groep-ID's.
auth-preset-github-note = Maak 'n OAuth-toep in GitHub → Settings → Developer settings. GitHub stuur geen groepe nie, so gee rolle in Instellings → Rekening, en skakel die maak van rekeninge by die eerste aanmelding af tensy enigiemand met 'n GitHub-rekening moet kan inkom.
auth-preset-gitlab-note = Maak 'n toepassing in GitLab → Preferences → Applications (of jou instansie se admin-area: verander gitlab.com in die adresse).
auth-discovered = Die verskaffer se adresse is ingevul.
auth-copied = Gekopieer
auth-copy-failed = Kon nie kopieer nie: kies dit en kopieer dit self
auth-sign-in-with = Meld aan met
auth-own-accounts-still-work = Hoard se eie rekeninge meld steeds met hul eie wagwoorde aan, so 'n gids of verskaffer wat af is of verkeerd opgestel is, kan jou nie uitsluit nie.
auth-sessions = Sessies
auth-sign-out-after = Meld mense af
auth-ldap-server = LDAP-bediener
auth-server-type = Bedienertipe
auth-server-type-custom = Pasgemaak
auth-servers = Bedieners
auth-server-n = Bediener { $n }
auth-remove-server-n = Verwyder bediener { $n }
auth-remove = Verwyder
auth-add-server = Voeg 'n bediener by
auth-servers-hint = Die volgende een word probeer wanneer een nie bereik kan word nie.
auth-security = Sekuriteit
auth-connection-security = Verbindingsekuriteit
auth-skip-verify = Moenie die bediener se sertifikaat nagaan nie (net om dinge uit te probeer)
auth-ca = Sertifikaatowerheid
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (leeg: dié wat die stelsel vertrou)
auth-lookup = Mense opsoek
auth-anonymously = Anoniem
auth-reader-account = Leesrekening
auth-reader-password = Sy wagwoord
auth-saved-type-to-replace = Gestoor: tik om te vervang
auth-people-under = Mense is onder
auth-name-attribute = Naamattribuut
auth-only-matching = Net inskrywings wat pas by
auth-groups-under = Groepe is onder
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (leeg: net memberOf)
auth-their-groups = Hul groepe
auth-group-filter-hint = { "{" }dn{ "}" } is die persoon se inskrywing, { "{" }user{ "}" } die naam waarmee hulle aangemeld het.
auth-test = Toets
auth-test-hint = Bereik elke bediener en meld aan as die leesrekening. Met 'n naam en wagwoord meld dit ook as hulle aan en wys wat Hoard daarvan sou maak. Niks word gestoor nie.
auth-test-user = Iemand om as aan te meld
auth-test-user-placeholder = Iemand om as aan te meld (opsioneel)
auth-test-password = Hul wagwoord
auth-testing = Toets…
auth-test-connectivity = Toets verbinding
auth-test-server-ok = bereik, en die leesrekening het aangemeld
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }het aangemeld (
auth-test-signed-in-after = ).
auth-test-role = Hul rol sou wees: { $role }.
auth-test-no-role = Hulle is in geeneen van die groepe wat onder Rolle gelys is nie, dus sou hulle weggewys word.
auth-test-groups = Groepe: { $groups }
auth-test-no-groups = Geen groepe gevind nie.
auth-provider = Verskaffer
auth-redirect = Herleidingsadres
auth-copy = Kopieer
auth-redirect-hint = Gee dit aan die verskaffer wanneer jy Hoard se kliënt maak. Dit kom van die publieke adres in Instellings → Verbinding, of die adres wat jy gebruik.
auth-issuer = Issuer
auth-discover = Ontdek
auth-client-id = Kliënt-ID
auth-client-secret = Kliëntgeheim
auth-authorization-url = Magtigingsadres
auth-token-url = Token-adres
auth-userinfo-url = Gebruikersinligting-adres
auth-userinfo-placeholder = (leeg: die ID-token sê wie dit is)
auth-scopes = Scopes
auth-name-from = Naam uit
auth-groups-from = Groepe uit
auth-groups-from-placeholder = (geen)
auth-button = Knoppie
auth-button-placeholder = jou rekening
auth-accounts = Rekeninge
auth-provision = Maak 'n Hoard-rekening die eerste keer dat iemand aanmeld
auth-provision-on-hint = Andersins voeg 'n admin eers elke persoon by, in Instellings → Rekening.
auth-provision-off-hint = 'n Admin voeg eers elke persoon by, in Instellings → Rekening, en kies hoe hulle aanmeld.
auth-open-to-anyone = Enigiemand met 'n { $provider }-rekening kan inkom. Voeg hieronder kykersgroepe by, of skakel dit af en voeg self mense by.
auth-new-accounts = Nuwe rekeninge
auth-roles-from-groups = Rolle uit groepe
auth-roles-hint = Groepe waarvan die lede elke rol kry, een per reël, volgens naam of DN. Die hoogste wat pas, wen, en dit word by elke aanmelding nagegaan. As geen gelys is nie, kry nuwe rekeninge die rol hierbo en 'n admin verander dit daarna. As groepe onder Kan kyk gelys is, mag net lede van 'n gelyste groep aanmeld.
auth-roles-hint-oauth = Groepe waarvan die lede elke rol kry, een per reël, volgens naam of DN (of wat ook al die groups-claim lys). Die hoogste wat pas, wen, en dit word by elke aanmelding nagegaan. As geen gelys is nie, kry nuwe rekeninge die rol hierbo en 'n admin verander dit daarna. As groepe onder Kan kyk gelys is, mag net lede van 'n gelyste groep aanmeld.
auth-saving = Stoor…
auth-save = Stoor

## HttpsPanel.vue (Instellings → Verbinding)

https-load-failed = Kon nie HTTPS laai nie: { $error }
https-off-redirecting = HTTPS is af. Neem jou na http://
https-now-on = HTTPS is aan
https-saved = Gestoor. Nuwe verbindings gebruik die nuwe sertifikaat.
https-now-off = HTTPS is af
https-http3-failed-toast = HTTP/3 kon nie begin nie: { $error }
https-copied = Gekopieer
https-copy-failed = Kon nie kopieer nie: kies dit en kopieer dit self
https-mode-off = Af
https-mode-off-hint = Net gewone http://.
https-mode-own = Hoard se eie sertifikaat
https-mode-own-hint = Hoard maak een vir hierdie bediener se adresse. Installeer Hoard se sertifikaatowerheid een keer op elke toestel en sy blaaiers vertrou dit.
https-mode-custom = My sertifikaat
https-mode-custom-hint = Een wat jy reeds het, bv. van tailscale cert of Let’s Encrypt. Blaaiers vertrou dit sonder om enigiets te installeer.
https-untrusted-own = Hierdie blaaier vertrou dit nog nie vir { $host } nie: installeer Hoard se sertifikaatowerheid hieronder. Jy sal 'n waarskuwing sien waarby jy kan verbyklik.
https-untrusted-uncovered = Hierdie blaaier vertrou dit nog nie vir { $host } nie: die sertifikaat dek nie hierdie adres nie. Jy sal 'n waarskuwing sien waarby jy kan verbyklik.
https-untrusted = Hierdie blaaier vertrou dit nog nie vir { $host } nie. Jy sal 'n waarskuwing sien waarby jy kan verbyklik.
https-now = Nou
https-start-failed = HTTPS is opgestel maar kon nie begin nie: { $error }
https-on-port = HTTPS is aan, op poort { $port } soos http://.
https-using-it = Jy gebruik dit nou.
https-open = Maak { $url } oop
https-trusted = Hierdie blaaier vertrou dit.
https-cert-for-before = Sertifikaat vir{ " " }
https-cert-for-after = , tot { $date }.
https-cert-for-after-own = , tot { $date } (word self hernu).
https-not-covered-before = Dit dek nie{ " " }
https-not-covered-after = { " " }nie, die adres wat jy gebruik.
https-is-off = HTTPS is af: bladsye word oor gewone http:// bedien.
https-addresses = Adresse
https-addresses-hint = Die sertifikaat dek die adresse wat Hoard gevind het, en enige wat jy byvoeg ('n naam wat jou roeteerder dit gee, 'n Tailscale-naam).
https-remove-name = Verwyder { $name }
https-another-address = Nog 'n adres
https-another-address-placeholder = Nog 'n adres, bv. nas.local
https-add = Voeg by
https-your-cert = Jou sertifikaat
https-saved-cert-before = Gestoor:{ " " }
https-saved-cert-after = { " " }van { $issuer }, tot { $date }. Plak 'n nuwe een om dit te vervang.
https-pem-before = Die sertifikaat (met sy ketting) en sy privaat sleutel, as PEM-teks: die lêers wat begin met{ " " }
https-pem-after = .
https-certificate = Sertifikaat
https-choose-file = Kies lêer…
https-private-key = Privaat sleutel
https-checked-before = Vir{ " " }
https-checked-after = , van { $issuer }, tot { $date }.
https-checked-after-days =
    , van { $issuer }, tot { $date } ({ $days ->
        [one] { $days } dag
       *[other] { $days } dae
    }).
https-self-signed = Dit is selfondertekend: blaaiers sal daaroor waarsku.
https-key-matches = Die sleutel pas.
https-where-to-get = Waar om een te kry
https-get-tailscale-before = skakel HTTPS vir jou tailnet aan, en voer dan{ " " }
https-get-tailscale-after = { " " }op die bediener uit en laai die twee lêers op. Hulle hou 90 dae: voer dit weer uit en laai hulle weer op voor dan.
https-get-acme-title = Let’s Encrypt of 'n ander ACME-kliënt
https-get-acme-before = (certbot, acme.sh, lego) vir 'n domein wat jy besit: laai{ " " }
https-get-acme-and = { " " }en{ " " }
https-get-acme-after = { " " }op.
https-get-proxy-title = 'n Omgekeerde instaanbediener
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) wat die sertifikaat het: laat HTTPS hier af en laat die instaanbediener gewone http:// na Hoard stuur.
https-trying = Probeer dit…
https-apply = Pas toe
https-apply-hint = Hoard probeer eers die nuwe opstelling, en hou wat dit het as dit nie werk nie.
https-trust-title = Vertrou Hoard op jou toestelle
https-download-ca = Laai hoard-ca.crt af
https-trust-hint = Installeer Hoard se sertifikaatowerheid een keer op elke toestel, en sy blaaiers vertrou hierdie bediener (en enige sertifikaat wat Hoard later daarvoor maak). Dit kan net vir hierdie Hoard instaan. Vingerafdruk (SHA-256):
https-ca-windows = Maak die lêer oop → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Herbegin die blaaier.
https-ca-macos = Maak die lêer oop; dit gaan na Keychain Access. Dubbelklik daar op “{ $subject }” → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone en iPad
https-ca-ios = Laai dit in Safari af en laat dit toe. Settings → Profile Downloaded → Install. Dan Settings → General → About → Certificate Trust Settings → skakel “{ $subject }” aan.
https-ca-android = Settings → Security → More security settings → Encryption & credentials → Install a certificate → CA certificate, en kies die lêer. (Die name verskil effens tussen fone; soek in Settings na “certificate”.)
https-ca-linux-1 = Debian en Ubuntu:{ " " }
https-ca-linux-2 = . Fedora en Arch:{ " " }
https-ca-linux-3 = . Chrome op Linux hou sy eie lys: Settings → Privacy and security → Security → Manage certificates → Import.
https-ca-firefox = Firefox hou sy eie lys: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import, en merk “Trust this CA to identify websites”.
https-ca-cli-title = Die hoard-opdragreëlhulpmiddel
https-ca-cli-before = { "" }
https-ca-cli-middle = , of sit die pad in{ " " }
https-ca-cli-after = .
https-options = Opsies
https-redirect = Stuur blaaiers op ander toestelle van http:// na https://
https-redirect-hint = Net bladsylaaie word aangestuur. Die hoard-opdragreëlhulpmiddel, die Unreal-werker, skrips en KI-toepe werk steeds op http://, en so ook http://localhost op die bediener self.
https-http3-hint-before = Vinniger op onbetroubare Wi-Fi en fone. Het HTTPS nodig, en UDP-poort { $port } oop sowel as TCP (in Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Loop.
https-http3-waits = Dit begin wanneer HTTPS aan is.
https-http3-failed = HTTP/3 kon nie begin nie: { $error }
https-lockout-title = As jy uitgesluit word
https-lockout-1 = { "" }
https-lockout-2 = { " " }op die bediener self werk altyd oor gewone http://. Of skakel HTTPS af vanaf die bediener se opdragreël en begin Hoard weer:{ " " }
https-lockout-3 = { " " }(in Docker:{ " " }
https-lockout-4 = , dan{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Instellings → Rekening)

account-role-view = Kan kyk
account-role-view-hint = Soek, voorskou en aflaai
account-role-edit = Kan verander
account-role-edit-hint = Ook byvoeg, verander, etiketteer, versamel en argiveer
account-role-admin = Admin
account-role-admin-hint = Ook Instellings, rekeninge en instandhouding, en uitvee vir altyd
account-source-internal = 'n Hoard-wagwoord
account-source-ldap = Hul { $directory }-wagwoord
account-source-oauth = Meld aan met { $provider }
account-source-word-directory = gids
account-load-failed = Kon nie rekeninge laai nie: { $error }
account-failed = { $what }: { $error }
account-password-failed = Kon nie jou wagwoord verander nie
account-password-changed = Wagwoord verander. Jou ander toestelle is afgemeld.
account-add-failed = Kon nie { $name } byvoeg nie
account-added = { $name } kan nou aanmeld.
account-role-failed = Kon nie verander wat { $name } kan doen nie
account-remove-title = Verwyder { $name }?
account-remove-body = Hulle word oral afgemeld en kan nie weer aanmeld nie. Niks wat hulle by die biblioteek gevoeg het, word verwyder nie.
account-remove = Verwyder
account-remove-failed = Kon nie { $name } verwyder nie
account-token-failed = Kon nie die token maak nie
account-copied = Gekopieer
account-copy-token-failed = Kon nie kopieer nie: kies die token en kopieer dit self
account-copy-failed = Kon nie kopieer nie: kies dit en kopieer dit self
account-revoke-title = Herroep “{ $name }”?
account-revoke-body = Enigiets wat dit nog gebruik, word van nou af weggewys.
account-revoke = Herroep
account-revoke-failed = Kon nie “{ $name }” herroep nie
account-disconnect-title = Ontkoppel { $app }?
account-disconnect-body = Dit word van nou af weggewys, en moet weer gekoppel word om Hoard te gebruik.
account-disconnect = Ontkoppel
account-disconnect-failed = Kon nie { $app } ontkoppel nie
account-guest-failed = Kon nie dit verander nie
account-never = nooit
account-you = Jy
account-sign-out = Meld af
account-signed-in-as-before = Aangemeld as{ " " }
account-signed-in-as-admin = , wat 'n admin is.
account-signed-in-as-edit = , wat die biblioteek kan verander.
account-signed-in-as-view = , wat kan kyk maar niks kan verander nie.
account-password-at-provider = Jou wagwoord word gehou deur die verskaffer waarmee jy aanmeld, so jy verander dit daar.
account-password-at-directory = Jou wagwoord word gehou deur die gids waarmee jy aanmeld, so jy verander dit daar.
account-current-password = Huidige wagwoord
account-new-password = Nuwe wagwoord
account-new-password-again = Nuwe wagwoord weer
account-change-password = Verander wagwoord
account-password-mismatch = Die nuwe wagwoorde stem nie ooreen nie.
account-password-rule = Minstens 8 karakters.
account-people = Mense
account-people-hint = Elke persoon kan kyk (soek, voorskou en aflaai), ook die biblioteek verander (byvoeg, wysig, etiketteer, versamel en argiveer), of boonop 'n admin wees (Instellings, rekeninge en instandhouding, en uitvee vir altyd).
account-you-tag = (jy)
account-signs-in-oauth = Meld aan met die OAuth-verskaffer
account-signs-in-directory = Meld aan met die gids
account-since = sedert { $date }
account-what-can-do = Wat { $name } kan doen
account-name = Naam
account-how-sign-in = Hoe hulle aanmeld
account-password-label = Wagwoord (minstens 8 karakters)
account-password-placeholder = Wagwoord (8+ karakters)
account-what-they-can-do = Wat hulle kan doen
account-add = Voeg by
account-ai-apps = Claude en ander KI-toepe
account-ai-apps-hint = Koppel Claude (of enige MCP-toep) en dit kan bates vir jou soek en haal, en beskryf en etiketteer wat Hoard nog nie het nie. Dit meld die eerste keer by Hoard aan; jy laat dit hier in die blaaier toe.
account-mcp-address = Hoard se MCP-adres
account-copy = Kopieer
account-claude-code-1 = voer uit:{ " " }
account-claude-code-2 = , dan{ " " }
account-claude-code-3 = { " " }om aan te meld.{ " " }
account-claude-code-4 = { " " }beskryf 'n pak.
account-claude-ai = Settings → Connectors → Add custom connector, plak die adres, dan Connect. Dié koppel vanaf Anthropic se bedieners, so Hoard moet oor HTTPS vanaf die internet bereikbaar wees.
account-claude-ai-local = Settings → Connectors → Add custom connector, plak die adres, dan Connect. Dié koppel vanaf Anthropic se bedieners, so Hoard moet oor HTTPS vanaf die internet bereikbaar wees (hierdie adres is nie: stel die publieke adres in Instellings → Verbinding).
account-connection-meta = vir { $user } sedert { $since } · laas gebruik { $used }
account-no-apps = Nog geen toepe gekoppel nie.
account-tokens = Toegangtokens
account-tokens-hint-1 = Vir die{ " " }
account-tokens-hint-2 = { " " }-opdragreëlhulpmiddel, skrips, MCP-kliënte en die Unreal-werker: hulle stuur dit as{ " " }
account-tokens-hint-3 = . 'n Token kan doen wat jy kies, tot by wat jy kan, maar nooit rekeninge of tokens bestuur nie. Een wat net kyk, is genoeg om te soek en af te laai, en om 'n projek te sinkroniseer.
account-tokens-admin = Jy sien almal se tokens en gekoppelde toepe; ander sien hul eie.
account-made-before = { "" }
account-made-after = : kopieer dit nou. Dit word nie weer gewys nie.
account-new-token = Die nuwe token
account-cli-1 = Vir die CLI:{ " " }
account-cli-2 = , of sit dit in{ " " }
account-cli-3 = .
account-done = Klaar
account-token-meta = gemaak { $made } · laas gebruik { $used }
account-token-meta-by = gemaak { $made } deur { $by } · laas gebruik { $used }
account-no-tokens = Nog geen tokens nie.
account-token-name = Wat dit sal gebruik
account-token-name-placeholder = Wat dit sal gebruik, bv. “Unreal-werker”
account-token-role = Wat dit kan doen
account-make-token = Maak 'n token
account-guests = Gaste
account-guests-allow = Laat enigiemand wat die bediener kan bereik, blaai sonder om aan te meld
account-guests-hint = Hulle kan soek, voorskou en aflaai, maar niks byvoeg, verander of uitvee nie, of Instellings sien nie.

## SignIn.vue

signin-create-title = Skep jou rekening
signin-title = Meld aan
signin-setup-lede = Hierdie bediener het nog geen rekening nie. Die een wat jy hier maak, kan alles doen; voeg later rekeninge vir ander mense by vanaf Instellings → Rekening.
signin-overlay-lede = Blaai is hier vir almal oop, maar veranderinge het 'n rekening nodig.
signin-with = Meld aan met { $provider }
signin-or-password = of met 'n naam en wagwoord
signin-username = Gebruikersnaam
signin-name = Naam
signin-new-password = Nuwe wagwoord
signin-password = Wagwoord
signin-password-rule = Minstens 8 karakters.
signin-confirm-password = Bevestig wagwoord
signin-mismatch = Die twee wagwoorde stem nie ooreen nie.
signin-setup-code = Opstelkode
signin-setup-code-hint-before = Die kode wat hierdie bediener gegee is in{ " " }
signin-setup-code-hint-after = .
signin-creating = Skep…
signin-signing-in = Meld aan…
signin-create = Skep rekening
signin-submit = Meld aan
signin-keep-browsing = Hou aan blaai
signin-directory-hint = Gebruik jou { $directory }-naam en -wagwoord.
signin-forgot-1 = 'n Hoard-wagwoord vergeet? Voer op die bediener{ " " }
signin-forgot-2 = { " " }uit (in Docker,{ " " }
signin-forgot-3 = ) vir 'n nuwe een.

## SetupWizard.vue (eerste besoek aan 'n nuwe bediener)

setup-title = Waar moet jou biblioteek woon?
setup-lede = Kies 'n vouer op die bediener waarin Hoard jou pakke hou, of wys dit na 'n vouer met pakke wat jy reeds het. Jy kan later meer liggings, op ander skywe, vanaf Instellings byvoeg.
setup-unreachable = Kon nie die bediener bereik nie: { $error }
setup-try-again = Probeer weer
setup-loading = Laai…
setup-start = Begin die biblioteek
setup-ready = Jou biblioteek is gereed
setup-managed-before = Nuwe pakke gaan in{ " " }
setup-managed-after = . Los lêers, vouers of argiewe enige plek op die bladsy om jou eerstes by te voeg.
setup-watched-before = Hoard monitor{ " " }
setup-watched-after = ; pakke verskyn soos dit hulle vind. Om ook op te laai, voeg 'n biblioteek wat Hoard hou by vanaf Instellings → Biblioteekliggings.
setup-open = Maak die biblioteek oop

## ConnectApp.vue ('n toep soos Claude vra om aan te meld)

connect-title = Koppel { $app }?
connect-lede-1 = { " " }sal alles in hierdie Hoard kan deursoek, aflaai, beskryf en etiketteer, as{ " " }
connect-lede-2 = . Dit kan nie rekeninge of toegangtokens bestuur nie.
connect-back-before = Jy gaan terug na{ " " }
connect-back-after = . Ontkoppel dit enige tyd in Instellings → Rekening.
connect-allow = Laat toe
connect-cancel = Kanselleer
connect-failed = Kan nie hierdie toep koppel nie
connect-go-library = Gaan na die biblioteek
connect-checking = Gaan na…

## InsecureNotice.vue (die reël wat oor gewone http:// gewys word)

insecure-text = Dit is gewone http://, so 'n paar dinge het HTTPS nodig: 3D met WebGPU, die moderne knipbord, Hoard as 'n toep installeer, en Claude-connectors.
insecure-switch = Skakel oor na https://
insecure-set-up = Stel HTTPS op
insecure-dismiss = Moenie dit weer wys nie

## FormatWizard.vue (Instellings → Winkellêers → Hoard 'n winkel se lêer leer)

formatwizard-too-big = Hierdie lêer is te groot om as 'n winkel s'n gelees te word.
formatwizard-not-json = Hierdie lêer is nie geldige JSON nie, so dit word as teks gewys.
formatwizard-read-failed = Kon nie die lêer lees nie: { $error }
formatwizard-back-title = Terug na die vorm?
formatwizard-back-body = Veranderinge in die TOML gaan verlore: die vorm skryf die formaat weer uit wat dit het.
formatwizard-back-to-form = Terug na die vorm
formatwizard-miss-name = Sy lêers neem nie die naam { $file } nie. Voeg dit by Lêers.
formatwizard-miss-not-the-stores = Dit waaraan dit die winkel se lêers herken, is nie in hierdie lêer nie. Verander Herken aan.
formatwizard-miss-no-title = Dit vind geen titel in hierdie lêer nie. Kies die titel.
formatwizard-miss-short-name = nie sy naam nie
formatwizard-miss-short-not-the-stores = nie die winkel s'n nie
formatwizard-miss-short-no-title = geen titel
formatwizard-saved-packs =
    { $file } gestoor. { $packs ->
        [one] { $packs } pak word
       *[other] { $packs } pakke word
    } weer gelees.
formatwizard-saved = { $file } gestoor.
formatwizard-save-failed = Kon nie die formaat stoor nie: { $error }
formatwizard-zip-failed = Kon nie die zip maak nie: { $error }
formatwizard-stop-title = Hou op om hierdie lêer te leer?
formatwizard-stop-body = Die formaat is nie gestoor nie.
formatwizard-stop = Hou op
formatwizard-label = Formaatassistent
formatwizard-changing = Verander { $name }
formatwizard-starting-from = Begin vanaf { $name }
formatwizard-title = Leer Hoard 'n winkel se lêer
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }in{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Wysig as TOML
formatwizard-file-reference = Die lêer, ter verwysing.
formatwizard-click-json = Klik op die { $field } in die lêer.
formatwizard-click-text = Klik op die { $field } in die lêer, of kies dit.
formatwizard-pick-first = Kies 'n veld regs, en klik dan hier op sy waarde.
formatwizard-loading = Laai…
formatwizard-the-format = Die formaat
formatwizard-syntax-before = Die sintaksis is in{ " " }
formatwizard-syntax-after = .
formatwizard-store = Winkel
formatwizard-store-placeholder = Die winkel se naam, soos Kenney
formatwizard-files = Lêers
formatwizard-recognize = Herken aan
formatwizard-recognize-json = 'n Sleutel wat net die winkel se lêers het
formatwizard-recognize-text = 'n Reël wat net die winkel se lêers het
formatwizard-recognize-hint = Een manier per reël; enigeen daarvan is genoeg. Nodig wanneer ander lêers dieselfde naam het.
formatwizard-seller = Verkoper
formatwizard-seller-placeholder = Wanneer die lêer dit nie sê nie
formatwizard-tags = Etikette
formatwizard-tags-placeholder = Vir alles wat dit beskryf, geskei deur kommas
formatwizard-fields = Velde
formatwizard-selector = Kieser vir { $field }
formatwizard-clear = Maak { $field } skoon
formatwizard-pick = Kies
formatwizard-what-it-reads = Wat dit lees
formatwizard-read-title = Titel
formatwizard-read-seller = Verkoper
formatwizard-read-license = Lisensie
formatwizard-read-url = Winkelbladsy
formatwizard-read-kind = Soort
formatwizard-read-tags = Etikette
formatwizard-read-origin = Gemaak in
formatwizard-read-size = Grootte
formatwizard-also = { $format } lees ook hierdie lêer. Hierdie formaat sal eerste probeer word.
formatwizard-library =
    { $files ->
        [one] { $files } lêer
       *[other] { $files } lêers
    } in { $packs ->
        [one] { $packs } pak
       *[other] { $packs } pakke
    } het 'n naam wat dit neem. Van { $tried } wat probeer is, lees dit { $read }.
formatwizard-library-none = Geen lêer in die biblioteek het 'n naam wat dit neem nie.
formatwizard-share = Deel dit
formatwizard-share-hint = 'n Zip van die formaat, hierdie lêer as sy voorbeeld en wat die voorbeeld as lees, uitgelê soos Hoard se repository, om in te stuur sodat almal dit kry. Haal eers enigiets in die voorbeeld uit wat nie gedeel moet word nie.
formatwizard-sample = Voorbeeld om te deel
formatwizard-download = Laai { $file } af
formatwizard-close = Sluit
formatwizard-use = Gebruik dit hier
formatwizard-cancel = Kanselleer
