## Błędy serwera

error-account-not-found = nie znaleziono konta
error-asset-gone = nie znaleziono zasobu
error-asset-not-found = nie znaleziono zasobu { $id }
error-blob-not-found = nie znaleziono obiektu blob { $hash }
error-collection-not-found = nie znaleziono kolekcji { $id }
error-connection-not-found = nie znaleziono połączenia
error-derivative-not-found = nie znaleziono wersji pochodnej { $kind } zasobu { $id }
error-file-in-pack-not-found = nie znaleziono { $path } w paczce { $id }
error-format-not-found = nie znaleziono formatu { $file }
error-frame-sheet-aseprite-only = arkusz klatek mają tylko pliki Aseprite
error-frame-sheet-frames = to ma jedną klatkę albo zbyt wiele, by je rozłożyć
error-location-not-found = nie znaleziono lokalizacji { $id }
error-maintenance-task-not-found = nie znaleziono zadania konserwacji { $id }
error-open-upload-not-found = nie znaleziono otwartego przesyłania { $id }
error-original-not-found = nie znaleziono oryginalnego pliku zasobu { $id }
error-pack-not-found = nie znaleziono paczki { $id }
error-preview-too-big = ten obraz jest zbyt duży, by zrobić z niego lekki podgląd
error-project-not-found = nie znaleziono projektu { $name }
error-saved-search-not-found = nie znaleziono zapisanego wyszukiwania { $id }
error-sign-in-first = najpierw należy się zalogować (lub wysłać token dostępu)
error-those-assets-not-found = nie znaleziono tych zasobów
error-thumbnail-not-found = nie znaleziono miniatury zasobu { $id }
error-token-not-found = nie znaleziono tokenu
error-upload-not-found = nie znaleziono przesyłania { $id }
error-pack-needs-name = paczka musi mieć nazwę
error-no-library = nie ma jeszcze lokalizacji biblioteki do przechowywania plików: należy dodać zarządzaną w Ustawienia → Lokalizacje biblioteki
error-upload-failed = przesyłanie nie powiodło się: { $reason }
error-upload-no-files = brak plików w przesyłaniu
error-analyze-nothing-to-do = nic do zrobienia: rules=false, a LLM jest wyłączony
error-too-many-assets = zbyt wiele zasobów w jednym żądaniu
error-too-many-items = zbyt wiele elementów w jednym żądaniu
error-not-a-hash = to nie jest skrót blake3 w zapisie szesnastkowym
error-merge-into-tag = into musi być tagiem
error-nothing-to-merge = nic do scalenia
error-selection-size = należy podać od 1 do { $max } identyfikatorów zasobów
error-collection-needs-name = kolekcja musi mieć nazwę
error-collection-exists = kolekcja o nazwie { $name } już istnieje
error-collection-empty = kolekcja { $name } jest pusta
error-conversion-failed = nie udało się przekonwertować do { $format }: { $reason }
error-max-size = max musi być potęgą dwójki od 64 do 16384
error-replace-ext = ext musi być krótkim rozszerzeniem pliku
error-empty-file = pusty plik
error-imitation-silent = Nic w nagraniu nie wybija się ponad tło. Spróbuj ponownie bliżej mikrofonu.
error-replace-duplicate = przekonwertowany plik powiela plik, który już jest w paczce; pakiet został z nim scalony
error-ai-address = Adres AI powinien zaczynać się od http:// lub https://
error-local-llm-action = action to list, download lub delete, a dwie ostatnie wymagają modelu
error-role = Rola to view, edit lub admin
error-admin-only = tylko administrator może to zrobić
error-view-only = to konto może przeglądać, ale niczego nie może zmieniać: administrator może to zmienić w Ustawienia → Konto
error-portal-only = kontami i tokenami można zarządzać po zalogowaniu się do portalu
error-not-from-portal = to żądanie nie pochodzi z portalu Hoard
error-account-name = Nazwa może mieć maksymalnie 64 znaki
error-password-length = Hasło musi mieć co najmniej { $min } znaków
error-too-many-codes = Zbyt wiele błędnych kodów: należy odczekać { $seconds } s
error-already-set-up = Konto już istnieje: należy się zalogować
error-too-many-tries = Zbyt wiele błędnych prób: należy odczekać { $seconds } s
error-directory-unavailable = Katalog nie mógł sprawdzić hasła: { $reason }
error-password-elsewhere = Hasło przechowuje katalog lub dostawca, przez którego następuje logowanie
error-current-password = Obecne hasło jest nieprawidłowe
error-account-source = Konto loguje się przez Hoard (internal), katalog (ldap) lub OAuth
error-account-exists = Konto o tej nazwie już istnieje
error-own-role = Własną rolę może zmienić tylko inny administrator
error-remove-self = Nie można usunąć konta, na które jest się zalogowanym
error-token-name = Token należy nazwać od tego, co będzie go używać, w maksymalnie 80 znakach
error-token-role = Token nie może mieć większych uprawnień niż konto, które go tworzy
error-wrong-setup-code = To nie jest kod konfiguracji podany serwerowi w HOARD_SETUP_CODE
error-wrong-password = Nazwa i hasło nie pasują do siebie
error-folder-missing = { $path } nie istnieje (niezamontowany dysk?)
error-s3-path = lokalizacja S3 wygląda tak: s3://bucket
error-s3-address = adres S3 musi zaczynać się od http:// lub https://
error-s3-header = dodatkowy nagłówek S3 wygląda tak: Name=value, a „{ $header }” taki nie jest
error-keychain = nie udało się zachować tego w systemowym pęku kluczy: { $reason }
error-not-built = ta kopia Hoarda została zbudowana bez { $feature }
error-openai-auth-unreachable = nie udało się połączyć z auth.openai.com
error-openai-unexpected = nieoczekiwana odpowiedź od OpenAI
error-openai-answered = OpenAI odpowiedziało { $status }: { $reason }
error-chatgpt-id-token-not-jwt = token ID nie jest tokenem JWT
error-chatgpt-id-token-issuer = token ID pochodzi od { $issuer }, a nie od { $expected }
error-chatgpt-id-token-audience = token ID jest przeznaczony dla innej aplikacji
error-chatgpt-id-token-expired = token ID wygasł
error-chatgpt-id-token-nonce = token ID nie należy do tego logowania
error-chatgpt-no-state = adres nie zawiera parametru state; należy skopiować cały adres
error-chatgpt-not-started = to logowanie nie zostało rozpoczęte tutaj albo rozpoczęto je ponad 15 minut temu. Należy rozpocząć je ponownie w Ustawieniach.
error-chatgpt-canceled = logowanie zostało anulowane
error-chatgpt-refused = ChatGPT odrzucił logowanie: { $reason }
error-chatgpt-no-code = adres nie zawiera kodu logowania; należy skopiować cały adres
error-chatgpt-other-registration = ChatGPT odpowiedział dla innej rejestracji tego serwera; należy rozpocząć logowanie ponownie
error-chatgpt-other-registration-desktop = ChatGPT odpowiedział dla innej kopii Hoard; należy rozpocząć logowanie ponownie
error-chatgpt-not-registered = ChatGPT nie dokończył rejestracji Hoard; należy rozpocząć logowanie ponownie
error-chatgpt-code-used = kod logowania został już użyty lub wygasł; należy zalogować się ponownie
error-chatgpt-refused-code = ChatGPT odrzucił logowanie ({ $code })
error-chatgpt-no-id-token = ChatGPT nie wysłał tokenu ID
error-chatgpt-plan-not-allowed = Zalogowano, ale nie zezwolono na korzystanie z planu. Należy zalogować się ponownie i zezwolić na to albo użyć klucza API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = brak zalogowania do ChatGPT; można się zalogować w Ustawienia → Tagowanie AI
error-chatgpt-refresh-later = ChatGPT nie mógł teraz odświeżyć logowania ({ $code }); nastąpi ponowna próba później
error-chatgpt-signed-out-problem = ChatGPT wylogował ten serwer ({ $code }). Aby dalej tagować w ramach planu, należy zalogować się ponownie.
error-chatgpt-signed-out-problem-desktop = ChatGPT wylogował Hoard ({ $code }). Aby dalej tagować w ramach planu, należy zalogować się ponownie.
error-chatgpt-signed-out = ChatGPT wylogował ten serwer ({ $code }); należy zalogować się ponownie w Ustawienia → Tagowanie AI
error-chatgpt-signed-out-desktop = ChatGPT wylogował Hoard ({ $code }); należy zalogować się ponownie w Ustawienia → Tagowanie AI
error-openai-api-unreachable = nie udało się połączyć z api.openai.com
error-openai-broke-off = odpowiedź od OpenAI urwała się
error-chatgpt-kept-refusing = OpenAI wielokrotnie odrzucało logowanie do ChatGPT; należy zalogować się ponownie w Ustawienia → Tagowanie AI
error-chatgpt-request-failed = żądanie nie powiodło się
error-chatgpt-cut-off = odpowiedź została ucięta ({ $reason })
error-chatgpt-cut-off-no-reason = odpowiedź została ucięta (bez podania przyczyny)
error-openai-ended-early = odpowiedź od OpenAI skończyła się, zanim była kompletna
error-chatgpt-usage-limit = limit użycia planu ChatGPT dla Hoard na razie się wyczerpał. Limit można sprawdzić lub zmienić pod adresem { $url }
error-chatgpt-not-eligible = to konto lub obszar roboczy ChatGPT nie może korzystać ze swojego planu w innych aplikacjach. Należy użyć klucza API.
error-chatgpt-usage-unavailable = ChatGPT nie mógł teraz sprawdzić użycia planu; nastąpi ponowna próba później
error-chatgpt-unsupported = korzystanie z planu ChatGPT nie obsługuje { $param } w tym modelu
error-chatgpt-unsupported-images = korzystanie z planu ChatGPT nie obsługuje { $param } w tym modelu; należy wybrać model przyjmujący obrazy albo wyłączyć wysyłanie miniatur
error-chatgpt-unsupported-part = korzystanie z planu ChatGPT nie obsługuje części żądania w tym modelu
error-chatgpt-port = port to ten, przez który przeglądarka łączy się z Hoard
error-chatgpt-paste-address = należy wkleić cały adres, zaczynający się od http://127.0.0.1
error-too-many-hashes = maksymalnie { $max } skrótów naraz
error-credits-pack = paczka { $key }
error-credits-collection = kolekcja { $key }
error-format-too-long = format jest zbyt długi
error-not-started = serwer nie został uruchomiony
error-not-started-desktop = Hoard nie został uruchomiony
error-not-format-file = to nie jest plik formatu: { $name }
error-format-name = nazwa musi zawierać literę lub cyfrę od A do Z albo od 0 do 9
error-format-miss-name = pliki formatu nie obejmują { $file }
error-format-miss-store = format nie obejmuje { $file } jako pliku sklepu (zob. require)
error-format-miss-title = format nie znajduje tytułu w { $file }
error-cant-download-as = plików .{ $ext } nie można pobrać jako { $format }
error-only-images-resize = tylko obrazom można zmienić rozmiar
error-https-no-certificate = Najpierw należy wkleić lub przesłać certyfikat i jego klucz
error-https-no-cert-found = Nie znaleziono certyfikatu: należy wkleić tekst zaczynający się od -----BEGIN CERTIFICATE-----
error-https-no-key-found = Nie znaleziono klucza prywatnego: należy wkleić tekst zaczynający się od -----BEGIN PRIVATE KEY-----
error-https-several-keys = Jest więcej niż jeden klucz prywatny: należy wkleić tylko ten, który pasuje do certyfikatu
error-https-key-password = Klucz jest chroniony hasłem: należy podać go bez hasła (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Certyfikat nie jest prawidłowym plikiem PEM
error-https-key-not-pem = Klucz prywatny nie jest prawidłowym plikiem PEM
error-https-key-mismatch = Klucz nie należy do certyfikatu. Jeśli podano łańcuch, certyfikat serwera musi być pierwszy
error-https-unusable = Nie można użyć certyfikatu lub klucza
error-https-expired = Ten certyfikat wygasł { $date }
error-https-not-yet-valid = Ten certyfikat jest ważny dopiero od { $date }
error-https-unreadable = To nie jest certyfikat, który Hoard potrafi odczytać
error-https-bad-name = „{ $name }” nie jest adresem, który może znaleźć się w certyfikacie
error-https-test-timed-out = upłynął limit czasu połączenia testowego
error-https-test-failed = połączenie testowe nie powiodło się
error-https-no-ca = Hoard nie utworzył jeszcze swojego urzędu certyfikacji
error-https-unavailable = HTTPS nie jest dostępny na tym serwerze
error-https-certificate-mode = Certyfikat jest potrzebny przy opcji „Mój certyfikat”
error-https-test = Certyfikat nie zadziałał w połączeniu testowym: { $reason }
error-nothing-to-retry = nie ma tu nic do ponowienia
error-ai-unreachable = nie udało się połączyć z { $url }
error-ai-key-refused = { $url } odrzucił klucz API ({ $status })
error-ai-answered = { $url } odpowiedział { $status }: { $reason }
error-ai-not-json = { $url }/models nie odpowiedział w formacie JSON
error-model-server-refused = serwer modeli: { $reason }
error-no-model-server = pod adresem { $url } nie odpowiada żaden serwer modeli
error-not-llama-router = { $url } nie jest serwerem llama.cpp w trybie routera, więc nie może pobierać modeli
error-model-download = nie udało się pobrać { $model }: { $reason }
error-model-delete = nie udało się usunąć { $model }: { $reason }
error-unknown-kind = nieznany rodzaj zasobu { $kind }
error-watched-is-folder = obserwowana lokalizacja to folder na tym serwerze
error-cant-reach = brak dostępu do { $path }: { $reason }
error-folder-full-path = należy podać pełną ścieżkę folderu
error-cant-create = nie można utworzyć { $path }
error-cant-write = nie można zapisywać w { $path }
error-cant-read = nie można odczytać { $path }
error-overlaps-location = to nachodzi na lokalizację { $name } ({ $path })
error-overlaps-data = to nachodzi na folder danych Hoard ({ $path })
error-overlaps-cache = to nachodzi na folder pamięci podręcznej Hoard ({ $path })
error-not-watched = { $name } nie jest obserwowanym folderem
error-watched-no-kinds = obserwowany folder nie przyjmuje nowych plików, więc nie ma rodzajów do zmiany
error-keep-catch-all = należy zamiast tego ustawić inną lokalizację jako domyślną dla wszystkiego
error-location-holds-files = { $name } nadal zawiera pliki: najpierw należy przenieść je do innych lokalizacji (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = brak dostępu do { $name }, więc Hoard nie może stwierdzić, czy nadal zawiera pliki
error-relocate-running = pliki są już przenoszone między lokalizacjami; należy spróbować ponownie, gdy to się skończy
error-watched-not-moved = { $name } to obserwowany folder: Hoard nie przenosi jego plików
error-no-other-location = nie ma innej lokalizacji biblioteki, do której można przenieść pliki: najpierw należy ją dodać
error-full-path = należy podać pełną ścieżkę
error-pick-folder = należy wybrać folder
error-task-running = „{ $task }” już trwa
error-token-account-gone = konto tego tokenu już nie istnieje, więc nie ma on ulubionych ani wyszukiwań
error-favorites-too-many = maksymalnie 10 000 naraz
error-saved-search-name = zapisane wyszukiwanie musi mieć nazwę o długości maksymalnie 80 znaków
error-saved-search-too-long = to wyszukiwanie jest zbyt długie, by je zapisać
error-texture-pattern-map = Wzorzec nazw tekstur musi zawierać { "{" }map{ "}" }, inaczej każda mapa materiału dostanie tę samą nazwę
error-texture-pattern-fields = Wzorzec nazw tekstur może zawierać { "{" }name{ "}" }, { "{" }map{ "}" } i { "{" }size{ "}" }
error-texture-name-chars = Nazwy tekstur nie mogą zawierać znaków < > : " / \ | ? ani *
error-texture-map-type = Nie ma typu mapy o nazwie { $role }
server-role-base-color = kolor
server-role-normal = normalne
server-role-normal-dx = normalne (DirectX)
server-role-roughness = chropowatość
server-role-gloss = połysk
server-role-metalness = metaliczność
server-role-ao = okluzja otoczenia
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = wysokość
server-role-bump = wypukłość
server-role-cavity = wgłębienia
server-role-specular = odbicie (specular)
server-role-opacity = krycie
server-role-emissive = emisja
server-role-translucency = półprzezroczystość
error-texture-suffix-missing = Mapy typu „{ $map }” muszą mieć sufiks
error-texture-suffix-chars = Sufiks map typu „{ $map }” nie może zawierać znaków < > : " / \ | ? ani *
error-texture-ending-missing = Każda własna końcówka musi mieć końcówkę do wyszukania
error-texture-ending-chars = Własne końcówki nie mogą zawierać znaków < > : " / \ | ? ani *
error-texture-ending-twice = { $ending } występuje dwa razy wśród własnych końcówek
error-oauth-no-client-id = Ten link nie zawiera client_id.
error-oauth-unknown-app = Ta aplikacja nie jest (już) tu zarejestrowana. Należy ponownie rozpocząć łączenie w aplikacji.
error-oauth-redirect = Aplikacja poprosiła o przekierowanie na adres, którego nie zarejestrowała.
error-oauth-response-type = Hoard wydaje tylko kody logowania (response_type=code).
error-oauth-pkce = Aby zalogować się do Hoard, aplikacja musi używać PKCE z S256.
error-light-preview-images = lekki podgląd mają tylko obrazy
error-no-light-preview = brak lekkiego podglądu: { $reason }
error-no-frame-sheet = brak arkusza klatek: { $reason }
error-thumbnail-source = nieznane źródło miniatury: { $source }
error-thumbnail-too-big = ten obraz jest zbyt duży na miniaturę
error-thumbnail-models-only = tylko modele mogą mieć miniaturę narysowaną przez przeglądarkę
error-not-a-picture = to nie jest obraz, który Hoard potrafi odczytać: { $reason }
error-project-name = projekt musi mieć nazwę o długości maksymalnie 200 znaków
error-project-too-big = maksymalnie milion zasobów w jednym projekcie
error-project-too-many-at-once = maksymalnie 100 000 naraz
error-server-name = Nazwa może mieć maksymalnie { $max } znaków, w jednym wierszu
error-tool-setting = { $tool }: należy wybrać „wyłączone”, „automatycznie” lub podać ścieżkę
error-ai-model = Należy wybrać model, którego ma używać usługa AI
error-public-address = Adres publiczny powinien zaczynać się od http:// lub https://
error-update-hook-address = Adres wywoływany przy aktualizacjach powinien zaczynać się od http:// lub https://
error-update-hook-auth = Nagłówek Authorization nie może zawierać podziałów wiersza ani innych znaków sterujących
error-no-soundfont = Na serwerze nie ma pliku SoundFont pod ścieżką { $path }
error-no-soundfont-desktop = Nie ma pliku SoundFont pod ścieżką { $path }
error-workers = Procesy w tle: od 1 do { $max }
error-ai-batch = Zasoby na jedno żądanie AI: od 1 do 200
error-conversion-cache = Przekonwertowane pliki do pobrania: co najmniej 100 MB
error-conversion-keep-days = Reguły konwersji: oryginały można przechowywać od 1 do { $max } dni
error-conversion-target = Reguły konwersji: Hoard nie potrafi konwertować plików do { $format }
error-conversion-no-source = Reguły konwersji: każda reguła musi mieć co najmniej jeden format do konwersji
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Reguły konwersji: plików .{ $ext } nie można przekonwertować do { $format }
error-conversion-twice = Reguły konwersji: .{ $ext } występuje w dwóch regułach
error-conversion-cant-undo = Ten plik nie został przekonwertowany przez regułę albo jego oryginał został już usunięty.
error-conversion-path-taken = Pod { $path } jest teraz inny plik, więc oryginał nie może tam wrócić.
error-limits-zero = Limity nie mogą wynosić zero
error-settings-object = oczekiwano obiektu JSON z ustawieniami
error-bad-setting = błędne ustawienie: { $reason }
error-auth-default-role = Domyślna rola to view, edit lub admin
error-auth-session-hours = Sesje trwają od 1 godziny do roku
error-auth-settings = błędne ustawienia uwierzytelniania: { $reason }
error-ldap-settings = błędne ustawienia LDAP: { $reason }
error-ldap-no-server = Należy dodać adres serwera katalogowego
error-ldap-base-dn = Należy wskazać, gdzie są wpisy osób (base DN)
error-ldap-username-attribute = Należy wskazać, który atrybut zawiera nazwę używaną do logowania
error-ldap-bind-dn = Należy podać konto, które wyszukuje osoby, albo wyszukiwać je anonimowo
error-ldap-ca-cert = Urząd certyfikacji nie jest certyfikatem PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Adres serwera katalogowego jest pusty
error-ldap-server-form = { $server }: adres serwera katalogowego ma postać host:port, ldap://… lub ldaps://…
error-ldap-not-starttls = { $server }: to jest LDAPS, a nie StartTLS
error-ldap-no-certificate = nie zawiera certyfikatu
error-ldap-password-again = Należy ponownie podać hasło konta, które wyszukuje osoby: zapisane hasło jest wysyłane tylko do serwerów, dla których je podano
error-ldap-unreachable = nie udało się połączyć z żadnym serwerem katalogowym ({ $reasons })
error-ldap-no-anonymous = katalog nie pozwala na anonimowe wyszukiwanie osób ({ $reason })
error-ldap-bind-failed = konto, które wyszukuje osoby, nie mogło się zalogować ({ $reason })
error-ldap-search = wyszukiwanie { $name } w { $base }
error-ldap-group-search = wyszukiwanie grup { $name } w { $base }
error-ldap-several =
    { $count ->
        [one] { $count } wpis w katalogu pasuje do { $name }: należy zawęzić filtr użytkowników
        [few] { $count } wpisy w katalogu pasują do { $name }: należy zawęzić filtr użytkowników
        [many] { $count } wpisów w katalogu pasuje do { $name }: należy zawęzić filtr użytkowników
       *[other] { $count } wpisu w katalogu pasuje do { $name }: należy zawęzić filtr użytkowników
    }
error-ldap-password-check = katalog nie chciał sprawdzić hasła { $name } ({ $reason })
error-ldap-wrong-password = Katalog nie zna { $name } z tym hasłem
error-oauth-client-id = Należy podać identyfikator klienta, który dostawca utworzył dla Hoard
error-oauth-addresses = Adresy autoryzacji i tokenu powinny zaczynać się od https://
error-oauth-userinfo-address = Adres informacji o użytkowniku powinien zaczynać się od https://
error-oauth-username-claim = Należy wskazać, które oświadczenie (claim) zawiera nazwę osoby
error-oauth-secret-again = Należy ponownie podać klucz tajny klienta: zapisany jest wysyłany tylko na adres tokenu, dla którego go podano
error-oauth-issuer = Wystawca (issuer) powinien zaczynać się od https://
error-oauth-discovery-unreachable = Nie udało się połączyć z { $url }: { $reason }
error-oauth-discovery-status = { $url } odpowiedział { $status }
error-oauth-discovery-bad = { $url } nie jest dokumentem discovery
error-oauth-discovery-incomplete = { $url } nie podaje adresów autoryzacji i tokenu
error-signin-no-group = { $name } nie należy do grupy, która może korzystać z Hoard
error-signin-other-source = Istnieje już konto Hoard o nazwie { $name }, które loguje się w inny sposób
error-signin-other-subject = Istnieje już konto Hoard o nazwie { $name } należące do innej osoby u dostawcy
error-signin-no-account = { $name } nie ma jeszcze konta Hoard: administrator może je dodać w Ustawienia → Konto
error-signin-bad-name = Ta nazwa nie może być kontem Hoard (maksymalnie 64 znaki)
error-oauth-off = Logowanie przez OAuth nie jest włączone
error-oauth-bad-authorization-address = adres autoryzacji jest nieprawidłowy
error-oauth-refused = Dostawca nie zalogował: { $reason }
error-oauth-no-code = Dostawca przekierował z powrotem bez logowania
error-oauth-expired = Logowanie trwało zbyt długo lub zostało rozpoczęte gdzie indziej: należy spróbować ponownie
error-oauth-no-claim = Dostawca nie podał tożsamości (brak oświadczenia { $claim })
error-oauth-no-sub = Dostawca nie podał tożsamości (brak oświadczenia sub ani id)
error-oauth-token-unreachable = nie udało się połączyć z adresem tokenu dostawcy
error-oauth-token-refused = Dostawca nie chciał dokończyć logowania: { $reason }
error-oauth-no-access-token = dostawca nie wysłał tokenu dostępu
error-oauth-userinfo-unreachable = nie udało się połączyć z adresem informacji o użytkowniku dostawcy
error-oauth-userinfo-refused = Dostawca nie chciał podać, kto się zalogował ({ $status })
error-oauth-userinfo-not-json = informacje o użytkowniku od dostawcy nie są w formacie JSON
error-format-no-name = name jest puste
error-format-no-files = files jest puste: należy podać, które pliki czyta format, np. ["*.json"]
error-format-empty-selector = puste
error-format-unopened-bracket = nawias zamykający bez otwierającego
error-format-unclosed-bracket = nawias nie jest zamknięty
error-format-empty-step-dots = pusty krok (dwie kropki z rzędu albo kropka na początku lub końcu)
error-format-unclosed-except = !( nie jest zamknięte
error-format-unclosed-filter = [ nie jest zamknięte
error-format-after-filters = { $rest } po kluczu i filtrach { $step }
error-format-except-needs-glob = !(…) stawia się po kluczu zawierającym *
error-format-except-not-glob = !(…) stawia się po kluczu zawierającym *, a nie po { $key }
error-format-empty-step = pusty krok
error-format-not-a-filter = [{ $filter }] nie jest filtrem: należy użyć [], [key=value|other], [array], [string], [number], [bool] lub [object]
error-format-filter-needs-value = [{ $filter }] wymaga klucza i wartości
error-store-search-empty = brak frazy do wyszukania; należy wpisać nazwę paczki
error-update-hook-not-set = najpierw należy ustawić adres do wywołania
error-upload-stalled = przesyłanie { $path } utknęło; należy ponownie wysłać tę część
error-upload-busy = { $path } jest już zapisywany
error-upload-incomplete =
    { $count ->
        [one] { $count } plik jest niekompletny, np. { $files }
        [few] { $count } pliki są niekompletne, np. { $files }
        [many] { $count } plików jest niekompletnych, np. { $files }
       *[other] { $count } pliku jest niekompletne, np. { $files }
    }
error-upload-no-files-listed = brak plików
error-upload-not-part = { $path } nie należy do przesyłania { $id } albo przesyłanie zostało zamknięte
error-bad-path = błędna ścieżka
error-upload-part-offset = { $offset } nie jest miejscem, w którym zaczyna się część { $path }
error-upload-interrupted = przesyłanie przerwane: { $reason }
error-upload-part-long = część { $part } pliku { $path } jest dłuższa niż { $len } B
error-upload-part-short = część { $part } pliku { $path }: otrzymano { $got } B, oczekiwano { $len }
error-upload-file-long = { $path } jest większy, niż zadeklarowano
error-upload-file-short = { $path }: otrzymano { $got } B, oczekiwano { $len }
error-upload-complete = przesyłanie jest już zakończone
error-already-imported = już zaimportowano jako „{ $name }”

## Teksty serwera

server-store-place-readme = README
server-store-place-folders = nazwy folderów
server-store-why-linked = Link w paczce ({ $place })
server-store-why-same-title = Ten sam tytuł co w paczce ({ $place })
server-store-why-by = Autor: { $author }, podany w paczce ({ $place })
server-store-why-meshes = Mniej więcej tyle siatek co w paczce ({ $count })
server-store-why-textures = Mniej więcej tyle tekstur co w paczce ({ $count })
server-chatgpt-revoke-failed = Wylogowano tutaj, ale nie udało się powiadomić OpenAI. Dla pewności należy odłączyć Hoard w ChatGPT: Ustawienia → Bezpieczeństwo i logowanie → Logowanie przez ChatGPT.
server-chatgpt-signed-in-title = Zalogowano do ChatGPT
server-chatgpt-signed-in = Hoard jest zalogowany. Tagowanie AI może teraz korzystać z planu ChatGPT.
server-chatgpt-signed-in-as = Hoard jest zalogowany jako { $email }. Tagowanie AI może teraz korzystać z planu ChatGPT.
server-chatgpt-not-finished-title = Logowanie nie zostało dokończone
server-chatgpt-other-computer = Jeśli logowanie rozpoczęto z Hoard na innym komputerze, należy skopiować adres tej strony z paska adresu i wkleić go tam w Ustawieniach Hoard.
server-back-to-settings = Wróć do Ustawień
server-issues-pack = Paczki, których nie udało się dodać
server-issues-pack-blurb = Można usunąć paczkę i dodać ją ponownie albo otworzyć ją, by zobaczyć, co zawiera.
server-issues-damaged = Przechowywane pliki, które są uszkodzone lub zniknęły
server-issues-damaged-blurb = Wykryte przez zadanie „Sprawdź pliki pod kątem uszkodzeń”. Należy przywrócić je z kopii zapasowej lub ponownie dodać ich paczkę, a potem znów uruchomić „Sprawdź pliki pod kątem uszkodzeń”.
server-issues-folder = Obserwowane foldery, których nie można odczytać
server-issues-folder-blurb = Ich paczki pozostają bez zmian, dopóki folder nie będzie znów czytelny.
server-issues-missing = Obserwowane paczki, których folder zniknął
server-issues-missing-blurb = Każda paczka jest przechowywana przez 3 dni na wypadek, gdyby folder był tylko niedostępny, a potem usuwana.
server-issues-job = Zadania w tle, które się nie powiodły
server-issues-job-blurb = Ponowienie uruchamia zadanie od początku.
server-issues-file = Pliki, których nie udało się otworzyć
server-issues-file-blurb = Hoard nie mógł ich otworzyć ani zapisać. Obserwowana paczka spróbuje ponownie przy następnym skanowaniu; plik, który już był w bibliotece, zachował poprzednią zawartość.
server-issues-unpack = Archiwa, których nie udało się rozpakować
server-issues-unpack-blurb = Każde zostaje w paczce jako zwykły plik, więc nic z jego zawartości nie trafia do biblioteki.
server-issues-read = Pliki, których Hoard nie zrozumiał
server-issues-read-blurb = Są w bibliotece, ale bez informacji, tagów i miniatury, które daje ich odczytanie.
server-issues-render = Modele, na których zawiódł renderer miniatur
server-issues-render-blurb = Nie mają miniatury. Otwarcie modelu w przeglądarce 3D może się udać i pozwala zapisać miniaturę.
server-issues-convert = Modele, których Blender nie przekonwertował
server-issues-convert-blurb = Nie mają kopii GLB, więc nie można ich wyświetlić w 3D.
server-issues-unreal = Pakiety Unreal, których nie udało się przekonwertować
server-issues-unreal-blurb = Pozostałe pakiety są na stronie Unreal paczki.
server-issues-reformat = Pliki, których reguła konwersji nie zdołała przekonwertować
server-issues-reformat-blurb = Pozostawiono je bez zmian.
server-issues-materials = Paczki, w których nie udało się ustalić materiałów
server-issues-materials-blurb = Wszystkie tekstury są na miejscu; po prostu nie są pogrupowane w materiały.
server-issues-material-thumb = Materiały bez miniatury
server-issues-material-thumb-blurb = Materiał został znaleziony, ale nie udało się narysować jego podglądu.
server-issues-store-files = Pliki sklepów, których nie udało się odczytać
server-issues-store-files-blurb = Pliki ze sklepu, z którego pochodzi paczka (np. JSON z Megascans), które nadają jej nazwę i tagi.
server-issues-scan-again = Skanuj ponownie
server-issues-retry = Ponów
server-issues-reanalyze = Analizuj paczkę ponownie
server-issues-convert-again = Konwertuj ponownie
server-issues-look-again = Sprawdź ponownie
server-issues-read-again = Odczytaj ponownie
server-issues-blob-missing = Przechowywany plik zniknął.
server-issues-blob-damaged = Przechowywany plik nie zgadza się z tym, co zapisano: jest uszkodzony.
server-issues-folder-gone = Nie znaleziono folderu „{ $folder }”.
server-job-a-pack = paczka
server-job-ingest = Przyjmowanie: { $pack }
server-job-retag = Ponowne tagowanie: { $pack }
server-job-llm-tag = Tagowanie AI: { $pack }
server-job-convert = Konwersja w Blenderze: { $pack }
server-job-unreal = Konwersja pakietów Unreal: { $pack }
server-job-reformat = Konwersja plików według reguł konwersji: { $pack }
server-job-scan = Skanowanie obserwowanego folderu
server-job-relocate = Przenoszenie plików między lokalizacjami biblioteki
server-job-looks = Ustalanie kolorów i wyglądu
server-job-sounds = Odsłuchiwanie dźwięków
server-job-embed = Rozpoznawanie, co przedstawiają obrazy
server-job-hear = Rozpoznawanie, jak brzmią dźwięki
server-job-materials = Wyszukiwanie materiałów: { $pack }
server-job-read-files = Odczytywanie plików: { $pack }
server-job-store-lookup = Wyszukiwanie strony w sklepie
server-maintenance-backup = Kopia zapasowa bazy danych
server-maintenance-backup-blurb = Spójna kopia bazy danych (paczki, tagi, notatki, kolekcje i Ustawienia) w folderze kopii zapasowych obok niej. Nie zawiera plików biblioteki: raz zapisane nigdy się nie zmieniają, więc kopię zrobi każda synchronizacja plików.
server-maintenance-scrub = Sprawdź pliki pod kątem uszkodzeń
server-maintenance-scrub-blurb = Odczytuje każdy przechowywany plik i porównuje go z jego odciskiem, by wykryć pliki, które uległy degradacji lub zniknęły z dysku lub NAS-a. Przy dużej bibliotece trwa to dłuższą chwilę.
server-maintenance-reanalyze = Analizuj ponownie wszystkie paczki
server-maintenance-reanalyze-blurb = Odczytuje każdy plik ponownie tą wersją Hoard: miniatury, podglądy, szczegóły i tagi z reguł powstają od nowa. Własne zmiany zostają; AI nie jest pytane ponownie.
server-maintenance-clear-cache = Wyczyść zapisane konwersje
server-maintenance-clear-cache-blurb = Usuwa pliki zachowane z konwersji „Pobierz jako” (zostaną utworzone ponownie przy następnym żądaniu) oraz pozostałości po przerwanej pracy.
server-maintenance-shrink-thumbs = Zmniejsz miniatury
server-maintenance-shrink-thumbs-blurb = Zapisuje miniatury i widoki modeli przechowywane jako PNG zgodnie z Ustawienia → Wydajność → Miniatury, co zwykle zmniejsza je kilkukrotnie. Te z przezroczystością oraz pixel art pozostają w PNG.
server-maintenance-gc = Usuń nieużywane pliki
server-maintenance-gc-blurb = Usuwa przechowywane pliki, na które nic już nie wskazuje, np. pliki paczek usuniętych na stałe.
server-update-no-address = nie ustawiono adresu do wywołania
server-update-no-answer = brak odpowiedzi w ciągu 30 sekund

## Ustawienia: Wygląd

appearance-language = Język
appearance-language-auto = Jak w przeglądarce ({ $name })

## Root

root-unreachable = Nie udało się połączyć z serwerem Hoard: { $error }
root-unreachable-desktop = Hoard nie odpowiada: { $error }
root-desktop-reopen = To okno zostało wylogowane. Zamknij Hoarda i otwórz go ponownie, aby kontynuować.
root-try-again = Spróbuj ponownie
root-lazy-connect = połączenia aplikacji
window-minimize = Minimalizuj
window-maximize = Maksymalizuj
window-restore = Przywróć
window-close = Zamknij

## Aplikacja: biblioteka, górny pasek i filtry

app-lazy-settings = ustawień
app-lazy-archive = archiwum
app-lazy-shortcuts = skrótów
app-lazy-setup = konfiguracji
app-theme-system = Motyw systemu
app-theme-light = Motyw jasny
app-theme-dark = Motyw ciemny
app-color-red = Czerwony
app-color-orange = Pomarańczowy
app-color-yellow = Żółty
app-color-green = Zielony
app-color-teal = Morski
app-color-blue = Niebieski
app-color-purple = Fioletowy
app-color-pink = Różowy
app-color-brown = Brązowy
app-color-sand = Piaskowy
app-color-gray = Szary
app-color-slate = Łupkowy
app-color-black = Czarny
app-color-white = Biały
app-color-swatch = { $color }: zasoby z dużą ilością tego koloru ({ $lower })
app-color-any = Dowolny kolor
app-color-pick = Wybierz dowolny kolor
app-color = Kolor
app-music = Muzyka
app-tempo = BPM
app-tempo-min = od
app-tempo-max = do
app-tempo-min-title = Muzyka w tym tempie lub szybsza, w uderzeniach na minutę (połowa lub podwójne tempo też się liczy)
app-tempo-max-title = Muzyka w tym tempie lub wolniejsza, w uderzeniach na minutę
app-key = Tonacja
app-key-any = Dowolna tonacja
app-key-major = Dur
app-key-minor = Moll
app-key-mix = I tonacje, które da się z nią zmiksować
app-key-mix-title = Jej tonacja równoległa durowa lub molowa oraz tonacje o kwintę wyżej i niżej, z którymi DJ-e ją miksują
app-imitation = Brzmi jak
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Zapisz dźwięk tak, jak byś go przeliterował: najpierw pojawią się podobnie brzmiące
app-imitation-record = Wydaj dźwięk do mikrofonu: najpierw pojawią się podobnie brzmiące
app-imitation-stop = Zatrzymaj nagrywanie
app-imitation-heard = Twój dźwięk
app-imitation-mic-failed = Nie udało się nagrać z mikrofonu: { $error }
app-retry = Ponów
app-undo = Cofnij
app-search-failed = Wyszukiwanie nie powiodło się: { $error }
app-unreachable = Nie udało się połączyć z serwerem: { $error }
app-unreachable-desktop = Hoard nie odpowiada: { $error }
app-forget-project-title = Zapomnieć { $name }?
app-forget-project-body = Hoard przestanie go wymieniać przy zasobach, których używa ({ $count }), dopóki następne hoard sync znów go nie zgłosi. W samym projekcie nic się nie zmienia.
app-forget-project-confirm = Zapomnij
app-forget-project-failed = Nie udało się zapomnieć { $name }: { $error }
app-forget-project-label = Zapomnij projekt { $name }
app-forget-project = Zapomnij ten projekt
app-save-name-like = podobne do { $name }
app-save-name-favorites = ulubione
app-search-saved = Zapisano „{ $name }”: jest w Filtrach, w sekcji Moje
app-rank-alike = Najbardziej podobne
app-rank-color = Najwięcej koloru
app-rank-match = Najlepsze dopasowanie
app-rank-imitation = Najbardziej podobne
app-sort = Sortuj
app-sort-newest = Najnowsze
app-sort-name = Nazwa
app-sort-largest = Największe
app-sort-oldest = Najstarsze
app-collection-added =
    Dodano { $count ->
        [one] { $count } zasób
        [few] { $count } zasoby
        [many] { $count } zasobów
       *[other] { $count } zasobu
    } do { $name }
app-collection-add-failed = Nie udało się dodać do kolekcji: { $error }
app-collection-removed = Usunięto z { $name }: { $count }
app-collection-remove-failed = Nie udało się usunąć z kolekcji: { $error }
app-ns-subject = Temat
app-ns-style = Styl
app-ns-theme = Motyw
app-ns-material = Materiał
app-ns-map = Mapa PBR
app-ns-color = Kolor
app-ns-mood = Nastrój
app-ns-genre = Gatunek
app-ns-license = Licencja
app-ns-res = Rozdzielczość
app-ns-duration = Długość
app-ns-poly = Liczba wielokątów
app-ns-format = Format
app-ns-weight = Rozmiar
app-ns-tech = Techniczne
app-ns-blender = Wersja Blendera
app-ns-depth = Głębia bitowa
app-ns-ucs = Kategoria dźwięku (UCS)
app-ns-bpm = Tempo
app-ns-root = Tonacja
app-ns-level = Głośność
app-ns-ue = Klasa Unreal
app-ns-engine = Silnik
app-ns-extracted = Odzyskane źródło
app-ns-script = Pismo
app-bulk-failed =
    Nie udało się zmienić { $count ->
        [one] tego zasobu
        [few] tych { $count } zasobów
        [many] tych { $count } zasobów
       *[other] tych { $count } zasobu
    }: { $error }
app-bulk-untagged =
    Usunięto tag { $tag } z { $count ->
        [one] { $count } zasobu
        [few] { $count } zasobów
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
app-bulk-tagged =
    Dodano tag { $tag } do { $count ->
        [one] { $count } zasobu
        [few] { $count } zasobów
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
app-bulk-category-set =
    Ustawiono kategorię { $category } dla { $count ->
        [one] { $count } zasobu
        [few] { $count } zasobów
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
app-bulk-category-cleared =
    Wyczyszczono kategorię { $count ->
        [one] { $count } zasobu
        [few] { $count } zasobów
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
app-credits-failed = Nie udało się utworzyć listy autorów: { $error }
app-archived-assets =
    Zarchiwizowano { $count ->
        [one] { $count } zasób
        [few] { $count } zasoby
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
app-archived-one = Zarchiwizowano { $name }
app-open-failed = Nie udało się otworzyć tego zasobu: { $error }
app-unnamed-pack = paczka
app-unnamed-collection = kolekcja
app-unnamed-asset = zasób
app-home-version = Hoard { $version } — wróć do biblioteki
app-home = Wróć do biblioteki
app-search-label = Szukaj zasobów
app-search-placeholder = Szukaj nazw, ścieżek, tagów
app-views = Widoki
app-library = Biblioteka
app-packs = Paczki
app-packs-working = W toku: { $count }
app-update-title = Jest już Hoard { $version } (ten serwer działa na { $current }). Wystarczy pobrać nowy obraz lub plik binarny i uruchomić ponownie; baza danych zaktualizuje się sama.
app-update = Aktualizuj
app-theme = Motyw
app-sign-in = Zaloguj się
app-archive-title = Archiwum: { $contents }
app-archive = Archiwum
app-settings = Ustawienia
app-palette-title = Paleta poleceń ({ $keys })
app-import-folder = Importuj folder
app-add-files = Dodaj pliki
app-add-files-short = Dodaj
app-split-label = Jak zaimportować ten folder
app-split-before = { "" }
app-split-after =
    { " " }zawiera { $count ->
        [one] { $count } folder lub archiwum.
        [few] { $count } foldery lub archiwa.
        [many] { $count } folderów lub archiwów.
       *[other] { $count } folderu lub archiwum.
    }
app-split-each = Importuj każdy jako osobną paczkę ({ $count })
app-split-one = Importuj jako jedną paczkę
app-cancel = Anuluj
app-filters = Filtry
app-done = Gotowe
app-yours = Moje
app-favorites = Ulubione
app-forget-search-label = Zapomnij zapisane wyszukiwanie { $name }
app-forget-search = Zapomnij to wyszukiwanie
app-projects-title = Projekty synchronizowane stąd przez hoard sync, z tym, co miała ich ostatnia synchronizacja
app-projects = Projekty
app-project-synced = Synchronizacja: { $when }
app-project-synced-by = Synchronizacja: { $when }, { $who }
app-collections = Kolekcje
app-no-filters = Brak filtrów dla tego wyszukiwania.
app-kind = Rodzaj
app-facets-more = jeszcze { $count }
app-facets-fewer = mniej
app-searching = Wyszukiwanie…
app-searching-live = Wyszukiwanie
app-asset-count =
    { $count ->
        [one] { $count } zasób
        [few] { $count } zasoby
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
app-clear-all = Wyczyść wszystko
app-save-name-label = Nazwa tego wyszukiwania
app-save-name-placeholder = Nazwa
app-save = Zapisz
app-save-search-title = Zachowaj to wyszukiwanie w Filtrach, w sekcji Moje
app-save-search = Zapisz wyszukiwanie
app-select-title = Zaznacz kilka zasobów, by razem je otagować, skategoryzować, dodać do kolekcji, pobrać lub zarchiwizować (X lub Ctrl+kliknięcie)
app-select = Zaznacz
app-tile-size = Rozmiar kafelków
app-supporting-title = Bufory glTF, biblioteki materiałów, mapy materiałów, materiały używane tylko przez jeden model i pliki sklepów opisujące modele. Są pobierane razem z tym, co ich używa. Strony arkusza sprite’ów oraz jego sprite’y, które paczka ma też jako osobne obrazy, są pokazywane razem z arkuszem. Zasób, który paczka ma w kilku formatach, jest pokazywany raz, w formacie, który Hoard czyta najlepiej.
app-supporting-hidden =
    { $count ->
        [one] Ukryto { $count } plik pomocniczy
        [few] Ukryto { $count } pliki pomocnicze
        [many] Ukryto { $count } plików pomocniczych
       *[other] Ukryto { $count } pliku pomocniczego
    }
app-supporting-show = Pokaż
app-supporting-shown = Z plikami pomocniczymi
app-supporting-hide = Ukryj
app-folders = Foldery
app-folders-title = Przeglądaj paczkę tak, jak była na dysku, folder po folderze
app-folders-label = Foldery paczki
app-welcome-title = Upuść paczkę zasobów w dowolnym miejscu tej strony
app-welcome-before = Działają pliki zip, archiwa tar, foldery i luźne pliki. Hoard je rozpakowuje, przechowuje każdy plik tylko raz według skrótu, odczytuje rozmiary, długości i liczby wielokątów, a wszystko taguje, by można było wyszukiwać np.{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }lub{ " " }
app-welcome-after = .
app-welcome-import-folder = Importuj folder
app-welcome-choose-files = Wybierz pliki
app-none-similar = Nic innego nie jest jeszcze zbyt podobne do { $name }.
app-none = Brak wyników.
app-none-query = Brak wyników dla „{ $query }”.
app-none-imitation = Żaden zmierzony dźwięk nie brzmi jeszcze zbytnio jak „{ $word }”.
app-none-heard = Żaden zmierzony dźwięk nie brzmi jeszcze zbytnio jak twój dźwięk.
app-none-filters = Brak wyników przy tych filtrach.
app-none-query-filters = Brak wyników dla „{ $query }” przy tych filtrach.
app-clear-search-filters = Wyczyść wyszukiwanie i filtry
app-loading-more = Ładowanie kolejnych
app-keys-hint = Strzałki przesuwają zaznaczenie, Enter otwiera, Spacja odtwarza dźwięk, X lub Ctrl+kliknięcie zaznacza kilka (na ekranie dotykowym: długie przytrzymanie).
app-resize-label = Zmień szerokość panelu szczegółów
app-resize-title = Przeciągnij, by zmienić szerokość · kliknij dwukrotnie, by dopasować
app-selected-label = Zaznaczone zasoby
app-selected = Zaznaczono: { $count }
app-pick-assets = Wybierz zasoby
app-download-zip = Pobierz zip
app-credits-title = CREDITS.md dla paczek, z których pochodzą: kto je stworzył, ich licencje i wymagane przez nie podziękowania
app-credits = Autorzy
app-prefer-sources-title = Dźwięki SoundWave i tekstury Unreal, których oryginalny plik Hoard odzyskał
app-prefer-sources = Odzyskane WAV/PNG zamiast .uasset ({ $count })
bag-clear = Wyczyść zaznaczenie
bag-glb-heading = Modele jako GLB
bag-glb-text = Każdy model jako plik .glb ustawiony osią Y w górę i przodem do przodu, jego tekstury jako osobne pliki w folderze textures wspólnym dla modeli. Cała reszta trafia tam bez zmian.
bag-glb-download = Pobierz zip z GLB
bag-plain-heading = Pliki bez zmian
bag-plain-text = Każdy plik tak, jak został zaimportowany, z tym, czego potrzebuje, w osobnym folderze dla każdej paczki.
bag-remove = Usuń { $name } z zaznaczenia
bag-more = i jeszcze { $count }
app-bulk-tag-label = Tag do dodania do zaznaczonych zasobów lub usunięcia z nich
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Dodaj ten tag do każdego zaznaczonego zasobu
app-bulk-tag-add = Dodaj
app-bulk-tag-remove-title = Usuń ten tag z każdego zaznaczonego zasobu
app-bulk-tag-remove = Usuń
app-bulk-category-label = Kategoria zaznaczonych zasobów (puste pole ją czyści)
app-bulk-category-placeholder = Kategoria
app-bulk-category-set-title = Ustaw kategorię każdego zaznaczonego zasobu
app-bulk-category-clear-title = Wyczyść kategorię każdego zaznaczonego zasobu
app-bulk-category-set-button = Ustaw
app-bulk-category-clear-button = Wyczyść
app-remove-from-collection = Usuń z { $name }
app-unfavorite = Usuń z ulubionych
app-favorite = Dodaj do ulubionych
app-select-all = Zaznacz wszystkie ({ $count })
app-archive-selected-title = Znikną z wyszukiwania, dopóki nie zostaną przywrócone lub usunięte na stałe z Archiwum
app-drop-items =
    { $count ->
        [one] Upuść { $count } element, aby go dodać
        [few] Upuść { $count } elementy, aby je dodać
        [many] Upuść { $count } elementów, aby je dodać
       *[other] Upuść { $count } elementu, aby je dodać
    }
app-drop = Upuść, aby dodać do biblioteki
app-drop-hint = Każdy plik lub folder najwyższego poziomu staje się osobną paczką. Gigabajty nie są problemem; duże pliki wznawiają przesyłanie, gdy połączenie zostanie zerwane.

## Paleta poleceń (CommandPalette.vue, z poleceniami z App.vue)

palette-group-go-to = Przejdź do
palette-group-actions = Działania
palette-group-theme = Motyw
palette-group-kinds = Rodzaje
palette-group-packs = Paczki
palette-group-collections = Kolekcje
palette-group-projects = Projekty
palette-group-saved = Zapisane wyszukiwania
palette-group-assets = Zasoby
palette-hint-current = bieżący
palette-library = Biblioteka
palette-library-keywords = zasoby siatka start strona główna assets grid home
palette-favorites = Ulubione
palette-favorites-keywords = gwiazdka gwiazdki polubione ulubiony starred stars liked
palette-packs = Paczki
palette-archive = Archiwum
palette-archive-keywords = kosz usunięte przywróć przywracanie trash recycle bin deleted restore
palette-stats = Statystyki biblioteki
palette-stats-keywords = statystyki liczby rozmiar dysk miejsce liczba statistics numbers size disk space count
palette-copies = Prawie kopie
palette-copies-keywords = duplikaty takie same podobne przeskalowane przekonwertowane zmarnowane miejsce duplicates same similar resized converted wasted space dedupe
palette-search = Szukaj
palette-add-files = Dodaj pliki…
palette-add-files-keywords = prześlij przesyłanie import importuj upload
palette-import-folder = Importuj folder…
palette-import-folder-keywords = prześlij przesyłanie paczka upload pack
palette-select-all = Zaznacz wszystkie widoczne ({ $count })
palette-select-all-keywords = zaznacz wybierz check
palette-select-several = Zaznacz kilka…
palette-select-several-keywords = zbiorczo wiele tag archiwizuj bulk multiple archive
palette-archive-selected = Zarchiwizuj zaznaczone ({ $count })
palette-archive-selected-keywords = usuń kosz delete remove trash
palette-clear-selection = Wyczyść zaznaczenie
palette-clear-filters = Wyczyść filtry
palette-clear-filters-keywords = resetuj reset
palette-hide-supporting = Ukryj pliki pomocnicze
palette-show-supporting = Pokaż pliki pomocnicze
palette-supporting-keywords = mapy normalnych chropowatość maski normal maps roughness masks
palette-account = Konto i tokeny dostępu
palette-account-keywords = hasło logowanie użytkownik token api password login user
palette-sign-out = Wyloguj się ({ $name })
palette-sign-out-keywords = wyloguj wylogowanie log out logout
palette-sign-in = Zaloguj się
palette-sign-in-keywords = zaloguj logowanie konto hasło log in login account password
palette-settings = Ustawienia
palette-settings-keywords = blender ffmpeg llm ai tagowanie unreal administrator narzędzia preferencje opcje tagging admin tools preferences
palette-shortcuts = Skróty klawiszowe
palette-shortcuts-keywords = pomoc klawisze klawiatura help keys
palette-theme-keywords = motyw wygląd tryb theme appearance mode
palette-theme-variant-keywords = wygląd kolor appearance color
palette-light-theme = Motyw jasny: { $name }
palette-dark-theme = Motyw ciemny: { $name }
palette-kind-all = Wszystkie rodzaje (nie tylko { $kind })
palette-kind-only = Tylko { $kind }
palette-title = Paleta poleceń
palette-placeholder = Szukaj zasobów, przejdź do paczki, uruchom polecenie…
palette-results = Wyniki
palette-searching = Wyszukiwanie…
palette-no-match = Brak wyników dla „{ $query }”
palette-foot-move = przejdź
palette-foot-run = uruchom
palette-foot-shortcuts = wszystkie skróty

## Skróty klawiszowe (ShortcutsDialog.vue)

shortcuts-title = Skróty klawiszowe
shortcuts-close = Zamknij
shortcuts-anywhere = Wszędzie
shortcuts-palette = Paleta poleceń: szukanie, przechodzenie, uruchamianie
shortcuts-search = Szukaj
shortcuts-this-list = Ta lista
shortcuts-escape = Zamknij, wyczyść zaznaczenie lub zamknij szczegóły
shortcuts-library = Biblioteka
shortcuts-move = Przechodzenie między kafelkami (lub J i K)
shortcuts-open = Otwórz szczegóły
shortcuts-play-sound = Odtwórz lub zatrzymaj dźwięk
shortcuts-select = Zaznacz lub odznacz
shortcuts-favorite = Dodaj do ulubionych lub usuń z ulubionych
shortcuts-key-click = kliknięcie
shortcuts-select-range = Zaznacz zakres
shortcuts-select-all = Zaznacz wszystko, co widoczne
shortcuts-sound-player = Odtwarzacz dźwięku
shortcuts-play-pause = Odtwórz lub wstrzymaj
shortcuts-skip = Przewiń do tyłu lub do przodu
shortcuts-start-end = Początek lub koniec
shortcuts-image-inspector = Inspektor obrazów
shortcuts-zoom = Powiększ lub pomniejsz
shortcuts-fit = Dopasuj do okna
shortcuts-actual-pixels = Rzeczywiste piksele
shortcuts-tile = Powiel 3×3, by sprawdzić szwy
shortcuts-background = Następne tło
shortcuts-channel = Pokaż jeden kanał

## KindChips.vue

kinds-group = Rodzaje zasobów

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = dodany przez człowieka
tag-source-rule = z tagowania offline
tag-source-meta = zmierzony z pliku
tag-source-ai = zasugerowany przez AI
tag-remove = Usuń tag { $tag }

## ActivityIndicator.vue

activity-label-idle = Aktywność w tle: bezczynność
activity-label-running = Aktywność w tle: w toku { $running }
activity-label-queued = Aktywność w tle: w toku { $running }, w kolejce { $queued }
activity-title = Aktywność w tle
activity-unreachable = Brak połączenia z serwerem
activity-unreachable-desktop = Hoard nie odpowiada
activity-empty = Nic nie jest uruchomione. Importy, tagowanie, konwersje i pobieranie plików zip pojawiają się tutaj w trakcie pracy.
activity-queued = w kolejce
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } godz. { $min } min
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } trójk.
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Odtwórz { $name }
tile-pause = Wstrzymaj { $name }
tile-favorite = Dodaj do ulubionych: { $name }
tile-in-favorites = W ulubionych
tile-add-favorite = Dodaj do ulubionych (F)
tile-more-like = Więcej podobnych do { $name }
tile-more-like-this = Więcej podobnych
tile-select = Zaznacz { $name }

## CollectionPicker.vue

collections-add = Dodaj do kolekcji
collections-menu = Kolekcje
collections-find-label = Znajdź lub utwórz kolekcję
collections-find-placeholder = Znajdź kolekcję lub nazwij nową
collections-create = Utwórz
collections-empty = Nie ma jeszcze kolekcji. Wpisanie nazwy tworzy nową.

## FolderPicker.vue

folders-list = Foldery
folders-close = Zamknij
folders-browse = Przeglądaj…
folders-up-title = Folder wyżej
folders-up = W górę
folders-this-computer = Ten komputer
folders-none = Brak folderów.

## LocationForm.vue

locations-mode-question = Co Hoard ma zrobić z tym folderem?
locations-mode-managed = Przechowywać tu bibliotekę
locations-mode-managed-hint = Przesyłane lub importowane paczki są kopiowane do tego folderu. Hoard nim zarządza, więc nie należy ruszać jego zawartości.
locations-mode-watched = Indeksować istniejący folder
locations-mode-watched-hint = Każdy folder w nim staje się paczką. Hoard odczytuje pliki tam, gdzie są, nigdy ich nie zmienia, nie przenosi ani nie usuwa i wykrywa nowe foldery, gdy się pojawiają.
locations-folder = Folder na serwerze
locations-folder-desktop = Folder
locations-folder-placeholder = Folder z paczkami
locations-folder-hint-managed = Hoard utworzy go, jeśli nie istnieje. W Dockerze jest to folder zamontowany w kontenerze.
locations-folder-hint-managed-desktop = Hoard utworzy go, jeśli nie istnieje.
locations-folder-hint-watched = W Dockerze należy najpierw zamontować folder w kontenerze (może być tylko do odczytu), a potem wybrać go tutaj.
locations-folder-hint-watched-desktop = Hoard tylko go odczytuje: nic w nim nie jest zmieniane.
locations-folder-hint-cloud = Pasuje też folder synchronizowany przez Dropbox, Google Drive, OneDrive lub iCloud Drive, a także s3://bucket-name dla zasobnika w magazynie S3.
locations-name = Nazwa
locations-name-placeholder-managed = Biblioteka
locations-name-placeholder-watched = Moje paczki
locations-files = Pliki, które tu trafiają
locations-catch-all = Także wszystko, czego nie przyjmuje żadna inna lokalizacja
locations-kinds-hint = Pliki nowej paczki trafiają do lokalizacji, która ma zaznaczony ich rodzaj. Reszta trafia do lokalizacji domyślnej.
locations-files-all = Na razie wszystkie. Później w Ustawieniach można dodać kolejne lokalizacje, by wysyłać niektóre rodzaje (np. muzykę) na inny dysk.
locations-first-watched-note = Przesyłanie wymaga biblioteki zarządzanej przez Hoard. Można ją dodać później w Ustawienia → Lokalizacje biblioteki.
locations-checking = Sprawdzanie folderu…
locations-add = Dodaj lokalizację
locations-cancel = Anuluj

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Nie udało się załadować lokalizacji biblioteki
locations-takes-and-rest = { $kinds } i wszystko inne
locations-takes-rest = Wszystko, czego nie przyjmują inne
locations-takes-everything = Wszystko
locations-takes-nothing = Na razie nic
locations-save-failed = Nie udało się zapisać lokalizacji
locations-unwatch-title = Przestać obserwować { $name }?
locations-unwatch-body =
    { $count ->
        [one] Jego { $count } paczka zniknie z biblioteki razem z tagami i notatkami. Pliki w folderze pozostaną nietknięte.
        [few] Jego { $count } paczki znikną z biblioteki razem z tagami i notatkami. Pliki w folderze pozostaną nietknięte.
        [many] Jego { $count } paczek zniknie z biblioteki razem z tagami i notatkami. Pliki w folderze pozostaną nietknięte.
       *[other] Jego { $count } paczki zniknie z biblioteki razem z tagami i notatkami. Pliki w folderze pozostaną nietknięte.
    }
locations-unwatch-body-empty = Pliki w folderze pozostaną nietknięte.
locations-unwatch = Przestań obserwować
locations-remove-title = Usunąć { $name }?
locations-remove-body = Wszystkie pliki z tej lokalizacji zostaną najpierw przeniesione do innych lokalizacji według rodzaju; zostanie usunięta, gdy będzie pusta. Nic nie zostanie utracone.
locations-remove-body-only = Jeśli zawiera pliki, najpierw należy dodać inną lokalizację, do której zostaną przeniesione.
locations-remove = Usuń
locations-removed = Usunięto { $name }
locations-emptying = Przenoszenie plików z { $name }; zostanie usunięta, gdy to się skończy
locations-offline-title = Brak dostępu do { $name }
locations-offline-body = Nie można przenieść jej plików, gdy jest niedostępna. Usunąć mimo to? Pliki, które jeszcze na niej są, będą widoczne jako brakujące.
locations-remove-anyway = Usuń mimo to
locations-remove-failed = Nie udało się usunąć { $name }
locations-rebalancing = Przenoszenie zapisanych plików do lokalizacji przyjmujących ich rodzaj
locations-rebalance-failed = Nie udało się przenieść plików
locations-scanned-never = jeszcze nie skanowano
locations-scanned-now = zeskanowano przed chwilą
locations-scanned-minutes = zeskanowano { $mins } min temu
locations-scanned-hours = zeskanowano { $hours } godz. temu
locations-scanned-date = zeskanowano { $date }
locations-scan-failed = Nie udało się zeskanować { $name }
locations-watching = Obserwowanie: { $name }
locations-added = Dodano { $name }
locations-loading = Ładowanie…
locations-managed-title = Biblioteki zarządzane przez Hoard
locations-watched-title = Obserwowane foldery
locations-managed-hint = Tu kopiowane są przesłane i zaimportowane pliki. Każda lokalizacja przyjmuje zaznaczone rodzaje; lokalizacja domyślna przyjmuje resztę.
locations-watched-hint = Foldery paczek indeksowane w miejscu. Hoard nigdy nie zmienia znajdujących się w nich plików i co 15 minut sprawdza, czy foldery doszły lub zniknęły.
locations-managed-empty = Na razie brak, więc przesłane pliki nie mają dokąd trafić. Można dodać lokalizację poniżej.
locations-reachable = Dostępna
locations-unreachable-now = Obecnie niedostępna
locations-catch-all-tag = domyślna
locations-packs =
    { $count ->
        [one] { $count } paczka
        [few] { $count } paczki
        [many] { $count } paczek
       *[other] { $count } paczki
    }
locations-unreachable = niedostępna
locations-scanning = Skanowanie…
locations-scan = Skanuj teraz
locations-edit = Edytuj
locations-catch-all-locked = Aby to zmienić, należy ustawić inną lokalizację jako domyślną
locations-edit-hint-before = Dotyczy plików dodanych od teraz. Aby przenieść to, co już zapisano, należy użyć opcji
locations-edit-hint-after = poniżej.
locations-save = Zapisz
locations-rebalance = Przenieś zapisane pliki zgodnie z ustawieniami
locations-rebalance-hint = Po zmianie tego, co przyjmuje lokalizacja: już zapisane pliki zostaną przeniesione do lokalizacji, do której teraz trafia ich rodzaj.
locations-add-title = Dodaj lokalizację
locations-add-hint = Inny dysk dla niektórych rodzajów plików albo istniejący folder z paczkami.
locations-s3-title = Magazyn S3
locations-s3-hint = Lokalizacja s3://bucket-name to zasobnik w tej usłudze: AWS lub dowolnej usłudze zgodnej z S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage na serwerze NAS).
locations-s3-endpoint = Adres
locations-s3-endpoint-placeholder = Puste dla AWS; np. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Region
locations-s3-access-key = Identyfikator klucza dostępu
locations-s3-secret-key = Tajny klucz dostępu
locations-s3-headers = Dodatkowe nagłówki
locations-s3-headers-placeholder = Name=value, jeden w wierszu (np. dla Cloudflare Access)
locations-s3-kept = Zapisano (wpisz, aby zastąpić)
locations-s3-secrets = Klucze i nagłówki są przechowywane razem z ustawieniami serwera i nigdy więcej nie są wyświetlane.
locations-s3-secrets-desktop = Klucze i nagłówki są przechowywane w pęku kluczy tego komputera i nigdy więcej nie są wyświetlane.
locations-s3-saved = Zapisano magazyn S3
locations-s3-save-failed = Nie udało się zapisać magazynu S3
locations-add-more = Dodaj lokalizację…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Nie udało się załadować problemów
issues-started =
    { $count ->
        [one] Uruchomiono ponownie.
        [few] Ponownie uruchomiono { $count } elementy.
        [many] Ponownie uruchomiono { $count } elementów.
       *[other] Ponownie uruchomiono { $count } elementu.
    }
issues-retry-failed = Nie udało się ponowić
issues-dismiss-failed = Nie udało się odrzucić
issues-dismiss-all-title = Odrzucić wszystkie ({ $count })?
issues-dismiss-all-body = Grupa „{ $title }” zostanie wyczyszczona. Wszystko, co znów się nie powiedzie, pojawi się ponownie.
issues-dismiss-all = Odrzuć wszystkie
issues-just-now = przed chwilą
issues-minutes-ago = { $mins } min temu
issues-hours-ago = { $hours } godz. temu
issues-loading = Ładowanie…
issues-none = Nic nie poszło źle.
issues-action-all = { $action } wszystkie
issues-open-asset = Otwórz
issues-open-pack = Pokaż paczkę
issues-open-locations = Lokalizacje biblioteki
issues-where-before = w
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } raz
        [few] { $count } razy
        [many] { $count } razy
       *[other] { $count } raza
    }
issues-starting = Uruchamianie…
issues-dismiss-named = Odrzuć { $title }
issues-dismiss = Odrzuć
issues-show-fewer = Pokaż mniej
issues-show-all = Pokaż wszystkie ({ $count })
issues-not-listed = Niewymienione: jeszcze { $count }.

## archive.ts

archive-anyway = Zarchiwizuj mimo to
archive-failed = Nie udało się zarchiwizować: { $error }
archive-undo = Cofnij
archive-restore-failed = Nie udało się przywrócić: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } paczka
        [few] { $packs } paczki
        [many] { $packs } paczek
       *[other] { $packs } paczki
    } i { $assets ->
        [one] { $assets } zasób
        [few] { $assets } zasoby
        [many] { $assets } zasobów
       *[other] { $assets } zasobu
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } paczka
        [few] { $packs } paczki
        [many] { $packs } paczek
       *[other] { $packs } paczki
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } zasób
        [few] { $assets } zasoby
        [many] { $assets } zasobów
       *[other] { $assets } zasobu
    }
archive-count-nothing = nic

## ArchiveView.vue

archive-load-failed = Nie udało się załadować archiwum: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Przywrócono: { $what }
archive-purged = Usunięto na stałe: { $what }
archive-purge-failed = Nie udało się usunąć: { $error }
archive-purge-title = Usunąć { $name } na stałe?
archive-purge-pack-body = Usunięte zostaną też jej zasoby ({ $count }) i { $size } plików, z wyjątkiem plików współdzielonych z inną paczką. Tego nie można cofnąć.
archive-purge = Usuń na zawsze
archive-purge-asset-body = Jego plik zostanie zwolniony, chyba że współdzieli go inny zasób. Tego nie można cofnąć.
archive-empty-title = Opróżnić archiwum?
archive-empty-body = Na stałe zostanie usunięte: { $what }, a ich pliki zwolnione. Tego nie można cofnąć.
archive-empty = Opróżnij archiwum
archive-title = Archiwum
archive-intro = Zarchiwizowane paczki i zasoby nie pojawiają się w wyszukiwaniu, licznikach ani na liście tagów. Można je przywrócić albo usunąć na stałe, by zwolnić miejsce.
archive-loading = Ładowanie…
archive-is-empty = Archiwum jest puste
archive-is-empty-hint = Zarchiwizowana paczka lub zasób czeka tutaj, dopóki nie zostanie przywrócony lub usunięty na stałe.
archive-packs = Paczki
archive-assets = Zasoby
archive-asset-count =
    { $count ->
        [one] { $count } zasób
        [few] { $count } zasoby
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
archive-archived = zarchiwizowano { $when }
archive-from = z { $pack }
archive-restore = Przywróć
archive-show-more = Pokaż więcej (pozostało { $count })

## mine.ts

projects-favorite-add-failed = Nie udało się dodać do ulubionych: { $error }
projects-favorite-remove-failed = Nie udało się usunąć z ulubionych: { $error }
projects-save-search-failed = Nie udało się zapisać wyszukiwania: { $error }
projects-forget-search-failed = Nie udało się usunąć { $name }: { $error }

## projects.ts

projects-list-and = { $items } i { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = jeszcze { $count }
projects-other =
    { $count ->
        [one] { $count } inny projekt
        [few] { $count } inne projekty
        [many] { $count } innych projektów
       *[other] { $count } innego projektu
    }
projects-usage-title-one = { $name } tego używa
projects-usage-title =
    { $count ->
        [one] { $count } projekt tego używa
        [few] { $count } projekty tego używają
        [many] { $count } projektów tego używa
       *[other] { $count } projektu tego używa
    }
projects-usage-body-one = Używane w { $list }: następne hoard sync je usunie.
projects-usage-body = Używane w { $list }: następne hoard sync każdego z nich je usunie.

## uploader.ts

upload-network-error = błąd sieci
upload-gave-up = { $what }: { $error } (prób: { $attempts })
upload-failed-plain = niepowodzenie
upload-already-imported = już zaimportowano
upload-part-failed = przesyłanie nie powiodło się ({ $status })
upload-batch =
    { $count ->
        [one] partia { $count } pliku
        [few] partia { $count } plików
        [many] partia { $count } plików
       *[other] partia { $count } pliku
    }
upload-batch-failed = partia nie powiodła się ({ $status })
upload-finish = zakończenie przesyłania

## UploadQueue.vue

upload-eta-seconds = pozostało { $s } s
upload-eta-minutes = pozostało { $min } min
upload-eta-hours = pozostało { $h } godz.
upload-queue = Importy
upload-importing =
    { $count ->
        [one] Importowanie { $count } paczki
        [few] Importowanie { $count } paczek
        [many] Importowanie { $count } paczek
       *[other] Importowanie { $count } paczki
    }
upload-bytes-of = { $sent } z { $total }
upload-rate = { $size }/s
upload-waiting = Oczekiwanie
upload-unpacking = Przesłano, trwa rozpakowywanie
upload-already = Już w bibliotece
upload-stopped = Zatrzymano
upload-files-of =
    { $total ->
        [one] { $done } z { $total } pliku
        [few] { $done } z { $total } plików
        [many] { $done } z { $total } plików
       *[other] { $done } z { $total } pliku
    }
upload-resumed = wznowiono, { $size } było już na miejscu
upload-skipped = Dokładnie te pliki zaimportowano już jako „{ $name }”, więc zostały pominięte.
upload-error = { $error }. Postęp został zachowany: można ponowić albo później ponownie upuścić te same pliki.
upload-retry = Ponów
upload-import-again = Importuj mimo to
upload-dismiss = Odrzuć
upload-cancel = Anuluj

## PacksView.vue

packs-rename-failed = Nie udało się zmienić nazwy: { $error }
packs-collection-deleted = Usunięto kolekcję { $name }
packs-delete-failed = Nie udało się usunąć: { $error }
packs-status-uploading = Przesyłanie (pliki odebrane do tej pory)
packs-status-ingesting = Rozpakowywanie i tagowanie
packs-status-analyzing = LLM taguje
packs-status-ready = Gotowe
packs-status-failed = Niepowodzenie
packs-retag-failed = Nie udało się rozpocząć ponownego tagowania: { $error }
packs-read-files-failed = Nie udało się odczytać plików paczki: { $error }
packs-context-load-failed = Nie udało się załadować informacji o paczce ze sklepu: { $error }
packs-undo-store-failed = Nie udało się odłączyć strony sklepu: { $error }
packs-context-save-failed = Nie udało się zapisać informacji ze sklepu: { $error }
packs-unreal-load-failed = Nie udało się załadować stanu konwersji Unreal: { $error }
packs-unreal-discard-title = Odrzucić oryginały?
packs-unreal-discard-body = Oryginalne pliki .uasset w { $name } zostaną usunięte po konwersji. Dane dostępne tylko w Unreal, takie jak grafy materiałów, zostaną utracone na zawsze.
packs-unreal-discard-confirm = Konwertuj i odrzuć
packs-unreal-convert-failed = Nie udało się rozpocząć konwersji: { $error }
packs-unreal-fact-converted = Przekonwertowano z Unreal: { $done }
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } pakiet Unreal
        [few] { $left } pakiety Unreal
        [many] { $left } pakietów Unreal
       *[other] { $left } pakietu Unreal
    }
packs-unreal-fact-both = Przekonwertowano z Unreal: { $done }, nadal pakiety: { $left }
packs-thumbs-canceled = Anulowano po { $done } z { $total }
packs-thumbs-drew =
    { $count ->
        [one] Narysowano { $count } model
        [few] Narysowano { $count } modele
        [many] Narysowano { $count } modeli
       *[other] Narysowano { $count } modelu
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: zastąpiono { $improved } miniaturę.
        [few] { $lead }: zastąpiono { $improved } miniatury.
        [many] { $lead }: zastąpiono { $improved } miniatur.
       *[other] { $lead }: zastąpiono { $improved } miniatury.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: zastąpiono { $improved } miniaturę, nie udało się narysować: { $failed }.
        [few] { $lead }: zastąpiono { $improved } miniatury, nie udało się narysować: { $failed }.
        [many] { $lead }: zastąpiono { $improved } miniatur, nie udało się narysować: { $failed }.
       *[other] { $lead }: zastąpiono { $improved } miniatury, nie udało się narysować: { $failed }.
    }
packs-archived = Zarchiwizowano { $name } i jej zasoby ({ $count })
packs-blobs-bad =
    { $count ->
        [one] { $count } plik nie przeszedł ostatniej kontroli integralności.
        [few] { $count } pliki nie przeszły ostatniej kontroli integralności.
        [many] { $count } plików nie przeszło ostatniej kontroli integralności.
       *[other] { $count } pliku nie przeszło ostatniej kontroli integralności.
    }
packs-collections = Kolekcje
packs-browse-named = Przeglądaj { $name }
packs-collection-name = Nazwa kolekcji
packs-save = Zapisz
packs-cancel = Anuluj
packs-assets =
    { $count ->
        [one] { $count } zasób
        [few] { $count } zasoby
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
packs-updated = Zaktualizowano { $date }
packs-browse = Przeglądaj
packs-collection-download = Pobierz (.zip)
packs-collection-credits = Autorzy
packs-collection-credits-title = Kto stworzył paczki, z których pochodzą te zasoby, ich licencje i wymagane przez nie podziękowania
packs-rename = Zmień nazwę
packs-delete = Usuń
packs-packs = Paczki
packs-empty = Nie ma jeszcze paczek. Aby dodać paczkę, wystarczy upuścić plik zip lub folder w dowolnym miejscu tej strony.
packs-find = Znajdź paczkę
packs-find-placeholder = Znajdź paczkę po nazwie, sklepie, licencji lub tagu
packs-sort = Kolejność
packs-sort-new = Najpierw najnowsze
packs-sort-old = Najpierw najstarsze
packs-sort-name = Według nazwy
packs-sort-size = Najpierw największe
packs-sort-assets = Najpierw z największą liczbą zasobów
packs-with-kind = Paczki z
packs-state = Stan
packs-state-any = Dowolny stan
packs-state-working = W toku
packs-state-failed = Niepowodzenie
packs-state-untagged = Jeszcze bez opisu
packs-store = Sklep
packs-store-any = Dowolny sklep
packs-store-from = Z { $store }
packs-store-none = Bez strony w sklepie
packs-shown-of = { $shown } z { $total }
packs-clear = Wyczyść
packs-no-match = Żadna paczka nie pasuje.
packs-clear-all = Wyczyść wyszukiwanie i filtry
packs-pack-name = Nazwa paczki
packs-thumbs-drawing = Rysowanie miniatur w tej przeglądarce…
packs-thumbs-drawing-count = Rysowanie miniatur w tej przeglądarce: { $done } z { $total }…
packs-thumbs-stopped = Zatrzymano: { $error }
packs-thumbs-none = Każdy model ma już miniaturę narysowaną ze wszystkimi teksturami.
packs-dismiss = Odrzuć
packs-license = Licencja: { $license }
packs-license-tier = Licencja: { $license } ({ $tier })
packs-license-not-found = nie znaleziono
packs-from-before = Z
packs-from-after = { "" }
packs-added = Dodano { $date }
packs-download = Pobierz paczkę (.zip)
packs-store-details = Informacje ze sklepu
packs-more-named = Więcej działań dla { $name }
packs-more = Więcej
packs-rename-more = Zmień nazwę…
packs-retag-rules = Uruchom ponownie tagowanie offline
packs-retag-llm = Uruchom ponownie tagowanie LLM
packs-make-thumbs = Utwórz miniatury w tej przeglądarce
packs-make-thumbs-title = Otwiera każdy model w ukrytej przeglądarce 3D, jeden po drugim, i używa jego obrazu jako miniatury tam, gdzie jest lepszy niż ten z serwera
packs-make-thumbs-title-desktop = Otwiera każdy model w ukrytej przeglądarce 3D, jeden po drugim, i używa jego obrazu jako miniatury tam, gdzie jest lepszy niż ten utworzony przez Hoard
packs-read-files = Odczytaj ponownie pliki paczki
packs-credits = Pobierz listę autorów
packs-credits-title = CREDITS.md: kto stworzył paczkę, jej licencja i wymagane podziękowanie
packs-unreal-open = Konwertuj zawartość Unreal…
packs-archive = Zarchiwizuj paczkę
packs-unreal-intro = Hoard sam odczytuje pakiety Unreal i przechowuje ich zawartość jako zwykłe pliki: tekstury jako PNG lub EXR, dźwięki jako WAV, materiały jako JSON. Przekonwertowany plik zajmuje miejsce pakietu, zachowując jego tagi, powiązania i kolekcje.
packs-unreal-converted =
    { $count ->
        [one] przekonwertowany
        [few] przekonwertowane
        [many] przekonwertowanych
       *[other] przekonwertowanego
    }
packs-unreal-converted-kept =
    { $count ->
        [one] przekonwertowany, { $kept } z zachowanym oryginałem
        [few] przekonwertowane, { $kept } z zachowanym oryginałem
        [many] przekonwertowanych, { $kept } z zachowanym oryginałem
       *[other] przekonwertowanego, { $kept } z zachowanym oryginałem
    }
packs-unreal-pending =
    { $count ->
        [one] jeszcze niesprawdzony
        [few] jeszcze niesprawdzone
        [many] jeszcze niesprawdzonych
       *[other] jeszcze niesprawdzonego
    }
packs-unreal-needs-editor =
    { $count ->
        [one] czeka na Unreal Editor
        [few] czekają na Unreal Editor
        [many] czeka na Unreal Editor
       *[other] czeka na Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] bez popularnego formatu
    }
packs-unreal-failed =
    { $count ->
        [one] nieudany
        [few] nieudane
        [many] nieudanych
       *[other] nieudanego
    }
packs-unreal-reasons = Dlaczego pakiety pozostały
packs-unreal-editor-before = Siatki i animacje wymagają Unreal Editor. Na komputerze z zainstalowanym Unreal należy uruchomić
packs-unreal-editor-command = hoard unreal-worker --editor <ścieżka do UnrealEditor-Cmd>
packs-unreal-editor-after = ; eksportuje je to jako FBX i przesyła wyniki tutaj.
packs-unreal-discard = Odrzuć oryginalne pliki .uasset po konwersji
packs-unreal-discard-hint = Zwalnia miejsce. Dane dostępne tylko w Unreal, takie jak grafy materiałów, zostaną utracone.
packs-unreal-drop = Usuń pakiety bez popularnego formatu
packs-unreal-drop-hint = Blueprinty, poziomy i inne zasoby działające tylko w silniku.
packs-unreal-convert = Konwertuj
packs-context-hint-before = Skąd pochodzi paczka i co mówi o niej sprzedawca. Tagowanie LLM czyta to przy każdym pliku, więc opis ze sklepu, np. „120 czarów lodu i ognia”, pomaga mu nazwać
packs-context-hint-after = .
packs-store-auto = Znaleziono automatycznie: paczka sama wskazuje tę stronę.
packs-store-not-this = To nie ta
packs-lookup-label = Szukaj w sklepach
packs-lookup-placeholder = Nazwa paczki w sklepie
packs-lookup-searching = Wyszukiwanie…
packs-lookup-search = Szukaj w sklepach
packs-lookup-hide = Ukryj
packs-lookup-clues = Z plików paczki: { $clues }.
packs-lookup-also = Szukano też: { $queries }.
packs-lookup-quoted = „{ $query }”
packs-lookup-nothing = Nic na Fab, itch.io, Kenney ani OpenGameArt nie wygląda podobnie. Warto spróbować mniejszej liczby słów lub innych słów.
packs-lookup-use = Użyj
packs-lookup-searched = Wyszukano { $date }.
packs-lookup-again = Szukaj ponownie
packs-lookup-again-after = .
packs-lookup-unreachable = Tym razem nie udało się połączyć z { $stores }.
packs-lookup-also-look = Warto też poszukać na
packs-lookup-links-and = { " " }i{ " " }
packs-lookup-browser-only = (można je przeszukać tylko w przeglądarce); adres strony należy wkleić poniżej.
packs-lookup-open = Znajdź w sklepach…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Sprzedawca
packs-seller-placeholder = Kto to stworzył
packs-license-label = Licencja
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Poziom licencji
packs-license-tier-placeholder = Personal, Professional…
packs-store-page = Strona w sklepie
packs-store-text = Opis ze sklepu
packs-store-text-placeholder = Można wkleić tekst oferty albo zostawić puste, by pobrać go ze strony sklepu.
packs-store-text-fetch = Zapisanie pobierze opis ze strony sklepu.
packs-readme = README znalezione w paczce
packs-save-retag = Zapisz i uruchom ponownie tagowanie LLM

## AssetDetail.vue

asset-lazy-3d-viewer = przeglądarki 3D
asset-lazy-text = tekstu
asset-lazy-text-viewer = przeglądarki tekstu
asset-lazy-image-viewer = przeglądarki obrazów
asset-lazy-material-preview = podglądu materiału
asset-lazy-material-viewer = przeglądarki materiałów
asset-lazy-sprite-player = odtwarzacza sprite’ów
asset-lazy-sheet-view = przeglądarki arkuszy sprite’ów
asset-rel-texture = tekstura
asset-rel-buffer = dane geometrii
asset-rel-material = plik materiału
asset-rel-animation = animacja
asset-rel-library = powiązany plik
asset-rel-guess = prawdopodobnie pasuje
asset-rel-sidecar = informacje ze sklepu
asset-rel-variant = wariant
asset-rel-sprite = sprite
asset-rel-sample = sampel
asset-role-map = mapa: { $role }
asset-maps-count =
    { $count ->
        [one] { $count } mapa
        [few] { $count } mapy
        [many] { $count } map
       *[other] { $count } mapy
    }
asset-needs = Wymaga
asset-animations = Animacje
asset-sprites = Sprite’y
asset-used-by = Używany przez
asset-loose-sprites = Także jako osobne pliki
asset-in-sheet = W arkuszu sprite’ów
asset-other-formats = Inne formaty
asset-shown-as = W bibliotece pokazany jako
asset-conversion-failed = konwersja nie powiodła się
asset-as-converting = Konwertowanie…
asset-as-converting-percent = Konwertowanie… { $percent }%
asset-as-queued = W kolejce…
asset-download = Pobierz
asset-as-label = Pobierz jako
asset-as-original = { $format } (format oryginalny)
asset-as-pack-files = Pliki z paczki
asset-as-size = Rozmiar
asset-as-full-size = Pełny rozmiar
asset-as-up-to = Do { $size } px
asset-as-hint = Można wybrać rozmiar lub inny format
asset-meta-size = Rozmiar
asset-meta-length = Długość
asset-meta-sample-rate = Częstotliwość próbkowania
asset-meta-channels = Kanały
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Głębia bitowa
asset-meta-bits =
    { $bits ->
        [one] { $bits } bit
        [few] { $bits } bity
        [many] { $bits } bitów
       *[other] { $bits } bitu
    }
asset-meta-level = Poziom
asset-meta-level-value = szczyt { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = szczyt { $peak } dBFS, RMS { $rms } dBFS, przesterowanie
asset-meta-loop = Pętla
asset-meta-loop-value = od { $start } do { $end }
asset-meta-playback = Odtwarzanie
asset-meta-one-shot = Jednorazowe
asset-meta-envelope = Obwiednia
asset-meta-envelope-times = atak { $attack }, wybrzmiewanie { $tail }
asset-meta-envelope-hit = uderzenie
asset-meta-envelope-swell = narastanie
asset-meta-envelope-sustained = podtrzymany
asset-meta-tone = Barwa
asset-meta-tone-value = jasność { $khz } kHz, szum { $noise }%
asset-meta-pitch = Wysokość
asset-meta-pitch-rises =
    { $octaves ->
        [one] wznosi się o { $octaves } oktawę
        [few] wznosi się o { $octaves } oktawy
        [many] wznosi się o { $octaves } oktaw
       *[other] wznosi się o { $octaves } oktawy
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] opada o { $octaves } oktawę
        [few] opada o { $octaves } oktawy
        [many] opada o { $octaves } oktaw
       *[other] opada o { $octaves } oktawy
    }
asset-meta-loops-cleanly = Zapętla się płynnie
asset-meta-tempo = Tempo
asset-meta-tempo-heard = około { $bpm } BPM, na słuch
asset-meta-tempo-heard-or = około { $bpm } BPM (lub { $alt }), na słuch
asset-meta-key = Tonacja
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, na słuch
music-key-major = { $note }-dur
music-key-minor = { $note }-moll
asset-meta-root-note = Tonacja
asset-meta-cue-markers = Znaczniki cue
asset-meta-library = Biblioteka
asset-meta-keywords = Słowa kluczowe
asset-meta-title = Tytuł
asset-meta-artist = Wykonawca
asset-meta-album = Album
asset-meta-genre = Gatunek
asset-meta-generator = Generator
asset-meta-everything-with = Pokaż wszystko z tagiem { $tag }
asset-meta-saved-with = Zapisano w
asset-meta-color-mode = Tryb kolorów
asset-meta-layers = Warstwy
asset-meta-range = Zakres
asset-meta-hdr = HDR (podgląd z mapowaniem tonów)
asset-meta-format = Format
asset-meta-large-document = Duży dokument (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Utworzone przez generatywną AI
asset-meta-ai-edited = Edytowane przez generatywną AI
asset-meta-ai-how-metadata = metadane
asset-meta-ai-how-generator = ustawienia generatora
asset-meta-ai-how-iptc = typ źródła cyfrowego IPTC
asset-meta-ai-says = { $made } — według: { $how }
asset-meta-ai-by-says = { $made } ({ $by }) — według: { $how }
asset-meta-provenance = Pochodzenie
asset-meta-c2pa = Ma poświadczenia treści C2PA
asset-meta-compression = Kompresja
asset-meta-note = Uwaga
asset-meta-maps = Mapy
asset-meta-sizes =
    { $count ->
        [one] Rozmiar
        [few] Rozmiary
        [many] Rozmiary
       *[other] Rozmiary
    }
asset-meta-triangles = Trójkąty
asset-meta-vertices = Wierzchołki
asset-meta-meshes = Siatki
asset-meta-materials = Materiały
asset-meta-animations = Animacje
asset-meta-skins = Skórki (skins)
asset-meta-unreal-asset = Zasób Unreal
asset-meta-unreal-saved-with = { $class }, zapisany w UE { $engine }
asset-meta-package = Pakiet
asset-meta-cooked = Skompilowany (build gry): podglądy i źródła są usunięte
asset-meta-tiling = Kafelkowanie
asset-meta-seamless = Bezszwowy w obu osiach
asset-meta-analysis-error = Nie udało się w pełni odczytać tego pliku: { $error }
asset-copy-blocked = Nie udało się skopiować: przeglądarka to zablokowała
asset-save-error = Nie udało się zapisać: { $error }
asset-tag-removed = Usunięto { $tag }
asset-undo = Cofnij
asset-play-error = Nie udało się odtworzyć tego pliku: { $error }
asset-copied-hash = Skopiowano skrót
asset-copied-toml = Skopiowano toml
asset-copied-id = Skopiowano ID
asset-copied-link = Skopiowano link
asset-origin-before = Pierwotnie ten plik to
asset-origin-class-engine = , obiekt { $class } zapisany przez UE { $engine }.
asset-origin-class = , obiekt { $class }.
asset-origin-engine = { " " }zapisany przez UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Wyeksportowany przez Unreal Editor { $date }.
asset-origin-editor-notes = Wyeksportowany przez Unreal Editor { $date } ({ $notes }).
asset-origin-hoard = Odczytany z pakietu przez Hoard { $date }.
asset-origin-hoard-notes = Odczytany z pakietu przez Hoard { $date } ({ $notes }).
asset-origin-download = Pobierz oryginalny plik .{ $ext }
asset-origin-discarded = Oryginalny pakiet został odrzucony po konwersji.
asset-details = Szczegóły zasobu
asset-in-pack = w
asset-previous = Poprzedni zasób
asset-next = Następny zasób
asset-favorite = Ulubione
asset-favorite-on-title = W ulubionych (F usuwa)
asset-favorite-off-title = Dodaj do ulubionych (F)
asset-share = Udostępnij
asset-share-title = Udostępnij link do tego zasobu (kto go otworzy, najpierw zaloguje się do Hoard)
asset-layout-panel = Pokaż w panelu bocznym
asset-layout-page = Pokaż na całej stronie
asset-layout-page-title = Pokaż na całej stronie: duży podgląd obok szczegółów
asset-close = Zamknij szczegóły
asset-rename-title = Kliknij, aby zmienić nazwę
asset-name = Nazwa
asset-views-alt = Model z czterech stron
asset-inspect-aria = Obejrzyj { $name } na pełnym ekranie
asset-inspect-title = Obejrzyj: powiększenie, kafelki, kanały
asset-playback-position = Pozycja odtwarzania
asset-playback-value = { $current } z { $duration }
asset-pause = Wstrzymaj
asset-play = Odtwórz
asset-waveform-drawing = Rysowanie przebiegu…
asset-waveform-skipped = Brak przebiegu dla tak dużych plików
asset-play-animation = Odtwórz jako animację
asset-show-views = Pokaż arkusz czterech widoków, który widziało tagowanie
asset-palette = Kolory na obrazie
asset-swatch-title = { $hex }, { $percent }% obrazu. Znajdź zasoby z tym kolorem
asset-swatch-aria = Znajdź zasoby z kolorem { $hex }, { $percent }% tego obrazu
asset-more-like-this = Więcej podobnych
asset-desc-label = Własny opis
asset-desc-placeholder = Co to jest, gdzie można tego użyć…
asset-save = Zapisz
asset-cancel = Anuluj
asset-desc-family = Zapisuje we wszystkich wariantach ({ $count })
asset-desc-edit = Edytuj opis
asset-desc-write = Napisz opis
asset-desc-ai = Napisany przez AI
asset-desc-file = Z pliku
asset-desc-store-before = Ze
asset-desc-store-page = strony { $store }
asset-desc-store-page-unnamed = strony sklepu
asset-desc-store-after = (cała paczka)
asset-less = Mniej
asset-more = Więcej
asset-tags = Tagi
asset-tag-add-label = Dodaj tag
asset-tag-add-placeholder = Dodaj tag, np. mood:tense
asset-tag-add = Dodaj
asset-apply-family = Zastosuj zmiany tagów, kategorii i opisu do wszystkich wariantów ({ $count })
asset-collections = Kolekcje
asset-collection-remove = Usuń z { $name }
asset-collection-add-another = Dodaj do innej
asset-collection-add = Dodaj do kolekcji
asset-used-in = Używany w
asset-used-in-title = Pokaż, co ma { $name }
asset-used-in-synced = synchronizacja: { $when }
asset-maps = Mapy
asset-show-more = Pokaż jeszcze { $count }
asset-materials-heading =
    { $count ->
        [one] Materiał
        [few] Materiały
        [many] Materiały
       *[other] Materiały
    }
asset-material-maps-aria = { $name }: mapy
asset-variants = Warianty ({ $count })
asset-copies = Identyczne kopie
asset-copies-note = Te same bajty są przechowywane raz; te paczki również zawierają ten plik.
asset-near-copies = Prawie kopie
asset-near-copies-note = To samo w innym rozmiarze, częstotliwości próbkowania lub formacie. Najlepsza kopia jest oznaczona.
asset-near-copies-best = To samo w innym rozmiarze, częstotliwości próbkowania lub formacie. Ta jest najlepszą kopią.
asset-near-best = najlepsza
asset-unreal-converted = Przekonwertowano z Unreal
asset-rule-converted = Przekonwertowano regułą
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Reguła konwersji utworzyła ten plik { $format } dnia { $date } z pliku
asset-rule-kept-until = Oryginał jest przechowywany do { $date }.
asset-rule-put-back = Przywróć oryginał
asset-rule-put-back-title = Przywrócić { $file }?
asset-rule-put-back-body = Przekonwertowany plik zostanie usunięty, a jego miejsce zajmie oryginał, z jego tagami, kolekcjami i powiązaniami. Od tej pory reguły konwersji go pomijają.
asset-rule-put-back-done = Przywrócono { $file }
asset-rule-let-go = Oryginał został usunięty, gdy minął czas przechowywania oryginałów ustawiony w Ustawieniach.
asset-unreal-package = Z pakietu Unreal
asset-ue-needs-editor-before = Czeka na Unreal Editor: { $reason }. Na komputerze z zainstalowanym Unreal należy uruchomić
asset-ue-needs-editor-after = , a plik wróci jako FBX.
asset-ue-editor-path = ścieżka do UnrealEditor-Cmd
asset-ue-unsupported = Zachowano jako pakiet: { $reason }.
asset-ue-failed = Konwersja nie powiodła się: { $reason }
asset-recovered = Oryginalny plik .{ $ext } był zapisany w tym pakiecie, a Hoard go wydobył.
asset-recovered-download = Pobierz .{ $ext }
asset-ue-meshes-before = Siatki i animacje są zapisane we własnym formacie Unreal. Należy przekonwertować paczkę (strona paczki, Więcej), a potem uruchomić
asset-ue-meshes-after = obok instalacji Unreal Editor, by otrzymać FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } tag rejestru zasobów
        [few] { $count } tagi rejestru zasobów
        [many] { $count } tagów rejestru zasobów
       *[other] { $count } tagu rejestru zasobów
    }
asset-file = Plik
asset-bytes = Bajty
asset-source = Źródło
asset-path = Ścieżka
asset-path-open-folder = Zobacz, co jeszcze jest w { $folder }
asset-download-maps =
    { $count ->
        [one] Pobierz { $count } mapę
        [few] Pobierz wszystkie { $count } mapy
        [many] Pobierz wszystkie { $count } map
       *[other] Pobierz wszystkie { $count } mapy
    }
asset-download-linked =
    { $count ->
        [one] Pobierz z { $count } powiązanym plikiem
        [few] Pobierz z { $count } powiązanymi plikami
        [many] Pobierz z { $count } powiązanymi plikami
       *[other] Pobierz z { $count } powiązanego pliku
    }
asset-download-only-title = Tylko ten plik, bez plików, których używa
asset-download-only = Tylko ten plik
asset-include-animations = Dołącz animacje do pliku .zip
asset-copy-toml = Kopiuj wpis hoard.toml
asset-copy-id = Kopiuj ID
asset-teach = Naucz Hoard tego pliku
asset-teach-title = Opisz ten plik sklepu, aby Hoard go odczytywał (Ustawienia → Pliki sklepów)
asset-archive = Zarchiwizuj
asset-archive-title = Zniknie z wyszukiwania, dopóki nie zostanie przywrócony lub usunięty na stałe z Archiwum

## ImageViewer.vue

image-channel-red = Kanał czerwony
image-channel-green = Kanał zielony
image-channel-blue = Kanał niebieski
image-channel-alpha = Kanał alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Tło w szachownicę (K przełącza)
image-bg-black-title = Czarne tło (K przełącza)
image-bg-gray-title = Szare tło (K przełącza)
image-bg-white-title = Białe tło (K przełącza)
image-bg-checker = tło w szachownicę
image-bg-black = czarne tło
image-bg-gray = szare tło
image-bg-white = białe tło
image-inspect-aria = Obejrzyj { $name }
image-preview-size = podgląd { $width }×{ $height }, { $bytes }
image-preview = podgląd
image-resolution = Rozdzielczość
image-full-title = Wczytaj pełny obraz { $width }×{ $height } (F); to samo dzieje się po powiększeniu ponad podgląd
image-full-loading = Wczytywanie pełnej rozdzielczości…
image-full-failed = Nie wczytano pełnej rozdzielczości: ponów
image-full = Pełna rozdzielczość
image-zoom = Powiększenie
image-zoom-out-title = Pomniejsz (−)
image-zoom-out = Pomniejsz
image-zoom-in-title = Powiększ (+)
image-zoom-in = Powiększ
image-fit-title = Dopasuj (0)
image-fit = Dopasuj
image-actual-title = Rzeczywiste piksele (1)
image-view = Widok
image-tile-title = Powiel 3×3, by sprawdzić szwy (T)
image-tile = Kafelki
image-channels = Kanały
image-close = Zamknij (Esc)
image-failed = Nie udało się wczytać tego obrazu.
image-loading = Ładowanie…
image-key-gestures = Przewijanie lub szczypanie powiększa · przeciąganie przesuwa · dwuklik powiększa lub dopasowuje
image-key-tile = kafelki
image-key-background = tło
image-key-full = pełna rozdzielczość
image-key-channels = kanały

## SpritePlayer.vue

sprite-frame-aria = { $name }, klatka { $frame } z { $count }
sprite-failed = Nie można tego odtworzyć.
sprite-loading = Wczytywanie klatek…
sprite-previous = Poprzednia klatka
sprite-next = Następna klatka
sprite-pause = Wstrzymaj animację
sprite-play = Odtwórz animację
sprite-animation = Animacja
sprite-all-frames = Wszystkie klatki
sprite-grid-title = Jak arkusz jest dzielony na klatki
sprite-columns = Kolumny
sprite-rows = Wiersze
sprite-fps-aria = Klatki na sekundę
sprite-fps = kl./s

## Przeglądarka arkuszy sprite’ów (SheetView.vue) i jej formaty pobierania (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
        [few] { $count } sprite’y
        [many] { $count } sprite’ów
       *[other] { $count } sprite’a
    }
