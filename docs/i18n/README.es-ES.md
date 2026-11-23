<div align="center">

# <img src="../../web/public/favicon.svg" alt="" height="32" align="absmiddle"> Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](../../LICENSE)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/zf9ncS4EW)

[English](../../README.md)&nbsp;· [Afrikaans](README.af.md)&nbsp;· [Català](README.ca.md)&nbsp;· [Čeština](README.cs.md)&nbsp;· [Dansk](README.da.md)&nbsp;· [Deutsch](README.de.md)&nbsp;· [Ελληνικά](README.el.md)&nbsp;· [Español&nbsp;(Latinoamérica)](README.es-419.md)&nbsp;· Español&nbsp;(España)&nbsp;· [Suomi](README.fi.md)&nbsp;· [Français](README.fr.md)&nbsp;· [Magyar](README.hu.md)&nbsp;· [Bahasa&nbsp;Indonesia](README.id.md)&nbsp;· [Italiano](README.it.md)&nbsp;· [日&#8288;本&#8288;語](README.ja.md)&nbsp;· [한&#8288;국&#8288;어](README.ko.md)&nbsp;· [Norsk&nbsp;bokmål](README.nb.md)&nbsp;· [Nederlands](README.nl.md)&nbsp;· [Polski](README.pl.md)&nbsp;· [Português&nbsp;(Brasil)](README.pt-BR.md)&nbsp;· [Português&nbsp;(Portugal)](README.pt-PT.md)&nbsp;· [Română](README.ro.md)&nbsp;· [Русский](README.ru.md)&nbsp;· [Српски](README.sr-Cyrl.md)&nbsp;· [Svenska](README.sv.md)&nbsp;· [ไ&#8288;ท&#8288;ย](README.th.md)&nbsp;· [Türkçe](README.tr.md)&nbsp;· [Українська](README.uk.md)&nbsp;· [Tiếng&nbsp;Việt](README.vi.md)&nbsp;· [简&#8288;体&#8288;中&#8288;文](README.zh-Hans.md)&nbsp;· [繁&#8288;體&#8288;中&#8288;文](README.zh-Hant.md)

</div>

<h3 align="center">Todos los paquetes de recursos que tienes, con búsqueda, vista previa y a un clic de tu juego.</h3>

Hoard es una biblioteca autoalojada de recursos para videojuegos. Añade un zip, un paquete de Unity, una descarga de Fab
o una carpeta con diez mil archivos, y Hoard se pone manos a la obra. Los modelos giran en 3D con sus texturas, los
mapas de textura sueltos se convierten en materiales, los sonidos dibujan sus formas de onda, las hojas de sprites
reproducen sus animaciones y todo queda etiquetado, así que basta con escribir `footsteps gravel` o `rigged low-poly`
para encontrarlo. Después llega a Godot, Unity o Unreal con todo lo que necesita.

Sin suscripción, sin subir nada a la nube de nadie, sin precios por puesto: una imagen de Docker en tu propio servidor,
o una aplicación en tu propio ordenador, y es tuya. Piensa en ella como una tienda de recursos sin caja.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/hero-dark.webp">
  <img alt="La biblioteca de Hoard: modelos 3D en una cuadrícula, con un cofre del tesoro abierto en la vista previa 3D junto a sus etiquetas y descargas" src="../img/hero-light.webp">
</picture>

### Míralo todo antes de usarlo

Los modelos se abren en 3D con sus texturas, materiales y animaciones. Hoard lee por sí mismo FBX, glTF, OBJ, USD,
SketchUp, 3MF, STL y vóxeles, sin ninguna aplicación 3D instalada; los archivos de Blender entran a través de un Blender
sin interfaz, y los paquetes de Unreal muestran lo que el editor guardó sobre ellos, sin Unreal. Las miniaturas se
dibujan en el servidor, encuadradas siempre del mismo modo, para que puedas repasar un paquete entero de un vistazo.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/page-dark.webp">
  <img alt="Una lámpara de araña a página completa: una vista 3D grande junto a sus datos, etiquetas, texturas vinculadas y descargas" src="../img/page-light.webp">
</picture>

### Los mapas de textura sueltos se convierten en materiales

`Rock_BaseColor.png`, `Rock_Normal.png` y `Rock_Roughness.png` se convierten en un único material Rock sobre una esfera
iluminada. Hoard entiende la nomenclatura de Megascans, Poly Haven, ambientCG, Substance y Unreal, incluidos los mapas
empaquetados, y también los archivos `.mat` de Unity. El visor a pantalla completa muestra un material sobre una esfera,
un cubo o un plano, a su escala real, y cada mapa se puede activar y desactivar.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/material-dark.webp">
  <img alt="Un material de ladrillo en el visor a pantalla completa, sobre una esfera, con interruptores para sus mapas de color, normales, rugosidad y altura" src="../img/material-light.webp">
</picture>

### Hojas de sprites que se reproducen

Los atlas de TexturePacker, Starling, libGDX, Cocos2d y Godot se convierten en una única hoja de sprites que reproduce
sus animaciones y enumera sus sprites. Descárgala reempaquetada para el motor que estés usando: diez formatos, y a cada
uno solo se le da lo que puede contener.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sheet-dark.webp">
  <img alt="La hoja de sprites de un personaje a página completa, con una de sus animaciones reproduciéndose junto a sus etiquetas y formatos de descarga" src="../img/sheet-light.webp">
</picture>

### Sonidos que puedes ver

Cada sonido recibe una forma de onda coloreada según lo brillante que suena, así que un golpe seco y un ambiente largo
se ven distintos antes de reproducirlos. Se leen los metadatos de las bibliotecas de sonido (categorías UCS, campos de
Soundminer y BaseHead, puntos de bucle, tempo, nota fundamental), y la música MIDI y de tracker se renderiza para que se
reproduzca como cualquier otro sonido.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/sound-dark.webp">
  <img alt="Una cuadrícula de efectos de sonido dibujados como formas de onda de colores, con uno abierto que muestra su reproductor, etiquetas y tomas" src="../img/sound-light.webp">
</picture>

### Una búsqueda que sabe qué es cada cosa

Hoard lo etiqueta todo por sí mismo, sin conexión: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0`, y una categoría de un árbol de recursos para videojuegos. Busca por palabras, por color o por lo que
muestra una imagen, y pide a cualquier textura o sonido «Más como este». Conéctalo a cualquier modelo compatible con
OpenAI, local o alojado, y también escribirá nombres y descripciones.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/search-dark.webp">
  <img alt="Una búsqueda de «wood»: una mesa de madera, corteza, tablones y una superficie de madera juntos, con filtros por tipo y etiqueta al lado" src="../img/search-light.webp">
</picture>

### Directamente a tu motor

- **Plugins para motores** Godot 4, Unity, Unreal Engine 5 y Lumina: explora la biblioteca desde el editor y pulsa Añadir al
  proyecto. El modelo llega con sus texturas, en el sitio correcto, ya importado.
- **`hoard sync`** trae a un proyecto exactamente los archivos que este indica, fijados en un archivo de bloqueo, para
  que cada compañero de equipo y cada máquina de compilación reciba los mismos bytes.
- **Descargar como** GLB, PNG, WAV, MP3 o un formato de hoja de sprites, convertido sobre la marcha, con nombres de
  textura según la convención de tu estudio.
- **Los agentes de IA** buscan, inspeccionan y obtienen recursos mediante MCP, y pueden hacer cualquier otra cosa
  que hace el portal, con los permisos de la cuenta o el token con el que inician sesión.

### Guardado una vez, a salvo

Cada archivo se guarda una sola vez por su hash BLAKE3, así que la misma textura en cinco paquetes ocupa el espacio de
una, y ninguna corrupción puede pasar desapercibida. Guárdalo en un disco local o en cualquier bucket de S3. Las
carpetas que le indicas a Hoard se leen, nunca se modifican. Cuentas, tokens de solo lectura, inicio de sesión único,
HTTPS y HTTP/3 vienen de serie.

### O simplemente en tu ordenador

¿No tienes un servidor? La aplicación de escritorio para Windows, macOS y Linux es la misma biblioteca en su propia
ventana. Instálala, elige una carpeta y funciona sin conexión: sin cuentas, contraseñas ni ajustes de servidor, nada a
lo que pueda acceder el resto de la red, y las claves que escribas se guardan en el llavero del sistema. Las versiones
nuevas se descargan en segundo plano y se instalan cuando la cierras.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../img/desktop-dark.webp">
  <img alt="La aplicación de escritorio de Hoard en una ventana de Windows: los modelos de la biblioteca en una cuadrícula, con un farol abierto en la vista previa 3D junto a sus etiquetas y descargas" src="../img/desktop-light.webp">
</picture>

<sub>La biblioteca de demostración es arte CC0: modelos de [Poly Haven](https://polyhaven.com) (glTF y USD), superficies
de [ambientCG](https://ambientcg.com) y sonidos, hojas de sprites y fuentes de [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` crea estas imágenes y la tarjeta para redes sociales.</sub>

## Instalación de la aplicación de escritorio

Descárgala desde la [última versión](https://github.com/Azazel-Labs/Hoard/releases/latest): el `-setup.exe` para
Windows, el `.dmg` para macOS en Apple silicon, el `.AppImage` o el `.deb` para Linux. En Windows se instala solo para
ti, sin pedir permisos de administrador. Los instaladores aún no están firmados por Apple ni por Microsoft, así que
Windows pregunta antes de ejecutar el instalador (Más información → Ejecutar de todas formas) y macOS antes de abrir la
aplicación por primera vez (Ajustes del Sistema → Privacidad y seguridad → Abrir igualmente).

La primera vez que se abre, pregunta dónde va la biblioteca. Todo lo demás está en Ajustes, como en un servidor.

A partir de ahí se mantiene al día ella sola, sin que tengas que pulsar nada: cada versión nueva se descarga en segundo
plano y se instala al cerrarla, así que la próxima vez que abras Hoard ya tendrás la versión nueva. Un `.deb` se
actualiza instalando el siguiente.

## Instalación con Docker

Hoard se distribuye como una sola imagen para amd64 y arm64: `ghcr.io/azazel-labs/hoard`. Guarda esto como
`compose.yml` (es [deploy/compose.yml](../../deploy/compose.yml)):

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

Cada volumen puede ser, en su lugar, una carpeta en el disco que más le convenga: `/mnt/big/hoard:/library` pone tus
archivos en el disco grande, mientras la base de datos sigue siendo pequeña y fácil de copiar. Las carpetas que
montes no necesitan chown: Hoard se hace cargo de `/config` y `/cache` y se ejecuta como el propietario de `/library`
(define `PUID` y `PGID` para elegir tú el usuario).

Después, inícialo:

```sh
docker compose up -d        # open http://<host>:4627
```

La primera visita te pide que crees una cuenta y luego abre una breve configuración que pregunta dónde está tu
biblioteca. Sugiere `/library`, el volumen de arriba; elígelo y suelta un paquete en el portal. Más adelante puedes
añadir más ubicaciones desde **Ajustes → Ubicaciones de la biblioteca**:

- **Una biblioteca que gestiona Hoard** recibe copias de lo que subes o importas. En cada una se marcan los tipos de
  archivo que acepta (el audio en un disco, los modelos en otro), y una es la de reserva para el resto. Cambiar lo que
  acepta una ubicación se aplica a los archivos nuevos; **Mover los archivos guardados para que coincidan** mueve lo
  que ya está guardado. **Quitar** mueve primero los archivos de una ubicación a las demás, por tipo, y la quita en
  cuanto queda vacía. Cada archivo se comprueba con su hash antes de eliminar la copia antigua. Una ubicación cuyo
  disco ya no está se puede quitar igualmente, y sus archivos aparecen entonces como no encontrados.
- **Una carpeta vigilada** es una carpeta con paquetes que ya tienes. Cada carpeta que contiene se convierte en un
  paquete, indexado donde está. Hoard nunca cambia, mueve ni elimina nada en ella. Puedes montarla de solo lectura si
  quieres. Hoard la recorre al arrancar y luego cada 15 minutos (o cuando pulsas **Escanear ahora**), y detecta
  carpetas nuevas, archivos añadidos a un paquete o quitados de él, y paquetes cuya carpeta ya no existe. Una carpeta
  que de repente está vacía o no es accesible se toma por un disco sin montar, y sus paquetes se conservan. Un paquete
  cuya carpeta falta se conserva durante tres días, con sus etiquetas y notas, por si la carpeta vuelve. Lo que
  elimines definitivamente de un paquete vigilado sigue eliminado cuando se vuelve a escanear la carpeta, aunque sus
  archivos sigan ahí.

Una ubicación es cualquier carpeta que el servidor pueda ver, así que en Docker, móntala antes en el contenedor.

**Actualizar.** La barra superior muestra un aviso cuando hay una versión más reciente. Ejecuta
`docker compose pull && docker compose up -d` para instalarla; Hoard actualiza su base de datos al arrancar, y antes
guarda una copia en `/config/backups`. Cambia la etiqueta de la imagen a `:1.3` para recibir solo versiones de
corrección. Para actualizar automáticamente sin reiniciar Hoard en mitad de una subida, indica en
**Ajustes → Mantenimiento → Actualizaciones** una dirección a la que llamar: Hoard la llama cuando sale una versión y
no hay nada en curso (la API HTTP de Watchtower, un webhook de Portainer, un script; consulta
[docs/DEPLOYMENT.md](../DEPLOYMENT.md#updating)).

**GPU (opcional).** Hoard renderiza las miniaturas de los modelos 3D en la GPU cuando ve una, y por software cuando no.
La página de paquetes muestra cuál (`GPU: ...` o `No GPU found`). Para pasar la GPU al contenedor, añade la
superposición de tu hardware:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Descarga las dos superposiciones de [deploy/](../../deploy/). También ponen en la GPU el servidor de modelos local
(etiquetado con IA). Los archivos en un NAS, el almacenamiento S3, el acceso remoto a través de Cloudflare o Tailscale
y las copias de seguridad se explican en [docs/DEPLOYMENT.md](../DEPLOYMENT.md).

## Compilar desde el código fuente

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

O suelta el zip en el portal.

## El resto de este README

Qué le pasa a un paquete, cómo llevar los recursos a un motor, los paquetes de Unreal, la búsqueda, los créditos, MCP,
el almacenamiento, HTTPS, la configuración, la seguridad, las cuentas, la API y los datos y cifras se describen en
el [README en inglés](../../README.md).

## Traducciones

Los mensajes del portal y del servidor de Hoard están disponibles en 31 idiomas, que se eligen en Ajustes → Apariencia
(por defecto, el idioma del navegador). Las primeras traducciones se hicieron con ayuda de traducción automática; las
correcciones de hablantes nativos son bienvenidas. [docs/translations.md](../translations.md) explica cómo.

## Comunidad

¿Preguntas, ideas o algo que enseñar? Pásate a saludar por el [Discord de Azazel Labs](https://discord.gg/zf9ncS4EW). Los errores y las peticiones de funciones, mejor como [issues de GitHub](https://github.com/Azazel-Labs/Hoard/issues), donde no se pierden.

## Licencia

[MIT No Attribution](../../LICENSE) (MIT-0): usa, copia, modifica y redistribuye Hoard para cualquier fin, sin
necesidad de conservar el aviso de copyright. Los paquetes de recursos que guardes en Hoard conservan sus propias
licencias.
