/**
 * dragon-hoard.ts
 * ─────────────────────────────────────────────────────────────────────────────
 * A low-poly dragon guarding a treasure hoard whose size and make-up are driven
 * by how many assets someone owns. Every asset is one numbered piece of treasure,
 * drawn with instancing, its place worked out on the GPU from its number (three.js
 * WebGPURenderer + TSL). Where WebGPU is missing, three runs the same code on
 * WebGL2.
 *
 * Only dependency: `three` (written against r187; needs the `three/webgpu` and
 * `three/tsl` entry points, so build with a bundler).
 *
 * Drop-in widget:
 *
 *   import { mountDragonHoard, countAssets } from './dragon-hoard';
 *
 *   const hoard = mountDragonHoard(document.getElementById('hoard')!, {
 *     counts: countAssets(myAssets),        // or { gold: 1200, gems: 300, ... }
 *   });
 *   hoard.add({ gems: 50 });                 // 50 gems rain onto the pile
 *   hoard.setCounts(newCounts);              // pours in whatever grew
 *   hoard.dispose();
 *
 * Inside your own WebGPURenderer scene:
 *
 *   const hoard = new DragonHoard({ counts });
 *   scene.add(hoard);
 *   renderer.setAnimationLoop(() => {
 *     hoard.update(clock.getDelta());
 *     renderer.render(scene, camera);
 *   });
 *
 * Asset type → treasure (pass your own map to `countAssets` to change it):
 *   documents / text      → books
 *   models                → gems
 *   textures / materials  → gold
 *   sounds                → weapons
 *   animations / skeletal → bones (skeletons of foes)
 *   fonts                 → rune tablets
 *   icons                 → paintings
 *   sprites               → potion bottles
 *
 * How treasure arrives
 *   When the collection grows, the dragon roars a portal open in the sky, backs
 *   off the pile, and the new pieces stream down through it. The portal is one
 *   way: a piece is only drawn once it is through the mouth. Where the stream
 *   hits the crown, landed pieces spill out and run down the slope to their
 *   places. Once they are all down the dragon climbs back up. Small additions
 *   skip the walk.
 *
 * Always your collection's mix
 *   Each kind of treasure is spread evenly through the whole pile, so the layer
 *   you can see always holds every kind in the proportions you own, however the
 *   collection was built up. Add only fonts and a few more tablets surface; the
 *   gold does not disappear under them.
 *
 * Deterministic and mass-conserving
 *   Where a piece rests is a pure function of its number and of the collection
 *   as it stands. Nothing is integrated frame to frame. The same collection is
 *   always the same pile, at any frame rate and whether it poured in or loaded
 *   at rest, and the pile's size is exactly the pieces that have landed.
 *
 * Bounded drawing, no ceiling, no waiting
 *   Only the layer of treasure nearest the surface is instanced (`maxParticles`
 *   caps it), and the falling stream is a small recycled pool (`maxStream`).
 *   Nothing is stored per piece: the vertex shader places each one. So what is
 *   drawn follows the pile's surface, not its total, loading costs the same at
 *   any size, and a pour allocates nothing. The stream is paced by the hoard's
 *   size, so the same relative growth takes the same time at any scale.
 *
 * Quick to start
 *   All treasure, of every kind, resting or falling, is drawn with one shared
 *   material, so there is one treasure shader to compile however many kinds
 *   there are. The widget compiles what the first frame needs without blocking
 *   the page, shows it, and compiles the portal and the rest afterwards.
 *
 * Coming back
 *   Pass `lastSeen` (or let the widget keep it with `remember`) and the hoard
 *   appears at once as it was last seen; only what was gained since streams in.
 *
 * Domains
 *   The hoard lives in a domain of fixed size (ledge, den, cavern, great hall,
 *   mountain vault, then generated ones). When the pile fills it, the pour
 *   pauses, the dragon moves to a bigger domain, and the whole hoard streams
 *   into the new one.
 *
 * Arcade tube
 *   `retro: true` (or `handle.setRetro(true)`) shows everything through an
 *   analog-video chain: low fixed raster, phosphor persistence, glow, RGB split,
 *   scan lines, grain and the odd tear. `RetroDisplay` is exported for use with
 *   your own renderer.
 */

import * as THREE from 'three/webgpu';
import {
  acesFilmicToneMapping,
  attribute,
  cos,
  float,
  fract,
  hash,
  instanceIndex,
  mix,
  positionGeometry,
  positionWorld,
  screenCoordinate,
  select,
  sin,
  texture,
  uint,
  uniform,
  uniformArray,
  userData,
  uv,
  vec2,
  vec3,
  vec4,
} from 'three/tsl';

/* ════════════════════════════════ Public types ════════════════════════════════ */

export type TreasureType = 'gold' | 'gems' | 'books' | 'weapons' | 'bones' | 'tablets' | 'paintings' | 'potions';
export type HoardCounts = Partial<Record<TreasureType, number>>;
export type DragonTier = 'hatchling' | 'whelp' | 'drake' | 'wyrm' | 'ancient';
export type DragonMood = 'auto' | 'awake' | 'asleep';

export interface DragonColors {
  body: THREE.ColorRepresentation;
  belly: THREE.ColorRepresentation;
  wing: THREE.ColorRepresentation;
  horn: THREE.ColorRepresentation;
  eye: THREE.ColorRepresentation;
}

export interface DragonHoardOptions {
  /** The collection as it is now, per treasure type. */
  counts?: HoardCounts;
  /**
   * The collection as this person last saw it. The hoard appears in that state at once, and
   * whatever they have gained since streams in through the portal. Leave it out to appear
   * at `counts` with nothing streaming.
   */
  lastSeen?: HoardCounts;
  /**
   * Most resting pieces to draw. Only the layer of treasure nearest the surface is drawn, and
   * this thins that layer on very large piles. Default 150 000.
   */
  maxParticles?: number;
  /** Most pieces in the falling stream at once, whatever the size of the pour. Default 20 000. */
  maxStream?: number;
  /** Collection size at which the dragon is fully grown. Default 2 000 000. */
  fullAt?: number;
  /** Changes the random layout. */
  seed?: number;
  dragon?: Partial<DragonColors>;
  /** 'auto' sleeps on an empty hoard and is awake otherwise. */
  mood?: DragonMood;
  /** Breathe fire when roaring (young dragons only manage smoke). Default true. */
  fire?: boolean;
  /**
   * How fast treasure that is owed arrives: 2 runs the portal, the pour and the dragon's
   * walk at double speed. The number of pieces moving at once stays the same. Default 1.
   */
  speed?: number;
  /** Color of the sky portal treasure arrives through. */
  portal?: THREE.ColorRepresentation;
  /**
   * When treasure arrives the dragon summons the portal, backs off the pile, and climbs
   * back once it has landed. 'auto' skips the walk for small additions. Default 'auto'.
   */
  ceremony?: 'auto' | 'always' | 'never';
  /** The ladder of domains the hoard outgrows. Past the last one, bigger ones are generated. */
  domains?: readonly DomainDef[];
  /** Add the built-in lights. Default true. */
  lights?: boolean;
  /** Dragon casts a shadow on the hoard. Default true. */
  shadows?: boolean;
  /** Draw the rock base under the hoard. Default true. */
  base?: boolean;
}

/** A place the hoard lives in. It is full when the pile reaches 55% of its radius. */
export interface DomainDef {
  name: string;
  /** Floor radius in world units. Each domain should be at least twice the one before. */
  radius: number;
}

export type HoardPhase = 'idle' | 'summon' | 'pour' | 'ascend' | 'return';

export interface PourOptions {
  /** Rain the difference in instead of placing it instantly. Default true. */
  animate?: boolean;
  /** Seconds the pour lasts. Default scales with how much the hoard grows (2.5 to 12 s). */
  duration?: number;
}

export interface HoardStats {
  total: number;
  counts: Record<TreasureType, number>;
  /** Pieces that have come to rest in the pile, buried ones included. Equals `total` once a pour is done. */
  landed: number;
  /** Pieces currently drawn: the layer nearest the surface, plus the falling stream during a pour. */
  particles: number;
  /** Pile radius and height in world units. */
  radius: number;
  height: number;
  /** 0..1 dragon maturity. */
  age: number;
  tier: DragonTier;
  /** Which domain the hoard is in, and how full it is (0..1). */
  domain: { index: number; name: string; fill: number };
  /** What the dragon is doing: summoning the portal, waiting out the pour, moving domain, climbing back. */
  phase: HoardPhase;
  pouring: boolean;
}

export const TREASURE_TYPES: readonly TreasureType[] = ['gold', 'gems', 'books', 'weapons', 'bones', 'tablets', 'paintings', 'potions'];

export const DEFAULT_DOMAINS: readonly DomainDef[] = [
  { name: 'Rock Ledge', radius: 2 },
  { name: 'Hidden Den', radius: 4.5 },
  { name: 'Crystal Cavern', radius: 10 },
  { name: 'Great Hall', radius: 22 },
  { name: 'Mountain Vault', radius: 48 },
];

/** Default asset-type → treasure mapping. Keys are lower-cased with punctuation removed. */
export const DEFAULT_ASSET_MAP: Readonly<Record<string, TreasureType>> = {
  text: 'books', textfile: 'books', document: 'books', doc: 'books', script: 'books',
  model: 'gems', mesh: 'gems', staticmesh: 'gems',
  texture: 'gold', material: 'gold', image: 'gold',
  sound: 'weapons', audio: 'weapons', music: 'weapons',
  animation: 'bones', skeletalmesh: 'bones', skeleton: 'bones', rig: 'bones',
  font: 'tablets', typeface: 'tablets',
  icon: 'paintings',
  sprite: 'potions', spritesheet: 'potions',
};

/**
 * Turn your assets into hoard counts.
 * Accepts a list of things with a `type`, or a `{ assetType: count }` record.
 * Types missing from the map go to `fallback` (pass null to drop them).
 */
export function countAssets(
  assets: Iterable<{ type: string }> | Record<string, number>,
  map: Readonly<Record<string, TreasureType>> = DEFAULT_ASSET_MAP,
  fallback: TreasureType | null = 'gold',
): Record<TreasureType, number> {
  const out: Record<TreasureType, number> = { gold: 0, gems: 0, books: 0, weapons: 0, bones: 0, tablets: 0, paintings: 0, potions: 0 };
  const norm = (s: string) => s.toLowerCase().replace(/[^a-z0-9]/g, '');
  const lookup: Record<string, TreasureType> = {};
  for (const k of Object.keys(map)) lookup[norm(k)] = map[k];
  const bump = (type: string, n: number) => {
    const key = norm(type);
    const t = lookup[key] ?? lookup[key.replace(/s$/, '')] ?? fallback;
    if (t && n > 0) out[t] += n;
  };
  if (Symbol.iterator in (assets as object)) {
    for (const a of assets as Iterable<{ type: string }>) bump(a.type, 1);
  } else {
    const rec = assets as Record<string, number>;
    for (const k of Object.keys(rec)) bump(k, rec[k]);
  }
  return out;
}

/* ════════════════════════════════ Small utilities ═════════════════════════════ */

/** TSL node. The TSL typings are still moving between releases, so nodes are left loosely typed. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type N = any;

const sinN = sin as unknown as (x: N) => N;
const cosN = cos as unknown as (x: N) => N;

const TAU = Math.PI * 2;
const DEG = Math.PI / 180;
const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
const clamp01 = (x: number) => (x < 0 ? 0 : x > 1 ? 1 : x);
const clampN = (x: number, a: number, b: number) => (x < a ? a : x > b ? b : x);
const ease = (t: number) => t * t * (3 - 2 * t);

function mulberry32(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/* ════════════════════════════════ Pile model ══════════════════════════════════
 * A heap at its angle of repose: a cone of slope SLOPE with a small flat crown,
 * h(r) = SLOPE · (R − max(r, TOP · R)). The shape is self-similar, so volume is
 * VOL_K · R³, and adding volume lays an even skin over the whole surface. That is
 * what lets a piece be buried by later ones without ever having to move.          */

const SLOPE = 0.5;
const TOP = 0.15;
const VOL_K = (Math.PI * SLOPE * (1 - TOP * TOP * TOP)) / 3;
const GRAVITY = 9.8;

/** Height of the pile at t = r / R, as a share of SLOPE · R. */
const profile = (t: number) => (t >= 1 ? 0 : 1 - Math.max(t, TOP));
const radiusForVolume = (v: number) => Math.cbrt(Math.max(v, 0) / VOL_K);

interface Layout {
  R: number;
  H: number;
  /** Dragon scale. */
  s: number;
  /** Dragon maturity 0..1. */
  age: number;
  /** Radius the tail tip rests at, and of the ring the dragon retreats to. */
  rhoTip: number;
  retreatR: number;
  /** Radius of the domain floor. */
  domainR: number;
  heightAt(x: number, z: number): number;
}

/* ════════════════════════════════ Treasure kinds ══════════════════════════════ */

interface KindDef {
  id: string;
  type: TreasureType;
  /** Share of its treasure type that uses this shape. */
  share: number;
  /** Effective volume one item adds to the pile (includes packing gaps). */
  volume: number;
  /** How far the item's origin rests above the surface. */
  lift: number;
  size: [number, number];
  /** Max resting tilt in radians (π = any orientation). */
  tilt: number;
  /** Fraction planted point-down, and how far those are raised. */
  upright?: [number, number];
  /** Phong shininess and specular strength. */
  shine: number;
  spec: number;
  glint: number;
  glow: number;
  geometry(): THREE.BufferGeometry;
  /** Colors a piece's tinted parts are drawn from, one picked per piece. */
  palette(): THREE.Color[];
}

interface Piece {
  geo: THREE.BufferGeometry;
  color: number;
  /** 1 = multiplied by the per-instance tint, 0 = keeps its own color. */
  tint: number;
}

const piece = (geo: THREE.BufferGeometry, color: number, tint = 0): Piece => ({ geo, color, tint });

/** Merge primitives into one non-indexed geometry with `color` and `aTint` attributes. */
function mergePieces(pieces: Piece[]): THREE.BufferGeometry {
  const pos: number[] = [];
  const col: number[] = [];
  const tint: number[] = [];
  const c = new THREE.Color();
  for (const p of pieces) {
    const g = p.geo.index ? p.geo.toNonIndexed() : p.geo;
    const a = g.getAttribute('position');
    c.set(p.color);
    for (let i = 0; i < a.count; i++) {
      pos.push(a.getX(i), a.getY(i), a.getZ(i));
      col.push(c.r, c.g, c.b);
      tint.push(p.tint);
    }
    if (g !== p.geo) g.dispose();
    p.geo.dispose();
  }
  const out = new THREE.BufferGeometry();
  out.setAttribute('position', new THREE.Float32BufferAttribute(pos, 3));
  out.setAttribute('color', new THREE.Float32BufferAttribute(col, 3));
  out.setAttribute('aTint', new THREE.Float32BufferAttribute(tint, 1));
  return out;
}

const PALETTE = 16;
/** Hue, saturation and value to a (linear) color. */
function hsvColor(h: number, sat: number, val: number): THREE.Color {
  const ch = (o: number) => {
    const x = (((h + o) % 1) + 1) % 1;
    return lerp(1, clamp01(Math.abs(x * 6 - 3) - 1), sat) * val;
  };
  return new THREE.Color().setRGB(ch(0), ch(2 / 3), ch(1 / 3));
}
/** n colors running from one to another. */
function shades(from: number, to: number, n: number): THREE.Color[] {
  return Array.from({ length: n }, (_, i) => new THREE.Color(from).lerp(new THREE.Color(to), n > 1 ? i / (n - 1) : 0));
}

const KINDS: KindDef[] = [
  {
    id: 'coin', type: 'gold', share: 0.9, volume: 4.0e-4, lift: 0.012, size: [0.85, 1.15], tilt: 0.7,
    shine: 70, spec: 0.9, glint: 1.4, glow: 0,
    geometry() {
      const top = new THREE.ConeGeometry(0.07, 0.009, 7, 1, true);
      top.translate(0, 0.0045, 0);
      const bottom = new THREE.ConeGeometry(0.07, 0.009, 7, 1, true);
      bottom.rotateX(Math.PI);
      bottom.translate(0, -0.0045, 0);
      return mergePieces([piece(top, 0xffffff, 1), piece(bottom, 0xffffff, 1)]);
    },
    palette() {
      return [...shades(0xffb81c, 0xffdc6a, 14), new THREE.Color(0xd9dee6), new THREE.Color(0xcf7a45)];
    },
  },
  {
    id: 'ingot', type: 'gold', share: 0.1, volume: 1.4e-3, lift: 0.03, size: [0.85, 1.2], tilt: 0.35,
    shine: 80, spec: 0.9, glint: 0.8, glow: 0,
    geometry() {
      const bar = new THREE.CylinderGeometry(0.07, 0.095, 0.06, 4);
      bar.rotateY(Math.PI / 4);
      bar.scale(1.9, 1, 1);
      return mergePieces([piece(bar, 0xffffff, 1)]);
    },
    palette() {
      return [...shades(0xf2a81a, 0xffcf48, 14), new THREE.Color(0xd5dbe3), new THREE.Color(0xd5dbe3)];
    },
  },
  {
    id: 'gem', type: 'gems', share: 1, volume: 5.0e-4, lift: 0.045, size: [0.7, 1.4], tilt: Math.PI,
    shine: 110, spec: 1.0, glint: 1.6, glow: 0.08,
    geometry() {
      const g = new THREE.OctahedronGeometry(0.06, 0);
      g.scale(1, 1.35, 1);
      return mergePieces([piece(g, 0xffffff, 1)]);
    },
    palette() {
      // ruby, emerald, sapphire, amethyst, diamond, aquamarine
      return [hsvColor(0.0, 0.96, 0.8), hsvColor(0.38, 0.9, 0.85), hsvColor(0.6, 0.86, 1), hsvColor(0.77, 0.82, 1), hsvColor(0, 0, 1), hsvColor(0.5, 0.8, 0.95)];
    },
  },
  {
    id: 'book', type: 'books', share: 1, volume: 2.2e-3, lift: 0.028, size: [0.8, 1.25], tilt: 0.3,
    shine: 8, spec: 0.05, glint: 0, glow: 0,
    geometry() {
      // Box faces come in the order +x −x +y −y +z −z. Covers and spine take the tint; the rest are pages.
      const cover = new THREE.BoxGeometry(0.2, 0.05, 0.15);
      const pages = new THREE.BoxGeometry(0.19, 0.036, 0.142);
      pages.translate(0.008, 0, 0);
      const clasp = new THREE.BoxGeometry(0.03, 0.054, 0.154);
      clasp.translate(-0.07, 0, 0);
      const g = cover.toNonIndexed();
      const p = g.getAttribute('position');
      const keep: number[] = [];
      for (let f = 0; f < 6; f++) {
        if (f === 0 || f === 4 || f === 5) continue; // open the page sides
        for (let i = 0; i < 6; i++) keep.push(p.getX(f * 6 + i), p.getY(f * 6 + i), p.getZ(f * 6 + i));
      }
      cover.dispose();
      g.dispose();
      const shell = new THREE.BufferGeometry();
      shell.setAttribute('position', new THREE.Float32BufferAttribute(keep, 3));
      return mergePieces([piece(shell, 0xffffff, 1), piece(pages, 0xe9dfc4), piece(clasp, 0xd9a520)]);
    },
    palette() {
      // oxblood, leather, forest, navy, plum, tan
      return [0.99, 0.07, 0.36, 0.61, 0.8, 0.1].flatMap((h) => [hsvColor(h, 0.62, 0.36), hsvColor(h, 0.62, 0.5)]);
    },
  },
  {
    id: 'sword', type: 'weapons', share: 0.6, volume: 1.6e-3, lift: 0.02, size: [0.85, 1.3], tilt: 0.45,
    upright: [0.04, 0.2], shine: 90, spec: 0.9, glint: 0.9, glow: 0,
    geometry() {
      // Lies along +x with the point at +x; the origin sits a little up the blade.
      const blade = new THREE.CylinderGeometry(0.006, 0.03, 0.5, 4, 1, true);
      blade.rotateZ(-Math.PI / 2);
      blade.scale(1, 0.3, 1);
      blade.translate(0.08, 0, 0);
      const guard = new THREE.BoxGeometry(0.028, 0.026, 0.17);
      guard.translate(-0.17, 0, 0);
      const grip = new THREE.CylinderGeometry(0.013, 0.013, 0.11, 4, 1, true);
      grip.rotateZ(-Math.PI / 2);
      grip.translate(-0.235, 0, 0);
      const pommel = new THREE.OctahedronGeometry(0.026, 0);
      pommel.translate(-0.305, 0, 0);
      return mergePieces([
        piece(blade, 0xd4dae4), piece(guard, 0xe0a82a), piece(grip, 0x3b2418), piece(pommel, 0xffffff, 1),
      ]);
    },
    palette() {
      return Array.from({ length: PALETTE }, (_, i) => hsvColor(i / PALETTE, 0.8, 1));
    },
  },
  {
    id: 'axe', type: 'weapons', share: 0.4, volume: 2.0e-3, lift: 0.02, size: [0.85, 1.25], tilt: 0.45,
    upright: [0.02, 0.16], shine: 60, spec: 0.7, glint: 0.6, glow: 0,
    geometry() {
      const haft = new THREE.CylinderGeometry(0.014, 0.017, 0.5, 4, 1, true);
      haft.rotateZ(-Math.PI / 2);
      const bitA = new THREE.CylinderGeometry(0.11, 0.11, 0.022, 3);
      bitA.rotateY(Math.PI);
      bitA.translate(0.17, 0, 0.075);
      const bitB = new THREE.CylinderGeometry(0.11, 0.11, 0.022, 3);
      bitB.translate(0.17, 0, -0.075);
      return mergePieces([piece(haft, 0x6b4423), piece(bitA, 0xffffff, 1), piece(bitB, 0xffffff, 1)]);
    },
    palette() {
      return shades(0xaab4c2, 0xdfe5ee, PALETTE);
    },
  },
  {
    id: 'skull', type: 'bones', share: 0.25, volume: 2.4e-3, lift: 0.07, size: [0.85, 1.25], tilt: 0.6,
    shine: 10, spec: 0.08, glint: 0, glow: 0,
    geometry() {
      const cranium = new THREE.IcosahedronGeometry(0.085, 0);
      cranium.scale(0.88, 1, 1.05);
      const jaw = new THREE.BoxGeometry(0.085, 0.045, 0.07);
      jaw.translate(0, -0.075, 0.035);
      const socket = (x: number) => {
        const s = new THREE.TetrahedronGeometry(0.03, 0);
        s.translate(x, 0.005, 0.078);
        return piece(s, 0x17110d);
      };
      const nose = new THREE.TetrahedronGeometry(0.016, 0);
      nose.translate(0, -0.034, 0.088);
      return mergePieces([piece(cranium, 0xffffff, 1), piece(jaw, 0xffffff, 1), socket(-0.034), socket(0.034), piece(nose, 0x17110d)]);
    },
    palette() {
      return shades(0xcbbf9f, 0xf3ecd8, PALETTE);
    },
  },
  {
    id: 'bone', type: 'bones', share: 0.75, volume: 9.0e-4, lift: 0.022, size: [0.8, 1.5], tilt: 0.4,
    shine: 10, spec: 0.08, glint: 0, glow: 0,
    geometry() {
      const shaft = new THREE.CylinderGeometry(0.014, 0.014, 0.2, 4, 1, true);
      shaft.rotateZ(Math.PI / 2);
      const knob = (x: number) => {
        const k = new THREE.OctahedronGeometry(0.03, 0);
        k.scale(0.8, 1, 1.25);
        k.translate(x, 0, 0);
        return piece(k, 0xffffff, 1);
      };
      return mergePieces([piece(shaft, 0xffffff, 1), knob(-0.105), knob(0.105)]);
    },
    palette() {
      return shades(0xc9bd9c, 0xf1ead4, PALETTE);
    },
  },
  {
    id: 'tablet', type: 'tablets', share: 1, volume: 1.8e-3, lift: 0.028, size: [0.85, 1.25], tilt: 0.4,
    upright: [0.18, 0.2], shine: 6, spec: 0.05, glint: 0, glow: 0.7,
    geometry() {
      // A stone slab lying along x, with one rune cut into its face. Only the rune takes the tint, and glows.
      const slab = new THREE.BoxGeometry(0.27, 0.045, 0.18);
      const stroke = (len: number, x: number, z: number, turn: number) => {
        const g = new THREE.BoxGeometry(len, 0.01, 0.02);
        g.rotateY(turn);
        g.translate(x, 0.024, z);
        return piece(g, 0x8a8a8a, 1);
      };
      return mergePieces([
        piece(slab, 0x6c707c),
        stroke(0.18, 0, -0.03, 0),
        stroke(0.09, 0.035, 0.008, -0.75),
        stroke(0.09, -0.03, 0.008, -0.75),
      ]);
    },
    palette() {
      // cyan, violet, amber, green
      return [hsvColor(0.5, 0.75, 1), hsvColor(0.76, 0.75, 1), hsvColor(0.09, 0.75, 1), hsvColor(0.36, 0.75, 1)];
    },
  },
  {
    id: 'painting', type: 'paintings', share: 1, volume: 4.0e-3, lift: 0.022, size: [0.8, 1.3], tilt: 0.35,
    upright: [0.25, 0.14], shine: 24, spec: 0.25, glint: 0, glow: 0,
    geometry() {
      // A gilt frame around a canvas of two bands and a sun; the bands take the tint.
      const frame = new THREE.BoxGeometry(0.38, 0.03, 0.29);
      const sky = new THREE.BoxGeometry(0.16, 0.036, 0.23);
      sky.translate(0.08, 0, 0);
      const land = new THREE.BoxGeometry(0.16, 0.036, 0.23);
      land.translate(-0.08, 0, 0);
      const sun = new THREE.BoxGeometry(0.05, 0.04, 0.05);
      sun.translate(0.09, 0, 0.05);
      return mergePieces([piece(frame, 0xc9962a), piece(sky, 0xffffff, 1), piece(land, 0x595959, 1), piece(sun, 0xfff0b0)]);
    },
    palette() {
      return Array.from({ length: PALETTE }, (_, i) => hsvColor(i / PALETTE, 0.55, 0.85));
    },
  },
  {
    id: 'potion', type: 'potions', share: 1, volume: 1.0e-3, lift: 0.05, size: [0.6, 0.95], tilt: 0.5,
    upright: [0.4, 0.3], shine: 100, spec: 0.9, glint: 0.9, glow: 0.16,
    geometry() {
      // A round flask lying along x with its neck toward −x, so a planted one stands neck-up.
      // The liquid takes the tint and glows; neck and cork keep their own colors.
      const flask = new THREE.IcosahedronGeometry(0.075, 0);
      flask.scale(1, 0.92, 1);
      flask.translate(0.03, 0, 0);
      const neck = new THREE.CylinderGeometry(0.024, 0.03, 0.08, 5, 1, true);
      neck.rotateZ(Math.PI / 2);
      neck.translate(-0.075, 0, 0);
      const lip = new THREE.CylinderGeometry(0.034, 0.034, 0.014, 5);
      lip.rotateZ(Math.PI / 2);
      lip.translate(-0.115, 0, 0);
      const cork = new THREE.CylinderGeometry(0.02, 0.024, 0.04, 5);
      cork.rotateZ(Math.PI / 2);
      cork.translate(-0.14, 0, 0);
      return mergePieces([piece(flask, 0xffffff, 1), piece(neck, 0xcfe6ea), piece(lip, 0xcfe6ea), piece(cork, 0x8a5a2b)]);
    },
    palette() {
      // healing red, mana blue, poison green, arcane violet, sun gold, frost cyan
      return [0.99, 0.6, 0.33, 0.78, 0.13, 0.5].map((h) => hsvColor(h, 0.92, 0.85));
    },
  },
];

