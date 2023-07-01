use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{hud::HudEntity, sprite::Sprite},
    graphics::{
        material::{sprite_material::SpriteMaterial, Material},
        primitive::Primitive,
        shaders::{ui_shader::SpriteShader, ShaderProgram},
        vertices::{crosshair, generator, indices::TriangleGeometry, layouts::P2TVertex},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct SpriteRenderer {
    pub quad: Primitive<P2TVertex, TriangleGeometry>,
    pub material: SpriteMaterial,
}

impl SpriteRenderer {
    pub fn quad((width, height): (f32, f32), material: SpriteMaterial) -> SpriteRenderer {
        let (vertices, indices) = generator::quad(width, height);
        SpriteRenderer {
            quad: Primitive::new(&vertices, &indices),
            material,
        }
    }

    pub fn crosshair(material: SpriteMaterial) -> Self {
        let (vertices, indices) = crosshair::crosshair();
        let quad = Primitive::new(&vertices, &indices);
        Self { quad, material }
    }
}

pub struct SpriteRendererSystem {
    shader: ShaderProgram<SpriteShader>,
}

impl SpriteRendererSystem {
    pub fn new(shader: ShaderProgram<SpriteShader>) -> Self {
        SpriteRendererSystem { shader }
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a SpriteRenderer)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a SpriteRenderer)> + 'a> {
        let sprites = self
            .iter::<Sprite>()
            .map(|sprite_renderer| (&sprite_renderer.transform, &sprite_renderer.entity.renderer));
        let huds = self
            .iter::<HudEntity>()
            .map(|hud_entity| (&hud_entity.transform, &hud_entity.entity.crosshair));

        Box::new(sprites.chain(huds))
    }
}

impl SpriteRendererSystem {
    pub fn render(&self, entity_manager: &EntityManager, camera: &Camera) {
        let projection = camera.projection();
        self.shader.bind();
        entity_manager
            .get_view()
            .for_each(|(transform, shape): (&Transform, &SpriteRenderer)| {
                shape.material.bind();
                let mvp = projection * transform.model();
                self.shader.bind_projection_view_model(&mvp.to_cols_array());
                shape.quad.render();
            });
    }
}
