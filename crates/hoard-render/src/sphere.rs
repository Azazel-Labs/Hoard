//! A material swatch: a sphere shaded on the CPU from a set of PBR maps, for material thumbnails.
//!
//! No GPU and no meshes: every pixel of the sphere is worked out analytically (position, normal, texture
//! coordinates, tangent frame), its maps sampled and lit with the usual metal/roughness model: a GGX key light
//! plus a soft sky and ground. Gloss maps are inverted to roughness, DirectX normal maps have their green flipped,
//! packed maps (ORM, MRA, RMA) are unpacked, and a height or bump map stands in for a missing normal map. Four
//! samples a pixel smooth the edge, which is left transparent.
#![expect(
    clippy::many_single_char_names,
    clippy::similar_names,
    reason = "shading maths reads best with its usual names: u, v, n, l, h, dhdu, dhdv"
)]

use std::f32::consts::PI;

use glam::Vec3;
use image::{Rgba, RgbaImage};

/// How a packed map orders its channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Packed {
    /// Red occlusion, green roughness, blue metalness (also "ARM").
    Orm,
    /// Red metalness, green roughness, blue occlusion.
    Mra,
    /// Red roughness, green metalness, blue occlusion.
    Rma,
}

/// The maps a material swatch is shaded with. Any can be missing.
#[derive(Debug, Default)]
pub struct Maps {
    /// sRGB colour.
    pub base_color: Option<RgbaImage>,
    pub normal: Option<RgbaImage>,
    /// The normal map is DirectX-style (green points down).
    pub normal_dx: bool,
    /// Roughness, or gloss when `gloss` is set.
    pub roughness: Option<RgbaImage>,
    pub gloss: bool,
    pub metalness: Option<RgbaImage>,
    pub ao: Option<RgbaImage>,
    pub cavity: Option<RgbaImage>,
    /// Height, displacement or bump: only used when there's no normal map.
    pub height: Option<RgbaImage>,
    pub opacity: Option<RgbaImage>,
    /// sRGB colour.
    pub emissive: Option<RgbaImage>,
    pub packed: Option<(RgbaImage, Packed)>,
}

/// Times the texture wraps around the sphere: twice around and once top to bottom keeps square texels square.
const TILE_U: f32 = 2.0;
/// The sphere fills this much of the picture.
const FILL: f32 = 0.92;
const SUPERSAMPLE: u32 = 2;

/// Render the swatch as a `size` x `size` image with a transparent background.
pub fn render_sphere(maps: &Maps, size: u32) -> RgbaImage {
    let lut = srgb_lut();
    let shader = Shader::new(maps, &lut);
    let big = size * SUPERSAMPLE;
    let mut out = RgbaImage::new(size, size);
    for oy in 0..size {
        for ox in 0..size {
            // Premultiplied sum of the samples, so the edge blends into transparency rather than a dark fringe.
            let mut acc = [0.0f32; 4];
            for sy in 0..SUPERSAMPLE {
                for sx in 0..SUPERSAMPLE {
                    let px = (ox * SUPERSAMPLE + sx) as f32 + 0.5;
                    let py = (oy * SUPERSAMPLE + sy) as f32 + 0.5;
                    let x = (px / big as f32 * 2.0 - 1.0) / FILL;
                    let y = (1.0 - py / big as f32 * 2.0) / FILL;
                    if let Some((rgb, a)) = shader.shade(x, y) {
                        acc[0] += rgb.x * a;
                        acc[1] += rgb.y * a;
                        acc[2] += rgb.z * a;
                        acc[3] += a;
                    }
                }
            }
            let n = (SUPERSAMPLE * SUPERSAMPLE) as f32;
            let a = acc[3] / n;
            if a <= 0.0 {
                continue;
            }
            let c = |v: f32| (linear_to_srgb(v / acc[3]) * 255.0).round().clamp(0.0, 255.0) as u8;
            out.put_pixel(ox, oy, Rgba([c(acc[0]), c(acc[1]), c(acc[2]), (a * 255.0).round() as u8]));
        }
    }
    out
}

