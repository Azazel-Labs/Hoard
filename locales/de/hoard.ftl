## Serverfehler

error-account-not-found = Konto nicht gefunden
error-asset-gone = Asset nicht gefunden
error-asset-not-found = Asset { $id } nicht gefunden
error-blob-not-found = Blob { $hash } nicht gefunden
error-collection-not-found = Sammlung { $id } nicht gefunden
error-connection-not-found = Verbindung nicht gefunden
error-derivative-not-found = abgeleitete Datei ({ $kind }) von { $id } nicht gefunden
error-file-in-pack-not-found = { $path } in Paket { $id } nicht gefunden
error-site-not-a-page = nur Webseiten öffnen sich als Seite
error-site-link-expired = die Adresse dieser Seite ist abgelaufen; öffnen Sie die Seite erneut aus Hoard
error-format-not-found = Format { $file } nicht gefunden
error-frame-sheet-aseprite-only = nur Aseprite-Dateien haben ein Frame-Sheet
error-frame-sheet-frames = das hat nur einen Frame oder zu viele, um sie anzuordnen
error-location-not-found = Speicherort { $id } nicht gefunden
error-maintenance-task-not-found = Wartungsaufgabe { $id } nicht gefunden
error-open-upload-not-found = offener Upload { $id } nicht gefunden
error-original-not-found = Originaldatei von { $id } nicht gefunden
error-pack-not-found = Paket { $id } nicht gefunden
error-preview-too-big = dieses Bild ist zu groß für eine leichte Vorschau
error-project-not-found = Projekt { $name } nicht gefunden
error-saved-search-not-found = gespeicherte Suche { $id } nicht gefunden
error-sign-in-first = zuerst anmelden (oder ein Zugriffstoken mitsenden)
error-those-assets-not-found = diese Assets wurden nicht gefunden
error-thumbnail-not-found = Vorschaubild für { $id } nicht gefunden
error-token-not-found = Token nicht gefunden
error-upload-not-found = Upload { $id } nicht gefunden
error-pack-needs-name = ein Paket braucht einen Namen
error-no-library = es gibt noch keinen Speicherort für die Dateien der Bibliothek: Fügen Sie unter Einstellungen → Speicherorte einen verwalteten hinzu
error-upload-failed = Upload fehlgeschlagen: { $reason }
error-upload-no-files = keine Dateien im Upload
error-analyze-nothing-to-do = nichts zu tun: rules=false und das LLM ist aus
error-too-many-assets = zu viele Assets in einer Anfrage
error-change-not-found = diese Änderung steht nicht im Verlauf dieses Assets
error-bulk-by-look = eine Suche nach Aussehen lässt sich nicht auf einmal bearbeiten: wähle stattdessen die Assets aus
error-too-many-items = zu viele Einträge in einer Anfrage
error-not-a-hash = kein blake3-Hash in Hexadezimalschreibweise
error-merge-into-tag = into muss ein Tag sein
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = „{ $text }“ ist kein Tag
error-not-a-character = nur eine Figur kann die sein, auf der die Animationen ihres Skeletts laufen
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } steht schon unter { $tag }, deshalb kann { $tag } nicht darunter stehen
error-nothing-to-merge = nichts zum Zusammenführen
error-selection-size = geben Sie zwischen 1 und { $max } Asset-IDs an
error-collection-needs-name = eine Sammlung braucht einen Namen
error-collection-exists = es gibt bereits eine Sammlung namens { $name }
error-collection-empty = Sammlung { $name } ist leer
error-conversion-failed = Umwandlung in { $format } nicht möglich: { $reason }
error-max-size = max muss eine Zweierpotenz von 64 bis 16384 sein
error-replace-ext = ext muss eine kurze Dateiendung sein
error-empty-file = leere Datei
error-imitation-silent = In der Aufnahme hebt sich nichts vom Hintergrund ab. Versuch es näher am Mikrofon noch einmal.
error-replace-duplicate = die umgewandelte Datei gleicht einer, die schon im Paket ist; beide wurden zusammengeführt
error-ai-address = Die KI-Adresse sollte mit http:// oder https:// beginnen
error-hosted-key = Geben Sie den API-Schlüssel für { $provider } ein
error-hosted-address = Die Adresse des Bildsuchdienstes sollte mit http:// oder https:// beginnen
error-local-llm-action = action ist list, download oder delete, und die letzten beiden brauchen ein Modell
error-role = Eine Rolle ist view, edit oder admin
error-admin-only = das darf nur ein Admin
error-view-only = dieses Konto darf ansehen, aber nichts ändern: Ein Admin kann das unter Einstellungen → Konto ändern
error-portal-only = melden Sie sich im Portal an, um Konten und Tokens zu verwalten
error-not-from-portal = diese Anfrage kam nicht vom Hoard-Portal
error-account-name = Wählen Sie einen Namen mit bis zu 64 Zeichen
error-password-length = Verwenden Sie ein Passwort mit mindestens { $min } Zeichen
error-too-many-codes = Zu viele falsche Codes: Warten Sie { $seconds } Sekunden
error-already-set-up = Es gibt bereits ein Konto: Melden Sie sich an
error-too-many-tries = Zu viele Fehlversuche: Warten Sie { $seconds } Sekunden
error-directory-unavailable = Das Verzeichnis konnte Ihr Passwort nicht prüfen: { $reason }
error-password-elsewhere = Ihr Passwort wird vom Verzeichnis oder Anbieter verwaltet, über den Sie sich anmelden
error-current-password = Ihr aktuelles Passwort stimmt nicht
error-account-source = Ein Konto meldet sich über Hoard (internal), ein Verzeichnis (ldap) oder OAuth an
error-account-exists = Es gibt bereits ein Konto mit diesem Namen
error-own-role = Ihre eigene Rolle kann nur ein anderer Admin ändern
error-remove-self = Sie können Ihr eigenes Konto nicht entfernen, während Sie damit angemeldet sind
error-token-name = Benennen Sie das Token nach dem, was es verwenden wird, mit bis zu 80 Zeichen
error-token-role = Ein Token darf nicht mehr als das Konto, das es erstellt
error-wrong-setup-code = Das ist nicht der Einrichtungscode, der dem Server in HOARD_SETUP_CODE übergeben wurde
error-wrong-password = Name und Passwort passen nicht zusammen
error-folder-missing = { $path } ist nicht vorhanden (ein nicht eingehängtes Laufwerk?)
error-s3-path = ein S3-Speicherort sieht so aus: s3://bucket
error-s3-address = die S3-Adresse muss mit http:// oder https:// beginnen
error-s3-header = ein zusätzlicher S3-Header hat die Form Name=value, und „{ $header }“ ist keiner
error-keychain = konnte es nicht im Schlüsselbund des Systems ablegen: { $reason }
error-not-built = diese Hoard-Version wurde ohne { $feature } erstellt
error-openai-auth-unreachable = auth.openai.com ist nicht erreichbar
error-openai-unexpected = unerwartete Antwort von OpenAI
error-openai-answered = OpenAI antwortete { $status }: { $reason }
error-chatgpt-id-token-not-jwt = das ID-Token ist kein JWT
error-chatgpt-id-token-issuer = das ID-Token stammt von { $issuer }, nicht von { $expected }
error-chatgpt-id-token-audience = das ID-Token ist für eine andere App
error-chatgpt-id-token-expired = das ID-Token ist abgelaufen
error-chatgpt-id-token-nonce = das ID-Token gehört nicht zu dieser Anmeldung
error-chatgpt-no-state = die Adresse enthält keinen state; kopieren Sie die ganze Adresse
error-chatgpt-not-started = diese Anmeldung wurde nicht hier gestartet oder vor mehr als 15 Minuten. Starten Sie sie in den Einstellungen erneut.
error-chatgpt-canceled = die Anmeldung wurde abgebrochen
error-chatgpt-refused = ChatGPT hat die Anmeldung abgelehnt: { $reason }
error-chatgpt-no-code = die Adresse enthält keinen Anmeldecode; kopieren Sie die ganze Adresse
error-chatgpt-other-registration = ChatGPT hat für eine andere Registrierung dieses Servers geantwortet; starten Sie die Anmeldung erneut
error-chatgpt-other-registration-desktop = ChatGPT hat für eine andere Kopie von Hoard geantwortet; starten Sie die Anmeldung erneut
error-chatgpt-not-registered = ChatGPT hat die Registrierung von Hoard nicht abgeschlossen; starten Sie die Anmeldung erneut
error-chatgpt-code-used = der Anmeldecode wurde bereits verwendet oder ist abgelaufen; melden Sie sich erneut an
error-chatgpt-refused-code = ChatGPT hat die Anmeldung abgelehnt ({ $code })
error-chatgpt-no-id-token = ChatGPT hat kein ID-Token gesendet
error-chatgpt-plan-not-allowed = Angemeldet, aber die Nutzung des Abos wurde nicht erlaubt. Melden Sie sich erneut an und erlauben Sie sie, oder verwenden Sie stattdessen einen API-Schlüssel.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = nicht bei ChatGPT angemeldet; melden Sie sich unter Einstellungen → KI-Tagging an
error-chatgpt-refresh-later = ChatGPT konnte die Anmeldung gerade nicht erneuern ({ $code }); es wird später erneut versucht
error-chatgpt-signed-out-problem = ChatGPT hat diesen Server abgemeldet ({ $code }). Melden Sie sich erneut an, um weiter mit Ihrem Abo zu taggen.
error-chatgpt-signed-out-problem-desktop = ChatGPT hat Hoard abgemeldet ({ $code }). Melden Sie sich erneut an, um weiter mit Ihrem Abo zu taggen.
error-chatgpt-signed-out = ChatGPT hat diesen Server abgemeldet ({ $code }); melden Sie sich unter Einstellungen → KI-Tagging erneut an
error-chatgpt-signed-out-desktop = ChatGPT hat Hoard abgemeldet ({ $code }); melden Sie sich unter Einstellungen → KI-Tagging erneut an
error-openai-api-unreachable = api.openai.com ist nicht erreichbar
error-openai-broke-off = die Antwort von OpenAI brach ab
error-chatgpt-kept-refusing = OpenAI hat die ChatGPT-Anmeldung wiederholt abgelehnt; melden Sie sich unter Einstellungen → KI-Tagging erneut an
error-chatgpt-request-failed = die Anfrage ist fehlgeschlagen
error-chatgpt-cut-off = die Antwort wurde abgeschnitten ({ $reason })
error-chatgpt-cut-off-no-reason = die Antwort wurde abgeschnitten (ohne Angabe eines Grundes)
error-openai-ended-early = die Antwort von OpenAI endete, bevor sie vollständig war
error-chatgpt-usage-limit = das Kontingent Ihres ChatGPT-Abos für Hoard ist vorerst aufgebraucht. Das Limit können Sie unter { $url } einsehen oder ändern
error-chatgpt-not-eligible = dieses ChatGPT-Konto oder dieser Workspace kann sein Abo nicht in anderen Apps nutzen. Verwenden Sie stattdessen einen API-Schlüssel.
error-chatgpt-usage-unavailable = ChatGPT konnte das Kontingent des Abos gerade nicht prüfen; es wird später erneut versucht
error-chatgpt-unsupported = die Nutzung über das ChatGPT-Abo unterstützt { $param } bei diesem Modell nicht
error-chatgpt-unsupported-images = die Nutzung über das ChatGPT-Abo unterstützt { $param } bei diesem Modell nicht; wählen Sie ein Modell, das Bilder annimmt, oder schalten Sie das Senden von Vorschaubildern aus
error-chatgpt-unsupported-part = die Nutzung über das ChatGPT-Abo unterstützt einen Teil der Anfrage bei diesem Modell nicht
error-chatgpt-port = port ist der Port, über den der Browser Hoard erreicht
error-chatgpt-paste-address = fügen Sie die ganze Adresse ein, beginnend mit http://127.0.0.1
error-too-many-hashes = höchstens { $max } Hashes auf einmal
error-credits-pack = Paket { $key }
error-credits-collection = Sammlung { $key }
error-format-too-long = das Format ist zu lang
error-not-started = der Server ist nicht gestartet
error-not-started-desktop = Hoard ist nicht gestartet
error-not-format-file = keine Datei eines Formats: { $name }
error-format-name = der Name braucht einen Buchstaben von A bis Z oder eine Ziffer von 0 bis 9
error-format-miss-name = die Dateien des Formats umfassen { $file } nicht
error-format-miss-store = das Format nimmt { $file } nicht als Datei des Stores an (siehe require)
error-format-miss-title = das Format findet keinen Titel in { $file }
error-cant-download-as = .{ $ext }-Dateien können nicht als { $format } heruntergeladen werden
error-only-images-resize = nur Bilder können skaliert werden
error-https-no-certificate = Fügen Sie zuerst Ihr Zertifikat und dessen Schlüssel ein oder laden Sie sie hoch
error-https-no-cert-found = Kein Zertifikat gefunden: Fügen Sie den Text ein, der mit -----BEGIN CERTIFICATE----- beginnt
error-https-no-key-found = Kein privater Schlüssel gefunden: Fügen Sie den Text ein, der mit -----BEGIN PRIVATE KEY----- beginnt
error-https-several-keys = Es ist mehr als ein privater Schlüssel vorhanden: Fügen Sie nur den ein, der zum Zertifikat gehört
error-https-key-password = Der Schlüssel ist mit einem Passwort geschützt: Geben Sie ihn ohne Passwort an (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Das Zertifikat ist kein gültiges PEM
error-https-key-not-pem = Der private Schlüssel ist kein gültiges PEM
error-https-key-mismatch = Der Schlüssel gehört nicht zum Zertifikat. Wenn Sie eine Kette angegeben haben, stellen Sie das Zertifikat Ihres Servers an den Anfang
error-https-unusable = Das Zertifikat oder der Schlüssel kann nicht verwendet werden
error-https-expired = Dieses Zertifikat ist am { $date } abgelaufen
error-https-not-yet-valid = Dieses Zertifikat ist erst ab { $date } gültig
error-https-unreadable = Das ist kein Zertifikat, das Hoard lesen kann
error-https-bad-name = „{ $name }“ ist keine Adresse, die ein Zertifikat enthalten kann
error-https-test-timed-out = bei der Testverbindung kam es zu einer Zeitüberschreitung
error-https-test-failed = die Testverbindung ist fehlgeschlagen
error-https-no-ca = Hoard hat seine Zertifizierungsstelle noch nicht erstellt
error-https-unavailable = HTTPS ist auf diesem Server nicht verfügbar
error-https-certificate-mode = Zu „Eigenes Zertifikat“ gehört ein Zertifikat
error-https-test = Das Zertifikat hat bei einer Testverbindung nicht funktioniert: { $reason }
error-nothing-to-retry = dort gibt es nichts, was sich erneut versuchen lässt
error-ai-unreachable = { $url } ist nicht erreichbar
error-ai-key-refused = { $url } hat den API-Schlüssel abgelehnt ({ $status })
error-ai-answered = { $url } antwortete { $status }: { $reason }
error-ai-not-json = { $url }/models hat nicht mit JSON geantwortet
error-model-server-refused = der Modellserver: { $reason }
error-no-model-server = unter { $url } antwortet kein Modellserver
error-not-llama-router = { $url } ist kein llama.cpp-Server im Router-Modus und kann daher keine Modelle herunterladen
error-model-download = { $model } konnte nicht heruntergeladen werden: { $reason }
error-model-delete = { $model } konnte nicht gelöscht werden: { $reason }
error-unknown-kind = unbekannte Asset-Art { $kind }
error-watched-is-folder = ein überwachter Speicherort ist ein Ordner auf diesem Server
error-cant-reach = { $path } ist nicht erreichbar: { $reason }
error-folder-full-path = geben Sie den vollständigen Pfad des Ordners an
error-cant-create = { $path } kann nicht erstellt werden
error-cant-write = in { $path } kann nicht geschrieben werden
error-cant-read = { $path } kann nicht gelesen werden
error-overlaps-location = das überschneidet sich mit dem Speicherort { $name } ({ $path })
error-overlaps-data = das überschneidet sich mit Hoards Datenordner ({ $path })
error-overlaps-cache = das überschneidet sich mit Hoards Cache-Ordner ({ $path })
error-not-watched = { $name } ist kein überwachter Ordner
error-watched-no-kinds = ein überwachter Ordner nimmt keine neuen Dateien auf und hat daher keine Arten zum Ändern
error-keep-catch-all = machen Sie stattdessen einen anderen Speicherort zum Auffangort
error-location-holds-files = { $name } enthält noch Dateien: Verschieben Sie sie zuerst an die anderen Speicherorte (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } ist nicht erreichbar, daher kann Hoard nicht feststellen, ob dort noch Dateien liegen
error-relocate-running = es werden bereits Dateien zwischen Speicherorten verschoben; versuchen Sie es erneut, wenn das abgeschlossen ist
error-watched-not-moved = { $name } ist ein überwachter Ordner: Hoard verschiebt seine Dateien nicht
error-no-other-location = es gibt keinen anderen Speicherort, an den die Dateien verschoben werden könnten: Fügen Sie zuerst einen hinzu
error-full-path = geben Sie einen vollständigen Pfad an
error-pick-folder = wählen Sie einen Ordner
error-task-running = „{ $task }“ läuft bereits
error-job-cant-stop = „{ $job }“ lässt sich nach dem Start nicht mehr anhalten
error-job-not-found = dieser Auftrag wartet nicht mehr und läuft auch nicht
error-task-cant-stop = „{ $task }“ lässt sich nicht mittendrin anhalten
error-task-not-running = „{ $task }“ läuft nicht
error-token-account-gone = das Konto dieses Tokens existiert nicht mehr, daher hat es keine Favoriten oder Suchen
error-favorites-too-many = höchstens 10.000 auf einmal
error-saved-search-name = eine gespeicherte Suche braucht einen Namen mit höchstens 80 Zeichen
error-saved-search-too-long = diese Suche ist zu lang zum Speichern
error-hoard-seen = der Hort kann höchstens 32 Arten von Assets festhalten, jede mit einem Namen aus Kleinbuchstaben
error-news-seen = die Neuigkeiten können höchstens 256 gesehene Dinge festhalten, jedes mit einem Namen aus Kleinbuchstaben, Ziffern und Bindestrichen
error-texture-pattern-map = Das Muster für Texturnamen braucht { "{" }map{ "}" }, sonst bekämen alle Maps eines Materials denselben Namen
error-texture-pattern-fields = Das Muster für Texturnamen kann { "{" }name{ "}" }, { "{" }map{ "}" } und { "{" }size{ "}" } verwenden
error-texture-name-chars = Texturnamen dürfen kein < > : " / \ | ? oder * enthalten
error-texture-map-type = Es gibt keinen Map-Typ namens { $role }
server-role-base-color = Farbe
server-role-normal = Normal
server-role-normal-dx = Normal (DirectX)
server-role-roughness = Rauheit
server-role-gloss = Glanz
server-role-metalness = Metallizität
server-role-ao = Ambient Occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = Höhe
server-role-bump = Bump
server-role-cavity = Cavity
server-role-specular = Specular
server-role-opacity = Deckkraft
server-role-emissive = Emission
server-role-translucency = Transluzenz
error-texture-suffix-missing = Geben Sie Maps vom Typ { $map } ein Suffix
error-texture-suffix-chars = Das Suffix für Maps vom Typ { $map } darf kein < > : " / \ | ? oder * enthalten
error-texture-ending-missing = Jede eigene Endung braucht die Endung, nach der gesucht wird
error-texture-ending-chars = Eigene Endungen dürfen kein < > : " / \ | ? oder * enthalten
error-texture-ending-twice = { $ending } steht zweimal in Ihren Endungen
error-oauth-no-client-id = Dieser Link enthält keine client_id.
error-oauth-unknown-app = Diese App ist hier nicht (mehr) registriert. Starten Sie die Verbindung erneut in der App.
error-oauth-redirect = Die App wollte an eine Adresse zurückgeleitet werden, die sie nicht registriert hat.
error-oauth-response-type = Hoard gibt nur Anmeldecodes aus (response_type=code).
error-oauth-pkce = Die App muss PKCE mit S256 verwenden, um sich bei Hoard anzumelden.
error-light-preview-images = nur Bilder haben eine leichte Vorschau
error-no-light-preview = keine leichte Vorschau: { $reason }
error-no-frame-sheet = kein Frame-Sheet: { $reason }
error-thumbnail-source = unbekannte Quelle für Vorschaubild: { $source }
error-thumbnail-too-big = dieses Bild ist zu groß für ein Vorschaubild
error-thumbnail-models-only = nur Modelle nehmen ein vom Browser gezeichnetes Vorschaubild an
error-not-a-picture = kein Bild, das Hoard lesen kann: { $reason }
error-project-name = ein Projekt braucht einen Namen mit höchstens 200 Zeichen
error-project-too-big = höchstens eine Million Assets in einem Projekt
error-project-too-many-at-once = höchstens 100.000 auf einmal
error-server-name = Wählen Sie einen Namen mit bis zu { $max } Zeichen, in einer Zeile
error-tool-setting = { $tool }: Wählen Sie „Aus“ oder „Automatisch“, oder geben Sie einen Pfad an
error-ai-model = Wählen Sie das Modell, das der KI-Dienst verwenden soll
error-public-address = Die öffentliche Adresse sollte mit http:// oder https:// beginnen
error-update-hook-address = Die Adresse, die bei Updates aufgerufen wird, sollte mit http:// oder https:// beginnen
error-update-hook-auth = Der Authorization-Header darf keine Zeilenumbrüche oder anderen Steuerzeichen enthalten
error-no-soundfont = Unter { $path } gibt es auf dem Server keinen SoundFont
error-no-soundfont-desktop = Unter { $path } gibt es keinen SoundFont
error-soundfont-bad = { $name } ist kein SoundFont, mit dem MIDI spielen kann: { $reason }
error-workers = Hintergrund-Worker: zwischen 1 und { $max }
error-ai-batch = Assets pro KI-Anfrage: zwischen 1 und 200
error-conversion-cache = Umgewandelte Downloads: mindestens 100 MB
error-motion-fps = bewegte Vorschaubilder laufen mit 12, 24 oder 30 Bildern pro Sekunde
error-backup-keep = Geplante Sicherungen: 1 bis { $max } behalten
error-backup-copy-to = Geplante Sicherungen: { $path } ist kein Ordner auf dem Server (vollständigen Pfad angeben)
error-conversion-keep-days = Umwandlungsregeln: Originale 1 bis { $max } Tage behalten
error-conversion-target = Umwandlungsregeln: Hoard kann Dateien nicht in { $format } umwandeln
error-conversion-no-source = Umwandlungsregeln: Jede Regel braucht mindestens ein Format, das sie umwandelt
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Umwandlungsregeln: .{ $ext }-Dateien können nicht in { $format } umgewandelt werden
error-conversion-twice = Umwandlungsregeln: .{ $ext } steht in zwei Regeln
error-conversion-cant-undo = Diese Datei wurde nicht von einer Regel umgewandelt, oder ihr Original wurde schon gelöscht.
error-conversion-path-taken = Unter { $path } liegt jetzt eine andere Datei, daher kann das Original nicht dorthin zurück.
error-limits-zero = Limits dürfen nicht null sein
error-settings-object = ein JSON-Objekt mit Einstellungen erwartet
error-bad-setting = ungültige Einstellung: { $reason }
error-auth-default-role = Die Standardrolle ist view, edit oder admin
error-auth-session-hours = Sitzungen dauern zwischen 1 Stunde und einem Jahr
error-auth-settings = ungültige Authentifizierungseinstellungen: { $reason }
error-ldap-settings = ungültige LDAP-Einstellungen: { $reason }
error-ldap-no-server = Fügen Sie die Adresse des Verzeichnisservers hinzu
error-ldap-base-dn = Geben Sie an, wo die Einträge der Personen liegen (die Base-DN)
error-ldap-username-attribute = Geben Sie an, welches Attribut den Namen enthält, mit dem man sich anmeldet
error-ldap-bind-dn = Geben Sie das Konto an, das Personen nachschlägt, oder schlagen Sie sie anonym nach
error-ldap-ca-cert = Die Zertifizierungsstelle ist kein PEM-Zertifikat (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Die Adresse eines Verzeichnisservers ist leer
error-ldap-server-form = { $server }: Die Adresse eines Verzeichnisservers ist host:port, ldap://… oder ldaps://…
error-ldap-not-starttls = { $server }: Das ist LDAPS, nicht StartTLS
error-ldap-no-certificate = kein Zertifikat darin
error-ldap-password-again = Geben Sie das Passwort des Kontos, das Personen nachschlägt, erneut ein: Das gespeicherte wird nur an die Server gesendet, für die es angegeben wurde
error-ldap-unreachable = kein Verzeichnisserver war erreichbar ({ $reasons })
error-ldap-no-anonymous = das Verzeichnis erlaubt kein anonymes Nachschlagen ({ $reason })
error-ldap-bind-failed = das Konto, das Personen nachschlägt, konnte sich nicht anmelden ({ $reason })
error-ldap-search = Suche nach { $name } unter { $base }
error-ldap-group-search = Suche nach den Gruppen von { $name } unter { $base }
error-ldap-several = { $count } Einträge im Verzeichnis passen zu { $name }: Grenzen Sie den Benutzerfilter ein
error-ldap-password-check = das Verzeichnis wollte das Passwort von { $name } nicht prüfen ({ $reason })
error-ldap-wrong-password = Das Verzeichnis kennt { $name } mit diesem Passwort nicht
error-oauth-client-id = Geben Sie die Client-ID an, die der Anbieter für Hoard erstellt hat
error-oauth-addresses = Die Autorisierungs- und die Token-Adresse sollten mit https:// beginnen
error-oauth-userinfo-address = Die User-Info-Adresse sollte mit https:// beginnen
error-oauth-username-claim = Geben Sie an, welcher Claim den Namen der Person enthält
error-oauth-secret-again = Geben Sie das Client-Secret erneut ein: Das gespeicherte wird nur an die Token-Adresse gesendet, für die es angegeben wurde
error-oauth-issuer = Der Issuer sollte mit https:// beginnen
error-oauth-discovery-unreachable = { $url } ist nicht erreichbar: { $reason }
error-oauth-discovery-status = { $url } antwortete { $status }
error-oauth-discovery-bad = { $url } ist kein Discovery-Dokument
error-oauth-discovery-incomplete = { $url } nennt keine Autorisierungs- und Token-Adresse
error-signin-no-group = { $name } ist in keiner Gruppe, die Hoard verwenden darf
error-signin-other-source = Es gibt bereits ein Hoard-Konto namens { $name }, das sich auf andere Weise anmeldet
error-signin-other-subject = Es gibt bereits ein Hoard-Konto namens { $name } für eine andere Person beim Anbieter
error-signin-no-account = { $name } hat noch kein Hoard-Konto: Ein Admin kann es unter Einstellungen → Konto hinzufügen
error-signin-bad-name = Dieser Name kann kein Hoard-Konto sein (bis zu 64 Zeichen)
error-oauth-off = Die Anmeldung mit OAuth ist nicht aktiviert
error-oauth-bad-authorization-address = die Autorisierungsadresse ist ungültig
error-oauth-refused = Der Anbieter hat Sie nicht angemeldet: { $reason }
error-oauth-no-code = Der Anbieter hat Sie ohne Anmeldung zurückgeschickt
error-oauth-expired = Diese Anmeldung hat zu lange gedauert oder wurde woanders gestartet: Versuchen Sie es erneut
error-oauth-no-claim = Der Anbieter hat nicht mitgeteilt, wer Sie sind (kein Claim { $claim })
error-oauth-no-sub = Der Anbieter hat nicht mitgeteilt, wer Sie sind (kein Claim sub oder id)
error-oauth-token-unreachable = die Token-Adresse des Anbieters ist nicht erreichbar
error-oauth-token-refused = Der Anbieter wollte die Anmeldung nicht abschließen: { $reason }
error-oauth-no-access-token = der Anbieter hat kein Zugriffstoken gesendet
error-oauth-userinfo-unreachable = die User-Info-Adresse des Anbieters ist nicht erreichbar
error-oauth-userinfo-refused = Der Anbieter wollte nicht mitteilen, wer sich angemeldet hat ({ $status })
error-oauth-userinfo-not-json = die User-Info des Anbieters ist kein JSON
error-format-no-name = name ist leer
error-format-no-files = files ist leer: Geben Sie an, welche Dateien das Format liest, etwa ["*.json"]
error-format-empty-selector = leer
error-format-unopened-bracket = eine schließende Klammer ohne öffnende
error-format-unclosed-bracket = eine Klammer ist nicht geschlossen
error-format-empty-step-dots = ein leerer Schritt (zwei Punkte hintereinander oder ein Punkt am Anfang oder Ende)
error-format-unclosed-except = !( ist nicht geschlossen
error-format-unclosed-filter = [ ist nicht geschlossen
error-format-after-filters = { $rest } nach Schlüssel und Filtern von { $step }
error-format-except-needs-glob = !(…) steht nach einem Schlüssel mit * darin
error-format-except-not-glob = !(…) steht nach einem Schlüssel mit * darin, nicht nach { $key }
error-format-empty-step = ein leerer Schritt
error-format-not-a-filter = [{ $filter }] ist kein Filter: Verwenden Sie [], [key=value|other], [array], [string], [number], [bool] oder [object]
error-format-filter-needs-value = [{ $filter }] braucht einen Schlüssel und einen Wert
error-store-search-empty = nichts zu suchen; geben Sie den Namen des Pakets ein
error-update-hook-not-set = legen Sie zuerst eine Adresse fest, die aufgerufen werden soll
error-upload-stalled = der Upload von { $path } hängt; senden Sie diesen Teil erneut
error-upload-busy = { $path } wird bereits geschrieben
error-upload-incomplete =
    { $count ->
        [one] { $count } Datei ist unvollständig, z. B. { $files }
       *[other] { $count } Dateien sind unvollständig, z. B. { $files }
    }
error-upload-no-files-listed = keine Dateien
error-upload-not-part = { $path } gehört nicht zu Upload { $id }, oder der Upload ist geschlossen
error-bad-path = ungültiger Pfad
error-upload-part-offset = { $offset } ist nicht der Anfang eines Teils von { $path }
error-upload-interrupted = Upload unterbrochen: { $reason }
error-upload-part-long = Teil { $part } von { $path } ist länger als { $len } Bytes
error-upload-part-short = Teil { $part } von { $path }: { $got } Bytes erhalten, { $len } erwartet
error-upload-file-long = { $path } ist größer als angegeben
error-upload-file-short = { $path }: { $got } Bytes erhalten, { $len } erwartet
error-upload-complete = der Upload ist bereits abgeschlossen
error-already-imported = bereits importiert als „{ $name }“

## Servertexte

server-store-place-readme = README
server-store-place-folders = Ordnernamen
server-store-why-linked = Im Paket verlinkt ({ $place })
server-store-why-same-title = Gleicher Titel wie im Paket ({ $place })
server-store-why-by = Von { $author }, im Paket genannt ({ $place })
server-store-why-meshes = Etwa so viele Meshes wie das Paket ({ $count })
server-store-why-textures = Etwa so viele Texturen wie das Paket ({ $count })
server-chatgpt-revoke-failed = Hier abgemeldet, aber OpenAI konnte nicht benachrichtigt werden. Um sicherzugehen, trennen Sie Hoard in ChatGPT unter Einstellungen → Sicherheit und Anmeldung → Mit ChatGPT anmelden.
server-chatgpt-signed-in-title = Bei ChatGPT angemeldet
server-chatgpt-signed-in = Hoard ist angemeldet. Das KI-Tagging kann jetzt Ihr ChatGPT-Abo nutzen.
server-chatgpt-signed-in-as = Hoard ist als { $email } angemeldet. Das KI-Tagging kann jetzt Ihr ChatGPT-Abo nutzen.
server-chatgpt-not-finished-title = Anmeldung nicht abgeschlossen
server-chatgpt-other-computer = Wenn Sie die Anmeldung in Hoard auf einem anderen Computer gestartet haben, kopieren Sie die Adresse dieser Seite aus der Adressleiste und fügen Sie sie dort in Hoards Einstellungen ein.
server-back-to-settings = Zurück zu den Einstellungen
server-issues-pack = Pakete, die nicht hinzugefügt werden konnten
server-issues-pack-blurb = Löschen Sie das Paket und fügen Sie es erneut hinzu, oder öffnen Sie es, um zu sehen, was es enthält.
server-issues-damaged = Gespeicherte Dateien, die beschädigt oder verschwunden sind
server-issues-damaged-blurb = Gefunden von „Dateien auf Schäden prüfen“. Stellen Sie sie aus einem Backup wieder her oder fügen Sie ihr Paket erneut hinzu, und führen Sie die Prüfung dann erneut aus.
server-issues-folder = Überwachte Ordner, die nicht gelesen werden können
server-issues-folder-blurb = Ihre Pakete bleiben unverändert, bis der Ordner wieder gelesen werden kann.
server-issues-missing = Überwachte Pakete, deren Ordner verschwunden ist
server-issues-missing-blurb = Jedes Paket wird 3 Tage lang behalten, falls sein Ordner nur offline ist, und danach entfernt.
server-issues-job = Fehlgeschlagene Hintergrundaufträge
server-issues-job-blurb = Beim erneuten Versuch läuft der Auftrag noch einmal von vorn.
server-issues-file = Dateien, die nicht geöffnet werden konnten
server-issues-file-blurb = Hoard konnte sie nicht öffnen oder speichern. Ein überwachtes Paket versucht es beim nächsten Scan erneut; eine Datei, die schon in der Bibliothek war, behält ihren vorherigen Stand.
server-issues-unpack = Archive, die nicht entpackt werden konnten
server-issues-unpack-blurb = Jedes bleibt als einfache Datei in seinem Paket, daher ist nichts aus seinem Inhalt in der Bibliothek.
server-issues-read = Dateien, aus denen Hoard nicht schlau wurde
server-issues-read-blurb = Sie sind in der Bibliothek, aber ohne die Angaben, Tags oder Vorschaubilder, die das Lesen liefert.
server-issues-render = Modelle, an denen der Vorschaubild-Renderer gescheitert ist
server-issues-render-blurb = Sie haben kein Vorschaubild. Im Viewer lassen sie sich vielleicht trotzdem öffnen, und dort kann eines gespeichert werden.
server-issues-convert = Modelle, die Blender nicht umwandeln konnte
server-issues-convert-blurb = Sie haben keine GLB-Kopie und können daher nicht in 3D angezeigt werden.
server-issues-unreal = Unreal-Packages, die nicht umgewandelt werden konnten
server-issues-unreal-blurb = Die Unreal-Seite des Pakets zeigt seine übrigen Packages.
server-issues-reformat = Dateien, die eine Umwandlungsregel nicht umwandeln konnte
server-issues-reformat-blurb = Sie bleiben, wie sie waren.
server-issues-materials = Pakete, deren Materialien nicht ermittelt werden konnten
server-issues-materials-blurb = Ihre Texturen sind alle da; sie sind nur nicht zu Materialien gruppiert.
server-issues-material-thumb = Materialien ohne Vorschaubild
server-issues-material-thumb-blurb = Das Material wurde gefunden, aber seine Vorschau konnte nicht gezeichnet werden.
server-issues-store-files = Store-Dateien, die nicht gelesen werden konnten
server-issues-store-files-blurb = Dateien aus dem Store, aus dem ein Paket stammt, etwa ein Megascans-JSON, die es benennen und taggen.
server-issues-scan-again = Erneut scannen
server-issues-retry = Erneut versuchen
server-issues-reanalyze = Paket neu analysieren
server-issues-convert-again = Erneut umwandeln
server-issues-look-again = Erneut nachsehen
server-issues-read-again = Erneut lesen
server-issues-blob-missing = Die gespeicherte Datei ist verschwunden.
server-issues-blob-damaged = Die gespeicherte Datei stimmt nicht mit dem Gespeicherten überein: Sie ist beschädigt.
server-issues-folder-gone = Der Ordner „{ $folder }“ wurde nicht gefunden.
server-job-a-pack = ein Paket
server-job-ingest = { $pack } wird aufgenommen
server-job-retag = { $pack } wird neu getaggt
server-job-llm-tag = { $pack } wird mit KI getaggt
server-job-convert = { $pack } wird mit Blender umgewandelt
server-job-unreal = Unreal-Packages in { $pack } werden umgewandelt
server-job-reformat = Dateien in { $pack } werden nach den Umwandlungsregeln umgewandelt
server-job-scan = Ein überwachter Ordner wird gescannt
server-job-relocate = Dateien werden zwischen Speicherorten verschoben
server-job-looks = Farben und Aussehen werden ermittelt
server-job-sounds = Sounds werden angehört
server-job-embed = Bildinhalte werden gelernt
server-job-hear = Klang der Sounds wird gelernt
server-job-materials = Materialien in { $pack } werden gesucht
server-job-read-files = Die Dateien von { $pack } werden gelesen
server-job-store-lookup = Eine Store-Seite wird nachgeschlagen
server-maintenance-backup = Datenbank sichern
server-maintenance-backup-blurb = Eine konsistente Kopie der Datenbank (Pakete, Tags, Notizen, Sammlungen und Einstellungen) im Ordner „backups“ daneben. Die Dateien der Bibliothek sind nicht enthalten: Sie ändern sich nach dem Speichern nie mehr, daher sichert jede Dateisynchronisierung sie.
server-maintenance-scrub = Dateien auf Schäden prüfen
server-maintenance-scrub-blurb = Liest jede gespeicherte Datei zurück und vergleicht sie mit ihrem Fingerabdruck, um Dateien zu finden, die auf einer Festplatte oder einem NAS verrottet oder verschwunden sind. Dauert bei einer großen Bibliothek eine Weile.
server-maintenance-reanalyze = Alle Pakete neu analysieren
server-maintenance-reanalyze-blurb = Liest jede Datei mit dieser Version von Hoard erneut: Vorschaubilder, Vorschauen, Details und Regel-Tags werden neu erstellt. Ihre eigenen Änderungen bleiben; die KI wird nicht erneut gefragt.
server-maintenance-clear-cache = Zwischengespeicherte Umwandlungen leeren
server-maintenance-clear-cache-blurb = Verwirft die von „Herunterladen als“ aufbewahrten Dateien (sie werden bei der nächsten Anfrage neu erstellt) und was unterbrochene Arbeit hinterlassen hat.
server-maintenance-shrink-thumbs = Vorschaubilder verkleinern
server-maintenance-shrink-thumbs-blurb = Speichert die als PNG aufbewahrten Vorschaubilder und Modellansichten so, wie es Einstellungen → Leistung → Vorschaubilder vorgibt, was sie meist um ein Vielfaches kleiner macht. Bilder mit Transparenz und Pixel-Art bleiben PNG.
server-maintenance-avif-thumbs = Vorschaubilder in AVIF umwandeln
server-maintenance-avif-thumbs-blurb = Wandelt alle Vorschaubilder jetzt um, statt jedes erst beim ersten Anzeigen, wenn Vorschaubilder als AVIF gesendet werden.
server-maintenance-gc = Unbenutzte Dateien löschen
server-maintenance-gc-blurb = Löscht gespeicherte Dateien, auf die nichts mehr verweist, etwa die Dateien endgültig gelöschter Pakete.
server-update-no-address = es ist keine Adresse zum Aufrufen festgelegt
server-update-no-answer = keine Antwort innerhalb von 30 Sekunden

## Einstellungen: Darstellung

appearance-language = Sprache
appearance-language-auto = Wie im Browser ({ $name })
appearance-scheme = Hell oder dunkel
appearance-scheme-system = System
appearance-size-section = Text und Abstände
appearance-text-size = Textgröße
appearance-text-small = Klein
appearance-text-normal = Normal
appearance-text-large = Groß
appearance-density = Abstände
appearance-density-comfortable = Großzügig
appearance-density-compact = Kompakt
appearance-tiles = Kacheln
appearance-tiles-what = Was jede Kachel in der Bibliothek außer ihrem Bild zeigt.
appearance-tile-names = Namen
appearance-tile-details = Art und Größe (Pixel, Länge, Dreiecke)
appearance-tile-badges = Dateityp auf dem Bild
appearance-tile-motion = Animationen abspielen, solange der Zeiger darüber ist
appearance-tile-motion-always = Animationen immer abspielen, nicht nur unter dem Zeiger
appearance-top-bar = Obere Leiste
appearance-top-bar-what = Lass weg, was du nicht brauchst. / springt weiter ins Suchfeld, und { $keys } öffnet weiter die Befehlspalette.
appearance-top-activity = Hintergrundarbeit
appearance-top-theme = Umschalter für hell und dunkel
appearance-top-mic = Mikrofon im Suchfeld, um Klänge zu finden, indem du einen machst
appearance-top-slash = Der /-Hinweis im Suchfeld
appearance-top-palette = Schaltfläche für die Befehlspalette ({ $keys })
appearance-news = Neuigkeiten
appearance-news-what = Neues in Hoard bekommt einen funkelnden goldenen Punkt, bis du es gesehen hast, und die obere Leiste sammelt alles an einem Ort. Was du gesehen hast, folgt deinem Konto.
appearance-news-point-out = Auf Neues hinweisen

## Root

root-unreachable = Der Hoard-Server ist nicht erreichbar: { $error }
root-unreachable-desktop = Hoard antwortet nicht: { $error }
root-desktop-reopen = Dieses Fenster wurde abgemeldet. Schließen Sie Hoard und öffnen Sie es erneut, um weiterzumachen.
root-try-again = Erneut versuchen
root-lazy-connect = App-Verbindung
window-minimize = Minimieren
window-maximize = Maximieren
window-restore = Wiederherstellen
window-close = Schließen

## App: die Bibliothek, ihre obere Leiste und Filter

app-lazy-settings = Einstellungen
app-lazy-archive = Archiv
app-lazy-shortcuts = Tastenkürzel
app-lazy-setup = Einrichtung
app-theme-system = Systemdesign verwenden
app-theme-light = Helles Design
app-theme-dark = Dunkles Design
app-color-red = Rot
app-color-orange = Orange
app-color-yellow = Gelb
app-color-green = Grün
app-color-teal = Türkis
app-color-blue = Blau
app-color-purple = Lila
app-color-pink = Pink
app-color-brown = Braun
app-color-sand = Sand
app-color-gray = Grau
app-color-slate = Schiefer
app-color-black = Schwarz
app-color-white = Weiß
app-color-swatch = { $color }: Assets mit viel { $color }
app-color-any = Beliebige Farbe
app-color-pick = Beliebige Farbe wählen
app-color = Farbe
app-music = Musik
app-tempo = BPM
app-tempo-min = von
app-tempo-max = bis
app-tempo-min-title = Musik mit diesem Tempo oder schneller, in Schlägen pro Minute (halbes oder doppeltes Tempo zählt auch)
app-tempo-max-title = Musik mit diesem Tempo oder langsamer, in Schlägen pro Minute
app-key = Tonart
app-key-any = Beliebige Tonart
app-key-major = Dur
app-key-minor = Moll
app-key-mix = Und Tonarten, die dazu passen
app-key-mix-title = Die Paralleltonart in Dur oder Moll und die Tonarten eine Quinte darüber und darunter, die DJs damit mischen
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Schreib das Geräusch so, wie du es buchstabieren würdest: ähnlich klingende Sounds kommen zuerst
app-imitation-record = Mach das Geräusch ins Mikrofon: ähnlich klingende Sounds kommen zuerst
app-imitation-stop = Aufnahme beenden
app-imitation-heard = Dein Geräusch
app-imitation-mic-failed = Aufnahme vom Mikrofon nicht möglich: { $error }
app-retry = Erneut versuchen
app-undo = Rückgängig
app-search-failed = Suche fehlgeschlagen: { $error }
app-unreachable = Der Server ist nicht erreichbar: { $error }
app-unreachable-desktop = Hoard antwortet nicht: { $error }
app-forget-project-title = { $name } vergessen?
app-forget-project-body = Hoard zeigt es nicht mehr dort an, wo seine { $count } Assets verwendet werden, bis der nächste hoard sync es wieder meldet. Im Projekt selbst ändert sich nichts.
app-forget-project-confirm = Vergessen
app-forget-project-failed = { $name } konnte nicht vergessen werden: { $error }
app-forget-project-label = Projekt { $name } vergessen
app-forget-project = Dieses Projekt vergessen
app-save-name-like = wie { $name }
app-save-name-favorites = Favoriten
app-search-saved = „{ $name }“ gespeichert: Sie finden es unter Filter → Meine
app-search-updated = „{ $name }“ aktualisiert
app-rank-alike = Am ähnlichsten
app-rank-color = Am meisten von der Farbe
app-rank-match = Beste Treffer
app-rank-imitation = Klingt am ähnlichsten
app-sort = Sortieren
app-sort-name = Name
app-sort-created = Erstellt
app-sort-updated = Aktualisiert
app-sort-size = Größe
app-sort-rating = Bewertung
app-sort-newest-first = Neueste zuerst
app-sort-oldest-first = Älteste zuerst
app-sort-a-to-z = A bis Z
app-sort-z-to-a = Z bis A
app-sort-largest-first = Größte zuerst
app-sort-smallest-first = Kleinste zuerst
app-sort-best-rated-first = Beste Bewertung zuerst
app-sort-lowest-rated-first = Schlechteste Bewertung zuerst
app-collection-added =
    { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    } zu { $name } hinzugefügt
app-collection-add-failed = Hinzufügen zur Sammlung fehlgeschlagen: { $error }
app-collection-removed = { $count } aus { $name } entfernt
app-collection-remove-failed = Entfernen aus der Sammlung fehlgeschlagen: { $error }
app-ns-subject = Motiv
app-ns-style = Stil
app-ns-theme = Thema
app-ns-material = Material
app-ns-map = PBR-Map
app-ns-color = Farbe
app-ns-mood = Stimmung
app-ns-genre = Genre
app-ns-license = Lizenz
app-ns-res = Auflösung
app-ns-duration = Länge
app-ns-poly = Polygonzahl
app-ns-format = Format
app-ns-weight = Schriftstärke
app-ns-tech = Technisch
app-ns-blender = Blender-Version
app-ns-depth = Bittiefe
app-ns-ucs = Sound-Kategorie (UCS)
app-ns-bpm = Tempo
app-ns-root = Grundton
app-ns-level = Lautheit
app-ns-ue = Unreal-Klasse
app-ns-engine = Engine
app-ns-extracted = Wiederhergestellte Quelle
app-ns-script = Schriftsystem
app-bulk-failed =
    { $count ->
        [one] Dieses Asset konnte
       *[other] Diese { $count } Assets konnten
    } nicht geändert werden: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] { $count } Tag kopiert
       *[other] { $count } Tags kopiert
    }
app-tags-copy-none = { $name } hat keine Tags zum Kopieren
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Keine Tags kopiert. Mit C auf einem Asset kopierst du seine Tags.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [one] Es hat diese Tags schon
       *[other] Alle { $count } haben diese Tags schon
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [one]
            { $count ->
                [one] { $count } Tag hinzugefügt
               *[other] { $count } Tags hinzugefügt
            }
       *[other]
            { $count ->
                [one] { $count } Tag zu { $assets } Assets hinzugefügt
               *[other] { $count } Tags zu { $assets } Assets hinzugefügt
            }
    }
app-bulk-untagged =
    { $tag } von { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    } entfernt
app-bulk-tagged =
    { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    } mit { $tag } getaggt
app-bulk-review = Als geprüft markieren
app-bulk-review-title = Die ausgewählten Assets als geprüft markieren: Regeln und die KI lassen ihre Tags danach in Ruhe
app-bulk-unreview = Als ungeprüft markieren
app-bulk-unreview-title = Die Geprüft-Markierung von den ausgewählten Assets entfernen
app-bulk-reviewed =
    Als geprüft markiert: { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    }
app-bulk-unreviewed =
    Geprüft-Markierung entfernt bei { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    }
app-bulk-accept = Vorschläge behalten
app-bulk-accept-title = Alle Tags, die Regeln und die KI auf den ausgewählten Assets gesetzt haben, als eigene behalten
app-bulk-accepted =
    Vorgeschlagene Tags behalten bei { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    }
app-credits-failed = Die Credits konnten nicht erstellt werden: { $error }
app-archived-assets =
    { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    } archiviert
app-archived-one = { $name } archiviert
app-open-failed = Dieses Asset konnte nicht geöffnet werden: { $error }
app-unnamed-pack = Paket
app-unnamed-collection = Sammlung
app-unnamed-asset = Asset
app-home-version = Hoard { $version } – zurück zur Bibliothek
app-home = Zurück zur Bibliothek
app-search-label = Assets durchsuchen
app-search-placeholder = Namen, Pfade, Tags durchsuchen
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = Fels or Stein: eins von beiden · -Moos: weglassen · "trockenes Moos": diese Wörter zusammen · pack:Wald: im Namen eines Pakets
app-alt-leave-out = Alt-Klick, um es wegzulassen
app-views = Ansichten
app-library = Bibliothek
app-packs = Pakete
app-packs-working = { $count } in Arbeit
app-update-title = Hoard { $version } ist erschienen (dieser Server läuft mit { $current }). Laden Sie das neue Image oder Binary und starten Sie neu; die Datenbank aktualisiert sich selbst.
app-update = Update
app-theme = Design
app-sign-in = Anmelden
app-archive-title = Archiv: { $contents }
app-archive = Archiv
app-settings = Einstellungen
app-palette-title = Befehlspalette ({ $keys })
app-import-folder = Ordner importieren
app-add-files = Dateien hinzufügen
app-add-files-short = Hinzufügen
app-split-label = Wie dieser Ordner importiert werden soll
app-split-before = { "" }
app-split-after = { " " }enthält { $count } Ordner oder Archive.
app-split-each = Jeden als eigenes Paket importieren ({ $count })
app-split-one = Als ein Paket importieren
app-cancel = Abbrechen
app-filters = Filter
app-done = Fertig
app-yours = Meine
app-favorites = Favoriten
app-needs-review = Zu prüfen
app-needs-review-title = Nur Assets, die noch niemand geprüft hat (gilt zusammen mit den anderen Filtern)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Bewertet
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } Stern oder mehr
       *[other] { $count } Sterne oder mehr
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Erneut klicken, um alle Bewertungen zu zeigen
app-forget-search-label = Gespeicherte Suche { $name } vergessen
app-forget-search = Diese Suche vergessen
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Bibliothek mit dieser Suche öffnen
app-default-search-on-title = Die Bibliothek öffnet sich mit dieser Suche. Klicken, um das zu beenden.
app-default-search-label = Bibliothek mit { $name } öffnen
app-rename-search = Umbenennen
app-rename-search-label = Neuer Name für { $name }
app-projects-title = Projekte, die per hoard sync von hier synchronisieren, mit dem Stand ihrer letzten Synchronisierung
app-projects = Projekte
app-project-synced = Synchronisiert { $when }
app-project-synced-by = Synchronisiert { $when } von { $who }
app-collections = Sammlungen
app-no-filters = Keine Filter für diese Suche.
app-kind = Art
app-facets-more = { $count } weitere
app-facets-fewer = weniger
app-searching = Suche läuft…
app-searching-live = Suche läuft
app-asset-count =
    { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    }
