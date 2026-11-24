//! Where the camera goes: framing a model tightly from a direction, and for an animation, one camera for the whole
//! clip (following the figure, or still when it moves across a room), the room it moves across, and its trail.

use glam::{Vec2, Vec3};

use super::{Alpha, Draw, Globals, Vertex};

/// How much of the frame the model fills, edge to edge, along its longer side.
const FILL: f32 = 0.9;

pub(super) fn centroid(d: &Draw) -> Vec3 {
    let sum: Vec3 = d.vertices.iter().map(|v| Vec3::from(v.pos)).sum();
    sum / d.vertices.len().max(1) as f32
}

/// View-projection and key light for a camera looking from `dir`, close enough that the model fills [`FILL`] of
/// the frame (its silhouette from there, not its bounding sphere), and where the camera is.
///
/// The browser takes the same picture when it upgrades a thumbnail (`snapshot` in `modelStage.ts`); keep the two
/// in step.
pub(super) fn camera(draws: &[&Draw], dir: Vec3) -> Option<(Globals, Vec3)> {
    let shot = Shot::of(draws, dir, None)?;
    Some(shot.view(shot.dist))
}

/// The camera's field of view, top to bottom.
const FOV_DEG: f32 = 35.0;

/// How a camera looking from `dir` takes in some draws: where it aims, how far back it has to stand for every vertex
/// to fit inside [`FILL`] of the frame, and how near and far the model reaches along its line of sight.
#[derive(Clone, Copy)]
struct Shot {
    dir: Vec3,
    right: Vec3,
    up: Vec3,
    target: Vec3,
    dist: f32,
    /// The nearest and farthest vertex along `dir` (toward the camera), from `target`.
    near_z: f32,
    far_z: f32,
    radius: f32,
}

impl Shot {
    /// Aimed at `target`, or else at the middle of the silhouette from `dir`.
    fn of(draws: &[&Draw], dir: Vec3, target: Option<Vec3>) -> Option<Shot> {
        let (center, radius) = bounds(draws)?;
        let dir = dir.normalize();
        // Straight-down views need a different up vector.
        let up = if dir.y.abs() > 0.95 { Vec3::NEG_Z } else { Vec3::Y };
        let right = up.cross(dir).normalize_or_zero();
        let up = dir.cross(right);
        let points = || draws.iter().flat_map(|d| &d.vertices).map(|v| Vec3::from(v.pos));
        let target = target.unwrap_or_else(|| {
            // The silhouette's middle: each vertex across and up from the bounding box center.
            let (mut lo, mut hi) = (Vec2::splat(f32::INFINITY), Vec2::splat(f32::NEG_INFINITY));
            for p in points().map(|p| p - center) {
                let q = Vec2::new(p.dot(right), p.dot(up));
                lo = lo.min(q);
                hi = hi.max(q);
            }
            center + right * f32::midpoint(lo.x, hi.x) + up * f32::midpoint(lo.y, hi.y)
        });
        // Back off until every vertex is inside the frame.
        let t = (FOV_DEG.to_radians() / 2.0).tan() * FILL;
        let (mut near_z, mut far_z) = (f32::NEG_INFINITY, f32::INFINITY);
        let mut reach = 0f32;
        for p in points().map(|p| p - target) {
            let (x, y, z) = (p.dot(right), p.dot(up), p.dot(dir));
            near_z = near_z.max(z);
            far_z = far_z.min(z);
            reach = reach.max(z + x.abs().max(y.abs()) / t);
        }
        let dist = reach.max(near_z + radius * 0.02);
        Some(Shot { dir, right, up, target, dist, near_z, far_z, radius })
    }

    /// The same shot, seeing as far back as the farthest of `scenery` behind the model.
    fn behind(mut self, scenery: &[Draw]) -> Shot {
        for v in scenery.iter().flat_map(|d| &d.vertices) {
            self.far_z = self.far_z.min((Vec3::from(v.pos) - self.target).dot(self.dir));
        }
        self
    }