/* ════════════════════════════════ GPU treasure ════════════════════════════════
 * Every asset is one numbered piece, and each shape is spread evenly through the
 * whole pile: piece i of a shape with n pieces rests on the surface the pile had when
 * it was (i + ½) / n of its present size. So the newest pieces of every shape are the
 * ones on the surface, in the same proportions as the collection, and where a piece
 * rests depends only on the collection as it is now, never on the order things came.
 *
 * Only the pieces near the surface are drawn: the newest of each shape, instanced.
 * Where each one is gets worked out in the vertex shader from its number and a few
 * uniforms, so there are no buffers to fill or allocate. A newly landed piece appears
 * where the stream hits the crown and runs down the slope to its place. The falling
 * stream is a small recycled pool of the same pieces.
 *
 * Every shape, resting or falling, is drawn with ONE material. What differs between
 * shapes (size, tilt, palette, counts) is fed in as per-object uniforms, so the GPU
 * compiles a single treasure shader however many kinds of treasure there are.         */

interface SharedUniforms {
  time: N;
  /** Portal mouth (hoard space), radius of the falling stream, and speed pieces leave at. */
  portal: N; streamR: N; exitSpeed: N;
  /** World-space plane of the portal mouth (xyz normal, w offset). The stream is not drawn beyond it. */
  clip: N;
  /** Volume of the pile as landed so far. */
  volume: N;
  /** The pour's clock, how long a piece takes to fall to the crown, when the stream stops, and the stream's recycle period. */
  pourT: N; fall: N; pourEnd: N; period: N;
  /** 0..1 as the treasure first appears: pieces pop up one after another instead of all at once. */
  reveal: N;
}

const PALETTE_SIZE = 16;
const profileNode = (t: N): N => float(1).sub(t.max(TOP));

const rotAxis = (v: N, axis: N, angle: N): N => {
  const c: N = cosN(angle);
  const s: N = sinN(angle);
  return v.mul(c).add(axis.cross(v).mul(s)).add(axis.mul(axis.dot(v)).mul(float(1).sub(c)));
};

/** The one material all treasure is drawn with. Per-shape values arrive through each mesh's `userData`. */
function buildTreasureMaterial(U: SharedUniforms): THREE.MeshPhongNodeMaterial {
  const colors: THREE.Color[] = [];
  for (const kind of KINDS) {
    const row = kind.palette();
    for (let i = 0; i < PALETTE_SIZE; i++) colors.push(row[i % row.length]);
  }
  const palette: N = uniformArray(colors, 'color');

  const kA: N = userData('kA', 'vec4'); // salt, lift, smallest size, largest size
  const kB: N = userData('kB', 'vec4'); // tilt, share planted upright, how far those are raised, palette row
  const kC: N = userData('kC', 'vec4'); // shininess, specular strength, glint, glow
  const sA: N = userData('sA', 'vec4'); // pieces landed, newest index + 1, first index of this pour, pieces per second
  const sB: N = userData('sB', 'vec4'); // longest run down the slope, stream pool size, 1 for the stream
  const salt: N = kA.x.toUint();
  const isStream: N = sB.z.greaterThan(0.5);
  const slot: N = instanceIndex.toFloat();

  // ── a resting piece: slot 0 is the newest landed, slot 1 the one before it, and so on down ──
  const fi: N = sA.y.sub(1).sub(slot).max(0);
  const idR: N = fi.toUint().add(salt);
  // Its level: how far through the pile this shape has got by this piece.
  const level: N = fi.add(0.5).div(sA.x.max(fi.add(1)));
  const rp: N = U.volume.mul(level).div(VOL_K).pow(1 / 3);
  const th: N = hash(idR.add(uint(101))).mul(TAU);
  const rRest: N = rp.mul(hash(idR.add(uint(201))).sqrt());
  const lift: N = hash(idR.add(uint(103))).mul(0.6).add(0.7).mul(kA.y);
  // A piece that landed during this pour starts where the stream hit and runs down to its place.
  const rHit: N = U.streamR.mul(float(1).sub(rRest.div(U.streamR.max(1e-3)).negate().exp()));
  const run: N = rRest.sub(rHit).mul(Math.hypot(1, SLOPE));
  const runTime: N = run.sqrt().mul(0.5).add(0.25).min(sB.x);
  const landedAt: N = U.fall.add(fi.sub(sA.z).div(sA.w.max(1e-6)));
  const fresh: N = sA.w.greaterThan(0).and(fi.greaterThanEqual(sA.z));
  const u: N = select(fresh, U.pourT.sub(landedAt).max(0).div(runTime).min(1), float(1));
  const left: N = float(1).sub(u);
  const r: N = mix(rHit, rRest, float(1).sub(left.mul(left))); // fast off the crown, slowing to a stop
  const hops: N = hash(idR.add(uint(405))).mul(2).floor().add(2);
  const hop: N = sinN(u.mul(hops).mul(Math.PI)).abs().mul(left.mul(left)).mul(run.min(1.5).mul(0.22));
  const yRest: N = rp.mul(SLOPE).mul(profileNode(r.div(rp.max(1e-4)).min(1))).add(lift).add(hop);
  const posR: N = vec3(cosN(th).mul(r), yRest, sinN(th).mul(r));
  const angR: N = hash(idR.add(uint(401))).sub(0.5).mul(9).mul(left.mul(left));

  // ── a stream piece: drops from the portal to the crown, then goes round again ──
  const sid: N = instanceIndex.add(salt).add(uint(7777));
  const start: N = slot.add(hash(sid)).div(sB.y.max(1)).mul(U.period);
  const clock: N = U.pourT.sub(start);
  const trip: N = clock.div(U.period).floor().max(0);
  const age: N = clock.sub(trip.mul(U.period));
  const live: N = clock.greaterThanEqual(0).and(U.pourT.sub(age).lessThanEqual(U.pourEnd)).and(age.lessThan(U.fall));
  const idS: N = sid.add(trip.toUint().mul(uint(7919)));
  const ang: N = hash(idS.add(uint(23))).mul(TAU);
  // It leaves the mouth tight and loosens as it falls.
  const rad: N = hash(idS.add(uint(29))).sqrt().mul(U.streamR).mul(mix(float(0.7), float(1), age.div(U.fall.max(1e-3)).min(1).pow(1.5)));
  const yFall: N = U.portal.y.add(0.3).sub(U.exitSpeed.mul(age)).sub(age.mul(age).mul(GRAVITY / 2));
  const posS: N = vec3(cosN(ang).mul(rad), yFall, sinN(ang).mul(rad));
  const angS: N = hash(idS.add(uint(403))).mul(TAU).add(hash(idS.add(uint(401))).sub(0.5).mul(11).mul(age));

  // ── dress it: orientation, size and color from a hash of its number ──
  const id: N = select(isStream, idS, idR);
  const at: N = select(isStream, posS, posR);
  const angle: N = select(isStream, angS, angR);
  const shown: N = select(isStream, select(live, float(1), float(0)), float(1));
  const h1: N = hash(id);
  const h2: N = hash(id.add(uint(17)));
  const h3: N = hash(id.add(uint(29)));
  const h4: N = hash(id.add(uint(41)));
  const h5: N = hash(id.add(uint(53)));
  const scale: N = mix(kA.z, kA.w, h3).mul(shown).mul(U.reveal.mul(1.6).sub(h2.mul(0.6)).clamp(0, 1));
  const planted: N = h5.lessThan(kB.y);
  const roll: N = select(planted, h4.sub(0.5).mul(0.6).sub(Math.PI / 2), h4.sub(0.5).mul(2).mul(kB.x));
  let local: N = positionGeometry.mul(scale);
  local = rotAxis(local, vec3(1, 0, 0), h2.sub(0.5).mul(2).mul(kB.x));
  local = rotAxis(local, vec3(0, 0, 1), roll);
  local = rotAxis(local, vec3(h1, h2, h3).sub(0.5).add(vec3(0.001, 0, 0)).normalize(), angle);
  local = rotAxis(local, vec3(0, 1, 0), h1.mul(TAU));

  const material = new THREE.MeshPhongNodeMaterial({ flatShading: true });
  material.positionNode = local.add(at).add(vec3(0, select(planted, scale.mul(kB.z), float(0)), 0));
  const tint: N = palette.element(kB.w.mul(PALETTE_SIZE).add(h1.mul(PALETTE_SIZE).floor()).toInt()).mul(mix(float(0.85), float(1.1), hash(id.add(uint(67)))));
  const mask: N = attribute('aTint', 'float');
  const base: N = attribute('color', 'vec3').mul(mix(vec3(1, 1, 1), tint, mask));
  material.colorNode = base;
  material.shininessNode = kC.x;
  material.specularNode = mix(vec3(1, 1, 1), base, 0.5).mul(kC.y);
  const twinkle: N = sinN(U.time.mul(h2.mul(2).add(1.5)).add(h4.mul(TAU))).mul(0.5).add(0.5).pow(40);
  material.emissiveNode = tint.mul(mask).mul(select(h3.lessThan(0.35), twinkle.mul(kC.z), float(0)).add(kC.w));
  // One-way portal: a falling piece is only drawn where it has come through the mouth.
  material.maskNode = isStream.not().or(positionWorld.dot(U.clip.xyz).add(U.clip.w).lessThan(0));
  return material;
}

/** All the pieces of one treasure shape: the newest ones at rest, and its share of the stream. */
class KindLayer {
  readonly def: KindDef;
  /** Pieces of this shape at rest in the pile, buried ones included (not counting a pour still running). */
  count = 0;

  private readonly geometry: THREE.BufferGeometry;
  private readonly rest: THREE.Mesh;
  private readonly stream: THREE.Mesh;
  private readonly state = new THREE.Vector4(); // landed, head, from, rate
  private readonly restB = new THREE.Vector4(10, 1, 0, 0);
  private readonly streamB = new THREE.Vector4(10, 1, 1, 0);

  private readonly parent: THREE.Object3D;

  constructor(def: KindDef, index: number, material: THREE.Material, parent: THREE.Object3D, seed: number) {
    this.def = def;
    this.parent = parent;
    this.geometry = def.geometry();
    const salt = ((index + 1) * 7919 + seed * 104729) % 1000003;
    const up = def.upright ?? [0, 0];
    const kA = new THREE.Vector4(salt, def.lift, def.size[0], def.size[1]);
    const kB = new THREE.Vector4(def.tilt, up[0], up[1], index);
    const kC = new THREE.Vector4(def.shine, def.spec, def.glint, def.glow);
    const make = (name: string, sB: THREE.Vector4) => {
      const mesh = new THREE.Mesh(this.geometry, material);
      mesh.name = name;
      mesh.userData = { kA, kB, kC, sA: this.state, sB };
      mesh.frustumCulled = false;
      mesh.receiveShadow = true;
      mesh.count = 1;
      mesh.visible = false;
      parent.add(mesh);
      return mesh;
    };
    this.rest = make(`hoard-${def.id}`, this.restB);
    this.stream = make(`hoard-stream-${def.id}`, this.streamB);
  }

  /** n pieces, all at rest. */
  setRest(n: number): void {
    this.count = n;
    this.state.set(n, n, n, 0);
  }

  /** Pieces `from`..`to` arrive over `duration`; a piece's run down the slope takes at most `runCap` seconds. */
  beginPour(from: number, to: number, duration: number, runCap: number, pool: number): void {
    this.state.z = from;
    this.state.w = (to - from) / duration;
    this.restB.x = runCap;
    this.streamB.y = Math.max(1, pool);
    this.stream.count = Math.max(1, pool);
  }

  /** Set how many pieces have landed and how many of the newest to draw. Returns how many are drawn. */
  advance(landed: number, visible: number, streaming: boolean, show: boolean): number {
    const head = Math.ceil(landed);
    this.state.x = landed;
    this.state.y = head;
    const n = Math.max(0, Math.min(visible, head));
    this.rest.count = Math.max(1, n);
    this.rest.visible = show && n > 0;
    this.stream.visible = show && streaming && this.state.w > 0;
    return n + (this.stream.visible ? this.stream.count : 0);
  }

  /** The meshes, for compiling ahead of need. */
  get meshes(): THREE.Mesh[] {
    return [this.rest, this.stream];
  }

  /** A stand-in outside the scene with the same shader, for compiling it while the real ones stay hidden. */
  proxy(): THREE.Mesh {
    const mesh = new THREE.Mesh(this.geometry, this.rest.material);
    mesh.userData = this.rest.userData;
    mesh.frustumCulled = false;
    mesh.receiveShadow = true;
    mesh.count = 2;
    return mesh;
  }

  dispose(): void {
    this.parent.remove(this.rest, this.stream);
    this.geometry.dispose();
  }
}

/* ════════════════════════════════ Triangle buffer ═════════════════════════════
 * Growable soup of colored triangles. The dragon is rewritten into one of these
 * every frame, which is what lets it bend and breathe.                           */

const _d = new THREE.Vector3();
const _u = new THREE.Vector3();
const _w = new THREE.Vector3();
const _ref = new THREE.Vector3();

class TriBuffer {
  pos: Float32Array;
  col: Float32Array;
  count = 0;

  constructor(capacity = 1024) {
    this.pos = new Float32Array(capacity * 3);
    this.col = new Float32Array(capacity * 3);
  }

  reset(): void {
    this.count = 0;
  }

  private reserve(extra: number): void {
    const need = (this.count + extra) * 3;
    if (need <= this.pos.length) return;
    let cap = this.pos.length * 2;
    while (cap < need) cap *= 2;
    const p = new Float32Array(cap);
    p.set(this.pos.subarray(0, this.count * 3));
    this.pos = p;
    const c = new Float32Array(cap);
    c.set(this.col.subarray(0, this.count * 3));
    this.col = c;
  }

  raw(
    ax: number, ay: number, az: number, bx: number, by: number, bz: number, cx: number, cy: number, cz: number,
    r: number, g: number, b: number,
  ): void {
    this.reserve(3);
    const p = this.pos;
    const q = this.col;
    const o = this.count * 3;
    p[o] = ax; p[o + 1] = ay; p[o + 2] = az;
    p[o + 3] = bx; p[o + 4] = by; p[o + 5] = bz;
    p[o + 6] = cx; p[o + 7] = cy; p[o + 8] = cz;
    for (let i = 0; i < 9; i += 3) { q[o + i] = r; q[o + i + 1] = g; q[o + i + 2] = b; }
    this.count += 3;
  }

  tri(a: THREE.Vector3, b: THREE.Vector3, c: THREE.Vector3, color: THREE.Color, mul = 1): void {
    this.raw(a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z, color.r * mul, color.g * mul, color.b * mul);
  }

  /** Tapered prism from a (radius ra) to b (radius rb). */
  prism(a: THREE.Vector3, b: THREE.Vector3, ra: number, rb: number, sides: number, color: THREE.Color, mul = 1, caps = false): void {
    _d.subVectors(b, a);
    const len = _d.length();
    if (len < 1e-6) return;
    _d.divideScalar(len);
    _ref.set(0, 1, 0);
    if (Math.abs(_d.y) > 0.9) _ref.set(1, 0, 0);
    _u.crossVectors(_d, _ref).normalize();
    _w.crossVectors(_d, _u);
    const r = color.r * mul, g = color.g * mul, bl = color.b * mul;
    let pax = 0, pay = 0, paz = 0, pbx = 0, pby = 0, pbz = 0;
    for (let k = 0; k <= sides; k++) {
      const ang = (k / sides) * TAU;
      const c = Math.cos(ang), s = Math.sin(ang);
      const ox = _u.x * c + _w.x * s, oy = _u.y * c + _w.y * s, oz = _u.z * c + _w.z * s;
      const ax = a.x + ox * ra, ay = a.y + oy * ra, az = a.z + oz * ra;
      const bx = b.x + ox * rb, by = b.y + oy * rb, bz = b.z + oz * rb;
      if (k > 0) {
        const shade = 0.9 + 0.1 * ((k * 3) % 4) / 3;
        this.raw(pax, pay, paz, ax, ay, az, bx, by, bz, r * shade, g * shade, bl * shade);
        if (rb > 0) this.raw(pax, pay, paz, bx, by, bz, pbx, pby, pbz, r * shade, g * shade, bl * shade);
        if (caps) {
          this.raw(a.x, a.y, a.z, ax, ay, az, pax, pay, paz, r * 0.85, g * 0.85, bl * 0.85);
          if (rb > 0) this.raw(b.x, b.y, b.z, pbx, pby, pbz, bx, by, bz, r, g, bl);
        }
      }
      pax = ax; pay = ay; paz = az; pbx = bx; pby = by; pbz = bz;
    }
  }

  /** Skin consecutive rings of equal vertex count. */
  loft(rings: THREE.Vector3[][], colorOf: (face: number, span: number) => THREE.Color, capStart?: THREE.Color, capEnd?: THREE.Color): void {
    for (let i = 0; i < rings.length - 1; i++) {
      const a = rings[i], b = rings[i + 1];
      for (let k = 0; k < a.length; k++) {
        const k1 = (k + 1) % a.length;
        const c = colorOf(k, i);
        this.tri(a[k], a[k1], b[k1], c);
        this.tri(a[k], b[k1], b[k], c);
      }
    }
    const cap = (ring: THREE.Vector3[], color: THREE.Color) => {
      const mid = new THREE.Vector3();
      for (const v of ring) mid.add(v);
      mid.divideScalar(ring.length);
      for (let k = 0; k < ring.length; k++) this.tri(mid, ring[k], ring[(k + 1) % ring.length], color);
    };
    if (capStart) cap(rings[0], capStart);
    if (capEnd) cap(rings[rings.length - 1], capEnd);
  }

