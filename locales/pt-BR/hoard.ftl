## Erros do servidor

error-account-not-found = conta não encontrada
error-asset-gone = asset não encontrado
error-asset-not-found = asset { $id } não encontrado
error-blob-not-found = blob { $hash } não encontrado
error-collection-not-found = coleção { $id } não encontrada
error-connection-not-found = conexão não encontrada
error-derivative-not-found = derivado { $kind } de { $id } não encontrado
error-file-in-pack-not-found = { $path } não encontrado no pacote { $id }
error-format-not-found = formato { $file } não encontrado
error-frame-sheet-aseprite-only = só arquivos do Aseprite têm folha de quadros
error-frame-sheet-frames = isto tem um quadro só, ou quadros demais para organizar
error-location-not-found = local { $id } não encontrado
error-maintenance-task-not-found = tarefa de manutenção { $id } não encontrada
error-open-upload-not-found = envio aberto { $id } não encontrado
error-original-not-found = arquivo original de { $id } não encontrado
error-pack-not-found = pacote { $id } não encontrado
error-preview-too-big = esta imagem é grande demais para fazer uma prévia leve
error-project-not-found = projeto { $name } não encontrado
error-saved-search-not-found = busca salva { $id } não encontrada
error-sign-in-first = entre primeiro (ou envie um token de acesso)
error-those-assets-not-found = esses assets não foram encontrados
error-thumbnail-not-found = miniatura de { $id } não encontrada
error-token-not-found = token não encontrado
error-upload-not-found = envio { $id } não encontrado
error-pack-needs-name = um pacote precisa de um nome
error-no-library = ainda não há um local da biblioteca para guardar arquivos: adicione um gerenciado em Configurações → Locais da biblioteca
error-upload-failed = o envio falhou: { $reason }
error-upload-no-files = nenhum arquivo no envio
error-analyze-nothing-to-do = nada a fazer: rules=false e o LLM está desligado
error-too-many-assets = assets demais em uma só solicitação
error-too-many-items = itens demais em uma só solicitação
error-not-a-hash = não é um hash blake3 em hexadecimal
error-merge-into-tag = into precisa ser uma tag
error-nothing-to-merge = nada para mesclar
error-selection-size = informe entre 1 e { $max } ids de assets
error-collection-needs-name = uma coleção precisa de um nome
error-collection-exists = já existe uma coleção chamada { $name }
error-collection-empty = a coleção { $name } está vazia
error-conversion-failed = não foi possível converter para { $format }: { $reason }
error-max-size = max precisa ser uma potência de dois de 64 a 16384
error-replace-ext = ext precisa ser uma extensão de arquivo curta
error-empty-file = arquivo vazio
error-replace-duplicate = o arquivo convertido duplica um que já está no pacote; o pacote foi mesclado a ele
error-ai-address = O endereço da IA deve começar com http:// ou https://
error-local-llm-action = action é list, download ou delete, e as duas últimas precisam de um modelo
error-role = Um papel é view, edit ou admin
error-admin-only = só um administrador pode fazer isso
error-view-only = esta conta pode ver, mas não pode mudar nada: um administrador pode mudar isso em Configurações → Conta
error-portal-only = entre no portal para gerenciar contas e tokens
error-not-from-portal = esta solicitação não veio do portal do Hoard
error-account-name = Escolha um nome de até 64 caracteres
error-password-length = Use uma senha de pelo menos { $min } caracteres
error-too-many-codes = Códigos errados demais: aguarde { $seconds } segundos
error-already-set-up = Já existe uma conta: entre
error-too-many-tries = Tentativas erradas demais: aguarde { $seconds } segundos
error-directory-unavailable = O diretório não conseguiu verificar sua senha: { $reason }
error-password-elsewhere = Sua senha é mantida pelo diretório ou provedor com que você entra
error-current-password = Sua senha atual não está certa
error-account-source = Uma conta entra com o Hoard (internal), um diretório (ldap) ou OAuth
error-account-exists = Já existe uma conta com esse nome
error-own-role = Outro administrador pode mudar o seu papel
error-remove-self = Você não pode remover a própria conta enquanto estiver conectado a ela
error-token-name = Dê ao token o nome do que vai usá-lo, em até 80 caracteres
error-token-role = Um token não pode fazer mais do que a conta que o cria
error-wrong-setup-code = Esse não é o código de configuração dado ao servidor em HOARD_SETUP_CODE
error-wrong-password = Esse nome e essa senha não conferem
error-folder-missing = { $path } não está lá (um disco que não está montado?)
error-s3-path = um local S3 tem a forma s3://bucket
error-not-built = esta cópia do Hoard foi compilada sem { $feature }
error-openai-auth-unreachable = não foi possível acessar auth.openai.com
error-openai-unexpected = resposta inesperada da OpenAI
error-openai-answered = A OpenAI respondeu { $status }: { $reason }
error-chatgpt-id-token-not-jwt = o token de ID não é um JWT
error-chatgpt-id-token-issuer = o token de ID é de { $issuer }, não de { $expected }
error-chatgpt-id-token-audience = o token de ID é para outro app
error-chatgpt-id-token-expired = o token de ID expirou
error-chatgpt-id-token-nonce = o token de ID não pertence a este login
error-chatgpt-no-state = o endereço não tem state; copie o endereço inteiro
error-chatgpt-not-started = este login não foi iniciado aqui, ou foi iniciado há mais de 15 minutos. Comece de novo pelas Configurações.
error-chatgpt-canceled = o login foi cancelado
error-chatgpt-refused = O ChatGPT recusou o login: { $reason }
error-chatgpt-no-code = o endereço não tem código de login; copie o endereço inteiro
error-chatgpt-other-registration = O ChatGPT respondeu para outro registro deste servidor; comece o login de novo
error-chatgpt-other-registration-desktop = O ChatGPT respondeu para outra cópia do Hoard; comece o login de novo
error-chatgpt-not-registered = O ChatGPT não terminou de registrar o Hoard; comece o login de novo
error-chatgpt-code-used = o código de login já foi usado ou expirou; entre de novo
error-chatgpt-refused-code = O ChatGPT recusou o login ({ $code })
error-chatgpt-no-id-token = O ChatGPT não enviou um token de ID
error-chatgpt-plan-not-allowed = Login feito, mas o uso do plano não foi permitido. Entre de novo e permita, ou use uma chave de API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = não há login no ChatGPT; entre em Configurações → Tags por IA
error-chatgpt-refresh-later = O ChatGPT não conseguiu renovar o login agora ({ $code }); vamos tentar de novo mais tarde
error-chatgpt-signed-out-problem = O ChatGPT desconectou este servidor ({ $code }). Entre de novo para continuar gerando tags com o seu plano.
error-chatgpt-signed-out-problem-desktop = O ChatGPT desconectou o Hoard ({ $code }). Entre de novo para continuar gerando tags com o seu plano.
error-chatgpt-signed-out = O ChatGPT desconectou este servidor ({ $code }); entre de novo em Configurações → Tags por IA
error-chatgpt-signed-out-desktop = O ChatGPT desconectou o Hoard ({ $code }); entre de novo em Configurações → Tags por IA
error-openai-api-unreachable = não foi possível acessar api.openai.com
error-openai-broke-off = a resposta da OpenAI foi interrompida
error-chatgpt-kept-refusing = A OpenAI continuou recusando o login do ChatGPT; entre de novo em Configurações → Tags por IA
error-chatgpt-request-failed = a solicitação falhou
error-chatgpt-cut-off = a resposta foi cortada ({ $reason })
error-chatgpt-cut-off-no-reason = a resposta foi cortada (sem motivo informado)
error-openai-ended-early = a resposta da OpenAI terminou antes de estar completa
error-chatgpt-usage-limit = o uso do seu plano do ChatGPT para o Hoard acabou por enquanto. Veja ou mude o limite em { $url }
error-chatgpt-not-eligible = esta conta ou workspace do ChatGPT não pode usar o plano em outros apps. Use uma chave de API.
error-chatgpt-usage-unavailable = O ChatGPT não conseguiu verificar o uso do plano agora; vamos tentar de novo mais tarde
error-chatgpt-unsupported = O uso pelo plano do ChatGPT não aceita { $param } com este modelo
error-chatgpt-unsupported-images = O uso pelo plano do ChatGPT não aceita { $param } com este modelo; escolha um modelo que aceite imagens, ou desligue o envio de miniaturas
error-chatgpt-unsupported-part = O uso pelo plano do ChatGPT não aceita parte da solicitação com este modelo
error-chatgpt-port = port é a porta pela qual o navegador acessa o Hoard
error-chatgpt-paste-address = cole o endereço inteiro, começando com http://127.0.0.1
error-too-many-hashes = no máximo { $max } hashes por vez
error-credits-pack = pacote { $key }
error-credits-collection = coleção { $key }
error-format-too-long = o formato é longo demais
error-not-started = o servidor não foi iniciado
error-not-started-desktop = O Hoard não foi iniciado
error-not-format-file = não é um arquivo de formato: { $name }
error-format-name = o nome precisa de uma letra ou dígito de A a Z ou de 0 a 9
error-format-miss-name = os arquivos do formato não aceitam { $file }
error-format-miss-store = o formato não aceita { $file } como da loja (veja require)
error-format-miss-title = o formato não encontra um título em { $file }
error-cant-download-as = arquivos .{ $ext } não podem ser baixados como { $format }
error-only-images-resize = só imagens podem ser redimensionadas
error-https-no-certificate = Cole ou envie primeiro o seu certificado e a chave dele
error-https-no-cert-found = Nenhum certificado encontrado: cole o texto que começa com -----BEGIN CERTIFICATE-----
error-https-no-key-found = Nenhuma chave privada encontrada: cole o texto que começa com -----BEGIN PRIVATE KEY-----
error-https-several-keys = Há mais de uma chave privada: cole só a que acompanha o certificado
error-https-key-password = A chave está protegida por senha: forneça-a sem senha (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = O certificado não é um PEM válido
error-https-key-not-pem = A chave privada não é um PEM válido
error-https-key-mismatch = A chave não pertence ao certificado. Se você forneceu uma cadeia, coloque o certificado do seu servidor primeiro
error-https-unusable = O certificado ou a chave não podem ser usados
error-https-expired = Este certificado expirou em { $date }
error-https-not-yet-valid = Este certificado só é válido a partir de { $date }
error-https-unreadable = Esse não é um certificado que o Hoard consiga ler
error-https-bad-name = “{ $name }” não é um endereço que um certificado possa conter
error-https-test-timed-out = a conexão de teste esgotou o tempo
error-https-test-failed = a conexão de teste falhou
error-https-no-ca = O Hoard ainda não criou sua autoridade certificadora
error-https-unavailable = HTTPS não está disponível neste servidor
error-https-certificate-mode = Um certificado acompanha a opção “Meu certificado”
error-https-test = O certificado não funcionou em uma conexão de teste: { $reason }
error-nothing-to-retry = não há nada ali que possa ser tentado de novo
error-ai-unreachable = não foi possível acessar { $url }
error-ai-key-refused = { $url } recusou a chave de API ({ $status })
error-ai-answered = { $url } respondeu { $status }: { $reason }
error-ai-not-json = { $url }/models não respondeu com JSON
error-model-server-refused = o servidor de modelos: { $reason }
error-no-model-server = nenhum servidor de modelos responde em { $url }
error-not-llama-router = { $url } não é um servidor llama.cpp em modo router, então não pode baixar modelos
error-model-download = não foi possível baixar { $model }: { $reason }
error-model-delete = não foi possível excluir { $model }: { $reason }
error-unknown-kind = tipo de asset desconhecido { $kind }
error-watched-is-folder = um local monitorado é uma pasta neste servidor
error-cant-reach = não foi possível acessar { $path }: { $reason }
error-folder-full-path = informe o caminho completo da pasta
error-cant-create = não foi possível criar { $path }
error-cant-write = não foi possível gravar em { $path }
error-cant-read = não foi possível ler { $path }
error-overlaps-location = isso se sobrepõe ao local { $name } ({ $path })
error-overlaps-data = isso se sobrepõe à pasta de dados do Hoard ({ $path })
error-overlaps-cache = isso se sobrepõe à pasta de cache do Hoard ({ $path })
error-not-watched = { $name } não é uma pasta monitorada
error-watched-no-kinds = uma pasta monitorada não recebe arquivos novos, então não tem tipos para mudar
error-keep-catch-all = em vez disso, faça de outro local o local para todo o resto
error-location-holds-files = { $name } ainda contém arquivos: mova-os primeiro para os outros locais (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } não pode ser acessado, então o Hoard não sabe se ele ainda contém arquivos
error-relocate-running = já há arquivos sendo movidos entre locais; tente de novo quando isso terminar
error-watched-not-moved = { $name } é uma pasta monitorada: o Hoard não move os arquivos dela
error-no-other-location = não há outro local da biblioteca para onde mover os arquivos: adicione um primeiro
error-full-path = informe um caminho completo
error-pick-folder = escolha uma pasta
error-task-running = “{ $task }” já está em execução
error-token-account-gone = a conta deste token não existe mais, então ele não tem favoritos nem buscas
error-favorites-too-many = no máximo 10.000 por vez
error-saved-search-name = uma busca salva precisa de um nome de no máximo 80 caracteres
error-saved-search-too-long = essa busca é longa demais para salvar
error-texture-pattern-map = O padrão de nome das texturas precisa de { "{" }map{ "}" }, senão todos os mapas de um material teriam o mesmo nome
error-texture-pattern-fields = O padrão de nome das texturas pode usar { "{" }name{ "}" }, { "{" }map{ "}" } e { "{" }size{ "}" }
error-texture-name-chars = Nomes de textura não podem ter < > : " / \ | ? nem *
error-texture-map-type = Não há tipo de mapa chamado { $role }
server-role-base-color = cor
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = rugosidade
server-role-gloss = brilho
server-role-metalness = metalicidade
server-role-ao = oclusão de ambiente
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = altura
server-role-bump = relevo
server-role-cavity = cavidade
server-role-specular = especular
server-role-opacity = opacidade
server-role-emissive = emissão
server-role-translucency = translucidez
error-texture-suffix-missing = Dê um sufixo aos mapas de { $map }
error-texture-suffix-chars = O sufixo dos mapas de { $map } não pode ter < > : " / \ | ? nem *
error-texture-ending-missing = Cada terminação personalizada precisa da terminação a ser procurada
error-texture-ending-chars = Terminações personalizadas não podem ter < > : " / \ | ? nem *
error-texture-ending-twice = { $ending } aparece duas vezes nas suas terminações
error-oauth-no-client-id = Este link não tem client_id.
error-oauth-unknown-app = Este app não está registrado aqui (ou não está mais). Comece a conexão de novo pelo app.
error-oauth-redirect = O app pediu para voltar a um endereço que ele não registrou.
error-oauth-response-type = O Hoard só fornece códigos de login (response_type=code).
error-oauth-pkce = O app precisa usar PKCE com S256 para entrar no Hoard.
error-light-preview-images = só imagens têm prévia leve
error-no-light-preview = sem prévia leve: { $reason }
error-no-frame-sheet = sem folha de quadros: { $reason }
error-thumbnail-source = origem de miniatura desconhecida: { $source }
error-thumbnail-too-big = essa imagem é grande demais para uma miniatura
error-thumbnail-models-only = só modelos aceitam uma miniatura desenhada pelo navegador
error-not-a-picture = não é uma imagem que o Hoard consiga ler: { $reason }
error-project-name = um projeto precisa de um nome de no máximo 200 caracteres
error-project-too-big = no máximo um milhão de assets em um projeto
error-project-too-many-at-once = no máximo 100.000 por vez
error-server-name = Escolha um nome de até { $max } caracteres, em uma linha
error-tool-setting = { $tool }: escolha desligado, automático ou informe um caminho
error-ai-model = Escolha o modelo que o serviço de IA deve usar
error-public-address = O endereço público deve começar com http:// ou https://
error-update-hook-address = O endereço a chamar para atualizações deve começar com http:// ou https://
error-update-hook-auth = O cabeçalho Authorization não pode conter quebras de linha nem outros caracteres de controle
error-no-soundfont = Não há SoundFont em { $path } no servidor
error-no-soundfont-desktop = Não há SoundFont em { $path }
error-workers = Trabalhadores em segundo plano: entre 1 e { $max }
error-ai-batch = Assets por solicitação à IA: entre 1 e 200
error-conversion-cache = Downloads convertidos: pelo menos 100 MB
error-conversion-keep-days = Regras de conversão: manter os originais de 1 a { $max } dias
error-conversion-target = Regras de conversão: o Hoard não consegue converter arquivos para { $format }
error-conversion-no-source = Regras de conversão: cada regra precisa de pelo menos um formato para converter
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Regras de conversão: arquivos .{ $ext } não podem ser convertidos para { $format }
error-conversion-twice = Regras de conversão: .{ $ext } está em duas regras
error-conversion-cant-undo = Este arquivo não foi convertido por uma regra, ou o original dele já foi excluído.
error-conversion-path-taken = Agora há outro arquivo em { $path }, então o original não pode voltar para lá.
error-limits-zero = Os limites não podem ser zero
error-settings-object = esperava um objeto JSON de configurações
error-bad-setting = configuração inválida: { $reason }
error-auth-default-role = O papel padrão é view, edit ou admin
error-auth-session-hours = As sessões duram entre 1 hora e um ano
error-auth-settings = configurações de autenticação inválidas: { $reason }
error-ldap-settings = configurações de LDAP inválidas: { $reason }
error-ldap-no-server = Adicione o endereço do servidor de diretório
error-ldap-base-dn = Diga onde ficam as entradas das pessoas (o DN base)
error-ldap-username-attribute = Diga qual atributo contém o nome com que as pessoas entram
error-ldap-bind-dn = Informe a conta que procura as pessoas, ou procure-as anonimamente
error-ldap-ca-cert = A autoridade certificadora não é um certificado PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = O endereço de um servidor de diretório está vazio
error-ldap-server-form = { $server }: o endereço de um servidor de diretório é host:porta, ldap://… ou ldaps://…
error-ldap-not-starttls = { $server }: isso é LDAPS, não StartTLS
error-ldap-no-certificate = não há certificado nele
error-ldap-password-again = Informe de novo a senha da conta que procura as pessoas: a salva só é enviada aos servidores para os quais foi informada
error-ldap-unreachable = nenhum servidor de diretório pôde ser acessado ({ $reasons })
error-ldap-no-anonymous = o diretório não permite procurar pessoas anonimamente ({ $reason })
error-ldap-bind-failed = a conta que procura as pessoas não conseguiu entrar ({ $reason })
error-ldap-search = procurando { $name } em { $base }
error-ldap-group-search = procurando os grupos de { $name } em { $base }
error-ldap-several = { $count } entradas no diretório correspondem a { $name }: restrinja o filtro de usuários
error-ldap-password-check = o diretório não quis verificar a senha de { $name } ({ $reason })
error-ldap-wrong-password = O diretório não conhece { $name } com essa senha
error-oauth-client-id = Informe o client ID que o provedor criou para o Hoard
error-oauth-addresses = Os endereços de autorização e de token devem começar com https://
error-oauth-userinfo-address = O endereço de informações do usuário deve começar com https://
error-oauth-username-claim = Diga qual claim contém o nome da pessoa
error-oauth-secret-again = Informe o client secret de novo: o salvo só é enviado ao endereço de token para o qual foi informado
error-oauth-issuer = O issuer deve começar com https://
error-oauth-discovery-unreachable = Não foi possível acessar { $url }: { $reason }
error-oauth-discovery-status = { $url } respondeu { $status }
error-oauth-discovery-bad = { $url } não é um documento de descoberta
error-oauth-discovery-incomplete = { $url } não informa os endereços de autorização e de token
error-signin-no-group = { $name } não está em um grupo que pode usar o Hoard
error-signin-other-source = Já existe uma conta do Hoard chamada { $name } que entra de outra forma
error-signin-other-subject = Já existe uma conta do Hoard chamada { $name } para outra pessoa no provedor
error-signin-no-account = { $name } ainda não tem conta no Hoard: um administrador pode adicioná-la em Configurações → Conta
error-signin-bad-name = Esse nome não pode ser uma conta do Hoard (até 64 caracteres)
error-oauth-off = O login com OAuth não está ativado
error-oauth-bad-authorization-address = o endereço de autorização não é válido
error-oauth-refused = O provedor não fez o seu login: { $reason }
error-oauth-no-code = O provedor mandou você de volta sem um login
error-oauth-expired = Esse login demorou demais ou foi iniciado em outro lugar: tente de novo
error-oauth-no-claim = O provedor não disse quem você é (sem a claim { $claim })
error-oauth-no-sub = O provedor não disse quem você é (sem a claim sub ou id)
error-oauth-token-unreachable = não foi possível acessar o endereço de token do provedor
error-oauth-token-refused = O provedor não quis concluir o login: { $reason }
error-oauth-no-access-token = o provedor não enviou um token de acesso
error-oauth-userinfo-unreachable = não foi possível acessar o endereço de informações do usuário do provedor
error-oauth-userinfo-refused = O provedor não quis dizer quem entrou ({ $status })
error-oauth-userinfo-not-json = as informações do usuário do provedor não são JSON
error-format-no-name = name está vazio
error-format-no-files = files está vazio: diga quais arquivos o formato lê, como ["*.json"]
error-format-empty-selector = vazio
error-format-unopened-bracket = um colchete de fechamento sem um de abertura
error-format-unclosed-bracket = um colchete não foi fechado
error-format-empty-step-dots = um passo vazio (dois pontos seguidos, ou um ponto em uma das pontas)
error-format-unclosed-except = !( não foi fechado
error-format-unclosed-filter = [ não foi fechado
error-format-after-filters = { $rest } depois da chave e dos filtros de { $step }
error-format-except-needs-glob = !(…) vem depois de uma chave com * nela
error-format-except-not-glob = !(…) vem depois de uma chave com * nela, não de { $key }
error-format-empty-step = um passo vazio
error-format-not-a-filter = [{ $filter }] não é um filtro: use [], [key=value|other], [array], [string], [number], [bool] ou [object]
error-format-filter-needs-value = [{ $filter }] precisa de uma chave e de um valor
error-store-search-empty = nada para buscar; digite o nome do pacote
error-update-hook-not-set = defina primeiro um endereço a chamar
error-upload-stalled = o envio de { $path } travou; envie essa parte de novo
error-upload-busy = { $path } já está sendo gravado
error-upload-incomplete = { $count } arquivos estão incompletos, por exemplo { $files }
error-upload-no-files-listed = nenhum arquivo
error-upload-not-part = { $path } não faz parte do envio { $id }, ou o envio está fechado
error-bad-path = caminho inválido
error-upload-part-offset = { $offset } não é onde começa uma parte de { $path }
error-upload-interrupted = envio interrompido: { $reason }
error-upload-part-long = a parte { $part } de { $path } tem mais de { $len } bytes
error-upload-part-short = parte { $part } de { $path }: recebidos { $got } bytes, esperados { $len }
error-upload-file-long = { $path } é maior do que o declarado
error-upload-file-short = { $path }: recebidos { $got } bytes, esperados { $len }
error-upload-complete = o envio já está completo
error-already-imported = já importado como "{ $name }"

## Textos do servidor

server-store-place-readme = README
server-store-place-folders = pastas
server-store-why-linked = Com link em { $place } do pacote
server-store-why-same-title = Mesmo título que { $place } do pacote
server-store-why-by = De { $author }, citado em { $place } do pacote
server-store-why-meshes = Mais ou menos tantas malhas quanto o pacote ({ $count })
server-store-why-textures = Mais ou menos tantas texturas quanto o pacote ({ $count })
server-chatgpt-revoke-failed = Você saiu aqui, mas não foi possível avisar a OpenAI. Para ter certeza, desconecte o Hoard no ChatGPT em Configurações → Segurança e login → Fazer login com o ChatGPT.
server-chatgpt-signed-in-title = Conectado ao ChatGPT
server-chatgpt-signed-in = O Hoard está conectado. As tags por IA já podem usar o seu plano do ChatGPT.
server-chatgpt-signed-in-as = O Hoard está conectado como { $email }. As tags por IA já podem usar o seu plano do ChatGPT.
server-chatgpt-not-finished-title = Login não concluído
server-chatgpt-other-computer = Se você começou o login pelo Hoard em outro computador, copie o endereço desta página na barra de endereços e cole-o nas Configurações do Hoard de lá.
server-back-to-settings = Voltar às Configurações
server-issues-pack = Pacotes que não puderam ser adicionados
server-issues-pack-blurb = Exclua o pacote e adicione-o de novo, ou abra-o para ver o que ele tem.
server-issues-damaged = Arquivos guardados danificados ou que sumiram
server-issues-damaged-blurb = Encontrados por “Verificar danos nos arquivos”. Restaure-os de um backup ou adicione o pacote deles de novo e depois execute “Verificar danos nos arquivos” outra vez.
server-issues-folder = Pastas monitoradas que não podem ser lidas
server-issues-folder-blurb = Os pacotes delas ficam como estão até que a pasta possa ser lida de novo.
server-issues-missing = Pacotes monitorados cuja pasta sumiu
server-issues-missing-blurb = Cada pacote é mantido por 3 dias, caso a pasta esteja só offline, e depois é removido.
server-issues-job = Tarefas em segundo plano que falharam
server-issues-job-blurb = Tentar de novo executa a tarefa outra vez desde o início.
server-issues-file = Arquivos que não puderam ser abertos
server-issues-file-blurb = O Hoard não conseguiu abri-los nem guardá-los. Um pacote monitorado tenta de novo na próxima varredura; um arquivo que já está na biblioteca manteve o que tinha antes.
server-issues-unpack = Arquivos compactados que não puderam ser extraídos
server-issues-unpack-blurb = Cada um é mantido no pacote como um arquivo comum, então nada dentro dele está na biblioteca.
server-issues-read = Arquivos que o Hoard não conseguiu entender
server-issues-read-blurb = Estão na biblioteca, mas sem os dados, as tags ou a miniatura que a leitura daria.
server-issues-render = Modelos em que o renderizador de miniaturas falhou
server-issues-render-blurb = Eles não têm miniatura. Abrir um no visualizador ainda pode funcionar e pode salvar uma.
server-issues-convert = Modelos que o Blender não conseguiu converter
server-issues-convert-blurb = Eles não têm cópia em GLB, então não podem ser mostrados em 3D.
server-issues-unreal = Pacotes do Unreal que não puderam ser convertidos
server-issues-unreal-blurb = A página Unreal do pacote tem o restante dos pacotes dele.
server-issues-reformat = Arquivos que uma regra de conversão não conseguiu converter
server-issues-reformat-blurb = Eles ficam como estavam.
server-issues-materials = Pacotes cujos materiais não puderam ser identificados
server-issues-materials-blurb = As texturas estão todas lá; só não estão agrupadas em materiais.
server-issues-material-thumb = Materiais sem miniatura
server-issues-material-thumb-blurb = O material foi encontrado, mas a prévia dele não pôde ser desenhada.
server-issues-store-files = Arquivos de loja que não puderam ser lidos
server-issues-store-files-blurb = Arquivos da loja de onde um pacote veio, como um JSON do Megascans, que dão nome e tags a ele.
server-issues-scan-again = Varrer de novo
server-issues-retry = Tentar de novo
server-issues-reanalyze = Reanalisar pacote
server-issues-convert-again = Converter de novo
server-issues-look-again = Procurar de novo
server-issues-read-again = Ler de novo
server-issues-blob-missing = O arquivo guardado sumiu.
server-issues-blob-damaged = O arquivo guardado não corresponde ao que foi guardado: está danificado.
server-issues-folder-gone = A pasta “{ $folder }” não foi encontrada.
server-job-a-pack = um pacote
server-job-ingest = Importando { $pack }
server-job-retag = Refazendo as tags de { $pack }
server-job-llm-tag = Gerando tags para { $pack } com IA
server-job-convert = Convertendo { $pack } com o Blender
server-job-unreal = Convertendo pacotes do Unreal em { $pack }
server-job-reformat = Convertendo arquivos em { $pack } pelas regras de conversão
server-job-scan = Varrendo uma pasta monitorada
server-job-relocate = Movendo arquivos entre locais da biblioteca
server-job-looks = Identificando cores e aparências
server-job-sounds = Ouvindo sons
server-job-embed = Aprendendo o que as imagens mostram
server-job-materials = Procurando materiais em { $pack }
server-job-read-files = Lendo os arquivos de { $pack }
server-job-store-lookup = Consultando uma página de loja
server-maintenance-backup = Fazer backup do banco de dados
server-maintenance-backup-blurb = Uma cópia consistente do banco de dados (pacotes, tags, notas, coleções e Configurações) na pasta de backups ao lado dele. Os arquivos da biblioteca não estão nela: eles nunca mudam depois de guardados, então qualquer sincronização de arquivos faz backup deles.
server-maintenance-scrub = Verificar danos nos arquivos
server-maintenance-scrub-blurb = Lê de volta cada arquivo guardado e o compara com sua impressão digital, para pegar arquivos que se deterioraram ou sumiram de um disco ou NAS. Demora um pouco em uma biblioteca grande.
server-maintenance-reanalyze = Reanalisar todos os pacotes
server-maintenance-reanalyze-blurb = Lê de novo cada arquivo com esta versão do Hoard: miniaturas, prévias, detalhes e tags de regras são refeitos. Suas edições ficam; a IA não é consultada de novo.
server-maintenance-clear-cache = Limpar conversões em cache
server-maintenance-clear-cache-blurb = Descarta os arquivos guardados das conversões de “Baixar como” (eles são refeitos quando pedidos de novo) e o que trabalhos interrompidos deixaram para trás.
server-maintenance-shrink-thumbs = Reduzir miniaturas
server-maintenance-shrink-thumbs-blurb = Guarda as miniaturas e vistas de modelos mantidas em PNG do jeito que Configurações → Desempenho → Miniaturas pede, o que geralmente as deixa várias vezes menores. As que têm transparência, e pixel art, continuam em PNG.
server-maintenance-gc = Excluir arquivos não usados
server-maintenance-gc-blurb = Exclui arquivos guardados para os quais nada aponta mais, como os arquivos de pacotes excluídos definitivamente.
server-update-no-address = nenhum endereço a chamar foi definido
server-update-no-answer = nenhuma resposta em 30 segundos

## Configurações: Aparência

appearance-language = Idioma
appearance-language-auto = Do navegador ({ $name })

## Raiz

root-unreachable = Não foi possível acessar o servidor do Hoard: { $error }
root-unreachable-desktop = O Hoard não está respondendo: { $error }
root-desktop-reopen = Esta janela foi desconectada. Feche o Hoard e abra-o de novo para continuar.
root-try-again = Tentar de novo
root-lazy-connect = a conexão do app

## App: a biblioteca, a barra superior e os filtros

app-lazy-settings = as configurações
app-lazy-archive = os arquivados
app-lazy-shortcuts = os atalhos
app-lazy-setup = a configuração inicial
app-theme-system = Usar o tema do sistema
app-theme-light = Tema claro
app-theme-dark = Tema escuro
app-color-red = Vermelho
app-color-orange = Laranja
app-color-yellow = Amarelo
app-color-green = Verde
app-color-teal = Verde-azulado
app-color-blue = Azul
app-color-purple = Roxo
app-color-pink = Rosa
app-color-brown = Marrom
app-color-sand = Areia
app-color-gray = Cinza
app-color-slate = Ardósia
app-color-black = Preto
app-color-white = Branco
app-color-swatch = { $color }: assets com muito { $lower }
app-color-any = Qualquer cor
app-color-pick = Escolher uma cor
app-color = Cor
app-retry = Tentar de novo
app-undo = Desfazer
app-search-failed = A busca falhou: { $error }
app-unreachable = Não foi possível acessar o servidor: { $error }
app-unreachable-desktop = O Hoard não está respondendo: { $error }
app-forget-project-title = Esquecer { $name }?
app-forget-project-body = O Hoard deixa de mostrá-lo onde os { $count } assets dele são usados, até que o próximo hoard sync o informe de novo. Nada muda no projeto.
app-forget-project-confirm = Esquecer
app-forget-project-failed = Não foi possível esquecer { $name }: { $error }
app-forget-project-label = Esquecer o projeto { $name }
app-forget-project = Esquecer este projeto
app-save-name-like = parecido com { $name }
app-save-name-favorites = favoritos
app-search-saved = “{ $name }” foi salva: está em Filtros, em Seus
app-rank-alike = Mais parecidos
app-rank-color = Com mais da cor
app-rank-match = Melhor correspondência
app-sort = Ordenar
app-sort-newest = Mais recentes
app-sort-name = Nome
app-sort-largest = Maiores
app-sort-oldest = Mais antigos
app-collection-added =
    { $count ->
        [one] { $count } asset adicionado
       *[other] { $count } assets adicionados
    } a { $name }
app-collection-add-failed = Não foi possível adicionar à coleção: { $error }
app-collection-removed = Removidos de { $name }: { $count }
app-collection-remove-failed = Não foi possível remover da coleção: { $error }
app-ns-subject = Assunto
app-ns-style = Estilo
app-ns-theme = Tema
app-ns-material = Material
app-ns-map = Mapa PBR
app-ns-color = Cor
app-ns-mood = Clima
app-ns-genre = Gênero
app-ns-license = Licença
app-ns-res = Resolução
app-ns-duration = Duração
app-ns-poly = Contagem de polígonos
app-ns-format = Formato
app-ns-weight = Peso
app-ns-tech = Técnico
app-ns-blender = Versão do Blender
app-ns-depth = Profundidade de bits
app-ns-ucs = Categoria de som (UCS)
app-ns-bpm = Andamento
app-ns-root = Nota fundamental
app-ns-level = Volume
app-ns-ue = Classe do Unreal
app-ns-engine = Engine
app-ns-extracted = Original recuperado
app-ns-script = Sistema de escrita
app-bulk-failed =
    Não foi possível mudar { $count ->
        [one] esse asset
       *[other] esses { $count } assets
    }: { $error }
app-bulk-untagged =
    { $tag } foi tirada de { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-bulk-tagged =
    { $count ->
        [one] { $count } asset recebeu
       *[other] { $count } assets receberam
    } a tag { $tag }
app-bulk-category-set =
    A categoria de { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } agora é { $category }
app-bulk-category-cleared =
    A categoria de { $count ->
        [one] { $count } asset foi limpa
       *[other] { $count } assets foi limpa
    }
app-credits-failed = Não foi possível gerar os créditos: { $error }
app-archived-assets =
    { $count ->
        [one] { $count } asset arquivado
       *[other] { $count } assets arquivados
    }
app-archived-one = { $name } arquivado
app-open-failed = Não foi possível abrir esse asset: { $error }
app-unnamed-pack = pacote
app-unnamed-collection = coleção
app-unnamed-asset = asset
app-home-version = Hoard { $version } — voltar à biblioteca
app-home = Voltar à biblioteca
app-search-label = Buscar assets
app-search-placeholder = Busque nomes, caminhos, tags
app-views = Visualizações
app-library = Biblioteca
app-packs = Pacotes
app-packs-working = { $count } em andamento
app-update-title = O Hoard { $version } foi lançado (este servidor executa o { $current }). Baixe a nova imagem ou o novo binário e reinicie; o banco de dados se atualiza sozinho.
app-update = Atualizar
app-theme = Tema
app-sign-in = Entrar
app-archive-title = Arquivados: { $contents }
app-archive = Arquivados
app-settings = Configurações
app-palette-title = Paleta de comandos ({ $keys })
app-import-folder = Importar pasta
app-add-files = Adicionar arquivos
app-add-files-short = Adicionar
app-split-label = Como importar esta pasta
app-split-before = { "" }
app-split-after = { " " }contém { $count } pastas ou arquivos compactados.
app-split-each = Importar cada um como um pacote próprio ({ $count })
app-split-one = Importar como um só pacote
app-cancel = Cancelar
app-filters = Filtros
app-done = Concluído
app-yours = Seus
app-favorites = Favoritos
app-forget-search-label = Esquecer a busca salva { $name }
app-forget-search = Esquecer esta busca
app-projects-title = Projetos que fazem hoard sync daqui, com o que a última sincronização deles tinha
app-projects = Projetos
app-project-synced = Sincronizado { $when }
app-project-synced-by = Sincronizado { $when } por { $who }
app-collections = Coleções
app-no-filters = Nenhum filtro para esta busca.
app-kind = Tipo
app-facets-more = mais { $count }
app-facets-fewer = menos
app-searching = Buscando…
app-searching-live = Buscando
app-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-clear-all = Limpar tudo
app-save-name-label = Nome para esta busca
app-save-name-placeholder = Dê um nome
app-save = Salvar
app-save-search-title = Manter esta busca em Filtros, em Seus
app-save-search = Salvar busca
app-select-title = Escolha vários assets para dar tags, categorizar, colecionar, baixar ou arquivar juntos (X, ou Ctrl+clique)
app-select = Selecionar
app-tile-size = Tamanho dos blocos
app-supporting-title = Buffers glTF, bibliotecas de materiais, os mapas dos materiais, materiais que só um modelo usa e os arquivos de loja que descrevem modelos. Eles vêm junto quando você baixa o que os usa.
app-supporting-hidden =
    { $count ->
        [one] { $count } arquivo de apoio oculto
       *[other] { $count } arquivos de apoio ocultos
    }
app-supporting-show = Mostrar
app-supporting-shown = Incluindo arquivos de apoio
app-supporting-hide = Ocultar
app-welcome-title = Solte um pacote de assets em qualquer lugar desta página
app-welcome-before = Zips, tarballs, pastas ou arquivos soltos, tudo funciona. O Hoard os extrai, guarda cada arquivo uma vez pelo hash, lê tamanhos, durações e contagens de polígonos, e dá tags a tudo para você buscar coisas como{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }ou{ " " }
app-welcome-after = .
app-welcome-import-folder = Importar uma pasta
app-welcome-choose-files = Escolher arquivos
app-none-similar = Nada mais se parece muito com { $name } por enquanto.
app-none = Nada corresponde.
app-none-query = Nada corresponde a “{ $query }”.
app-none-filters = Nada corresponde com estes filtros.
app-none-query-filters = Nada corresponde a “{ $query }” com estes filtros.
app-clear-search-filters = Limpar busca e filtros
app-loading-more = Carregando mais
app-keys-hint = As setas movem, Enter abre, Espaço toca um som, X ou Ctrl+clique seleciona vários (toque longo em telas sensíveis ao toque).
app-resize-label = Redimensionar o painel de detalhes
app-resize-title = Arraste para redimensionar · clique duas vezes para a largura que cabe
app-selected-label = Assets selecionados
app-selected = { $count } selecionados
app-pick-assets = Escolher assets
app-download-zip = Baixar zip
app-credits-title = CREDITS.md dos pacotes de onde estes vêm: quem os fez, as licenças e as linhas de crédito que pedem
app-credits = Créditos
app-prefer-sources-title = SoundWaves e texturas do Unreal cujo arquivo original o Hoard recuperou
app-prefer-sources = WAV/PNG recuperado em vez de .uasset ({ $count })
bag-clear = Limpar a seleção
bag-glb-heading = Modelos como GLB
bag-glb-text = Cada modelo como arquivo .glb, em pé com o Y para cima e virado para a frente, com as texturas como arquivos separados numa pasta textures que os modelos compartilham. O resto vai como está.
bag-glb-download = Baixar zip de GLB
bag-plain-heading = Arquivos como estão
bag-plain-text = Cada arquivo como foi importado, com o que ele precisa, numa pasta para cada pacote.
bag-remove = Tirar { $name } da seleção
bag-more = e mais { $count }
app-bulk-tag-label = Uma tag para adicionar aos assets selecionados, ou tirar deles
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Adicionar esta tag a todos os assets selecionados
app-bulk-tag-add = Adicionar
app-bulk-tag-remove-title = Tirar esta tag de todos os assets selecionados
app-bulk-tag-remove = Remover
app-bulk-category-label = Categoria dos assets selecionados (vazio limpa)
app-bulk-category-placeholder = Categoria
app-bulk-category-set-title = Definir a categoria de todos os assets selecionados
app-bulk-category-clear-title = Limpar a categoria de todos os assets selecionados
app-bulk-category-set-button = Definir
app-bulk-category-clear-button = Limpar
app-remove-from-collection = Remover de { $name }
app-unfavorite = Desfavoritar
app-favorite = Favoritar
app-select-all = Selecionar todos os { $count }
app-archive-selected-title = Fora da busca até serem restaurados ou excluídos definitivamente, em Arquivados
app-drop-items = Solte { $count } itens para adicioná-los
app-drop = Solte para adicionar ao Hoard
app-drop-hint = Cada arquivo ou pasta de nível superior vira um pacote próprio. Gigabytes não são problema; arquivos grandes retomam se a conexão cair.

## Paleta de comandos

palette-group-go-to = Ir para
palette-group-actions = Ações
palette-group-theme = Tema
palette-group-kinds = Tipos
palette-group-packs = Pacotes
palette-group-collections = Coleções
palette-group-projects = Projetos
palette-group-saved = Buscas salvas
palette-group-assets = Assets
palette-hint-current = atual
palette-library = Biblioteca
palette-library-keywords = assets grade início home grid
palette-favorites = Favoritos
palette-favorites-keywords = estrela estrelas curtidos marcados starred
palette-packs = Pacotes
palette-archive = Arquivados
palette-archive-keywords = lixeira lixo excluídos restaurar arquivo trash deleted restore
palette-stats = Estatísticas da biblioteca
palette-stats-keywords = estatísticas números tamanho disco espaço contagem statistics
palette-search = Buscar
palette-add-files = Adicionar arquivos…
palette-add-files-keywords = enviar upload importar
palette-import-folder = Importar uma pasta…
palette-import-folder-keywords = enviar upload pacote
palette-select-all = Selecionar todos os { $count } mostrados
palette-select-all-keywords = marcar
palette-select-several = Selecionar vários…
palette-select-several-keywords = lote vários múltiplos tag arquivar
palette-archive-selected = Arquivar os { $count } selecionados
palette-archive-selected-keywords = excluir apagar remover lixeira
palette-clear-selection = Limpar a seleção
palette-clear-filters = Limpar filtros
palette-clear-filters-keywords = redefinir resetar
palette-hide-supporting = Ocultar arquivos de apoio
palette-show-supporting = Mostrar arquivos de apoio
palette-supporting-keywords = mapas normal rugosidade máscaras normal maps roughness masks
palette-account = Conta e tokens de acesso
palette-account-keywords = senha login usuário token api
palette-sign-out = Sair ({ $name })
palette-sign-out-keywords = sair deslogar logout
palette-sign-in = Entrar
palette-sign-in-keywords = entrar login conta senha
palette-settings = Configurações
palette-settings-keywords = blender ffmpeg llm ia tags unreal admin administrador ferramentas preferências
palette-shortcuts = Atalhos de teclado
palette-shortcuts-keywords = ajuda teclas
palette-theme-keywords = tema aparência modo
palette-theme-variant-keywords = aparência cor
palette-light-theme = Tema claro: { $name }
palette-dark-theme = Tema escuro: { $name }
palette-kind-all = Todos os tipos (deixar de mostrar só { $kind })
palette-kind-only = Só { $kind }
palette-title = Paleta de comandos
palette-placeholder = Busque assets, vá a um pacote, execute um comando…
palette-results = Resultados
palette-searching = Buscando…
palette-no-match = Nada corresponde a “{ $query }”
palette-foot-move = mover
palette-foot-run = executar
palette-foot-shortcuts = todos os atalhos

## Atalhos de teclado

shortcuts-title = Atalhos de teclado
shortcuts-close = Fechar
shortcuts-anywhere = Em qualquer lugar
shortcuts-palette = Paleta de comandos: buscar, pular, executar
shortcuts-search = Buscar
shortcuts-this-list = Esta lista
shortcuts-escape = Fechar, limpar a seleção ou fechar os detalhes
shortcuts-library = Biblioteca
shortcuts-move = Mover entre os blocos (ou J e K)
shortcuts-open = Abrir os detalhes
shortcuts-play-sound = Tocar ou parar um som
shortcuts-select = Selecionar ou desmarcar
shortcuts-favorite = Adicionar aos favoritos ou tirar deles
shortcuts-key-click = clique
shortcuts-select-range = Selecionar um intervalo
shortcuts-select-all = Selecionar tudo o que é mostrado
shortcuts-sound-player = Player de som
shortcuts-play-pause = Tocar ou pausar
shortcuts-skip = Voltar ou avançar
shortcuts-start-end = Início ou fim
shortcuts-image-inspector = Inspetor de imagens
shortcuts-zoom = Aumentar ou diminuir o zoom
shortcuts-fit = Ajustar à janela
shortcuts-actual-pixels = Pixels reais
shortcuts-tile = Repetir 3×3 para conferir as emendas
shortcuts-background = Próximo fundo
shortcuts-channel = Mostrar um canal

## KindChips.vue

kinds-group = Tipos de asset

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = adicionada por uma pessoa
tag-source-rule = do gerador de tags offline
tag-source-meta = medida a partir do arquivo
tag-source-ai = sugerida por IA
tag-remove = Remover a tag { $tag }

## ActivityIndicator.vue

activity-label-idle = Atividade em segundo plano: ociosa
activity-label-running = Atividade em segundo plano: { $running } em execução
activity-label-queued = Atividade em segundo plano: { $running } em execução, { $queued } na fila
activity-title = Atividade em segundo plano
activity-unreachable = Não foi possível acessar o servidor
activity-unreachable-desktop = O Hoard não está respondendo
activity-empty = Nada em execução. Importações, geração de tags, conversões e downloads de zip aparecem aqui enquanto trabalham.
activity-queued = na fila
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } tris
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Tocar { $name }
tile-pause = Pausar { $name }
tile-favorite = Favoritar { $name }
tile-in-favorites = Nos seus favoritos
tile-add-favorite = Adicionar aos seus favoritos (F)
tile-more-like = Mais como { $name }
tile-more-like-this = Mais como este
tile-select = Selecionar { $name }

## CollectionPicker.vue

collections-add = Adicionar à coleção
collections-menu = Coleções
collections-find-label = Encontrar ou criar uma coleção
collections-find-placeholder = Encontre ou dê nome a uma nova coleção
collections-create = Criar
collections-empty = Nenhuma coleção ainda. Digite um nome para começar uma.

## FolderPicker.vue

folders-list = Pastas
folders-close = Fechar
folders-browse = Procurar…
folders-up-title = Subir uma pasta
folders-up = Subir
folders-this-computer = Este computador
folders-none = Nenhuma pasta aqui.

## LocationForm.vue

locations-mode-question = O que o Hoard deve fazer com esta pasta?
locations-mode-managed = Manter minha biblioteca aqui
locations-mode-managed-hint = Os pacotes que você envia ou importa são copiados para esta pasta. O Hoard cuida dela, então não mexa no conteúdo.
locations-mode-watched = Indexar uma pasta que já tenho
locations-mode-watched-hint = Cada pasta dentro dela vira um pacote. O Hoard lê os arquivos onde estão e nunca os altera, move ou exclui, e detecta pastas novas quando elas aparecem.
locations-folder = Pasta no servidor
locations-folder-desktop = Pasta
locations-folder-placeholder = A pasta onde estão os seus pacotes
locations-folder-hint-managed = O Hoard a cria se ela não existir. No Docker, é uma pasta que você montou no contêiner.
locations-folder-hint-watched = No Docker, monte primeiro a pasta no contêiner (somente leitura não tem problema) e depois escolha-a aqui.
locations-name = Nome
locations-name-placeholder-managed = Biblioteca
locations-name-placeholder-watched = Meus pacotes
locations-files = Arquivos que vão para cá
locations-catch-all = Também tudo o que nenhum outro local recebe
locations-kinds-hint = Os arquivos de um pacote novo vão para o local que marca o tipo deles. O resto vai para o local geral.
locations-files-all = Todos, por enquanto. Adicione mais locais depois nas Configurações para mandar alguns tipos (música, por exemplo) para outro disco.
locations-first-watched-note = Enviar arquivos exige uma biblioteca mantida pelo Hoard. Você pode adicionar uma depois em Configurações → Locais da biblioteca.
locations-checking = Verificando a pasta…
locations-add = Adicionar local
locations-cancel = Cancelar

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Não foi possível carregar os locais da biblioteca
locations-takes-and-rest = { $kinds } e todo o resto
locations-takes-rest = Tudo o que os outros não recebem
locations-takes-everything = Tudo
locations-takes-nothing = Nada ainda
locations-save-failed = Não foi possível salvar o local
locations-unwatch-title = Parar de monitorar { $name }?
locations-unwatch-body =
    { $count ->
        [one] O { $count } pacote dela sai
       *[other] Os { $count } pacotes dela saem
    } do Hoard, com as tags e notas. Os arquivos na pasta não são tocados.
locations-unwatch-body-empty = Os arquivos na pasta não são tocados.
locations-unwatch = Parar de monitorar
locations-remove-title = Remover { $name }?
locations-remove-body = Os arquivos que estiverem nele são movidos primeiro para os outros locais, por tipo; ele sai quando ficar vazio. Nada se perde.
locations-remove-body-only = Se ele contiver arquivos, adicione primeiro outro local para onde eles possam ser movidos.
locations-remove = Remover
locations-removed = { $name } removido
locations-emptying = Movendo os arquivos para fora de { $name }; ele será removido quando isso terminar
locations-offline-title = { $name } não pode ser acessado
locations-offline-body = Os arquivos dele não podem ser movidos enquanto estiver offline. Remover mesmo assim? Os arquivos que ainda estiverem nele vão aparecer como ausentes.
locations-remove-anyway = Remover mesmo assim
locations-remove-failed = Não foi possível remover { $name }
locations-rebalancing = Movendo os arquivos guardados para os locais que recebem o tipo deles
locations-rebalance-failed = Não foi possível mover os arquivos
locations-scanned-never = ainda não varrido
locations-scanned-now = varrido agora mesmo
locations-scanned-minutes = varrido há { $mins } min
locations-scanned-hours = varrido há { $hours } h
locations-scanned-date = varrido em { $date }
locations-scan-failed = Não foi possível varrer { $name }
locations-watching = Monitorando { $name }
locations-added = { $name } adicionado
locations-loading = Carregando…
locations-managed-title = Bibliotecas mantidas pelo Hoard
locations-watched-title = Pastas monitoradas
locations-managed-hint = Envios e importações são copiados para cá. Cada local recebe os tipos que marca; o local geral recebe o resto.
locations-watched-hint = Pastas de pacotes indexadas onde estão. O Hoard nunca altera os arquivos nelas e procura pastas novas e removidas a cada 15 minutos.
locations-managed-empty = Nenhuma ainda, então não há onde colocar um envio. Adicione uma abaixo.
locations-reachable = Acessível
locations-unreachable-now = Inacessível no momento
locations-catch-all-tag = geral
locations-packs =
    { $count ->
        [one] { $count } pacote
       *[other] { $count } pacotes
    }
locations-unreachable = inacessível
locations-scanning = Varrendo…
locations-scan = Varrer agora
locations-edit = Editar
locations-catch-all-locked = Para mudar isso, faça de outro local o local geral
locations-edit-hint-before = Vale para os arquivos adicionados daqui em diante. Para mover o que já está guardado, use
locations-edit-hint-after = abaixo.
locations-save = Salvar
locations-rebalance = Mover os arquivos guardados para combinar
locations-rebalance-hint = Depois de mudar o que um local recebe: os arquivos já guardados vão para o local que agora recebe o tipo deles.
locations-add-title = Adicionar um local
locations-add-hint = Outro disco para alguns tipos de arquivo, ou uma pasta de pacotes que você já tem.
locations-add-more = Adicionar um local…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Não foi possível carregar os Problemas
issues-started =
    { $count ->
        [one] Iniciado de novo.
       *[other] { $count } itens iniciados de novo.
    }
issues-retry-failed = Não foi possível tentar de novo
issues-dismiss-failed = Não foi possível dispensar
issues-dismiss-all-title = Dispensar todos os { $count }?
issues-dismiss-all-body = “{ $title }” será limpo. O que der errado de novo volta a ser listado.
issues-dismiss-all = Dispensar todos
issues-just-now = agora mesmo
issues-minutes-ago = há { $mins } min
issues-hours-ago = há { $hours } h
issues-loading = Carregando…
issues-none = Nada deu errado.
issues-action-all = { $action }: todos
issues-open-asset = Abrir
issues-open-pack = Mostrar o pacote
issues-open-locations = Locais da biblioteca
issues-where-before = em
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } vez
       *[other] { $count } vezes
    }
issues-starting = Iniciando…
issues-dismiss-named = Dispensar { $title }
issues-dismiss = Dispensar
issues-show-fewer = Mostrar menos
issues-show-all = Mostrar todos os { $count }
issues-not-listed = Mais { $count } não estão listados.

## archive.ts

archive-anyway = Arquivar mesmo assim
archive-failed = Não foi possível arquivar: { $error }
archive-undo = Desfazer
archive-restore-failed = Não foi possível restaurar: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pacote
       *[other] { $packs } pacotes
    } e { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pacote
       *[other] { $packs } pacotes
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-nothing = nada

## ArchiveView.vue

archive-load-failed = Não foi possível carregar os arquivados: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Restaurado: { $what }
archive-purged = Excluído definitivamente: { $what }
archive-purge-failed = Não foi possível excluir: { $error }
archive-purge-title = Excluir { $name } definitivamente?
archive-purge-pack-body =
    { $count ->
        [one] O { $count } asset dele
       *[other] Os { $count } assets dele
    } e { $size } de arquivos também vão embora, exceto arquivos que outro pacote compartilha. Isso não pode ser desfeito.
archive-purge = Excluir para sempre
archive-purge-asset-body = O arquivo dele é liberado, a menos que outro asset o compartilhe. Isso não pode ser desfeito.
archive-empty-title = Esvaziar os arquivados?
archive-empty-body = { $what } será excluído definitivamente, e os arquivos, liberados. Isso não pode ser desfeito.
archive-empty = Esvaziar arquivados
archive-title = Arquivados
archive-intro = Pacotes e assets arquivados ficam fora da busca, das contagens e da lista de tags. Restaure-os, ou exclua-os definitivamente para liberar espaço.
archive-loading = Carregando…
archive-is-empty = Não há nada arquivado
archive-is-empty-hint = Arquive um pacote ou um asset e ele espera aqui até você restaurá-lo ou excluí-lo definitivamente.
archive-packs = Pacotes
archive-assets = Assets
archive-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
archive-archived = arquivado { $when }
archive-from = de { $pack }
archive-restore = Restaurar
archive-show-more = Mostrar mais (faltam { $count })

## mine.ts

projects-favorite-add-failed = Não foi possível adicionar aos seus favoritos: { $error }
projects-favorite-remove-failed = Não foi possível tirar dos seus favoritos: { $error }
projects-save-search-failed = Não foi possível salvar a busca: { $error }
projects-forget-search-failed = Não foi possível remover { $name }: { $error }

## projects.ts

projects-list-and = { $items } e { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = mais { $count }
projects-other =
    { $count ->
        [one] { $count } outro projeto
       *[other] { $count } outros projetos
    }
projects-usage-title-one = { $name } usa isto
projects-usage-title = { $count } projetos usam isto
projects-usage-body-one = Usado em { $list }: o próximo hoard sync o tira de lá.
projects-usage-body = Usado em { $list }: o próximo hoard sync de cada um o tira de lá.

## uploader.ts

upload-network-error = erro de rede
upload-gave-up = { $what }: { $error } depois de { $attempts } tentativas
upload-failed-plain = falhou
upload-already-imported = já importado
upload-part-failed = o envio falhou ({ $status })
upload-batch = lote de { $count } arquivos
upload-batch-failed = o lote falhou ({ $status })
upload-finish = concluir o envio

## UploadQueue.vue

upload-eta-seconds = faltam { $s } s
upload-eta-minutes = faltam { $min } min
upload-eta-hours = faltam { $h } h
upload-queue = Importações
upload-importing = Importando { $count } pacotes
upload-bytes-of = { $sent } de { $total }
upload-rate = { $size }/s
upload-waiting = Aguardando
upload-unpacking = Enviado, extraindo agora
upload-already = Já está na biblioteca
upload-stopped = Parado
upload-files-of = { $done } de { $total } arquivos
upload-resumed = retomado, { $size } já estavam lá
upload-skipped = Estes mesmos arquivos já foram importados como “{ $name }”, então foram ignorados.
upload-error = { $error }. Seu progresso fica guardado: tente de novo, ou solte os mesmos arquivos mais tarde.
upload-retry = Tentar de novo
upload-import-again = Importar de novo mesmo assim
upload-dismiss = Dispensar
upload-cancel = Cancelar

## PacksView.vue

packs-rename-failed = Não foi possível renomear: { $error }
packs-collection-deleted = A coleção { $name } foi excluída
packs-delete-failed = Não foi possível excluir: { $error }
packs-status-uploading = Enviando (arquivos recebidos até agora)
packs-status-ingesting = Extraindo e gerando tags
packs-status-analyzing = O LLM está gerando tags
packs-status-ready = Pronto
packs-status-failed = Falhou
packs-retag-failed = Não foi possível começar a refazer as tags: { $error }
packs-read-files-failed = Não foi possível ler os arquivos do pacote: { $error }
packs-context-load-failed = Não foi possível carregar os dados de loja do pacote: { $error }
packs-undo-store-failed = Não foi possível tirar a página da loja: { $error }
packs-context-save-failed = Não foi possível salvar os dados de loja: { $error }
packs-unreal-load-failed = Não foi possível carregar o status da conversão do Unreal: { $error }
packs-unreal-discard-title = Descartar os originais?
packs-unreal-discard-body = Os arquivos .uasset originais em { $name } são apagados depois de convertidos. Os dados exclusivos do Unreal neles, como grafos de materiais, se perdem para sempre.
packs-unreal-discard-confirm = Converter e descartar
packs-unreal-convert-failed = Não foi possível iniciar a conversão: { $error }
packs-unreal-fact-converted = { $done } convertidos do Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } pacote do Unreal
       *[other] { $left } pacotes do Unreal
    }
