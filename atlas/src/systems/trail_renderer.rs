use crate::{
    components::{
        camera::Camera, collider::Collider, physical_body::PhysicalBody, transform::Transform,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    event_bus::EventReader,
    graphics::{
        context::Context,
        material::EmptyMaterial,
        mesh::Mesh,
        shaders::flat_shader::{FlatShader, FlatShaderDefinition},
        vertices::{
            indices::LineGeometry,
            layouts::{Attribute, BufferElement, PVertex},
        },
    },
};

struct TrailInstance([f32; 3]);

pub enum TrailEvent {
    Focus(usize),
}

impl BufferElement for TrailInstance {
    fn layout() -> Vec<crate::graphics::vertices::layouts::Attribute> {
        vec![Attribute::Float(3)]
    }
}

pub struct TrailRenderer {
    focus: Option<usize>,
    vertices: Vec<PVertex>,
    indices: Vec<LineGeometry>,
    mesh: Mesh<PVertex, LineGeometry>,
    shader: FlatShaderDefinition,
}

impl TrailRenderer {
    pub fn new(shader: FlatShaderDefinition) -> Self {
        let (vertices, indices) = ([PVertex([0.0, 0.0, 0.0])], [LineGeometry([0, 0])]);
        Self {
            focus: None,
            mesh: Mesh::new(&vertices, &indices),
            vertices: vec![],
            indices: vec![],
            shader,
        }
    }

    pub fn render(
        &mut self,
        context: &mut Context,
        event_reader: &EventReader,
        reference: &PhysicalBody,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        self.focus = None;
        event_reader.read(|e: TrailEvent| match e {
            TrailEvent::Focus(focus) => self.focus = Some(focus),
        });

        self.vertices.clear();
        self.indices.clear();

        entity_manager
            .get_view()
            .filter(|(id, _, _, _)| self.focus.map_or(false, |f| f == *id))
            .enumerate()
            .for_each(
                |(k, (_, transform, _, physical_body)): (
                    _,
                    (usize, &Transform, &Collider, &PhysicalBody),
                )| {
                    let acc = physical_body.resultant_force() / physical_body.mass
                        - (reference.resultant_force() / reference.mass);
                    let mut vel = physical_body.velocity() - reference.velocity();
                    let mut pos = transform.position;

                    let delta = 0.1;

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
        self.mesh.load_vertices(&self.vertices);
        self.mesh.load_indices(&self.indices);
        context.use_shader(&self.shader, |context| {
            let (projection, view) = camera.ind_projection_view(camera_transform);
            context.shader.projection_view(&(projection * view));
            context.use_material(&EmptyMaterial {}, |_| self.mesh.render());
        });
    }
}
