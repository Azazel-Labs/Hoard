## Palvelimen virheet

error-account-not-found = tiliä ei löydy
error-asset-gone = resurssia ei löydy
error-asset-not-found = resurssia { $id } ei löydy
error-blob-not-found = blobia { $hash } ei löydy
error-collection-not-found = kokoelmaa { $id } ei löydy
error-connection-not-found = yhteyttä ei löydy
error-derivative-not-found = resurssin { $id } johdettua tiedostoa ei löydy (tyyppi: { $kind })
error-file-in-pack-not-found = tiedostoa { $path } ei löydy paketista { $id }
error-site-not-a-page = vain verkkosivut avautuvat sivuina
error-site-link-expired = tämän sivun osoite on vanhentunut; avaa sivu uudelleen Hoardista
error-format-not-found = muotoa { $file } ei löydy
error-frame-sheet-aseprite-only = vain Aseprite-tiedostoilla on ruutuarkki
error-frame-sheet-frames = tässä on yksi ruutu tai liian monta asetettavaksi arkkiin
error-location-not-found = sijaintia { $id } ei löydy
error-maintenance-task-not-found = ylläpitotehtävää { $id } ei löydy
error-open-upload-not-found = avointa lähetystä { $id } ei löydy
error-original-not-found = resurssin { $id } alkuperäistä tiedostoa ei löydy
error-pack-not-found = pakettia { $id } ei löydy
error-preview-too-big = kuva on liian suuri kevyen esikatselun tekemiseen
error-project-not-found = projektia { $name } ei löydy
error-saved-search-not-found = tallennettua hakua { $id } ei löydy
error-sign-in-first = kirjaudu ensin sisään (tai lähetä käyttöoikeustunnus)
error-those-assets-not-found = resursseja ei löydy
error-thumbnail-not-found = resurssin { $id } pikkukuvaa ei löydy
error-token-not-found = tunnusta ei löydy
error-upload-not-found = lähetystä { $id } ei löydy
error-pack-needs-name = paketti tarvitsee nimen
error-no-library = kirjastossa ei ole vielä sijaintia tiedostojen tallentamiseen: lisää hallittu sijainti kohdassa Asetukset → Kirjaston sijainnit
error-upload-failed = lähetys epäonnistui: { $reason }
error-upload-no-files = lähetyksessä ei ole tiedostoja
error-analyze-nothing-to-do = ei tehtävää: rules=false ja LLM on pois päältä
error-too-many-assets = liian monta resurssia yhdessä pyynnössä
error-too-many-items = liian monta kohdetta yhdessä pyynnössä
error-not-a-hash = ei ole blake3-heksatiiviste
error-merge-into-tag = into-kohteen on oltava tunniste
error-nothing-to-merge = ei yhdistettävää
error-selection-size = anna 1–{ $max } resurssin tunnistetta
error-collection-needs-name = kokoelma tarvitsee nimen
error-collection-exists = kokoelma nimeltä { $name } on jo olemassa
error-collection-empty = kokoelma { $name } on tyhjä
error-conversion-failed = muunnos muotoon { $format } epäonnistui: { $reason }
error-max-size = max-arvon on oltava kahden potenssi väliltä 64–16384
error-replace-ext = ext-arvon on oltava lyhyt tiedostopääte
error-empty-file = tyhjä tiedosto
error-imitation-silent = Mikään nauhoitteessa ei erotu taustasta. Yritä uudelleen lähempänä mikrofonia.
error-replace-duplicate = muunnettu tiedosto on sama kuin paketissa jo oleva tiedosto; paketti yhdistettiin siihen
error-ai-address = Tekoälyn osoitteen pitää alkaa http:// tai https://
error-hosted-key = Anna palvelun { $provider } API-avain
error-hosted-address = Kuvahakupalvelun osoitteen pitää alkaa http:// tai https://
error-local-llm-action = action on list, download tai delete, ja kaksi viimeistä tarvitsevat mallin
error-role = Rooli on view, edit tai admin
error-admin-only = vain ylläpitäjä voi tehdä tämän
error-view-only = tällä tilillä voi katsella mutta ei muuttaa mitään: ylläpitäjä voi muuttaa tämän kohdassa Asetukset → Tili
error-portal-only = kirjaudu portaaliin hallitaksesi tilejä ja tunnuksia
error-not-from-portal = tämä pyyntö ei tullut Hoard-portaalista
error-account-name = Valitse enintään 64 merkin nimi
error-password-length = Käytä vähintään { $min } merkin salasanaa
error-too-many-codes = Liian monta väärää koodia: odota { $seconds } s
error-already-set-up = Tili on jo olemassa: kirjaudu sisään
error-too-many-tries = Liian monta väärää yritystä: odota { $seconds } s
error-directory-unavailable = Hakemisto ei voinut tarkistaa salasanaasi: { $reason }
error-password-elsewhere = Salasanaasi säilyttää hakemisto tai palveluntarjoaja, jonka kautta kirjaudut
error-current-password = Nykyinen salasanasi ei ole oikein
error-account-source = Tili kirjautuu Hoardin kautta (internal), hakemiston kautta (ldap) tai OAuthilla
error-account-exists = Samanniminen tili on jo olemassa
error-own-role = Toinen ylläpitäjä voi muuttaa roolisi
error-remove-self = Et voi poistaa omaa tiliäsi, kun olet kirjautuneena siihen
error-token-name = Nimeä tunnus sen mukaan, mikä sitä käyttää, enintään 80 merkillä
error-token-role = Tunnus ei voi tehdä enempää kuin sen luova tili
error-wrong-setup-code = Tämä ei ole palvelimelle HOARD_SETUP_CODE-muuttujassa annettu asennuskoodi
error-wrong-password = Nimi ja salasana eivät täsmää
error-folder-missing = { $path } puuttuu (onko levy liittämättä?)
error-s3-path = S3-sijainti on muotoa s3://bucket
error-s3-address = S3-osoitteen alun on oltava http:// tai https://
error-s3-header = ylimääräinen S3-otsake on muotoa Name=value, eikä ”{ $header }” ole sellainen
error-keychain = sitä ei voitu tallentaa järjestelmän avainnippuun: { $reason }
error-not-built = tämä Hoardin kopio on käännetty ilman { $feature } -tukea
error-openai-auth-unreachable = auth.openai.com ei vastaa
error-openai-unexpected = odottamaton vastaus OpenAI:lta
error-openai-answered = OpenAI vastasi { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID-tunnus ei ole JWT
error-chatgpt-id-token-issuer = ID-tunnuksen myöntäjä on { $issuer }, ei { $expected }
error-chatgpt-id-token-audience = ID-tunnus on tarkoitettu toiselle sovellukselle
error-chatgpt-id-token-expired = ID-tunnus on vanhentunut
error-chatgpt-id-token-nonce = ID-tunnus ei kuulu tähän kirjautumiseen
error-chatgpt-no-state = osoitteessa ei ole state-arvoa; kopioi koko osoite
error-chatgpt-not-started = tätä kirjautumista ei aloitettu täällä, tai se aloitettiin yli 15 minuuttia sitten. Aloita se uudelleen asetuksista.
error-chatgpt-canceled = kirjautuminen peruttiin
error-chatgpt-refused = ChatGPT hylkäsi kirjautumisen: { $reason }
error-chatgpt-no-code = osoitteessa ei ole kirjautumiskoodia; kopioi koko osoite
error-chatgpt-other-registration = ChatGPT vastasi tämän palvelimen toiselle rekisteröinnille; aloita kirjautuminen uudelleen
error-chatgpt-other-registration-desktop = ChatGPT vastasi Hoardin toiselle kopiolle; aloita kirjautuminen uudelleen
error-chatgpt-not-registered = ChatGPT ei saanut Hoardin rekisteröintiä valmiiksi; aloita kirjautuminen uudelleen
error-chatgpt-code-used = kirjautumiskoodi on jo käytetty tai vanhentunut; kirjaudu uudelleen
error-chatgpt-refused-code = ChatGPT hylkäsi kirjautumisen ({ $code })
error-chatgpt-no-id-token = ChatGPT ei lähettänyt ID-tunnusta
error-chatgpt-plan-not-allowed = Kirjautuminen onnistui, mutta tilauksen käyttöä ei sallittu. Kirjaudu uudelleen ja salli se, tai käytä API-avainta.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = ChatGPT:hen ei ole kirjauduttu; kirjaudu kohdassa Asetukset → Tekoälytunnisteet
error-chatgpt-refresh-later = ChatGPT ei voinut juuri nyt uusia kirjautumista ({ $code }); yritetään myöhemmin uudelleen
error-chatgpt-signed-out-problem = ChatGPT kirjasi tämän palvelimen ulos ({ $code }). Kirjaudu uudelleen, jotta tunnisteiden lisäys tilauksellasi jatkuu.
error-chatgpt-signed-out-problem-desktop = ChatGPT kirjasi Hoardin ulos ({ $code }). Kirjaudu uudelleen, jotta tunnisteiden lisäys tilauksellasi jatkuu.
error-chatgpt-signed-out = ChatGPT kirjasi tämän palvelimen ulos ({ $code }); kirjaudu uudelleen kohdassa Asetukset → Tekoälytunnisteet
error-chatgpt-signed-out-desktop = ChatGPT kirjasi Hoardin ulos ({ $code }); kirjaudu uudelleen kohdassa Asetukset → Tekoälytunnisteet
error-openai-api-unreachable = api.openai.com ei vastaa
error-openai-broke-off = OpenAI:n vastaus katkesi
error-chatgpt-kept-refusing = OpenAI hylkäsi ChatGPT-kirjautumisen toistuvasti; kirjaudu uudelleen kohdassa Asetukset → Tekoälytunnisteet
error-chatgpt-request-failed = pyyntö epäonnistui
error-chatgpt-cut-off = vastaus katkesi ({ $reason })
error-chatgpt-cut-off-no-reason = vastaus katkesi (syytä ei kerrottu)
error-openai-ended-early = OpenAI:n vastaus päättyi ennen kuin se oli valmis
error-chatgpt-usage-limit = ChatGPT-tilauksesi käyttökiintiö Hoardille on toistaiseksi lopussa. Katso tai muuta rajaa osoitteessa { $url }
error-chatgpt-not-eligible = tämä ChatGPT-tili tai -työtila ei voi käyttää tilaustaan muissa sovelluksissa. Käytä API-avainta.
error-chatgpt-usage-unavailable = ChatGPT ei voinut juuri nyt tarkistaa tilauksen käyttöä; yritetään myöhemmin uudelleen
error-chatgpt-unsupported = ChatGPT-tilauksen käyttö ei tue tällä mallilla parametria { $param }
error-chatgpt-unsupported-images = ChatGPT-tilauksen käyttö ei tue tällä mallilla parametria { $param }; valitse kuvia vastaanottava malli tai poista pikkukuvien lähetys käytöstä
error-chatgpt-unsupported-part = ChatGPT-tilauksen käyttö ei tue tällä mallilla osaa pyynnöstä
error-chatgpt-port = portti on se, jonka kautta selain tavoittaa Hoardin
error-chatgpt-paste-address = liitä koko osoite, joka alkaa http://127.0.0.1
error-too-many-hashes = enintään { $max } tiivistettä kerrallaan
error-credits-pack = paketti { $key }
error-credits-collection = kokoelma { $key }
error-format-too-long = muoto on liian pitkä
error-not-started = palvelin ei ole käynnistynyt
error-not-started-desktop = Hoard ei ole käynnistynyt
error-not-format-file = ei ole muodon tiedosto: { $name }
error-format-name = nimessä on oltava kirjain tai numero väliltä A–Z tai 0–9
error-format-miss-name = muodon tiedostot eivät hyväksy tiedostoa { $file }
error-format-miss-store = muoto ei hyväksy tiedostoa { $file } kaupan tiedostoksi (katso require)
error-format-miss-title = muoto ei löydä otsikkoa tiedostosta { $file }
error-cant-download-as = .{ $ext }-tiedostoja ei voi ladata muodossa { $format }
error-only-images-resize = vain kuvien kokoa voi muuttaa
error-https-no-certificate = Liitä tai lähetä ensin varmenteesi ja sen avain
error-https-no-cert-found = Varmennetta ei löytynyt: liitä teksti, joka alkaa -----BEGIN CERTIFICATE-----
error-https-no-key-found = Yksityistä avainta ei löytynyt: liitä teksti, joka alkaa -----BEGIN PRIVATE KEY-----
error-https-several-keys = Yksityisiä avaimia on useampi kuin yksi: liitä vain varmenteeseen kuuluva
error-https-key-password = Avain on suojattu salasanalla: anna se ilman salasanaa (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Varmenne ei ole kelvollista PEM-muotoa
error-https-key-not-pem = Yksityinen avain ei ole kelvollista PEM-muotoa
error-https-key-mismatch = Avain ei kuulu varmenteeseen. Jos annoit ketjun, laita palvelimesi varmenne ensimmäiseksi
error-https-unusable = Varmennetta tai avainta ei voi käyttää
error-https-expired = Tämä varmenne vanheni { $date }
error-https-not-yet-valid = Tämä varmenne on voimassa vasta { $date } alkaen
error-https-unreadable = Hoard ei pysty lukemaan tätä varmennetta
error-https-bad-name = ”{ $name }” ei ole osoite, jonka varmenne voi sisältää
error-https-test-timed-out = testiyhteys aikakatkaistiin
error-https-test-failed = testiyhteys epäonnistui
error-https-no-ca = Hoard ei ole vielä luonut varmentajaansa
error-https-unavailable = HTTPS ei ole käytettävissä tällä palvelimella
error-https-certificate-mode = Varmenne kuuluu valintaan ”Oma varmenne”
error-https-test = Varmenne ei toiminut testiyhteydessä: { $reason }
error-nothing-to-retry = mitään ei voi yrittää uudelleen
error-ai-unreachable = { $url } ei vastaa
error-ai-key-refused = { $url } hylkäsi API-avaimen ({ $status })
error-ai-answered = { $url } vastasi { $status }: { $reason }
error-ai-not-json = { $url }/models ei vastannut JSON-muodossa
error-model-server-refused = mallipalvelin: { $reason }
error-no-model-server = osoitteessa { $url } ei vastaa mallipalvelinta
error-not-llama-router = { $url } ei ole router-tilassa toimiva llama.cpp-palvelin, joten se ei voi ladata malleja
error-model-download = mallin { $model } lataus epäonnistui: { $reason }
error-model-delete = mallin { $model } poisto epäonnistui: { $reason }
error-unknown-kind = tuntematon resurssityyppi { $kind }
error-watched-is-folder = valvottu sijainti on kansio tällä palvelimella
error-cant-reach = { $path } ei ole tavoitettavissa: { $reason }
error-folder-full-path = anna kansion koko polku
error-cant-create = ei voi luoda: { $path }
error-cant-write = ei voi kirjoittaa: { $path }
error-cant-read = ei voi lukea: { $path }
error-overlaps-location = tämä menee päällekkäin sijainnin { $name } kanssa ({ $path })
error-overlaps-data = tämä menee päällekkäin Hoardin datakansion kanssa ({ $path })
error-overlaps-cache = tämä menee päällekkäin Hoardin välimuistikansion kanssa ({ $path })
error-not-watched = { $name } ei ole valvottu kansio
error-watched-no-kinds = valvottu kansio ei ota vastaan uusia tiedostoja, joten sillä ei ole muutettavia tyyppejä
error-keep-catch-all = tee sen sijaan jostain toisesta sijainnista kaiken vastaanottava sijainti
error-location-holds-files = sijainnissa { $name } on yhä tiedostoja: siirrä ne ensin muihin sijainteihin (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = sijaintia { $name } ei tavoiteta, joten Hoard ei voi tietää, onko siinä yhä tiedostoja
error-relocate-running = tiedostoja siirretään jo sijaintien välillä; yritä uudelleen, kun se on valmis
error-watched-not-moved = { $name } on valvottu kansio: Hoard ei siirrä sen tiedostoja
error-no-other-location = kirjastossa ei ole muuta sijaintia, johon tiedostot voisi siirtää: lisää ensin sellainen
error-full-path = anna koko polku
error-pick-folder = valitse kansio
error-task-running = ”{ $task }” on jo käynnissä
error-token-account-gone = tämän tunnuksen tiliä ei enää ole, joten sillä ei ole suosikkeja eikä hakuja
error-favorites-too-many = enintään 10 000 kerrallaan
error-saved-search-name = tallennettu haku tarvitsee enintään 80 merkin nimen
error-saved-search-too-long = haku on liian pitkä tallennettavaksi
error-hoard-seen = aarre voi tallentaa enintään 32 resurssilajia, kunkin nimettynä pienin kirjaimin
error-texture-pattern-map = Tekstuurin nimimallissa on oltava { "{" }map{ "}" }, muuten materiaalin jokainen kartta saisi saman nimen
error-texture-pattern-fields = Tekstuurin nimimallissa voi käyttää kenttiä { "{" }name{ "}" }, { "{" }map{ "}" } ja { "{" }size{ "}" }
error-texture-name-chars = Tekstuurien nimissä ei voi olla merkkejä < > : " / \ | ? tai *
error-texture-map-type = Karttatyyppiä { $role } ei ole
server-role-base-color = väri
server-role-normal = normaali
server-role-normal-dx = normaali (DirectX)
server-role-roughness = karheus
server-role-gloss = kiilto
server-role-metalness = metallisuus
server-role-ao = ympäristövarjostus (AO)
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = korkeus
server-role-bump = kohouma
server-role-cavity = kolot
server-role-specular = heijastavuus
server-role-opacity = peittävyys
server-role-emissive = hehku
server-role-translucency = läpikuultavuus
error-texture-suffix-missing = Anna pääte karttatyypille { $map }
error-texture-suffix-chars = Karttatyypin { $map } päätteessä ei voi olla merkkejä < > : " / \ | ? tai *
error-texture-ending-missing = Jokainen oma loppuosa tarvitsee etsittävän loppuosan
error-texture-ending-chars = Omissa loppuosissa ei voi olla merkkejä < > : " / \ | ? tai *
error-texture-ending-twice = { $ending } on loppuosissasi kahdesti
error-oauth-no-client-id = Tässä linkissä ei ole client_id-arvoa.
error-oauth-unknown-app = Tätä sovellusta ei ole (enää) rekisteröity tänne. Aloita yhdistäminen uudelleen sovelluksesta.
error-oauth-redirect = Sovellus pyysi palautusta osoitteeseen, jota se ei rekisteröinyt.
error-oauth-response-type = Hoard antaa vain kirjautumiskoodeja (response_type=code).
error-oauth-pkce = Sovelluksen on käytettävä PKCE:tä ja S256:ta kirjautuakseen Hoardiin.
error-light-preview-images = vain kuvilla on kevyt esikatselu
error-no-light-preview = ei kevyttä esikatselua: { $reason }
error-no-frame-sheet = ei ruutuarkkia: { $reason }
error-thumbnail-source = tuntematon pikkukuvan lähde: { $source }
error-thumbnail-too-big = kuva on liian suuri pikkukuvaksi
error-thumbnail-models-only = vain malleille voi piirtää pikkukuvan selaimessa
error-not-a-picture = Hoard ei pysty lukemaan kuvaa: { $reason }
error-project-name = projekti tarvitsee enintään 200 merkin nimen
error-project-too-big = enintään miljoona resurssia yhdessä projektissa
error-project-too-many-at-once = enintään 100 000 kerrallaan
error-server-name = Valitse enintään { $max } merkin nimi yhdelle riville
error-tool-setting = { $tool }: valitse pois, automaattinen tai anna polku
error-ai-model = Valitse malli, jota tekoälypalvelu käyttää
error-public-address = Julkisen osoitteen pitää alkaa http:// tai https://
error-update-hook-address = Päivityksiä varten kutsuttavan osoitteen pitää alkaa http:// tai https://
error-update-hook-auth = Authorization-otsakkeessa ei voi olla rivinvaihtoja tai muita ohjausmerkkejä
error-no-soundfont = Palvelimella ei ole SoundFontia polussa { $path }
error-no-soundfont-desktop = Polussa { $path } ei ole SoundFontia
error-soundfont-bad = { $name } ei ole SoundFont, jolla MIDI voi soida: { $reason }
error-workers = Taustatyöntekijät: 1–{ $max }
error-ai-batch = Resursseja tekoälypyyntöä kohden: 1–200
error-conversion-cache = Muunnetut lataukset: vähintään 100 Mt
error-backup-keep = Ajastetut varmuuskopiot: säilytä 1–{ $max }
error-backup-copy-to = Ajastetut varmuuskopiot: { $path } ei ole kansio palvelimella (anna koko polku)
error-conversion-keep-days = Muunnossäännöt: alkuperäisiä voi säilyttää 1–{ $max } päivää
error-conversion-target = Muunnossäännöt: Hoard ei osaa muuntaa tiedostoja muotoon { $format }
error-conversion-no-source = Muunnossäännöt: jokaisessa säännössä on oltava vähintään yksi muunnettava muoto
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Muunnossäännöt: .{ $ext }-tiedostoja ei voi muuntaa muotoon { $format }
error-conversion-twice = Muunnossäännöt: .{ $ext } on kahdessa säännössä
error-conversion-cant-undo = Tätä tiedostoa ei muunnettu säännöllä, tai sen alkuperäinen on poistettu.
error-conversion-path-taken = Polussa { $path } on nyt toinen tiedosto, joten alkuperäistä ei voi palauttaa sinne.
error-limits-zero = Rajat eivät voi olla nolla
error-settings-object = odotettiin asetusten JSON-objektia
error-bad-setting = virheellinen asetus: { $reason }
error-auth-default-role = Oletusrooli on view, edit tai admin
error-auth-session-hours = Istunnot kestävät 1 tunnista vuoteen
error-auth-settings = virheelliset todennusasetukset: { $reason }
error-ldap-settings = virheelliset LDAP-asetukset: { $reason }
error-ldap-no-server = Lisää hakemistopalvelimen osoite
error-ldap-base-dn = Kerro, missä käyttäjien merkinnät ovat (base DN)
error-ldap-username-attribute = Kerro, mikä attribuutti sisältää nimen, jolla käyttäjät kirjautuvat
error-ldap-bind-dn = Anna tili, joka hakee käyttäjiä, tai hae heitä anonyymisti
error-ldap-ca-cert = Varmentaja ei ole PEM-varmenne (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Hakemistopalvelimen osoite on tyhjä
error-ldap-server-form = { $server }: hakemistopalvelimen osoite on muotoa host:port, ldap://… tai ldaps://…
error-ldap-not-starttls = { $server }: tämä on LDAPS, ei StartTLS
error-ldap-no-certificate = siinä ei ole varmennetta
error-ldap-password-again = Anna käyttäjiä hakevan tilin salasana uudelleen: tallennettu salasana lähetetään vain palvelimille, joita varten se annettiin
error-ldap-unreachable = yhtään hakemistopalvelinta ei tavoitettu ({ $reasons })
error-ldap-no-anonymous = hakemisto ei salli käyttäjien anonyymiä hakua ({ $reason })
error-ldap-bind-failed = käyttäjiä hakeva tili ei voinut kirjautua ({ $reason })
error-ldap-search = haetaan nimeä { $name } kohdasta { $base }
error-ldap-group-search = haetaan käyttäjän { $name } ryhmiä kohdasta { $base }
error-ldap-several =
    { $count ->
        [one] { $count } hakemiston merkintä vastaa nimeä { $name }: rajaa käyttäjäsuodatinta
       *[other] { $count } hakemiston merkintää vastaa nimeä { $name }: rajaa käyttäjäsuodatinta
    }
error-ldap-password-check = hakemisto ei suostunut tarkistamaan käyttäjän { $name } salasanaa ({ $reason })
error-ldap-wrong-password = Hakemisto ei tunne käyttäjää { $name } tällä salasanalla
error-oauth-client-id = Anna client ID, jonka palveluntarjoaja loi Hoardille
error-oauth-addresses = Valtuutus- ja token-osoitteiden pitää alkaa https://
error-oauth-userinfo-address = Käyttäjätieto-osoitteen pitää alkaa https://
error-oauth-username-claim = Kerro, mikä claim sisältää henkilön nimen
error-oauth-secret-again = Anna client secret uudelleen: tallennettu lähetetään vain token-osoitteeseen, jota varten se annettiin
error-oauth-issuer = Myöntäjän (issuer) pitää alkaa https://
error-oauth-discovery-unreachable = { $url } ei vastaa: { $reason }
error-oauth-discovery-status = { $url } vastasi { $status }
error-oauth-discovery-bad = { $url } ei ole discovery-dokumentti
error-oauth-discovery-incomplete = { $url } ei kerro valtuutus- ja token-osoitteita
error-signin-no-group = { $name } ei kuulu ryhmään, jolla on oikeus käyttää Hoardia
error-signin-other-source = Hoardissa on jo tili nimeltä { $name }, joka kirjautuu toisella tavalla
error-signin-other-subject = Hoardissa on jo tili nimeltä { $name }, joka kuuluu palveluntarjoajalla toiselle henkilölle
error-signin-no-account = Käyttäjällä { $name } ei ole vielä Hoard-tiliä: ylläpitäjä voi lisätä sen kohdassa Asetukset → Tili
error-signin-bad-name = Tämä nimi ei kelpaa Hoard-tilin nimeksi (enintään 64 merkkiä)
error-oauth-off = Kirjautuminen OAuthilla ei ole käytössä
error-oauth-bad-authorization-address = valtuutusosoite ei ole kelvollinen
error-oauth-refused = Palveluntarjoaja ei kirjannut sinua sisään: { $reason }
error-oauth-no-code = Palveluntarjoaja palautti sinut ilman kirjautumista
error-oauth-expired = Kirjautuminen kesti liian kauan tai se aloitettiin muualla: yritä uudelleen
error-oauth-no-claim = Palveluntarjoaja ei kertonut, kuka olet (ei { $claim }-claimia)
error-oauth-no-sub = Palveluntarjoaja ei kertonut, kuka olet (ei sub- tai id-claimia)
error-oauth-token-unreachable = palveluntarjoajan token-osoite ei vastaa
error-oauth-token-refused = Palveluntarjoaja ei suostunut viemään kirjautumista loppuun: { $reason }
error-oauth-no-access-token = palveluntarjoaja ei lähettänyt käyttöoikeustunnusta
error-oauth-userinfo-unreachable = palveluntarjoajan käyttäjätieto-osoite ei vastaa
error-oauth-userinfo-refused = Palveluntarjoaja ei suostunut kertomaan, kuka kirjautui ({ $status })
error-oauth-userinfo-not-json = palveluntarjoajan käyttäjätiedot eivät ole JSON-muodossa
error-format-no-name = name on tyhjä
error-format-no-files = files on tyhjä: kerro, mitä tiedostoja muoto lukee, esim. ["*.json"]
error-format-empty-selector = tyhjä
error-format-unopened-bracket = sulkeva hakasulje ilman avaavaa
error-format-unclosed-bracket = hakasulje on sulkematta
error-format-empty-step-dots = tyhjä askel (kaksi pistettä peräkkäin tai piste alussa tai lopussa)
error-format-unclosed-except = !( on sulkematta
error-format-unclosed-filter = [ on sulkematta
error-format-after-filters = { $rest } askeleen { $step } avaimen ja suodattimien jälkeen
error-format-except-needs-glob = !(…) tulee *-merkin sisältävän avaimen jälkeen
error-format-except-not-glob = !(…) tulee *-merkin sisältävän avaimen jälkeen, ei avaimen { $key }
error-format-empty-step = tyhjä askel
error-format-not-a-filter = [{ $filter }] ei ole suodatin: käytä [], [key=value|other], [array], [string], [number], [bool] tai [object]
error-format-filter-needs-value = [{ $filter }] tarvitsee avaimen ja arvon
error-store-search-empty = ei mitään haettavaa; kirjoita paketin nimi
error-update-hook-not-set = aseta ensin kutsuttava osoite
error-upload-stalled = tiedoston { $path } lähetys pysähtyi; lähetä tämä osa uudelleen
error-upload-busy = tiedostoa { $path } kirjoitetaan jo
error-upload-incomplete =
    { $count ->
        [one] { $count } tiedosto on keskeneräinen: { $files }
       *[other] { $count } tiedostoa on keskeneräisiä, esim. { $files }
    }
error-upload-no-files-listed = ei tiedostoja
error-upload-not-part = { $path } ei kuulu lähetykseen { $id }, tai lähetys on suljettu
error-bad-path = virheellinen polku
error-upload-part-offset = { $offset } ei ole kohta, josta tiedoston { $path } jokin osa alkaa
error-upload-interrupted = lähetys keskeytyi: { $reason }
error-upload-part-long = tiedoston { $path } osa { $part } on pidempi kuin { $len } tavua
error-upload-part-short = tiedoston { $path } osa { $part }: saatiin { $got } tavua, odotettiin { $len }
error-upload-file-long = { $path } on ilmoitettua suurempi
error-upload-file-short = { $path }: saatiin { $got } tavua, odotettiin { $len }
error-upload-complete = lähetys on jo valmis
error-already-imported = tuotu jo nimellä ”{ $name }”

## Palvelimen tekstit

server-store-place-readme = README
server-store-place-folders = kansiot
server-store-why-linked = Linkki paketissa ({ $place })
server-store-why-same-title = Sama otsikko kuin paketissa ({ $place })
server-store-why-by = Tekijä { $author }, mainittu paketissa ({ $place })
server-store-why-meshes = Suunnilleen yhtä monta verkkoa kuin paketissa ({ $count })
server-store-why-textures = Suunnilleen yhtä monta tekstuuria kuin paketissa ({ $count })
server-chatgpt-revoke-failed = Kirjauduttu ulos täällä, mutta OpenAI:lle ei saatu tietoa. Varmista katkaisemalla Hoardin yhteys ChatGPT:n asetuksissa: Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Kirjauduttu ChatGPT:hen
server-chatgpt-signed-in = Hoard on kirjautunut. Tekoälytunnisteet voivat nyt käyttää ChatGPT-tilaustasi.
server-chatgpt-signed-in-as = Hoard on kirjautunut käyttäjänä { $email }. Tekoälytunnisteet voivat nyt käyttää ChatGPT-tilaustasi.
server-chatgpt-not-finished-title = Kirjautuminen ei valmistunut
server-chatgpt-other-computer = Jos aloitit kirjautumisen Hoardista toisella tietokoneella, kopioi tämän sivun osoite osoiteriviltä ja liitä se siellä Hoardin asetuksiin.
server-back-to-settings = Takaisin asetuksiin
server-issues-pack = Paketit, joita ei voitu lisätä
server-issues-pack-blurb = Poista paketti ja lisää se uudelleen, tai avaa se nähdäksesi sen sisällön.
server-issues-damaged = Tallennetut tiedostot, jotka ovat vioittuneet tai kadonneet
server-issues-damaged-blurb = Löytyi tehtävällä ”Tarkista tiedostojen vauriot”. Palauta ne varmuuskopiosta tai lisää niiden paketti uudelleen ja suorita sitten ”Tarkista tiedostojen vauriot” uudelleen.
server-issues-folder = Valvotut kansiot, joita ei voi lukea
server-issues-folder-blurb = Niiden paketit jätetään ennalleen, kunnes kansiota voi taas lukea.
server-issues-missing = Valvotut paketit, joiden kansio on kadonnut
server-issues-missing-blurb = Jokainen paketti säilytetään 3 päivää siltä varalta, että sen kansio on vain offline-tilassa, ja poistetaan sitten.
server-issues-job = Epäonnistuneet taustatyöt
server-issues-job-blurb = Uudelleenyritys ajaa työn uudelleen alusta.
server-issues-file = Tiedostot, joita ei voitu avata
server-issues-file-blurb = Hoard ei voinut avata tai tallentaa niitä. Valvottu paketti yrittää niitä uudelleen seuraavassa tarkistuksessaan; kirjastossa jo oleva tiedosto säilytti aiemman sisältönsä.
server-issues-unpack = Arkistot, joita ei voitu purkaa
server-issues-unpack-blurb = Jokainen säilytetään paketissaan tavallisena tiedostona, joten mikään sen sisältä ei ole kirjastossa.
server-issues-read = Tiedostot, joita Hoard ei ymmärtänyt
server-issues-read-blurb = Ne ovat kirjastossa, mutta ilman tietoja, tunnisteita tai pikkukuvaa, jotka lukeminen antaisi.
server-issues-render = Mallit, joiden pikkukuvan piirto epäonnistui
server-issues-render-blurb = Niillä ei ole pikkukuvaa. Avaaminen katselimessa voi silti toimia, ja sieltä voi tallentaa pikkukuvan.
server-issues-convert = Mallit, joita Blender ei voinut muuntaa
server-issues-convert-blurb = Niillä ei ole GLB-kopiota, joten niitä ei voi näyttää 3D:nä.
server-issues-unreal = Unreal-paketit, joita ei voitu muuntaa
server-issues-unreal-blurb = Paketin Unreal-sivulla ovat sen muut paketit.
server-issues-reformat = Tiedostot, joita muunnossääntö ei pystynyt muuntamaan
server-issues-reformat-blurb = Ne on jätetty ennalleen.
server-issues-materials = Paketit, joiden materiaaleja ei saatu selvitettyä
server-issues-materials-blurb = Niiden tekstuurit ovat kaikki tallessa; niitä ei vain ole ryhmitelty materiaaleiksi.
server-issues-material-thumb = Materiaalit ilman pikkukuvaa
server-issues-material-thumb-blurb = Materiaali löytyi, mutta sen esikatselua ei voitu piirtää.
server-issues-store-files = Kauppatiedostot, joita ei voitu lukea
server-issues-store-files-blurb = Tiedostot kaupasta, josta paketti on peräisin, kuten Megascans-JSON, jotka nimeävät paketin ja antavat sille tunnisteita.
server-issues-scan-again = Tarkista uudelleen
server-issues-retry = Yritä uudelleen
server-issues-reanalyze = Analysoi paketti uudelleen
server-issues-convert-again = Muunna uudelleen
server-issues-look-again = Katso uudelleen
server-issues-read-again = Lue uudelleen
server-issues-blob-missing = Tallennettu tiedosto on kadonnut.
server-issues-blob-damaged = Tallennettu tiedosto ei vastaa tallennettua: se on vioittunut.
server-issues-folder-gone = Kansiota ”{ $folder }” ei löytynyt.
server-job-a-pack = paketti
server-job-ingest = Tuodaan: { $pack }
server-job-retag = Päivitetään tunnisteita: { $pack }
server-job-llm-tag = Lisätään tunnisteita tekoälyllä: { $pack }
server-job-convert = Muunnetaan Blenderillä: { $pack }
server-job-unreal = Muunnetaan Unreal-paketteja: { $pack }
server-job-reformat = Muunnetaan tiedostoja muunnossääntöjen mukaan: { $pack }
server-job-scan = Tarkistetaan valvottua kansiota
server-job-relocate = Siirretään tiedostoja kirjaston sijaintien välillä
server-job-looks = Selvitetään värejä ja ulkonäköä
server-job-sounds = Kuunnellaan ääniä
server-job-embed = Opetellaan, mitä kuvat esittävät
server-job-hear = Opetellaan, miltä äänet kuulostavat
server-job-materials = Etsitään materiaaleja: { $pack }
server-job-read-files = Luetaan tiedostoja: { $pack }
server-job-store-lookup = Haetaan kauppasivua
server-maintenance-backup = Varmuuskopioi tietokanta
server-maintenance-backup-blurb = Eheä kopio tietokannasta (paketit, tunnisteet, muistiinpanot, kokoelmat ja asetukset) sen vieressä olevaan backups-kansioon. Kirjaston tiedostot eivät ole siinä: ne eivät muutu tallentamisen jälkeen, joten mikä tahansa tiedostojen synkronointi varmuuskopioi ne.
server-maintenance-scrub = Tarkista tiedostojen vauriot
server-maintenance-scrub-blurb = Lukee jokaisen tallennetun tiedoston ja vertaa sitä sen sormenjälkeen, jotta levyltä tai NAS-laitteelta pilaantuneet tai kadonneet tiedostot löytyvät. Kestää jonkin aikaa suuressa kirjastossa.
server-maintenance-reanalyze = Analysoi kaikki paketit uudelleen
server-maintenance-reanalyze-blurb = Lukee jokaisen tiedoston uudelleen tällä Hoardin versiolla: pikkukuvat, esikatselut, tiedot ja sääntöjen tunnisteet tehdään alusta. Omat muokkauksesi säilyvät; tekoälyltä ei kysytä uudelleen.
server-maintenance-clear-cache = Tyhjennä muunnosten välimuisti
server-maintenance-clear-cache-blurb = Poistaa ”Lataa muodossa” -muunnoksista säilytetyt tiedostot (ne tehdään uudelleen, kun niitä seuraavan kerran pyydetään) ja keskeytyneiden töiden jäänteet.
server-maintenance-shrink-thumbs = Pienennä pikkukuvat
server-maintenance-shrink-thumbs-blurb = Tallentaa PNG-muodossa säilytetyt pikkukuvat ja mallinäkymät kohdan Asetukset → Suorituskyky → Pikkukuvat mukaisesti, mikä yleensä tekee niistä monta kertaa pienempiä. Läpinäkyvät kuvat ja pikseligrafiikka pysyvät PNG-muodossa.
server-maintenance-gc = Poista käyttämättömät tiedostot
server-maintenance-gc-blurb = Poistaa tallennetut tiedostot, joihin mikään ei enää viittaa, kuten lopullisesti poistettujen pakettien tiedostot.
server-update-no-address = kutsuttavaa osoitetta ei ole asetettu
server-update-no-answer = ei vastausta 30 sekunnissa

## Asetukset: Ulkoasu

appearance-language = Kieli
appearance-language-auto = Selaimen kieli ({ $name })

## Juuri

root-unreachable = Hoard-palvelinta ei tavoitettu: { $error }
root-unreachable-desktop = Hoard ei vastaa: { $error }
root-desktop-reopen = Tämä ikkuna kirjattiin ulos. Sulje Hoard ja avaa se uudelleen jatkaaksesi.
root-try-again = Yritä uudelleen
root-lazy-connect = sovellusyhteys
window-minimize = Pienennä
window-maximize = Suurenna
window-restore = Palauta
window-close = Sulje

## Sovellus: kirjasto, yläpalkki ja suodattimet

app-lazy-settings = asetukset
app-lazy-archive = arkisto
app-lazy-shortcuts = pikanäppäimet
app-lazy-setup = käyttöönotto
app-theme-system = Järjestelmän teema
app-theme-light = Vaalea teema
app-theme-dark = Tumma teema
app-color-red = Punainen
app-color-orange = Oranssi
app-color-yellow = Keltainen
app-color-green = Vihreä
app-color-teal = Sinivihreä
app-color-blue = Sininen
app-color-purple = Violetti
app-color-pink = Vaaleanpunainen
app-color-brown = Ruskea
app-color-sand = Hiekka
app-color-gray = Harmaa
app-color-slate = Liuskeenharmaa
app-color-black = Musta
app-color-white = Valkoinen
app-color-swatch = { $color }: resurssit, joissa on paljon tätä väriä
app-color-any = Mikä tahansa väri
app-color-pick = Valitse mikä tahansa väri
app-color = Väri
app-music = Musiikki
app-tempo = BPM
app-tempo-min = alkaen
app-tempo-max = asti
app-tempo-min-title = Musiikki tässä tempossa tai nopeampi, iskuina minuutissa (myös puolikas tai tuplatempo lasketaan)
app-tempo-max-title = Musiikki tässä tempossa tai hitaampi, iskuina minuutissa
app-key = Sävellaji
app-key-any = Mikä tahansa sävellaji
app-key-major = Duuri
app-key-minor = Molli
app-key-mix = Ja sävellajit, jotka sopivat miksattaviksi sen kanssa
app-key-mix-title = Sen rinnakkaisduuri tai -molli ja kvintin päässä kummallakin puolella olevat sävellajit, joita DJ:t miksaavat sen kanssa
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Kirjoita äänen tekemä ääni niin kuin sen tavaisit: sen kaltaiset äänet tulevat ensin
app-imitation-record = Tee ääni mikrofoniin: sen kaltaiset äänet tulevat ensin
app-imitation-stop = Lopeta nauhoitus
app-imitation-heard = Sinun äänesi
app-imitation-mic-failed = Mikrofonista ei voitu nauhoittaa: { $error }
app-retry = Yritä uudelleen
app-undo = Kumoa
app-search-failed = Haku epäonnistui: { $error }
app-unreachable = Palvelinta ei tavoitettu: { $error }
app-unreachable-desktop = Hoard ei vastaa: { $error }
app-forget-project-title = Unohdetaanko projekti { $name }?
app-forget-project-body = Hoard lakkaa näyttämästä projektia sen { $count } resurssin kohdalla, kunnes seuraava hoard sync kertoo siitä taas. Projektissa ei muutu mitään.
app-forget-project-confirm = Unohda
app-forget-project-failed = Projektin { $name } unohtaminen epäonnistui: { $error }
app-forget-project-label = Unohda projekti { $name }
app-forget-project = Unohda tämä projekti
app-save-name-like = kuin { $name }
app-save-name-favorites = suosikit
app-search-saved = Tallennettu ”{ $name }”: löytyy suodattimista kohdasta Omat
app-rank-alike = Samankaltaisimmat
app-rank-color = Eniten väriä
app-rank-match = Paras osuma
app-rank-imitation = Eniten sen kaltaiset
app-sort = Lajittelu
app-sort-name = Nimi
app-sort-created = Luotu
app-sort-updated = Päivitetty
app-sort-size = Koko
app-sort-newest-first = Uusimmat ensin
app-sort-oldest-first = Vanhimmat ensin
app-sort-a-to-z = A–Ö
app-sort-z-to-a = Ö–A
app-sort-largest-first = Suurimmat ensin
app-sort-smallest-first = Pienimmät ensin
app-collection-added =
    { $count ->
        [one] Lisättiin { $count } resurssi kokoelmaan { $name }
       *[other] Lisättiin { $count } resurssia kokoelmaan { $name }
    }
app-collection-add-failed = Kokoelmaan lisääminen epäonnistui: { $error }
app-collection-removed = Kokoelmasta { $name } poistettiin { $count }
app-collection-remove-failed = Kokoelmasta poistaminen epäonnistui: { $error }
app-ns-subject = Aihe
app-ns-style = Tyyli
app-ns-theme = Teema
app-ns-material = Materiaali
app-ns-map = PBR-kartta
app-ns-color = Väri
app-ns-mood = Tunnelma
app-ns-genre = Lajityyppi
app-ns-license = Lisenssi
app-ns-res = Resoluutio
app-ns-duration = Kesto
app-ns-poly = Polygonimäärä
app-ns-format = Muoto
app-ns-weight = Koko
app-ns-tech = Tekninen
app-ns-blender = Blender-versio
app-ns-depth = Bittisyvyys
app-ns-ucs = Ääniluokka (UCS)
app-ns-bpm = Tempo
app-ns-root = Pohjasävel
app-ns-level = Äänekkyys
app-ns-ue = Unreal-luokka
app-ns-engine = Pelimoottori
app-ns-extracted = Palautettu lähde
app-ns-script = Kirjoitusjärjestelmä
app-bulk-failed =
    { $count ->
        [one] Resurssin muuttaminen epäonnistui: { $error }
       *[other] { $count } resurssin muuttaminen epäonnistui: { $error }
    }
app-bulk-untagged =
    { $count ->
        [one] Tunniste { $tag } poistettiin { $count } resurssista
       *[other] Tunniste { $tag } poistettiin { $count } resurssista
    }
app-bulk-tagged =
    { $count ->
        [one] Tunniste { $tag } lisättiin { $count } resurssiin
       *[other] Tunniste { $tag } lisättiin { $count } resurssiin
    }
app-bulk-category-set =
    { $count ->
        [one] Luokaksi asetettiin { $category } { $count } resurssille
       *[other] Luokaksi asetettiin { $category } { $count } resurssille
    }
app-bulk-category-cleared =
    { $count ->
        [one] Luokka tyhjennettiin { $count } resurssilta
       *[other] Luokka tyhjennettiin { $count } resurssilta
    }
app-credits-failed = Tekijätietojen luonti epäonnistui: { $error }
app-archived-assets =
    { $count ->
        [one] Arkistoitiin { $count } resurssi
       *[other] Arkistoitiin { $count } resurssia
    }
app-archived-one = Arkistoitu: { $name }
app-open-failed = Resurssin avaaminen epäonnistui: { $error }
app-unnamed-pack = paketti
app-unnamed-collection = kokoelma
app-unnamed-asset = resurssi
app-home-version = Hoard { $version } – takaisin kirjastoon
app-home = Takaisin kirjastoon
app-search-label = Hae resursseja
app-search-placeholder = Hae nimiä, polkuja, tunnisteita
app-views = Näkymät
app-library = Kirjasto
app-packs = Paketit
app-packs-working = { $count } työn alla
app-update-title = Hoard { $version } on julkaistu (tämä palvelin käyttää versiota { $current }). Hae uusi image tai binääri ja käynnistä uudelleen; tietokanta päivittyy itse.
app-update = Päivitä
app-theme = Teema
app-sign-in = Kirjaudu sisään
app-archive-title = Arkisto: { $contents }
app-archive = Arkisto
app-settings = Asetukset
app-palette-title = Komentopaletti ({ $keys })
app-import-folder = Tuo kansio
app-add-files = Lisää tiedostoja
app-add-files-short = Lisää
app-split-label = Miten tämä kansio tuodaan
app-split-before = Kansiossa{ " " }
app-split-after =
    { $count ->
        [one] { " " }on { $count } kansio tai arkisto.
       *[other] { " " }on { $count } kansiota tai arkistoa.
    }
app-split-each = Tuo jokainen omana pakettinaan ({ $count })
app-split-one = Tuo yhtenä pakettina
app-cancel = Peruuta
app-filters = Suodattimet
app-done = Valmis
app-yours = Omat
app-favorites = Suosikit
app-forget-search-label = Unohda tallennettu haku { $name }
app-forget-search = Unohda tämä haku
app-projects-title = Projektit, jotka tekevät hoard sync -synkronoinnin täältä, ja mitä niiden viimeisin synkronointi sisälsi
app-projects = Projektit
app-project-synced = Synkronoitu { $when }
app-project-synced-by = Synkronoitu { $when }, synkronoija: { $who }
app-collections = Kokoelmat
app-no-filters = Tälle haulle ei ole suodattimia.
app-kind = Tyyppi
app-facets-more = { $count } lisää
app-facets-fewer = vähemmän
app-searching = Haetaan…
app-searching-live = Haetaan
app-asset-count =
    { $count ->
        [one] { $count } resurssi
       *[other] { $count } resurssia
    }
app-clear-all = Tyhjennä kaikki
app-save-name-label = Tämän haun nimi
app-save-name-placeholder = Anna nimi
app-save = Tallenna
app-save-search-title = Säilytä tämä haku suodattimissa kohdassa Omat
app-save-search = Tallenna haku
app-select-title = Valitse useita resursseja lisätäksesi tunnisteita, luokitellaksesi, kerätäksesi, ladataksesi tai arkistoidaksesi ne yhdessä (X tai Ctrl-napsautus)
app-select = Valitse
app-tile-size = Ruutujen koko
app-supporting-title = glTF-puskurit, materiaalikirjastot, materiaalien kartat, vain yhden mallin käyttämät materiaalit ja malleja kuvaavat kauppatiedostot. Ne tulevat mukana, kun lataat niitä käyttävän resurssin. Sprite-arkin sivut ja sen spritet, jotka paketissa on myös erillisinä kuvina, näytetään arkin yhteydessä. Resurssi, joka paketissa on useassa muodossa, näytetään kerran, siinä muodossa, jonka Hoard lukee parhaiten.
app-supporting-hidden =
    { $count ->
        [one] { $count } tukitiedosto piilotettu
       *[other] { $count } tukitiedostoa piilotettu
    }
app-supporting-show = Näytä
app-supporting-shown = Tukitiedostot mukana
app-supporting-hide = Piilota
app-folders = Kansiot
app-folders-title = Selaa pakettia kansio kerrallaan, niin kuin se oli levyllä
app-folders-label = Paketin kansiot
app-welcome-title = Pudota resurssipaketti mihin tahansa tälle sivulle
app-welcome-before = Zipit, tar-arkistot, kansiot ja irralliset tiedostot käyvät. Hoard purkaa ne, tallentaa jokaisen tiedoston kerran sen tiivisteen perusteella, lukee koot, kestot ja polygonimäärät ja lisää kaikkeen tunnisteet, jotta voit hakea esimerkiksi{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }tai{ " " }
app-welcome-after = .
app-welcome-import-folder = Tuo kansio
app-welcome-choose-files = Valitse tiedostot
app-none-similar = Mikään muu ei vielä näytä kovin samalta kuin { $name }.
app-none = Ei osumia.
app-none-query = Mikään ei vastaa hakua ”{ $query }”.
app-none-imitation = Mikään mitattu ääni ei vielä kuulosta paljon tältä: ”{ $word }”.
app-none-heard = Mikään mitattu ääni ei vielä kuulosta paljon sinun ääneltäsi.
app-none-filters = Näillä suodattimilla ei ole osumia.
app-none-query-filters = Mikään ei vastaa hakua ”{ $query }” näillä suodattimilla.
app-clear-search-filters = Tyhjennä haku ja suodattimet
app-loading-more = Ladataan lisää
app-keys-hint = Nuolinäppäimet liikkuvat, Enter avaa, välilyönti soittaa äänen, X tai Ctrl-napsautus valitsee useita (kosketusnäytöllä pitkä painallus).
app-resize-label = Muuta tietopaneelin kokoa
app-resize-title = Vedä muuttaaksesi kokoa · kaksoisnapsauta sopivaa leveyttä varten
app-selected-label = Valitut resurssit
app-selected = { $count } valittuna
app-pick-assets = Valitse resursseja
app-download-zip = Lataa zip
app-credits-title = CREDITS.md niille paketeille, joista nämä ovat: kuka ne teki, niiden lisenssit ja niiden pyytämät tekijämaininnat
app-credits = Tekijätiedot
app-prefer-sources-title = Unrealin SoundWavet ja tekstuurit, joiden alkuperäisen tiedoston Hoard palautti
app-prefer-sources = Palautettu WAV/PNG .uassetin sijaan ({ $count })
bag-clear = Tyhjennä valinta
bag-glb-heading = Mallit GLB-muodossa
bag-glb-text = Jokainen malli .glb-tiedostona Y ylöspäin seisten ja eteenpäin katsoen, sen tekstuurit erillisinä tiedostoina textures-kansiossa, jonka mallit jakavat. Kaikki muu menee mukaan sellaisenaan.
bag-glb-download = Lataa GLB-zip
bag-plain-heading = Tiedostot sellaisenaan
bag-plain-text = Jokainen tiedosto sellaisena kuin se tuotiin, tarvitsemiensa kanssa, omassa kansiossaan pakettia kohden.
bag-remove = Poista { $name } valinnasta
bag-more = ja { $count } lisää
app-bulk-tag-label = Tunniste, joka lisätään valittuihin resursseihin tai poistetaan niistä
app-bulk-tag-placeholder = Tunniste
app-bulk-tag-add-title = Lisää tämä tunniste jokaiseen valittuun resurssiin
app-bulk-tag-add = Lisää
app-bulk-tag-remove-title = Poista tämä tunniste jokaisesta valitusta resurssista
app-bulk-tag-remove = Poista
app-bulk-category-label = Valittujen resurssien luokka (tyhjä tyhjentää sen)
app-bulk-category-placeholder = Luokka
app-bulk-category-set-title = Aseta jokaisen valitun resurssin luokka
app-bulk-category-clear-title = Tyhjennä jokaisen valitun resurssin luokka
app-bulk-category-set-button = Aseta
app-bulk-category-clear-button = Tyhjennä
app-remove-from-collection = Poista kokoelmasta { $name }
app-unfavorite = Poista suosikeista
app-favorite = Suosikiksi
app-select-all = Valitse kaikki { $count }
app-archive-selected-title = Poissa hauista, kunnes ne palautetaan tai poistetaan lopullisesti arkistosta
app-drop-items = Pudota { $count } kohdetta lisätäksesi ne
app-drop = Pudota lisätäksesi Hoardiin
app-drop-hint = Jokaisesta ylimmän tason tiedostosta tai kansiosta tulee oma pakettinsa. Gigatavut eivät haittaa; suuret tiedostot jatkuvat, jos yhteys katkeaa.

## Komentopaletti

palette-group-go-to = Siirry
palette-group-actions = Toiminnot
palette-group-theme = Teema
palette-group-kinds = Tyypit
palette-group-packs = Paketit
palette-group-collections = Kokoelmat
palette-group-projects = Projektit
palette-group-saved = Tallennetut haut
palette-group-assets = Resurssit
palette-hint-current = nykyinen
palette-library = Kirjasto
palette-library-keywords = resurssit ruudukko koti alku assets grid home
palette-favorites = Suosikit
palette-favorites-keywords = tähdet tähditetyt tykätyt starred stars liked
palette-packs = Paketit
palette-archive = Arkisto
palette-archive-keywords = roskakori poistetut palauta trash recycle bin deleted restore
palette-stats = Kirjaston tilastot
palette-stats-keywords = tilastot luvut koko levytila määrä statistics numbers size disk space count
palette-copies = Lähes samat kopiot
palette-copies-keywords = kaksoiskappaleet samat samankaltaiset skaalatut muunnetut hukattu tila duplicates same similar resized converted wasted space dedupe
palette-search = Haku
palette-add-files = Lisää tiedostoja…
palette-add-files-keywords = lähetä tuo upload import
palette-import-folder = Tuo kansio…
palette-import-folder-keywords = lähetä paketti upload pack
palette-select-all =
    { $count ->
        [one] Valitse näytetty { $count }
       *[other] Valitse kaikki { $count } näytettyä
    }
palette-select-all-keywords = valitse check
palette-select-several = Valitse useita…
palette-select-several-keywords = joukko useita tunniste arkistoi bulk multiple tag archive
palette-archive-selected =
    { $count ->
        [one] Arkistoi valittu
       *[other] Arkistoi { $count } valittua
    }
palette-archive-selected-keywords = poista roskakori delete remove trash
palette-clear-selection = Tyhjennä valinta
palette-clear-filters = Tyhjennä suodattimet
palette-clear-filters-keywords = nollaa reset
palette-hide-supporting = Piilota tukitiedostot
palette-show-supporting = Näytä tukitiedostot
palette-supporting-keywords = normaalikartat karheus maskit normal maps roughness masks
palette-account = Tili ja käyttöoikeustunnukset
palette-account-keywords = salasana kirjautuminen käyttäjä tunnus password login user token api
palette-sign-out = Kirjaudu ulos ({ $name })
palette-sign-out-keywords = kirjaudu ulos log out logout
palette-sign-in = Kirjaudu sisään
palette-sign-in-keywords = kirjaudu tili salasana log in login account password
palette-settings = Asetukset
palette-settings-keywords = blender ffmpeg llm tekoäly ai tunnisteet tagging unreal ylläpitäjä admin työkalut tools preferences
palette-shortcuts = Pikanäppäimet
palette-shortcuts-keywords = ohje näppäimet help keys
palette-theme-keywords = teema ulkoasu tila theme appearance mode
palette-theme-variant-keywords = ulkoasu väri appearance color
palette-light-theme = Vaalea teema: { $name }
palette-dark-theme = Tumma teema: { $name }
palette-kind-all = Kaikki tyypit (älä näytä vain tyyppiä: { $kind })
palette-kind-only = Vain { $kind }
palette-title = Komentopaletti
palette-placeholder = Hae resursseja, siirry pakettiin, suorita komento…
palette-results = Tulokset
palette-searching = Haetaan…
palette-no-match = Mikään ei vastaa hakua ”{ $query }”
palette-foot-move = liiku
palette-foot-run = suorita
palette-foot-shortcuts = kaikki pikanäppäimet

## Pikanäppäimet

shortcuts-title = Pikanäppäimet
shortcuts-close = Sulje
shortcuts-anywhere = Kaikkialla
shortcuts-palette = Komentopaletti: hae, siirry, suorita
shortcuts-search = Haku
shortcuts-this-list = Tämä luettelo
shortcuts-escape = Sulje, tyhjennä valinta tai sulje tiedot
shortcuts-library = Kirjasto
shortcuts-move = Liiku ruutujen välillä (tai J ja K)
shortcuts-open = Avaa tiedot
shortcuts-play-sound = Toista tai pysäytä ääni
shortcuts-select = Valitse tai poista valinta
shortcuts-favorite = Lisää suosikkeihin tai poista niistä
shortcuts-key-click = napsautus
shortcuts-select-range = Valitse alue
shortcuts-select-all = Valitse kaikki näytetyt
shortcuts-sound-player = Äänisoitin
shortcuts-play-pause = Toista tai tauko
shortcuts-skip = Siirry taaksepäin tai eteenpäin
shortcuts-start-end = Alkuun tai loppuun
shortcuts-image-inspector = Kuvatarkastin
shortcuts-zoom = Lähennä tai loitonna
shortcuts-fit = Sovita ikkunaan
shortcuts-actual-pixels = Todelliset pikselit
shortcuts-tile = Monista 3×3 saumojen tarkistamiseksi
shortcuts-background = Seuraava tausta
shortcuts-channel = Näytä yksi kanava

## Tyyppivalitsimet

kinds-group = Resurssityypit

## Tunnisteet

tag-title-with-source = { $tag } ({ $source })
tag-source-human = ihmisen lisäämä
tag-source-rule = sääntöpohjaisen tunnistelijan antama
tag-source-meta = mitattu tiedostosta
tag-source-ai = tekoälyn ehdottama
tag-remove = Poista tunniste { $tag }

## Taustatoiminta

activity-label-idle = Taustatoiminta: ei käynnissä
activity-label-running = Taustatoiminta: { $running } käynnissä
activity-label-queued = Taustatoiminta: { $running } käynnissä, { $queued } jonossa
activity-title = Taustatoiminta
activity-unreachable = Palvelinta ei tavoiteta
activity-unreachable-desktop = Hoard ei vastaa
activity-empty = Mitään ei ole käynnissä. Tuonnit, tunnisteiden lisäys, muunnokset ja zip-lataukset näkyvät täällä niiden ollessa käynnissä.
activity-queued = jonossa
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct } %

## Resurssiruutu

tile-triangles =
    { $count ->
        [one] { $count } kolmio
       *[other] { $count } kolmiota
    }
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Toista { $name }
tile-pause = Keskeytä { $name }
tile-favorite = Suosikiksi: { $name }
tile-in-favorites = Suosikeissasi
tile-add-favorite = Lisää suosikkeihin (F)
tile-more-like = Samankaltaisia: { $name }
tile-more-like-this = Lisää samankaltaisia
tile-select = Valitse { $name }

## Kokoelmavalitsin

collections-add = Lisää kokoelmaan
collections-menu = Kokoelmat
collections-find-label = Etsi tai luo kokoelma
collections-find-placeholder = Etsi tai nimeä uusi kokoelma
collections-create = Luo
collections-empty = Ei vielä kokoelmia. Kirjoita nimi aloittaaksesi uuden.

## Kansiovalitsin

folders-list = Kansiot
folders-close = Sulje
folders-browse = Selaa…
folders-up-title = Yksi kansio ylöspäin
folders-up = Ylös
folders-this-computer = Tämä tietokone
folders-none = Täällä ei ole kansioita.

## Sijaintilomake

locations-mode-question = Mitä Hoardin pitäisi tehdä tällä kansiolla?
locations-mode-managed = Ota se haltuun: Hoard säilyttää pakettini täällä
locations-mode-managed-hint = Lähettämäsi tai tuomasi paketit kopioidaan tänne, ja Hoard pitää ne järjestyksessä. Kansio on nyt Hoardin, joten älä koske sen sisältöön.
locations-mode-watched = Valvo sitä: pakettini ovat jo täällä
locations-mode-watched-hint = Jokaisesta sen sisällä olevasta kansiosta tulee paketti. Hoard lukee tiedostot paikoillaan eikä koskaan muuta, siirrä tai poista niitä, ja se huomaa uudet kansiot niiden ilmestyessä.
locations-folder-desktop = Kansio
locations-folder-placeholder = Kansio, jossa pakettisi ovat
locations-folder-hint-managed = Hoard luo sen, jos sitä ei ole. Dockerissa tämä on kansio, jonka olet liittänyt konttiin.
locations-folder-hint-managed-desktop = Hoard luo sen, jos sitä ei ole.
locations-folder-hint-watched = Liitä kansio Dockerissa ensin konttiin (pelkkä lukuoikeus riittää) ja valitse se sitten tästä.
locations-folder-hint-watched-desktop = Hoard vain lukee sitä: mitään sen sisällä ei muuteta.
locations-name = Nimi
locations-name-placeholder-managed = Kirjasto
locations-name-placeholder-watched = Omat paketit
locations-files = Tänne tulevat tiedostot
locations-catch-all = Myös kaikki, mitä mikään muu sijainti ei ota
locations-kinds-hint = Uuden paketin tiedostot menevät sijaintiin, jossa niiden tyyppi on valittuna. Loput menevät kaiken vastaanottavaan sijaintiin.
locations-files-all = Toistaiseksi kaikki. Lisää myöhemmin sijainteja asetuksista lähettääksesi joitakin tyyppejä (vaikkapa musiikin) toiselle levylle.
locations-first-watched-note = Lähettäminen vaatii kirjaston, jota Hoard hoitaa. Voit lisätä sellaisen myöhemmin kohdassa Asetukset → Kirjaston sijainnit.
locations-checking = Tarkistetaan kansiota…
locations-add = Lisää sijainti
locations-cancel = Peruuta

## Sijainnit

locations-failed = { $what }: { $error }
locations-load-failed = Kirjaston sijaintien lataus epäonnistui
locations-takes-and-rest = { $kinds } ja kaikki muu
locations-takes-rest = Kaikki, mitä muut eivät ota
locations-takes-everything = Kaikki
locations-takes-nothing = Ei vielä mitään
locations-save-failed = Sijainnin tallennus epäonnistui
locations-unwatch-title = Lopetetaanko kansion { $name } valvonta?
locations-unwatch-body =
    { $count ->
        [one] Sen { $count } paketti poistuu Hoardista tunnisteineen ja muistiinpanoineen. Kansion tiedostoihin ei kosketa.
       *[other] Sen { $count } pakettia poistuu Hoardista tunnisteineen ja muistiinpanoineen. Kansion tiedostoihin ei kosketa.
    }
locations-unwatch-body-empty = Kansion tiedostoihin ei kosketa.
locations-unwatch = Lopeta valvonta
locations-remove-title = Poistetaanko sijainti { $name }?
locations-remove-body = Sen tiedostot siirretään ensin tyypin mukaan muihin sijainteihin; sijainti poistuu, kun se on tyhjä. Mitään ei menetetä.
locations-remove-body-only = Jos siinä on tiedostoja, lisää ensin toinen sijainti, johon ne siirretään.
locations-remove = Poista
locations-removed = Poistettu: { $name }
locations-emptying = Siirretään tiedostoja pois sijainnista { $name }; se poistetaan, kun siirto on valmis
locations-offline-title = { $name } ei ole tavoitettavissa
locations-offline-body = Sen tiedostoja ei voi siirtää pois, kun se on offline-tilassa. Poistetaanko silti? Siihen jääneet tiedostot näkyvät puuttuvina.
locations-remove-anyway = Poista silti
locations-remove-failed = Sijainnin { $name } poisto epäonnistui
locations-rebalancing = Siirretään tallennettuja tiedostoja sijainteihin, jotka ottavat niiden tyypin
locations-rebalance-failed = Tiedostojen siirto epäonnistui
locations-scanned-never = ei vielä tarkistettu
locations-scanned-now = tarkistettu juuri
locations-scanned-minutes = tarkistettu { $mins } min sitten
locations-scanned-hours = tarkistettu { $hours } h sitten
locations-scanned-date = tarkistettu { $date }
locations-scan-failed = Sijainnin { $name } tarkistus epäonnistui
locations-watching = Valvotaan: { $name }
locations-added = Lisätty: { $name }
locations-loading = Ladataan…
locations-managed-title = Hoardin hoitamat kirjastot
locations-watched-title = Valvotut kansiot
locations-managed-hint = Lähetykset ja tuonnit kopioidaan tänne. Kukin sijainti ottaa valitsemansa tyypit; kaiken vastaanottava sijainti ottaa loput.
locations-watched-hint = Pakettikansiot indeksoidaan paikoillaan. Hoard ei koskaan muuta niissä olevia tiedostoja ja etsii uusia ja poistettuja kansioita 15 minuutin välein.
locations-managed-empty = Ei vielä yhtään, joten lähetyksille ei ole paikkaa. Lisää sellainen alla.
locations-reachable = Tavoitettavissa
locations-unreachable-now = Ei tavoitettavissa juuri nyt
locations-catch-all-tag = ottaa loput
locations-packs =
    { $count ->
        [one] { $count } paketti
       *[other] { $count } pakettia
    }
locations-unreachable = ei tavoitettavissa
locations-scanning = Tarkistetaan…
locations-scan = Tarkista nyt
locations-edit = Muokkaa
locations-catch-all-locked = Muuttaaksesi tätä tee toisesta sijainnista kaiken vastaanottava sijainti
locations-edit-hint-before = Koskee tästä lähtien lisättäviä tiedostoja. Jo tallennetut voi siirtää toiminnolla
locations-edit-hint-after = alla.
locations-save = Tallenna
locations-rebalance = Siirrä tallennetut tiedostot vastaavasti
locations-rebalance-hint = Kun olet muuttanut, mitä sijainti ottaa: jo tallennetut tiedostot siirtyvät sijaintiin, johon niiden tyyppi nyt kuuluu.
locations-add-title = Lisää sijainti
locations-add-hint = Toinen levy joillekin tiedostotyypeille tai kansio paketeista, jotka sinulla jo on.
locations-kind-question = Millainen sijainti se on?
locations-kind-folder = Kansio
locations-kind-folder-hint = Levyllä tai konttiin liitetyssä NAS-jaossa, tai kansio, jonka Dropbox, Google Drive, OneDrive tai iCloud Drive pitää synkronoituna. Hoard voi säilyttää tiedostoja siellä tai seurata paketteja, jotka sinulla jo on.
locations-kind-folder-hint-desktop = Tällä tietokoneella, toisella levyllä tai NAS-jaossa, tai kansio, jonka Dropbox, Google Drive, OneDrive tai iCloud Drive pitää synkronoituna. Hoard voi säilyttää tiedostoja siellä tai seurata paketteja, jotka sinulla jo on.
locations-kind-s3-hint = Bucket Amazon S3:ssa tai vastaavassa palvelussa: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard säilyttää tiedostoja siellä.
locations-s3-title = S3-tallennustila
locations-s3-hint = Sijainti s3://bucket-name on bucket tässä palvelussa: AWS tai mikä tahansa S3-yhteensopiva (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage NAS-laitteella).
locations-bucket = Bucket
locations-bucket-hint = Jo olemassa olevan bucketin nimi. Yllä olevilla avaimilla on voitava listata, lukea ja kirjoittaa sitä.
locations-s3-endpoint = Osoite
locations-s3-endpoint-placeholder = Tyhjä, jos AWS; esim. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Alue
locations-s3-access-key = Käyttöavaimen tunnus
locations-s3-secret-key = Salainen käyttöavain
locations-s3-headers = Lisäotsakkeet
locations-s3-headers-placeholder = Name=value, yksi riviä kohden (esimerkiksi Cloudflare Access -palvelua varten)
locations-s3-kept = Tallennettu (kirjoita korvataksesi)
locations-s3-secrets = Avaimet ja otsakkeet tallennetaan palvelimen asetusten mukana, eikä niitä näytetä enää koskaan.
locations-s3-secrets-desktop = Avaimet ja otsakkeet tallennetaan tämän tietokoneen avainnippuun, eikä niitä näytetä enää koskaan.
locations-s3-saved = S3-tallennustilan asetukset tallennettu
locations-s3-save-failed = S3-tallennustilan tallennus epäonnistui
locations-add-more = Lisää sijainti…

## Ongelmat

issues-failed = { $what }: { $error }
issues-load-failed = Ongelmien lataus epäonnistui
issues-started =
    { $count ->
        [one] Käynnistetty uudelleen.
       *[other] { $count } kohdetta käynnistetty uudelleen.
    }
issues-retry-failed = Uudelleenyritys epäonnistui
issues-dismiss-failed = Kuittaus epäonnistui
issues-dismiss-all-title = Kuitataanko kaikki { $count }?
issues-dismiss-all-body = ”{ $title }” tyhjennetään. Jos jokin menee taas pieleen, se näytetään uudelleen.
issues-dismiss-all = Kuittaa kaikki
issues-just-now = juuri nyt
issues-minutes-ago = { $mins } min sitten
issues-hours-ago = { $hours } h sitten
issues-loading = Ladataan…
issues-none = Mikään ei ole mennyt pieleen.
issues-action-all = { $action } (kaikki)
issues-open-asset = Avaa
issues-open-pack = Näytä paketti
issues-open-locations = Kirjaston sijainnit
issues-where-before = kohteessa
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } kerta
       *[other] { $count } kertaa
    }
issues-starting = Käynnistetään…
issues-dismiss-named = Kuittaa { $title }
issues-dismiss = Kuittaa
issues-show-fewer = Näytä vähemmän
issues-show-all = Näytä kaikki { $count }
issues-not-listed =
    { $count ->
        [one] { $count } muu ei näy luettelossa.
       *[other] { $count } muuta ei näy luettelossa.
    }

## Arkistointi

archive-anyway = Arkistoi silti
archive-failed = Arkistointi epäonnistui: { $error }
archive-undo = Kumoa
archive-restore-failed = Palautus epäonnistui: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } paketti
       *[other] { $packs } pakettia
    } ja { $assets ->
        [one] { $assets } resurssi
       *[other] { $assets } resurssia
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } paketti
       *[other] { $packs } pakettia
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } resurssi
       *[other] { $assets } resurssia
    }
