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

/** A PNG with a `tEXt` chunk added after its header, as image generators write their settings. */
export function withText(image: Buffer, key: string, text: string): Buffer {
  const ihdrEnd = 8 + 25
  const data = Buffer.concat([Buffer.from(key, 'latin1'), Buffer.from([0]), Buffer.from(text, 'latin1')])
  return Buffer.concat([image.subarray(0, ihdrEnd), chunk('tEXt', data), image.subarray(ihdrEnd)])
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

/** The names of the files in a zip, from its central directory. */
export function zipNames(buf: Buffer): string[] {
  let end = buf.length - 22
  while (end >= 0 && buf.readUInt32LE(end) !== 0x06054b50) end--
  if (end < 0) throw new Error('not a zip')
  const count = buf.readUInt16LE(end + 10)
  let at = buf.readUInt32LE(end + 16)
  const names: string[] = []
  for (let i = 0; i < count; i++) {
    const len = buf.readUInt16LE(at + 28)
    const extra = buf.readUInt16LE(at + 30)
    const comment = buf.readUInt16LE(at + 32)
    names.push(buf.toString('utf8', at + 46, at + 46 + len))
    at += 46 + len + extra + comment
  }
  return names
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

/** A square glTF with texture coordinates and no material of its own, its buffer in `<bin>` beside it. */
export function quadGltf(bin: string): { gltf: Buffer; bin: Buffer } {
  const positions = new Float32Array([-1, 0, 0, 1, 0, 0, 1, 2, 0, -1, 2, 0])
  const uvs = new Float32Array([0, 1, 1, 1, 1, 0, 0, 0])
  const indices = new Uint16Array([0, 1, 2, 0, 2, 3])
  const data = Buffer.concat([Buffer.from(positions.buffer), Buffer.from(uvs.buffer), Buffer.from(indices.buffer)])
  const gltf = {
    asset: { version: '2.0', generator: 'hoard e2e' },
    scene: 0,
    scenes: [{ nodes: [0] }],
    nodes: [{ mesh: 0, name: 'sign' }],
    meshes: [{ name: 'sign', primitives: [{ attributes: { POSITION: 0, TEXCOORD_0: 1 }, indices: 2 }] }],
    buffers: [{ uri: bin, byteLength: data.length }],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: 48, target: 34962 },
      { buffer: 0, byteOffset: 48, byteLength: 32, target: 34962 },
      { buffer: 0, byteOffset: 80, byteLength: 12, target: 34963 },
    ],
    accessors: [
      { bufferView: 0, componentType: 5126, count: 4, type: 'VEC3', min: [-1, 0, 0], max: [1, 2, 0] },
      { bufferView: 1, componentType: 5126, count: 4, type: 'VEC2' },
      { bufferView: 2, componentType: 5123, count: 6, type: 'SCALAR' },
    ],
  }
  return { gltf: Buffer.from(JSON.stringify(gltf, null, 2)), bin: data }
}

