# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · Português (Brasil) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Uma biblioteca de assets auto-hospedada para desenvolvimento de jogos. Solte pacotes de assets inteiros (zips, tarballs,
pastas) e o Hoard guarda cada arquivo uma única vez pelo hash, descobre o que cada arquivo é, marca-o com tags, liga as
malhas às texturas e animações delas e deixa tudo pesquisável por um portal web, uma CLI, um endpoint MCP para agentes
e um `hoard sync` baseado em lockfile que baixa exatamente os arquivos de que o seu projeto de jogo precisa.

Pense nele como uma loja de assets sem caixa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="A biblioteca do Hoard: modelos e materiais em uma grade, com um baú do tesouro aberto na prévia 3D ao lado das tags e dos downloads dele" src="../img/hero-light.webp">
</picture>

<sub>A biblioteca de demonstração é arte CC0: modelos do [Poly Haven](https://polyhaven.com) (glTF e USD) e superfícies do
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` gera estas imagens e o cartão social.</sub>

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
da imagem para `:1.2` para receber só versões de correção. Para atualizar automaticamente sem reiniciar o Hoard no meio
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
armazenamento, HTTPS, configuração, segurança, contas e a API estão descritos no [README em inglês](../../README.md).

## Traduções

O portal e as mensagens do servidor do Hoard estão disponíveis em 31 idiomas, escolhidos em Configurações → Aparência
(por padrão, o idioma do navegador). As primeiras traduções foram feitas com ajuda de máquina; correções de falantes
nativos são bem-vindas. O [docs/translations.md](../translations.md) explica como.

## Licença

[MIT No Attribution](../../LICENSE) (MIT-0): use, copie, modifique e redistribua o Hoard para qualquer finalidade, sem
precisar manter o aviso de copyright. Os pacotes de assets que você guarda no Hoard mantêm as próprias licenças.
