use shared::{
    extra::{perspective, Deg, InnerSpace, Matrix4, Point3, Vector3},
    Module,
};

use crate::input::Input;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
}

pub struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_input(&mut self, input: &Module<Input>) {
        let input = input.read().unwrap();

        self.is_forward_pressed = e(input.query_action_strength("forward").unwrap());
        self.is_backward_pressed = e(input.query_action_strength("backward").unwrap());
        self.is_left_pressed = e(input.query_action_strength("left").unwrap());
        self.is_right_pressed = e(input.query_action_strength("right").unwrap());
        self.is_up_pressed = e(input.query_action_strength("up").unwrap());
        self.is_down_pressed = e(input.query_action_strength("down").unwrap());

        // Temporary until camera overhaul
        fn e(strength: f32) -> bool {
            strength.round() as u8 != 0
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();

        if self.is_forward_pressed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }
        if self.is_up_pressed {
            camera.eye += camera.up * self.speed;
        }
        if self.is_down_pressed {
            camera.eye -= camera.up * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the up/ down is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
