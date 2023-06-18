use std::f32::consts::PI;

use glam::Vec3;
use rand::Rng;

use super::{indices::TriangleGeometry, layouts::PTNVertex};

pub fn asteroid(radius: f32, details: usize) -> (Vec<PTNVertex>, Vec<TriangleGeometry>) {
    let pos = pos_asteroid(radius, details);
    let tex = tex_asteroid(details);
    let norm = norm_asteroid(details);

    let indices = index_asteroid(details);

    let vertices = pos
        .iter()
        .zip(tex)
        .zip(norm)
        .map(|((pos, tex), norm)| PTNVertex(*pos, tex, norm))
        .collect();

    (vertices, indices)
}

fn pos_asteroid(radius: f32, details: usize) -> Vec<[f32; 3]> {

    let mut rng = rand::thread_rng();
    let radius = radius * rng.gen_range(0.8..5.0);

    let deformations: Vec<_> = (0..(rng.gen_range(10..20)))
        .map(|i| {
            let x_angle = rng.gen_range(0.0..2.0 * PI);
            let y_angle = rng.gen_range(0.0..2.0 * PI);
            [
                x_angle.cos() * y_angle.cos(),
                y_angle.sin(),
                x_angle.sin() * y_angle.cos(),
            ]
        })
        .collect();
    let def_ref = &deformations;

    (0..details)
        .map(|i| {
            (0..details).map(move |j| {
                let angle_x = 2.0 * PI * j as f32 / (details - 1) as f32;
                let angle_y = PI * i as f32 / (details - 1) as f32 - 0.5 * PI;

                let vec = [
                    angle_x.cos() * angle_y.cos(),
                    angle_y.sin(),
                    angle_x.sin() * angle_y.cos(),
                ];

                let min_dist = def_ref
                    .iter()
                    .fold(0.0, |max, def| {
                        (vec[0] * def[0] + vec[1] * def[1] + vec[2] * def[2]).max(max)
                    })
                    .powi(16);

                let radius = radius + min_dist * radius * 0.25;
                let vec = [vec[0] * radius, vec[1] * radius, vec[2] * radius];
                vec
            })
        })
        .flatten()
        .collect()
}

fn norm_asteroid(details: usize) -> impl Iterator<Item = [f32; 3]> {
    (0..details)
        .map(move |i| {
            (0..details).map(move |j| {
                let angle_x = 2.0 * PI * j as f32 / (details - 1) as f32;
                let angle_y = PI * i as f32 / (details - 1) as f32 - 0.5 * PI;
                [
                    angle_x.cos() * angle_y.cos(),
                    angle_y.sin(),
                    angle_x.sin() * angle_y.cos(),
                ]
            })
        })
        .flatten()
}

fn tex_asteroid(details: usize) -> Vec<[f32; 2]> {
    let mut rng = rand::thread_rng();

    let deformations: Vec<_> = (0..(rng.gen_range(10..20)))
        .map(|i| {
            let x_angle = rng.gen_range(0.0..2.0 * PI);
            let y_angle = rng.gen_range(0.0..2.0 * PI);
            //let radius = rng.gen_range(0.8..1.2);

            [
                x_angle.cos() * y_angle.cos(),
                y_angle.sin(),
                x_angle.sin() * y_angle.cos(),
            ]
        })
        .collect();
    let def_ref = &deformations;

    (0..details)
        .map(move |i| {
            (0..details).map(move |j| {
                [
                    i as f32 / (details - 1) as f32,
                    j as f32 / (details - 1) as f32,
                ]
            })
        })
        .flatten()
        .collect()
}

fn index_asteroid(details: usize) -> Vec<TriangleGeometry> {
    (0..(details - 1))
        .map(|i| {
            (0..(details - 1))
                .map(move |j| {
                    [
                        TriangleGeometry([
                            (i * details + j) as u32,
                            (i * details + j + 1) as u32,
                            ((i + 1) * details + j) as u32,
                        ]),
                        TriangleGeometry([
                            ((i + 1) * details + j) as u32,
                            ((i + 1) * details + j + 1) as u32,
                            (i * details + j + 1) as u32,
                        ]),
                    ]
                })
                .flatten()
        })
        .flatten()
        .collect()
}
