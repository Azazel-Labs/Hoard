## Erros do servidor

error-account-not-found = conta não encontrada
error-asset-gone = recurso não encontrado
error-asset-not-found = recurso { $id } não encontrado
error-blob-not-found = blob { $hash } não encontrado
error-collection-not-found = coleção { $id } não encontrada
error-connection-not-found = ligação não encontrada
error-derivative-not-found = derivado { $kind } de { $id } não encontrado
error-file-in-pack-not-found = { $path } não encontrado no pacote { $id }
error-site-not-a-page = só as páginas web abrem como páginas
error-site-link-expired = o endereço desta página expirou; abra a página novamente a partir do Hoard
error-format-not-found = formato { $file } não encontrado
error-frame-sheet-aseprite-only = só os ficheiros do Aseprite têm folha de fotogramas
error-frame-sheet-frames = isto tem um só fotograma, ou demasiados para dispor
error-location-not-found = local { $id } não encontrado
error-maintenance-task-not-found = tarefa de manutenção { $id } não encontrada
error-open-upload-not-found = carregamento aberto { $id } não encontrado
error-original-not-found = ficheiro original de { $id } não encontrado
error-pack-not-found = pacote { $id } não encontrado
error-preview-too-big = esta imagem é demasiado grande para fazer uma pré-visualização leve
error-project-not-found = projeto { $name } não encontrado
error-saved-search-not-found = pesquisa guardada { $id } não encontrada
error-sign-in-first = inicie sessão primeiro (ou envie um token de acesso)
error-those-assets-not-found = esses recursos não foram encontrados
error-thumbnail-not-found = miniatura de { $id } não encontrada
error-token-not-found = token não encontrado
error-upload-not-found = carregamento { $id } não encontrado
error-pack-needs-name = um pacote precisa de um nome
error-no-library = ainda não há nenhum local da biblioteca onde guardar ficheiros: adicione um gerido em Definições → Locais da biblioteca
error-upload-failed = o carregamento falhou: { $reason }
error-upload-no-files = não há ficheiros no carregamento
error-analyze-nothing-to-do = nada a fazer: rules=false e o LLM está desligado
error-too-many-assets = demasiados recursos num só pedido
error-too-many-items = demasiados itens num só pedido
error-not-a-hash = não é um hash blake3 em hexadecimal
error-merge-into-tag = into tem de ser uma etiqueta
error-nothing-to-merge = nada para fundir
error-selection-size = indique entre 1 e { $max } IDs de recursos
error-collection-needs-name = uma coleção precisa de um nome
error-collection-exists = já existe uma coleção chamada { $name }
error-collection-empty = a coleção { $name } está vazia
error-conversion-failed = não foi possível converter para { $format }: { $reason }
error-max-size = max tem de ser uma potência de dois entre 64 e 16384
error-replace-ext = ext tem de ser uma extensão de ficheiro curta
error-empty-file = ficheiro vazio
error-imitation-silent = Nada na gravação se destaca do fundo. Tente novamente mais perto do microfone.
error-replace-duplicate = o ficheiro convertido duplica um que já está no pacote; o pacote foi fundido nele
error-ai-address = O endereço da IA deve começar por http:// ou https://
error-hosted-key = Introduza a chave de API de { $provider }
error-hosted-address = O endereço do serviço de pesquisa de imagens deve começar por http:// ou https://
error-local-llm-action = action é list, download ou delete, e as duas últimas precisam de um modelo
error-role = Uma função é view, edit ou admin
error-admin-only = só um administrador pode fazer isto
error-view-only = esta conta pode ver mas não alterar nada: um administrador pode mudar isso em Definições → Conta
error-portal-only = inicie sessão no portal para gerir contas e tokens
error-not-from-portal = este pedido não veio do portal do Hoard
error-account-name = Escolha um nome com até 64 caracteres
error-password-length = Use uma palavra-passe com pelo menos { $min } caracteres
error-too-many-codes = Demasiados códigos errados: aguarde { $seconds } segundos
error-already-set-up = Já existe uma conta: inicie sessão
error-too-many-tries = Demasiadas tentativas erradas: aguarde { $seconds } segundos
error-directory-unavailable = O diretório não conseguiu verificar a sua palavra-passe: { $reason }
error-password-elsewhere = A sua palavra-passe é guardada pelo diretório ou fornecedor com que inicia sessão
error-current-password = A sua palavra-passe atual não está correta
error-account-source = Uma conta inicia sessão com o Hoard (internal), um diretório (ldap) ou OAuth
error-account-exists = Já existe uma conta com esse nome
error-own-role = Só outro administrador pode alterar a sua função
error-remove-self = Não pode remover a sua própria conta enquanto tem sessão iniciada nela
error-token-name = Dê ao token o nome daquilo que o vai usar, com até 80 caracteres
error-token-role = Um token não pode fazer mais do que a conta que o cria
error-wrong-setup-code = Esse não é o código de configuração dado ao servidor em HOARD_SETUP_CODE
error-wrong-password = Esse nome e essa palavra-passe não correspondem
error-folder-missing = { $path } não existe (um disco que não está montado?)
error-s3-path = um local S3 tem o formato s3://bucket
error-s3-address = o endereço do S3 tem de começar por http:// ou https://
error-s3-header = um cabeçalho S3 extra tem o formato Name=value, e «{ $header }» não tem
error-keychain = não foi possível guardá-lo no porta-chaves do sistema: { $reason }
error-not-built = esta cópia do Hoard foi compilada sem { $feature }
error-openai-auth-unreachable = não foi possível contactar auth.openai.com
error-openai-unexpected = resposta inesperada da OpenAI
error-openai-answered = A OpenAI respondeu { $status }: { $reason }
error-chatgpt-id-token-not-jwt = o token de ID não é um JWT
error-chatgpt-id-token-issuer = o token de ID é de { $issuer }, não de { $expected }
error-chatgpt-id-token-audience = o token de ID é para outra aplicação
error-chatgpt-id-token-expired = o token de ID expirou
error-chatgpt-id-token-nonce = o token de ID não pertence a este início de sessão
error-chatgpt-no-state = o endereço não tem state; copie o endereço completo
error-chatgpt-not-started = este início de sessão não foi começado aqui, ou foi começado há mais de 15 minutos. Comece-o de novo a partir das Definições.
error-chatgpt-canceled = o início de sessão foi cancelado
error-chatgpt-refused = O ChatGPT recusou o início de sessão: { $reason }
error-chatgpt-no-code = o endereço não tem código de início de sessão; copie o endereço completo
error-chatgpt-other-registration = O ChatGPT respondeu para outro registo deste servidor; comece de novo o início de sessão
error-chatgpt-other-registration-desktop = O ChatGPT respondeu para outra cópia do Hoard; comece de novo o início de sessão
error-chatgpt-not-registered = O ChatGPT não terminou o registo do Hoard; comece de novo o início de sessão
error-chatgpt-code-used = o código de início de sessão já foi usado ou expirou; inicie sessão de novo
error-chatgpt-refused-code = O ChatGPT recusou o início de sessão ({ $code })
error-chatgpt-no-id-token = O ChatGPT não enviou nenhum token de ID
error-chatgpt-plan-not-allowed = Sessão iniciada, mas o uso do plano não foi permitido. Inicie sessão de novo e permita-o, ou use antes uma chave de API.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = sem sessão iniciada no ChatGPT; inicie sessão em Definições → Etiquetagem por IA
error-chatgpt-refresh-later = O ChatGPT não conseguiu renovar a sessão agora ({ $code }); será tentado de novo mais tarde
error-chatgpt-signed-out-problem = O ChatGPT terminou a sessão deste servidor ({ $code }). Inicie sessão de novo para continuar a etiquetar com o seu plano.
error-chatgpt-signed-out-problem-desktop = O ChatGPT terminou a sessão do Hoard ({ $code }). Inicie sessão de novo para continuar a etiquetar com o seu plano.
error-chatgpt-signed-out = O ChatGPT terminou a sessão deste servidor ({ $code }); inicie sessão de novo em Definições → Etiquetagem por IA
error-chatgpt-signed-out-desktop = O ChatGPT terminou a sessão do Hoard ({ $code }); inicie sessão de novo em Definições → Etiquetagem por IA
error-openai-api-unreachable = não foi possível contactar api.openai.com
error-openai-broke-off = a resposta da OpenAI foi interrompida
error-chatgpt-kept-refusing = A OpenAI continuou a recusar o início de sessão no ChatGPT; inicie sessão de novo em Definições → Etiquetagem por IA
error-chatgpt-request-failed = o pedido falhou
error-chatgpt-cut-off = a resposta foi cortada ({ $reason })
error-chatgpt-cut-off-no-reason = a resposta foi cortada (sem motivo indicado)
error-openai-ended-early = a resposta da OpenAI terminou antes de estar completa
error-chatgpt-usage-limit = o uso do seu plano ChatGPT para o Hoard esgotou-se por agora. Veja ou altere o limite em { $url }
error-chatgpt-not-eligible = esta conta ou área de trabalho do ChatGPT não pode usar o seu plano noutras aplicações. Use antes uma chave de API.
error-chatgpt-usage-unavailable = O ChatGPT não conseguiu verificar o uso do plano agora; será tentado de novo mais tarde
error-chatgpt-unsupported = o uso do plano ChatGPT não suporta { $param } com este modelo
error-chatgpt-unsupported-images = o uso do plano ChatGPT não suporta { $param } com este modelo; escolha um modelo que aceite imagens, ou desligue o envio de miniaturas
error-chatgpt-unsupported-part = o uso do plano ChatGPT não suporta parte do pedido com este modelo
error-chatgpt-port = port é a porta pela qual o navegador chega ao Hoard
error-chatgpt-paste-address = cole o endereço completo, começado por http://127.0.0.1
error-too-many-hashes = no máximo { $max } hashes de cada vez
error-credits-pack = pacote { $key }
error-credits-collection = coleção { $key }
error-format-too-long = o formato é demasiado longo
error-not-started = o servidor não arrancou
error-not-started-desktop = O Hoard não arrancou
error-not-format-file = não é um ficheiro de formato: { $name }
error-format-name = o nome precisa de uma letra ou dígito de A a Z ou de 0 a 9
error-format-miss-name = os ficheiros do formato não incluem { $file }
error-format-miss-store = o formato não aceita { $file } como ficheiro da loja (ver require)
error-format-miss-title = o formato não encontra nenhum título em { $file }
error-cant-download-as = os ficheiros .{ $ext } não podem ser transferidos como { $format }
error-only-images-resize = só as imagens podem ser redimensionadas
error-https-no-certificate = Cole ou carregue primeiro o seu certificado e a respetiva chave
error-https-no-cert-found = Nenhum certificado encontrado: cole o texto que começa por -----BEGIN CERTIFICATE-----
error-https-no-key-found = Nenhuma chave privada encontrada: cole o texto que começa por -----BEGIN PRIVATE KEY-----
error-https-several-keys = Há mais do que uma chave privada: cole só a que corresponde ao certificado
error-https-key-password = A chave está protegida por palavra-passe: forneça-a sem palavra-passe (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = O certificado não é PEM válido
error-https-key-not-pem = A chave privada não é PEM válida
error-https-key-mismatch = A chave não pertence ao certificado. Se forneceu uma cadeia, ponha primeiro o certificado do seu servidor
error-https-unusable = O certificado ou a chave não podem ser usados
error-https-expired = Este certificado expirou a { $date }
error-https-not-yet-valid = Este certificado só é válido a partir de { $date }
error-https-unreadable = Isso não é um certificado que o Hoard consiga ler
error-https-bad-name = «{ $name }» não é um endereço que um certificado possa conter
error-https-test-timed-out = a ligação de teste excedeu o tempo limite
error-https-test-failed = a ligação de teste falhou
error-https-no-ca = O Hoard ainda não criou a sua autoridade de certificação
error-https-unavailable = O HTTPS não está disponível neste servidor
error-https-certificate-mode = Um certificado vai com «O meu certificado»
error-https-test = O certificado não funcionou numa ligação de teste: { $reason }
error-nothing-to-retry = não há nada aí que possa ser tentado de novo
error-ai-unreachable = não foi possível contactar { $url }
error-ai-key-refused = { $url } recusou a chave de API ({ $status })
error-ai-answered = { $url } respondeu { $status }: { $reason }
error-ai-not-json = { $url }/models não respondeu com JSON
error-model-server-refused = o servidor de modelos: { $reason }
error-no-model-server = nenhum servidor de modelos responde em { $url }
error-not-llama-router = { $url } não é um servidor llama.cpp em modo router, por isso não pode transferir modelos
error-model-download = não foi possível transferir { $model }: { $reason }
error-model-delete = não foi possível eliminar { $model }: { $reason }
error-unknown-kind = tipo de recurso desconhecido { $kind }
error-watched-is-folder = um local vigiado é uma pasta neste servidor
error-cant-reach = não é possível aceder a { $path }: { $reason }
error-folder-full-path = indique o caminho completo da pasta
error-cant-create = não é possível criar { $path }
error-cant-write = não é possível escrever em { $path }
error-cant-read = não é possível ler { $path }
error-overlaps-location = isso sobrepõe-se ao local { $name } ({ $path })
error-overlaps-data = isso sobrepõe-se à pasta de dados do Hoard ({ $path })
error-overlaps-cache = isso sobrepõe-se à pasta de cache do Hoard ({ $path })
error-not-watched = { $name } não é uma pasta vigiada
error-watched-no-kinds = uma pasta vigiada não recebe ficheiros novos, por isso não tem tipos para alterar
error-keep-catch-all = em vez disso, torne outro local o local para tudo o resto
error-location-holds-files = { $name } ainda contém ficheiros: mova-os primeiro para os outros locais (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = não é possível aceder a { $name }, por isso o Hoard não consegue saber se ainda contém ficheiros
error-relocate-running = já há ficheiros a ser movidos entre locais; tente de novo quando isso terminar
error-watched-not-moved = { $name } é uma pasta vigiada: o Hoard não move os ficheiros dela
error-no-other-location = não há outro local da biblioteca para onde mover os ficheiros: adicione um primeiro
error-full-path = indique um caminho completo
error-pick-folder = escolha uma pasta
error-task-running = «{ $task }» já está em execução
error-token-account-gone = a conta deste token já não existe, por isso não tem favoritos nem pesquisas
error-favorites-too-many = no máximo 10 000 de cada vez
error-saved-search-name = uma pesquisa guardada precisa de um nome com no máximo 80 caracteres
error-saved-search-too-long = essa pesquisa é demasiado longa para ser guardada
error-hoard-seen = o tesouro pode registar no máximo 32 tipos de recurso, cada um com nome em letras minúsculas
error-texture-pattern-map = O padrão de nome das texturas precisa de { "{" }map{ "}" }, senão todos os mapas de um material ficariam com o mesmo nome
error-texture-pattern-fields = O padrão de nome das texturas pode usar { "{" }name{ "}" }, { "{" }map{ "}" } e { "{" }size{ "}" }
error-texture-name-chars = Os nomes das texturas não podem ter < > : " / \ | ? nem *
error-texture-map-type = Não existe nenhum tipo de mapa chamado { $role }
server-role-base-color = cor
server-role-normal = normais
server-role-normal-dx = normais (DirectX)
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
server-role-specular = especularidade
server-role-opacity = opacidade
server-role-emissive = emissão
server-role-translucency = translucidez
error-texture-suffix-missing = Dê um sufixo aos mapas de { $map }
error-texture-suffix-chars = O sufixo dos mapas de { $map } não pode ter < > : " / \ | ? nem *
error-texture-ending-missing = Cada terminação personalizada precisa da terminação a procurar
error-texture-ending-chars = As terminações personalizadas não podem ter < > : " / \ | ? nem *
error-texture-ending-twice = { $ending } aparece duas vezes nas suas terminações
error-oauth-no-client-id = Este link não tem client_id.
error-oauth-unknown-app = Esta aplicação não está (já) registada aqui. Comece de novo a ligação a partir da aplicação.
error-oauth-redirect = A aplicação pediu para ser reencaminhada para um endereço que não registou.
error-oauth-response-type = O Hoard só entrega códigos de início de sessão (response_type=code).
error-oauth-pkce = A aplicação tem de usar PKCE com S256 para iniciar sessão no Hoard.
error-light-preview-images = só as imagens têm pré-visualização leve
error-no-light-preview = sem pré-visualização leve: { $reason }
error-no-frame-sheet = sem folha de fotogramas: { $reason }
error-thumbnail-source = origem de miniatura desconhecida: { $source }
error-thumbnail-too-big = essa imagem é demasiado grande para uma miniatura
error-thumbnail-models-only = só os modelos aceitam uma miniatura desenhada por um navegador
error-not-a-picture = não é uma imagem que o Hoard consiga ler: { $reason }
error-project-name = um projeto precisa de um nome com no máximo 200 caracteres
error-project-too-big = no máximo um milhão de recursos num projeto
error-project-too-many-at-once = no máximo 100 000 de cada vez
error-server-name = Escolha um nome com até { $max } caracteres, numa só linha
error-tool-setting = { $tool }: escolha desligado, automático, ou indique um caminho
error-ai-model = Escolha o modelo que o serviço de IA deve usar
error-public-address = O endereço público deve começar por http:// ou https://
error-update-hook-address = O endereço a chamar para atualizações deve começar por http:// ou https://
error-update-hook-auth = O cabeçalho Authorization não pode conter quebras de linha nem outros caracteres de controlo
error-no-soundfont = Não existe nenhum SoundFont em { $path } no servidor
error-no-soundfont-desktop = Não existe nenhum SoundFont em { $path }
error-soundfont-bad = { $name } não é um SoundFont com que o MIDI possa tocar: { $reason }
error-workers = Processos em segundo plano: entre 1 e { $max }
error-ai-batch = Recursos por pedido à IA: entre 1 e 200
error-conversion-cache = Transferências convertidas: pelo menos 100 MB
error-conversion-keep-days = Regras de conversão: manter os originais de 1 a { $max } dias
error-conversion-target = Regras de conversão: o Hoard não consegue converter ficheiros para { $format }
error-conversion-no-source = Regras de conversão: cada regra precisa de pelo menos um formato a converter
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Regras de conversão: os ficheiros .{ $ext } não podem ser convertidos para { $format }
error-conversion-twice = Regras de conversão: .{ $ext } está em duas regras
error-conversion-cant-undo = Este ficheiro não foi convertido por uma regra, ou o seu original já foi eliminado.
error-conversion-path-taken = Agora está outro ficheiro em { $path }, por isso o original não pode voltar para lá.
error-limits-zero = Os limites não podem ser zero
error-settings-object = era esperado um objeto JSON de definições
error-bad-setting = definição inválida: { $reason }
error-auth-default-role = A função predefinida é view, edit ou admin
error-auth-session-hours = As sessões duram entre 1 hora e um ano
error-auth-settings = definições de autenticação inválidas: { $reason }
error-ldap-settings = definições de LDAP inválidas: { $reason }
error-ldap-no-server = Adicione o endereço do servidor de diretório
error-ldap-base-dn = Indique onde estão as entradas das pessoas (o DN base)
error-ldap-username-attribute = Indique que atributo contém o nome com que as pessoas iniciam sessão
error-ldap-bind-dn = Indique a conta que procura as pessoas, ou procure-as anonimamente
error-ldap-ca-cert = A autoridade de certificação não é um certificado PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = O endereço de um servidor de diretório está vazio
error-ldap-server-form = { $server }: o endereço de um servidor de diretório é host:porta, ldap://… ou ldaps://…
error-ldap-not-starttls = { $server }: isso é LDAPS, não StartTLS
error-ldap-no-certificate = não contém nenhum certificado
error-ldap-password-again = Indique de novo a palavra-passe da conta que procura as pessoas: a guardada só é enviada para os servidores para os quais foi indicada
error-ldap-unreachable = não foi possível contactar nenhum servidor de diretório ({ $reasons })
error-ldap-no-anonymous = o diretório não permite procurar pessoas anonimamente ({ $reason })
error-ldap-bind-failed = a conta que procura as pessoas não conseguiu iniciar sessão ({ $reason })
error-ldap-search = ao procurar { $name } em { $base }
error-ldap-group-search = ao procurar os grupos de { $name } em { $base }
error-ldap-several = { $count } entradas do diretório correspondem a { $name }: restrinja o filtro de utilizadores
error-ldap-password-check = o diretório não quis verificar a palavra-passe de { $name } ({ $reason })
error-ldap-wrong-password = O diretório não conhece { $name } com essa palavra-passe
error-oauth-client-id = Indique o ID de cliente que o fornecedor criou para o Hoard
error-oauth-addresses = Os endereços de autorização e de token devem começar por https://
error-oauth-userinfo-address = O endereço de informações do utilizador deve começar por https://
error-oauth-username-claim = Indique que claim contém o nome da pessoa
error-oauth-secret-again = Indique de novo o segredo do cliente: o guardado só é enviado para o endereço de token para o qual foi indicado
error-oauth-issuer = O emissor deve começar por https://
error-oauth-discovery-unreachable = Não foi possível contactar { $url }: { $reason }
error-oauth-discovery-status = { $url } respondeu { $status }
error-oauth-discovery-bad = { $url } não é um documento de descoberta
error-oauth-discovery-incomplete = { $url } não indica os endereços de autorização e de token
error-signin-no-group = { $name } não pertence a nenhum grupo que possa usar o Hoard
error-signin-other-source = Já existe uma conta do Hoard chamada { $name } que inicia sessão de outra forma
error-signin-other-subject = Já existe uma conta do Hoard chamada { $name } para outra pessoa no fornecedor
error-signin-no-account = { $name } ainda não tem conta no Hoard: um administrador pode adicioná-la em Definições → Conta
error-signin-bad-name = Esse nome não pode ser uma conta do Hoard (até 64 caracteres)
error-oauth-off = O início de sessão com OAuth não está ativado
error-oauth-bad-authorization-address = o endereço de autorização não é válido
error-oauth-refused = O fornecedor não iniciou a sua sessão: { $reason }
error-oauth-no-code = O fornecedor reencaminhou-o de volta sem início de sessão
error-oauth-expired = Esse início de sessão demorou demasiado ou foi começado noutro sítio: tente de novo
error-oauth-no-claim = O fornecedor não disse quem é (sem a claim { $claim })
error-oauth-no-sub = O fornecedor não disse quem é (sem a claim sub nem id)
error-oauth-token-unreachable = não foi possível contactar o endereço de token do fornecedor
error-oauth-token-refused = O fornecedor não quis concluir o início de sessão: { $reason }
error-oauth-no-access-token = o fornecedor não enviou nenhum token de acesso
error-oauth-userinfo-unreachable = não foi possível contactar o endereço de informações do utilizador do fornecedor
error-oauth-userinfo-refused = O fornecedor não quis dizer quem iniciou sessão ({ $status })
error-oauth-userinfo-not-json = as informações do utilizador do fornecedor não são JSON
error-format-no-name = name está vazio
error-format-no-files = files está vazio: indique que ficheiros o formato lê, como ["*.json"]
error-format-empty-selector = vazio
error-format-unopened-bracket = um parêntese reto de fecho sem o de abertura
error-format-unclosed-bracket = um parêntese reto não está fechado
error-format-empty-step-dots = um passo vazio (dois pontos seguidos, ou um ponto numa das pontas)
error-format-unclosed-except = !( não está fechado
error-format-unclosed-filter = [ não está fechado
error-format-after-filters = { $rest } depois da chave e dos filtros de { $step }
error-format-except-needs-glob = !(…) vem depois de uma chave que tenha *
error-format-except-not-glob = !(…) vem depois de uma chave com *, não de { $key }
error-format-empty-step = um passo vazio
error-format-not-a-filter = [{ $filter }] não é um filtro: use [], [key=value|other], [array], [string], [number], [bool] ou [object]
error-format-filter-needs-value = [{ $filter }] precisa de uma chave e de um valor
error-store-search-empty = nada para pesquisar; escreva o nome do pacote
error-update-hook-not-set = defina primeiro um endereço a chamar
error-upload-stalled = o carregamento de { $path } parou; envie essa parte de novo
error-upload-busy = { $path } já está a ser escrito
error-upload-incomplete = { $count } ficheiros estão incompletos, p. ex. { $files }
error-upload-no-files-listed = sem ficheiros
error-upload-not-part = { $path } não faz parte do carregamento { $id }, ou o carregamento está fechado
error-bad-path = caminho inválido
error-upload-part-offset = { $offset } não é onde começa uma parte de { $path }
error-upload-interrupted = carregamento interrompido: { $reason }
error-upload-part-long = a parte { $part } de { $path } tem mais de { $len } bytes
error-upload-part-short = parte { $part } de { $path }: recebidos { $got } bytes, esperados { $len }
error-upload-file-long = { $path } é maior do que o declarado
error-upload-file-short = { $path }: recebidos { $got } bytes, esperados { $len }
error-upload-complete = o carregamento já está completo
error-already-imported = já importado como «{ $name }»

## Texto do servidor

server-store-place-readme = README
server-store-place-folders = pastas
server-store-why-linked = Link presente no pacote ({ $place })
server-store-why-same-title = Mesmo título que o pacote ({ $place })
server-store-why-by = De { $author }, indicado no pacote ({ $place })
server-store-why-meshes = Mais ou menos tantas malhas como o pacote ({ $count })
server-store-why-textures = Mais ou menos tantas texturas como o pacote ({ $count })
server-chatgpt-revoke-failed = Sessão terminada aqui, mas não foi possível avisar a OpenAI. Para ter a certeza, desligue o Hoard nas definições do ChatGPT: Settings → Security and login → Sign in with ChatGPT.
server-chatgpt-signed-in-title = Sessão iniciada no ChatGPT
server-chatgpt-signed-in = O Hoard tem sessão iniciada. A etiquetagem por IA já pode usar o seu plano ChatGPT.
server-chatgpt-signed-in-as = O Hoard tem sessão iniciada como { $email }. A etiquetagem por IA já pode usar o seu plano ChatGPT.
server-chatgpt-not-finished-title = Início de sessão por concluir
server-chatgpt-other-computer = Se começou o início de sessão a partir do Hoard noutro computador, copie o endereço desta página da barra de endereço e cole-o nas Definições do Hoard nesse computador.
server-back-to-settings = Voltar às Definições
server-issues-pack = Pacotes que não puderam ser adicionados
server-issues-pack-blurb = Elimine o pacote e adicione-o de novo, ou abra-o para ver o que contém.
server-issues-damaged = Ficheiros guardados danificados ou desaparecidos
server-issues-damaged-blurb = Encontrados por «Verificar danos nos ficheiros». Restaure-os de uma cópia de segurança ou adicione de novo o pacote deles e depois execute «Verificar danos nos ficheiros» outra vez.
server-issues-folder = Pastas vigiadas que não podem ser lidas
server-issues-folder-blurb = Os pacotes delas ficam como estão até a pasta poder ser lida de novo.
server-issues-missing = Pacotes vigiados cuja pasta desapareceu
server-issues-missing-blurb = Cada pacote é mantido durante 3 dias, caso a pasta esteja só offline, e depois é removido.
server-issues-job = Tarefas em segundo plano que falharam
server-issues-job-blurb = Tentar de novo executa a tarefa outra vez desde o início.
server-issues-file = Ficheiros que não puderam ser abertos
server-issues-file-blurb = O Hoard não conseguiu abri-los nem guardá-los. Um pacote vigiado tenta de novo na próxima análise; um ficheiro que já estava na biblioteca manteve o que tinha antes.
server-issues-unpack = Arquivos que não puderam ser descomprimidos
server-issues-unpack-blurb = Cada um fica no seu pacote como um ficheiro simples, por isso nada do que contém está na biblioteca.
server-issues-read = Ficheiros que o Hoard não conseguiu interpretar
server-issues-read-blurb = Estão na biblioteca, mas sem os dados, etiquetas ou miniatura que a leitura lhes daria.
server-issues-render = Modelos em que o renderizador de miniaturas falhou
server-issues-render-blurb = Não têm miniatura. Abrir um no visualizador pode funcionar na mesma, e permite guardar uma.
server-issues-convert = Modelos que o Blender não conseguiu converter
server-issues-convert-blurb = Não têm cópia GLB, por isso não podem ser mostrados em 3D.
server-issues-unreal = Packages do Unreal que não puderam ser convertidos
server-issues-unreal-blurb = A página Unreal do pacote tem os restantes packages.
server-issues-reformat = Ficheiros que uma regra de conversão não conseguiu converter
server-issues-reformat-blurb = Ficam como estavam.
server-issues-materials = Pacotes cujos materiais não puderam ser determinados
server-issues-materials-blurb = As texturas estão todas lá; só não estão agrupadas em materiais.
server-issues-material-thumb = Materiais sem miniatura
server-issues-material-thumb-blurb = O material foi encontrado, mas não foi possível desenhar a pré-visualização dele.
server-issues-store-files = Ficheiros de loja que não puderam ser lidos
server-issues-store-files-blurb = Ficheiros da loja de onde um pacote veio, como um JSON da Megascans, que lhe dão nome e etiquetas.
server-issues-scan-again = Analisar de novo
server-issues-retry = Tentar de novo
server-issues-reanalyze = Reanalisar pacote
server-issues-convert-again = Converter de novo
server-issues-look-again = Procurar de novo
server-issues-read-again = Ler de novo
server-issues-blob-missing = O ficheiro guardado desapareceu.
server-issues-blob-damaged = O ficheiro guardado não corresponde ao que foi guardado: está danificado.
server-issues-folder-gone = A pasta «{ $folder }» não foi encontrada.
server-job-a-pack = um pacote
server-job-ingest = A importar { $pack }
server-job-retag = A etiquetar de novo { $pack }
server-job-llm-tag = A etiquetar { $pack } com IA
server-job-convert = A converter { $pack } com o Blender
server-job-unreal = A converter os packages do Unreal em { $pack }
server-job-reformat = A converter ficheiros em { $pack } segundo as regras de conversão
server-job-scan = A analisar uma pasta vigiada
server-job-relocate = A mover ficheiros entre locais da biblioteca
server-job-looks = A determinar cores e aspeto
server-job-sounds = A ouvir sons
server-job-embed = A aprender o que as imagens mostram
server-job-hear = A aprender como soam os sons
server-job-materials = A procurar materiais em { $pack }
server-job-read-files = A ler os ficheiros de { $pack }
server-job-store-lookup = A consultar uma página de loja
server-maintenance-backup = Fazer cópia de segurança da base de dados
server-maintenance-backup-blurb = Uma cópia consistente da base de dados (pacotes, etiquetas, notas, coleções e Definições) na pasta de cópias de segurança ao lado dela. Os ficheiros da biblioteca não estão incluídos: nunca mudam depois de guardados, por isso qualquer sincronização de ficheiros faz cópia deles.
server-maintenance-scrub = Verificar danos nos ficheiros
server-maintenance-scrub-blurb = Lê de novo cada ficheiro guardado e compara-o com a sua impressão digital, para apanhar ficheiros que se degradaram ou desapareceram de um disco ou NAS. Demora algum tempo numa biblioteca grande.
server-maintenance-reanalyze = Reanalisar todos os pacotes
server-maintenance-reanalyze-blurb = Lê de novo todos os ficheiros com esta versão do Hoard: miniaturas, pré-visualizações, detalhes e etiquetas das regras são refeitos. As suas edições mantêm-se; a IA não é consultada de novo.
server-maintenance-clear-cache = Limpar conversões em cache
server-maintenance-clear-cache-blurb = Descarta os ficheiros guardados das conversões «Transferir como» (são refeitos quando voltarem a ser pedidos) e o que o trabalho interrompido deixou para trás.
server-maintenance-shrink-thumbs = Reduzir miniaturas
server-maintenance-shrink-thumbs-blurb = Guarda as miniaturas e vistas de modelos mantidas em PNG como pede Definições → Desempenho → Miniaturas, o que normalmente as torna várias vezes mais pequenas. As que têm transparência, e a pixel art, ficam em PNG.
server-maintenance-gc = Eliminar ficheiros não usados
server-maintenance-gc-blurb = Elimina os ficheiros guardados para os quais já nada aponta, como os ficheiros de pacotes eliminados definitivamente.
server-update-no-address = não está definido nenhum endereço a chamar
server-update-no-answer = sem resposta em 30 segundos

## Definições: Aspeto

appearance-language = Idioma
appearance-language-auto = O do navegador ({ $name })

## Raiz

root-unreachable = Não foi possível contactar o servidor do Hoard: { $error }
root-unreachable-desktop = O Hoard não está a responder: { $error }
root-desktop-reopen = Esta janela terminou a sessão. Feche o Hoard e volte a abri-lo para continuar.
root-try-again = Tentar de novo
root-lazy-connect = a ligação da aplicação
window-minimize = Minimizar
window-maximize = Maximizar
window-restore = Restaurar
window-close = Fechar

## Aplicação: a biblioteca, a barra superior e os filtros

app-lazy-settings = as definições
app-lazy-archive = o arquivo
app-lazy-shortcuts = os atalhos
app-lazy-setup = a configuração
app-theme-system = Seguir o tema do sistema
app-theme-light = Tema claro
app-theme-dark = Tema escuro
app-color-red = Vermelho
app-color-orange = Laranja
app-color-yellow = Amarelo
app-color-green = Verde
app-color-teal = Azul-petróleo
app-color-blue = Azul
app-color-purple = Roxo
app-color-pink = Cor-de-rosa
app-color-brown = Castanho
app-color-sand = Areia
app-color-gray = Cinzento
app-color-slate = Ardósia
app-color-black = Preto
app-color-white = Branco
app-color-swatch = { $color }: recursos com muito { $lower }
app-color-any = Qualquer cor
app-color-pick = Escolher qualquer cor
app-color = Cor
app-music = Música
app-tempo = BPM
app-tempo-min = de
app-tempo-max = até
app-tempo-min-title = Música neste andamento ou mais rápida, em batidas por minuto (metade ou o dobro também contam)
app-tempo-max-title = Música neste andamento ou mais lenta, em batidas por minuto
app-key = Tonalidade
app-key-any = Qualquer tonalidade
app-key-major = Maior
app-key-minor = Menor
app-key-mix = E tonalidades que combinam com ela
app-key-mix-title = A sua relativa maior ou menor e as tonalidades uma quinta acima e abaixo, que os DJ misturam com ela
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Escreva o barulho que um som faz tal como o soletraria: os parecidos aparecem primeiro
app-imitation-record = Faça o barulho no microfone: os sons parecidos aparecem primeiro
app-imitation-stop = Parar a gravação
app-imitation-heard = O seu barulho
app-imitation-mic-failed = Não foi possível gravar a partir do microfone: { $error }
app-retry = Tentar de novo
app-undo = Anular
app-search-failed = A pesquisa falhou: { $error }
app-unreachable = Não foi possível contactar o servidor: { $error }
app-unreachable-desktop = O Hoard não está a responder: { $error }
app-forget-project-title = Esquecer { $name }?
app-forget-project-body = O Hoard deixa de o indicar onde os seus { $count } recursos são usados, até que o próximo hoard sync o volte a informar. Nada muda no projeto.
app-forget-project-confirm = Esquecer
app-forget-project-failed = Não foi possível esquecer { $name }: { $error }
app-forget-project-label = Esquecer o projeto { $name }
app-forget-project = Esquecer este projeto
app-save-name-like = semelhante a { $name }
app-save-name-favorites = favoritos
app-search-saved = «{ $name }» guardada: está em Filtros, em Seus
app-rank-alike = Mais semelhantes
app-rank-color = Com mais desta cor
app-rank-match = Melhor correspondência
app-rank-imitation = Os mais parecidos
app-sort = Ordenar
app-sort-newest = Mais recentes
app-sort-name = Nome
app-sort-largest = Maiores
app-sort-oldest = Mais antigos
app-collection-added =
    { $count ->
        [one] { $count } recurso adicionado
       *[other] { $count } recursos adicionados
    } a { $name }
app-collection-add-failed = Não foi possível adicionar à coleção: { $error }
app-collection-removed = { $count } removidos de { $name }
app-collection-remove-failed = Não foi possível remover da coleção: { $error }
app-ns-subject = Assunto
app-ns-style = Estilo
app-ns-theme = Tema
app-ns-material = Material
app-ns-map = Mapa PBR
app-ns-color = Cor
app-ns-mood = Ambiente
app-ns-genre = Género
app-ns-license = Licença
app-ns-res = Resolução
app-ns-duration = Duração
app-ns-poly = Número de polígonos
app-ns-format = Formato
app-ns-weight = Peso
app-ns-tech = Técnico
app-ns-blender = Versão do Blender
app-ns-depth = Profundidade de bits
app-ns-ucs = Categoria de som (UCS)
app-ns-bpm = Andamento
app-ns-root = Nota fundamental
app-ns-level = Intensidade sonora
app-ns-ue = Classe do Unreal
app-ns-engine = Motor
app-ns-extracted = Original recuperado
app-ns-script = Sistema de escrita
app-bulk-failed =
    Não foi possível alterar { $count ->
        [one] esse recurso
       *[other] esses { $count } recursos
    }: { $error }
app-bulk-untagged =
    { $tag } retirada de { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
app-bulk-tagged =
    { $count ->
        [one] { $count } recurso etiquetado
       *[other] { $count } recursos etiquetados
    } com { $tag }
app-bulk-category-set =
    Categoria de { $count ->
        [one] { $count } recurso definida
       *[other] { $count } recursos definida
    } como { $category }
app-bulk-category-cleared =
    Categoria de { $count ->
        [one] { $count } recurso limpa
       *[other] { $count } recursos limpa
    }
app-credits-failed = Não foi possível criar os créditos: { $error }
app-archived-assets =
    { $count ->
        [one] { $count } recurso arquivado
       *[other] { $count } recursos arquivados
    }
app-archived-one = { $name } arquivado
app-open-failed = Não foi possível abrir esse recurso: { $error }
app-unnamed-pack = pacote
app-unnamed-collection = coleção
app-unnamed-asset = recurso
app-home-version = Hoard { $version } — voltar à biblioteca
app-home = Voltar à biblioteca
app-search-label = Pesquisar recursos
app-search-placeholder = Pesquisar nomes, caminhos, etiquetas
app-views = Vistas
app-library = Biblioteca
app-packs = Pacotes
app-packs-working = { $count } em curso
app-update-title = Saiu o Hoard { $version } (este servidor executa o { $current }). Obtenha a nova imagem ou o novo binário e reinicie; a base de dados atualiza-se sozinha.
app-update = Atualizar
app-theme = Tema
app-sign-in = Iniciar sessão
app-archive-title = Arquivo: { $contents }
app-archive = Arquivo
app-settings = Definições
app-palette-title = Paleta de comandos ({ $keys })
app-import-folder = Importar pasta
app-add-files = Adicionar ficheiros
app-add-files-short = Adicionar
app-split-label = Como importar esta pasta
app-split-before = { "" }
app-split-after = { " " }contém { $count } pastas ou arquivos.
app-split-each = Importar cada um como pacote próprio ({ $count })
app-split-one = Importar como um só pacote
app-cancel = Cancelar
app-filters = Filtros
app-done = Concluído
app-yours = Seus
app-favorites = Favoritos
app-forget-search-label = Esquecer a pesquisa guardada { $name }
app-forget-search = Esquecer esta pesquisa
app-projects-title = Projetos que fazem hoard sync a partir daqui, com o que tinha a última sincronização
app-projects = Projetos
app-project-synced = Sincronizado { $when }
app-project-synced-by = Sincronizado { $when } por { $who }
app-collections = Coleções
app-no-filters = Não há filtros para esta pesquisa.
app-kind = Tipo
app-facets-more = mais { $count }
app-facets-fewer = menos
app-searching = A pesquisar…
app-searching-live = A pesquisar
app-asset-count =
    { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
app-clear-all = Limpar tudo
app-save-name-label = Nome para esta pesquisa
app-save-name-placeholder = Dê-lhe um nome
app-save = Guardar
app-save-search-title = Manter esta pesquisa em Filtros, em Seus
app-save-search = Guardar pesquisa
app-select-title = Escolher vários recursos para etiquetar, categorizar, colecionar, transferir ou arquivar em conjunto (X, ou Ctrl-clique)
app-select = Selecionar
app-tile-size = Tamanho dos mosaicos
app-supporting-title = Buffers glTF, bibliotecas de materiais, os mapas dos materiais, materiais usados por um só modelo e os ficheiros de loja que descrevem modelos. Vêm junto quando transfere o que os usa. As páginas de uma folha de sprites, e os sprites que o pacote também tem como imagens separadas, aparecem com a folha. Um recurso que o pacote tem em vários formatos aparece uma vez, no formato que o Hoard lê melhor.
app-supporting-hidden =
    { $count ->
        [one] { $count } ficheiro de apoio oculto
       *[other] { $count } ficheiros de apoio ocultos
    }
app-supporting-show = Mostrar
app-supporting-shown = A incluir ficheiros de apoio
app-supporting-hide = Ocultar
app-folders = Pastas
app-folders-title = Ver o pacote como estava no disco, uma pasta de cada vez
app-folders-label = Pastas do pacote
app-welcome-title = Largue um pacote de recursos em qualquer parte desta página
app-welcome-before = Funcionam zips, tarballs, pastas ou ficheiros soltos. O Hoard descomprime-os, guarda cada ficheiro uma só vez pelo seu hash, lê tamanhos, durações e números de polígonos, e etiqueta tudo para que possa pesquisar coisas como{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }ou{ " " }
app-welcome-after = .
app-welcome-import-folder = Importar uma pasta
app-welcome-choose-files = Escolher ficheiros
app-none-similar = Ainda nada se parece muito com { $name }.
app-none = Nada corresponde.
app-none-query = Nada corresponde a «{ $query }».
app-none-imitation = Nenhum som medido soa ainda muito como «{ $word }».
app-none-heard = Nenhum som medido soa ainda muito como o seu barulho.
app-none-filters = Nada corresponde com estes filtros.
app-none-query-filters = Nada corresponde a «{ $query }» com estes filtros.
app-clear-search-filters = Limpar pesquisa e filtros
app-loading-more = A carregar mais
app-keys-hint = As setas movem, Enter abre, Espaço reproduz um som, X ou Ctrl-clique seleciona vários (toque prolongado em ecrãs táteis).
app-resize-label = Redimensionar o painel de detalhes
app-resize-title = Arraste para redimensionar · duplo clique para a largura que se ajusta
app-selected-label = Recursos selecionados
app-selected = { $count } selecionados
app-pick-assets = Escolher recursos
app-download-zip = Transferir zip
app-credits-title = CREDITS.md dos pacotes de onde estes vêm: quem os fez, as licenças e as linhas de crédito que pedem
app-credits = Créditos
app-prefer-sources-title = SoundWaves e texturas do Unreal cujo ficheiro original o Hoard recuperou
app-prefer-sources = WAV/PNG recuperado em vez de .uasset ({ $count })
bag-clear = Limpar a seleção
bag-glb-heading = Modelos como GLB
bag-glb-text = Cada modelo como ficheiro .glb, de pé com o Y para cima e virado para a frente, com as texturas como ficheiros separados numa pasta textures partilhada pelos modelos. O resto vai tal como está.
bag-glb-download = Transferir zip de GLB
bag-plain-heading = Ficheiros tal como estão
bag-plain-text = Cada ficheiro tal como foi importado, com o que precisa, numa pasta para cada pacote.
bag-remove = Retirar { $name } da seleção
bag-more = e mais { $count }
app-bulk-tag-label = Uma etiqueta a adicionar ou a retirar dos recursos selecionados
app-bulk-tag-placeholder = Etiqueta
app-bulk-tag-add-title = Adicionar esta etiqueta a todos os recursos selecionados
app-bulk-tag-add = Adicionar
app-bulk-tag-remove-title = Retirar esta etiqueta de todos os recursos selecionados
app-bulk-tag-remove = Remover
app-bulk-category-label = Categoria dos recursos selecionados (vazio limpa-a)
app-bulk-category-placeholder = Categoria
app-bulk-category-set-title = Definir a categoria de todos os recursos selecionados
app-bulk-category-clear-title = Limpar a categoria de todos os recursos selecionados
app-bulk-category-set-button = Definir
app-bulk-category-clear-button = Limpar
app-remove-from-collection = Remover de { $name }
app-unfavorite = Retirar dos favoritos
app-favorite = Adicionar aos favoritos
app-select-all = Selecionar todos os { $count }
app-archive-selected-title = Fora das pesquisas até serem restaurados ou eliminados definitivamente, a partir do Arquivo
app-drop-items = Largue { $count } itens para os adicionar
app-drop = Largue para adicionar ao Hoard
app-drop-hint = Cada ficheiro ou pasta de nível superior torna-se um pacote próprio. Gigabytes não são problema; os ficheiros grandes retomam se a ligação cair.

## Paleta de comandos (CommandPalette.vue, com os comandos de App.vue)

palette-group-go-to = Ir para
palette-group-actions = Ações
palette-group-theme = Tema
palette-group-kinds = Tipos
palette-group-packs = Pacotes
palette-group-collections = Coleções
palette-group-projects = Projetos
palette-group-saved = Pesquisas guardadas
palette-group-assets = Recursos
palette-hint-current = atual
palette-library = Biblioteca
palette-library-keywords = recursos grelha início assets grid home
palette-favorites = Favoritos
palette-favorites-keywords = estrela estrelas gostos marcados starred stars liked
palette-packs = Pacotes
palette-archive = Arquivo
palette-archive-keywords = lixo reciclagem eliminados restaurar trash recycle bin deleted restore
palette-stats = Estatísticas da biblioteca
palette-stats-keywords = estatísticas números tamanho disco espaço contagem statistics size disk space count
palette-copies = Quase cópias
palette-copies-keywords = duplicados iguais parecidos redimensionados convertidos espaço desperdiçado duplicates same similar resized converted wasted space dedupe
palette-search = Pesquisar
palette-add-files = Adicionar ficheiros…
palette-add-files-keywords = carregar importar upload import
palette-import-folder = Importar uma pasta…
palette-import-folder-keywords = carregar pacote upload pack
palette-select-all = Selecionar os { $count } mostrados
palette-select-all-keywords = marcar check
palette-select-several = Selecionar vários…
palette-select-several-keywords = lote vários múltiplos etiquetar arquivar bulk multiple tag archive
palette-archive-selected = Arquivar os { $count } selecionados
palette-archive-selected-keywords = eliminar remover apagar lixo delete remove trash
palette-clear-selection = Limpar a seleção
palette-clear-filters = Limpar filtros
palette-clear-filters-keywords = repor reset
palette-hide-supporting = Ocultar ficheiros de apoio
palette-show-supporting = Mostrar ficheiros de apoio
palette-supporting-keywords = mapas normais rugosidade máscaras normal maps roughness masks
palette-account = Conta e tokens de acesso
palette-account-keywords = palavra-passe sessão utilizador token api password login user
palette-sign-out = Terminar sessão ({ $name })
palette-sign-out-keywords = sair terminar log out logout
palette-sign-in = Iniciar sessão
palette-sign-in-keywords = entrar sessão conta palavra-passe log in login account password
palette-settings = Definições
palette-settings-keywords = blender ffmpeg llm ia etiquetagem unreal administrador ferramentas preferências ai tagging admin tools preferences
palette-shortcuts = Atalhos de teclado
palette-shortcuts-keywords = ajuda teclas help keys
palette-theme-keywords = tema aspeto modo theme appearance mode
palette-theme-variant-keywords = aspeto cor appearance color
palette-light-theme = Tema claro: { $name }
palette-dark-theme = Tema escuro: { $name }
palette-kind-all = Todos os tipos (deixar de mostrar só { $kind })
palette-kind-only = Só { $kind }
palette-title = Paleta de comandos
palette-placeholder = Pesquisar recursos, ir para um pacote, executar um comando…
palette-results = Resultados
palette-searching = A pesquisar…
palette-no-match = Nada corresponde a «{ $query }»
palette-foot-move = mover
palette-foot-run = executar
palette-foot-shortcuts = todos os atalhos

## Atalhos de teclado (ShortcutsDialog.vue)

shortcuts-title = Atalhos de teclado
shortcuts-close = Fechar
shortcuts-anywhere = Em qualquer lado
shortcuts-palette = Paleta de comandos: pesquisar, saltar, executar
shortcuts-search = Pesquisar
shortcuts-this-list = Esta lista
shortcuts-escape = Fechar, limpar a seleção ou fechar os detalhes
shortcuts-library = Biblioteca
shortcuts-move = Mover entre mosaicos (ou J e K)
shortcuts-open = Abrir os detalhes
shortcuts-play-sound = Reproduzir ou parar um som
shortcuts-select = Selecionar ou desmarcar
shortcuts-favorite = Adicionar aos favoritos ou retirar deles
shortcuts-key-click = clique
shortcuts-select-range = Selecionar um intervalo
shortcuts-select-all = Selecionar tudo o que está visível
shortcuts-sound-player = Leitor de som
shortcuts-play-pause = Reproduzir ou pausar
shortcuts-skip = Recuar ou avançar
shortcuts-start-end = Início ou fim
shortcuts-image-inspector = Inspetor de imagens
shortcuts-zoom = Aumentar ou reduzir o zoom
shortcuts-fit = Ajustar à janela
shortcuts-actual-pixels = Píxeis reais
shortcuts-tile = Repetir 3×3 para verificar as junções
shortcuts-background = Fundo seguinte
shortcuts-channel = Mostrar um só canal

## KindChips.vue

kinds-group = Tipos de recursos

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = adicionada por uma pessoa
tag-source-rule = do etiquetador offline
tag-source-meta = medida a partir do ficheiro
tag-source-ai = sugerida pela IA
tag-remove = Remover a etiqueta { $tag }

## ActivityIndicator.vue

activity-label-idle = Atividade em segundo plano: inativa
activity-label-running = Atividade em segundo plano: { $running } em curso
activity-label-queued = Atividade em segundo plano: { $running } em curso, { $queued } em fila
activity-title = Atividade em segundo plano
activity-unreachable = Não é possível contactar o servidor
activity-unreachable-desktop = O Hoard não está a responder
activity-empty = Nada em curso. As importações, a etiquetagem, as conversões e as transferências de zips aparecem aqui enquanto decorrem.
activity-queued = em fila
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } tris
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Reproduzir { $name }
tile-pause = Pausar { $name }
tile-favorite = Adicionar { $name } aos favoritos
tile-in-favorites = Nos seus favoritos
tile-add-favorite = Adicionar aos seus favoritos (F)
tile-more-like = Mais como { $name }
tile-more-like-this = Mais como este
tile-select = Selecionar { $name }

## CollectionPicker.vue

collections-add = Adicionar à coleção
collections-menu = Coleções
collections-find-label = Procurar ou criar uma coleção
collections-find-placeholder = Procure ou dê nome a uma nova coleção
collections-create = Criar
collections-empty = Ainda não há coleções. Escreva um nome para criar uma.

## FolderPicker.vue

folders-list = Pastas
folders-close = Fechar
folders-browse = Procurar…
folders-up-title = Subir uma pasta
folders-up = Subir
folders-this-computer = Este computador
folders-none = Não há pastas aqui.

## LocationForm.vue

locations-mode-question = O que deve o Hoard fazer com esta pasta?
locations-mode-managed = Tomar posse: o Hoard guarda aqui os meus pacotes
locations-mode-managed-hint = Os pacotes que carregar ou importar são copiados para aqui, e o Hoard mantém-nos em ordem. A pasta passa a ser do Hoard, por isso não mexa no conteúdo.
locations-mode-watched = Vigiar: os meus pacotes já estão aqui
locations-mode-watched-hint = Cada pasta lá dentro torna-se um pacote. O Hoard lê os ficheiros onde estão e nunca os altera, move ou elimina, e deteta pastas novas à medida que aparecem.
locations-folder-desktop = Pasta
locations-folder-placeholder = A pasta onde estão os seus pacotes
locations-folder-hint-managed = O Hoard cria-a se não existir. No Docker, é uma pasta que montou no contentor.
locations-folder-hint-managed-desktop = O Hoard cria-a se não existir.
locations-folder-hint-watched = No Docker, monte primeiro a pasta no contentor (só de leitura não faz mal) e depois escolha-a aqui.
locations-folder-hint-watched-desktop = O Hoard apenas a lê: nada nela é alterado.
locations-name = Nome
locations-name-placeholder-managed = Biblioteca
locations-name-placeholder-watched = Os meus pacotes
locations-files = Ficheiros que vão para aqui
locations-catch-all = Também tudo o que nenhum outro local recebe
locations-kinds-hint = Os ficheiros de um pacote novo vão para o local que tem o seu tipo assinalado. O resto vai para o local de tudo o resto.
locations-files-all = Todos, por agora. Adicione mais locais mais tarde nas Definições para enviar alguns tipos (música, por exemplo) para outro disco.
locations-first-watched-note = Carregar ficheiros precisa de uma biblioteca mantida pelo Hoard. Pode adicionar uma mais tarde em Definições → Locais da biblioteca.
locations-checking = A verificar a pasta…
locations-add = Adicionar local
locations-cancel = Cancelar

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = Não foi possível carregar os locais da biblioteca
locations-takes-and-rest = { $kinds } e tudo o resto
locations-takes-rest = Tudo o que os outros não recebem
locations-takes-everything = Tudo
locations-takes-nothing = Nada por agora
locations-save-failed = Não foi possível guardar o local
locations-unwatch-title = Deixar de vigiar { $name }?
locations-unwatch-body =
    { $count ->
        [one] O seu único pacote sai
       *[other] Os seus { $count } pacotes saem
    } do Hoard, com as etiquetas e notas. Os ficheiros na pasta não são tocados.
locations-unwatch-body-empty = Os ficheiros na pasta não são tocados.
locations-unwatch = Deixar de vigiar
locations-remove-title = Remover { $name }?
locations-remove-body = Os ficheiros que tiver são primeiro movidos para os outros locais, por tipo; é removido quando ficar vazio. Nada se perde.
locations-remove-body-only = Se contiver ficheiros, adicione primeiro outro local para onde possam ser movidos.
locations-remove = Remover
locations-removed = { $name } removido
locations-emptying = A mover os ficheiros para fora de { $name }; é removido quando isso terminar
locations-offline-title = Não é possível aceder a { $name }
locations-offline-body = Os ficheiros não podem ser movidos enquanto estiver offline. Remover mesmo assim? Os ficheiros que ainda lá estejam vão aparecer como em falta.
locations-remove-anyway = Remover mesmo assim
locations-remove-failed = Não foi possível remover { $name }
locations-rebalancing = A mover os ficheiros guardados para os locais que recebem o tipo deles
locations-rebalance-failed = Não foi possível mover os ficheiros
locations-scanned-never = ainda não analisado
locations-scanned-now = analisado agora mesmo
locations-scanned-minutes = analisado há { $mins } min
locations-scanned-hours = analisado há { $hours } h
locations-scanned-date = analisado a { $date }
locations-scan-failed = Não foi possível analisar { $name }
locations-watching = A vigiar { $name }
locations-added = { $name } adicionado
locations-loading = A carregar…
locations-managed-title = Bibliotecas mantidas pelo Hoard
locations-watched-title = Pastas vigiadas
locations-managed-hint = Os carregamentos e importações são copiados para aqui. Cada local recebe os tipos que assinala; o local de tudo o resto recebe o restante.
locations-watched-hint = Pastas de pacotes indexadas onde estão. O Hoard nunca altera os ficheiros nelas, e procura pastas novas e removidas a cada 15 minutos.
locations-managed-empty = Ainda nenhuma, por isso não há onde pôr um carregamento. Adicione uma abaixo.
locations-reachable = Acessível
locations-unreachable-now = Inacessível neste momento
locations-catch-all-tag = tudo o resto
locations-packs =
    { $count ->
        [one] { $count } pacote
       *[other] { $count } pacotes
    }
locations-unreachable = inacessível
locations-scanning = A analisar…
locations-scan = Analisar agora
locations-edit = Editar
locations-catch-all-locked = Para alterar isto, torne outro local o local de tudo o resto
locations-edit-hint-before = Aplica-se aos ficheiros adicionados a partir de agora. Para mover o que já está guardado, use
locations-edit-hint-after = abaixo.
locations-save = Guardar
locations-rebalance = Mover os ficheiros guardados em conformidade
locations-rebalance-hint = Depois de alterar o que um local recebe: os ficheiros já guardados passam para o local para onde o tipo deles vai agora.
locations-add-title = Adicionar um local
locations-add-hint = Outro disco para alguns tipos de ficheiros, ou uma pasta de pacotes que já tem.
locations-kind-question = Que tipo de local é?
locations-kind-folder = Uma pasta
locations-kind-folder-hint = Num disco ou partilha de NAS montado no contentor, ou uma pasta que o Dropbox, Google Drive, OneDrive ou iCloud Drive mantém sincronizada. O Hoard pode guardar ficheiros lá ou vigiar pacotes que já tem.
locations-kind-folder-hint-desktop = Neste computador, noutro disco ou numa partilha de NAS, ou uma pasta que o Dropbox, Google Drive, OneDrive ou iCloud Drive mantém sincronizada. O Hoard pode guardar ficheiros lá ou vigiar pacotes que já tem.
locations-kind-s3-hint = Um bucket no Amazon S3 ou num serviço semelhante: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. O Hoard guarda ficheiros lá.
locations-s3-title = Armazenamento S3
locations-s3-hint = Um local em s3://bucket-name é um bucket neste serviço: a AWS, ou qualquer serviço compatível com S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage num NAS).
locations-bucket = Bucket
locations-bucket-hint = O nome de um bucket que já existe. As chaves acima têm de conseguir listá-lo, lê-lo e escrever nele.
locations-s3-endpoint = Endereço
locations-s3-endpoint-placeholder = Vazio para a AWS; p. ex. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Região
locations-s3-access-key = ID da chave de acesso
locations-s3-secret-key = Chave de acesso secreta
locations-s3-headers = Cabeçalhos adicionais
locations-s3-headers-placeholder = Name=value, um por linha (para o Cloudflare Access, por exemplo)
locations-s3-kept = Guardado (escreva para substituir)
locations-s3-secrets = As chaves e os cabeçalhos são guardados com as definições do servidor e nunca mais são mostrados.
locations-s3-secrets-desktop = As chaves e os cabeçalhos são guardados no porta-chaves deste computador e nunca mais são mostrados.
locations-s3-saved = Armazenamento S3 guardado
locations-s3-save-failed = Não foi possível guardar o armazenamento S3
locations-add-more = Adicionar um local…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = Não foi possível carregar os Problemas
issues-started =
    { $count ->
        [one] Recomeçado.
       *[other] { $count } itens recomeçados.
    }
issues-retry-failed = Não foi possível tentar de novo
issues-dismiss-failed = Não foi possível dispensar
issues-dismiss-all-title = Dispensar todos os { $count }?
issues-dismiss-all-body = «{ $title }» fica limpo. O que voltar a correr mal volta a ser listado.
issues-dismiss-all = Dispensar todos
issues-just-now = agora mesmo
issues-minutes-ago = há { $mins } min
issues-hours-ago = há { $hours } h
issues-loading = A carregar…
issues-none = Nada correu mal.
issues-action-all = { $action } (todos)
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
issues-starting = A começar…
issues-dismiss-named = Dispensar { $title }
issues-dismiss = Dispensar
issues-show-fewer = Mostrar menos
issues-show-all = Mostrar todos os { $count }
issues-not-listed = Mais { $count } não estão listados.

## archive.ts

archive-anyway = Arquivar mesmo assim
archive-failed = Não foi possível arquivar: { $error }
archive-undo = Anular
archive-restore-failed = Não foi possível restaurar: { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pacote
       *[other] { $packs } pacotes
    } e { $assets ->
        [one] { $assets } recurso
       *[other] { $assets } recursos
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pacote
       *[other] { $packs } pacotes
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } recurso
       *[other] { $assets } recursos
    }
archive-count-nothing = nada

## ArchiveView.vue

archive-load-failed = Não foi possível carregar o arquivo: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Restaurado: { $what }
archive-purged = Eliminado definitivamente: { $what }
archive-purge-failed = Não foi possível eliminar: { $error }
archive-purge-title = Eliminar { $name } definitivamente?
archive-purge-pack-body =
    { $count ->
        [one] O seu único recurso
       *[other] Os seus { $count } recursos
    } e { $size } de ficheiros também desaparecem, exceto os ficheiros partilhados com outro pacote. Não é possível anular.
archive-purge = Eliminar para sempre
archive-purge-asset-body = O ficheiro é libertado, a não ser que outro recurso o partilhe. Não é possível anular.
archive-empty-title = Esvaziar o arquivo?
archive-empty-body = Isto elimina definitivamente { $what } e liberta os respetivos ficheiros. Não é possível anular.
archive-empty = Esvaziar arquivo
archive-title = Arquivo
archive-intro = Os pacotes e recursos arquivados ficam fora das pesquisas, das contagens e da lista de etiquetas. Restaure-os, ou elimine-os definitivamente para libertar o espaço.
archive-loading = A carregar…
archive-is-empty = O arquivo está vazio
archive-is-empty-hint = Arquive um pacote ou um recurso e ele fica aqui à espera até o restaurar ou eliminar definitivamente.
archive-packs = Pacotes
archive-assets = Recursos
archive-asset-count =
    { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
archive-archived = arquivado { $when }
archive-from = de { $pack }
archive-restore = Restaurar
archive-show-more = Mostrar mais (faltam { $count })

## mine.ts

projects-favorite-add-failed = Não foi possível adicionar aos seus favoritos: { $error }
projects-favorite-remove-failed = Não foi possível retirar dos seus favoritos: { $error }
projects-save-search-failed = Não foi possível guardar a pesquisa: { $error }
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
projects-usage-body-one = Usado em { $list }: o próximo hoard sync retira-os.
projects-usage-body = Usado em { $list }: o próximo hoard sync de cada um retira-os.

## uploader.ts

upload-network-error = erro de rede
upload-gave-up = { $what }: { $error } após { $attempts } tentativas
upload-failed-plain = falhou
upload-already-imported = já importado
upload-part-failed = o carregamento falhou ({ $status })
upload-batch = lote de { $count } ficheiros
upload-batch-failed = o lote falhou ({ $status })
upload-finish = concluir o carregamento

## UploadQueue.vue

upload-eta-seconds = faltam { $s } s
upload-eta-minutes = faltam { $min } min
upload-eta-hours = faltam { $h } h
upload-queue = Importações
upload-importing = A importar { $count } pacotes
upload-bytes-of = { $sent } de { $total }
upload-rate = { $size }/s
upload-waiting = À espera
upload-unpacking = Carregado, agora a descomprimir
upload-already = Já está na biblioteca
upload-stopped = Parado
upload-files-of = { $done } de { $total } ficheiros
upload-resumed = retomado, { $size } já lá estavam
upload-skipped = Estes mesmos ficheiros já tinham sido importados como «{ $name }», por isso foram ignorados.
upload-error = { $error }. O seu progresso fica guardado: tente de novo, ou largue os mesmos ficheiros outra vez mais tarde.
upload-retry = Tentar de novo
upload-import-again = Importar de novo mesmo assim
upload-dismiss = Dispensar
upload-cancel = Cancelar
upload-minimize = Minimizar
upload-expand = Mostrar importações
upload-pill = A importar { $count } · { $percent }%
upload-pill-stopped = Paradas: { $count }
upload-interrupted = Interrompida
upload-resume = Retomar
upload-resume-all = Retomar todas
upload-resume-hint-folder = Escolha novamente a mesma pasta, ou largue-a na página, para continuar de onde parou.
upload-resume-hint-file = Escolha novamente o mesmo ficheiro, ou largue-o na página, para continuar de onde parou.
upload-not-same = Esses não são os mesmos ficheiros, por isso nada foi retomado.
upload-permission-denied = O navegador não teve permissão para voltar a ler os ficheiros. Escolha-os em vez disso.

## PacksView.vue

packs-rename-failed = Não foi possível mudar o nome: { $error }
packs-collection-deleted = Coleção { $name } eliminada
packs-delete-failed = Não foi possível eliminar: { $error }
packs-status-uploading = A carregar (ficheiros recebidos até agora)
packs-status-ingesting = A descomprimir e etiquetar
packs-status-analyzing = O LLM está a etiquetar
packs-status-ready = Pronto
packs-status-failed = Falhou
packs-retag-failed = Não foi possível começar a etiquetar de novo: { $error }
packs-read-files-failed = Não foi possível ler os ficheiros do pacote: { $error }
packs-context-load-failed = Não foi possível carregar os detalhes de loja do pacote: { $error }
packs-undo-store-failed = Não foi possível retirar a página da loja: { $error }
packs-context-save-failed = Não foi possível guardar os detalhes de loja: { $error }
packs-unreal-load-failed = Não foi possível carregar o estado da conversão do Unreal: { $error }
packs-unreal-discard-title = Descartar os originais?
packs-unreal-discard-body = Os ficheiros .uasset originais em { $name } desaparecem depois de convertidos. Os dados só do Unreal que contêm, como os grafos de materiais, perdem-se definitivamente.
packs-unreal-discard-confirm = Converter e descartar
packs-unreal-convert-failed = Não foi possível começar a conversão: { $error }
packs-unreal-fact-converted = { $done } convertidos do Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } package do Unreal
       *[other] { $left } packages do Unreal
    }
packs-unreal-fact-both = { $done } convertidos do Unreal, { $left } ainda em packages
packs-thumbs-canceled = Cancelado após { $done } de { $total }
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
packs-archived = { $name } e os seus { $count } recursos foram arquivados
packs-blobs-bad = { $count } ficheiros falharam a última verificação de integridade.
packs-collections = Coleções
packs-browse-named = Explorar { $name }
packs-collection-name = Nome da coleção
packs-save = Guardar
packs-cancel = Cancelar
packs-assets =
    { $count ->
        [one] { $count } recurso
       *[other] { $count } recursos
    }
packs-updated = Atualizado a { $date }
packs-browse = Explorar
packs-collection-download = Transferir (.zip)
packs-collection-credits = Créditos
packs-collection-credits-title = Quem fez os pacotes de onde vêm estes recursos, as licenças e as linhas de crédito que pedem
packs-rename = Mudar o nome
packs-delete = Eliminar
packs-packs = Pacotes
packs-empty = Ainda não há pacotes. Largue um zip ou uma pasta em qualquer parte desta página para adicionar um.
packs-find = Procurar um pacote
packs-find-placeholder = Procurar um pacote por nome, loja, licença ou etiqueta
packs-sort = Ordem
packs-sort-new = Mais recentes primeiro
packs-sort-old = Mais antigos primeiro
packs-sort-name = Por nome
packs-sort-size = Maiores primeiro
packs-sort-assets = Mais recursos primeiro
packs-with-kind = Pacotes com
packs-state = Estado
packs-state-any = Qualquer estado
packs-state-working = Em curso
packs-state-failed = Falhados
packs-state-untagged = Ainda sem descrição
packs-state-unlicensed = Sem licença indicada
packs-store = Loja
packs-store-any = Qualquer loja
packs-store-from = De { $store }
packs-store-none = Sem página de loja
packs-shown-of = { $shown } de { $total }
packs-clear = Limpar
packs-no-match = Nenhum pacote corresponde.
packs-clear-all = Limpar a pesquisa e os filtros
packs-pack-name = Nome do pacote
packs-thumbs-drawing = A desenhar miniaturas neste navegador…
packs-thumbs-drawing-count = A desenhar miniaturas neste navegador: { $done } de { $total }…
packs-thumbs-stopped = Parado: { $error }
packs-thumbs-none = Todos os modelos aqui já têm uma miniatura desenhada com todas as texturas.
packs-dismiss = Dispensar
packs-license = Licença: { $license }
packs-license-tier = Licença: { $license } ({ $tier })
packs-license-not-found = não encontrada
packs-from-before = De
packs-from-after = { "" }
packs-added = Adicionado a { $date }
pack-panel-label = Sobre este pacote
pack-panel-hide = Ocultar os detalhes do pacote
pack-panel-kinds = O que contém
pack-panel-all = Todos os pacotes
packs-download = Transferir pacote (.zip)
packs-store-details = Detalhes de loja
packs-more-named = Mais ações para { $name }
packs-more = Mais
packs-rename-more = Mudar o nome…
packs-retag-rules = Executar de novo o etiquetador offline
packs-retag-llm = Executar de novo o etiquetador LLM
packs-make-thumbs = Criar miniaturas neste navegador
packs-make-thumbs-title = Abrir cada modelo num visualizador 3D fora de vista, um de cada vez, e usar a imagem dele como miniatura quando for melhor do que a do servidor
packs-make-thumbs-title-desktop = Abrir cada modelo num visualizador 3D fora de vista, um de cada vez, e usar a imagem dele como miniatura quando for melhor do que a que o Hoard fez
packs-read-files = Ler de novo os ficheiros do pacote
packs-credits = Transferir créditos
packs-credits-title = CREDITS.md: quem fez o pacote, a licença e a linha de crédito que pede
packs-unreal-open = Converter conteúdo do Unreal…
packs-archive = Arquivar pacote
packs-unreal-intro = O Hoard lê ele próprio os packages do Unreal e guarda o que contêm como ficheiros comuns: texturas em PNG ou EXR, sons em WAV, materiais em JSON. O ficheiro convertido toma o lugar do package, mantendo as etiquetas, ligações e coleções.
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
        [one] ainda não analisado
       *[other] ainda não analisados
    }
packs-unreal-needs-editor =
    { $count ->
        [one] à espera do Unreal Editor
       *[other] à espera do Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] sem formato comum
       *[other] sem formato comum
    }
packs-unreal-failed =
    { $count ->
        [one] falhou
       *[other] falharam
    }
packs-unreal-reasons = Porque ficam packages por converter
packs-unreal-editor-before = As malhas e animações precisam do Unreal Editor. Numa máquina com o Unreal instalado, execute
packs-unreal-editor-command = hoard unreal-worker --editor <caminho para UnrealEditor-Cmd>
packs-unreal-editor-after = ; exporta-as em FBX e carrega os resultados para aqui.
packs-unreal-discard = Descartar os ficheiros .uasset originais depois de convertidos
packs-unreal-discard-hint = Liberta o espaço. Os dados só do Unreal, como os grafos de materiais, perdem-se.
packs-unreal-drop = Eliminar os packages sem formato comum
packs-unreal-drop-hint = Blueprints, níveis e outros recursos que só existem no motor.
packs-unreal-convert = Converter
packs-context-hint-before = De onde veio o pacote e o que o vendedor diz sobre ele. O etiquetador LLM lê isto com cada ficheiro, por isso um texto da loja como "120 feitiços de gelo e fogo" ajuda-o a dar nome a
packs-context-hint-after = .
packs-store-auto = Encontrada automaticamente: o próprio pacote indica esta página.
packs-store-not-this = Não é esta
packs-lookup-label = Pesquisar nas lojas por
packs-lookup-placeholder = O nome do pacote na loja
packs-lookup-searching = A pesquisar…
packs-lookup-search = Pesquisar nas lojas
packs-lookup-hide = Ocultar
packs-lookup-clues = Dos ficheiros do pacote: { $clues }.
packs-lookup-also = Também pesquisado: { $queries }.
packs-lookup-quoted = «{ $query }»
packs-lookup-nothing = Nada na Fab, itch.io, Kenney ou OpenGameArt se parece com isto. Experimente menos palavras ou palavras diferentes.
packs-lookup-use = Usar
packs-lookup-searched = Pesquisado a { $date }.
packs-lookup-again = Pesquisar de novo
packs-lookup-again-after = .
packs-lookup-unreachable = Não foi possível contactar { $stores } desta vez.
packs-lookup-also-look = Procure também em
packs-lookup-links-and = { " " }e{ " " }
packs-lookup-browser-only = (só podem ser pesquisadas no seu navegador); cole abaixo o endereço da página.
packs-lookup-open = Procurar nas lojas…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Vendedor
packs-seller-placeholder = Quem o fez
packs-license-label = Licença
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Nível de licença
packs-license-tier-placeholder = Pessoal, Profissional…
packs-store-page = Página da loja
packs-store-text = Descrição da loja
packs-store-text-placeholder = Cole o texto do anúncio, ou deixe vazio para o obter da página da loja.
packs-store-text-fetch = Guardar obtém a descrição da página da loja.
packs-readme = README encontrado no pacote
packs-save-retag = Guardar e executar de novo o etiquetador LLM

## AssetDetail.vue

asset-lazy-3d-viewer = o visualizador 3D
asset-lazy-text = o texto
asset-lazy-text-viewer = o visualizador de texto
asset-lazy-image-viewer = o visualizador de imagens
asset-lazy-material-preview = a pré-visualização do material
asset-lazy-material-viewer = o visualizador de materiais
asset-lazy-sprite-player = o leitor de sprites
asset-lazy-sheet-view = o visualizador de folhas de sprites
asset-rel-texture = textura
asset-rel-buffer = dados de geometria
asset-rel-material = ficheiro de material
asset-rel-animation = animação
asset-rel-library = ficheiro ligado
asset-rel-guess = provavelmente acompanha
asset-rel-sidecar = detalhes de loja
asset-rel-variant = variante
asset-rel-sprite = sprite
asset-rel-sample = amostra
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
asset-loose-sprites = Também como ficheiros separados
asset-in-sheet = Numa folha de sprites
asset-other-formats = Outros formatos
asset-shown-as = Mostrado na biblioteca como
asset-conversion-failed = a conversão falhou
asset-as-converting = A converter…
asset-as-converting-percent = A converter… { $percent }%
asset-as-queued = Em fila…
asset-download = Transferir
asset-as-label = Transferir como
asset-as-original = { $format } (formato original)
asset-as-pack-files = Os ficheiros do pacote
asset-as-size = Tamanho
asset-as-full-size = Tamanho total
asset-as-up-to = Até { $size } px
asset-as-hint = Escolha um tamanho ou outro formato
asset-meta-size = Tamanho
asset-meta-length = Duração
asset-meta-sample-rate = Frequência de amostragem
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
asset-meta-one-shot = Disparo único
asset-meta-envelope = Envolvente
asset-meta-envelope-times = ataque { $attack }, cauda { $tail }
asset-meta-envelope-hit = pancada
asset-meta-envelope-swell = crescente
asset-meta-envelope-sustained = sustentado
asset-meta-tone = Timbre
asset-meta-tone-value = brilho { $khz } kHz, ruído { $noise } %
asset-meta-pitch = Altura
asset-meta-pitch-rises =
    { $octaves ->
        [one] sobe { $octaves } oitava
       *[other] sobe { $octaves } oitavas
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] desce { $octaves } oitava
       *[other] desce { $octaves } oitavas
    }
