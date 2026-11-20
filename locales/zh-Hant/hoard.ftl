## 伺服器錯誤

error-account-not-found = 找不到帳號
error-asset-gone = 找不到素材
error-asset-not-found = 找不到素材 { $id }
error-blob-not-found = 找不到 blob { $hash }
error-collection-not-found = 找不到收藏集 { $id }
error-connection-not-found = 找不到連線
error-derivative-not-found = 找不到 { $id } 的 { $kind } 衍生檔案
error-file-in-pack-not-found = 在素材包 { $id } 中找不到 { $path }
error-site-not-a-page = 只有網頁才能以頁面方式開啟
error-site-link-expired = 此頁面的網址已過期，請從 Hoard 重新開啟該頁面
error-format-not-found = 找不到格式 { $file }
error-frame-sheet-aseprite-only = 只有 Aseprite 檔案才有影格表
error-frame-sheet-frames = 這個檔案只有一個影格，或影格太多而無法排列
error-location-not-found = 找不到位置 { $id }
error-maintenance-task-not-found = 找不到維護作業 { $id }
error-open-upload-not-found = 找不到進行中的上傳 { $id }
error-original-not-found = 找不到 { $id } 的原始檔案
error-pack-not-found = 找不到素材包 { $id }
error-preview-too-big = 這張圖片太大，無法製作輕量預覽
error-project-not-found = 找不到專案 { $name }
error-saved-search-not-found = 找不到已儲存的搜尋 { $id }
error-sign-in-first = 請先登入（或傳送存取權杖）
error-those-assets-not-found = 找不到這些素材
error-thumbnail-not-found = 找不到 { $id } 的縮圖
error-token-not-found = 找不到權杖
error-upload-not-found = 找不到上傳 { $id }
error-pack-needs-name = 素材包需要名稱
error-no-library = 目前還沒有可存放檔案的素材庫位置：請在「設定 → 素材庫位置」新增一個受管理的位置
error-upload-failed = 上傳失敗：{ $reason }
error-upload-no-files = 上傳內容中沒有檔案
error-analyze-nothing-to-do = 沒有需要做的事：rules=false 且 LLM 已關閉
error-too-many-assets = 單次請求的素材太多
error-too-many-items = 單次請求的項目太多
error-not-a-hash = 不是 blake3 十六進位雜湊值
error-merge-into-tag = into 必須是標籤
error-nothing-to-merge = 沒有可合併的項目
error-selection-size = 請提供 1 到 { $max } 個素材 ID
error-collection-needs-name = 收藏集需要名稱
error-collection-exists = 已經有名為 { $name } 的收藏集
error-collection-empty = 收藏集 { $name } 是空的
error-conversion-failed = 無法轉換為 { $format }：{ $reason }
error-max-size = max 必須是 64 到 16384 之間的 2 的次方
error-replace-ext = ext 必須是簡短的副檔名
error-empty-file = 空白檔案
error-imitation-silent = 錄音中沒有比背景更突出的聲音。請靠近麥克風再試一次。
error-replace-duplicate = 轉換後的檔案與素材包中既有的檔案重複；已將這個套件合併到該檔案
error-ai-address = AI 網址應以 http:// 或 https:// 開頭
error-hosted-key = 請輸入 { $provider } 的 API 金鑰
error-hosted-address = 圖片搜尋服務的網址應以 http:// 或 https:// 開頭
error-local-llm-action = action 必須是 list、download 或 delete，後兩者需要指定模型
error-role = 角色必須是 view（檢視）、edit（編輯）或 admin（管理）
error-admin-only = 只有管理員能執行這項操作
error-view-only = 這個帳號只能檢視、無法變更任何內容：管理員可以在「設定 → 帳號」中變更
error-portal-only = 請登入入口網站來管理帳號與權杖
error-not-from-portal = 這個請求不是來自 Hoard 入口網站
error-account-name = 請選擇最多 64 個字元的名稱
error-password-length = 請使用至少 { $min } 個字元的密碼
error-too-many-codes = 錯誤的代碼太多次：請等待 { $seconds } 秒
error-already-set-up = 已經有帳號了：請登入
error-too-many-tries = 錯誤嘗試太多次：請等待 { $seconds } 秒
error-directory-unavailable = 目錄無法檢查你的密碼：{ $reason }
error-password-elsewhere = 你的密碼由你登入時使用的目錄或提供者保管
error-current-password = 你目前的密碼不正確
error-account-source = 帳號可透過 Hoard（internal）、目錄（ldap）或 OAuth 登入
error-account-exists = 已經有使用這個名稱的帳號
error-own-role = 你的角色需由其他管理員變更
error-remove-self = 你無法在登入狀態下移除自己的帳號
error-token-name = 請以會使用這個權杖的程式來命名，最多 80 個字元
error-token-role = 權杖的權限不能超過建立它的帳號
error-wrong-setup-code = 這不是在 HOARD_SETUP_CODE 中提供給伺服器的設定代碼
error-wrong-password = 名稱與密碼不符
error-folder-missing = { $path } 不存在（磁碟沒有掛載？）
error-s3-path = S3 位置的格式為 s3://bucket
error-s3-address = S3 網址必須以 http:// 或 https:// 開頭
error-s3-header = 額外的 S3 標頭格式應為 Name=value，而「{ $header }」不是
error-keychain = 無法存放到系統鑰匙圈：{ $reason }
error-not-built = 此 Hoard 建置時未包含 { $feature }
error-openai-auth-unreachable = 無法連線到 auth.openai.com
error-openai-unexpected = OpenAI 傳回了非預期的回應
error-openai-answered = OpenAI 回應 { $status }：{ $reason }
error-chatgpt-id-token-not-jwt = ID 權杖不是 JWT
error-chatgpt-id-token-issuer = ID 權杖來自 { $issuer }，而不是 { $expected }
error-chatgpt-id-token-audience = ID 權杖是發給其他應用程式的
error-chatgpt-id-token-expired = ID 權杖已過期
error-chatgpt-id-token-nonce = ID 權杖不屬於這次登入
error-chatgpt-no-state = 網址中沒有 state；請複製完整網址
error-chatgpt-not-started = 這次登入不是從這裡開始的，或已經開始超過 15 分鐘。請從「設定」重新開始。
error-chatgpt-canceled = 登入已取消
error-chatgpt-refused = ChatGPT 拒絕了登入：{ $reason }
error-chatgpt-no-code = 網址中沒有登入代碼；請複製完整網址
error-chatgpt-other-registration = ChatGPT 回應的是這台伺服器的另一筆註冊；請重新登入
error-chatgpt-other-registration-desktop = ChatGPT 回應的是另一份 Hoard；請重新登入
error-chatgpt-not-registered = ChatGPT 尚未完成 Hoard 的註冊；請重新登入
error-chatgpt-code-used = 登入代碼已使用過或已過期；請重新登入
error-chatgpt-refused-code = ChatGPT 拒絕了登入（{ $code }）
error-chatgpt-no-id-token = ChatGPT 沒有傳送 ID 權杖
error-chatgpt-plan-not-allowed = 已登入，但未獲准使用方案。請重新登入並允許，或改用 API 金鑰。
error-chatgpt-problem = ChatGPT：{ $problem }
error-chatgpt-not-signed-in = 尚未登入 ChatGPT；請從「設定 → AI 標記」登入
error-chatgpt-refresh-later = ChatGPT 目前無法更新登入狀態（{ $code }）；稍後會再試一次
error-chatgpt-signed-out-problem = ChatGPT 已將這台伺服器登出（{ $code }）。請重新登入，以繼續用你的方案加上標籤。
error-chatgpt-signed-out-problem-desktop = ChatGPT 已將 Hoard 登出（{ $code }）。請重新登入，以繼續用你的方案加上標籤。
error-chatgpt-signed-out = ChatGPT 已將這台伺服器登出（{ $code }）；請從「設定 → AI 標記」重新登入
error-chatgpt-signed-out-desktop = ChatGPT 已將 Hoard 登出（{ $code }）；請從「設定 → AI 標記」重新登入
error-openai-api-unreachable = 無法連線到 api.openai.com
error-openai-broke-off = OpenAI 的回覆中斷了
error-chatgpt-kept-refusing = OpenAI 一直拒絕 ChatGPT 登入；請從「設定 → AI 標記」重新登入
error-chatgpt-request-failed = 請求失敗
error-chatgpt-cut-off = 回覆被截斷了（{ $reason }）
error-chatgpt-cut-off-no-reason = 回覆被截斷了（未提供原因）
error-openai-ended-early = OpenAI 的回覆在完成前就結束了
error-chatgpt-usage-limit = 你的 ChatGPT 方案目前已用完 Hoard 的使用額度。請到 { $url } 查看或變更上限
error-chatgpt-not-eligible = 這個 ChatGPT 帳號或工作區無法在其他應用程式中使用方案。請改用 API 金鑰。
error-chatgpt-usage-unavailable = ChatGPT 目前無法檢查方案的使用量；稍後會再試一次
error-chatgpt-unsupported = ChatGPT 方案用量不支援在這個模型使用 { $param }
error-chatgpt-unsupported-images = ChatGPT 方案用量不支援在這個模型使用 { $param }；請選擇可接收圖片的模型，或關閉傳送縮圖
error-chatgpt-unsupported-part = ChatGPT 方案用量不支援在這個模型使用請求中的部分內容
error-chatgpt-port = port 是瀏覽器連線到 Hoard 時使用的連接埠
error-chatgpt-paste-address = 請貼上完整網址，開頭是 http://127.0.0.1
error-too-many-hashes = 一次最多 { $max } 個雜湊值
error-credits-pack = 素材包 { $key }
error-credits-collection = 收藏集 { $key }
error-format-too-long = 格式太長
error-not-started = 伺服器尚未啟動
error-not-started-desktop = Hoard 尚未啟動
error-not-format-file = 不是格式檔案：{ $name }
error-format-name = 名稱需要包含 A 到 Z 的字母或 0 到 9 的數字
error-format-miss-name = 這個格式的檔案不包含 { $file }
error-format-miss-store = 這個格式不會將 { $file } 當作商店的檔案（請見 require）
error-format-miss-title = 這個格式在 { $file } 中找不到標題
error-cant-download-as = .{ $ext } 檔案無法下載為 { $format }
error-only-images-resize = 只有圖片可以調整大小
error-https-no-certificate = 請先貼上或上傳你的憑證及其金鑰
error-https-no-cert-found = 找不到憑證：請貼上以 -----BEGIN CERTIFICATE----- 開頭的文字
error-https-no-key-found = 找不到私密金鑰：請貼上以 -----BEGIN PRIVATE KEY----- 開頭的文字
error-https-several-keys = 有不只一把私密金鑰：請只貼上與憑證配對的那一把
error-https-key-password = 金鑰受密碼保護：請提供沒有密碼的版本（openssl pkey -in key.pem -out plain.pem）
error-https-cert-not-pem = 憑證不是有效的 PEM
error-https-key-not-pem = 私密金鑰不是有效的 PEM
error-https-key-mismatch = 金鑰與憑證不相符。如果你提供的是憑證鏈，請將伺服器的憑證放在最前面
error-https-unusable = 無法使用這個憑證或金鑰
error-https-expired = 這個憑證已於 { $date } 過期
error-https-not-yet-valid = 這個憑證要到 { $date } 才會生效
error-https-unreadable = 這不是 Hoard 可以讀取的憑證
error-https-bad-name = 「{ $name }」不是憑證可以包含的位址
error-https-test-timed-out = 測試連線逾時
error-https-test-failed = 測試連線失敗
error-https-no-ca = Hoard 尚未建立自己的憑證授權單位
error-https-unavailable = 這台伺服器無法使用 HTTPS
error-https-certificate-mode = 憑證需搭配「我的憑證」使用
error-https-test = 憑證在測試連線中無法運作：{ $reason }
error-nothing-to-retry = 沒有可以重試的項目
error-ai-unreachable = 無法連線到 { $url }
error-ai-key-refused = { $url } 拒絕了 API 金鑰（{ $status }）
error-ai-answered = { $url } 回應 { $status }：{ $reason }
error-ai-not-json = { $url }/models 沒有以 JSON 回應
error-model-server-refused = 模型伺服器：{ $reason }
error-no-model-server = { $url } 沒有模型伺服器回應
error-not-llama-router = { $url } 不是路由模式的 llama.cpp 伺服器，因此無法下載模型
error-model-download = 無法下載 { $model }：{ $reason }
error-model-delete = 無法刪除 { $model }：{ $reason }
error-unknown-kind = 未知的素材類型 { $kind }
error-watched-is-folder = 監看位置是這台伺服器上的資料夾
error-cant-reach = 無法連線到 { $path }：{ $reason }
error-folder-full-path = 請提供資料夾的完整路徑
error-cant-create = 無法建立 { $path }
error-cant-write = 無法寫入 { $path }
error-cant-read = 無法讀取 { $path }
error-overlaps-location = 這與位置 { $name }（{ $path }）重疊
error-overlaps-data = 這與 Hoard 的資料資料夾（{ $path }）重疊
error-overlaps-cache = 這與 Hoard 的快取資料夾（{ $path }）重疊
error-not-watched = { $name } 不是監看資料夾
error-watched-no-kinds = 監看資料夾不接收新檔案，因此沒有可變更的類型
error-keep-catch-all = 請改將另一個位置設為全部接收的位置
error-location-holds-files = { $name } 仍有檔案：請先將檔案移到其他位置（POST /api/admin/locations/{ $id }/empty）
error-location-unreachable = 無法連線到 { $name }，因此 Hoard 無法判斷其中是否仍有檔案
error-relocate-running = 目前已在位置之間移動檔案；請等完成後再試一次
error-watched-not-moved = { $name } 是監看資料夾：Hoard 不會移動其中的檔案
error-no-other-location = 沒有其他素材庫位置可以移入檔案：請先新增一個
error-full-path = 請提供完整路徑
error-pick-folder = 請選擇資料夾
error-task-running = 「{ $task }」已經在執行中
error-token-account-gone = 這個權杖的帳號已不存在，因此沒有最愛或搜尋
error-favorites-too-many = 一次最多 10,000 個
error-saved-search-name = 已儲存的搜尋需要最多 80 個字元的名稱
error-saved-search-too-long = 這個搜尋太長，無法儲存
error-hoard-seen = 寶藏最多可記錄 32 種素材，每種都須以小寫字母命名
error-texture-pattern-map = 貼圖名稱樣式需要包含 { "{" }map{ "}" }，否則材質的每張貼圖都會得到相同的名稱
error-texture-pattern-fields = 貼圖名稱樣式可以使用 { "{" }name{ "}" }、{ "{" }map{ "}" } 和 { "{" }size{ "}" }
error-texture-name-chars = 貼圖名稱不能包含 < > : " / \ | ? 或 *
error-texture-map-type = 沒有名為 { $role } 的貼圖類型
server-role-base-color = 顏色
server-role-normal = 法線
server-role-normal-dx = 法線（DirectX）
server-role-roughness = 粗糙度
server-role-gloss = 光澤度
server-role-metalness = 金屬度
server-role-ao = 環境光遮蔽
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = 高度
server-role-bump = 凹凸
server-role-cavity = 凹洞
server-role-specular = 鏡面反射
server-role-opacity = 不透明度
server-role-emissive = 自發光
server-role-translucency = 半透明
error-texture-suffix-missing = 請為{ $map }貼圖設定字尾
error-texture-suffix-chars = { $map }貼圖的字尾不能包含 < > : " / \ | ? 或 *
error-texture-ending-missing = 每個自訂結尾都需要填寫要尋找的結尾
error-texture-ending-chars = 自訂結尾不能包含 < > : " / \ | ? 或 *
error-texture-ending-twice = { $ending } 在你的結尾中出現了兩次
error-oauth-no-client-id = 這個連結沒有 client_id。
error-oauth-unknown-app = 這個應用程式未在此註冊（或已不再註冊）。請從應用程式重新開始連線。
error-oauth-redirect = 應用程式要求導回一個它沒有註冊過的網址。
error-oauth-response-type = Hoard 只核發登入代碼（response_type=code）。
error-oauth-pkce = 應用程式必須使用 PKCE 搭配 S256 才能登入 Hoard。
error-light-preview-images = 只有圖片有輕量預覽
error-no-light-preview = 沒有輕量預覽：{ $reason }
error-no-frame-sheet = 沒有影格表：{ $reason }
error-thumbnail-source = 未知的縮圖來源：{ $source }
error-thumbnail-too-big = 這張圖片太大，無法當作縮圖
error-thumbnail-models-only = 只有模型可以使用由瀏覽器繪製的縮圖
error-not-a-picture = 不是 Hoard 可以讀取的圖片：{ $reason }
error-project-name = 專案需要最多 200 個字元的名稱
error-project-too-big = 一個專案最多一百萬個素材
error-project-too-many-at-once = 一次最多 100,000 個
error-server-name = 請選擇最多 { $max } 個字元的單行名稱
error-tool-setting = { $tool }：請選擇關閉、自動，或提供路徑
error-ai-model = 請選擇 AI 服務要使用的模型
error-public-address = 公開網址應以 http:// 或 https:// 開頭
error-update-hook-address = 更新時要呼叫的網址應以 http:// 或 https:// 開頭
error-update-hook-auth = Authorization 標頭不能包含換行或其他控制字元
error-no-soundfont = 伺服器上的 { $path } 沒有 SoundFont
error-no-soundfont-desktop = { $path } 沒有 SoundFont
error-soundfont-bad = { $name } 不是 MIDI 能用來播放的 SoundFont：{ $reason }
error-workers = 背景工作程序：1 到 { $max } 個
error-ai-batch = 每次 AI 請求的素材數：1 到 200 個
error-conversion-cache = 轉換後的下載檔案：至少 100 MB
error-backup-keep = 排程備份：可保留 1 到 { $max } 份
error-backup-copy-to = 排程備份：{ $path } 不是伺服器上的資料夾（請填寫完整路徑）
error-conversion-keep-days = 轉換規則：原始檔案保留 1 到 { $max } 天
error-conversion-target = 轉換規則：Hoard 無法將檔案轉換為 { $format }
error-conversion-no-source = 轉換規則：每條規則至少需要一種要轉換的格式
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = 轉換規則：.{ $ext } 檔案無法轉換為 { $format }
error-conversion-twice = 轉換規則：.{ $ext } 出現在兩條規則中
error-conversion-cant-undo = 此檔案不是由規則轉換的，或其原始檔案已被刪除。
error-conversion-path-taken = { $path } 現在有另一個檔案，因此原始檔案無法放回原處。
error-limits-zero = 上限不能為零
error-settings-object = 設定應為 JSON 物件
error-bad-setting = 無效的設定：{ $reason }
error-auth-default-role = 預設角色必須是 view（檢視）、edit（編輯）或 admin（管理）
error-auth-session-hours = 工作階段的有效期間為 1 小時到 1 年
error-auth-settings = 無效的驗證設定：{ $reason }
error-ldap-settings = 無效的 LDAP 設定：{ $reason }
error-ldap-no-server = 請新增目錄伺服器的位址
error-ldap-base-dn = 請指定使用者項目的位置（base DN）
error-ldap-username-attribute = 請指定哪個屬性存放使用者登入時使用的名稱
error-ldap-bind-dn = 請提供用來查詢使用者的帳號，或以匿名方式查詢
error-ldap-ca-cert = 憑證授權單位不是 PEM 憑證（-----BEGIN CERTIFICATE-----）
error-ldap-empty-server = 目錄伺服器的位址是空的
error-ldap-server-form = { $server }：目錄伺服器的位址格式為 host:port、ldap://… 或 ldaps://…
error-ldap-not-starttls = { $server }：這是 LDAPS，不是 StartTLS
error-ldap-no-certificate = 其中沒有憑證
error-ldap-password-again = 請再次提供用來查詢使用者的帳號密碼：已儲存的密碼只會傳送給當初提供它的伺服器
error-ldap-unreachable = 無法連線到任何目錄伺服器（{ $reasons }）
error-ldap-no-anonymous = 目錄不允許以匿名方式查詢使用者（{ $reason }）
error-ldap-bind-failed = 用來查詢使用者的帳號無法登入（{ $reason }）
error-ldap-search = 正在 { $base } 下尋找 { $name }
error-ldap-group-search = 正在 { $base } 下尋找 { $name } 所屬的群組
error-ldap-several = 目錄中有 { $count } 個項目符合 { $name }：請縮小使用者篩選條件
error-ldap-password-check = 目錄不願檢查 { $name } 的密碼（{ $reason }）
error-ldap-wrong-password = 目錄中沒有使用這組密碼的 { $name }
error-oauth-client-id = 請提供提供者為 Hoard 建立的用戶端 ID
error-oauth-addresses = 授權網址與權杖網址應以 https:// 開頭
error-oauth-userinfo-address = 使用者資訊網址應以 https:// 開頭
error-oauth-username-claim = 請指定哪個 claim 存放使用者的名稱
error-oauth-secret-again = 請再次提供用戶端密碼：已儲存的密碼只會傳送給當初提供它的權杖網址
error-oauth-issuer = 簽發者應以 https:// 開頭
error-oauth-discovery-unreachable = 無法連線到 { $url }：{ $reason }
error-oauth-discovery-status = { $url } 回應 { $status }
error-oauth-discovery-bad = { $url } 不是探索文件
error-oauth-discovery-incomplete = { $url } 沒有指出授權網址與權杖網址
error-signin-no-group = { $name } 不在可以使用 Hoard 的群組中
error-signin-other-source = 已經有名為 { $name } 的 Hoard 帳號，且使用其他方式登入
error-signin-other-subject = 已經有名為 { $name } 的 Hoard 帳號，屬於提供者中的另一個人
error-signin-no-account = { $name } 還沒有 Hoard 帳號：管理員可以在「設定 → 帳號」中新增
error-signin-bad-name = 這個名稱不能作為 Hoard 帳號（最多 64 個字元）
error-oauth-off = 尚未開啟 OAuth 登入
error-oauth-bad-authorization-address = 授權網址無效
error-oauth-refused = 提供者沒有讓你登入：{ $reason }
error-oauth-no-code = 提供者將你導回，但沒有完成登入
error-oauth-expired = 這次登入花太久時間，或是從其他地方開始的：請再試一次
error-oauth-no-claim = 提供者沒有說明你的身分（沒有 { $claim } claim）
error-oauth-no-sub = 提供者沒有說明你的身分（沒有 sub 或 id claim）
error-oauth-token-unreachable = 無法連線到提供者的權杖網址
error-oauth-token-refused = 提供者不願完成登入：{ $reason }
error-oauth-no-access-token = 提供者沒有傳送存取權杖
error-oauth-userinfo-unreachable = 無法連線到提供者的使用者資訊網址
error-oauth-userinfo-refused = 提供者不願說明是誰登入（{ $status }）
error-oauth-userinfo-not-json = 提供者的使用者資訊不是 JSON
error-format-no-name = name 是空的
error-format-no-files = files 是空的：請指定這個格式要讀取哪些檔案，例如 ["*.json"]
error-format-empty-selector = 空的
error-format-unopened-bracket = 有右括號但沒有對應的左括號
error-format-unclosed-bracket = 有括號沒有閉合
error-format-empty-step-dots = 空的步驟（連續兩個點，或開頭或結尾有點）
error-format-unclosed-except = !( 沒有閉合
error-format-unclosed-filter = [ 沒有閉合
error-format-after-filters = { $step } 的鍵與篩選條件之後有 { $rest }
error-format-except-needs-glob = !(…) 必須放在含有 * 的鍵之後
error-format-except-not-glob = !(…) 必須放在含有 * 的鍵之後，而不是 { $key }
error-format-empty-step = 空的步驟
error-format-not-a-filter = [{ $filter }] 不是篩選條件：請使用 []、[key=value|other]、[array]、[string]、[number]、[bool] 或 [object]
error-format-filter-needs-value = [{ $filter }] 需要鍵和值
error-store-search-empty = 沒有要搜尋的內容；請輸入素材包的名稱
error-update-hook-not-set = 請先設定要呼叫的網址
error-upload-stalled = { $path } 的上傳停滯了；請重新傳送該部分
error-upload-busy = { $path } 正在寫入中
error-upload-incomplete = 有 { $count } 個檔案不完整，例如 { $files }
error-upload-no-files-listed = 沒有檔案
error-upload-not-part = { $path } 不屬於上傳 { $id }，或該上傳已關閉
error-bad-path = 無效的路徑
error-upload-part-offset = { $offset } 不是 { $path } 某個部分的起始位置
error-upload-interrupted = 上傳中斷：{ $reason }
error-upload-part-long = { $path } 的第 { $part } 部分超過 { $len } 位元組
error-upload-part-short = { $path } 的第 { $part } 部分：收到 { $got } 位元組，預期為 { $len }
error-upload-file-long = { $path } 比宣告的大小還大
error-upload-file-short = { $path }：收到 { $got } 位元組，預期為 { $len }
error-upload-complete = 上傳已經完成
error-already-imported = 已匯入為「{ $name }」

## 伺服器文字

server-store-place-readme = README
server-store-place-folders = 資料夾
server-store-why-linked = 素材包的 { $place } 中有連結到它
server-store-why-same-title = 與素材包的 { $place } 標題相同
server-store-why-by = 作者為 { $author }，素材包的 { $place } 中有提到
server-store-why-meshes = 網格數量與素材包相近（{ $count }）
server-store-why-textures = 貼圖數量與素材包相近（{ $count }）
server-chatgpt-revoke-failed = 已在這裡登出，但無法通知 OpenAI。為了保險起見，請在 ChatGPT 的「設定 → 安全性與登入 → 使用 ChatGPT 登入」中中斷 Hoard 的連線。
server-chatgpt-signed-in-title = 已登入 ChatGPT
server-chatgpt-signed-in = Hoard 已登入。AI 標記現在可以使用你的 ChatGPT 方案了。
server-chatgpt-signed-in-as = Hoard 已以 { $email } 登入。AI 標記現在可以使用你的 ChatGPT 方案了。
server-chatgpt-not-finished-title = 登入尚未完成
server-chatgpt-other-computer = 如果你是在另一台電腦上的 Hoard 開始登入的，請從網址列複製這個頁面的網址，貼到那台電腦上 Hoard 的「設定」中。
server-back-to-settings = 返回設定
server-issues-pack = 無法新增的素材包
server-issues-pack-blurb = 刪除素材包後重新新增，或開啟它查看其中有什麼。
server-issues-damaged = 已損毀或遺失的儲存檔案
server-issues-damaged-blurb = 由「檢查檔案是否損毀」發現。請從備份還原，或重新新增所屬的素材包，然後再次執行「檢查檔案是否損毀」。
server-issues-folder = 無法讀取的監看資料夾
server-issues-folder-blurb = 在資料夾可以再次讀取之前，其中的素材包會維持原狀。
server-issues-missing = 資料夾已消失的監看素材包
server-issues-missing-blurb = 每個素材包會保留 3 天，以防資料夾只是暫時離線，之後就會移除。
server-issues-job = 失敗的背景工作
server-issues-job-blurb = 重試會從頭再執行一次工作。
server-issues-file = 無法開啟的檔案
server-issues-file-blurb = Hoard 無法開啟或儲存這些檔案。監看素材包會在下次掃描時再試一次；已在素材庫中的檔案則保留原本的內容。
server-issues-unpack = 無法解壓縮的壓縮檔
server-issues-unpack-blurb = 每個壓縮檔都以一般檔案的形式保留在素材包中，因此其中的內容都不在素材庫裡。
server-issues-read = Hoard 無法解讀的檔案
server-issues-read-blurb = 這些檔案在素材庫中，但沒有讀取後才會有的資訊、標籤或縮圖。
server-issues-render = 縮圖算繪器處理失敗的模型
server-issues-render-blurb = 這些模型沒有縮圖。在檢視器中開啟仍可能成功，並且可以儲存縮圖。
server-issues-convert = Blender 無法轉換的模型
server-issues-convert-blurb = 這些模型沒有 GLB 副本，因此無法以 3D 顯示。
server-issues-unreal = 無法轉換的 Unreal 套件
server-issues-unreal-blurb = 素材包的 Unreal 頁面中有其餘的套件。
server-issues-reformat = 轉換規則無法轉換的檔案
server-issues-reformat-blurb = 這些檔案維持原樣。
server-issues-materials = 無法判斷材質的素材包
server-issues-materials-blurb = 貼圖都在，只是沒有分組成材質。
server-issues-material-thumb = 沒有縮圖的材質
server-issues-material-thumb-blurb = 已找到材質，但無法繪製它的預覽。
server-issues-store-files = 無法讀取的商店檔案
server-issues-store-files-blurb = 來自素材包所屬商店的檔案（例如 Megascans JSON），用來為素材包命名與加上標籤。
server-issues-scan-again = 重新掃描
server-issues-retry = 重試
server-issues-reanalyze = 重新分析素材包
server-issues-convert-again = 重新轉換
server-issues-look-again = 重新查詢
server-issues-read-again = 重新讀取
server-issues-blob-missing = 儲存的檔案已不存在。
server-issues-blob-damaged = 儲存的檔案與當初儲存的內容不符：檔案已損毀。
server-issues-folder-gone = 找不到資料夾「{ $folder }」。
server-job-a-pack = 一個素材包
server-job-ingest = 正在匯入 { $pack }
server-job-retag = 正在重新為 { $pack } 加上標籤
server-job-llm-tag = 正在用 AI 為 { $pack } 加上標籤
server-job-convert = 正在用 Blender 轉換 { $pack }
server-job-unreal = 正在轉換 { $pack } 中的 Unreal 套件
server-job-reformat = 正在依轉換規則轉換 { $pack } 中的檔案
server-job-scan = 正在掃描監看資料夾
server-job-relocate = 正在素材庫位置之間移動檔案
server-job-looks = 正在分析顏色與外觀
server-job-sounds = 正在聆聽音效
server-job-embed = 正在學習圖片的內容
server-job-hear = 正在學習音效聽起來像什麼
server-job-materials = 正在尋找 { $pack } 中的材質
server-job-read-files = 正在讀取 { $pack } 的檔案
server-job-store-lookup = 正在查詢商店頁面
server-maintenance-backup = 備份資料庫
server-maintenance-backup-blurb = 將資料庫（素材包、標籤、筆記、收藏集與設定）的一致副本存放在資料庫旁的 backups 資料夾中。素材庫的檔案不在其中：這些檔案儲存後就不會再變更，因此任何檔案同步工具都能備份它們。
server-maintenance-scrub = 檢查檔案是否損毀
server-maintenance-scrub-blurb = 讀回每個儲存的檔案並與其指紋比對，找出在磁碟或 NAS 上已損壞或遺失的檔案。素材庫很大時需要一段時間。
server-maintenance-reanalyze = 重新分析所有素材包
server-maintenance-reanalyze-blurb = 用這個版本的 Hoard 重新讀取每個檔案：重新產生縮圖、預覽、詳細資訊與規則標籤。你自己的編輯會保留；不會再詢問 AI。
server-maintenance-clear-cache = 清除快取的轉換檔案
server-maintenance-clear-cache-blurb = 刪除「下載為」轉換時保留的檔案（下次需要時會重新產生），以及中斷的工作留下的檔案。
server-maintenance-shrink-thumbs = 縮小縮圖
server-maintenance-shrink-thumbs-blurb = 依照「設定 → 效能 → 縮圖」的設定，重新儲存以 PNG 保存的縮圖與模型視圖，通常可以縮小好幾倍。有透明度的圖片與像素畫會維持 PNG。
server-maintenance-gc = 刪除未使用的檔案
server-maintenance-gc-blurb = 刪除已沒有任何項目指向的儲存檔案，例如已永久刪除的素材包的檔案。
server-update-no-address = 未設定要呼叫的網址
server-update-no-answer = 30 秒內沒有回應

## 設定：外觀

appearance-language = 語言
appearance-language-auto = 瀏覽器的語言（{ $name }）

## 根頁面

root-unreachable = 無法連線到 Hoard 伺服器：{ $error }
root-unreachable-desktop = Hoard 沒有回應：{ $error }
root-desktop-reopen = 此視窗已登出。請關閉 Hoard 並重新開啟以繼續。
root-try-again = 再試一次
root-lazy-connect = 應用程式連線
window-minimize = 最小化
window-maximize = 最大化
window-restore = 還原
window-close = 關閉

## 應用程式：素材庫、頂端列與篩選條件

app-lazy-settings = 設定
app-lazy-archive = 封存區
app-lazy-shortcuts = 快速鍵
app-lazy-setup = 設定精靈
app-theme-system = 配合系統主題
app-theme-light = 淺色主題
app-theme-dark = 深色主題
app-color-red = 紅色
app-color-orange = 橙色
app-color-yellow = 黃色
app-color-green = 綠色
app-color-teal = 藍綠色
app-color-blue = 藍色
app-color-purple = 紫色
app-color-pink = 粉紅色
app-color-brown = 棕色
app-color-sand = 沙色
app-color-gray = 灰色
app-color-slate = 石板灰
app-color-black = 黑色
app-color-white = 白色
app-color-swatch = { $color }：含有大量{ $lower }的素材
app-color-any = 任何顏色
app-color-pick = 挑選任意顏色
app-color = 顏色
app-music = 音樂
app-tempo = BPM
app-tempo-min = 從
app-tempo-max = 到
app-tempo-min-title = 此速度或更快的音樂，單位為每分鐘拍數（減半或加倍也算）
app-tempo-max-title = 此速度或更慢的音樂，單位為每分鐘拍數
app-key = 調性
app-key-any = 任何調性
app-key-major = 大調
app-key-minor = 小調
app-key-mix = 以及可與之混音的調
app-key-mix-title = 它的關係大調或小調，以及上下相差五度的調，DJ 會將它們混在一起
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = 照拼寫方式寫出聲音（例如 whooom）：形狀相近的聲音排在前面
app-imitation-record = 對著麥克風發出聲音：形狀相近的聲音排在前面
app-imitation-stop = 停止錄音
app-imitation-heard = 你的聲音
app-imitation-mic-failed = 無法從麥克風錄音：{ $error }
app-retry = 重試
app-undo = 復原
app-search-failed = 搜尋失敗：{ $error }
app-unreachable = 無法連線到伺服器：{ $error }
app-unreachable-desktop = Hoard 沒有回應：{ $error }
app-forget-project-title = 要忘記 { $name } 嗎？
app-forget-project-body = Hoard 將不再於使用它的 { $count } 個素材上列出這個專案，直到下次 hoard sync 再次回報為止。專案本身不會有任何變更。
app-forget-project-confirm = 忘記
app-forget-project-failed = 無法忘記 { $name }：{ $error }
app-forget-project-label = 忘記專案 { $name }
app-forget-project = 忘記這個專案
app-save-name-like = 類似 { $name }
app-save-name-favorites = 最愛
app-search-saved = 已儲存「{ $name }」：位於「篩選條件」的「我的」底下
app-rank-alike = 最相似
app-rank-color = 顏色最多
app-rank-match = 最符合
app-rank-imitation = 聽起來最像
app-sort = 排序
app-sort-newest = 最新
app-sort-name = 名稱
app-sort-largest = 最大
app-sort-oldest = 最舊
app-collection-added = 已將 { $count } 個素材加入 { $name }
app-collection-add-failed = 無法加入收藏集：{ $error }
app-collection-removed = 已從 { $name } 移除 { $count } 個
app-collection-remove-failed = 無法從收藏集移除：{ $error }
app-ns-subject = 主題
app-ns-style = 風格
app-ns-theme = 題材
app-ns-material = 材質
app-ns-map = PBR 貼圖
app-ns-color = 顏色
app-ns-mood = 氛圍
app-ns-genre = 類型
app-ns-license = 授權
app-ns-res = 解析度
app-ns-duration = 長度
app-ns-poly = 多邊形數
app-ns-format = 格式
app-ns-weight = 大小
app-ns-tech = 技術
app-ns-blender = Blender 版本
app-ns-depth = 位元深度
app-ns-ucs = 音效分類（UCS）
app-ns-bpm = 節奏
app-ns-root = 根音
app-ns-level = 響度
app-ns-ue = Unreal 類別
app-ns-engine = 引擎
app-ns-extracted = 復原的來源檔
app-ns-script = 文字系統
app-bulk-failed =
    { $count ->
       *[other] 無法變更這 { $count } 個素材：{ $error }
    }
app-bulk-untagged = 已從 { $count } 個素材移除 { $tag }
app-bulk-tagged = 已為 { $count } 個素材加上 { $tag }
app-bulk-category-set = 已將 { $count } 個素材的類別設為 { $category }
app-bulk-category-cleared = 已清除 { $count } 個素材的類別
app-credits-failed = 無法製作致謝名單：{ $error }
app-archived-assets = 已封存 { $count } 個素材
app-archived-one = 已封存 { $name }
app-open-failed = 無法開啟該素材：{ $error }
app-unnamed-pack = 素材包
app-unnamed-collection = 收藏集
app-unnamed-asset = 素材
app-home-version = Hoard { $version } — 返回素材庫
app-home = 返回素材庫
app-search-label = 搜尋素材
app-search-placeholder = 搜尋名稱、路徑、標籤
app-views = 檢視
app-library = 素材庫
app-packs = 素材包
app-packs-working = { $count } 個處理中
app-update-title = Hoard { $version } 已推出（這台伺服器執行的是 { $current }）。請拉取新的映像檔或執行檔並重新啟動；資料庫會自動升級。
app-update = 更新
app-theme = 主題
app-sign-in = 登入
app-archive-title = 封存區：{ $contents }
app-archive = 封存區
app-settings = 設定
app-palette-title = 命令面板（{ $keys }）
app-import-folder = 匯入資料夾
app-add-files = 新增檔案
app-add-files-short = 新增
app-split-label = 如何匯入這個資料夾
app-split-before = { "" }
app-split-after = { " " }包含 { $count } 個資料夾或壓縮檔。
app-split-each = 各自匯入為獨立的素材包（{ $count }）
app-split-one = 匯入為一個素材包
app-cancel = 取消
app-filters = 篩選條件
app-done = 完成
app-yours = 我的
app-favorites = 最愛
app-forget-search-label = 忘記已儲存的搜尋 { $name }
app-forget-search = 忘記這個搜尋
app-projects-title = 從這裡執行 hoard sync 的專案，以及它們上次同步的內容
app-projects = 專案
app-project-synced = { $when }同步
app-project-synced-by = { $who } { $when }同步
app-collections = 收藏集
app-no-filters = 這個搜尋沒有篩選條件。
app-kind = 類型
app-facets-more = 還有 { $count } 個
app-facets-fewer = 收合
app-searching = 搜尋中…
app-searching-live = 搜尋中
app-asset-count = { $count } 個素材
app-clear-all = 全部清除
app-save-name-label = 這個搜尋的名稱
app-save-name-placeholder = 命名
app-save = 儲存
app-save-search-title = 將這個搜尋保留在「篩選條件」的「我的」底下
app-save-search = 儲存搜尋
app-select-title = 挑選多個素材，一起加上標籤、分類、收藏、下載或封存（X 或 Ctrl+按一下）
app-select = 選取
app-tile-size = 圖塊大小
app-supporting-title = glTF 緩衝區、材質庫、材質的貼圖、只有一個模型使用的材質，以及描述模型的商店檔案。下載使用它們的素材時會一併附上。 精靈圖集的頁面，以及套件中也以單獨圖片存在的精靈，會與圖集一起顯示。 套件中有多種格式的資源只顯示一次，使用 Hoard 讀取得最好的格式。
app-supporting-hidden = 已隱藏 { $count } 個輔助檔案
app-supporting-show = 顯示
app-supporting-shown = 包含輔助檔案
app-supporting-hide = 隱藏
app-folders = 資料夾
app-folders-title = 依磁碟上的原樣逐一瀏覽此套件的資料夾
app-folders-label = 套件的資料夾
app-welcome-title = 將素材包拖放到這個頁面的任何地方
app-welcome-before = Zip、tarball、資料夾或零散的檔案都可以。Hoard 會解壓縮，依雜湊值將每個檔案只儲存一次，讀取大小、長度與多邊形數，並為所有內容加上標籤，讓你可以搜尋例如{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }或{ " " }
app-welcome-after = { " " }這類的內容。
app-welcome-import-folder = 匯入資料夾
app-welcome-choose-files = 選擇檔案
app-none-similar = 目前還沒有和 { $name } 很相似的東西。
app-none = 沒有符合的項目。
app-none-query = 沒有符合「{ $query }」的項目。
app-none-imitation = 還沒有已測量的聲音聽起來像「{ $word }」。
app-none-heard = 還沒有已測量的聲音聽起來像你的聲音。
app-none-filters = 沒有符合這些篩選條件的項目。
app-none-query-filters = 沒有符合「{ $query }」及這些篩選條件的項目。
app-clear-search-filters = 清除搜尋與篩選條件
app-loading-more = 正在載入更多
app-keys-hint = 方向鍵移動、Enter 開啟、空白鍵播放音效、X 或 Ctrl+按一下可選取多個（觸控裝置請長按）。
app-resize-label = 調整詳細資訊面板大小
app-resize-title = 拖曳以調整大小 · 按兩下以符合內容寬度
app-selected-label = 已選取的素材
app-selected = 已選取 { $count } 個
app-pick-assets = 挑選素材
app-download-zip = 下載 zip
app-credits-title = 這些素材所屬素材包的 CREDITS.md：作者、授權，以及他們要求的致謝文字
app-credits = 致謝名單
app-prefer-sources-title = Hoard 已復原原始檔案的 Unreal SoundWave 與貼圖
app-prefer-sources = 使用復原的 WAV/PNG 而非 .uasset（{ $count }）
bag-clear = 清除選取
bag-glb-heading = 模型轉為 GLB
bag-glb-text = 每個模型都轉為 .glb 檔案，Y 軸朝上、面向前方，材質貼圖以獨立檔案放在模型共用的 textures 資料夾裡。其他檔案原樣放入。
bag-glb-download = 下載 GLB zip
bag-plain-heading = 檔案保持原樣
bag-plain-text = 每個檔案保持匯入時的樣子，連同所需的檔案，依素材包各放一個資料夾。
bag-remove = 將 { $name } 移出選取
bag-more = 還有 { $count } 個
app-bulk-tag-label = 要加到所選素材或從中移除的標籤
app-bulk-tag-placeholder = 標籤
app-bulk-tag-add-title = 為每個選取的素材加上這個標籤
app-bulk-tag-add = 加上
app-bulk-tag-remove-title = 從每個選取的素材移除這個標籤
app-bulk-tag-remove = 移除
app-bulk-category-label = 所選素材的類別（留空即清除）
app-bulk-category-placeholder = 類別
app-bulk-category-set-title = 設定每個選取素材的類別
app-bulk-category-clear-title = 清除每個選取素材的類別
app-bulk-category-set-button = 設定
app-bulk-category-clear-button = 清除
app-remove-from-collection = 從 { $name } 移除
app-unfavorite = 取消最愛
app-favorite = 加入最愛
app-select-all = 全選 { $count } 個
app-archive-selected-title = 從搜尋中移除，直到從封存區還原或永久刪除為止
app-drop-items = 放開以新增 { $count } 個項目
app-drop = 放開以加入 Hoard
app-drop-hint = 每個最上層的檔案或資料夾都會成為獨立的素材包。數 GB 也沒問題；連線中斷時，大型檔案會接續上傳。

## 命令面板

palette-group-go-to = 前往
palette-group-actions = 動作
palette-group-theme = 主題
palette-group-kinds = 類型
palette-group-packs = 素材包
palette-group-collections = 收藏集
palette-group-projects = 專案
palette-group-saved = 已儲存的搜尋
palette-group-assets = 素材
palette-hint-current = 目前
palette-library = 素材庫
palette-library-keywords = 素材 格線 首頁 assets grid home
palette-favorites = 最愛
palette-favorites-keywords = 星號 收藏 喜歡 starred stars liked
palette-packs = 素材包
palette-archive = 封存區
palette-archive-keywords = 垃圾桶 資源回收筒 已刪除 還原 trash recycle bin deleted restore
palette-stats = 素材庫統計
palette-stats-keywords = 統計 數字 大小 磁碟空間 數量 statistics size disk space count
palette-copies = 近似副本
palette-copies-keywords = 重複 相同 相似 縮放 轉換 浪費 空間 duplicates same similar resized converted wasted space dedupe
palette-search = 搜尋
palette-add-files = 新增檔案…
palette-add-files-keywords = 上傳 匯入 upload import
palette-import-folder = 匯入資料夾…
palette-import-folder-keywords = 上傳 素材包 upload pack
palette-select-all = 全選顯示的 { $count } 個
palette-select-all-keywords = 勾選 check
palette-select-several = 選取多個…
palette-select-several-keywords = 大量 批次 多個 標籤 封存 bulk multiple tag archive
palette-archive-selected = 封存選取的 { $count } 個
palette-archive-selected-keywords = 刪除 移除 垃圾桶 delete remove trash
palette-clear-selection = 清除選取
palette-clear-filters = 清除篩選條件
palette-clear-filters-keywords = 重設 reset
palette-hide-supporting = 隱藏輔助檔案
palette-show-supporting = 顯示輔助檔案
palette-supporting-keywords = 法線貼圖 粗糙度 遮罩 normal maps roughness masks
palette-account = 帳號與存取權杖
palette-account-keywords = 密碼 登入 使用者 權杖 password login user token api
palette-sign-out = 登出（{ $name }）
palette-sign-out-keywords = 登出 log out logout
palette-sign-in = 登入
palette-sign-in-keywords = 登入 帳號 密碼 log in login account password
palette-settings = 設定
palette-settings-keywords = blender ffmpeg llm ai 標記 unreal 管理員 工具 偏好設定 tagging admin tools preferences
palette-shortcuts = 鍵盤快速鍵
palette-shortcuts-keywords = 說明 按鍵 help keys
palette-theme-keywords = 主題 外觀 模式 theme appearance mode
palette-theme-variant-keywords = 外觀 顏色 appearance color
palette-light-theme = 淺色主題：{ $name }
palette-dark-theme = 深色主題：{ $name }
palette-kind-all = 所有類型（不再只顯示{ $kind }）
palette-kind-only = 只顯示{ $kind }
palette-title = 命令面板
palette-placeholder = 搜尋素材、前往素材包、執行命令…
palette-results = 結果
palette-searching = 搜尋中…
palette-no-match = 沒有符合「{ $query }」的項目
palette-foot-move = 移動
palette-foot-run = 執行
palette-foot-shortcuts = 所有快速鍵

## 鍵盤快速鍵

shortcuts-title = 鍵盤快速鍵
shortcuts-close = 關閉
shortcuts-anywhere = 任何地方
shortcuts-palette = 命令面板：搜尋、跳轉、執行
shortcuts-search = 搜尋
shortcuts-this-list = 這份清單
shortcuts-escape = 關閉、清除選取，或關閉詳細資訊
shortcuts-library = 素材庫
shortcuts-move = 在圖塊之間移動（或 J 和 K）
shortcuts-open = 開啟詳細資訊
shortcuts-play-sound = 播放或停止音效
shortcuts-select = 選取或取消選取
shortcuts-favorite = 加入或移出最愛
shortcuts-key-click = 按一下
shortcuts-select-range = 選取範圍
shortcuts-select-all = 選取所有顯示的項目
shortcuts-sound-player = 音效播放器
shortcuts-play-pause = 播放或暫停
shortcuts-skip = 倒轉或快轉
shortcuts-start-end = 開頭或結尾
shortcuts-image-inspector = 圖片檢查器
shortcuts-zoom = 放大或縮小
shortcuts-fit = 符合視窗大小
shortcuts-actual-pixels = 實際像素
shortcuts-tile = 以 3×3 拼貼檢查接縫
shortcuts-background = 下一個背景
shortcuts-channel = 只顯示一個色版

## 素材類型

kinds-group = 素材類型

## 標籤

tag-title-with-source = { $tag }（{ $source }）
tag-source-human = 由使用者加上
tag-source-rule = 來自離線標記器
tag-source-meta = 從檔案量測
tag-source-ai = AI 建議
tag-remove = 移除標籤 { $tag }

## 背景活動

activity-label-idle = 背景活動：閒置
activity-label-running = 背景活動：{ $running } 個執行中
activity-label-queued = 背景活動：{ $running } 個執行中、{ $queued } 個排隊中
activity-title = 背景活動
activity-unreachable = 無法連線到伺服器
activity-unreachable-desktop = Hoard 沒有回應
activity-empty = 目前沒有執行中的工作。匯入、標記、轉換與 zip 下載在進行時會顯示在這裡。
activity-queued = 排隊中
activity-seconds = { $s } 秒
activity-minutes = { $min } 分鐘
activity-hours = { $h } 小時 { $min } 分鐘
activity-percent = { $pct }%

## 素材圖塊

tile-triangles = { $count } 個三角形
tile-label = { $name }，{ $category }
tile-label-detail = { $name }，{ $category }，{ $detail }
tile-play = 播放 { $name }
tile-pause = 暫停 { $name }
tile-favorite = 將 { $name } 加入最愛
tile-in-favorites = 已在你的最愛中
tile-add-favorite = 加入你的最愛（F）
tile-more-like = 更多類似 { $name } 的項目
tile-more-like-this = 更多類似項目
tile-select = 選取 { $name }

## 收藏集選擇器

collections-add = 加入收藏集
collections-menu = 收藏集
collections-find-label = 尋找或建立收藏集
collections-find-placeholder = 尋找或為新收藏集命名
collections-create = 建立
collections-empty = 還沒有收藏集。輸入名稱即可建立一個。

## 資料夾選擇器

folders-list = 資料夾
folders-close = 關閉
folders-browse = 瀏覽…
folders-up-title = 上一層資料夾
folders-up = 上一層
folders-this-computer = 這台電腦
folders-none = 這裡沒有資料夾。

## 位置表單

locations-mode-question = Hoard 應該如何處理這個資料夾？
locations-mode-managed = 占為己有：Hoard 把我的素材包存放在這裡
locations-mode-managed-hint = 你上傳或匯入的素材包會複製到這裡，由 Hoard 整理妥當。這個資料夾現在歸 Hoard 所有，請不要變動其中的內容。
locations-mode-watched = 看守：我的素材包已經在這裡
locations-mode-watched-hint = 其中的每個資料夾都會成為一個素材包。Hoard 會直接讀取原位置的檔案，絕不會變更、移動或刪除它們，並會在新資料夾出現時自動加入。
locations-folder-desktop = 資料夾
locations-folder-placeholder = 存放素材包的資料夾
locations-folder-hint-managed = 如果資料夾不存在，Hoard 會建立它。在 Docker 中，這是你掛載到容器內的資料夾。
locations-folder-hint-managed-desktop = 如果資料夾不存在，Hoard 會建立它。
locations-folder-hint-watched = 在 Docker 中，請先將資料夾掛載到容器內（唯讀也可以），然後在這裡選擇它。
locations-folder-hint-watched-desktop = Hoard 只會讀取這個資料夾，不會變更其中的任何內容。
locations-name = 名稱
locations-name-placeholder-managed = 素材庫
locations-name-placeholder-watched = 我的素材包
locations-files = 要存放在這裡的檔案
locations-catch-all = 以及其他位置都不接收的所有檔案
locations-kinds-hint = 新素材包的檔案會存放到勾選了該類型的位置，其餘的則存放到全部接收的位置。
locations-files-all = 目前是全部。之後可以從「設定」新增更多位置，把某些類型（例如音樂）存放到另一個磁碟。
locations-first-watched-note = 上傳需要由 Hoard 管理的素材庫。你之後可以從「設定 → 素材庫位置」新增。
locations-checking = 正在檢查資料夾…
locations-add = 新增位置
locations-cancel = 取消

## 位置面板

locations-failed = { $what }：{ $error }
locations-load-failed = 無法載入素材庫位置
locations-takes-and-rest = { $kinds }，以及其他所有檔案
locations-takes-rest = 其他位置不接收的所有檔案
locations-takes-everything = 所有檔案
locations-takes-nothing = 目前沒有
locations-save-failed = 無法儲存位置
locations-unwatch-title = 要停止監看 { $name } 嗎？
locations-unwatch-body = 其中的 { $count } 個素材包會連同標籤與筆記一起從 Hoard 中移除。資料夾中的檔案不會受到影響。
locations-unwatch-body-empty = 資料夾中的檔案不會受到影響。
locations-unwatch = 停止監看
locations-remove-title = 要移除 { $name } 嗎？
locations-remove-body = 其中的檔案會先依類型移到其他位置；清空後就會移除這個位置。不會遺失任何檔案。
locations-remove-body-only = 如果其中有檔案，請先新增另一個位置讓檔案移入。
locations-remove = 移除
locations-removed = 已移除 { $name }
locations-emptying = 正在將檔案移出 { $name }；完成後就會移除
locations-offline-title = 無法連線到 { $name }
locations-offline-body = 在離線狀態下無法移出其中的檔案。仍要移除嗎？仍在其中的檔案將會顯示為遺失。
locations-remove-anyway = 仍要移除
locations-remove-failed = 無法移除 { $name }
locations-rebalancing = 正在將儲存的檔案移到接收該類型的位置
locations-rebalance-failed = 無法移動檔案
locations-scanned-never = 尚未掃描
locations-scanned-now = 剛剛掃描過
locations-scanned-minutes = { $mins } 分鐘前掃描過
locations-scanned-hours = { $hours } 小時前掃描過
locations-scanned-date = { $date } 掃描過
locations-scan-failed = 無法掃描 { $name }
locations-watching = 正在監看 { $name }
locations-added = 已新增 { $name }
locations-loading = 載入中…
locations-managed-title = Hoard 管理的素材庫
locations-watched-title = 監看資料夾
locations-managed-hint = 上傳與匯入的檔案會複製到這裡。每個位置接收它勾選的類型；全部接收的位置則接收其餘的檔案。
locations-watched-hint = 在原位置建立索引的素材包資料夾。Hoard 絕不會變更其中的檔案，並每 15 分鐘檢查一次是否有新增或移除的資料夾。
locations-managed-empty = 目前還沒有，因此上傳的檔案無處存放。請在下方新增一個。
locations-reachable = 可以連線
locations-unreachable-now = 目前無法連線
locations-catch-all-tag = 全部接收
locations-packs = { $count } 個素材包
locations-unreachable = 無法連線
locations-scanning = 掃描中…
locations-scan = 立即掃描
locations-edit = 編輯
locations-catch-all-locked = 若要變更這項設定，請將另一個位置設為全部接收的位置
locations-edit-hint-before = 只會套用到之後新增的檔案。若要移動已儲存的檔案，請使用下方的
locations-edit-hint-after = 按鈕。
locations-save = 儲存
locations-rebalance = 移動已儲存的檔案以符合設定
locations-rebalance-hint = 變更位置接收的類型後：已儲存的檔案會移到其類型現在對應的位置。
locations-add-title = 新增位置
locations-add-hint = 用來存放某些類型檔案的另一個磁碟，或是你現有的素材包資料夾。
locations-kind-question = 是哪種位置？
locations-kind-folder = 資料夾
locations-kind-folder-hint = 在掛載到容器內的磁碟或 NAS 共用資料夾上，或由 Dropbox、Google Drive、OneDrive 或 iCloud Drive 同步的資料夾。Hoard 可以在那裡存放檔案，也可以關注你已有的資源包。
locations-kind-folder-hint-desktop = 在這台電腦、另一顆磁碟或 NAS 共用資料夾上，或由 Dropbox、Google Drive、OneDrive 或 iCloud Drive 同步的資料夾。Hoard 可以在那裡存放檔案，也可以關注你已有的資源包。
locations-kind-s3-hint = Amazon S3 或類似服務中的儲存貯體：Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage。Hoard 在那裡存放檔案。
locations-s3-title = S3 儲存空間
locations-s3-hint = s3://bucket-name 位置是這個服務中的儲存貯體：AWS，或任何相容 S3 的服務（Backblaze B2、Cloudflare R2、Wasabi、MinIO、NAS 上的 Garage）。
locations-bucket = 儲存貯體
locations-bucket-hint = 一個已經存在的儲存貯體的名稱。上面的金鑰需要能列出、讀取和寫入它。
locations-s3-endpoint = 網址
locations-s3-endpoint-placeholder = AWS 請留空；例如 https://s3.eu-central-003.backblazeb2.com
locations-s3-region = 區域
locations-s3-access-key = 存取金鑰 ID
locations-s3-secret-key = 私密存取金鑰
locations-s3-headers = 額外標頭
locations-s3-headers-placeholder = Name=value，每行一個（例如用於 Cloudflare Access）
locations-s3-kept = 已儲存（輸入以取代）
locations-s3-secrets = 金鑰與標頭會隨伺服器設定一起儲存，且不會再顯示。
locations-s3-secrets-desktop = 金鑰與標頭會儲存在這台電腦的鑰匙圈中，且不會再顯示。
locations-s3-saved = 已儲存 S3 儲存空間
locations-s3-save-failed = 無法儲存 S3 儲存空間
locations-add-more = 新增位置…

## 問題面板

issues-failed = { $what }：{ $error }
issues-load-failed = 無法載入問題
issues-started =
    { $count ->
       *[other] 已重新開始 { $count } 個項目。
    }
issues-retry-failed = 無法重試
issues-dismiss-failed = 無法略過
issues-dismiss-all-title = 要略過全部 { $count } 個嗎？
issues-dismiss-all-body = 「{ $title }」將會清除。之後若再發生錯誤，會再次列出。
issues-dismiss-all = 全部略過
issues-just-now = 剛剛
issues-minutes-ago = { $mins } 分鐘前
issues-hours-ago = { $hours } 小時前
issues-loading = 載入中…
issues-none = 沒有發生任何問題。
issues-action-all = 全部{ $action }
issues-open-asset = 開啟
issues-open-pack = 顯示素材包
issues-open-locations = 素材庫位置
issues-where-before = 位於
issues-where-after = { "" }
issues-times = { $count } 次
issues-starting = 正在開始…
issues-dismiss-named = 略過 { $title }
issues-dismiss = 略過
issues-show-fewer = 顯示較少
issues-show-all = 顯示全部 { $count } 個
issues-not-listed = 另有 { $count } 個未列出。

## 封存

archive-anyway = 仍要封存
archive-failed = 無法封存：{ $error }
archive-undo = 復原
archive-restore-failed = 無法還原：{ $error }
archive-count-packs-and-assets = { $packs } 個素材包和 { $assets } 個素材
archive-count-packs = { $packs } 個素材包
archive-count-assets = { $assets } 個素材
archive-count-nothing = 沒有任何項目

## 封存區

archive-load-failed = 無法載入封存區：{ $error }
archive-warning-with-uses = { $uses }{ $body }
archive-restored = 已還原 { $what }
archive-purged = 已永久刪除 { $what }
archive-purge-failed = 無法刪除：{ $error }
archive-purge-title = 要永久刪除 { $name } 嗎？
archive-purge-pack-body = 其中的 { $count } 個素材與 { $size } 的檔案也會一併刪除，但與其他素材包共用的檔案除外。這個動作無法復原。
archive-purge = 永久刪除
archive-purge-asset-body = 除非有其他素材共用它的檔案，否則該檔案也會被刪除。這個動作無法復原。
archive-empty-title = 要清空封存區嗎？
archive-empty-body = { $what }將會永久刪除，並釋放它們的檔案。這個動作無法復原。
archive-empty = 清空封存區
archive-title = 封存區
archive-intro = 已封存的素材包與素材不會出現在搜尋、數量統計與標籤清單中。你可以還原它們，或永久刪除以釋放空間。
archive-loading = 載入中…
archive-is-empty = 封存區是空的
archive-is-empty-hint = 封存素材包或素材後，它會在這裡等待，直到你還原或永久刪除它。
archive-packs = 素材包
archive-assets = 素材
archive-asset-count = { $count } 個素材
archive-archived = { $when }封存
archive-from = 來自 { $pack }
archive-restore = 還原
archive-show-more = 顯示更多（還有 { $count } 個）

## 我的

projects-favorite-add-failed = 無法加入你的最愛：{ $error }
projects-favorite-remove-failed = 無法從你的最愛移除：{ $error }
projects-save-search-failed = 無法儲存搜尋：{ $error }
projects-forget-search-failed = 無法移除 { $name }：{ $error }

## 專案

projects-list-and = { $items }和 { $last }
projects-list-separator = 、
projects-use = { $name }（{ $files }）
projects-use-more = 另外 { $count } 個
projects-other = 其他 { $count } 個專案
projects-usage-title-one = { $name } 正在使用這個項目
projects-usage-title = 有 { $count } 個專案正在使用這個項目
projects-usage-body-one = 使用於 { $list }：下次執行 hoard sync 時會將它們移除。
projects-usage-body = 使用於 { $list }：各專案下次執行 hoard sync 時會將它們移除。

## 上傳

upload-network-error = 網路錯誤
upload-gave-up = { $what }：嘗試 { $attempts } 次後仍失敗：{ $error }
upload-failed-plain = 失敗
upload-already-imported = 已匯入
upload-part-failed = 上傳失敗（{ $status }）
upload-batch = 一批 { $count } 個檔案
upload-batch-failed = 批次失敗（{ $status }）
upload-finish = 完成上傳

## 上傳佇列

upload-eta-seconds = 還剩 { $s } 秒
upload-eta-minutes = 還剩 { $min } 分鐘
upload-eta-hours = 還剩 { $h } 小時
upload-queue = 匯入
upload-importing = 正在匯入 { $count } 個素材包
upload-bytes-of = { $sent }／{ $total }
upload-rate = { $size }/s
upload-waiting = 等待中
upload-unpacking = 已上傳，正在解壓縮
upload-already = 已在素材庫中
upload-stopped = 已停止
upload-files-of = { $done }／{ $total } 個檔案
upload-resumed = 已接續上傳，已有 { $size } 在伺服器上
upload-skipped = 這些完全相同的檔案已匯入為「{ $name }」，因此已略過。
upload-error = { $error }。你的進度已保留：請重試，或稍後再次拖放相同的檔案。
upload-retry = 重試
upload-import-again = 仍要再次匯入
upload-dismiss = 略過
upload-cancel = 取消
upload-minimize = 最小化
upload-expand = 顯示匯入
upload-pill = 正在匯入 { $count } 個 · { $percent }%
upload-pill-stopped = 已停止：{ $count }
upload-interrupted = 已中斷
upload-resume = 繼續
upload-resume-all = 全部繼續
upload-resume-hint-folder = 再次選擇同一個資料夾，或把它拖放到頁面上，即可從停下的地方繼續。
upload-resume-hint-file = 再次選擇同一個檔案，或把它拖放到頁面上，即可從停下的地方繼續。
upload-not-same = 這些不是相同的檔案，因此沒有繼續任何匯入。
upload-permission-denied = 瀏覽器未獲准再次讀取這些檔案。請改為手動選擇它們。

## 素材包檢視

packs-rename-failed = 無法重新命名：{ $error }
packs-collection-deleted = 已刪除收藏集 { $name }
packs-delete-failed = 無法刪除：{ $error }
packs-status-uploading = 上傳中（目前已收到的檔案）
packs-status-ingesting = 正在解壓縮並加上標籤
packs-status-analyzing = LLM 正在加上標籤
packs-status-ready = 就緒
packs-status-failed = 失敗
packs-retag-failed = 無法開始重新加上標籤：{ $error }
packs-read-files-failed = 無法讀取素材包的檔案：{ $error }
packs-context-load-failed = 無法載入素材包的商店詳細資訊：{ $error }
packs-undo-store-failed = 無法移除商店頁面：{ $error }
packs-context-save-failed = 無法儲存商店詳細資訊：{ $error }
packs-unreal-load-failed = 無法載入 Unreal 轉換狀態：{ $error }
packs-unreal-discard-title = 要捨棄原始檔案嗎？
packs-unreal-discard-body = { $name } 中的原始 .uasset 檔案會在轉換後刪除。其中只有 Unreal 才能使用的資料（例如材質圖）將永久遺失。
packs-unreal-discard-confirm = 轉換並捨棄
packs-unreal-convert-failed = 無法開始轉換：{ $error }
packs-unreal-fact-converted = 已從 Unreal 轉換 { $done } 個
packs-unreal-fact-packages = { $left } 個 Unreal 套件
packs-unreal-fact-both = 已從 Unreal 轉換 { $done } 個，仍有 { $left } 個套件
packs-thumbs-canceled = 已在完成 { $done }／{ $total } 個後取消
packs-thumbs-drew = 已繪製 { $count } 個模型
packs-thumbs-result = { $lead }：已取代 { $improved } 張縮圖。
packs-thumbs-result-failed = { $lead }：已取代 { $improved } 張縮圖，{ $failed } 張無法繪製。
packs-archived = 已封存 { $name } 及其 { $count } 個素材
packs-blobs-bad = 有 { $count } 個檔案未通過上次的完整性檢查。
packs-collections = 收藏集
packs-browse-named = 瀏覽 { $name }
packs-collection-name = 收藏集名稱
packs-save = 儲存
packs-cancel = 取消
packs-assets = { $count } 個素材
packs-updated = { $date } 更新
packs-browse = 瀏覽
packs-collection-download = 下載（.zip）
packs-collection-credits = 致謝名單
packs-collection-credits-title = 這些素材所屬素材包的作者、授權，以及他們要求的致謝文字
packs-rename = 重新命名
packs-delete = 刪除
packs-packs = 素材包
packs-empty = 還沒有素材包。將 zip 或資料夾拖放到這個頁面的任何地方即可新增。
packs-find = 尋找素材包
packs-find-placeholder = 依名稱、商店、授權或標籤尋找素材包
packs-sort = 排序
packs-sort-new = 最新的在前
packs-sort-old = 最舊的在前
packs-sort-name = 依名稱
packs-sort-size = 最大的在前
packs-sort-assets = 素材最多的在前
packs-with-kind = 包含以下類型的素材包
packs-state = 狀態
packs-state-any = 任何狀態
packs-state-working = 處理中
packs-state-failed = 失敗
packs-state-untagged = 尚未描述
packs-state-unlicensed = 未註明授權
packs-store = 商店
packs-store-any = 任何商店
packs-store-from = 來自 { $store }
packs-store-none = 沒有商店頁面
packs-shown-of = { $shown }／{ $total }
packs-clear = 清除
packs-no-match = 沒有符合的素材包。
packs-clear-all = 清除搜尋與篩選條件
packs-pack-name = 素材包名稱
packs-thumbs-drawing = 正在這個瀏覽器中繪製縮圖…
packs-thumbs-drawing-count = 正在這個瀏覽器中繪製縮圖：{ $done }／{ $total }…
packs-thumbs-stopped = 已停止：{ $error }
packs-thumbs-none = 這裡的每個模型都已有以完整貼圖繪製的縮圖。
packs-dismiss = 略過
packs-license = 授權：{ $license }
packs-license-tier = 授權：{ $license }（{ $tier }）
packs-license-not-found = 找不到
packs-from-before = 來自
packs-from-after = { "" }
packs-added = { $date } 新增
pack-panel-label = 關於此素材包
pack-panel-hide = 隱藏素材包詳細資料
pack-panel-kinds = 包含內容
pack-panel-all = 所有素材包
packs-download = 下載素材包（.zip）
packs-store-details = 商店詳細資訊
packs-more-named = { $name } 的更多動作
packs-more = 更多
packs-rename-more = 重新命名…
packs-retag-rules = 重新執行離線標記器
packs-retag-llm = 重新執行 LLM 標記器
packs-make-thumbs = 在這個瀏覽器中製作縮圖
packs-make-thumbs-title = 在背景的 3D 檢視器中逐一開啟每個模型，若其畫面比伺服器產生的更好，就用來當作縮圖
packs-make-thumbs-title-desktop = 在背景的 3D 檢視器中逐一開啟每個模型，若其畫面比 Hoard 產生的更好，就用來當作縮圖
packs-read-files = 重新讀取素材包的檔案
packs-credits = 下載致謝名單
packs-credits-title = CREDITS.md：素材包的作者、授權，以及要求的致謝文字
packs-unreal-open = 轉換 Unreal 內容…
packs-archive = 封存素材包
packs-unreal-intro = Hoard 會自行讀取 Unreal 套件，並將其中的內容儲存為一般檔案：貼圖存為 PNG 或 EXR，音效存為 WAV，材質存為 JSON。轉換後的檔案會取代套件，並保留其標籤、連結與收藏集。
packs-unreal-converted =
    { $count ->
       *[other] 個已轉換
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] 個已轉換，其中 { $kept } 個保留了原始檔案
    }
packs-unreal-pending =
    { $count ->
       *[other] 個尚未檢查
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] 個等待 Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] 個沒有通用格式
    }
