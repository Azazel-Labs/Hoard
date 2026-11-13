//! Headless wgpu renderer for model thumbnails: glTF/GLB in, RGBA image out. (Other formats reach it as a GLB
//! the server makes.)
//!
//! Deliberately small: bakes every mesh primitive of the default scene into world space, frames the model tightly
//! from a 3/4 angle, and shades with base color (factor × texture, with alpha cut-outs and blending; glass, glTF's
//! transmission, as a mostly clear surface that shows edges and a highlight) under a soft sky/ground light and a key light over the camera's shoulder, tone mapped like the browser viewer (Khronos PBR
//! Neutral), so a thumbnail looks like the model does when opened. No skinning or morph targets (meshes render in
//! bind pose).
//!
//! On machines without a GPU it runs on Mesa's lavapipe (software Vulkan) or any other adapter wgpu finds,
//! including the fallback adapter.
//!
//! [`sphere`] renders material swatches on the CPU, with no GPU at all.

use std::path::Path;

use anyhow::{Context, Result, bail};
use glam::{Mat3, Mat4, Vec3};
use wgpu::util::DeviceExt;

const RENDER_SIZE: u32 = 512;
const SAMPLES: u32 = 4;
const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// How much of the frame the model fills, edge to edge, along its longer side.
const FILL: f32 = 0.9;

const SHADER: &str = "
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

// Khronos PBR Neutral, the browser viewer's tone mapping: colours stay as painted, highlights roll off.
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
    // Glass: light passes through, so the surface shows what's behind it, as much as its colour lets through (dark
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
";

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    view_proj: [[f32; 4]; 4],
    light_dir: [f32; 4],
    eye: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialUniform {
    color: [f32; 4],
    flags: [f32; 4],
    glass: [f32; 4],
}

pub mod sphere;
/// How a surface's alpha counts, as glTF's `alphaMode` says.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Alpha {
    Opaque,
    /// Cut out below this alpha (leaves, fences).
    Mask(f32),
    /// See-through (glass, water): drawn after everything solid, back to front.
    Blend,
}

/// One draw call's worth of baked geometry.
#[derive(Debug)]
struct Draw {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    color: [f32; 4],
    texture: Option<usize>,
    alpha: Alpha,
    /// How much light passes through (glTF's `KHR_materials_transmission`): glass, water, clear plastic.
    transmission: f32,
    metallic: f32,
    roughness: f32,
    /// The metallic-roughness texture (roughness in green, metalness in blue), as an image index.
    metallic_roughness: Option<usize>,
}

/// GPU buffers for one draw.
struct Gpu {
    vb: wgpu::Buffer,
    ib: wgpu::Buffer,
    count: u32,
    bg: wgpu::BindGroup,
    /// For a glass surface that isn't all glass: its see-through part, drawn with the blended surfaces (`bg` draws
    /// the rest with the solid ones).
    bg_clear: Option<wgpu::BindGroup>,
}

/// The default thumbnail camera: 3/4 view from the front-right, slightly above.
const THREE_QUARTER: Vec3 = Vec3::new(0.75, 0.55, 1.0);
/// Directions for [`Renderer::render_views_file`], in reading order.
const VIEWS: [Vec3; 4] =
    [THREE_QUARTER, Vec3::new(-0.75, 0.45, -1.0), Vec3::new(1.0, 0.12, 0.0), Vec3::new(0.0, 1.0, 0.001)];

/// Reusable GPU state. Create once, render many; `render_*` calls are serialized by the caller.
#[derive(Debug)]
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    /// The same, blending over what's drawn and leaving the depth buffer alone, for see-through surfaces.
    blend_pipeline: wgpu::RenderPipeline,
    globals_layout: wgpu::BindGroupLayout,
    material_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    white: wgpu::TextureView,
    adapter_name: String,
    hardware: bool,
    backend: String,
}

impl Renderer {
    /// Find any adapter (hardware, lavapipe, or wgpu's fallback) and build the pipeline.
    pub fn new() -> Result<Self> {
        pollster::block_on(Self::new_async())
    }