archive-count-nothing = ei mitään

## Arkisto

archive-load-failed = Arkiston lataus epäonnistui: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Palautettu: { $what }
archive-purged = Poistettu lopullisesti: { $what }
archive-purge-failed = Poisto epäonnistui: { $error }
archive-purge-title = Poistetaanko { $name } lopullisesti?
archive-purge-pack-body =
    { $count ->
        [one] Myös sen { $count } resurssi ja { $size } tiedostoja poistuu, paitsi tiedostot, jotka toinen paketti jakaa. Tätä ei voi perua.
       *[other] Myös sen { $count } resurssia ja { $size } tiedostoja poistuu, paitsi tiedostot, jotka toinen paketti jakaa. Tätä ei voi perua.
    }
archive-purge = Poista lopullisesti
archive-purge-asset-body = Sen tiedosto vapautetaan, ellei toinen resurssi jaa sitä. Tätä ei voi perua.
archive-empty-title = Tyhjennetäänkö arkisto?
archive-empty-body = Lopullisesti poistetaan: { $what }. Niiden tiedostot vapautetaan. Tätä ei voi perua.
archive-empty = Tyhjennä arkisto
archive-title = Arkisto
archive-intro = Arkistoidut paketit ja resurssit eivät näy hauissa, määrissä eivätkä tunnisteluettelossa. Palauta ne tai poista ne lopullisesti vapauttaaksesi niiden tilan.
archive-loading = Ladataan…
archive-is-empty = Arkisto on tyhjä
archive-is-empty-hint = Arkistoi paketti tai resurssi, niin se odottaa täällä, kunnes palautat sen tai poistat sen lopullisesti.
archive-packs = Paketit
archive-assets = Resurssit
archive-asset-count =
    { $count ->
        [one] { $count } resurssi
       *[other] { $count } resurssia
    }
