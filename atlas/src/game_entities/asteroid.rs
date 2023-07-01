use std::f32::consts::PI;

use glam::{Vec2, Vec3};
use rand::Rng;

use crate::{
    components::{collider::Collider, physical_body::PhysicalBody},
    game_root::GameError,
    graphics::{
        material::phong_material::PhongMaterial,
        mesh::Mesh,
        primitive::Primitive,
        texture::{ChannelLayout, Texture},
        vertices::asteroid::asteroid,
    },
};

pub struct AsteroidEntity {
    pub mesh: Mesh,
    pub body: PhysicalBody,
    pub collider: Collider,
}

impl AsteroidEntity {
    pub fn prefab(material: PhongMaterial, radius: f32) -> Self {
        let (vertices, indices) = asteroid(radius, 15);
        let primitive = Primitive::new(&vertices, &indices);
        let mut rnd = rand::thread_rng();
        let mut body = PhysicalBody::new(100.0, 100.0);
        body.momentum = Vec3::new(
            rnd.gen_range(-150.0..150.0),
            rnd.gen_range(-150.0..150.0),
            rnd.gen_range(-150.0..150.0),
        );
        Self {
            mesh: Mesh {
                primitives: vec![(material, primitive)],
            },
            body,
            collider: Collider { radius: radius *1.1},
        }
    }
}

pub fn generate_asteroid((width, height): (usize, usize)) -> Result<Texture, GameError> {
    let mut buffer = vec![96; width * height * 3];
    add_perlin_noise(&mut buffer, (width, height), 4, 64.0);
    add_perlin_noise(&mut buffer, (width, height), 16, 32.0);
    add_perlin_noise(&mut buffer, (width, height), 32, 32.0);
    add_perlin_noise(&mut buffer, (width, height), 64, 32.0);
    Texture::from_buff(&buffer, ChannelLayout::Rgb8, (width as u32, height as u32))
}

pub fn add_perlin_noise(
    buffer: &mut [u8],
    (width, height): (usize, usize),
    details: usize,
    strength: f32,
) {
    let mut rnd = rand::thread_rng();

    let mut perlin_grid = vec![];
    for _ in 0..details {
        let mut row = vec![];
        for _ in 0..details {
            let angle = rnd.gen_range(0.0..2.0 * PI);
            row.push(Vec2::new(angle.cos(), angle.sin()));
        }
        perlin_grid.push(row);
    }

    let perlin_grid = &perlin_grid;
    let x_step = details as f32 / width as f32;
    let y_step = details as f32 / height as f32;

    (0..(height)).for_each(|y| {
        (0..(width)).for_each(|x| {
            let index = (y * width + x) * 3;
            let intensity = (strength
                * perlin(
                    Vec2::new(x as f32 * x_step, y as f32 * y_step),
                    perlin_grid,
                    details,
                )) as u8;
            buffer[index] += intensity;
            buffer[index + 1] += intensity;
            buffer[index + 2] += intensity;
        })
    });
}

pub fn perlin(pos: Vec2, perlin_grid: &[Vec<Vec2>], spacing: usize) -> f32 {
    let x = pos.x.floor() as usize;
    let y = pos.y.floor() as usize;
    let dx = pos.x - pos.x.floor();
    let dy = pos.y - pos.y.floor();

    let vecs = [
        perlin_grid[x][y],
        perlin_grid[(x + 1) % spacing][y],
        perlin_grid[(x + 1) % spacing][(y + 1) % spacing],
        perlin_grid[x][(y + 1) % spacing],
    ];

    let offsets = [
        Vec2::new(dx, dy),
        Vec2::new(dx - 1.0, dy),
        Vec2::new(dx - 1.0, dy - 1.0),
        Vec2::new(dx, dy - 1.0),
    ];

    let dots: Vec<_> = vecs
        .iter()
        .zip(offsets.iter())
        .map(|(&vec, &offset)| vec.dot(offset))
        .collect();

    let smooth_x = smooth(dx);
    let smooth_y = smooth(dy);

    lerp(
        lerp(dots[0], dots[1], smooth_x),
        lerp(dots[3], dots[2], smooth_x),
        smooth_y,
    )
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn smooth(x: f32) -> f32 {
    ((6.0 * x - 15.0) * x + 10.0) * x * x * x
}
