# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · Suomi · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Itse ylläpidettävä resurssikirjasto pelinkehitykseen. Pudota sisään kokonaisia resurssipaketteja (zipit, tar-paketit,
kansiot), ja Hoard tallentaa jokaisen tiedoston kerran sen tiivisteen perusteella, selvittää, mikä kukin tiedosto on,
merkitsee sen tunnisteilla, linkittää verkot niiden tekstuureihin ja animaatioihin ja tekee kaikesta haettavaa
verkkoportaalista, komentoriviltä, agenteille tarkoitetusta MCP-päätepisteestä sekä lukitustiedostoon perustuvalla
`hoard sync` -komennolla, joka hakee juuri ne tiedostot, joita peliprojektisi tarvitsee.

Ajattele sitä resurssikauppana ilman kassaa.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard-kirjasto: malleja ja materiaaleja ruudukossa, ja aarrearkku avoinna 3D-esikatselussa tunnisteidensa ja latausten vieressä" src="../img/hero-light.webp">
</picture>

<sub>Esittelykirjaston grafiikka on CC0-lisensoitua: mallit [Poly Havenista](https://polyhaven.com) (glTF ja USD) ja pinnat
[ambientCG:stä](https://ambientcg.com). `node web/scripts/readme-images.mts` tekee nämä kuvat ja some-kortin.</sub>

## Asennus Dockerilla

Hoard toimitetaan yhtenä levykuvana amd64- ja arm64-alustoille: `ghcr.io/azazel-labs/hoard`. Tallenna tämä nimellä
`compose.yml` (se on [deploy/compose.yml](../../deploy/compose.yml)):

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

Kukin taltio voi sen sijaan olla kansio sille parhaiten sopivalla levyllä: `/mnt/big/hoard:/library` laittaa tiedostosi
isolle levylle, kun taas tietokanta pysyy pienenä ja helposti varmuuskopioitavana. Liittämiäsi kansioita ei tarvitse
chownata: Hoard ottaa haltuunsa kansiot `/config` ja `/cache` ja toimii sen käyttäjän oikeuksin, joka omistaa kansion
`/library` (aseta `PUID` ja `PGID`, jos haluat valita käyttäjän itse).

Käynnistä se sitten:

```sh
docker compose up -d        # open http://<host>:4627
```

Ensimmäisellä käynnillä sinua pyydetään luomaan tili, minkä jälkeen avautuu lyhyt käyttöönotto, joka kysyy, missä kirjastosi on. Se ehdottaa
kansiota `/library`, yllä olevaa taltiota; valitse se ja pudota paketti portaaliin. Lisää sijainteja voi lisätä myöhemmin
kohdassa **Asetukset → Kirjaston sijainnit**:

- **Hoardin ylläpitämä kirjasto** saa kopiot siitä, mitä lähetät tai tuot. Kustakin valitaan, millaisia tiedostoja se
  ottaa vastaan (äänet yhdelle levylle, mallit toiselle), ja yksi ottaa kaiken muun. Sijainnin vastaanottamien
  tyyppien muutos koskee uusia tiedostoja; **Siirrä tallennetut tiedostot vastaavasti** siirtää jo tallennetut.
  **Poista** siirtää ensin sijainnin tiedostot muihin tyypin mukaan ja poistaa sijainnin, kun se on tyhjä. Jokainen
  tiedosto tarkistetaan tiivistettään vasten ennen kuin vanha kopio poistetaan. Sijainnin, jonka levy on poissa, voi
  poistaa silti, ja sen tiedostot näkyvät sitten puuttuvina.
- **Valvottu kansio** on kansio, jossa on jo omistamiasi paketteja. Jokaisesta sen sisällä olevasta kansiosta tulee
  paketti, joka indeksoidaan paikoillaan. Hoard ei koskaan muuta, siirrä tai poista siellä mitään. Voit liittää sen
  vain luku -tilassa, jos haluat. Hoard käy sen läpi käynnistyessään ja sen jälkeen 15 minuutin välein (tai kun
  painat **Tarkista nyt**) ja huomaa uudet kansiot, pakettiin lisätyt tai siitä poistetut tiedostot ja paketit, joiden
  kansio on poissa. Yhtäkkiä tyhjentynyt tai tavoittamaton kansio tulkitaan levyksi, jota ei ole liitetty, ja sen
  paketit säilytetään. Paketti, jonka kansio puuttuu, säilytetään tunnisteineen ja muistiinpanoineen kolme päivää
  siltä varalta, että kansio palaa. Mikä tahansa, minkä poistat valvotusta paketista lopullisesti, pysyy poistettuna,
  kun kansio tarkistetaan uudelleen, vaikka sen tiedostot ovat yhä siellä.

Sijainti on mikä tahansa kansio, jonka palvelin näkee, joten Dockerissa liitä se ensin konttiin.

**Päivittäminen.** Yläpalkissa näkyy merkki, kun uudempi versio on olemassa. Aja `docker compose pull && docker compose up -d`
ottaaksesi sen käyttöön; Hoard päivittää tietokantansa käynnistyessään ja tallentaa ensin kopion kansioon
`/config/backups`. Vaihda levykuvan tunnisteeksi `:0.8`, jos haluat vain korjausversiot. Päivittääksesi automaattisesti
käynnistämättä Hoardia uudelleen kesken lähetyksen anna kohtaan **Asetukset → Ylläpito → Päivitykset** kutsuttava
osoite: Hoard kutsuu sitä, kun versio on julkaistu eikä mitään ole käynnissä (Watchtowerin HTTP API, Portainer-webhook,
skripti; katso [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**Näytönohjain (valinnainen).** Hoard renderöi 3D-mallien pikkukuvat näytönohjaimella, kun se näkee sellaisen, ja
ohjelmallisesti, kun ei näe. Pakettisivu näyttää kumpi (`GPU: ...` tai `No GPU found`). Välittääksesi näytönohjaimen
konttiin lisää laitteistollesi sopiva lisätiedosto:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Molemmat lisätiedostot löytyvät hakemistosta [deploy/](../../deploy/). Ne laittavat myös paikallisen mallipalvelimen (tekoälytunnisteet) näytönohjaimelle. Tiedostot NAS-laitteella, S3-tallennus, etäkäyttö Cloudflaren
tai Tailscalen kautta ja varmuuskopiot käsitellään tiedostossa [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Kääntäminen lähdekoodista

```sh
# Server (Rust 1.99.1 is pinned in rust-toolchain.toml; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Tai pudota zip portaaliin.

## Tämän README:n loppuosa

Mitä paketille tapahtuu, resurssien vienti pelimoottoriin, Unreal-paketit, haku, tekijätiedot, MCP, tallennus, HTTPS,
asetukset, tietoturva, tilit ja API kuvataan [englanninkielisessä README:ssä](../../README.md).

## Käännökset

Hoardin portaali ja palvelimen viestit ovat saatavilla 31 kielellä, ja kielen voi valita kohdassa Asetukset → Ulkoasu
(oletuksena selaimen kieli). Ensimmäiset käännökset tehtiin konekäännöksen avulla; äidinkielisten korjaukset ovat
tervetulleita. [docs/translations.md](../translations.md) kertoo, miten.

## Lisenssi

[MIT No Attribution](../../LICENSE) (MIT-0): käytä, kopioi, muokkaa ja levitä Hoardia mihin tahansa tarkoitukseen ilman
tarvetta säilyttää tekijänoikeusilmoitusta. Hoardiin tallentamasi resurssipaketit säilyttävät omat lisenssinsä.
