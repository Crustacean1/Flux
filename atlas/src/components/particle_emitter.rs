use crate::graphics::{
    instanced_mesh::InstancedMesh,
    material::particle_material::ParticleMaterial,
    shaders::particle_shader::ParticleInstance,
    vertices::{
        indices::TriangleGeometry,
        layouts::{Attribute, BufferElement, P2TVertex},
    },
};

impl BufferElement for ParticleInstance {
    fn layout() -> Vec<Attribute> {
        vec![
            Attribute::Float(3),
            Attribute::Float(3),
            Attribute::Float(2),
            Attribute::Float(2),
        ]
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Particle {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub size: f32,
    pub color: [f32; 3],
    pub lifetime: f32,
}

#[derive(Clone)]
pub struct ParticleEmitterDefinition {
    pub count: usize,
    pub rate: f32,
}

pub struct ParticleEmitter {
    pub definition: ParticleEmitterDefinition,
    pub mesh: InstancedMesh<ParticleInstance, P2TVertex, TriangleGeometry>,
    pub material: ParticleMaterial,
    pub spawner: Box<dyn Fn(&mut Particle)>,
    pub particles: Vec<Particle>,
    pub particle_instances: Vec<ParticleInstance>,
    pub since_last_spawn: f32,
}

impl ParticleEmitter {
    pub fn new(
        definition: ParticleEmitterDefinition,
        material: ParticleMaterial,
        mesh: InstancedMesh<ParticleInstance, P2TVertex, TriangleGeometry>,
        spawner: Box<dyn Fn(&mut Particle)>,
    ) -> Self {
        Self {
            particles: Vec::with_capacity(definition.count),
            particle_instances: vec![Default::default(); definition.count],
            material,
            mesh,
            definition,
            since_last_spawn: 0.0,
            spawner,
        }
    }
}
