use crate::{
    components::{
        camera::Camera, collider::Collider, physical_body::PhysicalBody, transform::Transform,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    graphics::{
        mesh::Mesh,
        shaders::flat_shader::FlatShader,
        vertices::{
            indices::LineGeometry,
            layouts::{Attribute, BufferElement, PVertex},
        },
    },
};

struct TrailInstance([f32; 3]);

impl BufferElement for TrailInstance {
    fn layout() -> Vec<crate::graphics::vertices::layouts::Attribute> {
        vec![Attribute::Float(3)]
    }
}

pub struct TrailRenderer {
    vertices: Vec<PVertex>,
    indices: Vec<LineGeometry>,
    mesh: Mesh<PVertex, LineGeometry>,
    shader: FlatShader,
}

impl TrailRenderer {
    pub fn new(shader: FlatShader) -> Self {
        let (vertices, indices) = ([PVertex([0.0, 0.0, 0.0])], [LineGeometry([0, 0])]);
        Self {
            mesh: Mesh::new(&vertices, &indices),
            vertices: vec![],
            indices: vec![],
            shader,
        }
    }

    pub fn render(
        &mut self,
        reference: &PhysicalBody,
        entity_manager: &EntityManager,
        _camera: &Camera,
        _camera_transform: &Transform,
    ) {
        self.vertices.clear();
        self.indices.clear();

        entity_manager.get_view().enumerate().for_each(
            |(k, (transform, _, physical_body)): (_, (&Transform, &Collider, &PhysicalBody))| {
                let acc = physical_body.resultant_force() / physical_body.mass
                    - (reference.resultant_force() / reference.mass);
                let mut vel = physical_body.velocity() - reference.velocity();
                let mut pos = transform.position;

                let delta = 0.2;

                (0..49).for_each(|i| {
                    self.indices
                        .push(LineGeometry([(k * 50 + i) as u32, (k * 50 + i + 1) as u32]))
                });
                (0..50).for_each(|_| {
                    self.vertices.push(PVertex(pos.to_array()));
                    pos += delta * vel;
                    pos += delta.powi(2) * acc;
                    vel += delta * acc;
                })
            },
        );

        //self.shader.bind();
        //let (projection, view) = camera.projection_view(camera_transform);
        //self.shader.bind_projection_view(&(projection * view));
        //self.mesh.load_vertices(&self.vertices);
        //self.mesh.load_indices(&self.indices);
        //self.mesh.render();
    }
}
