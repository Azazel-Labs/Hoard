## Galat server

error-account-not-found = akun tidak ditemukan
error-asset-gone = aset tidak ditemukan
error-asset-not-found = aset { $id } tidak ditemukan
error-blob-not-found = blob { $hash } tidak ditemukan
error-collection-not-found = koleksi { $id } tidak ditemukan
error-connection-not-found = koneksi tidak ditemukan
error-derivative-not-found = berkas turunan { $kind } dari { $id } tidak ditemukan
error-file-in-pack-not-found = { $path } di paket { $id } tidak ditemukan
error-format-not-found = format { $file } tidak ditemukan
error-frame-sheet-aseprite-only = hanya berkas Aseprite yang punya lembar bingkai
error-frame-sheet-frames = ini hanya punya satu bingkai, atau terlalu banyak untuk ditata
error-location-not-found = lokasi { $id } tidak ditemukan
error-maintenance-task-not-found = tugas pemeliharaan { $id } tidak ditemukan
error-open-upload-not-found = unggahan terbuka { $id } tidak ditemukan
error-original-not-found = berkas asli dari { $id } tidak ditemukan
error-pack-not-found = paket { $id } tidak ditemukan
error-preview-too-big = gambar ini terlalu besar untuk dibuatkan pratinjau ringan
error-project-not-found = proyek { $name } tidak ditemukan
error-saved-search-not-found = pencarian tersimpan { $id } tidak ditemukan
error-sign-in-first = masuk dulu (atau kirim token akses)
error-those-assets-not-found = aset-aset itu tidak ditemukan
error-thumbnail-not-found = gambar mini untuk { $id } tidak ditemukan
error-token-not-found = token tidak ditemukan
error-upload-not-found = unggahan { $id } tidak ditemukan
error-pack-needs-name = paket perlu nama
error-no-library = belum ada lokasi pustaka untuk menyimpan berkas: tambahkan lokasi terkelola di Pengaturan → Lokasi pustaka
error-upload-failed = unggahan gagal: { $reason }
error-upload-no-files = tidak ada berkas dalam unggahan
error-analyze-nothing-to-do = tidak ada yang perlu dikerjakan: rules=false dan LLM mati
error-too-many-assets = terlalu banyak aset dalam satu permintaan
error-too-many-items = terlalu banyak item dalam satu permintaan
error-not-a-hash = bukan hash hex blake3
error-merge-into-tag = into harus berupa tag
error-nothing-to-merge = tidak ada yang perlu digabungkan
error-selection-size = berikan antara 1 dan { $max } ID aset
error-collection-needs-name = koleksi perlu nama
error-collection-exists = sudah ada koleksi bernama { $name }
error-collection-empty = koleksi { $name } kosong
error-conversion-failed = tidak dapat mengonversi ke { $format }: { $reason }
error-max-size = max harus pangkat dua dari 64 sampai 16384
error-replace-ext = ext harus ekstensi berkas yang pendek
error-empty-file = berkas kosong
error-replace-duplicate = berkas hasil konversi sama dengan berkas yang sudah ada di paket; paketnya digabungkan ke berkas itu
error-ai-address = Alamat AI harus diawali http:// atau https://
error-local-llm-action = action berupa list, download atau delete, dan dua yang terakhir memerlukan model
error-role = Peran berupa view, edit atau admin
error-admin-only = hanya admin yang dapat melakukan ini
error-view-only = akun ini hanya dapat melihat, tidak dapat mengubah apa pun: admin dapat mengubahnya di Pengaturan → Akun
error-portal-only = masuk ke portal untuk mengelola akun dan token
error-not-from-portal = permintaan ini tidak berasal dari portal Hoard
error-account-name = Pilih nama hingga 64 karakter
error-password-length = Gunakan kata sandi minimal { $min } karakter
error-too-many-codes = Terlalu banyak kode yang salah: tunggu { $seconds } detik
error-already-set-up = Sudah ada akun: silakan masuk
error-too-many-tries = Terlalu banyak percobaan yang salah: tunggu { $seconds } detik
error-directory-unavailable = Direktori tidak dapat memeriksa kata sandi Anda: { $reason }
error-password-elsewhere = Kata sandi Anda disimpan oleh direktori atau penyedia yang Anda gunakan untuk masuk
error-current-password = Kata sandi Anda saat ini tidak benar
error-account-source = Akun masuk lewat Hoard (internal), direktori (ldap) atau OAuth
error-account-exists = Sudah ada akun dengan nama itu
error-own-role = Admin lain dapat mengubah peran Anda
error-remove-self = Anda tidak dapat menghapus akun Anda sendiri selagi masuk dengan akun itu
error-token-name = Beri nama token sesuai yang akan memakainya, hingga 80 karakter
error-token-role = Token tidak dapat melakukan lebih dari akun yang membuatnya
error-wrong-setup-code = Itu bukan kode penyiapan yang diberikan ke server di HOARD_SETUP_CODE
error-wrong-password = Nama dan kata sandi itu tidak cocok
error-folder-missing = { $path } tidak ada (disk yang belum dipasang?)
error-s3-path = lokasi S3 berbentuk s3://bucket
error-s3-address = alamat S3 harus diawali http:// atau https://
error-s3-header = header S3 tambahan berbentuk Name=value, dan “{ $header }” tidak berbentuk begitu
error-keychain = tidak dapat menyimpannya di keychain sistem: { $reason }
error-not-built = salinan Hoard ini dibuat tanpa { $feature }
error-openai-auth-unreachable = tidak dapat menghubungi auth.openai.com
error-openai-unexpected = jawaban tak terduga dari OpenAI
error-openai-answered = OpenAI menjawab { $status }: { $reason }
error-chatgpt-id-token-not-jwt = token ID bukan JWT
error-chatgpt-id-token-issuer = token ID berasal dari { $issuer }, bukan { $expected }
error-chatgpt-id-token-audience = token ID ditujukan untuk aplikasi lain
error-chatgpt-id-token-expired = token ID sudah kedaluwarsa
error-chatgpt-id-token-nonce = token ID bukan milik proses masuk ini
error-chatgpt-no-state = alamatnya tidak memiliki state; salin seluruh alamat
error-chatgpt-not-started = proses masuk ini tidak dimulai di sini, atau dimulai lebih dari 15 menit yang lalu. Mulai lagi dari Pengaturan.
error-chatgpt-canceled = proses masuk dibatalkan
error-chatgpt-refused = ChatGPT menolak proses masuk: { $reason }
error-chatgpt-no-code = alamatnya tidak memiliki kode masuk; salin seluruh alamat
error-chatgpt-other-registration = ChatGPT menjawab untuk pendaftaran lain dari server ini; mulai proses masuk lagi
error-chatgpt-other-registration-desktop = ChatGPT menjawab untuk salinan Hoard yang lain; mulai proses masuk lagi
error-chatgpt-not-registered = ChatGPT belum selesai mendaftarkan Hoard; mulai proses masuk lagi
error-chatgpt-code-used = kode masuk sudah dipakai atau sudah kedaluwarsa; masuk lagi
error-chatgpt-refused-code = ChatGPT menolak proses masuk ({ $code })
error-chatgpt-no-id-token = ChatGPT tidak mengirim token ID
error-chatgpt-plan-not-allowed = Sudah masuk, tetapi penggunaan langganan tidak diizinkan. Masuk lagi dan izinkan, atau gunakan kunci API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = belum masuk ke ChatGPT; masuk dari Pengaturan → Penandaan AI
error-chatgpt-refresh-later = ChatGPT tidak dapat memperbarui proses masuk saat ini ({ $code }); akan dicoba lagi nanti
error-chatgpt-signed-out-problem = ChatGPT mengeluarkan server ini ({ $code }). Masuk lagi agar tetap dapat memberi tag dengan langganan Anda.
error-chatgpt-signed-out-problem-desktop = ChatGPT mengeluarkan Hoard ({ $code }). Masuk lagi agar tetap dapat memberi tag dengan langganan Anda.
error-chatgpt-signed-out = ChatGPT mengeluarkan server ini ({ $code }); masuk lagi dari Pengaturan → Penandaan AI
error-chatgpt-signed-out-desktop = ChatGPT mengeluarkan Hoard ({ $code }); masuk lagi dari Pengaturan → Penandaan AI
error-openai-api-unreachable = tidak dapat menghubungi api.openai.com
error-openai-broke-off = balasan dari OpenAI terputus
error-chatgpt-kept-refusing = OpenAI terus menolak proses masuk ChatGPT; masuk lagi dari Pengaturan → Penandaan AI
error-chatgpt-request-failed = permintaan gagal
error-chatgpt-cut-off = balasan terpotong ({ $reason })
error-chatgpt-cut-off-no-reason = balasan terpotong (tanpa alasan)
error-openai-ended-early = balasan dari OpenAI berakhir sebelum lengkap
error-chatgpt-usage-limit = jatah pemakaian langganan ChatGPT Anda untuk Hoard sudah habis untuk saat ini. Lihat atau ubah batasnya di { $url }
error-chatgpt-not-eligible = akun atau workspace ChatGPT ini tidak dapat memakai langganannya di aplikasi lain. Gunakan kunci API.
error-chatgpt-usage-unavailable = ChatGPT tidak dapat memeriksa pemakaian langganan saat ini; akan dicoba lagi nanti
error-chatgpt-unsupported = pemakaian langganan ChatGPT tidak mendukung { $param } dengan model ini
error-chatgpt-unsupported-images = pemakaian langganan ChatGPT tidak mendukung { $param } dengan model ini; pilih model yang menerima gambar, atau matikan pengiriman gambar mini
error-chatgpt-unsupported-part = pemakaian langganan ChatGPT tidak mendukung sebagian permintaan dengan model ini
error-chatgpt-port = port adalah port yang dipakai browser untuk menjangkau Hoard
error-chatgpt-paste-address = tempel seluruh alamat, yang diawali http://127.0.0.1
error-too-many-hashes = paling banyak { $max } hash sekaligus
error-credits-pack = paket { $key }
error-credits-collection = koleksi { $key }
error-format-too-long = formatnya terlalu panjang
error-not-started = server belum dimulai
error-not-started-desktop = Hoard belum dimulai
error-not-format-file = bukan berkas format: { $name }
error-format-name = nama perlu huruf atau angka dari A sampai Z atau 0 sampai 9
error-format-miss-name = berkas format tidak menerima { $file }
error-format-miss-store = format tidak menerima { $file } untuk toko (lihat require)
error-format-miss-title = format tidak menemukan judul di { $file }
error-cant-download-as = berkas .{ $ext } tidak dapat diunduh sebagai { $format }
error-only-images-resize = hanya gambar yang dapat diubah ukurannya
error-https-no-certificate = Tempel atau unggah sertifikat dan kuncinya terlebih dahulu
error-https-no-cert-found = Sertifikat tidak ditemukan: tempel teks yang diawali -----BEGIN CERTIFICATE-----
error-https-no-key-found = Kunci privat tidak ditemukan: tempel teks yang diawali -----BEGIN PRIVATE KEY-----
error-https-several-keys = Ada lebih dari satu kunci privat: tempel hanya kunci yang cocok dengan sertifikat
error-https-key-password = Kunci dilindungi kata sandi: berikan kunci tanpa kata sandi (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Sertifikat bukan PEM yang valid
error-https-key-not-pem = Kunci privat bukan PEM yang valid
error-https-key-mismatch = Kunci tidak cocok dengan sertifikat. Jika Anda memberikan rantai, letakkan sertifikat server Anda di urutan pertama
error-https-unusable = Sertifikat atau kunci tidak dapat dipakai
error-https-expired = Sertifikat ini kedaluwarsa pada { $date }
error-https-not-yet-valid = Sertifikat ini belum berlaku sampai { $date }
error-https-unreadable = Itu bukan sertifikat yang dapat dibaca Hoard
error-https-bad-name = “{ $name }” bukan alamat yang dapat dimuat sertifikat
error-https-test-timed-out = koneksi uji kehabisan waktu
error-https-test-failed = koneksi uji gagal
error-https-no-ca = Hoard belum membuat otoritas sertifikatnya
error-https-unavailable = HTTPS tidak tersedia di server ini
error-https-certificate-mode = Sertifikat diperlukan untuk “Sertifikat saya”
error-https-test = Sertifikat tidak berfungsi dalam koneksi uji: { $reason }
error-nothing-to-retry = tidak ada yang dapat dicoba lagi di sana
error-ai-unreachable = tidak dapat menghubungi { $url }
error-ai-key-refused = { $url } menolak kunci API ({ $status })
error-ai-answered = { $url } menjawab { $status }: { $reason }
error-ai-not-json = { $url }/models tidak menjawab dengan JSON
error-model-server-refused = server model: { $reason }
error-no-model-server = tidak ada server model yang menjawab di { $url }
error-not-llama-router = { $url } bukan server llama.cpp dalam mode router, jadi tidak dapat mengunduh model
error-model-download = tidak dapat mengunduh { $model }: { $reason }
error-model-delete = tidak dapat menghapus { $model }: { $reason }
error-unknown-kind = jenis aset { $kind } tidak dikenal
error-watched-is-folder = lokasi pantauan adalah folder di server ini
error-cant-reach = tidak dapat menjangkau { $path }: { $reason }
error-folder-full-path = berikan path lengkap folder
error-cant-create = tidak dapat membuat { $path }
error-cant-write = tidak dapat menulis ke { $path }
error-cant-read = tidak dapat membaca { $path }
error-overlaps-location = itu bertumpang tindih dengan lokasi { $name } ({ $path })
error-overlaps-data = itu bertumpang tindih dengan folder data Hoard ({ $path })
error-overlaps-cache = itu bertumpang tindih dengan folder cache Hoard ({ $path })
error-not-watched = { $name } bukan folder pantauan
error-watched-no-kinds = folder pantauan tidak menerima berkas baru, jadi tidak ada jenis yang bisa diubah
error-keep-catch-all = jadikan lokasi lain sebagai lokasi penampung
error-location-holds-files = { $name } masih berisi berkas: pindahkan dulu ke lokasi lain (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } tidak dapat dijangkau, jadi Hoard tidak tahu apakah lokasi itu masih berisi berkas
error-relocate-running = berkas sedang dipindahkan antarlokasi; coba lagi setelah selesai
error-watched-not-moved = { $name } adalah folder pantauan: Hoard tidak memindahkan berkasnya
error-no-other-location = tidak ada lokasi pustaka lain untuk tujuan pemindahan berkas: tambahkan dulu
error-full-path = berikan path lengkap
error-pick-folder = pilih folder
error-task-running = “{ $task }” sudah berjalan
error-token-account-gone = akun token ini sudah tidak ada, jadi token ini tidak punya favorit atau pencarian
error-favorites-too-many = paling banyak 10.000 sekaligus
error-saved-search-name = pencarian tersimpan perlu nama paling banyak 80 karakter
error-saved-search-too-long = pencarian itu terlalu panjang untuk disimpan
error-texture-pattern-map = Pola nama tekstur perlu { "{" }map{ "}" }, atau semua map dari satu material akan mendapat nama yang sama
error-texture-pattern-fields = Pola nama tekstur dapat memakai { "{" }name{ "}" }, { "{" }map{ "}" } dan { "{" }size{ "}" }
error-texture-name-chars = Nama tekstur tidak boleh berisi < > : " / \ | ? atau *
error-texture-map-type = Tidak ada jenis map bernama { $role }
server-role-base-color = warna
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = roughness
server-role-gloss = gloss
server-role-metalness = metalness
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = height
server-role-bump = bump
server-role-cavity = cavity
server-role-specular = spekular
server-role-opacity = opasitas
server-role-emissive = emisif
server-role-translucency = translusensi
error-texture-suffix-missing = Beri akhiran untuk map { $map }
error-texture-suffix-chars = Akhiran untuk map { $map } tidak boleh berisi < > : " / \ | ? atau *
error-texture-ending-missing = Setiap akhir nama buatan sendiri perlu akhir nama yang dicari
error-texture-ending-chars = Akhir nama buatan sendiri tidak boleh berisi < > : " / \ | ? atau *
error-texture-ending-twice = { $ending } ada dua kali dalam akhir nama Anda
error-oauth-no-client-id = Tautan ini tidak memiliki client_id.
error-oauth-unknown-app = Aplikasi ini tidak (lagi) terdaftar di sini. Mulai menghubungkan lagi dari aplikasi.
error-oauth-redirect = Aplikasi meminta dikembalikan ke alamat yang tidak didaftarkannya.
error-oauth-response-type = Hoard hanya memberikan kode masuk (response_type=code).
error-oauth-pkce = Aplikasi harus memakai PKCE dengan S256 untuk masuk ke Hoard.
error-light-preview-images = hanya gambar yang memiliki pratinjau ringan
error-no-light-preview = tidak ada pratinjau ringan: { $reason }
error-no-frame-sheet = tidak ada lembar bingkai: { $reason }
error-thumbnail-source = sumber gambar mini tidak dikenal: { $source }
error-thumbnail-too-big = gambar itu terlalu besar untuk gambar mini
error-thumbnail-models-only = hanya model yang menerima gambar mini yang digambar oleh browser
error-not-a-picture = bukan gambar yang dapat dibaca Hoard: { $reason }
error-project-name = proyek perlu nama paling banyak 200 karakter
error-project-too-big = paling banyak satu juta aset dalam satu proyek
error-project-too-many-at-once = paling banyak 100.000 sekaligus
error-server-name = Pilih nama hingga { $max } karakter, dalam satu baris
error-tool-setting = { $tool }: pilih mati, otomatis, atau berikan path
error-ai-model = Pilih model yang harus dipakai layanan AI
error-public-address = Alamat publik harus diawali http:// atau https://
error-update-hook-address = Alamat yang dipanggil untuk pembaruan harus diawali http:// atau https://
error-update-hook-auth = Header Authorization tidak boleh berisi baris baru atau karakter kontrol lainnya
error-no-soundfont = Tidak ada SoundFont di { $path } pada server
error-no-soundfont-desktop = Tidak ada SoundFont di { $path }
error-workers = Pekerja latar belakang: antara 1 dan { $max }
error-ai-batch = Aset per permintaan AI: antara 1 dan 200
error-conversion-cache = Unduhan hasil konversi: minimal 100 MB
error-conversion-keep-days = Aturan konversi: simpan berkas asli selama 1 sampai { $max } hari
error-conversion-target = Aturan konversi: Hoard tidak dapat mengonversi berkas ke { $format }
error-conversion-no-source = Aturan konversi: setiap aturan memerlukan setidaknya satu format untuk dikonversi
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Aturan konversi: berkas .{ $ext } tidak dapat dikonversi ke { $format }
error-conversion-twice = Aturan konversi: .{ $ext } ada di dua aturan
error-conversion-cant-undo = Berkas ini tidak dikonversi oleh aturan, atau berkas aslinya sudah dihapus.
error-conversion-path-taken = Sekarang ada berkas lain di { $path }, jadi berkas asli tidak dapat dikembalikan ke sana.
error-limits-zero = Batas tidak boleh nol
error-settings-object = seharusnya objek JSON berisi pengaturan
error-bad-setting = pengaturan tidak valid: { $reason }
error-auth-default-role = Peran bawaan berupa view, edit atau admin
error-auth-session-hours = Sesi berlangsung antara 1 jam dan satu tahun
error-auth-settings = pengaturan autentikasi tidak valid: { $reason }
error-ldap-settings = pengaturan LDAP tidak valid: { $reason }
error-ldap-no-server = Tambahkan alamat server direktori
error-ldap-base-dn = Sebutkan tempat entri orang-orang berada (base DN)
error-ldap-username-attribute = Sebutkan atribut yang berisi nama untuk masuk
error-ldap-bind-dn = Berikan akun yang mencari orang, atau cari mereka secara anonim
error-ldap-ca-cert = Otoritas sertifikat bukan sertifikat PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Alamat server direktori kosong
error-ldap-server-form = { $server }: alamat server direktori berbentuk host:port, ldap://… atau ldaps://…
error-ldap-not-starttls = { $server }: itu LDAPS, bukan StartTLS
error-ldap-no-certificate = tidak ada sertifikat di dalamnya
error-ldap-password-again = Berikan lagi kata sandi akun yang mencari orang: kata sandi tersimpan hanya dikirim ke server tempat kata sandi itu diberikan
error-ldap-unreachable = tidak ada server direktori yang dapat dijangkau ({ $reasons })
error-ldap-no-anonymous = direktori tidak mengizinkan pencarian orang secara anonim ({ $reason })
error-ldap-bind-failed = akun yang mencari orang tidak dapat masuk ({ $reason })
error-ldap-search = mencari { $name } di bawah { $base }
error-ldap-group-search = mencari grup milik { $name } di bawah { $base }
error-ldap-several = { $count } entri di direktori cocok dengan { $name }: persempit filter pengguna
error-ldap-password-check = direktori tidak mau memeriksa kata sandi { $name } ({ $reason })
error-ldap-wrong-password = Direktori tidak mengenal { $name } dengan kata sandi itu
error-oauth-client-id = Berikan client ID yang dibuat penyedia untuk Hoard
error-oauth-addresses = Alamat otorisasi dan token harus diawali https://
error-oauth-userinfo-address = Alamat info pengguna harus diawali https://
error-oauth-username-claim = Sebutkan claim yang berisi nama orang tersebut
error-oauth-secret-again = Berikan lagi client secret: yang tersimpan hanya dikirim ke alamat token tempat client secret itu diberikan
error-oauth-issuer = Issuer harus diawali https://
error-oauth-discovery-unreachable = Tidak dapat menghubungi { $url }: { $reason }
error-oauth-discovery-status = { $url } menjawab { $status }
error-oauth-discovery-bad = { $url } bukan dokumen discovery
error-oauth-discovery-incomplete = { $url } tidak menyebutkan alamat otorisasi dan token
error-signin-no-group = { $name } tidak berada dalam grup yang boleh memakai Hoard
error-signin-other-source = Sudah ada akun Hoard bernama { $name } yang masuk dengan cara lain
error-signin-other-subject = Sudah ada akun Hoard bernama { $name } untuk orang lain di penyedia
error-signin-no-account = { $name } belum memiliki akun Hoard: admin dapat menambahkannya di Pengaturan → Akun
error-signin-bad-name = Nama itu tidak dapat dijadikan akun Hoard (hingga 64 karakter)
error-oauth-off = Masuk dengan OAuth belum diaktifkan
error-oauth-bad-authorization-address = alamat otorisasi tidak valid
error-oauth-refused = Penyedia tidak memasukkan Anda: { $reason }
error-oauth-no-code = Penyedia mengembalikan Anda tanpa proses masuk
error-oauth-expired = Proses masuk itu terlalu lama atau dimulai di tempat lain: coba lagi
error-oauth-no-claim = Penyedia tidak menyebutkan siapa Anda (tidak ada claim { $claim })
error-oauth-no-sub = Penyedia tidak menyebutkan siapa Anda (tidak ada claim sub atau id)
error-oauth-token-unreachable = tidak dapat menghubungi alamat token penyedia
error-oauth-token-refused = Penyedia tidak mau menyelesaikan proses masuk: { $reason }
error-oauth-no-access-token = penyedia tidak mengirim token akses
error-oauth-userinfo-unreachable = tidak dapat menghubungi alamat info pengguna penyedia
error-oauth-userinfo-refused = Penyedia tidak mau menyebutkan siapa yang masuk ({ $status })
error-oauth-userinfo-not-json = info pengguna dari penyedia bukan JSON
error-format-no-name = name kosong
error-format-no-files = files kosong: sebutkan berkas yang dibaca format, misalnya ["*.json"]
error-format-empty-selector = kosong
error-format-unopened-bracket = kurung tutup tanpa kurung buka
error-format-unclosed-bracket = ada kurung yang tidak ditutup
error-format-empty-step-dots = langkah kosong (dua titik berurutan, atau titik di salah satu ujung)
error-format-unclosed-except = !( tidak ditutup
error-format-unclosed-filter = [ tidak ditutup
error-format-after-filters = { $rest } setelah key dan filter milik { $step }
error-format-except-needs-glob = !(…) diletakkan setelah key yang berisi *
error-format-except-not-glob = !(…) diletakkan setelah key yang berisi *, bukan { $key }
error-format-empty-step = langkah kosong
error-format-not-a-filter = [{ $filter }] bukan filter: gunakan [], [key=value|other], [array], [string], [number], [bool] atau [object]
error-format-filter-needs-value = [{ $filter }] memerlukan key dan value
error-store-search-empty = tidak ada yang dicari; ketik nama paket
error-update-hook-not-set = tetapkan dulu alamat yang akan dipanggil
error-upload-stalled = unggahan { $path } macet; kirim lagi bagian itu
error-upload-busy = { $path } sedang ditulis
error-upload-incomplete = { $count } berkas belum lengkap, misalnya { $files }
error-upload-no-files-listed = tidak ada berkas
error-upload-not-part = { $path } bukan bagian dari unggahan { $id }, atau unggahan sudah ditutup
error-bad-path = path tidak valid
error-upload-part-offset = { $offset } bukan posisi awal bagian dari { $path }
error-upload-interrupted = unggahan terputus: { $reason }
error-upload-part-long = bagian { $part } dari { $path } lebih panjang dari { $len } byte
error-upload-part-short = bagian { $part } dari { $path }: diterima { $got } byte, seharusnya { $len }
error-upload-file-long = { $path } lebih besar dari yang dinyatakan
error-upload-file-short = { $path }: diterima { $got } byte, seharusnya { $len }
error-upload-complete = unggahan sudah selesai
error-already-imported = sudah diimpor sebagai "{ $name }"

## Teks server

server-store-place-readme = README
server-store-place-folders = folder
server-store-why-linked = Ditautkan di { $place } paket
server-store-why-same-title = Judulnya sama dengan { $place } paket
server-store-why-by = Oleh { $author }, disebutkan di { $place } paket
server-store-why-meshes = Jumlah mesh kira-kira sama dengan paket ({ $count })
server-store-why-textures = Jumlah tekstur kira-kira sama dengan paket ({ $count })
server-chatgpt-revoke-failed = Sudah keluar di sini, tetapi OpenAI tidak dapat diberi tahu. Untuk memastikan, putuskan Hoard di ChatGPT Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Sudah masuk ke ChatGPT
server-chatgpt-signed-in = Hoard sudah masuk. Penandaan AI kini dapat memakai langganan ChatGPT Anda.
server-chatgpt-signed-in-as = Hoard sudah masuk sebagai { $email }. Penandaan AI kini dapat memakai langganan ChatGPT Anda.
server-chatgpt-not-finished-title = Proses masuk belum selesai
server-chatgpt-other-computer = Jika Anda memulai proses masuk dari Hoard di komputer lain, salin alamat halaman ini dari bilah alamat dan tempelkan ke Pengaturan Hoard di sana.
server-back-to-settings = Kembali ke Pengaturan
server-issues-pack = Paket yang tidak dapat ditambahkan
server-issues-pack-blurb = Hapus paket lalu tambahkan lagi, atau buka untuk melihat isinya.
server-issues-damaged = Berkas tersimpan yang rusak atau hilang
server-issues-damaged-blurb = Ditemukan oleh “Periksa kerusakan berkas”. Pulihkan dari cadangan atau tambahkan lagi paketnya, lalu jalankan “Periksa kerusakan berkas” lagi.
server-issues-folder = Folder pantauan yang tidak dapat dibaca
server-issues-folder-blurb = Paketnya dibiarkan apa adanya sampai folder dapat dibaca lagi.
server-issues-missing = Paket pantauan yang foldernya hilang
server-issues-missing-blurb = Setiap paket disimpan selama 3 hari untuk berjaga-jaga jika foldernya hanya sedang offline, lalu dihapus.
server-issues-job = Pekerjaan latar belakang yang gagal
server-issues-job-blurb = Mencoba lagi akan menjalankan pekerjaan dari awal.
server-issues-file = Berkas yang tidak dapat dibuka
server-issues-file-blurb = Hoard tidak dapat membuka atau menyimpannya. Paket pantauan mencobanya lagi pada pemindaian berikutnya; berkas yang sudah ada di pustaka tetap menyimpan isi sebelumnya.
server-issues-unpack = Arsip yang tidak dapat dibongkar
server-issues-unpack-blurb = Masing-masing disimpan di paketnya sebagai berkas biasa, jadi isinya tidak ada di pustaka.
server-issues-read = Berkas yang tidak dapat dipahami Hoard
server-issues-read-blurb = Berkas itu ada di pustaka, tetapi tanpa info, tag, atau gambar mini yang didapat dari membacanya.
server-issues-render = Model yang gagal dirender oleh perender gambar mini
server-issues-render-blurb = Model itu tidak punya gambar mini. Membukanya di penampil mungkin tetap berhasil, dan dapat menyimpan gambar mini.
server-issues-convert = Model yang tidak dapat dikonversi Blender
server-issues-convert-blurb = Model itu tidak punya salinan GLB, jadi tidak dapat ditampilkan dalam 3D.
server-issues-unreal = Paket Unreal yang tidak dapat dikonversi
server-issues-unreal-blurb = Halaman Unreal milik paket berisi paket Unreal lainnya.
server-issues-reformat = Berkas yang tidak dapat dikonversi oleh aturan konversi
server-issues-reformat-blurb = Berkas tersebut dibiarkan seperti semula.
server-issues-materials = Paket yang materialnya tidak dapat dikenali
server-issues-materials-blurb = Semua teksturnya ada; hanya saja tidak dikelompokkan menjadi material.
server-issues-material-thumb = Material tanpa gambar mini
server-issues-material-thumb-blurb = Materialnya ditemukan, tetapi pratinjaunya tidak dapat digambar.
server-issues-store-files = Berkas toko yang tidak dapat dibaca
server-issues-store-files-blurb = Berkas dari toko asal paket, seperti JSON Megascans, yang memberi nama dan tag pada paket.
server-issues-scan-again = Pindai lagi
server-issues-retry = Coba lagi
server-issues-reanalyze = Analisis ulang paket
server-issues-convert-again = Konversi lagi
server-issues-look-again = Cari lagi
server-issues-read-again = Baca lagi
server-issues-blob-missing = Berkas tersimpan sudah hilang.
server-issues-blob-damaged = Berkas tersimpan tidak cocok dengan yang disimpan: berkasnya rusak.
server-issues-folder-gone = Folder “{ $folder }” tidak ditemukan.
server-job-a-pack = sebuah paket
server-job-ingest = Memasukkan { $pack }
server-job-retag = Memberi tag ulang { $pack }
server-job-llm-tag = Memberi tag { $pack } dengan AI
server-job-convert = Mengonversi { $pack } dengan Blender
server-job-unreal = Mengonversi paket Unreal di { $pack }
server-job-reformat = Mengonversi berkas di { $pack } sesuai aturan konversi
server-job-scan = Memindai folder pantauan
server-job-relocate = Memindahkan berkas antarlokasi pustaka
server-job-looks = Menentukan warna dan tampilan
server-job-sounds = Mendengarkan suara
server-job-embed = Mempelajari isi gambar
server-job-hear = Mempelajari bunyi suara
server-job-materials = Mencari material di { $pack }
server-job-read-files = Membaca berkas { $pack }
server-job-store-lookup = Mencari halaman toko
server-maintenance-backup = Cadangkan basis data
server-maintenance-backup-blurb = Salinan basis data yang konsisten (paket, tag, catatan, koleksi, dan Pengaturan) di folder backups di sebelahnya. Berkas pustaka tidak termasuk: berkas itu tidak pernah berubah setelah disimpan, jadi sinkronisasi berkas apa pun dapat mencadangkannya.
server-maintenance-scrub = Periksa kerusakan berkas
server-maintenance-scrub-blurb = Membaca kembali setiap berkas tersimpan dan mencocokkannya dengan sidik jarinya, untuk menemukan berkas yang rusak atau hilang dari disk atau NAS. Butuh waktu lama untuk pustaka yang besar.
server-maintenance-reanalyze = Analisis ulang semua paket
server-maintenance-reanalyze-blurb = Membaca lagi setiap berkas dengan versi Hoard ini: gambar mini, pratinjau, detail, dan tag aturan dibuat ulang. Suntingan Anda sendiri tetap ada; AI tidak ditanya lagi.
server-maintenance-clear-cache = Hapus cache konversi
server-maintenance-clear-cache-blurb = Membuang berkas yang disimpan dari konversi “Unduh sebagai” (dibuat lagi saat diminta berikutnya) dan sisa pekerjaan yang terputus.
server-maintenance-shrink-thumbs = Perkecil gambar mini
server-maintenance-shrink-thumbs-blurb = Menyimpan gambar mini dan tampilan model yang disimpan sebagai PNG sesuai Pengaturan → Performa → Gambar mini, yang biasanya membuatnya beberapa kali lebih kecil. Gambar dengan transparansi, dan pixel art, tetap PNG.
server-maintenance-gc = Hapus berkas yang tidak dipakai
server-maintenance-gc-blurb = Menghapus berkas tersimpan yang tidak dirujuk apa pun lagi, seperti berkas dari paket yang sudah dihapus permanen.
server-update-no-address = belum ada alamat yang ditetapkan untuk dipanggil
server-update-no-answer = tidak ada jawaban dalam 30 detik

## Pengaturan: Tampilan

appearance-language = Bahasa
appearance-language-auto = Bahasa browser ({ $name })

## Root

root-unreachable = Tidak dapat menghubungi server Hoard: { $error }
root-unreachable-desktop = Hoard tidak merespons: { $error }
root-desktop-reopen = Jendela ini telah keluar. Tutup Hoard lalu buka lagi untuk melanjutkan.
root-try-again = Coba lagi
root-lazy-connect = koneksi aplikasi
window-minimize = Minimalkan
window-maximize = Maksimalkan
window-restore = Pulihkan
window-close = Tutup

## Aplikasi: pustaka, bilah atas, dan filter

app-lazy-settings = pengaturan
app-lazy-archive = arsip
app-lazy-shortcuts = pintasan
app-lazy-setup = penyiapan
app-theme-system = Ikuti tema sistem
app-theme-light = Tema terang
app-theme-dark = Tema gelap
app-color-red = Merah
app-color-orange = Oranye
app-color-yellow = Kuning
app-color-green = Hijau
app-color-teal = Hijau kebiruan
app-color-blue = Biru
app-color-purple = Ungu
app-color-pink = Merah muda
app-color-brown = Cokelat
app-color-sand = Pasir
app-color-gray = Abu-abu
app-color-slate = Abu-abu batu
app-color-black = Hitam
app-color-white = Putih
app-color-swatch = { $color }: aset yang banyak mengandung warna { $lower }
app-color-any = Warna apa saja
app-color-pick = Pilih warna apa saja
app-color = Warna
app-imitation = Terdengar seperti
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Tulis bunyi suara seperti kamu mengejanya: suara yang mirip muncul lebih dulu
app-retry = Coba lagi
app-undo = Urungkan
app-search-failed = Pencarian gagal: { $error }
app-unreachable = Tidak dapat menghubungi server: { $error }
app-unreachable-desktop = Hoard tidak merespons: { $error }
app-forget-project-title = Lupakan { $name }?
app-forget-project-body = Hoard berhenti mencantumkannya di tempat { $count } asetnya dipakai, sampai hoard sync berikutnya memberi tahu lagi. Tidak ada yang berubah di proyek.
app-forget-project-confirm = Lupakan
app-forget-project-failed = Tidak dapat melupakan { $name }: { $error }
app-forget-project-label = Lupakan proyek { $name }
app-forget-project = Lupakan proyek ini
app-save-name-like = mirip { $name }
app-save-name-favorites = favorit
app-search-saved = “{ $name }” disimpan: ada di Filter, di bawah Milik Anda
app-rank-alike = Paling mirip
app-rank-color = Paling banyak warnanya
app-rank-match = Paling cocok
app-rank-imitation = Paling mirip
app-sort = Urutkan
app-sort-newest = Terbaru
app-sort-name = Nama
app-sort-largest = Terbesar
app-sort-oldest = Terlama
app-collection-added = { $count } aset ditambahkan ke { $name }
app-collection-add-failed = Tidak dapat menambahkan ke koleksi: { $error }
app-collection-removed = { $count } dikeluarkan dari { $name }
app-collection-remove-failed = Tidak dapat mengeluarkan dari koleksi: { $error }
app-ns-subject = Subjek
app-ns-style = Gaya
app-ns-theme = Tema
app-ns-material = Material
app-ns-map = Map PBR
app-ns-color = Warna
app-ns-mood = Suasana
app-ns-genre = Genre
app-ns-license = Lisensi
app-ns-res = Resolusi
app-ns-duration = Durasi
app-ns-poly = Jumlah poligon
app-ns-format = Format
app-ns-weight = Bobot
app-ns-tech = Teknis
app-ns-blender = Versi Blender
app-ns-depth = Kedalaman bit
app-ns-ucs = Kategori suara (UCS)
app-ns-bpm = Tempo
app-ns-root = Nada dasar
app-ns-level = Kenyaringan
app-ns-ue = Kelas Unreal
app-ns-engine = Engine
app-ns-extracted = Sumber yang dipulihkan
app-ns-script = Sistem tulisan
app-bulk-failed = Tidak dapat mengubah { $count } aset itu: { $error }
app-bulk-untagged = { $tag } dilepas dari { $count } aset
app-bulk-tagged = { $count } aset diberi tag { $tag }
app-bulk-category-set = Kategori { $count } aset ditetapkan menjadi { $category }
app-bulk-category-cleared = Kategori { $count } aset dikosongkan
app-credits-failed = Tidak dapat membuat kredit: { $error }
app-archived-assets = { $count } aset diarsipkan
app-archived-one = { $name } diarsipkan
app-open-failed = Tidak dapat membuka aset itu: { $error }
app-unnamed-pack = paket
app-unnamed-collection = koleksi
app-unnamed-asset = aset
app-home-version = Hoard { $version } — kembali ke pustaka
app-home = Kembali ke pustaka
app-search-label = Cari aset
app-search-placeholder = Cari nama, path, tag
app-views = Tampilan
app-library = Pustaka
app-packs = Paket
app-packs-working = { $count } sedang diproses
app-update-title = Hoard { $version } sudah rilis (server ini menjalankan { $current }). Tarik image atau biner baru lalu mulai ulang; basis data memperbarui dirinya sendiri.
app-update = Perbarui
app-theme = Tema
app-sign-in = Masuk
app-archive-title = Arsip: { $contents }
app-archive = Arsip
app-settings = Pengaturan
app-palette-title = Palet perintah ({ $keys })
app-import-folder = Impor folder
app-add-files = Tambah berkas
app-add-files-short = Tambah
app-split-label = Cara mengimpor folder ini
app-split-before = { "" }
app-split-after = { " " }berisi { $count } folder atau arsip.
app-split-each = Impor masing-masing sebagai paket tersendiri ({ $count })
app-split-one = Impor sebagai satu paket
app-cancel = Batal
app-filters = Filter
app-done = Selesai
app-yours = Milik Anda
app-favorites = Favorit
app-forget-search-label = Lupakan pencarian tersimpan { $name }
app-forget-search = Lupakan pencarian ini
app-projects-title = Proyek yang menjalankan hoard sync dari sini, beserta isi sinkronisasi terakhirnya
app-projects = Proyek
app-project-synced = Disinkronkan { $when }
app-project-synced-by = Disinkronkan { $when } oleh { $who }
app-collections = Koleksi
app-no-filters = Tidak ada filter untuk pencarian ini.
app-kind = Jenis
app-facets-more = { $count } lagi
app-facets-fewer = lebih sedikit
app-searching = Mencari…
app-searching-live = Mencari
app-asset-count = { $count } aset
app-clear-all = Hapus semua
app-save-name-label = Nama untuk pencarian ini
app-save-name-placeholder = Beri nama
app-save = Simpan
app-save-search-title = Simpan pencarian ini di Filter, di bawah Milik Anda
app-save-search = Simpan pencarian
app-select-title = Pilih beberapa aset untuk diberi tag, dikategorikan, dikoleksi, diunduh, atau diarsipkan bersama (X, atau Ctrl-klik)
app-select = Pilih
app-tile-size = Ukuran ubin
app-supporting-title = Buffer glTF, pustaka material, map dari material, material yang hanya dipakai satu model, dan berkas toko yang menjelaskan model. Semuanya ikut terunduh saat Anda mengunduh yang memakainya. Halaman lembar sprite, dan sprite-nya yang juga ada di paket sebagai gambar terpisah, ditampilkan bersama lembarnya. Aset yang ada di paket dalam beberapa format ditampilkan sekali, dalam format yang paling baik dibaca Hoard.
app-supporting-hidden = { $count } berkas pendukung disembunyikan
app-supporting-show = Tampilkan
app-supporting-shown = Termasuk berkas pendukung
app-supporting-hide = Sembunyikan
app-folders = Folder
app-folders-title = Lihat paket seperti di disk, satu folder demi satu folder
app-folders-label = Folder paket
app-welcome-title = Jatuhkan paket aset di mana saja di halaman ini
app-welcome-before = Zip, tarball, folder, atau berkas lepas, semuanya bisa. Hoard membongkarnya, menyimpan setiap berkas sekali berdasarkan hash-nya, membaca ukuran, durasi, dan jumlah poligon, lalu memberi tag pada semuanya agar Anda dapat mencari hal seperti{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }atau{ " " }
app-welcome-after = .
app-welcome-import-folder = Impor folder
app-welcome-choose-files = Pilih berkas
app-none-similar = Belum ada yang cukup mirip dengan { $name }.
app-none = Tidak ada yang cocok.
app-none-query = Tidak ada yang cocok dengan “{ $query }”.
app-none-imitation = Belum ada suara terukur yang mirip “{ $word }”.
app-none-filters = Tidak ada yang cocok dengan filter ini.
app-none-query-filters = Tidak ada yang cocok dengan “{ $query }” dengan filter ini.
app-clear-search-filters = Hapus pencarian dan filter
app-loading-more = Memuat lebih banyak
app-keys-hint = Tombol panah untuk berpindah, Enter untuk membuka, Spasi untuk memutar suara, X atau Ctrl-klik untuk memilih beberapa (tekan lama di layar sentuh).
app-resize-label = Ubah ukuran panel detail
app-resize-title = Seret untuk mengubah ukuran · klik dua kali untuk lebar yang pas
app-selected-label = Aset terpilih
app-selected = { $count } dipilih
app-pick-assets = Pilih aset
app-download-zip = Unduh zip
app-credits-title = CREDITS.md untuk paket asal aset ini: siapa pembuatnya, lisensinya, dan baris kredit yang diminta
app-credits = Kredit
app-prefer-sources-title = SoundWave dan tekstur Unreal yang berkas aslinya berhasil dipulihkan Hoard
app-prefer-sources = WAV/PNG yang dipulihkan, bukan .uasset ({ $count })
bag-clear = Kosongkan pilihan
bag-glb-heading = Model sebagai GLB
bag-glb-text = Setiap model sebagai file .glb yang berdiri dengan Y ke atas dan menghadap ke depan, teksturnya sebagai file terpisah dalam folder textures yang dipakai bersama oleh model-model itu. Selebihnya masuk apa adanya.
bag-glb-download = Unduh zip GLB
bag-plain-heading = File apa adanya
bag-plain-text = Setiap file seperti saat diimpor, dengan yang dibutuhkannya, dalam folder untuk tiap paket.
bag-remove = Keluarkan { $name } dari pilihan
bag-more = dan { $count } lagi
app-bulk-tag-label = Tag untuk ditambahkan ke, atau dilepas dari, aset terpilih
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Tambahkan tag ini ke setiap aset terpilih
app-bulk-tag-add = Tambah
app-bulk-tag-remove-title = Lepas tag ini dari setiap aset terpilih
app-bulk-tag-remove = Lepas
app-bulk-category-label = Kategori untuk aset terpilih (kosong untuk mengosongkannya)
app-bulk-category-placeholder = Kategori
app-bulk-category-set-title = Tetapkan kategori setiap aset terpilih
app-bulk-category-clear-title = Kosongkan kategori setiap aset terpilih
app-bulk-category-set-button = Tetapkan
app-bulk-category-clear-button = Kosongkan
app-remove-from-collection = Keluarkan dari { $name }
app-unfavorite = Hapus dari favorit
app-favorite = Favoritkan
app-select-all = Pilih semua { $count }
app-archive-selected-title = Keluar dari pencarian sampai dipulihkan atau dihapus permanen, dari Arsip
app-drop-items = Jatuhkan { $count } item untuk menambahkannya
app-drop = Jatuhkan untuk menambahkan ke Hoard
app-drop-hint = Setiap berkas atau folder tingkat atas menjadi paket tersendiri. Ukuran gigabyte tidak masalah; berkas besar dilanjutkan jika koneksi terputus.

## Palet perintah (CommandPalette.vue, dengan perintahnya dari App.vue)

palette-group-go-to = Buka
palette-group-actions = Tindakan
palette-group-theme = Tema
palette-group-kinds = Jenis
palette-group-packs = Paket
palette-group-collections = Koleksi
palette-group-projects = Proyek
palette-group-saved = Pencarian tersimpan
palette-group-assets = Aset
palette-hint-current = saat ini
palette-library = Pustaka
palette-library-keywords = aset kisi beranda assets grid home
palette-favorites = Favorit
palette-favorites-keywords = bintang disukai starred stars liked
palette-packs = Paket
palette-archive = Arsip
palette-archive-keywords = sampah tempat sampah dihapus pulihkan trash recycle bin deleted restore
palette-stats = Statistik pustaka
palette-stats-keywords = statistik angka ukuran ruang disk jumlah statistics numbers size disk space count
palette-search = Cari
palette-add-files = Tambah berkas…
palette-add-files-keywords = unggah impor upload import
palette-import-folder = Impor folder…
palette-import-folder-keywords = unggah paket upload pack
palette-select-all = Pilih semua { $count } yang ditampilkan
palette-select-all-keywords = centang check
palette-select-several = Pilih beberapa…
palette-select-several-keywords = massal banyak tag arsip bulk multiple archive
palette-archive-selected = Arsipkan { $count } yang dipilih
palette-archive-selected-keywords = hapus buang sampah delete remove trash
palette-clear-selection = Kosongkan pilihan
palette-clear-filters = Hapus filter
palette-clear-filters-keywords = setel ulang reset
palette-hide-supporting = Sembunyikan berkas pendukung
palette-show-supporting = Tampilkan berkas pendukung
palette-supporting-keywords = normal map roughness mask normal maps roughness masks
palette-account = Akun dan token akses
palette-account-keywords = kata sandi masuk pengguna token api password login user
palette-sign-out = Keluar ({ $name })
palette-sign-out-keywords = keluar log out logout
palette-sign-in = Masuk
palette-sign-in-keywords = masuk akun kata sandi log in login account password
palette-settings = Pengaturan
palette-settings-keywords = blender ffmpeg llm ai penandaan tag unreal admin alat preferensi tagging tools preferences
palette-shortcuts = Pintasan keyboard
palette-shortcuts-keywords = bantuan tombol help keys
palette-theme-keywords = tema tampilan mode theme appearance
palette-theme-variant-keywords = tampilan warna appearance color
palette-light-theme = Tema terang: { $name }
palette-dark-theme = Tema gelap: { $name }
palette-kind-all = Semua jenis (berhenti menampilkan hanya { $kind })
palette-kind-only = Hanya { $kind }
palette-title = Palet perintah
palette-placeholder = Cari aset, buka paket, jalankan perintah…
palette-results = Hasil
palette-searching = Mencari…
palette-no-match = Tidak ada yang cocok dengan “{ $query }”
palette-foot-move = pindah
palette-foot-run = jalankan
palette-foot-shortcuts = semua pintasan

## Pintasan keyboard (ShortcutsDialog.vue)

shortcuts-title = Pintasan keyboard
shortcuts-close = Tutup
shortcuts-anywhere = Di mana saja
shortcuts-palette = Palet perintah: cari, lompat, jalankan
shortcuts-search = Cari
shortcuts-this-list = Daftar ini
shortcuts-escape = Tutup, kosongkan pilihan, atau tutup detail
shortcuts-library = Pustaka
shortcuts-move = Berpindah antarubin (atau J dan K)
shortcuts-open = Buka detail
shortcuts-play-sound = Putar atau hentikan suara
shortcuts-select = Pilih atau batalkan pilihan
shortcuts-favorite = Tambahkan ke atau keluarkan dari favorit Anda
shortcuts-key-click = klik
shortcuts-select-range = Pilih rentang
shortcuts-select-all = Pilih semua yang ditampilkan
shortcuts-sound-player = Pemutar suara
shortcuts-play-pause = Putar atau jeda
shortcuts-skip = Mundur atau maju
shortcuts-start-end = Awal atau akhir
shortcuts-image-inspector = Pemeriksa gambar
shortcuts-zoom = Perbesar atau perkecil
shortcuts-fit = Sesuaikan dengan jendela
shortcuts-actual-pixels = Piksel sebenarnya
shortcuts-tile = Ubin 3×3 untuk memeriksa sambungan
shortcuts-background = Latar berikutnya
shortcuts-channel = Tampilkan satu kanal

## KindChips.vue

kinds-group = Jenis aset

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = ditambahkan oleh orang
tag-source-rule = dari pemberi tag offline
tag-source-meta = diukur dari berkas
tag-source-ai = disarankan oleh AI
tag-remove = Hapus tag { $tag }

## ActivityIndicator.vue

activity-label-idle = Aktivitas latar belakang: diam
activity-label-running = Aktivitas latar belakang: { $running } berjalan
activity-label-queued = Aktivitas latar belakang: { $running } berjalan, { $queued } dalam antrean
activity-title = Aktivitas latar belakang
activity-unreachable = Tidak dapat menghubungi server
activity-unreachable-desktop = Hoard tidak merespons
activity-empty = Tidak ada yang berjalan. Impor, pemberian tag, konversi, dan unduhan zip muncul di sini selagi diproses.
activity-queued = dalam antrean
activity-seconds = { $s } dtk
activity-minutes = { $min } mnt
activity-hours = { $h } j { $min } mnt
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } segitiga
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Putar { $name }
tile-pause = Jeda { $name }
tile-favorite = Favoritkan { $name }
tile-in-favorites = Ada di favorit Anda
tile-add-favorite = Tambahkan ke favorit Anda (F)
tile-more-like = Lainnya yang mirip { $name }
tile-more-like-this = Lainnya yang mirip ini
tile-select = Pilih { $name }

## CollectionPicker.vue

collections-add = Tambahkan ke koleksi
collections-menu = Koleksi
collections-find-label = Cari atau buat koleksi
collections-find-placeholder = Cari atau beri nama koleksi baru
collections-create = Buat
collections-empty = Belum ada koleksi. Ketik nama untuk memulai.

## FolderPicker.vue

folders-list = Folder
folders-close = Tutup
folders-browse = Telusuri…
folders-up-title = Naik satu folder
folders-up = Naik
folders-this-computer = Komputer ini
folders-none = Tidak ada folder di sini.

## LocationForm.vue

locations-mode-question = Apa yang harus dilakukan Hoard dengan folder ini?
locations-mode-managed = Simpan pustaka saya di sini
locations-mode-managed-hint = Paket yang Anda unggah atau impor disalin ke folder ini. Hoard yang mengurusnya, jadi jangan ubah isinya.
locations-mode-watched = Indeks folder yang sudah saya punya
locations-mode-watched-hint = Setiap folder di dalamnya menjadi paket. Hoard membaca berkas di tempatnya dan tidak pernah mengubah, memindahkan, atau menghapusnya, serta mengambil folder baru saat muncul.
locations-folder = Folder di server
locations-folder-desktop = Folder
locations-folder-placeholder = Folder tempat paket Anda berada
locations-folder-hint-managed = Hoard membuatnya jika belum ada. Di Docker, ini adalah folder yang sudah Anda pasang ke dalam container.
locations-folder-hint-managed-desktop = Hoard membuatnya jika belum ada.
locations-folder-hint-watched = Di Docker, pasang dulu folder ke dalam container (hanya-baca tidak masalah), lalu pilih di sini.
locations-folder-hint-watched-desktop = Hoard hanya membacanya: tidak ada isinya yang diubah.
locations-folder-hint-cloud = Folder yang disinkronkan oleh Dropbox, Google Drive, OneDrive, atau iCloud Drive juga bisa, begitu pula s3://bucket-name untuk bucket di penyimpanan S3.
locations-name = Nama
locations-name-placeholder-managed = Pustaka
locations-name-placeholder-watched = Paket saya
locations-files = Berkas yang disimpan di sini
locations-catch-all = Juga semua yang tidak diterima lokasi lain
locations-kinds-hint = Berkas paket baru disimpan di lokasi yang mencentang jenisnya. Sisanya ke lokasi penampung.
locations-files-all = Semuanya, untuk saat ini. Tambahkan lokasi lain nanti dari Pengaturan untuk mengirim jenis tertentu (misalnya musik) ke disk lain.
locations-first-watched-note = Mengunggah memerlukan pustaka yang disimpan Hoard. Anda dapat menambahkannya nanti dari Pengaturan → Lokasi pustaka.
locations-checking = Memeriksa folder…
locations-add = Tambah lokasi
locations-cancel = Batal

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Tidak dapat memuat lokasi pustaka
locations-takes-and-rest = { $kinds }, dan semua yang lain
locations-takes-rest = Semua yang tidak diterima lokasi lain
locations-takes-everything = Semuanya
locations-takes-nothing = Belum ada
locations-save-failed = Tidak dapat menyimpan lokasi
locations-unwatch-title = Berhenti memantau { $name }?
locations-unwatch-body = { $count } paketnya keluar dari Hoard, beserta tag dan catatannya. Berkas di folder tidak disentuh.
locations-unwatch-body-empty = Berkas di folder tidak disentuh.
locations-unwatch = Berhenti memantau
locations-remove-title = Hapus { $name }?
locations-remove-body = Semua berkas di dalamnya dipindahkan dulu ke lokasi lain, sesuai jenisnya; lokasi ini dihapus setelah kosong. Tidak ada yang hilang.
locations-remove-body-only = Jika lokasi ini berisi berkas, tambahkan dulu lokasi lain sebagai tujuan pemindahan.
locations-remove = Hapus
locations-removed = { $name } dihapus
locations-emptying = Memindahkan berkas keluar dari { $name }; lokasi ini dihapus setelah selesai
locations-offline-title = { $name } tidak dapat dijangkau
locations-offline-body = Berkasnya tidak dapat dipindahkan selagi lokasi ini offline. Tetap hapus? Berkas yang masih ada di sana akan tampil sebagai hilang.
locations-remove-anyway = Tetap hapus
locations-remove-failed = Tidak dapat menghapus { $name }
locations-rebalancing = Memindahkan berkas tersimpan ke lokasi yang menerima jenisnya
locations-rebalance-failed = Tidak dapat memindahkan berkas
locations-scanned-never = belum dipindai
locations-scanned-now = baru saja dipindai
locations-scanned-minutes = dipindai { $mins } mnt yang lalu
locations-scanned-hours = dipindai { $hours } j yang lalu
locations-scanned-date = dipindai { $date }
locations-scan-failed = Tidak dapat memindai { $name }
locations-watching = Memantau { $name }
locations-added = { $name } ditambahkan
locations-loading = Memuat…
locations-managed-title = Pustaka yang disimpan Hoard
locations-watched-title = Folder pantauan
locations-managed-hint = Unggahan dan impor disalin ke sini. Setiap lokasi menerima jenis yang dicentangnya; lokasi penampung menerima sisanya.
locations-watched-hint = Folder berisi paket yang diindeks di tempatnya. Hoard tidak pernah mengubah berkas di dalamnya, dan mencari folder baru serta yang dihapus setiap 15 menit.
locations-managed-empty = Belum ada, jadi unggahan tidak punya tempat. Tambahkan di bawah.
locations-reachable = Dapat dijangkau
locations-unreachable-now = Tidak dapat dijangkau saat ini
locations-catch-all-tag = penampung
locations-packs = { $count } paket
locations-unreachable = tidak dapat dijangkau
locations-scanning = Memindai…
locations-scan = Pindai sekarang
locations-edit = Edit
locations-catch-all-locked = Untuk mengubah ini, jadikan lokasi lain sebagai lokasi penampung
locations-edit-hint-before = Berlaku untuk berkas yang ditambahkan mulai sekarang. Untuk memindahkan yang sudah tersimpan, gunakan
locations-edit-hint-after = di bawah.
locations-save = Simpan
locations-rebalance = Pindahkan berkas tersimpan agar sesuai
locations-rebalance-hint = Setelah mengubah apa yang diterima lokasi: berkas yang sudah tersimpan dipindahkan ke lokasi tujuan jenisnya saat ini.
locations-add-title = Tambahkan lokasi
locations-add-hint = Disk lain untuk jenis berkas tertentu, atau folder berisi paket yang sudah Anda punya.
locations-s3-title = Penyimpanan S3
locations-s3-hint = Lokasi s3://bucket-name adalah bucket di layanan ini: AWS, atau layanan apa pun yang kompatibel dengan S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage di NAS).
locations-s3-endpoint = Alamat
locations-s3-endpoint-placeholder = Kosongkan untuk AWS; mis. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Wilayah
locations-s3-access-key = ID kunci akses
locations-s3-secret-key = Kunci akses rahasia
locations-s3-headers = Header tambahan
locations-s3-headers-placeholder = Name=value, satu per baris (misalnya untuk Cloudflare Access)
locations-s3-kept = Tersimpan (ketik untuk mengganti)
locations-s3-secrets = Kunci dan header disimpan bersama pengaturan server, dan tidak pernah ditampilkan lagi.
locations-s3-secrets-desktop = Kunci dan header disimpan di keychain komputer ini, dan tidak pernah ditampilkan lagi.
locations-s3-saved = Pengaturan penyimpanan S3 tersimpan
locations-s3-save-failed = Tidak dapat menyimpan pengaturan penyimpanan S3
locations-add-more = Tambahkan lokasi…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Tidak dapat memuat Masalah
issues-started =
    { $count ->
        [1] Dimulai lagi.
       *[other] { $count } hal dimulai lagi.
    }
issues-retry-failed = Tidak dapat mencoba lagi
issues-dismiss-failed = Tidak dapat mengabaikan
issues-dismiss-all-title = Abaikan semua { $count }?
issues-dismiss-all-body = “{ $title }” dikosongkan. Apa pun yang gagal lagi akan tercantum lagi.
issues-dismiss-all = Abaikan semua
issues-just-now = baru saja
issues-minutes-ago = { $mins } mnt yang lalu
issues-hours-ago = { $hours } j yang lalu
issues-loading = Memuat…
issues-none = Tidak ada yang salah.
issues-action-all = { $action } semua
issues-open-asset = Buka
issues-open-pack = Tampilkan paket
issues-open-locations = Lokasi pustaka
issues-where-before = di
issues-where-after = { "" }
issues-times = { $count } kali
issues-starting = Memulai…
issues-dismiss-named = Abaikan { $title }
issues-dismiss = Abaikan
issues-show-fewer = Tampilkan lebih sedikit
issues-show-all = Tampilkan semua { $count }
issues-not-listed = { $count } lainnya tidak tercantum.

## archive.ts

archive-anyway = Tetap arsipkan
archive-failed = Tidak dapat mengarsipkan: { $error }
archive-undo = Urungkan
archive-restore-failed = Tidak dapat memulihkan: { $error }
archive-count-packs-and-assets = { $packs } paket dan { $assets } aset
archive-count-packs = { $packs } paket
archive-count-assets = { $assets } aset
archive-count-nothing = tidak ada

## ArchiveView.vue

archive-load-failed = Tidak dapat memuat arsip: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = { $what } dipulihkan
archive-purged = { $what } dihapus permanen
archive-purge-failed = Tidak dapat menghapus: { $error }
archive-purge-title = Hapus { $name } secara permanen?
archive-purge-pack-body = { $count } asetnya dan { $size } berkas juga ikut terhapus, kecuali berkas yang dipakai bersama paket lain. Tindakan ini tidak dapat diurungkan.
archive-purge = Hapus permanen
archive-purge-asset-body = Berkasnya dibebaskan kecuali dipakai bersama aset lain. Tindakan ini tidak dapat diurungkan.
archive-empty-title = Kosongkan arsip?
archive-empty-body = { $what } akan dihapus permanen, dan berkasnya dibebaskan. Tindakan ini tidak dapat diurungkan.
archive-empty = Kosongkan arsip
archive-title = Arsip
archive-intro = Paket dan aset yang diarsipkan tidak muncul di pencarian, jumlah, dan daftar tag. Pulihkan, atau hapus permanen untuk membebaskan ruangnya.
archive-loading = Memuat…
archive-is-empty = Arsip kosong
archive-is-empty-hint = Arsipkan paket atau aset, dan itu akan menunggu di sini sampai Anda memulihkan atau menghapusnya secara permanen.
archive-packs = Paket
archive-assets = Aset
archive-asset-count = { $count } aset
archive-archived = diarsipkan { $when }
archive-from = dari { $pack }
archive-restore = Pulihkan
archive-show-more = Tampilkan lebih banyak ({ $count } tersisa)

## mine.ts

projects-favorite-add-failed = Tidak dapat menambahkan ke favorit Anda: { $error }
projects-favorite-remove-failed = Tidak dapat mengeluarkan dari favorit Anda: { $error }
projects-save-search-failed = Tidak dapat menyimpan pencarian: { $error }
projects-forget-search-failed = Tidak dapat menghapus { $name }: { $error }

## projects.ts

projects-list-and = { $items } dan { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } lainnya
projects-other = { $count } proyek lain
projects-usage-title-one = { $name } memakai ini
projects-usage-title = { $count } proyek memakai ini
projects-usage-body-one = Dipakai di { $list }: hoard sync berikutnya akan mengeluarkannya.
projects-usage-body = Dipakai di { $list }: hoard sync berikutnya di masing-masing proyek akan mengeluarkannya.

## uploader.ts

upload-network-error = galat jaringan
upload-gave-up = { $what }: { $error } setelah { $attempts } percobaan
upload-failed-plain = gagal
upload-already-imported = sudah diimpor
upload-part-failed = unggahan gagal ({ $status })
upload-batch = kelompok berisi { $count } berkas
upload-batch-failed = kelompok gagal ({ $status })
upload-finish = selesaikan unggahan

## UploadQueue.vue

upload-eta-seconds = { $s } dtk lagi
upload-eta-minutes = { $min } mnt lagi
upload-eta-hours = { $h } j lagi
upload-queue = Impor
upload-importing = Mengimpor { $count } paket
upload-bytes-of = { $sent } dari { $total }
upload-rate = { $size }/dtk
upload-waiting = Menunggu
upload-unpacking = Sudah diunggah, sekarang dibongkar
upload-already = Sudah ada di pustaka
upload-stopped = Dihentikan
upload-files-of = { $done } dari { $total } berkas
upload-resumed = dilanjutkan, { $size } sudah ada
upload-skipped = Berkas yang persis sama sudah diimpor sebagai “{ $name }”, jadi dilewati.
upload-error = { $error }. Kemajuan Anda tersimpan: coba lagi, atau jatuhkan lagi berkas yang sama nanti.
upload-retry = Coba lagi
upload-import-again = Tetap impor lagi
upload-dismiss = Abaikan
upload-cancel = Batal

## PacksView.vue

packs-rename-failed = Tidak dapat mengganti nama: { $error }
packs-collection-deleted = Koleksi { $name } dihapus
packs-delete-failed = Tidak dapat menghapus: { $error }
packs-status-uploading = Mengunggah (berkas yang sudah diterima)
packs-status-ingesting = Membongkar dan memberi tag
packs-status-analyzing = LLM sedang memberi tag
packs-status-ready = Siap
packs-status-failed = Gagal
packs-retag-failed = Tidak dapat memulai pemberian tag ulang: { $error }
packs-read-files-failed = Tidak dapat membaca berkas paket: { $error }
packs-context-load-failed = Tidak dapat memuat detail toko paket: { $error }
packs-undo-store-failed = Tidak dapat melepas halaman toko: { $error }
packs-context-save-failed = Tidak dapat menyimpan detail toko: { $error }
packs-unreal-load-failed = Tidak dapat memuat status konversi Unreal: { $error }
packs-unreal-discard-title = Buang berkas aslinya?
packs-unreal-discard-body = Berkas .uasset asli di { $name } dibuang setelah dikonversi. Data khusus Unreal di dalamnya, seperti grafik material, hilang selamanya.
packs-unreal-discard-confirm = Konversi dan buang
packs-unreal-convert-failed = Tidak dapat memulai konversi: { $error }
packs-unreal-fact-converted = { $done } dikonversi dari Unreal
packs-unreal-fact-packages = { $left } paket Unreal
packs-unreal-fact-both = { $done } dikonversi dari Unreal, { $left } masih berupa paket
packs-thumbs-canceled = Dibatalkan setelah { $done } dari { $total }
packs-thumbs-drew = { $count } model digambar
packs-thumbs-result = { $lead }: { $improved } gambar mini diganti.
packs-thumbs-result-failed = { $lead }: { $improved } gambar mini diganti, { $failed } tidak dapat digambar.
packs-archived = { $name } dan { $count } asetnya diarsipkan
packs-blobs-bad = { $count } berkas gagal dalam pemeriksaan integritas terakhirnya.
packs-collections = Koleksi
packs-browse-named = Telusuri { $name }
packs-collection-name = Nama koleksi
packs-save = Simpan
packs-cancel = Batal
packs-assets = { $count } aset
packs-updated = Diperbarui { $date }
packs-browse = Telusuri
packs-collection-download = Unduh (.zip)
packs-collection-credits = Kredit
packs-collection-credits-title = Siapa pembuat paket asal aset ini, lisensinya, dan baris kredit yang diminta
packs-rename = Ganti nama
packs-delete = Hapus
packs-packs = Paket
packs-empty = Belum ada paket. Jatuhkan zip atau folder di mana saja di halaman ini untuk menambahkannya.
packs-find = Cari paket
packs-find-placeholder = Cari paket berdasarkan nama, toko, lisensi, atau tag
packs-sort = Urutan
packs-sort-new = Terbaru dulu
packs-sort-old = Terlama dulu
packs-sort-name = Menurut nama
packs-sort-size = Terbesar dulu
packs-sort-assets = Aset terbanyak dulu
packs-with-kind = Paket berisi
packs-state = Status
packs-state-any = Status apa saja
packs-state-working = Sedang diproses
packs-state-failed = Gagal
packs-state-untagged = Belum dideskripsikan
packs-store = Toko
packs-store-any = Toko apa saja
packs-store-from = Dari { $store }
packs-store-none = Tanpa halaman toko
packs-shown-of = { $shown } dari { $total }
packs-clear = Hapus
packs-no-match = Tidak ada paket yang cocok.
packs-clear-all = Hapus pencarian dan filter
packs-pack-name = Nama paket
packs-thumbs-drawing = Menggambar gambar mini di browser ini…
packs-thumbs-drawing-count = Menggambar gambar mini di browser ini: { $done } dari { $total }…
packs-thumbs-stopped = Dihentikan: { $error }
packs-thumbs-none = Setiap model di sini sudah memiliki gambar mini yang digambar dengan semua teksturnya.
packs-dismiss = Abaikan
packs-license = Lisensi: { $license }
packs-license-tier = Lisensi: { $license } ({ $tier })
packs-license-not-found = tidak ditemukan
packs-from-before = Dari
packs-from-after = { "" }
packs-added = Ditambahkan { $date }
packs-download = Unduh paket (.zip)
packs-store-details = Detail toko
packs-more-named = Tindakan lain untuk { $name }
packs-more = Lainnya
packs-rename-more = Ganti nama…
packs-retag-rules = Jalankan ulang pemberi tag offline
packs-retag-llm = Jalankan ulang pemberi tag LLM
packs-make-thumbs = Buat gambar mini di browser ini
packs-make-thumbs-title = Buka setiap model di penampil 3D tersembunyi, satu per satu, dan pakai gambarnya sebagai gambar mini jika lebih bagus daripada buatan server
packs-make-thumbs-title-desktop = Buka setiap model di penampil 3D tersembunyi, satu per satu, dan pakai gambarnya sebagai gambar mini jika lebih bagus daripada buatan Hoard
packs-read-files = Baca lagi berkas paket
packs-credits = Unduh kredit
packs-credits-title = CREDITS.md: siapa pembuat paket, lisensinya, dan baris kredit yang diminta
packs-unreal-open = Konversi konten Unreal…
packs-archive = Arsipkan paket
packs-unreal-intro = Hoard membaca sendiri paket Unreal dan menyimpan isinya sebagai berkas biasa: tekstur sebagai PNG atau EXR, suara sebagai WAV, material sebagai JSON. Berkas hasil konversi menggantikan paket Unreal itu, dengan tetap mempertahankan tag, tautan, dan koleksinya.
packs-unreal-converted =
    { $count ->
       *[other] dikonversi
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] dikonversi, { $kept } dengan berkas asli tetap disimpan
    }
packs-unreal-pending =
    { $count ->
       *[other] belum diperiksa
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] menunggu Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] tanpa format umum
    }
