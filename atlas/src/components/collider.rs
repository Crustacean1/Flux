use glam::Vec3;

use super::{physical_body::PhysicalBody, transform::Transform};

pub struct Collider {
    pub radius: f32,
    pub callback: Option<Box<dyn Fn(usize, usize, Vec3)>>,
    pub last_impact: Vec3,
    pub toi: f32,
}

pub enum QuadraticSolution {
    None,
    Single(f32),
    Double(f32, f32),
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> QuadraticSolution {
    let delta = b.powi(2) - 4.0 * a * c;
    if delta > 0.0 {
        let delta_sq = delta.sqrt();
        QuadraticSolution::Double((-b - delta_sq) / (2.0 * a), (-b + delta_sq) / (2.0 * a))
    } else if delta == 0.0 {
        QuadraticSolution::Single(b / (2.0 * a))
    } else {
        QuadraticSolution::None
    }
}

pub fn collide(
    delta: f32,
    (transform_a, collider_a, physical_a): (&Transform, &Collider, &PhysicalBody),
    (transform_b, collider_b, physical_b): (&Transform, &Collider, &PhysicalBody),
) -> Option<f32> {
    let velocity = physical_b.velocity() - physical_a.velocity();
    let radius = collider_a.radius + collider_b.radius;
    let position = transform_b.position - transform_a.position;
    if position.length() < velocity.length() + radius || true {
        let [dx, dy, dz] = velocity.to_array();
        let [x, y, z] = position.to_array();

        let (a, b, c) = (
            dx.powi(2) + dy.powi(2) + dz.powi(2),
            2.0 * dx * x + 2.0 * dy * y + 2.0 * dz * z,
            x.powi(2) + y.powi(2) + z.powi(2) - radius.powi(2),
        );
        let time = solve_quadratic(a, b, c);

        match time {
            QuadraticSolution::Single(time) if time < delta => Some(time),
            QuadraticSolution::Double(time1, _) => {
                if time1 > 0.0 && time1 <= delta {
                    Some(time1)
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}