sheet-pages =
    { $count ->
        [one] { $count } strona
        [few] { $count } strony
        [many] { $count } stron
       *[other] { $count } strony
    }
sheet-animations =
    { $count ->
        [one] { $count } animacja
        [few] { $count } animacje
        [many] { $count } animacji
       *[other] { $count } animacji
    }
# The box that narrows the list of sprites by name.
sheet-find = Znajdź sprite’a
# Under the list of sprites when more match than it draws.
sheet-showing = Wyświetlono { $shown } z { $total }
sheet-no-match = Żaden sprite nie ma tego w nazwie.
sheet-failed = Nie można odczytać tego arkusza.
sheet-loading = Wczytywanie arkusza…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Własny format Hoard: strony bez zmian, sprite’y obrócone w dowolną stronę i przycięte, z animacjami i czasem każdej klatki.
sheet-note-texturepacker = Sprite’y obrócone zgodnie z ruchem wskazówek zegara i przycięte, z animacjami według nazw, tak jak odczytują je PixiJS i Phaser.
sheet-note-clockwise = Sprite’y obrócone zgodnie z ruchem wskazówek zegara i przycięte. Ten format nie obsługuje animacji: klatki rozpoznaje się po numerach w nazwach.
sheet-note-libgdx = Sprite’y obrócone przeciwnie do ruchu wskazówek zegara i przycięte. Ten format nie obsługuje animacji: klatki rozpoznaje się po numerach w nazwach.
sheet-note-godot-frames = Sprite’y nigdy nieobracane, a przycięte brzegi zachowane jako marginesy, z animacjami i czasem każdej klatki.
sheet-note-godot-tileset = Kafelki nigdy nieobracane ani nieprzycinane, na siatce o rozmiarze kafelka; obrazy, które już są na siatce, zostają bez zmian.
sheet-note-grid = Każdy sprite nieobrócony, na całym swoim płótnie, w jednakowych komórkach, dla każdego narzędzia, które tnie arkusz według siatki.
sheet-note-frames = Każdy sprite nieobrócony, na całym swoim płótnie, każdy jako osobny plik PNG nazwany tak jak on.