    async fn new_async() -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12 | wgpu::Backends::GL,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });
        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
        {
            Ok(a) => a,
            Err(_) => instance
                .request_adapter(&wgpu::RequestAdapterOptions { force_fallback_adapter: true, ..Default::default() })
                .await
                .context("no GPU adapter found (install a GPU driver or Mesa's lavapipe for software rendering)")?,
        };
        let info = adapter.get_info();
        let adapter_name = info.name.clone();
        // lavapipe/llvmpipe and wgpu's fallback report `Cpu`; anything else is a real (or passed-through) GPU.
        let hardware = matches!(
            info.device_type,
            wgpu::DeviceType::DiscreteGpu | wgpu::DeviceType::IntegratedGpu | wgpu::DeviceType::VirtualGpu
        );
        let backend = info.backend.to_string();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("hoard-render"),
                required_limits: wgpu::Limits::downlevel_defaults(),
                ..Default::default()
            })
            .await?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hoard-render shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let uniform_entry = |binding, visibility| wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let globals_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("globals"),
            entries: &[uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
        });
        let material_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("material"),
            entries: &[
                uniform_entry(0, wgpu::ShaderStages::FRAGMENT),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hoard-render layout"),
            bind_group_layouts: &[Some(&globals_layout), Some(&material_layout)],
            immediate_size: 0,
        });
        let make_pipeline = |blend: bool| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(if blend { "hoard-render blend pipeline" } else { "hoard-render pipeline" }),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[Some(wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
                    })],
                },
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: Some(!blend),
                    depth_compare: Some(wgpu::CompareFunction::Less),
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState { count: SAMPLES, ..Default::default() },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: COLOR_FORMAT,
                        blend: blend.then_some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = make_pipeline(false);
        let blend_pipeline = make_pipeline(true);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let white = upload_rgba(&device, &queue, 1, 1, &[255, 255, 255, 255], true);
        Ok(Self {
            device,
            queue,
            pipeline,
            blend_pipeline,
            globals_layout,
            material_layout,
            sampler,
            white,
            adapter_name,
            hardware,
            backend,
        })
    }

    /// Name of the adapter in use, e.g. "llvmpipe (LLVM 19.1.7, 256 bits)" for lavapipe.
    pub fn adapter_name(&self) -> &str {
        &self.adapter_name
    }

    /// Whether the adapter is a GPU (discrete, integrated or virtual) rather than a software rasterizer.
    pub fn is_hardware(&self) -> bool {
        self.hardware
    }

    /// The graphics API in use, e.g. "vulkan".
    pub fn backend(&self) -> &str {
        &self.backend
    }

    /// Render a .gltf/.glb file; external buffers and images resolve relative to its folder.
    pub fn render_file(&self, path: &Path, size: u32) -> Result<image::RgbaImage> {
        let (doc, buffers, images) = gltf::import(path).with_context(|| format!("load {}", path.display()))?;
        self.render(&bake(&doc, &buffers), &gltf_images(&images), size, THREE_QUARTER)
    }

    /// Render a model from four directions (front 3/4, back 3/4, side, top) into a 2x2 sheet of `cell`-sized
    /// tiles. One image a vision model can read in place of four, in the spirit of `Cap3D`'s multi-view captions.
    pub fn render_views_file(&self, path: &Path, cell: u32) -> Result<image::RgbaImage> {
        let (doc, buffers, images) = gltf::import(path).with_context(|| format!("load {}", path.display()))?;
        let (draws, images) = (bake(&doc, &buffers), gltf_images(&images));
        let mut sheet = image::RgbaImage::new(cell * 2, cell * 2);
        for (i, dir) in VIEWS.iter().enumerate() {
            let tile = self.render(&draws, &images, cell, *dir)?;
            let (x, y) = ((i as u32 % 2) * cell, (i as u32 / 2) * cell);
            image::imageops::overlay(&mut sheet, &tile, i64::from(x), i64::from(y));
        }
        Ok(sheet)
    }

    /// Render a .gltf/.glb file wearing `color` as its colour map, the way the portal shows a model with a material
    /// beside it. `None` when the model has colour maps of its own: those are what it shows.
    pub fn render_file_dressed(
        &self,
        path: &Path,
        size: u32,
        color: image::RgbaImage,
    ) -> Result<Option<image::RgbaImage>> {
        let (doc, buffers, images) = gltf::import(path).with_context(|| format!("load {}", path.display()))?;
        let mut draws = bake(&doc, &buffers);
        if draws.iter().any(|d| d.texture.is_some()) {
            return Ok(None);
        }
        let mut images = gltf_images(&images);
        images.push(Some(color));
        for d in &mut draws {
            d.texture = Some(images.len() - 1);
            d.color = [1.0, 1.0, 1.0, d.color[3]];
        }
        self.render(&draws, &images, size, THREE_QUARTER).map(Some)
    }

    /// Render a self-contained GLB (or glTF with embedded data URIs) from memory.
    pub fn render_slice(&self, bytes: &[u8], size: u32) -> Result<image::RgbaImage> {
        let (doc, buffers, images) = gltf::import_slice(bytes).context("parse glTF")?;
        self.render(&bake(&doc, &buffers), &gltf_images(&images), size, THREE_QUARTER)
    }

    #[expect(clippy::too_many_lines, reason = "one linear setup of the frame, its passes and the readback")]
    fn render(
        &self,
        draws: &[Draw],
        images: &[Option<image::RgbaImage>],
        size: u32,
        dir: Vec3,
    ) -> Result<image::RgbaImage> {
        if draws.is_empty() {
            bail!("model has no triangle geometry");
        }
        let (globals, eye) = camera(draws, dir).context("model has no vertices")?;

        let d = &self.device;
        let globals_buf = d.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("globals"),
            contents: bytemuck::bytes_of(&globals),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let globals_bg = d.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("globals"),
            layout: &self.globals_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: globals_buf.as_entire_binding() }],
        });

        // Upload only the images actually used: as base color, and the metallic-roughness maps of glass (linear).
        let mut textures: Vec<Option<wgpu::TextureView>> = (0..images.len()).map(|_| None).collect();
        let mut data_textures: Vec<Option<wgpu::TextureView>> = (0..images.len()).map(|_| None).collect();
        let uses = draws.iter().flat_map(|d| {
            [d.texture.map(|i| (i, true)), d.metallic_roughness.filter(|_| d.transmission > 0.0).map(|i| (i, false))]
        });
        for (i, srgb) in uses.flatten() {
            let slot = if srgb { &mut textures } else { &mut data_textures };
            if slot.get(i).is_some_and(Option::is_none)
                && let Some(Some(img)) = images.get(i)
            {
                // No mipmaps here, and the GPU may take no more than 2048 px: a big texture is scaled down first, which
                // also keeps it from shimmering into noise at thumbnail size.
                let small;
                let img = if img.width().max(img.height()) > MAX_TEXTURE {
                    let k = f64::from(MAX_TEXTURE) / f64::from(img.width().max(img.height()));
                    let (w, h) = (scaled(img.width(), k), scaled(img.height(), k));
                    small = image::imageops::resize(img, w, h, image::imageops::FilterType::Triangle);
                    &small
                } else {
                    img
                };
                slot[i] = Some(upload_rgba(d, &self.queue, img.width(), img.height(), img.as_raw(), srgb));
            }
        }

        let gpu: Vec<Gpu> = draws
            .iter()
            .map(|draw| {
                let tex = draw.texture.and_then(|i| textures[i].as_ref());
                let mr = draw.metallic_roughness.and_then(|i| data_textures[i].as_ref());
                let (cutoff, blend) = match draw.alpha {
                    Alpha::Opaque => (0.0, 0.0),
                    Alpha::Mask(c) => (c.max(0.001), 0.0),
                    // Nearly clear texels of a blended surface draw nothing (and so hide nothing behind them).
                    Alpha::Blend => (0.02, 1.0),
                };
                let split = draw.transmission > 0.0 && draw.alpha != Alpha::Blend;
                let bind = |cutoff: f32, blend: f32, part: f32| {
                    let mat = MaterialUniform {
                        color: draw.color,
                        flags: [f32::from(u8::from(tex.is_some())), cutoff, blend, part],
                        glass: [draw.transmission, draw.roughness, draw.metallic, 0.0],
                    };
                    let mat_buf = d.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("material"),
                        contents: bytemuck::bytes_of(&mat),
                        usage: wgpu::BufferUsages::UNIFORM,
                    });
                    d.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("material"),
                        layout: &self.material_layout,
                        entries: &[
                            wgpu::BindGroupEntry { binding: 0, resource: mat_buf.as_entire_binding() },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(tex.unwrap_or(&self.white)),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(&self.sampler),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::TextureView(mr.unwrap_or(&self.white)),
                            },
                        ],
                    })
                };
                Gpu {
                    vb: d.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("vertices"),
                        contents: bytemuck::cast_slice(&draw.vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }),
                    ib: d.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("indices"),
                        contents: bytemuck::cast_slice(&draw.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                    count: draw.indices.len() as u32,
                    bg: bind(cutoff, blend, if split { 1.0 } else { 0.0 }),
                    // Glass is see-through whatever its alpha mode says.
                    bg_clear: split.then(|| bind(0.02, 1.0, 2.0)),
                }
            })
            .collect();

        let extent = wgpu::Extent3d { width: RENDER_SIZE, height: RENDER_SIZE, depth_or_array_layers: 1 };
        let target = |format, samples, usage| {
            d.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: extent,
                mip_level_count: 1,
                sample_count: samples,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                view_formats: &[],
            })
        };
        let msaa = target(COLOR_FORMAT, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT);
        let resolve = target(COLOR_FORMAT, 1, wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC);
        let depth = target(DEPTH_FORMAT, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT);
        let (msaa_v, resolve_v, depth_v) = (
            msaa.create_view(&wgpu::TextureViewDescriptor::default()),
            resolve.create_view(&wgpu::TextureViewDescriptor::default()),
            depth.create_view(&wgpu::TextureViewDescriptor::default()),
        );

        let bytes_per_row = RENDER_SIZE * 4; // 2048: already a multiple of 256
        let readback = d.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"),
            size: u64::from(bytes_per_row * RENDER_SIZE),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut enc = d.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("thumbnail") });
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("thumbnail"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa_v,
                    depth_slice: None,
                    resolve_target: Some(&resolve_v),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_v,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Discard }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_bind_group(0, &globals_bg, &[]);
            // Solid surfaces first, then see-through ones, farthest first.
            let solid: Vec<usize> = (0..draws.len()).filter(|&i| draws[i].alpha != Alpha::Blend).collect();
            let mut clear: Vec<usize> =
                (0..draws.len()).filter(|&i| draws[i].alpha == Alpha::Blend || gpu[i].bg_clear.is_some()).collect();
            let nearness = |i: usize| -(centroid(&draws[i]) - eye).length_squared();
            clear.sort_by(|&a, &b| nearness(a).total_cmp(&nearness(b)));
            for (pipeline, list, clear) in [(&self.pipeline, &solid, false), (&self.blend_pipeline, &clear, true)] {
                pass.set_pipeline(pipeline);
                for &i in list {
                    let g = &gpu[i];
                    let bg = if clear { g.bg_clear.as_ref().unwrap_or(&g.bg) } else { &g.bg };
                    pass.set_bind_group(1, bg, &[]);
                    pass.set_vertex_buffer(0, g.vb.slice(..));
                    pass.set_index_buffer(g.ib.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..g.count, 0, 0..1);
                }
            }
        }
        enc.copy_texture_to_buffer(
            resolve.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(RENDER_SIZE),
                },
            },
            extent,
        );
        self.queue.submit([enc.finish()]);

        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        d.poll(wgpu::PollType::wait_indefinitely())?;
        let pixels = slice.get_mapped_range()?.to_vec();
        readback.unmap();

        let img = image::RgbaImage::from_raw(RENDER_SIZE, RENDER_SIZE, pixels).context("readback size mismatch")?;
        // Scale down while still premultiplied (so clear pixels lend no colour to the edges), then undo it.
        let mut img = image::imageops::resize(&img, size, size, image::imageops::FilterType::Triangle);
        for p in img.pixels_mut() {
            let a = u32::from(p[3]);
            if a > 0 && a < 255 {
                for c in &mut p.0[..3] {
                    *c = u8::try_from((u32::from(*c) * 255 + a / 2) / a).unwrap_or(255);
                }
            }
        }
        Ok(img)
    }
}

