use std::f32::consts::PI;

use super::{indices::TriangleGeometry, layouts::PTNVertex};

pub fn sphere(radius: f32, details: usize) -> (Vec<PTNVertex>, Vec<TriangleGeometry>) {
    let vertices = pos_sphere(radius, details)
        .zip(tex_sphere(details))
        .zip(norm_sphere(details))
        .map(|((pos, tex), norm)| PTNVertex(pos, tex, norm))
        .collect();
    let indices = index_sphere(details);
    (vertices, indices)
}

fn pos_sphere(radius: f32, details: usize) -> impl Iterator<Item = [f32; 3]> {
    (0..details)
        .map(move |i| {
            (0..details).map(move |j| {
                let angle_x = 2.0 * PI * j as f32 / (details - 1) as f32;
                let angle_y = PI * i as f32 / (details - 1) as f32 - 0.5 * PI;
                [
                    radius * angle_x.cos() * angle_y.cos(),
                    radius * angle_y.sin(),
                    radius * angle_x.sin() * angle_y.cos(),
                ]
            })
        })
        .flatten()
}

fn norm_sphere(details: usize) -> impl Iterator<Item = [f32; 3]> {
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

fn tex_sphere(details: usize) -> impl Iterator<Item = [f32; 2]> {
    (0..details)
        .map(move |i| {
            (0..details).map(move |j| [j as f32 / details as f32, i as f32 / details as f32])
        })
        .flatten()
}

fn index_sphere(details: usize) -> Vec<TriangleGeometry> {
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
