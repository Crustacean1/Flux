use std::mem::size_of;

use super::{Index, PrimitiveType, Shapely, Vertex};
use macros::Vertex;

const PI: f32 = 3.1415926535;

#[derive(Vertex, Debug)]
#[repr(C)]
pub struct Vertex2P {
    pos: [f32; 2],
}

#[derive(Vertex, Debug)]
#[repr(C)]
pub struct Vertex2PT {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

#[derive(Vertex, Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex3PT {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
}

impl Shapely for Vertex3PT {
    type Attribute = Vertex3PT;

    fn skybox(side: f32) -> Vec<Vertex3PT> {
        (0..3)
            .map(|j| {
                (0..8).map(move |i| Vertex3PT {
                    pos: [
                        if ((i >> 0) & 1) == 1 { side } else { -side },
                        if ((i >> 1) & 1) == 1 { side } else { -side },
                        if ((i >> 2) & 1) == 1 { side } else { -side },
                    ],
                    tex: match j {
                        0 => [
                            if ((i >> 0) & 1) == 0 { 1.0 } else { 0.0 },
                            if ((i >> 1) & 1) == 0 { 1.0 } else { 0.0 },
                        ],
                        1 => [
                            if ((i >> 2) & 1) == 0 { 1.0 } else { 0.0 },
                            if ((i >> 0) & 1) == 0 { 1.0 } else { 0.0 },
                        ],
                        2 => [
                            if ((i >> 2) & 1) == 0 { 1.0 } else { 0.0 },
                            if ((i >> 1) & 1) == 0 { 1.0 } else { 0.0 },
                        ],
                        _ => {
                            panic!()
                        }
                    },
                })
            })
            .flatten()
            .collect()
    }

    fn sphere(radius: f32, detail: u32) -> Vec<Self::Attribute> {
        (0..detail)
            .map(|i| {
                (0..detail).map(move |j| {
                    let (x_angle, y_angle) = (
                        2.0 * PI * i as f32 / (detail - 1) as f32,
                        PI * j as f32 / (detail - 1) as f32 - PI * 0.5,
                    );
                    Vertex3PT {
                        pos: [
                            radius * y_angle.cos() * x_angle.cos(),
                            radius * y_angle.sin(),
                            radius * y_angle.cos() * x_angle.sin(),
                        ],
                        tex: [i as f32 / detail as f32, j as f32 / detail as f32],
                    }
                })
            })
            .flatten()
            .collect()
    }

    fn quad(_: f32, _: f32) -> Vec<Self::Attribute> {
        todo!()
    }
}

impl Shapely for Vertex2P {
    type Attribute = Vertex2P;
    fn quad(width: f32, height: f32) -> Vec<Vertex2P> {
        let (width, height) = (width * 0.5, height * 0.5);
        vec![
            Vertex2P {
                pos: [-width, -height],
            },
            Vertex2P {
                pos: [width, -height],
            },
            Vertex2P {
                pos: [width, height],
            },
            Vertex2P {
                pos: [-width, height],
            },
        ]
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        todo!()
    }

    fn sphere(radius: f32, detail: u32) -> Vec<Self::Attribute> {
        todo!()
    }
}

impl Shapely for Vertex2PT {
    type Attribute = Vertex2PT;
    fn quad(width: f32, height: f32) -> Vec<Vertex2PT> {
        vec![
            Vertex2PT {
                pos: [-width, -height],
                tex: [0.0, 0.0],
            },
            Vertex2PT {
                pos: [width, -height],
                tex: [1.0, 0.0],
            },
            Vertex2PT {
                pos: [width, height],
                tex: [1.0, 1.0],
            },
            Vertex2PT {
                pos: [-width, height],
                tex: [0.0, 1.0],
            },
        ]
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        todo!()
    }

    fn sphere(radius: f32, detail: u32) -> Vec<Self::Attribute> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TriangleIndex {
    pub triangle: [u32; 3],
}

impl Index for TriangleIndex {
    type IndexType = TriangleIndex;

    fn primitive_type() -> PrimitiveType {
        PrimitiveType::Triangles
    }

    fn index_count(poly_count: usize) -> usize {
        poly_count * 3
    }

    fn size(count: usize) -> usize {
        count * size_of::<Self::IndexType>()
    }
}

impl Shapely for TriangleIndex {
    type Attribute = TriangleIndex;

    fn quad(_width: f32, _height: f32) -> Vec<TriangleIndex> {
        vec![
            TriangleIndex {
                triangle: [0, 1, 2],
            },
            TriangleIndex {
                triangle: [2, 3, 0],
            },
        ]
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        vec![
            TriangleIndex {
                triangle: [0, 1, 3],
            },
            TriangleIndex {
                triangle: [3, 2, 0],
            },
            TriangleIndex {
                triangle: [4, 5, 7],
            },
            TriangleIndex {
                triangle: [7, 6, 4],
            },
            /*************************/
            TriangleIndex {
                triangle: [0 + 8, 4 + 8, 5 + 8],
            },
            TriangleIndex {
                triangle: [0 + 8, 5 + 8, 1 + 8],
            },
            TriangleIndex {
                triangle: [3 + 8, 7 + 8, 2 + 8],
            },
            TriangleIndex {
                triangle: [2 + 8, 7 + 8, 6 + 8],
            },
            /*************************/
            TriangleIndex {
                triangle: [0 + 16, 4 + 16, 2 + 16],
            },
            TriangleIndex {
                triangle: [4 + 16, 6 + 16, 2 + 16],
            },
            TriangleIndex {
                triangle: [1 + 16, 5 + 16, 3 + 16],
            },
            TriangleIndex {
                triangle: [5 + 16, 7 + 16, 3 + 16],
            },
            /*************************/
        ]
    }

    fn sphere(_: f32, detail: u32) -> Vec<Self::Attribute> {
        (0..(detail - 1))
            .map(|i| {
                (0..(detail - 1))
                    .map(move |j| {
                        [
                            TriangleIndex {
                                triangle: [
                                    i * detail + j,
                                    i * detail + j + 1,
                                    (i + 1) * detail + j,
                                ],
                            },
                            TriangleIndex {
                                triangle: [
                                    (i + 1) * detail + j,
                                    (i + 1) * detail + j + 1,
                                    i * detail + j + 1,
                                ],
                            },
                        ]
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    }
}