asset-meta-loops-cleanly = Repete sem cortes
asset-meta-tempo = Andamento
asset-meta-tempo-heard = cerca de { $bpm } BPM, de ouvido
asset-meta-tempo-heard-or = cerca de { $bpm } BPM (ou { $alt }), de ouvido
asset-meta-key = Tonalidade
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, de ouvido
music-key-major = { $note } maior
music-key-minor = { $note } menor
asset-meta-root-note = Nota fundamental
asset-meta-cue-markers = Marcadores de cue
asset-meta-library = Biblioteca
asset-meta-keywords = Palavras-chave
asset-meta-title = Título
asset-meta-artist = Artista
asset-meta-album = Álbum
asset-meta-genre = Género
asset-meta-generator = Gerador
asset-meta-everything-with = Mostrar tudo com a etiqueta { $tag }
asset-meta-saved-with = Guardado com
asset-meta-color-mode = Modo de cor
asset-meta-layers = Camadas
asset-meta-range = Intervalo
asset-meta-hdr = HDR (pré-visualização com tone mapping)
asset-meta-format = Formato
asset-meta-large-document = Documento grande (PSB)
asset-meta-ai = IA
asset-meta-ai-made = Feito com IA generativa
asset-meta-ai-edited = Editado com IA generativa
asset-meta-ai-how-metadata = metadados
asset-meta-ai-how-generator = definições do gerador
asset-meta-ai-how-iptc = tipo de fonte digital IPTC
asset-meta-ai-says = { $made } (indicado por: { $how })
asset-meta-ai-by-says = { $made } ({ $by }) (indicado por: { $how })
asset-meta-provenance = Proveniência
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
asset-meta-unreal-asset = Recurso do Unreal
asset-meta-unreal-saved-with = { $class }, guardado com o UE { $engine }
asset-meta-package = Package
asset-meta-cooked = Cooked (build de jogo): as pré-visualizações e os originais foram removidos
asset-meta-tiling = Repetição
asset-meta-seamless = Sem junções nos dois eixos
asset-meta-analysis-error = Não foi possível ler este ficheiro por completo: { $error }
asset-copy-blocked = Não foi possível copiar: o navegador bloqueou
asset-save-error = Não foi possível guardar: { $error }
asset-tag-removed = { $tag } removida
asset-undo = Anular
asset-play-error = Não foi possível reproduzir este ficheiro: { $error }
asset-copied-hash = Hash copiado
asset-copied-toml = toml copiado
asset-copied-id = ID copiado
asset-copied-link = Ligação copiada
asset-origin-before = Este ficheiro era
asset-origin-class-engine = , um { $class } guardado pelo UE { $engine }.
asset-origin-class = , um { $class }.
asset-origin-engine = { " " }guardado pelo UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = O Unreal Editor exportou-o a { $date }.
asset-origin-editor-notes = O Unreal Editor exportou-o a { $date } ({ $notes }).
asset-origin-hoard = O Hoard extraiu-o do package a { $date }.
asset-origin-hoard-notes = O Hoard extraiu-o do package a { $date } ({ $notes }).
asset-origin-download = Transferir o .{ $ext } original
asset-origin-discarded = O package original foi descartado após a conversão.
asset-details = Detalhes do recurso
asset-in-pack = em
asset-previous = Recurso anterior
asset-next = Recurso seguinte
asset-favorite = Favorito
asset-favorite-on-title = Nos seus favoritos (F retira-o)
asset-favorite-off-title = Adicionar aos seus favoritos (F)
asset-share = Partilhar
asset-share-title = Partilhar uma ligação para este recurso (quem a abrir inicia sessão no Hoard primeiro)
asset-layout-panel = Mostrar num painel lateral
asset-layout-page = Mostrar em página inteira
asset-layout-page-title = Mostrar em página inteira: uma pré-visualização grande ao lado dos detalhes
asset-close = Fechar detalhes
asset-rename-title = Clique para mudar o nome
asset-name = Nome
asset-views-alt = O modelo visto de quatro direções
asset-inspect-aria = Inspecionar { $name } em ecrã inteiro
asset-inspect-title = Inspecionar: zoom, repetição, canais
asset-playback-position = Posição de reprodução
asset-playback-value = { $current } de { $duration }
asset-pause = Pausar
asset-play = Reproduzir
asset-waveform-drawing = A desenhar a forma de onda…
asset-waveform-skipped = Sem forma de onda para ficheiros tão grandes
asset-play-animation = Reproduzir como animação
asset-show-views = Mostrar a folha de quatro vistas que o etiquetador viu
asset-palette = Cores na imagem
asset-swatch-title = { $hex }, { $percent }% da imagem. Procurar recursos com esta cor
asset-swatch-aria = Procurar recursos com { $hex }, { $percent }% desta imagem
asset-more-like-this = Mais como este
asset-desc-label = A sua descrição
asset-desc-placeholder = O que é, onde o usaria…
asset-save = Guardar
asset-cancel = Cancelar
asset-desc-family = Guarda nas { $count } variantes
asset-desc-edit = Editar descrição
asset-desc-write = Escrever uma descrição
asset-desc-ai = Escrita pela IA
asset-desc-file = Do ficheiro
asset-desc-store-before = Da
asset-desc-store-page = página de { $store }
asset-desc-store-page-unnamed = página da loja
asset-desc-store-after = (o pacote todo)
asset-less = Menos
asset-more = Mais
asset-tags = Etiquetas
asset-tag-add-label = Adicionar uma etiqueta
asset-tag-add-placeholder = Adicione uma etiqueta, p. ex. mood:tense
asset-tag-add = Adicionar
asset-apply-family = Aplicar as alterações de etiquetas, categoria e descrição às { $count } variantes
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
asset-fit-animations = Animações compatíveis ({ $count })
asset-fit-characters = Personagens compatíveis ({ $count })
asset-copies = Cópias idênticas
asset-copies-note = Os mesmos bytes são guardados uma só vez; estes pacotes também trazem este ficheiro.
asset-near-copies = Quase cópias
asset-near-copies-note = A mesma coisa noutro tamanho, taxa de amostragem ou formato. A melhor cópia está assinalada.
asset-near-copies-best = A mesma coisa noutro tamanho, taxa de amostragem ou formato. Esta é a melhor cópia.
asset-near-best = melhor
asset-unreal-converted = Convertido do Unreal
asset-rule-converted = Convertido por uma regra
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Uma regra de conversão criou este { $format } a { $date } a partir de
asset-rule-kept-until = O original é guardado até { $date }.
asset-rule-rebuilt = O FLAC guarda tudo o que havia no WAV, pelo que o WAV pode ser transferido ou voltar exatamente como era a qualquer momento.
asset-rule-put-back = Repor o original
asset-rule-put-back-title = Repor { $file }?
asset-rule-put-back-body = O ficheiro convertido é eliminado e o original volta ao seu lugar, com as etiquetas, coleções e ligações. A partir daí, as regras de conversão deixam-no em paz.
asset-rule-put-back-done = { $file } foi reposto
asset-rule-let-go = O original foi eliminado quando terminou o prazo durante o qual as Definições guardam os originais.
asset-unreal-package = Do package do Unreal
asset-ue-needs-editor-before = À espera do Unreal Editor: { $reason }. Numa máquina com o Unreal instalado, execute
asset-ue-needs-editor-after = e volta em FBX.
asset-ue-editor-path = caminho para UnrealEditor-Cmd
asset-ue-unsupported = Mantido como package: { $reason }.
asset-ue-failed = A conversão falhou: { $reason }
asset-recovered = O .{ $ext } original estava guardado dentro deste package, e o Hoard extraiu-o.
asset-recovered-download = Transferir .{ $ext }
asset-ue-meshes-before = As malhas e animações estão guardadas no formato próprio do Unreal. Converta o pacote (página do pacote, Mais) e depois execute
asset-ue-meshes-after = junto de uma instalação do Unreal Editor para obter FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } etiqueta do asset registry
       *[other] { $count } etiquetas do asset registry
    }
