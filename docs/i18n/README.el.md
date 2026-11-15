# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · Ελληνικά · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Μια αυτοφιλοξενούμενη βιβλιοθήκη στοιχείων για την ανάπτυξη παιχνιδιών. Ρίξτε μέσα ολόκληρα πακέτα στοιχείων (zip,
tarball, φακέλους) και το Hoard αποθηκεύει κάθε αρχείο μία φορά με βάση το hash του, καταλαβαίνει τι είναι κάθε
αρχείο, του βάζει ετικέτες, συνδέει τα πλέγματα με τις υφές και τα animations τους και κάνει τα πάντα αναζητήσιμα από
μια διαδικτυακή πύλη, ένα CLI, ένα endpoint MCP για πράκτορες AI και ένα `hoard sync` βασισμένο σε lockfile, που
φέρνει ακριβώς τα αρχεία που χρειάζεται το έργο του παιχνιδιού σας.

Σκεφτείτε το σαν ένα κατάστημα στοιχείων χωρίς ταμείο.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Η βιβλιοθήκη του Hoard: μοντέλα και υλικά σε πλέγμα, με ένα ανοιχτό σεντούκι θησαυρού στην προεπισκόπηση 3D δίπλα στις ετικέτες και τις λήψεις του" src="../img/hero-light.webp">
</picture>

