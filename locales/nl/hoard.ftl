## Serverfouten

error-account-not-found = account niet gevonden
error-asset-gone = asset niet gevonden
error-asset-not-found = asset { $id } niet gevonden
error-blob-not-found = blob { $hash } niet gevonden
error-collection-not-found = collectie { $id } niet gevonden
error-connection-not-found = verbinding niet gevonden
error-derivative-not-found = afgeleid bestand ({ $kind }) van { $id } niet gevonden
error-file-in-pack-not-found = { $path } in pakket { $id } niet gevonden
error-site-not-a-page = alleen webpagina’s openen als pagina
error-site-link-expired = het adres van deze pagina is verlopen; open de pagina opnieuw vanuit Hoard
error-format-not-found = formaat { $file } niet gevonden
error-frame-sheet-aseprite-only = alleen Aseprite-bestanden hebben een framesheet
error-frame-sheet-frames = dit heeft één frame, of te veel om naast elkaar te zetten
error-location-not-found = locatie { $id } niet gevonden
error-maintenance-task-not-found = onderhoudstaak { $id } niet gevonden
error-open-upload-not-found = open upload { $id } niet gevonden
error-original-not-found = origineel bestand van { $id } niet gevonden
error-pack-not-found = pakket { $id } niet gevonden
error-preview-too-big = deze afbeelding is te groot om er een licht voorbeeld van te maken
error-project-not-found = project { $name } niet gevonden
error-saved-search-not-found = opgeslagen zoekopdracht { $id } niet gevonden
error-sign-in-first = log eerst in (of stuur een toegangstoken mee)
error-those-assets-not-found = die assets zijn niet gevonden
error-thumbnail-not-found = miniatuur van { $id } niet gevonden
error-token-not-found = token niet gevonden
error-upload-not-found = upload { $id } niet gevonden
error-pack-needs-name = een pakket heeft een naam nodig
error-no-library = er is nog geen bibliotheeklocatie om bestanden in op te slaan: voeg een beheerde locatie toe in Instellingen → Bibliotheeklocaties
error-upload-failed = upload mislukt: { $reason }
error-upload-no-files = geen bestanden in de upload
error-analyze-nothing-to-do = niets te doen: rules=false en het LLM staat uit
error-too-many-assets = te veel assets in één verzoek
error-change-not-found = die wijziging staat niet in de geschiedenis van dit asset
error-bulk-by-look = een zoekopdracht op uiterlijk kan niet in één keer worden bewerkt: selecteer de assets
error-too-many-items = te veel items in één verzoek
error-not-a-hash = geen blake3-hash in hex
error-merge-into-tag = into moet een tag zijn
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = "{ $text }" is geen tag
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } staat al onder { $tag }, dus { $tag } kan er niet onder
error-nothing-to-merge = niets om samen te voegen
error-selection-size = geef tussen 1 en { $max } asset-ID's op
error-collection-needs-name = een collectie heeft een naam nodig
error-collection-exists = er is al een collectie met de naam { $name }
error-collection-empty = collectie { $name } is leeg
error-conversion-failed = kon niet converteren naar { $format }: { $reason }
error-max-size = max moet een macht van twee zijn van 64 tot 16384
error-replace-ext = ext moet een korte bestandsextensie zijn
error-empty-file = leeg bestand
error-imitation-silent = Niets in de opname steekt boven de achtergrond uit. Probeer het opnieuw dichter bij de microfoon.
error-replace-duplicate = het geconverteerde bestand is gelijk aan een bestand dat al in het pakket zit; de asset is ermee samengevoegd
error-ai-address = Het AI-adres moet beginnen met http:// of https://
error-hosted-key = Vul de API-sleutel voor { $provider } in
error-hosted-address = Het adres van de dienst voor zoeken op beeld moet beginnen met http:// of https://
error-local-llm-action = action is list, download of delete, en de laatste twee hebben een model nodig
error-role = Een rol is view, edit of admin
error-admin-only = alleen een beheerder kan dit doen
error-view-only = dit account kan kijken maar niets wijzigen: een beheerder kan dat aanpassen in Instellingen → Account
error-portal-only = log in op het portaal om accounts en tokens te beheren
error-not-from-portal = dit verzoek kwam niet van het Hoard-portaal
error-account-name = Kies een naam van maximaal 64 tekens
error-password-length = Gebruik een wachtwoord van minstens { $min } tekens
error-too-many-codes = Te veel verkeerde codes: wacht { $seconds } seconden
error-already-set-up = Er is al een account: log in
error-too-many-tries = Te veel mislukte pogingen: wacht { $seconds } seconden
error-directory-unavailable = De directory kon je wachtwoord niet controleren: { $reason }
error-password-elsewhere = Je wachtwoord wordt beheerd door de directory of provider waarmee je inlogt
error-current-password = Je huidige wachtwoord klopt niet
error-account-source = Een account logt in via Hoard (internal), een directory (ldap) of OAuth
error-account-exists = Er is al een account met die naam
error-own-role = Een andere beheerder kan je rol wijzigen
error-remove-self = Je kunt je eigen account niet verwijderen terwijl je ermee bent ingelogd
error-token-name = Noem het token naar wat het gaat gebruiken, in maximaal 80 tekens
error-token-role = Een token kan niet meer dan het account dat het aanmaakt
error-wrong-setup-code = Dat is niet de installatiecode die de server in HOARD_SETUP_CODE heeft gekregen
error-wrong-password = Die naam en dat wachtwoord horen niet bij elkaar
error-folder-missing = { $path } is er niet (een schijf die niet gekoppeld is?)
error-s3-path = een S3-locatie ziet eruit als s3://bucket
error-s3-address = het S3-adres moet beginnen met http:// of https://
error-s3-header = een extra S3-header ziet eruit als Name=value, en “{ $header }” is dat niet
error-keychain = kon het niet bewaren in de sleutelhanger van het systeem: { $reason }
error-not-built = deze kopie van Hoard is gebouwd zonder { $feature }
error-openai-auth-unreachable = kon auth.openai.com niet bereiken
error-openai-unexpected = onverwacht antwoord van OpenAI
error-openai-answered = OpenAI antwoordde { $status }: { $reason }
error-chatgpt-id-token-not-jwt = het ID-token is geen JWT
error-chatgpt-id-token-issuer = het ID-token komt van { $issuer }, niet van { $expected }
error-chatgpt-id-token-audience = het ID-token is voor een andere app
error-chatgpt-id-token-expired = het ID-token is verlopen
error-chatgpt-id-token-nonce = het ID-token hoort niet bij deze aanmelding
error-chatgpt-no-state = het adres heeft geen state; kopieer het hele adres
error-chatgpt-not-started = deze aanmelding is hier niet gestart, of meer dan 15 minuten geleden. Begin opnieuw vanuit Instellingen.
error-chatgpt-canceled = de aanmelding is geannuleerd
error-chatgpt-refused = ChatGPT weigerde de aanmelding: { $reason }
error-chatgpt-no-code = het adres heeft geen aanmeldcode; kopieer het hele adres
error-chatgpt-other-registration = ChatGPT antwoordde voor een andere registratie van deze server; begin opnieuw met inloggen
error-chatgpt-other-registration-desktop = ChatGPT antwoordde voor een andere kopie van Hoard; begin opnieuw met inloggen
error-chatgpt-not-registered = ChatGPT heeft de registratie van Hoard niet afgerond; begin opnieuw met inloggen
error-chatgpt-code-used = de aanmeldcode is al gebruikt of verlopen; log opnieuw in
error-chatgpt-refused-code = ChatGPT weigerde de aanmelding ({ $code })
error-chatgpt-no-id-token = ChatGPT stuurde geen ID-token
error-chatgpt-plan-not-allowed = Ingelogd, maar het abonnement gebruiken werd niet toegestaan. Log opnieuw in en sta het toe, of gebruik in plaats daarvan een API-sleutel.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = niet ingelogd bij ChatGPT; log in via Instellingen → AI-tagging
error-chatgpt-refresh-later = ChatGPT kon de aanmelding nu niet vernieuwen ({ $code }); het wordt later opnieuw geprobeerd
error-chatgpt-signed-out-problem = ChatGPT heeft deze server uitgelogd ({ $code }). Log opnieuw in om met je abonnement te blijven taggen.
error-chatgpt-signed-out-problem-desktop = ChatGPT heeft Hoard uitgelogd ({ $code }). Log opnieuw in om met je abonnement te blijven taggen.
error-chatgpt-signed-out = ChatGPT heeft deze server uitgelogd ({ $code }); log opnieuw in via Instellingen → AI-tagging
error-chatgpt-signed-out-desktop = ChatGPT heeft Hoard uitgelogd ({ $code }); log opnieuw in via Instellingen → AI-tagging
error-openai-api-unreachable = kon api.openai.com niet bereiken
error-openai-broke-off = het antwoord van OpenAI brak af
error-chatgpt-kept-refusing = OpenAI bleef de ChatGPT-aanmelding weigeren; log opnieuw in via Instellingen → AI-tagging
error-chatgpt-request-failed = het verzoek is mislukt
error-chatgpt-cut-off = het antwoord werd afgekapt ({ $reason })
error-chatgpt-cut-off-no-reason = het antwoord werd afgekapt (geen reden opgegeven)
error-openai-ended-early = het antwoord van OpenAI stopte voordat het compleet was
error-chatgpt-usage-limit = het gebruik van je ChatGPT-abonnement voor Hoard is voorlopig op. Bekijk of wijzig de limiet op { $url }
error-chatgpt-not-eligible = dit ChatGPT-account of deze werkruimte kan het abonnement niet in andere apps gebruiken. Gebruik in plaats daarvan een API-sleutel.
error-chatgpt-usage-unavailable = ChatGPT kon het gebruik van het abonnement nu niet controleren; het wordt later opnieuw geprobeerd
error-chatgpt-unsupported = gebruik via het ChatGPT-abonnement ondersteunt { $param } niet met dit model
error-chatgpt-unsupported-images = gebruik via het ChatGPT-abonnement ondersteunt { $param } niet met dit model; kies een model dat afbeeldingen aankan, of zet het meesturen van miniaturen uit
error-chatgpt-unsupported-part = gebruik via het ChatGPT-abonnement ondersteunt een deel van het verzoek niet met dit model
error-chatgpt-port = port is de poort waarop de browser Hoard bereikt
error-chatgpt-paste-address = plak het hele adres, beginnend met http://127.0.0.1
error-too-many-hashes = maximaal { $max } hashes tegelijk
error-credits-pack = pakket { $key }
error-credits-collection = collectie { $key }
error-format-too-long = het formaat is te lang
error-not-started = de server is niet gestart
error-not-started-desktop = Hoard is niet gestart
error-not-format-file = geen bestand van een formaat: { $name }
error-format-name = de naam heeft een letter of cijfer nodig van A tot Z of 0 tot 9
error-format-miss-name = de bestanden van het formaat omvatten { $file } niet
error-format-miss-store = het formaat neemt { $file } niet mee voor die van de store (zie require)
error-format-miss-title = het formaat vindt geen titel in { $file }
error-cant-download-as = .{ $ext }-bestanden kunnen niet als { $format } worden gedownload
error-only-images-resize = alleen afbeeldingen kunnen van formaat worden veranderd
error-https-no-certificate = Plak of upload eerst je certificaat en de sleutel ervan
error-https-no-cert-found = Geen certificaat gevonden: plak de tekst die begint met -----BEGIN CERTIFICATE-----
error-https-no-key-found = Geen privésleutel gevonden: plak de tekst die begint met -----BEGIN PRIVATE KEY-----
error-https-several-keys = Er is meer dan één privésleutel: plak alleen de sleutel die bij het certificaat hoort
error-https-key-password = De sleutel is beveiligd met een wachtwoord: geef hem zonder wachtwoord op (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Het certificaat is geen geldige PEM
error-https-key-not-pem = De privésleutel is geen geldige PEM
error-https-key-mismatch = De sleutel hoort niet bij het certificaat. Als je een keten hebt opgegeven, zet dan het certificaat van je server vooraan
error-https-unusable = Het certificaat of de sleutel kan niet worden gebruikt
error-https-expired = Dit certificaat is verlopen op { $date }
error-https-not-yet-valid = Dit certificaat is pas geldig vanaf { $date }
error-https-unreadable = Dat is geen certificaat dat Hoard kan lezen
error-https-bad-name = “{ $name }” is geen adres dat in een certificaat kan staan
error-https-test-timed-out = er is een time-out opgetreden bij de testverbinding
error-https-test-failed = de testverbinding is mislukt
error-https-no-ca = Hoard heeft zijn certificeringsinstantie nog niet aangemaakt
error-https-unavailable = HTTPS is niet beschikbaar op deze server
error-https-certificate-mode = Een certificaat hoort bij “Mijn certificaat”
error-https-test = Het certificaat werkte niet in een testverbinding: { $reason }
error-nothing-to-retry = er is daar niets dat opnieuw kan worden geprobeerd
error-ai-unreachable = kon { $url } niet bereiken
error-ai-key-refused = { $url } weigerde de API-sleutel ({ $status })
error-ai-answered = { $url } antwoordde { $status }: { $reason }
error-ai-not-json = { $url }/models antwoordde niet met JSON
error-model-server-refused = de modelserver: { $reason }
error-no-model-server = er antwoordt geen modelserver op { $url }
error-not-llama-router = { $url } is geen llama.cpp-server in routermodus, dus kan hij geen modellen downloaden
error-model-download = kon { $model } niet downloaden: { $reason }
error-model-delete = kon { $model } niet verwijderen: { $reason }
error-unknown-kind = onbekende assetsoort { $kind }
error-watched-is-folder = een bewaakte locatie is een map op deze server
error-cant-reach = kan { $path } niet bereiken: { $reason }
error-folder-full-path = geef het volledige pad van de map op
error-cant-create = kan { $path } niet aanmaken
error-cant-write = kan niet naar { $path } schrijven
error-cant-read = kan { $path } niet lezen
error-overlaps-location = dat overlapt met de locatie { $name } ({ $path })
error-overlaps-data = dat overlapt met de datamap van Hoard ({ $path })
error-overlaps-cache = dat overlapt met de cachemap van Hoard ({ $path })
error-not-watched = { $name } is geen bewaakte map
error-watched-no-kinds = een bewaakte map neemt geen nieuwe bestanden aan, dus er zijn geen soorten om te wijzigen
error-keep-catch-all = maak in plaats daarvan een andere locatie de opvanglocatie
error-location-holds-files = { $name } bevat nog bestanden: verplaats ze eerst naar de andere locaties (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } is niet bereikbaar, dus Hoard kan niet zien of er nog bestanden in staan
error-relocate-running = er worden al bestanden tussen locaties verplaatst; probeer het opnieuw als dat klaar is
error-watched-not-moved = { $name } is een bewaakte map: Hoard verplaatst de bestanden daarin niet
error-no-other-location = er is geen andere bibliotheeklocatie om de bestanden naartoe te verplaatsen: voeg er eerst een toe
error-full-path = geef een volledig pad op
error-pick-folder = kies een map
error-task-running = “{ $task }” loopt al
error-job-cant-stop = “{ $job }” kan niet meer worden gestopt als het eenmaal loopt
error-job-not-found = die taak wacht of loopt niet meer
error-task-cant-stop = “{ $task }” kan niet halverwege worden gestopt
error-task-not-running = “{ $task }” loopt niet
error-token-account-gone = het account van dit token bestaat niet meer, dus het heeft geen favorieten of zoekopdrachten
error-favorites-too-many = maximaal 10.000 tegelijk
error-saved-search-name = een opgeslagen zoekopdracht heeft een naam nodig van maximaal 80 tekens
error-saved-search-too-long = die zoekopdracht is te lang om op te slaan
error-hoard-seen = de schat kan hoogstens 32 soorten assets bijhouden, elk met een naam in kleine letters
error-texture-pattern-map = Het patroon voor textuurnamen heeft { "{" }map{ "}" } nodig, anders krijgt elke map van een materiaal dezelfde naam
error-texture-pattern-fields = Het patroon voor textuurnamen kan { "{" }name{ "}" }, { "{" }map{ "}" } en { "{" }size{ "}" } gebruiken
error-texture-name-chars = Textuurnamen mogen geen < > : " / \ | ? of * bevatten
error-texture-map-type = Er is geen maptype met de naam { $role }
server-role-base-color = kleur
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = ruwheid
server-role-gloss = glans
server-role-metalness = metaal
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = hoogte
server-role-bump = bump
server-role-cavity = cavity
server-role-specular = specular
server-role-opacity = dekking
server-role-emissive = emissie
server-role-translucency = doorschijnendheid
error-texture-suffix-missing = Geef maps voor { $map } een achtervoegsel
error-texture-suffix-chars = Het achtervoegsel voor maps voor { $map } mag geen < > : " / \ | ? of * bevatten
error-texture-ending-missing = Elke eigen uitgang heeft de uitgang nodig waarnaar gezocht wordt
error-texture-ending-chars = Eigen uitgangen mogen geen < > : " / \ | ? of * bevatten
error-texture-ending-twice = { $ending } staat twee keer in je uitgangen
error-oauth-no-client-id = Deze link heeft geen client_id.
error-oauth-unknown-app = Deze app is hier niet (meer) geregistreerd. Begin opnieuw met verbinden vanuit de app.
error-oauth-redirect = De app vroeg om teruggestuurd te worden naar een adres dat hij niet heeft geregistreerd.
error-oauth-response-type = Hoard geeft alleen aanmeldcodes uit (response_type=code).
error-oauth-pkce = De app moet PKCE met S256 gebruiken om in te loggen bij Hoard.
error-light-preview-images = alleen afbeeldingen hebben een licht voorbeeld
error-no-light-preview = geen licht voorbeeld: { $reason }
error-no-frame-sheet = geen framesheet: { $reason }
error-thumbnail-source = onbekende bron voor miniatuur: { $source }
error-thumbnail-too-big = die afbeelding is te groot voor een miniatuur
error-thumbnail-models-only = alleen modellen krijgen een miniatuur die door een browser wordt getekend
error-not-a-picture = geen afbeelding die Hoard kan lezen: { $reason }
error-project-name = een project heeft een naam nodig van maximaal 200 tekens
error-project-too-big = maximaal een miljoen assets in één project
error-project-too-many-at-once = maximaal 100.000 tegelijk
error-server-name = Kies een naam van maximaal { $max } tekens, op één regel
error-tool-setting = { $tool }: kies uit, automatisch, of geef een pad op
error-ai-model = Kies het model dat de AI-dienst moet gebruiken
error-public-address = Het openbare adres moet beginnen met http:// of https://
error-update-hook-address = Het adres dat bij updates wordt aangeroepen moet beginnen met http:// of https://
error-update-hook-auth = De Authorization-header mag geen regeleinden of andere stuurtekens bevatten
error-no-soundfont = Er staat geen SoundFont op { $path } op de server
error-no-soundfont-desktop = Er staat geen SoundFont op { $path }
error-soundfont-bad = { $name } is geen SoundFont waarmee MIDI kan spelen: { $reason }
error-workers = Achtergrondprocessen: tussen 1 en { $max }
error-ai-batch = Assets per AI-verzoek: tussen 1 en 200
error-conversion-cache = Geconverteerde downloads: minstens 100 MB
error-backup-keep = Geplande back-ups: bewaar 1 tot { $max }
error-backup-copy-to = Geplande back-ups: { $path } is geen map op de server (geef het volledige pad)
error-conversion-keep-days = Conversieregels: bewaar originelen 1 tot { $max } dagen
error-conversion-target = Conversieregels: Hoard kan geen bestanden naar { $format } converteren
error-conversion-no-source = Conversieregels: elke regel heeft minstens één formaat nodig om te converteren
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Conversieregels: .{ $ext }-bestanden kunnen niet naar { $format } worden geconverteerd
error-conversion-twice = Conversieregels: .{ $ext } staat in twee regels
error-conversion-cant-undo = Dit bestand is niet door een regel geconverteerd, of het origineel is al verwijderd.
error-conversion-path-taken = Op { $path } staat nu een ander bestand, dus het origineel kan daar niet terug.
error-limits-zero = Limieten kunnen niet nul zijn
error-settings-object = verwachtte een JSON-object met instellingen
error-bad-setting = ongeldige instelling: { $reason }
error-auth-default-role = De standaardrol is view, edit of admin
error-auth-session-hours = Sessies duren tussen 1 uur en een jaar
error-auth-settings = ongeldige authenticatie-instellingen: { $reason }
error-ldap-settings = ongeldige LDAP-instellingen: { $reason }
error-ldap-no-server = Voeg het adres van de directoryserver toe
error-ldap-base-dn = Geef aan waar de gegevens van mensen staan (de base DN)
error-ldap-username-attribute = Geef aan welk attribuut de naam bevat waarmee mensen inloggen
error-ldap-bind-dn = Geef het account op dat mensen opzoekt, of zoek ze anoniem op
error-ldap-ca-cert = De certificeringsinstantie is geen PEM-certificaat (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Het adres van een directoryserver is leeg
error-ldap-server-form = { $server }: het adres van een directoryserver is host:poort, ldap://… of ldaps://…
error-ldap-not-starttls = { $server }: dat is LDAPS, geen StartTLS
error-ldap-no-certificate = er staat geen certificaat in
error-ldap-password-again = Geef het wachtwoord van het account dat mensen opzoekt opnieuw op: het opgeslagen wachtwoord wordt alleen naar de servers gestuurd waarvoor het is opgegeven
error-ldap-unreachable = er kon geen directoryserver worden bereikt ({ $reasons })
error-ldap-no-anonymous = de directory staat anoniem opzoeken van mensen niet toe ({ $reason })
error-ldap-bind-failed = het account dat mensen opzoekt kon niet inloggen ({ $reason })
error-ldap-search = zoeken naar { $name } onder { $base }
error-ldap-group-search = zoeken naar de groepen van { $name } onder { $base }
error-ldap-several = { $count } items in de directory komen overeen met { $name }: maak het gebruikersfilter nauwer
error-ldap-password-check = de directory wilde het wachtwoord van { $name } niet controleren ({ $reason })
error-ldap-wrong-password = De directory kent { $name } niet met dat wachtwoord
error-oauth-client-id = Geef de client-ID op die de provider voor Hoard heeft aangemaakt
error-oauth-addresses = De autorisatie- en tokenadressen moeten beginnen met https://
error-oauth-userinfo-address = Het adres voor gebruikersgegevens moet beginnen met https://
error-oauth-username-claim = Geef aan welke claim de naam van de persoon bevat
error-oauth-secret-again = Geef het clientgeheim opnieuw op: het opgeslagen geheim wordt alleen naar het tokenadres gestuurd waarvoor het is opgegeven
error-oauth-issuer = De issuer moet beginnen met https://
error-oauth-discovery-unreachable = Kon { $url } niet bereiken: { $reason }
error-oauth-discovery-status = { $url } antwoordde { $status }
error-oauth-discovery-bad = { $url } is geen discovery-document
error-oauth-discovery-incomplete = { $url } noemt de autorisatie- en tokenadressen niet
error-signin-no-group = { $name } zit niet in een groep die Hoard mag gebruiken
error-signin-other-source = Er is al een Hoard-account met de naam { $name } dat op een andere manier inlogt
error-signin-other-subject = Er is al een Hoard-account met de naam { $name } voor iemand anders bij de provider
error-signin-no-account = { $name } heeft nog geen Hoard-account: een beheerder kan het toevoegen in Instellingen → Account
error-signin-bad-name = Die naam kan geen Hoard-account zijn (maximaal 64 tekens)
error-oauth-off = Inloggen met OAuth staat niet aan
error-oauth-bad-authorization-address = het autorisatieadres is niet geldig
error-oauth-refused = De provider heeft je niet ingelogd: { $reason }
error-oauth-no-code = De provider stuurde je terug zonder aanmelding
error-oauth-expired = Die aanmelding duurde te lang of is ergens anders gestart: probeer het opnieuw
error-oauth-no-claim = De provider vertelde niet wie je bent (geen claim { $claim })
error-oauth-no-sub = De provider vertelde niet wie je bent (geen claim sub of id)
error-oauth-token-unreachable = kon het tokenadres van de provider niet bereiken
error-oauth-token-refused = De provider wilde de aanmelding niet afronden: { $reason }
error-oauth-no-access-token = de provider stuurde geen toegangstoken
error-oauth-userinfo-unreachable = kon het adres voor gebruikersgegevens van de provider niet bereiken
error-oauth-userinfo-refused = De provider wilde niet zeggen wie er inlogde ({ $status })
error-oauth-userinfo-not-json = de gebruikersgegevens van de provider zijn geen JSON
error-format-no-name = name is leeg
error-format-no-files = files is leeg: geef aan welke bestanden het formaat leest, bijvoorbeeld ["*.json"]
error-format-empty-selector = leeg
error-format-unopened-bracket = een sluithaakje zonder openingshaakje
error-format-unclosed-bracket = een haakje is niet gesloten
error-format-empty-step-dots = een lege stap (twee punten achter elkaar, of een punt aan het begin of eind)
error-format-unclosed-except = !( is niet gesloten
error-format-unclosed-filter = [ is niet gesloten
error-format-after-filters = { $rest } na de sleutel en filters van { $step }
error-format-except-needs-glob = !(…) komt na een sleutel met * erin
error-format-except-not-glob = !(…) komt na een sleutel met * erin, niet na { $key }
error-format-empty-step = een lege stap
error-format-not-a-filter = [{ $filter }] is geen filter: gebruik [], [key=value|other], [array], [string], [number], [bool] of [object]
error-format-filter-needs-value = [{ $filter }] heeft een sleutel en een waarde nodig
error-store-search-empty = niets om naar te zoeken; typ de naam van het pakket
error-update-hook-not-set = stel eerst een adres in om aan te roepen
error-upload-stalled = upload van { $path } loopt vast; stuur dat deel opnieuw
error-upload-busy = { $path } wordt al geschreven
error-upload-incomplete = { $count } bestanden zijn onvolledig, bijvoorbeeld { $files }
error-upload-no-files-listed = geen bestanden
error-upload-not-part = { $path } hoort niet bij upload { $id }, of de upload is gesloten
error-bad-path = ongeldig pad
error-upload-part-offset = { $offset } is niet waar een deel van { $path } begint
error-upload-interrupted = upload onderbroken: { $reason }
error-upload-part-long = deel { $part } van { $path } is langer dan { $len } bytes
error-upload-part-short = deel { $part } van { $path }: { $got } bytes ontvangen, { $len } verwacht
error-upload-file-long = { $path } is groter dan opgegeven
error-upload-file-short = { $path }: { $got } bytes ontvangen, { $len } verwacht
error-upload-complete = de upload is al compleet
error-already-imported = al geïmporteerd als “{ $name }”

## Servertekst

server-store-place-readme = README
server-store-place-folders = mappen
server-store-why-linked = Gelinkt in de { $place } van het pakket
server-store-why-same-title = Zelfde titel als in de { $place } van het pakket
server-store-why-by = Door { $author }, genoemd in de { $place } van het pakket
server-store-why-meshes = Ongeveer evenveel meshes als het pakket ({ $count })
server-store-why-textures = Ongeveer evenveel texturen als het pakket ({ $count })
server-chatgpt-revoke-failed = Hier uitgelogd, maar OpenAI kon het niet worden doorgegeven. Ontkoppel Hoard voor de zekerheid in ChatGPT via Instellingen → Beveiliging en inloggen → Inloggen met ChatGPT.
server-chatgpt-signed-in-title = Ingelogd bij ChatGPT
server-chatgpt-signed-in = Hoard is ingelogd. AI-tagging kan nu je ChatGPT-abonnement gebruiken.
server-chatgpt-signed-in-as = Hoard is ingelogd als { $email }. AI-tagging kan nu je ChatGPT-abonnement gebruiken.
server-chatgpt-not-finished-title = Inloggen niet afgerond
server-chatgpt-other-computer = Als je het inloggen bent begonnen vanuit Hoard op een andere computer, kopieer dan het adres van deze pagina uit de adresbalk en plak het daar in de Instellingen van Hoard.
server-back-to-settings = Terug naar Instellingen
server-issues-pack = Pakketten die niet konden worden toegevoegd
server-issues-pack-blurb = Verwijder het pakket en voeg het opnieuw toe, of open het om te zien wat erin zit.
server-issues-damaged = Opgeslagen bestanden die beschadigd of verdwenen zijn
server-issues-damaged-blurb = Gevonden door “Bestanden op schade controleren”. Zet ze terug uit een back-up of voeg hun pakket opnieuw toe, en voer daarna “Bestanden op schade controleren” opnieuw uit.
server-issues-folder = Bewaakte mappen die niet kunnen worden gelezen
server-issues-folder-blurb = Hun pakketten blijven zoals ze zijn tot de map weer kan worden gelezen.
server-issues-missing = Bewaakte pakketten waarvan de map verdwenen is
server-issues-missing-blurb = Elk pakket wordt 3 dagen bewaard voor het geval de map alleen offline is, en daarna verwijderd.
server-issues-job = Mislukte achtergrondtaken
server-issues-job-blurb = Opnieuw proberen voert de taak opnieuw uit vanaf het begin.
server-issues-file = Bestanden die niet konden worden geopend
server-issues-file-blurb = Hoard kon ze niet openen of opslaan. Een bewaakt pakket probeert ze opnieuw bij de volgende scan; een bestand dat al in de bibliotheek stond, heeft behouden wat het had.
server-issues-unpack = Archieven die niet konden worden uitgepakt
server-issues-unpack-blurb = Elk ervan blijft als gewoon bestand in zijn pakket, dus niets van de inhoud staat in de bibliotheek.
server-issues-read = Bestanden waar Hoard niets van kon maken
server-issues-read-blurb = Ze staan in de bibliotheek, maar zonder de gegevens, tags of miniatuur die het lezen ervan oplevert.
server-issues-render = Modellen waarop de miniatuurrenderer vastliep
server-issues-render-blurb = Ze hebben geen miniatuur. Openen in de viewer kan nog steeds werken, en kan er een opslaan.
server-issues-convert = Modellen die Blender niet kon converteren
server-issues-convert-blurb = Ze hebben geen GLB-kopie, dus ze kunnen niet in 3D worden getoond.
server-issues-unreal = Unreal-packages die niet konden worden geconverteerd
server-issues-unreal-blurb = De Unreal-pagina van het pakket heeft de rest van de packages.
server-issues-reformat = Bestanden die een conversieregel niet kon converteren
server-issues-reformat-blurb = Ze blijven zoals ze waren.
server-issues-materials = Pakketten waarvan de materialen niet konden worden uitgezocht
server-issues-materials-blurb = De texturen zijn er allemaal; ze zijn alleen niet in materialen gegroepeerd.
server-issues-material-thumb = Materialen zonder miniatuur
server-issues-material-thumb-blurb = Het materiaal is gevonden, maar het voorbeeld ervan kon niet worden getekend.
server-issues-store-files = Storebestanden die niet konden worden gelezen
server-issues-store-files-blurb = Bestanden van de store waar een pakket vandaan komt, zoals een Megascans-JSON, die het een naam en tags geven.
server-issues-scan-again = Opnieuw scannen
server-issues-retry = Opnieuw proberen
server-issues-reanalyze = Pakket opnieuw analyseren
server-issues-convert-again = Opnieuw converteren
server-issues-look-again = Opnieuw kijken
server-issues-read-again = Opnieuw lezen
server-issues-blob-missing = Het opgeslagen bestand is verdwenen.
server-issues-blob-damaged = Het opgeslagen bestand komt niet overeen met wat is opgeslagen: het is beschadigd.
server-issues-folder-gone = De map “{ $folder }” is niet gevonden.
server-job-a-pack = een pakket
server-job-ingest = { $pack } verwerken
server-job-retag = { $pack } opnieuw taggen
server-job-llm-tag = { $pack } taggen met AI
server-job-convert = { $pack } converteren met Blender
server-job-unreal = Unreal-packages in { $pack } converteren
server-job-reformat = Bestanden in { $pack } converteren volgens de conversieregels
server-job-scan = Een bewaakte map scannen
server-job-relocate = Bestanden tussen bibliotheeklocaties verplaatsen
server-job-looks = Kleuren en uiterlijk uitzoeken
server-job-sounds = Naar geluiden luisteren
server-job-embed = Leren wat afbeeldingen tonen
server-job-hear = Leren hoe geluiden klinken
server-job-materials = Materialen zoeken in { $pack }
server-job-read-files = De bestanden van { $pack } lezen
server-job-store-lookup = Een storepagina opzoeken
server-maintenance-backup = Back-up van de database maken
server-maintenance-backup-blurb = Een consistente kopie van de database (pakketten, tags, notities, collecties en Instellingen) in de map backups ernaast. De bestanden van de bibliotheek zitten er niet in: die veranderen nooit meer nadat ze zijn opgeslagen, dus elke bestandssynchronisatie maakt er een back-up van.
server-maintenance-scrub = Bestanden op schade controleren
server-maintenance-scrub-blurb = Leest elk opgeslagen bestand terug en vergelijkt het met de vingerafdruk, om bestanden te vinden die zijn aangetast of van een schijf of NAS zijn verdwenen. Duurt even bij een grote bibliotheek.
server-maintenance-reanalyze = Elk pakket opnieuw analyseren
server-maintenance-reanalyze-blurb = Leest elk bestand opnieuw met deze versie van Hoard: miniaturen, voorbeelden, details en regeltags worden opnieuw gemaakt. Je eigen wijzigingen blijven; de AI wordt niet opnieuw gevraagd.
server-maintenance-clear-cache = Gecachete conversies wissen
server-maintenance-clear-cache-blurb = Gooit de bestanden weg die zijn bewaard van conversies via “Downloaden als” (ze worden opnieuw gemaakt als erom wordt gevraagd) en wat onderbroken werk heeft achtergelaten.
server-maintenance-shrink-thumbs = Miniaturen verkleinen
server-maintenance-shrink-thumbs-blurb = Slaat de miniaturen en modelweergaven die als PNG zijn bewaard op zoals Instellingen → Prestaties → Miniaturen aangeeft, wat ze meestal een aantal keer kleiner maakt. Die met transparantie, en pixelart, blijven PNG.
server-maintenance-avif-thumbs = Miniaturen naar AVIF converteren
server-maintenance-avif-thumbs-blurb = Converteert alle miniaturen nu, in plaats van elk zodra hij voor het eerst getoond wordt, als miniaturen als AVIF verstuurd worden.
server-maintenance-gc = Ongebruikte bestanden verwijderen
server-maintenance-gc-blurb = Verwijdert opgeslagen bestanden waar niets meer naar verwijst, zoals de bestanden van definitief verwijderde pakketten.
server-update-no-address = er is geen adres ingesteld om aan te roepen
server-update-no-answer = geen antwoord binnen 30 seconden

## Instellingen: Weergave

appearance-language = Taal
appearance-language-auto = Die van de browser ({ $name })

## Root

root-unreachable = Kon de Hoard-server niet bereiken: { $error }
root-unreachable-desktop = Hoard reageert niet: { $error }
root-desktop-reopen = Dit venster is afgemeld. Sluit Hoard en open het opnieuw om verder te gaan.
root-try-again = Opnieuw proberen
root-lazy-connect = de appverbinding
window-minimize = Minimaliseren
window-maximize = Maximaliseren
window-restore = Vorige grootte
window-close = Sluiten

## App: de bibliotheek, de bovenbalk en filters

app-lazy-settings = de instellingen
app-lazy-archive = het archief
app-lazy-shortcuts = de sneltoetsen
app-lazy-setup = de installatie
app-theme-system = Systeemthema volgen
app-theme-light = Licht thema
app-theme-dark = Donker thema
app-color-red = Rood
app-color-orange = Oranje
app-color-yellow = Geel
app-color-green = Groen
app-color-teal = Blauwgroen
app-color-blue = Blauw
app-color-purple = Paars
app-color-pink = Roze
app-color-brown = Bruin
app-color-sand = Zand
app-color-gray = Grijs
app-color-slate = Leisteen
app-color-black = Zwart
app-color-white = Wit
app-color-swatch = { $color }: assets met veel { $lower } erin
app-color-any = Elke kleur
app-color-pick = Kies een kleur
app-color = Kleur
app-music = Muziek
app-tempo = BPM
app-tempo-min = van
app-tempo-max = tot
app-tempo-min-title = Muziek in dit tempo of sneller, in tellen per minuut (half of dubbel telt ook)
app-tempo-max-title = Muziek in dit tempo of langzamer, in tellen per minuut
app-key = Toonsoort
app-key-any = Elke toonsoort
app-key-major = Majeur
app-key-minor = Mineur
app-key-mix = En toonsoorten die erbij mixen
app-key-mix-title = De parallelle majeur of mineur en de toonsoorten een kwint hoger en lager, die dj's ermee mixen
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Schrijf het geluid zoals je het zou spellen: geluiden die erop lijken komen eerst
app-imitation-record = Maak het geluid in de microfoon: geluiden die erop lijken komen eerst
app-imitation-stop = Opname stoppen
app-imitation-heard = Jouw geluid
app-imitation-mic-failed = Kon niet opnemen met de microfoon: { $error }
app-retry = Opnieuw proberen
app-undo = Ongedaan maken
app-search-failed = Zoeken mislukt: { $error }
app-unreachable = Kon de server niet bereiken: { $error }
app-unreachable-desktop = Hoard reageert niet: { $error }
app-forget-project-title = { $name } vergeten?
app-forget-project-body = Hoard toont het niet meer bij de { $count } assets die het gebruikt, tot de volgende hoard sync het weer doorgeeft. Er verandert niets in het project.
app-forget-project-confirm = Vergeten
app-forget-project-failed = Kon { $name } niet vergeten: { $error }
app-forget-project-label = Het project { $name } vergeten
app-forget-project = Dit project vergeten
app-save-name-like = zoals { $name }
app-save-name-favorites = favorieten
app-search-saved = “{ $name }” opgeslagen: je vindt het in Filters, onder Van jou
app-search-updated = “{ $name }” bijgewerkt
app-rank-alike = Meest gelijkend
app-rank-color = Meeste van de kleur
app-rank-match = Beste overeenkomst
app-rank-imitation = Klinkt het meest zo
app-sort = Sorteren
app-sort-name = Naam
app-sort-created = Gemaakt
app-sort-updated = Bijgewerkt
app-sort-size = Grootte
app-sort-rating = Beoordeling
app-sort-newest-first = Nieuwste eerst
app-sort-oldest-first = Oudste eerst
app-sort-a-to-z = A tot Z
app-sort-z-to-a = Z tot A
app-sort-largest-first = Grootste eerst
app-sort-smallest-first = Kleinste eerst
app-sort-best-rated-first = Best beoordeeld eerst
app-sort-lowest-rated-first = Laagst beoordeeld eerst
app-collection-added =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } toegevoegd aan { $name }
app-collection-add-failed = Kon niet aan de collectie toevoegen: { $error }
app-collection-removed = { $count } verwijderd uit { $name }
app-collection-remove-failed = Kon niet uit de collectie verwijderen: { $error }
app-ns-subject = Onderwerp
app-ns-style = Stijl
app-ns-theme = Thema
app-ns-material = Materiaal
app-ns-map = PBR-map
app-ns-color = Kleur
app-ns-mood = Sfeer
app-ns-genre = Genre
app-ns-license = Licentie
app-ns-res = Resolutie
app-ns-duration = Lengte
app-ns-poly = Aantal polygonen
app-ns-format = Formaat
app-ns-weight = Gewicht
app-ns-tech = Technisch
app-ns-blender = Blender-versie
app-ns-depth = Bitdiepte
app-ns-ucs = Geluidscategorie (UCS)
app-ns-bpm = Tempo
app-ns-root = Grondtoon
app-ns-level = Luidheid
app-ns-ue = Unreal-klasse
app-ns-engine = Engine
app-ns-extracted = Teruggewonnen bron
app-ns-script = Schrift
app-bulk-failed =
    Kon { $count ->
        [one] die asset
       *[other] die { $count } assets
    } niet wijzigen: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] { $count } tag gekopieerd
       *[other] { $count } tags gekopieerd
    }
app-tags-copy-none = { $name } heeft geen tags om te kopiëren
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Geen tags gekopieerd. Druk op C bij een asset om de tags te kopiëren.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [one] Heeft die tags al
       *[other] Alle { $count } hebben die tags al
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [one]
            { $count ->
                [one] { $count } tag toegevoegd
               *[other] { $count } tags toegevoegd
            }
       *[other]
            { $count ->
                [one] { $count } tag toegevoegd aan { $assets } assets
               *[other] { $count } tags toegevoegd aan { $assets } assets
            }
    }
