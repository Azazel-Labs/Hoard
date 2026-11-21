## 服务器错误

error-account-not-found = 未找到账户
error-asset-gone = 未找到资源
error-asset-not-found = 未找到资源 { $id }
error-blob-not-found = 未找到数据块 { $hash }
error-collection-not-found = 未找到合集 { $id }
error-connection-not-found = 未找到连接
error-derivative-not-found = 未找到 { $id } 的 { $kind } 派生文件
error-file-in-pack-not-found = 在资源包 { $id } 中未找到 { $path }
error-site-not-a-page = 只有网页才能以页面方式打开
error-site-link-expired = 此页面的地址已过期，请从 Hoard 重新打开该页面
error-format-not-found = 未找到格式 { $file }
error-frame-sheet-aseprite-only = 只有 Aseprite 文件才有帧序列图
error-frame-sheet-frames = 它只有一帧，或帧数太多无法排布
error-location-not-found = 未找到位置 { $id }
error-maintenance-task-not-found = 未找到维护任务 { $id }
error-open-upload-not-found = 未找到进行中的上传 { $id }
error-original-not-found = 未找到 { $id } 的原始文件
error-pack-not-found = 未找到资源包 { $id }
error-preview-too-big = 这张图片太大，无法生成轻量预览
error-project-not-found = 未找到项目 { $name }
error-saved-search-not-found = 未找到已保存的搜索 { $id }
error-sign-in-first = 请先登录（或发送访问令牌）
error-those-assets-not-found = 未找到这些资源
error-thumbnail-not-found = 未找到 { $id } 的缩略图
error-token-not-found = 未找到令牌
error-upload-not-found = 未找到上传 { $id }
error-pack-needs-name = 资源包需要一个名称
error-no-library = 还没有可存放文件的资源库位置：请在“设置 → 资源库位置”中添加一个托管位置
error-upload-failed = 上传失败：{ $reason }
error-upload-no-files = 上传中没有文件
error-analyze-nothing-to-do = 无事可做：rules=false 且 LLM 已关闭
error-too-many-assets = 一次请求中的资源过多
error-change-not-found = 此资源的历史中没有这项更改
error-bulk-by-look = 按外观搜索的结果无法一次全部编辑：请改为选择资源
error-too-many-items = 一次请求中的条目过多
error-not-a-hash = 不是 blake3 十六进制哈希
error-merge-into-tag = into 必须是标签
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = “{ $text }”不是标签
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } 已在 { $tag } 下，所以 { $tag } 不能放到它下面
error-nothing-to-merge = 没有可合并的内容
error-selection-size = 请提供 1 到 { $max } 个资源 ID
error-collection-needs-name = 合集需要一个名称
error-collection-exists = 已有名为 { $name } 的合集
error-collection-empty = 合集 { $name } 是空的
error-conversion-failed = 无法转换为 { $format }：{ $reason }
error-max-size = max 必须是 64 到 16384 之间的 2 的幂
error-replace-ext = ext 必须是简短的文件扩展名
error-empty-file = 空文件
error-imitation-silent = 录音中没有比背景更突出的声音。请靠近麦克风再试一次。
error-replace-duplicate = 转换后的文件与资源包中已有的文件重复；已将其合并到该文件
error-ai-address = AI 地址应以 http:// 或 https:// 开头
error-hosted-key = 请输入 { $provider } 的 API 密钥
error-hosted-address = 图像搜索服务的地址应以 http:// 或 https:// 开头
error-local-llm-action = action 可为 list、download 或 delete，后两者需要指定模型
error-role = 角色只能是 view、edit 或 admin
error-admin-only = 只有管理员才能执行此操作
error-view-only = 此账户只能查看，不能做任何更改：管理员可在“设置 → 账户”中更改
error-portal-only = 请登录门户来管理账户和令牌
error-not-from-portal = 此请求并非来自 Hoard 门户
error-account-name = 请选择不超过 64 个字符的名称
error-password-length = 请使用至少 { $min } 个字符的密码
error-too-many-codes = 错误代码次数过多：请等待 { $seconds } 秒
error-already-set-up = 已有账户：请登录
error-too-many-tries = 错误尝试次数过多：请等待 { $seconds } 秒
error-directory-unavailable = 目录服务无法验证你的密码：{ $reason }
error-password-elsewhere = 你的密码由你登录所用的目录服务或提供方保管
error-current-password = 当前密码不正确
error-account-source = 账户可通过 Hoard（internal）、目录服务（ldap）或 OAuth 登录
error-account-exists = 已有同名账户
error-own-role = 你的角色只能由其他管理员更改
error-remove-self = 登录状态下不能删除自己的账户
error-token-name = 请以使用该令牌的程序为其命名，不超过 80 个字符
error-token-role = 令牌的权限不能超过创建它的账户
error-wrong-setup-code = 这不是通过 HOARD_SETUP_CODE 提供给服务器的设置代码
error-wrong-password = 用户名和密码不匹配
error-folder-missing = { $path } 不存在（磁盘未挂载？）
error-s3-path = S3 位置的格式类似 s3://bucket
error-s3-address = S3 地址必须以 http:// 或 https:// 开头
error-s3-header = 额外的 S3 标头格式应为 Name=value，而“{ $header }”不是
error-keychain = 无法保存到系统钥匙串：{ $reason }
error-not-built = 此 Hoard 构建时未包含 { $feature }
error-openai-auth-unreachable = 无法连接到 auth.openai.com
error-openai-unexpected = OpenAI 返回了意外的响应
error-openai-answered = OpenAI 返回 { $status }：{ $reason }
error-chatgpt-id-token-not-jwt = ID 令牌不是 JWT
error-chatgpt-id-token-issuer = ID 令牌来自 { $issuer }，而不是 { $expected }
error-chatgpt-id-token-audience = ID 令牌是给其他应用的
error-chatgpt-id-token-expired = ID 令牌已过期
error-chatgpt-id-token-nonce = ID 令牌不属于本次登录
error-chatgpt-no-state = 地址中没有 state；请复制完整地址
error-chatgpt-not-started = 本次登录不是从这里发起的，或已发起超过 15 分钟。请从“设置”中重新开始。
error-chatgpt-canceled = 登录已取消
error-chatgpt-refused = ChatGPT 拒绝了登录：{ $reason }
error-chatgpt-no-code = 地址中没有登录代码；请复制完整地址
error-chatgpt-other-registration = ChatGPT 的响应针对的是此服务器的另一次注册；请重新登录
error-chatgpt-other-registration-desktop = ChatGPT 的响应针对的是另一份 Hoard；请重新登录
error-chatgpt-not-registered = ChatGPT 未完成 Hoard 的注册；请重新登录
error-chatgpt-code-used = 登录代码已被使用或已过期；请重新登录
error-chatgpt-refused-code = ChatGPT 拒绝了登录（{ $code }）
error-chatgpt-no-id-token = ChatGPT 未发送 ID 令牌
error-chatgpt-plan-not-allowed = 已登录，但未获准使用该套餐。请重新登录并允许，或改用 API 密钥。
error-chatgpt-problem = ChatGPT：{ $problem }
error-chatgpt-not-signed-in = 未登录 ChatGPT；请在“设置 → AI 标注”中登录
error-chatgpt-refresh-later = ChatGPT 暂时无法刷新登录（{ $code }）；稍后会重试
error-chatgpt-signed-out-problem = ChatGPT 已将此服务器退出登录（{ $code }）。请重新登录，以继续使用你的套餐打标签。
error-chatgpt-signed-out-problem-desktop = ChatGPT 已将 Hoard 退出登录（{ $code }）。请重新登录，以继续使用你的套餐打标签。
error-chatgpt-signed-out = ChatGPT 已将此服务器退出登录（{ $code }）；请在“设置 → AI 标注”中重新登录
error-chatgpt-signed-out-desktop = ChatGPT 已将 Hoard 退出登录（{ $code }）；请在“设置 → AI 标注”中重新登录
error-openai-api-unreachable = 无法连接到 api.openai.com
error-openai-broke-off = OpenAI 的回复中断了
error-chatgpt-kept-refusing = OpenAI 一直拒绝 ChatGPT 登录；请在“设置 → AI 标注”中重新登录
error-chatgpt-request-failed = 请求失败
error-chatgpt-cut-off = 回复被截断了（{ $reason }）
error-chatgpt-cut-off-no-reason = 回复被截断了（未给出原因）
error-openai-ended-early = OpenAI 的回复未完成就结束了
error-chatgpt-usage-limit = 你的 ChatGPT 套餐中给 Hoard 的用量暂时已用完。可在 { $url } 查看或更改限额
error-chatgpt-not-eligible = 此 ChatGPT 账户或工作区不能在其他应用中使用其套餐。请改用 API 密钥。
error-chatgpt-usage-unavailable = ChatGPT 暂时无法查询套餐用量；稍后会重试
error-chatgpt-unsupported = 使用 ChatGPT 套餐时，此模型不支持 { $param }
error-chatgpt-unsupported-images = 使用 ChatGPT 套餐时，此模型不支持 { $param }；请选择能接收图片的模型，或关闭发送缩略图
error-chatgpt-unsupported-part = 使用 ChatGPT 套餐时，此模型不支持请求中的部分内容
error-chatgpt-port = port 是浏览器访问 Hoard 所用的端口
error-chatgpt-paste-address = 请粘贴以 http://127.0.0.1 开头的完整地址
error-too-many-hashes = 一次最多 { $max } 个哈希
error-credits-pack = 资源包 { $key }
error-credits-collection = 合集 { $key }
error-format-too-long = 格式内容太长
error-not-started = 服务器尚未启动
error-not-started-desktop = Hoard 尚未启动
error-not-format-file = 不是格式文件：{ $name }
error-format-name = 名称中需要至少一个 A 到 Z 的字母或 0 到 9 的数字
error-format-miss-name = 该格式的文件不包括 { $file }
error-format-miss-store = 该格式不会把 { $file } 当作商店的文件（见 require）
error-format-miss-title = 该格式在 { $file } 中找不到标题
error-cant-download-as = .{ $ext } 文件不能下载为 { $format }
error-only-images-resize = 只有图片可以调整大小
error-https-no-certificate = 请先粘贴或上传你的证书及其密钥
error-https-no-cert-found = 未找到证书：请粘贴以 -----BEGIN CERTIFICATE----- 开头的文本
error-https-no-key-found = 未找到私钥：请粘贴以 -----BEGIN PRIVATE KEY----- 开头的文本
error-https-several-keys = 私钥不止一个：只需粘贴与证书配对的那个
error-https-key-password = 该密钥受密码保护：请提供不带密码的密钥（openssl pkey -in key.pem -out plain.pem）
error-https-cert-not-pem = 证书不是有效的 PEM
error-https-key-not-pem = 私钥不是有效的 PEM
error-https-key-mismatch = 密钥与证书不匹配。如果提供的是证书链，请把服务器证书放在最前面
error-https-unusable = 证书或密钥无法使用
error-https-expired = 此证书已于 { $date } 过期
error-https-not-yet-valid = 此证书在 { $date } 之前无效
error-https-unreadable = 这不是 Hoard 能读取的证书
error-https-bad-name = “{ $name }”不是证书可以包含的地址
error-https-test-timed-out = 测试连接超时
error-https-test-failed = 测试连接失败
error-https-no-ca = Hoard 尚未创建其证书颁发机构
error-https-unavailable = 此服务器不支持 HTTPS
error-https-certificate-mode = 选择“我的证书”时需要提供证书
error-https-test = 证书未能通过测试连接：{ $reason }
error-nothing-to-retry = 没有可以重试的项目
error-ai-unreachable = 无法连接到 { $url }
error-ai-key-refused = { $url } 拒绝了 API 密钥（{ $status }）
error-ai-answered = { $url } 返回 { $status }：{ $reason }
error-ai-not-json = { $url }/models 返回的不是 JSON
error-model-server-refused = 模型服务器：{ $reason }
error-no-model-server = { $url } 上没有模型服务器响应
error-not-llama-router = { $url } 不是处于路由模式的 llama.cpp 服务器，因此无法下载模型
error-model-download = 无法下载 { $model }：{ $reason }
error-model-delete = 无法删除 { $model }：{ $reason }
error-unknown-kind = 未知的资源类型 { $kind }
error-watched-is-folder = 监视位置是此服务器上的文件夹
error-cant-reach = 无法访问 { $path }：{ $reason }
error-folder-full-path = 请提供文件夹的完整路径
error-cant-create = 无法创建 { $path }
error-cant-write = 无法写入 { $path }
error-cant-read = 无法读取 { $path }
error-overlaps-location = 与位置 { $name }（{ $path }）重叠
error-overlaps-data = 与 Hoard 的数据文件夹（{ $path }）重叠
error-overlaps-cache = 与 Hoard 的缓存文件夹（{ $path }）重叠
error-not-watched = { $name } 不是监视文件夹
error-watched-no-kinds = 监视文件夹不接收新文件，因此没有可更改的类型
error-keep-catch-all = 请改为将另一个位置设为兜底位置
error-location-holds-files = { $name } 中仍有文件：请先将它们移到其他位置（POST /api/admin/locations/{ $id }/empty）
error-location-unreachable = 无法访问 { $name }，因此 Hoard 无法判断其中是否仍有文件
error-relocate-running = 已经在各位置之间移动文件；请等其完成后再试
error-watched-not-moved = { $name } 是监视文件夹：Hoard 不会移动其中的文件
error-no-other-location = 没有其他资源库位置可移入这些文件：请先添加一个
error-full-path = 请提供完整路径
error-pick-folder = 请选择一个文件夹
error-task-running = “{ $task }”已在运行
error-token-account-gone = 此令牌所属的账户已不存在，因此没有收藏或已保存的搜索
error-favorites-too-many = 一次最多 10,000 个
error-saved-search-name = 已保存的搜索需要一个不超过 80 个字符的名称
error-saved-search-too-long = 该搜索太长，无法保存
error-hoard-seen = 宝藏最多可记录 32 种资源，每种都须以小写字母命名
error-texture-pattern-map = 纹理命名模式需要包含 { "{" }map{ "}" }，否则同一材质的所有贴图都会得到相同的名称
error-texture-pattern-fields = 纹理命名模式可以使用 { "{" }name{ "}" }、{ "{" }map{ "}" } 和 { "{" }size{ "}" }
error-texture-name-chars = 纹理名称不能包含 < > : " / \ | ? 或 *
error-texture-map-type = 没有名为 { $role } 的贴图类型
server-role-base-color = 颜色
server-role-normal = 法线
server-role-normal-dx = 法线（DirectX）
server-role-roughness = 粗糙度
server-role-gloss = 光泽度
server-role-metalness = 金属度
server-role-ao = 环境光遮蔽
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = 高度
server-role-bump = 凹凸
server-role-cavity = 缝隙
server-role-specular = 高光
server-role-opacity = 不透明度
server-role-emissive = 自发光
server-role-translucency = 半透明
error-texture-suffix-missing = 请为{ $map }贴图指定后缀
error-texture-suffix-chars = { $map }贴图的后缀不能包含 < > : " / \ | ? 或 *
error-texture-ending-missing = 每个自定义结尾都需要填写要查找的结尾
error-texture-ending-chars = 自定义结尾不能包含 < > : " / \ | ? 或 *
error-texture-ending-twice = { $ending } 在你的结尾中出现了两次
error-oauth-no-client-id = 此链接没有 client_id。
error-oauth-unknown-app = 此应用（已）未在此处注册。请从应用中重新开始连接。
error-oauth-redirect = 该应用要求跳转回一个它未注册的地址。
error-oauth-response-type = Hoard 只发放登录代码（response_type=code）。
error-oauth-pkce = 应用必须使用 PKCE 和 S256 才能登录 Hoard。
error-light-preview-images = 只有图片才有轻量预览
error-no-light-preview = 没有轻量预览：{ $reason }
error-no-frame-sheet = 没有帧序列图：{ $reason }
error-thumbnail-source = 未知的缩略图来源：{ $source }
error-thumbnail-too-big = 该图片太大，无法用作缩略图
error-thumbnail-models-only = 只有模型可以使用由浏览器绘制的缩略图
error-not-a-picture = 不是 Hoard 能读取的图片：{ $reason }
error-project-name = 项目需要一个不超过 200 个字符的名称
error-project-too-big = 一个项目最多包含一百万个资源
error-project-too-many-at-once = 一次最多 100,000 个
error-server-name = 请选择不超过 { $max } 个字符的单行名称
error-tool-setting = { $tool }：请选择关闭、自动，或提供路径
error-ai-model = 请选择 AI 服务要使用的模型
error-public-address = 公开地址应以 http:// 或 https:// 开头
error-update-hook-address = 用于更新通知的地址应以 http:// 或 https:// 开头
error-update-hook-auth = Authorization 标头不能包含换行符或其他控制字符
error-no-soundfont = 服务器上的 { $path } 处没有 SoundFont
error-no-soundfont-desktop = { $path } 处没有 SoundFont
error-soundfont-bad = { $name } 不是 MIDI 能用来播放的 SoundFont：{ $reason }
error-workers = 后台工作线程：1 到 { $max } 之间
error-ai-batch = 每次 AI 请求的资源数：1 到 200 之间
error-conversion-cache = 转换后的下载：至少 100 MB
error-backup-keep = 定时备份：可保留 1 到 { $max } 份
error-backup-copy-to = 定时备份：{ $path } 不是服务器上的文件夹（请填写完整路径）
error-conversion-keep-days = 转换规则：原始文件保留 1 到 { $max } 天
error-conversion-target = 转换规则：Hoard 无法将文件转换为 { $format }
error-conversion-no-source = 转换规则：每条规则至少需要一种要转换的格式
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = 转换规则：.{ $ext } 文件无法转换为 { $format }
error-conversion-twice = 转换规则：.{ $ext } 出现在两条规则中
error-conversion-cant-undo = 此文件不是由规则转换的，或者其原始文件已被删除。
error-conversion-path-taken = { $path } 处现在有另一个文件，因此原始文件无法放回原处。
error-limits-zero = 限制不能为零
error-settings-object = 设置应为 JSON 对象
error-bad-setting = 设置有误：{ $reason }
error-auth-default-role = 默认角色只能是 view、edit 或 admin
error-auth-session-hours = 会话时长应在 1 小时到 1 年之间
error-auth-settings = 身份验证设置有误：{ $reason }
error-ldap-settings = LDAP 设置有误：{ $reason }
error-ldap-no-server = 请添加目录服务器的地址
error-ldap-base-dn = 请指明用户条目所在的位置（base DN）
error-ldap-username-attribute = 请指明哪个属性保存用户登录所用的名称
error-ldap-bind-dn = 请提供用于查找用户的账户，或改为匿名查找
error-ldap-ca-cert = 证书颁发机构不是 PEM 证书（-----BEGIN CERTIFICATE-----）
error-ldap-empty-server = 有一个目录服务器地址为空
error-ldap-server-form = { $server }：目录服务器地址应为 host:port、ldap://… 或 ldaps://…
error-ldap-not-starttls = { $server }：这是 LDAPS，不是 StartTLS
error-ldap-no-certificate = 其中没有证书
error-ldap-password-again = 请重新输入用于查找用户的账户密码：已保存的密码只会发送给当初提供它的服务器
error-ldap-unreachable = 无法连接到任何目录服务器（{ $reasons }）
error-ldap-no-anonymous = 目录服务不允许匿名查找用户（{ $reason }）
error-ldap-bind-failed = 用于查找用户的账户无法登录（{ $reason }）
error-ldap-search = 正在 { $base } 下查找 { $name }
error-ldap-group-search = 正在 { $base } 下查找 { $name } 所在的组
error-ldap-several = 目录中有 { $count } 个条目与 { $name } 匹配：请缩小用户筛选条件
error-ldap-password-check = 目录服务拒绝验证 { $name } 的密码（{ $reason }）
error-ldap-wrong-password = 目录中没有使用该密码的 { $name }
error-oauth-client-id = 请提供提供方为 Hoard 创建的客户端 ID
error-oauth-addresses = 授权地址和令牌地址应以 https:// 开头
error-oauth-userinfo-address = 用户信息地址应以 https:// 开头
error-oauth-username-claim = 请指明哪个 claim 保存用户的名称
error-oauth-secret-again = 请重新输入客户端密钥：已保存的密钥只会发送给当初提供它的令牌地址
error-oauth-issuer = 颁发者应以 https:// 开头
error-oauth-discovery-unreachable = 无法连接到 { $url }：{ $reason }
error-oauth-discovery-status = { $url } 返回 { $status }
error-oauth-discovery-bad = { $url } 不是发现文档
error-oauth-discovery-incomplete = { $url } 未指明授权地址和令牌地址
error-signin-no-group = { $name } 不在允许使用 Hoard 的组中
error-signin-other-source = 已有名为 { $name } 的 Hoard 账户，它使用其他方式登录
error-signin-other-subject = 已有名为 { $name } 的 Hoard 账户，属于提供方中的另一个人
error-signin-no-account = { $name } 还没有 Hoard 账户：管理员可在“设置 → 账户”中添加
error-signin-bad-name = 该名称不能用作 Hoard 账户（不超过 64 个字符）
error-oauth-off = 未开启 OAuth 登录
error-oauth-bad-authorization-address = 授权地址无效
error-oauth-refused = 提供方未让你登录：{ $reason }
error-oauth-no-code = 提供方将你送回，但未完成登录
error-oauth-expired = 该登录耗时过长或是在别处发起的：请重试
error-oauth-no-claim = 提供方未说明你是谁（没有 { $claim } claim）
error-oauth-no-sub = 提供方未说明你是谁（没有 sub 或 id claim）
error-oauth-token-unreachable = 无法连接到提供方的令牌地址
error-oauth-token-refused = 提供方拒绝完成登录：{ $reason }
error-oauth-no-access-token = 提供方未发送访问令牌
error-oauth-userinfo-unreachable = 无法连接到提供方的用户信息地址
error-oauth-userinfo-refused = 提供方拒绝说明登录者是谁（{ $status }）
error-oauth-userinfo-not-json = 提供方的用户信息不是 JSON
error-format-no-name = name 为空
error-format-no-files = files 为空：请指明该格式读取哪些文件，例如 ["*.json"]
error-format-empty-selector = 为空
error-format-unopened-bracket = 有右括号但没有对应的左括号
error-format-unclosed-bracket = 有括号未闭合
error-format-empty-step-dots = 有空的步骤（连续两个点，或开头/结尾有点）
error-format-unclosed-except = !( 未闭合
error-format-unclosed-filter = [ 未闭合
error-format-after-filters = { $step } 的键和筛选器之后出现了 { $rest }
error-format-except-needs-glob = !(…) 应跟在含有 * 的键之后
error-format-except-not-glob = !(…) 应跟在含有 * 的键之后，而不是 { $key }
error-format-empty-step = 有空的步骤
error-format-not-a-filter = [{ $filter }] 不是筛选器：请使用 []、[key=value|other]、[array]、[string]、[number]、[bool] 或 [object]
error-format-filter-needs-value = [{ $filter }] 需要一个键和一个值
error-store-search-empty = 没有可搜索的内容；请输入资源包的名称
error-update-hook-not-set = 请先设置要调用的地址
error-upload-stalled = { $path } 的上传停滞了；请重新发送该部分
error-upload-busy = { $path } 正在写入中
error-upload-incomplete = { $count } 个文件不完整，例如 { $files }
error-upload-no-files-listed = 没有文件
error-upload-not-part = { $path } 不属于上传 { $id }，或该上传已关闭
error-bad-path = 路径有误
error-upload-part-offset = { $offset } 不是 { $path } 某个分块的起始位置
error-upload-interrupted = 上传中断：{ $reason }
error-upload-part-long = { $path } 的第 { $part } 块超过 { $len } 字节
error-upload-part-short = { $path } 的第 { $part } 块：收到 { $got } 字节，应为 { $len } 字节
error-upload-file-long = { $path } 比声明的大
error-upload-file-short = { $path }：收到 { $got } 字节，应为 { $len } 字节
error-upload-complete = 上传已完成
error-already-imported = 已作为“{ $name }”导入

## 服务器文本

server-store-place-readme = README
server-store-place-folders = 文件夹
server-store-why-linked = 资源包的{ $place }中有链接指向它
server-store-why-same-title = 与资源包的{ $place }中的标题相同
server-store-why-by = 作者为 { $author }，资源包的{ $place }中提到了此人
server-store-why-meshes = 网格数量与资源包相近（{ $count }）
server-store-why-textures = 纹理数量与资源包相近（{ $count }）
server-chatgpt-revoke-failed = 已在此处退出登录，但无法通知 OpenAI。为保险起见，请在 ChatGPT 的“设置 → 安全与登录 → 使用 ChatGPT 登录”中断开 Hoard。
server-chatgpt-signed-in-title = 已登录 ChatGPT
server-chatgpt-signed-in = Hoard 已登录。AI 标注现在可以使用你的 ChatGPT 套餐了。
server-chatgpt-signed-in-as = Hoard 已以 { $email } 身份登录。AI 标注现在可以使用你的 ChatGPT 套餐了。
server-chatgpt-not-finished-title = 登录未完成
server-chatgpt-other-computer = 如果你是在另一台电脑上的 Hoard 中发起登录的，请从地址栏复制本页地址，粘贴到那台电脑上 Hoard 的“设置”中。
server-back-to-settings = 返回设置
server-issues-pack = 无法添加的资源包
server-issues-pack-blurb = 删除该资源包后重新添加，或打开它查看其中的内容。
server-issues-damaged = 已损坏或丢失的存储文件
server-issues-damaged-blurb = 由“检查文件是否损坏”发现。请从备份中恢复它们，或重新添加其资源包，然后再次运行“检查文件是否损坏”。
server-issues-folder = 无法读取的监视文件夹
server-issues-folder-blurb = 在文件夹能再次读取之前，其中的资源包保持不变。
server-issues-missing = 文件夹已消失的监视资源包
server-issues-missing-blurb = 每个资源包会保留 3 天，以防其文件夹只是暂时离线，之后将被移除。
server-issues-job = 失败的后台任务
server-issues-job-blurb = 重试会从头再次运行该任务。
server-issues-file = 无法打开的文件
server-issues-file-blurb = Hoard 无法打开或存储它们。监视资源包会在下次扫描时重试；已在资源库中的文件保留之前的内容。
server-issues-unpack = 无法解压的压缩包
server-issues-unpack-blurb = 每个压缩包都作为普通文件保留在其资源包中，因此其中的内容都不在资源库里。
server-issues-read = Hoard 无法解析的文件
server-issues-read-blurb = 它们在资源库中，但缺少读取后才能得到的信息、标签或缩略图。
server-issues-render = 缩略图渲染器处理失败的模型
server-issues-render-blurb = 它们没有缩略图。在查看器中打开可能仍然可行，并且可以保存一张缩略图。
server-issues-convert = Blender 无法转换的模型
server-issues-convert-blurb = 它们没有 GLB 副本，因此无法以 3D 显示。
server-issues-unreal = 无法转换的 Unreal 包
server-issues-unreal-blurb = 资源包的 Unreal 页面中有其余的包。
server-issues-reformat = 转换规则无法转换的文件
server-issues-reformat-blurb = 它们保持原样。
server-issues-materials = 无法识别材质的资源包
server-issues-materials-blurb = 它们的纹理都在，只是没有归入材质。
server-issues-material-thumb = 没有缩略图的材质
server-issues-material-thumb-blurb = 已找到材质，但无法绘制其预览。
server-issues-store-files = 无法读取的商店文件
server-issues-store-files-blurb = 来自资源包所属商店的文件（例如 Megascans JSON），用于为资源包命名和打标签。
server-issues-scan-again = 重新扫描
server-issues-retry = 重试
server-issues-reanalyze = 重新分析资源包
server-issues-convert-again = 重新转换
server-issues-look-again = 重新查找
server-issues-read-again = 重新读取
server-issues-blob-missing = 存储的文件已丢失。
server-issues-blob-damaged = 存储的文件与当初存储的内容不符：已损坏。
server-issues-folder-gone = 未找到文件夹“{ $folder }”。
server-job-a-pack = 一个资源包
server-job-ingest = 正在导入 { $pack }
server-job-retag = 正在重新为 { $pack } 打标签
server-job-llm-tag = 正在用 AI 为 { $pack } 打标签
server-job-convert = 正在用 Blender 转换 { $pack }
server-job-unreal = 正在转换 { $pack } 中的 Unreal 包
server-job-reformat = 正在按转换规则转换 { $pack } 中的文件
server-job-scan = 正在扫描监视文件夹
server-job-relocate = 正在资源库位置之间移动文件
server-job-looks = 正在分析颜色和外观
server-job-sounds = 正在聆听声音
server-job-embed = 正在识别图片内容
server-job-hear = 正在识别声音听起来像什么
server-job-materials = 正在查找 { $pack } 中的材质
server-job-read-files = 正在读取 { $pack } 的文件
server-job-store-lookup = 正在查找商店页面
server-maintenance-backup = 备份数据库
server-maintenance-backup-blurb = 在数据库旁的 backups 文件夹中生成一份一致的数据库副本（资源包、标签、备注、合集和设置）。资源库的文件不在其中：文件一经存储就不会再改变，因此任何文件同步工具都能备份它们。
server-maintenance-scrub = 检查文件是否损坏
server-maintenance-scrub-blurb = 读回每个存储的文件并与其指纹比对，以发现磁盘或 NAS 上已损坏或丢失的文件。资源库较大时需要一段时间。
server-maintenance-reanalyze = 重新分析所有资源包
server-maintenance-reanalyze-blurb = 用此版本的 Hoard 重新读取每个文件：重新生成缩略图、预览、详细信息和规则标签。你自己的编辑会保留；不会再次询问 AI。
server-maintenance-clear-cache = 清除缓存的转换文件
server-maintenance-clear-cache-blurb = 删除为“下载为”转换而保留的文件（下次请求时会重新生成），以及中断的工作留下的文件。
server-maintenance-shrink-thumbs = 压缩缩略图
server-maintenance-shrink-thumbs-blurb = 按照“设置 → 性能 → 缩略图”的要求重新存储以 PNG 保存的缩略图和模型视图，通常能让它们缩小好几倍。带透明度的图片和像素画仍保持 PNG。
server-maintenance-gc = 删除未使用的文件
server-maintenance-gc-blurb = 删除不再被任何内容引用的存储文件，例如已永久删除的资源包的文件。
server-update-no-address = 未设置要调用的地址
server-update-no-answer = 30 秒内没有响应

## 设置：外观

appearance-language = 语言
appearance-language-auto = 跟随浏览器（{ $name }）

## 根

root-unreachable = 无法连接到 Hoard 服务器：{ $error }
root-unreachable-desktop = Hoard 没有响应：{ $error }
root-desktop-reopen = 此窗口已退出登录。请关闭 Hoard 并重新打开以继续。
root-try-again = 重试
root-lazy-connect = 应用连接
window-minimize = 最小化
window-maximize = 最大化
window-restore = 还原
window-close = 关闭

## 应用：资源库、顶栏和筛选

app-lazy-settings = 设置
app-lazy-archive = 归档
app-lazy-shortcuts = 快捷键
app-lazy-setup = 初始设置
app-theme-system = 跟随系统主题
app-theme-light = 浅色主题
app-theme-dark = 深色主题
app-color-red = 红色
app-color-orange = 橙色
app-color-yellow = 黄色
app-color-green = 绿色
app-color-teal = 青色
app-color-blue = 蓝色
app-color-purple = 紫色
app-color-pink = 粉色
app-color-brown = 棕色
app-color-sand = 沙色
app-color-gray = 灰色
app-color-slate = 石板灰
app-color-black = 黑色
app-color-white = 白色
app-color-swatch = { $color }：含有大量{ $lower }的资源
app-color-any = 任意颜色
app-color-pick = 选择任意颜色
app-color = 颜色
app-music = 音乐
app-tempo = BPM
app-tempo-min = 从
app-tempo-max = 到
app-tempo-min-title = 此速度或更快的音乐，单位为每分钟节拍数（减半或加倍也算）
app-tempo-max-title = 此速度或更慢的音乐，单位为每分钟节拍数
app-key = 调性
app-key-any = 任意调性
app-key-major = 大调
app-key-minor = 小调
app-key-mix = 以及可与之混音的调
app-key-mix-title = 它的关系大调或小调，以及上下相差五度的调，DJ 会把它们混在一起
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = 按拼写方式写出声音（如 whooom）：形状相近的声音排在前面
app-imitation-record = 对着麦克风发出声音：形状相近的声音排在前面
app-imitation-stop = 停止录音
app-imitation-heard = 你的声音
app-imitation-mic-failed = 无法从麦克风录音：{ $error }
app-retry = 重试
app-undo = 撤销
app-search-failed = 搜索失败：{ $error }
app-unreachable = 无法连接到服务器：{ $error }
app-unreachable-desktop = Hoard 没有响应：{ $error }
app-forget-project-title = 忘记 { $name }？
app-forget-project-body = Hoard 将不再在其 { $count } 个资源的使用位置列出它，直到下次 hoard sync 再次报告。项目本身不会有任何改变。
app-forget-project-confirm = 忘记
app-forget-project-failed = 无法忘记 { $name }：{ $error }
app-forget-project-label = 忘记项目 { $name }
app-forget-project = 忘记此项目
app-save-name-like = 类似 { $name }
app-save-name-favorites = 收藏
app-search-saved = 已保存“{ $name }”：位于“筛选”中的“我的”下
app-search-updated = 已更新“{ $name }”
app-rank-alike = 最相似
app-rank-color = 该颜色最多
app-rank-match = 最佳匹配
app-rank-imitation = 听起来最像
app-sort = 排序
app-sort-name = 名称
app-sort-created = 创建时间
app-sort-updated = 更新时间
app-sort-size = 大小
app-sort-rating = 评分
app-sort-newest-first = 最新优先
app-sort-oldest-first = 最早优先
app-sort-a-to-z = A 到 Z
app-sort-z-to-a = Z 到 A
app-sort-largest-first = 最大优先
app-sort-smallest-first = 最小优先
app-sort-best-rated-first = 评分最高优先
app-sort-lowest-rated-first = 评分最低优先
app-collection-added = 已将 { $count } 个资源添加到 { $name }
app-collection-add-failed = 无法添加到合集：{ $error }
app-collection-removed = 已从 { $name } 中移除 { $count } 个
app-collection-remove-failed = 无法从合集中移除：{ $error }
app-ns-subject = 主体
app-ns-style = 风格
app-ns-theme = 主题
app-ns-material = 材质
app-ns-map = PBR 贴图
app-ns-color = 颜色
app-ns-mood = 氛围
app-ns-genre = 流派
app-ns-license = 许可证
app-ns-res = 分辨率
app-ns-duration = 时长
app-ns-poly = 面数
app-ns-format = 格式
app-ns-weight = 字重
app-ns-tech = 技术
app-ns-blender = Blender 版本
app-ns-depth = 位深
app-ns-ucs = 声音类别（UCS）
app-ns-bpm = 速度
app-ns-root = 根音
app-ns-level = 响度
app-ns-ue = Unreal 类
app-ns-engine = 引擎
app-ns-extracted = 恢复的源文件
app-ns-script = 文字系统
app-bulk-failed =
    无法更改{ $count ->
       *[other] 这 { $count } 个资源
    }：{ $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied = 已复制 { $count } 个标签
app-tags-copy-none = { $name } 没有可复制的标签
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = 还没有复制标签。在资源上按 C 即可复制它的标签。
# Pasting tags that every one of the assets already has.
app-tags-paste-had = 这 { $count } 个资源都已有这些标签
# After pasting: $count tags were added to $assets assets.
app-tags-pasted = 已为 { $assets } 个资源添加 { $count } 个标签
app-bulk-untagged = 已从 { $count } 个资源上移除 { $tag }
app-bulk-tagged = 已为 { $count } 个资源添加标签 { $tag }
app-bulk-review = 标记已审阅
app-bulk-review-title = 将所选资源标记为已看过：此后规则和 AI 不再改动它们的标签
app-bulk-unreview = 标记未审阅
app-bulk-unreview-title = 取消所选资源的已审阅标记
app-bulk-reviewed = 已将 { $count } 个资源标记为已审阅
app-bulk-unreviewed = 已取消 { $count } 个资源的已审阅标记
app-bulk-accept = 保留推测标签
app-bulk-accept-title = 将规则和 AI 为所选资源添加的所有标签保留为你自己的标签
app-bulk-accepted = 已保留 { $count } 个资源的推测标签
app-credits-failed = 无法生成致谢名单：{ $error }
app-archived-assets = 已归档 { $count } 个资源
app-archived-one = 已归档 { $name }
app-open-failed = 无法打开该资源：{ $error }
app-unnamed-pack = 资源包
app-unnamed-collection = 合集
app-unnamed-asset = 资源
app-home-version = Hoard { $version } — 返回资源库
app-home = 返回资源库
app-search-label = 搜索资源
app-search-placeholder = 搜索名称、路径、标签
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = 岩石 or 石头：二选一 · -苔藓：排除 · "干苔藓"：这些词一起出现
app-views = 视图
app-library = 资源库
app-packs = 资源包
app-packs-working = { $count } 个处理中
app-update-title = Hoard { $version } 已发布（此服务器运行的是 { $current }）。拉取新的镜像或二进制文件并重启即可；数据库会自动升级。
app-update = 更新
app-theme = 主题
app-sign-in = 登录
app-archive-title = 归档：{ $contents }
app-archive = 归档
app-settings = 设置
app-palette-title = 命令面板（{ $keys }）
app-import-folder = 导入文件夹
app-add-files = 添加文件
app-add-files-short = 添加
app-split-label = 如何导入此文件夹
app-split-before = { "" }
app-split-after = { " " }包含 { $count } 个文件夹或压缩包。
app-split-each = 每个单独导入为一个资源包（{ $count }）
app-split-one = 导入为一个资源包
app-cancel = 取消
app-filters = 筛选
app-done = 完成
app-yours = 我的
app-favorites = 收藏
app-needs-review = 待审阅
app-needs-review-title = 仅显示还没人看过的资源（可与其他筛选条件同时使用）
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = 已评
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
       *[other] { $count } 星及以上
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = 再次点击可显示所有评分
app-forget-search-label = 忘记已保存的搜索 { $name }
app-forget-search = 忘记此搜索
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = 打开资源库时使用此搜索
app-default-search-on-title = 资源库将以此搜索打开。点击可取消。
app-default-search-label = 打开资源库时使用 { $name }
app-rename-search = 重命名
app-rename-search-label = { $name } 的新名称
app-projects-title = 从此处执行 hoard sync 的项目，以及它们上次同步的内容
app-projects = 项目
app-project-synced = { $when }同步
app-project-synced-by = { $who } 于{ $when }同步
app-collections = 合集
app-no-filters = 此搜索没有可用的筛选条件。
app-kind = 类型
app-facets-more = 还有 { $count } 个
app-facets-fewer = 收起
app-searching = 正在搜索…
app-searching-live = 正在搜索
app-asset-count = { $count } 个资源
app-clear-all = 全部清除
app-save-name-label = 此搜索的名称
app-save-name-placeholder = 命名
app-save = 保存
app-save-search-title = 将此搜索保留在“筛选”中的“我的”下
app-save-search = 保存搜索
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = 更新“{ $name }”
app-update-search-title = 将当前所选内容保留在此已保存的搜索中
app-save-search-new = 另存为新搜索
app-select-title = 选择多个资源，一起打标签、分类、加入合集、下载或归档（X 或 Ctrl+单击）
app-select = 选择
app-tile-size = 图块大小
app-supporting-title = glTF 缓冲区、材质库、材质的贴图、仅由一个模型使用的材质，以及描述模型的商店文件。下载使用它们的内容时会一并带上。 精灵图集的页面，以及包中也作为单独图片存在的精灵，会与图集一起显示。 包中有多种格式的资源只显示一次，使用 Hoard 读取得最好的格式。
app-supporting-hidden = 已隐藏 { $count } 个辅助文件
app-supporting-show = 显示
app-supporting-shown = 包括辅助文件
app-supporting-hide = 隐藏
app-folders = 文件夹
app-folders-title = 按磁盘上的原样逐个文件夹浏览此包
app-folders-label = 包的文件夹
app-welcome-title = 将资源包拖放到此页面的任意位置
app-welcome-before = zip、tar 包、文件夹或零散文件都可以。Hoard 会解压它们，按哈希值将每个文件只存储一次，读取大小、时长和面数，并为所有内容打标签，这样你就能搜索诸如{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }或{ " " }
app-welcome-after = { " " }之类的内容。
app-welcome-import-folder = 导入文件夹
app-welcome-choose-files = 选择文件
app-none-similar = 目前还没有与 { $name } 很像的内容。
app-none = 没有匹配项。
app-none-query = 没有与“{ $query }”匹配的内容。
app-none-imitation = 还没有已测量的声音听起来像“{ $word }”。
app-none-heard = 还没有已测量的声音听起来像你的声音。
app-none-filters = 在这些筛选条件下没有匹配项。
app-none-query-filters = 在这些筛选条件下没有与“{ $query }”匹配的内容。
app-clear-search-filters = 清除搜索和筛选
app-loading-more = 正在加载更多
app-keys-hint = 方向键移动，Enter 打开，Space 播放声音，X 或 Ctrl+单击可多选（触屏上长按）。
app-resize-label = 调整详情面板大小
app-resize-title = 拖动调整大小 · 双击适配宽度
app-selected-label = 已选资源
app-selected = 已选 { $count } 个
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = 编辑…
app-edit-selected-title = 更改全部的标签和字段 (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
       *[other] 已选 { $count } 个结果
    }
app-pick-assets = 选择资源
app-download-zip = 下载 zip
app-credits-title = 这些资源所属资源包的 CREDITS.md：作者、许可证以及他们要求的署名文字
app-credits = 致谢
app-prefer-sources-title = Hoard 已恢复原始文件的 Unreal SoundWave 和纹理
app-prefer-sources = 使用恢复的 WAV/PNG 而非 .uasset（{ $count }）
bag-clear = 清空所选
bag-glb-heading = 模型转为 GLB
bag-glb-text = 每个模型都转为 .glb 文件，Y 轴朝上、面朝前方，纹理作为单独的文件放在模型共用的 textures 文件夹里。其他文件原样放入。
bag-glb-download = 下载 GLB zip
bag-plain-heading = 文件保持原样
bag-plain-text = 每个文件保持导入时的样子，连同所需的文件，按资源包各放一个文件夹。
bag-remove = 将 { $name } 移出所选
bag-more = 还有 { $count } 个
app-bulk-tag-label = 要为所选资源添加或移除的标签
app-bulk-tag-placeholder = 标签
app-bulk-tag-add-title = 为每个所选资源添加此标签
app-bulk-tag-add = 添加
app-bulk-tag-remove-title = 从每个所选资源上移除此标签
app-bulk-tag-remove = 移除
app-remove-from-collection = 从 { $name } 中移除
app-unfavorite = 取消收藏
app-favorite = 收藏
app-select-all = 全选 { $count } 个
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
       *[other] 选择全部 { $count } 个结果
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
       *[other] 编辑 { $count } 个资源
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
       *[other] 编辑全部 { $count } 个结果
    }
bulk-edit-close = 关闭
bulk-edit-tags = 标签
bulk-edit-tag-add = 为全部添加标签
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } / { $count }
bulk-edit-untick = 全部取消勾选
bulk-edit-untick-title = 移除所有标签，再勾选要保留的标签
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = 其中最常用的标签
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = 各不相同，保持原样
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = 保持原样
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = 已在全部中清除
bulk-edit-reset = 恢复原样
bulk-edit-all-note = 这会更改所有结果，包括尚未加载的，且无法一步撤销。
bulk-edit-save =
    { $count ->
       *[other] 保存 { $count } 项更改
    }
bulk-edit-nothing = 没有更改
bulk-edit-saved =
    { $count ->
       *[other] 已更新 { $count } 个资源
    }
app-archive-selected-title = 在从“归档”中恢复或永久删除之前，不会出现在搜索中
app-drop-items = 放下 { $count } 个项目以添加
app-drop = 放下以添加到 Hoard
app-drop-hint = 每个顶层文件或文件夹都会成为一个单独的资源包。几 GB 也没问题；连接中断时，大文件会续传。

## 命令面板

palette-group-go-to = 转到
palette-group-actions = 操作
palette-group-theme = 主题
palette-group-kinds = 类型
palette-group-packs = 资源包
palette-group-collections = 合集
palette-group-projects = 项目
palette-group-saved = 已保存的搜索
palette-group-assets = 资源
palette-hint-current = 当前
palette-library = 资源库
palette-library-keywords = 资源 网格 主页 首页 assets grid home
palette-favorites = 收藏
palette-favorites-keywords = 星标 收藏夹 喜欢 starred stars liked
palette-packs = 资源包
palette-archive = 归档
palette-archive-keywords = 回收站 垃圾桶 已删除 恢复 trash recycle bin deleted restore
palette-stats = 资源库统计
palette-stats-keywords = 统计 数字 大小 磁盘空间 数量 statistics numbers size disk space count
palette-copies = 近似副本
palette-copies-keywords = 重复 相同 相似 缩放 转换 浪费 空间 duplicates same similar resized converted wasted space dedupe
palette-search = 搜索
palette-add-files = 添加文件…
palette-add-files-keywords = 上传 导入 upload import
palette-import-folder = 导入文件夹…
palette-import-folder-keywords = 上传 资源包 upload pack
palette-select-all = 全选显示的 { $count } 个
palette-select-all-keywords = 勾选 选中 check
palette-select-several = 选择多个…
palette-select-several-keywords = 批量 多选 标签 归档 bulk multiple tag archive
palette-archive-selected = 归档所选的 { $count } 个
palette-archive-selected-keywords = 删除 移除 回收站 delete remove trash
palette-edit-selected = 编辑已选的 { $count } 个
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = 批量更改 标签 分类 许可证 评分
palette-paste-tags = 将复制的标签粘贴到所选的 { $count } 个资源
palette-clear-selection = 清除选择
palette-clear-filters = 清除筛选
palette-clear-filters-keywords = 重置 reset
palette-hide-supporting = 隐藏辅助文件
palette-show-supporting = 显示辅助文件
palette-supporting-keywords = 法线 贴图 粗糙度 遮罩 normal maps roughness masks
palette-account = 账户和访问令牌
palette-account-keywords = 密码 登录 用户 令牌 password login user token api
palette-sign-out = 退出登录（{ $name }）
palette-sign-out-keywords = 退出 注销 登出 log out logout
palette-sign-in = 登录
palette-sign-in-keywords = 登录 账户 密码 log in login account password
palette-settings = 设置
palette-settings-keywords = 标注 管理员 工具 偏好 首选项 blender ffmpeg llm ai tagging unreal admin tools preferences
palette-shortcuts = 键盘快捷键
palette-shortcuts-keywords = 帮助 按键 help keys
palette-theme-keywords = 主题 外观 模式 theme appearance mode
palette-theme-variant-keywords = 外观 颜色 appearance color
palette-light-theme = 浅色主题：{ $name }
palette-dark-theme = 深色主题：{ $name }
palette-kind-all = 所有类型（不再只显示{ $kind }）
palette-kind-only = 只显示{ $kind }
palette-title = 命令面板
palette-placeholder = 搜索资源、转到资源包、运行命令…
palette-results = 结果
palette-searching = 正在搜索…
palette-no-match = 没有与“{ $query }”匹配的内容
palette-foot-move = 移动
palette-foot-run = 运行
palette-foot-shortcuts = 所有快捷键

## 键盘快捷键

shortcuts-title = 键盘快捷键
shortcuts-close = 关闭
shortcuts-anywhere = 任意位置
shortcuts-palette = 命令面板：搜索、跳转、运行
shortcuts-search = 搜索
shortcuts-this-list = 此列表
shortcuts-escape = 关闭、清除选择或关闭详情
shortcuts-library = 资源库
shortcuts-move = 在图块之间移动（或 J 和 K）
shortcuts-open = 打开详情
shortcuts-play-sound = 播放或停止声音
shortcuts-select = 选择或取消选择
shortcuts-favorite = 加入或移出收藏
shortcuts-reviewed = 标记为已审阅或取消
shortcuts-tag = 添加标签
shortcuts-edit = 编辑所选项
shortcuts-copy-tags = 复制它的标签
shortcuts-paste-tags = 将复制的标签粘贴到所选项
shortcuts-key-click = 单击
shortcuts-select-range = 选择范围
shortcuts-select-all = 选择显示的全部内容
shortcuts-sound-player = 声音播放器
shortcuts-play-pause = 播放或暂停
shortcuts-skip = 后退或快进
shortcuts-start-end = 开头或结尾
shortcuts-image-inspector = 图像检查器
shortcuts-zoom = 放大或缩小
shortcuts-fit = 适合窗口
shortcuts-actual-pixels = 实际像素
shortcuts-tile = 3×3 平铺以检查接缝
shortcuts-background = 下一个背景
shortcuts-channel = 显示单个通道

## KindChips.vue

kinds-group = 资源类型

## TagChip.vue

tag-title-with-source = { $tag }（{ $source }）
tag-source-human = 由人工添加
tag-source-rule = 来自离线标注器
tag-source-meta = 从文件测得
tag-source-ai = AI 建议
tag-remove = 移除标签 { $tag }
tag-accept = 保留标签 { $tag }
tag-accept-title = 将此推测保留为你自己的标签
tag-reject-title = 移除它；自动标记不会再加回来
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = 新标签
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = 你是不是想找
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = 原名 { $alias }

## ActivityIndicator.vue

activity-label-idle = 后台活动：空闲
activity-label-running = 后台活动：{ $running } 个运行中
activity-label-queued = 后台活动：{ $running } 个运行中，{ $queued } 个排队中
activity-title = 后台活动
activity-unreachable = 无法连接到服务器
activity-unreachable-desktop = Hoard 没有响应
activity-empty = 没有正在运行的任务。导入、打标签、转换和 zip 下载在进行时会显示在这里。
activity-queued = 排队中
activity-seconds = { $s } 秒
activity-minutes = { $min } 分钟
activity-hours = { $h } 小时 { $min } 分钟
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } 个三角面
tile-label = { $name }，{ $category }
tile-label-detail = { $name }，{ $category }，{ $detail }
tile-play = 播放 { $name }
tile-pause = 暂停 { $name }
tile-favorite = 收藏 { $name }
tile-in-favorites = 已在收藏中
tile-add-favorite = 加入收藏（F）
tile-more-like = 更多类似 { $name } 的内容
tile-more-like-this = 更多类似内容
tile-select = 选择 { $name }

