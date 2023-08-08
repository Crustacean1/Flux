use std::{f32::consts::PI, mem};

use glam::{Vec3, Vec4};
use rand::Rng;

use crate::{
    components::{
        particle_emitter::{Particle, ParticleEmitter},
        transform::Transform,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager},
};

pub fn update_particles(entity_manager: &mut EntityManager, delta: u128) {
    let delta = delta as f32 / 1000_000_000.0;
    entity_manager
        .get_view()
        .for_each(|(transform, emitter): (&Transform, &ParticleEmitter)| {
            let emitter = to_mut(emitter);

            emitter.since_last_spawn += delta;
            emitter.particles.iter_mut().for_each(|particle| {
                particle.lifetime -= delta;
                particle.position[0] += particle.velocity[0] * delta;
                particle.position[1] += particle.velocity[1] * delta;
                particle.position[2] += particle.velocity[2] * delta;
            });

            kill_particles(emitter);
            spawn_particles(emitter, transform);
            update_particle_instances(emitter, transform);
        });
}

pub fn thruster_spawner(particle: &mut Particle) {
    let mut rng = rand::thread_rng();

    let radius = rng.gen_range(0.05..0.1);
    let angle = rng.gen_range(0.0..(2.0 * PI));

    let position = [radius * angle.cos(), radius * angle.sin(), 0.8];
    let velocity = [0.0, 0.0, rng.gen_range(4.0..5.0)];

    particle.lifetime = 0.5 * (0.05 / (radius) + 0.01).powi(2);
    particle.position = position;
    particle.velocity = velocity;
    particle.color = [1.0, 1. - 10.0 * radius, 0.0];
    particle.size = 0.6 - radius * 3.;
}

fn spawn_particles(emitter: &mut ParticleEmitter, _transform: &Transform) {
    let mut particle = Particle {
        position: [0.0, 0.0, 0.0],
        velocity: [0.0, 0.0, 0.0],
        size: 0.0,
        color: [0.0, 0.0, 0.0],
        lifetime: 0.0,
    };

    while emitter.since_last_spawn > emitter.definition.rate
        && emitter.particles.len() < emitter.definition.count
    {
        (emitter.spawner)(&mut particle);
        emitter.particles.push(particle.clone());

        emitter.since_last_spawn -= emitter.definition.rate;
    }
}

fn kill_particles(emitter: &mut ParticleEmitter) {
    emitter.particles = emitter
        .particles
        .iter()
        .filter(|particle| particle.lifetime > 0.0)
        .map(|particle| *particle)
        .collect();
}

fn update_particle_instances(emitter: &mut ParticleEmitter, transform: &Transform) {
    emitter
        .particle_instances
        .iter_mut()
        .zip(emitter.particles.iter())
        .for_each(|(instance, particle)| {
            let position = transform.model() * Vec4::from((Vec3::from(particle.position), 1.0));
            instance.position = [position.x, position.y, position.z];
            instance.color = particle.color;
            instance.transform = [particle.size, 0.0, 0.0, particle.size];
        });

    let size = emitter
        .particles
        .len()
        .min(emitter.particle_instances.len());

    emitter
        .mesh
        .load_instances(&emitter.particle_instances[0..size]);
}

fn to_mut<T>(val: &T) -> &mut T {
    unsafe {
        let val: *const T = val;
        mem::transmute(val)
    }
}
