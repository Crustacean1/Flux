use glam::{Mat4, Vec3};

use crate::{
    components::{
        camera::Camera, collider::Collider, physical_body::PhysicalBody, transform::Transform,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{player_ship::PlayerShip, GameEntity},
    graphics::{
        context::Context,
        instanced_mesh::InstancedMesh,
        material::EmptyMaterial,
        shaders::flat_shader::{FlatShader, FlatShaderDefinition},
        vertices::{
            indices::TriangleGeometry,
            layouts::{Attribute, BufferElement, PTNVertex},
            sphere::sphere,
        },
    },
};

struct CollisionInstance {
    transform: [f32; 16],
    direction: [f32; 4],
}

impl BufferElement for CollisionInstance {
    fn layout() -> Vec<crate::graphics::vertices::layouts::Attribute> {
        vec![
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
        ]
    }
}

pub struct CollisionRenderer {
    sphere: InstancedMesh<CollisionInstance, PTNVertex, TriangleGeometry>,
    shader: FlatShaderDefinition,
    instances: Vec<CollisionInstance>,
}

impl CollisionRenderer {
    pub fn new(shader: FlatShaderDefinition) -> Self {
        let (vertices, indices) = sphere(1.0, 50);
        let sphere = InstancedMesh::new(&vertices, &indices, &vec![]);

        Self {
            instances: vec![],
            shader,
            sphere,
        }
    }

    pub fn render(
        &mut self,
        context: &mut Context,
        time: f32,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        self.instances.clear();

        entity_manager.iter::<PlayerShip>().for_each(|player| {
            let (transform, collider) = (&player.transform, &player.entity.collider);
            self.instances.push(CollisionInstance {
                direction: [
                    collider.last_impact.x,
                    collider.last_impact.y,
                    collider.last_impact.z,
                    (time - collider.toi),
                ],
                transform: (Mat4::from_translation(transform.position)
                    * Mat4::from_scale(Vec3::new(
                        collider.radius,
                        collider.radius,
                        collider.radius,
                    )))
                .to_cols_array(),
            })
        });

        self.sphere.load_instances(&self.instances);

        context.use_shader(&mut self.shader, |context| {
            let (projection, view) = camera.projection_view(camera_transform);
            context.shader.projection_view(&(projection * view));
            context.use_material(&EmptyMaterial {}, |context| {
                self.sphere.render();
            })
        });
    }
}