/** A Bambu Studio-style 3MF: a 10 mm cube printed in white (filament 1) with its top painted red (filament 2). */
export function paintedCube3mf(): Buffer {
  const v = [[0, 0, 0], [10, 0, 0], [10, 10, 0], [0, 10, 0], [0, 0, 10], [10, 0, 10], [10, 10, 10], [0, 10, 10]]
  // Bottom, top, then the four sides; the top's two triangles (2 and 3) are painted.
  const t = [[0, 2, 1], [0, 3, 2], [4, 5, 6], [4, 6, 7], [0, 1, 5], [0, 5, 4], [1, 2, 6], [1, 6, 5], [2, 3, 7], [2, 7, 6], [3, 0, 4], [3, 4, 7]]
  const vertices = v.map(([x, y, z]) => `<vertex x="${x}" y="${y}" z="${z}"/>`).join('')
  const triangles = t.map(([a, b, c], i) => `<triangle v1="${a}" v2="${b}" v3="${c}"${i === 2 || i === 3 ? ' paint_color="8"' : ''}/>`).join('')
  const model = `<?xml version="1.0" encoding="UTF-8"?>
<model unit="millimeter" xml:lang="en-US" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02">
 <metadata name="Application">BambuStudio-01.10.00.00</metadata>
 <resources><object id="1" type="model"><mesh><vertices>${vertices}</vertices><triangles>${triangles}</triangles></mesh></object></resources>
 <build><item objectid="1" transform="1 0 0 0 1 0 0 0 1 128 128 5"/></build>
</model>`
  const rels = `<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Target="/3D/3dmodel.model" Id="rel-1" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/></Relationships>`
  const settings = `<?xml version="1.0" encoding="UTF-8"?>
<config><object id="1"><metadata key="name" value="Cube"/><metadata key="extruder" value="1"/></object></config>`
  return zip([
    { path: '_rels/.rels', data: Buffer.from(rels) },
    { path: '3D/3dmodel.model', data: Buffer.from(model) },
    { path: 'Metadata/model_settings.config', data: Buffer.from(settings) },
    { path: 'Metadata/project_settings.config', data: Buffer.from(JSON.stringify({ filament_colour: ['#FFFFFF', '#FF0000'] })) },
  ])
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

/** The folder a Megascans 3D asset downloads as: files named with the asset's id, and its `<id>.json`. */
export const MEGASCANS_FOLDER = 'Historic_Miscellaneous_Stand_Wood_Old_05_xeokah0_Raw'
/** The listing's title, from that JSON. */
export const MEGASCANS_TITLE = 'Historic Miscellaneous Stand Wood Old 05'

/** A Megascans 3D asset: a box of a model, four maps that make a material, and the JSON that says what it is. */
export function megascans(): { path: string; data: Buffer }[] {
  const base = MEGASCANS_FOLDER
  const v = [
    [-1, 0, -0.5], [1, 0, -0.5], [1, 0, 0.5], [-1, 0, 0.5],
    [-1, 1.6, -0.5], [1, 1.6, -0.5], [1, 1.6, 0.5], [-1, 1.6, 0.5],
  ]
  const obj = [
    'o Stand',
    ...v.map((p) => `v ${p.join(' ')}`),
    'vt 0 0', 'vt 1 0', 'vt 1 1', 'vt 0 1',
    'f 1/1 2/2 3/3 4/4', 'f 5/1 8/4 7/3 6/2', 'f 1/1 5/4 6/3 2/2',
    'f 2/1 6/4 7/3 3/2', 'f 3/1 7/4 8/3 4/2', 'f 4/1 8/4 5/3 1/2',
    '',
  ].join('\n')
  const maps: [string, [number, number, number], [number, number, number]][] = [
    ['BaseColor', [120, 80, 50], [100, 64, 40]],
    ['Normal', [128, 128, 255], [120, 120, 250]],
    ['Roughness', [200, 200, 200], [180, 180, 180]],
    ['AO', [230, 230, 230], [210, 210, 210]],
  ]
  const meta = {
    id: 'xeokah0',
    semanticTags: {
      name: MEGASCANS_TITLE,
      name_formats: { title: MEGASCANS_TITLE, slug: 'historic_miscellaneous_stand_wood_old_05' },
      contains: ['table'],
      color: ['brown'],
      country: 'Denmark',
      region: 'Europe',
      asset_type: '3D asset',
    },
    meta: [
      { name: 'Length', key: 'length', value: '118 cm' },
      { name: 'Width', key: 'width', value: '54 cm' },
      { name: 'Height', key: 'height', value: '81 cm' },
    ],
    tags: ['wood', 'old', 'stand', 'furniture', 'rustic'],
  }
  return [
    { path: `${base}/${base}.obj`, data: Buffer.from(obj) },
    ...maps.map(([m, a, b]) => ({ path: `${base}/${base}_2K_${m}.png`, data: checker(64, a, b, 8) })),
    { path: `${base}/xeokah0.json`, data: Buffer.from(JSON.stringify(meta, null, 2)) },
  ]
}