archive-archived = arkistoitu { $when }
archive-from = paketista { $pack }
archive-restore = Palauta
archive-show-more = Näytä lisää ({ $count } jäljellä)

## Omat

projects-favorite-add-failed = Suosikkeihin lisääminen epäonnistui: { $error }
projects-favorite-remove-failed = Suosikeista poistaminen epäonnistui: { $error }
projects-save-search-failed = Haun tallennus epäonnistui: { $error }
projects-forget-search-failed = Haun { $name } poisto epäonnistui: { $error }

## Projektit

projects-list-and = { $items } ja { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more =
    { $count ->
        [one] { $count } muu
       *[other] { $count } muuta
    }
projects-other =
    { $count ->
        [one] { $count } muu projekti
       *[other] { $count } muuta projektia
    }
projects-usage-title-one = { $name } käyttää tätä
projects-usage-title = { $count } projektia käyttää tätä
projects-usage-body-one = Käytössä projektissa { $list }: sen seuraava hoard sync poistaa ne.
projects-usage-body = Käytössä projekteissa { $list }: kunkin seuraava hoard sync poistaa ne.

## Lähettäjä

upload-network-error = verkkovirhe
upload-gave-up = { $what }: { $error } ({ $attempts } yrityksen jälkeen)
upload-failed-plain = epäonnistui
upload-already-imported = tuotu jo
upload-part-failed = lähetys epäonnistui ({ $status })
upload-batch = { $count } tiedoston erä
upload-batch-failed = erä epäonnistui ({ $status })
upload-finish = lähetyksen viimeistely

## Lähetysjono

upload-eta-seconds = { $s } s jäljellä
upload-eta-minutes = { $min } min jäljellä
upload-eta-hours = { $h } h jäljellä
upload-queue = Tuonnit
upload-importing =
    { $count ->
        [one] Tuodaan { $count } paketti
       *[other] Tuodaan { $count } pakettia
    }
upload-bytes-of = { $sent } / { $total }
upload-rate = { $size }/s
upload-waiting = Odottaa
upload-unpacking = Lähetetty, puretaan
upload-already = Jo kirjastossa
upload-stopped = Pysäytetty
upload-files-of = { $done } / { $total } tiedostoa
upload-resumed = jatkettu, { $size } oli jo perillä
upload-skipped = Täsmälleen nämä tiedostot oli jo tuotu nimellä ”{ $name }”, joten ne ohitettiin.
upload-error = { $error }. Edistymisesi on tallessa: yritä uudelleen tai pudota samat tiedostot myöhemmin uudelleen.
upload-retry = Yritä uudelleen
# $name is the pack the library has already that these files go into.
upload-update = Päivitä ”{ $name }”
upload-copy = Tuo kopiona
# $name is the pack the library has already that the upload goes into.
upload-into = Menee pakettiin ”{ $name }”, joka on jo kirjastossa: uudet tiedostot lisätään ja muuttuneet korvataan.
upload-own-pack = Tee siitä oma paketti
upload-dismiss = Kuittaa
upload-cancel = Peruuta
upload-minimize = Pienennä
upload-expand = Näytä tuonnit
upload-pill = Tuodaan { $count } · { $percent } %
upload-pill-stopped = Pysähtyneet: { $count }
upload-interrupted = Keskeytynyt
upload-resume = Jatka
upload-resume-all = Jatka kaikkia
upload-resume-hint-folder = Valitse sama kansio uudelleen tai pudota se sivulle, niin tuonti jatkuu siitä, mihin se jäi.
upload-resume-hint-file = Valitse sama tiedosto uudelleen tai pudota se sivulle, niin tuonti jatkuu siitä, mihin se jäi.
upload-not-same = Nämä eivät ole samat tiedostot, joten mitään ei jatkettu.
upload-permission-denied = Selain ei saanut lukea tiedostoja uudelleen. Valitse ne sen sijaan.

## Paketit

packs-rename-failed = Uudelleennimeäminen epäonnistui: { $error }
packs-collection-deleted = Kokoelma { $name } poistettiin
packs-delete-failed = Poisto epäonnistui: { $error }
packs-status-uploading = Lähetetään (tähän mennessä vastaanotetut tiedostot)
packs-status-ingesting = Puretaan ja lisätään tunnisteita
packs-status-analyzing = LLM lisää tunnisteita
packs-status-ready = Valmis
packs-status-failed = Epäonnistui
packs-retag-failed = Tunnisteiden päivityksen aloitus epäonnistui: { $error }
packs-read-files-failed = Paketin tiedostojen luku epäonnistui: { $error }
packs-context-load-failed = Paketin kauppatietojen lataus epäonnistui: { $error }
packs-undo-store-failed = Kauppasivun poisto epäonnistui: { $error }
packs-context-save-failed = Kauppatietojen tallennus epäonnistui: { $error }
packs-unreal-load-failed = Unreal-muunnoksen tilan lataus epäonnistui: { $error }
packs-unreal-discard-title = Hylätäänkö alkuperäiset?
packs-unreal-discard-body = Paketin { $name } alkuperäiset .uasset-tiedostot poistetaan muuntamisen jälkeen. Niiden vain Unrealissa käytettävät tiedot, kuten materiaaligraafit, menetetään lopullisesti.
packs-unreal-discard-confirm = Muunna ja hylkää
packs-unreal-convert-failed = Muunnoksen aloitus epäonnistui: { $error }
packs-unreal-fact-converted = { $done } muunnettu Unrealista
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal-paketti
       *[other] { $left } Unreal-pakettia
    }
packs-unreal-fact-both = { $done } muunnettu Unrealista, { $left } yhä paketteina
packs-thumbs-canceled = Peruttu, { $done }/{ $total } valmiina
packs-thumbs-drew =
    { $count ->
        [one] Piirrettiin { $count } malli
       *[other] Piirrettiin { $count } mallia
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } pikkukuva korvattu.
       *[other] { $lead }: { $improved } pikkukuvaa korvattu.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } pikkukuva korvattu, { $failed } ei voitu piirtää.
       *[other] { $lead }: { $improved } pikkukuvaa korvattu, { $failed } ei voitu piirtää.
    }
packs-archived =
    { $count ->
        [one] Arkistoitiin { $name } ja sen { $count } resurssi
       *[other] Arkistoitiin { $name } ja sen { $count } resurssia
    }
packs-blobs-bad =
    { $count ->
        [one] { $count } tiedosto ei läpäissyt viimeisintä eheystarkistusta.
       *[other] { $count } tiedostoa ei läpäissyt viimeisintä eheystarkistusta.
    }
packs-collections = Kokoelmat
packs-browse-named = Selaa: { $name }
packs-collection-name = Kokoelman nimi
packs-save = Tallenna
packs-cancel = Peruuta
packs-assets =
    { $count ->
        [one] { $count } resurssi
       *[other] { $count } resurssia
    }
