use glam::{Mat3, Mat4, Quat, Vec3, Vec4};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vec3::new(0., 0., 0.),
            scale: Vec3::new(1., 1., 1.),
            rotation: Quat::IDENTITY,
        }
    }

    pub fn pos(position: Vec3) -> Self {
        Transform {
            position,
            scale: Vec3::new(1., 1., 1.),
            rotation: Quat::IDENTITY,
        }
    }

    pub fn model(&self) -> Mat4 {
        self.translation() * self.rotation() * self.scale()
    }

    fn scale(&self) -> Mat4 {
        Mat4::from_scale(self.scale)
    }

    fn translation(&self) -> Mat4 {
        Mat4::from_translation(self.position)
    }

    fn rotation(&self) -> Mat4 {
        Mat4::from_quat(self.rotation)
    }
}
