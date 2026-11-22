## 서버 오류

error-account-not-found = 계정을 찾을 수 없습니다
error-asset-gone = 에셋을 찾을 수 없습니다
error-asset-not-found = 에셋 { $id }을(를) 찾을 수 없습니다
error-blob-not-found = blob { $hash }을(를) 찾을 수 없습니다
error-collection-not-found = 컬렉션 { $id }을(를) 찾을 수 없습니다
error-connection-not-found = 연결을 찾을 수 없습니다
error-derivative-not-found = { $id }의 { $kind } 파생 파일을 찾을 수 없습니다
error-file-in-pack-not-found = 팩 { $id }에서 { $path }을(를) 찾을 수 없습니다
error-site-not-a-page = 웹 페이지만 페이지로 열립니다
error-site-link-expired = 이 페이지의 주소가 만료되었습니다. Hoard에서 페이지를 다시 여세요
error-format-not-found = 형식 { $file }을(를) 찾을 수 없습니다
error-frame-sheet-aseprite-only = 프레임 시트는 Aseprite 파일에만 있습니다
error-frame-sheet-frames = 프레임이 하나뿐이거나 너무 많아서 배치할 수 없습니다
error-location-not-found = 위치 { $id }을(를) 찾을 수 없습니다
error-maintenance-task-not-found = 유지 관리 작업 { $id }을(를) 찾을 수 없습니다
error-open-upload-not-found = 열린 업로드 { $id }을(를) 찾을 수 없습니다
error-original-not-found = { $id }의 원본 파일을 찾을 수 없습니다
error-pack-not-found = 팩 { $id }을(를) 찾을 수 없습니다
error-preview-too-big = 이미지가 너무 커서 가벼운 미리보기를 만들 수 없습니다
error-project-not-found = 프로젝트 { $name }을(를) 찾을 수 없습니다
error-saved-search-not-found = 저장된 검색 { $id }을(를) 찾을 수 없습니다
error-sign-in-first = 먼저 로그인하세요(또는 액세스 토큰을 보내세요)
error-those-assets-not-found = 해당 에셋을 찾을 수 없습니다
error-thumbnail-not-found = { $id }의 썸네일을 찾을 수 없습니다
error-token-not-found = 토큰을 찾을 수 없습니다
error-upload-not-found = 업로드 { $id }을(를) 찾을 수 없습니다
error-pack-needs-name = 팩에는 이름이 필요합니다
error-no-library = 아직 파일을 저장할 라이브러리 위치가 없습니다. 설정 → 라이브러리 위치에서 관리 위치를 추가하세요
error-upload-failed = 업로드 실패: { $reason }
error-upload-no-files = 업로드에 파일이 없습니다
error-analyze-nothing-to-do = 할 일이 없습니다: rules=false이고 LLM이 꺼져 있습니다
error-too-many-assets = 한 번의 요청에 에셋이 너무 많습니다
error-change-not-found = 그 변경 사항은 이 에셋의 기록에 없습니다
error-bulk-by-look = 모양으로 찾은 결과는 한꺼번에 편집할 수 없습니다. 대신 에셋을 선택하세요
error-too-many-items = 한 번의 요청에 항목이 너무 많습니다
error-not-a-hash = blake3 16진수 해시가 아닙니다
error-merge-into-tag = into는 태그여야 합니다
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = "{ $text }"은(는) 태그가 아닙니다
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent }이(가) 이미 { $tag } 아래에 있어서 { $tag }을(를) 그 아래에 넣을 수 없습니다
error-nothing-to-merge = 병합할 것이 없습니다
error-selection-size = 에셋 ID를 1개에서 { $max }개 사이로 지정하세요
error-collection-needs-name = 컬렉션에는 이름이 필요합니다
error-collection-exists = 이름이 { $name }인 컬렉션이 이미 있습니다
error-collection-empty = 컬렉션 { $name }이(가) 비어 있습니다
error-conversion-failed = { $format }(으)로 변환할 수 없습니다: { $reason }
error-max-size = max는 64에서 16384 사이의 2의 거듭제곱이어야 합니다
error-replace-ext = ext는 짧은 파일 확장자여야 합니다
error-empty-file = 빈 파일입니다
error-imitation-silent = 녹음에서 배경 소음보다 두드러지는 소리가 없습니다. 마이크에 더 가까이 대고 다시 해 보세요.
error-replace-duplicate = 변환된 파일이 팩에 이미 있는 파일과 같아서 패키지를 그 파일에 병합했습니다
error-ai-address = AI 주소는 http:// 또는 https://로 시작해야 합니다
error-hosted-key = { $provider } API 키를 입력하세요
error-hosted-address = 이미지 검색 서비스 주소는 http:// 또는 https://로 시작해야 합니다
error-local-llm-action = action은 list, download, delete 중 하나이며, 뒤의 두 가지에는 모델이 필요합니다
error-role = 역할은 view, edit, admin 중 하나입니다
error-admin-only = 관리자만 할 수 있습니다
error-view-only = 이 계정은 보기만 할 수 있고 아무것도 바꿀 수 없습니다. 관리자가 설정 → 계정에서 바꿀 수 있습니다
error-portal-only = 계정과 토큰을 관리하려면 포털에 로그인하세요
error-not-from-portal = 이 요청은 Hoard 포털에서 온 것이 아닙니다
error-account-name = 64자 이하의 이름을 정하세요
error-password-length = { $min }자 이상의 비밀번호를 사용하세요
error-too-many-codes = 잘못된 코드를 너무 많이 입력했습니다. { $seconds }초 후에 다시 시도하세요
error-already-set-up = 이미 계정이 있습니다. 로그인하세요
error-too-many-tries = 잘못된 시도가 너무 많습니다. { $seconds }초 후에 다시 시도하세요
error-directory-unavailable = 디렉터리에서 비밀번호를 확인할 수 없었습니다: { $reason }
error-password-elsewhere = 비밀번호는 로그인에 사용하는 디렉터리나 제공자가 관리합니다
error-current-password = 현재 비밀번호가 올바르지 않습니다
error-account-source = 계정은 Hoard(internal), 디렉터리(ldap) 또는 OAuth로 로그인합니다
error-account-exists = 같은 이름의 계정이 이미 있습니다
error-own-role = 내 역할은 다른 관리자가 바꿀 수 있습니다
error-remove-self = 로그인한 상태에서는 내 계정을 삭제할 수 없습니다
error-token-name = 토큰을 사용할 대상의 이름을 80자 이내로 붙이세요
error-token-role = 토큰은 토큰을 만든 계정보다 많은 일을 할 수 없습니다
error-wrong-setup-code = HOARD_SETUP_CODE로 서버에 지정한 설정 코드가 아닙니다
error-wrong-password = 이름과 비밀번호가 일치하지 않습니다
error-folder-missing = { $path }이(가) 없습니다(마운트되지 않은 디스크인가요?)
error-s3-path = S3 위치는 s3://bucket 형태입니다
error-s3-address = S3 주소는 http:// 또는 https://로 시작해야 합니다
error-s3-header = 추가 S3 헤더는 Name=value 형태인데, “{ $header }”은(는) 그렇지 않습니다
error-keychain = 시스템 키체인에 보관하지 못했습니다: { $reason }
error-not-built = 이 Hoard는 { $feature } 없이 빌드되었습니다
error-openai-auth-unreachable = auth.openai.com에 연결할 수 없습니다
error-openai-unexpected = OpenAI에서 예상치 못한 응답이 왔습니다
error-openai-answered = OpenAI 응답 { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID 토큰이 JWT가 아닙니다
error-chatgpt-id-token-issuer = ID 토큰의 발급자가 { $expected }이(가) 아니라 { $issuer }입니다
error-chatgpt-id-token-audience = ID 토큰이 다른 앱용입니다
error-chatgpt-id-token-expired = ID 토큰이 만료되었습니다
error-chatgpt-id-token-nonce = ID 토큰이 이 로그인에 속하지 않습니다
error-chatgpt-no-state = 주소에 state가 없습니다. 주소 전체를 복사하세요
error-chatgpt-not-started = 이 로그인은 여기서 시작되지 않았거나 시작한 지 15분이 넘었습니다. 설정에서 다시 시작하세요.
error-chatgpt-canceled = 로그인이 취소되었습니다
error-chatgpt-refused = ChatGPT가 로그인을 거부했습니다: { $reason }
error-chatgpt-no-code = 주소에 로그인 코드가 없습니다. 주소 전체를 복사하세요
error-chatgpt-other-registration = ChatGPT가 이 서버의 다른 등록에 대해 응답했습니다. 로그인을 다시 시작하세요
error-chatgpt-other-registration-desktop = ChatGPT가 다른 Hoard 사본에 대해 응답했습니다. 로그인을 다시 시작하세요
error-chatgpt-not-registered = ChatGPT가 Hoard 등록을 끝내지 못했습니다. 로그인을 다시 시작하세요
error-chatgpt-code-used = 로그인 코드가 이미 사용되었거나 만료되었습니다. 다시 로그인하세요
error-chatgpt-refused-code = ChatGPT가 로그인을 거부했습니다({ $code })
error-chatgpt-no-id-token = ChatGPT가 ID 토큰을 보내지 않았습니다
error-chatgpt-plan-not-allowed = 로그인은 되었지만 요금제 사용이 허용되지 않았습니다. 다시 로그인해서 허용하거나 API 키를 대신 사용하세요.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = ChatGPT에 로그인되어 있지 않습니다. 설정 → AI 태깅에서 로그인하세요
error-chatgpt-refresh-later = ChatGPT가 지금은 로그인을 갱신할 수 없습니다({ $code }). 나중에 다시 시도합니다
error-chatgpt-signed-out-problem = ChatGPT가 이 서버를 로그아웃시켰습니다({ $code }). 요금제로 계속 태깅하려면 다시 로그인하세요.
error-chatgpt-signed-out-problem-desktop = ChatGPT가 Hoard를 로그아웃시켰습니다({ $code }). 요금제로 계속 태깅하려면 다시 로그인하세요.
error-chatgpt-signed-out = ChatGPT가 이 서버를 로그아웃시켰습니다({ $code }). 설정 → AI 태깅에서 다시 로그인하세요
error-chatgpt-signed-out-desktop = ChatGPT가 Hoard를 로그아웃시켰습니다({ $code }). 설정 → AI 태깅에서 다시 로그인하세요
error-openai-api-unreachable = api.openai.com에 연결할 수 없습니다
error-openai-broke-off = OpenAI의 응답이 중간에 끊겼습니다
error-chatgpt-kept-refusing = OpenAI가 ChatGPT 로그인을 계속 거부합니다. 설정 → AI 태깅에서 다시 로그인하세요
error-chatgpt-request-failed = 요청이 실패했습니다
error-chatgpt-cut-off = 응답이 잘렸습니다({ $reason })
error-chatgpt-cut-off-no-reason = 응답이 잘렸습니다(이유 없음)
error-openai-ended-early = OpenAI의 응답이 완료되기 전에 끝났습니다
error-chatgpt-usage-limit = ChatGPT 요금제의 Hoard 사용량을 당분간 모두 사용했습니다. 한도는 { $url }에서 확인하거나 바꿀 수 있습니다
error-chatgpt-not-eligible = 이 ChatGPT 계정 또는 워크스페이스는 다른 앱에서 요금제를 사용할 수 없습니다. API 키를 대신 사용하세요.
error-chatgpt-usage-unavailable = ChatGPT가 지금은 요금제 사용량을 확인할 수 없습니다. 나중에 다시 시도합니다
error-chatgpt-unsupported = ChatGPT 요금제 사용 시 이 모델에서는 { $param }을(를) 지원하지 않습니다
error-chatgpt-unsupported-images = ChatGPT 요금제 사용 시 이 모델에서는 { $param }을(를) 지원하지 않습니다. 이미지를 받는 모델을 고르거나 썸네일 보내기를 끄세요
error-chatgpt-unsupported-part = ChatGPT 요금제 사용 시 이 모델에서는 요청의 일부를 지원하지 않습니다
error-chatgpt-port = port는 브라우저가 Hoard에 접속하는 포트입니다
error-chatgpt-paste-address = http://127.0.0.1로 시작하는 주소 전체를 붙여 넣으세요
error-too-many-hashes = 한 번에 최대 { $max }개의 해시만 가능합니다
error-credits-pack = 팩 { $key }
error-credits-collection = 컬렉션 { $key }
error-format-too-long = 형식이 너무 깁니다
error-not-started = 서버가 시작되지 않았습니다
error-not-started-desktop = Hoard가 시작되지 않았습니다
error-not-format-file = 형식 파일이 아닙니다: { $name }
error-format-name = 이름에는 A~Z의 문자나 0~9의 숫자가 하나 이상 있어야 합니다
error-format-miss-name = 이 형식의 파일은 { $file }을(를) 받지 않습니다
error-format-miss-store = 이 형식은 스토어용으로 { $file }을(를) 받지 않습니다(require 참고)
error-format-miss-title = 이 형식은 { $file }에서 제목을 찾지 못했습니다
error-cant-download-as = .{ $ext } 파일은 { $format }(으)로 다운로드할 수 없습니다
error-only-images-resize = 이미지만 크기를 조정할 수 있습니다
error-https-no-certificate = 먼저 인증서와 키를 붙여 넣거나 업로드하세요
error-https-no-cert-found = 인증서를 찾을 수 없습니다. -----BEGIN CERTIFICATE-----로 시작하는 텍스트를 붙여 넣으세요
error-https-no-key-found = 개인 키를 찾을 수 없습니다. -----BEGIN PRIVATE KEY-----로 시작하는 텍스트를 붙여 넣으세요
error-https-several-keys = 개인 키가 둘 이상 있습니다. 인증서와 짝이 맞는 키 하나만 붙여 넣으세요
error-https-key-password = 키가 비밀번호로 보호되어 있습니다. 비밀번호가 없는 키를 주세요(openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = 인증서가 올바른 PEM이 아닙니다
error-https-key-not-pem = 개인 키가 올바른 PEM이 아닙니다
error-https-key-mismatch = 키가 인증서와 짝이 맞지 않습니다. 체인을 넣었다면 서버 인증서를 맨 앞에 두세요
error-https-unusable = 인증서나 키를 사용할 수 없습니다
error-https-expired = 이 인증서는 { $date }에 만료되었습니다
error-https-not-yet-valid = 이 인증서는 { $date }부터 유효합니다
error-https-unreadable = Hoard가 읽을 수 있는 인증서가 아닙니다
error-https-bad-name = “{ $name }”은(는) 인증서에 넣을 수 있는 주소가 아닙니다
error-https-test-timed-out = 테스트 연결 시간이 초과되었습니다
error-https-test-failed = 테스트 연결에 실패했습니다
error-https-no-ca = Hoard가 아직 인증 기관을 만들지 않았습니다
error-https-unavailable = 이 서버에서는 HTTPS를 사용할 수 없습니다
error-https-certificate-mode = 인증서는 “내 인증서”와 함께 사용합니다
error-https-test = 테스트 연결에서 인증서가 작동하지 않았습니다: { $reason }
error-nothing-to-retry = 다시 시도할 수 있는 항목이 없습니다
error-ai-unreachable = { $url }에 연결할 수 없습니다
error-ai-key-refused = { $url }이(가) API 키를 거부했습니다({ $status })
error-ai-answered = { $url } 응답 { $status }: { $reason }
error-ai-not-json = { $url }/models가 JSON으로 응답하지 않았습니다
error-model-server-refused = 모델 서버: { $reason }
error-no-model-server = { $url }에서 응답하는 모델 서버가 없습니다
error-not-llama-router = { $url }은(는) 라우터 모드의 llama.cpp 서버가 아니어서 모델을 다운로드할 수 없습니다
error-model-download = { $model }을(를) 다운로드할 수 없습니다: { $reason }
error-model-delete = { $model }을(를) 삭제할 수 없습니다: { $reason }
error-unknown-kind = 알 수 없는 에셋 종류 { $kind }
error-watched-is-folder = 감시 위치는 이 서버에 있는 폴더입니다
error-cant-reach = { $path }에 연결할 수 없습니다: { $reason }
error-folder-full-path = 폴더의 전체 경로를 입력하세요
error-cant-create = { $path }을(를) 만들 수 없습니다
error-cant-write = { $path }에 쓸 수 없습니다
error-cant-read = { $path }을(를) 읽을 수 없습니다
error-overlaps-location = { $name } 위치({ $path })와 겹칩니다
error-overlaps-data = Hoard의 데이터 폴더({ $path })와 겹칩니다
error-overlaps-cache = Hoard의 캐시 폴더({ $path })와 겹칩니다
error-not-watched = { $name }은(는) 감시 폴더가 아닙니다
error-watched-no-kinds = 감시 폴더는 새 파일을 받지 않으므로 바꿀 종류가 없습니다
error-keep-catch-all = 대신 다른 위치를 기본 위치로 지정하세요
error-location-holds-files = { $name }에 아직 파일이 있습니다. 먼저 다른 위치로 옮기세요(POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name }에 연결할 수 없어서 Hoard가 아직 파일이 있는지 알 수 없습니다
error-relocate-running = 이미 위치 간에 파일을 옮기는 중입니다. 끝난 뒤에 다시 시도하세요
error-watched-not-moved = { $name }은(는) 감시 폴더입니다. Hoard는 감시 폴더의 파일을 옮기지 않습니다
error-no-other-location = 파일을 옮길 다른 라이브러리 위치가 없습니다. 먼저 하나 추가하세요
error-full-path = 전체 경로를 입력하세요
error-pick-folder = 폴더를 고르세요
error-task-running = “{ $task }” 작업이 이미 실행 중입니다
error-job-cant-stop = “{ $job }” 작업은 시작된 후에는 중지할 수 없습니다
error-job-not-found = 이 작업은 더 이상 대기 중이거나 실행 중이 아닙니다
error-task-cant-stop = “{ $task }” 작업은 도중에 중지할 수 없습니다
error-task-not-running = “{ $task }” 작업이 실행 중이 아닙니다
error-token-account-gone = 이 토큰의 계정이 없어져서 즐겨찾기나 검색이 없습니다
error-favorites-too-many = 한 번에 최대 10,000개까지 가능합니다
error-saved-search-name = 저장된 검색에는 80자 이하의 이름이 필요합니다
error-saved-search-too-long = 검색이 너무 길어서 저장할 수 없습니다
error-hoard-seen = 보물 더미에는 에셋 종류를 최대 32개까지 기록할 수 있으며, 각각 소문자 이름이어야 합니다
error-texture-pattern-map = 텍스처 이름 패턴에는 { "{" }map{ "}" }이 있어야 합니다. 그렇지 않으면 머티리얼의 모든 맵이 같은 이름을 갖게 됩니다
error-texture-pattern-fields = 텍스처 이름 패턴에는 { "{" }name{ "}" }, { "{" }map{ "}" }, { "{" }size{ "}" }를 사용할 수 있습니다
error-texture-name-chars = 텍스처 이름에는 < > : " / \ | ? * 를 쓸 수 없습니다
error-texture-map-type = { $role }(이)라는 맵 종류가 없습니다
server-role-base-color = 색상
server-role-normal = 노멀
server-role-normal-dx = 노멀(DirectX)
server-role-roughness = 러프니스
server-role-gloss = 글로스
server-role-metalness = 메탈니스
server-role-ao = 앰비언트 오클루전
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = 높이
server-role-bump = 범프
server-role-cavity = 캐비티
server-role-specular = 스페큘러
server-role-opacity = 불투명도
server-role-emissive = 이미시브
server-role-translucency = 반투명
error-texture-suffix-missing = { $map } 맵에 접미사를 지정하세요
error-texture-suffix-chars = { $map } 맵의 접미사에는 < > : " / \ | ? * 를 쓸 수 없습니다
error-texture-ending-missing = 직접 지정한 끝부분마다 찾을 끝부분을 입력해야 합니다
error-texture-ending-chars = 직접 지정한 끝부분에는 < > : " / \ | ? * 를 쓸 수 없습니다
error-texture-ending-twice = { $ending }이(가) 끝부분 목록에 두 번 있습니다
error-oauth-no-client-id = 이 링크에 client_id가 없습니다.
error-oauth-unknown-app = 이 앱은 여기에 등록되어 있지 않습니다(더 이상). 앱에서 연결을 다시 시작하세요.
error-oauth-redirect = 앱이 등록하지 않은 주소로 돌려보내 달라고 요청했습니다.
error-oauth-response-type = Hoard는 로그인 코드만 발급합니다(response_type=code).
error-oauth-pkce = Hoard에 로그인하려면 앱이 S256 방식의 PKCE를 사용해야 합니다.
error-light-preview-images = 가벼운 미리보기는 이미지에만 있습니다
error-no-light-preview = 가벼운 미리보기 없음: { $reason }
error-no-frame-sheet = 프레임 시트 없음: { $reason }
error-thumbnail-source = 알 수 없는 썸네일 소스: { $source }
error-thumbnail-too-big = 그림이 너무 커서 썸네일로 쓸 수 없습니다
error-thumbnail-models-only = 브라우저가 그린 썸네일은 모델에만 쓸 수 있습니다
error-not-a-picture = Hoard가 읽을 수 있는 그림이 아닙니다: { $reason }
error-project-name = 프로젝트에는 200자 이하의 이름이 필요합니다
error-project-too-big = 한 프로젝트에는 에셋을 최대 100만 개까지 넣을 수 있습니다
error-project-too-many-at-once = 한 번에 최대 100,000개까지 가능합니다
error-server-name = 한 줄로 된 { $max }자 이하의 이름을 정하세요
error-tool-setting = { $tool }: 끄기, 자동 중에서 고르거나 경로를 입력하세요
error-ai-model = AI 서비스가 사용할 모델을 고르세요
error-public-address = 공개 주소는 http:// 또는 https://로 시작해야 합니다
error-update-hook-address = 업데이트 시 호출할 주소는 http:// 또는 https://로 시작해야 합니다
error-update-hook-auth = Authorization 헤더에는 줄바꿈이나 다른 제어 문자를 넣을 수 없습니다
error-no-soundfont = 서버의 { $path }에 SoundFont가 없습니다
error-no-soundfont-desktop = { $path }에 SoundFont가 없습니다
error-soundfont-bad = { $name }은(는) MIDI 재생에 쓸 수 있는 SoundFont가 아닙니다: { $reason }
error-workers = 백그라운드 작업자: 1에서 { $max } 사이
error-ai-batch = AI 요청당 에셋 수: 1에서 200 사이
error-conversion-cache = 변환된 다운로드: 100 MB 이상
error-backup-keep = 예약 백업: 1~{ $max }개를 보관할 수 있습니다
error-backup-copy-to = 예약 백업: { $path }은(는) 서버의 폴더가 아닙니다(전체 경로를 입력하세요)
error-conversion-keep-days = 변환 규칙: 원본은 1~{ $max }일 동안 보관할 수 있습니다
error-conversion-target = 변환 규칙: Hoard는 파일을 { $format }(으)로 변환할 수 없습니다
error-conversion-no-source = 변환 규칙: 각 규칙에는 변환할 형식이 하나 이상 있어야 합니다
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = 변환 규칙: .{ $ext } 파일은 { $format }(으)로 변환할 수 없습니다
error-conversion-twice = 변환 규칙: .{ $ext }이(가) 두 규칙에 들어 있습니다
error-conversion-cant-undo = 이 파일은 규칙으로 변환된 파일이 아니거나 원본이 삭제되었습니다.
error-conversion-path-taken = 지금 { $path }에 다른 파일이 있어서 원본을 그곳으로 되돌릴 수 없습니다.
error-limits-zero = 한도는 0일 수 없습니다
error-settings-object = 설정의 JSON 객체가 필요합니다
error-bad-setting = 잘못된 설정: { $reason }
error-auth-default-role = 기본 역할은 view, edit, admin 중 하나입니다
error-auth-session-hours = 세션은 1시간에서 1년 사이로 유지됩니다
error-auth-settings = 잘못된 인증 설정: { $reason }
error-ldap-settings = 잘못된 LDAP 설정: { $reason }
error-ldap-no-server = 디렉터리 서버의 주소를 추가하세요
error-ldap-base-dn = 사용자 항목이 있는 위치(base DN)를 지정하세요
error-ldap-username-attribute = 로그인에 쓰는 이름이 들어 있는 속성을 지정하세요
error-ldap-bind-dn = 사용자를 조회할 계정을 지정하거나 익명으로 조회하세요
error-ldap-ca-cert = 인증 기관이 PEM 인증서(-----BEGIN CERTIFICATE-----)가 아닙니다
error-ldap-empty-server = 디렉터리 서버의 주소가 비어 있습니다
error-ldap-server-form = { $server }: 디렉터리 서버의 주소는 host:port, ldap://… 또는 ldaps://… 형태입니다
error-ldap-not-starttls = { $server }: StartTLS가 아니라 LDAPS입니다
error-ldap-no-certificate = 인증서가 들어 있지 않습니다
error-ldap-password-again = 사용자를 조회하는 계정의 비밀번호를 다시 입력하세요. 저장된 비밀번호는 원래 지정된 서버에만 보냅니다
error-ldap-unreachable = 연결할 수 있는 디렉터리 서버가 없습니다({ $reasons })
error-ldap-no-anonymous = 디렉터리에서 익명 조회를 허용하지 않습니다({ $reason })
error-ldap-bind-failed = 사용자를 조회하는 계정이 로그인하지 못했습니다({ $reason })
error-ldap-search = { $base } 아래에서 { $name } 찾는 중
error-ldap-group-search = { $base } 아래에서 { $name }의 그룹 찾는 중
error-ldap-several = 디렉터리에서 { $name }과(와) 일치하는 항목이 { $count }개입니다. 사용자 필터를 좁히세요
error-ldap-password-check = 디렉터리가 { $name }의 비밀번호를 확인하지 않았습니다({ $reason })
error-ldap-wrong-password = 디렉터리에 그 비밀번호를 쓰는 { $name }이(가) 없습니다
error-oauth-client-id = 제공자가 Hoard용으로 만든 클라이언트 ID를 입력하세요
error-oauth-addresses = 인증 주소와 토큰 주소는 https://로 시작해야 합니다
error-oauth-userinfo-address = 사용자 정보 주소는 https://로 시작해야 합니다
error-oauth-username-claim = 사용자 이름이 들어 있는 클레임을 지정하세요
error-oauth-secret-again = 클라이언트 시크릿을 다시 입력하세요. 저장된 시크릿은 원래 지정된 토큰 주소에만 보냅니다
error-oauth-issuer = 발급자는 https://로 시작해야 합니다
error-oauth-discovery-unreachable = { $url }에 연결할 수 없습니다: { $reason }
error-oauth-discovery-status = { $url } 응답: { $status }
error-oauth-discovery-bad = { $url }은(는) 디스커버리 문서가 아닙니다
error-oauth-discovery-incomplete = { $url }에 인증 주소와 토큰 주소가 없습니다
error-signin-no-group = { $name }은(는) Hoard를 사용할 수 있는 그룹에 속해 있지 않습니다
error-signin-other-source = 다른 방식으로 로그인하는 { $name } Hoard 계정이 이미 있습니다
error-signin-other-subject = 제공자의 다른 사람에게 속한 { $name } Hoard 계정이 이미 있습니다
error-signin-no-account = { $name }에게는 아직 Hoard 계정이 없습니다. 관리자가 설정 → 계정에서 추가할 수 있습니다
error-signin-bad-name = 이 이름은 Hoard 계정 이름으로 쓸 수 없습니다(64자 이하)
error-oauth-off = OAuth 로그인이 켜져 있지 않습니다
error-oauth-bad-authorization-address = 인증 주소가 올바르지 않습니다
error-oauth-refused = 제공자가 로그인시켜 주지 않았습니다: { $reason }
error-oauth-no-code = 제공자가 로그인 없이 돌려보냈습니다
error-oauth-expired = 로그인이 너무 오래 걸렸거나 다른 곳에서 시작되었습니다. 다시 시도하세요
error-oauth-no-claim = 제공자가 사용자가 누구인지 알려 주지 않았습니다({ $claim } 클레임 없음)
error-oauth-no-sub = 제공자가 사용자가 누구인지 알려 주지 않았습니다(sub 또는 id 클레임 없음)
error-oauth-token-unreachable = 제공자의 토큰 주소에 연결할 수 없습니다
error-oauth-token-refused = 제공자가 로그인을 마치지 않았습니다: { $reason }
error-oauth-no-access-token = 제공자가 액세스 토큰을 보내지 않았습니다
error-oauth-userinfo-unreachable = 제공자의 사용자 정보 주소에 연결할 수 없습니다
error-oauth-userinfo-refused = 제공자가 로그인한 사람이 누구인지 알려 주지 않았습니다({ $status })
error-oauth-userinfo-not-json = 제공자의 사용자 정보가 JSON이 아닙니다
error-format-no-name = name이 비어 있습니다
error-format-no-files = files가 비어 있습니다. ["*.json"]처럼 이 형식이 읽을 파일을 지정하세요
error-format-empty-selector = 비어 있음
error-format-unopened-bracket = 여는 괄호 없이 닫는 괄호가 있습니다
error-format-unclosed-bracket = 괄호가 닫히지 않았습니다
error-format-empty-step-dots = 빈 단계가 있습니다(점이 두 개 연속이거나 양 끝에 점이 있음)
error-format-unclosed-except = !( 이(가) 닫히지 않았습니다
error-format-unclosed-filter = [ 이(가) 닫히지 않았습니다
error-format-after-filters = { $step }의 키와 필터 뒤에 { $rest }이(가) 있습니다
error-format-except-needs-glob = !(…)는 *가 들어 있는 키 뒤에 씁니다
error-format-except-not-glob = !(…)는 { $key }이(가) 아니라 *가 들어 있는 키 뒤에 씁니다
error-format-empty-step = 빈 단계가 있습니다
error-format-not-a-filter = [{ $filter }]은(는) 필터가 아닙니다. [], [key=value|other], [array], [string], [number], [bool], [object] 중 하나를 사용하세요
error-format-filter-needs-value = [{ $filter }]에는 키와 값이 필요합니다
error-store-search-empty = 검색할 내용이 없습니다. 팩 이름을 입력하세요
error-update-hook-not-set = 먼저 호출할 주소를 설정하세요
error-upload-stalled = { $path } 업로드가 멈췄습니다. 그 부분을 다시 보내세요
error-upload-busy = { $path }은(는) 이미 쓰는 중입니다
error-upload-incomplete = 파일 { $count }개가 완료되지 않았습니다. 예: { $files }
error-upload-no-files-listed = 파일 없음
error-upload-not-part = { $path }은(는) 업로드 { $id }의 일부가 아니거나 업로드가 닫혔습니다
error-bad-path = 잘못된 경로
error-upload-part-offset = { $offset }은(는) { $path }의 부분이 시작하는 위치가 아닙니다
error-upload-interrupted = 업로드가 중단되었습니다: { $reason }
error-upload-part-long = { $path }의 { $part }번 부분이 { $len }바이트보다 깁니다
error-upload-part-short = { $path }의 { $part }번 부분: { $len }바이트를 예상했지만 { $got }바이트를 받았습니다
error-upload-file-long = { $path }이(가) 선언된 크기보다 큽니다
error-upload-file-short = { $path }: { $len }바이트를 예상했지만 { $got }바이트를 받았습니다
error-upload-complete = 업로드가 이미 완료되었습니다
error-already-imported = 이미 "{ $name }"(으)로 가져왔습니다

## 서버 텍스트

server-store-place-readme = README
server-store-place-folders = 폴더
server-store-why-linked = 팩의 { $place }에 링크되어 있음
server-store-why-same-title = 팩의 { $place }와(과) 제목이 같음
server-store-why-by = 팩의 { $place }에 이름이 나온 { $author }의 작품
server-store-why-meshes = 메시 수가 팩과 비슷함({ $count })
server-store-why-textures = 텍스처 수가 팩과 비슷함({ $count })
server-chatgpt-revoke-failed = 여기서는 로그아웃했지만 OpenAI에 알리지 못했습니다. 확실히 하려면 ChatGPT의 Settings → Security and login → Sign in with ChatGPT에서 Hoard 연결을 해제하세요.
server-chatgpt-signed-in-title = ChatGPT에 로그인됨
server-chatgpt-signed-in = Hoard가 로그인되었습니다. 이제 AI 태깅에 ChatGPT 요금제를 사용할 수 있습니다.
server-chatgpt-signed-in-as = Hoard가 { $email }(으)로 로그인되었습니다. 이제 AI 태깅에 ChatGPT 요금제를 사용할 수 있습니다.
server-chatgpt-not-finished-title = 로그인이 완료되지 않음
server-chatgpt-other-computer = 다른 컴퓨터의 Hoard에서 로그인을 시작했다면 주소 표시줄에서 이 페이지의 주소를 복사해 그쪽 Hoard의 설정에 붙여 넣으세요.
server-back-to-settings = 설정으로 돌아가기
server-issues-pack = 추가하지 못한 팩
server-issues-pack-blurb = 팩을 삭제하고 다시 추가하거나, 열어서 무엇이 들어 있는지 확인하세요.
server-issues-damaged = 손상되었거나 없어진 저장 파일
server-issues-damaged-blurb = 파일 손상 검사에서 발견되었습니다. 백업에서 복원하거나 팩을 다시 추가한 다음 파일 손상 검사를 실행하세요.
server-issues-folder = 읽을 수 없는 감시 폴더
server-issues-folder-blurb = 폴더를 다시 읽을 수 있을 때까지 팩은 그대로 둡니다.
server-issues-missing = 폴더가 없어진 감시 팩
server-issues-missing-blurb = 폴더가 잠시 오프라인일 수 있으므로 각 팩을 3일 동안 보관한 뒤 삭제합니다.
server-issues-job = 실패한 백그라운드 작업
server-issues-job-blurb = 다시 시도하면 작업을 처음부터 다시 실행합니다.
server-issues-file = 열지 못한 파일
server-issues-file-blurb = Hoard가 열거나 저장하지 못했습니다. 감시 팩은 다음 검사 때 다시 시도하고, 이미 라이브러리에 있던 파일은 이전 상태를 유지합니다.
server-issues-unpack = 풀지 못한 압축 파일
server-issues-unpack-blurb = 각 파일은 팩 안에 일반 파일로 보관되므로, 그 안의 내용은 라이브러리에 없습니다.
server-issues-read = Hoard가 해석하지 못한 파일
server-issues-read-blurb = 라이브러리에는 있지만, 파일을 읽어서 얻는 정보, 태그, 썸네일이 없습니다.
server-issues-render = 썸네일 렌더러가 실패한 모델
server-issues-render-blurb = 썸네일이 없습니다. 뷰어에서 열면 작동할 수도 있고, 그때 썸네일을 저장할 수 있습니다.
server-issues-convert = Blender가 변환하지 못한 모델
server-issues-convert-blurb = GLB 사본이 없어서 3D로 표시할 수 없습니다.
server-issues-unreal = 변환하지 못한 Unreal 패키지
server-issues-unreal-blurb = 나머지 패키지는 팩의 Unreal 페이지에 있습니다.
server-issues-reformat = 변환 규칙으로 변환하지 못한 파일
server-issues-reformat-blurb = 원래대로 남겨 두었습니다.
server-issues-materials = 머티리얼을 알아내지 못한 팩
server-issues-materials-blurb = 텍스처는 모두 있지만 머티리얼로 묶이지 않았을 뿐입니다.
server-issues-material-thumb = 썸네일이 없는 머티리얼
server-issues-material-thumb-blurb = 머티리얼은 찾았지만 미리보기를 그리지 못했습니다.
server-issues-store-files = 읽지 못한 스토어 파일
server-issues-store-files-blurb = Megascans JSON처럼 팩을 받아 온 스토어에서 온 파일로, 팩의 이름과 태그를 알려 줍니다.
server-issues-scan-again = 다시 검사
server-issues-retry = 다시 시도
server-issues-reanalyze = 팩 다시 분석
server-issues-convert-again = 다시 변환
server-issues-look-again = 다시 찾기
server-issues-read-again = 다시 읽기
server-issues-blob-missing = 저장된 파일이 없어졌습니다.
server-issues-blob-damaged = 저장된 파일이 저장할 때와 다릅니다. 손상되었습니다.
server-issues-folder-gone = “{ $folder }” 폴더를 찾을 수 없습니다.
server-job-a-pack = 팩
server-job-ingest = { $pack } 가져오는 중
server-job-retag = { $pack } 다시 태그하는 중
server-job-llm-tag = AI로 { $pack } 태그하는 중
server-job-convert = Blender로 { $pack } 변환하는 중
server-job-unreal = { $pack }의 Unreal 패키지 변환하는 중
server-job-reformat = 변환 규칙에 따라 { $pack }의 파일 변환하는 중
server-job-scan = 감시 폴더 검사하는 중
server-job-relocate = 라이브러리 위치 간에 파일 옮기는 중
server-job-looks = 색상과 외형 분석하는 중
server-job-sounds = 사운드 듣는 중
server-job-embed = 그림 내용 학습하는 중
server-job-hear = 사운드가 어떻게 들리는지 학습하는 중
server-job-materials = { $pack }에서 머티리얼 찾는 중
server-job-read-files = { $pack }의 파일 읽는 중
server-job-store-lookup = 스토어 페이지 찾는 중
server-maintenance-backup = 데이터베이스 백업
server-maintenance-backup-blurb = 데이터베이스(팩, 태그, 메모, 컬렉션, 설정)의 일관된 사본을 데이터베이스 옆의 backups 폴더에 만듭니다. 라이브러리의 파일은 포함되지 않습니다. 한 번 저장된 파일은 바뀌지 않으므로 어떤 파일 동기화 도구로든 백업할 수 있습니다.
server-maintenance-scrub = 파일 손상 검사
server-maintenance-scrub-blurb = 저장된 모든 파일을 다시 읽어서 지문과 비교하여, 디스크나 NAS에서 손상되었거나 없어진 파일을 찾아냅니다. 라이브러리가 크면 시간이 걸립니다.
server-maintenance-reanalyze = 모든 팩 다시 분석
server-maintenance-reanalyze-blurb = 현재 버전의 Hoard로 모든 파일을 다시 읽어서 썸네일, 미리보기, 세부 정보, 규칙 태그를 새로 만듭니다. 직접 편집한 내용은 유지되고 AI에는 다시 묻지 않습니다.
server-maintenance-clear-cache = 캐시된 변환 지우기
server-maintenance-clear-cache-blurb = “다른 형식으로 다운로드” 변환에서 보관한 파일(다음에 요청하면 다시 만듭니다)과 중단된 작업이 남긴 파일을 지웁니다.
server-maintenance-shrink-thumbs = 썸네일 줄이기
server-maintenance-shrink-thumbs-blurb = PNG로 보관된 썸네일과 모델 뷰를 설정 → 성능 → 썸네일에 지정한 방식으로 저장합니다. 보통 크기가 몇 배 작아집니다. 투명도가 있는 것과 픽셀 아트는 PNG로 유지됩니다.
server-maintenance-avif-thumbs = 썸네일을 AVIF로 변환
server-maintenance-avif-thumbs-blurb = 썸네일을 AVIF로 보낼 때, 처음 표시될 때마다 변환하는 대신 모든 썸네일을 지금 변환합니다.
server-maintenance-gc = 사용하지 않는 파일 삭제
server-maintenance-gc-blurb = 영구 삭제된 팩의 파일처럼 더 이상 아무것도 가리키지 않는 저장 파일을 삭제합니다.
server-update-no-address = 호출할 주소가 설정되지 않았습니다
server-update-no-answer = 30초 안에 응답이 없습니다

## 설정: 모양

appearance-language = 언어
appearance-language-auto = 브라우저 언어({ $name })
appearance-scheme = 밝게 또는 어둡게
appearance-scheme-system = 시스템
appearance-size-section = 글자와 간격
appearance-text-size = 글자 크기
appearance-text-small = 작게
appearance-text-normal = 보통
appearance-text-large = 크게
appearance-density = 간격
appearance-density-comfortable = 넉넉하게
appearance-density-compact = 촘촘하게
appearance-tiles = 타일
appearance-tiles-what = 라이브러리의 각 타일이 그림 외에 보여 주는 것.
appearance-tile-names = 이름
appearance-tile-details = 종류와 크기(픽셀, 길이, 삼각형)
appearance-tile-badges = 그림 위의 파일 형식
appearance-tile-motion = 포인터를 올려 두는 동안 애니메이션 재생
appearance-top-bar = 상단 막대
appearance-top-bar-what = 쓰지 않는 것은 숨기세요. /는 계속 검색 상자로 이동하고 { $keys }는 계속 명령 팔레트를 엽니다.
appearance-top-activity = 백그라운드 작업
appearance-top-theme = 밝게/어둡게 전환
appearance-top-mic = 검색 상자의 마이크(소리를 내서 비슷한 소리 찾기)
appearance-top-slash = 검색 상자의 / 힌트
appearance-top-palette = 명령 팔레트 버튼({ $keys })

## 루트

root-unreachable = Hoard 서버에 연결할 수 없습니다: { $error }
root-unreachable-desktop = Hoard가 응답하지 않습니다: { $error }
root-desktop-reopen = 이 창은 로그아웃되었습니다. 계속하려면 Hoard를 닫았다가 다시 여세요.
root-try-again = 다시 시도
root-lazy-connect = 앱 연결
window-minimize = 최소화
window-maximize = 최대화
window-restore = 이전 크기로 복원
window-close = 닫기

## 앱: 라이브러리, 상단 바, 필터

app-lazy-settings = 설정
app-lazy-archive = 보관함
app-lazy-shortcuts = 단축키
app-lazy-setup = 설정 마법사
app-theme-system = 시스템 테마 따르기
app-theme-light = 밝은 테마
app-theme-dark = 어두운 테마
app-color-red = 빨강
app-color-orange = 주황
app-color-yellow = 노랑
app-color-green = 초록
app-color-teal = 청록
app-color-blue = 파랑
app-color-purple = 보라
app-color-pink = 분홍
app-color-brown = 갈색
app-color-sand = 모래색
app-color-gray = 회색
app-color-slate = 슬레이트
app-color-black = 검정
app-color-white = 흰색
app-color-swatch = { $color }: { $lower }이(가) 많이 들어간 에셋
app-color-any = 모든 색상
app-color-pick = 색상 직접 고르기
app-color = 색상
app-music = 음악
app-tempo = BPM
app-tempo-min = 최소
app-tempo-max = 최대
app-tempo-min-title = 이 템포 이상인 음악, 분당 박자 수 기준 (절반이나 두 배도 포함)
app-tempo-max-title = 이 템포 이하인 음악, 분당 박자 수 기준
app-key = 조성
app-key-any = 모든 조성
app-key-major = 장조
app-key-minor = 단조
app-key-mix = 함께 믹스할 수 있는 조성 포함
app-key-mix-title = 나란한조와 5도 위아래의 조성으로, DJ가 함께 믹스하는 조성입니다
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = 소리를 철자로 적듯이 입력하세요 (예: whooom). 모양이 비슷한 소리가 먼저 나옵니다
app-imitation-record = 마이크에 대고 소리를 내세요. 모양이 비슷한 소리가 먼저 나옵니다
app-imitation-stop = 녹음 중지
app-imitation-heard = 내 소리
app-imitation-mic-failed = 마이크로 녹음할 수 없습니다: { $error }
app-retry = 다시 시도
app-undo = 실행 취소
app-search-failed = 검색 실패: { $error }
app-unreachable = 서버에 연결할 수 없습니다: { $error }
app-unreachable-desktop = Hoard가 응답하지 않습니다: { $error }
app-forget-project-title = { $name }을(를) 잊을까요?
app-forget-project-body = 다음 hoard sync가 다시 알려 줄 때까지 Hoard는 이 프로젝트의 에셋 { $count }개가 사용되는 곳에 이 프로젝트를 표시하지 않습니다. 프로젝트 자체는 아무것도 바뀌지 않습니다.
app-forget-project-confirm = 잊기
app-forget-project-failed = { $name }을(를) 잊지 못했습니다: { $error }
app-forget-project-label = 프로젝트 { $name } 잊기
app-forget-project = 이 프로젝트 잊기
app-save-name-like = { $name }과(와) 비슷한 것
app-save-name-favorites = 즐겨찾기
app-search-saved = “{ $name }”을(를) 저장했습니다. 필터의 내 항목에 있습니다
app-search-updated = “{ $name }”을(를) 업데이트했습니다
app-rank-alike = 가장 비슷한 순
app-rank-color = 색상이 많은 순
app-rank-match = 관련도 순
app-rank-imitation = 소리가 비슷한 순
app-sort = 정렬
app-sort-name = 이름
app-sort-created = 생성일
app-sort-updated = 수정일
app-sort-size = 크기
app-sort-rating = 평점
app-sort-newest-first = 최신순
app-sort-oldest-first = 오래된 순
app-sort-a-to-z = A→Z
app-sort-z-to-a = Z→A
app-sort-largest-first = 큰 순
app-sort-smallest-first = 작은 순
app-sort-best-rated-first = 평점 높은 순
app-sort-lowest-rated-first = 평점 낮은 순
app-collection-added = 에셋 { $count }개를 { $name }에 추가했습니다
app-collection-add-failed = 컬렉션에 추가하지 못했습니다: { $error }
app-collection-removed = { $name }에서 { $count }개를 제거했습니다
app-collection-remove-failed = 컬렉션에서 제거하지 못했습니다: { $error }
app-ns-subject = 주제
app-ns-style = 스타일
app-ns-theme = 테마
app-ns-material = 재질
app-ns-map = PBR 맵
app-ns-color = 색상
app-ns-mood = 분위기
app-ns-genre = 장르
app-ns-license = 라이선스
app-ns-res = 해상도
app-ns-duration = 길이
app-ns-poly = 폴리곤 수
app-ns-format = 형식
app-ns-weight = 용량
app-ns-tech = 기술 정보
app-ns-blender = Blender 버전
app-ns-depth = 비트 심도
app-ns-ucs = 사운드 분류(UCS)
app-ns-bpm = 템포
app-ns-root = 근음
app-ns-level = 음량
app-ns-ue = Unreal 클래스
app-ns-engine = 엔진
app-ns-extracted = 복구된 원본
app-ns-script = 문자 체계
app-bulk-failed = 에셋 { $count }개를 변경하지 못했습니다: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied = 태그 { $count }개를 복사했습니다
app-tags-copy-none = { $name }: 복사할 태그가 없습니다
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = 복사된 태그가 없습니다. 에셋에서 C를 눌러 태그를 복사하세요.
# Pasting tags that every one of the assets already has.
app-tags-paste-had = 에셋 { $count }개에 이미 해당 태그가 있습니다
# After pasting: $count tags were added to $assets assets.
app-tags-pasted = 에셋 { $assets }개에 태그 { $count }개를 추가했습니다
app-bulk-untagged = 에셋 { $count }개에서 { $tag } 태그를 뗐습니다
app-bulk-tagged = 에셋 { $count }개에 { $tag } 태그를 붙였습니다
app-bulk-review = 검토함으로 표시
app-bulk-review-title = 선택한 에셋을 검토한 것으로 표시: 이후 규칙과 AI는 태그를 건드리지 않습니다
app-bulk-unreview = 검토 해제
app-bulk-unreview-title = 선택한 에셋의 검토함 표시 해제
app-bulk-reviewed = 에셋 { $count }개를 검토함으로 표시했습니다
app-bulk-unreviewed = 에셋 { $count }개의 검토함 표시를 해제했습니다
app-bulk-accept = 추측 태그 유지
app-bulk-accept-title = 규칙과 AI가 선택한 에셋에 붙인 태그를 모두 내 태그로 유지
app-bulk-accepted = 에셋 { $count }개의 추측 태그를 유지했습니다
app-credits-failed = 크레디트를 만들지 못했습니다: { $error }
app-archived-assets = 에셋 { $count }개를 보관했습니다
app-archived-one = { $name }을(를) 보관했습니다
app-open-failed = 에셋을 열지 못했습니다: { $error }
app-unnamed-pack = 팩
app-unnamed-collection = 컬렉션
app-unnamed-asset = 에셋
app-home-version = Hoard { $version } — 라이브러리로 돌아가기
app-home = 라이브러리로 돌아가기
app-search-label = 에셋 검색
app-search-placeholder = 이름, 경로, 태그 검색
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = 바위 or 돌: 둘 중 하나 · -이끼: 제외 · "마른 이끼": 이 단어들을 함께 · pack:숲: 팩 이름에서
app-alt-leave-out = Alt-클릭하면 제외
app-views = 보기
app-library = 라이브러리
app-packs = 팩
app-packs-working = { $count }개 처리 중
app-update-title = Hoard { $version }이(가) 나왔습니다(이 서버는 { $current } 실행 중). 새 이미지나 바이너리를 받아서 다시 시작하세요. 데이터베이스는 자동으로 업그레이드됩니다.
app-update = 업데이트
app-theme = 테마
app-sign-in = 로그인
app-archive-title = 보관함: { $contents }
app-archive = 보관함
app-settings = 설정
app-palette-title = 명령 팔레트({ $keys })
app-import-folder = 폴더 가져오기
app-add-files = 파일 추가
app-add-files-short = 추가
app-split-label = 이 폴더를 가져오는 방법
app-split-before = { "" }
app-split-after = 에 폴더 또는 압축 파일이 { $count }개 있습니다.
app-split-each = 각각 별도의 팩으로 가져오기({ $count })
app-split-one = 하나의 팩으로 가져오기
app-cancel = 취소
app-filters = 필터
app-done = 완료
app-yours = 내 항목
app-favorites = 즐겨찾기
app-needs-review = 검토 필요
app-needs-review-title = 아직 아무도 검토하지 않은 에셋만(다른 필터와 함께 적용)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = 평점
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
       *[other] 별 { $count }개 이상
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = 다시 클릭하면 모든 평점 표시
app-forget-search-label = 저장된 검색 { $name } 잊기
app-forget-search = 이 검색 잊기
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = 이 검색으로 라이브러리 열기
app-default-search-on-title = 라이브러리가 이 검색으로 열립니다. 클릭하면 해제합니다.
app-default-search-label = { $name }(으)로 라이브러리 열기
app-rename-search = 이름 바꾸기
app-rename-search-label = { $name }의 새 이름
app-projects-title = 여기서 hoard sync하는 프로젝트와 마지막 동기화 때의 내용
app-projects = 프로젝트
app-project-synced = { $when } 동기화됨
app-project-synced-by = { $when } { $who }이(가) 동기화함
app-collections = 컬렉션
app-no-filters = 이 검색에는 필터가 없습니다.
app-kind = 종류
app-facets-more = { $count }개 더 보기
app-facets-fewer = 접기
app-searching = 검색 중…
app-searching-live = 검색 중
app-asset-count = 에셋 { $count }개
app-clear-all = 모두 지우기
app-save-name-label = 이 검색의 이름
app-save-name-placeholder = 이름 입력
app-save = 저장
app-save-search-title = 이 검색을 필터의 내 항목에 보관합니다
app-save-search = 검색 저장
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = “{ $name }” 업데이트
app-update-search-title = 지금 선택한 내용을 이 저장된 검색에 유지
app-save-search-new = 새로 저장
app-select-title = 여러 에셋을 골라서 한꺼번에 태그, 분류, 컬렉션 추가, 다운로드, 보관합니다(X 또는 Ctrl+클릭)
app-select = 선택
app-tile-size = 타일 크기
app-supporting-title = glTF 버퍼, 머티리얼 라이브러리, 머티리얼의 맵, 모델 하나만 쓰는 머티리얼, 모델을 설명하는 스토어 파일입니다. 이 파일들을 사용하는 것을 다운로드하면 함께 받아집니다. 스프라이트 시트의 페이지와, 팩에 별도 이미지로도 있는 그 스프라이트는 시트와 함께 표시됩니다. 팩에 여러 형식으로 있는 에셋은 Hoard가 가장 잘 읽는 형식으로 한 번만 표시됩니다.
app-supporting-hidden = 보조 파일 { $count }개 숨김
app-supporting-show = 표시
app-supporting-shown = 보조 파일 포함
app-supporting-hide = 숨기기
app-folders = 폴더
app-folders-title = 팩을 디스크에 있던 그대로 폴더별로 보기
app-folders-label = 팩의 폴더
app-welcome-title = 이 페이지 아무 곳에나 에셋 팩을 놓으세요
app-welcome-before = zip, tarball, 폴더, 개별 파일 모두 됩니다. Hoard가 압축을 풀고, 모든 파일을 해시 기준으로 한 번만 저장하고, 크기·길이·폴리곤 수를 읽고, 모든 것에 태그를 붙이므로 이렇게 검색할 수 있습니다:{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }또는{ " " }
app-welcome-after = .
app-welcome-import-folder = 폴더 가져오기
app-welcome-choose-files = 파일 선택
app-none-similar = 아직 { $name }과(와) 많이 비슷한 것이 없습니다.
app-none = 일치하는 항목이 없습니다.
app-none-query = “{ $query }”과(와) 일치하는 항목이 없습니다.
app-none-imitation = 아직 “{ $word }”와(과) 비슷한 측정된 소리가 없습니다.
app-none-heard = 아직 내 소리와 비슷한 측정된 소리가 없습니다.
app-none-filters = 이 필터와 일치하는 항목이 없습니다.
app-none-query-filters = 이 필터로는 “{ $query }”과(와) 일치하는 항목이 없습니다.
app-clear-search-filters = 검색어와 필터 지우기
app-loading-more = 더 불러오는 중
app-keys-hint = 화살표 키로 이동, Enter로 열기, Space로 사운드 재생, X 또는 Ctrl+클릭으로 여러 개 선택(터치에서는 길게 누르기).
app-resize-label = 세부 정보 패널 크기 조정
app-resize-title = 드래그하여 크기 조정 · 더블클릭하면 맞는 너비로
app-selected-label = 선택한 에셋
app-selected = { $count }개 선택됨
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = 편집…
app-edit-selected-title = 모든 항목의 태그와 필드 변경 (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
       *[other] 검색 결과 { $count }개 선택됨
    }
app-pick-assets = 에셋 선택
app-download-zip = zip 다운로드
app-credits-title = 이 에셋들이 속한 팩의 CREDITS.md: 만든 사람, 라이선스, 요청하는 크레디트 문구
app-credits = 크레디트
app-prefer-sources-title = Hoard가 원본 파일을 복구한 Unreal SoundWave와 텍스처
app-prefer-sources = .uasset 대신 복구된 WAV/PNG({ $count })
bag-clear = 선택 비우기
bag-glb-heading = 모델을 GLB로
bag-glb-text = 모든 모델을 Y축이 위, 앞쪽을 향하도록 세운 .glb 파일로 만들고, 텍스처는 모델들이 함께 쓰는 textures 폴더에 별도 파일로 넣습니다. 나머지는 그대로 들어갑니다.
bag-glb-download = GLB zip 다운로드
bag-plain-heading = 파일 그대로
bag-plain-text = 각 파일을 가져온 그대로, 필요한 파일과 함께 팩마다 폴더에 넣습니다.
bag-remove = { $name }을(를) 선택에서 빼기
bag-more = 외 { $count }개
app-bulk-tag-label = 선택한 에셋에 붙이거나 뗄 태그
app-bulk-tag-placeholder = 태그
app-bulk-tag-add-title = 선택한 모든 에셋에 이 태그 붙이기
app-bulk-tag-add = 추가
app-bulk-tag-remove-title = 선택한 모든 에셋에서 이 태그 떼기
app-bulk-tag-remove = 제거
app-remove-from-collection = { $name }에서 제거
app-unfavorite = 즐겨찾기 해제
app-favorite = 즐겨찾기
app-select-all = { $count }개 모두 선택
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
       *[other] 검색 결과 { $count }개 모두 선택
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
       *[other] 에셋 { $count }개 편집
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
       *[other] 검색 결과 { $count }개 모두 편집
    }
bulk-edit-close = 닫기
bulk-edit-tags = 태그
bulk-edit-tag-add = 모든 항목에 태그 추가
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $count }개 중 { $have }개
bulk-edit-untick = 모두 선택 해제
bulk-edit-untick-title = 모든 태그를 제거한 다음 유지할 태그를 선택하세요
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = 그중 가장 많이 쓰인 태그
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = 값이 다름, 그대로 둠
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = 그대로 둠
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = 모든 항목에서 비움
bulk-edit-reset = 원래대로
bulk-edit-all-note = 아직 불러오지 않은 결과를 포함해 모든 검색 결과가 변경되며, 한 번에 되돌릴 수 없습니다.
bulk-edit-save =
    { $count ->
       *[other] 변경 { $count }개 저장
    }
bulk-edit-nothing = 변경된 내용 없음
bulk-edit-saved =
    { $count ->
       *[other] 에셋 { $count }개를 업데이트했습니다
    }
app-archive-selected-title = 보관함에서 복원하거나 영구 삭제할 때까지 검색에서 빠집니다
app-drop-items = 놓아서 항목 { $count }개 추가
app-drop = 놓아서 Hoard에 추가
app-drop-hint = 최상위 파일이나 폴더 하나하나가 각각 팩이 됩니다. 기가바이트 단위도 괜찮고, 연결이 끊겨도 큰 파일은 이어서 올립니다.

## 명령 팔레트

palette-group-go-to = 이동
palette-group-actions = 작업
palette-group-theme = 테마
palette-group-kinds = 종류
palette-group-packs = 팩
palette-group-collections = 컬렉션
palette-group-projects = 프로젝트
palette-group-saved = 저장된 검색
palette-group-assets = 에셋
palette-hint-current = 현재
palette-library = 라이브러리
palette-library-keywords = 에셋 그리드 홈 assets grid home
palette-favorites = 즐겨찾기
palette-favorites-keywords = 별표 좋아요 starred stars liked
palette-packs = 팩
palette-archive = 보관함
palette-archive-keywords = 휴지통 삭제 복원 trash recycle bin deleted restore
palette-stats = 라이브러리 통계
palette-stats-keywords = 통계 숫자 크기 디스크 공간 개수 statistics numbers size disk space count
palette-copies = 거의 같은 사본
palette-copies-keywords = 중복 같은 비슷한 크기변경 변환 낭비 공간 duplicates same similar resized converted wasted space dedupe
palette-search = 검색
palette-add-files = 파일 추가…
palette-add-files-keywords = 업로드 가져오기 upload import
palette-import-folder = 폴더 가져오기…
palette-import-folder-keywords = 업로드 팩 upload pack
palette-select-all = 표시된 { $count }개 모두 선택
palette-select-all-keywords = 체크 check
palette-select-several = 여러 개 선택…
palette-select-several-keywords = 일괄 여러 태그 보관 bulk multiple tag archive
palette-archive-selected = 선택한 { $count }개 보관
palette-archive-selected-keywords = 삭제 제거 휴지통 delete remove trash
palette-edit-selected = 선택한 { $count }개 편집
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = 일괄 변경 태그 카테고리 라이선스 평점
palette-paste-tags = 복사한 태그를 선택한 { $count }개에 붙여넣기
palette-clear-selection = 선택 해제
palette-clear-filters = 필터 지우기
palette-clear-filters-keywords = 초기화 재설정 reset
palette-hide-supporting = 보조 파일 숨기기
palette-show-supporting = 보조 파일 표시
palette-supporting-keywords = 노멀 맵 러프니스 마스크 normal maps roughness masks
palette-account = 계정 및 액세스 토큰
palette-account-keywords = 비밀번호 로그인 사용자 토큰 password login user token api
palette-sign-out = 로그아웃({ $name })
palette-sign-out-keywords = 로그아웃 log out logout
palette-sign-in = 로그인
palette-sign-in-keywords = 로그인 계정 비밀번호 log in login account password
palette-settings = 설정
palette-settings-keywords = 관리자 도구 환경설정 태깅 blender ffmpeg llm ai tagging unreal admin tools preferences
palette-shortcuts = 단축키
palette-shortcuts-keywords = 도움말 키 help keys
palette-theme-keywords = 테마 모양 모드 theme appearance mode
palette-theme-variant-keywords = 모양 색상 appearance color
palette-light-theme = 밝은 테마: { $name }
palette-dark-theme = 어두운 테마: { $name }
palette-kind-all = 모든 종류({ $kind }만 표시 해제)
palette-kind-only = { $kind }만
palette-title = 명령 팔레트
palette-placeholder = 에셋 검색, 팩으로 이동, 명령 실행…
palette-results = 결과
palette-searching = 검색 중…
palette-no-match = “{ $query }”과(와) 일치하는 항목이 없습니다
palette-foot-move = 이동
palette-foot-run = 실행
palette-foot-shortcuts = 모든 단축키

## 단축키

shortcuts-title = 단축키
shortcuts-close = 닫기
shortcuts-anywhere = 어디서나
shortcuts-palette = 명령 팔레트: 검색, 이동, 실행
shortcuts-search = 검색
shortcuts-this-list = 이 목록
shortcuts-escape = 닫기, 선택 해제 또는 세부 정보 닫기
shortcuts-library = 라이브러리
shortcuts-move = 타일 간 이동(또는 J와 K)
shortcuts-open = 세부 정보 열기
shortcuts-play-sound = 사운드 재생 또는 정지
shortcuts-select = 선택 또는 선택 해제
shortcuts-favorite = 즐겨찾기에 추가 또는 제거
shortcuts-reviewed = 검토함으로 표시 또는 해제
shortcuts-tag = 태그 추가
shortcuts-edit = 선택 항목 편집
shortcuts-copy-tags = 태그 복사
shortcuts-paste-tags = 복사한 태그를 선택 항목에 붙여넣기
shortcuts-key-click = 클릭
shortcuts-select-range = 범위 선택
shortcuts-select-all = 표시된 모든 항목 선택
shortcuts-sound-player = 사운드 플레이어
shortcuts-play-pause = 재생 또는 일시 정지
shortcuts-skip = 뒤로 또는 앞으로 건너뛰기
shortcuts-start-end = 처음 또는 끝
shortcuts-image-inspector = 이미지 검사기
shortcuts-zoom = 확대 또는 축소
shortcuts-fit = 창에 맞추기
shortcuts-actual-pixels = 실제 픽셀
shortcuts-tile = 3×3으로 타일링해서 이음새 확인
shortcuts-background = 다음 배경
shortcuts-channel = 채널 하나만 표시

## KindChips.vue

kinds-group = 에셋 종류

## TagChip.vue

tag-title-with-source = { $tag }({ $source })
tag-source-human = 사람이 추가함
tag-source-rule = 오프라인 태거가 붙임
tag-source-meta = 파일에서 측정함
tag-source-ai = AI가 제안함
tag-remove = { $tag } 태그 제거
tag-accept = { $tag } 태그 유지
tag-accept-title = 이 추측을 내 태그로 유지
tag-reject-title = 제거합니다. 태거가 다시 붙이지 않습니다
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = 새 태그
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = 혹시 이것인가요
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = 이전 이름 { $alias }

## ActivityIndicator.vue

activity-label-idle = 백그라운드 작업: 대기 중
activity-label-running = 백그라운드 작업: { $running }개 실행 중
activity-label-queued = 백그라운드 작업: { $running }개 실행 중, { $queued }개 대기 중
activity-title = 백그라운드 작업
activity-unreachable = 서버에 연결할 수 없습니다
activity-unreachable-desktop = Hoard가 응답하지 않습니다
activity-empty = 실행 중인 작업이 없습니다. 가져오기, 태깅, 변환, zip 다운로드가 진행되는 동안 여기에 표시됩니다.
activity-queued = 대기 중
activity-stop = “{ $what }” 중지
activity-stop-tagging = AI 태깅 모두 중지 ({ $count })
activity-seconds = { $s }초
activity-minutes = { $min }분
activity-hours = { $h }시간 { $min }분
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = 삼각형 { $count }개
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = { $name } 재생
tile-pause = { $name } 일시 정지
tile-favorite = { $name } 즐겨찾기
tile-in-favorites = 즐겨찾기에 있음
tile-add-favorite = 즐겨찾기에 추가(F)
tile-more-like = { $name }과(와) 비슷한 것 더 보기
tile-more-like-this = 비슷한 것 더 보기
tile-select = { $name } 선택

## CollectionPicker.vue

collections-add = 컬렉션에 추가
collections-menu = 컬렉션
collections-find-label = 컬렉션 찾기 또는 만들기
collections-find-placeholder = 컬렉션을 찾거나 새 이름 입력
collections-create = 만들기
collections-empty = 아직 컬렉션이 없습니다. 이름을 입력해서 만드세요.

## FolderPicker.vue

folders-list = 폴더
folders-close = 닫기
folders-browse = 찾아보기…
folders-up-title = 상위 폴더로
folders-up = 위로
folders-this-computer = 이 컴퓨터
folders-none = 이 안에 폴더가 없습니다.

## LocationForm.vue

locations-mode-question = Hoard가 이 폴더를 어떻게 사용할까요?
locations-mode-managed = 차지하기: Hoard가 여기에 팩을 보관
locations-mode-managed-hint = 업로드하거나 가져온 팩이 여기에 복사되고, Hoard가 정리해 둡니다. 이제 이 폴더는 Hoard의 것이므로 내용을 직접 건드리지 마세요.
locations-mode-watched = 지켜보기: 팩이 이미 여기에 있음
locations-mode-watched-hint = 그 안의 폴더 하나하나가 팩이 됩니다. Hoard는 파일을 제자리에서 읽기만 하고 절대 바꾸거나 옮기거나 삭제하지 않으며, 새 폴더가 생기면 가져옵니다.
locations-folder-desktop = 폴더
locations-folder-placeholder = 팩이 들어 있는 폴더
locations-folder-hint-managed = 폴더가 없으면 Hoard가 만듭니다. Docker에서는 컨테이너에 마운트한 폴더입니다.
locations-folder-hint-managed-desktop = 폴더가 없으면 Hoard가 만듭니다.
locations-folder-hint-watched = Docker에서는 먼저 폴더를 컨테이너에 마운트한 다음(읽기 전용도 괜찮습니다) 여기서 고르세요.
locations-folder-hint-watched-desktop = Hoard는 읽기만 하며, 안의 어떤 것도 바꾸지 않습니다.
locations-name = 이름
locations-name-placeholder-managed = 라이브러리
locations-name-placeholder-watched = 내 팩
locations-files = 여기로 올 파일
locations-catch-all = 다른 위치가 받지 않는 나머지도 모두
locations-kinds-hint = 새 팩의 파일은 그 종류에 체크한 위치로 갑니다. 나머지는 기본 위치로 갑니다.
locations-files-all = 지금은 전부입니다. 나중에 설정에서 위치를 더 추가해서 일부 종류(예: 음악)를 다른 디스크로 보낼 수 있습니다.
locations-first-watched-note = 업로드하려면 Hoard가 관리하는 라이브러리가 필요합니다. 나중에 설정 → 라이브러리 위치에서 추가할 수 있습니다.
locations-checking = 폴더 확인 중…
locations-add = 위치 추가
locations-cancel = 취소

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = 라이브러리 위치를 불러오지 못했습니다
locations-takes-and-rest = { $kinds } 및 그 밖의 모든 것
locations-takes-rest = 다른 위치가 받지 않는 모든 것
locations-takes-everything = 모든 것
locations-takes-nothing = 아직 없음
locations-save-failed = 위치를 저장하지 못했습니다
locations-unwatch-title = { $name } 감시를 중지할까요?
locations-unwatch-body = 이 위치의 팩 { $count }개가 태그 및 메모와 함께 Hoard에서 빠집니다. 폴더의 파일은 건드리지 않습니다.
locations-unwatch-body-empty = 폴더의 파일은 건드리지 않습니다.
locations-unwatch = 감시 중지
locations-remove-title = { $name }을(를) 제거할까요?
locations-remove-body = 안에 있는 파일은 먼저 종류에 따라 다른 위치로 옮겨지고, 비워지면 위치가 제거됩니다. 잃는 것은 없습니다.
locations-remove-body-only = 파일이 들어 있다면 먼저 파일을 옮길 다른 위치를 추가하세요.
locations-remove = 제거
locations-removed = { $name }을(를) 제거했습니다
locations-emptying = { $name }에서 파일을 옮기는 중입니다. 끝나면 제거됩니다
locations-offline-title = { $name }에 연결할 수 없습니다
locations-offline-body = 오프라인 상태에서는 파일을 옮길 수 없습니다. 그래도 제거할까요? 남아 있는 파일은 없어진 것으로 표시됩니다.
locations-remove-anyway = 그래도 제거
locations-remove-failed = { $name }을(를) 제거하지 못했습니다
locations-rebalancing = 저장된 파일을 종류에 맞는 위치로 옮기는 중
locations-rebalance-failed = 파일을 옮기지 못했습니다
locations-scanned-never = 아직 검사하지 않음
locations-scanned-now = 방금 검사함
locations-scanned-minutes = { $mins }분 전 검사함
locations-scanned-hours = { $hours }시간 전 검사함
locations-scanned-date = { $date } 검사함
locations-scan-failed = { $name }을(를) 검사하지 못했습니다
locations-watching = { $name } 감시 중
locations-added = { $name }을(를) 추가했습니다
locations-loading = 불러오는 중…
locations-managed-title = Hoard가 관리하는 라이브러리
locations-watched-title = 감시 폴더
locations-managed-hint = 업로드와 가져오기가 여기에 복사됩니다. 각 위치는 체크한 종류를 받고, 나머지는 기본 위치가 받습니다.
locations-watched-hint = 제자리에서 색인하는 팩 폴더입니다. Hoard는 그 안의 파일을 절대 바꾸지 않으며, 15분마다 새 폴더와 없어진 폴더를 찾습니다.
locations-managed-empty = 아직 없어서 업로드를 둘 곳이 없습니다. 아래에서 추가하세요.
locations-reachable = 연결 가능
locations-unreachable-now = 지금 연결할 수 없음
locations-catch-all-tag = 기본 위치
locations-packs = 팩 { $count }개
locations-unreachable = 연결할 수 없음
locations-scanning = 검사 중…
locations-scan = 지금 검사
locations-edit = 편집
locations-catch-all-locked = 바꾸려면 다른 위치를 기본 위치로 지정하세요
locations-edit-hint-before = 지금부터 추가하는 파일에 적용됩니다. 이미 저장된 파일을 옮기려면 아래의
locations-edit-hint-after = 버튼을 사용하세요.
locations-save = 저장
locations-rebalance = 저장된 파일을 설정에 맞게 옮기기
locations-rebalance-hint = 위치가 받는 종류를 바꾼 뒤에 사용하세요. 이미 저장된 파일이 지금 그 종류가 가는 위치로 옮겨집니다.
locations-add-title = 위치 추가
locations-add-hint = 일부 종류의 파일을 둘 다른 디스크, 또는 이미 가지고 있는 팩 폴더입니다.
locations-kind-question = 어떤 위치인가요?
locations-kind-folder = 폴더
locations-kind-folder-hint = 컨테이너에 마운트한 디스크나 NAS 공유 폴더, 또는 Dropbox, Google Drive, OneDrive, iCloud Drive가 동기화하는 폴더. Hoard는 그곳에 파일을 보관하거나, 이미 가진 팩을 지켜볼 수 있습니다.
locations-kind-folder-hint-desktop = 이 컴퓨터, 다른 디스크나 NAS 공유 폴더, 또는 Dropbox, Google Drive, OneDrive, iCloud Drive가 동기화하는 폴더. Hoard는 그곳에 파일을 보관하거나, 이미 가진 팩을 지켜볼 수 있습니다.
locations-kind-s3-hint = Amazon S3 또는 비슷한 서비스의 버킷: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard가 그곳에 파일을 보관합니다.
locations-s3-title = S3 스토리지
locations-s3-hint = s3://bucket-name 위치는 이 서비스의 버킷입니다. AWS 또는 S3 호환 서비스(Backblaze B2, Cloudflare R2, Wasabi, MinIO, NAS의 Garage)를 쓸 수 있습니다.
locations-bucket = 버킷
locations-bucket-hint = 이미 있는 버킷의 이름. 위의 키로 목록 보기, 읽기, 쓰기를 할 수 있어야 합니다.
locations-s3-endpoint = 주소
locations-s3-endpoint-placeholder = AWS는 비워 두세요. 예: https://s3.eu-central-003.backblazeb2.com
locations-s3-region = 리전
locations-s3-access-key = 액세스 키 ID
locations-s3-secret-key = 비밀 액세스 키
locations-s3-headers = 추가 헤더
locations-s3-headers-placeholder = 한 줄에 하나씩 Name=value(예: Cloudflare Access용)
locations-s3-kept = 저장됨(입력하면 교체)
locations-s3-secrets = 키와 헤더는 서버 설정과 함께 보관되며 다시 표시되지 않습니다.
locations-s3-secrets-desktop = 키와 헤더는 이 컴퓨터의 키체인에 보관되며 다시 표시되지 않습니다.
locations-s3-saved = S3 스토리지를 저장했습니다
locations-s3-save-failed = S3 스토리지를 저장하지 못했습니다
locations-add-more = 위치 추가…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = 문제 목록을 불러오지 못했습니다
issues-started = { $count }개를 다시 시작했습니다.
issues-retry-failed = 다시 시도하지 못했습니다
issues-dismiss-failed = 닫지 못했습니다
issues-dismiss-all-title = { $count }개를 모두 닫을까요?
issues-dismiss-all-body = “{ $title }” 목록이 지워집니다. 다시 문제가 생기면 다시 표시됩니다.
issues-dismiss-all = 모두 닫기
issues-just-now = 방금
issues-minutes-ago = { $mins }분 전
issues-hours-ago = { $hours }시간 전
issues-loading = 불러오는 중…
issues-none = 문제가 없습니다.
issues-action-all = 모두 { $action }
issues-open-asset = 열기
issues-open-pack = 팩 보기
issues-open-locations = 라이브러리 위치
issues-where-before = 위치:
issues-where-after = { "" }
issues-times = { $count }회
issues-starting = 시작 중…
issues-dismiss-named = { $title } 닫기
issues-dismiss = 닫기
issues-show-fewer = 접기
issues-show-all = { $count }개 모두 보기
issues-not-listed = { $count }개는 더 표시하지 않습니다.

## archive.ts

archive-anyway = 그래도 보관
archive-failed = 보관하지 못했습니다: { $error }
archive-undo = 실행 취소
archive-restore-failed = 복원하지 못했습니다: { $error }
archive-count-packs-and-assets = 팩 { $packs }개와 에셋 { $assets }개
archive-count-packs = 팩 { $packs }개
archive-count-assets = 에셋 { $assets }개
archive-count-nothing = 항목 0개

## ArchiveView.vue

archive-load-failed = 보관함을 불러오지 못했습니다: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = { $what }를 복원했습니다
archive-purged = { $what }를 영구 삭제했습니다
archive-purge-failed = 삭제하지 못했습니다: { $error }
archive-purge-title = { $name }을(를) 영구 삭제할까요?
archive-purge-pack-body = 이 팩의 에셋 { $count }개와 파일 { $size }도 함께 삭제됩니다. 다른 팩과 공유하는 파일은 제외됩니다. 되돌릴 수 없습니다.
archive-purge = 영구 삭제
archive-purge-asset-body = 다른 에셋과 공유하지 않는 한 파일이 삭제됩니다. 되돌릴 수 없습니다.
archive-empty-title = 보관함을 비울까요?
archive-empty-body = { $what }가 영구 삭제되고 파일이 지워집니다. 되돌릴 수 없습니다.
archive-empty = 보관함 비우기
archive-title = 보관함
archive-intro = 보관한 팩과 에셋은 검색, 개수, 태그 목록에서 빠집니다. 복원하거나, 영구 삭제해서 공간을 확보하세요.
archive-loading = 불러오는 중…
archive-is-empty = 보관함이 비어 있습니다
archive-is-empty-hint = 팩이나 에셋을 보관하면 복원하거나 영구 삭제할 때까지 여기에 있습니다.
archive-packs = 팩
archive-assets = 에셋
archive-asset-count = 에셋 { $count }개
archive-archived = { $when } 보관됨
archive-from = 출처: { $pack }
archive-restore = 복원
archive-show-more = 더 보기({ $count }개 남음)

## mine.ts

projects-favorite-add-failed = 즐겨찾기에 추가하지 못했습니다: { $error }
projects-favorite-remove-failed = 즐겨찾기에서 제거하지 못했습니다: { $error }
projects-save-search-failed = 검색을 저장하지 못했습니다: { $error }
projects-forget-search-failed = { $name }을(를) 제거하지 못했습니다: { $error }
projects-change-search-failed = { $name }을(를) 변경하지 못했습니다: { $error }

## projects.ts

projects-list-and = { $items } 및 { $last }
projects-list-separator = ,{ " " }
projects-use = { $name }({ $files })
projects-use-more = 그 밖의 { $count }개
projects-other = 다른 프로젝트 { $count }개
projects-usage-title-one = { $name }에서 사용 중입니다
projects-usage-title = 프로젝트 { $count }개에서 사용 중입니다
projects-usage-body-one = { $list }에서 사용 중입니다. 다음 hoard sync 때 빠집니다.
projects-usage-body = { $list }에서 사용 중입니다. 각 프로젝트의 다음 hoard sync 때 빠집니다.

## uploader.ts

upload-network-error = 네트워크 오류
upload-gave-up = { $what }: { $attempts }회 시도 후 { $error }
upload-failed-plain = 실패
upload-already-imported = 이미 가져옴
upload-part-failed = 업로드 실패({ $status })
upload-batch = 파일 { $count }개 묶음
upload-batch-failed = 묶음 실패({ $status })
upload-finish = 업로드 마무리

## UploadQueue.vue

upload-eta-seconds = { $s }초 남음
upload-eta-minutes = { $min }분 남음
upload-eta-hours = { $h }시간 남음
upload-queue = 가져오기
upload-importing = 팩 { $count }개 가져오는 중
upload-bytes-of = { $sent } / { $total }
upload-rate = { $size }/s
upload-waiting = 대기 중
upload-unpacking = 업로드 완료, 압축 푸는 중
upload-already = 이미 라이브러리에 있음
upload-stopped = 중지됨
upload-files-of = 파일 { $done } / { $total }개
upload-resumed = 이어서 올림, { $size }는 이미 있었음
upload-skipped = 똑같은 파일들을 이미 “{ $name }”(으)로 가져와서 건너뛰었습니다.
upload-error = { $error }. 진행 상황은 유지됩니다. 다시 시도하거나 나중에 같은 파일을 다시 놓으세요.
upload-retry = 다시 시도
# $name is the pack the library has already that these files go into.
upload-update = “{ $name }” 업데이트
upload-copy = 사본으로 가져오기
# $name is the pack the library has already that the upload goes into.
upload-into = 이미 라이브러리에 있는 “{ $name }”에 들어갑니다. 새 파일은 추가되고 변경된 파일은 교체됩니다.
upload-own-pack = 별도의 팩으로 만들기
upload-dismiss = 닫기
upload-cancel = 취소
upload-minimize = 최소화
upload-expand = 가져오기 보기
upload-pill = { $count }개 가져오는 중 · { $percent }%
upload-pill-stopped = 중지됨: { $count }
upload-interrupted = 중단됨
upload-resume = 이어서 하기
upload-resume-all = 모두 이어서 하기
upload-resume-hint-folder = 같은 폴더를 다시 고르거나 페이지에 끌어다 놓으면 멈춘 곳부터 이어서 합니다.
upload-resume-hint-file = 같은 파일을 다시 고르거나 페이지에 끌어다 놓으면 멈춘 곳부터 이어서 합니다.
upload-not-same = 같은 파일이 아니어서 아무것도 이어서 하지 않았습니다.
upload-permission-denied = 브라우저가 파일을 다시 읽도록 허용되지 않았습니다. 대신 파일을 직접 고르세요.

## PacksView.vue

packs-rename-failed = 이름을 바꾸지 못했습니다: { $error }
packs-collection-deleted = 컬렉션 { $name }을(를) 삭제했습니다
packs-delete-failed = 삭제하지 못했습니다: { $error }
packs-status-uploading = 업로드 중(지금까지 받은 파일)
packs-status-ingesting = 압축 풀고 태그하는 중
packs-status-analyzing = LLM이 태그하는 중
packs-status-ready = 준비됨
packs-status-failed = 실패
packs-retag-failed = 다시 태그하기를 시작하지 못했습니다: { $error }
packs-read-files-failed = 팩의 파일을 읽지 못했습니다: { $error }
packs-context-load-failed = 팩의 스토어 정보를 불러오지 못했습니다: { $error }
packs-undo-store-failed = 스토어 페이지를 떼어 내지 못했습니다: { $error }
packs-context-save-failed = 스토어 정보를 저장하지 못했습니다: { $error }
packs-unreal-load-failed = Unreal 변환 상태를 불러오지 못했습니다: { $error }
packs-unreal-discard-title = 원본을 버릴까요?
packs-unreal-discard-body = { $name }의 원본 .uasset 파일은 변환되면 삭제됩니다. 머티리얼 그래프처럼 그 안에 있는 Unreal 전용 데이터는 영구히 사라집니다.
packs-unreal-discard-confirm = 변환하고 원본 버리기
packs-unreal-convert-failed = 변환을 시작하지 못했습니다: { $error }
packs-unreal-fact-converted = Unreal에서 { $done }개 변환됨
packs-unreal-fact-packages = Unreal 패키지 { $left }개
packs-unreal-fact-both = Unreal에서 { $done }개 변환됨, { $left }개는 아직 패키지
packs-thumbs-canceled = { $total }개 중 { $done }개 후 취소됨
packs-thumbs-drew = 모델 { $count }개를 그렸습니다
packs-thumbs-result = { $lead }: 썸네일 { $improved }개를 교체했습니다.
packs-thumbs-result-failed = { $lead }: 썸네일 { $improved }개를 교체했고, { $failed }개는 그리지 못했습니다.
packs-archived = { $name }과(와) 에셋 { $count }개를 보관했습니다
packs-blobs-bad = 파일 { $count }개가 마지막 무결성 검사에서 실패했습니다.
packs-collections = 컬렉션
packs-browse-named = { $name } 둘러보기
packs-collection-name = 컬렉션 이름
packs-save = 저장
packs-cancel = 취소
packs-assets = 에셋 { $count }개
packs-updated = { $date } 업데이트됨
packs-browse = 둘러보기
packs-collection-download = 다운로드(.zip)
packs-collection-credits = 크레디트
packs-collection-credits-title = 이 에셋들이 속한 팩을 만든 사람, 라이선스, 요청하는 크레디트 문구
packs-rename = 이름 바꾸기
packs-delete = 삭제
packs-packs = 팩
packs-empty = 아직 팩이 없습니다. 이 페이지 아무 곳에나 zip이나 폴더를 놓아서 추가하세요.
packs-find = 팩 찾기
packs-find-placeholder = 이름, 스토어, 라이선스, 태그로 팩 찾기
packs-sort = 순서
packs-sort-new = 최신순
packs-sort-old = 오래된 순
packs-sort-size = 큰 순
packs-sort-assets = 에셋 많은 순
packs-sort-count = 에셋 수
packs-sort-smallest = 작은 순
packs-sort-fewest = 에셋 적은 순
packs-with-kind = 포함된 종류
packs-state = 상태
packs-state-any = 모든 상태
packs-state-working = 처리 중
packs-state-failed = 실패
packs-state-untagged = 아직 설명 없음
packs-state-unlicensed = 라이선스 기록 없음
packs-store = 스토어
packs-store-any = 모든 스토어
packs-store-from = { $store }에서 받음
packs-store-none = 스토어 페이지 없음
packs-shown-of = { $total }개 중 { $shown }개
packs-clear = 지우기
packs-no-match = 일치하는 팩이 없습니다.
packs-clear-all = 검색어와 필터 지우기
packs-pack-name = 팩 이름
packs-thumbs-drawing = 이 브라우저에서 썸네일 그리는 중…
packs-thumbs-drawing-count = 이 브라우저에서 썸네일 그리는 중: { $total }개 중 { $done }개…
packs-thumbs-stopped = 중지됨: { $error }
packs-thumbs-none = 여기 있는 모든 모델에 이미 모든 텍스처를 입힌 썸네일이 있습니다.
packs-dismiss = 닫기
packs-license = 라이선스: { $license }
packs-license-tier = 라이선스: { $license }({ $tier })
packs-license-not-found = 찾을 수 없음
packs-from-before = 출처:
packs-from-after = { "" }
packs-added = { $date } 추가됨
pack-panel-label = 이 팩 정보
pack-panel-hide = 팩 세부 정보 숨기기
pack-panel-kinds = 포함된 항목
pack-panel-all = 모든 팩
packs-download = 팩 다운로드(.zip)
packs-store-details = 스토어 정보
packs-more-named = { $name } 추가 작업
packs-more = 더 보기
packs-rename-more = 이름 바꾸기…
packs-retag-rules = 오프라인 태거 다시 실행
packs-retag-llm = LLM 태거 다시 실행
packs-make-thumbs = 이 브라우저에서 썸네일 만들기
packs-make-thumbs-title = 보이지 않는 3D 뷰어에서 모델을 하나씩 열고, 서버의 썸네일보다 나은 경우 그 그림을 썸네일로 사용합니다
packs-make-thumbs-title-desktop = 보이지 않는 3D 뷰어에서 모델을 하나씩 열고, Hoard가 만든 썸네일보다 나은 경우 그 그림을 썸네일로 사용합니다
packs-read-files = 팩의 파일 다시 읽기
packs-credits = 크레디트 다운로드
packs-credits-title = CREDITS.md: 팩을 만든 사람, 라이선스, 요청하는 크레디트 문구
packs-unreal-open = Unreal 콘텐츠 변환…
packs-archive = 팩 보관
packs-unreal-intro = Hoard는 Unreal 패키지를 직접 읽어서 안에 든 것을 일반 파일로 저장합니다. 텍스처는 PNG 또는 EXR, 사운드는 WAV, 머티리얼은 JSON으로 저장합니다. 변환된 파일은 패키지를 대신하며 태그, 링크, 컬렉션을 그대로 유지합니다.
packs-unreal-converted = 변환됨
packs-unreal-converted-kept = 변환됨, { $kept }개는 원본 유지
packs-unreal-pending = 아직 확인 안 함
packs-unreal-needs-editor = Unreal Editor 대기 중
packs-unreal-unsupported = 공통 형식 없음
packs-unreal-failed = 실패
packs-unreal-reasons = 패키지가 남은 이유
packs-unreal-editor-before = 메시와 애니메이션에는 Unreal Editor가 필요합니다. Unreal이 설치된 컴퓨터에서 다음을 실행하세요:
packs-unreal-editor-command = hoard unreal-worker --editor <UnrealEditor-Cmd 경로>
packs-unreal-editor-after = . FBX로 내보내서 결과를 여기에 업로드합니다.
packs-unreal-discard = 변환 후 원본 .uasset 파일 버리기
packs-unreal-discard-hint = 공간이 확보됩니다. 머티리얼 그래프 같은 Unreal 전용 데이터는 사라집니다.
packs-unreal-drop = 공통 형식이 없는 패키지 삭제
packs-unreal-drop-hint = 블루프린트, 레벨 및 기타 엔진 전용 에셋입니다.
packs-unreal-convert = 변환
packs-context-hint-before = 팩의 출처와 판매자의 설명입니다. LLM 태거가 모든 파일과 함께 이 내용을 읽으므로, "120 ice and fire spells" 같은 스토어 설명이 있으면 다음과 같은 파일의 이름을 짓는 데 도움이 됩니다:
packs-context-hint-after = .
packs-store-auto = 자동으로 찾음: 팩 자체에 이 페이지가 적혀 있습니다.
packs-store-not-this = 이게 아님
packs-lookup-label = 스토어에서 검색할 내용
packs-lookup-placeholder = 스토어에서의 팩 이름
packs-lookup-searching = 검색 중…
packs-lookup-search = 스토어 검색
packs-lookup-hide = 숨기기
packs-lookup-clues = 팩의 파일에서 찾은 단어: { $clues }.
packs-lookup-also = { $queries }도 검색했습니다.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Fab, itch.io, Kenney, OpenGameArt에서 비슷한 것을 찾지 못했습니다. 단어를 줄이거나 바꿔 보세요.
packs-lookup-use = 사용
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = 현재
packs-review-store = 스토어 내용
packs-review-same = 같음
packs-review-read-page = 저장할 때 페이지에서 읽어옴
packs-review-take =
    { $count ->
       *[other] 필드 { $count }개 채우기
    }
packs-lookup-searched = { $date }에 검색했습니다.
packs-lookup-again = 다시 검색
packs-lookup-again-after = .
packs-lookup-unreachable = 이번에는 { $stores }에 연결할 수 없었습니다.
packs-lookup-also-look = 다음에서도 찾아보세요:
packs-lookup-links-and = { " " }및{ " " }
packs-lookup-browser-only = (브라우저에서만 검색할 수 있습니다). 페이지 주소를 아래에 붙여 넣으세요.
packs-lookup-open = 스토어에서 찾기…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = 판매자
packs-seller-placeholder = 만든 사람
packs-license-label = 라이선스
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = 라이선스 등급
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
       *[other] 에셋 { $count }개에 자체 라이선스가 있음: 팩의 라이선스를 따르게 하기
    }
packs-store-page = 스토어 페이지
packs-store-text = 스토어 설명
packs-store-text-placeholder = 판매 페이지의 텍스트를 붙여 넣거나, 비워 두면 스토어 페이지에서 가져옵니다.
packs-store-text-fetch = 저장하면 스토어 페이지에서 설명을 가져옵니다.
packs-readme = 팩에서 찾은 README
packs-save-retag = 저장하고 LLM 태거 다시 실행

## AssetDetail.vue

asset-lazy-3d-viewer = 3D 뷰어
asset-lazy-text = 텍스트
asset-lazy-text-viewer = 텍스트 뷰어
asset-lazy-image-viewer = 이미지 뷰어
asset-lazy-material-preview = 머티리얼 미리보기
asset-lazy-material-viewer = 머티리얼 뷰어
asset-lazy-sprite-player = 스프라이트 플레이어
asset-lazy-sheet-view = 스프라이트 시트 뷰어
asset-rel-texture = 텍스처
asset-rel-buffer = 지오메트리 데이터
asset-rel-material = 머티리얼 파일
asset-rel-animation = 애니메이션
asset-rel-library = 연결된 파일
asset-rel-guess = 함께 쓰는 것으로 보임
asset-rel-sidecar = 스토어 정보
asset-rel-variant = 변형
asset-rel-sprite = 스프라이트
asset-rel-sample = 샘플
asset-role-map = { $role } 맵
asset-maps-count = 맵 { $count }개
asset-needs = 필요한 파일
asset-animations = 애니메이션
asset-sprites = 스프라이트
asset-used-by = 사용하는 곳
asset-loose-sprites = 별도 파일로도 있음
asset-in-sheet = 스프라이트 시트에 포함
asset-other-formats = 다른 형식
asset-shown-as = 라이브러리에 표시되는 형식
asset-conversion-failed = 변환 실패
asset-as-converting = 변환 중…
asset-as-converting-percent = 변환 중… { $percent }%
asset-as-queued = 대기 중…
asset-download = 다운로드
asset-as-label = 다른 형식으로 다운로드
asset-as-original = { $format }(원본 형식)
asset-as-pack-files = 팩에 있는 파일
asset-as-size = 크기
asset-as-full-size = 원래 크기
asset-as-up-to = 최대 { $size }px
asset-as-hint = 크기나 다른 형식을 고르세요
asset-meta-size = 크기
asset-meta-length = 길이
asset-meta-sample-rate = 샘플 레이트
asset-meta-channels = 채널
asset-meta-mono = 모노
asset-meta-stereo = 스테레오
asset-meta-bit-depth = 비트 심도
asset-meta-bits = { $bits }비트
asset-meta-level = 레벨
asset-meta-level-value = 피크 { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = 피크 { $peak } dBFS, RMS { $rms } dBFS, 클리핑 있음
asset-meta-loop = 루프
asset-meta-loop-value = { $start } ~ { $end }
asset-meta-playback = 재생
asset-meta-one-shot = 원샷
asset-meta-envelope = 엔벨로프
asset-meta-envelope-times = 어택 { $attack }, 테일 { $tail }
asset-meta-envelope-hit = 타격
asset-meta-envelope-swell = 점점 커짐
asset-meta-envelope-sustained = 지속
asset-meta-tone = 음색
asset-meta-tone-value = 밝기 { $khz } kHz, 노이즈 { $noise }%
asset-meta-pitch = 음높이
asset-meta-pitch-rises = { $octaves }옥타브 상승
asset-meta-pitch-falls = { $octaves }옥타브 하강
asset-meta-loops-cleanly = 끊김 없이 반복
asset-meta-tempo = 템포
asset-meta-tempo-heard = 약 { $bpm } BPM, 청취 기준
asset-meta-tempo-heard-or = 약 { $bpm } BPM (또는 { $alt }), 청취 기준
asset-meta-key = 조성
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, 청취 기준
music-key-major = { $note } 장조
music-key-minor = { $note } 단조
asset-meta-root-note = 근음
asset-meta-cue-markers = 큐 마커
asset-meta-library = 라이브러리
asset-meta-keywords = 키워드
asset-meta-title = 제목
asset-meta-artist = 아티스트
asset-meta-album = 앨범
asset-meta-genre = 장르
asset-meta-generator = 생성 도구
asset-meta-everything-with = { $tag } 태그가 있는 항목 모두 보기
asset-meta-saved-with = 저장 프로그램
asset-meta-color-mode = 색상 모드
asset-meta-layers = 레이어
asset-meta-range = 범위
asset-meta-hdr = HDR(미리보기는 톤 매핑됨)
asset-meta-format = 형식
asset-meta-large-document = 대형 문서(PSB)
asset-meta-ai = AI
asset-meta-ai-made = 생성형 AI로 만듦
asset-meta-ai-edited = 생성형 AI로 편집함
asset-meta-ai-how-metadata = 메타데이터
asset-meta-ai-how-generator = 생성 도구 설정
asset-meta-ai-how-iptc = IPTC 디지털 소스 유형
asset-meta-ai-says = { $made }({ $how }에 따름)
asset-meta-ai-by-says = { $made }({ $by }, { $how }에 따름)
asset-meta-provenance = 출처 정보
asset-meta-c2pa = C2PA 콘텐츠 자격 증명 있음
asset-meta-compression = 압축
asset-meta-note = 메모
asset-meta-maps = 맵
asset-meta-sizes = 크기
asset-meta-triangles = 삼각형
asset-meta-vertices = 버텍스
asset-meta-meshes = 메시
asset-meta-materials = 머티리얼
asset-meta-animations = 애니메이션
asset-meta-skins = 스킨
asset-meta-unreal-asset = Unreal 에셋
asset-meta-unreal-saved-with = { $class }, UE { $engine }에서 저장
asset-meta-package = 패키지
asset-meta-cooked = 쿠킹됨(게임 빌드): 미리보기와 원본이 제거되어 있음
asset-meta-tiling = 타일링
asset-meta-seamless = 양쪽 축 모두 이음새 없음
asset-meta-analysis-error = 이 파일을 완전히 읽지 못했습니다: { $error }
asset-copy-blocked = 복사하지 못했습니다: 브라우저가 막았습니다
asset-save-error = 저장하지 못했습니다: { $error }
asset-tag-removed = { $tag } 태그를 제거했습니다
asset-tag-restored = { $tag } 태그를 되돌렸습니다
asset-tags-added = 태그 { $count }개를 추가했습니다
asset-tags-accepted = 추측 태그 { $count }개를 유지했습니다
asset-reviewed = 검토함
asset-reviewed-on-title = 검토함: 규칙과 AI는 태그를 건드리지 않습니다. 클릭하면 표시 해제(R)
asset-reviewed-off-title = 검토함으로 표시해 규칙과 AI가 태그를 건드리지 않게 하기(R)
asset-reviewed-on = 검토함으로 표시했습니다
asset-reviewed-off = 검토함 표시를 해제했습니다
# $name is an asset's name.
asset-reviewed-on-named = { $name }을(를) 검토함으로 표시했습니다
asset-reviewed-off-named = { $name }의 검토함 표시를 해제했습니다
asset-undo = 실행 취소
asset-play-error = 이 파일을 재생하지 못했습니다: { $error }
asset-copied-hash = 해시를 복사했습니다
asset-copied-toml = toml을 복사했습니다
asset-copied-id = ID를 복사했습니다
asset-copied-link = 링크를 복사했습니다
asset-origin-before = 원래 파일:
asset-origin-class-engine = (UE { $engine }에서 저장한 { $class }).
asset-origin-class = ({ $class }).
asset-origin-engine = (UE { $engine }에서 저장).
asset-origin-end = { "." }
asset-origin-editor = { $date }에 Unreal Editor가 내보냈습니다.
asset-origin-editor-notes = { $date }에 Unreal Editor가 내보냈습니다({ $notes }).
asset-origin-hoard = { $date }에 Hoard가 패키지에서 읽어 냈습니다.
asset-origin-hoard-notes = { $date }에 Hoard가 패키지에서 읽어 냈습니다({ $notes }).
asset-origin-download = 원본 .{ $ext } 다운로드
asset-origin-discarded = 원본 패키지는 변환 후 버렸습니다.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = 기록
history-loading = 불러오는 중…
history-none = 아직 변경된 내용이 없습니다.
history-load-failed = 기록을 불러오지 못했습니다: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = AI
history-by-rule = 규칙
history-by-someone = 누군가
history-by-app = { $who }, 앱 이용
history-by-assistant = { $who }, 어시스턴트 이용
history-field-description = 설명
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = { $tag } 태그 붙임
history-tag-removed = { $tag } 태그 뗌
history-tag-kept = 제안된 { $tag } 태그 유지
history-tag-changed = { $tag } 태그 변경
history-reviewed = 검토함으로 표시
history-unreviewed = 검토 안 함으로 표시
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: { $to }(으)로 설정
history-cleared = { $field }: 비움(이전 값 { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = 되돌리기
history-revert-title = 이 변경으로 바뀐 내용을 되돌립니다
history-reverted = 되돌림: { $change }
history-revert-failed = 되돌리지 못했습니다: { $error }
asset-details = 에셋 세부 정보
asset-in-pack = ·
asset-previous = 이전 에셋
asset-next = 다음 에셋
asset-favorite = 즐겨찾기
asset-favorite-on-title = 즐겨찾기에 있음(F로 제거)
asset-favorite-off-title = 즐겨찾기에 추가(F)
asset-share = 공유
asset-share-title = 이 에셋으로 가는 링크 공유(여는 사람은 먼저 Hoard에 로그인합니다)
asset-layout-panel = 사이드 패널에 표시
asset-layout-page = 전체 페이지로 표시
asset-layout-page-title = 전체 페이지로 표시: 세부 정보 옆에 큰 미리보기
asset-close = 세부 정보 닫기
asset-rename-title = 클릭해서 이름 바꾸기
asset-name = 이름
asset-views-alt = 네 방향에서 본 모델
asset-inspect-aria = { $name } 전체 화면으로 검사
asset-inspect-title = 검사: 확대, 타일링, 채널
asset-playback-position = 재생 위치
asset-playback-value = { $duration } 중 { $current }
asset-pause = 일시 정지
asset-play = 재생
asset-waveform-drawing = 파형 그리는 중…
asset-waveform-skipped = 이렇게 큰 파일은 파형을 표시하지 않습니다
asset-play-animation = 애니메이션으로 재생
asset-show-views = 태거가 본 네 방향 시트 표시
asset-palette = 그림의 색상
asset-swatch-title = { $hex }, 그림의 { $percent }%. 이 색상이 있는 에셋 찾기
asset-swatch-aria = { $hex }이(가) 있는 에셋 찾기, 이 그림의 { $percent }%
asset-more-like-this = 비슷한 것 더 보기
asset-desc-label = 내 설명
asset-desc-placeholder = 무엇인지, 어디에 쓸지…
asset-save = 저장
asset-cancel = 취소
asset-desc-family = 변형 { $count }개 모두에 저장됩니다
asset-desc-edit = 설명 편집
asset-desc-write = 설명 쓰기
asset-desc-ai = AI가 작성함
asset-desc-file = 파일에서 가져옴
asset-desc-store-before = 출처:
asset-desc-store-page = { $store } 페이지
asset-desc-store-page-unnamed = 스토어 페이지
asset-desc-store-after = (팩 전체)
asset-less = 접기
asset-more = 더 보기
asset-tags = 태그
asset-tag-add-label = 태그 추가
asset-tag-add-placeholder = 태그 추가, 예: mood:tense
asset-apply-family = 변경 사항을 변형 { $count }개 모두에 적용
asset-tags-accept-all = 추측 { $count }개 모두 유지
asset-tags-accept-all-title = 규칙과 AI가 추측한 태그를 모두 내 태그로 유지
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = 제외한 태그:
asset-tag-restore-title = { $tag } 태그 되돌리기
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = 비슷한 에셋의 태그:
asset-tag-suggest-title = { $tag } 추가
asset-details-heading = 세부 정보
asset-field-category = 분류
asset-field-license = 라이선스
asset-field-source = 소스
asset-field-rating = 평점
# Shown in place of an empty value, to fill it in.
asset-field-add = 추가
asset-field-new = 필드 추가
asset-field-name = 필드 이름
asset-field-value = 값
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = 팩에서 가져옴
asset-field-license-placeholder = CC0, CC-BY 4.0 같은 형식
asset-field-remove-placeholder = 비우면 제거
# $field is a field's name, like "License".
asset-field-edit-title = { $field } 편집
asset-field-saved = { $field }을(를) 저장했습니다
asset-field-cleared = { $field }을(를) 지웠습니다
asset-rate = 별 { $count }개
asset-rated = 별 { $count }개로 평가했습니다
asset-rating-clear-title = 다시 클릭하면 평점 지우기
asset-rating-cleared = 평점을 지웠습니다
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
       *[other] 다시 클릭하면 별 { $count }개
    }
asset-collections = 컬렉션
asset-collection-remove = { $name }에서 제거
asset-collection-add-another = 다른 컬렉션에 추가
asset-collection-add = 컬렉션에 추가
asset-used-in = 사용 중인 곳
asset-used-in-title = { $name }에 있는 것 보기
asset-used-in-synced = { $when } 동기화됨
asset-maps = 맵
asset-show-more = { $count }개 더 보기
asset-materials-heading = 머티리얼
asset-material-maps-aria = { $name }: 맵
asset-variants = 변형({ $count }가지)
asset-fit-animations = 맞는 애니메이션 ({ $count })
asset-fit-characters = 맞는 캐릭터 ({ $count })
asset-copies = 동일한 사본
asset-copies-note = 같은 바이트는 한 번만 저장됩니다. 이 팩들에도 이 파일이 들어 있습니다.
asset-near-copies = 거의 같은 사본
asset-near-copies-note = 크기, 샘플 레이트 또는 형식만 다른 같은 것입니다. 가장 좋은 사본에 표시가 있습니다.
asset-near-copies-best = 크기, 샘플 레이트 또는 형식만 다른 같은 것입니다. 이것이 가장 좋은 사본입니다.
asset-near-best = 최상
asset-unreal-converted = Unreal에서 변환됨
asset-rule-converted = 규칙으로 변환됨
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = { $date }에 변환 규칙이 이 { $format } 파일을 만들었습니다. 원본:
asset-rule-kept-until = 원본은 { $date }까지 보관됩니다.
asset-rule-rebuilt = FLAC에 WAV의 모든 내용이 보관되어 있으므로 언제든 WAV를 원래 모습 그대로 다운로드하거나 되돌릴 수 있습니다.
asset-rule-put-back = 원본 되돌리기
asset-rule-put-back-title = { $file }을(를) 되돌릴까요?
asset-rule-put-back-body = 변환된 파일은 삭제되고 원본이 태그, 컬렉션, 링크와 함께 그 자리로 돌아옵니다. 그 후로는 변환 규칙이 이 파일을 건드리지 않습니다.
asset-rule-put-back-done = { $file }을(를) 되돌렸습니다
asset-rule-let-go = 설정에서 정한 원본 보관 기간이 지나서 원본이 삭제되었습니다.
asset-unreal-package = Unreal 패키지에서 가져옴
asset-ue-needs-editor-before = Unreal Editor 대기 중: { $reason }. Unreal이 설치된 컴퓨터에서 다음을 실행하면
asset-ue-needs-editor-after = FBX로 돌아옵니다.
asset-ue-editor-path = UnrealEditor-Cmd 경로
asset-ue-unsupported = 패키지로 보관됨: { $reason }.
asset-ue-failed = 변환 실패: { $reason }
asset-recovered = 원본 .{ $ext } 파일이 이 패키지 안에 저장되어 있어서 Hoard가 꺼냈습니다.
asset-recovered-download = .{ $ext } 다운로드
asset-ue-meshes-before = 메시와 애니메이션은 Unreal 자체 형식으로 저장되어 있습니다. 팩을 변환한 다음(팩 페이지, 더 보기), Unreal Editor가 설치된 곳에서 다음 명령을 실행하면 FBX를 얻을 수 있습니다:
asset-ue-meshes-after = { "" }
asset-ue-registry-tags = 에셋 레지스트리 태그 { $count }개
asset-file = 파일
asset-bytes = 바이트
asset-source = 소스
asset-path = 경로
asset-path-open-folder = { $folder }에 있는 다른 항목 보기
asset-download-maps = 맵 { $count }개 모두 다운로드
asset-download-linked = 연결된 파일 { $count }개와 함께 다운로드
asset-download-only-title = 사용하는 파일 없이 이 파일만
asset-download-only = 이 파일만
asset-include-animations = .zip에 애니메이션 포함
asset-include-character = 캐릭터 { $name } 포함
asset-copy-toml = hoard.toml 항목 복사
asset-copy-id = ID 복사
asset-teach = 이 파일을 Hoard에 가르치기
asset-teach-title = 이 스토어 파일을 설명해서 Hoard가 읽을 수 있게 합니다(설정 → 스토어 파일)
asset-archive = 보관
asset-archive-title = 보관함에서 복원하거나 영구 삭제할 때까지 검색에서 빠집니다

## ImageViewer.vue

image-channel-red = 빨강 채널
image-channel-green = 초록 채널
image-channel-blue = 파랑 채널
image-channel-alpha = 알파 채널
image-channel-title = { $channel }({ $key })
image-bg-checker-title = 체커 배경(K로 전환)
image-bg-black-title = 검정 배경(K로 전환)
image-bg-gray-title = 회색 배경(K로 전환)
image-bg-white-title = 흰색 배경(K로 전환)
image-bg-checker = 체커 배경
image-bg-black = 검정 배경
image-bg-gray = 회색 배경
image-bg-white = 흰색 배경
image-inspect-aria = { $name } 검사
image-preview-size = 미리보기 { $width }×{ $height }, { $bytes }
image-preview = 미리보기
image-resolution = 해상도
image-full-title = 전체 { $width }×{ $height } 그림 불러오기(F). 미리보기 이상으로 확대해도 불러옵니다
image-full-loading = 원래 해상도 불러오는 중…
image-full-failed = 원래 해상도를 불러오지 못했습니다. 다시 시도
image-full = 원래 해상도
image-zoom = 확대/축소
image-zoom-out-title = 축소(−)
image-zoom-out = 축소
image-zoom-in-title = 확대(+)
image-zoom-in = 확대
image-fit-title = 맞추기(0)
image-fit = 맞추기
image-actual-title = 실제 픽셀(1)
image-view = 보기
image-tile-title = 3×3으로 타일링해서 이음새 확인(T)
image-tile = 타일링
image-channels = 채널
image-close = 닫기(Esc)
image-failed = 이 이미지를 불러오지 못했습니다.
image-loading = 불러오는 중…
image-key-gestures = 스크롤하거나 핀치해서 확대/축소 · 드래그해서 이동 · 더블클릭해서 확대 또는 맞추기
image-key-tile = 타일링
image-key-background = 배경
image-key-full = 원래 해상도
image-key-channels = 채널

## SpritePlayer.vue

sprite-frame-aria = { $name }, 프레임 { $frame }/{ $count }
sprite-failed = 재생할 수 없습니다.
sprite-loading = 프레임 불러오는 중…
sprite-previous = 이전 프레임
sprite-next = 다음 프레임
sprite-pause = 애니메이션 일시 정지
sprite-play = 애니메이션 재생
sprite-animation = 애니메이션
sprite-all-frames = 모든 프레임
sprite-grid-title = 시트를 프레임으로 자르는 방법
sprite-columns = 열
sprite-rows = 행
sprite-fps-aria = 초당 프레임 수
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
       *[other] 스프라이트 { $count }개
    }
sheet-pages =
    { $count ->
       *[other] { $count }페이지
    }
sheet-animations =
    { $count ->
       *[other] 애니메이션 { $count }개
    }
# The box that narrows the list of sprites by name.
sheet-find = 스프라이트 찾기
# Under the list of sprites when more match than it draws.
sheet-showing = { $total }개 중 { $shown }개 표시
sheet-no-match = 이름에 그 내용이 들어간 스프라이트가 없습니다.
sheet-failed = 이 시트를 읽을 수 없습니다.
sheet-loading = 시트 불러오는 중…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoard 전용 형식: 페이지는 그대로 두고, 스프라이트는 어느 방향으로든 회전하고 여백을 잘라 내며, 애니메이션과 각 프레임의 시간을 담습니다.
sheet-note-texturepacker = 스프라이트를 시계 방향으로 회전하고 여백을 잘라 내며, 애니메이션을 이름별로 담아 PixiJS와 Phaser에서 읽을 수 있습니다.
sheet-note-clockwise = 스프라이트를 시계 방향으로 회전하고 여백을 잘라 냅니다. 애니메이션은 담을 수 없어 프레임은 번호가 붙은 이름으로 구분됩니다.
sheet-note-libgdx = 스프라이트를 시계 반대 방향으로 회전하고 여백을 잘라 냅니다. 애니메이션은 담을 수 없어 프레임은 번호가 붙은 이름으로 구분됩니다.
sheet-note-godot-frames = 스프라이트는 회전하지 않고, 잘라 낸 여백은 마진으로 남기며, 애니메이션과 각 프레임의 시간을 담습니다.
sheet-note-godot-tileset = 타일은 회전하거나 잘라 내지 않고 타일 크기의 격자에 배치하며, 이미 격자로 되어 있는 이미지는 그대로 둡니다.
sheet-note-grid = 모든 스프라이트를 바로 세워 캔버스 전체 그대로 모두 같은 크기의 칸에 담습니다. 시트를 격자로 자르는 도구라면 무엇이든 읽을 수 있습니다.
sheet-note-frames = 모든 스프라이트를 바로 세워 캔버스 전체 그대로, 각각 그 이름을 딴 PNG로 저장합니다.

## TextViewer.vue, TextPreview.vue, text.ts

text-lang-config = 설정 파일
text-lang-table = 표
text-lang-code = 코드
text-lang-script = 스크립트
text-lang-text = 텍스트
text-copy-blocked = 복사하지 못했습니다: 브라우저가 막았습니다
text-read-aria = { $name } 읽기
text-lines = { $count }줄
text-show-as = 표시 방식
text-tree-title = 접을 수 있는 트리(T)
text-tree = 트리
text-pretty-title = 들여쓰기 정리(P)
text-pretty = 정리
text-raw-title = 작성된 그대로(R)
text-raw = 원문
text-document-title = 문서로(D)
text-document = 문서
text-source-title = 작성된 그대로의 마크다운(R)
text-source = 소스
text-page-title = 브라우저가 폴더에서 열었을 때처럼, Hoard와 분리해서 (P)
text-page = 페이지
text-page-source-title = 작성된 그대로의 페이지 (R)
text-page-aria = 페이지로 본 { $name }
text-page-error = 이 페이지를 열 수 없습니다: { $error }
text-fold = 접기
text-expand-all-title = 모두 펼치기
text-expand-all = 모두 펼치기
text-collapse-all-title = 모두 접기
text-collapse-all = 모두 접기
text-group-text = 텍스트
text-wrap-title = 긴 줄 바꿈(W)
text-wrap = 줄 바꿈
text-copy-title = 텍스트 복사(C)
text-copied = 복사됨
text-copy = 복사
text-download-title = 파일 다운로드
text-download = 다운로드
text-close = 닫기(Esc)
text-truncated = { $total } 중 처음 { $shown }만 표시합니다.
text-truncated-json = { $total } 중 처음 { $shown }만 표시합니다(트리에는 파일 전체가 필요합니다).
text-show-all = 모두 표시
text-json-error = 이 JSON을 해석할 수 없습니다: { $error }
text-contents-aria = { $name } 내용
text-loading = 불러오는 중…
text-load-error = 이 파일을 불러오지 못했습니다: { $error }
text-binary = 이 파일은 텍스트가 아닌 바이너리라서 여기서 읽을 내용이 없습니다. 다운로드해서 해당 도구로 여세요.
text-key-tree = 트리
text-key-pretty = 정리
text-key-raw = 원문
text-key-document = 문서
text-key-source = 소스
text-key-page = 페이지
text-key-wrap = 줄 바꿈
text-key-copy = 복사
text-key-find = 찾기
text-key-close = 닫기
text-preview-aria = { $name } 전체 화면으로 읽기
text-preview-read = 읽기

## JsonTree.vue, JsonPicker.vue

json-items = 항목 { $count }개
json-keys = 키 { $count }개
json-fold = 접기
json-unfold = 펼치기
json-unfold-summary = { $summary } 펼치기
json-show-more = { $count }개 더 보기
json-left = (남은 { $count }개 중)
json-picker-more = … { $count }개 더

## 3D 모델 뷰어

model-viewer-failed = 3D 뷰어를 불러오지 못했습니다({ $error }). 페이지를 새로 고치면 해결될 수 있습니다
model-loading-viewer = 3D 뷰어 불러오는 중…
model-loading = 모델 불러오는 중…
model-show-failed = 이 모델을 표시하지 못했습니다: { $error }
model-preview-failed = 이 모델을 미리 보지 못했습니다: { $error }
model-error-converting = 서버에서 아직 이 모델을 변환하는 중입니다
model-error-converting-desktop = Hoard가 아직 이 모델을 변환하는 중입니다
model-error-empty-collada = 빈 Collada 파일입니다
model-error-no-loader = .{ $ext }용 브라우저 로더가 없습니다
model-dialog = { $name } 3D로 보기
model-title-stats = 삼각형 { $triangles }개 · 메시 { $meshes }개 · 머티리얼 { $materials }개
model-triangles = 삼각형 { $count }개
model-group-camera = 카메라
model-group-shading = 셰이딩
model-group-background = 배경
model-group-animation = 애니메이션
model-group-view = 보기
model-group-show = 표시
model-frame-title = 모델에 맞추기(F)
model-frame = 맞추기
model-reset-title = 처음 시점으로(R)
model-reset = 초기화
model-turn-title = 천천히 회전(A)
model-turn = 회전
model-textures-title = 텍스처(T)
model-textures = 텍스처
model-full-res-title = 원래 해상도 텍스처(H): 켜면 크기와 상관없이 텍스처 파일을 그대로, 끄면 최대 2048px의 가벼운 미리보기를 사용합니다
model-full-res = 원래 해상도
model-light-title = 환경 조명(E)
model-light = 조명
model-grid-title = 바닥 그리드(G)
model-grid = 그리드
model-bg-checker = 체커 배경
model-bg-dark = 어두운 배경
model-bg-gray = 회색 배경
model-bg-light = 밝은 배경
model-bg-env = 환경 배경
model-bg-title = { $background }(K로 전환)
model-pause-title = 일시 정지(Space)
model-play-title = 재생(Space)
model-pause = 일시 정지
model-play = 재생
model-animation-clip = 애니메이션 클립
model-animation = 애니메이션
model-plays-on = 재생할 캐릭터
model-rest-pose = 기본 포즈
model-close-title = 닫기(Esc)
model-full-screen = 전체 화면
model-open-full-screen = 전체 화면 3D 뷰어 열기
model-menu-fold = 메뉴 접기
model-menu-show = 메뉴 표시
model-view-prefix = 보기:{ " " }
model-menu-render = 렌더
model-menu-maps = 맵
model-menu-inspect = 검사
model-mode-final = 최종 렌더
model-mode-raw = 기본 렌더
model-mode-color = 베이스 컬러
model-mode-roughness = 러프니스
model-mode-metalness = 메탈니스
model-mode-normal = 노멀
model-mode-ao = 오클루전
model-mode-matcap = 매트캡
model-mode-uv = UV 체커
model-mode-geometry = 지오메트리
model-mode-title = { $mode }({ $key })
model-wireframe-title = 와이어프레임(W), 모든 보기 위에 표시
model-wireframe = 와이어프레임
model-info-title = 장면 정보(I)
model-info = 장면 정보
model-info-close = 장면 정보 닫기
model-info-close-title = 닫기(I)
model-info-geometries = 지오메트리
model-info-materials = 머티리얼
model-info-textures = 텍스처
model-info-triangles = 삼각형
model-info-vertices = 버텍스
model-info-meters = glTF는 미터 단위입니다
model-info-no-units = 이 형식은 단위를 지정하지 않습니다
model-info-height = 높이
model-info-length = 길이
model-info-width = 너비
model-length-units = { NUMBER($n, maximumSignificantDigits: 3) } 단위
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = 텍스처를 찾을 수 없음: { $name }
model-missing-some = 텍스처 { $count }개를 찾을 수 없음: { $names }
model-missing-more = 텍스처 { $count }개를 찾을 수 없음: { $names }…
model-missing-count = 텍스처 { $count }개를 찾을 수 없음
model-guessed = 파일에 텍스처 이름이 없어서 이름으로 텍스처를 짝지었습니다.
model-group-facing = 방향
model-facing-turn = 90° 회전
model-facing-turn-title = 위에서 볼 때 반시계 방향으로 모델을 90° 회전
model-facing-tip = 90° 기울이기
model-facing-tip-title = 모델 윗부분을 앞쪽으로 90° 기울여 누워 있는 모델을 세우기
model-facing-save = 이 팩에 적용
model-facing-save-title = { $pack }의 모든 모델을 이렇게 돌립니다. 썸네일은 다시 그려지고, 다운로드는 만들어진 그대로 유지됩니다.
model-facing-undo = 실행 취소
model-facing-undo-title = 팩 모델의 원래 방향으로 되돌리기
model-facing-saved = 이제 { $pack }의 모든 모델이 이 방향을 향합니다. 썸네일을 다시 그리는 중입니다.
model-facing-failed = 팩의 모델을 돌리지 못했습니다: { $error }
model-keys-mouse = 드래그해서 궤도 회전 · 오른쪽 드래그해서 이동 · 스크롤해서 확대/축소 · 더블클릭해서 맞추기
model-key-frame = 맞추기
model-key-reset = 초기화
model-key-view = 보기
model-key-wireframe = 와이어프레임
model-key-info = 장면 정보
model-key-textures = 텍스처
model-key-full-res = 원래 해상도 텍스처
model-key-light = 조명
model-key-grid = 그리드
model-key-background = 배경
model-key-play = 재생

## 머티리얼

material-role-base-color = 색상
material-role-normal = 노멀
material-role-normal-dx = 노멀(DirectX)
material-role-roughness = 러프니스
material-role-gloss = 글로스
material-role-metalness = 메탈니스
material-role-ao = 앰비언트 오클루전
material-role-orm = 오클루전, 러프니스, 메탈
material-role-mra = 메탈, 러프니스, 오클루전
material-role-rma = 러프니스, 메탈, 오클루전
material-role-height = 높이
material-role-bump = 범프
material-role-cavity = 캐비티
material-role-specular = 스페큘러
material-role-opacity = 불투명도
material-role-emissive = 이미시브
material-role-translucency = 반투명
material-role-base-color-name = 색상
material-role-normal-name = 노멀
material-role-normal-dx-name = 노멀(DirectX)
material-role-roughness-name = 러프니스
material-role-gloss-name = 글로스
material-role-metalness-name = 메탈니스
material-role-ao-name = 앰비언트 오클루전
material-role-orm-name = 오클루전, 러프니스, 메탈
material-role-mra-name = 메탈, 러프니스, 오클루전
material-role-rma-name = 러프니스, 메탈, 오클루전
material-role-height-name = 높이
material-role-bump-name = 범프
material-role-cavity-name = 캐비티
material-role-specular-name = 스페큘러
material-role-opacity-name = 불투명도
material-role-emissive-name = 이미시브
material-role-translucency-name = 반투명
material-maps = 맵 { $count }개
material-dialog = 머티리얼 { $name } 보기
material-up-to = 최대 { $size }
material-repeat-size = 1회 반복 { $size }
material-shape-sphere = 구
material-shape-cube = 정육면체
material-shape-plane = 평면
material-shape-title = { $shape }({ $key })
material-group-shape = 모양
material-group-tiling = 타일링
material-real-title = 실제 크기: 1회 반복이 { $size }, 구의 지름은 2 m(T로 전환)
material-repeat-title = { $n }회 반복(T로 전환)
material-real = 실제
material-repeat = { $n }×
material-height-title = 높이 맵이 표면을 얼마나 밀어낼지
material-height = 높이
material-height-strength = 높이 강도
material-loading = 머티리얼 불러오는 중…
material-show-failed = 이 머티리얼을 표시하지 못했습니다: { $error }
material-preview-failed = 이 머티리얼을 미리 보지 못했습니다: { $error }
material-group-maps = 맵
material-map-show = { $map } 맵 표시
material-map-hide = { $map } 맵 숨기기
material-map-unused = 미리보기에서 사용하지 않음
material-open-full-screen = 전체 화면 머티리얼 뷰어 열기
material-keys-mouse = 드래그해서 궤도 회전 · 오른쪽 드래그해서 이동 · 스크롤해서 확대/축소 · 맵을 클릭해서 끄기
material-key-shape = 모양
material-key-tiling = 타일링
material-key-turn = 회전

## theme.ts

theme-slate = 슬레이트
theme-paper = 페이퍼
theme-linen = 리넨
theme-sage = 세이지
theme-lilac = 라일락
theme-blush = 블러시
theme-graphite = 그래파이트
theme-obsidian = 옵시디언
theme-ember = 엠버
theme-moss = 모스
theme-aubergine = 오버진
theme-oxblood = 옥스블러드
theme-light-slate-blurb = 차가운 청회색
theme-light-paper-blurb = 무난한 중성 회색
theme-light-linen-blurb = 따뜻한 크림색, 호박색
theme-light-sage-blurb = 부드러운 초록
theme-light-lilac-blurb = 연한 보라
theme-light-blush-blurb = 차분한 장미색
theme-dark-graphite-blurb = 색조 없는 중성 회색
theme-dark-slate-blurb = 청회색
theme-dark-obsidian-blurb = OLED용 검정에 가까운 색
theme-dark-ember-blurb = 따뜻한 숯색, 호박색
theme-dark-moss-blurb = 숲 같은 초록빛 검정
theme-dark-aubergine-blurb = 자두빛 검정, 라일락
theme-dark-oxblood-blurb = 와인빛 검정, 코럴

## texnames.ts

texnames-needs-map = 패턴에는 { "{" }map{ "}" }이 있어야 합니다. 그렇지 않으면 머티리얼의 모든 맵이 같은 이름을 갖게 됩니다
texnames-placeholders = 패턴에는 { "{" }name{ "}" }, { "{" }map{ "}" }, { "{" }size{ "}" }를 사용할 수 있습니다
texnames-bad-characters = 이름에는 < > : " / \ | ? * 를 쓸 수 없습니다
texnames-no-suffix = { $role } 맵에 접미사를 지정하세요
texnames-bad-suffix = { $role } 맵의 접미사에는 < > : " / \ | ? * 를 쓸 수 없습니다
texnames-slot-no-suffix = “{ $slot }”에 접미사를 지정하세요
texnames-slot-bad-suffix = “{ $slot }”의 접미사에는 < > : " / \ | ? * 를 쓸 수 없습니다
texnames-ending-missing = 직접 지정한 끝부분마다 찾을 끝부분을 입력해야 합니다
texnames-ending-bad = 직접 지정한 끝부분에는 < > : " / \ | ? * 를 쓸 수 없습니다
texnames-ending-twice = { $ending }이(가) 끝부분 목록에 두 번 있습니다
texnames-slot-metallic-roughness = 메탈과 러프니스
texnames-slot-specular-color = 스페큘러 색상
texnames-slot-specular-glossiness = 스페큘러와 글로시니스
texnames-slot-clearcoat = 클리어코트
texnames-slot-clearcoat-roughness = 클리어코트 러프니스
texnames-slot-clearcoat-normal = 클리어코트 노멀
texnames-slot-sheen-color = 광택 색상
texnames-slot-sheen-roughness = 광택 러프니스
texnames-slot-transmission = 투과
texnames-slot-thickness = 두께
texnames-slot-diffuse-transmission = 확산 투과
texnames-slot-diffuse-transmission-color = 확산 투과 색상
texnames-slot-iridescence = 이리데선스
texnames-slot-iridescence-thickness = 이리데선스 두께
texnames-slot-anisotropy = 이방성

## storeFormats.ts

storefiles-field-title = 제목
storefiles-field-title-hint = 판매 페이지의 정확한 제목입니다. 팩의 이름이 되며, 없으면 파일을 건너뜁니다.
storefiles-field-seller = 판매자
storefiles-field-seller-hint = 만들거나 파는 사람입니다.
storefiles-field-license = 라이선스
storefiles-field-license-hint = 파일에 적힌 그대로입니다. 예: CC-BY-4.0
storefiles-field-url = 스토어 페이지
storefiles-field-url-hint = http:// 또는 https://로 시작하는 판매 페이지 주소입니다.
storefiles-field-words = 태그
storefiles-field-words-hint = 태그로 붙일 단어입니다. 쉼표로 구분된 값은 나눕니다.
storefiles-field-type = 종류
storefiles-field-type-hint = “3D model”이나 “surface”처럼 무엇인지 나타냅니다. 태그로도 붙습니다.
storefiles-field-colors = 색상
storefiles-field-colors-hint = color: 태그로 붙일 색상입니다.
storefiles-field-id = 스토어 ID
storefiles-field-id-hint = 스토어에서 쓰는 ID입니다. 문자, 숫자, _ 또는 -.
storefiles-field-origin = 제작지
storefiles-field-origin-hint = 만들거나 스캔한 곳으로, 설명에 들어갑니다.
storefiles-field-size = 크기
storefiles-field-size-hint = 크기를 말로 나타낸 것으로, 설명에 들어갑니다.
storefiles-field-title-name = 제목
storefiles-field-seller-name = 판매자
storefiles-field-license-name = 라이선스
storefiles-field-url-name = 스토어 페이지
storefiles-field-words-name = 태그
storefiles-field-type-name = 종류
storefiles-field-colors-name = 색상
storefiles-field-id-name = 스토어 ID
storefiles-field-origin-name = 제작지
storefiles-field-size-name = 크기

## StoreFormatsPanel.vue

storefiles-load-failed = 스토어 파일 형식을 불러오지 못했습니다: { $error }
storefiles-remove-title = { $name }을(를) 제거할까요?
storefiles-remove-body = 데이터 폴더에서 { $file }이(가) 삭제됩니다. 이미 이 형식으로 읽은 팩은 읽은 내용을 유지합니다.
storefiles-remove = 제거
storefiles-removed = { $name }을(를) 제거했습니다.
storefiles-remove-failed = { $name }을(를) 제거하지 못했습니다: { $error }
storefiles-unread = Hoard가 읽지 않는 파일
storefiles-loading = 불러오는 중…
storefiles-unread-none = 없음: 라이브러리에 읽지 않은 스토어 파일처럼 보이는 파일이 없습니다.
storefiles-unread-what = 둘 이상의 팩에 있거나 스토어 파일 같은 이름인데, 어떤 형식도 읽지 않는 파일입니다.
storefiles-unread-packs = 팩 { $count }개
storefiles-unread-example = 예: { $pack }
storefiles-teach = 이 파일을 Hoard에 가르치기
storefiles-formats = 형식
storefiles-formats-what = 내가 만든 형식을 먼저 시도하며, 같은 이름의 기본 제공 형식을 대신합니다.
storefiles-dir-before = 보관 위치:
storefiles-dir-after = { "" }
storefiles-yours = 내 형식
storefiles-built-in = 기본 제공
storefiles-edit = 편집
storefiles-start-from = 이것으로 시작

## TagsPanel.vue

tags-load-failed = 태그를 불러오지 못했습니다: { $error }
tags-rename-title = { $tag }의 이름을 바꿀까요?
tags-merge-title = 태그 { $count }개를 병합할까요?
tags-rename-body = 이 태그가 붙은 에셋 { $count }개에 대신 { $into }이(가) 붙고, 앞으로 태거도 새 이름을 사용합니다.
tags-merge-body = 최대 에셋 { $count }개에서 { $tags }이(가) { $into }(으)로 바뀝니다. 앞으로 태거는 { $into }을(를) 사용합니다.
tags-rename = 이름 바꾸기
tags-merge = 병합
tags-merged = 이제 에셋 { $count }개에 { $into } 태그가 붙어 있습니다
tags-merge-failed = 병합하지 못했습니다: { $error }
tags-forget-failed = 변경하지 못했습니다: { $error }
tags-find = 태그 찾기
tags-find-placeholder = 태그 찾기, 예: rock 또는 set:
tags-sort = 정렬
tags-sort-count = 많이 쓰인 순
tags-sort-name = 이름순
tags-count = 태그 { $count }개
tags-count-more = 태그 { $count }개 이상
tags-what = 같은 뜻의 태그를 골라서 병합하거나, 하나를 골라서 이름을 바꾸세요.
tags-new-name = 새 이름
tags-merge-into = 병합 대상
tags-saved-as-before = 저장된 이름:
tags-saved-as-after = { "" }
tags-cancel = 취소
tags-loading = 불러오는 중…
tags-none = 태그가 없습니다.
tags-none-containing = “{ $text }”이(가) 들어간 태그가 없습니다.
tags-browse-title = { $tag } 태그가 붙은 에셋 둘러보기
tags-browse = 둘러보기
tags-merged-heading = 병합된 태그
tags-merged-what = 태거가 이 옛 이름 중 하나를 내놓으면, 에셋에는 병합된 태그가 붙습니다.
tags-allow-old = 옛 이름 다시 허용
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count = 에셋 { $count }개
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
       *[other] 이전 이름: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag }이(가) { $into }에 병합되었습니다.
tag-page-show = { $tag } 보기
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = 이 태그의 의미를 적어 보세요…
tag-page-describe-label = 이 태그의 의미
tag-page-describe-placeholder = 어떤 용도인지, 무엇이 여기에 속하는지 적어 주세요. Enter로 저장합니다.
tag-page-describe-edit = 클릭해서 변경
tag-page-description-saved = 태그의 의미를 저장했습니다
# The tag's picture: one of its assets stands for it.
tag-page-picture = 이 태그의 이미지 고르기
tag-page-picture-help = 이 태그의 에셋 중 하나를 골라 대표로 삼으세요
tag-page-picture-nothing = 표시된 에셋 중에 이미지가 있는 것이 없습니다.
tag-page-picture-none = 이미지 없음
tag-page-picture-saved = 이미지를 설정했습니다
tag-page-picture-removed = 이미지를 해제했습니다
tag-page-rename = 이름 바꾸기…
tag-page-rename-title = 새 이름이나, 병합할 다른 태그의 이름을 입력하세요
tag-page-rename-label = 새 이름
tag-page-save-failed = 태그를 저장하지 못했습니다: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = 상위 태그
tag-page-parent-add = 아래에 넣기…
tag-page-parent-add-title = 이 태그를 더 넓은 태그 아래에 넣으세요. 그 태그를 검색하면 이 태그도 함께 찾아요
tag-page-parent-placeholder = 더 넓은 태그, 예: 탈것
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = { $tag }을(를) { $parent } 아래에서 빼기
tag-page-parent-saved = { $tag }이(가) 이제 { $parent } 아래에 있습니다
tag-page-parent-removed = { $tag }이(가) 더 이상 { $parent } 아래에 있지 않습니다
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = 하위 태그:
tag-page-narrower-title = { $tag }을(를) 검색하면 이 태그도 함께 찾아요
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = 상위 및 하위
tags-tree-what = 태그 페이지에서 더 넓은 태그 아래에 넣은 태그입니다. 태그를 검색하면 그 아래에 있는 모든 것도 함께 찾아요.

## LibraryStats.vue

stats-source-human = 사람이 추가함
stats-source-ai = AI가 제안함
stats-source-rule = 기본 규칙
stats-source-meta = 파일에서 가져옴
stats-assets = 에셋
stats-packs = 팩
stats-on-disk = 디스크 사용량
stats-duplicates-title = 파일 { $count }개가 다른 파일과 바이트 단위로 똑같아서 한 번만 저장되었습니다
stats-saved-dedup = 중복 제거로 절약한 공간
stats-distinct-tags = 서로 다른 태그
stats-archive = 보관함에 있음
stats-load-failed = 세부 정보를 불러오지 못했습니다: { $error }
stats-counting = 세는 중…
stats-as-of = { $time } 기준
stats-updating = 업데이트 중…
stats-by-kind = 종류별 공간
stats-files-size = { $name }: 파일 { $count }개, { $size }
stats-months = 지난 1년 동안 추가됨
stats-month = { $month }: 에셋 { $count }개, { $size }
stats-tagging = 태깅
stats-tagged = 태그 있음
stats-untagged = { $count }개는 태그 없음
stats-described = 설명 있음
stats-described-assets = 에셋 { $count }개
stats-human-edited = 직접 편집함
stats-human-edited-unit = 에셋
stats-source-different = 서로 다른 태그 { $count }개
stats-most-used = 가장 많이 쓰인 태그
stats-show-tag = { $tag } 에셋 보기
stats-file-types = 파일 형식
stats-biggest-packs = 가장 큰 팩
stats-browse-pack = { $name } 둘러보기
stats-largest-files = 가장 큰 파일
stats-open-file = { $name } 열기
stats-empty = 라이브러리가 아직 비어 있습니다. 팩을 추가하면 여기에 수치가 표시됩니다.

## HoardPanel.vue

hoard-title = 나의 보물 더미
hoard-what = 라이브러리의 모든 에셋은 보물 한 조각이며, 종류에 따라 어떤 보물인지 정해집니다. 새것을 모으면 내 더미에 더해집니다.
hoard-sign-in = 로그인하면 나만의 보물 더미를 모을 수 있습니다.
hoard-new = 모을 수 있는 새 전리품: { $count }
hoard-none-new = 새로 모을 것이 없습니다.
hoard-collect = 모으기
hoard-unavailable = 이 브라우저에서는 보물 더미를 그릴 수 없습니다: { $error }
hoard-collect-failed = 모은 것을 기록하지 못했습니다: { $error }
hoard-arcade = 아케이드 화면
hoard-full-screen = 전체 화면
hoard-leave-full-screen = 전체 화면 종료
hoard-label = 보물 더미를 지키는 용. 보물: { $count }
hoard-gold = 금
hoard-gems = 보석
hoard-books = 책
hoard-weapons = 무기
hoard-bones = 뼈
hoard-tablets = 룬 석판
hoard-paintings = 그림
hoard-potions = 물약

## UpdatesPanel.vue

updates-saved = 업데이트가 준비되면 Hoard가 이 주소를 호출합니다
updates-saved-none = 호출할 주소 없음
updates-save-failed = 저장하지 못했습니다
updates-call-answered = 응답: { $result }
updates-call-refused = 요청을 받지 않았습니다: { $result }
updates-call-failed = 호출하지 못했습니다
updates-called = { $when }에 { $version } 때문에 호출함: { $result }
updates-called-failed = { $when }에 { $version } 때문에 호출함: { $result }. Hoard가 1시간 뒤에 다시 시도합니다.
updates-latest = Hoard { $version }이(가) 최신 릴리스입니다.
updates-out = Hoard { $version }이(가) 나왔습니다.
updates-out-idle = Hoard { $version }이(가) 나왔습니다. 진행 중인 작업이 없으면 몇 분 안에 호출합니다.
updates-out-busy = Hoard { $version }이(가) 나왔습니다. 진행 중인 작업이 없을 때까지 기다립니다({ $work }).
updates-busy-more = 그 밖의 { $count }개
updates-busy-jobs = 대기 중인 작업 { $count }개
updates-busy-uploads = 업로드 { $count }개
updates-list-separator = ,{ " " }
updates-what = 새 릴리스가 나오면 Hoard는 진행 중인 작업(업로드, 작업, 변환, 유지 관리)이 없을 때 주소를 호출할 수 있습니다. 그래서 컨테이너를 업데이트하는 도구가 업로드 도중에 Hoard를 다시 시작하지 않습니다. 릴리스 정보를 JSON 본문에 담아 POST로 보냅니다. 위의 릴리스 확인이 켜져 있어야 합니다.
updates-url = 호출할 주소
updates-url-placeholder = 없음(예: http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = 저장됨(입력하면 교체)
updates-auth-placeholder = 없음(예: Bearer my-token)
updates-remove = 제거
updates-save = 저장
updates-call-now-title = 연결된 것이 실행됩니다
updates-call-now = 지금 호출
updates-what-calls = 무엇을 호출할 수 있나요?
updates-watchtower = Watchtower HTTP API
updates-watchtower-at = :{ " " }
updates-watchtower-with = ,{ " " }
updates-watchtower-token = { " " }및{ " " }
updates-watchtower-its = 의 토큰 사용(
updates-watchtower-endpoint = { " " }엔드포인트를 켜야 함).
updates-portainer = Portainer 웹훅
updates-portainer-rest = : Hoard의 스택 또는 컨테이너에 만들고 이미지 다시 받기(re-pull image)를 켭니다. 토큰은 URL에 들어 있습니다.
updates-script = 호스트의 스크립트
updates-script-listener = : 웹훅 리스너(예:{ " " }
updates-script-runs = ) 뒤에 두고 다음을 실행합니다:{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }또는 웹훅을 받는 그 밖의 무엇이든.
updates-poll-before = 주소 없이 하는 방법도 있습니다. 액세스 토큰이 있는 스크립트가{ " " }
updates-poll-until = 을(를) 폴링하다가{ " " }
updates-poll-and = 이(가) 설정되고{ " " }
updates-poll-after = 이(가) true가 되면 새 버전을 받아서 Hoard를 다시 시작할 수 있습니다.

## MaintenancePanel.vue

maintenance-load-failed = 유지 관리 정보를 불러오지 못했습니다
maintenance-reanalyze-title = 모든 팩을 다시 분석할까요?
maintenance-reanalyze-body = 모든 파일을 다시 읽으므로 라이브러리가 크면 오래 걸릴 수 있습니다. 차례가 될 때까지 팩은 처리 중으로 표시됩니다. 직접 정한 이름, 태그, 메모는 유지됩니다.
maintenance-reanalyze-confirm = 다시 분석
maintenance-gc-title = 사용하지 않는 파일을 삭제할까요?
maintenance-gc-body = 라이브러리에서 아무것도 가리키지 않는 저장 파일이 영구 삭제됩니다. 보관함에 있는 팩의 파일은 유지됩니다.
maintenance-gc-confirm = 삭제
maintenance-start-failed = “{ $task }”을(를) 시작하지 못했습니다
maintenance-just-now = 방금
maintenance-minutes-ago = { $count }분 전
maintenance-hours-ago = { $count }시간 전
maintenance-took-seconds = { $count }초
maintenance-took-minutes = { $count }분
maintenance-took-hours = { $hours }시간
maintenance-loading = 불러오는 중…
maintenance-running = 실행 중…
maintenance-starting = 시작 중…
maintenance-run = 실행
maintenance-stop = 중지
maintenance-stopping = 중지하는 중…
maintenance-stop-failed = “{ $task }” 작업을 중지하지 못했습니다
maintenance-started = { $detail } · { $when } 시작
maintenance-starting-detail = 시작 중
maintenance-finished = 완료:{ " " }
maintenance-failed = 실패:{ " " }
maintenance-never-run = 실행한 적 없음
maintenance-history = 지난 실행
maintenance-col-task = 작업
maintenance-col-finished = 완료
maintenance-col-took = 소요 시간
maintenance-col-result = 결과
maintenance-script-before = 백업과 손상 검사는 스크립트에서도 실행할 수 있습니다:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }및{ " " }
maintenance-script-after = . 이렇게 실행한 작업도 여기에 표시됩니다.

## SettingsPage.vue

settings-title = 설정
settings-loading = 불러오는 중…
settings-find = 설정 찾기
settings-find-none = “{ $query }”에 맞는 설정이 없습니다.
settings-load-failed = 설정을 불러오지 못했습니다: { $error }
settings-page-appearance = 모양
settings-page-appearance-blurb = 언어, 테마, 글자 크기와 간격, 타일과 상단 막대에 보이는 것. 이 브라우저에만 저장됩니다.
settings-page-dragon = 드래곤
settings-page-dragon-blurb = 당신의 드래곤이 모은 보물: 라이브러리의 에셋 하나하나가 그 보물입니다.
settings-page-library = 라이브러리
settings-page-library-blurb = Hoard에 무엇이 있는지: 크기, 구성, 늘어난 과정, 태그가 얼마나 잘 붙었는지. 보관한 항목은 세지 않습니다.
settings-page-copies = 거의 같은 사본
settings-page-copies-blurb = 크기, 샘플 레이트 또는 형식만 다른 같은 그림, 소리, 모델 파일입니다. 공간을 가장 많이 차지하는 것부터 보여 줍니다. 각 묶음에서 가장 좋은 것에 표시가 있으며, 아무것도 삭제하지 않습니다.
copies-loading = 거의 같은 사본을 찾는 중…
copies-refreshing = 찾은 이후 라이브러리가 바뀌었습니다. 다시 찾는 중…
copies-none = 거의 같은 사본이 없습니다. 모든 그림, 소리, 모델이 한 번씩만 있습니다.
copies-load-failed = 거의 같은 사본을 나열하지 못했습니다: { $error }
copies-sum = 거의 같은 사본 묶음: { $count }개. 가장 좋은 것이 아닌 사본이 차지하는 공간: { $size }.
copies-set = 묶음 { $n }
copies-set-wasted = 가장 좋은 것이 아닌 사본: { $size }
copies-more = 더 보기 ({ $count }개 남음)
settings-page-tags = 태그
settings-page-tags-blurb = 라이브러리의 모든 태그입니다. 같은 뜻의 태그를 병합하거나 이름을 바꾸세요. 그 뒤로 태거는 새 이름을 사용합니다.
settings-page-store-files = 스토어 파일
settings-page-store-files-blurb = 스토어가 작품 옆에 넣어 두는 파일로, Hoard가 읽어서 팩의 이름을 짓고 만든 사람을 표시합니다. 모르는 파일을 가르치고, 가르친 내용을 공유하세요.
settings-page-locations = 라이브러리 위치
settings-page-locations-blurb = 라이브러리를 담는 서버의 폴더입니다. Hoard가 파일을 보관하는 폴더와, 제자리에서 색인하는 팩 폴더가 있습니다.
settings-page-locations-blurb-desktop = 라이브러리를 담는 폴더입니다. Hoard가 파일을 보관하는 폴더와, 제자리에서 색인하는 팩 폴더가 있습니다.
settings-page-tools = 도구
settings-page-tools-blurb = Hoard가 파일을 변환하고 읽을 때 쓰는 외부 프로그램입니다.
settings-page-ai = AI 태깅
settings-page-ai-blurb = 기본 규칙에 더해 AI 모델이 새 팩의 이름을 짓고, 설명하고, 태그를 붙입니다. AI가 붙인 태그는 제안으로 표시됩니다.
settings-page-search-blurb = 이름뿐 아니라 그림은 나타난 내용으로, 사운드는 들리는 소리로 검색하고, 색상뿐 아니라 종류가 비슷한 것을 찾습니다.
settings-page-search-tab = 검색
settings-page-adding = 팩 추가
settings-page-adding-blurb = 팩이 추가될 때 읽고 태그하는 것 말고 Hoard가 하는 일입니다.
settings-page-downloads = 다운로드
settings-page-downloads-blurb = Hoard가 보내는 파일(다운로드, zip, hoard 도구, AI 어시스턴트)의 이름을 짓는 방식입니다. 라이브러리의 파일은 그대로 둡니다.
settings-page-account = 계정
settings-page-account-blurb = 내 비밀번호, 로그인할 수 있는 다른 사람, 스크립트와 다른 프로그램용 액세스 토큰입니다.
settings-page-auth = 인증
settings-page-auth-blurb = 로그인 방식: Hoard 자체 계정, LDAP 디렉터리, Microsoft Active Directory 또는 OAuth 제공자, 세션 유지 시간, 그룹별 역할입니다.
settings-page-connection = 연결
settings-page-connection-blurb = 브라우저와 앱이 Hoard에 접속하는 방식: Hoard의 주소와, 일부 브라우저 기능에 필요한 HTTPS입니다.
settings-page-performance = 성능
settings-page-performance-blurb = 서버가 얼마나 일하는지, 썸네일을 만들고 보관하는 방식, 받을 수 있는 최대 크기입니다.
settings-page-performance-blurb-desktop = Hoard가 얼마나 일하는지, 썸네일을 만들고 보관하는 방식, 받을 수 있는 최대 크기입니다.
settings-page-issues = 문제
settings-page-issues-blurb = 파일을 열거나 처리하다가 생긴 문제입니다: 풀리지 않은 압축 파일, 읽지 못한 파일, 실패한 작업. 원인을 고친 뒤 다시 시도하거나 닫으세요.
settings-page-maintenance = 유지 관리
settings-page-maintenance-blurb = 백업, 손상 검사, 정리를 합니다. 각 작업은 한 번에 하나씩 실행되며, 실행 중에 이 페이지를 떠나도 됩니다.
settings-page-about = 정보
settings-page-about-blurb = Hoard가 무엇인지, 어떤 버전인지, 그리고 Hoard를 이루는 다른 사람들의 작업.
settings-the-default = 기본값
settings-default-on = 켜짐
settings-default-off = 꺼짐
settings-default-auto = 자동
settings-option-saved = { $name }: { $value }
settings-back-to-default = 기본값으로
settings-save = 저장
settings-use = 사용
settings-status-off = 꺼짐

## SettingsPage.vue: 모양

settings-themes = 테마
settings-look-light = 밝게
settings-look-dark = 어둡게
settings-look-group =
    { $scheme ->
        [light] 밝은 테마
       *[dark] 어두운 테마
    }
settings-preview =
    확인할 수 있도록 { $preview ->
        [light] 밝은
       *[dark] 어두운
    } 테마를 표시하고 있습니다. 현재 { $used ->
        [light] 밝은
       *[dark] 어두운
    } 테마를 사용 중이며{ $system ->
        [yes] { " " }(시스템 설정을 따름)
       *[other] { "" }
    }, 이 페이지를 나가면 원래대로 돌아옵니다.
settings-preview-back =
    { $used ->
        [light] 밝은
       *[dark] 어두운
    } 테마로 돌아가기
settings-look-hint = 상단 바에서 밝게와 어둡게(또는 시스템 설정 따르기)를 전환할 수 있습니다.

## SettingsPage.vue: 도구

settings-blender-what = .blend, .dae, .abc, .x3d 모델을 백그라운드에서 glTF로 변환해서 썸네일과 3D 미리보기를 만듭니다. 꺼져 있을 때 추가된 팩은 준비되면 변환합니다.
settings-blender-download = 설치된 Blender를 사용하거나, blender.org 미러에서 Blender 4.5 LTS(약 400 MB)를 다운로드합니다.
settings-ffmpeg-what = MP3, OGG, Opus 다운로드를 인코딩하고, Hoard가 직접 디코딩할 수 없는 오디오(AAC, M4A, WMA 등)를 읽습니다.
settings-ffmpeg-download = 필요한 인코더가 있는 시스템 ffmpeg를 사용하거나, 정적 빌드(약 80 MB)를 다운로드합니다.
settings-tool-reset = { $tool }: 기본값으로
settings-tool-no-download = 설치된 { $tool }을(를) 사용합니다. 이 컴퓨터용으로 다운로드할 빌드가 없으니 설치하거나 경로를 입력하세요.
settings-tool-path = { $tool } 프로그램 경로
settings-mode-off = 끄기
settings-mode-auto = 자동
settings-mode-path = 이 프로그램
settings-tool-looking = 찾는 중…
settings-tool-downloading = 다운로드 중… 진행 상황은 작업 표시기에 있습니다
settings-tool-ready = 준비됨: { $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = MIDI 파일을 재생하고 변환할 때 쓰는 악기입니다.
settings-soundfont-file = SoundFont 파일
settings-soundfont-placeholder = 서버에서 찾거나 기본 제공
settings-soundfont-placeholder-desktop = 이 컴퓨터에서 찾거나 기본 제공
settings-soundfont-default = 서버에서 찾거나 기본 제공
settings-soundfont-default-desktop = 이 컴퓨터에서 찾거나 기본 제공
settings-soundfont-send = 파일 선택…
settings-soundfont-sent = 이제 MIDI는 { $name }(으)로 재생됩니다

## SettingsPage.vue: 이미지 검색

settings-clip-heading = 그림 내용으로 검색
settings-clip-reset = 이미지 검색: 기본값으로
settings-clip-saved-on = 이미지 검색: 켜짐
settings-clip-saved-off = 이미지 검색: 꺼짐
settings-clip-what = 찾는 것을 입력하면(“mossy rock”, “sci-fi door”, “wooden barrel”) 이름에 그 단어가 들어간 것과 함께 그 내용이 담긴 그림이 나타납니다. “비슷한 것 더 보기”는 색상뿐 아니라 종류가 같은 것도 찾습니다.
settings-clip-toggle = 이미지 검색 켜기
settings-clip-hint = Google의 SigLIP 2 모델을 이 서버의 프로세서에서 실행하므로 아무것도 밖으로 나가지 않습니다. 켜면 Hugging Face에서 모델({ $size } MB)을 다운로드한 다음, 낮은 우선순위로 백그라운드에서 모든 그림을 학습합니다. 학습하거나 누군가 검색하는 동안 메모리를 350~750 MB 사용합니다. 끄면 모델은 삭제되지만 학습한 내용은 다음을 위해 남겨 둡니다.
settings-clip-hint-desktop = Google의 SigLIP 2 모델을 이 컴퓨터의 프로세서에서 실행하므로 아무것도 밖으로 나가지 않습니다. 켜면 Hugging Face에서 모델({ $size } MB)을 다운로드한 다음, 낮은 우선순위로 백그라운드에서 모든 그림을 학습합니다. 학습하거나 검색하는 동안 메모리를 350~750 MB 사용합니다. 끄면 모델은 삭제되지만 학습한 내용은 다음을 위해 남겨 둡니다.
settings-clip-looking = 모델 찾는 중…
settings-clip-downloading = 모델 다운로드 중… 진행 상황은 작업 표시기에 있습니다
settings-clip-ready-empty = 준비됨. 팩이 추가되면 그림을 학습합니다.
settings-clip-ready-all = 준비됨: 그림 { $count }개를 모두 학습했습니다
settings-clip-learning = 준비됨: 그림 학습 중, 지금까지 { $total }개 중 { $done }개
settings-clip-relearning = 새 모델이 모든 그림을 학습할 때까지 검색은 이전 모델이 학습한 내용을 사용합니다.
settings-clip-unavailable = 모델을 가져오지 못했습니다: { $detail }. 1시간마다 다시 시도합니다.
settings-clip-where = 실행 위치
settings-clip-here = 이 서버: 아무것도 밖으로 나가지 않음
settings-clip-here-desktop = 이 컴퓨터: 아무것도 밖으로 나가지 않음
settings-clip-hosted = 호스팅 서비스
settings-clip-hosted-warning = 그림이 이 서버 밖으로 나갑니다. 라이브러리에 있는 모든 그림의 썸네일과 모든 이미지 검색어가 { $service }(으)로 전송되며, 요금이 부과되고 해당 약관에 따라 보관됩니다.
settings-clip-hosted-warning-desktop = 그림이 이 컴퓨터 밖으로 나갑니다. 라이브러리에 있는 모든 그림의 썸네일과 모든 이미지 검색어가 { $service }(으)로 전송되며, 요금이 부과되고 해당 약관에 따라 보관됩니다.
settings-clip-hosted-hint = 여기서는 아무것도 내려받거나 실행하지 않습니다. 서비스가 모든 그림을 묶음 단위로 학습합니다. 다른 서비스나 모델로 바꾸면 전부 다시 학습하며, 각각 학습한 내용은 되돌아갈 때를 위해 보관됩니다.
settings-clip-saved-here = 이미지 검색: 이 서버에서 실행
settings-clip-saved-here-desktop = 이미지 검색: 이 컴퓨터에서 실행
settings-clip-saved-hosted = 이미지 검색: { $service }에서 실행
settings-clip-hosted-unavailable = { $service }을(를) 사용하지 못했습니다: { $detail }. 1시간마다 다시 시도합니다.
settings-clap-heading = 들리는 소리로 사운드 검색
settings-clap-reset = 사운드 검색: 기본값으로
settings-clap-saved-on = 사운드 검색: 켜짐
settings-clap-saved-off = 사운드 검색: 꺼짐
settings-clap-what = 찾는 것을 입력하면(“distant thunder”, “glassy UI blip”, “footsteps on gravel”) 이름에 그 단어가 들어간 것과 함께 그렇게 들리는 사운드가 나타납니다.
settings-clap-toggle = 사운드 검색 켜기
settings-clap-hint = LAION의 CLAP 모델을 이 서버의 프로세서에서 실행하므로 아무것도 밖으로 나가지 않습니다. 켜면 Hugging Face에서 모델({ $size } MB)을 다운로드한 다음, 낮은 우선순위로 백그라운드에서 모든 사운드의 앞부분을 들어 봅니다. 학습하거나 누군가 검색하는 동안 메모리를 최대 1 GB 정도 사용합니다. 끄면 모델은 삭제되지만 학습한 내용은 다음을 위해 남겨 둡니다.
settings-clap-hint-desktop = LAION의 CLAP 모델을 이 컴퓨터의 프로세서에서 실행하므로 아무것도 밖으로 나가지 않습니다. 켜면 Hugging Face에서 모델({ $size } MB)을 다운로드한 다음, 낮은 우선순위로 백그라운드에서 모든 사운드의 앞부분을 들어 봅니다. 학습하거나 검색하는 동안 메모리를 최대 1 GB 정도 사용합니다. 끄면 모델은 삭제되지만 학습한 내용은 다음을 위해 남겨 둡니다.
settings-clap-ready-empty = 준비됨. 팩이 추가되면 사운드를 학습합니다.
settings-clap-ready-all = 준비됨: 사운드 { $count }개를 모두 학습했습니다
settings-clap-learning = 준비됨: 사운드 학습 중, 지금까지 { $total }개 중 { $done }개

## SettingsPage.vue: AI 태깅

settings-ai-service = 서비스
settings-ai-off = 끄기
settings-ai-off-hint = 기본 규칙으로만 팩에 태그를 붙입니다.
settings-ai-local = 로컬 모델
settings-ai-local-hint = Hoard의 compose 파일에 있는 llm 서비스(또는 cargo xtask llm)입니다. 아래에서 모델을 다운로드하세요. 이 컴퓨터에서 실행되며 아무것도 밖으로 나가지 않습니다.
settings-ai-chatgpt = ChatGPT 요금제
settings-ai-chatgpt-hint = API 키 대신 ChatGPT Go, Plus, Pro 또는 Business 요금제를 사용합니다.
settings-ai-openrouter-hint = 키 하나로 수백 개의 호스팅 모델을 쓸 수 있고, 일부는 무료입니다. 이미지를 받는 모델은 표시되어 있습니다.
settings-ai-openai-hint = OpenAI가 호스팅합니다.
settings-ai-anthropic-hint = Anthropic이 호스팅하며, OpenAI 호환 엔드포인트를 통해 사용합니다.
settings-ai-gemini-hint = Google이 호스팅하며, OpenAI 호환 엔드포인트를 통해 사용합니다.
settings-ai-ollama-hint = 직접 실행하는 Ollama 서버입니다. ollama라는 이름의 컨테이너가 아니면 주소를 바꾸세요.
settings-ai-custom = 기타(OpenAI 호환)
settings-ai-custom-hint = OpenAI 채팅 API를 지원하는 모든 것: LM Studio, vLLM, llama.cpp, LiteLLM 등
settings-ai-connecting = 연결 중…
settings-ai-connected = 연결됨 · 모델 { $count }개
settings-ai-connected-none = 연결되었지만 모델 목록이 없습니다. 모델 이름을 입력하세요
settings-chatgpt-connected-none = 연결됨. 모델 이름을 입력하세요
settings-ai-takes-images = 이미지 지원
settings-ai-saved = AI 태깅: { $service }
settings-ai-saved-off = AI 태깅이 꺼졌습니다
settings-ai-reset = AI 태깅: 기본값으로
settings-ai-on = 켜짐: { $service } · { $model }
settings-ai-url = 주소
settings-ai-key = API 키
settings-ai-key-saved = 저장됨(입력하면 교체)
settings-ai-key-paste = 키를 붙여 넣으세요
settings-ai-key-optional = 없음(선택 사항)
settings-ai-key-remove = 제거
settings-ai-get-key = 키 받기
settings-ai-model = 모델
settings-ai-model-pick = 모델을 고르거나 입력하세요
settings-ai-model-name = 모델 이름
settings-ai-refresh = 새로 고침
settings-ai-connect = 연결
settings-ai-need-key = 모델을 보려면 API 키를 추가하세요
settings-ai-vision = 썸네일도 보내기(이미지를 받는 모델 필요)
settings-ai-batch = 요청당
settings-ai-batch-unit = 에셋(작은 로컬 모델은 적게, 큰 호스팅 모델은 많이)

## SettingsPage.vue: ChatGPT 로그인

settings-chatgpt-account = 계정
settings-chatgpt-checking = 확인 중…
settings-chatgpt-status-as = { $email }(으)로 로그인됨
settings-chatgpt-status = 로그인됨
settings-chatgpt-signed-in-as = { $email }(으)로 ChatGPT에 로그인했습니다
settings-chatgpt-signed-in = ChatGPT에 로그인했습니다
settings-chatgpt-signed-out = ChatGPT에서 로그아웃했습니다
settings-chatgpt-continue = ChatGPT로 계속하기
settings-chatgpt-sign-out = 로그아웃
settings-chatgpt-sign-out-title = ChatGPT에서 로그아웃할까요?
settings-chatgpt-sign-out-in-use = AI 태깅이 ChatGPT 요금제를 사용하고 있으므로, 다시 로그인하거나 다른 서비스를 고를 때까지 멈춥니다.
settings-chatgpt-sign-out-body = Hoard가 로그인 정보를 잊고 OpenAI에 로그인을 끝내도록 알립니다.
settings-chatgpt-manage = 사용량 관리
settings-chatgpt-finish-before = 열린 창에서 로그인을 마치세요(
settings-chatgpt-finish-link = 다시 열기
settings-chatgpt-finish-middle = ). 마지막에 열리지 않는 페이지가 나오고 그 주소가{ " " }
settings-chatgpt-finish-after = (으)로 시작하면, 그 주소를 복사해서 여기에 붙여 넣으세요:
settings-chatgpt-paste = 로그인이 끝난 페이지의 주소
settings-chatgpt-finish = 완료
settings-chatgpt-usage = 태깅 요청은 ChatGPT 요금제 사용량에 포함됩니다. 사용량 관리에서 Hoard가 사용한 양을 보고 한도를 정할 수 있습니다. ChatGPT 쪽에서 Hoard 연결을 끊으려면 chatgpt.com에서 Settings → Security and login → Sign in with ChatGPT로 이동하세요.

## SettingsPage.vue: 로컬 모델

settings-local-downloading-progress = 다운로드 중 { $percent }% · { $total } 중 { $done }
settings-local-downloading = 다운로드 중…
settings-local-incomplete = 다운로드 중단됨
settings-local-failed = 지난번에 시작하지 못함
settings-local-in-use-running = 사용 중 · 실행 중
settings-local-in-use = 사용 중
settings-local-starting = 시작 중…
settings-local-running = 실행 중
settings-local-ready-images = 준비됨 · 이미지 지원
settings-local-ready-text = 준비됨 · 텍스트만
settings-local-delete-title = { $name }을(를) 삭제할까요?
settings-local-delete-in-use = AI 태깅이 이 모델을 사용하고 있으므로, 다른 모델을 고를 때까지 멈춥니다. 모델 파일은 모델 서버에서 삭제됩니다.
settings-local-delete-downloading = 다운로드가 중지되고 지금까지 받은 내용이 삭제됩니다.
settings-local-delete-body = 모델 파일이 모델 서버에서 삭제됩니다. 나중에 다시 다운로드할 수 있습니다.
settings-local-delete = 삭제
settings-local-deleting = 삭제 중…
settings-local-cancel = 취소
settings-local-resume = 이어받기
settings-local-download = 다운로드
settings-local-start-before = 다음 명령으로 시작하세요:{ " " }
settings-local-start-or = { " " }(개발 중에는{ " " }
settings-local-start-developing = ). 그런 다음{ " " }
settings-local-start-retry = 다시 시도
settings-local-start-after = 하세요. 모델을 하나만 실행하는 서버도 됩니다. 아래에 그 모델의 이름을 입력하세요.
settings-local-list = 모델 서버에 있음
settings-local-none = 아직 다운로드한 모델이 없습니다. 아래에서 하나 고르세요. 서버에 보관되며 언제든 바꿀 수 있습니다.
settings-local-catalog = 다운로드할 모델
settings-local-size = { $size } GB · { $license }
settings-local-other = Hugging Face의 다른 모델
settings-local-other-placeholder = Hugging Face의 다른 모델: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: 팩 추가

settings-store-pages = 스토어 페이지
settings-store-pages-what = 새 팩을 백그라운드에서 스토어에서 찾아 스토어 페이지, 판매자, 라이선스를 가져옵니다.
settings-store-lookup = 스토어 페이지 자동으로 찾기
settings-unreal = Unreal 패키지
settings-unreal-reset = Unreal 패키지: 기본값으로
settings-unreal-what = .uasset 파일을 어떻게 할지 정합니다.
settings-conv = 변환 규칙
settings-conv-what = Hoard가 보관하는 파일을 하나의 형식으로 변환해서 라이브러리를 통일합니다. 예를 들어 모든 OBJ와 FBX를 GLB로 바꿉니다. 변환된 파일은 원본을 대신하며 이름, 태그, 컬렉션, 링크를 그대로 유지합니다. 감시 폴더의 파일은 절대 바꾸지 않습니다.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = 켜기
settings-conv-to = 변환할 형식
# Heading over the file formats a rule converts.
settings-conv-from = 원본 형식
settings-conv-remove = 제거
settings-conv-remove-label = 규칙 { $n } 제거
settings-conv-taken = 다른 규칙이 이미 이 형식을 변환합니다
settings-conv-upright = 변환할 때 팩의 모델을 표시하는 방식대로 모델을 똑바로 세우기(Y 위쪽, +Z 방향)
settings-conv-flac-what = 소리는 정확히 그대로이고 공간은 절반 정도만 차지하며, 루프 지점, 큐 마커, 태그 등 WAV의 나머지 정보도 모두 FLAC 안에 보관되므로 언제든 WAV를 원래 모습 그대로 되돌릴 수 있습니다. 부동소수점 또는 32비트 샘플이 있는 WAV는 WAV로 남습니다.
settings-conv-add = 규칙 추가
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = 원본 보관 기간
settings-conv-keep-days =
    { $count ->
       *[other] 일
    }
settings-conv-keep-what = 그때까지는 변환된 파일의 페이지에서 원본을 되돌릴 수 있습니다. 텍스처는 변환된 모델에 복사되지 않으며, 모델은 옆에 있는 이미지 파일을 가리킵니다. 모델의 .mtl 또는 .bin 파일은 그대로 남습니다.
settings-conv-save = 규칙 저장
settings-conv-revert = 변경 사항 취소
settings-conv-existing = 라이브러리에 이미 있는 파일 변환
settings-conv-saved = 변환 규칙을 저장했습니다. 지금부터 추가하는 팩에 적용됩니다.
settings-conv-converting =
    { $count ->
       *[other] 백그라운드에서 파일 { $count }개 변환하는 중
    }
settings-conv-nothing = 라이브러리에 변환할 파일이 없습니다.
settings-conv-ask-title =
    { $count ->
       *[other] 라이브러리에 이미 있는 파일 { $count }개를 변환할까요?
    }
settings-conv-ask-body =
    { $packs ->
       *[other] 팩 { $packs }개에 들어 있습니다.
    } 각 원본은 { $days }일 동안 보관되며, 그때까지는 해당 페이지에서 되돌릴 수 있습니다.
settings-conv-ask-busy =
    { $count ->
       *[other] { $count }개는 아직 추가 중인 팩에 있습니다. 추가가 끝나면 변환됩니다.
    }
settings-conv-ask-list = 변환될 파일
settings-conv-ask-more = …외 { $count }개
settings-conv-ask-skipped = 그대로 둘 파일:
settings-conv-skip-watched =
    { $count ->
       *[other] 감시 폴더의 파일 { $count }개
    }
settings-conv-skip-unity =
    { $count ->
       *[other] Unity 프로젝트가 이름으로 참조하는 파일 { $count }개
    }
settings-conv-skip-used =
    { $count ->
       *[other] 장면이나 다른 파일이 이름으로 참조하는 파일 { $count }개
    }
settings-conv-skip-beside =
    { $count ->
       *[other] 변환된 사본이 이미 팩에 있는 파일 { $count }개
    }
settings-conv-skip-blender =
    { $count ->
       *[other] Blender가 아직 읽지 않은 파일 { $count }개(설정 → 도구)
    }
settings-conv-ask-new = 저장, 새 팩에만 적용
settings-conv-ask-save-convert =
    { $count ->
       *[other] 저장하고 파일 { $count }개 변환
    }
settings-conv-ask-convert =
    { $count ->
       *[other] 파일 { $count }개 변환
    }
settings-unreal-off = 그대로 두기
settings-unreal-off-hint = 원할 때 팩 페이지에서 변환하세요.
settings-unreal-keep = 변환하고 원본 유지
settings-unreal-keep-hint = 텍스처, 사운드, 머티리얼이 공통 형식으로 나오고 .uasset도 남습니다.
settings-unreal-replace = 변환하고 원본 버리기
settings-unreal-replace-hint = 내용을 꺼낸 뒤 공간을 절약합니다. 메시는 여전히 Unreal 워커를 기다립니다.

## SettingsPage.vue: 다운로드

settings-tn = 텍스처 이름
settings-tn-saved = 텍스처 이름을 저장했습니다
settings-tn-reset = 텍스처 이름: 기본값으로
settings-tn-what = 파일 이름이 어떻든 머티리얼의 맵은 머티리얼 이름과 맵의 용도에 따라 이름이 붙어서 나갑니다. rock_moss_xeokah0_raw_8K_Normal.png가 아니라 Rock_Normal.png처럼요. 라이브러리의 파일은 건드리지 않으며, 함께 보내는 모델이나 MTL이 이름을 지정한 맵은 그 이름을 유지하므로 모델이 여전히 맵을 찾을 수 있습니다.
settings-tn-toggle = 텍스처 맵 이름을 머티리얼에 맞춰 짓기
settings-tn-style = 스타일
settings-tn-style-group = 이름 스타일
settings-tn-descriptive = 설명형
settings-tn-short = 짧게
settings-tn-custom = 직접 지정
settings-tn-pattern = 패턴
settings-tn-hint-name = { " " }머티리얼 이름,{ " " }
settings-tn-hint-map = { " " }맵의 용도,{ " " }
settings-tn-hint-size = { " " }세트가 여러 크기로 제공될 때의 크기(패턴에 없으면 끝에 붙습니다).
settings-tn-example = 예시
settings-tn-suffixes = 맵 종류별 접미사
settings-tn-slots = GLB 다운로드에서 자체 파일 없이 모델 안에 들어 있는 텍스처는 머티리얼과 그 텍스처가 채우는 슬롯에 따라 이름이 붙으며, 이 슬롯에도 접미사가 있습니다:
settings-tn-endings = 직접 지정한 끝부분
settings-tn-endings-what = 머티리얼의 맵이 아닌 텍스처는 원래 이름을 유지하지만, 다음 중 하나로 끝나면 바뀝니다. _diff → _BC는 rock_diff.png를 rock_BC.png로 보냅니다. 두 번째 칸을 비워 두면 끝부분을 없앱니다.
settings-tn-ending-from = 끝부분
settings-tn-ending-from-example = _diff
settings-tn-ending-to = 바꿀 내용
settings-tn-ending-to-example = _BC
settings-tn-ending-add = 끝부분 추가
settings-tn-ending-remove = 제거
settings-tn-ending-remove-label = { $ending } 제거

## SettingsPage.vue: 일반 및 연결

settings-name = 이름
settings-name-what = 상단 바, 브라우저 탭, 로그인 페이지에 표시되는 이 서버의 이름입니다. Hoard가 여러 개일 때 구분할 수 있습니다.
settings-name-what-desktop = 상단 바에 표시되는 이 라이브러리의 이름입니다.
settings-address = 주소
settings-address-what = 이 브라우저가 쓰는 주소가 아닐 때, 사람과 앱이 Hoard에 접속하는 주소입니다. 리버스 프록시, 터널 또는 tailnet 이름(예: https://hoard.example.com)입니다. Hoard가 내주는 링크(Claude와 다른 앱, 로그인, 인증서)에 이 주소를 사용합니다.
settings-public-url = 공개 주소
settings-public-url-placeholder = 브라우저 주소 사용(예: https://hoard.example.com)
settings-public-url-default = 브라우저 주소 사용

## ProfilerPanel.vue: 설정 → 성능 맨 위의 프로파일러

profiler = 프로파일러
profiler-what = 지난 10분 동안 Hoard가 한 일로, 보고 있는 동안 2초마다 업데이트됩니다. 메모리에만 보관되며 Hoard를 다시 시작하면 처음부터 다시 시작합니다.
profiler-starting = 곧 첫 측정값이 표시됩니다.
profiler-failed = 프로파일러를 읽지 못했습니다: { $error }
profiler-cpu = 프로세서 사용량
profiler-cpu-hoard = Hoard
profiler-cpu-machine = 머신 전체
profiler-cpu-summary = Hoard는 프로세서의 { $hoard }를 사용하고 있으며, 머신 전체는 { $machine }를 사용하고 있습니다.
profiler-ram = RAM
profiler-ram-peak = 최대
profiler-ram-of = { $used } / { $total }
profiler-ram-summary = Hoard가 RAM을 { $hoard } 사용 중이며, 최근 10분 동안 최대 { $peak }를 사용했습니다.
profiler-serving = 처리 중인 요청
profiler-working = 작업 중
profiler-working-none = 백그라운드에서 실행된 작업이 없습니다.
profiler-working-more =
    { $count ->
       *[other] 외 { $count }개 더.
    }
profiler-requests = 요청 처리 시간
profiler-requests-what = 각 막대는 10초 동안의 요청이며, 그 시간을 어디에 썼는지에 따라 나눠 보여 줍니다.
profiler-part-db = 데이터베이스
profiler-part-db-title = 데이터베이스를 기다리고 쿼리를 실행하는 시간
profiler-part-disk = 디스크
profiler-part-disk-title = 저장된 파일을 읽고 보낼 내용을 압축하는 시간
profiler-part-send = 전송
profiler-part-send-title = 보낸 내용을 네트워크가 가져가기를 기다리는 시간
profiler-part-other = 기타
profiler-part-other-title = 응답을 만드는 시간
profiler-request-count =
    { $count ->
       *[other] 요청 { $count }개
    }
profiler-requests-none = 지난 10분 동안 요청이 없습니다.
profiler-slowest = 가장 느림: { $route }, { $time }
profiler-routes = 경로별
profiler-route = 경로
profiler-count = 요청
profiler-average = 평균
profiler-p95 = 가장 느린 5%
profiler-split = 시간 분포
profiler-sent = 전송량
profiler-portal = 인터페이스 자체 파일

## SettingsPage.vue: 성능

settings-render = 3D 썸네일
settings-render-toggle = 3D 모델의 썸네일 렌더링
settings-gpu-on = GPU 사용: { $renderer }
settings-gpu-software = GPU가 없어서 소프트웨어로 렌더링({ $renderer })
settings-gpu-none = GPU나 소프트웨어 렌더러가 없어서 모델에 썸네일이 생기지 않습니다
settings-thumbs = 썸네일
settings-thumbs-what = 새 썸네일과 모델 뷰에 적용됩니다. 투명한 것과 픽셀 아트는 PNG로 유지됩니다. 기존 썸네일은 유지 관리의 썸네일 줄이기로 다시 만듭니다.
settings-previews-size = 미리 보기: { $size }
settings-thumbs-saved = 썸네일: { $quality }
settings-thumbs-format = 형식
settings-thumbs-quality = 품질 { $quality }.
settings-thumbs-small = 작게
settings-thumbs-small-hint = 그리드에서는 괜찮고, 가까이 보면 약간 흐립니다.
settings-thumbs-balanced = 균형
settings-thumbs-balanced-hint = 썸네일 크기에서는 원본과 구분하기 어렵습니다.
settings-thumbs-sharp = 선명하게
settings-thumbs-sharp-hint = 균형의 약 두 배 크기입니다.
settings-thumbs-lossless = 무손실
settings-thumbs-lossless-hint = PNG, 만든 그대로입니다. 몇 배 더 큽니다. AVIF로 보내지 않습니다.
settings-thumbs-avif-what = AVIF는 보통 크기가 절반입니다. 각 썸네일은 처음 표시될 때 변환됩니다.
settings-thumbs-avif-now = 지금 모두 변환
settings-thumbs-avif-converting = 변환 중…
settings-motion-thumbs = 애니메이션 움직이는 썸네일
settings-motion-thumbs-what = 각 애니메이션에 타일을 가리키면 재생되는 짧은 반복 영상도 만듭니다. 하나에 약 40KB이고, 큰 라이브러리에서는 만드는 데 시간이 걸립니다. 끄면 삭제됩니다.
settings-limits = 작업 및 한도
settings-limits-what = 각 항목은 필드를 벗어나거나 Enter를 누르면 저장됩니다. 크기에는 단위를 씁니다: 100 GB, 512 MB, 1.5 TB. 숫자만 쓰면 GB입니다.
settings-workers = 백그라운드 작업자
settings-workers-unit = 동시 작업 수(가져오기, 태깅, 변환)
settings-max-upload = 최대 업로드
settings-max-upload-unit = 파일당
settings-max-expand = 최대 팩
settings-max-expand-unit = 압축을 푼 크기
settings-conversion-cache = 변환된 다운로드
settings-conversion-cache-unit-now = 까지 보관, 현재 { $size }. 가장 오래 쓰지 않은 것부터 지웁니다
settings-conversion-cache-unit = 까지 보관. 가장 오래 쓰지 않은 것부터 지웁니다

## SettingsPage.vue: 유지 관리

settings-updates = 업데이트
settings-version = 이 서버는 Hoard { $version }입니다.
settings-update-check = 몇 시간마다 GitHub에서 새 릴리스 확인
settings-desktop-updates = Hoard는 시작할 때마다 새 릴리스를 찾아 백그라운드에서 내려받고, 종료할 때 설치합니다.
settings-backups = 예약 백업
settings-backups-what = 데이터베이스가 스스로 백업합니다. 각 백업은 아래 기록에 표시됩니다.
settings-backup-every = 백업
settings-backup-off = 안 함
settings-backup-daily = 매일
settings-backup-weekly = 매주
settings-backup-keep = 보관
settings-backup-keep-unit = 개의 최신 예약 백업
settings-backup-copy-to = 다음에도 복사
settings-backup-copy-to-none = 다른 폴더 없음
settings-backups-kept = 각 백업은 백업 폴더에 hoard-auto-…db로 저장되고, 다른 폴더(다른 디스크나 동기화 앱이 백업하는 폴더)가 있으면 그곳에도 복사됩니다. 폴더는 서버의 전체 경로로 지정하세요. 오래된 예약 백업만 삭제되며, 직접 만든 백업은 그대로 남습니다.
about-what = Hoard는 게임 에셋(모델, 텍스처, 사운드, 음악, 스프라이트 등)을 하나의 라이브러리에 모아 검색하고 미리 보고 가져다 쓸 수 있게 하며, 접근을 허용한 모든 사람과 함께 씁니다.
about-version = 버전
about-edition = 에디션
about-edition-server = 서버
about-edition-desktop = 데스크톱 앱
about-edition-steam = Steam용 데스크톱 앱
about-made-by = 만든 곳
about-source = GitHub의 소스 코드
about-issues = 문제 신고
about-discord = Discord에서 대화하기
about-steam = Steam의 Hoard
about-license = 라이선스
about-license-what = Hoard는 MIT No Attribution 라이선스의 오픈 소스입니다. 원하는 대로 사용하고, 바꾸고, 공유할 수 있습니다.
about-downloaded = 필요할 때 내려받음
about-downloaded-what = Hoard에는 이것들이 포함되어 있지 않습니다. 처음 필요할 때 각각 내려받으며, 각각 자체 라이선스를 따릅니다.
about-blender = 3D 모델을 변환하고 썸네일을 만듭니다.
about-ffmpeg = 사운드와 동영상을 변환하고 미리 보기를 만듭니다.
about-siglip = 그림에 담긴 내용으로 그림을 찾습니다.
about-clap = 들리는 느낌으로 사운드를 찾습니다.
about-models = 로컬 AI 서버용으로 고른 모델에는 만든 사람의 라이선스가 적용됩니다.
about-built-with = 사용한 소프트웨어
about-built-with-what = Hoard를 이루는 오픈 소스 라이브러리와 그 라이선스입니다.
about-steamworks = Steam 버전에는 Steamworks SDK(© Valve Corporation)도 포함되어 있습니다.
about-licenses-loading = 목록을 불러오는 중…
about-licenses-failed = 목록을 불러올 수 없습니다.
about-filter = 이름이나 라이선스로 필터
about-count = { $shown } / { $total }
about-no-license = 라이선스 표시 없음
about-report = 문제 신고…
report-title = 문제 신고
report-what = 문제를 살펴보려면 GitHub 이슈에 필요한 정보입니다. 개인적인 내용은 없습니다. 액세스 토큰, 비밀번호, 키, 이메일 주소, 폴더 경로에 있는 사용자 이름은 제거됩니다. 보내기 전에 읽어 보세요.
report-loading = 보고서를 모으는 중…
report-failed = 보고서를 모을 수 없습니다: { $error }
report-log = 서버가 최근에 기록한 로그
report-copy = 복사
report-copied = 보고서를 복사했습니다.
report-github = GitHub에서 이슈 열기
report-github-hint = 새 이슈가 이 내용으로 채워집니다. 로그가 너무 길면 마지막 줄만 들어가며, 복사에는 전체가 담깁니다.
report-close = 닫기

## Asset kinds (api.ts)

api-kind-texture = 텍스처
api-kind-vector = 벡터 아트
api-kind-sheet = 스프라이트 시트
api-kind-audio = 오디오
api-kind-model = 모델
api-kind-animation = 애니메이션
api-kind-font = 글꼴
api-kind-shader = 셰이더
api-kind-material = 머티리얼
api-kind-video = 비디오
api-kind-document = 문서
api-kind-data = 데이터
api-kind-other = 기타
api-kind-texture-name = 텍스처
api-kind-vector-name = 벡터 아트
api-kind-sheet-name = 스프라이트 시트
api-kind-audio-name = 오디오
api-kind-model-name = 모델
api-kind-animation-name = 애니메이션
api-kind-font-name = 글꼴
api-kind-shader-name = 셰이더
api-kind-material-name = 머티리얼
api-kind-video-name = 비디오
api-kind-document-name = 문서
api-kind-data-name = 데이터
api-kind-other-name = 기타

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = 실행 취소
ui-today = 오늘
ui-yesterday = 어제
ui-days-ago = { $days }일 전
ui-notifications = 알림
ui-dismiss = 닫기
ui-cancel = 취소
ui-lazy-loading = { $what } 불러오는 중…
ui-lazy-failed = { $what }을(를) 불러오지 못했습니다.
ui-lazy-reload = 페이지 새로 고침

## ResetButton.vue

reset-title = 기본값으로 되돌리기: { $value }
reset-label = 기본값으로 되돌리기, { $value }

## SizeInput.vue

size-invalid = 100 GB, 512 MB, 1.5 TB 같은 크기
size-hint = 100 GB, 512 MB, 1.5 TB 같은 크기. 숫자만 쓰면 GB입니다

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = 인증 설정을 불러오지 못했습니다: { $error }
auth-saved = 저장했습니다. 다음 로그인부터 적용됩니다.
auth-method-internal = 내부
auth-method-internal-hint = Hoard가 보관하는 이름과 비밀번호로, 설정 → 계정에서 추가합니다.
auth-method-ldap-hint = 로그인 페이지에 입력한 이름과 비밀번호를 디렉터리가 확인합니다: OpenLDAP, FreeIPA, LLDAP 또는 다른 LDAP 서버.
auth-method-ad-hint = Windows 도메인 계정이 평소 쓰는 이름과 비밀번호로 로그인합니다.
auth-method-oauth-hint = Google, Microsoft, GitHub, Keycloak, Authentik 또는 다른 제공자로 갔다가 돌아오는 “…(으)로 로그인” 버튼입니다.
auth-session-unused = 30일 동안 사용하지 않으면
auth-session-1h = 로그인하고 1시간 뒤
auth-session-8h = 로그인하고 8시간 뒤
auth-session-day = 로그인하고 하루 뒤
auth-session-week = 로그인하고 일주일 뒤
auth-session-30d = 로그인하고 30일 뒤
auth-session-hours = 로그인하고 { $hours }시간 뒤
auth-role-view = 보기 가능
auth-role-edit = 변경 가능
auth-role-admin = 관리자
auth-role-word-view = 보기 가능
auth-role-word-edit = 변경 가능
auth-role-word-admin = 관리자
auth-role-word-default = { $role }(기본 역할)
auth-tls-starttls-hint = 일반 LDAP(포트 389)를 무엇이든 보내기 전에 TLS로 전환합니다.
auth-tls-ldaps-hint = 처음부터 TLS(포트 636)를 사용합니다.
auth-tls-none = 없음
auth-tls-none-hint = 비밀번호가 그대로 네트워크를 지나갑니다. 신뢰하는 네트워크에서만 사용하세요.
auth-test-failed = 테스트하지 못했습니다: { $error }
auth-preset-custom = 기타(OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel 등: 발급자를 입력하면 검색이 주소를 채워 줍니다.
auth-preset-google-note = Google Cloud Console → APIs & Services → Credentials에서 OAuth 클라이언트(Web application)를 만드세요. Google은 그룹을 보내지 않으므로 설정 → 계정에서 역할을 주고, Google 계정이 있는 누구나 들어와도 되는 경우가 아니라면 첫 로그인 때 계정 만들기를 끄세요.
auth-preset-microsoft-note = Entra ID → App registrations에서 앱을 등록하세요(아래 주소로 가는 Web 리디렉션). 내 조직만 들어오게 하려면 “organizations” 자리에 테넌트 ID를 넣으세요. 그룹으로 역할을 정하려면 Token configuration에서 groups 클레임을 추가하세요. 그룹 ID 목록이 들어옵니다.
auth-preset-github-note = GitHub → Settings → Developer settings에서 OAuth 앱을 만드세요. GitHub는 그룹을 보내지 않으므로 설정 → 계정에서 역할을 주고, GitHub 계정이 있는 누구나 들어와도 되는 경우가 아니라면 첫 로그인 때 계정 만들기를 끄세요.
auth-preset-gitlab-note = GitLab → Preferences → Applications(또는 자체 인스턴스의 관리 영역. 이때는 주소의 gitlab.com을 바꾸세요)에서 애플리케이션을 만드세요.
auth-discovered = 제공자의 주소를 채웠습니다.
auth-copied = 복사했습니다
auth-copy-failed = 복사하지 못했습니다. 직접 선택해서 복사하세요
auth-sign-in-with = 로그인 방법
auth-own-accounts-still-work = Hoard 자체 계정은 계속 자체 비밀번호로 로그인하므로, 디렉터리나 제공자가 멈추거나 잘못 설정되어도 잠겨서 못 들어가는 일은 없습니다.
auth-sessions = 세션
auth-sign-out-after = 로그아웃 시점
auth-ldap-server = LDAP 서버
auth-server-type = 서버 종류
auth-server-type-custom = 사용자 지정
auth-servers = 서버
auth-server-n = 서버 { $n }
auth-remove-server-n = 서버 { $n } 제거
auth-remove = 제거
auth-add-server = 서버 추가
auth-servers-hint = 서버에 연결할 수 없으면 다음 서버를 시도합니다.
auth-security = 보안
auth-connection-security = 연결 보안
auth-skip-verify = 서버 인증서를 확인하지 않음(시험용으로만)
auth-ca = 인증 기관
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (비우면 시스템이 신뢰하는 인증 기관)
auth-lookup = 사용자 조회
auth-anonymously = 익명으로
auth-reader-account = 조회용 계정
auth-reader-password = 조회용 계정 비밀번호
auth-saved-type-to-replace = 저장됨: 입력하면 교체
auth-people-under = 사용자 위치
auth-name-attribute = 이름 속성
auth-only-matching = 다음과 일치하는 항목만
auth-groups-under = 그룹 위치
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (비우면 memberOf만)
auth-their-groups = 사용자의 그룹
auth-group-filter-hint = { "{" }dn{ "}" }은 사용자의 항목, { "{" }user{ "}" }는 로그인할 때 쓴 이름입니다.
auth-test = 테스트
auth-test-hint = 각 서버에 연결하고 조회용 계정으로 로그인합니다. 이름과 비밀번호를 입력하면 그 사용자로도 로그인해서 Hoard가 어떻게 처리할지 보여 줍니다. 아무것도 저장하지 않습니다.
auth-test-user = 로그인해 볼 사용자
auth-test-user-placeholder = 로그인해 볼 사용자(선택 사항)
auth-test-password = 사용자의 비밀번호
auth-testing = 테스트 중…
auth-test-connectivity = 연결 테스트
auth-test-server-ok = 연결됨, 조회용 계정 로그인 성공
auth-test-signed-in-before = 로그인 성공:{ " " }
auth-test-signed-in-middle = { " " }(
auth-test-signed-in-after = ).
auth-test-role = 역할: { $role }.
auth-test-no-role = 역할 아래에 나열된 그룹 중 어디에도 속하지 않으므로 거부됩니다.
auth-test-groups = 그룹: { $groups }
auth-test-no-groups = 그룹을 찾지 못했습니다.
auth-provider = 제공자
auth-redirect = 리디렉션 주소
auth-copy = 복사
auth-redirect-hint = Hoard용 클라이언트를 만들 때 이 주소를 제공자에게 알려 주세요. 설정 → 연결의 공개 주소나, 지금 사용 중인 주소에서 만들어집니다.
auth-issuer = 발급자
auth-discover = 검색
auth-client-id = 클라이언트 ID
auth-client-secret = 클라이언트 시크릿
auth-authorization-url = 인증 주소
auth-token-url = 토큰 주소
auth-userinfo-url = 사용자 정보 주소
auth-userinfo-placeholder = (비우면 ID 토큰으로 사용자를 확인)
auth-scopes = 범위
auth-name-from = 이름 클레임
auth-groups-from = 그룹 클레임
auth-groups-from-placeholder = (없음)
auth-button = 버튼
auth-button-placeholder = 내 계정
auth-accounts = 계정
auth-provision = 처음 로그인할 때 Hoard 계정 만들기
auth-provision-on-hint = 끄면 관리자가 설정 → 계정에서 각 사용자를 먼저 추가합니다.
auth-provision-off-hint = 관리자가 설정 → 계정에서 로그인 방식을 골라 각 사용자를 먼저 추가합니다.
auth-open-to-anyone = { $provider } 계정이 있는 누구나 들어올 수 있습니다. 아래에 보기 그룹을 추가하거나, 이 옵션을 끄고 사용자를 직접 추가하세요.
auth-new-accounts = 새 계정
auth-roles-from-groups = 그룹별 역할
auth-roles-hint = 각 역할을 받을 그룹을 한 줄에 하나씩 이름이나 DN으로 적습니다. 일치하는 것 중 가장 높은 역할이 적용되며, 로그인할 때마다 확인합니다. 아무것도 적지 않으면 새 계정은 위의 역할을 받고 나중에 관리자가 바꿉니다. 보기 가능에 그룹을 적으면 적힌 그룹의 구성원만 로그인할 수 있습니다.
auth-roles-hint-oauth = 각 역할을 받을 그룹을 한 줄에 하나씩 이름이나 DN(또는 groups 클레임에 나오는 값)으로 적습니다. 일치하는 것 중 가장 높은 역할이 적용되며, 로그인할 때마다 확인합니다. 아무것도 적지 않으면 새 계정은 위의 역할을 받고 나중에 관리자가 바꿉니다. 보기 가능에 그룹을 적으면 적힌 그룹의 구성원만 로그인할 수 있습니다.
auth-saving = 저장 중…
auth-save = 저장

## HttpsPanel.vue (Settings → Connection)

https-load-failed = HTTPS 설정을 불러오지 못했습니다: { $error }
https-off-redirecting = HTTPS가 꺼졌습니다. http://로 이동합니다
https-now-on = HTTPS가 켜졌습니다
https-saved = 저장했습니다. 새 연결부터 새 인증서를 사용합니다.
https-now-off = HTTPS가 꺼졌습니다
https-http3-failed-toast = HTTP/3을 시작하지 못했습니다: { $error }
https-copied = 복사했습니다
https-copy-failed = 복사하지 못했습니다. 직접 선택해서 복사하세요
https-mode-off = 끄기
https-mode-off-hint = 일반 http://만 사용합니다.
https-mode-own = Hoard 자체 인증서
https-mode-own-hint = Hoard가 이 서버의 주소용 인증서를 만듭니다. 각 기기에 Hoard의 인증 기관을 한 번 설치하면 그 기기의 브라우저가 신뢰합니다.
https-mode-custom = 내 인증서
https-mode-custom-hint = 이미 가지고 있는 인증서(예: tailscale cert나 Let’s Encrypt에서 받은 것)입니다. 아무것도 설치하지 않아도 브라우저가 신뢰합니다.
https-untrusted-own = 이 브라우저는 아직 { $host }에 대해 이 인증서를 신뢰하지 않습니다. 아래에서 Hoard의 인증 기관을 설치하세요. 경고가 표시되지만 무시하고 계속할 수 있습니다.
https-untrusted-uncovered = 이 브라우저는 아직 { $host }에 대해 이 인증서를 신뢰하지 않습니다. 인증서에 이 주소가 포함되어 있지 않습니다. 경고가 표시되지만 무시하고 계속할 수 있습니다.
https-untrusted = 이 브라우저는 아직 { $host }에 대해 이 인증서를 신뢰하지 않습니다. 경고가 표시되지만 무시하고 계속할 수 있습니다.
https-now = 현재 상태
https-start-failed = HTTPS가 설정되었지만 시작하지 못했습니다: { $error }
https-on-port = HTTPS가 켜져 있으며, http://와 같은 포트 { $port }를 사용합니다.
https-using-it = 지금 사용하고 있습니다.
https-open = { $url } 열기
https-trusted = 이 브라우저가 신뢰합니다.
https-cert-for-before = 인증서 대상:{ " " }
https-cert-for-after = , { $date }까지 유효합니다.
https-cert-for-after-own = , { $date }까지 유효합니다(자동으로 갱신됨).
https-not-covered-before = 지금 사용 중인 주소{ " " }
https-not-covered-after = 은(는) 이 인증서에 포함되어 있지 않습니다.
https-is-off = HTTPS가 꺼져 있습니다. 페이지를 일반 http://로 제공합니다.
https-addresses = 주소
https-addresses-hint = 인증서에는 Hoard가 찾은 주소와 직접 추가한 주소(공유기가 붙여 준 이름, Tailscale 이름)가 포함됩니다.
https-remove-name = { $name } 제거
https-another-address = 다른 주소
https-another-address-placeholder = 다른 주소(예: nas.local)
https-add = 추가
https-your-cert = 내 인증서
https-saved-cert-before = 저장됨:{ " " }
https-saved-cert-after = { " " }· 발급자 { $issuer }, { $date }까지 유효합니다. 교체하려면 새 인증서를 붙여 넣으세요.
https-pem-before = 인증서(체인 포함)와 개인 키를 PEM 텍스트로 넣으세요. 다음으로 시작하는 파일입니다:{ " " }
https-pem-after = .
https-certificate = 인증서
https-choose-file = 파일 선택…
https-private-key = 개인 키
https-checked-before = 대상:{ " " }
https-checked-after = , 발급자 { $issuer }, { $date }까지 유효합니다.
https-checked-after-days = , 발급자 { $issuer }, { $date }까지 유효합니다({ $days }일 남음).
https-self-signed = 자체 서명 인증서입니다. 브라우저가 경고를 표시합니다.
https-key-matches = 키가 일치합니다.
https-where-to-get = 인증서를 받는 곳
https-get-tailscale-before = tailnet에서 HTTPS를 켠 다음 서버에서{ " " }
https-get-tailscale-after = { " " }명령을 실행하고 두 파일을 업로드하세요. 90일 동안 유효하므로 그 전에 다시 실행해서 다시 업로드하세요.
https-get-acme-title = Let’s Encrypt 또는 다른 ACME 클라이언트
https-get-acme-before = (certbot, acme.sh, lego): 소유한 도메인용으로 받아서{ " " }
https-get-acme-and = { " " }및{ " " }
https-get-acme-after = 을(를) 업로드하세요.
https-get-proxy-title = 리버스 프록시
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel)에 인증서가 있다면: 여기서는 HTTPS를 끄고 프록시가 Hoard에 일반 http://로 보내게 하세요.
https-trying = 시도하는 중…
https-apply = 적용
https-apply-hint = Hoard는 새 설정을 먼저 시도해 보고, 잘 안 되면 지금 설정을 유지합니다.
https-trust-title = 기기에서 Hoard 신뢰하기
https-download-ca = hoard-ca.crt 다운로드
https-trust-hint = 각 기기에 Hoard의 인증 기관을 한 번 설치하면 그 기기의 브라우저가 이 서버(와 나중에 Hoard가 이 서버용으로 만드는 모든 인증서)를 신뢰합니다. 이 인증 기관은 이 Hoard만 보증할 수 있습니다. 지문(SHA-256):
https-ca-windows = 파일 열기 → 인증서 설치 → 현재 사용자 → 모든 인증서를 다음 저장소에 저장 → 찾아보기 → 신뢰할 수 있는 루트 인증 기관 → 마침. 그런 다음 브라우저를 다시 시작하세요.
https-ca-macos = 파일을 열면 키체인 접근에 들어갑니다. 거기서 “{ $subject }”를 두 번 클릭 → 신뢰 → 이 인증서 사용 시: 항상 신뢰.
https-ca-ios-title = iPhone 및 iPad
https-ca-ios = Safari에서 다운로드하고 허용하세요. 설정 → 프로파일이 다운로드됨 → 설치. 그런 다음 설정 → 일반 → 정보 → 인증서 신뢰 설정 → “{ $subject }” 켜기.
https-ca-android = 설정 → 보안 → 기타 보안 설정 → 암호화 및 사용자 인증 정보 → 인증서 설치 → CA 인증서에서 파일을 고르세요. (메뉴 이름은 휴대폰마다 조금씩 다릅니다. 설정에서 “인증서”를 검색하세요.)
https-ca-linux-1 = Debian 및 Ubuntu:{ " " }
https-ca-linux-2 = . Fedora 및 Arch:{ " " }
https-ca-linux-3 = . Linux의 Chrome은 자체 목록을 사용합니다: 설정 → 개인정보 보호 및 보안 → 보안 → 인증서 관리 → 가져오기.
https-ca-firefox = Firefox는 자체 목록을 사용합니다: 설정 → 개인 정보 및 보안 → 인증서 → 인증서 보기 → 인증 기관 → 가져오기를 누르고 “이 인증 기관이 웹 사이트를 식별하는 것을 신뢰합니다”에 체크하세요.
https-ca-cli-title = hoard 명령줄 도구
https-ca-cli-before = { "" }
https-ca-cli-middle = { " " }명령을 쓰거나, 경로를{ " " }
https-ca-cli-after = { " " }환경 변수에 넣으세요.
https-options = 옵션
https-redirect = 다른 기기의 브라우저를 http://에서 https://로 보내기
https-redirect-hint = 페이지 열기만 전환합니다. hoard 명령줄 도구, Unreal 워커, 스크립트, AI 앱은 계속 http://로 작동하며, 서버 자체의 http://localhost도 마찬가지입니다.
https-http3-hint-before = 불안정한 Wi-Fi와 휴대폰에서 더 빠릅니다. HTTPS가 필요하고, TCP와 함께 UDP 포트 { $port }도 열려 있어야 합니다(Docker에서는:{ " " }
https-http3-hint-after = ).
https-http3-running = 실행 중입니다.
https-http3-waits = HTTPS가 켜지면 시작합니다.
https-http3-failed = HTTP/3을 시작하지 못했습니다: { $error }
https-lockout-title = 잠겨서 들어갈 수 없을 때
https-lockout-1 = 서버 자체에서는{ " " }
https-lockout-2 = { " " }주소가 항상 일반 http://로 작동합니다. 또는 서버의 명령줄에서 HTTPS를 끄고 Hoard를 다시 시작하세요:{ " " }
https-lockout-3 = { " " }(Docker에서는:{ " " }
https-lockout-4 = { " " }실행 후{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = 보기 가능
account-role-view-hint = 검색, 미리보기, 다운로드
account-role-edit = 변경 가능
account-role-edit-hint = 추가, 변경, 태그, 컬렉션, 보관도 가능
account-role-admin = 관리자
account-role-admin-hint = 설정, 계정, 유지 관리, 영구 삭제도 가능
account-source-internal = Hoard 비밀번호
account-source-ldap = 사용자의 { $directory } 비밀번호
account-source-oauth = { $provider }(으)로 로그인
account-source-word-directory = 디렉터리
account-load-failed = 계정을 불러오지 못했습니다: { $error }
account-failed = { $what }: { $error }
account-password-failed = 비밀번호를 바꾸지 못했습니다
account-password-changed = 비밀번호를 바꿨습니다. 다른 기기에서는 로그아웃되었습니다.
account-add-failed = { $name }을(를) 추가하지 못했습니다
account-added = 이제 { $name }이(가) 로그인할 수 있습니다.
account-role-failed = { $name }의 권한을 바꾸지 못했습니다
account-remove-title = { $name }을(를) 제거할까요?
account-remove-body = 모든 곳에서 로그아웃되며 다시 로그인할 수 없습니다. 이 사용자가 라이브러리에 추가한 것은 삭제되지 않습니다.
account-remove = 제거
account-remove-failed = { $name }을(를) 제거하지 못했습니다
account-token-failed = 토큰을 만들지 못했습니다
account-copied = 복사했습니다
account-copy-token-failed = 복사하지 못했습니다. 토큰을 직접 선택해서 복사하세요
account-copy-failed = 복사하지 못했습니다. 직접 선택해서 복사하세요
account-revoke-title = “{ $name }”을(를) 폐기할까요?
account-revoke-body = 이 토큰을 아직 쓰는 것은 지금부터 거부됩니다.
account-revoke = 폐기
account-revoke-failed = “{ $name }”을(를) 폐기하지 못했습니다
account-disconnect-title = { $app } 연결을 끊을까요?
account-disconnect-body = 지금부터 거부되며, Hoard를 사용하려면 다시 연결해야 합니다.
account-disconnect = 연결 끊기
account-disconnect-failed = { $app } 연결을 끊지 못했습니다
account-guest-failed = 변경하지 못했습니다
account-never = 없음
account-you = 나
account-sign-out = 로그아웃
account-signed-in-as-before = 로그인한 사용자:{ " " }
account-signed-in-as-admin = { " " }(관리자)
account-signed-in-as-edit = { " " }(라이브러리를 변경할 수 있음)
account-signed-in-as-view = { " " }(볼 수는 있지만 아무것도 변경할 수 없음)
account-app-signin = Hoard 앱이 알아서 로그인하므로 비밀번호가 없습니다. 엔진 플러그인과 hoard 명령에 쓸 액세스 토큰은 아래에서 만드세요.
account-password-at-provider = 비밀번호는 로그인에 쓰는 제공자가 보관하므로 그곳에서 바꾸세요.
account-password-at-directory = 비밀번호는 로그인에 쓰는 디렉터리가 보관하므로 그곳에서 바꾸세요.
account-current-password = 현재 비밀번호
account-new-password = 새 비밀번호
account-new-password-again = 새 비밀번호 확인
account-change-password = 비밀번호 변경
account-password-mismatch = 새 비밀번호가 일치하지 않습니다.
account-password-rule = 8자 이상.
account-people = 사용자
account-people-hint = 각 사용자는 보기(검색, 미리보기, 다운로드)만 하거나, 라이브러리 변경(추가, 편집, 태그, 컬렉션, 보관)까지 하거나, 관리자(설정, 계정, 유지 관리, 영구 삭제)까지 될 수 있습니다.
account-you-tag = (나)
account-signs-in-oauth = OAuth 제공자로 로그인
account-signs-in-directory = 디렉터리로 로그인
account-since = { $date }부터
account-what-can-do = { $name }의 권한
account-name = 이름
account-how-sign-in = 로그인 방식
account-password-label = 비밀번호(8자 이상)
account-password-placeholder = 비밀번호(8자 이상)
account-what-they-can-do = 권한
account-add = 추가
account-ai-apps = Claude 및 다른 AI 앱
account-ai-apps-hint = Claude(또는 MCP 앱)를 연결하면 대신 에셋을 검색하고 가져오며, Hoard가 아직 설명하거나 태그하지 않은 것을 설명하고 태그할 수 있습니다. 처음에 Hoard에 로그인하며, 여기 브라우저에서 허용하면 됩니다.
account-mcp-address = Hoard의 MCP 주소
account-copy = 복사
account-claude-code-1 = { "" }
account-claude-code-2 = { " " }명령을 실행한 다음{ " " }
account-claude-code-3 = { " " }명령으로 로그인합니다.{ " " }
account-claude-code-4 = { " " }명령은 팩을 설명합니다.
account-claude-ai = 설정 → 커넥터 → 사용자 지정 커넥터 추가에서 주소를 붙여 넣고 연결을 누르세요. 이 앱들은 Anthropic의 서버에서 연결하므로 인터넷에서 HTTPS로 Hoard에 접속할 수 있어야 합니다.
account-claude-ai-local = 설정 → 커넥터 → 사용자 지정 커넥터 추가에서 주소를 붙여 넣고 연결을 누르세요. 이 앱들은 Anthropic의 서버에서 연결하므로 인터넷에서 HTTPS로 Hoard에 접속할 수 있어야 합니다(이 주소는 그렇지 않습니다. 설정 → 연결에서 공개 주소를 설정하세요).
account-connection-meta = 사용자 { $user } · { $since }부터 · 마지막 사용 { $used }
account-no-apps = 아직 연결된 앱이 없습니다.
account-live = 지금 Hoard를 사용 중
account-live-hint = 최근 10분 안에 Hoard에 무언가를 요청한 앱입니다. 자동으로 업데이트됩니다.
account-live-none = 최근 10분 안에 Hoard에 무언가를 요청한 것이 없습니다.
account-live-last = 마지막: { $what } · { $when } · 요청: { $calls }
account-tokens = 액세스 토큰
account-tokens-hint-1 = { "" }
account-tokens-hint-2 = { " " }명령줄 도구, 스크립트, MCP 클라이언트, Unreal 워커용입니다. 토큰은 다음과 같이 보냅니다:{ " " }
account-tokens-hint-3 = . 토큰은 내가 할 수 있는 범위 안에서 고른 일을 할 수 있지만, 계정이나 토큰은 절대 관리할 수 없습니다. 보기만 하는 토큰으로도 검색, 다운로드, 프로젝트 동기화를 할 수 있습니다.
account-tokens-admin = 모든 사람의 토큰과 연결된 앱이 보입니다. 다른 사람에게는 자기 것만 보입니다.
account-made-before = { "" }
account-made-after = : 지금 복사하세요. 다시 표시되지 않습니다.
account-new-token = 새 토큰
account-cli-1 = CLI에서:{ " " }
account-cli-2 = , 또는 토큰을{ " " }
account-cli-3 = { " " }환경 변수에 넣으세요.
account-done = 완료
account-token-meta = { $made } 만듦 · 마지막 사용 { $used }
account-token-meta-by = { $made }에 { $by }이(가) 만듦 · 마지막 사용 { $used }
account-no-tokens = 아직 토큰이 없습니다.
account-token-name = 사용할 곳
account-token-name-placeholder = 사용할 곳(예: “Unreal 워커”)
account-token-role = 권한
account-make-token = 토큰 만들기
account-guests = 게스트
account-guests-allow = 서버에 접속할 수 있는 누구나 로그인하지 않고 둘러보도록 허용
account-guests-hint = 검색, 미리보기, 다운로드는 할 수 있지만, 무엇이든 추가, 변경, 삭제하거나 설정을 볼 수는 없습니다.

## SignIn.vue

signin-create-title = 계정 만들기
signin-title = 로그인
signin-setup-lede = 이 서버에는 아직 계정이 없습니다. 여기서 만드는 계정은 모든 것을 할 수 있습니다. 다른 사람의 계정은 나중에 설정 → 계정에서 추가하세요.
signin-overlay-lede = 이곳은 누구나 둘러볼 수 있지만, 변경하려면 계정이 필요합니다.
signin-with = { $provider }(으)로 로그인
signin-or-password = 또는 이름과 비밀번호로
signin-username = 사용자 이름
signin-name = 이름
signin-new-password = 새 비밀번호
signin-password = 비밀번호
signin-password-rule = 8자 이상.
signin-confirm-password = 비밀번호 확인
signin-mismatch = 두 비밀번호가 일치하지 않습니다.
signin-setup-code = 설정 코드
signin-setup-code-hint-before = 이 서버의{ " " }
signin-setup-code-hint-after = 에 지정된 코드입니다.
signin-creating = 만드는 중…
signin-signing-in = 로그인하는 중…
signin-create = 계정 만들기
signin-submit = 로그인
signin-keep-browsing = 계속 둘러보기
signin-directory-hint = { $directory } 이름과 비밀번호를 사용하세요.
signin-forgot-1 = Hoard 비밀번호를 잊었나요? 서버에서{ " " }
signin-forgot-2 = { " " }(Docker에서는{ " " }
signin-forgot-3 = )을(를) 실행하면 새 비밀번호를 정할 수 있습니다.

## SetupWizard.vue (first visit to a new server)

setup-title = 라이브러리를 어디에 둘까요?
setup-lede = Hoard가 팩을 보관할 서버의 폴더를 고르거나, 이미 가지고 있는 팩 폴더를 지정하세요. 나중에 설정에서 다른 디스크의 위치를 더 추가할 수 있습니다.
setup-lede-desktop = Hoard가 팩을 보관할 폴더를 고르거나, 이미 가지고 있는 팩 폴더를 지정하세요. 나중에 설정에서 다른 디스크의 위치를 더 추가할 수 있습니다.
setup-unreachable = 서버에 연결하지 못했습니다: { $error }
setup-unreachable-desktop = Hoard가 응답하지 않습니다: { $error }
setup-try-again = 다시 시도
setup-loading = 불러오는 중…
setup-start = 라이브러리 시작
setup-ready = 라이브러리가 준비되었습니다
setup-managed-before = 새 팩은{ " " }
setup-managed-after = 에 들어갑니다. 페이지 아무 곳에나 파일, 폴더, 압축 파일을 놓아서 첫 팩을 추가하세요.
setup-watched-before = Hoard가{ " " }
setup-watched-after = 을(를) 감시하고 있으며, 팩을 찾는 대로 표시합니다. 업로드도 하려면 설정 → 라이브러리 위치에서 Hoard가 보관하는 라이브러리를 추가하세요.
setup-open = 라이브러리 열기

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = { $app }을(를) 연결할까요?
connect-lede-1 = 은(는){ " " }
connect-lede-2 = { " " }계정으로 이 Hoard의 모든 것을 검색, 다운로드, 설명, 태그할 수 있게 됩니다. 계정이나 액세스 토큰은 관리할 수 없습니다.
connect-back-before = 다음으로 돌아갑니다:{ " " }
connect-back-after = . 설정 → 계정에서 언제든 연결을 끊을 수 있습니다.
connect-allow = 허용
connect-cancel = 취소
connect-failed = 이 앱을 연결할 수 없습니다
connect-go-library = 라이브러리로 이동
connect-checking = 확인 중…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = 일반 http://로 접속 중이므로 몇 가지 기능에는 HTTPS가 필요합니다: WebGPU 3D, 최신 클립보드, Hoard를 앱으로 설치하기, Claude 커넥터.
insecure-switch = https://로 전환
insecure-set-up = HTTPS 설정
insecure-dismiss = 다시 표시하지 않기

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = 이 파일은 너무 커서 스토어 파일로 읽을 수 없습니다.
formatwizard-not-json = 이 파일은 올바른 JSON이 아니므로 텍스트로 표시합니다.
formatwizard-read-failed = 파일을 읽지 못했습니다: { $error }
formatwizard-back-title = 양식으로 돌아갈까요?
formatwizard-back-body = TOML에서 바꾼 내용은 사라집니다. 양식이 가진 내용으로 형식을 다시 씁니다.
formatwizard-back-to-form = 양식으로 돌아가기
formatwizard-miss-name = 이 형식의 파일 이름에 { $file }이(가) 해당하지 않습니다. 파일에 추가하세요.
formatwizard-miss-not-the-stores = 스토어 파일을 알아보는 기준이 이 파일에 없습니다. 알아보는 기준을 바꾸세요.
formatwizard-miss-no-title = 이 파일에서 제목을 찾지 못했습니다. 제목을 고르세요.
formatwizard-miss-short-name = 이름이 다름
formatwizard-miss-short-not-the-stores = 스토어 파일 아님
formatwizard-miss-short-no-title = 제목 없음
formatwizard-saved-packs = { $file }을(를) 저장했습니다. 팩 { $packs }개를 다시 읽는 중입니다.
formatwizard-saved = { $file }을(를) 저장했습니다.
formatwizard-save-failed = 형식을 저장하지 못했습니다: { $error }
formatwizard-zip-failed = zip을 만들지 못했습니다: { $error }
formatwizard-stop-title = 이 파일 가르치기를 그만둘까요?
formatwizard-stop-body = 형식은 저장되지 않습니다.
formatwizard-stop = 그만두기
formatwizard-label = 형식 마법사
formatwizard-changing = { $name } 변경 중
formatwizard-starting-from = { $name }에서 시작
formatwizard-title = 스토어 파일을 Hoard에 가르치기
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }(팩:{ " " }
formatwizard-place-3 = )
formatwizard-edit-toml = TOML로 편집
formatwizard-file-reference = 참고용 파일입니다.
formatwizard-click-json = 파일에서 { $field }을(를) 클릭하세요.
formatwizard-click-text = 파일에서 { $field }을(를) 클릭하거나 선택하세요.
formatwizard-pick-first = 오른쪽에서 필드를 고른 다음, 여기서 그 값을 클릭하세요.
formatwizard-loading = 불러오는 중…
formatwizard-the-format = 형식
formatwizard-syntax-before = 문법은{ " " }
formatwizard-syntax-after = 에 있습니다.
formatwizard-store = 스토어
formatwizard-store-placeholder = 스토어 이름(예: Kenney)
formatwizard-files = 파일
formatwizard-recognize = 알아보는 기준
formatwizard-recognize-json = 스토어 파일에만 있는 키
formatwizard-recognize-text = 스토어 파일에만 있는 줄
formatwizard-recognize-hint = 한 줄에 하나씩 적으며, 그중 하나만 맞으면 됩니다. 다른 파일이 같은 이름을 가질 때 필요합니다.
formatwizard-seller = 판매자
formatwizard-seller-placeholder = 파일에 나와 있지 않을 때
formatwizard-tags = 태그
formatwizard-tags-placeholder = 설명하는 모든 것에 붙일 태그, 쉼표로 구분
formatwizard-fields = 필드
formatwizard-selector = { $field } 선택자
formatwizard-clear = { $field } 지우기
formatwizard-pick = 고르기
formatwizard-what-it-reads = 읽어 낸 내용
formatwizard-read-title = 제목
formatwizard-read-seller = 판매자
formatwizard-read-license = 라이선스
formatwizard-read-url = 스토어 페이지
formatwizard-read-kind = 종류
formatwizard-read-tags = 태그
formatwizard-read-origin = 제작 도구
formatwizard-read-size = 크기
formatwizard-also = { $format }도 이 파일을 읽습니다. 이 형식을 먼저 시도합니다.
formatwizard-library = 팩 { $packs }개에 있는 파일 { $files }개가 이 형식이 받는 이름입니다. 시도한 { $tried }개 중 { $read }개를 읽습니다.
formatwizard-library-none = 라이브러리에 이 형식이 받는 이름의 파일이 없습니다.
formatwizard-share = 공유하기
formatwizard-share-hint = 형식, 샘플로 쓸 이 파일, 샘플을 읽은 결과를 Hoard 저장소와 같은 구조로 담은 zip입니다. 보내 주시면 모두가 쓸 수 있게 됩니다. 샘플에서 공유하면 안 되는 내용은 먼저 빼 주세요.
formatwizard-sample = 공유할 샘플
formatwizard-download = { $file } 다운로드
formatwizard-close = 닫기
formatwizard-use = 여기서 사용
formatwizard-cancel = 취소
settings-unreal-editor-what = 내보내기 작업자용 Unreal Editor를 설치합니다. 에셋을 내보내려면 작업자를 별도로 실행하세요.
settings-unreal-download = Epic의 Linux x86-64 에디터를 Hoard의 도구 폴더에 한 번 다운로드합니다. 수십 GB의 디스크 공간과 Epic이 허용한 GitHub 계정이 필요합니다.
settings-unreal-github-step-link-go = 연결 방법
settings-unreal-github-step-token = read:packages만 선택한 GitHub 토큰(classic)을 만드세요.
settings-unreal-github-step-token-go = 토큰 만들기
settings-unreal-github-user = GitHub 사용자 이름
settings-unreal-github-token = 토큰
settings-unreal-github-token-kept = 저장됨(바꾸려면 새 토큰을 입력하세요)
settings-unreal-github-save = 로그인 저장
settings-unreal-github-forget = 연결 해제
settings-unreal-github-saved = GitHub 로그인을 저장했습니다. 다운로드를 다시 시도하는 중입니다
settings-unreal-github-forgotten = GitHub 연결 해제됨
settings-unreal-github-from-server = 이 서버에도 자체 로그인이 설정되어 있습니다(GHCR_TOKEN_FILE 또는 DOCKER_CONFIG). 여기에 저장된 로그인이 없으면 그것을 사용합니다.
settings-unreal-github-connect = GitHub 연결…
settings-unreal-github-connect-title = GitHub 연결
settings-unreal-github-connect-why = Epic은 Epic Games에 연결된 GitHub 계정에 Unreal Editor를 공유합니다. Hoard는 패키지 읽기 권한만 요청합니다.
settings-unreal-github-sign-in = GitHub로 로그인
settings-unreal-github-enter-code = 클릭하면 GitHub가 새 탭에서 열립니다. 그곳에 이 코드를 입력하세요.
settings-unreal-github-waiting = GitHub를 기다리는 중…
settings-unreal-github-expired = 코드가 만료되었습니다.
settings-unreal-github-denied = GitHub에서 로그인이 취소되었습니다.
settings-unreal-github-failed = GitHub에서 코드를 받지 못했습니다.
settings-unreal-github-again = 새 코드 받기
settings-unreal-github-before = 시작하기 전에
settings-unreal-github-before-what = GitHub를 Epic Games 계정에 연결하고 GitHub가 이메일로 보내는 EpicGames 초대를 수락하세요.
settings-unreal-github-use-token = 대신 토큰 사용
settings-unreal-github-use-sign-in = 대신 GitHub로 로그인
settings-unreal-github-no-password = Hoard에는 어떤 비밀번호도 전달되지 않습니다
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = { $user }(으)로 연결됨, 다운로드를 다시 시도합니다
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = { $user }(으)로 연결됨
settings-unreal-github-as-what = Unreal Editor는 이 GitHub 계정으로 다운로드됩니다.
settings-unreal-github-none = GitHub가 연결되지 않음
settings-unreal-github-none-what = 에디터를 다운로드하는 데 필요합니다.
settings-unreal-github-change = 변경…
settings-unreal-image = Unreal 버전 또는 이미지
settings-editor-path = 설치된 { $tool }의 실행 파일 경로를 입력하세요. 자동 설치는 Docker에서만 사용할 수 있습니다.
error-unreal-image = Unreal 5 버전, Epic dev/dev-slim 버전 태그 또는 Epic SHA-256 이미지 참조를 입력하세요.
error-github-no-app = 이 서버에는 로그인에 쓸 GitHub 앱이 없습니다. 대신 토큰을 사용하세요.
error-github-unreachable = GitHub에 연결할 수 없습니다.
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub가 로그인을 거부했습니다: { $reason }