  toGeometry(): THREE.BufferGeometry {
    const g = new THREE.BufferGeometry();
    g.setAttribute('position', new THREE.BufferAttribute(this.pos.slice(0, this.count * 3), 3));
    g.setAttribute('color', new THREE.BufferAttribute(this.col.slice(0, this.count * 3), 3));
    return g;
  }
}

/* ════════════════════════════════ Smoke and fire ══════════════════════════════ */

class Puffs {
  readonly mesh: THREE.InstancedMesh;
  private readonly n = 128;
  private readonly p = new Float32Array(this.n * 3);
  private readonly v = new Float32Array(this.n * 3);
  private readonly life = new Float32Array(this.n);
  private readonly span = new Float32Array(this.n);
  private readonly size = new Float32Array(this.n);
  private readonly fire = new Uint8Array(this.n);
  private cursor = 0;
  private readonly m = new THREE.Matrix4();
  private readonly q = new THREE.Quaternion();
  private readonly e = new THREE.Euler();
  private readonly sc = new THREE.Vector3();
  private readonly ps = new THREE.Vector3();
  private readonly c = new THREE.Color();
  active = 0;

  constructor() {
    const geo = new THREE.IcosahedronGeometry(1, 0);
    const mat = new THREE.MeshBasicMaterial({ color: 0xffffff });
    this.mesh = new THREE.InstancedMesh(geo, mat, this.n);
    this.mesh.frustumCulled = false;
    this.mesh.name = 'dragon-breath';
    this.m.makeScale(0, 0, 0);
    for (let i = 0; i < this.n; i++) {
      this.mesh.setMatrixAt(i, this.m);
      this.mesh.setColorAt(i, this.c.setRGB(1, 1, 1));
    }
  }

  emit(fire: boolean, pos: THREE.Vector3, vel: THREE.Vector3, span: number, size: number): void {
    const i = this.cursor;
    this.cursor = (this.cursor + 1) % this.n;
    this.p[i * 3] = pos.x; this.p[i * 3 + 1] = pos.y; this.p[i * 3 + 2] = pos.z;
    this.v[i * 3] = vel.x; this.v[i * 3 + 1] = vel.y; this.v[i * 3 + 2] = vel.z;
    this.life[i] = span;
    this.span[i] = span;
    this.size[i] = size;
    this.fire[i] = fire ? 1 : 0;
  }

  update(dt: number): void {
    let active = 0;
    for (let i = 0; i < this.n; i++) {
      if (this.life[i] <= 0) continue;
      this.life[i] -= dt;
      const k = 1 - Math.max(0, this.life[i]) / this.span[i];
      const o = i * 3;
      const drag = Math.exp(-dt * (this.fire[i] ? 1.6 : 0.9));
      this.v[o] *= drag; this.v[o + 2] *= drag;
      this.v[o + 1] = this.v[o + 1] * drag + dt * (this.fire[i] ? 0.6 : 0.5) * this.size[i] * 6;
      this.p[o] += this.v[o] * dt; this.p[o + 1] += this.v[o + 1] * dt; this.p[o + 2] += this.v[o + 2] * dt;
      let s: number;
      if (this.life[i] <= 0) {
        s = 0;
      } else if (this.fire[i]) {
        s = this.size[i] * (0.7 + 1.3 * k) * Math.sqrt(1 - k);
        if (k < 0.3) this.c.setRGB(1, 0.8 - k * 1.2, 0.3 - k * 0.8);
        else this.c.setRGB(1 - (k - 0.3) * 0.8, 0.44 - (k - 0.3) * 0.55, 0.06);
        active++;
      } else {
        s = this.size[i] * (0.6 + 1.6 * k) * Math.min(1, (1 - k) * 3);
        const gr = 0.5 - k * 0.2;
        this.c.setRGB(gr, gr, gr * 1.08);
        active++;
      }
      this.e.set(i * 1.3 + k * 2, i * 0.7 + k * 3, 0);
      this.q.setFromEuler(this.e);
      this.m.compose(this.ps.set(this.p[o], this.p[o + 1], this.p[o + 2]), this.q, this.sc.set(s, s, s));
      this.mesh.setMatrixAt(i, this.m);
      this.mesh.setColorAt(i, this.c);
    }
    this.active = active;
    this.mesh.instanceMatrix.needsUpdate = true;
    if (this.mesh.instanceColor) this.mesh.instanceColor.needsUpdate = true;
  }

  dispose(): void {
    this.mesh.geometry.dispose();
    (this.mesh.material as THREE.Material).dispose();
    this.mesh.dispose();
  }
}

/* ════════════════════════════════ The dragon ══════════════════════════════════
 * The body is a tube skinned along a spline: tail spiralling round the hoard,
 * torso curled behind the peak, neck rising to the head. Legs, wings and spikes
 * hang off the same frames, so everything follows the pile as it grows.          */

const TH_SHOULDER = 20 * DEG;
const TH_TIP = -235 * DEG;
const A_HIP = 145 / 255;
const SPIRAL_POINTS = 12;
const CONTROL_POINTS = SPIRAL_POINTS + 4;
const RINGS = (CONTROL_POINTS - 1) * 3 + 1;
const RING_SIDES = 8;
const RING_SHOULDER = 33;
const RING_WING = 31;
const RING_FORELEG = 32;
const RING_HINDLEG = 21;
const RING_TRAIL = 23;
const ROAR_SECONDS = 2.6;

// Wing bone directions in (forward, up, out) of the shoulder frame: folded → spread.
// Bones: upper arm, forearm, then four fingers from the leading edge back.
const WING_FOLDED: ReadonlyArray<readonly [number, number, number]> = [
  [-0.45, 0.85, 0.25], [0.7, 0.55, 0.15], [-0.85, 0.3, 0.08], [-0.97, 0.06, 0.1], [-0.96, -0.18, 0.12], [-0.88, -0.42, 0.14],
];
const WING_SPREAD: ReadonlyArray<readonly [number, number, number]> = [
  [-0.15, 0.6, 0.8], [0.3, 0.45, 0.85], [0.25, 0.38, 0.95], [-0.25, 0.14, 0.95], [-0.62, -0.08, 0.78], [-0.92, -0.28, 0.4],
];
const WING_LENGTHS = [0.62, 0.72, 1.4, 1.4, 1.22, 0.95];

/** How far past the tail tip the track keeps spiralling out before it reaches the retreat ring. */
const TRACK_EXT = 150 * DEG;
const TRACK_SAMPLES = 900;
const TRACK_STEP = 2 * DEG;
const AIR_SAMPLES = 49;

/**
 * The path the body lies along, as radius by angle: once round the top of the pile
 * (torso), spiralling down it (tail), then on out to a ring clear of the hoard. Sliding
 * the body along it is how the dragon backs off the pile and climbs back up.
 */
function trackRho(th: number, s: number, rhoTip: number, retreatR: number): number {
  if (th >= -90 * DEG) return s;
  if (th >= TH_TIP) return s + (rhoTip - s) * Math.pow((-90 * DEG - th) / (145 * DEG), 1.2);
  return rhoTip + (retreatR - rhoTip) * ease(Math.min(1, (TH_TIP - th) / TRACK_EXT));
}

/** Distance of each spine control point behind the shoulders, per unit of dragon scale. */
const BODY_OFFSETS: number[] = (() => {
  const out: number[] = new Array(SPIRAL_POINTS).fill(0);
  let d = 0, th = TH_SHOULDER, px = Math.cos(th), pz = Math.sin(th);
  for (let j = SPIRAL_POINTS - 2; j >= 0; j--) {
    const goal = lerp(TH_TIP, TH_SHOULDER, j / (SPIRAL_POINTS - 1));
    while (th > goal) {
      th = Math.max(goal, th - 0.5 * DEG);
      const rho = trackRho(th, 1, 2, 2);
      const x = Math.cos(th) * rho, z = Math.sin(th) * rho;
      d += Math.hypot(x - px, z - pz);
      px = x; pz = z;
    }
    out[j] = d;
  }
  return out;
})();

function controlRadius(j: number): number {
  if (j < SPIRAL_POINTS) {
    const a = j / (SPIRAL_POINTS - 1);
    if (a < A_HIP) return 0.03 + 0.4 * Math.pow(a / A_HIP, 0.85);
    const q = (a - A_HIP) / (1 - A_HIP);
    return 0.43 + 0.08 * Math.sin(Math.PI * q) - 0.03 * q;
  }
  return [0.27, 0.19, 0.165, 0.155][j - SPIRAL_POINTS];
}

interface DragonPalette {
  body: THREE.Color; bodyDark: THREE.Color; stripe: THREE.Color; belly: THREE.Color; wing: THREE.Color;
  horn: THREE.Color; mouth: THREE.Color; tooth: THREE.Color;
}

class Dragon {
  readonly group = new THREE.Group();
  readonly puffs = new Puffs();
  readonly fireLight = new THREE.PointLight(0xff8a3c, 0, 0, 2);
  /** Head position in hoard space, for look targets and camera framing. */
  readonly headPos = new THREE.Vector3();

  private readonly buf = new TriBuffer(8192);
  private readonly geo = new THREE.BufferGeometry();
  private readonly mesh: THREE.Mesh;
  private readonly bodyMat: THREE.Material;
  private readonly eyeMat: THREE.MeshBasicMaterial;
  private readonly pupilMat: THREE.MeshBasicMaterial;
  private readonly head = new THREE.Group();
  private readonly jaw = new THREE.Group();
  private readonly skull: THREE.Mesh;
  private readonly jawMesh: THREE.Mesh;
  private readonly eyes: THREE.Group[] = [];
  private headKey = -1;
  private pal: DragonPalette;

  private readonly curve: THREE.CatmullRomCurve3;
  private readonly P: THREE.Vector3[] = [];
  private readonly T: THREE.Vector3[] = [];
  private readonly Nn: THREE.Vector3[] = [];
  private readonly B: THREE.Vector3[] = [];
  private readonly ringR = new Float32Array(RINGS);
  private readonly ringPos = new Float32Array(RINGS * RING_SIDES * 3);
  private readonly ctrlR: number[] = [];
  private readonly tmp: THREE.Vector3[] = [];
  private readonly look = new THREE.Vector3();
  private readonly mat4 = new THREE.Matrix4();

  private readonly trD = new Float32Array(TRACK_SAMPLES);
  private readonly trX = new Float32Array(TRACK_SAMPLES);
  private readonly trZ = new Float32Array(TRACK_SAMPLES);
  private trN = 0;
  /** Track distance from the throne to the retreat ring, as of the last update. */
  trackExtent = 1;
  /** Length of the flight from the throne out to the ring, including the landing run. */
  leaveLength = 1;
  private trackTip = 0; // track distance of the foot of the pile, where the tail tip rests
  private readonly airQ = new Float32Array(AIR_SAMPLES);
  private readonly airX = new Float32Array(AIR_SAMPLES);
  private readonly airY = new Float32Array(AIR_SAMPLES);
  private readonly airZ = new Float32Array(AIR_SAMPLES);
  private readonly ctrlAir = new Float32Array(CONTROL_POINTS);
  private lastLeaving = false;
  /** Shoulder position in hoard space: where the dragon currently is. */
  readonly center = new THREE.Vector3();
  private lastShift = 0;
  private move = 0;
  private gait = 0;

  private yaw = -0.2;
  private yawRel = 0;
  private pitch = -0.1;
  private wake = 1;
  private roarT = -1;
  private blink = 0;
  private nextBlink = 2;
  private nextPuff = 3;
  private fireCarry = 0;
  private wingOpen = 0.2;
  // Idle business: 1 yawn, 2 peer at the hoard, 3 stretch, 4 shake.
  private idleKind = 0;
  private idleT = 0;
  private idleLen = 1;
  private nextIdle = 5;
  private readonly rnd = mulberry32(99);

  /** `matte` is the hoard's shared flat-shaded vertex-color material, so dragon and scenery compile one shader between them. */
  constructor(colors: DragonColors, matte: THREE.Material) {
    this.pal = Dragon.palette(colors);
    this.bodyMat = matte;
    this.eyeMat = new THREE.MeshBasicMaterial({ color: colors.eye });
    this.pupilMat = new THREE.MeshBasicMaterial({ color: 0x120a06 });

    const pts: THREE.Vector3[] = [];
    for (let j = 0; j < CONTROL_POINTS; j++) {
      pts.push(new THREE.Vector3(j, 0, 0));
      this.ctrlR.push(controlRadius(j));
    }
    this.curve = new THREE.CatmullRomCurve3(pts, false, 'centripetal');
    for (let i = 0; i < RINGS; i++) {
      this.P.push(new THREE.Vector3());
      this.T.push(new THREE.Vector3());
      this.Nn.push(new THREE.Vector3());
      this.B.push(new THREE.Vector3());
    }
    for (let i = 0; i < 36; i++) this.tmp.push(new THREE.Vector3());

    this.mesh = new THREE.Mesh(this.geo, this.bodyMat);
    this.mesh.name = 'dragon-body';
    this.mesh.frustumCulled = false;
    this.mesh.castShadow = true;
    this.mesh.receiveShadow = true;

    this.skull = new THREE.Mesh(new THREE.BufferGeometry(), this.bodyMat);
    this.jawMesh = new THREE.Mesh(new THREE.BufferGeometry(), this.bodyMat);
    this.skull.castShadow = this.jawMesh.castShadow = true;
    this.jaw.position.set(0, -0.035, -0.02);
    this.jaw.add(this.jawMesh);
    this.head.add(this.skull, this.jaw);
    const eyeGeo = new THREE.OctahedronGeometry(0.055, 0);
    const pupilGeo = new THREE.BoxGeometry(0.012, 0.07, 0.028);
    for (const side of [-1, 1]) {
      const eye = new THREE.Group();
      const ball = new THREE.Mesh(eyeGeo, this.eyeMat);
      ball.scale.set(0.45, 0.8, 1.25);
      const pupil = new THREE.Mesh(pupilGeo, this.pupilMat);
      pupil.position.x = side * 0.024;
      eye.add(ball, pupil);
      eye.position.set(side * 0.19, 0.105, 0.2);
      this.head.add(eye);
      this.eyes.push(eye);
    }
    this.group.add(this.mesh, this.head, this.puffs.mesh, this.fireLight);
    this.group.name = 'dragon';
  }

  private static palette(c: DragonColors): DragonPalette {
    const body = new THREE.Color(c.body);
    return {
      body,
      bodyDark: body.clone().multiplyScalar(0.72),
      stripe: body.clone().offsetHSL(-0.02, 0, 0).multiplyScalar(0.5),
      belly: new THREE.Color(c.belly),
      wing: new THREE.Color(c.wing),
      horn: new THREE.Color(c.horn),
      mouth: new THREE.Color(0x5a1420),
      tooth: new THREE.Color(0xf4ecd8),
    };
  }

  setColors(colors: DragonColors): void {
    this.pal = Dragon.palette(colors);
    this.eyeMat.color.set(colors.eye);
    this.headKey = -1;
  }

  roar(): void {
    if (this.roarT < 0) this.roarT = 0;
  }

  get roaring(): boolean {
    return this.roarT >= 0;
  }

  /** Rebuild the skull when the dragon has aged enough for its horns to change. */
  private buildHead(age: number): void {
    const key = Math.round(age * 16);
    if (key === this.headKey) return;
    this.headKey = key;
    const pal = this.pal;
    const V = (x: number, y: number, z: number) => new THREE.Vector3(x, y, z);
    const hex = (z: number, w: number, top: number, bot: number) => {
      const mid = (top + bot) / 2 + 0.02;
      return [V(-w / 2, mid, z), V(-w * 0.34, top, z), V(w * 0.34, top, z), V(w / 2, mid, z), V(w * 0.3, bot, z), V(-w * 0.3, bot, z)];
    };
    const snout = lerp(0.82, 1, age); // hatchlings have stubbier faces

    const b = new TriBuffer(1024);
    b.loft(
      [
        hex(-0.2, 0.26, 0.13, -0.08), hex(-0.02, 0.42, 0.21, -0.11), hex(0.24, 0.4, 0.19, -0.05),
        hex(0.4 * snout, 0.27, 0.11, -0.03), hex(0.82 * snout, 0.22, 0.07, -0.03), hex(0.96 * snout, 0.13, 0.02, -0.03),
      ],
      (face, span) => (face === 4 ? (span >= 3 ? pal.mouth : pal.belly) : face === 3 || face === 5 ? pal.bodyDark : pal.body),
      pal.bodyDark, pal.body,
    );
    for (const side of [-1, 1]) {
      // brow ridge, nostril, horns
      b.prism(V(side * 0.13, 0.2, 0.31), V(side * 0.215, 0.235, 0.02), 0.04, 0.062, 4, pal.bodyDark, 1, true);
      b.prism(V(side * 0.05, 0.06, 0.8 * snout), V(side * 0.055, 0.085, 0.9 * snout), 0.03, 0.012, 4, pal.bodyDark);
      const hl = lerp(0.2, 1.3, age);
      const h0 = V(side * 0.13, 0.17, -0.06);
      const h1 = V(side * (0.13 + 0.1 * hl), 0.17 + 0.16 * hl, -0.06 - 0.3 * hl);
      const h2 = V(h1.x + side * 0.05 * hl, h1.y + 0.2 * hl, h1.z - 0.2 * hl);
      b.prism(h0, h1, 0.062, 0.04, 5, pal.horn);
      b.prism(h1, h2, 0.04, 0, 5, pal.horn, 1.1);
      // cheek frill: three spines with a web between them
      const fl = lerp(0.4, 1.05, age);
      const root = V(side * 0.195, 0.02, -0.02);
      const tips = [
        V(side * (0.2 + 0.2 * fl), 0.02 + 0.15 * fl, -0.02 - 0.26 * fl),
        V(side * (0.2 + 0.27 * fl), 0.01, -0.02 - 0.33 * fl),
        V(side * (0.2 + 0.19 * fl), 0.02 - 0.14 * fl, -0.02 - 0.25 * fl),
      ];
      for (const tip of tips) b.prism(root, tip, 0.026, 0, 3, pal.horn);
      b.tri(root, tips[0], tips[1], pal.wing, 1.15);
      b.tri(root, tips[1], tips[2], pal.wing, 0.95);
      // upper teeth
      for (let i = 0; i < 4; i++) {
        const z = lerp(0.48, 0.88, i / 3) * snout;
        const x = side * lerp(0.115, 0.085, i / 3);
        b.prism(V(x, -0.025, z), V(x, i === 3 ? -0.1 : -0.075, z), 0.02, 0, 3, pal.tooth);
      }
    }
    if (age > 0.5) b.prism(V(0, 0.06, 0.76 * snout), V(0, 0.06 + 0.22 * (age - 0.3), 0.66 * snout), 0.034, 0, 4, pal.horn); // nose horn
    this.skull.geometry.dispose();
    this.skull.geometry = b.toGeometry();

    const j = new TriBuffer(512);
    const quad = (z: number, w: number, d: number) => [V(-w / 2, 0, z), V(w / 2, 0, z), V(w * 0.36, -d, z), V(-w * 0.36, -d, z)];
    j.loft(
      [quad(0, 0.34, 0.09), quad(0.4 * snout, 0.25, 0.07), quad(0.84 * snout, 0.17, 0.045), quad(0.93 * snout, 0.1, 0.02)],
      (face) => (face === 0 ? pal.mouth : face === 2 ? pal.belly : pal.bodyDark),
      pal.bodyDark, pal.belly,
    );
    for (const side of [-1, 1]) {
      for (let i = 0; i < 3; i++) {
        const z = lerp(0.54, 0.82, i / 2) * snout;
        const x = side * lerp(0.1, 0.07, i / 2);
        j.prism(V(x, 0, z), V(x, 0.055, z), 0.018, 0, 3, pal.tooth);
      }
    }
    if (age > 0.6) j.prism(V(0, -0.05, 0.6 * snout), V(0, -0.05 - 0.22 * (age - 0.5), 0.5 * snout), 0.035, 0, 4, pal.horn);
    this.jawMesh.geometry.dispose();
    this.jawMesh.geometry = j.toGeometry();
  }