## TextViewer.vue, TextPreview.vue i text.ts

text-lang-config = Konfiguracja
text-lang-table = Tabela
text-lang-code = Kod
text-lang-script = Skrypt
text-lang-text = Tekst
text-copy-blocked = Nie udało się skopiować: przeglądarka to zablokowała
text-read-aria = Czytaj { $name }
text-lines =
    { $count ->
        [one] { $count } wiersz
        [few] { $count } wiersze
        [many] { $count } wierszy
       *[other] { $count } wiersza
    }
text-show-as = Pokaż jako
text-tree-title = Zwijane drzewo (T)
text-tree = Drzewo
text-pretty-title = Z nowymi wcięciami (P)
text-pretty = Sformatowany
text-raw-title = Tak, jak zapisano (R)
text-raw = Surowy
text-document-title = Jako dokument (D)
text-document = Dokument
text-source-title = Markdown tak, jak zapisano (R)
text-source = Źródło
text-fold = Zwiń
text-expand-all-title = Rozwiń wszystko
text-expand-all = Rozwiń wszystko
text-collapse-all-title = Zwiń wszystko
text-collapse-all = Zwiń wszystko
text-group-text = Tekst
text-wrap-title = Zawijaj długie wiersze (W)
text-wrap = Zawijaj
text-copy-title = Kopiuj tekst (C)
text-copied = Skopiowano
text-copy = Kopiuj
text-download-title = Pobierz plik
text-download = Pobierz
text-close = Zamknij (Esc)
text-truncated = Wyświetlono początek: { $shown } z { $total }.
text-truncated-json = Wyświetlono początek: { $shown } z { $total } (drzewo wymaga całego pliku).
text-show-all = Pokaż wszystko
text-json-error = Tego JSON-a nie da się przetworzyć: { $error }
text-contents-aria = Zawartość { $name }
text-loading = Ładowanie…
text-load-error = Nie udało się załadować tego pliku: { $error }
text-binary = To plik binarny, a nie tekstowy, więc nie ma tu nic do czytania. Należy go pobrać i otworzyć w odpowiednim programie.
text-key-tree = drzewo
text-key-pretty = sformatowany
text-key-raw = surowy
text-key-document = dokument
text-key-source = źródło
text-key-wrap = zawijanie
text-key-copy = kopiuj
text-key-find = znajdź
text-key-close = zamknij
text-preview-aria = Czytaj { $name } na pełnym ekranie
text-preview-read = Czytaj