/// One texture, sampled with bilinear filtering, wrapping across and clamped top to bottom.
struct Tex<'a>(&'a RgbaImage);

impl Tex<'_> {
    fn texel(&self, x: i64, y: i64) -> [f32; 4] {
        let (w, h) = (i64::from(self.0.width()), i64::from(self.0.height()));
        let p = self.0.get_pixel(x.rem_euclid(w) as u32, y.clamp(0, h - 1) as u32).0;
        [f32::from(p[0]), f32::from(p[1]), f32::from(p[2]), f32::from(p[3])]
    }

    /// Bilinear sample at texture coordinates (0..1 across one tile, `v` down from the top), as 0..255 channels.
    fn sample(&self, u: f32, v: f32) -> [f32; 4] {
        let x = u * self.0.width() as f32 - 0.5;
        let y = v * self.0.height() as f32 - 0.5;
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let (x0, y0) = (x0 as i64, y0 as i64);
        let (a, b, c, d) =
            (self.texel(x0, y0), self.texel(x0 + 1, y0), self.texel(x0, y0 + 1), self.texel(x0 + 1, y0 + 1));
        std::array::from_fn(|i| {
            let top = a[i] + (b[i] - a[i]) * fx;
            let bottom = c[i] + (d[i] - c[i]) * fx;
            top + (bottom - top) * fy
        })
    }

    /// The first channel, 0..1 (grey maps).
    fn value(&self, u: f32, v: f32) -> f32 {
        self.sample(u, v)[0] / 255.0
    }

    fn texel_size(&self) -> (f32, f32) {
        (1.0 / self.0.width().max(1) as f32, 1.0 / self.0.height().max(1) as f32)
    }
}

struct Shader<'a> {
    maps: &'a Maps,
    lut: &'a [f32; 256],
    light: Vec3,
}

impl<'a> Shader<'a> {
    fn new(maps: &'a Maps, lut: &'a [f32; 256]) -> Self {
        Self { maps, lut, light: Vec3::new(-0.55, 0.65, 0.75).normalize() }
    }

