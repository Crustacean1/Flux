use super::{indices::TriangleGeometry, layouts::PTVertex};

pub fn skybox(side: f32) -> (Vec<PTVertex>, Vec<TriangleGeometry>) {
    let pos = pos_skybox(side);
    let tex = tex_skybox();
    let indices = index_skybox();

    let vertices: Vec<_> = pos.zip(tex).map(|(pos, tex)| PTVertex(pos, tex)).collect();
    (vertices, indices)
}

fn pos_skybox(side: f32) -> impl Iterator<Item = [f32; 3]> {
    let x_axis = (0..8).map(move |i| {
        [
            if ((i >> 1) & 1) == 1 { side } else { -side },
            if ((i >> 2) & 1) == 1 { side } else { -side },
            if ((i >> 0) & 1) == 1 { side } else { -side },
        ]
    });

    let y_axis = (0..8).map(move |i| {
        [
            if ((i >> 0) & 1) == 1 { side } else { -side },
            if ((i >> 2) & 1) == 1 { side } else { -side },
            if ((i >> 1) & 1) == 1 { side } else { -side },
        ]
    });

    let z_axis = (0..8).map(move |i| {
        [
            if ((i >> 0) & 1) == 1 { side } else { -side },
            if ((i >> 1) & 1) == 1 { side } else { -side },
            if ((i >> 2) & 1) == 1 { side } else { -side },
        ]
    });

    x_axis.chain(y_axis).chain(z_axis)
}

fn tex_skybox() -> impl Iterator<Item = [f32; 2]> {
    (0..3)
        .map(|_| {
            (0..8).map(move |i| {
                [
                    if ((i >> 0) & 1) == 0 { 1.0 } else { 0.0 },
                    if ((i >> 1) & 1) == 0 { 1.0 } else { 0.0 },
                ]
            })
        })
        .flatten()
}

fn index_skybox() -> Vec<TriangleGeometry> {
    (0..6)
        .map(|i| {
            [
                TriangleGeometry([0 + i * 4, 1 + i * 4, 2 + i * 4]),
                TriangleGeometry([2 + i * 4, 3 + i * 4, 1 + i * 4]),
            ]
        })
        .flatten()
        .collect()
}