packs-unreal-failed =
    { $count ->
       *[other] 個失敗
    }
packs-unreal-reasons = 套件未轉換的原因
packs-unreal-editor-before = 網格與動畫需要 Unreal Editor。請在安裝了 Unreal 的電腦上執行
packs-unreal-editor-command = hoard unreal-worker --editor <UnrealEditor-Cmd 的路徑>
packs-unreal-editor-after = ；它會將它們匯出為 FBX，並將結果上傳到這裡。
packs-unreal-discard = 轉換後捨棄原始 .uasset 檔案
packs-unreal-discard-hint = 釋放空間。只有 Unreal 才能使用的資料（例如材質圖）將會遺失。
packs-unreal-drop = 刪除沒有通用格式的套件
packs-unreal-drop-hint = 藍圖、關卡與其他只能在引擎中使用的素材。
packs-unreal-convert = 轉換
packs-context-hint-before = 素材包的來源，以及賣家對它的介紹。LLM 標記器會連同每個檔案一起讀取這些內容，因此像「120 種冰與火的法術」這樣的商店簡介，能幫助它為
packs-context-hint-after = { " " }命名。
packs-store-auto = 自動找到：素材包本身就提到了這個頁面。
packs-store-not-this = 不是這個
packs-lookup-label = 在商店中搜尋
packs-lookup-placeholder = 素材包在商店中的名稱
packs-lookup-searching = 搜尋中…
packs-lookup-search = 搜尋商店
packs-lookup-hide = 隱藏
packs-lookup-clues = 來自素材包的檔案：{ $clues }。
packs-lookup-also = 也搜尋了 { $queries }。
packs-lookup-quoted = 「{ $query }」
packs-lookup-nothing = Fab、itch.io、Kenney 或 OpenGameArt 上都沒有看起來相符的項目。請試著減少或更換關鍵字。
packs-lookup-use = 使用
packs-lookup-searched = 於 { $date } 搜尋。
packs-lookup-again = 重新搜尋
packs-lookup-again-after = 。
packs-lookup-unreachable = 這次無法連線到 { $stores }。
packs-lookup-also-look = 也可以到
packs-lookup-links-and = { " " }和{ " " }
packs-lookup-browser-only = 查看（這些商店只能在你的瀏覽器中搜尋）；請將頁面網址貼到下方。
packs-lookup-open = 在商店中尋找…
packs-store-placeholder = Fab、itch.io、Humble Bundle…
packs-seller = 賣家
packs-seller-placeholder = 作者
packs-license-label = 授權
packs-license-placeholder = Fab Standard License、CC BY 4.0…
packs-license-tier-label = 授權等級
packs-license-tier-placeholder = 個人、專業…
packs-store-page = 商店頁面
packs-store-text = 商店的說明
packs-store-text-placeholder = 貼上商品頁面的文字，或留空以從商店頁面擷取。
packs-store-text-fetch = 儲存時會從商店頁面擷取說明。
packs-readme = 在素材包中找到的 README
packs-save-retag = 儲存並重新執行 LLM 標記器

