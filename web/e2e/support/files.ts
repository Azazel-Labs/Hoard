// Fixture files made in code, so the suite needs nothing checked in and no network: PNGs, a zip, an OBJ with its
// material and texture, a glTF, JSON and plain text.
import { deflateSync } from 'node:zlib'

const CRC_TABLE = (() => {
  const t = new Uint32Array(256)
  for (let n = 0; n < 256; n++) {
    let c = n
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1
    t[n] = c >>> 0
  }
  return t
})()

export function crc32(buf: Uint8Array): number {
  let c = 0xffffffff
  for (const b of buf) c = CRC_TABLE[(c ^ b) & 0xff] ^ (c >>> 8)
  return (c ^ 0xffffffff) >>> 0
}

function chunk(type: string, data: Buffer): Buffer {
  const len = Buffer.alloc(4)
  len.writeUInt32BE(data.length)
  const body = Buffer.concat([Buffer.from(type, 'ascii'), data])
  const crc = Buffer.alloc(4)
  crc.writeUInt32BE(crc32(body))
  return Buffer.concat([len, body, crc])
}

/** An RGBA PNG; `pixel(x, y)` gives each pixel's [r, g, b, a]. */
export function png(width: number, height: number, pixel: (x: number, y: number) => [number, number, number, number]): Buffer {
  const raw = Buffer.alloc((width * 4 + 1) * height)
  let o = 0
  for (let y = 0; y < height; y++) {
    raw[o++] = 0 // no filter
    for (let x = 0; x < width; x++) {
      const [r, g, b, a] = pixel(x, y)
      raw[o++] = r
      raw[o++] = g
      raw[o++] = b
      raw[o++] = a
    }
  }
  const ihdr = Buffer.alloc(13)
  ihdr.writeUInt32BE(width, 0)
  ihdr.writeUInt32BE(height, 4)
  ihdr[8] = 8 // bit depth
  ihdr[9] = 6 // RGBA
  return Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    chunk('IHDR', ihdr),
    chunk('IDAT', deflateSync(raw)),
    chunk('IEND', Buffer.alloc(0)),
  ])
}

/** A two-colour checkerboard, so every image is different bytes and reads as a picture in a thumbnail. */
export function checker(size: number, a: [number, number, number], b: [number, number, number], cell = 4): Buffer {
  return png(size, size, (x, y) => [...((Math.floor(x / cell) + Math.floor(y / cell)) % 2 ? a : b), 255])
}

/** A colour for the n-th of many images: spread round the hue wheel. */
export function hue(n: number, of: number): [number, number, number] {
  const h = (n / of) * 6
  const x = Math.round(255 * (1 - Math.abs((h % 2) - 1)))
  const table: [number, number, number][] = [[255, x, 0], [x, 255, 0], [0, 255, x], [0, x, 255], [x, 0, 255], [255, 0, x]]
  return table[Math.floor(h) % 6]
}

/** A zip of stored (uncompressed) entries. */
export function zip(entries: { path: string; data: Buffer }[]): Buffer {
  const locals: Buffer[] = []
  const centrals: Buffer[] = []
  let offset = 0
  for (const e of entries) {
    const name = Buffer.from(e.path, 'utf8')
    const crc = crc32(e.data)
    const local = Buffer.alloc(30)
    local.writeUInt32LE(0x04034b50, 0)
    local.writeUInt16LE(20, 4) // version needed
    local.writeUInt16LE(0x0800, 6) // UTF-8 names
    local.writeUInt16LE(0, 8) // stored
    local.writeUInt16LE(0, 10) // time
    local.writeUInt16LE(0x5a21, 12) // date: 2025-01-01
    local.writeUInt32LE(crc, 14)
    local.writeUInt32LE(e.data.length, 18)
    local.writeUInt32LE(e.data.length, 22)
    local.writeUInt16LE(name.length, 26)
    local.writeUInt16LE(0, 28)
    locals.push(local, name, e.data)

    const central = Buffer.alloc(46)
    central.writeUInt32LE(0x02014b50, 0)
    central.writeUInt16LE(20, 4) // made by
    central.writeUInt16LE(20, 6)
    central.writeUInt16LE(0x0800, 8)
    central.writeUInt16LE(0, 10)
    central.writeUInt16LE(0, 12)
    central.writeUInt16LE(0x5a21, 14)
    central.writeUInt32LE(crc, 16)
    central.writeUInt32LE(e.data.length, 20)
    central.writeUInt32LE(e.data.length, 24)
    central.writeUInt16LE(name.length, 28)
    central.writeUInt32LE(offset, 42)
    centrals.push(central, name)
    offset += local.length + name.length + e.data.length
  }
  const dir = Buffer.concat(centrals)
  const end = Buffer.alloc(22)
  end.writeUInt32LE(0x06054b50, 0)
  end.writeUInt16LE(entries.length, 8)
  end.writeUInt16LE(entries.length, 10)
  end.writeUInt32LE(dir.length, 12)
  end.writeUInt32LE(offset, 16)
  return Buffer.concat([...locals, dir, end])
}