## CollectionPicker.vue

collections-add = 添加到合集
collections-menu = 合集
collections-find-label = 查找或创建合集
collections-find-placeholder = 查找合集或为新合集命名
collections-create = 创建
collections-empty = 还没有合集。输入名称即可新建。

## FolderPicker.vue

folders-list = 文件夹
folders-close = 关闭
folders-browse = 浏览…
folders-up-title = 上一级文件夹
folders-up = 上一级
folders-this-computer = 此电脑
folders-none = 这里没有文件夹。

## LocationForm.vue

locations-mode-question = Hoard 应如何处理此文件夹？
locations-mode-managed = 占为己有：Hoard 把我的资源包存放在这里
locations-mode-managed-hint = 你上传或导入的资源包会复制到这里，由 Hoard 整理妥当。这个文件夹现在归 Hoard 所有，请不要改动其中的内容。
locations-mode-watched = 看守：我的资源包已经在这里
locations-mode-watched-hint = 其中的每个文件夹都会成为一个资源包。Hoard 就地读取文件，从不更改、移动或删除它们，并会在出现新文件夹时自动收录。
locations-folder-desktop = 文件夹
locations-folder-placeholder = 资源包所在的文件夹
locations-folder-hint-managed = 如果文件夹不存在，Hoard 会创建它。在 Docker 中，这是你挂载到容器中的文件夹。
locations-folder-hint-managed-desktop = 如果文件夹不存在，Hoard 会创建它。
locations-folder-hint-watched = 在 Docker 中，请先将文件夹挂载到容器中（只读即可），然后在这里选择它。
locations-folder-hint-watched-desktop = Hoard 只读取该文件夹，不会更改其中的任何内容。
locations-name = 名称
locations-name-placeholder-managed = 资源库
locations-name-placeholder-watched = 我的资源包
locations-files = 存放在这里的文件
locations-catch-all = 同时存放其他位置都不接收的所有文件
locations-kinds-hint = 新资源包的文件会存入勾选了其类型的位置，其余的存入兜底位置。
locations-files-all = 暂时存放所有文件。以后可以在“设置”中添加更多位置，把某些类型（比如音乐）放到另一块磁盘。
locations-first-watched-note = 上传需要一个由 Hoard 管理的资源库。你可以稍后在“设置 → 资源库位置”中添加。
locations-checking = 正在检查文件夹…
locations-add = 添加位置
locations-cancel = 取消