    /// The view from `dist` back (as far as this shot needs, or farther), and where the camera is.
    fn view(&self, dist: f32) -> (Globals, Vec3) {
        let Shot { dir, right, up, target, near_z, far_z, radius, .. } = *self;
        let eye = target + dir * dist;
        let view = glam::camera::rh::view::look_at_mat4(eye, target, up);
        // Key light over the camera's right shoulder, as the browser viewer's rides with its camera.
        let light = (right * 1.2 + up * 1.6 + dir * 2.6).normalize();
        // wgpu clip space is Z in [0, 1], Y up: the "directx" convention.
        let near = (dist - near_z).max(radius * 0.001) * 0.5;
        let proj = glam::camera::rh::proj::directx::perspective(FOV_DEG.to_radians(), 1.0, near, dist - far_z + radius);
        let globals = Globals {
            view_proj: (proj * view).to_cols_array_2d(),
            light_dir: [light.x, light.y, light.z, 0.0],
            eye: [eye.x, eye.y, eye.z, 1.0],
        };
        (globals, eye)
    }
}

/// How much farther back than a still thumbnail's a clip's camera stands, so a little of the room shows around the
/// character: the tiles behind it are what show it moving, so the camera stays close.
const ROOMY: f32 = 1.2;

/// One camera per frame of a clip: the same one for all of them when the character stays where it is (an idle, a
/// swing), so it moves in a picture that holds still; when it goes somewhere (a walk, a run, a fall), one that stays
/// on it at one distance, as big as it would be standing still, with the room going by behind it.
pub(super) fn clip_cameras(frames: &[Vec<Draw>], room: &[Draw], dir: Vec3) -> Option<Vec<(Globals, Vec3)>> {
    let (mids, tall) = middles(frames)?;
    if !goes(&mids, tall) {
        let all: Vec<&Draw> = frames.iter().flatten().collect();
        let whole = Shot::of(&all, dir, None)?;
        return Some(vec![whole.behind(room).view(whole.dist * ROOMY); frames.len()]);
    }
    let each: Vec<Shot> =
        frames.iter().map(|f| Shot::of(&f.iter().collect::<Vec<_>>(), dir, None)).collect::<Option<_>>()?;
    let aims = steady(&each.iter().map(|s| s.target).collect::<Vec<_>>());
    let follow: Vec<Shot> = frames
        .iter()
        .zip(aims)
        .map(|(f, aim)| Shot::of(&f.iter().collect::<Vec<_>>(), dir, Some(aim)))
        .collect::<Option<_>>()?;
    let dist = follow.iter().map(|s| s.dist).fold(0f32, f32::max);
    Some(follow.iter().map(|s| s.behind(room).view(dist * ROOMY)).collect())
}

/// A following camera's aims: each middle averaged with its neighbors on both sides (fewer toward the ends, the same
/// number each side), so the camera keeps pace with a run, a fall or a lap of a circle, but a flailing arm doesn't
/// shake it. A steady run or fall is followed exactly.
fn steady(middles: &[Vec3]) -> Vec<Vec3> {
    let n = middles.len();
    let reach = (n / 8).max(1);
    (0..n)
        .map(|i| {
            let h = reach.min(i).min(n - 1 - i);
            let near = &middles[i - h..=i + h];
            #[expect(clippy::cast_precision_loss, reason = "a few frames")]
            let count = near.len() as f32;
            near.iter().sum::<Vec3>() / count
        })
        .collect()
}

/// The middle of each of a clip's poses (of its bounding box) and the character's height, its tallest pose's.
fn middles(frames: &[Vec<Draw>]) -> Option<(Vec<Vec3>, f32)> {
    let mut tall = 0f32;
    let mut mids = Vec::with_capacity(frames.len());
    for frame in frames {
        let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
        for p in frame.iter().flat_map(|d| &d.vertices).map(|v| Vec3::from(v.pos)) {
            lo = lo.min(p);
            hi = hi.max(p);
        }
        if !lo.is_finite() {
            return None;
        }
        tall = tall.max(hi.y - lo.y);
        mids.push((lo + hi) / 2.0);
    }
    Some((mids, tall))
}