app-clear-all = Alle entfernen
app-save-name-label = Name für diese Suche
app-save-name-placeholder = Name
app-save = Speichern
app-save-search-title = Diese Suche unter Filter → Meine behalten
app-save-search = Suche speichern
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = „{ $name }“ aktualisieren
app-update-search-title = Die aktuelle Auswahl in dieser gespeicherten Suche behalten
app-save-search-new = Als neu speichern
app-select-title = Mehrere Assets auswählen, um sie gemeinsam zu taggen, zu kategorisieren, zu sammeln, herunterzuladen oder zu archivieren (X oder Ctrl-Klick)
app-select = Auswählen
app-tile-size = Kachelgröße
app-supporting-title = glTF-Buffer, Materialbibliotheken, die Maps von Materialien, Materialien, die nur ein Modell trägt, und die Store-Dateien, die Modelle beschreiben. Sie kommen mit, wenn Sie herunterladen, was sie verwendet. Die Seiten eines Spritesheets und seine Sprites, die das Paket auch als einzelne Bilder hat, werden mit dem Spritesheet gezeigt. Ein Asset, das das Paket in mehreren Formaten hat, wird einmal gezeigt, im Format, das Hoard am besten liest.
app-supporting-hidden =
    { $count } { $count ->
        [one] Begleitdatei
       *[other] Begleitdateien
    } ausgeblendet
app-supporting-show = Anzeigen
app-supporting-shown = Mit Begleitdateien
app-supporting-hide = Ausblenden
app-folders = Ordner
app-folders-title = Das Paket so durchsehen, wie es auf der Festplatte lag, Ordner für Ordner
app-folders-label = Ordner des Pakets
app-welcome-title = Legen Sie ein Asset-Paket irgendwo auf dieser Seite ab
app-welcome-before = Zips, Tarballs, Ordner oder einzelne Dateien – alles funktioniert. Hoard entpackt sie, speichert jede Datei einmal anhand ihres Hashs, liest Größen, Längen und Polygonzahlen aus und taggt alles, damit Sie nach Dingen wie{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }oder{ " " }
app-welcome-after = { " " }suchen können.
app-welcome-import-folder = Ordner importieren
app-welcome-choose-files = Dateien auswählen
app-none-similar = Noch sieht nichts anderes { $name } sehr ähnlich.
app-none = Keine Treffer.
app-none-query = Keine Treffer für „{ $query }“.
app-none-imitation = Noch kein gemessener Sound klingt sehr nach „{ $word }“.
app-none-heard = Noch kein gemessener Sound klingt sehr nach deinem Geräusch.
app-none-filters = Keine Treffer mit diesen Filtern.
app-none-query-filters = Keine Treffer für „{ $query }“ mit diesen Filtern.
app-clear-search-filters = Suche und Filter zurücksetzen
app-loading-more = Weitere werden geladen
app-keys-hint = Pfeiltasten bewegen, Enter öffnet, Leertaste spielt einen Sound ab, X oder Ctrl-Klick wählt mehrere aus (auf Touchscreens lange drücken).
app-resize-label = Größe des Detailbereichs ändern
app-resize-title = Ziehen zum Ändern der Größe · Doppelklick für die passende Breite
app-selected-label = Ausgewählte Assets
app-selected = { $count } ausgewählt
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Bearbeiten…
app-edit-selected-title = Tags und Felder aller ändern (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] { $count } Treffer ausgewählt
       *[other] Alle { $count } Treffer ausgewählt
    }
app-pick-assets = Assets auswählen
app-download-zip = Zip herunterladen
app-credits-title = CREDITS.md für die Pakete, aus denen diese stammen: wer sie gemacht hat, ihre Lizenzen und die gewünschten Nennungen
app-credits = Credits
app-prefer-sources-title = Unreal-SoundWaves und -Texturen, deren Originaldatei Hoard wiederhergestellt hat
app-prefer-sources = Wiederhergestellte WAV/PNG statt .uasset ({ $count })
bag-clear = Auswahl aufheben
bag-glb-heading = Modelle als GLB
bag-glb-text = Jedes Modell als .glb-Datei, mit Y nach oben stehend und nach vorn gerichtet, seine Texturen als eigene Dateien in einem Ordner textures, den sich die Modelle teilen. Alles andere kommt so hinein, wie es ist.
bag-glb-download = GLB-Zip herunterladen
bag-plain-heading = Dateien, wie sie sind
bag-plain-text = Jede Datei, wie sie importiert wurde, mit allem, was sie braucht, in einem Ordner pro Paket.
bag-remove = { $name } aus der Auswahl nehmen
bag-more = und { $count } weitere
app-bulk-tag-label = Ein Tag, das den ausgewählten Assets hinzugefügt oder entfernt werden soll
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Dieses Tag jedem ausgewählten Asset hinzufügen
app-bulk-tag-add = Hinzufügen
app-bulk-tag-remove-title = Dieses Tag von jedem ausgewählten Asset entfernen
app-bulk-tag-remove = Entfernen
app-remove-from-collection = Aus { $name } entfernen
app-unfavorite = Kein Favorit mehr
app-favorite = Favorit
app-select-all = Alle { $count } auswählen
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] { $count } Treffer auswählen
       *[other] Alle { $count } Treffer auswählen
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] { $count } Asset bearbeiten
       *[other] { $count } Assets bearbeiten
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] { $count } Treffer bearbeiten
       *[other] Alle { $count } Treffer bearbeiten
    }