## JsonTree.vue i JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
json-keys =
    { $count ->
        [one] { $count } klucz
        [few] { $count } klucze
        [many] { $count } kluczy
       *[other] { $count } klucza
    }
json-fold = Zwiń
json-unfold = Rozwiń
json-unfold-summary = Rozwiń { $summary }
json-show-more = Pokaż jeszcze { $count }
json-left =
    { $count ->
        [one] z { $count } pozostałego
        [few] z { $count } pozostałych
        [many] z { $count } pozostałych
       *[other] z { $count } pozostałego
    }
json-picker-more = … jeszcze { $count }

## Przeglądarki modeli 3D (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = nie załadowano przeglądarki 3D ({ $error }); może pomóc ponowne wczytanie strony
model-loading-viewer = Ładowanie przeglądarki 3D…
model-loading = Ładowanie modelu…
model-show-failed = Nie udało się wyświetlić tego modelu: { $error }
model-preview-failed = Nie udało się pokazać podglądu tego modelu: { $error }
model-error-converting = serwer nadal konwertuje ten model
model-error-converting-desktop = Hoard nadal konwertuje ten model
model-error-empty-collada = pusty plik Collada
model-error-no-loader = Przeglądarka nie ma modułu wczytującego dla .{ $ext }
model-dialog = Wyświetl { $name } w 3D
model-title-stats =
    { $triangles } trójk. · { $meshes ->
        [one] { $meshes } siatka
        [few] { $meshes } siatki
        [many] { $meshes } siatek
       *[other] { $meshes } siatki
    } · { $materials ->
        [one] { $materials } materiał
        [few] { $materials } materiały
        [many] { $materials } materiałów
       *[other] { $materials } materiału
    }
