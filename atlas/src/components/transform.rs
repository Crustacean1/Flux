use glam::{Mat4, Vec3, Vec4};

pub struct Transform {
    entity_id: usize,
    position: Vec3,
    scale: Vec3,
    rotation: Vec3,
}

impl Transform {
    pub fn mat(&self) -> Mat4 {
        Self::scale(self.scale) * Self::transform(self.position) * Self::rotation(self.rotation)
    }

    fn scale(scale: Vec3) -> Mat4 {
        todo!();
    }

    fn transform(transform: Vec3) -> Mat4 {
        todo!();
    }

    fn rotation(rotation: Vec3) -> Mat4 {
        todo!();
    }
}
