use glam::Vec3;

use crate::graphics::{
    instanced_primitive::InstancedPrimitive,
    material::particle_material::ParticleMaterial,
    vertices::{
        generator,
        indices::TriangleGeometry,
        layouts::{BufferElement, P2TVertex},
    },
};

#[repr(C)]
pub struct ParticleInstance {
    position: [f32; 3],
    transform: [f32; 4],
}

impl BufferElement for ParticleInstance {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![3, 4]
    }
}

pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub size: f32,
    pub color: [f32; 3],
    pub lifetime: f32,
}

pub struct ParticleEmmiterDefinition {
    pub particle_count: usize,
    pub start_velocity: Vec3,
    pub lifetime: f32,
}

pub struct ParticleEmitter {
    particles: Vec<Particle>,
    pub material: ParticleMaterial,
    pub mesh: InstancedPrimitive<ParticleInstance, P2TVertex, TriangleGeometry>,
}

impl ParticleEmitter {
    pub fn new(material: ParticleMaterial, count: usize) -> Self {
        let (vertices, indices) = generator::quad(1., 1.);

        let instances = vec![
            ParticleInstance {
                position: [0.0, 5.0, 0.0],
                transform: [1.0, 1.0, 1.0, 1.0],
            },
            ParticleInstance {
                position: [0.0, 0.0, 0.0],
                transform: [1.0, 0.0, 0.0, 1.0],
            },
        ];

        let mesh = InstancedPrimitive::new(&vertices, &indices, &instances);

        Self {
            particles: vec![],
            mesh,
            material,
        }
    }

    pub fn update(delta: f32) {}
}