bulk-edit-close = Schließen
bulk-edit-tags = Tags
bulk-edit-tag-add = Allen einen Tag hinzufügen
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } von { $count }
bulk-edit-untick = Alle abwählen
bulk-edit-untick-title = Alle Tags entfernen, dann die zu behaltenden auswählen
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = Die am häufigsten verwendeten Tags darunter
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Gemischt, bleibt unverändert
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Bleibt unverändert
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Bei allen geleert
bulk-edit-reset = Zurücksetzen
bulk-edit-all-note = Das ändert alle Treffer, auch noch nicht geladene, und lässt sich nicht in einem Schritt rückgängig machen.
bulk-edit-save =
    { $count ->
        [one] { $count } Änderung speichern
       *[other] { $count } Änderungen speichern
    }
bulk-edit-nothing = Nichts geändert
bulk-edit-saved =
    { $count ->
        [one] { $count } Asset aktualisiert
       *[other] { $count } Assets aktualisiert
    }
app-archive-selected-title = Nicht mehr in der Suche, bis sie im Archiv wiederhergestellt oder endgültig gelöscht werden
app-drop-items = { $count } Elemente ablegen, um sie hinzuzufügen
app-drop = Zum Hinzufügen ablegen
app-drop-hint = Jede Datei und jeder Ordner der obersten Ebene wird zu einem eigenen Paket. Gigabytes sind kein Problem; große Dateien setzen fort, wenn die Verbindung abbricht.

## Befehlspalette (CommandPalette.vue, mit ihren Befehlen aus App.vue)

palette-group-go-to = Gehe zu
palette-group-actions = Aktionen
palette-group-theme = Design
palette-group-news = Neuigkeiten
palette-news-show = Neuigkeiten anzeigen
palette-news-mark-all = Alles Neue als gesehen markieren
palette-news-keywords = neu neuigkeiten neuheiten änderungen funktionen version gesehen new news
palette-group-kinds = Arten
palette-group-packs = Pakete
palette-group-collections = Sammlungen
palette-group-projects = Projekte
palette-group-saved = Gespeicherte Suchen
palette-group-assets = Assets
palette-hint-current = aktuell
palette-library = Bibliothek
palette-library-keywords = assets raster start startseite grid home
palette-favorites = Favoriten
palette-favorites-keywords = markiert sterne gefällt merkliste starred stars liked
palette-packs = Pakete
palette-archive = Archiv
palette-archive-keywords = papierkorb gelöscht wiederherstellen trash recycle bin deleted restore
palette-stats = Bibliotheksstatistik
palette-stats-keywords = statistik zahlen größe speicherplatz anzahl statistics numbers size disk space count
palette-copies = Beinahe-Kopien
palette-copies-keywords = duplikate gleich ähnlich skaliert konvertiert verschwendeter speicherplatz duplicates same similar resized converted wasted space dedupe
palette-search = Suchen
palette-add-files = Dateien hinzufügen…
palette-add-files-keywords = hochladen importieren upload import
palette-import-folder = Ordner importieren…
palette-import-folder-keywords = hochladen paket upload pack
palette-select-all = Alle { $count } angezeigten auswählen
palette-select-all-keywords = markieren auswählen check
palette-select-several = Mehrere auswählen…
palette-select-several-keywords = mehrfach massenbearbeitung tag archivieren bulk multiple archive
palette-archive-selected = Die { $count } ausgewählten archivieren
palette-archive-selected-keywords = löschen entfernen papierkorb delete remove trash
palette-edit-selected = Die { $count } ausgewählten bearbeiten
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = Massenänderung Tags Kategorie Lizenz Bewertung
palette-paste-tags =
    { $count ->
        [one] Kopierte Tags in das ausgewählte Asset einfügen
       *[other] Kopierte Tags in die { $count } ausgewählten einfügen
    }
palette-clear-selection = Auswahl aufheben
palette-clear-filters = Filter zurücksetzen
palette-clear-filters-keywords = zurücksetzen löschen reset
palette-hide-supporting = Begleitdateien ausblenden
palette-show-supporting = Begleitdateien anzeigen
palette-supporting-keywords = normal maps roughness masken masks
palette-account = Konto und Zugriffstokens
palette-account-keywords = passwort anmeldung benutzer token api password login user
palette-sign-out = Abmelden ({ $name })
palette-sign-out-keywords = abmelden ausloggen log out logout
palette-sign-in = Anmelden
palette-sign-in-keywords = anmelden einloggen konto passwort log in login account password
palette-settings = Einstellungen
palette-settings-keywords = blender ffmpeg llm ki tagging unreal admin werkzeuge optionen ai tools preferences
palette-shortcuts = Tastenkürzel
palette-shortcuts-keywords = hilfe tasten tastatur help keys
palette-theme-keywords = design darstellung modus theme appearance mode
palette-theme-variant-keywords = darstellung farbe appearance color
palette-light-theme = Helles Design: { $name }
palette-dark-theme = Dunkles Design: { $name }
palette-kind-all = Alle Arten (nicht mehr nur { $kind })
palette-kind-only = Nur { $kind }
palette-title = Befehlspalette
palette-placeholder = Assets suchen, zu einem Paket springen, einen Befehl ausführen…
palette-results = Ergebnisse
palette-searching = Suche läuft…
palette-no-match = Keine Treffer für „{ $query }“
palette-foot-move = bewegen
palette-foot-run = ausführen
palette-foot-shortcuts = alle Tastenkürzel

## Tastenkürzel (ShortcutsDialog.vue)

shortcuts-title = Tastenkürzel
shortcuts-close = Schließen
shortcuts-anywhere = Überall
shortcuts-palette = Befehlspalette: suchen, springen, ausführen
shortcuts-search = Suchen
shortcuts-this-list = Diese Liste
shortcuts-escape = Schließen, Auswahl aufheben oder Details schließen
shortcuts-library = Bibliothek
shortcuts-move = Zwischen Kacheln wechseln (oder J und K)
shortcuts-open = Details öffnen
shortcuts-play-sound = Sound abspielen oder stoppen
shortcuts-select = Auswählen oder Auswahl aufheben
shortcuts-favorite = Zu Favoriten hinzufügen oder daraus entfernen
shortcuts-reviewed = Als geprüft markieren oder nicht
shortcuts-tag = Tag hinzufügen
shortcuts-edit = Auswahl bearbeiten
shortcuts-copy-tags = Seine Tags kopieren
shortcuts-paste-tags = Kopierte Tags in die Auswahl einfügen
shortcuts-key-click = Klick
shortcuts-select-range = Bereich auswählen
shortcuts-select-all = Alles Angezeigte auswählen
shortcuts-sound-player = Sound-Player
shortcuts-play-pause = Abspielen oder pausieren
shortcuts-skip = Zurück- oder vorspringen
shortcuts-start-end = Anfang oder Ende
shortcuts-image-inspector = Bildinspektor
shortcuts-zoom = Vergrößern oder verkleinern
shortcuts-fit = An Fenster anpassen
shortcuts-actual-pixels = Originalpixel
shortcuts-tile = 3×3 kacheln, um die Nahtstellen zu prüfen
shortcuts-background = Nächster Hintergrund
shortcuts-channel = Einen Kanal anzeigen

## KindChips.vue

kinds-group = Asset-Arten

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = von einer Person hinzugefügt
tag-source-rule = vom Offline-Tagger
tag-source-meta = aus der Datei gemessen
tag-source-ai = von der KI vorgeschlagen
tag-remove = Tag { $tag } entfernen
tag-accept = Tag { $tag } behalten
tag-accept-title = Diesen Vorschlag als eigenes Tag behalten
tag-reject-title = Entfernen; der Tagger setzt es nicht wieder
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Neues Tag
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Meintest du
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = war { $alias }

## ActivityIndicator.vue

activity-label-idle = Hintergrundaktivität: untätig
activity-label-running = Hintergrundaktivität: { $running } laufen
activity-label-queued = Hintergrundaktivität: { $running } laufen, { $queued } in der Warteschlange
activity-title = Hintergrundaktivität
activity-unreachable = Der Server ist nicht erreichbar
activity-unreachable-desktop = Hoard antwortet nicht
activity-empty = Es läuft nichts. Importe, Tagging, Umwandlungen und Zip-Downloads erscheinen hier, während sie arbeiten.
activity-queued = in der Warteschlange
activity-stop = „{ $what }“ anhalten
activity-stop-tagging = Gesamtes KI-Tagging anhalten ({ $count })
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } Dreiecke
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = { $name } abspielen
tile-pause = { $name } pausieren
tile-favorite = { $name } als Favorit
tile-in-favorites = In Ihren Favoriten
tile-add-favorite = Zu Ihren Favoriten hinzufügen (F)
tile-more-like = Mehr wie { $name }
tile-more-like-this = Mehr davon
tile-select = { $name } auswählen

## CollectionPicker.vue

collections-add = Zu Sammlung hinzufügen
collections-menu = Sammlungen
collections-find-label = Sammlung suchen oder erstellen
collections-find-placeholder = Sammlung suchen oder neue benennen
collections-create = Erstellen
collections-empty = Noch keine Sammlungen. Geben Sie einen Namen ein, um eine anzulegen.

## FolderPicker.vue

folders-list = Ordner
folders-close = Schließen
folders-browse = Durchsuchen…
folders-up-title = Einen Ordner nach oben
folders-up = Nach oben
folders-this-computer = Dieser Computer
folders-none = Keine Ordner hier.

## LocationForm.vue

locations-mode-question = Was soll Hoard mit diesem Ordner tun?
locations-mode-managed = Beanspruchen: Hoard bewahrt meine Pakete hier auf
locations-mode-managed-hint = Pakete, die Sie hochladen oder importieren, werden hierher kopiert, und Hoard hält sie in Ordnung. Der Ordner gehört jetzt Hoard, lassen Sie seinen Inhalt also unberührt.
locations-mode-watched = Beobachten: Meine Pakete liegen schon hier
locations-mode-watched-hint = Jeder Ordner darin wird zu einem Paket. Hoard liest die Dateien dort, wo sie liegen, und ändert, verschiebt oder löscht sie nie; neue Ordner werden erkannt, sobald sie erscheinen.
locations-folder-desktop = Ordner
locations-folder-placeholder = Der Ordner mit Ihren Paketen
locations-folder-hint-managed = Hoard erstellt ihn, falls er nicht existiert. In Docker ist das ein Ordner, den Sie in den Container eingehängt haben.
locations-folder-hint-managed-desktop = Hoard erstellt ihn, falls er nicht existiert.
locations-folder-hint-watched = Hängen Sie den Ordner in Docker zuerst in den Container ein (schreibgeschützt genügt) und wählen Sie ihn dann hier aus.
locations-folder-hint-watched-desktop = Hoard liest ihn nur: Darin wird nichts geändert.
locations-name = Name
locations-name-placeholder-managed = Bibliothek
locations-name-placeholder-watched = Meine Pakete
locations-files = Dateien, die hierher kommen
locations-catch-all = Auch alles, was kein anderer Speicherort aufnimmt
locations-kinds-hint = Die Dateien eines neuen Pakets kommen an den Speicherort, bei dem ihre Art angehakt ist. Der Rest kommt an den Auffangort.
locations-files-all = Vorerst alle. Fügen Sie später in den Einstellungen weitere Speicherorte hinzu, um manche Arten (etwa Musik) auf ein anderes Laufwerk zu schicken.
locations-first-watched-note = Zum Hochladen braucht es eine Bibliothek, die Hoard verwaltet. Sie können später eine unter Einstellungen → Speicherorte hinzufügen.
locations-checking = Ordner wird geprüft…
locations-add = Speicherort hinzufügen
locations-cancel = Abbrechen

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Die Speicherorte konnten nicht geladen werden
locations-takes-and-rest = { $kinds } und alles andere
locations-takes-rest = Alles, was die anderen nicht aufnehmen
locations-takes-everything = Alles
locations-takes-nothing = Noch nichts
locations-save-failed = Der Speicherort konnte nicht gespeichert werden
locations-unwatch-title = { $name } nicht mehr überwachen?
locations-unwatch-body =
    { $count ->
        [one] Sein { $count } Paket wird
       *[other] Seine { $count } Pakete werden
    } samt Tags und Notizen aus Hoard entfernt. Die Dateien im Ordner bleiben unberührt.
locations-unwatch-body-empty = Die Dateien im Ordner bleiben unberührt.
locations-unwatch = Nicht mehr überwachen
locations-remove-title = { $name } entfernen?
locations-remove-body = Alle Dateien darin werden zuerst nach Art an die anderen Speicherorte verschoben; er wird entfernt, sobald er leer ist. Es geht nichts verloren.
locations-remove-body-only = Falls er Dateien enthält, fügen Sie zuerst einen anderen Speicherort hinzu, an den sie verschoben werden können.
locations-remove = Entfernen
locations-removed = { $name } entfernt
locations-emptying = Die Dateien werden aus { $name } verschoben; er wird entfernt, sobald das abgeschlossen ist
locations-offline-title = { $name } ist nicht erreichbar
locations-offline-body = Seine Dateien können nicht verschoben werden, solange er offline ist. Trotzdem entfernen? Alle Dateien, die noch darauf liegen, werden als fehlend angezeigt.
locations-remove-anyway = Trotzdem entfernen
locations-remove-failed = { $name } konnte nicht entfernt werden
locations-rebalancing = Gespeicherte Dateien werden an die Speicherorte verschoben, die ihre Art aufnehmen
locations-rebalance-failed = Die Dateien konnten nicht verschoben werden
locations-scanned-never = noch nicht gescannt
locations-scanned-now = gerade gescannt
locations-scanned-minutes = vor { $mins } Min. gescannt
locations-scanned-hours = vor { $hours } Std. gescannt
locations-scanned-date = gescannt am { $date }
locations-scan-failed = { $name } konnte nicht gescannt werden
locations-watching = { $name } wird überwacht
locations-added = { $name } hinzugefügt
locations-loading = Wird geladen…
locations-managed-title = Bibliotheken, die Hoard verwaltet
locations-watched-title = Überwachte Ordner
locations-managed-hint = Uploads und Importe werden hierher kopiert. Jeder Speicherort nimmt die angehakten Arten auf; der Auffangort nimmt den Rest.
locations-watched-hint = Ordner mit Paketen, die dort indiziert werden, wo sie liegen. Hoard ändert die Dateien darin nie und sucht alle 15 Minuten nach neuen und entfernten Ordnern.
locations-managed-empty = Noch keine, daher gibt es keinen Platz für Uploads. Fügen Sie unten eine hinzu.
locations-reachable = Erreichbar
locations-unreachable-now = Gerade nicht erreichbar
locations-catch-all-tag = Auffangort
locations-packs =
    { $count ->
        [one] { $count } Paket
       *[other] { $count } Pakete
    }
locations-unreachable = nicht erreichbar
locations-scanning = Wird gescannt…
locations-scan = Jetzt scannen
locations-edit = Bearbeiten
locations-catch-all-locked = Um das zu ändern, machen Sie einen anderen Speicherort zum Auffangort
locations-edit-hint-before = Gilt für Dateien, die ab jetzt hinzukommen. Um bereits Gespeichertes zu verschieben, verwenden Sie
locations-edit-hint-after = weiter unten.
locations-save = Speichern
locations-rebalance = Gespeicherte Dateien entsprechend verschieben
locations-rebalance-hint = Nachdem Sie geändert haben, was ein Speicherort aufnimmt: Bereits gespeicherte Dateien werden an den Speicherort verschoben, zu dem ihre Art jetzt gehört.
locations-add-title = Speicherort hinzufügen
locations-add-hint = Ein weiteres Laufwerk für bestimmte Dateiarten oder ein Ordner mit Paketen, die Sie schon haben.
locations-kind-question = Was für ein Speicherort ist es?
locations-kind-folder = Ein Ordner
locations-kind-folder-hint = Auf einem Laufwerk oder einer NAS-Freigabe, die in den Container eingehängt ist, oder einer, den Dropbox, Google Drive, OneDrive oder iCloud Drive synchron hält. Hoard kann dort Dateien ablegen oder Pakete beobachten, die Sie schon haben.
locations-kind-folder-hint-desktop = Auf diesem Computer, einem anderen Laufwerk oder einer NAS-Freigabe, oder einer, den Dropbox, Google Drive, OneDrive oder iCloud Drive synchron hält. Hoard kann dort Dateien ablegen oder Pakete beobachten, die Sie schon haben.
locations-kind-s3-hint = Ein Bucket in Amazon S3 oder einem ähnlichen Dienst: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard legt dort Dateien ab.
locations-s3-title = S3-Speicher
locations-s3-hint = Ein Speicherort unter s3://bucket-name ist ein Bucket in diesem Dienst: AWS oder ein beliebiger S3-kompatibler (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage auf einem NAS).
locations-bucket = Bucket
locations-bucket-hint = Der Name eines Buckets, den es schon gibt. Die Schlüssel oben müssen ihn auflisten, lesen und beschreiben können.
locations-s3-endpoint = Adresse
locations-s3-endpoint-placeholder = Leer für AWS; z. B. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Region
locations-s3-access-key = Zugriffsschlüssel-ID
locations-s3-secret-key = Geheimer Zugriffsschlüssel
locations-s3-headers = Zusätzliche Header
locations-s3-headers-placeholder = Name=value, einer pro Zeile (etwa für Cloudflare Access)
locations-s3-kept = Gespeichert (zum Ersetzen tippen)
locations-s3-secrets = Die Schlüssel und Header werden mit den Einstellungen des Servers gespeichert und nie wieder angezeigt.
locations-s3-secrets-desktop = Die Schlüssel und Header werden im Schlüsselbund dieses Computers gespeichert und nie wieder angezeigt.
locations-s3-saved = S3-Speicher gespeichert
locations-s3-save-failed = Der S3-Speicher konnte nicht gespeichert werden
locations-add-more = Speicherort hinzufügen…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Die Probleme konnten nicht geladen werden
issues-started =
    { $count ->
        [one] Erneut gestartet.
       *[other] { $count } Vorgänge erneut gestartet.
    }
issues-retry-failed = Erneuter Versuch fehlgeschlagen
issues-dismiss-failed = Verwerfen fehlgeschlagen
issues-dismiss-all-title = Alle { $count } verwerfen?
issues-dismiss-all-body = „{ $title }“ wird geleert. Was erneut schiefgeht, wird wieder aufgeführt.
issues-dismiss-all = Alle verwerfen
issues-just-now = gerade eben
issues-minutes-ago = vor { $mins } Min.
issues-hours-ago = vor { $hours } Std.
issues-loading = Wird geladen…
issues-none = Es ist nichts schiefgegangen.
issues-action-all = { $action } (alle)
issues-open-asset = Öffnen
issues-open-pack = Paket anzeigen
issues-open-locations = Speicherorte
issues-where-before = in
issues-where-after = { "" }
issues-times = { $count }-mal
issues-starting = Wird gestartet…
issues-dismiss-named = { $title } verwerfen
issues-dismiss = Verwerfen
issues-show-fewer = Weniger anzeigen
issues-show-all = Alle { $count } anzeigen
issues-not-listed = { $count } weitere sind nicht aufgeführt.

## archive.ts

archive-anyway = Trotzdem archivieren
archive-failed = Archivieren fehlgeschlagen: { $error }
archive-undo = Rückgängig
archive-restore-failed = Wiederherstellen fehlgeschlagen: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } Paket
       *[other] { $packs } Pakete
    } und { $assets ->
        [one] { $assets } Asset
       *[other] { $assets } Assets
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } Paket
       *[other] { $packs } Pakete
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } Asset
       *[other] { $assets } Assets
    }
archive-count-nothing = nichts

## ArchiveView.vue

archive-load-failed = Das Archiv konnte nicht geladen werden: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = { $what } wiederhergestellt
archive-purged = { $what } endgültig gelöscht
archive-purge-failed = Löschen fehlgeschlagen: { $error }
archive-purge-title = { $name } endgültig löschen?
archive-purge-pack-body =
    { $count ->
        [one] Sein { $count } Asset und
       *[other] Seine { $count } Assets und
    } { $size } an Dateien werden ebenfalls gelöscht, außer Dateien, die ein anderes Paket mitbenutzt. Das kann nicht rückgängig gemacht werden.
archive-purge = Endgültig löschen
archive-purge-asset-body = Seine Datei wird freigegeben, sofern kein anderes Asset sie mitbenutzt. Das kann nicht rückgängig gemacht werden.
archive-empty-title = Archiv leeren?
archive-empty-body = { $what } werden endgültig gelöscht und ihre Dateien freigegeben. Das kann nicht rückgängig gemacht werden.
archive-empty = Archiv leeren
archive-title = Archiv
archive-intro = Archivierte Pakete und Assets erscheinen nicht in der Suche, den Zählungen und der Tag-Liste. Stellen Sie sie wieder her oder löschen Sie sie endgültig, um ihren Speicherplatz freizugeben.
archive-loading = Wird geladen…
archive-is-empty = Das Archiv ist leer
archive-is-empty-hint = Archivieren Sie ein Paket oder ein Asset, und es wartet hier, bis Sie es wiederherstellen oder endgültig löschen.
archive-packs = Pakete
archive-assets = Assets
archive-asset-count =
    { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    }
archive-archived = archiviert { $when }
archive-from = aus { $pack }
archive-restore = Wiederherstellen
archive-show-more = Mehr anzeigen (noch { $count })

## mine.ts

projects-favorite-add-failed = Hinzufügen zu Ihren Favoriten fehlgeschlagen: { $error }
projects-favorite-remove-failed = Entfernen aus Ihren Favoriten fehlgeschlagen: { $error }
projects-save-search-failed = Die Suche konnte nicht gespeichert werden: { $error }
projects-forget-search-failed = { $name } konnte nicht entfernt werden: { $error }
projects-change-search-failed = { $name } konnte nicht geändert werden: { $error }

## projects.ts

projects-list-and = { $items } und { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } weitere
projects-other =
    { $count ->
        [one] { $count } weiteres Projekt
       *[other] { $count } weitere Projekte
    }
projects-usage-title-one = { $name } verwendet das
projects-usage-title = { $count } Projekte verwenden das
projects-usage-body-one = Verwendet in { $list }: Der nächste hoard sync entfernt es dort.
projects-usage-body = Verwendet in { $list }: Der jeweils nächste hoard sync entfernt es dort.

## uploader.ts

upload-network-error = Netzwerkfehler
upload-gave-up = { $what }: { $error } nach { $attempts } Versuchen
upload-failed-plain = fehlgeschlagen
upload-already-imported = bereits importiert
upload-part-failed = Upload fehlgeschlagen ({ $status })
upload-batch = Stapel aus { $count } Dateien
upload-batch-failed = Stapel fehlgeschlagen ({ $status })
upload-finish = Upload abschließen

## UploadQueue.vue

upload-eta-seconds = noch { $s } s
upload-eta-minutes = noch { $min } min
upload-eta-hours = noch { $h } h
upload-queue = Importe
upload-importing = { $count } Pakete werden importiert
upload-bytes-of = { $sent } von { $total }
upload-rate = { $size }/s
upload-waiting = Wartet
upload-unpacking = Hochgeladen, wird entpackt
upload-already = Bereits in der Bibliothek
upload-stopped = Angehalten
upload-files-of = { $done } von { $total } Dateien
upload-resumed = fortgesetzt, { $size } waren schon da
upload-skipped = Genau diese Dateien wurden bereits als „{ $name }“ importiert und daher übersprungen.
upload-error = { $error }. Ihr Fortschritt bleibt erhalten: Versuchen Sie es erneut, oder legen Sie dieselben Dateien später noch einmal ab.
upload-retry = Erneut versuchen
# $name is the pack the library has already that these files go into.
upload-update = „{ $name }“ aktualisieren
upload-copy = Als Kopie importieren
# $name is the pack the library has already that the upload goes into.
upload-into = Geht in „{ $name }“, das bereits in der Bibliothek ist: Neue Dateien werden hinzugefügt, geänderte ersetzt.
upload-own-pack = Als eigenes Paket anlegen
upload-dismiss = Verwerfen
upload-cancel = Abbrechen
upload-minimize = Minimieren
upload-expand = Importe anzeigen
upload-pill = { $count } werden importiert · { $percent } %
upload-pill-stopped = Angehalten: { $count }
upload-interrupted = Unterbrochen
upload-resume = Fortsetzen
upload-resume-all = Alle fortsetzen
upload-resume-hint-folder = Wählen Sie denselben Ordner erneut oder legen Sie ihn auf der Seite ab, um dort weiterzumachen, wo er angehalten hat.
upload-resume-hint-file = Wählen Sie dieselbe Datei erneut oder legen Sie sie auf der Seite ab, um dort weiterzumachen, wo sie angehalten hat.
upload-not-same = Das sind nicht dieselben Dateien, daher wurde nichts fortgesetzt.
upload-permission-denied = Der Browser durfte die Dateien nicht erneut lesen. Wählen Sie sie stattdessen aus.

## PacksView.vue

packs-rename-failed = Umbenennen fehlgeschlagen: { $error }
packs-collection-deleted = Sammlung { $name } gelöscht
packs-delete-failed = Löschen fehlgeschlagen: { $error }
packs-status-uploading = Wird hochgeladen (bisher empfangene Dateien)
packs-status-ingesting = Wird entpackt und getaggt
packs-status-analyzing = LLM taggt
packs-status-ready = Bereit
packs-status-failed = Fehlgeschlagen
packs-retag-failed = Neues Tagging konnte nicht gestartet werden: { $error }
packs-read-files-failed = Die Dateien des Pakets konnten nicht gelesen werden: { $error }
packs-context-load-failed = Die Store-Details des Pakets konnten nicht geladen werden: { $error }
packs-undo-store-failed = Die Store-Seite konnte nicht entfernt werden: { $error }
packs-context-save-failed = Die Store-Details konnten nicht gespeichert werden: { $error }
packs-unreal-load-failed = Der Status der Unreal-Umwandlung konnte nicht geladen werden: { $error }
packs-unreal-discard-title = Originale verwerfen?
packs-unreal-discard-body = Die originalen .uasset-Dateien in { $name } werden nach der Umwandlung gelöscht. Daten darin, die nur Unreal versteht, etwa Material-Graphen, gehen endgültig verloren.
packs-unreal-discard-confirm = Umwandeln und verwerfen
packs-unreal-convert-failed = Die Umwandlung konnte nicht gestartet werden: { $error }
packs-unreal-fact-converted = { $done } aus Unreal umgewandelt
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal-Package
       *[other] { $left } Unreal-Packages
    }
packs-unreal-fact-both = { $done } aus Unreal umgewandelt, { $left } noch Packages
packs-thumbs-canceled = Abgebrochen nach { $done } von { $total }
packs-thumbs-drew =
    { $count ->
        [one] { $count } Modell gezeichnet
       *[other] { $count } Modelle gezeichnet
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } Vorschaubild ersetzt.
       *[other] { $lead }: { $improved } Vorschaubilder ersetzt.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } Vorschaubild ersetzt, { $failed } konnten nicht gezeichnet werden.
       *[other] { $lead }: { $improved } Vorschaubilder ersetzt, { $failed } konnten nicht gezeichnet werden.
    }
packs-archived = { $name } und seine { $count } Assets archiviert
packs-blobs-bad = { $count } Dateien haben ihre letzte Integritätsprüfung nicht bestanden.
packs-collections = Sammlungen
packs-browse-named = { $name } durchsuchen
packs-collection-name = Name der Sammlung
packs-save = Speichern
packs-cancel = Abbrechen
packs-assets =
    { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    }