app-bulk-untagged =
    { $tag } verwijderd van { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-bulk-tagged =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } getagd met { $tag }
app-bulk-review = Markeer als nagekeken
app-bulk-review-title = Markeer de geselecteerde assets als nagekeken: regels en AI laten hun tags daarna met rust
app-bulk-unreview = Markeer als niet nagekeken
app-bulk-unreview-title = Haal de markering 'nagekeken' van de geselecteerde assets
app-bulk-reviewed =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } als nagekeken gemarkeerd
app-bulk-unreviewed =
    Markering 'nagekeken' weggehaald bij { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-bulk-accept = Voorgestelde tags behouden
app-bulk-accept-title = Alle tags die regels en AI op de geselecteerde assets hebben gezet als eigen tags behouden
app-bulk-accepted =
    Voorgestelde tags behouden bij { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-credits-failed = Kon de vermeldingen niet maken: { $error }
app-archived-assets =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } gearchiveerd
app-archived-one = { $name } gearchiveerd
app-open-failed = Kon die asset niet openen: { $error }
app-unnamed-pack = pakket
app-unnamed-collection = collectie
app-unnamed-asset = asset
app-home-version = Hoard { $version } — terug naar de bibliotheek
app-home = Terug naar de bibliotheek
app-search-label = Assets zoeken
app-search-placeholder = Zoek in namen, paden, tags
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = rots or steen: een van beide · -mos: weglaten · "droog mos": deze woorden samen
app-views = Weergaven
app-library = Bibliotheek
app-packs = Pakketten
app-packs-working = { $count } bezig
app-update-title = Hoard { $version } is uit (deze server draait { $current }). Haal de nieuwe image of binary op en herstart; de database werkt zichzelf bij.
app-update = Bijwerken
app-theme = Thema
app-sign-in = Inloggen
app-archive-title = Archief: { $contents }
app-archive = Archief
app-settings = Instellingen
app-palette-title = Opdrachtpalet ({ $keys })
app-import-folder = Map importeren
app-add-files = Bestanden toevoegen
app-add-files-short = Toevoegen
app-split-label = Hoe deze map importeren
app-split-before = { "" }
app-split-after = { " " }bevat { $count } mappen of archieven.
app-split-each = Elk als eigen pakket importeren ({ $count })
app-split-one = Als één pakket importeren
app-cancel = Annuleren
app-filters = Filters
app-done = Klaar
app-yours = Van jou
app-favorites = Favorieten
app-needs-review = Nakijken nodig
app-needs-review-title = Alleen assets die nog niemand heeft nagekeken (werkt samen met de andere filters)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Beoordeeld
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } ster of meer
       *[other] { $count } sterren of meer
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Klik nogmaals om alle beoordelingen te tonen
app-forget-search-label = De opgeslagen zoekopdracht { $name } vergeten
app-forget-search = Deze zoekopdracht vergeten
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = De bibliotheek openen met deze zoekopdracht
app-default-search-on-title = De bibliotheek opent met deze zoekopdracht. Klik om dit te stoppen.
app-default-search-label = De bibliotheek openen met { $name }
app-rename-search = Naam wijzigen
app-rename-search-label = Nieuwe naam voor { $name }
app-projects-title = Projecten die vanaf hier hoard sync gebruiken, met wat hun laatste synchronisatie had
app-projects = Projecten
app-project-synced = Gesynchroniseerd { $when }
app-project-synced-by = Gesynchroniseerd { $when } door { $who }
app-collections = Collecties
app-no-filters = Geen filters voor deze zoekopdracht.
app-kind = Soort
app-facets-more = Nog { $count }
app-facets-fewer = minder
app-searching = Zoeken…
app-searching-live = Zoeken
app-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-clear-all = Alles wissen
app-save-name-label = Naam voor deze zoekopdracht
app-save-name-placeholder = Geef een naam
app-save = Opslaan
app-save-search-title = Bewaar deze zoekopdracht in Filters, onder Van jou
app-save-search = Zoekopdracht opslaan
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = “{ $name }” bijwerken
app-update-search-title = Bewaar wat nu is gekozen in deze opgeslagen zoekopdracht
app-save-search-new = Opslaan als nieuwe
app-select-title = Kies meerdere assets om ze samen te taggen, te categoriseren, in een collectie te zetten, te downloaden of te archiveren (X, of Ctrl-klik)
app-select = Selecteren
app-tile-size = Tegelgrootte
app-supporting-title = glTF-buffers, materiaalbibliotheken, de maps van materialen, materialen die maar één model draagt en de storebestanden die modellen beschrijven. Ze gaan mee als je downloadt wat ze gebruikt. De pagina’s van een spritesheet, en de sprites die het pakket ook als losse afbeeldingen heeft, worden bij de spritesheet getoond. Een asset die het pakket in meerdere formaten heeft, wordt één keer getoond, in het formaat dat Hoard het best leest.
app-supporting-hidden =
    { $count } ondersteunende { $count ->
        [one] bestand
       *[other] bestanden
    } verborgen
app-supporting-show = Tonen
app-supporting-shown = Inclusief ondersteunende bestanden
app-supporting-hide = Verbergen
app-folders = Mappen
app-folders-title = Het pakket bekijken zoals het op de schijf stond, map voor map
app-folders-label = Mappen van het pakket
app-welcome-title = Zet een assetpakket ergens op deze pagina neer
app-welcome-before = Zips, tarballs, mappen of losse bestanden werken allemaal. Hoard pakt ze uit, slaat elk bestand één keer op aan de hand van de hash, leest afmetingen, lengtes en aantallen polygonen, en tagt alles zodat je kunt zoeken naar dingen als{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }of{ " " }
app-welcome-after = .
app-welcome-import-folder = Een map importeren
app-welcome-choose-files = Bestanden kiezen
app-none-similar = Er lijkt nog niets anders veel op { $name }.
app-none = Niets gevonden.
app-none-query = Niets gevonden voor “{ $query }”.
app-none-imitation = Nog geen gemeten geluid klinkt echt als “{ $word }”.
app-none-heard = Nog geen gemeten geluid klinkt echt als jouw geluid.
app-none-filters = Niets gevonden met deze filters.
app-none-query-filters = Niets gevonden voor “{ $query }” met deze filters.
app-clear-search-filters = Zoekopdracht en filters wissen
app-loading-more = Meer laden
app-keys-hint = Pijltjestoetsen verplaatsen, Enter opent, Spatie speelt een geluid af, X of Ctrl-klik selecteert meerdere (lang indrukken op touchscreens).
app-resize-label = Grootte van het detailpaneel wijzigen
app-resize-title = Sleep om de grootte te wijzigen · dubbelklik voor de passende breedte
app-selected-label = Geselecteerde assets
app-selected = { $count } geselecteerd
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Bewerken…
app-edit-selected-title = De tags en velden van alle wijzigen (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] { $count } resultaat geselecteerd
       *[other] Alle { $count } resultaten geselecteerd
    }
app-pick-assets = Kies assets
app-download-zip = Zip downloaden
app-credits-title = CREDITS.md voor de pakketten waar deze uit komen: wie ze heeft gemaakt, hun licenties en de vermeldingen die ze vragen
app-credits = Vermeldingen
app-prefer-sources-title = Unreal-SoundWaves en -texturen waarvan Hoard het originele bestand heeft teruggewonnen
app-prefer-sources = Teruggewonnen WAV/PNG in plaats van .uasset ({ $count })
bag-clear = Selectie wissen
bag-glb-heading = Modellen als GLB
bag-glb-text = Elk model als .glb-bestand, rechtop met Y omhoog en naar voren gericht, met zijn textures als losse bestanden in een map textures die de modellen delen. Al het andere gaat erin zoals het is.
bag-glb-download = GLB-zip downloaden
bag-plain-heading = Bestanden zoals ze zijn
bag-plain-text = Elk bestand zoals het geïmporteerd is, met wat het nodig heeft, in een map per pakket.
bag-remove = { $name } uit de selectie halen
bag-more = en nog { $count }
app-bulk-tag-label = Een tag om toe te voegen aan of te verwijderen van de geselecteerde assets
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Deze tag toevoegen aan elke geselecteerde asset
app-bulk-tag-add = Toevoegen
app-bulk-tag-remove-title = Deze tag verwijderen van elke geselecteerde asset
app-bulk-tag-remove = Verwijderen
app-remove-from-collection = Verwijderen uit { $name }
app-unfavorite = Uit favorieten
app-favorite = Favoriet
app-select-all = Alle { $count } selecteren
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] Het { $count } resultaat selecteren
       *[other] Alle { $count } resultaten selecteren
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] { $count } asset bewerken
       *[other] { $count } assets bewerken
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] { $count } resultaat bewerken
       *[other] Alle { $count } resultaten bewerken
    }
bulk-edit-close = Sluiten
bulk-edit-tags = Tags
bulk-edit-tag-add = Een tag aan alle toevoegen
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } van { $count }
bulk-edit-untick = Alles uitvinken
bulk-edit-untick-title = Alle tags verwijderen en daarna de tags aanvinken die moeten blijven
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = De meest gebruikte tags daarvan
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Gemengd, blijft ongewijzigd
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Blijft ongewijzigd
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Bij alle leeggemaakt
bulk-edit-reset = Terugzetten zoals het was
bulk-edit-all-note = Dit wijzigt alle resultaten, ook die nog niet geladen zijn, en kan niet in één keer ongedaan worden gemaakt.
bulk-edit-save =
    { $count ->
        [one] { $count } wijziging opslaan
       *[other] { $count } wijzigingen opslaan
    }
bulk-edit-nothing = Niets gewijzigd
bulk-edit-saved =
    { $count ->
        [one] { $count } asset bijgewerkt
       *[other] { $count } assets bijgewerkt
    }
app-archive-selected-title = Niet meer in zoekresultaten tot ze worden teruggezet of definitief verwijderd, vanuit het Archief
app-drop-items = Zet { $count } items neer om ze toe te voegen
app-drop = Zet neer om aan de hoard toe te voegen
app-drop-hint = Elk bestand of elke map op het hoogste niveau wordt een eigen pakket. Gigabytes zijn geen probleem; grote bestanden gaan verder als de verbinding wegvalt.