model-triangles =
    { $count ->
        [one] { $count } trójkąt
        [few] { $count } trójkąty
        [many] { $count } trójkątów
       *[other] { $count } trójkąta
    }
model-group-camera = Kamera
model-group-shading = Cieniowanie
model-group-background = Tło
model-group-animation = Animacja
model-group-view = Widok
model-group-show = Pokaż
model-frame-title = Wykadruj model (F)
model-frame = Kadruj
model-reset-title = Powrót do widoku początkowego (R)
model-reset = Resetuj
model-turn-title = Obracaj powoli (A)
model-turn = Obrót
model-textures-title = Tekstury (T)
model-textures = Tekstury
model-full-res-title = Tekstury w pełnej rozdzielczości (H): każdy plik tekstury taki, jaki jest, niezależnie od rozmiaru; po wyłączeniu lekkie podglądy maksymalnie 2048 px
model-full-res = Pełna rozdz.
model-light-title = Oświetlenie otoczenia (E)
model-light = Światło
model-grid-title = Siatka podłogi (G)
model-grid = Siatka
model-bg-checker = Tło w szachownicę
model-bg-dark = Ciemne tło
model-bg-gray = Szare tło
model-bg-light = Jasne tło
model-bg-env = Tło otoczenia
model-bg-title = { $background } (K przełącza)
model-pause-title = Wstrzymaj (Spacja)
model-play-title = Odtwórz (Spacja)
model-pause = Wstrzymaj
model-play = Odtwórz
model-animation-clip = Klip animacji
model-animation = Animacja
model-plays-on = Odtwarzaj na
model-close-title = Zamknij (Esc)
model-full-screen = Pełny ekran
model-open-full-screen = Otwórz przeglądarkę 3D na pełnym ekranie
model-menu-fold = Zwiń menu
model-menu-show = Pokaż menu
model-view-prefix = Widok:{ " " }
model-menu-render = Render
model-menu-maps = Mapy
model-menu-inspect = Analiza
model-mode-final = Render końcowy
model-mode-raw = Render surowy
model-mode-color = Kolor bazowy
model-mode-roughness = Chropowatość
model-mode-metalness = Metaliczność
model-mode-normal = Normalne
model-mode-ao = Okluzja
model-mode-matcap = Matcap
model-mode-uv = Siatka kontrolna UV
model-mode-geometry = Geometrie
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Siatka krawędzi (W), na dowolnym widoku
model-wireframe = Siatka krawędzi
model-info-title = Informacje o scenie (I)
model-info = Informacje o scenie
model-info-close = Zamknij informacje o scenie
model-info-close-title = Zamknij (I)
model-info-geometries = Geometrie
model-info-materials = Materiały
model-info-textures = Tekstury
model-info-triangles = Trójkąty
model-info-vertices = Wierzchołki
model-info-meters = glTF jest w metrach
model-info-no-units = Ten format nie określa swoich jednostek
model-info-height = Wysokość
model-info-length = Długość
model-info-width = Szerokość
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } jednostka
        [few] { NUMBER($n, maximumSignificantDigits: 3) } jednostki
        [many] { NUMBER($n, maximumSignificantDigits: 3) } jednostek
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } jednostki
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Nie znaleziono tekstury: { $name }
model-missing-some = Nie znaleziono tekstur ({ $count }): { $names }
model-missing-more = Nie znaleziono tekstur ({ $count }): { $names }…
model-missing-count = Nie znaleziono tekstur: { $count }
model-guessed = Tekstury dopasowano po nazwie; plik nie wskazuje żadnych.
model-group-facing = Orientacja
model-facing-turn = Obróć o ćwierć
model-facing-turn-title = Obróć model o ćwierć obrotu przeciwnie do ruchu wskazówek zegara, patrząc z góry
model-facing-tip = Przechyl o ćwierć
model-facing-tip-title = Przechyl górę modelu o ćwierć obrotu do przodu, by postawić leżący model
model-facing-save = Użyj dla paczki
model-facing-save-title = Obróć tak wszystkie modele w paczce { $pack }. Ich miniatury zostaną narysowane ponownie; pliki do pobrania pozostaną takie, jakie zostały utworzone.
model-facing-undo = Cofnij
model-facing-undo-title = Powrót do obecnego ustawienia modeli paczki
model-facing-saved = Wszystkie modele w paczce { $pack } są teraz tak ustawione. Ich miniatury są rysowane ponownie.
model-facing-failed = Nie udało się obrócić modeli paczki: { $error }
model-keys-mouse = Przeciąganie obraca · przeciąganie prawym przyciskiem przesuwa · przewijanie powiększa · dwuklik kadruje
model-key-frame = kadruj
model-key-reset = resetuj
model-key-view = widok
model-key-wireframe = siatka krawędzi
model-key-info = informacje o scenie
model-key-textures = tekstury
model-key-full-res = tekstury w pełnej rozdzielczości
model-key-light = światło
model-key-grid = siatka
model-key-background = tło
model-key-play = odtwarzanie

## Materiały (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Kolor
material-role-normal = Normalne
material-role-normal-dx = Normalne (DirectX)
material-role-roughness = Chropowatość
material-role-gloss = Połysk
material-role-metalness = Metaliczność
material-role-ao = Okluzja otoczenia
material-role-orm = Okluzja, chropowatość, metaliczność
material-role-mra = Metaliczność, chropowatość, okluzja
material-role-rma = Chropowatość, metaliczność, okluzja
material-role-height = Wysokość
material-role-bump = Wypukłość
material-role-cavity = Wgłębienia
material-role-specular = Odbicie (specular)
material-role-opacity = Krycie
material-role-emissive = Emisja
material-role-translucency = Półprzezroczystość
material-role-base-color-name = kolor
material-role-normal-name = normalne
material-role-normal-dx-name = normalne (DirectX)
material-role-roughness-name = chropowatość
material-role-gloss-name = połysk
material-role-metalness-name = metaliczność
material-role-ao-name = okluzja otoczenia
material-role-orm-name = okluzja, chropowatość, metaliczność
material-role-mra-name = metaliczność, chropowatość, okluzja
material-role-rma-name = chropowatość, metaliczność, okluzja
material-role-height-name = wysokość
material-role-bump-name = wypukłość
material-role-cavity-name = wgłębienia
material-role-specular-name = odbicie (specular)
material-role-opacity-name = krycie
material-role-emissive-name = emisja
material-role-translucency-name = półprzezroczystość
material-maps =
    { $count ->
        [one] { $count } mapa
        [few] { $count } mapy
        [many] { $count } map
       *[other] { $count } mapy
    }
material-dialog = Wyświetl materiał { $name }
material-up-to = do { $size }
material-repeat-size = { $size } na powtórzenie
material-shape-sphere = Kula
material-shape-cube = Sześcian
material-shape-plane = Płaszczyzna
material-shape-title = { $shape } ({ $key })
material-group-shape = Kształt
material-group-tiling = Kafelkowanie
material-real-title = Rzeczywisty rozmiar: jedno powtórzenie to { $size }, kula ma 2 m średnicy (T przełącza)
material-repeat-title = Powtórz { $n }× (T przełącza)
material-real = Rzecz.
material-repeat = { $n }×
material-height-title = Jak mocno mapa wysokości wypycha powierzchnię
material-height = Wysokość
material-height-strength = Siła wysokości
material-loading = Ładowanie materiału…
material-show-failed = Nie udało się wyświetlić tego materiału: { $error }
material-preview-failed = Nie udało się pokazać podglądu tego materiału: { $error }
material-group-maps = Mapy
material-map-show = Pokaż mapę „{ $map }”
material-map-hide = Ukryj mapę „{ $map }”
material-map-unused = Nieużywana przez podgląd
material-open-full-screen = Otwórz przeglądarkę materiałów na pełnym ekranie
material-keys-mouse = Przeciąganie obraca · przeciąganie prawym przyciskiem przesuwa · przewijanie powiększa · kliknięcie mapy ją wyłącza
material-key-shape = kształt
material-key-tiling = kafelkowanie
material-key-turn = obrót

## theme.ts

theme-slate = Łupek
theme-paper = Papier
theme-linen = Len
theme-sage = Szałwia
theme-lilac = Bez
theme-blush = Róż
theme-graphite = Grafit
theme-obsidian = Obsydian
theme-ember = Żar
theme-moss = Mech
theme-aubergine = Bakłażan
theme-oxblood = Bordo
theme-light-slate-blurb = Chłodny niebieskoszary
theme-light-paper-blurb = Zwykły neutralny szary
theme-light-linen-blurb = Ciepły kremowy, bursztyn
theme-light-sage-blurb = Delikatna zieleń
theme-light-lilac-blurb = Blady fiolet
theme-light-blush-blurb = Przygaszony róż
theme-dark-graphite-blurb = Neutralny szary, bez odcienia
theme-dark-slate-blurb = Niebieskoszary
theme-dark-obsidian-blurb = Prawie czarny, do OLED
theme-dark-ember-blurb = Ciepły grafit, bursztyn
theme-dark-moss-blurb = Leśna zieleń z czernią
theme-dark-aubergine-blurb = Śliwkowa czerń, liliowy
theme-dark-oxblood-blurb = Winna czerń, koral

## texnames.ts

texnames-needs-map = Wzorzec musi zawierać { "{" }map{ "}" }, inaczej każda mapa materiału dostanie tę samą nazwę
texnames-placeholders = Wzorzec może zawierać { "{" }name{ "}" }, { "{" }map{ "}" } i { "{" }size{ "}" }
texnames-bad-characters = Nazwy nie mogą zawierać znaków < > : " / \ | ? ani *
texnames-no-suffix = Mapy typu „{ $role }” muszą mieć sufiks
texnames-bad-suffix = Sufiks map typu „{ $role }” nie może zawierać znaków < > : " / \ | ? ani *
texnames-slot-no-suffix = Slot „{ $slot }” musi mieć sufiks
texnames-slot-bad-suffix = Sufiks slotu „{ $slot }” nie może zawierać znaków < > : " / \ | ? ani *
texnames-ending-missing = Każda własna końcówka musi mieć końcówkę do wyszukania
texnames-ending-bad = Własne końcówki nie mogą zawierać znaków < > : " / \ | ? ani *
texnames-ending-twice = { $ending } występuje dwa razy wśród własnych końcówek
texnames-slot-metallic-roughness = Metaliczność i chropowatość
texnames-slot-specular-color = Kolor odbicia
texnames-slot-specular-glossiness = Odbicie i połysk
texnames-slot-clearcoat = Lakier
texnames-slot-clearcoat-roughness = Chropowatość lakieru
texnames-slot-clearcoat-normal = Normalne lakieru
texnames-slot-sheen-color = Kolor połysku tkaniny
texnames-slot-sheen-roughness = Chropowatość połysku tkaniny
texnames-slot-transmission = Transmisja
texnames-slot-thickness = Grubość
texnames-slot-diffuse-transmission = Transmisja rozproszona
texnames-slot-diffuse-transmission-color = Kolor transmisji rozproszonej
texnames-slot-iridescence = Iryzacja
texnames-slot-iridescence-thickness = Grubość iryzacji
texnames-slot-anisotropy = Anizotropia

## storeFormats.ts

storefiles-field-title = Tytuł
storefiles-field-title-hint = Dokładny tytuł oferty. Nadaje nazwę paczce, a bez niego plik jest pomijany.
storefiles-field-seller = Sprzedawca
storefiles-field-seller-hint = Kto to stworzył lub sprzedaje.
storefiles-field-license = Licencja
storefiles-field-license-hint = Tak, jak zapisano w pliku, np. CC-BY-4.0.
storefiles-field-url = Strona w sklepie
storefiles-field-url-hint = Adres oferty, zaczynający się od http:// lub https://.
storefiles-field-words = Tagi
storefiles-field-words-hint = Słowa, którymi zostanie otagowana. Wartości rozdzielone przecinkami są dzielone.
storefiles-field-type = Rodzaj
storefiles-field-type-hint = Co to jest, np. „model 3D” lub „powierzchnia”. Także tag.
storefiles-field-colors = Kolory
storefiles-field-colors-hint = Jej kolory, jako tagi color:.
storefiles-field-id = Identyfikator w sklepie
storefiles-field-id-hint = Identyfikator nadany przez sklep: litery, cyfry, _ lub -.
storefiles-field-origin = Miejsce powstania
storefiles-field-origin-hint = Gdzie powstała lub została zeskanowana, do opisu.
storefiles-field-size = Rozmiar
storefiles-field-size-hint = Jak duża jest, słowami, do opisu.
storefiles-field-title-name = tytuł
storefiles-field-seller-name = sprzedawca
storefiles-field-license-name = licencja
storefiles-field-url-name = strona w sklepie
storefiles-field-words-name = tagi
storefiles-field-type-name = rodzaj
storefiles-field-colors-name = kolory
storefiles-field-id-name = identyfikator w sklepie
storefiles-field-origin-name = miejsce powstania
storefiles-field-size-name = rozmiar

## StoreFormatsPanel.vue

storefiles-load-failed = Nie udało się załadować formatów plików sklepów: { $error }
storefiles-remove-title = Usunąć { $name }?
storefiles-remove-body = Plik { $file } zostanie usunięty z folderu danych. Paczki już nim odczytane zachowają to, co odczytał.
storefiles-remove = Usuń
storefiles-removed = Usunięto { $name }.
storefiles-remove-failed = Nie udało się usunąć { $name }: { $error }
storefiles-unread = Pliki, których Hoard nie odczytuje
storefiles-loading = Ładowanie…
storefiles-unread-none = Brak: żaden plik w bibliotece nie wygląda na nieodczytany plik sklepu.
storefiles-unread-what = Pliki występujące w więcej niż jednej paczce lub nazwane jak pliki sklepu, których nie odczytuje żaden format.
storefiles-unread-packs =
    { $count ->
        [one] { $count } paczka
        [few] { $count } paczki
        [many] { $count } paczek
       *[other] { $count } paczki
    }
storefiles-unread-example = np. { $pack }
storefiles-teach = Naucz Hoard tego pliku
storefiles-formats = Formaty
storefiles-formats-what = Własne formaty są sprawdzane jako pierwsze i zastępują wbudowane o tej samej nazwie.
storefiles-dir-before = Są przechowywane w
storefiles-dir-after = .
storefiles-yours = Własny
storefiles-built-in = Wbudowany
storefiles-edit = Edytuj
storefiles-start-from = Zacznij od tego

## TagsPanel.vue

tags-load-failed = Nie udało się załadować tagów: { $error }
tags-rename-title = Zmienić nazwę { $tag }?
tags-merge-title =
    { $count ->
        [one] Scalić { $count } tag?
        [few] Scalić { $count } tagi?
        [many] Scalić { $count } tagów?
       *[other] Scalić { $count } tagu?
    }
tags-rename-body = Jego zasoby ({ $count }) dostaną zamiast niego { $into }, a tagowanie będzie odtąd używać nowej nazwy.
tags-merge-body = { $tags } zmienią się w { $into } (zasobów: maksymalnie { $count }). Tagowanie będzie odtąd używać { $into }.
tags-rename = Zmień nazwę
tags-merge = Scal
tags-merged =
    { $count ->
        [one] { $count } zasób ma teraz tag { $into }
        [few] { $count } zasoby mają teraz tag { $into }
        [many] { $count } zasobów ma teraz tag { $into }
       *[other] { $count } zasobu ma teraz tag { $into }
    }
tags-merge-failed = Nie udało się scalić: { $error }
tags-forget-failed = Nie udało się tego zmienić: { $error }
tags-find = Znajdź tagi
tags-find-placeholder = Znajdź tagi, np. rock lub set:
tags-count =
    { $count ->
        [one] { $count } tag
        [few] { $count } tagi
        [many] { $count } tagów
       *[other] { $count } tagu
    }
tags-count-more = { $count }+ tagów
tags-what = Można wybrać tagi o tym samym znaczeniu i je scalić albo wybrać jeden, by zmienić jego nazwę.
tags-new-name = Nowa nazwa
tags-merge-into = Scal w
tags-saved-as-before = Zapisano jako
tags-saved-as-after = { "" }
tags-cancel = Anuluj
tags-loading = Ładowanie…
tags-none = Brak tagów.
tags-none-containing = Brak tagów zawierających „{ $text }”.
tags-browse-title = Przeglądaj zasoby z tagiem { $tag }
tags-browse = Przeglądaj
tags-merged-heading = Scalone tagi
tags-merged-what = Gdy tagowanie wymyśli jedną z tych starych nazw, zasób dostanie tag, w który ją scalono.
tags-allow-old = Zezwól znów na starą nazwę

## LibraryStats.vue

stats-source-human = Dodane przez ludzi
stats-source-ai = Zasugerowane przez AI
stats-source-rule = Wbudowane reguły
stats-source-meta = Z plików
stats-assets = Zasoby
stats-packs = Paczki
stats-on-disk = Na dysku
stats-duplicates-title =
    { $count ->
        [one] { $count } plik to identyczna bajt w bajt kopia innego, przechowywana raz
        [few] { $count } pliki to identyczne bajt w bajt kopie innych, przechowywane raz
        [many] { $count } plików to identyczne bajt w bajt kopie innych, przechowywane raz
       *[other] { $count } pliku to identyczne bajt w bajt kopie innych, przechowywane raz
    }
stats-saved-dedup = Zaoszczędzone dzięki deduplikacji
stats-distinct-tags = Różne tagi
stats-archive = W archiwum
stats-load-failed = Nie udało się załadować szczegółów: { $error }
stats-counting = Liczenie…
stats-by-kind = Miejsce według rodzaju
stats-files-size =
    { $count ->
        [one] { $name }: { $count } plik, { $size }
        [few] { $name }: { $count } pliki, { $size }
        [many] { $name }: { $count } plików, { $size }
       *[other] { $name }: { $count } pliku, { $size }
    }
stats-months = Dodane w ostatnim roku
stats-month =
    { $count ->
        [one] { $month }: { $count } zasób, { $size }
        [few] { $month }: { $count } zasoby, { $size }
        [many] { $month }: { $count } zasobów, { $size }
       *[other] { $month }: { $count } zasobu, { $size }
    }
stats-tagging = Tagowanie
stats-tagged = Otagowane
stats-untagged = Bez tagów: { $count }
stats-described = Opisane
stats-described-assets =
    { $count ->
        [one] { $count } zasób
        [few] { $count } zasoby
        [many] { $count } zasobów
       *[other] { $count } zasobu
    }
stats-human-edited = Edytowane ręcznie
stats-human-edited-unit =
    { $count ->
        [one] zasób
        [few] zasoby
        [many] zasobów
       *[other] zasobu
    }
