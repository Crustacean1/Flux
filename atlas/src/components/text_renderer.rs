use glam::{Mat4, Vec3};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{hud::HudEntity, ui_label::UiLabel},
    graphics::{
        mesh::Mesh,
        shaders::{text_shader::TextShader, ShaderProgram},
        vertices::{indices::TriangleGeometry, layouts::P2TVertex},
    },
    resource_manager::font::Font,
};

use super::{camera::Camera, transform::Transform};

pub struct TextRendererSystem {
    shader: TextShader,
}

impl TextRendererSystem {
    pub fn new(shader: TextShader) -> Self {
        TextRendererSystem { shader }
    }
}

pub struct TextRenderer {
    text: String,
    primitive: Mesh<P2TVertex, TriangleGeometry>,
    transform: Transform,
    font: Font,
}

impl TextRenderer {
    pub fn new(transform: Transform, text: &str, font: Font) -> Self {
        let mut primitive = Mesh::new(&vec![], &vec![]);
        font.render(text, &mut primitive);

        TextRenderer {
            text: String::from(text),
            transform,
            primitive,
            font,
        }
    }

    pub fn primitive(&self) -> &Mesh<P2TVertex, TriangleGeometry> {
        &self.primitive
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.font.render(&self.text, &mut self.primitive);
    }
}

impl<'a> ComponentIteratorGenerator<'a, (Transform, &'a TextRenderer)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (Transform, &'a TextRenderer)> + 'a> {
        let labels = self.iter::<UiLabel>().map(|label| {
            (
                label.transform.compose(&label.entity.renderer.transform),
                &label.entity.renderer,
            )
        });

        let huds = self.iter::<HudEntity>().map(|hud| {
            (
                hud.transform.compose(&hud.entity.velocity.transform),
                &hud.entity.velocity,
            )
        });

        Box::new(labels.chain(huds))
    }
}

impl TextRendererSystem {
    pub fn render(&mut self, entity_manager: &EntityManager, camera: &Camera) {
        self.shader.bind();

        entity_manager.get_view().for_each(
            |(transform, text_renderer): (Transform, &TextRenderer)| {
                let projection_view_model = camera.projection() * transform.model();

                text_renderer.font.bind();

                let pass = self.shader.new_pass();
                pass.render(text_renderer.primitive(), &projection_view_model, 0);
                text_renderer.primitive().render();
            },
        );
    }
}
