use crate::{
    components::{camera::Camera, transform::Transform},
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::bullet::BulletEntity,
    graphics::{
        context::Context,
        instanced_mesh::InstancedMesh,
        material::EmptyMaterial,
        shaders::bullet_shader::BulletShaderDefinition,
        vertices::{
            bullet::bullet,
            indices::PointGeometry,
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

pub struct BulletInstance {
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
    shader: BulletShaderDefinition,
    meshes: InstancedMesh<BulletInstance, PVertex, PointGeometry>,
    bullet_instances: Vec<BulletInstance>,
}

impl BulletRenderer {
    pub fn new(shader: BulletShaderDefinition) -> Self {
        let (vertices, indices) = bullet();

        BulletRenderer {
            shader,
            meshes: InstancedMesh::new(&vertices, &indices, &vec![]),
            bullet_instances: vec![],
        }
    }

    pub fn render_bullets(
        &mut self,
        context: &mut Context,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        self.reload_instances(entity_manager);
        self.meshes.load_instances(&self.bullet_instances);
        let material = EmptyMaterial {};

        context.use_shader(&self.shader, |context| {
            let (projection, view) = camera.ind_projection_view(camera_transform);
            context.shader.view(&view);
            context.shader.projection(&projection);

            context.use_material(&material, |_context| {
                self.meshes.render();
            })
        });
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
