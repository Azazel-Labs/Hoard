#!/usr/bin/env python3
"""Generate a small fake asset pack (textures, sprites, audio, models, license) for demos and tests.

Usage: python3 tools/make_sample_pack.py OUT_DIR
Produces OUT_DIR/forest-starter-kit.zip. Needs Pillow + numpy; uses ffmpeg for .ogg if available.
"""

import base64
import io
import json
import math
import shutil
import struct
import subprocess
import sys
import wave
import zipfile
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

RNG = np.random.default_rng(7)


def tileable_noise(size: int, octaves: int = 5) -> np.ndarray:
    """Sum of periodic sines -> wraps seamlessly on both axes."""
    y, x = np.mgrid[0:size, 0:size] / size * 2 * math.pi
    out = np.zeros((size, size))
    for o in range(octaves):
        f = 2 ** o
        for _ in range(3):
            ax, ay, ph = RNG.integers(1, 4) * f, RNG.integers(1, 4) * f, RNG.random() * 6.28
            out += np.sin(ax * x + ay * y + ph) / f
    out -= out.min()
    return out / out.max()


def save_rgb(path: Path, arr: np.ndarray) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    Image.fromarray(np.clip(arr * 255, 0, 255).astype(np.uint8)).save(path)


def textures(root: Path) -> None:
    n = tileable_noise(512)
    rock = np.stack([0.45 + 0.3 * n, 0.42 + 0.28 * n, 0.38 + 0.25 * n], -1)
    save_rgb(root / "Textures/Rock/Rock_Cliff_Albedo.png", rock)
    gx, gy = np.gradient(n)
    normal = np.stack([0.5 + gx * 8, 0.5 + gy * 8, np.full_like(n, 0.95)], -1)
    save_rgb(root / "Textures/Rock/Rock_Cliff_Normal.png", normal)
    save_rgb(root / "Textures/Rock/Rock_Cliff_Roughness.png", np.stack([0.6 + 0.3 * n] * 3, -1))
    save_rgb(root / "Textures/Rock/Rock_Cliff_AO.png", np.stack([0.7 + 0.3 * n] * 3, -1))
    g = tileable_noise(256)
    save_rgb(root / "Textures/Grass_Diffuse.png", np.stack([0.15 + 0.2 * g, 0.45 + 0.35 * g, 0.1 + 0.1 * g], -1))
    # Not tileable: a gradient sky.
    yy = np.linspace(0, 1, 256)[:, None, None]
    sky = np.concatenate([0.3 + 0.5 * yy, 0.5 + 0.4 * yy, np.ones_like(yy)], -1) * np.ones((256, 512, 1))
    save_rgb(root / "Textures/Sky/sunset_sky_gradient.png", sky)


def sprites(root: Path) -> None:
    palette = np.array([[0, 0, 0, 0], [40, 30, 60, 255], [220, 60, 50, 255], [250, 210, 160, 255], [60, 90, 200, 255]],
                       dtype=np.uint8)
    frames = []
    for f in range(6):
        idx = np.zeros((16, 16), dtype=np.uint8)
        idx[2:6, 5:11] = 3  # head
        idx[6:12, 4:12] = 2  # body
        leg = 1 if f % 2 else 0
        idx[12:16, 4 + leg:7 + leg] = 4
        idx[12:16, 9 - leg:12 - leg] = 4
        idx[1, 5:11] = 1
        frames.append(palette[idx])
    strip = np.concatenate(frames, axis=1)
    p = root / "Sprites/Characters/hero_run_strip.png"
    p.parent.mkdir(parents=True, exist_ok=True)
    Image.fromarray(strip, "RGBA").save(p)

    coin = np.zeros((64, 64, 4), dtype=np.uint8)
    yy, xx = np.mgrid[0:64, 0:64]
    mask = (xx - 32) ** 2 + (yy - 32) ** 2 < 26 ** 2
    coin[mask] = [245, 200, 40, 255]
    coin[((xx - 32) ** 2 + (yy - 32) ** 2 < 18 ** 2) & mask] = [255, 225, 90, 255]
    p = root / "UI/Icons/icon_coin.png"
    p.parent.mkdir(parents=True, exist_ok=True)
    Image.fromarray(coin, "RGBA").save(p)

    btn = np.zeros((48, 160, 4), dtype=np.uint8)
    btn[4:44, 4:156] = [50, 110, 230, 255]
    btn[4:10, 4:156] = [110, 160, 250, 255]
    Image.fromarray(btn, "RGBA").save(root / "UI/button_blue.png")