packs-updated = Päivitetty { $date }
packs-browse = Selaa
packs-collection-download = Lataa (.zip)
packs-collection-credits = Tekijätiedot
packs-collection-credits-title = Kuka teki paketit, joista nämä resurssit ovat, niiden lisenssit ja niiden pyytämät tekijämaininnat
packs-rename = Nimeä uudelleen
packs-delete = Poista
packs-packs = Paketit
packs-empty = Ei vielä paketteja. Pudota zip tai kansio mihin tahansa tälle sivulle lisätäksesi sellaisen.
packs-find = Etsi paketti
packs-find-placeholder = Etsi pakettia nimen, kaupan, lisenssin tai tunnisteen perusteella
packs-sort = Järjestys
packs-sort-new = Uusimmat ensin
packs-sort-old = Vanhimmat ensin
packs-sort-size = Suurimmat ensin
packs-sort-assets = Eniten resursseja ensin
packs-sort-count = Resurssien määrä
packs-sort-smallest = Pienimmät ensin
packs-sort-fewest = Vähiten resursseja ensin
packs-with-kind = Paketit, joissa on
packs-state = Tila
packs-state-any = Mikä tahansa tila
packs-state-working = Työn alla
packs-state-failed = Epäonnistuneet
packs-state-untagged = Ei vielä kuvattu
packs-state-unlicensed = Ei merkittyä lisenssiä
packs-store = Kauppa
packs-store-any = Mikä tahansa kauppa
packs-store-from = Kaupasta { $store }
packs-store-none = Ei kauppasivua
packs-shown-of = { $shown } / { $total }
packs-clear = Tyhjennä
packs-no-match = Mikään paketti ei vastaa hakua.
packs-clear-all = Tyhjennä haku ja suodattimet
packs-pack-name = Paketin nimi
packs-thumbs-drawing = Piirretään pikkukuvia tässä selaimessa…
packs-thumbs-drawing-count = Piirretään pikkukuvia tässä selaimessa: { $done } / { $total }…
packs-thumbs-stopped = Pysäytetty: { $error }
packs-thumbs-none = Jokaisella mallilla on jo pikkukuva, joka on piirretty kaikkine tekstuureineen.
packs-dismiss = Kuittaa
packs-license = Lisenssi: { $license }
packs-license-tier = Lisenssi: { $license } ({ $tier })
packs-license-not-found = ei löytynyt
packs-from-before = Lähde:
packs-from-after = { "" }
packs-added = Lisätty { $date }
pack-panel-label = Tietoja tästä paketista
pack-panel-hide = Piilota paketin tiedot
pack-panel-kinds = Mitä siinä on
pack-panel-all = Kaikki paketit
packs-download = Lataa paketti (.zip)
packs-store-details = Kauppatiedot
packs-more-named = Lisää toimintoja: { $name }
packs-more = Lisää
packs-rename-more = Nimeä uudelleen…
packs-retag-rules = Aja sääntöpohjainen tunnistelija uudelleen
packs-retag-llm = Aja LLM-tunnistelija uudelleen
packs-make-thumbs = Tee pikkukuvat tässä selaimessa
packs-make-thumbs-title = Avaa jokainen malli vuorollaan näkymättömissä 3D-katselimessa ja käytä sen kuvaa pikkukuvana, jos se on parempi kuin palvelimen tekemä
packs-make-thumbs-title-desktop = Avaa jokainen malli vuorollaan näkymättömissä 3D-katselimessa ja käytä sen kuvaa pikkukuvana, jos se on parempi kuin Hoardin tekemä
packs-read-files = Lue paketin tiedostot uudelleen
packs-credits = Lataa tekijätiedot
packs-credits-title = CREDITS.md: kuka teki paketin, sen lisenssi ja sen pyytämä tekijämaininta
packs-unreal-open = Muunna Unreal-sisältö…
packs-archive = Arkistoi paketti
packs-unreal-intro = Hoard lukee Unreal-paketit itse ja tallentaa niiden sisällön tavallisina tiedostoina: tekstuurit PNG- tai EXR-muodossa, äänet WAV-muodossa ja materiaalit JSON-muodossa. Muunnettu tiedosto korvaa paketin ja säilyttää sen tunnisteet, linkit ja kokoelmat.
packs-unreal-converted =
    { $count ->
        [one] muunnettu
       *[other] muunnettua
    }
packs-unreal-converted-kept =
    { $count ->
        [one] muunnettu (alkuperäinen säilytetty: { $kept })
       *[other] muunnettua (alkuperäinen säilytetty: { $kept })
    }
packs-unreal-pending =
    { $count ->
        [one] ei vielä tarkasteltu
       *[other] ei vielä tarkasteltu
    }
packs-unreal-needs-editor =
    { $count ->
        [one] odottaa Unreal Editoria
       *[other] odottaa Unreal Editoria
    }
packs-unreal-unsupported =
    { $count ->
        [one] ilman yleistä tiedostomuotoa
       *[other] ilman yleistä tiedostomuotoa
    }
packs-unreal-failed =
    { $count ->
        [one] epäonnistui
       *[other] epäonnistui
    }
packs-unreal-reasons = Miksi paketteja jäi jäljelle
packs-unreal-editor-before = Verkot ja animaatiot tarvitsevat Unreal Editorin. Aja koneella, jolle Unreal on asennettu,
packs-unreal-editor-command = hoard unreal-worker --editor <path to UnrealEditor-Cmd>
packs-unreal-editor-after = ; se vie ne FBX-muotoon ja lähettää tulokset tänne.
packs-unreal-discard = Hylkää alkuperäiset .uasset-tiedostot muuntamisen jälkeen
packs-unreal-discard-hint = Vapauttaa tilaa. Vain Unrealissa käytettävät tiedot, kuten materiaaligraafit, menetetään.
packs-unreal-drop = Poista paketit, joilla ei ole yleistä tiedostomuotoa
packs-unreal-drop-hint = Blueprintit, tasot ja muut vain pelimoottorissa toimivat resurssit.
packs-unreal-convert = Muunna
packs-context-hint-before = Mistä paketti on peräisin ja mitä myyjä kertoo siitä. LLM-tunnistelija lukee tämän jokaisen tiedoston kanssa, joten kaupan kuvaus kuten ”120 ice and fire spells” auttaa sitä nimeämään esimerkiksi tiedoston
packs-context-hint-after = .
packs-store-auto = Löytyi automaattisesti: paketti itse mainitsee tämän sivun.
packs-store-not-this = Ei tämä
packs-lookup-label = Hae kaupoista
packs-lookup-placeholder = Paketin nimi kaupassa
packs-lookup-searching = Haetaan…
packs-lookup-search = Hae kaupoista
packs-lookup-hide = Piilota
packs-lookup-clues = Paketin tiedostoista: { $clues }.
packs-lookup-also = Haettiin myös: { $queries }.
packs-lookup-quoted = ”{ $query }”
packs-lookup-nothing = Mikään Fabissa, itch.io:ssa, Kenneyllä tai OpenGameArtissa ei näytä siltä. Kokeile vähemmän tai eri sanoja.
packs-lookup-use = Käytä
packs-lookup-searched = Haettu { $date }.
packs-lookup-again = Hae uudelleen
packs-lookup-again-after = .
packs-lookup-unreachable = Näitä ei tavoitettu tällä kertaa: { $stores }.
packs-lookup-also-look = Katso myös
packs-lookup-links-and = { " " }ja{ " " }
packs-lookup-browser-only = (niistä voi hakea vain selaimessasi); liitä sivun osoite alle.
packs-lookup-open = Etsi kaupoista…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Myyjä
packs-seller-placeholder = Kuka sen teki
packs-license-label = Lisenssi
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Lisenssitaso
packs-license-tier-placeholder = Personal, Professional…
packs-store-page = Kauppasivu
packs-store-text = Kuvaus kaupasta
packs-store-text-placeholder = Liitä myyntisivun teksti tai jätä tyhjäksi, niin se haetaan kauppasivulta.
packs-store-text-fetch = Tallennus hakee kuvauksen kauppasivulta.
packs-readme = Paketista löytynyt README
packs-save-retag = Tallenna ja aja LLM-tunnistelija uudelleen

## Resurssin tiedot

asset-lazy-3d-viewer = 3D-katselin
asset-lazy-text = teksti
asset-lazy-text-viewer = tekstikatselin
asset-lazy-image-viewer = kuvakatselin
asset-lazy-material-preview = materiaalin esikatselu
asset-lazy-material-viewer = materiaalikatselin
asset-lazy-sprite-player = sprite-soitin
asset-lazy-sheet-view = sprite-arkkikatselin
asset-rel-texture = tekstuuri
asset-rel-buffer = geometriadata
asset-rel-material = materiaalitiedosto
asset-rel-animation = animaatio
asset-rel-library = linkitetty tiedosto
asset-rel-guess = kuuluu luultavasti yhteen
asset-rel-sidecar = kauppatiedot
asset-rel-variant = muunnelma
asset-rel-sprite = sprite
asset-rel-sample = näyte
asset-role-map = kartta: { $role }
asset-maps-count =
    { $count ->
        [one] { $count } kartta
       *[other] { $count } karttaa
    }
asset-needs = Tarvitsee
asset-animations = Animaatiot
asset-sprites = Spritet
asset-used-by = Käyttäjät
asset-loose-sprites = Myös erillisinä tiedostoina
asset-in-sheet = Sprite-arkissa
asset-other-formats = Muut muodot
asset-shown-as = Näytetään kirjastossa muodossa
asset-conversion-failed = muunnos epäonnistui
asset-as-converting = Muunnetaan…
asset-as-converting-percent = Muunnetaan… { $percent } %
asset-as-queued = Jonossa…
asset-download = Lataa
asset-as-label = Lataa muodossa
asset-as-original = { $format } (alkuperäinen muoto)
asset-as-pack-files = Paketin omat tiedostot
asset-as-size = Koko
asset-as-full-size = Täysi koko
asset-as-up-to = Enintään { $size } px
asset-as-hint = Valitse koko tai toinen muoto
asset-meta-size = Koko
asset-meta-length = Kesto
asset-meta-sample-rate = Näytteenottotaajuus
asset-meta-channels = Kanavat
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bittisyvyys
asset-meta-bits = { $bits }-bittinen
asset-meta-level = Taso
asset-meta-level-value = huippu { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = huippu { $peak } dBFS, RMS { $rms } dBFS, leikkautuu
asset-meta-loop = Silmukka
asset-meta-loop-value = { $start }–{ $end }
asset-meta-playback = Toisto
asset-meta-one-shot = Kertatoisto
asset-meta-envelope = Verhokäyrä
asset-meta-envelope-times = nousu { $attack }, häivytys { $tail }
asset-meta-envelope-hit = isku
asset-meta-envelope-swell = paisuva
asset-meta-envelope-sustained = kestävä
asset-meta-tone = Sävy
asset-meta-tone-value = kirkkaus { $khz } kHz, kohina { $noise } %
asset-meta-pitch = Sävelkorkeus
asset-meta-pitch-rises =
    { $octaves ->
        [one] nousee { $octaves } oktaavin
       *[other] nousee { $octaves } oktaavia
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] laskee { $octaves } oktaavin
       *[other] laskee { $octaves } oktaavia
    }
asset-meta-loops-cleanly = Toistuu saumattomasti
asset-meta-tempo = Tempo
asset-meta-tempo-heard = noin { $bpm } BPM, kuulon perusteella
asset-meta-tempo-heard-or = noin { $bpm } BPM (tai { $alt }), kuulon perusteella
asset-meta-key = Sävellaji
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, kuulon perusteella
music-key-major = { $note }-duuri
music-key-minor = { $note }-molli
asset-meta-root-note = Pohjasävel
asset-meta-cue-markers = Cue-merkit
asset-meta-library = Kirjasto
asset-meta-keywords = Avainsanat
asset-meta-title = Nimi
asset-meta-artist = Esittäjä
asset-meta-album = Albumi
asset-meta-genre = Lajityyppi
asset-meta-generator = Generaattori
asset-meta-everything-with = Näytä kaikki, joilla on tunniste { $tag }
asset-meta-saved-with = Tallennettu ohjelmalla
asset-meta-color-mode = Väritila
asset-meta-layers = Tasot
asset-meta-range = Alue
asset-meta-hdr = HDR (esikatselu on sävykartoitettu)
asset-meta-format = Muoto
asset-meta-large-document = Suuri dokumentti (PSB)
asset-meta-ai = Tekoäly
asset-meta-ai-made = Tehty generatiivisella tekoälyllä
asset-meta-ai-edited = Muokattu generatiivisella tekoälyllä
asset-meta-ai-how-metadata = metatiedot
asset-meta-ai-how-generator = generaattorin asetukset
asset-meta-ai-how-iptc = IPTC digital source type
asset-meta-ai-says = { $made } (lähde: { $how })
asset-meta-ai-by-says = { $made } ({ $by }; lähde: { $how })
asset-meta-provenance = Alkuperä
asset-meta-c2pa = Sisältää C2PA Content Credentials -tiedot
asset-meta-compression = Pakkaus
asset-meta-note = Huomautus
asset-meta-maps = Kartat
asset-meta-sizes =
    { $count ->
        [one] Koko
       *[other] Koot
    }
asset-meta-triangles = Kolmiot
asset-meta-vertices = Kärkipisteet
asset-meta-meshes = Verkot
asset-meta-materials = Materiaalit
asset-meta-animations = Animaatiot
asset-meta-skins = Skinit
asset-meta-unreal-asset = Unreal-resurssi
asset-meta-unreal-saved-with = { $class }, tallennettu: UE { $engine }
asset-meta-package = Paketti
asset-meta-cooked = Cooked (pelikäännös): esikatselut ja lähteet on poistettu
asset-meta-tiling = Toistuvuus
asset-meta-seamless = Saumaton molemmilla akseleilla
asset-meta-analysis-error = Tiedostoa ei voitu lukea kokonaan: { $error }
asset-copy-blocked = Kopiointi epäonnistui: selain esti sen
asset-save-error = Tallennus epäonnistui: { $error }
asset-tag-removed = Tunniste { $tag } poistettu
asset-undo = Kumoa
asset-play-error = Tiedoston toisto epäonnistui: { $error }
asset-copied-hash = Tiiviste kopioitu
asset-copied-toml = toml kopioitu
asset-copied-id = ID kopioitu
asset-copied-link = Linkki kopioitu
asset-origin-before = Tämä tiedosto oli alun perin
asset-origin-class-engine = , { $class } (tallennettu: UE { $engine }).
asset-origin-class = , { $class }.
asset-origin-engine = { " " }(tallennettu: UE { $engine }).
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor vei sen { $date }.
asset-origin-editor-notes = Unreal Editor vei sen { $date } ({ $notes }).
asset-origin-hoard = Hoard luki sen paketista { $date }.
asset-origin-hoard-notes = Hoard luki sen paketista { $date } ({ $notes }).
asset-origin-download = Lataa alkuperäinen .{ $ext }-tiedosto
asset-origin-discarded = Alkuperäinen paketti hylättiin muunnoksen jälkeen.
asset-details = Resurssin tiedot
asset-in-pack = paketissa
asset-previous = Edellinen resurssi
asset-next = Seuraava resurssi
asset-favorite = Suosikki
asset-favorite-on-title = Suosikeissasi (F poistaa sen)
asset-favorite-off-title = Lisää suosikkeihin (F)
asset-share = Jaa
asset-share-title = Jaa linkki tähän resurssiin (avaaja kirjautuu ensin Hoardiin)
asset-layout-panel = Näytä sivupaneelissa
asset-layout-page = Näytä koko sivulla
asset-layout-page-title = Näytä koko sivulla: suuri esikatselu tietojen vieressä
asset-close = Sulje tiedot
asset-rename-title = Napsauta nimetäksesi uudelleen
asset-name = Nimi
asset-views-alt = Malli neljästä suunnasta
asset-inspect-aria = Tarkastele kohdetta { $name } koko näytöllä
asset-inspect-title = Tarkastele: lähennä, monista, kanavat
asset-playback-position = Toistokohta
asset-playback-value = { $current } / { $duration }
asset-pause = Tauko
asset-play = Toista
asset-waveform-drawing = Piirretään aaltomuotoa…
asset-waveform-skipped = Näin suurille tiedostoille ei piirretä aaltomuotoa
asset-play-animation = Toista animaationa
asset-show-views = Näytä neljän näkymän arkki, jonka tunnistelija näki
asset-palette = Kuvan värit
asset-swatch-title = { $hex }, { $percent } % kuvasta. Etsi resursseja, joissa on tätä väriä
asset-swatch-aria = Etsi resursseja värillä { $hex }, { $percent } % tästä kuvasta
asset-more-like-this = Lisää samankaltaisia
asset-desc-label = Oma kuvauksesi
asset-desc-placeholder = Mikä se on, mihin sitä käyttäisit…
asset-save = Tallenna
asset-cancel = Peruuta
asset-desc-family = Tallentuu kaikkiin { $count } muunnelmaan
asset-desc-edit = Muokkaa kuvausta
asset-desc-write = Kirjoita kuvaus
asset-desc-ai = Tekoälyn kirjoittama
asset-desc-file = Tiedostosta
asset-desc-store-before = Lähde:
asset-desc-store-page = { $store }-sivu
asset-desc-store-page-unnamed = kauppasivu
asset-desc-store-after = (koko paketti)
asset-less = Vähemmän
asset-more = Lisää
asset-tags = Tunnisteet
asset-tag-add-label = Lisää tunniste
asset-tag-add-placeholder = Lisää tunniste, esim. mood:tense
asset-tag-add = Lisää
asset-apply-family = Käytä tunnisteen, luokan ja kuvauksen muutoksia kaikkiin { $count } muunnelmaan
asset-collections = Kokoelmat
asset-collection-remove = Poista kokoelmasta { $name }
asset-collection-add-another = Lisää toiseen
asset-collection-add = Lisää kokoelmaan
asset-used-in = Käytössä
asset-used-in-title = Näytä, mitä projektissa { $name } on
asset-used-in-synced = synkronoitu { $when }
asset-maps = Kartat
asset-show-more = Näytä { $count } lisää
asset-materials-heading =
    { $count ->
        [one] Materiaali
       *[other] Materiaalit
    }
asset-material-maps-aria = { $name }: kartat
asset-variants =
    { $count ->
        [one] Muunnelmat ({ $count } versio)
       *[other] Muunnelmat ({ $count } versiota)
    }
asset-fit-animations = Sopivat animaatiot ({ $count })
asset-fit-characters = Hahmot, joihin se sopii ({ $count })
asset-copies = Identtiset kopiot
asset-copies-note = Samat tavut tallennetaan kerran; nämäkin paketit sisältävät tämän tiedoston.
asset-near-copies = Lähes samat kopiot
asset-near-copies-note = Sama asia eri koossa, näytteenottotaajuudella tai muodossa. Paras kopio on merkitty.
asset-near-copies-best = Sama asia eri koossa, näytteenottotaajuudella tai muodossa. Tämä on paras kopio.
asset-near-best = paras
asset-unreal-converted = Muunnettu Unrealista
asset-rule-converted = Muunnettu säännöllä
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Muunnossääntö teki tämän { $format }-tiedoston { $date } tiedostosta
asset-rule-kept-until = Alkuperäinen säilytetään { $date } asti.
asset-rule-rebuilt = FLAC säilyttää kaiken, mitä WAV-tiedostossa oli, joten WAV voidaan milloin tahansa ladata tai palauttaa täsmälleen sellaisena kuin se oli.
asset-rule-put-back = Palauta alkuperäinen
asset-rule-put-back-title = Palautetaanko { $file }?
asset-rule-put-back-body = Muunnettu tiedosto poistetaan, ja alkuperäinen palaa sen paikalle tunnisteineen, kokoelmineen ja linkkeineen. Muunnossäännöt eivät sen jälkeen enää koske siihen.
asset-rule-put-back-done = { $file } on palautettu
asset-rule-let-go = Alkuperäinen poistettiin, kun Asetuksissa määritetty alkuperäisten säilytysaika päättyi.
asset-unreal-package = Unreal-paketista
asset-ue-needs-editor-before = Odottaa Unreal Editoria: { $reason }. Aja koneella, jolle Unreal on asennettu,
asset-ue-needs-editor-after = niin se palautuu FBX-muodossa.
asset-ue-editor-path = path to UnrealEditor-Cmd
asset-ue-unsupported = Säilytetty pakettina: { $reason }.
asset-ue-failed = Muunnos epäonnistui: { $reason }
asset-recovered = Alkuperäinen .{ $ext }-tiedosto oli tallennettu tämän paketin sisään, ja Hoard poimi sen.
asset-recovered-download = Lataa .{ $ext }
asset-ue-meshes-before = Verkot ja animaatiot on tallennettu Unrealin omaan muotoon. Muunna paketti (paketin sivu, Lisää) ja aja sitten
asset-ue-meshes-after = Unreal Editor -asennuksen vieressä saadaksesi FBX-tiedostot.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } asset registry -tunniste
       *[other] { $count } asset registry -tunnistetta
    }
asset-file = Tiedosto
asset-bytes = Tavut
asset-source = Lähde
asset-path = Polku
asset-path-open-folder = Katso, mitä muuta kansiossa { $folder } on
asset-download-maps =
    { $count ->
        [one] Lataa { $count } kartta
       *[other] Lataa kaikki { $count } karttaa
    }
asset-download-linked =
    { $count ->
        [one] Lataa { $count } linkitetyn tiedoston kanssa
       *[other] Lataa { $count } linkitetyn tiedoston kanssa
    }
asset-download-only-title = Vain tämä tiedosto ilman tiedostoja, joita se käyttää
asset-download-only = Vain tämä tiedosto
asset-include-animations = Sisällytä animaatiot .zip-tiedostoon
asset-include-character = Sisällytä hahmo { $name }
asset-copy-toml = Kopioi hoard.toml-merkintä
asset-copy-id = Kopioi ID
asset-teach = Opeta Hoardille tämä tiedosto
asset-teach-title = Kuvaile tämän kaupan tiedosto, jotta Hoard osaa lukea sen (Asetukset → Kauppatiedostot)
asset-archive = Arkistoi
asset-archive-title = Poissa hauista, kunnes se palautetaan tai poistetaan lopullisesti arkistosta

## Kuvakatselin

image-channel-red = Punainen kanava
image-channel-green = Vihreä kanava
image-channel-blue = Sininen kanava
image-channel-alpha = Alfakanava
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Ruudukkotausta (K vaihtaa)
image-bg-black-title = Musta tausta (K vaihtaa)
image-bg-gray-title = Harmaa tausta (K vaihtaa)
image-bg-white-title = Valkoinen tausta (K vaihtaa)
image-bg-checker = ruudukkotausta
image-bg-black = musta tausta
image-bg-gray = harmaa tausta
image-bg-white = valkoinen tausta
image-inspect-aria = Tarkastele kohdetta { $name }
image-preview-size = esikatselu { $width }×{ $height }, { $bytes }
image-preview = esikatselu
image-resolution = Resoluutio
image-full-title = Lataa täysikokoinen { $width }×{ $height } kuva (F); myös esikatselua pidemmälle lähentäminen lataa sen
image-full-loading = Ladataan täyttä resoluutiota…
image-full-failed = Täysi resoluutio ei latautunut: yritä uudelleen
image-full = Täysi resoluutio
image-zoom = Zoomaus
image-zoom-out-title = Loitonna (−)
image-zoom-out = Loitonna
image-zoom-in-title = Lähennä (+)
image-zoom-in = Lähennä
image-fit-title = Sovita (0)
image-fit = Sovita
image-actual-title = Todelliset pikselit (1)
image-view = Näkymä
image-tile-title = Monista 3×3 saumojen tarkistamiseksi (T)
image-tile = Monista
image-channels = Kanavat
image-close = Sulje (Esc)
image-failed = Tätä kuvaa ei voitu ladata.
image-loading = Ladataan…
image-key-gestures = Vieritä tai nipistä zoomataksesi · vedä panoroidaksesi · kaksoisnapsauta lähentääksesi tai sovittaaksesi
image-key-tile = monista
image-key-background = tausta
image-key-full = täysi resoluutio
image-key-channels = kanavat

## Sprite-soitin

sprite-frame-aria = { $name }, ruutu { $frame }/{ $count }
sprite-failed = Tätä ei voi toistaa.
sprite-loading = Ladataan ruutuja…
sprite-previous = Edellinen ruutu
sprite-next = Seuraava ruutu
sprite-pause = Keskeytä animaatio
sprite-play = Toista animaatio
sprite-animation = Animaatio
sprite-all-frames = Kaikki ruudut
sprite-grid-title = Miten arkki jaetaan ruutuihin
sprite-columns = Sarakkeet
sprite-rows = Rivit
sprite-fps-aria = Ruutua sekunnissa
sprite-fps = fps

## Sprite-arkkikatselin ja sen latausmuodot

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
       *[other] { $count } spriteä
    }
sheet-pages =
    { $count ->
        [one] { $count } sivu
       *[other] { $count } sivua
    }
sheet-animations =
    { $count ->
        [one] { $count } animaatio
       *[other] { $count } animaatiota
    }
# The box that narrows the list of sprites by name.
sheet-find = Etsi sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Näytetään { $shown }/{ $total }
sheet-no-match = Minkään spriten nimessä ei ole tätä.
sheet-failed = Tätä arkkia ei voi lukea.
sheet-loading = Ladataan arkkia…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoardin oma muoto: sivut sellaisinaan, spritet käännettyinä mihin tahansa suuntaan ja rajattuina, mukana animaatiot ja jokaisen ruudun kesto.
sheet-note-texturepacker = Spritet käännettyinä myötäpäivään ja rajattuina, animaatiot nimettyinä, niin kuin PixiJS ja Phaser ne lukevat.
sheet-note-clockwise = Spritet käännettyinä myötäpäivään ja rajattuina. Animaatioita se ei tallenna: ruudut tunnistetaan numeroiduista nimistään.
sheet-note-libgdx = Spritet käännettyinä vastapäivään ja rajattuina. Animaatioita se ei tallenna: ruudut tunnistetaan numeroiduista nimistään.
sheet-note-godot-frames = Spritejä ei käännetä, ja rajaus säilyy reunuksina; mukana animaatiot ja jokaisen ruudun kesto.
sheet-note-godot-tileset = Laattoja ei käännetä eikä rajata, ne asetellaan laatan kokoiseen ruudukkoon, ja kuvat säilyvät ennallaan, jos ne ovat jo ruudukossa.
sheet-note-grid = Jokainen sprite pystyssä koko kuva-alallaan, samankokoisissa soluissa, mille tahansa ohjelmalle, joka leikkaa arkin ruudukoksi.
sheet-note-frames = Jokainen sprite pystyssä koko kuva-alallaan, kukin omana PNG-kuvanaan, joka on nimetty spriten mukaan.

## Tekstikatselin

text-lang-config = Määritys
text-lang-table = Taulukko
text-lang-code = Koodi
text-lang-script = Skripti
text-lang-text = Teksti
text-copy-blocked = Kopiointi epäonnistui: selain esti sen
text-read-aria = Lue { $name }
text-lines =
    { $count ->
        [one] { $count } rivi
       *[other] { $count } riviä
    }
