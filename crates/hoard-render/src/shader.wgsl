struct Globals { view_proj: mat4x4<f32>, light_dir: vec4<f32>, eye: vec4<f32> };
// flags: x = has a texture, y = alpha cut-off (0 for none), z = blended, w = which part of a glass surface this
// draw is (0 all of it, 1 what lets no light through, 2 what does).
// glass: x = transmission (0 to 1), y = roughness, z = metalness (both times the metallic-roughness texture).
struct Material { color: vec4<f32>, flags: vec4<f32>, glass: vec4<f32> };

@group(0) @binding(0) var<uniform> g: Globals;
@group(1) @binding(0) var<uniform> m: Material;
@group(1) @binding(1) var tex: texture_2d<f32>;
@group(1) @binding(2) var samp: sampler;
@group(1) @binding(3) var mr_tex: texture_2d<f32>;

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) n: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world: vec3<f32>,
};

@vertex
fn vs(@location(0) p: vec3<f32>, @location(1) n: vec3<f32>, @location(2) uv: vec2<f32>) -> VOut {
    var o: VOut;
    o.pos = g.view_proj * vec4<f32>(p, 1.0);
    o.n = n;
    o.uv = uv;
    o.world = p;
    return o;
}

// Khronos PBR Neutral, the browser viewer's tone mapping: colors stay as painted, highlights roll off.
fn neutral(v: vec3<f32>) -> vec3<f32> {
    let start = 0.76;
    let x = min(v.r, min(v.g, v.b));
    let offset = select(0.04, x - 6.25 * x * x, x < 0.08);
    var c = v - offset;
    let peak = max(c.r, max(c.g, c.b));
    if (peak < start) { return c; }
    let d = 1.0 - start;
    let new_peak = 1.0 - d * d / (peak + d - start);
    c = c * (new_peak / peak);
    let k = 1.0 - 1.0 / (0.15 * (peak - new_peak) + 1.0);
    return mix(c, vec3<f32>(new_peak), k);
}

@fragment
fn fs(i: VOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    let sampled = textureSample(tex, samp, i.uv);
    let mr = textureSample(mr_tex, samp, i.uv);
    let base = m.color * select(vec4<f32>(1.0), sampled, m.flags.x > 0.5);
    if (m.flags.y > 0.0 && base.a < m.flags.y) { discard; }
    // Metal lets no light through, whatever the transmission says (a lantern's brass and glass can share one
    // material and one texture).
    let t = m.glass.x * (1.0 - clamp(m.glass.z * mr.b, 0.0, 1.0));
    let glassy = t > 0.05;
    if ((m.flags.w == 1.0 && glassy) || (m.flags.w == 2.0 && !glassy)) { discard; }
    // Both sides of a face are drawn; the back lights as if it faced the camera.
    let n = normalize(select(-i.n, i.n, front));
    let key = max(dot(n, g.light_dir.xyz), 0.0);
    // A soft room, bright overhead and darker toward the floor so no side goes black (the browser's environment
    // light), plus the key light.
    let room = mix(0.5, 0.95, 0.5 + 0.5 * n.y);
    var rgb = neutral(base.rgb * (room + 0.6 * key));
    var a = select(1.0, base.a, m.flags.z > 0.5);
    // Glass: light passes through, so the surface shows what's behind it, as much as its color lets through (dark
    // glass hides more, as the browser's viewer tints what's behind by it) and less the rougher (frosted) it is.
    // What gives it away is the edge, where it reflects (Fresnel), and the key light's highlight. No refraction: at
    // thumbnail size, this reads as glass.
    if (glassy) {
        let rough = clamp(m.glass.y * mr.g, 0.0, 1.0);
        let v = normalize(g.eye.xyz - i.world);
        let fresnel = 0.04 + 0.96 * pow(1.0 - abs(dot(n, v)), 5.0);
        let h = normalize(v + g.light_dir.xyz);
        let shine = pow(max(dot(n, h), 0.0), mix(256.0, 8.0, rough)) * (1.0 - 0.8 * rough);
        let through = max(base.r, max(base.g, base.b)) * (1.0 - smoothstep(0.3, 0.9, rough));
        let solid = clamp(1.0 - 0.85 * through + fresnel + shine, 0.0, 1.0);
        a = mix(a, solid * base.a, t);
        rgb = mix(rgb, vec3<f32>(1.0), clamp(shine * t, 0.0, 1.0));
    }
    // Premultiplied, so blended surfaces and antialiased edges composite right; undone on readback.
    return vec4<f32>(rgb * a, a);
}