def write_wav(path: Path, samples: np.ndarray, rate: int = 44100, channels: int = 1) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    data = (np.clip(samples, -1, 1) * 32767).astype("<i2")
    with wave.open(str(path), "wb") as w:
        w.setnchannels(channels)
        w.setsampwidth(2)
        w.setframerate(rate)
        w.writeframes(data.tobytes())


def library_wav(path: Path, samples: np.ndarray, title: str, desc: str, keywords: str, cat: str, sub: str,
                loop: bool = False, rate: int = 44100) -> None:
    """A WAV the way commercial SFX libraries ship them: BWF bext, LIST/INFO, iXML with UCS fields, smpl, acid."""
    path.parent.mkdir(parents=True, exist_ok=True)
    data = (np.clip(samples, -1, 1) * 32767).astype("<i2").tobytes()

    def chunk(cid: bytes, body: bytes) -> bytes:
        pad = b"\0" if len(body) % 2 else b""
        return cid + struct.pack("<I", len(body)) + body + pad

    fmt = struct.pack("<HHIIHH", 1, 1, rate, rate * 2, 2, 16)
    bext = desc.encode()[:256].ljust(256, b"\0") + b"Hoard sample".ljust(32, b"\0") + b"".ljust(32, b"\0")
    bext += b"2026-11-11" + b"12:00:00" + bytes(8) + struct.pack("<H", 1) + bytes(64 + 190)
    info = b"INFO" + chunk(b"INAM", title.encode() + b"\0") + chunk(b"IKEY", keywords.encode() + b"\0")
    ixml = (f"<?xml version=\"1.0\"?><BWFXML><USER><CATEGORY>{cat}</CATEGORY><SUBCATEGORY>{sub}</SUBCATEGORY>"
            f"<CATID>{cat[:3]}{sub[:4]}</CATID><LIBRARY>Hoard Sample SFX</LIBRARY>"
            f"<KEYWORDS>{keywords}</KEYWORDS></USER></BWFXML>").encode()
    body = chunk(b"fmt ", fmt) + chunk(b"bext", bext) + chunk(b"LIST", info) + chunk(b"iXML", ixml)
    frames = len(data) // 2
    if loop:
        smpl = struct.pack("<9I", 0, 0, 0, 60, 0, 0, 0, 1, 0) + struct.pack("<6I", 0, 0, 0, frames - 1, 0, 0)
        body += chunk(b"smpl", smpl)
        body += chunk(b"acid", struct.pack("<IHHfIHHf", 0x02, 57, 0x8000, 0.0, 8, 4, 4, 96.0))
    body += chunk(b"data", data)
    path.write_bytes(b"RIFF" + struct.pack("<I", 4 + len(body)) + b"WAVE" + body)


