## Lỗi máy chủ

error-account-not-found = không tìm thấy tài khoản
error-asset-gone = không tìm thấy tài nguyên
error-asset-not-found = không tìm thấy tài nguyên { $id }
error-blob-not-found = không tìm thấy blob { $hash }
error-collection-not-found = không tìm thấy bộ sưu tập { $id }
error-connection-not-found = không tìm thấy kết nối
error-derivative-not-found = không tìm thấy tệp phái sinh { $kind } của { $id }
error-file-in-pack-not-found = không tìm thấy { $path } trong gói { $id }
error-site-not-a-page = chỉ trang web mới mở dưới dạng trang
error-site-link-expired = địa chỉ của trang này đã hết hạn; hãy mở lại trang từ Hoard
error-format-not-found = không tìm thấy định dạng { $file }
error-frame-sheet-aseprite-only = chỉ tệp Aseprite mới có bảng khung hình
error-frame-sheet-frames = tệp này chỉ có một khung hình, hoặc có quá nhiều khung hình để sắp xếp
error-location-not-found = không tìm thấy vị trí { $id }
error-maintenance-task-not-found = không tìm thấy tác vụ bảo trì { $id }
error-open-upload-not-found = không tìm thấy lượt tải lên đang mở { $id }
error-original-not-found = không tìm thấy tệp gốc của { $id }
error-pack-not-found = không tìm thấy gói { $id }
error-preview-too-big = ảnh này quá lớn để tạo bản xem trước nhẹ
error-project-not-found = không tìm thấy dự án { $name }
error-saved-search-not-found = không tìm thấy tìm kiếm đã lưu { $id }
error-sign-in-first = hãy đăng nhập trước (hoặc gửi token truy cập)
error-those-assets-not-found = không tìm thấy các tài nguyên đó
error-thumbnail-not-found = không tìm thấy ảnh thu nhỏ của { $id }
error-token-not-found = không tìm thấy token
error-upload-not-found = không tìm thấy lượt tải lên { $id }
error-pack-needs-name = gói cần có tên
error-no-library = chưa có vị trí thư viện nào để lưu tệp: hãy thêm một vị trí được quản lý trong Cài đặt → Vị trí thư viện
error-upload-failed = tải lên thất bại: { $reason }
error-upload-no-files = lượt tải lên không có tệp nào
error-analyze-nothing-to-do = không có gì để làm: rules=false và LLM đang tắt
error-too-many-assets = quá nhiều tài nguyên trong một yêu cầu
error-change-not-found = thay đổi đó không có trong lịch sử của tài nguyên này
error-bulk-by-look = không thể sửa cùng lúc kết quả tìm theo hình thức: hãy chọn các tài nguyên
error-too-many-items = quá nhiều mục trong một yêu cầu
error-not-a-hash = không phải mã băm blake3 dạng hex
error-merge-into-tag = into phải là một thẻ
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = "{ $text }" không phải là thẻ
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } đã nằm dưới { $tag }, nên { $tag } không thể đặt dưới nó
error-nothing-to-merge = không có gì để gộp
error-selection-size = hãy đưa từ 1 đến { $max } id tài nguyên
error-collection-needs-name = bộ sưu tập cần có tên
error-collection-exists = đã có bộ sưu tập tên là { $name }
error-collection-empty = bộ sưu tập { $name } đang trống
error-conversion-failed = không thể chuyển đổi sang { $format }: { $reason }
error-max-size = max phải là lũy thừa của hai, từ 64 đến 16384
error-replace-ext = ext phải là một phần mở rộng tệp ngắn
error-empty-file = tệp trống
error-imitation-silent = Không có gì trong bản ghi nổi bật hơn tiếng nền. Hãy thử lại gần micrô hơn.
error-replace-duplicate = tệp đã chuyển đổi trùng với một tệp đã có trong gói; gói tệp đã được gộp vào tệp đó
error-ai-address = Địa chỉ AI phải bắt đầu bằng http:// hoặc https://
error-hosted-key = Nhập khóa API cho { $provider }
error-hosted-address = Địa chỉ của dịch vụ tìm kiếm hình ảnh phải bắt đầu bằng http:// hoặc https://
error-local-llm-action = action là list, download hoặc delete, và hai giá trị sau cần có model
error-role = Vai trò là view, edit hoặc admin
error-admin-only = chỉ quản trị viên mới làm được việc này
error-view-only = tài khoản này chỉ được xem, không được thay đổi gì: quản trị viên có thể đổi điều đó trong Cài đặt → Tài khoản
error-portal-only = hãy đăng nhập vào cổng web để quản lý tài khoản và token
error-not-from-portal = yêu cầu này không đến từ cổng web Hoard
error-account-name = Hãy chọn tên dài tối đa 64 ký tự
error-password-length = Hãy dùng mật khẩu dài ít nhất { $min } ký tự
error-too-many-codes = Nhập sai mã quá nhiều lần: hãy chờ { $seconds } giây
error-already-set-up = Đã có tài khoản: hãy đăng nhập
error-too-many-tries = Thử sai quá nhiều lần: hãy chờ { $seconds } giây
error-directory-unavailable = Máy chủ thư mục không thể kiểm tra mật khẩu của bạn: { $reason }
error-password-elsewhere = Mật khẩu của bạn do máy chủ thư mục hoặc nhà cung cấp mà bạn dùng để đăng nhập quản lý
error-current-password = Mật khẩu hiện tại của bạn không đúng
error-account-source = Tài khoản đăng nhập bằng Hoard (internal), máy chủ thư mục (ldap) hoặc OAuth
error-account-exists = Đã có tài khoản với tên đó
error-own-role = Một quản trị viên khác có thể đổi vai trò của bạn
error-remove-self = Bạn không thể xóa tài khoản của chính mình khi đang đăng nhập vào nó
error-token-name = Hãy đặt tên token theo thứ sẽ dùng nó, tối đa 80 ký tự
error-token-role = Token không thể làm nhiều hơn tài khoản tạo ra nó
error-wrong-setup-code = Đó không phải mã thiết lập đã cấp cho máy chủ trong HOARD_SETUP_CODE
error-wrong-password = Tên và mật khẩu không khớp
error-folder-missing = { $path } không tồn tại (ổ đĩa chưa được gắn?)
error-s3-path = vị trí S3 có dạng s3://bucket
error-s3-address = địa chỉ S3 phải bắt đầu bằng http:// hoặc https://
error-s3-header = header S3 bổ sung có dạng Name=value, và “{ $header }” không phải dạng đó
error-keychain = không thể lưu vào kho mật khẩu của hệ thống: { $reason }
error-not-built = bản Hoard này được dựng mà không có { $feature }
error-openai-auth-unreachable = không thể kết nối tới auth.openai.com
error-openai-unexpected = OpenAI trả lời không như mong đợi
error-openai-answered = OpenAI trả lời { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID token không phải JWT
error-chatgpt-id-token-issuer = ID token đến từ { $issuer }, không phải { $expected }
error-chatgpt-id-token-audience = ID token dành cho ứng dụng khác
error-chatgpt-id-token-expired = ID token đã hết hạn
error-chatgpt-id-token-nonce = ID token không thuộc lần đăng nhập này
error-chatgpt-no-state = địa chỉ không có state; hãy sao chép toàn bộ địa chỉ
error-chatgpt-not-started = lần đăng nhập này không được bắt đầu ở đây, hoặc đã bắt đầu hơn 15 phút trước. Hãy bắt đầu lại từ Cài đặt.
error-chatgpt-canceled = lần đăng nhập đã bị hủy
error-chatgpt-refused = ChatGPT từ chối đăng nhập: { $reason }
error-chatgpt-no-code = địa chỉ không có mã đăng nhập; hãy sao chép toàn bộ địa chỉ
error-chatgpt-other-registration = ChatGPT trả lời cho một lần đăng ký khác của máy chủ này; hãy bắt đầu đăng nhập lại
error-chatgpt-other-registration-desktop = ChatGPT trả lời cho một bản Hoard khác; hãy bắt đầu đăng nhập lại
error-chatgpt-not-registered = ChatGPT chưa hoàn tất việc đăng ký Hoard; hãy bắt đầu đăng nhập lại
error-chatgpt-code-used = mã đăng nhập đã được dùng hoặc đã hết hạn; hãy đăng nhập lại
error-chatgpt-refused-code = ChatGPT từ chối đăng nhập ({ $code })
error-chatgpt-no-id-token = ChatGPT không gửi ID token
error-chatgpt-plan-not-allowed = Đã đăng nhập, nhưng việc dùng gói đăng ký không được cho phép. Hãy đăng nhập lại và cho phép, hoặc dùng khóa API thay thế.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = chưa đăng nhập ChatGPT; hãy đăng nhập từ Cài đặt → Gắn thẻ AI
error-chatgpt-refresh-later = ChatGPT chưa thể làm mới phiên đăng nhập lúc này ({ $code }); sẽ thử lại sau
error-chatgpt-signed-out-problem = ChatGPT đã đăng xuất máy chủ này ({ $code }). Hãy đăng nhập lại để tiếp tục gắn thẻ bằng gói đăng ký của bạn.
error-chatgpt-signed-out-problem-desktop = ChatGPT đã đăng xuất Hoard ({ $code }). Hãy đăng nhập lại để tiếp tục gắn thẻ bằng gói đăng ký của bạn.
error-chatgpt-signed-out = ChatGPT đã đăng xuất máy chủ này ({ $code }); hãy đăng nhập lại từ Cài đặt → Gắn thẻ AI
error-chatgpt-signed-out-desktop = ChatGPT đã đăng xuất Hoard ({ $code }); hãy đăng nhập lại từ Cài đặt → Gắn thẻ AI
error-openai-api-unreachable = không thể kết nối tới api.openai.com
error-openai-broke-off = câu trả lời từ OpenAI bị ngắt giữa chừng
error-chatgpt-kept-refusing = OpenAI liên tục từ chối phiên đăng nhập ChatGPT; hãy đăng nhập lại từ Cài đặt → Gắn thẻ AI
error-chatgpt-request-failed = yêu cầu thất bại
error-chatgpt-cut-off = câu trả lời bị cắt ngang ({ $reason })
error-chatgpt-cut-off-no-reason = câu trả lời bị cắt ngang (không rõ lý do)
error-openai-ended-early = câu trả lời từ OpenAI kết thúc trước khi hoàn chỉnh
error-chatgpt-usage-limit = hạn mức sử dụng cho Hoard trong gói ChatGPT của bạn tạm thời đã hết. Xem hoặc thay đổi hạn mức tại { $url }
error-chatgpt-not-eligible = tài khoản hoặc không gian làm việc ChatGPT này không thể dùng gói đăng ký trong ứng dụng khác. Hãy dùng khóa API thay thế.
error-chatgpt-usage-unavailable = ChatGPT chưa thể kiểm tra mức sử dụng của gói lúc này; sẽ thử lại sau
error-chatgpt-unsupported = gói ChatGPT không hỗ trợ { $param } với mô hình này
error-chatgpt-unsupported-images = gói ChatGPT không hỗ trợ { $param } với mô hình này; hãy chọn mô hình nhận được hình ảnh, hoặc tắt việc gửi ảnh thu nhỏ
error-chatgpt-unsupported-part = gói ChatGPT không hỗ trợ một phần của yêu cầu với mô hình này
error-chatgpt-port = port là cổng mà trình duyệt dùng để kết nối tới Hoard
error-chatgpt-paste-address = hãy dán toàn bộ địa chỉ, bắt đầu bằng http://127.0.0.1
error-too-many-hashes = tối đa { $max } mã băm mỗi lần
error-credits-pack = gói { $key }
error-credits-collection = bộ sưu tập { $key }
error-format-too-long = định dạng quá dài
error-not-started = máy chủ chưa khởi động
error-not-started-desktop = Hoard chưa khởi động
error-not-format-file = không phải tệp của định dạng: { $name }
error-format-name = tên cần có ít nhất một chữ cái hoặc chữ số từ A đến Z hoặc 0 đến 9
error-format-miss-name = các tệp của định dạng không bao gồm { $file }
error-format-miss-store = định dạng không nhận { $file } cho cửa hàng (xem require)
error-format-miss-title = định dạng không tìm thấy tiêu đề nào trong { $file }
error-cant-download-as = không thể tải tệp .{ $ext } xuống dưới dạng { $format }
error-only-images-resize = chỉ có thể đổi kích thước hình ảnh
error-https-no-certificate = Hãy dán hoặc tải lên chứng chỉ và khóa của nó trước
error-https-no-cert-found = Không tìm thấy chứng chỉ: hãy dán đoạn văn bản bắt đầu bằng -----BEGIN CERTIFICATE-----
error-https-no-key-found = Không tìm thấy khóa riêng: hãy dán đoạn văn bản bắt đầu bằng -----BEGIN PRIVATE KEY-----
error-https-several-keys = Có nhiều hơn một khóa riêng: chỉ dán khóa đi kèm với chứng chỉ
error-https-key-password = Khóa được bảo vệ bằng mật khẩu: hãy cung cấp khóa không có mật khẩu (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Chứng chỉ không phải PEM hợp lệ
error-https-key-not-pem = Khóa riêng không phải PEM hợp lệ
error-https-key-mismatch = Khóa không thuộc về chứng chỉ. Nếu bạn đưa một chuỗi chứng chỉ, hãy đặt chứng chỉ của máy chủ lên đầu
error-https-unusable = Không thể dùng chứng chỉ hoặc khóa này
error-https-expired = Chứng chỉ này đã hết hạn vào { $date }
error-https-not-yet-valid = Chứng chỉ này chưa có hiệu lực cho đến { $date }
error-https-unreadable = Đó không phải chứng chỉ mà Hoard đọc được
error-https-bad-name = “{ $name }” không phải địa chỉ mà chứng chỉ có thể chứa
error-https-test-timed-out = kết nối thử đã hết thời gian chờ
error-https-test-failed = kết nối thử thất bại
error-https-no-ca = Hoard chưa tạo tổ chức cấp chứng chỉ (CA) của mình
error-https-unavailable = HTTPS không khả dụng trên máy chủ này
error-https-certificate-mode = Chứng chỉ đi kèm với tùy chọn “Chứng chỉ của tôi”
error-https-test = Chứng chỉ không hoạt động trong kết nối thử: { $reason }
error-nothing-to-retry = không có gì ở đó để thử lại
error-ai-unreachable = không thể kết nối tới { $url }
error-ai-key-refused = { $url } từ chối khóa API ({ $status })
error-ai-answered = { $url } trả lời { $status }: { $reason }
error-ai-not-json = { $url }/models không trả lời bằng JSON
error-model-server-refused = máy chủ mô hình: { $reason }
error-no-model-server = không có máy chủ mô hình nào trả lời tại { $url }
error-not-llama-router = { $url } không phải máy chủ llama.cpp ở chế độ router, nên không thể tải mô hình xuống
error-model-download = không thể tải xuống { $model }: { $reason }
error-model-delete = không thể xóa { $model }: { $reason }
error-unknown-kind = loại tài nguyên không xác định { $kind }
error-watched-is-folder = vị trí theo dõi là một thư mục trên máy chủ này
error-cant-reach = không thể truy cập { $path }: { $reason }
error-folder-full-path = hãy đưa đường dẫn đầy đủ của thư mục
error-cant-create = không thể tạo { $path }
error-cant-write = không thể ghi vào { $path }
error-cant-read = không thể đọc { $path }
error-overlaps-location = vị trí này chồng lấn với vị trí { $name } ({ $path })
error-overlaps-data = vị trí này chồng lấn với thư mục dữ liệu của Hoard ({ $path })
error-overlaps-cache = vị trí này chồng lấn với thư mục bộ nhớ đệm của Hoard ({ $path })
error-not-watched = { $name } không phải thư mục theo dõi
error-watched-no-kinds = thư mục theo dõi không nhận tệp mới, nên không có loại nào để thay đổi
error-keep-catch-all = hãy đặt một vị trí khác làm vị trí chứa mọi loại
error-location-holds-files = { $name } vẫn còn chứa tệp: hãy chuyển chúng sang các vị trí khác trước (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = không thể truy cập { $name }, nên Hoard không biết nó còn chứa tệp hay không
error-relocate-running = tệp đang được chuyển giữa các vị trí; hãy thử lại khi việc đó xong
error-watched-not-moved = { $name } là thư mục theo dõi: Hoard không di chuyển tệp của nó
error-no-other-location = không có vị trí thư viện nào khác để chuyển tệp tới: hãy thêm một vị trí trước
error-full-path = hãy đưa đường dẫn đầy đủ
error-pick-folder = hãy chọn một thư mục
error-task-running = “{ $task }” đang chạy rồi
error-token-account-gone = tài khoản của token này không còn, nên nó không có mục yêu thích hay tìm kiếm nào
error-favorites-too-many = tối đa 10.000 mục mỗi lần
error-saved-search-name = tìm kiếm đã lưu cần có tên dài tối đa 80 ký tự
error-saved-search-too-long = tìm kiếm đó quá dài để lưu
error-hoard-seen = kho báu chỉ ghi được tối đa 32 loại tài nguyên, mỗi loại đặt tên bằng chữ thường
error-texture-pattern-map = Mẫu tên texture cần có { "{" }map{ "}" }, nếu không mọi map của một vật liệu sẽ có cùng tên
error-texture-pattern-fields = Mẫu tên texture có thể dùng { "{" }name{ "}" }, { "{" }map{ "}" } và { "{" }size{ "}" }
error-texture-name-chars = Tên texture không được chứa < > : " / \ | ? hoặc *
error-texture-map-type = Không có loại map nào tên là { $role }
server-role-base-color = màu
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = độ nhám
server-role-gloss = độ bóng
server-role-metalness = độ kim loại
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = độ cao
server-role-bump = bump
server-role-cavity = cavity
server-role-specular = specular
server-role-opacity = độ mờ đục
server-role-emissive = phát sáng
server-role-translucency = độ trong mờ
error-texture-suffix-missing = Hãy đặt hậu tố cho map { $map }
error-texture-suffix-chars = Hậu tố cho map { $map } không được chứa < > : " / \ | ? hoặc *
error-texture-ending-missing = Mỗi đuôi tên riêng của bạn cần có đuôi tên cần tìm
error-texture-ending-chars = Đuôi tên riêng không được chứa < > : " / \ | ? hoặc *
error-texture-ending-twice = { $ending } xuất hiện hai lần trong các đuôi tên của bạn
error-oauth-no-client-id = Liên kết này không có client_id.
error-oauth-unknown-app = Ứng dụng này (không còn) được đăng ký ở đây. Hãy bắt đầu kết nối lại từ ứng dụng.
error-oauth-redirect = Ứng dụng yêu cầu được chuyển về một địa chỉ mà nó chưa đăng ký.
error-oauth-response-type = Hoard chỉ cấp mã đăng nhập (response_type=code).
error-oauth-pkce = Ứng dụng phải dùng PKCE với S256 để đăng nhập vào Hoard.
error-light-preview-images = chỉ hình ảnh mới có bản xem trước nhẹ
error-no-light-preview = không có bản xem trước nhẹ: { $reason }
error-no-frame-sheet = không có bảng khung hình: { $reason }
error-thumbnail-source = nguồn ảnh thu nhỏ không xác định: { $source }
error-thumbnail-too-big = ảnh đó quá lớn để làm ảnh thu nhỏ
error-thumbnail-models-only = chỉ mô hình mới nhận ảnh thu nhỏ do trình duyệt vẽ
error-not-a-picture = không phải ảnh mà Hoard đọc được: { $reason }
error-project-name = dự án cần có tên dài tối đa 200 ký tự
error-project-too-big = tối đa một triệu tài nguyên trong một dự án
error-project-too-many-at-once = tối đa 100.000 mục mỗi lần
error-server-name = Hãy chọn tên dài tối đa { $max } ký tự, trên một dòng
error-tool-setting = { $tool }: hãy chọn tắt, tự động, hoặc đưa đường dẫn
error-ai-model = Hãy chọn mô hình mà dịch vụ AI sẽ dùng
error-public-address = Địa chỉ công khai phải bắt đầu bằng http:// hoặc https://
error-update-hook-address = Địa chỉ gọi khi có cập nhật phải bắt đầu bằng http:// hoặc https://
error-update-hook-auth = Header Authorization không được chứa dấu xuống dòng hay ký tự điều khiển khác
error-no-soundfont = Không có SoundFont tại { $path } trên máy chủ
error-no-soundfont-desktop = Không có SoundFont tại { $path }
error-soundfont-bad = { $name } không phải SoundFont mà MIDI có thể dùng để phát: { $reason }
error-workers = Tiến trình nền: từ 1 đến { $max }
error-ai-batch = Số tài nguyên mỗi yêu cầu AI: từ 1 đến 200
error-conversion-cache = Bản tải xuống đã chuyển đổi: ít nhất 100 MB
error-backup-keep = Sao lưu theo lịch: giữ từ 1 đến { $max }
error-backup-copy-to = Sao lưu theo lịch: { $path } không phải là thư mục trên máy chủ (hãy ghi đường dẫn đầy đủ)
error-conversion-keep-days = Quy tắc chuyển đổi: giữ tệp gốc từ 1 đến { $max } ngày
error-conversion-target = Quy tắc chuyển đổi: Hoard không thể chuyển đổi tệp sang { $format }
error-conversion-no-source = Quy tắc chuyển đổi: mỗi quy tắc cần ít nhất một định dạng để chuyển đổi
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Quy tắc chuyển đổi: không thể chuyển đổi tệp .{ $ext } sang { $format }
error-conversion-twice = Quy tắc chuyển đổi: .{ $ext } nằm trong hai quy tắc
error-conversion-cant-undo = Tệp này không được chuyển đổi bởi quy tắc, hoặc tệp gốc của nó đã bị xóa.
error-conversion-path-taken = Hiện đã có tệp khác ở { $path }, nên không thể đưa tệp gốc về lại đó.
error-limits-zero = Giới hạn không được bằng 0
error-settings-object = cần một đối tượng JSON chứa các cài đặt
error-bad-setting = cài đặt không hợp lệ: { $reason }
error-auth-default-role = Vai trò mặc định là view, edit hoặc admin
error-auth-session-hours = Phiên đăng nhập kéo dài từ 1 giờ đến một năm
error-auth-settings = cài đặt xác thực không hợp lệ: { $reason }
error-ldap-settings = cài đặt LDAP không hợp lệ: { $reason }
error-ldap-no-server = Hãy thêm địa chỉ máy chủ thư mục
error-ldap-base-dn = Hãy cho biết các mục của người dùng nằm ở đâu (base DN)
error-ldap-username-attribute = Hãy cho biết thuộc tính nào chứa tên mà người dùng đăng nhập
error-ldap-bind-dn = Hãy đưa tài khoản dùng để tra cứu người dùng, hoặc tra cứu ẩn danh
error-ldap-ca-cert = Tổ chức cấp chứng chỉ không phải chứng chỉ PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = Địa chỉ của một máy chủ thư mục đang trống
error-ldap-server-form = { $server }: địa chỉ máy chủ thư mục có dạng host:port, ldap://… hoặc ldaps://…
error-ldap-not-starttls = { $server }: đó là LDAPS, không phải StartTLS
error-ldap-no-certificate = không có chứng chỉ nào trong đó
error-ldap-password-again = Hãy nhập lại mật khẩu của tài khoản dùng để tra cứu người dùng: mật khẩu đã lưu chỉ được gửi tới các máy chủ mà nó được cấp cho
error-ldap-unreachable = không kết nối được tới máy chủ thư mục nào ({ $reasons })
error-ldap-no-anonymous = máy chủ thư mục không cho phép tra cứu ẩn danh ({ $reason })
error-ldap-bind-failed = tài khoản dùng để tra cứu người dùng không đăng nhập được ({ $reason })
error-ldap-search = tìm { $name } trong { $base }
error-ldap-group-search = tìm các nhóm của { $name } trong { $base }
error-ldap-several = { $count } mục trong máy chủ thư mục khớp với { $name }: hãy thu hẹp bộ lọc người dùng
error-ldap-password-check = máy chủ thư mục không kiểm tra mật khẩu của { $name } ({ $reason })
error-ldap-wrong-password = Máy chủ thư mục không biết { $name } với mật khẩu đó
error-oauth-client-id = Hãy đưa client ID mà nhà cung cấp đã tạo cho Hoard
error-oauth-addresses = Địa chỉ authorization và token phải bắt đầu bằng https://
error-oauth-userinfo-address = Địa chỉ thông tin người dùng phải bắt đầu bằng https://
error-oauth-username-claim = Hãy cho biết claim nào chứa tên của người dùng
error-oauth-secret-again = Hãy nhập lại client secret: secret đã lưu chỉ được gửi tới địa chỉ token mà nó được cấp cho
error-oauth-issuer = Issuer phải bắt đầu bằng https://
error-oauth-discovery-unreachable = Không thể kết nối tới { $url }: { $reason }
error-oauth-discovery-status = { $url } trả lời { $status }
error-oauth-discovery-bad = { $url } không phải tài liệu discovery
error-oauth-discovery-incomplete = { $url } không nêu địa chỉ authorization và token
error-signin-no-group = { $name } không thuộc nhóm nào được phép dùng Hoard
error-signin-other-source = Đã có tài khoản Hoard tên { $name } đăng nhập theo cách khác
error-signin-other-subject = Đã có tài khoản Hoard tên { $name } thuộc về người khác ở nhà cung cấp
error-signin-no-account = { $name } chưa có tài khoản Hoard: quản trị viên có thể thêm trong Cài đặt → Tài khoản
error-signin-bad-name = Tên đó không thể dùng làm tài khoản Hoard (tối đa 64 ký tự)
error-oauth-off = Đăng nhập bằng OAuth chưa được bật
error-oauth-bad-authorization-address = địa chỉ authorization không hợp lệ
error-oauth-refused = Nhà cung cấp không đăng nhập cho bạn: { $reason }
error-oauth-no-code = Nhà cung cấp đưa bạn trở lại mà không đăng nhập
error-oauth-expired = Lần đăng nhập đó quá lâu hoặc được bắt đầu ở nơi khác: hãy thử lại
error-oauth-no-claim = Nhà cung cấp không cho biết bạn là ai (không có claim { $claim })
error-oauth-no-sub = Nhà cung cấp không cho biết bạn là ai (không có claim sub hoặc id)
error-oauth-token-unreachable = không thể kết nối tới địa chỉ token của nhà cung cấp
error-oauth-token-refused = Nhà cung cấp không hoàn tất việc đăng nhập: { $reason }
error-oauth-no-access-token = nhà cung cấp không gửi access token
error-oauth-userinfo-unreachable = không thể kết nối tới địa chỉ thông tin người dùng của nhà cung cấp
error-oauth-userinfo-refused = Nhà cung cấp không cho biết ai đã đăng nhập ({ $status })
error-oauth-userinfo-not-json = thông tin người dùng của nhà cung cấp không phải JSON
error-format-no-name = name đang trống
error-format-no-files = files đang trống: hãy cho biết định dạng đọc những tệp nào, ví dụ ["*.json"]
error-format-empty-selector = trống
error-format-unopened-bracket = có dấu ngoặc đóng mà không có dấu ngoặc mở
error-format-unclosed-bracket = một dấu ngoặc chưa được đóng
error-format-empty-step-dots = một bước trống (hai dấu chấm liền nhau, hoặc dấu chấm ở đầu hay cuối)
error-format-unclosed-except = !( chưa được đóng
error-format-unclosed-filter = [ chưa được đóng
error-format-after-filters = { $rest } sau khóa và bộ lọc của { $step }
error-format-except-needs-glob = !(…) phải đứng sau một khóa có * trong đó
error-format-except-not-glob = !(…) phải đứng sau một khóa có * trong đó, không phải { $key }
error-format-empty-step = một bước trống
error-format-not-a-filter = [{ $filter }] không phải bộ lọc: hãy dùng [], [key=value|other], [array], [string], [number], [bool] hoặc [object]
error-format-filter-needs-value = [{ $filter }] cần có khóa và giá trị
error-store-search-empty = không có gì để tìm; hãy nhập tên gói
error-update-hook-not-set = hãy đặt địa chỉ để gọi trước
error-upload-stalled = việc tải lên { $path } bị đứng; hãy gửi lại phần đó
error-upload-busy = { $path } đang được ghi
error-upload-incomplete = { $count } tệp chưa hoàn chỉnh, ví dụ { $files }
error-upload-no-files-listed = không có tệp nào
error-upload-not-part = { $path } không thuộc lượt tải lên { $id }, hoặc lượt tải lên đã đóng
error-bad-path = đường dẫn không hợp lệ
error-upload-part-offset = { $offset } không phải vị trí bắt đầu của một phần của { $path }
error-upload-interrupted = tải lên bị gián đoạn: { $reason }
error-upload-part-long = phần { $part } của { $path } dài hơn { $len } byte
error-upload-part-short = phần { $part } của { $path }: nhận được { $got } byte, cần { $len }
error-upload-file-long = { $path } lớn hơn kích thước đã khai báo
error-upload-file-short = { $path }: nhận được { $got } byte, cần { $len }
error-upload-complete = lượt tải lên đã hoàn tất
error-already-imported = đã nhập dưới tên “{ $name }”

## Văn bản máy chủ

server-store-place-readme = README
server-store-place-folders = thư mục
server-store-why-linked = Có liên kết trong { $place } của gói
server-store-why-same-title = Cùng tiêu đề với { $place } của gói
server-store-why-by = Của { $author }, được nêu tên trong { $place } của gói
server-store-why-meshes = Số mesh xấp xỉ với gói ({ $count })
server-store-why-textures = Số texture xấp xỉ với gói ({ $count })
server-chatgpt-revoke-failed = Đã đăng xuất ở đây, nhưng không báo được cho OpenAI. Để chắc chắn, hãy ngắt kết nối Hoard trong ChatGPT Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Đã đăng nhập ChatGPT
server-chatgpt-signed-in = Hoard đã đăng nhập. Gắn thẻ AI giờ có thể dùng gói đăng ký ChatGPT của bạn.
server-chatgpt-signed-in-as = Hoard đã đăng nhập dưới tên { $email }. Gắn thẻ AI giờ có thể dùng gói đăng ký ChatGPT của bạn.
server-chatgpt-not-finished-title = Chưa hoàn tất đăng nhập
server-chatgpt-other-computer = Nếu bạn bắt đầu đăng nhập từ Hoard trên một máy tính khác, hãy sao chép địa chỉ của trang này từ thanh địa chỉ và dán vào phần Cài đặt của Hoard ở máy đó.
server-back-to-settings = Quay lại Cài đặt
server-issues-pack = Gói không thể thêm
server-issues-pack-blurb = Hãy xóa gói rồi thêm lại, hoặc mở nó để xem bên trong có gì.
server-issues-damaged = Tệp đã lưu bị hỏng hoặc mất
server-issues-damaged-blurb = Được phát hiện bởi “Kiểm tra tệp hư hỏng”. Hãy khôi phục chúng từ bản sao lưu hoặc thêm lại gói của chúng, rồi chạy lại “Kiểm tra tệp hư hỏng”.
server-issues-folder = Thư mục theo dõi không đọc được
server-issues-folder-blurb = Các gói của chúng được giữ nguyên cho đến khi thư mục đọc được trở lại.
server-issues-missing = Gói theo dõi có thư mục đã mất
server-issues-missing-blurb = Mỗi gói được giữ 3 ngày phòng khi thư mục chỉ tạm thời ngoại tuyến, sau đó sẽ bị xóa.
server-issues-job = Tác vụ nền thất bại
server-issues-job-blurb = Thử lại sẽ chạy lại tác vụ từ đầu.
server-issues-file = Tệp không thể mở
server-issues-file-blurb = Hoard không thể mở hoặc lưu chúng. Gói theo dõi sẽ thử lại ở lần quét tiếp theo; tệp đã có trong thư viện vẫn giữ nội dung trước đó.
server-issues-unpack = Tệp nén không thể giải nén
server-issues-unpack-blurb = Mỗi tệp được giữ trong gói của nó như một tệp thường, nên không có gì bên trong nó nằm trong thư viện.
server-issues-read = Tệp Hoard không hiểu được
server-issues-read-blurb = Chúng có trong thư viện, nhưng không có thông tin, thẻ hay ảnh thu nhỏ mà việc đọc chúng mang lại.
server-issues-render = Mô hình mà trình kết xuất ảnh thu nhỏ gặp lỗi
server-issues-render-blurb = Chúng không có ảnh thu nhỏ. Mở một mô hình trong trình xem vẫn có thể hoạt động và có thể lưu một ảnh thu nhỏ.
server-issues-convert = Mô hình Blender không thể chuyển đổi
server-issues-convert-blurb = Chúng không có bản sao GLB, nên không thể hiển thị ở dạng 3D.
server-issues-unreal = Gói Unreal không thể chuyển đổi
server-issues-unreal-blurb = Trang Unreal của gói có các package còn lại của nó.
server-issues-reformat = Tệp mà quy tắc chuyển đổi không thể chuyển đổi
server-issues-reformat-blurb = Chúng được giữ nguyên như cũ.
server-issues-materials = Gói không xác định được vật liệu
server-issues-materials-blurb = Các texture của chúng vẫn đầy đủ; chỉ là chưa được nhóm thành vật liệu.
server-issues-material-thumb = Vật liệu không có ảnh thu nhỏ
server-issues-material-thumb-blurb = Đã tìm thấy vật liệu, nhưng không vẽ được bản xem trước của nó.
server-issues-store-files = Tệp cửa hàng không đọc được
server-issues-store-files-blurb = Các tệp từ cửa hàng mà gói xuất phát, chẳng hạn JSON của Megascans, dùng để đặt tên và gắn thẻ cho gói.
server-issues-scan-again = Quét lại
server-issues-retry = Thử lại
server-issues-reanalyze = Phân tích lại gói
server-issues-convert-again = Chuyển đổi lại
server-issues-look-again = Tìm lại
server-issues-read-again = Đọc lại
server-issues-blob-missing = Tệp đã lưu không còn nữa.
server-issues-blob-damaged = Tệp đã lưu không khớp với nội dung lúc lưu: tệp đã bị hỏng.
server-issues-folder-gone = Không tìm thấy thư mục “{ $folder }”.
server-job-a-pack = một gói
server-job-ingest = Đang nhập { $pack }
server-job-retag = Đang gắn thẻ lại { $pack }
server-job-llm-tag = Đang gắn thẻ { $pack } bằng AI
server-job-convert = Đang chuyển đổi { $pack } bằng Blender
server-job-unreal = Đang chuyển đổi các package Unreal trong { $pack }
server-job-reformat = Đang chuyển đổi tệp trong { $pack } theo quy tắc chuyển đổi
server-job-scan = Đang quét một thư mục theo dõi
server-job-relocate = Đang chuyển tệp giữa các vị trí thư viện
server-job-looks = Đang xác định màu sắc và diện mạo
server-job-sounds = Đang nghe âm thanh
server-job-embed = Đang tìm hiểu nội dung hình ảnh
server-job-hear = Đang tìm hiểu âm thanh nghe ra sao
server-job-materials = Đang tìm vật liệu trong { $pack }
server-job-read-files = Đang đọc các tệp của { $pack }
server-job-store-lookup = Đang tra cứu trang cửa hàng
server-maintenance-backup = Sao lưu cơ sở dữ liệu
server-maintenance-backup-blurb = Một bản sao nhất quán của cơ sở dữ liệu (gói, thẻ, ghi chú, bộ sưu tập và Cài đặt) trong thư mục backups bên cạnh nó. Tệp của thư viện không có trong đó: chúng không bao giờ thay đổi sau khi lưu, nên bất kỳ công cụ đồng bộ tệp nào cũng sao lưu được.
server-maintenance-scrub = Kiểm tra tệp hư hỏng
server-maintenance-scrub-blurb = Đọc lại mọi tệp đã lưu và so với dấu vân tay của nó, để phát hiện tệp bị hỏng dần hoặc biến mất khỏi ổ đĩa hay NAS. Mất khá lâu với thư viện lớn.
server-maintenance-reanalyze = Phân tích lại mọi gói
server-maintenance-reanalyze-blurb = Đọc lại mọi tệp bằng phiên bản Hoard này: ảnh thu nhỏ, bản xem trước, chi tiết và thẻ theo quy tắc được tạo mới. Những chỉnh sửa của bạn vẫn giữ nguyên; AI không được hỏi lại.
server-maintenance-clear-cache = Xóa bộ nhớ đệm chuyển đổi
server-maintenance-clear-cache-blurb = Bỏ các tệp được giữ lại từ các lần chuyển đổi “Tải xuống dưới dạng” (chúng sẽ được tạo lại khi cần) và những gì công việc bị gián đoạn để lại.
server-maintenance-shrink-thumbs = Thu nhỏ ảnh thu nhỏ
server-maintenance-shrink-thumbs-blurb = Lưu các ảnh thu nhỏ và ảnh xem mô hình đang ở dạng PNG theo cách Cài đặt → Hiệu năng → Ảnh thu nhỏ yêu cầu, thường giúp chúng nhỏ đi vài lần. Ảnh có độ trong suốt và ảnh pixel art vẫn giữ PNG.
server-maintenance-gc = Xóa tệp không dùng
server-maintenance-gc-blurb = Xóa các tệp đã lưu mà không còn gì trỏ tới, chẳng hạn tệp của các gói đã bị xóa vĩnh viễn.
server-update-no-address = chưa đặt địa chỉ để gọi
server-update-no-answer = không có phản hồi trong 30 giây

## Cài đặt: Giao diện

appearance-language = Ngôn ngữ
appearance-language-auto = Theo trình duyệt ({ $name })

## Gốc

root-unreachable = Không thể kết nối tới máy chủ Hoard: { $error }
root-unreachable-desktop = Hoard không phản hồi: { $error }
root-desktop-reopen = Cửa sổ này đã bị đăng xuất. Hãy đóng Hoard rồi mở lại để tiếp tục.
root-try-again = Thử lại
root-lazy-connect = kết nối ứng dụng
window-minimize = Thu nhỏ
window-maximize = Phóng to
window-restore = Khôi phục
window-close = Đóng

## Ứng dụng: thư viện, thanh trên cùng và bộ lọc

app-lazy-settings = cài đặt
app-lazy-archive = kho lưu trữ
app-lazy-shortcuts = phím tắt
app-lazy-setup = thiết lập
app-theme-system = Theo chủ đề hệ thống
app-theme-light = Chủ đề sáng
app-theme-dark = Chủ đề tối
app-color-red = Đỏ
app-color-orange = Cam
app-color-yellow = Vàng
app-color-green = Xanh lá
app-color-teal = Xanh mòng két
app-color-blue = Xanh dương
app-color-purple = Tím
app-color-pink = Hồng
app-color-brown = Nâu
app-color-sand = Màu cát
app-color-gray = Xám
app-color-slate = Xám đá
app-color-black = Đen
app-color-white = Trắng
app-color-swatch = { $color }: tài nguyên có nhiều màu { $lower }
app-color-any = Màu bất kỳ
app-color-pick = Chọn màu bất kỳ
app-color = Màu
app-music = Nhạc
app-tempo = BPM
app-tempo-min = từ
app-tempo-max = đến
app-tempo-min-title = Nhạc có nhịp độ này hoặc nhanh hơn, tính theo phách mỗi phút (tính cả nửa hoặc gấp đôi)
app-tempo-max-title = Nhạc có nhịp độ này hoặc chậm hơn, tính theo phách mỗi phút
app-key = Giọng
app-key-any = Giọng bất kỳ
app-key-major = Trưởng
app-key-minor = Thứ
app-key-mix = Và các giọng phối trộn được với nó
app-key-mix-title = Giọng trưởng hoặc thứ song song của nó và các giọng cách một quãng năm ở hai bên, mà DJ thường trộn cùng
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Viết tiếng động theo cách bạn đánh vần: âm thanh giống vậy sẽ hiện trước
app-imitation-record = Tạo tiếng động vào micrô: âm thanh giống vậy sẽ hiện trước
app-imitation-stop = Dừng ghi âm
app-imitation-heard = Tiếng động của bạn
app-imitation-mic-failed = Không ghi âm được từ micrô: { $error }
app-retry = Thử lại
app-undo = Hoàn tác
app-search-failed = Tìm kiếm thất bại: { $error }
app-unreachable = Không thể kết nối tới máy chủ: { $error }
app-unreachable-desktop = Hoard không phản hồi: { $error }
app-forget-project-title = Quên { $name }?
app-forget-project-body = Hoard sẽ ngừng liệt kê dự án này ở nơi { $count } tài nguyên của nó được dùng, cho đến khi lần hoard sync tiếp theo báo lại. Không có gì trong dự án thay đổi.
app-forget-project-confirm = Quên dự án
app-forget-project-failed = Không thể quên { $name }: { $error }
app-forget-project-label = Quên dự án { $name }
app-forget-project = Quên dự án này
app-save-name-like = giống { $name }
app-save-name-favorites = yêu thích
app-search-saved = Đã lưu “{ $name }”: nằm trong Bộ lọc, mục Của bạn
app-search-updated = Đã cập nhật “{ $name }”
app-rank-alike = Giống nhất
app-rank-color = Nhiều màu đó nhất
app-rank-match = Khớp nhất
app-rank-imitation = Giống nhất
app-sort = Sắp xếp
app-sort-name = Tên
app-sort-created = Ngày tạo
app-sort-updated = Ngày cập nhật
app-sort-size = Kích thước
app-sort-rating = Xếp hạng
app-sort-newest-first = Mới nhất trước
app-sort-oldest-first = Cũ nhất trước
app-sort-a-to-z = A đến Z
app-sort-z-to-a = Z đến A
app-sort-largest-first = Lớn nhất trước
app-sort-smallest-first = Nhỏ nhất trước
app-sort-best-rated-first = Xếp hạng cao nhất trước
app-sort-lowest-rated-first = Xếp hạng thấp nhất trước
app-collection-added =
    Đã thêm { $count ->
       *[other] { $count } tài nguyên
    } vào { $name }
app-collection-add-failed = Không thể thêm vào bộ sưu tập: { $error }
app-collection-removed = Đã gỡ { $count } khỏi { $name }
app-collection-remove-failed = Không thể gỡ khỏi bộ sưu tập: { $error }
app-ns-subject = Chủ đề
app-ns-style = Phong cách
app-ns-theme = Bối cảnh
app-ns-material = Vật liệu
app-ns-map = Map PBR
app-ns-color = Màu
app-ns-mood = Không khí
app-ns-genre = Thể loại
app-ns-license = Giấy phép
app-ns-res = Độ phân giải
app-ns-duration = Độ dài
app-ns-poly = Số polygon
app-ns-format = Định dạng
app-ns-weight = Dung lượng
app-ns-tech = Kỹ thuật
app-ns-blender = Phiên bản Blender
app-ns-depth = Độ sâu bit
app-ns-ucs = Loại âm thanh (UCS)
app-ns-bpm = Nhịp độ
app-ns-root = Nốt gốc
app-ns-level = Độ lớn âm thanh
app-ns-ue = Lớp Unreal
app-ns-engine = Engine
app-ns-extracted = Nguồn đã khôi phục
app-ns-script = Hệ chữ viết
app-bulk-failed = Không thể thay đổi { $count } tài nguyên đó: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied = Đã sao chép { $count } thẻ
app-tags-copy-none = { $name } không có thẻ nào để sao chép
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Chưa sao chép thẻ nào. Nhấn C trên một tài nguyên để sao chép thẻ của nó.
# Pasting tags that every one of the assets already has.
app-tags-paste-had = Cả { $count } tài nguyên đều đã có các thẻ đó
# After pasting: $count tags were added to $assets assets.
app-tags-pasted = Đã thêm { $count } thẻ vào { $assets } tài nguyên
app-bulk-untagged = Đã bỏ thẻ { $tag } khỏi { $count } tài nguyên
app-bulk-tagged = Đã gắn thẻ { $tag } cho { $count } tài nguyên
app-bulk-review = Đánh dấu đã duyệt
app-bulk-review-title = Đánh dấu các tài nguyên đã chọn là đã xem qua: sau đó quy tắc và AI sẽ không đụng đến thẻ của chúng
app-bulk-unreview = Đánh dấu chưa duyệt
app-bulk-unreview-title = Bỏ dấu đã duyệt khỏi các tài nguyên đã chọn
app-bulk-reviewed = Đã đánh dấu đã duyệt cho { $count } tài nguyên
app-bulk-unreviewed = Đã bỏ dấu đã duyệt khỏi { $count } tài nguyên
app-bulk-accept = Giữ thẻ phỏng đoán
app-bulk-accept-title = Giữ mọi thẻ mà quy tắc và AI đã gắn cho các tài nguyên đã chọn làm thẻ của riêng bạn
app-bulk-accepted = Đã giữ thẻ phỏng đoán của { $count } tài nguyên
app-credits-failed = Không thể tạo phần ghi công: { $error }
app-archived-assets = Đã lưu trữ { $count } tài nguyên
app-archived-one = Đã lưu trữ { $name }
app-open-failed = Không thể mở tài nguyên đó: { $error }
app-unnamed-pack = gói
app-unnamed-collection = bộ sưu tập
app-unnamed-asset = tài nguyên
app-home-version = Hoard { $version } — quay lại thư viện
app-home = Quay lại thư viện
app-search-label = Tìm tài nguyên
app-search-placeholder = Tìm theo tên, đường dẫn, thẻ
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = đá tảng or đá: một trong hai · -rêu: loại trừ · "rêu khô": các từ này cùng nhau
app-views = Chế độ xem
app-library = Thư viện
app-packs = Gói
app-packs-working = { $count } đang xử lý
app-update-title = Đã có Hoard { $version } (máy chủ này đang chạy { $current }). Hãy tải image hoặc tệp chạy mới rồi khởi động lại; cơ sở dữ liệu sẽ tự nâng cấp.
app-update = Cập nhật
app-theme = Chủ đề
app-sign-in = Đăng nhập
app-archive-title = Kho lưu trữ: { $contents }
app-archive = Kho lưu trữ
app-settings = Cài đặt
app-palette-title = Bảng lệnh ({ $keys })
app-import-folder = Nhập thư mục
app-add-files = Thêm tệp
app-add-files-short = Thêm
app-split-label = Cách nhập thư mục này
app-split-before = { "" }
app-split-after = { " " }chứa { $count } thư mục hoặc tệp nén.
app-split-each = Nhập mỗi mục thành một gói riêng ({ $count })
app-split-one = Nhập thành một gói
app-cancel = Hủy
app-filters = Bộ lọc
app-done = Xong
app-yours = Của bạn
app-favorites = Yêu thích
app-needs-review = Cần duyệt
app-needs-review-title = Chỉ các tài nguyên chưa ai xem qua (dùng cùng các bộ lọc khác)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Đã xếp hạng
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
       *[other] Từ { $count } sao trở lên
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Nhấp lại để hiện mọi xếp hạng
app-forget-search-label = Quên tìm kiếm đã lưu { $name }
app-forget-search = Quên tìm kiếm này
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Mở thư viện với tìm kiếm này
app-default-search-on-title = Thư viện mở với tìm kiếm này. Nhấp để tắt.
app-default-search-label = Mở thư viện với { $name }
app-rename-search = Đổi tên
app-rename-search-label = Tên mới cho { $name }
app-projects-title = Các dự án chạy hoard sync từ đây, cùng nội dung của lần đồng bộ gần nhất
app-projects = Dự án
app-project-synced = Đã đồng bộ { $when }
app-project-synced-by = { $who } đã đồng bộ { $when }
app-collections = Bộ sưu tập
app-no-filters = Không có bộ lọc nào cho tìm kiếm này.
app-kind = Loại
app-facets-more = Thêm { $count }
app-facets-fewer = bớt
app-searching = Đang tìm…
app-searching-live = Đang tìm
app-asset-count = { $count } tài nguyên
app-clear-all = Xóa tất cả
app-save-name-label = Tên cho tìm kiếm này
app-save-name-placeholder = Đặt tên
app-save = Lưu
app-save-search-title = Giữ tìm kiếm này trong Bộ lọc, mục Của bạn
app-save-search = Lưu tìm kiếm
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Cập nhật “{ $name }”
app-update-search-title = Giữ lựa chọn hiện tại trong tìm kiếm đã lưu này
app-save-search-new = Lưu thành mới
app-select-title = Chọn nhiều tài nguyên để gắn thẻ, phân loại, đưa vào bộ sưu tập, tải xuống hoặc lưu trữ cùng lúc (X, hoặc Ctrl-nhấp)
app-select = Chọn
app-tile-size = Kích thước ô
app-supporting-title = Buffer glTF, thư viện vật liệu, các map của vật liệu, vật liệu chỉ một mô hình dùng và tệp cửa hàng mô tả mô hình. Chúng được tải kèm khi bạn tải xuống thứ dùng đến chúng. Các trang của một sprite sheet, và các sprite của nó mà gói cũng có dưới dạng ảnh riêng, được hiển thị cùng sheet. Một tài nguyên mà gói có ở nhiều định dạng được hiển thị một lần, ở định dạng Hoard đọc tốt nhất.
app-supporting-hidden = Đã ẩn { $count } tệp hỗ trợ
app-supporting-show = Hiện
app-supporting-shown = Đang bao gồm tệp hỗ trợ
app-supporting-hide = Ẩn
app-folders = Thư mục
app-folders-title = Xem gói như khi còn trên đĩa, từng thư mục một
app-folders-label = Các thư mục của gói
app-welcome-title = Thả một gói tài nguyên vào bất kỳ đâu trên trang này
app-welcome-before = Tệp zip, tarball, thư mục hay tệp rời đều được. Hoard giải nén chúng, lưu mỗi tệp một lần theo mã băm, đọc kích thước, độ dài và số polygon, rồi gắn thẻ mọi thứ để bạn có thể tìm những thứ như{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }hoặc{ " " }
app-welcome-after = .
app-welcome-import-folder = Nhập một thư mục
app-welcome-choose-files = Chọn tệp
app-none-similar = Chưa có gì khác trông giống { $name }.
app-none = Không có kết quả phù hợp.
app-none-query = Không có kết quả phù hợp với “{ $query }”.
app-none-imitation = Chưa có âm thanh nào đã đo nghe giống “{ $word }”.
app-none-heard = Chưa có âm thanh nào đã đo nghe giống tiếng động của bạn.
app-none-filters = Không có kết quả phù hợp với các bộ lọc này.
app-none-query-filters = Không có kết quả phù hợp với “{ $query }” và các bộ lọc này.
app-clear-search-filters = Xóa tìm kiếm và bộ lọc
app-loading-more = Đang tải thêm
app-keys-hint = Phím mũi tên để di chuyển, Enter để mở, Space để phát âm thanh, X hoặc Ctrl-nhấp để chọn nhiều (nhấn giữ trên màn hình cảm ứng).
app-resize-label = Đổi kích thước bảng chi tiết
app-resize-title = Kéo để đổi kích thước · nhấp đúp để có độ rộng vừa khít
app-selected-label = Tài nguyên đã chọn
app-selected = Đã chọn { $count }
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Sửa…
app-edit-selected-title = Đổi thẻ và các trường của tất cả (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
       *[other] Đã chọn { $count } kết quả
    }
app-pick-assets = Chọn tài nguyên
app-download-zip = Tải xuống zip
app-credits-title = CREDITS.md cho các gói chứa những tài nguyên này: ai đã tạo, giấy phép của chúng và dòng ghi công mà họ yêu cầu
app-credits = Ghi công
app-prefer-sources-title = SoundWave và texture Unreal mà Hoard đã khôi phục được tệp gốc
app-prefer-sources = WAV/PNG đã khôi phục thay cho .uasset ({ $count })
bag-clear = Bỏ chọn tất cả
bag-glb-heading = Mô hình dạng GLB
bag-glb-text = Mỗi mô hình là một tệp .glb đứng thẳng với trục Y hướng lên và quay về phía trước, các texture của nó là những tệp riêng trong thư mục textures mà các mô hình dùng chung. Mọi thứ khác được giữ nguyên.
bag-glb-download = Tải xuống zip GLB
bag-plain-heading = Tệp giữ nguyên
bag-plain-text = Mỗi tệp giữ nguyên như khi nhập, kèm những gì nó cần, trong một thư mục cho mỗi gói.
bag-remove = Bỏ { $name } khỏi lựa chọn
bag-more = và { $count } mục nữa
app-bulk-tag-label = Thẻ để thêm vào, hoặc bỏ khỏi, các tài nguyên đã chọn
app-bulk-tag-placeholder = Thẻ
app-bulk-tag-add-title = Thêm thẻ này cho mọi tài nguyên đã chọn
app-bulk-tag-add = Thêm
app-bulk-tag-remove-title = Bỏ thẻ này khỏi mọi tài nguyên đã chọn
app-bulk-tag-remove = Bỏ
app-remove-from-collection = Gỡ khỏi { $name }
app-unfavorite = Bỏ yêu thích
app-favorite = Yêu thích
app-select-all = Chọn tất cả { $count }
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
       *[other] Chọn tất cả { $count } kết quả
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
       *[other] Sửa { $count } tài nguyên
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
       *[other] Sửa tất cả { $count } kết quả
    }
bulk-edit-close = Đóng
bulk-edit-tags = Thẻ
bulk-edit-tag-add = Thêm thẻ cho tất cả
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } trên { $count }
bulk-edit-untick = Bỏ chọn tất cả
bulk-edit-untick-title = Gỡ mọi thẻ, rồi chọn lại những thẻ cần giữ
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = Các thẻ dùng nhiều nhất trong số đó
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Khác nhau, giữ nguyên
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Giữ nguyên
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Đã xóa ở tất cả
bulk-edit-reset = Hoàn lại như cũ
bulk-edit-all-note = Thao tác này thay đổi mọi kết quả, kể cả những kết quả chưa tải, và không thể hoàn tác trong một bước.
bulk-edit-save =
    { $count ->
       *[other] Lưu { $count } thay đổi
    }
bulk-edit-nothing = Không có gì thay đổi
bulk-edit-saved =
    { $count ->
       *[other] Đã cập nhật { $count } tài nguyên
    }
app-archive-selected-title = Không xuất hiện trong tìm kiếm cho đến khi được khôi phục hoặc xóa vĩnh viễn, từ Kho lưu trữ
app-drop-items = Thả { $count } mục để thêm
app-drop = Thả để thêm vào kho Hoard
app-drop-hint = Mỗi tệp hoặc thư mục cấp cao nhất sẽ thành một gói riêng. Hàng gigabyte cũng được; tệp lớn sẽ tiếp tục tải nếu mất kết nối.

## Bảng lệnh

palette-group-go-to = Đi tới
palette-group-actions = Thao tác
palette-group-theme = Chủ đề
palette-group-kinds = Loại
palette-group-packs = Gói
palette-group-collections = Bộ sưu tập
palette-group-projects = Dự án
palette-group-saved = Tìm kiếm đã lưu
palette-group-assets = Tài nguyên
palette-hint-current = hiện tại
palette-library = Thư viện
palette-library-keywords = tài nguyên lưới trang chủ thư viện assets grid home
palette-favorites = Yêu thích
palette-favorites-keywords = yêu thích sao đánh dấu starred stars liked
palette-packs = Gói
palette-archive = Kho lưu trữ
palette-archive-keywords = thùng rác đã xóa khôi phục lưu trữ trash recycle bin deleted restore
palette-stats = Thống kê thư viện
palette-stats-keywords = thống kê số liệu kích thước dung lượng ổ đĩa số lượng statistics size disk space count
palette-copies = Bản gần giống
palette-copies-keywords = trùng lặp giống nhau tương tự thu nhỏ chuyển đổi lãng phí dung lượng duplicates same similar resized converted wasted space dedupe
palette-search = Tìm kiếm
palette-add-files = Thêm tệp…
palette-add-files-keywords = tải lên nhập upload import
palette-import-folder = Nhập một thư mục…
palette-import-folder-keywords = tải lên gói upload pack
palette-select-all = Chọn tất cả { $count } mục đang hiện
palette-select-all-keywords = đánh dấu chọn check
palette-select-several = Chọn nhiều…
palette-select-several-keywords = hàng loạt nhiều gắn thẻ lưu trữ bulk multiple tag archive
palette-archive-selected = Lưu trữ { $count } mục đã chọn
palette-archive-selected-keywords = xóa gỡ thùng rác delete remove trash
palette-edit-selected = Sửa { $count } mục đã chọn
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = thay đổi hàng loạt thẻ danh mục giấy phép xếp hạng
palette-paste-tags = Dán các thẻ đã sao chép vào { $count } mục đã chọn
palette-clear-selection = Bỏ chọn
palette-clear-filters = Xóa bộ lọc
palette-clear-filters-keywords = đặt lại reset
palette-hide-supporting = Ẩn tệp hỗ trợ
palette-show-supporting = Hiện tệp hỗ trợ
palette-supporting-keywords = normal map roughness mask độ nhám
palette-account = Tài khoản và token truy cập
palette-account-keywords = mật khẩu đăng nhập người dùng token api password login user
palette-sign-out = Đăng xuất ({ $name })
palette-sign-out-keywords = đăng xuất thoát log out logout
palette-sign-in = Đăng nhập
palette-sign-in-keywords = đăng nhập tài khoản mật khẩu log in login account password
palette-settings = Cài đặt
palette-settings-keywords = blender ffmpeg llm ai gắn thẻ unreal quản trị công cụ tùy chọn tagging admin tools preferences
palette-shortcuts = Phím tắt
palette-shortcuts-keywords = trợ giúp phím help keys
palette-theme-keywords = chủ đề giao diện chế độ theme appearance mode
palette-theme-variant-keywords = giao diện màu appearance color
palette-light-theme = Chủ đề sáng: { $name }
palette-dark-theme = Chủ đề tối: { $name }
palette-kind-all = Mọi loại (thôi chỉ hiện { $kind })
palette-kind-only = Chỉ { $kind }
palette-title = Bảng lệnh
palette-placeholder = Tìm tài nguyên, đi tới một gói, chạy một lệnh…
palette-results = Kết quả
palette-searching = Đang tìm…
palette-no-match = Không có kết quả phù hợp với “{ $query }”
palette-foot-move = di chuyển
palette-foot-run = chạy
palette-foot-shortcuts = tất cả phím tắt

## Phím tắt

shortcuts-title = Phím tắt
shortcuts-close = Đóng
shortcuts-anywhere = Mọi nơi
shortcuts-palette = Bảng lệnh: tìm kiếm, chuyển tới, chạy
shortcuts-search = Tìm kiếm
shortcuts-this-list = Danh sách này
shortcuts-escape = Đóng, bỏ chọn, hoặc đóng chi tiết
shortcuts-library = Thư viện
shortcuts-move = Di chuyển giữa các ô (hoặc J và K)
shortcuts-open = Mở chi tiết
shortcuts-play-sound = Phát hoặc dừng âm thanh
shortcuts-select = Chọn hoặc bỏ chọn
shortcuts-favorite = Thêm vào hoặc bỏ khỏi mục yêu thích
shortcuts-reviewed = Đánh dấu đã duyệt, hoặc bỏ đánh dấu
shortcuts-tag = Thêm thẻ
shortcuts-edit = Sửa mục đã chọn
shortcuts-copy-tags = Sao chép thẻ của nó
shortcuts-paste-tags = Dán thẻ đã sao chép vào mục đã chọn
shortcuts-key-click = nhấp
shortcuts-select-range = Chọn một dải
shortcuts-select-all = Chọn mọi thứ đang hiện
shortcuts-sound-player = Trình phát âm thanh
shortcuts-play-pause = Phát hoặc tạm dừng
shortcuts-skip = Lùi lại hoặc tua tới
shortcuts-start-end = Đầu hoặc cuối
shortcuts-image-inspector = Trình xem ảnh
shortcuts-zoom = Phóng to hoặc thu nhỏ
shortcuts-fit = Vừa với cửa sổ
shortcuts-actual-pixels = Kích thước thật
shortcuts-tile = Lát 3×3 để kiểm tra đường nối
shortcuts-background = Nền tiếp theo
shortcuts-channel = Hiện một kênh

## Loại tài nguyên

kinds-group = Loại tài nguyên

## Thẻ

tag-title-with-source = { $tag } ({ $source })
tag-source-human = do người thêm
tag-source-rule = từ bộ gắn thẻ ngoại tuyến
tag-source-meta = đo từ tệp
tag-source-ai = do AI gợi ý
tag-remove = Bỏ thẻ { $tag }
tag-accept = Giữ thẻ { $tag }
tag-accept-title = Giữ phỏng đoán này làm thẻ của riêng bạn
tag-reject-title = Bỏ thẻ; bộ gắn thẻ sẽ không thêm lại
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Thẻ mới
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Ý bạn là
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = trước là { $alias }

## Hoạt động nền

activity-label-idle = Hoạt động nền: đang rảnh
activity-label-running = Hoạt động nền: { $running } đang chạy
activity-label-queued = Hoạt động nền: { $running } đang chạy, { $queued } đang chờ
activity-title = Hoạt động nền
activity-unreachable = Không thể kết nối tới máy chủ
activity-unreachable-desktop = Hoard không phản hồi
activity-empty = Không có gì đang chạy. Việc nhập, gắn thẻ, chuyển đổi và tải zip xuống sẽ hiện ở đây khi đang chạy.
activity-queued = đang chờ
activity-seconds = { $s } giây
activity-minutes = { $min } phút
activity-hours = { $h } giờ { $min } phút
activity-percent = { $pct }%

## Ô tài nguyên

tile-triangles = { $count } tam giác
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Phát { $name }
tile-pause = Tạm dừng { $name }
tile-favorite = Yêu thích { $name }
tile-in-favorites = Có trong mục yêu thích của bạn
tile-add-favorite = Thêm vào mục yêu thích (F)
tile-more-like = Thêm nữa giống { $name }
tile-more-like-this = Thêm nữa giống thế này
tile-select = Chọn { $name }

## Chọn bộ sưu tập

collections-add = Thêm vào bộ sưu tập
collections-menu = Bộ sưu tập
collections-find-label = Tìm hoặc tạo bộ sưu tập
collections-find-placeholder = Tìm hoặc đặt tên bộ sưu tập mới
collections-create = Tạo
collections-empty = Chưa có bộ sưu tập nào. Nhập tên để tạo một bộ.

## Chọn thư mục

folders-list = Thư mục
folders-close = Đóng
folders-browse = Duyệt…
folders-up-title = Lên một thư mục
folders-up = Lên
folders-this-computer = Máy tính này
folders-none = Không có thư mục nào ở đây.

## Biểu mẫu vị trí

locations-mode-question = Hoard nên làm gì với thư mục này?
locations-mode-managed = Chiếm lấy: Hoard giữ các gói của tôi ở đây
locations-mode-managed-hint = Các gói bạn tải lên hoặc nhập sẽ được sao chép vào đây, và Hoard giữ chúng ngăn nắp. Giờ thư mục này là của Hoard, nên đừng động vào nội dung bên trong.
locations-mode-watched = Canh chừng: các gói của tôi đã ở đây
locations-mode-watched-hint = Mỗi thư mục bên trong sẽ thành một gói. Hoard đọc tệp tại chỗ và không bao giờ thay đổi, di chuyển hay xóa chúng, đồng thời nhận các thư mục mới khi chúng xuất hiện.
locations-folder-desktop = Thư mục
locations-folder-placeholder = Thư mục chứa các gói của bạn
locations-folder-hint-managed = Hoard sẽ tạo thư mục nếu chưa có. Trong Docker, đây là thư mục bạn đã gắn vào container.
locations-folder-hint-managed-desktop = Hoard sẽ tạo thư mục nếu chưa có.
locations-folder-hint-watched = Trong Docker, hãy gắn thư mục vào container trước (chỉ đọc cũng được), rồi chọn nó ở đây.
locations-folder-hint-watched-desktop = Hoard chỉ đọc thư mục: không có gì trong đó bị thay đổi.
locations-name = Tên
locations-name-placeholder-managed = Thư viện
locations-name-placeholder-watched = Gói của tôi
locations-files = Tệp được đưa vào đây
locations-catch-all = Cả mọi thứ mà không vị trí nào khác nhận
locations-kinds-hint = Tệp của gói mới sẽ vào vị trí có đánh dấu loại của chúng. Phần còn lại vào vị trí chứa mọi loại.
locations-files-all = Tạm thời là tất cả. Sau này hãy thêm vị trí từ Cài đặt để đưa một số loại (ví dụ nhạc) sang ổ đĩa khác.
locations-first-watched-note = Tải lên cần một thư viện do Hoard quản lý. Bạn có thể thêm sau từ Cài đặt → Vị trí thư viện.
locations-checking = Đang kiểm tra thư mục…
locations-add = Thêm vị trí
locations-cancel = Hủy

## Bảng vị trí

locations-failed = { $what }: { $error }
locations-load-failed = Không thể tải các vị trí thư viện
locations-takes-and-rest = { $kinds }, và mọi thứ khác
locations-takes-rest = Mọi thứ mà các vị trí khác không nhận
locations-takes-everything = Mọi thứ
locations-takes-nothing = Chưa có gì
locations-save-failed = Không thể lưu vị trí
locations-unwatch-title = Ngừng theo dõi { $name }?
locations-unwatch-body = { $count } gói của nó sẽ rời khỏi Hoard, cùng với thẻ và ghi chú của chúng. Các tệp trong thư mục không bị động tới.
locations-unwatch-body-empty = Các tệp trong thư mục không bị động tới.
locations-unwatch = Ngừng theo dõi
locations-remove-title = Gỡ { $name }?
locations-remove-body = Mọi tệp trong đó sẽ được chuyển sang các vị trí khác trước, theo loại; vị trí bị gỡ khi đã trống. Không có gì bị mất.
locations-remove-body-only = Nếu nó đang chứa tệp, hãy thêm một vị trí khác để chuyển tệp sang trước.
locations-remove = Gỡ
locations-removed = Đã gỡ { $name }
locations-emptying = Đang chuyển tệp ra khỏi { $name }; vị trí sẽ bị gỡ khi xong
locations-offline-title = Không thể truy cập { $name }
locations-offline-body = Không thể chuyển tệp ra khi vị trí đang ngoại tuyến. Vẫn gỡ? Những tệp còn trên đó sẽ hiện là bị mất.
locations-remove-anyway = Vẫn gỡ
locations-remove-failed = Không thể gỡ { $name }
locations-rebalancing = Đang chuyển tệp đã lưu sang vị trí nhận loại của chúng
locations-rebalance-failed = Không thể chuyển tệp
locations-scanned-never = chưa quét
locations-scanned-now = vừa quét
locations-scanned-minutes = đã quét { $mins } phút trước
locations-scanned-hours = đã quét { $hours } giờ trước
locations-scanned-date = đã quét { $date }
locations-scan-failed = Không thể quét { $name }
locations-watching = Đang theo dõi { $name }
locations-added = Đã thêm { $name }
locations-loading = Đang tải…
locations-managed-title = Thư viện do Hoard quản lý
locations-watched-title = Thư mục theo dõi
locations-managed-hint = Tệp tải lên và nhập vào được sao chép vào đây. Mỗi vị trí nhận các loại nó đánh dấu; vị trí chứa mọi loại nhận phần còn lại.
locations-watched-hint = Thư mục chứa các gói được lập chỉ mục tại chỗ. Hoard không bao giờ thay đổi tệp trong đó, và tìm thư mục mới hoặc đã bị xóa mỗi 15 phút.
locations-managed-empty = Chưa có, nên chưa có nơi nào để chứa tệp tải lên. Hãy thêm một vị trí bên dưới.
locations-reachable = Truy cập được
locations-unreachable-now = Hiện không truy cập được
locations-catch-all-tag = chứa mọi loại
locations-packs = { $count } gói
locations-unreachable = không truy cập được
locations-scanning = Đang quét…
locations-scan = Quét ngay
locations-edit = Sửa
locations-catch-all-locked = Để thay đổi điều này, hãy đặt một vị trí khác làm vị trí chứa mọi loại
locations-edit-hint-before = Áp dụng cho tệp được thêm từ giờ trở đi. Để chuyển những gì đã lưu, hãy dùng
locations-edit-hint-after = bên dưới.
locations-save = Lưu
locations-rebalance = Chuyển tệp đã lưu cho khớp
locations-rebalance-hint = Sau khi thay đổi những gì một vị trí nhận: tệp đã lưu sẽ chuyển sang vị trí mà loại của chúng giờ thuộc về.
locations-add-title = Thêm vị trí
locations-add-hint = Một ổ đĩa khác cho một số loại tệp, hoặc một thư mục chứa các gói bạn đã có.
locations-kind-question = Đây là loại vị trí nào?
locations-kind-folder = Một thư mục
locations-kind-folder-hint = Trên ổ đĩa hoặc thư mục chia sẻ NAS đã gắn vào container, hoặc thư mục được Dropbox, Google Drive, OneDrive hay iCloud Drive đồng bộ. Hoard có thể giữ tệp ở đó, hoặc theo dõi các gói bạn đã có.
locations-kind-folder-hint-desktop = Trên máy tính này, một ổ đĩa khác hoặc thư mục chia sẻ NAS, hoặc thư mục được Dropbox, Google Drive, OneDrive hay iCloud Drive đồng bộ. Hoard có thể giữ tệp ở đó, hoặc theo dõi các gói bạn đã có.
locations-kind-s3-hint = Một bucket trong Amazon S3 hoặc dịch vụ tương tự: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard giữ tệp ở đó.
locations-s3-title = Kho lưu trữ S3
locations-s3-hint = Vị trí s3://bucket-name là một bucket trong dịch vụ này: AWS, hoặc bất kỳ dịch vụ tương thích S3 nào (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage trên NAS).
locations-bucket = Bucket
locations-bucket-hint = Tên một bucket đã có sẵn. Các khóa ở trên phải liệt kê, đọc và ghi được vào bucket đó.
locations-s3-endpoint = Địa chỉ
locations-s3-endpoint-placeholder = Để trống cho AWS; ví dụ https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Vùng
locations-s3-access-key = ID khóa truy cập
locations-s3-secret-key = Khóa truy cập bí mật
locations-s3-headers = Header bổ sung
locations-s3-headers-placeholder = Name=value, mỗi dòng một mục (ví dụ cho Cloudflare Access)
locations-s3-kept = Đã lưu (nhập để thay thế)
locations-s3-secrets = Khóa và header được lưu cùng cài đặt của máy chủ, và sẽ không bao giờ hiện lại.
locations-s3-secrets-desktop = Khóa và header được lưu trong kho mật khẩu của máy tính này, và sẽ không bao giờ hiện lại.
locations-s3-saved = Đã lưu kho lưu trữ S3
locations-s3-save-failed = Không thể lưu kho lưu trữ S3
locations-add-more = Thêm vị trí…

## Bảng sự cố

issues-failed = { $what }: { $error }
issues-load-failed = Không thể tải Sự cố
issues-started =
    { $count ->
       *[other] Đã chạy lại { $count } mục.
    }
issues-retry-failed = Không thể thử lại
issues-dismiss-failed = Không thể bỏ qua
issues-dismiss-all-title = Bỏ qua tất cả { $count }?
issues-dismiss-all-body = “{ $title }” sẽ được xóa. Nếu lại có lỗi, nó sẽ được liệt kê lại.
issues-dismiss-all = Bỏ qua tất cả
issues-just-now = vừa xong
issues-minutes-ago = { $mins } phút trước
issues-hours-ago = { $hours } giờ trước
issues-loading = Đang tải…
issues-none = Không có gì trục trặc.
issues-action-all = { $action } tất cả
issues-open-asset = Mở
issues-open-pack = Hiện gói
issues-open-locations = Vị trí thư viện
issues-where-before = trong
issues-where-after = { "" }
issues-times = { $count } lần
issues-starting = Đang bắt đầu…
issues-dismiss-named = Bỏ qua { $title }
issues-dismiss = Bỏ qua
issues-show-fewer = Hiện ít hơn
issues-show-all = Hiện tất cả { $count }
issues-not-listed = { $count } mục khác không được liệt kê.

## Lưu trữ

archive-anyway = Vẫn lưu trữ
archive-failed = Không thể lưu trữ: { $error }
archive-undo = Hoàn tác
archive-restore-failed = Không thể khôi phục: { $error }
archive-count-packs-and-assets = { $packs } gói và { $assets } tài nguyên
archive-count-packs = { $packs } gói
archive-count-assets = { $assets } tài nguyên
archive-count-nothing = không có gì

## Kho lưu trữ

archive-load-failed = Không thể tải kho lưu trữ: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Đã khôi phục { $what }
archive-purged = Đã xóa vĩnh viễn { $what }
archive-purge-failed = Không thể xóa: { $error }
archive-purge-title = Xóa vĩnh viễn { $name }?
archive-purge-pack-body = { $count } tài nguyên và { $size } tệp của gói cũng sẽ bị xóa, trừ những tệp mà gói khác dùng chung. Không thể hoàn tác.
archive-purge = Xóa vĩnh viễn
archive-purge-asset-body = Tệp của nó sẽ bị xóa, trừ khi một tài nguyên khác dùng chung. Không thể hoàn tác.
archive-empty-title = Dọn sạch kho lưu trữ?
archive-empty-body = { $what } sẽ bị xóa vĩnh viễn và tệp của chúng được giải phóng. Không thể hoàn tác.
archive-empty = Dọn sạch kho lưu trữ
archive-title = Kho lưu trữ
archive-intro = Gói và tài nguyên đã lưu trữ không xuất hiện trong tìm kiếm, số đếm và danh sách thẻ. Hãy khôi phục chúng, hoặc xóa vĩnh viễn để giải phóng dung lượng.
archive-loading = Đang tải…
archive-is-empty = Kho lưu trữ trống
archive-is-empty-hint = Lưu trữ một gói hoặc tài nguyên và nó sẽ chờ ở đây cho đến khi bạn khôi phục hoặc xóa vĩnh viễn.
archive-packs = Gói
archive-assets = Tài nguyên
archive-asset-count = { $count } tài nguyên
archive-archived = đã lưu trữ { $when }
archive-from = từ { $pack }
archive-restore = Khôi phục
archive-show-more = Hiện thêm (còn { $count })

## Của tôi

projects-favorite-add-failed = Không thể thêm vào mục yêu thích: { $error }
projects-favorite-remove-failed = Không thể bỏ khỏi mục yêu thích: { $error }
projects-save-search-failed = Không thể lưu tìm kiếm: { $error }
projects-forget-search-failed = Không thể gỡ { $name }: { $error }
projects-change-search-failed = Không thể thay đổi { $name }: { $error }

## Dự án

projects-list-and = { $items } và { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } tệp khác
projects-other = { $count } dự án khác
projects-usage-title-one = { $name } đang dùng mục này
projects-usage-title = { $count } dự án đang dùng mục này
projects-usage-body-one = Được dùng trong { $list }: lần hoard sync tiếp theo của dự án sẽ gỡ chúng ra.
projects-usage-body = Được dùng trong { $list }: lần hoard sync tiếp theo của mỗi dự án sẽ gỡ chúng ra.

## Tải lên

upload-network-error = lỗi mạng
upload-gave-up = { $what }: { $error } sau { $attempts } lần thử
upload-failed-plain = thất bại
upload-already-imported = đã nhập rồi
upload-part-failed = tải lên thất bại ({ $status })
upload-batch = lô { $count } tệp
upload-batch-failed = lô thất bại ({ $status })
upload-finish = hoàn tất tải lên

## Hàng đợi tải lên

upload-eta-seconds = còn { $s } giây
upload-eta-minutes = còn { $min } phút
upload-eta-hours = còn { $h } giờ
upload-queue = Nhập
upload-importing = Đang nhập { $count } gói
upload-bytes-of = { $sent } / { $total }
upload-rate = { $size }/s
upload-waiting = Đang chờ
upload-unpacking = Đã tải lên, đang giải nén
upload-already = Đã có trong thư viện
upload-stopped = Đã dừng
upload-files-of = { $done } / { $total } tệp
upload-resumed = đã tiếp tục, { $size } đã có sẵn
upload-skipped = Chính những tệp này đã được nhập dưới tên “{ $name }”, nên đã bỏ qua.
upload-error = { $error }. Tiến độ của bạn được giữ lại: hãy thử lại, hoặc thả lại cùng các tệp đó sau.
upload-retry = Thử lại
# $name is the pack the library has already that these files go into.
upload-update = Cập nhật “{ $name }”
upload-copy = Nhập thành bản sao
# $name is the pack the library has already that the upload goes into.
upload-into = Đang vào “{ $name }”, gói đã có trong thư viện: tệp mới được thêm và tệp đã đổi được thay thế.
upload-own-pack = Tách thành gói riêng
upload-dismiss = Bỏ qua
upload-cancel = Hủy
upload-minimize = Thu nhỏ
upload-expand = Hiện các lần nhập
upload-pill = Đang nhập { $count } · { $percent }%
upload-pill-stopped = Đã dừng: { $count }
upload-interrupted = Bị gián đoạn
upload-resume = Tiếp tục
upload-resume-all = Tiếp tục tất cả
upload-resume-hint-folder = Chọn lại đúng thư mục đó, hoặc thả nó vào trang, để tiếp tục từ chỗ đã dừng.
upload-resume-hint-file = Chọn lại đúng tệp đó, hoặc thả nó vào trang, để tiếp tục từ chỗ đã dừng.
upload-not-same = Đó không phải là cùng các tệp, nên không có gì được tiếp tục.
upload-permission-denied = Trình duyệt không được phép đọc lại các tệp. Hãy chọn chúng thay vào đó.

## Trang gói

packs-rename-failed = Không thể đổi tên: { $error }
packs-collection-deleted = Đã xóa bộ sưu tập { $name }
packs-delete-failed = Không thể xóa: { $error }
packs-status-uploading = Đang tải lên (các tệp đã nhận đến giờ)
packs-status-ingesting = Đang giải nén và gắn thẻ
packs-status-analyzing = LLM đang gắn thẻ
packs-status-ready = Sẵn sàng
packs-status-failed = Thất bại
packs-retag-failed = Không thể bắt đầu gắn thẻ lại: { $error }
packs-read-files-failed = Không thể đọc các tệp của gói: { $error }
packs-context-load-failed = Không thể tải thông tin cửa hàng của gói: { $error }
packs-undo-store-failed = Không thể gỡ trang cửa hàng: { $error }
packs-context-save-failed = Không thể lưu thông tin cửa hàng: { $error }
packs-unreal-load-failed = Không thể tải trạng thái chuyển đổi Unreal: { $error }
packs-unreal-discard-title = Bỏ các tệp gốc?
packs-unreal-discard-body = Các tệp .uasset gốc trong { $name } sẽ bị xóa sau khi chuyển đổi. Dữ liệu chỉ Unreal mới có trong đó, như đồ thị vật liệu, sẽ mất vĩnh viễn.
packs-unreal-discard-confirm = Chuyển đổi và bỏ tệp gốc
packs-unreal-convert-failed = Không thể bắt đầu chuyển đổi: { $error }
packs-unreal-fact-converted = { $done } đã chuyển đổi từ Unreal
packs-unreal-fact-packages = { $left } package Unreal
packs-unreal-fact-both = { $done } đã chuyển đổi từ Unreal, { $left } vẫn là package
packs-thumbs-canceled = Đã hủy sau { $done } / { $total }
packs-thumbs-drew = Đã vẽ { $count } mô hình
packs-thumbs-result = { $lead }: đã thay { $improved } ảnh thu nhỏ.
packs-thumbs-result-failed = { $lead }: đã thay { $improved } ảnh thu nhỏ, { $failed } không vẽ được.
packs-archived = Đã lưu trữ { $name } và { $count } tài nguyên của nó
packs-blobs-bad = { $count } tệp không qua được lần kiểm tra toàn vẹn gần nhất.
packs-collections = Bộ sưu tập
packs-browse-named = Duyệt { $name }
packs-collection-name = Tên bộ sưu tập
packs-save = Lưu
packs-cancel = Hủy
packs-assets = { $count } tài nguyên
packs-updated = Cập nhật { $date }
packs-browse = Duyệt
packs-collection-download = Tải xuống (.zip)
packs-collection-credits = Ghi công
packs-collection-credits-title = Ai đã tạo các gói chứa những tài nguyên này, giấy phép của chúng và dòng ghi công mà họ yêu cầu
packs-rename = Đổi tên
packs-delete = Xóa
packs-packs = Gói
packs-empty = Chưa có gói nào. Thả một tệp zip hoặc thư mục vào bất kỳ đâu trên trang này để thêm.
packs-find = Tìm gói
packs-find-placeholder = Tìm gói theo tên, cửa hàng, giấy phép hoặc thẻ
packs-sort = Thứ tự
packs-sort-new = Mới nhất trước
packs-sort-old = Cũ nhất trước
packs-sort-size = Lớn nhất trước
packs-sort-assets = Nhiều tài nguyên nhất trước
packs-sort-count = Số tài nguyên
packs-sort-smallest = Nhỏ nhất trước
packs-sort-fewest = Ít tài nguyên nhất trước
packs-with-kind = Gói có
packs-state = Trạng thái
packs-state-any = Mọi trạng thái
packs-state-working = Đang xử lý
packs-state-failed = Thất bại
packs-state-untagged = Chưa được mô tả
packs-state-unlicensed = Chưa ghi giấy phép
packs-store = Cửa hàng
packs-store-any = Mọi cửa hàng
packs-store-from = Từ { $store }
packs-store-none = Không có trang cửa hàng
packs-shown-of = { $shown } / { $total }
packs-clear = Xóa
packs-no-match = Không có gói nào phù hợp.
packs-clear-all = Xóa tìm kiếm và bộ lọc
packs-pack-name = Tên gói
packs-thumbs-drawing = Đang vẽ ảnh thu nhỏ trong trình duyệt này…
packs-thumbs-drawing-count = Đang vẽ ảnh thu nhỏ trong trình duyệt này: { $done } / { $total }…
packs-thumbs-stopped = Đã dừng: { $error }
packs-thumbs-none = Mọi mô hình ở đây đều đã có ảnh thu nhỏ được vẽ với đầy đủ texture.
packs-dismiss = Bỏ qua
packs-license = Giấy phép: { $license }
packs-license-tier = Giấy phép: { $license } ({ $tier })
packs-license-not-found = không tìm thấy
packs-from-before = Từ
packs-from-after = { "" }
packs-added = Đã thêm { $date }
pack-panel-label = Về gói này
pack-panel-hide = Ẩn chi tiết của gói
pack-panel-kinds = Có gì bên trong
pack-panel-all = Tất cả các gói
packs-download = Tải gói xuống (.zip)
packs-store-details = Thông tin cửa hàng
packs-more-named = Thêm thao tác cho { $name }
packs-more = Thêm
packs-rename-more = Đổi tên…
packs-retag-rules = Chạy lại bộ gắn thẻ ngoại tuyến
packs-retag-llm = Chạy lại bộ gắn thẻ LLM
packs-make-thumbs = Tạo ảnh thu nhỏ trong trình duyệt này
packs-make-thumbs-title = Mở từng mô hình trong một trình xem 3D ẩn, lần lượt từng cái, và dùng hình của nó làm ảnh thu nhỏ khi đẹp hơn ảnh của máy chủ
packs-make-thumbs-title-desktop = Mở từng mô hình trong một trình xem 3D ẩn, lần lượt từng cái, và dùng hình của nó làm ảnh thu nhỏ khi đẹp hơn ảnh do Hoard tạo
packs-read-files = Đọc lại các tệp của gói
packs-credits = Tải xuống phần ghi công
packs-credits-title = CREDITS.md: ai đã tạo gói, giấy phép của nó và dòng ghi công mà nó yêu cầu
packs-unreal-open = Chuyển đổi nội dung Unreal…
packs-archive = Lưu trữ gói
packs-unreal-intro = Hoard tự đọc các package Unreal và lưu nội dung bên trong thành tệp thông thường: texture thành PNG hoặc EXR, âm thanh thành WAV, vật liệu thành JSON. Tệp đã chuyển đổi thay thế package, giữ nguyên thẻ, liên kết và bộ sưu tập của nó.
packs-unreal-converted =
    { $count ->
       *[other] đã chuyển đổi
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] đã chuyển đổi, { $kept } còn giữ tệp gốc
    }
packs-unreal-pending =
    { $count ->
       *[other] chưa được xem xét
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] đang chờ Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] không có định dạng phổ biến
    }