stats-source-different = różnych: { $count }
stats-most-used = Najczęściej używane
stats-show-tag = Pokaż zasoby z tagiem { $tag }
stats-file-types = Typy plików
stats-biggest-packs = Największe paczki
stats-browse-pack = Przeglądaj { $name }
stats-largest-files = Największe pliki
stats-open-file = Otwórz { $name }
stats-empty = Biblioteka jest jeszcze pusta. Po dodaniu paczki pojawią się tu jej liczby.

## UpdatesPanel.vue

updates-saved = Hoard wywoła ten adres, gdy aktualizacja będzie gotowa
updates-saved-none = Brak adresu do wywołania
updates-save-failed = Nie udało się zapisać
updates-call-answered = Odpowiedź: { $result }
updates-call-refused = Żądanie nie zostało przyjęte: { $result }
updates-call-failed = Nie udało się wywołać adresu
updates-called = Wywołano { $when } dla wersji { $version }: { $result }
updates-called-failed = Wywołano { $when } dla wersji { $version }: { $result }. Hoard spróbuje ponownie za godzinę.
updates-latest = Hoard { $version } to najnowsza wersja.
updates-out = Dostępna jest wersja Hoard { $version }.
updates-out-idle = Dostępna jest wersja Hoard { $version }; wywołanie nastąpi w ciągu kilku minut, gdy nic nie będzie się działo.
updates-out-busy = Dostępna jest wersja Hoard { $version }; oczekiwanie, aż nic nie będzie się działo ({ $work }).
updates-busy-more = jeszcze { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } zadanie czeka
        [few] { $count } zadania czekają
        [many] { $count } zadań czeka
       *[other] { $count } zadania czeka
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } przesyłanie
        [few] { $count } przesyłania
        [many] { $count } przesyłań
       *[other] { $count } przesyłania
    }
updates-list-separator = ,{ " " }
updates-what = Gdy pojawi się nowa wersja, Hoard może wywołać adres, kiedy nic się nie dzieje (brak przesyłania, zadań, konwersji i konserwacji), dzięki czemu narzędzie aktualizujące jego kontener nie uruchomi go ponownie w trakcie przesyłania. Wysyła żądanie POST z informacją o wersji w treści JSON. Wymaga to powyższego sprawdzania nowych wersji.
updates-url = Adres do wywołania
updates-url-placeholder = Brak (np. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Nagłówek Authorization
updates-auth-saved = Zapisano (wpisz, aby zastąpić)
updates-auth-placeholder = Brak (np. Bearer my-token)
updates-remove = Usuń
updates-save = Zapisz
updates-call-now-title = Uruchamia to, z czym adres jest połączony
updates-call-now = Wywołaj teraz
updates-what-calls = Co można wywołać?
updates-watchtower = API HTTP Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , z{ " " }
updates-watchtower-token = { " " }i tokenem z{ " " }
updates-watchtower-its = { " " }(z włączonym endpointem{ " " }
updates-watchtower-endpoint = ).
updates-portainer = Webhook Portainer
updates-portainer-rest = { " " }na stosie lub kontenerze Hoard, z włączoną opcją ponownego pobierania obrazu. Token jest częścią adresu URL.
updates-script = Skrypt na hoście
updates-script-listener = { " " }za odbiornikiem webhooków (np.{ " " }
updates-script-runs = ), który uruchamia{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }lub cokolwiek innego, co przyjmuje webhooki.
updates-poll-before = Można też obejść się bez adresu: skrypt z tokenem dostępu może odpytywać{ " " }
updates-poll-until = , aż pole{ " " }
updates-poll-and = { " " }zostanie ustawione, a{ " " }
updates-poll-after = { " " }będzie mieć wartość true, a następnie pobrać nową wersję i ponownie uruchomić Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Nie udało się załadować strony Konserwacja
maintenance-reanalyze-title = Przeanalizować ponownie każdą paczkę?
maintenance-reanalyze-body = Każdy plik zostanie odczytany ponownie, co w dużej bibliotece może zająć sporo czasu; paczki będą oznaczone jako „w toku”, dopóki nie przyjdzie ich kolej. Nadane nazwy, tagi i notatki zostaną zachowane.
maintenance-reanalyze-confirm = Analizuj ponownie
maintenance-gc-title = Usunąć nieużywane pliki?
maintenance-gc-body = Przechowywane pliki, na które nic w bibliotece nie wskazuje, zostaną usunięte na zawsze. Paczki w archiwum zachowują swoje.
maintenance-gc-confirm = Usuń je
maintenance-start-failed = Nie udało się uruchomić zadania „{ $task }”
maintenance-just-now = przed chwilą
maintenance-minutes-ago = { $count } min temu
maintenance-hours-ago = { $count } godz. temu
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } godz.
maintenance-loading = Ładowanie…
maintenance-running = W toku…
maintenance-starting = Uruchamianie…
maintenance-run = Uruchom
maintenance-started = { $detail } · rozpoczęto { $when }
maintenance-starting-detail = Uruchamianie
maintenance-finished = Zakończono:{ " " }
maintenance-failed = Niepowodzenie:{ " " }
maintenance-never-run = Jeszcze nie uruchomiono
maintenance-history = Poprzednie uruchomienia
maintenance-col-task = Zadanie
maintenance-col-finished = Zakończono
maintenance-col-took = Czas trwania
maintenance-col-result = Wynik
maintenance-script-before = Kopie zapasowe i kontrolę uszkodzeń można też uruchamiać ze skryptu:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }i{ " " }
maintenance-script-after = . Takie uruchomienia też są tu widoczne.

## SettingsPage.vue

settings-title = Ustawienia
settings-loading = Ładowanie…
settings-load-failed = Nie udało się załadować ustawień: { $error }
settings-page-general = Ogólne
settings-page-general-blurb = Nazwa tego serwera, widoczna dla wszystkich jego użytkowników.
settings-page-general-blurb-desktop = Nazwa tej biblioteki.
settings-page-appearance = Wygląd
settings-page-appearance-blurb = Motywy tej przeglądarki w trybie jasnym i ciemnym. Zapisywane tylko w tej przeglądarce.
settings-page-library = Biblioteka
settings-page-library-blurb = Co jest w zbiorze: jak jest duży, z czego się składa, jak rósł i jak dobrze jest otagowany. Zarchiwizowane elementy nie są liczone.
settings-page-copies = Prawie kopie
settings-page-copies-blurb = Pliki, które są tym samym obrazem, dźwiękiem lub modelem w innym rozmiarze, częstotliwości próbkowania lub formacie, najpierw te, które zajmują najwięcej miejsca. Najlepsza w każdym zestawie jest oznaczona; nic nie jest usuwane.
copies-loading = Szukanie prawie kopii…
copies-none = Brak prawie kopii: każdy obraz, dźwięk i model jest tu tylko raz.
copies-load-failed = Nie udało się wyświetlić prawie kopii: { $error }
copies-sum = Zestawy prawie kopii: { $count }. Kopie, które nie są najlepsze, zajmują { $size }.
copies-set = Zestaw { $n }
copies-set-wasted = { $size } w kopiach, które nie są najlepsze
copies-more = Pokaż więcej (zostało { $count })
settings-page-tags = Tagi
settings-page-tags-blurb = Wszystkie tagi w bibliotece. Można scalić tagi o tym samym znaczeniu lub zmienić nazwę tagu; od tej pory tagowanie używa nowej nazwy.
settings-page-store-files = Pliki sklepów
settings-page-store-files-blurb = Pliki, które sklepy dołączają do grafik, a Hoard odczytuje, by nazywać paczki i wskazywać ich autorów. Można nauczyć go nieznanych plików i udostępnić innym to, czego go nauczono.
settings-page-locations = Lokalizacje biblioteki
settings-page-locations-blurb = Foldery na serwerze, w których znajduje się biblioteka: te, w których Hoard przechowuje pliki, oraz foldery z paczkami indeksowanymi tam, gdzie leżą.
settings-page-locations-blurb-desktop = Foldery, w których znajduje się biblioteka: te, w których Hoard przechowuje pliki, oraz foldery z paczkami indeksowanymi tam, gdzie leżą.
settings-page-tools = Narzędzia
settings-page-tools-blurb = Zewnętrzne programy, których Hoard używa do konwertowania i odczytywania plików.
settings-page-ai = Tagowanie AI
settings-page-ai-blurb = Model AI nadaje nowym paczkom nazwy, opisy i tagi, uzupełniając wbudowane reguły. Jego tagi są oznaczane jako sugerowane.
settings-page-search = Wyszukiwanie obrazów i dźwięków
settings-page-search-blurb = Wyszukiwanie obrazów według tego, co przedstawiają, i dźwięków według tego, jak brzmią, a nie tylko według nazw, oraz znajdowanie rzeczy podobnych rodzajem, a nie tylko kolorem.
settings-page-adding = Dodawanie paczek
settings-page-adding-blurb = Co Hoard robi z paczką po jej dodaniu, oprócz odczytania i otagowania.
settings-page-downloads = Pobieranie
settings-page-downloads-blurb = Nazwy plików w tym, co wysyła Hoard: pobraniach, plikach zip, narzędziu hoard i asystentach AI. Pliki w bibliotece pozostają bez zmian.
settings-page-account = Konto
settings-page-account-blurb = Hasło, inne osoby, które mogą się logować, oraz tokeny dostępu dla skryptów i innych programów.
settings-page-auth = Uwierzytelnianie
settings-page-auth-blurb = Sposób logowania: własne konta Hoard, katalog LDAP, Microsoft Active Directory lub dostawca OAuth, czas trwania sesji oraz role przypisane grupom.
settings-page-connection = Połączenie
settings-page-connection-blurb = Jak przeglądarki i aplikacje łączą się z Hoard: adres, pod którym jest dostępny, oraz HTTPS, wymagany przez niektóre funkcje przeglądarek.
settings-page-performance = Wydajność
settings-page-performance-blurb = Jak intensywnie pracuje serwer, jak powstają i są przechowywane miniatury oraz ile maksymalnie przyjmuje.
settings-page-performance-blurb-desktop = Jak intensywnie pracuje Hoard, jak powstają i są przechowywane miniatury oraz ile maksymalnie przyjmuje.
settings-page-issues = Problemy
settings-page-issues-blurb = Co poszło nie tak przy otwieraniu lub przetwarzaniu plików: archiwa, których nie dało się rozpakować, pliki, których nie dało się odczytać, nieudane zadania. Po usunięciu przyczyny można spróbować ponownie albo je odrzucić.
settings-page-maintenance = Konserwacja
settings-page-maintenance-blurb = Kopie zapasowe, kontrola uszkodzeń i porządki. Każde z tych zadań działa naraz tylko raz, a w trakcie można opuścić tę stronę.
settings-the-default = wartość domyślna
settings-default-on = włączone
settings-default-off = wyłączone
settings-default-auto = automatycznie
settings-option-saved = { $name }: { $value }
settings-back-to-default = Przywróć domyślne
settings-save = Zapisz
settings-use = Użyj
settings-status-off = Wyłączone

## SettingsPage.vue: Appearance

settings-themes = Motywy
settings-look-light = Jasny
settings-look-dark = Ciemny
settings-look-group =
    { $scheme ->
        [light] Motyw jasny
       *[dark] Motyw ciemny
    }
settings-preview =
    Wyświetlany jest motyw { $preview ->
        [light] jasny
       *[dark] ciemny
    }, aby można go było obejrzeć. Używany jest motyw { $used ->
        [light] jasny
       *[dark] ciemny
    }{ $system ->
        [yes] { " " }(zgodnie z systemem)
       *[other] { "" }
    } i wróci on po opuszczeniu tej strony.
settings-preview-back =
    Wróć do motywu { $used ->
        [light] jasnego
       *[dark] ciemnego
    }
settings-look-hint = Tryb jasny lub ciemny (albo zgodny z systemem) można przełączać na górnym pasku.

## SettingsPage.vue: Tools

settings-blender-what = Konwertuje w tle modele .blend, .dae, .abc i .x3d do glTF, by miały miniatury i podgląd 3D. Paczki dodane, gdy był wyłączony, zostaną przekonwertowane, gdy będzie gotowy.
settings-blender-download = Używa zainstalowanego Blendera lub pobiera Blender 4.5 LTS (ok. 400 MB) z serwera lustrzanego blender.org.
settings-ffmpeg-what = Koduje pobrania w formatach MP3, OGG i Opus oraz odczytuje dźwięk, którego Hoard nie potrafi sam zdekodować (AAC, M4A, WMA, …).
settings-ffmpeg-download = Używa systemowego ffmpeg z odpowiednimi koderami lub pobiera statyczną kompilację (ok. 80 MB).
settings-tool-reset = { $tool }: przywrócono wartość domyślną
settings-tool-no-download = Używa zainstalowanego programu { $tool }. Dla tego komputera nie ma kompilacji do pobrania, więc należy go zainstalować lub podać jego ścieżkę.
settings-tool-path = Ścieżka programu { $tool }
settings-mode-off = Wyłączone
settings-mode-auto = Automatycznie
settings-mode-path = Ten program
settings-tool-looking = Wyszukiwanie…
settings-tool-downloading = Pobieranie… postęp widać we wskaźniku aktywności
settings-tool-ready = Gotowe: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Instrumenty, którymi odtwarzane i konwertowane są pliki MIDI.
settings-soundfont-file = Plik SoundFont
settings-soundfont-placeholder = Znaleziony na serwerze lub wbudowany
settings-soundfont-placeholder-desktop = Znaleziony na tym komputerze lub wbudowany
settings-soundfont-default = znaleziony na serwerze lub wbudowany
settings-soundfont-default-desktop = znaleziony na tym komputerze lub wbudowany

## SettingsPage.vue: Image search

settings-clip-heading = Wyszukiwanie według treści obrazów
settings-clip-reset = Wyszukiwanie obrazów: przywrócono wartość domyślną
settings-clip-saved-on = Wyszukiwanie obrazów: włączone
settings-clip-saved-off = Wyszukiwanie obrazów: wyłączone
settings-clip-what = Wystarczy wpisać, czego się szuka („omszała skała”, „drzwi sci-fi”, „drewniana beczka”), a obok nazw zawierających te słowa pojawią się obrazy, które to przedstawiają. „Więcej podobnych” znajduje rzeczy tego samego rodzaju, a nie tylko w tych samych kolorach.
settings-clip-toggle = Włącz wyszukiwanie obrazów
settings-clip-hint = Używa modelu SigLIP 2 od Google, uruchamianego na procesorze tego serwera: nic go nie opuszcza. Włączenie pobiera model ({ $size } MB) z Hugging Face, a następnie w tle, z niskim priorytetem, poznaje każdy obraz. Podczas pracy lub wyszukiwania zajmuje od 350 do 750 MB pamięci. Wyłączenie usuwa model, ale zachowuje to, czego się nauczył, na następny raz.
settings-clip-hint-desktop = Używa modelu SigLIP 2 od Google, uruchamianego na procesorze tego komputera: nic go nie opuszcza. Włączenie pobiera model ({ $size } MB) z Hugging Face, a następnie w tle, z niskim priorytetem, poznaje każdy obraz. Podczas pracy lub wyszukiwania zajmuje od 350 do 750 MB pamięci. Wyłączenie usuwa model, ale zachowuje to, czego się nauczył, na następny raz.
settings-clip-looking = Wyszukiwanie modelu…
settings-clip-downloading = Pobieranie modelu… postęp widać we wskaźniku aktywności
settings-clip-ready-empty = Gotowe. Obrazy są poznawane w miarę dodawania paczek.
settings-clip-ready-all =
    { $count ->
        [one] Gotowe: zna { $count } obraz
        [few] Gotowe: zna wszystkie { $count } obrazy
        [many] Gotowe: zna wszystkie { $count } obrazów
       *[other] Gotowe: zna wszystkie { $count } obrazu
    }
settings-clip-learning = Gotowe: poznawanie obrazów, na razie { $done } z { $total }
settings-clip-relearning = Dopóki nowy model nie nauczy się wszystkich obrazów, wyszukiwanie korzysta z tego, czego nauczył się stary.
settings-clip-unavailable = Nie udało się pobrać modelu: { $detail }. Ponowna próba co godzinę.
settings-clap-heading = Wyszukiwanie według brzmienia dźwięków
settings-clap-reset = Wyszukiwanie dźwięków: przywrócono wartość domyślną
settings-clap-saved-on = Wyszukiwanie dźwięków: włączone
settings-clap-saved-off = Wyszukiwanie dźwięków: wyłączone
settings-clap-what = Wystarczy wpisać, czego się szuka („odległy grzmot”, „szklane piknięcie interfejsu”, „kroki na żwirze”), a obok nazw zawierających te słowa pojawią się dźwięki, które tak brzmią.
settings-clap-toggle = Włącz wyszukiwanie dźwięków
settings-clap-hint = Używa modelu CLAP od LAION, uruchamianego na procesorze tego serwera: nic go nie opuszcza. Włączenie pobiera model ({ $size } MB) z Hugging Face, a następnie w tle, z niskim priorytetem, odsłuchuje początek każdego dźwięku. Podczas pracy lub wyszukiwania zajmuje do około 1 GB pamięci. Wyłączenie usuwa model, ale zachowuje to, czego się nauczył, na następny raz.
settings-clap-hint-desktop = Używa modelu CLAP od LAION, uruchamianego na procesorze tego komputera: nic go nie opuszcza. Włączenie pobiera model ({ $size } MB) z Hugging Face, a następnie w tle, z niskim priorytetem, odsłuchuje początek każdego dźwięku. Podczas pracy lub wyszukiwania zajmuje do około 1 GB pamięci. Wyłączenie usuwa model, ale zachowuje to, czego się nauczył, na następny raz.
settings-clap-ready-empty = Gotowe. Dźwięki są poznawane w miarę dodawania paczek.
settings-clap-ready-all =
    { $count ->
        [one] Gotowe: zna { $count } dźwięk
        [few] Gotowe: zna wszystkie { $count } dźwięki
        [many] Gotowe: zna wszystkie { $count } dźwięków
       *[other] Gotowe: zna wszystkie { $count } dźwięku
    }
settings-clap-learning = Gotowe: poznawanie dźwięków, na razie { $done } z { $total }

## SettingsPage.vue: AI tagging

settings-ai-service = Usługa
settings-ai-off = Wyłączone
settings-ai-off-hint = Paczki są tagowane wyłącznie wbudowanymi regułami.
settings-ai-local = Model lokalny
settings-ai-local-hint = Usługa llm w pliku compose Hoard (lub cargo xtask llm). Model można pobrać poniżej; działa na tym komputerze i nic go nie opuszcza.
settings-ai-chatgpt = Plan ChatGPT
settings-ai-chatgpt-hint = Używa planu ChatGPT Go, Plus, Pro lub Business zamiast klucza API.
settings-ai-openrouter-hint = Jeden klucz do setek hostowanych modeli, w tym niektórych bezpłatnych. Modele przyjmujące obrazy są oznaczone.
settings-ai-openai-hint = Hostowane przez OpenAI.
settings-ai-anthropic-hint = Hostowane przez Anthropic, przez endpoint zgodny z OpenAI.
settings-ai-gemini-hint = Hostowane przez Google, przez endpoint zgodny z OpenAI.
settings-ai-ollama-hint = Własny serwer Ollama. Adres należy zmienić, jeśli nie jest to kontener o nazwie ollama.
settings-ai-custom = Inna (zgodna z OpenAI)
settings-ai-custom-hint = Wszystko, co obsługuje API czatu OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Łączenie…
settings-ai-connected =
    { $count ->
        [one] Połączono · { $count } model
        [few] Połączono · { $count } modele
        [many] Połączono · { $count } modeli
       *[other] Połączono · { $count } modelu
    }
settings-ai-connected-none = Połączono, ale usługa nie podaje żadnych modeli; należy wpisać nazwę modelu
settings-chatgpt-connected-none = Połączono; należy wpisać nazwę modelu
settings-ai-takes-images = przyjmuje obrazy
settings-ai-saved = Tagowanie AI: { $service }
settings-ai-saved-off = Tagowanie AI jest wyłączone
settings-ai-reset = Tagowanie AI: przywrócono wartość domyślną
settings-ai-on = Włączone: { $service } · { $model }
settings-ai-url = Adres
settings-ai-key = Klucz API
settings-ai-key-saved = Zapisano (wpisz, aby zastąpić)
settings-ai-key-paste = Wklej klucz
settings-ai-key-optional = Brak (opcjonalnie)
settings-ai-key-remove = Usuń
settings-ai-get-key = Uzyskaj klucz
settings-ai-model = Model
settings-ai-model-pick = Wybierz lub wpisz model
settings-ai-model-name = Nazwa modelu
settings-ai-refresh = Odśwież
settings-ai-connect = Połącz
settings-ai-need-key = Należy dodać klucz API, aby zobaczyć modele
settings-ai-vision = Wysyłaj też miniatury (wymaga modelu przyjmującego obrazy)
settings-ai-batch = Na zapytanie
settings-ai-batch-unit = zasobów (mniej dla małych modeli lokalnych, więcej dla dużych hostowanych)

## SettingsPage.vue: ChatGPT sign-in

settings-chatgpt-account = Konto
settings-chatgpt-checking = Sprawdzanie…
settings-chatgpt-status-as = Zalogowano jako { $email }
settings-chatgpt-status = Zalogowano
settings-chatgpt-signed-in-as = Zalogowano do ChatGPT jako { $email }
settings-chatgpt-signed-in = Zalogowano do ChatGPT
settings-chatgpt-signed-out = Wylogowano z ChatGPT
settings-chatgpt-continue = Kontynuuj z ChatGPT
settings-chatgpt-sign-out = Wyloguj
settings-chatgpt-sign-out-title = Wylogować z ChatGPT?
settings-chatgpt-sign-out-in-use = Tagowanie AI korzysta z planu ChatGPT, więc zostanie wstrzymane do ponownego zalogowania lub wyboru innej usługi.
settings-chatgpt-sign-out-body = Hoard zapomni dane logowania i poprosi OpenAI o zakończenie sesji.
settings-chatgpt-manage = Zarządzaj użyciem
settings-chatgpt-finish-before = Należy dokończyć logowanie w oknie, które się otworzyło (
settings-chatgpt-finish-link = otwórz je ponownie
settings-chatgpt-finish-middle = ). Jeśli logowanie skończy się na stronie, która się nie wczytuje, pod adresem zaczynającym się od{ " " }
settings-chatgpt-finish-after = , należy skopiować ten adres i wkleić go tutaj:
settings-chatgpt-paste = Adres, na którym zakończyło się logowanie
settings-chatgpt-finish = Zakończ
settings-chatgpt-usage = Zapytania o tagowanie wliczają się do limitu użycia planu ChatGPT. „Zarządzaj użyciem” pokazuje, ile wykorzystał Hoard, i pozwala to ograniczyć. Aby odłączyć Hoard od samego ChatGPT, należy przejść na chatgpt.com do Ustawienia → Bezpieczeństwo i logowanie → Logowanie przez ChatGPT.

## SettingsPage.vue: Local models

settings-local-downloading-progress = Pobieranie { $percent }% · { $done } z { $total }
settings-local-downloading = Pobieranie…
settings-local-incomplete = Pobieranie przerwane
settings-local-failed = Ostatnio nie udało się uruchomić
settings-local-in-use-running = W użyciu · działa
settings-local-in-use = W użyciu
settings-local-starting = Uruchamianie…
settings-local-running = Działa
settings-local-ready-images = Gotowy · przyjmuje obrazy
settings-local-ready-text = Gotowy · tylko tekst
settings-local-delete-title = Usunąć { $name }?
settings-local-delete-in-use = Tagowanie AI używa tego modelu, więc zostanie wstrzymane do wyboru innego. Pliki modelu zostaną usunięte z serwera modeli.
settings-local-delete-downloading = Pobieranie zostanie przerwane, a pobrane do tej pory dane usunięte.
settings-local-delete-body = Pliki modelu zostaną usunięte z serwera modeli. Można go później pobrać ponownie.
settings-local-delete = Usuń
settings-local-deleting = Usuwanie…
settings-local-cancel = Anuluj
settings-local-resume = Wznów
settings-local-download = Pobierz
settings-local-start-before = Należy go uruchomić poleceniem{ " " }
settings-local-start-or = { " " }(lub{ " " }
settings-local-start-developing = { " " }podczas programowania), a następnie{ " " }
settings-local-start-retry = spróbować ponownie
settings-local-start-after = . Działa też serwer z jednym modelem: wystarczy podać poniżej nazwę tego modelu.
settings-local-list = Na serwerze modeli
settings-local-none = Nic jeszcze nie pobrano. Wystarczy wybrać model poniżej; zostanie zachowany na serwerze, a zmienić go można w każdej chwili.
settings-local-catalog = Modele do pobrania
settings-local-size = { $size } GB · { $license }
settings-local-other = Inny model z Hugging Face
settings-local-other-placeholder = Inny z Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Adding packs

