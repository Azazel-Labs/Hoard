# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · Français · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Une bibliothèque de ressources auto-hébergée pour le développement de jeux. Déposez des packs de ressources entiers (zips, tarballs, dossiers), et Hoard
stocke chaque fichier une seule fois d’après son hash, détermine ce qu’est chaque fichier, l’étiquette, relie les maillages à leurs textures et
animations, et rend le tout consultable depuis un portail web, une CLI, un point d’accès MCP pour les agents, et un `hoard sync`
basé sur un fichier de verrouillage qui récupère exactement les fichiers dont votre projet de jeu a besoin.

Voyez-le comme une boutique de ressources sans passage en caisse.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="La bibliothèque Hoard : des modèles et des matériaux en grille, avec un coffre au trésor ouvert dans l’aperçu 3D à côté de ses étiquettes et téléchargements" src="../img/hero-light.webp">
</picture>

<sub>La bibliothèque de démonstration est faite d’œuvres CC0 : des modèles de [Poly Haven](https://polyhaven.com) (glTF et USD) et des surfaces de
[ambientCG](https://ambientcg.com). `node web/scripts/readme-images.mts` crée ces images et la carte pour les réseaux sociaux.</sub>

## Installer avec Docker

Hoard est distribué sous forme d’une seule image pour amd64 et arm64 : `ghcr.io/azazel-labs/hoard`. Enregistrez ceci sous `compose.yml` (c’est
[deploy/compose.yml](../../deploy/compose.yml)) :

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

Chaque volume peut plutôt être un dossier sur le disque qui lui convient : `/mnt/big/hoard:/library` place vos fichiers sur le gros
disque, tandis que la base de données reste petite et facile à sauvegarder. Les dossiers que vous montez n’ont pas besoin de chown : Hoard prend en charge
`/config` et `/cache` et s’exécute sous l’utilisateur propriétaire de `/library` (définissez `PUID` et `PGID` pour choisir vous-même un utilisateur).

Puis démarrez-le :

```sh
docker compose up -d        # open http://<host>:4627
```

La première visite vous demande de créer un compte, puis ouvre une courte configuration qui demande où se trouve votre bibliothèque. Elle propose `/library`, le volume ci-dessus ; choisissez-le
et déposez un pack sur le portail. D’autres emplacements s’ajoutent plus tard, depuis **Paramètres → Emplacements de la bibliothèque** :

- **Une bibliothèque gérée par Hoard** reçoit des copies de ce que vous envoyez ou importez. Chacune coche les types de fichiers qu’elle accepte
  (l’audio sur un disque, les modèles sur un autre), et l’une d’elles est l’emplacement par défaut pour le reste. Modifier ce qu’un emplacement accepte
  s’applique aux nouveaux fichiers ; **Déplacer les fichiers stockés en conséquence** déplace ce qui est déjà stocké. **Retirer** déplace d’abord les
  fichiers d’un emplacement vers les autres, par type, et le retire une fois vide. Chaque fichier est vérifié par rapport à son hash avant
  la suppression de l’ancienne copie. Un emplacement dont le disque a disparu peut être retiré malgré tout, et ses fichiers apparaissent alors comme manquants.
- **Un dossier surveillé** est un dossier de packs que vous avez déjà. Chaque dossier qu’il contient devient un pack, indexé là où il
  se trouve. Hoard n’y modifie, ne déplace ni ne supprime jamais rien. Montez-le en lecture seule si vous le souhaitez. Hoard le parcourt
  au démarrage puis toutes les 15 minutes (ou quand vous appuyez sur **Analyser maintenant**), et repère les nouveaux dossiers, les fichiers ajoutés à un pack ou
  retirés, et les packs dont le dossier a disparu. Un dossier soudain vide ou injoignable est considéré comme un
  disque non monté, et ses packs sont conservés. Un pack dont le dossier manque est conservé trois jours, avec ses
  étiquettes et ses notes, au cas où le dossier reviendrait. Tout ce que vous supprimez définitivement d’un pack surveillé reste supprimé quand
  le dossier est réanalysé, même si ses fichiers sont toujours là.

Un emplacement est n’importe quel dossier que le serveur peut voir : avec Docker, montez-le d’abord dans le conteneur.

**Mises à jour.** L’en-tête affiche un badge quand une version plus récente existe. Exécutez `docker compose pull && docker compose up -d`
pour l’installer ; Hoard met à niveau sa base de données au démarrage, après en avoir enregistré une copie dans `/config/backups`. Changez l’étiquette de l’image en
`:0.8` pour ne recevoir que les versions correctives. Pour mettre à jour automatiquement sans redémarrer Hoard en plein envoi, indiquez dans
**Paramètres → Maintenance → Mises à jour** une adresse à appeler : Hoard l’appelle dès qu’une version est sortie et que rien n’est en cours
(l’API HTTP de Watchtower, un webhook Portainer, un script ; voir [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (facultatif).** Hoard crée les miniatures des modèles 3D sur le GPU quand il en voit un, et en logiciel sinon.
La page des packs indique lequel (`GPU: ...` ou `No GPU found`). Pour donner accès au GPU, ajoutez la surcouche correspondant à votre
matériel :

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Les deux surcouches se trouvent dans [deploy/](../../deploy/). Elles placent aussi le serveur de modèles local (étiquetage par IA) sur le GPU. Les fichiers sur un NAS, le stockage S3, l’accès à distance via Cloudflare
ou Tailscale, et les sauvegardes sont traités dans [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Compiler depuis les sources

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Ou déposez le zip sur le portail.

## La suite de ce README

Ce qui arrive à un pack, l’intégration des ressources dans un moteur, les packs Unreal, la recherche, les crédits, MCP, le stockage, HTTPS,
la configuration, la sécurité, les comptes et l’API sont décrits dans le [README en anglais](../../README.md).

## Traductions

Les messages du portail et du serveur de Hoard existent en 31 langues, à choisir dans Paramètres → Apparence (la langue du navigateur par
défaut). Les premières traductions ont été faites avec l’aide de la machine ; les corrections de locuteurs natifs sont les bienvenues. [docs/translations.md](../translations.md) explique comment faire.

## Licence

[MIT No Attribution](../../LICENSE) (MIT-0) : utilisez, copiez, modifiez et redistribuez Hoard pour n’importe quel usage, sans avoir à
conserver la mention de copyright. Les packs de ressources que vous stockez dans Hoard gardent leurs propres licences.