asset-file = Ficheiro
asset-bytes = Bytes
asset-source = Origem
asset-path = Caminho
asset-path-open-folder = Ver o que mais existe em { $folder }
asset-download-maps =
    { $count ->
        [one] Transferir o { $count } mapa
       *[other] Transferir os { $count } mapas
    }
asset-download-linked =
    { $count ->
        [one] Transferir com { $count } ficheiro ligado
       *[other] Transferir com { $count } ficheiros ligados
    }
asset-download-only-title = Só este ficheiro, sem os ficheiros que usa
asset-download-only = Só este ficheiro
asset-include-animations = Incluir animações no .zip
asset-include-character = Incluir a personagem { $name }
asset-copy-toml = Copiar entrada do hoard.toml
asset-copy-id = Copiar ID
asset-teach = Ensinar este ficheiro ao Hoard
asset-teach-title = Descrever este ficheiro de loja para que o Hoard o leia (Definições → Ficheiros de loja)
asset-archive = Arquivar
asset-archive-title = Fora das pesquisas até ser restaurado ou eliminado definitivamente, a partir do Arquivo

## ImageViewer.vue

image-channel-red = Canal vermelho
image-channel-green = Canal verde
image-channel-blue = Canal azul
image-channel-alpha = Canal alfa
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Fundo xadrez (K alterna)
image-bg-black-title = Fundo preto (K alterna)
image-bg-gray-title = Fundo cinzento (K alterna)
image-bg-white-title = Fundo branco (K alterna)
image-bg-checker = fundo xadrez
image-bg-black = fundo preto
image-bg-gray = fundo cinzento
image-bg-white = fundo branco
image-inspect-aria = Inspecionar { $name }
image-preview-size = pré-visualização { $width }×{ $height }, { $bytes }
image-preview = pré-visualização
image-resolution = Resolução
image-full-title = Carregar a imagem completa de { $width }×{ $height } (F); aumentar o zoom para lá da pré-visualização também a carrega
image-full-loading = A carregar a resolução total…
image-full-failed = A resolução total não carregou: tentar de novo
image-full = Resolução total
image-zoom = Zoom
image-zoom-out-title = Reduzir (−)
image-zoom-out = Reduzir
image-zoom-in-title = Ampliar (+)
image-zoom-in = Ampliar
image-fit-title = Ajustar (0)
image-fit = Ajustar
image-actual-title = Píxeis reais (1)
image-view = Vista
image-tile-title = Repetir 3×3 para verificar as junções (T)
image-tile = Repetir
image-channels = Canais
image-close = Fechar (Esc)
image-failed = Não foi possível carregar esta imagem.
image-loading = A carregar…
image-key-gestures = Deslize ou aperte para fazer zoom · arraste para deslocar · duplo clique para ampliar ou ajustar
image-key-tile = repetir
image-key-background = fundo
image-key-full = resolução total
image-key-channels = canais

