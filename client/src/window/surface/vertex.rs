use shared::math::{UVec2, UVec4};

pub static SKYBOX_VERTICES: &[SkyboxVertex] = &[
    SkyboxVertex::new(-1.0, 1.0, -1.0),
    SkyboxVertex::new(-1.0, -1.0, -1.0),
    SkyboxVertex::new(1.0, -1.0, -1.0),
    SkyboxVertex::new(1.0, -1.0, -1.0),
    SkyboxVertex::new(1.0, 1.0, -1.0),
    SkyboxVertex::new(-1.0, 1.0, -1.0),

    SkyboxVertex::new(-1.0, -1.0, 1.0),
    SkyboxVertex::new(-1.0, -1.0, -1.0),
    SkyboxVertex::new(-1.0, 1.0, -1.0),
    SkyboxVertex::new(-1.0, 1.0, -1.0),
    SkyboxVertex::new(-1.0, 1.0, 1.0),
    SkyboxVertex::new(-1.0, -1.0, 1.0),

    SkyboxVertex::new(1.0, -1.0, -1.0),
    SkyboxVertex::new(1.0, -1.0, 1.0),
    SkyboxVertex::new(1.0, 1.0, 1.0),
    SkyboxVertex::new(1.0, 1.0, 1.0),
    SkyboxVertex::new(1.0, 1.0, -1.0),
    SkyboxVertex::new(1.0, -1.0, -1.0),

    SkyboxVertex::new(-1.0, -1.0, 1.0),
    SkyboxVertex::new(-1.0, 1.0, 1.0),
    SkyboxVertex::new(1.0, 1.0, 1.0),
    SkyboxVertex::new(1.0, 1.0, 1.0),
    SkyboxVertex::new(1.0, -1.0, 1.0),
    SkyboxVertex::new(-1.0, -1.0, 1.0),

    SkyboxVertex::new(-1.0, 1.0, -1.0),
    SkyboxVertex::new(1.0, 1.0, -1.0),
    SkyboxVertex::new(1.0, 1.0, 1.0),
    SkyboxVertex::new(1.0, 1.0, 1.0),
    SkyboxVertex::new(-1.0, 1.0, 1.0),
    SkyboxVertex::new(-1.0, 1.0, -1.0),

    SkyboxVertex::new(-1.0, -1.0, -1.0),
    SkyboxVertex::new(-1.0, -1.0, 1.0),
    SkyboxVertex::new(1.0, -1.0, -1.0),
    SkyboxVertex::new(1.0, -1.0, -1.0),
    SkyboxVertex::new(-1.0, -1.0, 1.0),
    SkyboxVertex::new(1.0, -1.0, 1.0)
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture_index: u32,
    pub data: u32,
}

impl Vertex {
    #[profiling::function]
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 3]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 3]>() + mem::size_of::<u32>()) as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }

    #[profiling::function]
    pub fn encode(light: UVec4, tex_coords: UVec2) -> u32 {
        (light.x << 28) | (light.y << 24) | (light.z << 20) | (light.w << 16) | (tex_coords.x << 15) | (tex_coords.y << 14)
    }  
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkyboxVertex {
    pub position: [f32; 3],
    pub element_type: u32,
}

impl SkyboxVertex {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            element_type: 0,
        }
    }

    #[profiling::function]
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SkyboxVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}