/// Whether a clip's character goes somewhere: its middle travels [`GOES`] of its height.
fn goes(mids: &[Vec3], tall: f32) -> bool {
    let (lo, hi) = mids
        .iter()
        .fold((Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)), |(l, h), p| (l.min(*p), h.max(*p)));
    mids.len() >= 2 && (hi - lo).length() >= tall.max(1e-6) * GOES
}

/// The room an animation plays in: a tiled floor under the lowest the character gets, and the two walls behind it
/// from `dir`, reaching past everywhere it goes, with tiles a quarter of its height. A camera that follows a fall or a
/// run shows it going by them. Its draws wear image `tiles` (see [`tiles`]).
pub(super) fn room(frames: &[Vec<Draw>], dir: Vec3, tiles: usize) -> Option<Vec<Draw>> {
    let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    let mut tall = 0f32;
    for frame in frames {
        let (mut flo, mut fhi) = (f32::INFINITY, f32::NEG_INFINITY);
        for p in frame.iter().flat_map(|d| &d.vertices).map(|v| Vec3::from(v.pos)) {
            lo = lo.min(p);
            hi = hi.max(p);
            flo = flo.min(p.y);
            fhi = fhi.max(p.y);
        }
        tall = tall.max(fhi - flo);
    }
    if !lo.is_finite() || !hi.is_finite() {
        return None;
    }
    let size = (hi - lo).max_element().max(1e-4);
    let tall = if tall > size * 1e-3 { tall } else { size };
    let tile = tall / 4.0;
    // The walls stand a little behind it, on the sides away from the camera; the floor runs on toward the camera and
    // the walls on up, past the frame's edges.
    let gap = tall * 0.4;
    let (sx, sz) = (if dir.x >= 0.0 { -1.0 } else { 1.0 }, if dir.z >= 0.0 { -1.0 } else { 1.0 });
    let wall_x = if sx < 0.0 { lo.x - gap } else { hi.x + gap };
    let wall_z = if sz < 0.0 { lo.z - gap } else { hi.z + gap };
    let far_x = if sx < 0.0 { hi.x + tall * 4.0 } else { lo.x - tall * 4.0 };
    let far_z = if sz < 0.0 { hi.z + tall * 4.0 } else { lo.z - tall * 4.0 };
    let (floor, top) = (lo.y, hi.y + tall * 2.0);
    let uv = |a: f32, b: f32| [a / (tile * 2.0), b / (tile * 2.0)];
    let quad = |corners: [Vec3; 4], normal: Vec3, uvs: [[f32; 2]; 4], color: [f32; 4]| Draw {
        vertices: corners
            .iter()
            .zip(uvs)
            .map(|(p, uv)| Vertex { pos: p.to_array(), normal: normal.to_array(), uv })
            .collect(),
        indices: vec![0, 1, 2, 0, 2, 3],
        color,
        texture: Some(tiles),
        alpha: Alpha::Opaque,
        transmission: 0.0,
        metallic: 0.0,
        roughness: 0.9,
        metallic_roughness: None,
    };
    let floor_quad = {
        let c = [
            Vec3::new(wall_x, floor, wall_z),
            Vec3::new(far_x, floor, wall_z),
            Vec3::new(far_x, floor, far_z),
            Vec3::new(wall_x, floor, far_z),
        ];
        quad(c, Vec3::Y, c.map(|p| uv(p.x, p.z)), [1.0; 4])
    };
    let wall_across_x = {
        // Facing along z, at `wall_z`.
        let c = [
            Vec3::new(wall_x, floor, wall_z),
            Vec3::new(far_x, floor, wall_z),
            Vec3::new(far_x, top, wall_z),
            Vec3::new(wall_x, top, wall_z),
        ];
        quad(c, Vec3::new(0.0, 0.0, -sz), c.map(|p| uv(p.x, p.y)), [0.93, 0.93, 0.95, 1.0])
    };
    let wall_across_z = {
        // Facing along x, at `wall_x`.
        let c = [
            Vec3::new(wall_x, floor, wall_z),
            Vec3::new(wall_x, floor, far_z),
            Vec3::new(wall_x, top, far_z),
            Vec3::new(wall_x, top, wall_z),
        ];
        quad(c, Vec3::new(-sx, 0.0, 0.0), c.map(|p| uv(p.z, p.y)), [0.88, 0.88, 0.91, 1.0])
    };
    Some(vec![floor_quad, wall_across_x, wall_across_z])
}