packs-updated = Aktualisiert { $date }
packs-browse = Durchsuchen
packs-collection-download = Herunterladen (.zip)
packs-collection-credits = Credits
packs-collection-credits-title = Wer die Pakete gemacht hat, aus denen diese Assets stammen, ihre Lizenzen und die gewünschten Nennungen
packs-rename = Umbenennen
packs-delete = Löschen
packs-packs = Pakete
packs-empty = Noch keine Pakete. Legen Sie irgendwo auf dieser Seite ein Zip oder einen Ordner ab, um eines hinzuzufügen.
packs-find = Paket suchen
packs-find-placeholder = Paket nach Name, Store, Lizenz oder Tag suchen
packs-sort = Reihenfolge
packs-sort-new = Neueste zuerst
packs-sort-old = Älteste zuerst
packs-sort-size = Größte zuerst
packs-sort-assets = Meiste Assets zuerst
packs-sort-count = Anzahl Assets
packs-sort-smallest = Kleinste zuerst
packs-sort-fewest = Wenigste Assets zuerst
packs-with-kind = Pakete mit
packs-state = Status
packs-state-any = Jeder Status
packs-state-working = In Arbeit
packs-state-failed = Fehlgeschlagen
packs-state-untagged = Noch nicht beschrieben
packs-state-unlicensed = Keine Lizenz vermerkt
packs-store = Store
packs-store-any = Jeder Store
packs-store-from = Von { $store }
packs-store-none = Keine Store-Seite
packs-shown-of = { $shown } von { $total }
packs-clear = Zurücksetzen
packs-no-match = Keine passenden Pakete.
packs-clear-all = Suche und Filter zurücksetzen
packs-pack-name = Name des Pakets
packs-thumbs-drawing = Vorschaubilder werden in diesem Browser gezeichnet…
packs-thumbs-drawing-count = Vorschaubilder werden in diesem Browser gezeichnet: { $done } von { $total }…
packs-thumbs-stopped = Angehalten: { $error }
packs-thumbs-none = Jedes Modell hier hat bereits ein Vorschaubild, das mit allen Texturen gezeichnet wurde.
packs-dismiss = Schließen
packs-license = Lizenz: { $license }
packs-license-tier = Lizenz: { $license } ({ $tier })
packs-license-not-found = nicht gefunden
packs-from-before = Von
packs-from-after = { "" }
packs-added = Hinzugefügt { $date }
pack-panel-label = Über dieses Paket
pack-panel-hide = Details des Pakets ausblenden
pack-panel-kinds = Was drin ist
pack-panel-all = Alle Pakete
packs-download = Paket herunterladen (.zip)
packs-store-details = Store-Details
packs-more-named = Weitere Aktionen für { $name }
packs-more = Mehr
packs-rename-more = Umbenennen…
packs-retag-rules = Offline-Tagger erneut ausführen
packs-retag-llm = LLM-Tagger erneut ausführen
packs-make-thumbs = Vorschaubilder in diesem Browser erstellen
packs-make-thumbs-title = Jedes Modell nacheinander unsichtbar in einem 3D-Viewer öffnen und sein Bild als Vorschaubild verwenden, wo es besser ist als das des Servers
packs-make-thumbs-title-desktop = Jedes Modell nacheinander unsichtbar in einem 3D-Viewer öffnen und sein Bild als Vorschaubild verwenden, wo es besser ist als das von Hoard erstellte
packs-read-files = Die Dateien des Pakets erneut lesen
packs-credits = Credits herunterladen
packs-credits-title = CREDITS.md: wer das Paket gemacht hat, seine Lizenz und die gewünschte Nennung
packs-unreal-open = Unreal-Inhalte umwandeln…
packs-archive = Paket archivieren
packs-unreal-intro = Hoard liest Unreal-Packages selbst und speichert ihren Inhalt als gewöhnliche Dateien: Texturen als PNG oder EXR, Sounds als WAV, Materialien als JSON. Die umgewandelte Datei tritt an die Stelle des Packages und behält dessen Tags, Verknüpfungen und Sammlungen.
packs-unreal-converted =
    { $count ->
       *[other] umgewandelt
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] umgewandelt, { $kept } davon mit behaltenem Original
    }
packs-unreal-pending =
    { $count ->
       *[other] noch nicht angesehen
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] warten auf den Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] ohne gängiges Format
    }
packs-unreal-failed =
    { $count ->
       *[other] fehlgeschlagen
    }
packs-unreal-reasons = Warum Packages übrig bleiben
packs-unreal-editor-before = Meshes und Animationen brauchen den Unreal Editor. Führen Sie auf einem Rechner mit installiertem Unreal Folgendes aus:
packs-unreal-editor-command = hoard unreal-worker --editor <Pfad zu UnrealEditor-Cmd>
packs-unreal-editor-after = ; der Befehl exportiert sie als FBX und lädt die Ergebnisse hierher hoch.
packs-unreal-discard = Die originalen .uasset-Dateien nach der Umwandlung verwerfen
packs-unreal-discard-hint = Gibt Speicherplatz frei. Daten, die nur Unreal versteht, etwa Material-Graphen, gehen verloren.
packs-unreal-drop = Packages ohne gängiges Format löschen
packs-unreal-drop-hint = Blueprints, Levels und andere Assets, die nur die Engine versteht.
packs-unreal-convert = Umwandeln
packs-context-hint-before = Woher das Paket stammt und was der Verkäufer darüber sagt. Der LLM-Tagger liest das bei jeder Datei mit, daher hilft ihm ein Store-Text wie „120 Eis- und Feuerzauber“ beim Benennen von
packs-context-hint-after = .
packs-store-auto = Automatisch gefunden: Das Paket nennt diese Seite selbst.
packs-store-not-this = Nicht diese
packs-lookup-label = In den Stores suchen nach
packs-lookup-placeholder = Der Name des Pakets im Store
packs-lookup-searching = Suche läuft…
packs-lookup-search = Stores durchsuchen
packs-lookup-hide = Ausblenden
packs-lookup-clues = Aus den Dateien des Pakets: { $clues }.
packs-lookup-also = Auch gesucht: { $queries }.
packs-lookup-quoted = „{ $query }“
packs-lookup-nothing = Auf Fab, itch.io, Kenney und OpenGameArt sieht nichts danach aus. Versuchen Sie weniger oder andere Wörter.
packs-lookup-use = Verwenden
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Jetzt
packs-review-store = Aus dem Store
packs-review-same = Gleich
packs-review-read-page = Wird beim Speichern von der Seite gelesen
packs-review-take =
    { $count ->
        [one] { $count } Feld übernehmen
       *[other] { $count } Felder übernehmen
    }
packs-lookup-searched = Gesucht am { $date }.
packs-lookup-again = Erneut suchen
packs-lookup-again-after = .
packs-lookup-unreachable = { $stores } war diesmal nicht erreichbar.
packs-lookup-also-look = Suchen Sie auch auf
packs-lookup-links-and = { " " }und{ " " }
packs-lookup-browser-only = (dort lässt sich nur im Browser suchen) und fügen Sie die Adresse der Seite unten ein.
packs-lookup-open = In Stores suchen…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Verkäufer
packs-seller-placeholder = Wer es gemacht hat
packs-license-label = Lizenz
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Lizenzstufe
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] { $count } Asset hat eine eigene Lizenz: stattdessen die des Pakets verwenden
       *[other] { $count } Assets haben eine eigene Lizenz: stattdessen die des Pakets verwenden
    }
packs-store-page = Store-Seite
packs-store-text = Beschreibung aus dem Store
packs-store-text-placeholder = Fügen Sie den Text des Angebots ein, oder lassen Sie das Feld leer, um ihn von der Store-Seite zu holen.
packs-store-text-fetch = Beim Speichern wird die Beschreibung von der Store-Seite geholt.
packs-readme = Im Paket gefundene README
packs-save-retag = Speichern und LLM-Tagger erneut ausführen

## AssetDetail.vue

asset-lazy-3d-viewer = 3D-Viewer
asset-lazy-text = Text
asset-lazy-text-viewer = Textanzeige
asset-lazy-image-viewer = Bildanzeige
asset-lazy-material-preview = Materialvorschau
asset-lazy-material-viewer = Materialanzeige
asset-lazy-sprite-player = Sprite-Player
asset-lazy-sheet-view = Spritesheet-Anzeige
asset-rel-texture = Textur
asset-rel-buffer = Geometriedaten
asset-rel-material = Materialdatei
asset-rel-animation = Animation
asset-rel-library = verknüpfte Datei
asset-rel-guess = gehört wohl dazu
asset-rel-sidecar = Store-Details
asset-rel-variant = Variante
asset-rel-sprite = Sprite
asset-rel-sample = Sample
asset-role-map = Map: { $role }
asset-maps-count =
    { $count ->
        [one] { $count } Map
       *[other] { $count } Maps
    }
asset-needs = Benötigt
asset-animations = Animationen
asset-sprites = Sprites
asset-used-by = Verwendet von
asset-loose-sprites = Auch als einzelne Dateien
asset-in-sheet = In einem Spritesheet
asset-other-formats = Andere Formate
asset-shown-as = In der Bibliothek gezeigt als
asset-conversion-failed = Umwandlung fehlgeschlagen
asset-as-converting = Wird umgewandelt…
asset-as-converting-percent = Wird umgewandelt… { $percent } %
asset-as-queued = In der Warteschlange…
asset-download = Herunterladen
asset-as-label = Herunterladen als
asset-as-original = { $format } (Originalformat)
asset-as-pack-files = Die Dateien des Pakets
asset-as-size = Größe
asset-as-full-size = Volle Größe
asset-as-up-to = Bis { $size } px
asset-as-hint = Wählen Sie eine Größe oder ein anderes Format
asset-meta-size = Größe
asset-meta-length = Länge
asset-meta-sample-rate = Abtastrate
asset-meta-channels = Kanäle
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bittiefe
asset-meta-bits = { $bits } Bit
asset-meta-level = Pegel
asset-meta-level-value = Spitze { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = Spitze { $peak } dBFS, RMS { $rms } dBFS, übersteuert
asset-meta-loop = Schleife
asset-meta-loop-value = { $start } bis { $end }
asset-meta-playback = Wiedergabe
asset-meta-one-shot = One-Shot
asset-meta-envelope = Hüllkurve
asset-meta-envelope-times = Anstieg { $attack }, Ausklang { $tail }
asset-meta-envelope-hit = Schlag
asset-meta-envelope-swell = anschwellend
asset-meta-envelope-sustained = gehalten
asset-meta-tone = Klang
asset-meta-tone-value = Helligkeit { $khz } kHz, Rauschen { $noise } %
asset-meta-pitch = Tonhöhe
asset-meta-pitch-rises =
    { $octaves ->
        [one] steigt um { $octaves } Oktave
       *[other] steigt um { $octaves } Oktaven
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] fällt um { $octaves } Oktave
       *[other] fällt um { $octaves } Oktaven
    }
asset-meta-loops-cleanly = Nahtlos loopbar
asset-meta-tempo = Tempo
asset-meta-tempo-heard = etwa { $bpm } BPM, nach Gehör
asset-meta-tempo-heard-or = etwa { $bpm } BPM (oder { $alt }), nach Gehör
asset-meta-key = Tonart
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, nach Gehör
music-key-major = { $note }-Dur
music-key-minor = { $note }-Moll
asset-meta-root-note = Grundton
asset-meta-cue-markers = Cue-Marker
asset-meta-library = Bibliothek
asset-meta-keywords = Schlüsselwörter
asset-meta-title = Titel
asset-meta-artist = Künstler
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Generator
asset-meta-everything-with = Alles mit dem Tag { $tag } anzeigen
asset-meta-saved-with = Gespeichert mit
asset-meta-color-mode = Farbmodus
asset-meta-layers = Ebenen
asset-meta-range = Bereich
asset-meta-hdr = HDR (Vorschau ist tonemapped)
asset-meta-format = Format
asset-meta-large-document = Großes Dokument (PSB)
asset-meta-ai = KI
asset-meta-ai-made = Mit generativer KI erstellt
asset-meta-ai-edited = Mit generativer KI bearbeitet
asset-meta-ai-how-metadata = Metadaten
asset-meta-ai-how-generator = Generator-Einstellungen
asset-meta-ai-how-iptc = IPTC Digital Source Type
asset-meta-ai-says = { $made } (laut { $how })
asset-meta-ai-by-says = { $made } ({ $by }, laut { $how })
asset-meta-provenance = Herkunft
asset-meta-c2pa = Hat C2PA Content Credentials
asset-meta-compression = Kompression
asset-meta-note = Hinweis
asset-meta-maps = Maps
asset-meta-sizes =
    { $count ->
        [one] Größe
       *[other] Größen
    }
asset-meta-triangles = Dreiecke
asset-meta-vertices = Vertices
asset-meta-meshes = Meshes
asset-meta-materials = Materialien
asset-meta-animations = Animationen
asset-meta-skins = Skins
asset-meta-unreal-asset = Unreal-Asset
asset-meta-unreal-saved-with = { $class }, gespeichert mit UE { $engine }
asset-meta-package = Package
asset-meta-cooked = Cooked (Spiel-Build): Vorschauen und Quellen sind entfernt
asset-meta-tiling = Kachelung
asset-meta-seamless = Nahtlos auf beiden Achsen
asset-meta-analysis-error = Diese Datei konnte nicht vollständig gelesen werden: { $error }
asset-copy-blocked = Kopieren nicht möglich: Der Browser hat es blockiert
asset-save-error = Speichern fehlgeschlagen: { $error }
asset-tag-removed = { $tag } entfernt
asset-tag-restored = { $tag } wieder gesetzt
asset-tags-added =
    Hinzugefügt: { $count ->
        [one] { $count } Tag
       *[other] { $count } Tags
    }
asset-tags-accepted =
    { $count ->
        [one] { $count } vorgeschlagenes Tag behalten
       *[other] { $count } vorgeschlagene Tags behalten
    }
asset-reviewed = Geprüft
asset-reviewed-on-title = Geprüft: Regeln und die KI lassen die Tags in Ruhe. Klicken, um die Markierung zu entfernen (R)
asset-reviewed-off-title = Als geprüft markieren, damit Regeln und die KI die Tags in Ruhe lassen (R)
asset-reviewed-on = Als geprüft markiert
asset-reviewed-off = Geprüft-Markierung entfernt
# $name is an asset's name.
asset-reviewed-on-named = { $name } als geprüft markiert
asset-reviewed-off-named = Geprüft-Markierung von { $name } entfernt
asset-undo = Rückgängig
asset-play-error = Diese Datei kann nicht abgespielt werden: { $error }
asset-copied-hash = Hash kopiert
asset-copied-toml = TOML kopiert
asset-copied-id = ID kopiert
asset-copied-link = Link kopiert
asset-origin-before = Diese Datei war
asset-origin-class-engine = , ein { $class }, gespeichert mit UE { $engine }.
asset-origin-class = , ein { $class }.
asset-origin-engine = , gespeichert mit UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Der Unreal Editor hat sie am { $date } exportiert.
asset-origin-editor-notes = Der Unreal Editor hat sie am { $date } exportiert ({ $notes }).
asset-origin-hoard = Hoard hat sie am { $date } aus dem Package ausgelesen.
asset-origin-hoard-notes = Hoard hat sie am { $date } aus dem Package ausgelesen ({ $notes }).
asset-origin-download = Original-.{ $ext } herunterladen
asset-origin-discarded = Das originale Package wurde nach der Umwandlung verworfen.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Verlauf
history-loading = Wird geladen…
history-none = Hier wurde noch nichts geändert.
history-load-failed = Der Verlauf konnte nicht geladen werden: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = KI
history-by-rule = Eine Regel
history-by-someone = Jemand
history-by-app = { $who } über eine App
history-by-assistant = { $who } über einen Assistenten
history-field-description = Beschreibung
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Tag { $tag } hinzugefügt
history-tag-removed = Tag { $tag } entfernt
history-tag-kept = Vorgeschlagenes Tag { $tag } übernommen
history-tag-changed = Tag { $tag } geändert
history-reviewed = Als geprüft markiert
history-unreviewed = Als ungeprüft markiert
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: auf { $to } gesetzt
history-cleared = { $field }: geleert (war { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Rückgängig
history-revert-title = Den Stand vor dieser Änderung wiederherstellen
history-reverted = Rückgängig gemacht: { $change }
history-revert-failed = Das ließ sich nicht rückgängig machen: { $error }
asset-details = Asset-Details
asset-in-pack = in
asset-previous = Vorheriges Asset
asset-next = Nächstes Asset
asset-favorite = Favorit
asset-favorite-on-title = In Ihren Favoriten (F entfernt es)
asset-favorite-off-title = Zu Ihren Favoriten hinzufügen (F)
asset-share = Teilen
asset-share-title = Einen Link zu diesem Asset teilen (wer ihn öffnet, meldet sich zuerst bei Hoard an)
asset-layout-panel = In einer Seitenleiste anzeigen
asset-layout-page = Ganzseitig anzeigen
asset-layout-page-title = Ganzseitig anzeigen: eine große Vorschau neben den Details
asset-close = Details schließen
asset-rename-title = Zum Umbenennen klicken
asset-name = Name
asset-views-alt = Das Modell aus vier Richtungen
asset-inspect-aria = { $name } im Vollbild untersuchen
asset-inspect-title = Untersuchen: zoomen, kacheln, Kanäle
asset-playback-position = Wiedergabeposition
asset-playback-value = { $current } von { $duration }
asset-pause = Pause
asset-play = Abspielen
asset-waveform-drawing = Wellenform wird gezeichnet…
asset-waveform-skipped = Keine Wellenform für so große Dateien
asset-play-animation = Als Animation abspielen
asset-show-views = Die Vier-Ansichten-Übersicht zeigen, die der Tagger gesehen hat
asset-palette = Farben im Bild
asset-swatch-title = { $hex }, { $percent } % des Bildes. Assets mit dieser Farbe suchen
asset-swatch-aria = Assets mit { $hex } suchen, { $percent } % dieses Bildes
asset-more-like-this = Mehr davon
asset-desc-label = Ihre Beschreibung
asset-desc-placeholder = Was es ist, wofür Sie es verwenden würden…
asset-save = Speichern
asset-cancel = Abbrechen
asset-desc-family = Wird für alle { $count } Varianten gespeichert
asset-desc-edit = Beschreibung bearbeiten
asset-desc-write = Beschreibung schreiben
asset-desc-ai = Von KI geschrieben
asset-desc-file = Aus der Datei
asset-desc-store-before = Von der
asset-desc-store-page = { $store }-Seite
asset-desc-store-page-unnamed = Store-Seite
asset-desc-store-after = (das ganze Paket)
asset-less = Weniger
asset-more = Mehr
asset-tags = Tags
asset-tag-add-label = Tag hinzufügen
asset-tag-add-placeholder = Tag hinzufügen, z. B. mood:tense
asset-apply-family = Änderungen auf alle { $count } Varianten anwenden
asset-tags-accept-all =
    { $count ->
        [one] { $count } Vorschlag behalten
       *[other] Alle { $count } Vorschläge behalten
    }
asset-tags-accept-all-title = Alle von Regeln und KI vorgeschlagenen Tags als eigene behalten
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Abgelehnt:
asset-tag-restore-title = { $tag } wieder setzen
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = Ähnliche haben:
asset-tag-suggest-title = { $tag } hinzufügen
asset-details-heading = Details
asset-field-category = Kategorie
asset-field-license = Lizenz
asset-field-source = Quelle
asset-field-rating = Bewertung
# Shown in place of an empty value, to fill it in.
asset-field-add = Hinzufügen
asset-field-new = Feld hinzufügen
asset-field-name = Feldname
asset-field-value = Wert
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = aus dem Paket
asset-field-license-placeholder = Etwa CC0 oder CC-BY 4.0
asset-field-remove-placeholder = Leer lassen zum Entfernen
# $field is a field's name, like "License".
asset-field-edit-title = { $field } bearbeiten
asset-field-saved = { $field } gespeichert
asset-field-cleared = { $field } geleert
asset-rate =
    { $count ->
        [one] { $count } Stern
       *[other] { $count } Sterne
    }
asset-rated =
    Mit { $count ->
        [one] { $count } Stern
       *[other] { $count } Sternen
    } bewertet
asset-rating-clear-title = Erneut klicken, um die Bewertung zu entfernen
asset-rating-cleared = Bewertung entfernt
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Erneut klicken für { $count } Stern
       *[other] Erneut klicken für { $count } Sterne
    }
asset-collections = Sammlungen
asset-collection-remove = Aus { $name } entfernen
asset-collection-add-another = Zu weiterer hinzufügen
asset-collection-add = Zu Sammlung hinzufügen
asset-used-in = Verwendet in
asset-used-in-title = Zeigen, was { $name } hat
asset-used-in-synced = synchronisiert { $when }
asset-maps = Maps
asset-show-more = { $count } weitere anzeigen
asset-materials-heading =
    { $count ->
        [one] Material
       *[other] Materialien
    }
asset-material-maps-aria = { $name }: Maps
asset-variants = Varianten ({ $count } Takes)
asset-fit-animations = Passende Animationen ({ $count })
asset-fit-characters = Figuren, zu denen sie passt ({ $count })
asset-copies = Identische Kopien
asset-copies-note = Gleiche Bytes werden nur einmal gespeichert; diese Pakete enthalten die Datei ebenfalls.
asset-near-copies = Beinahe-Kopien
asset-near-copies-note = Dasselbe in anderer Größe, Abtastrate oder einem anderen Format. Die beste Kopie ist markiert.
asset-near-copies-best = Dasselbe in anderer Größe, Abtastrate oder einem anderen Format. Diese hier ist die beste Kopie.
asset-near-best = beste
asset-unreal-converted = Aus Unreal umgewandelt
asset-rule-converted = Von einer Regel umgewandelt
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Am { $date } von einer Umwandlungsregel als { $format } erstellt aus
asset-rule-kept-until = Das Original wird bis { $date } aufbewahrt.
asset-rule-rebuilt = Das FLAC enthält alles, was im WAV war, sodass das WAV jederzeit genau so heruntergeladen oder zurückgeholt werden kann, wie es war.
asset-rule-put-back = Original wiederherstellen
asset-rule-put-back-title = { $file } wiederherstellen?
asset-rule-put-back-body = Die umgewandelte Datei wird gelöscht, und das Original tritt mit seinen Tags, Sammlungen und Verknüpfungen an ihre Stelle. Die Umwandlungsregeln lassen es von da an in Ruhe.
asset-rule-put-back-done = { $file } ist wieder da
asset-rule-let-go = Das Original wurde gelöscht, als die in den Einstellungen festgelegte Aufbewahrungszeit für Originale abgelaufen war.
asset-unreal-package = Aus dem Unreal-Package
asset-ue-needs-editor-before = Wartet auf den Unreal Editor: { $reason }. Führen Sie auf einem Rechner mit installiertem Unreal
asset-ue-needs-editor-after = aus, dann kommt es als FBX zurück.
asset-ue-editor-path = Pfad zu UnrealEditor-Cmd
asset-ue-unsupported = Als Package behalten: { $reason }.
asset-ue-failed = Umwandlung fehlgeschlagen: { $reason }
asset-recovered = Die originale .{ $ext }-Datei war in diesem Package gespeichert, und Hoard hat sie herausgeholt.
asset-recovered-download = .{ $ext } herunterladen
asset-ue-meshes-before = Meshes und Animationen sind in Unreals eigenem Format gespeichert. Wandeln Sie das Paket um (Paketseite, Mehr) und führen Sie dann
asset-ue-meshes-after = neben einer Unreal-Editor-Installation aus, um FBX zu erhalten.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } Asset-Registry-Tag
       *[other] { $count } Asset-Registry-Tags
    }
asset-file = Datei
asset-bytes = Bytes
asset-source = Quelle
asset-path = Pfad
asset-path-open-folder = Zeigen, was sonst in { $folder } liegt
asset-download-maps =
    { $count ->
        [one] Die { $count } Map herunterladen
       *[other] Alle { $count } Maps herunterladen
    }
asset-download-linked =
    { $count ->
        [one] Mit { $count } verknüpften Datei herunterladen
       *[other] Mit { $count } verknüpften Dateien herunterladen
    }
asset-download-only-title = Nur diese Datei, ohne die Dateien, die sie verwendet
asset-download-only = Nur diese Datei
asset-include-animations = Animationen in die .zip aufnehmen
asset-include-character = Figur { $name } mitliefern
asset-copy-toml = hoard.toml-Eintrag kopieren
asset-copy-id = ID kopieren
asset-teach = Hoard diese Datei beibringen
asset-teach-title = Die Datei dieses Stores beschreiben, damit Hoard sie liest (Einstellungen → Store-Dateien)
asset-archive = Archivieren
asset-archive-title = Nicht mehr in der Suche, bis es im Archiv wiederhergestellt oder endgültig gelöscht wird

## ImageViewer.vue

image-channel-red = Rotkanal
image-channel-green = Grünkanal
image-channel-blue = Blaukanal
image-channel-alpha = Alphakanal
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Schachbrett-Hintergrund (K wechselt)
image-bg-black-title = Schwarzer Hintergrund (K wechselt)
image-bg-gray-title = Grauer Hintergrund (K wechselt)
image-bg-white-title = Weißer Hintergrund (K wechselt)
image-bg-checker = Schachbrett-Hintergrund
image-bg-black = schwarzer Hintergrund
image-bg-gray = grauer Hintergrund
image-bg-white = weißer Hintergrund
image-inspect-aria = { $name } untersuchen
image-preview-size = Vorschau { $width }×{ $height }, { $bytes }
image-preview = Vorschau
image-resolution = Auflösung
image-full-title = Das volle Bild mit { $width }×{ $height } laden (F); auch das Hineinzoomen über die Vorschau hinaus lädt es
image-full-loading = Volle Auflösung wird geladen…
image-full-failed = Volle Auflösung nicht geladen: erneut versuchen
image-full = Volle Auflösung
image-zoom = Zoom
image-zoom-out-title = Verkleinern (−)
image-zoom-out = Verkleinern
image-zoom-in-title = Vergrößern (+)
image-zoom-in = Vergrößern
image-fit-title = Einpassen (0)
image-fit = Einpassen
image-actual-title = Originalpixel (1)
image-view = Ansicht
image-tile-title = 3×3 kacheln, um die Nahtstellen zu prüfen (T)
image-tile = Kacheln
image-channels = Kanäle
image-close = Schließen (Esc)
image-failed = Dieses Bild konnte nicht geladen werden.
image-loading = Wird geladen…
image-key-gestures = Scrollen oder Zwei-Finger-Geste zum Zoomen · ziehen zum Verschieben · Doppelklick zum Vergrößern oder Einpassen
image-key-tile = kacheln
image-key-background = Hintergrund
image-key-full = volle Auflösung
image-key-channels = Kanäle

## SpritePlayer.vue

sprite-frame-aria = { $name }, Frame { $frame } von { $count }
sprite-failed = Das lässt sich nicht abspielen.
sprite-loading = Frames werden geladen…
sprite-previous = Vorheriger Frame
sprite-next = Nächster Frame
sprite-pause = Animation pausieren
sprite-play = Animation abspielen
sprite-animation = Animation
sprite-all-frames = Alle Frames
sprite-grid-title = Wie das Sheet in Frames zerlegt wird
sprite-columns = Spalten
sprite-rows = Zeilen
sprite-fps-aria = Frames pro Sekunde
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } Sprite
       *[other] { $count } Sprites
    }
sheet-pages =
    { $count ->
        [one] { $count } Seite
       *[other] { $count } Seiten
    }
sheet-animations =
    { $count ->
        [one] { $count } Animation
       *[other] { $count } Animationen
    }
# The box that narrows the list of sprites by name.
sheet-find = Sprite suchen
# Under the list of sprites when more match than it draws.
sheet-showing = { $shown } von { $total } angezeigt
sheet-no-match = Kein Sprite hat das im Namen.
sheet-failed = Dieses Sheet lässt sich nicht lesen.
sheet-loading = Sheet wird geladen…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoards eigenes Format: die Seiten unverändert, Sprites beliebig gedreht und beschnitten, mit Animationen und der Dauer jedes Frames.
sheet-note-texturepacker = Sprites im Uhrzeigersinn gedreht und beschnitten, mit Animationen nach Namen, so wie PixiJS und Phaser sie lesen.
sheet-note-clockwise = Sprites im Uhrzeigersinn gedreht und beschnitten. Animationen sind darin nicht möglich: Frames richten sich nach ihren nummerierten Namen.
sheet-note-libgdx = Sprites gegen den Uhrzeigersinn gedreht und beschnitten. Animationen sind darin nicht möglich: Frames richten sich nach ihren nummerierten Namen.
sheet-note-godot-frames = Sprites nie gedreht, der weggeschnittene Rand bleibt als Abstand erhalten, mit Animationen und der Dauer jedes Frames.
sheet-note-godot-tileset = Kacheln nie gedreht oder beschnitten, in einem Raster der Kachelgröße; Bilder, die schon in einem Raster liegen, bleiben, wie sie sind.
sheet-note-grid = Jedes Sprite aufrecht auf seiner ganzen Leinwand, in gleich großen Zellen, für alles, was ein Sheet in ein Raster zerlegt.
sheet-note-frames = Jedes Sprite aufrecht auf seiner ganzen Leinwand, jeweils als PNG mit seinem Namen.