def audio(root: Path) -> None:
    r = 44100
    t = np.arange(int(r * 1.4)) / r
    boom = RNG.normal(0, 1, t.size) * np.exp(-t * 3.5) + 0.6 * np.sin(2 * np.pi * 50 * t) * np.exp(-t * 2)
    write_wav(root / "Audio/SFX/explosion_big_01.wav", boom * 0.6)
    t = np.arange(int(r * 0.25)) / r
    write_wav(root / "Audio/SFX/footstep_grass_01.wav", RNG.normal(0, 0.5, t.size) * np.exp(-t * 25))
    write_wav(root / "Audio/SFX/footstep_grass_02.wav", RNG.normal(0, 0.5, t.size) * np.exp(-t * 30))
    t = np.arange(int(r * 0.06)) / r
    write_wav(root / "Audio/UI/ui_click.wav", np.sin(2 * np.pi * 1800 * t) * np.exp(-t * 80))
    t = np.arange(int(r * 0.5)) / r
    coin = np.sin(2 * np.pi * np.where(t < 0.08, 988, 1319) * t) * np.exp(-t * 6) * 0.5
    write_wav(root / "Audio/SFX/coin_pickup.wav", coin)

    # A family of whoosh takes, with library metadata embedded the way Soundminer/BaseHead write it.
    t = np.arange(int(r * 0.6)) / r
    for n in range(1, 4):
        env = np.sin(np.pi * t / t[-1]) ** (2 + n)
        swoosh = RNG.normal(0, 0.2, t.size) * env
        library_wav(root / f"Audio/SFX/Whoosh_Fast_0{n}.wav", swoosh, f"Whoosh Fast {n}",
                    "Fast airy whoosh pass-by, short tail.", "whoosh; swish; air; fast; pass-by", "SWOOSHES", "SWISH")
    t = np.arange(int(r * 2.5)) / r
    hum = 0.2 * np.sin(2 * np.pi * 220 * t) * (1 + 0.3 * np.sin(2 * np.pi * 3 * t))
    library_wav(root / "Audio/SFX/magic_hum_loop.wav", hum, "Magic Shield Hum",
                "Sustained magical shield hum, seamless loop.", "magic; shield; hum; loop; aura", "MAGIC", "SHIMMER",
                loop=True)

    # 50 s stereo "music" loop: arpeggio over a drone.
    t = np.arange(int(r * 50)) / r
    notes = [220, 277.18, 329.63, 440, 329.63, 277.18]
    step = (t * 4).astype(int) % len(notes)
    freq = np.array(notes)[step]
    lead = 0.25 * np.sin(2 * np.pi * freq * t) * (1 - (t * 4 % 1))
    drone = 0.15 * np.sin(2 * np.pi * 110 * t)
    music = np.stack([lead + drone, lead * 0.8 + drone], -1).reshape(-1)
    wav = root / "Audio/Music/forest_theme_loop.wav"
    write_wav(wav, music, channels=2)
    if shutil.which("ffmpeg"):
        ogg = wav.with_suffix(".ogg")
        subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-i", str(wav), "-c:a", "libvorbis", "-q:a", "3",
                        "-metadata", "title=Forest Theme", "-metadata", "genre=Ambient", str(ogg)], check=True)
        wav.unlink()