  /**
   * Advance the animation and rewrite the body for the current pile. `away` runs 0 (on the
   * throne) to 1 (settled on the ring round the hoard). Going out (`leaving`) the dragon uncoils
   * into the air and circles down to the ring. Coming back it glides to the foot of the pile
   * and climbs round it to the top. Either way the head leads and the body follows.
   */
  update(time: number, dt: number, L: Layout, awake: boolean, lookAt: THREE.Vector3 | null, fire: boolean, away: number, leaving: boolean): void {
    const s = L.s;
    const age = L.age;
    const t = this.tmp;
    const pal = this.pal;

    // ── timeline ────────────────────────────────────────────────────────────
    let roarEnv = 0;
    let roarJaw = 0;
    if (this.roarT >= 0) {
      this.roarT += dt;
      const p = this.roarT / ROAR_SECONDS;
      if (p >= 1) this.roarT = -1;
      else {
        roarEnv = ease(clamp01(p / 0.18)) * ease(clamp01((1 - p) / 0.3));
        roarJaw = ease(clamp01((p - 0.12) / 0.12)) * ease(clamp01((0.86 - p) / 0.14));
      }
    }
    const wakeGoal = awake || this.roarT >= 0 ? 1 : 0;
    this.wake += (wakeGoal - this.wake) * (1 - Math.exp(-dt * 2.5));
    const wake = this.wake;
    this.nextBlink -= dt;
    if (this.nextBlink < 0) {
      this.blink = 0.14;
      this.nextBlink = 2.5 + this.rnd() * 4;
    }
    this.blink = Math.max(0, this.blink - dt);

    // Now and then, with nothing else going on, he yawns, peers at his hoard, stretches or shakes.
    let yawn = 0, peer = 0, stretch = 0, shake = 0;
    const settled = awake && this.roarT < 0 && away <= 0.001;
    if (this.idleKind === 0) {
      if (settled) this.nextIdle -= dt;
      if (this.nextIdle < 0) {
        this.idleKind = 1 + Math.floor(this.rnd() * 4);
        this.idleT = 0;
        this.idleLen = [0, 2.6, 3.4, 3.2, 1.1][this.idleKind];
      }
    } else {
      this.idleT += dt;
      const p = this.idleT / this.idleLen;
      if (p >= 1 || !settled) {
        this.idleKind = 0;
        this.nextIdle = 6 + this.rnd() * 9;
      } else {
        const env = ease(clamp01(p / 0.3)) * ease(clamp01((1 - p) / 0.3));
        if (this.idleKind === 1) yawn = env;
        else if (this.idleKind === 2) peer = env;
        else if (this.idleKind === 3) stretch = env;
        else shake = env;
      }
    }

    // ── track: where the body can lie on the ground, by distance from the throne ──
    {
      let th = TH_SHOULDER, d = 0, n = 0, ext = -1, tip = -1;
      const need = BODY_OFFSETS[0] * s + 0.5;
      for (; n < TRACK_SAMPLES; n++, th -= TRACK_STEP) {
        const rho = trackRho(th, s, L.rhoTip, L.retreatR);
        const x = Math.cos(th) * rho, z = Math.sin(th) * rho;
        if (n > 0) d += Math.hypot(x - this.trX[n - 1], z - this.trZ[n - 1]);
        this.trD[n] = d; this.trX[n] = x; this.trZ[n] = z;
        if (tip < 0 && th <= TH_TIP) tip = d;
        if (ext < 0 && th <= TH_TIP - TRACK_EXT) ext = d;
        if (ext >= 0 && d > ext + need) { n++; break; }
      }
      this.trN = n;
      this.trackExtent = ext < 0 ? d : ext;
      this.trackTip = tip < 0 ? 0 : tip;
    }
    const bodyLength = BODY_OFFSETS[0] * s;

    // ── flight out: a rising turn round the hoard that comes down onto the ring ──
    // It leaves the throne in the direction the dragon faces and lands a body length short
    // of where he will rest, so the last stretch is a landing run along the ring.
    const landAngle = TH_TIP - TRACK_EXT + 2 * TAU - bodyLength / L.retreatR;
    const sweep = Math.max(90 * DEG, landAngle - TH_SHOULDER);
    const landFrom = TH_SHOULDER + sweep;
    {
      const y0 = L.heightAt(Math.cos(TH_SHOULDER) * s, Math.sin(TH_SHOULDER) * s) + this.ctrlR[SPIRAL_POINTS - 1] * s * 0.8;
      let q = 0;
      for (let k = 0; k < AIR_SAMPLES; k++) {
        const u = k / (AIR_SAMPLES - 1);
        const th = TH_SHOULDER + sweep * u;
        const rho = lerp(s, L.retreatR, ease(u));
        const x = Math.cos(th) * rho, z = Math.sin(th) * rho;
        const ground = L.heightAt(x, z) + 0.4 * s;
        const arc = Math.sin(Math.PI * Math.pow(u, 0.8));
        const y = Math.max(lerp(y0, 0.35 * s, ease(u)) + 1.9 * s * arc, ground + 0.7 * s * arc);
        if (k > 0) q += Math.hypot(x - this.airX[k - 1], y - this.airY[k - 1], z - this.airZ[k - 1]);
        this.airQ[k] = q; this.airX[k] = x; this.airY[k] = y; this.airZ[k] = z;
      }
    }
    const airLength = this.airQ[AIR_SAMPLES - 1];
    this.leaveLength = airLength + bodyLength;

    // Where a point `q` along the way out is. Behind the start it is still on the throne's coil.
    // Returns how airborne that point is, 0..1.
    const at = t[19];
    const tan = t[20];
    const wayOut = (q: number, out: THREE.Vector3): number => {
      if (q <= 0) {
        this.trackAt(-q, out, tan);
        out.y = -1; // on the ground: height comes from the pile
        return 0;
      }
      if (q >= airLength) {
        const th = landFrom + (q - airLength) / L.retreatR;
        out.set(Math.cos(th) * L.retreatR, -1, Math.sin(th) * L.retreatR);
        return 0;
      }
      let lo = 0, hi = AIR_SAMPLES - 1;
      while (hi - lo > 1) {
        const mid = (lo + hi) >> 1;
        if (this.airQ[mid] <= q) lo = mid;
        else hi = mid;
      }
      const f = (q - this.airQ[lo]) / (this.airQ[hi] - this.airQ[lo] || 1);
      out.set(lerp(this.airX[lo], this.airX[hi], f), lerp(this.airY[lo], this.airY[hi], f), lerp(this.airZ[lo], this.airZ[hi], f));
      return clamp01(Math.min(q / (0.1 * airLength), (airLength - q) / (0.14 * airLength)));
    };
    // The way back: over the stretch between the ring and the foot of the pile he is off the ground.
    const hopSpan = Math.max(1e-3, this.trackExtent - this.trackTip);
    const hopAt = (d: number): number => (leaving || d <= this.trackTip || d >= this.trackExtent ? 0 : Math.sin(Math.PI * ((d - this.trackTip) / hopSpan)));

    const along = clamp01(away) * (leaving ? this.leaveLength : this.trackExtent);
    const traveled = leaving === this.lastLeaving ? Math.abs(along - this.lastShift) : 0;
    this.lastShift = along;
    this.lastLeaving = leaving;
    const moveGoal = dt > 0 ? clamp01(traveled / dt / (1.2 * s)) : 0;
    this.move += (moveGoal - this.move) * (1 - Math.exp(-dt * 6));
    this.gait += (traveled / (0.75 * s)) * Math.PI;
    const move = this.move;

    // ── spine control points ────────────────────────────────────────────────
    const pts = this.curve.points;
    for (let j = 0; j < SPIRAL_POINTS; j++) {
      const a = j / (SPIRAL_POINTS - 1);
      const behind = BODY_OFFSETS[j] * s;
      let air: number;
      let y: number;
      if (leaving) {
        air = wayOut(along - behind, at);
        y = at.y;
      } else {
        this.trackAt(behind + along, at, tan);
        air = hopAt(behind + along);
        y = -1;
      }
      const ground = L.heightAt(at.x, at.z) + this.ctrlR[j] * s * 0.8;
      let lift = a > A_HIP * 0.7 ? 0.16 * s * move * (1 - air) : 0; // walking carries the belly off the ground
      if (a < A_HIP) lift += Math.pow(1 - a / A_HIP, 3) * (0.1 + 0.07 * Math.sin(time * 1.9)) * s * wake;
      pts[j].set(at.x, y < 0 ? ground + lift + 1.15 * s * air : Math.max(y, ground), at.z);
      this.ctrlAir[j] = air;
    }
    // The tail swishes across its own line.
    for (let j = 1; j < SPIRAL_POINTS - 1; j++) {
      const a = j / (SPIRAL_POINTS - 1);
      if (a >= A_HIP) break;
      const k = 1 - a / A_HIP; // 1 at the tail tip
      const swish = Math.sin(time * 1.3 - a * 9) * 0.2 * s * Math.pow(k, 1.4) * (0.35 + 0.65 * wake);
      const lx = pts[j + 1].x - pts[j - 1].x, lz = pts[j + 1].z - pts[j - 1].z;
      const ll = Math.hypot(lx, lz) || 1;
      pts[j].x += (lz / ll) * swish;
      pts[j].z += (-lx / ll) * swish;
    }
    for (let j = SPIRAL_POINTS; j < CONTROL_POINTS; j++) this.ctrlAir[j] = this.ctrlAir[SPIRAL_POINTS - 1];
    const fly = this.ctrlAir[SPIRAL_POINTS - 1]; // how airborne the shoulders are
    const flyHind = lerp(this.ctrlAir[6], this.ctrlAir[7], 0.26);
    const sh = pts[SPIRAL_POINTS - 1];
    this.center.copy(sh);
    let fx: number, fz: number; // forward at the shoulders
    if (leaving) {
      wayOut(along + 0.25 * s, at);
      const hx = at.x - sh.x, hz = at.z - sh.z;
      const hl = Math.hypot(hx, hz) || 1;
      fx = hx / hl; fz = hz / hl;
    } else {
      this.trackAt(along, at, tan);
      fx = -tan.x; fz = -tan.z;
    }
    const ox = fz, oz = -fx; // away from the hoard
    const nk = lerp(0.72, 1, age) * s; // neck length
    const sway = Math.sin(time * 0.6) * 0.06 * s * wake;
    const bob = Math.sin(time * 0.9) * 0.04 * s * wake;

    // Where the head wants to point, as a turn away from straight ahead.
    const restYaw = Math.atan2(fx, fz) + 0.13;
    let goalYaw = 0.32 * Math.sin(time * 0.31) + 0.1 * Math.sin(time * 0.83);
    let goalPitch = -0.08 + 0.07 * Math.sin(time * 0.47);
    if (lookAt) {
      const dx = lookAt.x - this.headPos.x, dy = lookAt.y - this.headPos.y, dz = lookAt.z - this.headPos.z;
      const len = Math.hypot(dx, dy, dz) || 1;
      let turn = Math.atan2(dx, dz) - restYaw;
      turn = Math.atan2(Math.sin(turn), Math.cos(turn));
      if (Math.abs(turn) < 1.9) {
        goalYaw = clampN(turn, -1.4, 1.4);
        goalPitch = clampN(Math.asin(dy / len), -0.6, 0.75);
      }
    }
    if (peer > 0) {
      // Nose down into the pile he is curled around.
      let turn = Math.atan2(-this.headPos.x, -this.headPos.z) - restYaw;
      turn = Math.atan2(Math.sin(turn), Math.cos(turn));
      goalYaw = lerp(goalYaw, clampN(turn, -1.4, 1.4), peer);
      goalPitch = lerp(goalPitch, -0.8, peer);
    }
    goalPitch += 0.4 * yawn + 0.28 * stretch;
    if (fly > 0.3) {
      // In the air he looks where he is going.
      goalYaw = lerp(goalYaw, 0, fly);
      goalPitch = lerp(goalPitch, -0.12, fly);
    }
    goalPitch += 0.38 * roarEnv;
    const follow = 1 - Math.exp(-dt * 4);
    this.yawRel += (goalYaw - this.yawRel) * follow;
    this.pitch += (goalPitch - this.pitch) * follow;
    this.yaw = restYaw + this.yawRel;

    // Awake pose: an S-curve up from the shoulders. Asleep: chin on the hoard.
    const a1 = t[0].set(sh.x + fx * 0.4 * nk, sh.y + 0.36 * nk, sh.z + fz * 0.4 * nk);
    const a2 = t[1].set(sh.x + fx * 0.52 * nk - ox * 0.08 * nk, sh.y + 0.9 * nk, sh.z + fz * 0.52 * nk - oz * 0.08 * nk);
    const a3 = t[2].set(
      sh.x + fx * 0.85 * nk - ox * 0.25 * nk + ox * sway + Math.sin(this.yaw) * 0.12 * s,
      sh.y + 1.38 * nk + bob + 0.22 * s * roarEnv,
      sh.z + fz * 0.85 * nk - oz * 0.25 * nk + oz * sway + Math.cos(this.yaw) * 0.12 * s,
    );
    a3.y += (0.12 * yawn + 0.2 * stretch - 0.5 * peer) * nk;
    a2.y -= 0.18 * peer * nk;
    if (fly > 0) {
      a1.lerp(t[21].set(sh.x + fx * 0.5 * nk, sh.y + 0.16 * nk, sh.z + fz * 0.5 * nk), fly);
      a2.lerp(t[21].set(sh.x + fx * 1.0 * nk, sh.y + 0.26 * nk, sh.z + fz * 1.0 * nk), fly);
      a3.lerp(t[21].set(sh.x + fx * 1.45 * nk, sh.y + 0.24 * nk + bob, sh.z + fz * 1.45 * nk), fly);
    }
    const s1 = t[3].set(sh.x + fx * 0.45 * nk, sh.y + 0.08 * nk, sh.z + fz * 0.45 * nk);
    const s2 = t[4].set(sh.x + fx * 0.95 * nk - ox * 0.25 * nk, 0, sh.z + fz * 0.95 * nk - oz * 0.25 * nk);
    s2.y = L.heightAt(s2.x, s2.z) + 0.24 * s;
    const s3 = t[5].set(sh.x + fx * 1.4 * nk - ox * 0.62 * nk, 0, sh.z + fz * 1.4 * nk - oz * 0.62 * nk);
    s3.y = L.heightAt(s3.x, s3.z) + 0.2 * s + Math.sin(time * 1.1) * 0.012 * s;
    const sleepYaw = Math.atan2(s3.x - s2.x, s3.z - s2.z);
    const yaw = wake > 0.999 ? this.yaw : this.yaw + Math.atan2(Math.sin(sleepYaw - this.yaw), Math.cos(sleepYaw - this.yaw)) * (1 - wake);
    const pitch = this.pitch * wake;
    const look = this.look.set(Math.sin(yaw) * Math.cos(pitch), Math.sin(pitch), Math.cos(yaw) * Math.cos(pitch));
    pts[SPIRAL_POINTS].lerpVectors(s1, a1, wake);
    pts[SPIRAL_POINTS + 1].lerpVectors(s2, a2, wake);
    const hb = pts[SPIRAL_POINTS + 2].lerpVectors(s3, a3, wake);
    pts[SPIRAL_POINTS + 3].copy(hb).addScaledVector(look, 0.16 * s);

    // ── sample the spine and build frames ───────────────────────────────────
    const breathe = 1 + (0.03 + 0.02 * (1 - wake)) * Math.sin(time * lerp(1.1, 1.7, wake));
    for (let i = 0; i < RINGS; i++) {
      const u = i / (RINGS - 1);
      this.curve.getPoint(u, this.P[i]);
      const f = u * (CONTROL_POINTS - 1);
      const j = Math.min(CONTROL_POINTS - 2, Math.floor(f));
      let r = lerp(this.ctrlR[j], this.ctrlR[j + 1], f - j) * s;
      if (i >= 17 && i <= RING_SHOULDER) r *= breathe;
      this.ringR[i] = i === 0 ? 0.004 * s : r;
    }
    for (let i = 0; i < RINGS; i++) {
      const a = this.P[Math.max(0, i - 1)];
      const b = this.P[Math.min(RINGS - 1, i + 1)];
      this.T[i].subVectors(b, a).normalize();
      const n = this.Nn[i];
      if (i === 0) n.set(0, 1, 0);
      else n.copy(this.Nn[i - 1]);
      n.addScaledVector(this.T[i], -n.dot(this.T[i]));
      if (n.lengthSq() < 1e-8) n.set(0, 1, 0);
      n.normalize();
      this.B[i].crossVectors(this.T[i], n);
    }
    // Lean into the turn while airborne.
    for (let i = 0; i < RINGS; i++) {
      const f = (i / (RINGS - 1)) * (CONTROL_POINTS - 1);
      const j = Math.min(CONTROL_POINTS - 2, Math.floor(f));
      const bank = (leaving ? 0.5 : 0.25) * lerp(this.ctrlAir[j], this.ctrlAir[j + 1], f - j);
      if (bank < 1e-3) continue;
      const n = this.Nn[i];
      n.multiplyScalar(Math.cos(bank)).addScaledVector(this.B[i], Math.sin(bank));
      this.B[i].crossVectors(this.T[i], n);
    }

    // ── body tube ───────────────────────────────────────────────────────────
    const buf = this.buf;
    buf.reset();
    const rp = this.ringPos;
    for (let i = 0; i < RINGS; i++) {
      const p = this.P[i], n = this.Nn[i], b = this.B[i], r = this.ringR[i];
      for (let k = 0; k < RING_SIDES; k++) {
        const ang = (k / RING_SIDES) * TAU;
        const c = Math.cos(ang), sn = Math.sin(ang);
        const up = (k === 0 ? 1.14 : c > 0 ? 1 : 0.78) * c * r;
        const out = 1.08 * sn * r;
        const o = (i * RING_SIDES + k) * 3;
        rp[o] = p.x + n.x * up + b.x * out;
        rp[o + 1] = p.y + n.y * up + b.y * out;
        rp[o + 2] = p.z + n.z * up + b.z * out;
      }
    }
    for (let i = 0; i < RINGS - 1; i++) {
      const fade = 0.7 + 0.3 * clamp01(i / 12); // darker toward the tail tip
      for (let k = 0; k < RING_SIDES; k++) {
        const k1 = (k + 1) % RING_SIDES;
        const a = (i * RING_SIDES + k) * 3, b = (i * RING_SIDES + k1) * 3;
        const c = ((i + 1) * RING_SIDES + k1) * 3, d = ((i + 1) * RING_SIDES + k) * 3;
        const belly = k === 3 || k === 4;
        const top = k === 0 || k === 7;
        // Belly plates alternate; the back carries dark saddle stripes; every scale is a slightly different red.
        const col = belly ? pal.belly : top && i % 3 === 0 ? pal.stripe : pal.body;
        const facet = ((i * 73 + k * 151) % 17) / 16;
        const m = fade * (belly ? (i % 2 ? 0.8 : 1) : (0.9 + 0.16 * facet) * (k === 2 || k === 5 ? 0.84 : 1));
        buf.raw(rp[a], rp[a + 1], rp[a + 2], rp[b], rp[b + 1], rp[b + 2], rp[c], rp[c + 1], rp[c + 2], col.r * m, col.g * m, col.b * m);
        buf.raw(rp[a], rp[a + 1], rp[a + 2], rp[c], rp[c + 1], rp[c + 2], rp[d], rp[d + 1], rp[d + 2], col.r * m, col.g * m, col.b * m);
      }
    }

    // ── spikes down the back, webbed into a sail over the shoulders and up the neck ──
    const spike = lerp(0.5, 1.3, age) * s;
    const prevApex = t[22], prevBase = t[23];
    for (let i = 4; i < RINGS - 3; i += 2) {
      const q = (i - 4) / (RINGS - 8);
      const h = spike * (0.05 + 0.21 * Math.pow(Math.sin(Math.PI * Math.pow(q, 1.6)), 0.8));
      const w = h * 0.42;
      const p = this.P[i], n = this.Nn[i], tg = this.T[i], b = this.B[i];
      const base = t[6].copy(p).addScaledVector(n, this.ringR[i] * 1.05);
      const v0 = t[7].copy(base).addScaledVector(tg, w);
      const v1 = t[8].copy(base).addScaledVector(tg, -w * 0.7).addScaledVector(b, w * 0.6);
      const v2 = t[9].copy(base).addScaledVector(tg, -w * 0.7).addScaledVector(b, -w * 0.6);
      const apex = t[10].copy(base).addScaledVector(n, h).addScaledVector(tg, -h * 0.4);
      buf.tri(v0, apex, v1, pal.horn);
      buf.tri(v1, apex, v2, pal.horn, 0.85);
      buf.tri(v2, apex, v0, pal.horn);
      if (i > 28) {
        const dip = t[24].addVectors(prevApex, apex).multiplyScalar(0.5).lerp(base, 0.3);
        buf.tri(prevApex, prevBase, dip, pal.wing, 1.1);
        buf.tri(prevBase, base, dip, pal.wing, 0.9);
        buf.tri(base, apex, dip, pal.wing, 1.0);
      }
      prevApex.copy(apex);
      prevBase.copy(base);
    }
    // tail blade
    {
      const bl = lerp(0.12, 0.34, age) * s;
      const tip = t[6].copy(this.P[0]).addScaledVector(this.T[0], -bl);
      const left = t[7].copy(this.P[2]).addScaledVector(this.B[2], bl * 0.5);
      const right = t[8].copy(this.P[2]).addScaledVector(this.B[2], -bl * 0.5);
      buf.tri(tip, left, this.P[2], pal.horn);
      buf.tri(tip, this.P[2], right, pal.horn, 0.85);
    }

    // ── legs ────────────────────────────────────────────────────────────────
    for (const side of [-1, 1]) {
      {
        const i = RING_FORELEG;
        const p = this.P[i], n = this.Nn[i], tg = this.T[i], b = this.B[i], r = this.ringR[i];
        const joint = t[6].copy(p).addScaledVector(b, side * r * 0.8).addScaledVector(n, -r * 0.25);
        const ph = this.gait + (side > 0 ? Math.PI : 0);
        const reach = 0.24 * s * move * Math.cos(ph) * (1 - fly);
        const raise = 0.2 * s * move * Math.max(0, Math.sin(ph));
        const elbow = t[7].copy(joint).addScaledVector(tg, -0.12 * s + reach * 0.4 - 0.1 * s * fly).addScaledVector(b, side * 0.16 * s);
        elbow.y = lerp(L.heightAt(elbow.x, elbow.z) + (0.17 + 0.14 * move) * s, joint.y - 0.16 * s, fly);
        // On the ground the foreleg reaches forward; in the air it folds back under the chest.
        const paw = t[8].copy(elbow).addScaledVector(tg, lerp(0.62 * s - 0.2 * s * move + reach, -0.38 * s, fly)).addScaledVector(b, side * 0.04 * s);
        paw.y = lerp(L.heightAt(paw.x, paw.z) + 0.15 * s + raise, joint.y - 0.3 * s, fly);
        const toe = t[9].copy(paw).addScaledVector(tg, lerp(0.2, -0.18, fly) * s);
        toe.y = lerp(L.heightAt(toe.x, toe.z) + 0.12 * s + raise, paw.y - 0.06 * s, fly);
        buf.prism(joint, elbow, 0.23 * s, 0.14 * s, 6, pal.body, 1, true);
        buf.prism(elbow, paw, 0.14 * s, 0.09 * s, 5, pal.body);
        buf.prism(paw, toe, 0.115 * s, 0.085 * s, 5, pal.bodyDark, 1, true);
        const spur = t[11].copy(elbow).addScaledVector(tg, -0.2 * s);
        spur.y += 0.1 * s;
        buf.prism(elbow, spur, 0.045 * s, 0, 4, pal.horn);
        for (let c = -1.5; c <= 1.5; c++) {
          const c0 = t[10].copy(toe).addScaledVector(b, c * 0.05 * s);
          const c1 = t[11].copy(c0).addScaledVector(tg, 0.18 * s).addScaledVector(b, c * 0.03 * s);
          c1.y -= 0.07 * s;
          buf.prism(c0, c1, 0.036 * s, 0, 4, pal.horn);
        }
      }
      {
        const i = RING_HINDLEG;
        const p = this.P[i], tg = this.T[i], b = this.B[i], r = this.ringR[i];
        const joint = t[6].copy(p).addScaledVector(b, side * r * 0.75);
        const h0 = t[7].copy(joint).addScaledVector(tg, -0.26 * s);
        h0.y += 0.05 * s;
        const h1 = t[8].copy(joint).addScaledVector(tg, 0.3 * s);
        h1.y -= 0.1 * s;
        buf.prism(h0, h1, 0.3 * s, 0.21 * s, 6, pal.body, 0.95, true);
        const ph = this.gait + (side > 0 ? 0 : Math.PI);
        const reach = 0.22 * s * move * Math.cos(ph) * (1 - flyHind);
        const raise = 0.18 * s * move * Math.max(0, Math.sin(ph));
        const f0 = t[9].copy(joint).addScaledVector(tg, lerp(0.26 * s - 0.2 * s * move + reach, -0.2 * s, flyHind)).addScaledVector(b, side * 0.14 * s);
        f0.y = lerp(L.heightAt(f0.x, f0.z) + 0.14 * s + raise, joint.y - 0.3 * s, flyHind);
        const f1 = t[10].copy(f0).addScaledVector(tg, lerp(0.42, -0.4, flyHind) * s);
        f1.y = lerp(L.heightAt(f1.x, f1.z) + 0.11 * s + raise, f0.y - 0.05 * s, flyHind);
        buf.prism(f0, f1, 0.12 * s, 0.085 * s, 5, pal.bodyDark, 1, true);
        for (let c = -1; c <= 1; c++) {
          const c0 = t[11].copy(f1).addScaledVector(b, c * 0.055 * s);
          const c1 = t[12].copy(c0).addScaledVector(tg, 0.17 * s).addScaledVector(b, c * 0.03 * s);
          c1.y -= 0.06 * s;
          buf.prism(c0, c1, 0.036 * s, 0, 4, pal.horn);
        }
      }
    }

    // ── wings ───────────────────────────────────────────────────────────────
    const aloft = Math.max(fly, flyHind);
    const openGoal = roarEnv > 0.05 || aloft > 0.05 ? 1 : Math.max(wake * (0.14 + 0.05 * Math.sin(time * 0.8)) + 0.03, 0.45 * move, 0.92 * stretch, 0.3 * shake);
    this.wingOpen += (openGoal - this.wingOpen) * (1 - Math.exp(-dt * (openGoal >= 1 ? 6 : 2.2)));
    // Deep slow beats in the air, a nervous flutter while roaring, barely a stir at rest.
    const flap = 0.04 * Math.sin(time * 1.6) * wake + 0.28 * Math.sin(time * 9) * roarEnv * (1 - aloft) + (0.15 + 0.5 * Math.sin(time * 7.5)) * aloft
      + 0.16 * Math.sin(time * 21) * shake + 0.12 * stretch;
    this.writeWing(1, this.wingOpen, flap, L);
    this.writeWing(-1, this.wingOpen, flap, L);

    // ── upload ──────────────────────────────────────────────────────────────
    const posAttr = this.geo.getAttribute('position') as THREE.BufferAttribute | undefined;
    if (!posAttr || posAttr.array !== buf.pos) {
      const pa = new THREE.BufferAttribute(buf.pos, 3);
      const ca = new THREE.BufferAttribute(buf.col, 3);
      pa.setUsage(THREE.DynamicDrawUsage);
      ca.setUsage(THREE.DynamicDrawUsage);
      this.geo.setAttribute('position', pa);
      this.geo.setAttribute('color', ca);
    } else {
      posAttr.needsUpdate = true;
      (this.geo.getAttribute('color') as THREE.BufferAttribute).needsUpdate = true;
    }
    this.geo.setDrawRange(0, buf.count);

    // ── head ────────────────────────────────────────────────────────────────
    this.buildHead(age);
    const hs = s * lerp(1.35, 1, age);
    this.headPos.copy(hb).addScaledVector(look, 0.05 * s);
    this.head.position.copy(this.headPos);
    this.mat4.lookAt(look, t[13].set(0, 0, 0), t[14].set(0, 1, 0));
    this.head.quaternion.setFromRotationMatrix(this.mat4);
    // A slow curious tilt of the head, and a rattle when he shakes.
    this.head.rotateZ((0.1 * Math.sin(time * 0.43) + 0.06 * Math.sin(time * 1.07)) * wake * (1 - fly) + 0.4 * Math.sin(time * 23) * shake);
    this.head.scale.setScalar(hs);
    this.jaw.rotation.x = 0.03 + 0.62 * roarJaw + 0.52 * yawn + 0.03 * (1 - wake) * (0.5 + 0.5 * Math.sin(time * 1.1));
    const lid = this.blink > 0 ? 0.08 : Math.max(0.08, lerp(0.08, 1, clamp01(wake * 1.6 - 0.3)) * (1 - 0.92 * yawn));
    const eyeSize = lerp(1.45, 1, age);
    for (const eye of this.eyes) eye.scale.set(eyeSize, eyeSize * lid, eyeSize);

    // ── smoke and fire ──────────────────────────────────────────────────────
    const mouth = t[15].set(0, -0.02, 0.95 * lerp(0.82, 1, age)).multiplyScalar(hs).applyQuaternion(this.head.quaternion).add(this.headPos);
    const breathesFire = fire && age > 0.22;
    if (roarJaw > 0.5) {
      this.fireCarry += dt * (breathesFire ? 90 : 14);
      while (this.fireCarry >= 1) {
        this.fireCarry -= 1;
        const spread = breathesFire ? 0.2 : 0.35;
        const vel = t[16].copy(look).multiplyScalar((breathesFire ? 5.5 : 1.6) * s);
        vel.x += (this.rnd() - 0.5) * spread * 5 * s;
        vel.y += (this.rnd() - 0.5) * spread * 5 * s;
        vel.z += (this.rnd() - 0.5) * spread * 5 * s;
        this.puffs.emit(breathesFire, mouth, vel, 0.4 + this.rnd() * 0.35, (0.06 + this.rnd() * 0.05) * s);
      }
    }
    this.nextPuff -= dt;
    if (this.nextPuff < 0 && this.roarT < 0) {
      this.nextPuff = wake > 0.5 ? 3 + this.rnd() * 4 : 3.2;
      for (const side of [-1, 1]) {
        const nostril = t[16].set(side * 0.05, 0.09, 0.9 * lerp(0.82, 1, age)).multiplyScalar(hs).applyQuaternion(this.head.quaternion).add(this.headPos);
        const vel = t[17].copy(look).multiplyScalar(0.35 * s);
        vel.y += 0.25 * s;
        this.puffs.emit(false, nostril, vel, 1.1 + this.rnd() * 0.5, 0.045 * s);
      }
    }
    this.puffs.update(dt);
    this.fireLight.position.copy(mouth).addScaledVector(look, 0.8 * s);
    this.fireLight.intensity = breathesFire ? Math.min(1, this.puffs.active / 30) * 30 * s * s : 0;
  }