## Opdrachtpalet (CommandPalette.vue, met de opdrachten uit App.vue)

palette-group-go-to = Ga naar
palette-group-actions = Acties
palette-group-theme = Thema
palette-group-kinds = Soorten
palette-group-packs = Pakketten
palette-group-collections = Collecties
palette-group-projects = Projecten
palette-group-saved = Opgeslagen zoekopdrachten
palette-group-assets = Assets
palette-hint-current = huidig
palette-library = Bibliotheek
palette-library-keywords = assets raster start home grid
palette-favorites = Favorieten
palette-favorites-keywords = ster sterren leuk starred liked
palette-packs = Pakketten
palette-archive = Archief
palette-archive-keywords = prullenbak verwijderd terugzetten herstellen trash deleted restore
palette-stats = Bibliotheekstatistieken
palette-stats-keywords = statistieken cijfers grootte schijfruimte aantal statistics size
palette-copies = Bijna-kopieën
palette-copies-keywords = duplicaten dezelfde vergelijkbaar verkleind geconverteerd verspilde ruimte duplicates same similar resized converted wasted space dedupe
palette-search = Zoeken
palette-add-files = Bestanden toevoegen…
palette-add-files-keywords = uploaden importeren upload import
palette-import-folder = Een map importeren…
palette-import-folder-keywords = uploaden pakket upload pack
palette-select-all = Alle { $count } getoonde selecteren
palette-select-all-keywords = aanvinken check
palette-select-several = Meerdere selecteren…
palette-select-several-keywords = bulk meerdere taggen archiveren tag archive
palette-archive-selected = De { $count } geselecteerde archiveren
palette-archive-selected-keywords = verwijderen weghalen prullenbak delete remove
palette-edit-selected = De { $count } geselecteerde bewerken
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = bulkwijziging tags categorie licentie beoordeling
palette-paste-tags =
    { $count ->
        [one] De gekopieerde tags plakken op de geselecteerde asset
       *[other] De gekopieerde tags plakken op de { $count } geselecteerde
    }
palette-clear-selection = Selectie wissen
palette-clear-filters = Filters wissen
palette-clear-filters-keywords = resetten herstellen reset
palette-hide-supporting = Ondersteunende bestanden verbergen
palette-show-supporting = Ondersteunende bestanden tonen
palette-supporting-keywords = normal maps roughness maskers masks
palette-account = Account en toegangstokens
palette-account-keywords = wachtwoord inloggen gebruiker token api password login
palette-sign-out = Uitloggen ({ $name })
palette-sign-out-keywords = afmelden uitloggen logout
palette-sign-in = Inloggen
palette-sign-in-keywords = aanmelden inloggen account wachtwoord login
palette-settings = Instellingen
palette-settings-keywords = blender ffmpeg llm ai tagging unreal beheerder hulpprogramma's voorkeuren admin tools
palette-shortcuts = Sneltoetsen
palette-shortcuts-keywords = hulp toetsen help keys
palette-theme-keywords = thema weergave modus theme
palette-theme-variant-keywords = weergave kleur color
palette-light-theme = Licht thema: { $name }
palette-dark-theme = Donker thema: { $name }
palette-kind-all = Alle soorten (niet meer alleen { $kind } tonen)
palette-kind-only = Alleen { $kind }
palette-title = Opdrachtpalet
palette-placeholder = Zoek assets, ga naar een pakket, voer een opdracht uit…
palette-results = Resultaten
palette-searching = Zoeken…
palette-no-match = Niets gevonden voor “{ $query }”
palette-foot-move = verplaatsen
palette-foot-run = uitvoeren
palette-foot-shortcuts = alle sneltoetsen

## Sneltoetsen (ShortcutsDialog.vue)

shortcuts-title = Sneltoetsen
shortcuts-close = Sluiten
shortcuts-anywhere = Overal
shortcuts-palette = Opdrachtpalet: zoeken, springen, uitvoeren
shortcuts-search = Zoeken
shortcuts-this-list = Deze lijst
shortcuts-escape = Sluiten, de selectie wissen of de details sluiten
shortcuts-library = Bibliotheek
shortcuts-move = Tussen tegels bewegen (of J en K)
shortcuts-open = De details openen
shortcuts-play-sound = Een geluid afspelen of stoppen
shortcuts-select = Selecteren of deselecteren
shortcuts-favorite = Aan je favorieten toevoegen of eruit halen
shortcuts-reviewed = Markeren als nagekeken of niet
shortcuts-tag = Een tag toevoegen
shortcuts-edit = Selectie bewerken
shortcuts-copy-tags = De tags kopiëren
shortcuts-paste-tags = Gekopieerde tags plakken op de selectie
shortcuts-key-click = klik
shortcuts-select-range = Een reeks selecteren
shortcuts-select-all = Alles wat getoond wordt selecteren
shortcuts-sound-player = Geluidsspeler
shortcuts-play-pause = Afspelen of pauzeren
shortcuts-skip = Terug- of vooruitspoelen
shortcuts-start-end = Begin of einde
shortcuts-image-inspector = Afbeeldingsinspector
shortcuts-zoom = In- of uitzoomen
shortcuts-fit = Aan venster aanpassen
shortcuts-actual-pixels = Werkelijke pixels
shortcuts-tile = 3×3 tegelen om de naden te controleren
shortcuts-background = Volgende achtergrond
shortcuts-channel = Eén kanaal tonen

## KindChips.vue

kinds-group = Assetsoorten

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = toegevoegd door een persoon
tag-source-rule = van de offline tagger
tag-source-meta = gemeten uit het bestand
tag-source-ai = voorgesteld door AI
tag-remove = Tag { $tag } verwijderen
tag-accept = Tag { $tag } behouden
tag-accept-title = Deze suggestie als je eigen tag behouden
tag-reject-title = Verwijderen; de tagger zet hem niet terug
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Nieuwe tag
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Bedoelde je
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = was { $alias }

## ActivityIndicator.vue

activity-label-idle = Achtergrondactiviteit: inactief
activity-label-running = Achtergrondactiviteit: { $running } bezig
activity-label-queued = Achtergrondactiviteit: { $running } bezig, { $queued } in de wachtrij
activity-title = Achtergrondactiviteit
activity-unreachable = Kan de server niet bereiken
activity-unreachable-desktop = Hoard reageert niet
activity-empty = Er loopt niets. Imports, taggen, conversies en zipdownloads verschijnen hier terwijl ze bezig zijn.
activity-queued = in de wachtrij
activity-stop = “{ $what }” stoppen
activity-stop-tagging = Alle AI-tagging stoppen ({ $count })
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } u { $min } min
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } driehoeken
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = { $name } afspelen
tile-pause = { $name } pauzeren
tile-favorite = { $name } als favoriet markeren
tile-in-favorites = In je favorieten
tile-add-favorite = Aan je favorieten toevoegen (F)
tile-more-like = Meer zoals { $name }
tile-more-like-this = Meer zoals dit
tile-select = { $name } selecteren

## CollectionPicker.vue

collections-add = Aan collectie toevoegen
collections-menu = Collecties
collections-find-label = Een collectie zoeken of aanmaken
collections-find-placeholder = Zoek of geef een nieuwe collectie een naam
collections-create = Aanmaken
collections-empty = Nog geen collecties. Typ een naam om er een te beginnen.

## FolderPicker.vue

folders-list = Mappen
folders-close = Sluiten
folders-browse = Bladeren…
folders-up-title = Eén map omhoog
folders-up = Omhoog
folders-this-computer = Deze computer
folders-none = Geen mappen hierin.

## LocationForm.vue

locations-mode-question = Wat moet Hoard met deze map doen?
locations-mode-managed = Claimen: Hoard bewaart mijn pakketten hier
locations-mode-managed-hint = Pakketten die je uploadt of importeert, worden hierheen gekopieerd, en Hoard houdt ze op orde. De map is nu van Hoard, dus laat de inhoud met rust.
locations-mode-watched = In de gaten houden: mijn pakketten staan hier al
locations-mode-watched-hint = Elke map erin wordt een pakket. Hoard leest de bestanden waar ze staan en wijzigt, verplaatst of verwijdert ze nooit, en pikt nieuwe mappen op zodra ze verschijnen.
locations-folder-desktop = Map
locations-folder-placeholder = De map waarin je pakketten staan
locations-folder-hint-managed = Hoard maakt hem aan als hij er niet is. In Docker is dit een map die je in de container hebt gekoppeld.
locations-folder-hint-managed-desktop = Hoard maakt hem aan als hij er niet is.
locations-folder-hint-watched = Koppel de map in Docker eerst in de container (alleen-lezen is prima) en kies hem dan hier.
locations-folder-hint-watched-desktop = Hoard leest hem alleen: er wordt niets in gewijzigd.
locations-name = Naam
locations-name-placeholder-managed = Bibliotheek
locations-name-placeholder-watched = Mijn pakketten
locations-files = Bestanden die hierheen gaan
locations-catch-all = Ook alles wat geen andere locatie aanneemt
locations-kinds-hint = De bestanden van een nieuw pakket gaan naar de locatie waar hun soort is aangevinkt. De rest gaat naar de opvanglocatie.
locations-files-all = Voorlopig allemaal. Voeg later meer locaties toe via Instellingen om sommige soorten (muziek bijvoorbeeld) naar een andere schijf te sturen.
locations-first-watched-note = Voor uploaden is een bibliotheek nodig die Hoard beheert. Je kunt er later een toevoegen via Instellingen → Bibliotheeklocaties.
locations-checking = De map controleren…
locations-add = Locatie toevoegen
locations-cancel = Annuleren

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Kon de bibliotheeklocaties niet laden
locations-takes-and-rest = { $kinds }, en al het andere
locations-takes-rest = Alles wat de andere niet aannemen
locations-takes-everything = Alles
locations-takes-nothing = Nog niets
locations-save-failed = Kon de locatie niet opslaan
locations-unwatch-title = Stoppen met het bewaken van { $name }?
locations-unwatch-body =
    { $count ->
        [one] Het { $count } pakket verdwijnt
       *[other] De { $count } pakketten verdwijnen
    } uit de hoard, met hun tags en notities. De bestanden in de map worden niet aangeraakt.
locations-unwatch-body-empty = De bestanden in de map worden niet aangeraakt.
locations-unwatch = Niet meer bewaken
locations-remove-title = { $name } verwijderen?
locations-remove-body = Alle bestanden erin worden eerst per soort naar de andere locaties verplaatst; de locatie verdwijnt zodra hij leeg is. Er gaat niets verloren.
locations-remove-body-only = Als er bestanden in staan, voeg dan eerst een andere locatie toe waar ze naartoe kunnen.
locations-remove = Verwijderen
locations-removed = { $name } verwijderd
locations-emptying = De bestanden worden uit { $name } verplaatst; de locatie wordt verwijderd als dat klaar is
locations-offline-title = { $name } is niet bereikbaar
locations-offline-body = De bestanden kunnen er niet uit worden verplaatst zolang hij offline is. Toch verwijderen? Bestanden die er nog op staan, worden als ontbrekend getoond.
locations-remove-anyway = Toch verwijderen
locations-remove-failed = Kon { $name } niet verwijderen
locations-rebalancing = Opgeslagen bestanden worden verplaatst naar de locaties die hun soort aannemen
locations-rebalance-failed = Kon de bestanden niet verplaatsen
locations-scanned-never = nog niet gescand
locations-scanned-now = zojuist gescand
locations-scanned-minutes = { $mins } min geleden gescand
locations-scanned-hours = { $hours } u geleden gescand
locations-scanned-date = gescand op { $date }
locations-scan-failed = Kon { $name } niet scannen
locations-watching = { $name } wordt bewaakt
locations-added = { $name } toegevoegd
locations-loading = Laden…
locations-managed-title = Bibliotheken die Hoard beheert
locations-watched-title = Bewaakte mappen
locations-managed-hint = Uploads en imports worden hierheen gekopieerd. Elke locatie neemt de soorten aan die ze aanvinkt; de opvanglocatie neemt de rest.
locations-watched-hint = Mappen met pakketten die worden geïndexeerd waar ze staan. Hoard wijzigt de bestanden erin nooit, en zoekt elke 15 minuten naar nieuwe en verwijderde mappen.
locations-managed-empty = Nog geen, dus er is geen plek voor een upload. Voeg er hieronder een toe.
locations-reachable = Bereikbaar
locations-unreachable-now = Nu niet bereikbaar
locations-catch-all-tag = opvanglocatie
locations-packs =
    { $count ->
        [one] { $count } pakket
       *[other] { $count } pakketten
    }
locations-unreachable = niet bereikbaar
locations-scanning = Scannen…
locations-scan = Nu scannen
locations-edit = Bewerken
locations-catch-all-locked = Maak een andere locatie de opvanglocatie om dit te wijzigen
locations-edit-hint-before = Geldt voor bestanden die vanaf nu worden toegevoegd. Om wat al is opgeslagen te verplaatsen, gebruik je
locations-edit-hint-after = hieronder.
locations-save = Opslaan
locations-rebalance = Opgeslagen bestanden mee verplaatsen
locations-rebalance-hint = Na het wijzigen van wat een locatie aanneemt: bestanden die al zijn opgeslagen, gaan naar de locatie waar hun soort nu naartoe gaat.
locations-add-title = Een locatie toevoegen
locations-add-hint = Een andere schijf voor sommige soorten bestanden, of een map met pakketten die je al hebt.
locations-kind-question = Wat voor locatie is het?
locations-kind-folder = Een map
locations-kind-folder-hint = Op een schijf of NAS-share die in de container is gekoppeld, of een map die Dropbox, Google Drive, OneDrive of iCloud Drive synchroniseert. Hoard kan er bestanden bewaren, of pakketten in de gaten houden die je al hebt.
locations-kind-folder-hint-desktop = Op deze computer, een andere schijf of een NAS-share, of een map die Dropbox, Google Drive, OneDrive of iCloud Drive synchroniseert. Hoard kan er bestanden bewaren, of pakketten in de gaten houden die je al hebt.
locations-kind-s3-hint = Een bucket in Amazon S3 of een vergelijkbare dienst: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard bewaart er bestanden.
locations-s3-title = S3-opslag
locations-s3-hint = Een locatie op s3://bucket-name is een bucket in deze dienst: AWS, of een andere S3-compatibele dienst (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage op een NAS).
locations-bucket = Bucket
locations-bucket-hint = De naam van een bucket die al bestaat. De sleutels hierboven moeten hem kunnen opsommen, lezen en beschrijven.
locations-s3-endpoint = Adres
locations-s3-endpoint-placeholder = Leeg voor AWS; bijv. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Regio
locations-s3-access-key = Toegangssleutel-ID
locations-s3-secret-key = Geheime toegangssleutel
locations-s3-headers = Extra headers
locations-s3-headers-placeholder = Name=value, één per regel (bijvoorbeeld voor Cloudflare Access)
locations-s3-kept = Opgeslagen (typ om te vervangen)
locations-s3-secrets = De sleutels en headers worden bewaard bij de instellingen van de server en nooit meer getoond.
locations-s3-secrets-desktop = De sleutels en headers worden bewaard in de sleutelhanger van deze computer en nooit meer getoond.
locations-s3-saved = Instellingen voor S3-opslag opgeslagen
locations-s3-save-failed = Kon de instellingen voor S3-opslag niet opslaan
locations-add-more = Een locatie toevoegen…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Kon Problemen niet laden
issues-started =
    { $count ->
        [one] Opnieuw gestart.
       *[other] { $count } dingen opnieuw gestart.
    }
issues-retry-failed = Kon niet opnieuw proberen
issues-dismiss-failed = Kon niet negeren
issues-dismiss-all-title = Alle { $count } negeren?
issues-dismiss-all-body = “{ $title }” wordt gewist. Alles wat opnieuw misgaat, wordt opnieuw vermeld.
issues-dismiss-all = Alles negeren
issues-just-now = zojuist
issues-minutes-ago = { $mins } min geleden
issues-hours-ago = { $hours } u geleden
issues-loading = Laden…
issues-none = Er is niets misgegaan.
issues-action-all = { $action } (alle)
issues-open-asset = Openen
issues-open-pack = Pakket tonen
issues-open-locations = Bibliotheeklocaties
issues-where-before = in
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } keer
       *[other] { $count } keer
    }
issues-starting = Starten…
issues-dismiss-named = { $title } negeren
issues-dismiss = Negeren
issues-show-fewer = Minder tonen
issues-show-all = Alle { $count } tonen
issues-not-listed = Nog { $count } worden niet getoond.

## archive.ts

archive-anyway = Toch archiveren
archive-failed = Kon niet archiveren: { $error }
archive-undo = Ongedaan maken
archive-restore-failed = Kon niet terugzetten: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pakket
       *[other] { $packs } pakketten
    } en { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pakket
       *[other] { $packs } pakketten
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-nothing = niets

## ArchiveView.vue

archive-load-failed = Kon het archief niet laden: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = { $what } teruggezet
archive-purged = { $what } definitief verwijderd
archive-purge-failed = Kon niet verwijderen: { $error }
archive-purge-title = { $name } definitief verwijderen?
archive-purge-pack-body =
    { $count ->
        [one] De { $count } asset
       *[other] De { $count } assets
    } en { $size } aan bestanden verdwijnen ook, behalve bestanden die een ander pakket deelt. Dit kan niet ongedaan worden gemaakt.
archive-purge = Definitief verwijderen
archive-purge-asset-body = Het bestand wordt vrijgegeven, tenzij een andere asset het deelt. Dit kan niet ongedaan worden gemaakt.
archive-empty-title = Het archief leegmaken?
archive-empty-body = Hiermee worden { $what } definitief verwijderd en hun bestanden vrijgegeven. Dit kan niet ongedaan worden gemaakt.
archive-empty = Archief leegmaken
archive-title = Archief
archive-intro = Gearchiveerde pakketten en assets staan niet in zoekresultaten, de tellingen en de taglijst. Zet ze terug, of verwijder ze definitief om hun ruimte vrij te maken.
archive-loading = Laden…
archive-is-empty = Het archief is leeg
archive-is-empty-hint = Archiveer een pakket of een asset en het wacht hier tot je het terugzet of definitief verwijdert.
archive-packs = Pakketten
archive-assets = Assets
archive-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
archive-archived = { $when } gearchiveerd
archive-from = uit { $pack }
archive-restore = Terugzetten
archive-show-more = Meer tonen (nog { $count })

## mine.ts

projects-favorite-add-failed = Kon niet aan je favorieten toevoegen: { $error }
projects-favorite-remove-failed = Kon niet uit je favorieten halen: { $error }
projects-save-search-failed = Kon de zoekopdracht niet opslaan: { $error }
projects-forget-search-failed = Kon { $name } niet verwijderen: { $error }
projects-change-search-failed = Kon { $name } niet wijzigen: { $error }

## projects.ts

projects-list-and = { $items } en { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = nog { $count }
projects-other =
    { $count ->
        [one] { $count } ander project
       *[other] { $count } andere projecten
    }
projects-usage-title-one = { $name } gebruikt dit
projects-usage-title = { $count } projecten gebruiken dit
projects-usage-body-one = Gebruikt in { $list }: de volgende hoard sync haalt ze eruit.
projects-usage-body = Gebruikt in { $list }: bij elk ervan haalt de volgende hoard sync ze eruit.

## uploader.ts

upload-network-error = netwerkfout
upload-gave-up = { $what }: { $error } na { $attempts } pogingen
upload-failed-plain = mislukt
upload-already-imported = al geïmporteerd
upload-part-failed = upload mislukt ({ $status })
upload-batch = reeks van { $count } bestanden
upload-batch-failed = reeks mislukt ({ $status })
upload-finish = upload afronden

## UploadQueue.vue

upload-eta-seconds = nog { $s } s
upload-eta-minutes = nog { $min } min
upload-eta-hours = nog { $h } u
upload-queue = Imports
upload-importing = { $count } pakketten importeren
upload-bytes-of = { $sent } van { $total }
upload-rate = { $size }/s
upload-waiting = Wachten
upload-unpacking = Geüpload, nu uitpakken
upload-already = Al in de bibliotheek
upload-stopped = Gestopt
upload-files-of = { $done } van { $total } bestanden
upload-resumed = hervat, { $size } was er al
upload-skipped = Precies deze bestanden waren al geïmporteerd als “{ $name }”, dus ze zijn overgeslagen.
upload-error = { $error }. Je voortgang blijft bewaard: probeer het opnieuw, of zet later dezelfde bestanden nog eens neer.
upload-retry = Opnieuw proberen
# $name is the pack the library has already that these files go into.
upload-update = “{ $name }” bijwerken
upload-copy = Als kopie importeren
# $name is the pack the library has already that the upload goes into.
upload-into = Gaat naar “{ $name }”, dat al in de bibliotheek staat: nieuwe bestanden worden toegevoegd en gewijzigde vervangen.
upload-own-pack = Er een eigen pakket van maken
upload-dismiss = Sluiten
upload-cancel = Annuleren
upload-minimize = Minimaliseren
upload-expand = Imports tonen
upload-pill = { $count } worden geïmporteerd · { $percent }%
upload-pill-stopped = Gestopt: { $count }
upload-interrupted = Onderbroken
upload-resume = Hervatten
upload-resume-all = Alles hervatten
upload-resume-hint-folder = Kies dezelfde map opnieuw, of zet hem neer op de pagina, om verder te gaan waar hij stopte.
upload-resume-hint-file = Kies hetzelfde bestand opnieuw, of zet het neer op de pagina, om verder te gaan waar het stopte.
upload-not-same = Dat zijn niet dezelfde bestanden, dus er is niets hervat.
upload-permission-denied = De browser mocht de bestanden niet opnieuw lezen. Kies ze in plaats daarvan.

## PacksView.vue

packs-rename-failed = Kon de naam niet wijzigen: { $error }
packs-collection-deleted = De collectie { $name } is verwijderd
packs-delete-failed = Kon niet verwijderen: { $error }
packs-status-uploading = Uploaden (tot nu toe ontvangen bestanden)
packs-status-ingesting = Uitpakken en taggen
packs-status-analyzing = LLM is aan het taggen
packs-status-ready = Klaar
packs-status-failed = Mislukt
packs-retag-failed = Kon opnieuw taggen niet starten: { $error }
packs-read-files-failed = Kon de bestanden van het pakket niet lezen: { $error }
packs-context-load-failed = Kon de storegegevens van het pakket niet laden: { $error }
packs-undo-store-failed = Kon de storepagina niet loskoppelen: { $error }
packs-context-save-failed = Kon de storegegevens niet opslaan: { $error }
packs-unreal-load-failed = Kon de status van de Unreal-conversie niet laden: { $error }
packs-unreal-discard-title = De originelen weggooien?
packs-unreal-discard-body = De originele .uasset-bestanden in { $name } verdwijnen zodra ze zijn geconverteerd. Gegevens die alleen Unreal kent, zoals materiaalgrafen, gaan definitief verloren.
packs-unreal-discard-confirm = Converteren en weggooien
packs-unreal-convert-failed = Kon de conversie niet starten: { $error }
packs-unreal-fact-converted = { $done } geconverteerd vanuit Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal-package
       *[other] { $left } Unreal-packages
    }
packs-unreal-fact-both = { $done } geconverteerd vanuit Unreal, { $left } nog packages
packs-thumbs-canceled = Geannuleerd na { $done } van { $total }
packs-thumbs-drew =
    { $count ->
        [one] { $count } model getekend
       *[other] { $count } modellen getekend
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniatuur vervangen.
       *[other] { $lead }: { $improved } miniaturen vervangen.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniatuur vervangen, { $failed } konden niet worden getekend.
       *[other] { $lead }: { $improved } miniaturen vervangen, { $failed } konden niet worden getekend.
    }
packs-archived = { $name } en de { $count } assets ervan gearchiveerd
packs-blobs-bad = { $count } bestanden kwamen niet door hun laatste integriteitscontrole.
packs-collections = Collecties
packs-browse-named = { $name } bekijken
packs-collection-name = Naam van de collectie
packs-save = Opslaan
packs-cancel = Annuleren
packs-assets =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
packs-updated = Bijgewerkt op { $date }
packs-browse = Bekijken
packs-collection-download = Downloaden (.zip)
packs-collection-credits = Vermeldingen
packs-collection-credits-title = Wie de pakketten heeft gemaakt waar deze assets uit komen, hun licenties en de vermeldingen die ze vragen
packs-rename = Naam wijzigen
packs-delete = Verwijderen
packs-packs = Pakketten
packs-empty = Nog geen pakketten. Zet ergens op deze pagina een zip of map neer om er een toe te voegen.
packs-find = Een pakket zoeken
packs-find-placeholder = Zoek een pakket op naam, store, licentie of tag
packs-sort = Volgorde
packs-sort-new = Nieuwste eerst
packs-sort-old = Oudste eerst
packs-sort-size = Grootste eerst
packs-sort-assets = Meeste assets eerst
packs-sort-count = Aantal assets
packs-sort-smallest = Kleinste eerst
packs-sort-fewest = Minste assets eerst
packs-with-kind = Pakketten met
packs-state = Status
packs-state-any = Elke status
packs-state-working = Bezig
packs-state-failed = Mislukt
packs-state-untagged = Nog niet beschreven
packs-state-unlicensed = Geen licentie vermeld
packs-store = Store
packs-store-any = Elke store
packs-store-from = Van { $store }
packs-store-none = Geen storepagina
packs-shown-of = { $shown } van { $total }
packs-clear = Wissen
packs-no-match = Geen pakketten gevonden.
packs-clear-all = Zoekopdracht en filters wissen
packs-pack-name = Naam van het pakket
packs-thumbs-drawing = Miniaturen tekenen in deze browser…
packs-thumbs-drawing-count = Miniaturen tekenen in deze browser: { $done } van { $total }…
packs-thumbs-stopped = Gestopt: { $error }
packs-thumbs-none = Elk model hier heeft al een miniatuur die met al zijn texturen is getekend.
packs-dismiss = Sluiten
packs-license = Licentie: { $license }
packs-license-tier = Licentie: { $license } ({ $tier })
packs-license-not-found = niet gevonden
packs-from-before = Van
packs-from-after = { "" }
packs-added = Toegevoegd op { $date }
pack-panel-label = Over dit pakket
pack-panel-hide = Details van het pakket verbergen
pack-panel-kinds = Wat erin zit
pack-panel-all = Alle pakketten
packs-download = Pakket downloaden (.zip)
packs-store-details = Storegegevens
packs-more-named = Meer acties voor { $name }
packs-more = Meer
packs-rename-more = Naam wijzigen…
packs-retag-rules = Offline tagger opnieuw uitvoeren
packs-retag-llm = LLM-tagger opnieuw uitvoeren
packs-make-thumbs = Miniaturen maken in deze browser
packs-make-thumbs-title = Open elk model één voor één onzichtbaar in een 3D-viewer en gebruik de afbeelding als miniatuur waar die beter is dan die van de server
packs-make-thumbs-title-desktop = Open elk model één voor één onzichtbaar in een 3D-viewer en gebruik de afbeelding als miniatuur waar die beter is dan die Hoard heeft gemaakt
packs-read-files = De bestanden van het pakket opnieuw lezen
packs-credits = Vermeldingen downloaden
packs-credits-title = CREDITS.md: wie het pakket heeft gemaakt, de licentie en de vermelding die het vraagt
packs-unreal-open = Unreal-inhoud converteren…
packs-archive = Pakket archiveren
packs-unreal-intro = Hoard leest Unreal-packages zelf en slaat de inhoud op als gewone bestanden: texturen als PNG of EXR, geluiden als WAV, materialen als JSON. Het geconverteerde bestand neemt de plaats van de package in en behoudt de tags, links en collecties.
packs-unreal-converted =
    { $count ->
       *[other] geconverteerd
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] geconverteerd, { $kept } met het origineel bewaard
    }