## TextViewer.vue, TextPreview.vue und text.ts

text-lang-config = Konfiguration
text-lang-table = Tabelle
text-lang-code = Code
text-lang-script = Skript
text-lang-text = Text
text-copy-blocked = Kopieren nicht möglich: Der Browser hat es blockiert
text-read-aria = { $name } lesen
text-lines =
    { $count ->
        [one] { $count } Zeile
       *[other] { $count } Zeilen
    }
text-show-as = Anzeigen als
text-tree-title = Aufklappbarer Baum (T)
text-tree = Baum
text-pretty-title = Neu eingerückt (P)
text-pretty = Formatiert
text-raw-title = Wie geschrieben (R)
text-raw = Roh
text-document-title = Als Dokument (D)
text-document = Dokument
text-source-title = Das Markdown wie geschrieben (R)
text-source = Quelltext
text-page-title = Wie ein Browser sie aus ihrem Ordner zeigt, von Hoard getrennt (P)
text-page = Seite
text-page-source-title = Die Seite, wie sie geschrieben ist (R)
text-page-aria = { $name } als Seite
text-page-error = Diese Seite konnte nicht geöffnet werden: { $error }
text-fold = Einklappen
text-expand-all-title = Alles aufklappen
text-expand-all = Alle aufklappen
text-collapse-all-title = Alles einklappen
text-collapse-all = Alle einklappen
text-group-text = Text
text-wrap-title = Lange Zeilen umbrechen (W)
text-wrap = Umbrechen
text-copy-title = Den Text kopieren (C)
text-copied = Kopiert
text-copy = Kopieren
text-download-title = Die Datei herunterladen
text-download = Herunterladen
text-close = Schließen (Esc)
text-truncated = Die ersten { $shown } von { $total } werden angezeigt.
text-truncated-json = Die ersten { $shown } von { $total } werden angezeigt (der Baum braucht die ganze Datei).
text-show-all = Alles anzeigen
text-json-error = Dieses JSON lässt sich nicht parsen: { $error }
text-contents-aria = Inhalt von { $name }
text-loading = Wird geladen…
text-load-error = Diese Datei konnte nicht geladen werden: { $error }
text-binary = Diese Datei ist binär, kein Text, daher gibt es hier nichts zu lesen. Laden Sie sie herunter, um sie mit ihrem eigenen Programm zu öffnen.
text-key-tree = Baum
text-key-pretty = formatiert
text-key-raw = roh
text-key-document = Dokument
text-key-source = Quelltext
text-key-page = Seite
text-key-wrap = umbrechen
text-key-copy = kopieren
text-key-find = suchen
text-key-close = schließen
text-preview-aria = { $name } im Vollbild lesen
text-preview-read = Lesen

## JsonTree.vue und JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } Eintrag
       *[other] { $count } Einträge
    }
json-keys =
    { $count ->
        [one] { $count } Schlüssel
       *[other] { $count } Schlüssel
    }
json-fold = Einklappen
json-unfold = Aufklappen
json-unfold-summary = { $summary } aufklappen
json-show-more = { $count } weitere anzeigen
json-left = von noch { $count }
json-picker-more = … { $count } weitere

## 3D-Modellanzeige (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = der 3D-Viewer wurde nicht geladen ({ $error }); ein Neuladen der Seite hilft vielleicht
model-loading-viewer = 3D-Viewer wird geladen…
model-loading = Modell wird geladen…
model-show-failed = Dieses Modell kann nicht angezeigt werden: { $error }
model-preview-failed = Für dieses Modell ist keine Vorschau möglich: { $error }
model-error-converting = der Server wandelt dieses Modell noch um
model-error-converting-desktop = Hoard wandelt dieses Modell noch um
model-error-empty-collada = leere Collada-Datei
model-error-no-loader = Kein Browser-Loader für .{ $ext }
model-dialog = { $name } in 3D ansehen
model-title-stats =
    { $triangles } Dreiecke · { $meshes ->
        [one] { $meshes } Mesh
       *[other] { $meshes } Meshes
    } · { $materials ->
        [one] { $materials } Material
       *[other] { $materials } Materialien
    }
model-triangles =
    { $count ->
        [one] { $count } Dreieck
       *[other] { $count } Dreiecke
    }
model-group-camera = Kamera
model-group-shading = Shading
model-group-background = Hintergrund
model-group-animation = Animation
model-group-view = Ansicht
model-group-show = Anzeigen
model-frame-title = Modell einpassen (F)
model-frame = Einpassen
model-reset-title = Zurück zur Startansicht (R)
model-reset = Zurücksetzen
model-turn-title = Langsam drehen (A)
model-turn = Drehen
model-textures-title = Texturen (T)
model-textures = Texturen
model-full-res-title = Texturen in voller Auflösung (H): jede Texturdatei so, wie sie ist, egal wie groß; aus: leichte Vorschauen mit höchstens 2048 px
model-full-res = Voll
model-light-title = Umgebungslicht (E)
model-light = Licht
model-grid-title = Bodenraster (G)
model-grid = Raster
model-bg-checker = Schachbrett-Hintergrund
model-bg-dark = Dunkler Hintergrund
model-bg-gray = Grauer Hintergrund
model-bg-light = Heller Hintergrund
model-bg-env = Umgebungshintergrund
model-bg-title = { $background } (K wechselt)
model-pause-title = Pause (Leertaste)
model-play-title = Abspielen (Leertaste)
model-pause = Pause
model-play = Abspielen
model-animation-clip = Animationsclip
model-animation = Animation
model-plays-on = Abspielen auf
model-plays-everywhere = Überall verwenden
model-plays-everywhere-title = Alle Animationen mit diesem Skelett auf { $name } abspielen, auch in ihren Vorschaubildern
model-plays-everywhere-done = Animationen mit diesem Skelett laufen jetzt auf { $name }. Ihre Vorschaubilder werden neu gezeichnet.
model-plays-everywhere-failed = Die Figur, auf der sie laufen, konnte nicht geändert werden: { $error }
model-rest-pose = Ruhepose
model-close-title = Schließen (Esc)
model-full-screen = Vollbild
model-open-full-screen = Den 3D-Viewer im Vollbild öffnen
model-menu-fold = Menü einklappen
model-menu-show = Menü anzeigen
model-view-prefix = Ansicht:{ " " }
model-menu-render = Rendern
model-menu-maps = Maps
model-menu-inspect = Untersuchen
model-mode-final = Endgültiges Rendering
model-mode-raw = Rohes Rendering
model-mode-color = Grundfarbe
model-mode-roughness = Rauheit
model-mode-metalness = Metallizität
model-mode-normal = Normal
model-mode-ao = Occlusion
model-mode-matcap = Matcap
model-mode-uv = UV-Prüfmuster
model-mode-geometry = Geometrien
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Drahtgitter (W), über jeder Ansicht
model-wireframe = Drahtgitter
model-info-title = Szeneninformationen (I)
model-info = Szeneninformationen
model-info-close = Szeneninformationen schließen
model-info-close-title = Schließen (I)
model-info-geometries = Geometrien
model-info-materials = Materialien
model-info-textures = Texturen
model-info-triangles = Dreiecke
model-info-vertices = Vertices
model-info-meters = glTF ist in Metern
model-info-no-units = Dieses Format gibt seine Einheiten nicht an
model-info-height = Höhe
model-info-length = Länge
model-info-width = Breite
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } Einheit
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } Einheiten
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Textur nicht gefunden: { $name }
model-missing-some = { $count } Texturen nicht gefunden: { $names }
model-missing-more = { $count } Texturen nicht gefunden: { $names }…
model-missing-count = { $count } Texturen nicht gefunden
model-guessed = Texturen nach Namen zugeordnet; die Datei nennt keine.
model-group-facing = Ausrichtung
model-facing-turn = Viertel drehen
model-facing-turn-title = Das Modell, von oben gesehen, eine Vierteldrehung gegen den Uhrzeigersinn drehen
model-facing-tip = Viertel kippen
model-facing-tip-title = Die Oberseite des Modells eine Vierteldrehung nach vorn kippen, um ein liegendes Modell aufzustellen
model-facing-save = Auf Paket anwenden
model-facing-save-title = Alle Modelle in { $pack } so drehen. Ihre Vorschaubilder werden neu gezeichnet; Downloads bleiben, wie sie erstellt wurden.
model-facing-undo = Rückgängig
model-facing-undo-title = Zurück zur bisherigen Ausrichtung der Modelle des Pakets
model-facing-saved = Alle Modelle in { $pack } sind jetzt so ausgerichtet. Ihre Vorschaubilder werden neu gezeichnet.
model-facing-failed = Die Modelle des Pakets konnten nicht gedreht werden: { $error }
model-keys-mouse = Ziehen zum Kreisen · rechts ziehen zum Verschieben · scrollen zum Zoomen · Doppelklick zum Einpassen
model-key-frame = einpassen
model-key-reset = zurücksetzen
model-key-view = Ansicht
model-key-wireframe = Drahtgitter
model-key-info = Szeneninformationen
model-key-textures = Texturen
model-key-full-res = Texturen in voller Auflösung
model-key-light = Licht
model-key-grid = Raster
model-key-background = Hintergrund
model-key-play = abspielen

## Materialien (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Farbe
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Rauheit
material-role-gloss = Glanz
material-role-metalness = Metallizität
material-role-ao = Ambient Occlusion
material-role-orm = Occlusion, Rauheit, Metall
material-role-mra = Metall, Rauheit, Occlusion
material-role-rma = Rauheit, Metall, Occlusion
material-role-height = Höhe
material-role-bump = Bump
material-role-cavity = Cavity
material-role-specular = Specular
material-role-opacity = Deckkraft
material-role-emissive = Emission
material-role-translucency = Transluzenz
material-role-base-color-name = Farbe
material-role-normal-name = Normal
material-role-normal-dx-name = Normal (DirectX)
material-role-roughness-name = Rauheit
material-role-gloss-name = Glanz
material-role-metalness-name = Metallizität
material-role-ao-name = Ambient Occlusion
material-role-orm-name = Occlusion, Rauheit, Metall
material-role-mra-name = Metall, Rauheit, Occlusion
material-role-rma-name = Rauheit, Metall, Occlusion
material-role-height-name = Höhe
material-role-bump-name = Bump
material-role-cavity-name = Cavity
material-role-specular-name = Specular
material-role-opacity-name = Deckkraft
material-role-emissive-name = Emission
material-role-translucency-name = Transluzenz
material-maps =
    { $count ->
        [one] { $count } Map
       *[other] { $count } Maps
    }
material-dialog = Material { $name } ansehen
material-up-to = bis { $size }
material-repeat-size = { $size } pro Wiederholung
material-shape-sphere = Kugel
material-shape-cube = Würfel
material-shape-plane = Ebene
material-shape-title = { $shape } ({ $key })
material-group-shape = Form
material-group-tiling = Kachelung
material-real-title = Echte Größe: eine Wiederholung ist { $size }, die Kugel hat 2 m Durchmesser (T wechselt)
material-repeat-title = { $n }× wiederholen (T wechselt)
material-real = Echt
material-repeat = { $n }×
material-height-title = Wie weit die Height-Map die Oberfläche verschiebt
material-height = Höhe
material-height-strength = Höhenstärke
material-loading = Material wird geladen…
material-show-failed = Dieses Material kann nicht angezeigt werden: { $error }
material-preview-failed = Für dieses Material ist keine Vorschau möglich: { $error }
material-group-maps = Maps
material-map-show = Map „{ $map }“ anzeigen
material-map-hide = Map „{ $map }“ ausblenden
material-map-unused = Von der Vorschau nicht verwendet
material-open-full-screen = Den Material-Viewer im Vollbild öffnen
material-keys-mouse = Ziehen zum Kreisen · rechts ziehen zum Verschieben · scrollen zum Zoomen · eine Map anklicken, um sie auszuschalten
material-key-shape = Form
material-key-tiling = Kachelung
material-key-turn = drehen

## theme.ts

theme-slate = Schiefer
theme-paper = Papier
theme-linen = Leinen
theme-sage = Salbei
theme-lilac = Flieder
theme-blush = Rosé
theme-graphite = Graphit
theme-obsidian = Obsidian
theme-ember = Glut
theme-moss = Moos
theme-aubergine = Aubergine
theme-oxblood = Ochsenblut
theme-sky = Himmel
theme-mint = Minze
theme-apricot = Aprikose
theme-lemon = Zitrone
theme-midnight = Mitternacht
theme-lagoon = Lagune
theme-neon = Neon
theme-light-slate-blurb = Kühles Blaugrau
theme-light-paper-blurb = Schlichtes neutrales Grau
theme-light-linen-blurb = Warmes Creme, Bernstein
theme-light-sage-blurb = Sanftes Grün
theme-light-lilac-blurb = Blasses Violett
theme-light-blush-blurb = Altrosa
theme-light-sky-blurb = Klares Himmelblau
theme-light-mint-blurb = Kühles Aqua-Mint
theme-light-apricot-blurb = Sanfter Pfirsich, Orange
theme-light-lemon-blurb = Blassgelb, Ocker
theme-dark-graphite-blurb = Neutrales Grau, ohne Tönung
theme-dark-slate-blurb = Blaugrau
theme-dark-obsidian-blurb = Fast schwarz, für OLED
theme-dark-ember-blurb = Warmes Anthrazit, Bernstein
theme-dark-moss-blurb = Waldgrün-Schwarz
theme-dark-aubergine-blurb = Pflaumenschwarz, Flieder
theme-dark-oxblood-blurb = Weinschwarz, Koralle
theme-dark-midnight-blurb = Tiefes Marineblau, Himmelblau
theme-dark-lagoon-blurb = Tiefes Petrol, Aqua
theme-dark-neon-blurb = Fast schwarz, Pink

## texnames.ts

texnames-needs-map = Das Muster braucht { "{" }map{ "}" }, sonst bekämen alle Maps eines Materials denselben Namen
texnames-placeholders = Das Muster kann { "{" }name{ "}" }, { "{" }map{ "}" } und { "{" }size{ "}" } verwenden
texnames-bad-characters = Namen dürfen kein < > : " / \ | ? oder * enthalten
texnames-no-suffix = Geben Sie Maps vom Typ „{ $role }“ ein Suffix
texnames-bad-suffix = Das Suffix für Maps vom Typ „{ $role }“ darf kein < > : " / \ | ? oder * enthalten
texnames-slot-no-suffix = Geben Sie „{ $slot }“ ein Suffix
texnames-slot-bad-suffix = Das Suffix für „{ $slot }“ darf kein < > : " / \ | ? oder * enthalten
texnames-ending-missing = Jede eigene Endung braucht die Endung, nach der gesucht wird
texnames-ending-bad = Eigene Endungen dürfen kein < > : " / \ | ? oder * enthalten
texnames-ending-twice = { $ending } steht zweimal in Ihren Endungen
texnames-slot-metallic-roughness = Metall und Rauheit
texnames-slot-specular-color = Specular-Farbe
texnames-slot-specular-glossiness = Specular und Glanz
texnames-slot-clearcoat = Clearcoat
texnames-slot-clearcoat-roughness = Clearcoat-Rauheit
texnames-slot-clearcoat-normal = Clearcoat-Normal
texnames-slot-sheen-color = Sheen-Farbe
texnames-slot-sheen-roughness = Sheen-Rauheit
texnames-slot-transmission = Transmission
texnames-slot-thickness = Dicke
texnames-slot-diffuse-transmission = Diffuse Transmission
texnames-slot-diffuse-transmission-color = Farbe der diffusen Transmission
texnames-slot-iridescence = Irisieren
texnames-slot-iridescence-thickness = Dicke des Irisierens
texnames-slot-anisotropy = Anisotropie

## storeFormats.ts

storefiles-field-title = Titel
storefiles-field-title-hint = Der genaue Titel des Angebots. Er benennt das Paket; ohne ihn wird die Datei übersprungen.
storefiles-field-seller = Verkäufer
storefiles-field-seller-hint = Wer es gemacht hat oder verkauft.
storefiles-field-license = Lizenz
storefiles-field-license-hint = So, wie die Datei sie schreibt, etwa CC-BY-4.0.
storefiles-field-url = Store-Seite
storefiles-field-url-hint = Die Adresse des Angebots, beginnend mit http:// oder https://.
storefiles-field-words = Tags
storefiles-field-words-hint = Wörter, mit denen es getaggt wird. Kommagetrennte Werte werden aufgeteilt.
storefiles-field-type = Art
storefiles-field-type-hint = Was es ist, etwa „3D model“ oder „surface“. Wird auch zum Tag.
storefiles-field-colors = Farben
storefiles-field-colors-hint = Seine Farben, als color:-Tags.
storefiles-field-id = Store-ID
storefiles-field-id-hint = Die ID des Stores dafür: Buchstaben, Ziffern, _ oder -.
storefiles-field-origin = Hergestellt in
storefiles-field-origin-hint = Wo es hergestellt oder gescannt wurde, für die Beschreibung.
storefiles-field-size = Größe
storefiles-field-size-hint = Wie groß es ist, in Worten, für die Beschreibung.
storefiles-field-title-name = den Titel
storefiles-field-seller-name = den Verkäufer
storefiles-field-license-name = die Lizenz
storefiles-field-url-name = die Store-Seite
storefiles-field-words-name = die Tags
storefiles-field-type-name = die Art
storefiles-field-colors-name = die Farben
storefiles-field-id-name = die Store-ID
storefiles-field-origin-name = den Herstellungsort
storefiles-field-size-name = die Größe

## StoreFormatsPanel.vue

storefiles-load-failed = Die Store-Dateiformate konnten nicht geladen werden: { $error }
storefiles-remove-title = { $name } entfernen?
storefiles-remove-body = { $file } wird aus dem Datenordner gelöscht. Pakete, die bereits damit gelesen wurden, behalten, was es gelesen hat.
storefiles-remove = Entfernen
storefiles-removed = { $name } entfernt.
storefiles-remove-failed = { $name } konnte nicht entfernt werden: { $error }
storefiles-unread = Dateien, die Hoard nicht liest
storefiles-loading = Wird geladen…
storefiles-unread-none = Keine: Keine Datei in der Bibliothek sieht nach der eines ungelesenen Stores aus.
storefiles-unread-what = Dateien, die in mehr als einem Paket vorkommen oder wie die eines Stores benannt sind und die kein Format liest.
storefiles-unread-packs =
    { $count ->
        [one] { $count } Paket
       *[other] { $count } Pakete
    }
storefiles-unread-example = z. B. { $pack }
storefiles-teach = Hoard diese Datei beibringen
storefiles-formats = Formate
storefiles-formats-what = Ihre eigenen werden zuerst versucht und ersetzen ein eingebautes mit demselben Namen.
storefiles-dir-before = Sie liegen in
storefiles-dir-after = .
storefiles-yours = Eigenes
storefiles-built-in = Eingebaut
storefiles-edit = Bearbeiten
storefiles-start-from = Davon ausgehen

## TagsPanel.vue

tags-load-failed = Die Tags konnten nicht geladen werden: { $error }
tags-rename-title = { $tag } umbenennen?
tags-merge-title = { $count } Tags zusammenführen?
tags-rename-body = Seine { $count } Assets bekommen stattdessen { $into }, und die Tagger verwenden ab jetzt den neuen Namen.
tags-merge-body = { $tags } werden auf bis zu { $count } Assets zu { $into }. Die Tagger verwenden ab jetzt { $into }.
tags-rename = Umbenennen
tags-merge = Zusammenführen
tags-merged =
    { $count ->
        [one] { $count } Asset ist jetzt mit { $into } getaggt
       *[other] { $count } Assets sind jetzt mit { $into } getaggt
    }
tags-merge-failed = Zusammenführen fehlgeschlagen: { $error }
tags-forget-failed = Das konnte nicht geändert werden: { $error }
tags-find = Tags suchen
tags-find-placeholder = Tags suchen, z. B. rock oder set:
tags-sort = Reihenfolge
tags-sort-count = Meistgenutzt
tags-sort-name = A–Z
tags-count =
    { $count ->
        [one] { $count } Tag
       *[other] { $count } Tags
    }
tags-count-more = { $count }+ Tags
tags-what = Wählen Sie Tags mit gleicher Bedeutung aus und führen Sie sie zusammen, oder wählen Sie eines aus, um es umzubenennen.
tags-new-name = Neuer Name
tags-merge-into = Zusammenführen zu
tags-saved-as-before = Gespeichert als
tags-saved-as-after = { "" }
tags-cancel = Abbrechen
tags-loading = Wird geladen…
tags-none = Keine Tags.
tags-none-containing = Keine Tags mit „{ $text }“.
tags-browse-title = Assets mit dem Tag { $tag } durchsuchen
tags-browse = Durchsuchen
tags-merged-heading = Zusammengeführte Tags
tags-merged-what = Wenn die Tagger auf einen dieser alten Namen kommen, bekommt das Asset das Tag, in das er zusammengeführt wurde.
tags-allow-old = Alten Namen wieder zulassen
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } Asset
       *[other] { $count } Assets
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Alter Name: { $names }
       *[other] Alte Namen: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } wurde mit { $into } zusammengeführt.
tag-page-show = { $tag } anzeigen
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Beschreiben Sie, was dieses Tag bedeutet …
tag-page-describe-label = Was dieses Tag bedeutet
tag-page-describe-placeholder = Wofür es gedacht ist, was dazugehört. Enter speichert.
tag-page-describe-edit = Zum Ändern klicken
tag-page-description-saved = Bedeutung des Tags gespeichert
# The tag's picture: one of its assets stands for it.
tag-page-picture = Bild für dieses Tag auswählen
tag-page-picture-help = Eines seiner Assets als Bild auswählen
tag-page-picture-nothing = Keines der angezeigten Assets hat ein Bild.
tag-page-picture-none = Kein Bild
tag-page-picture-saved = Bild festgelegt
tag-page-picture-removed = Bild entfernt
tag-page-rename = Umbenennen …
tag-page-rename-title = Geben Sie ihm einen neuen Namen oder den Namen eines anderen Tags, um es damit zusammenzuführen
tag-page-rename-label = Neuer Name
tag-page-save-failed = Das Tag konnte nicht gespeichert werden: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Unter
tag-page-parent-add = Einordnen unter…
tag-page-parent-add-title = Ordnen Sie dieses Tag unter einem allgemeineren ein: Wer danach sucht, findet auch diese
tag-page-parent-placeholder = Ein allgemeineres Tag, z. B. Fahrzeug
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = { $tag } aus { $parent } herausnehmen
tag-page-parent-saved = { $tag } steht jetzt unter { $parent }
tag-page-parent-removed = { $tag } steht nicht mehr unter { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Spezieller:
tag-page-narrower-title = Die Suche nach { $tag } findet auch diese
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Allgemeiner und spezieller
tags-tree-what = Stell ein Tag unter ein allgemeineres, hier oder auf seiner Tag-Seite, dann findet die Suche nach dem allgemeineren auch alles darunter.
tags-tree-none = Noch steht nichts unter etwas anderem. Stell doch mal Auto unter Fahrzeug.
tags-tree-tag-label = Tag, das unter ein allgemeineres soll
tags-tree-tag-placeholder = Ein Tag, z. B. Auto
tags-tree-goes-under = gehört unter
tags-tree-parent-label = Allgemeineres Tag
tags-tree-put = Einordnen

## LibraryStats.vue

stats-source-human = Von Personen hinzugefügt
stats-source-ai = Von KI vorgeschlagen
stats-source-rule = Eingebaute Regeln
stats-source-meta = Aus den Dateien
stats-assets = Assets
stats-packs = Pakete
stats-on-disk = Auf der Festplatte
stats-duplicates-title = { $count } Dateien sind bytegleiche Kopien anderer und werden nur einmal gespeichert
stats-saved-dedup = Durch Deduplizierung gespart
stats-distinct-tags = Verschiedene Tags
stats-archive = Im Archiv
stats-load-failed = Die Details konnten nicht geladen werden: { $error }
stats-counting = Wird gezählt…
stats-as-of = Stand: { $time }
stats-updating = Wird aktualisiert…
stats-by-kind = Speicherplatz nach Art
stats-files-size = { $name }: { $count } Dateien, { $size }
stats-months = Im letzten Jahr hinzugefügt
stats-month = { $month }: { $count } Assets, { $size }
stats-tagging = Tagging
stats-tagged = Getaggt
stats-untagged = { $count } ohne Tags
stats-described = Beschrieben
stats-described-assets = { $count } Assets
stats-human-edited = Von Hand bearbeitet
stats-human-edited-unit =
    { $count ->
       *[other] Assets
    }
stats-source-different = { $count } verschiedene
stats-most-used = Am häufigsten verwendet
stats-show-tag = Die Assets mit { $tag } anzeigen
stats-file-types = Dateitypen
stats-biggest-packs = Größte Pakete
stats-browse-pack = { $name } durchsuchen
stats-largest-files = Größte Dateien
stats-open-file = { $name } öffnen
stats-empty = Noch nichts in der Bibliothek. Fügen Sie ein Paket hinzu, dann erscheinen seine Zahlen hier.

## HoardPanel.vue

hoard-title = Ihr Hort
hoard-what = Jedes Asset in der Bibliothek ist ein Schatzstück, und seine Art bestimmt, welches. Sammeln Sie das Neue ein, um es Ihrem Haufen hinzuzufügen.
hoard-sign-in = Melden Sie sich an, um einen eigenen Hort zu sammeln.
hoard-new = Neue Beute zum Einsammeln: { $count }
hoard-none-new = Nichts Neues zum Einsammeln.
hoard-collect = Einsammeln
hoard-unavailable = Dieser Browser kann den Hort nicht zeichnen: { $error }
hoard-collect-failed = Was Sie eingesammelt haben, konnte nicht gespeichert werden: { $error }
hoard-arcade = Arcade-Bildschirm
hoard-full-screen = Vollbild
hoard-leave-full-screen = Vollbild beenden
hoard-label = Ein Drache bewacht seinen Hort. Schätze: { $count }
hoard-gold = Gold
hoard-gems = Edelsteine
hoard-books = Bücher
hoard-weapons = Waffen
hoard-bones = Knochen
hoard-tablets = Runentafeln
hoard-paintings = Gemälde
hoard-potions = Tränke

## UpdatesPanel.vue

updates-saved = Hoard ruft sie auf, wenn ein Update bereitsteht
updates-saved-none = Keine Adresse zum Aufrufen
updates-save-failed = Speichern fehlgeschlagen
updates-call-answered = Antwort: { $result }
updates-call-refused = Die Anfrage wurde nicht angenommen: { $result }
updates-call-failed = Aufruf fehlgeschlagen
updates-called = Am { $when } für { $version } aufgerufen: { $result }
updates-called-failed = Am { $when } für { $version } aufgerufen: { $result }. Hoard versucht es in einer Stunde erneut.
updates-latest = Hoard { $version } ist die neueste Version.
updates-out = Hoard { $version } ist erschienen.
updates-out-idle = Hoard { $version } ist erschienen; der Aufruf erfolgt in wenigen Minuten, sobald gerade nichts läuft.
updates-out-busy = Hoard { $version } ist erschienen; es wird gewartet, bis nichts mehr läuft ({ $work }).
updates-busy-more = { $count } weitere
updates-busy-jobs =
    { $count ->
        [one] { $count } wartender Auftrag
       *[other] { $count } wartende Aufträge
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } Upload
       *[other] { $count } Uploads
    }