/// The room's tiles: two by two of them, light and a shade darker with a fine line between, repeated across it.
pub(super) fn tiles() -> image::RgbaImage {
    const SIDE: u32 = 32;
    image::RgbaImage::from_fn(SIDE, SIDE, |x, y| {
        let (cx, cy) = (x % (SIDE / 2), y % (SIDE / 2));
        let grout = cx == 0 || cy == 0;
        let dark = (x < SIDE / 2) != (y < SIDE / 2);
        let v = if grout {
            164
        } else if dark {
            184
        } else {
            196
        };
        image::Rgba([v, v, v + 6, 255])
    })
}

/// A clip goes somewhere when its middle travels this much of the character's height: then the camera follows it and
/// its thumbnail carries a trail of where.
const GOES: f32 = 0.3;

/// Where a clip's character goes, frame by frame, seen from the side it goes most across: from the side along the way
/// it heads (a fall, a run, a jump forward), or from above when it wanders around more than it rises and falls (a turn,
/// a circle). Each point is the middle of its pose, in the plane of that view, y up. None when it stays put.
pub(super) fn trail(frames: &[Vec<Draw>]) -> Option<Vec<Vec2>> {
    let (middles, tall) = middles(frames)?;
    if !goes(&middles, tall) {
        return None;
    }
    let n = middles.len() as f32;
    let mean = middles.iter().sum::<Vec3>() / n;
    // The way it heads across the floor: the long axis of where its middle goes, seen from above.
    let (mut xx, mut xz, mut zz, mut yy) = (0f32, 0f32, 0f32, 0f32);
    for p in middles.iter().map(|p| *p - mean) {
        xx += p.x * p.x;
        xz += p.x * p.z;
        zz += p.z * p.z;
        yy += p.y * p.y;
    }
    let angle = 0.5 * (2.0 * xz).atan2(xx - zz);
    let mut along = Vec3::new(angle.cos(), 0.0, angle.sin());
    let across = Vec3::new(-along.z, 0.0, along.x);
    let spread = |axis: Vec3| middles.iter().map(|p| (*p - mean).dot(axis).powi(2)).sum::<f32>();
    // Left to right, the way it goes.
    if (middles[middles.len() - 1] - middles[0]).dot(along) < 0.0 {
        along = -along;
    }
    let up = if spread(across) > yy { across } else { Vec3::Y };
    Some(middles.iter().map(|p| Vec2::new((*p - mean).dot(along), (*p - mean).dot(up))).collect())
}