## LocationsPanel.vue

locations-failed = { $what }：{ $error }
locations-load-failed = 无法加载资源库位置
locations-takes-and-rest = { $kinds }，以及其他所有文件
locations-takes-rest = 其他位置不接收的所有文件
locations-takes-everything = 所有文件
locations-takes-nothing = 暂无
locations-save-failed = 无法保存该位置
locations-unwatch-title = 停止监视 { $name }？
locations-unwatch-body = 其中的 { $count } 个资源包将连同其标签和备注一起从 Hoard 中移除。文件夹中的文件不受影响。
locations-unwatch-body-empty = 文件夹中的文件不受影响。
locations-unwatch = 停止监视
locations-remove-title = 移除 { $name }？
locations-remove-body = 其中的所有文件会先按类型移到其他位置；清空后该位置即被移除。不会丢失任何内容。
locations-remove-body-only = 如果其中有文件，请先添加另一个位置供文件移入。
locations-remove = 移除
locations-removed = 已移除 { $name }
locations-emptying = 正在将文件移出 { $name }；完成后将移除该位置
locations-offline-title = 无法访问 { $name }
locations-offline-body = 该位置离线时无法移出其中的文件。仍要移除吗？仍在其中的文件将显示为丢失。
locations-remove-anyway = 仍然移除
locations-remove-failed = 无法移除 { $name }
locations-rebalancing = 正在将存储的文件移到接收其类型的位置
locations-rebalance-failed = 无法移动文件
locations-scanned-never = 尚未扫描
locations-scanned-now = 刚刚扫描
locations-scanned-minutes = { $mins } 分钟前扫描
locations-scanned-hours = { $hours } 小时前扫描
locations-scanned-date = { $date }扫描
locations-scan-failed = 无法扫描 { $name }
locations-watching = 正在监视 { $name }
locations-added = 已添加 { $name }
locations-loading = 正在加载…
locations-managed-title = 由 Hoard 管理的资源库
locations-watched-title = 监视文件夹
locations-managed-hint = 上传和导入的内容会被复制到这里。每个位置接收其勾选的类型；兜底位置接收其余所有文件。
locations-watched-hint = 就地建立索引的资源包文件夹。Hoard 从不更改其中的文件，每 15 分钟检查一次新增和移除的文件夹。
locations-managed-empty = 还没有，因此上传的内容无处存放。请在下方添加一个。
locations-reachable = 可访问
locations-unreachable-now = 目前无法访问
locations-catch-all-tag = 兜底
locations-packs = { $count } 个资源包
locations-unreachable = 无法访问
locations-scanning = 正在扫描…
locations-scan = 立即扫描
locations-edit = 编辑
locations-catch-all-locked = 如需更改，请将另一个位置设为兜底位置
locations-edit-hint-before = 仅对今后添加的文件生效。要移动已存储的文件，请使用下方的
locations-edit-hint-after = 按钮。
locations-save = 保存
locations-rebalance = 移动已存储的文件以匹配
locations-rebalance-hint = 更改位置接收的类型后使用：已存储的文件会移到其类型现在对应的位置。
locations-add-title = 添加位置
locations-add-hint = 用于存放某些类型文件的另一块磁盘，或你已有的资源包文件夹。
locations-kind-question = 是哪种位置？
locations-kind-folder = 文件夹
locations-kind-folder-hint = 在挂载到容器中的磁盘或 NAS 共享上，或由 Dropbox、Google Drive、OneDrive 或 iCloud Drive 同步的文件夹。Hoard 可以在那里存放文件，也可以关注你已有的资源包。
locations-kind-folder-hint-desktop = 在这台电脑、另一块磁盘或 NAS 共享上，或由 Dropbox、Google Drive、OneDrive 或 iCloud Drive 同步的文件夹。Hoard 可以在那里存放文件，也可以关注你已有的资源包。
locations-kind-s3-hint = Amazon S3 或类似服务中的存储桶：Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage。Hoard 在那里存放文件。
locations-s3-title = S3 存储
locations-s3-hint = s3://bucket-name 位置是此服务中的一个存储桶：AWS，或任何兼容 S3 的服务（Backblaze B2、Cloudflare R2、Wasabi、MinIO、NAS 上的 Garage）。
locations-bucket = 存储桶
locations-bucket-hint = 一个已经存在的存储桶的名称。上面的密钥需要能列出、读取和写入它。
locations-s3-endpoint = 地址
locations-s3-endpoint-placeholder = AWS 留空；例如 https://s3.eu-central-003.backblazeb2.com
locations-s3-region = 区域
locations-s3-access-key = 访问密钥 ID
locations-s3-secret-key = 秘密访问密钥
locations-s3-headers = 额外标头
locations-s3-headers-placeholder = Name=value，每行一个（例如用于 Cloudflare Access）
locations-s3-kept = 已保存（输入以替换）
locations-s3-secrets = 密钥和标头会随服务器设置一起保存，且不会再显示。
locations-s3-secrets-desktop = 密钥和标头保存在这台电脑的钥匙串中，且不会再显示。
locations-s3-saved = 已保存 S3 存储
locations-s3-save-failed = 无法保存 S3 存储
locations-add-more = 添加位置…

## IssuesPanel.vue

issues-failed = { $what }：{ $error }
issues-load-failed = 无法加载问题列表
issues-started =
    { $count ->
       *[other] 已重新开始 { $count } 项。
    }
issues-retry-failed = 无法重试
issues-dismiss-failed = 无法忽略
issues-dismiss-all-title = 忽略全部 { $count } 项？
issues-dismiss-all-body = “{ $title }”将被清空。如果再次出错，会重新列出。
issues-dismiss-all = 全部忽略
issues-just-now = 刚刚
issues-minutes-ago = { $mins } 分钟前
issues-hours-ago = { $hours } 小时前
issues-loading = 正在加载…
issues-none = 没有出现任何问题。
issues-action-all = 全部{ $action }
issues-open-asset = 打开
issues-open-pack = 显示资源包
issues-open-locations = 资源库位置
issues-where-before = 在
issues-where-after = 中
issues-times = { $count } 次
issues-starting = 正在开始…
issues-dismiss-named = 忽略 { $title }
issues-dismiss = 忽略
issues-show-fewer = 收起
issues-show-all = 显示全部 { $count } 项
issues-not-listed = 还有 { $count } 项未列出。

## archive.ts

archive-anyway = 仍然归档
archive-failed = 无法归档：{ $error }
archive-undo = 撤销
archive-restore-failed = 无法恢复：{ $error }
archive-count-packs-and-assets = { $packs } 个资源包和 { $assets } 个资源
archive-count-packs = { $packs } 个资源包
archive-count-assets = { $assets } 个资源
archive-count-nothing = 无

## ArchiveView.vue

archive-load-failed = 无法加载归档：{ $error }
archive-warning-with-uses = { $uses }{ $body }
archive-restored = 已恢复 { $what }
archive-purged = 已永久删除 { $what }
archive-purge-failed = 无法删除：{ $error }
archive-purge-title = 永久删除 { $name }？
archive-purge-pack-body = 其中的 { $count } 个资源和 { $size } 的文件也会被删除，与其他资源包共用的文件除外。此操作无法撤销。
archive-purge = 永久删除
archive-purge-asset-body = 其文件将被释放，除非有其他资源共用。此操作无法撤销。
archive-empty-title = 清空归档？
archive-empty-body = { $what }将被永久删除，其文件也会被释放。此操作无法撤销。
archive-empty = 清空归档
archive-title = 归档
archive-intro = 已归档的资源包和资源不会出现在搜索、计数和标签列表中。你可以恢复它们，或将其永久删除以释放空间。
archive-loading = 正在加载…
archive-is-empty = 归档是空的
archive-is-empty-hint = 归档资源包或资源后，它会留在这里，直到你恢复或永久删除它。
archive-packs = 资源包
archive-assets = 资源
archive-asset-count = { $count } 个资源
archive-archived = { $when }归档
archive-from = 来自 { $pack }
archive-restore = 恢复
archive-show-more = 显示更多（还剩 { $count } 个）

## mine.ts

projects-favorite-add-failed = 无法加入收藏：{ $error }
projects-favorite-remove-failed = 无法移出收藏：{ $error }
projects-save-search-failed = 无法保存搜索：{ $error }
projects-forget-search-failed = 无法移除 { $name }：{ $error }
projects-change-search-failed = 无法更改 { $name }：{ $error }

## projects.ts

projects-list-and = { $items }和{ $last }
projects-list-separator = 、
projects-use = { $name }（{ $files }）
projects-use-more = 另外 { $count } 个
projects-other = 其他 { $count } 个项目
projects-usage-title-one = { $name } 使用了此内容
projects-usage-title = { $count } 个项目使用了此内容
projects-usage-body-one = 在 { $list } 中使用：其下次 hoard sync 时会将它们移除。
projects-usage-body = 在 { $list } 中使用：每个项目下次 hoard sync 时都会将它们移除。

## uploader.ts

upload-network-error = 网络错误
upload-gave-up = { $what }：尝试 { $attempts } 次后仍失败：{ $error }
upload-failed-plain = 失败
upload-already-imported = 已导入
upload-part-failed = 上传失败（{ $status }）
upload-batch = 一批 { $count } 个文件
upload-batch-failed = 批次失败（{ $status }）
upload-finish = 完成上传

## UploadQueue.vue

upload-eta-seconds = 剩余 { $s } 秒
upload-eta-minutes = 剩余 { $min } 分钟
upload-eta-hours = 剩余 { $h } 小时
upload-queue = 导入
upload-importing = 正在导入 { $count } 个资源包
upload-bytes-of = { $sent } / { $total }
upload-rate = { $size }/s
upload-waiting = 等待中
upload-unpacking = 已上传，正在解压
upload-already = 已在资源库中
upload-stopped = 已停止
upload-files-of = { $done } / { $total } 个文件
upload-resumed = 已续传，{ $size } 已存在
upload-skipped = 这些完全相同的文件已作为“{ $name }”导入过，因此已跳过。
upload-error = { $error }。你的进度已保留：可以重试，或稍后再次拖放相同的文件。
upload-retry = 重试
# $name is the pack the library has already that these files go into.
upload-update = 更新“{ $name }”
upload-copy = 作为副本导入
# $name is the pack the library has already that the upload goes into.
upload-into = 将并入资源库中已有的“{ $name }”：新文件会被添加，已更改的文件会被替换。
upload-own-pack = 单独作为一个资源包
upload-dismiss = 忽略
upload-cancel = 取消
upload-minimize = 最小化
upload-expand = 显示导入
upload-pill = 正在导入 { $count } 个 · { $percent }%
upload-pill-stopped = 已停止：{ $count }
upload-interrupted = 已中断
upload-resume = 继续
upload-resume-all = 全部继续
upload-resume-hint-folder = 再次选择同一个文件夹，或把它拖放到页面上，即可从停下的地方继续。
upload-resume-hint-file = 再次选择同一个文件，或把它拖放到页面上，即可从停下的地方继续。
upload-not-same = 这些不是相同的文件，因此没有继续任何导入。
upload-permission-denied = 浏览器未获准再次读取这些文件。请改为手动选择它们。

## PacksView.vue

packs-rename-failed = 无法重命名：{ $error }
packs-collection-deleted = 已删除合集 { $name }
packs-delete-failed = 无法删除：{ $error }
packs-status-uploading = 正在上传（已收到的文件）
packs-status-ingesting = 正在解压并打标签
packs-status-analyzing = LLM 正在打标签
packs-status-ready = 就绪
packs-status-failed = 失败
packs-retag-failed = 无法开始重新打标签：{ $error }
packs-read-files-failed = 无法读取资源包的文件：{ $error }
packs-context-load-failed = 无法加载资源包的商店详情：{ $error }
packs-undo-store-failed = 无法移除商店页面：{ $error }
packs-context-save-failed = 无法保存商店详情：{ $error }
packs-unreal-load-failed = 无法加载 Unreal 转换状态：{ $error }
packs-unreal-discard-title = 丢弃原始文件？
packs-unreal-discard-body = { $name } 中的原始 .uasset 文件在转换后将被删除。其中仅 Unreal 可用的数据（例如材质图表）将永久丢失。
packs-unreal-discard-confirm = 转换并丢弃
packs-unreal-convert-failed = 无法开始转换：{ $error }
packs-unreal-fact-converted = { $done } 个已从 Unreal 转换
packs-unreal-fact-packages = { $left } 个 Unreal 包
packs-unreal-fact-both = { $done } 个已从 Unreal 转换，{ $left } 个仍为包
packs-thumbs-canceled = 在完成 { $done } / { $total } 后取消
packs-thumbs-drew = 已绘制 { $count } 个模型
packs-thumbs-result = { $lead }：已替换 { $improved } 张缩略图。
packs-thumbs-result-failed = { $lead }：已替换 { $improved } 张缩略图，{ $failed } 张无法绘制。
packs-archived = 已归档 { $name } 及其 { $count } 个资源
packs-blobs-bad = { $count } 个文件未通过上次完整性检查。
packs-collections = 合集
packs-browse-named = 浏览 { $name }
packs-collection-name = 合集名称
packs-save = 保存
packs-cancel = 取消
packs-assets = { $count } 个资源
packs-updated = 更新于 { $date }
packs-browse = 浏览
packs-collection-download = 下载（.zip）
packs-collection-credits = 致谢
packs-collection-credits-title = 这些资源所属资源包的作者、许可证以及他们要求的署名文字
packs-rename = 重命名
packs-delete = 删除
packs-packs = 资源包
packs-empty = 还没有资源包。将 zip 或文件夹拖放到此页面的任意位置即可添加。
packs-find = 查找资源包
packs-find-placeholder = 按名称、商店、许可证或标签查找资源包
packs-sort = 排序
packs-sort-new = 最新优先
packs-sort-old = 最早优先
packs-sort-size = 最大优先
packs-sort-assets = 资源最多优先
packs-sort-count = 资源数量
packs-sort-smallest = 最小优先
packs-sort-fewest = 资源最少优先
packs-with-kind = 包含以下类型的资源包
packs-state = 状态
packs-state-any = 任意状态
packs-state-working = 处理中
packs-state-failed = 失败
packs-state-untagged = 尚未描述
packs-state-unlicensed = 未注明许可证
packs-store = 商店
packs-store-any = 任意商店
packs-store-from = 来自 { $store }
packs-store-none = 无商店页面
packs-shown-of = { $shown } / { $total }
packs-clear = 清除
packs-no-match = 没有匹配的资源包。
packs-clear-all = 清除搜索和筛选
packs-pack-name = 资源包名称
packs-thumbs-drawing = 正在此浏览器中绘制缩略图…
packs-thumbs-drawing-count = 正在此浏览器中绘制缩略图：{ $done } / { $total }…
packs-thumbs-stopped = 已停止：{ $error }
packs-thumbs-none = 这里的每个模型都已有带全部纹理的缩略图。
packs-dismiss = 忽略
packs-license = 许可证：{ $license }
packs-license-tier = 许可证：{ $license }（{ $tier }）
packs-license-not-found = 未找到
packs-from-before = 来自
packs-from-after = { "" }
packs-added = 添加于 { $date }
pack-panel-label = 关于此资源包
pack-panel-hide = 隐藏资源包详情
pack-panel-kinds = 包含内容
pack-panel-all = 所有资源包
packs-download = 下载资源包（.zip）
packs-store-details = 商店详情
packs-more-named = { $name } 的更多操作
packs-more = 更多
packs-rename-more = 重命名…
packs-retag-rules = 重新运行离线标注器
packs-retag-llm = 重新运行 LLM 标注器
packs-make-thumbs = 在此浏览器中生成缩略图
packs-make-thumbs-title = 在后台的 3D 查看器中逐个打开每个模型，如果其画面优于服务器生成的缩略图，就用它作为缩略图
packs-make-thumbs-title-desktop = 在后台的 3D 查看器中逐个打开每个模型，如果其画面优于 Hoard 生成的缩略图，就用它作为缩略图
packs-read-files = 重新读取资源包的文件
packs-credits = 下载致谢名单
packs-credits-title = CREDITS.md：资源包的作者、许可证以及其要求的署名文字
packs-unreal-open = 转换 Unreal 内容…
packs-archive = 归档资源包
packs-unreal-intro = Hoard 能自行读取 Unreal 包，并将其中的内容存储为普通文件：纹理存为 PNG 或 EXR，声音存为 WAV，材质存为 JSON。转换后的文件会取代原来的包，并保留其标签、链接和合集。
packs-unreal-converted =
    { $count ->
       *[other] 个已转换
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] 个已转换，其中 { $kept } 个保留了原始文件
    }
