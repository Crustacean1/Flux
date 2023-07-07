use super::{
    indices::TriangleGeometry,
    layouts::{Attribute, BufferElement, PTVertex},
};

#[repr(C)]
pub struct SkyboxInstance {
    billboard: u32,
    transform: [f32; 16],
}

impl BufferElement for SkyboxInstance {
    fn layout() -> Vec<Attribute> {
        vec![
            Attribute::UnsignedInt(1),
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
            Attribute::Float(4),
        ]
    }
}

pub fn skybox(side: f32) -> (Vec<PTVertex>, Vec<TriangleGeometry>, Vec<SkyboxInstance>) {
    let pos = pos_skybox(side);
    let tex = tex_skybox();
    let indices = index_skybox();

    let vertices: Vec<_> = pos.zip(tex).map(|(pos, tex)| PTVertex(pos, tex)).collect();

    indices
        .iter()
        .for_each(|vertex| println!("Skybox indices: {:?}", vertex));
    vertices
        .iter()
        .for_each(|vertex| println!("Skybox vertices: {:?}", vertex));

    let instances = instances_skybox();

    (vertices, indices, instances)
}

fn pos_skybox(side: f32) -> impl Iterator<Item = [f32; 3]> {
    (0..4).map(move |i| {
        [
            if ((i >> 1) & 1) == 1 { -side } else { side },
            if ((i >> 0) & 1) == 1 { side } else { -side },
            -side,
        ]
    })
}

fn tex_skybox() -> impl Iterator<Item = [f32; 2]> {
    (0..4).map(move |i| {
        [
            if ((i >> 1) & 1) == 0 { 1.0 } else { 0.0 },
            if ((i >> 0) & 1) == 0 { 1.0 } else { 0.0 },
        ]
    })
}

fn index_skybox() -> Vec<TriangleGeometry> {
    vec![TriangleGeometry([0, 1, 2]), TriangleGeometry([2, 3, 1])]
}

fn instances_skybox() -> Vec<SkyboxInstance> {
    let back = [
        -1.0, 0.0, 0.0, 0.0, /**/
        0.0, 1.0, 0.0, 0.0, /**/
        0.0, 0.0, -1.0, 0.0, /**/
        0.0, 0.0, 0.0, 1.0, /**/
    ];
    let front = [
        1.0, 0.0, 0.0, 0.0, /**/
        0.0, 1.0, 0.0, 0.0, /**/
        0.0, 0.0, 1.0, 0.0, /**/
        0.0, 0.0, 0.0, 1.0, /**/
    ];
    let right = [
        0.0, 0.0, 1.0, 0.0, /**/
        0.0, 1.0, 0.0, 0.0, /**/
        -1.0, 0.0, 0.0, 0.0, /**/
        0.0, 0.0, 0.0, 1.0, /**/
    ];
    let left = [
        0.0, 0.0, -1.0, 0.0, /**/
        0.0, 1.0, 0.0, 0.0, /**/
        1.0, 0.0, 0.0, 0.0, /**/
        0.0, 0.0, 0.0, 1.0, /**/
    ];
    let top = [
        1.0, 0.0, 0.0, 0.0, /**/
        0.0, 0.0, 1.0, 0.0, /**/
        0.0, -1.0, 0.0, 0.0, /**/
        0.0, 0.0, 0.0, 1.0, /**/
    ];
    let bottom = [
        1.0, 0.0, 0.0, 0.0, /**/
        0.0, 0.0, -1.0, 0.0, /**/
        0.0, 1.0, 0.0, 0.0, /**/
        0.0, 0.0, 0.0, 1.0, /**/
    ];

    vec![
        SkyboxInstance {
            billboard: 0,
            transform: front,
        },
        SkyboxInstance {
            billboard: 1,
            transform: back,
        },
        SkyboxInstance {
            billboard: 2,
            transform: bottom,
        },
        SkyboxInstance {
            billboard: 3,
            transform: top,
        },
        SkyboxInstance {
            billboard: 4,
            transform: left,
        },
        SkyboxInstance {
            billboard: 5,
            transform: right,
        },
    ]
}