packs-unreal-pending =
    { $count ->
       *[other] nog niet bekeken
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] wachten op Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] zonder gangbaar formaat
    }
packs-unreal-failed =
    { $count ->
       *[other] mislukt
    }
packs-unreal-reasons = Waarom packages zijn overgebleven
packs-unreal-editor-before = Meshes en animaties hebben Unreal Editor nodig. Voer op een computer met Unreal het volgende uit:
packs-unreal-editor-command = hoard unreal-worker --editor <pad naar UnrealEditor-Cmd>
packs-unreal-editor-after = ; dat exporteert ze als FBX en uploadt de resultaten hierheen.
packs-unreal-discard = De originele .uasset-bestanden weggooien zodra ze zijn geconverteerd
packs-unreal-discard-hint = Maakt ruimte vrij. Gegevens die alleen Unreal kent, zoals materiaalgrafen, gaan verloren.
packs-unreal-drop = Packages zonder gangbaar formaat verwijderen
packs-unreal-drop-hint = Blueprints, levels en andere assets die alleen de engine kent.
packs-unreal-convert = Converteren
packs-context-hint-before = Waar het pakket vandaan komt en wat de verkoper erover zegt. De LLM-tagger leest dit bij elk bestand, dus een storetekst als "120 ice and fire spells" helpt hem bij het benoemen van
packs-context-hint-after = .
packs-store-auto = Automatisch gevonden: het pakket noemt deze pagina zelf.
packs-store-not-this = Niet deze
packs-lookup-label = In de stores zoeken naar
packs-lookup-placeholder = De naam van het pakket in de store
packs-lookup-searching = Zoeken…
packs-lookup-search = Stores doorzoeken
packs-lookup-hide = Verbergen
packs-lookup-clues = Uit de bestanden van het pakket: { $clues }.
packs-lookup-also = Ook gezocht naar { $queries }.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Niets op Fab, itch.io, Kenney of OpenGameArt lijkt erop. Probeer minder of andere woorden.
packs-lookup-use = Gebruiken
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Nu
packs-review-store = Uit de store
packs-review-same = Gelijk
packs-review-read-page = Wordt bij het opslaan van de pagina gelezen
packs-review-take =
    { $count ->
        [one] { $count } veld invullen
       *[other] { $count } velden invullen
    }
packs-lookup-searched = Gezocht op { $date }.
packs-lookup-again = Opnieuw zoeken
packs-lookup-again-after = .
packs-lookup-unreachable = Kon { $stores } deze keer niet bereiken.
packs-lookup-also-look = Kijk ook op
packs-lookup-links-and = { " " }en{ " " }
packs-lookup-browser-only = (die kunnen alleen in je browser worden doorzocht); plak het adres van de pagina hieronder.
packs-lookup-open = Zoeken in stores…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Verkoper
packs-seller-placeholder = Wie het heeft gemaakt
packs-license-label = Licentie
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Licentieniveau
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] { $count } asset heeft een eigen licentie: laat hem ook die van het pakket volgen
       *[other] { $count } assets hebben een eigen licentie: laat ze ook die van het pakket volgen
    }
packs-store-page = Storepagina
packs-store-text = Beschrijving uit de store
packs-store-text-placeholder = Plak de tekst van de pagina, of laat leeg om hem van de storepagina op te halen.
packs-store-text-fetch = Bij opslaan wordt de beschrijving van de storepagina opgehaald.
packs-readme = README gevonden in het pakket
packs-save-retag = Opslaan en LLM-tagger opnieuw uitvoeren

## AssetDetail.vue

asset-lazy-3d-viewer = de 3D-viewer
asset-lazy-text = de tekst
asset-lazy-text-viewer = de tekstviewer
asset-lazy-image-viewer = de afbeeldingsviewer
asset-lazy-material-preview = het materiaalvoorbeeld
asset-lazy-material-viewer = de materiaalviewer
asset-lazy-sprite-player = de spritespeler
asset-lazy-sheet-view = de spritesheetviewer
asset-rel-texture = textuur
asset-rel-buffer = geometriegegevens
asset-rel-material = materiaalbestand
asset-rel-animation = animatie
asset-rel-library = gelinkt bestand
asset-rel-guess = hoort er waarschijnlijk bij
asset-rel-sidecar = storegegevens
asset-rel-variant = variant
asset-rel-sprite = sprite
asset-rel-sample = sample
asset-role-map = map voor { $role }
asset-maps-count =
    { $count ->
        [one] { $count } map
       *[other] { $count } maps
    }
asset-needs = Heeft nodig
asset-animations = Animaties
asset-sprites = Sprites
asset-used-by = Gebruikt door
asset-loose-sprites = Ook als losse bestanden
asset-in-sheet = In een spritesheet
asset-other-formats = Andere formaten
asset-shown-as = In de bibliotheek getoond als
asset-conversion-failed = conversie mislukt
asset-as-converting = Converteren…
asset-as-converting-percent = Converteren… { $percent }%
asset-as-queued = In de wachtrij…
asset-download = Downloaden
asset-as-label = Downloaden als
asset-as-original = { $format } (origineel formaat)
asset-as-pack-files = De eigen bestanden van het pakket
asset-as-size = Grootte
asset-as-full-size = Volledige grootte
asset-as-up-to = Tot { $size } px
asset-as-hint = Kies een grootte of een ander formaat
asset-meta-size = Grootte
asset-meta-length = Lengte
asset-meta-sample-rate = Samplefrequentie
asset-meta-channels = Kanalen
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bitdiepte
asset-meta-bits = { $bits }-bits
asset-meta-level = Niveau
asset-meta-level-value = piek { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = piek { $peak } dBFS, RMS { $rms } dBFS, clipt
asset-meta-loop = Loop
asset-meta-loop-value = { $start } tot { $end }
asset-meta-playback = Afspelen
asset-meta-one-shot = One-shot
asset-meta-envelope = Envelop
asset-meta-envelope-times = aanzet { $attack }, uitsterven { $tail }
asset-meta-envelope-hit = klap
asset-meta-envelope-swell = aanzwellend
asset-meta-envelope-sustained = aangehouden
asset-meta-tone = Klankkleur
asset-meta-tone-value = helderheid { $khz } kHz, ruis { $noise }%
asset-meta-pitch = Toonhoogte
asset-meta-pitch-rises =
    { $octaves ->
        [one] stijgt { $octaves } octaaf
       *[other] stijgt { $octaves } octaven
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] daalt { $octaves } octaaf
       *[other] daalt { $octaves } octaven
    }
asset-meta-loops-cleanly = Loopt naadloos rond
asset-meta-tempo = Tempo
asset-meta-tempo-heard = ongeveer { $bpm } BPM, op het gehoor
asset-meta-tempo-heard-or = ongeveer { $bpm } BPM (of { $alt }), op het gehoor
asset-meta-key = Toonsoort
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, op het gehoor
music-key-major = { $note } majeur
music-key-minor = { $note } mineur
asset-meta-root-note = Grondtoon
asset-meta-cue-markers = Cuemarkeringen
asset-meta-library = Bibliotheek
asset-meta-keywords = Trefwoorden
asset-meta-title = Titel
asset-meta-artist = Artiest
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Generator
asset-meta-everything-with = Alles met de tag { $tag } tonen
asset-meta-saved-with = Opgeslagen met
asset-meta-color-mode = Kleurmodus
asset-meta-layers = Lagen
asset-meta-range = Bereik
asset-meta-hdr = HDR (voorbeeld is tonemapped)
asset-meta-format = Formaat
asset-meta-large-document = Groot document (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Gemaakt met generatieve AI
asset-meta-ai-edited = Bewerkt met generatieve AI
asset-meta-ai-how-metadata = de metadata
asset-meta-ai-how-generator = de generatorinstellingen
asset-meta-ai-how-iptc = het IPTC-veld Digital Source Type
asset-meta-ai-says = { $made }, volgens { $how }
asset-meta-ai-by-says = { $made } ({ $by }), volgens { $how }
asset-meta-provenance = Herkomst
asset-meta-c2pa = Heeft C2PA Content Credentials
asset-meta-compression = Compressie
asset-meta-note = Notitie
asset-meta-maps = Maps
asset-meta-sizes =
    { $count ->
        [one] Grootte
       *[other] Groottes
    }
asset-meta-triangles = Driehoeken
asset-meta-vertices = Vertices
asset-meta-meshes = Meshes
asset-meta-materials = Materialen
asset-meta-animations = Animaties
asset-meta-skins = Skins
asset-meta-unreal-asset = Unreal-asset
asset-meta-unreal-saved-with = { $class }, opgeslagen met UE { $engine }
asset-meta-package = Package
asset-meta-cooked = Cooked (gamebuild): voorbeelden en bronnen zijn verwijderd
asset-meta-tiling = Tegeling
asset-meta-seamless = Naadloos op beide assen
asset-meta-analysis-error = Kon dit bestand niet volledig lezen: { $error }
asset-copy-blocked = Kon niet kopiëren: de browser blokkeerde het
asset-save-error = Kon niet opslaan: { $error }
asset-tag-removed = { $tag } verwijderd
asset-tag-restored = { $tag } teruggezet
asset-tags-added =
    Toegevoegd: { $count ->
        [one] { $count } tag
       *[other] { $count } tags
    }
asset-tags-accepted =
    { $count ->
        [one] { $count } voorgestelde tag
       *[other] { $count } voorgestelde tags
    } behouden
asset-reviewed = Nagekeken
asset-reviewed-on-title = Nagekeken: regels en AI laten de tags met rust. Klik om de markering weg te halen (R)
asset-reviewed-off-title = Markeren als nagekeken, zodat regels en AI de tags met rust laten (R)
asset-reviewed-on = Gemarkeerd als nagekeken
asset-reviewed-off = Markering 'nagekeken' weggehaald
# $name is an asset's name.
asset-reviewed-on-named = { $name } als nagekeken gemarkeerd
asset-reviewed-off-named = Markering 'nagekeken' weggehaald bij { $name }
asset-undo = Ongedaan maken
asset-play-error = Kon dit bestand niet afspelen: { $error }
asset-copied-hash = Hash gekopieerd
asset-copied-toml = Toml gekopieerd
asset-copied-id = ID gekopieerd
asset-copied-link = Link gekopieerd
asset-origin-before = Dit bestand was
asset-origin-class-engine = , een { $class }, opgeslagen door UE { $engine }.
asset-origin-class = , een { $class }.
asset-origin-engine = { " " }opgeslagen door UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor heeft het geëxporteerd op { $date }.
asset-origin-editor-notes = Unreal Editor heeft het geëxporteerd op { $date } ({ $notes }).
asset-origin-hoard = Hoard heeft het uit de package gelezen op { $date }.
asset-origin-hoard-notes = Hoard heeft het uit de package gelezen op { $date } ({ $notes }).
asset-origin-download = Het originele .{ $ext } downloaden
asset-origin-discarded = De originele package is na de conversie weggegooid.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Geschiedenis
history-loading = Laden…
history-none = Hier is nog niets gewijzigd.
history-load-failed = De geschiedenis kon niet worden geladen: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = AI
history-by-rule = Een regel
history-by-someone = Iemand
history-by-app = { $who } via een app
history-by-assistant = { $who } via een assistent
history-field-description = Beschrijving
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Tag { $tag } toegevoegd
history-tag-removed = Tag { $tag } verwijderd
history-tag-kept = Voorgestelde tag { $tag } behouden
history-tag-changed = Tag { $tag } gewijzigd
history-reviewed = Gemarkeerd als beoordeeld
history-unreviewed = Gemarkeerd als niet beoordeeld
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: ingesteld op { $to }
history-cleared = { $field }: gewist (was { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Ongedaan maken
history-revert-title = Zet terug wat deze wijziging veranderde
history-reverted = Ongedaan gemaakt: { $change }
history-revert-failed = Dat kon niet ongedaan worden gemaakt: { $error }
asset-details = Assetdetails
asset-in-pack = in
asset-previous = Vorige asset
asset-next = Volgende asset
asset-favorite = Favoriet
asset-favorite-on-title = In je favorieten (F haalt het eruit)
asset-favorite-off-title = Aan je favorieten toevoegen (F)
asset-share = Delen
asset-share-title = Een link naar deze asset delen (wie hem opent, meldt zich eerst aan bij Hoard)
asset-layout-panel = In een zijpaneel tonen
asset-layout-page = Volledige pagina tonen
asset-layout-page-title = Volledige pagina tonen: een groot voorbeeld naast de details
asset-close = Details sluiten
asset-rename-title = Klik om de naam te wijzigen
asset-name = Naam
asset-views-alt = Het model vanuit vier richtingen
asset-inspect-aria = { $name } schermvullend bekijken
asset-inspect-title = Bekijken: zoomen, tegelen, kanalen
asset-playback-position = Afspeelpositie
asset-playback-value = { $current } van { $duration }
asset-pause = Pauzeren
asset-play = Afspelen
asset-waveform-drawing = Golfvorm tekenen…
asset-waveform-skipped = Geen golfvorm voor zulke grote bestanden
asset-play-animation = Als animatie afspelen
asset-show-views = Het blad met vier aanzichten tonen dat de tagger zag
asset-palette = Kleuren in de afbeelding
asset-swatch-title = { $hex }, { $percent }% van de afbeelding. Assets met deze kleur zoeken
asset-swatch-aria = Assets zoeken met { $hex }, { $percent }% van deze afbeelding
asset-more-like-this = Meer zoals dit
asset-desc-label = Je beschrijving
asset-desc-placeholder = Wat het is, waar je het zou gebruiken…
asset-save = Opslaan
asset-cancel = Annuleren
asset-desc-family = Wordt opgeslagen voor alle { $count } varianten
asset-desc-edit = Beschrijving bewerken
asset-desc-write = Een beschrijving schrijven
asset-desc-ai = Geschreven door AI
asset-desc-file = Uit het bestand
asset-desc-store-before = Van de
asset-desc-store-page = { $store }-pagina
asset-desc-store-page-unnamed = storepagina
asset-desc-store-after = (het hele pakket)
asset-less = Minder
asset-more = Meer
asset-tags = Tags
asset-tag-add-label = Een tag toevoegen
asset-tag-add-placeholder = Voeg een tag toe, bijv. mood:tense
asset-apply-family = Wijzigingen toepassen op alle { $count } varianten
asset-tags-accept-all =
    { $count ->
        [one] { $count } suggestie
       *[other] { $count } suggesties
    } behouden
asset-tags-accept-all-title = Alle tags die regels en AI hebben voorgesteld als eigen tags behouden
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Afgewezen:
asset-tag-restore-title = { $tag } terugzetten
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = Vergelijkbare hebben:
asset-tag-suggest-title = { $tag } toevoegen
asset-details-heading = Details
asset-field-category = Categorie
asset-field-license = Licentie
asset-field-source = Bron
asset-field-rating = Beoordeling
# Shown in place of an empty value, to fill it in.
asset-field-add = Toevoegen
asset-field-new = Veld toevoegen
asset-field-name = Veldnaam
asset-field-value = Waarde
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = uit het pakket
asset-field-license-placeholder = Zoals CC0 of CC-BY 4.0
asset-field-remove-placeholder = Leeg laten om te verwijderen
# $field is a field's name, like "License".
asset-field-edit-title = { $field } bewerken
asset-field-saved = { $field } opgeslagen
asset-field-cleared = { $field } gewist
asset-rate =
    { $count ->
        [one] { $count } ster
       *[other] { $count } sterren
    }
asset-rated =
    Beoordeeld met { $count ->
        [one] { $count } ster
       *[other] { $count } sterren
    }
asset-rating-clear-title = Klik nogmaals om de beoordeling te wissen
asset-rating-cleared = Beoordeling gewist
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Klik nogmaals voor { $count } ster
       *[other] Klik nogmaals voor { $count } sterren
    }
asset-collections = Collecties
asset-collection-remove = Verwijderen uit { $name }
asset-collection-add-another = Aan een andere toevoegen
asset-collection-add = Aan collectie toevoegen
asset-used-in = Gebruikt in
asset-used-in-title = Tonen wat { $name } heeft
asset-used-in-synced = gesynchroniseerd { $when }
asset-maps = Maps
asset-show-more = Nog { $count } tonen
asset-materials-heading =
    { $count ->
        [one] Materiaal
       *[other] Materialen
    }
asset-material-maps-aria = { $name }: maps
asset-variants = Varianten ({ $count } versies)
asset-fit-animations = Passende animaties ({ $count })
asset-fit-characters = Personages waarop hij past ({ $count })
asset-copies = Identieke kopieën
asset-copies-note = Dezelfde bytes worden één keer opgeslagen; deze pakketten leveren dit bestand ook mee.
asset-near-copies = Bijna-kopieën
asset-near-copies-note = Hetzelfde in een andere grootte, samplefrequentie of een ander formaat. De beste kopie is gemarkeerd.
asset-near-copies-best = Hetzelfde in een andere grootte, samplefrequentie of een ander formaat. Deze is de beste kopie.
asset-near-best = beste
asset-unreal-converted = Geconverteerd vanuit Unreal
asset-rule-converted = Geconverteerd door een regel
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Een conversieregel heeft deze { $format } op { $date } gemaakt van
asset-rule-kept-until = Het origineel wordt bewaard tot { $date }.
asset-rule-rebuilt = De FLAC bewaart alles wat in de WAV zat, zodat de WAV op elk moment precies zoals hij was kan worden gedownload of teruggezet.
asset-rule-put-back = Het origineel terugzetten
asset-rule-put-back-title = { $file } terugzetten?
asset-rule-put-back-body = Het geconverteerde bestand wordt verwijderd en het origineel neemt zijn plaats weer in, met zijn tags, collecties en links. De conversieregels laten het daarna met rust.
asset-rule-put-back-done = { $file } is terug
asset-rule-let-go = Het origineel is verwijderd toen de bewaartermijn voor originelen uit Instellingen voorbij was.
asset-unreal-package = Uit de Unreal-package
asset-ue-needs-editor-before = Wachten op Unreal Editor: { $reason }. Voer op een computer met Unreal
asset-ue-needs-editor-after = uit, dan komt het terug als FBX.
asset-ue-editor-path = pad naar UnrealEditor-Cmd
asset-ue-unsupported = Bewaard als package: { $reason }.
asset-ue-failed = Conversie mislukt: { $reason }
asset-recovered = Het originele .{ $ext } zat in deze package opgeslagen, en Hoard heeft het eruit gehaald.
asset-recovered-download = .{ $ext } downloaden
asset-ue-meshes-before = Meshes en animaties zijn opgeslagen in het eigen formaat van Unreal. Converteer het pakket (pakketpagina, Meer) en voer daarna
asset-ue-meshes-after = uit naast een installatie van Unreal Editor om FBX te krijgen.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } asset-registry-tag
       *[other] { $count } asset-registry-tags
    }
asset-file = Bestand
asset-bytes = Bytes
asset-source = Bron
asset-path = Pad
asset-path-open-folder = Bekijken wat er nog meer in { $folder } staat
asset-download-maps =
    { $count ->
        [one] De { $count } map downloaden
       *[other] Alle { $count } maps downloaden
    }
asset-download-linked =
    { $count ->
        [one] Downloaden met { $count } gelinkt bestand
       *[other] Downloaden met { $count } gelinkte bestanden
    }
asset-download-only-title = Alleen dit bestand, zonder de bestanden die het gebruikt
asset-download-only = Alleen dit bestand
asset-include-animations = Animaties in de .zip opnemen
asset-include-character = Personage { $name } meenemen
asset-copy-toml = hoard.toml-regel kopiëren
asset-copy-id = ID kopiëren
asset-teach = Hoard dit bestand leren
asset-teach-title = Beschrijf het bestand van deze store zodat Hoard het leest (Instellingen → Storebestanden)
asset-archive = Archiveren
asset-archive-title = Niet meer in zoekresultaten tot het wordt teruggezet of definitief verwijderd, vanuit het Archief

## ImageViewer.vue

image-channel-red = Rood kanaal
image-channel-green = Groen kanaal
image-channel-blue = Blauw kanaal
image-channel-alpha = Alfakanaal
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Geblokte achtergrond (K wisselt)
image-bg-black-title = Zwarte achtergrond (K wisselt)
image-bg-gray-title = Grijze achtergrond (K wisselt)
image-bg-white-title = Witte achtergrond (K wisselt)
image-bg-checker = geblokte achtergrond
image-bg-black = zwarte achtergrond
image-bg-gray = grijze achtergrond
image-bg-white = witte achtergrond
image-inspect-aria = { $name } bekijken
image-preview-size = voorbeeld { $width }×{ $height }, { $bytes }
image-preview = voorbeeld
image-resolution = Resolutie
image-full-title = De volledige afbeelding van { $width }×{ $height } laden (F); verder inzoomen dan het voorbeeld doet dat ook
image-full-loading = Volledige resolutie laden…
image-full-failed = Volledige resolutie niet geladen: opnieuw proberen
image-full = Volledige resolutie
image-zoom = Zoomen
image-zoom-out-title = Uitzoomen (−)
image-zoom-out = Uitzoomen
image-zoom-in-title = Inzoomen (+)
image-zoom-in = Inzoomen
image-fit-title = Passend (0)
image-fit = Passend
image-actual-title = Werkelijke pixels (1)
image-view = Weergave
image-tile-title = 3×3 tegelen om de naden te controleren (T)
image-tile = Tegelen
image-channels = Kanalen
image-close = Sluiten (Esc)
image-failed = Deze afbeelding kon niet worden geladen.
image-loading = Laden…
image-key-gestures = Scroll of knijp om te zoomen · sleep om te verschuiven · dubbelklik om in te zoomen of passend te maken
image-key-tile = tegelen
image-key-background = achtergrond
image-key-full = volledige resolutie
image-key-channels = kanalen

## SpritePlayer.vue

sprite-frame-aria = { $name }, frame { $frame } van { $count }
sprite-failed = Deze kan niet worden afgespeeld.
sprite-loading = Frames laden…
sprite-previous = Vorig frame
sprite-next = Volgend frame
sprite-pause = Animatie pauzeren
sprite-play = Animatie afspelen
sprite-animation = Animatie
sprite-all-frames = Alle frames
sprite-grid-title = Hoe het blad in frames wordt verdeeld
sprite-columns = Kolommen
sprite-rows = Rijen
sprite-fps-aria = Frames per seconde
sprite-fps = fps

## De spritesheetviewer (SheetView.vue) en de downloadformaten (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
       *[other] { $count } sprites
    }
sheet-pages =
    { $count ->
        [one] { $count } pagina
       *[other] { $count } pagina's
    }
sheet-animations =
    { $count ->
        [one] { $count } animatie
       *[other] { $count } animaties
    }
