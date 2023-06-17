use glam::{Mat4, Vec3};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::ui_label::UiLabel,
    graphics::{
        primitive::Primitive,
        shaders::{text_shader::TextShader, ShaderProgram},
        vertices::{indices::TriangleGeometry, layouts::P2TVertex},
    },
    resource_manager::font::Font,
};

use super::{camera::Camera, transform::Transform};

pub struct TextRendererSystem {
    shader: ShaderProgram<TextShader>,
}

impl TextRendererSystem {
    pub fn new(shader: ShaderProgram<TextShader>) -> Self {
        TextRendererSystem { shader }
    }
}

pub struct TextRenderer {
    text: String,
    primitive: Primitive<P2TVertex, TriangleGeometry>,
    font: Font,
}

impl TextRenderer {
    pub fn new(text: &str, font: Font) -> Self {
        let mut primitive = Primitive::new(&vec![], &vec![]);
        font.render(text, &mut primitive);

        TextRenderer {
            text: String::from(text),
            primitive,
            font,
        }
    }

    pub fn primitive(&self) -> &Primitive<P2TVertex, TriangleGeometry> {
        &self.primitive
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a TextRenderer)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a TextRenderer)> + 'a> {
        let labels = self
            .iter::<UiLabel>()
            .map(|label| (&label.transform, &label.entity.renderer));

        Box::new(labels)
    }
}

impl TextRendererSystem {
    pub fn render(&mut self, entity_manager: &EntityManager, camera: &Camera) {
        self.shader.bind();

        entity_manager.get_view().for_each(
            |(transform, text_renderer): (&Transform, &TextRenderer)| {
                let projection_view_model = camera.projection() * transform.model();

                text_renderer.font.bind();
                self.shader
                    .bind_projection_view_model(&projection_view_model.to_cols_array());
                self.shader.bind_atlas(0);

                text_renderer.primitive().render();
            },
        );
    }
}
