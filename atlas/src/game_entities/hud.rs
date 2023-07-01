use crate::components::{sprite_renderer::SpriteRenderer, text_renderer::TextRenderer};

pub struct HudEntity {
    pub crosshair: SpriteRenderer,
    pub velocity: TextRenderer,
}