packs-unreal-pending =
    { $count ->
       *[other] 个尚未处理
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] 个等待 Unreal Editor 处理
    }
packs-unreal-unsupported =
    { $count ->
       *[other] 个没有通用格式
    }
packs-unreal-failed =
    { $count ->
       *[other] 个失败
    }
packs-unreal-reasons = 包未转换的原因
packs-unreal-editor-before = 网格和动画需要 Unreal Editor。请在安装了 Unreal 的机器上运行
packs-unreal-editor-command = hoard unreal-worker --editor <UnrealEditor-Cmd 的路径>
packs-unreal-editor-after = ；它会将它们导出为 FBX，并把结果上传到这里。
packs-unreal-discard = 转换后丢弃原始 .uasset 文件
packs-unreal-discard-hint = 释放空间。仅 Unreal 可用的数据（例如材质图表）将会丢失。
packs-unreal-drop = 删除没有通用格式的包
packs-unreal-drop-hint = 蓝图、关卡及其他仅引擎可用的资源。
packs-unreal-convert = 转换
packs-context-hint-before = 资源包的来源以及卖家对它的描述。LLM 标注器会在处理每个文件时读取这些信息，因此像“120 种冰与火法术”这样的商店简介能帮助它为
packs-context-hint-after = { " " }命名。
packs-store-auto = 自动找到：资源包本身提到了此页面。
packs-store-not-this = 不是这个
packs-lookup-label = 在商店中搜索
packs-lookup-placeholder = 资源包在商店中的名称
packs-lookup-searching = 正在搜索…
packs-lookup-search = 搜索商店
packs-lookup-hide = 隐藏
packs-lookup-clues = 来自资源包文件：{ $clues }。
packs-lookup-also = 还搜索了 { $queries }。
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Fab、itch.io、Kenney 或 OpenGameArt 上没有类似的内容。请尝试更少或不同的关键词。
packs-lookup-use = 使用
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = 当前
packs-review-store = 来自商店
packs-review-same = 相同
packs-review-read-page = 保存时从页面读取
packs-review-take =
    { $count ->
       *[other] 填入 { $count } 个字段
    }
packs-lookup-searched = 搜索于 { $date }。
packs-lookup-again = 重新搜索
packs-lookup-again-after = { "" }
packs-lookup-unreachable = 这次无法连接到 { $stores }。
packs-lookup-also-look = 还可以在
packs-lookup-links-and = { " " }和{ " " }
packs-lookup-browser-only = 上查找（它们只能在你的浏览器中搜索）；请将页面地址粘贴到下方。
packs-lookup-open = 在商店中查找…
packs-store-placeholder = Fab、itch.io、Humble Bundle…
packs-seller = 卖家
packs-seller-placeholder = 作者
packs-license-label = 许可证
packs-license-placeholder = Fab Standard License、CC BY 4.0…
packs-license-tier-label = 许可证级别
packs-license-tier-placeholder = 个人版、专业版…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
       *[other] { $count } 个资源有自己的许可证：改为沿用资源包的许可证
    }
packs-store-page = 商店页面
packs-store-text = 商店中的描述
packs-store-text-placeholder = 粘贴商品页面的文字，或留空以从商店页面获取。
packs-store-text-fetch = 保存时会从商店页面获取描述。
packs-readme = 在资源包中找到的 README
packs-save-retag = 保存并重新运行 LLM 标注器

## AssetDetail.vue

asset-lazy-3d-viewer = 3D 查看器
asset-lazy-text = 文本
asset-lazy-text-viewer = 文本查看器
asset-lazy-image-viewer = 图像查看器
asset-lazy-material-preview = 材质预览
asset-lazy-material-viewer = 材质查看器
asset-lazy-sprite-player = 精灵播放器
asset-lazy-sheet-view = 精灵图集查看器
asset-rel-texture = 纹理
asset-rel-buffer = 几何数据
asset-rel-material = 材质文件
asset-rel-animation = 动画
asset-rel-library = 链接文件
asset-rel-guess = 可能相关
asset-rel-sidecar = 商店详情
asset-rel-variant = 变体
asset-rel-sprite = 精灵
asset-rel-sample = 采样
asset-role-map = { $role }贴图
asset-maps-count = { $count } 张贴图
asset-needs = 依赖
asset-animations = 动画
asset-sprites = 精灵
asset-used-by = 被使用于
asset-loose-sprites = 也有单独的文件
asset-in-sheet = 所在的精灵图集
asset-other-formats = 其他格式
asset-shown-as = 在资源库中显示为
asset-conversion-failed = 转换失败
asset-as-converting = 正在转换…
asset-as-converting-percent = 正在转换… { $percent }%
asset-as-queued = 排队中…
asset-download = 下载
asset-as-label = 下载为
asset-as-original = { $format }（原始格式）
asset-as-pack-files = 包内的文件
asset-as-size = 尺寸
asset-as-full-size = 原始尺寸
asset-as-up-to = 最大 { $size } px
asset-as-hint = 选择尺寸或其他格式
asset-meta-size = 大小
asset-meta-length = 时长
asset-meta-sample-rate = 采样率
asset-meta-channels = 声道
asset-meta-mono = 单声道
asset-meta-stereo = 立体声
asset-meta-bit-depth = 位深
asset-meta-bits = { $bits } 位
asset-meta-level = 电平
asset-meta-level-value = 峰值 { $peak } dBFS，RMS { $rms } dBFS
asset-meta-level-clips = 峰值 { $peak } dBFS，RMS { $rms } dBFS，有削波
asset-meta-loop = 循环
asset-meta-loop-value = { $start } 至 { $end }
asset-meta-playback = 播放方式
asset-meta-one-shot = 单次
asset-meta-envelope = 包络
asset-meta-envelope-times = 起音 { $attack }，尾音 { $tail }
asset-meta-envelope-hit = 打击
asset-meta-envelope-swell = 渐强
asset-meta-envelope-sustained = 持续
asset-meta-tone = 音色
asset-meta-tone-value = 亮度 { $khz } kHz，噪声 { $noise }%
asset-meta-pitch = 音高
asset-meta-pitch-rises = 上升 { $octaves } 个八度
asset-meta-pitch-falls = 下降 { $octaves } 个八度
asset-meta-loops-cleanly = 可无缝循环
asset-meta-tempo = 速度
asset-meta-tempo-heard = 约 { $bpm } BPM，据音频测得
asset-meta-tempo-heard-or = 约 { $bpm } BPM（或 { $alt }），据音频测得
asset-meta-key = 调性
asset-meta-key-value = { $key }（{ $camelot }）
asset-meta-key-heard = { $key }，据音频测得
music-key-major = { $note } 大调
music-key-minor = { $note } 小调
asset-meta-root-note = 根音
asset-meta-cue-markers = 提示标记
asset-meta-library = 音效库
asset-meta-keywords = 关键词
asset-meta-title = 标题
asset-meta-artist = 艺术家
asset-meta-album = 专辑
asset-meta-genre = 流派
asset-meta-generator = 生成工具
asset-meta-everything-with = 显示所有带有标签 { $tag } 的内容
asset-meta-saved-with = 保存工具
asset-meta-color-mode = 颜色模式
asset-meta-layers = 图层
asset-meta-range = 范围
asset-meta-hdr = HDR（预览已经过色调映射）
asset-meta-format = 格式
asset-meta-large-document = 大型文档（PSB）
asset-meta-ai = AI
asset-meta-ai-made = 使用生成式 AI 制作
asset-meta-ai-edited = 使用生成式 AI 编辑
asset-meta-ai-how-metadata = 元数据
asset-meta-ai-how-generator = 生成器设置
asset-meta-ai-how-iptc = IPTC 数字来源类型
asset-meta-ai-says = { $made }（依据其{ $how }）
asset-meta-ai-by-says = { $made }（{ $by }，依据其{ $how }）
asset-meta-provenance = 来源证明
asset-meta-c2pa = 包含 C2PA 内容凭证
asset-meta-compression = 压缩
asset-meta-note = 备注
asset-meta-maps = 贴图
asset-meta-sizes = 尺寸
asset-meta-triangles = 三角面
asset-meta-vertices = 顶点
asset-meta-meshes = 网格
asset-meta-materials = 材质
asset-meta-animations = 动画
asset-meta-skins = 蒙皮
asset-meta-unreal-asset = Unreal 资源
asset-meta-unreal-saved-with = { $class }，由 UE { $engine } 保存
asset-meta-package = 包
asset-meta-cooked = 已烘焙（游戏构建）：预览和源数据已被剥离
asset-meta-tiling = 平铺
asset-meta-seamless = 两个方向均无缝
asset-meta-analysis-error = 无法完整读取此文件：{ $error }
asset-copy-blocked = 无法复制：浏览器阻止了此操作
asset-save-error = 无法保存：{ $error }
asset-tag-removed = 已移除 { $tag }
asset-tag-restored = 已恢复 { $tag }
asset-tags-added = 已添加 { $count } 个标签
asset-tags-accepted = 已保留 { $count } 个推测标签
asset-reviewed = 已审阅
asset-reviewed-on-title = 已审阅：规则和 AI 不再改动它的标签。点击取消标记（R）
asset-reviewed-off-title = 标记为已审阅，这样规则和 AI 就不会改动它的标签（R）
asset-reviewed-on = 已标记为已审阅
asset-reviewed-off = 已取消已审阅标记
# $name is an asset's name.
asset-reviewed-on-named = 已将 { $name } 标记为已审阅
asset-reviewed-off-named = 已取消 { $name } 的已审阅标记
asset-undo = 撤销
asset-play-error = 无法播放此文件：{ $error }
asset-copied-hash = 已复制哈希
asset-copied-toml = 已复制 toml
asset-copied-id = 已复制 ID
asset-copied-link = 已复制链接
asset-origin-before = 此文件原为
asset-origin-class-engine = ，是由 UE { $engine } 保存的 { $class }。
asset-origin-class = ，是一个 { $class }。
asset-origin-engine = ，由 UE { $engine } 保存。
asset-origin-end = 。
asset-origin-editor = 由 Unreal Editor 于 { $date } 导出。
asset-origin-editor-notes = 由 Unreal Editor 于 { $date } 导出（{ $notes }）。
asset-origin-hoard = 由 Hoard 于 { $date } 从包中读出。
asset-origin-hoard-notes = 由 Hoard 于 { $date } 从包中读出（{ $notes }）。
asset-origin-download = 下载原始 .{ $ext }
asset-origin-discarded = 原始包已在转换后丢弃。
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = 历史记录
history-loading = 正在加载…
history-none = 这里还没有任何更改。
history-load-failed = 无法加载历史记录：{ $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = AI
history-by-rule = 规则
history-by-someone = 某人
history-by-app = { $who }（通过应用）
history-by-assistant = { $who }（通过助手）
history-field-description = 描述
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = 添加了标签 { $tag }
history-tag-removed = 移除了标签 { $tag }
history-tag-kept = 保留了建议的标签 { $tag }
history-tag-changed = 更改了标签 { $tag }
history-reviewed = 标记为已审核
history-unreviewed = 标记为未审核
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }：设为 { $to }
history-cleared = { $field }：已清空（原为 { $from }）
history-changed = { $field }：{ $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = 撤销
history-revert-title = 恢复此更改所改动的内容
history-reverted = 已撤销：{ $change }
history-revert-failed = 无法撤销：{ $error }
asset-details = 资源详情
asset-in-pack = 来自
asset-previous = 上一个资源
asset-next = 下一个资源
asset-favorite = 收藏
asset-favorite-on-title = 已在收藏中（按 F 移出）
asset-favorite-off-title = 加入收藏（F）
asset-share = 分享
asset-share-title = 分享此资源的链接（打开的人需先登录 Hoard）
asset-layout-panel = 在侧边面板中显示
asset-layout-page = 整页显示
asset-layout-page-title = 整页显示：大预览图与详情并排
asset-close = 关闭详情
asset-rename-title = 单击以重命名
asset-name = 名称
asset-views-alt = 从四个方向看到的模型
asset-inspect-aria = 全屏检查 { $name }
asset-inspect-title = 检查：缩放、平铺、通道
asset-playback-position = 播放位置
asset-playback-value = { $current } / { $duration }
asset-pause = 暂停
asset-play = 播放
asset-waveform-drawing = 正在绘制波形…
asset-waveform-skipped = 文件过大，不显示波形
asset-play-animation = 作为动画播放
asset-show-views = 显示标注器看到的四视图
asset-palette = 图片中的颜色
asset-swatch-title = { $hex }，占图片的 { $percent }%。查找含有此颜色的资源
asset-swatch-aria = 查找含有 { $hex } 的资源（占此图片的 { $percent }%）
asset-more-like-this = 更多类似内容
asset-desc-label = 你的描述
asset-desc-placeholder = 它是什么，你会在哪里用到它…
asset-save = 保存
asset-cancel = 取消
asset-desc-family = 保存到全部 { $count } 个变体
asset-desc-edit = 编辑描述
asset-desc-write = 撰写描述
asset-desc-ai = 由 AI 撰写
asset-desc-file = 来自文件
asset-desc-store-before = 来自
asset-desc-store-page = { $store } 页面
asset-desc-store-page-unnamed = 商店页面
asset-desc-store-after = （整个资源包）
asset-less = 收起
asset-more = 更多
asset-tags = 标签
asset-tag-add-label = 添加标签
asset-tag-add-placeholder = 添加标签，例如 mood:tense
asset-apply-family = 将更改应用到全部 { $count } 个变体
asset-tags-accept-all = 保留全部 { $count } 个推测
asset-tags-accept-all-title = 将规则和 AI 推测的所有标签保留为你自己的标签
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = 已拒绝：
asset-tag-restore-title = 恢复 { $tag }
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = 相似的有：
asset-tag-suggest-title = 添加 { $tag }
asset-details-heading = 详情
asset-field-category = 类别
asset-field-license = 许可证
asset-field-source = 来源
asset-field-rating = 评分
# Shown in place of an empty value, to fill it in.
asset-field-add = 添加
asset-field-new = 添加字段
asset-field-name = 字段名称
asset-field-value = 值
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = 来自资源包
asset-field-license-placeholder = 例如 CC0 或 CC-BY 4.0
asset-field-remove-placeholder = 留空则移除
# $field is a field's name, like "License".
asset-field-edit-title = 编辑 { $field }
asset-field-saved = 已保存 { $field }
asset-field-cleared = 已清除 { $field }
asset-rate = { $count } 星
asset-rated = 已评 { $count } 星
asset-rating-clear-title = 再次点击可清除评分
asset-rating-cleared = 已清除评分
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
       *[other] 再次点击可改为 { $count } 星
    }
asset-collections = 合集
asset-collection-remove = 从 { $name } 中移除
asset-collection-add-another = 添加到其他合集
asset-collection-add = 添加到合集
asset-used-in = 使用于
asset-used-in-title = 显示 { $name } 包含的内容
asset-used-in-synced = { $when }同步
asset-maps = 贴图
asset-show-more = 再显示 { $count } 个
asset-materials-heading = 材质
asset-material-maps-aria = { $name }：贴图
asset-variants = 变体（{ $count } 个版本）
asset-fit-animations = 适用的动画 ({ $count })
asset-fit-characters = 适用的角色 ({ $count })
asset-copies = 完全相同的副本
asset-copies-note = 相同的字节只存储一次；这些资源包中也包含此文件。
asset-near-copies = 近似副本
asset-near-copies-note = 只是尺寸、采样率或格式不同的同一内容。最佳副本已标出。
asset-near-copies-best = 只是尺寸、采样率或格式不同的同一内容。这个就是最佳副本。
asset-near-best = 最佳
asset-unreal-converted = 从 Unreal 转换
asset-rule-converted = 由规则转换
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = 转换规则于 { $date } 生成了此 { $format }，来源：
asset-rule-kept-until = 原始文件保留至 { $date }。
asset-rule-rebuilt = FLAC 保留了 WAV 中的所有内容，因此随时可以原样下载或恢复 WAV。
asset-rule-put-back = 恢复原始文件
asset-rule-put-back-title = 恢复 { $file }？
asset-rule-put-back-body = 转换后的文件会被删除，原始文件连同其标签、合集和链接回到原位。此后转换规则不会再处理它。
asset-rule-put-back-done = 已恢复 { $file }
asset-rule-let-go = 原始文件已在“设置”中设定的保留期限到期后删除。
asset-unreal-package = 来自 Unreal 包
asset-ue-needs-editor-before = 等待 Unreal Editor：{ $reason }。请在安装了 Unreal 的机器上运行
asset-ue-needs-editor-after = ，它会以 FBX 格式返回。
asset-ue-editor-path = UnrealEditor-Cmd 的路径
asset-ue-unsupported = 保留为包：{ $reason }。
asset-ue-failed = 转换失败：{ $reason }
asset-recovered = 原始 .{ $ext } 文件存储在此包中，Hoard 已将其提取出来。
asset-recovered-download = 下载 .{ $ext }
asset-ue-meshes-before = 网格和动画以 Unreal 自己的格式存储。请先转换资源包（资源包页面 → 更多），然后在安装了 Unreal Editor 的机器上运行
asset-ue-meshes-after = 以获取 FBX。
asset-ue-registry-tags = { $count } 个资源注册表标签
asset-file = 文件
asset-bytes = 字节
asset-source = 来源
asset-path = 路径
asset-path-open-folder = 查看 { $folder } 中的其他内容
asset-download-maps = 下载全部 { $count } 张贴图
asset-download-linked = 连同 { $count } 个链接文件一起下载
asset-download-only-title = 只下载此文件，不包括它使用的文件
asset-download-only = 仅此文件
asset-include-animations = 在 .zip 中包含动画
asset-include-character = 包含角色 { $name }
asset-copy-toml = 复制 hoard.toml 条目
asset-copy-id = 复制 ID
asset-teach = 让 Hoard 学会读取此文件
asset-teach-title = 描述此商店文件，以便 Hoard 读取它（设置 → 商店文件）
asset-archive = 归档
asset-archive-title = 在从“归档”中恢复或永久删除之前，不会出现在搜索中

## ImageViewer.vue

image-channel-red = 红色通道
image-channel-green = 绿色通道
image-channel-blue = 蓝色通道
image-channel-alpha = Alpha 通道
image-channel-title = { $channel }（{ $key }）
image-bg-checker-title = 棋盘格背景（按 K 切换）
image-bg-black-title = 黑色背景（按 K 切换）
image-bg-gray-title = 灰色背景（按 K 切换）
image-bg-white-title = 白色背景（按 K 切换）
image-bg-checker = 棋盘格背景
image-bg-black = 黑色背景
image-bg-gray = 灰色背景
image-bg-white = 白色背景
image-inspect-aria = 检查 { $name }
image-preview-size = 预览 { $width }×{ $height }，{ $bytes }
image-preview = 预览
image-resolution = 分辨率
image-full-title = 加载完整的 { $width }×{ $height } 图片（F）；放大超过预览尺寸时也会加载
image-full-loading = 正在加载完整分辨率…
image-full-failed = 完整分辨率未能加载：点击重试
image-full = 完整分辨率
image-zoom = 缩放
image-zoom-out-title = 缩小（−）
image-zoom-out = 缩小
image-zoom-in-title = 放大（+）
image-zoom-in = 放大
image-fit-title = 适合窗口（0）
image-fit = 适合
image-actual-title = 实际像素（1）
image-view = 视图
image-tile-title = 3×3 平铺以检查接缝（T）
image-tile = 平铺
image-channels = 通道
image-close = 关闭（Esc）
image-failed = 无法加载此图片。
image-loading = 正在加载…
image-key-gestures = 滚动或双指缩放 · 拖动平移 · 双击放大或适合窗口
image-key-tile = 平铺
image-key-background = 背景
image-key-full = 完整分辨率
image-key-channels = 通道

## SpritePlayer.vue

sprite-frame-aria = { $name }，第 { $frame } 帧，共 { $count } 帧
sprite-failed = 无法播放此文件。
sprite-loading = 正在加载帧…
sprite-previous = 上一帧
sprite-next = 下一帧
sprite-pause = 暂停动画
sprite-play = 播放动画
sprite-animation = 动画
sprite-all-frames = 所有帧
sprite-grid-title = 如何将图集切分为帧
sprite-columns = 列数
sprite-rows = 行数
sprite-fps-aria = 每秒帧数
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
       *[other] { $count } 个精灵
    }
sheet-pages =
    { $count ->
       *[other] { $count } 页
    }
sheet-animations =
    { $count ->
       *[other] { $count } 个动画
    }