fn centroid(d: &Draw) -> Vec3 {
    let sum: Vec3 = d.vertices.iter().map(|v| Vec3::from(v.pos)).sum();
    sum / d.vertices.len().max(1) as f32
}

/// View-projection and key light for a camera looking from `dir`, close enough that the model fills [`FILL`] of
/// the frame (its silhouette from there, not its bounding sphere), and where the camera is.
///
/// The browser takes the same picture when it upgrades a thumbnail (`snapshot` in `modelStage.ts`); keep the two
/// in step.
fn camera(draws: &[Draw], dir: Vec3) -> Option<(Globals, Vec3)> {
    let (center, radius) = bounds(draws)?;
    let dir = dir.normalize();
    let fov = 35f32.to_radians();
    // Straight-down views need a different up vector.
    let up = if dir.y.abs() > 0.95 { Vec3::NEG_Z } else { Vec3::Y };
    let right = up.cross(dir).normalize_or_zero();
    let up = dir.cross(right);
    // Each vertex in the camera's axes, about the bounding box centre: across, up, and toward the camera.
    let local = || draws.iter().flat_map(|d| &d.vertices).map(|v| Vec3::from(v.pos) - center);
    let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    for p in local() {
        let q = Vec3::new(p.dot(right), p.dot(up), p.dot(dir));
        lo = lo.min(q);
        hi = hi.max(q);
    }
    // Aim at the middle of the silhouette, then back off until every vertex is inside the frame.
    let (mx, my) = (f32::midpoint(lo.x, hi.x), f32::midpoint(lo.y, hi.y));
    let target = center + right * mx + up * my;
    let t = (fov / 2.0).tan() * FILL;
    let mut dist = hi.z + radius * 0.02;
    for p in local() {
        let (x, y, z) = (p.dot(right) - mx, p.dot(up) - my, p.dot(dir));
        dist = dist.max(z + x.abs().max(y.abs()) / t);
    }
    let eye = target + dir * dist;
    let view = glam::camera::rh::view::look_at_mat4(eye, target, up);
    // Key light over the camera's right shoulder, as the browser viewer's rides with its camera.
    let light = (right * 1.2 + up * 1.6 + dir * 2.6).normalize();
    // wgpu clip space is Z in [0, 1], Y up: the "directx" convention.
    let near = (dist - hi.z).max(radius * 0.001) * 0.5;
    let proj = glam::camera::rh::proj::directx::perspective(fov, 1.0, near, dist - lo.z + radius);
    let globals = Globals {
        view_proj: (proj * view).to_cols_array_2d(),
        light_dir: [light.x, light.y, light.z, 0.0],
        eye: [eye.x, eye.y, eye.z, 1.0],
    };
    Some((globals, eye))
}