  /** Point on the track at distance d (xz in `out`) and the direction of travel away from the throne. */
  private trackAt(d: number, out: THREE.Vector3, tangent: THREE.Vector3): void {
    const n = this.trN;
    let lo = 0, hi = n - 1;
    d = clampN(d, 0, this.trD[hi]);
    while (hi - lo > 1) {
      const mid = (lo + hi) >> 1;
      if (this.trD[mid] <= d) lo = mid;
      else hi = mid;
    }
    const span = this.trD[hi] - this.trD[lo] || 1;
    const f = (d - this.trD[lo]) / span;
    out.set(lerp(this.trX[lo], this.trX[hi], f), 0, lerp(this.trZ[lo], this.trZ[hi], f));
    tangent.set((this.trX[hi] - this.trX[lo]) / span, 0, (this.trZ[hi] - this.trZ[lo]) / span);
  }

  private writeWing(side: number, open: number, flap: number, L: Layout): void {
    const t = this.tmp;
    const buf = this.buf;
    const pal = this.pal;
    const i = RING_WING;
    const tg = this.T[i], n = this.Nn[i];
    const b = t[6].copy(this.B[i]).multiplyScalar(side);
    const r = this.ringR[i];
    const ws = L.s * lerp(0.5, 1.15, L.age);
    const cf = Math.cos(flap), sf = Math.sin(flap);
    const dir = (k: number, out: THREE.Vector3): THREE.Vector3 => {
      const f = WING_FOLDED[k], o = WING_SPREAD[k];
      const dt = lerp(f[0], o[0], open);
      const dn = lerp(f[1], o[1], open);
      const db = lerp(f[2], o[2], open);
      return out.set(0, 0, 0).addScaledVector(tg, dt).addScaledVector(n, dn * cf + db * sf).addScaledVector(b, db * cf - dn * sf).normalize();
    };
    const S = t[7].copy(this.P[i]).addScaledVector(n, r * 0.7).addScaledVector(b, r * 0.55);
    const E = t[8].copy(S).addScaledVector(dir(0, t[18]), WING_LENGTHS[0] * ws);
    const W = t[9].copy(E).addScaledVector(dir(1, t[18]), WING_LENGTHS[1] * ws);
    const F1 = t[10].copy(W).addScaledVector(dir(2, t[18]), WING_LENGTHS[2] * ws);
    const F2 = t[11].copy(W).addScaledVector(dir(3, t[18]), WING_LENGTHS[3] * ws);
    const F3 = t[12].copy(W).addScaledVector(dir(4, t[18]), WING_LENGTHS[4] * ws);
    const F4 = t[25].copy(W).addScaledVector(dir(5, t[18]), WING_LENGTHS[5] * ws);
    const j = RING_TRAIL;
    const A = t[13].copy(this.P[j]).addScaledVector(this.Nn[j], this.ringR[j] * 0.6).addScaledVector(this.B[j], side * this.ringR[j] * 0.4);

    buf.prism(S, E, 0.08 * ws, 0.058 * ws, 4, pal.bodyDark, 1, true);
    buf.prism(E, W, 0.058 * ws, 0.042 * ws, 4, pal.bodyDark);
    buf.prism(W, F1, 0.036 * ws, 0.006 * ws, 3, pal.bodyDark);
    buf.prism(W, F2, 0.034 * ws, 0.006 * ws, 3, pal.bodyDark);
    buf.prism(W, F3, 0.032 * ws, 0.006 * ws, 3, pal.bodyDark);
    buf.prism(W, F4, 0.03 * ws, 0.006 * ws, 3, pal.bodyDark);
    const claw = t[14].copy(W).addScaledVector(dir(1, t[18]), 0.24 * ws).addScaledVector(n, 0.1 * ws);
    buf.prism(W, claw, 0.034 * ws, 0, 3, pal.horn);

    // Membrane panels between the fingers, each scalloped toward the wrist and paler at its edge.
    const mid = t[15];
    const inner = t[26], innerQ = t[27];
    const panel = (p: THREE.Vector3, q: THREE.Vector3, shade: number) => {
      mid.addVectors(p, q).multiplyScalar(0.5).lerp(W, 0.22);
      inner.lerpVectors(W, p, 0.55);
      innerQ.lerpVectors(W, q, 0.55);
      buf.tri(W, inner, innerQ, pal.wing, shade * 0.8);
      buf.tri(inner, p, mid, pal.wing, shade * 1.12);
      buf.tri(inner, mid, innerQ, pal.wing, shade);
      buf.tri(innerQ, mid, q, pal.wing, shade * 1.05);
    };
    panel(F1, F2, 1);
    panel(F2, F3, 0.92);
    panel(F3, F4, 1);
    mid.addVectors(F4, A).multiplyScalar(0.5).lerp(W, 0.15);
    buf.tri(W, F4, mid, pal.wing, 0.95);
    buf.tri(E, W, mid, pal.wing, 0.85);
    buf.tri(E, mid, A, pal.wing, 0.92);
    buf.tri(S, E, A, pal.wing, 0.8);
    buf.tri(S, W, E, pal.wing, 0.76);
  }

  dispose(): void {
    this.geo.dispose();
    this.skull.geometry.dispose();
    this.jawMesh.geometry.dispose();
    this.eyes[0].children.forEach((c) => (c as THREE.Mesh).geometry.dispose());
    this.eyeMat.dispose();
    this.pupilMat.dispose();
    this.puffs.dispose();
  }
}

/* ════════════════════════════════ Static scenery ══════════════════════════════ */

/**
 * Unit pile (radius 1, slope height 1) following the pile profile. Normally it sits dark just under
 * the drawn treasure; at start-up it stands in for the treasure, full size and gold, until that is ready.
 */
function buildMoundGeometry(seed: number): THREE.BufferGeometry {
  const rings = 14, segs = 44;
  const b = new TriBuffer(rings * segs * 6);
  const rnd = mulberry32(seed * 977 + 31);
  // Mostly gold, with the odd fleck of everything else, so from a distance it reads as treasure.
  const gold = [0xe0a51e, 0xf2b01e, 0xc98f14, 0xffcf48, 0xd49a1a].map((c) => new THREE.Color(c));
  const flecks = [0x8a5fd0, 0x3d8bff, 0x35b56a, 0xd8cfb4, 0xc0453a, 0x9aa3ad].map((c) => new THREE.Color(c));
  const tone = () => (rnd() < 0.9 ? gold[Math.floor(rnd() * gold.length)] : flecks[Math.floor(rnd() * flecks.length)]);
  const P = (j: number, k: number) => {
    const t = j / rings;
    const a = (k / segs) * TAU + (j % 2) * (Math.PI / segs);
    return new THREE.Vector3(Math.cos(a) * t, profile(t), Math.sin(a) * t);
  };
  for (let j = 0; j < rings; j++) {
    for (let k = 0; k < segs; k++) {
      const a = P(j, k), bb = P(j, k + 1), c = P(j + 1, k + 1), d = P(j + 1, k);
      if (j > 0) b.tri(a, c, bb, tone(), 0.6 + 0.3 * rnd());
      b.tri(a, d, c, tone(), 0.6 + 0.3 * rnd());
    }
  }
  return b.toGeometry();
}

/** How full a domain may get: pile radius as a share of the floor radius. */
const PILE_FILL = 0.55;

interface DomainLook {
  top: number; side: number; rock: number; glow: number;
  spikes: number; crystals: number; pillars: number; pillarH: number; braziers: number;
}

const DOMAIN_LOOKS: DomainLook[] = [
  { top: 0x4b4558, side: 0x2c2836, rock: 0x5a5368, glow: 0x7ad7ff, spikes: 12, crystals: 0, pillars: 0, pillarH: 0, braziers: 0 },
  { top: 0x3f4856, side: 0x232a35, rock: 0x566373, glow: 0x6fe3ff, spikes: 16, crystals: 6, pillars: 0, pillarH: 0, braziers: 0 },
  { top: 0x3b4258, side: 0x1f2438, rock: 0x4f5c7c, glow: 0x7df0e0, spikes: 12, crystals: 18, pillars: 0, pillarH: 0, braziers: 2 },
  { top: 0x57504a, side: 0x2f2a26, rock: 0x70685d, glow: 0xffa040, spikes: 4, crystals: 0, pillars: 9, pillarH: 0.7, braziers: 4 },
  { top: 0x4d4238, side: 0x261e18, rock: 0x8c7a52, glow: 0xffc24a, spikes: 4, crystals: 8, pillars: 13, pillarH: 0.95, braziers: 4 },
];

/**
 * Unit-radius scenery for a domain: a rock island with a ragged rim, dressed by tier with
 * stalagmites, glowing crystals, a colonnade and braziers. The front arc stays clear.
 */
function buildDomainGeometry(tier: number, seed: number): { rock: THREE.BufferGeometry; glow: THREE.BufferGeometry } {
  const look = DOMAIN_LOOKS[Math.min(tier, DOMAIN_LOOKS.length - 1)];
  const rnd = mulberry32(seed * 131 + tier * 977 + 7);
  const segs = 28;
  const b = new TriBuffer(4096);
  const g = new TriBuffer(1024);
  const top = new THREE.Color(look.top), side = new THREE.Color(look.side), rock = new THREE.Color(look.rock);
  const glow = new THREE.Color(look.glow), flame = new THREE.Color(0xff8a2a), ember = new THREE.Color(0xffd36a);
  const V = (x: number, y: number, z: number) => new THREE.Vector3(x, y, z);
  const polar = (a: number, r: number, y = 0) => V(Math.cos(a) * r, y, Math.sin(a) * r);
  const behind = () => (145 + rnd() * 250) * DEG; // anywhere but the front arc

  const rim: THREE.Vector3[] = [], inner: THREE.Vector3[] = [], lip: THREE.Vector3[] = [], low: THREE.Vector3[] = [];
  for (let k = 0; k < segs; k++) {
    const a = (k / segs) * TAU;
    const rr = 1 + (rnd() - 0.5) * 0.07;
    rim.push(polar(a, rr));
    inner.push(polar(a, 0.62));
    lip.push(polar(a, rr * 0.93, -0.07 - rnd() * 0.03));
    low.push(polar(a, rr * 0.5, -0.2 - rnd() * 0.08));
  }
  const center = V(0, 0, 0), bottom = V(0, -0.34, 0);
  for (let k = 0; k < segs; k++) {
    const k1 = (k + 1) % segs;
    const shade = 0.88 + 0.12 * rnd();
    b.tri(center, inner[k1], inner[k], top, shade);
    b.tri(inner[k], inner[k1], rim[k1], top, shade * 0.97);
    b.tri(inner[k], rim[k1], rim[k], top, shade * 1.03);
    b.tri(rim[k], rim[k1], lip[k1], side, shade);
    b.tri(rim[k], lip[k1], lip[k], side, shade);
    b.tri(lip[k], lip[k1], low[k1], side, shade * 0.85);
    b.tri(lip[k], low[k1], low[k], side, shade * 0.85);
    b.tri(low[k], low[k1], bottom, side, shade * 0.7);
  }

  for (let i = 0; i < look.spikes; i++) {
    const foot = polar(behind(), 0.9 + rnd() * 0.07, -0.01);
    const h = 0.05 + rnd() * rnd() * 0.3;
    const tip = V(foot.x + (rnd() - 0.5) * 0.04, h, foot.z + (rnd() - 0.5) * 0.04);
    b.prism(foot, tip, 0.025 + h * 0.2, i % 3 === 0 ? 0.02 + h * 0.1 : 0, 5, rock, 0.8 + rnd() * 0.3, true);
  }
  for (let i = 0; i < look.crystals; i++) {
    const a = behind(), r = 0.9 + rnd() * 0.07;
    for (let c = 0; c < 3; c++) {
      const foot = polar(a + (rnd() - 0.5) * 0.06, r + (rnd() - 0.5) * 0.03, -0.01);
      const h = 0.05 + rnd() * 0.13;
      const lean = V((rnd() - 0.5) * 0.5, 1, (rnd() - 0.5) * 0.5).normalize();
      const mid = foot.clone().addScaledVector(lean, h * 0.75);
      const tip = foot.clone().addScaledVector(lean, h);
      const w = 0.012 + h * 0.1;
      const shade = 0.7 + rnd() * 0.6;
      g.prism(foot, mid, w * 0.8, w, 6, glow, shade);
      g.prism(mid, tip, w, 0, 6, glow, shade * 1.25);
    }
  }
  if (look.pillars > 0) {
    const tops: THREE.Vector3[] = [];
    for (let i = 0; i < look.pillars; i++) {
      const a = (150 + (240 * i) / (look.pillars - 1)) * DEG;
      const h = look.pillarH * (0.94 + rnd() * 0.06);
      const p0 = polar(a, 0.92, -0.01), p1 = polar(a, 0.92, 0.035), p2 = polar(a, 0.92, h - 0.04), p3 = polar(a, 0.92, h);
      b.prism(p0, p1, 0.062, 0.058, 6, rock, 0.9, true);
      b.prism(p1, p2, 0.042, 0.037, 8, rock, 1.0);
      b.prism(p2, p3, 0.045, 0.062, 6, rock, 1.1, true);
      tops.push(p3);
    }
    for (let i = 0; i < tops.length - 1; i++) b.prism(tops[i], tops[i + 1], 0.034, 0.034, 4, rock, 0.82);
  }
  for (let i = 0; i < look.braziers; i++) {
    const a = (90 + (i % 2 ? -1 : 1) * (i < 2 ? 60 : 122)) * DEG;
    const foot = polar(a, 0.82, -0.01);
    const neck = V(foot.x, 0.075, foot.z), lipY = V(foot.x, 0.105, foot.z);
    b.prism(foot, neck, 0.02, 0.012, 5, side, 1.4, true);
    b.prism(neck, lipY, 0.016, 0.05, 6, side, 1.6);
    g.prism(V(foot.x, 0.095, foot.z), V(foot.x + 0.004, 0.2, foot.z), 0.04, 0, 5, flame, 1.5);
    g.prism(V(foot.x, 0.1, foot.z), V(foot.x - 0.003, 0.165, foot.z), 0.024, 0, 4, ember, 1.6);
  }
  if (g.count === 0) g.tri(center, center, center, glow); // keep the mesh valid when a tier has nothing lit
  return { rock: b.toGeometry(), glow: g.toGeometry() };
}

/* ════════════════════════════════ Sky portal ══════════════════════════════════ */

/** The rift treasure pours from: a ring, two counter-turning swirls and a few orbiting shards. */
class Portal {
  readonly group = new THREE.Group();
  readonly light = new THREE.PointLight(0xffffff, 0, 0, 2);
  /** 0 closed .. 1 fully open. */
  open = 0;
  readonly bright = new THREE.Color();
  private readonly ring: THREE.Mesh;
  private readonly swirlA: THREE.Mesh;
  private readonly swirlB: THREE.Mesh;
  private readonly shards: THREE.Mesh[] = [];
  private readonly solid = new THREE.MeshBasicMaterial();
  private readonly veil = new THREE.MeshBasicMaterial({ vertexColors: true, side: THREE.DoubleSide, transparent: true, opacity: 0.94, depthWrite: false });
  private readonly haze = new THREE.MeshBasicMaterial({ vertexColors: true, side: THREE.DoubleSide, transparent: true, opacity: 0.85, depthWrite: false, blending: THREE.AdditiveBlending });

  constructor(color: THREE.ColorRepresentation) {
    const ringGeo = new THREE.TorusGeometry(1, 0.06, 5, 32);
    ringGeo.rotateX(Math.PI / 2);
    this.ring = new THREE.Mesh(ringGeo, this.solid);
    this.swirlA = new THREE.Mesh(new THREE.BufferGeometry(), this.veil);
    this.swirlB = new THREE.Mesh(new THREE.BufferGeometry(), this.haze);
    this.swirlB.position.y = -0.03;
    const shardGeo = new THREE.OctahedronGeometry(0.07, 0);
    for (let i = 0; i < 7; i++) {
      const shard = new THREE.Mesh(shardGeo, this.solid);
      this.shards.push(shard);
      this.group.add(shard);
    }
    this.group.add(this.ring, this.swirlA, this.swirlB, this.light);
    this.group.name = 'hoard-portal';
    this.group.visible = false;
    this.setColor(color);
  }

