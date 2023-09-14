use crate::{
    components::{
        camera::Camera, health_renderer::HealthRenderer, sprite_renderer::SpriteRenderer,
        transform::Transform,
    },
    entity_manager::{self, ComponentIteratorGenerator, EntityManager},
    game_entities::hud::HudEntity,
    graphics::{
        context::Context, material::EmptyMaterial, shaders::health_shader::HealthShaderDefinition,
    },
};

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a HealthRenderer)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a HealthRenderer)> + 'a> {
        let huds = self
            .iter::<HudEntity>()
            .map(|hud_entity| [(&hud_entity.transform, &hud_entity.entity.health)])
            .flatten();

        Box::new(huds)
    }
}

pub struct HealthRendererSystem {
    shader: HealthShaderDefinition,
}

impl HealthRendererSystem {
    pub fn new(shader: HealthShaderDefinition) -> Self {
        Self { shader }
    }

    pub fn render(&self, context: &mut Context, entity_manager: &EntityManager, camera: &Camera) {
        let projection = camera.projection();
        let bars = entity_manager.get_view();

        context.use_shader(&self.shader, |context| {
            context.use_material(&EmptyMaterial {}, |context| {
                bars.filter(|(_, b): &(&Transform, &HealthRenderer)| b.enabled)
                    .for_each(|(transform, bar)| {
                        let mvp = projection * transform.model();
                        context.shader.projection_view_model(&mvp);
                        context.shader.health(bar.health);
                        bar.mesh.render();
                    });
            })
        });
    }
}
