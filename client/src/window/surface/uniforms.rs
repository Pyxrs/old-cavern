use std::time::Instant;

use block_mesh::ilattice::glam::Mat4;
use shared::math::Vec3;

use crate::window::camera::Camera;

use super::OPENGL_TO_WGPU_MATRIX;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    position: [f32; 3],
    padding: f32,
}

impl CameraUniform {
    #[profiling::function]
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            position: [0.0; 3],
            padding: 0.0
        }
    }

    #[profiling::function]
    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).to_cols_array_2d();
        self.position = camera.eye.to_array();
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkyUniform {
    sun_angle: [f32; 3],
    padding: f32,
}

impl SkyUniform {
    #[profiling::function]
    pub fn new() -> Self {
        Self {
            sun_angle: [0.0; 3],
            padding: 0.0,
        }
    }

    #[profiling::function]
    pub fn update_sun(&mut self, start: Instant) {
        let time = start.elapsed().as_secs_f32();
        let mut sun_angle = Vec3::ZERO;

        sun_angle.x = time.sin();
        sun_angle.y = 1.0;//(time / 5.0).sin();
        sun_angle.z = time.cos();
        sun_angle = sun_angle.normalize();

        self.sun_angle = [sun_angle.x, sun_angle.y, sun_angle.z];
    }
}