# The box that narrows the list of sprites by name.
sheet-find = Een sprite zoeken
# Under the list of sprites when more match than it draws.
sheet-showing = { $shown } van { $total } getoond
sheet-no-match = Geen enkele sprite heeft dat in zijn naam.
sheet-failed = Dit blad kan niet worden gelezen.
sheet-loading = Blad laden…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Het eigen formaat van Hoard: de pagina's zoals ze zijn, sprites alle kanten op gedraaid en bijgesneden, met animaties en de duur van elk frame.
sheet-note-texturepacker = Sprites met de klok mee gedraaid en bijgesneden, met animaties op naam, zoals PixiJS en Phaser ze lezen.
sheet-note-clockwise = Sprites met de klok mee gedraaid en bijgesneden. Animaties passen er niet in: frames gaan op hun genummerde namen.
sheet-note-libgdx = Sprites tegen de klok in gedraaid en bijgesneden. Animaties passen er niet in: frames gaan op hun genummerde namen.
sheet-note-godot-frames = Sprites nooit gedraaid, met wat er is bijgesneden bewaard als marges, met animaties en de duur van elk frame.
sheet-note-godot-tileset = Tegels nooit gedraaid of bijgesneden, op een raster van de tegelgrootte; afbeeldingen die al op een raster staan blijven zoals ze zijn.
sheet-note-grid = Elke sprite rechtop op zijn hele canvas, in cellen van één formaat, voor alles wat een blad in een raster verdeelt.
sheet-note-frames = Elke sprite rechtop op zijn hele canvas, elk als PNG met zijn eigen naam.

## TextViewer.vue, TextPreview.vue en text.ts

text-lang-config = Configuratie
text-lang-table = Tabel
text-lang-code = Code
text-lang-script = Script
text-lang-text = Tekst
text-copy-blocked = Kon niet kopiëren: de browser blokkeerde het
text-read-aria = { $name } lezen
text-lines =
    { $count ->
        [one] { $count } regel
       *[other] { $count } regels
    }
text-show-as = Tonen als
text-tree-title = Inklapbare boom (T)
text-tree = Boom
text-pretty-title = Opnieuw ingesprongen (P)
text-pretty = Netjes
text-raw-title = Zoals geschreven (R)
text-raw = Ruw
text-document-title = Als document (D)
text-document = Document
text-source-title = De markdown zoals geschreven (R)
text-source = Bron
text-page-title = Zoals een browser hem uit zijn map toont, los van Hoard (P)
text-page = Pagina
text-page-source-title = De pagina zoals hij geschreven is (R)
text-page-aria = { $name } als pagina
text-page-error = Kon deze pagina niet openen: { $error }
text-fold = Inklappen
text-expand-all-title = Alles uitklappen
text-expand-all = Alles uitklappen
text-collapse-all-title = Alles inklappen
text-collapse-all = Alles inklappen
text-group-text = Tekst
text-wrap-title = Lange regels afbreken (W)
text-wrap = Afbreken
text-copy-title = De tekst kopiëren (C)
text-copied = Gekopieerd
text-copy = Kopiëren
text-download-title = Het bestand downloaden
text-download = Downloaden
text-close = Sluiten (Esc)
text-truncated = De eerste { $shown } van { $total } wordt getoond.
text-truncated-json = De eerste { $shown } van { $total } wordt getoond (de boom heeft het hele bestand nodig).
text-show-all = Alles tonen
text-json-error = Deze JSON is niet te lezen: { $error }
text-contents-aria = Inhoud van { $name }
text-loading = Laden…
text-load-error = Kon dit bestand niet laden: { $error }
text-binary = Dit bestand is binair, geen tekst, dus er valt hier niets te lezen. Download het om het in het eigen programma te openen.
text-key-tree = boom
text-key-pretty = netjes
text-key-raw = ruw
text-key-document = document
text-key-source = bron
text-key-page = pagina
text-key-wrap = afbreken
text-key-copy = kopiëren
text-key-find = zoeken
text-key-close = sluiten
text-preview-aria = { $name } schermvullend lezen
text-preview-read = Lezen

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
json-fold = Inklappen
json-unfold = Uitklappen
json-unfold-summary = { $summary } uitklappen
json-show-more = Nog { $count } tonen
json-left = van { $count } resterend
json-picker-more = … nog { $count }

## 3D-modelviewers (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = de 3D-viewer is niet geladen ({ $error }); de pagina opnieuw laden kan helpen
model-loading-viewer = De 3D-viewer laden…
model-loading = Model laden…
model-show-failed = Kon dit model niet tonen: { $error }
model-preview-failed = Kon geen voorbeeld van dit model tonen: { $error }
model-error-converting = de server is dit model nog aan het converteren
model-error-converting-desktop = Hoard is dit model nog aan het converteren
model-error-empty-collada = leeg Collada-bestand
model-error-no-loader = Geen browserloader voor .{ $ext }
model-dialog = { $name } in 3D bekijken
model-title-stats =
    { $triangles } driehoeken · { $meshes ->
        [one] { $meshes } mesh
       *[other] { $meshes } meshes
    } · { $materials ->
        [one] { $materials } materiaal
       *[other] { $materials } materialen
    }
model-triangles =
    { $count ->
        [one] { $count } driehoek
       *[other] { $count } driehoeken
    }
model-group-camera = Camera
model-group-shading = Shading
model-group-background = Achtergrond
model-group-animation = Animatie
model-group-view = Weergave
model-group-show = Tonen
model-frame-title = Model in beeld brengen (F)
model-frame = In beeld
model-reset-title = Terug naar de beginweergave (R)
model-reset = Herstellen
model-turn-title = Langzaam draaien (A)
model-turn = Draaien
model-textures-title = Texturen (T)
model-textures = Texturen
model-full-res-title = Texturen op volledige resolutie (H): elk textuurbestand zoals het is, hoe groot ook; uit: lichte voorbeelden van maximaal 2048 px
model-full-res = Volledig
model-light-title = Omgevingslicht (E)
model-light = Licht
model-grid-title = Vloerraster (G)
model-grid = Raster
model-bg-checker = Geblokte achtergrond
model-bg-dark = Donkere achtergrond
model-bg-gray = Grijze achtergrond
model-bg-light = Lichte achtergrond
model-bg-env = Omgevingsachtergrond
model-bg-title = { $background } (K wisselt)
model-pause-title = Pauzeren (Spatie)
model-play-title = Afspelen (Spatie)
model-pause = Pauzeren
model-play = Afspelen
model-animation-clip = Animatieclip
model-animation = Animatie
model-plays-on = Afspelen op
model-rest-pose = Rustpose
model-close-title = Sluiten (Esc)
model-full-screen = Volledig scherm
model-open-full-screen = De 3D-viewer op volledig scherm openen
model-menu-fold = Het menu inklappen
model-menu-show = Het menu tonen
model-view-prefix = Weergave:{ " " }
model-menu-render = Render
model-menu-maps = Maps
model-menu-inspect = Inspecteren
model-mode-final = Eindrender
model-mode-raw = Ruwe render
model-mode-color = Basiskleur
model-mode-roughness = Ruwheid
model-mode-metalness = Metaal
model-mode-normal = Normal
model-mode-ao = Occlusie
model-mode-matcap = Matcap
model-mode-uv = UV-controle
model-mode-geometry = Geometrieën
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Wireframe (W), over elke weergave
model-wireframe = Wireframe
model-info-title = Scène-informatie (I)
model-info = Scène-informatie
model-info-close = Scène-informatie sluiten
model-info-close-title = Sluiten (I)
model-info-geometries = Geometrieën
model-info-materials = Materialen
model-info-textures = Texturen
model-info-triangles = Driehoeken
model-info-vertices = Vertices
model-info-meters = glTF is in meters
model-info-no-units = Dit formaat zegt niet welke eenheden het gebruikt
model-info-height = Hoogte
model-info-length = Lengte
model-info-width = Breedte
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } eenheid
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } eenheden
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Textuur niet gevonden: { $name }
model-missing-some = { $count } texturen niet gevonden: { $names }
model-missing-more = { $count } texturen niet gevonden: { $names }…
model-missing-count = { $count } texturen niet gevonden
model-guessed = Texturen op naam gekoppeld; het bestand noemt er geen.
model-group-facing = Richting
model-facing-turn = Kwartslag draaien
model-facing-turn-title = Het model een kwartslag tegen de klok in draaien, van bovenaf gezien
model-facing-tip = Kwartslag kantelen
model-facing-tip-title = De bovenkant van het model een kwartslag naar voren kantelen, om een liggend model rechtop te zetten
model-facing-save = Op pakket toepassen
model-facing-save-title = Alle modellen in { $pack } zo draaien. Hun miniaturen worden opnieuw getekend; downloads blijven zoals ze gemaakt zijn.
model-facing-undo = Ongedaan maken
model-facing-undo-title = Terug naar hoe de modellen van het pakket gedraaid zijn
model-facing-saved = Alle modellen in { $pack } staan nu zo. Hun miniaturen worden opnieuw getekend.
model-facing-failed = Kon de modellen van het pakket niet draaien: { $error }
model-keys-mouse = Sleep om te draaien · rechts slepen om te verschuiven · scroll om te zoomen · dubbelklik om in beeld te brengen
model-key-frame = in beeld
model-key-reset = herstellen
model-key-view = weergave
model-key-wireframe = wireframe
model-key-info = scène-informatie
model-key-textures = texturen
model-key-full-res = texturen op volledige resolutie
model-key-light = licht
model-key-grid = raster
model-key-background = achtergrond
model-key-play = afspelen

## Materialen (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Kleur
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Ruwheid
material-role-gloss = Glans
material-role-metalness = Metaal
material-role-ao = Ambient occlusion
material-role-orm = Occlusie, ruwheid, metaal
material-role-mra = Metaal, ruwheid, occlusie
material-role-rma = Ruwheid, metaal, occlusie
material-role-height = Hoogte
material-role-bump = Bump
material-role-cavity = Cavity
material-role-specular = Specular
material-role-opacity = Dekking
material-role-emissive = Emissie
material-role-translucency = Doorschijnendheid
material-role-base-color-name = kleur
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = ruwheid
material-role-gloss-name = glans
material-role-metalness-name = metaal
material-role-ao-name = ambient occlusion
material-role-orm-name = occlusie, ruwheid, metaal
material-role-mra-name = metaal, ruwheid, occlusie
material-role-rma-name = ruwheid, metaal, occlusie
material-role-height-name = hoogte
material-role-bump-name = bump
material-role-cavity-name = cavity
material-role-specular-name = specular
material-role-opacity-name = dekking
material-role-emissive-name = emissie
material-role-translucency-name = doorschijnendheid
material-maps =
    { $count ->
        [one] { $count } map
       *[other] { $count } maps
    }
material-dialog = Het materiaal { $name } bekijken
material-up-to = tot { $size }
material-repeat-size = { $size } per herhaling
material-shape-sphere = Bol
material-shape-cube = Kubus
material-shape-plane = Vlak
material-shape-title = { $shape } ({ $key })
material-group-shape = Vorm
material-group-tiling = Tegeling
material-real-title = Werkelijke grootte: één herhaling is { $size }, de bol is 2 m breed (T wisselt)
material-repeat-title = { $n }× herhalen (T wisselt)
material-real = Echt
material-repeat = { $n }×
material-height-title = Hoe ver de hoogtemap het oppervlak verplaatst
material-height = Hoogte
material-height-strength = Hoogtesterkte
material-loading = Materiaal laden…
material-show-failed = Kon dit materiaal niet tonen: { $error }
material-preview-failed = Kon geen voorbeeld van dit materiaal tonen: { $error }
material-group-maps = Maps
material-map-show = Map voor { $map } tonen
material-map-hide = Map voor { $map } verbergen
material-map-unused = Niet gebruikt door het voorbeeld
material-open-full-screen = De materiaalviewer op volledig scherm openen
material-keys-mouse = Sleep om te draaien · rechts slepen om te verschuiven · scroll om te zoomen · klik op een map om hem uit te zetten
material-key-shape = vorm
material-key-tiling = tegeling
material-key-turn = draaien

## theme.ts

theme-slate = Leisteen
theme-paper = Papier
theme-linen = Linnen
theme-sage = Salie
theme-lilac = Lila
theme-blush = Blos
theme-graphite = Grafiet
theme-obsidian = Obsidiaan
theme-ember = Sintel
theme-moss = Mos
theme-aubergine = Aubergine
theme-oxblood = Ossenbloed
theme-light-slate-blurb = Koel blauwgrijs
theme-light-paper-blurb = Eenvoudig neutraal grijs
theme-light-linen-blurb = Warm crème, amber
theme-light-sage-blurb = Zacht groen
theme-light-lilac-blurb = Bleek violet
theme-light-blush-blurb = Oudroze
theme-dark-graphite-blurb = Neutraal grijs, zonder tint
theme-dark-slate-blurb = Blauwgrijs
theme-dark-obsidian-blurb = Bijna zwart, voor OLED
theme-dark-ember-blurb = Warm antraciet, amber
theme-dark-moss-blurb = Bosgroen-zwart
theme-dark-aubergine-blurb = Pruimzwart, lila
theme-dark-oxblood-blurb = Wijnzwart, koraal

## texnames.ts

texnames-needs-map = Het patroon heeft { "{" }map{ "}" } nodig, anders krijgt elke map van een materiaal dezelfde naam
texnames-placeholders = Het patroon kan { "{" }name{ "}" }, { "{" }map{ "}" } en { "{" }size{ "}" } gebruiken
texnames-bad-characters = Namen mogen geen < > : " / \ | ? of * bevatten
texnames-no-suffix = Geef maps voor { $role } een achtervoegsel
texnames-bad-suffix = Het achtervoegsel voor maps voor { $role } mag geen < > : " / \ | ? of * bevatten
texnames-slot-no-suffix = Geef “{ $slot }” een achtervoegsel
texnames-slot-bad-suffix = Het achtervoegsel voor “{ $slot }” mag geen < > : " / \ | ? of * bevatten
texnames-ending-missing = Elke eigen uitgang heeft de uitgang nodig waarnaar gezocht wordt
texnames-ending-bad = Eigen uitgangen mogen geen < > : " / \ | ? of * bevatten
texnames-ending-twice = { $ending } staat twee keer in je uitgangen
texnames-slot-metallic-roughness = Metaal en ruwheid
texnames-slot-specular-color = Specular-kleur
texnames-slot-specular-glossiness = Specular en glans
texnames-slot-clearcoat = Clearcoat
texnames-slot-clearcoat-roughness = Clearcoat-ruwheid
texnames-slot-clearcoat-normal = Clearcoat-normal
texnames-slot-sheen-color = Sheen-kleur
texnames-slot-sheen-roughness = Sheen-ruwheid
texnames-slot-transmission = Transmissie
texnames-slot-thickness = Dikte
texnames-slot-diffuse-transmission = Diffuse transmissie
texnames-slot-diffuse-transmission-color = Kleur diffuse transmissie
texnames-slot-iridescence = Iridescentie
texnames-slot-iridescence-thickness = Dikte iridescentie
texnames-slot-anisotropy = Anisotropie

## storeFormats.ts

storefiles-field-title = Titel
storefiles-field-title-hint = De exacte titel van de storepagina. Die geeft het pakket zijn naam, en zonder titel wordt het bestand overgeslagen.
storefiles-field-seller = Verkoper
storefiles-field-seller-hint = Wie het heeft gemaakt of verkoopt.
storefiles-field-license = Licentie
storefiles-field-license-hint = Zoals het bestand het schrijft, bijvoorbeeld CC-BY-4.0.
storefiles-field-url = Storepagina
storefiles-field-url-hint = Het adres van de storepagina, beginnend met http:// of https://.
storefiles-field-words = Tags
storefiles-field-words-hint = Woorden om het mee te taggen. Door komma's gescheiden waarden worden gesplitst.
storefiles-field-type = Soort
storefiles-field-type-hint = Wat het is, zoals “3D-model” of “oppervlak”. Ook een tag.
storefiles-field-colors = Kleuren
storefiles-field-colors-hint = De kleuren, als color:-tags.
storefiles-field-id = Store-ID
storefiles-field-id-hint = De ID die de store ervoor gebruikt: letters, cijfers, _ of -.
storefiles-field-origin = Gemaakt in
storefiles-field-origin-hint = Waar het is gemaakt of gescand, voor de beschrijving.
storefiles-field-size = Grootte
storefiles-field-size-hint = Hoe groot het is, in woorden, voor de beschrijving.
storefiles-field-title-name = titel
storefiles-field-seller-name = verkoper
storefiles-field-license-name = licentie
storefiles-field-url-name = storepagina
storefiles-field-words-name = tags
storefiles-field-type-name = soort
storefiles-field-colors-name = kleuren
storefiles-field-id-name = store-ID
storefiles-field-origin-name = herkomst
storefiles-field-size-name = grootte

## StoreFormatsPanel.vue

storefiles-load-failed = Kon de formaten voor storebestanden niet laden: { $error }
storefiles-remove-title = { $name } verwijderen?
storefiles-remove-body = { $file } wordt uit de datamap verwijderd. Pakketten die er al mee zijn gelezen, houden wat het heeft gelezen.
storefiles-remove = Verwijderen
storefiles-removed = { $name } verwijderd.
storefiles-remove-failed = Kon { $name } niet verwijderen: { $error }
storefiles-unread = Bestanden die Hoard niet leest
storefiles-loading = Laden…
storefiles-unread-none = Geen: geen enkel bestand in de bibliotheek lijkt op dat van een store die niet wordt gelezen.
storefiles-unread-what = Bestanden in meer dan één pakket, of genoemd zoals die van een store, die geen formaat leest.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pakket
       *[other] { $count } pakketten
    }
storefiles-unread-example = bijv. { $pack }
storefiles-teach = Hoard dit bestand leren
storefiles-formats = Formaten
storefiles-formats-what = Die van jou worden eerst geprobeerd en vervangen een ingebouwd formaat met dezelfde naam.
storefiles-dir-before = Ze worden bewaard in
storefiles-dir-after = .
storefiles-yours = Van jou
storefiles-built-in = Ingebouwd
storefiles-edit = Bewerken
storefiles-start-from = Hiermee beginnen

## TagsPanel.vue

tags-load-failed = Kon de tags niet laden: { $error }
tags-rename-title = { $tag } hernoemen?
tags-merge-title = { $count } tags samenvoegen?
tags-rename-body = De { $count } assets ermee krijgen in plaats daarvan { $into }, en de taggers gebruiken vanaf nu de nieuwe naam.
tags-merge-body = { $tags } worden { $into } bij maximaal { $count } assets. De taggers gebruiken vanaf nu { $into }.
tags-rename = Hernoemen
tags-merge = Samenvoegen
tags-merged =
    { $count ->
        [one] { $count } asset heeft nu de tag { $into }
       *[other] { $count } assets hebben nu de tag { $into }
    }
tags-merge-failed = Kon niet samenvoegen: { $error }
tags-forget-failed = Kon dat niet wijzigen: { $error }
tags-find = Tags zoeken
tags-find-placeholder = Zoek tags, bijv. rock of set:
tags-count =
    { $count ->
        [one] { $count } tag
       *[other] { $count } tags
    }
tags-count-more = { $count }+ tags
tags-what = Kies tags die hetzelfde betekenen en voeg ze samen, of kies er een om hem te hernoemen.
tags-new-name = Nieuwe naam
tags-merge-into = Samenvoegen tot
tags-saved-as-before = Opgeslagen als
tags-saved-as-after = { "" }
tags-cancel = Annuleren
tags-loading = Laden…
tags-none = Geen tags.
tags-none-containing = Geen tags met “{ $text }”.
tags-browse-title = Assets met de tag { $tag } bekijken
tags-browse = Bekijken
tags-merged-heading = Samengevoegde tags
tags-merged-what = Als de taggers met een van deze oude namen komen, krijgt de asset de tag waarin die is samengevoegd.
tags-allow-old = De oude naam weer toestaan
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Oude naam: { $names }
       *[other] Oude namen: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } is samengevoegd met { $into }.
tag-page-show = { $tag } tonen
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Zeg wat deze tag betekent…
tag-page-describe-label = Wat deze tag betekent
tag-page-describe-placeholder = Waar hij voor dient, wat eronder valt. Enter slaat op.
tag-page-describe-edit = Klik om te wijzigen
tag-page-description-saved = Betekenis van de tag opgeslagen
# The tag's picture: one of its assets stands for it.
tag-page-picture = Kies een afbeelding voor deze tag
tag-page-picture-help = Kies een van de assets om hem te vertegenwoordigen
tag-page-picture-nothing = Geen van de getoonde assets heeft een afbeelding.
tag-page-picture-none = Geen afbeelding
tag-page-picture-saved = Afbeelding ingesteld
tag-page-picture-removed = Afbeelding verwijderd
tag-page-rename = Hernoemen…
tag-page-rename-title = Geef hem een nieuwe naam, of de naam van een andere tag om hem daarmee samen te voegen
tag-page-rename-label = Nieuwe naam
tag-page-save-failed = De tag kon niet worden opgeslagen: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Onder
tag-page-parent-add = Plaats onder…
tag-page-parent-add-title = Zet deze tag onder een bredere: wie op die zoekt, vindt ook deze
tag-page-parent-placeholder = Een bredere tag, zoals voertuig
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Haal { $tag } van onder { $parent } weg
tag-page-parent-saved = { $tag } staat nu onder { $parent }
tag-page-parent-removed = { $tag } staat niet meer onder { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Smallere:
tag-page-narrower-title = Zoeken op { $tag } vindt ook deze
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Breder en smaller
tags-tree-what = Tags die vanaf hun tagpagina onder bredere tags zijn gezet. Zoeken op een tag vindt ook alles eronder.

## LibraryStats.vue

stats-source-human = Toegevoegd door mensen
stats-source-ai = Voorgesteld door AI
stats-source-rule = Ingebouwde regels
stats-source-meta = Uit de bestanden
stats-assets = Assets
stats-packs = Pakketten
stats-on-disk = Op schijf
stats-duplicates-title = { $count } bestanden zijn bytegelijke kopieën van andere en worden één keer opgeslagen
stats-saved-dedup = Bespaard door ontdubbeling
stats-distinct-tags = Verschillende tags
stats-archive = In het archief
stats-load-failed = Kon de details niet laden: { $error }
stats-counting = Tellen…
stats-as-of = Stand van { $time }
stats-updating = Bijwerken…
stats-by-kind = Ruimte per soort
stats-files-size = { $name }: { $count } bestanden, { $size }
stats-months = Toegevoegd in het afgelopen jaar
stats-month = { $month }: { $count } assets, { $size }
stats-tagging = Tagging
stats-tagged = Getagd
stats-untagged = { $count } zonder tags
stats-described = Beschreven
stats-described-assets = { $count } assets
stats-human-edited = Met de hand bewerkt
stats-human-edited-unit =
    { $count ->
       *[other] assets
    }
stats-source-different = { $count } verschillende
stats-most-used = Meest gebruikt
stats-show-tag = De assets met { $tag } tonen
stats-file-types = Bestandstypen
stats-biggest-packs = Grootste pakketten
stats-browse-pack = { $name } bekijken
stats-largest-files = Grootste bestanden
stats-open-file = { $name } openen
stats-empty = Nog niets in de bibliotheek. Voeg een pakket toe en de cijfers verschijnen hier.

## HoardPanel.vue

hoard-title = Jouw schat
hoard-what = Elk asset in de bibliotheek is een stuk schat, en de soort bepaalt welk. Verzamel wat nieuw is om het aan je stapel toe te voegen.
hoard-sign-in = Meld je aan om een eigen schat te verzamelen.
hoard-new = Nieuwe buit om te verzamelen: { $count }
hoard-none-new = Niets nieuws om te verzamelen.
hoard-collect = Verzamelen
hoard-unavailable = Deze browser kan de schat niet tekenen: { $error }
hoard-collect-failed = Kon niet opslaan wat je hebt verzameld: { $error }
hoard-arcade = Arcadescherm
hoard-full-screen = Volledig scherm
hoard-leave-full-screen = Volledig scherm sluiten
hoard-label = Een draak die zijn schat bewaakt. Schatten: { $count }
hoard-gold = Goud
hoard-gems = Edelstenen
hoard-books = Boeken
hoard-weapons = Wapens
hoard-bones = Botten
hoard-tablets = Runentabletten
hoard-paintings = Schilderijen
hoard-potions = Drankjes

## UpdatesPanel.vue

updates-saved = Hoard roept het aan zodra er een update klaarstaat
updates-saved-none = Geen adres om aan te roepen
updates-save-failed = Kon niet opslaan
updates-call-answered = Het antwoordde { $result }
updates-call-refused = Het nam het verzoek niet aan: { $result }
updates-call-failed = Kon het niet aanroepen
updates-called = Aangeroepen op { $when } voor { $version }: { $result }
updates-called-failed = Aangeroepen op { $when } voor { $version }: { $result }. Hoard probeert het over een uur opnieuw.
updates-latest = Hoard { $version } is de nieuwste versie.
updates-out = Hoard { $version } is uit.
updates-out-idle = Hoard { $version } is uit; het adres wordt binnen een paar minuten aangeroepen, terwijl er niets gebeurt.
updates-out-busy = Hoard { $version } is uit; wachten tot er niets meer gebeurt ({ $work }).
updates-busy-more = nog { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } taak in de wachtrij
       *[other] { $count } taken in de wachtrij
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } upload
       *[other] { $count } uploads
    }