## SpritePlayer.vue

sprite-frame-aria = { $name }, fotograma { $frame } de { $count }
sprite-failed = Não é possível reproduzir este.
sprite-loading = A carregar fotogramas…
sprite-previous = Fotograma anterior
sprite-next = Fotograma seguinte
sprite-pause = Pausar animação
sprite-play = Reproduzir animação
sprite-animation = Animação
sprite-all-frames = Todos os fotogramas
sprite-grid-title = Como a folha é cortada em fotogramas
sprite-columns = Colunas
sprite-rows = Linhas
sprite-fps-aria = Fotogramas por segundo
sprite-fps = fps

## O visualizador de folhas de sprites (SheetView.vue) e os seus formatos de transferência (AssetDetail.vue)

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
sheet-find = Procurar um sprite
# Under the list of sprites when more match than it draws.
sheet-showing = A mostrar { $shown } de { $total }
sheet-no-match = Nenhum sprite tem isso no nome.
sheet-failed = Não é possível ler esta folha.
sheet-loading = A carregar a folha…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Formato próprio do Hoard: as páginas tal como estão, sprites rodados em qualquer sentido e recortados, com animações e a duração de cada fotograma.
sheet-note-texturepacker = Sprites rodados no sentido dos ponteiros do relógio e recortados, com animações por nome, tal como o PixiJS e o Phaser os leem.
sheet-note-clockwise = Sprites rodados no sentido dos ponteiros do relógio e recortados. Não suporta animações: os fotogramas distinguem-se pelos nomes numerados.
sheet-note-libgdx = Sprites rodados no sentido contrário ao dos ponteiros do relógio e recortados. Não suporta animações: os fotogramas distinguem-se pelos nomes numerados.
sheet-note-godot-frames = Sprites nunca rodados, com o recorte guardado como margens, com animações e a duração de cada fotograma.
sheet-note-godot-tileset = Mosaicos nunca rodados nem recortados, numa grelha do tamanho do mosaico, mantendo as imagens como estão quando já estão numa grelha.
sheet-note-grid = Cada sprite sem rodar, na sua tela inteira, em células todas do mesmo tamanho, para qualquer programa que corte a folha em grelha.
sheet-note-frames = Cada sprite sem rodar, na sua tela inteira, cada um num PNG com o seu nome.

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
text-raw = Em bruto
text-document-title = Como documento (D)
text-document = Documento
text-source-title = O markdown como foi escrito (R)
text-source = Código-fonte
text-page-title = Como um navegador a mostra a partir da respetiva pasta, separada do Hoard (P)
text-page = Página
text-page-source-title = A página tal como foi escrita (R)
text-page-aria = { $name } como página
text-page-error = Não foi possível abrir esta página: { $error }
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
text-download-title = Transferir o ficheiro
text-download = Transferir
text-close = Fechar (Esc)
text-truncated = A mostrar os primeiros { $shown } de { $total }.
text-truncated-json = A mostrar os primeiros { $shown } de { $total } (a árvore precisa do ficheiro completo).
text-show-all = Mostrar tudo
text-json-error = Este JSON não é válido: { $error }
text-contents-aria = Conteúdo de { $name }
text-loading = A carregar…
text-load-error = Não foi possível carregar este ficheiro: { $error }
text-binary = Este ficheiro é binário, não texto, por isso não há nada para ler aqui. Transfira-o para o abrir na ferramenta própria.
text-key-tree = árvore
text-key-pretty = formatado
text-key-raw = em bruto
text-key-document = documento
text-key-source = código-fonte
text-key-page = página
text-key-wrap = quebrar linhas
text-key-copy = copiar
text-key-find = procurar
text-key-close = fechar
text-preview-aria = Ler { $name } em ecrã inteiro
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

