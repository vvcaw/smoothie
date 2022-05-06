extern crate lyon;

use crate::renderer::primitive::Primitive;
use crate::renderer::vertex::Vertex;
use crate::renderer::with_id::WithId;
use crate::smoothie::DOM;
use std::ops::Range;

use lyon::lyon_tessellation::LineCap;
use lyon::math::point;
use lyon::path::{FillRule, Path};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator, VertexBuffers,
};
use std::sync::MutexGuard;
use wgpu::util::DeviceExt;
use wgpu::{Backends, BindGroup, Buffer};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

const PRIM_BUFFER_LEN: usize = 256;

/// The **Renderer** struct
pub struct RenderState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    index_buffer: Buffer,
    vertex_buffer: Buffer,
    fill_index_range: Range<u32>,
    stroke_index_range: Range<u32>,
    primitives: Vec<Primitive>,
    prims_ubo: Buffer,
    bind_group: BindGroup,
    sample_count: u32,
    size: winit::dpi::PhysicalSize<u32>,
}

impl RenderState {
    // Creating some of the wgpu types requires async code
    /// Initializes all relevant data for **Renderer**
    pub async fn new(window: &Window) -> Self {
        // Build a Path for the arrow. TODO: This should be done by some `impl Element` struct
        let mut builder = Path::builder();
        builder.begin(point(-1.0, -0.3));
        builder.line_to(point(0.0, -0.3));
        builder.line_to(point(0.0, -1.0));
        builder.line_to(point(1.0, 0.0));
        builder.line_to(point(0.0, 1.0));
        builder.line_to(point(0.0, 0.3));
        builder.line_to(point(-1.0, 0.3));
        builder.close();
        let arrow_path = builder.build();

        let tolerance = 0.02;
        let arrow_prim_id = 0;
        let stroke_prim_id = 1;

        // Create the vertex buffer
        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();

        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();

        fill_tess
            .tessellate_path(
                &arrow_path,
                &FillOptions::tolerance(tolerance).with_fill_rule(FillRule::NonZero),
                &mut BuffersBuilder::new(&mut geometry, WithId(arrow_prim_id as u32)),
            )
            .unwrap();

        let fill_index_range = 0..(geometry.indices.len() as u32);

        stroke_tess
            .tessellate_path(
                &arrow_path,
                &StrokeOptions::tolerance(tolerance)
                    .with_line_width(0.05)
                    .with_line_cap(LineCap::Round),
                &mut BuffersBuilder::new(&mut geometry, WithId(stroke_prim_id as u32)),
            )
            .unwrap();

        let stroke_index_range = fill_index_range.end..(geometry.indices.len() as u32);

        geometry.vertices.iter().for_each(|vertex| {
            println!("{:?}", vertex);
        });

        println!("{:?}", fill_index_range);
        println!("{:?}", stroke_index_range);

        let size = window.inner_size();

        // The instance to handle the GPU
        // Backends::all => Vulcan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to initialize adapter!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all features
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .expect("Failed to get device or create queue");

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&geometry.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&geometry.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });

        // Number of samples for anti-aliasing
        let sample_count = 4;

        // Create vector for binding primitives to GPU
        let mut primitives = Vec::with_capacity(PRIM_BUFFER_LEN);
        for _ in 0..PRIM_BUFFER_LEN {
            primitives.push(Primitive {
                color: [1.0, 0.0, 0.0, 1.0],
                z_index: 0,
                width: 0.0,
                translate: [0.0; 3],
                angle: 0.0,
                ..Primitive::DEFAULT
            })
        }

        // Fill arrow primitive data
        primitives[arrow_prim_id] = Primitive {
            color: [0.0, 1.0, 0.0, 1.0],
            z_index: 0,
            width: 1.0,
            scale: 0.8,
            translate: [0.0, 0.0, 0.0],
            ..Primitive::DEFAULT
        };

        primitives[stroke_prim_id] = Primitive {
            color: [0.0, 0.0, 0.0, 1.0],
            z_index: 0,
            width: 1.0,
            scale: 0.8,
            translate: [0.0, 0.0, 0.0],
            ..Primitive::DEFAULT
        };

        println!("{:?}", primitives[0]);
        println!("{:?}", primitives[1]);
        println!("{:?}", primitives[2]);
        println!("{:?}", std::mem::size_of::<Primitive>());

        // Determine size of primitive buffer
        let prim_buffer_byte_size = (PRIM_BUFFER_LEN * std::mem::size_of::<Primitive>()) as u64;

        // Create primitive buffer
        let prims_ubo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Prims ubo"),
            size: prim_buffer_byte_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout for uniform buffers
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(prim_buffer_byte_size),
                },
                count: None,
            }],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(prims_ubo.as_entire_buffer_binding()),
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // Render primitives clock wise
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: sample_count,
                mask: !0,
                // Anti-Aliasing
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            fill_index_range,
            stroke_index_range,
            primitives,
            prims_ubo,
            bind_group,
            sample_count,
            size,
        }
    }

    /// Resizes the **surface** to the new size
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // If width or height of surface changes to zero, the application might crash
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // TODO: Think about exposing this as a function to the user too, and use the normal function as an **physics** or **update** loop and use this for input only, or expose events and time to smoothie
    /// Processes the given **event**
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.primitives[0].scale = (position.x as f32 / self.size.width as f32);
                self.primitives[1].scale = (position.y as f32 / self.size.height as f32);

                return true;
            }
            _ => return false,
        }

        // If this returns true, the event loop is not executed further!
        false
    }

    /// Renders the current frame
    pub fn render(&mut self, dom: MutexGuard<DOM>) -> Result<(), wgpu::SurfaceError> {
        // Receive DOM as MutexGuard<DOM> to unlock after rendering
        dom.iter().for_each(|(k, v)| {
            //println!("{}, {}", k, v);
        });

        let frame = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                println!("Swap-chain error: {:?}", e);
                panic!("Swap-chain error!"); // TODO: Proper error handling
            }
        };

        // Frame view to later render
        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
            label: Some("Multisampled frame descriptor"),
            size: wgpu::Extent3d {
                width: self.size.width,
                height: self.size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: self.sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        };

        // Create multisampled view to later pass to frame view
        let multisampled_render_target = if self.sample_count > 1 {
            Some(
                self.device
                    .create_texture(multisampled_frame_descriptor)
                    .create_view(&wgpu::TextureViewDescriptor::default()),
            )
        } else {
            None
        };

        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // Manipulate data in primitives...
        let a = bytemuck::cast_slice(&self.primitives);
        self.queue.write_buffer(&self.prims_ubo, 0, a);

        // command_encoder is borrowed here, but dropped after scope ends to access it later
        {
            // A resolve target is only supported if the attachment actually uses anti-aliasing
            // So if sample_count == 1 then we must render directly to the surface's buffer
            let color_attachment = if let Some(msaa_target) = &multisampled_render_target {
                wgpu::RenderPassColorAttachment {
                    view: &msaa_target,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                    resolve_target: Some(&frame_view),
                }
            } else {
                wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                    resolve_target: None,
                }
            };

            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[color_attachment],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(self.fill_index_range.clone(), 0, 0..1);
            render_pass.draw_indexed(self.stroke_index_range.clone(), 0, 0..1);
            //render_pass.draw(0..self.num_vertices, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(command_encoder.finish()));
        frame.present();

        Ok(())
    }

    /// Returns the current window size
    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }
}
