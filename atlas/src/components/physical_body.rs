use glam::Vec3;

use super::transform::Transform;

#[derive(Clone, Copy)]
pub enum PhysicalInteraction {
    ForceApplied(f32, Vec3),
}

pub struct PhysicalBody {
    pub mass: f32,
    pub momentum: Vec3,
    pub dampening_factor: f32,
    resultant_force: Vec3,

    pub angular_inertia: f32,
    pub angular_momentum: Vec3,
    resultant_ang_force: Vec3,
}

impl PhysicalBody {
    pub fn new(mass: f32, angular_inertia: f32, dampening: f32) -> Self {
        Self {
            mass,
            angular_inertia,
            dampening_factor: dampening,
            momentum: Vec3::ZERO,
            resultant_force: Vec3::ZERO,
            angular_momentum: Vec3::ZERO,
            resultant_ang_force: Vec3::ZERO,
        }
    }

    pub fn position_delta<'a>(&self, delta: f32) -> Vec3 {
        (self.momentum / self.mass) * delta
    }

    pub fn velocity(&self) -> Vec3 {
        self.momentum / self.mass
    }

    pub fn angular_velocity(&self) -> Vec3 {
        self.angular_momentum / self.angular_inertia
    }

    pub fn resultant_force(&self) -> Vec3 {
        self.resultant_force
    }

    pub fn update<'a>(&mut self, delta: f32, transform: &mut Transform) {
        transform.position += self.position_delta(delta);

        self.momentum += self.resultant_force ;
        self.angular_momentum += self.resultant_ang_force * delta;

        self.resultant_force = Vec3::ZERO;
        self.resultant_ang_force = Vec3::ZERO;
    }

    pub fn impulse(&mut self, force: Vec3) {
        self.resultant_force += force;
        //self.momentum += force;
    }
}
