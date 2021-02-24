use super::*;

use bytemuck::__core::fmt::Formatter;
use futures::executor::block_on;
use primitives::*;
use std::collections::HashMap;
use std::error::Error;
use wgpu::util::DeviceExt;
use wgpu::SwapChainError;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn make_vertices(rect: Rect) -> Vec<Vertex> {
    // NOTE: CCW

    vec![
        Vertex {
            position: [rect.origin.x, rect.origin.y + rect.size.height, 0.0],
            tex_coords: [0.0, 0.0],
        }, // A
        Vertex {
            position: [rect.origin.x, rect.origin.y, 0.0],
            tex_coords: [0.0, 1.0],
        }, // B
        Vertex {
            position: [rect.origin.x + rect.size.width, rect.origin.y, 0.0],
            tex_coords: [1.0, 1.0],
        }, // C
        Vertex {
            position: [
                rect.origin.x + rect.size.width,
                rect.origin.y + rect.size.height,
                0.0,
            ],
            tex_coords: [1.0, 0.0],
        },
    ]
}

// We don't need to implement Pod and Zeroable for our indices, because bytemuck has already implemented them for basic types such as u16
const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

pub struct QuadRenderer {
    scale_factor: f64,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    camera_controller: CameraController,
    quads: HashMap<usize, Quad>,
    quad_id_count: usize,
    // wgpu::BindGroupLayout
    // ...
    texture_bind_group_layout: wgpu::BindGroupLayout,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
}

pub struct Quad {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32, // 6. 그냥 일단 넣어놓음
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: Texture,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    rect: Rect,
}

impl Quad {
    fn update_texture(&mut self, queue: &wgpu::Queue, data: &[u8]) {
        self.diffuse_texture.update(queue, data)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct QuadId(usize);

// # Initialization
// window, instance -> surface -> (adapter) -> device, queue
// surface, swap_chain_descriptor -> swap_chain
// shader, render_pipeline_layout -> render_pipeline

// # Render pass
// swap_chain -> frame
// device -> command_encoder
// command_encoder, render_pipeline -> render_pass
// queue.submit

impl Quad {
    fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        camera: &Camera,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        rect: Rect,
    ) -> Self {
        let diffuse_texture = Texture::new(&device, &queue, Some("test texture")).unwrap();
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        let mut vertices = make_vertices(rect);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            rect,
        }
    }
}

impl QuadRenderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let scale_factor = window.scale_factor();
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // BindGroup (GL로 치면 uniform 레이아웃 여기서 서술하는 거임)
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // Camera & Uniforms

        // TODO: 말이 되게 고치기
        let logical_size = size.to_logical::<f32>(scale_factor);
        let camera = Camera {
            left: logical_size.width * -0.5,
            right: logical_size.width * 0.5,
            bottom: logical_size.height * -0.5,
            top: logical_size.height * 0.5,
            near: 0.0,
            far: 1.0,
        };

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        // TODO
        let vs_module = device.create_shader_module(wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(wgpu::include_spirv!("shader.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float3, 1 => Float2],
                }],
            },
            sample_count: 1,                  // AA
            sample_mask: !0,                  // AA
            alpha_to_coverage_enabled: false, // AA
        });

        let camera_controller = CameraController::new(0.2);

        let quads: HashMap<usize, Quad> = HashMap::new();

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            camera,
            camera_controller,
            quads,
            quad_id_count: 0,
            texture_bind_group_layout,
            uniform_bind_group_layout,
            scale_factor,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        // TODO: 말이 되게 고치기
        let logical_size = self.size.to_logical::<f32>(self.scale_factor);
        self.camera.left = logical_size.width * -0.5;
        self.camera.right = logical_size.width * 0.5;
        self.camera.bottom = logical_size.height * -0.5;
        self.camera.top = logical_size.height * 0.5;
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event);
        false
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        for quad in self.quads.values_mut() {
            quad.uniforms.update_view_proj(&self.camera);
            self.queue.write_buffer(
                &quad.uniform_buffer,
                0,
                bytemuck::cast_slice(&[quad.uniforms]),
            );
        }
    }

    pub fn render(&mut self) -> Result<(), RendererError> {
        // swap_chain -> frame
        // device -> command_encoder
        // command_encoder, render_pipeline -> render_pass
        // queue.submit
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    // any colors we draw to this attachment will get drawn to the screen.
                    attachment: &frame.view,
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

            // 어떻게 그릴지 쉽게 갈아치울 수 있구나... (challenge)
            // 다른 쉐이더를 먹인다거나 할 수 있구만..
            render_pass.set_pipeline(&self.render_pipeline);
            for quad in self.quads.values() {
                render_pass.set_bind_group(0, &quad.diffuse_bind_group, &[]);
                render_pass.set_bind_group(1, &quad.uniform_bind_group, &[]);
                render_pass.set_vertex_buffer(0, quad.vertex_buffer.slice(..));
                render_pass.set_index_buffer(quad.index_buffer.slice(..));
                // render_pass.draw(0..(VERTICES.len() as u32), 0..1); // 3 vertices, 1 instance -> gl_VertexIndex
                render_pass.draw_indexed(0..quad.num_indices, 0, 0..1);
            }
        }
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    pub fn new_quad(&mut self, offset_x: f32, offset_y: f32, width: f32, height: f32) -> QuadId {
        self.quad_id_count += 1;
        self.quads.insert(
            self.quad_id_count,
            Quad::new(
                &self.device,
                &self.queue,
                &self.camera,
                &self.texture_bind_group_layout,
                &self.uniform_bind_group_layout,
                Rect::new(Point::new(offset_x, offset_y), Size::new(width, height)),
            ),
        );
        QuadId(self.quad_id_count)
    }

    pub fn update_texture(&mut self, quad_id: QuadId, data: &[u8]) {
        self.quads
            .get_mut(&quad_id.0)
            .unwrap() // TODO
            .update_texture(&self.queue, data);
    }
}

#[derive(Debug)]
pub enum RendererError {
    SwapChainLost,
    SwapChainOutOfMemory,
    Unexpected,
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            RendererError::SwapChainLost => write!(f, "SwapChain lost"),
            RendererError::SwapChainOutOfMemory => write!(f, "SwapChain out of memory"),
            RendererError::Unexpected => write!(f, "Unexpected"),
        }
    }
}

impl std::error::Error for RendererError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // ???
        None
    }
}

impl From<wgpu::SwapChainError> for RendererError {
    fn from(e: SwapChainError) -> Self {
        match e {
            SwapChainError::Lost => RendererError::SwapChainLost,
            SwapChainError::OutOfMemory => RendererError::SwapChainOutOfMemory,
            _ => RendererError::Unexpected,
        }
    }
}
