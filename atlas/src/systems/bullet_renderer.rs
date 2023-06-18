use glam::Mat4;

use crate::{
    components::{camera::Camera, transform::Transform},
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::bullet::BulletEntity,
    graphics::{
        instanced_primitive::InstancedPrimitive,
        shaders::{bullet_shader::BulletShader, ShaderProgram},
        vertices::{
            bullet::bullet,
            indices::{LineGeometry, TriangleGeometry},
            layouts::{Attribute, BufferElement, PVertex},
        },
    },
};

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a BulletEntity)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a BulletEntity)> + 'a> {
        Box::new(
            self.iter()
                .map(|bullet| (&bullet.transform, &bullet.entity)),
        )
    }
}

struct BulletInstance {
    pub transform: [f32; 16],
}

impl BufferElement for BulletInstance {
    fn layout() -> Vec<Attribute> {
        vec![
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
        ]
    }
}

pub struct BulletRenderer {
    shader: ShaderProgram<BulletShader>,
    meshes: InstancedPrimitive<BulletInstance, PVertex, TriangleGeometry>,
    bullet_instances: Vec<BulletInstance>,
}

impl BulletRenderer {
    pub fn new(shader: ShaderProgram<BulletShader>) -> Self {
        let (vertices, indices) = bullet(0.05, 0.25);

        BulletRenderer {
            shader,
            meshes: InstancedPrimitive::new(&vertices, &indices, &vec![]),
            bullet_instances: vec![],
        }
    }

    pub fn render_bullets(
        &mut self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        self.reload_instances(entity_manager);
        let (projection, view) = camera.projection_view(camera_transform);

        self.shader.bind();
        self.shader
            .bind_projection_view(&projection.to_cols_array(), &view.to_cols_array());
        self.meshes.load_instances(&self.bullet_instances);
        self.meshes.render();
    }

    fn reload_instances(&mut self, entity_manager: &EntityManager) {
        self.bullet_instances.clear();
        entity_manager
            .get_view()
            .for_each(|(transform, _): (&Transform, &BulletEntity)| {
                self.bullet_instances.push(BulletInstance {
                    transform: transform.model().to_cols_array(),
                })
            });
    }
}