## 素材詳細資訊

asset-lazy-3d-viewer = 3D 檢視器
asset-lazy-text = 文字
asset-lazy-text-viewer = 文字檢視器
asset-lazy-image-viewer = 圖片檢視器
asset-lazy-material-preview = 材質預覽
asset-lazy-material-viewer = 材質檢視器
asset-lazy-sprite-player = 精靈圖播放器
asset-lazy-sheet-view = 精靈圖表檢視器
asset-rel-texture = 貼圖
asset-rel-buffer = 幾何資料
asset-rel-material = 材質檔
asset-rel-animation = 動畫
asset-rel-library = 連結的檔案
asset-rel-guess = 可能搭配使用
asset-rel-sidecar = 商店詳細資訊
asset-rel-variant = 變體
asset-rel-sprite = 精靈
asset-rel-sample = 取樣
asset-role-map = { $role }貼圖
asset-maps-count = { $count } 張貼圖
asset-needs = 需要
asset-animations = 動畫
asset-sprites = 精靈
asset-used-by = 被使用於
asset-loose-sprites = 也有單獨的檔案
asset-in-sheet = 所在的精靈圖集
asset-other-formats = 其他格式
asset-shown-as = 在資源庫中顯示為
asset-conversion-failed = 轉換失敗
asset-as-converting = 轉換中…
asset-as-converting-percent = 轉換中… { $percent }%
asset-as-queued = 排隊中…
asset-download = 下載
asset-as-label = 下載為
asset-as-original = { $format }（原始格式）
asset-as-pack-files = 套件內的檔案
asset-as-size = 大小
asset-as-full-size = 原始大小
asset-as-up-to = 最大 { $size } px
asset-as-hint = 選擇大小或其他格式
asset-meta-size = 大小
asset-meta-length = 長度
asset-meta-sample-rate = 取樣率
asset-meta-channels = 聲道
asset-meta-mono = 單聲道
asset-meta-stereo = 立體聲
asset-meta-bit-depth = 位元深度
asset-meta-bits = { $bits } 位元
asset-meta-level = 音量
asset-meta-level-value = 峰值 { $peak } dBFS，RMS { $rms } dBFS
asset-meta-level-clips = 峰值 { $peak } dBFS，RMS { $rms } dBFS，有削波
asset-meta-loop = 循環
asset-meta-loop-value = { $start } 到 { $end }
asset-meta-playback = 播放
asset-meta-one-shot = 單次播放
asset-meta-envelope = 包絡
asset-meta-envelope-times = 起音 { $attack }，尾音 { $tail }
asset-meta-envelope-hit = 打擊
asset-meta-envelope-swell = 漸強
asset-meta-envelope-sustained = 持續
asset-meta-tone = 音色
asset-meta-tone-value = 亮度 { $khz } kHz，雜訊 { $noise }%
asset-meta-pitch = 音高
asset-meta-pitch-rises = 上升 { $octaves } 個八度
asset-meta-pitch-falls = 下降 { $octaves } 個八度
asset-meta-loops-cleanly = 可無縫循環
asset-meta-tempo = 節奏
asset-meta-tempo-heard = 約 { $bpm } BPM，依音訊測得
asset-meta-tempo-heard-or = 約 { $bpm } BPM（或 { $alt }），依音訊測得
asset-meta-key = 調性
asset-meta-key-value = { $key }（{ $camelot }）
asset-meta-key-heard = { $key }，依音訊測得
music-key-major = { $note } 大調
music-key-minor = { $note } 小調
asset-meta-root-note = 根音
asset-meta-cue-markers = 提示標記
asset-meta-library = 音效庫
asset-meta-keywords = 關鍵字
asset-meta-title = 標題
asset-meta-artist = 演出者
asset-meta-album = 專輯
asset-meta-genre = 類型
asset-meta-generator = 產生器
asset-meta-everything-with = 顯示所有帶有標籤 { $tag } 的內容
asset-meta-saved-with = 儲存工具
asset-meta-color-mode = 色彩模式
asset-meta-layers = 圖層
asset-meta-range = 範圍
asset-meta-hdr = HDR（預覽經過色調映射）
asset-meta-format = 格式
asset-meta-large-document = 大型文件（PSB）
asset-meta-ai = AI
asset-meta-ai-made = 以生成式 AI 製作
asset-meta-ai-edited = 以生成式 AI 編輯
asset-meta-ai-how-metadata = 中繼資料
asset-meta-ai-how-generator = 產生器設定
asset-meta-ai-how-iptc = IPTC 數位來源類型
asset-meta-ai-says = { $made }，依據其{ $how }
asset-meta-ai-by-says = { $made }（{ $by }），依據其{ $how }
asset-meta-provenance = 來源出處
asset-meta-c2pa = 具有 C2PA 內容憑證
asset-meta-compression = 壓縮
asset-meta-note = 備註
asset-meta-maps = 貼圖
asset-meta-sizes =
    { $count ->
       *[other] 大小
    }
