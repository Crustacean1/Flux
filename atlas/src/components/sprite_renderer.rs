use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{hud::HudEntity, sprite::Sprite},
    graphics::{
        context::Context,
        material::sprite_material::SpriteMaterial,
        mesh::Mesh,
        shaders::sprite_shader::SpriteShaderDefinition,
        vertices::{
            crosshair, generator,
            health_bar::{self, health_bar},
            indices::TriangleGeometry,
            layouts::P2TVertex,
        },
    },
};

use super::{camera::Camera, transform::Transform};

pub struct SpriteRenderer {
    pub quad: Mesh<P2TVertex, TriangleGeometry>,
    pub material: SpriteMaterial,
}

impl SpriteRenderer {
    pub fn quad((width, height): (f32, f32), material: SpriteMaterial) -> SpriteRenderer {
        let (vertices, indices) = generator::quad(width, height);
        SpriteRenderer {
            quad: Mesh::new(&vertices, &indices),
            material,
        }
    }

    pub fn crosshair(material: SpriteMaterial) -> Self {
        let (vertices, indices) = crosshair::crosshair();
        let quad = Mesh::new(&vertices, &indices);
        Self { quad, material }
    }

}

pub struct SpriteRendererSystem {
    shader: SpriteShaderDefinition,
}

impl SpriteRendererSystem {
    pub fn new(shader: SpriteShaderDefinition) -> Self {
        SpriteRendererSystem { shader }
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a SpriteRenderer)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a SpriteRenderer)> + 'a> {
        let huds = self
            .iter::<HudEntity>()
            .map(|hud_entity| (&hud_entity.transform, &hud_entity.entity.crosshair));

        Box::new(huds)
    }
}

impl SpriteRendererSystem {
    pub fn render(&self, context: &mut Context, entity_manager: &EntityManager, camera: &Camera) {
        let projection = camera.projection();
        let sprites = entity_manager.get_view();

        context.use_shader(&self.shader, |context| {
            sprites.for_each(|(transform, shape): (&Transform, &SpriteRenderer)| {
                context.use_material(&shape.material, |context| {
                    let mvp = projection * transform.model();
                    context.shader.projection_view(&mvp);
                    context.shader.sprite(0);
                    shape.quad.render();
                });
            });
        });
    }
}
