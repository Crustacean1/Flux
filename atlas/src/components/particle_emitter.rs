use glam::{Vec3, Vec4};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    graphics::{
        instanced_primitive::InstancedPrimitive,
        material::particle_material::ParticleMaterial,
        vertices::{
            generator,
            indices::TriangleGeometry,
            layouts::{BufferElement, P2TVertex},
        },
    },
};

use super::transform::Transform;

#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ParticleInstance {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub transform: [f32; 4],
}

impl BufferElement for ParticleInstance {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![3, 3, 2, 2]
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
    pub mesh: InstancedPrimitive<ParticleInstance, P2TVertex, TriangleGeometry>,
    pub material: ParticleMaterial,
    pub spawner: &'static dyn Fn(&Transform, &mut Particle),
    pub particles: Vec<Particle>,
    pub particle_instances: Vec<ParticleInstance>,
    pub since_last_spawn: f32,
}

impl ParticleEmitter {
    pub fn new(
        definition: ParticleEmitterDefinition,
        material: ParticleMaterial,
        mesh: InstancedPrimitive<ParticleInstance, P2TVertex, TriangleGeometry>,
        spawner: &'static dyn Fn(&Transform, &mut Particle),
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