updates-list-separator = ,{ " " }
updates-what = Wenn eine neue Version erscheint, kann Hoard eine Adresse aufrufen, sobald nichts läuft (keine Uploads, Aufträge, Umwandlungen oder Wartung), damit das, was seinen Container aktualisiert, ihn nicht mitten in einem Upload neu startet. Es sendet ein POST mit der Version in einem JSON-Body. Dafür muss die Versionsprüfung oben aktiviert sein.
updates-url = Aufzurufende Adresse
updates-url-placeholder = Keine (z. B. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Gespeichert (zum Ersetzen tippen)
updates-auth-placeholder = Keiner (z. B. Bearer my-token)
updates-remove = Entfernen
updates-save = Speichern
updates-call-now-title = Was immer damit verbunden ist, wird ausgeführt
updates-call-now = Jetzt aufrufen
updates-what-calls = Was kann sie aufrufen?
updates-watchtower = Die HTTP-API von Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , mit{ " " }
updates-watchtower-token = { " " }und dem Token aus{ " " }
updates-watchtower-its = { " " }(mit aktiviertem{ " " }
updates-watchtower-endpoint = -Endpunkt).
updates-portainer = Ein Portainer-Webhook
updates-portainer-rest = { " " }für Hoards Stack oder Container, mit aktiviertem „Re-pull image“. Das Token steht in der URL.
updates-script = Ein Skript auf dem Host
updates-script-listener = { " " }hinter einem Webhook-Listener (etwa{ " " }
updates-script-runs = ), das{ " " }
updates-script-end = { " " }ausführt.
updates-others = Home Assistant, n8n
updates-others-rest = { " " }oder alles andere, was einen Webhook annimmt.
updates-poll-before = Oder ganz ohne Adresse: Ein Skript mit einem Zugriffstoken kann{ " " }
updates-poll-until = { " " }abfragen, bis{ " " }
updates-poll-and = { " " }gesetzt und{ " " }
updates-poll-after = { " " }true ist, und Hoard dann neu ziehen und neu starten.

## MaintenancePanel.vue

maintenance-load-failed = Die Wartung konnte nicht geladen werden
maintenance-reanalyze-title = Alle Pakete neu analysieren?
maintenance-reanalyze-body = Jede Datei wird erneut gelesen, was bei einer großen Bibliothek lange dauern kann; Pakete werden als in Arbeit angezeigt, bis sie an der Reihe sind. Ihre eigenen Namen, Tags und Notizen bleiben erhalten.
maintenance-reanalyze-confirm = Neu analysieren
maintenance-gc-title = Unbenutzte Dateien löschen?
maintenance-gc-body = Gespeicherte Dateien, auf die nichts in der Bibliothek verweist, werden endgültig gelöscht. Pakete im Archiv behalten ihre.
maintenance-gc-confirm = Löschen
maintenance-start-failed = „{ $task }“ konnte nicht gestartet werden
maintenance-just-now = gerade eben
maintenance-minutes-ago = vor { $count } Min.
maintenance-hours-ago = vor { $count } Std.
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Wird geladen…
maintenance-running = Läuft…
maintenance-starting = Wird gestartet…
maintenance-run = Ausführen
maintenance-stop = Anhalten
maintenance-stopping = Wird angehalten…
maintenance-stop-failed = „{ $task }“ konnte nicht angehalten werden
maintenance-started = { $detail } · gestartet { $when }
maintenance-starting-detail = Wird gestartet
maintenance-finished = Abgeschlossen:{ " " }
maintenance-failed = Fehlgeschlagen:{ " " }
maintenance-never-run = Noch nie ausgeführt
maintenance-history = Frühere Läufe
maintenance-col-task = Aufgabe
maintenance-col-finished = Abgeschlossen
maintenance-col-took = Dauer
maintenance-col-result = Ergebnis
maintenance-script-before = Backups und die Schadensprüfung lassen sich auch per Skript starten:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }und{ " " }
maintenance-script-after = . Diese Läufe erscheinen ebenfalls hier.

## SettingsPage.vue

settings-title = Einstellungen
settings-loading = Wird geladen…
settings-find = Einstellung finden
settings-find-none = Keine Einstellung passt zu „{ $query }“.
settings-load-failed = Die Einstellungen konnten nicht geladen werden: { $error }
settings-page-appearance = Darstellung
settings-page-appearance-blurb = Sprache, Designs, Textgröße und Abstände sowie was die Kacheln und die obere Leiste zeigen. Nur in diesem Browser gespeichert.
settings-page-dragon = Drache
settings-page-dragon-blurb = Der Hort deines Drachen: Jedes Asset in der Bibliothek ist ein Stück seines Schatzes.
settings-page-library = Bibliothek
settings-page-library-blurb = Was im Hoard steckt: wie groß er ist, woraus er besteht, wie er gewachsen ist und wie gut er getaggt ist. Archiviertes wird nicht mitgezählt.
settings-page-copies = Beinahe-Kopien
settings-page-copies-blurb = Dateien, die dasselbe Bild, derselbe Sound oder dasselbe Modell in anderer Größe, Abtastrate oder einem anderen Format sind, die mit dem meisten Platz zuerst. Die beste jeder Gruppe ist markiert; nichts wird gelöscht.
copies-loading = Suche nach Beinahe-Kopien…
copies-refreshing = Die Bibliothek hat sich geändert, seit diese gefunden wurden. Erneute Suche…
copies-none = Keine Beinahe-Kopien: Jedes Bild, jeder Sound und jedes Modell ist nur einmal da.
copies-load-failed = Beinahe-Kopien konnten nicht aufgelistet werden: { $error }
copies-sum = Gruppen von Beinahe-Kopien: { $count }. Die Kopien, die nicht die besten sind, belegen { $size }.
copies-set = Gruppe { $n }
copies-set-wasted = { $size } in den Kopien, die nicht die besten sind
copies-more = Mehr anzeigen (noch { $count })
settings-page-tags = Tags
settings-page-tags-blurb = Jedes Tag in der Bibliothek. Führen Sie gleichbedeutende zusammen oder benennen Sie eines um; die Tagger verwenden ab dann den neuen Namen.
settings-page-store-files = Store-Dateien
settings-page-store-files-blurb = Die Dateien, die Stores ihren Assets beilegen und die Hoard liest, um Pakete zu benennen und ihre Urheber zu nennen. Bringen Sie ihm unbekannte bei, und teilen Sie, was Sie ihm beigebracht haben.
settings-page-locations = Speicherorte
settings-page-locations-blurb = Die Ordner auf dem Server, die die Bibliothek enthalten: solche, in denen Hoard Dateien aufbewahrt, und Ordner mit Paketen, die es dort indiziert, wo sie liegen.
settings-page-locations-blurb-desktop = Die Ordner, die die Bibliothek enthalten: solche, in denen Hoard Dateien aufbewahrt, und Ordner mit Paketen, die es dort indiziert, wo sie liegen.
settings-page-tools = Werkzeuge
settings-page-tools-blurb = Externe Programme, mit denen Hoard Dateien umwandelt und liest.
settings-page-ai = KI-Tagging
settings-page-ai-blurb = Ein KI-Modell benennt, beschreibt und taggt neue Pakete, zusätzlich zu den eingebauten Regeln. Seine Tags werden als vorgeschlagen markiert.
settings-page-search-blurb = Bilder nach ihrem Inhalt und Sounds nach ihrem Klang suchen, nicht nur nach ihrem Namen, und Dinge finden, die sich in der Art ähneln, nicht nur in der Farbe.
settings-page-search-tab = Suche
settings-page-adding = Pakete hinzufügen
settings-page-adding-blurb = Was Hoard mit einem Paket macht, wenn es hinzugefügt wird, außer es zu lesen und zu taggen.
settings-page-downloads = Downloads
settings-page-downloads-blurb = Wie Dateien in dem benannt werden, was Hoard ausliefert: Downloads, Zips, das Werkzeug hoard und KI-Assistenten. Die Dateien in der Bibliothek bleiben, wie sie sind.
settings-page-account = Konto
settings-page-account-blurb = Ihr Passwort, die anderen Personen, die sich anmelden können, und Zugriffstokens für Skripte und andere Programme.
settings-page-auth = Authentifizierung
settings-page-auth-blurb = Wie sich Personen anmelden: Hoards eigene Konten, ein LDAP-Verzeichnis, Microsoft Active Directory oder ein OAuth-Anbieter, wie lange Sitzungen dauern und welche Gruppen welche Rolle bekommen.
settings-page-connection = Verbindung
settings-page-connection-blurb = Wie Browser und Apps Hoard erreichen: die Adresse, unter der es bekannt ist, und HTTPS, das manche Browserfunktionen brauchen.
settings-page-performance = Leistung
settings-page-performance-blurb = Wie viel der Server arbeitet, wie Vorschaubilder erstellt und aufbewahrt werden und wie viel er höchstens annimmt.
settings-page-performance-blurb-desktop = Wie viel Hoard arbeitet, wie Vorschaubilder erstellt und aufbewahrt werden und wie viel es höchstens annimmt.
settings-page-issues = Probleme
settings-page-issues-blurb = Was beim Öffnen oder Verarbeiten von Dateien schiefging: Archive, die sich nicht entpacken ließen, Dateien, die nicht gelesen werden konnten, fehlgeschlagene Aufträge. Versuchen Sie es erneut, sobald die Ursache behoben ist, oder verwerfen Sie sie.
settings-page-maintenance = Wartung
settings-page-maintenance-blurb = Sichern, auf Schäden prüfen und aufräumen. Von jeder Aufgabe läuft immer nur eine, und Sie können diese Seite verlassen, während sie läuft.
settings-page-about = Über
settings-page-about-blurb = Was Hoard ist, welche Version das ist und die Arbeit anderer, aus der es entstanden ist.
settings-the-default = Standard
settings-default-on = an
settings-default-off = aus
settings-default-auto = automatisch
settings-option-saved = { $name }: { $value }
settings-back-to-default = Auf Standard zurücksetzen
settings-save = Speichern
settings-use = Verwenden
settings-status-off = Aus

## SettingsPage.vue: Darstellung

settings-themes = Designs
settings-look-light = Hell
settings-look-dark = Dunkel
settings-look-group =
    { $scheme ->
        [light] Helles Design
       *[dark] Dunkles Design
    }
settings-preview =
    Das { $preview ->
        [light] helle
       *[dark] dunkle
    } Design wird zur Ansicht gezeigt. Sie verwenden das { $used ->
        [light] helle
       *[dark] dunkle
    }{ $system ->
        [yes] { " " }(wie das System)
       *[other] { "" }
    }, und es kehrt zurück, wenn Sie diese Seite verlassen.
settings-preview-back =
    Zurück zum { $used ->
        [light] hellen
       *[dark] dunklen
    } Design
settings-look-hint = Wechseln Sie in der oberen Leiste zwischen hell und dunkel (oder folgen Sie dem System).

## SettingsPage.vue: Werkzeuge

settings-blender-what = Wandelt .blend-, .dae-, .abc- und .x3d-Modelle im Hintergrund in glTF um, damit sie Vorschaubilder und eine 3D-Vorschau bekommen. Pakete, die hinzugefügt wurden, während es aus war, werden umgewandelt, sobald es bereit ist.
settings-blender-download = Verwendet ein installiertes Blender oder lädt Blender 4.5 LTS (etwa 400 MB) von einem blender.org-Mirror herunter.
settings-ffmpeg-what = Kodiert MP3-, OGG- und Opus-Downloads und liest Audio, das Hoard nicht selbst dekodieren kann (AAC, M4A, WMA, …).
settings-ffmpeg-download = Verwendet ein System-ffmpeg mit den passenden Encodern oder lädt einen statischen Build herunter (etwa 80 MB).
settings-tool-reset = { $tool }: auf Standard zurücksetzen
settings-tool-no-download = Verwendet ein installiertes { $tool }. Für diesen Rechner gibt es keinen Build zum Herunterladen, also installieren Sie es oder geben Sie seinen Pfad an.
settings-tool-path = Programmpfad von { $tool }
settings-mode-off = Aus
settings-mode-auto = Automatisch
settings-mode-path = Dieses Programm
settings-tool-looking = Wird gesucht…
settings-tool-downloading = Wird heruntergeladen… den Fortschritt zeigt die Aktivitätsanzeige
settings-tool-ready = Bereit: { $detail }
settings-soundfont = MIDI-SoundFont
settings-soundfont-what = Die Instrumente, mit denen MIDI-Dateien abgespielt und umgewandelt werden.
settings-soundfont-file = SoundFont-Datei
settings-soundfont-placeholder = Auf dem Server gefunden oder eingebaut
settings-soundfont-placeholder-desktop = Auf diesem Computer gefunden oder eingebaut
settings-soundfont-default = auf dem Server gefunden oder eingebaut
settings-soundfont-default-desktop = auf diesem Computer gefunden oder eingebaut
settings-soundfont-send = Datei wählen…
settings-soundfont-sent = MIDI spielt jetzt mit { $name }

## SettingsPage.vue: Bildsuche

settings-clip-heading = Nach Bildinhalt suchen
settings-clip-reset = Bildsuche: auf Standard zurücksetzen
settings-clip-saved-on = Bildsuche: an
settings-clip-saved-off = Bildsuche: aus
settings-clip-what = Geben Sie ein, was Sie suchen („mossy rock“, „sci-fi door“, „wooden barrel“), und Bilder, die das zeigen, erscheinen neben Namen, die die Wörter enthalten. „Mehr davon“ findet Dinge derselben Art, nicht nur mit denselben Farben.
settings-clip-toggle = Bildsuche einschalten
settings-clip-hint = Verwendet das SigLIP-2-Modell von Google, ausgeführt auf dem Prozessor dieses Servers: Nichts verlässt ihn. Beim Einschalten wird das Modell ({ $size } MB) von Hugging Face heruntergeladen, das dann im Hintergrund mit niedriger Priorität jedes Bild lernt. Während es arbeitet oder jemand sucht, belegt es 350 bis 750 MB Arbeitsspeicher. Beim Ausschalten wird das Modell gelöscht, das Gelernte aber für das nächste Mal behalten.
settings-clip-hint-desktop = Verwendet das SigLIP-2-Modell von Google, ausgeführt auf dem Prozessor dieses Computers: Nichts verlässt ihn. Beim Einschalten wird das Modell ({ $size } MB) von Hugging Face heruntergeladen, das dann im Hintergrund mit niedriger Priorität jedes Bild lernt. Während es arbeitet oder Sie suchen, belegt es 350 bis 750 MB Arbeitsspeicher. Beim Ausschalten wird das Modell gelöscht, das Gelernte aber für das nächste Mal behalten.
settings-clip-looking = Modell wird gesucht…
settings-clip-downloading = Modell wird heruntergeladen… den Fortschritt zeigt die Aktivitätsanzeige
settings-clip-ready-empty = Bereit. Bilder werden gelernt, sobald Pakete hinzukommen.
settings-clip-ready-all =
    { $count ->
        [one] Bereit: kennt { $count } Bild
       *[other] Bereit: kennt alle { $count } Bilder
    }
settings-clip-learning = Bereit: lernt Bilder, bisher { $done } von { $total }
settings-clip-relearning = Bis das neue Modell jedes Bild gelernt hat, nutzen Suchen, was das alte gelernt hat.
settings-clip-unavailable = Das Modell konnte nicht geholt werden: { $detail }. Neuer Versuch jede Stunde.
settings-clip-where = Läuft
settings-clip-here = Auf diesem Server: nichts verlässt ihn
settings-clip-here-desktop = Auf diesem Computer: nichts verlässt ihn
settings-clip-hosted = Bei einem gehosteten Dienst
settings-clip-hosted-warning = Bilder verlassen diesen Server. Eine Miniatur jedes Bildes der Bibliothek und die Wörter jeder Bildsuche werden an { $service } gesendet, das dafür Gebühren berechnet und sie nach seinen Bedingungen aufbewahrt.
settings-clip-hosted-warning-desktop = Bilder verlassen diesen Computer. Eine Miniatur jedes Bildes der Bibliothek und die Wörter jeder Bildsuche werden an { $service } gesendet, das dafür Gebühren berechnet und sie nach seinen Bedingungen aufbewahrt.
settings-clip-hosted-hint = Hier wird nichts heruntergeladen oder ausgeführt: Der Dienst lernt jedes Bild, stapelweise. Ein anderer Dienst oder ein anderes Modell lernt alle neu, und was jedes gelernt hat, bleibt erhalten, für den Wechsel zurück.
settings-clip-saved-here = Bildsuche: läuft auf diesem Server
settings-clip-saved-here-desktop = Bildsuche: läuft auf diesem Computer
settings-clip-saved-hosted = Bildsuche: läuft bei { $service }
settings-clip-hosted-unavailable = { $service } konnte nicht genutzt werden: { $detail }. Neuer Versuch jede Stunde.
settings-clap-heading = Nach Klang suchen
settings-clap-reset = Soundsuche: auf Standard zurücksetzen
settings-clap-saved-on = Soundsuche: an
settings-clap-saved-off = Soundsuche: aus
settings-clap-what = Geben Sie ein, was Sie suchen („distant thunder“, „glassy UI blip“, „footsteps on gravel“), und Sounds, die so klingen, erscheinen neben Namen, die die Wörter enthalten.
settings-clap-toggle = Soundsuche einschalten
settings-clap-hint = Verwendet das CLAP-Modell von LAION, ausgeführt auf dem Prozessor dieses Servers: Nichts verlässt ihn. Beim Einschalten wird das Modell ({ $size } MB) von Hugging Face heruntergeladen, das dann im Hintergrund mit niedriger Priorität in den Anfang jedes Sounds hineinhört. Während es arbeitet oder jemand sucht, belegt es bis zu etwa 1 GB Arbeitsspeicher. Beim Ausschalten wird das Modell gelöscht, das Gelernte aber für das nächste Mal behalten.
settings-clap-hint-desktop = Verwendet das CLAP-Modell von LAION, ausgeführt auf dem Prozessor dieses Computers: Nichts verlässt ihn. Beim Einschalten wird das Modell ({ $size } MB) von Hugging Face heruntergeladen, das dann im Hintergrund mit niedriger Priorität in den Anfang jedes Sounds hineinhört. Während es arbeitet oder Sie suchen, belegt es bis zu etwa 1 GB Arbeitsspeicher. Beim Ausschalten wird das Modell gelöscht, das Gelernte aber für das nächste Mal behalten.
settings-clap-ready-empty = Bereit. Sounds werden gelernt, sobald Pakete hinzukommen.
settings-clap-ready-all =
    { $count ->
        [one] Bereit: kennt { $count } Sound
       *[other] Bereit: kennt alle { $count } Sounds
    }
settings-clap-learning = Bereit: lernt Sounds, bisher { $done } von { $total }

## SettingsPage.vue: KI-Tagging

settings-ai-service = Dienst
settings-ai-off = Aus
settings-ai-off-hint = Pakete werden nur mit den eingebauten Regeln getaggt.
settings-ai-local = Lokales Modell
settings-ai-local-hint = Der Dienst llm in Hoards Compose-Datei (oder cargo xtask llm). Laden Sie unten ein Modell herunter; es läuft auf diesem Rechner, und nichts verlässt ihn.
settings-ai-chatgpt = ChatGPT-Abo
settings-ai-chatgpt-hint = Verwendet Ihr ChatGPT-Abo Go, Plus, Pro oder Business statt eines API-Schlüssels.
settings-ai-openrouter-hint = Ein Schlüssel für Hunderte gehostete Modelle, manche davon kostenlos. Modelle, die Bilder annehmen, sind markiert.
settings-ai-openai-hint = Gehostet von OpenAI.
settings-ai-anthropic-hint = Gehostet von Anthropic, über dessen OpenAI-kompatiblen Endpunkt.
settings-ai-gemini-hint = Gehostet von Google, über dessen OpenAI-kompatiblen Endpunkt.
settings-ai-ollama-hint = Ein Ollama-Server, den Sie betreiben. Ändern Sie die Adresse, wenn er kein Container namens ollama ist.
settings-ai-custom = Andere (OpenAI-kompatibel)
settings-ai-custom-hint = Alles, was die Chat-API von OpenAI spricht: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Verbindung wird hergestellt…
settings-ai-connected =
    { $count ->
        [one] Verbunden · { $count } Modell
       *[other] Verbunden · { $count } Modelle
    }
settings-ai-connected-none = Verbunden, aber es werden keine Modelle aufgelistet; geben Sie den Namen des Modells ein
settings-chatgpt-connected-none = Verbunden; geben Sie den Namen des Modells ein
settings-ai-takes-images = nimmt Bilder an
settings-ai-saved = KI-Tagging: { $service }
settings-ai-saved-off = KI-Tagging ist aus
settings-ai-reset = KI-Tagging: auf Standard zurücksetzen
settings-ai-on = An: { $service } · { $model }
settings-ai-url = Adresse
settings-ai-key = API-Schlüssel
settings-ai-key-saved = Gespeichert (zum Ersetzen tippen)
settings-ai-key-paste = Schlüssel einfügen
settings-ai-key-optional = Keiner (optional)
settings-ai-key-remove = Entfernen
settings-ai-get-key = Schlüssel holen
settings-ai-model = Modell
settings-ai-model-pick = Modell wählen oder eingeben
settings-ai-model-name = Modellname
settings-ai-refresh = Aktualisieren
settings-ai-connect = Verbinden
settings-ai-need-key = Fügen Sie den API-Schlüssel hinzu, um die Modelle zu sehen
settings-ai-vision = Auch Vorschaubilder senden (braucht ein Modell, das Bilder annimmt)
settings-ai-batch = Pro Anfrage
settings-ai-batch-unit = Assets (weniger für kleine lokale Modelle, mehr für große gehostete)

## SettingsPage.vue: ChatGPT-Anmeldung

settings-chatgpt-account = Konto
settings-chatgpt-checking = Wird geprüft…
settings-chatgpt-status-as = Angemeldet als { $email }
settings-chatgpt-status = Angemeldet
settings-chatgpt-signed-in-as = Bei ChatGPT angemeldet als { $email }
settings-chatgpt-signed-in = Bei ChatGPT angemeldet
settings-chatgpt-signed-out = Von ChatGPT abgemeldet
settings-chatgpt-continue = Mit ChatGPT fortfahren
settings-chatgpt-sign-out = Abmelden
settings-chatgpt-sign-out-title = Von ChatGPT abmelden?
settings-chatgpt-sign-out-in-use = Das KI-Tagging nutzt Ihr ChatGPT-Abo und hält daher an, bis Sie sich erneut anmelden oder einen anderen Dienst wählen.
settings-chatgpt-sign-out-body = Hoard vergisst die Anmeldung und bittet OpenAI, sie zu beenden.
settings-chatgpt-manage = Nutzung verwalten
settings-chatgpt-finish-before = Schließen Sie die Anmeldung im geöffneten Fenster ab (
settings-chatgpt-finish-link = erneut öffnen
settings-chatgpt-finish-middle = ). Endet sie auf einer Seite, die nicht lädt, unter einer Adresse, die mit{ " " }
settings-chatgpt-finish-after = { " " }beginnt, kopieren Sie diese Adresse und fügen Sie sie hier ein:
settings-chatgpt-paste = Die Adresse, auf der die Anmeldung endete
settings-chatgpt-finish = Abschließen
settings-chatgpt-usage = Tagging-Anfragen zählen zur Nutzung Ihres ChatGPT-Abos. „Nutzung verwalten“ zeigt, was Hoard verbraucht hat, und lässt Sie es begrenzen. Um Hoard von ChatGPT selbst zu trennen, gehen Sie auf chatgpt.com zu Einstellungen → Sicherheit und Anmeldung → Mit ChatGPT anmelden.

## SettingsPage.vue: Lokale Modelle

settings-local-downloading-progress = Wird heruntergeladen { $percent } % · { $done } von { $total }
settings-local-downloading = Wird heruntergeladen…
settings-local-incomplete = Download angehalten
settings-local-failed = Start beim letzten Mal fehlgeschlagen
settings-local-in-use-running = In Gebrauch · läuft
settings-local-in-use = In Gebrauch
settings-local-starting = Wird gestartet…
settings-local-running = Läuft
settings-local-ready-images = Bereit · nimmt Bilder an
settings-local-ready-text = Bereit · nur Text
settings-local-delete-title = { $name } löschen?
settings-local-delete-in-use = Das KI-Tagging verwendet dieses Modell und hält daher an, bis Sie ein anderes wählen. Seine Dateien werden vom Modellserver gelöscht.
settings-local-delete-downloading = Der Download wird abgebrochen und das bisher Heruntergeladene gelöscht.
settings-local-delete-body = Seine Dateien werden vom Modellserver gelöscht. Sie können es später erneut herunterladen.
settings-local-delete = Löschen
settings-local-deleting = Wird gelöscht…
settings-local-cancel = Abbrechen
settings-local-resume = Fortsetzen
settings-local-download = Herunterladen
settings-local-start-before = Starten Sie ihn mit{ " " }
settings-local-start-or = { " " }(oder{ " " }
settings-local-start-developing = { " " }bei der Entwicklung) und{ " " }
settings-local-start-retry = versuchen Sie es erneut
settings-local-start-after = . Ein Server, der nur ein Modell ausführt, geht auch: Geben Sie unten den Namen seines Modells an.
settings-local-list = Auf dem Modellserver
settings-local-none = Noch nichts heruntergeladen. Wählen Sie unten eines; es wird auf dem Server aufbewahrt, und Sie können jederzeit wechseln.
settings-local-catalog = Modelle zum Herunterladen
settings-local-size = { $size } GB · { $license }
settings-local-other = Ein anderes Modell von Hugging Face
settings-local-other-placeholder = Ein anderes von Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Pakete hinzufügen

settings-store-pages = Store-Seiten
settings-store-pages-what = Neue Pakete werden im Hintergrund in den Stores nachgeschlagen, damit sie ihre Store-Seite, ihren Verkäufer und ihre Lizenz bekommen.
settings-store-lookup = Store-Seiten automatisch nachschlagen
settings-unreal = Unreal-Packages
settings-unreal-reset = Unreal-Packages: auf Standard zurücksetzen
settings-unreal-what = Was mit .uasset-Dateien geschehen soll.
settings-conv = Umwandlungsregeln
settings-conv-what = Wandelt die Dateien, die Hoard speichert, in ein Format um, etwa alle OBJ und FBX in GLB, damit die Bibliothek einheitlich ist. Die umgewandelte Datei tritt an die Stelle des Originals und behält dessen Namen, Tags, Sammlungen und Verknüpfungen. Dateien in überwachten Ordnern werden nie verändert.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Aktiv
settings-conv-to = Umwandeln in
# Heading over the file formats a rule converts.
settings-conv-from = Aus
settings-conv-remove = Entfernen
settings-conv-remove-label = Regel { $n } entfernen
settings-conv-taken = Eine andere Regel wandelt dieses Format schon um
settings-conv-upright = Modelle beim Umwandeln aufrichten (Y nach oben, nach +Z gerichtet), so wie die Modelle des Pakets angezeigt werden
settings-conv-flac-what = Der Klang bleibt genau gleich, bei etwa halbem Platzbedarf, und alles andere im WAV (Loop-Punkte, Cue-Marker, Tags) bleibt im FLAC erhalten, sodass das WAV jederzeit genau so zurückgeholt werden kann, wie es war. WAVs mit Gleitkomma- oder 32-Bit-Samples bleiben WAV.
settings-conv-add = Regel hinzufügen
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Originale behalten für
settings-conv-keep-days =
    { $count ->
        [one] Tag
       *[other] Tage
    }
settings-conv-keep-what = Bis dahin lässt sich das Original über die Seite der umgewandelten Datei wiederherstellen. Texturen werden nicht in ein umgewandeltes Modell kopiert: Es verweist auf die Bilddateien daneben. Die .mtl- oder .bin-Dateien eines Modells bleiben erhalten.
settings-conv-save = Regeln speichern
settings-conv-revert = Änderungen verwerfen
settings-conv-existing = Dateien umwandeln, die schon in der Bibliothek sind
settings-conv-saved = Umwandlungsregeln gespeichert. Sie gelten für Pakete, die ab jetzt hinzugefügt werden.
settings-conv-converting =
    { $count ->
        [one] Eine Datei wird im Hintergrund umgewandelt
       *[other] { $count } Dateien werden im Hintergrund umgewandelt
    }
settings-conv-nothing = In der Bibliothek muss nichts umgewandelt werden.
settings-conv-ask-title =
    { $count ->
        [one] Eine Datei umwandeln, die schon in der Bibliothek ist?
       *[other] { $count } Dateien umwandeln, die schon in der Bibliothek sind?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Sie liegt in einem Paket.
       *[other] Sie liegen in { $packs } Paketen.
    } Jedes Original wird { $days } Tage aufbewahrt und lässt sich bis dahin über seine Seite wiederherstellen.
settings-conv-ask-busy =
    { $count ->
        [one] Eine liegt in einem Paket, das noch hinzugefügt wird; sie wird umgewandelt, sobald das fertig ist.
       *[other] { $count } liegen in Paketen, die noch hinzugefügt werden; sie werden umgewandelt, sobald das fertig ist.
    }