text-show-as = Näytä muodossa
text-tree-title = Kokoontaitettava puu (T)
text-tree = Puu
text-pretty-title = Sisennetty uudelleen (P)
text-pretty = Siisti
text-raw-title = Kuten kirjoitettu (R)
text-raw = Raaka
text-document-title = Dokumenttina (D)
text-document = Dokumentti
text-source-title = Markdown kirjoitettuna (R)
text-source = Lähde
text-page-title = Kuten selain näyttää sen kansiostaan, erillään Hoardista (P)
text-page = Sivu
text-page-source-title = Sivu sellaisena kuin se on kirjoitettu (R)
text-page-aria = { $name } sivuna
text-page-error = Tätä sivua ei voitu avata: { $error }
text-fold = Taita
text-expand-all-title = Avaa kaikki taitokset
text-expand-all = Laajenna kaikki
text-collapse-all-title = Taita kaikki
text-collapse-all = Supista kaikki
text-group-text = Teksti
text-wrap-title = Rivitä pitkät rivit (W)
text-wrap = Rivitys
text-copy-title = Kopioi teksti (C)
text-copied = Kopioitu
text-copy = Kopioi
text-download-title = Lataa tiedosto
text-download = Lataa
text-close = Sulje (Esc)
text-truncated = Näytetään ensimmäiset { $shown } (yhteensä { $total }).
text-truncated-json = Näytetään ensimmäiset { $shown } (yhteensä { $total }; puu tarvitsee koko tiedoston).
text-show-all = Näytä kaikki
text-json-error = Tätä JSONia ei voi jäsentää: { $error }
text-contents-aria = { $name }: sisältö
text-loading = Ladataan…
text-load-error = Tiedoston lataus epäonnistui: { $error }
text-binary = Tämä tiedosto on binääri eikä tekstiä, joten tässä ei ole mitään luettavaa. Lataa se avataksesi sen omalla työkalullaan.
text-key-tree = puu
text-key-pretty = siisti
text-key-raw = raaka
text-key-document = dokumentti
text-key-source = lähde
text-key-page = sivu
text-key-wrap = rivitys
text-key-copy = kopioi
text-key-find = etsi
text-key-close = sulje
text-preview-aria = Lue { $name } koko näytöllä
text-preview-read = Lue

## JSON-puu

json-items =
    { $count ->
        [one] { $count } kohde
       *[other] { $count } kohdetta
    }
json-keys =
    { $count ->
        [one] { $count } avain
       *[other] { $count } avainta
    }
json-fold = Taita
json-unfold = Avaa
json-unfold-summary = Avaa { $summary }
json-show-more = Näytä { $count } lisää
json-left = (jäljellä yhteensä { $count })
json-picker-more = … { $count } lisää

## 3D-mallikatselimet

model-viewer-failed = 3D-katselin ei latautunut ({ $error }); sivun lataaminen uudelleen voi auttaa
model-loading-viewer = Ladataan 3D-katselinta…
model-loading = Ladataan mallia…
model-show-failed = Mallia ei voitu näyttää: { $error }
model-preview-failed = Mallia ei voitu esikatsella: { $error }
model-error-converting = palvelin muuntaa tätä mallia vielä
model-error-converting-desktop = Hoard muuntaa tätä mallia vielä
model-error-empty-collada = tyhjä Collada-tiedosto
model-error-no-loader = .{ $ext }-tiedostoille ei ole selaimen lataajaa
model-dialog = Näytä { $name } 3D:nä
model-title-stats =
    { $triangles ->
        [one] { $triangles } kolmio
       *[other] { $triangles } kolmiota
    } · { $meshes ->
        [one] { $meshes } verkko
       *[other] { $meshes } verkkoa
    } · { $materials ->
        [one] { $materials } materiaali
       *[other] { $materials } materiaalia
    }
model-triangles =
    { $count ->
        [one] { $count } kolmio
       *[other] { $count } kolmiota
    }
model-group-camera = Kamera
model-group-shading = Varjostus
model-group-background = Tausta
model-group-animation = Animaatio
model-group-view = Näkymä
model-group-show = Näytä
model-frame-title = Rajaa malli (F)
model-frame = Rajaa
model-reset-title = Takaisin alkunäkymään (R)
model-reset = Palauta
model-turn-title = Pyöritä hitaasti (A)
model-turn = Pyöritä
model-textures-title = Tekstuurit (T)
model-textures = Tekstuurit
model-full-res-title = Täyden resoluution tekstuurit (H): jokainen tekstuuritiedosto sellaisenaan, oli se kuinka suuri tahansa; pois päältä enintään 2048 px:n kevyet esikatselut
model-full-res = Täysi res.
model-light-title = Ympäristövalo (E)
model-light = Valo
model-grid-title = Lattiaruudukko (G)
model-grid = Ruudukko
model-bg-checker = Ruudukkotausta
model-bg-dark = Tumma tausta
model-bg-gray = Harmaa tausta
model-bg-light = Vaalea tausta
model-bg-env = Ympäristötausta
model-bg-title = { $background } (K vaihtaa)
model-pause-title = Tauko (välilyönti)
model-play-title = Toista (välilyönti)
model-pause = Tauko
model-play = Toista
model-animation-clip = Animaatioleike
model-animation = Animaatio
model-plays-on = Toista hahmolla
model-rest-pose = Lepoasento
model-close-title = Sulje (Esc)
model-full-screen = Koko näyttö
model-open-full-screen = Avaa 3D-katselin koko näytölle
model-menu-fold = Taita valikko piiloon
model-menu-show = Näytä valikko
model-view-prefix = Näkymä:{ " " }
model-menu-render = Renderöinti
model-menu-maps = Kartat
model-menu-inspect = Tarkastelu
model-mode-final = Lopullinen renderöinti
model-mode-raw = Raaka renderöinti
model-mode-color = Perusväri
model-mode-roughness = Karheus
model-mode-metalness = Metallisuus
model-mode-normal = Normaali
model-mode-ao = Varjostus (AO)
model-mode-matcap = Matcap
model-mode-uv = UV-tarkistin
model-mode-geometry = Geometriat
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Rautalanka (W), minkä tahansa näkymän päällä
model-wireframe = Rautalanka
model-info-title = Kohtauksen tiedot (I)
model-info = Kohtauksen tiedot
model-info-close = Sulje kohtauksen tiedot
model-info-close-title = Sulje (I)
model-info-geometries = Geometriat
model-info-materials = Materiaalit
model-info-textures = Tekstuurit
model-info-triangles = Kolmiot
model-info-vertices = Kärkipisteet
model-info-meters = glTF on metreissä
model-info-no-units = Tämä muoto ei kerro yksiköitään
model-info-height = Korkeus
model-info-length = Pituus
model-info-width = Leveys
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } yksikkö
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } yksikköä
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Tekstuuria ei löytynyt: { $name }
model-missing-some = { $count } tekstuuria ei löytynyt: { $names }
model-missing-more = { $count } tekstuuria ei löytynyt: { $names }…
model-missing-count = { $count } tekstuuria ei löytynyt
model-guessed = Tekstuurit yhdistettiin nimen perusteella; tiedosto ei nimeä yhtään.
model-group-facing = Suunta
model-facing-turn = Käännä neljännes
model-facing-turn-title = Käännä mallia neljänneskierros vastapäivään ylhäältä katsottuna
model-facing-tip = Kallista neljännes
model-facing-tip-title = Kallista mallin yläosaa neljänneskierros eteenpäin, jotta makaava malli nousee pystyyn
model-facing-save = Käytä koko paketissa
model-facing-save-title = Käännä näin kaikki paketin { $pack } mallit. Niiden pikkukuvat piirretään uudelleen; lataukset pysyvät sellaisina kuin ne tehtiin.
model-facing-undo = Kumoa
model-facing-undo-title = Takaisin siihen, miten paketin mallit on käännetty
model-facing-saved = Kaikki paketin { $pack } mallit ovat nyt tässä asennossa. Niiden pikkukuvia piirretään uudelleen.
model-facing-failed = Paketin mallien kääntäminen epäonnistui: { $error }
model-keys-mouse = Vedä kiertääksesi · vedä hiiren oikealla panoroidaksesi · vieritä zoomataksesi · kaksoisnapsauta rajataksesi
model-key-frame = rajaa
model-key-reset = palauta
model-key-view = näkymä
model-key-wireframe = rautalanka
model-key-info = kohtauksen tiedot
model-key-textures = tekstuurit
model-key-full-res = täyden resoluution tekstuurit
model-key-light = valo
model-key-grid = ruudukko
model-key-background = tausta
model-key-play = toista

## Materiaalit

material-role-base-color = Väri
material-role-normal = Normaali
material-role-normal-dx = Normaali (DirectX)
material-role-roughness = Karheus
material-role-gloss = Kiilto
material-role-metalness = Metallisuus
material-role-ao = Ympäristövarjostus (AO)
material-role-orm = Varjostus, karheus, metalli
material-role-mra = Metalli, karheus, varjostus
material-role-rma = Karheus, metalli, varjostus
material-role-height = Korkeus
material-role-bump = Kohouma
material-role-cavity = Kolot
material-role-specular = Heijastavuus
material-role-opacity = Peittävyys
material-role-emissive = Hehku
material-role-translucency = Läpikuultavuus
material-role-base-color-name = väri
material-role-normal-name = normaali
material-role-normal-dx-name = normaali (DirectX)
material-role-roughness-name = karheus
material-role-gloss-name = kiilto
material-role-metalness-name = metallisuus
material-role-ao-name = ympäristövarjostus (AO)
material-role-orm-name = varjostus, karheus, metalli
material-role-mra-name = metalli, karheus, varjostus
material-role-rma-name = karheus, metalli, varjostus
material-role-height-name = korkeus
material-role-bump-name = kohouma
material-role-cavity-name = kolot
material-role-specular-name = heijastavuus
material-role-opacity-name = peittävyys
material-role-emissive-name = hehku
material-role-translucency-name = läpikuultavuus
material-maps =
    { $count ->
        [one] { $count } kartta
       *[other] { $count } karttaa
    }
material-dialog = Näytä materiaali { $name }
material-up-to = enintään { $size }
material-repeat-size = toisto { $size }
material-shape-sphere = Pallo
material-shape-cube = Kuutio
material-shape-plane = Taso
material-shape-title = { $shape } ({ $key })
material-group-shape = Muoto
material-group-tiling = Toistuvuus
material-real-title = Todellinen koko: yksi toisto on { $size }, pallon halkaisija 2 m (T vaihtaa)
material-repeat-title = Toistuu { $n }× (T vaihtaa)
material-real = Todellinen
material-repeat = { $n }×
material-height-title = Kuinka paljon korkeuskartta työntää pintaa
material-height = Korkeus
material-height-strength = Korkeuden voimakkuus
material-loading = Ladataan materiaalia…
material-show-failed = Materiaalia ei voitu näyttää: { $error }
material-preview-failed = Materiaalia ei voitu esikatsella: { $error }
material-group-maps = Kartat
material-map-show = Näytä kartta: { $map }
material-map-hide = Piilota kartta: { $map }
material-map-unused = Esikatselu ei käytä tätä
material-open-full-screen = Avaa materiaalikatselin koko näytölle
material-keys-mouse = Vedä kiertääksesi · vedä hiiren oikealla panoroidaksesi · vieritä zoomataksesi · napsauta karttaa kytkeäksesi sen pois
material-key-shape = muoto
material-key-tiling = toistuvuus
material-key-turn = pyöritä

## Teemat

theme-slate = Liuske
theme-paper = Paperi
theme-linen = Pellava
theme-sage = Salvia
theme-lilac = Sireeni
theme-blush = Punerrus
theme-graphite = Grafiitti
theme-obsidian = Obsidiaani
theme-ember = Hiillos
theme-moss = Sammal
theme-aubergine = Munakoiso
theme-oxblood = Häränveri
theme-light-slate-blurb = Viileä siniharmaa
theme-light-paper-blurb = Pelkistetty neutraali harmaa
theme-light-linen-blurb = Lämmin kerma, meripihka
theme-light-sage-blurb = Pehmeä vihreä
theme-light-lilac-blurb = Vaalea violetti
theme-light-blush-blurb = Himmeä ruusu
theme-dark-graphite-blurb = Neutraali harmaa, ei sävyä
theme-dark-slate-blurb = Siniharmaa
theme-dark-obsidian-blurb = Lähes musta, OLED-näytöille
theme-dark-ember-blurb = Lämmin hiilenharmaa, meripihka
theme-dark-moss-blurb = Metsänvihreä-musta
theme-dark-aubergine-blurb = Luumunmusta, sireeni
theme-dark-oxblood-blurb = Viininmusta, koralli

## Tekstuurien nimet

texnames-needs-map = Mallissa on oltava { "{" }map{ "}" }, muuten materiaalin jokainen kartta saisi saman nimen
texnames-placeholders = Mallissa voi käyttää kenttiä { "{" }name{ "}" }, { "{" }map{ "}" } ja { "{" }size{ "}" }
texnames-bad-characters = Nimissä ei voi olla merkkejä < > : " / \ | ? tai *
texnames-no-suffix = Anna pääte karttatyypille { $role }
texnames-bad-suffix = Karttatyypin { $role } päätteessä ei voi olla merkkejä < > : " / \ | ? tai *
texnames-slot-no-suffix = Anna paikalle ”{ $slot }” pääte
texnames-slot-bad-suffix = Paikan ”{ $slot }” päätteessä ei voi olla merkkejä < > : " / \ | ? tai *
texnames-ending-missing = Jokainen oma loppuosa tarvitsee etsittävän loppuosan
texnames-ending-bad = Omissa loppuosissa ei voi olla merkkejä < > : " / \ | ? tai *
texnames-ending-twice = { $ending } on loppuosissasi kahdesti
texnames-slot-metallic-roughness = Metalli ja karheus
texnames-slot-specular-color = Heijastavuuden väri
texnames-slot-specular-glossiness = Heijastavuus ja kiilto
texnames-slot-clearcoat = Lakka
texnames-slot-clearcoat-roughness = Lakan karheus
texnames-slot-clearcoat-normal = Lakan normaali
texnames-slot-sheen-color = Sametinhohdon väri
texnames-slot-sheen-roughness = Sametinhohdon karheus
texnames-slot-transmission = Läpäisy
texnames-slot-thickness = Paksuus
texnames-slot-diffuse-transmission = Diffuusi läpäisy
texnames-slot-diffuse-transmission-color = Diffuusin läpäisyn väri
texnames-slot-iridescence = Irisointi
texnames-slot-iridescence-thickness = Irisoinnin paksuus
texnames-slot-anisotropy = Anisotropia

## Kauppatiedostojen muodot

storefiles-field-title = Otsikko
storefiles-field-title-hint = Myyntisivun tarkka otsikko. Se nimeää paketin, ja ilman sitä tiedosto ohitetaan.
storefiles-field-seller = Myyjä
storefiles-field-seller-hint = Kuka sen teki tai myy.
storefiles-field-license = Lisenssi
storefiles-field-license-hint = Kuten tiedosto sen kirjoittaa, esim. CC-BY-4.0.
storefiles-field-url = Kauppasivu
storefiles-field-url-hint = Myyntisivun osoite, joka alkaa http:// tai https://.
storefiles-field-words = Tunnisteet
storefiles-field-words-hint = Sanat, joilla se merkitään. Pilkuilla erotetut arvot jaetaan.
storefiles-field-type = Tyyppi
storefiles-field-type-hint = Mikä se on, esim. ”3D model” tai ”surface”. Myös tunniste.
storefiles-field-colors = Värit
storefiles-field-colors-hint = Sen värit color:-tunnisteina.
storefiles-field-id = Kaupan tunniste
storefiles-field-id-hint = Kaupan sille antama tunniste: kirjaimia, numeroita, _ tai -.
storefiles-field-origin = Tekopaikka
storefiles-field-origin-hint = Missä se tehtiin tai skannattiin, kuvausta varten.
storefiles-field-size = Koko
storefiles-field-size-hint = Kuinka suuri se on sanoin, kuvausta varten.
storefiles-field-title-name = otsikko
storefiles-field-seller-name = myyjä
storefiles-field-license-name = lisenssi
storefiles-field-url-name = kauppasivu
storefiles-field-words-name = tunnisteet
storefiles-field-type-name = tyyppi
storefiles-field-colors-name = värit
storefiles-field-id-name = kaupan tunniste
storefiles-field-origin-name = tekopaikka
storefiles-field-size-name = koko

## Kauppatiedostot

storefiles-load-failed = Kauppatiedostojen muotojen lataus epäonnistui: { $error }
storefiles-remove-title = Poistetaanko { $name }?
storefiles-remove-body = { $file } poistetaan datakansiosta. Sen avulla jo luetut paketit säilyttävät luetut tiedot.
storefiles-remove = Poista
storefiles-removed = Poistettu: { $name }.
storefiles-remove-failed = Muodon { $name } poisto epäonnistui: { $error }
storefiles-unread = Tiedostot, joita Hoard ei lue
storefiles-loading = Ladataan…
storefiles-unread-none = Ei yhtään: mikään kirjaston tiedosto ei näytä lukemattomalta kauppatiedostolta.
storefiles-unread-what = Useammassa kuin yhdessä paketissa olevat tai kauppatiedoston tapaan nimetyt tiedostot, joita mikään muoto ei lue.
storefiles-unread-packs =
    { $count ->
        [one] { $count } paketti
       *[other] { $count } pakettia
    }
storefiles-unread-example = esim. { $pack }
storefiles-teach = Opeta Hoardille tämä tiedosto
storefiles-formats = Muodot
storefiles-formats-what = Omiasi kokeillaan ensin, ja ne korvaavat samannimisen sisäänrakennetun muodon.
storefiles-dir-before = Niitä säilytetään kansiossa
storefiles-dir-after = .
storefiles-yours = Oma
storefiles-built-in = Sisäänrakennettu
storefiles-edit = Muokkaa
storefiles-start-from = Aloita tästä

## Tunnisteet

tags-load-failed = Tunnisteiden lataus epäonnistui: { $error }
tags-rename-title = Nimetäänkö tunniste { $tag } uudelleen?
tags-merge-title = Yhdistetäänkö { $count } tunnistetta?
tags-rename-body =
    { $count ->
        [one] Sen { $count } resurssi saa sen sijaan tunnisteen { $into }, ja tunnistelijat käyttävät tästä lähtien uutta nimeä.
       *[other] Sen { $count } resurssia saa sen sijaan tunnisteen { $into }, ja tunnistelijat käyttävät tästä lähtien uutta nimeä.
    }
tags-merge-body = { $tags } muuttuvat tunnisteeksi { $into } enintään { $count } resurssissa. Tunnistelijat käyttävät tästä lähtien tunnistetta { $into }.
tags-rename = Nimeä uudelleen
tags-merge = Yhdistä
tags-merged =
    { $count ->
        [one] { $count } resurssilla on nyt tunniste { $into }
       *[other] { $count } resurssilla on nyt tunniste { $into }
    }
tags-merge-failed = Yhdistäminen epäonnistui: { $error }
tags-forget-failed = Muutos epäonnistui: { $error }
tags-find = Etsi tunnisteita
tags-find-placeholder = Etsi tunnisteita, esim. rock tai set:
tags-count =
    { $count ->
        [one] { $count } tunniste
       *[other] { $count } tunnistetta
    }
tags-count-more = { $count }+ tunnistetta
tags-what = Valitse samaa tarkoittavat tunnisteet ja yhdistä ne, tai valitse yksi nimetäksesi sen uudelleen.
tags-new-name = Uusi nimi
tags-merge-into = Yhdistä tunnisteeseen
tags-saved-as-before = Tallennettu nimellä
tags-saved-as-after = { "" }
tags-cancel = Peruuta
tags-loading = Ladataan…
tags-none = Ei tunnisteita.
tags-none-containing = Ei tunnisteita, joissa on ”{ $text }”.
tags-browse-title = Selaa resursseja, joilla on tunniste { $tag }
tags-browse = Selaa
tags-merged-heading = Yhdistetyt tunnisteet
tags-merged-what = Kun tunnistelijat ehdottavat jotakin näistä vanhoista nimistä, resurssi saa tunnisteen, johon se yhdistettiin.
tags-allow-old = Salli vanha nimi uudelleen

## Kirjaston tilastot

stats-source-human = Ihmisten lisäämät
stats-source-ai = Tekoälyn ehdottamat
stats-source-rule = Sisäänrakennetut säännöt
stats-source-meta = Tiedostoista
stats-assets = Resurssit
stats-packs = Paketit
stats-on-disk = Levyllä
stats-duplicates-title =
    { $count ->
        [one] { $count } tiedosto on tavulleen identtinen kopio toisesta ja tallennetaan kerran
       *[other] { $count } tiedostoa on tavulleen identtisiä kopioita muista ja tallennetaan kerran
    }
stats-saved-dedup = Säästetty kaksoiskappaleiden poistolla
stats-distinct-tags = Eri tunnisteita
stats-archive = Arkistossa
stats-load-failed = Tietojen lataus epäonnistui: { $error }
stats-counting = Lasketaan…
stats-as-of = Tilanne { $time }
stats-updating = Päivitetään…
stats-by-kind = Tila tyypeittäin
stats-files-size =
    { $count ->
        [one] { $name }: { $count } tiedosto, { $size }
       *[other] { $name }: { $count } tiedostoa, { $size }
    }
stats-months = Lisätty viimeisen vuoden aikana
stats-month =
    { $count ->
        [one] { $month }: { $count } resurssi, { $size }
       *[other] { $month }: { $count } resurssia, { $size }
    }
stats-tagging = Tunnisteet
stats-tagged = Tunnisteelliset
stats-untagged = { $count } ilman tunnisteita
stats-described = Kuvatut
stats-described-assets =
    { $count ->
        [one] { $count } resurssi
       *[other] { $count } resurssia
    }
stats-human-edited = Käsin muokatut
stats-human-edited-unit =
    { $count ->
        [one] resurssi
       *[other] resurssia
    }
stats-source-different =
    { $count ->
        [one] { $count } erilainen
       *[other] { $count } erilaista
    }
stats-most-used = Käytetyimmät
stats-show-tag = Näytä resurssit, joilla on tunniste { $tag }
stats-file-types = Tiedostotyypit
stats-biggest-packs = Suurimmat paketit
stats-browse-pack = Selaa: { $name }
stats-largest-files = Suurimmat tiedostot
stats-open-file = Avaa { $name }
stats-empty = Kirjastossa ei ole vielä mitään. Lisää paketti, niin sen luvut näkyvät täällä.

## HoardPanel.vue

hoard-title = Aarteesi
hoard-what = Jokainen kirjaston resurssi on aarteen kappale, ja sen laji ratkaisee, mikä. Kerää uudet, niin ne lisätään kasaasi.
hoard-sign-in = Kirjaudu sisään kerätäksesi oman aarteen.
hoard-new = Uutta saalista kerättävänä: { $count }
hoard-none-new = Ei mitään uutta kerättävää.
hoard-collect = Kerää
hoard-unavailable = Tämä selain ei pysty piirtämään aarretta: { $error }
hoard-collect-failed = Keräämiäsi ei voitu tallentaa: { $error }
hoard-arcade = Pelihallinäyttö
hoard-full-screen = Koko näyttö
hoard-leave-full-screen = Poistu koko näytöstä
hoard-label = Lohikäärme vartioi aarrettaan. Aarteita: { $count }
hoard-gold = Kultaa
hoard-gems = Jalokiviä
hoard-books = Kirjoja
hoard-weapons = Aseita
hoard-bones = Luita
hoard-tablets = Riimutauluja
hoard-paintings = Maalauksia
hoard-potions = Juomia

## Päivitykset

updates-saved = Hoard kutsuu sitä, kun päivitys on valmis
updates-saved-none = Ei kutsuttavaa osoitetta
updates-save-failed = Tallennus epäonnistui
updates-call-answered = Se vastasi { $result }
updates-call-refused = Se ei hyväksynyt pyyntöä: { $result }
updates-call-failed = Kutsu epäonnistui
updates-called = Kutsuttu { $when } versiolle { $version }: { $result }
updates-called-failed = Kutsuttu { $when } versiolle { $version }: { $result }. Hoard yrittää uudelleen tunnin kuluttua.
updates-latest = Hoard { $version } on uusin versio.
updates-out = Hoard { $version } on julkaistu.
updates-out-idle = Hoard { $version } on julkaistu; kutsu tehdään parin minuutin sisällä, kun mitään ei ole käynnissä.
updates-out-busy = Hoard { $version } on julkaistu; odotetaan, kunnes mitään ei ole käynnissä ({ $work }).
updates-busy-more =
    { $count ->
        [one] { $count } muu
       *[other] { $count } muuta
    }
updates-busy-jobs =
    { $count ->
        [one] { $count } työ odottaa
       *[other] { $count } työtä odottaa
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } lähetys
       *[other] { $count } lähetystä
    }