asset-meta-triangles = 三角形
asset-meta-vertices = 頂點
asset-meta-meshes = 網格
asset-meta-materials = 材質
asset-meta-animations = 動畫
asset-meta-skins = 蒙皮
asset-meta-unreal-asset = Unreal 素材
asset-meta-unreal-saved-with = { $class }，以 UE { $engine } 儲存
asset-meta-package = 套件
asset-meta-cooked = 已烘焙（遊戲建置版本）：預覽與原始檔已移除
asset-meta-tiling = 拼貼
asset-meta-seamless = 兩個軸向都無縫
asset-meta-analysis-error = 無法完整讀取這個檔案：{ $error }
asset-copy-blocked = 無法複製：瀏覽器已封鎖
asset-save-error = 無法儲存：{ $error }
asset-tag-removed = 已移除 { $tag }
asset-undo = 復原
asset-play-error = 無法播放這個檔案：{ $error }
asset-copied-hash = 已複製雜湊值
asset-copied-toml = 已複製 toml
asset-copied-id = 已複製 ID
asset-copied-link = 已複製連結
asset-origin-before = 這個檔案原本是
asset-origin-class-engine = ，是由 UE { $engine } 儲存的 { $class }。
asset-origin-class = ，是一個 { $class }。
asset-origin-engine = ，由 UE { $engine } 儲存。
asset-origin-end = 。
asset-origin-editor = Unreal Editor 於 { $date } 匯出了它。
asset-origin-editor-notes = Unreal Editor 於 { $date } 匯出了它（{ $notes }）。
asset-origin-hoard = Hoard 於 { $date } 從套件中讀出了它。
asset-origin-hoard-notes = Hoard 於 { $date } 從套件中讀出了它（{ $notes }）。
asset-origin-download = 下載原始 .{ $ext }
asset-origin-discarded = 原始套件已在轉換後捨棄。
asset-details = 素材詳細資訊
asset-in-pack = 位於
asset-previous = 上一個素材
asset-next = 下一個素材
asset-favorite = 最愛
asset-favorite-on-title = 已在你的最愛中（按 F 移除）
asset-favorite-off-title = 加入你的最愛（F）
asset-share = 分享
asset-share-title = 分享此素材的連結（開啟的人需先登入 Hoard）
asset-layout-panel = 在側邊面板中顯示
asset-layout-page = 以整頁顯示
asset-layout-page-title = 以整頁顯示：大型預覽與詳細資訊並排
asset-close = 關閉詳細資訊
asset-rename-title = 按一下以重新命名
asset-name = 名稱
asset-views-alt = 從四個方向看的模型
asset-inspect-aria = 以全螢幕檢查 { $name }
asset-inspect-title = 檢查：縮放、拼貼、色版
asset-playback-position = 播放位置
asset-playback-value = { $current }／{ $duration }
asset-pause = 暫停
asset-play = 播放
asset-waveform-drawing = 正在繪製波形…
asset-waveform-skipped = 檔案太大，不顯示波形
asset-play-animation = 以動畫播放
asset-show-views = 顯示標記器看到的四視圖
asset-palette = 圖片中的顏色
asset-swatch-title = { $hex }，佔圖片的 { $percent }%。尋找含有這個顏色的素材
asset-swatch-aria = 尋找含有 { $hex } 的素材，佔這張圖片的 { $percent }%
asset-more-like-this = 更多類似項目
asset-desc-label = 你的描述
asset-desc-placeholder = 這是什麼、你會用在哪裡…
asset-save = 儲存
asset-cancel = 取消
asset-desc-family = 儲存到全部 { $count } 個變體
asset-desc-edit = 編輯描述
asset-desc-write = 撰寫描述
asset-desc-ai = 由 AI 撰寫
asset-desc-file = 來自檔案
asset-desc-store-before = 來自
asset-desc-store-page = { $store } 頁面
asset-desc-store-page-unnamed = 商店頁面
asset-desc-store-after = （整個素材包）
asset-less = 收合
asset-more = 更多
asset-tags = 標籤
asset-tag-add-label = 新增標籤
asset-tag-add-placeholder = 新增標籤，例如 mood:tense
asset-tag-add = 新增
asset-apply-family = 將標籤、類別與描述的變更套用到全部 { $count } 個變體
asset-collections = 收藏集
asset-collection-remove = 從 { $name } 移除
asset-collection-add-another = 加入其他收藏集
asset-collection-add = 加入收藏集
asset-used-in = 使用於
asset-used-in-title = 顯示 { $name } 包含的項目
asset-used-in-synced = { $when }同步
asset-maps = 貼圖
asset-show-more = 再顯示 { $count } 個
asset-materials-heading =
    { $count ->
       *[other] 材質
    }
asset-material-maps-aria = { $name }：貼圖
asset-variants = 變體（{ $count } 個版本）
asset-fit-animations = 適用的動畫 ({ $count })
asset-fit-characters = 適用的角色 ({ $count })
asset-copies = 完全相同的副本
asset-copies-note = 相同的位元組只會儲存一次；這些素材包也附有這個檔案。
asset-near-copies = 近似副本
asset-near-copies-note = 只是尺寸、取樣率或格式不同的同一內容。最佳副本已標出。
asset-near-copies-best = 只是尺寸、取樣率或格式不同的同一內容。這個就是最佳副本。
asset-near-best = 最佳
asset-unreal-converted = 從 Unreal 轉換
asset-rule-converted = 由規則轉換
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = 轉換規則於 { $date } 產生了此 { $format }，來源：
asset-rule-kept-until = 原始檔案保留至 { $date }。
asset-rule-rebuilt = FLAC 保留了 WAV 中的所有內容，因此隨時可以原樣下載或還原 WAV。
asset-rule-put-back = 還原原始檔案
asset-rule-put-back-title = 要還原 { $file } 嗎？
asset-rule-put-back-body = 轉換後的檔案會被刪除，原始檔案連同其標籤、收藏集與連結回到原位。此後轉換規則不會再處理它。
asset-rule-put-back-done = 已還原 { $file }
asset-rule-let-go = 原始檔案已在「設定」中設定的保留期限到期後刪除。
asset-unreal-package = 來自 Unreal 套件
asset-ue-needs-editor-before = 等待 Unreal Editor：{ $reason }。請在安裝了 Unreal 的電腦上執行
asset-ue-needs-editor-after = ，它就會以 FBX 格式傳回。
asset-ue-editor-path = UnrealEditor-Cmd 的路徑
asset-ue-unsupported = 保留為套件：{ $reason }。
asset-ue-failed = 轉換失敗：{ $reason }
asset-recovered = 原始的 .{ $ext } 存放在這個套件中，Hoard 已將它取出。
asset-recovered-download = 下載 .{ $ext }
asset-ue-meshes-before = 網格與動畫以 Unreal 專屬的格式儲存。請先轉換素材包（素材包頁面的「更多」），然後在安裝了 Unreal Editor 的電腦上執行
asset-ue-meshes-after = 以取得 FBX。
asset-ue-registry-tags = { $count } 個素材登錄標籤
asset-file = 檔案
asset-bytes = 位元組
asset-source = 來源
asset-path = 路徑
asset-path-open-folder = 查看 { $folder } 中的其他內容
asset-download-maps = 下載全部 { $count } 張貼圖
asset-download-linked = 連同 { $count } 個連結的檔案一起下載
asset-download-only-title = 只下載這個檔案，不含它使用的檔案
asset-download-only = 只下載這個檔案
asset-include-animations = 在 .zip 中包含動畫
asset-include-character = 包含角色 { $name }
asset-copy-toml = 複製 hoard.toml 項目
asset-copy-id = 複製 ID
asset-teach = 教 Hoard 讀取這個檔案
asset-teach-title = 描述這個商店的檔案，讓 Hoard 能讀取它（設定 → 商店檔案）
asset-archive = 封存
asset-archive-title = 從搜尋中移除，直到從封存區還原或永久刪除為止

## 圖片檢視器

image-channel-red = 紅色色版
image-channel-green = 綠色色版
image-channel-blue = 藍色色版
image-channel-alpha = Alpha 色版
image-channel-title = { $channel }（{ $key }）
image-bg-checker-title = 棋盤格背景（按 K 切換）
image-bg-black-title = 黑色背景（按 K 切換）
image-bg-gray-title = 灰色背景（按 K 切換）
image-bg-white-title = 白色背景（按 K 切換）
image-bg-checker = 棋盤格背景
image-bg-black = 黑色背景
image-bg-gray = 灰色背景
image-bg-white = 白色背景
image-inspect-aria = 檢查 { $name }
image-preview-size = 預覽 { $width }×{ $height }，{ $bytes }
image-preview = 預覽
image-resolution = 解析度
image-full-title = 載入完整的 { $width }×{ $height } 圖片（F）；放大超過預覽解析度時也會載入
image-full-loading = 正在載入完整解析度…
image-full-failed = 完整解析度載入失敗：重試
image-full = 完整解析度
image-zoom = 縮放
image-zoom-out-title = 縮小（−）
image-zoom-out = 縮小
image-zoom-in-title = 放大（+）
image-zoom-in = 放大
image-fit-title = 符合大小（0）
image-fit = 符合大小
image-actual-title = 實際像素（1）
image-view = 檢視
image-tile-title = 以 3×3 拼貼檢查接縫（T）
image-tile = 拼貼
image-channels = 色版
image-close = 關閉（Esc）
image-failed = 無法載入這張圖片。
image-loading = 載入中…
image-key-gestures = 捲動或雙指縮放 · 拖曳以平移 · 按兩下以放大或符合大小
image-key-tile = 拼貼
image-key-background = 背景
image-key-full = 完整解析度
image-key-channels = 色版

## 精靈圖播放器

sprite-frame-aria = { $name }，第 { $frame }／{ $count } 格
sprite-failed = 無法播放這個檔案。
sprite-loading = 正在載入影格…
sprite-previous = 上一格
sprite-next = 下一格
sprite-pause = 暫停動畫
sprite-play = 播放動畫
sprite-animation = 動畫
sprite-all-frames = 所有影格
sprite-grid-title = 圖表如何切割成影格
sprite-columns = 欄
sprite-rows = 列
sprite-fps-aria = 每秒影格數
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
       *[other] { $count } 個精靈
    }
sheet-pages =
    { $count ->
       *[other] { $count } 頁
    }
sheet-animations =
    { $count ->
       *[other] { $count } 個動畫
    }
# The box that narrows the list of sprites by name.
sheet-find = 尋找精靈
# Under the list of sprites when more match than it draws.
sheet-showing = 顯示 { $shown }／{ $total } 個
sheet-no-match = 沒有名稱包含此內容的精靈。
sheet-failed = 無法讀取這個圖表。
sheet-loading = 正在載入圖表…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoard 專屬格式：各頁維持原樣，精靈可任意旋轉並裁去透明邊緣，包含動畫和每個影格的時長。
sheet-note-texturepacker = 精靈順時針旋轉並裁去透明邊緣，動畫依名稱保存，可供 PixiJS 和 Phaser 讀取。
sheet-note-clockwise = 精靈順時針旋轉並裁去透明邊緣。這個格式無法保存動畫：影格依編號名稱區分。
sheet-note-libgdx = 精靈逆時針旋轉並裁去透明邊緣。這個格式無法保存動畫：影格依編號名稱區分。
sheet-note-godot-frames = 精靈一律不旋轉，裁去的部分保留為邊距，包含動畫和每個影格的時長。
sheet-note-godot-tileset = 圖塊一律不旋轉也不裁切，依圖塊大小排列在格線上；圖片若已排成格線則維持原樣。
sheet-note-grid = 每個精靈都以完整畫布直立擺放，格子大小一致，適用於任何把圖表切成格線的工具。
sheet-note-frames = 每個精靈都以完整畫布直立擺放，各自存成以其名稱命名的 PNG。

## 文字檢視器

