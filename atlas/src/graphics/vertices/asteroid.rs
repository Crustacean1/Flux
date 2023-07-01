use std::f32::consts::PI;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
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
    let radius = radius;

    let deformations: Vec<_> = (0..(rng.gen_range(10..20)))
        .map(|i| {
            let x_angle = rng.gen_range(0.0..2.0 * PI);
            let y_angle = rng.gen_range(0.0..2.0 * PI);
            Vec3::new(
                x_angle.cos() * y_angle.cos(),
                y_angle.sin(),
                x_angle.sin() * y_angle.cos(),
            )
        })
        .collect();

    let def_ref = &deformations;

    let side = radius / (3.0 as f32).sqrt();
    get_transform_matrices()
        .iter()
        .map(|&mat| {
            (0..details).map(move |x| {
                (0..details).map(move |y| {
                    let vec = Vec4::new(
                        side * x as f32 / (details - 1) as f32 - side * 0.5,
                        side * y as f32 / (details - 1) as f32 - side * 0.5,
                        side * 0.5,
                        0.0,
                    );
                    let vec = mat * vec;
                    let vec = vec.normalize().xyz();

                    let min_dist = def_ref
                        .iter()
                        .fold(0.0, |max, &def| (vec.dot(def)).max(max))
                        .powi(16);

                    let vec = vec * radius * (1.0 );

                    [vec.x, vec.y, vec.z]
                })
            })
        })
        .flatten()
        .flatten()
        .collect()

    /*(0..details)
    .map(|i| {
        (0..details).map(move |j| {
            let angle_x = 2.0 * PI * j as f32 / (details - 1) as f32;
            let angle_y = PI * i as f32 / (details - 1) as f32 - 0.5 * PI;

            let vec = [
                angle_x.cos() * angle_y.cos(),
                angle_y.sin(),
                angle_x.sin() * angle_y.cos(),
            ];


            let radius = radius + min_dist * radius * 0.25;
            let vec = [vec[0] * radius, vec[1] * radius, vec[2] * radius];
            vec
        })
    })
    .flatten()
    .collect()*/
}

fn norm_asteroid(details: usize) -> Vec<[f32; 3]> {
    get_transform_matrices()
        .iter()
        .map(|&mat| {
            (0..details).map(move |x| {
                (0..details).map(move |y| {
                    let vec = Vec4::new(
                        x as f32 / (details - 1) as f32 - 0.5,
                        y as f32 / (details - 1) as f32 - 0.5,
                        0.5,
                        0.0,
                    );
                    let vec = mat * vec;
                    [vec.x, vec.y, vec.z]
                })
            })
        })
        .flatten()
        .flatten()
        .collect()
}

fn tex_asteroid(details: usize) -> Vec<[f32; 2]> {
    get_transform_matrices()
        .iter()
        .map(|mat| {
            (0..details).map(move |x| {
                (0..details).map(move |y| [x as f32 / details as f32, y as f32 / details as f32])
            })
        })
        .flatten()
        .flatten()
        .collect()
}

fn index_asteroid(details: usize) -> Vec<TriangleGeometry> {
    get_transform_matrices()
        .iter()
        .enumerate()
        .map(|(k, _)| {
            (0..(details - 1)).map(move |i| {
                (0..(details - 1))
                    .map(move |j| {
                        let index = (k * details + i) * details + j;
                        [
                            TriangleGeometry([
                                index as u32,
                                (index + 1) as u32,
                                (index + details) as u32,
                            ]),
                            TriangleGeometry([
                                (index + details) as u32,
                                (index + details + 1) as u32,
                                (index + 1) as u32,
                            ]),
                        ]
                    })
                    .flatten()
            })
        })
        .flatten()
        .flatten()
        .collect()
}

fn get_transform_matrices() -> Vec<Mat4> {
    vec![
        Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
        Mat4::from_cols(
            Vec4::new(-1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
        Mat4::from_cols(
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(-1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
        Mat4::from_cols(
            Vec4::new(0.0, 0.0, -1.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
        Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, -1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
        Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -1.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
    ]
}