packs-unreal-fact-both = { $done } convertidos do Unreal, { $left } ainda em pacotes
packs-thumbs-canceled = Cancelado depois de { $done } de { $total }
packs-thumbs-drew =
    { $count ->
        [one] { $count } modelo desenhado
       *[other] { $count } modelos desenhados
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } miniatura substituída.
       *[other] { $lead }: { $improved } miniaturas substituídas.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } miniatura substituída, { $failed } não puderam ser desenhadas.
       *[other] { $lead }: { $improved } miniaturas substituídas, { $failed } não puderam ser desenhadas.
    }
packs-archived = { $name } e os { $count } assets dele foram arquivados
packs-blobs-bad = { $count } arquivos falharam na última verificação de integridade.
packs-collections = Coleções
packs-browse-named = Explorar { $name }
packs-collection-name = Nome da coleção
packs-save = Salvar
packs-cancel = Cancelar
packs-assets =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
packs-updated = Atualizado em { $date }
packs-browse = Explorar
packs-collection-download = Baixar (.zip)
packs-collection-credits = Créditos
packs-collection-credits-title = Quem fez os pacotes de onde vêm estes assets, as licenças e as linhas de crédito que pedem
packs-rename = Renomear
packs-delete = Excluir
packs-packs = Pacotes
packs-empty = Nenhum pacote ainda. Solte um zip ou uma pasta em qualquer lugar desta página para adicionar um.
packs-find = Encontrar um pacote
packs-find-placeholder = Encontre um pacote por nome, loja, licença ou tag
packs-sort = Ordem
packs-sort-new = Mais recentes primeiro
packs-sort-old = Mais antigos primeiro
packs-sort-name = Por nome
packs-sort-size = Maiores primeiro
packs-sort-assets = Mais assets primeiro
packs-with-kind = Pacotes com
packs-state = Status
packs-state-any = Qualquer status
packs-state-working = Em andamento
packs-state-failed = Com falha
packs-state-untagged = Ainda sem descrição
packs-store = Loja
packs-store-any = Qualquer loja
packs-store-from = De { $store }
packs-store-none = Sem página de loja
packs-shown-of = { $shown } de { $total }
packs-clear = Limpar
packs-no-match = Nenhum pacote corresponde.
packs-clear-all = Limpar a busca e os filtros
packs-pack-name = Nome do pacote
packs-thumbs-drawing = Desenhando miniaturas neste navegador…
packs-thumbs-drawing-count = Desenhando miniaturas neste navegador: { $done } de { $total }…
packs-thumbs-stopped = Parado: { $error }
packs-thumbs-none = Todos os modelos aqui já têm uma miniatura desenhada com todas as texturas.
packs-dismiss = Dispensar
packs-license = Licença: { $license }
packs-license-tier = Licença: { $license } ({ $tier })
packs-license-not-found = não encontrada
packs-from-before = De
packs-from-after = { "" }
packs-added = Adicionado em { $date }
packs-download = Baixar pacote (.zip)
packs-store-details = Dados da loja
packs-more-named = Mais ações para { $name }
packs-more = Mais
packs-rename-more = Renomear…
packs-retag-rules = Executar de novo o gerador de tags offline
packs-retag-llm = Executar de novo o gerador de tags por LLM
packs-make-thumbs = Fazer miniaturas neste navegador
packs-make-thumbs-title = Abrir cada modelo em um visualizador 3D fora da vista, um por vez, e usar a imagem dele como miniatura quando ela for melhor que a do servidor
packs-make-thumbs-title-desktop = Abrir cada modelo em um visualizador 3D fora da vista, um por vez, e usar a imagem dele como miniatura quando ela for melhor que a que o Hoard fez
packs-read-files = Ler de novo os arquivos do pacote
packs-credits = Baixar créditos
packs-credits-title = CREDITS.md: quem fez o pacote, a licença dele e a linha de crédito que pede
packs-unreal-open = Converter conteúdo do Unreal…
packs-archive = Arquivar pacote
packs-unreal-intro = O Hoard lê os pacotes do Unreal sozinho e guarda o que há dentro como arquivos comuns: texturas como PNG ou EXR, sons como WAV, materiais como JSON. O arquivo convertido toma o lugar do pacote, mantendo as tags, os links e as coleções.
packs-unreal-converted =
    { $count ->
        [one] convertido
       *[other] convertidos
    }