text-lang-config = 設定檔
text-lang-table = 表格
text-lang-code = 程式碼
text-lang-script = 指令碼
text-lang-text = 文字
text-copy-blocked = 無法複製：瀏覽器已封鎖
text-read-aria = 閱讀 { $name }
text-lines = { $count } 行
text-show-as = 顯示方式
text-tree-title = 可摺疊的樹狀結構（T）
text-tree = 樹狀
text-pretty-title = 重新縮排（P）
text-pretty = 美化
text-raw-title = 原始內容（R）
text-raw = 原始
text-document-title = 以文件顯示（D）
text-document = 文件
text-source-title = 原始的 Markdown（R）
text-source = 原始碼
text-page-title = 如同瀏覽器從其所在資料夾開啟時的樣子，與 Hoard 隔離 (P)
text-page = 頁面
text-page-source-title = 頁面的原始寫法 (R)
text-page-aria = { $name } 的頁面檢視
text-page-error = 無法開啟此頁面：{ $error }
text-fold = 摺疊
text-expand-all-title = 展開所有內容
text-expand-all = 全部展開
text-collapse-all-title = 摺疊所有內容
text-collapse-all = 全部摺疊
text-group-text = 文字
text-wrap-title = 長行自動換行（W）
text-wrap = 換行
text-copy-title = 複製文字（C）
text-copied = 已複製
text-copy = 複製
text-download-title = 下載檔案
text-download = 下載
text-close = 關閉（Esc）
text-truncated = 顯示 { $total } 中的前 { $shown }。
text-truncated-json = 顯示 { $total } 中的前 { $shown }（樹狀結構需要完整的檔案）。
text-show-all = 全部顯示
text-json-error = 這個 JSON 無法解析：{ $error }
text-contents-aria = { $name } 的內容
text-loading = 載入中…
text-load-error = 無法載入這個檔案：{ $error }
text-binary = 這是二進位檔案而非文字，因此這裡沒有可閱讀的內容。請下載後用適合的工具開啟。
text-key-tree = 樹狀
text-key-pretty = 美化
text-key-raw = 原始
text-key-document = 文件
text-key-source = 原始碼
text-key-page = 頁面
text-key-wrap = 換行
text-key-copy = 複製
text-key-find = 尋找
text-key-close = 關閉
text-preview-aria = 以全螢幕閱讀 { $name }
text-preview-read = 閱讀

## JSON 樹狀結構

json-items = { $count } 個項目
json-keys = { $count } 個鍵
json-fold = 摺疊
json-unfold = 展開
json-unfold-summary = 展開 { $summary }
json-show-more = 再顯示 { $count } 個
json-left = （共剩 { $count } 個）
json-picker-more = … 還有 { $count } 個

## 3D 模型檢視器

model-viewer-failed = 3D 檢視器未能載入（{ $error }）；重新載入頁面可能會有幫助
model-loading-viewer = 正在載入 3D 檢視器…
model-loading = 正在載入模型…
model-show-failed = 無法顯示這個模型：{ $error }
model-preview-failed = 無法預覽這個模型：{ $error }
model-error-converting = 伺服器仍在轉換這個模型
model-error-converting-desktop = Hoard 仍在轉換這個模型
model-error-empty-collada = 空的 Collada 檔案
model-error-no-loader = 瀏覽器沒有可載入 .{ $ext } 的載入器
model-dialog = 以 3D 檢視 { $name }
model-title-stats = { $triangles } 個三角形 · { $meshes } 個網格 · { $materials } 個材質
model-triangles = { $count } 個三角形
model-group-camera = 攝影機
model-group-shading = 著色
model-group-background = 背景
model-group-animation = 動畫
model-group-view = 檢視
model-group-show = 顯示
model-frame-title = 將模型置中（F）
model-frame = 置中
model-reset-title = 回到起始視角（R）
model-reset = 重設
model-turn-title = 緩慢旋轉（A）
model-turn = 旋轉
model-textures-title = 貼圖（T）
model-textures = 貼圖
model-full-res-title = 完整解析度貼圖（H）：不論多大，都使用原始的貼圖檔案；關閉時使用最大 2048 px 的輕量預覽
model-full-res = 完整解析度
model-light-title = 環境光（E）
model-light = 光源
model-grid-title = 地面格線（G）
model-grid = 格線
model-bg-checker = 棋盤格背景
model-bg-dark = 深色背景
model-bg-gray = 灰色背景
model-bg-light = 淺色背景
model-bg-env = 環境背景
model-bg-title = { $background }（按 K 切換）
model-pause-title = 暫停（空白鍵）
model-play-title = 播放（空白鍵）
model-pause = 暫停
model-play = 播放
model-animation-clip = 動畫片段
model-animation = 動畫
model-plays-on = 播放於角色
model-rest-pose = 靜止姿勢
model-close-title = 關閉（Esc）
model-full-screen = 全螢幕
model-open-full-screen = 開啟全螢幕 3D 檢視器
model-menu-fold = 收合選單
model-menu-show = 顯示選單
model-view-prefix = 檢視：
model-menu-render = 算繪
model-menu-maps = 貼圖
model-menu-inspect = 檢查
model-mode-final = 最終算繪
model-mode-raw = 原始算繪
model-mode-color = 基本色
model-mode-roughness = 粗糙度
model-mode-metalness = 金屬度
model-mode-normal = 法線
model-mode-ao = 遮蔽
model-mode-matcap = Matcap
model-mode-uv = UV 棋盤格
model-mode-geometry = 幾何體
model-mode-title = { $mode }（{ $key }）
model-wireframe-title = 線框（W），可疊加在任何檢視上
model-wireframe = 線框
model-info-title = 場景資訊（I）
model-info = 場景資訊
model-info-close = 關閉場景資訊
model-info-close-title = 關閉（I）
model-info-geometries = 幾何體
model-info-materials = 材質
model-info-textures = 貼圖
model-info-triangles = 三角形
model-info-vertices = 頂點
model-info-meters = glTF 以公尺為單位
model-info-no-units = 這個格式沒有說明使用的單位
model-info-height = 高度
model-info-length = 長度
model-info-width = 寬度
model-length-units = { NUMBER($n, maximumSignificantDigits: 3) } 單位
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = 找不到貼圖：{ $name }
model-missing-some = 找不到 { $count } 張貼圖：{ $names }
model-missing-more = 找不到 { $count } 張貼圖：{ $names }…
model-missing-count = 找不到 { $count } 張貼圖
model-guessed = 貼圖是依名稱比對的；檔案本身沒有指定任何貼圖。
model-group-facing = 朝向
model-facing-turn = 旋轉 90°
model-facing-turn-title = 從上方看，將模型逆時針旋轉 90°
model-facing-tip = 前傾 90°
model-facing-tip-title = 將模型頂部向前傾倒 90°，讓躺著的模型站起來
model-facing-save = 套用至此素材包
model-facing-save-title = 將 { $pack } 中的所有模型都這樣旋轉。它們的縮圖會重新繪製；下載檔案保持產生時的樣子。
model-facing-undo = 復原
model-facing-undo-title = 回到素材包中模型原本的朝向
model-facing-saved = { $pack } 中的所有模型現在都是這個朝向。正在重新繪製它們的縮圖。
model-facing-failed = 無法旋轉素材包的模型：{ $error }
model-keys-mouse = 拖曳以環繞 · 右鍵拖曳以平移 · 捲動以縮放 · 按兩下以置中
model-key-frame = 置中
model-key-reset = 重設
model-key-view = 檢視
model-key-wireframe = 線框
model-key-info = 場景資訊
model-key-textures = 貼圖
model-key-full-res = 完整解析度貼圖
model-key-light = 光源
model-key-grid = 格線
model-key-background = 背景
model-key-play = 播放

## 材質

material-role-base-color = 顏色
material-role-normal = 法線
material-role-normal-dx = 法線（DirectX）
material-role-roughness = 粗糙度
material-role-gloss = 光澤度
material-role-metalness = 金屬度
material-role-ao = 環境光遮蔽
material-role-orm = 遮蔽、粗糙度、金屬度
material-role-mra = 金屬度、粗糙度、遮蔽
material-role-rma = 粗糙度、金屬度、遮蔽
material-role-height = 高度
material-role-bump = 凹凸
material-role-cavity = 凹洞
material-role-specular = 鏡面反射
material-role-opacity = 不透明度
material-role-emissive = 自發光
material-role-translucency = 半透明
material-role-base-color-name = 顏色
material-role-normal-name = 法線
material-role-normal-dx-name = 法線（DirectX）
material-role-roughness-name = 粗糙度
material-role-gloss-name = 光澤度
material-role-metalness-name = 金屬度
material-role-ao-name = 環境光遮蔽
material-role-orm-name = 遮蔽、粗糙度、金屬度
material-role-mra-name = 金屬度、粗糙度、遮蔽
material-role-rma-name = 粗糙度、金屬度、遮蔽
material-role-height-name = 高度
material-role-bump-name = 凹凸
material-role-cavity-name = 凹洞
material-role-specular-name = 鏡面反射
material-role-opacity-name = 不透明度
material-role-emissive-name = 自發光
material-role-translucency-name = 半透明
material-maps = { $count } 張貼圖
material-dialog = 檢視材質 { $name }
material-up-to = 最大 { $size }
material-repeat-size = 每次重複 { $size }
material-shape-sphere = 球體
material-shape-cube = 立方體
material-shape-plane = 平面
material-shape-title = { $shape }（{ $key }）
material-group-shape = 形狀
material-group-tiling = 拼貼
material-real-title = 實際大小：每次重複為 { $size }，球體直徑 2 m（按 T 切換）
material-repeat-title = 重複 { $n }×（按 T 切換）
material-real = 實際
material-repeat = { $n }×
material-height-title = 高度貼圖推動表面的程度
material-height = 高度
material-height-strength = 高度強度
material-loading = 正在載入材質…
material-show-failed = 無法顯示這個材質：{ $error }
material-preview-failed = 無法預覽這個材質：{ $error }
material-group-maps = 貼圖
material-map-show = 顯示{ $map }貼圖
material-map-hide = 隱藏{ $map }貼圖
material-map-unused = 預覽未使用
material-open-full-screen = 開啟全螢幕材質檢視器
material-keys-mouse = 拖曳以環繞 · 右鍵拖曳以平移 · 捲動以縮放 · 按一下貼圖即可關閉它
material-key-shape = 形狀
material-key-tiling = 拼貼
material-key-turn = 旋轉

## 主題

theme-slate = 石板
theme-paper = 紙張
theme-linen = 亞麻
theme-sage = 鼠尾草
theme-lilac = 丁香
theme-blush = 粉霧
theme-graphite = 石墨
theme-obsidian = 黑曜石
theme-ember = 餘燼
theme-moss = 苔蘚
theme-aubergine = 茄紫
theme-oxblood = 牛血紅
theme-light-slate-blurb = 冷調藍灰
theme-light-paper-blurb = 樸素中性灰
theme-light-linen-blurb = 溫暖奶油色、琥珀色
theme-light-sage-blurb = 柔和綠色
theme-light-lilac-blurb = 淡紫色
theme-light-blush-blurb = 灰玫瑰色
theme-dark-graphite-blurb = 中性灰，無色調
theme-dark-slate-blurb = 藍灰
theme-dark-obsidian-blurb = 近乎全黑，適合 OLED
theme-dark-ember-blurb = 溫暖炭灰、琥珀色
theme-dark-moss-blurb = 森林綠黑
theme-dark-aubergine-blurb = 梅紫黑、淡紫色
theme-dark-oxblood-blurb = 酒紅黑、珊瑚色

## 貼圖命名

texnames-needs-map = 樣式需要包含 { "{" }map{ "}" }，否則材質的每張貼圖都會得到相同的名稱
texnames-placeholders = 樣式可以使用 { "{" }name{ "}" }、{ "{" }map{ "}" } 和 { "{" }size{ "}" }
texnames-bad-characters = 名稱不能包含 < > : " / \ | ? 或 *
texnames-no-suffix = 請為{ $role }貼圖設定字尾
texnames-bad-suffix = { $role }貼圖的字尾不能包含 < > : " / \ | ? 或 *
texnames-slot-no-suffix = 請為「{ $slot }」設定字尾
texnames-slot-bad-suffix = 「{ $slot }」的字尾不能包含 < > : " / \ | ? 或 *
texnames-ending-missing = 每個自訂結尾都需要填寫要尋找的結尾
texnames-ending-bad = 自訂結尾不能包含 < > : " / \ | ? 或 *
texnames-ending-twice = { $ending } 在你的結尾中出現了兩次
texnames-slot-metallic-roughness = 金屬度與粗糙度
texnames-slot-specular-color = 鏡面反射顏色
texnames-slot-specular-glossiness = 鏡面反射與光澤度
texnames-slot-clearcoat = 清漆
texnames-slot-clearcoat-roughness = 清漆粗糙度
texnames-slot-clearcoat-normal = 清漆法線
texnames-slot-sheen-color = 絨光顏色
texnames-slot-sheen-roughness = 絨光粗糙度
texnames-slot-transmission = 透射
texnames-slot-thickness = 厚度
texnames-slot-diffuse-transmission = 漫透射
texnames-slot-diffuse-transmission-color = 漫透射顏色
texnames-slot-iridescence = 虹彩
texnames-slot-iridescence-thickness = 虹彩厚度
texnames-slot-anisotropy = 異向性

## 商店格式

storefiles-field-title = 標題
storefiles-field-title-hint = 商品頁面的確切標題。它會成為素材包的名稱，沒有標題的檔案會被略過。
storefiles-field-seller = 賣家
storefiles-field-seller-hint = 製作或販售者。
storefiles-field-license = 授權
storefiles-field-license-hint = 照檔案中的寫法，例如 CC-BY-4.0。
storefiles-field-url = 商店頁面
storefiles-field-url-hint = 商品頁面的網址，以 http:// 或 https:// 開頭。
storefiles-field-words = 標籤
storefiles-field-words-hint = 用來加上標籤的字詞。以逗號分隔的值會被拆開。
storefiles-field-type = 類型
storefiles-field-type-hint = 它是什麼，例如「3D model」或「surface」。也會成為標籤。
storefiles-field-colors = 顏色
storefiles-field-colors-hint = 它的顏色，以 color: 標籤表示。
storefiles-field-id = 商店 ID
storefiles-field-id-hint = 商店為它指定的 ID：字母、數字、_ 或 -。
storefiles-field-origin = 製作地點
storefiles-field-origin-hint = 製作或掃描的地點，用於描述。
storefiles-field-size = 大小
storefiles-field-size-hint = 以文字描述它有多大，用於描述。
storefiles-field-title-name = 標題
storefiles-field-seller-name = 賣家
storefiles-field-license-name = 授權
storefiles-field-url-name = 商店頁面
storefiles-field-words-name = 標籤
storefiles-field-type-name = 類型
storefiles-field-colors-name = 顏色
storefiles-field-id-name = 商店 ID
storefiles-field-origin-name = 製作地點
storefiles-field-size-name = 大小

## 商店格式面板

storefiles-load-failed = 無法載入商店檔案格式：{ $error }
storefiles-remove-title = 要移除 { $name } 嗎？
storefiles-remove-body = { $file } 會從資料資料夾中刪除。已用它讀取過的素材包會保留讀取到的內容。
storefiles-remove = 移除
storefiles-removed = 已移除 { $name }。
storefiles-remove-failed = 無法移除 { $name }：{ $error }
storefiles-unread = Hoard 無法讀取的檔案
storefiles-loading = 載入中…
storefiles-unread-none = 沒有：素材庫中沒有看起來像未讀取的商店檔案。
storefiles-unread-what = 出現在多個素材包中，或命名方式像商店檔案，但沒有任何格式能讀取的檔案。
storefiles-unread-packs = { $count } 個素材包
storefiles-unread-example = 例如 { $pack }
storefiles-teach = 教 Hoard 讀取這個檔案
storefiles-formats = 格式
storefiles-formats-what = 會優先嘗試你自己的格式，並取代同名的內建格式。
storefiles-dir-before = 這些格式存放在
storefiles-dir-after = 。
storefiles-yours = 我的
storefiles-built-in = 內建
storefiles-edit = 編輯
storefiles-start-from = 以此為起點

## 標籤面板

tags-load-failed = 無法載入標籤：{ $error }
tags-rename-title = 要重新命名 { $tag } 嗎？
tags-merge-title = 要合併 { $count } 個標籤嗎？
tags-rename-body = 它的 { $count } 個素材會改用 { $into }，標記器之後也會使用新名稱。
tags-merge-body = 最多 { $count } 個素材上的 { $tags } 會變成 { $into }。標記器之後會使用 { $into }。
tags-rename = 重新命名
tags-merge = 合併
tags-merged = 現在有 { $count } 個素材加上了 { $into } 標籤
tags-merge-failed = 無法合併：{ $error }
tags-forget-failed = 無法變更：{ $error }
tags-find = 尋找標籤
tags-find-placeholder = 尋找標籤，例如 rock 或 set:
tags-count = { $count } 個標籤
tags-count-more = { $count }+ 個標籤
tags-what = 挑選意思相同的標籤並合併，或挑選一個標籤來重新命名。
tags-new-name = 新名稱
tags-merge-into = 合併為
tags-saved-as-before = 儲存為
tags-saved-as-after = { "" }
tags-cancel = 取消
tags-loading = 載入中…
tags-none = 沒有標籤。
tags-none-containing = 沒有包含「{ $text }」的標籤。
tags-browse-title = 瀏覽加上 { $tag } 標籤的素材
tags-browse = 瀏覽
tags-merged-heading = 已合併的標籤
tags-merged-what = 當標記器產生這些舊名稱之一時，素材會改為加上它被合併到的標籤。
tags-allow-old = 再次允許使用舊名稱

## 素材庫統計

stats-source-human = 由使用者加上
stats-source-ai = AI 建議
stats-source-rule = 內建規則
stats-source-meta = 來自檔案
stats-assets = 素材
stats-packs = 素材包
stats-on-disk = 磁碟用量
stats-duplicates-title = 有 { $count } 個檔案與其他檔案的位元組完全相同，只儲存一次
stats-saved-dedup = 重複資料刪除節省的空間
stats-distinct-tags = 不同的標籤
stats-archive = 在封存區中
stats-load-failed = 無法載入詳細資訊：{ $error }
stats-counting = 計算中…
stats-as-of = 截至 { $time }
stats-updating = 正在更新…
stats-by-kind = 各類型使用的空間
stats-files-size = { $name }：{ $count } 個檔案，{ $size }
stats-months = 過去一年新增的
stats-month = { $month }：{ $count } 個素材，{ $size }
stats-tagging = 標記
stats-tagged = 已加上標籤
stats-untagged = { $count } 個沒有標籤
stats-described = 已描述
stats-described-assets = { $count } 個素材
stats-human-edited = 手動編輯
stats-human-edited-unit =
    { $count ->
       *[other] 個素材
    }
stats-source-different = { $count } 個不同的
stats-most-used = 最常使用
stats-show-tag = 顯示 { $tag } 素材
stats-file-types = 檔案類型
stats-biggest-packs = 最大的素材包
stats-browse-pack = 瀏覽 { $name }
stats-largest-files = 最大的檔案
stats-open-file = 開啟 { $name }
stats-empty = 素材庫中還沒有任何內容。新增素材包後，它的數據就會顯示在這裡。

## HoardPanel.vue

hoard-title = 你的寶藏
hoard-what = 素材庫中的每個素材都是一件寶物，素材的種類決定是哪種寶物。收集新內容，把它們加進你的寶堆。
hoard-sign-in = 登入後即可收集屬於你自己的寶藏。
hoard-new = 可收集的新戰利品：{ $count }
hoard-none-new = 沒有可收集的新內容。
hoard-collect = 收集
hoard-unavailable = 此瀏覽器無法繪製寶藏：{ $error }
hoard-collect-failed = 無法記錄你收集的內容：{ $error }
hoard-arcade = 街機螢幕
hoard-full-screen = 全螢幕
hoard-leave-full-screen = 結束全螢幕
hoard-label = 一條守護寶藏的龍。寶物：{ $count }
hoard-gold = 金幣
hoard-gems = 寶石
hoard-books = 書籍
hoard-weapons = 武器
hoard-bones = 骨頭
hoard-tablets = 符文石板
hoard-paintings = 畫作
hoard-potions = 藥水

## 更新面板

updates-saved = 有可用的更新時，Hoard 會呼叫這個網址
updates-saved-none = 沒有要呼叫的網址
updates-save-failed = 無法儲存
updates-call-answered = 它回應了 { $result }
updates-call-refused = 它沒有接受請求：{ $result }
updates-call-failed = 無法呼叫
updates-called = 已於 { $when } 為 { $version } 呼叫：{ $result }
updates-called-failed = 已於 { $when } 為 { $version } 呼叫：{ $result }。Hoard 會在一小時後再試一次。
updates-latest = Hoard { $version } 是最新版本。
updates-out = Hoard { $version } 已推出。
updates-out-idle = Hoard { $version } 已推出；會在沒有任何工作進行時，於幾分鐘內呼叫。
updates-out-busy = Hoard { $version } 已推出；正在等待所有工作結束（{ $work }）。
updates-busy-more = 另外 { $count } 個
updates-busy-jobs = { $count } 個工作等待中
updates-busy-uploads = { $count } 個上傳
updates-list-separator = 、
updates-what = 有新版本推出時，Hoard 可以在沒有任何工作進行時（沒有上傳、工作、轉換或維護）呼叫一個網址，讓負責更新容器的工具不會在上傳途中重新啟動它。它會傳送 POST 請求，並在 JSON 本文中附上版本資訊。這需要開啟上方的版本檢查。
updates-url = 要呼叫的網址
updates-url-placeholder = 無（例如 http://watchtower:8080/v1/update?container=hoard&async=true）
updates-auth = Authorization
updates-auth-saved = 已儲存（輸入以取代）
updates-auth-placeholder = 無（例如 Bearer my-token）
updates-remove = 移除
updates-save = 儲存
updates-call-now-title = 執行它所連接的動作
updates-call-now = 立即呼叫
updates-what-calls = 可以呼叫什麼？
updates-watchtower = Watchtower 的 HTTP API
updates-watchtower-at = ：
updates-watchtower-with = ，搭配{ " " }
updates-watchtower-token = { " " }與來自{ " " }
updates-watchtower-its = { " " }的權杖（需開啟它的{ " " }
updates-watchtower-endpoint = { " " }端點）。
updates-portainer = Portainer webhook
updates-portainer-rest = ，設定在 Hoard 的 stack 或容器上，並開啟 re-pull image。權杖就包含在網址中。
updates-script = 主機上的指令碼
updates-script-listener = ，放在 webhook 監聽程式（例如{ " " }
updates-script-runs = { " " }）後方，並執行{ " " }
updates-script-end = 。
updates-others = Home Assistant、n8n
updates-others-rest = { " " }或其他任何能接收 webhook 的服務。
updates-poll-before = 或者不設定網址：持有存取權杖的指令碼可以輪詢{ " " }
updates-poll-until = ，直到{ " " }
updates-poll-and = { " " }已設定且{ " " }
updates-poll-after = { " " }為 true，再拉取新版本並重新啟動 Hoard。

