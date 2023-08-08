use glam::Vec3;

#[derive(Clone, Copy)]
pub enum Light {
    PointLight(LightColor),
    DirectionalLight(Vec3, LightColor),
}

#[derive(Clone, Copy)]
pub struct LightColor {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}