packs-unreal-converted-kept =
    { $count ->
        [one] convertido, { $kept } com o original mantido
       *[other] convertidos, { $kept } com o original mantido
    }
packs-unreal-pending =
    { $count ->
        [one] ainda não examinado
       *[other] ainda não examinados
    }
packs-unreal-needs-editor =
    { $count ->
        [one] aguardando o Unreal Editor
       *[other] aguardando o Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] sem formato comum
       *[other] sem formato comum
    }
packs-unreal-failed =
    { $count ->
        [one] com falha
       *[other] com falha
    }
packs-unreal-reasons = Por que restam pacotes
packs-unreal-editor-before = Malhas e animações precisam do Unreal Editor. Em uma máquina com o Unreal instalado, execute
packs-unreal-editor-command = hoard unreal-worker --editor <caminho do UnrealEditor-Cmd>
packs-unreal-editor-after = ; ele os exporta como FBX e envia os resultados para cá.
packs-unreal-discard = Descartar os arquivos .uasset originais depois de convertidos
packs-unreal-discard-hint = Libera o espaço. Os dados exclusivos do Unreal, como grafos de materiais, se perdem.
packs-unreal-drop = Excluir pacotes sem formato comum
packs-unreal-drop-hint = Blueprints, níveis e outros assets exclusivos da engine.
packs-unreal-convert = Converter
packs-context-hint-before = De onde o pacote veio e o que o vendedor diz sobre ele. O gerador de tags por LLM lê isto junto com cada arquivo, então uma descrição de loja como "120 ice and fire spells" o ajuda a dar nome a
packs-context-hint-after = .
packs-store-auto = Encontrada automaticamente: o próprio pacote cita esta página.
packs-store-not-this = Não é esta
packs-lookup-label = Buscar nas lojas por
packs-lookup-placeholder = O nome do pacote na loja
packs-lookup-searching = Buscando…
packs-lookup-search = Buscar nas lojas
packs-lookup-hide = Ocultar
packs-lookup-clues = Dos arquivos do pacote: { $clues }.
packs-lookup-also = Também buscamos { $queries }.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Nada no Fab, itch.io, Kenney ou OpenGameArt se parece com ele. Tente menos palavras ou palavras diferentes.
packs-lookup-use = Usar
packs-lookup-searched = Buscado em { $date }.
packs-lookup-again = Buscar de novo
packs-lookup-again-after = .
packs-lookup-unreachable = Não foi possível acessar { $stores } desta vez.
packs-lookup-also-look = Procure também em
packs-lookup-links-and = { " " }e{ " " }
packs-lookup-browser-only = (só dá para buscar nelas pelo seu navegador); cole o endereço da página abaixo.
packs-lookup-open = Encontrar nas lojas…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Vendedor
packs-seller-placeholder = Quem fez
packs-license-label = Licença
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Nível da licença
packs-license-tier-placeholder = Personal, Professional…
packs-store-page = Página da loja
packs-store-text = Descrição da loja
packs-store-text-placeholder = Cole o texto do anúncio, ou deixe vazio para buscá-lo na página da loja.
packs-store-text-fetch = Ao salvar, a descrição é buscada na página da loja.
packs-readme = README encontrado no pacote
packs-save-retag = Salvar e executar de novo o gerador de tags por LLM

