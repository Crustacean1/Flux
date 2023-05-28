use glam::Vec3;

pub enum Light {
    PointLight(LightColor),
    DirectionalLight(Vec3, LightColor),
}

pub struct LightColor {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}