/** A textured cube: OBJ, its MTL and the diffuse texture the MTL names. */
export function crate(): { obj: Buffer; mtl: Buffer; texture: Buffer } {
  const v = [
    [-1, -1, -1], [1, -1, -1], [1, 1, -1], [-1, 1, -1],
    [-1, -1, 1], [1, -1, 1], [1, 1, 1], [-1, 1, 1],
  ]
  const faces = [
    [1, 2, 3, 4], [5, 8, 7, 6], [1, 5, 6, 2], [2, 6, 7, 3], [3, 7, 8, 4], [5, 1, 4, 8],
  ]
  const lines = ['# A crate for the e2e suite', 'mtllib crate.mtl', 'o crate']
  for (const p of v) lines.push(`v ${p.join(' ')}`)
  lines.push('vt 0 0', 'vt 1 0', 'vt 1 1', 'vt 0 1')
  lines.push('vn 0 0 -1', 'vn 0 0 1', 'vn 0 -1 0', 'vn 1 0 0', 'vn 0 1 0', 'vn -1 0 0')
  lines.push('usemtl crate')
  faces.forEach((f, i) => lines.push(`f ${f.map((p, j) => `${p}/${j + 1}/${i + 1}`).join(' ')}`))
  const mtl = ['newmtl crate', 'Kd 1 1 1', 'map_Kd crate_diffuse.png'].join('\n')
  return {
    obj: Buffer.from(lines.join('\n') + '\n'),
    mtl: Buffer.from(mtl + '\n'),
    texture: checker(64, [150, 100, 50], [90, 60, 30], 8),
  }
}

/** A single triangle with its buffer inline. */
export function triangleGltf(): Buffer {
  const positions = new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0])
  const bin = Buffer.from(positions.buffer)
  const gltf = {
    asset: { version: '2.0', generator: 'hoard e2e' },
    scene: 0,
    scenes: [{ nodes: [0] }],
    nodes: [{ mesh: 0, name: 'triangle' }],
    meshes: [{ name: 'triangle', primitives: [{ attributes: { POSITION: 0 } }] }],
    buffers: [{ uri: `data:application/octet-stream;base64,${bin.toString('base64')}`, byteLength: bin.length }],
    bufferViews: [{ buffer: 0, byteOffset: 0, byteLength: bin.length, target: 34962 }],
    accessors: [{ bufferView: 0, componentType: 5126, count: 3, type: 'VEC3', min: [0, 0, 0], max: [1, 1, 0] }],
  }
  return Buffer.from(JSON.stringify(gltf, null, 2))
}

export const LEVEL_JSON = {
  name: 'Forest clearing',
  version: 3,
  spawn: { x: 12.5, y: 0, z: -4 },
  enemies: [
    { kind: 'slime', hp: 10 },
    { kind: 'bat', hp: 6 },
  ],
  tiles: { ground: 'grass', walls: ['stone', 'wood'] },
}

export const NOTES_TXT = ['Notes for the e2e pack', '', 'The crate uses crate_diffuse.png.', 'The triangle is a glTF with its buffer inline.'].join('\n')
