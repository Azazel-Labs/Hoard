<div align="center">

# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · Português (Brasil) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

</div>

<h3 align="center">Todo pacote de assets que você tem, pesquisável, com prévia e a um clique do seu jogo.</h3>

O Hoard é uma biblioteca de assets para jogos que você mesmo hospeda. Solte um zip, um pacote do Unity, um download da
Fab ou uma pasta com dez mil arquivos, e o Hoard começa a trabalhar. Os modelos giram em 3D com suas texturas, mapas de
textura soltos viram materiais, os sons desenham suas formas de onda, as folhas de sprites tocam suas animações e tudo
ganha tags, então digitar `footsteps gravel` ou `rigged low-poly` encontra o que você procura. Depois, cada asset chega
ao Godot, ao Unity ou ao Unreal com tudo de que precisa.

Sem assinatura, sem upload para a nuvem de ninguém, sem preço por usuário: uma imagem Docker no seu próprio servidor, ou
um app no seu próprio computador, e ele é seu. Pense nele como uma loja de assets sem caixa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="A biblioteca do Hoard: modelos 3D em uma grade, com um baú do tesouro aberto na prévia 3D ao lado das tags e dos downloads dele" src="../img/hero-light.webp">
</picture>

### Veja tudo antes de usar

Os modelos abrem em 3D com suas texturas, materiais e animações. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL e voxels são
lidos pelo próprio Hoard, sem nenhum app 3D instalado; arquivos do Blender entram pelo Blender em modo headless, e
pacotes do Unreal mostram o que o editor salvou sobre eles, sem o Unreal. As miniaturas são desenhadas no servidor,
enquadradas sempre do mesmo jeito, para que um pacote inteiro possa ser visto de relance.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Um lustre em página inteira: uma grande visualização 3D ao lado dos dados, das tags, das texturas vinculadas e dos downloads dele" src="../img/page-light.webp">
</picture>

### Mapas de textura soltos viram materiais

`Rock_BaseColor.png`, `Rock_Normal.png` e `Rock_Roughness.png` viram um único material Rock em uma esfera iluminada. O
Hoard entende a nomenclatura de Megascans, Poly Haven, ambientCG, Substance e Unreal, incluindo mapas empacotados, e
também os arquivos `.mat` do Unity. O visualizador em tela cheia mostra um material em uma esfera, cubo ou plano, na
escala real, com cada mapa podendo ser ligado e desligado.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Um material de tijolos no visualizador em tela cheia, em uma esfera, com chaves para os mapas de cor, normal, rugosidade e altura" src="../img/material-light.webp">
</picture>

### Folhas de sprites que tocam

Atlas do TexturePacker, Starling, libGDX, Cocos2d e Godot viram uma única folha de sprites que toca suas animações e
lista seus sprites. Baixe-a reempacotada para a engine que você estiver usando: dez formatos, cada um recebendo só o
que consegue guardar.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Uma folha de sprites de personagem em página inteira, com uma das animações tocando ao lado das tags e dos formatos de download" src="../img/sheet-light.webp">
</picture>

### Sons que você pode ver

Cada som ganha uma forma de onda colorida conforme o brilho do seu timbre, então um golpe seco e uma ambiência longa
parecem diferentes antes mesmo de você tocá-los. Os metadados de bibliotecas de som são lidos (categorias UCS, campos
do Soundminer e do BaseHead, pontos de loop, andamento, nota fundamental), e músicas MIDI e de tracker são renderizadas
para tocar como qualquer outro som.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Uma grade de efeitos sonoros desenhados como formas de onda coloridas, com um aberto mostrando o player, as tags e os takes" src="../img/sound-light.webp">
</picture>

### Uma busca que sabe o que as coisas são