def cube_gltf(external_bin: str | None = None, texture: str | None = None) -> tuple[dict, bytes]:
    """A unit cube with normals and UVs (24 verts). Returns (gltf json, buffer bytes)."""
    faces = [((0, 0, 1), (1, 0, 0), (0, 1, 0)), ((0, 0, -1), (-1, 0, 0), (0, 1, 0)), ((1, 0, 0), (0, 0, -1), (0, 1, 0)),
             ((-1, 0, 0), (0, 0, 1), (0, 1, 0)), ((0, 1, 0), (1, 0, 0), (0, 0, -1)), ((0, -1, 0), (1, 0, 0), (0, 0, 1))]
    pos, nrm, uv, idx = [], [], [], []
    for n, u, v in faces:
        base = len(pos)
        for su, sv in ((-1, -1), (1, -1), (1, 1), (-1, 1)):
            pos.append(tuple(n[i] + su * u[i] + sv * v[i] for i in range(3)))
            nrm.append(n)
            uv.append(((su + 1) / 2, 1 - (sv + 1) / 2))
        idx += [base, base + 1, base + 2, base, base + 2, base + 3]
    pb = b"".join(struct.pack("<3f", *p) for p in pos)
    nb = b"".join(struct.pack("<3f", *n) for n in nrm)
    ub = b"".join(struct.pack("<2f", *t) for t in uv)
    ib = b"".join(struct.pack("<H", i) for i in idx)
    buf = pb + nb + ub + ib
    uri = external_bin or "data:application/octet-stream;base64," + base64.b64encode(buf).decode()
    o = [0, len(pb), len(pb) + len(nb), len(pb) + len(nb) + len(ub)]
    doc = {
        "asset": {"version": "2.0", "generator": "hoard sample generator"},
        "buffers": [{"byteLength": len(buf), "uri": uri}],
        "bufferViews": [{"buffer": 0, "byteOffset": o[0], "byteLength": len(pb)},
                        {"buffer": 0, "byteOffset": o[1], "byteLength": len(nb)},
                        {"buffer": 0, "byteOffset": o[2], "byteLength": len(ub)},
                        {"buffer": 0, "byteOffset": o[3], "byteLength": len(ib)}],
        "accessors": [
            {"bufferView": 0, "componentType": 5126, "count": 24, "type": "VEC3", "min": [-1, -1, -1], "max": [1, 1, 1]},
            {"bufferView": 1, "componentType": 5126, "count": 24, "type": "VEC3"},
            {"bufferView": 2, "componentType": 5126, "count": 24, "type": "VEC2"},
            {"bufferView": 3, "componentType": 5123, "count": 36, "type": "SCALAR"},
        ],
        "meshes": [{"primitives": [{"attributes": {"POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2}, "indices": 3, "material": 0}]}],
        "materials": [{"name": "wood", "pbrMetallicRoughness": {"baseColorFactor": [0.8, 0.55, 0.3, 1], "metallicFactor": 0}}],
        "nodes": [{"mesh": 0, "rotation": [0, 0.2588, 0, 0.9659]}],
        "scenes": [{"nodes": [0]}],
        "scene": 0,
    }
    if texture:
        doc["images"] = [{"uri": texture}]
        doc["textures"] = [{"source": 0}]
        doc["materials"][0]["pbrMetallicRoughness"] = {"baseColorTexture": {"index": 0}, "metallicFactor": 0}
    return doc, buf


def planks(size: int = 256) -> np.ndarray:
    y, x = np.mgrid[0:size, 0:size] / size
    board = (y * 5).astype(int)
    grain = 0.5 + 0.5 * np.sin((x * 40 + board * 7.3) + 3 * np.sin(y * 30 + board))
    base = np.stack([0.55 + 0.15 * grain, 0.36 + 0.1 * grain, 0.18 + 0.05 * grain], -1)
    base[(y * 5 % 1) < 0.04] *= 0.4  # gaps between boards
    edge = (x < 0.05) | (x > 0.95) | (y < 0.05) | (y > 0.95)
    base[edge] = [0.3, 0.2, 0.1]
    return base


def models(root: Path) -> None:
    d = root / "Models"
    (d / "textures").mkdir(parents=True, exist_ok=True)
    # Textured crate: .gltf + external .bin + external texture, to exercise dependency links.
    doc, buf = cube_gltf(external_bin="crate_wooden.bin", texture="textures/crate_wooden_albedo.png")
    (d / "crate_wooden.gltf").write_text(json.dumps(doc, indent=1))
    (d / "crate_wooden.bin").write_bytes(buf)
    save_rgb(d / "textures/crate_wooden_albedo.png", planks())

    def glb(doc: dict, buf: bytes) -> bytes:
        doc = json.loads(json.dumps(doc))
        doc["buffers"] = [{"byteLength": len(buf)}]
        js = json.dumps(doc).encode()
        js += b" " * (-len(js) % 4)
        bin_ = buf + b"\0" * (-len(buf) % 4)
        total = 12 + 8 + len(js) + 8 + len(bin_)
        return (struct.pack("<4sII", b"glTF", 2, total) + struct.pack("<I4s", len(js), b"JSON") + js
                + struct.pack("<I4s", len(bin_), b"BIN\0") + bin_)

    rigged, rbuf = cube_gltf()
    rigged["nodes"] = [{"mesh": 0, "skin": 0, "scale": [0.6, 1.4, 0.4]}, {"name": "root_bone"}]
    rigged["skins"] = [{"joints": [1]}]
    rigged["materials"][0]["pbrMetallicRoughness"]["baseColorFactor"] = [0.55, 0.6, 0.75, 1]
    (d / "Characters").mkdir(parents=True, exist_ok=True)
    (d / "Characters/knight_rigged.glb").write_bytes(glb(rigged, rbuf))
    # Animation-only clip using the Unity/Mixamo "model@clip" naming convention.
    anim = {"asset": {"version": "2.0"}, "nodes": [{"name": "root_bone"}], "animations": [{"name": "walk", "channels": [], "samplers": []}]}
    js = json.dumps(anim).encode(); js += b" " * (-len(js) % 4)
    (d / "Characters/knight_rigged@walk.glb").write_bytes(struct.pack("<4sII", b"glTF", 2, 20 + len(js)) + struct.pack("<I4s", len(js), b"JSON") + js)

    # Barrel: OBJ -> MTL -> texture chain.
    n = 16
    lines = ["mtllib barrel.mtl", "o Barrel", "usemtl barrel_wood"]
    for y in (0, 1.6):
        for a in np.linspace(0, 2 * math.pi, n, endpoint=False):
            r = 0.8 if y else 0.8
            lines.append(f"v {r * math.cos(a):.4f} {y} {r * math.sin(a):.4f}")
    for i in range(n + 1):
        lines.append(f"vt {i / n:.4f} 0")
        lines.append(f"vt {i / n:.4f} 1")
    lines += [f"f {i + 1}/{2 * i + 1} {(i + 1) % n + 1}/{2 * i + 3} {(i + 1) % n + n + 1}/{2 * i + 4} {i + n + 1}/{2 * i + 2}" for i in range(n)]
    lines.append("f " + " ".join(str(i + n + 1) for i in range(n)))
    (d / "barrel.obj").write_text("\n".join(lines) + "\n")
    (d / "barrel.mtl").write_text("newmtl barrel_wood\nKd 0.8 0.6 0.4\nmap_Kd textures/barrel_albedo.png\n")
    save_rgb(d / "textures/barrel_albedo.png", planks(128)[:, ::-1])


def blend_file(path: Path, preview: Image.Image, image_paths: list[str], library_paths: list[str]) -> None:
    """Write a minimal .blend (Blender 4.5 legacy header) with a TEST preview block and IM/LI blocks.

    Real Blender files have many more blocks; Hoard only reads these. zstd-compressed like Blender 3.0+ when the
    `zstandard` module is installed."""
    out = bytearray(b"BLENDER-v405")

    def block(code: bytes, data: bytes) -> None:
        out.extend(code + struct.pack("<iQii", len(data), 0, 0, 1) + data)

    block(b"REND", bytes(72))
    w, h = preview.size
    rgba = preview.convert("RGBA").transpose(Image.Transpose.FLIP_TOP_BOTTOM).tobytes()  # stored bottom-up
    block(b"TEST", struct.pack("<ii", w, h) + rgba)
    for p in image_paths:
        block(b"IM\0\0", bytes(16) + b"IMtexture\0" + p.encode() + b"\0" + bytes(64))
    for p in library_paths:
        block(b"LI\0\0", bytes(8) + p.encode() + b"\0" + bytes(32))
    block(b"ENDB", b"")
    data = bytes(out)
    try:
        import zstandard
        data = zstandard.ZstdCompressor(level=3).compress(data)
    except ImportError:
        pass
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)


def psd_file(path: Path, img: Image.Image) -> None:
    """Minimal RGB PSD: header, a JPEG thumbnail in image resource 1036, no layers, raw composite."""
    img = img.convert("RGB")
    w, h = img.size
    thumb = img.copy()
    thumb.thumbnail((160, 160))
    import io
    buf = io.BytesIO()
    thumb.save(buf, "JPEG", quality=85)
    jpeg = buf.getvalue()
    tw, th = thumb.size
    res_data = struct.pack(">IIIIIIHH", 1, tw, th, ((tw * 24 + 31) // 32) * 4, tw * th * 3, len(jpeg), 24, 1) + jpeg
    res = b"8BIM" + struct.pack(">H", 1036) + b"\0\0" + struct.pack(">I", len(res_data)) + res_data
    if len(res_data) % 2:
        res += b"\0"
    planes = b"".join(bytes(img.getchannel(c).tobytes()) for c in "RGB")
    out = (b"8BPS" + struct.pack(">H", 1) + bytes(6) + struct.pack(">HIIHH", 3, h, w, 8, 3)
           + struct.pack(">I", 0) + struct.pack(">I", len(res)) + res + struct.pack(">I", 0)
           + struct.pack(">H", 0) + planes)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(out)


def kra_file(path: Path, img: Image.Image, layers: int) -> None:
    """Krita document: a zip with mimetype, maindoc.xml (layer list) and the flattened mergedimage.png."""
    import io
    def png(im: Image.Image) -> bytes:
        b = io.BytesIO()
        im.save(b, "PNG")
        return b.getvalue()
    preview = img.copy()
    preview.thumbnail((256, 256))
    layer_xml = "".join(f'<layer name="Layer {i + 1}" nodetype="paintlayer"/>' for i in range(layers))
    doc = f'<DOC><IMAGE width="{img.width}" height="{img.height}"><layers>{layer_xml}</layers></IMAGE></DOC>'
    path.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(path, "w") as z:
        z.writestr("mimetype", "application/x-krita", compress_type=zipfile.ZIP_STORED)
        z.writestr("maindoc.xml", doc)
        z.writestr("mergedimage.png", png(img))
        z.writestr("preview.png", png(preview))


def sources(root: Path) -> None:
    """Authoring files: a .blend scene linking a texture and a library, a PSD and a Krita painting."""
    s = root / "Source"
    # A stand-in "viewport preview": sky, ground and a crate silhouette.
    prev = Image.new("RGB", (128, 128), (120, 160, 210))
    px = prev.load()
    for y in range(80, 128):
        for x in range(128):
            px[x, y] = (70, 110, 50)
    for y in range(50, 95):
        for x in range(44, 88):
            px[x, y] = (150, 100, 55) if (y // 9) % 2 else (130, 85, 45)
    blend_file(s / "forest_scene.blend", prev, ["//../Models/textures/crate_wooden_albedo.png"], ["//props_library.blend"])
    blend_file(s / "props_library.blend", prev.rotate(90), [], [])
    concept = Image.fromarray(np.clip(np.stack([tileable_noise(256) * 0.6 + 0.3, tileable_noise(256) * 0.3 + 0.2,
                                                 tileable_noise(256) * 0.5 + 0.4], -1) * 255, 0, 255).astype(np.uint8))
    psd_file(s / "hero_concept.psd", concept.resize((512, 384)))
    kra_file(s / "forest_matte_painting.kra", concept.rotate(180), layers=4)


def ue_package(cls: str, asset: str, deps: list[str], thumb: bytes, tags: dict[str, str], body: bytes) -> bytes:
    """An Unreal 5.3 editor package (.uasset): summary, name map, imports, one export, thumbnail, registry tags,
    then `body` where bulk data would be. Synthetic, but laid out field by field like FPackageFileSummary."""
    def fstr(s: str) -> bytes:
        b = s.encode() + b"\0"
        return struct.pack("<i", len(b)) + b

    i32 = lambda v: struct.pack("<i", v)
    names = ["/Script/Engine", "Package", cls, asset, "Class", "None", *deps]
    idx = names.index
    out = bytearray()
    out += struct.pack("<I", 0x9E2A83C1) + i32(-8) + i32(864) + i32(522) + i32(1009) + i32(0) + i32(0)
    total_at = len(out); out += i32(0)
    out += fstr("None") + struct.pack("<I", 0) + i32(len(names))
    name_at = len(out); out += i32(0)
    out += i32(0) + i32(0) + fstr("") + i32(0) + i32(0)
    out += i32(1); export_at = len(out); out += i32(0)
    out += i32(2 + len(deps)); import_at = len(out); out += i32(0)
    out += i32(0) + i32(0) + i32(0) + i32(0)
    thumb_at = len(out); out += i32(0)
    out += bytes(16) + bytes(16) + i32(1) + i32(1) + i32(len(names))
    for _ in range(2):
        out += struct.pack("<HHHI", 5, 3, 2, 0) + fstr("++UE5+Release-5.3")
    out += struct.pack("<I", 0) + i32(0) + struct.pack("<I", 0) + i32(0)
    registry_at = len(out); out += i32(0)
    out += struct.pack("<q", 0)

    def patch(at: int, v: int) -> None:
        out[at:at + 4] = i32(v)

    def fname(n: str) -> bytes:
        return i32(idx(n)) + i32(0)

    patch(name_at, len(out))
    for n in names:
        out += fstr(n) + bytes(4)
    patch(import_at, len(out))
    for c, outer, o in [("Package", 0, "/Script/Engine"), ("Class", -1, cls)]:
        out += fname("/Script/Engine") + fname(c) + i32(outer) + fname(o) + i32(0) + i32(0) + i32(0)
    for d in deps:
        out += fname("/Script/Engine") + fname("Package") + i32(0) + fname(d) + i32(0) + i32(0) + i32(0)
    patch(export_at, len(out))
    # The export's data is an empty property list ("None") at the start of the body.
    props = fname("None")
    out += i32(-2) + i32(0) + i32(0) + i32(0) + fname(asset) + struct.pack("<I", 0)
    serial_at = len(out); out += struct.pack("<qq", len(props), 0) + bytes(12)
    out += i32(0) + struct.pack("<I", 0) + i32(0) + i32(1) + i32(0) + bytes(20)
    patch(thumb_at, len(out))
    out += i32(1) + fstr(cls) + fstr(asset)
    data_at = len(out); out += i32(0)
    patch(data_at, len(out))
    out += i32(256) + i32(256) + i32(len(thumb)) + thumb
    patch(registry_at, len(out))
    out += struct.pack("<q", 0) + i32(1) + fstr(asset) + fstr(f"/Script/Engine.{cls}") + i32(len(tags))
    for k, v in tags.items():
        out += fstr(k) + fstr(v)
    patch(total_at, len(out))
    out[serial_at + 8:serial_at + 16] = struct.pack("<q", len(out))
    return bytes(out) + props + body


def unreal(root: Path) -> None:
    """A small Unreal content folder, the way Fab ships packs: Content/<Pack>/..."""
    content = root / "Unreal/Content/ForestKit"
    for d in ("Audio", "Textures", "Materials", "Meshes"):
        (content / d).mkdir(parents=True, exist_ok=True)

    def png(img: Image.Image) -> bytes:
        buf = io.BytesIO(); img.save(buf, "PNG"); return buf.getvalue()

    # SoundWave: Unreal keeps the imported WAV as bulk data.
    r = 44100
    t = np.arange(int(r * 1.2)) / r
    chirp = (0.3 * np.sin(2 * np.pi * (2400 + 900 * np.sin(2 * np.pi * 6 * t)) * t) * np.exp(-t * 2.5) * 32767).astype("<i2")
    wav = io.BytesIO()
    with wave.open(wav, "wb") as w:
        w.setnchannels(1); w.setsampwidth(2); w.setframerate(r); w.writeframes(chirp.tobytes())
    icon = png(Image.new("RGB", (256, 256), (40, 30, 60)))
    (content / "Audio/S_Birdsong.uasset").write_bytes(ue_package(
        "SoundWave", "S_Birdsong", [], icon, {"Duration": "1.20", "NumChannels": "1", "SampleRate": "44100"},
        b"\0" * 64 + wav.getvalue() + b"\0" * 16))

    # Texture2D: UE4-style source kept as PNG.
    moss = (tileable_noise(256) * 255).astype(np.uint8)
    moss_img = Image.fromarray(np.stack([moss * 0.35, moss * 0.6, moss * 0.25], -1).astype(np.uint8))
    (content / "Textures/T_Moss_D.uasset").write_bytes(ue_package(
        "Texture2D", "T_Moss_D", [], png(moss_img.resize((128, 128))), {"Dimensions": "256x256", "Format": "PF_DXT1"},
        b"\0" * 32 + png(moss_img) + b"\0" * 8))

    # Material instance using the texture, and a mesh using the material: only the editor thumbnail and
    # registry tags are readable without Unreal; the links come from the import tables.
    (content / "Materials/MI_Moss.uasset").write_bytes(ue_package(
        "MaterialInstanceConstant", "MI_Moss", ["/Game/ForestKit/Textures/T_Moss_D"], png(moss_img.resize((256, 256))),
        {"Parent": "/Script/Engine.Material'/Game/ForestKit/Materials/M_Base.M_Base'"}, b""))
    stump = Image.new("RGB", (256, 256), (30, 34, 44))
    d = ImageDraw.Draw(stump)
    d.ellipse((68, 60, 188, 110), fill=(150, 110, 70)); d.rectangle((68, 85, 188, 190), fill=(110, 75, 45))
    d.ellipse((68, 165, 188, 215), fill=(110, 75, 45))
    (content / "Meshes/SM_TreeStump.uasset").write_bytes(ue_package(
        "StaticMesh", "SM_TreeStump", ["/Game/ForestKit/Materials/MI_Moss"], png(stump),
        {"Triangles": "1840", "Vertices": "1012", "Materials": "1", "LODs": "3"}, b"\0" * 128))


def main() -> None:
    out = Path(sys.argv[1] if len(sys.argv) > 1 else "sample")
    root = out / "Forest Starter Kit"
    if root.exists():
        shutil.rmtree(root)
    textures(root)
    sprites(root)
    audio(root)
    models(root)
    sources(root)
    unreal(root)
    (root / "License.txt").write_text(
        "Forest Starter Kit (sample)\n\nLicense: Creative Commons Zero (CC0 1.0 Universal)\n"
        "https://creativecommons.org/publicdomain/zero/1.0/\n"
    )
    (root / "README.txt").write_text(
        "Forest Starter Kit\n\nA small kit for a cozy top-down forest game: tileable ground textures, a PBR rock,\n"
        "pixel sprites, a few sound effects and a looping theme, plus props (crate, barrel) and a rigged knight.\n\n"
        "Get updates at https://hoard-sample.itch.io/forest-starter-kit\n"
    )
    (root / ".DS_Store").write_bytes(b"junk")
    zpath = out / "forest-starter-kit.zip"
    with zipfile.ZipFile(zpath, "w", zipfile.ZIP_DEFLATED) as z:
        for f in sorted(root.rglob("*")):
            if f.is_file():
                z.write(f, f.relative_to(out))
    print(zpath)


if __name__ == "__main__":
    main()