## Visualizadores de modelos 3D (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = o visualizador 3D não carregou ({ $error }); recarregar a página pode ajudar
model-loading-viewer = A carregar o visualizador 3D…
model-loading = A carregar o modelo…
model-show-failed = Não foi possível mostrar este modelo: { $error }
model-preview-failed = Não foi possível pré-visualizar este modelo: { $error }
model-error-converting = o servidor ainda está a converter este modelo
model-error-converting-desktop = O Hoard ainda está a converter este modelo
model-error-empty-collada = ficheiro Collada vazio
model-error-no-loader = Não há carregador do navegador para .{ $ext }
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
model-group-camera = Câmara
model-group-shading = Sombreamento
model-group-background = Fundo
model-group-animation = Animação
model-group-view = Vista
model-group-show = Mostrar
model-frame-title = Enquadrar o modelo (F)
model-frame = Enquadrar
model-reset-title = Voltar à vista inicial (R)
model-reset = Repor
model-turn-title = Rodar devagar (A)
model-turn = Rodar
model-textures-title = Texturas (T)
model-textures = Texturas
model-full-res-title = Texturas em resolução total (H): cada ficheiro de textura tal como é, por maior que seja; desligado, pré-visualizações leves de no máximo 2048 px
model-full-res = Res. total
model-light-title = Luz ambiente (E)
model-light = Luz
model-grid-title = Grelha do chão (G)
model-grid = Grelha
model-bg-checker = Fundo xadrez
model-bg-dark = Fundo escuro
model-bg-gray = Fundo cinzento
model-bg-light = Fundo claro
model-bg-env = Fundo do ambiente
model-bg-title = { $background } (K alterna)
model-pause-title = Pausar (Espaço)
model-play-title = Reproduzir (Espaço)
model-pause = Pausar
model-play = Reproduzir
model-animation-clip = Clipe de animação
model-animation = Animação
model-plays-on = Reproduzir em
model-rest-pose = Pose de repouso
model-close-title = Fechar (Esc)
model-full-screen = Ecrã inteiro
model-open-full-screen = Abrir o visualizador 3D em ecrã inteiro
model-menu-fold = Recolher o menu
model-menu-show = Mostrar o menu
model-view-prefix = Vista:{ " " }
model-menu-render = Render
model-menu-maps = Mapas
model-menu-inspect = Inspecionar
model-mode-final = Render final
model-mode-raw = Render em bruto
model-mode-color = Cor base
model-mode-roughness = Rugosidade
model-mode-metalness = Metalicidade
model-mode-normal = Normais
model-mode-ao = Oclusão
model-mode-matcap = Matcap
model-mode-uv = Grelha de teste UV
model-mode-geometry = Geometrias
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Wireframe (W), sobre qualquer vista
model-wireframe = Wireframe
model-info-title = Informações da cena (I)
model-info = Informações da cena
model-info-close = Fechar as informações da cena
model-info-close-title = Fechar (I)
model-info-geometries = Geometrias
model-info-materials = Materiais
model-info-textures = Texturas
model-info-triangles = Triângulos
model-info-vertices = Vértices
model-info-meters = O glTF está em metros
model-info-no-units = Este formato não indica quais são as suas unidades
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
model-guessed = Texturas associadas pelo nome; o ficheiro não indica nenhuma.
model-group-facing = Orientação
model-facing-turn = Rodar um quarto
model-facing-turn-title = Rodar o modelo um quarto de volta no sentido contrário ao dos ponteiros do relógio, visto de cima
model-facing-tip = Inclinar um quarto
model-facing-tip-title = Inclinar o topo do modelo um quarto de volta para a frente, para pôr de pé um modelo deitado
model-facing-save = Usar neste pacote
model-facing-save-title = Rodar assim todos os modelos de { $pack }. As miniaturas são desenhadas de novo; as transferências ficam como foram feitas.
model-facing-undo = Anular
model-facing-undo-title = Voltar à forma como os modelos do pacote estão rodados
model-facing-saved = Todos os modelos de { $pack } estão agora virados assim. As miniaturas estão a ser desenhadas de novo.
model-facing-failed = Não foi possível rodar os modelos do pacote: { $error }
model-keys-mouse = Arraste para orbitar · arraste com o botão direito para deslocar · deslize para fazer zoom · duplo clique para enquadrar
model-key-frame = enquadrar
model-key-reset = repor
model-key-view = vista
model-key-wireframe = wireframe
model-key-info = informações da cena
model-key-textures = texturas
model-key-full-res = texturas em resolução total
model-key-light = luz
model-key-grid = grelha
model-key-background = fundo
model-key-play = reproduzir

## Materiais (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Cor
material-role-normal = Normais
material-role-normal-dx = Normais (DirectX)
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
material-role-specular = Especularidade
material-role-opacity = Opacidade
material-role-emissive = Emissão
material-role-translucency = Translucidez
material-role-base-color-name = cor
material-role-normal-name = normais
material-role-normal-dx-name = normais (DirectX)
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
material-role-specular-name = especularidade
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
material-real-title = Tamanho real: uma repetição mede { $size }, a esfera 2 m de diâmetro (T alterna)
material-repeat-title = Repetir { $n }× (T alterna)
material-real = Real
material-repeat = { $n }×
material-height-title = Quanto o mapa de altura desloca a superfície
material-height = Altura
material-height-strength = Intensidade da altura
material-loading = A carregar o material…
material-show-failed = Não foi possível mostrar este material: { $error }
material-preview-failed = Não foi possível pré-visualizar este material: { $error }
material-group-maps = Mapas
material-map-show = Mostrar o mapa de { $map }
material-map-hide = Ocultar o mapa de { $map }
material-map-unused = Não usado pela pré-visualização
material-open-full-screen = Abrir o visualizador de materiais em ecrã inteiro
material-keys-mouse = Arraste para orbitar · arraste com o botão direito para deslocar · deslize para fazer zoom · clique num mapa para o desligar
material-key-shape = forma
material-key-tiling = repetição
material-key-turn = rodar

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
theme-aubergine = Beringela
theme-oxblood = Bordéu
theme-light-slate-blurb = Cinzento-azulado frio
theme-light-paper-blurb = Cinzento neutro simples
theme-light-linen-blurb = Creme quente, âmbar
theme-light-sage-blurb = Verde suave
theme-light-lilac-blurb = Violeta pálido
theme-light-blush-blurb = Rosa velho
theme-dark-graphite-blurb = Cinzento neutro, sem tonalidade
theme-dark-slate-blurb = Cinzento-azulado
theme-dark-obsidian-blurb = Quase preto, para OLED
theme-dark-ember-blurb = Carvão quente, âmbar
theme-dark-moss-blurb = Preto esverdeado de floresta
theme-dark-aubergine-blurb = Preto ameixa, lilás
theme-dark-oxblood-blurb = Preto vinho, coral

## texnames.ts

texnames-needs-map = O padrão precisa de { "{" }map{ "}" }, senão todos os mapas de um material ficariam com o mesmo nome
texnames-placeholders = O padrão pode usar { "{" }name{ "}" }, { "{" }map{ "}" } e { "{" }size{ "}" }
texnames-bad-characters = Os nomes não podem ter < > : " / \ | ? nem *
texnames-no-suffix = Dê um sufixo aos mapas de { $role }
texnames-bad-suffix = O sufixo dos mapas de { $role } não pode ter < > : " / \ | ? nem *
texnames-slot-no-suffix = Dê um sufixo a «{ $slot }»
texnames-slot-bad-suffix = O sufixo de «{ $slot }» não pode ter < > : " / \ | ? nem *
texnames-ending-missing = Cada terminação personalizada precisa da terminação a procurar
texnames-ending-bad = As terminações personalizadas não podem ter < > : " / \ | ? nem *
texnames-ending-twice = { $ending } aparece duas vezes nas suas terminações
texnames-slot-metallic-roughness = Metal e rugosidade
texnames-slot-specular-color = Cor especular
texnames-slot-specular-glossiness = Especularidade e brilho
texnames-slot-clearcoat = Verniz
texnames-slot-clearcoat-roughness = Rugosidade do verniz
texnames-slot-clearcoat-normal = Normais do verniz
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
storefiles-field-title-hint = O título exato do anúncio. Dá nome ao pacote, e sem ele o ficheiro é ignorado.
storefiles-field-seller = Vendedor
storefiles-field-seller-hint = Quem o fez ou vende.
storefiles-field-license = Licença
storefiles-field-license-hint = Como o ficheiro a escreve, p. ex. CC-BY-4.0.
storefiles-field-url = Página da loja
storefiles-field-url-hint = O endereço do anúncio, começado por http:// ou https://.
storefiles-field-words = Etiquetas
storefiles-field-words-hint = Palavras com que o etiquetar. Os valores separados por vírgulas são divididos.
storefiles-field-type = Tipo
storefiles-field-type-hint = O que é, p. ex. «3D model» ou «surface». Também serve de etiqueta.
storefiles-field-colors = Cores
storefiles-field-colors-hint = As suas cores, como etiquetas color:.
storefiles-field-id = ID da loja
storefiles-field-id-hint = O ID que a loja lhe dá: letras, dígitos, _ ou -.
storefiles-field-origin = Origem
storefiles-field-origin-hint = Onde foi feito ou digitalizado, para a descrição.
storefiles-field-size = Tamanho
storefiles-field-size-hint = O tamanho, em palavras, para a descrição.
storefiles-field-title-name = o título
storefiles-field-seller-name = o vendedor
storefiles-field-license-name = a licença
storefiles-field-url-name = a página da loja
storefiles-field-words-name = as etiquetas
storefiles-field-type-name = o tipo
storefiles-field-colors-name = as cores
storefiles-field-id-name = o ID da loja
storefiles-field-origin-name = a origem
storefiles-field-size-name = o tamanho

## StoreFormatsPanel.vue

storefiles-load-failed = Não foi possível carregar os formatos de ficheiros de loja: { $error }
storefiles-remove-title = Remover { $name }?
storefiles-remove-body = { $file } é eliminado da pasta de dados. Os pacotes já lidos com ele mantêm o que leu.
storefiles-remove = Remover
storefiles-removed = { $name } removido.
storefiles-remove-failed = Não foi possível remover { $name }: { $error }
storefiles-unread = Ficheiros que o Hoard não lê
storefiles-loading = A carregar…
storefiles-unread-none = Nenhum: nenhum ficheiro na biblioteca parece ser de uma loja não lida.
storefiles-unread-what = Ficheiros em mais do que um pacote, ou com nome de ficheiro de loja, que nenhum formato lê.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pacote
       *[other] { $count } pacotes
    }
storefiles-unread-example = p. ex. { $pack }
storefiles-teach = Ensinar este ficheiro ao Hoard
storefiles-formats = Formatos
storefiles-formats-what = Os seus são experimentados primeiro e substituem um formato incorporado com o mesmo nome.
storefiles-dir-before = Estão guardados em
storefiles-dir-after = .
storefiles-yours = Seu
storefiles-built-in = Incorporado
storefiles-edit = Editar
storefiles-start-from = Começar a partir deste

## TagsPanel.vue

tags-load-failed = Não foi possível carregar as etiquetas: { $error }
tags-rename-title = Mudar o nome de { $tag }?
tags-merge-title = Fundir { $count } etiquetas?
tags-rename-body = Os seus { $count } recursos passam a ter { $into }, e os etiquetadores usam o novo nome a partir de agora.
tags-merge-body = { $tags } passam a ser { $into } em até { $count } recursos. Os etiquetadores usam { $into } a partir de agora.
tags-rename = Mudar o nome
tags-merge = Fundir
tags-merged =
    { $count ->
        [one] { $count } recurso agora etiquetado com { $into }
       *[other] { $count } recursos agora etiquetados com { $into }
    }
tags-merge-failed = Não foi possível fundir: { $error }
tags-forget-failed = Não foi possível alterar isso: { $error }
tags-find = Procurar etiquetas
tags-find-placeholder = Procurar etiquetas, p. ex. rock ou set:
tags-count =
    { $count ->
        [one] { $count } etiqueta
       *[other] { $count } etiquetas
    }
tags-count-more = { $count }+ etiquetas
tags-what = Escolha etiquetas que significam o mesmo e funda-as, ou escolha uma para lhe mudar o nome.
tags-new-name = Novo nome
tags-merge-into = Fundir em
tags-saved-as-before = Guardada como
tags-saved-as-after = { "" }
tags-cancel = Cancelar
tags-loading = A carregar…
tags-none = Não há etiquetas.
tags-none-containing = Não há etiquetas que contenham «{ $text }».
tags-browse-title = Explorar os recursos etiquetados com { $tag }
tags-browse = Explorar
tags-merged-heading = Etiquetas fundidas
tags-merged-what = Quando os etiquetadores chegam a um destes nomes antigos, o recurso recebe a etiqueta em que foi fundido.
tags-allow-old = Permitir de novo o nome antigo

## LibraryStats.vue

stats-source-human = Adicionadas por pessoas
stats-source-ai = Sugeridas pela IA
stats-source-rule = Regras incorporadas
stats-source-meta = Dos ficheiros
stats-assets = Recursos
stats-packs = Pacotes
stats-on-disk = Em disco
stats-duplicates-title = { $count } ficheiros são cópias byte a byte de outros e são guardados uma só vez
stats-saved-dedup = Poupado pela desduplicação
stats-distinct-tags = Etiquetas diferentes
stats-archive = No arquivo
stats-load-failed = Não foi possível carregar os detalhes: { $error }
stats-counting = A contar…
stats-as-of = A { $time }
stats-updating = A atualizar…
stats-by-kind = Espaço por tipo
stats-files-size = { $name }: { $count } ficheiros, { $size }
stats-months = Adicionados no último ano
stats-month = { $month }: { $count } recursos, { $size }
stats-tagging = Etiquetagem
stats-tagged = Etiquetados
stats-untagged = { $count } sem etiquetas
stats-described = Descritos
stats-described-assets = { $count } recursos
stats-human-edited = Editados à mão
stats-human-edited-unit =
    { $count ->
        [one] recurso
       *[other] recursos
    }
stats-source-different = { $count } diferentes
stats-most-used = Mais usadas
stats-show-tag = Mostrar os recursos com { $tag }
stats-file-types = Tipos de ficheiro
stats-biggest-packs = Maiores pacotes
stats-browse-pack = Explorar { $name }
stats-largest-files = Maiores ficheiros
stats-open-file = Abrir { $name }
stats-empty = Ainda não há nada na biblioteca. Adicione um pacote e os números aparecem aqui.

## HoardPanel.vue

hoard-title = O seu tesouro
hoard-what = Cada recurso da biblioteca é uma peça do tesouro, e o seu tipo decide qual. Recolha o que é novo para o juntar ao seu monte.
hoard-sign-in = Inicie sessão para juntar um tesouro só seu.
hoard-new = Saque novo para recolher: { $count }
hoard-none-new = Nada de novo para recolher.
hoard-collect = Recolher
hoard-unavailable = Este navegador não consegue desenhar o tesouro: { $error }
hoard-collect-failed = Não foi possível registar o que recolheu: { $error }
hoard-arcade = Ecrã de arcada
hoard-full-screen = Ecrã inteiro
hoard-leave-full-screen = Sair do ecrã inteiro
hoard-label = Um dragão a guardar o seu tesouro. Peças: { $count }
hoard-gold = Ouro
hoard-gems = Gemas
hoard-books = Livros
hoard-weapons = Armas
hoard-bones = Ossos
hoard-tablets = Tábuas rúnicas
hoard-paintings = Pinturas
hoard-potions = Poções

## UpdatesPanel.vue

updates-saved = O Hoard vai chamá-lo quando houver uma atualização pronta
updates-saved-none = Nenhum endereço a chamar
updates-save-failed = Não foi possível guardar
updates-call-answered = Respondeu { $result }
updates-call-refused = Não aceitou o pedido: { $result }
updates-call-failed = Não foi possível chamá-lo
updates-called = Chamado a { $when } para a { $version }: { $result }
updates-called-failed = Chamado a { $when } para a { $version }: { $result }. O Hoard tenta de novo dentro de uma hora.
updates-latest = O Hoard { $version } é a versão mais recente.
updates-out = Saiu o Hoard { $version }.
updates-out-idle = Saiu o Hoard { $version }; a chamada é feita dentro de poucos minutos, enquanto nada estiver a decorrer.
updates-out-busy = Saiu o Hoard { $version }; à espera de que nada esteja a decorrer ({ $work }).
updates-busy-more = mais { $count }
updates-busy-jobs =
    { $count ->
        [one] { $count } tarefa à espera
       *[other] { $count } tarefas à espera
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } carregamento
       *[other] { $count } carregamentos
    }
updates-list-separator = ,{ " " }
updates-what = Quando sai uma nova versão, o Hoard pode chamar um endereço assim que nada estiver a decorrer (sem carregamentos, tarefas, conversões nem manutenção), para que aquilo que atualiza o contentor não o reinicie a meio de um carregamento. Envia um POST com a versão num corpo JSON. Isto precisa da verificação de versões acima.
updates-url = Endereço a chamar
updates-url-placeholder = Nenhum (p. ex. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Guardado (escreva para substituir)
updates-auth-placeholder = Nenhum (p. ex. Bearer my-token)
updates-remove = Remover
updates-save = Guardar
updates-call-now-title = Executa aquilo a que estiver ligado
updates-call-now = Chamar agora
updates-what-calls = O que pode chamar?
updates-watchtower = A API HTTP do Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = , com{ " " }
updates-watchtower-token = { " " }e o token de{ " " }
updates-watchtower-its = { " " }(com o endpoint{ " " }
updates-watchtower-endpoint = { " " }ativado).
updates-portainer = Um webhook do Portainer
updates-portainer-rest = { " " }na stack ou no contentor do Hoard, com a opção de voltar a obter a imagem ativada. O token está no URL.
updates-script = Um script no anfitrião
updates-script-listener = { " " }atrás de um recetor de webhooks (como{ " " }
updates-script-runs = ) que executa{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }ou qualquer outra coisa que aceite um webhook.
updates-poll-before = Ou dispense o endereço: um script com um token de acesso pode consultar{ " " }
updates-poll-until = { " " }até{ " " }
updates-poll-and = { " " }estar definido e{ " " }
updates-poll-after = { " " }ser true, e depois obter a nova versão e reiniciar o Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Não foi possível carregar a Manutenção
maintenance-reanalyze-title = Reanalisar todos os pacotes?
maintenance-reanalyze-body = Todos os ficheiros são lidos de novo, o que pode demorar muito numa biblioteca grande; os pacotes aparecem como em curso até chegar a sua vez. Os seus próprios nomes, etiquetas e notas mantêm-se.
maintenance-reanalyze-confirm = Reanalisar
maintenance-gc-title = Eliminar os ficheiros não usados?
maintenance-gc-body = Os ficheiros guardados para os quais nada na biblioteca aponta são eliminados definitivamente. Os pacotes no arquivo mantêm os seus.
maintenance-gc-confirm = Eliminá-los
maintenance-start-failed = Não foi possível começar «{ $task }»
maintenance-just-now = agora mesmo
maintenance-minutes-ago = há { $count } min
maintenance-hours-ago = há { $count } h
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = A carregar…
maintenance-running = Em execução…
maintenance-starting = A começar…
maintenance-run = Executar
maintenance-started = { $detail } · começou { $when }
maintenance-starting-detail = A começar
maintenance-finished = Concluído:{ " " }
maintenance-failed = Falhou:{ " " }
maintenance-never-run = Nunca executado
maintenance-history = Execuções anteriores
maintenance-col-task = Tarefa
maintenance-col-finished = Concluída
maintenance-col-took = Duração
maintenance-col-result = Resultado
maintenance-script-before = As cópias de segurança e a verificação de danos também podem ser executadas a partir de um script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }e{ " " }
maintenance-script-after = . Essas execuções também aparecem aqui.