updates-list-separator = ,{ " " }
updates-what = Als er een nieuwe versie uit is, kan Hoard een adres aanroepen zodra er niets gebeurt (geen uploads, taken, conversies of onderhoud), zodat wat de container bijwerkt hem niet midden in een upload herstart. Het stuurt een POST met de versie in een JSON-body. Hiervoor is de versiecontrole hierboven nodig.
updates-url = Aan te roepen adres
updates-url-placeholder = Geen (bijv. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Opgeslagen (typ om te vervangen)
updates-auth-placeholder = Geen (bijv. Bearer my-token)
updates-remove = Verwijderen
updates-save = Opslaan
updates-call-now-title = Wat eraan gekoppeld is, wordt uitgevoerd
updates-call-now = Nu aanroepen
updates-what-calls = Wat kan het aanroepen?
updates-watchtower = De HTTP-API van Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , met{ " " }
updates-watchtower-token = { " " }en het token uit{ " " }
updates-watchtower-its = { " " }(met het{ " " }
updates-watchtower-endpoint = -endpoint ingeschakeld).
updates-portainer = Een Portainer-webhook
updates-portainer-rest = { " " }op de stack of container van Hoard, met re-pull image aan. Het token zit in de URL.
updates-script = Een script op de host
updates-script-listener = { " " }achter een webhook-listener (zoals{ " " }
updates-script-runs = ) die{ " " }
updates-script-end = { " " }uitvoert.
updates-others = Home Assistant, n8n
updates-others-rest = { " " }of iets anders dat een webhook aanneemt.
updates-poll-before = Of sla het adres over: een script met een toegangstoken kan{ " " }
updates-poll-until = { " " }blijven opvragen tot{ " " }
updates-poll-and = { " " }is ingesteld en{ " " }
updates-poll-after = { " " }true is, en dan de nieuwe versie ophalen en Hoard herstarten.

## MaintenancePanel.vue

maintenance-load-failed = Kon Onderhoud niet laden
maintenance-reanalyze-title = Elk pakket opnieuw analyseren?
maintenance-reanalyze-body = Elk bestand wordt opnieuw gelezen, wat bij een grote bibliotheek lang kan duren; pakketten staan op bezig tot ze aan de beurt zijn. Je eigen namen, tags en notities blijven.
maintenance-reanalyze-confirm = Opnieuw analyseren
maintenance-gc-title = Ongebruikte bestanden verwijderen?
maintenance-gc-body = Opgeslagen bestanden waar niets in de bibliotheek naar verwijst, worden definitief verwijderd. Pakketten in het archief houden die van hen.
maintenance-gc-confirm = Verwijderen
maintenance-start-failed = Kon “{ $task }” niet starten
maintenance-just-now = zojuist
maintenance-minutes-ago = { $count } min geleden
maintenance-hours-ago = { $count } u geleden
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } u
maintenance-loading = Laden…
maintenance-running = Bezig…
maintenance-starting = Starten…
maintenance-run = Uitvoeren
maintenance-stop = Stoppen
maintenance-stopping = Bezig met stoppen…
maintenance-stop-failed = Kon “{ $task }” niet stoppen
maintenance-started = { $detail } · gestart { $when }
maintenance-starting-detail = Starten
maintenance-finished = Voltooid:{ " " }
maintenance-failed = Mislukt:{ " " }
maintenance-never-run = Nooit uitgevoerd
maintenance-history = Eerdere runs
maintenance-col-task = Taak
maintenance-col-finished = Voltooid
maintenance-col-took = Duur
maintenance-col-result = Resultaat
maintenance-script-before = Back-ups en de schadecontrole kunnen ook vanuit een script worden uitgevoerd:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }en{ " " }
maintenance-script-after = . Die runs verschijnen hier ook.

## SettingsPage.vue

settings-title = Instellingen
settings-loading = Laden…
settings-find = Een instelling zoeken
settings-find-none = Geen instelling past bij ‘{ $query }’.
settings-load-failed = Kon de instellingen niet laden: { $error }
settings-page-appearance = Weergave
settings-page-appearance-blurb = De thema’s die deze browser draagt als hij licht is en als hij donker is. Alleen in deze browser opgeslagen.
settings-page-dragon = Draak
settings-page-dragon-blurb = De schat van je draak: elk asset in de bibliotheek is er een stuk van.
settings-page-library = Bibliotheek
settings-page-library-blurb = Wat er in de hoard zit: hoe groot hij is, waar hij uit bestaat, hoe hij is gegroeid en hoe goed hij is getagd. Gearchiveerde dingen tellen niet mee.
settings-page-copies = Bijna-kopieën
settings-page-copies-blurb = Bestanden die dezelfde afbeelding, hetzelfde geluid of model zijn in een andere grootte, samplefrequentie of een ander formaat, die met de meeste ruimte eerst. De beste van elke set is gemarkeerd; er wordt niets verwijderd.
copies-loading = Bijna-kopieën zoeken…
copies-refreshing = De bibliotheek is veranderd sinds deze zijn gevonden. Opnieuw zoeken…
copies-none = Geen bijna-kopieën: elke afbeelding, elk geluid en elk model staat hier maar één keer.
copies-load-failed = Kon bijna-kopieën niet tonen: { $error }
copies-sum = Sets bijna-kopieën: { $count }. De kopieën die niet de beste zijn, nemen { $size } in.
copies-set = Set { $n }
copies-set-wasted = { $size } in de kopieën die niet de beste zijn
copies-more = Meer tonen (nog { $count })
settings-page-tags = Tags
settings-page-tags-blurb = Elke tag in de bibliotheek. Voeg tags samen die hetzelfde betekenen, of hernoem er een; de taggers gebruiken vanaf dan de nieuwe naam.
settings-page-store-files = Storebestanden
settings-page-store-files-blurb = De bestanden die stores naast hun art zetten, die Hoard leest om pakketten een naam te geven en hun makers te vermelden. Leer het bestanden die het nog niet kent, en deel wat je het hebt geleerd.
settings-page-locations = Bibliotheeklocaties
settings-page-locations-blurb = De mappen op de server die de bibliotheek bevatten: mappen waarin Hoard bestanden bewaart, en mappen met pakketten die het indexeert waar ze staan.
settings-page-locations-blurb-desktop = De mappen die de bibliotheek bevatten: mappen waarin Hoard bestanden bewaart, en mappen met pakketten die het indexeert waar ze staan.
settings-page-tools = Hulpprogramma's
settings-page-tools-blurb = Externe programma’s die Hoard gebruikt om bestanden te converteren en te lezen.
settings-page-ai = AI-tagging
settings-page-ai-blurb = Een AI-model geeft nieuwe pakketten een naam, beschrijving en tags, bovenop de ingebouwde regels. De tags ervan worden als voorgesteld gemarkeerd.
settings-page-search-blurb = Zoek afbeeldingen op wat ze tonen en geluiden op hoe ze klinken, niet alleen op hun naam, en vind dingen die qua soort op elkaar lijken, niet alleen qua kleur.
settings-page-search-tab = Zoeken
settings-page-adding = Pakketten toevoegen
settings-page-adding-blurb = Wat Hoard met een pakket doet als het wordt toegevoegd, naast het lezen en taggen.
settings-page-downloads = Downloads
settings-page-downloads-blurb = Hoe bestanden heten in wat Hoard verstuurt: downloads, zips, de hoard-tool en AI-assistenten. Bestanden in de bibliotheek blijven zoals ze zijn.
settings-page-account = Account
settings-page-account-blurb = Je wachtwoord, de andere mensen die kunnen inloggen, en toegangstokens voor scripts en andere programma’s.
settings-page-auth = Authenticatie
settings-page-auth-blurb = Hoe mensen inloggen: met de eigen accounts van Hoard, een LDAP-directory, Microsoft Active Directory of een OAuth-provider, hoe lang sessies duren, en welke groepen welke rol krijgen.
settings-page-connection = Verbinding
settings-page-connection-blurb = Hoe browsers en apps Hoard bereiken: het adres waaronder het bekend is, en HTTPS, dat sommige browserfuncties nodig hebben.
settings-page-performance = Prestaties
settings-page-performance-blurb = Hoe hard de server werkt, hoe miniaturen worden gemaakt en bewaard, en hoeveel hij maximaal aanneemt.
settings-page-performance-blurb-desktop = Hoe hard Hoard werkt, hoe miniaturen worden gemaakt en bewaard, en hoeveel het maximaal aanneemt.
settings-page-issues = Problemen
settings-page-issues-blurb = Wat er misging bij het openen of verwerken van bestanden: archieven die niet uitpakten, bestanden die niet gelezen konden worden, taken die mislukten. Probeer ze opnieuw zodra de oorzaak is opgelost, of negeer ze.
settings-page-maintenance = Onderhoud
settings-page-maintenance-blurb = Back-ups maken, op schade controleren en opruimen. Van elk loopt er één tegelijk, en je kunt deze pagina verlaten terwijl ze bezig zijn.
settings-page-about = Over
settings-page-about-blurb = Wat Hoard is, welke versie dit is, en het werk van anderen waarmee het gemaakt is.
settings-the-default = de standaardwaarde
settings-default-on = aan
settings-default-off = uit
settings-default-auto = automatisch
settings-option-saved = { $name }: { $value }
settings-back-to-default = Terug naar de standaardwaarde
settings-save = Opslaan
settings-use = Gebruiken
settings-status-off = Uit

## SettingsPage.vue: Weergave

settings-themes = Thema’s
settings-look-light = Licht
settings-look-dark = Donker
settings-look-group =
    { $scheme ->
        [light] Licht thema
       *[dark] Donker thema
    }
settings-preview =
    Het { $preview ->
        [light] lichte
       *[dark] donkere
    } thema wordt getoond zodat je het kunt zien. Je gebruikt { $used ->
        [light] licht
       *[dark] donker
    }{ $system ->
        [yes] { " " }(volgens het systeem)
       *[other] { "" }
    }, en dat komt terug als je deze pagina verlaat.
settings-preview-back =
    Terug naar { $used ->
        [light] licht
       *[dark] donker
    }
settings-look-hint = Wissel tussen licht en donker (of volg het systeem) via de bovenbalk.

## SettingsPage.vue: Hulpprogramma's

settings-blender-what = Converteert .blend-, .dae-, .abc- en .x3d-modellen op de achtergrond naar glTF, zodat ze miniaturen en een 3D-voorbeeld krijgen. Pakketten die zijn toegevoegd terwijl het uit stond, worden geconverteerd zodra het klaar is.
settings-blender-download = Gebruikt een geïnstalleerde Blender, of downloadt Blender 4.5 LTS (ongeveer 400 MB) van een mirror van blender.org.
settings-ffmpeg-what = Codeert MP3-, OGG- en Opus-downloads, en leest audio die Hoard zelf niet kan decoderen (AAC, M4A, WMA, …).
settings-ffmpeg-download = Gebruikt een ffmpeg van het systeem met de juiste encoders, of downloadt een statische build (ongeveer 80 MB).
settings-tool-reset = { $tool }: terug naar de standaardwaarde
settings-tool-no-download = Gebruikt een geïnstalleerde { $tool }. Er is geen build om te downloaden voor deze machine, dus installeer het of geef het pad op.
settings-tool-path = Programmapad van { $tool }
settings-mode-off = Uit
settings-mode-auto = Automatisch
settings-mode-path = Dit programma
settings-tool-looking = Zoeken…
settings-tool-downloading = Downloaden… de voortgang staat in de activiteitsindicator
settings-tool-ready = Klaar: { $detail }
settings-soundfont = MIDI-SoundFont
settings-soundfont-what = De instrumenten waarmee MIDI-bestanden worden afgespeeld en geconverteerd.
settings-soundfont-file = SoundFont-bestand
settings-soundfont-placeholder = Gevonden op de server, of ingebouwd
settings-soundfont-placeholder-desktop = Gevonden op deze computer, of ingebouwd
settings-soundfont-default = gevonden op de server, of ingebouwd
settings-soundfont-default-desktop = gevonden op deze computer, of ingebouwd
settings-soundfont-send = Bestand kiezen…
settings-soundfont-sent = MIDI speelt nu met { $name }

## SettingsPage.vue: Zoeken op beeld

settings-clip-heading = Zoeken op wat afbeeldingen tonen
settings-clip-reset = Zoeken op beeld: terug naar de standaardwaarde
settings-clip-saved-on = Zoeken op beeld: aan
settings-clip-saved-off = Zoeken op beeld: uit
settings-clip-what = Typ wat je zoekt (“mossy rock”, “sci-fi door”, “wooden barrel”) en afbeeldingen die het tonen verschijnen naast namen met die woorden. “Meer zoals dit” vindt dingen van dezelfde soort, niet alleen met dezelfde kleuren.
settings-clip-toggle = Zoeken op beeld inschakelen
settings-clip-hint = Gebruikt het SigLIP 2-model van Google, uitgevoerd op de processor van deze server: er gaat niets naar buiten. Inschakelen downloadt het model ({ $size } MB) van Hugging Face en leert daarna elke afbeelding op de achtergrond met lage prioriteit. Het gebruikt 350 tot 750 MB geheugen terwijl het werkt of iemand zoekt. Uitschakelen verwijdert het model en bewaart wat het heeft geleerd, voor de volgende keer.
settings-clip-hint-desktop = Gebruikt het SigLIP 2-model van Google, uitgevoerd op de processor van deze computer: er gaat niets naar buiten. Inschakelen downloadt het model ({ $size } MB) van Hugging Face en leert daarna elke afbeelding op de achtergrond met lage prioriteit. Het gebruikt 350 tot 750 MB geheugen terwijl het werkt of je zoekt. Uitschakelen verwijdert het model en bewaart wat het heeft geleerd, voor de volgende keer.
settings-clip-looking = Het model zoeken…
settings-clip-downloading = Het model downloaden… de voortgang staat in de activiteitsindicator
settings-clip-ready-empty = Klaar. Afbeeldingen worden geleerd zodra pakketten worden toegevoegd.
settings-clip-ready-all =
    { $count ->
        [one] Klaar: kent de { $count } afbeelding
       *[other] Klaar: kent alle { $count } afbeeldingen
    }
settings-clip-learning = Klaar: afbeeldingen leren, tot nu toe { $done } van { $total }
settings-clip-relearning = Tot het nieuwe model elke afbeelding heeft geleerd, gebruiken zoekopdrachten wat het oude heeft geleerd.
settings-clip-unavailable = Kon het model niet ophalen: { $detail }. Elk uur wordt het opnieuw geprobeerd.
settings-clip-where = Draait op
settings-clip-here = Deze server: er gaat niets naar buiten
settings-clip-here-desktop = Deze computer: er gaat niets naar buiten
settings-clip-hosted = Een gehoste dienst
settings-clip-hosted-warning = Afbeeldingen verlaten deze server. Een miniatuur van elke afbeelding in de bibliotheek en de woorden van elke zoekopdracht op beeld worden naar { $service } gestuurd, dat ervoor rekent en ze bewaart zoals de voorwaarden zeggen.
settings-clip-hosted-warning-desktop = Afbeeldingen verlaten deze computer. Een miniatuur van elke afbeelding in de bibliotheek en de woorden van elke zoekopdracht op beeld worden naar { $service } gestuurd, dat ervoor rekent en ze bewaart zoals de voorwaarden zeggen.
settings-clip-hosted-hint = Hier wordt niets gedownload of uitgevoerd: de dienst leert elke afbeelding, per partij. Een andere dienst of een ander model leert ze allemaal opnieuw, en wat elk heeft geleerd, blijft bewaard om terug te kunnen schakelen.
settings-clip-saved-here = Zoeken op beeld: draait op deze server
settings-clip-saved-here-desktop = Zoeken op beeld: draait op deze computer
settings-clip-saved-hosted = Zoeken op beeld: draait bij { $service }
settings-clip-hosted-unavailable = Kon { $service } niet gebruiken: { $detail }. Elk uur wordt het opnieuw geprobeerd.
settings-clap-heading = Zoeken op hoe geluiden klinken
settings-clap-reset = Zoeken op geluid: terug naar de standaardwaarde
settings-clap-saved-on = Zoeken op geluid: aan
settings-clap-saved-off = Zoeken op geluid: uit
settings-clap-what = Typ wat je zoekt (“distant thunder”, “glassy UI blip”, “footsteps on gravel”) en geluiden die zo klinken verschijnen naast namen met die woorden.
settings-clap-toggle = Zoeken op geluid inschakelen
settings-clap-hint = Gebruikt het CLAP-model van LAION, uitgevoerd op de processor van deze server: er gaat niets naar buiten. Inschakelen downloadt het model ({ $size } MB) van Hugging Face en luistert daarna op de achtergrond met lage prioriteit naar het begin van elk geluid. Het gebruikt tot ongeveer 1 GB geheugen terwijl het werkt of iemand zoekt. Uitschakelen verwijdert het model en bewaart wat het heeft geleerd, voor de volgende keer.
settings-clap-hint-desktop = Gebruikt het CLAP-model van LAION, uitgevoerd op de processor van deze computer: er gaat niets naar buiten. Inschakelen downloadt het model ({ $size } MB) van Hugging Face en luistert daarna op de achtergrond met lage prioriteit naar het begin van elk geluid. Het gebruikt tot ongeveer 1 GB geheugen terwijl het werkt of je zoekt. Uitschakelen verwijdert het model en bewaart wat het heeft geleerd, voor de volgende keer.
settings-clap-ready-empty = Klaar. Geluiden worden geleerd zodra pakketten worden toegevoegd.
settings-clap-ready-all =
    { $count ->
        [one] Klaar: kent het { $count } geluid
       *[other] Klaar: kent alle { $count } geluiden
    }
settings-clap-learning = Klaar: geluiden leren, tot nu toe { $done } van { $total }

## SettingsPage.vue: AI-tagging

settings-ai-service = Dienst
settings-ai-off = Uit
settings-ai-off-hint = Pakketten worden alleen met de ingebouwde regels getagd.
settings-ai-local = Lokaal model
settings-ai-local-hint = De llm-service in het compose-bestand van Hoard (of cargo xtask llm). Download hieronder een model; het draait op deze machine en er gaat niets naar buiten.
settings-ai-chatgpt = ChatGPT-abonnement
settings-ai-chatgpt-hint = Gebruikt je ChatGPT Go-, Plus-, Pro- of Business-abonnement in plaats van een API-sleutel.
settings-ai-openrouter-hint = Eén sleutel voor honderden gehoste modellen, waarvan sommige gratis. Modellen die afbeeldingen aankunnen, zijn gemarkeerd.
settings-ai-openai-hint = Gehost door OpenAI.
settings-ai-anthropic-hint = Gehost door Anthropic, via het OpenAI-compatibele endpoint.
settings-ai-gemini-hint = Gehost door Google, via het OpenAI-compatibele endpoint.
settings-ai-ollama-hint = Een Ollama-server die je zelf draait. Wijzig het adres als het geen container met de naam ollama is.
settings-ai-custom = Anders (OpenAI-compatibel)
settings-ai-custom-hint = Alles wat de chat-API van OpenAI spreekt: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Verbinden…
settings-ai-connected =
    { $count ->
        [one] Verbonden · { $count } model
       *[other] Verbonden · { $count } modellen
    }
settings-ai-connected-none = Verbonden, maar er worden geen modellen vermeld; typ de naam van het model
settings-chatgpt-connected-none = Verbonden; typ de naam van het model
settings-ai-takes-images = kan afbeeldingen aan
settings-ai-saved = AI-tagging: { $service }
settings-ai-saved-off = AI-tagging staat uit
settings-ai-reset = AI-tagging: terug naar de standaardwaarde
settings-ai-on = Aan: { $service } · { $model }
settings-ai-url = Adres
settings-ai-key = API-sleutel
settings-ai-key-saved = Opgeslagen (typ om te vervangen)
settings-ai-key-paste = Plak je sleutel
settings-ai-key-optional = Geen (optioneel)
settings-ai-key-remove = Verwijderen
settings-ai-get-key = Een sleutel aanvragen
settings-ai-model = Model
settings-ai-model-pick = Kies of typ een model
settings-ai-model-name = Naam van het model
settings-ai-refresh = Vernieuwen
settings-ai-connect = Verbinden
settings-ai-need-key = Voeg de API-sleutel toe om de modellen te zien
settings-ai-vision = Ook miniaturen meesturen (vereist een model dat afbeeldingen aankan)
settings-ai-batch = Per verzoek
settings-ai-batch-unit = assets (minder voor kleine lokale modellen, meer voor grote gehoste)

## SettingsPage.vue: Inloggen bij ChatGPT

settings-chatgpt-account = Account
settings-chatgpt-checking = Controleren…
settings-chatgpt-status-as = Ingelogd als { $email }
settings-chatgpt-status = Ingelogd
settings-chatgpt-signed-in-as = Ingelogd bij ChatGPT als { $email }
settings-chatgpt-signed-in = Ingelogd bij ChatGPT
settings-chatgpt-signed-out = Uitgelogd bij ChatGPT
settings-chatgpt-continue = Doorgaan met ChatGPT
settings-chatgpt-sign-out = Uitloggen
settings-chatgpt-sign-out-title = Uitloggen bij ChatGPT?
settings-chatgpt-sign-out-in-use = AI-tagging gebruikt je ChatGPT-abonnement, dus het stopt tot je opnieuw inlogt of een andere dienst kiest.
settings-chatgpt-sign-out-body = Hoard vergeet de aanmelding en vraagt OpenAI die te beëindigen.
settings-chatgpt-manage = Gebruik beheren
settings-chatgpt-finish-before = Rond het inloggen af in het venster dat is geopend (
settings-chatgpt-finish-link = open het opnieuw
settings-chatgpt-finish-middle = ). Als het eindigt op een pagina die niet laadt, op een adres dat begint met{ " " }
settings-chatgpt-finish-after = , kopieer dat adres dan en plak het hier:
settings-chatgpt-paste = Het adres waarop het inloggen eindigde
settings-chatgpt-finish = Afronden
settings-chatgpt-usage = Tagverzoeken tellen mee voor het gebruik van je ChatGPT-abonnement. Gebruik beheren laat zien wat Hoard heeft gebruikt en laat je het beperken. Om Hoard los te koppelen van ChatGPT zelf, ga je op chatgpt.com naar Instellingen → Beveiliging en inloggen → Inloggen met ChatGPT.

## SettingsPage.vue: Lokale modellen

settings-local-downloading-progress = Downloaden { $percent }% · { $done } van { $total }
settings-local-downloading = Downloaden…
settings-local-incomplete = Download gestopt
settings-local-failed = Kon de vorige keer niet starten
settings-local-in-use-running = In gebruik · actief
settings-local-in-use = In gebruik
settings-local-starting = Starten…
settings-local-running = Actief
settings-local-ready-images = Klaar · kan afbeeldingen aan
settings-local-ready-text = Klaar · alleen tekst
settings-local-delete-title = { $name } verwijderen?
settings-local-delete-in-use = AI-tagging gebruikt dit model, dus het stopt tot je een ander kiest. De bestanden worden van de modelserver verwijderd.
settings-local-delete-downloading = De download stopt en wat er tot nu toe binnen is, wordt verwijderd.
settings-local-delete-body = De bestanden worden van de modelserver verwijderd. Je kunt het later opnieuw downloaden.
settings-local-delete = Verwijderen
settings-local-deleting = Verwijderen…
settings-local-cancel = Annuleren
settings-local-resume = Hervatten
settings-local-download = Downloaden
settings-local-start-before = Start hem met{ " " }
settings-local-start-or = { " " }(of{ " " }
settings-local-start-developing = { " " }tijdens het ontwikkelen) en{ " " }
settings-local-start-retry = probeer het opnieuw
settings-local-start-after = . Een server die maar één model draait, werkt ook: geef hieronder de naam van dat model op.
settings-local-list = Op de modelserver
settings-local-none = Nog niets gedownload. Kies hieronder een model; het wordt op de server bewaard en je kunt altijd wisselen.
settings-local-catalog = Modellen om te downloaden
settings-local-size = { $size } GB · { $license }
settings-local-other = Een ander model van Hugging Face
settings-local-other-placeholder = Een ander van Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Pakketten toevoegen

settings-store-pages = Storepagina's
settings-store-pages-what = Nieuwe pakketten worden op de achtergrond in de stores opgezocht, zodat ze hun storepagina, verkoper en licentie krijgen.
settings-store-lookup = Storepagina's automatisch opzoeken
settings-unreal = Unreal-packages
settings-unreal-reset = Unreal-packages: terug naar de standaardwaarde
settings-unreal-what = Wat er met .uasset-bestanden moet gebeuren.
settings-conv = Conversieregels
settings-conv-what = Converteer bestanden die Hoard zelf bewaart naar één formaat, zoals elke OBJ en FBX naar GLB, zodat de hele bibliotheek gelijk is. Het geconverteerde bestand neemt de plaats van het origineel in, met zijn naam, tags, collecties en links. Bestanden in bewaakte mappen worden nooit gewijzigd.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Aan
settings-conv-to = Converteren naar
# Heading over the file formats a rule converts.
settings-conv-from = Van
settings-conv-remove = Verwijderen
settings-conv-remove-label = Regel { $n } verwijderen
settings-conv-taken = Een andere regel converteert dit formaat al
settings-conv-upright = Modellen bij het converteren rechtop zetten (Y omhoog, kijkend naar +Z), zoals de modellen van het pakket worden getoond
settings-conv-flac-what = Het geluid blijft precies hetzelfde, in ongeveer de helft van de ruimte, en al het andere in de WAV (looppunten, cuemarkeringen, tags) wordt in de FLAC bewaard, zodat de WAV op elk moment precies zoals hij was kan worden teruggezet. WAV’s met floating-point- of 32-bits-samples blijven WAV.
settings-conv-add = Een regel toevoegen
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Originelen bewaren gedurende
settings-conv-keep-days =
    { $count ->
        [one] dag
       *[other] dagen
    }