settings-conv-ask-list = Dateien, die umgewandelt würden
settings-conv-ask-more = …und { $count } weitere
settings-conv-ask-skipped = Bleiben, wie sie sind:
settings-conv-skip-watched =
    { $count ->
        [one] eine Datei in einem überwachten Ordner
       *[other] { $count } Dateien in überwachten Ordnern
    }
settings-conv-skip-unity =
    { $count ->
        [one] eine Datei, die ein Unity-Projekt unter ihrem Namen kennt
       *[other] { $count } Dateien, die ein Unity-Projekt unter ihren Namen kennt
    }
settings-conv-skip-used =
    { $count ->
        [one] eine Datei, auf die eine Szene oder eine andere Datei verweist
       *[other] { $count } Dateien, auf die eine Szene oder eine andere Datei verweist
    }
settings-conv-skip-beside =
    { $count ->
        [one] eine Datei, deren umgewandelte Kopie schon in ihrem Paket liegt
       *[other] { $count } Dateien, deren umgewandelte Kopien schon in ihren Paketen liegen
    }
settings-conv-skip-blender =
    { $count ->
        [one] eine Datei, die Blender noch nicht gelesen hat (Einstellungen → Werkzeuge)
       *[other] { $count } Dateien, die Blender noch nicht gelesen hat (Einstellungen → Werkzeuge)
    }
settings-conv-ask-new = Speichern, nur neue Pakete
settings-conv-ask-save-convert =
    { $count ->
        [one] Speichern und umwandeln
       *[other] Speichern und { $count } Dateien umwandeln
    }
settings-conv-ask-convert =
    { $count ->
        [one] Umwandeln
       *[other] { $count } Dateien umwandeln
    }
settings-unreal-off = Unverändert lassen
settings-unreal-off-hint = Bei Bedarf auf der Seite eines Pakets umwandeln.
settings-unreal-keep = Umwandeln, Originale behalten
settings-unreal-keep-hint = Texturen, Sounds und Materialien kommen in gängigen Formaten heraus; die .uasset bleibt ebenfalls.
settings-unreal-replace = Umwandeln, Originale verwerfen
settings-unreal-replace-hint = Spart Platz, sobald der Inhalt heraus ist. Meshes warten weiterhin auf den Unreal-Worker.

## SettingsPage.vue: Downloads

settings-tn = Texturnamen
settings-tn-saved = Texturnamen gespeichert
settings-tn-reset = Texturnamen: auf Standard zurücksetzen
settings-tn-what = Die Maps eines Materials werden nach ihm und nach ihrem Zweck benannt ausgeliefert, egal wie die Dateien heißen: Rock_Normal.png statt rock_moss_xeokah0_raw_8K_Normal.png. Die Dateien in der Bibliothek bleiben unberührt, und eine Map, die ein mitgeliefertes Modell oder MTL beim Namen nennt, behält diesen Namen, damit das Modell sie weiterhin findet.
settings-tn-toggle = Textur-Maps nach ihrem Material benennen
settings-tn-style = Stil
settings-tn-style-group = Benennungsstil
settings-tn-descriptive = Beschreibend
settings-tn-short = Kurz
settings-tn-custom = Eigener
settings-tn-pattern = Muster
settings-tn-hint-name = { " " }das Material,{ " " }
settings-tn-hint-map = { " " }wofür die Map ist,{ " " }
settings-tn-hint-size = { " " }ihre Größe, wenn es das Set in mehreren gibt (wird am Ende angehängt, wenn das Muster sie weglässt).
settings-tn-example = Beispiel
settings-tn-suffixes = Suffixe für jede Art von Map
settings-tn-slots = In GLB-Downloads wird eine Textur, die ohne eigene Datei in ein Modell gepackt ist, nach dem Material und dem Slot benannt, den sie füllt, und auch diese Slots haben Suffixe:
settings-tn-endings = Eigene Endungen
settings-tn-endings-what = Eine Textur, die keine der Maps eines Materials ist, behält ihren eigenen Namen, außer er endet auf eine dieser Endungen: _diff → _BC liefert rock_diff.png als rock_BC.png aus. Lassen Sie das zweite Feld leer, um die Endung zu entfernen.
settings-tn-ending-from = Endung
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Ersetzen durch
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Endung hinzufügen
settings-tn-ending-remove = Entfernen
settings-tn-ending-remove-label = { $ending } entfernen

## SettingsPage.vue: Allgemein und Verbindung