    fn tex(img: Option<&'a RgbaImage>) -> Option<Tex<'a>> {
        img.filter(|i| i.width() > 0 && i.height() > 0).map(Tex)
    }

    fn srgb(&self, s: [f32; 4]) -> Vec3 {
        let l = |v: f32| self.lut[v.round().clamp(0.0, 255.0) as usize];
        Vec3::new(l(s[0]), l(s[1]), l(s[2]))
    }

    /// Colour (linear) and coverage of the sphere at picture coordinates -1..1, or None off the sphere.
    fn shade(&self, x: f32, y: f32) -> Option<(Vec3, f32)> {
        let r2 = x * x + y * y;
        if r2 > 1.0 {
            return None;
        }
        let n = Vec3::new(x, y, (1.0 - r2).sqrt());
        let u = (n.x.atan2(n.z) / (2.0 * PI) + 0.5) * TILE_U;
        let v = n.y.clamp(-1.0, 1.0).acos() / PI;
        // Along increasing u, and toward the top of the texture (the north pole).
        let tangent = Vec3::new(n.z, 0.0, -n.x).try_normalize().unwrap_or(Vec3::X);
        let bitangent = n.cross(tangent);
        let m = self.maps;

        let base = Self::tex(m.base_color.as_ref()).map_or(Vec3::splat(0.6), |t| self.srgb(t.sample(u, v)));
        let packed =
            m.packed.as_ref().and_then(|(img, layout)| Self::tex(Some(img)).map(|t| (t.sample(u, v), *layout)));
        let (mut ao, mut rough, mut metal) = (1.0, 0.6, 0.0);
        if let Some((s, layout)) = packed {
            let c = |i: usize| s[i] / 255.0;
            (ao, rough, metal) = match layout {
                Packed::Orm => (c(0), c(1), c(2)),
                Packed::Mra => (c(2), c(1), c(0)),
                Packed::Rma => (c(2), c(0), c(1)),
            };
        }
        if let Some(t) = Self::tex(m.roughness.as_ref()) {
            let r = t.value(u, v);
            rough = if m.gloss { 1.0 - r } else { r };
        }
        if let Some(t) = Self::tex(m.metalness.as_ref()) {
            metal = t.value(u, v);
        }
        if let Some(t) = Self::tex(m.ao.as_ref()) {
            ao = t.value(u, v);
        }
        if let Some(t) = Self::tex(m.cavity.as_ref()) {
            ao *= 0.5 + 0.5 * t.value(u, v);
        }
        let rough = rough.clamp(0.045, 1.0);

        let shading_normal = if let Some(t) = Self::tex(m.normal.as_ref()) {
            let s = t.sample(u, v);
            let mut tn = Vec3::new(s[0] / 127.5 - 1.0, s[1] / 127.5 - 1.0, s[2] / 127.5 - 1.0);
            if m.normal_dx {
                tn.y = -tn.y;
            }
            (tangent * tn.x + bitangent * tn.y + n * tn.z.max(0.05)).try_normalize().unwrap_or(n)
        } else if let Some(t) = Self::tex(m.height.as_ref()) {
            let (du, dv) = t.texel_size();
            let dhdu = t.value(u + du, v) - t.value(u - du, v);
            let dhdv = t.value(u, v + dv) - t.value(u, v - dv);
            let k = 3.0;
            (tangent * (-dhdu * k) + bitangent * (dhdv * k) + n).try_normalize().unwrap_or(n)
        } else {
            n
        };

        let view = Vec3::Z;
        let nv = shading_normal.dot(view).max(1e-3);
        let f0 = Vec3::splat(0.04).lerp(base, metal);
        let diffuse = base * (1.0 - metal);

        // Key light: GGX with Smith-Schlick visibility.
        let l = self.light;
        let nl = shading_normal.dot(l).max(0.0);
        let mut color = Vec3::ZERO;
        if nl > 0.0 {
            let h = (l + view).normalize();
            let nh = shading_normal.dot(h).max(0.0);
            let vh = view.dot(h).max(0.0);
            let a2 = (rough * rough) * (rough * rough);
            let d = a2 / (PI * (nh * nh * (a2 - 1.0) + 1.0).powi(2));
            let k = (rough + 1.0) * (rough + 1.0) / 8.0;
            let g = (nl / (nl * (1.0 - k) + k)) * (nv / (nv * (1.0 - k) + k));
            let f = f0 + (Vec3::ONE - f0) * (1.0 - vh).powi(5);
            let spec = f * (d * g / (4.0 * nl * nv + 1e-4));
            let kd = Vec3::ONE - f;
            color += (kd * diffuse / PI + spec) * nl * 2.6;
        }

        // Sky and ground: diffuse from the hemisphere, specular from the reflected direction, blurred with roughness.
        let irradiance = sky_irradiance(shading_normal);
        let reflected = (shading_normal * 2.0 * nv - view).normalize();
        let env = sky(reflected).lerp(sky_irradiance(reflected), rough.sqrt());
        let (scale, bias) = env_brdf(rough, nv);
        let spec_occlusion = ao.powf(0.5 + rough);
        color += diffuse * irradiance * ao + env * (f0 * scale + Vec3::splat(bias)) * spec_occlusion;

        if let Some(t) = Self::tex(m.emissive.as_ref()) {
            color += self.srgb(t.sample(u, v)) * 1.5;
        }
        let alpha = Self::tex(m.opacity.as_ref()).map_or(1.0, |t| t.value(u, v));
        Some((aces(color), alpha))
    }
}

/// The sky seen in a direction: bright toward the horizon, bluer overhead, a dark warm ground.
fn sky(d: Vec3) -> Vec3 {
    let horizon = Vec3::new(0.95, 0.95, 0.97);
    let zenith = Vec3::new(0.45, 0.55, 0.75);
    let ground = Vec3::new(0.16, 0.14, 0.12);
    if d.y >= 0.0 { horizon.lerp(zenith, d.y.powf(0.6)) } else { horizon.lerp(ground, (-d.y).powf(0.35)) }
}

/// Light arriving at a surface facing `n`, averaged over its hemisphere.
fn sky_irradiance(n: Vec3) -> Vec3 {
    let up = Vec3::new(0.72, 0.76, 0.85);
    let down = Vec3::new(0.24, 0.22, 0.2);
    down.lerp(up, 0.5 + 0.5 * n.y)
}

/// Karis' fit of the split-sum environment BRDF: (scale, bias) for F0.
fn env_brdf(rough: f32, nv: f32) -> (f32, f32) {
    let c0 = [-1.0, -0.0275, -0.572, 0.022];
    let c1 = [1.0, 0.0425, 1.04, -0.04];
    let r: [f32; 4] = std::array::from_fn(|i| rough * c0[i] + c1[i]);
    let a004 = (r[0] * r[0]).min((-9.28 * nv).exp2()) * r[0] + r[1];
    (a004 * -1.04 + r[2], a004 * 1.04 + r[3])
}

/// Narkowicz's fit of the ACES filmic curve.
fn aces(c: Vec3) -> Vec3 {
    let f = |x: f32| ((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14)).clamp(0.0, 1.0);
    Vec3::new(f(c.x), f(c.y), f(c.z))
}

fn srgb_lut() -> [f32; 256] {
    std::array::from_fn(|i| {
        let c = i as f32 / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    })
}

fn linear_to_srgb(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(rgb: [u8; 3]) -> RgbaImage {
        RgbaImage::from_pixel(8, 8, Rgba([rgb[0], rgb[1], rgb[2], 255]))
    }

    #[test]
    fn sphere_on_transparent_background() {
        let img = render_sphere(&Maps { base_color: Some(flat([200, 30, 30])), ..Maps::default() }, 64);
        assert_eq!(img.dimensions(), (64, 64));
        assert_eq!(img.get_pixel(0, 0)[3], 0, "corner is transparent");
        let mid = img.get_pixel(32, 32);
        assert_eq!(mid[3], 255);
        assert!(mid[0] > mid[1] * 2 && mid[0] > mid[2] * 2, "red material reads red: {mid:?}");
    }

    #[test]
    fn gloss_is_inverted_roughness() {
        let shiny_by_gloss = Maps {
            base_color: Some(flat([40, 40, 40])),
            roughness: Some(flat([255, 255, 255])),
            gloss: true,
            ..Maps::default()
        };
        let shiny_by_rough =
            Maps { base_color: Some(flat([40, 40, 40])), roughness: Some(flat([0, 0, 0])), ..Maps::default() };
        assert_eq!(render_sphere(&shiny_by_gloss, 32), render_sphere(&shiny_by_rough, 32));
    }

    #[test]
    fn packed_orm_matches_separate_maps() {
        let packed = Maps {
            base_color: Some(flat([180, 140, 90])),
            packed: Some((flat([255, 100, 200]), Packed::Orm)),
            ..Maps::default()
        };
        let separate = Maps {
            base_color: Some(flat([180, 140, 90])),
            ao: Some(flat([255, 255, 255])),
            roughness: Some(flat([100, 100, 100])),
            metalness: Some(flat([200, 200, 200])),
            ..Maps::default()
        };
        assert_eq!(render_sphere(&packed, 32), render_sphere(&separate, 32));
    }

    #[test]
    fn dx_normals_flip_green() {
        // A normal map tilted "up" in OpenGL terms is tilted "down" read as DirectX: the lit side moves.
        let tilted = flat([128, 200, 200]);
        let gl = render_sphere(&Maps { normal: Some(tilted.clone()), ..Maps::default() }, 32);
        let dx = render_sphere(&Maps { normal: Some(tilted), normal_dx: true, ..Maps::default() }, 32);
        assert_ne!(gl, dx);
    }
}
