## Sunucu hataları

error-account-not-found = hesap bulunamadı
error-asset-gone = varlık bulunamadı
error-asset-not-found = varlık bulunamadı: { $id }
error-blob-not-found = blob bulunamadı: { $hash }
error-collection-not-found = koleksiyon bulunamadı: { $id }
error-connection-not-found = bağlantı bulunamadı
error-derivative-not-found = { $id } için { $kind } türevi bulunamadı
error-file-in-pack-not-found = { $path } bulunamadı (paket: { $id })
error-format-not-found = biçim bulunamadı: { $file }
error-frame-sheet-aseprite-only = yalnızca Aseprite dosyalarının kare sayfası olur
error-frame-sheet-frames = bunun tek karesi var ya da yerleştirilemeyecek kadar çok karesi var
error-location-not-found = konum bulunamadı: { $id }
error-maintenance-task-not-found = bakım görevi bulunamadı: { $id }
error-open-upload-not-found = açık yükleme bulunamadı: { $id }
error-original-not-found = { $id } için özgün dosya bulunamadı
error-pack-not-found = paket bulunamadı: { $id }
error-preview-too-big = bu görsel, hafif önizlemesi yapılamayacak kadar büyük
error-project-not-found = proje bulunamadı: { $name }
error-saved-search-not-found = kayıtlı arama bulunamadı: { $id }
error-sign-in-first = önce oturum açın (ya da bir erişim belirteci gönderin)
error-those-assets-not-found = bu varlıklar bulunamadı
error-thumbnail-not-found = { $id } için küçük resim bulunamadı
error-token-not-found = belirteç bulunamadı
error-upload-not-found = yükleme bulunamadı: { $id }
error-pack-needs-name = bir paketin adı olmalı
error-no-library = dosyaların saklanacağı bir kütüphane konumu henüz yok: Ayarlar → Kütüphane konumları bölümünden yönetilen bir konum ekleyin
error-upload-failed = yükleme başarısız oldu: { $reason }
error-upload-no-files = yüklemede dosya yok
error-analyze-nothing-to-do = yapılacak bir şey yok: rules=false ve LLM kapalı
error-too-many-assets = tek istekte çok fazla varlık var
error-too-many-items = tek istekte çok fazla öğe var
error-not-a-hash = bu bir blake3 onaltılık özeti değil
error-merge-into-tag = into bir etiket olmalı
error-nothing-to-merge = birleştirilecek bir şey yok
error-selection-size = 1 ile { $max } arasında varlık kimliği verin
error-collection-needs-name = bir koleksiyonun adı olmalı
error-collection-exists = { $name } adlı bir koleksiyon zaten var
error-collection-empty = { $name } adlı koleksiyon boş
error-conversion-failed = { $format } biçimine dönüştürülemedi: { $reason }
error-max-size = max, 64 ile 16384 arasında ikinin bir kuvveti olmalı
error-replace-ext = ext kısa bir dosya uzantısı olmalı
error-empty-file = boş dosya
error-imitation-silent = Kayıtta arka plandan ayrılan bir şey yok. Mikrofona daha yakın durup yeniden dene.
error-replace-duplicate = dönüştürülen dosya pakette zaten bulunan bir dosyanın kopyası; paket o dosyayla birleştirildi
error-ai-address = Yapay zekâ adresi http:// ya da https:// ile başlamalı
error-local-llm-action = action list, download ya da delete olur; son ikisi bir model gerektirir
error-role = Rol view, edit ya da admin olur
error-admin-only = bunu yalnızca bir yönetici yapabilir
error-view-only = bu hesap görüntüleyebilir ama hiçbir şeyi değiştiremez: bir yönetici bunu Ayarlar → Hesap bölümünden değiştirebilir
error-portal-only = hesapları ve belirteçleri yönetmek için portalda oturum açın
error-not-from-portal = bu istek Hoard portalından gelmedi
error-account-name = En fazla 64 karakterlik bir ad seçin
error-password-length = En az { $min } karakterlik bir parola kullanın
error-too-many-codes = Çok fazla yanlış kod girildi: { $seconds } saniye bekleyin
error-already-set-up = Zaten bir hesap var: oturum açın
error-too-many-tries = Çok fazla yanlış deneme: { $seconds } saniye bekleyin
error-directory-unavailable = Dizin parolanızı denetleyemedi: { $reason }
error-password-elsewhere = Parolanızı, oturum açtığınız dizin ya da sağlayıcı saklıyor
error-current-password = Geçerli parolanız doğru değil
error-account-source = Bir hesap Hoard (internal), bir dizin (ldap) ya da OAuth ile oturum açar
error-account-exists = Bu adla zaten bir hesap var
error-own-role = Rolünüzü başka bir yönetici değiştirebilir
error-remove-self = Oturum açık olduğu sürece kendi hesabınızı kaldıramazsınız
error-token-name = Belirteci, onu kullanacak şeye göre en fazla 80 karakterle adlandırın
error-token-role = Bir belirteç, onu oluşturan hesaptan fazlasını yapamaz
error-wrong-setup-code = Bu, sunucuya HOARD_SETUP_CODE ile verilen kurulum kodu değil
error-wrong-password = Bu ad ve parola eşleşmiyor
error-folder-missing = { $path } orada değil (bağlanmamış bir disk mi?)
error-s3-path = bir S3 konumu s3://bucket biçimindedir
error-s3-address = S3 adresi http:// veya https:// ile başlamalıdır
error-s3-header = ek bir S3 üst bilgisi Name=value biçimindedir ve “{ $header }” böyle değil
error-keychain = sistemin anahtar zincirinde saklanamadı: { $reason }
error-not-built = Hoard'un bu kopyası { $feature } olmadan derlendi
error-openai-auth-unreachable = auth.openai.com adresine ulaşılamadı
error-openai-unexpected = OpenAI beklenmeyen bir yanıt verdi
error-openai-answered = OpenAI yanıtı { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID belirteci bir JWT değil
error-chatgpt-id-token-issuer = ID belirtecini veren: { $issuer } (beklenen: { $expected })
error-chatgpt-id-token-audience = ID belirteci başka bir uygulama için
error-chatgpt-id-token-expired = ID belirtecinin süresi doldu
error-chatgpt-id-token-nonce = ID belirteci bu oturum açma işlemine ait değil
error-chatgpt-no-state = adreste state yok; adresin tamamını kopyalayın
error-chatgpt-not-started = bu oturum açma işlemi burada başlatılmadı ya da 15 dakikadan uzun süre önce başlatıldı. Ayarlar’dan yeniden başlatın.
error-chatgpt-canceled = oturum açma iptal edildi
error-chatgpt-refused = ChatGPT oturum açmayı reddetti: { $reason }
error-chatgpt-no-code = adreste oturum açma kodu yok; adresin tamamını kopyalayın
error-chatgpt-other-registration = ChatGPT, bu sunucunun başka bir kaydı için yanıt verdi; oturum açmayı yeniden başlatın
error-chatgpt-other-registration-desktop = ChatGPT, Hoard’un başka bir kopyası için yanıt verdi; oturum açmayı yeniden başlatın
error-chatgpt-not-registered = ChatGPT, Hoard kaydını tamamlamadı; oturum açmayı yeniden başlatın
error-chatgpt-code-used = oturum açma kodu zaten kullanılmış ya da süresi dolmuş; yeniden oturum açın
error-chatgpt-refused-code = ChatGPT oturum açmayı reddetti ({ $code })
error-chatgpt-no-id-token = ChatGPT ID belirteci göndermedi
error-chatgpt-plan-not-allowed = Oturum açıldı ama planın kullanılmasına izin verilmedi. Yeniden oturum açıp izin verin ya da bunun yerine bir API anahtarı kullanın.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = ChatGPT oturumu açık değil; Ayarlar → Yapay zekâ etiketleme bölümünden oturum açın
error-chatgpt-refresh-later = ChatGPT oturumu şu anda yenileyemedi ({ $code }); daha sonra yeniden denenecek
error-chatgpt-signed-out-problem = ChatGPT bu sunucunun oturumunu kapattı ({ $code }). Planınızla etiketlemeye devam etmek için yeniden oturum açın.
error-chatgpt-signed-out-problem-desktop = ChatGPT, Hoard’un oturumunu kapattı ({ $code }). Planınızla etiketlemeye devam etmek için yeniden oturum açın.
error-chatgpt-signed-out = ChatGPT bu sunucunun oturumunu kapattı ({ $code }); Ayarlar → Yapay zekâ etiketleme bölümünden yeniden oturum açın
error-chatgpt-signed-out-desktop = ChatGPT, Hoard’un oturumunu kapattı ({ $code }); Ayarlar → Yapay zekâ etiketleme bölümünden yeniden oturum açın
error-openai-api-unreachable = api.openai.com adresine ulaşılamadı
error-openai-broke-off = OpenAI’ın yanıtı yarıda kesildi
error-chatgpt-kept-refusing = OpenAI, ChatGPT oturum açma işlemini reddetmeyi sürdürdü; Ayarlar → Yapay zekâ etiketleme bölümünden yeniden oturum açın
error-chatgpt-request-failed = istek başarısız oldu
error-chatgpt-cut-off = yanıt kesildi ({ $reason })
error-chatgpt-cut-off-no-reason = yanıt kesildi (neden belirtilmedi)
error-openai-ended-early = OpenAI’ın yanıtı tamamlanmadan bitti
error-chatgpt-usage-limit = ChatGPT planınızın Hoard için kullanım hakkı şimdilik doldu. Sınırı görmek ya da değiştirmek için: { $url }
error-chatgpt-not-eligible = bu ChatGPT hesabı ya da çalışma alanı, planını başka uygulamalarda kullanamaz. Bunun yerine bir API anahtarı kullanın.
error-chatgpt-usage-unavailable = ChatGPT planın kullanımını şu anda denetleyemedi; daha sonra yeniden denenecek
error-chatgpt-unsupported = ChatGPT plan kullanımı bu modelle şunu desteklemiyor: { $param }
error-chatgpt-unsupported-images = ChatGPT plan kullanımı bu modelle şunu desteklemiyor: { $param }; görsel alan bir model seçin ya da küçük resim göndermeyi kapatın
error-chatgpt-unsupported-part = ChatGPT plan kullanımı bu modelle isteğin bir bölümünü desteklemiyor
error-chatgpt-port = port, tarayıcının Hoard’a ulaştığı port olmalı
error-chatgpt-paste-address = http://127.0.0.1 ile başlayan adresin tamamını yapıştırın
error-too-many-hashes = bir seferde en fazla { $max } özet
error-credits-pack = paket { $key }
error-credits-collection = koleksiyon { $key }
error-format-too-long = biçim çok uzun
error-not-started = sunucu başlamadı
error-not-started-desktop = Hoard başlamadı
error-not-format-file = bir biçim dosyası değil: { $name }
error-format-name = adda A–Z arası bir harf ya da 0–9 arası bir rakam olmalı
error-format-miss-name = biçimin dosyaları şunu almıyor: { $file }
error-format-miss-store = biçim, mağazanınkiler için şunu almıyor: { $file } (bkz. require)
error-format-miss-title = biçim şu dosyada başlık bulamıyor: { $file }
error-cant-download-as = .{ $ext } dosyaları { $format } olarak indirilemez
error-only-images-resize = yalnızca görseller yeniden boyutlandırılabilir
error-https-no-certificate = Önce sertifikanızı ve anahtarını yapıştırın ya da yükleyin
error-https-no-cert-found = Sertifika bulunamadı: -----BEGIN CERTIFICATE----- ile başlayan metni yapıştırın
error-https-no-key-found = Özel anahtar bulunamadı: -----BEGIN PRIVATE KEY----- ile başlayan metni yapıştırın
error-https-several-keys = Birden fazla özel anahtar var: yalnızca sertifikaya ait olanı yapıştırın
error-https-key-password = Anahtar bir parolayla korunuyor: parolasız hâlini verin (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Sertifika geçerli bir PEM değil
error-https-key-not-pem = Özel anahtar geçerli bir PEM değil
error-https-key-mismatch = Anahtar bu sertifikaya ait değil. Bir zincir verdiyseniz sunucunuzun sertifikasını en başa koyun
error-https-unusable = Sertifika ya da anahtar kullanılamıyor
error-https-expired = Bu sertifikanın süresi şu tarihte doldu: { $date }
error-https-not-yet-valid = Bu sertifika şu tarihe kadar geçerli değil: { $date }
error-https-unreadable = Bu, Hoard’un okuyabildiği bir sertifika değil
error-https-bad-name = “{ $name }” bir sertifikanın taşıyabileceği bir adres değil
error-https-test-timed-out = test bağlantısı zaman aşımına uğradı
error-https-test-failed = test bağlantısı başarısız oldu
error-https-no-ca = Hoard henüz kendi sertifika yetkilisini oluşturmadı
error-https-unavailable = HTTPS bu sunucuda kullanılamıyor
error-https-certificate-mode = Sertifika, “Kendi sertifikam” seçeneğiyle birlikte verilir
error-https-test = Sertifika bir test bağlantısında çalışmadı: { $reason }
error-nothing-to-retry = orada yeniden denenebilecek bir şey yok
error-ai-unreachable = şu adrese ulaşılamadı: { $url }
error-ai-key-refused = { $url } API anahtarını reddetti ({ $status })
error-ai-answered = { $url } yanıtı { $status }: { $reason }
error-ai-not-json = { $url }/models JSON ile yanıt vermedi
error-model-server-refused = model sunucusu: { $reason }
error-no-model-server = şu adreste yanıt veren bir model sunucusu yok: { $url }
error-not-llama-router = { $url } yönlendirici modunda bir llama.cpp sunucusu değil, bu yüzden model indiremiyor
error-model-download = { $model } indirilemedi: { $reason }
error-model-delete = { $model } silinemedi: { $reason }
error-unknown-kind = bilinmeyen varlık türü: { $kind }
error-watched-is-folder = izlenen bir konum, bu sunucudaki bir klasördür
error-cant-reach = { $path } konumuna ulaşılamıyor: { $reason }
error-folder-full-path = klasörün tam yolunu verin
error-cant-create = oluşturulamıyor: { $path }
error-cant-write = yazılamıyor: { $path }
error-cant-read = okunamıyor: { $path }
error-overlaps-location = bu, { $name } adlı konumla çakışıyor ({ $path })
error-overlaps-data = bu, Hoard’un veri klasörüyle çakışıyor ({ $path })
error-overlaps-cache = bu, Hoard’un önbellek klasörüyle çakışıyor ({ $path })
error-not-watched = { $name } izlenen bir klasör değil
error-watched-no-kinds = izlenen bir klasör yeni dosya almaz, bu yüzden değiştirilecek türü yok
error-keep-catch-all = bunun yerine başka bir konumu genel konum yapın
error-location-holds-files = { $name } hâlâ dosya barındırıyor: önce onları diğer konumlara taşıyın (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } konumuna ulaşılamıyor, bu yüzden Hoard orada hâlâ dosya olup olmadığını anlayamıyor
error-relocate-running = dosyalar zaten konumlar arasında taşınıyor; bu bittiğinde yeniden deneyin
error-watched-not-moved = { $name } izlenen bir klasör: Hoard onun dosyalarını taşımaz
error-no-other-location = dosyaların taşınacağı başka bir kütüphane konumu yok: önce bir tane ekleyin
error-full-path = tam bir yol verin
error-pick-folder = bir klasör seçin
error-task-running = “{ $task }” zaten çalışıyor
error-token-account-gone = bu belirtecin hesabı artık yok, bu yüzden favorileri ya da aramaları yok
error-favorites-too-many = bir seferde en fazla 10.000
error-saved-search-name = kayıtlı aramanın en fazla 80 karakterlik bir adı olmalı
error-saved-search-too-long = bu arama kaydedilemeyecek kadar uzun
error-texture-pattern-map = Doku adı kalıbında { "{" }map{ "}" } olmalı, yoksa bir malzemenin tüm haritaları aynı adı alır
error-texture-pattern-fields = Doku adı kalıbı { "{" }name{ "}" }, { "{" }map{ "}" } ve { "{" }size{ "}" } kullanabilir
error-texture-name-chars = Doku adlarında < > : " / \ | ? ya da * olamaz
error-texture-map-type = Şu adda bir harita türü yok: { $role }
server-role-base-color = renk
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = pürüzlülük
server-role-gloss = parlaklık
server-role-metalness = metaliklik
server-role-ao = ortam kapanması
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = yükseklik
server-role-bump = tümsek
server-role-cavity = oyuk
server-role-specular = yansıma
server-role-opacity = opaklık
server-role-emissive = ışıma
server-role-translucency = yarı saydamlık
error-texture-suffix-missing = { $map } haritalarına bir sonek verin
error-texture-suffix-chars = { $map } haritalarının sonekinde < > : " / \ | ? ya da * olamaz
error-texture-ending-missing = Kendi ad sonlarınızın her biri için aranacak ad sonu gerekir
error-texture-ending-chars = Kendi ad sonlarınızda < > : " / \ | ? ya da * olamaz
error-texture-ending-twice = { $ending } ad sonlarınızda iki kez var
error-oauth-no-client-id = Bu bağlantıda client_id yok.
error-oauth-unknown-app = Bu uygulama burada kayıtlı değil (artık değil). Bağlanmaya uygulamadan yeniden başlayın.
error-oauth-redirect = Uygulama, kaydetmediği bir adrese geri gönderilmek istedi.
error-oauth-response-type = Hoard yalnızca oturum açma kodu verir (response_type=code).
error-oauth-pkce = Uygulamanın Hoard’da oturum açmak için PKCE’yi S256 ile kullanması gerekir.
error-light-preview-images = yalnızca görsellerin hafif önizlemesi olur
error-no-light-preview = hafif önizleme yok: { $reason }
error-no-frame-sheet = kare sayfası yok: { $reason }
error-thumbnail-source = bilinmeyen küçük resim kaynağı: { $source }
error-thumbnail-too-big = bu resim küçük resim için çok büyük
error-thumbnail-models-only = tarayıcıda çizilen küçük resmi yalnızca modeller alır
error-not-a-picture = Hoard’un okuyabildiği bir resim değil: { $reason }
error-project-name = bir projenin en fazla 200 karakterlik bir adı olmalı
error-project-too-big = bir projede en fazla bir milyon varlık
error-project-too-many-at-once = bir seferde en fazla 100.000
error-server-name = Tek satırda, en fazla { $max } karakterlik bir ad seçin
error-tool-setting = { $tool }: kapalı ya da otomatik seçin veya bir yol verin
error-ai-model = Yapay zekâ hizmetinin kullanacağı modeli seçin
error-public-address = Genel adres http:// ya da https:// ile başlamalı
error-update-hook-address = Güncellemeler için çağrılacak adres http:// ya da https:// ile başlamalı
error-update-hook-auth = Authorization üst bilgisinde satır sonu ya da başka denetim karakterleri olamaz
error-no-soundfont = Sunucuda şu yolda SoundFont yok: { $path }
error-no-soundfont-desktop = Şu yolda SoundFont yok: { $path }
error-workers = Arka plan çalışanları: 1 ile { $max } arası
error-ai-batch = Yapay zekâ isteği başına varlık: 1 ile 200 arası
error-conversion-cache = Dönüştürülmüş indirmeler: en az 100 MB
error-conversion-keep-days = Dönüştürme kuralları: özgünler 1 ile { $max } gün arasında tutulmalı
error-conversion-target = Dönüştürme kuralları: Hoard dosyaları { $format } biçimine dönüştüremez
error-conversion-no-source = Dönüştürme kuralları: her kuralın dönüştüreceği en az bir biçim olmalı
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Dönüştürme kuralları: .{ $ext } dosyaları { $format } biçimine dönüştürülemez
error-conversion-twice = Dönüştürme kuralları: .{ $ext } iki kuralda birden var
error-conversion-cant-undo = Bu dosya bir kuralla dönüştürülmedi ya da özgün dosyası silindi.
error-conversion-path-taken = { $path } konumunda artık başka bir dosya var, bu yüzden özgün dosya oraya geri konamaz.
error-limits-zero = Sınırlar sıfır olamaz
error-settings-object = ayarlardan oluşan bir JSON nesnesi bekleniyordu
error-bad-setting = hatalı ayar: { $reason }
error-auth-default-role = Varsayılan rol view, edit ya da admin olur
error-auth-session-hours = Oturumlar 1 saat ile bir yıl arasında sürer
error-auth-settings = hatalı kimlik doğrulama ayarları: { $reason }
error-ldap-settings = hatalı LDAP ayarları: { $reason }
error-ldap-no-server = Dizin sunucusunun adresini ekleyin
error-ldap-base-dn = Kişilerin kayıtlarının nerede olduğunu belirtin (temel DN)
error-ldap-username-attribute = Kişilerin oturum açarken kullandığı adı hangi özniteliğin tuttuğunu belirtin
error-ldap-bind-dn = Kişileri arayacak hesabı verin ya da onları anonim olarak arayın
error-ldap-ca-cert = Sertifika yetkilisi bir PEM sertifikası değil (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Bir dizin sunucusunun adresi boş
error-ldap-server-form = { $server }: bir dizin sunucusunun adresi host:port, ldap://… ya da ldaps://… biçimindedir
error-ldap-not-starttls = { $server }: bu StartTLS değil, LDAPS
error-ldap-no-certificate = içinde sertifika yok
error-ldap-password-again = Kişileri arayan hesabın parolasını yeniden verin: kayıtlı parola yalnızca verildiği sunuculara gönderilir
error-ldap-unreachable = hiçbir dizin sunucusuna ulaşılamadı ({ $reasons })
error-ldap-no-anonymous = dizin, kişilerin anonim olarak aranmasına izin vermiyor ({ $reason })
error-ldap-bind-failed = kişileri arayan hesap oturum açamadı ({ $reason })
error-ldap-search = aranıyor: { $name } (konum: { $base })
error-ldap-group-search = { $name } adlı kullanıcının grupları aranıyor (konum: { $base })
error-ldap-several = dizinde { $name } ile eşleşen { $count } kayıt var: kullanıcı filtresini daraltın
error-ldap-password-check = dizin, { $name } adlı kullanıcının parolasını denetlemedi ({ $reason })
error-ldap-wrong-password = Dizin, bu parolayla { $name } adlı bir kullanıcı tanımıyor
error-oauth-client-id = Sağlayıcının Hoard için oluşturduğu istemci kimliğini (client ID) verin
error-oauth-addresses = Yetkilendirme ve belirteç adresleri https:// ile başlamalı
error-oauth-userinfo-address = Kullanıcı bilgisi adresi https:// ile başlamalı
error-oauth-username-claim = Kişinin adını hangi claim’in tuttuğunu belirtin
error-oauth-secret-again = İstemci gizli anahtarını yeniden verin: kayıtlı anahtar yalnızca verildiği belirteç adresine gönderilir
error-oauth-issuer = Yayımcı (issuer) https:// ile başlamalı
error-oauth-discovery-unreachable = Şu adrese ulaşılamadı: { $url } ({ $reason })
error-oauth-discovery-status = { $url } yanıtı: { $status }
error-oauth-discovery-bad = { $url } bir keşif belgesi değil
error-oauth-discovery-incomplete = { $url } yetkilendirme ve belirteç adreslerini belirtmiyor
error-signin-no-group = { $name }, Hoard’u kullanabilen bir grupta değil
error-signin-other-source = { $name } adlı, başka bir yolla oturum açan bir Hoard hesabı zaten var
error-signin-other-subject = Sağlayıcıda başka birine ait { $name } adlı bir Hoard hesabı zaten var
error-signin-no-account = { $name } için henüz bir Hoard hesabı yok: bir yönetici bunu Ayarlar → Hesap bölümünden ekleyebilir
error-signin-bad-name = Bu ad bir Hoard hesabı olamaz (en fazla 64 karakter)
error-oauth-off = OAuth ile oturum açma etkin değil
error-oauth-bad-authorization-address = yetkilendirme adresi geçerli değil
error-oauth-refused = Sağlayıcı oturumunuzu açmadı: { $reason }
error-oauth-no-code = Sağlayıcı sizi oturum açmadan geri gönderdi
error-oauth-expired = Bu oturum açma işlemi çok uzun sürdü ya da başka bir yerde başlatıldı: yeniden deneyin
error-oauth-no-claim = Sağlayıcı kim olduğunuzu belirtmedi ({ $claim } claim’i yok)
error-oauth-no-sub = Sağlayıcı kim olduğunuzu belirtmedi (sub ya da id claim’i yok)
error-oauth-token-unreachable = sağlayıcının belirteç adresine ulaşılamadı
error-oauth-token-refused = Sağlayıcı oturum açmayı tamamlamadı: { $reason }
error-oauth-no-access-token = sağlayıcı erişim belirteci göndermedi
error-oauth-userinfo-unreachable = sağlayıcının kullanıcı bilgisi adresine ulaşılamadı
error-oauth-userinfo-refused = Sağlayıcı kimin oturum açtığını belirtmedi ({ $status })
error-oauth-userinfo-not-json = sağlayıcının kullanıcı bilgisi JSON değil
error-format-no-name = name boş
error-format-no-files = files boş: biçimin hangi dosyaları okuduğunu belirtin, örneğin ["*.json"]
error-format-empty-selector = boş
error-format-unopened-bracket = açılmamış bir kapanış köşeli parantezi
error-format-unclosed-bracket = bir köşeli parantez kapatılmamış
error-format-empty-step-dots = boş bir adım (art arda iki nokta ya da başta veya sonda bir nokta)
error-format-unclosed-except = !( kapatılmamış
error-format-unclosed-filter = [ kapatılmamış
error-format-after-filters = { $step } adımının anahtarı ve filtrelerinden sonra gelen { $rest }
error-format-except-needs-glob = !(…) içinde * olan bir anahtardan sonra gelir
error-format-except-not-glob = !(…) içinde * olan bir anahtardan sonra gelir; { $key } öyle değil
error-format-empty-step = boş bir adım
error-format-not-a-filter = [{ $filter }] bir filtre değil: [], [key=value|other], [array], [string], [number], [bool] ya da [object] kullanın
error-format-filter-needs-value = [{ $filter }] bir anahtar ve bir değer gerektirir
error-store-search-empty = aranacak bir şey yok; paketin adını yazın
error-update-hook-not-set = önce çağrılacak bir adres belirleyin
error-upload-stalled = { $path } yüklemesi takıldı; o parçayı yeniden gönderin
error-upload-busy = { $path } zaten yazılıyor
error-upload-incomplete = { $count } dosya eksik, örneğin: { $files }
error-upload-no-files-listed = dosya yok
error-upload-not-part = { $path }, { $id } yüklemesinin bir parçası değil ya da yükleme kapatılmış
error-bad-path = hatalı yol
error-upload-part-offset = { $offset }, { $path } dosyasında bir parçanın başladığı yer değil
error-upload-interrupted = yükleme kesildi: { $reason }
error-upload-part-long = { $path } dosyasının { $part }. parçası { $len } bayttan uzun
error-upload-part-short = { $path } dosyasının { $part }. parçası: { $got } bayt geldi, { $len } bekleniyordu
error-upload-file-long = { $path } bildirilenden büyük
error-upload-file-short = { $path }: { $got } bayt geldi, { $len } bekleniyordu
error-upload-complete = yükleme zaten tamamlandı
error-already-imported = zaten “{ $name }” olarak içe aktarıldı

## Sunucu metinleri

server-store-place-readme = README
server-store-place-folders = klasörler
server-store-why-linked = Paketteki { $place } içinde bağlantısı var
server-store-why-same-title = Paketteki { $place } ile aynı başlık
server-store-why-by = Yapan: { $author } (paketteki { $place } içinde adı geçiyor)
server-store-why-meshes = Paketle hemen hemen aynı sayıda mesh ({ $count })
server-store-why-textures = Paketle hemen hemen aynı sayıda doku ({ $count })
server-chatgpt-revoke-failed = Burada oturum kapatıldı ama OpenAI’a bildirilemedi. Emin olmak için ChatGPT’de Ayarlar → Güvenlik ve oturum açma → ChatGPT ile oturum aç bölümünden Hoard bağlantısını kesin.
server-chatgpt-signed-in-title = ChatGPT oturumu açıldı
server-chatgpt-signed-in = Hoard oturum açtı. Yapay zekâ etiketleme artık ChatGPT planınızı kullanabilir.
server-chatgpt-signed-in-as = Hoard şu hesapla oturum açtı: { $email }. Yapay zekâ etiketleme artık ChatGPT planınızı kullanabilir.
server-chatgpt-not-finished-title = Oturum açma tamamlanmadı
server-chatgpt-other-computer = Oturum açmayı başka bir bilgisayardaki Hoard’dan başlattıysanız bu sayfanın adresini adres çubuğundan kopyalayıp oradaki Hoard Ayarlar sayfasına yapıştırın.
server-back-to-settings = Ayarlar’a dön
server-issues-pack = Eklenemeyen paketler
server-issues-pack-blurb = Paketi silip yeniden ekleyin ya da içinde ne olduğunu görmek için açın.
server-issues-damaged = Hasarlı ya da kaybolmuş saklanan dosyalar
server-issues-damaged-blurb = “Dosyaları hasara karşı denetle” ile bulundu. Bunları bir yedekten geri yükleyin ya da paketlerini yeniden ekleyin, ardından “Dosyaları hasara karşı denetle”yi yeniden çalıştırın.
server-issues-folder = Okunamayan izlenen klasörler
server-issues-folder-blurb = Klasör yeniden okunabilene kadar paketlerine dokunulmaz.
server-issues-missing = Klasörü kaybolan izlenen paketler
server-issues-missing-blurb = Klasörü yalnızca çevrim dışı olabilir diye her paket 3 gün tutulur, sonra kaldırılır.
server-issues-job = Başarısız olan arka plan işleri
server-issues-job-blurb = Yeniden denemek işi baştan çalıştırır.
server-issues-file = Açılamayan dosyalar
server-issues-file-blurb = Hoard bunları açamadı ya da saklayamadı. İzlenen bir paket bunları bir sonraki taramasında yeniden dener; kütüphanede zaten bulunan bir dosya önceki hâlini korur.
server-issues-unpack = Açılamayan arşivler
server-issues-unpack-blurb = Her biri paketinde düz bir dosya olarak tutulur, bu yüzden içindeki hiçbir şey kütüphanede değildir.
server-issues-read = Hoard’un anlamlandıramadığı dosyalar
server-issues-read-blurb = Kütüphanedeler ama okunmalarıyla gelen bilgiler, etiketler ya da küçük resim olmadan.
server-issues-render = Küçük resim çizicisinin başarısız olduğu modeller
server-issues-render-blurb = Küçük resimleri yok. Birini görüntüleyicide açmak yine de işe yarayabilir ve bir küçük resim kaydedebilir.
server-issues-convert = Blender’ın dönüştüremediği modeller
server-issues-convert-blurb = GLB kopyaları yok, bu yüzden 3B olarak gösterilemezler.
server-issues-unreal = Dönüştürülemeyen Unreal paketleri
server-issues-unreal-blurb = Paketin diğer Unreal paketleri, paketin Unreal sayfasında.
server-issues-reformat = Bir dönüştürme kuralının dönüştüremediği dosyalar
server-issues-reformat-blurb = Oldukları gibi bırakıldılar.
server-issues-materials = Malzemeleri çıkarılamayan paketler
server-issues-materials-blurb = Dokuların hepsi orada; yalnızca malzemeler hâlinde gruplanmamışlar.
server-issues-material-thumb = Küçük resmi olmayan malzemeler
server-issues-material-thumb-blurb = Malzeme bulundu ama önizlemesi çizilemedi.
server-issues-store-files = Okunamayan mağaza dosyaları
server-issues-store-files-blurb = Paketin geldiği mağazaya ait, paketi adlandıran ve etiketleyen dosyalar (örneğin bir Megascans JSON’u).
server-issues-scan-again = Yeniden tara
server-issues-retry = Yeniden dene
server-issues-reanalyze = Paketi yeniden çözümle
server-issues-convert-again = Yeniden dönüştür
server-issues-look-again = Yeniden bak
server-issues-read-again = Yeniden oku
server-issues-blob-missing = Saklanan dosya kayboldu.
server-issues-blob-damaged = Saklanan dosya, saklanan içerikle eşleşmiyor: hasarlı.
server-issues-folder-gone = “{ $folder }” klasörü bulunamadı.
server-job-a-pack = bir paket
server-job-ingest = İçe alınıyor: { $pack }
server-job-retag = Yeniden etiketleniyor: { $pack }
server-job-llm-tag = Yapay zekâ ile etiketleniyor: { $pack }
server-job-convert = Blender ile dönüştürülüyor: { $pack }
server-job-unreal = Unreal paketleri dönüştürülüyor: { $pack }
server-job-reformat = Dönüştürme kurallarına göre dosyalar dönüştürülüyor: { $pack }
server-job-scan = İzlenen bir klasör taranıyor
server-job-relocate = Dosyalar kütüphane konumları arasında taşınıyor
server-job-looks = Renkler ve görünümler çıkarılıyor
server-job-sounds = Sesler dinleniyor
server-job-embed = Resimlerin ne gösterdiği öğreniliyor
server-job-hear = Seslerin neye benzediği öğreniliyor
server-job-materials = Malzemeler aranıyor: { $pack }
server-job-read-files = Dosyalar okunuyor: { $pack }
server-job-store-lookup = Bir mağaza sayfası aranıyor
server-maintenance-backup = Veritabanını yedekle
server-maintenance-backup-blurb = Veritabanının (paketler, etiketler, notlar, koleksiyonlar ve Ayarlar) tutarlı bir kopyası, yanındaki backups klasörüne. Kütüphanenin dosyaları buna dahil değildir: saklandıktan sonra hiç değişmezler, bu yüzden herhangi bir dosya eşitleme aracı onları yedekler.
server-maintenance-scrub = Dosyaları hasara karşı denetle
server-maintenance-scrub-blurb = Bir diskte ya da NAS’ta bozulmuş veya kaybolmuş dosyaları yakalamak için saklanan her dosyayı geri okur ve parmak iziyle karşılaştırır. Büyük bir kütüphanede biraz sürer.
server-maintenance-reanalyze = Tüm paketleri yeniden çözümle
server-maintenance-reanalyze-blurb = Her dosyayı Hoard’un bu sürümüyle yeniden okur: küçük resimler, önizlemeler, ayrıntılar ve kural etiketleri baştan oluşturulur. Kendi düzenlemeleriniz kalır; yapay zekâya yeniden sorulmaz.
server-maintenance-clear-cache = Önbellekteki dönüştürmeleri temizle
server-maintenance-clear-cache-blurb = “Farklı indir” dönüştürmelerinden tutulan dosyaları (bir sonraki istekte yeniden oluşturulurlar) ve yarıda kalan işlerin geride bıraktıklarını siler.
server-maintenance-shrink-thumbs = Küçük resimleri küçült
server-maintenance-shrink-thumbs-blurb = PNG olarak tutulan küçük resimleri ve model görünümlerini Ayarlar → Performans → Küçük resimler’in istediği biçimde saklar; bu genellikle onları birkaç kat küçültür. Saydamlığı olanlar ve piksel sanatı PNG olarak kalır.
server-maintenance-gc = Kullanılmayan dosyaları sil
server-maintenance-gc-blurb = Artık hiçbir şeyin göstermediği saklanan dosyaları siler; örneğin kalıcı olarak silinen paketlerin dosyaları.
server-update-no-address = çağrılacak bir adres ayarlanmamış
server-update-no-answer = 30 saniye içinde yanıt gelmedi

## Ayarlar: Görünüm

appearance-language = Dil
appearance-language-auto = Tarayıcının dili ({ $name })

## Kök

root-unreachable = Hoard sunucusuna ulaşılamadı: { $error }
root-unreachable-desktop = Hoard yanıt vermiyor: { $error }
root-desktop-reopen = Bu pencerenin oturumu kapatıldı. Devam etmek için Hoard'u kapatıp yeniden açın.
root-try-again = Yeniden dene
root-lazy-connect = uygulama bağlantısı
window-minimize = Simge durumuna küçült
window-maximize = Ekranı kapla
window-restore = Önceki boyut
window-close = Kapat

## Uygulama: kütüphane, üst çubuk ve filtreler

app-lazy-settings = ayarlar
app-lazy-archive = arşiv
app-lazy-shortcuts = kısayollar
app-lazy-setup = kurulum
app-theme-system = Sistem temasına uy
app-theme-light = Açık tema
app-theme-dark = Koyu tema
app-color-red = Kırmızı
app-color-orange = Turuncu
app-color-yellow = Sarı
app-color-green = Yeşil
app-color-teal = Camgöbeği
app-color-blue = Mavi
app-color-purple = Mor
app-color-pink = Pembe
app-color-brown = Kahverengi
app-color-sand = Kum
app-color-gray = Gri
app-color-slate = Arduvaz
app-color-black = Siyah
app-color-white = Beyaz
app-color-swatch = { $color }: içinde bolca { $lower } olan varlıklar
app-color-any = Herhangi bir renk
app-color-pick = Bir renk seç
app-color = Renk
app-imitation = Şöyle ses
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Bir sesin çıkardığı gürültüyü hecelediğin gibi yaz: ona benzeyen sesler önce gelir
app-imitation-record = Sesi mikrofona çıkar: ona benzeyen sesler önce gelir
app-imitation-stop = Kaydı durdur
app-imitation-heard = Senin sesin
app-imitation-mic-failed = Mikrofondan kayıt yapılamadı: { $error }
app-retry = Yeniden dene
app-undo = Geri al
app-search-failed = Arama başarısız oldu: { $error }
app-unreachable = Sunucuya ulaşılamadı: { $error }
app-unreachable-desktop = Hoard yanıt vermiyor: { $error }
app-forget-project-title = { $name } unutulsun mu?
app-forget-project-body = Hoard, bir sonraki hoard sync ona yeniden bildirene kadar bu projeyi, { $count } varlığının kullanıldığı yerlerde listelemeyi bırakır. Projede hiçbir şey değişmez.
app-forget-project-confirm = Unut
app-forget-project-failed = { $name } unutulamadı: { $error }
app-forget-project-label = { $name } projesini unut
app-forget-project = Bu projeyi unut
app-save-name-like = { $name } benzeri
app-save-name-favorites = favoriler
app-search-saved = “{ $name }” kaydedildi: Filtreler’de, Kişisel altında
app-rank-alike = En benzer
app-rank-color = En çok bu renk
app-rank-match = En iyi eşleşme
app-rank-imitation = En çok benzeyenler
app-sort = Sırala
app-sort-newest = En yeni
app-sort-name = Ad
app-sort-largest = En büyük
app-sort-oldest = En eski
app-collection-added = { $count } varlık şuraya eklendi: { $name }
app-collection-add-failed = Koleksiyona eklenemedi: { $error }
app-collection-removed = { $count } öğe şuradan kaldırıldı: { $name }
app-collection-remove-failed = Koleksiyondan kaldırılamadı: { $error }
app-ns-subject = Konu
app-ns-style = Tarz
app-ns-theme = Tema
app-ns-material = Malzeme
app-ns-map = PBR haritası
app-ns-color = Renk
app-ns-mood = Hava
app-ns-genre = Tür
app-ns-license = Lisans
app-ns-res = Çözünürlük
app-ns-duration = Süre
app-ns-poly = Poligon sayısı
app-ns-format = Biçim
app-ns-weight = Ağırlık
app-ns-tech = Teknik
app-ns-blender = Blender sürümü
app-ns-depth = Bit derinliği
app-ns-ucs = Ses kategorisi (UCS)
app-ns-bpm = Tempo
app-ns-root = Kök nota
app-ns-level = Ses yüksekliği
app-ns-ue = Unreal sınıfı
app-ns-engine = Motor
app-ns-extracted = Kurtarılan kaynak
app-ns-script = Yazı sistemi
app-bulk-failed =
    { $count ->
        [one] Bu varlık değiştirilemedi: { $error }
       *[other] Bu { $count } varlık değiştirilemedi: { $error }
    }
app-bulk-untagged = { $tag } etiketi { $count } varlıktan kaldırıldı
app-bulk-tagged = { $count } varlığa { $tag } etiketi eklendi
app-bulk-category-set = { $count } varlığın kategorisi { $category } olarak ayarlandı
app-bulk-category-cleared = { $count } varlığın kategorisi temizlendi
app-credits-failed = Emeği geçenler listesi oluşturulamadı: { $error }
app-archived-assets = { $count } varlık arşivlendi
app-archived-one = Arşivlendi: { $name }
app-open-failed = Bu varlık açılamadı: { $error }
app-unnamed-pack = paket
app-unnamed-collection = koleksiyon
app-unnamed-asset = varlık
app-home-version = Hoard { $version } — kütüphaneye dön
app-home = Kütüphaneye dön
app-search-label = Varlıklarda ara
app-search-placeholder = Ad, yol ya da etiket ara
app-views = Görünümler
app-library = Kütüphane
app-packs = Paketler
app-packs-working = { $count } işleniyor
app-update-title = Hoard { $version } çıktı (bu sunucu { $current } sürümünü çalıştırıyor). Yeni imajı ya da ikili dosyayı çekip yeniden başlatın; veritabanı kendini yükseltir.
app-update = Güncelle
app-theme = Tema
app-sign-in = Oturum aç
app-archive-title = Arşiv: { $contents }
app-archive = Arşiv
app-settings = Ayarlar
app-palette-title = Komut paleti ({ $keys })
app-import-folder = Klasörü içe aktar
app-add-files = Dosya ekle
app-add-files-short = Ekle
app-split-label = Bu klasör nasıl içe aktarılsın
app-split-before = { "" }
app-split-after = { " " }içinde { $count } klasör ya da arşiv var.
app-split-each = Her birini ayrı bir paket olarak içe aktar ({ $count })
app-split-one = Tek bir paket olarak içe aktar
app-cancel = İptal
app-filters = Filtreler
app-done = Bitti
app-yours = Kişisel
app-favorites = Favoriler
app-forget-search-label = { $name } kayıtlı aramasını unut
app-forget-search = Bu aramayı unut
app-projects-title = Buradan hoard sync yapan projeler ve son eşitlemelerinde olanlar
app-projects = Projeler
app-project-synced = Son eşitleme: { $when }
app-project-synced-by = Son eşitleme: { $when }, eşitleyen: { $who }
app-collections = Koleksiyonlar
app-no-filters = Bu arama için filtre yok.
app-kind = Tür
app-facets-more = { $count } tane daha
app-facets-fewer = daha az
app-searching = Aranıyor…
app-searching-live = Aranıyor
app-asset-count = { $count } varlık
app-clear-all = Tümünü temizle
app-save-name-label = Bu aramanın adı
app-save-name-placeholder = Ad verin
app-save = Kaydet
app-save-search-title = Bu aramayı Filtreler’de, Kişisel altında tut
app-save-search = Aramayı kaydet
app-select-title = Birlikte etiketlemek, kategorilemek, koleksiyona eklemek, indirmek ya da arşivlemek için birkaç varlık seçin (X ya da Ctrl ile tıklama)
app-select = Seç
app-tile-size = Kutucuk boyutu
app-supporting-title = glTF arabellekleri, malzeme kütüphaneleri, malzemelerin haritaları, yalnızca tek bir modelin kullandığı malzemeler ve modelleri tanımlayan mağaza dosyaları. Onları kullanan şeyi indirdiğinizde birlikte gelirler. Bir sprite sayfasının sayfaları ve paketin ayrı görseller olarak da içerdiği spriteları sayfayla birlikte gösterilir. Paketin birkaç biçimde içerdiği bir varlık, Hoard’un en iyi okuduğu biçimde bir kez gösterilir.
app-supporting-hidden = { $count } yardımcı dosya gizlendi
app-supporting-show = Göster
app-supporting-shown = Yardımcı dosyalar dahil
app-supporting-hide = Gizle
app-folders = Klasörler
app-folders-title = Paketi diskteki hâliyle, klasör klasör gör
app-folders-label = Paketin klasörleri
app-welcome-title = Bir varlık paketini bu sayfada herhangi bir yere bırakın
app-welcome-before = Zip’ler, tarball’lar, klasörler ya da tek tek dosyalar; hepsi olur. Hoard bunları açar, her dosyayı özetine göre bir kez saklar, boyutları, süreleri ve poligon sayılarını okur ve her şeyi etiketler; böylece şunlar gibi aramalar yapabilirsiniz:{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }ya da{ " " }
app-welcome-after = .
app-welcome-import-folder = Bir klasörü içe aktar
app-welcome-choose-files = Dosya seç
app-none-similar = Henüz { $name } ile pek benzeşen başka bir şey yok.
app-none = Eşleşen bir şey yok.
app-none-query = “{ $query }” ile eşleşen bir şey yok.
app-none-imitation = Ölçülmüş hiçbir ses henüz “{ $word }” sesine pek benzemiyor.
app-none-heard = Ölçülmüş hiçbir ses henüz senin sesine pek benzemiyor.
app-none-filters = Bu filtrelerle eşleşen bir şey yok.
app-none-query-filters = Bu filtrelerle “{ $query }” ile eşleşen bir şey yok.
app-clear-search-filters = Aramayı ve filtreleri temizle
app-loading-more = Daha fazlası yükleniyor
app-keys-hint = Ok tuşları gezinir, Enter açar, Space bir sesi çalar, X ya da Ctrl ile tıklama birden fazla seçer (dokunmatik ekranda uzun basın).
app-resize-label = Ayrıntılar panelini yeniden boyutlandır
app-resize-title = Yeniden boyutlandırmak için sürükleyin · sığan genişlik için çift tıklayın
app-selected-label = Seçili varlıklar
app-selected = { $count } seçili
app-pick-assets = Varlık seçin
app-download-zip = Zip olarak indir
app-credits-title = Bunların geldiği paketler için CREDITS.md: onları kimin yaptığı, lisansları ve istedikleri emek satırları
app-credits = Emeği geçenler
app-prefer-sources-title = Hoard’un özgün dosyasını kurtardığı Unreal SoundWave’leri ve dokuları
app-prefer-sources = .uasset yerine kurtarılmış WAV/PNG ({ $count })
bag-clear = Seçimi temizle
bag-glb-heading = GLB olarak modeller
bag-glb-text = Her model, Y yukarıda ve öne bakacak şekilde duran bir .glb dosyası olarak; dokuları ise modellerin ortak kullandığı bir textures klasöründe ayrı dosyalar olarak. Geri kalan her şey olduğu gibi girer.
bag-glb-download = GLB zip’ini indir
bag-plain-heading = Dosyalar olduğu gibi
bag-plain-text = Her dosya içe aktarıldığı gibi, ihtiyaç duyduklarıyla birlikte, her paket için bir klasörde.
bag-remove = { $name } öğesini seçimden çıkar
bag-more = ve { $count } tane daha
app-bulk-tag-label = Seçili varlıklara eklenecek ya da onlardan kaldırılacak bir etiket
app-bulk-tag-placeholder = Etiket
app-bulk-tag-add-title = Bu etiketi seçili her varlığa ekle
app-bulk-tag-add = Ekle
app-bulk-tag-remove-title = Bu etiketi seçili her varlıktan kaldır
app-bulk-tag-remove = Kaldır
app-bulk-category-label = Seçili varlıkların kategorisi (boş bırakmak temizler)
app-bulk-category-placeholder = Kategori
app-bulk-category-set-title = Seçili her varlığın kategorisini ayarla
app-bulk-category-clear-title = Seçili her varlığın kategorisini temizle
app-bulk-category-set-button = Ayarla
app-bulk-category-clear-button = Temizle
app-remove-from-collection = { $name } koleksiyonundan çıkar
app-unfavorite = Favorilerden çıkar
app-favorite = Favorilere ekle
app-select-all = Tümünü seç ({ $count })
app-archive-selected-title = Arşiv’den geri yüklenene ya da kalıcı olarak silinene kadar aramada görünmezler
app-drop-items = Eklemek için { $count } öğeyi bırakın
app-drop = Hoard’a eklemek için bırakın
app-drop-hint = Her üst düzey dosya ya da klasör kendi paketi olur. Gigabaytlar sorun değil; bağlantı koparsa büyük dosyalar kaldığı yerden devam eder.

## Komut paleti

palette-group-go-to = Git
palette-group-actions = Eylemler
palette-group-theme = Tema
palette-group-kinds = Türler
palette-group-packs = Paketler
palette-group-collections = Koleksiyonlar
palette-group-projects = Projeler
palette-group-saved = Kayıtlı aramalar
palette-group-assets = Varlıklar
palette-hint-current = geçerli
palette-library = Kütüphane
palette-library-keywords = varlıklar ızgara ana sayfa assets grid home
palette-favorites = Favoriler
palette-favorites-keywords = yıldızlı yıldız beğenilen starred stars liked
palette-packs = Paketler
palette-archive = Arşiv
palette-archive-keywords = çöp geri dönüşüm kutusu silinen geri yükle trash recycle bin deleted restore
palette-stats = Kütüphane istatistikleri
palette-stats-keywords = istatistik sayılar boyut disk alanı sayı statistics numbers size disk space count
palette-search = Ara
palette-add-files = Dosya ekle…
palette-add-files-keywords = yükle içe aktar upload import
palette-import-folder = Bir klasörü içe aktar…
palette-import-folder-keywords = yükle paket upload pack
palette-select-all = Gösterilen { $count } öğenin tümünü seç
palette-select-all-keywords = işaretle check
palette-select-several = Birkaç tane seç…
palette-select-several-keywords = toplu çoklu etiket arşiv bulk multiple tag archive
palette-archive-selected = Seçili { $count } öğeyi arşivle
palette-archive-selected-keywords = sil kaldır çöp delete remove trash
palette-clear-selection = Seçimi temizle
palette-clear-filters = Filtreleri temizle
palette-clear-filters-keywords = sıfırla reset
palette-hide-supporting = Yardımcı dosyaları gizle
palette-show-supporting = Yardımcı dosyaları göster
palette-supporting-keywords = normal haritalar pürüzlülük maskeler normal maps roughness masks
palette-account = Hesap ve erişim belirteçleri
palette-account-keywords = parola giriş kullanıcı belirteç password login user token api
palette-sign-out = Oturumu kapat ({ $name })
palette-sign-out-keywords = çıkış oturumu kapat log out logout
palette-sign-in = Oturum aç
palette-sign-in-keywords = giriş oturum aç hesap parola log in login account password
palette-settings = Ayarlar
palette-settings-keywords = blender ffmpeg llm yapay zekâ etiketleme unreal yönetici araçlar tercihler ai tagging admin tools preferences
palette-shortcuts = Klavye kısayolları
palette-shortcuts-keywords = yardım tuşlar help keys
palette-theme-keywords = tema görünüm mod theme appearance mode
palette-theme-variant-keywords = görünüm renk appearance color
palette-light-theme = Açık tema: { $name }
palette-dark-theme = Koyu tema: { $name }
palette-kind-all = Tüm türler (yalnızca { $kind } göstermeyi bırak)
palette-kind-only = Yalnızca { $kind }
palette-title = Komut paleti
palette-placeholder = Varlık ara, bir pakete git, bir komut çalıştır…
palette-results = Sonuçlar
palette-searching = Aranıyor…
palette-no-match = “{ $query }” ile eşleşen bir şey yok
palette-foot-move = gezin
palette-foot-run = çalıştır
palette-foot-shortcuts = tüm kısayollar

## Klavye kısayolları

shortcuts-title = Klavye kısayolları
shortcuts-close = Kapat
shortcuts-anywhere = Her yerde
shortcuts-palette = Komut paleti: ara, atla, çalıştır
shortcuts-search = Ara
shortcuts-this-list = Bu liste
shortcuts-escape = Kapat, seçimi temizle ya da ayrıntıları kapat
shortcuts-library = Kütüphane
shortcuts-move = Kutucuklar arasında gezin (ya da J ve K)
shortcuts-open = Ayrıntıları aç
shortcuts-play-sound = Bir sesi çal ya da durdur
shortcuts-select = Seç ya da seçimi kaldır
shortcuts-favorite = Favorilere ekle ya da favorilerden çıkar
shortcuts-key-click = tıklama
shortcuts-select-range = Bir aralık seç
shortcuts-select-all = Gösterilen her şeyi seç
shortcuts-sound-player = Ses oynatıcı
shortcuts-play-pause = Oynat ya da duraklat
shortcuts-skip = Geri ya da ileri atla
shortcuts-start-end = Başa ya da sona git
shortcuts-image-inspector = Görsel inceleyici
shortcuts-zoom = Yakınlaştır ya da uzaklaştır
shortcuts-fit = Pencereye sığdır
shortcuts-actual-pixels = Gerçek pikseller
shortcuts-tile = Dikişleri denetlemek için 3×3 döşe
shortcuts-background = Sonraki arka plan
shortcuts-channel = Tek bir kanalı göster

## KindChips.vue

kinds-group = Varlık türleri

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = bir kişi ekledi
tag-source-rule = çevrim dışı etiketleyiciden
tag-source-meta = dosyadan ölçüldü
tag-source-ai = yapay zekâ önerdi
tag-remove = { $tag } etiketini kaldır

## ActivityIndicator.vue

activity-label-idle = Arka plan etkinliği: boşta
activity-label-running = Arka plan etkinliği: { $running } çalışıyor
activity-label-queued = Arka plan etkinliği: { $running } çalışıyor, { $queued } sırada
activity-title = Arka plan etkinliği
activity-unreachable = Sunucuya ulaşılamıyor
activity-unreachable-desktop = Hoard yanıt vermiyor
activity-empty = Çalışan bir şey yok. İçe aktarmalar, etiketleme, dönüştürmeler ve zip indirmeleri çalışırken burada görünür.
activity-queued = sırada
activity-seconds = { $s } sn
activity-minutes = { $min } dk
activity-hours = { $h } sa { $min } dk
activity-percent = %{ $pct }

## AssetTile.vue

tile-triangles = { $count } üçgen
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Oynat: { $name }
tile-pause = Duraklat: { $name }
tile-favorite = Favorilere ekle: { $name }
tile-in-favorites = Favorilerinizde
tile-add-favorite = Favorilerinize ekleyin (F)
tile-more-like = { $name } benzerleri
tile-more-like-this = Buna benzerler
tile-select = Seç: { $name }

## CollectionPicker.vue

collections-add = Koleksiyona ekle
collections-menu = Koleksiyonlar
collections-find-label = Bir koleksiyon bul ya da oluştur
collections-find-placeholder = Bir koleksiyon bulun ya da yenisine ad verin
collections-create = Oluştur
collections-empty = Henüz koleksiyon yok. Başlatmak için bir ad yazın.

## FolderPicker.vue

folders-list = Klasörler
folders-close = Kapat
folders-browse = Gözat…
folders-up-title = Bir üst klasör
folders-up = Yukarı
folders-this-computer = Bu bilgisayar
folders-none = Burada klasör yok.

## LocationForm.vue

locations-mode-question = Hoard bu klasörle ne yapsın?
locations-mode-managed = Kütüphanemi burada tut
locations-mode-managed-hint = Yüklediğiniz ya da içe aktardığınız paketler bu klasöre kopyalanır. Klasörle Hoard ilgilenir, bu yüzden içeriğine dokunmayın.
locations-mode-watched = Zaten var olan bir klasörü dizinle
locations-mode-watched-hint = İçindeki her klasör bir paket olur. Hoard dosyaları bulundukları yerde okur, onları asla değiştirmez, taşımaz ya da silmez ve yeni klasörleri ortaya çıktıkça algılar.
locations-folder = Sunucudaki klasör
locations-folder-desktop = Klasör
locations-folder-placeholder = Paketlerinizin bulunduğu klasör
locations-folder-hint-managed = Yoksa Hoard onu oluşturur. Docker’da bu, kapsayıcıya bağladığınız bir klasördür.
locations-folder-hint-managed-desktop = Yoksa Hoard onu oluşturur.
locations-folder-hint-watched = Docker’da önce klasörü kapsayıcıya bağlayın (salt okunur olabilir), sonra burada seçin.
locations-folder-hint-watched-desktop = Hoard onu yalnızca okur: içindeki hiçbir şey değiştirilmez.
locations-folder-hint-cloud = Dropbox, Google Drive, OneDrive ya da iCloud Drive’ın eşitlediği bir klasör de olur; S3 depolamadaki bir bucket için s3://bucket-name de.
locations-name = Ad
locations-name-placeholder-managed = Kütüphane
locations-name-placeholder-watched = Paketlerim
locations-files = Buraya gelecek dosyalar
locations-catch-all = Başka hiçbir konumun almadığı her şey de
locations-kinds-hint = Yeni bir paketin dosyaları, türlerini işaretleyen konuma gider. Geri kalanı genel konuma gider.
locations-files-all = Şimdilik hepsi. Bazı türleri (örneğin müziği) başka bir diske göndermek için daha sonra Ayarlar’dan başka konumlar ekleyin.
locations-first-watched-note = Yükleme yapabilmek için Hoard’un tuttuğu bir kütüphane gerekir. Bunu daha sonra Ayarlar → Kütüphane konumları bölümünden ekleyebilirsiniz.
locations-checking = Klasör denetleniyor…
locations-add = Konum ekle
locations-cancel = İptal

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Kütüphane konumları yüklenemedi
locations-takes-and-rest = { $kinds } ve geri kalan her şey
locations-takes-rest = Diğerlerinin almadığı her şey
locations-takes-everything = Her şey
locations-takes-nothing = Henüz hiçbir şey
locations-save-failed = Konum kaydedilemedi
locations-unwatch-title = { $name } artık izlenmesin mi?
locations-unwatch-body = Buradaki { $count } paket, etiketleri ve notlarıyla birlikte Hoard’dan çıkar. Klasördeki dosyalara dokunulmaz.
locations-unwatch-body-empty = Klasördeki dosyalara dokunulmaz.
locations-unwatch = İzlemeyi bırak
locations-remove-title = { $name } kaldırılsın mı?
locations-remove-body = İçindeki dosyalar önce türlerine göre diğer konumlara taşınır; konum boşalınca kaldırılır. Hiçbir şey kaybolmaz.
locations-remove-body-only = Dosya barındırıyorsa önce onların taşınacağı başka bir konum ekleyin.
locations-remove = Kaldır
locations-removed = Kaldırıldı: { $name }
locations-emptying = Dosyalar { $name } konumundan taşınıyor; bu bitince konum kaldırılacak
locations-offline-title = { $name } konumuna ulaşılamıyor
locations-offline-body = Çevrim dışıyken dosyaları dışarı taşınamaz. Yine de kaldırılsın mı? Üzerinde kalan dosyalar kayıp olarak görünür.
locations-remove-anyway = Yine de kaldır
locations-remove-failed = { $name } kaldırılamadı
locations-rebalancing = Saklanan dosyalar türlerini alan konumlara taşınıyor
locations-rebalance-failed = Dosyalar taşınamadı
locations-scanned-never = henüz taranmadı
locations-scanned-now = az önce tarandı
locations-scanned-minutes = { $mins } dk önce tarandı
locations-scanned-hours = { $hours } sa önce tarandı
locations-scanned-date = son tarama: { $date }
locations-scan-failed = { $name } taranamadı
locations-watching = İzleniyor: { $name }
locations-added = Eklendi: { $name }
locations-loading = Yükleniyor…
locations-managed-title = Hoard’un tuttuğu kütüphaneler
locations-watched-title = İzlenen klasörler
locations-managed-hint = Yüklemeler ve içe aktarmalar buraya kopyalanır. Her konum işaretlediği türleri alır; genel konum geri kalanını alır.
locations-watched-hint = Bulundukları yerde dizinlenen paket klasörleri. Hoard içlerindeki dosyaları asla değiştirmez ve her 15 dakikada bir yeni ve kaldırılmış klasörlere bakar.
locations-managed-empty = Henüz yok, bu yüzden yüklemelerin konacağı bir yer yok. Aşağıdan bir tane ekleyin.
locations-reachable = Ulaşılabilir
locations-unreachable-now = Şu anda ulaşılamıyor
locations-catch-all-tag = genel
locations-packs = { $count } paket
locations-unreachable = ulaşılamıyor
locations-scanning = Taranıyor…
locations-scan = Şimdi tara
locations-edit = Düzenle
locations-catch-all-locked = Bunu değiştirmek için başka bir konumu genel konum yapın
locations-edit-hint-before = Bundan sonra eklenen dosyalara uygulanır. Zaten saklananları taşımak için aşağıdaki
locations-edit-hint-after = düğmesini kullanın.
locations-save = Kaydet
locations-rebalance = Saklanan dosyaları buna göre taşı
locations-rebalance-hint = Bir konumun aldığı türleri değiştirdikten sonra: zaten saklanan dosyalar, türlerinin artık gittiği konuma taşınır.
locations-add-title = Bir konum ekle
locations-add-hint = Bazı dosya türleri için başka bir disk ya da zaten sahip olduğunuz bir paket klasörü.
locations-s3-title = S3 depolama
locations-s3-hint = s3://bucket-name biçimindeki bir konum, bu hizmetteki bir bucket’tır: AWS ya da S3 uyumlu herhangi bir hizmet (Backblaze B2, Cloudflare R2, Wasabi, MinIO, NAS üzerinde Garage).
locations-s3-endpoint = Adres
locations-s3-endpoint-placeholder = AWS için boş bırakın; örn. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Bölge
locations-s3-access-key = Erişim anahtarı kimliği
locations-s3-secret-key = Gizli erişim anahtarı
locations-s3-headers = Ek üst bilgiler
locations-s3-headers-placeholder = Name=value, her satıra bir tane (örneğin Cloudflare Access için)
locations-s3-kept = Kaydedildi (değiştirmek için yazın)
locations-s3-secrets = Anahtarlar ve üst bilgiler sunucunun ayarlarıyla birlikte saklanır ve bir daha gösterilmez.
locations-s3-secrets-desktop = Anahtarlar ve üst bilgiler bu bilgisayarın anahtar zincirinde saklanır ve bir daha gösterilmez.
locations-s3-saved = S3 depolama kaydedildi
locations-s3-save-failed = S3 depolama kaydedilemedi
locations-add-more = Bir konum ekle…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Sorunlar yüklenemedi
issues-started =
    { $count ->
        [one] Yeniden başlatıldı.
       *[other] { $count } öğe yeniden başlatıldı.
    }
issues-retry-failed = Yeniden denenemedi
issues-dismiss-failed = Yok sayılamadı
issues-dismiss-all-title = { $count } öğenin tümü yok sayılsın mı?
issues-dismiss-all-body = “{ $title }” temizlenir. Yeniden ters giden her şey yeniden listelenir.
issues-dismiss-all = Tümünü yok say
issues-just-now = az önce
issues-minutes-ago = { $mins } dk önce
issues-hours-ago = { $hours } sa önce
issues-loading = Yükleniyor…
issues-none = Ters giden bir şey yok.
issues-action-all = { $action } (tümü)
issues-open-asset = Aç
issues-open-pack = Paketi göster
issues-open-locations = Kütüphane konumları
issues-where-before = Yer:
issues-where-after = { "" }
issues-times = { $count } kez
issues-starting = Başlatılıyor…
issues-dismiss-named = Yok say: { $title }
issues-dismiss = Yok say
issues-show-fewer = Daha az göster
issues-show-all = Tümünü göster ({ $count })
issues-not-listed = { $count } tane daha listelenmedi.

## archive.ts

archive-anyway = Yine de arşivle
archive-failed = Arşivlenemedi: { $error }
archive-undo = Geri al
archive-restore-failed = Geri yüklenemedi: { $error }
archive-count-packs-and-assets = { $packs } paket ve { $assets } varlık
archive-count-packs = { $packs } paket
archive-count-assets = { $assets } varlık
archive-count-nothing = hiçbir şey

## ArchiveView.vue

archive-load-failed = Arşiv yüklenemedi: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Geri yüklendi: { $what }
archive-purged = Kalıcı olarak silindi: { $what }
archive-purge-failed = Silinemedi: { $error }
archive-purge-title = { $name } kalıcı olarak silinsin mi?
archive-purge-pack-body = Paketin { $count } varlığı ve { $size } tutarındaki dosyaları da silinir; başka bir paketin paylaştığı dosyalar hariç. Bu geri alınamaz.
archive-purge = Kalıcı olarak sil
archive-purge-asset-body = Başka bir varlık paylaşmıyorsa dosyası serbest bırakılır. Bu geri alınamaz.
archive-empty-title = Arşiv boşaltılsın mı?
archive-empty-body = { $what } kalıcı olarak silinecek ve dosyaları serbest bırakılacak. Bu geri alınamaz.
archive-empty = Arşivi boşalt
archive-title = Arşiv
archive-intro = Arşivlenen paketler ve varlıklar aramada, sayılarda ve etiket listesinde yer almaz. Onları geri yükleyin ya da yerlerini boşaltmak için kalıcı olarak silin.
archive-loading = Yükleniyor…
archive-is-empty = Arşiv boş
archive-is-empty-hint = Bir paketi ya da varlığı arşivleyin; siz onu geri yükleyene ya da kalıcı olarak silene kadar burada bekler.
archive-packs = Paketler
archive-assets = Varlıklar
archive-asset-count = { $count } varlık
archive-archived = { $when } arşivlendi
archive-from = { $pack } paketinden
archive-restore = Geri yükle
archive-show-more = Daha fazla göster ({ $count } kaldı)

## mine.ts

projects-favorite-add-failed = Favorilerinize eklenemedi: { $error }
projects-favorite-remove-failed = Favorilerinizden çıkarılamadı: { $error }
projects-save-search-failed = Arama kaydedilemedi: { $error }
projects-forget-search-failed = { $name } kaldırılamadı: { $error }

## projects.ts

projects-list-and = { $items } ve { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } tane daha
projects-other = { $count } başka proje
projects-usage-title-one = { $name } bunu kullanıyor
projects-usage-title = { $count } proje bunu kullanıyor
projects-usage-body-one = Şurada kullanılıyor: { $list }. Bir sonraki hoard sync bunları çıkarır.
projects-usage-body = Şuralarda kullanılıyor: { $list }. Her birinin bir sonraki hoard sync işlemi bunları çıkarır.

## uploader.ts

upload-network-error = ağ hatası
upload-gave-up = { $what }: { $error } ({ $attempts } denemeden sonra)
upload-failed-plain = başarısız oldu
upload-already-imported = zaten içe aktarılmış
upload-part-failed = yükleme başarısız oldu ({ $status })
upload-batch = { $count } dosyalık toplu gönderim
upload-batch-failed = toplu gönderim başarısız oldu ({ $status })
upload-finish = yüklemeyi bitirme

## UploadQueue.vue

upload-eta-seconds = { $s } sn kaldı
upload-eta-minutes = { $min } dk kaldı
upload-eta-hours = { $h } sa kaldı
upload-queue = İçe aktarmalar
upload-importing = { $count } paket içe aktarılıyor
upload-bytes-of = { $sent } / { $total }
upload-rate = { $size }/sn
upload-waiting = Bekliyor
upload-unpacking = Yüklendi, şimdi açılıyor
upload-already = Zaten kütüphanede
upload-stopped = Durduruldu
upload-files-of = { $done } / { $total } dosya
upload-resumed = devam edildi, { $size } zaten oradaydı
upload-skipped = Bu dosyaların aynısı zaten “{ $name }” olarak içe aktarılmıştı, bu yüzden atlandı.
upload-error = { $error }. İlerlemeniz korunuyor: yeniden deneyin ya da aynı dosyaları daha sonra yeniden bırakın.
upload-retry = Yeniden dene
upload-import-again = Yine de yeniden içe aktar
upload-dismiss = Kapat
upload-cancel = İptal

## PacksView.vue

packs-rename-failed = Yeniden adlandırılamadı: { $error }
packs-collection-deleted = { $name } koleksiyonu silindi
packs-delete-failed = Silinemedi: { $error }
packs-status-uploading = Yükleniyor (şimdiye kadar gelen dosyalar)
packs-status-ingesting = Açılıyor ve etiketleniyor
packs-status-analyzing = LLM etiketliyor
packs-status-ready = Hazır
packs-status-failed = Başarısız
packs-retag-failed = Yeniden etiketleme başlatılamadı: { $error }
packs-read-files-failed = Paketin dosyaları okunamadı: { $error }
packs-context-load-failed = Paketin mağaza ayrıntıları yüklenemedi: { $error }
packs-undo-store-failed = Mağaza sayfası kaldırılamadı: { $error }
packs-context-save-failed = Mağaza ayrıntıları kaydedilemedi: { $error }
packs-unreal-load-failed = Unreal dönüştürme durumu yüklenemedi: { $error }
packs-unreal-discard-title = Özgün dosyalar silinsin mi?
packs-unreal-discard-body = { $name } içindeki özgün .uasset dosyaları dönüştürüldükten sonra silinir. İçlerindeki yalnızca Unreal’a özgü veriler (örneğin malzeme grafikleri) kalıcı olarak kaybolur.
packs-unreal-discard-confirm = Dönüştür ve sil
packs-unreal-convert-failed = Dönüştürme başlatılamadı: { $error }
packs-unreal-fact-converted = { $done } öğe Unreal’dan dönüştürüldü
packs-unreal-fact-packages = { $left } Unreal paketi
packs-unreal-fact-both = { $done } öğe Unreal’dan dönüştürüldü, { $left } öğe hâlâ paket
packs-thumbs-canceled = İptal edildi ({ $done } / { $total } tamamlanmıştı)
packs-thumbs-drew = { $count } model çizildi
packs-thumbs-result = { $lead }: { $improved } küçük resim değiştirildi.
packs-thumbs-result-failed = { $lead }: { $improved } küçük resim değiştirildi, { $failed } tanesi çizilemedi.
packs-archived = { $name } ve { $count } varlığı arşivlendi
packs-blobs-bad = { $count } dosya son bütünlük denetiminden geçemedi.
packs-collections = Koleksiyonlar
packs-browse-named = Göz at: { $name }
packs-collection-name = Koleksiyon adı
packs-save = Kaydet
packs-cancel = İptal
packs-assets = { $count } varlık
packs-updated = Güncellendi: { $date }
packs-browse = Göz at
packs-collection-download = İndir (.zip)
packs-collection-credits = Emeği geçenler
packs-collection-credits-title = Bu varlıkların geldiği paketleri kimin yaptığı, lisansları ve istedikleri emek satırları
packs-rename = Yeniden adlandır
packs-delete = Sil
packs-packs = Paketler
packs-empty = Henüz paket yok. Eklemek için bu sayfada herhangi bir yere bir zip ya da klasör bırakın.
packs-find = Paket bul
packs-find-placeholder = Ada, mağazaya, lisansa ya da etikete göre paket bulun
packs-sort = Sıra
packs-sort-new = Önce en yeni
packs-sort-old = Önce en eski
packs-sort-name = Ada göre
packs-sort-size = Önce en büyük
packs-sort-assets = Önce en çok varlık içeren
packs-with-kind = İçeriği
packs-state = Durum
packs-state-any = Herhangi bir durum
packs-state-working = İşleniyor
packs-state-failed = Başarısız
packs-state-untagged = Henüz tanımlanmamış
packs-store = Mağaza
packs-store-any = Herhangi bir mağaza
packs-store-from = Kaynak: { $store }
packs-store-none = Mağaza sayfası yok
packs-shown-of = { $shown } / { $total }
packs-clear = Temizle
packs-no-match = Eşleşen paket yok.
packs-clear-all = Aramayı ve filtreleri temizle
packs-pack-name = Paket adı
packs-thumbs-drawing = Küçük resimler bu tarayıcıda çiziliyor…
packs-thumbs-drawing-count = Küçük resimler bu tarayıcıda çiziliyor: { $done } / { $total }…
packs-thumbs-stopped = Durduruldu: { $error }
packs-thumbs-none = Buradaki her modelin zaten tüm dokularıyla çizilmiş bir küçük resmi var.
packs-dismiss = Kapat
packs-license = Lisans: { $license }
packs-license-tier = Lisans: { $license } ({ $tier })
packs-license-not-found = bulunamadı
packs-from-before = Kaynak:
packs-from-after = { "" }
packs-added = Eklendi: { $date }
packs-download = Paketi indir (.zip)
packs-store-details = Mağaza ayrıntıları
packs-more-named = { $name } için diğer eylemler
packs-more = Diğer
packs-rename-more = Yeniden adlandır…
packs-retag-rules = Çevrim dışı etiketleyiciyi yeniden çalıştır
packs-retag-llm = LLM etiketleyiciyi yeniden çalıştır
packs-make-thumbs = Küçük resimleri bu tarayıcıda oluştur
packs-make-thumbs-title = Her modeli tek tek, görünmeyen bir 3B görüntüleyicide açar ve sunucununkinden daha iyi olduğu yerde resmini küçük resim olarak kullanır
packs-make-thumbs-title-desktop = Her modeli tek tek, görünmeyen bir 3B görüntüleyicide açar ve Hoard’un oluşturduğundan daha iyi olduğu yerde resmini küçük resim olarak kullanır
packs-read-files = Paketin dosyalarını yeniden oku
packs-credits = Emeği geçenleri indir
packs-credits-title = CREDITS.md: paketi kimin yaptığı, lisansı ve istediği emek satırı
packs-unreal-open = Unreal içeriğini dönüştür…
packs-archive = Paketi arşivle
packs-unreal-intro = Hoard, Unreal paketlerini kendisi okur ve içindekileri sıradan dosyalar olarak saklar: dokular PNG ya da EXR, sesler WAV, malzemeler JSON olarak. Dönüştürülen dosya, etiketlerini, bağlantılarını ve koleksiyonlarını koruyarak paketin yerini alır.
packs-unreal-converted =
    { $count ->
       *[other] dönüştürüldü
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] dönüştürüldü, { $kept } tanesinin özgün dosyası tutuldu
    }
packs-unreal-pending =
    { $count ->
       *[other] henüz incelenmedi
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] Unreal Editor’ı bekliyor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] yaygın bir biçimi yok
    }
packs-unreal-failed =
    { $count ->
       *[other] başarısız oldu
    }
packs-unreal-reasons = Paketler neden kaldı
packs-unreal-editor-before = Mesh’ler ve animasyonlar Unreal Editor gerektirir. Unreal kurulu bir makinede şunu çalıştırın:
packs-unreal-editor-command = hoard unreal-worker --editor <UnrealEditor-Cmd yolu>
packs-unreal-editor-after = ; bunları FBX olarak dışa aktarır ve sonuçları buraya yükler.
packs-unreal-discard = Dönüştürüldükten sonra özgün .uasset dosyalarını sil
packs-unreal-discard-hint = Yer açar. Yalnızca Unreal’a özgü veriler (örneğin malzeme grafikleri) kaybolur.
packs-unreal-drop = Yaygın bir biçimi olmayan paketleri sil
packs-unreal-drop-hint = Blueprint’ler, seviyeler ve yalnızca motora özgü diğer varlıklar.
packs-unreal-convert = Dönüştür
packs-context-hint-before = Paketin nereden geldiği ve satıcının onun hakkında ne dediği. LLM etiketleyici bunu her dosyayla birlikte okur; bu yüzden "120 ice and fire spells" gibi bir mağaza tanıtımı, şu gibi dosyalara ad vermesine yardımcı olur:
packs-context-hint-after = .
packs-store-auto = Otomatik bulundu: paketin kendisi bu sayfayı belirtiyor.
packs-store-not-this = Bu değil
packs-lookup-label = Mağazalarda aranacak
packs-lookup-placeholder = Paketin mağazadaki adı
packs-lookup-searching = Aranıyor…
packs-lookup-search = Mağazalarda ara
packs-lookup-hide = Gizle
packs-lookup-clues = Paketin dosyalarından: { $clues }.
packs-lookup-also = Ayrıca arananlar: { $queries }.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Fab, itch.io, Kenney ya da OpenGameArt’ta buna benzeyen bir şey yok. Daha az ya da farklı sözcükler deneyin.
packs-lookup-use = Kullan
packs-lookup-searched = Arama tarihi: { $date }.
packs-lookup-again = Yeniden ara
packs-lookup-again-after = .
packs-lookup-unreachable = Bu sefer şunlara ulaşılamadı: { $stores }.
packs-lookup-also-look = Şuralara da bakın:
packs-lookup-links-and = { " " }ve{ " " }
packs-lookup-browser-only = (bunlarda yalnızca tarayıcınızda arama yapılabilir); sayfanın adresini aşağıya yapıştırın.
packs-lookup-open = Mağazalarda bul…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Satıcı
packs-seller-placeholder = Kim yaptı
packs-license-label = Lisans
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Lisans düzeyi
packs-license-tier-placeholder = Kişisel, Profesyonel…
packs-store-page = Mağaza sayfası
packs-store-text = Mağazadaki açıklama
packs-store-text-placeholder = İlan metnini yapıştırın ya da mağaza sayfasından alınması için boş bırakın.
packs-store-text-fetch = Kaydetmek, açıklamayı mağaza sayfasından alır.
packs-readme = Pakette bulunan README
packs-save-retag = Kaydet ve LLM etiketleyiciyi yeniden çalıştır

## AssetDetail.vue

asset-lazy-3d-viewer = 3B görüntüleyici
asset-lazy-text = metin
asset-lazy-text-viewer = metin görüntüleyici
asset-lazy-image-viewer = görsel görüntüleyici
asset-lazy-material-preview = malzeme önizlemesi
asset-lazy-material-viewer = malzeme görüntüleyici
asset-lazy-sprite-player = sprite oynatıcı
asset-lazy-sheet-view = sprite sayfası görüntüleyici
asset-rel-texture = doku
asset-rel-buffer = geometri verisi
asset-rel-material = malzeme dosyası
asset-rel-animation = animasyon
asset-rel-library = bağlantılı dosya
asset-rel-guess = muhtemelen birlikte kullanılır
asset-rel-sidecar = mağaza ayrıntıları
asset-rel-variant = varyant
asset-rel-sprite = sprite
asset-role-map = { $role } haritası
asset-maps-count = { $count } harita
asset-needs = Gerektirdikleri
asset-animations = Animasyonlar
asset-sprites = Sprite’lar
asset-used-by = Kullananlar
asset-loose-sprites = Ayrı dosyalar olarak da
asset-in-sheet = Bir sprite sayfasında
asset-other-formats = Diğer biçimler
asset-shown-as = Kitaplıkta gösterildiği biçim
asset-conversion-failed = dönüştürme başarısız oldu
asset-as-converting = Dönüştürülüyor…
asset-as-converting-percent = Dönüştürülüyor… %{ $percent }
asset-as-queued = Sırada…
asset-download = İndir
asset-as-label = Farklı indir
asset-as-original = { $format } (özgün biçim)
asset-as-pack-files = Paketin kendi dosyaları
asset-as-size = Boyut
asset-as-full-size = Tam boyut
asset-as-up-to = En fazla { $size } px
asset-as-hint = Bir boyut ya da başka bir biçim seçin
asset-meta-size = Boyut
asset-meta-length = Süre
asset-meta-sample-rate = Örnekleme hızı
asset-meta-channels = Kanallar
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bit derinliği
asset-meta-bits = { $bits } bit
asset-meta-level = Seviye
asset-meta-level-value = tepe { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = tepe { $peak } dBFS, RMS { $rms } dBFS, kırpılıyor
asset-meta-loop = Döngü
asset-meta-loop-value = { $start } – { $end }
asset-meta-playback = Çalma
asset-meta-one-shot = Tek seferlik
asset-meta-envelope = Zarf
asset-meta-envelope-times = atak { $attack }, kuyruk { $tail }
asset-meta-envelope-hit = vuruş
asset-meta-envelope-swell = kabaran
asset-meta-envelope-sustained = sürekli
asset-meta-tone = Tını
asset-meta-tone-value = parlaklık { $khz } kHz, gürültü %{ $noise }
asset-meta-pitch = Perde
asset-meta-pitch-rises = { $octaves } oktav yükselir
asset-meta-pitch-falls = { $octaves } oktav alçalır
asset-meta-loops-cleanly = Kesintisiz döngü yapar
asset-meta-tempo = Tempo
asset-meta-root-note = Kök nota
asset-meta-cue-markers = İşaretçiler
asset-meta-library = Kütüphane
asset-meta-keywords = Anahtar sözcükler
asset-meta-title = Başlık
asset-meta-artist = Sanatçı
asset-meta-album = Albüm
asset-meta-genre = Tür
asset-meta-generator = Oluşturucu
asset-meta-everything-with = { $tag } etiketli her şeyi göster
asset-meta-saved-with = Kaydeden
asset-meta-color-mode = Renk modu
asset-meta-layers = Katmanlar
asset-meta-range = Aralık
asset-meta-hdr = HDR (önizleme ton eşlemeli)
asset-meta-format = Biçim
asset-meta-large-document = Büyük belge (PSB)
asset-meta-ai = Yapay zekâ
asset-meta-ai-made = Üretken yapay zekâ ile yapıldı
asset-meta-ai-edited = Üretken yapay zekâ ile düzenlendi
asset-meta-ai-how-metadata = meta veriler
asset-meta-ai-how-generator = oluşturucu ayarları
asset-meta-ai-how-iptc = IPTC dijital kaynak türü
asset-meta-ai-says = { $made } (kaynak: { $how })
asset-meta-ai-by-says = { $made } ({ $by }; kaynak: { $how })
asset-meta-provenance = Köken
asset-meta-c2pa = C2PA İçerik Kimlik Bilgileri var
asset-meta-compression = Sıkıştırma
asset-meta-note = Not
asset-meta-maps = Haritalar
asset-meta-sizes =
    { $count ->
        [one] Boyut
       *[other] Boyutlar
    }
asset-meta-triangles = Üçgenler
asset-meta-vertices = Köşeler
asset-meta-meshes = Mesh’ler
asset-meta-materials = Malzemeler
asset-meta-animations = Animasyonlar
asset-meta-skins = Deriler
asset-meta-unreal-asset = Unreal varlığı
asset-meta-unreal-saved-with = { $class }, UE { $engine } ile kaydedilmiş
asset-meta-package = Paket
asset-meta-cooked = Pişirilmiş (oyun derlemesi): önizlemeler ve kaynaklar çıkarılmış
asset-meta-tiling = Döşeme
asset-meta-seamless = İki eksende de dikişsiz
asset-meta-analysis-error = Bu dosya tam olarak okunamadı: { $error }
asset-copy-blocked = Kopyalanamadı: tarayıcı engelledi
asset-save-error = Kaydedilemedi: { $error }
asset-tag-removed = Kaldırıldı: { $tag }
asset-undo = Geri al
asset-play-error = Bu dosya çalınamadı: { $error }
asset-copied-hash = Özet kopyalandı
asset-copied-toml = toml kopyalandı
asset-copied-id = Kimlik kopyalandı
asset-copied-link = Bağlantı kopyalandı
asset-origin-before = Bu dosyanın özgün hâli:
asset-origin-class-engine = , UE { $engine } ile kaydedilmiş bir { $class }.
asset-origin-class = , bir { $class }.
asset-origin-engine = , UE { $engine } ile kaydedilmiş.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor bunu { $date } tarihinde dışa aktardı.
asset-origin-editor-notes = Unreal Editor bunu { $date } tarihinde dışa aktardı ({ $notes }).
asset-origin-hoard = Hoard bunu { $date } tarihinde paketin içinden okudu.
asset-origin-hoard-notes = Hoard bunu { $date } tarihinde paketin içinden okudu ({ $notes }).
asset-origin-download = Özgün .{ $ext } dosyasını indir
asset-origin-discarded = Özgün paket dönüştürmeden sonra silindi.
asset-details = Varlık ayrıntıları
asset-in-pack = ·
asset-previous = Önceki varlık
asset-next = Sonraki varlık
asset-favorite = Favori
asset-favorite-on-title = Favorilerinizde (F ile çıkarılır)
asset-favorite-off-title = Favorilerinize ekleyin (F)
asset-share = Paylaş
asset-share-title = Bu varlığa bir bağlantı paylaşın (açan kişi önce Hoard’a oturum açar)
asset-layout-panel = Yan panelde göster
asset-layout-page = Tam sayfa göster
asset-layout-page-title = Tam sayfa göster: ayrıntıların yanında büyük bir önizleme
asset-close = Ayrıntıları kapat
asset-rename-title = Yeniden adlandırmak için tıklayın
asset-name = Ad
asset-views-alt = Dört yönden model
asset-inspect-aria = { $name } öğesini tam ekranda incele
asset-inspect-title = İncele: yakınlaştır, döşe, kanallar
asset-playback-position = Çalma konumu
asset-playback-value = { $current } / { $duration }
asset-pause = Duraklat
asset-play = Oynat
asset-waveform-drawing = Dalga biçimi çiziliyor…
asset-waveform-skipped = Bu kadar büyük dosyalar için dalga biçimi yok
asset-play-animation = Animasyon olarak oynat
asset-show-views = Etiketleyicinin gördüğü dört görünümlü sayfayı göster
asset-palette = Resimdeki renkler
asset-swatch-title = { $hex }, resmin %{ $percent } kadarı. Bu renkteki varlıkları bul
asset-swatch-aria = { $hex } rengine sahip varlıkları bul (bu resmin %{ $percent } kadarı)
asset-more-like-this = Buna benzerler
asset-desc-label = Açıklamanız
asset-desc-placeholder = Ne olduğu, nerede kullanacağınız…
asset-save = Kaydet
asset-cancel = İptal
asset-desc-family = { $count } varyantın tümüne kaydedilir
asset-desc-edit = Açıklamayı düzenle
asset-desc-write = Bir açıklama yazın
asset-desc-ai = Yapay zekâ yazdı
asset-desc-file = Dosyadan
asset-desc-store-before = Kaynak:
asset-desc-store-page = { $store } sayfası
asset-desc-store-page-unnamed = mağaza sayfası
asset-desc-store-after = (tüm paket)
asset-less = Daha az
asset-more = Daha fazla
asset-tags = Etiketler
asset-tag-add-label = Etiket ekle
asset-tag-add-placeholder = Bir etiket ekleyin, ör. mood:tense
asset-tag-add = Ekle
asset-apply-family = Etiket, kategori ve açıklama değişikliklerini { $count } varyantın tümüne uygula
asset-collections = Koleksiyonlar
asset-collection-remove = { $name } koleksiyonundan çıkar
asset-collection-add-another = Başka birine ekle
asset-collection-add = Koleksiyona ekle
asset-used-in = Kullanıldığı yerler
asset-used-in-title = { $name } projesinde neler olduğunu göster
asset-used-in-synced = eşitleme: { $when }
asset-maps = Haritalar
asset-show-more = { $count } tane daha göster
asset-materials-heading =
    { $count ->
        [one] Malzeme
       *[other] Malzemeler
    }
asset-material-maps-aria = { $name }: haritalar
asset-variants = Varyantlar ({ $count } adet)
asset-copies = Özdeş kopyalar
asset-copies-note = Aynı baytlar bir kez saklanır; bu paketler de bu dosyayı içeriyor.
asset-unreal-converted = Unreal’dan dönüştürüldü
asset-rule-converted = Bir kuralla dönüştürüldü
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Bir dönüştürme kuralı bu { $format } dosyasını { $date } tarihinde şu dosyadan oluşturdu:
asset-rule-kept-until = Özgün dosya { $date } tarihine kadar tutulur.
asset-rule-put-back = Özgün dosyayı geri koy
asset-rule-put-back-title = { $file } geri konsun mu?
asset-rule-put-back-body = Dönüştürülen dosya silinir ve özgün dosya; etiketleri, koleksiyonları ve bağlantılarıyla birlikte onun yerini alır. Dönüştürme kuralları bundan sonra ona dokunmaz.
asset-rule-put-back-done = { $file } geri geldi
asset-rule-let-go = Özgün dosya, Ayarlar’da belirlenen saklama süresi dolunca silindi.
asset-unreal-package = Unreal paketinden
asset-ue-needs-editor-before = Unreal Editor bekleniyor: { $reason }. Unreal kurulu bir makinede
asset-ue-needs-editor-after = komutunu çalıştırın; dosya FBX olarak geri gelir.
asset-ue-editor-path = UnrealEditor-Cmd yolu
asset-ue-unsupported = Paket olarak tutuldu: { $reason }.
asset-ue-failed = Dönüştürme başarısız oldu: { $reason }
asset-recovered = Özgün .{ $ext } dosyası bu paketin içinde saklanıyordu ve Hoard onu çıkardı.
asset-recovered-download = İndir (.{ $ext })
asset-ue-meshes-before = Mesh’ler ve animasyonlar Unreal’ın kendi biçiminde saklanır. Paketi dönüştürün (paket sayfası, Diğer), ardından FBX elde etmek için bir Unreal Editor kurulumunun yanında
asset-ue-meshes-after = komutunu çalıştırın.
asset-ue-registry-tags = { $count } varlık kayıt defteri etiketi
asset-file = Dosya
asset-bytes = Bayt
asset-source = Kaynak
asset-path = Yol
asset-path-open-folder = { $folder } içinde başka neler olduğunu gör
asset-download-maps =
    { $count ->
        [one] Haritayı indir
       *[other] { $count } haritanın tümünü indir
    }
asset-download-linked = { $count } bağlantılı dosyayla birlikte indir
asset-download-only-title = Yalnızca bu dosya, kullandığı dosyalar olmadan
asset-download-only = Yalnızca bu dosya
asset-include-animations = Animasyonları .zip’e dahil et
asset-copy-toml = hoard.toml girdisini kopyala
asset-copy-id = Kimliği kopyala
asset-teach = Bu dosyayı Hoard’a öğret
asset-teach-title = Bu mağazanın dosyasını, Hoard okuyabilsin diye tanımlayın (Ayarlar → Mağaza dosyaları)
asset-archive = Arşivle
asset-archive-title = Arşiv’den geri yüklenene ya da kalıcı olarak silinene kadar aramada görünmez

## ImageViewer.vue

image-channel-red = Kırmızı kanal
image-channel-green = Yeşil kanal
image-channel-blue = Mavi kanal
image-channel-alpha = Alfa kanalı
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Damalı arka plan (K ile değiştir)
image-bg-black-title = Siyah arka plan (K ile değiştir)
image-bg-gray-title = Gri arka plan (K ile değiştir)
image-bg-white-title = Beyaz arka plan (K ile değiştir)
image-bg-checker = damalı arka plan
image-bg-black = siyah arka plan
image-bg-gray = gri arka plan
image-bg-white = beyaz arka plan
image-inspect-aria = İncele: { $name }
image-preview-size = önizleme { $width }×{ $height }, { $bytes }
image-preview = önizleme
image-resolution = Çözünürlük
image-full-title = { $width }×{ $height } boyutundaki tam resmi yükle (F); önizlemenin ötesine yakınlaştırmak da yükler
image-full-loading = Tam çözünürlük yükleniyor…
image-full-failed = Tam çözünürlük yüklenmedi: yeniden deneyin
image-full = Tam çözünürlük
image-zoom = Yakınlaştırma
image-zoom-out-title = Uzaklaştır (−)
image-zoom-out = Uzaklaştır
image-zoom-in-title = Yakınlaştır (+)
image-zoom-in = Yakınlaştır
image-fit-title = Sığdır (0)
image-fit = Sığdır
image-actual-title = Gerçek pikseller (1)
image-view = Görünüm
image-tile-title = Dikişleri denetlemek için 3×3 döşe (T)
image-tile = Döşe
image-channels = Kanallar
image-close = Kapat (Esc)
image-failed = Bu görsel yüklenemedi.
image-loading = Yükleniyor…
image-key-gestures = Yakınlaştırmak için kaydırın ya da kıstırın · kaydırmak için sürükleyin · yakınlaştırmak ya da sığdırmak için çift tıklayın
image-key-tile = döşe
image-key-background = arka plan
image-key-full = tam çözünürlük
image-key-channels = kanallar

## SpritePlayer.vue

sprite-frame-aria = { $name }, kare { $frame } / { $count }
sprite-failed = Bu oynatılamıyor.
sprite-loading = Kareler yükleniyor…
sprite-previous = Önceki kare
sprite-next = Sonraki kare
sprite-pause = Animasyonu duraklat
sprite-play = Animasyonu oynat
sprite-animation = Animasyon
sprite-all-frames = Tüm kareler
sprite-grid-title = Sayfanın karelere nasıl bölündüğü
sprite-columns = Sütunlar
sprite-rows = Satırlar
sprite-fps-aria = Saniyedeki kare sayısı
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
       *[other] { $count } sprite
    }
sheet-pages =
    { $count ->
        [one] { $count } sayfa
       *[other] { $count } sayfa
    }
sheet-animations =
    { $count ->
        [one] { $count } animasyon
       *[other] { $count } animasyon
    }
# The box that narrows the list of sprites by name.
sheet-find = Sprite bul
# Under the list of sprites when more match than it draws.
sheet-showing = { $total } sprite’tan { $shown } tanesi gösteriliyor
sheet-no-match = Adında bu geçen bir sprite yok.
sheet-failed = Bu sprite sayfası okunamıyor.
sheet-loading = Sprite sayfası yükleniyor…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoard’un kendi biçimi: sayfalar olduğu gibi, sprite’lar her yöne döndürülmüş ve kırpılmış; animasyonlar ve her karenin süresiyle birlikte.
sheet-note-texturepacker = Sprite’lar saat yönünde döndürülmüş ve kırpılmış, animasyonlar adlarıyla birlikte; PixiJS ve Phaser’ın okuduğu biçimde.
sheet-note-clockwise = Sprite’lar saat yönünde döndürülmüş ve kırpılmış. Bu biçim animasyon tutamaz: kareler numaralı adlarıyla sıralanır.
sheet-note-libgdx = Sprite’lar saat yönünün tersine döndürülmüş ve kırpılmış. Bu biçim animasyon tutamaz: kareler numaralı adlarıyla sıralanır.
sheet-note-godot-frames = Sprite’lar hiç döndürülmez, kırpılan kısım kenar boşluğu olarak korunur; animasyonlar ve her karenin süresiyle birlikte.
sheet-note-godot-tileset = Karolar hiç döndürülmez ya da kırpılmaz, karo boyutunda bir ızgaraya dizilir; görseller zaten bir ızgaradaysa olduğu gibi kalır.
sheet-note-grid = Her sprite kendi tuvalinin tamamında dik durur, hepsi aynı boyutta hücrelerde; sayfayı ızgaraya bölen her araç için.
sheet-note-frames = Her sprite kendi tuvalinin tamamında dik durur, her biri kendi adını taşıyan bir PNG olarak.

## TextViewer.vue, TextPreview.vue ve text.ts

text-lang-config = Yapılandırma
text-lang-table = Tablo
text-lang-code = Kod
text-lang-script = Betik
text-lang-text = Metin
text-copy-blocked = Kopyalanamadı: tarayıcı engelledi
text-read-aria = Oku: { $name }
text-lines = { $count } satır
text-show-as = Gösterim
text-tree-title = Katlanabilir ağaç (T)
text-tree = Ağaç
text-pretty-title = Yeniden girintilenmiş (P)
text-pretty = Düzenli
text-raw-title = Yazıldığı gibi (R)
text-raw = Ham
text-document-title = Belge olarak (D)
text-document = Belge
text-source-title = Yazıldığı gibi markdown (R)
text-source = Kaynak
text-fold = Katla
text-expand-all-title = Her şeyi aç
text-expand-all = Tümünü genişlet
text-collapse-all-title = Her şeyi katla
text-collapse-all = Tümünü daralt
text-group-text = Metin
text-wrap-title = Uzun satırları kaydır (W)
text-wrap = Kaydır
text-copy-title = Metni kopyala (C)
text-copied = Kopyalandı
text-copy = Kopyala
text-download-title = Dosyayı indir
text-download = İndir
text-close = Kapat (Esc)
text-truncated = İlk { $shown } gösteriliyor (toplam { $total }).
text-truncated-json = İlk { $shown } gösteriliyor (toplam { $total }; ağaç için dosyanın tamamı gerekir).
text-show-all = Tümünü göster
text-json-error = Bu JSON ayrıştırılamıyor: { $error }
text-contents-aria = { $name } içeriği
text-loading = Yükleniyor…
text-load-error = Bu dosya yüklenemedi: { $error }
text-binary = Bu dosya metin değil, ikili bir dosya; bu yüzden burada okunacak bir şey yok. Kendi aracında açmak için indirin.
text-key-tree = ağaç
text-key-pretty = düzenli
text-key-raw = ham
text-key-document = belge
text-key-source = kaynak
text-key-wrap = kaydır
text-key-copy = kopyala
text-key-find = bul
text-key-close = kapat
text-preview-aria = { $name } öğesini tam ekranda oku
text-preview-read = Oku

## JsonTree.vue ve JsonPicker.vue

json-items = { $count } öğe
json-keys = { $count } anahtar
json-fold = Katla
json-unfold = Aç
json-unfold-summary = Aç: { $summary }
json-show-more = { $count } tane daha göster
json-left = (toplam { $count } kaldı)
json-picker-more = … { $count } tane daha

## 3B model görüntüleyicileri

model-viewer-failed = 3B görüntüleyici yüklenmedi ({ $error }); sayfayı yeniden yüklemek işe yarayabilir
model-loading-viewer = 3B görüntüleyici yükleniyor…
model-loading = Model yükleniyor…
model-show-failed = Bu model gösterilemedi: { $error }
model-preview-failed = Bu modelin önizlemesi gösterilemedi: { $error }
model-error-converting = sunucu bu modeli hâlâ dönüştürüyor
model-error-converting-desktop = Hoard bu modeli hâlâ dönüştürüyor
model-error-empty-collada = boş Collada dosyası
model-error-no-loader = .{ $ext } için tarayıcıda yükleyici yok
model-dialog = 3B olarak görüntüle: { $name }
model-title-stats = { $triangles } üçgen · { $meshes } mesh · { $materials } malzeme
model-triangles = { $count } üçgen
model-group-camera = Kamera
model-group-shading = Gölgelendirme
model-group-background = Arka plan
model-group-animation = Animasyon
model-group-view = Görünüm
model-group-show = Göster
model-frame-title = Modeli çerçevele (F)
model-frame = Çerçevele
model-reset-title = Başlangıç görünümüne dön (R)
model-reset = Sıfırla
model-turn-title = Yavaşça döndür (A)
model-turn = Döndür
model-textures-title = Dokular (T)
model-textures = Dokular
model-full-res-title = Tam çözünürlüklü dokular (H): her doku dosyası ne kadar büyük olursa olsun olduğu gibi; kapalıyken en fazla 2048 px’lik hafif önizlemeler
model-full-res = Tam çöz.
model-light-title = Ortam ışığı (E)
model-light = Işık
model-grid-title = Zemin ızgarası (G)
model-grid = Izgara
model-bg-checker = Damalı arka plan
model-bg-dark = Koyu arka plan
model-bg-gray = Gri arka plan
model-bg-light = Açık arka plan
model-bg-env = Ortam arka planı
model-bg-title = { $background } (K ile değiştir)
model-pause-title = Duraklat (Space)
model-play-title = Oynat (Space)
model-pause = Duraklat
model-play = Oynat
model-animation-clip = Animasyon klibi
model-animation = Animasyon
model-plays-on = Oynatılacak karakter
model-close-title = Kapat (Esc)
model-full-screen = Tam ekran
model-open-full-screen = Tam ekran 3B görüntüleyiciyi aç
model-menu-fold = Menüyü katla
model-menu-show = Menüyü göster
model-view-prefix = Görünüm:{ " " }
model-menu-render = Görüntü
model-menu-maps = Haritalar
model-menu-inspect = İncele
model-mode-final = Son görüntü
model-mode-raw = Ham görüntü
model-mode-color = Temel renk
model-mode-roughness = Pürüzlülük
model-mode-metalness = Metaliklik
model-mode-normal = Normal
model-mode-ao = Kapanma
model-mode-matcap = Matcap
model-mode-uv = UV denetleyici
model-mode-geometry = Geometriler
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Tel kafes (W), her görünümün üzerinde
model-wireframe = Tel kafes
model-info-title = Sahne bilgisi (I)
model-info = Sahne bilgisi
model-info-close = Sahne bilgisini kapat
model-info-close-title = Kapat (I)
model-info-geometries = Geometriler
model-info-materials = Malzemeler
model-info-textures = Dokular
model-info-triangles = Üçgenler
model-info-vertices = Köşeler
model-info-meters = glTF metre cinsindendir
model-info-no-units = Bu biçim birimlerinin ne olduğunu belirtmiyor
model-info-height = Yükseklik
model-info-length = Uzunluk
model-info-width = Genişlik
model-length-units = { NUMBER($n, maximumSignificantDigits: 3) } birim
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Doku bulunamadı: { $name }
model-missing-some = { $count } doku bulunamadı: { $names }
model-missing-more = { $count } doku bulunamadı: { $names }…
model-missing-count = { $count } doku bulunamadı
model-guessed = Dokular ada göre eşleştirildi; dosya hiçbirini belirtmiyor.
model-group-facing = Yön
model-facing-turn = Çeyrek döndür
model-facing-turn-title = Modeli yukarıdan bakıldığında saat yönünün tersine çeyrek tur döndür
model-facing-tip = Çeyrek eğ
model-facing-tip-title = Yatan bir modeli ayağa kaldırmak için modelin üstünü önüne doğru çeyrek tur eğ
model-facing-save = Bu pakette kullan
model-facing-save-title = { $pack } paketindeki tüm modelleri böyle döndür. Küçük resimleri yeniden çizilir; indirmeler oluşturuldukları gibi kalır.
model-facing-undo = Geri al
model-facing-undo-title = Paketin modellerinin döndürülme biçimine geri dön
model-facing-saved = { $pack } paketindeki tüm modeller artık bu yöne bakıyor. Küçük resimleri yeniden çiziliyor.
model-facing-failed = Paketin modelleri döndürülemedi: { $error }
model-keys-mouse = Yörüngede döndürmek için sürükleyin · kaydırmak için sağ tuşla sürükleyin · yakınlaştırmak için tekerleği çevirin · çerçevelemek için çift tıklayın
model-key-frame = çerçevele
model-key-reset = sıfırla
model-key-view = görünüm
model-key-wireframe = tel kafes
model-key-info = sahne bilgisi
model-key-textures = dokular
model-key-full-res = tam çözünürlüklü dokular
model-key-light = ışık
model-key-grid = ızgara
model-key-background = arka plan
model-key-play = oynat

## Malzemeler

material-role-base-color = Renk
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Pürüzlülük
material-role-gloss = Parlaklık
material-role-metalness = Metaliklik
material-role-ao = Ortam kapanması
material-role-orm = Kapanma, pürüzlülük, metal
material-role-mra = Metal, pürüzlülük, kapanma
material-role-rma = Pürüzlülük, metal, kapanma
material-role-height = Yükseklik
material-role-bump = Tümsek
material-role-cavity = Oyuk
material-role-specular = Yansıma
material-role-opacity = Opaklık
material-role-emissive = Işıma
material-role-translucency = Yarı saydamlık
material-role-base-color-name = renk
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = pürüzlülük
material-role-gloss-name = parlaklık
material-role-metalness-name = metaliklik
material-role-ao-name = ortam kapanması
material-role-orm-name = kapanma, pürüzlülük, metal
material-role-mra-name = metal, pürüzlülük, kapanma
material-role-rma-name = pürüzlülük, metal, kapanma
material-role-height-name = yükseklik
material-role-bump-name = tümsek
material-role-cavity-name = oyuk
material-role-specular-name = yansıma
material-role-opacity-name = opaklık
material-role-emissive-name = ışıma
material-role-translucency-name = yarı saydamlık
material-maps = { $count } harita
material-dialog = Malzemeyi görüntüle: { $name }
material-up-to = en fazla { $size }
material-repeat-size = tekrar başına { $size }
material-shape-sphere = Küre
material-shape-cube = Küp
material-shape-plane = Düzlem
material-shape-title = { $shape } ({ $key })
material-group-shape = Şekil
material-group-tiling = Döşeme
material-real-title = Gerçek boyut: bir tekrar { $size }, kürenin çapı 2 m (T ile değiştir)
material-repeat-title = { $n }× tekrarla (T ile değiştir)
material-real = Gerçek
material-repeat = { $n }×
material-height-title = Yükseklik haritasının yüzeyi ne kadar ittiği
material-height = Yükseklik
material-height-strength = Yükseklik gücü
material-loading = Malzeme yükleniyor…
material-show-failed = Bu malzeme gösterilemedi: { $error }
material-preview-failed = Bu malzemenin önizlemesi gösterilemedi: { $error }
material-group-maps = Haritalar
material-map-show = { $map } haritasını göster
material-map-hide = { $map } haritasını gizle
material-map-unused = Önizlemede kullanılmıyor
material-open-full-screen = Tam ekran malzeme görüntüleyiciyi aç
material-keys-mouse = Yörüngede döndürmek için sürükleyin · kaydırmak için sağ tuşla sürükleyin · yakınlaştırmak için tekerleği çevirin · kapatmak için bir haritaya tıklayın
material-key-shape = şekil
material-key-tiling = döşeme
material-key-turn = döndür

## theme.ts

theme-slate = Arduvaz
theme-paper = Kâğıt
theme-linen = Keten
theme-sage = Adaçayı
theme-lilac = Leylak
theme-blush = Allık
theme-graphite = Grafit
theme-obsidian = Obsidyen
theme-ember = Kor
theme-moss = Yosun
theme-aubergine = Patlıcan
theme-oxblood = Bordo
theme-light-slate-blurb = Serin mavi-gri
theme-light-paper-blurb = Sade nötr gri
theme-light-linen-blurb = Sıcak krem, kehribar
theme-light-sage-blurb = Yumuşak yeşil
theme-light-lilac-blurb = Soluk menekşe
theme-light-blush-blurb = Tozlu gül
theme-dark-graphite-blurb = Nötr gri, renk tonu yok
theme-dark-slate-blurb = Mavi-gri
theme-dark-obsidian-blurb = Siyaha yakın, OLED için
theme-dark-ember-blurb = Sıcak kömür, kehribar
theme-dark-moss-blurb = Orman yeşili-siyah
theme-dark-aubergine-blurb = Erik siyahı, leylak
theme-dark-oxblood-blurb = Şarap siyahı, mercan

## texnames.ts

texnames-needs-map = Kalıpta { "{" }map{ "}" } olmalı, yoksa bir malzemenin tüm haritaları aynı adı alır
texnames-placeholders = Kalıp { "{" }name{ "}" }, { "{" }map{ "}" } ve { "{" }size{ "}" } kullanabilir
texnames-bad-characters = Adlarda < > : " / \ | ? ya da * olamaz
texnames-no-suffix = { $role } haritalarına bir sonek verin
texnames-bad-suffix = { $role } haritalarının sonekinde < > : " / \ | ? ya da * olamaz
texnames-slot-no-suffix = “{ $slot }” için bir sonek verin
texnames-slot-bad-suffix = “{ $slot }” için sonekte < > : " / \ | ? ya da * olamaz
texnames-ending-missing = Kendi ad sonlarınızın her biri için aranacak ad sonu gerekir
texnames-ending-bad = Kendi ad sonlarınızda < > : " / \ | ? ya da * olamaz
texnames-ending-twice = { $ending } ad sonlarınızda iki kez var
texnames-slot-metallic-roughness = Metal ve pürüzlülük
texnames-slot-specular-color = Yansıma rengi
texnames-slot-specular-glossiness = Yansıma ve parlaklık
texnames-slot-clearcoat = Şeffaf kaplama
texnames-slot-clearcoat-roughness = Şeffaf kaplama pürüzlülüğü
texnames-slot-clearcoat-normal = Şeffaf kaplama normali
texnames-slot-sheen-color = Parıltı rengi
texnames-slot-sheen-roughness = Parıltı pürüzlülüğü
texnames-slot-transmission = Geçirgenlik
texnames-slot-thickness = Kalınlık
texnames-slot-diffuse-transmission = Dağınık geçirgenlik
texnames-slot-diffuse-transmission-color = Dağınık geçirgenlik rengi
texnames-slot-iridescence = Yanardönerlik
texnames-slot-iridescence-thickness = Yanardönerlik kalınlığı
texnames-slot-anisotropy = Anizotropi

## storeFormats.ts

storefiles-field-title = Başlık
storefiles-field-title-hint = İlanın tam başlığı. Paketi adlandırır; o olmadan dosya atlanır.
storefiles-field-seller = Satıcı
storefiles-field-seller-hint = Onu kim yaptı ya da kim satıyor.
storefiles-field-license = Lisans
storefiles-field-license-hint = Dosyada yazıldığı gibi, örneğin CC-BY-4.0.
storefiles-field-url = Mağaza sayfası
storefiles-field-url-hint = İlanın adresi; http:// ya da https:// ile başlar.
storefiles-field-words = Etiketler
storefiles-field-words-hint = Etiketlemek için sözcükler. Virgülle ayrılmış değerler bölünür.
storefiles-field-type = Tür
storefiles-field-type-hint = Ne olduğu, örneğin “3D model” ya da “surface”. Bu da bir etikettir.
storefiles-field-colors = Renkler
storefiles-field-colors-hint = Renkleri, color: etiketleri olarak.
storefiles-field-id = Mağaza kimliği
storefiles-field-id-hint = Mağazanın ona verdiği kimlik: harfler, rakamlar, _ ya da -.
storefiles-field-origin = Yapıldığı yer
storefiles-field-origin-hint = Açıklama için, nerede yapıldığı ya da tarandığı.
storefiles-field-size = Boyut
storefiles-field-size-hint = Açıklama için, sözcüklerle ne kadar büyük olduğu.
storefiles-field-title-name = başlık
storefiles-field-seller-name = satıcı
storefiles-field-license-name = lisans
storefiles-field-url-name = mağaza sayfası
storefiles-field-words-name = etiketler
storefiles-field-type-name = tür
storefiles-field-colors-name = renkler
storefiles-field-id-name = mağaza kimliği
storefiles-field-origin-name = yapıldığı yer
storefiles-field-size-name = boyut

## StoreFormatsPanel.vue

storefiles-load-failed = Mağaza dosyası biçimleri yüklenemedi: { $error }
storefiles-remove-title = { $name } kaldırılsın mı?
storefiles-remove-body = { $file } veri klasöründen silinir. Bununla zaten okunmuş paketler okunan bilgileri korur.
storefiles-remove = Kaldır
storefiles-removed = Kaldırıldı: { $name }.
storefiles-remove-failed = { $name } kaldırılamadı: { $error }
storefiles-unread = Hoard’un okumadığı dosyalar
storefiles-loading = Yükleniyor…
storefiles-unread-none = Yok: kütüphanedeki hiçbir dosya okunmayan bir mağazanınkine benzemiyor.
storefiles-unread-what = Birden fazla pakette bulunan ya da bir mağazanınki gibi adlandırılmış, hiçbir biçimin okumadığı dosyalar.
storefiles-unread-packs = { $count } paket
storefiles-unread-example = ör. { $pack }
storefiles-teach = Bu dosyayı Hoard’a öğret
storefiles-formats = Biçimler
storefiles-formats-what = Sizinkiler önce denenir ve aynı adlı yerleşik biçimin yerini alır.
storefiles-dir-before = Şurada tutulurlar:
storefiles-dir-after = .
storefiles-yours = Sizin
storefiles-built-in = Yerleşik
storefiles-edit = Düzenle
storefiles-start-from = Bundan başla

## TagsPanel.vue

tags-load-failed = Etiketler yüklenemedi: { $error }
tags-rename-title = { $tag } yeniden adlandırılsın mı?
tags-merge-title = { $count } etiket birleştirilsin mi?
tags-rename-body = Bu etiketi taşıyan { $count } varlık bunun yerine { $into } etiketini alır ve etiketleyiciler bundan sonra yeni adı kullanır.
tags-merge-body = { $tags }, en fazla { $count } varlıkta { $into } olur. Etiketleyiciler bundan sonra { $into } etiketini kullanır.
tags-rename = Yeniden adlandır
tags-merge = Birleştir
tags-merged = { $count } varlık artık { $into } etiketini taşıyor
tags-merge-failed = Birleştirilemedi: { $error }
tags-forget-failed = Bu değiştirilemedi: { $error }
tags-find = Etiket bul
tags-find-placeholder = Etiket bulun, ör. rock ya da set:
tags-count = { $count } etiket
tags-count-more = { $count }+ etiket
tags-what = Aynı anlama gelen etiketleri seçip birleştirin ya da yeniden adlandırmak için birini seçin.
tags-new-name = Yeni ad
tags-merge-into = Şununla birleştir
tags-saved-as-before = Kaydedilen ad:
tags-saved-as-after = { "" }
tags-cancel = İptal
tags-loading = Yükleniyor…
tags-none = Etiket yok.
tags-none-containing = “{ $text }” içeren etiket yok.
tags-browse-title = { $tag } etiketli varlıklara göz at
tags-browse = Göz at
tags-merged-heading = Birleştirilen etiketler
tags-merged-what = Etiketleyiciler bu eski adlardan birini bulduğunda varlık, bu adın birleştirildiği etiketi alır.
tags-allow-old = Eski ada yeniden izin ver

## LibraryStats.vue

stats-source-human = Kişilerin ekledikleri
stats-source-ai = Yapay zekânın önerdikleri
stats-source-rule = Yerleşik kurallar
stats-source-meta = Dosyalardan
stats-assets = Varlıklar
stats-packs = Paketler
stats-on-disk = Diskte
stats-duplicates-title = { $count } dosya diğerlerinin bayt bayt aynı kopyası ve bir kez saklanıyor
stats-saved-dedup = Yinelenenleri ayıklayarak kazanılan
stats-distinct-tags = Farklı etiketler
stats-archive = Arşivde
stats-load-failed = Ayrıntılar yüklenemedi: { $error }
stats-counting = Sayılıyor…
stats-by-kind = Türe göre alan
stats-files-size = { $name }: { $count } dosya, { $size }
stats-months = Son bir yılda eklenenler
stats-month = { $month }: { $count } varlık, { $size }
stats-tagging = Etiketleme
stats-tagged = Etiketli
stats-untagged = { $count } tanesi etiketsiz
stats-described = Açıklamalı
stats-described-assets = { $count } varlık
stats-human-edited = Elle düzenlenen
stats-human-edited-unit =
    { $count ->
       *[other] varlık
    }
stats-source-different = { $count } farklı
stats-most-used = En çok kullanılanlar
stats-show-tag = { $tag } etiketli varlıkları göster
stats-file-types = Dosya türleri
stats-biggest-packs = En büyük paketler
stats-browse-pack = Göz at: { $name }
stats-largest-files = En büyük dosyalar
stats-open-file = Aç: { $name }
stats-empty = Kütüphanede henüz bir şey yok. Bir paket ekleyin, sayıları burada görünür.

## UpdatesPanel.vue

updates-saved = Bir güncelleme hazır olduğunda Hoard bu adresi çağıracak
updates-saved-none = Çağrılacak adres yok
updates-save-failed = Kaydedilemedi
updates-call-answered = Yanıt: { $result }
updates-call-refused = İsteği kabul etmedi: { $result }
updates-call-failed = Çağrılamadı
updates-called = Son çağrı { $when }, sürüm { $version }: { $result }
updates-called-failed = Son çağrı { $when }, sürüm { $version }: { $result }. Hoard bir saat sonra yeniden dener.
updates-latest = Hoard { $version } en son sürüm.
updates-out = Hoard { $version } çıktı.
updates-out-idle = Hoard { $version } çıktı; hiçbir şey yapılmıyorken birkaç dakika içinde çağrılacak.
updates-out-busy = Hoard { $version } çıktı; hiçbir şey yapılmıyor olana kadar bekleniyor ({ $work }).
updates-busy-more = { $count } tane daha
updates-busy-jobs = { $count } iş bekliyor
updates-busy-uploads = { $count } yükleme
updates-list-separator = ,{ " " }
updates-what = Yeni bir sürüm çıktığında Hoard, hiçbir şey yapılmıyorken (yükleme, iş, dönüştürme ya da bakım yokken) bir adresi çağırabilir; böylece konteynerini güncelleyen şey onu bir yüklemenin ortasında yeniden başlatmaz. Sürümü bir JSON gövdesinde taşıyan bir POST gönderir. Bunun için yukarıdaki sürüm denetimi gerekir.
updates-url = Çağrılacak adres
updates-url-placeholder = Yok (örn. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Kaydedildi (değiştirmek için yazın)
updates-auth-placeholder = Yok (örn. Bearer my-token)
updates-remove = Kaldır
updates-save = Kaydet
updates-call-now-title = Bağlı olduğu şey çalışır
updates-call-now = Şimdi çağır
updates-what-calls = Neyi çağırabilir?
updates-watchtower = Watchtower HTTP API’si
updates-watchtower-at = :{ " " }
updates-watchtower-with = , kimlik doğrulama:{ " " }
updates-watchtower-token = { " " }ve şu değişkendeki belirteç:{ " " }
updates-watchtower-its = { " " }({ " " }
updates-watchtower-endpoint = { " " }uç noktası açık olmalı).
updates-portainer = Bir Portainer webhook’u
updates-portainer-rest = { " " }Hoard’un stack’inde ya da konteynerinde, imajı yeniden çekme (re-pull image) açık olarak. Belirteç URL’nin içindedir.
updates-script = Ana makinede bir betik
updates-script-listener = { " " }bir webhook dinleyicisinin arkasında (örneğin{ " " }
updates-script-runs = ); şunu çalıştırır:{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }ya da webhook alabilen başka herhangi bir şey.
updates-poll-before = Ya da adresi atlayın: erişim belirteci olan bir betik şu adresi yoklayabilir:{ " " }
updates-poll-until = ;{ " " }
updates-poll-and = { " " }ayarlandığında ve{ " " }
updates-poll-after = { " " }true olduğunda Hoard’u çekip yeniden başlatabilir.

## MaintenancePanel.vue

maintenance-load-failed = Bakım yüklenemedi
maintenance-reanalyze-title = Her paket yeniden çözümlensin mi?
maintenance-reanalyze-body = Her dosya yeniden okunur; büyük bir kütüphanede bu uzun sürebilir. Paketler sıraları gelene kadar işleniyor olarak görünür. Kendi verdiğiniz adlar, etiketler ve notlar kalır.
maintenance-reanalyze-confirm = Yeniden çözümle
maintenance-gc-title = Kullanılmayan dosyalar silinsin mi?
maintenance-gc-body = Kütüphanede hiçbir şeyin göstermediği saklı dosyalar kalıcı olarak silinir. Arşivdeki paketler kendi dosyalarını korur.
maintenance-gc-confirm = Sil
maintenance-start-failed = “{ $task }” başlatılamadı
maintenance-just-now = az önce
maintenance-minutes-ago = { $count } dk önce
maintenance-hours-ago = { $count } sa önce
maintenance-took-seconds = { $count } sn
maintenance-took-minutes = { $count } dk
maintenance-took-hours = { $hours } sa
maintenance-loading = Yükleniyor…
maintenance-running = Çalışıyor…
maintenance-starting = Başlatılıyor…
maintenance-run = Çalıştır
maintenance-started = { $detail } · başlangıç: { $when }
maintenance-starting-detail = Başlatılıyor
maintenance-finished = Tamamlandı:{ " " }
maintenance-failed = Başarısız:{ " " }
maintenance-never-run = Hiç çalıştırılmadı
maintenance-history = Geçmiş çalıştırmalar
maintenance-col-task = Görev
maintenance-col-finished = Bitiş
maintenance-col-took = Süre
maintenance-col-result = Sonuç
maintenance-script-before = Yedeklemeler ve hasar denetimi bir betikten de çalıştırılabilir:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }ve{ " " }
maintenance-script-after = . Bu çalıştırmalar da burada görünür.

## SettingsPage.vue

settings-title = Ayarlar
settings-loading = Yükleniyor…
settings-load-failed = Ayarlar yüklenemedi: { $error }
settings-page-general = Genel
settings-page-general-blurb = Bu sunucunun, onu kullanan herkes için adı.
settings-page-general-blurb-desktop = Bu kütüphanenin adı.
settings-page-appearance = Görünüm
settings-page-appearance-blurb = Bu tarayıcının açık ve koyu moddayken giydiği temalar. Yalnızca bu tarayıcıda kaydedilir.
settings-page-library = Kütüphane
settings-page-library-blurb = Hoard’da neler var: ne kadar büyük, nelerden oluşuyor, nasıl büyüdü ve ne kadar iyi etiketlenmiş. Arşivlenenler sayılmaz.
settings-page-tags = Etiketler
settings-page-tags-blurb = Kütüphanedeki her etiket. Aynı anlama gelenleri birleştirin ya da birini yeniden adlandırın; etiketleyiciler bundan sonra yeni adı kullanır.
settings-page-store-files = Mağaza dosyaları
settings-page-store-files-blurb = Mağazaların çizimlerin yanına koyduğu, Hoard’un paketlere ad vermek ve yapanlarını anmak için okuduğu dosyalar. Bilmediklerini ona öğretin ve öğrettiklerinizi paylaşın.
settings-page-locations = Kütüphane konumları
settings-page-locations-blurb = Sunucuda kütüphaneyi tutan klasörler: Hoard’un dosya sakladığı klasörler ve paketleri bulundukları yerde dizinlediği klasörler.
settings-page-locations-blurb-desktop = Kütüphaneyi tutan klasörler: Hoard’un dosya sakladığı klasörler ve paketleri bulundukları yerde dizinlediği klasörler.
settings-page-tools = Araçlar
settings-page-tools-blurb = Hoard’un dosyaları dönüştürmek ve okumak için kullandığı dış programlar.
settings-page-ai = Yapay zekâ etiketleme
settings-page-ai-blurb = Yerleşik kuralların üstüne, bir yapay zekâ modeli yeni paketlere ad verir, onları tanımlar ve etiketler. Onun etiketleri önerilen olarak işaretlenir.
settings-page-search = Görsel ve ses arama
settings-page-search-blurb = Resimleri gösterdiklerine, sesleri de neye benzediklerine göre arayın, yalnızca adlarına göre değil; yalnızca renkte değil, türde de benzeyen şeyleri bulun.
settings-page-adding = Paket ekleme
settings-page-adding-blurb = Bir paket eklendiğinde, okuyup etiketlemenin yanı sıra Hoard’un onunla ne yaptığı.
settings-page-downloads = İndirmeler
settings-page-downloads-blurb = Hoard’un gönderdiklerinde dosyaların nasıl adlandırıldığı: indirmeler, zip’ler, hoard aracı ve yapay zekâ asistanları. Kütüphanedeki dosyalar olduğu gibi kalır.
settings-page-account = Hesap
settings-page-account-blurb = Parolanız, oturum açabilen diğer kişiler ve betikler ile diğer programlar için erişim belirteçleri.
settings-page-auth = Kimlik doğrulama
settings-page-auth-blurb = Kişilerin nasıl oturum açtığı: Hoard’un kendi hesapları, bir LDAP dizini, Microsoft Active Directory ya da bir OAuth sağlayıcısı; oturumların ne kadar sürdüğü ve hangi grubun hangi rolü aldığı.
settings-page-connection = Bağlantı
settings-page-connection-blurb = Tarayıcıların ve uygulamaların Hoard’a nasıl ulaştığı: bilinen adresi ve bazı tarayıcı özelliklerinin gerektirdiği HTTPS.
settings-page-performance = Performans
settings-page-performance-blurb = Sunucunun ne kadar yoğun çalıştığı, küçük resimlerin nasıl oluşturulup saklandığı ve alabileceği en büyük boyutlar.
settings-page-performance-blurb-desktop = Hoard’un ne kadar yoğun çalıştığı, küçük resimlerin nasıl oluşturulup saklandığı ve alabileceği en büyük boyutlar.
settings-page-issues = Sorunlar
settings-page-issues-blurb = Dosyalar açılırken ya da işlenirken ters gidenler: açılmayan arşivler, okunamayan dosyalar, başarısız olan işler. Neden giderildikten sonra onları yeniden deneyin ya da yok sayın.
settings-page-maintenance = Bakım
settings-page-maintenance-blurb = Yedekleyin, hasarı denetleyin ve toparlayın. Her birinden aynı anda yalnızca biri çalışır; onlar çalışırken bu sayfadan ayrılabilirsiniz.
settings-the-default = varsayılan
settings-default-on = açık
settings-default-off = kapalı
settings-default-auto = otomatik
settings-option-saved = { $name }: { $value }
settings-back-to-default = Varsayılana dön
settings-save = Kaydet
settings-use = Kullan
settings-status-off = Kapalı

## SettingsPage.vue: Appearance

settings-themes = Temalar
settings-look-light = Açık
settings-look-dark = Koyu
settings-look-group =
    { $scheme ->
        [light] Açık tema
       *[dark] Koyu tema
    }
settings-preview =
    Görebilmeniz için { $preview ->
        [light] açık
       *[dark] koyu
    } tema gösteriliyor. Siz { $used ->
        [light] açık
       *[dark] koyu
    } tema kullanıyorsunuz{ $system ->
        [yes] { " " }(sistemi izleyerek)
       *[other] { "" }
    }; bu sayfadan ayrıldığınızda o geri gelir.
settings-preview-back =
    { $used ->
        [light] Açık
       *[dark] Koyu
    } temaya dön
settings-look-hint = Açık ve koyu arasında (ya da sistemi izlemek için) üst çubuktan geçiş yapın.

## SettingsPage.vue: Tools

settings-blender-what = .blend, .dae, .abc ve .x3d modellerini arka planda glTF’ye dönüştürür; böylece küçük resim ve 3B önizleme alırlar. Kapalıyken eklenen paketler, hazır olduğunda dönüştürülür.
settings-blender-download = Kurulu bir Blender kullanır ya da bir blender.org yansısından Blender 4.5 LTS (yaklaşık 400 MB) indirir.
settings-ffmpeg-what = MP3, OGG ve Opus indirmelerini kodlar ve Hoard’un kendisinin çözemediği sesleri (AAC, M4A, WMA, …) okur.
settings-ffmpeg-download = Doğru kodlayıcılara sahip bir sistem ffmpeg’i kullanır ya da statik bir derleme (yaklaşık 80 MB) indirir.
settings-tool-reset = { $tool }: varsayılana dön
settings-tool-no-download = Kurulu bir program kullanılır: { $tool }. Bu makine için indirilecek bir derleme yok; bu yüzden onu kurun ya da yolunu verin.
settings-tool-path = Program yolu: { $tool }
settings-mode-off = Kapalı
settings-mode-auto = Otomatik
settings-mode-path = Bu program
settings-tool-looking = Aranıyor…
settings-tool-downloading = İndiriliyor… ilerleme etkinlik göstergesinde
settings-tool-ready = Hazır: { $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = MIDI dosyalarının çalınıp dönüştürüldüğü çalgılar.
settings-soundfont-file = SoundFont dosyası
settings-soundfont-placeholder = Sunucuda bulunan ya da yerleşik
settings-soundfont-placeholder-desktop = Bu bilgisayarda bulunan ya da yerleşik
settings-soundfont-default = sunucuda bulunan ya da yerleşik
settings-soundfont-default-desktop = bu bilgisayarda bulunan ya da yerleşik

## SettingsPage.vue: Image search

settings-clip-heading = Resimlerin gösterdiğine göre ara
settings-clip-reset = Görsel arama: varsayılana dön
settings-clip-saved-on = Görsel arama: açık
settings-clip-saved-off = Görsel arama: kapalı
settings-clip-what = Aradığınızı yazın (“mossy rock”, “sci-fi door”, “wooden barrel”); onu gösteren resimler, adında bu sözcükler geçenlerin yanında çıkar. “Buna benzer daha fazla”, yalnızca aynı renkleri değil, aynı türden şeyleri bulur.
settings-clip-toggle = Görsel aramayı aç
settings-clip-hint = OpenAI’nin CLIP modelini kullanır ve bu sunucunun işlemcisinde çalışır: hiçbir şey dışarı çıkmaz. Açmak, modeli ({ $size } MB) Hugging Face’ten indirir, ardından her resmi arka planda düşük öncelikle öğrenir. Çalışırken ya da biri arama yaparken 300 ile 600 MB bellek kullanır. Kapatmak modeli siler, öğrendiklerini bir dahaki sefer için saklar.
settings-clip-hint-desktop = OpenAI’nin CLIP modelini kullanır ve bu bilgisayarın işlemcisinde çalışır: hiçbir şey dışarı çıkmaz. Açmak, modeli ({ $size } MB) Hugging Face’ten indirir, ardından her resmi arka planda düşük öncelikle öğrenir. Çalışırken ya da siz arama yaparken 300 ile 600 MB bellek kullanır. Kapatmak modeli siler, öğrendiklerini bir dahaki sefer için saklar.
settings-clip-looking = Model aranıyor…
settings-clip-downloading = Model indiriliyor… ilerleme etkinlik göstergesinde
settings-clip-ready-empty = Hazır. Resimler, paketler eklendikçe öğrenilir.
settings-clip-ready-all =
    { $count ->
       *[other] Hazır: { $count } resmin tümünü tanıyor
    }
settings-clip-learning = Hazır: resimler öğreniliyor, şimdiye kadar { $done } / { $total }
settings-clip-unavailable = Model alınamadı: { $detail }. Her saat yeniden deneniyor.
settings-clap-heading = Seslerin neye benzediğine göre ara
settings-clap-reset = Ses arama: varsayılana dön
settings-clap-saved-on = Ses arama: açık
settings-clap-saved-off = Ses arama: kapalı
settings-clap-what = Aradığınızı yazın (“distant thunder”, “glassy UI blip”, “footsteps on gravel”); ona benzeyen sesler, adında bu sözcükler geçenlerin yanında çıkar.
settings-clap-toggle = Ses aramayı aç
settings-clap-hint = LAION’un CLAP modelini kullanır ve bu sunucunun işlemcisinde çalışır: hiçbir şey dışarı çıkmaz. Açmak, modeli ({ $size } MB) Hugging Face’ten indirir, ardından her sesin başını arka planda düşük öncelikle dinler. Çalışırken ya da biri arama yaparken yaklaşık 1 GB’a kadar bellek kullanır. Kapatmak modeli siler, öğrendiklerini bir dahaki sefer için saklar.
settings-clap-hint-desktop = LAION’un CLAP modelini kullanır ve bu bilgisayarın işlemcisinde çalışır: hiçbir şey dışarı çıkmaz. Açmak, modeli ({ $size } MB) Hugging Face’ten indirir, ardından her sesin başını arka planda düşük öncelikle dinler. Çalışırken ya da siz arama yaparken yaklaşık 1 GB’a kadar bellek kullanır. Kapatmak modeli siler, öğrendiklerini bir dahaki sefer için saklar.
settings-clap-ready-empty = Hazır. Sesler, paketler eklendikçe öğrenilir.
settings-clap-ready-all =
    { $count ->
       *[other] Hazır: { $count } sesin tümünü tanıyor
    }
settings-clap-learning = Hazır: sesler öğreniliyor, şimdiye kadar { $done } / { $total }

## SettingsPage.vue: AI tagging

settings-ai-service = Hizmet
settings-ai-off = Kapalı
settings-ai-off-hint = Paketler yalnızca yerleşik kurallarla etiketlenir.
settings-ai-local = Yerel model
settings-ai-local-hint = Hoard’un compose dosyasındaki llm hizmeti (ya da cargo xtask llm). Aşağıdan bir model indirin; bu makinede çalışır ve hiçbir şey dışarı çıkmaz.
settings-ai-chatgpt = ChatGPT planı
settings-ai-chatgpt-hint = API anahtarı yerine ChatGPT Go, Plus, Pro ya da Business planınızı kullanır.
settings-ai-openrouter-hint = Bazıları ücretsiz, yüzlerce barındırılan model için tek bir anahtar. Resim alan modeller işaretlenir.
settings-ai-openai-hint = OpenAI tarafından barındırılır.
settings-ai-anthropic-hint = Anthropic tarafından, OpenAI uyumlu uç noktası üzerinden barındırılır.
settings-ai-gemini-hint = Google tarafından, OpenAI uyumlu uç noktası üzerinden barındırılır.
settings-ai-ollama-hint = Sizin çalıştırdığınız bir Ollama sunucusu. ollama adlı bir konteyner değilse adresi değiştirin.
settings-ai-custom = Diğer (OpenAI uyumlu)
settings-ai-custom-hint = OpenAI sohbet API’sini konuşan her şey: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Bağlanıyor…
settings-ai-connected = Bağlandı · { $count } model
settings-ai-connected-none = Bağlandı ama hiç model listelemiyor; modelin adını yazın
settings-chatgpt-connected-none = Bağlandı; modelin adını yazın
settings-ai-takes-images = resim alır
settings-ai-saved = Yapay zekâ etiketleme: { $service }
settings-ai-saved-off = Yapay zekâ etiketleme kapalı
settings-ai-reset = Yapay zekâ etiketleme: varsayılana dön
settings-ai-on = Açık: { $service } · { $model }
settings-ai-url = Adres
settings-ai-key = API anahtarı
settings-ai-key-saved = Kaydedildi (değiştirmek için yazın)
settings-ai-key-paste = Anahtarınızı yapıştırın
settings-ai-key-optional = Yok (isteğe bağlı)
settings-ai-key-remove = Kaldır
settings-ai-get-key = Anahtar al
settings-ai-model = Model
settings-ai-model-pick = Bir model seçin ya da yazın
settings-ai-model-name = Model adı
settings-ai-refresh = Yenile
settings-ai-connect = Bağlan
settings-ai-need-key = Modellerini görmek için API anahtarını ekleyin
settings-ai-vision = Küçük resimleri de gönder (resim alan bir model gerekir)
settings-ai-batch = İstek başına
settings-ai-batch-unit = varlık (küçük yerel modeller için daha az, büyük barındırılan modeller için daha çok)

## SettingsPage.vue: ChatGPT sign-in

settings-chatgpt-account = Hesap
settings-chatgpt-checking = Denetleniyor…
settings-chatgpt-status-as = Oturum açıldı: { $email }
settings-chatgpt-status = Oturum açıldı
settings-chatgpt-signed-in-as = ChatGPT’de oturum açıldı: { $email }
settings-chatgpt-signed-in = ChatGPT’de oturum açıldı
settings-chatgpt-signed-out = ChatGPT oturumu kapatıldı
settings-chatgpt-continue = ChatGPT ile devam et
settings-chatgpt-sign-out = Oturumu kapat
settings-chatgpt-sign-out-title = ChatGPT oturumu kapatılsın mı?
settings-chatgpt-sign-out-in-use = Yapay zekâ etiketleme ChatGPT planınızı kullanıyor; bu yüzden yeniden oturum açana ya da başka bir hizmet seçene kadar durur.
settings-chatgpt-sign-out-body = Hoard oturum bilgisini unutur ve OpenAI’ye oturumu sonlandırmasını söyler.
settings-chatgpt-manage = Kullanımı yönet
settings-chatgpt-finish-before = Oturum açmayı açılan pencerede tamamlayın (
settings-chatgpt-finish-link = yeniden açın
settings-chatgpt-finish-middle = ). Yüklenmeyen bir sayfada biterse ve adres şununla başlıyorsa:{ " " }
settings-chatgpt-finish-after = , o adresi kopyalayıp buraya yapıştırın:
settings-chatgpt-paste = Oturum açmanın bittiği adres
settings-chatgpt-finish = Tamamla
settings-chatgpt-usage = Etiketleme istekleri ChatGPT planınızın kullanımından sayılır. Kullanımı yönet, Hoard’un ne kadar kullandığını gösterir ve sınırlamanıza olanak tanır. Hoard’un ChatGPT bağlantısını ChatGPT tarafında kesmek için chatgpt.com’da Ayarlar → Güvenlik ve oturum açma → ChatGPT ile oturum aç bölümüne gidin.

## SettingsPage.vue: Local models

settings-local-downloading-progress = İndiriliyor: %{ $percent } · { $done } / { $total }
settings-local-downloading = İndiriliyor…
settings-local-incomplete = İndirme durdu
settings-local-failed = Geçen sefer başlatılamadı
settings-local-in-use-running = Kullanımda · çalışıyor
settings-local-in-use = Kullanımda
settings-local-starting = Başlatılıyor…
settings-local-running = Çalışıyor
settings-local-ready-images = Hazır · resim alır
settings-local-ready-text = Hazır · yalnızca metin
settings-local-delete-title = { $name } silinsin mi?
settings-local-delete-in-use = Yapay zekâ etiketleme bu modeli kullanıyor; bu yüzden başka birini seçene kadar durur. Dosyaları model sunucusundan silinir.
settings-local-delete-downloading = İndirme durur ve şimdiye kadar inenler silinir.
settings-local-delete-body = Dosyaları model sunucusundan silinir. Daha sonra yeniden indirebilirsiniz.
settings-local-delete = Sil
settings-local-deleting = Siliniyor…
settings-local-cancel = İptal
settings-local-resume = Sürdür
settings-local-download = İndir
settings-local-start-before = Şu komutla başlatın:{ " " }
settings-local-start-or = { " " }(ya da geliştirme sırasında{ " " }
settings-local-start-developing = ), ardından{ " " }
settings-local-start-retry = yeniden deneyin
settings-local-start-after = . Tek bir model çalıştıran bir sunucu da olur: modelinin adını aşağıya yazın.
settings-local-list = Model sunucusunda
settings-local-none = Henüz hiçbir şey indirilmedi. Aşağıdan birini seçin; sunucuda saklanır ve istediğiniz zaman değiştirebilirsiniz.
settings-local-catalog = İndirilecek modeller
settings-local-size = { $size } GB · { $license }
settings-local-other = Hugging Face’ten başka bir model
settings-local-other-placeholder = Hugging Face’ten başka bir model: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Adding packs

settings-store-pages = Mağaza sayfaları
settings-store-pages-what = Yeni paketler arka planda mağazalarda aranır; böylece mağaza sayfalarını, satıcılarını ve lisanslarını alırlar.
settings-store-lookup = Mağaza sayfalarını otomatik olarak ara
settings-unreal = Unreal paketleri
settings-unreal-reset = Unreal paketleri: varsayılana dön
settings-unreal-what = .uasset dosyalarıyla ne yapılacağı.
settings-conv = Dönüştürme kuralları
settings-conv-what = Hoard’un sakladığı dosyaları tek bir biçime dönüştürün; örneğin tüm OBJ ve FBX dosyalarını GLB’ye. Böylece kütüphanedeki her şey aynı biçimde olur. Dönüştürülen dosya; adı, etiketleri, koleksiyonları ve bağlantılarıyla özgün dosyanın yerini alır. İzlenen klasörlerdeki dosyalar hiçbir zaman değiştirilmez.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Açık
settings-conv-to = Şuna dönüştür
# Heading over the file formats a rule converts.
settings-conv-from = Kaynak biçimler
settings-conv-remove = Kaldır
settings-conv-remove-label = { $n }. kuralı kaldır
settings-conv-taken = Bu biçimi başka bir kural dönüştürüyor
settings-conv-upright = Modelleri dönüştürürken paketin modellerinin gösterildiği gibi dik çevir (Y yukarı, +Z’ye bakan)
settings-conv-add = Kural ekle
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Özgünleri sakla:
settings-conv-keep-days =
    { $count ->
        [one] gün
       *[other] gün
    }
settings-conv-keep-what = O zamana kadar özgün dosya, dönüştürülen dosyanın sayfasından geri konabilir. Dokular dönüştürülen modelin içine kopyalanmaz: model, yanındaki görüntü dosyalarını kullanır. Bir modelin .mtl ya da .bin dosyaları yerinde kalır.
settings-conv-save = Kuralları kaydet
settings-conv-revert = Değişiklikleri geri al
settings-conv-existing = Kütüphanede zaten olan dosyaları dönüştür
settings-conv-saved = Dönüştürme kuralları kaydedildi. Bundan sonra eklenen paketlere uygulanırlar.
settings-conv-converting =
    { $count ->
        [one] Arka planda bir dosya dönüştürülüyor
       *[other] Arka planda { $count } dosya dönüştürülüyor
    }
settings-conv-nothing = Kütüphanede dönüştürülmesi gereken bir şey yok.
settings-conv-ask-title =
    { $count ->
        [one] Kütüphanede zaten olan bir dosya dönüştürülsün mü?
       *[other] Kütüphanede zaten olan { $count } dosya dönüştürülsün mü?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Bir pakette.
       *[other] { $packs } pakette.
    } Her özgün dosya { $days } gün tutulur ve o zamana kadar sayfasından geri konabilir.
settings-conv-ask-busy =
    { $count ->
        [one] Biri hâlâ eklenmekte olan bir pakette; ekleme bitince dönüştürülür.
       *[other] { $count } tanesi hâlâ eklenmekte olan paketlerde; ekleme bitince dönüştürülürler.
    }
settings-conv-ask-list = Dönüştürülecek dosyalar
settings-conv-ask-more = …ve { $count } tane daha
settings-conv-ask-skipped = Olduğu gibi bırakılanlar:
settings-conv-skip-watched =
    { $count ->
        [one] izlenen bir klasördeki bir dosya
       *[other] izlenen klasörlerdeki { $count } dosya
    }
settings-conv-skip-unity =
    { $count ->
        [one] bir Unity projesinin adıyla tanıdığı bir dosya
       *[other] bir Unity projesinin adlarıyla tanıdığı { $count } dosya
    }
settings-conv-skip-used =
    { $count ->
        [one] bir sahnenin ya da başka bir dosyanın adını andığı bir dosya
       *[other] bir sahnenin ya da başka bir dosyanın adını andığı { $count } dosya
    }
settings-conv-skip-beside =
    { $count ->
        [one] dönüştürülmüş kopyası zaten paketinde olan bir dosya
       *[other] dönüştürülmüş kopyaları zaten paketlerinde olan { $count } dosya
    }
settings-conv-skip-blender =
    { $count ->
        [one] Blender’ın henüz okumadığı bir dosya (Ayarlar → Araçlar)
       *[other] Blender’ın henüz okumadığı { $count } dosya (Ayarlar → Araçlar)
    }
settings-conv-ask-new = Kaydet, yalnızca yeni paketler
settings-conv-ask-save-convert =
    { $count ->
        [one] Kaydet ve dönüştür
       *[other] Kaydet ve { $count } dosyayı dönüştür
    }
settings-conv-ask-convert =
    { $count ->
        [one] Dönüştür
       *[other] { $count } dosyayı dönüştür
    }
settings-unreal-off = Olduğu gibi bırak
settings-unreal-off-hint = İstediğinizde bir paketin sayfasından dönüştürün.
settings-unreal-keep = Dönüştür, özgünleri tut
settings-unreal-keep-hint = Dokular, sesler ve malzemeler yaygın biçimlerde çıkar; .uasset de kalır.
settings-unreal-replace = Dönüştür, özgünleri sil
settings-unreal-replace-hint = İçerik çıkarıldıktan sonra yer kazandırır. Mesh’ler yine de Unreal çalışanını bekler.

## SettingsPage.vue: Downloads

settings-tn = Doku adları
settings-tn-saved = Doku adları kaydedildi
settings-tn-reset = Doku adları: varsayılana dön
settings-tn-what = Bir malzemenin haritaları, dosyalar nasıl adlandırılmış olursa olsun, malzemenin adına ve her haritanın ne için olduğuna göre adlandırılarak gönderilir: rock_moss_xeokah0_raw_8K_Normal.png değil, Rock_Normal.png. Kütüphanedeki dosyalara dokunulmaz; bir modelin ya da birlikte gönderilen bir MTL’nin adını verdiği harita o adı korur, böylece model onu yine bulur.
settings-tn-toggle = Doku haritalarını malzemelerine göre adlandır
settings-tn-style = Biçem
settings-tn-style-group = Adlandırma biçemi
settings-tn-descriptive = Açıklayıcı
settings-tn-short = Kısa
settings-tn-custom = Kendi biçeminiz
settings-tn-pattern = Desen
settings-tn-hint-name = { " " }malzemenin adı,{ " " }
settings-tn-hint-map = { " " }haritanın ne için olduğu,{ " " }
settings-tn-hint-size = { " " }set birden fazla boyutta geliyorsa boyutu (desende yoksa sona eklenir).
settings-tn-example = Örnek
settings-tn-suffixes = Her harita türü için son ekler
settings-tn-slots = GLB indirmelerinde, bir modelin içine paketlenmiş ve kendi dosyası olmayan bir doku, malzemenin ve doldurduğu yuvanın adıyla adlandırılır; bu yuvaların da sonekleri vardır:
settings-tn-endings = Kendi ad sonlarınız
settings-tn-endings-what = Bir malzemenin haritalarından biri olmayan bir doku, bunlardan biriyle bitmedikçe kendi adını korur: _diff → _BC ile rock_diff.png, rock_BC.png olarak gönderilir. Ad sonunu atmak için ikincisini boş bırakın.
settings-tn-ending-from = Ad sonu
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Yerine
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Ad sonu ekle
settings-tn-ending-remove = Kaldır
settings-tn-ending-remove-label = { $ending } ad sonunu kaldır

## SettingsPage.vue: General and Connection

settings-name = Ad
settings-name-what = Bu sunucunun üst çubukta, tarayıcı sekmesinde ve oturum açma sayfasında görünen adı; böylece birden fazla Hoard varsa hangisinin hangisi olduğunu anlayabilirsiniz.
settings-name-what-desktop = Bu kütüphanenin üst çubukta görünen adı.
settings-address = Adres
settings-address-what = Kişilerin ve uygulamaların Hoard’a ulaşmak için kullandığı adres, bu tarayıcının kullandığından farklıysa: bir ters vekil sunucunun, bir tünelin adresi ya da bir tailnet adı, örneğin https://hoard.example.com. Hoard’un dağıttığı bağlantılar (Claude’a ve diğer uygulamalara, oturum açmada, sertifikasında) bunu kullanır.
settings-public-url = Genel adres
settings-public-url-placeholder = Tarayıcının adresinden (örn. https://hoard.example.com)
settings-public-url-default = tarayıcının adresinden

## ProfilerPanel.vue: the profiler at the top of Settings → Performance

profiler = Profil oluşturucu
profiler-what = Hoard’un son on dakikada yaptıkları; siz izlerken iki saniyede bir güncellenir. Yalnızca bellekte tutulur ve Hoard yeniden başladığında sıfırlanır.
profiler-starting = İlk ölçümler birazdan gelecek.
profiler-failed = Profil oluşturucu okunamadı: { $error }
profiler-cpu = İşlemci kullanımı
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Tüm makine
profiler-cpu-summary = Hoard işlemcilerin { $hoard } kadarını, tüm makine ise { $machine } kadarını kullanıyor.
profiler-memory = Bellek
profiler-serving = Süren istekler
profiler-working = Üzerinde çalışılan
profiler-working-none = Arka planda hiçbir şey çalışmadı.
profiler-working-more =
    { $count ->
        [one] Ve bir tane daha.
       *[other] Ve { $count } tane daha.
    }
profiler-requests = İstekleri karşılamaya harcanan süre
profiler-requests-what = Her çubuk on saniyelik isteği gösterir; süreleri nereye gittiğine göre bölünmüştür.
profiler-part-db = Veritabanı
profiler-part-db-title = Veritabanını bekleme ve sorgularını çalıştırma
profiler-part-disk = Disk
profiler-part-disk-title = Depolanan dosyaları okuma ve gönderileni sıkıştırma
profiler-part-send = Gönderme
profiler-part-send-title = Ağın gönderileni almasını bekleme
profiler-part-other = Diğer
profiler-part-other-title = Yanıtı hazırlama
profiler-request-count =
    { $count ->
        [one] { $count } istek
       *[other] { $count } istek
    }
profiler-requests-none = Son on dakikada istek yok.
profiler-slowest = En yavaş: { $route }, { $time }
profiler-routes = Rotaya göre
profiler-route = Rota
profiler-count = İstekler
profiler-average = Ortalama
profiler-p95 = En yavaş %5
profiler-split = Süre nereye gitti
profiler-sent = Gönderilen
profiler-portal = arayüzün kendi dosyaları

## SettingsPage.vue: Performance

settings-render = 3B küçük resimler
settings-render-toggle = 3B modeller için küçük resim çiz
settings-gpu-on = GPU’da: { $renderer }
settings-gpu-software = GPU bulunamadı, bu yüzden yazılımla ({ $renderer })
settings-gpu-none = GPU ya da yazılım çizici bulunamadı, bu yüzden modeller küçük resim almaz
settings-thumbs = Küçük resimler
settings-thumbs-what = Yeni küçük resimlerin ve model görünümlerinin nasıl saklandığı. Saydamlık içerenler ve piksel sanatı her zaman PNG olur. Mevcut olanları küçültmek için Bakım bölümünde Küçük resimleri küçült görevini çalıştırın.
settings-previews-size = Önizlemeler şu anda { $size } yer kaplıyor.
settings-thumbs-saved = Küçük resimler: { $quality }
settings-thumbs-small = Daha küçük
settings-thumbs-small-hint = JPEG, kalite 70. Izgarada iyi; yakından biraz yumuşak.
settings-thumbs-balanced = Dengeli
settings-thumbs-balanced-hint = JPEG, kalite 82. Küçük resim boyutunda özgününden ayırt etmek zor.
settings-thumbs-sharp = Daha keskin
settings-thumbs-sharp-hint = JPEG, kalite 92. Dengeli’nin yaklaşık iki katı boyutta.
settings-thumbs-lossless = Kayıpsız
settings-thumbs-lossless-hint = PNG, tam oluşturulduğu gibi. Birkaç kat daha büyük.
settings-motion-thumbs = Animasyonlar için hareketli küçük resimler
settings-motion-thumbs-what = Her animasyon ayrıca kutucuğunun üzerine geldiğinizde oynayan kısa, döngülü bir film de alır. Her biri yaklaşık 40 KB yer kaplar ve büyük bir kütüphanede oluşturulması biraz sürer. Kapatmak onları siler.
settings-limits = İş ve sınırlar
settings-limits-what = Her biri alandan ayrıldığınızda ya da Enter’a bastığınızda kaydedilir. Boyutlar birim alır: 100 GB, 512 MB, 1.5 TB; birimsiz bir sayı GB’dir.
settings-workers = Arka plan çalışanları
settings-workers-unit = aynı anda iş (içe alma, etiketleme, dönüştürmeler)
settings-max-upload = En büyük yükleme
settings-max-upload-unit = dosya başına
settings-max-expand = En büyük paket
settings-max-expand-unit = açıldıktan sonra
settings-conversion-cache = Dönüştürülmüş indirmeler
settings-conversion-cache-unit-now = en fazla bu kadar saklanır, şu anda { $size }; en uzun süredir kullanılmayanlar önce gider
settings-conversion-cache-unit = en fazla bu kadar saklanır; en uzun süredir kullanılmayanlar önce gider

## SettingsPage.vue: Maintenance

settings-updates = Güncellemeler
settings-version = Bu, Hoard { $version }.
settings-update-check = Yeni bir sürüm için birkaç saatte bir GitHub’ı denetle
settings-desktop-updates = Hoard her açılışta yeni bir sürüm arar, arka planda indirir ve çıktığınızda kurar.

## Asset kinds (api.ts)

api-kind-texture = Dokular
api-kind-vector = Vektör çizimler
api-kind-sheet = Sprite sayfaları
api-kind-audio = Ses
api-kind-model = Modeller
api-kind-animation = Animasyonlar
api-kind-font = Yazı tipleri
api-kind-shader = Gölgelendiriciler
api-kind-material = Malzemeler
api-kind-video = Video
api-kind-document = Belgeler
api-kind-data = Veri
api-kind-other = Diğer
api-kind-texture-name = dokular
api-kind-vector-name = vektör çizimler
api-kind-sheet-name = sprite sayfaları
api-kind-audio-name = ses
api-kind-model-name = modeller
api-kind-animation-name = animasyonlar
api-kind-font-name = yazı tipleri
api-kind-shader-name = gölgelendiriciler
api-kind-material-name = malzemeler
api-kind-video-name = video
api-kind-document-name = belgeler
api-kind-data-name = veri
api-kind-other-name = diğer

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Geri al
ui-today = bugün
ui-yesterday = dün
ui-days-ago = { $days } gün önce
ui-notifications = Bildirimler
ui-dismiss = Yok say
ui-cancel = İptal
ui-lazy-loading = Yükleniyor: { $what }…
ui-lazy-failed = Yüklenemedi: { $what }.
ui-lazy-reload = Sayfayı yeniden yükle

## ResetButton.vue

reset-title = Varsayılana sıfırla: { $value }
reset-label = Varsayılana sıfırla, { $value }

## SizeInput.vue

size-invalid = 100 GB, 512 MB ya da 1.5 TB gibi bir boyut
size-hint = 100 GB, 512 MB ya da 1.5 TB gibi bir boyut; birimsiz bir sayı GB’dir

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = Kimlik doğrulama ayarları yüklenemedi: { $error }
auth-saved = Kaydedildi. Bir sonraki oturum açmada geçerli olur.
auth-method-internal = Dahili
auth-method-internal-hint = Hoard’un tuttuğu, Ayarlar → Hesap bölümünden eklenen adlar ve parolalar.
auth-method-ldap-hint = Oturum açma sayfasında yazılan adı ve parolayı bir dizin denetler: OpenLDAP, FreeIPA, LLDAP ya da başka bir LDAP sunucusu.
auth-method-ad-hint = Windows etki alanı hesapları her zamanki adları ve parolalarıyla oturum açar.
auth-method-oauth-hint = Google, Microsoft, GitHub, Keycloak, Authentik ya da başka bir sağlayıcıya gidip geri dönen bir “… ile oturum aç” düğmesi.
auth-session-unused = 30 gün kullanılmadıktan sonra
auth-session-1h = Oturum açtıktan 1 saat sonra
auth-session-8h = Oturum açtıktan 8 saat sonra
auth-session-day = Oturum açtıktan bir gün sonra
auth-session-week = Oturum açtıktan bir hafta sonra
auth-session-30d = Oturum açtıktan 30 gün sonra
auth-session-hours = Oturum açtıktan { $hours } saat sonra
auth-role-view = Bakabilir
auth-role-edit = Değiştirebilir
auth-role-admin = Yönetici
auth-role-word-view = bakabilir
auth-role-word-edit = değiştirebilir
auth-role-word-admin = yönetici
auth-role-word-default = { $role } (varsayılan rol)
auth-tls-starttls-hint = Herhangi bir şey gönderilmeden önce TLS’ye geçen düz LDAP (bağlantı noktası 389).
auth-tls-ldaps-hint = Baştan itibaren TLS (bağlantı noktası 636).
auth-tls-none = Yok
auth-tls-none-hint = Parolalar ağda olduğu gibi gider. Yalnızca güvendiğiniz bir ağda kullanın.
auth-test-failed = Sınanamadı: { $error }
auth-preset-custom = Diğer (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel ve diğerleri: yayımcıyı girin, Keşfet adresleri doldursun.
auth-preset-google-note = Google Cloud Console → APIs & Services → Credentials bölümünde bir OAuth istemcisi (Web application) oluşturun. Google grup göndermez; bu yüzden rolleri Ayarlar → Hesap bölümünden verin ve Google hesabı olan herkesin girmesini istemiyorsanız ilk oturum açmada hesap oluşturmayı kapatın.
auth-preset-microsoft-note = Entra ID → App registrations bölümünde bir uygulama kaydedin (aşağıdaki adrese bir Web yönlendirmesiyle). Yalnızca kuruluşunuzun girmesi için “organizations” yerine kiracı kimliğinizi yazın. Rolleri gruplardan almak için Token configuration altında groups talebini ekleyin: grup kimliklerini listeler.
auth-preset-github-note = GitHub → Settings → Developer settings bölümünde bir OAuth uygulaması oluşturun. GitHub grup göndermez; bu yüzden rolleri Ayarlar → Hesap bölümünden verin ve GitHub hesabı olan herkesin girmesini istemiyorsanız ilk oturum açmada hesap oluşturmayı kapatın.
auth-preset-gitlab-note = GitLab → Preferences → Applications bölümünde bir uygulama oluşturun (ya da sunucunuzun yönetici alanında: adreslerdeki gitlab.com’u değiştirin).
auth-discovered = Sağlayıcının adresleri dolduruldu.
auth-copied = Kopyalandı
auth-copy-failed = Kopyalanamadı: seçip kendiniz kopyalayın
auth-sign-in-with = Oturum açma yöntemi
auth-own-accounts-still-work = Hoard’un kendi hesapları yine kendi parolalarıyla oturum açar; böylece çalışmayan ya da yanlış kurulmuş bir dizin ya da sağlayıcı sizi dışarıda bırakamaz.
auth-sessions = Oturumlar
auth-sign-out-after = Kişilerin oturumunu kapat
auth-ldap-server = LDAP sunucusu
auth-server-type = Sunucu türü
auth-server-type-custom = Özel
auth-servers = Sunucular
auth-server-n = Sunucu { $n }
auth-remove-server-n = { $n }. sunucuyu kaldır
auth-remove = Kaldır
auth-add-server = Sunucu ekle
auth-servers-hint = Biri erişilemez olduğunda sıradaki denenir.
auth-security = Güvenlik
auth-connection-security = Bağlantı güvenliği
auth-skip-verify = Sunucunun sertifikasını denetleme (yalnızca denemek için)
auth-ca = Sertifika yetkilisi
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (boş: sistemin güvendikleri)
auth-lookup = Kişileri arama
auth-anonymously = Anonim olarak
auth-reader-account = Okuyucu hesabı
auth-reader-password = Parolası
auth-saved-type-to-replace = Kaydedildi: değiştirmek için yazın
auth-people-under = Kişilerin bulunduğu yer
auth-name-attribute = Ad özniteliği
auth-only-matching = Yalnızca eşleşen girdiler
auth-groups-under = Grupların bulunduğu yer
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (boş: yalnızca memberOf)
auth-their-groups = Grupları
auth-group-filter-hint = { "{" }dn{ "}" } kişinin girdisi, { "{" }user{ "}" } oturum açtığı ad.
auth-test = Sına
auth-test-hint = Her sunucuya ulaşır ve okuyucu hesabıyla oturum açar. Bir ad ve parola verilirse onunla da oturum açar ve Hoard’un bundan ne çıkaracağını gösterir. Hiçbir şey kaydedilmez.
auth-test-user = Oturum açılacak kişi
auth-test-user-placeholder = Oturum açılacak kişi (isteğe bağlı)
auth-test-password = Parolası
auth-testing = Sınanıyor…
auth-test-connectivity = Bağlantıyı sına
auth-test-server-ok = ulaşıldı ve okuyucu hesabı oturum açtı
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }oturum açtı (
auth-test-signed-in-after = ).
auth-test-role = Rolü şu olurdu: { $role }.
auth-test-no-role = Roller altında listelenen grupların hiçbirinde değil; bu yüzden geri çevrilirdi.
auth-test-groups = Gruplar: { $groups }
auth-test-no-groups = Grup bulunamadı.
auth-provider = Sağlayıcı
auth-redirect = Yönlendirme adresi
auth-copy = Kopyala
auth-redirect-hint = Hoard’un istemcisini oluştururken bunu sağlayıcıya verin. Ayarlar → Bağlantı bölümündeki genel adresten ya da kullandığınız adresten gelir.
auth-issuer = Yayımcı
auth-discover = Keşfet
auth-client-id = İstemci kimliği
auth-client-secret = İstemci gizli anahtarı
auth-authorization-url = Yetkilendirme adresi
auth-token-url = Belirteç adresi
auth-userinfo-url = Kullanıcı bilgisi adresi
auth-userinfo-placeholder = (boş: kim olduğunu kimlik belirteci söyler)
auth-scopes = Kapsamlar
auth-name-from = Adın alındığı yer
auth-groups-from = Grupların alındığı yer
auth-groups-from-placeholder = (yok)
auth-button = Düğme
auth-button-placeholder = hesabınız
auth-accounts = Hesaplar
auth-provision = Biri ilk kez oturum açtığında bir Hoard hesabı oluştur
auth-provision-on-hint = Aksi halde bir yönetici her kişiyi önce Ayarlar → Hesap bölümünden ekler.
auth-provision-off-hint = Bir yönetici her kişiyi önce Ayarlar → Hesap bölümünden, nasıl oturum açacağını seçerek ekler.
auth-open-to-anyone = { $provider } hesabı olan herkes girebilir. Aşağıya görüntüleyici grupları ekleyin ya da bunu kapatıp kişileri kendiniz ekleyin.
auth-new-accounts = Yeni hesaplar
auth-roles-from-groups = Rolleri gruplardan al
auth-roles-hint = Üyeleri her rolü alan gruplar; her satıra bir tane, adıyla ya da DN’siyle. Eşleşen en yüksek rol geçerli olur ve her oturum açmada denetlenir. Hiçbiri listelenmezse yeni hesaplar yukarıdaki rolü alır ve sonra bir yönetici değiştirir. Bakabilir altında gruplar listelenmişse yalnızca listelenen bir grubun üyeleri oturum açabilir.
auth-roles-hint-oauth = Üyeleri her rolü alan gruplar; her satıra bir tane, adıyla ya da DN’siyle (ya da groups talebi ne listeliyorsa). Eşleşen en yüksek rol geçerli olur ve her oturum açmada denetlenir. Hiçbiri listelenmezse yeni hesaplar yukarıdaki rolü alır ve sonra bir yönetici değiştirir. Bakabilir altında gruplar listelenmişse yalnızca listelenen bir grubun üyeleri oturum açabilir.
auth-saving = Kaydediliyor…
auth-save = Kaydet

## HttpsPanel.vue (Settings → Connection)

https-load-failed = HTTPS yüklenemedi: { $error }
https-off-redirecting = HTTPS kapalı. http:// adresine götürülüyorsunuz
https-now-on = HTTPS açık
https-saved = Kaydedildi. Yeni bağlantılar yeni sertifikayı kullanır.
https-now-off = HTTPS kapalı
https-http3-failed-toast = HTTP/3 başlatılamadı: { $error }
https-copied = Kopyalandı
https-copy-failed = Kopyalanamadı: seçip kendiniz kopyalayın
https-mode-off = Kapalı
https-mode-off-hint = Yalnızca düz http://.
https-mode-own = Hoard’un kendi sertifikası
https-mode-own-hint = Hoard bu sunucunun adresleri için bir tane oluşturur. Hoard’un sertifika yetkilisini her cihaza bir kez kurun, o cihazın tarayıcıları ona güvensin.
https-mode-custom = Kendi sertifikam
https-mode-custom-hint = Zaten sahip olduğunuz bir sertifika, örneğin tailscale cert ya da Let’s Encrypt’ten. Tarayıcılar hiçbir şey kurmadan ona güvenir.
https-untrusted-own = Bu tarayıcı { $host } için ona henüz güvenmiyor: aşağıdan Hoard’un sertifika yetkilisini kurun. Geçip gidebileceğiniz bir uyarı göreceksiniz.
https-untrusted-uncovered = Bu tarayıcı { $host } için ona henüz güvenmiyor: sertifika bu adresi kapsamıyor. Geçip gidebileceğiniz bir uyarı göreceksiniz.
https-untrusted = Bu tarayıcı { $host } için ona henüz güvenmiyor. Geçip gidebileceğiniz bir uyarı göreceksiniz.
https-now = Şu anda
https-start-failed = HTTPS ayarlandı ama başlatılamadı: { $error }
https-on-port = HTTPS açık; http:// ile aynı bağlantı noktasında: { $port }.
https-using-it = Şu anda onu kullanıyorsunuz.
https-open = Aç: { $url }
https-trusted = Bu tarayıcı ona güveniyor.
https-cert-for-before = Sertifikanın kapsadığı adresler:{ " " }
https-cert-for-after = ; { $date } tarihine kadar geçerli.
https-cert-for-after-own = ; { $date } tarihine kadar geçerli (kendiliğinden yenilenir).
https-not-covered-before = Kullandığınız adresi kapsamıyor:{ " " }
https-not-covered-after = .
https-is-off = HTTPS kapalı: sayfalar düz http:// üzerinden sunuluyor.
https-addresses = Adresler
https-addresses-hint = Sertifika, Hoard’un bulduğu adresleri ve sizin eklediklerinizi (yönlendiricinizin ona verdiği bir ad, bir Tailscale adı) kapsar.
https-remove-name = Kaldır: { $name }
https-another-address = Başka bir adres
https-another-address-placeholder = Başka bir adres, örn. nas.local
https-add = Ekle
https-your-cert = Sertifikanız
https-saved-cert-before = Kaydedildi:{ " " }
https-saved-cert-after = { " " }· veren: { $issuer }, { $date } tarihine kadar geçerli. Değiştirmek için yenisini yapıştırın.
https-pem-before = Sertifika (zinciriyle birlikte) ve özel anahtarı, PEM metni olarak; yani şununla başlayan dosyalar:{ " " }
https-pem-after = .
https-certificate = Sertifika
https-choose-file = Dosya seç…
https-private-key = Özel anahtar
https-checked-before = Kapsadığı adresler:{ " " }
https-checked-after = ; veren: { $issuer }, { $date } tarihine kadar geçerli.
https-checked-after-days = ; veren: { $issuer }, { $date } tarihine kadar geçerli ({ $days } gün).
https-self-signed = Kendinden imzalı: tarayıcılar bu konuda uyarır.
https-key-matches = Anahtar eşleşiyor.
https-where-to-get = Nereden alınır
https-get-tailscale-before = tailnet’iniz için HTTPS’yi açın, ardından sunucuda şunu çalıştırın:{ " " }
https-get-tailscale-after = { " " }ve iki dosyayı yükleyin. 90 gün geçerlidir: süre dolmadan yeniden çalıştırıp yeniden yükleyin.
https-get-acme-title = Let’s Encrypt ya da başka bir ACME istemcisi
https-get-acme-before = (certbot, acme.sh, lego), sahip olduğunuz bir alan adı için: şu iki dosyayı yükleyin:{ " " }
https-get-acme-and = { " " }ve{ " " }
https-get-acme-after = .
https-get-proxy-title = Bir ters vekil sunucu
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) sertifikaya sahipse: burada HTTPS’yi kapalı bırakın, vekil sunucu Hoard’a düz http:// göndersin.
https-trying = Deneniyor…
https-apply = Uygula
https-apply-hint = Hoard önce yeni yapılandırmayı dener, işe yaramazsa elindekini korur.
https-trust-title = Cihazlarınızda Hoard’a güvenin
https-download-ca = hoard-ca.crt dosyasını indir
https-trust-hint = Hoard’un sertifika yetkilisini her cihaza bir kez kurun; o cihazın tarayıcıları bu sunucuya (ve Hoard’un daha sonra onun için oluşturacağı her sertifikaya) güvenir. Yalnızca bu Hoard için kefil olabilir. Parmak izi (SHA-256):
https-ca-windows = Dosyayı açın → Sertifika Yükle → Geçerli Kullanıcı → Tüm sertifikaları aşağıdaki depolama alanına yerleştir → Gözat → Güvenilen Kök Sertifika Yetkilileri → Son. Tarayıcıyı yeniden başlatın.
https-ca-macos = Dosyayı açın; Anahtar Zinciri Erişimi’ne eklenir. Orada “{ $subject }” öğesine çift tıklayın → Güven → Bu sertifikayı kullanırken: Her Zaman Güven.
https-ca-ios-title = iPhone ve iPad
https-ca-ios = Safari’de indirin ve izin verin. Ayarlar → Profil İndirildi → Yükle. Ardından Ayarlar → Genel → Hakkında → Sertifika Güven Ayarları → “{ $subject }” anahtarını açın.
https-ca-android = Ayarlar → Güvenlik → Diğer güvenlik ayarları → Şifreleme ve kimlik bilgileri → Sertifika yükle → CA sertifikası’na gidin ve dosyayı seçin. (Adlar telefondan telefona biraz değişir; Ayarlar’da “sertifika” diye arayın.)
https-ca-linux-1 = Debian ve Ubuntu:{ " " }
https-ca-linux-2 = . Fedora ve Arch:{ " " }
https-ca-linux-3 = . Linux’ta Chrome kendi listesini tutar: Ayarlar → Gizlilik ve güvenlik → Güvenlik → Sertifikaları yönet → İçe aktar.
https-ca-firefox = Firefox kendi listesini tutar: Ayarlar → Gizlilik ve Güvenlik → Sertifikalar → Sertifikaları göster → Yetkili makamlar → İçe aktar; ardından “Bu sertifika makamını web sitelerini tanımlamak için güvenilir say” kutusunu işaretleyin.
https-ca-cli-title = hoard komut satırı aracı
https-ca-cli-before = { "" }
https-ca-cli-middle = ; ya da yolu şu değişkene yazın:{ " " }
https-ca-cli-after = .
https-options = Seçenekler
https-redirect = Diğer cihazlardaki tarayıcıları http:// adresinden https:// adresine gönder
https-redirect-hint = Yalnızca sayfa yüklemeleri yönlendirilir. hoard komut satırı aracı, Unreal çalışanı, betikler ve yapay zekâ uygulamaları http:// üzerinde çalışmaya devam eder; sunucunun kendisindeki http://localhost da öyle.
https-http3-hint-before = Kararsız Wi-Fi’de ve telefonlarda daha hızlı. HTTPS gerektirir; TCP’nin yanı sıra UDP bağlantı noktası da açık olmalı: { $port } (Docker’da:{ " " }
https-http3-hint-after = ).
https-http3-running = Çalışıyor.
https-http3-waits = HTTPS açıldığında başlar.
https-http3-failed = HTTP/3 başlatılamadı: { $error }
https-lockout-title = Dışarıda kalırsanız
https-lockout-1 = { "" }
https-lockout-2 = { " " }sunucunun kendisinde her zaman düz http:// üzerinden çalışır. Ya da HTTPS’yi sunucunun komut satırından kapatıp Hoard’u yeniden başlatın:{ " " }
https-lockout-3 = { " " }(Docker’da:{ " " }
https-lockout-4 = , ardından{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Bakabilir
account-role-view-hint = Arama, önizleme ve indirme
account-role-edit = Değiştirebilir
account-role-edit-hint = Ayrıca ekleme, değiştirme, etiketleme, koleksiyona katma ve arşivleme
account-role-admin = Yönetici
account-role-admin-hint = Ayrıca Ayarlar, hesaplar ve bakım ile kalıcı silme
account-source-internal = Bir Hoard parolası
account-source-ldap = Kendi { $directory } parolası
account-source-oauth = { $provider } ile oturum açma
account-source-word-directory = dizin
account-load-failed = Hesaplar yüklenemedi: { $error }
account-failed = { $what }: { $error }
account-password-failed = Parolanız değiştirilemedi
account-password-changed = Parola değiştirildi. Diğer cihazlarınızdaki oturumlar kapatıldı.
account-add-failed = Eklenemedi: { $name }
account-added = { $name } artık oturum açabilir.
account-role-failed = { $name } adlı kişinin yetkileri değiştirilemedi
account-remove-title = { $name } kaldırılsın mı?
account-remove-body = Her yerdeki oturumları kapatılır ve yeniden oturum açamaz. Kütüphaneye eklediği hiçbir şey kaldırılmaz.
account-remove = Kaldır
account-remove-failed = Kaldırılamadı: { $name }
account-token-failed = Belirteç oluşturulamadı
account-copied = Kopyalandı
account-copy-token-failed = Kopyalanamadı: belirteci seçip kendiniz kopyalayın
account-copy-failed = Kopyalanamadı: seçip kendiniz kopyalayın
account-revoke-title = “{ $name }” iptal edilsin mi?
account-revoke-body = Onu hâlâ kullanan her şey bundan sonra geri çevrilir.
account-revoke = İptal et
account-revoke-failed = “{ $name }” iptal edilemedi
account-disconnect-title = { $app } bağlantısı kesilsin mi?
account-disconnect-body = Bundan sonra geri çevrilir; Hoard’u kullanmak için yeniden bağlanması gerekir.
account-disconnect = Bağlantıyı kes
account-disconnect-failed = Bağlantı kesilemedi: { $app }
account-guest-failed = Bu değiştirilemedi
account-never = hiç
account-you = Siz
account-sign-out = Oturumu kapat
account-signed-in-as-before = Oturum açan:{ " " }
account-signed-in-as-admin = , yönetici.
account-signed-in-as-edit = , kütüphaneyi değiştirebilir.
account-signed-in-as-view = , bakabilir ama hiçbir şeyi değiştiremez.
account-app-signin = Hoard uygulaması oturumunuzu kendisi açar, bu yüzden parola yoktur. Motor eklentileri ve hoard komutu için aşağıdan bir erişim belirteci oluşturun.
account-password-at-provider = Parolanızı oturum açtığınız sağlayıcı tutar; bu yüzden onu orada değiştirirsiniz.
account-password-at-directory = Parolanızı oturum açtığınız dizin tutar; bu yüzden onu orada değiştirirsiniz.
account-current-password = Geçerli parola
account-new-password = Yeni parola
account-new-password-again = Yeni parola (yeniden)
account-change-password = Parolayı değiştir
account-password-mismatch = Yeni parolalar eşleşmiyor.
account-password-rule = En az 8 karakter.
account-people = Kişiler
account-people-hint = Her kişi bakabilir (arama, önizleme ve indirme), kütüphaneyi de değiştirebilir (ekleme, düzenleme, etiketleme, koleksiyona katma ve arşivleme) ya da ayrıca yönetici olabilir (Ayarlar, hesaplar ve bakım ile kalıcı silme).
account-you-tag = (siz)
account-signs-in-oauth = OAuth sağlayıcısıyla oturum açar
account-signs-in-directory = Dizinle oturum açar
account-since = { $date } tarihinden beri
account-what-can-do = { $name } adlı kişinin yapabilecekleri
account-name = Ad
account-how-sign-in = Nasıl oturum açacağı
account-password-label = Parola (en az 8 karakter)
account-password-placeholder = Parola (8+ karakter)
account-what-they-can-do = Yapabilecekleri
account-add = Ekle
account-ai-apps = Claude ve diğer yapay zekâ uygulamaları
account-ai-apps-hint = Claude’u (ya da herhangi bir MCP uygulamasını) bağlayın; sizin için varlık arayıp getirebilir, Hoard’un henüz tanımlamadıklarını tanımlayıp etiketleyebilir. İlk seferde Hoard’da oturum açar; buna burada, tarayıcıda izin verirsiniz.
account-mcp-address = Hoard’un MCP adresi
account-copy = Kopyala
account-claude-code-1 = şunu çalıştırın:{ " " }
account-claude-code-2 = , ardından oturum açmak için{ " " }
account-claude-code-3 = { " " }komutunu kullanın.{ " " }
account-claude-code-4 = { " " }bir paketi tanımlar.
account-claude-ai = Settings → Connectors → Add custom connector bölümüne gidin, adresi yapıştırın, ardından Connect’e tıklayın. Bunlar Anthropic’in sunucularından bağlanır; bu yüzden Hoard’a internetten HTTPS üzerinden ulaşılabilmesi gerekir.
account-claude-ai-local = Settings → Connectors → Add custom connector bölümüne gidin, adresi yapıştırın, ardından Connect’e tıklayın. Bunlar Anthropic’in sunucularından bağlanır; bu yüzden Hoard’a internetten HTTPS üzerinden ulaşılabilmesi gerekir (bu adrese ulaşılamaz: Ayarlar → Bağlantı bölümünden Genel adresi ayarlayın).
account-connection-meta = kullanıcı: { $user } · başlangıç: { $since } · son kullanım: { $used }
account-no-apps = Henüz bağlı uygulama yok.
account-tokens = Erişim belirteçleri
account-tokens-hint-1 = { "" }
account-tokens-hint-2 = { " " }komut satırı aracı, betikler, MCP istemcileri ve Unreal çalışanı için: bunu şu şekilde gönderirler:{ " " }
account-tokens-hint-3 = . Bir belirteç, sizin yapabileceklerinizi aşmamak üzere seçtiğiniz şeyleri yapabilir, ama hesapları ya da belirteçleri asla yönetemez. Yalnızca bakabilen bir belirteç arama, indirme ve bir projeyi eşitleme için yeterlidir.
account-tokens-admin = Herkesin belirteçlerini ve bağlı uygulamalarını görürsünüz; diğerleri yalnızca kendilerininkini görür.
account-made-before = { "" }
account-made-after = : hemen kopyalayın. Bir daha gösterilmez.
account-new-token = Yeni belirteç
account-cli-1 = CLI için:{ " " }
account-cli-2 = ; ya da şu değişkene koyun:{ " " }
account-cli-3 = .
account-done = Tamam
account-token-meta = oluşturulma: { $made } · son kullanım: { $used }
account-token-meta-by = oluşturulma: { $made }, oluşturan: { $by } · son kullanım: { $used }
account-no-tokens = Henüz belirteç yok.
account-token-name = Onu ne kullanacak
account-token-name-placeholder = Onu ne kullanacak, örn. “Unreal çalışanı”
account-token-role = Neler yapabilir
account-make-token = Belirteç oluştur
account-guests = Konuklar
account-guests-allow = Sunucuya ulaşabilen herkesin oturum açmadan göz atmasına izin ver
account-guests-hint = Arayabilir, önizleyebilir ve indirebilirler ama hiçbir şey ekleyemez, değiştiremez ya da silemez, Ayarlar’ı da göremezler.

## SignIn.vue

signin-create-title = Hesabınızı oluşturun
signin-title = Oturum aç
signin-setup-lede = Bu sunucuda henüz hesap yok. Burada oluşturacağınız hesap her şeyi yapabilir; diğer kişiler için hesapları daha sonra Ayarlar → Hesap bölümünden ekleyin.
signin-overlay-lede = Burada göz atmak herkese açık ama değişiklik yapmak için hesap gerekir.
signin-with = { $provider } ile oturum aç
signin-or-password = ya da bir ad ve parolayla
signin-username = Kullanıcı adı
signin-name = Ad
signin-new-password = Yeni parola
signin-password = Parola
signin-password-rule = En az 8 karakter.
signin-confirm-password = Parolayı onayla
signin-mismatch = İki parola eşleşmiyor.
signin-setup-code = Kurulum kodu
signin-setup-code-hint-before = Bu sunucuya şu değişkenle verilen kod:{ " " }
signin-setup-code-hint-after = .
signin-creating = Oluşturuluyor…
signin-signing-in = Oturum açılıyor…
signin-create = Hesap oluştur
signin-submit = Oturum aç
signin-keep-browsing = Göz atmaya devam et
signin-directory-hint = { $directory } adınızı ve parolanızı kullanın.
signin-forgot-1 = Bir Hoard parolasını mı unuttunuz? Yenisi için sunucuda şunu çalıştırın:{ " " }
signin-forgot-2 = { " " }(Docker’da:{ " " }
signin-forgot-3 = ).

## SetupWizard.vue (first visit to a new server)

setup-title = Kütüphaneniz nerede dursun?
setup-lede = Sunucuda Hoard’un paketlerinizi saklayacağı bir klasör seçin ya da onu zaten sahip olduğunuz bir paket klasörüne yönlendirin. Daha sonra Ayarlar’dan başka disklerde başka konumlar da ekleyebilirsiniz.
setup-lede-desktop = Hoard’un paketlerinizi saklayacağı bir klasör seçin ya da onu zaten sahip olduğunuz bir paket klasörüne yönlendirin. Daha sonra Ayarlar’dan başka disklerde başka konumlar da ekleyebilirsiniz.
setup-unreachable = Sunucuya ulaşılamadı: { $error }
setup-unreachable-desktop = Hoard yanıt vermiyor: { $error }
setup-try-again = Yeniden dene
setup-loading = Yükleniyor…
setup-start = Kütüphaneyi başlat
setup-ready = Kütüphaneniz hazır
setup-managed-before = Yeni paketler şu klasöre gider:{ " " }
setup-managed-after = . İlk paketlerinizi eklemek için dosyaları, klasörleri ya da arşivleri sayfada herhangi bir yere bırakın.
setup-watched-before = Hoard şu klasörü izliyor:{ " " }
setup-watched-after = ; paketler o buldukça görünür. Yükleme de yapabilmek için Ayarlar → Kütüphane konumları bölümünden Hoard’un yönettiği bir kütüphane ekleyin.
setup-open = Kütüphaneyi aç

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = { $app } bağlansın mı?
connect-lede-1 = { " " }bu Hoard’daki her şeyi arayabilecek, indirebilecek, tanımlayabilecek ve etiketleyebilecek; bunu şu hesap adına yapacak:{ " " }
connect-lede-2 = . Hesapları ya da erişim belirteçlerini yönetemez.
connect-back-before = Şuraya geri döneceksiniz:{ " " }
connect-back-after = . Bağlantıyı istediğiniz zaman Ayarlar → Hesap bölümünden kesebilirsiniz.
connect-allow = İzin ver
connect-cancel = İptal
connect-failed = Bu uygulama bağlanamıyor
connect-go-library = Kütüphaneye git
connect-checking = Denetleniyor…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = Bu düz http://; bu yüzden birkaç şey HTTPS gerektirir: WebGPU ile 3B, modern pano, Hoard’u uygulama olarak kurma ve Claude bağlayıcıları.
insecure-switch = https:// adresine geç
insecure-set-up = HTTPS’yi ayarla
insecure-dismiss = Bunu bir daha gösterme

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = Bu dosya bir mağaza dosyası olarak okunamayacak kadar büyük.
formatwizard-not-json = Bu dosya geçerli bir JSON değil; bu yüzden metin olarak gösteriliyor.
formatwizard-read-failed = Dosya okunamadı: { $error }
formatwizard-back-title = Forma dönülsün mü?
formatwizard-back-body = TOML’de yapılan değişiklikler kaybolur: form biçimi elindekilerden yeniden yazar.
formatwizard-back-to-form = Forma dön
formatwizard-miss-name = Dosyaları şu adı almıyor: { $file }. Onu Dosyalar’a ekleyin.
formatwizard-miss-not-the-stores = Mağazanın dosyalarını tanıdığı şey bu dosyada yok. Tanıma ölçütü’nü değiştirin.
formatwizard-miss-no-title = Bu dosyada başlık bulamıyor. Başlığı seçin.
formatwizard-miss-short-name = adı uymuyor
formatwizard-miss-short-not-the-stores = mağazanın değil
formatwizard-miss-short-no-title = başlık yok
formatwizard-saved-packs = Kaydedildi: { $file }. { $packs } paket yeniden okunuyor.
formatwizard-saved = Kaydedildi: { $file }.
formatwizard-save-failed = Biçim kaydedilemedi: { $error }
formatwizard-zip-failed = Zip oluşturulamadı: { $error }
formatwizard-stop-title = Bu dosyayı öğretmeyi bırakmak istiyor musunuz?
formatwizard-stop-body = Biçim kaydedilmez.
formatwizard-stop = Bırak
formatwizard-label = Biçim sihirbazı
formatwizard-changing = Değiştiriliyor: { $name }
formatwizard-starting-from = Başlangıç: { $name }
formatwizard-title = Hoard’a bir mağaza dosyasını öğretin
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }(paket:{ " " }
formatwizard-place-3 = )
formatwizard-edit-toml = TOML olarak düzenle
formatwizard-file-reference = Başvuru için dosya.
formatwizard-click-json = Dosyada { $field } değerine tıklayın.
formatwizard-click-text = Dosyada { $field } değerine tıklayın ya da onu seçin.
formatwizard-pick-first = Sağdan bir alan seçin, ardından değerine burada tıklayın.
formatwizard-loading = Yükleniyor…
formatwizard-the-format = Biçim
formatwizard-syntax-before = Söz dizimi şurada:{ " " }
formatwizard-syntax-after = .
formatwizard-store = Mağaza
formatwizard-store-placeholder = Mağazanın adı, örneğin Kenney
formatwizard-files = Dosyalar
formatwizard-recognize = Tanıma ölçütü
formatwizard-recognize-json = Yalnızca mağazanın dosyalarında olan bir anahtar
formatwizard-recognize-text = Yalnızca mağazanın dosyalarında olan bir satır
formatwizard-recognize-hint = Her satıra bir yol; herhangi biri yeterlidir. Başka dosyalar da aynı adı taşıyorsa gerekir.
formatwizard-seller = Satıcı
formatwizard-seller-placeholder = Dosya belirtmiyorsa
formatwizard-tags = Etiketler
formatwizard-tags-placeholder = Tanımladığı her şey için, virgülle ayrılmış
formatwizard-fields = Alanlar
formatwizard-selector = { $field } seçicisi
formatwizard-clear = Temizle: { $field }
formatwizard-pick = Seç
formatwizard-what-it-reads = Ne okuyor
formatwizard-read-title = Başlık
formatwizard-read-seller = Satıcı
formatwizard-read-license = Lisans
formatwizard-read-url = Mağaza sayfası
formatwizard-read-kind = Tür
formatwizard-read-tags = Etiketler
formatwizard-read-origin = Yapıldığı araç
formatwizard-read-size = Boyut
formatwizard-also = Bu dosyayı { $format } biçimi de okuyor. Önce bu biçim denenir.
formatwizard-library = Kabul ettiği bir ada sahip { $files } dosya var ({ $packs } paket içinde). Denenen { $tried } dosyanın { $read } tanesini okuyor.
formatwizard-library-none = Kütüphanede kabul ettiği bir ada sahip dosya yok.
formatwizard-share = Paylaş
formatwizard-share-hint = Biçimin, örnek olarak bu dosyanın ve örneğin nasıl okunduğunun Hoard deposundaki gibi düzenlenmiş bir zip’i; herkesin alabilmesi için gönderin. Önce örnekteki paylaşılmaması gereken her şeyi çıkarın.
formatwizard-sample = Paylaşılacak örnek
formatwizard-download = İndir: { $file }
formatwizard-close = Kapat
formatwizard-use = Burada kullan
formatwizard-cancel = İptal