packs-unreal-failed =
    { $count ->
       *[other] thất bại
    }
packs-unreal-reasons = Vì sao còn package
packs-unreal-editor-before = Mesh và hoạt ảnh cần Unreal Editor. Trên một máy có cài Unreal, hãy chạy
packs-unreal-editor-command = hoard unreal-worker --editor <đường dẫn tới UnrealEditor-Cmd>
packs-unreal-editor-after = ; lệnh này xuất chúng thành FBX và tải kết quả lên đây.
packs-unreal-discard = Bỏ các tệp .uasset gốc sau khi chuyển đổi
packs-unreal-discard-hint = Giải phóng dung lượng. Dữ liệu chỉ Unreal mới có, như đồ thị vật liệu, sẽ bị mất.
packs-unreal-drop = Xóa các package không có định dạng phổ biến
packs-unreal-drop-hint = Blueprint, level và các tài nguyên khác chỉ dùng được trong engine.
packs-unreal-convert = Chuyển đổi
packs-context-hint-before = Gói đến từ đâu và người bán nói gì về nó. Bộ gắn thẻ LLM đọc phần này cùng mọi tệp, nên một đoạn mô tả của cửa hàng như “120 phép băng và lửa” giúp nó đặt tên cho
packs-context-hint-after = .
packs-store-auto = Tìm thấy tự động: chính gói nêu tên trang này.
packs-store-not-this = Không phải trang này
packs-lookup-label = Tìm trên các cửa hàng
packs-lookup-placeholder = Tên của gói trên cửa hàng
packs-lookup-searching = Đang tìm…
packs-lookup-search = Tìm trên cửa hàng
packs-lookup-hide = Ẩn
packs-lookup-clues = Từ các tệp của gói: { $clues }.
packs-lookup-also = Cũng đã tìm { $queries }.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Không có gì trên Fab, itch.io, Kenney hay OpenGameArt trông giống gói này. Hãy thử ít từ hơn hoặc từ khác.
packs-lookup-use = Dùng
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Hiện tại
packs-review-store = Từ cửa hàng
packs-review-same = Giống nhau
packs-review-read-page = Đọc từ trang khi lưu
packs-review-take =
    { $count ->
       *[other] Điền { $count } trường
    }