packs-unreal-failed =
    { $count ->
       *[other] gagal
    }
packs-unreal-reasons = Mengapa paket tersisa
packs-unreal-editor-before = Mesh dan animasi memerlukan Unreal Editor. Di mesin yang terpasang Unreal, jalankan
packs-unreal-editor-command = hoard unreal-worker --editor <path ke UnrealEditor-Cmd>
packs-unreal-editor-after = ; perintah itu mengekspornya sebagai FBX dan mengunggah hasilnya ke sini.
packs-unreal-discard = Buang berkas .uasset asli setelah dikonversi
packs-unreal-discard-hint = Membebaskan ruang. Data khusus Unreal, seperti grafik material, hilang.
packs-unreal-drop = Hapus paket Unreal tanpa format umum
packs-unreal-drop-hint = Blueprint, level, dan aset lain yang khusus engine.
packs-unreal-convert = Konversi
packs-context-hint-before = Asal paket dan apa kata penjualnya tentang paket itu. Pemberi tag LLM membaca ini bersama setiap berkas, jadi uraian toko seperti "120 ice and fire spells" membantunya menamai
packs-context-hint-after = .
packs-store-auto = Ditemukan otomatis: paket itu sendiri menyebutkan halaman ini.
packs-store-not-this = Bukan yang ini
packs-lookup-label = Cari di toko
packs-lookup-placeholder = Nama paket di toko
packs-lookup-searching = Mencari…
packs-lookup-search = Cari di toko
packs-lookup-hide = Sembunyikan
packs-lookup-clues = Dari berkas paket: { $clues }.
packs-lookup-also = Juga mencari { $queries }.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Tidak ada yang mirip di Fab, itch.io, Kenney, atau OpenGameArt. Coba kata yang lebih sedikit atau berbeda.
packs-lookup-use = Pakai
packs-lookup-searched = Dicari { $date }.
packs-lookup-again = Cari lagi
packs-lookup-again-after = .
packs-lookup-unreachable = Tidak dapat menghubungi { $stores } kali ini.
packs-lookup-also-look = Coba juga di
packs-lookup-links-and = { " " }dan{ " " }
packs-lookup-browser-only = (hanya dapat dicari di browser Anda); tempel alamat halamannya di bawah.
packs-lookup-open = Cari di toko…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Penjual
packs-seller-placeholder = Siapa pembuatnya
packs-license-label = Lisensi
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Tingkat lisensi
packs-license-tier-placeholder = Personal, Professional…
packs-store-page = Halaman toko
packs-store-text = Deskripsi dari toko
packs-store-text-placeholder = Tempel teks listingnya, atau biarkan kosong untuk mengambilnya dari halaman toko.
packs-store-text-fetch = Menyimpan akan mengambil deskripsi dari halaman toko.
packs-readme = README yang ditemukan di paket
packs-save-retag = Simpan dan jalankan ulang pemberi tag LLM