settings-conv-keep-what = Tot dan kan de pagina van een geconverteerd bestand het origineel terugzetten. Texturen worden niet in een geconverteerd model gekopieerd: het verwijst naar de afbeeldingsbestanden ernaast. De .mtl- of .bin-bestanden van een model blijven staan.
settings-conv-save = Regels opslaan
settings-conv-revert = Wijzigingen ongedaan maken
settings-conv-existing = Bestanden converteren die al in de bibliotheek staan
settings-conv-saved = Conversieregels opgeslagen. Ze gelden voor pakketten die vanaf nu worden toegevoegd.
settings-conv-converting =
    { $count ->
        [one] Eén bestand wordt op de achtergrond geconverteerd
       *[other] { $count } bestanden worden op de achtergrond geconverteerd
    }
settings-conv-nothing = Niets in de bibliotheek hoeft te worden geconverteerd.
settings-conv-ask-title =
    { $count ->
        [one] Eén bestand converteren dat al in de bibliotheek staat?
       *[other] { $count } bestanden converteren die al in de bibliotheek staan?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Het staat in één pakket.
       *[other] Ze staan in { $packs } pakketten.
    } Elk origineel wordt { $days } dagen bewaard, en tot dan kan het vanaf zijn pagina worden teruggezet.
settings-conv-ask-busy =
    { $count ->
        [one] Eén staat in een pakket dat nog wordt toegevoegd; het wordt geconverteerd zodra dat klaar is.
       *[other] { $count } staan in pakketten die nog worden toegevoegd; ze worden geconverteerd zodra dat klaar is.
    }
settings-conv-ask-list = Bestanden die zouden worden geconverteerd
settings-conv-ask-more = …en nog { $count }
settings-conv-ask-skipped = Blijven zoals ze zijn:
settings-conv-skip-watched =
    { $count ->
        [one] één bestand in een bewaakte map
       *[other] { $count } bestanden in bewaakte mappen
    }
settings-conv-skip-unity =
    { $count ->
        [one] één bestand dat een Unity-project bij zijn naam kent
       *[other] { $count } bestanden die een Unity-project bij hun naam kent
    }
settings-conv-skip-used =
    { $count ->
        [one] één bestand waar een scène of een ander bestand naar verwijst
       *[other] { $count } bestanden waar een scène of een ander bestand naar verwijst
    }
settings-conv-skip-beside =
    { $count ->
        [one] één bestand waarvan de geconverteerde kopie al in zijn pakket staat
       *[other] { $count } bestanden waarvan de geconverteerde kopieën al in hun pakketten staan
    }