packs-lookup-searched = Đã tìm { $date }.
packs-lookup-again = Tìm lại
packs-lookup-again-after = .
packs-lookup-unreachable = Lần này không thể kết nối tới { $stores }.
packs-lookup-also-look = Hãy tìm thêm trên
packs-lookup-links-and = { " " }và{ " " }
packs-lookup-browser-only = (chỉ có thể tìm trong trình duyệt của bạn); hãy dán địa chỉ trang bên dưới.
packs-lookup-open = Tìm trên cửa hàng…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Người bán
packs-seller-placeholder = Ai đã tạo
packs-license-label = Giấy phép
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Hạng giấy phép
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
       *[other] { $count } tài nguyên có giấy phép riêng: chuyển sang dùng giấy phép của gói
    }
packs-store-page = Trang cửa hàng
packs-store-text = Mô tả từ cửa hàng
packs-store-text-placeholder = Dán nội dung trang sản phẩm, hoặc để trống để lấy từ trang cửa hàng.
packs-store-text-fetch = Khi lưu, mô tả sẽ được lấy từ trang cửa hàng.
packs-readme = README tìm thấy trong gói
packs-save-retag = Lưu và chạy lại bộ gắn thẻ LLM

## Chi tiết tài nguyên

asset-lazy-3d-viewer = trình xem 3D
asset-lazy-text = văn bản
asset-lazy-text-viewer = trình xem văn bản
asset-lazy-image-viewer = trình xem ảnh
asset-lazy-material-preview = bản xem trước vật liệu
asset-lazy-material-viewer = trình xem vật liệu
asset-lazy-sprite-player = trình phát sprite
asset-lazy-sheet-view = trình xem bảng sprite
asset-rel-texture = texture
asset-rel-buffer = dữ liệu hình học
asset-rel-material = tệp vật liệu
asset-rel-animation = hoạt ảnh
asset-rel-library = tệp liên kết
asset-rel-guess = có lẽ đi kèm với
asset-rel-sidecar = thông tin cửa hàng
asset-rel-variant = biến thể
asset-rel-sprite = sprite
asset-rel-sample = mẫu âm thanh
asset-role-map = map { $role }
asset-maps-count = { $count } map
asset-needs = Cần
asset-animations = Hoạt ảnh
asset-sprites = Sprite
asset-used-by = Được dùng bởi
asset-loose-sprites = Cũng có dưới dạng tệp riêng
asset-in-sheet = Trong sprite sheet
asset-other-formats = Định dạng khác
asset-shown-as = Hiển thị trong thư viện dưới dạng
asset-conversion-failed = chuyển đổi thất bại
asset-as-converting = Đang chuyển đổi…
asset-as-converting-percent = Đang chuyển đổi… { $percent }%
asset-as-queued = Đang chờ…
asset-download = Tải xuống
asset-as-label = Tải xuống dưới dạng
asset-as-original = { $format } (định dạng gốc)
asset-as-pack-files = Tệp của chính gói
asset-as-size = Kích thước
asset-as-full-size = Kích thước đầy đủ
asset-as-up-to = Tối đa { $size } px
asset-as-hint = Chọn kích thước hoặc định dạng khác
asset-meta-size = Kích thước
asset-meta-length = Độ dài
asset-meta-sample-rate = Tần số lấy mẫu
asset-meta-channels = Kênh
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Độ sâu bit
asset-meta-bits = { $bits }-bit
asset-meta-level = Mức âm lượng
asset-meta-level-value = đỉnh { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = đỉnh { $peak } dBFS, RMS { $rms } dBFS, bị clip
asset-meta-loop = Vòng lặp
asset-meta-loop-value = { $start } đến { $end }
asset-meta-playback = Phát
asset-meta-one-shot = Phát một lần
asset-meta-envelope = Đường bao
asset-meta-envelope-times = độ bật { $attack }, đuôi { $tail }
asset-meta-envelope-hit = cú đánh
asset-meta-envelope-swell = tăng dần
asset-meta-envelope-sustained = kéo dài
asset-meta-tone = Âm sắc
asset-meta-tone-value = độ sáng { $khz } kHz, nhiễu { $noise }%
asset-meta-pitch = Cao độ
asset-meta-pitch-rises = tăng { $octaves } quãng tám
asset-meta-pitch-falls = giảm { $octaves } quãng tám
asset-meta-loops-cleanly = Lặp liền mạch
asset-meta-tempo = Nhịp độ
asset-meta-tempo-heard = khoảng { $bpm } BPM, đo khi nghe
asset-meta-tempo-heard-or = khoảng { $bpm } BPM (hoặc { $alt }), đo khi nghe
asset-meta-key = Giọng
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, đo khi nghe
music-key-major = { $note } trưởng
music-key-minor = { $note } thứ
asset-meta-root-note = Nốt gốc
asset-meta-cue-markers = Điểm đánh dấu cue
asset-meta-library = Thư viện
asset-meta-keywords = Từ khóa
asset-meta-title = Tiêu đề
asset-meta-artist = Nghệ sĩ
asset-meta-album = Album
asset-meta-genre = Thể loại
asset-meta-generator = Trình tạo
asset-meta-everything-with = Hiện mọi thứ có thẻ { $tag }
asset-meta-saved-with = Lưu bằng
asset-meta-color-mode = Chế độ màu
asset-meta-layers = Lớp
asset-meta-range = Dải
asset-meta-hdr = HDR (bản xem trước đã được tone-map)
asset-meta-format = Định dạng
asset-meta-large-document = Tài liệu lớn (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Được tạo bằng AI tạo sinh
asset-meta-ai-edited = Được chỉnh sửa bằng AI tạo sinh
asset-meta-ai-how-metadata = siêu dữ liệu
asset-meta-ai-how-generator = cài đặt trình tạo
asset-meta-ai-how-iptc = loại nguồn kỹ thuật số IPTC
asset-meta-ai-says = { $made }, theo { $how } của tệp
asset-meta-ai-by-says = { $made } ({ $by }), theo { $how } của tệp
asset-meta-provenance = Nguồn gốc
asset-meta-c2pa = Có C2PA Content Credentials
asset-meta-compression = Nén
asset-meta-note = Ghi chú
asset-meta-maps = Map
asset-meta-sizes = Kích thước
asset-meta-triangles = Tam giác
asset-meta-vertices = Đỉnh
asset-meta-meshes = Mesh
asset-meta-materials = Vật liệu
asset-meta-animations = Hoạt ảnh
asset-meta-skins = Skin
asset-meta-unreal-asset = Tài nguyên Unreal
asset-meta-unreal-saved-with = { $class }, lưu bằng UE { $engine }
asset-meta-package = Package
asset-meta-cooked = Đã cook (bản build game): bản xem trước và nguồn đã bị loại bỏ
asset-meta-tiling = Lát
asset-meta-seamless = Liền mạch trên cả hai trục
asset-meta-analysis-error = Không thể đọc hết tệp này: { $error }
asset-copy-blocked = Không thể sao chép: trình duyệt đã chặn
asset-save-error = Không thể lưu: { $error }
asset-tag-removed = Đã bỏ { $tag }
asset-tag-restored = Đã đặt lại { $tag }
asset-tags-added = Đã thêm { $count } thẻ
asset-tags-accepted = Đã giữ { $count } thẻ phỏng đoán
asset-reviewed = Đã duyệt
asset-reviewed-on-title = Đã duyệt: quy tắc và AI không đụng đến thẻ của nó. Nhấp để bỏ dấu (R)
asset-reviewed-off-title = Đánh dấu đã duyệt, để quy tắc và AI không đụng đến thẻ của nó (R)
asset-reviewed-on = Đã đánh dấu đã duyệt
asset-reviewed-off = Đã bỏ dấu đã duyệt
# $name is an asset's name.
asset-reviewed-on-named = Đã đánh dấu đã duyệt cho { $name }
asset-reviewed-off-named = Đã bỏ dấu đã duyệt khỏi { $name }
asset-undo = Hoàn tác
asset-play-error = Không thể phát tệp này: { $error }
asset-copied-hash = Đã sao chép mã băm
asset-copied-toml = Đã sao chép toml
asset-copied-id = Đã sao chép id
asset-copied-link = Đã sao chép liên kết
asset-origin-before = Tệp này vốn là
asset-origin-class-engine = , một { $class } được lưu bởi UE { $engine }.
asset-origin-class = , một { $class }.
asset-origin-engine = { " " }được lưu bởi UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor đã xuất tệp này vào { $date }.
asset-origin-editor-notes = Unreal Editor đã xuất tệp này vào { $date } ({ $notes }).
asset-origin-hoard = Hoard đã đọc tệp này ra từ package vào { $date }.
asset-origin-hoard-notes = Hoard đã đọc tệp này ra từ package vào { $date } ({ $notes }).
asset-origin-download = Tải xuống tệp .{ $ext } gốc
asset-origin-discarded = Package gốc đã bị bỏ sau khi chuyển đổi.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Lịch sử
history-loading = Đang tải…
history-none = Chưa có gì thay đổi ở đây.
history-load-failed = Không thể tải lịch sử: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = AI
history-by-rule = Một quy tắc
history-by-someone = Ai đó
history-by-app = { $who } qua một ứng dụng
history-by-assistant = { $who } qua một trợ lý
history-field-description = Mô tả
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Đã thêm thẻ { $tag }
history-tag-removed = Đã bỏ thẻ { $tag }
history-tag-kept = Đã giữ thẻ được gợi ý { $tag }
history-tag-changed = Đã đổi thẻ { $tag }
history-reviewed = Đã đánh dấu là đã xem xét
history-unreviewed = Đã đánh dấu là chưa xem xét
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: đặt thành { $to }
history-cleared = { $field }: đã xóa (trước đó là { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Hoàn tác
history-revert-title = Khôi phục lại như trước thay đổi này
history-reverted = Đã hoàn tác: { $change }
history-revert-failed = Không thể hoàn tác: { $error }
asset-details = Chi tiết tài nguyên
asset-in-pack = trong
asset-previous = Tài nguyên trước
asset-next = Tài nguyên tiếp theo
asset-favorite = Yêu thích
asset-favorite-on-title = Có trong mục yêu thích (F để bỏ ra)
asset-favorite-off-title = Thêm vào mục yêu thích (F)
asset-share = Chia sẻ
asset-share-title = Chia sẻ liên kết tới tài nguyên này (người mở sẽ đăng nhập Hoard trước)
asset-layout-panel = Hiện trong bảng bên
asset-layout-page = Hiện toàn trang
asset-layout-page-title = Hiện toàn trang: bản xem trước lớn bên cạnh chi tiết
asset-close = Đóng chi tiết
asset-rename-title = Nhấp để đổi tên
asset-name = Tên
asset-views-alt = Mô hình nhìn từ bốn hướng
asset-inspect-aria = Xem kỹ { $name } toàn màn hình
asset-inspect-title = Xem kỹ: phóng to, lát, kênh
asset-playback-position = Vị trí phát
asset-playback-value = { $current } / { $duration }
asset-pause = Tạm dừng
asset-play = Phát
asset-waveform-drawing = Đang vẽ dạng sóng…
asset-waveform-skipped = Không có dạng sóng cho tệp lớn thế này
asset-play-animation = Phát như hoạt ảnh
asset-show-views = Hiện bảng bốn góc nhìn mà bộ gắn thẻ đã xem
asset-palette = Màu trong ảnh
asset-swatch-title = { $hex }, { $percent }% ảnh. Tìm tài nguyên có màu này
asset-swatch-aria = Tìm tài nguyên có { $hex }, { $percent }% ảnh này
asset-more-like-this = Thêm nữa giống thế này
asset-desc-label = Mô tả của bạn
asset-desc-placeholder = Nó là gì, bạn sẽ dùng nó ở đâu…
asset-save = Lưu
asset-cancel = Hủy
asset-desc-family = Lưu cho cả { $count } biến thể
asset-desc-edit = Sửa mô tả
asset-desc-write = Viết mô tả
asset-desc-ai = Do AI viết
asset-desc-file = Từ tệp
asset-desc-store-before = Từ
asset-desc-store-page = trang { $store }
asset-desc-store-page-unnamed = trang cửa hàng
asset-desc-store-after = (cả gói)
asset-less = Thu gọn
asset-more = Thêm
asset-tags = Thẻ
asset-tag-add-label = Thêm thẻ
asset-tag-add-placeholder = Thêm thẻ, ví dụ mood:tense
asset-apply-family = Áp dụng thay đổi cho cả { $count } biến thể
asset-tags-accept-all = Giữ cả { $count } phỏng đoán
asset-tags-accept-all-title = Giữ mọi thẻ mà quy tắc và AI đã đoán làm thẻ của riêng bạn
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Đã từ chối:
asset-tag-restore-title = Đặt lại { $tag }
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = Những mục giống có:
asset-tag-suggest-title = Thêm { $tag }
asset-details-heading = Chi tiết
asset-field-category = Danh mục
asset-field-license = Giấy phép
asset-field-source = Nguồn
asset-field-rating = Xếp hạng
# Shown in place of an empty value, to fill it in.
asset-field-add = Thêm
asset-field-new = Thêm trường
asset-field-name = Tên trường
asset-field-value = Giá trị
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = từ gói
asset-field-license-placeholder = Như CC0 hoặc CC-BY 4.0
asset-field-remove-placeholder = Để trống để xóa
# $field is a field's name, like "License".
asset-field-edit-title = Sửa { $field }
asset-field-saved = Đã lưu { $field }
asset-field-cleared = Đã xóa { $field }
asset-rate = { $count } sao
asset-rated = Đã xếp hạng { $count } sao
asset-rating-clear-title = Nhấp lại để xóa xếp hạng
asset-rating-cleared = Đã xóa xếp hạng
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
       *[other] Nhấp lại để chọn { $count } sao
    }
asset-collections = Bộ sưu tập
asset-collection-remove = Gỡ khỏi { $name }
asset-collection-add-another = Thêm vào bộ khác
asset-collection-add = Thêm vào bộ sưu tập
asset-used-in = Được dùng trong
asset-used-in-title = Hiện những gì { $name } có
asset-used-in-synced = đồng bộ { $when }
asset-maps = Map
asset-show-more = Hiện thêm { $count }
asset-materials-heading = Vật liệu
asset-material-maps-aria = { $name }: map
asset-variants = Biến thể ({ $count } phiên bản)
asset-fit-animations = Hoạt ảnh phù hợp ({ $count })
asset-fit-characters = Nhân vật phù hợp ({ $count })
asset-copies = Bản sao giống hệt
asset-copies-note = Cùng một nội dung chỉ được lưu một lần; các gói này cũng có tệp này.
asset-near-copies = Bản gần giống
asset-near-copies-note = Cùng một thứ ở kích thước, tần số lấy mẫu hoặc định dạng khác. Bản tốt nhất được đánh dấu.
asset-near-copies-best = Cùng một thứ ở kích thước, tần số lấy mẫu hoặc định dạng khác. Đây là bản tốt nhất.
asset-near-best = tốt nhất
asset-unreal-converted = Đã chuyển đổi từ Unreal
asset-rule-converted = Đã chuyển đổi bởi quy tắc
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Một quy tắc chuyển đổi đã tạo tệp { $format } này vào { $date } từ
asset-rule-kept-until = Tệp gốc được giữ đến { $date }.
asset-rule-rebuilt = FLAC giữ mọi thứ có trong WAV, nên có thể tải xuống hoặc đưa WAV trở lại y như cũ bất cứ lúc nào.
asset-rule-put-back = Đưa tệp gốc trở lại
asset-rule-put-back-title = Đưa { $file } trở lại?
asset-rule-put-back-body = Tệp đã chuyển đổi bị xóa và tệp gốc thế chỗ, kèm theo thẻ, bộ sưu tập và liên kết của nó. Từ đó trở đi, các quy tắc chuyển đổi không động đến nó nữa.
asset-rule-put-back-done = Đã đưa { $file } trở lại
asset-rule-let-go = Tệp gốc đã bị xóa khi hết thời gian giữ tệp gốc trong Cài đặt.
asset-unreal-package = Từ package Unreal
asset-ue-needs-editor-before = Đang chờ Unreal Editor: { $reason }. Trên một máy có cài Unreal, hãy chạy
asset-ue-needs-editor-after = và nó sẽ trở lại dưới dạng FBX.
asset-ue-editor-path = đường dẫn tới UnrealEditor-Cmd
asset-ue-unsupported = Giữ nguyên dạng package: { $reason }.
asset-ue-failed = Chuyển đổi thất bại: { $reason }
asset-recovered = Tệp .{ $ext } gốc được lưu bên trong package này, và Hoard đã lấy nó ra.
asset-recovered-download = Tải xuống .{ $ext }
asset-ue-meshes-before = Mesh và hoạt ảnh được lưu ở định dạng riêng của Unreal. Hãy chuyển đổi gói (trang gói, Thêm), rồi chạy
asset-ue-meshes-after = cạnh một bản cài Unreal Editor để có FBX.
asset-ue-registry-tags = { $count } thẻ asset registry
asset-file = Tệp
asset-bytes = Byte
asset-source = Nguồn
asset-path = Đường dẫn
asset-path-open-folder = Xem còn gì khác trong { $folder }
asset-download-maps = Tải xuống cả { $count } map
asset-download-linked = Tải xuống cùng { $count } tệp liên kết
asset-download-only-title = Chỉ tệp này, không kèm các tệp nó dùng
asset-download-only = Chỉ tệp này
asset-include-animations = Kèm hoạt ảnh trong tệp .zip
asset-include-character = Kèm nhân vật { $name }
asset-copy-toml = Sao chép mục hoard.toml
asset-copy-id = Sao chép ID
asset-teach = Dạy Hoard đọc tệp này
asset-teach-title = Mô tả tệp của cửa hàng này để Hoard đọc được (Cài đặt → Tệp cửa hàng)
asset-archive = Lưu trữ
asset-archive-title = Không xuất hiện trong tìm kiếm cho đến khi được khôi phục hoặc xóa vĩnh viễn, từ Kho lưu trữ

## Trình xem ảnh

image-channel-red = Kênh đỏ
image-channel-green = Kênh xanh lá
image-channel-blue = Kênh xanh dương
image-channel-alpha = Kênh alpha
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Nền ô cờ (K để chuyển)
image-bg-black-title = Nền đen (K để chuyển)
image-bg-gray-title = Nền xám (K để chuyển)
image-bg-white-title = Nền trắng (K để chuyển)
image-bg-checker = nền ô cờ
image-bg-black = nền đen
image-bg-gray = nền xám
image-bg-white = nền trắng
image-inspect-aria = Xem kỹ { $name }
image-preview-size = bản xem trước { $width }×{ $height }, { $bytes }
image-preview = bản xem trước
image-resolution = Độ phân giải
image-full-title = Tải ảnh đầy đủ { $width }×{ $height } (F); phóng to quá bản xem trước cũng sẽ tải
image-full-loading = Đang tải độ phân giải đầy đủ…
image-full-failed = Không tải được độ phân giải đầy đủ: thử lại
image-full = Độ phân giải đầy đủ
image-zoom = Thu phóng
image-zoom-out-title = Thu nhỏ (−)
image-zoom-out = Thu nhỏ
image-zoom-in-title = Phóng to (+)
image-zoom-in = Phóng to
image-fit-title = Vừa khung (0)
image-fit = Vừa khung
image-actual-title = Kích thước thật (1)
image-view = Xem
image-tile-title = Lát 3×3 để kiểm tra đường nối (T)
image-tile = Lát
image-channels = Kênh
image-close = Đóng (Esc)
image-failed = Không thể tải ảnh này.
image-loading = Đang tải…
image-key-gestures = Cuộn hoặc chụm để thu phóng · kéo để di chuyển · nhấp đúp để phóng to hoặc vừa khung
image-key-tile = lát
image-key-background = nền
image-key-full = độ phân giải đầy đủ
image-key-channels = kênh

## Trình phát sprite

sprite-frame-aria = { $name }, khung hình { $frame } / { $count }
sprite-failed = Không thể phát tệp này.
sprite-loading = Đang tải khung hình…
sprite-previous = Khung hình trước
sprite-next = Khung hình tiếp theo
sprite-pause = Tạm dừng hoạt ảnh
sprite-play = Phát hoạt ảnh
sprite-animation = Hoạt ảnh
sprite-all-frames = Tất cả khung hình
sprite-grid-title = Cách cắt bảng thành các khung hình
sprite-columns = Cột
sprite-rows = Hàng
sprite-fps-aria = Khung hình mỗi giây
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
       *[other] { $count } sprite
    }
sheet-pages =
    { $count ->
       *[other] { $count } trang
    }
sheet-animations =
    { $count ->
       *[other] { $count } hoạt ảnh
    }
# The box that narrows the list of sprites by name.
sheet-find = Tìm sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Đang hiện { $shown } / { $total }
sheet-no-match = Không có sprite nào có tên chứa nội dung này.
sheet-failed = Không thể đọc bảng này.
sheet-loading = Đang tải bảng…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Định dạng riêng của Hoard: giữ nguyên các trang, sprite xoay theo mọi hướng và được cắt viền trong suốt, kèm hoạt ảnh và thời gian của từng khung hình.
sheet-note-texturepacker = Sprite xoay theo chiều kim đồng hồ và được cắt viền trong suốt, kèm hoạt ảnh theo tên, đúng như cách PixiJS và Phaser đọc.
sheet-note-clockwise = Sprite xoay theo chiều kim đồng hồ và được cắt viền trong suốt. Định dạng này không chứa được hoạt ảnh: khung hình được nhận theo tên có đánh số.
sheet-note-libgdx = Sprite xoay ngược chiều kim đồng hồ và được cắt viền trong suốt. Định dạng này không chứa được hoạt ảnh: khung hình được nhận theo tên có đánh số.
sheet-note-godot-frames = Sprite không bao giờ bị xoay, phần viền đã cắt được giữ lại làm lề, kèm hoạt ảnh và thời gian của từng khung hình.
sheet-note-godot-tileset = Ô không bao giờ bị xoay hay cắt viền, xếp trên lưới theo kích thước ô, và giữ nguyên ảnh nếu chúng đã nằm sẵn trên lưới.
sheet-note-grid = Mỗi sprite đứng thẳng trên toàn bộ khung vẽ của nó, trong các ô cùng kích thước, dùng cho bất kỳ công cụ nào cắt bảng thành lưới.
sheet-note-frames = Mỗi sprite đứng thẳng trên toàn bộ khung vẽ của nó, mỗi sprite là một tệp PNG mang tên của chính nó.

## Trình xem văn bản

text-lang-config = Cấu hình
text-lang-table = Bảng
text-lang-code = Mã
text-lang-script = Script
text-lang-text = Văn bản
text-copy-blocked = Không thể sao chép: trình duyệt đã chặn
text-read-aria = Đọc { $name }
text-lines = { $count } dòng
text-show-as = Hiển thị dạng
text-tree-title = Cây thu gọn được (T)
text-tree = Cây
text-pretty-title = Thụt lề lại (P)
text-pretty = Đẹp
text-raw-title = Như nguyên bản (R)
text-raw = Thô
text-document-title = Dạng tài liệu (D)
text-document = Tài liệu
text-source-title = Markdown như nguyên bản (R)
text-source = Nguồn
text-page-title = Như trình duyệt hiển thị từ thư mục của nó, tách biệt khỏi Hoard (P)
text-page = Trang
text-page-source-title = Trang như được viết (R)
text-page-aria = { $name } dưới dạng trang
text-page-error = Không mở được trang này: { $error }
text-fold = Thu gọn
text-expand-all-title = Mở rộng mọi thứ
text-expand-all = Mở rộng tất cả
text-collapse-all-title = Thu gọn mọi thứ
text-collapse-all = Thu gọn tất cả
text-group-text = Văn bản
text-wrap-title = Ngắt dòng dài (W)
text-wrap = Ngắt dòng
text-copy-title = Sao chép văn bản (C)
text-copied = Đã sao chép
text-copy = Sao chép
text-download-title = Tải tệp xuống
text-download = Tải xuống
text-close = Đóng (Esc)
text-truncated = Đang hiện { $shown } đầu tiên trong { $total }.
text-truncated-json = Đang hiện { $shown } đầu tiên trong { $total } (cây cần toàn bộ tệp).
text-show-all = Hiện tất cả
text-json-error = JSON này không phân tích được: { $error }
text-contents-aria = Nội dung { $name }
text-loading = Đang tải…
text-load-error = Không thể tải tệp này: { $error }
text-binary = Tệp này là tệp nhị phân, không phải văn bản, nên không có gì để đọc ở đây. Hãy tải xuống để mở bằng công cụ của nó.
text-key-tree = cây
text-key-pretty = đẹp
text-key-raw = thô
text-key-document = tài liệu
text-key-source = nguồn
text-key-page = trang
text-key-wrap = ngắt dòng
text-key-copy = sao chép
text-key-find = tìm
text-key-close = đóng
text-preview-aria = Đọc { $name } toàn màn hình
text-preview-read = Đọc

## Cây JSON

json-items = { $count } mục
json-keys = { $count } khóa
json-fold = Thu gọn
json-unfold = Mở rộng
json-unfold-summary = Mở rộng { $summary }
json-show-more = Hiện thêm { $count }
json-left = trong số { $count } còn lại
json-picker-more = … thêm { $count }

## Trình xem mô hình 3D

model-viewer-failed = trình xem 3D không tải được ({ $error }); tải lại trang có thể giúp ích
model-loading-viewer = Đang tải trình xem 3D…
model-loading = Đang tải mô hình…
model-show-failed = Không thể hiển thị mô hình này: { $error }
model-preview-failed = Không thể xem trước mô hình này: { $error }
model-error-converting = máy chủ vẫn đang chuyển đổi mô hình này
model-error-converting-desktop = Hoard vẫn đang chuyển đổi mô hình này
model-error-empty-collada = tệp Collada trống
model-error-no-loader = Trình duyệt không có trình tải cho .{ $ext }
model-dialog = Xem { $name } ở dạng 3D
model-title-stats = { $triangles } tam giác · { $meshes } mesh · { $materials } vật liệu
model-triangles = { $count } tam giác
model-group-camera = Camera
model-group-shading = Đổ bóng
model-group-background = Nền
model-group-animation = Hoạt ảnh
model-group-view = Góc nhìn
model-group-show = Hiển thị
model-frame-title = Căn khung mô hình (F)
model-frame = Căn khung
model-reset-title = Về góc nhìn ban đầu (R)
model-reset = Đặt lại
model-turn-title = Xoay chậm (A)
model-turn = Xoay
model-textures-title = Texture (T)
model-textures = Texture
model-full-res-title = Texture độ phân giải đầy đủ (H): từng tệp texture nguyên trạng, dù lớn đến đâu; khi tắt, dùng bản xem trước nhẹ tối đa 2048 px
model-full-res = Đầy đủ
model-light-title = Ánh sáng môi trường (E)
model-light = Ánh sáng
model-grid-title = Lưới sàn (G)
model-grid = Lưới
model-bg-checker = Nền ô cờ
model-bg-dark = Nền tối
model-bg-gray = Nền xám
model-bg-light = Nền sáng
model-bg-env = Nền môi trường
model-bg-title = { $background } (K để chuyển)
model-pause-title = Tạm dừng (Space)
model-play-title = Phát (Space)
model-pause = Tạm dừng
model-play = Phát
model-animation-clip = Đoạn hoạt ảnh
model-animation = Hoạt ảnh
model-plays-on = Phát trên
model-rest-pose = Tư thế nghỉ
model-close-title = Đóng (Esc)
model-full-screen = Toàn màn hình
model-open-full-screen = Mở trình xem 3D toàn màn hình
model-menu-fold = Thu gọn menu
model-menu-show = Hiện menu
model-view-prefix = Chế độ xem:{ " " }
model-menu-render = Kết xuất
model-menu-maps = Map
model-menu-inspect = Kiểm tra
model-mode-final = Kết xuất hoàn chỉnh
model-mode-raw = Kết xuất thô
model-mode-color = Màu cơ bản
model-mode-roughness = Độ nhám
model-mode-metalness = Độ kim loại
model-mode-normal = Normal
model-mode-ao = Occlusion
model-mode-matcap = Matcap
model-mode-uv = Kiểm tra UV
model-mode-geometry = Hình học
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Khung dây (W), chồng lên mọi chế độ xem
model-wireframe = Khung dây
model-info-title = Thông tin cảnh (I)
model-info = Thông tin cảnh
model-info-close = Đóng thông tin cảnh
model-info-close-title = Đóng (I)
model-info-geometries = Hình học
model-info-materials = Vật liệu
model-info-textures = Texture
model-info-triangles = Tam giác
model-info-vertices = Đỉnh
model-info-meters = glTF tính bằng mét
model-info-no-units = Định dạng này không cho biết đơn vị của nó
model-info-height = Chiều cao
model-info-length = Chiều dài
model-info-width = Chiều rộng
model-length-units = { NUMBER($n, maximumSignificantDigits: 3) } đơn vị
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Không tìm thấy texture: { $name }
model-missing-some = Không tìm thấy { $count } texture: { $names }
model-missing-more = Không tìm thấy { $count } texture: { $names }…
model-missing-count = Không tìm thấy { $count } texture
model-guessed = Texture được ghép theo tên; tệp không nêu tên texture nào.
model-group-facing = Hướng
model-facing-turn = Xoay 90°
model-facing-turn-title = Xoay mô hình 90° ngược chiều kim đồng hồ, nhìn từ trên xuống
model-facing-tip = Nghiêng 90°
model-facing-tip-title = Nghiêng phần đỉnh mô hình 90° về phía trước, để dựng đứng mô hình đang nằm
model-facing-save = Dùng cho gói này
model-facing-save-title = Xoay mọi mô hình trong { $pack } theo cách này. Ảnh thu nhỏ của chúng được vẽ lại; các bản tải xuống vẫn giữ như lúc được tạo.
model-facing-undo = Hoàn tác
model-facing-undo-title = Quay về cách các mô hình của gói đang được xoay
model-facing-saved = Mọi mô hình trong { $pack } giờ đã quay theo hướng này. Ảnh thu nhỏ của chúng đang được vẽ lại.
model-facing-failed = Không thể xoay các mô hình của gói: { $error }
model-keys-mouse = Kéo để xoay quanh · kéo chuột phải để di chuyển · cuộn để thu phóng · nhấp đúp để căn khung
model-key-frame = căn khung
model-key-reset = đặt lại
model-key-view = chế độ xem
model-key-wireframe = khung dây
model-key-info = thông tin cảnh
model-key-textures = texture
model-key-full-res = texture đầy đủ
model-key-light = ánh sáng
model-key-grid = lưới
model-key-background = nền
model-key-play = phát

## Vật liệu

material-role-base-color = Màu
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Độ nhám
material-role-gloss = Độ bóng
material-role-metalness = Độ kim loại
material-role-ao = Ambient occlusion
material-role-orm = Occlusion, độ nhám, kim loại
material-role-mra = Kim loại, độ nhám, occlusion
material-role-rma = Độ nhám, kim loại, occlusion
material-role-height = Độ cao
material-role-bump = Bump
material-role-cavity = Cavity
material-role-specular = Specular
material-role-opacity = Độ mờ đục
material-role-emissive = Phát sáng
material-role-translucency = Độ trong mờ
material-role-base-color-name = màu
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = độ nhám
material-role-gloss-name = độ bóng
material-role-metalness-name = độ kim loại
material-role-ao-name = ambient occlusion
material-role-orm-name = occlusion, độ nhám, kim loại
material-role-mra-name = kim loại, độ nhám, occlusion
material-role-rma-name = độ nhám, kim loại, occlusion
material-role-height-name = độ cao
material-role-bump-name = bump
material-role-cavity-name = cavity
material-role-specular-name = specular
material-role-opacity-name = độ mờ đục
material-role-emissive-name = phát sáng
material-role-translucency-name = độ trong mờ
material-maps = { $count } map
material-dialog = Xem vật liệu { $name }
material-up-to = tối đa { $size }
material-repeat-size = { $size } mỗi lần lặp
material-shape-sphere = Hình cầu
material-shape-cube = Hình lập phương
material-shape-plane = Mặt phẳng
material-shape-title = { $shape } ({ $key })
material-group-shape = Hình dạng
material-group-tiling = Lát
material-real-title = Kích thước thật: một lần lặp là { $size }, hình cầu rộng 2 m (T để chuyển)
material-repeat-title = Lặp { $n }× (T để chuyển)
material-real = Thật
material-repeat = { $n }×
material-height-title = Mức map độ cao đẩy bề mặt lên
material-height = Độ cao
material-height-strength = Cường độ độ cao
material-loading = Đang tải vật liệu…
material-show-failed = Không thể hiển thị vật liệu này: { $error }
material-preview-failed = Không thể xem trước vật liệu này: { $error }
material-group-maps = Map
material-map-show = Hiện map { $map }
material-map-hide = Ẩn map { $map }
material-map-unused = Bản xem trước không dùng map này
material-open-full-screen = Mở trình xem vật liệu toàn màn hình
material-keys-mouse = Kéo để xoay quanh · kéo chuột phải để di chuyển · cuộn để thu phóng · nhấp vào một map để tắt nó
material-key-shape = hình dạng
material-key-tiling = lát
material-key-turn = xoay

## Chủ đề

theme-slate = Đá phiến
theme-paper = Giấy
theme-linen = Vải lanh
theme-sage = Xô thơm
theme-lilac = Tử đinh hương
theme-blush = Hồng phấn
theme-graphite = Than chì
theme-obsidian = Đá vỏ chai
theme-ember = Than hồng
theme-moss = Rêu
theme-aubergine = Cà tím
theme-oxblood = Đỏ huyết
theme-light-slate-blurb = Xám xanh dịu mát
theme-light-paper-blurb = Xám trung tính đơn giản
theme-light-linen-blurb = Kem ấm, hổ phách
theme-light-sage-blurb = Xanh lá dịu
theme-light-lilac-blurb = Tím nhạt
theme-light-blush-blurb = Hồng phấn trầm
theme-dark-graphite-blurb = Xám trung tính, không ánh màu
theme-dark-slate-blurb = Xám xanh
theme-dark-obsidian-blurb = Gần như đen, cho màn hình OLED
theme-dark-ember-blurb = Than ấm, hổ phách
theme-dark-moss-blurb = Đen ánh xanh rừng
theme-dark-aubergine-blurb = Đen ánh mận, tím nhạt
theme-dark-oxblood-blurb = Đen ánh rượu vang, san hô

## Tên texture

texnames-needs-map = Mẫu cần có { "{" }map{ "}" }, nếu không mọi map của một vật liệu sẽ có cùng tên
texnames-placeholders = Mẫu có thể dùng { "{" }name{ "}" }, { "{" }map{ "}" } và { "{" }size{ "}" }
texnames-bad-characters = Tên không được chứa < > : " / \ | ? hoặc *
texnames-no-suffix = Hãy đặt hậu tố cho map { $role }
texnames-bad-suffix = Hậu tố cho map { $role } không được chứa < > : " / \ | ? hoặc *
texnames-slot-no-suffix = Hãy đặt hậu tố cho “{ $slot }”
texnames-slot-bad-suffix = Hậu tố cho “{ $slot }” không được chứa < > : " / \ | ? hoặc *
texnames-ending-missing = Mỗi đuôi tên riêng của bạn cần có đuôi tên cần tìm
texnames-ending-bad = Đuôi tên riêng không được chứa < > : " / \ | ? hoặc *
texnames-ending-twice = { $ending } xuất hiện hai lần trong các đuôi tên của bạn
texnames-slot-metallic-roughness = Kim loại và độ nhám
texnames-slot-specular-color = Màu specular
texnames-slot-specular-glossiness = Specular và độ bóng
texnames-slot-clearcoat = Clearcoat
texnames-slot-clearcoat-roughness = Độ nhám clearcoat
texnames-slot-clearcoat-normal = Normal clearcoat
texnames-slot-sheen-color = Màu sheen
texnames-slot-sheen-roughness = Độ nhám sheen
texnames-slot-transmission = Truyền sáng
texnames-slot-thickness = Độ dày
texnames-slot-diffuse-transmission = Truyền sáng khuếch tán
texnames-slot-diffuse-transmission-color = Màu truyền sáng khuếch tán
texnames-slot-iridescence = Ánh ngũ sắc
texnames-slot-iridescence-thickness = Độ dày ánh ngũ sắc
texnames-slot-anisotropy = Dị hướng

## Định dạng cửa hàng

storefiles-field-title = Tiêu đề
storefiles-field-title-hint = Tiêu đề chính xác của sản phẩm. Nó dùng để đặt tên gói, và nếu thiếu thì tệp sẽ bị bỏ qua.
storefiles-field-seller = Người bán
storefiles-field-seller-hint = Ai đã tạo hoặc bán nó.
storefiles-field-license = Giấy phép
storefiles-field-license-hint = Như tệp ghi, ví dụ CC-BY-4.0.
storefiles-field-url = Trang cửa hàng
storefiles-field-url-hint = Địa chỉ của sản phẩm, bắt đầu bằng http:// hoặc https://.
storefiles-field-words = Thẻ
storefiles-field-words-hint = Các từ để gắn thẻ. Giá trị phân tách bằng dấu phẩy sẽ được tách ra.
storefiles-field-type = Loại
storefiles-field-type-hint = Nó là gì, ví dụ “3D model” hoặc “surface”. Cũng là một thẻ.
storefiles-field-colors = Màu
storefiles-field-colors-hint = Các màu của nó, dưới dạng thẻ color:.
storefiles-field-id = Id cửa hàng
storefiles-field-id-hint = Id mà cửa hàng đặt cho nó: chữ cái, chữ số, _ hoặc -.
storefiles-field-origin = Nơi tạo
storefiles-field-origin-hint = Nơi nó được tạo hoặc quét, cho phần mô tả.
storefiles-field-size = Kích thước
storefiles-field-size-hint = Nó lớn cỡ nào, bằng lời, cho phần mô tả.
storefiles-field-title-name = tiêu đề
storefiles-field-seller-name = người bán
storefiles-field-license-name = giấy phép
storefiles-field-url-name = trang cửa hàng
storefiles-field-words-name = thẻ
storefiles-field-type-name = loại
storefiles-field-colors-name = màu
storefiles-field-id-name = id cửa hàng
storefiles-field-origin-name = nơi tạo
storefiles-field-size-name = kích thước

## Bảng định dạng cửa hàng

storefiles-load-failed = Không thể tải các định dạng tệp cửa hàng: { $error }
storefiles-remove-title = Gỡ { $name }?
storefiles-remove-body = { $file } sẽ bị xóa khỏi thư mục dữ liệu. Các gói đã được đọc bằng nó vẫn giữ những gì đã đọc.
storefiles-remove = Gỡ
storefiles-removed = Đã gỡ { $name }.
storefiles-remove-failed = Không thể gỡ { $name }: { $error }
storefiles-unread = Tệp Hoard không đọc
storefiles-loading = Đang tải…
storefiles-unread-none = Không có: không tệp nào trong thư viện trông giống tệp của một cửa hàng chưa được đọc.
storefiles-unread-what = Các tệp có trong nhiều gói, hoặc được đặt tên giống tệp của cửa hàng, mà không định dạng nào đọc.
storefiles-unread-packs = { $count } gói
storefiles-unread-example = ví dụ { $pack }
storefiles-teach = Dạy Hoard đọc tệp này
storefiles-formats = Định dạng
storefiles-formats-what = Định dạng của bạn được thử trước và thay thế định dạng có sẵn cùng tên.
storefiles-dir-before = Chúng được lưu trong
storefiles-dir-after = .
storefiles-yours = Của bạn
storefiles-built-in = Có sẵn
storefiles-edit = Sửa
storefiles-start-from = Bắt đầu từ định dạng này

## Bảng thẻ

tags-load-failed = Không thể tải các thẻ: { $error }
tags-rename-title = Đổi tên { $tag }?
tags-merge-title = Gộp { $count } thẻ?
tags-rename-body = { $count } tài nguyên của thẻ này sẽ nhận { $into } thay thế, và các bộ gắn thẻ sẽ dùng tên mới từ giờ trở đi.
tags-merge-body = { $tags } sẽ thành { $into } trên tối đa { $count } tài nguyên. Các bộ gắn thẻ sẽ dùng { $into } từ giờ trở đi.
tags-rename = Đổi tên
tags-merge = Gộp
tags-merged = { $count } tài nguyên giờ có thẻ { $into }
tags-merge-failed = Không thể gộp: { $error }
tags-forget-failed = Không thể thay đổi: { $error }
tags-find = Tìm thẻ
tags-find-placeholder = Tìm thẻ, ví dụ rock hoặc set:
tags-count = { $count } thẻ
tags-count-more = { $count }+ thẻ
tags-what = Chọn các thẻ cùng nghĩa để gộp, hoặc chọn một thẻ để đổi tên.
tags-new-name = Tên mới
tags-merge-into = Gộp vào
tags-saved-as-before = Đã lưu thành
tags-saved-as-after = { "" }
tags-cancel = Hủy
tags-loading = Đang tải…
tags-none = Không có thẻ nào.
tags-none-containing = Không có thẻ nào chứa “{ $text }”.
tags-browse-title = Duyệt tài nguyên có thẻ { $tag }
tags-browse = Duyệt
tags-merged-heading = Thẻ đã gộp
tags-merged-what = Khi các bộ gắn thẻ đưa ra một trong những tên cũ này, tài nguyên sẽ nhận thẻ mà tên đó đã được gộp vào.
tags-allow-old = Cho phép lại tên cũ
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count = { $count } tài nguyên
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
       *[other] Tên cũ: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } đã được gộp vào { $into }.
tag-page-show = Hiện { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Cho biết thẻ này có nghĩa là gì…
tag-page-describe-label = Thẻ này có nghĩa là gì
tag-page-describe-placeholder = Dùng để làm gì, những gì thuộc về thẻ. Nhấn Enter để lưu.
tag-page-describe-edit = Nhấp để thay đổi
tag-page-description-saved = Đã lưu ý nghĩa của thẻ
# The tag's picture: one of its assets stands for it.
tag-page-picture = Chọn ảnh cho thẻ này
tag-page-picture-help = Chọn một tài nguyên của thẻ để làm ảnh đại diện
tag-page-picture-nothing = Không tài nguyên nào đang hiện có ảnh.
tag-page-picture-none = Không có ảnh
tag-page-picture-saved = Đã đặt ảnh
tag-page-picture-removed = Đã bỏ ảnh
tag-page-rename = Đổi tên…
tag-page-rename-title = Đặt tên mới, hoặc tên của một thẻ khác để gộp vào thẻ đó
tag-page-rename-label = Tên mới
tag-page-save-failed = Không lưu được thẻ: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Dưới
tag-page-parent-add = Đặt dưới…
tag-page-parent-add-title = Đặt thẻ này dưới một thẻ rộng hơn: tìm thẻ rộng hơn sẽ thấy cả các thẻ này
tag-page-parent-placeholder = Một thẻ rộng hơn, ví dụ phương tiện
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Gỡ { $tag } khỏi dưới { $parent }
tag-page-parent-saved = { $tag } giờ nằm dưới { $parent }
tag-page-parent-removed = { $tag } không còn nằm dưới { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Hẹp hơn:
tag-page-narrower-title = Tìm { $tag } cũng sẽ thấy các thẻ này
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Rộng hơn và hẹp hơn
tags-tree-what = Các thẻ được đặt dưới thẻ rộng hơn từ trang thẻ của chúng. Tìm một thẻ cũng sẽ thấy mọi thứ nằm dưới nó.

## Thống kê thư viện

stats-source-human = Do người thêm
stats-source-ai = Do AI gợi ý
stats-source-rule = Quy tắc có sẵn
stats-source-meta = Từ tệp
stats-assets = Tài nguyên
stats-packs = Gói
stats-on-disk = Trên ổ đĩa
stats-duplicates-title = { $count } tệp là bản sao giống hệt từng byte của tệp khác và chỉ được lưu một lần
stats-saved-dedup = Tiết kiệm nhờ khử trùng lặp
stats-distinct-tags = Số thẻ khác nhau
stats-archive = Trong kho lưu trữ
stats-load-failed = Không thể tải chi tiết: { $error }
stats-counting = Đang đếm…
stats-as-of = Tính đến { $time }
stats-updating = Đang cập nhật…
stats-by-kind = Dung lượng theo loại
stats-files-size = { $name }: { $count } tệp, { $size }
stats-months = Đã thêm trong năm qua
stats-month = { $month }: { $count } tài nguyên, { $size }
stats-tagging = Gắn thẻ
stats-tagged = Đã gắn thẻ
stats-untagged = { $count } chưa có thẻ
stats-described = Đã mô tả
stats-described-assets = { $count } tài nguyên
stats-human-edited = Sửa thủ công
stats-human-edited-unit =
    { $count ->
       *[other] tài nguyên
    }
stats-source-different = { $count } thẻ khác nhau
stats-most-used = Dùng nhiều nhất
stats-show-tag = Hiện tài nguyên có thẻ { $tag }
stats-file-types = Loại tệp
stats-biggest-packs = Gói lớn nhất
stats-browse-pack = Duyệt { $name }
stats-largest-files = Tệp lớn nhất
stats-open-file = Mở { $name }
stats-empty = Thư viện chưa có gì. Hãy thêm một gói và số liệu của nó sẽ hiện ở đây.

## HoardPanel.vue

hoard-title = Kho báu của bạn
hoard-what = Mỗi tài nguyên trong thư viện là một món báu vật, và loại của nó quyết định là món nào. Thu thập những gì mới để thêm vào đống của bạn.
hoard-sign-in = Đăng nhập để thu thập kho báu của riêng bạn.
hoard-new = Chiến lợi phẩm mới để thu thập: { $count }
hoard-none-new = Không có gì mới để thu thập.
hoard-collect = Thu thập
hoard-unavailable = Trình duyệt này không vẽ được kho báu: { $error }
hoard-collect-failed = Không thể ghi lại những gì bạn đã thu thập: { $error }
hoard-arcade = Màn hình máy arcade
hoard-full-screen = Toàn màn hình
hoard-leave-full-screen = Thoát toàn màn hình
hoard-label = Một con rồng canh giữ kho báu của nó. Báu vật: { $count }
hoard-gold = Vàng
hoard-gems = Đá quý
hoard-books = Sách
hoard-weapons = Vũ khí
hoard-bones = Xương
hoard-tablets = Phiến đá rune
hoard-paintings = Tranh
hoard-potions = Bình thuốc

## Cập nhật

updates-saved = Hoard sẽ gọi địa chỉ này khi có bản cập nhật
updates-saved-none = Không có địa chỉ để gọi
updates-save-failed = Không thể lưu
updates-call-answered = Địa chỉ trả lời { $result }
updates-call-refused = Địa chỉ không nhận yêu cầu: { $result }
updates-call-failed = Không thể gọi
updates-called = Đã gọi lúc { $when } cho { $version }: { $result }
updates-called-failed = Đã gọi lúc { $when } cho { $version }: { $result }. Hoard sẽ thử lại sau một giờ.
updates-latest = Hoard { $version } là bản phát hành mới nhất.
updates-out = Đã có Hoard { $version }.
updates-out-idle = Đã có Hoard { $version }; sẽ gọi trong vài phút tới, khi không có việc gì đang chạy.
updates-out-busy = Đã có Hoard { $version }; đang chờ đến khi không còn việc gì chạy ({ $work }).
updates-busy-more = { $count } việc khác
updates-busy-jobs = { $count } tác vụ đang chờ
updates-busy-uploads = { $count } lượt tải lên
updates-list-separator = ,{ " " }
updates-what = Khi có bản phát hành mới, Hoard có thể gọi một địa chỉ khi không còn việc gì đang chạy (không có tải lên, tác vụ, chuyển đổi hay bảo trì), để công cụ cập nhật container không khởi động lại nó giữa lúc đang tải lên. Hoard gửi một yêu cầu POST với thông tin bản phát hành trong phần thân JSON. Việc này cần bật kiểm tra bản phát hành ở trên.
updates-url = Địa chỉ để gọi
updates-url-placeholder = Không có (ví dụ http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Đã lưu (nhập để thay thế)
updates-auth-placeholder = Không có (ví dụ Bearer my-token)
updates-remove = Gỡ
updates-save = Lưu
updates-call-now-title = Bất cứ thứ gì được nối với địa chỉ này sẽ chạy
updates-call-now = Gọi ngay
updates-what-calls = Có thể gọi những gì?
updates-watchtower = HTTP API của Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , với{ " " }
updates-watchtower-token = { " " }và token từ{ " " }
updates-watchtower-its = { " " }(đã bật endpoint{ " " }
updates-watchtower-endpoint = { "" }).
updates-portainer = Một webhook Portainer
updates-portainer-rest = { " " }trên stack hoặc container của Hoard, với tùy chọn re-pull image được bật. Token nằm trong URL.
updates-script = Một script trên máy chủ
updates-script-listener = { " " }đứng sau một trình nghe webhook (chẳng hạn{ " " }
updates-script-runs = ) chạy{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }hoặc bất cứ thứ gì nhận webhook.
updates-poll-before = Hoặc bỏ qua địa chỉ: một script có token truy cập có thể hỏi định kỳ{ " " }
updates-poll-until = { " " }cho đến khi{ " " }
updates-poll-and = { " " }được đặt và{ " " }
updates-poll-after = { " " }là true, rồi tải bản mới và khởi động lại Hoard.

## Bảo trì

maintenance-load-failed = Không thể tải Bảo trì
maintenance-reanalyze-title = Phân tích lại mọi gói?
maintenance-reanalyze-body = Mọi tệp sẽ được đọc lại, có thể mất rất lâu với thư viện lớn; các gói hiện là đang xử lý cho đến lượt của chúng. Tên, thẻ và ghi chú của bạn vẫn giữ nguyên.
maintenance-reanalyze-confirm = Phân tích lại
maintenance-gc-title = Xóa tệp không dùng?
maintenance-gc-body = Các tệp đã lưu mà không có gì trong thư viện trỏ tới sẽ bị xóa vĩnh viễn. Gói trong kho lưu trữ vẫn giữ tệp của chúng.
maintenance-gc-confirm = Xóa các tệp đó
maintenance-start-failed = Không thể bắt đầu “{ $task }”
maintenance-just-now = vừa xong
maintenance-minutes-ago = { $count } phút trước
maintenance-hours-ago = { $count } giờ trước
maintenance-took-seconds = { $count } giây
maintenance-took-minutes = { $count } phút
maintenance-took-hours = { $hours } giờ
maintenance-loading = Đang tải…
maintenance-running = Đang chạy…
maintenance-starting = Đang bắt đầu…
maintenance-run = Chạy
maintenance-started = { $detail } · bắt đầu { $when }
maintenance-starting-detail = Đang bắt đầu
maintenance-finished = Hoàn tất:{ " " }
maintenance-failed = Thất bại:{ " " }
maintenance-never-run = Chưa chạy lần nào
maintenance-history = Các lần chạy trước
maintenance-col-task = Tác vụ
maintenance-col-finished = Hoàn tất
maintenance-col-took = Thời gian
maintenance-col-result = Kết quả
maintenance-script-before = Sao lưu và kiểm tra hư hỏng cũng có thể chạy từ script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }và{ " " }
maintenance-script-after = . Những lần chạy đó cũng hiện ở đây.

## Trang cài đặt

settings-title = Cài đặt
settings-loading = Đang tải…
settings-find = Tìm cài đặt
settings-find-none = Không có cài đặt nào khớp với “{ $query }”.
settings-load-failed = Không thể tải cài đặt: { $error }
settings-page-appearance = Giao diện
settings-page-appearance-blurb = Chủ đề trình duyệt này dùng khi ở chế độ sáng và khi ở chế độ tối. Chỉ lưu trong trình duyệt này.
settings-page-dragon = Rồng
settings-page-dragon-blurb = Kho báu của rồng: mỗi tài nguyên trong thư viện là một món trong đó.
settings-page-library = Thư viện
settings-page-library-blurb = Những gì có trong kho: lớn cỡ nào, gồm những gì, đã tăng lên ra sao và được gắn thẻ tốt đến đâu. Những thứ đã lưu trữ không được tính.
settings-page-copies = Bản gần giống
settings-page-copies-blurb = Các tệp là cùng một hình, âm thanh hoặc mô hình ở kích thước, tần số lấy mẫu hoặc định dạng khác, tệp chiếm nhiều dung lượng nhất trước. Bản tốt nhất của mỗi bộ được đánh dấu; không có gì bị xóa.
copies-loading = Đang tìm bản gần giống…
copies-refreshing = Thư viện đã thay đổi kể từ khi tìm thấy các bản này. Đang tìm lại…
copies-none = Không có bản gần giống: mỗi hình, âm thanh và mô hình chỉ có ở đây một lần.
copies-load-failed = Không thể liệt kê bản gần giống: { $error }
copies-sum = Bộ bản gần giống: { $count }. Các bản không phải tốt nhất chiếm { $size }.
copies-set = Bộ { $n }
copies-set-wasted = { $size } trong các bản không phải tốt nhất
copies-more = Hiện thêm (còn { $count })
settings-page-tags = Thẻ
settings-page-tags-blurb = Mọi thẻ trong thư viện. Gộp các thẻ cùng nghĩa, hoặc đổi tên một thẻ; các bộ gắn thẻ sẽ dùng tên mới từ đó trở đi.
settings-page-store-files = Tệp cửa hàng
settings-page-store-files-blurb = Các tệp mà cửa hàng đặt kèm tác phẩm, Hoard đọc chúng để đặt tên gói và ghi công người tạo. Hãy dạy Hoard những tệp nó chưa biết, và chia sẻ những gì bạn đã dạy.
settings-page-locations = Vị trí thư viện
settings-page-locations-blurb = Các thư mục trên máy chủ chứa thư viện: nơi Hoard lưu tệp, và các thư mục chứa gói được lập chỉ mục tại chỗ.
settings-page-locations-blurb-desktop = Các thư mục chứa thư viện: nơi Hoard lưu tệp, và các thư mục chứa gói được lập chỉ mục tại chỗ.
settings-page-tools = Công cụ
settings-page-tools-blurb = Các chương trình bên ngoài mà Hoard dùng để chuyển đổi và đọc tệp.
settings-page-ai = Gắn thẻ AI
settings-page-ai-blurb = Một mô hình AI đặt tên, mô tả và gắn thẻ cho gói mới, bổ sung cho các quy tắc có sẵn. Thẻ của nó được đánh dấu là gợi ý.
settings-page-search-blurb = Tìm ảnh theo nội dung trong ảnh và tìm âm thanh theo cách chúng nghe ra sao, không chỉ theo tên, và tìm những thứ giống nhau về loại, không chỉ về màu.
settings-page-search-tab = Tìm kiếm
settings-page-adding = Thêm gói
settings-page-adding-blurb = Những gì Hoard làm với một gói khi nó được thêm vào, ngoài việc đọc và gắn thẻ.
settings-page-downloads = Tải xuống
settings-page-downloads-blurb = Cách đặt tên tệp trong những gì Hoard gửi đi: tải xuống, tệp zip, công cụ hoard và trợ lý AI. Tệp trong thư viện vẫn giữ nguyên.
settings-page-account = Tài khoản
settings-page-account-blurb = Mật khẩu của bạn, những người khác có thể đăng nhập, và token truy cập cho script và các chương trình khác.
settings-page-auth = Xác thực
settings-page-auth-blurb = Cách mọi người đăng nhập: tài khoản riêng của Hoard, thư mục LDAP, Microsoft Active Directory hoặc nhà cung cấp OAuth, phiên đăng nhập kéo dài bao lâu, và nhóm nào nhận vai trò nào.
settings-page-connection = Kết nối
settings-page-connection-blurb = Cách trình duyệt và ứng dụng kết nối tới Hoard: địa chỉ mà nó được biết đến, và HTTPS, thứ mà một số tính năng trình duyệt cần.
settings-page-performance = Hiệu năng
settings-page-performance-blurb = Máy chủ làm việc nặng đến đâu, ảnh thu nhỏ được tạo và lưu thế nào, và mức tối đa nó nhận vào.
settings-page-performance-blurb-desktop = Hoard làm việc nặng đến đâu, ảnh thu nhỏ được tạo và lưu thế nào, và mức tối đa nó nhận vào.
settings-page-issues = Sự cố
settings-page-issues-blurb = Những gì đã trục trặc khi mở hoặc xử lý tệp: tệp nén không giải nén được, tệp không đọc được, tác vụ thất bại. Hãy thử lại khi đã khắc phục nguyên nhân, hoặc bỏ qua chúng.
settings-page-maintenance = Bảo trì
settings-page-maintenance-blurb = Sao lưu, kiểm tra hư hỏng và dọn dẹp. Mỗi loại chỉ chạy một lần tại một thời điểm, và bạn có thể rời trang này trong khi chúng chạy.
settings-page-about = Giới thiệu
settings-page-about-blurb = Hoard là gì, đây là phiên bản nào, và công sức của những người khác đã góp phần tạo nên nó.
settings-the-default = mặc định
settings-default-on = bật
settings-default-off = tắt
settings-default-auto = tự động
settings-option-saved = { $name }: { $value }
settings-back-to-default = Về mặc định
settings-save = Lưu
settings-use = Dùng
settings-status-off = Tắt

## Cài đặt: Giao diện

settings-themes = Chủ đề
settings-look-light = Sáng
settings-look-dark = Tối
settings-look-group =
    { $scheme ->
        [light] Chủ đề sáng
       *[dark] Chủ đề tối
    }
settings-preview =
    Đang hiện chủ đề { $preview ->
        [light] sáng
       *[dark] tối
    } để bạn xem thử. Bạn đang dùng chế độ { $used ->
        [light] sáng
       *[dark] tối
    }{ $system ->
        [yes] { " " }(theo hệ thống)
       *[other] { "" }
    }, và nó sẽ trở lại khi bạn rời trang này.
settings-preview-back =
    Quay lại chế độ { $used ->
        [light] sáng
       *[dark] tối
    }
settings-look-hint = Chuyển giữa sáng và tối (hoặc theo hệ thống) từ thanh trên cùng.

## Cài đặt: Công cụ

settings-blender-what = Chuyển đổi mô hình .blend, .dae, .abc và .x3d sang glTF trong nền, để chúng có ảnh thu nhỏ và bản xem trước 3D. Các gói được thêm khi công cụ đang tắt sẽ được chuyển đổi khi nó sẵn sàng.
settings-blender-download = Dùng Blender đã cài, hoặc tải Blender 4.5 LTS (khoảng 400 MB) từ một máy chủ phản chiếu của blender.org.
settings-ffmpeg-what = Mã hóa bản tải xuống MP3, OGG và Opus, và đọc âm thanh mà Hoard không tự giải mã được (AAC, M4A, WMA, …).
settings-ffmpeg-download = Dùng ffmpeg của hệ thống có đủ bộ mã hóa, hoặc tải một bản build tĩnh (khoảng 80 MB).
settings-tool-reset = { $tool }: về mặc định
settings-tool-no-download = Dùng { $tool } đã cài. Không có bản build nào để tải cho máy này, nên hãy cài nó hoặc đưa đường dẫn của nó.
settings-tool-path = Đường dẫn chương trình { $tool }
settings-mode-off = Tắt
settings-mode-auto = Tự động
settings-mode-path = Chương trình này
settings-tool-looking = Đang tìm…
settings-tool-downloading = Đang tải xuống… tiến độ hiện ở chỉ báo hoạt động
settings-tool-ready = Sẵn sàng: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Các nhạc cụ dùng để phát và chuyển đổi tệp MIDI.
settings-soundfont-file = Tệp SoundFont
settings-soundfont-placeholder = Tìm thấy trên máy chủ, hoặc có sẵn
settings-soundfont-placeholder-desktop = Tìm thấy trên máy tính này, hoặc có sẵn
settings-soundfont-default = tìm thấy trên máy chủ, hoặc có sẵn
settings-soundfont-default-desktop = tìm thấy trên máy tính này, hoặc có sẵn
settings-soundfont-send = Chọn tệp…
settings-soundfont-sent = Giờ MIDI phát bằng { $name }

## Cài đặt: Tìm kiếm hình ảnh

settings-clip-heading = Tìm theo nội dung trong ảnh
settings-clip-reset = Tìm kiếm hình ảnh: về mặc định
settings-clip-saved-on = Tìm kiếm hình ảnh: bật
settings-clip-saved-off = Tìm kiếm hình ảnh: tắt
settings-clip-what = Nhập thứ bạn đang tìm (“mossy rock”, “sci-fi door”, “wooden barrel”) và những ảnh thể hiện nó sẽ hiện ra cạnh những tên có các từ đó. “Thêm nữa giống thế này” tìm những thứ cùng loại, không chỉ cùng màu.
settings-clip-toggle = Bật tìm kiếm hình ảnh
settings-clip-hint = Dùng mô hình SigLIP 2 của Google, chạy trên bộ xử lý của máy chủ này: không có gì rời khỏi máy chủ. Khi bật, mô hình ({ $size } MB) sẽ được tải từ Hugging Face, rồi học mọi ảnh trong nền với mức ưu tiên thấp. Nó dùng 350 đến 750 MB bộ nhớ khi đang làm việc hoặc khi có người tìm kiếm. Khi tắt, mô hình bị xóa nhưng những gì nó đã học được giữ lại cho lần sau.
settings-clip-hint-desktop = Dùng mô hình SigLIP 2 của Google, chạy trên bộ xử lý của máy tính này: không có gì rời khỏi máy. Khi bật, mô hình ({ $size } MB) sẽ được tải từ Hugging Face, rồi học mọi ảnh trong nền với mức ưu tiên thấp. Nó dùng 350 đến 750 MB bộ nhớ khi đang làm việc hoặc khi bạn tìm kiếm. Khi tắt, mô hình bị xóa nhưng những gì nó đã học được giữ lại cho lần sau.
settings-clip-looking = Đang tìm mô hình…
settings-clip-downloading = Đang tải mô hình xuống… tiến độ hiện ở chỉ báo hoạt động
settings-clip-ready-empty = Sẵn sàng. Ảnh sẽ được học khi gói được thêm vào.
settings-clip-ready-all =
    { $count ->
       *[other] Sẵn sàng: đã biết cả { $count } ảnh
    }
settings-clip-learning = Sẵn sàng: đang học ảnh, đến giờ được { $done } / { $total }
settings-clip-relearning = Cho đến khi mô hình mới học xong mọi ảnh, tìm kiếm dùng những gì mô hình cũ đã học.
settings-clip-unavailable = Không lấy được mô hình: { $detail }. Sẽ thử lại mỗi giờ.
settings-clip-where = Chạy trên
settings-clip-here = Máy chủ này: không có gì rời khỏi nó
settings-clip-here-desktop = Máy tính này: không có gì rời khỏi nó
settings-clip-hosted = Một dịch vụ được lưu trữ bên ngoài
settings-clip-hosted-warning = Hình ảnh rời khỏi máy chủ này. Ảnh thu nhỏ của mọi hình trong thư viện, và từ ngữ của mọi lần tìm kiếm hình ảnh, được gửi đến { $service }, dịch vụ này tính phí và giữ chúng theo điều khoản của mình.
settings-clip-hosted-warning-desktop = Hình ảnh rời khỏi máy tính này. Ảnh thu nhỏ của mọi hình trong thư viện, và từ ngữ của mọi lần tìm kiếm hình ảnh, được gửi đến { $service }, dịch vụ này tính phí và giữ chúng theo điều khoản của mình.
settings-clip-hosted-hint = Không có gì được tải xuống hay chạy ở đây: dịch vụ học từng hình, theo từng lô. Một dịch vụ hoặc mô hình khác sẽ học lại tất cả, và những gì mỗi bên đã học được giữ lại để chuyển về.
settings-clip-saved-here = Tìm kiếm hình ảnh: chạy trên máy chủ này
settings-clip-saved-here-desktop = Tìm kiếm hình ảnh: chạy trên máy tính này
settings-clip-saved-hosted = Tìm kiếm hình ảnh: chạy trên { $service }
settings-clip-hosted-unavailable = Không dùng được { $service }: { $detail }. Sẽ thử lại mỗi giờ.
settings-clap-heading = Tìm theo âm thanh nghe ra sao
settings-clap-reset = Tìm kiếm âm thanh: về mặc định
settings-clap-saved-on = Tìm kiếm âm thanh: bật
settings-clap-saved-off = Tìm kiếm âm thanh: tắt
settings-clap-what = Nhập thứ bạn đang tìm (“distant thunder”, “glassy UI blip”, “footsteps on gravel”) và những âm thanh nghe giống vậy sẽ hiện ra cạnh những tên có các từ đó.
settings-clap-toggle = Bật tìm kiếm âm thanh
settings-clap-hint = Dùng mô hình CLAP của LAION, chạy trên bộ xử lý của máy chủ này: không có gì rời khỏi máy chủ. Khi bật, mô hình ({ $size } MB) sẽ được tải từ Hugging Face, rồi nghe phần đầu của mọi âm thanh trong nền với mức ưu tiên thấp. Nó dùng tối đa khoảng 1 GB bộ nhớ khi đang làm việc hoặc khi có người tìm kiếm. Khi tắt, mô hình bị xóa nhưng những gì nó đã học được giữ lại cho lần sau.
settings-clap-hint-desktop = Dùng mô hình CLAP của LAION, chạy trên bộ xử lý của máy tính này: không có gì rời khỏi máy. Khi bật, mô hình ({ $size } MB) sẽ được tải từ Hugging Face, rồi nghe phần đầu của mọi âm thanh trong nền với mức ưu tiên thấp. Nó dùng tối đa khoảng 1 GB bộ nhớ khi đang làm việc hoặc khi bạn tìm kiếm. Khi tắt, mô hình bị xóa nhưng những gì nó đã học được giữ lại cho lần sau.
settings-clap-ready-empty = Sẵn sàng. Âm thanh sẽ được học khi gói được thêm vào.
settings-clap-ready-all =
    { $count ->
       *[other] Sẵn sàng: đã biết cả { $count } âm thanh
    }
settings-clap-learning = Sẵn sàng: đang học âm thanh, đến giờ được { $done } / { $total }

## Cài đặt: Gắn thẻ AI

settings-ai-service = Dịch vụ
settings-ai-off = Tắt
settings-ai-off-hint = Gói chỉ được gắn thẻ bằng các quy tắc có sẵn.
settings-ai-local = Mô hình cục bộ
settings-ai-local-hint = Dịch vụ llm trong tệp compose của Hoard (hoặc cargo xtask llm). Hãy tải một mô hình bên dưới; nó chạy trên máy này và không có gì rời khỏi máy.
settings-ai-chatgpt = Gói ChatGPT
settings-ai-chatgpt-hint = Dùng gói ChatGPT Go, Plus, Pro hoặc Business của bạn thay cho khóa API.
settings-ai-openrouter-hint = Một khóa cho hàng trăm mô hình được lưu trữ, một số miễn phí. Mô hình nhận hình ảnh được đánh dấu.
settings-ai-openai-hint = Do OpenAI lưu trữ.
settings-ai-anthropic-hint = Do Anthropic lưu trữ, qua endpoint tương thích OpenAI của họ.
settings-ai-gemini-hint = Do Google lưu trữ, qua endpoint tương thích OpenAI của họ.
settings-ai-ollama-hint = Một máy chủ Ollama do bạn chạy. Hãy đổi địa chỉ nếu nó không phải container tên ollama.
settings-ai-custom = Khác (tương thích OpenAI)
settings-ai-custom-hint = Bất cứ thứ gì hỗ trợ API chat của OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Đang kết nối…
settings-ai-connected = Đã kết nối · { $count } mô hình
settings-ai-connected-none = Đã kết nối, nhưng không có mô hình nào được liệt kê; hãy nhập tên mô hình
settings-chatgpt-connected-none = Đã kết nối; hãy nhập tên mô hình
settings-ai-takes-images = nhận hình ảnh
settings-ai-saved = Gắn thẻ AI: { $service }
settings-ai-saved-off = Gắn thẻ AI đang tắt
settings-ai-reset = Gắn thẻ AI: về mặc định
settings-ai-on = Bật: { $service } · { $model }
settings-ai-url = Địa chỉ
settings-ai-key = Khóa API
settings-ai-key-saved = Đã lưu (nhập để thay thế)
settings-ai-key-paste = Dán khóa của bạn
settings-ai-key-optional = Không có (tùy chọn)
settings-ai-key-remove = Gỡ
settings-ai-get-key = Lấy khóa
settings-ai-model = Mô hình
settings-ai-model-pick = Chọn hoặc nhập mô hình
settings-ai-model-name = Tên mô hình
settings-ai-refresh = Làm mới
settings-ai-connect = Kết nối
settings-ai-need-key = Thêm khóa API để xem các mô hình
settings-ai-vision = Gửi kèm ảnh thu nhỏ (cần mô hình nhận hình ảnh)
settings-ai-batch = Mỗi yêu cầu
settings-ai-batch-unit = tài nguyên (ít hơn cho mô hình cục bộ nhỏ, nhiều hơn cho mô hình lớn được lưu trữ)

## Cài đặt: Đăng nhập ChatGPT

settings-chatgpt-account = Tài khoản
settings-chatgpt-checking = Đang kiểm tra…
settings-chatgpt-status-as = Đã đăng nhập dưới tên { $email }
settings-chatgpt-status = Đã đăng nhập
settings-chatgpt-signed-in-as = Đã đăng nhập ChatGPT dưới tên { $email }
settings-chatgpt-signed-in = Đã đăng nhập ChatGPT
settings-chatgpt-signed-out = Đã đăng xuất khỏi ChatGPT
settings-chatgpt-continue = Tiếp tục với ChatGPT
settings-chatgpt-sign-out = Đăng xuất
settings-chatgpt-sign-out-title = Đăng xuất khỏi ChatGPT?
settings-chatgpt-sign-out-in-use = Gắn thẻ AI đang dùng gói ChatGPT của bạn, nên nó sẽ dừng cho đến khi bạn đăng nhập lại hoặc chọn dịch vụ khác.
settings-chatgpt-sign-out-body = Hoard sẽ quên phiên đăng nhập và báo OpenAI kết thúc nó.
settings-chatgpt-manage = Quản lý mức sử dụng
settings-chatgpt-finish-before = Hãy hoàn tất đăng nhập trong cửa sổ vừa mở (
settings-chatgpt-finish-link = mở lại
settings-chatgpt-finish-middle = ). Nếu nó kết thúc ở một trang không tải được, tại địa chỉ bắt đầu bằng{ " " }
settings-chatgpt-finish-after = , hãy sao chép địa chỉ đó và dán vào đây:
settings-chatgpt-paste = Địa chỉ nơi kết thúc đăng nhập
settings-chatgpt-finish = Hoàn tất
settings-chatgpt-usage = Các yêu cầu gắn thẻ được tính vào mức sử dụng của gói ChatGPT. Quản lý mức sử dụng cho biết Hoard đã dùng bao nhiêu và cho phép bạn giới hạn. Để ngắt kết nối Hoard khỏi chính ChatGPT, hãy vào Settings → Security and login → Sign in with ChatGPT trên chatgpt.com.

## Cài đặt: Mô hình cục bộ

settings-local-downloading-progress = Đang tải xuống { $percent }% · { $done } / { $total }
settings-local-downloading = Đang tải xuống…
settings-local-incomplete = Đã dừng tải xuống
settings-local-failed = Lần trước khởi động thất bại
settings-local-in-use-running = Đang dùng · đang chạy
settings-local-in-use = Đang dùng
settings-local-starting = Đang khởi động…
settings-local-running = Đang chạy
settings-local-ready-images = Sẵn sàng · nhận hình ảnh
settings-local-ready-text = Sẵn sàng · chỉ văn bản
settings-local-delete-title = Xóa { $name }?
settings-local-delete-in-use = Gắn thẻ AI đang dùng mô hình này, nên nó sẽ dừng cho đến khi bạn chọn mô hình khác. Các tệp của mô hình sẽ bị xóa khỏi máy chủ mô hình.
settings-local-delete-downloading = Việc tải xuống sẽ dừng và phần đã tải về sẽ bị xóa.
settings-local-delete-body = Các tệp của mô hình sẽ bị xóa khỏi máy chủ mô hình. Bạn có thể tải lại sau.
settings-local-delete = Xóa
settings-local-deleting = Đang xóa…
settings-local-cancel = Hủy
settings-local-resume = Tiếp tục
settings-local-download = Tải xuống
settings-local-start-before = Hãy khởi động nó bằng{ " " }
settings-local-start-or = { " " }(hoặc{ " " }
settings-local-start-developing = { " " }khi phát triển), rồi{ " " }
settings-local-start-retry = thử lại
settings-local-start-after = . Máy chủ chỉ chạy một mô hình cũng được: hãy nhập tên mô hình đó bên dưới.
settings-local-list = Trên máy chủ mô hình
settings-local-none = Chưa tải gì xuống. Hãy chọn một mô hình bên dưới; nó được giữ trên máy chủ, và bạn có thể đổi bất cứ lúc nào.
settings-local-catalog = Mô hình để tải xuống
settings-local-size = { $size } GB · { $license }
settings-local-other = Mô hình khác từ Hugging Face
settings-local-other-placeholder = Mô hình khác từ Hugging Face: owner/name-GGUF:Q4_K_M

## Cài đặt: Thêm gói

settings-store-pages = Trang cửa hàng
settings-store-pages-what = Gói mới được tra cứu trên các cửa hàng trong nền, để có trang cửa hàng, người bán và giấy phép.
settings-store-lookup = Tự động tra cứu trang cửa hàng
settings-unreal = Package Unreal
settings-unreal-reset = Package Unreal: về mặc định
settings-unreal-what = Làm gì với tệp .uasset.
settings-conv = Quy tắc chuyển đổi
settings-conv-what = Chuyển đổi các tệp Hoard lưu giữ sang một định dạng, chẳng hạn mọi tệp OBJ và FBX sang GLB, để cả thư viện đồng nhất. Tệp đã chuyển đổi thế chỗ tệp gốc, giữ nguyên tên, thẻ, bộ sưu tập và liên kết của nó. Tệp trong thư mục theo dõi không bao giờ bị thay đổi.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Bật
settings-conv-to = Chuyển đổi sang
# Heading over the file formats a rule converts.
settings-conv-from = Từ
settings-conv-remove = Gỡ
settings-conv-remove-label = Gỡ quy tắc { $n }
settings-conv-taken = Một quy tắc khác đã chuyển đổi định dạng này
settings-conv-upright = Dựng thẳng mô hình khi chuyển đổi (Y hướng lên, quay về +Z), theo cách các mô hình của gói được hiển thị
settings-conv-flac-what = Âm thanh giữ nguyên hoàn toàn, chỉ chiếm khoảng một nửa dung lượng, và mọi thứ khác trong WAV (điểm lặp, dấu cue, thẻ) đều được giữ trong FLAC, nên có thể đưa WAV trở lại y như cũ bất cứ lúc nào. Các WAV có mẫu dấu phẩy động hoặc 32 bit sẽ vẫn là WAV.
settings-conv-add = Thêm quy tắc
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Giữ tệp gốc trong
settings-conv-keep-days =
    { $count ->
       *[other] ngày
    }
settings-conv-keep-what = Cho đến lúc đó, trang của tệp đã chuyển đổi có thể đưa tệp gốc trở lại. Texture không được sao chép vào mô hình đã chuyển đổi: mô hình trỏ tới các tệp ảnh nằm cạnh nó. Các tệp .mtl hoặc .bin của mô hình vẫn được giữ lại.
settings-conv-save = Lưu quy tắc
settings-conv-revert = Hoàn tác thay đổi
settings-conv-existing = Chuyển đổi các tệp đã có trong thư viện
settings-conv-saved = Đã lưu quy tắc chuyển đổi. Chúng áp dụng cho các gói được thêm từ bây giờ.
settings-conv-converting =
    { $count ->
       *[other] Đang chuyển đổi { $count } tệp trong nền
    }
settings-conv-nothing = Không có gì trong thư viện cần chuyển đổi.
settings-conv-ask-title =
    { $count ->
       *[other] Chuyển đổi { $count } tệp đã có trong thư viện?
    }
settings-conv-ask-body =
    { $packs ->
       *[other] Chúng nằm trong { $packs } gói.
    } Mỗi tệp gốc được giữ { $days } ngày, và cho đến lúc đó có thể đưa trở lại từ trang của nó.
settings-conv-ask-busy =
    { $count ->
       *[other] { $count } tệp nằm trong các gói vẫn đang được thêm; chúng sẽ được chuyển đổi khi việc đó xong.
    }
settings-conv-ask-list = Các tệp sẽ được chuyển đổi
settings-conv-ask-more = …và { $count } tệp khác
settings-conv-ask-skipped = Giữ nguyên:
settings-conv-skip-watched =
    { $count ->
       *[other] { $count } tệp trong thư mục theo dõi
    }
settings-conv-skip-unity =
    { $count ->
       *[other] { $count } tệp mà một dự án Unity biết theo tên
    }
settings-conv-skip-used =
    { $count ->
       *[other] { $count } tệp được một cảnh hoặc tệp khác nhắc đến theo tên
    }
settings-conv-skip-beside =
    { $count ->
       *[other] { $count } tệp có bản đã chuyển đổi nằm sẵn trong gói của nó
    }
settings-conv-skip-blender =
    { $count ->
       *[other] { $count } tệp Blender chưa đọc (Cài đặt → Công cụ)
    }
settings-conv-ask-new = Lưu, chỉ gói mới
settings-conv-ask-save-convert =
    { $count ->
       *[other] Lưu và chuyển đổi { $count } tệp
    }
settings-conv-ask-convert =
    { $count ->
       *[other] Chuyển đổi { $count } tệp
    }
settings-unreal-off = Giữ nguyên
settings-unreal-off-hint = Chuyển đổi từ trang của gói khi bạn muốn.
settings-unreal-keep = Chuyển đổi, giữ tệp gốc
settings-unreal-keep-hint = Texture, âm thanh và vật liệu được xuất ra định dạng phổ biến; tệp .uasset vẫn được giữ.
settings-unreal-replace = Chuyển đổi, bỏ tệp gốc
settings-unreal-replace-hint = Tiết kiệm dung lượng sau khi nội dung đã được lấy ra. Mesh vẫn chờ Unreal worker.

## Cài đặt: Tải xuống

settings-tn = Tên texture
settings-tn-saved = Đã lưu tên texture
settings-tn-reset = Tên texture: về mặc định
settings-tn-what = Các map của một vật liệu được đặt tên theo vật liệu và công dụng của từng map khi gửi đi, dù tệp có tên gì: Rock_Normal.png, không phải rock_moss_xeokah0_raw_8K_Normal.png. Tệp trong thư viện không bị động tới, và map mà mô hình hoặc MTL đi kèm nêu tên sẽ giữ tên đó, để mô hình vẫn tìm thấy nó.
settings-tn-toggle = Đặt tên map texture theo vật liệu
settings-tn-style = Kiểu
settings-tn-style-group = Kiểu đặt tên
settings-tn-descriptive = Mô tả
settings-tn-short = Ngắn
settings-tn-custom = Tùy chỉnh
settings-tn-pattern = Mẫu
settings-tn-hint-name = { " " }là vật liệu,{ " " }
settings-tn-hint-map = { " " }là công dụng của map,{ " " }
settings-tn-hint-size = { " " }là kích thước khi bộ có nhiều kích thước (được thêm vào cuối nếu mẫu không có).
settings-tn-example = Ví dụ
settings-tn-suffixes = Hậu tố cho từng loại map
settings-tn-slots = Trong bản tải xuống GLB, texture được đóng gói bên trong mô hình mà không có tệp riêng sẽ được đặt tên theo vật liệu và slot mà nó chiếm, và các slot này cũng có hậu tố:
settings-tn-endings = Đuôi tên riêng
settings-tn-endings-what = Texture không phải là một map của vật liệu sẽ giữ tên riêng của nó, trừ khi tên kết thúc bằng một trong các đuôi này: _diff → _BC gửi rock_diff.png thành rock_BC.png. Để trống ô thứ hai để bỏ đuôi tên.
settings-tn-ending-from = Đuôi tên
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Thay bằng
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Thêm đuôi tên
settings-tn-ending-remove = Bỏ
settings-tn-ending-remove-label = Bỏ { $ending }

## Cài đặt: Chung và Kết nối

settings-name = Tên
settings-name-what = Tên của máy chủ này trên thanh trên cùng, tab trình duyệt và trang đăng nhập, để khi có nhiều máy chủ Hoard bạn biết cái nào là cái nào.
settings-name-what-desktop = Tên của thư viện này trên thanh trên cùng.
settings-address = Địa chỉ
settings-address-what = Địa chỉ mà mọi người và ứng dụng dùng để kết nối tới Hoard, khi nó khác với địa chỉ trình duyệt này đang dùng: địa chỉ của reverse proxy, tunnel hoặc tên tailnet, ví dụ https://hoard.example.com. Các liên kết Hoard đưa ra (cho Claude và ứng dụng khác, khi đăng nhập, trên chứng chỉ của nó) đều dùng địa chỉ này.
settings-public-url = Địa chỉ công khai
settings-public-url-placeholder = Lấy từ địa chỉ trình duyệt (ví dụ https://hoard.example.com)
settings-public-url-default = lấy từ địa chỉ trình duyệt

## Cài đặt: Hiệu năng, trình phân tích hiệu năng ở đầu trang

profiler = Trình phân tích hiệu năng
profiler-what = Những gì Hoard đã làm trong mười phút qua, cập nhật mỗi hai giây khi bạn đang xem. Dữ liệu chỉ được giữ trong bộ nhớ và bắt đầu lại khi Hoard khởi động lại.
profiler-starting = Các số đo đầu tiên sẽ có trong giây lát.
profiler-failed = Không thể đọc trình phân tích hiệu năng: { $error }
profiler-cpu = Mức dùng bộ xử lý
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Toàn bộ máy
profiler-cpu-summary = Hoard đang dùng { $hoard } bộ xử lý, còn toàn bộ máy dùng { $machine }.
profiler-ram = RAM
profiler-ram-peak = Cao nhất
profiler-ram-of = { $used } / { $total }
profiler-ram-summary = Hoard đang dùng { $hoard } RAM, và dùng nhiều nhất { $peak } trong mười phút qua.
profiler-serving = Yêu cầu đang xử lý
profiler-working = Đang làm
profiler-working-none = Không có gì chạy trong nền.
profiler-working-more =
    { $count ->
       *[other] Và { $count } việc khác.
    }
profiler-requests = Thời gian phục vụ yêu cầu
profiler-requests-what = Mỗi cột là mười giây yêu cầu, với thời gian được chia theo nơi nó được dùng.
profiler-part-db = Cơ sở dữ liệu
profiler-part-db-title = Chờ cơ sở dữ liệu và chạy các truy vấn của nó
profiler-part-disk = Ổ đĩa
profiler-part-disk-title = Đọc các tệp đã lưu và nén những gì được gửi
profiler-part-send = Gửi
profiler-part-send-title = Chờ trong khi mạng nhận những gì đã gửi
profiler-part-other = Khác
profiler-part-other-title = Tính toán câu trả lời
profiler-request-count =
    { $count ->
       *[other] { $count } yêu cầu
    }
profiler-requests-none = Không có yêu cầu nào trong mười phút qua.
profiler-slowest = Chậm nhất: { $route }, { $time }
profiler-routes = Theo tuyến
profiler-route = Tuyến
profiler-count = Yêu cầu
profiler-average = Trung bình
profiler-p95 = 5% chậm nhất
profiler-split = Thời gian đã dùng vào đâu
profiler-sent = Đã gửi
profiler-portal = các tệp của chính giao diện

## Cài đặt: Hiệu năng

settings-render = Ảnh thu nhỏ 3D
settings-render-toggle = Kết xuất ảnh thu nhỏ cho mô hình 3D
settings-gpu-on = Trên GPU: { $renderer }
settings-gpu-software = Không tìm thấy GPU, nên dùng phần mềm ({ $renderer })
settings-gpu-none = Không tìm thấy GPU hay trình kết xuất phần mềm, nên mô hình không có ảnh thu nhỏ
settings-thumbs = Ảnh thu nhỏ
settings-thumbs-what = Cách lưu ảnh thu nhỏ và ảnh xem mô hình mới. Ảnh có độ trong suốt và ảnh pixel art luôn là PNG. Để thu nhỏ những ảnh bạn đã có, hãy chạy Thu nhỏ ảnh thu nhỏ trong Bảo trì.
settings-previews-size = Bản xem trước hiện chiếm { $size }.
settings-thumbs-saved = Ảnh thu nhỏ: { $quality }
settings-thumbs-small = Nhỏ hơn
settings-thumbs-small-hint = JPEG, chất lượng 70. Ổn trong lưới; hơi mờ khi xem gần.
settings-thumbs-balanced = Cân bằng
settings-thumbs-balanced-hint = JPEG, chất lượng 82. Khó phân biệt với bản gốc ở kích thước ảnh thu nhỏ.
settings-thumbs-sharp = Sắc nét hơn
settings-thumbs-sharp-hint = JPEG, chất lượng 92. Lớn khoảng gấp đôi Cân bằng.
settings-thumbs-lossless = Không mất dữ liệu
settings-thumbs-lossless-hint = PNG, đúng như khi tạo. Lớn hơn vài lần.
settings-thumbs-avif = Gửi ảnh thu nhỏ dưới dạng AVIF
settings-thumbs-avif-what = Trình duyệt hiển thị được AVIF sẽ nhận ảnh thu nhỏ nhỏ hơn, thường chưa tới một nửa kích thước. Mỗi ảnh được tạo từ ảnh thu nhỏ đã lưu vào lần đầu hiển thị và được giữ cùng các bản chuyển đổi; trước đó ảnh đã lưu sẽ được gửi. Việc tạo chúng chiếm tới hai lõi trong khi một thư viện lớn được duyệt lần đầu. Ảnh thu nhỏ không mất dữ liệu và pixel art được gửi nguyên trạng.
settings-motion-thumbs = Hình thu nhỏ chuyển động cho hoạt ảnh
settings-motion-thumbs-what = Mỗi hoạt ảnh còn có một đoạn phim ngắn phát lặp lại khi bạn trỏ vào ô của nó. Mỗi đoạn khoảng 40 KB và cần một lúc để tạo cho thư viện lớn. Tắt tùy chọn này sẽ xóa chúng.
settings-limits = Công việc và giới hạn
settings-limits-what = Mỗi mục được lưu khi bạn rời ô hoặc nhấn Enter. Kích thước nhận đơn vị: 100 GB, 512 MB, 1.5 TB; số không có đơn vị là GB.
settings-workers = Tiến trình nền
settings-workers-unit = tác vụ cùng lúc (nhập, gắn thẻ, chuyển đổi)
settings-max-upload = Tải lên lớn nhất
settings-max-upload-unit = mỗi tệp
settings-max-expand = Gói lớn nhất
settings-max-expand-unit = sau khi giải nén
settings-conversion-cache = Bản tải xuống đã chuyển đổi
settings-conversion-cache-unit-now = giữ tối đa, hiện { $size }; thứ ít được dùng gần đây nhất sẽ bị xóa trước
settings-conversion-cache-unit = giữ tối đa; thứ ít được dùng gần đây nhất sẽ bị xóa trước

## Cài đặt: Bảo trì

settings-updates = Cập nhật
settings-version = Đây là Hoard { $version }.
settings-update-check = Kiểm tra GitHub vài giờ một lần để tìm bản phát hành mới
settings-desktop-updates = Mỗi lần khởi động, Hoard tìm bản phát hành mới, tải về ở chế độ nền và cài đặt khi bạn thoát.
settings-backups = Sao lưu theo lịch
settings-backups-what = Cơ sở dữ liệu tự sao lưu; mỗi bản sao lưu hiện trong lịch sử bên dưới.
settings-backup-every = Sao lưu
settings-backup-off = Không bao giờ
settings-backup-daily = Mỗi ngày
settings-backup-weekly = Mỗi tuần
settings-backup-keep = Giữ
settings-backup-keep-unit = bản sao lưu theo lịch mới nhất
settings-backup-copy-to = Cũng sao chép vào
settings-backup-copy-to-none = Không có thư mục khác
settings-backups-kept = Mỗi bản được lưu vào thư mục sao lưu với tên hoard-auto-…db, và vào thư mục khác (ổ đĩa khác, hoặc thư mục được ứng dụng đồng bộ sao lưu) nếu có, ghi bằng đường dẫn đầy đủ trên máy chủ. Chỉ các bản sao lưu theo lịch cũ hơn bị xóa; bản tạo thủ công vẫn giữ lại.
about-what = Hoard giữ tài nguyên game (mô hình, texture, âm thanh, nhạc, sprite và hơn thế) trong một thư viện mà bạn có thể tìm kiếm, xem trước và lấy ra dùng, được chia sẻ với mọi người bạn cho phép truy cập.
about-version = Phiên bản
about-edition = Ấn bản
about-edition-server = Máy chủ
about-edition-desktop = Ứng dụng máy tính
about-edition-steam = Ứng dụng máy tính từ Steam
about-made-by = Tạo bởi
about-source = Mã nguồn trên GitHub
about-issues = Báo cáo sự cố
about-discord = Trò chuyện với chúng tôi trên Discord
about-steam = Hoard trên Steam
about-license = Giấy phép
about-license-what = Hoard là mã nguồn mở theo giấy phép MIT No Attribution: bạn có thể dùng, sửa đổi và chia sẻ tùy ý.
about-downloaded = Tải về khi cần
about-downloaded-what = Hoard không đi kèm những thứ này. Mỗi thứ được tải về vào lần đầu có gì đó cần đến, và mỗi thứ có giấy phép riêng.
about-blender = Chuyển đổi mô hình 3D và tạo hình thu nhỏ.
about-ffmpeg = Chuyển đổi âm thanh và video và tạo bản xem trước.
about-siglip = Tìm hình ảnh theo nội dung trong ảnh.
about-clap = Tìm âm thanh theo cách chúng nghe.
about-models = Các mô hình bạn chọn cho máy chủ AI cục bộ tuân theo giấy phép của người tạo ra chúng.
about-built-with = Được xây dựng bằng
about-built-with-what = Các thư viện mã nguồn mở tạo nên Hoard, kèm giấy phép của chúng.
about-steamworks = Bản Steam còn bao gồm Steamworks SDK, © Valve Corporation.
about-licenses-loading = Đang tải danh sách…
about-licenses-failed = Không tải được danh sách.
about-filter = Lọc theo tên hoặc giấy phép
about-count = { $shown } / { $total }
about-no-license = Không ghi giấy phép
about-report = Báo cáo sự cố…
report-title = Báo cáo sự cố
report-what = Đây là những gì một issue trên GitHub cần để sự cố được xem xét. Không có gì riêng tư trong đó: token truy cập, mật khẩu, khóa, địa chỉ email và tên của bạn trong đường dẫn thư mục đều được loại bỏ. Hãy đọc lại trước khi gửi.
report-loading = Đang thu thập báo cáo…
report-failed = Không thể thu thập báo cáo: { $error }
report-log = Những gì máy chủ ghi nhật ký gần đây
report-copy = Sao chép
report-copied = Đã sao chép báo cáo.
report-github = Mở issue trên GitHub
report-github-hint = Issue mới được điền sẵn bằng nội dung này. Nếu nhật ký quá dài, chỉ những dòng cuối được đưa vào; Sao chép có tất cả.
report-close = Đóng

## Loại tài nguyên (api.ts)

api-kind-texture = Texture
api-kind-vector = Ảnh vector
api-kind-sheet = Bảng sprite
api-kind-audio = Âm thanh
api-kind-model = Mô hình
api-kind-animation = Hoạt ảnh
api-kind-font = Phông chữ
api-kind-shader = Shader
api-kind-material = Vật liệu
api-kind-video = Video
api-kind-document = Tài liệu
api-kind-data = Dữ liệu
api-kind-other = Khác
api-kind-texture-name = texture
api-kind-vector-name = ảnh vector
api-kind-sheet-name = bảng sprite
api-kind-audio-name = âm thanh
api-kind-model-name = mô hình
api-kind-animation-name = hoạt ảnh
api-kind-font-name = phông chữ
api-kind-shader-name = shader
api-kind-material-name = vật liệu
api-kind-video-name = video
api-kind-document-name = tài liệu
api-kind-data-name = dữ liệu
api-kind-other-name = loại khác

## Thông báo, hộp thoại và đang tải (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Hoàn tác
ui-today = hôm nay
ui-yesterday = hôm qua
ui-days-ago = { $days } ngày trước
ui-notifications = Thông báo
ui-dismiss = Bỏ qua
ui-cancel = Hủy
ui-lazy-loading = Đang tải { $what }…
ui-lazy-failed = Không thể tải { $what }.
ui-lazy-reload = Tải lại trang

## ResetButton.vue

reset-title = Về mặc định: { $value }
reset-label = Về mặc định, { $value }

## SizeInput.vue

size-invalid = Một kích thước như 100 GB, 512 MB hoặc 1.5 TB
size-hint = Một kích thước như 100 GB, 512 MB hoặc 1.5 TB; số không có đơn vị là GB

## AuthPanel.vue (Cài đặt → Xác thực)

auth-load-failed = Không thể tải cài đặt xác thực: { $error }
auth-saved = Đã lưu. Áp dụng từ lần đăng nhập tiếp theo.
auth-method-internal = Nội bộ
auth-method-internal-hint = Tên và mật khẩu do Hoard lưu giữ, thêm trong Cài đặt → Tài khoản.
auth-method-ldap-hint = Một máy chủ thư mục kiểm tra tên và mật khẩu nhập ở trang đăng nhập: OpenLDAP, FreeIPA, LLDAP hoặc máy chủ LDAP khác.
auth-method-ad-hint = Tài khoản miền Windows đăng nhập bằng tên và mật khẩu quen thuộc của họ.
auth-method-oauth-hint = Một nút “Đăng nhập bằng…” đi tới Google, Microsoft, GitHub, Keycloak, Authentik hoặc nhà cung cấp khác rồi quay lại.
auth-session-unused = Sau 30 ngày không dùng
auth-session-1h = 1 giờ sau khi đăng nhập
auth-session-8h = 8 giờ sau khi đăng nhập
auth-session-day = Một ngày sau khi đăng nhập
auth-session-week = Một tuần sau khi đăng nhập
auth-session-30d = 30 ngày sau khi đăng nhập
auth-session-hours = { $hours } giờ sau khi đăng nhập
auth-role-view = Được xem
auth-role-edit = Được sửa
auth-role-admin = Quản trị viên
auth-role-word-view = được xem
auth-role-word-edit = được sửa
auth-role-word-admin = quản trị viên
auth-role-word-default = { $role } (vai trò mặc định)
auth-tls-starttls-hint = LDAP thường (cổng 389) chuyển sang TLS trước khi gửi bất cứ thứ gì.
auth-tls-ldaps-hint = TLS ngay từ đầu (cổng 636).
auth-tls-none = Không có
auth-tls-none-hint = Mật khẩu đi qua mạng ở dạng nguyên bản. Chỉ dùng trên mạng bạn tin cậy.
auth-test-failed = Không thể kiểm tra: { $error }
auth-preset-custom = Khác (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel và các nhà cung cấp khác: hãy nhập issuer và Khám phá sẽ điền các địa chỉ.
auth-preset-google-note = Tạo một OAuth client (Web application) trong Google Cloud Console → APIs & Services → Credentials. Google không gửi nhóm, nên hãy cấp vai trò trong Cài đặt → Tài khoản, và tắt việc tạo tài khoản ở lần đăng nhập đầu tiên, trừ khi ai có tài khoản Google cũng được vào.
auth-preset-microsoft-note = Đăng ký một ứng dụng trong Entra ID → App registrations (chuyển hướng Web tới địa chỉ bên dưới). Thay “organizations” bằng tenant ID của bạn để chỉ cho tổ chức của bạn vào. Để lấy vai trò từ nhóm, hãy thêm groups claim trong Token configuration: nó liệt kê ID của nhóm.
auth-preset-github-note = Tạo một OAuth app trong GitHub → Settings → Developer settings. GitHub không gửi nhóm, nên hãy cấp vai trò trong Cài đặt → Tài khoản, và tắt việc tạo tài khoản ở lần đăng nhập đầu tiên, trừ khi ai có tài khoản GitHub cũng được vào.
auth-preset-gitlab-note = Tạo một application trong GitLab → Preferences → Applications (hoặc khu vực quản trị của máy chủ GitLab của bạn: hãy đổi gitlab.com trong các địa chỉ).
auth-discovered = Đã điền các địa chỉ của nhà cung cấp.
auth-copied = Đã sao chép
auth-copy-failed = Không thể sao chép: hãy chọn và tự sao chép
auth-sign-in-with = Đăng nhập bằng
auth-own-accounts-still-work = Tài khoản riêng của Hoard vẫn đăng nhập bằng mật khẩu riêng, nên máy chủ thư mục hay nhà cung cấp bị sập hoặc cấu hình sai cũng không thể khóa bạn ở ngoài.
auth-sessions = Phiên đăng nhập
auth-sign-out-after = Đăng xuất mọi người
auth-ldap-server = Máy chủ LDAP
auth-server-type = Loại máy chủ
auth-server-type-custom = Tùy chỉnh
auth-servers = Máy chủ
auth-server-n = Máy chủ { $n }
auth-remove-server-n = Gỡ máy chủ { $n }
auth-remove = Gỡ
auth-add-server = Thêm máy chủ
auth-servers-hint = Khi không kết nối được tới một máy chủ, máy chủ tiếp theo sẽ được thử.
auth-security = Bảo mật
auth-connection-security = Bảo mật kết nối
auth-skip-verify = Không kiểm tra chứng chỉ của máy chủ (chỉ để thử nghiệm)
auth-ca = Tổ chức cấp chứng chỉ
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (để trống: những tổ chức mà hệ thống tin cậy)
auth-lookup = Tra cứu người dùng
auth-anonymously = Ẩn danh
auth-reader-account = Tài khoản đọc
auth-reader-password = Mật khẩu của nó
auth-saved-type-to-replace = Đã lưu: nhập để thay thế
auth-people-under = Người dùng nằm dưới
auth-name-attribute = Thuộc tính tên
auth-only-matching = Chỉ các mục khớp
auth-groups-under = Nhóm nằm dưới
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (để trống: chỉ memberOf)
auth-their-groups = Nhóm của họ
auth-group-filter-hint = { "{" }dn{ "}" } là mục của người đó, { "{" }user{ "}" } là tên họ dùng để đăng nhập.
auth-test = Kiểm tra
auth-test-hint = Kết nối tới từng máy chủ và đăng nhập bằng tài khoản đọc. Nếu có tên và mật khẩu, cũng đăng nhập bằng người đó và cho thấy Hoard sẽ hiểu kết quả ra sao. Không có gì được lưu.
auth-test-user = Người để đăng nhập thử
auth-test-user-placeholder = Người để đăng nhập thử (tùy chọn)
auth-test-password = Mật khẩu của họ
auth-testing = Đang kiểm tra…
auth-test-connectivity = Kiểm tra kết nối
auth-test-server-ok = đã kết nối, và tài khoản đọc đã đăng nhập
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }đã đăng nhập (
auth-test-signed-in-after = ).
auth-test-role = Họ sẽ là: { $role }.
auth-test-no-role = Họ không thuộc nhóm nào trong Vai trò, nên sẽ bị từ chối.
auth-test-groups = Nhóm: { $groups }
auth-test-no-groups = Không tìm thấy nhóm nào.
auth-provider = Nhà cung cấp
auth-redirect = Địa chỉ chuyển hướng
auth-copy = Sao chép
auth-redirect-hint = Hãy đưa địa chỉ này cho nhà cung cấp khi tạo client cho Hoard. Nó lấy từ địa chỉ công khai trong Cài đặt → Kết nối, hoặc địa chỉ bạn đang dùng.
auth-issuer = Issuer
auth-discover = Khám phá
auth-client-id = Client ID
auth-client-secret = Client secret
auth-authorization-url = Địa chỉ ủy quyền
auth-token-url = Địa chỉ token
auth-userinfo-url = Địa chỉ thông tin người dùng
auth-userinfo-placeholder = (để trống: ID token cho biết đó là ai)
auth-scopes = Scope
auth-name-from = Tên lấy từ
auth-groups-from = Nhóm lấy từ
auth-groups-from-placeholder = (không có)
auth-button = Nút
auth-button-placeholder = tài khoản của bạn
auth-accounts = Tài khoản
auth-provision = Tạo tài khoản Hoard ở lần đầu tiên ai đó đăng nhập
auth-provision-on-hint = Nếu không, quản trị viên phải thêm từng người trước, trong Cài đặt → Tài khoản.
auth-provision-off-hint = Quản trị viên thêm từng người trước, trong Cài đặt → Tài khoản, và chọn cách họ đăng nhập.
auth-open-to-anyone = Bất kỳ ai có tài khoản { $provider } đều có thể vào. Hãy thêm nhóm được xem bên dưới, hoặc tắt mục này và tự thêm người dùng.
auth-new-accounts = Tài khoản mới
auth-roles-from-groups = Vai trò từ nhóm
auth-roles-hint = Các nhóm có thành viên nhận từng vai trò, mỗi dòng một nhóm, theo tên hoặc DN. Vai trò cao nhất khớp sẽ được dùng, và được kiểm tra ở mỗi lần đăng nhập. Nếu không liệt kê nhóm nào, tài khoản mới nhận vai trò ở trên và quản trị viên đổi sau. Nếu có nhóm được liệt kê ở Được xem, chỉ thành viên của nhóm được liệt kê mới được đăng nhập.
auth-roles-hint-oauth = Các nhóm có thành viên nhận từng vai trò, mỗi dòng một nhóm, theo tên hoặc DN (hoặc bất cứ thứ gì groups claim liệt kê). Vai trò cao nhất khớp sẽ được dùng, và được kiểm tra ở mỗi lần đăng nhập. Nếu không liệt kê nhóm nào, tài khoản mới nhận vai trò ở trên và quản trị viên đổi sau. Nếu có nhóm được liệt kê ở Được xem, chỉ thành viên của nhóm được liệt kê mới được đăng nhập.
auth-saving = Đang lưu…
auth-save = Lưu

## HttpsPanel.vue (Cài đặt → Kết nối)

https-load-failed = Không thể tải HTTPS: { $error }
https-off-redirecting = HTTPS đã tắt. Đang chuyển bạn tới http://
https-now-on = HTTPS đã bật
https-saved = Đã lưu. Các kết nối mới dùng chứng chỉ mới.
https-now-off = HTTPS đã tắt
https-http3-failed-toast = Không thể khởi động HTTP/3: { $error }
https-copied = Đã sao chép
https-copy-failed = Không thể sao chép: hãy chọn và tự sao chép
https-mode-off = Tắt
https-mode-off-hint = Chỉ http:// thông thường.
https-mode-own = Chứng chỉ của Hoard
https-mode-own-hint = Hoard tạo một chứng chỉ cho các địa chỉ của máy chủ này. Hãy cài tổ chức cấp chứng chỉ của Hoard một lần trên mỗi thiết bị và trình duyệt trên đó sẽ tin cậy nó.
https-mode-custom = Chứng chỉ của tôi
https-mode-custom-hint = Một chứng chỉ bạn đã có, ví dụ từ tailscale cert hoặc Let’s Encrypt. Trình duyệt tin cậy nó mà không cần cài gì.
https-untrusted-own = Trình duyệt này chưa tin cậy nó cho { $host }: hãy cài tổ chức cấp chứng chỉ của Hoard bên dưới. Bạn sẽ thấy một cảnh báo có thể bấm bỏ qua.
https-untrusted-uncovered = Trình duyệt này chưa tin cậy nó cho { $host }: chứng chỉ không bao gồm địa chỉ này. Bạn sẽ thấy một cảnh báo có thể bấm bỏ qua.
https-untrusted = Trình duyệt này chưa tin cậy nó cho { $host }. Bạn sẽ thấy một cảnh báo có thể bấm bỏ qua.
https-now = Hiện tại
https-start-failed = HTTPS đã được thiết lập nhưng không thể khởi động: { $error }
https-on-port = HTTPS đang bật, trên cổng { $port } giống http://.
https-using-it = Bạn đang dùng nó.
https-open = Mở { $url }
https-trusted = Trình duyệt này tin cậy nó.
https-cert-for-before = Chứng chỉ cho{ " " }
https-cert-for-after = , đến { $date }.
https-cert-for-after-own = , đến { $date } (tự gia hạn).
https-not-covered-before = Nó không bao gồm{ " " }
https-not-covered-after = , địa chỉ bạn đang dùng.
https-is-off = HTTPS đang tắt: các trang được phục vụ qua http:// thông thường.
https-addresses = Địa chỉ
https-addresses-hint = Chứng chỉ bao gồm các địa chỉ Hoard tìm thấy, và bất kỳ địa chỉ nào bạn thêm (tên router đặt cho nó, tên Tailscale).
https-remove-name = Gỡ { $name }
https-another-address = Địa chỉ khác
https-another-address-placeholder = Địa chỉ khác, ví dụ nas.local
https-add = Thêm
https-your-cert = Chứng chỉ của bạn
https-saved-cert-before = Đã lưu:{ " " }
https-saved-cert-after = { " " }từ { $issuer }, đến { $date }. Dán chứng chỉ mới để thay thế.
https-pem-before = Chứng chỉ (kèm chuỗi chứng chỉ) và khóa riêng của nó, ở dạng văn bản PEM: các tệp bắt đầu bằng{ " " }
https-pem-after = .
https-certificate = Chứng chỉ
https-choose-file = Chọn tệp…
https-private-key = Khóa riêng
https-checked-before = Cho{ " " }
https-checked-after = , từ { $issuer }, đến { $date }.
https-checked-after-days = , từ { $issuer }, đến { $date } ({ $days } ngày).
https-self-signed = Đây là chứng chỉ tự ký: trình duyệt sẽ cảnh báo về nó.
https-key-matches = Khóa khớp.
https-where-to-get = Lấy chứng chỉ ở đâu
https-get-tailscale-before = bật HTTPS cho tailnet của bạn, rồi chạy{ " " }
https-get-tailscale-after = { " " }trên máy chủ và tải hai tệp lên. Chúng có hạn 90 ngày: hãy chạy lại và tải lên lại trước khi hết hạn.
https-get-acme-title = Let’s Encrypt hoặc ACME client khác
https-get-acme-before = (certbot, acme.sh, lego) cho tên miền bạn sở hữu: tải lên{ " " }
https-get-acme-and = { " " }và{ " " }
https-get-acme-after = .
https-get-proxy-title = Một reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) có chứng chỉ: hãy để HTTPS tắt ở đây và để proxy gửi http:// thông thường tới Hoard.
https-trying = Đang thử…
https-apply = Áp dụng
https-apply-hint = Hoard thử thiết lập mới trước, và giữ thiết lập hiện có nếu nó không hoạt động.
https-trust-title = Tin cậy Hoard trên thiết bị của bạn
https-download-ca = Tải xuống hoard-ca.crt
https-trust-hint = Hãy cài tổ chức cấp chứng chỉ của Hoard một lần trên mỗi thiết bị, và trình duyệt trên đó sẽ tin cậy máy chủ này (và mọi chứng chỉ Hoard tạo cho nó sau này). Nó chỉ có thể bảo đảm cho Hoard này. Dấu vân tay (SHA-256):
https-ca-windows = Mở tệp → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Khởi động lại trình duyệt.
https-ca-macos = Mở tệp; nó sẽ vào Keychain Access. Nhấp đúp vào “{ $subject }” ở đó → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone và iPad
https-ca-ios = Tải xuống bằng Safari và cho phép. Cài đặt → Đã tải về hồ sơ → Cài đặt. Sau đó Cài đặt → Cài đặt chung → Giới thiệu → Cài đặt tin cậy chứng nhận → bật “{ $subject }”.
https-ca-android = Cài đặt → Bảo mật → Cài đặt bảo mật khác → Mã hóa và thông tin xác thực → Cài đặt chứng chỉ → Chứng chỉ CA, rồi chọn tệp. (Tên gọi hơi khác nhau giữa các điện thoại; hãy tìm “chứng chỉ” trong Cài đặt.)
https-ca-linux-1 = Debian và Ubuntu:{ " " }
https-ca-linux-2 = . Fedora và Arch:{ " " }
https-ca-linux-3 = . Chrome trên Linux giữ danh sách riêng: Settings → Privacy and security → Security → Manage certificates → Import.
https-ca-firefox = Firefox giữ danh sách riêng: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import, và đánh dấu “Trust this CA to identify websites”.
https-ca-cli-title = Công cụ dòng lệnh hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , hoặc đặt đường dẫn vào{ " " }
https-ca-cli-after = .
https-options = Tùy chọn
https-redirect = Chuyển trình duyệt trên thiết bị khác từ http:// sang https://
https-redirect-hint = Chỉ việc tải trang được chuyển. Công cụ dòng lệnh hoard, Unreal worker, script và ứng dụng AI vẫn hoạt động trên http://, và http://localhost trên chính máy chủ cũng vậy.
https-http3-hint-before = Nhanh hơn trên Wi-Fi chập chờn và điện thoại. Cần HTTPS, và cổng UDP { $port } được mở cùng với TCP (trong Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Đang chạy.
https-http3-waits = Nó khởi động khi HTTPS được bật.
https-http3-failed = Không thể khởi động HTTP/3: { $error }
https-lockout-title = Nếu bạn bị khóa ở ngoài
https-lockout-1 = { "" }
https-lockout-2 = { " " }trên chính máy chủ luôn hoạt động qua http:// thông thường. Hoặc tắt HTTPS từ dòng lệnh của máy chủ rồi khởi động lại Hoard:{ " " }
https-lockout-3 = { " " }(trong Docker:{ " " }
https-lockout-4 = , rồi{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Cài đặt → Tài khoản)

account-role-view = Được xem
account-role-view-hint = Tìm kiếm, xem trước và tải xuống
account-role-edit = Được sửa
account-role-edit-hint = Thêm cả việc thêm, sửa, gắn thẻ, gom vào bộ sưu tập và lưu trữ
account-role-admin = Quản trị viên
account-role-admin-hint = Thêm cả Cài đặt, tài khoản, bảo trì và xóa vĩnh viễn
account-source-internal = Mật khẩu Hoard
account-source-ldap = Mật khẩu { $directory } của họ
account-source-oauth = Đăng nhập bằng { $provider }
account-source-word-directory = máy chủ thư mục
account-load-failed = Không thể tải tài khoản: { $error }
account-failed = { $what }: { $error }
account-password-failed = Không thể đổi mật khẩu của bạn
account-password-changed = Đã đổi mật khẩu. Các thiết bị khác của bạn đã bị đăng xuất.
account-add-failed = Không thể thêm { $name }
account-added = { $name } giờ có thể đăng nhập.
account-role-failed = Không thể thay đổi quyền của { $name }
account-remove-title = Gỡ { $name }?
account-remove-body = Họ sẽ bị đăng xuất ở mọi nơi và không thể đăng nhập lại. Không có gì họ đã thêm vào thư viện bị gỡ.
account-remove = Gỡ
account-remove-failed = Không thể gỡ { $name }
account-token-failed = Không thể tạo token
account-copied = Đã sao chép
account-copy-token-failed = Không thể sao chép: hãy chọn token và tự sao chép
account-copy-failed = Không thể sao chép: hãy chọn và tự sao chép
account-revoke-title = Thu hồi “{ $name }”?
account-revoke-body = Mọi thứ còn dùng nó sẽ bị từ chối từ giờ trở đi.
account-revoke = Thu hồi
account-revoke-failed = Không thể thu hồi “{ $name }”
account-disconnect-title = Ngắt kết nối { $app }?
account-disconnect-body = Nó sẽ bị từ chối từ giờ trở đi, và phải kết nối lại để dùng Hoard.
account-disconnect = Ngắt kết nối
account-disconnect-failed = Không thể ngắt kết nối { $app }
account-guest-failed = Không thể thay đổi mục đó
account-never = chưa bao giờ
account-you = Bạn
account-sign-out = Đăng xuất
account-signed-in-as-before = Đã đăng nhập dưới tên{ " " }
account-signed-in-as-admin = , là quản trị viên.
account-signed-in-as-edit = , người có thể sửa thư viện.
account-signed-in-as-view = , người có thể xem nhưng không sửa được gì.
account-app-signin = Ứng dụng Hoard tự đăng nhập cho bạn nên không có mật khẩu. Hãy tạo mã truy cập bên dưới cho các plugin engine và lệnh hoard.
account-password-at-provider = Mật khẩu của bạn do nhà cung cấp bạn dùng để đăng nhập lưu giữ, nên hãy đổi nó ở đó.
account-password-at-directory = Mật khẩu của bạn do máy chủ thư mục bạn dùng để đăng nhập lưu giữ, nên hãy đổi nó ở đó.
account-current-password = Mật khẩu hiện tại
account-new-password = Mật khẩu mới
account-new-password-again = Nhập lại mật khẩu mới
account-change-password = Đổi mật khẩu
account-password-mismatch = Hai mật khẩu mới không khớp.
account-password-rule = Ít nhất 8 ký tự.
account-people = Người dùng
account-people-hint = Mỗi người có thể được xem (tìm kiếm, xem trước và tải xuống), được sửa thư viện nữa (thêm, sửa, gắn thẻ, gom vào bộ sưu tập và lưu trữ), hoặc là quản trị viên (Cài đặt, tài khoản, bảo trì và xóa vĩnh viễn).
account-you-tag = (bạn)
account-signs-in-oauth = Đăng nhập bằng nhà cung cấp OAuth
account-signs-in-directory = Đăng nhập bằng máy chủ thư mục
account-since = từ { $date }
account-what-can-do = Quyền của { $name }
account-name = Tên
account-how-sign-in = Cách họ đăng nhập
account-password-label = Mật khẩu (ít nhất 8 ký tự)
account-password-placeholder = Mật khẩu (8+ ký tự)
account-what-they-can-do = Quyền của họ
account-add = Thêm
account-ai-apps = Claude và các ứng dụng AI khác
account-ai-apps-hint = Kết nối Claude (hoặc bất kỳ ứng dụng MCP nào) và nó có thể tìm kiếm và lấy tài nguyên cho bạn, cũng như mô tả và gắn thẻ những gì Hoard chưa làm. Nó đăng nhập vào Hoard ở lần đầu tiên; bạn cho phép nó tại đây, trong trình duyệt.
account-mcp-address = Địa chỉ MCP của Hoard
account-copy = Sao chép
account-claude-code-1 = chạy{ " " }
account-claude-code-2 = , rồi{ " " }
account-claude-code-3 = { " " }để đăng nhập.{ " " }
account-claude-code-4 = { " " }mô tả một gói.
account-claude-ai = Settings → Connectors → Add custom connector, dán địa chỉ, rồi Connect. Các ứng dụng này kết nối từ máy chủ của Anthropic, nên Hoard phải truy cập được từ internet qua HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, dán địa chỉ, rồi Connect. Các ứng dụng này kết nối từ máy chủ của Anthropic, nên Hoard phải truy cập được từ internet qua HTTPS (địa chỉ này thì không: hãy đặt Địa chỉ công khai trong Cài đặt → Kết nối).
account-connection-meta = cho { $user } từ { $since } · dùng lần cuối { $used }
account-no-apps = Chưa có ứng dụng nào được kết nối.
account-live = Đang dùng Hoard
account-live-hint = Các ứng dụng đã yêu cầu Hoard điều gì đó trong 10 phút qua. Danh sách tự cập nhật.
account-live-none = Không có gì yêu cầu Hoard điều gì trong 10 phút qua.
account-live-last = gần nhất: { $what } · { $when } · yêu cầu: { $calls }
account-tokens = Token truy cập
account-tokens-hint-1 = Dành cho công cụ dòng lệnh{ " " }
account-tokens-hint-2 = , script, MCP client và Unreal worker: chúng gửi nó dưới dạng{ " " }
account-tokens-hint-3 = . Token có thể làm những gì bạn chọn, tối đa bằng quyền của bạn, nhưng không bao giờ quản lý tài khoản hay token. Một token chỉ được xem là đủ để tìm kiếm, tải xuống và đồng bộ một dự án.
account-tokens-admin = Bạn thấy token và ứng dụng đã kết nối của mọi người; người khác chỉ thấy của họ.
account-made-before = { "" }
account-made-after = : hãy sao chép ngay. Nó sẽ không hiện lại.
account-new-token = Token mới
account-cli-1 = Cho CLI:{ " " }
account-cli-2 = , hoặc đặt nó vào{ " " }
account-cli-3 = .
account-done = Xong
account-token-meta = tạo { $made } · dùng lần cuối { $used }
account-token-meta-by = tạo { $made } bởi { $by } · dùng lần cuối { $used }
account-no-tokens = Chưa có token nào.
account-token-name = Thứ sẽ dùng nó
account-token-name-placeholder = Thứ sẽ dùng nó, ví dụ “Unreal worker”
account-token-role = Quyền của nó
account-make-token = Tạo token
account-guests = Khách
account-guests-allow = Cho phép bất kỳ ai truy cập được máy chủ duyệt mà không cần đăng nhập
account-guests-hint = Họ có thể tìm kiếm, xem trước và tải xuống, nhưng không thể thêm, sửa hay xóa gì, hoặc xem Cài đặt.

## SignIn.vue

signin-create-title = Tạo tài khoản của bạn
signin-title = Đăng nhập
signin-setup-lede = Máy chủ này chưa có tài khoản nào. Tài khoản bạn tạo ở đây có thể làm mọi thứ; sau này hãy thêm tài khoản cho người khác từ Cài đặt → Tài khoản.
signin-overlay-lede = Ai cũng có thể duyệt ở đây, nhưng muốn thay đổi thì cần tài khoản.
signin-with = Đăng nhập bằng { $provider }
signin-or-password = hoặc bằng tên và mật khẩu
signin-username = Tên đăng nhập
signin-name = Tên
signin-new-password = Mật khẩu mới
signin-password = Mật khẩu
signin-password-rule = Ít nhất 8 ký tự.
signin-confirm-password = Xác nhận mật khẩu
signin-mismatch = Hai mật khẩu không khớp.
signin-setup-code = Mã thiết lập
signin-setup-code-hint-before = Mã được cấp cho máy chủ này trong{ " " }
signin-setup-code-hint-after = .
signin-creating = Đang tạo…
signin-signing-in = Đang đăng nhập…
signin-create = Tạo tài khoản
signin-submit = Đăng nhập
signin-keep-browsing = Tiếp tục duyệt
signin-directory-hint = Dùng tên và mật khẩu { $directory } của bạn.
signin-forgot-1 = Quên mật khẩu Hoard? Trên máy chủ, hãy chạy{ " " }
signin-forgot-2 = { " " }(trong Docker,{ " " }
signin-forgot-3 = ) để đặt mật khẩu mới.

## SetupWizard.vue (lần đầu mở một máy chủ mới)

setup-title = Thư viện của bạn nên đặt ở đâu?
setup-lede = Chọn một thư mục trên máy chủ để Hoard lưu các gói của bạn, hoặc trỏ nó tới một thư mục chứa gói bạn đã có. Sau này bạn có thể thêm vị trí khác, trên ổ đĩa khác, từ Cài đặt.
setup-lede-desktop = Chọn một thư mục để Hoard lưu các gói của bạn, hoặc trỏ nó tới một thư mục chứa gói bạn đã có. Sau này bạn có thể thêm vị trí khác, trên ổ đĩa khác, từ Cài đặt.
setup-unreachable = Không thể kết nối tới máy chủ: { $error }
setup-unreachable-desktop = Hoard không phản hồi: { $error }
setup-try-again = Thử lại
setup-loading = Đang tải…
setup-start = Bắt đầu thư viện
setup-ready = Thư viện của bạn đã sẵn sàng
setup-managed-before = Gói mới được đưa vào{ " " }
setup-managed-after = . Thả tệp, thư mục hoặc tệp nén vào bất kỳ đâu trên trang để thêm những gói đầu tiên.
setup-watched-before = Hoard đang theo dõi{ " " }
setup-watched-after = ; các gói sẽ hiện ra khi nó tìm thấy. Để tải lên nữa, hãy thêm một thư viện do Hoard quản lý từ Cài đặt → Vị trí thư viện.
setup-open = Mở thư viện

## ConnectApp.vue (một ứng dụng như Claude xin đăng nhập)

connect-title = Kết nối { $app }?
connect-lede-1 = { " " }sẽ có thể tìm kiếm, tải xuống, mô tả và gắn thẻ mọi thứ trong Hoard này, với tư cách{ " " }
connect-lede-2 = . Nó không thể quản lý tài khoản hay token truy cập.
connect-back-before = Bạn sẽ quay lại{ " " }
connect-back-after = . Có thể ngắt kết nối bất cứ lúc nào trong Cài đặt → Tài khoản.
connect-allow = Cho phép
connect-cancel = Hủy
connect-failed = Không thể kết nối ứng dụng này
connect-go-library = Đi tới thư viện
connect-checking = Đang kiểm tra…

## InsecureNotice.vue (dòng hiện trên http:// thông thường)

insecure-text = Đây là http:// thông thường, nên một vài thứ cần HTTPS: 3D với WebGPU, bộ nhớ tạm hiện đại, cài Hoard như một ứng dụng, và connector của Claude.
insecure-switch = Chuyển sang https://
insecure-set-up = Thiết lập HTTPS
insecure-dismiss = Không hiện lại

## FormatWizard.vue (Cài đặt → Tệp cửa hàng → dạy Hoard tệp của một cửa hàng)

formatwizard-too-big = Tệp này quá lớn để đọc như tệp của cửa hàng.
formatwizard-not-json = Tệp này không phải JSON hợp lệ, nên được hiển thị dưới dạng văn bản.
formatwizard-read-failed = Không thể đọc tệp: { $error }
formatwizard-back-title = Quay lại biểu mẫu?
formatwizard-back-body = Những thay đổi trong TOML sẽ mất: biểu mẫu viết lại định dạng từ những gì nó có.
formatwizard-back-to-form = Quay lại biểu mẫu
formatwizard-miss-name = Định dạng này không nhận tệp có tên { $file }. Hãy thêm tên đó vào Tệp.
formatwizard-miss-not-the-stores = Thứ mà nó dùng để nhận ra tệp của cửa hàng không có trong tệp này. Hãy đổi Nhận ra bằng.
formatwizard-miss-no-title = Nó không tìm thấy tiêu đề trong tệp này. Hãy chọn tiêu đề.
formatwizard-miss-short-name = không đúng tên
formatwizard-miss-short-not-the-stores = không phải của cửa hàng
formatwizard-miss-short-no-title = không có tiêu đề
formatwizard-saved-packs = Đã lưu { $file }. { $packs } gói đang được đọc lại.
formatwizard-saved = Đã lưu { $file }.
formatwizard-save-failed = Không thể lưu định dạng: { $error }
formatwizard-zip-failed = Không thể tạo tệp zip: { $error }
formatwizard-stop-title = Dừng dạy tệp này?
formatwizard-stop-body = Định dạng chưa được lưu.
formatwizard-stop = Dừng
formatwizard-label = Trình hướng dẫn định dạng
formatwizard-changing = Đang sửa { $name }
formatwizard-starting-from = Bắt đầu từ { $name }
formatwizard-title = Dạy Hoard tệp của một cửa hàng
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }trong{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Sửa dưới dạng TOML
formatwizard-file-reference = Tệp, để tham khảo.
formatwizard-click-json = Bấm vào { $field } trong tệp.
formatwizard-click-text = Bấm vào { $field } trong tệp, hoặc bôi chọn nó.
formatwizard-pick-first = Chọn một trường ở bên phải, rồi bấm vào giá trị của nó ở đây.
formatwizard-loading = Đang tải…
formatwizard-the-format = Định dạng
formatwizard-syntax-before = Cú pháp có trong{ " " }
formatwizard-syntax-after = .
formatwizard-store = Cửa hàng
formatwizard-store-placeholder = Tên cửa hàng, như Kenney
formatwizard-files = Tệp
formatwizard-recognize = Nhận ra bằng
formatwizard-recognize-json = Một khóa chỉ tệp của cửa hàng mới có
formatwizard-recognize-text = Một dòng chỉ tệp của cửa hàng mới có
formatwizard-recognize-hint = Mỗi dòng một cách; cách nào cũng được. Cần khi có tệp khác trùng tên.
formatwizard-seller = Người bán
formatwizard-seller-placeholder = Khi tệp không nói
formatwizard-tags = Thẻ
formatwizard-tags-placeholder = Cho mọi thứ nó mô tả, phân cách bằng dấu phẩy
formatwizard-fields = Trường
formatwizard-selector = Bộ chọn { $field }
formatwizard-clear = Xóa { $field }
formatwizard-pick = Chọn
formatwizard-what-it-reads = Những gì nó đọc được
formatwizard-read-title = Tiêu đề
formatwizard-read-seller = Người bán
formatwizard-read-license = Giấy phép
formatwizard-read-url = Trang cửa hàng
formatwizard-read-kind = Loại
formatwizard-read-tags = Thẻ
formatwizard-read-origin = Tạo bằng
formatwizard-read-size = Kích thước
formatwizard-also = { $format } cũng đọc tệp này. Định dạng này sẽ được thử trước.
formatwizard-library = { $files } tệp trong { $packs } gói có tên mà nó nhận. Trong { $tried } tệp đã thử, nó đọc được { $read }.
formatwizard-library-none = Không có tệp nào trong thư viện có tên mà nó nhận.
formatwizard-share = Chia sẻ
formatwizard-share-hint = Một tệp zip gồm định dạng, tệp này làm mẫu và kết quả đọc mẫu, sắp xếp giống kho mã của Hoard, để gửi đi cho mọi người cùng có. Hãy bỏ trước những gì trong mẫu không nên chia sẻ.
formatwizard-sample = Mẫu để chia sẻ
formatwizard-download = Tải xuống { $file }
formatwizard-close = Đóng
formatwizard-use = Dùng ở đây
formatwizard-cancel = Hủy
settings-unreal-editor-what = Cài đặt Unreal Editor cho tiến trình xuất. Chạy tiến trình riêng để xuất tài nguyên.
settings-unreal-download = Tải trình biên tập Linux x86-64 của Epic một lần vào thư mục công cụ của Hoard. Cần hàng chục GB dung lượng đĩa và một tài khoản GitHub đã được Epic cho phép (xem Đăng nhập GitHub).
settings-unreal-github = Đăng nhập GitHub
settings-unreal-github-why = Epic phát hành trình biên tập Linux dưới dạng image riêng tư trên kho gói của GitHub (ghcr.io), và Hoard tải nó từ đó bằng crane, công cụ đi kèm Hoard. GitHub chỉ giao nó cho các tài khoản đã được Epic cho phép, nên Hoard cần một tài khoản để tải về. Hoard giữ token cùng các cài đặt khác và không dùng nó vào việc gì khác.
settings-unreal-github-step-link = Liên kết tài khoản GitHub với tài khoản Epic Games của bạn.
settings-unreal-github-step-link-go = Cách liên kết
settings-unreal-github-step-invite = Chấp nhận lời mời vào tổ chức EpicGames mà GitHub gửi qua email cho bạn (có thể mất vài phút mới đến).
settings-unreal-github-step-token = Tạo một token GitHub (classic) chỉ đánh dấu read:packages.
settings-unreal-github-step-token-go = Tạo token
settings-unreal-github-step-enter = Nhập tên người dùng GitHub và token vào đây, rồi chọn “Tự động”.
settings-unreal-github-user = Tên người dùng GitHub
settings-unreal-github-token = Token
settings-unreal-github-token-kept = Đã lưu (nhập token mới để thay thế)
settings-unreal-github-save = Lưu đăng nhập
settings-unreal-github-forget = Quên
settings-unreal-github-saved = Đã lưu đăng nhập GitHub; đang thử tải lại
settings-unreal-github-forgotten = Đã quên đăng nhập GitHub
settings-unreal-github-from-server = Máy chủ này cũng được cấp thông tin đăng nhập riêng (GHCR_TOKEN_FILE hoặc DOCKER_CONFIG); thông tin đó được dùng khi ở đây chưa lưu gì.
settings-unreal-image = Phiên bản hoặc image Unreal
settings-editor-path = Nhập đường dẫn tệp thực thi của { $tool } đã cài đặt. Chỉ hỗ trợ cài đặt tự động trong Docker.
error-unreal-image = Nhập phiên bản Unreal 5, thẻ phiên bản Epic dev/dev-slim hoặc tham chiếu image Epic SHA-256.