## AssetDetail.vue

asset-lazy-3d-viewer = o visualizador 3D
asset-lazy-text = o texto
asset-lazy-text-viewer = o visualizador de texto
asset-lazy-image-viewer = o visualizador de imagens
asset-lazy-material-preview = a prévia do material
asset-lazy-material-viewer = o visualizador de materiais
asset-lazy-sprite-player = o player de sprites
asset-lazy-sheet-view = o visualizador de folhas de sprites
asset-rel-texture = textura
asset-rel-buffer = dados de geometria
asset-rel-material = arquivo de material
asset-rel-animation = animação
asset-rel-library = arquivo vinculado
asset-rel-guess = provavelmente acompanha
asset-rel-sidecar = dados da loja
asset-rel-variant = variante
asset-role-map = mapa de { $role }
asset-maps-count =
    { $count ->
        [one] { $count } mapa
       *[other] { $count } mapas
    }
asset-needs = Precisa de
asset-animations = Animações
asset-sprites = Sprites
asset-used-by = Usado por
asset-conversion-failed = a conversão falhou
asset-as-converting = Convertendo…
asset-as-converting-percent = Convertendo… { $percent }%
asset-as-queued = Na fila…
asset-download = Baixar
asset-as-label = Baixar como
asset-as-original = { $format } (formato original)
asset-as-size = Tamanho
asset-as-full-size = Tamanho original
asset-as-up-to = Até { $size } px
asset-as-hint = Escolha um tamanho ou outro formato
asset-meta-size = Tamanho
asset-meta-length = Duração
asset-meta-sample-rate = Taxa de amostragem
asset-meta-channels = Canais
asset-meta-mono = Mono
asset-meta-stereo = Estéreo
asset-meta-bit-depth = Profundidade de bits
asset-meta-bits = { $bits } bits
asset-meta-level = Nível
asset-meta-level-value = pico { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = pico { $peak } dBFS, RMS { $rms } dBFS, com clipping
asset-meta-loop = Loop
asset-meta-loop-value = { $start } a { $end }
asset-meta-playback = Reprodução
asset-meta-one-shot = One-shot
asset-meta-tempo = Andamento
asset-meta-root-note = Nota fundamental
asset-meta-cue-markers = Marcadores de cue
asset-meta-library = Biblioteca
asset-meta-keywords = Palavras-chave
asset-meta-title = Título
asset-meta-artist = Artista
asset-meta-album = Álbum
asset-meta-genre = Gênero
asset-meta-generator = Gerador
asset-meta-everything-with = Mostrar tudo com a tag { $tag }
asset-meta-saved-with = Salvo com
asset-meta-color-mode = Modo de cor
asset-meta-layers = Camadas
asset-meta-range = Faixa
asset-meta-hdr = HDR (a prévia tem tone mapping)
asset-meta-format = Formato
asset-meta-large-document = Documento grande (PSB)
asset-meta-ai = IA
asset-meta-ai-made = Feito com IA generativa
asset-meta-ai-edited = Editado com IA generativa
asset-meta-ai-how-metadata = metadados
asset-meta-ai-how-generator = configurações do gerador
asset-meta-ai-how-iptc = tipo de fonte digital IPTC
asset-meta-ai-says = { $made }, segundo: { $how }
asset-meta-ai-by-says = { $made } ({ $by }), segundo: { $how }
asset-meta-provenance = Procedência
asset-meta-c2pa = Tem Content Credentials C2PA
asset-meta-compression = Compressão
asset-meta-note = Nota
asset-meta-maps = Mapas
asset-meta-sizes =
    { $count ->
        [one] Tamanho
       *[other] Tamanhos
    }
asset-meta-triangles = Triângulos
asset-meta-vertices = Vértices
asset-meta-meshes = Malhas
asset-meta-materials = Materiais
asset-meta-animations = Animações
asset-meta-skins = Skins
asset-meta-unreal-asset = Asset do Unreal
asset-meta-unreal-saved-with = { $class }, salvo com o UE { $engine }
asset-meta-package = Pacote
asset-meta-cooked = Cooked (build do jogo): prévias e fontes foram removidas
asset-meta-tiling = Repetição
asset-meta-seamless = Sem emendas nos dois eixos
asset-meta-analysis-error = Não foi possível ler este arquivo por completo: { $error }
asset-copy-blocked = Não foi possível copiar: o navegador bloqueou
asset-save-error = Não foi possível salvar: { $error }
asset-tag-removed = { $tag } removida
asset-undo = Desfazer
asset-play-error = Não foi possível tocar este arquivo: { $error }
asset-copied-hash = Hash copiado
asset-copied-toml = toml copiado
asset-copied-id = ID copiado
asset-copied-link = Link copiado
asset-origin-before = Este arquivo era
asset-origin-class-engine = , um { $class } salvo pelo UE { $engine }.
asset-origin-class = , um { $class }.
asset-origin-engine = { " " }salvo pelo UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = O Unreal Editor o exportou em { $date }.
asset-origin-editor-notes = O Unreal Editor o exportou em { $date } ({ $notes }).
asset-origin-hoard = O Hoard o extraiu do pacote em { $date }.
asset-origin-hoard-notes = O Hoard o extraiu do pacote em { $date } ({ $notes }).
asset-origin-download = Baixar o .{ $ext } original
asset-origin-discarded = O pacote original foi descartado depois da conversão.
asset-details = Detalhes do asset
asset-in-pack = em
asset-previous = Asset anterior
asset-next = Próximo asset
asset-favorite = Favoritar
asset-favorite-on-title = Nos seus favoritos (F tira)
asset-favorite-off-title = Adicionar aos seus favoritos (F)
asset-share = Compartilhar
asset-share-title = Compartilhar um link para este asset (quem abrir entra no Hoard primeiro)
asset-layout-panel = Mostrar em um painel lateral
asset-layout-page = Mostrar em página inteira
asset-layout-page-title = Mostrar em página inteira: uma prévia grande ao lado dos detalhes
asset-close = Fechar detalhes
asset-rename-title = Clique para renomear
asset-name = Nome
asset-views-alt = O modelo visto de quatro direções
asset-inspect-aria = Inspecionar { $name } em tela cheia
asset-inspect-title = Inspecionar: zoom, repetição, canais
asset-playback-position = Posição da reprodução
asset-playback-value = { $current } de { $duration }
asset-pause = Pausar
asset-play = Tocar
asset-waveform-drawing = Desenhando a forma de onda…
asset-waveform-skipped = Sem forma de onda para arquivos tão grandes
asset-play-animation = Tocar como animação
asset-show-views = Mostrar a folha de quatro vistas que o gerador de tags viu
asset-palette = Cores na imagem
asset-swatch-title = { $hex }, { $percent }% da imagem. Encontrar assets com esta cor
asset-swatch-aria = Encontrar assets com { $hex }, { $percent }% desta imagem
asset-more-like-this = Mais como este
asset-desc-label = Sua descrição
asset-desc-placeholder = O que é, onde você usaria…
asset-save = Salvar
asset-cancel = Cancelar
asset-desc-family = Salva em todas as { $count } variantes
asset-desc-edit = Editar descrição
asset-desc-write = Escrever uma descrição
asset-desc-ai = Escrita por IA
asset-desc-file = Do arquivo
asset-desc-store-before = Da
asset-desc-store-page = página do { $store }
asset-desc-store-page-unnamed = página da loja
asset-desc-store-after = (o pacote inteiro)
asset-less = Menos
asset-more = Mais
asset-tags = Tags
asset-tag-add-label = Adicionar uma tag
asset-tag-add-placeholder = Adicione uma tag, por exemplo mood:tense
asset-tag-add = Adicionar
asset-apply-family = Aplicar as mudanças de tag, categoria e descrição a todas as { $count } variantes
asset-collections = Coleções
asset-collection-remove = Remover de { $name }
asset-collection-add-another = Adicionar a outra
asset-collection-add = Adicionar à coleção
asset-used-in = Usado em
asset-used-in-title = Mostrar o que { $name } tem
asset-used-in-synced = sincronizado { $when }
asset-maps = Mapas
asset-show-more = Mostrar mais { $count }
asset-materials-heading =
    { $count ->
        [one] Material
       *[other] Materiais
    }
asset-material-maps-aria = { $name }: mapas
asset-variants = Variantes ({ $count } versões)
asset-copies = Cópias idênticas
asset-copies-note = Os mesmos bytes são guardados uma vez só; estes pacotes também trazem este arquivo.
asset-unreal-converted = Convertido do Unreal
asset-rule-converted = Convertido por uma regra
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Uma regra de conversão criou este { $format } em { $date } a partir de
asset-rule-kept-until = O original fica guardado até { $date }.
asset-rule-put-back = Restaurar o original
asset-rule-put-back-title = Restaurar { $file }?
asset-rule-put-back-body = O arquivo convertido é excluído e o original volta para o lugar dele, com as tags, as coleções e os links. A partir daí, as regras de conversão não mexem mais nele.
asset-rule-put-back-done = { $file } foi restaurado
asset-rule-let-go = O original foi excluído quando terminou o prazo de guarda dos originais definido nas Configurações.
asset-unreal-package = Do pacote do Unreal
asset-ue-needs-editor-before = Aguardando o Unreal Editor: { $reason }. Em uma máquina com o Unreal instalado, execute
asset-ue-needs-editor-after = e ele volta como FBX.
asset-ue-editor-path = caminho do UnrealEditor-Cmd
asset-ue-unsupported = Mantido como pacote: { $reason }.
asset-ue-failed = A conversão falhou: { $reason }
asset-recovered = O .{ $ext } original estava guardado dentro deste pacote, e o Hoard o extraiu.
asset-recovered-download = Baixar .{ $ext }
asset-ue-meshes-before = Malhas e animações são guardadas no formato próprio do Unreal. Converta o pacote (página do pacote, Mais) e depois execute
asset-ue-meshes-after = junto a uma instalação do Unreal Editor para obter FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } tag do asset registry
       *[other] { $count } tags do asset registry
    }
asset-file = Arquivo
asset-bytes = Bytes
asset-source = Origem
asset-path = Caminho
asset-download-maps =
    { $count ->
        [one] Baixar { $count } mapa
       *[other] Baixar todos os { $count } mapas
    }
asset-download-linked =
    { $count ->
        [one] Baixar com { $count } arquivo vinculado
       *[other] Baixar com { $count } arquivos vinculados
    }
asset-download-only-title = Só este arquivo, sem os arquivos que ele usa
asset-download-only = Só este arquivo
asset-include-animations = Incluir animações no .zip
asset-copy-toml = Copiar entrada do hoard.toml
asset-copy-id = Copiar ID
asset-teach = Ensinar este arquivo ao Hoard
asset-teach-title = Descreva este arquivo de loja para o Hoard conseguir lê-lo (Configurações → Arquivos de loja)
asset-archive = Arquivar
asset-archive-title = Fora da busca até ser restaurado ou excluído definitivamente, em Arquivados

## ImageViewer.vue

image-channel-red = Canal vermelho
image-channel-green = Canal verde
image-channel-blue = Canal azul
image-channel-alpha = Canal alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Fundo quadriculado (K alterna)
image-bg-black-title = Fundo preto (K alterna)
image-bg-gray-title = Fundo cinza (K alterna)
image-bg-white-title = Fundo branco (K alterna)
image-bg-checker = fundo quadriculado
image-bg-black = fundo preto
image-bg-gray = fundo cinza
image-bg-white = fundo branco
image-inspect-aria = Inspecionar { $name }
image-preview-size = prévia { $width }×{ $height }, { $bytes }
image-preview = prévia
image-resolution = Resolução
image-full-title = Carregar a imagem inteira de { $width }×{ $height } (F); aumentar o zoom além da prévia também carrega
image-full-loading = Carregando a resolução original…
image-full-failed = A resolução original não carregou: tente de novo
image-full = Resolução original
image-zoom = Zoom
image-zoom-out-title = Diminuir zoom (−)
image-zoom-out = Diminuir zoom
image-zoom-in-title = Aumentar zoom (+)
image-zoom-in = Aumentar zoom
image-fit-title = Ajustar (0)
image-fit = Ajustar
image-actual-title = Pixels reais (1)
image-view = Exibição
image-tile-title = Repetir 3×3 para conferir as emendas (T)
image-tile = Repetir
image-channels = Canais
image-close = Fechar (Esc)
image-failed = Não foi possível carregar esta imagem.
image-loading = Carregando…
image-key-gestures = Role ou faça pinça para dar zoom · arraste para mover · clique duas vezes para aumentar o zoom ou ajustar
image-key-tile = repetir
image-key-background = fundo
image-key-full = resolução original
image-key-channels = canais

## SpritePlayer.vue

sprite-frame-aria = { $name }, quadro { $frame } de { $count }
sprite-failed = Não é possível tocar este.
sprite-loading = Carregando quadros…
sprite-previous = Quadro anterior
sprite-next = Próximo quadro
sprite-pause = Pausar animação
sprite-play = Tocar animação
sprite-animation = Animação
sprite-all-frames = Todos os quadros
sprite-grid-title = Como a folha é cortada em quadros
sprite-columns = Colunas
sprite-rows = Linhas
sprite-fps-aria = Quadros por segundo
sprite-fps = fps

## O visualizador de folhas de sprites (SheetView.vue) e seus formatos de download (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
       *[other] { $count } sprites
    }
sheet-pages =
    { $count ->
        [one] { $count } página
       *[other] { $count } páginas
    }
sheet-animations =
    { $count ->
        [one] { $count } animação
       *[other] { $count } animações
    }
# The box that narrows the list of sprites by name.
sheet-find = Buscar um sprite
# Under the list of sprites when more match than it draws.
sheet-showing = Mostrando { $shown } de { $total }
sheet-no-match = Nenhum sprite tem isso no nome.
sheet-failed = Não é possível ler esta folha.
sheet-loading = Carregando a folha…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Formato próprio do Hoard: as páginas como estão, sprites girados para qualquer lado e recortados, com animações e o tempo de cada quadro.
sheet-note-texturepacker = Sprites girados no sentido horário e recortados, com animações por nome, do jeito que o PixiJS e o Phaser leem.
sheet-note-clockwise = Sprites girados no sentido horário e recortados. Não comporta animações: os quadros são identificados pelos nomes numerados.
sheet-note-libgdx = Sprites girados no sentido anti-horário e recortados. Não comporta animações: os quadros são identificados pelos nomes numerados.
sheet-note-godot-frames = Sprites nunca girados, com o recorte guardado como margens, com animações e o tempo de cada quadro.
sheet-note-godot-tileset = Tiles nunca girados nem recortados, numa grade do tamanho do tile, mantendo as imagens como estão quando já estão numa grade.
sheet-note-grid = Cada sprite sem girar, no canvas inteiro, em células todas do mesmo tamanho, para qualquer programa que corte a folha em grade.
sheet-note-frames = Cada sprite sem girar, no canvas inteiro, cada um num PNG com o nome dele.

