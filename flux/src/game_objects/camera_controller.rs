use atlas::components::{camera::Camera, controller::Controller};
use glam::{Mat3, Mat4, Vec3};

pub struct UserCameraController {
    prev: (f32, f32),
    angles: (f32, f32),
    speed: f32,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    pos: Vec3,
}

impl UserCameraController {
    pub fn new() -> Self {
        UserCameraController {
            prev: (0.0, 0.0),
            angles: (0.0, 0.0),
            speed: 0.002,
            forward: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            pos: Vec3::new(0.0, 0.0, -1.0),
        }
    }
}

impl Controller for UserCameraController {
    fn look(&mut self, (x, y): (f32, f32), camera: &mut Camera) {
        let (delta_x, delta_y) = (
            (self.prev.0 - x).max(-50.0).min(50.0),
            (self.prev.1 - y).max(-50.0).min(50.0),
        );
        self.prev = (x, y);
        self.angles.0 += delta_x * self.speed;
        self.angles.1 += delta_y * self.speed;

        let rot_mat = Mat3::from_rotation_y(self.angles.0) * Mat3::from_rotation_x(self.angles.1);

        self.forward = rot_mat * Vec3::new(0.0, 0.0, 1.0);
        self.right = rot_mat * Vec3::new(1.0, 0.0, 0.0);
        self.up = rot_mat * Vec3::new(0.0, 1.0, 0.0);

        camera.view(self.right, self.up, self.forward, self.pos);
    }

    fn translate(&mut self, movement: Vec3, camera: &mut Camera) {
        self.pos += self.right * movement.x + self.forward * movement.z;
        camera.view(self.right, self.up, self.forward, self.pos);
    }
}
