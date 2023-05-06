pub trait Material {}

pub struct TextureMaterial {
    texture_id: u32,
}

impl Material for TextureMaterial {}

impl TextureMaterial {
    pub fn from_color(r: f32, g: f32, b: f32) -> Self {
        todo!();
    }
}