# The box that narrows the list of sprites by name.
sheet-find = 查找精灵
# Under the list of sprites when more match than it draws.
sheet-showing = 显示 { $shown } / { $total } 个
sheet-no-match = 没有名称中包含此内容的精灵。
sheet-failed = 无法读取此图集。
sheet-loading = 正在加载图集…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Hoard 自有格式：各页保持原样，精灵可任意旋转并裁去透明边，包含动画和每帧时长。
sheet-note-texturepacker = 精灵顺时针旋转并裁去透明边，动画按名称保存，可供 PixiJS 和 Phaser 读取。
sheet-note-clockwise = 精灵顺时针旋转并裁去透明边。此格式无法保存动画：帧按编号名称区分。
sheet-note-libgdx = 精灵逆时针旋转并裁去透明边。此格式无法保存动画：帧按编号名称区分。
sheet-note-godot-frames = 精灵从不旋转，裁去的部分保留为边距，包含动画和每帧时长。
sheet-note-godot-tileset = 图块从不旋转或裁剪，按图块大小排在网格上；图像若已排成网格则保持不变。
sheet-note-grid = 每个精灵都以完整画布直立放置，单元格大小一致，适用于任何按网格切分图集的工具。
sheet-note-frames = 每个精灵都以完整画布直立放置，各自保存为以其名称命名的 PNG。

## TextViewer.vue, TextPreview.vue and text.ts

text-lang-config = 配置
text-lang-table = 表格
text-lang-code = 代码
text-lang-script = 脚本
text-lang-text = 文本
text-copy-blocked = 无法复制：浏览器阻止了此操作
text-read-aria = 阅读 { $name }
text-lines = { $count } 行
text-show-as = 显示方式
text-tree-title = 可折叠树（T）
text-tree = 树
text-pretty-title = 重新缩进（P）
text-pretty = 美化
text-raw-title = 原样显示（R）
text-raw = 原始
text-document-title = 以文档形式（D）
text-document = 文档
text-source-title = 原样显示的 Markdown（R）
text-source = 源码
text-page-title = 如同浏览器从其所在文件夹打开时的样子，与 Hoard 隔离 (P)
text-page = 页面
text-page-source-title = 页面的原始写法 (R)
text-page-aria = { $name } 的页面视图
text-page-error = 无法打开此页面：{ $error }
text-fold = 折叠
text-expand-all-title = 展开全部
text-expand-all = 全部展开
text-collapse-all-title = 折叠全部
text-collapse-all = 全部折叠
text-group-text = 文本
text-wrap-title = 自动换行（W）
text-wrap = 换行
text-copy-title = 复制文本（C）
text-copied = 已复制
text-copy = 复制
text-download-title = 下载文件
text-download = 下载
text-close = 关闭（Esc）
text-truncated = 显示前 { $shown }，共 { $total }。
text-truncated-json = 显示前 { $shown }，共 { $total }（树状视图需要完整文件）。
text-show-all = 全部显示
text-json-error = 此 JSON 无法解析：{ $error }
text-contents-aria = { $name } 的内容
text-loading = 正在加载…
text-load-error = 无法加载此文件：{ $error }
text-binary = 此文件是二进制文件而非文本，因此这里没有可阅读的内容。请下载后用相应的工具打开。
text-key-tree = 树
text-key-pretty = 美化
text-key-raw = 原始
text-key-document = 文档
text-key-source = 源码
text-key-page = 页面
text-key-wrap = 换行
text-key-copy = 复制
text-key-find = 查找
text-key-close = 关闭
text-preview-aria = 全屏阅读 { $name }
text-preview-read = 阅读

## JsonTree.vue and JsonPicker.vue

json-items = { $count } 项
json-keys = { $count } 个键
json-fold = 折叠
json-unfold = 展开
json-unfold-summary = 展开 { $summary }
json-show-more = 再显示 { $count } 个
json-left = （还剩 { $count } 个）
json-picker-more = … 还有 { $count } 个

## 3D 模型查看器

model-viewer-failed = 3D 查看器未能加载（{ $error }）；重新加载页面可能会有帮助
model-loading-viewer = 正在加载 3D 查看器…
model-loading = 正在加载模型…
model-show-failed = 无法显示此模型：{ $error }
model-preview-failed = 无法预览此模型：{ $error }
model-error-converting = 服务器仍在转换此模型
model-error-converting-desktop = Hoard 仍在转换此模型
model-error-empty-collada = Collada 文件为空
model-error-no-loader = 浏览器没有 .{ $ext } 的加载器
model-dialog = 以 3D 查看 { $name }
model-title-stats = { $triangles } 个三角面 · { $meshes } 个网格 · { $materials } 个材质
model-triangles = { $count } 个三角面
model-group-camera = 相机
model-group-shading = 着色
model-group-background = 背景
model-group-animation = 动画
model-group-view = 视图
model-group-show = 显示
model-frame-title = 框选模型（F）
model-frame = 框选
model-reset-title = 回到初始视图（R）
model-reset = 重置
model-turn-title = 缓慢旋转（A）
model-turn = 旋转
model-textures-title = 纹理（T）
model-textures = 纹理
model-full-res-title = 全分辨率纹理（H）：按原样使用每个纹理文件，无论多大；关闭时使用最大 2048 px 的轻量预览
model-full-res = 全分辨率
model-light-title = 环境光照（E）
model-light = 光照
model-grid-title = 地面网格（G）
model-grid = 网格
model-bg-checker = 棋盘格背景
model-bg-dark = 深色背景
model-bg-gray = 灰色背景
model-bg-light = 浅色背景
model-bg-env = 环境背景
model-bg-title = { $background }（按 K 切换）
model-pause-title = 暂停（Space）
model-play-title = 播放（Space）
model-pause = 暂停
model-play = 播放
model-animation-clip = 动画片段
model-animation = 动画
model-plays-on = 播放于角色
model-rest-pose = 静止姿势
model-close-title = 关闭（Esc）
model-full-screen = 全屏
model-open-full-screen = 打开全屏 3D 查看器
model-menu-fold = 收起菜单
model-menu-show = 显示菜单
model-view-prefix = 视图：
model-menu-render = 渲染
model-menu-maps = 贴图
model-menu-inspect = 检查
model-mode-final = 最终渲染
model-mode-raw = 原始渲染
model-mode-color = 基础色
model-mode-roughness = 粗糙度
model-mode-metalness = 金属度
model-mode-normal = 法线
model-mode-ao = 遮蔽
model-mode-matcap = Matcap
model-mode-uv = UV 棋盘格
model-mode-geometry = 几何体
model-mode-title = { $mode }（{ $key }）
model-wireframe-title = 线框（W），可叠加在任意视图上
model-wireframe = 线框
model-info-title = 场景信息（I）
model-info = 场景信息
model-info-close = 关闭场景信息
model-info-close-title = 关闭（I）
model-info-geometries = 几何体
model-info-materials = 材质
model-info-textures = 纹理
model-info-triangles = 三角面
model-info-vertices = 顶点
model-info-meters = glTF 以米为单位
model-info-no-units = 此格式未说明其单位
model-info-height = 高度
model-info-length = 长度
model-info-width = 宽度
model-length-units = { NUMBER($n, maximumSignificantDigits: 3) } 单位
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = 未找到纹理：{ $name }
model-missing-some = 未找到 { $count } 个纹理：{ $names }
model-missing-more = 未找到 { $count } 个纹理：{ $names }…
model-missing-count = 未找到 { $count } 个纹理
model-guessed = 纹理是按名称匹配的；文件中未指明任何纹理。
model-group-facing = 朝向
model-facing-turn = 旋转 90°
model-facing-turn-title = 从上方看，将模型逆时针旋转 90°
model-facing-tip = 前倾 90°
model-facing-tip-title = 将模型顶部向前倾倒 90°，让躺着的模型立起来
model-facing-save = 用于此资源包
model-facing-save-title = 将 { $pack } 中的所有模型都这样旋转。它们的缩略图会重新绘制；下载内容保持生成时的样子。
model-facing-undo = 撤销
model-facing-undo-title = 恢复资源包中模型原来的朝向
model-facing-saved = { $pack } 中的所有模型现在都是这个朝向。正在重新绘制它们的缩略图。
model-facing-failed = 无法旋转资源包的模型：{ $error }
model-keys-mouse = 拖动环绕 · 右键拖动平移 · 滚动缩放 · 双击框选
model-key-frame = 框选
model-key-reset = 重置
model-key-view = 视图
model-key-wireframe = 线框
model-key-info = 场景信息
model-key-textures = 纹理
model-key-full-res = 全分辨率纹理
model-key-light = 光照
model-key-grid = 网格
model-key-background = 背景
model-key-play = 播放

## 材质

material-role-base-color = 颜色
material-role-normal = 法线
material-role-normal-dx = 法线（DirectX）
material-role-roughness = 粗糙度
material-role-gloss = 光泽度
material-role-metalness = 金属度
material-role-ao = 环境光遮蔽
material-role-orm = 遮蔽、粗糙度、金属度
material-role-mra = 金属度、粗糙度、遮蔽
material-role-rma = 粗糙度、金属度、遮蔽
material-role-height = 高度
material-role-bump = 凹凸
material-role-cavity = 缝隙
material-role-specular = 高光
material-role-opacity = 不透明度
material-role-emissive = 自发光
material-role-translucency = 半透明
material-role-base-color-name = 颜色
material-role-normal-name = 法线
material-role-normal-dx-name = 法线（DirectX）
material-role-roughness-name = 粗糙度
material-role-gloss-name = 光泽度
material-role-metalness-name = 金属度
material-role-ao-name = 环境光遮蔽
material-role-orm-name = 遮蔽、粗糙度、金属度
material-role-mra-name = 金属度、粗糙度、遮蔽
material-role-rma-name = 粗糙度、金属度、遮蔽
material-role-height-name = 高度
material-role-bump-name = 凹凸
material-role-cavity-name = 缝隙
material-role-specular-name = 高光
material-role-opacity-name = 不透明度
material-role-emissive-name = 自发光
material-role-translucency-name = 半透明
material-maps = { $count } 张贴图
material-dialog = 查看材质 { $name }
material-up-to = 最大 { $size }
material-repeat-size = 每次重复 { $size }
material-shape-sphere = 球体
material-shape-cube = 立方体
material-shape-plane = 平面
material-shape-title = { $shape }（{ $key }）
material-group-shape = 形状
material-group-tiling = 平铺
material-real-title = 真实尺寸：每次重复为 { $size }，球体直径 2 m（按 T 切换）
material-repeat-title = 重复 { $n }×（按 T 切换）
material-real = 真实
material-repeat = { $n }×
material-height-title = 高度贴图将表面推出的程度
material-height = 高度
material-height-strength = 高度强度
material-loading = 正在加载材质…
material-show-failed = 无法显示此材质：{ $error }
material-preview-failed = 无法预览此材质：{ $error }
material-group-maps = 贴图
material-map-show = 显示{ $map }贴图
material-map-hide = 隐藏{ $map }贴图
material-map-unused = 预览中未使用
material-open-full-screen = 打开全屏材质查看器
material-keys-mouse = 拖动环绕 · 右键拖动平移 · 滚动缩放 · 单击贴图将其关闭
material-key-shape = 形状
material-key-tiling = 平铺
material-key-turn = 旋转

## theme.ts

theme-slate = 石板
theme-paper = 纸张
theme-linen = 亚麻
theme-sage = 鼠尾草
theme-lilac = 丁香
theme-blush = 绯红
theme-graphite = 石墨
theme-obsidian = 黑曜石
theme-ember = 余烬
theme-moss = 苔藓
theme-aubergine = 茄紫
theme-oxblood = 牛血红
theme-light-slate-blurb = 冷调蓝灰
theme-light-paper-blurb = 素净中性灰
theme-light-linen-blurb = 暖调奶油色，琥珀点缀
theme-light-sage-blurb = 柔和绿色
theme-light-lilac-blurb = 淡紫色
theme-light-blush-blurb = 灰粉玫瑰色
theme-dark-graphite-blurb = 中性灰，无色调
theme-dark-slate-blurb = 蓝灰
theme-dark-obsidian-blurb = 近乎纯黑，适合 OLED
theme-dark-ember-blurb = 暖调炭灰，琥珀点缀
theme-dark-moss-blurb = 森林绿黑
theme-dark-aubergine-blurb = 梅紫黑，丁香点缀
theme-dark-oxblood-blurb = 酒红黑，珊瑚点缀

## texnames.ts

texnames-needs-map = 命名模式需要包含 { "{" }map{ "}" }，否则同一材质的所有贴图都会得到相同的名称
texnames-placeholders = 命名模式可以使用 { "{" }name{ "}" }、{ "{" }map{ "}" } 和 { "{" }size{ "}" }
texnames-bad-characters = 名称不能包含 < > : " / \ | ? 或 *
texnames-no-suffix = 请为{ $role }贴图指定后缀
texnames-bad-suffix = { $role }贴图的后缀不能包含 < > : " / \ | ? 或 *
texnames-slot-no-suffix = 请为“{ $slot }”指定后缀
texnames-slot-bad-suffix = “{ $slot }”的后缀不能包含 < > : " / \ | ? 或 *
texnames-ending-missing = 每个自定义结尾都需要填写要查找的结尾
texnames-ending-bad = 自定义结尾不能包含 < > : " / \ | ? 或 *
texnames-ending-twice = { $ending } 在你的结尾中出现了两次
texnames-slot-metallic-roughness = 金属度和粗糙度
texnames-slot-specular-color = 高光颜色
texnames-slot-specular-glossiness = 高光和光泽度
texnames-slot-clearcoat = 清漆
texnames-slot-clearcoat-roughness = 清漆粗糙度
texnames-slot-clearcoat-normal = 清漆法线
texnames-slot-sheen-color = 绒光颜色
texnames-slot-sheen-roughness = 绒光粗糙度
texnames-slot-transmission = 透射
texnames-slot-thickness = 厚度
texnames-slot-diffuse-transmission = 漫透射
texnames-slot-diffuse-transmission-color = 漫透射颜色
texnames-slot-iridescence = 虹彩
texnames-slot-iridescence-thickness = 虹彩厚度
texnames-slot-anisotropy = 各向异性

## storeFormats.ts

storefiles-field-title = 标题
storefiles-field-title-hint = 商品的确切标题。它用于命名资源包，缺少它时会跳过该文件。
storefiles-field-seller = 卖家
storefiles-field-seller-hint = 制作或销售它的人。
storefiles-field-license = 许可证
storefiles-field-license-hint = 按文件中的写法，例如 CC-BY-4.0。
storefiles-field-url = 商店页面
storefiles-field-url-hint = 商品的地址，以 http:// 或 https:// 开头。
storefiles-field-words = 标签
storefiles-field-words-hint = 用来打标签的词语。逗号分隔的值会被拆开。
storefiles-field-type = 类型
storefiles-field-type-hint = 它是什么，例如“3D 模型”或“表面”。也会作为标签。
storefiles-field-colors = 颜色
storefiles-field-colors-hint = 它的颜色，作为 color: 标签。
storefiles-field-id = 商店 ID
storefiles-field-id-hint = 商店给它的 ID：字母、数字、_ 或 -。
storefiles-field-origin = 产地
storefiles-field-origin-hint = 它的制作或扫描地点，用于描述。
storefiles-field-size = 尺寸
storefiles-field-size-hint = 用文字描述它的大小，用于描述。
storefiles-field-title-name = 标题
storefiles-field-seller-name = 卖家
storefiles-field-license-name = 许可证
storefiles-field-url-name = 商店页面
storefiles-field-words-name = 标签
storefiles-field-type-name = 类型
storefiles-field-colors-name = 颜色
storefiles-field-id-name = 商店 ID
storefiles-field-origin-name = 产地
storefiles-field-size-name = 尺寸

## StoreFormatsPanel.vue

storefiles-load-failed = 无法加载商店文件格式：{ $error }
storefiles-remove-title = 移除 { $name }？
storefiles-remove-body = { $file } 将从数据文件夹中删除。已用它读取过的资源包会保留读取到的内容。
storefiles-remove = 移除
storefiles-removed = 已移除 { $name }。
storefiles-remove-failed = 无法移除 { $name }：{ $error }
storefiles-unread = Hoard 不读取的文件
storefiles-loading = 正在加载…
storefiles-unread-none = 无：资源库中没有看起来像未读取的商店文件。
storefiles-unread-what = 出现在多个资源包中或按商店文件命名、但没有任何格式读取的文件。
storefiles-unread-packs = { $count } 个资源包
storefiles-unread-example = 例如 { $pack }
storefiles-teach = 让 Hoard 学会读取此文件
storefiles-formats = 格式
storefiles-formats-what = 你的格式会优先尝试，并取代同名的内置格式。
storefiles-dir-before = 它们保存在
storefiles-dir-after = 中。
storefiles-yours = 我的
storefiles-built-in = 内置
storefiles-edit = 编辑
storefiles-start-from = 以此为起点

## TagsPanel.vue

tags-load-failed = 无法加载标签：{ $error }
tags-rename-title = 重命名 { $tag }？
tags-merge-title = 合并 { $count } 个标签？
tags-rename-body = 它的 { $count } 个资源将改用 { $into }，标注器今后也将使用新名称。
tags-merge-body = 最多 { $count } 个资源上的 { $tags } 将变为 { $into }。标注器今后将使用 { $into }。
tags-rename = 重命名
tags-merge = 合并
tags-merged = 现在有 { $count } 个资源带有标签 { $into }
tags-merge-failed = 无法合并：{ $error }
tags-forget-failed = 无法更改：{ $error }
tags-find = 查找标签
tags-find-placeholder = 查找标签，例如 rock 或 set:
tags-count = { $count } 个标签
tags-count-more = { $count }+ 个标签
tags-what = 选择含义相同的标签进行合并，或选择一个标签进行重命名。
tags-new-name = 新名称
tags-merge-into = 合并为
tags-saved-as-before = 已保存为
tags-saved-as-after = { "" }
tags-cancel = 取消
tags-loading = 正在加载…
tags-none = 没有标签。
tags-none-containing = 没有包含“{ $text }”的标签。
tags-browse-title = 浏览带有标签 { $tag } 的资源
tags-browse = 浏览
tags-merged-heading = 已合并的标签
tags-merged-what = 当标注器给出这些旧名称之一时，资源会得到它被合并到的那个标签。
tags-allow-old = 重新允许旧名称
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count = { $count } 个资源
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
       *[other] 旧名称：{ $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } 已合并为 { $into }。