## AssetDetail.vue

asset-lazy-3d-viewer = penampil 3D
asset-lazy-text = teks
asset-lazy-text-viewer = penampil teks
asset-lazy-image-viewer = penampil gambar
asset-lazy-material-preview = pratinjau material
asset-lazy-material-viewer = penampil material
asset-lazy-sprite-player = pemutar sprite
asset-lazy-sheet-view = penampil lembar sprite
asset-rel-texture = tekstur
asset-rel-buffer = data geometri
asset-rel-material = berkas material
asset-rel-animation = animasi
asset-rel-library = berkas tertaut
asset-rel-guess = mungkin terkait
asset-rel-sidecar = detail toko
asset-rel-variant = varian
asset-rel-sprite = sprite
asset-role-map = map { $role }
asset-maps-count = { $count } map
asset-needs = Memerlukan
asset-animations = Animasi
asset-sprites = Sprite
asset-used-by = Dipakai oleh
asset-loose-sprites = Juga sebagai file terpisah
asset-in-sheet = Di lembar sprite
asset-other-formats = Format lain
asset-shown-as = Ditampilkan di pustaka sebagai
asset-conversion-failed = konversi gagal
asset-as-converting = Mengonversi…
asset-as-converting-percent = Mengonversi… { $percent }%
asset-as-queued = Dalam antrean…
asset-download = Unduh
asset-as-label = Unduh sebagai
asset-as-original = { $format } (format asli)
asset-as-pack-files = File milik paket
asset-as-size = Ukuran
asset-as-full-size = Ukuran penuh
asset-as-up-to = Hingga { $size } px
asset-as-hint = Pilih ukuran atau format lain
asset-meta-size = Ukuran
asset-meta-length = Durasi
asset-meta-sample-rate = Laju sampel
asset-meta-channels = Kanal
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Kedalaman bit
asset-meta-bits = { $bits }-bit
asset-meta-level = Level
asset-meta-level-value = puncak { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = puncak { $peak } dBFS, RMS { $rms } dBFS, terpotong (clipping)
asset-meta-loop = Loop
asset-meta-loop-value = { $start } sampai { $end }
asset-meta-playback = Pemutaran
asset-meta-one-shot = Sekali putar
asset-meta-envelope = Selubung
asset-meta-envelope-times = serangan { $attack }, ekor { $tail }
asset-meta-envelope-hit = pukulan
asset-meta-envelope-swell = membesar
asset-meta-envelope-sustained = bertahan
asset-meta-tone = Warna nada
asset-meta-tone-value = kecerahan { $khz } kHz, derau { $noise }%
asset-meta-pitch = Tinggi nada
asset-meta-pitch-rises = naik { $octaves } oktaf
asset-meta-pitch-falls = turun { $octaves } oktaf
asset-meta-loops-cleanly = Berulang mulus
asset-meta-tempo = Tempo
asset-meta-root-note = Nada dasar
asset-meta-cue-markers = Penanda cue
asset-meta-library = Pustaka
asset-meta-keywords = Kata kunci
asset-meta-title = Judul
asset-meta-artist = Artis
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Generator
asset-meta-everything-with = Tampilkan semua yang bertag { $tag }
asset-meta-saved-with = Disimpan dengan
asset-meta-color-mode = Mode warna
asset-meta-layers = Layer
asset-meta-range = Rentang
asset-meta-hdr = HDR (pratinjau di-tone-map)
asset-meta-format = Format
asset-meta-large-document = Dokumen besar (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Dibuat dengan AI generatif
asset-meta-ai-edited = Diedit dengan AI generatif
asset-meta-ai-how-metadata = metadata
asset-meta-ai-how-generator = pengaturan generator
asset-meta-ai-how-iptc = jenis sumber digital IPTC
asset-meta-ai-says = { $made }, menurut { $how }-nya
asset-meta-ai-by-says = { $made } ({ $by }), menurut { $how }-nya
asset-meta-provenance = Asal-usul
asset-meta-c2pa = Memiliki C2PA Content Credentials
asset-meta-compression = Kompresi
asset-meta-note = Catatan
asset-meta-maps = Map
asset-meta-sizes =
    { $count ->
       *[other] Ukuran
    }
asset-meta-triangles = Segitiga
asset-meta-vertices = Verteks
asset-meta-meshes = Mesh
asset-meta-materials = Material
asset-meta-animations = Animasi
asset-meta-skins = Skin
asset-meta-unreal-asset = Aset Unreal
asset-meta-unreal-saved-with = { $class }, disimpan dengan UE { $engine }
asset-meta-package = Paket Unreal
asset-meta-cooked = Cooked (build game): pratinjau dan sumber dihapus
asset-meta-tiling = Tiling
asset-meta-seamless = Mulus di kedua sumbu
asset-meta-analysis-error = Tidak dapat membaca berkas ini sepenuhnya: { $error }
asset-copy-blocked = Tidak dapat menyalin: browser memblokirnya
asset-save-error = Tidak dapat menyimpan: { $error }
asset-tag-removed = { $tag } dihapus
asset-undo = Urungkan
asset-play-error = Tidak dapat memutar berkas ini: { $error }
asset-copied-hash = Hash disalin
asset-copied-toml = toml disalin
asset-copied-id = ID disalin
asset-copied-link = Tautan disalin
asset-origin-before = Berkas ini dulunya
asset-origin-class-engine = , sebuah { $class } yang disimpan oleh UE { $engine }.
asset-origin-class = , sebuah { $class }.
asset-origin-engine = { " " }yang disimpan oleh UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor mengekspornya pada { $date }.
asset-origin-editor-notes = Unreal Editor mengekspornya pada { $date } ({ $notes }).
asset-origin-hoard = Hoard membacanya dari paket Unreal pada { $date }.
asset-origin-hoard-notes = Hoard membacanya dari paket Unreal pada { $date } ({ $notes }).
asset-origin-download = Unduh .{ $ext } asli
asset-origin-discarded = Paket Unreal asli dibuang setelah dikonversi.
asset-details = Detail aset
asset-in-pack = di
asset-previous = Aset sebelumnya
asset-next = Aset berikutnya
asset-favorite = Favorit
asset-favorite-on-title = Ada di favorit Anda (F untuk mengeluarkannya)
asset-favorite-off-title = Tambahkan ke favorit Anda (F)
asset-share = Bagikan
asset-share-title = Bagikan tautan ke aset ini (siapa pun yang membukanya masuk ke Hoard terlebih dahulu)
asset-layout-panel = Tampilkan di panel samping
asset-layout-page = Tampilkan satu halaman penuh
asset-layout-page-title = Tampilkan satu halaman penuh: pratinjau besar di samping detail
asset-close = Tutup detail
asset-rename-title = Klik untuk mengganti nama
asset-name = Nama
asset-views-alt = Model dari empat arah
asset-inspect-aria = Periksa { $name } dalam layar penuh
asset-inspect-title = Periksa: zoom, ubin, kanal
asset-playback-position = Posisi pemutaran
asset-playback-value = { $current } dari { $duration }
asset-pause = Jeda
asset-play = Putar
asset-waveform-drawing = Menggambar bentuk gelombang…
asset-waveform-skipped = Tidak ada bentuk gelombang untuk berkas sebesar ini
asset-play-animation = Putar sebagai animasi
asset-show-views = Tampilkan lembar empat sisi yang dilihat pemberi tag
asset-palette = Warna dalam gambar
asset-swatch-title = { $hex }, { $percent }% dari gambar. Cari aset dengan warna ini
asset-swatch-aria = Cari aset dengan { $hex }, { $percent }% dari gambar ini
asset-more-like-this = Lainnya yang mirip ini
asset-desc-label = Deskripsi Anda
asset-desc-placeholder = Apa ini, di mana Anda akan memakainya…
asset-save = Simpan
asset-cancel = Batal
asset-desc-family = Disimpan ke semua { $count } varian
asset-desc-edit = Edit deskripsi
asset-desc-write = Tulis deskripsi
asset-desc-ai = Ditulis oleh AI
asset-desc-file = Dari berkas
asset-desc-store-before = Dari
asset-desc-store-page = halaman { $store }
asset-desc-store-page-unnamed = halaman toko
asset-desc-store-after = (seluruh paket)
asset-less = Lebih sedikit
asset-more = Selengkapnya
asset-tags = Tag
asset-tag-add-label = Tambahkan tag
asset-tag-add-placeholder = Tambahkan tag, mis. mood:tense
asset-tag-add = Tambah
asset-apply-family = Terapkan perubahan tag, kategori, dan deskripsi ke semua { $count } varian
asset-collections = Koleksi
asset-collection-remove = Keluarkan dari { $name }
asset-collection-add-another = Tambahkan ke koleksi lain
asset-collection-add = Tambahkan ke koleksi
asset-used-in = Dipakai di
asset-used-in-title = Tampilkan isi { $name }
asset-used-in-synced = disinkronkan { $when }
asset-maps = Map
asset-show-more = Tampilkan { $count } lagi
asset-materials-heading =
    { $count ->
       *[other] Material
    }
asset-material-maps-aria = { $name }: map
asset-variants = Varian ({ $count } versi)
asset-copies = Salinan identik
asset-copies-note = Byte yang sama disimpan sekali; paket-paket ini juga menyertakan berkas ini.
asset-unreal-converted = Dikonversi dari Unreal
asset-rule-converted = Dikonversi oleh aturan
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Aturan konversi membuat { $format } ini pada { $date } dari
asset-rule-kept-until = Berkas asli disimpan sampai { $date }.
asset-rule-put-back = Kembalikan berkas asli
asset-rule-put-back-title = Kembalikan { $file }?
asset-rule-put-back-body = Berkas hasil konversi dihapus dan berkas asli kembali ke tempatnya, beserta tag, koleksi, dan tautannya. Sejak itu aturan konversi tidak akan mengubahnya lagi.
asset-rule-put-back-done = { $file } sudah dikembalikan
asset-rule-let-go = Berkas asli dihapus setelah masa simpan berkas asli yang diatur di Pengaturan habis.
asset-unreal-package = Dari paket Unreal
asset-ue-needs-editor-before = Menunggu Unreal Editor: { $reason }. Di mesin yang terpasang Unreal, jalankan
asset-ue-needs-editor-after = dan hasilnya kembali sebagai FBX.
asset-ue-editor-path = path ke UnrealEditor-Cmd
asset-ue-unsupported = Disimpan sebagai paket Unreal: { $reason }.
asset-ue-failed = Konversi gagal: { $reason }
asset-recovered = Berkas .{ $ext } asli tersimpan di dalam paket Unreal ini, dan Hoard mengeluarkannya.
asset-recovered-download = Unduh .{ $ext }
asset-ue-meshes-before = Mesh dan animasi disimpan dalam format khusus Unreal. Konversi paketnya (halaman paket, Lainnya), lalu jalankan
asset-ue-meshes-after = di samping instalasi Unreal Editor untuk mendapatkan FBX.
asset-ue-registry-tags = { $count } tag asset registry
asset-file = Berkas
asset-bytes = Byte
asset-source = Sumber
asset-path = Path
asset-path-open-folder = Lihat apa lagi yang ada di { $folder }
asset-download-maps = Unduh semua { $count } map
asset-download-linked = Unduh bersama { $count } berkas tertaut
asset-download-only-title = Hanya berkas ini, tanpa berkas yang dipakainya
asset-download-only = Hanya berkas ini
asset-include-animations = Sertakan animasi dalam .zip
asset-copy-toml = Salin entri hoard.toml
asset-copy-id = Salin ID
asset-teach = Ajari Hoard berkas ini
asset-teach-title = Jelaskan berkas toko ini agar Hoard dapat membacanya (Pengaturan → Berkas toko)
asset-archive = Arsipkan
asset-archive-title = Keluar dari pencarian sampai dipulihkan atau dihapus permanen, dari Arsip

## ImageViewer.vue

image-channel-red = Kanal merah
image-channel-green = Kanal hijau
image-channel-blue = Kanal biru
image-channel-alpha = Kanal alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Latar kotak-kotak (K untuk berganti)
image-bg-black-title = Latar hitam (K untuk berganti)
image-bg-gray-title = Latar abu-abu (K untuk berganti)
image-bg-white-title = Latar putih (K untuk berganti)
image-bg-checker = latar kotak-kotak
image-bg-black = latar hitam
image-bg-gray = latar abu-abu
image-bg-white = latar putih
image-inspect-aria = Periksa { $name }
image-preview-size = pratinjau { $width }×{ $height }, { $bytes }
image-preview = pratinjau
image-resolution = Resolusi
image-full-title = Muat gambar penuh { $width }×{ $height } (F); memperbesar melebihi pratinjau juga memuatnya
image-full-loading = Memuat resolusi penuh…
image-full-failed = Resolusi penuh tidak termuat: coba lagi
image-full = Resolusi penuh
image-zoom = Zoom
image-zoom-out-title = Perkecil (−)
image-zoom-out = Perkecil
image-zoom-in-title = Perbesar (+)
image-zoom-in = Perbesar
image-fit-title = Sesuaikan (0)
image-fit = Sesuaikan
image-actual-title = Piksel sebenarnya (1)
image-view = Tampilan
image-tile-title = Ubin 3×3 untuk memeriksa sambungan (T)
image-tile = Ubin
image-channels = Kanal
image-close = Tutup (Esc)
image-failed = Gambar ini tidak dapat dimuat.
image-loading = Memuat…
image-key-gestures = Gulir atau cubit untuk zoom · seret untuk menggeser · klik dua kali untuk memperbesar atau menyesuaikan
image-key-tile = ubin
image-key-background = latar
image-key-full = resolusi penuh
image-key-channels = kanal

## SpritePlayer.vue

sprite-frame-aria = { $name }, bingkai { $frame } dari { $count }
sprite-failed = Yang ini tidak dapat diputar.
sprite-loading = Memuat bingkai…
sprite-previous = Bingkai sebelumnya
sprite-next = Bingkai berikutnya
sprite-pause = Jeda animasi
sprite-play = Putar animasi
sprite-animation = Animasi
sprite-all-frames = Semua bingkai
sprite-grid-title = Cara lembar dipotong menjadi bingkai
sprite-columns = Kolom
sprite-rows = Baris
sprite-fps-aria = Bingkai per detik
sprite-fps = fps

## Penampil lembar sprite (SheetView.vue) dan format unduhannya (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
       *[other] { $count } sprite
    }
sheet-pages =
    { $count ->
       *[other] { $count } halaman
    }
sheet-animations =
    { $count ->
       *[other] { $count } animasi
    }
# The box that narrows the list of sprites by name.
sheet-find = Cari sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Menampilkan { $shown } dari { $total }
sheet-no-match = Tidak ada sprite yang namanya memuat itu.
sheet-failed = Lembar ini tidak dapat dibaca.
sheet-loading = Memuat lembar…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Format milik Hoard sendiri: halaman apa adanya, sprite diputar ke arah mana pun dan dipangkas, dengan animasi dan durasi tiap bingkai.
sheet-note-texturepacker = Sprite diputar searah jarum jam dan dipangkas, dengan animasi menurut nama, seperti yang dibaca PixiJS dan Phaser.
sheet-note-clockwise = Sprite diputar searah jarum jam dan dipangkas. Format ini tidak dapat menyimpan animasi: bingkai dikenali dari nama bernomornya.
sheet-note-libgdx = Sprite diputar berlawanan arah jarum jam dan dipangkas. Format ini tidak dapat menyimpan animasi: bingkai dikenali dari nama bernomornya.
sheet-note-godot-frames = Sprite tidak pernah diputar, bagian yang dipangkas disimpan sebagai margin, dengan animasi dan durasi tiap bingkai.
sheet-note-godot-tileset = Ubin tidak pernah diputar atau dipangkas, ditata pada kisi seukuran ubin, dan gambar dibiarkan apa adanya jika sudah berada pada kisi.
sheet-note-grid = Setiap sprite tegak pada seluruh kanvasnya, dalam sel yang semuanya berukuran sama, untuk apa pun yang memotong lembar menjadi kisi.
sheet-note-frames = Setiap sprite tegak pada seluruh kanvasnya, masing-masing sebagai PNG yang diberi nama sesuai sprite-nya.

## TextViewer.vue, TextPreview.vue dan text.ts

text-lang-config = Konfigurasi
text-lang-table = Tabel
text-lang-code = Kode
text-lang-script = Skrip
text-lang-text = Teks
text-copy-blocked = Tidak dapat menyalin: browser memblokirnya
text-read-aria = Baca { $name }
text-lines = { $count } baris
text-show-as = Tampilkan sebagai
text-tree-title = Pohon yang dapat dilipat (T)
text-tree = Pohon
text-pretty-title = Diindentasi ulang (P)
text-pretty = Rapi
text-raw-title = Seperti yang ditulis (R)
text-raw = Mentah
text-document-title = Sebagai dokumen (D)
text-document = Dokumen
text-source-title = Markdown seperti yang ditulis (R)
text-source = Sumber
text-fold = Lipat
text-expand-all-title = Buka semua lipatan
text-expand-all = Bentangkan semua
text-collapse-all-title = Lipat semuanya
text-collapse-all = Ciutkan semua
text-group-text = Teks
text-wrap-title = Bungkus baris panjang (W)
text-wrap = Bungkus
text-copy-title = Salin teks (C)
text-copied = Disalin
text-copy = Salin
text-download-title = Unduh berkas
text-download = Unduh
text-close = Tutup (Esc)
text-truncated = Menampilkan { $shown } pertama dari { $total }.
text-truncated-json = Menampilkan { $shown } pertama dari { $total } (pohon memerlukan seluruh berkas).
text-show-all = Tampilkan semua
text-json-error = JSON ini tidak dapat diurai: { $error }
text-contents-aria = Isi { $name }
text-loading = Memuat…
text-load-error = Tidak dapat memuat berkas ini: { $error }
text-binary = Berkas ini biner, bukan teks, jadi tidak ada yang bisa dibaca di sini. Unduh untuk membukanya dengan alatnya sendiri.
text-key-tree = pohon
text-key-pretty = rapi
text-key-raw = mentah
text-key-document = dokumen
text-key-source = sumber
text-key-wrap = bungkus
text-key-copy = salin
text-key-find = cari
text-key-close = tutup
text-preview-aria = Baca { $name } dalam layar penuh
text-preview-read = Baca

## JsonTree.vue dan JsonPicker.vue

json-items = { $count } item
json-keys = { $count } key
json-fold = Lipat
json-unfold = Buka lipatan
json-unfold-summary = Buka lipatan { $summary }
json-show-more = Tampilkan { $count } lagi
json-left = dari { $count } tersisa
json-picker-more = … { $count } lagi

## Penampil model 3D (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = penampil 3D tidak termuat ({ $error }); memuat ulang halaman mungkin membantu
model-loading-viewer = Memuat penampil 3D…
model-loading = Memuat model…
model-show-failed = Tidak dapat menampilkan model ini: { $error }
model-preview-failed = Tidak dapat menampilkan pratinjau model ini: { $error }
model-error-converting = server masih mengonversi model ini
model-error-converting-desktop = Hoard masih mengonversi model ini
model-error-empty-collada = berkas Collada kosong
model-error-no-loader = Tidak ada pemuat browser untuk .{ $ext }
model-dialog = Lihat { $name } dalam 3D
model-title-stats = { $triangles } segitiga · { $meshes } mesh · { $materials } material
model-triangles = { $count } segitiga
model-group-camera = Kamera
model-group-shading = Shading
model-group-background = Latar
model-group-animation = Animasi
model-group-view = Tampilan
model-group-show = Tampilkan
model-frame-title = Bingkai model (F)
model-frame = Bingkai
model-reset-title = Kembali ke tampilan awal (R)
model-reset = Setel ulang
model-turn-title = Putar perlahan (A)
model-turn = Putar
model-textures-title = Tekstur (T)
model-textures = Tekstur
model-full-res-title = Tekstur resolusi penuh (H): setiap berkas tekstur apa adanya, sebesar apa pun; jika mati, pratinjau ringan paling besar 2048 px
model-full-res = Res. penuh
model-light-title = Cahaya lingkungan (E)
model-light = Cahaya
model-grid-title = Kisi lantai (G)
model-grid = Kisi
model-bg-checker = Latar kotak-kotak
model-bg-dark = Latar gelap
model-bg-gray = Latar abu-abu
model-bg-light = Latar terang
model-bg-env = Latar lingkungan
model-bg-title = { $background } (K untuk berganti)
model-pause-title = Jeda (Spasi)
model-play-title = Putar (Spasi)
model-pause = Jeda
model-play = Putar
model-animation-clip = Klip animasi
model-animation = Animasi
model-plays-on = Diputar pada
model-close-title = Tutup (Esc)
model-full-screen = Layar penuh
model-open-full-screen = Buka penampil 3D layar penuh
model-menu-fold = Lipat menu
model-menu-show = Tampilkan menu
model-view-prefix = Tampilan:{ " " }
model-menu-render = Render
model-menu-maps = Map
model-menu-inspect = Periksa
model-mode-final = Render akhir
model-mode-raw = Render mentah
model-mode-color = Warna dasar
model-mode-roughness = Roughness
model-mode-metalness = Metalness
model-mode-normal = Normal
model-mode-ao = Occlusion
model-mode-matcap = Matcap
model-mode-uv = Pemeriksa UV
model-mode-geometry = Geometri
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Wireframe (W), di atas tampilan apa pun
model-wireframe = Wireframe
model-info-title = Informasi adegan (I)
model-info = Informasi adegan
model-info-close = Tutup informasi adegan
model-info-close-title = Tutup (I)
model-info-geometries = Geometri
model-info-materials = Material
model-info-textures = Tekstur
model-info-triangles = Segitiga
model-info-vertices = Verteks
model-info-meters = glTF memakai satuan meter
model-info-no-units = Format ini tidak menyebutkan satuannya
model-info-height = Tinggi
model-info-length = Panjang
model-info-width = Lebar
model-length-units = { NUMBER($n, maximumSignificantDigits: 3) } unit
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Tekstur tidak ditemukan: { $name }
model-missing-some = { $count } tekstur tidak ditemukan: { $names }
model-missing-more = { $count } tekstur tidak ditemukan: { $names }…
model-missing-count = { $count } tekstur tidak ditemukan
model-guessed = Tekstur dicocokkan berdasarkan nama; berkas model tidak menyebutkan satu pun.
model-group-facing = Arah hadap
model-facing-turn = Putar seperempat
model-facing-turn-title = Putar model seperempat putaran berlawanan arah jarum jam, dilihat dari atas
model-facing-tip = Miringkan seperempat
model-facing-tip-title = Miringkan bagian atas model seperempat putaran ke depan, untuk menegakkan model yang berbaring
model-facing-save = Pakai untuk paket ini
model-facing-save-title = Putar semua model di { $pack } seperti ini. Gambar mininya digambar ulang; unduhan tetap seperti saat dibuat.
model-facing-undo = Urungkan
model-facing-undo-title = Kembali ke arah model paket saat ini
model-facing-saved = Semua model di { $pack } kini menghadap ke arah ini. Gambar mininya sedang digambar ulang.
model-facing-failed = Tidak dapat memutar model paket: { $error }
model-keys-mouse = Seret untuk mengorbit · seret klik kanan untuk menggeser · gulir untuk zoom · klik dua kali untuk membingkai
model-key-frame = bingkai
model-key-reset = setel ulang
model-key-view = tampilan
model-key-wireframe = wireframe
model-key-info = informasi adegan
model-key-textures = tekstur
model-key-full-res = tekstur res. penuh
model-key-light = cahaya
model-key-grid = kisi
model-key-background = latar
model-key-play = putar

## Material (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Warna
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Roughness
material-role-gloss = Gloss
material-role-metalness = Metalness
material-role-ao = Ambient occlusion
material-role-orm = Occlusion, roughness, metal
material-role-mra = Metal, roughness, occlusion
material-role-rma = Roughness, metal, occlusion
material-role-height = Height
material-role-bump = Bump
material-role-cavity = Cavity
material-role-specular = Spekular
material-role-opacity = Opasitas
material-role-emissive = Emisif
material-role-translucency = Translusensi
material-role-base-color-name = warna
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = roughness
material-role-gloss-name = gloss
material-role-metalness-name = metalness
material-role-ao-name = ambient occlusion
material-role-orm-name = occlusion, roughness, metal
material-role-mra-name = metal, roughness, occlusion
material-role-rma-name = roughness, metal, occlusion
material-role-height-name = height
material-role-bump-name = bump
material-role-cavity-name = cavity
material-role-specular-name = spekular
material-role-opacity-name = opasitas
material-role-emissive-name = emisif
material-role-translucency-name = translusensi
material-maps = { $count } map
material-dialog = Lihat material { $name }
material-up-to = hingga { $size }
material-repeat-size = { $size } per ulangan
material-shape-sphere = Bola
material-shape-cube = Kubus
material-shape-plane = Bidang
material-shape-title = { $shape } ({ $key })
material-group-shape = Bentuk
material-group-tiling = Tiling
material-real-title = Ukuran nyata: satu ulangan { $size }, diameter bola 2 m (T untuk berganti)
material-repeat-title = Ulangi { $n }× (T untuk berganti)
material-real = Nyata
material-repeat = { $n }×
material-height-title = Seberapa jauh height map mendorong permukaan
material-height = Height
material-height-strength = Kekuatan height
material-loading = Memuat material…
material-show-failed = Tidak dapat menampilkan material ini: { $error }
material-preview-failed = Tidak dapat menampilkan pratinjau material ini: { $error }
material-group-maps = Map
material-map-show = Tampilkan map { $map }
material-map-hide = Sembunyikan map { $map }
material-map-unused = Tidak dipakai oleh pratinjau
material-open-full-screen = Buka penampil material layar penuh
material-keys-mouse = Seret untuk mengorbit · seret klik kanan untuk menggeser · gulir untuk zoom · klik map untuk mematikannya
material-key-shape = bentuk
material-key-tiling = tiling
material-key-turn = putar

## theme.ts

theme-slate = Batu Sabak
theme-paper = Kertas
theme-linen = Linen
theme-sage = Sage
theme-lilac = Lila
theme-blush = Merona
theme-graphite = Grafit
theme-obsidian = Obsidian
theme-ember = Bara
theme-moss = Lumut
theme-aubergine = Terung
theme-oxblood = Merah Hati
theme-light-slate-blurb = Biru keabuan yang sejuk
theme-light-paper-blurb = Abu-abu netral polos
theme-light-linen-blurb = Krem hangat, amber
theme-light-sage-blurb = Hijau lembut
theme-light-lilac-blurb = Ungu muda pucat
theme-light-blush-blurb = Merah mawar kusam
theme-dark-graphite-blurb = Abu-abu netral, tanpa rona
theme-dark-slate-blurb = Biru keabuan
theme-dark-obsidian-blurb = Hampir hitam, untuk OLED
theme-dark-ember-blurb = Arang hangat, amber
theme-dark-moss-blurb = Hitam kehijauan hutan
theme-dark-aubergine-blurb = Hitam keunguan, lila
theme-dark-oxblood-blurb = Hitam anggur, koral

## texnames.ts

texnames-needs-map = Pola perlu { "{" }map{ "}" }, atau semua map dari satu material akan mendapat nama yang sama
texnames-placeholders = Pola dapat memakai { "{" }name{ "}" }, { "{" }map{ "}" } dan { "{" }size{ "}" }
texnames-bad-characters = Nama tidak boleh berisi < > : " / \ | ? atau *
texnames-no-suffix = Beri akhiran untuk map { $role }
texnames-bad-suffix = Akhiran untuk map { $role } tidak boleh berisi < > : " / \ | ? atau *
texnames-slot-no-suffix = Beri akhiran untuk “{ $slot }”
texnames-slot-bad-suffix = Akhiran untuk “{ $slot }” tidak boleh berisi < > : " / \ | ? atau *
texnames-ending-missing = Setiap akhir nama buatan sendiri perlu akhir nama yang dicari
texnames-ending-bad = Akhir nama buatan sendiri tidak boleh berisi < > : " / \ | ? atau *
texnames-ending-twice = { $ending } ada dua kali dalam akhir nama Anda
texnames-slot-metallic-roughness = Metal dan roughness
texnames-slot-specular-color = Warna spekular
texnames-slot-specular-glossiness = Spekular dan glossiness
texnames-slot-clearcoat = Clearcoat
texnames-slot-clearcoat-roughness = Roughness clearcoat
texnames-slot-clearcoat-normal = Normal clearcoat
texnames-slot-sheen-color = Warna sheen
texnames-slot-sheen-roughness = Roughness sheen
texnames-slot-transmission = Transmisi
texnames-slot-thickness = Ketebalan
texnames-slot-diffuse-transmission = Transmisi difus
texnames-slot-diffuse-transmission-color = Warna transmisi difus
texnames-slot-iridescence = Iridesensi
texnames-slot-iridescence-thickness = Ketebalan iridesensi
texnames-slot-anisotropy = Anisotropi

## storeFormats.ts

storefiles-field-title = Judul
storefiles-field-title-hint = Judul persis dari listing. Judul ini menamai paket, dan tanpanya berkas dilewati.
storefiles-field-seller = Penjual
storefiles-field-seller-hint = Siapa pembuat atau penjualnya.
storefiles-field-license = Lisensi
storefiles-field-license-hint = Seperti yang tertulis di berkas, misalnya CC-BY-4.0.
storefiles-field-url = Halaman toko
storefiles-field-url-hint = Alamat listing, diawali http:// atau https://.
storefiles-field-words = Tag
storefiles-field-words-hint = Kata-kata untuk tag. Nilai yang dipisahkan koma akan dipecah.
storefiles-field-type = Jenis
storefiles-field-type-hint = Apa itu, misalnya “3D model” atau “surface”. Juga menjadi tag.
storefiles-field-colors = Warna
storefiles-field-colors-hint = Warnanya, sebagai tag color:.
storefiles-field-id = ID toko
storefiles-field-id-hint = ID dari toko untuk item ini: huruf, angka, _ atau -.
storefiles-field-origin = Dibuat di
storefiles-field-origin-hint = Tempat dibuat atau dipindai, untuk deskripsi.
storefiles-field-size = Ukuran
storefiles-field-size-hint = Seberapa besar, dalam kata-kata, untuk deskripsi.
storefiles-field-title-name = judul
storefiles-field-seller-name = penjual
storefiles-field-license-name = lisensi
storefiles-field-url-name = halaman toko
storefiles-field-words-name = tag
storefiles-field-type-name = jenis
storefiles-field-colors-name = warna
storefiles-field-id-name = ID toko
storefiles-field-origin-name = tempat pembuatan
storefiles-field-size-name = ukuran

## StoreFormatsPanel.vue

storefiles-load-failed = Tidak dapat memuat format berkas toko: { $error }
storefiles-remove-title = Hapus { $name }?
storefiles-remove-body = { $file } dihapus dari folder data. Paket yang sudah dibaca dengan format ini tetap menyimpan hasil bacaannya.
storefiles-remove = Hapus
storefiles-removed = { $name } dihapus.
storefiles-remove-failed = Tidak dapat menghapus { $name }: { $error }
storefiles-unread = Berkas yang tidak dibaca Hoard
storefiles-loading = Memuat…
storefiles-unread-none = Tidak ada: tidak ada berkas di pustaka yang tampak seperti berkas toko yang belum dibaca.
storefiles-unread-what = Berkas yang ada di lebih dari satu paket, atau bernama seperti berkas toko, yang tidak dibaca format mana pun.
storefiles-unread-packs = { $count } paket
storefiles-unread-example = mis. { $pack }
storefiles-teach = Ajari Hoard berkas ini
storefiles-formats = Format
storefiles-formats-what = Format milik Anda dicoba lebih dulu dan menggantikan format bawaan dengan nama yang sama.
storefiles-dir-before = Format disimpan di
storefiles-dir-after = .
storefiles-yours = Milik Anda
storefiles-built-in = Bawaan
storefiles-edit = Edit
storefiles-start-from = Mulai dari ini

## TagsPanel.vue

tags-load-failed = Tidak dapat memuat tag: { $error }
tags-rename-title = Ganti nama { $tag }?
tags-merge-title = Gabungkan { $count } tag?
tags-rename-body = { $count } asetnya mendapat { $into } sebagai gantinya, dan pemberi tag memakai nama baru mulai sekarang.
tags-merge-body = { $tags } menjadi { $into } pada hingga { $count } aset. Pemberi tag memakai { $into } mulai sekarang.
tags-rename = Ganti nama
tags-merge = Gabungkan
tags-merged = { $count } aset kini bertag { $into }
tags-merge-failed = Tidak dapat menggabungkan: { $error }
tags-forget-failed = Tidak dapat mengubahnya: { $error }
tags-find = Cari tag
tags-find-placeholder = Cari tag, mis. rock atau set:
tags-count = { $count } tag
tags-count-more = { $count }+ tag
tags-what = Pilih tag yang artinya sama lalu gabungkan, atau pilih satu untuk mengganti namanya.
tags-new-name = Nama baru
tags-merge-into = Gabungkan ke
tags-saved-as-before = Disimpan sebagai
tags-saved-as-after = { "" }
tags-cancel = Batal
tags-loading = Memuat…
tags-none = Tidak ada tag.
tags-none-containing = Tidak ada tag yang mengandung “{ $text }”.
tags-browse-title = Telusuri aset bertag { $tag }
tags-browse = Telusuri
tags-merged-heading = Tag yang digabungkan
tags-merged-what = Saat pemberi tag menghasilkan salah satu nama lama ini, aset mendapat tag tujuan penggabungannya.
tags-allow-old = Izinkan nama lama lagi

## LibraryStats.vue

stats-source-human = Ditambahkan oleh orang
stats-source-ai = Disarankan oleh AI
stats-source-rule = Aturan bawaan
stats-source-meta = Dari berkas
stats-assets = Aset
stats-packs = Paket
stats-on-disk = Di disk
stats-duplicates-title = { $count } berkas adalah salinan identik byte demi byte dari berkas lain dan disimpan sekali
stats-saved-dedup = Dihemat oleh deduplikasi
stats-distinct-tags = Tag berbeda
stats-archive = Di arsip
stats-load-failed = Tidak dapat memuat detail: { $error }
stats-counting = Menghitung…
stats-by-kind = Ruang per jenis
stats-files-size = { $name }: { $count } berkas, { $size }
stats-months = Ditambahkan dalam setahun terakhir
stats-month = { $month }: { $count } aset, { $size }
stats-tagging = Pemberian tag
stats-tagged = Bertag
stats-untagged = { $count } tanpa tag
stats-described = Dideskripsikan
stats-described-assets = { $count } aset
stats-human-edited = Diedit manual
stats-human-edited-unit =
    { $count ->
       *[other] aset
    }
stats-source-different = { $count } berbeda
stats-most-used = Paling sering dipakai
stats-show-tag = Tampilkan aset { $tag }
stats-file-types = Jenis berkas
stats-biggest-packs = Paket terbesar
stats-browse-pack = Telusuri { $name }
stats-largest-files = Berkas terbesar
stats-open-file = Buka { $name }
stats-empty = Pustaka masih kosong. Tambahkan paket dan angkanya muncul di sini.

## UpdatesPanel.vue

updates-saved = Hoard akan memanggilnya saat pembaruan siap
updates-saved-none = Tidak ada alamat yang dipanggil
updates-save-failed = Tidak dapat menyimpan
updates-call-answered = Alamat itu menjawab { $result }
updates-call-refused = Alamat itu tidak menerima permintaan: { $result }
updates-call-failed = Tidak dapat memanggilnya
updates-called = Dipanggil { $when } untuk { $version }: { $result }
updates-called-failed = Dipanggil { $when } untuk { $version }: { $result }. Hoard mencoba lagi dalam satu jam.
updates-latest = Hoard { $version } adalah rilis terbaru.
updates-out = Hoard { $version } sudah rilis.
updates-out-idle = Hoard { $version } sudah rilis; akan memanggil dalam beberapa menit, selagi tidak ada yang berjalan.
updates-out-busy = Hoard { $version } sudah rilis; menunggu sampai tidak ada yang berjalan ({ $work }).
updates-busy-more = { $count } lainnya
updates-busy-jobs = { $count } pekerjaan menunggu
updates-busy-uploads = { $count } unggahan
updates-list-separator = ,{ " " }
updates-what = Saat rilis baru keluar, Hoard dapat memanggil sebuah alamat begitu tidak ada yang berjalan (tidak ada unggahan, pekerjaan, konversi, atau pemeliharaan), sehingga apa pun yang memperbarui container-nya tidak memulai ulang Hoard di tengah unggahan. Hoard mengirim POST dengan rilis dalam body JSON. Ini memerlukan pemeriksaan rilis di atas.
updates-url = Alamat yang dipanggil
updates-url-placeholder = Tidak ada (mis. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Tersimpan (ketik untuk mengganti)
updates-auth-placeholder = Tidak ada (mis. Bearer my-token)
updates-remove = Hapus
updates-save = Simpan
updates-call-now-title = Apa pun yang terhubung dengannya akan berjalan
updates-call-now = Panggil sekarang
updates-what-calls = Apa yang dapat dipanggilnya?
updates-watchtower = HTTP API Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , dengan{ " " }
updates-watchtower-token = { " " }dan token dari{ " " }
updates-watchtower-its = { " " }(dengan endpoint{ " " }
updates-watchtower-endpoint = { " " }diaktifkan).
updates-portainer = Webhook Portainer
updates-portainer-rest = { " " }di stack atau container Hoard, dengan re-pull image aktif. Tokennya ada di URL.
updates-script = Skrip di host
updates-script-listener = { " " }di balik pendengar webhook (seperti{ " " }
updates-script-runs = ) yang menjalankan{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }atau apa pun yang menerima webhook.
updates-poll-before = Atau lewati alamat: skrip dengan token akses dapat memeriksa{ " " }
updates-poll-until = { " " }secara berkala sampai{ " " }
updates-poll-and = { " " }terisi dan{ " " }
updates-poll-after = { " " }bernilai true, lalu tarik dan mulai ulang Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Tidak dapat memuat Pemeliharaan
maintenance-reanalyze-title = Analisis ulang semua paket?
maintenance-reanalyze-body = Setiap berkas dibaca lagi, yang bisa lama sekali untuk pustaka yang besar; paket tampil sedang diproses sampai gilirannya tiba. Nama, tag, dan catatan Anda sendiri tetap ada.
maintenance-reanalyze-confirm = Analisis ulang
maintenance-gc-title = Hapus berkas yang tidak dipakai?
maintenance-gc-body = Berkas tersimpan yang tidak dirujuk apa pun di pustaka dihapus permanen. Paket di arsip tetap menyimpan berkasnya.
maintenance-gc-confirm = Hapus
maintenance-start-failed = Tidak dapat memulai “{ $task }”
maintenance-just-now = baru saja
maintenance-minutes-ago = { $count } mnt yang lalu
maintenance-hours-ago = { $count } j yang lalu
maintenance-took-seconds = { $count } dtk
maintenance-took-minutes = { $count } mnt
maintenance-took-hours = { $hours } j
maintenance-loading = Memuat…
maintenance-running = Berjalan…
maintenance-starting = Memulai…
maintenance-run = Jalankan
maintenance-started = { $detail } · dimulai { $when }
maintenance-starting-detail = Memulai
maintenance-finished = Selesai:{ " " }
maintenance-failed = Gagal:{ " " }
maintenance-never-run = Belum pernah dijalankan
maintenance-history = Riwayat proses
maintenance-col-task = Tugas
maintenance-col-finished = Selesai
maintenance-col-took = Durasi
maintenance-col-result = Hasil
maintenance-script-before = Pencadangan dan pemeriksaan kerusakan juga dapat dijalankan dari skrip:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }dan{ " " }
maintenance-script-after = . Proses tersebut juga muncul di sini.

## SettingsPage.vue

settings-title = Pengaturan
settings-loading = Memuat…
settings-load-failed = Tidak dapat memuat pengaturan: { $error }
settings-page-general = Umum
settings-page-general-blurb = Nama server ini, untuk semua orang yang memakainya.
settings-page-general-blurb-desktop = Nama pustaka ini.
settings-page-appearance = Tampilan
settings-page-appearance-blurb = Tema yang dipakai browser ini saat terang dan saat gelap. Hanya disimpan di browser ini.
settings-page-library = Pustaka
settings-page-library-blurb = Isi Hoard: seberapa besar, terdiri dari apa, bagaimana pertumbuhannya, dan seberapa baik tag-nya. Item yang diarsipkan tidak dihitung.
settings-page-tags = Tag
settings-page-tags-blurb = Semua tag di pustaka. Gabungkan tag yang artinya sama, atau ganti nama satu tag; pemberi tag memakai nama baru sejak saat itu.
settings-page-store-files = Berkas toko
settings-page-store-files-blurb = Berkas yang disertakan toko di samping karyanya, yang dibaca Hoard untuk menamai paket dan memberi kredit kepada pembuatnya. Ajari Hoard berkas yang belum dikenalnya, dan bagikan apa yang Anda ajarkan.
settings-page-locations = Lokasi pustaka
settings-page-locations-blurb = Folder di server yang menampung pustaka: folder tempat Hoard menyimpan berkas, dan folder berisi paket yang diindeksnya di tempat.
settings-page-locations-blurb-desktop = Folder yang menampung pustaka: folder tempat Hoard menyimpan berkas, dan folder berisi paket yang diindeksnya di tempat.
settings-page-tools = Alat
settings-page-tools-blurb = Program luar yang dipakai Hoard untuk mengonversi dan membaca berkas.
settings-page-ai = Penandaan AI
settings-page-ai-blurb = Model AI menamai, mendeskripsikan, dan memberi tag paket baru, di samping aturan bawaan. Tag-nya ditandai sebagai saran.
settings-page-search = Pencarian gambar dan suara
settings-page-search-blurb = Cari gambar berdasarkan isinya dan suara berdasarkan bunyinya, bukan hanya namanya, dan temukan hal yang sejenis, bukan hanya yang sewarna.
settings-page-adding = Menambahkan paket
settings-page-adding-blurb = Apa yang dilakukan Hoard dengan paket saat ditambahkan, selain membaca dan memberinya tag.
settings-page-downloads = Unduhan
settings-page-downloads-blurb = Cara berkas dinamai dalam kiriman Hoard: unduhan, zip, alat hoard, dan asisten AI. Berkas di pustaka tetap apa adanya.
settings-page-account = Akun
settings-page-account-blurb = Kata sandi Anda, orang lain yang dapat masuk, dan token akses untuk skrip dan program lain.
settings-page-auth = Autentikasi
settings-page-auth-blurb = Cara orang masuk: akun Hoard sendiri, direktori LDAP, Microsoft Active Directory, atau penyedia OAuth, berapa lama sesi berlangsung, dan grup mana mendapat peran apa.
settings-page-connection = Koneksi
settings-page-connection-blurb = Cara browser dan aplikasi menjangkau Hoard: alamat yang dikenalnya, dan HTTPS, yang diperlukan sebagian fitur browser.
settings-page-performance = Performa
settings-page-performance-blurb = Seberapa keras server bekerja, cara gambar mini dibuat dan disimpan, dan batas maksimum yang diterimanya.
settings-page-performance-blurb-desktop = Seberapa keras Hoard bekerja, cara gambar mini dibuat dan disimpan, dan batas maksimum yang diterimanya.
settings-page-issues = Masalah
settings-page-issues-blurb = Apa yang gagal saat membuka atau memproses berkas: arsip yang tidak bisa dibongkar, berkas yang tidak dapat dibaca, pekerjaan yang gagal. Coba lagi setelah penyebabnya diperbaiki, atau abaikan.
settings-page-maintenance = Pemeliharaan
settings-page-maintenance-blurb = Cadangkan, periksa kerusakan, dan rapikan. Masing-masing berjalan satu per satu, dan Anda dapat meninggalkan halaman ini selagi berjalan.
settings-the-default = bawaan
settings-default-on = aktif
settings-default-off = mati
settings-default-auto = otomatis
settings-option-saved = { $name }: { $value }
settings-back-to-default = Kembali ke bawaan
settings-save = Simpan
settings-use = Pakai
settings-status-off = Mati

## SettingsPage.vue: Tampilan

settings-themes = Tema
settings-look-light = Terang
settings-look-dark = Gelap
settings-look-group =
    { $scheme ->
        [light] Tema terang
       *[dark] Tema gelap
    }
settings-preview =
    Menampilkan tema { $preview ->
        [light] terang
       *[dark] gelap
    } agar Anda dapat melihatnya. Anda memakai tema { $used ->
        [light] terang
       *[dark] gelap
    }{ $system ->
        [yes] { " " }(mengikuti sistem)
       *[other] { "" }
    }, dan tema itu kembali saat Anda meninggalkan halaman ini.
settings-preview-back =
    Kembali ke tema { $used ->
        [light] terang
       *[dark] gelap
    }
settings-look-hint = Beralih antara terang dan gelap (atau ikuti sistem) dari bilah atas.

## SettingsPage.vue: Alat

settings-blender-what = Mengonversi model .blend, .dae, .abc, dan .x3d ke glTF di latar belakang, agar mendapat gambar mini dan pratinjau 3D. Paket yang ditambahkan selagi fitur ini mati dikonversi setelah siap.
settings-blender-download = Memakai Blender yang terpasang, atau mengunduh Blender 4.5 LTS (sekitar 400 MB) dari mirror blender.org.
settings-ffmpeg-what = Meng-encode unduhan MP3, OGG, dan Opus, serta membaca audio yang tidak dapat didekode Hoard sendiri (AAC, M4A, WMA, …).
settings-ffmpeg-download = Memakai ffmpeg sistem dengan encoder yang sesuai, atau mengunduh build statis (sekitar 80 MB).
settings-tool-reset = { $tool }: kembali ke bawaan
settings-tool-no-download = Memakai { $tool } yang terpasang. Tidak ada build yang dapat diunduh untuk mesin ini, jadi pasang atau berikan path-nya.
settings-tool-path = Path program { $tool }
settings-mode-off = Mati
settings-mode-auto = Otomatis
settings-mode-path = Program ini
settings-tool-looking = Mencari…
settings-tool-downloading = Mengunduh… kemajuannya ada di indikator aktivitas
settings-tool-ready = Siap: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Instrumen yang dipakai untuk memutar dan mengonversi berkas MIDI.
settings-soundfont-file = Berkas SoundFont
settings-soundfont-placeholder = Ditemukan di server, atau bawaan
settings-soundfont-placeholder-desktop = Ditemukan di komputer ini, atau bawaan
settings-soundfont-default = ditemukan di server, atau bawaan
settings-soundfont-default-desktop = ditemukan di komputer ini, atau bawaan

## SettingsPage.vue: Pencarian gambar

settings-clip-heading = Cari berdasarkan isi gambar
settings-clip-reset = Pencarian gambar: kembali ke bawaan
settings-clip-saved-on = Pencarian gambar: aktif
settings-clip-saved-off = Pencarian gambar: mati
settings-clip-what = Ketik apa yang Anda cari (“mossy rock”, “sci-fi door”, “wooden barrel”) dan gambar yang menampilkannya muncul di samping nama yang mengandung kata-kata itu. “Lainnya yang mirip ini” menemukan hal yang sejenis, bukan hanya yang warnanya sama.
settings-clip-toggle = Aktifkan pencarian gambar
settings-clip-hint = Memakai model CLIP dari OpenAI, dijalankan di prosesor server ini: tidak ada yang keluar dari server. Mengaktifkannya akan mengunduh model ({ $size } MB) dari Hugging Face, lalu mempelajari setiap gambar di latar belakang dengan prioritas rendah. Butuh memori 300 sampai 600 MB selagi bekerja atau saat seseorang mencari. Mematikannya akan menghapus model dan menyimpan apa yang sudah dipelajari, untuk lain kali.
settings-clip-hint-desktop = Memakai model CLIP dari OpenAI, dijalankan di prosesor komputer ini: tidak ada yang keluar dari komputer. Mengaktifkannya akan mengunduh model ({ $size } MB) dari Hugging Face, lalu mempelajari setiap gambar di latar belakang dengan prioritas rendah. Butuh memori 300 sampai 600 MB selagi bekerja atau saat Anda mencari. Mematikannya akan menghapus model dan menyimpan apa yang sudah dipelajari, untuk lain kali.
settings-clip-looking = Mencari model…
settings-clip-downloading = Mengunduh model… kemajuannya ada di indikator aktivitas
settings-clip-ready-empty = Siap. Gambar dipelajari saat paket ditambahkan.
settings-clip-ready-all =
    { $count ->
       *[other] Siap: mengenal semua { $count } gambar
    }
settings-clip-learning = Siap: mempelajari gambar, { $done } dari { $total } sejauh ini
settings-clip-unavailable = Tidak dapat mengambil model: { $detail }. Mencoba lagi setiap jam.
settings-clap-heading = Cari berdasarkan bunyi suara
settings-clap-reset = Pencarian suara: kembali ke bawaan
settings-clap-saved-on = Pencarian suara: aktif
settings-clap-saved-off = Pencarian suara: mati
settings-clap-what = Ketik apa yang Anda cari (“distant thunder”, “glassy UI blip”, “footsteps on gravel”) dan suara yang terdengar seperti itu muncul di samping nama yang mengandung kata-kata itu.
settings-clap-toggle = Aktifkan pencarian suara
settings-clap-hint = Memakai model CLAP dari LAION, dijalankan di prosesor server ini: tidak ada yang keluar dari server. Mengaktifkannya akan mengunduh model ({ $size } MB) dari Hugging Face, lalu mendengarkan bagian awal setiap suara di latar belakang dengan prioritas rendah. Butuh memori hingga sekitar 1 GB selagi bekerja atau saat seseorang mencari. Mematikannya akan menghapus model dan menyimpan apa yang sudah dipelajari, untuk lain kali.
settings-clap-hint-desktop = Memakai model CLAP dari LAION, dijalankan di prosesor komputer ini: tidak ada yang keluar dari komputer. Mengaktifkannya akan mengunduh model ({ $size } MB) dari Hugging Face, lalu mendengarkan bagian awal setiap suara di latar belakang dengan prioritas rendah. Butuh memori hingga sekitar 1 GB selagi bekerja atau saat Anda mencari. Mematikannya akan menghapus model dan menyimpan apa yang sudah dipelajari, untuk lain kali.
settings-clap-ready-empty = Siap. Suara dipelajari saat paket ditambahkan.
settings-clap-ready-all =
    { $count ->
       *[other] Siap: mengenal semua { $count } suara
    }
settings-clap-learning = Siap: mempelajari suara, { $done } dari { $total } sejauh ini

## SettingsPage.vue: Penandaan AI

settings-ai-service = Layanan
settings-ai-off = Mati
settings-ai-off-hint = Paket diberi tag hanya dengan aturan bawaan.
settings-ai-local = Model lokal
settings-ai-local-hint = Layanan llm di berkas compose Hoard (atau cargo xtask llm). Unduh model di bawah; model berjalan di mesin ini dan tidak ada yang keluar darinya.
settings-ai-chatgpt = Langganan ChatGPT
settings-ai-chatgpt-hint = Memakai langganan ChatGPT Go, Plus, Pro, atau Business Anda, bukan kunci API.
settings-ai-openrouter-hint = Satu kunci untuk ratusan model yang di-hosting, sebagian gratis. Model yang menerima gambar ditandai.
settings-ai-openai-hint = Di-hosting oleh OpenAI.
settings-ai-anthropic-hint = Di-hosting oleh Anthropic, melalui endpoint yang kompatibel dengan OpenAI.
settings-ai-gemini-hint = Di-hosting oleh Google, melalui endpoint yang kompatibel dengan OpenAI.
settings-ai-ollama-hint = Server Ollama yang Anda jalankan. Ubah alamatnya jika bukan container bernama ollama.
settings-ai-custom = Lainnya (kompatibel dengan OpenAI)
settings-ai-custom-hint = Apa pun yang memakai API chat OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Menghubungkan…
settings-ai-connected = Terhubung · { $count } model
settings-ai-connected-none = Terhubung, tetapi tidak ada model yang tercantum; ketik nama modelnya
settings-chatgpt-connected-none = Terhubung; ketik nama modelnya
settings-ai-takes-images = menerima gambar
settings-ai-saved = Penandaan AI: { $service }
settings-ai-saved-off = Penandaan AI mati
settings-ai-reset = Penandaan AI: kembali ke bawaan
settings-ai-on = Aktif: { $service } · { $model }
settings-ai-url = Alamat
settings-ai-key = Kunci API
settings-ai-key-saved = Tersimpan (ketik untuk mengganti)
settings-ai-key-paste = Tempel kunci Anda
settings-ai-key-optional = Tidak ada (opsional)
settings-ai-key-remove = Hapus
settings-ai-get-key = Dapatkan kunci
settings-ai-model = Model
settings-ai-model-pick = Pilih atau ketik model
settings-ai-model-name = Nama model
settings-ai-refresh = Muat ulang
settings-ai-connect = Hubungkan
settings-ai-need-key = Tambahkan kunci API untuk melihat modelnya
settings-ai-vision = Kirim juga gambar mini (memerlukan model yang menerima gambar)
settings-ai-batch = Per permintaan
settings-ai-batch-unit = aset (lebih sedikit untuk model lokal kecil, lebih banyak untuk model besar yang di-hosting)

## SettingsPage.vue: Masuk ChatGPT

settings-chatgpt-account = Akun
settings-chatgpt-checking = Memeriksa…
settings-chatgpt-status-as = Masuk sebagai { $email }
settings-chatgpt-status = Sudah masuk
settings-chatgpt-signed-in-as = Masuk ke ChatGPT sebagai { $email }
settings-chatgpt-signed-in = Sudah masuk ke ChatGPT
settings-chatgpt-signed-out = Sudah keluar dari ChatGPT
settings-chatgpt-continue = Lanjutkan dengan ChatGPT
settings-chatgpt-sign-out = Keluar
settings-chatgpt-sign-out-title = Keluar dari ChatGPT?
settings-chatgpt-sign-out-in-use = Penandaan AI memakai langganan ChatGPT Anda, jadi akan berhenti sampai Anda masuk lagi atau memilih layanan lain.
settings-chatgpt-sign-out-body = Hoard melupakan proses masuk dan meminta OpenAI mengakhirinya.
settings-chatgpt-manage = Kelola pemakaian
settings-chatgpt-finish-before = Selesaikan proses masuk di jendela yang terbuka (
settings-chatgpt-finish-link = buka lagi
settings-chatgpt-finish-middle = ). Jika berakhir di halaman yang tidak termuat, di alamat yang diawali{ " " }
settings-chatgpt-finish-after = , salin alamat itu dan tempel di sini:
settings-chatgpt-paste = Alamat tempat proses masuk berakhir
settings-chatgpt-finish = Selesai
settings-chatgpt-usage = Permintaan pemberian tag dihitung dalam pemakaian langganan ChatGPT Anda. Kelola pemakaian menampilkan apa yang sudah dipakai Hoard dan memungkinkan Anda membatasinya. Untuk memutuskan Hoard dari ChatGPT itu sendiri, buka Settings → Security and login → Sign in with ChatGPT di chatgpt.com.

## SettingsPage.vue: Model lokal

settings-local-downloading-progress = Mengunduh { $percent }% · { $done } dari { $total }
settings-local-downloading = Mengunduh…
settings-local-incomplete = Unduhan berhenti
settings-local-failed = Gagal dimulai terakhir kali
settings-local-in-use-running = Dipakai · berjalan
settings-local-in-use = Dipakai
settings-local-starting = Memulai…
settings-local-running = Berjalan
settings-local-ready-images = Siap · menerima gambar
settings-local-ready-text = Siap · hanya teks
settings-local-delete-title = Hapus { $name }?
settings-local-delete-in-use = Penandaan AI memakai model ini, jadi akan berhenti sampai Anda memilih model lain. Berkasnya dihapus dari server model.
settings-local-delete-downloading = Unduhan berhenti dan bagian yang sudah terunduh dihapus.
settings-local-delete-body = Berkasnya dihapus dari server model. Anda dapat mengunduhnya lagi nanti.
settings-local-delete = Hapus
settings-local-deleting = Menghapus…
settings-local-cancel = Batal
settings-local-resume = Lanjutkan
settings-local-download = Unduh
settings-local-start-before = Jalankan dengan{ " " }
settings-local-start-or = { " " }(atau{ " " }
settings-local-start-developing = { " " }saat pengembangan), lalu{ " " }
settings-local-start-retry = coba lagi
settings-local-start-after = . Server yang hanya menjalankan satu model juga bisa: berikan nama modelnya di bawah.
settings-local-list = Di server model
settings-local-none = Belum ada yang diunduh. Pilih salah satu di bawah; model disimpan di server, dan Anda dapat beralih kapan saja.
settings-local-catalog = Model untuk diunduh
settings-local-size = { $size } GB · { $license }
settings-local-other = Model lain dari Hugging Face
settings-local-other-placeholder = Lainnya dari Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Menambahkan paket

settings-store-pages = Halaman toko
settings-store-pages-what = Paket baru dicari di toko di latar belakang, agar mendapat halaman toko, penjual, dan lisensinya.
settings-store-lookup = Cari halaman toko secara otomatis
settings-unreal = Paket Unreal
settings-unreal-reset = Paket Unreal: kembali ke bawaan
settings-unreal-what = Apa yang dilakukan dengan berkas .uasset.
settings-conv = Aturan konversi
settings-conv-what = Konversi berkas yang disimpan Hoard ke satu format, misalnya semua OBJ dan FBX ke GLB, agar seluruh pustaka seragam. Berkas hasil konversi menggantikan berkas asli, dengan nama, tag, koleksi, dan tautannya. Berkas di folder pantauan tidak pernah diubah.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Aktif
settings-conv-to = Konversi ke
# Heading over the file formats a rule converts.
settings-conv-from = Dari
settings-conv-remove = Hapus
settings-conv-remove-label = Hapus aturan { $n }
settings-conv-taken = Aturan lain sudah mengonversi format ini
settings-conv-upright = Tegakkan model saat dikonversi (Y ke atas, menghadap +Z), seperti cara model paket ditampilkan
settings-conv-add = Tambah aturan
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Simpan berkas asli selama
settings-conv-keep-days =
    { $count ->
       *[other] hari
    }
settings-conv-keep-what = Sampai saat itu, halaman berkas hasil konversi dapat mengembalikan berkas asli. Tekstur tidak disalin ke dalam model hasil konversi: model merujuk ke berkas gambar di sebelahnya. Berkas .mtl atau .bin milik model tetap ada.
settings-conv-save = Simpan aturan
settings-conv-revert = Urungkan perubahan
settings-conv-existing = Konversi berkas yang sudah ada di pustaka
settings-conv-saved = Aturan konversi disimpan. Aturan ini berlaku untuk paket yang ditambahkan mulai sekarang.
settings-conv-converting =
    { $count ->
       *[other] Mengonversi { $count } berkas di latar belakang
    }
settings-conv-nothing = Tidak ada yang perlu dikonversi di pustaka.
settings-conv-ask-title =
    { $count ->
       *[other] Konversi { $count } berkas yang sudah ada di pustaka?
    }
settings-conv-ask-body =
    { $packs ->
       *[other] Berkas tersebut ada di { $packs } paket.
    } Setiap berkas asli disimpan selama { $days } hari, dan sampai saat itu dapat dikembalikan dari halamannya.
settings-conv-ask-busy =
    { $count ->
       *[other] { $count } berkas ada di paket yang masih ditambahkan; berkas tersebut dikonversi setelah penambahannya selesai.
    }
settings-conv-ask-list = Berkas yang akan dikonversi
settings-conv-ask-more = …dan { $count } lagi
settings-conv-ask-skipped = Dibiarkan seperti semula:
settings-conv-skip-watched =
    { $count ->
       *[other] { $count } berkas di folder pantauan
    }
settings-conv-skip-unity =
    { $count ->
       *[other] { $count } berkas yang dikenal proyek Unity dari namanya
    }
settings-conv-skip-used =
    { $count ->
       *[other] { $count } berkas yang disebut namanya oleh adegan atau berkas lain
    }
settings-conv-skip-beside =
    { $count ->
       *[other] { $count } berkas yang salinan hasil konversinya sudah ada di paketnya
    }
settings-conv-skip-blender =
    { $count ->
       *[other] { $count } berkas yang belum dibaca Blender (Pengaturan → Alat)
    }
settings-conv-ask-new = Simpan, hanya paket baru
settings-conv-ask-save-convert =
    { $count ->
       *[other] Simpan dan konversi { $count } berkas
    }
settings-conv-ask-convert =
    { $count ->
       *[other] Konversi { $count } berkas
    }
settings-unreal-off = Biarkan apa adanya
settings-unreal-off-hint = Konversi dari halaman paket kapan pun Anda mau.
settings-unreal-keep = Konversi, simpan aslinya
settings-unreal-keep-hint = Tekstur, suara, dan material dikeluarkan dalam format umum; .uasset juga tetap ada.
settings-unreal-replace = Konversi, buang aslinya
settings-unreal-replace-hint = Menghemat ruang setelah isinya dikeluarkan. Mesh tetap menunggu pekerja Unreal.

## SettingsPage.vue: Unduhan

settings-tn = Nama tekstur
settings-tn-saved = Nama tekstur disimpan
settings-tn-reset = Nama tekstur: kembali ke bawaan
settings-tn-what = Map dari sebuah material dikirim dengan nama material dan kegunaan tiap map, apa pun nama berkasnya: Rock_Normal.png, bukan rock_moss_xeokah0_raw_8K_Normal.png. Berkas di pustaka tidak disentuh, dan map yang disebutkan namanya oleh model atau MTL yang dikirim bersamanya tetap memakai nama itu, agar model tetap menemukannya.
settings-tn-toggle = Namai map tekstur sesuai materialnya
settings-tn-style = Gaya
settings-tn-style-group = Gaya penamaan
settings-tn-descriptive = Deskriptif
settings-tn-short = Pendek
settings-tn-custom = Buatan sendiri
settings-tn-pattern = Pola
settings-tn-hint-name = { " " }nama material,{ " " }
settings-tn-hint-map = { " " }kegunaan map,{ " " }
settings-tn-hint-size = { " " }ukurannya jika set tersedia dalam lebih dari satu ukuran (ditambahkan di akhir jika pola tidak menyertakannya).
settings-tn-example = Contoh
settings-tn-suffixes = Akhiran untuk tiap jenis map
settings-tn-slots = Dalam unduhan GLB, tekstur yang dikemas di dalam model tanpa berkas sendiri dinamai menurut material dan slot yang diisinya, dan slot ini juga punya akhiran:
settings-tn-endings = Akhir nama buatan sendiri
settings-tn-endings-what = Tekstur yang bukan salah satu map dari sebuah material tetap memakai namanya sendiri, kecuali namanya berakhir dengan salah satu dari ini: _diff → _BC mengirim rock_diff.png sebagai rock_BC.png. Kosongkan yang kedua untuk membuang akhir nama itu.
settings-tn-ending-from = Akhir nama
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Ganti dengan
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Tambahkan akhir nama
settings-tn-ending-remove = Hapus
settings-tn-ending-remove-label = Hapus { $ending }

## SettingsPage.vue: Umum dan Koneksi

settings-name = Nama
settings-name-what = Nama server ini di bilah atas, tab browser, dan halaman masuk, agar jika ada lebih dari satu Hoard Anda dapat membedakannya.
settings-name-what-desktop = Nama pustaka ini di bilah atas.
settings-address = Alamat
settings-address-what = Alamat yang dipakai orang dan aplikasi untuk menjangkau Hoard, jika bukan alamat yang sedang dibuka browser ini: alamat reverse proxy, tunnel, atau nama tailnet, seperti https://hoard.example.com. Tautan yang dibagikan Hoard (ke Claude dan aplikasi lain, saat masuk, di sertifikatnya) memakai alamat ini.
settings-public-url = Alamat publik
settings-public-url-placeholder = Dari alamat browser (mis. https://hoard.example.com)
settings-public-url-default = dari alamat browser

## ProfilerPanel.vue: profiler di bagian atas Pengaturan → Performa

profiler = Profiler
profiler-what = Apa yang dikerjakan Hoard selama sepuluh menit terakhir, diperbarui setiap dua detik selagi Anda melihat. Hanya disimpan di memori dan dimulai lagi dari awal saat Hoard dimulai ulang.
profiler-starting = Pembacaan pertama tiba sebentar lagi.
profiler-failed = Tidak dapat membaca profiler: { $error }
profiler-cpu = Penggunaan prosesor
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Seluruh mesin
profiler-cpu-summary = Hoard memakai { $hoard } dari prosesor, dan seluruh mesin { $machine }.
profiler-memory = Memori
profiler-serving = Permintaan yang sedang berjalan
profiler-working = Sedang mengerjakan
profiler-working-none = Tidak ada yang berjalan di latar belakang.
profiler-working-more =
    { $count ->
       *[other] Dan { $count } lagi.
    }
profiler-requests = Waktu melayani permintaan
profiler-requests-what = Setiap batang adalah sepuluh detik permintaan, dengan waktunya dibagi menurut ke mana waktu itu terpakai.
profiler-part-db = Basis data
profiler-part-db-title = Menunggu basis data dan menjalankan kuerinya
profiler-part-disk = Disk
profiler-part-disk-title = Membaca berkas tersimpan, dan memampatkan apa yang dikirim
profiler-part-send = Mengirim
profiler-part-send-title = Menunggu selagi jaringan menerima apa yang dikirim
profiler-part-other = Lainnya
profiler-part-other-title = Menyusun jawaban
profiler-request-count =
    { $count ->
       *[other] { $count } permintaan
    }
profiler-requests-none = Tidak ada permintaan dalam sepuluh menit terakhir.
profiler-slowest = Paling lambat: { $route }, { $time }
profiler-routes = Per rute
profiler-route = Rute
profiler-count = Permintaan
profiler-average = Rata-rata
profiler-p95 = 5% paling lambat
profiler-split = Ke mana waktunya terpakai
profiler-sent = Terkirim
profiler-portal = berkas antarmuka itu sendiri

## SettingsPage.vue: Performa

settings-render = Gambar mini 3D
settings-render-toggle = Render gambar mini untuk model 3D
settings-gpu-on = Di GPU: { $renderer }
settings-gpu-software = GPU tidak ditemukan, jadi memakai software ({ $renderer })
settings-gpu-none = GPU atau perender software tidak ditemukan, jadi model tidak mendapat gambar mini
settings-thumbs = Gambar mini
settings-thumbs-what = Cara gambar mini dan tampilan model baru disimpan. Gambar dengan transparansi, dan pixel art, selalu PNG. Untuk memperkecil yang sudah ada, jalankan Perkecil gambar mini di Pemeliharaan.
settings-previews-size = Pratinjau kini memakan { $size }.
settings-thumbs-saved = Gambar mini: { $quality }
settings-thumbs-small = Lebih kecil
settings-thumbs-small-hint = JPEG, kualitas 70. Cukup di kisi; sedikit buram dari dekat.
settings-thumbs-balanced = Seimbang
settings-thumbs-balanced-hint = JPEG, kualitas 82. Sulit dibedakan dari aslinya pada ukuran gambar mini.
settings-thumbs-sharp = Lebih tajam
settings-thumbs-sharp-hint = JPEG, kualitas 92. Sekitar dua kali ukuran Seimbang.
settings-thumbs-lossless = Lossless
settings-thumbs-lossless-hint = PNG, persis seperti dibuat. Beberapa kali lebih besar.
settings-motion-thumbs = Thumbnail bergerak untuk animasi
settings-motion-thumbs-what = Setiap animasi juga mendapat film pendek berulang yang diputar saat Anda menunjuk ubinnya. Masing-masing sekitar 40 KB dan perlu waktu untuk dibuat di pustaka besar. Mematikan ini akan menghapusnya.
settings-limits = Kerja dan batas
settings-limits-what = Masing-masing tersimpan saat Anda meninggalkan kolom atau menekan Enter. Ukuran memakai satuan: 100 GB, 512 MB, 1.5 TB; angka tanpa satuan berarti GB.
settings-workers = Pekerja latar belakang
settings-workers-unit = pekerjaan sekaligus (memasukkan, memberi tag, konversi)
settings-max-upload = Unggahan terbesar
settings-max-upload-unit = per berkas
settings-max-expand = Paket terbesar
settings-max-expand-unit = setelah dibongkar
settings-conversion-cache = Unduhan hasil konversi
settings-conversion-cache-unit-now = disimpan paling banyak, kini { $size }; yang paling lama tidak dipakai dibuang lebih dulu
settings-conversion-cache-unit = disimpan paling banyak; yang paling lama tidak dipakai dibuang lebih dulu

## SettingsPage.vue: Pemeliharaan

settings-updates = Pembaruan
settings-version = Ini Hoard { $version }.
settings-update-check = Periksa GitHub setiap beberapa jam untuk rilis baru
settings-desktop-updates = Hoard mencari rilis baru setiap kali dimulai, mengunduhnya di latar belakang, dan memasangnya saat kamu keluar.

## Jenis aset (api.ts)

api-kind-texture = Tekstur
api-kind-vector = Seni vektor
api-kind-sheet = Lembar sprite
api-kind-audio = Audio
api-kind-model = Model
api-kind-animation = Animasi
api-kind-font = Font
api-kind-shader = Shader
api-kind-material = Material
api-kind-video = Video
api-kind-document = Dokumen
api-kind-data = Data
api-kind-other = Lainnya
api-kind-texture-name = tekstur
api-kind-vector-name = seni vektor
api-kind-sheet-name = lembar sprite
api-kind-audio-name = audio
api-kind-model-name = model
api-kind-animation-name = animasi
api-kind-font-name = font
api-kind-shader-name = shader
api-kind-material-name = material
api-kind-video-name = video
api-kind-document-name = dokumen
api-kind-data-name = data
api-kind-other-name = lainnya

## Notifikasi, dialog, dan pemuatan (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Urungkan
ui-today = hari ini
ui-yesterday = kemarin
ui-days-ago = { $days } hari yang lalu
ui-notifications = Notifikasi
ui-dismiss = Abaikan
ui-cancel = Batal
ui-lazy-loading = Memuat { $what }…
ui-lazy-failed = Tidak dapat memuat { $what }.
ui-lazy-reload = Muat ulang halaman

## ResetButton.vue

reset-title = Kembali ke bawaan: { $value }
reset-label = Kembali ke bawaan, { $value }

## SizeInput.vue

size-invalid = Ukuran seperti 100 GB, 512 MB, atau 1.5 TB
size-hint = Ukuran seperti 100 GB, 512 MB, atau 1.5 TB; angka saja berarti GB

## AuthPanel.vue (Pengaturan → Autentikasi)

auth-load-failed = Tidak dapat memuat pengaturan autentikasi: { $error }
auth-saved = Tersimpan. Berlaku untuk proses masuk berikutnya.
auth-method-internal = Internal
auth-method-internal-hint = Nama dan kata sandi yang disimpan Hoard, ditambahkan di Pengaturan → Akun.
auth-method-ldap-hint = Sebuah direktori memeriksa nama dan kata sandi yang diketik di halaman masuk: OpenLDAP, FreeIPA, LLDAP, atau server LDAP lain.
auth-method-ad-hint = Akun domain Windows masuk dengan nama dan kata sandi biasanya.
auth-method-oauth-hint = Tombol “Masuk dengan…” yang membuka Google, Microsoft, GitHub, Keycloak, Authentik, atau penyedia lain lalu kembali.
auth-session-unused = Setelah 30 hari tidak dipakai
auth-session-1h = 1 jam setelah masuk
auth-session-8h = 8 jam setelah masuk
auth-session-day = Sehari setelah masuk
auth-session-week = Seminggu setelah masuk
auth-session-30d = 30 hari setelah masuk
auth-session-hours = { $hours } jam setelah masuk
auth-role-view = Dapat melihat
auth-role-edit = Dapat mengubah
auth-role-admin = Admin
auth-role-word-view = dapat melihat
auth-role-word-edit = dapat mengubah
auth-role-word-admin = admin
auth-role-word-default = { $role } (peran bawaan)
auth-tls-starttls-hint = LDAP biasa (port 389) yang beralih ke TLS sebelum apa pun dikirim.
auth-tls-ldaps-hint = TLS sejak awal (port 636).
auth-tls-none = Tidak ada
auth-tls-none-hint = Kata sandi melintasi jaringan apa adanya. Hanya untuk jaringan yang Anda percayai.
auth-test-failed = Tidak dapat menguji: { $error }
auth-preset-custom = Lainnya (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel, dan lainnya: berikan issuer-nya dan Temukan akan mengisi alamatnya.
auth-preset-google-note = Buat klien OAuth (Web application) di Google Cloud Console → APIs & Services → Credentials. Google tidak mengirim grup, jadi berikan peran di Pengaturan → Akun, dan matikan pembuatan akun saat pertama masuk kecuali siapa pun yang memiliki akun Google boleh masuk.
auth-preset-microsoft-note = Daftarkan aplikasi di Entra ID → App registrations (redirect Web ke alamat di bawah). Ganti “organizations” dengan ID tenant Anda agar hanya organisasi Anda yang dapat masuk. Untuk peran dari grup, tambahkan klaim groups di Token configuration: klaim itu mencantumkan ID grup.
auth-preset-github-note = Buat aplikasi OAuth di GitHub → Settings → Developer settings. GitHub tidak mengirim grup, jadi berikan peran di Pengaturan → Akun, dan matikan pembuatan akun saat pertama masuk kecuali siapa pun yang memiliki akun GitHub boleh masuk.
auth-preset-gitlab-note = Buat aplikasi di GitLab → Preferences → Applications (atau area admin instans Anda: ganti gitlab.com di alamatnya).
auth-discovered = Alamat penyedia sudah diisi.
auth-copied = Disalin
auth-copy-failed = Tidak dapat menyalin: pilih dan salin sendiri
auth-sign-in-with = Masuk dengan
auth-own-accounts-still-work = Akun Hoard sendiri tetap masuk dengan kata sandinya sendiri, jadi direktori atau penyedia yang mati atau salah diatur tidak dapat mengunci Anda di luar.
auth-sessions = Sesi
auth-sign-out-after = Keluarkan orang
auth-ldap-server = Server LDAP
auth-server-type = Jenis server
auth-server-type-custom = Kustom
auth-servers = Server
auth-server-n = Server { $n }
auth-remove-server-n = Hapus server { $n }
auth-remove = Hapus
auth-add-server = Tambahkan server
auth-servers-hint = Server berikutnya dicoba saat satu server tidak dapat dijangkau.
auth-security = Keamanan
auth-connection-security = Keamanan koneksi
auth-skip-verify = Jangan periksa sertifikat server (hanya untuk mencoba)
auth-ca = Otoritas sertifikat
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (kosong: yang dipercaya sistem)
auth-lookup = Mencari orang
auth-anonymously = Secara anonim
auth-reader-account = Akun pembaca
auth-reader-password = Kata sandinya
auth-saved-type-to-replace = Tersimpan: ketik untuk mengganti
auth-people-under = Orang ada di bawah
auth-name-attribute = Atribut nama
auth-only-matching = Hanya entri yang cocok dengan
auth-groups-under = Grup ada di bawah
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (kosong: hanya memberOf)
auth-their-groups = Grup mereka
auth-group-filter-hint = { "{" }dn{ "}" } adalah entri orang tersebut, { "{" }user{ "}" } nama yang dipakainya untuk masuk.
auth-test = Uji
auth-test-hint = Menjangkau setiap server dan masuk sebagai akun pembaca. Dengan nama dan kata sandi, juga masuk sebagai orang itu dan menunjukkan bagaimana Hoard memperlakukannya. Tidak ada yang disimpan.
auth-test-user = Orang yang dipakai untuk masuk
auth-test-user-placeholder = Orang yang dipakai untuk masuk (opsional)
auth-test-password = Kata sandinya
auth-testing = Menguji…
auth-test-connectivity = Uji konektivitas
auth-test-server-ok = terjangkau, dan akun pembaca berhasil masuk
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }berhasil masuk (
auth-test-signed-in-after = ).
auth-test-role = Perannya: { $role }.
auth-test-no-role = Orang ini tidak ada di grup mana pun yang tercantum di Peran, jadi akan ditolak.
auth-test-groups = Grup: { $groups }
auth-test-no-groups = Tidak ada grup yang ditemukan.
auth-provider = Penyedia
auth-redirect = Alamat redirect
auth-copy = Salin
auth-redirect-hint = Berikan ini kepada penyedia saat Anda membuat klien Hoard. Alamat ini berasal dari alamat publik di Pengaturan → Koneksi, atau alamat yang sedang Anda pakai.
auth-issuer = Issuer
auth-discover = Temukan
auth-client-id = ID klien
auth-client-secret = Rahasia klien
auth-authorization-url = Alamat otorisasi
auth-token-url = Alamat token
auth-userinfo-url = Alamat info pengguna
auth-userinfo-placeholder = (kosong: token ID menyatakan siapa orangnya)
auth-scopes = Scope
auth-name-from = Nama dari
auth-groups-from = Grup dari
auth-groups-from-placeholder = (tidak ada)
auth-button = Tombol
auth-button-placeholder = akun Anda
auth-accounts = Akun
auth-provision = Buat akun Hoard saat seseorang pertama kali masuk
auth-provision-on-hint = Jika tidak, admin menambahkan setiap orang lebih dulu, di Pengaturan → Akun.
auth-provision-off-hint = Admin menambahkan setiap orang lebih dulu, di Pengaturan → Akun, sambil memilih cara mereka masuk.
auth-open-to-anyone = Siapa pun yang memiliki akun { $provider } dapat masuk. Tambahkan grup peninjau di bawah, atau matikan ini dan tambahkan orang sendiri.
auth-new-accounts = Akun baru
auth-roles-from-groups = Peran dari grup
auth-roles-hint = Grup yang anggotanya mendapat tiap peran, satu per baris, menurut nama atau DN. Yang tertinggi yang cocok berlaku, dan diperiksa setiap kali masuk. Jika tidak ada yang dicantumkan, akun baru mendapat peran di atas dan admin mengubahnya setelahnya. Jika ada grup yang dicantumkan di Dapat melihat, hanya anggota grup yang tercantum yang boleh masuk.
auth-roles-hint-oauth = Grup yang anggotanya mendapat tiap peran, satu per baris, menurut nama atau DN (atau apa pun yang dicantumkan klaim groups). Yang tertinggi yang cocok berlaku, dan diperiksa setiap kali masuk. Jika tidak ada yang dicantumkan, akun baru mendapat peran di atas dan admin mengubahnya setelahnya. Jika ada grup yang dicantumkan di Dapat melihat, hanya anggota grup yang tercantum yang boleh masuk.
auth-saving = Menyimpan…
auth-save = Simpan

## HttpsPanel.vue (Pengaturan → Koneksi)

https-load-failed = Tidak dapat memuat HTTPS: { $error }
https-off-redirecting = HTTPS mati. Mengalihkan Anda ke http://
https-now-on = HTTPS aktif
https-saved = Tersimpan. Koneksi baru memakai sertifikat baru.
https-now-off = HTTPS mati
https-http3-failed-toast = HTTP/3 tidak dapat dimulai: { $error }
https-copied = Disalin
https-copy-failed = Tidak dapat menyalin: pilih dan salin sendiri
https-mode-off = Mati
https-mode-off-hint = Hanya http:// biasa.
https-mode-own = Sertifikat Hoard sendiri
https-mode-own-hint = Hoard membuatnya untuk alamat-alamat server ini. Pasang otoritas sertifikat Hoard sekali di setiap perangkat, dan browsernya akan memercayainya.
https-mode-custom = Sertifikat saya
https-mode-custom-hint = Sertifikat yang sudah Anda miliki, mis. dari tailscale cert atau Let’s Encrypt. Browser memercayainya tanpa perlu memasang apa pun.
https-untrusted-own = Browser ini belum memercayainya untuk { $host }: pasang otoritas sertifikat Hoard di bawah. Anda akan melihat peringatan yang bisa dilewati.
https-untrusted-uncovered = Browser ini belum memercayainya untuk { $host }: sertifikat tidak mencakup alamat ini. Anda akan melihat peringatan yang bisa dilewati.
https-untrusted = Browser ini belum memercayainya untuk { $host }. Anda akan melihat peringatan yang bisa dilewati.
https-now = Saat ini
https-start-failed = HTTPS sudah diatur tetapi tidak dapat dimulai: { $error }
https-on-port = HTTPS aktif, di port { $port } seperti http://.
https-using-it = Anda sedang memakainya.
https-open = Buka { $url }
https-trusted = Browser ini memercayainya.
https-cert-for-before = Sertifikat untuk{ " " }
https-cert-for-after = , sampai { $date }.
https-cert-for-after-own = , sampai { $date } (diperbarui sendiri).
https-not-covered-before = Sertifikat ini tidak mencakup{ " " }
https-not-covered-after = , alamat yang sedang Anda pakai.
https-is-off = HTTPS mati: halaman disajikan lewat http:// biasa.
https-addresses = Alamat
https-addresses-hint = Sertifikat mencakup alamat yang ditemukan Hoard, dan alamat yang Anda tambahkan (nama yang diberikan router Anda, nama Tailscale).
https-remove-name = Hapus { $name }
https-another-address = Alamat lain
https-another-address-placeholder = Alamat lain, mis. nas.local
https-add = Tambah
https-your-cert = Sertifikat Anda
https-saved-cert-before = Tersimpan:{ " " }
https-saved-cert-after = { " " }dari { $issuer }, sampai { $date }. Tempel yang baru untuk menggantinya.
https-pem-before = Sertifikat (beserta rantainya) dan kunci privatnya, sebagai teks PEM: berkas yang diawali{ " " }
https-pem-after = .
https-certificate = Sertifikat
https-choose-file = Pilih berkas…
https-private-key = Kunci privat
https-checked-before = Untuk{ " " }
https-checked-after = , dari { $issuer }, sampai { $date }.
https-checked-after-days = , dari { $issuer }, sampai { $date } ({ $days } hari).
https-self-signed = Sertifikat ini self-signed: browser akan memperingatkannya.
https-key-matches = Kuncinya cocok.
https-where-to-get = Tempat mendapatkannya
https-get-tailscale-before = aktifkan HTTPS untuk tailnet Anda, lalu jalankan{ " " }
https-get-tailscale-after = { " " }di server dan unggah kedua berkasnya. Berlaku 90 hari: jalankan lagi dan unggah lagi sebelum itu.
https-get-acme-title = Let’s Encrypt atau klien ACME lain
https-get-acme-before = (certbot, acme.sh, lego) untuk domain milik Anda: unggah{ " " }
https-get-acme-and = { " " }dan{ " " }
https-get-acme-after = .
https-get-proxy-title = Reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) yang memegang sertifikat: biarkan HTTPS mati di sini dan biarkan proxy mengirim http:// biasa ke Hoard.
https-trying = Mencobanya…
https-apply = Terapkan
https-apply-hint = Hoard mencoba pengaturan baru lebih dulu, dan mempertahankan yang lama jika tidak berhasil.
https-trust-title = Percayai Hoard di perangkat Anda
https-download-ca = Unduh hoard-ca.crt
https-trust-hint = Pasang otoritas sertifikat Hoard sekali di setiap perangkat, dan browsernya akan memercayai server ini (dan sertifikat apa pun yang dibuat Hoard untuknya nanti). Otoritas ini hanya dapat menjamin Hoard ini. Sidik jari (SHA-256):
https-ca-windows = Buka berkasnya → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Mulai ulang browser.
https-ca-macos = Buka berkasnya; berkas itu masuk ke Keychain Access. Klik dua kali “{ $subject }” di sana → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone dan iPad
https-ca-ios = Unduh di Safari dan izinkan. Settings → Profile Downloaded → Install. Lalu Settings → General → About → Certificate Trust Settings → aktifkan “{ $subject }”.
https-ca-android = Settings → Security → More security settings → Encryption & credentials → Install a certificate → CA certificate, lalu pilih berkasnya. (Namanya sedikit berbeda antar ponsel; cari “certificate” atau “sertifikat” di Settings.)
https-ca-linux-1 = Debian dan Ubuntu:{ " " }
https-ca-linux-2 = . Fedora dan Arch:{ " " }
https-ca-linux-3 = . Chrome di Linux menyimpan daftarnya sendiri: Settings → Privacy and security → Security → Manage certificates → Import.
https-ca-firefox = Firefox menyimpan daftarnya sendiri: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import, lalu centang “Trust this CA to identify websites”.
https-ca-cli-title = Alat baris perintah hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , atau taruh path-nya di{ " " }
https-ca-cli-after = .
https-options = Opsi
https-redirect = Alihkan browser di perangkat lain dari http:// ke https://
https-redirect-hint = Hanya pemuatan halaman yang dialihkan. Alat baris perintah hoard, pekerja Unreal, skrip, dan aplikasi AI tetap berjalan di http://, begitu juga http://localhost di server itu sendiri.
https-http3-hint-before = Lebih cepat di Wi-Fi yang tidak stabil dan di ponsel. Memerlukan HTTPS, dan port UDP { $port } terbuka selain TCP (di Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Berjalan.
https-http3-waits = Dimulai saat HTTPS aktif.
https-http3-failed = HTTP/3 tidak dapat dimulai: { $error }
https-lockout-title = Jika Anda terkunci di luar
https-lockout-1 = { "" }
https-lockout-2 = { " " }di server itu sendiri selalu bisa lewat http:// biasa. Atau matikan HTTPS dari baris perintah server dan mulai Hoard lagi:{ " " }
https-lockout-3 = { " " }(di Docker:{ " " }
https-lockout-4 = , lalu{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Pengaturan → Akun)

account-role-view = Dapat melihat
account-role-view-hint = Mencari, melihat pratinjau, dan mengunduh
account-role-edit = Dapat mengubah
account-role-edit-hint = Juga menambahkan, mengubah, memberi tag, mengoleksi, dan mengarsipkan
account-role-admin = Admin
account-role-admin-hint = Juga Pengaturan, akun, pemeliharaan, dan menghapus permanen
account-source-internal = Kata sandi Hoard
account-source-ldap = Kata sandi { $directory } mereka
account-source-oauth = Masuk dengan { $provider }
account-source-word-directory = direktori
account-load-failed = Tidak dapat memuat akun: { $error }
account-failed = { $what }: { $error }
account-password-failed = Tidak dapat mengubah kata sandi Anda
account-password-changed = Kata sandi diubah. Perangkat Anda yang lain sudah dikeluarkan.
account-add-failed = Tidak dapat menambahkan { $name }
account-added = { $name } kini dapat masuk.
account-role-failed = Tidak dapat mengubah apa yang dapat dilakukan { $name }
account-remove-title = Hapus { $name }?
account-remove-body = Orang ini dikeluarkan di mana pun dan tidak dapat masuk lagi. Apa pun yang ditambahkannya ke pustaka tidak dihapus.
account-remove = Hapus
account-remove-failed = Tidak dapat menghapus { $name }
account-token-failed = Tidak dapat membuat token
account-copied = Disalin
account-copy-token-failed = Tidak dapat menyalin: pilih token dan salin sendiri
account-copy-failed = Tidak dapat menyalin: pilih dan salin sendiri
account-revoke-title = Cabut “{ $name }”?
account-revoke-body = Apa pun yang masih memakainya akan ditolak mulai sekarang.
account-revoke = Cabut
account-revoke-failed = Tidak dapat mencabut “{ $name }”
account-disconnect-title = Putuskan { $app }?
account-disconnect-body = Aplikasi ini ditolak mulai sekarang, dan harus dihubungkan lagi untuk memakai Hoard.
account-disconnect = Putuskan
account-disconnect-failed = Tidak dapat memutuskan { $app }
account-guest-failed = Tidak dapat mengubahnya
account-never = belum pernah
account-you = Anda
account-sign-out = Keluar
account-signed-in-as-before = Masuk sebagai{ " " }
account-signed-in-as-admin = , seorang admin.
account-signed-in-as-edit = , yang dapat mengubah pustaka.
account-signed-in-as-view = , yang dapat melihat tetapi tidak dapat mengubah apa pun.
account-app-signin = Aplikasi Hoard memasukkanmu sendiri, jadi tidak ada kata sandi. Buat token akses di bawah untuk plugin mesin dan perintah hoard.
account-password-at-provider = Kata sandi Anda disimpan oleh penyedia yang Anda pakai untuk masuk, jadi ubah di sana.
account-password-at-directory = Kata sandi Anda disimpan oleh direktori yang Anda pakai untuk masuk, jadi ubah di sana.
account-current-password = Kata sandi saat ini
account-new-password = Kata sandi baru
account-new-password-again = Ulangi kata sandi baru
account-change-password = Ubah kata sandi
account-password-mismatch = Kata sandi baru tidak cocok.
account-password-rule = Minimal 8 karakter.
account-people = Orang
account-people-hint = Setiap orang dapat melihat (mencari, melihat pratinjau, dan mengunduh), juga mengubah pustaka (menambahkan, mengedit, memberi tag, mengoleksi, dan mengarsipkan), atau sekaligus menjadi admin (Pengaturan, akun, pemeliharaan, dan menghapus permanen).
account-you-tag = (Anda)
account-signs-in-oauth = Masuk dengan penyedia OAuth
account-signs-in-directory = Masuk dengan direktori
account-since = sejak { $date }
account-what-can-do = Apa yang dapat dilakukan { $name }
account-name = Nama
account-how-sign-in = Cara masuk
account-password-label = Kata sandi (minimal 8 karakter)
account-password-placeholder = Kata sandi (8+ karakter)
account-what-they-can-do = Apa yang dapat dilakukannya
account-add = Tambah
account-ai-apps = Claude dan aplikasi AI lain
account-ai-apps-hint = Hubungkan Claude (atau aplikasi MCP apa pun) dan aplikasi itu dapat mencari dan mengambil aset untuk Anda, serta mendeskripsikan dan memberi tag apa yang belum dilakukan Hoard. Aplikasi itu masuk ke Hoard saat pertama kali; Anda mengizinkannya di sini, di browser.
account-mcp-address = Alamat MCP Hoard
account-copy = Salin
account-claude-code-1 = jalankan{ " " }
account-claude-code-2 = , lalu{ " " }
account-claude-code-3 = { " " }untuk masuk.{ " " }
account-claude-code-4 = { " " }mendeskripsikan sebuah paket.
account-claude-ai = Settings → Connectors → Add custom connector, tempel alamatnya, lalu Connect. Semua ini terhubung dari server Anthropic, jadi Hoard harus dapat dijangkau dari internet lewat HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, tempel alamatnya, lalu Connect. Semua ini terhubung dari server Anthropic, jadi Hoard harus dapat dijangkau dari internet lewat HTTPS (alamat ini tidak: atur Alamat publik di Pengaturan → Koneksi).
account-connection-meta = untuk { $user } sejak { $since } · terakhir dipakai { $used }
account-no-apps = Belum ada aplikasi yang terhubung.
account-tokens = Token akses
account-tokens-hint-1 = Untuk alat baris perintah{ " " }
account-tokens-hint-2 = , skrip, klien MCP, dan pekerja Unreal: semuanya mengirimnya sebagai{ " " }
account-tokens-hint-3 = . Token dapat melakukan apa yang Anda pilih, sebatas yang dapat Anda lakukan, tetapi tidak pernah mengelola akun atau token. Token yang hanya dapat melihat sudah cukup untuk mencari dan mengunduh, serta menyinkronkan proyek.
account-tokens-admin = Anda melihat token dan aplikasi terhubung milik semua orang; orang lain hanya melihat miliknya sendiri.
account-made-before = { "" }
account-made-after = : salin sekarang. Token ini tidak ditampilkan lagi.
account-new-token = Token baru
account-cli-1 = Untuk CLI:{ " " }
account-cli-2 = , atau taruh di{ " " }
account-cli-3 = .
account-done = Selesai
account-token-meta = dibuat { $made } · terakhir dipakai { $used }
account-token-meta-by = dibuat { $made } oleh { $by } · terakhir dipakai { $used }
account-no-tokens = Belum ada token.
account-token-name = Apa yang akan memakainya
account-token-name-placeholder = Apa yang akan memakainya, mis. “Pekerja Unreal”
account-token-role = Apa yang dapat dilakukannya
account-make-token = Buat token
account-guests = Tamu
account-guests-allow = Izinkan siapa pun yang dapat menjangkau server menelusuri tanpa masuk
account-guests-hint = Tamu dapat mencari, melihat pratinjau, dan mengunduh, tetapi tidak dapat menambahkan, mengubah, atau menghapus apa pun, atau melihat Pengaturan.

## SignIn.vue

signin-create-title = Buat akun Anda
signin-title = Masuk
signin-setup-lede = Server ini belum memiliki akun. Akun yang Anda buat di sini dapat melakukan apa saja; tambahkan akun untuk orang lain dari Pengaturan → Akun nanti.
signin-overlay-lede = Siapa pun dapat menelusuri di sini, tetapi perubahan memerlukan akun.
signin-with = Masuk dengan { $provider }
signin-or-password = atau dengan nama dan kata sandi
signin-username = Nama pengguna
signin-name = Nama
signin-new-password = Kata sandi baru
signin-password = Kata sandi
signin-password-rule = Minimal 8 karakter.
signin-confirm-password = Konfirmasi kata sandi
signin-mismatch = Kedua kata sandi tidak cocok.
signin-setup-code = Kode penyiapan
signin-setup-code-hint-before = Kode yang diberikan kepada server ini di{ " " }
signin-setup-code-hint-after = .
signin-creating = Membuat…
signin-signing-in = Masuk…
signin-create = Buat akun
signin-submit = Masuk
signin-keep-browsing = Lanjut menelusuri
signin-directory-hint = Pakai nama dan kata sandi { $directory } Anda.
signin-forgot-1 = Lupa kata sandi Hoard? Di server, jalankan{ " " }
signin-forgot-2 = { " " }(di Docker,{ " " }
signin-forgot-3 = ) untuk membuat yang baru.

## SetupWizard.vue (kunjungan pertama ke server baru)

setup-title = Di mana pustaka Anda akan disimpan?
setup-lede = Pilih folder di server tempat Hoard menyimpan paket Anda, atau arahkan ke folder berisi paket yang sudah Anda miliki. Anda dapat menambahkan lokasi lain, di disk lain, dari Pengaturan nanti.
setup-lede-desktop = Pilih folder tempat Hoard menyimpan paket Anda, atau arahkan ke folder berisi paket yang sudah Anda miliki. Anda dapat menambahkan lokasi lain, di disk lain, dari Pengaturan nanti.
setup-unreachable = Tidak dapat menjangkau server: { $error }
setup-unreachable-desktop = Hoard tidak merespons: { $error }
setup-try-again = Coba lagi
setup-loading = Memuat…
setup-start = Mulai pustaka
setup-ready = Pustaka Anda siap
setup-managed-before = Paket baru masuk ke{ " " }
setup-managed-after = . Jatuhkan berkas, folder, atau arsip di mana saja di halaman ini untuk menambahkan yang pertama.
setup-watched-before = Hoard memantau{ " " }
setup-watched-after = ; paket muncul begitu ditemukan. Untuk juga mengunggah, tambahkan pustaka yang disimpan Hoard dari Pengaturan → Lokasi pustaka.
setup-open = Buka pustaka

## ConnectApp.vue (aplikasi seperti Claude meminta masuk)

connect-title = Hubungkan { $app }?
connect-lede-1 = { " " }akan dapat mencari, mengunduh, mendeskripsikan, dan memberi tag semua isi Hoard ini, atas nama{ " " }
connect-lede-2 = . Aplikasi ini tidak dapat mengelola akun atau token akses.
connect-back-before = Anda akan kembali ke{ " " }
connect-back-after = . Putuskan kapan saja di Pengaturan → Akun.
connect-allow = Izinkan
connect-cancel = Batal
connect-failed = Tidak dapat menghubungkan aplikasi ini
connect-go-library = Ke pustaka
connect-checking = Memeriksa…

## InsecureNotice.vue (baris yang tampil di atas http:// biasa)

insecure-text = Ini http:// biasa, jadi beberapa hal memerlukan HTTPS: 3D dengan WebGPU, clipboard modern, memasang Hoard sebagai aplikasi, dan konektor Claude.
insecure-switch = Beralih ke https://
insecure-set-up = Atur HTTPS
insecure-dismiss = Jangan tampilkan lagi

## FormatWizard.vue (Pengaturan → Berkas toko → mengajari Hoard berkas toko)

formatwizard-too-big = Berkas ini terlalu besar untuk dibaca sebagai berkas toko.
formatwizard-not-json = Berkas ini bukan JSON yang valid, jadi ditampilkan sebagai teks.
formatwizard-read-failed = Tidak dapat membaca berkas: { $error }
formatwizard-back-title = Kembali ke formulir?
formatwizard-back-body = Perubahan di TOML hilang: formulir menulis ulang format dari isinya sendiri.
formatwizard-back-to-form = Kembali ke formulir
formatwizard-miss-name = Berkasnya tidak memakai nama { $file }. Tambahkan ke Berkas.
formatwizard-miss-not-the-stores = Penanda berkas toko yang dipakainya tidak ada di berkas ini. Ubah Kenali dari.
formatwizard-miss-no-title = Tidak ada judul yang ditemukan di berkas ini. Pilih judulnya.
formatwizard-miss-short-name = bukan namanya
formatwizard-miss-short-not-the-stores = bukan milik toko
formatwizard-miss-short-no-title = tanpa judul
formatwizard-saved-packs = { $file } disimpan. { $packs } paket sedang dibaca lagi.
formatwizard-saved = { $file } disimpan.
formatwizard-save-failed = Tidak dapat menyimpan format: { $error }
formatwizard-zip-failed = Tidak dapat membuat zip: { $error }
formatwizard-stop-title = Berhenti mengajarkan berkas ini?
formatwizard-stop-body = Format tidak disimpan.
formatwizard-stop = Berhenti
formatwizard-label = Wizard format
formatwizard-changing = Mengubah { $name }
formatwizard-starting-from = Dimulai dari { $name }
formatwizard-title = Ajari Hoard berkas toko
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }di{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Edit sebagai TOML
formatwizard-file-reference = Berkasnya, sebagai rujukan.
formatwizard-click-json = Klik { $field } di berkas.
formatwizard-click-text = Klik { $field } di berkas, atau pilih teksnya.
formatwizard-pick-first = Pilih kolom di kanan, lalu klik nilainya di sini.
formatwizard-loading = Memuat…
formatwizard-the-format = Formatnya
formatwizard-syntax-before = Sintaksnya ada di{ " " }
formatwizard-syntax-after = .
formatwizard-store = Toko
formatwizard-store-placeholder = Nama toko, seperti Kenney
formatwizard-files = Berkas
formatwizard-recognize = Kenali dari
formatwizard-recognize-json = Kunci yang hanya dimiliki berkas toko
formatwizard-recognize-text = Baris yang hanya dimiliki berkas toko
formatwizard-recognize-hint = Satu per baris; salah satunya sudah cukup. Diperlukan jika berkas lain memiliki nama yang sama.
formatwizard-seller = Penjual
formatwizard-seller-placeholder = Jika berkas tidak menyebutkannya
formatwizard-tags = Tag
formatwizard-tags-placeholder = Untuk semua yang dideskripsikannya, dipisahkan koma
formatwizard-fields = Kolom
formatwizard-selector = Selektor { $field }
formatwizard-clear = Hapus { $field }
formatwizard-pick = Pilih
formatwizard-what-it-reads = Yang dibacanya
formatwizard-read-title = Judul
formatwizard-read-seller = Penjual
formatwizard-read-license = Lisensi
formatwizard-read-url = Halaman toko
formatwizard-read-kind = Jenis
formatwizard-read-tags = Tag
formatwizard-read-origin = Dibuat di
formatwizard-read-size = Ukuran
formatwizard-also = { $format } juga membaca berkas ini. Format ini akan dicoba lebih dulu.
formatwizard-library = { $files } berkas di { $packs } paket memiliki nama yang diterimanya. Dari { $tried } yang dicoba, { $read } terbaca.
formatwizard-library-none = Tidak ada berkas di pustaka yang memiliki nama yang diterimanya.
formatwizard-share = Bagikan
formatwizard-share-hint = Zip berisi format, berkas ini sebagai sampelnya, dan hasil bacaan sampel itu, disusun seperti repositori Hoard, untuk dikirim agar semua orang mendapatkannya. Keluarkan dulu apa pun di sampel yang tidak boleh dibagikan.
formatwizard-sample = Sampel untuk dibagikan
formatwizard-download = Unduh { $file }
formatwizard-close = Tutup
formatwizard-use = Pakai di sini
formatwizard-cancel = Batal