settings-conv-skip-blender =
    { $count ->
        [one] één bestand dat Blender nog niet heeft gelezen (Instellingen → Hulpprogramma's)
       *[other] { $count } bestanden die Blender nog niet heeft gelezen (Instellingen → Hulpprogramma's)
    }
settings-conv-ask-new = Opslaan, alleen nieuwe pakketten
settings-conv-ask-save-convert =
    { $count ->
        [one] Opslaan en het converteren
       *[other] Opslaan en { $count } bestanden converteren
    }
settings-conv-ask-convert =
    { $count ->
        [one] Het converteren
       *[other] { $count } bestanden converteren
    }
settings-unreal-off = Laten zoals ze zijn
settings-unreal-off-hint = Converteer vanaf de pagina van een pakket wanneer je wilt.
settings-unreal-keep = Converteren, originelen bewaren
settings-unreal-keep-hint = Texturen, geluiden en materialen komen eruit in gangbare formaten; de .uasset blijft ook.
settings-unreal-replace = Converteren, originelen weggooien
settings-unreal-replace-hint = Bespaart ruimte zodra de inhoud eruit is. Meshes wachten nog steeds op de Unreal-worker.

## SettingsPage.vue: Downloads

settings-tn = Textuurnamen
settings-tn-saved = Textuurnamen opgeslagen
settings-tn-reset = Textuurnamen: terug naar de standaardwaarde
settings-tn-what = De maps van een materiaal gaan de deur uit met de naam van het materiaal en waar elke map voor is, hoe de bestanden ook heten: Rock_Normal.png, niet rock_moss_xeokah0_raw_8K_Normal.png. De bestanden in de bibliotheek worden niet aangeraakt, en een map die een meegestuurd model of MTL bij naam noemt, houdt die naam, zodat het model hem nog vindt.
settings-tn-toggle = Texturemaps naar hun materiaal noemen
settings-tn-style = Stijl
settings-tn-style-group = Naamgevingsstijl
settings-tn-descriptive = Beschrijvend
settings-tn-short = Kort
settings-tn-custom = Eigen
settings-tn-pattern = Patroon
settings-tn-hint-name = { " " }het materiaal,{ " " }
settings-tn-hint-map = { " " }waar de map voor is,{ " " }
settings-tn-hint-size = { " " }de grootte als de set in meer dan één grootte komt (aan het eind toegevoegd als het patroon hem weglaat).
settings-tn-example = Voorbeeld
settings-tn-suffixes = Achtervoegsels per soort map
settings-tn-slots = In GLB-downloads krijgt een textuur die zonder eigen bestand in een model is verpakt de naam van het materiaal en het slot dat hij vult, en ook deze slots hebben achtervoegsels:
settings-tn-endings = Eigen uitgangen
settings-tn-endings-what = Een textuur die geen map van een materiaal is, houdt zijn eigen naam, tenzij die op een van deze eindigt: _diff → _BC stuurt rock_diff.png als rock_BC.png. Laat het tweede leeg om de uitgang weg te laten.
settings-tn-ending-from = Uitgang
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Vervangen door
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Een uitgang toevoegen
settings-tn-ending-remove = Verwijderen
settings-tn-ending-remove-label = { $ending } verwijderen

## SettingsPage.vue: Algemeen en Verbinding

settings-name = Naam
settings-name-what = Hoe deze server heet in de bovenbalk, het browsertabblad en op de inlogpagina, zodat je met meer dan één hoard kunt zien welke welke is.
settings-name-what-desktop = Hoe deze bibliotheek heet in de bovenbalk.
settings-address = Adres
settings-address-what = Het adres waarmee mensen en apps Hoard bereiken, als dat niet het adres is waarop deze browser zit: dat van een reverse proxy, een tunnel of een tailnet-naam, zoals https://hoard.example.com. Links die Hoard uitdeelt (aan Claude en andere apps, bij het inloggen, op zijn certificaat) gebruiken het.
settings-public-url = Openbaar adres
settings-public-url-placeholder = Uit het adres van de browser (bijv. https://hoard.example.com)
settings-public-url-default = uit het adres van de browser

## ProfilerPanel.vue: de profiler bovenaan Instellingen → Prestaties

profiler = Profiler
profiler-what = Wat Hoard de afgelopen tien minuten heeft gedaan, elke twee seconden bijgewerkt terwijl je kijkt. Het wordt alleen in het geheugen bewaard en begint opnieuw als Hoard herstart.
profiler-starting = De eerste metingen komen zo binnen.
profiler-failed = Kon de profiler niet lezen: { $error }
profiler-cpu = Processorgebruik
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Hele machine
profiler-cpu-summary = Hoard gebruikt { $hoard } van de processors, en de hele machine { $machine }.
profiler-ram = RAM
profiler-ram-peak = Piek
profiler-ram-of = { $used } van { $total }
profiler-ram-summary = Hoard gebruikt { $hoard } RAM en gebruikte de afgelopen tien minuten hoogstens { $peak }.
profiler-serving = Lopende verzoeken
profiler-working = Bezig met
profiler-working-none = Er liep niets op de achtergrond.
profiler-working-more =
    { $count ->
        [one] En nog één.
       *[other] En nog { $count }.
    }
profiler-requests = Tijd besteed aan verzoeken
profiler-requests-what = Elke balk is tien seconden aan verzoeken, met hun tijd verdeeld naar waar die naartoe ging.
profiler-part-db = Database
profiler-part-db-title = Wachten op de database en het uitvoeren van queries
profiler-part-disk = Schijf
profiler-part-disk-title = Opgeslagen bestanden lezen en comprimeren wat wordt verstuurd
profiler-part-send = Versturen
profiler-part-send-title = Wachten terwijl het netwerk overneemt wat is verstuurd
profiler-part-other = Overig
profiler-part-other-title = Het antwoord uitwerken
profiler-request-count =
    { $count ->
        [one] { $count } verzoek
       *[other] { $count } verzoeken
    }
profiler-requests-none = Geen verzoeken in de afgelopen tien minuten.
profiler-slowest = Traagste: { $route }, { $time }
profiler-routes = Per route
profiler-route = Route
profiler-count = Verzoeken
profiler-average = Gemiddeld
profiler-p95 = Traagste 5%
profiler-split = Waar de tijd naartoe ging
profiler-sent = Verstuurd
profiler-portal = de eigen bestanden van de interface

## SettingsPage.vue: Prestaties

settings-render = 3D-miniaturen
settings-render-toggle = Miniaturen renderen voor 3D-modellen
settings-gpu-on = Op de GPU: { $renderer }
settings-gpu-software = Geen GPU gevonden, dus in software ({ $renderer })
settings-gpu-none = Geen GPU of softwarerenderer gevonden, dus modellen krijgen geen miniaturen
settings-thumbs = Miniaturen
settings-thumbs-what = Voor nieuwe miniaturen en modelweergaven. Transparante en pixelart blijven PNG. Miniaturen verkleinen bij Onderhoud maakt de bestaande opnieuw.
settings-previews-size = Voorbeelden: { $size }
settings-thumbs-saved = Miniaturen: { $quality }
settings-thumbs-format = Formaat
settings-thumbs-quality = Kwaliteit { $quality }.
settings-thumbs-small = Kleiner
settings-thumbs-small-hint = Prima in het raster; van dichtbij een beetje zacht.
settings-thumbs-balanced = Gebalanceerd
settings-thumbs-balanced-hint = Op miniatuurformaat nauwelijks van het origineel te onderscheiden.
settings-thumbs-sharp = Scherper
settings-thumbs-sharp-hint = Ongeveer twee keer zo groot als Gebalanceerd.
settings-thumbs-lossless = Verliesvrij
settings-thumbs-lossless-hint = PNG, precies zoals gemaakt. Een aantal keer groter. Worden nooit als AVIF verstuurd.
settings-thumbs-avif-what = AVIF is vaak half zo groot. Elke miniatuur wordt geconverteerd zodra hij voor het eerst getoond wordt.
settings-thumbs-avif-now = Nu alles converteren
settings-thumbs-avif-converting = Bezig met converteren…
settings-motion-thumbs = Bewegende miniaturen voor animaties
settings-motion-thumbs-what = Elke animatie krijgt ook een kort filmpje in een lus, dat speelt zolang je naar de tegel wijst. Ze nemen elk zo'n 40 KB in en het maken duurt even bij een grote bibliotheek. Uitzetten verwijdert ze.
settings-limits = Werk en limieten
settings-limits-what = Elke waarde wordt opgeslagen als je het veld verlaat of op Enter drukt. Groottes nemen eenheden: 100 GB, 512 MB, 1.5 TB; een los getal is GB.
settings-workers = Achtergrondprocessen
settings-workers-unit = taken tegelijk (verwerken, taggen, conversies)
settings-max-upload = Grootste upload
settings-max-upload-unit = per bestand
settings-max-expand = Grootste pakket
settings-max-expand-unit = na uitpakken
settings-conversion-cache = Geconverteerde downloads
settings-conversion-cache-unit-now = maximaal bewaard, nu { $size }; de minst recent gebruikte gaan eerst
settings-conversion-cache-unit = maximaal bewaard; de minst recent gebruikte gaan eerst

## SettingsPage.vue: Onderhoud

settings-updates = Updates
settings-version = Dit is Hoard { $version }.
settings-update-check = Elke paar uur op GitHub controleren op een nieuwe versie
settings-desktop-updates = Hoard zoekt bij elke start naar een nieuwe versie, downloadt die op de achtergrond en installeert hem wanneer je afsluit.
settings-backups = Geplande back-ups
settings-backups-what = De database maakt zelf back-ups; elke back-up staat in de geschiedenis hieronder.
settings-backup-every = Back-up maken
settings-backup-off = Nooit
settings-backup-daily = Elke dag
settings-backup-weekly = Elke week
settings-backup-keep = Bewaren
settings-backup-keep-unit = nieuwste geplande back-ups
settings-backup-copy-to = Ook kopiëren naar
settings-backup-copy-to-none = Geen andere map
settings-backups-kept = Elke back-up komt als hoard-auto-…db in de back-upmap, en in de andere map (een andere schijf, of een map die een sync-app back-upt) als die er is, opgegeven met het volledige pad op de server. Alleen oudere geplande back-ups worden verwijderd; handmatig gemaakte blijven.
about-what = Hoard bewaart game-assets (modellen, texturen, geluiden, muziek, sprites en meer) in één bibliotheek waarin je kunt zoeken, voorbeelden bekijken en waaruit je kunt pakken, gedeeld met iedereen die je toegang geeft.
about-version = Versie
about-edition = Editie
about-edition-server = Server
about-edition-desktop = Desktop-app
about-edition-steam = Desktop-app van Steam
about-made-by = Gemaakt door
about-source = Broncode op GitHub
about-issues = Een probleem melden
about-discord = Chat met ons op Discord
about-steam = Hoard op Steam
about-license = Licentie
about-license-what = Hoard is open source onder de MIT No Attribution-licentie: gebruik het, verander het en deel het zoals je wilt.
about-downloaded = Gedownload wanneer nodig
about-downloaded-what = Hoard wordt hier niet mee geleverd. Het downloadt elk ervan de eerste keer dat iets het nodig heeft, en elk valt onder zijn eigen licentie.
about-blender = Zet 3D-modellen om en maakt hun miniaturen.
about-ffmpeg = Zet geluid en video om en maakt hun voorbeelden.
about-siglip = Vindt afbeeldingen op basis van wat erop staat.
about-clap = Vindt geluiden op basis van hoe ze klinken.
about-models = Modellen die je kiest voor de lokale AI-server vallen onder de licenties van hun makers.
about-built-with = Gemaakt met
about-built-with-what = De opensourcebibliotheken waaruit Hoard bestaat, met hun licenties.
about-steamworks = De Steam-versie bevat ook de Steamworks SDK, © Valve Corporation.
about-licenses-loading = Lijst laden…
about-licenses-failed = De lijst kon niet worden geladen.
about-filter = Filteren op naam of licentie
about-count = { $shown } van { $total }
about-no-license = Geen licentie opgegeven
about-report = Een probleem melden…
report-title = Een probleem melden
report-what = Dit heeft een issue op GitHub nodig om het probleem te kunnen onderzoeken. Er staat niets privés in: toegangstokens, wachtwoorden, sleutels, e-mailadressen en je naam in mappaden worden eruit gehaald. Lees het door voordat je het verstuurt.
report-loading = Rapport wordt verzameld…
report-failed = Het rapport kon niet worden verzameld: { $error }
report-log = Wat de server onlangs heeft gelogd
report-copy = Kopiëren
report-copied = Rapport gekopieerd.
report-github = Een issue openen op GitHub
report-github-hint = Het nieuwe issue wordt hiermee ingevuld. Is het logboek daarvoor te lang, dan gaan alleen de laatste regels mee; Kopiëren heeft alles.
report-close = Sluiten

## Asset kinds (api.ts)

api-kind-texture = Texturen
api-kind-vector = Vectorafbeeldingen
api-kind-sheet = Spritesheets
api-kind-audio = Audio
api-kind-model = Modellen
api-kind-animation = Animaties
api-kind-font = Lettertypen
api-kind-shader = Shaders
api-kind-material = Materialen
api-kind-video = Video
api-kind-document = Documenten
api-kind-data = Gegevens
api-kind-other = Overig
api-kind-texture-name = texturen
api-kind-vector-name = vectorafbeeldingen
api-kind-sheet-name = spritesheets
api-kind-audio-name = audio
api-kind-model-name = modellen
api-kind-animation-name = animaties
api-kind-font-name = lettertypen
api-kind-shader-name = shaders
api-kind-material-name = materialen
api-kind-video-name = video
api-kind-document-name = documenten
api-kind-data-name = gegevens
api-kind-other-name = overige

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Ongedaan maken
ui-today = vandaag
ui-yesterday = gisteren
ui-days-ago =
    { $days ->
        [one] { $days } dag geleden
       *[other] { $days } dagen geleden
    }
ui-notifications = Meldingen
ui-dismiss = Sluiten
ui-cancel = Annuleren
ui-lazy-loading = Bezig met laden van { $what }…
ui-lazy-failed = Kon { $what } niet laden.
ui-lazy-reload = De pagina opnieuw laden

## ResetButton.vue

reset-title = Terug naar de standaardwaarde: { $value }
reset-label = Terug naar de standaardwaarde, { $value }

## SizeInput.vue

size-invalid = Een grootte zoals 100 GB, 512 MB of 1.5 TB
size-hint = Een grootte zoals 100 GB, 512 MB of 1.5 TB; een los getal is GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = Kon de authenticatie-instellingen niet laden: { $error }
auth-saved = Opgeslagen. Het geldt vanaf de volgende keer dat iemand inlogt.
auth-method-internal = Intern
auth-method-internal-hint = Namen en wachtwoorden die Hoard zelf bewaart, toegevoegd in Instellingen → Account.
auth-method-ldap-hint = Een directory controleert de naam en het wachtwoord die op de inlogpagina worden getypt: OpenLDAP, FreeIPA, LLDAP of een andere LDAP-server.
auth-method-ad-hint = Windows-domeinaccounts loggen in met hun gebruikelijke naam en wachtwoord.
auth-method-oauth-hint = Een knop “Inloggen met…” die naar Google, Microsoft, GitHub, Keycloak, Authentik of een andere provider gaat en weer terug.
auth-session-unused = Na 30 dagen niet gebruikt
auth-session-1h = 1 uur na het inloggen
auth-session-8h = 8 uur na het inloggen
auth-session-day = Een dag na het inloggen
auth-session-week = Een week na het inloggen
auth-session-30d = 30 dagen na het inloggen
auth-session-hours =
    { $hours ->
        [one] { $hours } uur na het inloggen
       *[other] { $hours } uur na het inloggen
    }
auth-role-view = Mag kijken
auth-role-edit = Mag wijzigen
auth-role-admin = Beheerder
auth-role-word-view = mag kijken
auth-role-word-edit = mag wijzigen
auth-role-word-admin = beheerder
auth-role-word-default = { $role } (de standaardrol)
auth-tls-starttls-hint = Gewone LDAP (poort 389), overgeschakeld naar TLS voordat er iets wordt verstuurd.
auth-tls-ldaps-hint = TLS vanaf het begin (poort 636).
auth-tls-none = Geen
auth-tls-none-hint = Wachtwoorden gaan onversleuteld over het netwerk. Alleen op een netwerk dat je vertrouwt.
auth-test-failed = Kon niet testen: { $error }
auth-preset-custom = Anders (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel en andere: geef de issuer op en Ontdekken vult de adressen in.
auth-preset-google-note = Maak een OAuth-client (Webapplicatie) aan in Google Cloud Console → API's en services → Inloggegevens. Google stuurt geen groepen mee, dus geef rollen in Instellingen → Account, en schakel het aanmaken van accounts bij de eerste keer inloggen uit, tenzij iedereen met een Google-account binnen mag komen.
auth-preset-microsoft-note = Registreer een app in Entra ID → App-registraties (met een webomleiding naar het adres hieronder). Vervang “organizations” door je tenant-ID om alleen je eigen organisatie binnen te laten. Voor rollen op basis van groepen voeg je de groups-claim toe onder Tokenconfiguratie: die vermeldt groeps-ID's.
auth-preset-github-note = Maak een OAuth-app aan in GitHub → Settings → Developer settings. GitHub stuurt geen groepen mee, dus geef rollen in Instellingen → Account, en schakel het aanmaken van accounts bij de eerste keer inloggen uit, tenzij iedereen met een GitHub-account binnen mag komen.
auth-preset-gitlab-note = Maak een applicatie aan in GitLab → Voorkeuren → Applicaties (of in het beheergedeelte van je eigen instantie: wijzig dan gitlab.com in de adressen).
auth-discovered = De adressen van de provider zijn ingevuld.
auth-copied = Gekopieerd
auth-copy-failed = Kon niet kopiëren: selecteer het en kopieer het zelf
auth-sign-in-with = Inloggen met
auth-own-accounts-still-work = De eigen accounts van Hoard loggen nog steeds in met hun eigen wachtwoord, dus een directory of provider die uitvalt of verkeerd is ingesteld, kan je niet buitensluiten.
auth-sessions = Sessies
auth-sign-out-after = Mensen uitloggen
auth-ldap-server = LDAP-server
auth-server-type = Soort server
auth-server-type-custom = Aangepast
auth-servers = Servers
auth-server-n = Server { $n }
auth-remove-server-n = Server { $n } verwijderen
auth-remove = Verwijderen
auth-add-server = Een server toevoegen
auth-servers-hint = Als er een niet bereikbaar is, wordt de volgende geprobeerd.
auth-security = Beveiliging
auth-connection-security = Verbindingsbeveiliging
auth-skip-verify = Het certificaat van de server niet controleren (alleen om dingen uit te proberen)
auth-ca = Certificeringsinstantie
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (leeg: die het systeem vertrouwt)
auth-lookup = Mensen opzoeken
auth-anonymously = Anoniem
auth-reader-account = Leesaccount
auth-reader-password = Wachtwoord ervan
auth-saved-type-to-replace = Opgeslagen: typ om te vervangen
auth-people-under = Mensen staan onder
auth-name-attribute = Naamattribuut
auth-only-matching = Alleen items die overeenkomen met
auth-groups-under = Groepen staan onder
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (leeg: alleen memberOf)
auth-their-groups = Hun groepen
auth-group-filter-hint = { "{" }dn{ "}" } is het item van de persoon, { "{" }user{ "}" } de naam waarmee die inlogde.
auth-test = Testen
auth-test-hint = Maakt verbinding met elke server en logt in met het leesaccount. Met een naam en wachtwoord logt het ook als die persoon in en laat het zien wat Hoard ervan zou maken. Er wordt niets opgeslagen.
auth-test-user = Iemand om als in te loggen
auth-test-user-placeholder = Iemand om als in te loggen (optioneel)
auth-test-password = Diens wachtwoord
auth-testing = Testen…
auth-test-connectivity = Verbinding testen
auth-test-server-ok = bereikt, en het leesaccount is ingelogd
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }is ingelogd (
auth-test-signed-in-after = ).
auth-test-role = Die persoon zou zijn: { $role }.
auth-test-no-role = Die persoon zit in geen van de groepen onder Rollen, en zou dus worden geweigerd.
auth-test-groups = Groepen: { $groups }
auth-test-no-groups = Geen groepen gevonden.
auth-provider = Provider
auth-redirect = Omleidingsadres
auth-copy = Kopiëren
auth-redirect-hint = Geef dit aan de provider als je de client voor Hoard aanmaakt. Het komt van het openbare adres in Instellingen → Verbinding, of van het adres dat je nu gebruikt.
auth-issuer = Issuer
auth-discover = Ontdekken
auth-client-id = Client-ID
auth-client-secret = Clientgeheim
auth-authorization-url = Autorisatieadres
auth-token-url = Tokenadres
auth-userinfo-url = Adres voor gebruikersgegevens
auth-userinfo-placeholder = (leeg: het ID-token zegt wie het is)
auth-scopes = Scopes
auth-name-from = Naam uit
auth-groups-from = Groepen uit
auth-groups-from-placeholder = (geen)
auth-button = Knop
auth-button-placeholder = je account
auth-accounts = Accounts
auth-provision = Een Hoard-account aanmaken de eerste keer dat iemand inlogt
auth-provision-on-hint = Anders voegt een beheerder eerst elke persoon toe, in Instellingen → Account.
auth-provision-off-hint = Een beheerder voegt eerst elke persoon toe, in Instellingen → Account, en kiest hoe die inlogt.
auth-open-to-anyone = Iedereen met een { $provider }-account kan binnenkomen. Voeg hieronder kijkersgroepen toe, of schakel dit uit en voeg mensen zelf toe.
auth-new-accounts = Nieuwe accounts
auth-roles-from-groups = Rollen op basis van groepen
auth-roles-hint = Groepen waarvan de leden elke rol krijgen, één per regel, op naam of DN. De hoogste die overeenkomt wint, en dat wordt bij elke keer inloggen gecontroleerd. Als er geen zijn opgegeven, krijgen nieuwe accounts de rol hierboven en wijzigt een beheerder die daarna. Als er groepen onder Mag kijken staan, mogen alleen leden van een opgegeven groep inloggen.
auth-roles-hint-oauth = Groepen waarvan de leden elke rol krijgen, één per regel, op naam of DN (of wat de groups-claim ook vermeldt). De hoogste die overeenkomt wint, en dat wordt bij elke keer inloggen gecontroleerd. Als er geen zijn opgegeven, krijgen nieuwe accounts de rol hierboven en wijzigt een beheerder die daarna. Als er groepen onder Mag kijken staan, mogen alleen leden van een opgegeven groep inloggen.
auth-saving = Opslaan…
auth-save = Opslaan

## HttpsPanel.vue (Settings → Connection)

https-load-failed = Kon HTTPS niet laden: { $error }
https-off-redirecting = HTTPS staat uit. Je gaat naar http://
https-now-on = HTTPS staat aan
https-saved = Opgeslagen. Nieuwe verbindingen gebruiken het nieuwe certificaat.
https-now-off = HTTPS staat uit
https-http3-failed-toast = HTTP/3 kon niet starten: { $error }
https-copied = Gekopieerd
https-copy-failed = Kon niet kopiëren: selecteer het en kopieer het zelf
https-mode-off = Uit
https-mode-off-hint = Alleen gewone http://.
https-mode-own = Het eigen certificaat van Hoard
https-mode-own-hint = Hoard maakt er een voor de adressen van deze server. Installeer de certificeringsinstantie van Hoard één keer op elk apparaat, en de browsers daarop vertrouwen het.
https-mode-custom = Mijn certificaat
https-mode-custom-hint = Een certificaat dat je al hebt, bijv. van tailscale cert of Let’s Encrypt. Browsers vertrouwen het zonder dat je iets hoeft te installeren.
https-untrusted-own = Deze browser vertrouwt het nog niet voor { $host }: installeer hieronder de certificeringsinstantie van Hoard. Je krijgt een waarschuwing die je kunt wegklikken.
https-untrusted-uncovered = Deze browser vertrouwt het nog niet voor { $host }: het certificaat dekt dit adres niet. Je krijgt een waarschuwing die je kunt wegklikken.
https-untrusted = Deze browser vertrouwt het nog niet voor { $host }. Je krijgt een waarschuwing die je kunt wegklikken.
https-now = Nu
https-start-failed = HTTPS is ingesteld, maar kon niet starten: { $error }
https-on-port = HTTPS staat aan, op poort { $port }, net als http://.
https-using-it = Je gebruikt het nu.
https-open = { $url } openen
https-trusted = Deze browser vertrouwt het.
https-cert-for-before = Certificaat voor{ " " }
https-cert-for-after = , tot { $date }.
https-cert-for-after-own = , tot { $date } (wordt vanzelf vernieuwd).
https-not-covered-before = Het dekt{ " " }
https-not-covered-after = { " " }niet, het adres dat je gebruikt.
https-is-off = HTTPS staat uit: pagina's worden via gewone http:// geserveerd.
https-addresses = Adressen
https-addresses-hint = Het certificaat dekt de adressen die Hoard heeft gevonden, en alle die je toevoegt (een naam die je router hem geeft, een Tailscale-naam).
https-remove-name = { $name } verwijderen
https-another-address = Nog een adres
https-another-address-placeholder = Nog een adres, bijv. nas.local
https-add = Toevoegen
https-your-cert = Je certificaat
https-saved-cert-before = Opgeslagen:{ " " }
https-saved-cert-after = { " " }van { $issuer }, tot { $date }. Plak een nieuw certificaat om het te vervangen.
https-pem-before = Het certificaat (met zijn keten) en de privésleutel ervan, als PEM-tekst: de bestanden die beginnen met{ " " }
https-pem-after = .
https-certificate = Certificaat
https-choose-file = Bestand kiezen…
https-private-key = Privésleutel
https-checked-before = Voor{ " " }
https-checked-after = , van { $issuer }, tot { $date }.
https-checked-after-days =
    , van { $issuer }, tot { $date } ({ $days ->
        [one] { $days } dag
       *[other] { $days } dagen
    }).
https-self-signed = Het is zelfondertekend: browsers zullen ervoor waarschuwen.
https-key-matches = De sleutel past.
https-where-to-get = Waar je er een krijgt
https-get-tailscale-before = schakel HTTPS in voor je tailnet, voer daarna{ " " }
https-get-tailscale-after = { " " }uit op de server en upload de twee bestanden. Ze zijn 90 dagen geldig: voer het daarvoor opnieuw uit en upload ze opnieuw.
https-get-acme-title = Let’s Encrypt of een andere ACME-client
https-get-acme-before = (certbot, acme.sh, lego) voor een domein dat van jou is: upload{ " " }
https-get-acme-and = { " " }en{ " " }
https-get-acme-after = .
https-get-proxy-title = Een reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) die het certificaat heeft: laat HTTPS hier uit en laat de proxy gewone http:// naar Hoard sturen.
https-trying = Uitproberen…
https-apply = Toepassen
https-apply-hint = Hoard probeert de nieuwe instelling eerst uit, en houdt wat het heeft als die niet werkt.
https-trust-title = Hoard vertrouwen op je apparaten
https-download-ca = hoard-ca.crt downloaden
https-trust-hint = Installeer de certificeringsinstantie van Hoard één keer op elk apparaat, en de browsers daarop vertrouwen deze server (en elk certificaat dat Hoard er later voor maakt). Het kan alleen voor deze Hoard instaan. Vingerafdruk (SHA-256):
https-ca-windows = Open het bestand → Certificaat installeren → Huidige gebruiker → Alle certificaten in het onderstaande archief opslaan → Bladeren → Vertrouwde basiscertificeringsinstanties → Voltooien. Start de browser opnieuw.
https-ca-macos = Open het bestand; het komt in Sleutelhangertoegang terecht. Dubbelklik daar op “{ $subject }” → Vertrouw → Bij gebruik van dit certificaat: Altijd vertrouwen.
https-ca-ios-title = iPhone en iPad
https-ca-ios = Download het in Safari en sta het toe. Instellingen → Profiel gedownload → Installeer. Daarna Instellingen → Algemeen → Info → Certificaatvertrouwensinstellingen → schakel “{ $subject }” in.
https-ca-android = Instellingen → Beveiliging → Meer beveiligingsinstellingen → Versleuteling en inloggegevens → Een certificaat installeren → CA-certificaat, en kies het bestand. (De namen verschillen een beetje per telefoon; zoek in Instellingen naar “certificaat”.)
https-ca-linux-1 = Debian en Ubuntu:{ " " }
https-ca-linux-2 = . Fedora en Arch:{ " " }
https-ca-linux-3 = . Chrome op Linux houdt een eigen lijst bij: Instellingen → Privacy en beveiliging → Beveiliging → Certificaten beheren → Importeren.
https-ca-firefox = Firefox houdt een eigen lijst bij: Instellingen → Privacy & Beveiliging → Certificaten → Certificaten bekijken → Certificaatautoriteiten → Importeren, en vink “Deze CA vertrouwen voor het identificeren van websites” aan.
https-ca-cli-title = De opdrachtregeltool hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , of zet het pad in{ " " }
https-ca-cli-after = .
https-options = Opties
https-redirect = Browsers op andere apparaten van http:// naar https:// sturen
https-redirect-hint = Alleen het laden van pagina's wordt doorgestuurd. De opdrachtregeltool hoard, de Unreal-worker, scripts en AI-apps blijven werken op http://, net als http://localhost op de server zelf.
https-http3-hint-before = Sneller op wisselvallige wifi en telefoons. Vereist HTTPS, en UDP-poort { $port } open naast TCP (in Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Actief.
https-http3-waits = Het start zodra HTTPS aan staat.
https-http3-failed = HTTP/3 kon niet starten: { $error }
https-lockout-title = Als je buitengesloten raakt
https-lockout-1 = { "" }
https-lockout-2 = { " " }op de server zelf werkt altijd via gewone http://. Of schakel HTTPS uit vanaf de opdrachtregel van de server en start Hoard opnieuw:{ " " }
https-lockout-3 = { " " }(in Docker:{ " " }
https-lockout-4 = , daarna{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Mag kijken
account-role-view-hint = Zoeken, voorbeelden bekijken en downloaden
account-role-edit = Mag wijzigen
account-role-edit-hint = Ook toevoegen, wijzigen, taggen, verzamelen en archiveren
account-role-admin = Beheerder
account-role-admin-hint = Ook Instellingen, accounts en onderhoud, en definitief verwijderen
account-source-internal = Een Hoard-wachtwoord
account-source-ldap = Hun { $directory }-wachtwoord
account-source-oauth = Inloggen met { $provider }
account-source-word-directory = directory
account-load-failed = Kon de accounts niet laden: { $error }
account-failed = { $what }: { $error }
account-password-failed = Kon je wachtwoord niet wijzigen
account-password-changed = Wachtwoord gewijzigd. Je andere apparaten zijn uitgelogd.
account-add-failed = Kon { $name } niet toevoegen
account-added = { $name } kan nu inloggen.
account-role-failed = Kon niet wijzigen wat { $name } mag
account-remove-title = { $name } verwijderen?
account-remove-body = Die persoon wordt overal uitgelogd en kan niet meer inloggen. Niets wat die aan de bibliotheek heeft toegevoegd, wordt verwijderd.
account-remove = Verwijderen
account-remove-failed = Kon { $name } niet verwijderen
account-token-failed = Kon het token niet aanmaken
account-copied = Gekopieerd
account-copy-token-failed = Kon niet kopiëren: selecteer het token en kopieer het zelf
account-copy-failed = Kon niet kopiëren: selecteer het en kopieer het zelf
account-revoke-title = “{ $name }” intrekken?
account-revoke-body = Alles wat het nog gebruikt, wordt vanaf nu geweigerd.
account-revoke = Intrekken
account-revoke-failed = Kon “{ $name }” niet intrekken
account-disconnect-title = { $app } loskoppelen?
account-disconnect-body = Het wordt vanaf nu geweigerd en moet opnieuw worden gekoppeld om Hoard te gebruiken.
account-disconnect = Loskoppelen
account-disconnect-failed = Kon { $app } niet loskoppelen
account-guest-failed = Kon dat niet wijzigen
account-never = nooit
account-you = Jij
account-sign-out = Uitloggen
account-signed-in-as-before = Ingelogd als{ " " }
account-signed-in-as-admin = , met beheerdersrechten.
account-signed-in-as-edit = , die de bibliotheek mag wijzigen.
account-signed-in-as-view = , die mag kijken maar niets mag wijzigen.
account-app-signin = De Hoard-app meldt je zelf aan, dus er is geen wachtwoord. Maak hieronder een toegangstoken voor de engine-plug-ins en de opdracht hoard.
account-password-at-provider = Je wachtwoord wordt bewaard door de provider waarmee je inlogt, dus daar wijzig je het.
account-password-at-directory = Je wachtwoord wordt bewaard door de directory waarmee je inlogt, dus daar wijzig je het.
account-current-password = Huidig wachtwoord
account-new-password = Nieuw wachtwoord
account-new-password-again = Nieuw wachtwoord nogmaals
account-change-password = Wachtwoord wijzigen
account-password-mismatch = De nieuwe wachtwoorden komen niet overeen.
account-password-rule = Minstens 8 tekens.
account-people = Mensen
account-people-hint = Iedereen mag kijken (zoeken, voorbeelden bekijken en downloaden), mag ook de bibliotheek wijzigen (toevoegen, bewerken, taggen, verzamelen en archiveren), of is daarnaast beheerder (Instellingen, accounts en onderhoud, en definitief verwijderen).
account-you-tag = (jij)
account-signs-in-oauth = Logt in met de OAuth-provider
account-signs-in-directory = Logt in met de directory
account-since = sinds { $date }
account-what-can-do = Wat { $name } mag
account-name = Naam
account-how-sign-in = Hoe die inlogt
account-password-label = Wachtwoord (minstens 8 tekens)
account-password-placeholder = Wachtwoord (8+ tekens)
account-what-they-can-do = Wat die mag
account-add = Toevoegen
account-ai-apps = Claude en andere AI-apps
account-ai-apps-hint = Koppel Claude (of een andere MCP-app) en die kan voor je assets zoeken en ophalen, en beschrijven en taggen wat Hoard nog niet heeft gedaan. De app logt de eerste keer in bij Hoard; je staat dat hier in de browser toe.
account-mcp-address = MCP-adres van Hoard
account-copy = Kopiëren
account-claude-code-1 = voer dit uit:{ " " }
account-claude-code-2 = , daarna{ " " }
account-claude-code-3 = { " " }om in te loggen.{ " " }
account-claude-code-4 = { " " }beschrijft een pakket.
account-claude-ai = Instellingen → Connectors → Aangepaste connector toevoegen, plak het adres en kies dan Verbinden. Deze maken verbinding vanaf de servers van Anthropic, dus Hoard moet via HTTPS bereikbaar zijn vanaf het internet.
account-claude-ai-local = Instellingen → Connectors → Aangepaste connector toevoegen, plak het adres en kies dan Verbinden. Deze maken verbinding vanaf de servers van Anthropic, dus Hoard moet via HTTPS bereikbaar zijn vanaf het internet (dit adres is dat niet: stel het Openbare adres in bij Instellingen → Verbinding).
account-connection-meta = voor { $user } sinds { $since } · laatst gebruikt { $used }
account-no-apps = Nog geen apps gekoppeld.
account-live = Gebruiken Hoard nu
account-live-hint = Apps die Hoard de afgelopen 10 minuten iets hebben gevraagd. Dit werkt zichzelf bij.
account-live-none = Niets heeft Hoard de afgelopen 10 minuten iets gevraagd.
account-live-last = laatst: { $what } · { $when } · verzoeken: { $calls }
account-tokens = Toegangstokens
account-tokens-hint-1 = Voor de opdrachtregeltool{ " " }
account-tokens-hint-2 = , scripts, MCP-clients en de Unreal-worker: die sturen het mee als{ " " }
account-tokens-hint-3 = . Een token kan wat jij kiest, tot wat jij mag, maar nooit accounts of tokens beheren. Eentje dat alleen mag kijken, is genoeg om te zoeken en te downloaden, en om een project te synchroniseren.
account-tokens-admin = Jij ziet de tokens en gekoppelde apps van iedereen; anderen zien hun eigen.
account-made-before = { "" }
account-made-after = : kopieer het nu. Het wordt niet nog eens getoond.
account-new-token = Het nieuwe token
account-cli-1 = Voor de CLI:{ " " }
account-cli-2 = , of zet het in{ " " }
account-cli-3 = .
account-done = Klaar
account-token-meta = aangemaakt { $made } · laatst gebruikt { $used }
account-token-meta-by = aangemaakt { $made } door { $by } · laatst gebruikt { $used }
account-no-tokens = Nog geen tokens.
account-token-name = Wat het gaat gebruiken
account-token-name-placeholder = Wat het gaat gebruiken, bijv. “Unreal-worker”
account-token-role = Wat het mag
account-make-token = Een token aanmaken
account-guests = Gasten
account-guests-allow = Iedereen die de server kan bereiken laten rondkijken zonder in te loggen
account-guests-hint = Ze kunnen zoeken, voorbeelden bekijken en downloaden, maar niets toevoegen, wijzigen of verwijderen, en Instellingen niet zien.

## SignIn.vue

signin-create-title = Maak je account aan
signin-title = Inloggen
signin-setup-lede = Deze server heeft nog geen account. Het account dat je hier aanmaakt, mag alles; voeg later accounts voor anderen toe via Instellingen → Account.
signin-overlay-lede = Rondkijken staat hier voor iedereen open, maar voor wijzigingen heb je een account nodig.
signin-with = Inloggen met { $provider }
signin-or-password = of met een naam en wachtwoord
signin-username = Gebruikersnaam
signin-name = Naam
signin-new-password = Nieuw wachtwoord
signin-password = Wachtwoord
signin-password-rule = Minstens 8 tekens.
signin-confirm-password = Wachtwoord bevestigen
signin-mismatch = De twee wachtwoorden komen niet overeen.
signin-setup-code = Installatiecode
signin-setup-code-hint-before = De code die deze server heeft gekregen in{ " " }
signin-setup-code-hint-after = .
signin-creating = Aanmaken…
signin-signing-in = Inloggen…
signin-create = Account aanmaken
signin-submit = Inloggen
signin-keep-browsing = Verder rondkijken
signin-directory-hint = Gebruik je { $directory }-naam en -wachtwoord.
signin-forgot-1 = Een Hoard-wachtwoord vergeten? Voer op de server{ " " }
signin-forgot-2 = { " " }uit (in Docker{ " " }
signin-forgot-3 = ) voor een nieuw wachtwoord.

## SetupWizard.vue (first visit to a new server)

setup-title = Waar moet je bibliotheek komen?
setup-lede = Kies een map op de server waarin Hoard je pakketten bewaart, of wijs het een map met pakketten aan die je al hebt. Je kunt later via Instellingen meer locaties toevoegen, op andere schijven.
setup-lede-desktop = Kies een map waarin Hoard je pakketten bewaart, of wijs het een map met pakketten aan die je al hebt. Je kunt later via Instellingen meer locaties toevoegen, op andere schijven.
setup-unreachable = Kon de server niet bereiken: { $error }
setup-unreachable-desktop = Hoard reageert niet: { $error }
setup-try-again = Opnieuw proberen
setup-loading = Laden…
setup-start = De bibliotheek starten
setup-ready = Je bibliotheek is klaar
setup-managed-before = Nieuwe pakketten gaan naar{ " " }
setup-managed-after = . Zet ergens op de pagina bestanden, mappen of archieven neer om je eerste toe te voegen.
setup-watched-before = Hoard bewaakt{ " " }
setup-watched-after = ; pakketten verschijnen zodra het ze vindt. Om ook te kunnen uploaden, voeg je via Instellingen → Bibliotheeklocaties een bibliotheek toe die Hoard zelf bewaart.
setup-open = De bibliotheek openen

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = { $app } koppelen?
connect-lede-1 = { " " }kan alles in deze Hoard doorzoeken, downloaden, beschrijven en taggen, namens{ " " }
connect-lede-2 = . Het kan geen accounts of toegangstokens beheren.
connect-back-before = Je gaat terug naar{ " " }
connect-back-after = . Je kunt het altijd loskoppelen in Instellingen → Account.
connect-allow = Toestaan
connect-cancel = Annuleren
connect-failed = Kan deze app niet koppelen
connect-go-library = Naar de bibliotheek
connect-checking = Controleren…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = Dit is gewone http://, dus een paar dingen hebben HTTPS nodig: 3D met WebGPU, het moderne klembord, Hoard als app installeren en Claude-connectors.
insecure-switch = Overschakelen naar https://
insecure-set-up = HTTPS instellen
insecure-dismiss = Dit niet meer tonen

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Dit bestand is te groot om als storebestand te worden gelezen.
formatwizard-not-json = Dit bestand is geen geldige JSON, dus het wordt als tekst getoond.
formatwizard-read-failed = Kon het bestand niet lezen: { $error }
formatwizard-back-title = Terug naar het formulier?
formatwizard-back-body = Wijzigingen in de TOML gaan verloren: het formulier schrijft het formaat opnieuw op basis van wat het heeft.
formatwizard-back-to-form = Terug naar het formulier
formatwizard-miss-name = De bestanden ervan heten nooit { $file }. Voeg het toe bij Bestanden.
formatwizard-miss-not-the-stores = Waaraan het de bestanden van de store herkent, staat niet in dit bestand. Wijzig Herkennen aan.
formatwizard-miss-no-title = Het vindt geen titel in dit bestand. Kies de titel.
formatwizard-miss-short-name = niet de naam
formatwizard-miss-short-not-the-stores = niet van de store
formatwizard-miss-short-no-title = geen titel
formatwizard-saved-packs =
    { $file } opgeslagen. { $packs ->
        [one] { $packs } pakket wordt
       *[other] { $packs } pakketten worden
    } opnieuw gelezen.
formatwizard-saved = { $file } opgeslagen.
formatwizard-save-failed = Kon het formaat niet opslaan: { $error }
formatwizard-zip-failed = Kon de zip niet maken: { $error }
formatwizard-stop-title = Stoppen met dit bestand aanleren?
formatwizard-stop-body = Het formaat is niet opgeslagen.
formatwizard-stop = Stoppen
formatwizard-label = Formaatwizard
formatwizard-changing = { $name } wijzigen
formatwizard-starting-from = Beginnen vanaf { $name }
formatwizard-title = Hoard het bestand van een store aanleren
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }in{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Bewerken als TOML
formatwizard-file-reference = Het bestand, ter referentie.
formatwizard-click-json = Klik op de { $field } in het bestand.
formatwizard-click-text = Klik op de { $field } in het bestand, of selecteer die.
formatwizard-pick-first = Kies rechts een veld en klik dan hier op de waarde ervan.
formatwizard-loading = Laden…
formatwizard-the-format = Het formaat
formatwizard-syntax-before = De syntaxis staat in{ " " }
formatwizard-syntax-after = .
formatwizard-store = Store
formatwizard-store-placeholder = De naam van de store, zoals Kenney
formatwizard-files = Bestanden
formatwizard-recognize = Herkennen aan
formatwizard-recognize-json = Een sleutel die alleen de bestanden van de store hebben
formatwizard-recognize-text = Een regel die alleen de bestanden van de store hebben
formatwizard-recognize-hint = Eén manier per regel; elk ervan volstaat. Nodig als andere bestanden dezelfde naam hebben.
formatwizard-seller = Verkoper
formatwizard-seller-placeholder = Als het bestand het niet zegt
formatwizard-tags = Tags
formatwizard-tags-placeholder = Voor alles wat het beschrijft, gescheiden door komma's
formatwizard-fields = Velden
formatwizard-selector = Selector voor { $field }
formatwizard-clear = { $field } wissen
formatwizard-pick = Kiezen
formatwizard-what-it-reads = Wat het leest
formatwizard-read-title = Titel
formatwizard-read-seller = Verkoper
formatwizard-read-license = Licentie
formatwizard-read-url = Storepagina
formatwizard-read-kind = Soort
formatwizard-read-tags = Tags
formatwizard-read-origin = Gemaakt in
formatwizard-read-size = Grootte
formatwizard-also = { $format } leest dit bestand ook. Dit formaat zou eerst worden geprobeerd.
formatwizard-library =
    { $files ->
        [one] { $files } bestand
       *[other] { $files } bestanden
    } in { $packs ->
        [one] { $packs } pakket
       *[other] { $packs } pakketten
    } { $files ->
        [one] heeft
       *[other] hebben
    } een naam die het aanneemt. Van de { $tried } geprobeerde leest het er { $read }.
formatwizard-library-none = Geen enkel bestand in de bibliotheek heeft een naam die het aanneemt.
formatwizard-share = Delen
formatwizard-share-hint = Een zip met het formaat, dit bestand als voorbeeld en hoe dat voorbeeld wordt gelezen, ingedeeld zoals de repository van Hoard, om in te sturen zodat iedereen het krijgt. Haal eerst alles uit het voorbeeld wat niet gedeeld mag worden.
formatwizard-sample = Voorbeeld om te delen
formatwizard-download = { $file } downloaden
formatwizard-close = Sluiten
formatwizard-use = Hier gebruiken
formatwizard-cancel = Annuleren
settings-unreal-editor-what = Installeert Unreal Editor voor het exportproces. Start het proces apart om assets te exporteren.
settings-unreal-download = Downloadt Epics Linux x86-64-editor eenmalig naar de gereedschapsmap van Hoard. Hij neemt tientallen GB schijfruimte in en vereist een GitHub-account dat Epic heeft toegelaten (zie GitHub-aanmelding).
settings-unreal-github = GitHub-aanmelding
settings-unreal-github-why = Epic publiceert de Linux-editor als privé-image in het pakketregister van GitHub (ghcr.io), en Hoard downloadt hem daarvandaan met crane, dat bij Hoard wordt meegeleverd. GitHub geeft hem alleen aan accounts die Epic heeft toegelaten, dus Hoard heeft er een nodig om hem mee te downloaden. Hoard bewaart het token bij zijn andere instellingen en gebruikt het nergens anders voor.
settings-unreal-github-step-link = Koppel je GitHub-account aan je Epic Games-account.
settings-unreal-github-step-link-go = Zo koppel je ze
settings-unreal-github-step-invite = Accepteer de uitnodiging voor de organisatie EpicGames die GitHub je mailt (het kan een paar minuten duren voordat die aankomt).
settings-unreal-github-step-token = Maak een GitHub-token (classic) aan met alleen read:packages aangevinkt.
settings-unreal-github-step-token-go = Token aanmaken
settings-unreal-github-step-enter = Vul hier je GitHub-gebruikersnaam en het token in en kies dan “Automatisch”.
settings-unreal-github-user = GitHub-gebruikersnaam
settings-unreal-github-token = Token
settings-unreal-github-token-kept = Opgeslagen (typ een nieuw om het te vervangen)
settings-unreal-github-save = Aanmelding opslaan
settings-unreal-github-forget = Vergeten
settings-unreal-github-saved = GitHub-aanmelding opgeslagen; de download wordt opnieuw geprobeerd
settings-unreal-github-forgotten = GitHub-aanmelding vergeten
settings-unreal-github-from-server = Deze server heeft ook een eigen aanmelding gekregen (GHCR_TOKEN_FILE of DOCKER_CONFIG); die wordt gebruikt als hier niets is opgeslagen.
settings-unreal-image = Unreal-versie of image
settings-editor-path = Voer het pad naar het uitvoerbare bestand van je geïnstalleerde { $tool } in. Automatische installatie is alleen beschikbaar in Docker.
error-unreal-image = Voer een Unreal 5-versie, een Epic dev/dev-slim-versietag of een Epic SHA-256-imagereferentie in.