updates-list-separator = ,{ " " }
updates-what = Kun uusi versio julkaistaan, Hoard voi kutsua osoitetta heti, kun mitään ei ole käynnissä (ei lähetyksiä, töitä, muunnoksia eikä ylläpitoa), jottei sen konttia päivittävä työkalu käynnistä sitä uudelleen kesken lähetyksen. Se lähettää POST-pyynnön, jonka JSON-rungossa on versio. Tämä vaatii yllä olevan versiotarkistuksen.
updates-url = Kutsuttava osoite
updates-url-placeholder = Ei mitään (esim. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Tallennettu (kirjoita korvataksesi)
updates-auth-placeholder = Ei mitään (esim. Bearer my-token)
updates-remove = Poista
updates-save = Tallenna
updates-call-now-title = Siihen kytketty toiminto käynnistyy
updates-call-now = Kutsu nyt
updates-what-calls = Mitä se voi kutsua?
updates-watchtower = Watchtowerin HTTP API
updates-watchtower-at = :{ " " }
updates-watchtower-with = , otsakkeella{ " " }
updates-watchtower-token = { " " }ja tunnuksella muuttujasta{ " " }
updates-watchtower-its = { " " }(sen{ " " }
updates-watchtower-endpoint = { " " }-päätepiste käytössä).
updates-portainer = Portainer-webhook
updates-portainer-rest = { " " }Hoardin stackissa tai kontissa, re-pull image päällä. Tunnus on URL-osoitteessa.
updates-script = Skripti palvelinkoneella
updates-script-listener = { " " }webhook-kuuntelijan takana (esim.{ " " }
updates-script-runs = ), joka ajaa komennon{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }tai mikä tahansa muu, joka ottaa vastaan webhookin.
updates-poll-before = Tai jätä osoite pois: käyttöoikeustunnusta käyttävä skripti voi kysellä osoitetta{ " " }
updates-poll-until = { " " }kunnes{ " " }
updates-poll-and = { " " }on asetettu ja{ " " }
updates-poll-after = { " " }on true, ja sitten hakea uuden version ja käynnistää Hoardin uudelleen.

## Ylläpito

maintenance-load-failed = Ylläpidon lataus epäonnistui
maintenance-reanalyze-title = Analysoidaanko kaikki paketit uudelleen?
maintenance-reanalyze-body = Jokainen tiedosto luetaan uudelleen, mikä voi kestää pitkään suuressa kirjastossa; paketit näkyvät työn alla olevina, kunnes on niiden vuoro. Omat nimesi, tunnisteesi ja muistiinpanosi säilyvät.
maintenance-reanalyze-confirm = Analysoi uudelleen
maintenance-gc-title = Poistetaanko käyttämättömät tiedostot?
maintenance-gc-body = Tallennetut tiedostot, joihin mikään kirjastossa ei viittaa, poistetaan lopullisesti. Arkistossa olevat paketit säilyttävät omansa.
maintenance-gc-confirm = Poista ne
maintenance-start-failed = Tehtävän ”{ $task }” käynnistys epäonnistui
maintenance-just-now = juuri nyt
maintenance-minutes-ago = { $count } min sitten
maintenance-hours-ago = { $count } h sitten
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Ladataan…
maintenance-running = Käynnissä…
maintenance-starting = Käynnistetään…
maintenance-run = Aja
maintenance-started = { $detail } · aloitettu { $when }
maintenance-starting-detail = Käynnistyy
maintenance-finished = Valmis:{ " " }
maintenance-failed = Epäonnistui:{ " " }
maintenance-never-run = Ei koskaan ajettu
maintenance-history = Aiemmat ajot
maintenance-col-task = Tehtävä
maintenance-col-finished = Valmistui
maintenance-col-took = Kesto
maintenance-col-result = Tulos
maintenance-script-before = Varmuuskopiot ja vauriotarkistuksen voi ajaa myös skriptistä:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }ja{ " " }
maintenance-script-after = . Nekin ajot näkyvät täällä.

## Asetukset

settings-title = Asetukset
settings-loading = Ladataan…
settings-find = Etsi asetus
settings-find-none = Mikään asetus ei vastaa hakua ”{ $query }”.
settings-load-failed = Asetusten lataus epäonnistui: { $error }
settings-page-appearance = Ulkoasu
settings-page-appearance-blurb = Teemat, joita tämä selain käyttää vaaleassa ja tummassa tilassa. Tallennetaan vain tähän selaimeen.
settings-page-dragon = Lohikäärme
settings-page-dragon-blurb = Lohikäärmeesi aarre: jokainen kirjaston tiedosto on palanen sitä.
settings-page-library = Kirjasto
settings-page-library-blurb = Mitä Hoardissa on: kuinka suuri se on, mistä se koostuu, miten se on kasvanut ja kuinka hyvin sen sisältö on merkitty tunnisteilla. Arkistoituja ei lasketa.
settings-page-copies = Lähes samat kopiot
settings-page-copies-blurb = Tiedostot, jotka ovat sama kuva, ääni tai malli eri koossa, näytteenottotaajuudella tai muodossa, eniten tilaa vievät ensin. Kunkin joukon paras on merkitty; mitään ei poisteta.
copies-loading = Etsitään lähes samoja kopioita…
copies-refreshing = Kirjasto on muuttunut näiden löytymisen jälkeen. Etsitään uudelleen…
copies-none = Ei lähes samoja kopioita: jokainen kuva, ääni ja malli on täällä vain kerran.
copies-load-failed = Lähes samoja kopioita ei voitu listata: { $error }
copies-sum = Lähes samojen kopioiden joukkoja: { $count }. Kopiot, jotka eivät ole parhaita, vievät { $size }.
copies-set = Joukko { $n }
copies-set-wasted = { $size } kopioissa, jotka eivät ole parhaita
copies-more = Näytä lisää ({ $count } jäljellä)
settings-page-tags = Tunnisteet
settings-page-tags-blurb = Kirjaston kaikki tunnisteet. Yhdistä samaa tarkoittavat tai nimeä jokin uudelleen; tunnistelijat käyttävät siitä lähtien uutta nimeä.
settings-page-store-files = Kauppatiedostot
settings-page-store-files-blurb = Tiedostot, joita kaupat laittavat sisältönsä viereen ja joista Hoard lukee pakettien nimet ja tekijät. Opeta sille tiedostoja, joita se ei tunne, ja jaa opettamasi.
settings-page-locations = Kirjaston sijainnit
settings-page-locations-blurb = Palvelimen kansiot, joissa kirjasto on: kansiot, joissa Hoard säilyttää tiedostoja, ja pakettikansiot, jotka se indeksoi paikoillaan.
settings-page-locations-blurb-desktop = Kansiot, joissa kirjasto on: kansiot, joissa Hoard säilyttää tiedostoja, ja pakettikansiot, jotka se indeksoi paikoillaan.
settings-page-tools = Työkalut
settings-page-tools-blurb = Ulkoiset ohjelmat, joilla Hoard muuntaa ja lukee tiedostoja.
settings-page-ai = Tekoälytunnisteet
settings-page-ai-blurb = Tekoälymalli nimeää, kuvailee ja merkitsee uudet paketit tunnisteilla sisäänrakennettujen sääntöjen lisäksi. Sen tunnisteet merkitään ehdotetuiksi.
settings-page-search-blurb = Hae kuvia sen mukaan, mitä ne esittävät, ja ääniä sen mukaan, miltä ne kuulostavat, eikä vain nimen perusteella, ja löydä tyypiltään eikä vain väriltään samankaltaisia.
settings-page-search-tab = Haku
settings-page-adding = Pakettien lisäys
settings-page-adding-blurb = Mitä Hoard tekee paketille, kun se lisätään, lukemisen ja tunnisteiden lisäämisen lisäksi.
settings-page-downloads = Lataukset
settings-page-downloads-blurb = Miten tiedostot nimetään siinä, mitä Hoard lähettää: lataukset, zipit, hoard-työkalu ja tekoälyavustajat. Kirjaston tiedostot pysyvät ennallaan.
settings-page-account = Tili
settings-page-account-blurb = Salasanasi, muut kirjautumaan pääsevät ja käyttöoikeustunnukset skripteille ja muille ohjelmille.
settings-page-auth = Todennus
settings-page-auth-blurb = Miten käyttäjät kirjautuvat: Hoardin omat tilit, LDAP-hakemisto, Microsoft Active Directory tai OAuth-palveluntarjoaja, kuinka kauan istunnot kestävät ja mitkä ryhmät saavat minkäkin roolin.
settings-page-connection = Yhteys
settings-page-connection-blurb = Miten selaimet ja sovellukset tavoittavat Hoardin: osoite, jolla se tunnetaan, ja HTTPS, jota jotkin selaimen ominaisuudet tarvitsevat.
settings-page-performance = Suorituskyky
settings-page-performance-blurb = Kuinka kovasti palvelin työskentelee, miten pikkukuvat tehdään ja säilytetään ja paljonko se enintään ottaa vastaan.
settings-page-performance-blurb-desktop = Kuinka kovasti Hoard työskentelee, miten pikkukuvat tehdään ja säilytetään ja paljonko se enintään ottaa vastaan.
settings-page-issues = Ongelmat
settings-page-issues-blurb = Mikä meni pieleen tiedostoja avattaessa tai käsiteltäessä: arkistot, jotka eivät purkautuneet, tiedostot, joita ei voitu lukea, ja epäonnistuneet työt. Yritä niitä uudelleen, kun syy on korjattu, tai kuittaa ne.
settings-page-maintenance = Ylläpito
settings-page-maintenance-blurb = Varmuuskopioi, tarkista vauriot ja siivoa. Kutakin ajetaan yksi kerrallaan, ja voit poistua tältä sivulta niiden ollessa käynnissä.
settings-page-about = Tietoja
settings-page-about-blurb = Mikä Hoard on, mikä versio tämä on ja muiden työ, josta se on tehty.
settings-the-default = oletus
settings-default-on = päällä
settings-default-off = pois
settings-default-auto = automaattinen
settings-option-saved = { $name }: { $value }
settings-back-to-default = Palauta oletus
settings-save = Tallenna
settings-use = Käytä
settings-status-off = Pois päältä

## Asetukset: Ulkoasu

settings-themes = Teemat
settings-look-light = Vaalea
settings-look-dark = Tumma
settings-look-group =
    { $scheme ->
        [light] Vaalea teema
       *[dark] Tumma teema
    }
settings-preview =
    Näytetään { $preview ->
        [light] vaalea
       *[dark] tumma
    } teema, jotta näet sen. Käytät { $used ->
        [light] vaaleaa
       *[dark] tummaa
    } teemaa{ $system ->
        [yes] { " " }(järjestelmän mukaan)
       *[other] { "" }
    }, ja se palaa, kun poistut tältä sivulta.
settings-preview-back =
    Takaisin { $used ->
        [light] vaaleaan
       *[dark] tummaan
    } teemaan
settings-look-hint = Vaihda vaalean ja tumman välillä (tai seuraa järjestelmää) yläpalkista.

## Asetukset: Työkalut

settings-blender-what = Muuntaa .blend-, .dae-, .abc- ja .x3d-mallit taustalla glTF-muotoon, jotta ne saavat pikkukuvat ja 3D-esikatselun. Kun se oli pois päältä lisätyt paketit muunnetaan, kun se on valmis.
settings-blender-download = Käyttää asennettua Blenderiä tai lataa Blender 4.5 LTS:n (noin 400 Mt) blender.orgin peilipalvelimelta.
settings-ffmpeg-what = Koodaa MP3-, OGG- ja Opus-lataukset ja lukee äänen, jota Hoard ei itse osaa purkaa (AAC, M4A, WMA, …).
settings-ffmpeg-download = Käyttää järjestelmän ffmpegiä, jossa on oikeat kooderit, tai lataa staattisen version (noin 80 Mt).
settings-tool-reset = { $tool }: palauta oletus
settings-tool-no-download = Käyttää asennettua ohjelmaa { $tool }. Tälle koneelle ei ole ladattavaa versiota, joten asenna se tai anna sen polku.
settings-tool-path = Ohjelman { $tool } polku
settings-mode-off = Pois
settings-mode-auto = Automaattinen
settings-mode-path = Tämä ohjelma
settings-tool-looking = Etsitään…
settings-tool-downloading = Ladataan… edistyminen näkyy toimintailmaisimessa
settings-tool-ready = Valmis: { $detail }
settings-soundfont = MIDI-SoundFont
settings-soundfont-what = Soittimet, joilla MIDI-tiedostot toistetaan ja muunnetaan.
settings-soundfont-file = SoundFont-tiedosto
settings-soundfont-placeholder = Palvelimelta löytyvä tai sisäänrakennettu
settings-soundfont-placeholder-desktop = Tältä tietokoneelta löytyvä tai sisäänrakennettu
settings-soundfont-default = palvelimelta löytyvä tai sisäänrakennettu
settings-soundfont-default-desktop = tältä tietokoneelta löytyvä tai sisäänrakennettu
settings-soundfont-send = Valitse tiedosto…
settings-soundfont-sent = MIDI soi nyt fontilla { $name }

## Asetukset: Kuvahaku

settings-clip-heading = Hae kuvia sen mukaan, mitä ne esittävät
settings-clip-reset = Kuvahaku: palauta oletus
settings-clip-saved-on = Kuvahaku: päällä
settings-clip-saved-off = Kuvahaku: pois
settings-clip-what = Kirjoita, mitä etsit (”mossy rock”, ”sci-fi door”, ”wooden barrel”), niin sitä esittävät kuvat näkyvät sanat sisältävien nimien rinnalla. ”Lisää samankaltaisia” löytää samantyyppisiä asioita, ei vain samanvärisiä.
settings-clip-toggle = Ota kuvahaku käyttöön
settings-clip-hint = Käyttää Googlen SigLIP 2 -mallia, jota ajetaan tämän palvelimen suorittimella: mitään ei lähde palvelimelta. Käyttöönotto lataa mallin ({ $size } Mt) Hugging Facesta ja opettelee sitten jokaisen kuvan taustalla matalalla prioriteetilla. Se vie 350–750 Mt muistia työskennellessään tai kun joku hakee. Käytöstä poisto poistaa mallin mutta säilyttää opitun seuraavaa kertaa varten.
settings-clip-hint-desktop = Käyttää Googlen SigLIP 2 -mallia, jota ajetaan tämän tietokoneen suorittimella: mitään ei lähde tietokoneelta. Käyttöönotto lataa mallin ({ $size } Mt) Hugging Facesta ja opettelee sitten jokaisen kuvan taustalla matalalla prioriteetilla. Se vie 350–750 Mt muistia työskennellessään tai kun haet. Käytöstä poisto poistaa mallin mutta säilyttää opitun seuraavaa kertaa varten.
settings-clip-looking = Etsitään mallia…
settings-clip-downloading = Ladataan mallia… edistyminen näkyy toimintailmaisimessa
settings-clip-ready-empty = Valmis. Kuvat opitaan, kun paketteja lisätään.
settings-clip-ready-all =
    { $count ->
        [one] Valmis: tuntee { $count } kuvan
       *[other] Valmis: tuntee kaikki { $count } kuvaa
    }
settings-clip-learning = Valmis: opetellaan kuvia, { $done }/{ $total } tähän mennessä
settings-clip-relearning = Kunnes uusi malli on oppinut jokaisen kuvan, haut käyttävät sitä, mitä vanha oppi.
settings-clip-unavailable = Mallia ei saatu: { $detail }. Yritetään uudelleen tunnin välein.
settings-clip-where = Toimii
settings-clip-here = Tällä palvelimella: mitään ei lähde täältä
settings-clip-here-desktop = Tällä tietokoneella: mitään ei lähde täältä
settings-clip-hosted = Isännöidyssä palvelussa
settings-clip-hosted-warning = Kuvat lähtevät tältä palvelimelta. Pienoiskuva jokaisesta kirjaston kuvasta ja jokaisen kuvahaun sanat lähetetään palveluun { $service }, joka laskuttaa niistä ja säilyttää niitä ehtojensa mukaan.
settings-clip-hosted-warning-desktop = Kuvat lähtevät tältä tietokoneelta. Pienoiskuva jokaisesta kirjaston kuvasta ja jokaisen kuvahaun sanat lähetetään palveluun { $service }, joka laskuttaa niistä ja säilyttää niitä ehtojensa mukaan.
settings-clip-hosted-hint = Mitään ei ladata eikä ajeta täällä: palvelu oppii jokaisen kuvan erä kerrallaan. Toinen palvelu tai malli oppii ne kaikki uudelleen, ja se, mitä kukin on oppinut, säilytetään takaisin vaihtamista varten.
settings-clip-saved-here = Kuvahaku: toimii tällä palvelimella
settings-clip-saved-here-desktop = Kuvahaku: toimii tällä tietokoneella
settings-clip-saved-hosted = Kuvahaku: toimii palvelussa { $service }
settings-clip-hosted-unavailable = Palvelua { $service } ei voitu käyttää: { $detail }. Yritetään uudelleen tunnin välein.
settings-clap-heading = Hae ääniä sen mukaan, miltä ne kuulostavat
settings-clap-reset = Äänihaku: palauta oletus
settings-clap-saved-on = Äänihaku: päällä
settings-clap-saved-off = Äänihaku: pois
settings-clap-what = Kirjoita, mitä etsit (”distant thunder”, ”glassy UI blip”, ”footsteps on gravel”), niin siltä kuulostavat äänet näkyvät sanat sisältävien nimien rinnalla.
settings-clap-toggle = Ota äänihaku käyttöön
settings-clap-hint = Käyttää LAIONin CLAP-mallia, jota ajetaan tämän palvelimen suorittimella: mitään ei lähde palvelimelta. Käyttöönotto lataa mallin ({ $size } Mt) Hugging Facesta ja kuuntelee sitten jokaisen äänen alun taustalla matalalla prioriteetilla. Se vie enintään noin 1 Gt muistia työskennellessään tai kun joku hakee. Käytöstä poisto poistaa mallin mutta säilyttää opitun seuraavaa kertaa varten.
settings-clap-hint-desktop = Käyttää LAIONin CLAP-mallia, jota ajetaan tämän tietokoneen suorittimella: mitään ei lähde tietokoneelta. Käyttöönotto lataa mallin ({ $size } Mt) Hugging Facesta ja kuuntelee sitten jokaisen äänen alun taustalla matalalla prioriteetilla. Se vie enintään noin 1 Gt muistia työskennellessään tai kun haet. Käytöstä poisto poistaa mallin mutta säilyttää opitun seuraavaa kertaa varten.
settings-clap-ready-empty = Valmis. Äänet opitaan, kun paketteja lisätään.
settings-clap-ready-all =
    { $count ->
        [one] Valmis: tuntee { $count } äänen
       *[other] Valmis: tuntee kaikki { $count } ääntä
    }
settings-clap-learning = Valmis: opetellaan ääniä, { $done }/{ $total } tähän mennessä

## Asetukset: Tekoälytunnisteet

settings-ai-service = Palvelu
settings-ai-off = Pois
settings-ai-off-hint = Paketteihin lisätään tunnisteita vain sisäänrakennetuilla säännöillä.
settings-ai-local = Paikallinen malli
settings-ai-local-hint = llm-palvelu Hoardin compose-tiedostossa (tai cargo xtask llm). Lataa malli alta; se toimii tällä koneella, eikä mitään lähde siltä.
settings-ai-chatgpt = ChatGPT-tilaus
settings-ai-chatgpt-hint = Käyttää ChatGPT Go-, Plus-, Pro- tai Business-tilaustasi API-avaimen sijaan.
settings-ai-openrouter-hint = Yksi avain satoihin palvelussa ajettaviin malleihin, joista osa on ilmaisia. Kuvia vastaanottavat mallit on merkitty.
settings-ai-openai-hint = OpenAI:n ylläpitämä.
settings-ai-anthropic-hint = Anthropicin ylläpitämä, sen OpenAI-yhteensopivan rajapinnan kautta.
settings-ai-gemini-hint = Googlen ylläpitämä, sen OpenAI-yhteensopivan rajapinnan kautta.
settings-ai-ollama-hint = Itse ajamasi Ollama-palvelin. Vaihda osoite, jos se ei ole ollama-niminen kontti.
settings-ai-custom = Muu (OpenAI-yhteensopiva)
settings-ai-custom-hint = Mikä tahansa, joka puhuu OpenAI:n chat-rajapintaa: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Yhdistetään…
settings-ai-connected =
    { $count ->
        [one] Yhdistetty · { $count } malli
       *[other] Yhdistetty · { $count } mallia
    }
settings-ai-connected-none = Yhdistetty, mutta se ei luettele malleja; kirjoita mallin nimi
settings-chatgpt-connected-none = Yhdistetty; kirjoita mallin nimi
settings-ai-takes-images = ottaa kuvia
settings-ai-saved = Tekoälytunnisteet: { $service }
settings-ai-saved-off = Tekoälytunnisteet ovat pois päältä
settings-ai-reset = Tekoälytunnisteet: palauta oletus
settings-ai-on = Päällä: { $service } · { $model }
settings-ai-url = Osoite
settings-ai-key = API-avain
settings-ai-key-saved = Tallennettu (kirjoita korvataksesi)
settings-ai-key-paste = Liitä avaimesi
settings-ai-key-optional = Ei mitään (valinnainen)
settings-ai-key-remove = Poista
settings-ai-get-key = Hanki avain
settings-ai-model = Malli
settings-ai-model-pick = Valitse tai kirjoita malli
settings-ai-model-name = Mallin nimi
settings-ai-refresh = Päivitä
settings-ai-connect = Yhdistä
settings-ai-need-key = Lisää API-avain nähdäksesi sen mallit
settings-ai-vision = Lähetä myös pikkukuvat (vaatii kuvia vastaanottavan mallin)
settings-ai-batch = Pyyntöä kohden
settings-ai-batch-unit = resurssia (vähemmän pienille paikallisille malleille, enemmän suurille palvelumalleille)

## Asetukset: ChatGPT-kirjautuminen

settings-chatgpt-account = Tili
settings-chatgpt-checking = Tarkistetaan…
settings-chatgpt-status-as = Kirjautunut käyttäjänä { $email }
settings-chatgpt-status = Kirjautunut
settings-chatgpt-signed-in-as = Kirjauduttu ChatGPT:hen käyttäjänä { $email }
settings-chatgpt-signed-in = Kirjauduttu ChatGPT:hen
settings-chatgpt-signed-out = Kirjauduttu ulos ChatGPT:stä
settings-chatgpt-continue = Jatka ChatGPT:llä
settings-chatgpt-sign-out = Kirjaudu ulos
settings-chatgpt-sign-out-title = Kirjaudutaanko ulos ChatGPT:stä?
settings-chatgpt-sign-out-in-use = Tekoälytunnisteet käyttävät ChatGPT-tilaustasi, joten ne pysähtyvät, kunnes kirjaudut uudelleen tai valitset toisen palvelun.
settings-chatgpt-sign-out-body = Hoard unohtaa kirjautumisen ja pyytää OpenAI:ta päättämään sen.
settings-chatgpt-manage = Hallitse käyttöä
settings-chatgpt-finish-before = Viimeistele kirjautuminen avautuneessa ikkunassa (
settings-chatgpt-finish-link = avaa se uudelleen
settings-chatgpt-finish-middle = ). Jos se päättyy sivulle, joka ei lataudu, osoitteessa, joka alkaa{ " " }
settings-chatgpt-finish-after = , kopioi osoite ja liitä se tähän:
settings-chatgpt-paste = Osoite, johon kirjautuminen päättyi
settings-chatgpt-finish = Viimeistele
settings-chatgpt-usage = Tunnistepyynnöt lasketaan ChatGPT-tilauksesi käyttöön. Hallitse käyttöä näyttää, mitä Hoard on käyttänyt, ja antaa rajoittaa sitä. Katkaistaksesi Hoardin yhteyden itse ChatGPT:stä siirry chatgpt.comissa kohtaan Settings → Security and login → Sign in with ChatGPT.

## Asetukset: Paikalliset mallit

settings-local-downloading-progress = Ladataan { $percent } % · { $done } / { $total }
settings-local-downloading = Ladataan…
settings-local-incomplete = Lataus pysähtyi
settings-local-failed = Käynnistys epäonnistui viime kerralla
settings-local-in-use-running = Käytössä · käynnissä
settings-local-in-use = Käytössä
settings-local-starting = Käynnistyy…
settings-local-running = Käynnissä
settings-local-ready-images = Valmis · ottaa kuvia
settings-local-ready-text = Valmis · vain teksti
settings-local-delete-title = Poistetaanko { $name }?
settings-local-delete-in-use = Tekoälytunnisteet käyttävät tätä mallia, joten ne pysähtyvät, kunnes valitset toisen. Sen tiedostot poistetaan mallipalvelimelta.
settings-local-delete-downloading = Lataus pysähtyy, ja tähän mennessä ladattu poistetaan.
settings-local-delete-body = Sen tiedostot poistetaan mallipalvelimelta. Voit ladata sen myöhemmin uudelleen.
settings-local-delete = Poista
settings-local-deleting = Poistetaan…
settings-local-cancel = Peruuta
settings-local-resume = Jatka
settings-local-download = Lataa
settings-local-start-before = Käynnistä se komennolla{ " " }
settings-local-start-or = { " " }(tai{ " " }
settings-local-start-developing = { " " }kehityksen aikana) ja{ " " }
settings-local-start-retry = yritä uudelleen
settings-local-start-after = . Myös palvelin, joka ajaa vain yhtä mallia, käy: anna sen mallin nimi alla.
settings-local-list = Mallipalvelimella
settings-local-none = Mitään ei ole vielä ladattu. Valitse jokin alta; se säilytetään palvelimella, ja voit vaihtaa milloin tahansa.
settings-local-catalog = Ladattavat mallit
settings-local-size = { $size } Gt · { $license }
settings-local-other = Toinen malli Hugging Facesta
settings-local-other-placeholder = Toinen Hugging Facesta: owner/name-GGUF:Q4_K_M

## Asetukset: Pakettien lisäys

settings-store-pages = Kauppasivut
settings-store-pages-what = Uudet paketit haetaan kaupoista taustalla, jotta ne saavat kauppasivunsa, myyjänsä ja lisenssinsä.
settings-store-lookup = Hae kauppasivut automaattisesti
settings-unreal = Unreal-paketit
settings-unreal-reset = Unreal-paketit: palauta oletus
settings-unreal-what = Mitä .uasset-tiedostoille tehdään.
settings-conv = Muunnossäännöt
settings-conv-what = Muunna Hoardin säilyttämät tiedostot yhteen muotoon, esimerkiksi kaikki OBJ- ja FBX-tiedostot GLB-muotoon, jotta kirjasto on yhtenäinen. Muunnettu tiedosto korvaa alkuperäisen ja säilyttää sen nimen, tunnisteet, kokoelmat ja linkit. Valvottujen kansioiden tiedostoja ei koskaan muuteta.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Käytössä
settings-conv-to = Muunna muotoon
# Heading over the file formats a rule converts.
settings-conv-from = Lähtömuodot
settings-conv-remove = Poista
settings-conv-remove-label = Poista sääntö { $n }
settings-conv-taken = Toinen sääntö muuntaa jo tämän muodon
settings-conv-upright = Käännä mallit pystyyn muunnettaessa (Y ylös, etupuoli kohti +Z) samalla tavalla kuin paketin mallit näytetään
settings-conv-flac-what = Ääni pysyy täsmälleen samana noin puolessa tilasta, ja kaikki muu WAV-tiedoston sisältö (silmukkapisteet, cue-merkit, tunnisteet) säilyy FLAC-tiedostossa, joten WAV voidaan palauttaa milloin tahansa täsmälleen sellaisena kuin se oli. WAV-tiedostot, joissa on liukuluku- tai 32-bittisiä näytteitä, jäävät WAV-muotoon.
settings-conv-add = Lisää sääntö
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Säilytä alkuperäiset
settings-conv-keep-days =
    { $count ->
        [one] päivän
       *[other] päivää
    }
settings-conv-keep-what = Siihen asti muunnetun tiedoston sivulta voi palauttaa alkuperäisen. Tekstuureja ei kopioida muunnettuun malliin: se viittaa vieressään oleviin kuvatiedostoihin. Mallin .mtl- tai .bin-tiedostot jäävät paikalleen.
settings-conv-save = Tallenna säännöt
settings-conv-revert = Kumoa muutokset
settings-conv-existing = Muunna kirjastossa jo olevat tiedostot
settings-conv-saved = Muunnossäännöt tallennettu. Niitä sovelletaan tästä lähtien lisättäviin paketteihin.
settings-conv-converting =
    { $count ->
        [one] Muunnetaan yhtä tiedostoa taustalla
       *[other] Muunnetaan { $count } tiedostoa taustalla
    }
