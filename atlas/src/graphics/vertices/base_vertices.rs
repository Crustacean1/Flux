use std::mem::size_of;

use super::{Index, PrimitiveType, Shapely, Vertex};
use macros::Vertex;

#[derive(Vertex, Debug)]
#[repr(C)]
pub struct Vertex2P {
    pos: [f32; 2],
}

#[derive(Vertex, Debug)]
#[repr(C)]
pub struct Vertex2PT {
    pos: [f32; 2],
    tex: [f32; 2],
}

#[derive(Vertex, Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex3PT {
    pos: [f32; 3],
    tex: [f32; 2],
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
                            if ((i >> 0) & 1) == 1 { 1.0 } else { 0.0 },
                            if ((i >> 1) & 1) == 1 { 1.0 } else { 0.0 },
                        ],
                        1 => [
                            if ((i >> 0) & 1) == 0 { 1.0 } else { 0.0 },
                            if ((i >> 2) & 1) == 1 { 1.0 } else { 0.0 },
                        ],
                        2 => [
                            if ((i >> 1) & 1) == 1 { 1.0 } else { 0.0 },
                            if ((i >> 2) & 1) == 1 { 1.0 } else { 0.0 },
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

    fn gen_quad(width: f32, height: f32) -> Vec<Self::Attribute> {
        todo!()
    }
}

impl Shapely for Vertex2P {
    type Attribute = Vertex2P;
    fn gen_quad(width: f32, height: f32) -> Vec<Vertex2P> {
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
}

impl Shapely for Vertex2PT {
    type Attribute = Vertex2PT;
    fn gen_quad(width: f32, height: f32) -> Vec<Vertex2PT> {
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
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TriangleIndex {
    triangle: [u32; 3],
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

    fn gen_quad(_width: f32, _height: f32) -> Vec<TriangleIndex> {
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
}
