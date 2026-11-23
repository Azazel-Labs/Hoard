<div align="center">

# <img src="../../web/public/favicon.svg" alt="" height="32" align="absmiddle"> Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· Português&nbsp;(Portugal)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">Todos os pacotes de recursos que tem, pesquisáveis, pré-visualizáveis e à distância de um clique do seu jogo.</h3>

O Hoard é uma biblioteca de recursos para jogos, alojada por si. Largue lá um zip, um pacote do Unity, uma transferência
da Fab ou uma pasta com dez mil ficheiros, e o Hoard põe-se a trabalhar. Os modelos rodam em 3D com as suas texturas,
os mapas de textura soltos tornam-se materiais, os sons desenham as suas formas de onda, as folhas de sprites
reproduzem as suas animações, e tudo recebe etiquetas, para que escrever `footsteps gravel` ou `rigged low-poly` o
encontre. Depois, chega ao Godot, ao Unity ou ao Unreal com tudo aquilo de que precisa.

Sem subscrição, sem envio para a nuvem de ninguém, sem preço por utilizador: uma imagem Docker no seu próprio servidor,
ou uma aplicação no seu próprio computador, e é sua. Pense nele como uma loja de recursos sem caixa de pagamento.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="A biblioteca do Hoard: modelos 3D numa grelha, com uma arca do tesouro aberta na pré-visualização 3D ao lado das suas etiquetas e transferências" src="../img/hero-light.webp">
</picture>

### Veja tudo antes de o usar

Os modelos abrem em 3D com as suas texturas, materiais e animações. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL e voxels
são lidos pelo próprio Hoard, sem nenhuma aplicação 3D instalada; os ficheiros do Blender entram através do Blender em
modo headless, e os pacotes do Unreal mostram o que o editor guardou sobre eles, sem o Unreal. As miniaturas são
desenhadas no servidor, enquadradas sempre da mesma forma, para que um pacote inteiro possa ser percorrido num relance.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Um lustre em página inteira: uma grande vista 3D ao lado dos seus dados, etiquetas, texturas ligadas e transferências" src="../img/page-light.webp">
</picture>

### Mapas de textura soltos tornam-se materiais

`Rock_BaseColor.png`, `Rock_Normal.png` e `Rock_Roughness.png` tornam-se um único material Rock numa esfera iluminada.
O Hoard lê a nomenclatura do Megascans, Poly Haven, ambientCG, Substance e Unreal, incluindo mapas compactados, e
também os ficheiros `.mat` do Unity. O visualizador em ecrã inteiro mostra um material numa esfera, num cubo ou num
plano, à escala real, e cada mapa pode ser ligado e desligado.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Um material de tijolo no visualizador em ecrã inteiro, numa esfera, com interruptores para os seus mapas de cor, normais, rugosidade e altura" src="../img/material-light.webp">
</picture>

### Folhas de sprites que se reproduzem

Atlas do TexturePacker, Starling, libGDX, Cocos2d e Godot tornam-se uma única folha de sprites que reproduz as suas
animações e lista os seus sprites. Transfira-a reempacotada para o motor de jogo que estiver a usar: dez formatos, cada
um a receber apenas o que consegue guardar.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="Uma folha de sprites de uma personagem em página inteira, com uma das suas animações a reproduzir ao lado das etiquetas e dos formatos de transferência" src="../img/sheet-light.webp">
</picture>

### Sons que se veem

Cada som recebe uma forma de onda colorida conforme o brilho do seu timbre, para que um golpe seco e um ambiente longo
tenham aspetos diferentes antes de os reproduzir. Os metadados de bibliotecas de som são lidos (categorias UCS, campos
do Soundminer e do BaseHead, pontos de loop, andamento, nota fundamental), e a música MIDI e de tracker é renderizada
para se reproduzir como qualquer outro som.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Uma grelha de efeitos sonoros desenhados como formas de onda coloridas, com um aberto a mostrar o leitor, as etiquetas e os takes" src="../img/sound-light.webp">
</picture>

### Pesquisa que sabe o que as coisas são