  setColor(color: THREE.ColorRepresentation): void {
    const base = new THREE.Color(color);
    this.bright.copy(base).lerp(new THREE.Color(1, 1, 1), 0.3);
    const deep = base.clone().multiplyScalar(0.18);
    const accent = base.clone().offsetHSL(-0.2, 0, 0.08);
    this.solid.color.copy(this.bright);
    this.light.color.copy(base);
    const pinwheel = (segs: number, radius: number, a: THREE.Color, b: THREE.Color, hub: THREE.Color) => {
      const buf = new TriBuffer(segs * 3);
      const o = new THREE.Vector3(), p = new THREE.Vector3(), q = new THREE.Vector3();
      for (let k = 0; k < segs; k++) {
        const a0 = (k / segs) * TAU, a1 = ((k + 1) / segs) * TAU + 0.35;
        p.set(Math.cos(a0) * radius, 0, Math.sin(a0) * radius);
        q.set(Math.cos(a1) * radius, 0, Math.sin(a1) * radius);
        const c = k % 2 ? a : b;
        buf.raw(o.x, o.y, o.z, p.x, p.y, p.z, q.x, q.y, q.z, c.r, c.g, c.b);
        // darken the hub vertex so the swirl reads as a funnel
        buf.col.set([hub.r, hub.g, hub.b], (buf.count - 3) * 3);
      }
      return buf.toGeometry();
    };
    this.swirlA.geometry.dispose();
    this.swirlA.geometry = pinwheel(18, 0.97, base, deep, deep);
    this.swirlB.geometry.dispose();
    this.swirlB.geometry = pinwheel(11, 0.7, accent, this.bright, deep);
  }

  update(time: number, dt: number, goal: number, radius: number, center: THREE.Vector3): void {
    this.open += (goal - this.open) * (1 - Math.exp(-dt * (goal > this.open ? 3.2 : 4.5)));
    if (this.open < 0.004 && goal === 0) this.open = 0;
    const o = this.open;
    this.group.visible = o > 0;
    if (!this.group.visible) { this.light.intensity = 0; return; }
    const overshoot = 1 + 0.18 * Math.sin(Math.min(1, o) * Math.PI); // swell as it tears open
    this.group.position.copy(center);
    this.group.scale.setScalar(Math.max(1e-3, radius * o * overshoot));
    this.ring.rotation.y = time * 0.35;
    this.swirlA.rotation.y = -time * 1.4;
    this.swirlB.rotation.y = time * 2.3;
    this.shards.forEach((shard, i) => {
      const a = time * (0.9 + i * 0.07) + (i / this.shards.length) * TAU;
      shard.position.set(Math.cos(a) * 1.22, Math.sin(time * 1.7 + i) * 0.12, Math.sin(a) * 1.22);
      shard.rotation.set(time + i, time * 1.3, 0);
    });
    this.light.intensity = o * radius * radius * 18;
  }

  dispose(): void {
    this.ring.geometry.dispose();
    this.swirlA.geometry.dispose();
    this.swirlB.geometry.dispose();
    this.shards[0].geometry.dispose();
    this.solid.dispose();
    this.veil.dispose();
    this.haze.dispose();
  }
}

/* ════════════════════════════════ DragonHoard ═════════════════════════════════ */

const DEFAULT_DRAGON: DragonColors = { body: 0xb5272d, belly: 0xe2b66a, wing: 0xb5462a, horn: 0xe9dcc0, eye: 0xffc531 };
const ZERO_COUNTS = (): Record<TreasureType, number> => ({ gold: 0, gems: 0, books: 0, weapons: 0, bones: 0, tablets: 0, paintings: 0, potions: 0 });

/** One stretch of treasure arriving: volumes v0 → v1 over `duration`. `t` starts negative (the delay before the first piece). */
interface Pour {
  v0: number; v1: number; t: number; duration: number;
  /** Longest a landed piece takes to run down the slope, and per-shape counts before and after. */
  tail: number; from: number[]; to: number[];
}

const dragonScale = (R: number) => 0.38 + 0.26 * Math.pow(R, 0.95);
/** Height of the portal above the floor for a pile of radius R guarded by a dragon of scale s. */
const portalHeight = (R: number, s: number) => SLOPE * R + 1.2 + 0.4 * R + 3.0 * s;
const SUMMON_SECONDS = 0.9;
const ROMAN = ['I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX', 'X'];
const SLOPE_LENGTH = Math.hypot(1, SLOPE);

/**
 * The hoard and its dragon as a scene object. Add it to a scene rendered by a
 * `WebGPURenderer` and call `update(dt)` once per frame before rendering.
 * `mountDragonHoard` below does all of that for you.
 */
export class DragonHoard extends THREE.Group {
  private opts: DragonHoardOptions;
  private readonly pieces: KindLayer[];
  private readonly treasure: THREE.Material;
  private readonly matte: THREE.Material;
  private treasureShown = true;
  private reveal = 1;
  private readonly U: SharedUniforms;
  private readonly dragon: Dragon;
  private readonly portal: Portal;
  private readonly flash: THREE.Mesh;
  private readonly mound: THREE.Mesh;
  private readonly floor: THREE.Mesh;
  private readonly lit: THREE.Mesh;
  private readonly rig = new THREE.Group();
  private readonly key = new THREE.DirectionalLight(0xfff0dc, 2.1);

  /** The collection as asked for, per type and per shape. */
  private readonly counts = ZERO_COUNTS();
  private kindCounts: number[];

  private volume = 0; // volume of the pieces that have come to rest
  private carry = 0; // volume the dragon remembers from the domain it just left
  private pour: Pour | null = null;
  private pourSeconds: number | undefined;
  private rested = 0; // seconds since the last pour finished
  private unbaked = false; // a layout is waiting for its first frame
  private stageQueued = false;
  private drawn = 0;
  private landedNow = 0;
  private readonly streamPool: number[] = KINDS.map(() => 0);
  private time = 0;

  private domainIndex = -1;
  private phase: HoardPhase = 'idle';
  private phaseT = 0;
  private ceremonial = false;
  private switched = false;
  private away = 0;
  private leaving = false; // which way the dragon is traveling between throne and ring
  private portalGoal = 0;
  private portalR = 1;
  private readonly portalPos = new THREE.Vector3(0, 5, 0);
  private readonly planeNormal = new THREE.Vector3();
  private readonly planePoint = new THREE.Vector3();

  private readonly layout: Layout;
  private lookWorld: THREE.Vector3 | null = null;
  private readonly lookLocal = new THREE.Vector3();

  constructor(options: DragonHoardOptions = {}) {
    super();
    this.name = 'dragon-hoard';
    this.opts = { ...options };

    this.U = {
      time: uniform(0),
      portal: uniform(new THREE.Vector3(0, 5, 0)), streamR: uniform(0.5), exitSpeed: uniform(2),
      clip: uniform(new THREE.Vector4(0, 1, 0, -5)),
      volume: uniform(0), pourT: uniform(1e6), fall: uniform(1), pourEnd: uniform(0), period: uniform(2), reveal: uniform(1),
    };
    this.treasure = buildTreasureMaterial(this.U);
    this.pieces = KINDS.map((def, i) => new KindLayer(def, i, this.treasure, this, options.seed ?? 1));
    this.kindCounts = KINDS.map(() => 0);

    const self = this;
    this.layout = {
      R: 0, H: 0, s: 0.38, age: 0, rhoTip: 0.6, retreatR: 1.6, domainR: 2,
      heightAt(x: number, z: number) {
        return self.layout.R > 1e-6 ? self.layout.H * profile(Math.hypot(x, z) / self.layout.R) : 0;
      },
    };

    // Dark core under the layer of treasure that is kept: only ever glimpsed as shadow between pieces.
    this.mound = new THREE.Mesh(
      buildMoundGeometry(options.seed ?? 1),
      new THREE.MeshBasicMaterial({ vertexColors: true, side: THREE.DoubleSide }),
    );
    this.mound.name = 'hoard-core';
    // One flat-shaded vertex-color material for the dragon and the scenery: one shader between them.
    this.matte = new THREE.MeshPhongMaterial({ vertexColors: true, flatShading: true, shininess: 14, specular: 0x1c1c1c, side: THREE.DoubleSide });
    this.floor = new THREE.Mesh(new THREE.BufferGeometry(), this.matte);
    this.floor.name = 'hoard-domain';
    this.floor.receiveShadow = true;
    this.lit = new THREE.Mesh(new THREE.BufferGeometry(), new THREE.MeshBasicMaterial({ vertexColors: true, side: THREE.DoubleSide }));
    this.lit.name = 'hoard-domain-lights';
    this.floor.visible = this.lit.visible = options.base !== false;

    this.dragon = new Dragon({ ...DEFAULT_DRAGON, ...options.dragon }, this.matte);
    this.portal = new Portal(options.portal ?? 0x8f5bff);
    // A shell around everything that fades in to cover the move between domains.
    this.flash = new THREE.Mesh(
      new THREE.SphereGeometry(1, 16, 10),
      new THREE.MeshBasicMaterial({ color: 0xffffff, side: THREE.BackSide, transparent: true, opacity: 0, depthTest: false, depthWrite: false, toneMapped: false }),
    );
    this.flash.name = 'hoard-flash';
    this.flash.renderOrder = 1000;
    this.flash.frustumCulled = false;
    this.flash.visible = false;
    this.add(this.floor, this.lit, this.mound, this.dragon.group, this.portal.group, this.flash);

    // Lights travel with the hoard so the shadow always frames the dragon.
    this.rig.name = 'hoard-lights';
    const sky = new THREE.HemisphereLight(0xc4ceff, 0x5a4022, 0.6);
    const rim = new THREE.DirectionalLight(0x7f9cff, 1.0);
    rim.position.set(5, 4, -7);
    this.key.castShadow = options.shadows !== false;
    this.key.shadow.mapSize.set(2048, 2048);
    this.key.shadow.bias = -0.0006;
    this.key.shadow.normalBias = 0.03;
    this.rig.add(sky, rim, this.key, this.key.target);
    this.rig.visible = options.lights !== false;
    this.add(this.rig);

    this.setCounts(options.lastSeen ?? options.counts ?? {}, { animate: false });
    if (options.lastSeen && options.counts) this.setCounts(options.counts);
  }

  /* ───────────────────────────── public API ───────────────────────────── */

  /** Set the whole collection. Growth arrives through the portal; anything that shrinks is re-laid instantly. */
  setCounts(counts: HoardCounts, pour: PourOptions = {}): HoardStats {
    const next = ZERO_COUNTS();
    for (const t of TREASURE_TYPES) next[t] = Math.max(0, Math.floor(counts[t] ?? 0));
    const perKind = this.splitKinds(next);
    const shrinks = perKind.some((n, i) => n < this.kindCounts[i]);
    const grows = perKind.some((n, i) => n > this.kindCounts[i]);
    const animate = (pour.animate ?? true) && grows && !shrinks;

    Object.assign(this.counts, next);
    this.kindCounts = perKind;
    if (animate) {
      this.pourSeconds = pour.duration;
      // A pour or a move already under way picks the new total up when it finishes. A layout
      // that has not been drawn yet has to be set down first, so the stream starts next frame.
      if (this.unbaked) this.stageQueued = true;
      else if (!this.pour && this.phase !== 'ascend' && this.phase !== 'summon') this.nextStage();
    } else if (grows || shrinks || this.domainIndex < 0) {
      this.settle();
    }
    this.applyLayout();
    return this.stats;
  }

  /** Add to the collection; the new treasure pours onto the pile. */
  deposit(counts: HoardCounts, pour: PourOptions = {}): HoardStats {
    const next: HoardCounts = {};
    for (const t of TREASURE_TYPES) next[t] = this.counts[t] + Math.max(0, Math.floor(counts[t] ?? 0));
    return this.setCounts(next, pour);
  }

  /** Rear up, spread wings and breathe fire. */
  roar(): void {
    this.dragon.roar();
  }

  /**
   * Hide the treasure (dragon and scenery still draw), compile its shader in the background,
   * then show it. Lets a first frame go up before the largest shader is ready.
   */
  async primeTreasure(renderer: THREE.WebGPURenderer, camera: THREE.Camera): Promise<void> {
    this.treasureHidden = true;
    let root: THREE.Object3D = this;
    while (root.parent) root = root.parent;
    try {
      await renderer.compileAsync(this.pieces[0].proxy(), camera, root as THREE.Scene);
    } finally {
      this.treasureShown = true;
    }
  }

  /** Whether treasure is being held back by `primeTreasure`. */
  set treasureHidden(hidden: boolean) {
    this.treasureShown = !hidden;
    if (hidden) this.reveal = 0; // when it comes back it grows in over a moment
  }

  /**
   * Compile the shaders that are not needed for the first frame (the portal, the stream, the
   * domain change), in the background where the browser allows, so nothing stalls when they
   * first appear. The hoard must already be in its scene. `mountDragonHoard` calls this for
   * you once the first frame is up.
   */
  async prime(renderer: THREE.WebGPURenderer, camera: THREE.Camera): Promise<void> {
    const restore: [THREE.Object3D, boolean][] = [];
    const show = (o: THREE.Object3D) => { restore.push([o, o.visible]); o.visible = true; };
    for (const layer of this.pieces) layer.meshes.forEach(show);
    show(this.portal.group);
    show(this.flash);
    show(this.mound);
    let root: THREE.Object3D = this;
    while (root.parent) root = root.parent;
    try {
      await renderer.compileAsync(root, camera);
    } finally {
      for (const [o, visible] of restore) o.visible = visible;
    }
  }

  /** World-space point for the dragon to watch, or null to let it idle. */
  setLookTarget(world: THREE.Vector3 | null): void {
    this.lookWorld = world;
  }

  setOptions(options: Partial<DragonHoardOptions>): void {
    this.opts = { ...this.opts, ...options };
    if (options.dragon) this.dragon.setColors({ ...DEFAULT_DRAGON, ...this.opts.dragon });
    if (options.portal !== undefined) this.portal.setColor(options.portal);
    if (options.lights !== undefined) this.rig.visible = options.lights;
    if (options.shadows !== undefined) this.key.castShadow = options.shadows;
    if (options.base !== undefined) this.floor.visible = this.lit.visible = options.base;
    if (options.counts) this.setCounts(options.counts);
    else if (options.maxParticles !== undefined || options.domains !== undefined) {
      this.settle();
      this.applyLayout();
    }
  }

  get total(): number {
    return this.kindCounts.reduce((a, b) => a + b, 0);
  }

  get stats(): HoardStats {
    const age = this.layout.age;
    const tier: DragonTier = age < 0.2 ? 'hatchling' : age < 0.42 ? 'whelp' : age < 0.62 ? 'drake' : age < 0.82 ? 'wyrm' : 'ancient';
    const index = Math.max(0, this.domainIndex);
    return {
      total: this.total,
      counts: { ...this.counts },
      landed: Math.round(this.pour ? this.landedNow : this.pieces.reduce((n, l) => n + l.count, 0)),
      particles: this.drawn,
      radius: this.layout.R,
      height: this.layout.H * (1 - TOP),
      age,
      tier,
      domain: { index, name: this.domainAt(index).name, fill: clamp01(this.volume / this.domainCapacity(index)) },
      phase: this.phase,
      pouring: this.pour !== null || this.phase !== 'idle',
    };
  }

  /** Rough extent, for framing a camera: the hoard and its dragon, and the portal while it is open. */
  get bounds(): { radius: number; height: number; focus: number } {
    const L = this.layout;
    const still = L.H + 2.9 * L.s;
    const withPortal = Math.max(still, this.portalPos.y + this.portalR * 0.4);
    const open = ease(clamp01(this.portal.open));
    // Frame the hoard and its dragon; widen to take in the dragon when it backs off.
    const close = Math.max(L.rhoTip + 0.9 * L.s, L.R * 1.15, L.domainR * 0.42);
    return {
      radius: lerp(close, Math.max(close, Math.min(L.retreatR + 1.4 * L.s, L.domainR * 1.02)), ease(clamp01(this.away * 1.5))),
      height: lerp(still, withPortal, open),
      focus: lerp(Math.max(L.H * 0.55 + 0.9 * L.s, L.domainR * 0.06), withPortal * 0.48, open),
    };
  }

  /** Step the pour and the animation. Call once per frame, before rendering. */
  update(dt: number): void {
    dt = Math.min(Math.max(dt, 0), 0.05);
    this.time += dt;
    const U = this.U;
    const step = dt * clampN(this.opts.speed ?? 1, 0.05, 64); // the pour's clock; the dragon idles in real time
    if (this.stageQueued && !this.unbaked) {
      // The layout has had its frame on screen; now the catch-up can start.
      this.stageQueued = false;
      if (!this.pour && this.phase !== 'ascend' && this.phase !== 'summon') this.nextStage();
    }

    // ── the pour ────────────────────────────────────────────────────────────
    const exit = 1.5 + 0.6 * this.portalR;
    U.time.value = this.time;
    (U.portal.value as THREE.Vector3).copy(this.portalPos);
    U.streamR.value = this.portalR * 0.7;
    U.exitSpeed.value = exit;
    // The pile is whatever has landed: it trails the stream by one fall from the portal to the crown.
    const fall = this.fallSeconds(radiusForVolume(this.volume), exit);
    let arrived = 0; // share of the running pour that has landed
    if (this.pour) {
      const p = this.pour;
      p.t += step;
      arrived = clamp01((p.t - fall) / p.duration);
      this.volume = Math.max(this.volume, lerp(p.v0, p.v1, arrived));
      if (p.t >= p.duration + fall + p.tail) {
        this.volume = p.v1;
        this.pieces.forEach((layer, i) => layer.setRest(p.to[i]));
        this.pour = null;
        this.rested = 0;
      }
    } else {
      this.rested += step;
    }
    U.pourT.value = this.pour ? this.pour.t : 1e6;
    U.pourEnd.value = this.pour ? this.pour.duration : 0;
    U.fall.value = fall;
    if (this.volume >= this.carry) this.carry = 0;

    // ── the ceremony ────────────────────────────────────────────────────────
    this.phaseT += step;
    if (this.phase === 'summon' && this.phaseT >= SUMMON_SECONDS) this.setPhase('pour');
    if (this.phase === 'pour' && !this.pour && this.rested > 0.5) this.nextStage();
    if (this.phase === 'ascend') {
      const rise = 1.1, fall = 1.3;
      if (!this.switched && this.phaseT >= rise) this.changeDomain();
      const f = this.phaseT < rise ? ease(this.phaseT / rise) : 1 - ease(clamp01((this.phaseT - rise) / fall));
      (this.flash.material as THREE.MeshBasicMaterial).opacity = f;
      this.flash.visible = f > 0.003;
      if (this.phaseT >= rise + fall) {
        this.flash.visible = false;
        this.setPhase('idle');
        this.nextStage();
      }
    }
    if (this.phase === 'return' && this.away <= 0) {
      this.setPhase('idle');
      if (this.total > 0) this.dragon.roar();
    }
    const awayGoal = (this.phase === 'pour' && this.ceremonial) || (this.phase === 'ascend' && !this.switched && this.away > 0) ? 1 : 0;
    if (this.phase !== 'summon') {
      // He flies out and climbs back; a change of mind mid-way retraces the same route.
      if (this.away <= 0 && awayGoal === 1) this.leaving = true;
      if (this.away >= 1 && awayGoal === 0) this.leaving = false;
      const seconds = this.leaving ? this.flightSeconds() : clampN(this.dragon.trackExtent / (2.6 * this.layout.s), 2.4, 4.6);
      this.away = clampN(this.away + (Math.sign(awayGoal - this.away) * step) / seconds, 0, 1);
    }

    this.applyLayout();
    const L = this.layout;
    if (!this.pour && this.portal.open <= 0) this.portalPos.y = portalHeight(L.R, L.s);

    // ── pieces: draw the newest of each shape, down to the depth that is kept ──
    const pour = this.pour;
    const landedOf = (i: number) => (pour ? lerp(pour.from[i], pour.to[i], arrived) : this.pieces[i].count);
    let landedNow = 0;
    for (let i = 0; i < this.pieces.length; i++) landedNow += landedOf(i);
    this.landedNow = landedNow;
    const keep = this.keptDepth(L.R, landedNow > 0 ? this.volume / landedNow : 0);
    const coreR = Math.max(0, L.R - keep / (SLOPE * (1 - TOP)));
    const kept = L.R > 1e-6 ? 1 - Math.pow(coreR / L.R, 3) : 1; // share of every shape that lies above the core
    U.volume.value = this.volume;
    const streaming = pour !== null && pour.t >= 0 && pour.t < pour.duration + (U.period.value as number);
    let drawn = 0;
    this.pieces.forEach((layer, i) => {
      const landed = landedOf(i);
      drawn += layer.advance(landed, Math.ceil(landed * kept) + 4, streaming, this.treasureShown);
    });
    this.drawn = drawn;
    this.unbaked = false;

    // The portal's mouth as a world-space plane; treasure is only drawn on the near side of it.
    this.updateWorldMatrix(true, false);
    this.planeNormal.set(0, 1, 0).transformDirection(this.matrixWorld);
    this.planePoint.copy(this.portalPos).applyMatrix4(this.matrixWorld);
    (U.clip.value as THREE.Vector4).set(this.planeNormal.x, this.planeNormal.y, this.planeNormal.z, -this.planeNormal.dot(this.planePoint));

    // ── scenery ─────────────────────────────────────────────────────────────
    // While the treasure is not up yet the mound stands in for it: full size and gold. As the
    // pieces pop in it sinks back under them and darkens to the shadow between them.
    if (this.treasureShown && this.reveal < 1) this.reveal = Math.min(1, this.reveal + dt / 0.5);
    U.reveal.value = this.reveal;
    const settledIn = ease(this.reveal);
    const moundR = lerp(L.R, coreR, settledIn);
    this.mound.visible = moundR > 0.02;
    this.mound.scale.set(Math.max(moundR, 1e-3), Math.max(SLOPE * moundR, 1e-3), Math.max(moundR, 1e-3));
    (this.mound.material as THREE.MeshBasicMaterial).color.setRGB(lerp(0.85, 0.17, settledIn), lerp(0.85, 0.12, settledIn), lerp(0.85, 0.05, settledIn));
    this.floor.scale.setScalar(L.domainR);
    this.lit.scale.setScalar(L.domainR);
    this.flash.scale.setScalar(Math.max(400, L.domainR * 14));
    this.portal.update(this.time, step, this.portalGoal, this.portalR, this.portalPos);
    (this.flash.material as THREE.MeshBasicMaterial).color.copy(this.portal.bright);

    // ── dragon ──────────────────────────────────────────────────────────────
    const mood = this.opts.mood ?? 'auto';
    const awake = mood === 'awake' || (mood === 'auto' && (this.total > 0 || this.phase !== 'idle'));
    let look: THREE.Vector3 | null = null;
    if (this.phase === 'summon' || (this.portal.open > 0.2 && this.phase !== 'return')) look = this.lookLocal.copy(this.portalPos);
    else if (this.lookWorld) look = this.worldToLocal(this.lookLocal.copy(this.lookWorld));
    this.dragon.update(this.time, dt, L, awake, look, this.opts.fire !== false, ease(this.away), this.leaving);

    const reach = 3.4 * L.s + 0.5;
    const cam = this.key.shadow.camera;
    if (Math.abs(cam.right - reach) > 0.02) {
      cam.left = -reach; cam.right = reach; cam.top = reach; cam.bottom = -reach;
      cam.near = 0.1; cam.far = reach * 6;
      cam.updateProjectionMatrix();
    }
    const c = this.dragon.center;
    this.key.target.position.set(c.x, c.y + 0.3 * L.s, c.z);
    this.key.position.set(c.x - 1.6 * reach, c.y + 2.6 * reach, c.z + 1.9 * reach);
  }