<sub>Η βιβλιοθήκη επίδειξης είναι έργα CC0: μοντέλα από το [Poly Haven](https://polyhaven.com) (glTF και USD) και
επιφάνειες από το [ambientCG](https://ambientcg.com). Το `node web/scripts/readme-images.mts` δημιουργεί αυτές τις
εικόνες και την κάρτα κοινωνικών δικτύων.</sub>

## Εγκατάσταση με Docker

Το Hoard διατίθεται ως μία εικόνα για amd64 και arm64: `ghcr.io/azazel-labs/hoard`. Αποθηκεύστε το παρακάτω ως
`compose.yml` (είναι το [deploy/compose.yml](../../deploy/compose.yml)):

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

Κάθε volume μπορεί αντί γι' αυτό να είναι ένας φάκελος σε όποιον δίσκο του ταιριάζει: το `/mnt/big/hoard:/library`
βάζει τα αρχεία σας στον μεγάλο δίσκο, ενώ η βάση δεδομένων μένει μικρή και εύκολη στη δημιουργία αντιγράφων
ασφαλείας. Οι φάκελοι που προσαρτάτε δεν χρειάζονται chown: το Hoard αναλαμβάνει τα `/config` και `/cache` και
εκτελείται ως ο κάτοχος του `/library` (ορίστε τα `PUID` και `PGID` για να επιλέξετε χρήστη μόνοι σας).

Έπειτα ξεκινήστε το:

```sh
docker compose up -d        # open http://<host>:4627
```

Η πρώτη επίσκεψη σας ζητά να δημιουργήσετε λογαριασμό και μετά ανοίγει μια σύντομη αρχική ρύθμιση που ρωτά πού θα
βρίσκεται η βιβλιοθήκη σας. Προτείνει το `/library`, το παραπάνω volume· επιλέξτε το και ρίξτε ένα πακέτο στην πύλη.
Περισσότερες τοποθεσίες προστίθενται αργότερα, από τις **Ρυθμίσεις → Τοποθεσίες βιβλιοθήκης**:

- **Μια βιβλιοθήκη που φυλάει το Hoard** παίρνει αντίγραφα όσων μεταφορτώνετε ή εισάγετε. Στην καθεμία τσεκάρετε τους
  τύπους αρχείων που δέχεται (ήχος σε έναν δίσκο, μοντέλα σε άλλον), και μία είναι η τοποθεσία για όλα τα υπόλοιπα.
  Η αλλαγή του τι δέχεται μια τοποθεσία ισχύει για τα νέα αρχεία· η **Μετακίνηση αποθηκευμένων αρχείων ώστε να
  ταιριάζουν** μετακινεί όσα είναι ήδη αποθηκευμένα. Η **Κατάργηση** μετακινεί πρώτα τα αρχεία μιας τοποθεσίας στις
  άλλες, ανά τύπο, και την καταργεί μόλις αδειάσει. Κάθε αρχείο ελέγχεται με βάση το hash του πριν διαγραφεί το παλιό
  αντίγραφο. Μια τοποθεσία της οποίας ο δίσκος έχει χαθεί μπορεί να καταργηθεί ούτως ή άλλως, και τότε τα αρχεία της
  εμφανίζονται ως ελλείποντα.
- **Ένας παρακολουθούμενος φάκελος** είναι ένας φάκελος με πακέτα που έχετε ήδη. Κάθε φάκελος μέσα του γίνεται ένα
  πακέτο, που ευρετηριάζεται εκεί όπου βρίσκεται. Το Hoard δεν αλλάζει, δεν μετακινεί και δεν διαγράφει ποτέ τίποτα
  εκεί. Αν θέλετε, προσαρτήστε τον μόνο για ανάγνωση. Το Hoard τον εξετάζει κατά την εκκίνηση και κάθε 15 λεπτά μετά
  (ή όταν πατάτε **Σάρωση τώρα**), εντοπίζοντας νέους φακέλους, αρχεία που προστέθηκαν σε ένα πακέτο ή αφαιρέθηκαν από
  αυτό, και πακέτα των οποίων ο φάκελος έχει χαθεί. Ένας φάκελος που ξαφνικά είναι άδειος ή απρόσιτος θεωρείται δίσκος
  που δεν έχει προσαρτηθεί, και τα πακέτα του διατηρούνται. Ένα πακέτο του οποίου ο φάκελος λείπει διατηρείται για
  τρεις ημέρες, με τις ετικέτες και τις σημειώσεις του, σε περίπτωση που ο φάκελος επιστρέψει. Ό,τι διαγράφετε
  οριστικά από ένα παρακολουθούμενο πακέτο μένει διαγραμμένο όταν ο φάκελος σαρωθεί ξανά, παρόλο που τα αρχεία του
  βρίσκονται ακόμη εκεί.

Τοποθεσία είναι οποιοσδήποτε φάκελος βλέπει ο διακομιστής, οπότε στο Docker προσαρτήστε τον πρώτα στο container.

**Ενημέρωση.** Η κεφαλίδα εμφανίζει ένα σήμα όταν υπάρχει νεότερη έκδοση. Εκτελέστε `docker compose pull && docker compose up -d`
για να την πάρετε· το Hoard αναβαθμίζει τη βάση δεδομένων του κατά την εκκίνηση, αφού πρώτα αποθηκεύσει ένα αντίγραφο
στο `/config/backups`. Αλλάξτε την ετικέτα της εικόνας σε `:0.8` για να παίρνετε μόνο διορθωτικές εκδόσεις. Για
αυτόματη ενημέρωση χωρίς να επανεκκινείται το Hoard στη μέση μιας μεταφόρτωσης, δώστε στις
**Ρυθμίσεις → Συντήρηση → Ενημερώσεις** μια διεύθυνση για κλήση: το Hoard την καλεί μόλις κυκλοφορήσει μια έκδοση και
δεν γίνεται τίποτα (το HTTP API του Watchtower, ένα webhook του Portainer, ένα script· δείτε το
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (προαιρετικά).** Το Hoard δημιουργεί τις μικρογραφίες των μοντέλων 3D στη GPU όταν βλέπει μία και με λογισμικό
όταν δεν βλέπει. Η σελίδα των πακέτων δείχνει ποιο από τα δύο (`GPU: ...` ή `No GPU found`). Για να περάσετε τη GPU
στο container, προσθέστε το overlay για το υλικό σας:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Πάρτε και τα δύο overlays από το [deploy/](../../deploy/). Βάζουν επίσης στη GPU τον τοπικό διακομιστή μοντέλων (Ετικέτες με AI). Τα αρχεία σε NAS, ο χώρος αποθήκευσης S3, η απομακρυσμένη πρόσβαση μέσω Cloudflare
ή Tailscale και τα αντίγραφα ασφαλείας περιγράφονται στο [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Δημιουργία από τον πηγαίο κώδικα

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Ή ρίξτε το zip στην πύλη.

## Το υπόλοιπο αυτού του README

Τι συμβαίνει σε ένα πακέτο, πώς φέρνετε στοιχεία σε μια μηχανή παιχνιδιών, τα πακέτα Unreal, η αναζήτηση, τα credits,
το MCP, η αποθήκευση, το HTTPS, η διαμόρφωση, η ασφάλεια, οι λογαριασμοί και το API περιγράφονται στο
[αγγλικό README](../../README.md).

## Μεταφράσεις

Η πύλη και τα μηνύματα του διακομιστή του Hoard διατίθενται σε 31 γλώσσες, που επιλέγονται στις Ρυθμίσεις → Εμφάνιση
(από προεπιλογή, η γλώσσα του προγράμματος περιήγησης). Οι πρώτες μεταφράσεις έγιναν με τη βοήθεια μηχανής· οι
διορθώσεις από φυσικούς ομιλητές είναι ευπρόσδεκτες. Το [docs/translations.md](../translations.md) εξηγεί πώς.

## Άδεια χρήσης

[MIT No Attribution](../../LICENSE) (MIT-0): χρησιμοποιήστε, αντιγράψτε, τροποποιήστε και αναδιανείμετε το Hoard για
οποιονδήποτε σκοπό, χωρίς να χρειάζεται να διατηρήσετε τη σημείωση πνευματικών δικαιωμάτων. Τα πακέτα στοιχείων που
αποθηκεύετε στο Hoard διατηρούν τις δικές τους άδειες.
