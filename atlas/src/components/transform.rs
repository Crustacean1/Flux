use glam::{Mat4, Vec3, Vec4};

pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vec3::new(0., 0., 0.),
            scale: Vec3::new(0., 0., 0.),
            rotation: Vec3::new(0., 0., 0.),
        }
    }

    pub fn pos(position: Vec3) -> Self {
        Transform {
            position,
            scale: Vec3::new(1., 1., 1.),
            rotation: Vec3::new(0., 0., 0.),
        }
    }

    pub fn model(&self) -> Mat4 {
        Self::translation(self.position) * Self::scale(self.scale) * Self::rotation(self.rotation)
    }

    fn scale(scale: Vec3) -> Mat4 {
        Mat4::from_scale(scale)
    }

    fn translation(translation: Vec3) -> Mat4 {
        Mat4::from_translation(translation)
    }

    fn rotation(rotation: Vec3) -> Mat4 {
        Mat4::IDENTITY
    }
}