## TextViewer.vue, TextPreview.vue e text.ts

text-lang-config = Configuração
text-lang-table = Tabela
text-lang-code = Código
text-lang-script = Script
text-lang-text = Texto
text-copy-blocked = Não foi possível copiar: o navegador bloqueou
text-read-aria = Ler { $name }
text-lines =
    { $count ->
        [one] { $count } linha
       *[other] { $count } linhas
    }
text-show-as = Mostrar como
text-tree-title = Árvore recolhível (T)
text-tree = Árvore
text-pretty-title = Reindentado (P)
text-pretty = Formatado
text-raw-title = Como foi escrito (R)
text-raw = Bruto
text-document-title = Como documento (D)
text-document = Documento
text-source-title = O markdown como foi escrito (R)
text-source = Fonte
text-fold = Recolher
text-expand-all-title = Expandir tudo
text-expand-all = Expandir tudo
text-collapse-all-title = Recolher tudo
text-collapse-all = Recolher tudo
text-group-text = Texto
text-wrap-title = Quebrar linhas longas (W)
text-wrap = Quebrar linhas
text-copy-title = Copiar o texto (C)
text-copied = Copiado
text-copy = Copiar
text-download-title = Baixar o arquivo
text-download = Baixar
text-close = Fechar (Esc)
text-truncated = Mostrando os primeiros { $shown } de { $total }.
text-truncated-json = Mostrando os primeiros { $shown } de { $total } (a árvore precisa do arquivo inteiro).
text-show-all = Mostrar tudo
text-json-error = Este JSON não pôde ser interpretado: { $error }
text-contents-aria = Conteúdo de { $name }
text-loading = Carregando…
text-load-error = Não foi possível carregar este arquivo: { $error }
text-binary = Este arquivo é binário, não texto, então não há nada para ler aqui. Baixe-o para abri-lo na ferramenta própria dele.
text-key-tree = árvore
text-key-pretty = formatado
text-key-raw = bruto
text-key-document = documento
text-key-source = fonte
text-key-wrap = quebrar linhas
text-key-copy = copiar
text-key-find = localizar
text-key-close = fechar
text-preview-aria = Ler { $name } em tela cheia
text-preview-read = Ler

## JsonTree.vue e JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } item
       *[other] { $count } itens
    }
json-keys =
    { $count ->
        [one] { $count } chave
       *[other] { $count } chaves
    }
json-fold = Recolher
json-unfold = Expandir
json-unfold-summary = Expandir { $summary }
json-show-more = Mostrar mais { $count }
json-left = de { $count } restantes
json-picker-more = … mais { $count }

## Visualizadores de modelos 3D

model-viewer-failed = o visualizador 3D não carregou ({ $error }); recarregar a página pode ajudar
model-loading-viewer = Carregando o visualizador 3D…
model-loading = Carregando modelo…
model-show-failed = Não foi possível mostrar este modelo: { $error }
model-preview-failed = Não foi possível fazer a prévia deste modelo: { $error }
model-error-converting = o servidor ainda está convertendo este modelo
model-error-converting-desktop = O Hoard ainda está convertendo este modelo
model-error-empty-collada = arquivo Collada vazio
model-error-no-loader = Não há carregador no navegador para .{ $ext }
model-dialog = Ver { $name } em 3D
model-title-stats =
    { $triangles } tris · { $meshes ->
        [one] { $meshes } malha
       *[other] { $meshes } malhas
    } · { $materials ->
        [one] { $materials } material
       *[other] { $materials } materiais
    }
model-triangles =
    { $count ->
        [one] { $count } triângulo
       *[other] { $count } triângulos
    }
model-group-camera = Câmera
model-group-shading = Sombreamento
model-group-background = Fundo
model-group-animation = Animação
model-group-view = Exibição
model-group-show = Mostrar
model-frame-title = Enquadrar o modelo (F)
model-frame = Enquadrar
model-reset-title = Voltar à vista inicial (R)
model-reset = Redefinir
model-turn-title = Girar devagar (A)
model-turn = Girar
model-textures-title = Texturas (T)
model-textures = Texturas
model-full-res-title = Texturas em resolução original (H): cada arquivo de textura como é, por maior que seja; desligado, prévias leves de no máximo 2048 px
model-full-res = Res. original
model-light-title = Luz do ambiente (E)
model-light = Luz
model-grid-title = Grade do chão (G)
model-grid = Grade
model-bg-checker = Fundo quadriculado
model-bg-dark = Fundo escuro
model-bg-gray = Fundo cinza
model-bg-light = Fundo claro
model-bg-env = Fundo do ambiente
model-bg-title = { $background } (K alterna)
model-pause-title = Pausar (Espaço)
model-play-title = Tocar (Espaço)
model-pause = Pausar
model-play = Tocar
model-animation-clip = Clipe de animação
model-animation = Animação
model-plays-on = Reproduzir em
model-close-title = Fechar (Esc)
model-full-screen = Tela cheia
model-open-full-screen = Abrir o visualizador 3D em tela cheia
model-menu-fold = Recolher o menu
model-menu-show = Mostrar o menu
model-view-prefix = Vista:{ " " }
model-menu-render = Renderização
model-menu-maps = Mapas
model-menu-inspect = Inspecionar
model-mode-final = Render final
model-mode-raw = Render bruto
model-mode-color = Cor base
model-mode-roughness = Rugosidade
model-mode-metalness = Metalicidade
model-mode-normal = Normal
model-mode-ao = Oclusão
model-mode-matcap = Matcap
model-mode-uv = Verificador de UV
model-mode-geometry = Geometrias
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Wireframe (W), sobre qualquer vista
model-wireframe = Wireframe
model-info-title = Informações da cena (I)
model-info = Informações da cena
model-info-close = Fechar informações da cena
model-info-close-title = Fechar (I)
model-info-geometries = Geometrias
model-info-materials = Materiais
model-info-textures = Texturas
model-info-triangles = Triângulos
model-info-vertices = Vértices
model-info-meters = glTF é em metros
model-info-no-units = Este formato não diz quais são suas unidades
model-info-height = Altura
model-info-length = Comprimento
model-info-width = Largura
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } unidade
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } unidades
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Textura não encontrada: { $name }
model-missing-some = { $count } texturas não encontradas: { $names }
model-missing-more = { $count } texturas não encontradas: { $names }…
model-missing-count = { $count } texturas não encontradas
model-guessed = Texturas associadas pelo nome; o arquivo não cita nenhuma.
model-group-facing = Orientação
model-facing-turn = Girar um quarto
model-facing-turn-title = Girar o modelo um quarto de volta no sentido anti-horário, visto de cima
model-facing-tip = Inclinar um quarto
model-facing-tip-title = Inclinar o topo do modelo um quarto de volta para a frente, para pôr de pé um modelo deitado
model-facing-save = Usar neste pacote
model-facing-save-title = Girar assim todos os modelos de { $pack }. As miniaturas deles são desenhadas de novo; os downloads ficam como foram feitos.
model-facing-undo = Desfazer
model-facing-undo-title = Voltar a como os modelos do pacote estão girados
model-facing-saved = Todos os modelos de { $pack } agora estão virados assim. As miniaturas deles estão sendo desenhadas de novo.
model-facing-failed = Não foi possível girar os modelos do pacote: { $error }
model-keys-mouse = Arraste para orbitar · arraste com o botão direito para mover · role para dar zoom · clique duas vezes para enquadrar
model-key-frame = enquadrar
model-key-reset = redefinir
model-key-view = vista
model-key-wireframe = wireframe
model-key-info = informações da cena
model-key-textures = texturas
model-key-full-res = texturas em resolução original
model-key-light = luz
model-key-grid = grade
model-key-background = fundo
model-key-play = tocar

## Materiais

material-role-base-color = Cor
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Rugosidade
material-role-gloss = Brilho
material-role-metalness = Metalicidade
material-role-ao = Oclusão de ambiente
material-role-orm = Oclusão, rugosidade, metal
material-role-mra = Metal, rugosidade, oclusão
material-role-rma = Rugosidade, metal, oclusão
material-role-height = Altura
material-role-bump = Relevo
material-role-cavity = Cavidade
material-role-specular = Especular
material-role-opacity = Opacidade
material-role-emissive = Emissão
material-role-translucency = Translucidez
material-role-base-color-name = cor
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = rugosidade
material-role-gloss-name = brilho
material-role-metalness-name = metalicidade
material-role-ao-name = oclusão de ambiente
material-role-orm-name = oclusão, rugosidade, metal
material-role-mra-name = metal, rugosidade, oclusão
material-role-rma-name = rugosidade, metal, oclusão
material-role-height-name = altura
material-role-bump-name = relevo
material-role-cavity-name = cavidade
material-role-specular-name = especular
material-role-opacity-name = opacidade
material-role-emissive-name = emissão
material-role-translucency-name = translucidez
material-maps =
    { $count ->
        [one] { $count } mapa
       *[other] { $count } mapas
    }
material-dialog = Ver o material { $name }
material-up-to = até { $size }
material-repeat-size = { $size } por repetição
material-shape-sphere = Esfera
material-shape-cube = Cubo
material-shape-plane = Plano
material-shape-title = { $shape } ({ $key })
material-group-shape = Forma
material-group-tiling = Repetição
material-real-title = Tamanho real: uma repetição tem { $size }, a esfera tem 2 m de diâmetro (T alterna)
material-repeat-title = Repetir { $n }× (T alterna)
material-real = Real
material-repeat = { $n }×
material-height-title = Quanto o mapa de altura desloca a superfície
material-height = Altura
material-height-strength = Intensidade da altura
material-loading = Carregando material…
material-show-failed = Não foi possível mostrar este material: { $error }
material-preview-failed = Não foi possível fazer a prévia deste material: { $error }
material-group-maps = Mapas
material-map-show = Mostrar o mapa de { $map }
material-map-hide = Ocultar o mapa de { $map }
material-map-unused = Não usado pela prévia
material-open-full-screen = Abrir o visualizador de materiais em tela cheia
material-keys-mouse = Arraste para orbitar · arraste com o botão direito para mover · role para dar zoom · clique em um mapa para desligá-lo
material-key-shape = forma
material-key-tiling = repetição
material-key-turn = girar

## theme.ts

theme-slate = Ardósia
theme-paper = Papel
theme-linen = Linho
theme-sage = Sálvia
theme-lilac = Lilás
theme-blush = Rosado
theme-graphite = Grafite
theme-obsidian = Obsidiana
theme-ember = Brasa
theme-moss = Musgo
theme-aubergine = Berinjela
theme-oxblood = Bordô
theme-light-slate-blurb = Cinza-azulado frio
theme-light-paper-blurb = Cinza neutro simples
theme-light-linen-blurb = Creme quente, âmbar
theme-light-sage-blurb = Verde suave
theme-light-lilac-blurb = Violeta claro
theme-light-blush-blurb = Rosa antigo
theme-dark-graphite-blurb = Cinza neutro, sem matiz
theme-dark-slate-blurb = Cinza-azulado
theme-dark-obsidian-blurb = Quase preto, para OLED
theme-dark-ember-blurb = Carvão quente, âmbar
theme-dark-moss-blurb = Preto esverdeado de floresta
theme-dark-aubergine-blurb = Preto ameixa, lilás
theme-dark-oxblood-blurb = Preto vinho, coral

## texnames.ts

texnames-needs-map = O padrão precisa de { "{" }map{ "}" }, senão todos os mapas de um material teriam o mesmo nome
texnames-placeholders = O padrão pode usar { "{" }name{ "}" }, { "{" }map{ "}" } e { "{" }size{ "}" }
texnames-bad-characters = Os nomes não podem ter < > : " / \ | ? nem *
texnames-no-suffix = Dê um sufixo aos mapas de { $role }
texnames-bad-suffix = O sufixo dos mapas de { $role } não pode ter < > : " / \ | ? nem *
texnames-slot-no-suffix = Dê um sufixo a “{ $slot }”
texnames-slot-bad-suffix = O sufixo de “{ $slot }” não pode ter < > : " / \ | ? nem *
texnames-ending-missing = Cada terminação personalizada precisa da terminação a ser procurada
texnames-ending-bad = Terminações personalizadas não podem ter < > : " / \ | ? nem *
texnames-ending-twice = { $ending } aparece duas vezes nas suas terminações
texnames-slot-metallic-roughness = Metal e rugosidade
texnames-slot-specular-color = Cor especular
texnames-slot-specular-glossiness = Especular e brilho
texnames-slot-clearcoat = Verniz
texnames-slot-clearcoat-roughness = Rugosidade do verniz
texnames-slot-clearcoat-normal = Normal do verniz
texnames-slot-sheen-color = Cor do lustro
texnames-slot-sheen-roughness = Rugosidade do lustro
texnames-slot-transmission = Transmissão
texnames-slot-thickness = Espessura
texnames-slot-diffuse-transmission = Transmissão difusa
texnames-slot-diffuse-transmission-color = Cor da transmissão difusa
texnames-slot-iridescence = Iridescência
texnames-slot-iridescence-thickness = Espessura da iridescência
texnames-slot-anisotropy = Anisotropia

## storeFormats.ts

storefiles-field-title = Título
storefiles-field-title-hint = O título exato do anúncio. Ele dá nome ao pacote e, sem ele, o arquivo é ignorado.
storefiles-field-seller = Vendedor
storefiles-field-seller-hint = Quem fez ou vende.
storefiles-field-license = Licença
storefiles-field-license-hint = Como o arquivo escreve, por exemplo CC-BY-4.0.
storefiles-field-url = Página da loja
storefiles-field-url-hint = O endereço do anúncio, começando com http:// ou https://.
storefiles-field-words = Tags
storefiles-field-words-hint = Palavras para usar como tags. Valores separados por vírgula são divididos.
storefiles-field-type = Tipo
storefiles-field-type-hint = O que é, por exemplo “3D model” ou “surface”. Também vira tag.
storefiles-field-colors = Cores
storefiles-field-colors-hint = As cores dele, como tags color:.
storefiles-field-id = ID na loja
storefiles-field-id-hint = O ID dele na loja: letras, dígitos, _ ou -.
storefiles-field-origin = Feito em
storefiles-field-origin-hint = Onde foi feito ou escaneado, para a descrição.
storefiles-field-size = Tamanho
storefiles-field-size-hint = O tamanho, em palavras, para a descrição.
storefiles-field-title-name = o título
storefiles-field-seller-name = o vendedor
storefiles-field-license-name = a licença
storefiles-field-url-name = a página da loja
storefiles-field-words-name = a lista de tags
storefiles-field-type-name = o tipo
storefiles-field-colors-name = a lista de cores
storefiles-field-id-name = o ID na loja
storefiles-field-origin-name = o local de origem
storefiles-field-size-name = o tamanho

## StoreFormatsPanel.vue

storefiles-load-failed = Não foi possível carregar os formatos de arquivos de loja: { $error }
storefiles-remove-title = Remover { $name }?
storefiles-remove-body = { $file } será excluído da pasta de dados. Os pacotes já lidos com ele mantêm o que ele leu.
storefiles-remove = Remover
storefiles-removed = { $name } removido.
storefiles-remove-failed = Não foi possível remover { $name }: { $error }
storefiles-unread = Arquivos que o Hoard não lê
storefiles-loading = Carregando…
storefiles-unread-none = Nenhum: nenhum arquivo da biblioteca parece ser de uma loja não lida.
storefiles-unread-what = Arquivos em mais de um pacote, ou com nome de arquivo de loja, que nenhum formato lê.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pacote
       *[other] { $count } pacotes
    }
storefiles-unread-example = por exemplo { $pack }
storefiles-teach = Ensinar este arquivo ao Hoard
storefiles-formats = Formatos
storefiles-formats-what = Os seus são tentados primeiro e tomam o lugar de um embutido com o mesmo nome.
storefiles-dir-before = Eles ficam em
storefiles-dir-after = .
storefiles-yours = Seu
storefiles-built-in = Embutido
storefiles-edit = Editar
storefiles-start-from = Começar a partir deste

## TagsPanel.vue

tags-load-failed = Não foi possível carregar as tags: { $error }
tags-rename-title = Renomear { $tag }?
tags-merge-title = Mesclar { $count } tags?
tags-rename-body = Os { $count } assets dela passam a ter { $into }, e os geradores de tags usam o novo nome daqui em diante.
tags-merge-body = { $tags } viram { $into } em até { $count } assets. Os geradores de tags usam { $into } daqui em diante.
tags-rename = Renomear
tags-merge = Mesclar
tags-merged =
    { $count ->
        [one] { $count } asset agora tem a tag { $into }
       *[other] { $count } assets agora têm a tag { $into }
    }
tags-merge-failed = Não foi possível mesclar: { $error }
tags-forget-failed = Não foi possível mudar isso: { $error }
tags-find = Encontrar tags
tags-find-placeholder = Encontre tags, por exemplo rock ou set:
tags-count =
    { $count ->
        [one] { $count } tag
       *[other] { $count } tags
    }
tags-count-more = { $count }+ tags
tags-what = Escolha tags que significam a mesma coisa e mescle-as, ou escolha uma para renomeá-la.
tags-new-name = Novo nome
tags-merge-into = Mesclar em
tags-saved-as-before = Salva como
tags-saved-as-after = { "" }
tags-cancel = Cancelar
tags-loading = Carregando…
tags-none = Nenhuma tag.
tags-none-containing = Nenhuma tag contendo “{ $text }”.
tags-browse-title = Explorar assets com a tag { $tag }
tags-browse = Explorar
tags-merged-heading = Tags mescladas
tags-merged-what = Quando os geradores de tags chegam a um destes nomes antigos, o asset recebe a tag em que ele foi mesclado.
tags-allow-old = Permitir o nome antigo de novo

## LibraryStats.vue

stats-source-human = Adicionadas por pessoas
stats-source-ai = Sugeridas por IA
stats-source-rule = Regras embutidas
stats-source-meta = Dos arquivos
stats-assets = Assets
stats-packs = Pacotes
stats-on-disk = No disco
stats-duplicates-title = { $count } arquivos são cópias idênticas, byte a byte, de outros e são guardados uma vez só
stats-saved-dedup = Economizado com deduplicação
stats-distinct-tags = Tags diferentes
stats-archive = Arquivados
stats-load-failed = Não foi possível carregar os detalhes: { $error }
stats-counting = Contando…
stats-by-kind = Espaço por tipo
stats-files-size = { $name }: { $count } arquivos, { $size }
stats-months = Adicionados no último ano
stats-month = { $month }: { $count } assets, { $size }
stats-tagging = Tags
stats-tagged = Com tags
stats-untagged = { $count } sem tags
stats-described = Com descrição
stats-described-assets = { $count } assets
stats-human-edited = Editados à mão
stats-human-edited-unit =
    { $count ->
        [one] asset
       *[other] assets
    }
stats-source-different = { $count } diferentes
stats-most-used = Mais usadas
stats-show-tag = Mostrar os assets com { $tag }
stats-file-types = Tipos de arquivo
stats-biggest-packs = Maiores pacotes
stats-browse-pack = Explorar { $name }
stats-largest-files = Maiores arquivos
stats-open-file = Abrir { $name }
stats-empty = Nada na biblioteca ainda. Adicione um pacote e os números dele aparecem aqui.

## UpdatesPanel.vue

updates-saved = O Hoard vai chamá-lo quando houver uma atualização pronta
updates-saved-none = Nenhum endereço a chamar
updates-save-failed = Não foi possível salvar
updates-call-answered = Respondeu { $result }
updates-call-refused = Não aceitou a solicitação: { $result }
updates-call-failed = Não foi possível chamá-lo
updates-called = Chamado em { $when } para { $version }: { $result }
updates-called-failed = Chamado em { $when } para { $version }: { $result }. O Hoard tenta de novo em uma hora.
updates-latest = O Hoard { $version } é a versão mais recente.
updates-out = O Hoard { $version } foi lançado.
updates-out-idle = O Hoard { $version } foi lançado; a chamada acontece em alguns minutos, enquanto nada estiver acontecendo.
updates-out-busy = O Hoard { $version } foi lançado; aguardando até que nada esteja acontecendo ({ $work }).
updates-busy-more = mais { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } tarefa aguardando
       *[other] { $count } tarefas aguardando
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } envio
       *[other] { $count } envios
    }
