use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};

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

    pub fn to_global(&self, vec: Vec4) -> Vec4 {
        self.model() * vec
    }

    pub fn compose(&self, transform: &Transform) -> Transform {
        let pos = self.model() * Vec4::from((transform.position, 1.0));
        Transform {
            position: pos.xyz(),
            scale: self.scale,
            rotation: self.rotation,
        }
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