/// The largest texture side uploaded for a render.
const MAX_TEXTURE: u32 = 1024;

#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a scaled-down side, between 1 and 1024")]
fn scaled(side: u32, k: f64) -> u32 {
    ((f64::from(side) * k).round() as u32).max(1)
}

/// A texture from 8-bit RGBA: colour (`srgb`) or data such as a metallic-roughness map.
fn upload_rgba(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    w: u32,
    h: u32,
    rgba: &[u8],
    srgb: bool,
) -> wgpu::TextureView {
    let tex = device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: Some(if srgb { "color" } else { "data" }),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: if srgb { wgpu::TextureFormat::Rgba8UnormSrgb } else { wgpu::TextureFormat::Rgba8Unorm },
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        rgba,
    );
    tex.create_view(&wgpu::TextureViewDescriptor::default())
}

fn gltf_images(images: &[gltf::image::Data]) -> Vec<Option<image::RgbaImage>> {
    images.iter().map(|i| to_rgba8(i).and_then(|px| image::RgbaImage::from_raw(i.width, i.height, px))).collect()
}

fn to_rgba8(img: &gltf::image::Data) -> Option<Vec<u8>> {
    use gltf::image::Format;
    let px = (img.width * img.height) as usize;
    let p = &img.pixels;
    Some(match img.format {
        Format::R8G8B8A8 => p.clone(),
        Format::R8G8B8 => p.as_chunks::<3>().0.iter().flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        Format::R8G8 => p.as_chunks::<2>().0.iter().flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        Format::R8 => p.iter().flat_map(|&v| [v, v, v, 255]).collect(),
        Format::R16G16B16A16 => p
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| c[1])
            .collect::<Vec<_>>()
            .as_chunks::<4>()
            .0
            .iter()
            .flatten()
            .copied()
            .collect(),
        Format::R16G16B16 => p.as_chunks::<6>().0.iter().flat_map(|c| [c[1], c[3], c[5], 255]).collect(),
        _ => return None,
    })
    .filter(|v: &Vec<u8>| v.len() == px * 4)
}