## 維護面板

maintenance-load-failed = 無法載入維護
maintenance-reanalyze-title = 要重新分析所有素材包嗎？
maintenance-reanalyze-body = 每個檔案都會重新讀取，素材庫很大時可能需要很長的時間；輪到之前，素材包會顯示為處理中。你自己的名稱、標籤與筆記都會保留。
maintenance-reanalyze-confirm = 重新分析
maintenance-gc-title = 要刪除未使用的檔案嗎？
maintenance-gc-body = 素材庫中沒有任何項目指向的儲存檔案將會永久刪除。封存區中的素材包會保留它們的檔案。
maintenance-gc-confirm = 刪除
maintenance-start-failed = 無法開始「{ $task }」
maintenance-just-now = 剛剛
maintenance-minutes-ago = { $count } 分鐘前
maintenance-hours-ago = { $count } 小時前
maintenance-took-seconds = { $count } 秒
maintenance-took-minutes = { $count } 分鐘
maintenance-took-hours = { $hours } 小時
maintenance-loading = 載入中…
maintenance-running = 執行中…
maintenance-starting = 正在開始…
maintenance-run = 執行
maintenance-started = { $detail } · { $when }開始
maintenance-starting-detail = 正在開始
maintenance-finished = 已完成：
maintenance-failed = 失敗：
maintenance-never-run = 從未執行
maintenance-history = 過去的執行紀錄
maintenance-col-task = 作業
maintenance-col-finished = 完成時間
maintenance-col-took = 耗時
maintenance-col-result = 結果
maintenance-script-before = 備份與損毀檢查也可以從指令碼執行：{ " " }
maintenance-script-comma = 、
maintenance-script-and = { " " }和{ " " }
maintenance-script-after = 。這些執行紀錄也會顯示在這裡。

## 設定頁面

settings-title = 設定
settings-loading = 載入中…
settings-find = 尋找設定
settings-find-none = 沒有與「{ $query }」相符的設定。
settings-load-failed = 無法載入設定：{ $error }
settings-page-appearance = 外觀
settings-page-appearance-blurb = 這個瀏覽器在淺色與深色模式下使用的主題。只儲存在這個瀏覽器中。
settings-page-dragon = 巨龍
settings-page-dragon-blurb = 你的巨龍的寶藏：資源庫裡的每個資源都是其中一件寶物。
settings-page-library = 素材庫
settings-page-library-blurb = Hoard 裡有什麼：有多大、由哪些內容組成、如何成長，以及標籤的完整程度。已封存的項目不計入。
settings-page-copies = 近似副本
settings-page-copies-blurb = 只是尺寸、取樣率或格式不同的同一圖片、聲音或模型檔案，占用空間最多的排在前面。每組的最佳副本已標出；不會刪除任何內容。
copies-loading = 正在尋找近似副本…
copies-refreshing = 找到這些之後素材庫已有變更。正在重新尋找…
copies-none = 沒有近似副本：每張圖片、每個聲音和模型在這裡都只有一份。
copies-load-failed = 無法列出近似副本：{ $error }
copies-sum = 近似副本組：{ $count }。非最佳副本占用 { $size }。
copies-set = 第 { $n } 組
copies-set-wasted = 非最佳副本占用 { $size }
copies-more = 顯示更多（還剩 { $count }）
settings-page-tags = 標籤
settings-page-tags-blurb = 素材庫中的所有標籤。合併意思相同的標籤，或重新命名某個標籤；標記器之後會使用新名稱。
settings-page-store-files = 商店檔案
settings-page-store-files-blurb = 商店附在素材旁的檔案，Hoard 會讀取它們來為素材包命名並標示作者。教 Hoard 讀取它還不認識的檔案，並分享你教它的內容。
settings-page-locations = 素材庫位置
settings-page-locations-blurb = 伺服器上存放素材庫的資料夾：包括 Hoard 存放檔案的資料夾，以及它在原位置建立索引的素材包資料夾。
settings-page-locations-blurb-desktop = 存放素材庫的資料夾：包括 Hoard 存放檔案的資料夾，以及它在原位置建立索引的素材包資料夾。
settings-page-tools = 工具
settings-page-tools-blurb = Hoard 用來轉換與讀取檔案的外部程式。
settings-page-ai = AI 標記
settings-page-ai-blurb = 除了內建規則之外，由 AI 模型為新素材包命名、撰寫描述並加上標籤。它加上的標籤會標示為建議。
settings-page-search-blurb = 依圖片顯示的內容搜尋圖片、依聽起來像什麼搜尋音效，而不只是依名稱，並找出種類相似、而不只是顏色相似的項目。
settings-page-search-tab = 搜尋
settings-page-adding = 新增素材包
settings-page-adding-blurb = 新增素材包時，除了讀取與加上標籤之外，Hoard 還會做什麼。
settings-page-downloads = 下載
settings-page-downloads-blurb = Hoard 傳送的檔案如何命名：包括下載、zip、hoard 工具與 AI 助理。素材庫中的檔案維持原樣。
settings-page-account = 帳號
settings-page-account-blurb = 你的密碼、其他可以登入的使用者，以及供指令碼與其他程式使用的存取權杖。
settings-page-auth = 驗證
settings-page-auth-blurb = 使用者如何登入：Hoard 自己的帳號、LDAP 目錄、Microsoft Active Directory 或 OAuth 提供者，工作階段的有效期間，以及哪些群組取得哪種角色。
settings-page-connection = 連線
settings-page-connection-blurb = 瀏覽器與應用程式如何連線到 Hoard：它對外使用的網址，以及部分瀏覽器功能需要的 HTTPS。
settings-page-performance = 效能
settings-page-performance-blurb = 伺服器的工作量、縮圖如何製作與保存，以及它可接收的上限。
settings-page-performance-blurb-desktop = Hoard 的工作量、縮圖如何製作與保存，以及它可接收的上限。
settings-page-issues = 問題
settings-page-issues-blurb = 開啟或處理檔案時發生的錯誤：無法解壓縮的壓縮檔、無法讀取的檔案、失敗的工作。排除原因後可以重試，或將它們略過。
settings-page-maintenance = 維護
settings-page-maintenance-blurb = 備份、檢查損毀並進行整理。每種作業一次只會執行一個，執行期間你可以離開這個頁面。
settings-page-about = 關於
settings-page-about-blurb = Hoard 是什麼、這是哪個版本，以及構成它的他人成果。
settings-the-default = 預設值
settings-default-on = 開啟
settings-default-off = 關閉
settings-default-auto = 自動
settings-option-saved = { $name }：{ $value }
settings-back-to-default = 恢復預設值
settings-save = 儲存
settings-use = 使用
settings-status-off = 關閉

## 設定：外觀

settings-themes = 主題
settings-look-light = 淺色
settings-look-dark = 深色
settings-look-group =
    { $scheme ->
        [light] 淺色主題
       *[dark] 深色主題
    }
settings-preview =
    目前顯示{ $preview ->
        [light] 淺色
       *[dark] 深色
    }主題供你預覽。你使用的是{ $used ->
        [light] 淺色
       *[dark] 深色
    }主題{ $system ->
        [yes] （跟隨系統）
       *[other] { "" }
    }，離開這個頁面後就會恢復。
settings-preview-back =
    返回{ $used ->
        [light] 淺色
       *[dark] 深色
    }
settings-look-hint = 可以從頂端列切換淺色與深色（或跟隨系統）。

## 設定：工具