## SettingsPage.vue

settings-title = Definições
settings-loading = A carregar…
settings-find = Encontrar uma definição
settings-find-none = Nenhuma definição corresponde a «{ $query }».
settings-load-failed = Não foi possível carregar as definições: { $error }
settings-page-appearance = Aspeto
settings-page-appearance-blurb = Os temas que este navegador usa quando está claro e quando está escuro. Guardado só neste navegador.
settings-page-dragon = Dragão
settings-page-dragon-blurb = O tesouro do seu dragão: cada recurso da biblioteca é uma peça dele.
settings-page-library = Biblioteca
settings-page-library-blurb = O que há no Hoard: o tamanho, de que é feito, como cresceu e quão bem está etiquetado. As coisas arquivadas não contam.
settings-page-copies = Quase cópias
settings-page-copies-blurb = Ficheiros que são a mesma imagem, som ou modelo noutro tamanho, taxa de amostragem ou formato, primeiro os que ocupam mais espaço. A melhor de cada grupo está assinalada; nada é eliminado.
copies-loading = A procurar quase cópias…
copies-refreshing = A biblioteca mudou desde que estas foram encontradas. A procurar de novo…
copies-none = Nenhuma quase cópia: cada imagem, som e modelo está aqui apenas uma vez.
copies-load-failed = Não foi possível listar as quase cópias: { $error }
copies-sum = Grupos de quase cópias: { $count }. As cópias que não são a melhor ocupam { $size }.
copies-set = Grupo { $n }
copies-set-wasted = { $size } nas cópias que não são a melhor
copies-more = Mostrar mais (faltam { $count })
settings-page-tags = Etiquetas
settings-page-tags-blurb = Todas as etiquetas da biblioteca. Funda as que significam o mesmo, ou mude o nome de uma; os etiquetadores usam o novo nome a partir daí.
settings-page-store-files = Ficheiros de loja
settings-page-store-files-blurb = Os ficheiros que as lojas põem ao lado dos seus conteúdos, que o Hoard lê para dar nome aos pacotes e creditar os autores. Ensine-lhe os que não conhece, e partilhe o que lhe ensinou.
settings-page-locations = Locais da biblioteca
settings-page-locations-blurb = As pastas no servidor que contêm a biblioteca: aquelas onde o Hoard guarda ficheiros, e pastas de pacotes que indexa onde estão.
settings-page-locations-blurb-desktop = As pastas que contêm a biblioteca: aquelas onde o Hoard guarda ficheiros, e pastas de pacotes que indexa onde estão.
settings-page-tools = Ferramentas
settings-page-tools-blurb = Programas externos que o Hoard usa para converter e ler ficheiros.
settings-page-ai = Etiquetagem por IA
settings-page-ai-blurb = Um modelo de IA dá nome, descreve e etiqueta os pacotes novos, além das regras incorporadas. As etiquetas dele ficam marcadas como sugeridas.
settings-page-search-blurb = Pesquise imagens pelo que mostram e sons pela forma como soam, não só pelos nomes, e encontre coisas semelhantes no tipo, não só na cor.
settings-page-search-tab = Pesquisa
settings-page-adding = Adição de pacotes
settings-page-adding-blurb = O que o Hoard faz com um pacote quando é adicionado, além de o ler e etiquetar.
settings-page-downloads = Transferências
settings-page-downloads-blurb = Como os ficheiros são nomeados no que o Hoard envia: transferências, zips, a ferramenta hoard e assistentes de IA. Os ficheiros na biblioteca ficam como estão.
settings-page-account = Conta
settings-page-account-blurb = A sua palavra-passe, as outras pessoas que podem iniciar sessão, e tokens de acesso para scripts e outros programas.
settings-page-auth = Autenticação
settings-page-auth-blurb = Como as pessoas iniciam sessão: as contas do próprio Hoard, um diretório LDAP, o Microsoft Active Directory ou um fornecedor OAuth, quanto tempo duram as sessões e que grupos recebem que função.
settings-page-connection = Ligação
settings-page-connection-blurb = Como os navegadores e as aplicações chegam ao Hoard: o endereço pelo qual é conhecido, e o HTTPS, de que algumas funcionalidades do navegador precisam.
settings-page-performance = Desempenho
settings-page-performance-blurb = O esforço que o servidor faz, como as miniaturas são feitas e guardadas, e o máximo que aceita.
settings-page-performance-blurb-desktop = O esforço que o Hoard faz, como as miniaturas são feitas e guardadas, e o máximo que aceita.
settings-page-issues = Problemas
settings-page-issues-blurb = O que correu mal ao abrir ou processar ficheiros: arquivos que não se descomprimiram, ficheiros que não puderam ser lidos, tarefas que falharam. Tente-os de novo quando a causa estiver resolvida, ou dispense-os.
settings-page-maintenance = Manutenção
settings-page-maintenance-blurb = Faça cópias de segurança, verifique danos e arrume. Executa-se uma de cada de cada vez, e pode sair desta página enquanto decorrem.
settings-page-about = Acerca de
settings-page-about-blurb = O que é o Hoard, que versão é esta e o trabalho de outras pessoas com que é feito.
settings-the-default = a predefinição
settings-default-on = ligado
settings-default-off = desligado
settings-default-auto = automático
settings-option-saved = { $name }: { $value }
settings-back-to-default = Repor a predefinição
settings-save = Guardar
settings-use = Usar
settings-status-off = Desligado

## SettingsPage.vue: Aspeto

settings-themes = Temas
settings-look-light = Claro
settings-look-dark = Escuro
settings-look-group =
    { $scheme ->
        [light] Tema claro
       *[dark] Tema escuro
    }
settings-preview =
    A mostrar o tema { $preview ->
        [light] claro
       *[dark] escuro
    } para que o possa ver. Está a usar o { $used ->
        [light] claro
       *[dark] escuro
    }{ $system ->
        [yes] { " " }(a seguir o sistema)
       *[other] { "" }
    }, que volta quando sair desta página.
settings-preview-back =
    Voltar ao { $used ->
        [light] claro
       *[dark] escuro
    }
settings-look-hint = Alterne entre claro e escuro (ou siga o sistema) na barra superior.

## SettingsPage.vue: Ferramentas

