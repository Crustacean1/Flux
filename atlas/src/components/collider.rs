use glam::Vec3;

use super::transform::Transform;

pub enum Collider {
    Sphere(u32, f32),
    Box(u32, f32, f32, f32, f32),
}

pub fn collide(a: (&Transform, &Collider), b: (&Transform, &Collider)) -> Option<Vec3> {
    None
}