settings-blender-what = 在背景將 .blend、.dae、.abc 與 .x3d 模型轉換為 glTF，讓它們有縮圖與 3D 預覽。在它關閉期間新增的素材包，會在它就緒後轉換。
settings-blender-download = 使用已安裝的 Blender，或從 blender.org 的鏡像站下載 Blender 4.5 LTS（約 400 MB）。
settings-ffmpeg-what = 編碼 MP3、OGG 與 Opus 下載檔案，並讀取 Hoard 本身無法解碼的音訊（AAC、M4A、WMA…）。
settings-ffmpeg-download = 使用系統中具備所需編碼器的 ffmpeg，或下載靜態建置版本（約 80 MB）。
settings-tool-reset = { $tool }：恢復預設值
settings-tool-no-download = 使用已安裝的 { $tool }。這台電腦沒有可下載的建置版本，請自行安裝或提供它的路徑。
settings-tool-path = { $tool } 程式路徑
settings-mode-off = 關閉
settings-mode-auto = 自動
settings-mode-path = 指定程式
settings-tool-looking = 正在尋找…
settings-tool-downloading = 下載中…進度顯示在活動指示器中
settings-tool-ready = 就緒：{ $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = 播放與轉換 MIDI 檔案時使用的樂器。
settings-soundfont-file = SoundFont 檔案
settings-soundfont-placeholder = 在伺服器上找到的，或內建的
settings-soundfont-placeholder-desktop = 在這台電腦上找到的，或內建的
settings-soundfont-default = 在伺服器上找到的，或內建的
settings-soundfont-default-desktop = 在這台電腦上找到的，或內建的
settings-soundfont-send = 選擇檔案…
settings-soundfont-sent = MIDI 現在使用 { $name } 播放

## 設定：圖片搜尋

settings-clip-heading = 依圖片內容搜尋
settings-clip-reset = 圖片搜尋：恢復預設值
settings-clip-saved-on = 圖片搜尋：開啟
settings-clip-saved-off = 圖片搜尋：關閉
settings-clip-what = 輸入你要找的東西（「mossy rock」、「sci-fi door」、「wooden barrel」），顯示這些內容的圖片就會和名稱含有這些字詞的項目一起出現。「更多類似項目」會找出種類相同、而不只是顏色相同的項目。
settings-clip-toggle = 開啟圖片搜尋
settings-clip-hint = 使用 Google 的 SigLIP 2 模型，在這台伺服器的處理器上執行：任何資料都不會離開伺服器。開啟後會從 Hugging Face 下載模型（{ $size } MB），然後以低優先順序在背景學習每張圖片。在它工作或有人搜尋時，會使用 350 到 750 MB 的記憶體。關閉後會刪除模型，但保留它學到的內容，供下次使用。
settings-clip-hint-desktop = 使用 Google 的 SigLIP 2 模型，在這台電腦的處理器上執行：任何資料都不會離開這台電腦。開啟後會從 Hugging Face 下載模型（{ $size } MB），然後以低優先順序在背景學習每張圖片。在它工作或你搜尋時，會使用 350 到 750 MB 的記憶體。關閉後會刪除模型，但保留它學到的內容，供下次使用。
settings-clip-looking = 正在尋找模型…
settings-clip-downloading = 正在下載模型…進度顯示在活動指示器中
settings-clip-ready-empty = 就緒。新增素材包時會學習其中的圖片。
settings-clip-ready-all =
    { $count ->
       *[other] 就緒：已認識全部 { $count } 張圖片
    }
settings-clip-learning = 就緒：正在學習圖片，目前已完成 { $done }／{ $total }
settings-clip-relearning = 在新模型學完所有圖片之前，搜尋使用舊模型學到的內容。
settings-clip-unavailable = 無法取得模型：{ $detail }。每小時會再試一次。
settings-clip-where = 執行位置
settings-clip-here = 這台伺服器：不會有任何內容離開
settings-clip-here-desktop = 這台電腦：不會有任何內容離開
settings-clip-hosted = 託管服務
settings-clip-hosted-warning = 圖片會離開這台伺服器。圖庫中每張圖片的縮圖，以及每次圖片搜尋的文字，都會傳送給 { $service }，它會為此收費，並依其條款保留這些內容。
settings-clip-hosted-warning-desktop = 圖片會離開這台電腦。圖庫中每張圖片的縮圖，以及每次圖片搜尋的文字，都會傳送給 { $service }，它會為此收費，並依其條款保留這些內容。
settings-clip-hosted-hint = 這裡不會下載或執行任何東西：服務會分批學習每張圖片。換一個服務或模型會重新學習全部圖片，每個學過的內容都會保留，方便切換回來。
settings-clip-saved-here = 圖片搜尋：在這台伺服器上執行
settings-clip-saved-here-desktop = 圖片搜尋：在這台電腦上執行
settings-clip-saved-hosted = 圖片搜尋：在 { $service } 上執行
settings-clip-hosted-unavailable = 無法使用 { $service }：{ $detail }。每小時會再試一次。
settings-clap-heading = 依音效聽起來像什麼搜尋
settings-clap-reset = 音效搜尋：恢復預設值
settings-clap-saved-on = 音效搜尋：開啟
settings-clap-saved-off = 音效搜尋：關閉
settings-clap-what = 輸入你要找的東西（「distant thunder」、「glassy UI blip」、「footsteps on gravel」），聽起來像這樣的音效就會和名稱含有這些字詞的項目一起出現。
settings-clap-toggle = 開啟音效搜尋
settings-clap-hint = 使用 LAION 的 CLAP 模型，在這台伺服器的處理器上執行：任何資料都不會離開伺服器。開啟後會從 Hugging Face 下載模型（{ $size } MB），然後以低優先順序在背景聆聽每個音效的開頭。在它工作或有人搜尋時，最多會使用約 1 GB 的記憶體。關閉後會刪除模型，但保留它學到的內容，供下次使用。
settings-clap-hint-desktop = 使用 LAION 的 CLAP 模型，在這台電腦的處理器上執行：任何資料都不會離開這台電腦。開啟後會從 Hugging Face 下載模型（{ $size } MB），然後以低優先順序在背景聆聽每個音效的開頭。在它工作或你搜尋時，最多會使用約 1 GB 的記憶體。關閉後會刪除模型，但保留它學到的內容，供下次使用。
settings-clap-ready-empty = 就緒。新增素材包時會學習其中的音效。
settings-clap-ready-all =
    { $count ->
       *[other] 就緒：已認識全部 { $count } 個音效
    }
settings-clap-learning = 就緒：正在學習音效，目前已完成 { $done }／{ $total }

## 設定：AI 標記

settings-ai-service = 服務
settings-ai-off = 關閉
settings-ai-off-hint = 只用內建規則為素材包加上標籤。
settings-ai-local = 本機模型
settings-ai-local-hint = Hoard 的 compose 檔案中的 llm 服務（或 cargo xtask llm）。請在下方下載模型；它在這台電腦上執行，任何資料都不會離開。
settings-ai-chatgpt = ChatGPT 方案
settings-ai-chatgpt-hint = 使用你的 ChatGPT Go、Plus、Pro 或 Business 方案，而不是 API 金鑰。
settings-ai-openrouter-hint = 一把金鑰即可使用數百個託管模型，其中有些是免費的。可接收圖片的模型會有標示。
settings-ai-openai-hint = 由 OpenAI 託管。
settings-ai-anthropic-hint = 由 Anthropic 託管，透過其相容 OpenAI 的端點。
settings-ai-gemini-hint = 由 Google 託管，透過其相容 OpenAI 的端點。
settings-ai-ollama-hint = 你自己執行的 Ollama 伺服器。如果它不是名為 ollama 的容器，請變更網址。
settings-ai-custom = 其他（相容 OpenAI）
settings-ai-custom-hint = 任何支援 OpenAI chat API 的服務：LM Studio、vLLM、llama.cpp、LiteLLM…
settings-ai-connecting = 連線中…
settings-ai-connected = 已連線 · { $count } 個模型
settings-ai-connected-none = 已連線，但沒有列出任何模型；請輸入模型名稱
settings-chatgpt-connected-none = 已連線；請輸入模型名稱
settings-ai-takes-images = 可接收圖片
settings-ai-saved = AI 標記：{ $service }
settings-ai-saved-off = AI 標記已關閉
settings-ai-reset = AI 標記：恢復預設值
settings-ai-on = 開啟：{ $service } · { $model }
settings-ai-url = 網址
settings-ai-key = API 金鑰
settings-ai-key-saved = 已儲存（輸入以取代）
settings-ai-key-paste = 貼上你的金鑰
settings-ai-key-optional = 無（選填）
settings-ai-key-remove = 移除
settings-ai-get-key = 取得金鑰
settings-ai-model = 模型
settings-ai-model-pick = 選擇或輸入模型
settings-ai-model-name = 模型名稱
settings-ai-refresh = 重新整理
settings-ai-connect = 連線
settings-ai-need-key = 請新增 API 金鑰以查看它的模型
settings-ai-vision = 一併傳送縮圖（需要可接收圖片的模型）
settings-ai-batch = 每次請求
settings-ai-batch-unit = 個素材（小型本機模型請少一點，大型託管模型可以多一點）

## 設定：ChatGPT 登入

settings-chatgpt-account = 帳號
settings-chatgpt-checking = 檢查中…
settings-chatgpt-status-as = 已以 { $email } 登入
settings-chatgpt-status = 已登入
settings-chatgpt-signed-in-as = 已以 { $email } 登入 ChatGPT
settings-chatgpt-signed-in = 已登入 ChatGPT
settings-chatgpt-signed-out = 已登出 ChatGPT
settings-chatgpt-continue = 使用 ChatGPT 繼續
settings-chatgpt-sign-out = 登出
settings-chatgpt-sign-out-title = 要登出 ChatGPT 嗎？
settings-chatgpt-sign-out-in-use = AI 標記正在使用你的 ChatGPT 方案，因此會停止運作，直到你重新登入或選擇其他服務為止。
settings-chatgpt-sign-out-body = Hoard 會忘記這次登入，並通知 OpenAI 結束它。
settings-chatgpt-manage = 管理用量
settings-chatgpt-finish-before = 請在開啟的視窗中完成登入（
settings-chatgpt-finish-link = 重新開啟
settings-chatgpt-finish-middle = ）。如果最後停在無法載入的頁面，且網址以{ " " }
settings-chatgpt-finish-after = { " " }開頭，請複製該網址並貼到這裡：
settings-chatgpt-paste = 登入結束時的網址
settings-chatgpt-finish = 完成
settings-chatgpt-usage = 標記請求會計入你的 ChatGPT 方案用量。「管理用量」會顯示 Hoard 已使用的量，並讓你設定上限。若要從 ChatGPT 本身中斷 Hoard 的連線，請到 chatgpt.com 的「設定 → 安全性與登入 → 使用 ChatGPT 登入」。

## 設定：本機模型

settings-local-downloading-progress = 下載中 { $percent }% · { $done }／{ $total }
settings-local-downloading = 下載中…
settings-local-incomplete = 下載已停止
settings-local-failed = 上次啟動失敗
settings-local-in-use-running = 使用中 · 執行中
settings-local-in-use = 使用中
settings-local-starting = 正在啟動…
settings-local-running = 執行中
settings-local-ready-images = 就緒 · 可接收圖片
settings-local-ready-text = 就緒 · 僅限文字
settings-local-delete-title = 要刪除 { $name } 嗎？
settings-local-delete-in-use = AI 標記正在使用這個模型，因此會停止運作，直到你選擇其他模型為止。它的檔案會從模型伺服器中刪除。
settings-local-delete-downloading = 下載會停止，已下載的部分也會刪除。
settings-local-delete-body = 它的檔案會從模型伺服器中刪除。你之後可以再次下載。
settings-local-delete = 刪除
settings-local-deleting = 刪除中…
settings-local-cancel = 取消
settings-local-resume = 繼續
settings-local-download = 下載
settings-local-start-before = 請用{ " " }
settings-local-start-or = { " " }（開發時用{ " " }
settings-local-start-developing = ）啟動它，然後{ " " }
settings-local-start-retry = 再試一次
settings-local-start-after = 。只執行單一模型的伺服器也可以使用：請在下方輸入它的模型名稱。
settings-local-list = 模型伺服器上的模型
settings-local-none = 還沒有下載任何模型。請在下方選擇一個；它會保存在伺服器上，你隨時可以切換。
settings-local-catalog = 可下載的模型
settings-local-size = { $size } GB · { $license }
settings-local-other = Hugging Face 上的其他模型
settings-local-other-placeholder = Hugging Face 上的其他模型：owner/name-GGUF:Q4_K_M

## 設定：新增素材包

settings-store-pages = 商店頁面
settings-store-pages-what = 新的素材包會在背景中於商店查詢，以取得商店頁面、賣家與授權資訊。
settings-store-lookup = 自動查詢商店頁面
settings-unreal = Unreal 套件
settings-unreal-reset = Unreal 套件：恢復預設值
settings-unreal-what = 如何處理 .uasset 檔案。
settings-conv = 轉換規則
settings-conv-what = 將 Hoard 保存的檔案轉換為同一種格式，例如把所有 OBJ 與 FBX 轉為 GLB，讓整個素材庫的格式一致。轉換後的檔案會取代原始檔案，並保留其名稱、標籤、收藏集與連結。監看資料夾中的檔案永遠不會被變更。
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = 啟用
settings-conv-to = 轉換為
# Heading over the file formats a rule converts.
settings-conv-from = 來源格式
settings-conv-remove = 移除
settings-conv-remove-label = 移除規則 { $n }
settings-conv-taken = 另一條規則已在轉換此格式
settings-conv-upright = 轉換時將模型擺正（Y 軸朝上，面向 +Z），與素材包中模型的顯示方式一致
settings-conv-flac-what = 聲音完全不變，空間約為一半；WAV 中的其他所有內容（循環點、cue 標記、標籤）都保留在 FLAC 裡，因此隨時可以把 WAV 原樣還原。帶有浮點或 32 位元取樣的 WAV 會維持為 WAV。
settings-conv-add = 新增規則
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = 原始檔案保留
settings-conv-keep-days =
    { $count ->
       *[other] 天
    }
settings-conv-keep-what = 在此之前，可在轉換後檔案的頁面還原原始檔案。貼圖不會複製到轉換後的模型中：模型會參照旁邊的圖片檔案。模型的 .mtl 或 .bin 檔案會保留。
settings-conv-save = 儲存規則
settings-conv-revert = 復原變更
settings-conv-existing = 轉換素材庫中已有的檔案
settings-conv-saved = 轉換規則已儲存。它們會套用到此後新增的素材包。
settings-conv-converting =
    { $count ->
       *[other] 正在背景轉換 { $count } 個檔案
    }
settings-conv-nothing = 素材庫中沒有需要轉換的檔案。
settings-conv-ask-title =
    { $count ->
       *[other] 要轉換素材庫中已有的 { $count } 個檔案嗎？
    }
settings-conv-ask-body =
    { $packs ->
       *[other] 它們位於 { $packs } 個素材包中。
    }每個原始檔案會保留 { $days } 天，在此期間可從其頁面還原。
settings-conv-ask-busy =
    { $count ->
       *[other] 其中 { $count } 個位於仍在新增中的素材包，新增完成後就會轉換。
    }
settings-conv-ask-list = 將會轉換的檔案
settings-conv-ask-more = ……以及另外 { $count } 個
settings-conv-ask-skipped = 維持原樣：
settings-conv-skip-watched =
    { $count ->
       *[other] 監看資料夾中的 { $count } 個檔案
    }
settings-conv-skip-unity =
    { $count ->
       *[other] Unity 專案依名稱參照的 { $count } 個檔案
    }
settings-conv-skip-used =
    { $count ->
       *[other] 被場景或其他檔案依名稱參照的 { $count } 個檔案
    }
settings-conv-skip-beside =
    { $count ->
       *[other] 轉換後的副本已在其素材包中的 { $count } 個檔案
    }
settings-conv-skip-blender =
    { $count ->
       *[other] Blender 尚未讀取的 { $count } 個檔案（設定 → 工具）
    }
settings-conv-ask-new = 儲存，僅限新素材包
settings-conv-ask-save-convert =
    { $count ->
       *[other] 儲存並轉換 { $count } 個檔案
    }
settings-conv-ask-convert =
    { $count ->
       *[other] 轉換 { $count } 個檔案
    }
settings-unreal-off = 維持原樣
settings-unreal-off-hint = 需要時再從素材包頁面轉換。
settings-unreal-keep = 轉換，保留原始檔案
settings-unreal-keep-hint = 貼圖、音效與材質會轉為通用格式；.uasset 也會保留。
settings-unreal-replace = 轉換，捨棄原始檔案
settings-unreal-replace-hint = 內容取出後即可節省空間。網格仍需等待 Unreal 工作程序處理。

## 設定：下載

settings-tn = 貼圖名稱
settings-tn-saved = 已儲存貼圖名稱
settings-tn-reset = 貼圖名稱：恢復預設值
settings-tn-what = 不論檔案原本如何命名，材質的貼圖在傳送時都會以材質名稱與各貼圖的用途命名：例如 Rock_Normal.png，而不是 rock_moss_xeokah0_raw_8K_Normal.png。素材庫中的檔案不會變動；而模型或 MTL 中指定名稱的貼圖會保留該名稱，讓模型仍能找到它。
settings-tn-toggle = 依材質為貼圖命名
settings-tn-style = 樣式
settings-tn-style-group = 命名樣式
settings-tn-descriptive = 描述式
settings-tn-short = 簡短
settings-tn-custom = 自訂
settings-tn-pattern = 樣式
settings-tn-hint-name = { " " }是材質，{ " " }
settings-tn-hint-map = { " " }是貼圖的用途，{ " " }
settings-tn-hint-size = { " " }是組合有多種大小時的尺寸（如果樣式中沒有，會加在結尾）。
settings-tn-example = 範例
settings-tn-suffixes = 各種貼圖的字尾
settings-tn-slots = 在 GLB 下載中，封裝在模型內、沒有獨立檔案的貼圖會以材質和它所填入的插槽命名，這些插槽也有字尾：
settings-tn-endings = 自訂結尾
settings-tn-endings-what = 若貼圖不是某個材質的貼圖之一，會保留自己的名稱，除非名稱以下列其中一個結尾結束：_diff → _BC 會把 rock_diff.png 傳送為 rock_BC.png。第二欄留空即可去掉該結尾。
settings-tn-ending-from = 結尾
settings-tn-ending-from-example = _diff
settings-tn-ending-to = 替換為
settings-tn-ending-to-example = _BC
settings-tn-ending-add = 新增結尾
settings-tn-ending-remove = 移除
settings-tn-ending-remove-label = 移除 { $ending }

## 設定：一般與連線

settings-name = 名稱
settings-name-what = 這台伺服器在頂端列、瀏覽器分頁與登入頁面上顯示的名稱，讓你在有多個 Hoard 時能分辨。
settings-name-what-desktop = 這個素材庫在頂端列上顯示的名稱。
settings-address = 網址
settings-address-what = 當使用者與應用程式連線到 Hoard 的網址與這個瀏覽器使用的網址不同時填寫：例如反向代理、通道或 tailnet 名稱，像是 https://hoard.example.com。Hoard 提供的連結（給 Claude 與其他應用程式、登入時、憑證上）都會使用它。
settings-public-url = 公開網址
settings-public-url-placeholder = 取自瀏覽器的網址（例如 https://hoard.example.com）
settings-public-url-default = 取自瀏覽器的網址

## 設定：效能頂部的效能分析器

profiler = 效能分析器
profiler-what = Hoard 在過去十分鐘內做了什麼，你檢視時每兩秒更新一次。資料只保存在記憶體中，Hoard 重新啟動後會從頭開始。
profiler-starting = 第一批讀數馬上就到。
profiler-failed = 無法讀取效能分析器：{ $error }
profiler-cpu = 處理器使用率
profiler-cpu-hoard = Hoard
profiler-cpu-machine = 整台機器
profiler-cpu-summary = Hoard 正在使用 { $hoard } 的處理器，整台機器則為 { $machine }。
profiler-memory = 記憶體
profiler-serving = 進行中的請求
profiler-working = 正在處理
profiler-working-none = 背景沒有執行任何工作。
profiler-working-more =
    { $count ->
       *[other] 還有 { $count } 項。
    }
profiler-requests = 處理請求的時間
profiler-requests-what = 每根長條代表十秒內的請求，時間依去向分開。
profiler-part-db = 資料庫
profiler-part-db-title = 等待資料庫並執行其查詢
profiler-part-disk = 磁碟
profiler-part-disk-title = 讀取儲存的檔案，並壓縮要傳送的內容
profiler-part-send = 傳送
profiler-part-send-title = 等待網路接收已傳送的內容
profiler-part-other = 其他
profiler-part-other-title = 產生回應
profiler-request-count =
    { $count ->
       *[other] { $count } 個請求
    }
profiler-requests-none = 過去十分鐘沒有請求。
profiler-slowest = 最慢：{ $route }，{ $time }
profiler-routes = 依路由
profiler-route = 路由
profiler-count = 請求數
profiler-average = 平均
profiler-p95 = 最慢的 5%
profiler-split = 時間去向
profiler-sent = 已傳送
profiler-portal = 介面本身的檔案

## 設定：效能

settings-render = 3D 縮圖
settings-render-toggle = 為 3D 模型算繪縮圖
settings-gpu-on = 使用 GPU：{ $renderer }
settings-gpu-software = 找不到 GPU，因此使用軟體算繪（{ $renderer }）
settings-gpu-none = 找不到 GPU 或軟體算繪器，因此模型不會有縮圖
settings-thumbs = 縮圖
settings-thumbs-what = 新縮圖與模型視圖的儲存方式。有透明度的圖片與像素畫一律使用 PNG。若要縮小現有的縮圖，請在「維護」中執行「縮小縮圖」。
settings-previews-size = 預覽目前佔用 { $size }。
settings-thumbs-saved = 縮圖：{ $quality }
settings-thumbs-small = 較小
settings-thumbs-small-hint = JPEG，品質 70。在格線中沒問題；近看會有點模糊。
settings-thumbs-balanced = 平衡
settings-thumbs-balanced-hint = JPEG，品質 82。以縮圖大小來看，很難與原圖區分。
settings-thumbs-sharp = 較清晰
settings-thumbs-sharp-hint = JPEG，品質 92。大小約為「平衡」的兩倍。
settings-thumbs-lossless = 無失真
settings-thumbs-lossless-hint = PNG，與產生時完全相同。大好幾倍。
settings-thumbs-avif = 以 AVIF 傳送縮圖
settings-thumbs-avif-what = 能顯示 AVIF 的瀏覽器會收到較小的縮圖，往往不到一半大小。每張縮圖會在第一次顯示時由已儲存的縮圖產生，並與轉換結果一起保存；在此之前會傳送已儲存的那張。首次瀏覽大型素材庫時，產生縮圖最多會佔用兩個核心。無失真縮圖和像素畫會照原樣傳送。
settings-motion-thumbs = 動畫的動態縮圖
settings-motion-thumbs-what = 每個動畫還會產生一段循環播放的短片，指向其圖塊時播放。每段約 40 KB，大型資源庫需要一段時間才能產生完。關閉後會刪除它們。
settings-limits = 工作量與上限
settings-limits-what = 每個欄位在你離開欄位或按下 Enter 時儲存。大小需加上單位：100 GB、512 MB、1.5 TB；只輸入數字則為 GB。
settings-workers = 背景工作程序
settings-workers-unit = 個同時進行的工作（匯入、標記、轉換）
settings-max-upload = 最大上傳大小
settings-max-upload-unit = 每個檔案
settings-max-expand = 最大素材包大小
settings-max-expand-unit = 解壓縮後
settings-conversion-cache = 轉換後的下載檔案
settings-conversion-cache-unit-now = 最多保留的量，目前為 { $size }；最久未使用的會先刪除
settings-conversion-cache-unit = 最多保留的量；最久未使用的會先刪除

## 設定：維護

settings-updates = 更新
settings-version = 這是 Hoard { $version }。
settings-update-check = 每隔幾小時檢查 GitHub 是否有新版本
settings-desktop-updates = Hoard 每次啟動時都會尋找新版本，在背景下載，並在你結束時安裝。
settings-backups = 排程備份
settings-backups-what = 資料庫會自動備份，每次備份都會顯示在下方的歷史紀錄中。
settings-backup-every = 備份
settings-backup-off = 永不
settings-backup-daily = 每天
settings-backup-weekly = 每週
settings-backup-keep = 保留
settings-backup-keep-unit = 份最新的排程備份
settings-backup-copy-to = 同時複製到
settings-backup-copy-to-none = 不複製到其他資料夾
settings-backups-kept = 每份備份以 hoard-auto-…db 存放在備份資料夾中；如果填了其他資料夾（另一顆磁碟，或同步應用程式會備份的資料夾），也會複製一份過去，請填寫它在伺服器上的完整路徑。只會刪除較舊的排程備份，手動建立的備份會保留。
about-what = Hoard 把遊戲素材（模型、貼圖、音效、音樂、精靈圖等）放在一個素材庫裡，供你搜尋、預覽和取用，並與你允許存取的所有人共享。
about-version = 版本
about-edition = 發行版
about-edition-server = 伺服器
about-edition-desktop = 桌面應用程式
about-edition-steam = Steam 版桌面應用程式
about-made-by = 開發者
about-source = GitHub 上的原始碼
about-issues = 回報問題
about-discord = 在 Discord 上和我們聊天
about-steam = Steam 上的 Hoard
about-license = 授權條款
about-license-what = Hoard 以 MIT No Attribution 授權條款開源：你可以隨意使用、修改和分享。
about-downloaded = 需要時下載
about-downloaded-what = Hoard 不內建這些元件。它會在第一次需要時下載各個元件，每個元件都有各自的授權條款。
about-blender = 轉換 3D 模型並產生縮圖。
about-ffmpeg = 轉換音訊和影片並產生預覽。
about-siglip = 依圖片中的內容尋找圖片。
about-clap = 依聲音聽起來的樣子尋找音效。
about-models = 你為本機 AI 伺服器選擇的模型適用其作者的授權條款。
about-built-with = 使用的開放原始碼軟體
about-built-with-what = 構成 Hoard 的開放原始碼函式庫及其授權條款。
about-steamworks = Steam 版另含 Steamworks SDK，© Valve Corporation。
about-licenses-loading = 正在載入清單…
about-licenses-failed = 無法載入清單。
about-filter = 依名稱或授權條款篩選
about-count = { $shown } / { $total }
about-no-license = 未註明授權條款
about-report = 回報問題…
report-title = 回報問題
report-what = 這是 GitHub 上的 issue 調查問題所需的資訊。其中沒有任何隱私內容：存取權杖、密碼、金鑰、電子郵件地址以及資料夾路徑中的使用者名稱都會被移除。送出前請先讀過一遍。
report-loading = 正在收集報告…
report-failed = 無法收集報告：{ $error }
report-log = 伺服器最近記錄的日誌
report-copy = 複製
report-copied = 已複製報告。
report-github = 在 GitHub 上建立 issue
report-github-hint = 新的 issue 會自動填入這些內容。日誌太長時只會放入最後幾行；「複製」包含全部內容。
report-close = 關閉

## 素材類型

api-kind-texture = 貼圖
api-kind-vector = 向量圖
api-kind-sheet = 精靈圖表
api-kind-audio = 音訊
api-kind-model = 模型
api-kind-animation = 動畫
api-kind-font = 字型
api-kind-shader = 著色器
api-kind-material = 材質
api-kind-video = 影片
api-kind-document = 文件
api-kind-data = 資料
api-kind-other = 其他
api-kind-texture-name = 貼圖
api-kind-vector-name = 向量圖
api-kind-sheet-name = 精靈圖表
api-kind-audio-name = 音訊
api-kind-model-name = 模型
api-kind-animation-name = 動畫
api-kind-font-name = 字型
api-kind-shader-name = 著色器
api-kind-material-name = 材質
api-kind-video-name = 影片
api-kind-document-name = 文件
api-kind-data-name = 資料
api-kind-other-name = 其他

## 通知、對話方塊與載入

ui-undo = 復原
ui-today = 今天
ui-yesterday = 昨天
ui-days-ago = { $days } 天前
ui-notifications = 通知
ui-dismiss = 略過
ui-cancel = 取消
ui-lazy-loading = 正在載入{ $what }…
ui-lazy-failed = 無法載入{ $what }。
ui-lazy-reload = 重新載入頁面

## 重設按鈕

reset-title = 恢復預設值：{ $value }
reset-label = 恢復預設值，{ $value }

## 大小輸入

size-invalid = 請輸入像 100 GB、512 MB 或 1.5 TB 這樣的大小
size-hint = 像 100 GB、512 MB 或 1.5 TB 這樣的大小；只輸入數字則為 GB

## 驗證面板（設定 → 驗證）

auth-load-failed = 無法載入驗證設定：{ $error }
auth-saved = 已儲存。下次登入時生效。
auth-method-internal = 內部
auth-method-internal-hint = 由 Hoard 保存的名稱與密碼，在「設定 → 帳號」中新增。
auth-method-ldap-hint = 由目錄檢查在登入頁面輸入的名稱與密碼：OpenLDAP、FreeIPA、LLDAP 或其他 LDAP 伺服器。
auth-method-ad-hint = Windows 網域帳號使用平常的名稱與密碼登入。
auth-method-oauth-hint = 一個「使用…登入」按鈕，會前往 Google、Microsoft、GitHub、Keycloak、Authentik 或其他提供者，然後再返回。
auth-session-unused = 30 天未使用後
auth-session-1h = 登入 1 小時後
auth-session-8h = 登入 8 小時後
auth-session-day = 登入一天後
auth-session-week = 登入一週後
auth-session-30d = 登入 30 天後
auth-session-hours = 登入 { $hours } 小時後
auth-role-view = 可檢視
auth-role-edit = 可修改
auth-role-admin = 管理員
auth-role-word-view = 可檢視
auth-role-word-edit = 可修改
auth-role-word-admin = 管理員
auth-role-word-default = { $role }（預設角色）
auth-tls-starttls-hint = 一般 LDAP（連接埠 389），在傳送任何內容前切換為 TLS。
auth-tls-ldaps-hint = 從一開始就使用 TLS（連接埠 636）。
auth-tls-none = 無
auth-tls-none-hint = 密碼會以原樣在網路上傳送。僅限在你信任的網路上使用。
auth-test-failed = 無法測試：{ $error }
auth-preset-custom = 其他（OpenID Connect）
auth-preset-custom-note = Keycloak、Authentik、Authelia、Okta、Zitadel 等：提供 issuer，「探索」就會填入各個網址。
auth-preset-google-note = 在 Google Cloud Console → API 和服務 → 憑證 中建立 OAuth 用戶端（網頁應用程式）。Google 不會傳送群組，因此請在「設定 → 帳號」中指派角色，並關閉首次登入時建立帳號，除非任何擁有 Google 帳戶的人都應該能進入。
auth-preset-microsoft-note = 在 Entra ID → 應用程式註冊 中註冊應用程式（以下方網址作為 Web 重新導向）。將「organizations」換成你的租用戶 ID，就只會讓你的組織成員進入。若要依群組指派角色，請在「權杖設定」中新增 groups 宣告：它會列出群組 ID。
auth-preset-github-note = 在 GitHub → Settings → Developer settings 中建立 OAuth app。GitHub 不會傳送群組，因此請在「設定 → 帳號」中指派角色，並關閉首次登入時建立帳號，除非任何擁有 GitHub 帳號的人都應該能進入。
auth-preset-gitlab-note = 在 GitLab → Preferences → Applications（或你的執行個體的管理區域：請變更網址中的 gitlab.com）中建立應用程式。
auth-discovered = 已填入提供者的網址。
auth-copied = 已複製
auth-copy-failed = 無法複製：請自行選取並複製
auth-sign-in-with = 登入方式
auth-own-accounts-still-work = Hoard 自己的帳號仍然使用各自的密碼登入，因此目錄或提供者停擺或設定錯誤時，你也不會被鎖在外面。
auth-sessions = 工作階段
auth-sign-out-after = 將使用者登出
auth-ldap-server = LDAP 伺服器
auth-server-type = 伺服器類型
auth-server-type-custom = 自訂
auth-servers = 伺服器
auth-server-n = 伺服器 { $n }
auth-remove-server-n = 移除伺服器 { $n }
auth-remove = 移除
auth-add-server = 新增伺服器
auth-servers-hint = 某台伺服器無法連線時，會嘗試下一台。
auth-security = 安全性
auth-connection-security = 連線安全性
auth-skip-verify = 不檢查伺服器的憑證（僅供測試）
auth-ca = 憑證授權單位
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  （留空：使用系統信任的憑證）
auth-lookup = 查詢使用者
auth-anonymously = 匿名
auth-reader-account = 讀取帳號
auth-reader-password = 它的密碼
auth-saved-type-to-replace = 已儲存：輸入以取代
auth-people-under = 使用者位於
auth-name-attribute = 名稱屬性
auth-only-matching = 只限符合的項目
auth-groups-under = 群組位於
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  （留空：只使用 memberOf）
auth-their-groups = 他們的群組
auth-group-filter-hint = { "{" }dn{ "}" } 是使用者的項目，{ "{" }user{ "}" } 是他們登入時使用的名稱。
auth-test = 測試
auth-test-hint = 連線到每台伺服器，並以讀取帳號登入。若提供名稱與密碼，也會以該使用者登入，並顯示 Hoard 會如何判定。不會儲存任何內容。
auth-test-user = 要登入的使用者
auth-test-user-placeholder = 要登入的使用者（選填）
auth-test-password = 他們的密碼
auth-testing = 測試中…
auth-test-connectivity = 測試連線
auth-test-server-ok = 已連線，且讀取帳號已登入
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }已登入（
auth-test-signed-in-after = ）。
auth-test-role = 角色會是：{ $role }。
auth-test-no-role = 他們不在「角色」下列出的任何群組中，因此會被拒絕。
auth-test-groups = 群組：{ $groups }
auth-test-no-groups = 找不到任何群組。
auth-provider = 提供者
auth-redirect = 重新導向網址
auth-copy = 複製
auth-redirect-hint = 建立 Hoard 的用戶端時，請將這個網址提供給提供者。它來自「設定 → 連線」中的公開網址，或你目前使用的網址。
auth-issuer = Issuer
auth-discover = 探索
auth-client-id = 用戶端 ID
auth-client-secret = 用戶端密碼
auth-authorization-url = 授權網址
auth-token-url = 權杖網址
auth-userinfo-url = 使用者資訊網址
auth-userinfo-placeholder = （留空：由 ID 權杖表明身分）
auth-scopes = 範圍
auth-name-from = 名稱來源
auth-groups-from = 群組來源
auth-groups-from-placeholder = （無）
auth-button = 按鈕
auth-button-placeholder = 你的帳號
auth-accounts = 帳號
auth-provision = 在使用者首次登入時建立 Hoard 帳號
auth-provision-on-hint = 否則，管理員需先在「設定 → 帳號」中逐一新增使用者。
auth-provision-off-hint = 管理員需先在「設定 → 帳號」中逐一新增使用者，並選擇他們的登入方式。
auth-open-to-anyone = 任何擁有 { $provider } 帳號的人都能進入。請在下方新增檢視者群組，或關閉此選項並自行新增使用者。
auth-new-accounts = 新帳號
auth-roles-from-groups = 依群組指派角色
auth-roles-hint = 其成員會取得各角色的群組，每行一個，以名稱或 DN 表示。符合的最高角色優先，且每次登入時都會檢查。若未列出任何群組，新帳號會取得上方的角色，之後再由管理員變更。若在「可檢視」下列出群組，就只有所列群組的成員能夠登入。
auth-roles-hint-oauth = 其成員會取得各角色的群組，每行一個，以名稱或 DN 表示（或 groups 宣告所列出的任何內容）。符合的最高角色優先，且每次登入時都會檢查。若未列出任何群組，新帳號會取得上方的角色，之後再由管理員變更。若在「可檢視」下列出群組，就只有所列群組的成員能夠登入。
auth-saving = 儲存中…
auth-save = 儲存