updates-list-separator = ,{ " " }
updates-what = Quando sai uma versão nova, o Hoard pode chamar um endereço assim que nada estiver acontecendo (sem envios, tarefas, conversões ou manutenção), para que o que atualiza o contêiner dele não o reinicie no meio de um envio. Ele envia um POST com a versão em um corpo JSON. Isso precisa da verificação de versões acima.
updates-url = Endereço a chamar
updates-url-placeholder = Nenhum (por exemplo http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Salvo (digite para substituir)
updates-auth-placeholder = Nenhum (por exemplo Bearer my-token)
updates-remove = Remover
updates-save = Salvar
updates-call-now-title = O que estiver ligado a ele é executado
updates-call-now = Chamar agora
updates-what-calls = O que ele pode chamar?
updates-watchtower = A API HTTP do Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , com{ " " }
updates-watchtower-token = { " " }e o token de{ " " }
updates-watchtower-its = { " " }(com o endpoint{ " " }
updates-watchtower-endpoint = { " " }ativado).
updates-portainer = Um webhook do Portainer
updates-portainer-rest = { " " }na stack ou no contêiner do Hoard, com a opção de baixar a imagem de novo ativada. O token fica na URL.
updates-script = Um script no host
updates-script-listener = { " " }por trás de um receptor de webhooks (como{ " " }
updates-script-runs = ) que execute{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }ou qualquer outra coisa que aceite um webhook.
updates-poll-before = Ou dispense o endereço: um script com um token de acesso pode consultar{ " " }
updates-poll-until = { " " }até que{ " " }
updates-poll-and = { " " }esteja definido e{ " " }
updates-poll-after = { " " }seja true, e então baixar e reiniciar o Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Não foi possível carregar a Manutenção
maintenance-reanalyze-title = Reanalisar todos os pacotes?
maintenance-reanalyze-body = Cada arquivo é lido de novo, o que pode demorar bastante em uma biblioteca grande; os pacotes aparecem como em andamento até chegar a vez deles. Os seus nomes, tags e notas ficam.
maintenance-reanalyze-confirm = Reanalisar
maintenance-gc-title = Excluir arquivos não usados?
maintenance-gc-body = Os arquivos guardados para os quais nada na biblioteca aponta são excluídos definitivamente. Os pacotes arquivados mantêm os deles.
maintenance-gc-confirm = Excluir
maintenance-start-failed = Não foi possível iniciar “{ $task }”
maintenance-just-now = agora mesmo
maintenance-minutes-ago = há { $count } min
maintenance-hours-ago = há { $count } h
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Carregando…
maintenance-running = Em execução…
maintenance-starting = Iniciando…
maintenance-run = Executar
maintenance-started = { $detail } · iniciado { $when }
maintenance-starting-detail = Iniciando
maintenance-finished = Concluído:{ " " }
maintenance-failed = Falhou:{ " " }
maintenance-never-run = Nunca executado
maintenance-history = Execuções anteriores
maintenance-col-task = Tarefa
maintenance-col-finished = Concluída
maintenance-col-took = Duração
maintenance-col-result = Resultado
maintenance-script-before = Backups e a verificação de danos também podem ser executados por um script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }e{ " " }
maintenance-script-after = . Essas execuções também aparecem aqui.

## SettingsPage.vue

settings-title = Configurações
settings-loading = Carregando…
settings-load-failed = Não foi possível carregar as configurações: { $error }
settings-page-general = Geral
settings-page-general-blurb = Como este servidor se chama, para todos que o usam.
settings-page-general-blurb-desktop = Como esta biblioteca se chama.
settings-page-appearance = Aparência
settings-page-appearance-blurb = Os temas que este navegador usa quando está claro e quando está escuro. Salvo só neste navegador.
settings-page-library = Biblioteca
settings-page-library-blurb = O que há no Hoard: o tamanho, do que é feito, como cresceu e quão bem está marcado com tags. Itens arquivados não são contados.
settings-page-tags = Tags
settings-page-tags-blurb = Todas as tags da biblioteca. Mescle as que significam a mesma coisa, ou renomeie uma; os geradores de tags usam o novo nome dali em diante.
settings-page-store-files = Arquivos de loja
settings-page-store-files-blurb = Os arquivos que as lojas colocam junto da arte, que o Hoard lê para dar nome aos pacotes e creditar quem os fez. Ensine a ele os que ele não conhece e compartilhe o que você ensinou.
settings-page-locations = Locais da biblioteca
settings-page-locations-blurb = As pastas no servidor que contêm a biblioteca: aquelas em que o Hoard guarda arquivos e pastas de pacotes que ele indexa onde estão.
settings-page-locations-blurb-desktop = As pastas que contêm a biblioteca: aquelas em que o Hoard guarda arquivos e pastas de pacotes que ele indexa onde estão.
settings-page-tools = Ferramentas
settings-page-tools-blurb = Programas externos que o Hoard usa para converter e ler arquivos.
settings-page-ai = Tags por IA
settings-page-ai-blurb = Um modelo de IA dá nome, descreve e marca com tags os pacotes novos, além das regras embutidas. As tags dele são marcadas como sugeridas.
settings-page-search = Busca por imagem
settings-page-search-blurb = Busque imagens pelo que elas mostram, não só pelo nome, e encontre coisas parecidas no tipo, não só na cor.
settings-page-adding = Adição de pacotes
settings-page-adding-blurb = O que o Hoard faz com um pacote quando ele é adicionado, além de lê-lo e marcá-lo com tags.
settings-page-downloads = Downloads
settings-page-downloads-blurb = Como os arquivos são nomeados no que o Hoard envia: downloads, zips, a ferramenta hoard e assistentes de IA. Os arquivos da biblioteca ficam como estão.
settings-page-account = Conta
settings-page-account-blurb = Sua senha, as outras pessoas que podem entrar e tokens de acesso para scripts e outros programas.
settings-page-auth = Autenticação
settings-page-auth-blurb = Como as pessoas entram: as contas do próprio Hoard, um diretório LDAP, o Microsoft Active Directory ou um provedor OAuth, quanto duram as sessões e quais grupos recebem qual papel.
settings-page-connection = Conexão
settings-page-connection-blurb = Como navegadores e apps acessam o Hoard: o endereço pelo qual ele é conhecido, e HTTPS, que alguns recursos do navegador exigem.
settings-page-performance = Desempenho
settings-page-performance-blurb = Quanto o servidor trabalha, como as miniaturas são feitas e guardadas, e o máximo que ele recebe.
settings-page-performance-blurb-desktop = Quanto o Hoard trabalha, como as miniaturas são feitas e guardadas, e o máximo que ele recebe.
settings-page-issues = Problemas
settings-page-issues-blurb = O que deu errado ao abrir ou processar arquivos: arquivos compactados que não foram extraídos, arquivos que não puderam ser lidos, tarefas que falharam. Tente de novo depois de corrigir a causa, ou dispense-os.
settings-page-maintenance = Manutenção
settings-page-maintenance-blurb = Faça backup, verifique danos e faça uma limpeza. Uma de cada roda por vez, e você pode sair desta página enquanto elas rodam.
settings-the-default = o padrão
settings-default-on = ligado
settings-default-off = desligado
settings-default-auto = automático
settings-option-saved = { $name }: { $value }
settings-back-to-default = Voltar ao padrão
settings-save = Salvar
settings-use = Usar
settings-status-off = Desligado

## SettingsPage.vue: Aparência

settings-themes = Temas
settings-look-light = Claro
settings-look-dark = Escuro
settings-look-group =
    { $scheme ->
        [light] Tema claro
       *[dark] Tema escuro
    }
settings-preview =
    Mostrando o tema { $preview ->
        [light] claro
       *[dark] escuro
    } para você ver como fica. Você está usando o { $used ->
        [light] claro
       *[dark] escuro
    }{ $system ->
        [yes] { " " }(seguindo o sistema)
       *[other] { "" }
    }, e ele volta quando você sair desta página.
settings-preview-back =
    Voltar ao { $used ->
        [light] claro
       *[dark] escuro
    }
settings-look-hint = Alterne entre claro e escuro (ou siga o sistema) pela barra superior.

## SettingsPage.vue: Ferramentas

settings-blender-what = Converte modelos .blend, .dae, .abc e .x3d para glTF em segundo plano, para que tenham miniaturas e uma prévia 3D. Os pacotes adicionados enquanto ele estava desligado são convertidos quando ele estiver pronto.
settings-blender-download = Usa um Blender instalado, ou baixa o Blender 4.5 LTS (cerca de 400 MB) de um espelho do blender.org.
settings-ffmpeg-what = Codifica downloads em MP3, OGG e Opus, e lê áudio que o Hoard não consegue decodificar sozinho (AAC, M4A, WMA, …).
settings-ffmpeg-download = Usa um ffmpeg do sistema com os codificadores certos, ou baixa uma build estática (cerca de 80 MB).
settings-tool-reset = { $tool }: voltar ao padrão
settings-tool-no-download = Usa um { $tool } instalado. Não há build para baixar para esta máquina, então instale-o ou informe o caminho.
settings-tool-path = Caminho do programa { $tool }
settings-mode-off = Desligado
settings-mode-auto = Automático
settings-mode-path = Este programa
settings-tool-looking = Procurando…
settings-tool-downloading = Baixando… o progresso está no indicador de atividade
settings-tool-ready = Pronto: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Os instrumentos com que os arquivos MIDI são tocados e convertidos.
settings-soundfont-file = Arquivo SoundFont
settings-soundfont-placeholder = Encontrado no servidor, ou embutido
settings-soundfont-placeholder-desktop = Encontrado neste computador, ou embutido
settings-soundfont-default = encontrado no servidor, ou embutido
settings-soundfont-default-desktop = encontrado neste computador, ou embutido

## SettingsPage.vue: Busca por imagem

settings-clip-heading = Buscar pelo que as imagens mostram
settings-clip-reset = Busca por imagem: voltar ao padrão
settings-clip-saved-on = Busca por imagem: ligada
settings-clip-saved-off = Busca por imagem: desligada
settings-clip-what = Digite o que você procura (“mossy rock”, “sci-fi door”, “wooden barrel”) e as imagens que mostram isso aparecem junto com os nomes que têm essas palavras. “Mais como este” encontra coisas do mesmo tipo, não só das mesmas cores.
settings-clip-toggle = Ativar a busca por imagem
settings-clip-hint = Usa o modelo CLIP da OpenAI, executado no processador deste servidor: nada sai dele. Ativar baixa o modelo ({ $size } MB) do Hugging Face e depois aprende cada imagem em segundo plano, com baixa prioridade. Ele ocupa de 300 a 600 MB de memória enquanto trabalha ou alguém busca. Desativar exclui o modelo e mantém o que ele aprendeu, para a próxima vez.
settings-clip-hint-desktop = Usa o modelo CLIP da OpenAI, executado no processador deste computador: nada sai dele. Ativar baixa o modelo ({ $size } MB) do Hugging Face e depois aprende cada imagem em segundo plano, com baixa prioridade. Ele ocupa de 300 a 600 MB de memória enquanto trabalha ou você busca. Desativar exclui o modelo e mantém o que ele aprendeu, para a próxima vez.
settings-clip-looking = Procurando o modelo…
settings-clip-downloading = Baixando o modelo… o progresso está no indicador de atividade
settings-clip-ready-empty = Pronto. As imagens são aprendidas conforme os pacotes são adicionados.
settings-clip-ready-all =
    { $count ->
        [one] Pronto: conhece { $count } imagem
       *[other] Pronto: conhece todas as { $count } imagens
    }
settings-clip-learning = Pronto: aprendendo imagens, { $done } de { $total } até agora
settings-clip-unavailable = Não foi possível obter o modelo: { $detail }. Tentando de novo a cada hora.

## SettingsPage.vue: Tags por IA

settings-ai-service = Serviço
settings-ai-off = Desligado
settings-ai-off-hint = Os pacotes recebem tags só pelas regras embutidas.
settings-ai-local = Modelo local
settings-ai-local-hint = O serviço llm no arquivo compose do Hoard (ou cargo xtask llm). Baixe um modelo abaixo; ele roda nesta máquina e nada sai dela.
settings-ai-chatgpt = Plano do ChatGPT
settings-ai-chatgpt-hint = Usa o seu plano ChatGPT Go, Plus, Pro ou Business em vez de uma chave de API.
settings-ai-openrouter-hint = Uma chave para centenas de modelos hospedados, alguns gratuitos. Os modelos que aceitam imagens são marcados.
settings-ai-openai-hint = Hospedado pela OpenAI.
settings-ai-anthropic-hint = Hospedado pela Anthropic, pelo endpoint compatível com a OpenAI.
settings-ai-gemini-hint = Hospedado pelo Google, pelo endpoint compatível com a OpenAI.
settings-ai-ollama-hint = Um servidor Ollama que você executa. Mude o endereço se ele não for um contêiner chamado ollama.
settings-ai-custom = Outro (compatível com a OpenAI)
settings-ai-custom-hint = Qualquer coisa que fale a API de chat da OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Conectando…
settings-ai-connected =
    { $count ->
        [one] Conectado · { $count } modelo
       *[other] Conectado · { $count } modelos
    }
settings-ai-connected-none = Conectado, mas ele não lista modelos; digite o nome do modelo
settings-chatgpt-connected-none = Conectado; digite o nome do modelo
settings-ai-takes-images = aceita imagens
settings-ai-saved = Tags por IA: { $service }
settings-ai-saved-off = As tags por IA estão desligadas
settings-ai-reset = Tags por IA: voltar ao padrão
settings-ai-on = Ligado: { $service } · { $model }
settings-ai-url = Endereço
settings-ai-key = Chave de API
settings-ai-key-saved = Salva (digite para substituir)
settings-ai-key-paste = Cole a sua chave
settings-ai-key-optional = Nenhuma (opcional)
settings-ai-key-remove = Remover
settings-ai-get-key = Obter uma chave
settings-ai-model = Modelo
settings-ai-model-pick = Escolha ou digite um modelo
settings-ai-model-name = Nome do modelo
settings-ai-refresh = Atualizar
settings-ai-connect = Conectar
settings-ai-need-key = Adicione a chave de API para ver os modelos
settings-ai-vision = Enviar também as miniaturas (precisa de um modelo que aceite imagens)
settings-ai-batch = Por solicitação
settings-ai-batch-unit = assets (menos para modelos locais pequenos, mais para os grandes hospedados)

## SettingsPage.vue: Login no ChatGPT

settings-chatgpt-account = Conta
settings-chatgpt-checking = Verificando…
settings-chatgpt-status-as = Conectado como { $email }
settings-chatgpt-status = Conectado
settings-chatgpt-signed-in-as = Conectado ao ChatGPT como { $email }
settings-chatgpt-signed-in = Conectado ao ChatGPT
settings-chatgpt-signed-out = Desconectado do ChatGPT
settings-chatgpt-continue = Continuar com o ChatGPT
settings-chatgpt-sign-out = Sair
settings-chatgpt-sign-out-title = Sair do ChatGPT?
settings-chatgpt-sign-out-in-use = As tags por IA usam o seu plano do ChatGPT, então elas param até você entrar de novo ou escolher outro serviço.
settings-chatgpt-sign-out-body = O Hoard esquece o login e pede à OpenAI que o encerre.
settings-chatgpt-manage = Gerenciar uso
settings-chatgpt-finish-before = Conclua o login na janela que abriu (
settings-chatgpt-finish-link = abrir de novo
settings-chatgpt-finish-middle = ). Se terminar em uma página que não carrega, em um endereço que começa com{ " " }
settings-chatgpt-finish-after = , copie esse endereço e cole-o aqui:
settings-chatgpt-paste = O endereço em que o login terminou
settings-chatgpt-finish = Concluir
settings-chatgpt-usage = As solicitações de tags contam para o uso do seu plano do ChatGPT. Gerenciar uso mostra o que o Hoard usou e permite limitá-lo. Para desconectar o Hoard do próprio ChatGPT, vá em Configurações → Segurança e login → Fazer login com o ChatGPT em chatgpt.com.

## SettingsPage.vue: Modelos locais

settings-local-downloading-progress = Baixando { $percent }% · { $done } de { $total }
settings-local-downloading = Baixando…
settings-local-incomplete = Download interrompido
settings-local-failed = Não conseguiu iniciar da última vez
settings-local-in-use-running = Em uso · em execução
settings-local-in-use = Em uso
settings-local-starting = Iniciando…
settings-local-running = Em execução
settings-local-ready-images = Pronto · aceita imagens
settings-local-ready-text = Pronto · só texto
settings-local-delete-title = Excluir { $name }?
settings-local-delete-in-use = As tags por IA usam este modelo, então elas param até você escolher outro. Os arquivos dele são excluídos do servidor de modelos.
settings-local-delete-downloading = O download para e o que já foi baixado é excluído.
settings-local-delete-body = Os arquivos dele são excluídos do servidor de modelos. Você pode baixá-lo de novo depois.
settings-local-delete = Excluir
settings-local-deleting = Excluindo…
settings-local-cancel = Cancelar
settings-local-resume = Retomar
settings-local-download = Baixar
settings-local-start-before = Inicie-o com{ " " }
settings-local-start-or = { " " }(ou{ " " }
settings-local-start-developing = { " " }durante o desenvolvimento) e depois{ " " }
settings-local-start-retry = tente de novo
settings-local-start-after = . Um servidor que roda só um modelo também funciona: informe o nome do modelo dele abaixo.
settings-local-list = No servidor de modelos
settings-local-none = Nada baixado ainda. Escolha um abaixo; ele fica guardado no servidor, e você pode trocar quando quiser.
settings-local-catalog = Modelos para baixar
settings-local-size = { $size } GB · { $license }
settings-local-other = Outro modelo do Hugging Face
settings-local-other-placeholder = Outro do Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Adição de pacotes

settings-store-pages = Páginas de loja
settings-store-pages-what = Os pacotes novos são consultados nas lojas em segundo plano, para receberem a página da loja, o vendedor e a licença.
settings-store-lookup = Consultar páginas de loja automaticamente
settings-unreal = Pacotes do Unreal
settings-unreal-reset = Pacotes do Unreal: voltar ao padrão
settings-unreal-what = O que fazer com arquivos .uasset.
settings-conv = Regras de conversão
settings-conv-what = Converta os arquivos que o Hoard guarda para um só formato, como todo OBJ e FBX para GLB, para que a biblioteca fique toda igual. O arquivo convertido toma o lugar do original, com o nome, as tags, as coleções e os links dele. Arquivos em pastas monitoradas nunca são alterados.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Ativa
settings-conv-to = Converter para
# Heading over the file formats a rule converts.
settings-conv-from = De
settings-conv-remove = Remover
settings-conv-remove-label = Remover a regra { $n }
settings-conv-taken = Outra regra já converte este formato
settings-conv-upright = Deixar os modelos em pé ao convertê-los (Y para cima, de frente para +Z), do jeito que os modelos do pacote são mostrados
settings-conv-add = Adicionar uma regra
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Manter os originais por
settings-conv-keep-days =
    { $count ->
        [one] dia
       *[other] dias
    }
settings-conv-keep-what = Até lá, a página de um arquivo convertido pode restaurar o original. As texturas não são copiadas para dentro de um modelo convertido: ele aponta para os arquivos de imagem ao lado dele. Os arquivos .mtl ou .bin de um modelo continuam lá.
settings-conv-save = Salvar regras
settings-conv-revert = Desfazer alterações
settings-conv-existing = Converter arquivos que já estão na biblioteca
settings-conv-saved = Regras de conversão salvas. Elas valem para os pacotes adicionados daqui em diante.
settings-conv-converting =
    { $count ->
        [one] Convertendo um arquivo em segundo plano
       *[other] Convertendo { $count } arquivos em segundo plano
    }
settings-conv-nothing = Nada na biblioteca precisa ser convertido.
settings-conv-ask-title =
    { $count ->
        [one] Converter um arquivo que já está na biblioteca?
       *[other] Converter { $count } arquivos que já estão na biblioteca?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Está em um pacote.
       *[other] Estão em { $packs } pacotes.
    } Cada original fica guardado por { $days } dias, e até lá pode ser restaurado pela página dele.