O Hoard etiqueta tudo sozinho, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`, `license:cc0`,
e uma categoria de uma árvore de recursos de jogos. Pesquise por palavras, por cor ou pelo que uma imagem mostra, e
peça «Mais como este» a qualquer textura ou som. Aponte-o para qualquer modelo compatível com a OpenAI, local ou
alojado, e também escreve nomes e descrições.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Uma pesquisa por madeira: uma mesa de madeira, casca de árvore, tábuas e uma superfície de madeira juntas, com filtros por tipo e etiqueta ao lado" src="../img/search-light.webp">
</picture>

### Diretamente para o seu motor de jogo

- **Plugins de motor de jogo** para Godot 4, Unity, Unreal Engine 5 e Lumina: navegue pela biblioteca dentro do editor e
  carregue em Adicionar ao projeto. O modelo vem com as suas texturas, no sítio certo, já importado.
- **`hoard sync`** traz para um projeto exatamente os ficheiros que ele indica, fixados num lockfile, para que cada
  colega de equipa e cada máquina de compilação receba os mesmos bytes.
- **Transferir como** GLB, PNG, WAV, MP3 ou um formato de folha de sprites, convertido na hora, com os nomes das
  texturas segundo a convenção do seu estúdio.
- **Agentes de IA** pesquisam, inspecionam e obtêm recursos através do MCP, e podem fazer tudo o resto que o portal
  faz, com as permissões da conta ou do token com que iniciam sessão.

### Guardado uma vez, mantido em segurança

Cada ficheiro é guardado uma única vez pelo seu hash BLAKE3, por isso a mesma textura em cinco pacotes ocupa o espaço
de uma só, e a corrupção não pode passar despercebida. Guarde tudo num disco local ou em qualquer bucket S3. As pastas
para onde aponta o Hoard são lidas, nunca alteradas. Contas, tokens só de leitura, início de sessão único, HTTPS e
HTTP/3 vêm incorporados.

### Ou apenas no seu computador

Não tem um servidor? A aplicação de ambiente de trabalho para Windows, macOS e Linux é a mesma biblioteca numa janela
própria. Instale-a, escolha uma pasta e funciona sem ligação à Internet: sem contas, palavras-passe nem definições de
servidor, nada que o resto da rede consiga alcançar, e as chaves que introduz guardadas no porta-chaves do sistema. As
novas versões são transferidas em segundo plano e instaladas quando a fecha.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="A aplicação de ambiente de trabalho do Hoard numa janela do Windows: os modelos da biblioteca numa grelha, com uma lanterna aberta na pré-visualização 3D ao lado das suas etiquetas e transferências" src="../img/desktop-light.webp">
</picture>

<sub>A biblioteca de demonstração é arte CC0: modelos do [Poly Haven](https://polyhaven.com) (glTF e USD), superfícies do
[ambientCG](https://ambientcg.com), e sons, folhas de sprites e tipos de letra do [Kenney](https://kenney.nl).
O `node web/scripts/readme-images.mts` cria estas imagens e o cartão para redes sociais.</sub>

## Instalar a aplicação de ambiente de trabalho

Transfira-a da [versão mais recente](https://github.com/Azazel-Labs/Hoard/releases/latest): o `-setup.exe` para Windows,
o `.dmg` para macOS em Macs com Apple silicon, o `.AppImage` ou o `.deb` para Linux. No Windows, é instalada apenas para
si, sem pedir permissões de administrador. Os instaladores ainda não estão assinados pela Apple nem pela Microsoft, por
isso o Windows pergunta antes de executar o instalador (Mais informações → Executar mesmo assim) e o macOS antes de
abrir a aplicação pela primeira vez (Definições do Sistema → Privacidade e segurança → Abrir mesmo assim).

Da primeira vez que abre, pergunta onde fica a biblioteca. Tudo o resto está nas Definições, tal como num servidor.

A partir daí mantém-se atualizada sozinha, sem ter de clicar em nada: uma nova versão é transferida em segundo plano e
instalada quando sair, por isso da próxima vez que abrir o Hoard já é a nova versão. Um `.deb` atualiza-se instalando o
seguinte.

## Instalar com o Docker

O Hoard é distribuído como uma única imagem para amd64 e arm64: `ghcr.io/azazel-labs/hoard`. Guarde isto como
`compose.yml` (é o [deploy/compose.yml](../../deploy/compose.yml)):

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

Em vez disso, cada volume pode ser uma pasta no disco que mais lhe convier: `/mnt/big/hoard:/library` põe os seus
ficheiros no disco grande, enquanto a base de dados se mantém pequena e fácil de copiar. As pastas que montar não
precisam de chown: o Hoard toma conta de `/config` e `/cache` e é executado como o dono de `/library` (defina `PUID` e
`PGID` para escolher um utilizador manualmente).

Depois inicie-o:

```sh
docker compose up -d        # open http://<host>:4627
```

A primeira visita pede-lhe que crie uma conta e depois abre uma configuração curta que pergunta onde fica a sua biblioteca. Sugere `/library`, o volume acima; escolha-o
e largue um pacote no portal. Mais locais podem ser adicionados depois, em **Definições → Locais da biblioteca**:

- **Uma biblioteca mantida pelo Hoard** recebe cópias do que carregar ou importar. Cada uma assinala os tipos de
  ficheiros que aceita (áudio num disco, modelos noutro), e uma delas é o local de tudo o resto. Alterar o que um local
  aceita aplica-se aos ficheiros novos; **Mover os ficheiros guardados em conformidade** move o que já está guardado.
  **Remover** move primeiro os ficheiros de um local para os outros, por tipo, e remove-o quando ficar vazio. Cada
  ficheiro é verificado pelo seu hash antes de a cópia antiga ser eliminada. Um local cujo disco desapareceu pode ser
  removido mesmo assim, e os seus ficheiros passam a aparecer como em falta.
- **Uma pasta vigiada** é uma pasta de pacotes que já tem. Cada pasta dentro dela torna-se um pacote, indexado onde
  está. O Hoard nunca altera, move nem elimina nada lá. Monte-a só de leitura, se quiser. O Hoard percorre-a no arranque
  e a cada 15 minutos a partir daí (ou quando carregar em **Analisar agora**), encontrando pastas novas, ficheiros
  adicionados a um pacote ou removidos dele, e pacotes cuja pasta desapareceu. Uma pasta que de repente fica vazia ou
  inacessível é tomada por um disco que não está montado, e os seus pacotes são mantidos. Um pacote cuja pasta está em
  falta é mantido durante três dias, com as suas etiquetas e notas, para o caso de a pasta voltar. O que eliminar
  definitivamente de um pacote vigiado continua eliminado quando a pasta for analisada de novo, mesmo que os ficheiros
  ainda lá estejam.

Um local é qualquer pasta que o servidor consiga ver, por isso, no Docker, monte-a primeiro no contentor.

**Atualizar.** O cabeçalho mostra um distintivo quando existe uma versão mais recente. Execute
`docker compose pull && docker compose up -d` para a obter; o Hoard atualiza a base de dados ao arrancar, guardando
primeiro uma cópia em `/config/backups`. Altere a tag da imagem para `:1.3` para receber apenas versões de correção.
Para atualizar automaticamente sem reiniciar o Hoard a meio de um carregamento, indique em
**Definições → Manutenção → Atualizações** um endereço a chamar: o Hoard chama-o quando sai uma versão e não está nada
a decorrer (a API HTTP do Watchtower, um webhook do Portainer, um script; veja [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opcional).** O Hoard desenha as miniaturas dos modelos 3D na GPU quando consegue ver uma, e por software quando
não consegue. A página dos pacotes mostra qual (`GPU: ...` ou `No GPU found`). Para passar a GPU ao contentor,
adicione a sobreposição para o seu hardware:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Obtenha as duas sobreposições em [deploy/](../../deploy/). Também põem o servidor de modelos local (etiquetagem por IA) na GPU. Ficheiros num NAS, armazenamento S3, acesso remoto através do Cloudflare
ou do Tailscale, e cópias de segurança são tratados em [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

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

Ou largue o zip no portal.

## O resto deste README

O que acontece a um pacote, como levar recursos para um motor de jogo, pacotes do Unreal, pesquisa, créditos, MCP,
armazenamento, HTTPS, configuração, segurança, contas, a API e os factos e números são descritos no
[README em inglês](../../README.md).

## Traduções

O portal e as mensagens do servidor do Hoard estão disponíveis em 31 idiomas, escolhidos em Definições → Aspeto (por
predefinição, o idioma do navegador). As primeiras traduções foram feitas com ajuda de máquina; as correções de
falantes nativos são bem-vindas. O [docs/translations.md](../translations.md) explica como.

## Comunidade

Dúvidas, ideias ou algo para mostrar? Passe para dizer olá no [Discord da Azazel Labs](https://discord.gg/zf9ncS4EW). Erros e pedidos de funcionalidades ficam melhor como [issues no GitHub](https://github.com/Azazel-Labs/Hoard/issues), onde não se perdem.

## Licença

[MIT No Attribution](../../LICENSE) (MIT-0): use, copie, modifique e redistribua o Hoard para qualquer fim, sem ter de
manter o aviso de direitos de autor. Os pacotes de recursos que guardar no Hoard mantêm as suas próprias licenças.