settings-conv-nothing = Kirjastossa ei ole mitään muunnettavaa.
settings-conv-ask-title =
    { $count ->
        [one] Muunnetaanko yksi kirjastossa jo oleva tiedosto?
       *[other] Muunnetaanko { $count } kirjastossa jo olevaa tiedostoa?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Se on yhdessä paketissa.
       *[other] Ne ovat { $packs } paketissa.
    } Jokainen alkuperäinen säilytetään { $days } päivää, ja siihen asti sen voi palauttaa sen sivulta.
settings-conv-ask-busy =
    { $count ->
        [one] Yksi on paketissa, jota vielä lisätään; se muunnetaan, kun lisäys on valmis.
       *[other] { $count } on paketeissa, joita vielä lisätään; ne muunnetaan, kun lisäys on valmis.
    }
settings-conv-ask-list = Tiedostot, jotka muunnettaisiin
settings-conv-ask-more = …ja { $count } lisää
settings-conv-ask-skipped = Jätetään ennalleen:
settings-conv-skip-watched =
    { $count ->
        [one] yksi tiedosto valvotussa kansiossa
       *[other] { $count } tiedostoa valvotuissa kansioissa
    }
settings-conv-skip-unity =
    { $count ->
        [one] yksi tiedosto, jonka Unity-projekti tuntee nimeltä
       *[other] { $count } tiedostoa, jotka Unity-projekti tuntee nimeltä
    }
settings-conv-skip-used =
    { $count ->
        [one] yksi tiedosto, jonka kohtaus tai toinen tiedosto mainitsee nimeltä
       *[other] { $count } tiedostoa, jotka kohtaus tai toinen tiedosto mainitsee nimeltä
    }
settings-conv-skip-beside =
    { $count ->
        [one] yksi tiedosto, jonka muunnettu kopio on jo sen paketissa
       *[other] { $count } tiedostoa, joiden muunnetut kopiot ovat jo niiden paketeissa
    }
settings-conv-skip-blender =
    { $count ->
        [one] yksi tiedosto, jota Blender ei ole vielä lukenut (Asetukset → Työkalut)
       *[other] { $count } tiedostoa, joita Blender ei ole vielä lukenut (Asetukset → Työkalut)
    }
settings-conv-ask-new = Tallenna, vain uudet paketit
settings-conv-ask-save-convert =
    { $count ->
        [one] Tallenna ja muunna se
       *[other] Tallenna ja muunna { $count } tiedostoa
    }
settings-conv-ask-convert =
    { $count ->
        [one] Muunna se
       *[other] Muunna { $count } tiedostoa
    }
settings-unreal-off = Jätä ennalleen
settings-unreal-off-hint = Muunna paketin sivulta, kun haluat.
settings-unreal-keep = Muunna, säilytä alkuperäiset
settings-unreal-keep-hint = Tekstuurit, äänet ja materiaalit tulevat ulos yleisissä muodoissa; myös .uasset säilyy.
settings-unreal-replace = Muunna, hylkää alkuperäiset
settings-unreal-replace-hint = Säästää tilaa, kun sisältö on purettu. Verkot odottavat yhä Unreal-työntekijää.

## Asetukset: Lataukset

settings-tn = Tekstuurien nimet
settings-tn-saved = Tekstuurien nimet tallennettu
settings-tn-reset = Tekstuurien nimet: palauta oletus
settings-tn-what = Materiaalin kartat lähtevät nimettyinä materiaalin ja kunkin kartan käyttötarkoituksen mukaan, nimettiinpä tiedostot miten tahansa: Rock_Normal.png, ei rock_moss_xeokah0_raw_8K_Normal.png. Kirjaston tiedostoihin ei kosketa, ja kartta, jonka mukana lähetetty malli tai MTL nimeää, säilyttää nimensä, jotta malli löytää sen yhä.
settings-tn-toggle = Nimeä tekstuurikartat materiaalinsa mukaan
settings-tn-style = Tyyli
settings-tn-style-group = Nimeämistyyli
settings-tn-descriptive = Kuvaava
settings-tn-short = Lyhyt
settings-tn-custom = Oma
settings-tn-pattern = Malli
settings-tn-hint-name = { " " }materiaali,{ " " }
settings-tn-hint-map = { " " }kartan käyttötarkoitus,{ " " }
settings-tn-hint-size = { " " }sen koko, kun sarjasta on useita kokoja (lisätään loppuun, jos malli jättää sen pois).
settings-tn-example = Esimerkki
settings-tn-suffixes = Päätteet kullekin karttatyypille
settings-tn-slots = GLB-latauksissa mallin sisään pakattu tekstuuri, jolla ei ole omaa tiedostoa, nimetään materiaalin ja sen täyttämän paikan mukaan, ja näilläkin paikoilla on päätteet:
settings-tn-endings = Omat loppuosat
settings-tn-endings-what = Tekstuuri, joka ei ole mikään materiaalin kartoista, säilyttää oman nimensä, ellei se pääty johonkin näistä: _diff → _BC lähettää tiedoston rock_diff.png nimellä rock_BC.png. Jätä jälkimmäinen tyhjäksi, niin loppuosa poistetaan.
settings-tn-ending-from = Loppuosa
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Tilalle
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Lisää loppuosa
settings-tn-ending-remove = Poista
settings-tn-ending-remove-label = Poista { $ending }

## Asetukset: Yleiset ja Yhteys

settings-name = Nimi
settings-name-what = Mikä tämän palvelimen nimi on yläpalkissa, selaimen välilehdellä ja kirjautumissivulla, jotta useamman Hoardin kanssa tiedät, mikä on mikä.
settings-name-what-desktop = Mikä tämän kirjaston nimi on yläpalkissa.
settings-address = Osoite
settings-address-what = Osoite, jolla ihmiset ja sovellukset tavoittavat Hoardin, kun se ei ole sama kuin tämän selaimen käyttämä: käänteisen välityspalvelimen, tunnelin tai tailnet-nimen osoite, esim. https://hoard.example.com. Hoardin jakamat linkit (Claudelle ja muille sovelluksille, kirjautumisessa, sen varmenteessa) käyttävät sitä.
settings-public-url = Julkinen osoite
settings-public-url-placeholder = Selaimen osoitteesta (esim. https://hoard.example.com)
settings-public-url-default = selaimen osoitteesta

## Asetukset: Suorituskyky, profiloija sivun yläosassa

profiler = Profiloija
profiler-what = Mitä Hoard on tehnyt viimeisten kymmenen minuutin aikana, päivittyen kahden sekunnin välein, kun katsot. Tiedot pidetään vain muistissa ja alkavat alusta, kun Hoard käynnistyy uudelleen.
profiler-starting = Ensimmäiset lukemat tulevat hetken kuluttua.
profiler-failed = Profiloijan luku epäonnistui: { $error }
profiler-cpu = Suorittimen käyttö
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Koko kone
profiler-cpu-summary = Hoard käyttää { $hoard } suorittimista ja koko kone { $machine }.
profiler-memory = Muisti
profiler-serving = Käynnissä olevat pyynnöt
profiler-working = Työn alla
profiler-working-none = Taustalla ei ajettu mitään.
profiler-working-more =
    { $count ->
        [one] Ja yksi muu.
       *[other] Ja { $count } muuta.
    }
profiler-requests = Pyyntöjen käsittelyaika
profiler-requests-what = Jokainen palkki on kymmenen sekunnin pyynnöt, ja niiden aika on jaettu sen mukaan, mihin se kului.
profiler-part-db = Tietokanta
profiler-part-db-title = Tietokannan odottaminen ja sen kyselyjen suorittaminen
profiler-part-disk = Levy
profiler-part-disk-title = Tallennettujen tiedostojen lukeminen ja lähetettävän pakkaaminen
profiler-part-send = Lähetys
profiler-part-send-title = Odottaminen, kun verkko ottaa vastaan lähetettyä
profiler-part-other = Muu
profiler-part-other-title = Vastauksen laatiminen
profiler-request-count =
    { $count ->
        [one] { $count } pyyntö
       *[other] { $count } pyyntöä
    }
profiler-requests-none = Ei pyyntöjä viimeisten kymmenen minuutin aikana.
profiler-slowest = Hitain: { $route }, { $time }
profiler-routes = Reiteittäin
profiler-route = Reitti
profiler-count = Pyynnöt
profiler-average = Keskiarvo
profiler-p95 = Hitain 5 %
profiler-split = Mihin aika kului
profiler-sent = Lähetetty
profiler-portal = käyttöliittymän omat tiedostot

## Asetukset: Suorituskyky

settings-render = 3D-pikkukuvat
settings-render-toggle = Renderöi pikkukuvat 3D-malleille
settings-gpu-on = Näytönohjaimella: { $renderer }
settings-gpu-software = Näytönohjainta ei löytynyt, joten ohjelmallisesti ({ $renderer })
settings-gpu-none = Näytönohjainta tai ohjelmallista renderöijää ei löytynyt, joten mallit eivät saa pikkukuvia
settings-thumbs = Pikkukuvat
settings-thumbs-what = Miten uudet pikkukuvat ja mallinäkymät tallennetaan. Läpinäkyvät kuvat ja pikseligrafiikka ovat aina PNG-muodossa. Pienentääksesi jo olemassa olevat aja Ylläpito-sivulla Pienennä pikkukuvat.
settings-previews-size = Esikatselut vievät nyt { $size }.
settings-thumbs-saved = Pikkukuvat: { $quality }
settings-thumbs-small = Pienemmät
settings-thumbs-small-hint = JPEG, laatu 70. Riittää ruudukossa; hieman pehmeä läheltä.
settings-thumbs-balanced = Tasapainoinen
settings-thumbs-balanced-hint = JPEG, laatu 82. Vaikea erottaa alkuperäisestä pikkukuvan koossa.
settings-thumbs-sharp = Terävämmät
settings-thumbs-sharp-hint = JPEG, laatu 92. Noin kaksi kertaa Tasapainoisen kokoinen.
settings-thumbs-lossless = Häviötön
settings-thumbs-lossless-hint = PNG, täsmälleen tehtynä. Monta kertaa suurempi.
settings-thumbs-avif = Lähetä pikkukuvat AVIF-muodossa
settings-thumbs-avif-what = AVIF-kuvia näyttävät selaimet saavat pienempiä pikkukuvia, usein alle puolet koosta. Kukin tehdään tallennetusta pikkukuvasta, kun se näytetään ensimmäisen kerran, ja säilytetään muunnosten kanssa; siihen asti lähetetään tallennettu. Niiden tekeminen pitää enintään kaksi ydintä kiireisenä, kun suurta kirjastoa selataan ensimmäistä kertaa. Häviöttömät pikkukuvat ja pikselitaide lähetetään sellaisinaan.
settings-motion-thumbs = Liikkuvat esikatselukuvat animaatioille
settings-motion-thumbs-what = Jokainen animaatio saa myös lyhyen silmukkana toistuvan videon, joka pyörii, kun osoitat sen ruutua. Kukin vie noin 40 kt, ja suuressa kirjastossa niiden tekeminen kestää hetken. Kun poistat tämän käytöstä, ne poistetaan.
settings-limits = Työ ja rajat
settings-limits-what = Kukin tallentuu, kun poistut kentästä tai painat Enter. Koot ottavat yksiköt: 100 GB, 512 MB, 1.5 TB; pelkkä luku tarkoittaa gigatavuja.
settings-workers = Taustatyöntekijät
settings-workers-unit = työtä kerrallaan (tuonti, tunnisteet, muunnokset)
settings-max-upload = Suurin lähetys
settings-max-upload-unit = tiedostoa kohden
settings-max-expand = Suurin paketti
settings-max-expand-unit = purettuna
settings-conversion-cache = Muunnetut lataukset
settings-conversion-cache-unit-now = säilytetään enintään, nyt { $size }; vähiten käytetyt poistetaan ensin
settings-conversion-cache-unit = säilytetään enintään; vähiten käytetyt poistetaan ensin

## Asetukset: Ylläpito

settings-updates = Päivitykset
settings-version = Tämä on Hoard { $version }.
settings-update-check = Tarkista GitHubista muutaman tunnin välein, onko uutta versiota
settings-desktop-updates = Hoard etsii uutta versiota aina käynnistyessään, lataa sen taustalla ja asentaa sen, kun suljet sovelluksen.
settings-backups = Ajastetut varmuuskopiot
settings-backups-what = Tietokanta varmuuskopioi itse itsensä; jokainen varmuuskopio näkyy alla historiassa.
settings-backup-every = Varmuuskopioi
settings-backup-off = Ei koskaan
settings-backup-daily = Joka päivä
settings-backup-weekly = Joka viikko
settings-backup-keep = Säilytä
settings-backup-keep-unit = uusinta ajastettua varmuuskopiota
settings-backup-copy-to = Kopioi myös kansioon
settings-backup-copy-to-none = Ei muuta kansiota
settings-backups-kept = Jokainen tallentuu varmuuskopiokansioon nimellä hoard-auto-…db ja toiseen kansioon (toinen levy tai kansio, jonka synkronointisovellus varmuuskopioi), jos sellainen on annettu koko polkuna palvelimella. Vain vanhemmat ajastetut varmuuskopiot poistetaan; käsin tehdyt jäävät.
about-what = Hoard säilyttää pelien assetit (mallit, tekstuurit, äänet, musiikin, spritet ja muuta) yhdessä kirjastossa, jota voit hakea, esikatsella ja josta voit ottaa, jaettuna kaikkien kanssa, joille annat pääsyn.
about-version = Versio
about-edition = Laitos
about-edition-server = Palvelin
about-edition-desktop = Työpöytäsovellus
about-edition-steam = Työpöytäsovellus Steamista
about-made-by = Tekijä
about-source = Lähdekoodi GitHubissa
about-issues = Ilmoita ongelmasta
about-discord = Juttele kanssamme Discordissa
about-steam = Hoard Steamissa
about-license = Lisenssi
about-license-what = Hoard on avointa lähdekoodia MIT No Attribution -lisenssillä: käytä, muuta ja jaa sitä miten haluat.
about-downloaded = Ladataan tarvittaessa
about-downloaded-what = Nämä eivät tule Hoardin mukana. Se lataa kunkin, kun jokin tarvitsee sitä ensimmäisen kerran, ja kullakin on oma lisenssinsä.
about-blender = Muuntaa 3D-malleja ja tekee niiden pikkukuvat.
about-ffmpeg = Muuntaa ääntä ja videota ja tekee niiden esikatselut.
about-siglip = Löytää kuvia sen perusteella, mitä niissä on.
about-clap = Löytää ääniä sen perusteella, miltä ne kuulostavat.
about-models = Paikalliselle tekoälypalvelimelle valitsemiisi malleihin pätevät niiden tekijöiden lisenssit.
about-built-with = Tehty käyttäen
about-built-with-what = Avoimen lähdekoodin kirjastot, joista Hoard on tehty, lisensseineen.
about-steamworks = Steam-versio sisältää myös Steamworks SDK:n, © Valve Corporation.
about-licenses-loading = Ladataan luetteloa…
about-licenses-failed = Luetteloa ei voitu ladata.
about-filter = Suodata nimen tai lisenssin mukaan
about-count = { $shown } / { $total }
about-no-license = Lisenssiä ei ilmoitettu
about-report = Ilmoita ongelmasta…
report-title = Ilmoita ongelmasta
report-what = Tämä on se, mitä GitHubin issue tarvitsee, jotta ongelmaa voidaan selvittää. Siinä ei ole mitään yksityistä: käyttötunnukset, salasanat, avaimet, sähköpostiosoitteet ja nimesi kansiopoluissa poistetaan. Lue se läpi ennen lähettämistä.
report-loading = Kootaan raporttia…
report-failed = Raporttia ei voitu koota: { $error }
report-log = Mitä palvelin on viime aikoina kirjannut
report-copy = Kopioi
report-copied = Raportti kopioitu.
report-github = Avaa issue GitHubissa
report-github-hint = Uusi issue täytetään tällä. Jos loki on siihen liian pitkä, mukaan tulevat vain sen viimeiset rivit; Kopioi sisältää kaiken.
report-close = Sulje

## Resurssityypit

api-kind-texture = Tekstuurit
api-kind-vector = Vektorigrafiikka
api-kind-sheet = Sprite-arkit
api-kind-audio = Ääni
api-kind-model = Mallit
api-kind-animation = Animaatiot
api-kind-font = Fontit
api-kind-shader = Varjostimet
api-kind-material = Materiaalit
api-kind-video = Video
api-kind-document = Dokumentit
api-kind-data = Data
api-kind-other = Muut
api-kind-texture-name = tekstuurit
api-kind-vector-name = vektorigrafiikka
api-kind-sheet-name = sprite-arkit
api-kind-audio-name = ääni
api-kind-model-name = mallit
api-kind-animation-name = animaatiot
api-kind-font-name = fontit
api-kind-shader-name = varjostimet
api-kind-material-name = materiaalit
api-kind-video-name = video
api-kind-document-name = dokumentit
api-kind-data-name = data
api-kind-other-name = muut

## Ilmoitukset, valintaikkunat ja lataus

ui-undo = Kumoa
ui-today = tänään
ui-yesterday = eilen
ui-days-ago =
    { $days ->
        [one] { $days } päivä sitten
       *[other] { $days } päivää sitten
    }
ui-notifications = Ilmoitukset
ui-dismiss = Kuittaa
ui-cancel = Peruuta
ui-lazy-loading = Ladataan: { $what }…
ui-lazy-failed = Ei voitu ladata: { $what }.
ui-lazy-reload = Lataa sivu uudelleen

## Palautuspainike

reset-title = Palauta oletus: { $value }
reset-label = Palauta oletus, { $value }

## Kokokenttä

size-invalid = Koko, kuten 100 GB, 512 MB tai 1.5 TB
size-hint = Koko, kuten 100 GB, 512 MB tai 1.5 TB; pelkkä luku tarkoittaa gigatavuja

## Todennus

auth-load-failed = Todennusasetusten lataus epäonnistui: { $error }
auth-saved = Tallennettu. Koskee seuraavaa kirjautumista.
auth-method-internal = Sisäinen
auth-method-internal-hint = Hoardin säilyttämät nimet ja salasanat, lisätään kohdassa Asetukset → Tili.
auth-method-ldap-hint = Hakemisto tarkistaa kirjautumissivulla kirjoitetun nimen ja salasanan: OpenLDAP, FreeIPA, LLDAP tai muu LDAP-palvelin.
auth-method-ad-hint = Windows-toimialueen tilit kirjautuvat tavallisella nimellään ja salasanallaan.
auth-method-oauth-hint = ”Kirjaudu…”-painike, joka vie Googleen, Microsoftille, GitHubiin, Keycloakiin, Authentikiin tai muulle palveluntarjoajalle ja takaisin.
auth-session-unused = Kun käyttämättä 30 päivää
auth-session-1h = Tunti kirjautumisen jälkeen
auth-session-8h = 8 tuntia kirjautumisen jälkeen
auth-session-day = Vuorokausi kirjautumisen jälkeen
auth-session-week = Viikko kirjautumisen jälkeen
auth-session-30d = 30 päivää kirjautumisen jälkeen
auth-session-hours =
    { $hours ->
        [one] { $hours } tunti kirjautumisen jälkeen
       *[other] { $hours } tuntia kirjautumisen jälkeen
    }
auth-role-view = Voi katsoa
auth-role-edit = Voi muuttaa
auth-role-admin = Ylläpitäjä
auth-role-word-view = voi katsoa
auth-role-word-edit = voi muuttaa
auth-role-word-admin = ylläpitäjä
auth-role-word-default = { $role } (oletusrooli)
auth-tls-starttls-hint = Tavallinen LDAP (portti 389), joka vaihtaa TLS:ään ennen kuin mitään lähetetään.
auth-tls-ldaps-hint = TLS alusta asti (portti 636).
auth-tls-none = Ei mitään
auth-tls-none-hint = Salasanat kulkevat verkossa sellaisinaan. Vain verkossa, johon luotat.
auth-test-failed = Testaus epäonnistui: { $error }
auth-preset-custom = Muu (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel ja muut: anna myöntäjä, niin Etsi täyttää osoitteet.
auth-preset-google-note = Tee OAuth-asiakas (Web application) kohdassa Google Cloud Console → APIs & Services → Credentials. Google ei lähetä ryhmiä, joten anna roolit kohdassa Asetukset → Tili ja poista tilien luonti ensimmäisellä kirjautumisella käytöstä, ellei kenen tahansa Google-tilin omistajan pidä päästä sisään.
auth-preset-microsoft-note = Rekisteröi sovellus kohdassa Entra ID → App registrations (Web-uudelleenohjaus alla olevaan osoitteeseen). Laita vuokraajasi tunnus (tenant ID) ”organizations”-sanan tilalle, jotta vain organisaatiosi pääsee sisään. Saadaksesi roolit ryhmistä lisää groups-vaatimus kohdassa Token configuration: se luettelee ryhmien tunnukset.
auth-preset-github-note = Tee OAuth-sovellus kohdassa GitHub → Settings → Developer settings. GitHub ei lähetä ryhmiä, joten anna roolit kohdassa Asetukset → Tili ja poista tilien luonti ensimmäisellä kirjautumisella käytöstä, ellei kenen tahansa GitHub-tilin omistajan pidä päästä sisään.
auth-preset-gitlab-note = Tee sovellus kohdassa GitLab → Preferences → Applications (tai instanssisi ylläpitoalueella: vaihda gitlab.com osoitteissa).
auth-discovered = Palveluntarjoajan osoitteet täytetty.
auth-copied = Kopioitu
auth-copy-failed = Kopiointi epäonnistui: valitse ja kopioi itse
auth-sign-in-with = Kirjautumistapa
auth-own-accounts-still-work = Hoardin omat tilit kirjautuvat yhä omilla salasanoillaan, joten alhaalla oleva tai väärin määritetty hakemisto tai palveluntarjoaja ei voi lukita sinua ulos.
auth-sessions = Istunnot
auth-sign-out-after = Kirjaa käyttäjät ulos
auth-ldap-server = LDAP-palvelin
auth-server-type = Palvelimen tyyppi
auth-server-type-custom = Mukautettu
auth-servers = Palvelimet
auth-server-n = Palvelin { $n }
auth-remove-server-n = Poista palvelin { $n }
auth-remove = Poista
auth-add-server = Lisää palvelin
auth-servers-hint = Seuraavaa kokeillaan, kun yhteen ei saada yhteyttä.
auth-security = Suojaus
auth-connection-security = Yhteyden suojaus
auth-skip-verify = Älä tarkista palvelimen varmennetta (vain kokeiluun)
auth-ca = Varmentaja
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (tyhjä: järjestelmän luottamat)
auth-lookup = Käyttäjien haku
auth-anonymously = Nimettömänä
auth-reader-account = Lukijatili
auth-reader-password = Sen salasana
auth-saved-type-to-replace = Tallennettu: kirjoita korvataksesi
auth-people-under = Käyttäjät ovat kohdassa
auth-name-attribute = Nimen attribuutti
auth-only-matching = Vain merkinnät, jotka vastaavat
auth-groups-under = Ryhmät ovat kohdassa
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (tyhjä: vain memberOf)
auth-their-groups = Heidän ryhmänsä
auth-group-filter-hint = { "{" }dn{ "}" } on henkilön merkintä, { "{" }user{ "}" } nimi, jolla hän kirjautui.
auth-test = Testaa
auth-test-hint = Ottaa yhteyden jokaiseen palvelimeen ja kirjautuu lukijatilinä. Nimen ja salasanan kanssa kirjautuu myös tämän henkilönä ja näyttää, mitä Hoard siitä päättelisi. Mitään ei tallenneta.
auth-test-user = Kenenä kirjaudutaan
auth-test-user-placeholder = Kenenä kirjaudutaan (valinnainen)
auth-test-password = Hänen salasanansa
auth-testing = Testataan…
auth-test-connectivity = Testaa yhteys
auth-test-server-ok = tavoitettu, ja lukijatili kirjautui
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }kirjautui sisään (
auth-test-signed-in-after = ).
auth-test-role = Rooli olisi: { $role }.
auth-test-no-role = Hän ei ole yhdessäkään Roolit-kohdassa luetellussa ryhmässä, joten hänet käännytettäisiin pois.
auth-test-groups = Ryhmät: { $groups }
auth-test-no-groups = Ryhmiä ei löytynyt.
auth-provider = Palveluntarjoaja
auth-redirect = Uudelleenohjausosoite
auth-copy = Kopioi
auth-redirect-hint = Anna tämä palveluntarjoajalle, kun teet Hoardin asiakkaan. Se tulee julkisesta osoitteesta kohdassa Asetukset → Yhteys tai käyttämästäsi osoitteesta.
auth-issuer = Myöntäjä
auth-discover = Etsi
auth-client-id = Asiakastunnus
auth-client-secret = Asiakassalaisuus
auth-authorization-url = Valtuutusosoite
auth-token-url = Tunnusosoite
auth-userinfo-url = Käyttäjätieto-osoite
auth-userinfo-placeholder = (tyhjä: ID-tunnus kertoo, kuka se on)
auth-scopes = Laajuudet
auth-name-from = Nimi kentästä
auth-groups-from = Ryhmät kentästä
auth-groups-from-placeholder = (ei mitään)
auth-button = Painike
auth-button-placeholder = oma tilisi
auth-accounts = Tilit
auth-provision = Luo Hoard-tili, kun joku kirjautuu ensimmäisen kerran
auth-provision-on-hint = Muuten ylläpitäjä lisää jokaisen henkilön ensin kohdassa Asetukset → Tili.
auth-provision-off-hint = Ylläpitäjä lisää jokaisen henkilön ensin kohdassa Asetukset → Tili ja valitsee, miten tämä kirjautuu.
auth-open-to-anyone = Kuka tahansa, jolla on { $provider }-tili, pääsee sisään. Lisää katselijaryhmiä alle tai poista tämä käytöstä ja lisää käyttäjät itse.
auth-new-accounts = Uudet tilit
auth-roles-from-groups = Roolit ryhmistä
auth-roles-hint = Ryhmät, joiden jäsenet saavat kunkin roolin, yksi riviä kohden, nimellä tai DN:llä. Korkein vastaava voittaa, ja se tarkistetaan jokaisella kirjautumisella. Jos ryhmiä ei ole lueteltu, uudet tilit saavat yllä olevan roolin ja ylläpitäjä muuttaa sen myöhemmin. Jos Voi katsoa -kohdassa on lueteltu ryhmiä, vain lueteltujen ryhmien jäsenet voivat kirjautua.
auth-roles-hint-oauth = Ryhmät, joiden jäsenet saavat kunkin roolin, yksi riviä kohden, nimellä tai DN:llä (tai millä tahansa ryhmävaatimus niitä luettelee). Korkein vastaava voittaa, ja se tarkistetaan jokaisella kirjautumisella. Jos ryhmiä ei ole lueteltu, uudet tilit saavat yllä olevan roolin ja ylläpitäjä muuttaa sen myöhemmin. Jos Voi katsoa -kohdassa on lueteltu ryhmiä, vain lueteltujen ryhmien jäsenet voivat kirjautua.
auth-saving = Tallennetaan…
auth-save = Tallenna

## HTTPS

https-load-failed = HTTPS-asetusten lataus epäonnistui: { $error }
https-off-redirecting = HTTPS on pois päältä. Siirrytään osoitteeseen http://
https-now-on = HTTPS on päällä
https-saved = Tallennettu. Uudet yhteydet käyttävät uutta varmennetta.
https-now-off = HTTPS on pois päältä
https-http3-failed-toast = HTTP/3:n käynnistys epäonnistui: { $error }
https-copied = Kopioitu
https-copy-failed = Kopiointi epäonnistui: valitse ja kopioi itse
https-mode-off = Pois
https-mode-off-hint = Vain tavallinen http://.
https-mode-own = Hoardin oma varmenne
https-mode-own-hint = Hoard tekee varmenteen tämän palvelimen osoitteille. Asenna Hoardin varmentaja kerran jokaiselle laitteelle, niin sen selaimet luottavat siihen.
https-mode-custom = Oma varmenteeni
https-mode-custom-hint = Varmenne, joka sinulla jo on, esim. tailscale cert -komennosta tai Let’s Encryptilta. Selaimet luottavat siihen asentamatta mitään.
https-untrusted-own = Tämä selain ei vielä luota siihen osoitteessa { $host }: asenna Hoardin varmentaja alta. Näet varoituksen, jonka voit ohittaa.
https-untrusted-uncovered = Tämä selain ei vielä luota siihen osoitteessa { $host }: varmenne ei kata tätä osoitetta. Näet varoituksen, jonka voit ohittaa.
https-untrusted = Tämä selain ei vielä luota siihen osoitteessa { $host }. Näet varoituksen, jonka voit ohittaa.
https-now = Nyt
https-start-failed = HTTPS on määritetty, mutta sen käynnistys epäonnistui: { $error }
https-on-port = HTTPS on päällä portissa { $port } kuten http://.
https-using-it = Käytät sitä nyt.
https-open = Avaa { $url }
https-trusted = Tämä selain luottaa siihen.
https-cert-for-before = Varmenne osoitteille{ " " }
https-cert-for-after = , voimassa { $date } asti.
https-cert-for-after-own = , voimassa { $date } asti (uusiutuu itsestään).
https-not-covered-before = Se ei kata osoitetta{ " " }
https-not-covered-after = , jota käytät.
https-is-off = HTTPS on pois päältä: sivut tarjoillaan tavallisella http://-yhteydellä.
https-addresses = Osoitteet
https-addresses-hint = Varmenne kattaa Hoardin löytämät osoitteet ja lisäämäsi osoitteet (reitittimen sille antama nimi, Tailscale-nimi).
https-remove-name = Poista { $name }
https-another-address = Toinen osoite
https-another-address-placeholder = Toinen osoite, esim. nas.local
https-add = Lisää
https-your-cert = Oma varmenteesi
https-saved-cert-before = Tallennettu:{ " " }
https-saved-cert-after = { " " }– myöntäjä { $issuer }, voimassa { $date } asti. Liitä uusi korvataksesi sen.
https-pem-before = Varmenne (ketjuineen) ja sen yksityinen avain PEM-tekstinä: tiedostot, jotka alkavat{ " " }
https-pem-after = .
https-certificate = Varmenne
https-choose-file = Valitse tiedosto…
https-private-key = Yksityinen avain
https-checked-before = Osoitteille{ " " }
https-checked-after = , myöntäjä { $issuer }, voimassa { $date } asti.
https-checked-after-days =
    , myöntäjä { $issuer }, voimassa { $date } asti ({ $days ->
        [one] { $days } päivä
       *[other] { $days } päivää
    }).
https-self-signed = Se on itse allekirjoitettu: selaimet varoittavat siitä.
https-key-matches = Avain täsmää.
https-where-to-get = Mistä varmenteen saa
https-get-tailscale-before = ota HTTPS käyttöön tailnetissäsi, aja sitten{ " " }
https-get-tailscale-after = { " " }palvelimella ja lähetä kaksi tiedostoa. Ne ovat voimassa 90 päivää: aja komento ja lähetä tiedostot uudelleen ennen sitä.
https-get-acme-title = Let’s Encrypt tai muu ACME-asiakas
https-get-acme-before = (certbot, acme.sh, lego) omistamallesi verkkotunnukselle: lähetä{ " " }
https-get-acme-and = { " " }ja{ " " }
https-get-acme-after = .
https-get-proxy-title = Käänteinen välityspalvelin
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), jolla varmenne on: jätä HTTPS tässä pois päältä ja anna välityspalvelimen lähettää tavallista http://-liikennettä Hoardille.
https-trying = Kokeillaan…
https-apply = Ota käyttöön
https-apply-hint = Hoard kokeilee uutta määritystä ensin ja pitää nykyisen, jos se ei toimi.
https-trust-title = Luota Hoardiin laitteillasi
https-download-ca = Lataa hoard-ca.crt
https-trust-hint = Asenna Hoardin varmentaja kerran jokaiselle laitteelle, niin sen selaimet luottavat tähän palvelimeen (ja kaikkiin varmenteisiin, joita Hoard sille myöhemmin tekee). Se voi taata vain tämän Hoardin. Sormenjälki (SHA-256):
https-ca-windows = Avaa tiedosto → Asenna varmenne → Nykyinen käyttäjä → Sijoita kaikki varmenteet seuraavaan säilöön → Selaa → Luotetut päävarmenteiden myöntäjät → Valmis. Käynnistä selain uudelleen.
https-ca-macos = Avaa tiedosto; se menee Avainnippuun. Kaksoisosoita siellä ”{ $subject }” → Luottamus → Käytettäessä tätä varmennetta: Luota aina.
https-ca-ios-title = iPhone ja iPad
https-ca-ios = Lataa se Safarissa ja salli se. Asetukset → Profiili ladattu → Asenna. Sitten Asetukset → Yleiset → Tietoja → Varmenteiden luottamusasetukset → laita päälle ”{ $subject }”.
https-ca-android = Asetukset → Tietoturva → Lisää tietoturva-asetuksia → Salaus ja kirjautumistiedot → Asenna varmenne → CA-varmenne, ja valitse tiedosto. (Nimet vaihtelevat hieman puhelimittain; hae Asetuksista sanaa ”varmenne”.)
https-ca-linux-1 = Debian ja Ubuntu:{ " " }
https-ca-linux-2 = . Fedora ja Arch:{ " " }
https-ca-linux-3 = . Linuxin Chrome pitää omaa luetteloaan: Asetukset → Tietosuoja ja turvallisuus → Turvallisuus → Hallinnoi varmenteita → Tuo.
https-ca-firefox = Firefox pitää omaa luetteloaan: Asetukset → Tietosuoja ja turvallisuus → Varmenteet → Näytä varmenteet → Varmentajat → Tuo, ja valitse ”Luota tähän varmentajaan sivustojen tunnistamisessa”.
https-ca-cli-title = hoard-komentorivityökalu
https-ca-cli-before = { "" }
https-ca-cli-middle = , tai laita polku muuttujaan{ " " }
https-ca-cli-after = .
https-options = Valinnat
https-redirect = Ohjaa muiden laitteiden selaimet http://-osoitteesta https://-osoitteeseen
https-redirect-hint = Vain sivujen lataukset ohjataan. hoard-komentorivityökalu, Unreal-työntekijä, skriptit ja tekoälysovellukset toimivat yhä http://-osoitteella, kuten myös http://localhost itse palvelimella.
https-http3-hint-before = Nopeampi epävakaassa Wi-Fissä ja puhelimissa. Vaatii HTTPS:n sekä avoimen UDP-portin { $port } TCP:n lisäksi (Dockerissa:{ " " }
https-http3-hint-after = ).
https-http3-running = Käynnissä.
https-http3-waits = Käynnistyy, kun HTTPS on päällä.
https-http3-failed = HTTP/3:n käynnistys epäonnistui: { $error }
https-lockout-title = Jos lukitset itsesi ulos
https-lockout-1 = { "" }
https-lockout-2 = { " " }toimii itse palvelimella aina tavallisella http://-yhteydellä. Tai kytke HTTPS pois palvelimen komentoriviltä ja käynnistä Hoard uudelleen:{ " " }
https-lockout-3 = { " " }(Dockerissa:{ " " }
https-lockout-4 = , sitten{ " " }
https-lockout-5 = ).

