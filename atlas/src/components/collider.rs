use glam::Vec3;

use super::transform::Transform;

pub struct Collider {
    pub radius: f32,
    pub callback: Option<Box<dyn Fn(Vec3)>>,
}

pub fn collide(
    (transform_a, collider_a): (&Transform, &Collider),
    (transform_b, collider_b): (&Transform, &Collider),
) -> Option<Vec3> {
    let translation = transform_b.position - transform_a.position;
    let distance = translation.length_squared();
    let radius_square_sum = collider_a.radius.powi(2) + collider_b.radius.powi(2);
    if distance < radius_square_sum {
        let distance = distance.sqrt();
        let d = (collider_a.radius.powi(2) - collider_b.radius.powi(2) + distance.powi(2))
            / (2.0 * distance);
        Some(transform_a.position + translation.normalize() * d)
    } else {
        None
    }
}