settings-name = Name
settings-name-what = Wie dieser Server in der oberen Leiste, im Browser-Tab und auf der Anmeldeseite heißt, damit Sie bei mehreren Hoards erkennen, welcher welcher ist.
settings-name-what-desktop = Wie diese Bibliothek in der oberen Leiste heißt.
settings-address = Adresse
settings-address-what = Die Adresse, über die Personen und Apps Hoard erreichen, wenn es nicht die ist, die dieser Browser verwendet: die eines Reverse-Proxys, eines Tunnels oder ein Tailnet-Name, etwa https://hoard.example.com. Links, die Hoard herausgibt (an Claude und andere Apps, bei der Anmeldung, in seinem Zertifikat), verwenden sie.
settings-public-url = Öffentliche Adresse
settings-public-url-placeholder = Aus der Adresse des Browsers (z. B. https://hoard.example.com)
settings-public-url-default = aus der Adresse des Browsers

## ProfilerPanel.vue: der Profiler oben in Einstellungen → Leistung

profiler = Profiler
profiler-what = Was Hoard in den letzten zehn Minuten getan hat, alle zwei Sekunden aktualisiert, solange Sie zusehen. Es wird nur im Arbeitsspeicher gehalten und beginnt von vorn, wenn Hoard neu startet.
profiler-starting = Die ersten Messwerte kommen gleich.
profiler-failed = Der Profiler konnte nicht gelesen werden: { $error }
profiler-cpu = Prozessorauslastung
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Gesamter Rechner
profiler-cpu-summary = Hoard nutzt { $hoard } der Prozessoren, der gesamte Rechner { $machine }.
profiler-ram = RAM
profiler-ram-peak = Spitze
profiler-ram-of = { $used } von { $total }
profiler-ram-summary = Hoard belegt { $hoard } RAM und hat in den letzten zehn Minuten höchstens { $peak } belegt.
profiler-serving = Laufende Anfragen
profiler-working = In Arbeit
profiler-working-none = Im Hintergrund lief nichts.
profiler-working-more =
    { $count ->
        [one] Und eine weitere.
       *[other] Und { $count } weitere.
    }
profiler-requests = Zeit für Anfragen
profiler-requests-what = Jeder Balken steht für zehn Sekunden Anfragen, ihre Zeit danach aufgeteilt, wohin sie ging.
profiler-part-db = Datenbank
profiler-part-db-title = Warten auf die Datenbank und Ausführen ihrer Abfragen
profiler-part-disk = Datenträger
profiler-part-disk-title = Lesen gespeicherter Dateien und Komprimieren des Gesendeten
profiler-part-send = Senden
profiler-part-send-title = Warten, bis das Netzwerk das Gesendete abnimmt
profiler-part-other = Sonstiges
profiler-part-other-title = Erarbeiten der Antwort
profiler-request-count =
    { $count ->
        [one] { $count } Anfrage
       *[other] { $count } Anfragen
    }
profiler-requests-none = Keine Anfragen in den letzten zehn Minuten.
profiler-slowest = Am langsamsten: { $route }, { $time }
profiler-routes = Nach Route
profiler-route = Route
profiler-count = Anfragen
profiler-average = Durchschnitt
profiler-p95 = Langsamste 5 %
profiler-split = Wohin die Zeit ging
profiler-sent = Gesendet
profiler-portal = die eigenen Dateien der Oberfläche

## SettingsPage.vue: Leistung

settings-render = 3D-Vorschaubilder
settings-render-toggle = Vorschaubilder für 3D-Modelle rendern
settings-gpu-on = Auf der GPU: { $renderer }
settings-gpu-software = Keine GPU gefunden, daher per Software ({ $renderer })
settings-gpu-none = Weder GPU noch Software-Renderer gefunden, daher bekommen Modelle keine Vorschaubilder
settings-thumbs = Vorschaubilder
settings-thumbs-what = Für neue Vorschaubilder und Modellansichten. Transparente und Pixel-Art bleiben PNG. „Vorschaubilder verkleinern“ unter Wartung erstellt vorhandene neu.
settings-previews-size = Vorschauen: { $size }
settings-thumbs-saved = Vorschaubilder: { $quality }
settings-thumbs-format = Format
settings-thumbs-quality = Qualität { $quality }.
settings-thumbs-small = Kleiner
settings-thumbs-small-hint = Im Raster in Ordnung; aus der Nähe etwas weich.
settings-thumbs-balanced = Ausgewogen
settings-thumbs-balanced-hint = In Vorschaubildgröße kaum vom Original zu unterscheiden.
settings-thumbs-sharp = Schärfer
settings-thumbs-sharp-hint = Etwa doppelt so groß wie „Ausgewogen“.
settings-thumbs-lossless = Verlustfrei
settings-thumbs-lossless-hint = PNG, genau wie erstellt. Um ein Vielfaches größer. Werden nie als AVIF gesendet.
settings-thumbs-avif-what = AVIF ist oft nur halb so groß. Jedes Vorschaubild wird beim ersten Anzeigen umgewandelt.
settings-thumbs-avif-now = Jetzt alle umwandeln
settings-thumbs-avif-converting = Wird umgewandelt…
settings-motion-thumbs = Bewegte Vorschaubilder für Animationen
settings-motion-thumbs-what = Jede Animation bekommt zusätzlich einen kurzen Film in Schleife, der abspielt, solange du auf ihre Kachel zeigst. Jeder braucht etwa 40 KB, und bei einer großen Bibliothek dauert das Erstellen eine Weile. Beim Ausschalten werden sie gelöscht.
settings-motion-fps = Bildrate
settings-motion-fps-option = { $fps } fps
settings-motion-fps-what = Flüssigere Filme brauchen länger und mehr Platz: bei 24 fps etwa doppelt so viel wie bei 12, bei 30 noch mehr. Wenn du sie änderst, werden alle neu erstellt.
settings-limits = Arbeit und Limits
settings-limits-what = Jeder Wert wird gespeichert, wenn Sie das Feld verlassen oder Enter drücken. Größen nehmen Einheiten an: 100 GB, 512 MB, 1,5 TB; eine bloße Zahl bedeutet GB.
settings-workers = Hintergrund-Worker
settings-workers-unit = Aufträge gleichzeitig (Aufnahme, Tagging, Umwandlungen)
settings-max-upload = Größter Upload
settings-max-upload-unit = pro Datei
settings-max-expand = Größtes Paket
settings-max-expand-unit = nach dem Entpacken
settings-conversion-cache = Umgewandelte Downloads
settings-conversion-cache-unit-now = werden höchstens aufbewahrt, derzeit { $size }; die am längsten unbenutzten gehen zuerst
settings-conversion-cache-unit = werden höchstens aufbewahrt; die am längsten unbenutzten gehen zuerst

## SettingsPage.vue: Wartung

settings-updates = Updates
settings-version = Dies ist Hoard { $version }.
settings-update-check = Einmal am Tag auf GitHub nach einer neuen Version suchen
settings-desktop-updates = Hoard sucht bei jedem Start nach einer neuen Version, lädt sie im Hintergrund herunter und installiert sie, wenn Sie es beenden.
settings-backups = Geplante Sicherungen
settings-backups-what = Die Datenbank sichert sich selbst; jede Sicherung erscheint im Verlauf unten.
settings-backup-every = Sichern
settings-backup-off = Nie
settings-backup-daily = Jeden Tag
settings-backup-weekly = Jede Woche
settings-backup-keep = Behalten
settings-backup-keep-unit = neueste geplante Sicherungen
settings-backup-copy-to = Auch kopieren nach
settings-backup-copy-to-none = Kein weiterer Ordner
settings-backups-kept = Jede kommt als hoard-auto-…db in den Sicherungsordner und, falls angegeben, in den weiteren Ordner (eine andere Festplatte oder einen, den eine Sync-App sichert), angegeben mit seinem vollständigen Pfad auf dem Server. Nur ältere geplante Sicherungen werden entfernt; von Hand erstellte bleiben.
about-what = Hoard bewahrt Spiel-Assets (Modelle, Texturen, Sounds, Musik, Sprites und mehr) in einer Bibliothek auf, die Sie durchsuchen, in der Vorschau ansehen und aus der Sie sich bedienen können, geteilt mit allen, denen Sie Zugang geben.
about-version = Version
about-edition = Ausgabe
about-edition-server = Server
about-edition-desktop = Desktop-App
about-edition-steam = Desktop-App von Steam
about-made-by = Entwickelt von
about-source = Quellcode auf GitHub
about-issues = Ein Problem melden
about-discord = Chatte mit uns auf Discord
about-steam = Hoard auf Steam
about-license = Lizenz
about-license-what = Hoard ist Open Source unter der Lizenz MIT No Attribution: Sie dürfen es nutzen, ändern und weitergeben, wie Sie möchten.
about-downloaded = Bei Bedarf heruntergeladen
about-downloaded-what = Diese sind nicht in Hoard enthalten. Hoard lädt jedes davon herunter, sobald es zum ersten Mal gebraucht wird, und jedes steht unter seiner eigenen Lizenz.
about-blender = Wandelt 3D-Modelle um und erstellt ihre Vorschaubilder.
about-ffmpeg = Wandelt Ton und Video um und erstellt ihre Vorschauen.
about-siglip = Findet Bilder anhand dessen, was darauf zu sehen ist.
about-clap = Findet Sounds anhand dessen, wie sie klingen.
about-models = Modelle, die Sie für den lokalen KI-Server auswählen, stehen unter den Lizenzen ihrer Urheber.
about-built-with = Erstellt mit
about-built-with-what = Die Open-Source-Bibliotheken, aus denen Hoard besteht, mit ihren Lizenzen.
about-steamworks = Die Steam-Version enthält außerdem das Steamworks SDK, © Valve Corporation.
about-licenses-loading = Liste wird geladen…
about-licenses-failed = Die Liste konnte nicht geladen werden.
about-filter = Nach Name oder Lizenz filtern
about-count = { $shown } von { $total }
about-no-license = Keine Lizenz angegeben
about-report = Problem melden…
report-title = Problem melden
report-what = Das braucht ein Issue auf GitHub, damit dem Problem nachgegangen werden kann. Nichts Privates ist darin: Zugriffstokens, Passwörter, Schlüssel, E-Mail-Adressen und dein Name in Ordnerpfaden werden entfernt. Lies es durch, bevor du es abschickst.
report-loading = Bericht wird zusammengestellt…
report-failed = Der Bericht konnte nicht zusammengestellt werden: { $error }
report-log = Was der Server zuletzt protokolliert hat
report-copy = Kopieren
report-copied = Bericht kopiert.
report-github = Issue auf GitHub eröffnen
report-github-hint = Das neue Issue wird damit ausgefüllt. Ist das Protokoll dafür zu lang, kommen nur seine letzten Zeilen hinein; Kopieren enthält alles.
report-close = Schließen

## Asset-Arten (api.ts)

api-kind-texture = Texturen
api-kind-vector = Vektorgrafiken
api-kind-sheet = Spritesheets
api-kind-audio = Audio
api-kind-model = Modelle
api-kind-animation = Animationen
api-kind-font = Schriften
api-kind-shader = Shader
api-kind-material = Materialien
api-kind-video = Video
api-kind-document = Dokumente
api-kind-data = Daten
api-kind-other = Sonstiges
api-kind-texture-name = Texturen
api-kind-vector-name = Vektorgrafiken
api-kind-sheet-name = Spritesheets
api-kind-audio-name = Audio
api-kind-model-name = Modelle
api-kind-animation-name = Animationen
api-kind-font-name = Schriften
api-kind-shader-name = Shader
api-kind-material-name = Materialien
api-kind-video-name = Video
api-kind-document-name = Dokumente
api-kind-data-name = Daten
api-kind-other-name = Sonstiges

## Meldungen, Dialoge und Laden (ui.ts, UiLayer.vue, lazy.ts)

news-button = { $count } neu
news-button-title =
    { $count ->
        [one] { $count } Neuheit in Hoard
       *[other] { $count } Neuheiten in Hoard
    }
news-pip =
    { $count ->
        [one] Neu in Hoard
       *[other] { $count } Neuheiten hier
    }
news-title = Neu in Hoard
news-to-find =
    { $count ->
        [one] { $count } zu entdecken
       *[other] { $count } zu entdecken
    }
news-close = Schließen
news-got-it = Verstanden
news-take-me = Bring mich hin
news-look-again = Noch mal ansehen
news-none = Gerade gibt es nichts Neues zu entdecken.
news-seen-lately = Kürzlich gesehen
news-stop-pointing = Nicht mehr auf Neues hinweisen
news-point-out = Wieder auf Neues hinweisen
news-mark-all = Alle als gesehen markieren
news-marked-seen =
    { $count ->
        [one] { $count } Neuheit als gesehen markiert
       *[other] { $count } Neuheiten als gesehen markiert
    }
news-themes-title = Zehn helle und zehn dunkle Designs
news-themes-body = Himmel, Minze, Aprikose und Zitrone ergänzen die hellen Designs, Mitternacht, Lagune und Neon die dunklen, und der neue Look breitet sich von der Kachel aus, die du wählst.
news-tiles-move-title = Kacheln in Bewegung halten
news-tiles-move-body = Animationen und animierte Bilder auf den Kacheln der Bibliothek können durchgehend laufen, nicht nur unter dem Zeiger.
news-motion-trails-title = Bewegungsspuren bei Animationen
news-motion-trails-body = Das Vorschaubild einer Animation zeichnet den Weg nach, den ihre Figur nimmt, mit einem Punkt dort, wo sie gerade ist.
news-text-size-title = Textgröße und Abstände
news-text-size-body = Mach die Schrift größer oder kleiner und die Seite großzügiger oder kompakter, und wähle, was die Kacheln und die obere Leiste zeigen.
news-tag-tree-title = Tags in Tags
news-tag-tree-body = Stell ein Tag unter ein allgemeineres, dann findet die Suche nach Fahrzeug auch Auto und Limousine. Verschachteln kannst du sie unter Einstellungen → Tags oder auf der Seite eines Tags.
news-bulk-edit-title = Viele auf einmal bearbeiten
news-bulk-edit-body = Wähle ein paar Assets aus und drück E, um ihre Tags, Lizenz, Bewertung und mehr gemeinsam zu ändern, mit Rückgängig.
news-backups-title = Geplante Sicherungen
news-backups-body = Sichere die Datenbank jeden Tag oder jede Woche, behalte die neuesten paar und kopiere jede Sicherung in einen anderen Ordner.
ui-undo = Rückgängig
ui-today = heute
ui-yesterday = gestern
ui-days-ago =
    { $days ->
        [one] vor { $days } Tag
       *[other] vor { $days } Tagen
    }
ui-notifications = Benachrichtigungen
ui-dismiss = Schließen
ui-cancel = Abbrechen
ui-lazy-loading = Lade { $what }…
ui-lazy-failed = Laden fehlgeschlagen: { $what }.
ui-lazy-reload = Seite neu laden

## ResetButton.vue

reset-title = Auf Standard zurücksetzen: { $value }
reset-label = Auf Standard zurücksetzen, { $value }

## SizeInput.vue

size-invalid = Eine Größe wie 100 GB, 512 MB oder 1,5 TB
size-hint = Eine Größe wie 100 GB, 512 MB oder 1,5 TB; eine bloße Zahl bedeutet GB

## AuthPanel.vue (Einstellungen → Authentifizierung)

auth-load-failed = Die Authentifizierungseinstellungen konnten nicht geladen werden: { $error }
auth-saved = Gespeichert. Gilt ab der nächsten Anmeldung.
auth-method-internal = Intern
auth-method-internal-hint = Namen und Passwörter, die Hoard selbst verwaltet, hinzugefügt unter Einstellungen → Konto.
auth-method-ldap-hint = Ein Verzeichnis prüft den Namen und das Passwort, die auf der Anmeldeseite eingegeben werden: OpenLDAP, FreeIPA, LLDAP oder ein anderer LDAP-Server.
auth-method-ad-hint = Windows-Domänenkonten melden sich mit ihrem gewohnten Namen und Passwort an.
auth-method-oauth-hint = Eine Schaltfläche „Anmelden mit…“, die zu Google, Microsoft, GitHub, Keycloak, Authentik oder einem anderen Anbieter und zurück führt.
auth-session-unused = Nach 30 Tagen ohne Nutzung
auth-session-1h = 1 Stunde nach der Anmeldung
auth-session-8h = 8 Stunden nach der Anmeldung
auth-session-day = Einen Tag nach der Anmeldung
auth-session-week = Eine Woche nach der Anmeldung
auth-session-30d = 30 Tage nach der Anmeldung
auth-session-hours =
    { $hours ->
        [one] { $hours } Stunde nach der Anmeldung
       *[other] { $hours } Stunden nach der Anmeldung
    }
auth-role-view = Darf ansehen
auth-role-edit = Darf ändern
auth-role-admin = Admin
auth-role-word-view = darf ansehen
auth-role-word-edit = darf ändern
auth-role-word-admin = Admin
auth-role-word-default = { $role } (die Standardrolle)
auth-tls-starttls-hint = Einfaches LDAP (Port 389), das auf TLS umschaltet, bevor etwas gesendet wird.
auth-tls-ldaps-hint = TLS von Anfang an (Port 636).
auth-tls-none = Keine
auth-tls-none-hint = Passwörter gehen unverschlüsselt über das Netzwerk. Nur in einem Netzwerk, dem Sie vertrauen.
auth-test-failed = Test nicht möglich: { $error }
auth-preset-custom = Andere (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel und andere: Geben Sie den Issuer an, und „Ermitteln“ trägt die Adressen ein.
auth-preset-google-note = Legen Sie in der Google Cloud Console unter APIs & Dienste → Anmeldedaten einen OAuth-Client (Webanwendung) an. Google sendet keine Gruppen; vergeben Sie Rollen daher unter Einstellungen → Konto, und schalten Sie das Anlegen von Konten bei der ersten Anmeldung aus, es sei denn, jeder mit einem Google-Konto soll hineinkommen.
auth-preset-microsoft-note = Registrieren Sie eine App unter Entra ID → App-Registrierungen (mit einer Web-Weiterleitung auf die Adresse unten). Setzen Sie Ihre Mandanten-ID anstelle von „organizations“ ein, um nur Ihre Organisation hereinzulassen. Für Rollen aus Gruppen fügen Sie unter Tokenkonfiguration den Gruppenanspruch hinzu: Er listet Gruppen-IDs.
auth-preset-github-note = Legen Sie in GitHub unter Settings → Developer settings eine OAuth-App an. GitHub sendet keine Gruppen; vergeben Sie Rollen daher unter Einstellungen → Konto, und schalten Sie das Anlegen von Konten bei der ersten Anmeldung aus, es sei denn, jeder mit einem GitHub-Konto soll hineinkommen.
auth-preset-gitlab-note = Legen Sie in GitLab unter Preferences → Applications eine Anwendung an (oder im Admin-Bereich Ihrer Instanz: Ersetzen Sie dann gitlab.com in den Adressen).
auth-discovered = Die Adressen des Anbieters wurden eingetragen.
auth-copied = Kopiert
auth-copy-failed = Kopieren nicht möglich: Markieren und kopieren Sie es selbst
auth-sign-in-with = Anmelden mit
auth-own-accounts-still-work = Hoards eigene Konten melden sich weiterhin mit ihren eigenen Passwörtern an, sodass ein ausgefallenes oder falsch eingerichtetes Verzeichnis bzw. ein solcher Anbieter Sie nicht aussperren kann.
auth-sessions = Sitzungen
auth-sign-out-after = Personen abmelden
auth-ldap-server = LDAP-Server
auth-server-type = Servertyp
auth-server-type-custom = Benutzerdefiniert
auth-servers = Server
auth-server-n = Server { $n }
auth-remove-server-n = Server { $n } entfernen
auth-remove = Entfernen
auth-add-server = Server hinzufügen
auth-servers-hint = Ist einer nicht erreichbar, wird der nächste versucht.
auth-security = Sicherheit
auth-connection-security = Verbindungssicherheit
auth-skip-verify = Das Zertifikat des Servers nicht prüfen (nur zum Ausprobieren)
auth-ca = Zertifizierungsstelle
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (leer: die, denen das System vertraut)
auth-lookup = Personen nachschlagen
auth-anonymously = Anonym
auth-reader-account = Lesekonto
auth-reader-password = Sein Passwort
auth-saved-type-to-replace = Gespeichert: zum Ersetzen tippen
auth-people-under = Personen liegen unter
auth-name-attribute = Namensattribut
auth-only-matching = Nur Einträge, die passen auf
auth-groups-under = Gruppen liegen unter
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (leer: nur memberOf)
auth-their-groups = Ihre Gruppen
auth-group-filter-hint = { "{" }dn{ "}" } ist der Eintrag der Person, { "{" }user{ "}" } der Name, mit dem sie sich angemeldet hat.
auth-test = Testen
auth-test-hint = Erreicht jeden Server und meldet sich mit dem Lesekonto an. Mit Name und Passwort meldet es sich auch als diese Person an und zeigt, was Hoard daraus machen würde. Es wird nichts gespeichert.
auth-test-user = Anmelden als
auth-test-user-placeholder = Anmelden als (optional)
auth-test-password = Ihr Passwort
auth-testing = Wird getestet…
auth-test-connectivity = Verbindung testen
auth-test-server-ok = erreicht, und das Lesekonto hat sich angemeldet
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }hat sich angemeldet (
auth-test-signed-in-after = ).
auth-test-role = Rolle: { $role }.
auth-test-no-role = Die Person ist in keiner der unter „Rollen“ aufgeführten Gruppen und würde daher abgewiesen.
auth-test-groups = Gruppen: { $groups }
auth-test-no-groups = Keine Gruppen gefunden.
auth-provider = Anbieter
auth-redirect = Weiterleitungsadresse
auth-copy = Kopieren
auth-redirect-hint = Geben Sie diese dem Anbieter, wenn Sie den Client für Hoard anlegen. Sie ergibt sich aus der öffentlichen Adresse unter Einstellungen → Verbindung oder aus der Adresse, die Sie gerade verwenden.
auth-issuer = Issuer
auth-discover = Ermitteln
auth-client-id = Client-ID
auth-client-secret = Client-Secret
auth-authorization-url = Autorisierungsadresse
auth-token-url = Token-Adresse
auth-userinfo-url = Benutzerinfo-Adresse
auth-userinfo-placeholder = (leer: Das ID-Token sagt, wer es ist)
auth-scopes = Scopes
auth-name-from = Name aus
auth-groups-from = Gruppen aus
auth-groups-from-placeholder = (keine)
auth-button = Schaltfläche
auth-button-placeholder = Ihrem Konto
auth-accounts = Konten
auth-provision = Bei der ersten Anmeldung einer Person ein Hoard-Konto anlegen
auth-provision-on-hint = Andernfalls fügt ein Admin jede Person zuerst unter Einstellungen → Konto hinzu.
auth-provision-off-hint = Ein Admin fügt jede Person zuerst unter Einstellungen → Konto hinzu und wählt dabei, wie sie sich anmeldet.
auth-open-to-anyone = Jeder mit einem { $provider }-Konto kommt hinein. Fügen Sie unten Betrachtergruppen hinzu, oder schalten Sie dies aus und fügen Sie Personen selbst hinzu.
auth-new-accounts = Neue Konten
auth-roles-from-groups = Rollen aus Gruppen
auth-roles-hint = Gruppen, deren Mitglieder die jeweilige Rolle bekommen, eine pro Zeile, per Name oder DN. Die höchste passende gilt, und das wird bei jeder Anmeldung geprüft. Ist keine aufgeführt, bekommen neue Konten die Rolle oben, und ein Admin ändert sie danach. Sind unter „Darf ansehen“ Gruppen aufgeführt, dürfen sich nur Mitglieder einer aufgeführten Gruppe anmelden.
auth-roles-hint-oauth = Gruppen, deren Mitglieder die jeweilige Rolle bekommen, eine pro Zeile, per Name oder DN (oder was immer der Gruppenanspruch auflistet). Die höchste passende gilt, und das wird bei jeder Anmeldung geprüft. Ist keine aufgeführt, bekommen neue Konten die Rolle oben, und ein Admin ändert sie danach. Sind unter „Darf ansehen“ Gruppen aufgeführt, dürfen sich nur Mitglieder einer aufgeführten Gruppe anmelden.
auth-saving = Wird gespeichert…
auth-save = Speichern

## HttpsPanel.vue (Einstellungen → Verbindung)

https-load-failed = HTTPS konnte nicht geladen werden: { $error }
https-off-redirecting = HTTPS ist aus. Weiter zu http://
https-now-on = HTTPS ist an
https-saved = Gespeichert. Neue Verbindungen verwenden das neue Zertifikat.
https-now-off = HTTPS ist aus
https-http3-failed-toast = HTTP/3 konnte nicht starten: { $error }
https-copied = Kopiert
https-copy-failed = Kopieren nicht möglich: Markieren und kopieren Sie es selbst
https-mode-off = Aus
https-mode-off-hint = Nur einfaches http://.
https-mode-own = Hoards eigenes Zertifikat
https-mode-own-hint = Hoard erstellt eines für die Adressen dieses Servers. Installieren Sie Hoards Zertifizierungsstelle einmal auf jedem Gerät, dann vertrauen dessen Browser ihm.
https-mode-custom = Eigenes Zertifikat
https-mode-custom-hint = Eines, das Sie bereits haben, z. B. von tailscale cert oder Let’s Encrypt. Browser vertrauen ihm, ohne dass etwas installiert werden muss.
https-untrusted-own = Dieser Browser vertraut ihm für { $host } noch nicht: Installieren Sie unten Hoards Zertifizierungsstelle. Sie sehen eine Warnung, die Sie wegklicken können.
https-untrusted-uncovered = Dieser Browser vertraut ihm für { $host } noch nicht: Das Zertifikat deckt diese Adresse nicht ab. Sie sehen eine Warnung, die Sie wegklicken können.
https-untrusted = Dieser Browser vertraut ihm für { $host } noch nicht. Sie sehen eine Warnung, die Sie wegklicken können.
https-now = Aktuell
https-start-failed = HTTPS ist eingerichtet, konnte aber nicht starten: { $error }
https-on-port = HTTPS ist an, auf Port { $port } wie http://.
https-using-it = Sie verwenden es gerade.
https-open = { $url } öffnen
https-trusted = Dieser Browser vertraut ihm.
https-cert-for-before = Zertifikat für{ " " }
https-cert-for-after = , gültig bis { $date }.
https-cert-for-after-own = , gültig bis { $date } (erneuert sich selbst).
https-not-covered-before = Es deckt{ " " }
https-not-covered-after = { " " }nicht ab, die Adresse, die Sie verwenden.
https-is-off = HTTPS ist aus: Seiten werden über einfaches http:// ausgeliefert.
https-addresses = Adressen
https-addresses-hint = Das Zertifikat deckt die Adressen ab, die Hoard gefunden hat, und alle, die Sie hinzufügen (einen Namen, den Ihr Router vergibt, einen Tailscale-Namen).
https-remove-name = { $name } entfernen
https-another-address = Weitere Adresse
https-another-address-placeholder = Weitere Adresse, z. B. nas.local
https-add = Hinzufügen
https-your-cert = Ihr Zertifikat
https-saved-cert-before = Gespeichert:{ " " }
https-saved-cert-after = { " " }von { $issuer }, gültig bis { $date }. Fügen Sie ein neues ein, um es zu ersetzen.
https-pem-before = Das Zertifikat (mit seiner Kette) und sein privater Schlüssel als PEM-Text: die Dateien, die beginnen mit{ " " }
https-pem-after = .
https-certificate = Zertifikat
https-choose-file = Datei wählen…
https-private-key = Privater Schlüssel
https-checked-before = Für{ " " }
https-checked-after = , von { $issuer }, gültig bis { $date }.
https-checked-after-days =
    , von { $issuer }, gültig bis { $date } ({ $days ->
        [one] { $days } Tag
       *[other] { $days } Tage
    }).
https-self-signed = Es ist selbstsigniert: Browser werden davor warnen.
https-key-matches = Der Schlüssel passt.
https-where-to-get = Woher Sie eines bekommen
https-get-tailscale-before = Aktivieren Sie HTTPS für Ihr Tailnet, führen Sie dann{ " " }
https-get-tailscale-after = { " " }auf dem Server aus und laden Sie die beiden Dateien hoch. Sie gelten 90 Tage: Führen Sie den Befehl vorher erneut aus und laden Sie sie erneut hoch.
https-get-acme-title = Let’s Encrypt oder ein anderer ACME-Client
https-get-acme-before = (certbot, acme.sh, lego) für eine Domain, die Ihnen gehört: Laden Sie{ " " }
https-get-acme-and = { " " }und{ " " }
https-get-acme-after = { " " }hoch.
https-get-proxy-title = Ein Reverse-Proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), der das Zertifikat hat: Lassen Sie HTTPS hier aus, und lassen Sie den Proxy einfaches http:// an Hoard senden.
https-trying = Wird ausprobiert…
https-apply = Übernehmen
https-apply-hint = Hoard probiert die neue Einrichtung zuerst aus und behält die bisherige, falls sie nicht funktioniert.
https-trust-title = Hoard auf Ihren Geräten vertrauen
https-download-ca = hoard-ca.crt herunterladen
https-trust-hint = Installieren Sie Hoards Zertifizierungsstelle einmal auf jedem Gerät, dann vertrauen dessen Browser diesem Server (und jedem Zertifikat, das Hoard später dafür erstellt). Sie kann nur für dieses Hoard bürgen. Fingerabdruck (SHA-256):
https-ca-windows = Datei öffnen → Zertifikat installieren → Aktueller Benutzer → Alle Zertifikate in folgendem Speicher speichern → Durchsuchen → Vertrauenswürdige Stammzertifizierungsstellen → Fertig stellen. Starten Sie den Browser neu.
https-ca-macos = Öffnen Sie die Datei; sie landet in der Schlüsselbundverwaltung. Doppelklicken Sie dort auf „{ $subject }“ → Vertrauen → Bei Verwendung dieses Zertifikats: Immer vertrauen.
https-ca-ios-title = iPhone und iPad
https-ca-ios = Laden Sie sie in Safari herunter und erlauben Sie es. Einstellungen → Profil geladen → Installieren. Dann Einstellungen → Allgemein → Info → Zertifikatsvertrauenseinstellungen → „{ $subject }“ aktivieren.
https-ca-android = Einstellungen → Sicherheit → Weitere Sicherheitseinstellungen → Verschlüsselung und Anmeldedaten → Zertifikat installieren → CA-Zertifikat, und wählen Sie die Datei. (Die Namen unterscheiden sich je nach Telefon etwas; suchen Sie in den Einstellungen nach „Zertifikat“.)
https-ca-linux-1 = Debian und Ubuntu:{ " " }
https-ca-linux-2 = . Fedora und Arch:{ " " }
https-ca-linux-3 = . Chrome unter Linux führt eine eigene Liste: Einstellungen → Datenschutz und Sicherheit → Sicherheit → Zertifikate verwalten → Importieren.
https-ca-firefox = Firefox führt eine eigene Liste: Einstellungen → Datenschutz & Sicherheit → Zertifikate → Zertifikate anzeigen → Zertifizierungsstellen → Importieren, und setzen Sie den Haken bei „Dieser CA vertrauen, um Websites zu identifizieren“.
https-ca-cli-title = Das Kommandozeilenwerkzeug hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , oder tragen Sie den Pfad in{ " " }
https-ca-cli-after = { " " }ein.
https-options = Optionen
https-redirect = Browser auf anderen Geräten von http:// zu https:// umleiten
https-redirect-hint = Nur Seitenaufrufe werden umgeleitet. Das Kommandozeilenwerkzeug hoard, der Unreal-Worker, Skripte und KI-Apps funktionieren weiter über http://, ebenso http://localhost auf dem Server selbst.
https-http3-hint-before = Schneller bei wackligem WLAN und auf Telefonen. Braucht HTTPS und neben TCP auch offenen UDP-Port { $port } (in Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Läuft.
https-http3-waits = Startet, sobald HTTPS an ist.
https-http3-failed = HTTP/3 konnte nicht starten: { $error }
https-lockout-title = Falls Sie sich aussperren
https-lockout-1 = { "" }
https-lockout-2 = { " " }auf dem Server selbst funktioniert immer über einfaches http://. Oder schalten Sie HTTPS über die Kommandozeile des Servers aus und starten Sie Hoard neu:{ " " }
https-lockout-3 = { " " }(in Docker:{ " " }
https-lockout-4 = , dann{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Einstellungen → Konto)

account-role-view = Darf ansehen
account-role-view-hint = Suchen, ansehen und herunterladen
account-role-edit = Darf ändern
account-role-edit-hint = Außerdem hinzufügen, ändern, taggen, sammeln und archivieren
account-role-admin = Admin
account-role-admin-hint = Außerdem Einstellungen, Konten und Wartung sowie endgültiges Löschen
account-source-internal = Ein Hoard-Passwort
account-source-ldap = Das eigene { $directory }-Passwort
account-source-oauth = Anmeldung mit { $provider }
account-source-word-directory = Verzeichnis
account-load-failed = Die Konten konnten nicht geladen werden: { $error }
account-failed = { $what }: { $error }
account-password-failed = Ihr Passwort konnte nicht geändert werden
account-password-changed = Passwort geändert. Ihre anderen Geräte sind abgemeldet.
account-add-failed = { $name } konnte nicht hinzugefügt werden
account-added = { $name } kann sich jetzt anmelden.
account-role-failed = Die Rechte von { $name } konnten nicht geändert werden
account-remove-title = { $name } entfernen?
account-remove-body = Die Person wird überall abgemeldet und kann sich nicht wieder anmelden. Nichts, was sie der Bibliothek hinzugefügt hat, wird entfernt.
account-remove = Entfernen
account-remove-failed = { $name } konnte nicht entfernt werden
account-token-failed = Das Token konnte nicht erstellt werden
account-copied = Kopiert
account-copy-token-failed = Kopieren nicht möglich: Markieren und kopieren Sie das Token selbst
account-copy-failed = Kopieren nicht möglich: Markieren und kopieren Sie es selbst
account-revoke-title = „{ $name }“ widerrufen?
account-revoke-body = Alles, was es noch verwendet, wird ab sofort abgewiesen.
account-revoke = Widerrufen
account-revoke-failed = „{ $name }“ konnte nicht widerrufen werden
account-disconnect-title = { $app } trennen?
account-disconnect-body = Die App wird ab sofort abgewiesen und muss neu verbunden werden, um Hoard zu nutzen.
account-disconnect = Trennen
account-disconnect-failed = { $app } konnte nicht getrennt werden
account-guest-failed = Das konnte nicht geändert werden
account-never = nie
account-you = Sie
account-sign-out = Abmelden
account-signed-in-as-before = Angemeldet als{ " " }
account-signed-in-as-admin = , Admin.
account-signed-in-as-edit = , darf die Bibliothek ändern.
account-signed-in-as-view = , darf ansehen, aber nichts ändern.
account-app-signin = Die Hoard-App meldet Sie selbst an, deshalb gibt es kein Passwort. Erstellen Sie unten ein Zugriffstoken für die Engine-Plugins und den Befehl hoard.
account-password-at-provider = Ihr Passwort wird von dem Anbieter verwaltet, mit dem Sie sich anmelden; ändern Sie es dort.
account-password-at-directory = Ihr Passwort wird von dem Verzeichnis verwaltet, mit dem Sie sich anmelden; ändern Sie es dort.
account-current-password = Aktuelles Passwort
account-new-password = Neues Passwort
account-new-password-again = Neues Passwort wiederholen
account-change-password = Passwort ändern
account-password-mismatch = Die neuen Passwörter stimmen nicht überein.
account-password-rule = Mindestens 8 Zeichen.
account-people = Personen
account-people-hint = Jede Person darf ansehen (suchen, ansehen und herunterladen), zusätzlich die Bibliothek ändern (hinzufügen, bearbeiten, taggen, sammeln und archivieren) oder außerdem Admin sein (Einstellungen, Konten und Wartung sowie endgültiges Löschen).
account-you-tag = (Sie)
account-signs-in-oauth = Meldet sich über den OAuth-Anbieter an
account-signs-in-directory = Meldet sich über das Verzeichnis an
account-since = seit { $date }
account-what-can-do = Was { $name } darf
account-name = Name
account-how-sign-in = Anmeldung über
account-password-label = Passwort (mindestens 8 Zeichen)
account-password-placeholder = Passwort (8+ Zeichen)
account-what-they-can-do = Rechte
account-add = Hinzufügen
account-ai-apps = Claude und andere KI-Apps
account-ai-apps-hint = Verbinden Sie Claude (oder eine beliebige MCP-App), dann kann es für Sie Assets suchen und abrufen und beschreiben und taggen, was Hoard noch nicht erfasst hat. Beim ersten Mal meldet es sich bei Hoard an; Sie erlauben es hier im Browser.
account-mcp-address = MCP-Adresse von Hoard
account-copy = Kopieren
account-claude-code-1 = Führen Sie{ " " }
account-claude-code-2 = { " " }aus, dann{ " " }
account-claude-code-3 = { " " }zur Anmeldung.{ " " }
account-claude-code-4 = { " " }beschreibt ein Paket.
account-claude-ai = Einstellungen → Konnektoren → Benutzerdefinierten Konnektor hinzufügen, Adresse einfügen, dann Verbinden. Diese verbinden sich von den Servern von Anthropic aus, daher muss Hoard über HTTPS aus dem Internet erreichbar sein.
account-claude-ai-local = Einstellungen → Konnektoren → Benutzerdefinierten Konnektor hinzufügen, Adresse einfügen, dann Verbinden. Diese verbinden sich von den Servern von Anthropic aus, daher muss Hoard über HTTPS aus dem Internet erreichbar sein (diese Adresse ist es nicht: Legen Sie die öffentliche Adresse unter Einstellungen → Verbindung fest).
account-connection-meta = für { $user } seit { $since } · zuletzt verwendet { $used }
account-no-apps = Noch keine Apps verbunden.
account-live = Nutzen Hoard gerade
account-live-hint = Apps, die Hoard in den letzten 10 Minuten etwas gefragt haben. Die Liste aktualisiert sich selbst.
account-live-none = In den letzten 10 Minuten hat nichts Hoard etwas gefragt.
account-live-last = zuletzt: { $what } · { $when } · Anfragen: { $calls }
account-tokens = Zugriffstokens
account-tokens-hint-1 = Für das Kommandozeilenwerkzeug{ " " }
account-tokens-hint-2 = , Skripte, MCP-Clients und den Unreal-Worker: Diese senden es als{ " " }
account-tokens-hint-3 = . Ein Token kann, was Sie festlegen, höchstens so viel wie Sie, aber niemals Konten oder Tokens verwalten. Eines, das nur ansehen darf, genügt zum Suchen und Herunterladen und zum Synchronisieren eines Projekts.
account-tokens-admin = Sie sehen die Tokens und verbundenen Apps aller Personen; andere sehen nur ihre eigenen.
account-made-before = { "" }
account-made-after = : Kopieren Sie es jetzt. Es wird nicht noch einmal angezeigt.
account-new-token = Das neue Token
account-cli-1 = Für die CLI:{ " " }
account-cli-2 = , oder tragen Sie es in{ " " }
account-cli-3 = { " " }ein.
account-done = Fertig
account-token-meta = erstellt { $made } · zuletzt verwendet { $used }
account-token-meta-by = erstellt { $made } von { $by } · zuletzt verwendet { $used }
account-no-tokens = Noch keine Tokens.
account-token-name = Wofür es verwendet wird
account-token-name-placeholder = Wofür es verwendet wird, z. B. „Unreal-Worker“
account-token-role = Was es darf
account-make-token = Token erstellen
account-guests = Gäste
account-guests-allow = Alle, die den Server erreichen, ohne Anmeldung stöbern lassen
account-guests-hint = Sie können suchen, ansehen und herunterladen, aber nichts hinzufügen, ändern oder löschen und die Einstellungen nicht sehen.

## SignIn.vue

signin-create-title = Konto erstellen
signin-title = Anmelden
signin-setup-lede = Dieser Server hat noch kein Konto. Das Konto, das Sie hier erstellen, darf alles; Konten für andere Personen fügen Sie später unter Einstellungen → Konto hinzu.
signin-overlay-lede = Stöbern ist hier für alle offen, Änderungen brauchen aber ein Konto.
signin-with = Mit { $provider } anmelden
signin-or-password = oder mit Name und Passwort
signin-username = Benutzername
signin-name = Name
signin-new-password = Neues Passwort
signin-password = Passwort
signin-password-rule = Mindestens 8 Zeichen.
signin-confirm-password = Passwort bestätigen
signin-mismatch = Die beiden Passwörter stimmen nicht überein.
signin-setup-code = Einrichtungscode
signin-setup-code-hint-before = Der Code, den dieser Server in{ " " }
signin-setup-code-hint-after = { " " }bekommen hat.
signin-creating = Wird erstellt…
signin-signing-in = Anmeldung läuft…
signin-create = Konto erstellen
signin-submit = Anmelden
signin-keep-browsing = Weiter stöbern
signin-directory-hint = Verwenden Sie Ihren { $directory }-Namen und Ihr Passwort.
signin-forgot-1 = Hoard-Passwort vergessen? Führen Sie auf dem Server{ " " }
signin-forgot-2 = { " " }aus (in Docker:{ " " }
signin-forgot-3 = ), um ein neues festzulegen.

## SetupWizard.vue (erster Besuch auf einem neuen Server)

setup-title = Wo soll Ihre Bibliothek liegen?
setup-lede = Wählen Sie einen Ordner auf dem Server, in dem Hoard Ihre Pakete aufbewahrt, oder verweisen Sie es auf einen Ordner mit Paketen, die Sie bereits haben. Weitere Speicherorte auf anderen Datenträgern können Sie später in den Einstellungen hinzufügen.
setup-lede-desktop = Wählen Sie einen Ordner, in dem Hoard Ihre Pakete aufbewahrt, oder verweisen Sie es auf einen Ordner mit Paketen, die Sie bereits haben. Weitere Speicherorte auf anderen Datenträgern können Sie später in den Einstellungen hinzufügen.
setup-unreachable = Der Server war nicht erreichbar: { $error }
setup-unreachable-desktop = Hoard antwortet nicht: { $error }
setup-try-again = Erneut versuchen
setup-loading = Wird geladen…
setup-start = Bibliothek anlegen
setup-ready = Ihre Bibliothek ist bereit
setup-managed-before = Neue Pakete kommen nach{ " " }
setup-managed-after = . Legen Sie Dateien, Ordner oder Archive irgendwo auf der Seite ab, um die ersten hinzuzufügen.
setup-watched-before = Hoard überwacht{ " " }
setup-watched-after = ; Pakete erscheinen, sobald es sie findet. Um auch hochladen zu können, fügen Sie unter Einstellungen → Speicherorte eine Bibliothek hinzu, die Hoard verwaltet.
setup-open = Bibliothek öffnen

## ConnectApp.vue (eine App wie Claude bittet um Anmeldung)

connect-title = { $app } verbinden?
connect-lede-1 = { " " }kann dann alles in diesem Hoard durchsuchen, herunterladen, beschreiben und taggen, und zwar als{ " " }
connect-lede-2 = . Konten und Zugriffstokens kann es nicht verwalten.
connect-back-before = Sie kehren zurück zu{ " " }
connect-back-after = . Sie können die Verbindung jederzeit unter Einstellungen → Konto trennen.
connect-allow = Erlauben
connect-cancel = Abbrechen
connect-failed = Diese App kann nicht verbunden werden
connect-go-library = Zur Bibliothek
connect-checking = Wird geprüft…

## InsecureNotice.vue (der Hinweis über einfachem http://)

insecure-text = Dies ist einfaches http://, und einige Dinge brauchen HTTPS: 3D mit WebGPU, die moderne Zwischenablage, Hoard als App installieren und Claude-Konnektoren.
insecure-switch = Zu https:// wechseln
insecure-set-up = HTTPS einrichten
insecure-dismiss = Nicht mehr anzeigen

## FormatWizard.vue (Einstellungen → Store-Dateien → Hoard die Datei eines Stores beibringen)

formatwizard-too-big = Diese Datei ist zu groß, um als Store-Datei gelesen zu werden.
formatwizard-not-json = Diese Datei ist kein gültiges JSON und wird daher als Text angezeigt.
formatwizard-read-failed = Die Datei konnte nicht gelesen werden: { $error }
formatwizard-back-title = Zurück zum Formular?
formatwizard-back-body = Änderungen im TOML gehen verloren: Das Formular schreibt das Format aus seinen eigenen Angaben neu.
formatwizard-back-to-form = Zurück zum Formular
formatwizard-miss-name = Seine Dateien umfassen den Namen { $file } nicht. Fügen Sie ihn unter „Dateien“ hinzu.
formatwizard-miss-not-the-stores = Woran es die Dateien des Stores erkennt, steht nicht in dieser Datei. Ändern Sie „Erkennen an“.
formatwizard-miss-no-title = Es findet keinen Titel in dieser Datei. Wählen Sie den Titel.
formatwizard-miss-short-name = nicht sein Name
formatwizard-miss-short-not-the-stores = nicht vom Store
formatwizard-miss-short-no-title = kein Titel
formatwizard-saved-packs =
    { $file } gespeichert. { $packs ->
        [one] { $packs } Paket wird
       *[other] { $packs } Pakete werden
    } neu gelesen.
formatwizard-saved = { $file } gespeichert.
formatwizard-save-failed = Das Format konnte nicht gespeichert werden: { $error }
formatwizard-zip-failed = Das Zip konnte nicht erstellt werden: { $error }
formatwizard-stop-title = Das Beibringen dieser Datei beenden?
formatwizard-stop-body = Das Format wird nicht gespeichert.
formatwizard-stop = Beenden
formatwizard-label = Format-Assistent
formatwizard-changing = { $name } wird geändert
formatwizard-starting-from = Ausgehend von { $name }
formatwizard-title = Hoard die Datei eines Stores beibringen
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }im Paket{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Als TOML bearbeiten
formatwizard-file-reference = Die Datei, zum Nachschlagen.
formatwizard-click-json = Klicken Sie in der Datei auf { $field }.
formatwizard-click-text = Klicken Sie in der Datei auf { $field }, oder markieren Sie es.
formatwizard-pick-first = Wählen Sie rechts ein Feld, und klicken Sie dann hier auf seinen Wert.
formatwizard-loading = Wird geladen…
formatwizard-the-format = Das Format
formatwizard-syntax-before = Die Syntax steht in{ " " }
formatwizard-syntax-after = .
formatwizard-store = Store
formatwizard-store-placeholder = Der Name des Stores, etwa Kenney
formatwizard-files = Dateien
formatwizard-recognize = Erkennen an
formatwizard-recognize-json = Ein Schlüssel, den nur die Dateien des Stores haben
formatwizard-recognize-text = Eine Zeile, die nur die Dateien des Stores haben
formatwizard-recognize-hint = Eine Möglichkeit pro Zeile; jede davon genügt. Nötig, wenn andere Dateien denselben Namen haben.
formatwizard-seller = Verkäufer
formatwizard-seller-placeholder = Wenn die Datei es nicht angibt
formatwizard-tags = Tags
formatwizard-tags-placeholder = Für alles, was sie beschreibt, durch Kommas getrennt
formatwizard-fields = Felder
formatwizard-selector = Selektor für { $field }
formatwizard-clear = { $field } leeren
formatwizard-pick = Wählen
formatwizard-what-it-reads = Was es liest
formatwizard-read-title = Titel
formatwizard-read-seller = Verkäufer
formatwizard-read-license = Lizenz
formatwizard-read-url = Store-Seite
formatwizard-read-kind = Art
formatwizard-read-tags = Tags
formatwizard-read-origin = Hergestellt in
formatwizard-read-size = Größe
formatwizard-also = { $format } liest diese Datei ebenfalls. Dieses Format würde zuerst versucht.
formatwizard-library =
    Einen Namen, den es annimmt, { $files ->
        [one] hat { $files } Datei
       *[other] haben { $files } Dateien
    } in { $packs ->
        [one] { $packs } Paket
       *[other] { $packs } Paketen
    }. Von { $tried } versuchten liest es { $read }.
formatwizard-library-none = Keine Datei in der Bibliothek hat einen Namen, den es annimmt.
formatwizard-share = Teilen
formatwizard-share-hint = Ein Zip mit dem Format, dieser Datei als Beispiel und dem, was aus dem Beispiel gelesen wird, aufgebaut wie Hoards Repository, zum Einsenden, damit alle es bekommen. Entfernen Sie vorher alles aus dem Beispiel, was nicht geteilt werden soll.
formatwizard-sample = Zu teilendes Beispiel
formatwizard-download = { $file } herunterladen
formatwizard-close = Schließen
formatwizard-use = Hier verwenden
formatwizard-cancel = Abbrechen
settings-unreal-editor-what = Installiert Unreal Editor für den Export-Worker. Starte den Worker separat, um Assets zu exportieren.
settings-unreal-download = Lädt Epics Linux-x86-64-Editor einmalig in Hoards Tools-Ordner herunter. Er belegt mehrere Dutzend GB Speicherplatz und braucht ein GitHub-Konto, das Epic zugelassen hat.
settings-unreal-github-step-link-go = So verknüpfen Sie sie
settings-unreal-github-step-token = Erstellen Sie ein GitHub-Token (classic), bei dem nur read:packages angehakt ist.
settings-unreal-github-step-token-go = Token erstellen
settings-unreal-github-user = GitHub-Benutzername
settings-unreal-github-token = Token
settings-unreal-github-token-kept = Gespeichert (geben Sie ein neues ein, um es zu ersetzen)
settings-unreal-github-save = Anmeldung speichern
settings-unreal-github-forget = Trennen
settings-unreal-github-saved = GitHub-Anmeldung gespeichert; der Download wird erneut versucht
settings-unreal-github-forgotten = GitHub getrennt
settings-unreal-github-from-server = Dieser Server hat außerdem eine eigene Anmeldung erhalten (GHCR_TOKEN_FILE oder DOCKER_CONFIG); sie wird verwendet, wenn hier keine gespeichert ist.
settings-unreal-github-connect = GitHub verbinden…
settings-unreal-github-connect-title = GitHub verbinden
settings-unreal-github-connect-why = Epic gibt Unreal Editor an GitHub-Konten weiter, die mit Epic Games verknüpft sind. Hoard bittet nur darum, Pakete zu lesen.
settings-unreal-github-sign-in = Mit GitHub anmelden
settings-unreal-github-enter-code = Nach dem Klick öffnet sich GitHub in einem neuen Tab. Gib dort diesen Code ein.
settings-unreal-github-waiting = Warte auf GitHub…
settings-unreal-github-expired = Dieser Code ist abgelaufen.
settings-unreal-github-denied = Die Anmeldung wurde bei GitHub abgebrochen.
settings-unreal-github-failed = Von GitHub kam kein Code.
settings-unreal-github-again = Neuen Code holen
settings-unreal-github-before = Bevor du loslegst
settings-unreal-github-before-what = Verknüpfe GitHub mit deinem Epic-Games-Konto und nimm die EpicGames-Einladung an, die GitHub dir per E-Mail schickt.
settings-unreal-github-use-token = Stattdessen ein Token verwenden
settings-unreal-github-use-sign-in = Stattdessen mit GitHub anmelden
settings-unreal-github-no-password = Kein Passwort erreicht Hoard
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = Verbunden als { $user }; der Download wird erneut versucht
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = Verbunden als { $user }
settings-unreal-github-as-what = Unreal Editor wird mit diesem GitHub-Konto heruntergeladen.
settings-unreal-github-none = GitHub ist nicht verbunden
settings-unreal-github-none-what = Nötig, um den Editor herunterzuladen.
settings-unreal-github-change = Ändern…
settings-unreal-image = Unreal-Version oder Image
settings-editor-path = Gib den Pfad zur ausführbaren Datei deiner installierten Version von { $tool } an. Die automatische Installation ist nur in Docker verfügbar.
error-unreal-image = Gib eine Unreal-5-Version, ein Epic-dev/dev-slim-Versionstag oder eine Epic-Image-Referenz mit SHA-256 an.
error-github-no-app = Dieser Server hat keine GitHub-App für die Anmeldung; verwende stattdessen ein Token.
error-github-unreachable = GitHub ist nicht erreichbar.
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub hat die Anmeldung abgelehnt: { $reason }