## Tili

account-role-view = Voi katsoa
account-role-view-hint = Haku, esikatselu ja lataus
account-role-edit = Voi muuttaa
account-role-edit-hint = Lisäksi lisäys, muutokset, tunnisteet, kokoelmat ja arkistointi
account-role-admin = Ylläpitäjä
account-role-admin-hint = Lisäksi Asetukset, tilit ja ylläpito sekä lopullinen poisto
account-source-internal = Hoard-salasana
account-source-ldap = Hakemiston salasana ({ $directory })
account-source-oauth = Kirjautuminen: { $provider }
account-source-word-directory = hakemisto
account-load-failed = Tilien lataus epäonnistui: { $error }
account-failed = { $what }: { $error }
account-password-failed = Salasanan vaihto epäonnistui
account-password-changed = Salasana vaihdettu. Muut laitteesi on kirjattu ulos.
account-add-failed = Tilin lisäys epäonnistui: { $name }
account-added = { $name } voi nyt kirjautua.
account-role-failed = Oikeuksien muutos epäonnistui: { $name }
account-remove-title = Poistetaanko { $name }?
account-remove-body = Hänet kirjataan ulos kaikkialta, eikä hän voi enää kirjautua. Mitään hänen kirjastoon lisäämäänsä ei poisteta.
account-remove = Poista
account-remove-failed = Poisto epäonnistui: { $name }
account-token-failed = Tunnuksen luonti epäonnistui
account-copied = Kopioitu
account-copy-token-failed = Kopiointi epäonnistui: valitse tunnus ja kopioi se itse
account-copy-failed = Kopiointi epäonnistui: valitse ja kopioi itse
account-revoke-title = Perutaanko ”{ $name }”?
account-revoke-body = Kaikki sitä yhä käyttävä käännytetään tästä lähtien pois.
account-revoke = Peru
account-revoke-failed = Tunnuksen ”{ $name }” peruminen epäonnistui
account-disconnect-title = Katkaistaanko sovelluksen { $app } yhteys?
account-disconnect-body = Se käännytetään tästä lähtien pois, ja se on yhdistettävä uudelleen, jotta se voi käyttää Hoardia.
account-disconnect = Katkaise yhteys
account-disconnect-failed = Sovelluksen { $app } yhteyden katkaisu epäonnistui
account-guest-failed = Muutos epäonnistui
account-never = ei koskaan
account-you = Sinä
account-sign-out = Kirjaudu ulos
account-signed-in-as-before = Kirjautunut käyttäjänä{ " " }
account-signed-in-as-admin = , joka on ylläpitäjä.
account-signed-in-as-edit = , joka voi muuttaa kirjastoa.
account-signed-in-as-view = , joka voi katsoa muttei muuttaa mitään.
account-app-signin = Hoard-sovellus kirjaa sinut sisään itse, joten salasanaa ei ole. Luo alla käyttöoikeustunnus pelimoottorien laajennuksille ja hoard-komennolle.
account-password-at-provider = Salasanaasi säilyttää palveluntarjoaja, jonka kautta kirjaudut, joten vaihda se siellä.
account-password-at-directory = Salasanaasi säilyttää hakemisto, jonka kautta kirjaudut, joten vaihda se siellä.
account-current-password = Nykyinen salasana
account-new-password = Uusi salasana
account-new-password-again = Uusi salasana uudelleen
account-change-password = Vaihda salasana
account-password-mismatch = Uudet salasanat eivät täsmää.
account-password-rule = Vähintään 8 merkkiä.
account-people = Käyttäjät
account-people-hint = Kukin voi katsoa (hakea, esikatsella ja ladata), myös muuttaa kirjastoa (lisätä, muokata, merkitä tunnisteilla, koota kokoelmiin ja arkistoida) tai olla lisäksi ylläpitäjä (Asetukset, tilit ja ylläpito sekä lopullinen poisto).
account-you-tag = (sinä)
account-signs-in-oauth = Kirjautuu OAuth-palveluntarjoajan kautta
account-signs-in-directory = Kirjautuu hakemiston kautta
account-since = { $date } alkaen
account-what-can-do = Mitä { $name } voi tehdä
account-name = Nimi
account-how-sign-in = Miten hän kirjautuu
account-password-label = Salasana (vähintään 8 merkkiä)
account-password-placeholder = Salasana (8+ merkkiä)
account-what-they-can-do = Mitä hän voi tehdä
account-add = Lisää
account-ai-apps = Claude ja muut tekoälysovellukset
account-ai-apps-hint = Yhdistä Claude (tai mikä tahansa MCP-sovellus), niin se voi hakea ja noutaa resursseja puolestasi sekä kuvailla ja merkitä tunnisteilla sen, mitä Hoard ei ole vielä tehnyt. Se kirjautuu Hoardiin ensimmäisellä kerralla; sinä sallit sen täällä selaimessa.
account-mcp-address = Hoardin MCP-osoite
account-copy = Kopioi
account-claude-code-1 = aja{ " " }
account-claude-code-2 = , sitten{ " " }
account-claude-code-3 = { " " }kirjautuaksesi.{ " " }
account-claude-code-4 = { " " }kuvailee paketin.
account-claude-ai = Settings → Connectors → Add custom connector, liitä osoite ja valitse Connect. Nämä yhdistävät Anthropicin palvelimilta, joten Hoardin on oltava tavoitettavissa internetistä HTTPS:n kautta.
account-claude-ai-local = Settings → Connectors → Add custom connector, liitä osoite ja valitse Connect. Nämä yhdistävät Anthropicin palvelimilta, joten Hoardin on oltava tavoitettavissa internetistä HTTPS:n kautta (tämä osoite ei ole: aseta Julkinen osoite kohdassa Asetukset → Yhteys).
account-connection-meta = käyttäjälle { $user } { $since } alkaen · viimeksi käytetty: { $used }
account-no-apps = Ei vielä yhdistettyjä sovelluksia.
account-live = Käyttävät Hoardia nyt
account-live-hint = Sovellukset, jotka ovat pyytäneet Hoardilta jotain viimeisten 10 minuutin aikana. Päivittyy itsestään.
account-live-none = Mikään ei ole pyytänyt Hoardilta mitään viimeisten 10 minuutin aikana.
account-live-last = viimeksi: { $what } · { $when } · pyyntöjä: { $calls }
account-tokens = Käyttöoikeustunnukset
account-tokens-hint-1 = Käyttö:{ " " }
account-tokens-hint-2 = -komentorivityökalu, skriptit, MCP-asiakkaat ja Unreal-työntekijä lähettävät sen muodossa{ " " }
account-tokens-hint-3 = . Tunnus voi tehdä sen, minkä valitset, enintään sen, mitä itse voit, mutta ei koskaan hallita tilejä tai tunnuksia. Pelkkä katseluoikeus riittää hakuun, lataamiseen ja projektin synkronointiin.
account-tokens-admin = Näet kaikkien tunnukset ja yhdistetyt sovellukset; muut näkevät omansa.
account-made-before = { "" }
account-made-after = : kopioi se nyt. Sitä ei näytetä uudelleen.
account-new-token = Uusi tunnus
account-cli-1 = Komentoriville:{ " " }
account-cli-2 = , tai laita se muuttujaan{ " " }
account-cli-3 = .
account-done = Valmis
account-token-meta = tehty { $made } · viimeksi käytetty: { $used }
account-token-meta-by = tehty { $made }, tekijä { $by } · viimeksi käytetty: { $used }
account-no-tokens = Ei vielä tunnuksia.
account-token-name = Mikä sitä käyttää
account-token-name-placeholder = Mikä sitä käyttää, esim. ”Unreal-työntekijä”
account-token-role = Mitä se voi tehdä
account-make-token = Luo tunnus
account-guests = Vieraat
account-guests-allow = Anna kenen tahansa palvelimen tavoittavan selata kirjautumatta
account-guests-hint = He voivat hakea, esikatsella ja ladata, mutta eivät lisätä, muuttaa tai poistaa mitään eivätkä nähdä Asetuksia.

## Kirjautuminen

signin-create-title = Luo tilisi
signin-title = Kirjaudu sisään
signin-setup-lede = Tällä palvelimella ei ole vielä tiliä. Tässä luomasi tili voi tehdä kaiken; lisää tilit muille myöhemmin kohdassa Asetukset → Tili.
signin-overlay-lede = Selaaminen on täällä avoinna kaikille, mutta muutokset vaativat tilin.
signin-with = Kirjaudu sisään: { $provider }
signin-or-password = tai nimellä ja salasanalla
signin-username = Käyttäjänimi
signin-name = Nimi
signin-new-password = Uusi salasana
signin-password = Salasana
signin-password-rule = Vähintään 8 merkkiä.
signin-confirm-password = Vahvista salasana
signin-mismatch = Salasanat eivät täsmää.
signin-setup-code = Asennuskoodi
signin-setup-code-hint-before = Koodi, joka tälle palvelimelle annettiin muuttujassa{ " " }
signin-setup-code-hint-after = .
signin-creating = Luodaan…
signin-signing-in = Kirjaudutaan…
signin-create = Luo tili
signin-submit = Kirjaudu sisään
signin-keep-browsing = Jatka selaamista
signin-directory-hint = Käytä hakemiston ({ $directory }) nimeäsi ja salasanaasi.
signin-forgot-1 = Unohditko Hoard-salasanan? Aja palvelimella{ " " }
signin-forgot-2 = { " " }(Dockerissa{ " " }
signin-forgot-3 = ) saadaksesi uuden.

## Käyttöönotto

setup-title = Missä kirjastosi on?
setup-lede = Valitse palvelimelta kansio, jossa Hoard säilyttää pakettisi, tai osoita sille kansio, jossa sinulla jo on paketteja. Voit lisätä myöhemmin sijainteja muille levyille Asetuksista.
setup-lede-desktop = Valitse kansio, jossa Hoard säilyttää pakettisi, tai osoita sille kansio, jossa sinulla jo on paketteja. Voit lisätä myöhemmin sijainteja muille levyille Asetuksista.
setup-unreachable = Palvelinta ei tavoitettu: { $error }
setup-unreachable-desktop = Hoard ei vastaa: { $error }
setup-try-again = Yritä uudelleen
setup-loading = Ladataan…
setup-start = Aloita kirjasto
setup-ready = Kirjastosi on valmis
setup-managed-before = Uudet paketit menevät kansioon{ " " }
setup-managed-after = . Pudota tiedostoja, kansioita tai arkistoja mihin tahansa sivulle lisätäksesi ensimmäiset.
setup-watched-before = Hoard valvoo kansiota{ " " }
setup-watched-after = ; paketit ilmestyvät sitä mukaa kuin se löytää niitä. Jos haluat myös lähettää tiedostoja, lisää Hoardin ylläpitämä kirjasto kohdassa Asetukset → Kirjaston sijainnit.
setup-open = Avaa kirjasto

## Sovelluksen yhdistäminen

connect-title = Yhdistetäänkö { $app }?
connect-lede-1 = { " " }voi hakea, ladata, kuvailla ja merkitä tunnisteilla kaiken tässä Hoardissa toimien käyttäjänä{ " " }
connect-lede-2 = . Se ei voi hallita tilejä eikä käyttöoikeustunnuksia.
connect-back-before = Palaat osoitteeseen{ " " }
connect-back-after = . Voit katkaista yhteyden milloin tahansa kohdassa Asetukset → Tili.
connect-allow = Salli
connect-cancel = Peruuta
connect-failed = Tätä sovellusta ei voi yhdistää
connect-go-library = Siirry kirjastoon
connect-checking = Tarkistetaan…

## Salaamaton yhteys

insecure-text = Tämä on tavallinen http://, joten muutama asia vaatii HTTPS:n: 3D WebGPU:lla, nykyaikainen leikepöytä, Hoardin asentaminen sovellukseksi ja Claude-liittimet.
insecure-switch = Vaihda https://-osoitteeseen
insecure-set-up = Ota HTTPS käyttöön
insecure-dismiss = Älä näytä tätä uudelleen

## Muotoavustaja

formatwizard-too-big = Tiedosto on liian suuri luettavaksi kaupan tiedostona.
formatwizard-not-json = Tiedosto ei ole kelvollista JSONia, joten se näytetään tekstinä.
formatwizard-read-failed = Tiedoston luku epäonnistui: { $error }
formatwizard-back-title = Palataanko lomakkeeseen?
formatwizard-back-body = TOML-muokkaukset menetetään: lomake kirjoittaa muodon uudelleen omista tiedoistaan.
formatwizard-back-to-form = Takaisin lomakkeeseen
formatwizard-miss-name = Sen tiedostot eivät hyväksy nimeä { $file }. Lisää se kohtaan Tiedostot.
formatwizard-miss-not-the-stores = Se, mistä se tunnistaa kaupan tiedostot, ei ole tässä tiedostossa. Muuta kohtaa Tunnista.
formatwizard-miss-no-title = Se ei löydä tästä tiedostosta otsikkoa. Valitse otsikko.
formatwizard-miss-short-name = ei sen nimi
formatwizard-miss-short-not-the-stores = ei kaupan
formatwizard-miss-short-no-title = ei otsikkoa
formatwizard-saved-packs =
    Tallennettu: { $file }. { $packs ->
        [one] { $packs } paketti luetaan
       *[other] { $packs } pakettia luetaan
    } uudelleen.
formatwizard-saved = Tallennettu: { $file }.
formatwizard-save-failed = Muodon tallennus epäonnistui: { $error }
formatwizard-zip-failed = Zipin luonti epäonnistui: { $error }
formatwizard-stop-title = Lopetetaanko tämän tiedoston opettaminen?
formatwizard-stop-body = Muotoa ei tallenneta.
formatwizard-stop = Lopeta
formatwizard-label = Muotoavustaja
formatwizard-changing = Muokataan: { $name }
formatwizard-starting-from = Pohjana: { $name }
formatwizard-title = Opeta Hoardille kaupan tiedosto
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }paketissa{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Muokkaa TOML-muodossa
formatwizard-file-reference = Tiedosto viitteeksi.
formatwizard-click-json = Napsauta tiedostossa kohtaa, jossa on { $field }.
formatwizard-click-text = Napsauta tiedostossa kohtaa, jossa on { $field }, tai valitse se.
formatwizard-pick-first = Valitse kenttä oikealta ja napsauta sitten sen arvoa tässä.
formatwizard-loading = Ladataan…
formatwizard-the-format = Muoto
formatwizard-syntax-before = Syntaksi on kuvattu tiedostossa{ " " }
formatwizard-syntax-after = .
formatwizard-store = Kauppa
formatwizard-store-placeholder = Kaupan nimi, kuten Kenney
formatwizard-files = Tiedostot
formatwizard-recognize = Tunnista
formatwizard-recognize-json = Avain, joka on vain kaupan tiedostoissa
formatwizard-recognize-text = Rivi, joka on vain kaupan tiedostoissa
formatwizard-recognize-hint = Yksi tapa riviä kohden; mikä tahansa niistä riittää. Tarvitaan, kun muilla tiedostoilla on sama nimi.
formatwizard-seller = Myyjä
formatwizard-seller-placeholder = Kun tiedosto ei kerro
formatwizard-tags = Tunnisteet
formatwizard-tags-placeholder = Kaikelle, mitä se kuvaa, pilkuin eroteltuina
formatwizard-fields = Kentät
formatwizard-selector = Valitsin: { $field }
formatwizard-clear = Tyhjennä { $field }
formatwizard-pick = Valitse
formatwizard-what-it-reads = Mitä se lukee
formatwizard-read-title = Otsikko
formatwizard-read-seller = Myyjä
formatwizard-read-license = Lisenssi
formatwizard-read-url = Kauppasivu
formatwizard-read-kind = Tyyppi
formatwizard-read-tags = Tunnisteet
formatwizard-read-origin = Tekopaikka
formatwizard-read-size = Koko
formatwizard-also = { $format } lukee myös tämän tiedoston. Tätä muotoa kokeiltaisiin ensin.
formatwizard-library =
    { $files } { $files ->
        [one] tiedostolla
       *[other] tiedostolla
    } { $packs } { $packs ->
        [one] paketissa
       *[other] paketissa
    } on nimi, jonka se hyväksyy. Kokeilluista { $tried } tiedostosta se lukee { $read }.
formatwizard-library-none = Kirjastossa ei ole yhtään tiedostoa, jonka nimen se hyväksyy.
formatwizard-share = Jaa se
formatwizard-share-hint = Zip, jossa on muoto, tämä tiedosto sen näytteenä ja se, miksi näyte luetaan, järjestettynä kuten Hoardin repositoriossa, lähetettäväksi, jotta kaikki saavat sen. Poista ensin näytteestä kaikki, mitä ei pidä jakaa.
formatwizard-sample = Jaettava näyte
formatwizard-download = Lataa { $file }
formatwizard-close = Sulje
formatwizard-use = Käytä tässä
formatwizard-cancel = Peruuta
settings-unreal-editor-what = Asentaa Unreal Editorin vientityöntekijää varten. Vie aineistoa käynnistämällä työntekijä erikseen.
settings-unreal-download = Lataa Epicin Linux x86-64 -editorin. Vaatii yhdistetyn GitHub-käyttöoikeuden ja kymmeniä gigatavuja levytilaa.
settings-unreal-image = Unreal-versio tai levykuva
settings-editor-path = Anna asennetun { $tool } -ohjelman suoritettavan tiedoston polku. Automaattinen asennus on käytettävissä vain Dockerissa.
error-unreal-image = Anna Unreal 5 -versio, Epicin dev/dev-slim-versiotunniste tai Epicin SHA-256-kuvaviite.