settings-store-pages = Strony w sklepach
settings-store-pages-what = Nowe paczki są w tle wyszukiwane w sklepach, by uzyskać ich stronę w sklepie, sprzedawcę i licencję.
settings-store-lookup = Automatycznie wyszukuj strony w sklepach
settings-unreal = Pakiety Unreal
settings-unreal-reset = Pakiety Unreal: przywrócono wartość domyślną
settings-unreal-what = Co robić z plikami .uasset.
settings-conv = Reguły konwersji
settings-conv-what = Konwertuje pliki przechowywane przez Hoard do jednego formatu, np. każdy OBJ i FBX do GLB, by cała biblioteka była jednolita. Przekonwertowany plik zajmuje miejsce oryginału, z jego nazwą, tagami, kolekcjami i powiązaniami. Pliki w obserwowanych folderach nigdy nie są zmieniane.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Włączona
settings-conv-to = Konwertuj do
# Heading over the file formats a rule converts.
settings-conv-from = Z
settings-conv-remove = Usuń
settings-conv-remove-label = Usuń regułę { $n }
settings-conv-taken = Ten format konwertuje już inna reguła
settings-conv-upright = Ustawiaj modele pionowo podczas konwersji (Y w górę, przodem do +Z), tak jak są wyświetlane modele paczki
settings-conv-add = Dodaj regułę
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Przechowuj oryginały przez
settings-conv-keep-days =
    { $count ->
        [one] dzień
        [few] dni
        [many] dni
       *[other] dnia
    }
settings-conv-keep-what = Do tego czasu na stronie przekonwertowanego pliku można przywrócić oryginał. Tekstury nie są kopiowane do przekonwertowanego modelu: wskazuje on pliki obrazów leżące obok niego. Pliki .mtl i .bin modelu pozostają.
settings-conv-save = Zapisz reguły
settings-conv-revert = Cofnij zmiany
settings-conv-existing = Konwertuj pliki, które są już w bibliotece
settings-conv-saved = Zapisano reguły konwersji. Dotyczą paczek dodanych od teraz.
settings-conv-converting =
    { $count ->
        [one] Konwersja jednego pliku w tle
        [few] Konwersja { $count } plików w tle
        [many] Konwersja { $count } plików w tle
       *[other] Konwersja { $count } pliku w tle
    }
settings-conv-nothing = Nic w bibliotece nie wymaga konwersji.
settings-conv-ask-title =
    { $count ->
        [one] Przekonwertować jeden plik, który jest już w bibliotece?
        [few] Przekonwertować { $count } pliki, które są już w bibliotece?
        [many] Przekonwertować { $count } plików, które są już w bibliotece?
       *[other] Przekonwertować { $count } pliku, które są już w bibliotece?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Wszystko jest w jednej paczce.
        [few] Są w { $packs } paczkach.
        [many] Są w { $packs } paczkach.
       *[other] Są w { $packs } paczki.
    } Każdy oryginał jest przechowywany przez { $days ->
        [one] { $days } dzień
        [few] { $days } dni
        [many] { $days } dni
       *[other] { $days } dnia
    } i do tego czasu można go przywrócić z jego strony.
settings-conv-ask-busy =
    { $count ->
        [one] Jeden plik jest w paczce, która jest jeszcze dodawana; zostanie przekonwertowany, gdy to się skończy.
        [few] { $count } pliki są w paczkach, które są jeszcze dodawane; zostaną przekonwertowane, gdy to się skończy.
        [many] { $count } plików jest w paczkach, które są jeszcze dodawane; zostaną przekonwertowane, gdy to się skończy.
       *[other] { $count } pliku jest w paczkach, które są jeszcze dodawane; zostanie przekonwertowane, gdy to się skończy.
    }
settings-conv-ask-list = Pliki, które zostałyby przekonwertowane
settings-conv-ask-more = …i jeszcze { $count }
settings-conv-ask-skipped = Pozostawione bez zmian:
settings-conv-skip-watched =
    { $count ->
        [one] jeden plik w obserwowanym folderze
        [few] { $count } pliki w obserwowanych folderach
        [many] { $count } plików w obserwowanych folderach
       *[other] { $count } pliku w obserwowanych folderach
    }
settings-conv-skip-unity =
    { $count ->
        [one] jeden plik, który projekt Unity zna po nazwie
        [few] { $count } pliki, które projekt Unity zna po nazwie
        [many] { $count } plików, które projekt Unity zna po nazwie
       *[other] { $count } pliku, które projekt Unity zna po nazwie
    }
settings-conv-skip-used =
    { $count ->
        [one] jeden plik, do którego odwołuje się scena lub inny plik
        [few] { $count } pliki, do których odwołuje się scena lub inny plik
        [many] { $count } plików, do których odwołuje się scena lub inny plik
       *[other] { $count } pliku, do których odwołuje się scena lub inny plik
    }
settings-conv-skip-beside =
    { $count ->
        [one] jeden plik, którego przekonwertowana kopia jest już w jego paczce
        [few] { $count } pliki, których przekonwertowane kopie są już w ich paczkach
        [many] { $count } plików, których przekonwertowane kopie są już w ich paczkach
       *[other] { $count } pliku, których przekonwertowane kopie są już w ich paczkach
    }
settings-conv-skip-blender =
    { $count ->
        [one] jeden plik, którego Blender jeszcze nie odczytał (Ustawienia → Narzędzia)
        [few] { $count } pliki, których Blender jeszcze nie odczytał (Ustawienia → Narzędzia)
        [many] { $count } plików, których Blender jeszcze nie odczytał (Ustawienia → Narzędzia)
       *[other] { $count } pliku, których Blender jeszcze nie odczytał (Ustawienia → Narzędzia)
    }
settings-conv-ask-new = Zapisz, tylko nowe paczki
settings-conv-ask-save-convert =
    { $count ->
        [one] Zapisz i przekonwertuj go
        [few] Zapisz i przekonwertuj { $count } pliki
        [many] Zapisz i przekonwertuj { $count } plików
       *[other] Zapisz i przekonwertuj { $count } pliku
    }
settings-conv-ask-convert =
    { $count ->
        [one] Przekonwertuj go
        [few] Przekonwertuj { $count } pliki
        [many] Przekonwertuj { $count } plików
       *[other] Przekonwertuj { $count } pliku
    }
settings-unreal-off = Pozostaw bez zmian
settings-unreal-off-hint = Konwersja ze strony paczki, gdy będzie potrzebna.
settings-unreal-keep = Konwertuj, zachowaj oryginały
settings-unreal-keep-hint = Tekstury, dźwięki i materiały trafiają do popularnych formatów; plik .uasset również zostaje.
settings-unreal-replace = Konwertuj, usuń oryginały
settings-unreal-replace-hint = Oszczędza miejsce po wyodrębnieniu zawartości. Siatki nadal czekają na proces Unreal worker.

## SettingsPage.vue: Downloads

settings-tn = Nazwy tekstur
settings-tn-saved = Zapisano nazwy tekstur
settings-tn-reset = Nazwy tekstur: przywrócono wartość domyślną
settings-tn-what = Mapy materiału są wysyłane pod nazwą materiału i przeznaczenia każdej mapy, niezależnie od nazw plików: Rock_Normal.png, a nie rock_moss_xeokah0_raw_8K_Normal.png. Pliki w bibliotece pozostają nietknięte, a mapa, której nazwę podaje dołączony model lub plik MTL, zachowuje tę nazwę, by model nadal ją znajdował.
settings-tn-toggle = Nazywaj mapy tekstur według materiału
settings-tn-style = Styl
settings-tn-style-group = Styl nazewnictwa
settings-tn-descriptive = Opisowy
settings-tn-short = Krótki
settings-tn-custom = Własny
settings-tn-pattern = Wzorzec
settings-tn-hint-name = { " " }to materiał,{ " " }
settings-tn-hint-map = { " " }to przeznaczenie mapy,{ " " }
settings-tn-hint-size = { " " }to jej rozmiar, gdy zestaw występuje w kilku rozmiarach (dopisywany na końcu, jeśli wzorzec go pomija).
settings-tn-example = Przykład
settings-tn-suffixes = Sufiksy dla każdego rodzaju mapy
settings-tn-slots = W pobieranych plikach GLB tekstura spakowana w modelu bez własnego pliku dostaje nazwę od materiału i slotu, który wypełnia, a te sloty też mają sufiksy:
settings-tn-endings = Własne końcówki
settings-tn-endings-what = Tekstura, która nie jest jedną z map materiału, zachowuje własną nazwę, chyba że kończy się jedną z tych końcówek: _diff → _BC wysyła rock_diff.png jako rock_BC.png. Zostaw drugie pole puste, aby usunąć końcówkę.
settings-tn-ending-from = Końcówka
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Zamień na
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Dodaj końcówkę
settings-tn-ending-remove = Usuń
settings-tn-ending-remove-label = Usuń { $ending }

## SettingsPage.vue: General and Connection