settings-conv-ask-busy =
    { $count ->
        [one] Um está em um pacote que ainda está sendo adicionado; ele é convertido quando isso terminar.
       *[other] { $count } estão em pacotes que ainda estão sendo adicionados; eles são convertidos quando isso terminar.
    }
settings-conv-ask-list = Arquivos que seriam convertidos
settings-conv-ask-more = …e mais { $count }
settings-conv-ask-skipped = Ficam como estão:
settings-conv-skip-watched =
    { $count ->
        [one] um arquivo em uma pasta monitorada
       *[other] { $count } arquivos em pastas monitoradas
    }
settings-conv-skip-unity =
    { $count ->
        [one] um arquivo que um projeto Unity conhece pelo nome
       *[other] { $count } arquivos que um projeto Unity conhece pelo nome
    }
settings-conv-skip-used =
    { $count ->
        [one] um arquivo citado por uma cena ou outro arquivo
       *[other] { $count } arquivos citados por uma cena ou outro arquivo
    }
settings-conv-skip-beside =
    { $count ->
        [one] um arquivo cuja cópia convertida já está no pacote dele
       *[other] { $count } arquivos cujas cópias convertidas já estão nos pacotes deles
    }
settings-conv-skip-blender =
    { $count ->
        [one] um arquivo que o Blender ainda não leu (Configurações → Ferramentas)
       *[other] { $count } arquivos que o Blender ainda não leu (Configurações → Ferramentas)
    }
settings-conv-ask-new = Salvar, só pacotes novos
settings-conv-ask-save-convert =
    { $count ->
        [one] Salvar e convertê-lo
       *[other] Salvar e converter { $count } arquivos
    }
settings-conv-ask-convert =
    { $count ->
        [one] Convertê-lo
       *[other] Converter { $count } arquivos
    }
settings-unreal-off = Deixar como estão
settings-unreal-off-hint = Converta pela página de um pacote quando quiser.
settings-unreal-keep = Converter, manter os originais
settings-unreal-keep-hint = Texturas, sons e materiais saem em formatos comuns; o .uasset também fica.
settings-unreal-replace = Converter, descartar os originais
settings-unreal-replace-hint = Economiza espaço depois que o conteúdo é extraído. As malhas continuam aguardando o worker do Unreal.

## SettingsPage.vue: Downloads

settings-tn = Nomes de texturas
settings-tn-saved = Nomes de texturas salvos
settings-tn-reset = Nomes de texturas: voltar ao padrão
settings-tn-what = Os mapas de um material saem com o nome dele e a função de cada mapa, seja qual for o nome dos arquivos: Rock_Normal.png, não rock_moss_xeokah0_raw_8K_Normal.png. Os arquivos da biblioteca não são tocados, e um mapa que um modelo ou MTL enviado junto cita mantém esse nome, para o modelo continuar a encontrá-lo.
settings-tn-toggle = Dar aos mapas de textura o nome do material
settings-tn-style = Estilo
settings-tn-style-group = Estilo de nomes
settings-tn-descriptive = Descritivo
settings-tn-short = Curto
settings-tn-custom = Personalizado
settings-tn-pattern = Padrão
settings-tn-hint-name = { " " }o material,{ " " }
settings-tn-hint-map = { " " }a função do mapa,{ " " }
settings-tn-hint-size = { " " }o tamanho, quando o conjunto vem em mais de um (adicionado no fim se o padrão não o incluir).
settings-tn-example = Exemplo
settings-tn-suffixes = Sufixos para cada tipo de mapa
settings-tn-slots = Nos downloads de GLB, uma textura embutida em um modelo sem arquivo próprio recebe o nome do material e do slot que preenche, e esses slots também têm sufixos:
settings-tn-endings = Terminações personalizadas
settings-tn-endings-what = Uma textura que não é um dos mapas de um material mantém o próprio nome, a menos que termine em uma destas: _diff → _BC envia rock_diff.png como rock_BC.png. Deixe o segundo vazio para tirar a terminação.
settings-tn-ending-from = Terminação
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Trocar por
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Adicionar uma terminação
settings-tn-ending-remove = Remover
settings-tn-ending-remove-label = Remover { $ending }

## SettingsPage.vue: Geral e Conexão

