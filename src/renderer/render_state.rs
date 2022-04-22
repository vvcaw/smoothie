extern crate lyon;

use crate::renderer::vertex::Vertex;
use crate::smoothie::DOM;
use std::collections::btree_map::Range;

use lyon::math::point;
use lyon::path::{FillRule, Path};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeTessellator, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
};
use std::sync::MutexGuard;
use wgpu::util::DeviceExt;
use wgpu::{Backends, Buffer};
use winit::event::WindowEvent;
use winit::window::Window;

/// This vertex constructor forwards the positions and normals provided by the
/// tessellators and add a shape id.
pub struct WithId(pub u32);

impl FillVertexConstructor<Vertex> for WithId {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            color: [0.0, 1.0, 0.0],
            prim_id: self.0,
        }
    }
}

impl StrokeVertexConstructor<Vertex> for WithId {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            color: [0.0, 1.0, 0.0],
            prim_id: self.0,
        }
    }
}

/// The **Renderer** struct
pub struct RenderState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    sample_count: i32,
    index_buffer: Buffer,
    vertex_buffer: Buffer,
    num_indices: u32,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
}

impl RenderState {
    // Creating some of the wgpu types requires async code
    /// Initializes all relevant data for **Renderer**
    pub async fn new(window: &Window) -> Self {
        // Build a Path for the arrow.
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

        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();

        let mut fill_tess = FillTessellator::new();

        fill_tess
            .tessellate_path(
                &arrow_path,
                &FillOptions::tolerance(tolerance).with_fill_rule(FillRule::NonZero),
                &mut BuffersBuilder::new(&mut geometry, WithId(arrow_prim_id as u32)),
            )
            .unwrap();

        let num_indices = geometry.indices.len() as u32;

        geometry.vertices.iter().for_each(|vertex| {
            println!("{:?}", vertex);
        });

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

        let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&geometry.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
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

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
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
                count: 1,
                mask: !0,
                // Anti-Aliasing
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Number of samples for anti-aliasing
        let sample_count = 4;

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            sample_count,
            vertex_buffer: vbo,
            index_buffer: ibo,
            num_indices,
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

    // TODO: Think about removing event handling entirely
    /// Processes the given **event**
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        // If this returns true, the event loop is not executed further!
        false
    }

    /// Renders the current frame
    pub fn render(&mut self, dom: MutexGuard<DOM>) -> Result<(), wgpu::SurfaceError> {
        // Receive DOM as MutexGuard<DOM> to unlock after rendering
        dom.iter().for_each(|(k, v)| {
            //println!("{}, {}", k, v);
        });

        // Wait for surface to provide a new SurfaceTexture
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // command_encoder is borrowed here, but dropped after scope ends to access it later
        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            //render_pass.draw(0..self.num_vertices, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();

        Ok(())
    }
}