/// Flatten the default scene into world-space triangle lists.
fn bake(doc: &gltf::Document, buffers: &[gltf::buffer::Data]) -> Vec<Draw> {
    let mut draws = Vec::new();
    let scene = doc.default_scene().or_else(|| doc.scenes().next());
    let Some(scene) = scene else {
        // No scene: draw every mesh once, untransformed.
        for mesh in doc.meshes() {
            bake_mesh(&mesh, Mat4::IDENTITY, buffers, &mut draws);
        }
        return draws;
    };
    let mut stack: Vec<(gltf::Node<'_>, Mat4)> = scene.nodes().map(|n| (n, Mat4::IDENTITY)).collect();
    while let Some((node, parent)) = stack.pop() {
        let world = parent * Mat4::from_cols_array_2d(&node.transform().matrix());
        if let Some(mesh) = node.mesh() {
            bake_mesh(&mesh, world, buffers, &mut draws);
        }
        stack.extend(node.children().map(|c| (c, world)));
    }
    draws
}

fn bake_mesh(mesh: &gltf::Mesh<'_>, world: Mat4, buffers: &[gltf::buffer::Data], out: &mut Vec<Draw>) {
    let normal_m = Mat3::from_mat4(world).inverse().transpose();
    for prim in mesh.primitives() {
        if prim.mode() != gltf::mesh::Mode::Triangles {
            continue;
        }
        let reader = prim.reader(|b| buffers.get(b.index()).map(|d| &d.0[..]));
        let Some(positions) = reader.read_positions() else { continue };
        let positions: Vec<Vec3> = positions.map(|p| world.transform_point3(Vec3::from(p))).collect();
        let indices: Vec<u32> = match reader.read_indices() {
            Some(i) => i.into_u32().collect(),
            None => (0..positions.len() as u32).collect(),
        };
        if indices.iter().any(|&i| i as usize >= positions.len()) {
            continue;
        }
        let normals: Vec<Vec3> = match reader.read_normals() {
            Some(n) => n.map(|n| (normal_m * Vec3::from(n)).normalize_or_zero()).collect(),
            None => flat_normals(&positions, &indices),
        };
        let uvs: Vec<[f32; 2]> =
            reader.read_tex_coords(0).map_or_else(|| vec![[0.0, 0.0]; positions.len()], |t| t.into_f32().collect());
        let vertices = positions
            .iter()
            .enumerate()
            .map(|(i, p)| Vertex {
                pos: p.to_array(),
                normal: normals.get(i).copied().unwrap_or(Vec3::Y).to_array(),
                uv: uvs.get(i).copied().unwrap_or([0.0, 0.0]),
            })
            .collect();
        let material = prim.material();
        let pbr = material.pbr_metallic_roughness();
        out.push(Draw {
            vertices,
            indices,
            color: pbr.base_color_factor(),
            texture: pbr.base_color_texture().map(|t| t.texture().source().index()),
            alpha: match material.alpha_mode() {
                gltf::material::AlphaMode::Opaque => Alpha::Opaque,
                gltf::material::AlphaMode::Mask => Alpha::Mask(material.alpha_cutoff().unwrap_or(0.5)),
                gltf::material::AlphaMode::Blend => Alpha::Blend,
            },
            transmission: material.transmission().map_or(0.0, |t| t.transmission_factor().clamp(0.0, 1.0)),
            metallic: pbr.metallic_factor(),
            roughness: pbr.roughness_factor(),
            metallic_roughness: pbr.metallic_roughness_texture().map(|t| t.texture().source().index()),
        });
    }
}

fn flat_normals(positions: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for tri in indices.as_chunks::<3>().0 {
        let [a, b, c] = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
        let n = (positions[b] - positions[a]).cross(positions[c] - positions[a]);
        for i in [a, b, c] {
            normals[i] += n;
        }
    }
    normals.into_iter().map(Vec3::normalize_or_zero).collect()
}

fn bounds(draws: &[Draw]) -> Option<(Vec3, f32)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glass_is_read_from_transmission() {
        // One triangle, three positions (36 bytes) in an embedded buffer, worn by a glass material.
        let data = "AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAA";
        let gltf = format!(
            r#"{{"asset":{{"version":"2.0"}},"extensionsUsed":["KHR_materials_transmission"],
            "scene":0,"scenes":[{{"nodes":[0]}}],"nodes":[{{"mesh":0}}],
            "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}},"material":0}}]}}],
            "materials":[{{"pbrMetallicRoughness":{{"metallicFactor":0,"roughnessFactor":0.1}},
              "extensions":{{"KHR_materials_transmission":{{"transmissionFactor":0.9}}}}}}],
            "accessors":[{{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3",
              "min":[0,0,0],"max":[1,1,0]}}],
            "bufferViews":[{{"buffer":0,"byteLength":36}}],
            "buffers":[{{"byteLength":36,"uri":"data:application/octet-stream;base64,{data}"}}]}}"#
        );
        let (doc, buffers, _) = gltf::import_slice(gltf.as_bytes()).unwrap();
        let draws = bake(&doc, &buffers);
        assert_eq!(draws.len(), 1);
        let d = &draws[0];
        assert!((d.transmission - 0.9).abs() < 1e-6);
        assert!(d.metallic.abs() < 1e-6 && (d.roughness - 0.1).abs() < 1e-6);
        assert_eq!(d.alpha, Alpha::Opaque, "the alpha mode stays; the renderer splits off the clear part");
    }
}
