// The packs every run starts with, all made in code.
import type { UploadFile } from './api'
import { checker, crate, hue, LEVEL_JSON, NOTES_TXT, triangleGltf, zip } from './files'

export const TEXTURES = 'E2E Textures'
export const MODELS = 'E2E Models'
export const ZIPPED = 'E2E Zipped'
export const BIG = 'E2E Big Pack'
export const BIG_COUNT = 300
/** The store the zipped pack says it came from. */
export const STORE = 'Itch Example'

export interface SeedPack {
  name: string
  files: UploadFile[]
}

export function seedPacks(): SeedPack[] {
  const c = crate()
  const textures: [string, [number, number, number], [number, number, number]][] = [
    ['brick_wall', [170, 60, 40], [120, 40, 30]],
    ['grass_ground', [60, 160, 60], [40, 110, 40]],
    ['stone_tile', [140, 140, 150], [100, 100, 110]],
    ['water_surface', [40, 90, 200], [30, 60, 150]],
  ]
  return [
    {
      name: TEXTURES,
      files: textures.map(([n, a, b]) => ({ relpath: `${n}.png`, data: checker(32, a, b) })),
    },
    {
      name: MODELS,
      files: [
        { relpath: 'crate.obj', data: c.obj },
        { relpath: 'crate.mtl', data: c.mtl },
        { relpath: 'crate_diffuse.png', data: c.texture },
        { relpath: 'triangle.gltf', data: triangleGltf() },
        { relpath: 'level.json', data: Buffer.from(JSON.stringify(LEVEL_JSON)) },
        { relpath: 'notes.txt', data: Buffer.from(NOTES_TXT) },
      ],
    },
    {
      name: ZIPPED,
      files: [
        {
          relpath: 'e2e-zipped.zip',
          data: zip([
            { path: 'sprites/coin.png', data: checker(16, [230, 190, 40], [180, 140, 20], 2) },
            { path: 'sprites/gem.png', data: checker(16, [200, 40, 200], [140, 20, 140], 2) },
            {
              path: 'LICENSE.txt',
              data: Buffer.from('CC0 1.0 Universal\n\nThe person who associated a work with this deed has dedicated it to the public domain.\n'),
            },
          ]),
        },
      ],
    },
    {
      name: BIG,
      files: Array.from({ length: BIG_COUNT }, (_, i) => ({
        relpath: `img_${String(i).padStart(3, '0')}.png`,
        data: checker(16, hue(i, BIG_COUNT), [i % 256, (i * 7) % 256, (i * 13) % 256], 2),
      })),
    },
  ]
}
