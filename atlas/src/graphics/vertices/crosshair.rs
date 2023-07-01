use std::f32::consts::PI;

use glam::Vec2;

use super::{indices::TriangleGeometry, layouts::P2TVertex};

pub fn crosshair() -> (Vec<P2TVertex>, Vec<TriangleGeometry>) {
    let lod = 20;
    let mut vertices = pos_crosshair(lod);
    vertices.append(&mut pos_dot(lod));

    let mut indices = index_crosshair(lod);
    indices.append(&mut index_dot(lod));

    let vertices = vertices.iter().map(|&v| P2TVertex(v, [0.0, 0.0])).collect();

    (vertices, indices)
}

fn pos_crosshair(lod: usize) -> Vec<[f32; 2]> {
    let angle_start = -PI * 0.25;
    let angle_step = PI / lod as f32;
    let radius = 75.0;

    (0..lod)
        .map(|i| {
            let point = Vec2::new(
                radius * (angle_start + i as f32 * angle_step).cos() * 0.5,
                radius * (angle_start + i as f32 * angle_step).sin(),
            );
            [
                point + Vec2::new(50.0, 0.0),
                -point - Vec2::new(50.0, 0.0),
                point * 0.9 + Vec2::new(50.0, 0.0),
                -point * 0.9 - Vec2::new(50.0, 0.0),
            ]
        })
        .flatten()
        .map(|v| [v.x, v.y])
        .collect()
}

fn pos_dot(lod: usize) -> Vec<[f32; 2]> {
    let radius = 8.0;
    (0..lod)
        .map(|i| {
            let angle = i as f32 * 2.0 * PI / (lod - 1) as f32;
            let dist = radius * Vec2::new(angle.cos(), angle.sin());
            [dist, dist * 0.75]
        })
        .flatten()
        .map(|v| [v.x, v.y])
        .collect()
}

fn index_crosshair(lod: usize) -> Vec<TriangleGeometry> {
    (0..lod)
        .map(|i| {
            [
                TriangleGeometry([(i) as u32 * 2, (i) as u32 * 2 + 2, (i) as u32 * 2 + 4]),
                TriangleGeometry([(i) as u32 * 2 + 1, (i) as u32 * 2 + 3, (i) as u32 * 2 + 5]),
            ]
        })
        .flatten()
        .collect()
}

fn index_dot(lod: usize) -> Vec<TriangleGeometry> {
    (0..lod * 2)
        .map(|i| {
            let pos = lod as u32 * 4 + i as u32;
            TriangleGeometry([pos, pos + 1, pos + 2])
        })
        .collect()
}
