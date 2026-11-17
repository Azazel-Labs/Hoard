//! Face triangulation: ear clipping in the face's plane, with holes bridged into the outer loop. A port of
//! openskp's `triangulator.cpp` and `earcut.cpp`, with an O(n²) ear walk instead of restarting the search after
//! every ear, and each triangle turned to face the same way as the face.

type V3 = [f64; 3];

pub(crate) fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub(crate) fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(crate) fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub(crate) fn normalize(a: V3) -> Option<V3> {
    let len = dot(a, a).sqrt();
    (len > 1e-12 && len.is_finite()).then(|| [a[0] / len, a[1] / len, a[2] / len])
}

/// The normal of a loop by Newell's method, which copes with concave and slightly non-planar loops.
pub(crate) fn newell(points: &[V3]) -> Option<V3> {
    let mut n = [0.0; 3];
    for (i, a) in points.iter().enumerate() {
        let b = points[(i + 1) % points.len()];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    normalize(n)
}

/// A 2D point and its index into the face's concatenated loops.
#[derive(Debug, Clone, Copy)]
struct P {
    x: f64,
    y: f64,
    id: usize,
}

fn cross2(a: P, b: P, c: P) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn signed_area(poly: &[P]) -> f64 {
    let mut s = 0.0;
    for (i, a) in poly.iter().enumerate() {
        let b = poly[(i + 1) % poly.len()];
        s += a.x * b.y - b.x * a.y;
    }
    s / 2.0
}

fn point_in_polygon(x: f64, y: f64, poly: &[P]) -> bool {
    let mut inside = false;
    let mut j = poly.len().wrapping_sub(1);
    for (i, a) in poly.iter().enumerate() {
        let b = poly[j];
        if (a.y > y) != (b.y > y) && x < (b.x - a.x) * (y - a.y) / (b.y - a.y) + a.x {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Triangulates a face. `loops` holds the outer loop then its holes; the returned indices count through them in
/// that order. Each triangle winds counter-clockwise seen from the side `normal` points to.
pub(crate) fn triangulate(loops: &[Vec<V3>], normal: V3) -> Vec<[usize; 3]> {
    let Some(outer) = loops.first() else { return Vec::new() };
    let Some(n) = normalize(normal).or_else(|| newell(outer)) else { return Vec::new() };
    let points: Vec<V3> = loops.iter().flatten().copied().collect();
    let mut tris = if loops.len() == 1 && outer.len() == 3 {
        vec![[0, 1, 2]]
    } else if loops.len() == 1 && outer.len() == 4 {
        split_quad(outer)
    } else {
        // project onto the face plane
        let seed = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let Some(u) = normalize(cross(n, seed)) else { return Vec::new() };
        let v = cross(n, u);
        let origin = outer[0];
        let mut id = 0;
        let flat: Vec<Vec<P>> = loops
            .iter()
            .map(|l| {
                l.iter()
                    .map(|&p| {
                        let d = sub(p, origin);
                        id += 1;
                        P { x: dot(d, u), y: dot(d, v), id: id - 1 }
                    })
                    .collect()
            })
            .collect();
        earcut(flat)
    };
    // slivers along a straight run of edges draw nothing and have no reliable facing, so they go
    let scale = points.iter().map(|&p| dot(sub(p, outer[0]), sub(p, outer[0]))).fold(0.0, f64::max);
    tris.retain_mut(|t| {
        let c = cross(sub(points[t[1]], points[t[0]]), sub(points[t[2]], points[t[0]]));
        let facing = dot(c, n);
        if facing < 0.0 {
            t.swap(1, 2);
        }
        facing.abs() > scale * 1e-12
    });
    tris
}

/// Splits a quad along the diagonal that stays inside it: on a concave quad the other one leaves two halves
/// facing opposite ways.
fn split_quad(q: &[V3]) -> Vec<[usize; 3]> {
    let normal = |a: usize, b: usize, c: usize| cross(sub(q[b], q[a]), sub(q[c], q[a]));
    if dot(normal(0, 1, 2), normal(0, 2, 3)) >= 0.0 { vec![[0, 1, 2], [0, 2, 3]] } else { vec![[1, 2, 3], [1, 3, 0]] }
}

fn earcut(mut loops: Vec<Vec<P>>) -> Vec<[usize; 3]> {
    loops.retain(|l| l.len() >= 3);
    if loops.is_empty() {
        return Vec::new();
    }
    let mut outer = loops.remove(0);
    if signed_area(&outer) < 0.0 {
        outer.reverse();
    }
    for h in &mut loops {
        if signed_area(h) > 0.0 {
            h.reverse();
        }
    }
    if loops.len() > 1 && holes_overlap(&loops) {
        // Bridging overlapping holes makes a self-crossing polygon; clip the outline and cut the holes out after.
        let tris = clip(outer.clone());
        let lookup = |id: usize| outer.iter().find(|p| p.id == id).copied();
        return tris
            .into_iter()
            .filter(|t| {
                let corners: Option<Vec<P>> = t.iter().map(|&id| lookup(id)).collect();
                let Some(corners) = corners else { return false };
                let x = corners.iter().map(|p| p.x).sum::<f64>() / 3.0;
                let y = corners.iter().map(|p| p.y).sum::<f64>() / 3.0;
                !loops.iter().any(|h| point_in_polygon(x, y, h))
            })
            .collect();
    }
    // bridge the holes rightmost first, so later bridges see earlier ones as part of the outline
    loops.sort_by(|a, b| max_x(b).total_cmp(&max_x(a)));
    for (k, hole) in loops.iter().enumerate() {
        outer = bridge(&outer, hole, &loops[k + 1..]);
    }
    clip(outer)
}

fn max_x(poly: &[P]) -> f64 {
    poly.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max)
}

fn holes_overlap(holes: &[Vec<P>]) -> bool {
    holes
        .iter()
        .enumerate()
        .any(|(i, a)| holes.iter().enumerate().any(|(j, b)| i != j && a.iter().any(|p| point_in_polygon(p.x, p.y, b))))
}

fn segments_cross(a: P, b: P, c: P, d: P) -> bool {
    let d1 = cross2(a, b, c);
    let d2 = cross2(a, b, d);
    let d3 = cross2(c, d, a);
    let d4 = cross2(c, d, b);
    ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0)) && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
}

/// Joins a hole to the outline with a two-way cut from the hole's rightmost point to the nearest outline point
/// the cut can reach without crossing an edge.
fn bridge(outer: &[P], hole: &[P], rest: &[Vec<P>]) -> Vec<P> {
    let mut hi = 0;
    for (i, p) in hole.iter().enumerate() {
        if p.x.total_cmp(&hole[hi].x).then(hole[hi].y.total_cmp(&p.y)).is_gt() {
            hi = i;
        }
    }
    let h = hole[hi];
    let edges = || {
        let ring = |poly: &[P]| (0..poly.len()).map(|i| (poly[i], poly[(i + 1) % poly.len()])).collect::<Vec<_>>();
        let mut e = ring(outer);
        e.extend(ring(hole));
        for r in rest {
            e.extend(ring(r));
        }
        e
    };
    let all_edges = edges();
    let dist = |p: &P| (p.x - h.x).powi(2) + (p.y - h.y).powi(2);
    let visible = |o: &P| !all_edges.iter().any(|&(a, b)| segments_cross(h, *o, a, b));
    let pick = |filter: &dyn Fn(&P) -> bool| {
        outer
            .iter()
            .enumerate()
            .filter(|(_, p)| filter(p))
            .min_by(|a, b| dist(a.1).total_cmp(&dist(b.1)))
            .map(|(i, _)| i)
    };
    let oi = pick(&|p| p.x >= h.x && visible(p)).or_else(|| pick(&visible)).or_else(|| pick(&|_| true)).unwrap_or(0);
    let mut merged = Vec::with_capacity(outer.len() + hole.len() + 2);
    merged.extend_from_slice(&outer[..=oi]);
    merged.extend(hole[hi..].iter().chain(&hole[..hi]).copied());
    merged.push(h);
    merged.extend_from_slice(&outer[oi..]);
    merged
}

fn in_triangle(p: P, a: P, b: P, c: P) -> bool {
    const EPS: f64 = -1e-12;
    cross2(a, b, p) >= EPS && cross2(b, c, p) >= EPS && cross2(c, a, p) >= EPS
}

fn is_ear(poly: &[P], at: usize) -> bool {
    let len = poly.len();
    let (a, b, c) = (poly[(at + len - 1) % len], poly[at], poly[(at + 1) % len]);
    if cross2(a, b, c) <= 1e-12 {
        return false;
    }
    !poly.iter().any(|&p| p.id != a.id && p.id != b.id && p.id != c.id && in_triangle(p, a, b, c))
}

/// Ear clipping of a simple counter-clockwise polygon (bridged holes included).
fn clip(mut poly: Vec<P>) -> Vec<[usize; 3]> {
    if signed_area(&poly) < 0.0 {
        poly.reverse();
    }
    let mut out = Vec::new();
    let mut i = 0;
    let mut misses = 0;
    while poly.len() > 3 {
        let n = poly.len();
        i %= n;
        if misses >= n {
            // no ear in a full lap: drop a collinear point, or clip anyway on a self-crossing loop
            if let Some(k) = (0..n).find(|&k| cross2(poly[(k + n - 1) % n], poly[k], poly[(k + 1) % n]).abs() < 1e-10) {
                poly.remove(k);
            } else {
                out.push([poly[(i + n - 1) % n].id, poly[i].id, poly[(i + 1) % n].id]);
                poly.remove(i);
            }
            misses = 0;
            continue;
        }
        if is_ear(&poly, i) {
            out.push([poly[(i + n - 1) % n].id, poly[i].id, poly[(i + 1) % n].id]);
            poly.remove(i);
            // the previous point may have just become an ear
            i = (i + n - 2) % (n - 1);
            misses = 0;
        } else {
            i += 1;
            misses += 1;
        }
    }
    if let [a, b, c] = poly[..]
        && cross2(a, b, c).abs() > 1e-12
    {
        out.push([a.id, b.id, c.id]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn area(points: &[V3], tris: &[[usize; 3]]) -> f64 {
        tris.iter()
            .map(|t| {
                dot(cross(sub(points[t[1]], points[t[0]]), sub(points[t[2]], points[t[0]])), [0.0, 0.0, 1.0]) / 2.0
            })
            .sum()
    }

    fn square(x: f64, y: f64, s: f64) -> Vec<V3> {
        vec![[x, y, 0.0], [x + s, y, 0.0], [x + s, y + s, 0.0], [x, y + s, 0.0]]
    }

    #[test]
    fn triangle_and_quad() {
        let tri = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert_eq!(triangulate(&[tri], [0.0, 0.0, 1.0]), vec![[0, 1, 2]]);
        let q = square(0.0, 0.0, 2.0);
        let t = triangulate(std::slice::from_ref(&q), [0.0, 0.0, 1.0]);
        assert_eq!(t.len(), 2);
        assert!((area(&q, &t) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn winding_follows_the_normal() {
        let mut q = square(0.0, 0.0, 1.0);
        q.reverse();
        let t = triangulate(std::slice::from_ref(&q), [0.0, 0.0, 1.0]);
        assert!((area(&q, &t) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn concave_polygon() {
        // an L shape
        let l =
            vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 2.0, 0.0], [0.0, 2.0, 0.0]];
        let t = triangulate(std::slice::from_ref(&l), [0.0, 0.0, 1.0]);
        assert_eq!(t.len(), 4);
        assert!((area(&l, &t) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn holes_are_cut_out() {
        let loops = vec![square(0.0, 0.0, 10.0), square(2.0, 2.0, 2.0), square(6.0, 6.0, 2.0)];
        let points: Vec<V3> = loops.iter().flatten().copied().collect();
        let t = triangulate(&loops, [0.0, 0.0, 1.0]);
        assert!((area(&points, &t) - 92.0).abs() < 1e-9, "area {}", area(&points, &t));
    }

    #[test]
    fn degenerate_input_does_not_panic() {
        assert_eq!(triangulate(&[], [0.0, 0.0, 1.0]), Vec::<[usize; 3]>::new());
        let line = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0], [4.0, 0.0, 0.0]];
        let _ = triangulate(std::slice::from_ref(&line), [0.0, 0.0, 0.0]);
        assert_eq!(triangulate(&[line], [0.0, 0.0, 1.0]), Vec::<[usize; 3]>::new());
        // a triangle with an extra vertex halfway along one side still covers its area, with no sliver
        let tri = vec![[0.0, 0.0, 0.0], [5.0, 0.0, 0.0], [10.0, 0.0, 0.0], [0.0, 10.0, 0.0]];
        let areas: Vec<f64> = triangulate(std::slice::from_ref(&tri), [0.0, 0.0, 1.0])
            .iter()
            .map(|t| cross(sub(tri[t[1]], tri[t[0]]), sub(tri[t[2]], tri[t[0]]))[2] / 2.0)
            .collect();
        assert!(areas.iter().all(|&a| a > 1.0) && (areas.iter().sum::<f64>() - 50.0).abs() < 1e-9, "{areas:?}");
        let nan = vec![[f64::NAN; 3]; 5];
        let _ = triangulate(&[nan], [0.0, 0.0, 1.0]);
    }
}
