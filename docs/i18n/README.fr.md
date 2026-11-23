<div align="center">

# <img src="../../web/public/favicon.svg" alt="" height="32" align="absmiddle"> Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· [Español&nbsp;(España)](README.es-ES.md)&nbsp;· [Suomi](README.fi.md)&nbsp;· Français&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">Tous les packs de ressources que vous possédez, faciles à rechercher, à prévisualiser, et à un clic de votre jeu.</h3>

Hoard est une bibliothèque auto-hébergée pour les ressources de jeu. Déposez-y un zip, un package Unity, un
téléchargement Fab ou un dossier de dix mille fichiers, et Hoard se met au travail. Les modèles tournent en 3D avec
leurs textures, les maps de texture éparses deviennent des matériaux, les sons dessinent leur forme d’onde, les planches
de sprites jouent leurs animations, et tout reçoit des étiquettes : il suffit de taper `footsteps gravel` ou
`rigged low-poly` pour le trouver. Ensuite, la ressource arrive dans Godot, Unity ou Unreal avec tout ce qu’il lui faut.

Pas d’abonnement, pas d’envoi vers le cloud de qui que ce soit, pas de tarif par poste : une seule image Docker sur
votre propre serveur, ou une application sur votre propre ordinateur, et elle est à vous. Voyez-le comme une boutique de
ressources sans passage en caisse.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="La bibliothèque Hoard : des modèles 3D en grille, avec un coffre au trésor ouvert dans l’aperçu 3D à côté de ses étiquettes et téléchargements" src="../img/hero-light.webp">
</picture>

### Voyez tout avant de l’utiliser

Les modèles s’ouvrent en 3D avec leurs textures, matériaux et animations. Les formats FBX, glTF, OBJ, USD, SketchUp, 3MF
et STL ainsi que les voxels sont lus par Hoard lui-même, sans aucun logiciel 3D installé ; les fichiers Blender passent
par Blender sans interface (headless), et les packages Unreal montrent ce que l’éditeur a enregistré à leur sujet, sans
Unreal. Les miniatures sont dessinées sur le serveur, cadrées de la même façon à chaque fois, si bien qu’un pack entier
se parcourt d’un coup d’œil.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Un lustre affiché en pleine page : une grande vue 3D à côté de ses informations, étiquettes, textures liées et téléchargements" src="../img/page-light.webp">
</picture>

### Les maps de texture éparses deviennent des matériaux

`Rock_BaseColor.png`, `Rock_Normal.png` et `Rock_Roughness.png` deviennent un seul matériau Rock sur une sphère
éclairée. Hoard comprend les conventions de nommage de Megascans, Poly Haven, ambientCG, Substance et Unreal, maps
combinées comprises, ainsi que les fichiers `.mat` de Unity. La visionneuse plein écran montre un matériau sur une
sphère, un cube ou un plan, à son échelle réelle, en activant ou désactivant chaque map.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Un matériau de briques dans la visionneuse plein écran, sur une sphère, avec des interrupteurs pour ses maps de couleur, de normales, de rugosité et de hauteur" src="../img/material-light.webp">
</picture>

### Des planches de sprites qui s’animent

Les atlas de TexturePacker, Starling, libGDX, Cocos2d et Godot deviennent une seule planche de sprites qui joue ses
animations et liste ses sprites. Téléchargez-la réempaquetée pour le moteur que vous utilisez : dix formats, chacun ne
recevant que ce qu’il peut contenir.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="La planche de sprites d’un personnage en pleine page, l’une de ses animations en lecture à côté de ses étiquettes et formats de téléchargement" src="../img/sheet-light.webp">
</picture>

### Des sons qui se voient

Chaque son reçoit une forme d’onde colorée selon la brillance de son timbre, si bien qu’un impact sec et une longue
ambiance se distinguent avant même de les écouter. Les métadonnées des sonothèques sont lues (catégories UCS, champs
Soundminer et BaseHead, points de boucle, tempo, note fondamentale), et la musique MIDI et tracker est rendue pour se
jouer comme n’importe quel autre son.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Une grille d’effets sonores dessinés en formes d’onde colorées, l’un d’eux ouvert avec son lecteur, ses étiquettes et ses prises" src="../img/sound-light.webp">
</picture>

### Une recherche qui sait ce que sont les choses

Hoard étiquette tout lui-même, hors ligne : `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0`, et une catégorie tirée d’une arborescence de ressources de jeu. Cherchez par mots, par couleur ou par ce
que montre une image, et demandez à n’importe quelle texture ou n’importe quel son « Plus de ressources similaires ».
Branchez-le sur n’importe quel modèle compatible OpenAI, local ou hébergé, et il rédige aussi les noms et les
descriptions.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Une recherche de wood : une table en bois, de l’écorce, des planches et une surface en bois réunies, avec à côté des filtres par type et par étiquette" src="../img/search-light.webp">
</picture>

