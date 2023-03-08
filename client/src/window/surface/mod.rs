use std::{collections::HashMap, iter, num::NonZeroU32, time::Instant};

use shared::{
    addons::AddonManager,
    math::{Mat4, Vec3},
    resources,
    util::GetOrInsert,
};
use wgpu::{util::DeviceExt, Backends, Features, InstanceDescriptor, TextureView};
use winit::window::Window;

use crate::{config::Config, mesher::chunk::generate_mesh};

use self::{
    uniforms::{CameraUniform, SkyUniform},
    vertex::{Vertex, SkyboxVertex, SKYBOX_VERTICES},
};

use super::{
    camera::{Camera, CameraController},
    texture::Texture,
};

mod uniforms;
pub mod vertex;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
]);

pub struct WindowSurface {
    start: Instant,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    skybox_render_pipeline: wgpu::RenderPipeline,
    skybox_vertex_buffer: wgpu::Buffer,
    num_skybox_vertices: u32,
    texture_bind_group: wgpu::BindGroup,
    camera: Camera,
    pub camera_controller: CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    sky_uniform: SkyUniform,
    sky_buffer: wgpu::Buffer,
    sky_bind_group: wgpu::BindGroup,
    depth_texture: Texture,
    window: Window,
}

impl WindowSurface {
    #[profiling::function]
    pub async fn new(
        window: Window,
        client_config: &Config,
        _addon_manager: &AddonManager,
    ) -> Self {
        let start = Instant::now();
        let size = window.inner_size();

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let cap = surface.get_capabilities(&adapter);

        let mut features = Features::empty();
        features.insert(Features::POLYGON_MODE_LINE);
        features.insert(Features::POLYGON_MODE_POINT);
        features.insert(Features::TEXTURE_BINDING_ARRAY);
        features.insert(Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let mut texture_index: Vec<Texture> = vec![];
        let mut textures: HashMap<String, HashMap<String, usize>> = HashMap::new();

        for entry in resources::read_dir(&client_config.addons.0).unwrap() {
            let file_name = entry.as_ref().unwrap().file_name();
            let namespace = file_name.to_str().unwrap();
            let textures_folder = entry.unwrap().path().join("textures");

            for texture in resources::read_dir(textures_folder).unwrap() {
                let file_name = texture.as_ref().unwrap().file_name();
                let name = file_name.to_str().unwrap().split_once(".").unwrap().0;

                let bytes = resources::read_dir_entry_bytes(texture.as_ref().unwrap(), Some("png"));

                if let Ok(bytes) = bytes {
                    let texture = Texture::from_bytes(
                        &device,
                        &queue,
                        &bytes,
                        &format!("{}:{}", namespace, name),
                    )
                    .unwrap();

                    texture_index.push(texture);

                    textures
                        .get_or_insert(namespace, HashMap::new())
                        .insert(name.to_string(), texture_index.len() - 1);
                }
            }
        }

        let views: Vec<&TextureView> = texture_index.iter().map(|e| &e.view).collect();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: NonZeroU32::new(views.len() as u32),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(&views),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        let mut vertices = vec![];
        let mut indices = vec![];

        let mesh = generate_mesh(Vec3::new(-16.0, -16.0, -16.0));
        vertices.extend(mesh.0);
        indices.extend(mesh.1);

        // ============================= QUADS =============================

        let sky_uniform = SkyUniform::new();

        let camera = Camera {
            eye: (0.0, 5.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
        };
        let camera_controller = CameraController::new(0.2);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let sky_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sky Buffer"),
            contents: bytemuck::cast_slice(&[sky_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let sky_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("sky_bind_group_layout"),
            });

        let sky_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &sky_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: sky_buffer.as_entire_binding(),
            }],
            label: Some("sky_bind_group"),
        });

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,
                    &sky_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = create_pipeline(
            true,
            &device,
            &render_pipeline_layout,
            &[Vertex::desc()],
            config.format,
            client_config.debug.0,
            wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(client_config.shaders.0.clone().into()),
            },
        );

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        let skybox_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &sky_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let skybox_render_pipeline = create_pipeline(
            false,
            &device,
            &skybox_render_pipeline_layout,
            &[SkyboxVertex::desc()],
            config.format,
            wgpu::PolygonMode::Fill,
            wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(client_config.shaders.1.clone().into()),
            },
        );

        let skybox_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_skybox_vertices = SKYBOX_VERTICES.len() as u32;

        Self {
            start,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            skybox_render_pipeline,
            skybox_vertex_buffer,
            num_skybox_vertices,
            texture_bind_group,
            camera,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            sky_buffer,
            sky_bind_group,
            sky_uniform,
            depth_texture,
            window,
        }
    }

    #[profiling::function]
    pub fn window(&self) -> &Window {
        &self.window
    }

    #[profiling::function]
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
            self.depth_texture =
                Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    #[profiling::function]
    pub fn update(&mut self, _delta: f32) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // TODO: Sky updates
        self.sky_uniform.update_sun(self.start);
        self.queue.write_buffer(
            &self.sky_buffer,
            0,
            bytemuck::cast_slice(&[self.sky_uniform]),
        );
    }

    #[profiling::function]
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            
            // Skybox
            render_pass.set_pipeline(&self.skybox_render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.sky_bind_group, &[]); // TODO REORDER
            render_pass.set_vertex_buffer(0, self.skybox_vertex_buffer.slice(..));
            render_pass.draw(0..self.num_skybox_vertices, 0..1);

            // World
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.sky_bind_group, &[]); // TODO REORDER
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

            
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn create_pipeline(
    depth: bool,
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    color_format: wgpu::TextureFormat,
    polygon_mode: wgpu::PolygonMode,
    shader: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: Texture::DEPTH_FORMAT,
            depth_write_enabled: depth,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