settings-name = Nazwa
settings-name-what = Nazwa serwera widoczna na górnym pasku, na karcie przeglądarki i na stronie logowania, by przy kilku instancjach Hoard można je było odróżnić.
settings-name-what-desktop = Nazwa tej biblioteki widoczna na górnym pasku.
settings-address = Adres
settings-address-what = Adres, pod którym osoby i aplikacje łączą się z Hoard, jeśli jest inny niż ten otwarty w tej przeglądarce: adres odwrotnego proxy, tunelu lub nazwa w tailnecie, np. https://hoard.example.com. Używają go linki udostępniane przez Hoard (Claude i innym aplikacjom, przy logowaniu, w certyfikacie).
settings-public-url = Adres publiczny
settings-public-url-placeholder = Z adresu w przeglądarce (np. https://hoard.example.com)
settings-public-url-default = z adresu w przeglądarce

## ProfilerPanel.vue: the profiler at the top of Settings → Performance

profiler = Profiler
profiler-what = Co Hoard robił przez ostatnie dziesięć minut, odświeżane co dwie sekundy, gdy strona jest otwarta. Dane są trzymane tylko w pamięci i zaczynają się od nowa po ponownym uruchomieniu Hoard.
profiler-starting = Pierwsze odczyty pojawią się za chwilę.
profiler-failed = Nie udało się odczytać profilera: { $error }
profiler-cpu = Użycie procesora
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Cały komputer
profiler-cpu-summary = Hoard używa { $hoard } procesorów, a cały komputer { $machine }.
profiler-memory = Pamięć
profiler-serving = Trwające żądania
profiler-working = W toku
profiler-working-none = Nic nie działało w tle.
profiler-working-more =
    { $count ->
        [one] I jeszcze jedno.
        [few] I jeszcze { $count }.
        [many] I jeszcze { $count }.
       *[other] I jeszcze { $count }.
    }
profiler-requests = Czas obsługi żądań
profiler-requests-what = Każdy słupek to dziesięć sekund żądań, a ich czas jest podzielony według tego, na co poszedł.
profiler-part-db = Baza danych
profiler-part-db-title = Oczekiwanie na bazę danych i wykonywanie jej zapytań
profiler-part-disk = Dysk
profiler-part-disk-title = Odczyt zapisanych plików i kompresja tego, co jest wysyłane
profiler-part-send = Wysyłanie
profiler-part-send-title = Oczekiwanie, aż sieć odbierze to, co wysłano
profiler-part-other = Inne
profiler-part-other-title = Przygotowanie odpowiedzi
profiler-request-count =
    { $count ->
        [one] { $count } żądanie
        [few] { $count } żądania
        [many] { $count } żądań
       *[other] { $count } żądania
    }
profiler-requests-none = Brak żądań w ostatnich dziesięciu minutach.
profiler-slowest = Najwolniejsze: { $route }, { $time }
profiler-routes = Według ścieżki
profiler-route = Ścieżka
profiler-count = Żądania
profiler-average = Średnio
profiler-p95 = Najwolniejsze 5%
profiler-split = Na co poszedł czas
profiler-sent = Wysłano
profiler-portal = własne pliki interfejsu

## SettingsPage.vue: Performance

settings-render = Miniatury 3D
settings-render-toggle = Renderuj miniatury modeli 3D
settings-gpu-on = Na GPU: { $renderer }
settings-gpu-software = Nie znaleziono GPU, więc programowo ({ $renderer })
settings-gpu-none = Nie znaleziono GPU ani renderera programowego, więc modele nie dostają miniatur
settings-thumbs = Miniatury
settings-thumbs-what = Sposób przechowywania nowych miniatur i widoków modeli. Te z przezroczystością oraz pixel art są zawsze w PNG. Aby zmniejszyć już istniejące, należy uruchomić zadanie „Zmniejsz miniatury” na stronie Konserwacja.
settings-previews-size = Podglądy zajmują teraz { $size }.
settings-thumbs-saved = Miniatury: { $quality }
settings-thumbs-small = Mniejsze
settings-thumbs-small-hint = JPEG, jakość 70. Dobre w widoku miniatur; z bliska nieco miękkie.
settings-thumbs-balanced = Zrównoważone
settings-thumbs-balanced-hint = JPEG, jakość 82. W rozmiarze miniatury trudno je odróżnić od oryginału.
settings-thumbs-sharp = Ostrzejsze
settings-thumbs-sharp-hint = JPEG, jakość 92. Około dwa razy większe niż Zrównoważone.
settings-thumbs-lossless = Bezstratne
settings-thumbs-lossless-hint = PNG, dokładnie takie, jak zostały utworzone. Kilka razy większe.
settings-motion-thumbs = Ruchome miniatury animacji
settings-motion-thumbs-what = Każda animacja dostaje też krótki film w pętli, odtwarzany, gdy wskażesz jej kafelek. Każdy zajmuje około 40 KB, a w dużej bibliotece ich tworzenie chwilę trwa. Wyłączenie usuwa je.
settings-limits = Praca i limity
settings-limits-what = Każda wartość zapisuje się po opuszczeniu pola lub naciśnięciu Enter. Rozmiary podaje się z jednostką: 100 GB, 512 MB, 1.5 TB; sama liczba oznacza GB.
settings-workers = Procesy w tle
settings-workers-unit = zadań naraz (przetwarzanie, tagowanie, konwersje)
settings-max-upload = Limit przesyłania
settings-max-upload-unit = na plik
settings-max-expand = Limit paczki
settings-max-expand-unit = po rozpakowaniu
settings-conversion-cache = Przekonwertowane pobrania
settings-conversion-cache-unit-now = przechowywane maksymalnie, teraz { $size }; najdawniej używane są usuwane jako pierwsze
settings-conversion-cache-unit = przechowywane maksymalnie; najdawniej używane są usuwane jako pierwsze

## SettingsPage.vue: Maintenance

settings-updates = Aktualizacje
settings-version = To jest Hoard { $version }.
settings-update-check = Sprawdzaj co kilka godzin w GitHub, czy jest nowa wersja
settings-desktop-updates = Hoard przy każdym uruchomieniu szuka nowej wersji, pobiera ją w tle i instaluje, gdy go zamkniesz.

## Asset kinds (api.ts)

api-kind-texture = Tekstury
api-kind-vector = Grafika wektorowa
api-kind-sheet = Arkusze sprite’ów
api-kind-audio = Dźwięk
api-kind-model = Modele
api-kind-animation = Animacje
api-kind-font = Czcionki
api-kind-shader = Shadery
api-kind-material = Materiały
api-kind-video = Wideo
api-kind-document = Dokumenty
api-kind-data = Dane
api-kind-other = Inne
api-kind-texture-name = tekstury
api-kind-vector-name = grafika wektorowa
api-kind-sheet-name = arkusze sprite’ów
api-kind-audio-name = dźwięk
api-kind-model-name = modele
api-kind-animation-name = animacje
api-kind-font-name = czcionki
api-kind-shader-name = shadery
api-kind-material-name = materiały
api-kind-video-name = wideo
api-kind-document-name = dokumenty
api-kind-data-name = dane
api-kind-other-name = inne

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Cofnij
ui-today = dzisiaj
ui-yesterday = wczoraj
ui-days-ago =
    { $days ->
        [one] { $days } dzień temu
        [few] { $days } dni temu
        [many] { $days } dni temu
       *[other] { $days } dnia temu
    }
ui-notifications = Powiadomienia
ui-dismiss = Odrzuć
ui-cancel = Anuluj
ui-lazy-loading = Ładowanie { $what }…
ui-lazy-failed = Nie udało się załadować { $what }.
ui-lazy-reload = Wczytaj stronę ponownie

## ResetButton.vue

reset-title = Przywróć domyślne: { $value }
reset-label = Przywróć domyślne, { $value }

## SizeInput.vue

size-invalid = Rozmiar, np. 100 GB, 512 MB lub 1.5 TB
size-hint = Rozmiar, np. 100 GB, 512 MB lub 1.5 TB; sama liczba oznacza GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = Nie udało się załadować ustawień uwierzytelniania: { $error }
auth-saved = Zapisano. Zmiana obowiązuje od następnego logowania.
auth-method-internal = Wewnętrzne
auth-method-internal-hint = Nazwy i hasła przechowywane przez Hoard, dodawane w Ustawienia → Konto.
auth-method-ldap-hint = Katalog sprawdza nazwę i hasło wpisane na stronie logowania: OpenLDAP, FreeIPA, LLDAP lub inny serwer LDAP.
auth-method-ad-hint = Konta domeny Windows logują się swoją zwykłą nazwą i hasłem.
auth-method-oauth-hint = Przycisk „Zaloguj przez…”, który przenosi do Google, Microsoft, GitHub, Keycloak, Authentik lub innego dostawcy i z powrotem.
auth-session-unused = Po 30 dniach nieużywania
auth-session-1h = 1 godzinę po zalogowaniu
auth-session-8h = 8 godzin po zalogowaniu
auth-session-day = Dzień po zalogowaniu
auth-session-week = Tydzień po zalogowaniu
auth-session-30d = 30 dni po zalogowaniu
auth-session-hours =
    { $hours ->
        [one] { $hours } godzinę po zalogowaniu
        [few] { $hours } godziny po zalogowaniu
        [many] { $hours } godzin po zalogowaniu
       *[other] { $hours } godziny po zalogowaniu
    }
auth-role-view = Może przeglądać
auth-role-edit = Może zmieniać
auth-role-admin = Administrator
auth-role-word-view = może przeglądać
auth-role-word-edit = może zmieniać
auth-role-word-admin = administrator
auth-role-word-default = { $role } (rola domyślna)
auth-tls-starttls-hint = Zwykły LDAP (port 389), przełączany na TLS przed wysłaniem czegokolwiek.
auth-tls-ldaps-hint = TLS od samego początku (port 636).
auth-tls-none = Brak
auth-tls-none-hint = Hasła przechodzą przez sieć w postaci jawnej. Tylko w zaufanej sieci.
auth-test-failed = Nie udało się przetestować: { $error }
auth-preset-custom = Inny (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel i inne: wystarczy podać wystawcę, a „Wykryj” uzupełni adresy.
auth-preset-google-note = Należy utworzyć klienta OAuth (aplikacja internetowa) w Google Cloud Console → APIs & Services → Credentials. Google nie przesyła grup, więc role należy nadawać w Ustawienia → Konto, a tworzenie kont przy pierwszym logowaniu wyłączyć, chyba że dostęp ma mieć każdy, kto ma konto Google.
auth-preset-microsoft-note = Należy zarejestrować aplikację w Entra ID → App registrations (z przekierowaniem typu Web na poniższy adres). Aby wpuszczać tylko własną organizację, należy zastąpić „organizations” identyfikatorem dzierżawy. Aby role wynikały z grup, należy dodać oświadczenie groups w Token configuration: zawiera ono identyfikatory grup.
auth-preset-github-note = Należy utworzyć aplikację OAuth w GitHub → Settings → Developer settings. GitHub nie przesyła grup, więc role należy nadawać w Ustawienia → Konto, a tworzenie kont przy pierwszym logowaniu wyłączyć, chyba że dostęp ma mieć każdy, kto ma konto GitHub.
auth-preset-gitlab-note = Należy utworzyć aplikację w GitLab → Preferences → Applications (lub w panelu administratora własnej instancji: wtedy należy zmienić gitlab.com w adresach).
auth-discovered = Uzupełniono adresy dostawcy.
auth-copied = Skopiowano
auth-copy-failed = Nie udało się skopiować: należy zaznaczyć tekst i skopiować go ręcznie
auth-sign-in-with = Logowanie przez
auth-own-accounts-still-work = Własne konta Hoard nadal logują się własnymi hasłami, więc niedziałający lub źle skonfigurowany katalog albo dostawca nie odetnie dostępu.
auth-sessions = Sesje
auth-sign-out-after = Wylogowuj użytkowników
auth-ldap-server = Serwer LDAP
auth-server-type = Typ serwera
auth-server-type-custom = Niestandardowy
auth-servers = Serwery
auth-server-n = Serwer { $n }
auth-remove-server-n = Usuń serwer { $n }
auth-remove = Usuń
auth-add-server = Dodaj serwer
auth-servers-hint = Gdy jeden jest nieosiągalny, próbowany jest następny.
auth-security = Bezpieczeństwo
auth-connection-security = Zabezpieczenie połączenia
auth-skip-verify = Nie sprawdzaj certyfikatu serwera (tylko do testów)
auth-ca = Urząd certyfikacji
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (puste: te, którym ufa system)
auth-lookup = Wyszukiwanie osób
auth-anonymously = Anonimowo
auth-reader-account = Konto do odczytu
auth-reader-password = Jego hasło
auth-saved-type-to-replace = Zapisano: wpisz, aby zastąpić
auth-people-under = Osoby znajdują się w
auth-name-attribute = Atrybut nazwy
auth-only-matching = Tylko wpisy pasujące do
auth-groups-under = Grupy znajdują się w
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (puste: tylko memberOf)
auth-their-groups = Ich grupy
auth-group-filter-hint = { "{" }dn{ "}" } to wpis osoby, { "{" }user{ "}" } to nazwa, której użyła przy logowaniu.
auth-test = Test
auth-test-hint = Łączy się z każdym serwerem i loguje na konto do odczytu. Po podaniu nazwy i hasła loguje się też jako ta osoba i pokazuje, co Hoard by z tego wywnioskował. Nic nie jest zapisywane.
auth-test-user = Osoba do zalogowania
auth-test-user-placeholder = Osoba do zalogowania (opcjonalnie)
auth-test-password = Jej hasło
auth-testing = Testowanie…
auth-test-connectivity = Testuj połączenie
auth-test-server-ok = osiągnięty, konto do odczytu zalogowane
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = : zalogowano (
auth-test-signed-in-after = ).
auth-test-role = Ta osoba otrzymałaby rolę: { $role }.
auth-test-no-role = Ta osoba nie należy do żadnej z grup wymienionych w sekcji „Role z grup”, więc zostałaby odrzucona.
auth-test-groups = Grupy: { $groups }
auth-test-no-groups = Nie znaleziono grup.
auth-provider = Dostawca
auth-redirect = Adres przekierowania
auth-copy = Kopiuj
auth-redirect-hint = Ten adres należy podać dostawcy przy tworzeniu klienta dla Hoard. Pochodzi z adresu publicznego w Ustawienia → Połączenie lub z adresu, który jest teraz używany.
auth-issuer = Wystawca (issuer)
auth-discover = Wykryj
auth-client-id = Identyfikator klienta
auth-client-secret = Sekret klienta
auth-authorization-url = Adres autoryzacji
auth-token-url = Adres tokenu
auth-userinfo-url = Adres informacji o użytkowniku
auth-userinfo-placeholder = (puste: token ID określa, kto to jest)
auth-scopes = Zakresy
auth-name-from = Nazwa z
auth-groups-from = Grupy z
auth-groups-from-placeholder = (brak)
auth-button = Przycisk
auth-button-placeholder = konto
auth-accounts = Konta
auth-provision = Twórz konto Hoard przy pierwszym logowaniu danej osoby
auth-provision-on-hint = W przeciwnym razie administrator musi najpierw dodać każdą osobę w Ustawienia → Konto.
auth-provision-off-hint = Administrator najpierw dodaje każdą osobę w Ustawienia → Konto i wybiera sposób jej logowania.
auth-open-to-anyone = Zalogować się może każdy, kto ma konto { $provider }. Należy dodać poniżej grupy przeglądających lub wyłączyć tę opcję i dodawać osoby samodzielnie.
auth-new-accounts = Nowe konta
auth-roles-from-groups = Role z grup
auth-roles-hint = Grupy, których członkowie otrzymują daną rolę, po jednej w wierszu, według nazwy lub DN. Wygrywa najwyższa pasująca rola, sprawdzana przy każdym logowaniu. Jeśli nie podano żadnej, nowe konta otrzymują rolę ustawioną powyżej, a administrator może ją później zmienić. Jeśli podano grupy w polu „Może przeglądać”, zalogować się mogą tylko członkowie wymienionych grup.
auth-roles-hint-oauth = Grupy, których członkowie otrzymują daną rolę, po jednej w wierszu, według nazwy lub DN (lub tego, co zawiera oświadczenie groups). Wygrywa najwyższa pasująca rola, sprawdzana przy każdym logowaniu. Jeśli nie podano żadnej, nowe konta otrzymują rolę ustawioną powyżej, a administrator może ją później zmienić. Jeśli podano grupy w polu „Może przeglądać”, zalogować się mogą tylko członkowie wymienionych grup.
auth-saving = Zapisywanie…
auth-save = Zapisz

## HttpsPanel.vue (Settings → Connection)

https-load-failed = Nie udało się załadować ustawień HTTPS: { $error }
https-off-redirecting = HTTPS jest wyłączone. Przekierowanie na http://
https-now-on = HTTPS jest włączone
https-saved = Zapisano. Nowe połączenia używają nowego certyfikatu.
https-now-off = HTTPS jest wyłączone
https-http3-failed-toast = Nie udało się uruchomić HTTP/3: { $error }
https-copied = Skopiowano
https-copy-failed = Nie udało się skopiować: należy zaznaczyć tekst i skopiować go ręcznie
https-mode-off = Wyłączone
https-mode-off-hint = Tylko zwykłe http://.
https-mode-own = Certyfikat Hoard
https-mode-own-hint = Hoard tworzy certyfikat dla adresów tego serwera. Wystarczy raz zainstalować urząd certyfikacji Hoard na każdym urządzeniu, a jego przeglądarki będą mu ufać.
https-mode-custom = Własny certyfikat
https-mode-custom-hint = Posiadany już certyfikat, np. z tailscale cert lub Let’s Encrypt. Przeglądarki ufają mu bez instalowania czegokolwiek.
https-untrusted-own = Ta przeglądarka nie ufa mu jeszcze dla { $host }: należy zainstalować poniżej urząd certyfikacji Hoard. Pojawi się ostrzeżenie, które można pominąć.
https-untrusted-uncovered = Ta przeglądarka nie ufa mu jeszcze dla { $host }: certyfikat nie obejmuje tego adresu. Pojawi się ostrzeżenie, które można pominąć.
https-untrusted = Ta przeglądarka nie ufa mu jeszcze dla { $host }. Pojawi się ostrzeżenie, które można pominąć.
https-now = Obecnie
https-start-failed = HTTPS jest skonfigurowane, ale nie udało się go uruchomić: { $error }
https-on-port = HTTPS jest włączone, na porcie { $port }, tak jak http://.
https-using-it = Jest teraz używane.
https-open = Otwórz { $url }
https-trusted = Ta przeglądarka mu ufa.
https-cert-for-before = Certyfikat dla{ " " }
https-cert-for-after = , ważny do { $date }.
https-cert-for-after-own = , ważny do { $date } (odnawiany automatycznie).
https-not-covered-before = Nie obejmuje adresu{ " " }
https-not-covered-after = , który jest teraz używany.
https-is-off = HTTPS jest wyłączone: strony są udostępniane przez zwykłe http://.
https-addresses = Adresy
https-addresses-hint = Certyfikat obejmuje adresy znalezione przez Hoard oraz dodane ręcznie (nazwę nadaną przez router, nazwę w Tailscale).
https-remove-name = Usuń { $name }
https-another-address = Inny adres
https-another-address-placeholder = Inny adres, np. nas.local
https-add = Dodaj
https-your-cert = Własny certyfikat
https-saved-cert-before = Zapisano certyfikat dla{ " " }
https-saved-cert-after = { " " }(wystawca: { $issuer }), ważny do { $date }. Aby go zastąpić, należy wkleić nowy.
https-pem-before = Certyfikat (z łańcuchem) i jego klucz prywatny jako tekst PEM: pliki zaczynające się od{ " " }
https-pem-after = .
https-certificate = Certyfikat
https-choose-file = Wybierz plik…
https-private-key = Klucz prywatny
https-checked-before = Dla{ " " }
https-checked-after = , wystawca: { $issuer }, ważny do { $date }.
https-checked-after-days =
    , wystawca: { $issuer }, ważny do { $date } ({ $days ->
        [one] { $days } dzień
        [few] { $days } dni
        [many] { $days } dni
       *[other] { $days } dnia
    }).
https-self-signed = Jest samopodpisany: przeglądarki będą przed nim ostrzegać.
https-key-matches = Klucz pasuje.
https-where-to-get = Skąd go wziąć
https-get-tailscale-before = należy włączyć HTTPS dla tailnetu, a następnie uruchomić{ " " }
https-get-tailscale-after = { " " }na serwerze i przesłać oba pliki. Są ważne 90 dni: przed upływem tego czasu należy ponownie uruchomić polecenie i przesłać je jeszcze raz.
https-get-acme-title = Let’s Encrypt lub inny klient ACME
https-get-acme-before = (certbot, acme.sh, lego) dla posiadanej domeny: należy przesłać{ " " }
https-get-acme-and = { " " }i{ " " }
https-get-acme-after = .
https-get-proxy-title = Odwrotne proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel), które ma certyfikat: tutaj należy zostawić HTTPS wyłączone, a proxy powinno przesyłać do Hoard zwykłe http://.
https-trying = Sprawdzanie…
https-apply = Zastosuj
https-apply-hint = Hoard najpierw wypróbowuje nową konfigurację i zachowuje dotychczasową, jeśli nowa nie działa.
https-trust-title = Zaufanie do Hoard na urządzeniach
https-download-ca = Pobierz hoard-ca.crt
https-trust-hint = Wystarczy raz zainstalować urząd certyfikacji Hoard na każdym urządzeniu, a jego przeglądarki będą ufać temu serwerowi (i każdemu certyfikatowi, który Hoard później dla niego utworzy). Może on poświadczać tylko ten Hoard. Odcisk (SHA-256):
https-ca-windows = Należy otworzyć plik → Zainstaluj certyfikat → Bieżący użytkownik → Umieść wszystkie certyfikaty w następującym magazynie → Przeglądaj → Zaufane główne urzędy certyfikacji → Zakończ, a potem ponownie uruchomić przeglądarkę.
https-ca-macos = Należy otworzyć plik; trafi on do aplikacji Dostęp do pęku kluczy. Tam należy dwukrotnie kliknąć „{ $subject }” → Zaufanie → Podczas używania tego certyfikatu: Zawsze ufaj.
https-ca-ios-title = iPhone i iPad
https-ca-ios = Należy pobrać go w Safari i zezwolić na pobranie. Ustawienia → Pobrano profil → Zainstaluj. Następnie Ustawienia → Ogólne → To urządzenie → Ustawienia zaufania certyfikatów → włączyć „{ $subject }”.
https-ca-android = Ustawienia → Zabezpieczenia → Więcej ustawień zabezpieczeń → Szyfrowanie i dane logowania → Zainstaluj certyfikat → Certyfikat CA, a następnie wybrać plik. (Nazwy nieco różnią się między telefonami; warto wyszukać w Ustawieniach słowo „certyfikat”.)
https-ca-linux-1 = Debian i Ubuntu:{ " " }
https-ca-linux-2 = . Fedora i Arch:{ " " }
https-ca-linux-3 = . Chrome w systemie Linux ma własną listę: Ustawienia → Prywatność i bezpieczeństwo → Bezpieczeństwo → Zarządzaj certyfikatami → Importuj.
https-ca-firefox = Firefox ma własną listę: Ustawienia → Prywatność i bezpieczeństwo → Certyfikaty → Wyświetl certyfikaty → Organy certyfikacji → Importuj, a następnie zaznaczyć „Zaufaj temu CA przy identyfikacji witryn internetowych”.
https-ca-cli-title = Narzędzie wiersza poleceń hoard
https-ca-cli-before = Należy użyć{ " " }
https-ca-cli-middle = { " " }lub podać ścieżkę w zmiennej{ " " }
https-ca-cli-after = .
https-options = Opcje
https-redirect = Przekierowuj przeglądarki na innych urządzeniach z http:// na https://
https-redirect-hint = Przekierowywane są tylko wczytania stron. Narzędzie wiersza poleceń hoard, Unreal worker, skrypty i aplikacje AI nadal działają przez http://, podobnie jak http://localhost na samym serwerze.
https-http3-hint-before = Szybsze przy niestabilnym Wi-Fi i na telefonach. Wymaga HTTPS oraz otwartego portu UDP { $port } obok TCP (w Dockerze:{ " " }
https-http3-hint-after = ).
https-http3-running = Działa.
https-http3-waits = Uruchomi się po włączeniu HTTPS.
https-http3-failed = Nie udało się uruchomić HTTP/3: { $error }
https-lockout-title = W razie utraty dostępu
https-lockout-1 = Adres{ " " }
https-lockout-2 = { " " }na samym serwerze zawsze działa przez zwykłe http://. Można też wyłączyć HTTPS z wiersza poleceń serwera i ponownie uruchomić Hoard:{ " " }
https-lockout-3 = { " " }(w Dockerze:{ " " }
https-lockout-4 = , a następnie{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Może przeglądać
account-role-view-hint = Wyszukiwanie, podgląd i pobieranie
account-role-edit = Może zmieniać
account-role-edit-hint = Także dodawanie, zmienianie, tagowanie, kolekcje i archiwizowanie
account-role-admin = Administrator
account-role-admin-hint = Także Ustawienia, konta, konserwacja i trwałe usuwanie
account-source-internal = Hasło Hoard
account-source-ldap = Hasło { $directory }
account-source-oauth = Logowanie przez { $provider }
account-source-word-directory = katalog
account-load-failed = Nie udało się załadować kont: { $error }
account-failed = { $what }: { $error }
account-password-failed = Nie udało się zmienić hasła
account-password-changed = Zmieniono hasło. Inne urządzenia zostały wylogowane.
account-add-failed = Nie udało się dodać konta { $name }
account-added = { $name } może się teraz logować.
account-role-failed = Nie udało się zmienić uprawnień konta { $name }
account-remove-title = Usunąć konto { $name }?
account-remove-body = Ta osoba zostanie wszędzie wylogowana i nie będzie mogła zalogować się ponownie. Nic, co dodała do biblioteki, nie zostanie usunięte.
account-remove = Usuń
account-remove-failed = Nie udało się usunąć konta { $name }
account-token-failed = Nie udało się utworzyć tokenu
account-copied = Skopiowano
account-copy-token-failed = Nie udało się skopiować: należy zaznaczyć token i skopiować go ręcznie
account-copy-failed = Nie udało się skopiować: należy zaznaczyć tekst i skopiować go ręcznie
account-revoke-title = Unieważnić „{ $name }”?
account-revoke-body = Wszystko, co nadal go używa, będzie od teraz odrzucane.
account-revoke = Unieważnij
account-revoke-failed = Nie udało się unieważnić „{ $name }”
account-disconnect-title = Odłączyć { $app }?
account-disconnect-body = Aplikacja będzie od teraz odrzucana, a aby korzystała z Hoard, trzeba będzie połączyć ją ponownie.
account-disconnect = Odłącz
account-disconnect-failed = Nie udało się odłączyć { $app }
account-guest-failed = Nie udało się tego zmienić
account-never = nigdy
account-you = Ty
account-sign-out = Wyloguj
account-signed-in-as-before = Zalogowano jako{ " " }
account-signed-in-as-admin = , z uprawnieniami administratora.
account-signed-in-as-edit = , z prawem zmieniania biblioteki.
account-signed-in-as-view = , z prawem przeglądania bez możliwości zmian.
account-app-signin = Aplikacja Hoard sama cię loguje, więc nie ma hasła. Poniżej utwórz token dostępu dla wtyczek silników i polecenia hoard.
account-password-at-provider = Hasło jest przechowywane przez dostawcę, przez którego odbywa się logowanie, więc zmienia się je tam.
account-password-at-directory = Hasło jest przechowywane przez katalog, przez który odbywa się logowanie, więc zmienia się je tam.
account-current-password = Obecne hasło
account-new-password = Nowe hasło
account-new-password-again = Powtórz nowe hasło
account-change-password = Zmień hasło
account-password-mismatch = Nowe hasła nie są zgodne.
account-password-rule = Co najmniej 8 znaków.
account-people = Osoby
account-people-hint = Każda osoba może przeglądać (wyszukiwanie, podgląd i pobieranie), także zmieniać bibliotekę (dodawanie, edycja, tagowanie, kolekcje i archiwizowanie) albo dodatkowo być administratorem (Ustawienia, konta, konserwacja i trwałe usuwanie).
account-you-tag = (ty)
account-signs-in-oauth = Loguje się przez dostawcę OAuth
account-signs-in-directory = Loguje się przez katalog
account-since = od { $date }
account-what-can-do = Uprawnienia konta { $name }
account-name = Nazwa
account-how-sign-in = Sposób logowania
account-password-label = Hasło (co najmniej 8 znaków)
account-password-placeholder = Hasło (min. 8 znaków)
account-what-they-can-do = Uprawnienia
account-add = Dodaj
account-ai-apps = Claude i inne aplikacje AI
account-ai-apps-hint = Po połączeniu Claude (lub dowolna aplikacja MCP) może wyszukiwać i pobierać zasoby oraz opisywać i tagować to, czego Hoard jeszcze nie opisał. Za pierwszym razem loguje się do Hoard; zgodę wyraża się tutaj, w przeglądarce.
account-mcp-address = Adres MCP Hoard
account-copy = Kopiuj
account-claude-code-1 = należy uruchomić{ " " }
account-claude-code-2 = , a następnie wpisać{ " " }
account-claude-code-3 = , aby się zalogować.{ " " }
account-claude-code-4 = { " " }opisuje paczkę.
account-claude-ai = Ustawienia → Łączniki → Dodaj niestandardowy łącznik, wkleić adres i kliknąć Połącz. Te aplikacje łączą się z serwerów Anthropic, więc Hoard musi być dostępny z internetu przez HTTPS.
account-claude-ai-local = Ustawienia → Łączniki → Dodaj niestandardowy łącznik, wkleić adres i kliknąć Połącz. Te aplikacje łączą się z serwerów Anthropic, więc Hoard musi być dostępny z internetu przez HTTPS (ten adres nie jest: należy ustawić Adres publiczny w Ustawienia → Połączenie).
account-connection-meta = użytkownik { $user }, od { $since } · ostatnie użycie: { $used }
account-no-apps = Brak połączonych aplikacji.
account-tokens = Tokeny dostępu
account-tokens-hint-1 = Dla narzędzia wiersza poleceń{ " " }
account-tokens-hint-2 = , skryptów, klientów MCP i procesu Unreal worker: wysyłają go jako{ " " }
account-tokens-hint-3 = . Token może robić to, co zostanie wybrane, w granicach własnych uprawnień, ale nigdy nie zarządza kontami ani tokenami. Token tylko do przeglądania wystarczy do wyszukiwania i pobierania oraz do synchronizacji projektu.
account-tokens-admin = Widoczne są tokeny i połączone aplikacje wszystkich osób; pozostali widzą tylko własne.
account-made-before = { "" }
account-made-after = : należy skopiować go teraz. Nie zostanie pokazany ponownie.
account-new-token = Nowy token
account-cli-1 = W CLI należy użyć{ " " }
account-cli-2 = { " " }lub zapisać token w zmiennej{ " " }
account-cli-3 = .
account-done = Gotowe
account-token-meta = utworzono { $made } · ostatnie użycie: { $used }
account-token-meta-by = utworzono { $made }, autor: { $by } · ostatnie użycie: { $used }
account-no-tokens = Brak tokenów.
account-token-name = Do czego będzie używany
account-token-name-placeholder = Do czego będzie używany, np. „Unreal worker”
account-token-role = Uprawnienia
account-make-token = Utwórz token
account-guests = Goście
account-guests-allow = Pozwól każdemu, kto ma dostęp do serwera, przeglądać bez logowania
account-guests-hint = Goście mogą wyszukiwać, wyświetlać podgląd i pobierać, ale nie mogą niczego dodawać, zmieniać ani usuwać, ani widzieć Ustawień.

## SignIn.vue

signin-create-title = Utwórz konto
signin-title = Logowanie
signin-setup-lede = Ten serwer nie ma jeszcze żadnego konta. Konto utworzone tutaj może wszystko; konta dla innych osób można dodać później w Ustawienia → Konto.
signin-overlay-lede = Przeglądanie jest tu dostępne dla wszystkich, ale zmiany wymagają konta.
signin-with = Zaloguj przez { $provider }
signin-or-password = lub za pomocą nazwy i hasła
signin-username = Nazwa użytkownika
signin-name = Nazwa
signin-new-password = Nowe hasło
signin-password = Hasło
signin-password-rule = Co najmniej 8 znaków.
signin-confirm-password = Potwierdź hasło
signin-mismatch = Hasła nie są zgodne.
signin-setup-code = Kod konfiguracji
signin-setup-code-hint-before = Kod przekazany temu serwerowi w{ " " }
signin-setup-code-hint-after = .
signin-creating = Tworzenie…
signin-signing-in = Logowanie…
signin-create = Utwórz konto
signin-submit = Zaloguj
signin-keep-browsing = Przeglądaj dalej
signin-directory-hint = Należy użyć nazwy i hasła { $directory }.
signin-forgot-1 = Zapomniane hasło Hoard? Na serwerze należy uruchomić{ " " }
signin-forgot-2 = { " " }(w Dockerze:{ " " }
signin-forgot-3 = ), aby ustawić nowe.

## SetupWizard.vue (first visit to a new server)

setup-title = Gdzie ma się znajdować biblioteka?
setup-lede = Należy wybrać folder na serwerze, w którym Hoard będzie przechowywać paczki, lub wskazać folder z już posiadanymi paczkami. Później w Ustawieniach można dodać kolejne lokalizacje, także na innych dyskach.
setup-lede-desktop = Należy wybrać folder, w którym Hoard będzie przechowywać paczki, lub wskazać folder z już posiadanymi paczkami. Później w Ustawieniach można dodać kolejne lokalizacje, także na innych dyskach.
setup-unreachable = Nie udało się połączyć z serwerem: { $error }
setup-unreachable-desktop = Hoard nie odpowiada: { $error }
setup-try-again = Spróbuj ponownie
setup-loading = Ładowanie…
setup-start = Utwórz bibliotekę
setup-ready = Biblioteka jest gotowa
setup-managed-before = Nowe paczki trafiają do{ " " }
setup-managed-after = . Aby dodać pierwsze, wystarczy upuścić pliki, foldery lub archiwa w dowolnym miejscu strony.
setup-watched-before = Hoard obserwuje folder{ " " }
setup-watched-after = ; paczki pojawiają się w miarę ich znajdowania. Aby móc też przesyłać pliki, należy dodać bibliotekę zarządzaną przez Hoard w Ustawienia → Lokalizacje biblioteki.
setup-open = Otwórz bibliotekę

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = Połączyć { $app }?
connect-lede-1 = { " " }będzie mieć możliwość wyszukiwania, pobierania, opisywania i tagowania wszystkiego w tym Hoard w imieniu konta{ " " }
connect-lede-2 = . Nie będzie mieć możliwości zarządzania kontami ani tokenami dostępu.
connect-back-before = Nastąpi powrót do{ " " }
connect-back-after = . Aplikację można odłączyć w każdej chwili w Ustawienia → Konto.
connect-allow = Zezwól
connect-cancel = Anuluj
connect-failed = Nie można połączyć tej aplikacji
connect-go-library = Przejdź do biblioteki
connect-checking = Sprawdzanie…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = To jest zwykłe http://, a kilka funkcji wymaga HTTPS: 3D z WebGPU, nowoczesny schowek, instalacja Hoard jako aplikacji i łączniki Claude.
insecure-switch = Przełącz na https://
insecure-set-up = Skonfiguruj HTTPS
insecure-dismiss = Nie pokazuj więcej

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Ten plik jest za duży, by odczytać go jako plik sklepu.
formatwizard-not-json = Ten plik nie jest poprawnym plikiem JSON, więc jest wyświetlany jako tekst.
formatwizard-read-failed = Nie udało się odczytać pliku: { $error }
formatwizard-back-title = Wrócić do formularza?
formatwizard-back-body = Zmiany wprowadzone w TOML zostaną utracone: formularz zapisze format od nowa na podstawie swoich danych.
formatwizard-back-to-form = Wróć do formularza
formatwizard-miss-name = Format nie obejmuje plików o nazwie { $file }. Należy dodać ją w polu „Pliki”.
formatwizard-miss-not-the-stores = Tego, po czym format rozpoznaje pliki sklepu, nie ma w tym pliku. Należy zmienić pole „Rozpoznawaj po”.
formatwizard-miss-no-title = Format nie znajduje w tym pliku tytułu. Należy wskazać tytuł.
formatwizard-miss-short-name = inna nazwa
formatwizard-miss-short-not-the-stores = nie ze sklepu
formatwizard-miss-short-no-title = brak tytułu
formatwizard-saved-packs =
    Zapisano { $file }. { $packs ->
        [one] { $packs } paczka jest odczytywana
        [few] { $packs } paczki są odczytywane
        [many] { $packs } paczek jest odczytywanych
       *[other] { $packs } paczki jest odczytywane
    } ponownie.
formatwizard-saved = Zapisano { $file }.
formatwizard-save-failed = Nie udało się zapisać formatu: { $error }
formatwizard-zip-failed = Nie udało się utworzyć pliku zip: { $error }
formatwizard-stop-title = Przerwać uczenie tego pliku?
formatwizard-stop-body = Format nie zostanie zapisany.
formatwizard-stop = Przerwij
formatwizard-label = Kreator formatu
formatwizard-changing = Zmiana formatu { $name }
formatwizard-starting-from = Na podstawie formatu { $name }
formatwizard-title = Naucz Hoard pliku sklepu
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }z paczki{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Edytuj jako TOML
formatwizard-file-reference = Plik, do wglądu.
formatwizard-click-json = Należy kliknąć w pliku wartość pola „{ $field }”.
formatwizard-click-text = Należy kliknąć w pliku wartość pola „{ $field }” lub ją zaznaczyć.
formatwizard-pick-first = Należy wybrać pole po prawej, a następnie kliknąć tutaj jego wartość.
formatwizard-loading = Ładowanie…
formatwizard-the-format = Format
formatwizard-syntax-before = Składnia jest opisana w{ " " }
formatwizard-syntax-after = .
formatwizard-store = Sklep
formatwizard-store-placeholder = Nazwa sklepu, np. Kenney
formatwizard-files = Pliki
formatwizard-recognize = Rozpoznawaj po
formatwizard-recognize-json = Klucz, który mają tylko pliki sklepu
formatwizard-recognize-text = Wiersz, który mają tylko pliki sklepu
formatwizard-recognize-hint = Jeden sposób w wierszu; wystarczy dowolny z nich. Potrzebne, gdy inne pliki mają tę samą nazwę.
formatwizard-seller = Sprzedawca
formatwizard-seller-placeholder = Gdy plik tego nie podaje
formatwizard-tags = Tagi
formatwizard-tags-placeholder = Dla wszystkiego, co opisuje, rozdzielone przecinkami
formatwizard-fields = Pola
formatwizard-selector = Selektor: { $field }
formatwizard-clear = Wyczyść: { $field }
formatwizard-pick = Wskaż
formatwizard-what-it-reads = Co odczytuje
formatwizard-read-title = Tytuł
formatwizard-read-seller = Sprzedawca
formatwizard-read-license = Licencja
formatwizard-read-url = Strona w sklepie
formatwizard-read-kind = Rodzaj
formatwizard-read-tags = Tagi
formatwizard-read-origin = Miejsce powstania
formatwizard-read-size = Rozmiar
formatwizard-also = { $format } również odczytuje ten plik. Ten format zostałby sprawdzony jako pierwszy.
formatwizard-library =
    { $files ->
        [one] { $files } plik
        [few] { $files } pliki
        [many] { $files } plików
       *[other] { $files } pliku
    } w { $packs ->
        [one] { $packs } paczce
        [few] { $packs } paczkach
        [many] { $packs } paczkach
       *[other] { $packs } paczki
    } { $files ->
        [one] ma
        [few] mają
        [many] ma
       *[other] ma
    } obsługiwaną nazwę. Sprawdzono: { $tried }, odczytano: { $read }.
formatwizard-library-none = Żaden plik w bibliotece nie ma obsługiwanej nazwy.
formatwizard-share = Udostępnij
formatwizard-share-hint = Plik zip z formatem, tym plikiem jako przykładem i wynikiem jego odczytu, ułożony jak w repozytorium Hoard, do przesłania, by każdy mógł z niego korzystać. Najpierw należy usunąć z przykładu wszystko, czego nie należy udostępniać.
formatwizard-sample = Przykład do udostępnienia
formatwizard-download = Pobierz { $file }
formatwizard-close = Zamknij
formatwizard-use = Użyj tutaj
formatwizard-cancel = Anuluj