/// Draw a clip's trail small along the top of its frame `at`: the whole way faint, the stretch it's just come along
/// brighter, and a dot where it is.
pub(super) fn draw_trail(img: &mut image::RgbaImage, path: &[Vec2], at: usize) {
    #[expect(clippy::cast_precision_loss, reason = "a thumbnail's side")]
    let side = img.width().min(img.height()) as f32;
    let (lo, hi) = path
        .iter()
        .fold((Vec2::splat(f32::INFINITY), Vec2::splat(f32::NEG_INFINITY)), |(l, h), p| (l.min(*p), h.max(*p)));
    let (box_w, box_h, top) = (side * 0.62, side * 0.11, side * 0.045);
    let scale = (box_w / (hi.x - lo.x).max(1e-6)).min(box_h / (hi.y - lo.y).max(1e-6));
    let mid = (lo + hi) / 2.0;
    let screen: Vec<Vec2> = path
        .iter()
        .map(|p| Vec2::new(side / 2.0 + (p.x - mid.x) * scale, top + box_h / 2.0 - (p.y - mid.y) * scale))
        .collect();
    let line = (side / 160.0).max(1.0);
    // How far back the bright stretch reaches, in frames.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        reason = "a few frames"
    )]
    let tail = ((path.len() as f32 * 0.4).round() as usize).max(2);
    let head = screen[at];
    let pad = line * 4.0;
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to the image")]
    let (x0, x1, y0, y1) = (
        (side / 2.0 - box_w / 2.0 - pad).max(0.0) as u32,
        ((side / 2.0 + box_w / 2.0 + pad) as u32).min(img.width() - 1),
        (top - pad).max(0.0) as u32,
        ((top + box_h + pad) as u32).min(img.height() - 1),
    );
    let blend = |px: &mut image::Rgba<u8>, rgb: [f32; 3], a: f32| {
        for (c, v) in px.0.iter_mut().zip(rgb) {
            *c = (f32::from(*c) * (1.0 - a) + v * a).round().clamp(0.0, 255.0) as u8;
        }
    };
    for y in y0..=y1 {
        for x in x0..=x1 {
            #[expect(clippy::cast_precision_loss, reason = "a pixel of a thumbnail")]
            let q = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            // Nearest the whole way, and nearest the bright stretch, with how fresh it is there (1 at the dot).
            let (mut whole, mut bright, mut fresh) = (f32::INFINITY, f32::INFINITY, 0f32);
            for (i, seg) in screen.windows(2).enumerate() {
                let (d, t) = to_segment(q, seg[0], seg[1]);
                whole = whole.min(d);
                #[expect(clippy::cast_precision_loss, reason = "a few frames")]
                let age = at as f32 - (i as f32 + t);
                #[expect(clippy::cast_precision_loss, reason = "a few frames")]
                if (0.0..tail as f32).contains(&age) && d < bright {
                    bright = d;
                    fresh = 1.0 - age / tail as f32;
                }
            }
            let cover = |d: f32, half: f32| (half + 0.5 - d).clamp(0.0, 1.0);
            let px = img.get_pixel_mut(x, y);
            blend(px, [255.0; 3], cover(whole, line * 1.6) * 0.55);
            blend(px, [46.0, 52.0, 64.0], cover(whole, line * 0.5) * 0.45);
            blend(px, [32.0, 128.0, 112.0], cover(bright, line) * fresh.powf(0.8));
            let dot = q.distance(head);
            blend(px, [255.0; 3], cover(dot, line * 3.4));
            blend(px, [32.0, 128.0, 112.0], cover(dot, line * 2.4));
        }
    }
}

/// How far `q` is from the segment from `a` to `b`, and how far along it the nearest point is, 0 to 1.
fn to_segment(q: Vec2, a: Vec2, b: Vec2) -> (f32, f32) {
    let ab = b - a;
    let t = if ab.length_squared() > 0.0 { ((q - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
    (q.distance(a + ab * t), t)
}

fn bounds(draws: &[&Draw]) -> Option<(Vec3, f32)> {
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for v in draws.iter().flat_map(|d| &d.vertices) {
        let p = Vec3::from(v.pos);
        min = min.min(p);
        max = max.max(p);
    }
    if !min.is_finite() || !max.is_finite() {
        return None;
    }
    let center = (min + max) / 2.0;
    Some((center, ((max - min).length() / 2.0).max(1e-4)))
}
