"""Make the desktop app's icons from the portal's favicon.

    python tools/desktop_icons.py

Needs the `resvg` command (`cargo install resvg`). Writes crates/hoard-desktop/icons/: PNGs for Linux and the
bundler, icon.ico for Windows, and icon.icns for macOS (drawn smaller on its canvas, as macOS icons are).
"""

import os
import re
import struct
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SOURCE = os.path.join(ROOT, 'web', 'public', 'favicon.svg')
OUT = os.path.join(ROOT, 'crates', 'hoard-desktop', 'icons')

PNGS = {'32x32.png': 32, '128x128.png': 128, '128x128@2x.png': 256, 'icon.png': 512}
ICO_SIZES = [16, 24, 32, 48, 64, 256]
# ICNS types holding PNG data, by pixel size.
ICNS_TYPES = [(b'icp4', 16), (b'icp5', 32), (b'ic11', 32), (b'icp6', 64), (b'ic12', 64), (b'ic07', 128),
              (b'ic08', 256), (b'ic13', 256), (b'ic09', 512), (b'ic14', 512), (b'ic10', 1024)]


def render(svg_path, size, tmp):
    out = os.path.join(tmp, f'{os.path.basename(svg_path)}-{size}.png')
    if not os.path.exists(out):
        subprocess.run(['resvg', '-w', str(size), '-h', str(size), svg_path, out], check=True)
    with open(out, 'rb') as f:
        return f.read()


def ico(pngs):
    """An ICO holding each size as PNG data (Windows Vista and later read these)."""
    header = struct.pack('<HHH', 0, 1, len(pngs))
    offset = 6 + 16 * len(pngs)
    entries, data = b'', b''
    for size, png in pngs:
        dim = 0 if size >= 256 else size
        entries += struct.pack('<BBBBHHII', dim, dim, 0, 0, 1, 32, len(png), offset + len(data))
        data += png
    return header + entries + data


def icns(chunks):
    body = b''.join(kind + struct.pack('>I', 8 + len(png)) + png for kind, png in chunks)
    return b'icns' + struct.pack('>I', 8 + len(body)) + body


def main():
    with open(SOURCE, encoding='utf-8') as f:
        svg = f.read()
    m = re.search(r'viewBox="0 0 (\d+) (\d+)"', svg)
    w = int(m.group(1))
    # macOS icons sit inside a margin: the artwork is 824 of 1024 pixels across.
    pad = w * 100 / 1024
    mac = svg.replace(m.group(0), f'viewBox="{-pad:g} {-pad:g} {w + 2 * pad:g} {w + 2 * pad:g}"', 1)
    os.makedirs(OUT, exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        mac_path = os.path.join(tmp, 'mac.svg')
        with open(mac_path, 'w', encoding='utf-8') as f:
            f.write(mac)
        for name, size in PNGS.items():
            with open(os.path.join(OUT, name), 'wb') as f:
                f.write(render(SOURCE, size, tmp))
        with open(os.path.join(OUT, 'icon.ico'), 'wb') as f:
            f.write(ico([(s, render(SOURCE, s, tmp)) for s in ICO_SIZES]))
        with open(os.path.join(OUT, 'icon.icns'), 'wb') as f:
            f.write(icns([(kind, render(mac_path, s, tmp)) for kind, s in ICNS_TYPES]))
    print(f'wrote {OUT}')


if __name__ == '__main__':
    sys.exit(main())