O Hoard marca tudo com tags sozinho, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0` e uma categoria de uma árvore de assets de jogos. Pesquise por palavras, por cor ou pelo que uma imagem
mostra, e peça “Mais como este” a qualquer textura ou som. Aponte-o para qualquer modelo compatível com a OpenAI, local
ou hospedado, e ele também escreve nomes e descrições.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Uma busca por madeira: uma mesa de madeira, casca de árvore, tábuas e uma superfície de madeira juntas, com filtros por tipo e tag ao lado" src="../img/search-light.webp">
</picture>

### Direto para a sua engine

- **Plugins de engine** para Godot 4, Unity, Unreal Engine 5 e Lumina: navegue pela biblioteca dentro do editor e clique em
  Adicionar ao projeto. O modelo vem com as texturas, no lugar certo, já importado.
- **`hoard sync`** traz para um projeto exatamente os arquivos que ele cita, fixados em um lockfile, para que cada
  colega de equipe e cada máquina de build receba os mesmos bytes.
- **Baixar como** GLB, PNG, WAV, MP3 ou um formato de folha de sprites, convertido na hora, com os nomes das texturas
  na convenção do seu estúdio.
- **Agentes de IA** buscam, inspecionam e baixam assets via MCP.

### Guardado uma vez, mantido em segurança

Cada arquivo é guardado uma única vez pelo seu hash BLAKE3, então a mesma textura em cinco pacotes ocupa o espaço de
uma só, e nenhuma corrupção passa despercebida. Mantenha tudo em disco local ou em qualquer bucket S3. As pastas para
as quais você aponta o Hoard são lidas, nunca alteradas. Contas, tokens somente leitura, login único (single sign-on),
HTTPS e HTTP/3 já vêm embutidos.

### Ou só no seu computador

Não roda um servidor? O app de desktop para Windows, macOS e Linux é a mesma biblioteca em uma janela própria. Instale,
escolha uma pasta e ele funciona offline: sem contas, senhas ou configurações de servidor, nada que o resto da rede
consiga alcançar, e as chaves que você digita guardadas no chaveiro do sistema. Novas versões são baixadas em segundo
plano e instaladas quando você o fecha.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="O app de desktop do Hoard em uma janela do Windows: os modelos da biblioteca em uma grade, com uma lanterna aberta na prévia 3D ao lado das tags e dos downloads dela" src="../img/desktop-light.webp">
</picture>

<sub>A biblioteca de demonstração é arte CC0: modelos do [Poly Haven](https://polyhaven.com) (glTF e USD), superfícies do
[ambientCG](https://ambientcg.com) e sons, folhas de sprites e fontes do [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` gera estas imagens e o cartão social.</sub>

## Instalar o app de desktop

