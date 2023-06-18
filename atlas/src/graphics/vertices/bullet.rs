use super::{
    indices::{LineGeometry, TriangleGeometry},
    layouts::PVertex,
};

pub fn bullet(side: f32, length: f32) -> (Vec<PVertex>, Vec<TriangleGeometry>) {
    let vertices = (0..2)
        .map(|x| {
            (0..2).map(move |y| {
                (0..2).map(move |z| {
                    [
                        side - 2.0 * x as f32 * side,
                        side - 2.0 * y as f32 * side,
                        length - 2.0 * z as f32 * length,
                    ]
                })
            })
        })
        .flatten()
        .flatten()
        .map(|v| PVertex(v))
        .collect();

    let indices = vec![
        TriangleGeometry([0, 1, 2]),
        TriangleGeometry([1, 2, 3]),
        TriangleGeometry([4, 5, 6]),
        TriangleGeometry([5, 6, 7]),
        /**/
        TriangleGeometry([0, 2, 4]),
        TriangleGeometry([2, 4, 6]),
        TriangleGeometry([1, 3, 5]),
        TriangleGeometry([3, 5, 7]),
        /**/
        TriangleGeometry([2, 3, 6]),
        TriangleGeometry([3, 6, 7]),
        TriangleGeometry([4, 5, 8]),
        TriangleGeometry([5, 8, 9]),
    ];

    (vertices, indices)
}