settings-blender-what = Converte modelos .blend, .dae, .abc e .x3d para glTF em segundo plano, para que tenham miniaturas e pré-visualização 3D. Os pacotes adicionados enquanto estava desligado são convertidos quando estiver pronto.
settings-blender-download = Usa um Blender instalado, ou transfere o Blender 4.5 LTS (cerca de 400 MB) de um espelho do blender.org.
settings-ffmpeg-what = Codifica as transferências em MP3, OGG e Opus, e lê áudio que o Hoard não consegue descodificar sozinho (AAC, M4A, WMA, …).
settings-ffmpeg-download = Usa um ffmpeg do sistema com os codificadores certos, ou transfere uma build estática (cerca de 80 MB).
settings-tool-reset = { $tool }: repor a predefinição
settings-tool-no-download = Usa um { $tool } instalado. Não há nenhuma build para transferir para esta máquina, por isso instale-o ou indique o caminho.
settings-tool-path = Caminho do programa { $tool }
settings-mode-off = Desligado
settings-mode-auto = Automático
settings-mode-path = Este programa
settings-tool-looking = A procurar…
settings-tool-downloading = A transferir… o progresso está no indicador de atividade
settings-tool-ready = Pronto: { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Os instrumentos com que os ficheiros MIDI são reproduzidos e convertidos.
settings-soundfont-file = Ficheiro SoundFont
settings-soundfont-placeholder = Encontrado no servidor, ou incorporado
settings-soundfont-placeholder-desktop = Encontrado neste computador, ou incorporado
settings-soundfont-default = encontrado no servidor, ou incorporado
settings-soundfont-default-desktop = encontrado neste computador, ou incorporado
settings-soundfont-send = Escolher um ficheiro…
settings-soundfont-sent = O MIDI agora toca com { $name }

## SettingsPage.vue: Pesquisa de imagens

settings-clip-heading = Pesquisar pelo que as imagens mostram
settings-clip-reset = Pesquisa de imagens: repor a predefinição
settings-clip-saved-on = Pesquisa de imagens: ligada
settings-clip-saved-off = Pesquisa de imagens: desligada
settings-clip-what = Escreva o que procura («rocha com musgo», «porta de ficção científica», «barril de madeira») e as imagens que o mostram aparecem ao lado dos nomes que têm essas palavras. «Mais como este» encontra coisas do mesmo tipo, não só das mesmas cores.
settings-clip-toggle = Ligar a pesquisa de imagens
settings-clip-hint = Usa o modelo SigLIP 2 da Google, executado no processador deste servidor: nada sai dele. Ligá-la transfere o modelo ({ $size } MB) do Hugging Face e depois aprende todas as imagens em segundo plano, com baixa prioridade. Ocupa 350 a 750 MB de memória enquanto trabalha ou enquanto alguém pesquisa. Desligá-la elimina o modelo e mantém o que aprendeu, para a próxima vez.
settings-clip-hint-desktop = Usa o modelo SigLIP 2 da Google, executado no processador deste computador: nada sai dele. Ligá-la transfere o modelo ({ $size } MB) do Hugging Face e depois aprende todas as imagens em segundo plano, com baixa prioridade. Ocupa 350 a 750 MB de memória enquanto trabalha ou enquanto pesquisa. Desligá-la elimina o modelo e mantém o que aprendeu, para a próxima vez.
settings-clip-looking = A procurar o modelo…
settings-clip-downloading = A transferir o modelo… o progresso está no indicador de atividade
settings-clip-ready-empty = Pronto. As imagens são aprendidas à medida que os pacotes são adicionados.
settings-clip-ready-all =
    { $count ->
        [one] Pronto: conhece a { $count } imagem
       *[other] Pronto: conhece as { $count } imagens
    }
settings-clip-learning = Pronto: a aprender imagens, { $done } de { $total } até agora
settings-clip-relearning = Até o novo modelo aprender todas as imagens, as pesquisas usam o que o antigo aprendeu.
settings-clip-unavailable = Não foi possível obter o modelo: { $detail }. A tentar de novo a cada hora.
settings-clip-where = Corre em
settings-clip-here = Este servidor: nada sai dele
settings-clip-here-desktop = Este computador: nada sai dele
settings-clip-hosted = Um serviço alojado
settings-clip-hosted-warning = As imagens saem deste servidor. Uma miniatura de cada imagem da biblioteca e as palavras de cada pesquisa de imagens são enviadas para { $service }, que cobra por elas e as guarda conforme os seus termos.
settings-clip-hosted-warning-desktop = As imagens saem deste computador. Uma miniatura de cada imagem da biblioteca e as palavras de cada pesquisa de imagens são enviadas para { $service }, que cobra por elas e as guarda conforme os seus termos.
settings-clip-hosted-hint = Nada é transferido nem executado aqui: o serviço aprende cada imagem, por lotes. Outro serviço ou modelo aprende-as todas de novo, e o que cada um aprendeu é guardado, para poder voltar atrás.
settings-clip-saved-here = Pesquisa de imagens: corre neste servidor
settings-clip-saved-here-desktop = Pesquisa de imagens: corre neste computador
settings-clip-saved-hosted = Pesquisa de imagens: corre no { $service }
settings-clip-hosted-unavailable = Não foi possível usar { $service }: { $detail }. A tentar de novo a cada hora.
settings-clap-heading = Pesquisar sons pela forma como soam
settings-clap-reset = Pesquisa de sons: repor a predefinição
settings-clap-saved-on = Pesquisa de sons: ligada
settings-clap-saved-off = Pesquisa de sons: desligada
settings-clap-what = Escreva o que procura («trovão ao longe», «bip vítreo de interface», «passos na gravilha») e os sons que soam assim aparecem ao lado dos nomes que têm essas palavras.
settings-clap-toggle = Ligar a pesquisa de sons
settings-clap-hint = Usa o modelo CLAP da LAION, executado no processador deste servidor: nada sai dele. Ligá-la transfere o modelo ({ $size } MB) do Hugging Face e depois ouve o início de cada som em segundo plano, com baixa prioridade. Ocupa até cerca de 1 GB de memória enquanto trabalha ou enquanto alguém pesquisa. Desligá-la elimina o modelo e mantém o que aprendeu, para a próxima vez.
settings-clap-hint-desktop = Usa o modelo CLAP da LAION, executado no processador deste computador: nada sai dele. Ligá-la transfere o modelo ({ $size } MB) do Hugging Face e depois ouve o início de cada som em segundo plano, com baixa prioridade. Ocupa até cerca de 1 GB de memória enquanto trabalha ou enquanto pesquisa. Desligá-la elimina o modelo e mantém o que aprendeu, para a próxima vez.
settings-clap-ready-empty = Pronto. Os sons são aprendidos à medida que os pacotes são adicionados.
settings-clap-ready-all =
    { $count ->
        [one] Pronto: conhece o { $count } som
       *[other] Pronto: conhece os { $count } sons
    }
settings-clap-learning = Pronto: a aprender sons, { $done } de { $total } até agora

## SettingsPage.vue: Etiquetagem por IA

settings-ai-service = Serviço
settings-ai-off = Desligado
settings-ai-off-hint = Os pacotes são etiquetados só com as regras incorporadas.
settings-ai-local = Modelo local
settings-ai-local-hint = O serviço llm no ficheiro compose do Hoard (ou cargo xtask llm). Transfira um modelo abaixo; é executado nesta máquina e nada sai dela.
settings-ai-chatgpt = Plano ChatGPT
settings-ai-chatgpt-hint = Usa o seu plano ChatGPT Go, Plus, Pro ou Business em vez de uma chave de API.
settings-ai-openrouter-hint = Uma chave para centenas de modelos alojados, alguns gratuitos. Os modelos que aceitam imagens estão assinalados.
settings-ai-openai-hint = Alojado pela OpenAI.
settings-ai-anthropic-hint = Alojado pela Anthropic, através do endpoint compatível com a OpenAI.
settings-ai-gemini-hint = Alojado pela Google, através do endpoint compatível com a OpenAI.
settings-ai-ollama-hint = Um servidor Ollama que executa. Altere o endereço se não for um contentor chamado ollama.
settings-ai-custom = Outro (compatível com a OpenAI)
settings-ai-custom-hint = Qualquer coisa que fale a API de chat da OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = A ligar…
settings-ai-connected =
    { $count ->
        [one] Ligado · { $count } modelo
       *[other] Ligado · { $count } modelos
    }
settings-ai-connected-none = Ligado, mas não lista nenhum modelo; escreva o nome do modelo
settings-chatgpt-connected-none = Ligado; escreva o nome do modelo
settings-ai-takes-images = aceita imagens
settings-ai-saved = Etiquetagem por IA: { $service }
settings-ai-saved-off = A etiquetagem por IA está desligada
settings-ai-reset = Etiquetagem por IA: repor a predefinição
settings-ai-on = Ligada: { $service } · { $model }
settings-ai-url = Endereço
settings-ai-key = Chave de API
settings-ai-key-saved = Guardada (escreva para substituir)
settings-ai-key-paste = Cole a sua chave
settings-ai-key-optional = Nenhuma (opcional)
settings-ai-key-remove = Remover
settings-ai-get-key = Obter uma chave
settings-ai-model = Modelo
settings-ai-model-pick = Escolha ou escreva um modelo
settings-ai-model-name = Nome do modelo
settings-ai-refresh = Atualizar
settings-ai-connect = Ligar
settings-ai-need-key = Adicione a chave de API para ver os modelos
settings-ai-vision = Enviar também miniaturas (precisa de um modelo que aceite imagens)
settings-ai-batch = Por pedido
settings-ai-batch-unit = recursos (menos para modelos locais pequenos, mais para modelos alojados grandes)

## SettingsPage.vue: Início de sessão no ChatGPT

settings-chatgpt-account = Conta
settings-chatgpt-checking = A verificar…
settings-chatgpt-status-as = Sessão iniciada como { $email }
settings-chatgpt-status = Sessão iniciada
settings-chatgpt-signed-in-as = Sessão iniciada no ChatGPT como { $email }
settings-chatgpt-signed-in = Sessão iniciada no ChatGPT
settings-chatgpt-signed-out = Sessão terminada no ChatGPT
settings-chatgpt-continue = Continuar com o ChatGPT
settings-chatgpt-sign-out = Terminar sessão
settings-chatgpt-sign-out-title = Terminar sessão no ChatGPT?
settings-chatgpt-sign-out-in-use = A etiquetagem por IA usa o seu plano ChatGPT, por isso para até iniciar sessão de novo ou escolher outro serviço.
settings-chatgpt-sign-out-body = O Hoard esquece o início de sessão e pede à OpenAI para o terminar.
settings-chatgpt-manage = Gerir utilização
settings-chatgpt-finish-before = Conclua o início de sessão na janela que se abriu (
settings-chatgpt-finish-link = abri-la de novo
settings-chatgpt-finish-middle = ). Se terminar numa página que não carrega, num endereço começado por{ " " }
settings-chatgpt-finish-after = , copie esse endereço e cole-o aqui:
settings-chatgpt-paste = O endereço em que o início de sessão terminou
settings-chatgpt-finish = Concluir
settings-chatgpt-usage = Os pedidos de etiquetagem contam para a utilização do seu plano ChatGPT. Gerir utilização mostra o que o Hoard usou e permite limitá-lo. Para desligar o Hoard no próprio ChatGPT, vá a Settings → Security and login → Sign in with ChatGPT em chatgpt.com.

## SettingsPage.vue: Modelos locais

settings-local-downloading-progress = A transferir { $percent }% · { $done } de { $total }
settings-local-downloading = A transferir…
settings-local-incomplete = Transferência parada
settings-local-failed = Não arrancou da última vez
settings-local-in-use-running = Em uso · em execução
settings-local-in-use = Em uso
settings-local-starting = A arrancar…
settings-local-running = Em execução
settings-local-ready-images = Pronto · aceita imagens
settings-local-ready-text = Pronto · só texto
settings-local-delete-title = Eliminar { $name }?
settings-local-delete-in-use = A etiquetagem por IA usa este modelo, por isso para até escolher outro. Os ficheiros dele são eliminados do servidor de modelos.
settings-local-delete-downloading = A transferência para e o que já foi transferido é eliminado.
settings-local-delete-body = Os ficheiros dele são eliminados do servidor de modelos. Pode voltar a transferi-lo mais tarde.
settings-local-delete = Eliminar
settings-local-deleting = A eliminar…
settings-local-cancel = Cancelar
settings-local-resume = Retomar
settings-local-download = Transferir
settings-local-start-before = Inicie-o com{ " " }
settings-local-start-or = { " " }(ou{ " " }
settings-local-start-developing = { " " }em desenvolvimento) e depois{ " " }
settings-local-start-retry = tente de novo
settings-local-start-after = . Um servidor que execute um só modelo também funciona: indique abaixo o nome desse modelo.
settings-local-list = No servidor de modelos
settings-local-none = Ainda não foi transferido nada. Escolha um abaixo; fica guardado no servidor, e pode trocar a qualquer momento.
settings-local-catalog = Modelos para transferir
settings-local-size = { $size } GB · { $license }
settings-local-other = Outro modelo do Hugging Face
settings-local-other-placeholder = Outro do Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Adição de pacotes

settings-store-pages = Páginas de loja
settings-store-pages-what = Os pacotes novos são procurados nas lojas em segundo plano, para receberem a página de loja, o vendedor e a licença.
settings-store-lookup = Procurar páginas de loja automaticamente
settings-unreal = Packages do Unreal
settings-unreal-reset = Packages do Unreal: repor a predefinição
settings-unreal-what = O que fazer com os ficheiros .uasset.
settings-conv = Regras de conversão
settings-conv-what = Converta os ficheiros que o Hoard guarda para um só formato, por exemplo todos os OBJ e FBX para GLB, para que a biblioteca fique toda igual. O ficheiro convertido toma o lugar do original, com o nome, as etiquetas, as coleções e as ligações. Os ficheiros em pastas vigiadas nunca são alterados.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Ativa
settings-conv-to = Converter para
# Heading over the file formats a rule converts.
settings-conv-from = De
settings-conv-remove = Remover
settings-conv-remove-label = Remover a regra { $n }
settings-conv-taken = Outra regra já converte este formato
settings-conv-upright = Pôr os modelos direitos ao convertê-los (Y para cima, virados para +Z), tal como os modelos do pacote são mostrados
settings-conv-flac-what = O som fica exatamente igual, ocupando mais ou menos metade do espaço, e todo o resto do WAV (pontos de loop, marcadores de cue, etiquetas) fica guardado no FLAC, pelo que o WAV pode voltar exatamente como era a qualquer momento. Os WAV com amostras de vírgula flutuante ou de 32 bits continuam WAV.
settings-conv-add = Adicionar uma regra
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Guardar os originais durante
settings-conv-keep-days =
    { $count ->
        [one] dia
       *[other] dias
    }
settings-conv-keep-what = Até lá, a página de um ficheiro convertido pode repor o original. As texturas não são copiadas para dentro de um modelo convertido: este aponta para os ficheiros de imagem ao seu lado. Os ficheiros .mtl ou .bin de um modelo mantêm-se.
settings-conv-save = Guardar regras
settings-conv-revert = Anular alterações
settings-conv-existing = Converter ficheiros que já estão na biblioteca
settings-conv-saved = Regras de conversão guardadas. Aplicam-se aos pacotes adicionados a partir de agora.
settings-conv-converting =
    { $count ->
        [one] A converter um ficheiro em segundo plano
       *[other] A converter { $count } ficheiros em segundo plano
    }
settings-conv-nothing = Nada na biblioteca precisa de ser convertido.
settings-conv-ask-title =
    { $count ->
        [one] Converter um ficheiro que já está na biblioteca?
       *[other] Converter { $count } ficheiros que já estão na biblioteca?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Está num pacote.
       *[other] Estão em { $packs } pacotes.
    } Cada original é guardado durante { $days } dias e, até lá, pode ser reposto a partir da sua página.
settings-conv-ask-busy =
    { $count ->
        [one] Um está num pacote que ainda está a ser adicionado; é convertido quando isso terminar.
       *[other] { $count } estão em pacotes que ainda estão a ser adicionados; são convertidos quando isso terminar.
    }
settings-conv-ask-list = Ficheiros que seriam convertidos
settings-conv-ask-more = …e mais { $count }
settings-conv-ask-skipped = Ficam como estão:
settings-conv-skip-watched =
    { $count ->
        [one] um ficheiro numa pasta vigiada
       *[other] { $count } ficheiros em pastas vigiadas
    }
settings-conv-skip-unity =
    { $count ->
        [one] um ficheiro que um projeto Unity conhece pelo nome
       *[other] { $count } ficheiros que um projeto Unity conhece pelo nome
    }
settings-conv-skip-used =
    { $count ->
        [one] um ficheiro referido por uma cena ou por outro ficheiro
       *[other] { $count } ficheiros referidos por uma cena ou por outro ficheiro
    }
settings-conv-skip-beside =
    { $count ->
        [one] um ficheiro cuja cópia convertida já está no seu pacote
       *[other] { $count } ficheiros cujas cópias convertidas já estão nos seus pacotes
    }
settings-conv-skip-blender =
    { $count ->
        [one] um ficheiro que o Blender ainda não leu (Definições → Ferramentas)
       *[other] { $count } ficheiros que o Blender ainda não leu (Definições → Ferramentas)
    }
settings-conv-ask-new = Guardar, só pacotes novos
settings-conv-ask-save-convert =
    { $count ->
        [one] Guardar e convertê-lo
       *[other] Guardar e converter { $count } ficheiros
    }
settings-conv-ask-convert =
    { $count ->
        [one] Convertê-lo
       *[other] Converter { $count } ficheiros
    }
settings-unreal-off = Deixar como estão
settings-unreal-off-hint = Converta a partir da página de um pacote quando quiser.
settings-unreal-keep = Converter, manter os originais
settings-unreal-keep-hint = As texturas, sons e materiais saem em formatos comuns; o .uasset também fica.
settings-unreal-replace = Converter, descartar os originais
settings-unreal-replace-hint = Poupa espaço depois de o conteúdo ser extraído. As malhas continuam à espera do Unreal worker.

## SettingsPage.vue: Transferências

settings-tn = Nomes das texturas
settings-tn-saved = Nomes das texturas guardados
settings-tn-reset = Nomes das texturas: repor a predefinição
settings-tn-what = Os mapas de um material saem com o nome dele e aquilo para que cada mapa serve, seja qual for o nome dos ficheiros: Rock_Normal.png, não rock_moss_xeokah0_raw_8K_Normal.png. Os ficheiros na biblioteca não são tocados, e um mapa cujo nome é indicado por um modelo ou MTL enviado com ele mantém esse nome, para que o modelo continue a encontrá-lo.
settings-tn-toggle = Dar aos mapas de textura o nome do material
settings-tn-style = Estilo
settings-tn-style-group = Estilo de nomes
settings-tn-descriptive = Descritivo
settings-tn-short = Curto
settings-tn-custom = Personalizado
settings-tn-pattern = Padrão
settings-tn-hint-name = { " " }o material,{ " " }
settings-tn-hint-map = { " " }para que serve o mapa,{ " " }
settings-tn-hint-size = { " " }o tamanho, quando o conjunto existe em mais do que um (acrescentado no fim se o padrão o omitir).
settings-tn-example = Exemplo
settings-tn-suffixes = Sufixos para cada tipo de mapa
settings-tn-slots = Nas transferências de GLB, uma textura incorporada num modelo sem ficheiro próprio recebe o nome do material e do slot que ocupa, e esses slots também têm sufixos:
settings-tn-endings = Terminações personalizadas
settings-tn-endings-what = Uma textura que não é um dos mapas de um material mantém o seu nome, a não ser que termine numa destas: _diff → _BC envia rock_diff.png como rock_BC.png. Deixe o segundo vazio para retirar a terminação.
settings-tn-ending-from = Terminação
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Trocar por
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Adicionar uma terminação
settings-tn-ending-remove = Remover
settings-tn-ending-remove-label = Remover { $ending }

## SettingsPage.vue: Geral e Ligação

settings-name = Nome
settings-name-what = Como este servidor se chama na barra superior, no separador do navegador e na página de início de sessão, para que, com mais do que um Hoard, saiba qual é qual.
settings-name-what-desktop = Como esta biblioteca se chama na barra superior.
settings-address = Endereço
settings-address-what = O endereço que as pessoas e as aplicações usam para chegar ao Hoard, quando não é aquele em que este navegador está: o de um proxy inverso, de um túnel ou um nome de tailnet, como https://hoard.example.com. Os links que o Hoard fornece (ao Claude e a outras aplicações, no início de sessão, no seu certificado) usam-no.
settings-public-url = Endereço público
settings-public-url-placeholder = A partir do endereço do navegador (p. ex. https://hoard.example.com)
settings-public-url-default = a partir do endereço do navegador

## ProfilerPanel.vue: o profiler no topo de Definições → Desempenho

profiler = Profiler
profiler-what = O que o Hoard fez nos últimos dez minutos, atualizado a cada dois segundos enquanto está a ver. Fica apenas em memória e recomeça do zero quando o Hoard reinicia.
profiler-starting = As primeiras leituras chegam dentro de momentos.
profiler-failed = Não foi possível ler o profiler: { $error }
profiler-cpu = Utilização do processador
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Máquina inteira
profiler-cpu-summary = O Hoard está a usar { $hoard } dos processadores, e a máquina inteira { $machine }.
profiler-memory = Memória
profiler-serving = Pedidos em curso
profiler-working = A trabalhar em
profiler-working-none = Nada foi executado em segundo plano.
profiler-working-more =
    { $count ->
        [one] E mais uma.
       *[other] E mais { $count }.
    }
profiler-requests = Tempo a servir pedidos
profiler-requests-what = Cada barra corresponde a dez segundos de pedidos, com o tempo dividido conforme onde foi gasto.
profiler-part-db = Base de dados
profiler-part-db-title = A aguardar a base de dados e a executar as suas consultas
profiler-part-disk = Disco
profiler-part-disk-title = A ler ficheiros guardados e a comprimir o que é enviado
profiler-part-send = Envio
profiler-part-send-title = A aguardar enquanto a rede recebe o que foi enviado
profiler-part-other = Outros
profiler-part-other-title = A elaborar a resposta
profiler-request-count =
    { $count ->
        [one] { $count } pedido
       *[other] { $count } pedidos
    }
profiler-requests-none = Nenhum pedido nos últimos dez minutos.
profiler-slowest = Mais lento: { $route }, { $time }
profiler-routes = Por rota
profiler-route = Rota
profiler-count = Pedidos
profiler-average = Média
profiler-p95 = 5% mais lentos
profiler-split = Onde foi gasto o tempo
profiler-sent = Enviado
profiler-portal = os ficheiros da própria interface

## SettingsPage.vue: Desempenho

settings-render = Miniaturas 3D
settings-render-toggle = Renderizar miniaturas dos modelos 3D
settings-gpu-on = Na GPU: { $renderer }
settings-gpu-software = Nenhuma GPU encontrada, por isso em software ({ $renderer })
settings-gpu-none = Nenhuma GPU nem renderizador por software encontrado, por isso os modelos não têm miniaturas
settings-thumbs = Miniaturas
settings-thumbs-what = Como as novas miniaturas e vistas de modelos são guardadas. As que têm transparência, e a pixel art, são sempre PNG. Para reduzir as que já tem, execute Reduzir miniaturas em Manutenção.
settings-previews-size = As pré-visualizações ocupam agora { $size }.
settings-thumbs-saved = Miniaturas: { $quality }
settings-thumbs-small = Mais pequenas
settings-thumbs-small-hint = JPEG, qualidade 70. Bem na grelha; um pouco suaves de perto.
settings-thumbs-balanced = Equilibradas
settings-thumbs-balanced-hint = JPEG, qualidade 82. Difíceis de distinguir do original em tamanho de miniatura.
settings-thumbs-sharp = Mais nítidas
settings-thumbs-sharp-hint = JPEG, qualidade 92. Cerca do dobro do tamanho de Equilibradas.
settings-thumbs-lossless = Sem perdas
settings-thumbs-lossless-hint = PNG, exatamente como foram feitas. Várias vezes maiores.
settings-thumbs-avif = Enviar miniaturas como AVIF
settings-thumbs-avif-what = Os navegadores que mostram AVIF recebem miniaturas mais pequenas, muitas vezes com menos de metade do tamanho. Cada uma é feita a partir da miniatura guardada na primeira vez que é mostrada e guardada com as conversões; até lá, é enviada a guardada. Fazê-las mantém até dois núcleos ocupados enquanto uma biblioteca grande é percorrida pela primeira vez. As miniaturas sem perdas e a pixel art são enviadas tal como estão.
settings-motion-thumbs = Miniaturas em movimento para animações
settings-motion-thumbs-what = Cada animação recebe também um filme curto em ciclo, que é reproduzido enquanto aponta para o seu mosaico. Cada um ocupa cerca de 40 KB e demora algum tempo a criar numa biblioteca grande. Desativar apaga-os.
settings-limits = Trabalho e limites
settings-limits-what = Cada um é guardado quando sai do campo ou prime Enter. Os tamanhos levam unidades: 100 GB, 512 MB, 1,5 TB; um número simples é em GB.
settings-workers = Processos em segundo plano
settings-workers-unit = tarefas em simultâneo (importação, etiquetagem, conversões)
settings-max-upload = Maior carregamento
settings-max-upload-unit = por ficheiro
settings-max-expand = Maior pacote
settings-max-expand-unit = depois de descomprimido
settings-conversion-cache = Transferências convertidas
settings-conversion-cache-unit-now = no máximo, agora { $size }; os usados há mais tempo saem primeiro
settings-conversion-cache-unit = no máximo; os usados há mais tempo saem primeiro

## SettingsPage.vue: Manutenção

settings-updates = Atualizações
settings-version = Este é o Hoard { $version }.
settings-update-check = Verificar no GitHub, a cada poucas horas, se há uma nova versão
settings-desktop-updates = O Hoard procura uma nova versão sempre que é iniciado, transfere-a em segundo plano e instala-a quando o fecha.
about-what = O Hoard guarda recursos de jogos (modelos, texturas, sons, música, sprites e mais) numa biblioteca onde pode pesquisar, pré-visualizar e tirar o que precisar, partilhada com todas as pessoas a quem der acesso.
about-version = Versão
about-edition = Edição
about-edition-server = Servidor
about-edition-desktop = Aplicação de computador
about-edition-steam = Aplicação de computador da Steam
about-made-by = Feito por
about-source = Código-fonte no GitHub
about-issues = Comunicar um problema
about-discord = Converse connosco no Discord
about-steam = Hoard na Steam
about-license = Licença
about-license-what = O Hoard é de código aberto com a licença MIT No Attribution: utilize-o, altere-o e partilhe-o como quiser.
about-downloaded = Transferidos quando necessário
about-downloaded-what = O Hoard não inclui estes programas. Transfere cada um na primeira vez que algo precisa dele, e cada um tem a sua própria licença.
about-blender = Converte modelos 3D e cria as respetivas miniaturas.
about-ffmpeg = Converte som e vídeo e cria as respetivas pré-visualizações.
about-siglip = Encontra imagens pelo que mostram.
about-clap = Encontra sons pela forma como soam.
about-models = Os modelos que escolher para o servidor de IA local estão sujeitos às licenças dos respetivos criadores.
about-built-with = Feito com
about-built-with-what = As bibliotecas de código aberto de que o Hoard é feito, com as respetivas licenças.
about-steamworks = A versão da Steam inclui também o Steamworks SDK, © Valve Corporation.
about-licenses-loading = A carregar a lista…
about-licenses-failed = Não foi possível carregar a lista.
about-filter = Filtrar por nome ou licença
about-count = { $shown } de { $total }
about-no-license = Nenhuma licença indicada

## Tipos de recursos (api.ts)

api-kind-texture = Texturas
api-kind-vector = Arte vetorial
api-kind-sheet = Folhas de sprites
api-kind-audio = Áudio
api-kind-model = Modelos
api-kind-animation = Animações
api-kind-font = Tipos de letra
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
api-kind-font-name = tipos de letra
api-kind-shader-name = shaders
api-kind-material-name = materiais
api-kind-video-name = vídeo
api-kind-document-name = documentos
api-kind-data-name = dados
api-kind-other-name = outros

## Notificações, diálogos e carregamento (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Anular
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
ui-lazy-loading = A carregar { $what }…
ui-lazy-failed = Não foi possível carregar { $what }.
ui-lazy-reload = Recarregar a página

## ResetButton.vue

reset-title = Repor a predefinição: { $value }
reset-label = Repor a predefinição, { $value }

## SizeInput.vue

size-invalid = Um tamanho como 100 GB, 512 MB ou 1,5 TB
size-hint = Um tamanho como 100 GB, 512 MB ou 1,5 TB; um número simples é em GB

## AuthPanel.vue (Definições → Autenticação)

auth-load-failed = Não foi possível carregar as definições de autenticação: { $error }
auth-saved = Guardado. Aplica-se ao próximo início de sessão.
auth-method-internal = Interno
auth-method-internal-hint = Nomes e palavras-passe guardados pelo Hoard, adicionados em Definições → Conta.
auth-method-ldap-hint = Um diretório verifica o nome e a palavra-passe escritos na página de início de sessão: OpenLDAP, FreeIPA, LLDAP ou outro servidor LDAP.
auth-method-ad-hint = As contas de domínio do Windows iniciam sessão com o nome e a palavra-passe habituais.
auth-method-oauth-hint = Um botão «Iniciar sessão com…» que vai à Google, Microsoft, GitHub, Keycloak, Authentik ou outro fornecedor e volta.
auth-session-unused = Após 30 dias sem uso
auth-session-1h = 1 hora após iniciar sessão
auth-session-8h = 8 horas após iniciar sessão
auth-session-day = Um dia após iniciar sessão
auth-session-week = Uma semana após iniciar sessão
auth-session-30d = 30 dias após iniciar sessão
auth-session-hours =
    { $hours ->
        [one] { $hours } hora após iniciar sessão
       *[other] { $hours } horas após iniciar sessão
    }
auth-role-view = Pode ver
auth-role-edit = Pode alterar
auth-role-admin = Administrador
auth-role-word-view = pode ver
auth-role-word-edit = pode alterar
auth-role-word-admin = administrador
auth-role-word-default = { $role } (a função predefinida)
auth-tls-starttls-hint = LDAP simples (porta 389) que passa a TLS antes de enviar o que quer que seja.
auth-tls-ldaps-hint = TLS desde o início (porta 636).
auth-tls-none = Nenhuma
auth-tls-none-hint = As palavras-passe atravessam a rede tal como são. Só numa rede em que confie.
auth-test-failed = Não foi possível testar: { $error }
auth-preset-custom = Outro (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel e outros: indique o emissor e Descobrir preenche os endereços.
auth-preset-google-note = Crie um cliente OAuth (Web application) em Google Cloud Console → APIs & Services → Credentials. A Google não envia grupos, por isso atribua as funções em Definições → Conta, e desligue a criação de contas no primeiro início de sessão, a não ser que qualquer pessoa com uma conta Google deva poder entrar.
auth-preset-microsoft-note = Registe uma aplicação em Entra ID → App registrations (com um redirecionamento Web para o endereço abaixo). Ponha o ID do seu tenant no lugar de «organizations» para deixar entrar só a sua organização. Para funções a partir de grupos, adicione a claim groups em Token configuration: ela lista IDs de grupos.
auth-preset-github-note = Crie uma OAuth app em GitHub → Settings → Developer settings. O GitHub não envia grupos, por isso atribua as funções em Definições → Conta, e desligue a criação de contas no primeiro início de sessão, a não ser que qualquer pessoa com uma conta GitHub deva poder entrar.
auth-preset-gitlab-note = Crie uma aplicação em GitLab → Preferences → Applications (ou na área de administração da sua instância: altere gitlab.com nos endereços).
auth-discovered = Os endereços do fornecedor foram preenchidos.
auth-copied = Copiado
auth-copy-failed = Não foi possível copiar: selecione e copie manualmente
auth-sign-in-with = Iniciar sessão com
auth-own-accounts-still-work = As contas do próprio Hoard continuam a iniciar sessão com as suas palavras-passe, por isso um diretório ou fornecedor em baixo ou mal configurado não o pode deixar de fora.
auth-sessions = Sessões
auth-sign-out-after = Terminar a sessão das pessoas
auth-ldap-server = Servidor LDAP
auth-server-type = Tipo de servidor
auth-server-type-custom = Personalizado
auth-servers = Servidores
auth-server-n = Servidor { $n }
auth-remove-server-n = Remover o servidor { $n }
auth-remove = Remover
auth-add-server = Adicionar um servidor
auth-servers-hint = Quando um não está acessível, é tentado o seguinte.
auth-security = Segurança
auth-connection-security = Segurança da ligação
auth-skip-verify = Não verificar o certificado do servidor (só para experimentar)
auth-ca = Autoridade de certificação
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (vazio: as que o sistema considera fidedignas)
auth-lookup = Procura de pessoas
auth-anonymously = Anonimamente
auth-reader-account = Conta de leitura
auth-reader-password = A palavra-passe dela
auth-saved-type-to-replace = Guardada: escreva para substituir
auth-people-under = As pessoas estão em
auth-name-attribute = Atributo do nome
auth-only-matching = Só as entradas que correspondem a
auth-groups-under = Os grupos estão em
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (vazio: só memberOf)
auth-their-groups = Os grupos delas
auth-group-filter-hint = { "{" }dn{ "}" } é a entrada da pessoa, { "{" }user{ "}" } o nome com que iniciou sessão.
auth-test = Testar
auth-test-hint = Contacta cada servidor e inicia sessão com a conta de leitura. Com um nome e uma palavra-passe, inicia também sessão como essa pessoa e mostra o que o Hoard faria com ela. Nada é guardado.
auth-test-user = Alguém com quem iniciar sessão
auth-test-user-placeholder = Alguém com quem iniciar sessão (opcional)
auth-test-password = A palavra-passe dessa pessoa
auth-testing = A testar…
auth-test-connectivity = Testar a ligação
auth-test-server-ok = contactado, e a conta de leitura iniciou sessão
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }iniciou sessão (
auth-test-signed-in-after = ).
auth-test-role = Teria a função: { $role }.
auth-test-no-role = Não pertence a nenhum dos grupos listados em Funções, por isso seria recusada.
auth-test-groups = Grupos: { $groups }
auth-test-no-groups = Nenhum grupo encontrado.
auth-provider = Fornecedor
auth-redirect = Endereço de redirecionamento
auth-copy = Copiar
auth-redirect-hint = Indique isto ao fornecedor quando criar o cliente do Hoard. Vem do endereço público em Definições → Ligação, ou do endereço que está a usar.
auth-issuer = Emissor
auth-discover = Descobrir
auth-client-id = ID de cliente
auth-client-secret = Segredo do cliente
auth-authorization-url = Endereço de autorização
auth-token-url = Endereço de token
auth-userinfo-url = Endereço de informações do utilizador
auth-userinfo-placeholder = (vazio: o token de ID diz quem é)
auth-scopes = Âmbitos
auth-name-from = Nome a partir de
auth-groups-from = Grupos a partir de
auth-groups-from-placeholder = (nenhum)
auth-button = Botão
auth-button-placeholder = a sua conta
auth-accounts = Contas
auth-provision = Criar uma conta do Hoard da primeira vez que alguém inicia sessão
auth-provision-on-hint = Caso contrário, um administrador adiciona primeiro cada pessoa, em Definições → Conta.
auth-provision-off-hint = Um administrador adiciona primeiro cada pessoa, em Definições → Conta, escolhendo como inicia sessão.
auth-open-to-anyone = Qualquer pessoa com uma conta { $provider } pode entrar. Adicione grupos de visualização abaixo, ou desligue isto e adicione as pessoas manualmente.
auth-new-accounts = Contas novas
auth-roles-from-groups = Funções a partir de grupos
auth-roles-hint = Grupos cujos membros recebem cada função, um por linha, por nome ou DN. Ganha a mais alta que corresponder, e é verificada a cada início de sessão. Sem nenhum listado, as contas novas recebem a função acima e um administrador altera-a depois. Com grupos listados em Pode ver, só os membros de um grupo listado podem iniciar sessão.
auth-roles-hint-oauth = Grupos cujos membros recebem cada função, um por linha, por nome ou DN (ou o que a claim groups listar). Ganha a mais alta que corresponder, e é verificada a cada início de sessão. Sem nenhum listado, as contas novas recebem a função acima e um administrador altera-a depois. Com grupos listados em Pode ver, só os membros de um grupo listado podem iniciar sessão.
auth-saving = A guardar…
auth-save = Guardar

## HttpsPanel.vue (Definições → Ligação)

https-load-failed = Não foi possível carregar o HTTPS: { $error }
https-off-redirecting = O HTTPS está desligado. A levá-lo para http://
https-now-on = O HTTPS está ligado
https-saved = Guardado. As novas ligações usam o novo certificado.
https-now-off = O HTTPS está desligado
https-http3-failed-toast = Não foi possível iniciar o HTTP/3: { $error }
https-copied = Copiado
https-copy-failed = Não foi possível copiar: selecione e copie manualmente
https-mode-off = Desligado
https-mode-off-hint = Só http:// simples.
https-mode-own = O certificado do próprio Hoard
https-mode-own-hint = O Hoard cria um para os endereços deste servidor. Instale uma vez a autoridade de certificação do Hoard em cada dispositivo e os navegadores passam a confiar nele.
https-mode-custom = O meu certificado
https-mode-custom-hint = Um que já tenha, p. ex. do tailscale cert ou do Let’s Encrypt. Os navegadores confiam nele sem instalar nada.
https-untrusted-own = Este navegador ainda não confia nele para { $host }: instale abaixo a autoridade de certificação do Hoard. Vai ver um aviso que pode ignorar.
https-untrusted-uncovered = Este navegador ainda não confia nele para { $host }: o certificado não abrange este endereço. Vai ver um aviso que pode ignorar.
https-untrusted = Este navegador ainda não confia nele para { $host }. Vai ver um aviso que pode ignorar.
https-now = Agora
https-start-failed = O HTTPS está configurado mas não conseguiu arrancar: { $error }
https-on-port = O HTTPS está ligado, na porta { $port }, tal como o http://.
https-using-it = Está a usá-lo agora.
https-open = Abrir { $url }
https-trusted = Este navegador confia nele.
https-cert-for-before = Certificado para{ " " }
https-cert-for-after = , válido até { $date }.
https-cert-for-after-own = , válido até { $date } (renova-se sozinho).
https-not-covered-before = Não abrange{ " " }
https-not-covered-after = , o endereço que está a usar.
https-is-off = O HTTPS está desligado: as páginas são servidas por http:// simples.
https-addresses = Endereços
https-addresses-hint = O certificado abrange os endereços que o Hoard encontrou e os que adicionar (um nome que o seu router lhe dá, um nome do Tailscale).
https-remove-name = Remover { $name }
https-another-address = Outro endereço
https-another-address-placeholder = Outro endereço, p. ex. nas.local
https-add = Adicionar
https-your-cert = O seu certificado
https-saved-cert-before = Guardado:{ " " }
https-saved-cert-after = { " " }de { $issuer }, válido até { $date }. Cole um novo para o substituir.
https-pem-before = O certificado (com a cadeia) e a respetiva chave privada, como texto PEM: os ficheiros que começam por{ " " }
https-pem-after = .
https-certificate = Certificado
https-choose-file = Escolher ficheiro…
https-private-key = Chave privada
https-checked-before = Para{ " " }
https-checked-after = , de { $issuer }, válido até { $date }.
https-checked-after-days =
    , de { $issuer }, válido até { $date } ({ $days ->
        [one] { $days } dia
       *[other] { $days } dias
    }).
https-self-signed = É autoassinado: os navegadores vão avisar.
https-key-matches = A chave corresponde.
https-where-to-get = Onde obter um
https-get-tailscale-before = ative o HTTPS na sua tailnet e depois execute{ " " }
https-get-tailscale-after = { " " }no servidor e carregue os dois ficheiros. Duram 90 dias: execute-o de novo e volte a carregá-los antes disso.
https-get-acme-title = Let’s Encrypt ou outro cliente ACME
https-get-acme-before = (certbot, acme.sh, lego) para um domínio seu: carregue{ " " }
https-get-acme-and = { " " }e{ " " }
https-get-acme-after = .
https-get-proxy-title = Um proxy inverso
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) que tenha o certificado: deixe o HTTPS desligado aqui e deixe o proxy enviar http:// simples ao Hoard.
https-trying = A experimentar…
https-apply = Aplicar
https-apply-hint = O Hoard experimenta primeiro a nova configuração, e mantém a atual se não funcionar.
https-trust-title = Confiar no Hoard nos seus dispositivos
https-download-ca = Transferir hoard-ca.crt
https-trust-hint = Instale uma vez a autoridade de certificação do Hoard em cada dispositivo, e os navegadores passam a confiar neste servidor (e em qualquer certificado que o Hoard crie para ele mais tarde). Só pode garantir este Hoard. Impressão digital (SHA-256):
https-ca-windows = Abra o ficheiro → Instalar Certificado → Utilizador Atual → Colocar todos os certificados no seguinte arquivo → Procurar → Autoridades de Certificação de Raiz Fidedigna → Concluir. Reinicie o navegador.
https-ca-macos = Abra o ficheiro; vai para o Acesso a Porta-chaves. Faça duplo clique em «{ $subject }» → Confiança → Ao utilizar este certificado: Confiar sempre.
https-ca-ios-title = iPhone e iPad
https-ca-ios = Transfira-o no Safari e permita-o. Definições → Perfil descarregado → Instalar. Depois Definições → Geral → Informações → Definições de confiança de certificados → ative «{ $subject }».
https-ca-android = Definições → Segurança → Mais definições de segurança → Encriptação e credenciais → Instalar um certificado → Certificado da AC, e escolha o ficheiro. (Os nomes variam um pouco entre telemóveis; pesquise «certificado» nas Definições.)
https-ca-linux-1 = Debian e Ubuntu:{ " " }
https-ca-linux-2 = . Fedora e Arch:{ " " }
https-ca-linux-3 = . O Chrome no Linux tem a sua própria lista: Definições → Privacidade e segurança → Segurança → Gerir certificados → Importar.
https-ca-firefox = O Firefox tem a sua própria lista: Definições → Privacidade e segurança → Certificados → Ver certificados → Autoridades → Importar, e assinale «Confiar nesta AC para identificar websites».
https-ca-cli-title = A ferramenta de linha de comandos hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , ou ponha o caminho em{ " " }
https-ca-cli-after = .
https-options = Opções
https-redirect = Enviar os navegadores de outros dispositivos de http:// para https://
https-redirect-hint = Só os carregamentos de páginas são redirecionados. A ferramenta de linha de comandos hoard, o Unreal worker, os scripts e as aplicações de IA continuam a funcionar em http://, tal como http://localhost no próprio servidor.
https-http3-hint-before = Mais rápido em Wi-Fi instável e em telemóveis. Precisa de HTTPS, e da porta UDP { $port } aberta além da TCP (no Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Em execução.
https-http3-waits = Arranca quando o HTTPS estiver ligado.
https-http3-failed = Não foi possível iniciar o HTTP/3: { $error }
https-lockout-title = Se ficar sem acesso
https-lockout-1 = { "" }
https-lockout-2 = { " " }no próprio servidor funciona sempre por http:// simples. Ou desligue o HTTPS a partir da linha de comandos do servidor e inicie o Hoard de novo:{ " " }
https-lockout-3 = { " " }(no Docker:{ " " }
https-lockout-4 = , depois{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Definições → Conta)

account-role-view = Pode ver
account-role-view-hint = Pesquisar, pré-visualizar e transferir
account-role-edit = Pode alterar
account-role-edit-hint = Também adicionar, alterar, etiquetar, colecionar e arquivar
account-role-admin = Administrador
account-role-admin-hint = Também as Definições, as contas e a manutenção, e eliminar definitivamente
account-source-internal = Uma palavra-passe do Hoard
account-source-ldap = A palavra-passe { $directory } da pessoa
account-source-oauth = Iniciar sessão com { $provider }
account-source-word-directory = diretório
account-load-failed = Não foi possível carregar as contas: { $error }
account-failed = { $what }: { $error }
account-password-failed = Não foi possível alterar a sua palavra-passe
account-password-changed = Palavra-passe alterada. Os seus outros dispositivos terminaram a sessão.
account-add-failed = Não foi possível adicionar { $name }
account-added = { $name } já pode iniciar sessão.
account-role-failed = Não foi possível alterar o que { $name } pode fazer
account-remove-title = Remover { $name }?
account-remove-body = A sessão termina em todo o lado e não pode voltar a iniciar sessão. Nada do que adicionou à biblioteca é removido.
account-remove = Remover
account-remove-failed = Não foi possível remover { $name }
account-token-failed = Não foi possível criar o token
account-copied = Copiado
account-copy-token-failed = Não foi possível copiar: selecione o token e copie-o manualmente
account-copy-failed = Não foi possível copiar: selecione e copie manualmente
account-revoke-title = Revogar «{ $name }»?
account-revoke-body = O que ainda o usar passa a ser recusado a partir de agora.
account-revoke = Revogar
account-revoke-failed = Não foi possível revogar «{ $name }»
account-disconnect-title = Desligar { $app }?
account-disconnect-body = Passa a ser recusada a partir de agora, e tem de ser ligada de novo para usar o Hoard.
account-disconnect = Desligar
account-disconnect-failed = Não foi possível desligar { $app }
account-guest-failed = Não foi possível alterar isso
account-never = nunca
account-you = Você
account-sign-out = Terminar sessão
account-signed-in-as-before = Sessão iniciada como{ " " }
account-signed-in-as-admin = , que é administrador.
account-signed-in-as-edit = , que pode alterar a biblioteca.
account-signed-in-as-view = , que pode ver mas não alterar nada.
account-app-signin = A aplicação Hoard inicia a sessão por si, por isso não há palavra-passe. Crie abaixo um token de acesso para os plugins dos motores e o comando hoard.
account-password-at-provider = A sua palavra-passe é guardada pelo fornecedor com que inicia sessão, por isso é lá que a altera.
account-password-at-directory = A sua palavra-passe é guardada pelo diretório com que inicia sessão, por isso é lá que a altera.
account-current-password = Palavra-passe atual
account-new-password = Nova palavra-passe
account-new-password-again = Nova palavra-passe outra vez
account-change-password = Alterar a palavra-passe
account-password-mismatch = As novas palavras-passe não coincidem.
account-password-rule = Pelo menos 8 caracteres.
account-people = Pessoas
account-people-hint = Cada pessoa pode ver (pesquisar, pré-visualizar e transferir), também alterar a biblioteca (adicionar, editar, etiquetar, colecionar e arquivar) ou ainda ser administrador (Definições, contas e manutenção, e eliminar definitivamente).
account-you-tag = (você)
account-signs-in-oauth = Inicia sessão com o fornecedor OAuth
account-signs-in-directory = Inicia sessão com o diretório
account-since = desde { $date }
account-what-can-do = O que { $name } pode fazer
account-name = Nome
account-how-sign-in = Como inicia sessão
account-password-label = Palavra-passe (pelo menos 8 caracteres)
account-password-placeholder = Palavra-passe (8+ caracteres)
account-what-they-can-do = O que pode fazer
account-add = Adicionar
account-ai-apps = Claude e outras aplicações de IA
account-ai-apps-hint = Ligue o Claude (ou qualquer aplicação MCP) e ele pode pesquisar e obter recursos por si, e descrever e etiquetar o que o Hoard ainda não descreveu. Inicia sessão no Hoard da primeira vez; autoriza-o aqui, no navegador.
account-mcp-address = Endereço MCP do Hoard
account-copy = Copiar
account-claude-code-1 = execute{ " " }
account-claude-code-2 = , depois{ " " }
account-claude-code-3 = { " " }para iniciar sessão.{ " " }
account-claude-code-4 = { " " }descreve um pacote.
account-claude-ai = Settings → Connectors → Add custom connector, cole o endereço e depois Connect. Estes ligam-se a partir dos servidores da Anthropic, por isso o Hoard tem de estar acessível a partir da internet por HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, cole o endereço e depois Connect. Estes ligam-se a partir dos servidores da Anthropic, por isso o Hoard tem de estar acessível a partir da internet por HTTPS (este endereço não está: defina o Endereço público em Definições → Ligação).
account-connection-meta = para { $user } desde { $since } · último uso { $used }
account-no-apps = Ainda não há aplicações ligadas.
account-tokens = Tokens de acesso
account-tokens-hint-1 = Para a ferramenta de linha de comandos{ " " }
account-tokens-hint-2 = , scripts, clientes MCP e o Unreal worker: enviam-no como{ " " }
account-tokens-hint-3 = . Um token pode fazer o que escolher, até ao que você pode, mas nunca gerir contas ou tokens. Um que só pode ver chega para pesquisar e transferir, e para sincronizar um projeto.
account-tokens-admin = Vê os tokens e as aplicações ligadas de todos; os outros veem os seus.
account-made-before = { "" }
account-made-after = : copie-o agora. Não volta a ser mostrado.
account-new-token = O novo token
account-cli-1 = Para a CLI:{ " " }
account-cli-2 = , ou ponha-o em{ " " }
account-cli-3 = .
account-done = Concluído
account-token-meta = criado a { $made } · último uso { $used }
account-token-meta-by = criado a { $made } por { $by } · último uso { $used }
account-no-tokens = Ainda não há tokens.
account-token-name = O que o vai usar
account-token-name-placeholder = O que o vai usar, p. ex. «Unreal worker»
account-token-role = O que pode fazer
account-make-token = Criar um token
account-guests = Convidados
account-guests-allow = Deixar quem chegar ao servidor explorar sem iniciar sessão
account-guests-hint = Podem pesquisar, pré-visualizar e transferir, mas não adicionar, alterar ou eliminar nada, nem ver as Definições.

## SignIn.vue

signin-create-title = Crie a sua conta
signin-title = Iniciar sessão
signin-setup-lede = Este servidor ainda não tem nenhuma conta. A que criar aqui pode fazer tudo; adicione mais tarde contas para outras pessoas em Definições → Conta.
signin-overlay-lede = Aqui qualquer pessoa pode explorar, mas para alterar é preciso uma conta.
signin-with = Iniciar sessão com { $provider }
signin-or-password = ou com um nome e uma palavra-passe
signin-username = Nome de utilizador
signin-name = Nome
signin-new-password = Nova palavra-passe
signin-password = Palavra-passe
signin-password-rule = Pelo menos 8 caracteres.
signin-confirm-password = Confirmar a palavra-passe
signin-mismatch = As duas palavras-passe não coincidem.
signin-setup-code = Código de configuração
signin-setup-code-hint-before = O código que este servidor recebeu em{ " " }
signin-setup-code-hint-after = .
signin-creating = A criar…
signin-signing-in = A iniciar sessão…
signin-create = Criar conta
signin-submit = Iniciar sessão
signin-keep-browsing = Continuar a explorar
signin-directory-hint = Use o seu nome e palavra-passe { $directory }.
signin-forgot-1 = Esqueceu-se de uma palavra-passe do Hoard? No servidor, execute{ " " }
signin-forgot-2 = { " " }(no Docker,{ " " }
signin-forgot-3 = ) para obter uma nova.

## SetupWizard.vue (primeira visita a um servidor novo)

setup-title = Onde deve ficar a sua biblioteca?
setup-lede = Escolha uma pasta no servidor onde o Hoard guarde os seus pacotes, ou indique uma pasta de pacotes que já tenha. Pode adicionar mais locais, noutros discos, mais tarde nas Definições.
setup-lede-desktop = Escolha uma pasta onde o Hoard guarde os seus pacotes, ou indique uma pasta de pacotes que já tenha. Pode adicionar mais locais, noutros discos, mais tarde nas Definições.
setup-unreachable = Não foi possível contactar o servidor: { $error }
setup-unreachable-desktop = O Hoard não está a responder: { $error }
setup-try-again = Tentar de novo
setup-loading = A carregar…
setup-start = Iniciar a biblioteca
setup-ready = A sua biblioteca está pronta
setup-managed-before = Os pacotes novos vão para{ " " }
setup-managed-after = . Largue ficheiros, pastas ou arquivos em qualquer parte da página para adicionar os primeiros.
setup-watched-before = O Hoard está a vigiar{ " " }
setup-watched-after = ; os pacotes aparecem à medida que os encontra. Para também carregar, adicione uma biblioteca mantida pelo Hoard em Definições → Locais da biblioteca.
setup-open = Abrir a biblioteca

## ConnectApp.vue (uma aplicação como o Claude pede para iniciar sessão)

connect-title = Ligar { $app }?
connect-lede-1 = { " " }vai poder pesquisar, transferir, descrever e etiquetar tudo neste Hoard, em nome de{ " " }
connect-lede-2 = . Não pode gerir contas nem tokens de acesso.
connect-back-before = Vai voltar a{ " " }
connect-back-after = . Desligue-a quando quiser em Definições → Conta.
connect-allow = Permitir
connect-cancel = Cancelar
connect-failed = Não é possível ligar esta aplicação
connect-go-library = Ir para a biblioteca
connect-checking = A verificar…

## InsecureNotice.vue (a linha mostrada sobre http:// simples)

insecure-text = Isto é http:// simples, por isso algumas coisas precisam de HTTPS: 3D com WebGPU, a área de transferência moderna, instalar o Hoard como aplicação e os conectores do Claude.
insecure-switch = Mudar para https://
insecure-set-up = Configurar o HTTPS
insecure-dismiss = Não voltar a mostrar

## FormatWizard.vue (Definições → Ficheiros de loja → ensinar ao Hoard o ficheiro de uma loja)

formatwizard-too-big = Este ficheiro é demasiado grande para ser lido como ficheiro de loja.
formatwizard-not-json = Este ficheiro não é JSON válido, por isso é mostrado como texto.
formatwizard-read-failed = Não foi possível ler o ficheiro: { $error }
formatwizard-back-title = Voltar ao formulário?
formatwizard-back-body = As alterações feitas no TOML perdem-se: o formulário volta a escrever o formato a partir do que tem.
formatwizard-back-to-form = Voltar ao formulário
formatwizard-miss-name = Os ficheiros dele não aceitam o nome { $file }. Adicione-o em Ficheiros.
formatwizard-miss-not-the-stores = Aquilo por que reconhece os ficheiros da loja não está neste ficheiro. Altere Reconhecer por.
formatwizard-miss-no-title = Não encontra nenhum título neste ficheiro. Escolha o título.
formatwizard-miss-short-name = não é o nome dele
formatwizard-miss-short-not-the-stores = não é da loja
formatwizard-miss-short-no-title = sem título
formatwizard-saved-packs =
    { $file } guardado. { $packs ->
        [one] { $packs } pacote está a ser lido
       *[other] { $packs } pacotes estão a ser lidos
    } de novo.
formatwizard-saved = { $file } guardado.
formatwizard-save-failed = Não foi possível guardar o formato: { $error }
formatwizard-zip-failed = Não foi possível criar o zip: { $error }
formatwizard-stop-title = Deixar de ensinar este ficheiro?
formatwizard-stop-body = O formato não é guardado.
formatwizard-stop = Parar
formatwizard-label = Assistente de formatos
formatwizard-changing = A alterar { $name }
formatwizard-starting-from = A partir de { $name }
formatwizard-title = Ensinar ao Hoard o ficheiro de uma loja
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }em{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Editar como TOML
formatwizard-file-reference = O ficheiro, para referência.
formatwizard-click-json = No ficheiro, clique sobre { $field }.
formatwizard-click-text = No ficheiro, clique sobre { $field } ou selecione esse texto.
formatwizard-pick-first = Escolha um campo à direita e depois clique aqui no valor dele.
formatwizard-loading = A carregar…
formatwizard-the-format = O formato
formatwizard-syntax-before = A sintaxe está em{ " " }
formatwizard-syntax-after = .
formatwizard-store = Loja
formatwizard-store-placeholder = O nome da loja, como Kenney
formatwizard-files = Ficheiros
formatwizard-recognize = Reconhecer por
formatwizard-recognize-json = Uma chave que só os ficheiros da loja têm
formatwizard-recognize-text = Uma linha que só os ficheiros da loja têm
formatwizard-recognize-hint = Uma forma por linha; qualquer uma serve. Necessário quando outros ficheiros têm o mesmo nome.
formatwizard-seller = Vendedor
formatwizard-seller-placeholder = Quando o ficheiro não diz
formatwizard-tags = Etiquetas
formatwizard-tags-placeholder = Para tudo o que descreve, separadas por vírgulas
formatwizard-fields = Campos
formatwizard-selector = Seletor do campo { $field }
formatwizard-clear = Limpar o campo { $field }
formatwizard-pick = Escolher
formatwizard-what-it-reads = O que lê
formatwizard-read-title = Título
formatwizard-read-seller = Vendedor
formatwizard-read-license = Licença
formatwizard-read-url = Página da loja
formatwizard-read-kind = Tipo
formatwizard-read-tags = Etiquetas
formatwizard-read-origin = Origem
formatwizard-read-size = Tamanho
formatwizard-also = { $format } também lê este ficheiro. Este formato seria experimentado primeiro.
formatwizard-library =
    { $files ->
        [one] { $files } ficheiro
       *[other] { $files } ficheiros
    } em { $packs ->
        [one] { $packs } pacote
       *[other] { $packs } pacotes
    } { $files ->
        [one] tem
       *[other] têm
    } um nome que aceita. Dos { $tried } experimentados, lê { $read }.
formatwizard-library-none = Nenhum ficheiro da biblioteca tem um nome que aceite.
formatwizard-share = Partilhá-lo
formatwizard-share-hint = Um zip com o formato, este ficheiro como amostra e aquilo que a amostra dá, organizado como o repositório do Hoard, para enviar e todos o terem. Retire primeiro da amostra tudo o que não deva ser partilhado.
formatwizard-sample = Amostra a partilhar
formatwizard-download = Transferir { $file }
formatwizard-close = Fechar
formatwizard-use = Usar aqui
formatwizard-cancel = Cancelar
settings-unreal-editor-what = Instala o Unreal Editor para o processo de exportação. Execute-o separadamente para exportar recursos.
settings-unreal-download = Transfere o editor Linux x86-64 da Epic. Requer acesso associado ao GitHub e dezenas de GB em disco.
settings-unreal-image = Versão ou imagem do Unreal
settings-editor-path = Introduza o caminho do executável do { $tool } instalado. A instalação automática está disponível apenas no Docker.
error-unreal-image = Introduza uma versão do Unreal 5, uma etiqueta de versão dev/dev-slim da Epic ou uma referência de imagem SHA-256 da Epic.
