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
}

impl Shapely for Vertex2PT {
    type Attribute = Vertex2PT;
    fn gen_quad(width: f32, height: f32) -> Vec<Vertex2PT> {
        vec![
            Vertex2PT {
                pos: [-width, -height],
                tex: [0.0,1.0]
            },
            Vertex2PT {
                pos: [width, -height],
                tex: [1.0,1.0]
            },
            Vertex2PT {
                pos: [width, height],
                tex: [1.0,0.0]
            },
            Vertex2PT {
                pos: [-width, height],
                tex: [0.0,0.0]
            },
        ]
    }
}

#[derive(Debug)]
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
}