Baixe na [versão mais recente](https://github.com/Azazel-Labs/Hoard/releases/latest): o `-setup.exe` para Windows, o
`.dmg` para macOS em Macs com Apple silicon, o `.AppImage` ou o `.deb` para Linux. No Windows, ele é instalado só para
você, sem pedir permissão de administrador. Os instaladores ainda não são assinados pela Apple nem pela Microsoft, então
o Windows pergunta antes de executar o instalador (Mais informações → Executar assim mesmo) e o macOS antes de abrir o
app pela primeira vez (Ajustes do Sistema → Privacidade e Segurança → Abrir Mesmo Assim).

Na primeira vez que abre, ele pergunta onde a biblioteca vai ficar. Todo o resto fica em Configurações, como em um
servidor.

Depois disso, ele se mantém atualizado sozinho, sem você precisar clicar em nada: uma nova versão é baixada em segundo
plano e instalada quando você fecha o app, então da próxima vez que abrir o Hoard, já é a nova versão. Um `.deb` é
atualizado instalando o próximo.

## Instalar com Docker

O Hoard vem como uma única imagem para amd64 e arm64: `ghcr.io/azazel-labs/hoard`. Salve isto como `compose.yml` (é o
[deploy/compose.yml](../../deploy/compose.yml)):

```yaml
services:
  hoard:
    container_name: hoard
    image: ghcr.io/azazel-labs/hoard:latest
    restart: unless-stopped
    volumes:
      - hoard-config:/config      # database, Settings, backups: small, back it up
      - hoard-library:/library    # your files: big, put it on the disk with room
      - hoard-cache:/cache        # thumbnails, uploads in progress, Blender/ffmpeg: fast disk, can be rebuilt
    ports:
      - "4627:4627"

volumes:
  hoard-config:
  hoard-library:
  hoard-cache:
```

Cada volume pode ser, em vez disso, uma pasta no disco que for mais adequado: `/mnt/big/hoard:/library` coloca os seus
arquivos no disco grande, enquanto o banco de dados continua pequeno e fácil de copiar para backup. As pastas que você
monta não precisam de chown: o Hoard assume `/config` e `/cache` e roda como o dono de `/library` (defina `PUID` e
`PGID` para escolher um usuário você mesmo).

Depois, inicie-o:

```sh
docker compose up -d        # open http://<host>:4627
```

A primeira visita pede que você crie uma conta e depois abre uma configuração curta que pergunta onde a sua biblioteca fica. Ela sugere `/library`, o volume acima; escolha-o
e solte um pacote no portal. Mais locais podem vir depois, em **Configurações → Locais da biblioteca**:

- **Uma biblioteca guardada pelo Hoard** recebe cópias do que você envia ou importa. Cada uma marca os tipos de arquivo
  que aceita (áudio em um disco, modelos em outro), e uma é o local geral para o resto. Mudar o que um local aceita
  vale para arquivos novos; **Mover os arquivos guardados para combinar** move o que já está guardado. **Remover**
  move antes os arquivos de um local para os outros, por tipo, e o remove quando ele fica vazio. Cada arquivo é
  conferido pelo hash antes de a cópia antiga ser excluída. Um local cujo disco sumiu pode ser removido mesmo assim, e
  os arquivos dele passam a aparecer como ausentes.
- **Uma pasta monitorada** é uma pasta de pacotes que você já tem. Cada pasta dentro dela vira um pacote, indexado onde
  está. O Hoard nunca altera, move ou exclui nada ali. Monte-a como somente leitura, se quiser. O Hoard a percorre na
  inicialização e a cada 15 minutos depois (ou quando você pressiona **Varrer agora**), detectando pastas novas,
  arquivos adicionados a um pacote ou removidos dele, e pacotes cuja pasta sumiu. Uma pasta que de repente fica vazia
  ou inacessível é tratada como um disco não montado, e os pacotes dela são mantidos. Um pacote cuja pasta está ausente
  é mantido por três dias, com as tags e notas dele, caso a pasta volte. Tudo o que você exclui definitivamente de um
  pacote monitorado continua excluído quando a pasta é varrida de novo, mesmo que os arquivos ainda estejam lá.

Um local é qualquer pasta que o servidor consegue ver; então, no Docker, monte-a no contêiner primeiro.

**Atualização.** O cabeçalho mostra um selo quando existe uma versão mais nova. Execute `docker compose pull && docker compose up -d`
para instalá-la; o Hoard atualiza o banco de dados ao iniciar, salvando antes uma cópia em `/config/backups`. Mude a tag
da imagem para `:1.3` para receber só versões de correção. Para atualizar automaticamente sem reiniciar o Hoard no meio
de um envio, informe em **Configurações → Manutenção → Atualizações** um endereço a chamar: o Hoard o chama quando sai
uma versão e nada está acontecendo (a API HTTP do Watchtower, um webhook do Portainer, um script; veja
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opcional).** O Hoard renderiza as miniaturas de modelos 3D na GPU quando consegue ver uma, e por software quando
não consegue. A página de pacotes mostra qual (`GPU: ...` ou `No GPU found`). Para repassar a GPU, adicione o overlay do
seu hardware:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Baixe os dois overlays em [deploy/](../../deploy/). Eles também colocam o servidor de modelos local (tags por IA) na GPU. Arquivos em um NAS, armazenamento S3, acesso remoto pelo Cloudflare
ou pelo Tailscale, e backups são tratados em [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Compilar a partir do código-fonte

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Ou solte o zip no portal.

## O resto deste README

O que acontece com um pacote, como levar assets para uma engine, pacotes do Unreal, busca, créditos, MCP,
armazenamento, HTTPS, configuração, segurança, contas, a API e os fatos e números estão descritos no
[README em inglês](../../README.md).

## Traduções

O portal e as mensagens do servidor do Hoard estão disponíveis em 31 idiomas, escolhidos em Configurações → Aparência
(por padrão, o idioma do navegador). As primeiras traduções foram feitas com ajuda de máquina; correções de falantes
nativos são bem-vindas. O [docs/translations.md](../translations.md) explica como.

## Licença

[MIT No Attribution](../../LICENSE) (MIT-0): use, copie, modifique e redistribua o Hoard para qualquer finalidade, sem
precisar manter o aviso de copyright. Os pacotes de assets que você guarda no Hoard mantêm as próprias licenças.
