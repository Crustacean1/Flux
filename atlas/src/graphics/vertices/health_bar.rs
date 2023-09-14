use super::{indices::TriangleGeometry, layouts::P2TVertex};

pub fn health_bar() -> (Vec<P2TVertex>, Vec<TriangleGeometry>) {
    let (inner, outer) = (100.0, 125.0);

    let quarter = 3.1415926535 / 4.0;
    let hemi = 3.1416926535 / 2.0;
    let vertices = (0..21)
        .map(|a| (hemi + quarter + hemi * a as f32 / 20.0, a as f32))
        .map(|(a, x)| {
            [
                P2TVertex([inner * a.cos(), inner * a.sin()], [x / 20.0, 0.0]),
                P2TVertex([outer * a.cos(), outer * a.sin()], [x / 20.0, 0.0]),
            ]
        })
        .flatten()
        .collect();

    let indices = (0..200)
        .map(|i| TriangleGeometry([i, i + 1, i + 2]))
        .collect();

    (vertices, indices)
}
