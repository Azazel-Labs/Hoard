# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)

[English](../../README.md) · [Afrikaans](README.af.md) · [Català](README.ca.md) · [Čeština](README.cs.md) · [Dansk](README.da.md) · [Deutsch](README.de.md) · [Ελληνικά](README.el.md) · [Español (Latinoamérica)](README.es-419.md) · [Español (España)](README.es-ES.md) · [Suomi](README.fi.md) · [Français](README.fr.md) · [Magyar](README.hu.md) · [Bahasa Indonesia](README.id.md) · [Italiano](README.it.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Norsk bokmål](README.nb.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md) · [Română](README.ro.md) · [Русский](README.ru.md) · [Српски](README.sr-Cyrl.md) · [Svenska](README.sv.md) · [ไทย](README.th.md) · Türkçe · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [简体中文](README.zh-Hans.md) · [繁體中文](README.zh-Hant.md)

Oyun geliştirme için kendi sunucunuzda barındırdığınız bir varlık kütüphanesi. Varlık paketlerini olduğu gibi bırakın
(zip’ler, tarball’lar, klasörler); Hoard her dosyayı karmasına göre bir kez saklar, her dosyanın ne olduğunu çıkarır,
onu etiketler, mesh’leri dokularına ve animasyonlarına bağlar ve hepsini bir web portalından, bir CLI’dan, ajanlar
için bir MCP uç noktasından ve oyun projenizin tam olarak ihtiyaç duyduğu dosyaları çeken, kilit dosyasına dayalı
`hoard sync` komutundan aranabilir kılar.

Onu ödeme adımı olmayan bir varlık mağazası gibi düşünün.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="Hoard kütüphanesi: bir ızgarada modeller ve malzemeler; yanda, etiketleri ve indirmeleriyle birlikte 3B önizlemede açık bir hazine sandığı" src="../img/hero-light.webp">
</picture>