tag-page-show = 显示 { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = 说说这个标签是什么意思…
tag-page-describe-label = 这个标签的含义
tag-page-describe-placeholder = 用来做什么，哪些内容属于它。按 Enter 保存。
tag-page-describe-edit = 点击修改
tag-page-description-saved = 已保存标签的含义
# The tag's picture: one of its assets stands for it.
tag-page-picture = 为这个标签选一张图片
tag-page-picture-help = 选一个它的资源作为代表
tag-page-picture-nothing = 当前显示的资源都没有图片。
tag-page-picture-none = 无图片
tag-page-picture-saved = 已设置图片
tag-page-picture-removed = 已移除图片
tag-page-rename = 重命名…
tag-page-rename-title = 给它起个新名称，或输入另一个标签的名称，把它合并过去
tag-page-rename-label = 新名称
tag-page-save-failed = 无法保存标签：{ $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = 上级标签
tag-page-parent-add = 设置上级标签…
tag-page-parent-add-title = 把这个标签放到更宽泛的标签下：搜索那个标签时，也会找到这些
tag-page-parent-placeholder = 更宽泛的标签，例如交通工具
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = 把 { $tag } 从 { $parent } 下移出
tag-page-parent-saved = { $tag } 现在位于 { $parent } 下
tag-page-parent-removed = { $tag } 不再位于 { $parent } 下
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = 下级标签：
tag-page-narrower-title = 搜索 { $tag } 时，也会找到这些
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = 上级与下级
tags-tree-what = 在标签页中放到更宽泛标签下的标签。搜索一个标签时，也会找到它下面的所有内容。

## LibraryStats.vue

stats-source-human = 人工添加
stats-source-ai = AI 建议
stats-source-rule = 内置规则
stats-source-meta = 来自文件
stats-assets = 资源
stats-packs = 资源包
stats-on-disk = 磁盘占用
stats-duplicates-title = 有 { $count } 个文件与其他文件逐字节相同，只存储一次
stats-saved-dedup = 去重节省
stats-distinct-tags = 不同标签
stats-archive = 归档中
stats-load-failed = 无法加载详细信息：{ $error }
stats-counting = 正在统计…
stats-as-of = 截至 { $time }
stats-updating = 正在更新…
stats-by-kind = 按类型的空间占用
stats-files-size = { $name }：{ $count } 个文件，{ $size }
stats-months = 过去一年添加的
stats-month = { $month }：{ $count } 个资源，{ $size }
stats-tagging = 标注
stats-tagged = 已打标签
stats-untagged = { $count } 个没有标签
stats-described = 已描述
stats-described-assets = { $count } 个资源
stats-human-edited = 手动编辑
stats-human-edited-unit =
    { $count ->
       *[other] 个资源
    }
stats-source-different = { $count } 个不同标签
stats-most-used = 最常用
stats-show-tag = 显示 { $tag } 资源
stats-file-types = 文件类型
stats-biggest-packs = 最大的资源包
stats-browse-pack = 浏览 { $name }
stats-largest-files = 最大的文件
stats-open-file = 打开 { $name }
stats-empty = 资源库中还没有内容。添加一个资源包，它的数据就会显示在这里。

## HoardPanel.vue

hoard-title = 你的宝藏
hoard-what = 资源库中的每个资源都是一件宝物，资源的种类决定是哪种宝物。收集新内容，把它们加进你的宝堆。
hoard-sign-in = 登录后即可收集属于你自己的宝藏。
hoard-new = 可收集的新战利品：{ $count }
hoard-none-new = 没有可收集的新内容。
hoard-collect = 收集
hoard-unavailable = 此浏览器无法绘制宝藏：{ $error }
hoard-collect-failed = 无法记录你收集的内容：{ $error }
hoard-arcade = 街机屏幕
hoard-full-screen = 全屏
hoard-leave-full-screen = 退出全屏
hoard-label = 一条守护宝藏的龙。宝物：{ $count }
hoard-gold = 金币
hoard-gems = 宝石
hoard-books = 书籍
hoard-weapons = 武器
hoard-bones = 骨头
hoard-tablets = 符文石板
hoard-paintings = 画作
hoard-potions = 药水

## UpdatesPanel.vue

updates-saved = 有可用更新时，Hoard 会调用此地址
updates-saved-none = 没有要调用的地址
updates-save-failed = 无法保存
updates-call-answered = 它返回了 { $result }
updates-call-refused = 它未接受请求：{ $result }
updates-call-failed = 无法调用
updates-called = { $when } 已为 { $version } 调用：{ $result }
updates-called-failed = { $when } 已为 { $version } 调用：{ $result }。Hoard 将在一小时后重试。
updates-latest = Hoard { $version } 是最新版本。
updates-out = Hoard { $version } 已发布。
updates-out-idle = Hoard { $version } 已发布；将在几分钟内趁空闲时调用。
updates-out-busy = Hoard { $version } 已发布；正在等待空闲（{ $work }）。
updates-busy-more = 另外 { $count } 项
updates-busy-jobs = { $count } 个任务等待中
updates-busy-uploads = { $count } 个上传
updates-list-separator = 、
updates-what = 新版本发布后，Hoard 可以在空闲时（没有上传、任务、转换或维护）调用一个地址，这样负责更新容器的程序就不会在上传中途重启它。它会发送一个 POST 请求，JSON 正文中包含版本信息。此功能需要开启上方的版本检查。
updates-url = 要调用的地址
updates-url-placeholder = 无（例如 http://watchtower:8080/v1/update?container=hoard&async=true）
updates-auth = Authorization
updates-auth-saved = 已保存（输入以替换）
updates-auth-placeholder = 无（例如 Bearer my-token）
updates-remove = 移除
updates-save = 保存
updates-call-now-title = 与它关联的程序会运行
updates-call-now = 立即调用
updates-what-calls = 可以调用什么？
updates-watchtower = Watchtower 的 HTTP API
updates-watchtower-at = ：
updates-watchtower-with = ，使用{ " " }
updates-watchtower-token = { " " }加上{ " " }
updates-watchtower-its = { " " }中的令牌（需开启其{ " " }
updates-watchtower-endpoint = { " " }端点）。
updates-portainer = Portainer webhook
updates-portainer-rest = ，设置在 Hoard 的 stack 或容器上，并开启 re-pull image。令牌包含在 URL 中。
updates-script = 主机上的脚本
updates-script-listener = ，由 webhook 监听器（例如{ " " }
updates-script-runs = ）触发，运行{ " " }
updates-script-end = 。
updates-others = Home Assistant、n8n
updates-others-rest = { " " }或其他任何接收 webhook 的程序。
updates-poll-before = 或者不设地址：持有访问令牌的脚本可以轮询{ " " }
updates-poll-until = { " " }，直到{ " " }
updates-poll-and = { " " }已设置且{ " " }
updates-poll-after = { " " }为 true，然后拉取新版本并重启 Hoard。

## MaintenancePanel.vue

maintenance-load-failed = 无法加载维护信息
maintenance-reanalyze-title = 重新分析所有资源包？
maintenance-reanalyze-body = 每个文件都会被重新读取，资源库较大时可能需要很长时间；轮到之前，资源包会显示为处理中。你自己设置的名称、标签和备注会保留。
maintenance-reanalyze-confirm = 重新分析
maintenance-gc-title = 删除未使用的文件？
maintenance-gc-body = 资源库中没有任何内容引用的存储文件将被永久删除。归档中的资源包会保留它们的文件。
maintenance-gc-confirm = 删除
maintenance-start-failed = 无法启动“{ $task }”
maintenance-just-now = 刚刚
maintenance-minutes-ago = { $count } 分钟前
maintenance-hours-ago = { $count } 小时前
maintenance-took-seconds = { $count } 秒
maintenance-took-minutes = { $count } 分钟
maintenance-took-hours = { $hours } 小时
maintenance-loading = 正在加载…
maintenance-running = 正在运行…
maintenance-starting = 正在启动…
maintenance-run = 运行
maintenance-started = { $detail } · { $when }开始
maintenance-starting-detail = 正在启动
maintenance-finished = 已完成：
maintenance-failed = 失败：
maintenance-never-run = 从未运行
maintenance-history = 历史运行
maintenance-col-task = 任务
maintenance-col-finished = 完成时间
maintenance-col-took = 耗时
maintenance-col-result = 结果
maintenance-script-before = 备份和损坏检查也可以通过脚本运行：{ " " }
maintenance-script-comma = 、
maintenance-script-and = { " " }和{ " " }
maintenance-script-after = 。这些运行也会显示在这里。

## SettingsPage.vue

settings-title = 设置
settings-loading = 正在加载…
settings-find = 查找设置
settings-find-none = 没有与“{ $query }”匹配的设置。
settings-load-failed = 无法加载设置：{ $error }
settings-page-appearance = 外观
settings-page-appearance-blurb = 此浏览器在浅色和深色模式下使用的主题。仅保存在此浏览器中。
settings-page-dragon = 巨龙
settings-page-dragon-blurb = 你的巨龙的宝藏：资源库里的每个资源都是其中一件宝物。
settings-page-library = 资源库
settings-page-library-blurb = 资源库里有什么：它有多大、由什么组成、如何增长，以及标签打得如何。已归档的内容不计入。
settings-page-copies = 近似副本
settings-page-copies-blurb = 只是尺寸、采样率或格式不同的同一图片、声音或模型文件，占用空间最多的排在前面。每组的最佳副本已标出；不会删除任何内容。
copies-loading = 正在查找近似副本…
copies-refreshing = 找到这些之后资源库已有变化。正在重新查找…
copies-none = 没有近似副本：每张图片、每个声音和模型在这里都只有一份。
copies-load-failed = 无法列出近似副本：{ $error }
copies-sum = 近似副本组：{ $count }。非最佳副本占用 { $size }。
copies-set = 第 { $n } 组
copies-set-wasted = 非最佳副本占用 { $size }
copies-more = 显示更多（还剩 { $count }）
settings-page-tags = 标签
settings-page-tags-blurb = 资源库中的所有标签。合并含义相同的标签，或重命名某个标签；此后标注器将使用新名称。
settings-page-store-files = 商店文件
settings-page-store-files-blurb = 商店随美术资源一起附带的文件，Hoard 会读取它们来为资源包命名并注明作者。你可以教它读取还不认识的文件，并分享你教给它的内容。
settings-page-locations = 资源库位置
settings-page-locations-blurb = 服务器上存放资源库的文件夹：Hoard 存放文件的文件夹，以及它就地建立索引的资源包文件夹。
settings-page-locations-blurb-desktop = 存放资源库的文件夹：Hoard 存放文件的文件夹，以及它就地建立索引的资源包文件夹。
settings-page-tools = 工具
settings-page-tools-blurb = Hoard 用于转换和读取文件的外部程序。
settings-page-ai = AI 标注
settings-page-ai-blurb = 在内置规则之外，由 AI 模型为新资源包命名、撰写描述并打标签。它的标签会标记为建议。
settings-page-search-blurb = 按图片内容搜索图片、按听起来像什么搜索声音，而不仅是按名称，并查找类型相似而不仅是颜色相似的内容。
settings-page-search-tab = 搜索
settings-page-adding = 添加资源包
settings-page-adding-blurb = 添加资源包时，除了读取和打标签之外，Hoard 还会做什么。
settings-page-downloads = 下载
settings-page-downloads-blurb = Hoard 发送的文件如何命名：下载、zip、hoard 工具和 AI 助手。资源库中的文件保持不变。
settings-page-account = 账户
settings-page-account-blurb = 你的密码、其他可以登录的人，以及供脚本和其他程序使用的访问令牌。
settings-page-auth = 身份验证
settings-page-auth-blurb = 用户如何登录：Hoard 自己的账户、LDAP 目录、Microsoft Active Directory 或 OAuth 提供方；会话持续多久；以及哪些组获得哪种角色。
settings-page-connection = 连接
settings-page-connection-blurb = 浏览器和应用如何访问 Hoard：它对外使用的地址，以及某些浏览器功能所需的 HTTPS。
settings-page-performance = 性能
settings-page-performance-blurb = 服务器的工作强度、缩略图的生成和保存方式，以及它最多接收多少内容。
settings-page-performance-blurb-desktop = Hoard 的工作强度、缩略图的生成和保存方式，以及它最多接收多少内容。
settings-page-issues = 问题
settings-page-issues-blurb = 打开或处理文件时出现的问题：无法解压的压缩包、无法读取的文件、失败的任务。解决原因后可以重试，也可以忽略。
settings-page-maintenance = 维护
settings-page-maintenance-blurb = 备份、检查损坏和清理。每种任务同一时间只运行一个，运行期间你可以离开此页面。
settings-page-about = 关于
settings-page-about-blurb = Hoard 是什么、这是哪个版本，以及构成它的他人成果。
settings-the-default = 默认值
settings-default-on = 开启
settings-default-off = 关闭
settings-default-auto = 自动
settings-option-saved = { $name }：{ $value }
settings-back-to-default = 恢复默认值
settings-save = 保存
settings-use = 使用
settings-status-off = 已关闭

## SettingsPage.vue: 外观

settings-themes = 主题
settings-look-light = 浅色
settings-look-dark = 深色
settings-look-group =
    { $scheme ->
        [light] 浅色主题
       *[dark] 深色主题
    }
settings-preview =
    正在显示{ $preview ->
        [light] 浅色
       *[dark] 深色
    }主题供你查看。你当前使用的是{ $used ->
        [light] 浅色
       *[dark] 深色
    }{ $system ->
        [yes] （跟随系统）
       *[other] { "" }
    }，离开此页面后会恢复。
settings-preview-back =
    返回{ $used ->
        [light] 浅色
       *[dark] 深色
    }
settings-look-hint = 可在顶栏中切换浅色和深色（或跟随系统）。

## SettingsPage.vue: 工具

settings-blender-what = 在后台将 .blend、.dae、.abc 和 .x3d 模型转换为 glTF，使它们获得缩略图和 3D 预览。关闭期间添加的资源包会在它就绪后转换。
settings-blender-download = 使用已安装的 Blender，或从 blender.org 镜像下载 Blender 4.5 LTS（约 400 MB）。
settings-ffmpeg-what = 编码 MP3、OGG 和 Opus 下载，并读取 Hoard 自身无法解码的音频（AAC、M4A、WMA 等）。
settings-ffmpeg-download = 使用带有所需编码器的系统 ffmpeg，或下载静态构建版本（约 80 MB）。
settings-tool-reset = { $tool }：恢复默认值
settings-tool-no-download = 使用已安装的 { $tool }。没有适用于此机器的可下载版本，请安装它或提供其路径。
settings-tool-path = { $tool } 程序路径
settings-mode-off = 关闭
settings-mode-auto = 自动
settings-mode-path = 指定程序
settings-tool-looking = 正在查找…
settings-tool-downloading = 正在下载… 进度显示在活动指示器中
settings-tool-ready = 就绪：{ $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = 播放和转换 MIDI 文件所用的乐器音色。
settings-soundfont-file = SoundFont 文件
settings-soundfont-placeholder = 在服务器上查找，或使用内置的
settings-soundfont-placeholder-desktop = 在这台电脑上查找，或使用内置的
settings-soundfont-default = 在服务器上查找，或使用内置的
settings-soundfont-default-desktop = 在这台电脑上查找，或使用内置的
settings-soundfont-send = 选择文件…
settings-soundfont-sent = MIDI 现在使用 { $name } 播放

## SettingsPage.vue: 图像搜索

settings-clip-heading = 按图片内容搜索
settings-clip-reset = 图像搜索：恢复默认值
settings-clip-saved-on = 图像搜索：开启
settings-clip-saved-off = 图像搜索：关闭
settings-clip-what = 输入你要找的内容（“mossy rock”“sci-fi door”“wooden barrel”），显示这些内容的图片会与名称含有这些词的结果一起出现。“更多类似内容”会查找同类的东西，而不仅是颜色相同的。
settings-clip-toggle = 开启图像搜索
settings-clip-hint = 使用 Google 的 SigLIP 2 模型，在此服务器的处理器上运行：任何数据都不会离开服务器。开启后会从 Hugging Face 下载模型（{ $size } MB），然后在后台以低优先级识别每张图片。工作或有人搜索时会占用 350 到 750 MB 内存。关闭后会删除模型，但保留已识别的内容，供下次使用。
settings-clip-hint-desktop = 使用 Google 的 SigLIP 2 模型，在这台电脑的处理器上运行：任何数据都不会离开这台电脑。开启后会从 Hugging Face 下载模型（{ $size } MB），然后在后台以低优先级识别每张图片。工作或你搜索时会占用 350 到 750 MB 内存。关闭后会删除模型，但保留已识别的内容，供下次使用。
settings-clip-looking = 正在查找模型…
settings-clip-downloading = 正在下载模型… 进度显示在活动指示器中
settings-clip-ready-empty = 就绪。添加资源包时会识别其中的图片。
settings-clip-ready-all =
    { $count ->
       *[other] 就绪：已识别全部 { $count } 张图片
    }
settings-clip-learning = 就绪：正在识别图片，目前已完成 { $done } / { $total }
settings-clip-relearning = 在新模型学完所有图片之前，搜索使用旧模型学到的内容。
settings-clip-unavailable = 无法获取模型：{ $detail }。每小时重试一次。
settings-clip-where = 运行位置
settings-clip-here = 本服务器：不会有任何内容离开
settings-clip-here-desktop = 本机：不会有任何内容离开
settings-clip-hosted = 托管服务
settings-clip-hosted-warning = 图片会离开本服务器。库中每张图片的缩略图，以及每次图像搜索的文字，都会发送给 { $service }，它会为此收费，并按其条款保留这些内容。
settings-clip-hosted-warning-desktop = 图片会离开本机。库中每张图片的缩略图，以及每次图像搜索的文字，都会发送给 { $service }，它会为此收费，并按其条款保留这些内容。
settings-clip-hosted-hint = 这里不下载也不运行任何东西：服务会分批学习每张图片。换一个服务或模型会重新学习全部图片，每个学过的内容都会保留，方便切换回来。
settings-clip-saved-here = 图像搜索：在本服务器上运行
settings-clip-saved-here-desktop = 图像搜索：在本机上运行
settings-clip-saved-hosted = 图像搜索：在 { $service } 上运行
settings-clip-hosted-unavailable = 无法使用 { $service }：{ $detail }。每小时重试一次。
settings-clap-heading = 按声音听起来像什么搜索
settings-clap-reset = 声音搜索：恢复默认值
settings-clap-saved-on = 声音搜索：开启
settings-clap-saved-off = 声音搜索：关闭
settings-clap-what = 输入你要找的内容（“distant thunder”“glassy UI blip”“footsteps on gravel”），听起来像这样的声音会与名称含有这些词的结果一起出现。
settings-clap-toggle = 开启声音搜索
settings-clap-hint = 使用 LAION 的 CLAP 模型，在此服务器的处理器上运行：任何数据都不会离开服务器。开启后会从 Hugging Face 下载模型（{ $size } MB），然后在后台以低优先级聆听每个声音的开头。工作或有人搜索时最多占用约 1 GB 内存。关闭后会删除模型，但保留已识别的内容，供下次使用。
settings-clap-hint-desktop = 使用 LAION 的 CLAP 模型，在这台电脑的处理器上运行：任何数据都不会离开这台电脑。开启后会从 Hugging Face 下载模型（{ $size } MB），然后在后台以低优先级聆听每个声音的开头。工作或你搜索时最多占用约 1 GB 内存。关闭后会删除模型，但保留已识别的内容，供下次使用。
settings-clap-ready-empty = 就绪。添加资源包时会识别其中的声音。
settings-clap-ready-all =
    { $count ->
       *[other] 就绪：已识别全部 { $count } 个声音
    }
settings-clap-learning = 就绪：正在识别声音，目前已完成 { $done } / { $total }

## SettingsPage.vue: AI 标注

settings-ai-service = 服务
settings-ai-off = 关闭
settings-ai-off-hint = 仅使用内置规则为资源包打标签。
settings-ai-local = 本地模型
settings-ai-local-hint = Hoard 的 compose 文件中的 llm 服务（或 cargo xtask llm）。请在下方下载模型；它在本机运行，任何数据都不会离开本机。
settings-ai-chatgpt = ChatGPT 套餐
settings-ai-chatgpt-hint = 使用你的 ChatGPT Go、Plus、Pro 或 Business 套餐，而不是 API 密钥。
settings-ai-openrouter-hint = 一个密钥即可使用数百个托管模型，其中一些免费。能接收图片的模型会有标记。
settings-ai-openai-hint = 由 OpenAI 托管。
settings-ai-anthropic-hint = 由 Anthropic 托管，通过其兼容 OpenAI 的端点访问。
settings-ai-gemini-hint = 由 Google 托管，通过其兼容 OpenAI 的端点访问。
settings-ai-ollama-hint = 你自己运行的 Ollama 服务器。如果它不是名为 ollama 的容器，请更改地址。
settings-ai-custom = 其他（兼容 OpenAI）
settings-ai-custom-hint = 任何支持 OpenAI chat API 的服务：LM Studio、vLLM、llama.cpp、LiteLLM 等
settings-ai-connecting = 正在连接…
settings-ai-connected = 已连接 · { $count } 个模型
settings-ai-connected-none = 已连接，但未列出任何模型；请输入模型名称
settings-chatgpt-connected-none = 已连接；请输入模型名称
settings-ai-takes-images = 可接收图片
settings-ai-saved = AI 标注：{ $service }
settings-ai-saved-off = AI 标注已关闭
settings-ai-reset = AI 标注：恢复默认值
settings-ai-on = 已开启：{ $service } · { $model }
settings-ai-url = 地址
settings-ai-key = API 密钥
settings-ai-key-saved = 已保存（输入以替换）
settings-ai-key-paste = 粘贴你的密钥
settings-ai-key-optional = 无（可选）
settings-ai-key-remove = 移除
settings-ai-get-key = 获取密钥
settings-ai-model = 模型
settings-ai-model-pick = 选择或输入模型
settings-ai-model-name = 模型名称
settings-ai-refresh = 刷新
settings-ai-connect = 连接
settings-ai-need-key = 添加 API 密钥后即可查看其模型
settings-ai-vision = 同时发送缩略图（需要能接收图片的模型）
settings-ai-batch = 每次请求
settings-ai-batch-unit = 个资源（小型本地模型宜少，大型托管模型可多）

## SettingsPage.vue: ChatGPT 登录

settings-chatgpt-account = 账户
settings-chatgpt-checking = 正在检查…
settings-chatgpt-status-as = 已以 { $email } 身份登录
settings-chatgpt-status = 已登录
settings-chatgpt-signed-in-as = 已以 { $email } 身份登录 ChatGPT
settings-chatgpt-signed-in = 已登录 ChatGPT
settings-chatgpt-signed-out = 已退出 ChatGPT
settings-chatgpt-continue = 使用 ChatGPT 继续
settings-chatgpt-sign-out = 退出登录
settings-chatgpt-sign-out-title = 退出 ChatGPT 登录？
settings-chatgpt-sign-out-in-use = AI 标注正在使用你的 ChatGPT 套餐，因此在你重新登录或选择其他服务之前，它会停止工作。
settings-chatgpt-sign-out-body = Hoard 会忘记此登录，并通知 OpenAI 结束它。
settings-chatgpt-manage = 管理用量
settings-chatgpt-finish-before = 请在打开的窗口中完成登录（
settings-chatgpt-finish-link = 重新打开
settings-chatgpt-finish-middle = ）。如果最后停在一个无法加载的页面，且地址以{ " " }
settings-chatgpt-finish-after = { " " }开头，请复制该地址并粘贴到这里：
settings-chatgpt-paste = 登录结束时所在的地址
settings-chatgpt-finish = 完成
settings-chatgpt-usage = 标注请求会计入你的 ChatGPT 套餐用量。“管理用量”会显示 Hoard 已使用的量，并允许你设置限额。如需在 ChatGPT 端断开 Hoard，请前往 chatgpt.com 上的“设置 → 安全与登录 → 使用 ChatGPT 登录”。

## SettingsPage.vue: 本地模型

settings-local-downloading-progress = 正在下载 { $percent }% · { $done } / { $total }
settings-local-downloading = 正在下载…
settings-local-incomplete = 下载已停止
settings-local-failed = 上次启动失败
settings-local-in-use-running = 使用中 · 运行中
settings-local-in-use = 使用中
settings-local-starting = 正在启动…
settings-local-running = 运行中
settings-local-ready-images = 就绪 · 可接收图片
settings-local-ready-text = 就绪 · 仅文本
settings-local-delete-title = 删除 { $name }？
settings-local-delete-in-use = AI 标注正在使用此模型，因此在你选择其他模型之前，它会停止工作。其文件将从模型服务器中删除。
settings-local-delete-downloading = 下载将停止，已下载的部分会被删除。
settings-local-delete-body = 其文件将从模型服务器中删除。你以后可以重新下载。
settings-local-delete = 删除
settings-local-deleting = 正在删除…
settings-local-cancel = 取消
settings-local-resume = 继续
settings-local-download = 下载
settings-local-start-before = 请用{ " " }
settings-local-start-or = { " " }启动它（开发时用{ " " }
settings-local-start-developing = { " " }），然后{ " " }
settings-local-start-retry = 重试
settings-local-start-after = 。只运行一个模型的服务器也可以：请在下方填写其模型名称。
settings-local-list = 模型服务器上的模型
settings-local-none = 还没有下载任何模型。请在下方选择一个；它会保存在服务器上，你可以随时切换。
settings-local-catalog = 可下载的模型
settings-local-size = { $size } GB · { $license }
settings-local-other = Hugging Face 上的其他模型
settings-local-other-placeholder = Hugging Face 上的其他模型：owner/name-GGUF:Q4_K_M

## SettingsPage.vue: 添加资源包

settings-store-pages = 商店页面
settings-store-pages-what = 新资源包会在后台到各商店中查找，以获取其商店页面、卖家和许可证。
settings-store-lookup = 自动查找商店页面
settings-unreal = Unreal 包
settings-unreal-reset = Unreal 包：恢复默认值
settings-unreal-what = 如何处理 .uasset 文件。
settings-conv = 转换规则
settings-conv-what = 将 Hoard 保存的文件转换为同一种格式，例如把所有 OBJ 和 FBX 转为 GLB，使整个资源库格式统一。转换后的文件会取代原始文件，并保留其名称、标签、合集和链接。监视文件夹中的文件永远不会被更改。
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = 启用
settings-conv-to = 转换为
# Heading over the file formats a rule converts.
settings-conv-from = 源格式
settings-conv-remove = 移除
settings-conv-remove-label = 移除规则 { $n }
settings-conv-taken = 另一条规则已在转换此格式
settings-conv-upright = 转换时将模型摆正（Y 轴朝上，面向 +Z），与资源包中模型的显示方式一致
settings-conv-flac-what = 声音完全不变，空间约为一半；WAV 中的其他所有内容（循环点、cue 标记、标签）都保留在 FLAC 里，因此随时可以把 WAV 原样恢复。带有浮点或 32 位采样的 WAV 会保持为 WAV。
settings-conv-add = 添加规则
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = 原始文件保留
settings-conv-keep-days =
    { $count ->
       *[other] 天
    }
settings-conv-keep-what = 在此之前，可在转换后文件的页面上恢复原始文件。纹理不会复制到转换后的模型中：模型引用其旁边的图像文件。模型的 .mtl 或 .bin 文件会保留。
settings-conv-save = 保存规则
settings-conv-revert = 撤销更改
settings-conv-existing = 转换资源库中已有的文件
settings-conv-saved = 转换规则已保存。它们将应用于此后添加的资源包。
settings-conv-converting =
    { $count ->
       *[other] 正在后台转换 { $count } 个文件
    }
settings-conv-nothing = 资源库中没有需要转换的文件。
settings-conv-ask-title =
    { $count ->
       *[other] 要转换资源库中已有的 { $count } 个文件吗？
    }
settings-conv-ask-body =
    { $packs ->
       *[other] 它们位于 { $packs } 个资源包中。
    }每个原始文件保留 { $days } 天，在此期间可从其页面恢复。
settings-conv-ask-busy =
    { $count ->
       *[other] 其中 { $count } 个位于仍在添加中的资源包里，添加完成后即会转换。
    }
settings-conv-ask-list = 将要转换的文件
settings-conv-ask-more = ……以及另外 { $count } 个
settings-conv-ask-skipped = 保持原样：
settings-conv-skip-watched =
    { $count ->
       *[other] 监视文件夹中的 { $count } 个文件
    }
settings-conv-skip-unity =
    { $count ->
       *[other] Unity 项目按名称引用的 { $count } 个文件
    }
settings-conv-skip-used =
    { $count ->
       *[other] 被场景或其他文件按名称引用的 { $count } 个文件
    }
settings-conv-skip-beside =
    { $count ->
       *[other] 转换后的副本已在其资源包中的 { $count } 个文件
    }
settings-conv-skip-blender =
    { $count ->
       *[other] Blender 尚未读取的 { $count } 个文件（设置 → 工具）
    }
settings-conv-ask-new = 保存，仅用于新资源包
settings-conv-ask-save-convert =
    { $count ->
       *[other] 保存并转换 { $count } 个文件
    }
settings-conv-ask-convert =
    { $count ->
       *[other] 转换 { $count } 个文件
    }
settings-unreal-off = 保持原样
settings-unreal-off-hint = 需要时可在资源包页面中转换。
settings-unreal-keep = 转换并保留原始文件
settings-unreal-keep-hint = 纹理、声音和材质会转换为通用格式；.uasset 也会保留。
settings-unreal-replace = 转换并丢弃原始文件
settings-unreal-replace-hint = 内容提取后可节省空间。网格仍需等待 Unreal worker 处理。

## SettingsPage.vue: 下载

settings-tn = 纹理名称
settings-tn-saved = 纹理名称已保存
settings-tn-reset = 纹理名称：恢复默认值
settings-tn-what = 无论文件原本如何命名，材质的贴图在发送时都会以材质名称和贴图用途命名：Rock_Normal.png，而不是 rock_moss_xeokah0_raw_8K_Normal.png。资源库中的文件不受影响；模型或 MTL 中指定了名称的贴图会保留该名称，以便模型仍能找到它。
settings-tn-toggle = 按材质命名纹理贴图
settings-tn-style = 风格
settings-tn-style-group = 命名风格
settings-tn-descriptive = 描述性
settings-tn-short = 简短
settings-tn-custom = 自定义
settings-tn-pattern = 模式
settings-tn-hint-name = { " " }材质名称，{ " " }
settings-tn-hint-map = { " " }贴图用途，{ " " }
settings-tn-hint-size = { " " }尺寸（当一套贴图有多种尺寸时；如果模式中省略，会加在末尾）。
settings-tn-example = 示例
settings-tn-suffixes = 各类贴图的后缀
settings-tn-slots = 在 GLB 下载中，打包在模型内部、没有独立文件的纹理会以材质和它所占用的槽位命名，这些槽位也有后缀：
settings-tn-endings = 自定义结尾
settings-tn-endings-what = 不属于材质贴图的纹理会保留自己的名称，除非名称以下列结尾之一结束：_diff → _BC 会把 rock_diff.png 发送为 rock_BC.png。第二项留空即可去掉该结尾。
settings-tn-ending-from = 结尾
settings-tn-ending-from-example = _diff
settings-tn-ending-to = 替换为
settings-tn-ending-to-example = _BC
settings-tn-ending-add = 添加结尾
settings-tn-ending-remove = 移除
settings-tn-ending-remove-label = 移除 { $ending }

## SettingsPage.vue: 常规和连接

settings-name = 名称
settings-name-what = 此服务器在顶栏、浏览器标签页和登录页面上显示的名称，这样在有多个 Hoard 时你能分清哪个是哪个。
settings-name-what-desktop = 此资源库在顶栏上显示的名称。
settings-address = 地址
settings-address-what = 当用户和应用访问 Hoard 所用的地址与此浏览器所用的不同时填写：反向代理、隧道或 tailnet 名称，例如 https://hoard.example.com。Hoard 发出的链接（给 Claude 和其他应用的、登录中的、证书上的）都会使用它。
settings-public-url = 公开地址
settings-public-url-placeholder = 取自浏览器地址（例如 https://hoard.example.com）
settings-public-url-default = 取自浏览器地址

## ProfilerPanel.vue: 设置 → 性能顶部的性能分析器

profiler = 性能分析器
profiler-what = Hoard 在过去十分钟里做了什么，你查看时每两秒更新一次。数据只保存在内存中，Hoard 重启后会从头开始。
profiler-starting = 第一批读数马上就到。
profiler-failed = 无法读取性能分析器：{ $error }
profiler-cpu = 处理器使用率
profiler-cpu-hoard = Hoard
profiler-cpu-machine = 整台机器
profiler-cpu-summary = Hoard 正在使用 { $hoard } 的处理器，整台机器为 { $machine }。
profiler-ram = 内存
profiler-ram-peak = 峰值
profiler-ram-of = { $used } / { $total }
profiler-ram-summary = Hoard 正在使用 { $hoard } 内存，过去十分钟内最多使用 { $peak }。
profiler-serving = 进行中的请求
profiler-working = 正在处理
profiler-working-none = 后台没有运行任何任务。
profiler-working-more =
    { $count ->
       *[other] 还有 { $count } 项。
    }
profiler-requests = 处理请求的时间
profiler-requests-what = 每根柱代表十秒内的请求，其时间按去向拆分。
profiler-part-db = 数据库
profiler-part-db-title = 等待数据库并执行其查询
profiler-part-disk = 磁盘
profiler-part-disk-title = 读取存储的文件，并压缩要发送的内容
profiler-part-send = 发送
profiler-part-send-title = 等待网络接收已发送的内容
profiler-part-other = 其他
profiler-part-other-title = 生成响应
profiler-request-count =
    { $count ->
       *[other] { $count } 个请求
    }
profiler-requests-none = 过去十分钟没有请求。
profiler-slowest = 最慢：{ $route }，{ $time }
profiler-routes = 按路由
profiler-route = 路由
profiler-count = 请求数
profiler-average = 平均
profiler-p95 = 最慢的 5%
profiler-split = 时间去向
profiler-sent = 已发送
profiler-portal = 界面自身的文件

## SettingsPage.vue: 性能

settings-render = 3D 缩略图
settings-render-toggle = 为 3D 模型渲染缩略图
settings-gpu-on = 使用 GPU：{ $renderer }
settings-gpu-software = 未找到 GPU，因此使用软件渲染（{ $renderer }）
settings-gpu-none = 未找到 GPU 或软件渲染器，因此模型不会有缩略图
settings-thumbs = 缩略图
settings-thumbs-what = 新缩略图和模型视图的存储方式。带透明度的图片和像素画始终为 PNG。要压缩已有的缩略图，请在“维护”中运行“压缩缩略图”。
settings-previews-size = 预览目前占用 { $size }。
settings-thumbs-saved = 缩略图：{ $quality }
settings-thumbs-sent = { $format }，质量 { $quality }。
settings-thumbs-small = 更小
settings-thumbs-small-hint = 在网格中没问题；近看稍有模糊。
settings-thumbs-balanced = 均衡
settings-thumbs-balanced-hint = 在缩略图尺寸下很难与原图区分。
settings-thumbs-sharp = 更清晰
settings-thumbs-sharp-hint = 大小约为“均衡”的两倍。
settings-thumbs-lossless = 无损
settings-thumbs-lossless-hint = PNG，与生成时完全一致。大好几倍。从不以 AVIF 发送。
settings-thumbs-avif = 以 AVIF 发送缩略图
settings-thumbs-avif-what = 能显示 AVIF 的浏览器会收到与上面所选清晰度相同的缩略图，常常不到一半大小。每张缩略图在第一次显示时由已存的缩略图生成，并与转换结果一起保存；在此之前发送已存的那张。首次浏览大型资源库时，生成它们最多会占用两个核心。无损缩略图和像素画按原样发送。
settings-motion-thumbs = 动画的动态缩略图
settings-motion-thumbs-what = 每个动画还会生成一段循环播放的短片，指向其图块时播放。每段约 40 KB，大型资源库需要一段时间才能生成完。关闭后会删除它们。
settings-limits = 工作量与限制
settings-limits-what = 每项在你离开输入框或按 Enter 时保存。大小需带单位：100 GB、512 MB、1.5 TB；纯数字按 GB 计。
settings-workers = 后台工作线程
settings-workers-unit = 个任务同时进行（导入、打标签、转换）
settings-max-upload = 最大上传
settings-max-upload-unit = 每个文件
settings-max-expand = 最大资源包
settings-max-expand-unit = 解压后
settings-conversion-cache = 转换后的下载
settings-conversion-cache-unit-now = 最多保留，目前为 { $size }；最久未使用的先删除
settings-conversion-cache-unit = 最多保留；最久未使用的先删除

## SettingsPage.vue: 维护

settings-updates = 更新
settings-version = 这是 Hoard { $version }。
settings-update-check = 每隔几小时检查 GitHub 上是否有新版本
settings-desktop-updates = Hoard 每次启动时都会查找新版本，在后台下载，并在你退出时安装。
settings-backups = 定时备份
settings-backups-what = 数据库会自动备份，每次备份都会显示在下方的历史记录中。
settings-backup-every = 备份
settings-backup-off = 从不
settings-backup-daily = 每天
settings-backup-weekly = 每周
settings-backup-keep = 保留
settings-backup-keep-unit = 份最新的定时备份
settings-backup-copy-to = 同时复制到
settings-backup-copy-to-none = 不复制到其他文件夹
settings-backups-kept = 每份备份以 hoard-auto-…db 保存在备份文件夹中；如果填写了其他文件夹（另一块磁盘，或同步应用会备份的文件夹），也会复制一份过去，请填写它在服务器上的完整路径。只会删除较旧的定时备份，手动创建的备份会保留。
about-what = Hoard 把游戏素材（模型、纹理、音效、音乐、精灵图等）放在一个资源库里，供你搜索、预览和取用，并与你允许访问的所有人共享。
about-version = 版本
about-edition = 发行版
about-edition-server = 服务器
about-edition-desktop = 桌面应用
about-edition-steam = Steam 版桌面应用
about-made-by = 开发者
about-source = GitHub 上的源代码
about-issues = 报告问题
about-discord = 在 Discord 上和我们聊天
about-steam = Steam 上的 Hoard
about-license = 许可证
about-license-what = Hoard 以 MIT No Attribution 许可证开源：你可以随意使用、修改和分享。
about-downloaded = 按需下载
about-downloaded-what = Hoard 不自带这些组件。它会在首次需要时下载各个组件，每个组件都有各自的许可证。
about-blender = 转换 3D 模型并生成缩略图。
about-ffmpeg = 转换音频和视频并生成预览。
about-siglip = 按图片中的内容查找图片。
about-clap = 按声音听起来的样子查找音效。
about-models = 你为本地 AI 服务器选择的模型遵循其作者的许可证。
about-built-with = 使用的开源软件
about-built-with-what = 构成 Hoard 的开源库及其许可证。
about-steamworks = Steam 版还包含 Steamworks SDK，© Valve Corporation。
about-licenses-loading = 正在加载列表…
about-licenses-failed = 无法加载列表。
about-filter = 按名称或许可证筛选
about-count = { $shown } / { $total }
about-no-license = 未注明许可证
about-report = 报告问题…
report-title = 报告问题
report-what = 这是 GitHub 上的 issue 调查问题所需的信息。其中没有任何隐私内容：访问令牌、密码、密钥、电子邮件地址以及文件夹路径中的用户名都会被去除。发送前请通读一遍。
report-loading = 正在收集报告…
report-failed = 无法收集报告：{ $error }
report-log = 服务器最近记录的日志
report-copy = 复制
report-copied = 已复制报告。
report-github = 在 GitHub 上新建 issue
report-github-hint = 新 issue 会自动填入这些内容。日志太长时只放入最后几行；“复制”包含全部内容。
report-close = 关闭

## 资源类型（api.ts）

api-kind-texture = 纹理
api-kind-vector = 矢量图
api-kind-sheet = 精灵图集
api-kind-audio = 音频
api-kind-model = 模型
api-kind-animation = 动画
api-kind-font = 字体
api-kind-shader = 着色器
api-kind-material = 材质
api-kind-video = 视频
api-kind-document = 文档
api-kind-data = 数据
api-kind-other = 其他
api-kind-texture-name = 纹理
api-kind-vector-name = 矢量图
api-kind-sheet-name = 精灵图集
api-kind-audio-name = 音频
api-kind-model-name = 模型
api-kind-animation-name = 动画
api-kind-font-name = 字体
api-kind-shader-name = 着色器
api-kind-material-name = 材质
api-kind-video-name = 视频
api-kind-document-name = 文档
api-kind-data-name = 数据
api-kind-other-name = 其他资源

## 提示、对话框与加载（ui.ts、UiLayer.vue、lazy.ts）

ui-undo = 撤销
ui-today = 今天
ui-yesterday = 昨天
ui-days-ago =
    { $days ->
       *[other] { $days } 天前
    }
ui-notifications = 通知
ui-dismiss = 忽略
ui-cancel = 取消
ui-lazy-loading = 正在加载{ $what }…
ui-lazy-failed = 无法加载{ $what }。
ui-lazy-reload = 重新加载页面

## ResetButton.vue

reset-title = 恢复默认值：{ $value }
reset-label = 恢复默认值，{ $value }

## SizeInput.vue

size-invalid = 请输入大小，例如 100 GB、512 MB 或 1.5 TB
size-hint = 请输入大小，例如 100 GB、512 MB 或 1.5 TB；纯数字按 GB 计

## AuthPanel.vue（设置 → 身份验证）

auth-load-failed = 无法加载身份验证设置：{ $error }
auth-saved = 已保存。将在下次登录时生效。
auth-method-internal = 内置
auth-method-internal-hint = 由 Hoard 保存的用户名和密码，在“设置 → 账户”中添加。
auth-method-ldap-hint = 由目录服务核对在登录页面输入的用户名和密码：OpenLDAP、FreeIPA、LLDAP 或其他 LDAP 服务器。
auth-method-ad-hint = Windows 域账户使用平常的用户名和密码登录。
auth-method-oauth-hint = 一个“使用…登录”按钮，跳转到 Google、Microsoft、GitHub、Keycloak、Authentik 或其他提供方，然后返回。
auth-session-unused = 30 天未使用后
auth-session-1h = 登录 1 小时后
auth-session-8h = 登录 8 小时后
auth-session-day = 登录 1 天后
auth-session-week = 登录 1 周后
auth-session-30d = 登录 30 天后
auth-session-hours =
    { $hours ->
       *[other] 登录 { $hours } 小时后
    }
auth-role-view = 可查看
auth-role-edit = 可修改
auth-role-admin = 管理员
auth-role-word-view = 可查看
auth-role-word-edit = 可修改
auth-role-word-admin = 管理员
auth-role-word-default = { $role }（默认角色）
auth-tls-starttls-hint = 普通 LDAP（端口 389），在发送任何内容之前切换到 TLS。
auth-tls-ldaps-hint = 从一开始就使用 TLS（端口 636）。
auth-tls-none = 无
auth-tls-none-hint = 密码将以明文形式在网络上传输。仅限在你信任的网络中使用。
auth-test-failed = 无法测试：{ $error }
auth-preset-custom = 其他（OpenID Connect）
auth-preset-custom-note = Keycloak、Authentik、Authelia、Okta、Zitadel 等：填写颁发者后，点击“发现”即可自动填入各个地址。
auth-preset-google-note = 在 Google Cloud Console → API 和服务 → 凭据 中创建一个 OAuth 客户端（Web 应用）。Google 不提供组信息，因此请在“设置 → 账户”中分配角色；除非任何拥有 Google 账户的人都应能进入，否则请关闭“首次登录时创建账户”。
auth-preset-microsoft-note = 在 Entra ID → 应用注册 中注册一个应用（Web 重定向地址填写下方的地址）。将“organizations”替换为你的租户 ID，即可只允许你的组织成员进入。如需按组分配角色，请在“令牌配置”中添加 groups 声明：它会列出组 ID。
auth-preset-github-note = 在 GitHub → Settings → Developer settings 中创建一个 OAuth 应用。GitHub 不提供组信息，因此请在“设置 → 账户”中分配角色；除非任何拥有 GitHub 账户的人都应能进入，否则请关闭“首次登录时创建账户”。
auth-preset-gitlab-note = 在 GitLab → 偏好设置 → 应用 中创建一个应用（或在你的实例的管理中心中创建：并将地址中的 gitlab.com 替换掉）。
auth-discovered = 已填入提供方的地址。
auth-copied = 已复制
auth-copy-failed = 无法复制：请选中后自行复制
auth-sign-in-with = 登录方式
auth-own-accounts-still-work = Hoard 自己的账户仍使用各自的密码登录，因此即使目录服务或提供方宕机或配置错误，也不会把你锁在门外。
auth-sessions = 会话
auth-sign-out-after = 自动退出登录
auth-ldap-server = LDAP 服务器
auth-server-type = 服务器类型
auth-server-type-custom = 自定义
auth-servers = 服务器
auth-server-n = 服务器 { $n }
auth-remove-server-n = 移除服务器 { $n }
auth-remove = 移除
auth-add-server = 添加服务器
auth-servers-hint = 一台服务器无法连接时，将尝试下一台。
auth-security = 安全
auth-connection-security = 连接安全
auth-skip-verify = 不检查服务器的证书（仅用于试用）
auth-ca = 证书颁发机构
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  （留空：使用系统信任的证书）
auth-lookup = 查找用户
auth-anonymously = 匿名
auth-reader-account = 查询账户
auth-reader-password = 其密码
auth-saved-type-to-replace = 已保存：输入新值以替换
auth-people-under = 用户位于
auth-name-attribute = 用户名属性
auth-only-matching = 仅限匹配以下条件的条目
auth-groups-under = 组位于
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  （留空：仅使用 memberOf）
auth-their-groups = 用户所在的组
auth-group-filter-hint = { "{" }dn{ "}" } 是此人的条目，{ "{" }user{ "}" } 是其登录时使用的用户名。
auth-test = 测试
auth-test-hint = 连接每台服务器并以查询账户登录。如果填写了用户名和密码，还会以该用户身份登录，并显示 Hoard 会如何处理。不会保存任何内容。
auth-test-user = 用于测试登录的用户
auth-test-user-placeholder = 用于测试登录的用户（可选）
auth-test-password = 该用户的密码
auth-testing = 正在测试…
auth-test-connectivity = 测试连接
auth-test-server-ok = 已连接，查询账户已登录
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }已登录（
auth-test-signed-in-after = ）。
auth-test-role = 其角色将是：{ $role }。
auth-test-no-role = 此人不在“角色”下列出的任何组中，因此将被拒绝。
auth-test-groups = 组：{ $groups }
auth-test-no-groups = 未找到任何组。
auth-provider = 提供方
auth-redirect = 重定向地址
auth-copy = 复制
auth-redirect-hint = 为 Hoard 创建客户端时，请将此地址提供给提供方。它来自“设置 → 连接”中的公开地址，或你当前使用的地址。
auth-issuer = 颁发者
auth-discover = 发现
auth-client-id = 客户端 ID
auth-client-secret = 客户端密钥
auth-authorization-url = 授权地址
auth-token-url = 令牌地址
auth-userinfo-url = 用户信息地址
auth-userinfo-placeholder = （留空：由 ID 令牌说明用户身份）
auth-scopes = 作用域
auth-name-from = 用户名来自
auth-groups-from = 组来自
auth-groups-from-placeholder = （无）
auth-button = 按钮
auth-button-placeholder = 你的账户
auth-accounts = 账户
auth-provision = 有人首次登录时为其创建 Hoard 账户
auth-provision-on-hint = 否则需要由管理员先在“设置 → 账户”中逐个添加用户。
auth-provision-off-hint = 由管理员先在“设置 → 账户”中逐个添加用户，并选择其登录方式。
auth-open-to-anyone = 任何拥有 { $provider } 账户的人都能进入。请在下方添加查看者组，或关闭此选项并自行添加用户。
auth-new-accounts = 新账户
auth-roles-from-groups = 按组分配角色
auth-roles-hint = 各角色对应的组，每行一个，填写组名或 DN。匹配到的最高角色生效，并在每次登录时检查。如果未列出任何组，新账户将获得上方的角色，之后由管理员修改。如果在“可查看”下列出了组，则只有所列组的成员才能登录。
auth-roles-hint-oauth = 各角色对应的组，每行一个，填写组名或 DN（或 groups 声明中列出的任何值）。匹配到的最高角色生效，并在每次登录时检查。如果未列出任何组，新账户将获得上方的角色，之后由管理员修改。如果在“可查看”下列出了组，则只有所列组的成员才能登录。
auth-saving = 正在保存…
auth-save = 保存

## HttpsPanel.vue（设置 → 连接）

https-load-failed = 无法加载 HTTPS 设置：{ $error }
https-off-redirecting = HTTPS 已关闭。正在跳转到 http://
https-now-on = HTTPS 已开启
https-saved = 已保存。新的连接将使用新证书。
https-now-off = HTTPS 已关闭
https-http3-failed-toast = HTTP/3 无法启动：{ $error }
https-copied = 已复制
https-copy-failed = 无法复制：请选中后自行复制
https-mode-off = 关闭
https-mode-off-hint = 仅使用普通 http://。
https-mode-own = Hoard 自己的证书
https-mode-own-hint = Hoard 为此服务器的地址生成证书。在每台设备上安装一次 Hoard 的证书颁发机构，该设备上的浏览器就会信任它。
https-mode-custom = 我的证书
https-mode-custom-hint = 你已有的证书，例如来自 tailscale cert 或 Let’s Encrypt。浏览器无需安装任何东西即可信任它。
https-untrusted-own = 此浏览器尚未信任它用于 { $host }：请在下方安装 Hoard 的证书颁发机构。你会看到一个可以点击跳过的警告。
https-untrusted-uncovered = 此浏览器尚未信任它用于 { $host }：证书未涵盖此地址。你会看到一个可以点击跳过的警告。
https-untrusted = 此浏览器尚未信任它用于 { $host }。你会看到一个可以点击跳过的警告。
https-now = 当前状态
https-start-failed = HTTPS 已设置，但无法启动：{ $error }
https-on-port = HTTPS 已开启，与 http:// 使用相同的端口 { $port }。
https-using-it = 你当前正在使用它。
https-open = 打开 { $url }
https-trusted = 此浏览器信任它。
https-cert-for-before = 证书适用于{ " " }
https-cert-for-after = ，有效期至 { $date }。
https-cert-for-after-own = ，有效期至 { $date }（会自动续期）。
https-not-covered-before = 证书未涵盖你正在使用的地址{ " " }
https-not-covered-after = 。
https-is-off = HTTPS 已关闭：页面通过普通 http:// 提供。
https-addresses = 地址
https-addresses-hint = 证书涵盖 Hoard 找到的地址，以及你添加的任何地址（路由器为它分配的名称、Tailscale 名称）。
https-remove-name = 移除 { $name }
https-another-address = 其他地址
https-another-address-placeholder = 其他地址，例如 nas.local
https-add = 添加
https-your-cert = 你的证书
https-saved-cert-before = 已保存：
https-saved-cert-after = ，由 { $issuer } 颁发，有效期至 { $date }。粘贴新证书即可替换。
https-pem-before = 证书（含证书链）及其私钥，格式为 PEM 文本：即以{ " " }
https-pem-after = { " " }开头的文件。
https-certificate = 证书
https-choose-file = 选择文件…
https-private-key = 私钥
https-checked-before = 适用于{ " " }
https-checked-after = ，由 { $issuer } 颁发，有效期至 { $date }。
https-checked-after-days =
    ，由 { $issuer } 颁发，有效期至 { $date }（剩余 { $days ->
       *[other] { $days } 天
    }）。
https-self-signed = 这是自签名证书：浏览器会对此发出警告。
https-key-matches = 私钥与证书匹配。
https-where-to-get = 从哪里获取证书
https-get-tailscale-before = 为你的 tailnet 开启 HTTPS，然后在服务器上运行{ " " }
https-get-tailscale-after = ，并上传生成的两个文件。证书有效期为 90 天：请在到期前再次运行该命令并重新上传。
https-get-acme-title = Let’s Encrypt 或其他 ACME 客户端
https-get-acme-before = （certbot、acme.sh、lego），适用于你拥有的域名：上传{ " " }
https-get-acme-and = { " " }和{ " " }
https-get-acme-after = 。
https-get-proxy-title = 反向代理
https-get-proxy = （Caddy、Traefik、nginx、Cloudflare Tunnel），由它持有证书：在这里保持 HTTPS 关闭，让代理通过普通 http:// 将请求转发给 Hoard。
https-trying = 正在尝试…
https-apply = 应用
https-apply-hint = Hoard 会先试用新设置，如果不成功，则保留现有设置。
https-trust-title = 在你的设备上信任 Hoard
https-download-ca = 下载 hoard-ca.crt
https-trust-hint = 在每台设备上安装一次 Hoard 的证书颁发机构，该设备上的浏览器就会信任此服务器（以及 Hoard 日后为它生成的任何证书）。它只能为此 Hoard 担保。指纹（SHA-256）：
https-ca-windows = 打开文件 → 安装证书 → 当前用户 → 将所有的证书都放入下列存储 → 浏览 → 受信任的根证书颁发机构 → 完成。然后重新启动浏览器。
https-ca-macos = 打开文件，它会被添加到“钥匙串访问”中。在其中双击“{ $subject }”→ 信任 → 使用此证书时：始终信任。
https-ca-ios-title = iPhone 和 iPad
https-ca-ios = 在 Safari 中下载并允许。设置 → 已下载描述文件 → 安装。然后前往 设置 → 通用 → 关于本机 → 证书信任设置 → 开启“{ $subject }”。
https-ca-android = 设置 → 安全 → 更多安全设置 → 加密与凭据 → 安装证书 → CA 证书，然后选择该文件。（不同手机上的名称略有差异；可以在“设置”中搜索“证书”。）
https-ca-linux-1 = Debian 和 Ubuntu：
https-ca-linux-2 = 。Fedora 和 Arch：
https-ca-linux-3 = 。Linux 上的 Chrome 有自己的证书列表：设置 → 隐私和安全 → 安全 → 管理证书 → 导入。
https-ca-firefox = Firefox 有自己的证书列表：设置 → 隐私与安全 → 证书 → 查看证书 → 证书颁发机构 → 导入，并勾选“信任由此证书颁发机构来标识网站”。
https-ca-cli-title = hoard 命令行工具
https-ca-cli-before = 使用{ " " }
https-ca-cli-middle = ，或将路径设置到{ " " }
https-ca-cli-after = { " " }中。
https-options = 选项
https-redirect = 将其他设备上的浏览器从 http:// 转到 https://
https-redirect-hint = 只有页面加载会被转过去。hoard 命令行工具、Unreal worker、脚本和 AI 应用仍可通过 http:// 正常工作，服务器本机上的 http://localhost 也一样。
https-http3-hint-before = 在不稳定的 Wi-Fi 和手机上更快。需要 HTTPS，并且除 TCP 外还要开放 UDP 端口 { $port }（在 Docker 中：
https-http3-hint-after = ）。
https-http3-running = 正在运行。
https-http3-waits = HTTPS 开启后它就会启动。
https-http3-failed = HTTP/3 无法启动：{ $error }
https-lockout-title = 如果你被锁在门外
https-lockout-1 = { "" }
https-lockout-2 = { " " }在服务器本机上始终可以通过普通 http:// 访问。也可以在服务器的命令行中关闭 HTTPS，然后重新启动 Hoard：
https-lockout-3 = （在 Docker 中：
https-lockout-4 = ，然后运行{ " " }
https-lockout-5 = ）。

## AccountPanel.vue（设置 → 账户）

account-role-view = 可查看
account-role-view-hint = 搜索、预览和下载
account-role-edit = 可修改
account-role-edit-hint = 还可以添加、修改、打标签、加入合集和归档
account-role-admin = 管理员
account-role-admin-hint = 还可以管理设置、账户和维护，以及永久删除
account-source-internal = Hoard 密码
account-source-ldap = 其 { $directory } 密码
account-source-oauth = 使用 { $provider } 登录
account-source-word-directory = 目录服务
account-load-failed = 无法加载账户：{ $error }
account-failed = { $what }：{ $error }
account-password-failed = 无法修改你的密码
account-password-changed = 密码已修改。你的其他设备已退出登录。
account-add-failed = 无法添加 { $name }
account-added = { $name } 现在可以登录了。
account-role-failed = 无法修改 { $name } 的权限
account-remove-title = 移除 { $name }？
account-remove-body = 此人将在所有设备上退出登录，并且无法再次登录。其添加到资源库的内容不会被移除。
account-remove = 移除
account-remove-failed = 无法移除 { $name }
account-token-failed = 无法创建令牌
account-copied = 已复制
account-copy-token-failed = 无法复制：请选中令牌后自行复制
account-copy-failed = 无法复制：请选中后自行复制
account-revoke-title = 吊销“{ $name }”？
account-revoke-body = 从现在起，任何仍在使用它的程序都将被拒绝。
account-revoke = 吊销
account-revoke-failed = 无法吊销“{ $name }”
account-disconnect-title = 断开 { $app }？
account-disconnect-body = 从现在起它将被拒绝，必须重新连接才能使用 Hoard。
account-disconnect = 断开连接
account-disconnect-failed = 无法断开 { $app }
account-guest-failed = 无法更改此设置
account-never = 从未
account-you = 你
account-sign-out = 退出登录
account-signed-in-as-before = 当前登录身份：
account-signed-in-as-admin = ，管理员。
account-signed-in-as-edit = ，可以修改资源库。
account-signed-in-as-view = ，可以查看但不能修改任何内容。
account-app-signin = Hoard 应用会自动为你登录，因此没有密码。可在下方为引擎插件和 hoard 命令创建访问令牌。
account-password-at-provider = 你的密码由你登录时使用的提供方保管，因此请在那里修改。
account-password-at-directory = 你的密码由你登录时使用的目录服务保管，因此请在那里修改。
account-current-password = 当前密码
account-new-password = 新密码
account-new-password-again = 再次输入新密码
account-change-password = 修改密码
account-password-mismatch = 两次输入的新密码不一致。
account-password-rule = 至少 8 个字符。
account-people = 用户
account-people-hint = 每个用户可以只查看（搜索、预览和下载），也可以修改资源库（添加、编辑、打标签、加入合集和归档），或者再成为管理员（设置、账户和维护，以及永久删除）。
account-you-tag = （你）
account-signs-in-oauth = 通过 OAuth 提供方登录
account-signs-in-directory = 通过目录服务登录
account-since = 自 { $date } 起
account-what-can-do = { $name } 的权限
account-name = 用户名
account-how-sign-in = 登录方式
account-password-label = 密码（至少 8 个字符）
account-password-placeholder = 密码（8 个字符以上）
account-what-they-can-do = 权限
account-add = 添加
account-ai-apps = Claude 及其他 AI 应用
account-ai-apps-hint = 连接 Claude（或任何 MCP 应用）后，它可以为你搜索和获取资源，并为 Hoard 尚未处理的内容添加描述和标签。它会在首次使用时登录 Hoard，你需要在这里的浏览器中授权。
account-mcp-address = Hoard 的 MCP 地址
account-copy = 复制
account-claude-code-1 = 运行{ " " }
account-claude-code-2 = ，然后运行{ " " }
account-claude-code-3 = { " " }进行登录。{ " " }
account-claude-code-4 = { " " }可以描述一个资源包。
account-claude-ai = 设置 → 连接器 → 添加自定义连接器，粘贴地址，然后点击“连接”。这些应用从 Anthropic 的服务器发起连接，因此 Hoard 必须能通过 HTTPS 从互联网访问。
account-claude-ai-local = 设置 → 连接器 → 添加自定义连接器，粘贴地址，然后点击“连接”。这些应用从 Anthropic 的服务器发起连接，因此 Hoard 必须能通过 HTTPS 从互联网访问（此地址不行：请在“设置 → 连接”中设置公开地址）。
account-connection-meta = 用户 { $user }，连接于 { $since } · 上次使用：{ $used }
account-no-apps = 尚未连接任何应用。
account-live = 正在使用 Hoard
account-live-hint = 最近 10 分钟内向 Hoard 请求过内容的应用。列表会自动更新。
account-live-none = 最近 10 分钟内没有任何应用向 Hoard 发出请求。
account-live-last = 最近：{ $what } · { $when } · 请求数：{ $calls }
account-tokens = 访问令牌
account-tokens-hint-1 = 供{ " " }
account-tokens-hint-2 = { " " }命令行工具、脚本、MCP 客户端和 Unreal worker 使用：它们以{ " " }
account-tokens-hint-3 = { " " }的形式发送令牌。令牌可以执行你选择的操作，但不能超出你自己的权限，也永远不能管理账户或令牌。仅可查看的令牌就足以搜索和下载，以及同步项目。
account-tokens-admin = 你可以看到所有人的令牌和已连接的应用；其他人只能看到自己的。
account-made-before = { "" }
account-made-after = ：请立即复制。它不会再次显示。
account-new-token = 新令牌
account-cli-1 = 用于 CLI：
account-cli-2 = ，或将其设置到{ " " }
account-cli-3 = { " " }中。
account-done = 完成
account-token-meta = 创建于 { $made } · 上次使用：{ $used }
account-token-meta-by = 由 { $by } 创建于 { $made } · 上次使用：{ $used }
account-no-tokens = 还没有令牌。
account-token-name = 用途
account-token-name-placeholder = 用途，例如“Unreal worker”
account-token-role = 权限
account-make-token = 创建令牌
account-guests = 访客
account-guests-allow = 允许任何能访问此服务器的人无需登录即可浏览
account-guests-hint = 访客可以搜索、预览和下载，但不能添加、修改或删除任何内容，也看不到设置。

## SignIn.vue

signin-create-title = 创建你的账户
signin-title = 登录
signin-setup-lede = 此服务器还没有账户。你在这里创建的账户拥有全部权限；之后可以在“设置 → 账户”中为其他人添加账户。
signin-overlay-lede = 这里所有人都可以浏览，但进行更改需要账户。
signin-with = 使用 { $provider } 登录
signin-or-password = 或使用用户名和密码
signin-username = 用户名
signin-name = 用户名
signin-new-password = 新密码
signin-password = 密码
signin-password-rule = 至少 8 个字符。
signin-confirm-password = 确认密码
signin-mismatch = 两次输入的密码不一致。
signin-setup-code = 设置代码
signin-setup-code-hint-before = 在{ " " }
signin-setup-code-hint-after = { " " }中为此服务器设置的代码。
signin-creating = 正在创建…
signin-signing-in = 正在登录…
signin-create = 创建账户
signin-submit = 登录
signin-keep-browsing = 继续浏览
signin-directory-hint = 请使用你的 { $directory } 用户名和密码。
signin-forgot-1 = 忘记了 Hoard 密码？请在服务器上运行{ " " }
signin-forgot-2 = （在 Docker 中运行{ " " }
signin-forgot-3 = ）来设置新密码。

## SetupWizard.vue（首次访问新服务器）

setup-title = 你的资源库要放在哪里？
setup-lede = 在服务器上选择一个文件夹，让 Hoard 在其中保存你的资源包；也可以指向一个已有资源包的文件夹。之后可以在“设置”中添加更多位于其他磁盘上的位置。
setup-lede-desktop = 选择一个文件夹，让 Hoard 在其中保存你的资源包；也可以指向一个已有资源包的文件夹。之后可以在“设置”中添加更多位于其他磁盘上的位置。
setup-unreachable = 无法连接到服务器：{ $error }
setup-unreachable-desktop = Hoard 没有响应：{ $error }
setup-try-again = 重试
setup-loading = 正在加载…
setup-start = 创建资源库
setup-ready = 你的资源库已就绪
setup-managed-before = 新的资源包将放入{ " " }
setup-managed-after = 。将文件、文件夹或压缩包拖放到页面上的任意位置，即可添加你的第一批资源包。
setup-watched-before = Hoard 正在监视{ " " }
setup-watched-after = ；找到资源包后就会显示出来。如果还想上传，请在“设置 → 资源库位置”中添加一个由 Hoard 管理的资源库。
setup-open = 打开资源库

## ConnectApp.vue（Claude 等应用请求登录）

connect-title = 连接 { $app }？
connect-lede-1 = { " " }将能够以{ " " }
connect-lede-2 = { " " }的身份搜索、下载、描述和标注此 Hoard 中的所有内容。它无法管理账户或访问令牌。
connect-back-before = 你将返回到{ " " }
connect-back-after = 。可以随时在“设置 → 账户”中断开它。
connect-allow = 允许
connect-cancel = 取消
connect-failed = 无法连接此应用
connect-go-library = 前往资源库
connect-checking = 正在检查…

## InsecureNotice.vue（在普通 http:// 页面上方显示的提示）

insecure-text = 当前使用的是普通 http://，有几项功能需要 HTTPS：使用 WebGPU 的 3D、现代剪贴板、将 Hoard 安装为应用，以及 Claude 连接器。
insecure-switch = 切换到 https://
insecure-set-up = 设置 HTTPS
insecure-dismiss = 不再显示

## FormatWizard.vue（设置 → 商店文件 → 教 Hoard 读取商店的文件）

formatwizard-too-big = 此文件太大，无法作为商店文件读取。
formatwizard-not-json = 此文件不是有效的 JSON，因此以文本形式显示。
formatwizard-read-failed = 无法读取文件：{ $error }
formatwizard-back-title = 返回表单？
formatwizard-back-body = 在 TOML 中所做的更改将会丢失：表单会根据它已有的内容重新生成格式。
formatwizard-back-to-form = 返回表单
formatwizard-miss-name = 此格式的文件名中没有 { $file }。请将其添加到“文件”中。
formatwizard-miss-not-the-stores = 此文件中没有它用来识别该商店文件的内容。请修改“识别依据”。
formatwizard-miss-no-title = 它在此文件中找不到标题。请选取标题。
formatwizard-miss-short-name = 文件名不符
formatwizard-miss-short-not-the-stores = 不是该商店的文件
formatwizard-miss-short-no-title = 无标题
formatwizard-saved-packs =
    已保存 { $file }。{ $packs ->
       *[other] { $packs } 个资源包
    }正在重新读取。
formatwizard-saved = 已保存 { $file }。
formatwizard-save-failed = 无法保存格式：{ $error }
formatwizard-zip-failed = 无法生成 zip：{ $error }
formatwizard-stop-title = 停止教 Hoard 读取此文件？
formatwizard-stop-body = 格式不会被保存。
formatwizard-stop = 停止
formatwizard-label = 格式向导
formatwizard-changing = 正在修改 { $name }
formatwizard-starting-from = 以 { $name } 为基础
formatwizard-title = 教 Hoard 读取商店的文件
formatwizard-place-1 = { "" }
formatwizard-place-2 = （位于资源包{ " " }
formatwizard-place-3 = { " " }中）
formatwizard-edit-toml = 以 TOML 编辑
formatwizard-file-reference = 文件内容，仅供参考。
formatwizard-click-json = 在文件中点击{ $field }。
formatwizard-click-text = 在文件中点击{ $field }，或将其选中。
formatwizard-pick-first = 先在右侧选择一个字段，然后在这里点击它的值。
formatwizard-loading = 正在加载…
formatwizard-the-format = 格式
formatwizard-syntax-before = 语法说明见{ " " }
formatwizard-syntax-after = 。
formatwizard-store = 商店
formatwizard-store-placeholder = 商店名称，例如 Kenney
formatwizard-files = 文件
formatwizard-recognize = 识别依据
formatwizard-recognize-json = 只有该商店的文件才有的键
formatwizard-recognize-text = 只有该商店的文件才有的一行
formatwizard-recognize-hint = 每行一种方式；满足任意一种即可。当其他文件同名时需要填写。
formatwizard-seller = 卖家
formatwizard-seller-placeholder = 文件中未说明时使用
formatwizard-tags = 标签
formatwizard-tags-placeholder = 适用于它描述的所有内容，以逗号分隔
formatwizard-fields = 字段
formatwizard-selector = { $field }选择器
formatwizard-clear = 清除{ $field }
formatwizard-pick = 选取
formatwizard-what-it-reads = 读取结果
formatwizard-read-title = 标题
formatwizard-read-seller = 卖家
formatwizard-read-license = 许可证
formatwizard-read-url = 商店页面
formatwizard-read-kind = 类型
formatwizard-read-tags = 标签
formatwizard-read-origin = 制作地点
formatwizard-read-size = 大小
formatwizard-also = { $format } 也会读取此文件。此格式将优先尝试。
formatwizard-library =
    { $packs ->
       *[other] { $packs } 个资源包
    }中有 { $files ->
       *[other] { $files } 个文件
    }的文件名符合要求。在尝试的 { $tried } 个文件中，它能读取 { $read } 个。
formatwizard-library-none = 资源库中没有文件名符合要求的文件。
formatwizard-share = 分享
formatwizard-share-hint = 一个 zip 压缩包，包含此格式、作为样本的此文件以及样本的读取结果，按 Hoard 仓库的结构排列，提交后所有人都能用上。请先删除样本中不应分享的内容。
formatwizard-sample = 要分享的样本
formatwizard-download = 下载 { $file }
formatwizard-close = 关闭
formatwizard-use = 在此使用
formatwizard-cancel = 取消
settings-unreal-editor-what = 为导出工作进程安装 Unreal Editor。请单独运行该进程以导出资源。
settings-unreal-download = 将 Epic 的 Linux x86-64 编辑器一次性下载到 Hoard 的工具文件夹。它会占用数十 GB 的磁盘空间，并需要一个 Epic 已准许加入的 GitHub 账户（见“GitHub 登录”）。
settings-unreal-github = GitHub 登录
settings-unreal-github-why = Epic 将 Linux 版编辑器作为私有镜像发布在 GitHub 的软件包注册表（ghcr.io）上，Hoard 使用 Hoard 自带的 crane 从那里下载。GitHub 只把它提供给 Epic 已准许加入的账户，因此 Hoard 需要一个账户来下载。Hoard 将令牌与其他设置一起保存，不会用于其他任何用途。
settings-unreal-github-step-link = 将你的 GitHub 账户关联到你的 Epic Games 账户。
settings-unreal-github-step-link-go = 如何关联
settings-unreal-github-step-invite = 接受 GitHub 通过电子邮件发来的 EpicGames 组织邀请（可能需要几分钟才能收到）。
settings-unreal-github-step-token = 创建一个只勾选 read:packages 的 GitHub 令牌（classic）。
settings-unreal-github-step-token-go = 创建令牌
settings-unreal-github-step-enter = 在此输入你的 GitHub 用户名和令牌，然后选择“自动”。
settings-unreal-github-user = GitHub 用户名
settings-unreal-github-token = 令牌
settings-unreal-github-token-kept = 已保存（输入新令牌即可替换）
settings-unreal-github-save = 保存登录信息
settings-unreal-github-forget = 忘记
settings-unreal-github-saved = 已保存 GitHub 登录信息；正在重试下载
settings-unreal-github-forgotten = 已忘记 GitHub 登录信息
settings-unreal-github-from-server = 此服务器另外配置了自己的登录信息（GHCR_TOKEN_FILE 或 DOCKER_CONFIG）；当此处未保存登录信息时将使用它。
settings-unreal-image = Unreal 版本或镜像
settings-editor-path = 输入已安装的 { $tool } 可执行文件路径。仅在 Docker 中支持自动安装。
error-unreal-image = 请输入 Unreal 5 版本、Epic dev/dev-slim 版本标签或 Epic SHA-256 镜像引用。