  dispose(): void {
    for (const layer of this.pieces) layer.dispose();
    this.treasure.dispose();
    this.dragon.dispose();
    this.portal.dispose();
    for (const mesh of [this.mound, this.floor, this.lit, this.flash]) {
      mesh.geometry.dispose();
      if (mesh.material !== this.matte) (mesh.material as THREE.Material).dispose();
    }
    this.matte.dispose();
    this.key.shadow.map?.dispose();
    this.removeFromParent();
  }

  /* ───────────────────────────── domains ──────────────────────────────── */

  private domainAt(index: number): DomainDef {
    const list = this.opts.domains && this.opts.domains.length > 0 ? this.opts.domains : DEFAULT_DOMAINS;
    if (index < list.length) return list[index];
    const last = list[list.length - 1];
    const beyond = index - list.length + 1;
    return { name: `${last.name} ${ROMAN[Math.min(beyond, ROMAN.length - 1)]}`, radius: last.radius * Math.pow(2.2, beyond) };
  }

  /** Pile volume at which a domain is full. */
  private domainCapacity(index: number): number {
    return VOL_K * Math.pow(PILE_FILL * this.domainAt(index).radius, 3);
  }

  private domainFor(volume: number): number {
    let i = 0;
    while (this.domainCapacity(i) < volume && i < 60) i++;
    return i;
  }

  private setDomain(index: number): void {
    if (index === this.domainIndex) return;
    this.domainIndex = index;
    const built = buildDomainGeometry(index, this.opts.seed ?? 1);
    this.floor.geometry.dispose();
    this.lit.geometry.dispose();
    this.floor.geometry = built.rock;
    this.lit.geometry = built.glow;
  }

  /* ───────────────────────────── pours ────────────────────────────────── */

  private volumeOf(perKind: number[]): number {
    return perKind.reduce((v, n, i) => v + n * KINDS[i].volume, 0);
  }

  /** Seconds the dragon takes to fly from the throne out to the ring. */
  private flightSeconds(): number {
    return clampN(this.dragon.leaveLength / (5.2 * this.layout.s), 1.9, 3.4);
  }

  /** Seconds a piece takes to fall from the portal to the crown of a pile of radius r. */
  private fallSeconds(r: number, exit: number): number {
    const drop = Math.max(0.01, this.portalPos.y + 0.3 - SLOPE * r * (1 - TOP));
    return (-exit + Math.sqrt(exit * exit + 2 * GRAVITY * drop)) / GRAVITY;
  }

  /**
   * How deep a layer of pieces is drawn under the surface. Anything deeper is hidden by the
   * pieces above it. `maxParticles` thins the layer on very large piles, but never below a
   * couple of pieces deep.
   */
  private keptDepth(R: number, each: number): number {
    const area = Math.PI * R * R * SLOPE_LENGTH;
    const budget = Math.max(1000, this.opts.maxParticles ?? 150_000);
    return clampN((budget * (each > 0 ? each : 7e-4)) / Math.max(area, 1e-6), 0.1, 0.22);
  }

  /** Share of every shape that lies within the drawn layer, for a pile of `volume` holding `pieces`. */
  private keptShare(volume: number, pieces: number): number {
    const R = radiusForVolume(volume);
    if (R <= 1e-6) return 1;
    const coreR = Math.max(0, R - this.keptDepth(R, pieces > 0 ? volume / pieces : 0) / (SLOPE * (1 - TOP)));
    return 1 - Math.pow(coreR / R, 3);
  }

  /** Lay the whole collection out at once, in the domain that fits it. */
  private settle(): void {
    this.pour = null;
    this.setPhase('idle');
    this.away = 0;
    this.carry = 0;
    this.portalGoal = 0;
    this.flash.visible = false;
    this.volume = this.volumeOf(this.kindCounts);
    this.setDomain(this.domainFor(this.volume));
    // Nothing is stored per piece, so this costs the same for two million as for two hundred.
    this.unbaked = true;
    this.stageQueued = false;
    this.pieces.forEach((layer, i) => {
      const n = this.kindCounts[i];
      layer.setRest(n);
    });
  }

  private setPhase(phase: HoardPhase): void {
    this.phase = phase;
    this.phaseT = 0;
  }

  /** Decide what happens now that nothing is falling: pour more, move domain, or go home. */
  private nextStage(): void {
    const shown = this.pieces.map((l) => l.count);
    const owed = this.kindCounts.map((n, i) => n - shown[i]);
    const remaining = this.volumeOf(owed);
    if (remaining <= 1e-9) {
      this.portalGoal = 0;
      this.setPhase(this.away > 0 ? 'return' : 'idle');
      return;
    }
    const capacity = this.domainCapacity(this.domainIndex);
    if (this.volume >= capacity * (1 - 1e-6)) {
      // The domain is full. Hold the rest back and move somewhere bigger.
      this.setPhase('ascend');
      this.switched = false;
      this.portalGoal = 1;
      this.portalR *= 1.5;
      this.dragon.roar();
      return;
    }

    // Pour as much of what is owed as this domain can take, as whole pieces.
    const share = Math.min(1, (capacity - this.volume) / remaining);
    const to = shown.map((n, i) => (share >= 1 ? this.kindCounts[i] : n + Math.ceil(share * owed[i])));
    const pieces = to.reduce((a, n, i) => a + n - shown[i], 0);
    const v1 = this.volume + this.volumeOf(to.map((n, i) => n - shown[i]));

    const mode = this.opts.ceremony ?? 'auto';
    const grand = mode === 'always' || (mode === 'auto' && (pieces >= 40 || v1 > this.volume * 1.25 + 0.01));
    const rT = radiusForVolume(v1);
    const sT = dragonScale(radiusForVolume(Math.max(v1, this.carry)));
    this.portalPos.set(0, portalHeight(rT, sT), 0);
    this.portalR = grand ? 0.35 + 0.16 * rT + 0.45 * sT : 0.1 + 0.45 * sT;
    this.portalGoal = 1;

    // The stream is paced by where the hoard is: the same relative growth takes the same time
    // at any size, so a bigger hoard gets a heavier stream. A very large pour is stretched just
    // enough that the surface is not replaced more than about twice a second.
    const landed = shown.reduce((a, n) => a + n, 0);
    const keptT = this.keptShare(v1, landed + pieces);
    const surface = Math.max(1, (landed + pieces) * keptT);
    const natural = lerp(2.5, 12, Math.sqrt(pieces / Math.max(1, landed + pieces))) * (share < 1 ? 0.6 : 1);
    const duration = this.pourSeconds ?? Math.max(natural, (pieces * 0.5) / surface);

    const exit = 1.5 + 0.6 * this.portalR;
    const fall = this.fallSeconds(radiusForVolume(this.volume), exit);
    const tail = 0.25 + 0.5 * Math.sqrt(rT * SLOPE_LENGTH);
    const inAir = Math.min(pieces, (pieces / duration) * fall, Math.max(100, this.opts.maxStream ?? 20_000));
    this.U.period.value = fall + 0.3;
    this.pieces.forEach((layer, i) => {
      const n = to[i] - shown[i];
      this.streamPool[i] = n > 0 ? Math.max(1, Math.round((inAir * n) / pieces)) : 0;
      // A piece must finish its run before newer ones push it out of the drawn layer.
      const runCap = n > 0 ? Math.max(0.3, (0.8 * to[i] * keptT * duration) / n) : 10;
      if (n > 0) layer.beginPour(shown[i], to[i], duration, runCap, this.streamPool[i]);
    });

    let delay = 0.45;
    if (grand && this.away < 1) {
      if (this.away <= 0 && this.phase !== 'pour') {
        // Summon the portal first, then back away; treasure starts once the dragon is clear.
        this.setPhase('summon');
        this.dragon.roar();
        delay = SUMMON_SECONDS + 0.35 * this.flightSeconds();
      } else {
        this.setPhase('pour');
      }
    } else {
      this.setPhase('pour');
    }
    this.ceremonial = grand || this.away > 0;
    this.pour = { v0: this.volume, v1, t: -delay, duration, tail, from: shown, to };
  }

  /** The moment, hidden by the flash, when the hoard moves to its next domain. */
  private changeDomain(): void {
    this.switched = true;
    this.carry = Math.max(this.carry, this.volume);
    this.setDomain(this.domainIndex + 1); // one domain at a time, so none is skipped on the way up
    this.volume = 0;
    this.away = 0;
    this.portal.open = 0;
    this.portalGoal = 0;
    for (const layer of this.pieces) layer.setRest(0);
  }

  private splitKinds(counts: Record<TreasureType, number>): number[] {
    const out = KINDS.map(() => 0);
    for (const type of TREASURE_TYPES) {
      const idx = KINDS.map((k, i) => (k.type === type ? i : -1)).filter((i) => i >= 0);
      let left = counts[type];
      for (let n = idx.length - 1; n > 0; n--) {
        const share = Math.round(counts[type] * KINDS[idx[n]].share);
        out[idx[n]] = Math.min(share, left);
        left -= out[idx[n]];
      }
      out[idx[0]] = left;
    }
    return out;
  }

  private applyLayout(): void {
    const L = this.layout;
    const D = this.domainAt(Math.max(0, this.domainIndex)).radius;
    L.R = radiusForVolume(this.volume);
    L.H = SLOPE * L.R;
    // The dragon keeps the size it earned in the last domain while the new pile catches up.
    const grown = Math.max(this.volume, this.carry);
    L.s = dragonScale(radiusForVolume(grown));
    const target = this.volumeOf(this.kindCounts);
    const reached = target > 0 ? this.total * clamp01(grown / target) : 0;
    L.age = clamp01(Math.log10(1 + reached) / Math.log10(1 + (this.opts.fullAt ?? 2_000_000)));
    L.domainR = D;
    L.rhoTip = Math.min(clampN(L.R * 0.85, 1.6 * L.s, 2.5 * L.s), 0.8 * D);
    // Back off just far enough to clear the pile as it will be once this pour has landed.
    const pileSoon = Math.max(L.R, this.pour ? radiusForVolume(this.pour.v1) : 0);
    L.retreatR = clampN(pileSoon * 1.08 + 1.9 * L.s + 0.3, L.rhoTip + 0.2, Math.max(L.rhoTip + 0.2, 0.86 * D - 0.25 * L.s));
  }
}

/* ════════════════════════════════ Arcade display ══════════════════════════════
 * An optional post-process that shows the scene as if on an arcade cabinet's tube.
 * The scene is drawn into a small fixed raster, then put through an analog video
 * chain: phosphor persistence, a soft glow, and a composite pass with RGB split,
 * horizontal bandwidth blur, scan lines, grain, hum, vignette and the odd tear.   */

export interface RetroOptions {
  /** Size of one picture pixel, in CSS pixels. Bigger is chunkier. Default 2. */
  pixelSize?: number;
  /** Fix the picture at this many scan lines instead, whatever the canvas size. Overrides `pixelSize`. */
  lines?: number;
  /** How crisp each pixel's edges are: 0 is a soft, smeary tube, 1 is hard-edged pixels. Default 1. */
  sharpness?: number;
  /** Shades per color channel after dithering (2 = stark, 0 = leave color alone). Default 10. */
  shades?: number;
  /** Strength of the glow around bright things. Default 0.5. */
  glow?: number;
  /** Darkness of the gaps between scan lines, 0..1. Default 0.22. */
  scanlines?: number;
  /** Red/blue fringing, in picture pixels. Default 0.7. */
  split?: number;
  /** Phosphor persistence: 0 none, 1 a slow tube that smears motion, more for longer trails. Default 0.35. */
  trails?: number;
  /** Film-grain amount. Default 0.6. */
  grain?: number;
}

const hash2 = (p: N): N => fract(sinN(p.dot(vec2(127.1, 311.7))).mul(43758.5453));

/**
 * Draws a scene through the arcade-tube chain. Use it in place of `renderer.render`:
 *
 *   const tube = new RetroDisplay(renderer);
 *   renderer.toneMapping = THREE.NoToneMapping;   // the chain grades the picture itself
 *   tube.render(scene, camera, dt);
 */
export class RetroDisplay {
  /** 0..1 picture tearing. Set it to kick a glitch; it settles on its own. */
  glitch = 0;

  private opts: Required<RetroOptions>;
  private readonly content: THREE.RenderTarget;
  private readonly persist: [THREE.RenderTarget, THREE.RenderTarget];
  private readonly bloom: [THREE.RenderTarget, THREE.RenderTarget];
  private readonly quad: THREE.QuadMesh;
  private readonly passes: Record<'persist' | 'down' | 'blurH' | 'blurV' | 'video', THREE.NodeMaterial>;
  private readonly tPrev: N;
  private readonly tPersist: N;
  private readonly uCres: N = uniform(new THREE.Vector2(400, 300));
  private readonly uOut: N = uniform(new THREE.Vector2(800, 600));
  private readonly uTime: N = uniform(0);
  private readonly uGlitch: N = uniform(0);
  private readonly uDecay: N = uniform(0);
  private readonly uSeed: N = uniform(0);
  private readonly uShades: N = uniform(6);
  private readonly uGlow: N = uniform(0.65);
  private readonly uScan: N = uniform(0.3);
  private readonly uSplit: N = uniform(1.6);
  private readonly uGrain: N = uniform(1);
  private readonly uSharp: N = uniform(1);
  private readonly size = new THREE.Vector2();
  private flip = 0;
  private time = 0;
  private seed = 0x12345678;
  private nextBlip = 6;

  private readonly renderer: THREE.WebGPURenderer;

  constructor(renderer: THREE.WebGPURenderer, options: RetroOptions = {}) {
    this.renderer = renderer;
    this.opts = { pixelSize: 2, lines: 0, sharpness: 1, shades: 10, glow: 0.5, scanlines: 0.22, split: 0.7, trails: 0.35, grain: 0.6, ...options };
    const hdr = { type: THREE.HalfFloatType, depthBuffer: false };
    this.content = new THREE.RenderTarget(4, 4, { type: THREE.HalfFloatType });
    this.persist = [new THREE.RenderTarget(4, 4, hdr), new THREE.RenderTarget(4, 4, hdr)];
    this.bloom = [new THREE.RenderTarget(4, 4, hdr), new THREE.RenderTarget(4, 4, hdr)];

    const at: N = uv();
    const px: N = vec2(1, 1).div(this.uCres);
    const tCur: N = texture(this.content.texture);
    this.tPrev = texture(this.persist[1].texture);
    this.tPersist = texture(this.persist[0].texture);
    const tBloomA: N = texture(this.bloom[0].texture);
    const tBloomB: N = texture(this.bloom[1].texture);

    // Grade the scene for the tube: tone-map to display values, then cut to a few shades with an ordered dither.
    const bayer2 = (a: N): N => fract(a.x.mul(0.5).add(a.y.mul(a.y).mul(0.75)));
    const grade = (where: N): N => {
      const mapped: N = acesFilmicToneMapping(tCur.sample(where).rgb, float(1));
      const display: N = mapped.max(0).pow(1 / 2.2);
      const luma: N = display.dot(vec3(0.299, 0.587, 0.114));
      const shown: N = mix(vec3(luma, luma, luma), display, 1.3).max(0); // arcade boards ran their colors hot
      const cell: N = where.mul(this.uCres).floor();
      const dither: N = bayer2(cell.mul(0.5).floor()).mul(0.25).add(bayer2(cell));
      const steps: N = this.uShades.sub(1).max(1);
      // A gentle dither: enough to break up banding without laying a weave over flat areas.
      return select(this.uShades.greaterThan(1.5), shown.mul(steps).add(dither.sub(0.5).mul(0.6).add(0.5)).floor().div(steps), shown);
    };

    // Phosphor: each pixel holds the brighter of the new picture (plus a faint echo) and its own fading past.
    const persist = new THREE.NodeMaterial();
    persist.fragmentNode = vec4(
      grade(at).add(grade(at.sub(vec2(px.x.mul(4), 0))).mul(float(0.07).mul(float(1).sub(this.uSharp)))).max(this.tPrev.sample(at).rgb.mul(this.uDecay)),
      1,
    );

    // Glow comes from the bright parts of the picture only, so a lit floor does not fog the screen.
    const bright = (where: N): N => this.tPersist.sample(where).rgb.sub(0.55).max(0).mul(2);
    const down = new THREE.NodeMaterial();
    down.fragmentNode = vec4(
      bright(at.add(vec2(px.x.negate(), px.y.negate())))
        .add(bright(at.add(vec2(px.x, px.y.negate()))))
        .add(bright(at.add(vec2(px.x.negate(), px.y))))
        .add(bright(at.add(px)))
        .mul(0.25),
      1,
    );
    const gaussian = (src: N, dir: N): N => {
      const t: N = dir.mul(px).mul(4 * 1.5);
      return src.sample(at).rgb.mul(0.227027)
        .add(src.sample(at.add(t.mul(1.3846154))).rgb.add(src.sample(at.sub(t.mul(1.3846154))).rgb).mul(0.3162162))
        .add(src.sample(at.add(t.mul(3.2307692))).rgb.add(src.sample(at.sub(t.mul(3.2307692))).rgb).mul(0.0702703));
    };
    const blurH = new THREE.NodeMaterial();
    blurH.fragmentNode = vec4(gaussian(tBloomA, vec2(1, 0)), 1);
    const blurV = new THREE.NodeMaterial();
    blurV.fragmentNode = vec4(gaussian(tBloomB, vec2(0, 1)), 1);

    // Analog video composite.
    const t: N = this.uTime;
    const g: N = this.uGlitch;
    const frame: N = t.mul(30).floor();
    const row: N = at.y.mul(this.uCres.y).mul(0.5).floor();
    const jitter: N = hash2(vec2(row, frame)).sub(0.5).mul(0.0012); // line jitter
    const blocks: N = hash2(vec2(t.mul(8).floor(), 2)).mul(40).floor().add(24); // glitch tears
    const band: N = at.y.mul(blocks).floor();
    const tear: N = select(hash2(vec2(band, t.mul(24).floor())).lessThan(g.mul(0.5)), hash2(vec2(band, frame.add(3))).sub(0.5).mul(0.12).mul(g), float(0));
    // Rows stay crisp like scan lines; only the horizontal is soft, like a tube's bandwidth.
    const line: N = vec2(at.x.add(jitter).add(tear), at.y.mul(this.uCres.y).floor().add(0.5).div(this.uCres.y));
    const soft = (where: N): N => {
      const dx: N = vec2(px.x.mul(0.9), 0);
      const smeared: N = this.tPersist.sample(where.sub(dx)).rgb.mul(0.27).add(this.tPersist.sample(where).rgb.mul(0.46)).add(this.tPersist.sample(where.add(dx)).rgb.mul(0.27));
      // Sharpness pulls each sample back to the middle of its own pixel, squaring the pixels off.
      const exact: N = this.tPersist.sample(vec2(where.x.mul(this.uCres.x).floor().add(0.5).div(this.uCres.x), where.y)).rgb;
      return mix(smeared, exact, this.uSharp);
    };
    const fringe: N = vec2(this.uSplit.add(g.mul(7)).mul(px.x), 0); // RGB split
    let col: N = vec3(soft(line.add(fringe)).r, soft(line).g, soft(line.sub(fringe)).b);
    col = col.add(tBloomA.sample(vec2(line.x, at.y)).rgb.mul(this.uGlow)); // glow
    const gaps: N = cosN(at.y.mul(this.uCres.y).mul(TAU)).mul(0.5).add(0.5);
    col = col.mul(float(1).sub(this.uScan.mul(gaps).mul(this.uOut.y.div(this.uCres.y).smoothstep(1.2, 2.2)))); // scan lines
    col = col.add(hash2(screenCoordinate.xy.add(vec2(this.uSeed.mul(91), this.uSeed.mul(17)))).sub(0.5).mul(0.04).mul(this.uGrain)); // grain
    col = col.mul(sinN(at.y.mul(9).sub(t.mul(2))).mul(0.03).add(0.97)); // hum bar
    const corner: N = at.x.mul(at.y).mul(float(1).sub(at.x)).mul(float(1).sub(at.y));
    col = col.mul(mix(float(0.72), float(1), corner.mul(16).clamp(0, 1).pow(0.35))); // vignette
    col = col.max(0).add(vec3(0.006, 0.004, 0.012)); // the tube is never quite black
    const video = new THREE.NodeMaterial();
    video.fragmentNode = vec4(col.pow(2.2), 1); // back to linear; the renderer encodes for the screen

    this.passes = { persist, down, blurH, blurV, video };
    this.quad = new THREE.QuadMesh(video);
    this.setOptions({});
  }

  setOptions(options: RetroOptions): void {
    this.opts = { ...this.opts, ...options };
    this.uShades.value = this.opts.shades;
    this.uGlow.value = this.opts.glow;
    this.uScan.value = this.opts.scanlines;
    this.uSplit.value = this.opts.split;
    this.uGrain.value = this.opts.grain;
    this.uSharp.value = clamp01(this.opts.sharpness);
  }

