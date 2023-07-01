use glam::Vec3;

#[derive(Clone, Copy)]
pub enum PhysicalInteraction {
    ForceApplied(f32, Vec3),
}

pub struct PhysicalBody {
    pub mass: f32,
    pub momentum: Vec3,
    resultant_force: Vec3,

    pub angular_inertia: f32,
    pub angular_momentum: Vec3,
    resultant_ang_force: Vec3,
}

impl PhysicalBody {
    pub fn new(mass: f32, angular_inertia: f32) -> Self {
        Self {
            mass,
            angular_inertia,
            momentum: Vec3::ZERO,
            resultant_force: Vec3::ZERO,
            angular_momentum: Vec3::ZERO,
            resultant_ang_force: Vec3::ZERO,
        }
    }

    pub fn position_delta<'a>(&self, delta: f32) -> Vec3 {
        (self.momentum / self.mass) * delta
            + (self.resultant_force / self.mass) * delta.powi(2) * 0.5
    }

    pub fn velocity(&self) -> Vec3 {
        self.momentum / self.mass
    }

    pub fn angular_veolcity(&self) -> Vec3 {
        self.angular_momentum / self.angular_inertia
    }

    pub fn resultant_force(&self) -> Vec3{
        self.resultant_force
    }

    pub fn update<'a>(&mut self, delta: f32) {
        self.momentum += self.resultant_force * delta;
        self.angular_momentum += self.resultant_ang_force * delta;
        self.resultant_force = Vec3::ZERO;
        self.resultant_ang_force = Vec3::ZERO;
    }

    pub fn add_force(&mut self, force: Vec3) {
        self.resultant_force += force;
    }
}