settings-name = Nome
settings-name-what = Como este servidor se chama na barra superior, na aba do navegador e na página de login, para você distinguir um do outro quando tiver mais de um Hoard.
settings-name-what-desktop = Como esta biblioteca se chama na barra superior.
settings-address = Endereço
settings-address-what = O endereço que pessoas e apps usam para acessar o Hoard, quando não é o que este navegador está usando: o de um proxy reverso, o de um túnel ou um nome da tailnet, como https://hoard.example.com. Os links que o Hoard fornece (ao Claude e a outros apps, no login, no certificado dele) usam esse endereço.
settings-public-url = Endereço público
settings-public-url-placeholder = Do endereço do navegador (por exemplo https://hoard.example.com)
settings-public-url-default = do endereço do navegador

## ProfilerPanel.vue: o profiler no topo de Configurações → Desempenho

profiler = Profiler
profiler-what = O que o Hoard fez nos últimos dez minutos, atualizado a cada dois segundos enquanto você acompanha. Fica só na memória e recomeça do zero quando o Hoard reinicia.
profiler-starting = As primeiras leituras chegam em instantes.
profiler-failed = Não foi possível ler o profiler: { $error }
profiler-cpu = Uso do processador
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Máquina inteira
profiler-cpu-summary = O Hoard está usando { $hoard } dos processadores, e a máquina inteira, { $machine }.
profiler-memory = Memória
profiler-serving = Solicitações em andamento
profiler-working = Trabalhando em
profiler-working-none = Nada foi executado em segundo plano.
profiler-working-more =
    { $count ->
        [one] E mais uma.
       *[other] E mais { $count }.
    }
profiler-requests = Tempo atendendo solicitações
profiler-requests-what = Cada barra é dez segundos de solicitações, com o tempo dividido conforme onde foi gasto.
profiler-part-db = Banco de dados
profiler-part-db-title = Esperando o banco de dados e executando as consultas dele
profiler-part-disk = Disco
profiler-part-disk-title = Lendo arquivos armazenados e compactando o que é enviado
profiler-part-send = Envio
profiler-part-send-title = Esperando enquanto a rede recebe o que foi enviado
profiler-part-other = Outros
profiler-part-other-title = Elaborando a resposta
profiler-request-count =
    { $count ->
        [one] { $count } solicitação
       *[other] { $count } solicitações
    }
profiler-requests-none = Nenhuma solicitação nos últimos dez minutos.
profiler-slowest = Mais lenta: { $route }, { $time }
profiler-routes = Por rota
profiler-route = Rota
profiler-count = Solicitações
profiler-average = Média
profiler-p95 = 5% mais lentas
profiler-split = Onde o tempo foi gasto
profiler-sent = Enviado
profiler-portal = os arquivos da própria interface

## SettingsPage.vue: Desempenho

settings-render = Miniaturas 3D
settings-render-toggle = Renderizar miniaturas de modelos 3D
settings-gpu-on = Na GPU: { $renderer }
settings-gpu-software = Nenhuma GPU encontrada, então por software ({ $renderer })
settings-gpu-none = Nenhuma GPU nem renderizador por software encontrado, então os modelos ficam sem miniatura
settings-thumbs = Miniaturas
settings-thumbs-what = Como as novas miniaturas e vistas de modelos são guardadas. As que têm transparência, e pixel art, são sempre PNG. Para reduzir as que você já tem, execute Reduzir miniaturas em Manutenção.
settings-previews-size = As prévias ocupam { $size } agora.
settings-thumbs-saved = Miniaturas: { $quality }
settings-thumbs-small = Menores
settings-thumbs-small-hint = JPEG, qualidade 70. Bom na grade; um pouco suave de perto.
settings-thumbs-balanced = Equilibradas
settings-thumbs-balanced-hint = JPEG, qualidade 82. Difícil de distinguir do original no tamanho de miniatura.
settings-thumbs-sharp = Mais nítidas
settings-thumbs-sharp-hint = JPEG, qualidade 92. Cerca do dobro do tamanho de Equilibradas.
settings-thumbs-lossless = Sem perdas
settings-thumbs-lossless-hint = PNG, exatamente como foi feita. Várias vezes maior.
settings-motion-thumbs = Miniaturas em movimento para animações
settings-motion-thumbs-what = Cada animação também ganha um filme curto em loop, que toca enquanto você aponta para o bloco dela. Cada um ocupa cerca de 40 KB e leva um tempo para ser feito numa biblioteca grande. Desativar apaga todos.
settings-limits = Trabalho e limites
settings-limits-what = Cada um é salvo quando você sai do campo ou pressiona Enter. Tamanhos aceitam unidades: 100 GB, 512 MB, 1.5 TB; um número sem unidade é GB.
settings-workers = Trabalhadores em segundo plano
settings-workers-unit = tarefas ao mesmo tempo (importação, tags, conversões)
settings-max-upload = Maior envio
settings-max-upload-unit = por arquivo
settings-max-expand = Maior pacote
settings-max-expand-unit = depois de extraído
settings-conversion-cache = Downloads convertidos
settings-conversion-cache-unit-now = guardados no máximo, { $size } agora; os usados há mais tempo saem primeiro
settings-conversion-cache-unit = guardados no máximo; os usados há mais tempo saem primeiro

## SettingsPage.vue: Manutenção

settings-updates = Atualizações
settings-version = Este é o Hoard { $version }.
settings-update-check = Verificar no GitHub a cada poucas horas se há uma versão nova
settings-desktop-updates = O Hoard procura uma nova versão sempre que é iniciado, baixa em segundo plano e instala quando você o fecha.

## Tipos de asset (api.ts)

api-kind-texture = Texturas
api-kind-vector = Arte vetorial
api-kind-sheet = Folhas de sprites
api-kind-audio = Áudio
api-kind-model = Modelos
api-kind-animation = Animações
api-kind-font = Fontes
api-kind-shader = Shaders
api-kind-material = Materiais
api-kind-video = Vídeo
api-kind-document = Documentos
api-kind-data = Dados
api-kind-other = Outros
api-kind-texture-name = texturas
api-kind-vector-name = arte vetorial
api-kind-sheet-name = folhas de sprites
api-kind-audio-name = áudio
api-kind-model-name = modelos
api-kind-animation-name = animações
api-kind-font-name = fontes
api-kind-shader-name = shaders
api-kind-material-name = materiais
api-kind-video-name = vídeo
api-kind-document-name = documentos
api-kind-data-name = dados
api-kind-other-name = outros

## Avisos, diálogos e carregamento (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Desfazer
ui-today = hoje
ui-yesterday = ontem
ui-days-ago =
    { $days ->
        [one] há { $days } dia
       *[other] há { $days } dias
    }
ui-notifications = Notificações
ui-dismiss = Dispensar
ui-cancel = Cancelar
ui-lazy-loading = Carregando { $what }…
ui-lazy-failed = Não foi possível carregar { $what }.
ui-lazy-reload = Recarregar a página

## ResetButton.vue

reset-title = Voltar ao padrão: { $value }
reset-label = Voltar ao padrão, { $value }

## SizeInput.vue

size-invalid = Um tamanho como 100 GB, 512 MB ou 1.5 TB
size-hint = Um tamanho como 100 GB, 512 MB ou 1.5 TB; um número sem unidade é GB

## AuthPanel.vue (Configurações → Autenticação)

auth-load-failed = Não foi possível carregar as configurações de autenticação: { $error }
auth-saved = Salvo. Vale a partir do próximo login.
auth-method-internal = Interno
auth-method-internal-hint = Nomes e senhas guardados pelo Hoard, adicionados em Configurações → Conta.
auth-method-ldap-hint = Um diretório confere o nome e a senha digitados na página de login: OpenLDAP, FreeIPA, LLDAP ou outro servidor LDAP.
auth-method-ad-hint = Contas de domínio do Windows entram com o nome e a senha de sempre.
auth-method-oauth-hint = Um botão “Entrar com…” que leva ao Google, Microsoft, GitHub, Keycloak, Authentik ou outro provedor e volta.
auth-session-unused = Depois de 30 dias sem uso
auth-session-1h = 1 hora depois de entrar
auth-session-8h = 8 horas depois de entrar
auth-session-day = Um dia depois de entrar
auth-session-week = Uma semana depois de entrar
auth-session-30d = 30 dias depois de entrar
auth-session-hours =
    { $hours ->
        [one] { $hours } hora depois de entrar
       *[other] { $hours } horas depois de entrar
    }
auth-role-view = Pode ver
auth-role-edit = Pode alterar
auth-role-admin = Administrador
auth-role-word-view = pode ver
auth-role-word-edit = pode alterar
auth-role-word-admin = administrador
auth-role-word-default = { $role } (o papel padrão)
auth-tls-starttls-hint = LDAP simples (porta 389) passado para TLS antes de qualquer coisa ser enviada.
auth-tls-ldaps-hint = TLS desde o início (porta 636).
auth-tls-none = Nenhuma
auth-tls-none-hint = As senhas atravessam a rede como estão. Só em uma rede em que você confia.
auth-test-failed = Não foi possível testar: { $error }
auth-preset-custom = Outro (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel e outros: informe o emissor e Descobrir preenche os endereços.
auth-preset-google-note = Crie um cliente OAuth (Aplicativo da Web) no Google Cloud Console → APIs e serviços → Credenciais. O Google não envia grupos, então dê os papéis em Configurações → Conta, e desative a criação de contas no primeiro login, a menos que qualquer pessoa com uma conta Google deva poder entrar.
auth-preset-microsoft-note = Registre um app no Entra ID → Registros de aplicativo (com um redirecionamento Web para o endereço abaixo). Coloque o ID do seu locatário no lugar de “organizations” para deixar entrar só a sua organização. Para papéis a partir de grupos, adicione a declaração groups em Configuração de token: ela lista IDs de grupos.
auth-preset-github-note = Crie um OAuth app no GitHub → Settings → Developer settings. O GitHub não envia grupos, então dê os papéis em Configurações → Conta, e desative a criação de contas no primeiro login, a menos que qualquer pessoa com uma conta GitHub deva poder entrar.
auth-preset-gitlab-note = Crie um aplicativo no GitLab → Preferências → Aplicativos (ou na área de administração da sua instância: troque gitlab.com nos endereços).
auth-discovered = Os endereços do provedor foram preenchidos.
auth-copied = Copiado
auth-copy-failed = Não foi possível copiar: selecione e copie você mesmo
auth-sign-in-with = Entrar com
auth-own-accounts-still-work = As contas do próprio Hoard continuam entrando com as próprias senhas, então um diretório ou provedor fora do ar ou mal configurado não consegue deixar você trancado do lado de fora.
auth-sessions = Sessões
auth-sign-out-after = Desconectar as pessoas
auth-ldap-server = Servidor LDAP
auth-server-type = Tipo de servidor
auth-server-type-custom = Personalizado
auth-servers = Servidores
auth-server-n = Servidor { $n }
auth-remove-server-n = Remover o servidor { $n }
auth-remove = Remover
auth-add-server = Adicionar um servidor
auth-servers-hint = O próximo é tentado quando um não pode ser acessado.
auth-security = Segurança
auth-connection-security = Segurança da conexão
auth-skip-verify = Não verificar o certificado do servidor (só para testar)
auth-ca = Autoridade certificadora
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (vazio: as que o sistema considera confiáveis)
auth-lookup = Busca de pessoas
auth-anonymously = Anonimamente
auth-reader-account = Conta de leitura
auth-reader-password = A senha dela
auth-saved-type-to-replace = Salva: digite para substituir
auth-people-under = As pessoas ficam em
auth-name-attribute = Atributo do nome
auth-only-matching = Só entradas que correspondem a
auth-groups-under = Os grupos ficam em
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (vazio: só memberOf)
auth-their-groups = Os grupos da pessoa
auth-group-filter-hint = { "{" }dn{ "}" } é a entrada da pessoa, { "{" }user{ "}" } o nome com que ela entrou.
auth-test = Teste
auth-test-hint = Acessa cada servidor e entra com a conta de leitura. Com um nome e uma senha, entra também como essa pessoa e mostra o que o Hoard concluiria. Nada é salvo.
auth-test-user = Alguém com quem entrar
auth-test-user-placeholder = Alguém com quem entrar (opcional)
auth-test-password = A senha dessa pessoa
auth-testing = Testando…
auth-test-connectivity = Testar a conectividade
auth-test-server-ok = acessado, e a conta de leitura entrou
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }entrou (
auth-test-signed-in-after = ).
auth-test-role = A pessoa teria: { $role }.
auth-test-no-role = A pessoa não está em nenhum dos grupos listados em Papéis, então seria recusada.
auth-test-groups = Grupos: { $groups }
auth-test-no-groups = Nenhum grupo encontrado.
auth-provider = Provedor
auth-redirect = Endereço de redirecionamento
auth-copy = Copiar
auth-redirect-hint = Informe isto ao provedor quando criar o cliente do Hoard. Vem do endereço público em Configurações → Conexão, ou do endereço que você está usando.
auth-issuer = Emissor
auth-discover = Descobrir
auth-client-id = ID do cliente
auth-client-secret = Segredo do cliente
auth-authorization-url = Endereço de autorização
auth-token-url = Endereço do token
auth-userinfo-url = Endereço de informações do usuário
auth-userinfo-placeholder = (vazio: o token de ID diz quem é)
auth-scopes = Escopos
auth-name-from = Nome a partir de
auth-groups-from = Grupos a partir de
auth-groups-from-placeholder = (nenhum)
auth-button = Botão
auth-button-placeholder = sua conta
auth-accounts = Contas
auth-provision = Criar uma conta no Hoard na primeira vez que alguém entrar
auth-provision-on-hint = Caso contrário, um administrador adiciona cada pessoa antes, em Configurações → Conta.
auth-provision-off-hint = Um administrador adiciona cada pessoa antes, em Configurações → Conta, escolhendo como ela entra.
auth-open-to-anyone = Qualquer pessoa com uma conta { $provider } pode entrar. Adicione grupos de visualização abaixo, ou desative isto e adicione as pessoas você mesmo.
auth-new-accounts = Contas novas
auth-roles-from-groups = Papéis a partir de grupos
auth-roles-hint = Grupos cujos membros recebem cada papel, um por linha, pelo nome ou DN. Vale o mais alto que corresponder, e isso é verificado a cada login. Sem nenhum listado, as contas novas recebem o papel acima e um administrador o altera depois. Com grupos listados em Pode ver, só membros de um grupo listado podem entrar.
auth-roles-hint-oauth = Grupos cujos membros recebem cada papel, um por linha, pelo nome ou DN (ou o que a declaração de grupos listar). Vale o mais alto que corresponder, e isso é verificado a cada login. Sem nenhum listado, as contas novas recebem o papel acima e um administrador o altera depois. Com grupos listados em Pode ver, só membros de um grupo listado podem entrar.
auth-saving = Salvando…
auth-save = Salvar

## HttpsPanel.vue (Configurações → Conexão)

https-load-failed = Não foi possível carregar o HTTPS: { $error }
https-off-redirecting = O HTTPS está desligado. Levando você para http://
https-now-on = O HTTPS está ligado
https-saved = Salvo. As novas conexões usam o novo certificado.
https-now-off = O HTTPS está desligado
https-http3-failed-toast = Não foi possível iniciar o HTTP/3: { $error }
https-copied = Copiado
https-copy-failed = Não foi possível copiar: selecione e copie você mesmo
https-mode-off = Desligado
https-mode-off-hint = Só http:// simples.
https-mode-own = Certificado do próprio Hoard
https-mode-own-hint = O Hoard cria um para os endereços deste servidor. Instale a autoridade certificadora do Hoard uma vez em cada dispositivo e os navegadores dele passam a confiar nele.
https-mode-custom = Meu certificado
https-mode-custom-hint = Um que você já tem, por exemplo do tailscale cert ou do Let’s Encrypt. Os navegadores confiam nele sem instalar nada.
https-untrusted-own = Este navegador ainda não confia nele para { $host }: instale a autoridade certificadora do Hoard abaixo. Você verá um aviso que pode ignorar.
https-untrusted-uncovered = Este navegador ainda não confia nele para { $host }: o certificado não cobre este endereço. Você verá um aviso que pode ignorar.
https-untrusted = Este navegador ainda não confia nele para { $host }. Você verá um aviso que pode ignorar.
https-now = Agora
https-start-failed = O HTTPS está configurado, mas não conseguiu iniciar: { $error }
https-on-port = O HTTPS está ligado, na porta { $port }, como o http://.
https-using-it = Você está usando agora.
https-open = Abrir { $url }
https-trusted = Este navegador confia nele.
https-cert-for-before = Certificado para{ " " }
https-cert-for-after = , até { $date }.
https-cert-for-after-own = , até { $date } (renovado sozinho).
https-not-covered-before = Ele não cobre{ " " }
https-not-covered-after = , o endereço que você está usando.
https-is-off = O HTTPS está desligado: as páginas são servidas só por http://.
https-addresses = Endereços
https-addresses-hint = O certificado cobre os endereços que o Hoard encontrou e qualquer um que você adicionar (um nome que o seu roteador dá a ele, um nome do Tailscale).
https-remove-name = Remover { $name }
https-another-address = Outro endereço
https-another-address-placeholder = Outro endereço, por exemplo nas.local
https-add = Adicionar
https-your-cert = Seu certificado
https-saved-cert-before = Salvo:{ " " }
https-saved-cert-after = { " " }de { $issuer }, até { $date }. Cole um novo para substituí-lo.
https-pem-before = O certificado (com a cadeia) e a chave privada, como texto PEM: os arquivos que começam com{ " " }
https-pem-after = .
https-certificate = Certificado
https-choose-file = Escolher arquivo…
https-private-key = Chave privada
https-checked-before = Para{ " " }
https-checked-after = , de { $issuer }, até { $date }.
https-checked-after-days =
    , de { $issuer }, até { $date } ({ $days ->
        [one] { $days } dia
       *[other] { $days } dias
    }).
https-self-signed = É autoassinado: os navegadores vão avisar sobre ele.
https-key-matches = A chave corresponde.
https-where-to-get = Onde conseguir um
https-get-tailscale-before = ative o HTTPS na sua tailnet, depois execute{ " " }
https-get-tailscale-after = { " " }no servidor e envie os dois arquivos. Eles duram 90 dias: execute de novo e envie-os de novo antes disso.
https-get-acme-title = Let’s Encrypt ou outro cliente ACME
https-get-acme-before = (certbot, acme.sh, lego) para um domínio seu: envie{ " " }
https-get-acme-and = { " " }e{ " " }
https-get-acme-after = .
https-get-proxy-title = Um proxy reverso
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) que tenha o certificado: deixe o HTTPS desligado aqui e deixe o proxy enviar http:// simples ao Hoard.
https-trying = Testando…
https-apply = Aplicar
https-apply-hint = O Hoard testa a nova configuração primeiro e mantém a que tem se ela não funcionar.
https-trust-title = Confie no Hoard nos seus dispositivos
https-download-ca = Baixar hoard-ca.crt
https-trust-hint = Instale a autoridade certificadora do Hoard uma vez em cada dispositivo, e os navegadores dele passam a confiar neste servidor (e em qualquer certificado que o Hoard criar para ele depois). Ela só pode atestar este Hoard. Impressão digital (SHA-256):
https-ca-windows = Abra o arquivo → Instalar Certificado → Usuário Atual → Colocar todos os certificados no repositório a seguir → Procurar → Autoridades de Certificação Raiz Confiáveis → Concluir. Reinicie o navegador.
https-ca-macos = Abra o arquivo; ele vai para o Acesso às Chaves. Clique duas vezes em “{ $subject }” lá → Confiar → Ao usar este certificado: Sempre Confiar.
https-ca-ios-title = iPhone e iPad
https-ca-ios = Baixe-o no Safari e permita. Ajustes → Perfil Transferido → Instalar. Depois Ajustes → Geral → Sobre → Ajustes de Confiança de Certificados → ative “{ $subject }”.
https-ca-android = Configurações → Segurança → Mais configurações de segurança → Criptografia e credenciais → Instalar um certificado → Certificado de CA, e escolha o arquivo. (Os nomes variam um pouco entre celulares; busque “certificado” nas Configurações.)
https-ca-linux-1 = Debian e Ubuntu:{ " " }
https-ca-linux-2 = . Fedora e Arch:{ " " }
https-ca-linux-3 = . O Chrome no Linux tem a própria lista: Configurações → Privacidade e segurança → Segurança → Gerenciar certificados → Importar.
https-ca-firefox = O Firefox tem a própria lista: Configurações → Privacidade e Segurança → Certificados → Ver certificados → Autoridades → Importar, e marque “Confiar nesta CA para identificar sites”.
https-ca-cli-title = A ferramenta de linha de comando hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , ou coloque o caminho em{ " " }
https-ca-cli-after = .
https-options = Opções
https-redirect = Mandar navegadores de outros dispositivos de http:// para https://
https-redirect-hint = Só os carregamentos de página são redirecionados. A ferramenta de linha de comando hoard, o worker do Unreal, scripts e apps de IA continuam funcionando por http://, assim como http://localhost no próprio servidor.
https-http3-hint-before = Mais rápido em Wi-Fi instável e em celulares. Precisa de HTTPS, e da porta UDP { $port } aberta além da TCP (no Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Em execução.
https-http3-waits = Ele inicia quando o HTTPS estiver ligado.
https-http3-failed = Não foi possível iniciar o HTTP/3: { $error }
https-lockout-title = Se você ficar trancado do lado de fora
https-lockout-1 = { "" }
https-lockout-2 = { " " }no próprio servidor sempre funciona por http://. Ou desligue o HTTPS pela linha de comando do servidor e inicie o Hoard de novo:{ " " }
https-lockout-3 = { " " }(no Docker:{ " " }
https-lockout-4 = , depois{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Configurações → Conta)

account-role-view = Pode ver
account-role-view-hint = Buscar, ver prévias e baixar
account-role-edit = Pode alterar
account-role-edit-hint = Também adicionar, alterar, marcar com tags, organizar em coleções e arquivar
account-role-admin = Administrador
account-role-admin-hint = Também Configurações, contas e manutenção, e excluir definitivamente
account-source-internal = Uma senha do Hoard
account-source-ldap = A senha do { $directory } da pessoa
account-source-oauth = Entrando com { $provider }
account-source-word-directory = diretório
account-load-failed = Não foi possível carregar as contas: { $error }
account-failed = { $what }: { $error }
account-password-failed = Não foi possível alterar a sua senha
account-password-changed = Senha alterada. Seus outros dispositivos foram desconectados.
account-add-failed = Não foi possível adicionar { $name }
account-added = { $name } já pode entrar.
account-role-failed = Não foi possível alterar o que { $name } pode fazer
account-remove-title = Remover { $name }?
account-remove-body = A pessoa é desconectada em todo lugar e não pode entrar de novo. Nada do que ela adicionou à biblioteca é removido.
account-remove = Remover
account-remove-failed = Não foi possível remover { $name }
account-token-failed = Não foi possível criar o token
account-copied = Copiado
account-copy-token-failed = Não foi possível copiar: selecione o token e copie você mesmo
account-copy-failed = Não foi possível copiar: selecione e copie você mesmo
account-revoke-title = Revogar “{ $name }”?
account-revoke-body = O que ainda o usar será recusado daqui em diante.
account-revoke = Revogar
account-revoke-failed = Não foi possível revogar “{ $name }”
account-disconnect-title = Desconectar { $app }?
account-disconnect-body = Ele será recusado daqui em diante e precisará ser conectado de novo para usar o Hoard.
account-disconnect = Desconectar
account-disconnect-failed = Não foi possível desconectar { $app }
account-guest-failed = Não foi possível alterar isso
account-never = nunca
account-you = Você
account-sign-out = Sair
account-signed-in-as-before = Conectado como{ " " }
account-signed-in-as-admin = , que é administrador.
account-signed-in-as-edit = , que pode alterar a biblioteca.
account-signed-in-as-view = , que pode ver, mas não alterar nada.
account-app-signin = O app Hoard faz o login por você, então não há senha. Crie abaixo um token de acesso para os plugins das engines e o comando hoard.
account-password-at-provider = Sua senha fica com o provedor pelo qual você entra, então altere-a lá.
account-password-at-directory = Sua senha fica com o diretório pelo qual você entra, então altere-a lá.
account-current-password = Senha atual
account-new-password = Nova senha
account-new-password-again = Nova senha de novo
account-change-password = Alterar senha
account-password-mismatch = As novas senhas não coincidem.
account-password-rule = Pelo menos 8 caracteres.
account-people = Pessoas
account-people-hint = Cada pessoa pode ver (buscar, ver prévias e baixar), também alterar a biblioteca (adicionar, editar, marcar com tags, organizar em coleções e arquivar), ou ainda ser administrador (Configurações, contas e manutenção, e excluir definitivamente).
account-you-tag = (você)
account-signs-in-oauth = Entra pelo provedor OAuth
account-signs-in-directory = Entra pelo diretório
account-since = desde { $date }
account-what-can-do = O que { $name } pode fazer
account-name = Nome
account-how-sign-in = Como a pessoa entra
account-password-label = Senha (pelo menos 8 caracteres)
account-password-placeholder = Senha (8+ caracteres)
account-what-they-can-do = O que a pessoa pode fazer
account-add = Adicionar
account-ai-apps = Claude e outros apps de IA
account-ai-apps-hint = Conecte o Claude (ou qualquer app MCP) e ele pode buscar e trazer assets para você, e descrever e marcar com tags o que o Hoard ainda não marcou. Ele entra no Hoard na primeira vez; você autoriza aqui no navegador.
account-mcp-address = Endereço MCP do Hoard
account-copy = Copiar
account-claude-code-1 = execute{ " " }
account-claude-code-2 = , depois{ " " }
account-claude-code-3 = { " " }para entrar.{ " " }
account-claude-code-4 = { " " }descreve um pacote.
account-claude-ai = Configurações → Conectores → Adicionar conector personalizado, cole o endereço e depois Conectar. Eles se conectam a partir dos servidores da Anthropic, então o Hoard precisa ser acessível pela internet via HTTPS.
account-claude-ai-local = Configurações → Conectores → Adicionar conector personalizado, cole o endereço e depois Conectar. Eles se conectam a partir dos servidores da Anthropic, então o Hoard precisa ser acessível pela internet via HTTPS (este endereço não é: defina o Endereço público em Configurações → Conexão).
account-connection-meta = para { $user } desde { $since } · último uso { $used }
account-no-apps = Nenhum app conectado ainda.
account-tokens = Tokens de acesso
account-tokens-hint-1 = Para a ferramenta de linha de comando{ " " }
account-tokens-hint-2 = , scripts, clientes MCP e o worker do Unreal: eles o enviam como{ " " }
account-tokens-hint-3 = . Um token pode fazer o que você escolher, até o que você pode, mas nunca gerenciar contas ou tokens. Um que só vê basta para buscar e baixar, e para sincronizar um projeto.
account-tokens-admin = Você vê os tokens e apps conectados de todos; os outros veem os próprios.
account-made-before = { "" }
account-made-after = : copie agora. Ele não será mostrado de novo.
account-new-token = O novo token
account-cli-1 = Para a CLI:{ " " }
account-cli-2 = , ou coloque-o em{ " " }
account-cli-3 = .
account-done = Pronto
account-token-meta = criado { $made } · último uso { $used }
account-token-meta-by = criado { $made } por { $by } · último uso { $used }
account-no-tokens = Nenhum token ainda.
account-token-name = O que vai usá-lo
account-token-name-placeholder = O que vai usá-lo, por exemplo “worker do Unreal”
account-token-role = O que ele pode fazer
account-make-token = Criar um token
account-guests = Visitantes
account-guests-allow = Deixar qualquer pessoa que acesse o servidor explorar sem entrar
account-guests-hint = Elas podem buscar, ver prévias e baixar, mas não adicionar, alterar ou excluir nada, nem ver as Configurações.

## SignIn.vue

signin-create-title = Crie a sua conta
signin-title = Entrar
signin-setup-lede = Este servidor ainda não tem nenhuma conta. A que você criar aqui pode fazer tudo; adicione contas para outras pessoas em Configurações → Conta depois.
signin-overlay-lede = Aqui qualquer pessoa pode explorar, mas alterações precisam de uma conta.
signin-with = Entrar com { $provider }
signin-or-password = ou com nome e senha
signin-username = Nome de usuário
signin-name = Nome
signin-new-password = Nova senha
signin-password = Senha
signin-password-rule = Pelo menos 8 caracteres.
signin-confirm-password = Confirmar senha
signin-mismatch = As duas senhas não coincidem.
signin-setup-code = Código de configuração
signin-setup-code-hint-before = O código dado a este servidor em{ " " }
signin-setup-code-hint-after = .
signin-creating = Criando…
signin-signing-in = Entrando…
signin-create = Criar conta
signin-submit = Entrar
signin-keep-browsing = Continuar explorando
signin-directory-hint = Use o seu nome e a sua senha do { $directory }.
signin-forgot-1 = Esqueceu uma senha do Hoard? No servidor, execute{ " " }
signin-forgot-2 = { " " }(no Docker,{ " " }
signin-forgot-3 = ) para criar uma nova.

## SetupWizard.vue (primeira visita a um servidor novo)

setup-title = Onde a sua biblioteca deve ficar?
setup-lede = Escolha uma pasta no servidor em que o Hoard vai guardar os seus pacotes, ou aponte-o para uma pasta de pacotes que você já tem. Você pode adicionar mais locais, em outros discos, pelas Configurações depois.
setup-lede-desktop = Escolha uma pasta em que o Hoard vai guardar os seus pacotes, ou aponte-o para uma pasta de pacotes que você já tem. Você pode adicionar mais locais, em outros discos, pelas Configurações depois.
setup-unreachable = Não foi possível acessar o servidor: { $error }
setup-unreachable-desktop = O Hoard não está respondendo: { $error }
setup-try-again = Tentar de novo
setup-loading = Carregando…
setup-start = Iniciar a biblioteca
setup-ready = Sua biblioteca está pronta
setup-managed-before = Os pacotes novos vão para{ " " }
setup-managed-after = . Solte arquivos, pastas ou arquivos compactados em qualquer lugar da página para adicionar os primeiros.
setup-watched-before = O Hoard está monitorando{ " " }
setup-watched-after = ; os pacotes aparecem conforme ele os encontra. Para também enviar arquivos, adicione uma biblioteca guardada pelo Hoard em Configurações → Locais da biblioteca.
setup-open = Abrir a biblioteca

## ConnectApp.vue (um app como o Claude pede para entrar)

connect-title = Conectar { $app }?
connect-lede-1 = { " " }poderá buscar, baixar, descrever e marcar com tags tudo neste Hoard, agindo como{ " " }
connect-lede-2 = . Ele não pode gerenciar contas nem tokens de acesso.
connect-back-before = Você vai voltar para{ " " }
connect-back-after = . Desconecte-o quando quiser em Configurações → Conta.
connect-allow = Permitir
connect-cancel = Cancelar
connect-failed = Não é possível conectar este app
connect-go-library = Ir para a biblioteca
connect-checking = Verificando…

## InsecureNotice.vue (a linha mostrada sobre http:// simples)

insecure-text = Isto é só http://, então algumas coisas precisam de HTTPS: 3D com WebGPU, a área de transferência moderna, instalar o Hoard como app e os conectores do Claude.
insecure-switch = Mudar para https://
insecure-set-up = Configurar o HTTPS
insecure-dismiss = Não mostrar de novo

## FormatWizard.vue (Configurações → Arquivos de loja → ensinar ao Hoard o arquivo de uma loja)

formatwizard-too-big = Este arquivo é grande demais para ser lido como arquivo de loja.
formatwizard-not-json = Este arquivo não é um JSON válido, então é mostrado como texto.
formatwizard-read-failed = Não foi possível ler o arquivo: { $error }
formatwizard-back-title = Voltar ao formulário?
formatwizard-back-body = As alterações feitas no TOML se perdem: o formulário escreve o formato de novo a partir do que tem.
formatwizard-back-to-form = Voltar ao formulário
formatwizard-miss-name = Os arquivos dele não aceitam o nome { $file }. Adicione-o em Arquivos.
formatwizard-miss-not-the-stores = Aquilo pelo qual ele reconhece os arquivos da loja não está neste arquivo. Altere Reconhecer por.
formatwizard-miss-no-title = Ele não encontra nenhum título neste arquivo. Escolha o título.
formatwizard-miss-short-name = não é o nome dele
formatwizard-miss-short-not-the-stores = não é da loja
formatwizard-miss-short-no-title = sem título
formatwizard-saved-packs =
    { $file } salvo. { $packs ->
        [one] { $packs } pacote está sendo lido de novo.
       *[other] { $packs } pacotes estão sendo lidos de novo.
    }
formatwizard-saved = { $file } salvo.
formatwizard-save-failed = Não foi possível salvar o formato: { $error }
formatwizard-zip-failed = Não foi possível criar o zip: { $error }
formatwizard-stop-title = Parar de ensinar este arquivo?
formatwizard-stop-body = O formato não é salvo.
formatwizard-stop = Parar
formatwizard-label = Assistente de formato
formatwizard-changing = Alterando { $name }
formatwizard-starting-from = Partindo de { $name }
formatwizard-title = Ensine ao Hoard o arquivo de uma loja
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }em{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Editar como TOML
formatwizard-file-reference = O arquivo, para referência.
formatwizard-click-json = Clique onde fica { $field } no arquivo.
formatwizard-click-text = Clique onde fica { $field } no arquivo, ou selecione o texto.
formatwizard-pick-first = Escolha um campo à direita e depois clique no valor dele aqui.
formatwizard-loading = Carregando…
formatwizard-the-format = O formato
formatwizard-syntax-before = A sintaxe está em{ " " }
formatwizard-syntax-after = .
formatwizard-store = Loja
formatwizard-store-placeholder = O nome da loja, como Kenney
formatwizard-files = Arquivos
formatwizard-recognize = Reconhecer por
formatwizard-recognize-json = Uma chave que só os arquivos da loja têm
formatwizard-recognize-text = Uma linha que só os arquivos da loja têm
formatwizard-recognize-hint = Uma forma por linha; qualquer uma serve. Necessário quando outros arquivos têm o mesmo nome.
formatwizard-seller = Vendedor
formatwizard-seller-placeholder = Quando o arquivo não diz
formatwizard-tags = Tags
formatwizard-tags-placeholder = Para tudo o que ele descreve, separadas por vírgula
formatwizard-fields = Campos
formatwizard-selector = Seletor: { $field }
formatwizard-clear = Limpar: { $field }
formatwizard-pick = Escolher
formatwizard-what-it-reads = O que ele lê
formatwizard-read-title = Título
formatwizard-read-seller = Vendedor
formatwizard-read-license = Licença
formatwizard-read-url = Página da loja
formatwizard-read-kind = Tipo
formatwizard-read-tags = Tags
formatwizard-read-origin = Feito em
formatwizard-read-size = Tamanho
formatwizard-also = { $format } também lê este arquivo. Este formato seria tentado primeiro.
formatwizard-library =
    { $files ->
        [one] { $files } arquivo
       *[other] { $files } arquivos
    } em { $packs ->
        [one] { $packs } pacote
       *[other] { $packs } pacotes
    } { $files ->
        [one] tem
       *[other] têm
    } um nome que ele aceita. Dos { $tried } testados, ele lê { $read }.
formatwizard-library-none = Nenhum arquivo da biblioteca tem um nome que ele aceita.
formatwizard-share = Compartilhe
formatwizard-share-hint = Um zip do formato, com este arquivo como amostra e o que a amostra deve resultar, organizado como o repositório do Hoard, para enviar e todos o receberem. Antes, retire da amostra qualquer coisa que não deva ser compartilhada.
formatwizard-sample = Amostra a compartilhar
formatwizard-download = Baixar { $file }
formatwizard-close = Fechar
formatwizard-use = Usar aqui
formatwizard-cancel = Cancelar