<sub>Tanıtım kütüphanesi CC0 çizimlerden oluşur: [Poly Haven](https://polyhaven.com)’den modeller (glTF ve USD) ve
[ambientCG](https://ambientcg.com)’den yüzeyler. Bu resimleri ve sosyal medya kartını `node web/scripts/readme-images.mts` oluşturur.</sub>

## Docker ile kurulum

Hoard, amd64 ve arm64 için tek bir imaj olarak gelir: `ghcr.io/azazel-labs/hoard`. Bunu `compose.yml` olarak kaydedin
(bu dosya [deploy/compose.yml](../../deploy/compose.yml)):

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

Her birim bunun yerine kendisine uygun diskteki bir klasör olabilir: `/mnt/big/hoard:/library` dosyalarınızı büyük
diske koyar, veritabanı ise küçük ve yedeklemesi kolay kalır. Bağladığınız klasörler chown gerektirmez: Hoard
`/config` ve `/cache` klasörlerini sahiplenir ve `/library` kimin sahipliğindeyse onun olarak çalışır (bir kullanıcıyı
kendiniz seçmek için `PUID` ve `PGID` ayarlayın).

Ardından başlatın:

```sh
docker compose up -d        # open http://<host>:4627
```

İlk ziyaret sizden bir hesap oluşturmanızı ister, ardından kütüphanenizin nerede duracağını soran kısa bir kurulum açar. Yukarıdaki birim olan `/library` klasörünü önerir;
onu seçin ve portala bir paket bırakın. Daha fazla konum daha sonra **Ayarlar → Kütüphane konumları** bölümünden eklenir:

- **Hoard’un tuttuğu bir kütüphane**, yüklediğiniz ya da içe aktardığınız şeylerin kopyalarını alır. Her biri aldığı
  dosya türlerini işaretler (bir diskte ses, başka birinde modeller) ve biri geri kalanlar için genel konumdur. Bir
  konumun aldığı türleri değiştirmek yeni dosyalara uygulanır; **Saklanan dosyaları buna göre taşı** zaten saklananları
  taşır. **Kaldır**, bir konumun dosyalarını önce türlerine göre diğerlerine taşır ve konum boşalınca onu kaldırır. Eski
  kopya silinmeden önce her dosya karmasına göre denetlenir. Diski artık olmayan bir konum yine de kaldırılabilir; o
  zaman dosyaları kayıp olarak görünür.
- **İzlenen bir klasör**, zaten sahip olduğunuz bir paket klasörüdür. İçindeki her klasör, bulunduğu yerde dizinlenen
  bir paket olur. Hoard oradaki hiçbir şeyi asla değiştirmez, taşımaz ya da silmez. İsterseniz onu salt okunur bağlayın.
  Hoard başlangıçta ve sonra her 15 dakikada bir (ya da **Şimdi tara**’ya bastığınızda) ona bakar; yeni klasörleri, bir
  pakete eklenen ya da paketten kaldırılan dosyaları ve klasörü kaybolan paketleri algılar. Birden boşalan ya da
  erişilemeyen bir klasör bağlanmamış bir disk sayılır ve paketleri tutulur. Klasörü kayıp olan bir paket, klasör geri
  gelir diye etiketleri ve notlarıyla birlikte üç gün tutulur. İzlenen bir paketten kalıcı olarak sildiğiniz her şey,
  dosyaları hâlâ orada olsa bile, klasör yeniden tarandığında silinmiş olarak kalır.

Konum, sunucunun görebildiği herhangi bir klasördür; bu yüzden Docker’da onu önce konteynere bağlayın.

**Güncelleme.** Daha yeni bir sürüm olduğunda üst çubukta bir rozet görünür. Almak için `docker compose pull && docker compose up -d`
komutunu çalıştırın; Hoard başlarken veritabanını yükseltir, önce bir kopyasını `/config/backups` içine kaydeder. Yalnızca
yama sürümlerini almak için imaj etiketini `:0.8` olarak değiştirin. Hoard’u bir yüklemenin ortasında yeniden başlatmadan
otomatik olarak güncellemek için **Ayarlar → Bakım → Güncellemeler** bölümüne çağrılacak bir adres verin: bir sürüm
çıktığında ve hiçbir şey yapılmıyorken Hoard o adresi çağırır (Watchtower’ın HTTP API’si, bir Portainer webhook’u, bir
betik; bkz. [docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (isteğe bağlı).** Hoard, bir GPU görebildiğinde 3B model küçük resimlerini GPU’da, göremediğinde yazılımla çizer.
Paketler sayfası hangisi olduğunu gösterir (`GPU: ...` ya da `No GPU found`). GPU’yu konteynere geçirmek için
donanımınıza uygun ek dosyayı ekleyin:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Her iki ek dosyayı [deploy/](../../deploy/) klasöründen alın. Bunlar yerel model sunucusunu (yapay zekâ etiketleme) da GPU’ya taşır. NAS üzerindeki dosyalar, S3 depolama, Cloudflare
ya da Tailscale üzerinden uzaktan erişim ve yedeklemeler [docs/DEPLOYMENT.md](../DEPLOYMENT.md) içinde anlatılır.

## Kaynaktan derleme

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Ya da zip’i portala bırakın.

## Bu README’nin geri kalanı

Bir pakete ne olduğu, varlıkları bir motora aktarma, Unreal paketleri, arama, emeği geçenler, MCP, depolama, HTTPS,
yapılandırma, güvenlik, hesaplar ve API [İngilizce README](../../README.md)’de anlatılır.

## Çeviriler

Hoard’un portalı ve sunucu iletileri 31 dilde gelir; dil Ayarlar → Görünüm bölümünden seçilir (varsayılan olarak
tarayıcının dili). İlk çeviriler makine desteğiyle yapıldı; ana dili konuşanlardan gelen düzeltmeler memnuniyetle karşılanır. Nasıl yapılacağını [docs/translations.md](../translations.md) anlatır.

## Lisans

[MIT No Attribution](../../LICENSE) (MIT-0): Hoard’u herhangi bir amaçla kullanın, kopyalayın, değiştirin ve yeniden
dağıtın; telif hakkı bildirimini korumanız gerekmez. Hoard’da sakladığınız varlık paketleri kendi lisanslarını korur.
