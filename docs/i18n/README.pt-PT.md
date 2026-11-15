# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · Português (Portugal) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Uma biblioteca de recursos auto-alojada para o desenvolvimento de jogos. Largue pacotes de recursos inteiros (zips,
tarballs, pastas) e o Hoard guarda cada ficheiro uma única vez pelo seu hash, descobre o que é cada ficheiro, etiqueta-o,
liga as malhas às respetivas texturas e animações, e torna tudo pesquisável a partir de um portal web, de uma CLI, de
um endpoint MCP para agentes e de um `hoard sync` baseado num lockfile que traz exatamente os ficheiros de que o seu
projeto de jogo precisa.

Pense nele como uma loja de recursos sem caixa de pagamento.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="A biblioteca do Hoard: modelos e materiais numa grelha, com uma arca do tesouro aberta na pré-visualização 3D ao lado das suas etiquetas e transferências" src="../img/hero-light.webp">
</picture>

<sub>A biblioteca de demonstração é arte CC0: modelos do [Poly Haven](https://polyhaven.com) (glTF e USD) e superfícies do
[ambientCG](https://ambientcg.com). O `node web/scripts/readme-images.mts` cria estas imagens e o cartão para redes sociais.</sub>

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
primeiro uma cópia em `/config/backups`. Altere a tag da imagem para `:0.8` para receber apenas versões de correção.
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
armazenamento, HTTPS, configuração, segurança, contas e a API são descritos no [README em inglês](../../README.md).

## Traduções

O portal e as mensagens do servidor do Hoard estão disponíveis em 31 idiomas, escolhidos em Definições → Aspeto (por
predefinição, o idioma do navegador). As primeiras traduções foram feitas com ajuda de máquina; as correções de
falantes nativos são bem-vindas. O [docs/translations.md](../translations.md) explica como.

## Licença

[MIT No Attribution](../../LICENSE) (MIT-0): use, copie, modifique e redistribua o Hoard para qualquer fim, sem ter de
manter o aviso de direitos de autor. Os pacotes de recursos que guardar no Hoard mantêm as suas próprias licenças.