## HTTPS 面板（設定 → 連線）

https-load-failed = 無法載入 HTTPS：{ $error }
https-off-redirecting = HTTPS 已關閉。正在帶你前往 http://
https-now-on = HTTPS 已開啟
https-saved = 已儲存。新的連線會使用新的憑證。
https-now-off = HTTPS 已關閉
https-http3-failed-toast = HTTP/3 無法啟動：{ $error }
https-copied = 已複製
https-copy-failed = 無法複製：請自行選取並複製
https-mode-off = 關閉
https-mode-off-hint = 只使用一般的 http://。
https-mode-own = Hoard 自己的憑證
https-mode-own-hint = Hoard 會為這台伺服器的網址建立憑證。在每台裝置上安裝一次 Hoard 的憑證授權單位，裝置上的瀏覽器就會信任它。
https-mode-custom = 我的憑證
https-mode-custom-hint = 你已經擁有的憑證，例如來自 tailscale cert 或 Let’s Encrypt。瀏覽器不需安裝任何東西就會信任它。
https-untrusted-own = 這個瀏覽器尚未信任 { $host } 的這個憑證：請在下方安裝 Hoard 的憑證授權單位。你會看到一個可以略過的警告。
https-untrusted-uncovered = 這個瀏覽器尚未信任 { $host } 的這個憑證：憑證並未涵蓋這個網址。你會看到一個可以略過的警告。
https-untrusted = 這個瀏覽器尚未信任 { $host } 的這個憑證。你會看到一個可以略過的警告。
https-now = 目前狀態
https-start-failed = HTTPS 已設定，但無法啟動：{ $error }
https-on-port = HTTPS 已開啟，與 http:// 同樣使用連接埠 { $port }。
https-using-it = 你目前正在使用它。
https-open = 開啟 { $url }
https-trusted = 這個瀏覽器信任它。
https-cert-for-before = 憑證涵蓋{ " " }
https-cert-for-after = ，有效期限至 { $date }。
https-cert-for-after-own = ，有效期限至 { $date }（會自動更新）。
https-not-covered-before = 它並未涵蓋你目前使用的網址{ " " }
https-not-covered-after = 。
https-is-off = HTTPS 已關閉：頁面透過一般的 http:// 提供。
https-addresses = 網址
https-addresses-hint = 憑證涵蓋 Hoard 找到的網址，以及你新增的任何網址（路由器指派給它的名稱、Tailscale 名稱）。
https-remove-name = 移除 { $name }
https-another-address = 其他網址
https-another-address-placeholder = 其他網址，例如 nas.local
https-add = 新增
https-your-cert = 你的憑證
https-saved-cert-before = 已儲存：{ " " }
https-saved-cert-after = { " " }，由 { $issuer } 簽發，有效期限至 { $date }。貼上新的憑證即可取代。
https-pem-before = 憑證（含其憑證鏈）與私密金鑰的 PEM 文字：也就是以{ " " }
https-pem-after = { " " }開頭的檔案。
https-certificate = 憑證
https-choose-file = 選擇檔案…
https-private-key = 私密金鑰
https-checked-before = 涵蓋{ " " }
https-checked-after = ，由 { $issuer } 簽發，有效期限至 { $date }。
https-checked-after-days = ，由 { $issuer } 簽發，有效期限至 { $date }（{ $days } 天）。
https-self-signed = 這是自我簽署的憑證：瀏覽器會對它發出警告。
https-key-matches = 金鑰相符。
https-where-to-get = 從哪裡取得
https-get-tailscale-before = 為你的 tailnet 開啟 HTTPS，然後在伺服器上執行{ " " }
https-get-tailscale-after = { " " }，並上傳這兩個檔案。它們的有效期為 90 天：請在到期前再次執行並重新上傳。
https-get-acme-title = Let’s Encrypt 或其他 ACME 用戶端
https-get-acme-before = （certbot、acme.sh、lego），適用於你擁有的網域：上傳{ " " }
https-get-acme-and = { " " }和{ " " }
https-get-acme-after = 。
https-get-proxy-title = 反向代理
https-get-proxy = （Caddy、Traefik、nginx、Cloudflare Tunnel）持有憑證時：請在這裡保持 HTTPS 關閉，讓代理以一般的 http:// 傳送給 Hoard。
https-trying = 嘗試中…
https-apply = 套用
https-apply-hint = Hoard 會先試用新的設定，若無法運作就保留原本的設定。
https-trust-title = 在你的裝置上信任 Hoard
https-download-ca = 下載 hoard-ca.crt
https-trust-hint = 在每台裝置上安裝一次 Hoard 的憑證授權單位，裝置上的瀏覽器就會信任這台伺服器（以及 Hoard 之後為它建立的任何憑證）。它只能為這個 Hoard 擔保。指紋（SHA-256）：
https-ca-windows = 開啟檔案 → 安裝憑證 → 目前使用者 → 將所有憑證放入以下的存放區 → 瀏覽 → 受信任的根憑證授權單位 → 完成。然後重新啟動瀏覽器。
https-ca-macos = 開啟檔案，它會加入「鑰匙圈存取」。在那裡按兩下「{ $subject }」→ 信任 → 使用此憑證時：永遠信任。
https-ca-ios-title = iPhone 與 iPad
https-ca-ios = 在 Safari 中下載並允許。設定 → 已下載描述檔 → 安裝。接著到 設定 → 一般 → 關於本機 → 憑證信任設定 → 開啟「{ $subject }」。
https-ca-android = 設定 → 安全性 → 更多安全性設定 → 加密與憑證 → 安裝憑證 → CA 憑證，然後選擇檔案。（各手機的名稱略有不同；可在設定中搜尋「憑證」。）
https-ca-linux-1 = Debian 與 Ubuntu：{ " " }
https-ca-linux-2 = 。Fedora 與 Arch：{ " " }
https-ca-linux-3 = 。Linux 上的 Chrome 有自己的清單：設定 → 隱私權和安全性 → 安全性 → 管理憑證 → 匯入。
https-ca-firefox = Firefox 有自己的清單：設定 → 隱私權與安全性 → 憑證 → 檢視憑證 → 憑證機構 → 匯入，並勾選「信任由此憑證機構來識別網站」。
https-ca-cli-title = hoard 命令列工具
https-ca-cli-before = { "" }
https-ca-cli-middle = ，或將路徑設定在{ " " }
https-ca-cli-after = { " " }中。
https-options = 選項
https-redirect = 將其他裝置上的瀏覽器從 http:// 導向 https://
https-redirect-hint = 只有頁面載入會被導向。hoard 命令列工具、Unreal 工作程序、指令碼與 AI 應用程式仍可透過 http:// 運作，伺服器本機上的 http://localhost 也一樣。
https-http3-hint-before = 在不穩定的 Wi-Fi 與手機上更快。需要 HTTPS，且除了 TCP 之外也要開啟 UDP 連接埠 { $port }（在 Docker 中：{ " " }
https-http3-hint-after = ）。
https-http3-running = 執行中。
https-http3-waits = 會在 HTTPS 開啟時啟動。
https-http3-failed = HTTP/3 無法啟動：{ $error }
https-lockout-title = 如果你被鎖在外面
https-lockout-1 = { "" }
https-lockout-2 = { " " }在伺服器本機上一律可以透過一般的 http:// 使用。或者從伺服器的命令列關閉 HTTPS，然後重新啟動 Hoard：{ " " }
https-lockout-3 = { " " }（在 Docker 中：{ " " }
https-lockout-4 = ，然後{ " " }
https-lockout-5 = ）。

## 帳號面板（設定 → 帳號）

account-role-view = 可檢視
account-role-view-hint = 搜尋、預覽與下載
account-role-edit = 可修改
account-role-edit-hint = 另外還能新增、修改、加上標籤、加入收藏集與封存
account-role-admin = 管理員
account-role-admin-hint = 另外還能使用設定、管理帳號與維護，以及永久刪除
account-source-internal = Hoard 密碼
account-source-ldap = 他們的 { $directory } 密碼
account-source-oauth = 使用 { $provider } 登入
account-source-word-directory = 目錄
account-load-failed = 無法載入帳號：{ $error }
account-failed = { $what }：{ $error }
account-password-failed = 無法變更你的密碼
account-password-changed = 密碼已變更。你的其他裝置都已登出。
account-add-failed = 無法新增 { $name }
account-added = { $name } 現在可以登入了。
account-role-failed = 無法變更 { $name } 的權限
account-remove-title = 要移除 { $name } 嗎？
account-remove-body = 對方會在所有地方登出，且無法再次登入。對方新增到素材庫的內容都不會移除。
account-remove = 移除
account-remove-failed = 無法移除 { $name }
account-token-failed = 無法建立權杖
account-copied = 已複製
account-copy-token-failed = 無法複製：請自行選取權杖並複製
account-copy-failed = 無法複製：請自行選取並複製
account-revoke-title = 要撤銷「{ $name }」嗎？
account-revoke-body = 從現在起，任何仍在使用它的程式都會被拒絕。
account-revoke = 撤銷
account-revoke-failed = 無法撤銷「{ $name }」
account-disconnect-title = 要中斷 { $app } 的連線嗎？
account-disconnect-body = 從現在起它會被拒絕，必須重新連線才能使用 Hoard。
account-disconnect = 中斷連線
account-disconnect-failed = 無法中斷 { $app } 的連線
account-guest-failed = 無法變更這項設定
account-never = 從未
account-you = 你
account-sign-out = 登出
account-signed-in-as-before = 已登入為{ " " }
account-signed-in-as-admin = ，身分為管理員。
account-signed-in-as-edit = ，可以修改素材庫。
account-signed-in-as-view = ，可以檢視但無法修改任何內容。
account-app-signin = Hoard 應用程式會自動為你登入，因此沒有密碼。可在下方為引擎外掛和 hoard 指令建立存取權杖。
account-password-at-provider = 你的密碼由你登入時使用的提供者保管，因此請到那裡變更。
account-password-at-directory = 你的密碼由你登入時使用的目錄保管，因此請到那裡變更。
account-current-password = 目前的密碼
account-new-password = 新密碼
account-new-password-again = 再次輸入新密碼
account-change-password = 變更密碼
account-password-mismatch = 兩次輸入的新密碼不一致。
account-password-rule = 至少 8 個字元。
account-people = 使用者
account-people-hint = 每位使用者可以檢視（搜尋、預覽與下載）、也可以修改素材庫（新增、編輯、加上標籤、加入收藏集與封存），或同時是管理員（設定、帳號與維護，以及永久刪除）。
account-you-tag = （你）
account-signs-in-oauth = 透過 OAuth 提供者登入
account-signs-in-directory = 透過目錄登入
account-since = 自 { $date } 起
account-what-can-do = { $name } 的權限
account-name = 名稱
account-how-sign-in = 登入方式
account-password-label = 密碼（至少 8 個字元）
account-password-placeholder = 密碼（8 個字元以上）
account-what-they-can-do = 權限
account-add = 新增
account-ai-apps = Claude 與其他 AI 應用程式
account-ai-apps-hint = 連接 Claude（或任何 MCP 應用程式）後，它就能為你搜尋與取得素材，並為 Hoard 尚未處理的內容撰寫描述與加上標籤。它第一次會登入 Hoard，由你在這裡的瀏覽器中允許。
account-mcp-address = Hoard 的 MCP 網址
account-copy = 複製
account-claude-code-1 = 執行{ " " }
account-claude-code-2 = ，然後用{ " " }
account-claude-code-3 = { " " }登入。{ " " }
account-claude-code-4 = { " " }會描述一個素材包。
account-claude-ai = 設定 → 連接器 → 新增自訂連接器，貼上網址，然後按「連接」。這些應用程式從 Anthropic 的伺服器連線，因此 Hoard 必須能透過 HTTPS 從網際網路存取。
account-claude-ai-local = 設定 → 連接器 → 新增自訂連接器，貼上網址，然後按「連接」。這些應用程式從 Anthropic 的伺服器連線，因此 Hoard 必須能透過 HTTPS 從網際網路存取（這個網址無法存取：請在「設定 → 連線」中設定公開網址）。
account-connection-meta = 使用者 { $user } · 自 { $since } 起 · 上次使用 { $used }
account-no-apps = 還沒有連接任何應用程式。
account-live = 正在使用 Hoard
account-live-hint = 最近 10 分鐘內向 Hoard 提出過請求的應用程式。清單會自動更新。
account-live-none = 最近 10 分鐘內沒有任何應用程式向 Hoard 提出請求。
account-live-last = 最近：{ $what } · { $when } · 請求數：{ $calls }
account-tokens = 存取權杖
account-tokens-hint-1 = 供{ " " }
account-tokens-hint-2 = { " " }命令列工具、指令碼、MCP 用戶端與 Unreal 工作程序使用：它們會以{ " " }
account-tokens-hint-3 = { " " }的形式傳送。權杖可以做你選擇的事，最多到你自己的權限，但絕不能管理帳號或權杖。只能檢視的權杖就足以搜尋、下載以及同步專案。
account-tokens-admin = 你可以看到所有人的權杖與已連接的應用程式；其他人只能看到自己的。
account-made-before = { "" }
account-made-after = ：請立即複製。之後不會再顯示。
account-new-token = 新的權杖
account-cli-1 = CLI：{ " " }
account-cli-2 = ，或將它設定在{ " " }
account-cli-3 = { " " }中。
account-done = 完成
account-token-meta = 建立於 { $made } · 上次使用 { $used }
account-token-meta-by = 由 { $by } 建立於 { $made } · 上次使用 { $used }
account-no-tokens = 還沒有任何權杖。
account-token-name = 使用它的程式
account-token-name-placeholder = 使用它的程式，例如「Unreal 工作程序」
account-token-role = 權限
account-make-token = 建立權杖
account-guests = 訪客
account-guests-allow = 讓任何能連線到伺服器的人不需登入即可瀏覽
account-guests-hint = 他們可以搜尋、預覽與下載，但無法新增、修改或刪除任何內容，也無法查看設定。

## 登入

signin-create-title = 建立你的帳號
signin-title = 登入
signin-setup-lede = 這台伺服器還沒有任何帳號。你在這裡建立的帳號可以做任何事；之後可以在「設定 → 帳號」中為其他人新增帳號。
signin-overlay-lede = 這裡開放所有人瀏覽，但進行變更需要帳號。
signin-with = 使用 { $provider } 登入
signin-or-password = 或使用名稱與密碼
signin-username = 使用者名稱
signin-name = 名稱
signin-new-password = 新密碼
signin-password = 密碼
signin-password-rule = 至少 8 個字元。
signin-confirm-password = 確認密碼
signin-mismatch = 兩次輸入的密碼不一致。
signin-setup-code = 設定碼
signin-setup-code-hint-before = 這台伺服器在{ " " }
signin-setup-code-hint-after = { " " }中取得的代碼。
signin-creating = 建立中…
signin-signing-in = 登入中…
signin-create = 建立帳號
signin-submit = 登入
signin-keep-browsing = 繼續瀏覽
signin-directory-hint = 請使用你的 { $directory } 名稱與密碼。
signin-forgot-1 = 忘記 Hoard 密碼了嗎？請在伺服器上執行{ " " }
signin-forgot-2 = { " " }（在 Docker 中為{ " " }
signin-forgot-3 = ）以設定新密碼。

## 設定精靈（首次造訪新伺服器）

setup-title = 你的素材庫要放在哪裡？
setup-lede = 在伺服器上選擇一個資料夾，讓 Hoard 存放你的素材包，或指向你已經有素材包的資料夾。之後可以在設定中新增其他磁碟上的位置。
setup-lede-desktop = 選擇一個資料夾，讓 Hoard 存放你的素材包，或指向你已經有素材包的資料夾。之後可以在設定中新增其他磁碟上的位置。
setup-unreachable = 無法連線到伺服器：{ $error }
setup-unreachable-desktop = Hoard 沒有回應：{ $error }
setup-try-again = 再試一次
setup-loading = 載入中…
setup-start = 建立素材庫
setup-ready = 你的素材庫已就緒
setup-managed-before = 新的素材包會放入{ " " }
setup-managed-after = 。將檔案、資料夾或壓縮檔拖放到頁面上的任何地方，即可新增第一批素材包。
setup-watched-before = Hoard 正在監看{ " " }
setup-watched-after = { " " }；找到的素材包會陸續出現。若也要上傳，請在「設定 → 素材庫位置」中新增一個由 Hoard 保管的素材庫。
setup-open = 開啟素材庫

## 連接應用程式

connect-title = 要連接 { $app } 嗎？
connect-lede-1 = { " " }將能搜尋、下載、描述並標記這個 Hoard 中的所有內容，並以{ " " }
connect-lede-2 = { " " }的身分執行。它無法管理帳號或存取權杖。
connect-back-before = 你將返回{ " " }
connect-back-after = 。你隨時可以在「設定 → 帳號」中中斷它的連線。
connect-allow = 允許
connect-cancel = 取消
connect-failed = 無法連接這個應用程式
connect-go-library = 前往素材庫
connect-checking = 檢查中…

## 不安全連線提示

insecure-text = 這是一般的 http://，因此有幾項功能需要 HTTPS：使用 WebGPU 的 3D、新式剪貼簿、將 Hoard 安裝為應用程式，以及 Claude 連接器。
insecure-switch = 切換到 https://
insecure-set-up = 設定 HTTPS
insecure-dismiss = 不要再顯示

## 格式精靈（設定 → 商店檔案 → 教 Hoard 讀取商店的檔案）

formatwizard-too-big = 這個檔案太大，無法當作商店的檔案讀取。
formatwizard-not-json = 這個檔案不是有效的 JSON，因此以文字顯示。
formatwizard-read-failed = 無法讀取檔案：{ $error }
formatwizard-back-title = 要返回表單嗎？
formatwizard-back-body = 在 TOML 中所做的變更會遺失：表單會根據它現有的內容重新寫出格式。
formatwizard-back-to-form = 返回表單
formatwizard-miss-name = 它的檔案不包括名稱 { $file }。請將它加入「檔案」。
formatwizard-miss-not-the-stores = 這個檔案中沒有它用來辨識商店檔案的內容。請變更「辨識依據」。
formatwizard-miss-no-title = 它在這個檔案中找不到標題。請選擇標題。
formatwizard-miss-short-name = 名稱不符
formatwizard-miss-short-not-the-stores = 不是該商店的
formatwizard-miss-short-no-title = 沒有標題
formatwizard-saved-packs = 已儲存 { $file }。正在重新讀取 { $packs } 個素材包。
formatwizard-saved = 已儲存 { $file }。
formatwizard-save-failed = 無法儲存格式：{ $error }
formatwizard-zip-failed = 無法建立 zip：{ $error }
formatwizard-stop-title = 要停止教 Hoard 讀取這個檔案嗎？
formatwizard-stop-body = 格式不會儲存。
formatwizard-stop = 停止
formatwizard-label = 格式精靈
formatwizard-changing = 正在修改 { $name }
formatwizard-starting-from = 以 { $name } 為起點
formatwizard-title = 教 Hoard 讀取商店的檔案
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }位於{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = 以 TOML 編輯
formatwizard-file-reference = 檔案內容，供參考。
formatwizard-click-json = 在檔案中點選{ $field }。
formatwizard-click-text = 在檔案中點選或選取{ $field }。
formatwizard-pick-first = 先在右側選擇欄位，然後在這裡點選它的值。
formatwizard-loading = 載入中…
formatwizard-the-format = 格式
formatwizard-syntax-before = 語法說明請見{ " " }
formatwizard-syntax-after = 。
formatwizard-store = 商店
formatwizard-store-placeholder = 商店名稱，例如 Kenney
formatwizard-files = 檔案
formatwizard-recognize = 辨識依據
formatwizard-recognize-json = 只有該商店的檔案才有的鍵
formatwizard-recognize-text = 只有該商店的檔案才有的一行文字
formatwizard-recognize-hint = 每行一種方式；符合任何一種即可。當其他檔案也有相同名稱時需要填寫。
formatwizard-seller = 賣家
formatwizard-seller-placeholder = 檔案中沒有註明時使用
formatwizard-tags = 標籤
formatwizard-tags-placeholder = 套用到它描述的所有內容，以逗號分隔
formatwizard-fields = 欄位
formatwizard-selector = { $field }選擇器
formatwizard-clear = 清除{ $field }
formatwizard-pick = 選擇
formatwizard-what-it-reads = 讀取結果
formatwizard-read-title = 標題
formatwizard-read-seller = 賣家
formatwizard-read-license = 授權
formatwizard-read-url = 商店頁面
formatwizard-read-kind = 類型
formatwizard-read-tags = 標籤
formatwizard-read-origin = 製作工具
formatwizard-read-size = 大小
formatwizard-also = { $format } 也會讀取這個檔案。這個格式會優先嘗試。
formatwizard-library = { $packs } 個素材包中有 { $files } 個檔案的名稱符合。在嘗試的 { $tried } 個中，可讀取 { $read } 個。
formatwizard-library-none = 素材庫中沒有任何檔案的名稱符合。
formatwizard-share = 分享
formatwizard-share-hint = 一個 zip 檔，內含格式、以這個檔案作為範例，以及範例的讀取結果，並依照 Hoard 儲存庫的結構排列，送出後就能讓每個人都使用。請先移除範例中不應分享的內容。
formatwizard-sample = 要分享的範例
formatwizard-download = 下載 { $file }
formatwizard-close = 關閉
formatwizard-use = 在這裡使用
formatwizard-cancel = 取消
settings-unreal-editor-what = 為匯出工作程序安裝 Unreal Editor。請另外執行該程序以匯出資源。
settings-unreal-download = 下載 Epic 的 Linux x86-64 編輯器。需要已連結的 GitHub 存取權限及數十 GB 的磁碟空間。
settings-unreal-image = Unreal 版本或映像檔
settings-editor-path = 輸入已安裝的 { $tool } 執行檔路徑。僅在 Docker 中支援自動安裝。
error-unreal-image = 請輸入 Unreal 5 版本、Epic dev/dev-slim 版本標籤或 Epic SHA-256 映像檔參照。