### Directement dans votre moteur

- **Des plugins de moteur** pour Godot 4, Unity, Unreal Engine 5 et Lumina : parcourez la bibliothèque dans l’éditeur et cliquez
  sur Ajouter au projet. Le modèle arrive avec ses textures, au bon endroit, importé.
- **`hoard sync`** récupère dans un projet exactement les fichiers qu’il désigne, épinglés dans un fichier de
  verrouillage, pour que chaque coéquipier et chaque machine de build obtienne les mêmes octets.
- **Télécharger en** GLB, PNG, WAV, MP3 ou dans un format de planche de sprites, converti à la volée, avec des noms de
  textures qui suivent la convention de votre studio.
- **Les agents IA** recherchent, inspectent et récupèrent des ressources via MCP.

### Stocké une fois, gardé en sécurité

Chaque fichier est stocké une seule fois d’après son hash BLAKE3 : la même texture présente dans cinq packs n’occupe que
la place d’une seule, et aucune corruption ne peut passer inaperçue. Gardez tout sur un disque local ou dans n’importe
quel bucket S3. Les dossiers que vous indiquez à Hoard sont lus, jamais modifiés. Comptes, jetons en lecture seule,
authentification unique, HTTPS et HTTP/3 sont intégrés.

### Ou simplement sur votre ordinateur

Pas de serveur ? L’application de bureau pour Windows, macOS et Linux, c’est la même bibliothèque dans sa propre
fenêtre. Installez-la, choisissez un dossier, et elle fonctionne hors ligne : pas de comptes, de mots de passe ni de
paramètres de serveur, rien que le reste du réseau puisse atteindre, et les clés que vous saisissez sont conservées dans
le trousseau du système. Les nouvelles versions se téléchargent en arrière-plan et s’installent quand vous la quittez.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="L’application de bureau Hoard dans une fenêtre Windows : les modèles de la bibliothèque en grille, avec une lanterne ouverte dans l’aperçu 3D à côté de ses étiquettes et téléchargements" src="../img/desktop-light.webp">
</picture>

<sub>La bibliothèque de démonstration est faite d’œuvres CC0 : des modèles de [Poly Haven](https://polyhaven.com) (glTF
et USD), des surfaces de [ambientCG](https://ambientcg.com), et des sons, planches de sprites et polices de
[Kenney](https://kenney.nl). `node web/scripts/readme-images.mts` crée ces images et la carte pour les réseaux
sociaux.</sub>

## Installer l’application de bureau

Téléchargez-la depuis la [dernière version](https://github.com/Azazel-Labs/Hoard/releases/latest) : le `-setup.exe` pour
Windows, le `.dmg` pour macOS sur Apple silicon, le `.AppImage` ou le `.deb` pour Linux. Sous Windows, elle s’installe
uniquement pour vous, sans demander de droits d’administrateur. Les installateurs ne sont pas encore signés par Apple ni
par Microsoft : Windows demande donc une confirmation avant d’exécuter le programme d’installation (Informations
complémentaires → Exécuter quand même) et macOS avant d’ouvrir l’application la première fois (Réglages Système →
Confidentialité et sécurité → Ouvrir quand même).

La première fois qu’elle s’ouvre, elle demande où mettre la bibliothèque. Tout le reste se trouve dans les Paramètres,
comme sur un serveur.

Ensuite, elle se tient à jour toute seule, sans rien avoir à cliquer : une nouvelle version se télécharge en
arrière-plan et s’installe quand vous quittez l’application, si bien qu’à la prochaine ouverture de Hoard, c’est déjà la
nouvelle version. Un `.deb` se met à jour en installant le suivant.

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
`:1.3` pour ne recevoir que les versions correctives. Pour mettre à jour automatiquement sans redémarrer Hoard en plein envoi, indiquez dans
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
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
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
la configuration, la sécurité, les comptes, l’API ainsi que les faits et chiffres sont décrits dans le
[README en anglais](../../README.md).

## Traductions

Les messages du portail et du serveur de Hoard existent en 31 langues, à choisir dans Paramètres → Apparence (la langue du navigateur par
défaut). Les premières traductions ont été faites avec l’aide de la machine ; les corrections de locuteurs natifs sont les bienvenues. [docs/translations.md](../translations.md) explique comment faire.

## Communauté

Des questions, des idées ou quelque chose à montrer ? Passez dire bonjour sur le [Discord d’Azazel Labs](https://discord.gg/zf9ncS4EW). Les bugs et les demandes de fonctionnalités ont leur place dans les [issues GitHub](https://github.com/Azazel-Labs/Hoard/issues), où ils ne se perdent pas.

## Licence

[MIT No Attribution](../../LICENSE) (MIT-0) : utilisez, copiez, modifiez et redistribuez Hoard pour n’importe quel usage, sans avoir à
conserver la mention de copyright. Les packs de ressources que vous stockez dans Hoard gardent leurs propres licences.
