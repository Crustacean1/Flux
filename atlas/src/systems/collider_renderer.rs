use glam::{Mat4, Vec3};

use crate::{
    components::{
        camera::Camera, collider::Collider, physical_body::PhysicalBody, transform::Transform,
    },
    entity_manager::{self, ComponentIteratorGenerator, EntityManager},
    graphics::{
        instanced_primitive::InstancedPrimitive,
        primitive::Primitive,
        shaders::{flat_shader::FlatShader, ShaderProgram},
        vertices::{
            indices::TriangleGeometry,
            layouts::{Attribute, BufferElement, PTNVertex, PTVertex},
            sphere::sphere,
        },
    },
};

struct CollisionInstance {
    transform: [f32; 16],
}

impl BufferElement for CollisionInstance {
    fn layout() -> Vec<crate::graphics::vertices::layouts::Attribute> {
        vec![
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
        ]
    }
}

pub struct CollisionRenderer {
    sphere: InstancedPrimitive<CollisionInstance, PTNVertex, TriangleGeometry>,
    shader: ShaderProgram<FlatShader>,
    instances: Vec<CollisionInstance>,
}

impl CollisionRenderer {
    pub fn new(shader: ShaderProgram<FlatShader>) -> Self {
        let (vertices, indices) = sphere(1.0, 50);
        let sphere = InstancedPrimitive::new(&vertices, &indices, &vec![]);

        Self {
            instances: vec![],
            shader,
            sphere,
        }
    }

    pub fn render(
        &mut self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        self.instances.clear();
        entity_manager.get_view().for_each(
            |(transform, collider, _): (&Transform, &Collider, &PhysicalBody)| {
                self.instances.push(CollisionInstance {
                    transform: (Mat4::from_translation(transform.position)
                        * Mat4::from_scale(Vec3::new(
                            collider.radius,
                            collider.radius,
                            collider.radius,
                        )))
                    .to_cols_array(),
                })
            },
        );

        self.shader.bind();
        let (projection, view) = camera.projection_view(camera_transform);
        self.shader.bind_projection_view(&(projection * view));
        self.sphere.load_instances(&self.instances);
        self.sphere.render();
    }
}