  /** Render the scene through the tube to the canvas, or to `output` if given. */
  render(scene: THREE.Scene, camera: THREE.Camera, dt: number, output: THREE.RenderTarget | null = null): void {
    const r = this.renderer;
    if (output) this.size.set(output.width, output.height);
    else r.getDrawingBufferSize(this.size);
    const outW = Math.max(1, this.size.x), outH = Math.max(1, this.size.y);
    // Picture height: a fixed line count if asked for, otherwise whatever gives pixels of the chosen size.
    const scale = output ? 1 : r.getPixelRatio();
    const wanted = this.opts.lines > 0 ? this.opts.lines : outH / (Math.max(1, this.opts.pixelSize) * scale);
    const h = Math.max(48, Math.min(Math.round(wanted), outH));
    const w = Math.max(48, Math.round((h * outW) / outH));
    if (this.content.width !== w || this.content.height !== h) {
      this.content.setSize(w, h);
      this.persist[0].setSize(w, h);
      this.persist[1].setSize(w, h);
      this.bloom[0].setSize(Math.ceil(w / 4), Math.ceil(h / 4));
      this.bloom[1].setSize(Math.ceil(w / 4), Math.ceil(h / 4));
    }

    this.time += dt;
    this.glitch = Math.max(0, this.glitch - dt * 1.8);
    this.nextBlip -= dt;
    if (this.nextBlip < 0) {
      // Like any old tube, it hiccups now and then.
      this.seed = (Math.imul(this.seed, 1664525) + 1013904223) >>> 0;
      this.nextBlip = 5 + ((this.seed >>> 8) / (1 << 24)) * 9;
      this.glitch = Math.max(this.glitch, 0.15 + ((this.seed >>> 4) & 255) / 1700);
    }
    this.seed = (Math.imul(this.seed, 1664525) + 1013904223) >>> 0;
    (this.uCres.value as THREE.Vector2).set(w, h);
    (this.uOut.value as THREE.Vector2).set(outW, outH);
    this.uTime.value = this.time;
    this.uGlitch.value = this.glitch;
    this.uDecay.value = this.opts.trails > 0 ? Math.pow(0.003, dt / this.opts.trails) : 0;
    this.uSeed.value = (this.seed >>> 8) / (1 << 24);

    const draw = (pass: THREE.NodeMaterial, target: THREE.RenderTarget | null) => {
      this.quad.material = pass;
      r.setRenderTarget(target);
      this.quad.render(r);
    };
    const previous = r.getRenderTarget();
    r.setRenderTarget(this.content);
    r.render(scene, camera);

    const now = this.persist[this.flip], before = this.persist[1 - this.flip];
    this.tPrev.value = before.texture;
    draw(this.passes.persist, now);
    this.tPersist.value = now.texture;
    draw(this.passes.down, this.bloom[0]);
    draw(this.passes.blurH, this.bloom[1]);
    draw(this.passes.blurV, this.bloom[0]);
    draw(this.passes.video, output);
    r.setRenderTarget(previous);
    this.flip = 1 - this.flip;
  }

  dispose(): void {
    this.content.dispose();
    this.persist.forEach((t) => t.dispose());
    this.bloom.forEach((t) => t.dispose());
    Object.values(this.passes).forEach((m) => m.dispose());
  }
}

/* ════════════════════════════════ Mounting ════════════════════════════════════ */

export interface MountOptions extends DragonHoardOptions {
  /** Canvas clear color, or null for a transparent canvas. Default null. */
  background?: THREE.ColorRepresentation | null;
  /** 'sway' rocks the camera gently, 'orbit' circles the hoard, 'none' holds still. Default 'sway'. */
  cameraMotion?: 'sway' | 'orbit' | 'none';
  /** Drag to look around. Default true. */
  interactive?: boolean;
  /** The dragon watches the pointer. Default true. */
  followPointer?: boolean;
  /** Clicking makes the dragon roar. Default true. */
  roarOnClick?: boolean;
  /** Scroll or pinch to zoom in on the center of the hoard. Default true. */
  zoom?: boolean;
  /** Show the scene as if on an arcade cabinet's tube. Pass options to tune the look. Default false. */
  retro?: boolean | RetroOptions;
  /**
   * A localStorage key. The hoard this person has watched arrive is saved under it, and next
   * time used as `lastSeen`: they see the hoard as they left it, then what is new streams in.
   */
  remember?: string;
  maxPixelRatio?: number;
  fov?: number;
  /** Skip WebGPU and use the WebGL2 backend. */
  forceWebGL?: boolean;
  /** Honor prefers-reduced-motion by stilling the camera. Default true. */
  respectReducedMotion?: boolean;
  /** Text alternative for the canvas. Defaults to a description built from the stats. */
  label?: string | ((stats: HoardStats) => string);
  onStats?: (stats: HoardStats) => void;
  /** Called once the GPU is up, and again if the widget has to restart on WebGL2. */
  onReady?: (info: { backend: HoardBackend; timings: LoadTimings }) => void;
  /** Called when everything, treasure included, is on screen. */
  onLoaded?: (timings: LoadTimings) => void;
}

export type HoardBackend = 'webgpu' | 'webgl2';

/** Milliseconds from mounting to each step of start-up. */
export interface LoadTimings {
  /** GPU device ready. */
  gpu: number;
  /** Dragon and scenery shaders compiled, first frame drawn. */
  firstFrame: number;
  /** Treasure shader compiled and treasure on screen; null until then. */
  treasure: number | null;
}

export interface DragonHoardHandle {
  /** These four are replaced if the widget restarts on WebGL2 after losing the WebGPU device. */
  readonly hoard: DragonHoard;
  readonly renderer: THREE.WebGPURenderer;
  readonly scene: THREE.Scene;
  readonly camera: THREE.PerspectiveCamera;
  readonly canvas: HTMLCanvasElement;
  /** Resolves once the first frame is up; `backend` says which path three picked. */
  readonly ready: Promise<{ backend: HoardBackend }>;
  /** How long start-up took, step by step. */
  readonly timings: LoadTimings;
  setCounts(counts: HoardCounts, pour?: PourOptions): HoardStats;
  add(counts: HoardCounts, pour?: PourOptions): HoardStats;
  setOptions(options: Partial<MountOptions>): void;
  roar(): void;
  /** Undo any zooming and orbiting. */
  resetView(): void;
  /** Turn the arcade-tube look on or off, or retune it. */
  setRetro(retro: boolean | RetroOptions): void;
  pause(): void;
  resume(): void;
  readonly stats: HoardStats;
  dispose(): void;
}

/** One running renderer + scene. `mountDragonHoard` owns one of these and can swap it for another. */
interface Stage {
  hoard: DragonHoard;
  renderer: THREE.WebGPURenderer;
  scene: THREE.Scene;
  camera: THREE.PerspectiveCamera;
  canvas: HTMLCanvasElement;
  apply(next: Partial<MountOptions>): void;
  resetView(): void;
  applyRetro(): void;
  timings: LoadTimings;
  publish(): void;
  sync(): void;
  dispose(): void;
}

/** Create a canvas inside `container` (which needs a size of its own) and run the hoard in it. */
export function mountDragonHoard(container: HTMLElement, options: MountOptions = {}): DragonHoardHandle {
  let opts: MountOptions = { ...options };
  let paused = false, visible = true, disposed = false;
  let resolveReady!: (v: { backend: HoardBackend }) => void;
  const ready = new Promise<{ backend: HoardBackend }>((res) => { resolveReady = res; });
  const reduced = opts.respectReducedMotion !== false && typeof matchMedia === 'function' && matchMedia('(prefers-reduced-motion: reduce)').matches;

  const describe = (s: HoardStats) => {
    if (typeof opts.label === 'string') return opts.label;
    if (typeof opts.label === 'function') return opts.label(s);
    return s.total === 0
      ? 'A sleeping dragon hatchling on bare rock, waiting for a hoard.'
      : `A ${s.tier} dragon guarding a hoard of ${s.total.toLocaleString()} treasures in the ${s.domain.name}.`;
  };

  // What this person last watched arrive, if we were asked to keep track.
  let saved = '';
  const recall = (): HoardCounts | undefined => {
    if (!opts.remember) return undefined;
    try {
      const raw = localStorage.getItem(opts.remember);
      const parsed: unknown = raw ? JSON.parse(raw) : null;
      if (!parsed || typeof parsed !== 'object') return undefined;
      const seen: HoardCounts = {};
      for (const t of TREASURE_TYPES) {
        const n = Number((parsed as Record<string, unknown>)[t]);
        if (Number.isFinite(n) && n > 0) seen[t] = Math.floor(n);
      }
      return seen;
    } catch {
      return undefined; // storage can be unavailable; then every visit simply starts at `counts`
    }
  };
  const keep = (stats: HoardStats) => {
    if (!opts.remember || stats.pouring) return; // only what has finished arriving counts as seen
    const text = JSON.stringify(stats.counts);
    if (text === saved) return;
    saved = text;
    try { localStorage.setItem(opts.remember, text); } catch { /* nowhere to keep it */ }
  };

  const createStage = (forceWebGL: boolean, counts: HoardCounts, lastSeen: HoardCounts | undefined): Stage => {
    const renderer = new THREE.WebGPURenderer({ antialias: true, alpha: true, forceWebGL });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, opts.maxPixelRatio ?? 2));
    renderer.toneMappingExposure = 1.0;
    renderer.shadowMap.enabled = opts.shadows !== false;

    const canvas = renderer.domElement as HTMLCanvasElement;
    Object.assign(canvas.style, { display: 'block', width: '100%', height: '100%', touchAction: 'pan-y' });
    canvas.setAttribute('role', 'img');
    container.appendChild(canvas);

    const scene = new THREE.Scene();
    const applyBackground = () => {
      scene.background = opts.background == null ? null : new THREE.Color(opts.background);
      renderer.setClearColor(0x000000, opts.background == null ? 0 : 1);
    };
    applyBackground();

    const camera = new THREE.PerspectiveCamera(opts.fov ?? 30, 1, 0.1, 600);
    const hoard = new DragonHoard({ ...opts, counts, lastSeen });
    scene.add(hoard);

    // Camera rig: user azimuth/elevation plus an optional idle motion, distance fitted to the pile.
    let azimuth = 0.18, elevation = 0.3, idle = 0;
    let dist = 0, focusY = 0, goalDist = 10, goalFocus = 1;
    const fit = () => {
      const b = hoard.bounds;
      const half = Math.tan((camera.fov * DEG) / 2);
      const tall = (Math.max(b.height - b.focus, b.focus) * 1.12 + 0.2) / half;
      const wide = (b.radius * 1.02) / (half * camera.aspect);
      goalDist = Math.max(tall, wide) + b.radius * 0.55;
      goalFocus = b.focus;
    };

    // Zoom: 1 frames the hoard; smaller moves in along the same line. The camera always orbits the center.
    let zoom = 1;
    const pointer = new THREE.Vector2();
    const touches = new Map<number, { x: number; y: number }>();
    let pinch = 0;
    let pointerInside = false, dragging = false, dragTravel = 0, lastX = 0, lastY = 0, holdUntil = 0;
    const ndc = (x: number, y: number) => {
      const r = canvas.getBoundingClientRect();
      pointer.set(((x - r.left) / Math.max(1, r.width)) * 2 - 1, -(((y - r.top) / Math.max(1, r.height)) * 2 - 1));
    };
    const zoomBy = (factor: number) => {
      const closest = clampN(1.2 / Math.max(dist, 1e-3), 0.004, 1);
      zoom = clampN(zoom * factor, closest, 1.25);
      holdUntil = performance.now() + 2500;
    };
    const onMove = (e: PointerEvent) => {
      if (touches.has(e.pointerId)) touches.set(e.pointerId, { x: e.clientX, y: e.clientY });
      if (touches.size === 2 && opts.zoom !== false) {
        const [a, b] = [...touches.values()];
        const span = Math.hypot(a.x - b.x, a.y - b.y);
        ndc((a.x + b.x) / 2, (a.y + b.y) / 2);
        if (pinch > 0 && span > 0) zoomBy(pinch / span);
        pinch = span;
        dragTravel += 10; // a pinch is never a click
        return;
      }
      ndc(e.clientX, e.clientY);
      pointerInside = true;
      if (dragging) {
        const dx = e.clientX - lastX, dy = e.clientY - lastY;
        lastX = e.clientX; lastY = e.clientY;
        dragTravel += Math.abs(dx) + Math.abs(dy);
        azimuth -= dx * 0.008;
        elevation = clampN(elevation + dy * 0.005, 0.06, 1.15);
      }
    };
    const onDown = (e: PointerEvent) => {
      ndc(e.clientX, e.clientY);
      if (e.pointerType === 'touch') touches.set(e.pointerId, { x: e.clientX, y: e.clientY });
      pinch = 0;
      if (opts.interactive === false) return;
      dragging = true; dragTravel = 0; lastX = e.clientX; lastY = e.clientY;
      canvas.setPointerCapture?.(e.pointerId);
    };
    const onUp = (e: PointerEvent) => {
      const wasDrag = dragging && dragTravel > 6;
      touches.delete(e.pointerId);
      pinch = 0;
      dragging = touches.size > 0 && dragging;
      lastX = e.clientX; lastY = e.clientY;
      canvas.releasePointerCapture?.(e.pointerId);
      holdUntil = performance.now() + 2500;
      if (!wasDrag && touches.size === 0 && opts.roarOnClick !== false) hoard.roar();
    };
    const onLeave = (e: PointerEvent) => { pointerInside = false; dragging = false; touches.delete(e.pointerId); pinch = 0; };
    const onWheel = (e: WheelEvent) => {
      if (opts.zoom === false) return;
      e.preventDefault();
      ndc(e.clientX, e.clientY);
      zoomBy(Math.exp(clampN(e.deltaY * (e.deltaMode === 1 ? 16 : 1), -240, 240) * 0.0016));
    };
    canvas.addEventListener('pointermove', onMove);
    canvas.addEventListener('pointerdown', onDown);
    canvas.addEventListener('pointerup', onUp);
    canvas.addEventListener('pointerleave', onLeave);
    canvas.addEventListener('pointercancel', onLeave);
    canvas.addEventListener('wheel', onWheel, { passive: false });

    const resize = () => {
      const w = Math.max(1, container.clientWidth), h = Math.max(1, container.clientHeight);
      renderer.setSize(w, h, false);
      camera.aspect = w / h;
      camera.updateProjectionMatrix();
      fit();
    };
    const resizeObserver = new ResizeObserver(resize);
    resizeObserver.observe(container);
    resize();

    let lastLabel = '';
    let lastStatsAt = 0;
    const publish = (force: boolean) => {
      const now = performance.now();
      if (!force && now - lastStatsAt < 250) return;
      lastStatsAt = now;
      const s = hoard.stats;
      const label = describe(s);
      if (label !== lastLabel) { canvas.setAttribute('aria-label', label); lastLabel = label; }
      keep(s);
      opts.onStats?.(s);
    };

    // Arcade tube: created on first use. While it is on, it does the tone mapping itself.
    let tube: RetroDisplay | null = null;
    let lastPhase: HoardPhase = 'idle';
    const applyRetro = () => {
      const on = opts.retro !== undefined && opts.retro !== false;
      if (on && !tube) tube = new RetroDisplay(renderer);
      if (on && typeof opts.retro === 'object') tube?.setOptions(opts.retro);
      renderer.toneMapping = on ? THREE.NoToneMapping : THREE.ACESFilmicToneMapping;
    };
    applyRetro();

    let started = false, running = false, gone = false;
    let last = 0;
    const target = new THREE.Vector3(), right = new THREE.Vector3(), up = new THREE.Vector3();

    const frame = (now: number) => {
      if (gone) return;
      if (!started) {
        started = true;
        last = now;
      }
      const dt = Math.min(0.05, Math.max(0, (now - last) / 1000));
      last = now;

      fit();
      const k = dist === 0 ? 1 : 1 - Math.exp(-dt * 2.5);
      dist += (goalDist - dist) * k;
      focusY += (goalFocus - focusY) * k;
      const motion = reduced ? 'none' : opts.cameraMotion ?? 'sway';
      if (!dragging && now > holdUntil) {
        if (motion === 'orbit') azimuth += dt * 0.15;
        else if (motion === 'sway') idle += dt;
      }
      const az = azimuth + (motion === 'sway' ? Math.sin(idle * 0.25) * 0.4 * Math.min(1, zoom) : 0);
      const reach = dist * zoom;
      const range = Math.max(600, dist * 6);
      const near = clampN(reach * 0.02, 0.02, 0.5);
      if (Math.abs(camera.far - range) > 1 || Math.abs(camera.near - near) > near * 0.2) {
        camera.far = range;
        camera.near = near;
        camera.updateProjectionMatrix();
      }
      camera.position.set(Math.sin(az) * Math.cos(elevation) * reach, focusY + Math.sin(elevation) * reach, Math.cos(az) * Math.cos(elevation) * reach);
      camera.lookAt(0, focusY, 0);
      camera.updateMatrixWorld();

      // The dragon watches the pointer if there is one, the camera otherwise.
      target.copy(camera.position);
      if (pointerInside && opts.followPointer !== false) {
        right.setFromMatrixColumn(camera.matrixWorld, 0);
        up.setFromMatrixColumn(camera.matrixWorld, 1);
        target.addScaledVector(right, pointer.x * reach * 0.45).addScaledVector(up, pointer.y * reach * 0.3);
      }
      hoard.setLookTarget(target);

      hoard.update(dt);
      if (tube && opts.retro !== undefined && opts.retro !== false) {
        // The picture tears when the dragon calls the portal, and badly when the hoard changes domain.
        const phase = hoard.stats.phase;
        if (phase !== lastPhase && phase === 'summon') tube.glitch = Math.max(tube.glitch, 0.3);
        if (phase !== lastPhase && phase === 'ascend') tube.glitch = Math.max(tube.glitch, 0.6);
        lastPhase = phase;
        tube.render(scene, camera, dt);
      } else {
        renderer.render(scene, camera);
      }
      publish(false);
    };

    let primed = false;
    const sync = () => {
      const should = primed && !paused && visible && !document.hidden && !gone;
      if (should === running) return;
      running = should;
      last = performance.now();
      void renderer.setAnimationLoop(should ? frame : null);
    };

    // Start-up in steps, none of which blocks the page: bring the GPU up; compile the dragon and
    // scenery and start drawing; compile the treasure shader and show the treasure; then compile
    // what is only needed later (the portal, the domain change).
    const began = performance.now();
    const timings: LoadTimings = { gpu: 0, firstFrame: 0, treasure: null };
    hoard.treasureHidden = true;
    void (async () => {
      try {
        await renderer.init();
        if (gone) return;
        timings.gpu = Math.round(performance.now() - began);
        hoard.update(0);
        await renderer.compileAsync(scene, camera);
      } catch (err) {
        console.warn('[dragon-hoard] could not compile ahead of time; shaders will compile as they are first used.', err);
      }
      if (gone) return;
      const backend = renderer.backend as { isWebGPUBackend?: boolean; device?: { lost: Promise<{ message: string }> } };
      const isGpu = backend.isWebGPUBackend === true;
      // If the WebGPU device goes away (driver reset, sandboxed GPU), carry on in WebGL2.
      if (isGpu) void backend.device?.lost.then((info) => { if (!gone) restartOnWebGL(info.message); });
      primed = true;
      sync();
      timings.firstFrame = Math.round(performance.now() - began);
      const info = { backend: (isGpu ? 'webgpu' : 'webgl2') as HoardBackend, timings };
      resolveReady(info);
      opts.onReady?.(info);
      try { await hoard.primeTreasure(renderer, camera); } catch { hoard.treasureHidden = false; }
      if (gone) return;
      timings.treasure = Math.round(performance.now() - began);
      opts.onLoaded?.(timings);
      try { await hoard.prime(renderer, camera); } catch { /* they will compile on first use instead */ }
    })();

    return {
      hoard, renderer, scene, camera, canvas, sync, timings,
      publish: () => publish(true),
      resetView() { zoom = 1; azimuth = 0.18; elevation = 0.3; },
      applyRetro,
      apply(next) {
        hoard.setOptions(next);
        if ('background' in next) applyBackground();
        if (next.shadows !== undefined) renderer.shadowMap.enabled = next.shadows;
        if ('retro' in next) applyRetro();
        if (next.fov) { camera.fov = next.fov; camera.updateProjectionMatrix(); }
        if (next.maxPixelRatio) { renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, next.maxPixelRatio)); resize(); }
      },
      dispose() {
        if (gone) return;
        gone = true;
        void renderer.setAnimationLoop(null);
        resizeObserver.disconnect();
        canvas.removeEventListener('pointermove', onMove);
        canvas.removeEventListener('pointerdown', onDown);
        canvas.removeEventListener('pointerup', onUp);
        canvas.removeEventListener('pointerleave', onLeave);
        canvas.removeEventListener('pointercancel', onLeave);
        canvas.removeEventListener('wheel', onWheel);
        hoard.dispose();
        tube?.dispose();
        try { renderer.dispose(); } catch { /* a lost device may refuse to clean up */ }
        canvas.remove();
      },
    };
  };

  let stage = createStage(opts.forceWebGL === true, opts.counts ?? {}, opts.lastSeen ?? recall());

  function restartOnWebGL(reason: string): void {
    if (disposed) return;
    console.warn(`[dragon-hoard] WebGPU device lost (${reason || 'no reason given'}); restarting on WebGL2.`);
    const counts = stage.hoard.stats.counts;
    stage.dispose();
    opts = { ...opts, forceWebGL: true };
    // The GPU process may need a moment before it hands out contexts again.
    let tries = 0;
    const attempt = () => {
      if (disposed) return;
      const probe = document.createElement('canvas').getContext('webgl2');
      if (probe) {
        probe.getExtension('WEBGL_lose_context')?.loseContext();
        stage = createStage(true, counts, undefined);
        stage.sync();
      } else if (++tries < 10) {
        setTimeout(attempt, 400);
      } else {
        console.error('[dragon-hoard] no WebGPU or WebGL2 context is available; giving up.');
      }
    };
    attempt();
  }

  const intersection = new IntersectionObserver((entries) => {
    visible = entries[entries.length - 1].isIntersecting;
    stage.sync();
  });
  intersection.observe(container);
  const onVisibility = () => stage.sync();
  document.addEventListener('visibilitychange', onVisibility);
  stage.sync();

  return {
    get hoard() { return stage.hoard; },
    get renderer() { return stage.renderer; },
    get scene() { return stage.scene; },
    get camera() { return stage.camera; },
    get canvas() { return stage.canvas; },
    get timings() { return stage.timings; },
    ready,
    setCounts(counts, pour) {
      const stats = stage.hoard.setCounts(counts, pour);
      stage.publish();
      return stats;
    },
    add(counts, pour) {
      const stats = stage.hoard.deposit(counts, pour);
      stage.publish();
      return stats;
    },
    setOptions(next) {
      opts = { ...opts, ...next };
      stage.apply(next);
      stage.publish();
    },
    roar: () => stage.hoard.roar(),
    resetView: () => stage.resetView(),
    setRetro(retro) {
      opts = { ...opts, retro };
      stage.applyRetro();
    },
    pause() { paused = true; stage.sync(); },
    resume() { paused = false; stage.sync(); },
    get stats() { return stage.hoard.stats; },
    dispose() {
      if (disposed) return;
      disposed = true;
      intersection.disconnect();
      document.removeEventListener('visibilitychange', onVisibility);
      stage.dispose();
    },
  };
}
